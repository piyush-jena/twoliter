//! The partition table as firmware measures it into PCR 5.

use bit_field::BitField;
use gptman::{GPTPartitionEntry, GPT};
use hex_literal::hex;
use nutype::nutype;
use snafu::{ensure_whatever, ResultExt};
use std::io::{Read, Seek, SeekFrom};
use std::ops::RangeInclusive;
use uuid::Uuid;

use crate::error::Result;

/// The most partition entries a Bottlerocket GPT header claims.
const MAX_PARTITION_ENTRIES: u32 = 128;

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
pub const BOTTLEROCKET_BOOT: [u8; 16] = uuid_to_guid(hex!("6b636168 7420 6568 2070 6c616e657421"));

/// Bottlerocket private partition type GUID.
pub const BOTTLEROCKET_PRIVATE: [u8; 16] =
    uuid_to_guid(hex!("440408bb eb0b 4328 a6e5 a29038fad706"));

/// EFI System Partition type GUID.
pub const EFI_SYSTEM_PARTITION: [u8; 16] =
    uuid_to_guid(hex!("c12a7328 f81f 11d2 ba4b 00a0c93ec93b"));

/// The primary GPT: header and every entry, since PCR 5 measures all of them.
#[derive(Debug)]
pub struct PartitionTable(GPT);

impl PartitionTable {
    pub fn new(gpt: GPT) -> Result<Self> {
        // gptman falls back to the backup GPT, but firmware measures the primary.
        ensure_whatever!(gpt.is_primary(), "primary GPT is invalid");
        Ok(Self(gpt))
    }

    pub fn read_from<R: Read + Seek>(disk: &mut R) -> Result<Self> {
        // gptman reserves memory for every entry the header claims before reading any of them.
        let mut header = [0; 92];
        disk.seek(SeekFrom::Start(512))
            .and_then(|_| disk.read_exact(&mut header))
            .whatever_context("failed to read GPT header")?;
        let entries = u32::from_le_bytes([header[80], header[81], header[82], header[83]]);
        ensure_whatever!(
            !header.starts_with(b"EFI PART") || entries <= MAX_PARTITION_ENTRIES,
            "GPT header claims {entries} partition entries"
        );
        Self::new(GPT::find_from(disk).whatever_context("failed to parse GPT")?)
    }

    /// Number of the `n`th partition (0-based) with the given type GUID.
    pub fn nth_of_type(&self, type_guid: &[u8; 16], n: usize) -> Option<u32> {
        self.0
            .iter()
            .filter(|(_, p)| &p.partition_type_guid == type_guid)
            .nth(n)
            .map(|(number, _)| number)
    }

    pub fn byte_range(&self, part: u32) -> Result<RangeInclusive<u64>> {
        self.0
            .get_partition_byte_range(part)
            .with_whatever_context(|_| format!("invalid range for partition {part}"))
    }

    pub fn unique_guid(&self, part: u32) -> Uuid {
        Uuid::from_bytes_le(self.0[part].unique_partition_guid)
    }

    pub fn attributes(&self, part: u32) -> GptPrio {
        GptPrio::from(self.0[part].attribute_bits)
    }

    /// EFI_GPT_DATA as EDK2 measures it: the header, the used-entry count and the used entries,
    /// with `attributes` applied and the CRCs recomputed.
    pub fn efi_gpt_data(&self, attributes: &[(u32, GptPrio)]) -> Result<Vec<u8>> {
        let mut entries: Vec<GPTPartitionEntry> = self.0.iter().map(|(_, p)| p.clone()).collect();
        for &(part, attrs) in attributes {
            entries[part as usize - 1].attribute_bits = attrs.into();
        }
        let mut header = self.0.header.clone();
        header.update_partition_entry_array_crc32(&entries);
        header.update_crc32_checksum();

        let used: Vec<_> = entries.iter().filter(|p| p.is_used()).collect();
        let mut result = wincode::serialize(&header).whatever_context("failed to serialize GPT")?;
        result.extend_from_slice(&(used.len() as u64).to_le_bytes());
        for entry in used {
            result.extend(wincode::serialize(entry).whatever_context("failed to serialize GPT")?);
        }

        Ok(result)
    }
}

/// 4-bit value (0-15) for GPT priority/tries fields.
#[nutype(validate(less_or_equal = 15), derive(Debug, Clone, Copy))]
pub struct Nibble(u8);

/// GPT priority bits wrapper for boot partition attributes.
#[derive(Debug, Clone, Copy)]
pub struct GptPrio(u64);

impl GptPrio {
    /// Get priority value (bits 48-51).
    pub fn priority(self) -> Nibble {
        Nibble::try_new(self.0.get_bits(48..52) as u8).expect("4-bit field")
    }

    /// Set priority value (bits 48-51).
    pub fn set_priority(&mut self, priority: Nibble) {
        self.0.set_bits(48..52, priority.into_inner() as u64);
    }

    /// Get tries remaining (bits 52-55).
    pub fn tries_left(self) -> Nibble {
        Nibble::try_new(self.0.get_bits(52..56) as u8).expect("4-bit field")
    }

    /// Set tries remaining (bits 52-55).
    pub fn set_tries_left(&mut self, tries_left: Nibble) {
        self.0.set_bits(52..56, tries_left.into_inner() as u64);
    }

    /// Get successful boot flag (bit 56).
    pub fn successful(self) -> bool {
        self.0.get_bit(56)
    }

    /// Set successful boot flag (bit 56).
    pub fn set_successful(&mut self, successful: bool) {
        self.0.set_bit(56, successful);
    }

    /// Get boot-has-succeeded flag (bit 57).
    pub fn has_boot_succeeded(self) -> bool {
        self.0.get_bit(57)
    }

    /// Set boot-has-succeeded flag (bit 57).
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::predict::test_support::mock_gpt;
    use std::io::Cursor;

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
        prio.set_priority(Nibble::try_new(2).unwrap());
        prio.set_tries_left(Nibble::try_new(3).unwrap());
        prio.set_successful(true);

        assert_eq!(prio.priority().into_inner(), 2);
        assert_eq!(prio.tries_left().into_inner(), 3);
        assert!(prio.successful());
        assert!(!prio.has_boot_succeeded());

        prio.set_boot_succeeded(true);
        assert!(prio.has_boot_succeeded());
    }

    #[test]
    fn efi_gpt_data_matches_the_written_table() {
        let mut gpt = mock_gpt(2);
        let mut attrs = GptPrio::from(gpt[2].attribute_bits);
        attrs.set_boot_succeeded(true);
        let data = PartitionTable::new(gpt.clone())
            .unwrap()
            .efi_gpt_data(&[(2, attrs)])
            .unwrap();

        // Firmware reads the table as written, CRCs included.
        gpt[2].attribute_bits = attrs.into();
        let mut disk = Cursor::new(vec![0u8; 128 * 512]);
        gpt.write_into(&mut disk).unwrap();
        let disk = disk.into_inner();
        let mut expected = disk[512..512 + 92].to_vec();
        expected.extend_from_slice(&2u64.to_le_bytes());
        expected.extend_from_slice(&disk[1024..1024 + 2 * 128]);
        assert_eq!(data, expected);
    }

    #[test]
    fn a_header_claiming_too_many_entries_is_rejected_before_they_are_read() {
        let mut disk = Cursor::new(vec![0u8; 128 * 512]);
        mock_gpt(2).write_into(&mut disk).unwrap();
        let mut disk = disk.into_inner();
        disk[512 + 80..512 + 84].copy_from_slice(&0x0100_0000u32.to_le_bytes());
        let err = PartitionTable::read_from(&mut Cursor::new(disk)).unwrap_err();
        assert!(
            err.to_string()
                .contains("claims 16777216 partition entries"),
            "{err}"
        );
    }

    #[test]
    fn partitions_are_found_by_type() {
        let table = PartitionTable::new(mock_gpt(3)).unwrap();
        assert_eq!(table.nth_of_type(&[2; 16], 0), Some(2));
        assert_eq!(table.nth_of_type(&[2; 16], 1), None);
    }

    #[test]
    fn the_unique_guid_is_read_in_gpt_byte_order() {
        let mut gpt = mock_gpt(1);
        gpt[1].unique_partition_guid = hex!("33221100 5544 7766 8899 aabbccddeeff");
        let table = PartitionTable::new(gpt).unwrap();
        assert_eq!(
            table.unique_guid(1).to_string(),
            "00112233-4455-6677-8899-aabbccddeeff"
        );
    }
}
