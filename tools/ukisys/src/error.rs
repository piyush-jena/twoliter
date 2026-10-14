//! Common error handling for ukisys.

use crate::pe::PeError;
use snafu::Snafu;
use std::path::PathBuf;

/// Result type for ukisys CLI-level operations.
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    #[snafu(display("Failed to read '{}': {source}", path.display()))]
    FileRead {
        path: PathBuf,
        source: std::io::Error,
    },

    #[snafu(display("Failed to write '{}': {source}", path.display()))]
    FileWrite {
        path: PathBuf,
        source: std::io::Error,
    },

    #[snafu(display("Failed to {operation}: {source}"))]
    Bootconfig {
        operation: &'static str,
        source: bootconfig::Error,
    },

    #[snafu(display("Failed to parse UKI '{}': {source}", path.display()))]
    UkiRead { path: PathBuf, source: PeError },

    #[snafu(display("Failed to remove UKI signature: {source}"))]
    RemoveSignature { source: PeError },

    #[snafu(display("Failed to derive systemd stub: {source}"))]
    DeriveStub { source: PeError },

    #[snafu(display("Failed to write stub '{}': {source}", path.display()))]
    StubWrite { path: PathBuf, source: PeError },
}
