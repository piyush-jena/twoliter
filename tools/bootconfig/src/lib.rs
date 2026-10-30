//! Bootconfig decoding and boot command-line construction shared by image assembly and PCR
//! prediction.

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

/// Compose the command line to embed in a UKI from binary bootconfig and the image's base
/// arguments: `<kernel.*> <kernel_args> -- <init.*> <init_args>`.
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

/// Predict the command line GRUB starts a kernel with, and the `/proc/cmdline` it reports:
/// `<kernel.*> BOOT_IMAGE=<path> <grub kernel args> -- <init.*> <grub init args>`.
///
/// `boot_partuuid` is the BOOT partition's PARTUUID, which GRUB substitutes for `$boot_uuid`.
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
    fn pcrsys_predicts_the_grub_command_lines() {
        let data = bootconfig_data("kernel.fips = 1\n");
        let predicted =
            predict_grub_cmdline(&data, b"linux ($root)/vmlinuz quiet -- x", "abcd-1234").unwrap();
        assert_eq!(
            predicted.proc_cmdline,
            "fips=1 BOOT_IMAGE=/vmlinuz quiet -- x"
        );
        // GRUB passes only its own arguments; the kernel adds the bootconfig parameters.
        let load_options: Vec<u8> = "BOOT_IMAGE=/vmlinuz quiet -- x"
            .encode_utf16()
            .flat_map(u16::to_le_bytes)
            .collect();
        assert_eq!(predicted.load_options, load_options);
    }

    #[test]
    fn an_empty_part_leaves_a_single_space() {
        // As rpm2img passes them when a variant sets no kernel parameters.
        let data = bootconfig_data("kernel.module_blacklist = i8042\ninit.x = 1\n");
        assert_eq!(
            compose_uki_cmdline(&data, " root=/dev/dm-0", " quiet").unwrap(),
            "module_blacklist=i8042 root=/dev/dm-0 -- x=1 quiet"
        );
    }

    #[test]
    fn a_quoted_value_with_single_spaces_is_embeddable() {
        assert_eq!(
            compose_uki_cmdline(&bootconfig_data(""), "dm-mod.create=\"a b\"", "").unwrap(),
            "dm-mod.create=\"a b\" --"
        );
    }

    #[test_case::test_case("root=/dev/dm-0  ro" ; "a_run_of_spaces")]
    #[test_case::test_case("root=/dev/dm-0\tro" ; "a_tab")]
    fn whitespace_a_repack_would_collapse_is_rejected(kernel_args: &str) {
        assert!(matches!(
            compose_uki_cmdline(&bootconfig_data(""), kernel_args, ""),
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
    fn an_unpaired_quote_is_rejected() {
        assert!(matches!(
            compose_uki_cmdline(&bootconfig_data(""), "dm-mod.create=\"root", ""),
            Err(Error::UnpairedQuote)
        ));
    }

    #[test]
    fn a_line_too_long_for_the_kernel_is_rejected() {
        assert!(matches!(
            compose_uki_cmdline(&bootconfig_data(""), &"x".repeat(2048), ""),
            Err(Error::InvalidUkiSectionLength { .. })
        ));
    }
}
