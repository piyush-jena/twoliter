//! Prediction of the command line GRUB starts a kernel with, and the one the
//! kernel reports, which PCR 9 measures on images that boot through GRUB.

use crate::error::{self, Result};
use crate::params::BootconfigParams;
use crate::parsers::grub::GrubConfig;
use snafu::{ensure, OptionExt};

const KERNEL_PATH: &str = "()/vmlinuz";
const BOOT_IMAGE: &str = "BOOT_IMAGE=/vmlinuz";
const SEPARATOR: &str = "--";
const PARTUUID_PLACEHOLDER: &str = "PARTUUID=/PARTNROFF=";

/// The arguments GRUB passes a kernel after its path, rendered as the kernel receives them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct GrubCmdline {
    arguments: Vec<String>,
    /// Where the first separator is in `arguments`; the kernel's own parameters precede it.
    separator: Option<usize>,
}

impl GrubCmdline {
    /// Decode the `linux` directive of a grub.cfg, with `set` variables
    /// resolved, into the arguments it gives the kernel.
    ///
    /// `boot_partuuid` fills in the `PARTUUID=/PARTNROFF=` placeholder wherever it appears.
    pub(crate) fn parse(grub_cfg: &[u8], boot_partuuid: &str) -> Result<Self> {
        let config = GrubConfig::parse(grub_cfg)?;
        let arguments = config.linux_arguments()?;

        let (kernel_path, arguments) = arguments
            .split_first()
            .context(error::MissingGrubKernelPathSnafu)?;
        ensure!(
            kernel_path == KERNEL_PATH,
            error::InvalidGrubKernelPathSnafu {
                expected: KERNEL_PATH,
                actual: kernel_path.clone(),
            }
        );

        let separator = arguments.iter().position(|argument| argument == SEPARATOR);
        let partuuid = format!("PARTUUID={boot_partuuid}/PARTNROFF=");
        let arguments = arguments
            .iter()
            .map(|argument| repair_quotes(&argument.replace(PARTUUID_PLACEHOLDER, &partuuid)))
            .collect();
        Ok(Self {
            arguments,
            separator,
        })
    }

    /// The command line GRUB starts the kernel with. Bottlerocket's GRUB writes
    /// `BOOT_IMAGE=/vmlinuz` and a space ahead of the arguments, even when there are none.
    pub(crate) fn loader_cmdline(&self) -> String {
        format!("{BOOT_IMAGE} {}", self.arguments.join(" "))
    }
}

impl BootconfigParams {
    /// The command line a GRUB-booted kernel reports, with these parameters added on each side
    /// of the separator.
    pub(crate) fn grub_cmdline(&self, grub: &GrubCmdline) -> Result<String> {
        let (kernel, init) = match grub.separator {
            Some(index) => (&grub.arguments[..index], &grub.arguments[index + 1..]),
            None => (&grub.arguments[..], &grub.arguments[..0]),
        };
        self.cmdline(
            &format!("{BOOT_IMAGE} {}", kernel.join(" ")),
            &init.join(" "),
        )
    }
}

/// Rewrite `key="value"` as `"key=value"`, as GRUB passes an argument that contains a space.
/// GRUB leaves a value without spaces unquoted, which this does not model.
fn repair_quotes(cmdline: &str) -> String {
    let mut result = String::with_capacity(cmdline.len());
    let mut chars = cmdline.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '=' && chars.peek() == Some(&'"') {
            // Found `="`; scan back to find start of key
            let key_start = result.rfind(' ').map(|i| i + 1).unwrap_or(0);
            let key = result[key_start..].to_string();
            result.truncate(key_start);

            // Skip the opening quote
            chars.next();

            // Collect value until closing quote
            let mut value = String::new();
            for vc in chars.by_ref() {
                if vc == '"' {
                    break;
                }
                value.push(vc);
            }

            // Output as "key=value"
            result.push('"');
            result.push_str(&key);
            result.push('=');
            result.push_str(&value);
            result.push('"');
        } else {
            result.push(c);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::bootconfig::parse_text;
    use crate::Error;
    use test_case::test_case;

    fn predict_cmdline(grub_cfg: &[u8], bootconfig: &str, boot_partuuid: &str) -> Result<String> {
        parse_text(bootconfig)?.grub_cmdline(&GrubCmdline::parse(grub_cfg, boot_partuuid)?)
    }

    #[test_case(
        b"linux ($root)/vmlinuz console=tty0 -- systemd.log_target=journal",
        "kernel.FOO = bar\n",
        "FOO=bar BOOT_IMAGE=/vmlinuz console=tty0 -- systemd.log_target=journal"
        ; "kernel_param_before_boot_image"
    )]
    #[test_case(
        b"linux ($root)/vmlinuz quiet -- init=/sbin/init",
        "init.BAZ = qux\n",
        "BOOT_IMAGE=/vmlinuz quiet -- BAZ=qux init=/sbin/init"
        ; "init_param_after_separator"
    )]
    #[test_case(
        b"linux ($root)/vmlinuz console=tty0 -- systemd.log_color=0",
        "kernel.A = 1\ninit.B = 2\n",
        "A=1 BOOT_IMAGE=/vmlinuz console=tty0 -- B=2 systemd.log_color=0"
        ; "both_kernel_and_init_params"
    )]
    #[test_case(
        b"linux ($root)/vmlinuz quiet -- x",
        "",
        "BOOT_IMAGE=/vmlinuz quiet -- x"
        ; "empty_bootconfig"
    )]
    #[test_case(
        b"linux ($root)/vmlinuz ro rootwait -- x",
        "kernel {}\ninit {}\n",
        "BOOT_IMAGE=/vmlinuz ro rootwait -- x"
        ; "empty_bootconfig_namespaces"
    )]
    #[test_case(
        b"linux ($root)/vmlinuz -- x",
        "kernel.A = 1\n",
        "A=1 BOOT_IMAGE=/vmlinuz -- x"
        ; "kernel_only_no_init_bootconfig"
    )]
    #[test_case(
        b"linux ($root)/vmlinuz foo -- bar",
        "init.X = y\n",
        "BOOT_IMAGE=/vmlinuz foo -- X=y bar"
        ; "init_only_no_kernel_bootconfig"
    )]
    #[test_case(
        b"linux ($root)/vmlinuz dm-mod.create=\"root verity\" -- x",
        "",
        r#"BOOT_IMAGE=/vmlinuz "dm-mod.create=root verity" -- x"#
        ; "grub_quoted_value_gets_repaired"
    )]
    #[test_case(
        b"linux ($root)/vmlinuz -- x",
        "kernel.MSG = \"hello world\"\n",
        r#"MSG="hello world" BOOT_IMAGE=/vmlinuz -- x"#
        ; "bootconfig_quoted_value_not_repaired"
    )]
    #[test_case(
        b"linux ($root)/vmlinuz a=1 b=2 -- c=3 d=4",
        "kernel.K = v\ninit.I = w\n",
        "K=v BOOT_IMAGE=/vmlinuz a=1 b=2 -- I=w c=3 d=4"
        ; "multiple_grub_args_both_sides"
    )]
    fn test_predict_cmdline(grub_cfg: &[u8], bootconfig: &str, expected: &str) {
        assert_eq!(
            predict_cmdline(grub_cfg, bootconfig, "abcd-1234").unwrap(),
            expected
        );
    }

    #[test_case(
        br#"linux ($root)/vmlinuz dm-mod.create="a -- b" -- x"#,
        r#"BOOT_IMAGE=/vmlinuz "dm-mod.create=a -- b" -- x"#
        ; "the_separator_inside_a_value_does_not_divide_the_arguments"
    )]
    #[test_case(
        b"linux ($root)/vmlinuz quiet --",
        "BOOT_IMAGE=/vmlinuz quiet --"
        ; "a_separator_with_nothing_after_it_is_not_repeated"
    )]
    #[test_case(
        b"linux ($root)/vmlinuz quiet",
        "BOOT_IMAGE=/vmlinuz quiet --"
        ; "every_argument_stays_with_the_kernel_without_a_separator"
    )]
    #[test_case(
        b"linux ($root)/vmlinuz",
        "BOOT_IMAGE=/vmlinuz --"
        ; "a_directive_with_no_arguments_boots_the_image_alone"
    )]
    fn test_predict_cmdline_separator(grub_cfg: &[u8], expected: &str) {
        assert_eq!(
            predict_cmdline(grub_cfg, "", "abcd-1234").unwrap(),
            expected
        );
    }

    #[test_case(
        br#"linux ($root)/vmlinuz quiet dm-mod.create="a b" -- systemd.log_target=journal"#,
        r#"BOOT_IMAGE=/vmlinuz quiet "dm-mod.create=a b" -- systemd.log_target=journal"#
        ; "every_argument_in_order_with_the_separator"
    )]
    #[test_case(
        b"linux ($root)/vmlinuz quiet --",
        "BOOT_IMAGE=/vmlinuz quiet --"
        ; "a_trailing_separator_is_kept"
    )]
    #[test_case(
        b"linux ($root)/vmlinuz quiet",
        "BOOT_IMAGE=/vmlinuz quiet"
        ; "no_separator_is_added"
    )]
    #[test_case(
        b"linux ($root)/vmlinuz",
        "BOOT_IMAGE=/vmlinuz "
        ; "the_image_alone_keeps_the_space_grub_writes_after_it"
    )]
    #[test_case(
        b"linux ($root)/vmlinuz root=PARTUUID=/PARTNROFF=1 -- x",
        "BOOT_IMAGE=/vmlinuz root=PARTUUID=abcd-1234/PARTNROFF=1 -- x"
        ; "the_partuuid_is_filled_in"
    )]
    fn test_loader_cmdline(grub_cfg: &[u8], expected: &str) {
        let grub = GrubCmdline::parse(grub_cfg, "abcd-1234").unwrap();
        assert_eq!(grub.loader_cmdline(), expected);
    }

    #[test]
    fn test_predict_cmdline_partuuid_substitution() {
        let grub_cfg = b"linux ($root)/vmlinuz root=PARTUUID=/PARTNROFF=1 -- x";
        assert_eq!(
            predict_cmdline(grub_cfg, "", "abcd-1234").unwrap(),
            "BOOT_IMAGE=/vmlinuz root=PARTUUID=abcd-1234/PARTNROFF=1 -- x"
        );
    }

    #[test]
    fn test_predict_cmdline_multiple_partuuid_substitution() {
        let grub_cfg = b"linux ($root)/vmlinuz PARTUUID=/PARTNROFF=1 PARTUUID=/PARTNROFF=2 -- x";
        assert_eq!(
            predict_cmdline(grub_cfg, "", "uuid-here").unwrap(),
            concat!(
                "BOOT_IMAGE=/vmlinuz PARTUUID=uuid-here/PARTNROFF=1 ",
                "PARTUUID=uuid-here/PARTNROFF=2 -- x",
            )
        );
    }

    #[test_case(b"linux /wrong/path console=tty0 -- x" ; "wrong_kernel_path")]
    #[test_case(b"linux (hd0,gpt3)/vmlinuz console=tty0 -- x" ; "explicit_device_in_path")]
    fn test_predict_cmdline_errors(grub_cfg: &[u8]) {
        assert!(matches!(
            predict_cmdline(grub_cfg, "", "abcd-1234"),
            Err(Error::InvalidGrubKernelPath { .. })
        ));
    }

    #[test]
    fn a_directive_naming_no_kernel_is_rejected() {
        assert!(matches!(
            predict_cmdline(b"linux\n", "", "abcd-1234"),
            Err(Error::MissingGrubKernelPath)
        ));
    }

    #[test_case("key=value", "key=value" ; "no_quotes_unchanged")]
    #[test_case("simple", "simple" ; "no_equals_unchanged")]
    #[test_case(r#"key="value""#, r#""key=value""# ; "simple_quoted_value")]
    #[test_case(r#"key="value with spaces""#, r#""key=value with spaces""# ; "quoted_value_with_spaces")]
    #[test_case(r#"foo=bar key="quoted value" baz=qux"#, r#"foo=bar "key=quoted value" baz=qux"# ; "mixed_quoted_and_unquoted")]
    #[test_case(r#"dm-mod.create="root,,,ro,0 123 verity""#, r#""dm-mod.create=root,,,ro,0 123 verity""# ; "dm_mod_create_style")]
    #[test_case(r#"a="1" b="2" c="3""#, r#""a=1" "b=2" "c=3""# ; "multiple_quoted_values")]
    #[test_case(r#"first="val""#, r#""first=val""# ; "quoted_at_start")]
    #[test_case("", "" ; "empty_string")]
    fn test_repair_quotes(input: &str, expected: &str) {
        assert_eq!(repair_quotes(input), expected);
    }
}
