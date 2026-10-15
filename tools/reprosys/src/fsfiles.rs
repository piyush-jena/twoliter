//! File-level comparison of root filesystem contents and metadata.
use crate::error::Result;
use crate::kmod;
use ext4_view::Ext4;
use regex::bytes::Regex;
use serde::Serialize;
use sha2::{Digest, Sha256};
use snafu::{ensure_whatever, ResultExt};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::io::{Cursor, Read};
use std::sync::LazyLock;

/// The TUF root role, which rebuilds cannot reproduce without the release keys.
const TUF_ROOT_ROLE: &[u8] = b"/usr/share/updog/root.json";

/// The SBOMs in ROOT-A, which sbomtool cannot reproduce yet.
const SBOMS: &[&[u8]] = &[
    b"/usr/share/bottlerocket/spdx-sbom.json",
    b"/usr/share/bottlerocket/cyclonedx-sbom.json",
];

/// Largest file read into memory for a content-aware comparison.
const MAX_BUFFERED_SIZE: u64 = 256 << 20;

/// Which differences are expected in a filesystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rules {
    /// ROOT-A: module signatures, the TUF root role and the SBOMs.
    Root,
    /// PRIVATE: none; compared only to explain a difference.
    Exact,
}

/// The verity root hash and salt in a device-mapper table.
static VERITY_DIGESTS_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(sha256) [0-9a-fA-F]{64} (?:[0-9a-fA-F]+|-)").expect("valid regex")
});

/// An absolute path within a filesystem. File names are bytes and need not be UTF-8.
pub type FsPath = Vec<u8>;

pub type Files = BTreeMap<FsPath, Vec<u8>>;

/// Extended attributes, by full name (such as `security.selinux`).
pub type Xattrs = BTreeMap<Vec<u8>, Vec<u8>>;

/// Show a path or name in reports; invalid UTF-8 is replaced.
pub fn display(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

/// Insert `value` at `key`, failing if it is already present.
pub fn insert_unique<V>(map: &mut BTreeMap<Vec<u8>, V>, key: Vec<u8>, value: V) -> Result<()> {
    ensure_whatever!(!map.contains_key(&key), "duplicate entry {}", display(&key));
    map.insert(key, value);
    Ok(())
}

/// The keys of either map, in order.
pub fn union_keys<'a, K: Ord, V, W>(
    a: &'a BTreeMap<K, V>,
    b: &'a BTreeMap<K, W>,
) -> BTreeSet<&'a K> {
    a.keys().chain(b.keys()).collect()
}

/// The first `n` of `items`, and how many more there are, or "none".
pub fn first_n(items: &[String], n: usize) -> String {
    match items.len() {
        0 => "none".to_string(),
        len if len > n => format!("{} and {} more", items[..n].join(", "), len - n),
        _ => items.join(", "),
    }
}

/// Bounds the file data read from a partition to the partition's size.
struct ReadBudget {
    limit: u64,
    remaining: u64,
}

impl ReadBudget {
    fn new(partition: &[u8]) -> Self {
        let limit = partition.len() as u64;
        Self {
            limit,
            remaining: limit,
        }
    }

    /// Account for reading `len` bytes of `path`.
    fn take(&mut self, path: &str, len: u64) -> Result<()> {
        ensure_whatever!(
            len <= self.remaining,
            "reading {path} ({len} bytes) would read more file data than the {}-byte partition holds",
            self.limit
        );
        self.remaining -= len;
        Ok(())
    }
}

/// Read every regular file from an ext4 filesystem image.
pub fn ext4_files(partition: Vec<u8>) -> Result<Files> {
    let mut budget = ReadBudget::new(&partition);
    let fs = Ext4::load(Box::new(partition)).whatever_context("failed to mount ext4 filesystem")?;
    let mut files = Files::new();
    let mut stack = vec![ext4_view::PathBuf::new("/")];
    while let Some(dir) = stack.pop() {
        let entries = fs
            .read_dir(&dir)
            .whatever_context(format!("failed to read directory {}", dir.display()))?;
        for entry in entries {
            let entry =
                entry.whatever_context(format!("failed to read directory {}", dir.display()))?;
            let name = entry.file_name();
            let name = name.as_ref();
            if name == b"." || name == b".." || name == b"lost+found" {
                continue;
            }
            let path = entry.path();
            let metadata = entry
                .metadata()
                .whatever_context(format!("failed to stat {}", path.display()))?;
            if metadata.is_dir() {
                stack.push(path);
            } else if metadata.file_type().is_regular_file() {
                // Sparse files and hardlinks can claim more than the partition holds.
                budget.take(&path.display().to_string(), metadata.len())?;
                let contents = fs
                    .read(&path)
                    .whatever_context(format!("failed to read {}", path.display()))?;
                insert_unique(&mut files, path.as_ref().to_vec(), contents)?;
            }
        }
    }
    Ok(files)
}

/// How a region or a file of the published image compares to the rebuilt one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Status {
    Match,
    ExpectedDifference,
    Mismatch,
}

#[derive(Debug, Clone, Serialize)]
pub struct FileReport {
    pub path: String,
    pub status: Status,
    pub detail: String,
}

/// Replace the verity root hash and salt in `data` with placeholders.
pub fn normalize_verity(data: &[u8]) -> Vec<u8> {
    VERITY_DIGESTS_RE
        .replace_all(data, &b"$1 <root-hash> <salt>"[..])
        .into_owned()
}

/// Compare a kernel module, ignoring its appended signature.
fn compare_module(path: &[u8], expected: &[u8], actual: &[u8]) -> (Status, String) {
    let unsigned = |data: &[u8]| -> Result<Option<Vec<u8>>> {
        let module = kmod::decompress(path, data)?;
        Ok(kmod::split_signature(&module)?.map(|(body, _)| body.to_vec()))
    };
    match (unsigned(expected), unsigned(actual)) {
        (Ok(Some(e)), Ok(Some(a))) if e == a => (
            Status::ExpectedDifference,
            "differs only in the appended module signature".to_string(),
        ),
        (Ok(Some(e)), Ok(Some(a))) => (
            Status::Mismatch,
            format!(
                "module differs beyond its signature (unsigned: {} vs {})",
                ContentDigest::of_slice(&e),
                ContentDigest::of_slice(&a)
            ),
        ),
        (Ok(None), Ok(None)) => (
            Status::Mismatch,
            format!(
                "unsigned module contents differ ({} vs {})",
                ContentDigest::of_slice(expected),
                ContentDigest::of_slice(actual)
            ),
        ),
        (Ok(e), Ok(a)) => {
            let side = |m: &Option<Vec<u8>>| if m.is_some() { "signed" } else { "unsigned" };
            (
                Status::Mismatch,
                format!(
                    "module is {} in the rebuilt image but {} in the published image",
                    side(&e),
                    side(&a)
                ),
            )
        }
        (Err(e), _) | (_, Err(e)) => (Status::Mismatch, format!("failed to read module: {e}")),
    }
}

fn is_sbom(path: &[u8]) -> bool {
    SBOMS.iter().any(|s| path.ends_with(s))
}

fn report(path: &[u8], status: Status, detail: impl Into<String>) -> FileReport {
    FileReport {
        path: display(path),
        status,
        detail: detail.into(),
    }
}

/// Compare one file between the rebuilt and the published image.
fn compare_file(
    path: &[u8],
    expected: Option<&[u8]>,
    actual: Option<&[u8]>,
    rules: Rules,
) -> FileReport {
    let (expected, actual) = match (expected, actual) {
        (Some(e), Some(a)) => (e, a),
        (None, _) => return report(path, Status::Mismatch, "missing from rebuilt image"),
        (_, None) => return report(path, Status::Mismatch, "missing from published image"),
    };

    if expected == actual {
        return report(path, Status::Match, "identical");
    }
    match rules {
        Rules::Root => {
            if kmod::is_module(path) {
                let (status, detail) = compare_module(path, expected, actual);
                return report(path, status, detail);
            }
            if path.ends_with(TUF_ROOT_ROLE) {
                return report(
                    path,
                    Status::ExpectedDifference,
                    "TUF root role; skipped, rebuilds generate their own without the release keys",
                );
            }
            if is_sbom(path) {
                return report(
                    path,
                    Status::ExpectedDifference,
                    "SBOM; skipped, sbomtool output is not reproducible",
                );
            }
        }
        Rules::Exact => {}
    }
    report(
        path,
        Status::Mismatch,
        format!(
            "contents differ ({} vs {})",
            ContentDigest::of_slice(expected),
            ContentDigest::of_slice(actual)
        ),
    )
}

/// Compare all files; identical ones are logged but not returned.
pub fn compare_files(expected: &Files, actual: &Files, rules: Rules) -> Vec<FileReport> {
    union_keys(expected, actual)
        .into_iter()
        .map(|p| {
            let (e, a) = (
                expected.get(p).map(Vec::as_slice),
                actual.get(p).map(Vec::as_slice),
            );
            compare_file(p, e, a, rules)
        })
        .filter(|r| r.status != Status::Match)
        .collect()
}

/// The type of a filesystem entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    File,
    Directory,
    Symlink,
    CharDevice,
    BlockDevice,
    Fifo,
    Socket,
}

impl Kind {
    pub fn name(self) -> &'static str {
        match self {
            Kind::File => "file",
            Kind::Directory => "directory",
            Kind::Symlink => "symlink",
            Kind::CharDevice => "character device",
            Kind::BlockDevice => "block device",
            Kind::Fifo => "fifo",
            Kind::Socket => "socket",
        }
    }
}

/// The attributes of a filesystem entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub kind: Kind,
    /// Permission bits, including setuid, setgid and sticky.
    pub mode: u32,
    pub uid: u32,
    pub gid: u32,
    pub link: Option<Vec<u8>>,
    pub xattrs: Xattrs,
    /// Size of a regular file's contents, which are compared separately.
    pub size: u64,
    /// Major and minor number of a character or block device.
    pub device: Option<(u32, u32)>,
}

/// Split a `new_encode_dev` device number into its major and minor number.
pub fn decode_device(dev: u32) -> (u32, u32) {
    (
        (dev & 0xf_ff00) >> 8,
        (dev & 0xff) | ((dev >> 12) & 0xf_ff00),
    )
}

/// Values longer than this are shown by size and digest.
const MAX_SHOWN_XATTR: usize = 128;

/// Show an xattr value as text, hex, or by size and digest if it is long.
fn describe_xattr(value: Option<&Vec<u8>>) -> String {
    let Some(value) = value else {
        return "<none>".to_string();
    };
    let text = value.strip_suffix(b"\0").unwrap_or(value);
    if value.len() > MAX_SHOWN_XATTR {
        ContentDigest::of_slice(value).to_string()
    } else if !text.is_empty() && text.iter().all(|b| b.is_ascii_graphic() || *b == b' ') {
        format!("\"{}\"", display(text))
    } else {
        format!("0x{}", hex::encode(value))
    }
}

impl Entry {
    /// How `self` (rebuilt) differs from `other` (published), apart from contents.
    fn differences(&self, other: &Self) -> Vec<String> {
        let mut out = Vec::new();
        if self.kind != other.kind {
            out.push(format!(
                "type {} vs {}",
                self.kind.name(),
                other.kind.name()
            ));
        }
        if self.mode != other.mode {
            out.push(format!("mode {:04o} vs {:04o}", self.mode, other.mode));
        }
        if self.uid != other.uid || self.gid != other.gid {
            out.push(format!(
                "owner {}:{} vs {}:{}",
                self.uid, self.gid, other.uid, other.gid
            ));
        }
        if self.link != other.link {
            let target = |l: &Option<Vec<u8>>| l.as_deref().map_or("<none>".to_string(), display);
            out.push(format!(
                "symlink target {} vs {}",
                target(&self.link),
                target(&other.link)
            ));
        }
        if self.device != other.device {
            let device = |d: Option<(u32, u32)>| {
                d.map_or("<none>".to_string(), |(major, minor)| {
                    format!("{major}:{minor}")
                })
            };
            out.push(format!(
                "device {} vs {}",
                device(self.device),
                device(other.device)
            ));
        }
        for name in union_keys(&self.xattrs, &other.xattrs) {
            let (e, a) = (self.xattrs.get(name), other.xattrs.get(name));
            if e != a {
                out.push(format!(
                    "xattr {} {} vs {}",
                    display(name),
                    describe_xattr(e),
                    describe_xattr(a)
                ));
            }
        }
        out
    }
}

/// A filesystem whose entries can be listed and whose files can be read.
pub trait FileTree {
    /// What `open` needs to find a file again.
    type Handle;

    /// Every entry, including the root directory, keyed by absolute path.
    fn entries(&self) -> Result<BTreeMap<FsPath, (Self::Handle, Entry)>>;

    /// Open a regular file to read its contents.
    fn open(&self, handle: &Self::Handle) -> Result<Box<dyn Read + '_>>;
}

/// The size and sha256 of a file's contents.
#[derive(PartialEq, Eq)]
pub struct ContentDigest {
    size: u64,
    sha256: [u8; 32],
}

impl ContentDigest {
    /// Hash up to `size` bytes from `reader`, without holding them in memory.
    fn of(reader: impl Read, size: u64) -> Result<Self> {
        let mut reader = reader.take(size);
        let mut hasher = Sha256::new();
        let mut buf = vec![0u8; 1 << 20];
        let mut size = 0u64;
        loop {
            let n = reader
                .read(&mut buf)
                .whatever_context("failed to read file")?;
            if n == 0 {
                break;
            }
            hasher.update(&buf[..n]);
            size += n as u64;
        }
        Ok(Self {
            size,
            sha256: hasher.finalize().into(),
        })
    }

    pub fn of_slice(data: &[u8]) -> Self {
        Self {
            size: data.len() as u64,
            sha256: Sha256::digest(data).into(),
        }
    }
}

impl fmt::Display for ContentDigest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} bytes, sha256 {}",
            self.size,
            hex::encode(self.sha256)
        )
    }
}

fn read_contents(reader: impl Read, size: u64) -> Result<Vec<u8>> {
    let mut out = Vec::new();
    reader
        .take(size)
        .read_to_end(&mut out)
        .whatever_context("failed to read file")?;
    Ok(out)
}

/// A regular file in a tree: the tree, the file's handle and its size.
type TreeFile<'a, T> = (&'a T, &'a <T as FileTree>::Handle, u64);

/// Compare a regular file in both trees. Published files larger than both
/// `size_limit` and the rebuilt file are not read.
fn compare_contents<E: FileTree, A: FileTree>(
    path: &[u8],
    (e_tree, e_handle, e_size): TreeFile<'_, E>,
    (a_tree, a_handle, a_size): TreeFile<'_, A>,
    rules: Rules,
    size_limit: u64,
) -> Result<FileReport> {
    if a_size > size_limit.max(e_size) {
        return Ok(report(
            path,
            Status::Mismatch,
            format!(
                "published file is {a_size} bytes, more than the rebuilt file ({e_size} bytes) \
                 and its partition ({size_limit} bytes); not read"
            ),
        ));
    }
    let content_aware = rules == Rules::Root
        && (kmod::is_module(path) || path.ends_with(TUF_ROOT_ROLE) || is_sbom(path));
    if content_aware {
        if e_size.max(a_size) > MAX_BUFFERED_SIZE {
            return Ok(report(
                path,
                Status::Mismatch,
                format!(
                    "too large to compare ({e_size} vs {a_size} bytes, limit {MAX_BUFFERED_SIZE})"
                ),
            ));
        }
        let expected = read_contents(e_tree.open(e_handle)?, e_size)?;
        let actual = read_contents(a_tree.open(a_handle)?, a_size)?;
        return Ok(compare_file(path, Some(&expected), Some(&actual), rules));
    }
    let expected = ContentDigest::of(e_tree.open(e_handle)?, e_size)?;
    let actual = ContentDigest::of(a_tree.open(a_handle)?, a_size)?;
    Ok(if expected == actual {
        report(path, Status::Match, "identical")
    } else {
        report(
            path,
            Status::Mismatch,
            format!("contents differ ({expected} vs {actual})"),
        )
    })
}

/// Compare two filesystems entry by entry, apart from timestamps. Identical
/// entries are omitted.
pub fn compare_trees<E: FileTree, A: FileTree>(
    expected: &E,
    actual: &A,
    rules: Rules,
    size_limit: u64,
) -> Result<Vec<FileReport>> {
    let expected_entries = expected.entries()?;
    let actual_entries = actual.entries()?;
    let mut reports = Vec::new();
    for path in union_keys(&expected_entries, &actual_entries) {
        let (e, a) = (expected_entries.get(path), actual_entries.get(path));
        let file = match (e, a) {
            (Some((e_handle, e)), Some((a_handle, a))) => {
                let differences = e.differences(a);
                if !differences.is_empty() {
                    report(path, Status::Mismatch, differences.join("; "))
                } else if e.kind != Kind::File {
                    report(path, Status::Match, "identical")
                } else {
                    compare_contents(
                        path,
                        (expected, e_handle, e.size),
                        (actual, a_handle, a.size),
                        rules,
                        size_limit,
                    )?
                }
            }
            (e, _) => {
                let side = if e.is_none() { "rebuilt" } else { "published" };
                report(path, Status::Mismatch, format!("missing from {side} image"))
            }
        };
        if file.status != Status::Match {
            reports.push(file);
        }
    }
    Ok(reports)
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    fn files(entries: &[(&str, Vec<u8>)]) -> Files {
        entries
            .iter()
            .map(|(p, c)| (p.as_bytes().to_vec(), c.clone()))
            .collect()
    }

    /// Assert that `reports` are of exactly the files in `expected`, with their statuses.
    fn assert_statuses(reports: Vec<FileReport>, expected: &[(&str, Status)]) {
        let found: BTreeMap<_, _> = reports.into_iter().map(|r| (r.path, r.status)).collect();
        let expected = expected.iter().map(|(p, s)| (p.to_string(), *s)).collect();
        assert_eq!(found, expected);
    }

    /// The status of `path` holding `rebuilt` in one image and `published` in the other.
    fn status(path: &str, rebuilt: &[u8], published: &[u8], rules: Rules) -> Status {
        let reports = compare_files(
            &files(&[(path, rebuilt.to_vec())]),
            &files(&[(path, published.to_vec())]),
            rules,
        );
        reports.first().map_or(Status::Match, |r| r.status)
    }

    #[test]
    fn root_rules() {
        let sign = crate::kmod::tests::sign;
        let role = "/aarch64-bottlerocket-linux-gnu/sys-root/usr/share/updog/root.json";
        for (path, rebuilt, published, expected) in [
            (
                "/m/xfs.ko",
                sign(b"module", b"key1"),
                sign(b"module", b"key2"),
                Status::ExpectedDifference,
            ),
            (
                "/m/xfs.ko",
                sign(b"module", b"key1"),
                sign(b"modulf", b"key2"),
                Status::Mismatch,
            ),
            (
                "/m/xfs.ko",
                b"module".to_vec(),
                sign(b"module", b"key2"),
                Status::Mismatch,
            ),
            // Certificates are only signing artifacts in the boot partitions.
            (
                "/etc/ca.crt",
                b"ca1".to_vec(),
                b"ca2".to_vec(),
                Status::Mismatch,
            ),
            // The root role is skipped, whatever it holds.
            (
                role,
                b"aaaa".to_vec(),
                b"not json".to_vec(),
                Status::ExpectedDifference,
            ),
        ] {
            let found = status(path, &rebuilt, &published, Rules::Root);
            assert_eq!(found, expected, "{path}: {published:?}");
        }
    }

    const SBOM: &str =
        "/x86_64-bottlerocket-linux-gnu/sys-root/usr/share/bottlerocket/spdx-sbom.json";

    fn entry(mode: u32, uid: u32, size: u64, xattrs: &[(&str, &[u8])]) -> Entry {
        Entry {
            kind: Kind::File,
            mode,
            uid,
            gid: 0,
            link: None,
            xattrs: xattrs
                .iter()
                .map(|(n, v)| (n.as_bytes().to_vec(), v.to_vec()))
                .collect(),
            size,
            device: None,
        }
    }

    #[test]
    fn reports_xattr_differences() {
        let label: &[(&str, &[u8])] = &[("security.selinux", b"system_u:object_r:os_t:s0\0")];
        let rebuilt = entry(0o755, 0, 0, label);
        assert!(rebuilt.differences(&rebuilt.clone()).is_empty());
        let capability: &[u8] = &[1, 0, 0, 2, 0, 0x20, 0, 0];
        let capable = [("security.capability", capability), label[0]];
        let capable = entry(0o755, 0, 0, &capable);
        assert_eq!(rebuilt.differences(&capable).len(), 1);
    }

    /// An in-memory tree whose files can be opened, for testing `compare_trees`.
    struct MemTree(BTreeMap<FsPath, (Vec<u8>, Entry)>);

    impl FileTree for MemTree {
        type Handle = Vec<u8>;

        fn entries(&self) -> Result<BTreeMap<FsPath, (Vec<u8>, Entry)>> {
            Ok(self.0.clone())
        }

        fn open(&self, handle: &Vec<u8>) -> Result<Box<dyn Read + '_>> {
            Ok(Box::new(Cursor::new(handle.clone())))
        }
    }

    fn mem_tree(files: &[(&str, &[u8], u64)]) -> MemTree {
        let files = files.iter().map(|(path, contents, size)| {
            let entry = entry(0o644, 0, *size, &[]);
            (path.as_bytes().to_vec(), (contents.to_vec(), entry))
        });
        MemTree(files.collect())
    }

    #[test]
    fn compare_trees_rules() {
        let (e, a): (&[u8], &[u8]) = (b"{\"spdx\":1}", b"{\"spdx\":22}");
        let expected = mem_tree(&[
            ("/a", b"one", 3),
            ("/b", b"two", 3),
            ("/big", b"", 0),
            (SBOM, e, e.len() as u64),
        ]);
        let actual = mem_tree(&[
            ("/a", b"one", 3),
            ("/b", b"tw0", 3),
            // The size alone fails it.
            ("/big", b"", 1 << 40),
            // Read from a tree, as ROOT-A is.
            (SBOM, a, a.len() as u64),
        ]);
        let reports = compare_trees(&expected, &actual, Rules::Root, 1 << 20).unwrap();
        let big = reports.iter().find(|r| r.path == "/big").unwrap();
        assert!(big.detail.contains("not read"), "{big:?}");
        assert_statuses(
            reports,
            &[
                ("/b", Status::Mismatch),
                ("/big", Status::Mismatch),
                (SBOM, Status::ExpectedDifference),
            ],
        );
    }
}
