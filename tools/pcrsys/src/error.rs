//! Common error handling for pcrsys.

use snafu::Snafu;

/// A pcrsys error: a message, and the error that caused it, if any. Unlike `snafu::Whatever`, it is
/// `Send` and `Sync`, so that users of the library, such as twoliter, can pass it on.
#[derive(Debug, Snafu)]
#[snafu(whatever, display("{message}"))]
pub struct Error {
    message: String,
    #[snafu(source(from(Box<dyn std::error::Error + Send + Sync>, Some)))]
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

/// Result type for pcrsys operations.
pub type Result<T> = std::result::Result<T, Error>;
