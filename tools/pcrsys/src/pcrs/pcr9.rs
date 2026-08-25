//! PCR 9: Kernel Command Line
//!
//! PCR 9 measures the kernel command line. Bottlerocket has two boot chains,
//! each of which measures PCR 9 differently:
//!
//! **GRUB images** — the running kernel measures `/proc/cmdline` at runtime
//! (via `measure-cmdline.service`/rottweiler). `/proc/cmdline` is assembled by
//! the kernel from:
//! 1. Static parameters from grub.cfg
//! 2. Dynamic parameters from bootconfig.data (kernel.* and init.* sections)
//!
//! The final `/proc/cmdline` format is:
//! `<kernel.* params> BOOT_IMAGE=<path> <grub params before --> -- <init.* params> <grub params after -->`
//!
//! **UKI images** — `measure-cmdline.service` is skipped and there is no GRUB
//! boot chain. Instead the **Linux kernel EFI stub** extends PCR 9 with two
//! `EV_EVENT_TAG` events, in order:
//!   1. `LOADED_IMAGE::LoadOptions` — `SHA256` of the kernel command line as the
//!      raw UEFI `LoadOptions` buffer, i.e. the `.cmdline` PE section encoded as
//!      UTF-16LE **including its terminating NUL word** (systemd-stub sets
//!      `LoadOptions`/`LoadOptionsSize = strsize16(cmdline)`).
//!   2. `Linux initrd` — `SHA256` of the combined initrd blob systemd-stub
//!      serves to the kernel via the `LINUX_EFI_INITRD_MEDIA_GUID` LoadFile2
//!      protocol. For a Bottlerocket UKI that blob is the single cpio archive
//!      systemd-stub synthesizes from the `.osrel` section (`/.extra/os-release`);
//!      there are no microcode/base-initrd/PCR-signature/profile sections and no
//!      ESP credentials or sysexts. See [`predict_uki`].
//!
//! So `PCR9(UKI) = extend(extend(0, SHA256(load_options)), SHA256(initrd))`,
//! measured by the kernel stub (NOT `SHA256(/proc/cmdline)` as on GRUB).
//!
//! For GRUB the running kernel measures `SHA256(/proc/cmdline + "\n")` extended
//! into PCR 9 from the zero init value.

use crate::error::Result;
use crate::parsers::{bootconfig, grub};
use crate::pe;
use crate::predict::{extend_pcr, extend_pcr_string, PcrContext, PcrIndex, PcrRecord, PCR_INIT_VAL};
use sha2::{Digest, Sha256};
use snafu::{whatever, OptionExt};

const KERNEL_PATH_PREFIX: &str = "()/vmlinuz ";

/// Transform grub.cfg shell-style quoting `key="value"` to kernel cmdline format `"key=value"`.
///
/// grub.cfg uses shell-style quoting where values are quoted: `root="UUID=abc"`
/// The kernel command line expects the entire key=value pair quoted: `"root=UUID=abc"`
/// This function performs that transformation for PCR 9 prediction.
fn repair_quotes(cmdline: &str) -> String {
    let mut result = String::with_capacity(cmdline.len());
    let mut chars = cmdline.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '=' && chars.peek() == Some(&'"') {
            // Found `="`; scan back to find start of key
            let key_start = result.rfind(' ').map(|i| i + 1).unwrap_or(0);
            let key = result[key_start..].to_string();
            result.truncate(key_start);

            // Skip the opening quote
            chars.next();

            // Collect value until closing quote
            let mut value = String::new();
            for vc in chars.by_ref() {
                if vc == '"' {
                    break;
                }
                value.push(vc);
            }

            // Output as "key=value"
            result.push('"');
            result.push_str(&key);
            result.push('=');
            result.push_str(&value);
            result.push('"');
        } else {
            result.push(c);
        }
    }
    result
}

/// Predict /proc/cmdline from grub.cfg and bootconfig.
///
/// The kernel constructs /proc/cmdline as:
/// `<kernel.* bootconfig> BOOT_IMAGE=<path> <grub args> -- <init.* bootconfig> <grub args after -->`
///
/// If `boot_partuuid` is provided, replaces `PARTUUID=/PARTNROFF=` with the actual UUID.
fn predict_cmdline(
    grub_cfg: &[u8],
    bootconfig_data: &[u8],
    boot_partuuid: Option<&str>,
) -> Result<String> {
    let grub_cmdline = grub::parse(grub_cfg)?;
    let bootconfig_params = bootconfig::parse(bootconfig_data)?;
    let kernel_params = bootconfig::format_params(&bootconfig_params.kernel);
    let init_params = bootconfig::format_params(&bootconfig_params.init);

    // Verify and transform kernel path
    if !grub_cmdline.starts_with(KERNEL_PATH_PREFIX) {
        whatever!(
            "grub.cfg kernel path must start with '{}', got: {}",
            KERNEL_PATH_PREFIX.trim(),
            &grub_cmdline[..grub_cmdline.len().min(20)]
        );
    }
    let mut grub_args = grub_cmdline.replacen(KERNEL_PATH_PREFIX, "BOOT_IMAGE=/vmlinuz ", 1);

    // Substitute PARTUUID placeholder with actual boot partition UUID
    if let Some(uuid) = boot_partuuid {
        grub_args = grub_args.replace(
            "PARTUUID=/PARTNROFF=",
            &format!("PARTUUID={uuid}/PARTNROFF="),
        );
    }

    // Apply kernel's quote repair transformation to grub args
    grub_args = repair_quotes(&grub_args);

    // Split grub args at "--"
    let (before_sep, after_sep) = if let Some(pos) = grub_args.find(" -- ") {
        (&grub_args[..pos], &grub_args[pos + 4..])
    } else {
        (grub_args.as_str(), "")
    };

    // Construct final cmdline
    let mut cmdline = String::new();
    cmdline.push_str(&kernel_params);
    cmdline.push_str(before_sep);
    cmdline.push_str(" -- ");
    cmdline.push_str(&init_params);
    cmdline.push_str(after_sep);

    Ok(cmdline)
}

/// Append an 8-character lowercase-hex cpio (newc) header field.
fn cpio_word(buf: &mut Vec<u8>, v: u32) {
    buf.extend_from_slice(format!("{v:08x}").as_bytes());
}

/// Pad `buf` with NUL bytes until its length is a multiple of 4.
fn cpio_pad4(buf: &mut Vec<u8>) {
    while buf.len() % 4 != 0 {
        buf.push(0);
    }
}

/// Append a cpio (newc) directory inode. Mirrors systemd `pack_cpio_dir`.
fn cpio_dir(buf: &mut Vec<u8>, path: &str, mode: u32, inode: &mut u32) {
    buf.extend_from_slice(b"070701"); // magic
    cpio_word(buf, *inode);
    *inode += 1;
    cpio_word(buf, mode | 0o040000); // S_IFDIR
    cpio_word(buf, 0); // uid
    cpio_word(buf, 0); // gid
    cpio_word(buf, 1); // nlink
    cpio_word(buf, 0); // mtime (0 for stable measurements)
    cpio_word(buf, 0); // filesize
    cpio_word(buf, 0); // dev major
    cpio_word(buf, 0); // dev minor
    cpio_word(buf, 0); // rdev major
    cpio_word(buf, 0); // rdev minor
    cpio_word(buf, path.len() as u32 + 1); // namesize (incl NUL)
    cpio_word(buf, 0); // crc
    buf.extend_from_slice(path.as_bytes());
    buf.push(0);
    cpio_pad4(buf);
}

/// Append a cpio (newc) regular-file inode at `prefix/filename`. Mirrors
/// systemd `pack_cpio_one`.
fn cpio_file(buf: &mut Vec<u8>, prefix: &str, filename: &str, contents: &[u8], mode: u32, inode: &mut u32) {
    buf.extend_from_slice(b"070701"); // magic
    cpio_word(buf, *inode);
    *inode += 1;
    cpio_word(buf, mode | 0o100000); // S_IFREG
    cpio_word(buf, 0); // uid
    cpio_word(buf, 0); // gid
    cpio_word(buf, 1); // nlink
    cpio_word(buf, 0); // mtime
    cpio_word(buf, contents.len() as u32); // filesize
    cpio_word(buf, 0); // dev major
    cpio_word(buf, 0); // dev minor
    cpio_word(buf, 0); // rdev major
    cpio_word(buf, 0); // rdev minor
    cpio_word(buf, (prefix.len() + filename.len() + 2) as u32); // prefix + '/' + name + NUL
    cpio_word(buf, 0); // crc
    buf.extend_from_slice(prefix.as_bytes());
    buf.push(b'/');
    buf.extend_from_slice(filename.as_bytes());
    buf.push(0);
    cpio_pad4(buf);
    buf.extend_from_slice(contents);
    cpio_pad4(buf);
}

/// Append the fixed cpio `TRAILER!!!` record. Mirrors systemd
/// `pack_cpio_trailer` byte-for-byte.
///
/// NOTE: systemd hard-codes this record as a C string literal rather than
/// generating it with `write_cpio_word`, so its namesize field is the
/// **uppercase** hex `0000000B` (= 11). This differs from every other cpio
/// field (which are lowercase) and is load-bearing: it changes the SHA-256 of
/// the initrd and therefore the measured PCR 9. The literal also carries three
/// explicit trailing NULs plus the implicit string terminator, keeping the
/// record 4-byte aligned (124 bytes total).
fn cpio_trailer(buf: &mut Vec<u8>) {
    buf.extend_from_slice(b"070701"); // magic
    buf.extend_from_slice(b"00000000"); // inode
    buf.extend_from_slice(b"00000000"); // mode
    buf.extend_from_slice(b"00000000"); // uid
    buf.extend_from_slice(b"00000000"); // gid
    buf.extend_from_slice(b"00000001"); // nlink
    buf.extend_from_slice(b"00000000"); // mtime
    buf.extend_from_slice(b"00000000"); // filesize
    buf.extend_from_slice(b"00000000"); // dev major
    buf.extend_from_slice(b"00000000"); // dev minor
    buf.extend_from_slice(b"00000000"); // rdev major
    buf.extend_from_slice(b"00000000"); // rdev minor
    buf.extend_from_slice(b"0000000B"); // namesize = 11 (uppercase B, as systemd hard-codes it)
    buf.extend_from_slice(b"00000000"); // crc
    buf.extend_from_slice(b"TRAILER!!!");
    buf.extend_from_slice(&[0, 0, 0, 0]); // 3 explicit + 1 implicit NUL; keeps 4-alignment
}

/// Build the single-file cpio systemd-stub's `pack_cpio_literal` produces for an
/// embedded PE section: a `.extra` directory (mode 0555) followed by the file
/// `.extra/<filename>` (mode 0444) and the trailer.
fn build_extra_cpio(filename: &str, contents: &[u8]) -> Vec<u8> {
    let mut buf = Vec::new();
    let mut inode: u32 = 1;
    cpio_dir(&mut buf, ".extra", 0o555, &mut inode);
    cpio_file(&mut buf, ".extra", filename, contents, 0o444, &mut inode);
    cpio_trailer(&mut buf);
    buf
}

/// Reconstruct the combined initrd blob systemd-stub serves to the kernel (and
/// that the kernel EFI stub measures into PCR 9 as the "Linux initrd" event).
///
/// systemd-stub (`src/boot/stub.c`) merges a fixed, ordered set of initrd
/// components. We reconstruct the ones that can be embedded in the UKI PE, in
/// systemd's merge order:
///   * `.ucode`   → raw microcode cpio (`INITRD_UCODE`)
///   * `.initrd`  → raw base initrd (`INITRD_BASE`)
///   * `.pcrsig`  → cpio `/.extra/tpm2-pcr-signature.json` (`INITRD_PCRSIG`)
///   * `.pcrpkey` → cpio `/.extra/tpm2-pcr-public-key.pem` (`INITRD_PCRPKEY`)
///   * `.osrel`   → cpio `/.extra/os-release` (`INITRD_OSREL`)
///   * `.profile` → cpio `/.extra/profile` (`INITRD_PROFILE`)
///
/// The credential/sysext/confext components come from the ESP at runtime and are
/// empty for Bottlerocket, so they contribute nothing. When more than one
/// component is present each is padded to a 4-byte boundary before
/// concatenation (systemd `combine_initrds`); a single component is used
/// verbatim; zero components yield an empty initrd (no measurement — see
/// [`predict_uki`]). For a Bottlerocket UKI only `.osrel` is present, so the
/// result is exactly the `/.extra/os-release` cpio.
fn build_uki_initrd(uki: &[u8]) -> Result<Vec<u8>> {
    let mut components: Vec<Vec<u8>> = Vec::new();
    if let Some(d) = pe::get_optional_section(uki, ".ucode")? {
        components.push(d);
    }
    if let Some(d) = pe::get_optional_section(uki, ".initrd")? {
        components.push(d);
    }
    if let Some(d) = pe::get_optional_section(uki, ".pcrsig")? {
        components.push(build_extra_cpio("tpm2-pcr-signature.json", &d));
    }
    if let Some(d) = pe::get_optional_section(uki, ".pcrpkey")? {
        components.push(build_extra_cpio("tpm2-pcr-public-key.pem", &d));
    }
    if let Some(d) = pe::get_optional_section(uki, ".osrel")? {
        components.push(build_extra_cpio("os-release", &d));
    }
    if let Some(d) = pe::get_optional_section(uki, ".profile")? {
        components.push(build_extra_cpio("profile", &d));
    }

    match components.len() {
        0 => Ok(Vec::new()),
        1 => Ok(components.pop().unwrap()),
        _ => {
            let mut out = Vec::new();
            for c in &components {
                out.extend_from_slice(c);
                while out.len() % 4 != 0 {
                    out.push(0);
                }
            }
            Ok(out)
        }
    }
}

/// Predict the PCR 9 value for a UKI image.
///
/// The Linux kernel EFI stub extends PCR 9 twice (see the module docs): once
/// with the `LoadOptions` (command line) buffer and once with the combined
/// initrd blob. We reconstruct both from the UKI PE:
///
/// * `D_cmdline = SHA256(utf16le(.cmdline) + 0x0000)` — the `LoadOptions`
///   buffer systemd-stub hands to the kernel (`strsize16` includes the wide
///   NUL). This is NOT hashed with a trailing `\n` (that is the GRUB
///   `/proc/cmdline` behavior).
/// * `D_initrd = SHA256(build_uki_initrd(...))` — the LoadFile2 initrd blob.
///
/// Returns `extend(extend(0, D_cmdline), D_initrd)`. If the UKI has no initrd
/// components the initrd is empty; systemd-stub then registers no initrd and the
/// kernel measures no initrd event, so PCR 9 is `extend(0, D_cmdline)` only.
fn predict_uki(uki: &[u8]) -> Result<[u8; 32]> {
    let cmdline_bytes = pe::extract_cmdline(uki)?;
    let cmdline = std::str::from_utf8(&cmdline_bytes)
        .ok()
        .whatever_context("UKI .cmdline section is not valid UTF-8")?;

    // LoadOptions buffer: UTF-16LE command line including the terminating NUL
    // word (systemd sets LoadOptionsSize = strsize16(cmdline)).
    let mut load_options = Vec::with_capacity(cmdline_bytes.len() * 2 + 2);
    for unit in cmdline.encode_utf16() {
        load_options.extend_from_slice(&unit.to_le_bytes());
    }
    load_options.extend_from_slice(&[0, 0]);
    let d_cmdline: [u8; 32] = Sha256::digest(&load_options).into();

    let mut pcr9 = extend_pcr(&PCR_INIT_VAL, &d_cmdline);

    let initrd = build_uki_initrd(uki)?;
    if !initrd.is_empty() {
        let d_initrd: [u8; 32] = Sha256::digest(&initrd).into();
        pcr9 = extend_pcr(&pcr9, &d_initrd);
    }

    Ok(pcr9)
}

/// Predict PCR 9 value.
///
/// PCR 9 = extend(init, SHA256(cmdline + newline))
/// The trailing newline matches /proc/cmdline format.
pub fn predict(ctx: &PcrContext) -> Result<Option<(PcrIndex, PcrRecord)>> {
    // UKI images: the command line is measured by systemd-stub from the signed
    // .cmdline PE section, not reconstructed from grub.cfg + bootconfig.
    if !ctx.uki.is_empty() {
        let pcr9 = predict_uki(ctx.uki)?;
        return Ok(Some((PcrIndex::Pcr9, PcrRecord::new(pcr9))));
    }

    // GRUB images: reconstruct the BOOT-A command line. Even on dual-bank (A/B)
    // images — which is every standard AWS GRUB variant — only BOOT-A is
    // populated at build/AMI time and it is the highest-priority bank, so a
    // freshly launched instance always boots BOOT-A. The reconstruction sources
    // grub.cfg (EFI-A), bootconfig (PRIVATE), and the BOOT-A PARTUUID, so the
    // predicted PCR 9 matches the value the kernel measures on first boot. We
    // therefore predict for A/B images too (PCR 9 fidelity for aws-ecs-4 is a
    // hard requirement; see spec AC-6 / C9).
    let mut cmdline = predict_cmdline(ctx.grub_cfg, ctx.bootconfig, Some(ctx.boot_partuuid))?;
    cmdline.push('\n');
    let pcr9 = extend_pcr_string(&PCR_INIT_VAL, &cmdline);
    Ok(Some((PcrIndex::Pcr9, PcrRecord::new(pcr9))))
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    fn make_bootconfig(text: &str) -> Vec<u8> {
        let text_bytes = text.as_bytes();
        let size = text_bytes.len() as u32;
        let checksum: u32 = text_bytes.iter().map(|&b| b as u32).sum();
        let padding = (4 - (text_bytes.len() % 4)) % 4;
        let mut data = text_bytes.to_vec();
        data.extend(vec![0u8; padding]);
        data.extend(size.to_le_bytes());
        data.extend(checksum.to_le_bytes());
        data.extend(b"#BOOTCONFIG\n");
        data
    }

    #[test_case("key=value", "key=value" ; "no_quotes_unchanged")]
    #[test_case("simple", "simple" ; "no_equals_unchanged")]
    #[test_case(r#"key="value""#, r#""key=value""# ; "simple_quoted_value")]
    #[test_case(r#"key="value with spaces""#, r#""key=value with spaces""# ; "quoted_value_with_spaces")]
    #[test_case(r#"foo=bar key="quoted value" baz=qux"#, r#"foo=bar "key=quoted value" baz=qux"# ; "mixed_quoted_and_unquoted")]
    #[test_case(r#"dm-mod.create="root,,,ro,0 123 verity""#, r#""dm-mod.create=root,,,ro,0 123 verity""# ; "dm_mod_create_style")]
    #[test_case(r#"a="1" b="2" c="3""#, r#""a=1" "b=2" "c=3""# ; "multiple_quoted_values")]
    #[test_case(r#"first="val""#, r#""first=val""# ; "quoted_at_start")]
    #[test_case("", "" ; "empty_string")]
    fn test_repair_quotes(input: &str, expected: &str) {
        assert_eq!(repair_quotes(input), expected);
    }

    #[test_case(
        b"linux ($root)/vmlinuz console=tty0 -- systemd.log_target=journal",
        "kernel.FOO = bar\n",
        "FOO=bar BOOT_IMAGE=/vmlinuz console=tty0 -- systemd.log_target=journal"
        ; "kernel_param_before_boot_image"
    )]
    #[test_case(
        b"linux ($root)/vmlinuz quiet -- init=/sbin/init",
        "init.BAZ = qux\n",
        "BOOT_IMAGE=/vmlinuz quiet -- BAZ=qux init=/sbin/init"
        ; "init_param_after_separator"
    )]
    #[test_case(
        b"linux ($root)/vmlinuz console=tty0 -- systemd.log_color=0",
        "kernel.A = 1\ninit.B = 2\n",
        "A=1 BOOT_IMAGE=/vmlinuz console=tty0 -- B=2 systemd.log_color=0"
        ; "both_kernel_and_init_params"
    )]
    #[test_case(
        b"linux ($root)/vmlinuz quiet -- x",
        "",
        "BOOT_IMAGE=/vmlinuz quiet -- x"
        ; "empty_bootconfig"
    )]
    #[test_case(
        b"linux ($root)/vmlinuz -- x",
        "kernel.A = 1\n",
        "A=1 BOOT_IMAGE=/vmlinuz -- x"
        ; "kernel_only_no_init_bootconfig"
    )]
    #[test_case(
        b"linux ($root)/vmlinuz foo -- bar",
        "init.X = y\n",
        "BOOT_IMAGE=/vmlinuz foo -- X=y bar"
        ; "init_only_no_kernel_bootconfig"
    )]
    #[test_case(
        b"linux ($root)/vmlinuz dm-mod.create=\"root verity\" -- x",
        "",
        r#"BOOT_IMAGE=/vmlinuz "dm-mod.create=root verity" -- x"#
        ; "grub_quoted_value_gets_repaired"
    )]
    #[test_case(
        b"linux ($root)/vmlinuz -- x",
        "kernel.MSG = \"hello world\"\n",
        r#"MSG="hello world" BOOT_IMAGE=/vmlinuz -- x"#
        ; "bootconfig_quoted_value_not_repaired"
    )]
    #[test_case(
        b"linux ($root)/vmlinuz a=1 b=2 -- c=3 d=4",
        "kernel.K = v\ninit.I = w\n",
        "K=v BOOT_IMAGE=/vmlinuz a=1 b=2 -- I=w c=3 d=4"
        ; "multiple_grub_args_both_sides"
    )]
    #[test_case(
        b"linux ($root)/vmlinuz PARTUUID=/PARTNROFF=1 PARTUUID=/PARTNROFF=2 -- x",
        "",
        "BOOT_IMAGE=/vmlinuz PARTUUID=/PARTNROFF=1 PARTUUID=/PARTNROFF=2 -- x"
        ; "multiple_partuuid_without_substitution"
    )]
    // --- T04: value-less and array bootconfig behavior through pcr9 assembly ---
    #[test_case(
        b"linux ($root)/vmlinuz console=tty0 -- x",
        "kernel.mods = a, b, c\n",
        "mods=a mods=b mods=c BOOT_IMAGE=/vmlinuz console=tty0 -- x"
        ; "kernel_array_repeats_key_before_boot_image"
    )]
    #[test_case(
        b"linux ($root)/vmlinuz quiet -- x",
        "init.mods = a, b\n",
        "BOOT_IMAGE=/vmlinuz quiet -- mods=a mods=b x"
        ; "init_array_repeats_key_after_separator"
    )]
    #[test_case(
        b"linux ($root)/vmlinuz quiet -- x",
        "kernel.debug\n",
        "debug BOOT_IMAGE=/vmlinuz quiet -- x"
        ; "kernel_valueless_key_before_boot_image"
    )]
    #[test_case(
        b"linux ($root)/vmlinuz quiet -- x",
        "init.splash\n",
        "BOOT_IMAGE=/vmlinuz quiet -- splash x"
        ; "init_valueless_key_after_separator"
    )]
    #[test_case(
        b"linux ($root)/vmlinuz quiet -- x",
        "kernel.debug\nkernel.mods = a, b\ninit.splash\n",
        "debug mods=a mods=b BOOT_IMAGE=/vmlinuz quiet -- splash x"
        ; "valueless_and_array_both_sections"
    )]
    // repair_quotes must touch ONLY the grub args, never bootconfig values:
    // a grub-quoted `key="v v"` becomes `"key=v v"`, while a bootconfig-quoted
    // value keeps kernel `key="v v"` form.
    #[test_case(
        b"linux ($root)/vmlinuz dm-mod.create=\"root verity\" -- x",
        "kernel.MSG = \"hello world\"\n",
        r#"MSG="hello world" BOOT_IMAGE=/vmlinuz "dm-mod.create=root verity" -- x"#
        ; "bootconfig_quoted_and_grub_quoted_coexist"
    )]
    fn test_predict_cmdline(grub_cfg: &[u8], bootconfig_text: &str, expected: &str) {
        let bootconfig = make_bootconfig(bootconfig_text);
        let cmdline = predict_cmdline(grub_cfg, &bootconfig, None).unwrap();
        assert_eq!(cmdline, expected);
    }

    #[test]
    fn test_predict_cmdline_partuuid_substitution() {
        let grub_cfg = b"linux ($root)/vmlinuz root=PARTUUID=/PARTNROFF=1 -- x";
        let bootconfig = make_bootconfig("");
        let cmdline = predict_cmdline(grub_cfg, &bootconfig, Some("abcd-1234")).unwrap();
        assert_eq!(
            cmdline,
            "BOOT_IMAGE=/vmlinuz root=PARTUUID=abcd-1234/PARTNROFF=1 -- x"
        );
    }

    #[test]
    fn test_predict_cmdline_multiple_partuuid_substitution() {
        let grub_cfg = b"linux ($root)/vmlinuz PARTUUID=/PARTNROFF=1 PARTUUID=/PARTNROFF=2 -- x";
        let bootconfig = make_bootconfig("");
        let cmdline = predict_cmdline(grub_cfg, &bootconfig, Some("uuid-here")).unwrap();
        assert_eq!(cmdline, "BOOT_IMAGE=/vmlinuz PARTUUID=uuid-here/PARTNROFF=1 PARTUUID=uuid-here/PARTNROFF=2 -- x");
    }

    #[test]
    fn test_predict_includes_trailing_newline() {
        let grub_cfg = b"linux ($root)/vmlinuz -- x";
        let bootconfig = make_bootconfig("");
        use crate::predict::test_support::MockCtx;
        let m = MockCtx::new();
        let ctx = PcrContext::builder()
            .platform(crate::platform::Platform::Aws)
            .efi_vars(&m.efi_vars)
            .partitions(&m.layout)
            .grub_cfg(grub_cfg.as_slice())
            .bootconfig(bootconfig.as_slice())
            .build();
        let result = predict(&ctx).unwrap().unwrap();
        let cmdline_with_newline = "BOOT_IMAGE=/vmlinuz -- x\n";
        let expected = extend_pcr_string(&PCR_INIT_VAL, cmdline_with_newline);
        assert_eq!(result.1.sha256[0], hex::encode(expected));
    }

    #[test_case(b"linux /wrong/path console=tty0 -- x" ; "wrong_kernel_path")]
    #[test_case(b"linux (hd0,gpt3)/vmlinuz console=tty0 -- x" ; "explicit_device_in_path")]
    fn test_predict_cmdline_errors(grub_cfg: &[u8]) {
        let bootconfig = make_bootconfig("");
        let result = predict_cmdline(grub_cfg, &bootconfig, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_predict_dual_bank_reconstructs_boot_a() {
        // Dual-bank (A/B) GRUB images still get a PCR 9 prediction: at build/AMI
        // time only BOOT-A is populated and it is the highest-priority bank, so
        // the instance boots BOOT-A on first boot. We reconstruct the BOOT-A
        // command line even though a BOOT-B partition exists in the GPT, so
        // predicted PCR 9 matches the measured value (spec AC-6 / C9).
        use crate::predict::test_support::MockCtx;
        let grub_cfg = b"linux ($root)/vmlinuz console=tty0 -- x";
        let bootconfig = make_bootconfig("kernel.A = 1\n");
        let m = MockCtx::dual_bank();
        let ctx = PcrContext::builder()
            .platform(crate::platform::Platform::Aws)
            .efi_vars(&m.efi_vars)
            .partitions(&m.layout)
            .grub_cfg(grub_cfg.as_slice())
            .bootconfig(bootconfig.as_slice())
            .build();
        let (index, record) = predict(&ctx).unwrap().unwrap();
        assert_eq!(index, PcrIndex::Pcr9);
        let expected_cmdline = "A=1 BOOT_IMAGE=/vmlinuz console=tty0 -- x\n";
        let expected = extend_pcr_string(&PCR_INIT_VAL, expected_cmdline);
        assert_eq!(record.sha256[0], hex::encode(expected));
    }

    // --- T09/T17: UKI PCR9 prediction (kernel EFI stub two-event chain) ---

    /// Compute the LoadOptions digest for a UKI command line: the UTF-16LE
    /// buffer including the terminating NUL word, hashed with SHA-256.
    fn load_options_digest(cmdline: &[u8]) -> [u8; 32] {
        let s = std::str::from_utf8(cmdline).unwrap();
        let mut lo = Vec::new();
        for u in s.encode_utf16() {
            lo.extend_from_slice(&u.to_le_bytes());
        }
        lo.extend_from_slice(&[0, 0]);
        Sha256::digest(&lo).into()
    }

    /// Build a PcrContext carrying a UKI PE and predict PCR 9 from it.
    fn predict_uki_pcr9(cmdline: &[u8]) -> (crate::predict::PcrIndex, crate::predict::PcrRecord) {
        use crate::predict::test_support::MockCtx;
        let uki = crate::pe::tests::build_test_uki(cmdline);
        let m = MockCtx::new();
        let ctx = PcrContext::builder()
            .platform(crate::platform::Platform::Aws)
            .efi_vars(&m.efi_vars)
            .partitions(&m.layout)
            .uki(uki.as_slice())
            .build();
        predict(&ctx).unwrap().unwrap()
    }

    #[test]
    fn test_predict_uki_loadoptions_only_when_no_initrd() {
        // build_test_uki has only a .cmdline section (no .osrel/.initrd/etc.),
        // so systemd-stub would register no initrd and the kernel measures only
        // the LoadOptions event: PCR9 = extend(0, D_cmdline).
        let cmdline = b"root=/dev/dm-0 -- systemd.log_color=0";
        let (index, record) = predict_uki_pcr9(cmdline);
        assert_eq!(index, PcrIndex::Pcr9);
        let expected = extend_pcr(&PCR_INIT_VAL, &load_options_digest(cmdline));
        assert_eq!(record.sha256, vec![hex::encode(expected)]);
    }

    #[test]
    fn test_predict_uki_loadoptions_is_utf16_not_utf8_newline() {
        // The command line is measured as the UTF-16LE LoadOptions buffer (with
        // a trailing NUL word), NOT as UTF-8 with a trailing '\n' (which is the
        // GRUB /proc/cmdline behavior). Confirm the two differ and we use UTF-16.
        let cmdline = b"BOOT_IMAGE=/vmlinuz quiet -- x";
        let (_, record) = predict_uki_pcr9(cmdline);

        let utf16 = extend_pcr(&PCR_INIT_VAL, &load_options_digest(cmdline));
        assert_eq!(record.sha256, vec![hex::encode(utf16)]);

        let utf8_newline =
            extend_pcr_string(&PCR_INIT_VAL, &format!("{}\n", std::str::from_utf8(cmdline).unwrap()));
        assert_ne!(record.sha256, vec![hex::encode(utf8_newline)]);
    }

    #[test]
    fn test_predict_uki_takes_precedence_over_grub_fields() {
        // Even when grub_cfg/bootconfig happen to be present, a non-empty UKI
        // routes through the UKI path (no grub.cfg parsing).
        use crate::predict::test_support::MockCtx;
        let cmdline = b"root=/dev/dm-0 -- x";
        let uki = crate::pe::tests::build_test_uki(cmdline);
        let bootconfig = make_bootconfig("kernel.FOO = bar\n");
        let m = MockCtx::new();
        let ctx = PcrContext::builder()
            .platform(crate::platform::Platform::Aws)
            .efi_vars(&m.efi_vars)
            .partitions(&m.layout)
            .grub_cfg(b"this is not valid grub.cfg".as_slice())
            .bootconfig(bootconfig.as_slice())
            .uki(uki.as_slice())
            .build();
        let (index, record) = predict(&ctx).unwrap().unwrap();
        assert_eq!(index, PcrIndex::Pcr9);
        let expected = extend_pcr(&PCR_INIT_VAL, &load_options_digest(cmdline));
        assert_eq!(record.sha256, vec![hex::encode(expected)]);
    }

    // --- Golden vectors captured from a real booted aws-mantle-1 UKI instance ---
    // The `.osrel`/`.cmdline` section bytes below were extracted from the built
    // UKI (bottlerocket.efi); the digests were read from the instance's TPM
    // event log (PCR 9 EV_EVENT_TAG events) and confirm predicted == measured.

    /// Exact `.osrel` PE section (461 bytes) of the built aws-mantle-1 UKI.
    const GOLDEN_OSREL: &[u8] = b"NAME=Bottlerocket\nID=bottlerocket\nVERSION=\"1.64.0 (aws-mantle-1)\"\nPRETTY_NAME=\"Bottlerocket OS 1.64.0 (aws-mantle-1)\"\nVARIANT_ID=aws-mantle-1\nVERSION_ID=1.64.0\nBUILD_ID=841ea622-dirty\nVENDOR_NAME=\"Bottlerocket\"\nHOME_URL=\"https://github.com/bottlerocket-os/bottlerocket\"\nSUPPORT_URL=\"https://github.com/bottlerocket-os/bottlerocket/discussions\"\nBUG_REPORT_URL=\"https://github.com/bottlerocket-os/bottlerocket/issues\"\nDOCUMENTATION_URL=\"https://bottlerocket.dev\"\n";

    /// Exact `.cmdline` PE section (728 bytes) of the same built UKI.
    const GOLDEN_CMDLINE: &[u8] = b"SYSTEMD_CGROUP_ENABLE_LEGACY_FORCE=1 SYSTEMD_DEFAULT_MOUNT_RATE_LIMIT_BURST=25 module_blacklist=i8042 console=tty0 console=ttyS0,115200n8 net.ifnames=0 netdog.default-interface=eth0:dhcp4,dhcp6? quiet root=/dev/dm-0 rootwait ro raid=noautodetect random.trust_cpu=on selinux=1 enforcing=1 \"dm-mod.create=root,,,ro,0 418032 verity 1 PARTUUID=3BA52C48-6C91-4BB3-AB2E-515EEAED3BD9/PARTNROFF=1 PARTUUID=3BA52C48-6C91-4BB3-AB2E-515EEAED3BD9/PARTNROFF=2 4096 4096 52254 1 sha256 2e59e246e42130f289df4a5347ebb94f6b8433409f42b397f67d749ff9d385a0 dda703cc7c12427b572e2793eb143204321cf851be471c241310f5d81fdde95d 2 restart_on_corruption ignore_zero_blocks\" -- systemd.log_target=journal-or-kmsg systemd.log_color=0 systemd.show_status=true";

    #[test]
    fn test_golden_osrel_section_lengths() {
        assert_eq!(GOLDEN_OSREL.len(), 461);
        assert_eq!(GOLDEN_CMDLINE.len(), 728);
    }

    #[test]
    fn test_golden_initrd_cpio_digest() {
        // D_initrd = SHA256(/.extra/os-release cpio) as measured on hardware
        // (event 43, "Linux initrd").
        let cpio = build_extra_cpio("os-release", GOLDEN_OSREL);
        let digest: [u8; 32] = Sha256::digest(&cpio).into();
        assert_eq!(
            hex::encode(digest),
            "7aaf4b59c7013acf255ff3d2708f762ac6c17ebc30af67215ff3e046487c94f7"
        );
    }

    #[test]
    fn test_golden_loadoptions_digest() {
        // D_cmdline = SHA256(utf16le(cmdline) + NUL) as measured on hardware
        // (event 42, "LOADED_IMAGE::LoadOptions").
        assert_eq!(
            hex::encode(load_options_digest(GOLDEN_CMDLINE)),
            "ee23489fd148d2787139df3439020db045d78625b1123e138440e110c51194c1"
        );
    }

    #[test]
    fn test_golden_pcr9_chain() {
        // extend(extend(0, D_cmdline), D_initrd) == measured PCR 9 on the booted
        // aws-mantle-1 instance (tpm2_pcrread sha256:9).
        let d_cmdline = load_options_digest(GOLDEN_CMDLINE);
        let cpio = build_extra_cpio("os-release", GOLDEN_OSREL);
        let d_initrd: [u8; 32] = Sha256::digest(&cpio).into();
        let pcr9 = extend_pcr(&extend_pcr(&PCR_INIT_VAL, &d_cmdline), &d_initrd);
        assert_eq!(
            hex::encode(pcr9),
            "fa8e7cddfaa2c4d2037a3be12930a08d8098694a5286ed1e9121163773932563"
        );
    }

    #[test]
    fn test_cpio_trailer_uppercase_namesize() {
        // Regression guard for the load-bearing detail: systemd's cpio trailer
        // hard-codes the namesize field as uppercase "0000000B". A lowercase
        // "0000000b" would change the initrd hash and break the measurement.
        let cpio = build_extra_cpio("os-release", GOLDEN_OSREL);
        let s = String::from_utf8_lossy(&cpio);
        assert!(s.contains("0000000B00000000TRAILER!!!"));
        assert!(!s.contains("0000000b00000000TRAILER!!!"));
    }
}
