use crate::{error, format_params, parse, parsers::grub, Result};
use snafu::ensure;

const KERNEL_PATH_PREFIX: &str = "()/vmlinuz ";

/// grub.cfg uses shell-style quoting where values are quoted: `root="UUID=abc"`
/// The kernel command line expects the entire key=value pair quoted: `"root=UUID=abc"`
/// This function performs that transformation for PCR 9 prediction.
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

/// Predict /proc/cmdline from grub.cfg and bootconfig.
///
/// The kernel constructs /proc/cmdline as:
/// `<kernel.* bootconfig> BOOT_IMAGE=<path> <grub args> -- <init.* bootconfig> <grub args after -->`
///
/// If `boot_partuuid` is provided, replaces `PARTUUID=/PARTNROFF=` with the actual UUID.
pub fn predict_grub_cmdline(
    grub_cfg: &[u8],
    bootconfig_data: &[u8],
    boot_partuuid: Option<&str>,
) -> Result<String> {
    let grub_cmdline = grub::parse(grub_cfg)?;
    let bootconfig_params = parse(bootconfig_data)?;
    let kernel_params = format_params(&bootconfig_params.kernel)?;
    let init_params = format_params(&bootconfig_params.init)?;

    // Verify and transform kernel path
    ensure!(
        grub_cmdline.starts_with(KERNEL_PATH_PREFIX),
        error::InvalidGrubKernelPathSnafu {
            expected: KERNEL_PATH_PREFIX.trim(),
            actual: grub_cmdline.chars().take(20).collect::<String>(),
        }
    );
    let mut grub_args = grub_cmdline.replacen(KERNEL_PATH_PREFIX, "BOOT_IMAGE=/vmlinuz ", 1);

    // Substitute PARTUUID placeholder with actual boot partition UUID
    if let Some(uuid) = boot_partuuid {
        grub_args = grub_args.replace(
            "PARTUUID=/PARTNROFF=",
            &format!("PARTUUID={uuid}/PARTNROFF="),
        );
    }

    // Apply kernel's quote repair transformation to grub args
    grub_args = repair_quotes(&grub_args);

    // Split grub args at "--"
    let (before_sep, after_sep) = if let Some(pos) = grub_args.find(" -- ") {
        (&grub_args[..pos], &grub_args[pos + 4..])
    } else {
        (grub_args.as_str(), "")
    };

    // Construct final cmdline
    let mut cmdline = String::new();
    cmdline.push_str(&kernel_params);
    cmdline.push_str(before_sep);
    cmdline.push_str(" -- ");
    cmdline.push_str(&init_params);
    cmdline.push_str(after_sep);

    Ok(cmdline)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

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
