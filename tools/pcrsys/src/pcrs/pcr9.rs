//! PCR 9: Kernel Command Line and UKI initrd
//!
//! On GRUB images, rottweiler measures the kernel command line early in boot, which for
//! Bottlerocket consists of:
//! 1. Static parameters from grub.cfg
//! 2. Dynamic parameters from bootconfig.data (kernel.* and init.* sections)
//!
//! The final /proc/cmdline format is:
//! `<kernel.* params> BOOT_IMAGE=<path> <grub params before --> -- <init.* params> <grub params after -->`
//!
//! On aarch64, the kernel's EFI stub first measures the command line GRUB starts it with.
//!
//! Direct UKI boot instead measures EFI LoadOptions followed by the loaded initrd.

use crate::boot::BootChain;
use crate::error::Result;
use crate::pe::Arch;
use crate::predict::{
    extend_pcr_data, extend_pcr_string, PcrContext, PcrIndex, PcrRecord, PCR_INIT_VAL,
};
use bootconfig::predict_grub_cmdline as predict_cmdline;
use snafu::ResultExt;
use std::fmt::Write;

// The stub uses uppercase B in the trailer's namesize field. Preserve the exact
// bytes because the archive is measured into PCR 9.
const CPIO_TRAILER: &[u8] = b"07070100000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000B00000000TRAILER!!!\0\0\0\0";

/// Predict PCR 9 value.
///
/// GRUB images extend the /proc/cmdline contents, newline included, after the aarch64 LoadOptions.
/// UKI extends LoadOptions and the stub-generated os-release initrd.
pub fn predict(ctx: &PcrContext) -> Result<Option<(PcrIndex, PcrRecord)>> {
    let payload = match ctx.chain {
        BootChain::Uki(uki) => {
            // Linux EFI stub measures LoadOptions, then the complete loaded initrd.
            // The supported stub supplies only the generated os-release CPIO archive.
            let load_options = bootconfig::encode_uki_load_options(uki.cmdline()?)
                .whatever_context("failed to encode UKI load options")?;
            let pcr = extend_pcr_data(&PCR_INIT_VAL, &load_options);
            let pcr = extend_pcr_data(&pcr, &stub_generated_uki_initrd(uki.section(".osrel"))?);
            return Ok(Some((PcrIndex::Pcr9, PcrRecord::new(pcr))));
        }
        // In-place updates can change the command line over the lifetime of the instance.
        BootChain::GrubAb { .. } => return Ok(None),
        BootChain::GrubSingle { payload, .. } => payload,
    };

    let predicted = predict_cmdline(
        &payload.bootconfig,
        &payload.grub_cfg,
        &payload.boot_partuuid,
    )
    .whatever_context("failed to predict GRUB command line")?;
    let pcr9 = match payload.arch {
        // GRUB jumps to an aarch64 kernel's PE entry, whose EFI stub (Linux 6.1+) measures the
        // LoadOptions; it hands an x86_64 kernel over through EFI handover, which measures none.
        Arch::Aarch64 => extend_pcr_data(&PCR_INIT_VAL, &predicted.load_options),
        Arch::X86_64 => PCR_INIT_VAL,
    };
    let pcr9 = extend_pcr_string(&pcr9, &format!("{}\n", predicted.proc_cmdline));
    Ok(Some((PcrIndex::Pcr9, PcrRecord::new(pcr9))))
}

// systemd v257.13 pack_cpio_literal(".extra", "os-release", 0555, 0444).
//
// Archive layout (newc format):
//   [header + ".extra"]                          directory,    mode 040555
//   [header + ".extra/os-release" + .osrel data] regular file, mode 0100444
//   [TRAILER!!!]
fn stub_generated_uki_initrd(data: &[u8]) -> Result<Vec<u8>> {
    let mut out = cpio_entry(".extra", 1, 0o40555, &[])?;
    out.extend(cpio_entry(".extra/os-release", 2, 0o100444, data)?);
    out.extend_from_slice(CPIO_TRAILER);
    Ok(out)
}

/// Encode one newc CPIO entry: header and NUL-terminated name, then data, each 4-byte aligned.
fn cpio_entry(name: &str, inode: u32, mode: u32, data: &[u8]) -> Result<Vec<u8>> {
    let size = u32::try_from(data.len()).whatever_context("CPIO payload too large")?;
    // namesize counts the terminating NUL.
    let namesize = u32::try_from(name.len() + 1).whatever_context("CPIO name too long")?;

    // Header layout, e.g. for ".extra" (110 bytes):
    //   070701 00000001 0000416d 00000000 00000000 00000001 00000000
    //   magic  inode    mode     uid      gid      nlink    mtime
    //
    //   00000000 00000000 00000000 00000000 00000000 00000007 00000000
    //   filesize devmaj   devmin   rdevmaj  rdevmin  namesize check
    let mut header = String::from("070701");
    for word in [inode, mode, 0, 0, 1, 0, size, 0, 0, 0, 0, namesize, 0] {
        write!(&mut header, "{word:08x}").whatever_context("failed to format CPIO header")?;
    }
    let mut out = header.into_bytes();

    // NUL-terminated name, padded so the data starts 4-byte aligned.
    out.extend(name.as_bytes());
    out.push(0);
    out.resize(out.len().next_multiple_of(4), 0);

    // Data, padded so the next entry starts 4-byte aligned.
    out.extend(data);
    out.resize(out.len().next_multiple_of(4), 0);
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pe::tests::{build_test_uki, uki};
    use crate::platform::Platform;
    use crate::predict::test_support::{assert_pins, grub_single, MockCtx};
    use bootconfig::test_util::encode as make_bootconfig;
    use std::error::Error;
    use std::fs;
    use std::path::Path;
    use test_case::test_case;

    #[test]
    fn test_predict_includes_trailing_newline() {
        let bootconfig = make_bootconfig("");
        let m = MockCtx::with_chain(grub_single(
            Arch::X86_64,
            b"linux ($root)/vmlinuz -- x",
            &bootconfig,
            "",
        ));
        let result = predict(&m.build(Platform::Aws)).unwrap().unwrap();
        let cmdline_with_newline = "BOOT_IMAGE=/vmlinuz -- x\n";
        let expected = extend_pcr_string(&PCR_INIT_VAL, cmdline_with_newline);
        assert_eq!(result.1, PcrRecord::new(expected));
    }

    #[test]
    fn aarch64_first_measures_the_load_options_grub_sets() {
        let bootconfig = make_bootconfig("kernel.A = 1\n");
        let m = MockCtx::with_chain(grub_single(
            Arch::Aarch64,
            b"linux ($root)/vmlinuz quiet -- x",
            &bootconfig,
            "",
        ));
        let result = predict(&m.build(Platform::Aws)).unwrap().unwrap();
        // GRUB passes only its own arguments; the kernel then reports the bootconfig ones too.
        let load_options: Vec<u8> = "BOOT_IMAGE=/vmlinuz quiet -- x"
            .encode_utf16()
            .flat_map(u16::to_le_bytes)
            .collect();
        let expected = extend_pcr_data(&PCR_INIT_VAL, &load_options);
        let expected = extend_pcr_string(&expected, "A=1 BOOT_IMAGE=/vmlinuz quiet -- x\n");
        assert_eq!(result.1, PcrRecord::new(expected));
    }

    /// The grub.cfg and bootconfig.data of aws-dev 1.67.0 images, and the PCR 9 values their
    /// NitroTPMs reported when booted on m7g and m7i instances.
    #[test_case(
        Arch::Aarch64,
        "aws-dev-aarch64",
        "bd06db87-42ee-410f-8092-e7b1130b59bf",
        "0204dab16c5d7f1a624a3fb1f8a9d9b6e6530ae113d18fb4fcc1deb9ec976bac",
        "4d2d2c0beb185ff9fe3fb1d58f82af50dd376707429d97624f619df7a3166edb51c4e8d92a0747889d52397f5e700e0d"
        ; "aarch64"
    )]
    #[test_case(
        Arch::X86_64,
        "aws-dev-x86_64",
        "7c24a52f-2279-464a-9daa-c99427a296cb",
        "a6bdc18776ce74ccae172abdd5f5357a9f7333e36538a1a004ad49c04a60287a",
        "b4fe6ce24e5bca4bc43aadb9f87e6a1e102f1c8757d5786a255d41ce14384929628551b516aeb42ef904337eea480c29"
        ; "x86_64"
    )]
    fn ec2_measurements_are_predicted(
        arch: Arch,
        image: &str,
        boot_partuuid: &str,
        sha256: &str,
        sha384: &str,
    ) {
        let inputs = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/instances")
            .join(image);
        let grub_cfg = fs::read(inputs.join("grub.cfg")).unwrap();
        let bootconfig = fs::read(inputs.join("bootconfig.data")).unwrap();
        let m = MockCtx::with_chain(grub_single(arch, &grub_cfg, &bootconfig, boot_partuuid));
        let record = predict(&m.build(Platform::Aws)).unwrap().unwrap().1;
        assert_eq!(record.sha256, [sha256]);
        assert_eq!(record.sha384, [sha384]);
    }

    #[test]
    fn test_predict_retains_bootconfig_error_source() {
        let m = MockCtx::with_chain(grub_single(
            Arch::X86_64,
            b"linux ($root)/vmlinuz -- x",
            b"invalid bootconfig",
            "",
        ));
        let error = predict(&m.build(Platform::Aws)).unwrap_err();
        assert_eq!(error.to_string(), "failed to predict GRUB command line");
        assert!(matches!(
            error.source().unwrap().downcast_ref::<bootconfig::Error>(),
            Some(bootconfig::Error::BinaryTooShort { .. })
        ));
    }

    #[test]
    fn test_predict_retains_load_options_error_source() {
        let mut image = build_test_uki(0x8664);
        // The fixture's first payload is .cmdline at byte 1024. Valid UTF-8
        // survives section decoding, but non-ASCII LoadOptions are unsupported.
        image[1024..1026].copy_from_slice("é".as_bytes());
        let m = MockCtx::with_chain(BootChain::Uki(uki(image).unwrap()));
        let error = predict(&m.build(Platform::Aws)).unwrap_err();
        assert_eq!(error.to_string(), "failed to encode UKI load options");
        assert!(matches!(
            error.source().unwrap().downcast_ref::<bootconfig::Error>(),
            Some(bootconfig::Error::InvalidLoadOptions)
        ));
    }

    #[test]
    fn test_predict_skipped_for_ab() {
        let m = MockCtx::dual_bank();
        assert!(predict(&m.build(Platform::Aws)).unwrap().is_none());
    }

    #[test]
    fn records_are_pinned() {
        // Shaped like the grub.cfg rpm2img writes, with GRUB's $boot_uuid placeholder.
        let grub_cfg = br#"set default="0"
set timeout="0"
set dm_verity_root="root,,,ro,0 123 verity 1 PARTUUID=$boot_uuid/PARTNROFF=1 PARTUUID=$boot_uuid/PARTNROFF=2 4096 4096 15 1 sha256 aa bb"

menuentry "Bottlerocket OS 1.67.0" --unrestricted {
   linux ($root)/vmlinuz \
       console=tty0 \
       bootconfig \
       root=/dev/dm-0 rootwait ro \
       dm-mod.create="$dm_verity_root" \
       -- \
       systemd.log_target=journal-or-kmsg
   initrd ($private)/bootconfig.data
   boot
}
"#;
        let bootconfig = make_bootconfig(
            "kernel.module_blacklist = i8042\ninit.systemd.unit = multi-user.target\n",
        );
        let grub_record = |arch| {
            let grub = MockCtx::with_chain(grub_single(
                arch,
                grub_cfg,
                &bootconfig,
                "bd06db87-42ee-410f-8092-e7b1130b59bf",
            ));
            predict(&grub.build(Platform::Aws)).unwrap().unwrap().1
        };
        let uki = MockCtx::uki();
        let uki_record = predict(&uki.build(Platform::Aws)).unwrap().unwrap().1;
        assert_pins(&[
            (
                "grub",
                &grub_record(Arch::X86_64),
                "8a70896a6316b2d08ae323426143acb070072c596e6eb9aa28b890e6f6cc53f1",
            ),
            (
                "grub aarch64",
                &grub_record(Arch::Aarch64),
                "a2b4d0757ae94797ab6661762b15f4a942ad3b6888cfec0399d3ccda7344564b",
            ),
            (
                "uki",
                &uki_record,
                "9d5b2476117ba4f8fa895571eabededa266ab4dfe7146f869bc178afd847fc32",
            ),
        ]);
    }
}
