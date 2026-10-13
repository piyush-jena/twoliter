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
