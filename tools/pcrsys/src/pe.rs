//! PE file parsing and Authenticode hash calculation.
//!
//! This matches the behavior of `pesign --hash`.

use crate::error::Result;
use crate::predict::PcrDigests;
use authenticode::{authenticode_digest, PeTrait};
use object::read::pe::PeFile64;
use object::{Object, ObjectSection};
use sha2::{Digest, Sha256, Sha384};
use snafu::{ensure_whatever, whatever, OptionExt, ResultExt};
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::ops::Range;

/// Sections that mark a UKI. `.sbat` is deliberately not one, since shim also has it.
const UKI_MARKERS: [&str; 3] = [".linux", ".osrel", ".cmdline"];

/// The sections systemd-stub 257 measures into PCR 11 when present, in its order
/// (src/fundamental/uki.h); it loads `.pcrsig` without measuring it.
const STUB_SECTIONS: [&str; 14] = [
    ".linux", ".osrel", ".cmdline", ".initrd", ".ucode", ".splash", ".dtb", ".uname", ".sbat",
    ".pcrsig", ".pcrpkey", ".profile", ".dtbauto", ".hwids",
];

/// The stub sections that Bottlerocket's UKIs carry, and pcrsys predicts, in measurement order.
pub const MEASURED_UKI_SECTIONS: [&str; 5] = [".linux", ".osrel", ".cmdline", ".uname", ".sbat"];

/// Calculate Authenticode SHA-256 and SHA-384 hashes of a PE32+ file.
/// Pads to 8-byte boundary to match pesign behavior.
pub fn get_authenticode_hash(pe_data: &[u8]) -> Result<PcrDigests> {
    let padding = (8 - (pe_data.len() % 8)) % 8;
    let data: Cow<[u8]> = if padding > 0 {
        let mut padded = pe_data.to_vec();
        padded.resize(pe_data.len() + padding, 0);
        padded.into()
    } else {
        pe_data.into()
    };

    let pe = parse_pe_data(&data)?;

    let mut hasher = Sha256::new();
    ensure_whatever!(
        authenticode_digest(&pe, &mut hasher).is_ok(),
        "authenticode digest failed"
    );
    let sha256 = hasher.finalize().into();
    let mut hasher = Sha384::new();
    ensure_whatever!(
        authenticode_digest(&pe, &mut hasher).is_ok(),
        "authenticode SHA-384 digest failed"
    );
    let sha384 = hasher.finalize().into();
    Ok(PcrDigests { sha256, sha384 })
}

/// The firmware fallback executable, parsed once.
pub enum Fallback {
    /// A UKI, which the firmware runs directly.
    Uki(Uki),
    /// shim, which chain-loads GRUB.
    Shim(Shim),
}

impl Fallback {
    /// Parse the fallback executable, telling a UKI from shim by systemd-stub's payload sections.
    pub fn parse(image: Vec<u8>) -> Result<Self> {
        let Sections {
            headers,
            ranges: sections,
            signed,
        } = Sections::read(&image).whatever_context("invalid firmware fallback executable")?;
        ensure_whatever!(signed, "the firmware fallback executable is not signed");
        let section = |name: &str| {
            sections
                .get(name)
                .map_or(&[][..], |range| &image[range.clone()])
        };
        if UKI_MARKERS.iter().any(|name| !section(name).is_empty()) {
            let uki = Uki { image, sections };
            uki.ensure_predictable(&headers)?;
            return Ok(Self::Uki(uki));
        }
        let required = |name: &str| {
            let data = section(name);
            ensure_whatever!(!data.is_empty(), "section '{name}' not found");
            Ok(data)
        };
        let sbat_level = sbat_level(required(".sbatlevel")?)?.to_vec();
        let vendor_cert = vendor_cert(required(".vendor_cert")?)?.to_vec();
        Ok(Self::Shim(Shim {
            image,
            sbat_level,
            vendor_cert,
        }))
    }
}

/// A UKI: its image, and where each section's initialized bytes lie in it.
pub struct Uki {
    image: Vec<u8>,
    sections: BTreeMap<String, Range<usize>>,
}

impl Uki {
    /// The whole image, as firmware measures it.
    pub fn data(&self) -> &[u8] {
        &self.image
    }

    /// Initialized bytes of the named section, empty when the UKI has none.
    pub fn section(&self, name: &str) -> &[u8] {
        self.sections
            .get(name)
            .map_or(&[][..], |range| &self.image[range.clone()])
    }

    /// The kernel command line the UKI was built with, as systemd-stub measures it.
    pub fn cmdline(&self) -> Result<&str> {
        bootconfig::decode_uki_cmdline(self.section(".cmdline"))
            .whatever_context("failed to parse UKI command line")
    }

    /// Check that the UKI measures nothing that pcrsys does not predict.
    fn ensure_predictable(&self, headers: &[SectionHeader]) -> Result<()> {
        for name in STUB_SECTIONS {
            // systemd-stub treats a section as present when its VirtualSize is not zero.
            let mut present = headers
                .iter()
                .filter(|header| header.name == name && header.virtual_size != 0);
            let Some(header) = present.next() else {
                continue;
            };
            ensure_whatever!(
                MEASURED_UKI_SECTIONS.contains(&name),
                "UKI section '{name}' changes measurements that pcrsys does not predict"
            );
            ensure_whatever!(
                present.next().is_none(),
                "UKI has more than one '{name}' section"
            );
            // systemd-stub measures VirtualSize bytes, zero-filled past the file's data.
            ensure_whatever!(
                header.virtual_size <= header.raw_size,
                "UKI section '{name}' is larger in memory than in the file"
            );
        }
        let overrides_initrd = bootconfig::uki_kernel_parameters(self.cmdline()?)
            .into_iter()
            .any(|parameter| parameter == "initrd" || parameter == "noinitrd");
        ensure_whatever!(
            !overrides_initrd,
            "UKI command line overrides the initrd, which pcrsys does not predict"
        );
        Ok(())
    }
}

/// shim: its image, and what it measures into PCRs 7 and 14.
pub struct Shim {
    image: Vec<u8>,
    sbat_level: Vec<u8>,
    vendor_cert: Vec<u8>,
}

impl Shim {
    /// The automatic SBAT level, which shim measures into PCR 7.
    pub fn sbat_level(&self) -> &[u8] {
        &self.sbat_level
    }

    /// The DER-encoded vendor certificate shim trusts.
    pub fn vendor_cert(&self) -> &[u8] {
        &self.vendor_cert
    }

    /// shim's Authenticode digests, as the firmware measures it into PCR 4.
    pub fn authenticode(&self) -> Result<PcrDigests> {
        get_authenticode_hash(&self.image)
    }
}

/// The CPU architectures Bottlerocket boots on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Arch {
    X86_64,
    Aarch64,
}

impl Arch {
    /// The architecture a PE32+ image is built for.
    pub fn of(image: &[u8]) -> Result<Self> {
        match parse_pe_data(image)?.architecture() {
            object::Architecture::X86_64 => Ok(Self::X86_64),
            object::Architecture::Aarch64 => Ok(Self::Aarch64),
            other => {
                whatever!("unsupported architecture {other:?}");
            }
        }
    }
}

/// Parse a PE32+ image held in a borrowed buffer.
fn parse_pe_data(pe_data: &[u8]) -> Result<PeFile64<'_>> {
    PeFile64::parse(pe_data).whatever_context("failed to parse PE32+ file")
}

/// Check that a PE32+ image is signed, as Secure Boot requires of everything it measures.
pub fn ensure_signed(image: &[u8], name: &str) -> Result<()> {
    let pe = parse_pe_data(image)?;
    ensure_whatever!(is_signed(&pe, image), "{name} is not signed");
    Ok(())
}

/// Whether a PE32+ image carries a certificate table within the file.
fn is_signed(pe: &PeFile64<'_>, image: &[u8]) -> bool {
    matches!(
        pe.certificate_table_range(),
        Ok(Some(range)) if !range.is_empty() && range.end <= image.len()
    )
}

/// A section's name and sizes, as its header gives them.
struct SectionHeader {
    name: String,
    virtual_size: u32,
    raw_size: u32,
}

/// A PE32+ image's sections, read and bounds-checked once, and whether it is signed.
struct Sections {
    /// Every section header, in file order.
    headers: Vec<SectionHeader>,
    /// Where each name's initialized bytes lie; the first section of a name wins, as with
    /// `section_by_name`.
    ranges: BTreeMap<String, Range<usize>>,
    signed: bool,
}

impl Sections {
    fn read(image: &[u8]) -> Result<Self> {
        let pe = parse_pe_data(image)?;
        let mut headers = Vec::new();
        let mut ranges = BTreeMap::new();
        for section in pe.sections() {
            let name = section
                .name()
                .whatever_context("failed to read PE section name")?;
            let data = section
                .data()
                .with_whatever_context(|_| format!("failed to read section '{name}'"))?;
            let header = section.pe_section();
            let (offset, _) = header.pe_file_range();
            let start = usize::try_from(offset).expect("usize holds a u32");
            ranges
                .entry(name.to_string())
                .or_insert(start..start + data.len());
            headers.push(SectionHeader {
                name: name.to_string(),
                virtual_size: header.virtual_size.get(object::LittleEndian),
                raw_size: header.size_of_raw_data.get(object::LittleEndian),
            });
        }
        Ok(Self {
            headers,
            ranges,
            signed: is_signed(&pe, image),
        })
    }
}

/// Read the u32 at `offset` of a section header.
fn u32_at(bytes: &[u8], offset: usize) -> usize {
    let field: [u8; 4] = bytes[offset..offset + 4].try_into().expect("4-byte field");
    usize::try_from(u32::from_le_bytes(field)).expect("usize holds a u32")
}

/// The DER vendor certificate in shim's `.vendor_cert` section, whose header holds the
/// authorized and deauthorized sizes, then their offsets, as u32s.
fn vendor_cert(section: &[u8]) -> Result<&[u8]> {
    ensure_whatever!(
        section.len() >= 16,
        ".vendor_cert section too small for header"
    );
    let size = u32_at(section, 0);
    let offset = u32_at(section, 8);
    ensure_whatever!(size > 0, "no vendor certificate present");
    // shim would add a vendor dbx to MokListX, which PCR 14 predicts empty.
    ensure_whatever!(
        u32_at(section, 4) == 0,
        "shim carries a vendor dbx, which pcrsys does not predict"
    );
    offset
        .checked_add(size)
        .and_then(|end| section.get(offset..end))
        .whatever_context("vendor certificate extends beyond section")
}

/// The automatic SBAT level string in shim's `.sbatlevel` section: a u32 version and two
/// u32 string offsets relative to byte 4 put it at byte 12.
fn sbat_level(section: &[u8]) -> Result<&[u8]> {
    const HEADER_SIZE: usize = 12;
    ensure_whatever!(
        section.len() > HEADER_SIZE,
        ".sbatlevel section too small for header"
    );
    ensure_whatever!(
        u32_at(section, 4) == HEADER_SIZE - 4,
        "unexpected .sbatlevel layout: the automatic string does not follow the header"
    );
    let strings = &section[HEADER_SIZE..];
    let end = strings
        .iter()
        .position(|&byte| byte == 0)
        .whatever_context("SBAT level string not null-terminated")?;
    ensure_whatever!(end > 0, "empty SBAT level string");
    Ok(&strings[..end])
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// Build a mock shim PE with .sbatlevel and .vendor_cert sections.
    ///
    /// Uses COFF string table for section names longer than 8 bytes.
    /// Section name format for long names: "/" followed by decimal offset into string table.
    pub fn build_test_shim() -> Vec<u8> {
        build_test_shim_for(0xaa64)
    }

    /// The test shim, built for the given COFF machine type.
    pub(crate) fn build_test_shim_for(machine: u16) -> Vec<u8> {
        with_cert_table(build_unsigned_test_shim(machine))
    }

    fn build_unsigned_test_shim(machine: u16) -> Vec<u8> {
        let mut pe = vec![0u8; 0xa00];

        // DOS header
        pe[0..2].copy_from_slice(b"MZ");
        pe[0x3c..0x40].copy_from_slice(&0x40u32.to_le_bytes()); // PE header offset

        // PE signature + COFF header at 0x40
        pe[0x40..0x44].copy_from_slice(b"PE\0\0");
        pe[0x44..0x46].copy_from_slice(&machine.to_le_bytes()); // Machine
        pe[0x46..0x48].copy_from_slice(&3u16.to_le_bytes()); // NumberOfSections
        pe[0x4c..0x50].copy_from_slice(&0x800u32.to_le_bytes()); // PointerToSymbolTable (string table follows)
        pe[0x50..0x54].copy_from_slice(&0u32.to_le_bytes()); // NumberOfSymbols
        pe[0x54..0x56].copy_from_slice(&0xf0u16.to_le_bytes()); // SizeOfOptionalHeader
        pe[0x56..0x58].copy_from_slice(&0x22u16.to_le_bytes()); // Characteristics

        // Optional header (PE32+) at 0x58
        pe[0x58..0x5a].copy_from_slice(&0x20bu16.to_le_bytes()); // Magic: PE32+
        pe[0x5a] = 0x01; // MajorLinkerVersion
        pe[0x78..0x7c].copy_from_slice(&0x1000u32.to_le_bytes()); // SectionAlignment
        pe[0x7c..0x80].copy_from_slice(&0x200u32.to_le_bytes()); // FileAlignment
        pe[0x80..0x84].copy_from_slice(&1u32.to_le_bytes()); // MajorOperatingSystemVersion
        pe[0x88..0x8c].copy_from_slice(&1u32.to_le_bytes()); // MajorSubsystemVersion
        pe[0x90..0x94].copy_from_slice(&0x5000u32.to_le_bytes()); // SizeOfImage
        pe[0x94..0x98].copy_from_slice(&0x200u32.to_le_bytes()); // SizeOfHeaders
        pe[0x9c..0x9e].copy_from_slice(&10u16.to_le_bytes()); // Subsystem: EFI Application
        pe[0xc4..0xc8].copy_from_slice(&16u32.to_le_bytes()); // NumberOfRvaAndSizes

        // Section headers start at 0x148 (after optional header)
        // Section 1: .text (short name, fits in 8 bytes)
        pe[0x148..0x150].copy_from_slice(b".text\0\0\0");
        pe[0x150..0x154].copy_from_slice(&0x10u32.to_le_bytes()); // VirtualSize
        pe[0x154..0x158].copy_from_slice(&0x1000u32.to_le_bytes()); // VirtualAddress
        pe[0x158..0x15c].copy_from_slice(&0x200u32.to_le_bytes()); // SizeOfRawData
        pe[0x15c..0x160].copy_from_slice(&0x200u32.to_le_bytes()); // PointerToRawData
        pe[0x16c..0x170].copy_from_slice(&0x60000020u32.to_le_bytes()); // Characteristics

        // Section 2: .sbatlevel (long name -> "/4" = offset 4 in string table)
        pe[0x170..0x178].copy_from_slice(b"/4\0\0\0\0\0\0");
        pe[0x178..0x17c].copy_from_slice(&0x4eu32.to_le_bytes()); // VirtualSize
        pe[0x17c..0x180].copy_from_slice(&0x2000u32.to_le_bytes()); // VirtualAddress
        pe[0x180..0x184].copy_from_slice(&0x200u32.to_le_bytes()); // SizeOfRawData
        pe[0x184..0x188].copy_from_slice(&0x400u32.to_le_bytes()); // PointerToRawData
        pe[0x194..0x198].copy_from_slice(&0x40000040u32.to_le_bytes()); // Characteristics

        // Section 3: .vendor_cert (long name -> "/15" = offset 15 in string table)
        pe[0x198..0x1a0].copy_from_slice(b"/15\0\0\0\0\0");
        pe[0x1a0..0x1a4].copy_from_slice(&0x30u32.to_le_bytes()); // VirtualSize
        pe[0x1a4..0x1a8].copy_from_slice(&0x3000u32.to_le_bytes()); // VirtualAddress
        pe[0x1a8..0x1ac].copy_from_slice(&0x200u32.to_le_bytes()); // SizeOfRawData
        pe[0x1ac..0x1b0].copy_from_slice(&0x600u32.to_le_bytes()); // PointerToRawData
        pe[0x1bc..0x1c0].copy_from_slice(&0x40000040u32.to_le_bytes()); // Characteristics

        // .text data at 0x200
        pe[0x200..0x210].fill(0xcc);

        // .sbatlevel data at 0x400, laid out like shim 16.1: a format version, then the offsets
        // of the automatic and latest strings relative to byte 4, then both strings.
        let automatic = b"sbat,1,2024010900\nshim,4\ngrub,3\n\0";
        let latest = b"sbat,1,2025051000\nshim,4\ngrub,5\n\0";
        let latest_offset = 8 + u32::try_from(automatic.len()).expect("fixture string fits in u32");
        pe[0x400..0x404].copy_from_slice(&0u32.to_le_bytes()); // format version
        pe[0x404..0x408].copy_from_slice(&8u32.to_le_bytes()); // automatic, from byte 4
        pe[0x408..0x40c].copy_from_slice(&latest_offset.to_le_bytes()); // latest, from byte 4
        pe[0x40c..0x40c + automatic.len()].copy_from_slice(automatic);
        let latest_start = 0x40c + automatic.len();
        pe[latest_start..latest_start + latest.len()].copy_from_slice(latest);

        // .vendor_cert data at 0x600: header + mock DER cert
        let mock_cert = [0x30, 0x82, 0x00, 0x10, 0x02, 0x01, 0x00]; // minimal ASN.1 SEQUENCE
        let cert_size = mock_cert.len() as u32;
        pe[0x600..0x604].copy_from_slice(&cert_size.to_le_bytes()); // vendor_authorized_size
        pe[0x604..0x608].copy_from_slice(&0u32.to_le_bytes()); // vendor_deauthorized_size
        pe[0x608..0x60c].copy_from_slice(&16u32.to_le_bytes()); // vendor_authorized_offset
        pe[0x60c..0x610].copy_from_slice(&0u32.to_le_bytes()); // vendor_deauthorized_offset
        pe[0x610..0x610 + mock_cert.len()].copy_from_slice(&mock_cert);

        // COFF string table at 0x800 (PointerToSymbolTable)
        // Format: 4-byte size followed by null-terminated strings
        // String table layout:
        //   offset 0-3: size (u32 LE)
        //   offset 4: ".sbatlevel\0" (11 bytes)
        //   offset 15: ".vendor_cert\0" (13 bytes)
        let strtab_size: u32 = 4 + 11 + 13; // size field + both strings
        pe[0x800..0x804].copy_from_slice(&strtab_size.to_le_bytes());
        pe[0x804..0x80f].copy_from_slice(b".sbatlevel\0");
        pe[0x80f..0x81c].copy_from_slice(b".vendor_cert\0");

        pe
    }

    /// Append a placeholder signature and point the Security directory at it, as signing does.
    pub(crate) fn with_cert_table(mut pe: Vec<u8>) -> Vec<u8> {
        const SECURITY_DIRECTORY: usize = 4;
        const WIN_CERT_REVISION_2_0: u16 = 0x0200;
        const WIN_CERT_TYPE_PKCS_SIGNED_DATA: u16 = 0x0002;
        const PLACEHOLDER: [u8; 8] = *b"fixture!";

        pe.resize(pe.len().next_multiple_of(8), 0);
        let offset = u32::try_from(pe.len()).expect("fixture fits in u32");
        let length = u32::try_from(8 + PLACEHOLDER.len()).expect("certificate fits in u32");
        pe.extend(length.to_le_bytes());
        pe.extend(WIN_CERT_REVISION_2_0.to_le_bytes());
        pe.extend(WIN_CERT_TYPE_PKCS_SIGNED_DATA.to_le_bytes());
        pe.extend(PLACEHOLDER);

        let nt_headers = u32::from_le_bytes(pe[0x3c..0x40].try_into().expect("4-byte field"));
        let nt_headers = usize::try_from(nt_headers).expect("offset fits in usize");
        let directory = nt_headers + 24 + 112 + SECURITY_DIRECTORY * 8;
        pe[directory..directory + 4].copy_from_slice(&offset.to_le_bytes());
        pe[directory + 4..directory + 8].copy_from_slice(&length.to_le_bytes());
        pe
    }

    // Minimal PE32+ image with the requested sections. Used both for the outer
    // UKI and for the kernel nested in its `.linux` section.
    pub(crate) fn build_test_pe(machine: u16, sections: &[(&str, &[u8])]) -> Vec<u8> {
        let mut image = vec![0u8; 1024];
        image[..2].copy_from_slice(b"MZ");
        image[0x40..0x44].copy_from_slice(b"PE\0\0");
        for (offset, value) in [
            (0x44, machine),
            (0x46, sections.len() as u16),
            (0x54, 0xf0),
            (0x56, 0x22),
            (0x58, 0x20b),
            (0x9c, 10),
        ] {
            image[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
        }
        image[0x70..0x78].copy_from_slice(&0x400000u64.to_le_bytes());
        for (offset, value) in [
            (0x3c, 0x40u32),
            (0x78, 4096),
            (0x7c, 512),
            (0x90, (sections.len() as u32 + 1) * 4096),
            (0x94, 1024),
            (0xc4, 16),
        ] {
            image[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
        }
        for (index, (name, data)) in sections.iter().enumerate() {
            let header = 0x148 + 40 * index;
            let offset = image.len();
            let raw_size = data.len().next_multiple_of(512);
            image[header..header + name.len()].copy_from_slice(name.as_bytes());
            for (field, value) in [
                (8, data.len() as u32),
                (12, (index as u32 + 1) * 4096),
                (16, raw_size as u32),
                (20, offset as u32),
                (36, 0x40000040),
            ] {
                image[header + field..header + field + 4].copy_from_slice(&value.to_le_bytes());
            }
            image.extend_from_slice(data);
            image.resize(offset + raw_size, 0);
        }
        image
    }

    pub(crate) fn build_test_uki(machine: u16) -> Vec<u8> {
        with_cert_table(build_unsigned_test_uki(machine))
    }

    fn build_unsigned_test_uki(machine: u16) -> Vec<u8> {
        let kernel = build_test_pe(machine, &[(".text", b"fixture kernel")]);
        // File order deliberately differs from systemd's measurement order.
        build_test_pe(
            machine,
            &[
                (
                    ".cmdline",
                    b"console=ttyS0 dm-mod.create=\"root test\" -- systemd.unit=fipscheck.target\0",
                ),
                (".uname", b"6.18.48\0"),
                (".osrel", b"ID=bottlerocket\n"),
                (".linux", &kernel),
                (".sbat", b"sbat,1\n"),
            ],
        )
    }

    #[test]
    fn the_architecture_comes_from_the_machine_type() {
        let pe = |machine| with_cert_table(build_test_pe(machine, &[(".text", b"kernel")]));
        assert_eq!(Arch::of(&pe(0x8664)).unwrap(), Arch::X86_64);
        assert_eq!(Arch::of(&pe(0xaa64)).unwrap(), Arch::Aarch64);
        let err = Arch::of(&pe(0x5064)).unwrap_err();
        assert!(
            err.to_string().contains("unsupported architecture"),
            "{err}"
        );
    }

    #[test]
    fn test_authenticode_hash() {
        let pe = build_test_shim();
        let hash = get_authenticode_hash(&pe).unwrap();
        assert_eq!(
            hex::encode(hash.sha256),
            "a7f31bd8d50a1000d6250ec496a512bf119eccc0caa241f7f37e53f50e76aa6b"
        );
    }

    #[test]
    fn authenticode_pads_unaligned_images_like_pesign() {
        let mut pe = build_unsigned_test_shim(0xaa64);
        pe.extend([1, 2, 3]);
        let hash = get_authenticode_hash(&pe).unwrap();
        assert_eq!(
            hex::encode(hash.sha256),
            "30b5b493094dd3f75c49d3e50af28ba8969f72ec6774b434d62f1eb6fdf5c4de"
        );
        assert_eq!(
            hex::encode(hash.sha384),
            "e92c979218118271cb2a9f1d56b3b67c65d4334dd0cc0c4ef11f4249708aedefd755d50c46d85d4c73bbb743422a56ed"
        );
    }

    /// An error and every error it wraps, as one line.
    pub(crate) fn chain(error: &dyn std::error::Error) -> String {
        std::iter::successors(Some(error), |error| error.source())
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(": ")
    }

    /// Parse `image` as shim.
    pub(crate) fn shim(image: Vec<u8>) -> Result<Shim> {
        match Fallback::parse(image)? {
            Fallback::Shim(shim) => Ok(shim),
            Fallback::Uki(_) => panic!("expected shim, found a UKI"),
        }
    }

    /// Parse `image` as a UKI.
    pub(crate) fn uki(image: Vec<u8>) -> Result<Uki> {
        match Fallback::parse(image)? {
            Fallback::Uki(uki) => Ok(uki),
            Fallback::Shim(_) => panic!("expected a UKI, found shim"),
        }
    }

    /// Rename a test shim section in the COFF string table, so the image lacks it.
    fn without_section(mut image: Vec<u8>, name: &[u8]) -> Vec<u8> {
        let strings = 0x804..0x81c;
        let at = image[strings.clone()]
            .windows(name.len())
            .position(|window| window == name)
            .expect("section name in the string table")
            + strings.start;
        image[at + name.len() - 1] = b'X';
        image
    }

    #[test]
    fn test_vendor_cert() {
        let cert = shim(build_test_shim()).unwrap().vendor_cert().to_vec();
        assert_eq!(&cert[0..2], &[0x30, 0x82]); // ASN.1 SEQUENCE
        assert_eq!(cert.len(), 7);
    }

    #[test]
    fn test_sbat_level() {
        let shim = shim(build_test_shim()).unwrap();
        assert_eq!(shim.sbat_level(), b"sbat,1,2024010900\nshim,4\ngrub,3\n");
    }

    #[test]
    fn test_sbat_no_null_terminator() {
        let mut image = build_test_shim();
        // Keep the header, but leave no NUL in the strings.
        image[0x40c..0x600].fill(b'A');
        let err = shim(image).err().unwrap();
        assert!(err.to_string().contains("not null-terminated"));
    }

    #[test]
    fn test_sbat_section_too_small() {
        let mut image = build_test_shim();
        image[0x180..0x184].copy_from_slice(&10u32.to_le_bytes()); // SizeOfRawData = 10
        let err = shim(image).err().unwrap();
        assert!(err.to_string().contains("too small"));
    }

    #[test]
    fn test_vendor_cert_zero_size() {
        let mut image = build_test_shim();
        image[0x600..0x604].copy_from_slice(&0u32.to_le_bytes());
        let err = shim(image).err().unwrap();
        assert!(err.to_string().contains("no vendor certificate"));
    }

    #[test]
    fn test_vendor_cert_extends_beyond_section() {
        let mut image = build_test_shim();
        image[0x600..0x604].copy_from_slice(&0xFFFFu32.to_le_bytes());
        let err = shim(image).err().unwrap();
        assert!(err.to_string().contains("extends beyond"));
    }

    #[test]
    fn test_section_not_found() {
        let err = shim(without_section(build_test_shim(), b".vendor_cert"))
            .err()
            .unwrap();
        assert!(
            err.to_string().contains("'.vendor_cert' not found"),
            "{err}"
        );
        let err = shim(without_section(build_test_shim(), b".sbatlevel"))
            .err()
            .unwrap();
        assert!(err.to_string().contains("'.sbatlevel' not found"), "{err}");
    }

    #[test]
    fn a_fallback_that_is_not_pe32_plus_is_rejected() {
        let err = Fallback::parse(vec![0; 100]).err().unwrap();
        assert!(chain(&err).contains("PE32+"), "{}", chain(&err));
    }

    /// The fallback executable selects the prediction route, so shim must not
    /// be mistaken for a UKI.
    #[test]
    fn shim_is_not_detected_as_a_uki() {
        assert!(matches!(
            Fallback::parse(build_test_shim()).unwrap(),
            Fallback::Shim(_)
        ));
        assert!(matches!(
            Fallback::parse(build_test_uki(0x8664)).unwrap(),
            Fallback::Uki(_)
        ));
    }

    /// Shim also carries `.sbat`, so that section alone must not mark a UKI.
    #[test]
    fn sbat_alone_does_not_mark_a_uki() {
        let image = with_cert_table(build_test_pe(
            0x8664,
            &[(".text", b"shim"), (".sbat", b"sbat,1\n")],
        ));
        let err = Fallback::parse(image).err().unwrap();
        assert!(err.to_string().contains("'.sbatlevel' not found"), "{err}");
    }

    #[test]
    fn uki_sections_are_trimmed_and_absent_ones_are_empty() {
        let image = build_test_uki(0x8664);
        let uki = uki(image.clone()).unwrap();
        assert_eq!(uki.section(".uname"), b"6.18.48\0");
        assert!(uki.section(".initrd").is_empty());
        assert_eq!(uki.data(), image.as_slice());
    }

    #[test]
    fn uki_rejects_section_data_past_the_end_of_the_file() {
        let mut image = build_unsigned_test_uki(0x8664);
        // Keep only 3 of the 7 bytes of `.sbat`, the last section.
        image.truncate(image.len() - 512 + 3);
        let err = Fallback::parse(image).err().unwrap();
        assert!(chain(&err).contains("'.sbat'"), "{}", chain(&err));
    }

    /// A signed UKI with the given payload sections, and a kernel in `.linux`.
    fn signed_uki(sections: &[(&str, &[u8])]) -> Vec<u8> {
        let kernel = build_test_pe(0x8664, &[(".text", b"fixture kernel")]);
        let mut sections = sections.to_vec();
        sections.push((".linux", &kernel));
        with_cert_table(build_test_pe(0x8664, &sections))
    }

    #[test]
    fn an_unsigned_fallback_is_rejected() {
        for image in [
            build_unsigned_test_shim(0xaa64),
            build_unsigned_test_uki(0x8664),
        ] {
            let err = Fallback::parse(image).err().unwrap();
            assert!(err.to_string().contains("not signed"), "{err}");
        }
        let err = ensure_signed(&build_unsigned_test_shim(0xaa64), "grub").unwrap_err();
        assert_eq!(err.to_string(), "grub is not signed");
        ensure_signed(&build_test_shim(), "grub").unwrap();
    }

    #[test]
    fn a_uki_with_two_cmdline_sections_is_rejected() {
        let image = signed_uki(&[(".cmdline", b"first\0"), (".cmdline", b"second\0")]);
        let err = Fallback::parse(image).err().unwrap();
        assert!(
            err.to_string().contains("more than one '.cmdline'"),
            "{err}"
        );
    }

    #[test_case::test_case(".initrd" ; "an_initrd")]
    #[test_case::test_case(".ucode" ; "microcode")]
    #[test_case::test_case(".profile" ; "a_profile")]
    #[test_case::test_case(".pcrpkey" ; "a_pcr_public_key")]
    fn a_uki_section_pcrsys_does_not_predict_is_rejected(name: &str) {
        let image = signed_uki(&[(".cmdline", b"quiet\0"), (name, b"payload")]);
        let err = Fallback::parse(image).err().unwrap();
        assert!(err.to_string().contains(&format!("'{name}'")), "{err}");
    }

    #[test]
    fn a_uki_section_larger_in_memory_than_in_the_file_is_rejected() {
        let mut image = build_unsigned_test_uki(0x8664);
        // `.osrel` is the third section header; give it a VirtualSize past its 512 raw bytes.
        let virtual_size = 0x148 + 2 * 40 + 8;
        image[virtual_size..virtual_size + 4].copy_from_slice(&0x1000u32.to_le_bytes());
        let err = Fallback::parse(with_cert_table(image)).err().unwrap();
        assert!(
            err.to_string().contains("'.osrel' is larger in memory"),
            "{err}"
        );
    }

    #[test_case::test_case(b"noinitrd quiet\0" ; "noinitrd")]
    #[test_case::test_case(b"quiet \"initrd=\\\\EFI\\\\x\"\0" ; "a_quoted_initrd")]
    fn a_uki_command_line_that_overrides_the_initrd_is_rejected(cmdline: &[u8]) {
        let image = signed_uki(&[(".cmdline", cmdline)]);
        let err = Fallback::parse(image).err().unwrap();
        assert!(err.to_string().contains("overrides the initrd"), "{err}");
        // After the separator, the same words go to init, not the kernel.
        let image = signed_uki(&[(".cmdline", b"quiet -- noinitrd initrd=x\0")]);
        assert!(matches!(Fallback::parse(image).unwrap(), Fallback::Uki(_)));
    }

    #[test]
    fn a_shim_with_a_vendor_dbx_is_rejected() {
        let mut image = build_test_shim();
        image[0x604..0x608].copy_from_slice(&4u32.to_le_bytes()); // vendor_deauthorized_size
        let err = shim(image).err().unwrap();
        assert!(err.to_string().contains("vendor dbx"), "{err}");
    }

    #[test]
    fn a_shim_with_an_unexpected_sbatlevel_layout_is_rejected() {
        let mut image = build_test_shim();
        image[0x404..0x408].copy_from_slice(&12u32.to_le_bytes()); // automatic offset
        let err = shim(image).err().unwrap();
        assert!(
            err.to_string().contains("unexpected .sbatlevel layout"),
            "{err}"
        );
    }
}
