//! Test helpers for crates that feed binary bootconfig to this one.

/// Encode bootconfig text as `bootconfig -a` does: NUL-terminated and padded to four bytes,
/// then the padded size, the byte-sum checksum, and the magic.
pub fn encode(text: &str) -> Vec<u8> {
    let mut data = text.as_bytes().to_vec();
    data.push(0);
    data.resize(data.len().next_multiple_of(4), 0);
    let size = u32::try_from(data.len()).expect("bootconfig fits in u32");
    let checksum = data.iter().map(|&byte| u32::from(byte)).sum::<u32>();
    data.extend(size.to_le_bytes());
    data.extend(checksum.to_le_bytes());
    data.extend(b"#BOOTCONFIG\n");
    data
}
