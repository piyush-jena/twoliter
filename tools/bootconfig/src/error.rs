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

    #[snafu(display(
        "Invalid standalone bootconfig size or alignment: declared {size}, file length {length}"
    ))]
    InvalidBinarySize { size: usize, length: usize },

    #[snafu(display("Bootconfig checksum mismatch: expected {expected}, calculated {actual}"))]
    ChecksumMismatch { expected: u32, actual: u32 },

    #[snafu(display("Invalid bootconfig NUL terminator or padding"))]
    InvalidPadding,

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

    #[snafu(display("Failed to parse grub.cfg: {source}"))]
    GrubParse {
        #[snafu(source(from(pest::error::Error<grub::Rule>, Box::new)))]
        source: Box<pest::error::Error<grub::Rule>>,
    },

    #[snafu(display("Bootconfig kernel and init must be namespaces"))]
    InvalidNamespace,

    #[snafu(display("Module blacklist drop-in must contain only its assignment"))]
    InvalidBlacklistStatement,

    #[snafu(display("Module blacklist requires a value"))]
    MissingBlacklistValue,

    #[snafu(display("Invalid module blacklist name '{name}'"))]
    InvalidModuleName { name: String },

    #[snafu(display("Module blacklist must be a single-line assignment"))]
    InvalidBlacklistForm,

    #[snafu(display("Unsupported quote in '{name}'"))]
    UnsupportedQuote { name: String },

    #[snafu(display("grub.cfg kernel path must start with '{expected}', got: {actual}"))]
    InvalidGrubKernelPath {
        expected: &'static str,
        actual: String,
    },

    #[snafu(display("UKI command line exceeds kernel buffer: {length} bytes"))]
    UkiCommandLineTooLong { length: usize },

    #[snafu(display("UKI command line must be ASCII without embedded NUL"))]
    InvalidLoadOptions,

    #[snafu(display("UKI command line has data after NUL"))]
    UkiDataAfterNul,

    #[snafu(display("Unsupported UKI command-line length: {length} bytes"))]
    InvalidUkiSectionLength { length: usize },
}
