//! ukisys: merges bootconfig drop-ins and constructs the kernel command line
//! for UKIs, and derives the unsigned systemd-stub PE a Bottlerocket UKI was
//! built from, by stripping its Authenticode signature and truncating the
//! trailing payload sections. Those sections are `.osrel`, `.cmdline`,
//! `.uname`, and `.linux`.

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
    about = "Command-line construction and PE section removal for Bottlerocket Unified Kernel Images"
)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

/// Subcommands for UKI repack section removal and command line construction.
#[derive(Subcommand)]
enum Command {
    /// Combine the drop-ins' module_blacklist and initcall_blacklist values before
    /// generating binary bootconfig.
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

/// Merge the drop-in contributions in the bootconfig input text.
fn merge_bootconfig(input: &Path, output: &Path) -> Result<()> {
    let text = std::fs::read_to_string(input).with_whatever_context(|_| {
        format!(
            "Failed to read collected bootconfig drop-ins '{}'",
            input.display()
        )
    })?;
    let merged = bootconfig::merge_parameters(&text).with_whatever_context(|_| {
        format!("Failed to merge bootconfig drop-ins '{}'", input.display())
    })?;
    std::fs::write(output, merged).with_whatever_context(|_| {
        format!(
            "Failed to write merged bootconfig drop-ins '{}'",
            output.display()
        )
    })
}

fn cmdline(bootconfig: &Path, kernel_args: &str, init_args: &str, output: &Path) -> Result<()> {
    let data = std::fs::read(bootconfig).with_whatever_context(|_| {
        format!("Failed to read bootconfig '{}'", bootconfig.display())
    })?;
    let cmdline = bootconfig::compose_uki_cmdline(&data, kernel_args, init_args)
        .with_whatever_context(|_| "Failed to construct UKI command line".to_string())?;
    std::fs::write(output, cmdline).with_whatever_context(|_| {
        format!("Failed to write UKI command line '{}'", output.display())
    })
}

fn derive_stub(uki_path: &Path, stub_path: &Path) -> Result<()> {
    let mut image = PeImage::load(uki_path)
        .with_whatever_context(|_| format!("Failed to parse UKI '{}'", uki_path.display()))?;

    image
        .remove_signature()
        .with_whatever_context(|_| "Failed to remove signature".to_string())?;

    image
        .derive_stub_by_truncating_trailing_sections(TRAILING_SECTIONS_TO_REMOVE)
        .with_whatever_context(|_| "Failed to derive stub".to_string())?;

    image
        .write_to(stub_path)
        .with_whatever_context(|_| format!("Failed to write stub '{}'", stub_path.display()))?;

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

    use bootconfig::test_util::encode as bootconfig_data;

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
        let written = std::fs::read_to_string(output).unwrap();
        assert_eq!(
            written,
            concat!(
                "console=tty0 console=ttyS0 root=/dev/dm-0 fips=1 ",
                "-- systemd.unit=fipscheck.target systemd.log_target=journal",
            )
        );
        // No trailing newline: `ukify` copies an `@path` argument into the
        // `.cmdline` section verbatim, so one would be measured into PCR 9.
        assert!(!written.ends_with('\n'));
    }

    #[test]
    fn invalid_input_does_not_publish_cmdline_output() {
        let dir = tempfile::tempdir().unwrap();
        let input = dir.path().join("bad.data");
        let output = dir.path().join("cmdline");
        std::fs::write(&input, b"not a bootconfig").unwrap();
        assert!(cmdline(&input, "", "", &output).is_err());
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
        // A list key is emitted once, with its values sorted and deduplicated,
        // other keys as written, and the keys sorted, so the result does not
        // depend on the order the packages contributed in.
        assert_eq!(
            std::fs::read_to_string(&input).unwrap(),
            concat!(
                "kernel.module_blacklist = \"binfmt_misc,i8042\"\n",
                "kernel.systemd.mask = proc-sys-fs-binfmt_misc.mount\n",
            )
        );
    }
}
