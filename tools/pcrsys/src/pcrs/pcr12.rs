//! PCR 12: Zero (unused)

use crate::error::Result;
use crate::predict::{PcrContext, PcrIndex, PcrRecord, PCR_INIT_VAL};

/// Predict PCR 12 value (always zero, unused).
pub fn predict(ctx: &PcrContext) -> Result<Option<(PcrIndex, PcrRecord)>> {
    // LoadOptions include the selected bank UUIDs and its fixed root hash.
    if ctx.systemd_boot_ab {
        return Ok(None);
    }

    Ok(Some((PcrIndex::Pcr12, PcrRecord::new(PCR_INIT_VAL))))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::platform::Platform;
    use crate::predict::test_support::MockCtx;

    #[test]
    fn systemd_boot_ab_does_not_emit_a_legacy_prediction() {
        let m = MockCtx::dual_bank();
        let ctx = PcrContext::builder()
            .platform(crate::platform::Platform::Aws)
            .efi_vars(&m.efi_vars)
            .partitions(&m.layout)
            .systemd_boot_ab(true)
            .build();
        assert!(predict(&ctx).unwrap().is_none());
    }

    #[test]
    fn test_predict() {
        let m = MockCtx::new();
        let ctx = m.build(Platform::Aws);
        let result = predict(&ctx).unwrap().unwrap();
        assert_eq!(result.0, PcrIndex::Pcr12);
        assert_eq!(result.1.sha256.len(), 1);
        assert_eq!(
            result.1.sha256[0],
            "0000000000000000000000000000000000000000000000000000000000000000"
        );
    }
}
