use crate::{error, format_params, BootConfig, Result};
use snafu::{ensure, ResultExt};

impl BootConfig {
    /// Compose the signed UKI command line from BootConfig and base arguments.
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
mod tests {
    use super::*;

    #[test]
    fn uki_arguments_keep_kernel_and_init_namespaces_separate() {
        let config = crate::parse_text(
            "kernel { console=tty0,ttyS0; quiet }\ninit.systemd.unit=rescue.target",
        )
        .unwrap();
        assert_eq!(
            config.uki_cmdline("root=/dev/dm-0", "").unwrap(),
            "console=tty0 console=ttyS0 quiet root=/dev/dm-0 -- systemd.unit=rescue.target"
        );
    }

    #[test]
    fn load_options_normalize_whitespace_and_include_utf16_nul() {
        assert_eq!(uki_load_options(" \tA\tB ").unwrap(), b"A\0 \0B\0\0\0");
        assert!(uki_load_options("a\0b").is_err());
    }

    #[test]
    fn section_decoding_accepts_only_trailing_nul_padding() {
        for section in [b"quiet --".as_slice(), b"quiet --\0\0\0".as_slice()] {
            assert_eq!(parse_uki_cmdline(section).unwrap(), "quiet --");
        }
        assert!(matches!(
            parse_uki_cmdline(b"quiet\0extra"),
            Err(crate::Error::UkiDataAfterNul)
        ));
        assert!(matches!(
            parse_uki_cmdline(b"\xff"),
            Err(crate::Error::InvalidUtf8 { .. })
        ));
        assert!(matches!(
            parse_uki_cmdline(b"\0"),
            Err(crate::Error::InvalidUkiSectionLength { .. })
        ));
        assert!(parse_uki_cmdline(&vec![b'x'; 2047]).is_ok());
        assert!(matches!(
            parse_uki_cmdline(&vec![b'x'; 2048]),
            Err(crate::Error::InvalidUkiSectionLength { .. })
        ));
    }

    #[test]
    fn image_builder_arguments_are_used_without_an_extra_validation_pass() {
        let config = BootConfig::default();
        assert_eq!(
            config.uki_cmdline("example=\"open", "").unwrap(),
            "example=\"open --"
        );
        assert!(matches!(
            config.uki_cmdline(&"x".repeat(2048), ""),
            Err(crate::Error::UkiCommandLineTooLong { .. })
        ));
    }
}
