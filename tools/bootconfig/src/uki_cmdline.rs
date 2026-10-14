use crate::{error, format_params, BootConfig, Result};
use snafu::{ensure, ResultExt};

impl BootConfig {
    /// Compose the signed UKI command line from bootconfig and base arguments.
    /// Always includes an explicit `--` boundary, even with no init arguments.
    ///
    /// Base arguments are supplied by the image builder. The complete command
    /// line must fit the supported kernel's 2048-byte buffer including NUL.
    pub fn uki_cmdline(&self, kernel: &str, init: &str) -> Result<String> {
        let kernel = format!("{}{kernel}", format_params(&self.kernel)?);
        let init = format!("{}{init}", format_params(&self.init)?);
        let cmdline = [kernel.trim(), "--", init.trim()]
            .into_iter()
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join(" ");
        ensure!(
            cmdline.len() < 2048,
            error::UkiCommandLineTooLongSnafu {
                length: cmdline.len(),
            }
        );
        Ok(cmdline)
    }
}

/// Apply systemd-stub's control-character and surrounding whitespace handling.
/// The returned bytes are EFI LoadOptions, encoded as UTF-16LE including NUL.
///
/// Embedded NUL characters are rejected rather than truncating the command line.
pub fn uki_load_options(cmdline: &str) -> Result<Vec<u8>> {
    ensure!(
        cmdline.is_ascii() && !cmdline.contains('\0'),
        error::InvalidLoadOptionsSnafu
    );
    let whitespace = |c: char| c <= '\u{20}' || c == '\u{7f}';
    let normalized: String = cmdline
        .trim_matches(whitespace)
        .chars()
        .map(|c| if whitespace(c) { ' ' } else { c })
        .collect();
    Ok(normalized
        .encode_utf16()
        .chain([0])
        .flat_map(u16::to_le_bytes)
        .collect())
}

/// Decode an already-extracted UKI `.cmdline` section.
///
/// The caller owns PE parsing and checks for unsupported measurement inputs.
pub fn parse_uki_cmdline(section: &[u8]) -> Result<&str> {
    let end = section
        .iter()
        .position(|b| *b == 0)
        .unwrap_or(section.len());
    // PE sections can carry trailing NUL padding, but text after a terminator
    // would make the bytes inspected by the predictor ambiguous.
    ensure!(
        section[end..].iter().all(|b| *b == 0),
        error::UkiDataAfterNulSnafu
    );
    let cmdline = std::str::from_utf8(&section[..end]).context(error::InvalidUtf8Snafu {
        input: "UKI command line",
    })?;
    ensure!(
        !cmdline.is_empty() && cmdline.len() < 2048,
        error::InvalidUkiSectionLengthSnafu {
            length: cmdline.len(),
        }
    );
    Ok(cmdline)
}

#[cfg(test)]
#[path = "test_uki_cmdline.rs"]
mod tests;
