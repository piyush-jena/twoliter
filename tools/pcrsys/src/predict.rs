//! Core PCR prediction types and functions.

use serde::Serialize;

use std::collections::BTreeMap;

use sha2::{Digest, Sha256, Sha384};

use crate::boot::BootChain;
use crate::efi::SecureBootVars;
use crate::gpt::Gpt;
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

/// Everything read from the disk and the EFI variables that PCR predictions need.
pub struct PcrContext<'a> {
    pub platform: Platform,
    pub efi_vars: &'a SecureBootVars,
    pub gpt: &'a Gpt,
    pub chain: &'a BootChain,
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
    use super::{PcrContext, PcrRecord};
    use crate::boot::{BootChain, GrubPayload};
    use crate::efi::SecureBootVars;
    use crate::gpt::tests::mock_disk_with_partitions;
    use crate::gpt::{Gpt, PartitionLayout};
    use crate::pe::tests::{build_test_shim, build_test_uki, shim, uki};
    use crate::pe::{get_authenticode_hash, Arch, Shim, Uki};
    use crate::platform::Platform;
    use sha2::{Digest, Sha256};

    /// SHA-256 of extending PCR init value with the EV_SEPARATOR event.
    pub const SEPARATOR_HASH: &str =
        "3d458cfe55cc03ea1f443f1562beec8df51c75e14a9fcf9a7234a13f198e7969";

    /// One digest over a record's JSON, pinning every state of both banks.
    pub fn pin(record: &PcrRecord) -> String {
        hex::encode(Sha256::digest(
            serde_json::to_vec(record).expect("records serialize"),
        ))
    }

    /// Check every `(label, record, expected pin)`, reporting all mismatches together.
    pub fn assert_pins(cases: &[(&str, &PcrRecord, &str)]) {
        let mismatches: Vec<String> = cases
            .iter()
            .filter(|(_, record, expected)| pin(record) != *expected)
            .map(|(label, record, _)| format!("{label}: {}", pin(record)))
            .collect();
        assert!(
            mismatches.is_empty(),
            "pins changed:\n{}",
            mismatches.join("\n")
        );
    }

    /// The test shim, parsed.
    pub fn test_shim() -> Shim {
        shim(build_test_shim()).expect("the test shim parses")
    }

    /// The test UKI, parsed.
    pub fn test_uki() -> Uki {
        uki(build_test_uki(0x8664)).expect("the test UKI parses")
    }

    /// The mock disk's GPT: an ESP, BOOT-A, BOOT-B and PRIVATE.
    pub fn mock_gpt() -> Gpt {
        Gpt::read(&mock_disk_with_partitions()).expect("the mock disk has a valid GPT")
    }

    /// The mock disk's GRUB partitions, with or without BOOT-B.
    pub fn mock_layout(dual_bank: bool) -> PartitionLayout {
        let mut layout = mock_gpt()
            .grub_layout()
            .expect("the mock disk has GRUB partitions");
        if !dual_bank {
            layout.boot_b = None;
        }
        layout
    }

    /// A single-bank chain whose shim, GRUB and kernel are all the test shim.
    pub fn grub_single(
        arch: Arch,
        grub_cfg: &[u8],
        bootconfig: &[u8],
        boot_partuuid: &str,
    ) -> BootChain {
        let digest = get_authenticode_hash(&build_test_shim()).expect("the test shim hashes");
        BootChain::GrubSingle {
            shim: test_shim(),
            layout: mock_layout(false),
            payload: Box::new(GrubPayload {
                shim: digest,
                grub: digest,
                vmlinuz: digest,
                arch,
                grub_cfg: grub_cfg.to_vec(),
                bootconfig: bootconfig.to_vec(),
                boot_partuuid: boot_partuuid.to_string(),
            }),
        }
    }

    /// An A/B chain with the test shim.
    pub fn grub_ab() -> BootChain {
        grub_chain(mock_layout(true))
    }

    /// A GRUB chain over `layout`, A/B when it has a BOOT-B.
    pub fn grub_chain(layout: PartitionLayout) -> BootChain {
        if layout.boot_b.is_some() {
            return BootChain::GrubAb {
                shim: test_shim(),
                layout,
            };
        }
        match grub_single(Arch::X86_64, b"", b"", "") {
            BootChain::GrubSingle { shim, payload, .. } => BootChain::GrubSingle {
                shim,
                layout,
                payload,
            },
            _ => unreachable!("grub_single builds a single-bank chain"),
        }
    }

    /// Owns what a test's `PcrContext` borrows.
    pub struct MockCtx {
        pub efi_vars: SecureBootVars,
        pub gpt: Gpt,
        pub chain: BootChain,
    }

    impl MockCtx {
        /// A single-bank GRUB image with no Secure Boot variables.
        pub fn new() -> Self {
            Self::with_chain(grub_single(Arch::X86_64, b"", b"", ""))
        }

        /// A dual-bank (A/B) GRUB image with no Secure Boot variables.
        pub fn dual_bank() -> Self {
            Self::with_chain(grub_ab())
        }

        /// A UKI image with no Secure Boot variables.
        pub fn uki() -> Self {
            Self::with_chain(BootChain::Uki(test_uki()))
        }

        /// The mock GPT and `chain`, with no Secure Boot variables.
        pub fn with_chain(chain: BootChain) -> Self {
            Self {
                efi_vars: SecureBootVars::default(),
                gpt: mock_gpt(),
                chain,
            }
        }

        /// Build a PcrContext with the given platform.
        pub fn build(&self, platform: Platform) -> PcrContext<'_> {
            PcrContext {
                platform,
                efi_vars: &self.efi_vars,
                gpt: &self.gpt,
                chain: &self.chain,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    // Fixed expectations computed with Python hashlib, independent of the Rust
    // implementation: H(zero_digest || H(event)).
    #[test_case(
        b"test",
        "516caf854bba78a30ba2a84f9e400642c01c1a3fa429268ff5b47c32a655d4b3",
        "e6c7526759cbdca9a11ba0bf7efe6d2193308532a85ba7e969889200de19583f4d3746983b22bf4638d571ec8aeabb85";
        "text_event"
    )]
    #[test_case(
        &[0; 4],
        "3d458cfe55cc03ea1f443f1562beec8df51c75e14a9fcf9a7234a13f198e7969",
        "518923b0f955d08da077c96aaba522b9decede61c599cea6c41889cfbea4ae4d50529d96fe4d1afdafb65e7f95bf23c4";
        "separator_event"
    )]
    fn event_extends_each_bank_at_its_own_width(event: &[u8], sha256: &str, sha384: &str) {
        let result = extend_pcr_data(&PCR_INIT_VAL, event);
        assert_eq!(hex::encode(result.sha256), sha256);
        assert_eq!(hex::encode(result.sha384), sha384);
    }

    #[test]
    fn records_serialize_both_banks_in_state_order() {
        let first = PcrDigests {
            sha256: [0xab; 32],
            sha384: [0xcd; 48],
        };
        let second = PcrDigests {
            sha256: [0xef; 32],
            sha384: [0x12; 48],
        };
        let records = [
            PcrRecord::new(first),
            PcrRecord::new_multi(vec![first, second]),
        ];
        for (record, count) in records.into_iter().zip([1, 2]) {
            let expected256 = ["ab".repeat(32), "ef".repeat(32)];
            let expected384 = ["cd".repeat(48), "12".repeat(48)];
            assert_eq!(
                serde_json::to_value(record).unwrap(),
                serde_json::json!({
                    "sha256": expected256[..count],
                    "sha384": expected384[..count],
                })
            );
        }
    }
}
