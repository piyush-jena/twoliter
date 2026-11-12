use anyhow::{Context, Result};
use clap::Parser;
use lz4_flex::frame::FrameDecoder;
use pcrsys::efi::EfiVars;
use pcrsys::image::DiskImage;
use pcrsys::platform::Platform;
use pcrsys::predict::PcrPredictions;
use std::fs::File;
use std::io::{BufReader, ErrorKind, Read, Seek};
use std::path::{Path, PathBuf};
use tracing::info;

/// The magic number at the start of an LZ4 frame, such as the `.img.lz4` files from image builds.
const LZ4_FRAME_MAGIC: [u8; 4] = [0x04, 0x22, 0x4d, 0x18];

/// Predict TPM PCR values for a Bottlerocket disk image or AMI.
#[derive(Debug, Parser)]
pub(crate) enum PcrCommand {
    Disk(PcrDisk),
    Ami(PcrAmi),
}

impl PcrCommand {
    pub(crate) async fn run(self) -> Result<()> {
        match self {
            PcrCommand::Disk(command) => command.run(),
            PcrCommand::Ami(command) => command.run().await,
        }
    }
}

/// Predict PCRs from a local disk image
#[derive(Debug, Parser)]
pub(crate) struct PcrDisk {
    /// Path to the disk image, which may be lz4-compressed like the `.img.lz4` files from builds
    #[clap(long)]
    image: PathBuf,

    /// Path to efi-vars.json containing Secure Boot variables
    #[clap(long)]
    efi_vars: PathBuf,

    /// Target platform
    #[clap(long, value_enum, default_value_t = Platform::Aws)]
    platform: Platform,
}

impl PcrDisk {
    pub(super) fn run(&self) -> Result<()> {
        let efi_vars = EfiVars::load(&self.efi_vars)?;
        let mut image = open_image(&self.image)?;
        print_predictions(&DiskImage::parse(&mut image)?.predict(&efi_vars, self.platform)?)
    }
}

/// Predict PCRs from an AWS AMI
#[derive(Debug, Parser)]
pub(crate) struct PcrAmi {
    /// AMI ID (e.g., ami-0123456789abcdef0)
    #[clap(long)]
    ami_id: String,

    /// AWS region to use
    #[clap(long)]
    region: Option<String>,

    /// AWS profile to use
    #[clap(long)]
    profile: Option<String>,
}

impl PcrAmi {
    pub(super) async fn run(&self) -> Result<()> {
        let predictions =
            pcrsys::aws::ami::predict(&self.ami_id, self.region.as_ref(), self.profile.as_ref())
                .await?;
        print_predictions(&predictions)
    }
}

/// Open the disk image at `path`. pcrsys reads an image out of order, so an lz4-compressed image is
/// decompressed into an anonymous temporary file first.
fn open_image(path: &Path) -> Result<File> {
    let mut file = File::open(path).context(format!("Unable to open '{}'", path.display()))?;
    let mut magic = [0; 4];
    let compressed = match file.read_exact(&mut magic) {
        Ok(()) => magic == LZ4_FRAME_MAGIC,
        // A file too short to hold the magic number is not compressed.
        Err(e) if e.kind() == ErrorKind::UnexpectedEof => false,
        Err(e) => return Err(e).context(format!("Unable to read '{}'", path.display())),
    };
    file.rewind()
        .context(format!("Unable to read '{}'", path.display()))?;
    if !compressed {
        return Ok(file);
    }

    info!("Decompressing '{}'", path.display());
    let mut image =
        tempfile::tempfile().context("Unable to create a temporary file for the image")?;
    std::io::copy(&mut FrameDecoder::new(BufReader::new(file)), &mut image)
        .context(format!("Unable to decompress '{}'", path.display()))?;
    image
        .rewind()
        .context("Unable to read the decompressed image")?;
    Ok(image)
}

/// Print PCR predictions to stdout as JSON.
fn print_predictions(predictions: &PcrPredictions) -> Result<()> {
    let json = serde_json::to_string_pretty(predictions)
        .context("Unable to serialize the PCR predictions")?;
    println!("{json}");
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use lz4_flex::frame::FrameEncoder;
    use std::io::Write;
    use tempfile::NamedTempFile;

    /// Returns the contents of the image that `open_image` opens for a file holding `contents`.
    fn opened(contents: &[u8]) -> Vec<u8> {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(contents).unwrap();
        let mut image = Vec::new();
        open_image(file.path())
            .unwrap()
            .read_to_end(&mut image)
            .unwrap();
        image
    }

    #[test]
    fn test_raw_images_are_read_as_is() {
        let image: Vec<u8> = (0..=255).cycle().take(10_000).collect();
        assert_eq!(opened(&image), image);
        assert_eq!(opened(&[1, 2, 3]), [1, 2, 3]);
        assert_eq!(opened(&[]), Vec::<u8>::new());
    }

    #[test]
    fn test_lz4_images_are_decompressed() {
        let image: Vec<u8> = (0..=255).cycle().take(1_000_000).collect();
        let mut encoder = FrameEncoder::new(Vec::new());
        encoder.write_all(&image).unwrap();
        assert_eq!(opened(&encoder.finish().unwrap()), image);
    }

    #[test]
    fn test_corrupt_lz4_images_are_an_error() {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(&LZ4_FRAME_MAGIC).unwrap();
        file.write_all(b"not an lz4 frame").unwrap();
        let err = open_image(file.path()).unwrap_err();
        assert!(
            format!("{err}").starts_with("Unable to decompress"),
            "{err}"
        );
    }
}
