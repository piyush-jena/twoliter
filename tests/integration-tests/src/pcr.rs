use crate::{run_command, TWOLITER_PATH};
use std::fs::write;
use std::path::Path;
use std::process::{Command, Output};
use tempfile::TempDir;

/// Secure Boot variables that pcrsys accepts, so that it goes on to read the disk image.
const EFI_VARS: &str = r#"{
  "variables": [
    { "name": "PK", "guid": "8be4df61-93ca-11d2-aa0d-00e098032b8c", "data": "00" },
    { "name": "KEK", "guid": "8be4df61-93ca-11d2-aa0d-00e098032b8c", "data": "00" },
    { "name": "db", "guid": "d719b2cb-3d3a-4596-a3bc-dad00e67656f", "data": "00" },
    { "name": "dbx", "guid": "d719b2cb-3d3a-4596-a3bc-dad00e67656f", "data": "00" }
  ]
}"#;

/// The error pcrsys reports for the disk image from `invalid_gpt_image`, which shows that it read
/// that image.
const INVALID_GPT_ERROR: &str = "GPT header claims 4096 partition entries while max is 128";

/// A disk image whose GPT header claims more partition entries than pcrsys allows.
fn invalid_gpt_image() -> Vec<u8> {
    let mut image = vec![0; 1024];
    image[512..520].copy_from_slice(b"EFI PART");
    image[592..596].copy_from_slice(&4096u32.to_le_bytes());
    image
}

fn twoliter_pcr_disk(image: &Path, efi_vars: &Path) -> Output {
    let args = [
        "--log-level",
        "info",
        "pcr",
        "disk",
        "--image",
        image.to_str().unwrap(),
        "--efi-vars",
        efi_vars.to_str().unwrap(),
    ];
    run_command(TWOLITER_PATH, args, [])
}

/// Writes efi-vars.json and the disk image to `dir`, and returns their paths.
fn write_inputs(dir: &Path) -> (std::path::PathBuf, std::path::PathBuf) {
    let image = dir.join("bottlerocket.img");
    let efi_vars = dir.join("efi-vars.json");
    write(&image, invalid_gpt_image()).unwrap();
    write(&efi_vars, EFI_VARS).unwrap();
    (image, efi_vars)
}

#[test]
fn test_pcr_disk_raw_image() {
    let dir = TempDir::new().unwrap();
    let (image, efi_vars) = write_inputs(dir.path());

    let output = twoliter_pcr_disk(&image, &efi_vars);

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains(INVALID_GPT_ERROR), "{stderr}");
    assert!(!stderr.contains("Decompressing"), "{stderr}");
}

#[test]
fn test_pcr_disk_lz4_image() {
    let dir = TempDir::new().unwrap();
    let (image, efi_vars) = write_inputs(dir.path());
    // Compress the image the way image builds do.
    let status = Command::new("lz4")
        .args(["-9", "-q"])
        .arg(&image)
        .status()
        .expect("lz4 must be installed");
    assert!(status.success());
    let compressed = dir.path().join("bottlerocket.img.lz4");

    let output = twoliter_pcr_disk(&compressed, &efi_vars);

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Decompressing"), "{stderr}");
    assert!(stderr.contains(INVALID_GPT_ERROR), "{stderr}");
}

#[test]
fn test_pcr_disk_missing_image() {
    let dir = TempDir::new().unwrap();
    let (_, efi_vars) = write_inputs(dir.path());

    let output = twoliter_pcr_disk(&dir.path().join("nonexistent.img"), &efi_vars);

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("nonexistent.img"), "{stderr}");
}
