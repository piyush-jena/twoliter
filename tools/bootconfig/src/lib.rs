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

/// Parameters in Linux bootconfig tree traversal order, with namespaces removed.
#[derive(Debug, Default)]
pub struct BootConfig {
    pub kernel: Vec<Parameter>,
    pub init: Vec<Parameter>,
}

/// Decode a standalone SDK-generated binary bootconfig.
///
/// Rejects invalid trailers, checksum, size, padding, UTF-8 and syntax.
/// An initrd with a prepended payload is not a standalone bootconfig.
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
/// Linux treats hyphens and underscores alike in parameter names, so both
/// spellings are merged into the canonical `kernel.module_blacklist`. Other
/// lines retain their order; the combined blacklist is appended at the end.
/// Module names must be nonempty ASCII letters, digits or underscores; empty
/// contributions cannot erase another package's blacklist.
/// This must run before the SDK bootconfig tool, not while predicting a
/// command line from an already generated binary.
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
    // Block-form blacklists bypass the line merger and cannot safely be
    // combined with its single canonical assignment.
    let config = parse_text(&output)?;
    let blacklists = config
        .kernel
        .iter()
        .filter(|p| matches!(p.name.as_str(), "module_blacklist" | "module-blacklist"))
        .collect::<Vec<_>>();
    ensure!(
        blacklists.is_empty()
            || (!modules.is_empty() && blacklists.len() == 1 && blacklists[0].value.is_some()),
        error::InvalidBlacklistFormSnafu
    );
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
mod test_lib;
