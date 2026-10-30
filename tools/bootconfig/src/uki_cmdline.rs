//! Decode command line of a built UKI from the .cmdline section.

use crate::error::{self, Result};
use snafu::{ensure, ResultExt};

/// Largest command line the linux kernel can hold, including the terminating NUL
const COMMAND_LINE_SIZE: usize = 2048;

/// Decode an already extracted `.cmdline` section.
pub(crate) fn decode(section: &[u8]) -> Result<&str> {
    let end = section
        .iter()
        .position(|byte| *byte == 0)
        .unwrap_or(section.len());
    // PE sections can carry trailing NUL padding, but text after a terminator
    // would make the measured bytes ambiguous.
    ensure!(
        section[end..].iter().all(|byte| *byte == 0),
        error::UkiDataAfterNulSnafu
    );
    let cmdline = std::str::from_utf8(&section[..end]).context(error::InvalidUtf8Snafu {
        input: "UKI command line",
    })?;
    ensure!(
        !cmdline.is_empty() && cmdline.len() < COMMAND_LINE_SIZE,
        error::InvalidUkiSectionLengthSnafu {
            length: cmdline.len(),
        }
    );
    Ok(cmdline)
}

/// Check that a command line to embed separates its arguments with single spaces, pairs its
/// quotes, and fits the kernel, so tools that collapse whitespace leave it unchanged.
pub(crate) fn ensure_embeddable(cmdline: &str) -> Result<()> {
    ensure!(
        cmdline.trim() == cmdline
            && !cmdline.contains("  ")
            && !cmdline.contains(|character: char| character.is_whitespace() && character != ' '),
        error::UncollapsedWhitespaceSnafu
    );
    ensure!(
        cmdline.matches('"').count().is_multiple_of(2),
        error::UnpairedQuoteSnafu
    );
    ensure!(
        !cmdline.is_empty() && cmdline.len() < COMMAND_LINE_SIZE,
        error::InvalidUkiSectionLengthSnafu {
            length: cmdline.len(),
        }
    );
    Ok(())
}

/// Encode the EFI LoadOptions that systemd-stub measures into PCR 9, as
/// UTF-16LE including the terminating NUL.
pub(crate) fn load_options(cmdline: &str) -> Result<Vec<u8>> {
    ensure!(cmdline.is_ascii(), error::InvalidLoadOptionsSnafu);

    // whitespace normalization and encoding in UTF-16 LE
    let whitespace = |character: char| character <= '\u{20}' || character == '\u{7f}';
    let normalized: String = cmdline
        .trim_matches(whitespace)
        .chars()
        .map(|character| match whitespace(character) {
            true => ' ',
            false => character,
        })
        .collect();
    Ok(normalized
        .encode_utf16()
        .chain([0])
        .flat_map(u16::to_le_bytes)
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;

    #[test]
    fn a_section_is_decoded_with_only_trailing_nul_padding() {
        for section in [b"quiet --".as_slice(), b"quiet --\0\0\0".as_slice()] {
            assert_eq!(decode(section).unwrap(), "quiet --");
        }
        assert!(matches!(
            decode(b"quiet\0extra".as_slice()),
            Err(Error::UkiDataAfterNul)
        ));
        assert!(matches!(
            decode(b"\xff".as_slice()),
            Err(Error::InvalidUtf8 { input, .. }) if input == "UKI command line"
        ));
    }

    #[test]
    fn a_section_the_kernel_could_not_have_booted_is_rejected() {
        for section in [b"\0".as_slice(), &vec![b'x'; COMMAND_LINE_SIZE]] {
            assert!(matches!(
                decode(section),
                Err(Error::InvalidUkiSectionLength { .. })
            ));
        }
        assert!(decode(vec![b'x'; COMMAND_LINE_SIZE - 1].as_slice()).is_ok());
    }

    #[test]
    fn load_options_normalize_whitespace_and_end_with_a_utf16_nul() {
        assert_eq!(load_options(" \tA\tB ").unwrap(), b"A\0 \0B\0\0\0");
    }
}
