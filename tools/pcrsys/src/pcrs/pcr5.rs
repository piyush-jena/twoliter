//! PCR 5: GPT partition table measurements.
//!
//! Generates predictions covering all valid GPT priority bit combinations:
//! - Dual-bank: 6 BOOT-A states × 6 BOOT-B states × 2 PRIVATE states = 72 combinations
//! - Single-bank: 6 BOOT-A states × 2 PRIVATE states = 12 combinations
//! - Direct UKI: no BOOT partitions, so 1 state from the unmodified GPT
//!
//! Platform differences:
//! - AWS/Metal: separator -> GPT -> exit actions
//! - VMware: Not predicted (EV_EFI_VARIABLE_BOOT events for BootOrder, Boot0000, Boot0001
//!   contain VM-specific device paths with MAC addresses that vary per VM instance)

use crate::boot::BootChain;
use crate::error::Result;
use crate::gpt::{GptPrio, PartitionInfo};
use crate::platform::Platform;
use crate::predict::{
    extend_pcr_data, extend_pcr_separator, extend_pcr_string, PcrContext, PcrDigests, PcrIndex,
    PcrRecord, PCR_INIT_VAL,
};

const ACTION_STRING_1: &str = "Exit Boot Services Invocation";
const ACTION_STRING_2: &str = "Exit Boot Services Returned with Success";

/// Priority bit combinations: (priority, tries_left, successful)
const COMBINATIONS: [(u8, u8, bool); 6] = [
    (0, 0, false), // inactive
    (0, 1, false), // inactive with tries
    (1, 0, true),  // priority 1, successful
    (2, 0, false), // priority 2, no tries
    (2, 0, true),  // priority 2, successful
    (2, 1, false), // priority 2, with tries
];

/// Predict PCR 5 values for all GPT state combinations.
///
/// Returns `None` for VMware since boot variables contain VM-specific data.
pub fn predict(ctx: &PcrContext) -> Result<Option<(PcrIndex, PcrRecord)>> {
    if ctx.platform == Platform::Vmware {
        return Ok(None);
    }

    let layout = match ctx.chain {
        // Direct UKI boot has no BOOT partitions, so nothing rewrites GPT attributes.
        BootChain::Uki(_) => {
            let record = PcrRecord::new(measure(&ctx.gpt.efi_gpt_data(&[])));
            return Ok(Some((PcrIndex::Pcr5, record)));
        }
        BootChain::GrubAb { layout, .. } | BootChain::GrubSingle { layout, .. } => layout,
    };

    let boot_b_combos: &[(u8, u8, bool)] = if layout.boot_b.is_some() {
        &COMBINATIONS
    } else {
        &[(0, 0, false)] // Single placeholder for single-bank
    };

    // A boot partition's attributes with its gptprio fields set, keeping its other bits.
    let gptprio = |partition: &PartitionInfo, (priority, tries, successful): (u8, u8, bool)| {
        let mut prio = GptPrio::from(ctx.gpt.attributes(partition));
        prio.set_priority(priority);
        prio.set_tries_left(tries);
        prio.set_successful(successful);
        (partition.number, u64::from(prio))
    };

    let mut digests = Vec::with_capacity(COMBINATIONS.len() * boot_b_combos.len() * 2);
    for private_bit57 in [false, true] {
        let mut private = GptPrio::from(ctx.gpt.attributes(&layout.private));
        private.set_boot_succeeded(private_bit57);
        for &boot_a in &COMBINATIONS {
            for &boot_b in boot_b_combos {
                let mut attributes = vec![gptprio(&layout.boot_a, boot_a)];
                if let Some(partition) = &layout.boot_b {
                    attributes.push(gptprio(partition, boot_b));
                }
                attributes.push((layout.private.number, u64::from(private)));
                digests.push(measure(&ctx.gpt.efi_gpt_data(&attributes)));
            }
        }
    }

    Ok(Some((PcrIndex::Pcr5, PcrRecord::new_multi(digests))))
}

/// Measure PCR 5 for one GPT state: separator -> EFI_GPT_DATA -> exit actions.
fn measure(efi_gpt_data: &[u8]) -> PcrDigests {
    let pcr = extend_pcr_separator(&PCR_INIT_VAL);
    let pcr = extend_pcr_data(&pcr, efi_gpt_data);
    let pcr = extend_pcr_string(&pcr, ACTION_STRING_1);
    extend_pcr_string(&pcr, ACTION_STRING_2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::efi::SecureBootVars;
    use crate::gpt::tests::{mock_disk_with_partitions, recalc_disk_crcs};
    use crate::gpt::Gpt;
    use crate::predict::test_support::{assert_pins, grub_chain, test_uki, MockCtx};

    /// The mock disk with valid CRCs, rpm2img's preset gptprio bits on BOOT-A, one unrelated
    /// attribute bit that every state must keep, and `count` BOOT partitions, as an image of
    /// that layout would be read.
    fn mock_ctx(count: usize) -> MockCtx {
        const ENTRIES: usize = 1024;
        let mut disk = mock_disk_with_partitions();
        let boot_a_attributes = ENTRIES + 128 + 48;
        let bits: u64 = (1 << 48) | (1 << 56) | (1 << 2);
        disk[boot_a_attributes..boot_a_attributes + 8].copy_from_slice(&bits.to_le_bytes());
        // Partitions 2 and 3 are BOOT-A and BOOT-B; clear the type of those not wanted.
        for entry in [2, 3].into_iter().skip(count) {
            let start = ENTRIES + (entry - 1) * 128;
            disk[start..start + 16].fill(0);
        }
        recalc_disk_crcs(&mut disk);
        let gpt = Gpt::read(&disk).unwrap();
        let chain = match count {
            0 => BootChain::Uki(test_uki()),
            _ => grub_chain(gpt.grub_layout().unwrap()),
        };
        MockCtx {
            efi_vars: SecureBootVars::default(),
            gpt,
            chain,
        }
    }

    fn record(count: usize, platform: Platform) -> Option<PcrRecord> {
        predict(&mock_ctx(count).build(platform))
            .unwrap()
            .map(|(index, record)| {
                assert_eq!(index, PcrIndex::Pcr5);
                record
            })
    }

    #[test]
    fn test_dual_bank_aws() {
        let record = record(2, Platform::Aws).unwrap();
        assert_eq!(record.sha256.len(), 72);
        assert_eq!(record.sha384.len(), 72);
    }

    #[test]
    fn test_single_bank_aws() {
        let record = record(1, Platform::Aws).unwrap();
        assert_eq!(record.sha256.len(), 12);
        assert_eq!(record.sha384.len(), 12);
    }

    #[test]
    fn test_uki_single_state() {
        // The single-bank GPT, measured as a UKI image measures it.
        let m = MockCtx {
            chain: BootChain::Uki(test_uki()),
            ..mock_ctx(1)
        };
        let uki = predict(&m.build(Platform::Aws)).unwrap().unwrap().1;
        assert_eq!(uki.sha256.len(), 1);
        assert_eq!(uki.sha384.len(), 1);

        // The unmodified GPT, with BOOT-A's preset priority 1 and successful bits, matches
        // one GRUB state.
        let grub = record(1, Platform::Aws).unwrap();
        let state = grub
            .sha256
            .iter()
            .position(|d| *d == uki.sha256[0])
            .unwrap();
        assert_eq!(grub.sha384[state], uki.sha384[0]);
    }

    #[test]
    fn test_vmware_not_predicted() {
        assert!(record(1, Platform::Vmware).is_none());
    }

    #[test]
    fn records_are_pinned() {
        let dual = record(2, Platform::Aws).unwrap();
        assert_eq!(dual.sha256.len(), 72);
        assert_eq!(record(2, Platform::Metal).unwrap(), dual);
        assert_pins(&[
            (
                "dual bank",
                &dual,
                "f08838f7b8a40fa3988eeaa898574124be07f4fc15a1b730d53e5f97133bb7f7",
            ),
            (
                "single bank",
                &record(1, Platform::Aws).unwrap(),
                "2228967d2349f4249eee02da7f3c229c18a49dd7076fa2764a0d5ef1730cabd8",
            ),
            (
                "uki",
                &record(0, Platform::Aws).unwrap(),
                "4d1dd91f1d6b249efd4ed93a0827d1537e5d7b9e6dd04e7bb11f86a5a6195151",
            ),
        ]);
    }
}
