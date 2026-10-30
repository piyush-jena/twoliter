//! The primary GPT, read and validated once.

use bit_field::BitField;
use gptman::{GPTHeader, GPTPartitionEntry};
use hex_literal::hex;
use snafu::{ensure_whatever, OptionExt, ResultExt};
use std::io::Cursor;
use std::ops::Range;
use uuid::Uuid;

use crate::disk::DiskImage;
use crate::error::Result;

/// Logical block size; Bottlerocket images use 512-byte sectors.
const SECTOR_SIZE: u64 = 512;
/// LBA 0 through 34: gptman reads 152 bytes for each 128-byte entry, so the last runs past LBA 33.
const TABLE_SECTORS: usize = 35;
/// The primary header and entry array, LBA 1-33, as firmware measures them.
const PRIMARY: Range<usize> = 512..34 * 512;

// Offsets within the primary GPT, which starts at LBA 1.
const HEADER_LEN: usize = 92;
const HEADER_CRC: Range<usize> = 16..20;
const ENTRIES_CRC: Range<usize> = 88..92;
const ENTRIES: Range<usize> = 512..512 + ENTRY_COUNT * ENTRY_LEN;
const ENTRY_LEN: usize = 128;
const ENTRY_COUNT: usize = 128;
const ENTRY_ATTRIBUTES: usize = 48;

/// Convert UUID byte order to GPT GUID byte order.
///
/// GUIDs store the first three fields as little-endian, while UUIDs are big-endian.
pub const fn uuid_to_guid(uuid: [u8; 16]) -> [u8; 16] {
    [
        uuid[3], uuid[2], uuid[1], uuid[0], uuid[5], uuid[4], uuid[7], uuid[6], uuid[8], uuid[9],
        uuid[10], uuid[11], uuid[12], uuid[13], uuid[14], uuid[15],
    ]
}

/// Bottlerocket boot partition type GUID.
const BOTTLEROCKET_BOOT: [u8; 16] = uuid_to_guid(hex!("6b636168 7420 6568 2070 6c616e657421"));

/// Bottlerocket private partition type GUID.
const BOTTLEROCKET_PRIVATE: [u8; 16] = uuid_to_guid(hex!("440408bb eb0b 4328 a6e5 a29038fad706"));

/// EFI System Partition type GUID.
const EFI_SYSTEM_PARTITION: [u8; 16] = uuid_to_guid(hex!("c12a7328 f81f 11d2 ba4b 00a0c93ec93b"));

/// GPT priority bits wrapper for boot partition attributes.
#[derive(Debug, Clone, Copy)]
pub struct GptPrio(u64);

impl GptPrio {
    /// Set the 4-bit priority (bits 48-51).
    pub fn set_priority(&mut self, priority: u8) {
        self.0.set_bits(48..52, u64::from(priority));
    }

    /// Set the 4-bit tries remaining (bits 52-55).
    pub fn set_tries_left(&mut self, tries_left: u8) {
        self.0.set_bits(52..56, u64::from(tries_left));
    }

    /// Set the successful boot flag (bit 56).
    pub fn set_successful(&mut self, successful: bool) {
        self.0.set_bit(56, successful);
    }

    /// Set the boot-has-succeeded flag (bit 57).
    pub fn set_boot_succeeded(&mut self, succeeded: bool) {
        self.0.set_bit(57, succeeded);
    }
}

impl From<u64> for GptPrio {
    /// Create a `GptPrio` from raw partition attribute flags.
    fn from(flags: u64) -> Self {
        Self(flags)
    }
}

impl From<GptPrio> for u64 {
    /// Extract raw partition attribute flags from a `GptPrio`.
    fn from(flags: GptPrio) -> Self {
        flags.0
    }
}

/// One GPT partition: its number, LBA range, and unique GUID.
#[derive(Debug, Clone)]
pub struct PartitionInfo {
    /// 1-based partition number.
    pub number: u32,
    /// Starting LBA (512-byte sectors).
    pub start_lba: u64,
    /// Ending LBA (inclusive).
    pub end_lba: u64,
    /// The partition's unique GUID, in on-disk byte order.
    pub unique_guid: [u8; 16],
}

impl PartitionInfo {
    /// Partition offset in bytes.
    pub fn offset_bytes(&self) -> u64 {
        self.start_lba * SECTOR_SIZE
    }

    /// Partition size in bytes.
    pub fn size_bytes(&self) -> u64 {
        (self.end_lba - self.start_lba + 1) * SECTOR_SIZE
    }

    /// The PARTUUID, formatted as GRUB prints it.
    pub fn partuuid(&self) -> String {
        Uuid::from_bytes_le(self.unique_guid).to_string()
    }
}

/// The partitions a GRUB image boots from; single-bank images have no BOOT-B.
#[derive(Debug, Clone)]
pub struct PartitionLayout {
    /// Boot partition A, which holds vmlinuz and grub.cfg.
    pub boot_a: PartitionInfo,
    /// Boot partition B, on images that update in place.
    pub boot_b: Option<PartitionInfo>,
    /// The private partition, which holds bootconfig.data.
    pub private: PartitionInfo,
}

/// The primary GPT: the bytes firmware measures, and the partitions they describe.
pub struct Gpt {
    primary: Vec<u8>,
    entries: Vec<GPTPartitionEntry>,
    first_usable_lba: u64,
    last_usable_lba: u64,
    disk_sectors: u64,
}

impl Gpt {
    /// Read the primary GPT, validating its header, entry array, and geometry.
    pub fn read(disk: &dyn DiskImage) -> Result<Self> {
        let mut table = vec![0; TABLE_SECTORS * PRIMARY.start];
        disk.read_exact_at(&mut table, 0)
            .whatever_context("failed to read GPT")?;
        let mut reader = Cursor::new(table.as_slice());
        reader.set_position(SECTOR_SIZE);
        let header = GPTHeader::read_from(&mut reader).whatever_context("failed to parse GPT")?;
        // gptman trusts these fields when reading the entries, so check them first.
        ensure_whatever!(
            header.primary_lba == 1
                && header.partition_entry_lba == 2
                && usize::try_from(header.number_of_partition_entries) == Ok(ENTRY_COUNT)
                && usize::try_from(header.size_of_partition_entry) == Ok(ENTRY_LEN),
            "unsupported or damaged primary GPT layout"
        );
        let entries = header
            .read_partitions(&mut reader, SECTOR_SIZE)
            .whatever_context("failed to parse GPT")?;
        Ok(Self {
            primary: table[PRIMARY].to_vec(),
            entries,
            first_usable_lba: header.first_usable_lba,
            last_usable_lba: header.last_usable_lba,
            disk_sectors: disk.size() / SECTOR_SIZE,
        })
    }

    /// The image's one EFI System Partition.
    pub fn esp(&self) -> Result<PartitionInfo> {
        let mut esps = self.of_type(EFI_SYSTEM_PARTITION);
        let esp = esps.next().whatever_context("EFI-A partition not found")?;
        ensure_whatever!(esps.next().is_none(), "ambiguous layout: multiple ESPs");
        self.bounded(esp, "ESP")
    }

    /// GRUB's partitions: the first two BOOT partitions and the first PRIVATE one.
    pub fn grub_layout(&self) -> Result<PartitionLayout> {
        let mut boot = self.of_type(BOTTLEROCKET_BOOT);
        let boot_a = boot.next().whatever_context("BOOT-A partition not found")?;
        let boot_b = boot.next();
        let private = self
            .of_type(BOTTLEROCKET_PRIVATE)
            .next()
            .whatever_context("PRIVATE partition not found")?;
        Ok(PartitionLayout {
            boot_a: self.bounded(boot_a, "BOOT-A")?,
            boot_b: boot_b
                .map(|boot_b| self.bounded(boot_b, "BOOT-B"))
                .transpose()?,
            private: self.bounded(private, "PRIVATE")?,
        })
    }

    /// A partition's attribute bits.
    pub fn attributes(&self, partition: &PartitionInfo) -> u64 {
        self.entries[entry_index(partition.number)].attribute_bits
    }

    /// EFI_GPT_DATA as firmware measures it, after setting the given partitions' attribute bits.
    pub fn efi_gpt_data(&self, attributes: &[(u32, u64)]) -> Vec<u8> {
        let mut gpt = self.primary.clone();
        for &(number, bits) in attributes {
            let offset = ENTRIES.start + entry_index(number) * ENTRY_LEN + ENTRY_ATTRIBUTES;
            gpt[offset..offset + 8].copy_from_slice(&bits.to_le_bytes());
        }
        // The entry array's CRC is part of the header the header CRC covers.
        let entries_crc = crc32fast::hash(&gpt[ENTRIES]);
        gpt[ENTRIES_CRC].copy_from_slice(&entries_crc.to_le_bytes());
        gpt[HEADER_CRC].fill(0);
        let header_crc = crc32fast::hash(&gpt[..HEADER_LEN]);
        gpt[HEADER_CRC].copy_from_slice(&header_crc.to_le_bytes());

        let used: Vec<&[u8; ENTRY_LEN]> = gpt[ENTRIES]
            .as_chunks::<ENTRY_LEN>()
            .0
            .iter()
            .filter(|entry| entry[..16].iter().any(|&byte| byte != 0))
            .collect();
        let count = u64::try_from(used.len()).expect("at most 128 entries");
        let mut data = Vec::with_capacity(HEADER_LEN + 8 + used.len() * ENTRY_LEN);
        data.extend_from_slice(&gpt[..HEADER_LEN]);
        data.extend_from_slice(&count.to_le_bytes());
        for entry in used {
            data.extend_from_slice(entry);
        }
        data
    }

    /// The numbered entries of one partition type, in table order.
    fn of_type(&self, type_guid: [u8; 16]) -> impl Iterator<Item = (u32, &GPTPartitionEntry)> {
        (1..)
            .zip(&self.entries)
            .filter(move |(_, entry)| entry.partition_type_guid == type_guid)
    }

    /// The partition an entry describes, which must lie within the usable LBAs and the disk.
    fn bounded(
        &self,
        (number, entry): (u32, &GPTPartitionEntry),
        name: &str,
    ) -> Result<PartitionInfo> {
        ensure_whatever!(
            entry.starting_lba >= self.first_usable_lba
                && entry.starting_lba <= entry.ending_lba
                && entry.ending_lba <= self.last_usable_lba
                && entry.ending_lba < self.disk_sectors,
            "{name} partition is outside disk bounds"
        );
        Ok(PartitionInfo {
            number,
            start_lba: entry.starting_lba,
            end_lba: entry.ending_lba,
            unique_guid: entry.unique_partition_guid,
        })
    }
}

/// The index of 1-based partition `number` in the entry array.
fn entry_index(number: u32) -> usize {
    usize::try_from(number - 1).expect("partition numbers come from the 128-entry table")
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    impl GptPrio {
        fn priority(self) -> u64 {
            self.0.get_bits(48..52)
        }

        fn tries_left(self) -> u64 {
            self.0.get_bits(52..56)
        }

        fn successful(self) -> bool {
            self.0.get_bit(56)
        }

        fn has_boot_succeeded(self) -> bool {
            self.0.get_bit(57)
        }
    }

    #[test]
    fn test_uuid_to_guid() {
        // Test from signpost: "Hah!IdontNeedEFI" BIOS boot partition GUID
        assert_eq!(
            uuid_to_guid(hex!("21686148 6449 6e6f 744e 656564454649")),
            *b"Hah!IdontNeedEFI"
        );
    }

    #[test]
    fn test_gpt_prio() {
        let mut prio = GptPrio::from(0u64);
        prio.set_priority(2);
        prio.set_tries_left(3);
        prio.set_successful(true);

        assert_eq!(prio.priority(), 2);
        assert_eq!(prio.tries_left(), 3);
        assert!(prio.successful());
        assert!(!prio.has_boot_succeeded());

        prio.set_boot_succeeded(true);
        assert!(prio.has_boot_succeeded());
    }

    #[test]
    fn gpt_prio_bits_sit_where_gptprio_reads_them() {
        let mut prio = GptPrio::from(1 << 2);
        prio.set_priority(2);
        prio.set_tries_left(3);
        prio.set_successful(true);
        assert_eq!(u64::from(prio), 0x0132_0000_0000_0004);
        prio.set_boot_succeeded(true);
        assert_eq!(u64::from(prio), 0x0332_0000_0000_0004);
    }

    #[test]
    #[should_panic]
    fn gpt_prio_rejects_a_priority_wider_than_four_bits() {
        GptPrio::from(0).set_priority(16);
    }

    #[test]
    fn test_partition_info_bytes() {
        let info = PartitionInfo {
            number: 1,
            start_lba: 2048,
            end_lba: 4095,
            unique_guid: [0; 16],
        };
        assert_eq!(info.offset_bytes(), 2048 * 512);
        assert_eq!(info.size_bytes(), (4095 - 2048 + 1) * 512);
    }

    pub(crate) fn mock_disk_with_partitions() -> Vec<u8> {
        // Disk: MBR + GPT header + partition entries + backup
        let mut disk = vec![0u8; 512 * 68]; // MBR + header + 33 entry sectors + backup header

        // Protective MBR at LBA 0
        disk[446] = 0x00; // Boot indicator
        disk[450] = 0xEE; // Partition type (GPT protective)
        disk[510] = 0x55; // MBR signature
        disk[511] = 0xAA;

        // GPT header at LBA 1 (offset 512)
        let hdr = 512;
        disk[hdr..hdr + 8].copy_from_slice(b"EFI PART");
        disk[hdr + 8..hdr + 12].copy_from_slice(&0x00010000u32.to_le_bytes()); // Revision
        disk[hdr + 12..hdr + 16].copy_from_slice(&92u32.to_le_bytes()); // Header size

        // CRC32 at offset 16 - will set after

        disk[hdr + 20..hdr + 24].copy_from_slice(&0u32.to_le_bytes()); // Reserved
        disk[hdr + 24..hdr + 32].copy_from_slice(&1u64.to_le_bytes()); // Current LBA
        disk[hdr + 32..hdr + 40].copy_from_slice(&67u64.to_le_bytes()); // Backup LBA
        disk[hdr + 40..hdr + 48].copy_from_slice(&34u64.to_le_bytes()); // First usable LBA
        disk[hdr + 48..hdr + 56].copy_from_slice(&66u64.to_le_bytes()); // Last usable LBA

        disk[hdr + 56..hdr + 72].copy_from_slice(&[1u8; 16]); // Disk GUID at offset 56 (16 bytes)
        disk[hdr + 72..hdr + 80].copy_from_slice(&2u64.to_le_bytes()); // Partition entry LBA
        disk[hdr + 80..hdr + 84].copy_from_slice(&128u32.to_le_bytes()); // Number of entries
        disk[hdr + 84..hdr + 88].copy_from_slice(&128u32.to_le_bytes()); // Entry size

        // Partition array CRC at offset 88 - will set after

        // Partition entries at LBA 2 (offset 1024)
        let entries = 1024;

        // Partition 1: EFI System Partition
        disk[entries..entries + 16].copy_from_slice(&EFI_SYSTEM_PARTITION);
        disk[entries + 16..entries + 32].copy_from_slice(&[0x11; 16]); // Unique GUID
        disk[entries + 32..entries + 40].copy_from_slice(&34u64.to_le_bytes()); // Start LBA
        disk[entries + 40..entries + 48].copy_from_slice(&40u64.to_le_bytes()); // End LBA

        // Partition 2: BOOT-A
        let p2 = entries + 128;
        disk[p2..p2 + 16].copy_from_slice(&BOTTLEROCKET_BOOT);
        disk[p2 + 16..p2 + 32].copy_from_slice(&[0x22; 16]);
        disk[p2 + 32..p2 + 40].copy_from_slice(&41u64.to_le_bytes());
        disk[p2 + 40..p2 + 48].copy_from_slice(&50u64.to_le_bytes());

        // Partition 3: BOOT-B
        let p3 = entries + 256;
        disk[p3..p3 + 16].copy_from_slice(&BOTTLEROCKET_BOOT);
        disk[p3 + 16..p3 + 32].copy_from_slice(&[0x33; 16]);
        disk[p3 + 32..p3 + 40].copy_from_slice(&51u64.to_le_bytes());
        disk[p3 + 40..p3 + 48].copy_from_slice(&60u64.to_le_bytes());

        // Partition 4: PRIVATE
        let p4 = entries + 384;
        disk[p4..p4 + 16].copy_from_slice(&BOTTLEROCKET_PRIVATE);
        disk[p4 + 16..p4 + 32].copy_from_slice(&[0x44; 16]);
        disk[p4 + 32..p4 + 40].copy_from_slice(&61u64.to_le_bytes());
        disk[p4 + 40..p4 + 48].copy_from_slice(&66u64.to_le_bytes());

        // Calculate partition array CRC
        let part_crc = crc32fast::hash(&disk[entries..entries + 128 * 128]);
        disk[hdr + 88..hdr + 92].copy_from_slice(&part_crc.to_le_bytes());

        // Calculate header CRC (with CRC field zeroed)
        let header_crc = crc32fast::hash(&disk[hdr..hdr + 92]);
        disk[hdr + 16..hdr + 20].copy_from_slice(&header_crc.to_le_bytes());

        disk
    }

    /// Recalculate GPT CRCs for a full disk image (GPT at offset 512).
    pub(crate) fn recalc_disk_crcs(disk: &mut [u8]) {
        let part_crc = crc32fast::hash(&disk[1024..1024 + 128 * 128]);
        disk[512 + 88..512 + 92].copy_from_slice(&part_crc.to_le_bytes());
        disk[512 + 16..512 + 20].copy_from_slice(&[0u8; 4]);
        let header_crc = crc32fast::hash(&disk[512..512 + 92]);
        disk[512 + 16..512 + 20].copy_from_slice(&header_crc.to_le_bytes());
    }

    fn gpt(disk: Vec<u8>) -> Result<Gpt> {
        Gpt::read(&disk)
    }

    #[test]
    fn the_measured_bytes_are_lba_1_to_33() {
        let disk = mock_disk_with_partitions();
        let gpt = gpt(disk.clone()).unwrap();
        assert_eq!(gpt.primary, disk[512..34 * 512]);
        assert_eq!(&gpt.primary[..8], b"EFI PART");
    }

    #[test]
    fn test_grub_layout() {
        let layout = gpt(mock_disk_with_partitions())
            .unwrap()
            .grub_layout()
            .unwrap();
        assert_eq!(layout.boot_a.number, 2);
        assert_eq!(layout.boot_a.start_lba, 41);
        assert_eq!(layout.boot_b.as_ref().unwrap().number, 3);
        assert_eq!(layout.private.number, 4);
    }

    #[test]
    fn boot_partuuid_is_formatted_like_grub() {
        let mut disk = mock_disk_with_partitions();
        // BOOT-A's unique GUID, asymmetric so that a byte-order mistake shows.
        let guid: Vec<u8> = (0..16).collect();
        disk[1024 + 128 + 16..1024 + 128 + 32].copy_from_slice(&guid);
        recalc_disk_crcs(&mut disk);
        let layout = gpt(disk).unwrap().grub_layout().unwrap();
        assert_eq!(
            layout.boot_a.partuuid(),
            "03020100-0504-0706-0809-0a0b0c0d0e0f"
        );
    }

    /// Damage the mock disk, then expect reading its ESP to fail with `message`.
    fn assert_esp_rejected(damage: impl FnOnce(&mut Vec<u8>), message: &str) {
        let mut disk = mock_disk_with_partitions();
        damage(&mut disk);
        let err = gpt(disk).and_then(|gpt| gpt.esp()).unwrap_err();
        assert!(err.to_string().contains(message), "{err}");
    }

    #[test]
    fn find_esp_rejects_missing_and_ambiguous_esps() {
        assert_esp_rejected(
            |disk| {
                disk[1024..1024 + 16].fill(0);
                recalc_disk_crcs(disk);
            },
            "EFI-A partition not found",
        );
        assert_esp_rejected(
            |disk| {
                disk[1024 + 256..1024 + 256 + 16].copy_from_slice(&EFI_SYSTEM_PARTITION);
                recalc_disk_crcs(disk);
            },
            "multiple ESPs",
        );
    }

    #[test]
    fn find_esp_rejects_an_esp_outside_the_disk() {
        assert_esp_rejected(
            |disk| {
                disk[1024 + 40..1024 + 48].copy_from_slice(&100u64.to_le_bytes());
                recalc_disk_crcs(disk);
            },
            "outside disk bounds",
        );
    }

    #[test]
    fn find_esp_rejects_unsupported_geometry() {
        assert_esp_rejected(
            |disk| {
                // 64 entries, with CRCs that are valid for that count.
                disk[512 + 80..512 + 84].copy_from_slice(&64u32.to_le_bytes());
                let part_crc = crc32fast::hash(&disk[1024..1024 + 64 * 128]);
                disk[512 + 88..512 + 92].copy_from_slice(&part_crc.to_le_bytes());
                disk[512 + 16..512 + 20].fill(0);
                let header_crc = crc32fast::hash(&disk[512..512 + 92]);
                disk[512 + 16..512 + 20].copy_from_slice(&header_crc.to_le_bytes());
            },
            "unsupported or damaged primary GPT layout",
        );
    }

    #[test]
    fn a_boot_partition_outside_the_disk_is_rejected() {
        let mut disk = mock_disk_with_partitions();
        // BOOT-A ends before it starts.
        disk[1024 + 128 + 40..1024 + 128 + 48].copy_from_slice(&40u64.to_le_bytes());
        recalc_disk_crcs(&mut disk);
        let err = gpt(disk).unwrap().grub_layout().unwrap_err();
        assert!(
            err.to_string()
                .contains("BOOT-A partition is outside disk bounds"),
            "{err}"
        );
    }

    #[test]
    fn test_grub_layout_missing_efi_still_finds_grub_partitions() {
        let mut disk = mock_disk_with_partitions();
        disk[1024..1024 + 16].copy_from_slice(&[0u8; 16]);
        recalc_disk_crcs(&mut disk);
        let gpt = gpt(disk).unwrap();
        assert!(gpt.esp().unwrap_err().to_string().contains("EFI-A"));
        assert!(gpt.grub_layout().is_ok());
    }

    #[test]
    fn test_grub_layout_missing_boot() {
        let mut disk = mock_disk_with_partitions();
        disk[1024 + 128..1024 + 128 + 16].copy_from_slice(&[0u8; 16]);
        disk[1024 + 256..1024 + 256 + 16].copy_from_slice(&[0u8; 16]);
        recalc_disk_crcs(&mut disk);
        let err = gpt(disk).unwrap().grub_layout().unwrap_err();
        assert!(err.to_string().contains("BOOT-A"));
    }

    #[test]
    fn test_grub_layout_missing_private() {
        let mut disk = mock_disk_with_partitions();
        disk[1024 + 384..1024 + 384 + 16].copy_from_slice(&[0u8; 16]);
        recalc_disk_crcs(&mut disk);
        let err = gpt(disk).unwrap().grub_layout().unwrap_err();
        assert!(err.to_string().contains("PRIVATE"));
    }

    #[test]
    fn test_invalid_gpt() {
        let err = gpt(vec![0u8; 512 * 68]).err().unwrap();
        assert!(err.to_string().contains("GPT"));
    }

    #[test]
    fn efi_gpt_data_holds_the_header_count_and_used_entries() {
        let disk = mock_disk_with_partitions();
        let gpt = gpt(disk.clone()).unwrap();
        let data = gpt.efi_gpt_data(&[]);
        // 92 (header) + 8 (count) + 4*128 (4 used entries)
        assert_eq!(data.len(), 92 + 8 + 4 * 128);
        assert_eq!(data[..92], disk[512..512 + 92]);
        assert_eq!(u64::from_le_bytes(data[92..100].try_into().unwrap()), 4);
        assert_eq!(data[100..], disk[1024..1024 + 4 * 128]);
    }

    #[test]
    fn efi_gpt_data_sets_attributes_and_recalculates_crcs() {
        let disk = mock_disk_with_partitions();
        let gpt = gpt(disk.clone()).unwrap();
        let data = gpt.efi_gpt_data(&[(2, 0x0132_0000_0000_0000)]);
        let mut expected = disk;
        expected[1024 + 128 + 48..1024 + 128 + 56]
            .copy_from_slice(&0x0132_0000_0000_0000u64.to_le_bytes());
        recalc_disk_crcs(&mut expected);
        assert_eq!(data[..92], expected[512..512 + 92]);
        assert_eq!(data[100..], expected[1024..1024 + 4 * 128]);
    }
}
