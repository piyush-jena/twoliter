//! PCR 4: Boot Manager Code (shim, grub, vmlinuz or direct UKI)

use crate::boot::BootChain;
use crate::error::Result;
use crate::pe::get_authenticode_hash;
use crate::platform::Platform;
use crate::predict::{
    extend_pcr, extend_pcr_separator, extend_pcr_string, PcrContext, PcrIndex, PcrRecord,
    PCR_INIT_VAL,
};

const ACTION_STRING: &str = "Calling EFI Application from Boot Option";

/// Predict PCR 4 value.
///
/// AWS/Metal extend an action string before the separator, VMware does not.
///
/// AWS/Metal: GRUB variant - action -> separator -> shim -> grub -> vmlinuz
/// AWS:       UKI variant  - action -> separator -> UKI image
/// VMware:              separator -> shim -> grub -> vmlinuz
pub fn predict(ctx: &PcrContext) -> Result<Option<(PcrIndex, PcrRecord)>> {
    let images = match ctx.chain {
        // In-place updates can change the kernel over the lifetime of the instance.
        BootChain::GrubAb { .. } => return Ok(None),
        // Firmware measures the UKI, not its embedded kernel separately.
        BootChain::Uki(uki) => vec![get_authenticode_hash(uki.data())?],
        BootChain::GrubSingle { payload, .. } => {
            vec![payload.shim, payload.grub, payload.vmlinuz]
        }
    };

    // AWS/Metal: action string first, VMware: start with zeros
    let start = match ctx.platform {
        Platform::Aws | Platform::Metal => extend_pcr_string(&PCR_INIT_VAL, ACTION_STRING),
        Platform::Vmware => PCR_INIT_VAL,
    };
    let pcr = images
        .iter()
        .fold(extend_pcr_separator(&start), |pcr, image| {
            extend_pcr(&pcr, image)
        });
    Ok(Some((PcrIndex::Pcr4, PcrRecord::new(pcr))))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pe::tests::build_test_uki;
    use crate::predict::test_support::{assert_pins, MockCtx};

    #[test]
    fn test_predict_aws() {
        let m = MockCtx::new();
        let result = predict(&m.build(Platform::Aws)).unwrap().unwrap();
        assert_eq!(result.0, PcrIndex::Pcr4);
        assert_eq!(
            result.1.sha256[0],
            "d24f8ffb30337ca20444f47508aa1d39dbc9c5f718d8fb022b471fb97377440e"
        );
    }

    #[test]
    fn test_predict_vmware() {
        let m = MockCtx::new();
        let result = predict(&m.build(Platform::Vmware)).unwrap().unwrap();
        assert_eq!(result.0, PcrIndex::Pcr4);
        assert_eq!(
            result.1.sha256[0],
            "2b9aca375ae525f79e24af739876fc92086d165a0313a394c1571c092321f0f7"
        );
    }

    #[test]
    fn test_predict_skipped_for_ab() {
        let m = MockCtx::dual_bank();
        assert!(predict(&m.build(Platform::Aws)).unwrap().is_none());
    }

    #[test]
    fn test_predict_uki_measures_the_whole_image() {
        let m = MockCtx::uki();
        let result = predict(&m.build(Platform::Aws)).unwrap().unwrap();
        let pcr = extend_pcr_separator(&extend_pcr_string(&PCR_INIT_VAL, ACTION_STRING));
        let image = build_test_uki(0x8664);
        let expected = extend_pcr(&pcr, &get_authenticode_hash(&image).unwrap());
        assert_eq!(result.1, PcrRecord::new(expected));
    }

    #[test]
    fn records_are_pinned() {
        let grub = MockCtx::new();
        let grub = |platform| predict(&grub.build(platform)).unwrap().unwrap().1;
        let uki = MockCtx::uki();
        let uki_record = predict(&uki.build(Platform::Aws)).unwrap().unwrap().1;
        assert_pins(&[
            (
                "grub aws",
                &grub(Platform::Aws),
                "7436a3d2f921c10ce786baf14e85e1923384658685ef03cf8453d7c0266721fe",
            ),
            (
                "grub vmware",
                &grub(Platform::Vmware),
                "212d1382f9c6c844772a87b1dacfa43e302a645f73a2f1cc8c1d1a48af0d9f30",
            ),
            (
                "grub metal",
                &grub(Platform::Metal),
                "7436a3d2f921c10ce786baf14e85e1923384658685ef03cf8453d7c0266721fe",
            ),
            (
                "uki aws",
                &uki_record,
                "82d0c62b1c0bf1d1ef3f47a5f6b65891247395060933407395e426e407707b4b",
            ),
        ]);
    }
}
