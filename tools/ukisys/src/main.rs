//! UKI command-line construction and recovery of an unsigned systemd-stub
//! from a finished Bottlerocket UKI.
//! ukisys: constructs kernel commandline for UKIs, derives the unsigned
//! systemd-stub PE a Bottlerocket UKI was built from, by stripping its
//! Authenticode signature and truncating the trailing payload sections.
//! Those sections are `.osrel`, `.cmdline`, `.uname`, and `.linux`.

mod error;
mod pe;

use clap::{Parser, Subcommand};
use pe::PeImage;
use snafu::ResultExt;
use std::path::{Path, PathBuf};

use crate::error::Result;

/// All four sections must be removed, not just the three payload sections:
/// `ukify build` overwrites any same-named section already in the stub in
/// place instead of appending fresh content, rather than skipping it.
const TRAILING_SECTIONS_TO_REMOVE: &[&str] = &[".osrel", ".cmdline", ".uname", ".linux"];

/// Command-line arguments for ukisys.
#[derive(Parser)]
#[command(
    version,
    about = "Command-line construction and PE section removal for Bottlerocket UKIs"
)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

/// Subcommands for UKI construction and repacking.
#[derive(Subcommand)]
enum Command {
    /// Merge package drop-ins before generating binary bootconfig.
    MergeBootconfig {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    /// Construct a UKI command-line file from generated binary bootconfig.
    Cmdline {
        #[arg(long)]
        bootconfig: PathBuf,
        /// Base kernel arguments, with literal kernel quoting.
        #[arg(long, allow_hyphen_values = true)]
        kernel_args: String,
        /// Base init arguments (after the separator).
        #[arg(long, allow_hyphen_values = true)]
        init_args: String,
        #[arg(long)]
        output: PathBuf,
    },
    /// Derive an unsigned systemd-stub PE from a finished, signed UKI.
    DeriveStub {
        /// Path to the finished, signed UKI to strip.
        uki: PathBuf,

        /// Path to write the derived, unsigned stub to.
        stub: PathBuf,
    },
}

#[snafu::report]
fn main() -> Result<()> {
    let args = Args::parse();
    match &args.command {
        Command::MergeBootconfig { input, output } => merge_bootconfig(input, output),
        Command::Cmdline {
            bootconfig,
            kernel_args,
            init_args,
            output,
        } => cmdline(bootconfig, kernel_args, init_args, output),
        Command::DeriveStub { uki, stub } => derive_stub(uki, stub),
    }
}

/// Merge drop-in contributions in the bootconfig input text.
///
/// The input is read in full before writing, so `output` may be `input`.
fn merge_bootconfig(input: &Path, output: &Path) -> Result<()> {
    let text = std::fs::read_to_string(input).context(error::FileReadSnafu { path: input })?;
    let merged = bootconfig::merge_blacklisted_modules(&text).context(error::BootconfigSnafu {
        operation: "merge bootconfig module blacklists",
    })?;
    std::fs::write(output, merged).context(error::FileWriteSnafu { path: output })
}

/// Compose the UKI command line from generated binary bootconfig.
///
/// Nothing is written unless the complete command line is constructed, so a
/// failed run cannot leave a partial command line behind for UKI assembly.
fn cmdline(bootconfig: &Path, kernel_args: &str, init_args: &str, output: &Path) -> Result<()> {
    let data = std::fs::read(bootconfig).context(error::FileReadSnafu { path: bootconfig })?;
    let config = bootconfig::parse(&data).context(error::BootconfigSnafu {
        operation: "decode bootconfig",
    })?;
    let cmdline = config
        .uki_cmdline(kernel_args, init_args)
        .context(error::BootconfigSnafu {
            operation: "construct UKI command line",
        })?;
    std::fs::write(output, cmdline).context(error::FileWriteSnafu { path: output })
}

fn derive_stub(uki_path: &Path, stub_path: &Path) -> Result<()> {
    let mut image = PeImage::load(uki_path).context(error::UkiReadSnafu { path: uki_path })?;

    image
        .remove_signature()
        .context(error::RemoveSignatureSnafu)?;

    image
        .derive_stub_by_truncating_trailing_sections(TRAILING_SECTIONS_TO_REMOVE)
        .context(error::DeriveStubSnafu)?;

    image
        .write_to(stub_path)
        .context(error::StubWriteSnafu { path: stub_path })?;

    eprintln!(
        "ukisys: stripped signature and trailing sections {:?} from '{}', wrote {} bytes to '{}'",
        TRAILING_SECTIONS_TO_REMOVE,
        uki_path.display(),
        image.bytes.len(),
        stub_path.display(),
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Encode bootconfig text the way the SDK's generator does: NUL-padded to a
    /// four-byte boundary, followed by the size, byte-sum checksum, and magic.
    fn bootconfig_data(text: &str) -> Vec<u8> {
        let mut data = text.as_bytes().to_vec();
        data.resize(text.len() + 4 - text.len() % 4, 0);
        let size = data.len() as u32;
        let checksum = data.iter().fold(0u32, |sum, b| sum + u32::from(*b));
        data.extend(size.to_le_bytes());
        data.extend(checksum.to_le_bytes());
        data.extend(b"#BOOTCONFIG\n");
        data
    }

    #[test]
    fn cmdline_joins_bootconfig_with_image_builder_arguments() {
        let dir = tempfile::tempdir().unwrap();
        let input = dir.path().join("bootconfig.data");
        let output = dir.path().join("cmdline");
        let data = bootconfig_data(concat!(
            "kernel.console = tty0, ttyS0\n",
            "init.systemd.unit = fipscheck.target\n",
        ));
        std::fs::write(&input, data).unwrap();
        cmdline(
            &input,
            "root=/dev/dm-0 fips=1",
            "systemd.log_target=journal",
            &output,
        )
        .unwrap();
        assert_eq!(
            std::fs::read_to_string(output).unwrap(),
            concat!(
                "console=tty0 console=ttyS0 root=/dev/dm-0 fips=1 ",
                "-- systemd.unit=fipscheck.target systemd.log_target=journal",
            )
        );
    }

    #[test]
    fn invalid_input_does_not_publish_cmdline_output() {
        let dir = tempfile::tempdir().unwrap();
        let input = dir.path().join("bad.data");
        let output = dir.path().join("cmdline");
        std::fs::write(&input, b"not a bootconfig").unwrap();
        assert!(matches!(
            cmdline(&input, "", "", &output),
            Err(error::Error::Bootconfig { .. })
        ));
        assert!(!output.exists());
    }

    #[test]
    fn merge_bootconfig_combines_dropins_in_place() {
        let dir = tempfile::tempdir().unwrap();
        let input = dir.path().join("bootconfig.in");
        let snippets = concat!(
            "kernel.module_blacklist = i8042\n",
            "kernel.module_blacklist = binfmt_misc\n",
            "kernel.module_blacklist = i8042\n",
            "kernel.systemd.mask = proc-sys-fs-binfmt_misc.mount\n",
        );
        std::fs::write(&input, snippets).unwrap();
        merge_bootconfig(&input, &input).unwrap();
        assert_eq!(
            std::fs::read_to_string(&input).unwrap(),
            concat!(
                "kernel.systemd.mask = proc-sys-fs-binfmt_misc.mount\n",
                "kernel.module_blacklist = \"binfmt_misc,i8042\"\n",
            )
        );
    }

    #[test]
    fn merge_bootconfig_preserves_failed_input() {
        let dir = tempfile::tempdir().unwrap();
        let input = dir.path().join("bootconfig.in");
        let invalid = "kernel.module_blacklist = \"i8042,,binfmt_misc\"\n";
        std::fs::write(&input, invalid).unwrap();
        assert!(matches!(
            merge_bootconfig(&input, &input),
            Err(error::Error::Bootconfig { .. })
        ));
        assert_eq!(std::fs::read_to_string(input).unwrap(), invalid);
    }
}
