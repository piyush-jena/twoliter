//! Core PCR prediction types and functions.

use serde::Serialize;

use std::collections::BTreeMap;

use sha2::{Digest, Sha256, Sha384};

use crate::efi::EfiVars;
use crate::gpt::PartitionLayout;
use crate::pe::UkiImage;
use crate::platform::Platform;

use crate::error::Result;

/// Digests for the two predicted TPM banks.
///
/// Keep the banks together so every event and alternative PCR state is measured
/// in both algorithms, using each bank's own digest width.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PcrDigests {
    pub sha256: [u8; 32],
    pub sha384: [u8; 48],
}

impl PcrDigests {
    pub fn hash(data: &[u8]) -> Self {
        Self {
            sha256: Sha256::digest(data).into(),
            sha384: Sha384::digest(data).into(),
        }
    }
}

/// Initial PCR values: one zero-filled digest per bank.
pub const PCR_INIT_VAL: PcrDigests = PcrDigests {
    sha256: [0; 32],
    sha384: [0; 48],
};

/// TPM Platform Configuration Register index.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum PcrIndex {
    Pcr0 = 0,
    Pcr1 = 1,
    Pcr2 = 2,
    Pcr3 = 3,
    Pcr4 = 4,
    Pcr5 = 5,
    Pcr6 = 6,
    Pcr7 = 7,
    #[allow(dead_code)]
    Pcr8 = 8,
    Pcr9 = 9,
    Pcr10 = 10,
    Pcr11 = 11,
    Pcr12 = 12,
    Pcr13 = 13,
    Pcr14 = 14,
    Pcr15 = 15,
}

impl Serialize for PcrIndex {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_u8(*self as u8)
    }
}

/// All extracted boot context needed for PCR predictions.
#[derive(bon::Builder)]
pub struct PcrContext<'a> {
    pub platform: Platform,
    pub efi_vars: &'a EfiVars,
    /// GRUB bank layout; direct UKI boot has no BOOT partitions.
    pub partitions: Option<&'a PartitionLayout>,
    /// Validated UKI payloads, when the firmware fallback is a UKI.
    pub uki: Option<&'a UkiImage<'a>>,
    #[builder(default)]
    pub gpt_bin: &'a [u8],
    #[builder(default)]
    pub shim: &'a [u8],
    #[builder(default)]
    pub grub: &'a [u8],
    #[builder(default)]
    pub vmlinuz: &'a [u8],
    #[builder(default)]
    pub grub_cfg: &'a [u8],
    #[builder(default)]
    pub bootconfig: &'a [u8],
    #[builder(default)]
    pub boot_partuuid: &'a str,
}

/// Collection of PCR predictions for JSON output.
#[derive(Default, Serialize)]
pub struct PcrPredictions {
    /// Map of PCR index to record.
    pub pcrs: BTreeMap<PcrIndex, PcrRecord>,
}

impl PcrPredictions {
    /// Create a new empty set of PCR predictions.
    pub fn new() -> Self {
        Self::default()
    }

    /// Try a prediction function; insert result if Some, propagate errors.
    ///
    /// This enables fluent chaining of PCR predictions.
    pub fn try_extend(
        mut self,
        f: impl FnOnce() -> Result<Option<(PcrIndex, PcrRecord)>>,
    ) -> Result<Self> {
        if let Some((index, record)) = f()? {
            self.pcrs.insert(index, record);
        }
        Ok(self)
    }
}

/// A single PCR prediction record.
#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct PcrRecord {
    /// SHA-256 digest values for this PCR.
    pub sha256: Vec<String>,
    /// SHA-384 digest values in the same state order as SHA-256.
    pub sha384: Vec<String>,
}

impl PcrRecord {
    /// Create a new PCR record with one state for both banks.
    pub fn new(digest: PcrDigests) -> Self {
        Self {
            sha256: vec![hex::encode(digest.sha256)],
            sha384: vec![hex::encode(digest.sha384)],
        }
    }

    /// Create a new PCR record with multiple states for both banks.
    pub fn new_multi(digests: Vec<PcrDigests>) -> Self {
        let (sha256, sha384) = digests
            .into_iter()
            .map(|digest| (hex::encode(digest.sha256), hex::encode(digest.sha384)))
            .unzip();
        Self { sha256, sha384 }
    }
}

/// Extend each bank with its own event digest: `H(current || digest)`.
pub fn extend_pcr(current: &PcrDigests, digest: &PcrDigests) -> PcrDigests {
    let mut hasher = Sha256::new();
    hasher.update(current.sha256);
    hasher.update(digest.sha256);
    let sha256 = hasher.finalize().into();
    let mut hasher = Sha384::new();
    hasher.update(current.sha384);
    hasher.update(digest.sha384);
    let sha384 = hasher.finalize().into();
    PcrDigests { sha256, sha384 }
}

/// Extend a PCR with an event payload by first hashing its bytes.
pub fn extend_pcr_data(current: &PcrDigests, data: &[u8]) -> PcrDigests {
    extend_pcr(current, &PcrDigests::hash(data))
}

/// Extend a PCR with a string by first hashing the string.
pub fn extend_pcr_string(current: &PcrDigests, s: &str) -> PcrDigests {
    extend_pcr_data(current, s.as_bytes())
}

/// Extend a PCR with the EV_SEPARATOR event (4 zero bytes).
pub fn extend_pcr_separator(current: &PcrDigests) -> PcrDigests {
    extend_pcr_data(current, &[0; 4])
}

#[cfg(test)]
pub mod test_support {
    use super::PcrContext;
    use crate::efi::EfiVars;
    use crate::gpt::{PartitionInfo, PartitionLayout};
    use crate::platform::Platform;

    /// SHA-256 of extending PCR init value with the EV_SEPARATOR event.
    pub const SEPARATOR_HASH: &str =
        "3d458cfe55cc03ea1f443f1562beec8df51c75e14a9fcf9a7234a13f198e7969";

    /// Create empty EFI variables for testing.
    pub fn mock_efi_vars() -> EfiVars {
        EfiVars { variables: vec![] }
    }

    /// Create a minimal partition layout for testing.
    pub fn mock_layout() -> PartitionLayout {
        PartitionLayout {
            efi_a: PartitionInfo {
                number: 1,
                start_lba: 0,
                end_lba: 1,
            },
            boot_a: PartitionInfo {
                number: 2,
                start_lba: 2,
                end_lba: 3,
            },
            boot_b: None,
            private: PartitionInfo {
                number: 3,
                start_lba: 4,
                end_lba: 5,
            },
        }
    }

    /// Create a dual-bank (A/B) partition layout for testing.
    pub fn mock_layout_dual_bank() -> PartitionLayout {
        PartitionLayout {
            efi_a: PartitionInfo {
                number: 1,
                start_lba: 0,
                end_lba: 1,
            },
            boot_a: PartitionInfo {
                number: 2,
                start_lba: 2,
                end_lba: 3,
            },
            boot_b: Some(PartitionInfo {
                number: 4,
                start_lba: 6,
                end_lba: 7,
            }),
            private: PartitionInfo {
                number: 3,
                start_lba: 4,
                end_lba: 5,
            },
        }
    }

    pub use crate::pe::tests::build_test_shim;

    /// Test context that owns EfiVars and PartitionLayout for convenient test setup.
    pub struct MockCtx {
        pub efi_vars: EfiVars,
        pub layout: PartitionLayout,
    }

    impl MockCtx {
        /// Create a new MockCtx with single-bank layout.
        pub fn new() -> Self {
            Self {
                efi_vars: mock_efi_vars(),
                layout: mock_layout(),
            }
        }

        /// Create a MockCtx with dual-bank (A/B) layout.
        pub fn dual_bank() -> Self {
            Self {
                efi_vars: mock_efi_vars(),
                layout: mock_layout_dual_bank(),
            }
        }

        /// Build a PcrContext with the given platform.
        pub fn build(&self, platform: Platform) -> PcrContext<'_> {
            PcrContext::builder()
                .platform(platform)
                .efi_vars(&self.efi_vars)
                .partitions(&self.layout)
                .build()
        }
    }
}

#[cfg(test)]
#[path = "test_predict.rs"]
mod tests;
