//! The platform's file calls: `CreateFileA`, `GetFileType`, `SetFilePointer`, `WriteFile`,
//! `ReadFile` and `CloseHandle` on file handles (KERNEL32 imports, the CRT's file layer below
//! `fopen`/`fclose`/`fwrite` uses them: `__tsopen_nolock`, `_lseek`, `_write`, `_close`), with
//! the documented Win32 semantics.
//!
//! Also the file system around them: `GetFileAttributesA`, `CreateDirectoryA`, `DeleteFileA`,
//! `CopyFileA` and `SHGetFolderPathA`.
//!
//! Every file the game makes, changes, copies or deletes lives in [`Disk`], in engine-side
//! memory: nothing reaches the player's disk. The player's own files are read only: opened, they
//! are read into the run's copy (only when opened for writing; opened for reading they are read
//! in place); deleted, they are only gone for the game. An open of a file that exists nowhere
//! with a disposition that needs one fails as Windows answers it (measured).

use super::KernelObject;
use crate::{Engine, Ptr};
use std::collections::BTreeMap;

/// `GENERIC_READ` and `GENERIC_WRITE` (the access mask's top bits).
pub const GENERIC_READ: u32 = 0x8000_0000;
pub const GENERIC_WRITE: u32 = 0x4000_0000;
/// The creation dispositions (`CREATE_NEW` .. `TRUNCATE_EXISTING`).
pub const CREATE_NEW: u32 = 1;
pub const CREATE_ALWAYS: u32 = 2;
pub const OPEN_EXISTING: u32 = 3;
pub const OPEN_ALWAYS: u32 = 4;
pub const TRUNCATE_EXISTING: u32 = 5;
/// `INVALID_HANDLE_VALUE` and `INVALID_SET_FILE_POINTER`.
pub const INVALID_HANDLE_VALUE: u32 = 0xffff_ffff;
pub const INVALID_SET_FILE_POINTER: u32 = 0xffff_ffff;
/// `FILE_TYPE_UNKNOWN` and `FILE_TYPE_DISK`.
pub const FILE_TYPE_UNKNOWN: u32 = 0;
pub const FILE_TYPE_DISK: u32 = 1;
/// The error codes these calls set (winerror.h).
pub const ERROR_SUCCESS: u32 = 0;
pub const ERROR_FILE_NOT_FOUND: u32 = 2;
pub const ERROR_ACCESS_DENIED: u32 = 5;
pub const ERROR_INVALID_HANDLE: u32 = 6;
pub const ERROR_INVALID_PARAMETER: u32 = 87;
pub const ERROR_FILE_EXISTS: u32 = 80;
pub const ERROR_NEGATIVE_SEEK: u32 = 131;
pub const ERROR_ALREADY_EXISTS: u32 = 183;

/// The files the game made in this run, by path (ASCII lower case, `/` as `\`), and their bytes.
#[derive(Debug, Default)]
pub struct Disk {
    pub files: BTreeMap<Vec<u8>, Vec<u8>>,
    /// Files the game deleted in this run (the player's files stay where they are).
    pub deleted: std::collections::BTreeSet<Vec<u8>>,
    /// Directories the game made in this run (nothing is made on the player's disk).
    pub directories: std::collections::BTreeSet<Vec<u8>>,
    /// Files of the player's opened for reading only, by path as in `files`: read from the
    /// player's disk where a read asks (the archives are gigabytes; nothing is copied). Opened
    /// for writing, a file is read whole into `files` first, as before.
    pub player: BTreeMap<Vec<u8>, std::path::PathBuf>,
    /// The directory entries of the files in `files` as a search lists them ([`Entry`]), by the
    /// same keys. A file in `files` without one (made by `CopyFileA`, whose times were not
    /// measured) stops a search that would list it.
    pub entries: BTreeMap<Vec<u8>, Entry>,
    /// Names removed from a directory in this run, for the file system's tunnelling
    /// ([`Tunnelled`]), oldest first.
    pub tunnel: Vec<Tunnelled>,
    /// The clock file times are stamped with when set (tests); otherwise the real clock
    /// ([`file_clock`]).
    pub clock: Option<u64>,
}

/// A file's directory entry as `FindFirstFileA` describes it, for a file the game made or
/// changed in this run: what the file system keeps in the directory and publishes when a handle
/// that wrote is closed (measured: `build43-scratch/measure/findfile-changed/findfile-2026-11-03.txt`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    /// The name as the directory stores it: in the case of the call that made it or renamed it
    /// there.
    pub name: String,
    /// Its 8.3 short name, empty for a name that needs none ([`short_name`]); `None` where it
    /// could not be made as Windows makes it (writing it stops the run).
    pub short: Option<String>,
    /// `FILE_ATTRIBUTE_*`.
    pub attributes: u32,
    /// The size a search shows: the file's size when it was made, or when the last handle that
    /// wrote to it was closed (a search while it is open shows the size before).
    pub size: u64,
    /// The creation, last-access and last-write `FILETIME`s a search shows.
    pub created: u64,
    pub accessed: u64,
    pub written: u64,
}

/// A name removed from a directory in this run (deleted, or renamed away), as the file system's
/// tunnelling keeps it: a file made or renamed in under the same name in the same directory
/// within [`TUNNEL_TICKS`] takes its creation time and its short name (measured).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tunnelled {
    /// The name's key (as in [`Disk::files`]: the folder's path and the name).
    pub name: Vec<u8>,
    /// Its short name and creation time when it was removed.
    pub short: Option<String>,
    pub created: u64,
    /// When it was removed.
    pub at: u64,
}

/// How long a removed name tunnels: 15 seconds, Windows' default (measured on this machine:
/// made again 1 s and 14 s after the removal it took the old creation time, after 16 s and
/// 17 s not; `findfile-2026-11-03.txt`).
pub const TUNNEL_TICKS: u64 = 15 * 10_000_000;

/// The time the file system stamps a file with now: the clock `GetSystemTimeAsFileTime` reads
/// (measured: every time a search showed equalled that call's reading just before the call
/// that set it, not the finer `GetSystemTimePreciseAsFileTime`), or [`Disk::clock`] when a test
/// set one.
pub fn file_clock(e: &Engine) -> u64 {
    if let Some(now) = e.kernel.disk.clock {
        return now;
    }
    #[cfg(windows)]
    {
        extern "system" {
            fn GetSystemTimeAsFileTime(time: *mut u64);
        }
        let mut now = 0u64;
        // SAFETY: the call writes one FILETIME (8 bytes) to `now`.
        unsafe { GetSystemTimeAsFileTime(&mut now) };
        now
    }
    #[cfg(not(windows))]
    {
        let since = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("the system clock is before 1970");
        super::zone::UNIX_EPOCH_TICKS
            + since.as_secs() * super::zone::TICKS_PER_SECOND
            + u64::from(since.subsec_nanos() / 100)
    }
}

/// The size of the file at `k` (a [`Disk`] key): the run's copy, else the player's file opened
/// for reading only; 0 for neither.
fn file_size(e: &Engine, k: &[u8]) -> u64 {
    if let Some(data) = e.kernel.disk.files.get(k) {
        return data.len() as u64;
    }
    e.kernel
        .disk
        .player
        .get(k)
        .and_then(|p| std::fs::metadata(p).ok())
        .map_or(0, |m| m.len())
}

/// Up to `count` bytes of the file at `k` from `start` (fewer at the end, none past it): the
/// run's copy, else the player's file opened for reading only (read from disk).
fn read_range(e: &Engine, k: &[u8], start: u64, count: u32) -> Vec<u8> {
    if let Some(data) = e.kernel.disk.files.get(k) {
        let start = (start as usize).min(data.len());
        let end = start.saturating_add(count as usize).min(data.len());
        return data[start..end].to_vec();
    }
    let Some(path) = e.kernel.disk.player.get(k) else {
        return vec![];
    };
    use std::io::{Read, Seek, SeekFrom};
    let mut file = std::fs::File::open(path).unwrap_or_else(|err| {
        panic!("ReadFile: the player's file {path:?} can no longer be read: {err}")
    });
    let size = file.metadata().map_or(0, |m| m.len());
    let start = start.min(size);
    let take = u64::from(count).min(size - start);
    file.seek(SeekFrom::Start(start))
        .unwrap_or_else(|err| panic!("ReadFile: seeking in {path:?}: {err}"));
    let mut out = vec![0u8; take as usize];
    file.read_exact(&mut out)
        .unwrap_or_else(|err| panic!("ReadFile: reading {path:?}: {err}"));
    out
}

/// Files the game deleted in this run (the player's own files stay where they are) and the
/// directories it made (nothing is made on the player's disk) are in [`Disk`].
/// `FILE_ATTRIBUTE_DIRECTORY`, `FILE_ATTRIBUTE_ARCHIVE` and `INVALID_FILE_ATTRIBUTES`.
pub const FILE_ATTRIBUTE_DIRECTORY: u32 = 0x10;
pub const FILE_ATTRIBUTE_ARCHIVE: u32 = 0x20;
pub const INVALID_FILE_ATTRIBUTES: u32 = 0xffff_ffff;

/// Whether `name` is a full path (a drive letter and a separator, or `\\` for a network path).
pub fn is_full_path(name: &[u8]) -> bool {
    matches!(name, [d, b':', b'\\' | b'/', ..] if d.is_ascii_alphabetic())
        || name.starts_with(b"\\\\")
}

/// `name` as a full path: a relative name is in the current directory
/// (`Kernel::current_directory`).
pub fn full_path(e: &Engine, name: &[u8]) -> Vec<u8> {
    if is_full_path(name) {
        return name.to_vec();
    }
    let mut path = e.kernel.current_directory.clone();
    if !path.ends_with(b"\\") && !path.is_empty() {
        path.push(b'\\');
    }
    path.extend_from_slice(name.strip_prefix(b".\\").unwrap_or(name));
    path
}

fn real_path(path: &[u8]) -> std::path::PathBuf {
    std::path::PathBuf::from(String::from_utf8_lossy(path).into_owned())
}

/// The bytes of the file at the full path `path`: as the game made it in this run, else the
/// player's file (read only); `None` when the game deleted it or there is none.
pub fn read_whole(e: &Engine, path: &[u8]) -> Option<Vec<u8>> {
    let k = key(path);
    if let Some(data) = e.kernel.disk.files.get(&k) {
        return Some(data.clone());
    }
    if e.kernel.disk.deleted.contains(&k) {
        return None;
    }
    std::fs::read(real_path(path)).ok()
}

/// The attributes of the full path `path` as Windows reports them: a file or directory the game
/// made in this run, else the player's (`FILE_ATTRIBUTE_*` as the file system keeps them);
/// `None` when there is none.
fn attributes(e: &Engine, path: &[u8]) -> Option<u32> {
    let k = key(path);
    let trimmed = key(path.strip_suffix(b"\\").unwrap_or(path));
    if e.kernel.disk.files.contains_key(&k) {
        return Some(
            e.kernel
                .disk
                .entries
                .get(&k)
                .map_or(FILE_ATTRIBUTE_ARCHIVE, |entry| entry.attributes),
        );
    }
    if e.kernel.disk.directories.contains(&trimmed) {
        return Some(FILE_ATTRIBUTE_DIRECTORY);
    }
    if e.kernel.disk.deleted.contains(&k) {
        return None;
    }
    let meta = std::fs::metadata(real_path(path)).ok()?;
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        Some(meta.file_attributes())
    }
    #[cfg(not(windows))]
    {
        Some(if meta.is_dir() {
            FILE_ATTRIBUTE_DIRECTORY
        } else {
            FILE_ATTRIBUTE_ARCHIVE
        })
    }
}

// Platform: replaces KERNEL32.DLL GetFileAttributesA (import)
/// `GetFileAttributesA(name)`: the file's or directory's attributes ([`attributes`]), or
/// `INVALID_FILE_ATTRIBUTES` with `ERROR_FILE_NOT_FOUND`.
pub fn get_file_attributes_a(e: &mut Engine, name: Ptr) -> u32 {
    let path = full_path(e, &e.mem.cstr(name.addr()));
    match attributes(e, &path) {
        Some(a) => a,
        None => {
            e.kernel.last_error = ERROR_FILE_NOT_FOUND;
            INVALID_FILE_ATTRIBUTES
        }
    }
}

// Platform: replaces KERNEL32.DLL CreateDirectoryA (import)
/// `CreateDirectoryA(name, security)`: false with `ERROR_ALREADY_EXISTS` when a file or
/// directory of that name exists ([`attributes`]); otherwise the directory is recorded as made
/// (in engine memory: nothing is made on the player's disk) and true.
pub fn create_directory_a(e: &mut Engine, name: Ptr, _security: u32) -> u32 {
    let path = full_path(e, &e.mem.cstr(name.addr()));
    if attributes(e, &path).is_some() {
        e.kernel.last_error = ERROR_ALREADY_EXISTS;
        return 0;
    }
    let trimmed = key(path.strip_suffix(b"\\").unwrap_or(&path));
    e.kernel.disk.directories.insert(trimmed);
    e.kernel.last_error = ERROR_SUCCESS;
    1
}

// Platform: replaces KERNEL32.DLL DeleteFileA (import)
/// `DeleteFileA(name)`: a file the game made goes; a file of the player's is from then on gone
/// for the game (recorded, never deleted on disk); true. False with `ERROR_FILE_NOT_FOUND` when
/// there is none, `ERROR_ACCESS_DENIED` for a directory. The name's creation time and short name
/// are kept for the file system's tunnelling ([`Tunnelled`]).
pub fn delete_file_a(e: &mut Engine, name: Ptr) -> u32 {
    let path = full_path(e, &e.mem.cstr(name.addr()));
    match attributes(e, &path) {
        None => {
            e.kernel.last_error = ERROR_FILE_NOT_FOUND;
            0
        }
        Some(a) if a & FILE_ATTRIBUTE_DIRECTORY != 0 => {
            e.kernel.last_error = ERROR_ACCESS_DENIED;
            0
        }
        Some(_) => {
            let k = key(&path);
            let entry = entry_of(e, &path);
            e.kernel.disk.files.remove(&k);
            e.kernel.disk.entries.remove(&k);
            e.kernel.disk.player.remove(&k);
            e.kernel.disk.deleted.insert(k.clone());
            removed(e, &k, entry.as_ref());
            e.kernel.last_error = ERROR_SUCCESS;
            1
        }
    }
}

// Platform: replaces KERNEL32.DLL CopyFileA (import)
/// `CopyFileA(source, target, fail if exists)`: the source's bytes ([`read_whole`]) become the
/// target, a file of this run (engine memory, never on disk); false with `ERROR_FILE_NOT_FOUND`
/// without a source, `ERROR_FILE_EXISTS` when the target exists and `fail if exists` is set.
/// The times and attributes a copy gets were not measured: the target has no [`Entry`], so a
/// search that would list it stops the run.
pub fn copy_file_a(e: &mut Engine, source: Ptr, target: Ptr, fail_if_exists: u32) -> u32 {
    let from = full_path(e, &e.mem.cstr(source.addr()));
    let to = full_path(e, &e.mem.cstr(target.addr()));
    let Some(data) = read_whole(e, &from) else {
        e.kernel.last_error = ERROR_FILE_NOT_FOUND;
        return 0;
    };
    if fail_if_exists != 0 && attributes(e, &to).is_some() {
        e.kernel.last_error = ERROR_FILE_EXISTS;
        return 0;
    }
    let k = key(&to);
    e.kernel.disk.deleted.remove(&k);
    e.kernel.disk.entries.remove(&k);
    e.kernel.disk.files.insert(k, data);
    e.kernel.last_error = ERROR_SUCCESS;
    1
}

// Platform: replaces KERNEL32.DLL MoveFileA (import)
/// `MoveFileA(existing, new)` for files, as measured on scratch files
/// (`build43-scratch/measure/movefile-2026-10-23.txt`): no source gives false with
/// `ERROR_FILE_NOT_FOUND`; a target in a folder that does not exist `ERROR_PATH_NOT_FOUND`; a
/// target that exists `ERROR_ALREADY_EXISTS`, unless it is the source itself (the same name, in
/// any case: true, nothing changes here, the run's file names being kept in lower case).
/// Otherwise the source's bytes become the target (a file of this run, engine memory) and the
/// source is gone for the game (a player's file recorded as deleted, never touched); true, the
/// last error left as it was (the measured values after a success are the call's internal
/// leftovers). Moving a directory is not provided.
///
/// The target's [`Entry`] is the source's under the new name (in the call's case): size,
/// attributes, last access and last write kept; the creation time kept and a short name made for
/// the new name ([`short_name`]), unless the new name was removed from that folder within 15 s
/// (tunnelling: its creation time and short name; measured, `findfile-changed/
/// findfile-2026-11-03.txt` 2 and 3). The source's name is kept for tunnelling in turn.
pub fn move_file_a(e: &mut Engine, existing: Ptr, new: Ptr) -> u32 {
    let from = full_path(e, &e.mem.cstr(existing.addr()));
    let to = full_path(e, &e.mem.cstr(new.addr()));
    let Some(from_attributes) = attributes(e, &from) else {
        e.kernel.last_error = ERROR_FILE_NOT_FOUND;
        return 0;
    };
    if from_attributes & FILE_ATTRIBUTE_DIRECTORY != 0 {
        panic!("MoveFileA of a directory is not provided");
    }
    let (kf, kt) = (key(&from), key(&to));
    if kf == kt {
        return 1;
    }
    let parent = to
        .iter()
        .rposition(|&b| b == b'\\')
        .map(|i| to[..i].to_vec());
    if let Some(parent) = parent {
        let is_root = parent.len() == 2 && parent[1] == b':';
        if !is_root
            && !matches!(attributes(e, &parent), Some(a) if a & FILE_ATTRIBUTE_DIRECTORY != 0)
        {
            e.kernel.last_error = ERROR_PATH_NOT_FOUND;
            return 0;
        }
    }
    if attributes(e, &to).is_some() {
        e.kernel.last_error = ERROR_ALREADY_EXISTS;
        return 0;
    }
    let entry = entry_of(e, &from);
    let data = read_whole(e, &from).expect("the source file's bytes");
    e.kernel.disk.deleted.remove(&kt);
    e.kernel.disk.files.insert(kt.clone(), data);
    e.kernel.disk.files.remove(&kf);
    e.kernel.disk.entries.remove(&kf);
    e.kernel.disk.player.remove(&kf);
    e.kernel.disk.deleted.insert(kf.clone());
    removed(e, &kf, entry.as_ref());
    match entry {
        Some(entry) => {
            let (created, short) = match tunnelled(e, &kt) {
                Some(old) => (old.created, old.short),
                None => (entry.created, short_name(e, &to)),
            };
            let renamed = Entry {
                name: last_part(&to),
                short,
                created,
                ..entry
            };
            e.kernel.disk.entries.insert(kt, renamed);
        }
        None => {
            e.kernel.disk.entries.remove(&kt);
        }
    }
    1
}

// Platform: replaces SHELL32.DLL SHGetFolderPathA (import)
/// `SHGetFolderPathA(owner, folder, token, flags, path)`: the player's folder for that CSIDL as
/// the player's table recorded it (`d3d9::folder_path`), copied into `path`, with its result. A
/// folder the table doesn't have, or `CSIDL_FLAG_CREATE`, is not provided.
pub fn sh_get_folder_path_a(
    e: &mut Engine,
    _owner: u32,
    folder: u32,
    _token: u32,
    flags: u32,
    path: Ptr,
) -> u32 {
    assert!(
        folder & 0x8000 == 0,
        "SHGetFolderPathA with CSIDL_FLAG_CREATE is not provided"
    );
    let Some((hr, text)) = super::d3d9::folder_path(e, folder) else {
        panic!(
            "SHGetFolderPathA({folder:#x}, flags {flags:#x}) was not captured in the adapter table: not provided"
        );
    };
    e.mem.set_cstr(path.addr(), &text);
    hr
}

/// An open file handle's state ([`KernelObject::File`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenFile {
    pub path: Vec<u8>,
    pub position: u64,
    pub read: bool,
    pub write: bool,
    /// The time of this handle's last `WriteFile` ([`file_clock`]), published to the file's
    /// [`Entry`] when the handle is closed ([`closed`]); `None` while it wrote nothing.
    pub wrote: Option<u64>,
}

fn key(path: &[u8]) -> Vec<u8> {
    path.iter()
        .map(|&b| {
            if b == b'/' {
                b'\\'
            } else {
                b.to_ascii_lowercase()
            }
        })
        .collect()
}

fn open_file(e: &mut Engine, handle: u32) -> Option<&mut OpenFile> {
    match e.kernel.objects.get_mut(&handle) {
        Some(KernelObject::File(file)) => Some(file),
        _ => None,
    }
}

/// `ERROR_PATH_NOT_FOUND`.
pub const ERROR_PATH_NOT_FOUND: u32 = 3;

/// Whether the directory holding the full path `path` exists: one the game made in this run,
/// else the player's (and not deleted).
fn directory_exists(e: &Engine, path: &[u8]) -> bool {
    let trimmed = path.strip_suffix(b"\\").unwrap_or(path);
    let Some(cut) = trimmed.iter().rposition(|&b| b == b'\\' || b == b'/') else {
        return false;
    };
    let parent = &trimmed[..cut];
    let k = key(parent);
    if e.kernel.disk.directories.contains(&k) {
        return true;
    }
    if e.kernel.disk.deleted.contains(&k) {
        return false;
    }
    std::fs::metadata(real_path(parent)).is_ok_and(|m| m.is_dir())
}

// Platform: replaces KERNEL32.DLL FindFirstFileA (import)
/// `FindFirstFileA(name, data)` (a name with wildcards: [`find_first_wild`]): the player's file or directory,
/// described as Windows does in the `WIN32_FIND_DATAA` at `data` (0x140 bytes): attributes,
/// creation, last-access and last-write `FILETIME`s (UTC, as the file system keeps them), the
/// size (high word, then low), `dwReserved0`/`dwReserved1` 0, and `cFileName` the name as the
/// directory stores it (its case), followed by one zero byte; the rest of that buffer is left as
/// it was. A search handle is answered and the last error is left unchanged. Nothing found
/// answers `INVALID_HANDLE_VALUE` with `ERROR_FILE_NOT_FOUND` (2), or `ERROR_PATH_NOT_FOUND`
/// (3) when the directory is missing too, leaving `data` as it was; so does a directory's name
/// with a trailing separator.
///
/// Measured on this machine (2026-10-21, PowerShell P/Invoke of `FindFirstFileA` on
/// `...\Data\Fallout - Meshes.bsa`, the same in other case, a missing file, a file in a missing
/// directory, `...\Data` and `...\Data\`, with the buffer filled with 0xAA first): as above;
/// the last error stayed what it was (203) on success. Not provided here: `cAlternateFileName`
/// of the player's entries (Windows wrote the 8.3 name `FALLOU~1.BSA` for the archive, nothing
/// for `Data`) is left empty (one zero byte); directories the game made in this run, a
/// trailing separator on a missing name, and non-ASCII names stop the run.
///
/// A file the game made or changed in this run is described by its [`Entry`] (the name in the
/// case it was made or renamed in, its short name, attributes, size and times; [`write_entry`]),
/// as a search of its folder lists it (`findfile-changed/findfile-2026-11-03.txt`): the save
/// list reads a new save's last-write time this way (`00846a20`).
pub fn find_first_file_a(e: &mut Engine, name: Ptr, data: Ptr) -> u32 {
    let text = e.mem.cstr(name.addr());
    if text.iter().any(|&b| b == b'*' || b == b'?') {
        return find_first_wild(e, &text, data);
    }
    let path = full_path(e, &text);
    let k = key(&path);
    let shown = String::from_utf8_lossy(&path).into_owned();
    if e.kernel.disk.files.contains_key(&k) {
        let entry = run_entry(e, &k, &shown);
        write_entry(e, data, &entry, &shown);
        return e.kernel.insert(KernelObject::FindFile(Search::default()));
    }
    if e.kernel.disk.directories.contains(&k) {
        panic!(
            "FindFirstFileA(\"{shown}\"): a directory the game made in this run is not provided"
        );
    }
    let fail = |e: &mut Engine, code: u32| {
        e.kernel.last_error = code;
        INVALID_HANDLE_VALUE
    };
    if e.kernel.disk.deleted.contains(&k) {
        return fail(e, ERROR_FILE_NOT_FOUND);
    }
    let real = real_path(&path);
    let trailing = path.ends_with(b"\\") || path.ends_with(b"/");
    let meta = match std::fs::metadata(&real) {
        Ok(meta) if trailing && meta.is_dir() => return fail(e, ERROR_FILE_NOT_FOUND),
        Ok(_) if trailing => {
            panic!("FindFirstFileA(\"{shown}\"): a trailing separator is not provided")
        }
        Ok(meta) => meta,
        Err(_) if trailing => {
            panic!("FindFirstFileA(\"{shown}\"): a trailing separator is not provided")
        }
        Err(_) => {
            let parent_exists = real.parent().is_some_and(|p| p.is_dir());
            return fail(
                e,
                if parent_exists {
                    ERROR_FILE_NOT_FOUND
                } else {
                    ERROR_PATH_NOT_FOUND
                },
            );
        }
    };
    let wanted = real
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    let stored = real
        .parent()
        .and_then(|p| std::fs::read_dir(p).ok())
        .and_then(|entries| {
            entries
                .filter_map(Result::ok)
                .map(|entry| entry.file_name().to_string_lossy().into_owned())
                .find(|n| n.eq_ignore_ascii_case(&wanted))
        })
        .unwrap_or(wanted);
    assert!(
        stored.is_ascii(),
        "FindFirstFileA(\"{shown}\"): a non-ASCII name is not provided"
    );
    write_find_data(e, data, &meta, &stored, &shown);
    e.kernel.insert(KernelObject::FindFile(Search::default()))
}

/// Writes the `WIN32_FIND_DATAA` at `data` for an entry: attributes, creation, last-access and
/// last-write times, the size (high word, then low), the two reserved words 0, `cFileName` (the
/// name as stored, then one zero byte) and an empty `cAlternateFileName` (one zero byte at
/// +0x130); the rest of the buffer is left as it was.
fn write_find_data(e: &mut Engine, data: Ptr, meta: &std::fs::Metadata, stored: &str, shown: &str) {
    #[cfg(windows)]
    let _ = shown;
    #[cfg(windows)]
    let (attributes, created, accessed, written) = {
        use std::os::windows::fs::MetadataExt;
        (
            meta.file_attributes(),
            meta.creation_time(),
            meta.last_access_time(),
            meta.last_write_time(),
        )
    };
    #[cfg(not(windows))]
    let (attributes, created, accessed, written): (u32, u64, u64, u64) = {
        let _ = &meta;
        panic!("FindFirstFileA(\"{shown}\"): file times are only provided on Windows");
    };
    let d = data.addr();
    e.mem.set_u32(d, attributes);
    for (offset, time) in [(4u32, created), (0xc, accessed), (0x14, written)] {
        e.mem.set_u32(d + offset, time as u32);
        e.mem.set_u32(d + offset + 4, (time >> 32) as u32);
    }
    let size = meta.len();
    e.mem.set_u32(d + 0x1c, (size >> 32) as u32);
    e.mem.set_u32(d + 0x20, size as u32);
    e.mem.set_u32(d + 0x24, 0);
    e.mem.set_u32(d + 0x28, 0);
    e.mem.write(d + 0x2c, stored.as_bytes());
    e.mem.set_u8(d + 0x2c + stored.len() as u32, 0);
    e.mem.set_u8(d + 0x130, 0);
}

/// Writes the `WIN32_FIND_DATAA` at `data` for an entry of this run as [`write_find_data`] writes
/// one, with its short name in `cAlternateFileName` (+0x130) and a zero byte; the rest of the
/// buffer is left as it was (measured: the bytes after the name's terminator stayed 0xAA,
/// `findfile-changed/findfile-2026-11-03.txt` 8). A short name that could not be made as Windows
/// makes it ([`short_name`]) stops the run.
fn write_entry(e: &mut Engine, data: Ptr, entry: &Entry, shown: &str) {
    let Some(short) = &entry.short else {
        panic!(
            "FindFirstFileA(\"{shown}\"): the 8.3 short name of \"{}\", made in this run, is not \
             provided",
            entry.name
        );
    };
    let d = data.addr();
    e.mem.set_u32(d, entry.attributes);
    for (offset, time) in [
        (4u32, entry.created),
        (0xc, entry.accessed),
        (0x14, entry.written),
    ] {
        e.mem.set_u32(d + offset, time as u32);
        e.mem.set_u32(d + offset + 4, (time >> 32) as u32);
    }
    e.mem.set_u32(d + 0x1c, (entry.size >> 32) as u32);
    e.mem.set_u32(d + 0x20, entry.size as u32);
    e.mem.set_u32(d + 0x24, 0);
    e.mem.set_u32(d + 0x28, 0);
    e.mem.write(d + 0x2c, entry.name.as_bytes());
    e.mem.set_u8(d + 0x2c + entry.name.len() as u32, 0);
    e.mem.write(d + 0x130, short.as_bytes());
    e.mem.set_u8(d + 0x130 + short.len() as u32, 0);
}

/// `ERROR_NO_MORE_FILES`.
pub const ERROR_NO_MORE_FILES: u32 = 18;

/// What a search handle has left to answer ([`KernelObject::FindFile`]): the entries after the
/// one answered last, in the order Windows answers them ([`Found`]).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Search {
    pub rest: std::collections::VecDeque<Found>,
    /// A search of a folder with a wildcard: the folder's [`Disk`] key and the game's own
    /// changes in it when the search began ([`folder_state`]). `FindNextFileA` stops the run
    /// when they changed since: Windows then answers some later entries from the folder as it
    /// is now and some from what it read before (measured 2026-11-03 on folders of 5 and 202
    /// files changed after the first and after the third entry,
    /// `build43-scratch/measure/review-findfile/run1.txt`, `run2.txt`), not modelled. No caller
    /// read changes the folder inside its loop (`00851330`, `00851680`, `0085f900`).
    pub folder: Option<(Vec<u8>, FolderState)>,
}

/// The game's own changes in one folder as a search can see them: each file of the run with
/// its [`Entry`] (0), each file deleted (1), each directory made (2), by key.
pub type FolderState = Vec<(u8, Vec<u8>, Option<Entry>)>;

/// The [`FolderState`] of the folder with key `folder`.
fn folder_state(disk: &Disk, folder: &[u8]) -> FolderState {
    let files = disk.files.keys().filter(|k| in_folder(k, folder));
    let deleted = disk.deleted.iter().filter(|k| in_folder(k, folder));
    let made = disk.directories.iter().filter(|k| in_folder(k, folder));
    files
        .map(|k| (0, k.clone(), disk.entries.get(k).cloned()))
        .chain(deleted.map(|k| (1, k.clone(), None)))
        .chain(made.map(|k| (2, k.clone(), None)))
        .collect()
}

/// Whether the [`Disk`] key `k` is directly in the folder with key `folder`.
fn in_folder(k: &[u8], folder: &[u8]) -> bool {
    k.len() > folder.len() + 1
        && k.starts_with(folder)
        && k[folder.len()] == b'\\'
        && !k[folder.len() + 1..].contains(&b'\\')
}

/// The keys of the [`Disk`] records directly in the folder with key `folder` (`below`: at any
/// depth) that the game made, changed or deleted in this run.
fn changed_keys<'a>(disk: &'a Disk, folder: &'a [u8], below: bool) -> Vec<&'a Vec<u8>> {
    let inside = move |k: &&Vec<u8>| {
        if below {
            k.len() > folder.len() + 1 && k.starts_with(folder) && k[folder.len()] == b'\\'
        } else {
            in_folder(k, folder)
        }
    };
    disk.files
        .keys()
        .filter(inside)
        .chain(disk.deleted.iter().filter(inside))
        .chain(disk.directories.iter().filter(inside))
        .collect()
}

/// The last part of the full path `path` (after its last separator), as text (ASCII; another
/// character stops the run).
fn last_part(path: &[u8]) -> String {
    let at = path
        .iter()
        .rposition(|&b| b == b'\\' || b == b'/')
        .map_or(0, |i| i + 1);
    let name = String::from_utf8_lossy(&path[at..]).into_owned();
    assert!(
        name.is_ascii(),
        "the non-ASCII file name \"{name}\" is not provided"
    );
    name
}

/// The [`Entry`] of the file at key `k`, made or changed in this run; a file of the run without
/// one (`CopyFileA`'s copies: not measured) stops the run.
fn run_entry(e: &Engine, k: &[u8], shown: &str) -> Entry {
    e.kernel.disk.entries.get(k).cloned().unwrap_or_else(|| {
        panic!(
            "FindFirstFileA(\"{shown}\"): {}, a file of this run whose directory entry was not \
             recorded (CopyFileA's times were not measured), is not provided",
            String::from_utf8_lossy(k)
        )
    })
}

/// The directory entry of the file at the full path `path` now: the run's ([`Disk::entries`]),
/// else the player's as the disk keeps it ([`player_entry`]); `None` for a file of the run
/// without one, or a player's file elsewhere than on Windows (times not provided there).
fn entry_of(e: &Engine, path: &[u8]) -> Option<Entry> {
    let k = key(path);
    if e.kernel.disk.files.contains_key(&k) {
        return e.kernel.disk.entries.get(&k).cloned();
    }
    player_entry(path)
}

/// The name at key `k` removed from its folder (deleted or renamed away) with the entry it had:
/// kept [`TUNNEL_TICKS`] for tunnelling ([`tunnelled`]); older records go.
fn removed(e: &mut Engine, k: &[u8], entry: Option<&Entry>) {
    let now = file_clock(e);
    let tunnel = &mut e.kernel.disk.tunnel;
    tunnel.retain(|t| now.saturating_sub(t.at) <= TUNNEL_TICKS && t.name != k);
    let Some(entry) = entry else {
        return;
    };
    tunnel.push(Tunnelled {
        name: k.to_vec(),
        short: entry.short.clone(),
        created: entry.created,
        at: now,
    });
}

/// The name at key `k` made, or renamed to, in its folder now: the record of the same name
/// removed from that folder within [`TUNNEL_TICKS`], taken out of the tunnel (measured: the new
/// file takes its creation time and short name, `findfile-changed/findfile-2026-11-03.txt` 3).
/// Only the long name is matched (a new name equal to a removed short name was not measured).
fn tunnelled(e: &mut Engine, k: &[u8]) -> Option<Tunnelled> {
    let now = file_clock(e);
    let tunnel = &mut e.kernel.disk.tunnel;
    let at = tunnel
        .iter()
        .rposition(|t| t.name == k && now.saturating_sub(t.at) <= TUNNEL_TICKS)?;
    Some(tunnel.remove(at))
}

/// Whether `name` matches the wildcard `pattern` as Windows' file search matches a long name
/// (both compared without case): `*` any run of characters, `?` one character or, at the end
/// of the name or before a dot, none (`*.ES?` finds `e.es`, measured).
fn wild_match(pattern: &[u8], name: &[u8]) -> bool {
    match (pattern.first(), name.first()) {
        (None, None) => true,
        (Some(b'*'), _) => {
            wild_match(&pattern[1..], name) || (!name.is_empty() && wild_match(pattern, &name[1..]))
        }
        (Some(b'?'), None) => wild_match(&pattern[1..], name),
        (Some(b'?'), Some(b'.')) => {
            wild_match(&pattern[1..], name) || wild_match(&pattern[1..], &name[1..])
        }
        (Some(b'?'), Some(_)) => wild_match(&pattern[1..], &name[1..]),
        (Some(p), Some(n)) => p.eq_ignore_ascii_case(n) && wild_match(&pattern[1..], &name[1..]),
        _ => false,
    }
}

/// Whether `name` needs an 8.3 short name (more than 8 characters before the extension or 3
/// after it, more than one dot, or a character a short name cannot hold) and, when it does,
/// the extension of that short name: the first three characters of its last extension, upper
/// case, with spaces left out (`d.esmx` is `D6104~1.ESM`, measured). The rest of a short name is
/// a hash Windows makes (for a name made in this run: [`short_name`]).
fn short_extension(name: &str) -> Option<String> {
    let dots = name.matches('.').count();
    let (base, extension) = match name.rfind('.') {
        Some(at) => (&name[..at], &name[at + 1..]),
        None => (name, ""),
    };
    let unusual = |c: char| c == ' ' || "+,;=[]".contains(c) || !c.is_ascii();
    let needs = base.len() > 8 || extension.len() > 3 || dots > 1 || name.chars().any(unusual);
    if !needs {
        return None;
    }
    Some(
        extension
            .chars()
            .filter(|&c| c != ' ')
            .take(3)
            .collect::<String>()
            .to_ascii_uppercase(),
    )
}

/// Whether a search pattern finds `name`: its long name, or its 8.3 short name where the parts
/// of it that are known decide (measured: `*.esm` finds `d.esmx` through `D6104~1.ESM`, not
/// `c.esm.bak`; `?.esm` does not find `a b.esm`, `ABE286~1.ESM`). A short name is its
/// extension ([`short_extension`]), then a base of at least three characters (`X~1`) whose
/// first character is the long name's first one other than a space or a dot, upper case (the
/// rest may be a hash Windows makes: `D6104~1`). A pattern the known parts cannot decide stops the
/// run as not provided.
fn search_finds(pattern: &[u8], name: &str, shown: &str) -> bool {
    if wild_match(pattern, name.as_bytes()) {
        return true;
    }
    if name == "." || name == ".." {
        return false;
    }
    let Some(extension) = short_extension(name) else {
        return false;
    };
    let (base_pattern, extension_pattern): (&[u8], &[u8]) =
        match pattern.iter().rposition(|&b| b == b'.') {
            Some(at) => (&pattern[..at], &pattern[at + 1..]),
            None => (pattern, b""),
        };
    if !wild_match(extension_pattern, extension.as_bytes()) {
        return false;
    }
    if base_pattern == b"*" {
        return true;
    }
    let known: Vec<u8> = name
        .bytes()
        .filter(|&b| b != b' ' && b != b'.')
        .take(1)
        .map(|b| b.to_ascii_uppercase())
        .collect();
    let literal: Vec<u8> = base_pattern
        .iter()
        .take_while(|&&b| b != b'*' && b != b'?')
        .map(|b| b.to_ascii_uppercase())
        .collect();
    if literal.iter().zip(&known).any(|(a, b)| a != b) {
        return false;
    }
    if !base_pattern.contains(&b'*') && base_pattern.len() < 3 {
        return false;
    }
    panic!("FindFirstFileA(\"{shown}\"): matching the short name of \"{name}\" is not provided")
}

/// `FindFirstFileA` with wildcards in the last part of the name: every entry of the directory
/// the pattern finds, in Windows' order, the first written as [`find_first_file_a`] writes one
/// with its short name; the last error stays as it was (measured). Nothing found:
/// `INVALID_HANDLE_VALUE` with `ERROR_FILE_NOT_FOUND`, `ERROR_PATH_NOT_FOUND` for a missing
/// directory. Wildcards before the last part and non-ASCII names stop the run.
///
/// The entries are the directory as Windows would list it after the game's own calls in this
/// run (measured on scratch files, `build43-scratch/measure/findfile-changed/
/// findfile-2026-11-03.txt`): the player's entries ([`listing`]; `.` and `..` too, directories
/// as well as files) less the files the game deleted, renamed away or took into the run, and
/// the files of the run in that directory with their [`Entry`] (as made, written, renamed,
/// emptied), matched against their long and short names with the measured matcher
/// ([`wild_match`]), all in upper-case byte order with `.` and `..` first (5). Not provided (the
/// run stops): `.` of a directory the game changed, `..` of one whose parent it changed (their
/// times move with the changes, 7), the entry of a subdirectory whose contents it changed (not
/// measured), a directory the game made in this run, a file of the run without an [`Entry`].
fn find_first_wild(e: &mut Engine, text: &[u8], data: Ptr) -> u32 {
    let path = full_path(e, text);
    let shown = String::from_utf8_lossy(&path).into_owned();
    let cut = path
        .iter()
        .rposition(|&b| b == b'\\' || b == b'/')
        .unwrap_or_else(|| panic!("FindFirstFileA(\"{shown}\"): a name without a directory"));
    let (directory, pattern) = (&path[..cut], &path[cut + 1..]);
    assert!(
        !directory.iter().any(|&b| b == b'*' || b == b'?'),
        "FindFirstFileA(\"{shown}\"): wildcards before the last part are not provided"
    );
    let folder = key(directory);
    let real = real_path(directory);
    // A directory the game made in this run is not on the disk: only its own entries.
    let made_here = e.kernel.disk.directories.contains(&folder);
    let players = match listing(&real, pattern, &shown) {
        Ok(found) => found,
        Err(_) if made_here => vec![],
        Err(ERROR_FILE_NOT_FOUND) => vec![],
        Err(code) => {
            e.kernel.last_error = code;
            return INVALID_HANDLE_VALUE;
        }
    };
    let disk = &e.kernel.disk;
    let changed_here = !changed_keys(disk, &folder, false).is_empty();
    let parent = folder[..folder.iter().rposition(|&b| b == b'\\').unwrap_or(0)].to_vec();
    let mut found: Vec<Found> = Vec::new();
    for item in players {
        let Found::Disk {
            name, directory, ..
        } = &item
        else {
            unreachable!("a listing answers the player's entries")
        };
        let k = key(&[folder.as_slice(), b"\\", name.as_bytes()].concat());
        let why = match name.as_str() {
            "." if changed_here => {
                Some("\".\" of a directory the game changed in this run (its times)".to_string())
            }
            ".." if !changed_keys(disk, &parent, false).is_empty() => Some(
                "\"..\" of a directory whose parent the game changed in this run (its times)"
                    .to_string(),
            ),
            "." | ".." => None,
            _ if disk.deleted.contains(&k) || disk.files.contains_key(&k) => continue,
            _ if *directory && !changed_keys(disk, &k, true).is_empty() => Some(format!(
                "the entry of \"{name}\", a directory whose contents the game changed in this run"
            )),
            _ => None,
        };
        if let Some(why) = why {
            panic!("FindFirstFileA(\"{shown}\"): {why} is not provided");
        }
        found.push(item);
    }
    if made_here && (search_finds(pattern, ".", &shown) || search_finds(pattern, "..", &shown)) {
        panic!(
            "FindFirstFileA(\"{shown}\"): \".\" and \"..\" of a directory the game made in this \
             run are not provided"
        );
    }
    for k in disk.directories.iter().filter(|k| in_folder(k, &folder)) {
        let name = last_part(k);
        if search_finds(pattern, &name, &shown) {
            panic!(
                "FindFirstFileA(\"{shown}\"): \"{name}\", a directory the game made in this run, \
                 is not provided"
            );
        }
    }
    for k in disk.files.keys().filter(|k| in_folder(k, &folder)) {
        let entry = run_entry(e, k, &shown);
        let finds = match &entry.short {
            Some(short) => {
                wild_match(pattern, entry.name.as_bytes())
                    || (!short.is_empty() && wild_match(pattern, short.as_bytes()))
            }
            None => search_finds(pattern, &entry.name, &shown),
        };
        if finds {
            found.push(Found::Run(entry));
        }
    }
    found.sort_by_cached_key(|item| {
        let name = item.name();
        let rank = match name {
            "." => 0,
            ".." => 1,
            _ => 2,
        };
        (rank, name.to_ascii_uppercase())
    });
    let mut rest: std::collections::VecDeque<_> = found.into();
    let Some(first) = rest.pop_front() else {
        e.kernel.last_error = ERROR_FILE_NOT_FOUND;
        return INVALID_HANDLE_VALUE;
    };
    write_found(e, data, &first, &shown);
    let seen = folder_state(&e.kernel.disk, &folder);
    e.kernel.insert(KernelObject::FindFile(Search {
        rest,
        folder: Some((folder, seen)),
    }))
}

/// One entry a search found.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Found {
    /// An entry of the player's directory: its full path, its name and its 8.3 short name as
    /// the directory stores them (the short name empty when it has none), and whether it is a
    /// directory; described from the disk when it is answered.
    Disk {
        path: std::path::PathBuf,
        name: String,
        short: String,
        directory: bool,
    },
    /// A file of this run, as its [`Entry`] was when the search began.
    Run(Entry),
}

impl Found {
    /// The entry's name as the directory stores it.
    pub fn name(&self) -> &str {
        match self {
            Found::Disk { name, .. } => name,
            Found::Run(entry) => &entry.name,
        }
    }
}

/// Writes a found entry as `FindFirstFileA` does, `cAlternateFileName` (+0x130, 14 bytes) its
/// short name and a zero byte.
fn write_found(e: &mut Engine, data: Ptr, found: &Found, shown: &str) {
    match found {
        Found::Disk {
            path, name, short, ..
        } => {
            let meta = std::fs::metadata(path).expect("the found entry's metadata");
            write_find_data(e, data, &meta, name, shown);
            let alternate = data.addr() + 0x130;
            e.mem.write(alternate, short.as_bytes());
            e.mem.set_u8(alternate + short.len() as u32, 0);
        }
        Found::Run(entry) => write_entry(e, data, entry, shown),
    }
}

/// One entry of the host's own search of a directory on Windows (`WIN32_FIND_DATAW`).
#[cfg(windows)]
struct HostEntry {
    name: String,
    short: String,
    attributes: u32,
    size: u64,
    created: u64,
    accessed: u64,
    written: u64,
}

/// The host's own `FindFirstFileW` / `FindNextFileW` of `query` (a directory and a pattern, or
/// a full name), read only: every entry in the order they answer, or the error of a search that
/// finds nothing. It opens nothing for writing and changes no attribute (a search handle only
/// lists, as `std::fs::read_dir` does).
#[cfg(windows)]
fn host_search(query: &std::path::Path, shown: &str) -> Result<Vec<HostEntry>, u32> {
    use std::os::windows::ffi::{OsStrExt, OsStringExt};
    #[repr(C)]
    struct FindDataW {
        attributes: u32,
        times: [u32; 6],
        size: [u32; 2],
        reserved: [u32; 2],
        name: [u16; 260],
        alternate: [u16; 14],
    }
    extern "system" {
        fn FindFirstFileW(name: *const u16, data: *mut FindDataW) -> isize;
        fn FindNextFileW(handle: isize, data: *mut FindDataW) -> i32;
        fn FindClose(handle: isize) -> i32;
    }
    let text = |units: &[u16]| {
        let end = units.iter().position(|&u| u == 0).unwrap_or(units.len());
        let name = std::ffi::OsString::from_wide(&units[..end])
            .to_string_lossy()
            .into_owned();
        assert!(
            name.is_ascii(),
            "FindFirstFileA(\"{shown}\"): a non-ASCII name is not provided"
        );
        name
    };
    let mut wide: Vec<u16> = query.as_os_str().encode_wide().collect();
    wide.push(0);
    // SAFETY: `wide` is a terminated wide string and `data` a buffer of the documented
    // `WIN32_FIND_DATAW` layout; the handle is closed before returning.
    let mut data: FindDataW = unsafe { std::mem::zeroed() };
    let handle = unsafe { FindFirstFileW(wide.as_ptr(), &mut data) };
    if handle == -1 {
        return Err(std::io::Error::last_os_error().raw_os_error().unwrap_or(2) as u32);
    }
    let time = |t: &[u32]| u64::from(t[0]) | (u64::from(t[1]) << 32);
    let mut found = Vec::new();
    loop {
        found.push(HostEntry {
            name: text(&data.name),
            short: text(&data.alternate),
            attributes: data.attributes,
            size: (u64::from(data.size[0]) << 32) | u64::from(data.size[1]),
            created: time(&data.times[0..2]),
            accessed: time(&data.times[2..4]),
            written: time(&data.times[4..6]),
        });
        if unsafe { FindNextFileW(handle, &mut data) } == 0 {
            break;
        }
    }
    unsafe { FindClose(handle) };
    Ok(found)
}

/// The entries of `directory` that `pattern` finds, in the order Windows answers them, or the
/// error code of a search that finds nothing. On Windows the host's own search
/// ([`host_search`]) answers, read only on the player's directory: the matching against long and
/// 8.3 short names and the order are the file system's (measured on scratch files,
/// `build43-scratch/measure/findfile-2026-10-23.txt`: upper-case byte order, `.` and `..`, `?`
/// matching nothing before a dot or at the end, `*.esm` finding `d.esmx` by `D6104~1.ESM`).
/// Other systems run the measured matcher ([`search_finds`], the other `listing`), which stops
/// on short names it cannot decide.
#[cfg(windows)]
fn listing(directory: &std::path::Path, pattern: &[u8], shown: &str) -> Result<Vec<Found>, u32> {
    let query = directory.join(String::from_utf8_lossy(pattern).as_ref());
    Ok(host_search(&query, shown)?
        .into_iter()
        .map(|h| Found::Disk {
            path: directory.join(&h.name),
            directory: h.attributes & FILE_ATTRIBUTE_DIRECTORY != 0,
            name: h.name,
            short: h.short,
        })
        .collect())
}

/// The entries of `directory` that `pattern` finds, elsewhere than on Windows: matched with
/// [`search_finds`] and ordered by upper-case name as measured; no short names.
#[cfg(not(windows))]
fn listing(directory: &std::path::Path, pattern: &[u8], shown: &str) -> Result<Vec<Found>, u32> {
    let Ok(entries) = std::fs::read_dir(directory) else {
        return Err(if directory.exists() {
            ERROR_FILE_NOT_FOUND
        } else {
            ERROR_PATH_NOT_FOUND
        });
    };
    let mut found: Vec<Found> = Vec::new();
    for special in [".", ".."] {
        if search_finds(pattern, special, shown) {
            found.push(Found::Disk {
                path: directory.join(special),
                name: special.to_string(),
                short: String::new(),
                directory: true,
            });
        }
    }
    for entry in entries.filter_map(Result::ok) {
        let name = entry.file_name().to_string_lossy().into_owned();
        assert!(
            name.is_ascii(),
            "FindFirstFileA(\"{shown}\"): a non-ASCII name is not provided"
        );
        if search_finds(pattern, &name, shown) {
            found.push(Found::Disk {
                path: entry.path(),
                directory: entry.file_type().is_ok_and(|t| t.is_dir()),
                name,
                short: String::new(),
            });
        }
    }
    found.sort_by(|a, b| {
        a.name()
            .to_ascii_uppercase()
            .cmp(&b.name().to_ascii_uppercase())
    });
    Ok(found)
}

/// The directory entry of the player's file at the full path `path` as the disk keeps it (its
/// name as stored, short name, attributes, size and times: the host's own search of that name,
/// [`host_search`]), for a file the game takes into the run, renames or deletes; `None` when
/// there is none, and elsewhere than on Windows (file times are only provided there).
fn player_entry(path: &[u8]) -> Option<Entry> {
    #[cfg(windows)]
    {
        let shown = String::from_utf8_lossy(path).into_owned();
        let found = host_search(&real_path(path), &shown).ok()?;
        let h = found.into_iter().next()?;
        Some(Entry {
            name: h.name,
            short: Some(h.short),
            attributes: h.attributes,
            size: h.size,
            created: h.created,
            accessed: h.accessed,
            written: h.written,
        })
    }
    #[cfg(not(windows))]
    {
        let _ = path;
        None
    }
}

/// The 8.3 short name the file system gives the name of the full path `path` when it enters its
/// directory now (not tunnelled): empty for a name that needs none ([`short_extension`]: in any
/// case, `nvfix.fos` none); otherwise the first of the names ntdll's `RtlGenerate8dot3Name` makes
/// for it (one context, asked again) that is not already a long or short name in the directory
/// (the player's entries the game did not delete, take or rename away, and the run's): `NAME~1`
/// to `~4`, then the checksum names. Measured (`findfile-changed/findfile-2026-11-03.txt` 4,
/// `gen-2026-11-03.tsv`): ntdll's names were NTFS's for all 25 long names measured with an
/// extension of three characters or fewer; for a longer extension they differ (`d.esmx`: ntdll
/// `D6104~1.ES~`, NTFS `D6104~1.ESM`), so such a name's short name is `None` (not provided), as
/// it is elsewhere than on Windows.
fn short_name(e: &Engine, path: &[u8]) -> Option<String> {
    let name = last_part(path);
    if short_extension(&name).is_none() {
        return Some(String::new());
    }
    let extension = name.rfind('.').map_or("", |at| &name[at + 1..]);
    if extension.len() > 3 {
        return None;
    }
    #[cfg(windows)]
    {
        let cut = path.iter().rposition(|&b| b == b'\\' || b == b'/')?;
        let taken = names_in(e, &path[..cut]);
        let mut generator = ntdll::Generator::new(&name);
        (0..1000)
            .map(|_| generator.next())
            .find(|short| !taken.contains(&short.to_ascii_uppercase()))
    }
    #[cfg(not(windows))]
    {
        let _ = e;
        None
    }
}

/// Every long and short name the directory at the full path `directory` holds now, upper case:
/// the player's entries ([`host_search`]) the game did not delete, take into the run or rename
/// away, and the run's files there ([`Entry`]).
#[cfg(windows)]
fn names_in(e: &Engine, directory: &[u8]) -> std::collections::HashSet<String> {
    let folder = key(directory);
    let shown = String::from_utf8_lossy(directory).into_owned();
    let disk = &e.kernel.disk;
    let mut taken = std::collections::HashSet::new();
    let mut add = |name: &str, short: &str| {
        taken.insert(name.to_ascii_uppercase());
        if !short.is_empty() {
            taken.insert(short.to_ascii_uppercase());
        }
    };
    for h in host_search(&real_path(directory).join("*"), &shown).unwrap_or_default() {
        let k = key(&[folder.as_slice(), b"\\", h.name.as_bytes()].concat());
        if !disk.deleted.contains(&k) && !disk.files.contains_key(&k) {
            add(&h.name, &h.short);
        }
    }
    for (k, entry) in &disk.entries {
        if in_folder(k, &folder) && disk.files.contains_key(k) {
            add(&entry.name, entry.short.as_deref().unwrap_or(""));
        }
    }
    taken
}

/// ntdll's `RtlGenerate8dot3Name` (the system's own 8.3 name maker), asked for one long name
/// again and again with one context, as a file system asks while the name it got is taken.
#[cfg(windows)]
mod ntdll {
    #[repr(C)]
    struct UnicodeString {
        length: u16,
        maximum: u16,
        buffer: *mut u16,
    }

    /// `GENERATE_NAME_CONTEXT` (ntifs.h).
    #[repr(C)]
    #[derive(Default)]
    struct Context {
        checksum: u16,
        checksum_inserted: u8,
        name_length: u8,
        name_buffer: [u16; 8],
        extension_length: u32,
        extension_buffer: [u16; 4],
        last_index_value: u32,
    }

    #[link(name = "ntdll")]
    extern "system" {
        fn RtlGenerate8dot3Name(
            name: *const UnicodeString,
            allow_extended_characters: u8,
            context: *mut Context,
            name8dot3: *mut UnicodeString,
        ) -> i32;
    }

    pub struct Generator {
        name: Vec<u16>,
        context: Context,
    }

    impl Generator {
        pub fn new(name: &str) -> Self {
            Generator {
                name: name.encode_utf16().collect(),
                context: Context::default(),
            }
        }

        /// The next name (`NAME~1`, `NAME~2`, ...).
        pub fn next(&mut self) -> String {
            let bytes = (self.name.len() * 2) as u16;
            let long = UnicodeString {
                length: bytes,
                maximum: bytes,
                buffer: self.name.as_mut_ptr(),
            };
            let mut buffer = [0u16; 12];
            let mut short = UnicodeString {
                length: 0,
                maximum: 24,
                buffer: buffer.as_mut_ptr(),
            };
            // SAFETY: `long` points at the name's UTF-16 units (no terminator needed: the length
            // is given), `short` at a 12-unit buffer (an 8.3 name's most), and the context is
            // the zeroed `GENERATE_NAME_CONTEXT` the first call takes, kept between calls.
            unsafe { RtlGenerate8dot3Name(&long, 0, &mut self.context, &mut short) };
            String::from_utf16_lossy(&buffer[..usize::from(short.length / 2)])
        }
    }
}

// Platform: replaces KERNEL32.DLL FindNextFileA (import)
/// `FindNextFileA(handle, data)`: the search's next entry written as `FindFirstFileA` writes
/// one, true; when none is left FALSE with `ERROR_NO_MORE_FILES` (measured). Another handle is
/// not provided, nor a search of a folder the game changed since its `FindFirstFileA`
/// ([`Search::folder`]).
pub fn find_next_file_a(e: &mut Engine, handle: u32, data: Ptr) -> u32 {
    let disk = &e.kernel.disk;
    let next = match e.kernel.objects.get_mut(&handle) {
        Some(KernelObject::FindFile(search)) => {
            if let Some((folder, seen)) = &search.folder {
                if folder_state(disk, folder) != *seen {
                    panic!(
                        "FindNextFileA({handle:#x}): the game changed the folder \"{}\" since \
                         FindFirstFileA; not provided",
                        String::from_utf8_lossy(folder)
                    );
                }
            }
            search.rest.pop_front()
        }
        _ => panic!("FindNextFileA({handle:#x}): not a search handle; not provided"),
    };
    let Some(found) = next else {
        e.kernel.last_error = ERROR_NO_MORE_FILES;
        return 0;
    };
    let shown = found.name().to_string();
    write_found(e, data, &found, &shown);
    1
}
// Platform: replaces KERNEL32.DLL CompareFileTime (import)
/// `CompareFileTime(a, b)`: -1, 0 or 1 as the 64-bit time at `a` is less than, equal to or
/// greater than the one at `b`.
pub fn compare_file_time(e: &mut Engine, a: Ptr, b: Ptr) -> i32 {
    let time = |e: &Engine, p: Ptr| {
        u64::from(e.mem.u32(p.addr())) | (u64::from(e.mem.u32(p.addr() + 4)) << 32)
    };
    match time(e, a).cmp(&time(e, b)) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}

// Platform: replaces KERNEL32.DLL FindClose (import)
/// `FindClose(handle)`: a search handle from [`find_first_file_a`] is closed, TRUE, the last
/// error left as it was. `INVALID_HANDLE_VALUE` answers FALSE with `ERROR_INVALID_HANDLE` and
/// raises nothing: the game passes it after a search that found nothing, closing the search
/// whatever `FindFirstFileA` answered (`007cbd62` in the start menu's `007cbaf0` on a PC with no
/// saves, `00846ab0`, `00b00461`, `0085fd34`, `0085fdc4`, `00da2805`). Measured on this machine
/// (2026-11-02, a 32-bit process, with no debugger and under one,
/// `build43-scratch/measure/small-fixes/findclose-2026-11-02.tsv`).
/// Any other number stops the run: Windows reads it as its own search record, so the answer
/// depends on the memory at that number, which this layer does not model (measured for 0, a
/// search closed before, a file handle and `0x0ffffff0`: FALSE with `ERROR_NOACCESS` after an
/// access violation it handles itself; the file stays open). No call of the game's passes one:
/// the other 16 calls of the import slot `00fdf064` run only on a search handle.
pub fn find_close(e: &mut Engine, handle: u32) -> u32 {
    if handle == INVALID_HANDLE_VALUE {
        e.kernel.last_error = ERROR_INVALID_HANDLE;
        return 0;
    }
    assert!(
        matches!(
            e.kernel.objects.get(&handle),
            Some(KernelObject::FindFile(_))
        ),
        "FindClose({handle:#x}): not a search handle; not provided (Windows reads the number as \
         its search record: FALSE with ERROR_NOACCESS after an access violation it handles, on \
         this machine)"
    );
    e.kernel.objects.remove(&handle);
    1
}

// Platform: replaces KERNEL32.DLL CreateFileA (import)
/// `CreateFileA(path, access, share, security, disposition, attributes, template)`: opens or
/// makes the file in [`Disk`] by the disposition: `CREATE_NEW` fails with `ERROR_FILE_EXISTS`
/// on an existing file; `CREATE_ALWAYS` empties or makes it (`ERROR_ALREADY_EXISTS` when it
/// existed, else 0); `OPEN_EXISTING` and `TRUNCATE_EXISTING` (which needs `GENERIC_WRITE` and
/// empties it) fail with `ERROR_FILE_NOT_FOUND` on a missing file; `OPEN_ALWAYS` makes a missing
/// one (`ERROR_ALREADY_EXISTS` when it existed). Answers the handle (position 0) or
/// `INVALID_HANDLE_VALUE`. Sharing, security and attributes don't change the outcome here.
/// A missing file for `OPEN_EXISTING` or `TRUNCATE_EXISTING` answers `ERROR_FILE_NOT_FOUND`,
/// or `ERROR_PATH_NOT_FOUND` when its directory is missing too (measured; see the arm).
#[allow(clippy::too_many_arguments)]
pub fn create_file_a(
    e: &mut Engine,
    path: Ptr,
    access: u32,
    _share: u32,
    _security: u32,
    disposition: u32,
    _attributes: u32,
    _template: u32,
) -> u32 {
    let name = full_path(e, &e.mem.cstr(path.addr()));
    let path = key(&name);
    // A sound file (music among them) opened: named in the sound recording's log, when
    // there is one (`super::dsound::note`).
    if [".mp3", ".wav", ".ogg", ".bik"]
        .iter()
        .any(|x| path.ends_with(x.as_bytes()))
    {
        let what = format!("open {}", String::from_utf8_lossy(&name));
        super::dsound::note(e, &what);
    }
    // A file of the player's opened for reading only is read from disk where the reads ask
    // (`Disk::player`); the outcome is that of an existing file's open.
    if access & GENERIC_WRITE == 0
        && matches!(disposition, OPEN_EXISTING | OPEN_ALWAYS)
        && !e.kernel.disk.files.contains_key(&path)
        && !e.kernel.disk.deleted.contains(&path)
    {
        let real = real_path(&name);
        if std::fs::metadata(&real).is_ok_and(|m| m.is_file()) {
            e.kernel.disk.player.insert(path.clone(), real);
            e.kernel.last_error = if disposition == OPEN_ALWAYS {
                ERROR_ALREADY_EXISTS
            } else {
                ERROR_SUCCESS
            };
            return e.kernel.insert(KernelObject::File(OpenFile {
                path,
                position: 0,
                read: access & GENERIC_READ != 0,
                write: false,
                wrote: None,
            }));
        }
    }
    // A file of the player's (on disk, not deleted in this run, not yet the run's): it exists
    // for every disposition. Opened for writing it is read into the run's copy first, emptied
    // when the disposition empties it, with its directory entry as the disk keeps it ([`adopt`]).
    let players = !e.kernel.disk.files.contains_key(&path)
        && !e.kernel.disk.deleted.contains(&path)
        && std::fs::metadata(real_path(&name)).is_ok_and(|m| m.is_file());
    let exists = e.kernel.disk.files.contains_key(&path) || players;
    let fail = |e: &mut Engine, code: u32| {
        e.kernel.last_error = code;
        INVALID_HANDLE_VALUE
    };
    let mut error = ERROR_SUCCESS;
    match disposition {
        CREATE_NEW if exists => return fail(e, ERROR_FILE_EXISTS),
        CREATE_NEW => made(e, &name, &path),
        CREATE_ALWAYS => {
            if exists {
                error = ERROR_ALREADY_EXISTS;
                adopt(e, &name, &path, false);
                emptied(e, &path);
            } else {
                made(e, &name, &path);
            }
        }
        // Measured on this machine (2026-10-21, PowerShell P/Invoke, GENERIC_READ): a missing
        // file answers ERROR_FILE_NOT_FOUND (2), one in a missing directory
        // ERROR_PATH_NOT_FOUND (3); TRUNCATE_EXISTING without GENERIC_WRITE answers
        // ERROR_INVALID_PARAMETER (87) for a missing file too.
        TRUNCATE_EXISTING if !exists && access & GENERIC_WRITE == 0 => {
            return fail(e, ERROR_INVALID_PARAMETER)
        }
        OPEN_EXISTING | TRUNCATE_EXISTING if !exists => {
            let code = if directory_exists(e, &name) {
                ERROR_FILE_NOT_FOUND
            } else {
                ERROR_PATH_NOT_FOUND
            };
            return fail(e, code);
        }
        OPEN_EXISTING => adopt(e, &name, &path, true),
        TRUNCATE_EXISTING => {
            if access & GENERIC_WRITE == 0 {
                return fail(e, ERROR_INVALID_PARAMETER);
            }
            adopt(e, &name, &path, false);
            emptied(e, &path);
        }
        OPEN_ALWAYS => {
            if exists {
                error = ERROR_ALREADY_EXISTS;
                adopt(e, &name, &path, true);
            } else {
                made(e, &name, &path);
            }
        }
        _ => return fail(e, ERROR_INVALID_PARAMETER),
    }
    e.kernel.last_error = error;
    e.kernel.insert(KernelObject::File(OpenFile {
        path,
        position: 0,
        read: access & GENERIC_READ != 0,
        write: access & GENERIC_WRITE != 0,
        wrote: None,
    }))
}

/// A file of the player's at the full path `name` (key `k`) that a write open takes into the run:
/// its bytes (when `read`; a disposition that empties it needs none) and its directory entry as
/// the disk keeps it ([`entry_of`]). Nothing for a file already in the run.
fn adopt(e: &mut Engine, name: &[u8], k: &[u8], read: bool) {
    if e.kernel.disk.files.contains_key(k) {
        return;
    }
    let entry = entry_of(e, name);
    let data = if read {
        read_whole(e, name).unwrap_or_default()
    } else {
        vec![]
    };
    e.kernel.disk.files.insert(k.to_vec(), data);
    if let Some(entry) = entry {
        e.kernel.disk.entries.insert(k.to_vec(), entry);
    }
}

/// The file at key `k` emptied by `CREATE_ALWAYS` or `TRUNCATE_EXISTING`: size 0, last access
/// and last write now, the creation time kept, `FILE_ATTRIBUTE_ARCHIVE` set (measured,
/// `findfile-changed/findfile-2026-11-03.txt` 2: listed so after the handle was closed with
/// nothing written).
fn emptied(e: &mut Engine, k: &[u8]) {
    let now = file_clock(e);
    e.kernel.disk.files.insert(k.to_vec(), vec![]);
    if let Some(entry) = e.kernel.disk.entries.get_mut(k) {
        entry.size = 0;
        entry.accessed = now;
        entry.written = now;
        entry.attributes |= FILE_ATTRIBUTE_ARCHIVE;
    }
}

/// A file made at the full path `name` (key `k`): empty, with the entry the file system gives a
/// new file (`findfile-changed/findfile-2026-11-03.txt` 1, 2): `FILE_ATTRIBUTE_ARCHIVE`, size
/// 0, the three times now; the creation time and short name of the same name removed from that
/// folder within 15 s ([`tunnelled`]), else a short name made for it ([`short_name`]).
fn made(e: &mut Engine, name: &[u8], k: &[u8]) {
    let now = file_clock(e);
    let (created, short) = match tunnelled(e, k) {
        Some(old) => (old.created, old.short),
        None => (now, short_name(e, name)),
    };
    e.kernel.disk.deleted.remove(k);
    e.kernel.disk.files.insert(k.to_vec(), vec![]);
    let entry = Entry {
        name: last_part(name),
        short,
        attributes: FILE_ATTRIBUTE_ARCHIVE,
        size: 0,
        created,
        accessed: now,
        written: now,
    };
    e.kernel.disk.entries.insert(k.to_vec(), entry);
}

/// A file handle closed (`CloseHandle`): a handle that wrote publishes the file's size, and the
/// time of its last write as the last access and last write (measured: a search shows neither
/// while the handle is open, `findfile-changed/findfile-2026-11-03.txt` 2).
pub fn closed(e: &mut Engine, file: &OpenFile) {
    let Some(at) = file.wrote else {
        return;
    };
    let size = file_size(e, &file.path);
    if let Some(entry) = e.kernel.disk.entries.get_mut(&file.path) {
        entry.size = size;
        entry.accessed = at;
        entry.written = at;
    }
}

// Platform: replaces KERNEL32.DLL GetFileType (import)
/// `GetFileType(handle)`: `FILE_TYPE_DISK` for a file handle; `FILE_TYPE_UNKNOWN` with
/// `ERROR_INVALID_HANDLE` for anything else.
pub fn get_file_type(e: &mut Engine, handle: u32) -> u32 {
    if open_file(e, handle).is_some() {
        e.kernel.last_error = ERROR_SUCCESS;
        FILE_TYPE_DISK
    } else {
        e.kernel.last_error = ERROR_INVALID_HANDLE;
        FILE_TYPE_UNKNOWN
    }
}

// Platform: replaces KERNEL32.DLL SetFilePointer (import)
/// `SetFilePointer(handle, low, high, method)`: moves the position by the distance (`low` alone
/// as a signed 32-bit value when `high` is null, else the signed 64-bit `high:low`) from the
/// start (0), the position (1) or the end (2). Answers the new position's low word (its high
/// word stored through `high`) with last error 0; a position before the start fails with
/// `ERROR_NEGATIVE_SEEK`, a bad method with `ERROR_INVALID_PARAMETER`, both answering
/// `INVALID_SET_FILE_POINTER` and leaving the position.
pub fn set_file_pointer(e: &mut Engine, handle: u32, low: u32, high: Ptr, method: u32) -> u32 {
    let distance = if high.addr() == 0 {
        i64::from(low as i32)
    } else {
        ((u64::from(e.mem.u32(high.addr())) << 32) | u64::from(low)) as i64
    };
    let Some(file) = open_file(e, handle) else {
        e.kernel.last_error = ERROR_INVALID_HANDLE;
        return INVALID_SET_FILE_POINTER;
    };
    let path = file.path.clone();
    let position = file.position;
    let size = file_size(e, &path);
    let base = match method {
        0 => 0i64,
        1 => position as i64,
        2 => size as i64,
        _ => {
            e.kernel.last_error = ERROR_INVALID_PARAMETER;
            return INVALID_SET_FILE_POINTER;
        }
    };
    let target = base.wrapping_add(distance);
    if target < 0 {
        e.kernel.last_error = ERROR_NEGATIVE_SEEK;
        return INVALID_SET_FILE_POINTER;
    }
    if let Some(file) = open_file(e, handle) {
        file.position = target as u64;
    }
    if high.addr() != 0 {
        e.mem.set_u32(high.addr(), (target as u64 >> 32) as u32);
    }
    e.kernel.last_error = ERROR_SUCCESS;
    target as u32
}

// Platform: replaces KERNEL32.DLL WriteFile (import)
/// `WriteFile(handle, buffer, count, written, overlapped)`: writes `count` bytes at the position
/// (the file grows, a gap left by a seek past the end reads as zeros), moves the position on and
/// stores the count through `written`; a handle opened without `GENERIC_WRITE` fails with
/// `ERROR_ACCESS_DENIED`, any other handle with `ERROR_INVALID_HANDLE` (0 answered and 0 written).
/// The handle keeps the time of the write for the file's entry when it is closed ([`closed`]).
/// Overlapped writes are not provided (stops the run).
pub fn write_file(
    e: &mut Engine,
    handle: u32,
    buffer: Ptr,
    count: u32,
    written: Ptr,
    overlapped: u32,
) -> i32 {
    assert!(
        overlapped == 0,
        "WriteFile: overlapped writes are not provided"
    );
    if written.addr() != 0 {
        e.mem.set_u32(written.addr(), 0);
    }
    let (path, position) = match open_file(e, handle) {
        Some(file) if file.write => (file.path.clone(), file.position),
        Some(_) => {
            e.kernel.last_error = ERROR_ACCESS_DENIED;
            return 0;
        }
        None => {
            e.kernel.last_error = ERROR_INVALID_HANDLE;
            return 0;
        }
    };
    let bytes = e.mem.bytes(buffer.addr(), count);
    let now = file_clock(e);
    let data = e.kernel.disk.files.entry(path).or_default();
    let start = position as usize;
    let end = start + bytes.len();
    if data.len() < end {
        data.resize(end, 0);
    }
    data[start..end].copy_from_slice(&bytes);
    if let Some(file) = open_file(e, handle) {
        file.position = end as u64;
        // A write of no bytes changes no time a search shows (measured 2026-11-03: a file
        // opened again, 0 bytes written, closed: its three times as before,
        // `build43-scratch/measure/review-findfile/run1.txt`, `zero:`).
        if count > 0 {
            file.wrote = Some(now);
        }
    }
    if written.addr() != 0 {
        e.mem.set_u32(written.addr(), count);
    }
    e.kernel.last_error = ERROR_SUCCESS;
    1
}

// Platform: replaces KERNEL32.DLL ReadFile (import)
/// `ReadFile(handle, buffer, count, read, overlapped)`: reads up to `count` bytes from the
/// position (fewer at the end, none past it: success with 0 read), moves the position on and
/// stores the count read through `read`; a handle opened without `GENERIC_READ` fails with
/// `ERROR_ACCESS_DENIED`, any other handle with `ERROR_INVALID_HANDLE`. Overlapped reads are not
/// provided (stops the run).
pub fn read_file(
    e: &mut Engine,
    handle: u32,
    buffer: Ptr,
    count: u32,
    read: Ptr,
    overlapped: u32,
) -> i32 {
    assert!(
        overlapped == 0,
        "ReadFile: overlapped reads are not provided"
    );
    if read.addr() != 0 {
        e.mem.set_u32(read.addr(), 0);
    }
    let (path, position) = match open_file(e, handle) {
        Some(file) if file.read => (file.path.clone(), file.position),
        Some(_) => {
            e.kernel.last_error = ERROR_ACCESS_DENIED;
            return 0;
        }
        None => {
            e.kernel.last_error = ERROR_INVALID_HANDLE;
            return 0;
        }
    };
    let bytes = read_range(e, &path, position, count);
    e.mem.write(buffer.addr(), &bytes);
    let got = bytes.len() as u32;
    if let Some(file) = open_file(e, handle) {
        file.position = position + u64::from(got);
    }
    if read.addr() != 0 {
        e.mem.set_u32(read.addr(), got);
    }
    e.kernel.last_error = ERROR_SUCCESS;
    1
}

// Platform: replaces KERNEL32.DLL CreateFileW (import)
/// `CreateFileW(path, access, share, security, disposition, attributes, template)`: as
/// [`create_file_a`] on the same path. Measured on this machine (2026-10-27,
/// `build43-scratch/measure/platform5`): for an existing file, a missing one and one in a
/// missing directory, each of the four accesses (read, write, both, none) and the
/// dispositions 0..=6, `CreateFileW` and `CreateFileA` gave the same answer, the same last
/// error and the same file afterwards (84 cases). A path with a character outside ASCII is not
/// provided (the files are kept by their ANSI names; stops the run).
#[allow(clippy::too_many_arguments)]
pub fn create_file_w(
    e: &mut Engine,
    path: Ptr,
    access: u32,
    share: u32,
    security: u32,
    disposition: u32,
    attributes: u32,
    template: u32,
) -> u32 {
    let mut name = Vec::new();
    let mut at = path.addr();
    loop {
        let unit = e.mem.u16(at);
        if unit == 0 {
            break;
        }
        assert!(
            unit < 0x80,
            "CreateFileW: the non-ASCII character {unit:#x} in a path is not provided"
        );
        name.push(unit as u8);
        at = at.wrapping_add(2);
    }
    let narrow = e.mem.alloc(name.len() as u32 + 1);
    e.mem.set_cstr(narrow, &name);
    let handle = create_file_a(
        e,
        Ptr::new(narrow),
        access,
        share,
        security,
        disposition,
        attributes,
        template,
    );
    e.mem.free(narrow);
    handle
}

// Platform: replaces KERNEL32.DLL GetFileSizeEx (import)
/// `GetFileSizeEx(handle, size)`: for a file handle (whatever its access) stores the file's
/// size as a 64-bit number and answers TRUE, the last error left as it was; for a handle that is
/// not open (closed, 0, -1, any other number) answers FALSE with `ERROR_INVALID_HANDLE` and
/// leaves `size` as it was. Measured on this machine (2026-10-27,
/// `build43-scratch/measure/platform5`). A handle of another kernel object (an event, a thread)
/// is not provided (not measured; stops the run).
pub fn get_file_size_ex(e: &mut Engine, handle: u32, size: Ptr) -> u32 {
    let path = match e.kernel.objects.get(&handle) {
        Some(KernelObject::File(file)) => file.path.clone(),
        Some(other) => panic!("GetFileSizeEx({handle:#x}) on {other:?}: not provided"),
        None => {
            e.kernel.last_error = ERROR_INVALID_HANDLE;
            return 0;
        }
    };
    let bytes = file_size(e, &path);
    e.mem.set_u32(size.addr(), bytes as u32);
    e.mem.set_u32(size.addr() + 4, (bytes >> 32) as u32);
    1
}

/// `ERROR_INVALID_NAME` (winerror.h), `GetFullPathNameA`'s answer to an empty name.
pub const ERROR_INVALID_NAME: u32 = 123;
/// The path separator `\`.
const SEPARATOR: u8 = b'\\';

/// The parts of a path after its root, `.` and `..` resolved, empty parts (repeated separators)
/// dropped; `None` for a shape not measured (a part other than `.`/`..` that ends in a dot or a
/// space before the last part).
fn resolve_parts(parts: Vec<&[u8]>) -> Option<Vec<Vec<u8>>> {
    let mut out: Vec<Vec<u8>> = Vec::new();
    let count = parts.len();
    for (index, part) in parts.into_iter().enumerate() {
        match part {
            b"" | b"." => {}
            b".." => {
                out.pop();
            }
            _ => {
                let last = index + 1 == count;
                let mut part = part.to_vec();
                if last {
                    while matches!(part.last(), Some(b'.') | Some(b' ')) {
                        part.pop();
                    }
                } else if matches!(part.last(), Some(b'.') | Some(b' ')) {
                    return None;
                }
                if !part.is_empty() {
                    out.push(part);
                }
            }
        }
    }
    Some(out)
}

// Platform: replaces KERNEL32.DLL GetFullPathNameA (import slot 00fdf194)
/// `GetFullPathNameA(name, size, buffer, file part)` as measured on scratch names
/// (`build43-scratch/measure/fullpath/measured-2026-10-27.txt`, Windows 11): a relative name is
/// joined to the current directory (`Kernel::current_directory`, `GetCurrentDirectoryA`'s), a
/// name starting with a separator to its drive, and a drive-relative name of the current drive
/// (`C:x`) to the current directory; `/` reads as `\`, repeated separators collapse, `.` parts go
/// and `..` goes up (never past the root), and the last part loses its trailing dots and spaces.
/// A name that ends in a separator keeps it and has no file part (null); otherwise the file part
/// points at the last part. The answer is the length; with too small a buffer, the length with
/// its terminator, the buffer untouched. The last error is left as it was, except for an empty
/// name: 0 and `ERROR_INVALID_NAME`. Other drives, UNC and device names, and trailing dots or
/// spaces inside the path were not measured and stop the run.
pub fn get_full_path_name_a(
    e: &mut Engine,
    name: Ptr,
    size: u32,
    buffer: Ptr,
    part_out: Ptr,
) -> u32 {
    let text = e.mem.cstr(name.addr());
    if text.is_empty() {
        e.kernel.last_error = ERROR_INVALID_NAME;
        return 0;
    }
    let text: Vec<u8> = text
        .iter()
        .map(|&c| if c == b'/' { SEPARATOR } else { c })
        .collect();
    let current = e.kernel.current_directory.clone();
    assert!(
        current.len() >= 2 && current[1] == b':' && !text.starts_with(br"\\"),
        "GetFullPathNameA({}): not provided (UNC or device name)",
        String::from_utf8_lossy(&text)
    );
    let drive = current[..2].to_vec();
    let (root, rest): (Vec<u8>, Vec<u8>) = if text.len() >= 2 && text[1] == b':' {
        assert!(
            text[0].eq_ignore_ascii_case(&drive[0]),
            "GetFullPathNameA({}): another drive is not provided",
            String::from_utf8_lossy(&text)
        );
        if text.get(2) == Some(&SEPARATOR) {
            (text[..2].to_vec(), text[2..].to_vec())
        } else {
            let mut joined = current[2..].to_vec();
            joined.push(SEPARATOR);
            joined.extend_from_slice(&text[2..]);
            (drive.clone(), joined)
        }
    } else if text[0] == SEPARATOR {
        (drive.clone(), text.clone())
    } else {
        let mut joined = current[2..].to_vec();
        joined.push(SEPARATOR);
        joined.extend_from_slice(&text);
        (drive.clone(), joined)
    };
    let ends_with_separator = text.last() == Some(&SEPARATOR);
    let parts: Vec<&[u8]> = rest.split(|&c| c == SEPARATOR).collect();
    let resolved = resolve_parts(parts).unwrap_or_else(|| {
        panic!(
            "GetFullPathNameA({}): a part ending in a dot or space is not provided",
            String::from_utf8_lossy(&text)
        )
    });
    let mut full = root;
    let mut file_part = 0usize;
    for part in &resolved {
        full.push(SEPARATOR);
        file_part = full.len();
        full.extend_from_slice(part);
    }
    if resolved.is_empty() || ends_with_separator {
        full.push(SEPARATOR);
    }
    let length = full.len() as u32;
    if size <= length {
        return length + 1;
    }
    e.mem.set_cstr(buffer.addr(), &full);
    if !part_out.is_null() {
        let part = if ends_with_separator || resolved.is_empty() {
            0
        } else {
            buffer.addr() + file_part as u32
        };
        e.mem.set_u32(part_out.addr(), part);
    }
    length
}

#[cfg(test)]
mod tests_full_path_name {
    use super::*;

    /// `GetFullPathNameA` of `name` with a `size`-byte buffer: the answer, the text (empty when
    /// the buffer was left untouched), the file part's offset in the buffer (0 for null,
    /// `0xcccccccc` untouched) and the last error.
    fn full(e: &mut Engine, name: &str, size: u32) -> (u32, String, u32, u32) {
        let at = e.mem.alloc(0x400);
        e.mem.set_cstr(at, name.as_bytes());
        let buffer = at + 0x100;
        let part = at + 0x300;
        e.mem.write(buffer, &[0xcc; 5]);
        e.mem.set_u32(part, 0xcccc_cccc);
        e.kernel.last_error = 0x1234;
        let n = get_full_path_name_a(e, Ptr::new(at), size, Ptr::new(buffer), Ptr::new(part));
        let text = if e.mem.u8(buffer) == 0xcc {
            String::new()
        } else {
            String::from_utf8(e.mem.cstr(buffer).to_vec()).unwrap_or_default()
        };
        let p = e.mem.u32(part);
        let offset = if p == 0 || p == 0xcccc_cccc {
            p
        } else {
            p - buffer
        };
        (n, text, offset, e.kernel.last_error)
    }

    /// Every row of the measurement (2026-10-27), with the current directory `C:\G\F` in place
    /// of the scratch folder's: the text, the length, the file part and the last error.
    #[test]
    fn full_path_names_follow_the_measured_rows() {
        let mut e = Engine::new();
        e.kernel.current_directory = br"C:\G\F".to_vec();
        let rows: &[(&str, &str, u32)] = &[
            (
                r"Data\Video\FNVIntro.bik",
                r"C:\G\F\Data\Video\FNVIntro.bik",
                18,
            ),
            ("Data/Video/a.bik", r"C:\G\F\Data\Video\a.bik", 18),
            (r".\Data\x", r"C:\G\F\Data\x", 12),
            (r"..\x\y", r"C:\G\x\y", 7),
            (r"Data\..\y", r"C:\G\F\y", 7),
            (r"\root\x", r"C:\root\x", 8),
            ("C:x", r"C:\G\F\x", 7),
            (r"C:\a\b", r"C:\a\b", 5),
            (r"a\\b", r"C:\G\F\a\b", 9),
            ("x.", r"C:\G\F\x", 7),
            ("x ", r"C:\G\F\x", 7),
            (r"Data\Video\", r"C:\G\F\Data\Video\", 0),
            ("a/./b/../c", r"C:\G\F\a\c", 9),
            (r"C:\a\..\..\..\b", r"C:\b", 3),
        ];
        for &(name, text, part) in rows {
            assert_eq!(
                full(&mut e, name, 260),
                (text.len() as u32, text.into(), part, 0x1234),
                "{name:?}"
            );
            // Four bytes: the length with its terminator, the buffer untouched (the file part
            // was not measured here).
            let (n, short, _, error) = full(&mut e, name, 4);
            assert_eq!(
                (n, short, error),
                (text.len() as u32 + 1, String::new(), 0x1234),
                "{name:?} with four bytes"
            );
        }
        for size in [260, 4] {
            let (n, text, _, error) = full(&mut e, "", size);
            assert_eq!((n, text, error), (0, String::new(), 123), "empty, {size}");
        }
    }
}

#[cfg(test)]
mod tests_wide_and_size {
    use super::*;

    fn wide(e: &mut Engine, text: &str) -> u32 {
        let units: Vec<u16> = text.encode_utf16().chain(Some(0)).collect();
        let at = e.mem.alloc(2 * units.len() as u32);
        for (i, u) in units.iter().enumerate() {
            e.mem.set_u16(at + 2 * i as u32, *u);
        }
        at
    }

    #[test]
    fn create_file_w_opens_as_create_file_a_and_the_size_follows_the_writes() {
        let mut e = Engine::new();
        let name = wide(&mut e, "C:\\Some\\Measured.bin");
        let h = create_file_w(
            &mut e,
            Ptr::new(name),
            GENERIC_WRITE | GENERIC_READ,
            0,
            0,
            CREATE_ALWAYS,
            0x80,
            0,
        );
        assert_ne!(h, INVALID_HANDLE_VALUE);
        assert_eq!(e.kernel.last_error, ERROR_SUCCESS);
        let data = e.mem.alloc(0x10);
        assert_eq!(
            write_file(&mut e, h, Ptr::new(data), 0x0b, Ptr::new(0), 0),
            1
        );
        let size = e.mem.alloc(8);
        e.mem.set_u32(size, 0x5555_5555);
        e.mem.set_u32(size + 4, 0x5555_5555);
        e.kernel.last_error = 0x1234;
        assert_eq!(get_file_size_ex(&mut e, h, Ptr::new(size)), 1);
        assert_eq!((e.mem.u32(size), e.mem.u32(size + 4)), (0x0b, 0));
        assert_eq!(e.kernel.last_error, 0x1234);
        assert!(e
            .kernel
            .disk
            .files
            .contains_key(b"c:\\some\\measured.bin".as_slice()));
        // CREATE_NEW on it now fails as CreateFileA does.
        let again = create_file_w(
            &mut e,
            Ptr::new(name),
            GENERIC_READ,
            0,
            0,
            CREATE_NEW,
            0x80,
            0,
        );
        assert_eq!(again, INVALID_HANDLE_VALUE);
        assert_eq!(e.kernel.last_error, ERROR_FILE_EXISTS);
    }

    #[test]
    fn get_file_size_ex_on_a_handle_that_is_not_open_fails_and_keeps_the_size() {
        let mut e = Engine::new();
        let size = e.mem.alloc(8);
        e.mem.set_u32(size, 0x5555_5555);
        for handle in [0u32, 0xffff_ffff, 0x1230] {
            e.kernel.last_error = 0;
            assert_eq!(get_file_size_ex(&mut e, handle, Ptr::new(size)), 0);
            assert_eq!(e.kernel.last_error, ERROR_INVALID_HANDLE);
            assert_eq!(e.mem.u32(size), 0x5555_5555);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        let name = e.mem.alloc(0x40);
        e.mem.write(name, b"C:\\Some\\RendererInfo.txt\0");
        (e, name)
    }

    #[test]
    fn create_always_makes_then_reports_an_existing_file() {
        let (mut e, name) = engine();
        let h = create_file_a(
            &mut e,
            Ptr::new(name),
            GENERIC_WRITE,
            0,
            0,
            CREATE_ALWAYS,
            0x80,
            0,
        );
        assert_ne!(h, INVALID_HANDLE_VALUE);
        assert_eq!(e.kernel.last_error, 0);
        assert_eq!(get_file_type(&mut e, h), FILE_TYPE_DISK);
        let h2 = create_file_a(
            &mut e,
            Ptr::new(name),
            GENERIC_WRITE,
            0,
            0,
            CREATE_ALWAYS,
            0x80,
            0,
        );
        assert_ne!(h2, h);
        assert_eq!(e.kernel.last_error, ERROR_ALREADY_EXISTS);
        assert_eq!(
            create_file_a(
                &mut e,
                Ptr::new(name),
                GENERIC_WRITE,
                0,
                0,
                CREATE_NEW,
                0,
                0
            ),
            INVALID_HANDLE_VALUE
        );
        assert_eq!(e.kernel.last_error, ERROR_FILE_EXISTS);
    }

    #[test]
    fn writes_seeks_and_reads_back() {
        let (mut e, name) = engine();
        let rw = GENERIC_READ | GENERIC_WRITE;
        let h = create_file_a(&mut e, Ptr::new(name), rw, 0, 0, OPEN_ALWAYS, 0, 0);
        let buffer = e.mem.alloc(0x20);
        let count = e.mem.alloc(4);
        e.mem.write(buffer, b"abcdef");
        assert_eq!(
            write_file(&mut e, h, Ptr::new(buffer), 6, Ptr::new(count), 0),
            1
        );
        assert_eq!(e.mem.u32(count), 6);
        // From the end, -2: position 4.
        assert_eq!(
            set_file_pointer(&mut e, h, (-2i32) as u32, Ptr::new(0), 2),
            4
        );
        assert_eq!(
            read_file(&mut e, h, Ptr::new(buffer + 0x10), 8, Ptr::new(count), 0),
            1
        );
        assert_eq!(e.mem.u32(count), 2);
        assert_eq!(e.mem.bytes(buffer + 0x10, 2), b"ef".to_vec());
        // At the end: success, nothing read.
        assert_eq!(
            read_file(&mut e, h, Ptr::new(buffer), 8, Ptr::new(count), 0),
            1
        );
        assert_eq!(e.mem.u32(count), 0);
        // Before the start fails and keeps the position.
        assert_eq!(
            set_file_pointer(&mut e, h, (-100i32) as u32, Ptr::new(0), 1),
            INVALID_SET_FILE_POINTER
        );
        assert_eq!(e.kernel.last_error, ERROR_NEGATIVE_SEEK);
        // A seek past the end leaves a gap of zeros.
        assert_eq!(set_file_pointer(&mut e, h, 8, Ptr::new(0), 0), 8);
        write_file(&mut e, h, Ptr::new(buffer), 1, Ptr::new(count), 0);
        let path = key(b"c:/some/rendererinfo.txt");
        assert_eq!(e.kernel.disk.files[&path], b"abcdef\0\0a".to_vec());
        assert_eq!(super::super::close_handle(&mut e, h), 1);
        assert_eq!(get_file_type(&mut e, h), FILE_TYPE_UNKNOWN);
        assert_eq!(e.kernel.last_error, ERROR_INVALID_HANDLE);
    }

    #[test]
    fn a_read_only_handle_refuses_writes() {
        let (mut e, name) = engine();
        let h = create_file_a(
            &mut e,
            Ptr::new(name),
            GENERIC_WRITE,
            0,
            0,
            CREATE_NEW,
            0,
            0,
        );
        super::super::close_handle(&mut e, h);
        let h = create_file_a(
            &mut e,
            Ptr::new(name),
            GENERIC_READ,
            0,
            0,
            OPEN_EXISTING,
            0,
            0,
        );
        let buffer = e.mem.alloc(4);
        assert_eq!(
            write_file(&mut e, h, Ptr::new(buffer), 1, Ptr::new(0), 0),
            0
        );
        assert_eq!(e.kernel.last_error, ERROR_ACCESS_DENIED);
    }

    /// Was `opening_a_file_the_game_did_not_make_stops` (the open stopped the run); a missing
    /// file now fails as Windows answers it (measured).
    #[test]
    fn opening_a_missing_file_fails_as_windows_does() {
        let (mut e, name) = engine();
        // `C:\Some` is not there: the path is missing.
        let h = create_file_a(
            &mut e,
            Ptr::new(name),
            GENERIC_READ,
            0,
            0,
            OPEN_EXISTING,
            0,
            0,
        );
        assert_eq!(
            (h, e.kernel.last_error),
            (INVALID_HANDLE_VALUE, ERROR_PATH_NOT_FOUND)
        );
        let h = create_file_a(
            &mut e,
            Ptr::new(name),
            GENERIC_READ,
            0,
            0,
            TRUNCATE_EXISTING,
            0,
            0,
        );
        assert_eq!(
            (h, e.kernel.last_error),
            (INVALID_HANDLE_VALUE, ERROR_INVALID_PARAMETER)
        );
        // In a directory that exists (the crate's own): the file is missing.
        let missing = e.mem.alloc(0x200);
        let text = concat!(env!("CARGO_MANIFEST_DIR"), "/NoSuch.txt");
        e.mem.set_cstr(missing, text.as_bytes());
        let h = create_file_a(
            &mut e,
            Ptr::new(missing),
            GENERIC_READ,
            0,
            0,
            OPEN_EXISTING,
            0,
            0,
        );
        assert_eq!(
            (h, e.kernel.last_error),
            (INVALID_HANDLE_VALUE, ERROR_FILE_NOT_FOUND)
        );
    }

    #[test]
    fn directories_copies_and_deletes_stay_in_the_run() {
        let (mut e, name) = engine();
        let dir = e.mem.alloc(0x40);
        e.mem.set_cstr(dir, br"Q:\nv-rs-test-dir");
        assert_eq!(
            get_file_attributes_a(&mut e, Ptr::new(dir)),
            INVALID_FILE_ATTRIBUTES
        );
        assert_eq!(create_directory_a(&mut e, Ptr::new(dir), 0), 1);
        assert_eq!(
            get_file_attributes_a(&mut e, Ptr::new(dir)),
            FILE_ATTRIBUTE_DIRECTORY
        );
        assert_eq!(create_directory_a(&mut e, Ptr::new(dir), 0), 0);
        assert_eq!(e.kernel.last_error, ERROR_ALREADY_EXISTS);
        let h = create_file_a(
            &mut e,
            Ptr::new(name),
            GENERIC_WRITE,
            0,
            0,
            CREATE_NEW,
            0,
            0,
        );
        let buffer = e.mem.alloc(4);
        e.mem.write(buffer, b"ab");
        write_file(&mut e, h, Ptr::new(buffer), 2, Ptr::new(0), 0);
        let copy = e.mem.alloc(0x40);
        e.mem.set_cstr(copy, br"Q:\nv-rs-test-dir\copy.txt");
        assert_eq!(copy_file_a(&mut e, Ptr::new(name), Ptr::new(copy), 1), 1);
        assert_eq!(copy_file_a(&mut e, Ptr::new(name), Ptr::new(copy), 1), 0);
        assert_eq!(e.kernel.last_error, ERROR_FILE_EXISTS);
        assert_eq!(
            read_whole(&e, br"q:\NV-RS-TEST-DIR\copy.txt"),
            Some(b"ab".to_vec())
        );
        assert_eq!(delete_file_a(&mut e, Ptr::new(copy)), 1);
        assert_eq!(delete_file_a(&mut e, Ptr::new(copy)), 0);
        assert_eq!(e.kernel.last_error, ERROR_FILE_NOT_FOUND);
        assert_eq!(delete_file_a(&mut e, Ptr::new(dir)), 0);
        assert_eq!(e.kernel.last_error, ERROR_ACCESS_DENIED);
    }

    #[test]
    fn move_file_follows_the_measured_answers() {
        let (mut e, name) = engine();
        let dir = e.mem.alloc(0x40);
        e.mem.set_cstr(dir, br"Q:\nv-rs-move-dir");
        create_directory_a(&mut e, Ptr::new(dir), 0);
        let h = create_file_a(
            &mut e,
            Ptr::new(name),
            GENERIC_WRITE,
            0,
            0,
            CREATE_NEW,
            0,
            0,
        );
        let buffer = e.mem.alloc(4);
        e.mem.write(buffer, b"ab");
        write_file(&mut e, h, Ptr::new(buffer), 2, Ptr::new(0), 0);
        let path = |e: &mut Engine, p: &[u8]| {
            let at = e.mem.alloc(0x40);
            e.mem.set_cstr(at, p);
            Ptr::new(at)
        };
        let moved = path(&mut e, br"Q:\nv-rs-move-dir\moved.txt");
        let nowhere = path(&mut e, br"Q:\nv-rs-no-dir\moved.txt");
        assert_eq!(move_file_a(&mut e, Ptr::new(name), nowhere), 0);
        assert_eq!(e.kernel.last_error, ERROR_PATH_NOT_FOUND);
        assert_eq!(move_file_a(&mut e, Ptr::new(name), moved), 1);
        assert_eq!(
            read_whole(&e, br"q:\nv-rs-move-dir\MOVED.txt"),
            Some(b"ab".to_vec())
        );
        assert_eq!(move_file_a(&mut e, Ptr::new(name), moved), 0);
        assert_eq!(e.kernel.last_error, ERROR_FILE_NOT_FOUND);
        let upper = path(&mut e, br"Q:\NV-RS-MOVE-DIR\MOVED.TXT");
        assert_eq!(
            move_file_a(&mut e, moved, upper),
            1,
            "the same file in another case"
        );
        let other = path(&mut e, br"Q:\nv-rs-move-dir\other.txt");
        copy_file_a(&mut e, moved, other, 0);
        assert_eq!(move_file_a(&mut e, moved, other), 0);
        assert_eq!(e.kernel.last_error, ERROR_ALREADY_EXISTS);
    }

    #[test]
    fn relative_names_are_in_the_current_directory() {
        let mut e = Engine::new();
        e.kernel.current_directory = br"C:\Games\FNV".to_vec();
        assert_eq!(
            full_path(&e, b"Warnings.txt"),
            br"C:\Games\FNV\Warnings.txt".to_vec()
        );
        assert_eq!(full_path(&e, br"D:\x"), br"D:\x".to_vec());
        assert!(is_full_path(br"\\server\share"));
    }
}

#[cfg(test)]
mod tests_player_files {
    use super::*;

    /// A file of the player's opened for reading only is read where the reads ask, not copied:
    /// the crate's own manifest stands in for one.
    #[test]
    fn searches_match_and_order_as_measured() {
        // The measured directory (findfile-2026-10-23.txt) by its names.
        let names = [
            "A.esm",
            "b.ESM",
            "c.esm.bak",
            "d.esmx",
            "Z_long_name_file.esm",
            "e.es",
            "f.esp",
            "_under.esm",
            "10.esm",
            "2.esm",
            "aa.esm",
            "Ab.esm",
            "a b.esm",
            "g.esm",
        ];
        let found = |pattern: &[u8]| {
            let mut v: Vec<&str> = names
                .iter()
                .copied()
                .filter(|n| search_finds(pattern, n, "test"))
                .collect();
            v.sort_by_key(|n| n.to_ascii_uppercase());
            v
        };
        assert_eq!(
            found(b"*.esm"),
            [
                "10.esm",
                "2.esm",
                "a b.esm",
                "A.esm",
                "aa.esm",
                "Ab.esm",
                "b.ESM",
                "d.esmx",
                "g.esm",
                "Z_long_name_file.esm",
                "_under.esm"
            ]
        );
        assert_eq!(found(b"*.esp"), ["f.esp"]);
        assert_eq!(
            found(b"*.ES?").len(),
            13,
            "e.es and f.esp too, not c.esm.bak"
        );
        assert!(found(b"x*.esm").is_empty());
        assert_eq!(found(b"?.esm"), ["2.esm", "A.esm", "b.ESM", "g.esm"]);
        assert!(search_finds(b"*", ".", "test") && search_finds(b"*", "..", "test"));
        assert!(!search_finds(b"*.esm", "..", "test"));
        assert_eq!(short_extension("d.esmx").as_deref(), Some("ESM"));
        assert_eq!(short_extension("A.esm"), None);
        assert_eq!(short_extension("c.esm.bak").as_deref(), Some("BAK"));
    }

    #[test]
    fn a_wildcard_search_lists_then_runs_out() {
        let mut e = Engine::new();
        let name = e.mem.alloc(0x200);
        let pattern = concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.*");
        e.mem.set_cstr(name, pattern.as_bytes());
        let data = e.mem.alloc(0x140);
        let h = find_first_file_a(&mut e, Ptr::new(name), Ptr::new(data));
        assert_ne!(h, INVALID_HANDLE_VALUE);
        assert_eq!(e.mem.cstr(data + 0x2c), b"Cargo.toml".to_vec());
        e.kernel.last_error = 0;
        assert_eq!(find_next_file_a(&mut e, h, Ptr::new(data)), 0);
        assert_eq!(e.kernel.last_error, ERROR_NO_MORE_FILES);
        assert_eq!(find_close(&mut e, h), 1);
        let none = concat!(env!("CARGO_MANIFEST_DIR"), "/NoSuch*.x");
        e.mem.set_cstr(name, none.as_bytes());
        assert_eq!(
            find_first_file_a(&mut e, Ptr::new(name), Ptr::new(data)),
            INVALID_HANDLE_VALUE
        );
        assert_eq!(e.kernel.last_error, ERROR_FILE_NOT_FOUND);
    }

    /// The game closes a search whether or not `FindFirstFileA` found anything (`007cbd62` in
    /// the start menu's `007cbaf0`, also `00846ab0`, `00b00461`, `0085fd34`, `0085fdc4`,
    /// `00da2805`): on a PC with no saves the start menu's check passes `INVALID_HANDLE_VALUE`.
    /// Windows answers FALSE with `ERROR_INVALID_HANDLE` and raises nothing; on a search handle
    /// TRUE, the last error left as it was (measured, 32-bit,
    /// `build43-scratch/measure/small-fixes/findclose-2026-11-02.tsv`).
    #[test]
    fn find_close_after_a_search_that_found_nothing_answers_false() {
        let mut e = Engine::new();
        let name = e.mem.alloc(0x200);
        let data = e.mem.alloc(0x140);
        let none = concat!(env!("CARGO_MANIFEST_DIR"), "/NoSuch*.x");
        e.mem.set_cstr(name, none.as_bytes());
        let h = find_first_file_a(&mut e, Ptr::new(name), Ptr::new(data));
        assert_eq!(h, INVALID_HANDLE_VALUE);
        assert_eq!(e.kernel.last_error, ERROR_FILE_NOT_FOUND);
        assert_eq!(find_close(&mut e, h), 0);
        assert_eq!(e.kernel.last_error, ERROR_INVALID_HANDLE);
        let pattern = concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.*");
        e.mem.set_cstr(name, pattern.as_bytes());
        let h = find_first_file_a(&mut e, Ptr::new(name), Ptr::new(data));
        assert_ne!(h, INVALID_HANDLE_VALUE);
        e.kernel.last_error = 0x1234_5678;
        assert_eq!(find_close(&mut e, h), 1);
        assert_eq!(e.kernel.last_error, 0x1234_5678);
        assert!(!e.kernel.objects.contains_key(&h));
    }

    /// Any other number (0, a closed search, a file handle) stops the run: Windows reads it as
    /// its own search record, so the answer depends on memory this layer does not model (on
    /// this machine FALSE with `ERROR_NOACCESS` after an access violation it handles itself).
    /// No call of the game's passes one.
    #[test]
    #[should_panic(expected = "not a search handle")]
    fn find_close_on_a_handle_that_is_no_search_stops() {
        let mut e = Engine::new();
        find_close(&mut e, 0);
    }

    #[test]
    fn compare_file_time_orders_the_64_bit_times() {
        let mut e = Engine::new();
        let a = e.mem.alloc(8);
        let b = e.mem.alloc(8);
        e.mem.set_u32(a + 4, 1);
        e.mem.set_u32(b, 0xffff_ffff);
        assert_eq!(compare_file_time(&mut e, Ptr::new(a), Ptr::new(b)), 1);
        assert_eq!(compare_file_time(&mut e, Ptr::new(b), Ptr::new(a)), -1);
        assert_eq!(compare_file_time(&mut e, Ptr::new(a), Ptr::new(a)), 0);
    }

    #[test]
    fn a_read_only_open_reads_the_players_file_in_place() {
        let mut e = Engine::new();
        let real = concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml");
        let bytes = std::fs::read(real).unwrap();
        let name = e.mem.alloc(0x200);
        e.mem.set_cstr(name, real.as_bytes());
        let h = create_file_a(
            &mut e,
            Ptr::new(name),
            GENERIC_READ,
            1,
            0,
            OPEN_EXISTING,
            0x80,
            0,
        );
        assert_ne!(h, INVALID_HANDLE_VALUE);
        assert_eq!(e.kernel.last_error, ERROR_SUCCESS);
        assert!(e.kernel.disk.files.is_empty());
        assert_eq!(e.kernel.disk.player.len(), 1);
        let end = set_file_pointer(&mut e, h, 0, Ptr::new(0), 2);
        assert_eq!(end as usize, bytes.len());
        assert_eq!(set_file_pointer(&mut e, h, 2, Ptr::new(0), 0), 2);
        let buffer = e.mem.alloc(0x10);
        let count = e.mem.alloc(4);
        assert_eq!(
            read_file(&mut e, h, Ptr::new(buffer), 5, Ptr::new(count), 0),
            1
        );
        assert_eq!(e.mem.u32(count), 5);
        assert_eq!(e.mem.bytes(buffer, 5), bytes[2..7].to_vec());
        // Past the end: success, nothing read.
        set_file_pointer(&mut e, h, 0, Ptr::new(0), 2);
        assert_eq!(
            read_file(&mut e, h, Ptr::new(buffer), 5, Ptr::new(count), 0),
            1
        );
        assert_eq!(e.mem.u32(count), 0);
        // FindFirstFileA describes it, in the case the directory keeps (asked in upper case).
        let upper = e.mem.alloc(0x200);
        let shouted = real.replace("Cargo.toml", "CARGO.TOML");
        e.mem.set_cstr(upper, shouted.as_bytes());
        let data = e.mem.alloc(0x140);
        e.mem.write(data, &[0xaa; 0x140]);
        e.kernel.last_error = 203;
        let find = find_first_file_a(&mut e, Ptr::new(upper), Ptr::new(data));
        assert_ne!(find, INVALID_HANDLE_VALUE);
        assert_eq!(e.kernel.last_error, 203);
        assert_eq!(e.mem.u32(data + 0x20) as usize, bytes.len());
        assert_eq!(e.mem.u32(data + 0x1c), 0);
        assert_eq!(e.mem.u32(data) & FILE_ATTRIBUTE_DIRECTORY, 0);
        assert_ne!(e.mem.u32(data + 0x18), 0);
        assert_eq!(e.mem.cstr(data + 0x2c), b"Cargo.toml".to_vec());
        assert_eq!(e.mem.u8(data + 0x2c + 11), 0xaa);
        assert_eq!(find_close(&mut e, find), 1);
        // A missing file, and a file in a missing directory.
        let missing = e.mem.alloc(0x200);
        e.mem.set_cstr(
            missing,
            real.replace("Cargo.toml", "NoSuch.toml").as_bytes(),
        );
        let none = find_first_file_a(&mut e, Ptr::new(missing), Ptr::new(data));
        assert_eq!(
            (none, e.kernel.last_error),
            (INVALID_HANDLE_VALUE, ERROR_FILE_NOT_FOUND)
        );
        e.mem.set_cstr(
            missing,
            real.replace("Cargo.toml", "NoDir/x.toml").as_bytes(),
        );
        let none = find_first_file_a(&mut e, Ptr::new(missing), Ptr::new(data));
        assert_eq!(
            (none, e.kernel.last_error),
            (INVALID_HANDLE_VALUE, ERROR_PATH_NOT_FOUND)
        );
        // A write open still copies it into the run.
        let w = create_file_a(
            &mut e,
            Ptr::new(name),
            GENERIC_WRITE,
            0,
            0,
            OPEN_EXISTING,
            0x80,
            0,
        );
        assert_ne!(w, INVALID_HANDLE_VALUE);
        assert_eq!(e.kernel.disk.files.len(), 1);
    }
}

/// Searches of a directory the game changed in this run list it as Windows would after the
/// same calls (`build43-scratch/measure/findfile-changed/findfile-2026-11-03.txt`): the pause
/// menu's `Saves\*.fos` after a save stopped the run before (`007cbaf0`).
#[cfg(test)]
mod tests_changed_directories {
    use super::*;

    const SECOND: u64 = 10_000_000;
    const SAVE_138: &str = "Save 138   Alexxander  Doc Mitchell s House  02 02 23.fos";

    /// A scratch folder of the test's own (under the system's temporary folder) holding `files`
    /// (name, size), made one after the other.
    fn scratch(test: &str, files: &[(&str, usize)]) -> String {
        let dir = std::env::temp_dir().join(format!("nv-rs-{test}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        for &(name, size) in files {
            std::fs::write(dir.join(name), vec![b'p'; size]).unwrap();
        }
        dir.to_string_lossy().into_owned()
    }

    fn text(e: &mut Engine, s: &str) -> Ptr {
        let at = e.mem.alloc(0x200);
        e.mem.set_cstr(at, s.as_bytes());
        Ptr::new(at)
    }

    fn tick(e: &mut Engine) -> u64 {
        let now = e.kernel.disk.clock.unwrap_or(0x01dd_6b85_0000_0000) + SECOND;
        e.kernel.disk.clock = Some(now);
        now
    }

    fn open(e: &mut Engine, path: &str, access: u32, disposition: u32) -> u32 {
        let name = text(e, path);
        create_file_a(e, name, access, 0, 0, disposition, 0x80, 0)
    }

    fn write(e: &mut Engine, h: u32, n: u32) {
        let buffer = e.mem.alloc(n);
        assert_eq!(write_file(e, h, Ptr::new(buffer), n, Ptr::new(0), 0), 1);
    }

    fn close(e: &mut Engine, h: u32) {
        assert_eq!(super::super::close_handle(e, h), 1);
    }

    fn attributes_of(e: &mut Engine, path: &str) -> u32 {
        let name = text(e, path);
        get_file_attributes_a(e, name)
    }

    fn rename(e: &mut Engine, from: &str, to: &str) -> u32 {
        let (a, b) = (text(e, from), text(e, to));
        move_file_a(e, a, b)
    }

    fn delete(e: &mut Engine, path: &str) -> u32 {
        let name = text(e, path);
        delete_file_a(e, name)
    }

    /// One listed entry: name, short name, attributes, size, creation, last access, last write.
    type Listed = (String, String, u32, u64, u64, u64, u64);

    fn read_entry(e: &Engine, d: u32) -> Listed {
        let time = |at: u32| u64::from(e.mem.u32(at)) | (u64::from(e.mem.u32(at + 4)) << 32);
        (
            String::from_utf8(e.mem.cstr(d + 0x2c).to_vec()).unwrap(),
            String::from_utf8(e.mem.cstr(d + 0x130).to_vec()).unwrap(),
            e.mem.u32(d),
            (u64::from(e.mem.u32(d + 0x1c)) << 32) | u64::from(e.mem.u32(d + 0x20)),
            time(d + 4),
            time(d + 0xc),
            time(d + 0x14),
        )
    }

    /// Every entry `FindFirstFileA`/`FindNextFileA` answer for `pattern`, then the search runs
    /// out with `ERROR_NO_MORE_FILES` and is closed; empty when the first call finds nothing.
    fn listed(e: &mut Engine, pattern: &str) -> Vec<Listed> {
        let name = text(e, pattern);
        let data = e.mem.alloc(0x140);
        let h = find_first_file_a(e, name, Ptr::new(data));
        if h == INVALID_HANDLE_VALUE {
            return vec![];
        }
        let mut all = vec![read_entry(e, data)];
        while find_next_file_a(e, h, Ptr::new(data)) == 1 {
            all.push(read_entry(e, data));
        }
        assert_eq!(e.kernel.last_error, ERROR_NO_MORE_FILES);
        assert_eq!(find_close(e, h), 1);
        all
    }

    /// The calls the game makes to save `name` into `dir` (the port's record of console
    /// `save nvfix`, `findfile-2026-11-03.txt`): the temporary file made and closed, opened
    /// again, written (`sizes`), a seek, closed; an older `NAME.fos` renamed to `NAME.fos.bak`
    /// over the old backup (`CloseSaveFile`), the temporary file renamed to `NAME.fos`. A second
    /// passes before each call. Answers the times of the make and of the last write.
    fn save(e: &mut Engine, dir: &str, name: &str, sizes: &[u32]) -> (u64, u64) {
        let base = format!("{dir}\\{name}.fos");
        let tmp = format!("{base}.tmp");
        let bak = format!("{base}.bak");
        let rw = GENERIC_READ | GENERIC_WRITE;
        tick(e);
        assert_eq!(attributes_of(e, &tmp), INVALID_FILE_ATTRIBUTES);
        assert_eq!(open(e, &tmp, rw, OPEN_EXISTING), INVALID_HANDLE_VALUE);
        let made = tick(e);
        let h = open(e, &tmp, GENERIC_WRITE, CREATE_ALWAYS);
        close(e, h);
        tick(e);
        let h = open(e, &tmp, rw, OPEN_EXISTING);
        let mut last = 0;
        for &n in sizes {
            last = tick(e);
            write(e, h, n);
        }
        tick(e);
        set_file_pointer(e, h, 0x10, Ptr::new(0), 0);
        tick(e);
        close(e, h);
        tick(e);
        if attributes_of(e, &base) != INVALID_FILE_ATTRIBUTES {
            if attributes_of(e, &bak) != INVALID_FILE_ATTRIBUTES {
                assert_eq!(delete(e, &bak), 1);
            }
            assert_eq!(rename(e, &base, &bak), 1);
        }
        assert_eq!(rename(e, &tmp, &base), 1);
        (made, last)
    }

    /// The player's file as the disk keeps it: its short name, attributes and three times.
    #[cfg(windows)]
    fn real_entry(path: &str) -> (String, u32, u64, u64, u64) {
        let h = &host_search(std::path::Path::new(path), path).unwrap()[0];
        (
            h.short.clone(),
            h.attributes,
            h.created,
            h.accessed,
            h.written,
        )
    }

    /// The console's `save nvfix` beside a save of the player's, then the pause menu's
    /// `Saves\*.fos` (`007cbaf0`) and the save list's look at the new file (`00846a20`): the new
    /// save is listed first (upper-case order), `ARCHIVE`, no short name (a valid 8.3 name), its
    /// size, made at the `CREATE_ALWAYS`, last written (and accessed) at its last `WriteFile`;
    /// the temporary file is gone. Before, the search stopped the run ("a directory the game
    /// changed in this run is not provided").
    #[cfg(windows)]
    #[test]
    fn a_save_made_in_the_run_is_listed_in_order_with_its_size_and_times() {
        let dir = scratch("listed", &[(SAVE_138, 300)]);
        let mut e = Engine::new();
        let (made, last) = save(&mut e, &dir, "nvfix", &[0x10000, 0x5c000]);
        let list = listed(&mut e, &format!("{dir}\\*.fos"));
        let names: Vec<&str> = list.iter().map(|l| l.0.as_str()).collect();
        assert_eq!(names, ["nvfix.fos", SAVE_138]);
        assert_eq!(
            list[0],
            (
                "nvfix.fos".into(),
                String::new(),
                FILE_ATTRIBUTE_ARCHIVE,
                0x6c000,
                made,
                last,
                last
            )
        );
        assert_eq!(list[1].3, 300, "the player's save as on disk");
        // The file by its name, as 00846a20 reads its last-write time.
        assert_eq!(
            listed(&mut e, &format!("{dir}\\nvfix.fos")),
            [list[0].clone()]
        );
        assert!(listed(&mut e, &format!("{dir}\\*.tmp")).is_empty());
        assert_eq!(e.kernel.last_error, ERROR_FILE_NOT_FOUND);
        // A save by a name that needs a short name gets the one Windows makes.
        save(&mut e, &dir, "Quicksave", &[0x100]);
        let list = listed(&mut e, &format!("{dir}\\*.fos"));
        let names: Vec<&str> = list.iter().map(|l| l.0.as_str()).collect();
        assert_eq!(names, ["nvfix.fos", "Quicksave.fos", SAVE_138]);
        #[cfg(windows)]
        assert_eq!(list[1].1, "QUICKS~1.FOS");
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// While the handle that writes is open a search shows the file as made (size 0, the
    /// make's times); a rewrite (`CREATE_ALWAYS`, fewer bytes) shows its new size and write
    /// time with the creation time kept; an open for writing with no write and a read change
    /// nothing (measured, `findfile-2026-11-03.txt` 2).
    #[test]
    fn a_rewritten_file_is_listed_with_its_new_size_after_the_close() {
        let dir = scratch("rewritten", &[]);
        let mut e = Engine::new();
        let path = format!("{dir}\\nvfix.fos");
        let made = tick(&mut e);
        let h = open(&mut e, &path, GENERIC_WRITE, CREATE_ALWAYS);
        tick(&mut e);
        write(&mut e, h, 0x400);
        let open_list = listed(&mut e, &path);
        assert_eq!((open_list[0].3, open_list[0].6), (0, made));
        let first = tick(&mut e);
        write(&mut e, h, 0x400);
        close(&mut e, h);
        let before = listed(&mut e, &path);
        assert_eq!(
            (before[0].3, before[0].4, before[0].5, before[0].6),
            (0x800, made, first, first)
        );
        tick(&mut e);
        let h = open(&mut e, &path, GENERIC_READ | GENERIC_WRITE, OPEN_EXISTING);
        let buffer = e.mem.alloc(0x10);
        read_file(&mut e, h, Ptr::new(buffer), 0x10, Ptr::new(0), 0);
        close(&mut e, h);
        assert_eq!(listed(&mut e, &path), before, "no write: nothing changes");
        let emptied_at = tick(&mut e);
        let h = open(&mut e, &path, GENERIC_WRITE, CREATE_ALWAYS);
        assert_eq!(e.kernel.last_error, ERROR_ALREADY_EXISTS);
        let rewritten = tick(&mut e);
        write(&mut e, h, 0x30);
        let during = listed(&mut e, &path);
        assert_eq!((during[0].3, during[0].6), (0, emptied_at));
        close(&mut e, h);
        let after = listed(&mut e, &path);
        assert_eq!(
            (after[0].3, after[0].4, after[0].5, after[0].6),
            (0x30, made, rewritten, rewritten)
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// A deleted file of the player's is not listed (`*.fos` after DeleteFileA); one renamed
    /// away is listed by its new name only.
    #[cfg(windows)]
    #[test]
    fn a_deleted_file_is_not_listed() {
        let dir = scratch("deleted", &[(SAVE_138, 300), ("Old.fos", 10)]);
        let mut e = Engine::new();
        tick(&mut e);
        save(&mut e, &dir, "nvfix", &[0x100]);
        assert_eq!(delete(&mut e, &format!("{dir}\\{SAVE_138}")), 1);
        let names: Vec<String> = listed(&mut e, &format!("{dir}\\*.fos"))
            .into_iter()
            .map(|l| l.0)
            .collect();
        assert_eq!(names, ["nvfix.fos", "Old.fos"]);
        tick(&mut e);
        assert_eq!(
            rename(
                &mut e,
                &format!("{dir}\\Old.fos"),
                &format!("{dir}\\Older.fos")
            ),
            1
        );
        let names: Vec<String> = listed(&mut e, &format!("{dir}\\*.fos"))
            .into_iter()
            .map(|l| l.0)
            .collect();
        assert_eq!(names, ["nvfix.fos", "Older.fos"]);
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// An autosave over an older one: `Autosave.fos.bak` deleted, `Autosave.fos` renamed to
    /// it, the new file renamed to `Autosave.fos`. Tunnelling gives the new `Autosave.fos` the
    /// old one's creation time and the renamed backup the deleted backup's (measured,
    /// `findfile-2026-11-03.txt` 3); the backup keeps the old file's size and write time.
    #[cfg(windows)]
    #[test]
    fn an_autosave_over_an_older_one_takes_the_old_creation_times() {
        let dir = scratch("autosave", &[("Autosave.fos.bak", 100)]);
        std::thread::sleep(std::time::Duration::from_millis(50));
        std::fs::write(format!("{dir}\\Autosave.fos"), vec![b'o'; 200]).unwrap();
        let old_backup = real_entry(&format!("{dir}\\Autosave.fos.bak"));
        let old = real_entry(&format!("{dir}\\Autosave.fos"));
        assert_ne!(old.2, old_backup.2, "two creation times to tell apart");
        let mut e = Engine::new();
        let (_, last) = save(&mut e, &dir, "Autosave", &[0x1000]);
        let list = listed(&mut e, &format!("{dir}\\Autosave.fos*"));
        assert_eq!(
            list,
            [
                (
                    "Autosave.fos".into(),
                    String::new(),
                    FILE_ATTRIBUTE_ARCHIVE,
                    0x1000,
                    old.2,
                    last,
                    last
                ),
                (
                    "Autosave.fos.bak".into(),
                    old_backup.0.clone(),
                    old.1,
                    200,
                    old_backup.2,
                    old.3,
                    old.4
                ),
            ]
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// Short names as NTFS makes them (`NAME~1` to `~4`, then the checksum name; measured
    /// `S1`), a short name and creation time tunnelled to the same name made again within
    /// 15 s, and neither after 16 s (`U`, `W`).
    #[cfg(windows)]
    #[test]
    fn new_names_get_windows_short_names_and_tunnelling() {
        let mut e = Engine::new();
        let dir = r"Q:\nv-rs-made-dir";
        let name = text(&mut e, dir);
        assert_eq!(create_directory_a(&mut e, name, 0), 1);
        let make = |e: &mut Engine, name: &str| {
            let h = open(e, &format!("{dir}\\{name}"), GENERIC_WRITE, CREATE_NEW);
            assert_ne!(h, INVALID_HANDLE_VALUE);
            close(e, h);
        };
        for n in 140..=144 {
            tick(&mut e);
            make(&mut e, &format!("Save {n} x.fos"));
        }
        let shorts = |e: &mut Engine| -> Vec<(String, String, u64)> {
            listed(e, &format!("{dir}\\*.fos"))
                .into_iter()
                .map(|l| (l.0, l.1, l.4))
                .collect()
        };
        let before = shorts(&mut e);
        let names: Vec<&str> = before.iter().map(|l| l.1.as_str()).collect();
        assert_eq!(
            names,
            [
                "SAVE14~1.FOS",
                "SAVE14~2.FOS",
                "SAVE14~3.FOS",
                "SAVE14~4.FOS",
                "SAEB56~1.FOS"
            ]
        );
        // Renamed away, made again 1 s later: the old creation time and SAVE14~2.
        tick(&mut e);
        rename(
            &mut e,
            &format!("{dir}\\Save 141 x.fos"),
            &format!("{dir}\\x.tmp"),
        );
        tick(&mut e);
        make(&mut e, "Save 141 x.fos");
        // Renamed away, made again 16 s later: a new creation time, the free SAVE14~3.
        rename(
            &mut e,
            &format!("{dir}\\Save 142 x.fos"),
            &format!("{dir}\\y.tmp"),
        );
        let later = e.kernel.disk.clock.unwrap() + 16 * SECOND;
        e.kernel.disk.clock = Some(later);
        make(&mut e, "Save 142 x.fos");
        let after = shorts(&mut e);
        assert_eq!(after[1], before[1], "tunnelled within 15 s");
        assert_eq!(
            after[2],
            ("Save 142 x.fos".into(), "SAVE14~3.FOS".into(), later)
        );
        assert_eq!(listed(&mut e, &format!("{dir}\\*.tmp")).len(), 2);
    }

    /// A write of no bytes, then the close: the size and the three times stay as they were
    /// (measured, `review-findfile/run1.txt` `zero:`).
    #[test]
    fn a_write_of_no_bytes_changes_no_time() {
        let dir = scratch("zero", &[]);
        let mut e = Engine::new();
        let path = format!("{dir}\\z.bin");
        tick(&mut e);
        let h = open(&mut e, &path, GENERIC_WRITE, CREATE_ALWAYS);
        write(&mut e, h, 10);
        close(&mut e, h);
        let before = listed(&mut e, &path);
        assert_eq!(before[0].3, 10);
        tick(&mut e);
        let h = open(&mut e, &path, GENERIC_WRITE, OPEN_EXISTING);
        tick(&mut e);
        write(&mut e, h, 0);
        close(&mut e, h);
        assert_eq!(listed(&mut e, &path), before);
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// A file made in the folder between `FindFirstFileA` and `FindNextFileA`: Windows answers
    /// from the folder as it is now or from what it read before, depending on how far the search
    /// got (measured, `review-findfile/run1.txt`); not modelled, the run stops.
    #[test]
    #[should_panic(expected = "since FindFirstFileA; not provided")]
    fn a_search_whose_folder_changed_since_it_began_stops() {
        let dir = scratch("midsearch", &[("a.fos", 5), ("b.fos", 5)]);
        let mut e = Engine::new();
        let name = text(&mut e, &format!("{dir}\\*.fos"));
        let data = e.mem.alloc(0x140);
        let h = find_first_file_a(&mut e, name, Ptr::new(data));
        assert_ne!(h, INVALID_HANDLE_VALUE);
        tick(&mut e);
        let made = open(
            &mut e,
            &format!("{dir}\\zz-late.fos"),
            GENERIC_WRITE,
            CREATE_NEW,
        );
        close(&mut e, made);
        let _ = std::fs::remove_dir_all(&dir);
        find_next_file_a(&mut e, h, Ptr::new(data));
    }

    /// `.` of a directory the game changed is not listed as Windows would (its times move with
    /// the changes, not modelled): the run stops.
    #[test]
    #[should_panic(expected = "of a directory the game changed in this run")]
    fn a_search_that_finds_dot_in_a_changed_directory_stops() {
        let dir = scratch("dot", &[]);
        let mut e = Engine::new();
        save(&mut e, &dir, "nvfix", &[0x10]);
        listed(&mut e, &format!("{dir}\\*"));
    }
}
