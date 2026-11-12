//! AMI-based PCR prediction support.

use super::{build_client_config, parse_aws_uefi_data};
use crate::efi::EfiVars;
use crate::error::Result;
use crate::image::DiskImage;
use crate::platform::Platform;
use crate::predict::PcrPredictions;
use aws_sdk_ec2::types::ImageAttributeName;
use aws_sdk_ec2::Client;
use coldsnap::SnapshotDownloader;
use snafu::{OptionExt, ResultExt};
use std::fs;

/// Predict the PCRs of an AMI by downloading its root snapshot.
pub async fn predict(
    ami_id: &str,
    region: Option<&String>,
    profile: Option<&String>,
) -> Result<PcrPredictions> {
    let config = build_client_config(region, profile).await;

    let ec2_client = Client::new(&config);
    let ebs_client = aws_sdk_ebs::Client::new(&config);

    let efi_vars = get_uefi_data(&ec2_client, ami_id).await?;

    let snapshot_id = get_root_snapshot_id(&ec2_client, ami_id).await?;

    let temp_file =
        tempfile::NamedTempFile::new().whatever_context("failed to create temp file")?;

    let downloader = SnapshotDownloader::new(ebs_client);
    downloader
        .download_to_file(&snapshot_id, temp_file.path(), None, None, None)
        .await
        .whatever_context("failed to download snapshot")?;

    let mut disk =
        fs::File::open(temp_file.path()).whatever_context("failed to open downloaded snapshot")?;

    DiskImage::parse(&mut disk)?.predict(&efi_vars, Platform::Aws)
}

/// Retrieve UEFI data from an AMI using DescribeImageAttribute.
pub async fn get_uefi_data(ec2_client: &Client, ami_id: &str) -> Result<EfiVars> {
    let response = ec2_client
        .describe_image_attribute()
        .image_id(ami_id)
        .attribute(ImageAttributeName::UefiData)
        .send()
        .await
        .map_err(|e| error_utils::format_aws_sdk_error(&e))
        .whatever_context("failed to describe image attribute")?;

    let uefi_b64 = response
        .uefi_data()
        .and_then(|v| v.value())
        .whatever_context("AMI has no UEFI data (not a UEFI boot mode AMI?)")?;

    parse_aws_uefi_data(uefi_b64)
}

/// Find the root snapshot ID from an AMI.
pub async fn get_root_snapshot_id(ec2_client: &Client, ami_id: &str) -> Result<String> {
    let response = ec2_client
        .describe_images()
        .image_ids(ami_id)
        .send()
        .await
        .map_err(|e| error_utils::format_aws_sdk_error(&e))
        .whatever_context("failed to describe images")?;

    let image = response
        .images()
        .first()
        .whatever_context("AMI not found")?;

    let root_device = image.root_device_name().unwrap_or("/dev/xvda");

    image
        .block_device_mappings()
        .iter()
        .find(|bdm| bdm.device_name() == Some(root_device))
        .and_then(|bdm| bdm.ebs())
        .and_then(|ebs| ebs.snapshot_id())
        .map(String::from)
        .whatever_context("no root snapshot found in AMI")
}
