//! PCR prediction tool for Bottlerocket.
//!
//! Predicts TPM Platform Configuration Register (PCR) values based on
//! boot components, EFI variables, and GPT partition tables.

use clap::{Parser, Subcommand};
use snafu::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};

use pcrsys::aws;
use pcrsys::efi::EfiVars;
use pcrsys::error::Result;
use pcrsys::image::DiskImage;
use pcrsys::platform::Platform;
use pcrsys::predict::PcrPredictions;

/// Command-line arguments for pcrsys.
#[derive(Parser)]
#[command(version, about = "Predict TPM PCR values for Bottlerocket")]
struct Args {
    #[command(subcommand)]
    command: Command,
}

/// Subcommands for PCR prediction from different sources.
#[derive(Subcommand)]
enum Command {
    /// Predict PCRs from a local disk image
    Disk {
        /// Path to disk image containing GPT, ESP, and boot partitions
        #[arg(long)]
        image: PathBuf,

        /// Path to efi-vars.json containing Secure Boot variables
        #[arg(long)]
        efi_vars: PathBuf,

        /// Target platform (aws, vmware, metal)
        #[arg(long, value_enum, default_value_t = Platform::Aws)]
        platform: Platform,
    },

    /// Predict PCRs from an AWS AMI
    Ami {
        /// AMI ID (e.g., ami-0123456789abcdef0)
        #[arg(long)]
        ami_id: String,

        /// AWS region to use
        #[arg(long)]
        region: Option<String>,

        /// AWS profile to use
        #[arg(long)]
        profile: Option<String>,
    },
}

#[snafu::report]
#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    match &args.command {
        Command::Disk {
            image,
            efi_vars,
            platform,
        } => run_disk(image, efi_vars, *platform),
        Command::Ami {
            ami_id,
            region,
            profile,
        } => run_ami(ami_id, region.as_ref(), profile.as_ref()).await,
    }
}

/// Run PCR prediction from a local disk image file.
fn run_disk(image: &Path, efi_vars_path: &Path, platform: Platform) -> Result<()> {
    let efi_vars = EfiVars::load(efi_vars_path)?;

    let mut disk = fs::File::open(image)
        .whatever_context(format!("failed to open disk image: {}", image.display()))?;

    print_predictions(&DiskImage::parse(&mut disk)?.predict(&efi_vars, platform)?)
}

/// Run PCR prediction from an AWS AMI by downloading its snapshot.
async fn run_ami(ami_id: &str, region: Option<&String>, profile: Option<&String>) -> Result<()> {
    print_predictions(&aws::ami::predict(ami_id, region, profile).await?)
}

/// Print PCR predictions to stdout as JSON.
fn print_predictions(predictions: &PcrPredictions) -> Result<()> {
    let json = serde_json::to_string_pretty(predictions)
        .whatever_context("failed to serialize predictions")?;
    println!("{json}");

    Ok(())
}
