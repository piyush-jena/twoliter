//! Bootconfig decoding and boot command-line construction shared by image
//! assembly and PCR prediction. No shell evaluation or I/O occurs here.
//!
//! Every operation an external crate performs is a function in this module:
//! [`merge_parameters`] for the drop-ins packages contribute,
//! [`compose_uki_cmdline`] for the command line a UKI is built with, and
//! [`decode_uki_cmdline`], [`uki_kernel_parameters`], [`encode_uki_load_options`] and
//! [`predict_grub_cmdline`] for predicting what a built image measures into
//! PCR 9. The parsing and composition behind them stay internal.

mod cmdline;
mod error;
mod grub_cmdline;
mod params;
mod parsers;
#[cfg(any(test, feature = "test-util"))]
pub mod test_util;
mod uki_cmdline;

pub use error::{Error, Result};

use grub_cmdline::GrubCmdline;

/// Merge the drop-ins' `module_blacklist` and `initcall_blacklist` values into one quoted
/// value each, keeping every other assignment as written.
///
/// # Errors
///
/// Fails if the text is not valid bootconfig, or a list value holds a quote.
pub fn merge_parameters(text: &str) -> Result<String> {
    params::merge_parameters(text)
}

/// Compose the command line to embed in a UKI, from binary bootconfig and the
/// base arguments of the image.
///
/// The bootconfig parameters of each namespace precede the base arguments on
/// their side of the separator: `<kernel.*> <kernel_args> -- <init.*> <init_args>`,
/// joined by single spaces.
///
/// # Errors
///
/// Fails if the bootconfig is invalid, or the command line would not survive whitespace
/// collapsing unchanged, has an unpaired quote, or does not fit the kernel.
pub fn compose_uki_cmdline(
    bootconfig: &[u8],
    kernel_args: &str,
    init_args: &str,
) -> Result<String> {
    let cmdline =
        parsers::bootconfig::parse(bootconfig)?.cmdline(kernel_args.trim(), init_args.trim())?;
    uki_cmdline::ensure_embeddable(&cmdline)?;
    Ok(cmdline)
}

/// Decode the command line a built UKI carries, from its `.cmdline` section.
///
/// # Errors
///
/// Fails if the section is not UTF-8, has data after its NUL, or does not fit the kernel.
pub fn decode_uki_cmdline(section: &[u8]) -> Result<&str> {
    uki_cmdline::decode(section)
}

/// The names of a UKI command line's kernel parameters, split as the kernel splits them.
pub fn uki_kernel_parameters(cmdline: &str) -> Vec<&str> {
    cmdline::kernel_parameters(cmdline)
}

/// Encode the EFI LoadOptions that systemd-stub measures into PCR 9, as
/// UTF-16LE including the terminating NUL.
///
/// # Errors
///
/// Fails if the command line is not ASCII.
pub fn encode_uki_load_options(cmdline: &str) -> Result<Vec<u8>> {
    uki_cmdline::load_options(cmdline)
}

/// The command line GRUB starts a kernel with, and the one the kernel reports.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrubCmdlinePrediction {
    /// The EFI LoadOptions GRUB starts the kernel with, in UTF-16LE without a terminating NUL.
    pub load_options: Vec<u8>,
    /// The command line the kernel reports on `/proc/cmdline`.
    pub proc_cmdline: String,
}

/// Predict the command line GRUB starts a kernel with, and the `/proc/cmdline` it reports.
///
/// GRUB passes `BOOT_IMAGE=<path> <grub kernel args> -- <grub init args>`. The kernel places
/// the bootconfig parameters of each namespace ahead of the GRUB arguments on their side of
/// the separator:
/// `<kernel.*> BOOT_IMAGE=<path> <grub kernel args> -- <init.*> <grub init args>`
///
/// `boot_partuuid` fills in the `PARTUUID=/PARTNROFF=` placeholder the image
/// builder leaves for the partition GRUB booted from.
///
/// # Errors
///
/// Fails if the bootconfig or grub.cfg is invalid, or grub.cfg boots no `($root)/vmlinuz`.
pub fn predict_grub_cmdline(
    bootconfig: &[u8],
    grub_cfg: &[u8],
    boot_partuuid: &str,
) -> Result<GrubCmdlinePrediction> {
    let grub = GrubCmdline::parse(grub_cfg, boot_partuuid)?;
    let proc_cmdline = parsers::bootconfig::parse(bootconfig)?.grub_cmdline(&grub)?;
    // GRUB converts the command line to UTF-16 and stops at its NUL, which it leaves out.
    let load_options = grub
        .loader_cmdline()
        .encode_utf16()
        .flat_map(u16::to_le_bytes)
        .collect();
    Ok(GrubCmdlinePrediction {
        load_options,
        proc_cmdline,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test_util::encode as bootconfig_data;

    #[test]
    fn the_bootconfig_binary_merges_drop_in_contributions() {
        let merged = merge_parameters(concat!(
            "kernel.module_blacklist = i8042\n",
            "kernel.console = tty0, ttyS0\n",
            "kernel.module_blacklist = binfmt_misc\n",
            "init.systemd.unit = fipscheck.target\n",
        ))
        .unwrap();
        assert_eq!(
            merged,
            concat!(
                "init.systemd.unit = fipscheck.target\n",
                "kernel.console = tty0, ttyS0\n",
                "kernel.module_blacklist = \"binfmt_misc,i8042\"\n",
            )
        );
        // Rebuilding an image from its own merged input is a no-op.
        assert_eq!(merge_parameters(&merged).unwrap(), merged);
        assert_eq!(
            compose_uki_cmdline(&bootconfig_data(&merged), "root=/dev/dm-0", "").unwrap(),
            concat!(
                "console=tty0 console=ttyS0 module_blacklist=binfmt_misc,i8042 ",
                "root=/dev/dm-0 -- systemd.unit=fipscheck.target",
            )
        );
    }

    #[test]
    fn ukisys_composes_the_command_line_to_embed() {
        let data = bootconfig_data("kernel.fips = 1\ninit.systemd.unit = fipscheck.target\n");
        assert_eq!(
            compose_uki_cmdline(&data, "root=/dev/dm-0 ro", "quiet").unwrap(),
            "fips=1 root=/dev/dm-0 ro -- systemd.unit=fipscheck.target quiet"
        );
    }

    #[test]
    fn pcrsys_decodes_a_uki_section_then_encodes_load_options() {
        let mut section = b"quiet --".to_vec();
        section.extend([0, 0, 0]);

        let cmdline = decode_uki_cmdline(&section).unwrap();
        assert_eq!(cmdline, "quiet --");

        // The caller keeps its own policy on what it will measure.
        assert!(!cmdline
            .replace('"', "")
            .split_ascii_whitespace()
            .any(|argument| argument == "noinitrd" || argument.starts_with("initrd=")));

        assert_eq!(
            encode_uki_load_options(cmdline).unwrap(),
            b"q\0u\0i\0e\0t\0 \0-\0-\0\0\0"
        );
    }

    #[test]
    fn pcrsys_predicts_the_grub_command_lines() {
        let grub_cfg = br#"
    set dm_verity_root="root,,,ro,0 123 verity"
    menuentry "Bottlerocket" {
    linux ($root)/vmlinuz \
        root=/dev/dm-0 \
        dm-mod.create="$dm_verity_root" \
        boot_source=PARTUUID=/PARTNROFF=1 \
        -- \
        systemd.log_target=journal
    initrd ($private)/bootconfig.data
    }
    "#;
        let data = bootconfig_data("kernel.fips = 1\ninit.systemd.unit = multi-user.target\n");
        let predicted = predict_grub_cmdline(&data, grub_cfg, "abcd-1234").unwrap();
        assert_eq!(
            predicted.proc_cmdline,
            concat!(
                "fips=1 BOOT_IMAGE=/vmlinuz root=/dev/dm-0 ",
                "\"dm-mod.create=root,,,ro,0 123 verity\" ",
                "boot_source=PARTUUID=abcd-1234/PARTNROFF=1 ",
                "-- systemd.unit=multi-user.target systemd.log_target=journal",
            )
        );
        // GRUB passes only its own arguments; the kernel adds the bootconfig parameters.
        let loader_cmdline = concat!(
            "BOOT_IMAGE=/vmlinuz root=/dev/dm-0 ",
            "\"dm-mod.create=root,,,ro,0 123 verity\" ",
            "boot_source=PARTUUID=abcd-1234/PARTNROFF=1 ",
            "-- systemd.log_target=journal",
        );
        let utf16: Vec<u8> = loader_cmdline
            .encode_utf16()
            .flat_map(u16::to_le_bytes)
            .collect();
        assert_eq!(predicted.load_options, utf16);
        assert_eq!(
            predicted.load_options.len(),
            2 * loader_cmdline.len(),
            "two bytes for each character, and no terminating NUL"
        );
    }

    #[test]
    fn errors_are_matchable_by_variant() {
        let composed: Result<String> = compose_uki_cmdline(b"short", "", "");
        assert!(matches!(composed, Err(Error::BinaryTooShort { .. })));
        assert!(matches!(
            encode_uki_load_options("é"),
            Err(Error::InvalidLoadOptions)
        ));
        assert!(matches!(
            predict_grub_cmdline(&bootconfig_data(""), b"linux /wrong -- x", "abcd-1234"),
            Err(Error::InvalidGrubKernelPath { .. })
        ));
    }

    /// The drop-ins and arguments rpm2img composes for aws-mantle-1.
    const MANTLE_DROP_INS: &str = concat!(
        "kernel.SYSTEMD_CGROUP_ENABLE_LEGACY_FORCE = 1\n",
        "kernel.SYSTEMD_DEFAULT_MOUNT_RATE_LIMIT_BURST = 25\n",
        "kernel.module_blacklist = \"i8042\"\n",
    );
    const MANTLE_KERNEL_ARGS: &str = concat!(
        "console=tty0 console=ttyS0,115200n8 net.ifnames=0 ",
        "netdog.default-interface=eth0:dhcp4,dhcp6? quiet root=/dev/dm-0 rootwait ro ",
        "raid=noautodetect random.trust_cpu=on selinux=1 enforcing=1 ",
        "dm-mod.create=\"root,,,ro,0 430080 verity 1 PARTUUID=bd06db87/PARTNROFF=1 ",
        "PARTUUID=bd06db87/PARTNROFF=2 4096 4096 53760 1 sha256 5c5e 0be9 2 ",
        "restart_on_corruption ignore_zero_blocks\"",
    );
    const MANTLE_INIT_ARGS: &str =
        "systemd.log_target=journal-or-kmsg systemd.log_color=0 systemd.show_status=true";

    #[test]
    fn composing_todays_inputs_is_unchanged() {
        let data = bootconfig_data(MANTLE_DROP_INS);
        let legacy = parsers::bootconfig::parse(&data)
            .unwrap()
            .cmdline(MANTLE_KERNEL_ARGS, MANTLE_INIT_ARGS)
            .unwrap();
        let composed = compose_uki_cmdline(&data, MANTLE_KERNEL_ARGS, MANTLE_INIT_ARGS).unwrap();
        assert_eq!(composed, legacy);
        assert_eq!(
            uki_kernel_parameters(&composed)[..3],
            [
                "SYSTEMD_CGROUP_ENABLE_LEGACY_FORCE",
                "SYSTEMD_DEFAULT_MOUNT_RATE_LIMIT_BURST",
                "module_blacklist"
            ]
        );
    }

    #[test]
    fn an_empty_part_leaves_a_single_space() {
        // rpm2img passes "${KERNEL_PARAMETERS} ${BASE_KERNEL_ARGS} ...", which starts with a
        // space when a variant sets no kernel parameters.
        let data = bootconfig_data("kernel.module_blacklist = i8042\ninit.x = 1\n");
        assert_eq!(
            compose_uki_cmdline(&data, " root=/dev/dm-0", " quiet").unwrap(),
            "module_blacklist=i8042 root=/dev/dm-0 -- x=1 quiet"
        );
    }

    #[test_case::test_case("root=/dev/dm-0  ro", "" ; "a_run_of_spaces")]
    #[test_case::test_case("root=/dev/dm-0\tro", "" ; "a_tab")]
    #[test_case::test_case("root=/dev/dm-0\nro", "" ; "a_newline")]
    #[test_case::test_case("dm-mod.create=\"a  b\"", "" ; "a_run_of_spaces_in_quotes")]
    fn whitespace_a_repack_would_collapse_is_rejected(kernel_args: &str, init_args: &str) {
        let data = bootconfig_data("");
        assert!(matches!(
            compose_uki_cmdline(&data, kernel_args, init_args),
            Err(Error::UncollapsedWhitespace)
        ));
    }

    #[test]
    fn a_bootconfig_value_with_a_run_of_spaces_is_rejected() {
        let data = bootconfig_data("kernel.x = \"a  b\"\n");
        assert!(matches!(
            compose_uki_cmdline(&data, "", ""),
            Err(Error::UncollapsedWhitespace)
        ));
    }

    #[test]
    fn an_unpaired_quote_or_an_oversized_line_is_rejected() {
        let data = bootconfig_data("");
        assert!(matches!(
            compose_uki_cmdline(&data, "dm-mod.create=\"root", ""),
            Err(Error::UnpairedQuote)
        ));
        let long = "x".repeat(2048);
        assert!(matches!(
            compose_uki_cmdline(&data, &long, ""),
            Err(Error::InvalidUkiSectionLength { .. })
        ));
    }
}
