//! Positional reads of a disk image, and of one partition within it.

use crate::gpt::PartitionInfo;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::os::unix::fs::FileExt;
use std::path::Path;
use std::sync::Arc;

/// A disk image read by position, so that several filesystem readers can share it.
pub trait DiskImage: Send + Sync {
    /// Fill `buf` from the image, starting at byte `offset`.
    fn read_exact_at(&self, buf: &mut [u8], offset: u64) -> io::Result<()>;

    /// The image's size in bytes.
    fn size(&self) -> u64;
}

/// A disk image file or block device.
pub struct FileImage {
    file: File,
    size: u64,
}

impl FileImage {
    /// Open the image at `path` for reading.
    pub fn open(path: &Path) -> io::Result<Self> {
        let mut file = File::open(path)?;
        // Unlike metadata, seeking also sizes block devices.
        let size = file.seek(SeekFrom::End(0))?;
        Ok(Self { file, size })
    }
}

impl DiskImage for FileImage {
    fn read_exact_at(&self, buf: &mut [u8], offset: u64) -> io::Result<()> {
        FileExt::read_exact_at(&self.file, buf, offset)
    }

    fn size(&self) -> u64 {
        self.size
    }
}

impl DiskImage for Vec<u8> {
    fn read_exact_at(&self, buf: &mut [u8], offset: u64) -> io::Result<()> {
        let source = usize::try_from(offset)
            .ok()
            .and_then(|start| self.get(start..start.checked_add(buf.len())?))
            .ok_or_else(|| io::Error::from(io::ErrorKind::UnexpectedEof))?;
        buf.copy_from_slice(source);
        Ok(())
    }

    fn size(&self) -> u64 {
        u64::try_from(self.len()).unwrap_or(u64::MAX)
    }
}

/// One partition of a disk image, read with offsets relative to its start.
pub struct PartitionReader {
    disk: Arc<dyn DiskImage>,
    start: u64,
    size: u64,
    position: u64,
}

impl PartitionReader {
    /// A reader over `partition`, whose extent the GPT has already bounded by the disk.
    pub fn new(disk: Arc<dyn DiskImage>, partition: &PartitionInfo) -> Self {
        Self {
            disk,
            start: partition.offset_bytes(),
            size: partition.size_bytes(),
            position: 0,
        }
    }
}

/// Like a `Cursor`: a read at the end returns fewer bytes, or none.
impl Read for PartitionReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let remaining = self.size.saturating_sub(self.position);
        let count =
            usize::try_from(remaining).map_or(buf.len(), |remaining| remaining.min(buf.len()));
        let buf = &mut buf[..count];
        self.disk.read_exact_at(buf, self.start + self.position)?;
        self.position += u64::try_from(count).expect("a read fits in u64");
        Ok(count)
    }
}

impl Seek for PartitionReader {
    fn seek(&mut self, position: SeekFrom) -> io::Result<u64> {
        let position = match position {
            SeekFrom::Start(offset) => Some(offset),
            SeekFrom::End(delta) => self.size.checked_add_signed(delta),
            SeekFrom::Current(delta) => self.position.checked_add_signed(delta),
        }
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "seek before the partition"))?;
        self.position = position;
        Ok(position)
    }
}

/// The filesystem readers only read, and refusing writes keeps the image untouched.
impl Write for PartitionReader {
    fn write(&mut self, _: &[u8]) -> io::Result<usize> {
        #[cfg(test)]
        tests::WRITE_ATTEMPTS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "partitions are read-only",
        ))
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// Like a `Vec`: a read past the end of the partition is an error.
impl ext4_view::Ext4Read for PartitionReader {
    fn read(
        &mut self,
        start_byte: u64,
        dst: &mut [u8],
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let end = u64::try_from(dst.len())
            .ok()
            .and_then(|len| start_byte.checked_add(len));
        if end.is_none_or(|end| end > self.size) {
            return Err(format!(
                "read of {} bytes at {start_byte} is past the partition",
                dst.len()
            )
            .into());
        }
        Ok(self.disk.read_exact_at(dst, self.start + start_byte)?)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use std::sync::atomic::AtomicUsize;
    use std::sync::Mutex;

    /// Writes that filesystem readers attempted, across every test.
    pub(crate) static WRITE_ATTEMPTS: AtomicUsize = AtomicUsize::new(0);

    /// A disk image that records the byte ranges read from it.
    pub(crate) struct CountingImage {
        image: Vec<u8>,
        pub(crate) reads: Mutex<Vec<(u64, u64)>>,
    }

    impl CountingImage {
        pub(crate) fn new(image: Vec<u8>) -> Self {
            Self {
                image,
                reads: Mutex::new(Vec::new()),
            }
        }

        /// Bytes read that fall inside `partition`.
        pub(crate) fn bytes_read_from(&self, partition: &PartitionInfo) -> u64 {
            let (start, end) = (
                partition.offset_bytes(),
                partition.offset_bytes() + partition.size_bytes(),
            );
            self.reads
                .lock()
                .expect("no test panics while holding the lock")
                .iter()
                .map(|&(offset, len)| (offset + len).min(end).saturating_sub(offset.max(start)))
                .sum()
        }
    }

    impl DiskImage for CountingImage {
        fn read_exact_at(&self, buf: &mut [u8], offset: u64) -> io::Result<()> {
            let len = u64::try_from(buf.len()).expect("a read fits in u64");
            self.reads
                .lock()
                .expect("no test panics while holding the lock")
                .push((offset, len));
            self.image.read_exact_at(buf, offset)
        }

        fn size(&self) -> u64 {
            self.image.size()
        }
    }

    fn partition(start_lba: u64, end_lba: u64) -> PartitionInfo {
        PartitionInfo {
            number: 1,
            start_lba,
            end_lba,
            unique_guid: [0; 16],
        }
    }

    #[test]
    fn reads_stay_inside_the_partition() {
        let image: Vec<u8> = (0..4 * 512)
            .map(|i| u8::try_from(i / 512).unwrap())
            .collect();
        let mut reader = PartitionReader::new(Arc::new(image), &partition(1, 2));
        let mut buf = vec![0; 2048];
        assert_eq!(Read::read(&mut reader, &mut buf).unwrap(), 1024);
        assert!(buf[..512].iter().all(|&b| b == 1) && buf[512..1024].iter().all(|&b| b == 2));
        assert_eq!(Read::read(&mut reader, &mut buf).unwrap(), 0);
        reader.seek(SeekFrom::Start(1000)).unwrap();
        assert_eq!(Read::read(&mut reader, &mut buf).unwrap(), 24);
        assert!(reader.seek(SeekFrom::Current(-2000)).is_err());

        let mut dst = [0; 16];
        ext4_view::Ext4Read::read(&mut reader, 1008, &mut dst).unwrap();
        assert!(ext4_view::Ext4Read::read(&mut reader, 1009, &mut dst).is_err());
    }
}
