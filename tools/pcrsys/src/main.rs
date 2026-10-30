//! PCR prediction tool for Bottlerocket.
//!
//! Predicts TPM Platform Configuration Register (PCR) values based on
//! boot components, EFI variables, and GPT partition tables.

mod aws;
mod boot;
mod disk;
mod diskfs;
mod efi;
mod error;
mod gpt;
mod pcrs;
mod pe;
mod platform;
mod predict;

use aws_config::profile::ProfileFileCredentialsProvider;
use aws_types::region::Region;
use aws_types::SdkConfig;
use clap::{Parser, Subcommand};
use coldsnap::SnapshotDownloader;
use snafu::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::boot::BootChain;
use crate::disk::{DiskImage, FileImage};
use crate::efi::SecureBootVars;
use crate::error::Result;
use crate::gpt::Gpt;
use crate::platform::Platform;
use crate::predict::{PcrContext, PcrPredictions};

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

    let predictions = match &args.command {
        Command::Disk {
            image,
            efi_vars,
            platform,
        } => run_disk(image, efi_vars, *platform)?,
        Command::Ami {
            ami_id,
            region,
            profile,
        } => run_ami(ami_id, region.as_deref(), profile.as_deref()).await?,
    };
    let json = serde_json::to_string_pretty(&predictions)
        .whatever_context("failed to serialize predictions")?;
    println!("{json}");
    Ok(())
}

/// Predict PCRs from a local disk image file.
fn run_disk(image: &Path, efi_vars_path: &Path, platform: Platform) -> Result<PcrPredictions> {
    let efi_vars_json = fs::read_to_string(efi_vars_path).whatever_context(format!(
        "failed to read efi-vars: {}",
        efi_vars_path.display()
    ))?;
    let disk = FileImage::open(image)
        .whatever_context(format!("failed to open disk image: {}", image.display()))?;
    let efi_vars: efi::EfiVars =
        serde_json::from_str(&efi_vars_json).whatever_context("failed to parse efi-vars.json")?;
    let efi_vars = SecureBootVars::from_json(&efi_vars)?;
    predict_pcrs(&efi_vars, Arc::new(disk), platform)
}

/// Predict PCRs from an AWS AMI by downloading its snapshot.
async fn run_ami(
    ami_id: &str,
    region: Option<&str>,
    profile: Option<&str>,
) -> Result<PcrPredictions> {
    let config = build_client_config(region, profile).await;

    let ec2_client = aws_sdk_ec2::Client::new(&config);
    let ebs_client = aws_sdk_ebs::Client::new(&config);

    let efi_vars = aws::ami::get_uefi_data(&ec2_client, ami_id).await?;

    let snapshot_id = aws::ami::get_root_snapshot_id(&ec2_client, ami_id).await?;

    let temp_file =
        tempfile::NamedTempFile::new().whatever_context("failed to create temp file")?;

    let downloader = SnapshotDownloader::new(ebs_client);
    downloader
        .download_to_file(&snapshot_id, temp_file.path(), None, None, None)
        .await
        .whatever_context("failed to download snapshot")?;

    let disk =
        FileImage::open(temp_file.path()).whatever_context("failed to open downloaded snapshot")?;
    predict_pcrs(&efi_vars, Arc::new(disk), Platform::Aws)
}

/// Build AWS SDK config, handling region and profile options like coldsnap.
async fn build_client_config(region: Option<&str>, profile: Option<&str>) -> SdkConfig {
    let config = match (region, profile) {
        (Some(r), _) => aws_config::from_env().region(Region::new(r.to_string())),
        (None, Some(p)) => aws_config::from_env().region(
            aws_config::profile::ProfileFileRegionProvider::builder()
                .profile_name(p)
                .build(),
        ),
        (None, None) => aws_config::from_env(),
    };

    let config = match profile {
        Some(p) => config.credentials_provider(
            ProfileFileCredentialsProvider::builder()
                .profile_name(p)
                .build(),
        ),
        None => config,
    };

    config.load().await
}

/// Predict PCRs from the Secure Boot variables and a disk image.
fn predict_pcrs(
    efi_vars: &SecureBootVars,
    disk: Arc<dyn DiskImage>,
    platform: Platform,
) -> Result<PcrPredictions> {
    let gpt = Gpt::read(disk.as_ref())?;
    let chain = BootChain::read(&disk, &gpt, platform)?;
    let ctx = PcrContext {
        platform,
        efi_vars,
        gpt: &gpt,
        chain: &chain,
    };

    PcrPredictions::new()
        .try_extend(|| pcrs::pcr0::predict(&ctx))?
        .try_extend(|| pcrs::pcr1::predict(&ctx))?
        .try_extend(|| pcrs::pcr2::predict(&ctx))?
        .try_extend(|| pcrs::pcr3::predict(&ctx))?
        .try_extend(|| pcrs::pcr4::predict(&ctx))?
        .try_extend(|| pcrs::pcr5::predict(&ctx))?
        .try_extend(|| pcrs::pcr6::predict(&ctx))?
        .try_extend(|| pcrs::pcr7::predict(&ctx))?
        .try_extend(|| pcrs::pcr9::predict(&ctx))?
        .try_extend(|| pcrs::pcr10::predict(&ctx))?
        .try_extend(|| pcrs::pcr11::predict(&ctx))?
        .try_extend(|| pcrs::pcr12::predict(&ctx))?
        .try_extend(|| pcrs::pcr13::predict(&ctx))?
        .try_extend(|| pcrs::pcr14::predict(&ctx))?
        .try_extend(|| pcrs::pcr15::predict(&ctx))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::disk::tests::CountingImage;
    use crate::pcrs::pcr7::tests::build_x509_esl;
    use crate::pe::tests::{
        build_test_pe, build_test_shim, build_test_shim_for, build_test_uki, with_cert_table,
    };
    use crate::pe::Arch;
    use flate2::read::GzDecoder;
    use std::io::Read;

    /// The disk fixtures, each with the platforms it is predicted for. VMware runs x86_64 only.
    const FIXTURES: &[(&str, &[Platform])] = &[
        ("ab", &[Platform::Aws, Platform::Vmware, Platform::Metal]),
        ("single-bank-aarch64", &[Platform::Aws, Platform::Metal]),
        (
            "single-bank-x86_64",
            &[Platform::Aws, Platform::Vmware, Platform::Metal],
        ),
        ("uki", &[Platform::Aws]),
    ];

    fn fixtures() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
    }

    /// A disk fixture's image.
    fn image(name: &str) -> Vec<u8> {
        let mut image = Vec::new();
        let compressed = fs::File::open(fixtures().join(format!("{name}.img.gz"))).unwrap();
        GzDecoder::new(compressed).read_to_end(&mut image).unwrap();
        image
    }

    /// Predict a disk fixture's PCRs, as `pcrsys disk` reads and prints them.
    fn predictions(name: &str, platform: Platform) -> Result<String> {
        let file = tempfile::NamedTempFile::new().unwrap();
        fs::write(file.path(), image(name)).unwrap();
        let predictions = run_disk(file.path(), &fixtures().join("efi-vars.json"), platform)?;
        Ok(serde_json::to_string_pretty(&predictions).unwrap() + "\n")
    }

    fn golden(name: &str, platform: Platform) -> PathBuf {
        fixtures().join(format!("{name}.{platform}.json"))
    }

    #[test]
    fn disk_fixtures_match_goldens() {
        for (name, platforms) in FIXTURES {
            for &platform in *platforms {
                let expected = fs::read_to_string(golden(name, platform)).unwrap();
                assert_eq!(
                    predictions(name, platform).unwrap(),
                    expected,
                    "{name} on {platform}"
                );
            }
        }
    }

    #[test]
    fn uki_fixture_is_predicted_for_aws_only() {
        for platform in [Platform::Vmware, Platform::Metal] {
            let err = predictions("uki", platform).unwrap_err();
            assert!(err.to_string().contains("AWS only"), "{err}");
        }
    }

    #[test]
    #[ignore = "regenerates the committed fixtures; needs sgdisk, mtools and mkfs.ext4"]
    fn regenerate_disk_fixtures() {
        // Shaped like the grub.cfg rpm2img writes, with GRUB's $boot_uuid placeholder.
        const GRUB_CFG: &str = r#"set default="0"
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
        let input = tempfile::tempdir().unwrap();
        let grub = with_cert_table(build_test_pe(0xaa64, &[(".text", b"fixture grub")]));
        let vmlinuz = with_cert_table(build_test_pe(0xaa64, &[(".text", b"fixture vmlinuz")]));
        let bootconfig = bootconfig::test_util::encode(
            "kernel.module_blacklist = i8042\ninit.systemd.unit = multi-user.target\n",
        );
        let x86_64_pe = |text| with_cert_table(build_test_pe(0x8664, &[(".text", text)]));
        fs::create_dir(input.path().join("x86_64")).unwrap();
        for (name, contents) in [
            ("bootaa64.efi", build_test_shim()),
            ("grubaa64.efi", grub),
            ("vmlinuz", vmlinuz),
            ("x86_64/bootx64.efi", build_test_shim_for(0x8664)),
            ("x86_64/grubx64.efi", x86_64_pe(b"fixture grub")),
            ("x86_64/vmlinuz", x86_64_pe(b"fixture vmlinuz")),
            ("BOOTX64.EFI", build_test_uki(0x8664)),
            ("grub.cfg", GRUB_CFG.as_bytes().to_vec()),
            ("bootconfig.data", bootconfig),
        ] {
            fs::write(input.path().join(name), contents).unwrap();
        }
        let status = std::process::Command::new(fixtures().join("make-disks.sh"))
            .arg(input.path())
            .arg(fixtures())
            .status()
            .unwrap();
        assert!(status.success(), "make-disks.sh failed");

        let esl = hex::encode(build_x509_esl());
        let variable = |name, guid| serde_json::json!({ "name": name, "guid": guid, "data": esl });
        let efi_vars = serde_json::json!({ "variables": [
            variable("PK", "8be4df61-93ca-11d2-aa0d-00e098032b8c"),
            variable("KEK", "8be4df61-93ca-11d2-aa0d-00e098032b8c"),
            variable("db", "d719b2cb-3d3a-4596-a3bc-dad00e67656f"),
            variable("dbx", "d719b2cb-3d3a-4596-a3bc-dad00e67656f"),
        ]});
        let efi_vars = serde_json::to_string_pretty(&efi_vars).unwrap() + "\n";
        fs::write(fixtures().join("efi-vars.json"), efi_vars).unwrap();

        for (name, platforms) in FIXTURES {
            for &platform in *platforms {
                fs::write(golden(name, platform), predictions(name, platform).unwrap()).unwrap();
            }
        }
    }

    /// The architecture of the kernel decides how GRUB starts it, and so what PCR 9 measures.
    #[test]
    fn the_kernel_architecture_is_read_from_vmlinuz() {
        for (name, arch) in [
            ("single-bank-aarch64", Arch::Aarch64),
            ("single-bank-x86_64", Arch::X86_64),
        ] {
            let disk: Arc<dyn DiskImage> = Arc::new(image(name));
            let gpt = Gpt::read(disk.as_ref()).unwrap();
            match BootChain::read(&disk, &gpt, Platform::Aws).unwrap() {
                BootChain::GrubSingle { payload, .. } => assert_eq!(payload.arch, arch, "{name}"),
                _ => panic!("{name} boots a single bank through GRUB"),
            }
        }
    }

    /// The partitions an A/B image does not measure are only checked, not read.
    #[test]
    fn ab_images_read_only_metadata_from_unmeasured_partitions() {
        let efi_vars: efi::EfiVars =
            serde_json::from_str(&fs::read_to_string(fixtures().join("efi-vars.json")).unwrap())
                .unwrap();
        let efi_vars = SecureBootVars::from_json(&efi_vars).unwrap();
        for name in ["ab", "single-bank-aarch64", "single-bank-x86_64", "uki"] {
            let disk = Arc::new(CountingImage::new(image(name)));
            predict_pcrs(&efi_vars, disk.clone(), Platform::Aws).unwrap();
            // Every filesystem is a few MiB, so reading one whole would show.
            let total: u64 = disk.reads.lock().unwrap().iter().map(|&(_, len)| len).sum();
            assert!(total < 512 * 1024, "{name}: read {total} bytes");
            if name == "ab" {
                let layout = Gpt::read(disk.as_ref()).unwrap().grub_layout().unwrap();
                for partition in [&layout.boot_a, &layout.private] {
                    let read = disk.bytes_read_from(partition);
                    let size = partition.size_bytes();
                    assert!(
                        read < size / 8,
                        "{name}: read {read} of {size} bytes of partition {}",
                        partition.number
                    );
                }
            }
        }
    }

    /// What pcrsys does not model must be absent from the fixtures it is tested on.
    #[test]
    fn fixtures_carry_none_of_the_unmodeled_inputs() {
        use crate::diskfs::Esp;
        use object::read::pe::PeFile64;
        use object::{Object, ObjectSection};

        let efi_vars = fs::read_to_string(fixtures().join("efi-vars.json")).unwrap();
        assert!(!efi_vars.contains("\"dbt\""), "dbt is not modeled in PCR 7");
        for name in ["ab", "single-bank-aarch64", "single-bank-x86_64", "uki"] {
            let disk: Arc<dyn DiskImage> = Arc::new(image(name));
            let gpt = Gpt::read(disk.as_ref()).unwrap();
            let esp = Esp::mount(&disk, &gpt.esp().unwrap()).unwrap();
            for path in esp.paths() {
                let file = path.rsplit('/').next().unwrap();
                assert!(
                    !path.starts_with("/loader") && !path.ends_with(".extra.d"),
                    "{name}: systemd-stub add-ons or credentials at {path}"
                );
                assert!(
                    !file.starts_with("revocations")
                        && !file.starts_with("shim_certificate")
                        && !file.starts_with("fb"),
                    "{name}: unmodeled shim input {path}"
                );
            }
            if name == "uki" {
                let uki = esp.fallback().unwrap();
                let pe = PeFile64::parse(uki.as_slice()).unwrap();
                for section in pe.sections() {
                    let raw_name = section.pe_section().raw_name();
                    assert_ne!(raw_name.first(), Some(&b'/'), "{name}: a long section name");
                    assert_ne!(section.name().unwrap(), ".profile", "{name}: a UKI profile");
                }
                for required in [".osrel", ".linux"] {
                    let section = pe.section_by_name(required).expect(required);
                    assert!(
                        !section.data().unwrap().is_empty(),
                        "{name}: empty {required}"
                    );
                }
            }
        }
    }
}
