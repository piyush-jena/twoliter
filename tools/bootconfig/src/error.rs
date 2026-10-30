use crate::parsers::{bootconfig, grub};
use snafu::Snafu;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    #[snafu(display("Bootconfig data too short: {length} bytes"))]
    BinaryTooShort { length: usize },

    #[snafu(display("Invalid bootconfig magic"))]
    InvalidMagic,

    #[snafu(display("Bootconfig size {size} exceeds data length {length}"))]
    InvalidBinarySize { size: usize, length: usize },

    #[snafu(display("Bootconfig checksum mismatch: expected {expected}, calculated {actual}"))]
    ChecksumMismatch { expected: u32, actual: u32 },

    #[snafu(display("Invalid UTF-8 in {input}: {source}"))]
    InvalidUtf8 {
        input: &'static str,
        source: std::str::Utf8Error,
    },

    #[snafu(display("Failed to parse bootconfig: {source}"))]
    BootconfigParse {
        #[snafu(source(from(pest::error::Error<bootconfig::Rule>, Box::new)))]
        source: Box<pest::error::Error<bootconfig::Rule>>,
    },

    #[snafu(display("Bootconfig parameter '{name}' requires a value"))]
    MissingValue { name: String },

    #[snafu(display("Unsupported quote in '{name}'"))]
    UnsupportedQuote { name: String },

    #[snafu(display("Failed to parse grub.cfg: {source}"))]
    GrubParse {
        #[snafu(source(from(pest::error::Error<grub::Rule>, Box::new)))]
        source: Box<pest::error::Error<grub::Rule>>,
    },

    #[snafu(display("grub.cfg has no linux directive to boot"))]
    MissingGrubLinuxDirective,

    #[snafu(display("grub.cfg linux directive names no kernel"))]
    MissingGrubKernelPath,

    #[snafu(display("grub.cfg kernel path must be '{expected}', got: '{actual}'"))]
    InvalidGrubKernelPath {
        expected: &'static str,
        actual: String,
    },

    #[snafu(display("UKI command line has data after NUL"))]
    UkiDataAfterNul,

    #[snafu(display("Unsupported UKI command-line length: {length} bytes"))]
    InvalidUkiSectionLength { length: usize },

    #[snafu(display("UKI command line must be ASCII"))]
    InvalidLoadOptions,

    #[snafu(display("UKI command line must separate its arguments with single spaces"))]
    UncollapsedWhitespace,

    #[snafu(display("UKI command line has an unpaired quote"))]
    UnpairedQuote,
}
