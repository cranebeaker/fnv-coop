use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::ops::Range;
use std::path::Path;
use std::sync::Mutex;

use esm::{inflate, text};

use crate::error::{Error, Result};
use crate::hash::{hash_file_name, hash_folder_name};

pub const MAGIC: &[u8; 4] = b"BSA\0";
pub const HEADER_LEN: usize = 36;
const FOLDER_RECORD_LEN: u64 = 16;
const FILE_RECORD_LEN: u64 = 16;
/// Bit in a file record's size field that flips the archive's default
/// compression for that one file.
const COMPRESSION_TOGGLE: u32 = 0x4000_0000;
const SIZE_MASK: u32 = 0x3FFF_FFFF;

/// Archive flag bits (header field `archive_flags`).
pub mod archive_flags {
    pub const DIRECTORY_NAMES: u32 = 0x001;
    pub const FILE_NAMES: u32 = 0x002;
    /// Files are compressed unless their record toggles it off.
    pub const COMPRESSED: u32 = 0x004;
    pub const RETAIN_DIRECTORY_NAMES: u32 = 0x008;
    pub const RETAIN_FILE_NAMES: u32 = 0x010;
    pub const RETAIN_FILE_NAME_OFFSETS: u32 = 0x020;
    /// Big-endian Xbox 360 archive.
    pub const XBOX: u32 = 0x040;
    pub const RETAIN_STRINGS: u32 = 0x080;
    /// Version 104: each file's data starts with its full path.
    pub const EMBEDDED_FILE_NAMES: u32 = 0x100;
    /// Xbox 360 XMem compression.
    pub const XMEM: u32 = 0x200;

    pub const NAMED: [(u32, &str); 10] = [
        (DIRECTORY_NAMES, "directory names"),
        (FILE_NAMES, "file names"),
        (COMPRESSED, "compressed"),
        (RETAIN_DIRECTORY_NAMES, "retain directory names"),
        (RETAIN_FILE_NAMES, "retain file names"),
        (RETAIN_FILE_NAME_OFFSETS, "retain file name offsets"),
        (XBOX, "Xbox 360"),
        (RETAIN_STRINGS, "retain strings"),
        (EMBEDDED_FILE_NAMES, "embedded file names"),
        (XMEM, "XMem codec"),
    ];

    pub fn describe(value: u32) -> String {
        super::describe_bits(value, &NAMED)
    }
}

/// Content-type bits (header field `file_flags`).
pub mod content_flags {
    pub const NAMED: [(u32, &str); 9] = [
        (0x001, "meshes"),
        (0x002, "textures"),
        (0x004, "menus"),
        (0x008, "sounds"),
        (0x010, "voices"),
        (0x020, "shaders"),
        (0x040, "trees"),
        (0x080, "fonts"),
        (0x100, "misc"),
    ];

    pub fn describe(value: u32) -> String {
        super::describe_bits(value, &NAMED)
    }
}

fn describe_bits(value: u32, names: &[(u32, &str)]) -> String {
    let mut parts: Vec<String> = Vec::new();
    let mut rest = value;
    for &(bit, name) in names {
        if value & bit != 0 {
            parts.push(name.to_string());
            rest &= !bit;
        }
    }
    if rest != 0 {
        parts.push(format!("other bits {rest:#x}"));
    }
    if parts.is_empty() {
        "none".into()
    } else {
        parts.join(", ")
    }
}

/// The archive's fixed 36-byte header.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Header {
    pub version: u32,
    pub archive_flags: u32,
    pub folder_count: u32,
    pub file_count: u32,
    pub total_folder_name_length: u32,
    pub total_file_name_length: u32,
    /// Which kinds of content the archive holds; see [`content_flags`].
    pub content_flags: u32,
}

impl Header {
    pub fn has(&self, flag: u32) -> bool {
        self.archive_flags & flag != 0
    }

    /// Size of everything before the file data: header, folder records,
    /// file record blocks and the file name table.
    fn directory_len(&self) -> u64 {
        let folders = u64::from(self.folder_count);
        let mut len = HEADER_LEN as u64 + folders * FOLDER_RECORD_LEN;
        if self.has(archive_flags::DIRECTORY_NAMES) {
            len += folders + u64::from(self.total_folder_name_length);
        }
        len += u64::from(self.file_count) * FILE_RECORD_LEN;
        if self.has(archive_flags::FILE_NAMES) {
            len += u64::from(self.total_file_name_length);
        }
        len
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Folder {
    /// Lower-case path with backslashes, e.g. `meshes\weapons`.
    pub name: String,
    pub name_hash: u64,
    /// Indexes into [`Archive::files`].
    pub files: Range<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileEntry {
    /// Full lower-case path with backslashes, e.g. `meshes\weapons\gun.nif`.
    pub path: String,
    /// File name without the folder.
    pub name: String,
    /// Index into [`Archive::folders`].
    pub folder: usize,
    pub name_hash: u64,
    /// Absolute offset of the file's data in the archive.
    pub offset: u32,
    /// Bytes stored in the archive (compressed size for compressed files).
    pub stored_size: u32,
    pub compressed: bool,
}

/// Result of [`Archive::check_name_hashes`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HashCheck {
    pub folders_matched: usize,
    pub folders_total: usize,
    pub files_matched: usize,
    pub files_total: usize,
}

enum Source {
    File(Mutex<File>),
    Memory(Vec<u8>),
}

/// An opened BSA archive.
pub struct Archive {
    source: Source,
    len: u64,
    header: Header,
    folders: Vec<Folder>,
    files: Vec<FileEntry>,
    by_path: HashMap<String, usize>,
}

/// Lower-cases a path, uses backslashes, and drops a leading `data\`, so
/// `"Data/Meshes/Gun.NIF"` and `"meshes\gun.nif"` refer to the same file.
pub fn normalize_path(path: &str) -> String {
    let mut p = path.replace('/', "\\").to_ascii_lowercase();
    while p.starts_with('\\') {
        p.remove(0);
    }
    if let Some(rest) = p.strip_prefix("data\\") {
        p = rest.to_string();
    }
    p
}

impl Archive {
    /// Opens an archive on disk, reading only its directory.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let mut file = File::open(path)?;
        let len = file.metadata()?.len();
        let header_bytes = read_exact_at(&mut file, 0, HEADER_LEN.min(len as usize))?;
        let header = parse_header(&header_bytes)?;
        let directory_len = checked_directory_len(&header, len)?;
        let directory = read_exact_at(&mut file, 0, directory_len)?;
        let (folders, files) = parse_directory(&header, &directory, len)?;
        Ok(Self::assemble(
            Source::File(Mutex::new(file)),
            len,
            header,
            folders,
            files,
        ))
    }

    /// Reads an archive held in memory.
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        let len = bytes.len() as u64;
        let header = parse_header(&bytes[..HEADER_LEN.min(bytes.len())])?;
        let directory_len = checked_directory_len(&header, len)?;
        let (folders, files) = parse_directory(&header, &bytes[..directory_len], len)?;
        Ok(Self::assemble(
            Source::Memory(bytes),
            len,
            header,
            folders,
            files,
        ))
    }

    fn assemble(
        source: Source,
        len: u64,
        header: Header,
        folders: Vec<Folder>,
        files: Vec<FileEntry>,
    ) -> Self {
        let mut by_path = HashMap::with_capacity(files.len());
        for (i, f) in files.iter().enumerate() {
            by_path.entry(f.path.clone()).or_insert(i);
        }
        Self {
            source,
            len,
            header,
            folders,
            files,
            by_path,
        }
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    /// Size of the archive file in bytes.
    pub fn len(&self) -> u64 {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }

    pub fn folders(&self) -> &[Folder] {
        &self.folders
    }

    pub fn files(&self) -> &[FileEntry] {
        &self.files
    }

    /// Looks a file up by path, ignoring case and slash direction.
    pub fn find(&self, path: &str) -> Option<&FileEntry> {
        self.by_path
            .get(&normalize_path(path))
            .map(|&i| &self.files[i])
    }

    /// Reads and, if needed, decompresses one file.
    pub fn read(&self, entry: &FileEntry) -> Result<Vec<u8>> {
        let raw = self.read_range(u64::from(entry.offset), entry.stored_size as usize)?;
        let mut data: &[u8] = &raw;
        if self.header.version == 104 && self.header.has(archive_flags::EMBEDDED_FILE_NAMES) {
            let name_len = usize::from(
                *data
                    .first()
                    .ok_or_else(|| malformed_file(entry, "is empty"))?,
            );
            data = data
                .get(1 + name_len..)
                .ok_or_else(|| malformed_file(entry, "is shorter than its embedded name"))?;
        }
        if !entry.compressed {
            return Ok(data.to_vec());
        }
        if self.header.has(archive_flags::XMEM) {
            return Err(Error::Unsupported(
                "Xbox 360 XMem-compressed files can't be read".into(),
            ));
        }
        if data.len() < 4 {
            return Err(malformed_file(
                entry,
                "is too short to hold its uncompressed size",
            ));
        }
        let expected = u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize;
        let hint = expected.min(data.len().saturating_mul(64)).min(256 << 20);
        let out = inflate::zlib_decompress(&data[4..], Some(hint)).map_err(|reason| {
            Error::Decompress {
                path: entry.path.clone(),
                reason,
            }
        })?;
        if out.len() != expected {
            return Err(Error::Decompress {
                path: entry.path.clone(),
                reason: format!("expected {expected} bytes, got {}", out.len()),
            });
        }
        Ok(out)
    }

    /// Compares every stored name hash with the hash of the name it's paired
    /// with. All matching confirms the names were read in the right order.
    pub fn check_name_hashes(&self) -> HashCheck {
        HashCheck {
            folders_matched: self
                .folders
                .iter()
                .filter(|f| hash_folder_name(&f.name) == f.name_hash)
                .count(),
            folders_total: self.folders.len(),
            files_matched: self
                .files
                .iter()
                .filter(|f| hash_file_name(&f.name) == f.name_hash)
                .count(),
            files_total: self.files.len(),
        }
    }

    fn read_range(&self, offset: u64, len: usize) -> Result<Vec<u8>> {
        match &self.source {
            Source::Memory(bytes) => {
                let start = offset as usize;
                Ok(bytes[start..start + len].to_vec())
            }
            Source::File(file) => {
                let mut file = file.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
                read_exact_at(&mut file, offset, len)
            }
        }
    }
}

fn read_exact_at(file: &mut File, offset: u64, len: usize) -> Result<Vec<u8>> {
    file.seek(SeekFrom::Start(offset))?;
    let mut buf = vec![0u8; len];
    file.read_exact(&mut buf)?;
    Ok(buf)
}

fn malformed_file(entry: &FileEntry, problem: &str) -> Error {
    Error::Malformed {
        offset: u64::from(entry.offset),
        reason: format!("the data for {} {problem}", entry.path),
    }
}

fn parse_header(bytes: &[u8]) -> Result<Header> {
    let mut found = [0u8; 4];
    let n = bytes.len().min(4);
    found[..n].copy_from_slice(&bytes[..n]);
    if &found != MAGIC {
        return Err(Error::NotAnArchive { found });
    }
    if bytes.len() < HEADER_LEN {
        return Err(Error::Malformed {
            offset: 0,
            reason: format!("the file is shorter than the {HEADER_LEN}-byte header"),
        });
    }
    let u32_at =
        |i: usize| u32::from_le_bytes([bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]]);
    let version = u32_at(4);
    match version {
        103 | 104 => {}
        105 => {
            return Err(Error::Unsupported(
                "version 105 archives come from Skyrim Special Edition, not Fallout 3 or New Vegas"
                    .into(),
            ))
        }
        v => {
            return Err(Error::Unsupported(format!(
                "archive version {v} (New Vegas uses 104)"
            )))
        }
    }
    let folder_offset = u32_at(8);
    if folder_offset as usize != HEADER_LEN {
        return Err(Error::Malformed {
            offset: 8,
            reason: format!("folder records should start at {HEADER_LEN}, not {folder_offset}"),
        });
    }
    let header = Header {
        version,
        archive_flags: u32_at(12),
        folder_count: u32_at(16),
        file_count: u32_at(20),
        total_folder_name_length: u32_at(24),
        total_file_name_length: u32_at(28),
        content_flags: u32_at(32),
    };
    if header.has(archive_flags::XBOX) {
        return Err(Error::Unsupported("Xbox 360 (big-endian) archives".into()));
    }
    Ok(header)
}

fn checked_directory_len(header: &Header, file_len: u64) -> Result<usize> {
    let len = header.directory_len();
    if len > file_len {
        return Err(Error::Malformed {
            offset: 0,
            reason: format!(
                "the header describes a {len}-byte directory but the file is only {file_len} bytes"
            ),
        });
    }
    Ok(len as usize)
}

/// Little-endian reader over the directory bytes.
struct Reader<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    fn take(&mut self, n: usize, what: &str) -> Result<&'a [u8]> {
        let slice = self
            .buf
            .get(self.pos..self.pos + n)
            .ok_or_else(|| Error::Malformed {
                offset: self.pos as u64,
                reason: format!("the directory ends in the middle of {what}"),
            })?;
        self.pos += n;
        Ok(slice)
    }

    fn u8(&mut self, what: &str) -> Result<u8> {
        Ok(self.take(1, what)?[0])
    }

    fn u32(&mut self, what: &str) -> Result<u32> {
        let b = self.take(4, what)?;
        Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }

    fn u64(&mut self, what: &str) -> Result<u64> {
        let b = self.take(8, what)?;
        Ok(u64::from_le_bytes(b.try_into().expect("8 bytes")))
    }
}

/// Parses folder records, file record blocks and the name table.
fn parse_directory(
    header: &Header,
    directory: &[u8],
    file_len: u64,
) -> Result<(Vec<Folder>, Vec<FileEntry>)> {
    let mut r = Reader {
        buf: directory,
        pos: HEADER_LEN,
    };

    let mut folder_records = Vec::with_capacity(header.folder_count as usize);
    for _ in 0..header.folder_count {
        let hash = r.u64("a folder record")?;
        let count = r.u32("a folder record")?;
        let _offset = r.u32("a folder record")?;
        folder_records.push((hash, count));
    }

    let default_compressed = header.has(archive_flags::COMPRESSED);
    let mut folders = Vec::with_capacity(folder_records.len());
    let mut files: Vec<FileEntry> = Vec::with_capacity(header.file_count as usize);
    for (hash, count) in folder_records {
        let name = if header.has(archive_flags::DIRECTORY_NAMES) {
            let len = usize::from(r.u8("a folder name")?);
            crate::normalize_path(&text::zstring(r.take(len, "a folder name")?))
        } else {
            format!("{hash:016x}")
        };
        let first = files.len();
        for _ in 0..count {
            let name_hash = r.u64("a file record")?;
            let raw_size = r.u32("a file record")?;
            let offset = r.u32("a file record")?;
            let stored_size = raw_size & SIZE_MASK;
            if u64::from(offset) + u64::from(stored_size) > file_len {
                return Err(Error::Malformed {
                    offset: r.pos as u64 - FILE_RECORD_LEN,
                    reason: format!(
                        "a file in {name} claims {stored_size} bytes at offset {offset:#x}, \
                         past the end of the archive"
                    ),
                });
            }
            files.push(FileEntry {
                path: String::new(),
                name: format!("{name_hash:016x}"),
                folder: folders.len(),
                name_hash,
                offset,
                stored_size,
                compressed: default_compressed ^ (raw_size & COMPRESSION_TOGGLE != 0),
            });
        }
        folders.push(Folder {
            name,
            name_hash: hash,
            files: first..files.len(),
        });
    }
    if files.len() != header.file_count as usize {
        return Err(Error::Malformed {
            offset: 20,
            reason: format!(
                "the folders list {} files but the header says {}",
                files.len(),
                header.file_count
            ),
        });
    }

    if header.has(archive_flags::FILE_NAMES) {
        let table = r.take(
            header.total_file_name_length as usize,
            "the file name table",
        )?;
        let mut names = table.split(|&b| b == 0);
        for file in &mut files {
            let name = names
                .next()
                .filter(|n| !n.is_empty())
                .ok_or_else(|| Error::Malformed {
                    offset: r.pos as u64,
                    reason: format!(
                        "the file name table has fewer names than the {} files",
                        header.file_count
                    ),
                })?;
            file.name = crate::normalize_path(&text::decode_cp1252(name));
        }
    }
    for file in &mut files {
        let folder = &folders[file.folder].name;
        file.path = if folder.is_empty() || folder == "." {
            file.name.clone()
        } else {
            format!("{folder}\\{}", file.name)
        };
    }
    Ok((folders, files))
}
