//! The filesystems holding boot components: the FAT ESP and the ext4 BOOT and PRIVATE partitions.

use crate::disk::{DiskImage, PartitionReader};
use crate::error::Result;
use crate::gpt::PartitionInfo;

use ext4_view::Ext4;
use fatfs::{FileSystem, FsOptions};
use snafu::{ensure_whatever, whatever, ResultExt};
use std::io::{ErrorKind, Read};
use std::sync::Arc;

/// The UEFI removable-media fallback path: shim on a GRUB image, the UKI itself on a UKI image.
const FALLBACK: &[&str] = &["bootx64.efi", "bootaa64.efi"];

/// The GRUB EFI binary that shim loads.
const GRUB: &[&str] = &["grubx64.efi", "grubaa64.efi"];

/// The ESP's FAT filesystem, mounted once.
pub struct Esp(FileSystem<PartitionReader>);

impl Esp {
    /// Mount the FAT filesystem on `esp`.
    pub fn mount(disk: &Arc<dyn DiskImage>, esp: &PartitionInfo) -> Result<Self> {
        FileSystem::new(
            PartitionReader::new(Arc::clone(disk), esp),
            FsOptions::new(),
        )
        .map(Self)
        .whatever_context("failed to mount ESP")
    }

    /// Read the architecture's firmware fallback executable.
    pub fn fallback(&self) -> Result<Vec<u8>> {
        self.read(FALLBACK)
    }

    /// Read the architecture's GRUB EFI binary.
    pub fn grub(&self) -> Result<Vec<u8>> {
        self.read(GRUB)
    }

    /// Check that the ESP holds one GRUB EFI binary, without reading it.
    pub fn ensure_grub(&self) -> Result<()> {
        self.open(GRUB).map(drop)
    }

    /// Open the one file from `names` that `/EFI/BOOT/` holds.
    fn open(
        &self,
        names: &[&'static str],
    ) -> Result<(&'static str, fatfs::File<'_, PartitionReader>)> {
        let boot_dir = self
            .0
            .root_dir()
            .open_dir("EFI/BOOT")
            .whatever_context("/EFI/BOOT not found on ESP")?;
        let mut found = None;
        for &name in names {
            match boot_dir.open_file(name) {
                Ok(file) => {
                    ensure_whatever!(
                        found.is_none(),
                        "ambiguous ESP: more than one of {names:?} in /EFI/BOOT/"
                    );
                    found = Some((name, file));
                }
                Err(e) if e.kind() == ErrorKind::NotFound => {}
                Err(e) => {
                    return Err(e).with_whatever_context(|_| format!("failed to open {name}"))
                }
            }
        }
        let Some(found) = found else {
            whatever!("none of {names:?} found in /EFI/BOOT/ on ESP");
        };
        Ok(found)
    }

    /// Every path on the ESP, lowercased, for tests to inspect.
    #[cfg(test)]
    pub fn paths(&self) -> Vec<String> {
        fn walk(dir: fatfs::Dir<'_, PartitionReader>, prefix: &str, paths: &mut Vec<String>) {
            for entry in dir.iter().map(|entry| entry.expect("readable directory")) {
                let name = entry.file_name().to_lowercase();
                if name == "." || name == ".." {
                    continue;
                }
                let path = format!("{prefix}/{name}");
                if entry.is_dir() {
                    walk(entry.to_dir(), &path, paths);
                }
                paths.push(path);
            }
        }
        let mut paths = Vec::new();
        walk(self.0.root_dir(), "", &mut paths);
        paths
    }

    /// Read the one file from `names` that `/EFI/BOOT/` holds.
    fn read(&self, names: &[&'static str]) -> Result<Vec<u8>> {
        let (name, mut file) = self.open(names)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)
            .with_whatever_context(|_| format!("failed to read {name}"))?;
        Ok(contents)
    }
}

/// An ext4 filesystem, mounted once.
pub struct Ext4Partition(Ext4);

impl Ext4Partition {
    /// Mount the ext4 filesystem on `partition`, named `name` in errors.
    pub fn mount(disk: &Arc<dyn DiskImage>, partition: &PartitionInfo, name: &str) -> Result<Self> {
        Ext4::load(Box::new(PartitionReader::new(Arc::clone(disk), partition)))
            .map(Self)
            .with_whatever_context(|_| format!("failed to mount {name} filesystem"))
    }

    /// Read the file at `path`.
    pub fn read(&self, path: &str) -> Result<Vec<u8>> {
        self.0
            .read(path)
            .with_whatever_context(|_| format!("failed to read {path}"))
    }

    /// Check that `path` is a regular file, without reading it.
    pub fn ensure_file(&self, path: &str) -> Result<()> {
        let metadata = self
            .0
            .metadata(path)
            .with_whatever_context(|_| format!("failed to read {path}"))?;
        ensure_whatever!(
            metadata.file_type().is_regular_file(),
            "failed to read {path}: not a regular file"
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::disk::tests::WRITE_ATTEMPTS;
    use std::sync::atomic::Ordering;

    /// Generate a minimal FAT12 filesystem image with /EFI/BOOT/ directory structure.
    fn generate_esp_image() -> Vec<u8> {
        const SIZE: usize = 1024 * 1024; // 1MB
        const SECTOR_SIZE: usize = 512;
        const CLUSTER_SIZE: usize = SECTOR_SIZE * 4; // 4 sectors per cluster
        const RESERVED_SECTORS: usize = 1;
        const NUM_FATS: usize = 2;
        const ROOT_ENTRIES: usize = 512;
        const ROOT_DIR_SECTORS: usize = (ROOT_ENTRIES * 32) / SECTOR_SIZE;
        const TOTAL_SECTORS: usize = SIZE / SECTOR_SIZE;
        const FAT_SECTORS: usize = 2; // enough for FAT12 on 1MB

        let mut img = vec![0u8; SIZE];

        // Boot sector (BPB)
        img[0..3].copy_from_slice(&[0xEB, 0x3C, 0x90]); // Jump + NOP
        img[3..11].copy_from_slice(b"mkfs.fat"); // OEM name
        img[11..13].copy_from_slice(&(SECTOR_SIZE as u16).to_le_bytes()); // bytes/sector
        img[13] = (CLUSTER_SIZE / SECTOR_SIZE) as u8; // sectors/cluster
        img[14..16].copy_from_slice(&(RESERVED_SECTORS as u16).to_le_bytes()); // reserved sectors
        img[16] = NUM_FATS as u8; // number of FATs
        img[17..19].copy_from_slice(&(ROOT_ENTRIES as u16).to_le_bytes()); // root entries
        img[19..21].copy_from_slice(&(TOTAL_SECTORS as u16).to_le_bytes()); // total sectors
        img[21] = 0xF8; // media type (fixed disk)
        img[22..24].copy_from_slice(&(FAT_SECTORS as u16).to_le_bytes()); // sectors/FAT
        img[24..26].copy_from_slice(&16u16.to_le_bytes()); // sectors/track
        img[26..28].copy_from_slice(&2u16.to_le_bytes()); // heads
        img[38] = 0x29; // extended boot signature
        img[39..43].copy_from_slice(&0x12345678u32.to_le_bytes()); // volume ID
        img[43..54].copy_from_slice(b"NO NAME    "); // volume label
        img[54..62].copy_from_slice(b"FAT12   "); // filesystem type
        img[510..512].copy_from_slice(&[0x55, 0xAA]); // boot signature

        let fat_start = RESERVED_SECTORS * SECTOR_SIZE;
        let root_dir_start = fat_start + NUM_FATS * FAT_SECTORS * SECTOR_SIZE;
        let data_start = root_dir_start + ROOT_DIR_SECTORS * SECTOR_SIZE;

        // Helper to write FAT12 entry
        let write_fat12 = |fat: &mut [u8], cluster: usize, value: u16| {
            let offset = cluster + cluster / 2;
            if cluster.is_multiple_of(2) {
                fat[offset] = (value & 0xFF) as u8;
                fat[offset + 1] = (fat[offset + 1] & 0xF0) | ((value >> 8) & 0x0F) as u8;
            } else {
                fat[offset] = (fat[offset] & 0x0F) | ((value << 4) & 0xF0) as u8;
                fat[offset + 1] = (value >> 4) as u8;
            }
        };

        // Initialize FAT (both copies)
        for fat_num in 0..NUM_FATS {
            let fat = &mut img[fat_start + fat_num * FAT_SECTORS * SECTOR_SIZE..];
            fat[0] = 0xF8; // media byte
            fat[1] = 0xFF;
            fat[2] = 0xFF;
            // Cluster 2: EFI dir -> EOF
            write_fat12(fat, 2, 0xFFF);
            // Cluster 3: BOOT dir -> EOF
            write_fat12(fat, 3, 0xFFF);
            // Cluster 4: bootaa64.efi -> EOF
            write_fat12(fat, 4, 0xFFF);
            // Cluster 5: grubaa64.efi -> EOF
            write_fat12(fat, 5, 0xFFF);
        }

        // Helper to create directory entry
        let make_dir_entry = |name: &[u8; 11], attr: u8, cluster: u16, size: u32| -> [u8; 32] {
            let mut entry = [0u8; 32];
            entry[0..11].copy_from_slice(name);
            entry[11] = attr;
            entry[26..28].copy_from_slice(&cluster.to_le_bytes());
            entry[28..32].copy_from_slice(&size.to_le_bytes());
            entry
        };

        // Root directory: EFI entry
        let efi_entry = make_dir_entry(b"EFI        ", 0x10, 2, 0);
        img[root_dir_start..root_dir_start + 32].copy_from_slice(&efi_entry);

        // EFI directory (cluster 2): . and .. and BOOT
        let cluster2_offset = data_start;
        let dot = make_dir_entry(b".          ", 0x10, 2, 0);
        let dotdot = make_dir_entry(b"..         ", 0x10, 0, 0);
        let boot_entry = make_dir_entry(b"BOOT       ", 0x10, 3, 0);
        img[cluster2_offset..cluster2_offset + 32].copy_from_slice(&dot);
        img[cluster2_offset + 32..cluster2_offset + 64].copy_from_slice(&dotdot);
        img[cluster2_offset + 64..cluster2_offset + 96].copy_from_slice(&boot_entry);

        // BOOT directory (cluster 3): . and .. and files
        let cluster3_offset = data_start + CLUSTER_SIZE;
        let dot = make_dir_entry(b".          ", 0x10, 3, 0);
        let dotdot = make_dir_entry(b"..         ", 0x10, 2, 0);
        let bootaa64 = make_dir_entry(b"BOOTAA64EFI", 0x00, 4, 12);
        let grubaa64 = make_dir_entry(b"GRUBAA64EFI", 0x00, 5, 12);
        img[cluster3_offset..cluster3_offset + 32].copy_from_slice(&dot);
        img[cluster3_offset + 32..cluster3_offset + 64].copy_from_slice(&dotdot);
        img[cluster3_offset + 64..cluster3_offset + 96].copy_from_slice(&bootaa64);
        img[cluster3_offset + 96..cluster3_offset + 128].copy_from_slice(&grubaa64);

        // File data
        let cluster4_offset = data_start + CLUSTER_SIZE * 2;
        let cluster5_offset = data_start + CLUSTER_SIZE * 3;
        img[cluster4_offset..cluster4_offset + 12].copy_from_slice(b"bootaa64.efi");
        img[cluster5_offset..cluster5_offset + 12].copy_from_slice(b"grubaa64.efi");

        img
    }

    /// The test ESP image's partition: the whole image.
    fn whole_image(image: &[u8]) -> PartitionInfo {
        PartitionInfo {
            number: 1,
            start_lba: 0,
            end_lba: u64::try_from(image.len() / 512).unwrap() - 1,
            unique_guid: [0; 16],
        }
    }

    fn mount_esp(image: Vec<u8>) -> Result<Esp> {
        let partition = whole_image(&image);
        let disk: Arc<dyn DiskImage> = Arc::new(image);
        Esp::mount(&disk, &partition)
    }

    #[test]
    fn test_read_fallback_and_grub() {
        let esp = mount_esp(generate_esp_image()).unwrap();
        assert_eq!(esp.fallback().unwrap(), b"bootaa64.efi");
        assert_eq!(esp.grub().unwrap(), b"grubaa64.efi");
        esp.ensure_grub().unwrap();
        drop(esp);
        assert_eq!(
            WRITE_ATTEMPTS.load(Ordering::Relaxed),
            0,
            "reading the ESP wrote to it"
        );
    }

    /// Generate ESP image with only x86_64 files (no aarch64).
    fn generate_esp_image_x64() -> Vec<u8> {
        let mut img = generate_esp_image();

        // Constants from generate_esp_image
        const SECTOR_SIZE: usize = 512;
        const CLUSTER_SIZE: usize = SECTOR_SIZE * 4;
        const RESERVED_SECTORS: usize = 1;
        const NUM_FATS: usize = 2;
        const FAT_SECTORS: usize = 2;
        const ROOT_ENTRIES: usize = 512;
        const ROOT_DIR_SECTORS: usize = (ROOT_ENTRIES * 32) / SECTOR_SIZE;

        let fat_start = RESERVED_SECTORS * SECTOR_SIZE;
        let root_dir_start = fat_start + NUM_FATS * FAT_SECTORS * SECTOR_SIZE;
        let data_start = root_dir_start + ROOT_DIR_SECTORS * SECTOR_SIZE;
        let cluster3_offset = data_start + CLUSTER_SIZE;

        // Replace BOOTAA64EFI with BOOTX64 EFI in BOOT directory (8.3 format)
        img[cluster3_offset + 64..cluster3_offset + 75].copy_from_slice(b"BOOTX64 EFI");
        img[cluster3_offset + 96..cluster3_offset + 107].copy_from_slice(b"GRUBX64 EFI");

        // Update file data (content doesn't matter for this test, just needs to be readable)
        let cluster4_offset = data_start + CLUSTER_SIZE * 2;
        let cluster5_offset = data_start + CLUSTER_SIZE * 3;
        img[cluster4_offset..cluster4_offset + 12].copy_from_slice(b"bootx64.efi\0");
        img[cluster5_offset..cluster5_offset + 12].copy_from_slice(b"grubx64.efi\0");

        img
    }

    #[test]
    fn test_read_fallback_and_grub_x64() {
        let esp = mount_esp(generate_esp_image_x64()).unwrap();
        assert!(esp.fallback().unwrap().starts_with(b"bootx64.efi"));
        assert!(esp.grub().unwrap().starts_with(b"grubx64.efi"));
    }

    /// The `/EFI/BOOT` directory's offset in the test ESP image.
    fn boot_dir_offset() -> usize {
        const SECTOR_SIZE: usize = 512;
        const CLUSTER_SIZE: usize = SECTOR_SIZE * 4;
        const RESERVED_SECTORS: usize = 1;
        const NUM_FATS: usize = 2;
        const FAT_SECTORS: usize = 2;
        const ROOT_ENTRIES: usize = 512;
        const ROOT_DIR_SECTORS: usize = (ROOT_ENTRIES * 32) / SECTOR_SIZE;
        let root_dir_start = RESERVED_SECTORS * SECTOR_SIZE + NUM_FATS * FAT_SECTORS * SECTOR_SIZE;
        root_dir_start + ROOT_DIR_SECTORS * SECTOR_SIZE + CLUSTER_SIZE
    }

    #[test]
    fn test_fallback_not_found() {
        // Zero out the file entries in /EFI/BOOT, keeping . and ..
        let mut image = generate_esp_image();
        let boot_dir = boot_dir_offset();
        image[boot_dir + 64..boot_dir + 128].fill(0);
        let err = mount_esp(image).unwrap().fallback().unwrap_err();
        assert!(err.to_string().contains("none of"));
        // A missing GRUB fails the presence check too.
        let mut image = generate_esp_image();
        image[boot_dir + 96..boot_dir + 128].fill(0);
        let err = mount_esp(image).unwrap().ensure_grub().unwrap_err();
        assert!(err.to_string().contains("none of"));
    }

    #[test]
    fn test_fallback_ambiguous() {
        // Rename GRUBAA64EFI to BOOTX64 EFI, so both fallback names are present.
        let mut image = generate_esp_image();
        let boot_dir = boot_dir_offset();
        image[boot_dir + 96..boot_dir + 107].copy_from_slice(b"BOOTX64 EFI");
        let err = mount_esp(image).unwrap().fallback().unwrap_err();
        assert!(err.to_string().contains("ambiguous"));
    }

    #[test]
    fn test_missing_efi_boot_directory() {
        const ROOT_DIR_START: usize = 512 + 2 * 2 * 512;
        // No EFI directory.
        let mut image = generate_esp_image();
        image[ROOT_DIR_START..ROOT_DIR_START + 32].fill(0);
        let err = mount_esp(image).unwrap().fallback().unwrap_err();
        assert!(err.to_string().contains("/EFI/BOOT"));
        // EFI, but no BOOT directory in it.
        let mut image = generate_esp_image();
        let efi_dir = boot_dir_offset() - 4 * 512;
        image[efi_dir + 64..efi_dir + 96].fill(0);
        let err = mount_esp(image).unwrap().fallback().unwrap_err();
        assert!(err.to_string().contains("/EFI/BOOT"));
    }
}
