//! Bootconfig decoding and boot command-line construction shared by image
//! assembly and PCR prediction. No shell evaluation or I/O occurs here.

pub mod error;
mod grub_cmdline;
mod parsers;
mod uki_cmdline;

pub use error::{Error, Result};
pub use grub_cmdline::predict_grub_cmdline;
pub use parsers::bootconfig::parse_text;
pub use uki_cmdline::{parse_uki_cmdline, uki_load_options};

use parsers::bootconfig::{BootconfigParser, Rule};
use pest::Parser;
use snafu::{ensure, OptionExt, ResultExt};
use std::collections::BTreeSet;

// The trailer stores two little-endian u32 fields followed by the magic.
const BOOTCONFIG_MAGIC: &[u8] = b"#BOOTCONFIG\n";
const BOOTCONFIG_FIELD_SIZE: usize = size_of::<u32>();
const BOOTCONFIG_TRAILER_SIZE: usize = 2 * BOOTCONFIG_FIELD_SIZE + BOOTCONFIG_MAGIC.len();

/// One boot argument. `None` is a flag; `Some("")` is an explicit empty value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parameter {
    pub name: String,
    pub value: Option<String>,
}

/// Parsed bootconfig parameters split by prefix.
#[derive(Debug, Default)]
pub struct BootConfig {
    /// kernel.* parameters (prefix stripped).
    pub kernel: Vec<Parameter>,
    /// init.* parameters (prefix stripped).
    pub init: Vec<Parameter>,
}

/// Parse bootconfig.data binary format and extract parameters.
pub fn parse(data: &[u8]) -> Result<BootConfig> {
    // Require the complete trailer before reading its size, checksum, and magic.
    ensure!(
        data.len() >= BOOTCONFIG_TRAILER_SIZE,
        error::BinaryTooShortSnafu { length: data.len() }
    );
    let footer = data.len() - BOOTCONFIG_TRAILER_SIZE;
    ensure!(
        &data[footer + 2 * BOOTCONFIG_FIELD_SIZE..] == BOOTCONFIG_MAGIC,
        error::InvalidMagicSnafu
    );
    let size = u32::from_le_bytes(
        data[footer..footer + BOOTCONFIG_FIELD_SIZE]
            .try_into()
            .unwrap(),
    ) as usize;
    let checksum = u32::from_le_bytes(
        data[footer + BOOTCONFIG_FIELD_SIZE..footer + 2 * BOOTCONFIG_FIELD_SIZE]
            .try_into()
            .unwrap(),
    );
    ensure!(
        size == footer && size > 0 && size <= 32767 && data.len().is_multiple_of(4),
        error::InvalidBinarySizeSnafu {
            size,
            length: data.len(),
        }
    );
    let text_data = &data[..size];
    let sum = text_data
        .iter()
        .fold(0u32, |sum, b| sum.wrapping_add(u32::from(*b)));
    ensure!(
        checksum == sum,
        error::ChecksumMismatchSnafu {
            expected: checksum,
            actual: sum,
        }
    );
    let end = text_data.iter().position(|b| *b == 0).unwrap_or(size);
    ensure!(
        end < size && size - end <= 4 && text_data[end..].iter().all(|b| *b == 0),
        error::InvalidPaddingSnafu
    );
    parse_text(
        std::str::from_utf8(&text_data[..end]).context(error::InvalidUtf8Snafu {
            input: "bootconfig",
        })?,
    )
}

/// Merge single-line bootconfig drop-ins before generating binary bootconfig.
///
/// `kernel.module_blacklist` contributions are combined into one quoted,
/// comma-separated value, with module names sorted and deduplicated.
pub fn merge_blacklisted_modules(text: &str) -> Result<String> {
    let mut output = String::new();
    let mut modules = BTreeSet::new();
    for line in text.lines() {
        let blacklist = line.split_once('=').is_some_and(|(key, _)| {
            matches!(
                key.trim(),
                "kernel.module_blacklist" | "kernel.module-blacklist"
            )
        });
        if !blacklist {
            output.push_str(line);
            output.push('\n');
            continue;
        }
        let config = parse_text(line)?;
        let parsed = BootconfigParser::parse(Rule::config, line)
            .context(error::BootconfigParseSnafu)?
            .next()
            .expect("grammar requires config");
        let mut statements = parsed.into_inner().filter(|p| p.as_rule() != Rule::EOI);
        // Removing a blacklist line must not discard another assignment that
        // happens to follow it on the same line.
        ensure!(
            statements.next().is_some_and(|p| p.as_rule() == Rule::pair)
                && statements.next().is_none(),
            error::InvalidBlacklistStatementSnafu
        );
        for parameter in config.kernel {
            let value = parameter.value.context(error::MissingBlacklistValueSnafu)?;
            for module in value.split(',') {
                let module = module.trim();
                ensure!(
                    !module.is_empty()
                        && module
                            .bytes()
                            .all(|b| b.is_ascii_alphanumeric() || b == b'_'),
                    error::InvalidModuleNameSnafu { name: module }
                );
                modules.insert(module.to_owned());
            }
        }
    }
    // Check untouched lines before appending: duplicate-value replacement
    // would otherwise hide a block-form blacklist behind the merged assignment.
    let config = parse_text(&output)?;
    let blacklists = config
        .kernel
        .iter()
        .filter(|p| matches!(p.name.as_str(), "module_blacklist" | "module-blacklist"))
        .collect::<Vec<_>>();
    ensure!(blacklists.is_empty(), error::InvalidBlacklistFormSnafu);
    if !modules.is_empty() {
        let value = modules
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
            .join(",");
        // Quoting is essential: an unquoted bootconfig array would emit a
        // separate command-line assignment for each module.
        output.push_str(&format!("kernel.module_blacklist = \"{value}\"\n"));
    }
    Ok(output)
}

/// Render using Linux `xbc_snprint_cmdline` rules, including trailing spaces.
///
/// Double quotes inside values cannot be faithfully represented in a kernel
/// argument and are rejected instead of being interpreted as shell escapes.
pub fn format_params(params: &[Parameter]) -> Result<String> {
    let mut out = String::new();
    for parameter in params {
        out.push_str(&parameter.name);
        if let Some(value) = &parameter.value {
            ensure!(
                !value.contains('"'),
                error::UnsupportedQuoteSnafu {
                    name: &parameter.name,
                }
            );
            out.push('=');
            let quote = value.contains([' ', '\t', '\r', '\n']);
            if quote {
                out.push('"');
            }
            out.push_str(value);
            if quote {
                out.push('"');
            }
        }
        out.push(' ');
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_trailer_is_required_and_checked() {
        // Padded "kernel.x=1", size 12, byte-sum checksum 917, and magic.
        let data = b"kernel.x=1\0\0\x0c\0\0\0\x95\x03\0\0#BOOTCONFIG\n";
        assert_eq!(parse(data).unwrap().kernel[0].value.as_deref(), Some("1"));
        assert!(parse(&data[..BOOTCONFIG_TRAILER_SIZE - 1]).is_err());
        let mut corrupt = *data;
        corrupt[0] = b'K';
        assert!(parse(&corrupt).is_err());
    }

    #[test]
    fn text_preserves_tree_order_and_replaces_duplicate_values() {
        let config = parse_text("kernel.z.x=first\nkernel.a=middle\nkernel.z.y=last").unwrap();
        assert_eq!(
            format_params(&config.kernel).unwrap(),
            "z.x=first z.y=last a=middle "
        );
        let config = parse_text("kernel.x=a\nkernel.x=b,c").unwrap();
        assert_eq!(format_params(&config.kernel).unwrap(), "x=b x=c ");
    }

    #[test]
    fn grammar_handles_quotes_comments_and_nested_blocks() {
        for prefix in [
            "kernel.x=\"a\\\"b\"\n",
            "kernel.x=\"a\\\\\"\n",
            "kernel.x='a\\'\n",
            "# comment\r",
        ] {
            let nested = format!("{prefix}{}{}", "a {".repeat(65), "}".repeat(65));
            assert!(parse_text(&nested).is_ok(), "{prefix:?}");
        }
    }

    #[test]
    fn flag_with_children_matches_linux_namespace_handling() {
        let config = parse_text("kernel.debug\nkernel.debug.level=1").unwrap();
        assert_eq!(format_params(&config.kernel).unwrap(), "debug.level=1 ");
    }

    #[test]
    fn text_parser_leaves_size_and_character_policy_to_callers() {
        let key = "x".repeat(256);
        let value = "é".repeat(17000);
        let config = parse_text(&format!("kernel.{key}=\"{value}\"")).unwrap();
        assert_eq!(config.kernel[0].name, key);
        assert_eq!(config.kernel[0].value.as_deref(), Some(value.as_str()));
    }

    #[test]
    fn syntax_error_keeps_the_parser_diagnostic() {
        let error = parse_text("kernel.x=\"unterminated").unwrap_err();
        assert!(matches!(error, Error::BootconfigParse { .. }));
        assert!(error.to_string().contains("kernel.x"));
    }

    #[test]
    fn combines_package_blacklists_into_one_kernel_argument() {
        let input = concat!(
            "kernel.module_blacklist = i8042\n",
            "kernel.fips = 1\n",
            "kernel.module_blacklist = binfmt_misc\n",
            "kernel.module_blacklist = \"i8042,binfmt_misc\"\n",
            "kernel.sysctl.kernel.kexec_load_disabled = 1\n",
            "kernel.systemd.mask = proc-sys-fs-binfmt_misc.mount\n",
            "init.systemd.unit = fipscheck.target\n",
        );
        let merged = merge_blacklisted_modules(input).unwrap();
        let config = parse_text(&merged).unwrap();
        assert_eq!(
            config.uki_cmdline("root=/dev/dm-0", "").unwrap(),
            concat!(
                "fips=1 ",
                "sysctl.kernel.kexec_load_disabled=1 ",
                "systemd.mask=proc-sys-fs-binfmt_misc.mount ",
                "module_blacklist=binfmt_misc,i8042 ",
                "root=/dev/dm-0 -- systemd.unit=fipscheck.target"
            )
        );
        assert_eq!(merge_blacklisted_modules(&merged).unwrap(), merged);
    }

    #[test]
    fn preserves_other_arrays_and_namespaces() {
        let input = concat!(
            "kernel.console = tty0, ttyS0\n",
            "kernel.module_blacklist = i8042, binfmt_misc, i8042\n",
            "init.module_blacklist = userspace\n",
        );
        let config = parse_text(&merge_blacklisted_modules(input).unwrap()).unwrap();
        assert_eq!(
            format_params(&config.kernel).unwrap(),
            "console=tty0 console=ttyS0 module_blacklist=binfmt_misc,i8042 "
        );
        assert_eq!(
            format_params(&config.init).unwrap(),
            "module_blacklist=userspace "
        );
    }

    #[test]
    fn merges_kernel_parameter_alias_without_losing_contributions() {
        let merged = merge_blacklisted_modules(concat!(
            "kernel.module-blacklist = i8042\n",
            "kernel.module_blacklist = binfmt_misc\n",
        ))
        .unwrap();
        assert_eq!(merged, "kernel.module_blacklist = \"binfmt_misc,i8042\"\n");
        assert_eq!(
            format_params(&parse_text(&merged).unwrap().kernel).unwrap(),
            "module_blacklist=binfmt_misc,i8042 "
        );
    }

    #[test]
    fn rejects_empty_or_malformed_blacklist_assignments() {
        for input in [
            "kernel.module_blacklist =",
            "kernel.module_blacklist = \"i8042,,binfmt_misc\"",
            "kernel.module_blacklist = \"i8042 binfmt_misc\"",
            "kernel.module_blacklist = ../binfmt_misc",
            "kernel.module_blacklist = i8042; init.extra = value",
            "kernel.module_blacklist = i8042; other.extra = value",
            "kernel.module_blacklist += binfmt_misc",
            "kernel.module_blacklist",
            "kernel { module_blacklist = i8042 }",
            "kernel { module_blacklist = binfmt_misc }\nkernel.module_blacklist = i8042",
        ] {
            assert!(merge_blacklisted_modules(input).is_err(), "{input}");
        }
    }
}
