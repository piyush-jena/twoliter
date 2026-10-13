use crate::pe::{get_authenticode_hash, tests::build_test_uki, UkiImage};
use crate::platform::Platform;
use crate::predict::{test_support::MockCtx, PcrContext};
use crate::{pcrs, pe};
use test_case::test_case;

#[test_case("x86_64", 0x8664)]
#[test_case("aarch64", 0xaa64)]
fn uki_and_grub_match_independent_bank_vectors(arch: &str, machine: u16) {
    let vectors: serde_json::Value =
        serde_json::from_str(include_str!("../tests/data/bank-vectors.json")).unwrap();
    let image = build_test_uki(machine);
    let uki = UkiImage::parse(&image, machine).unwrap().unwrap();
    let vars = pe::tests::efi_vars();
    let ctx = PcrContext::builder()
        .platform(Platform::Aws)
        .efi_vars(&vars)
        .uki(&uki)
        .build();
    let authenticode = get_authenticode_hash(&image).unwrap();
    assert_eq!(
        hex::encode(authenticode.sha256),
        vectors[arch]["sha256"]["authenticode"]
    );
    assert_eq!(
        hex::encode(authenticode.sha384),
        vectors[arch]["sha384"]["authenticode"]
    );
    let uki_records = [
        pcrs::pcr4::predict(&ctx),
        pcrs::pcr7::predict(&ctx),
        pcrs::pcr9::predict(&ctx),
        pcrs::pcr11::predict(&ctx),
    ];
    let mock = MockCtx::new();
    let shim = pe::tests::build_test_shim();
    let ctx = PcrContext::builder()
        .platform(Platform::Aws)
        .efi_vars(&vars)
        .partitions(&mock.layout)
        .shim(&shim)
        .grub_cfg(b"linux ($root)/vmlinuz -- x")
        .bootconfig(b"\0\0\0\0\x04\0\0\0\0\0\0\0#BOOTCONFIG\n")
        .build();
    let grub_records = [
        pcrs::pcr7::predict(&ctx),
        pcrs::pcr9::predict(&ctx),
        pcrs::pcr11::predict(&ctx),
        pcrs::pcr14::predict(&ctx),
    ];
    for (path, records) in [("uki", uki_records), ("grub", grub_records)] {
        for result in records {
            let (index, record) = result.unwrap().unwrap();
            let index = (index as u8).to_string();
            let record = serde_json::to_value(record).unwrap();
            for bank in ["sha256", "sha384"] {
                assert_eq!(
                    record[bank], vectors[arch][bank][path][&index],
                    "{arch} {path} PCR{index} {bank}"
                );
            }
        }
    }
}
