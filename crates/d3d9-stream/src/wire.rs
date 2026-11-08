//! The stream as bytes: a file of commands (`NV_D3D9_RECORD`), read back and replayed into any
//! [`Backend`] (a run's first frames recorded on the real device, replayed into another backend in
//! its tests), and the hashes of `NV_D3D9_STREAM_CHECK`.
//!
//! A file is [`MAGIC`], the [`LIST_HASH`] of the command list it was written with (a little-endian
//! `u64`), then one record per command: its length (a little-endian `u32`, the bytes after it),
//! its number (the command's place in the list in `cmd.rs`) and its fields in order: `u32`, `i32`
//! and `isize` (as `i64`) little-endian, `bool` and `u8` one byte, arrays element by element; a
//! payload as its length (`u32`) and its bytes, an optional payload as one byte (0 none, 1 some)
//! and the payload. The numbers and layouts follow the list, so a file written with another list
//! (a command added, moved or changed) is refused, never read with other meanings. Each command
//! read is checked ([`Cmd::check`]): its payloads have the sizes the call gives them.

use crate::cmd::{Backend, Cmd, Data, LIST};

/// The first bytes of a file of commands.
pub const MAGIC: &[u8; 8] = b"NVD3D9S1";

/// The hash of the command list this build encodes with (its text: names, fields, kinds).
pub const LIST_HASH: u64 = hash_const(LIST.as_bytes());

/// [`hash`] at compile time.
const fn hash_const(bytes: &[u8]) -> u64 {
    let mut h = HASH_START;
    let mut i = 0;
    while i < bytes.len() {
        h ^= bytes[i] as u64;
        h = h.wrapping_mul(0x0100_0000_01b3);
        i += 1;
    }
    h
}

/// Why bytes could not be read as commands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WireError {
    /// The bytes do not start with [`MAGIC`].
    NotAStream,
    /// The file was written with another command list (its [`LIST_HASH`]).
    OtherList { file: u64, this: u64 },
    /// The bytes end inside a record that starts at `at`.
    Truncated { at: usize },
    /// No command has this number.
    UnknownCommand { at: usize, tag: u8 },
    /// The record at `at` is not as long as it says.
    Length { at: usize },
    /// The command at `at` carries a payload of another size than its call gives it.
    Payload { at: usize, why: String },
}

impl std::fmt::Display for WireError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WireError::NotAStream => write!(f, "not a Direct3D 9 command stream (no {MAGIC:?})"),
            WireError::OtherList { file, this } => write!(
                f,
                "written with another command list ({file:016x}; this build's is {this:016x})"
            ),
            WireError::Truncated { at } => write!(f, "the stream ends inside the record at {at}"),
            WireError::UnknownCommand { at, tag } => {
                write!(f, "unknown command number {tag} at {at}")
            }
            WireError::Length { at } => write!(f, "the record at {at} has another length"),
            WireError::Payload { at, why } => write!(f, "the record at {at}: {why}"),
        }
    }
}

impl std::error::Error for WireError {}

/// A plain field's encoding.
pub(crate) trait Wire: Sized {
    fn put(&self, out: &mut Vec<u8>);
    fn get(r: &mut Reader<'_>) -> Result<Self, WireError>;
}

impl Wire for u32 {
    fn put(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.to_le_bytes());
    }
    fn get(r: &mut Reader<'_>) -> Result<Self, WireError> {
        Ok(u32::from_le_bytes(r.take(4)?.try_into().expect("4 bytes")))
    }
}

impl Wire for i32 {
    fn put(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.to_le_bytes());
    }
    fn get(r: &mut Reader<'_>) -> Result<Self, WireError> {
        Ok(i32::from_le_bytes(r.take(4)?.try_into().expect("4 bytes")))
    }
}

impl Wire for isize {
    fn put(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&(*self as i64).to_le_bytes());
    }
    fn get(r: &mut Reader<'_>) -> Result<Self, WireError> {
        Ok(i64::from_le_bytes(r.take(8)?.try_into().expect("8 bytes")) as isize)
    }
}

impl Wire for u8 {
    fn put(&self, out: &mut Vec<u8>) {
        out.push(*self);
    }
    fn get(r: &mut Reader<'_>) -> Result<Self, WireError> {
        Ok(r.take(1)?[0])
    }
}

impl Wire for bool {
    fn put(&self, out: &mut Vec<u8>) {
        out.push(u8::from(*self));
    }
    fn get(r: &mut Reader<'_>) -> Result<Self, WireError> {
        Ok(r.take(1)?[0] != 0)
    }
}

impl<T: Wire + Copy + Default, const N: usize> Wire for [T; N] {
    fn put(&self, out: &mut Vec<u8>) {
        for x in self {
            x.put(out);
        }
    }
    fn get(r: &mut Reader<'_>) -> Result<Self, WireError> {
        let mut a = [T::default(); N];
        for x in &mut a {
            *x = T::get(r)?;
        }
        Ok(a)
    }
}

/// A payload: its length and its bytes.
pub(crate) fn put_data(d: &Data<'_>, out: &mut Vec<u8>) {
    let len = d.len();
    len.put(out);
    let at = out.len();
    out.resize(at + len as usize, 0);
    d.copy_to(0, &mut out[at..]);
}

/// An optional payload: 0, or 1 and the payload.
pub(crate) fn put_optional(d: &Option<Data<'_>>, out: &mut Vec<u8>) {
    match d {
        None => out.push(0),
        Some(d) => {
            out.push(1);
            put_data(d, out);
        }
    }
}

/// Reads fields from bytes.
pub(crate) struct Reader<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl<'a> Reader<'a> {
    /// Where it reads next.
    pub(crate) fn at(&self) -> usize {
        self.at
    }

    fn take(&mut self, n: usize) -> Result<&'a [u8], WireError> {
        let end = self.at.checked_add(n).filter(|&e| e <= self.bytes.len());
        let Some(end) = end else {
            return Err(WireError::Truncated { at: self.at });
        };
        let b = &self.bytes[self.at..end];
        self.at = end;
        Ok(b)
    }

    pub(crate) fn data(&mut self) -> Result<Data<'a>, WireError> {
        let len = u32::get(self)?;
        Ok(Data::Bytes(self.take(len as usize)?))
    }

    pub(crate) fn optional(&mut self) -> Result<Option<Data<'a>>, WireError> {
        match u8::get(self)? {
            0 => Ok(None),
            _ => Ok(Some(self.data()?)),
        }
    }
}

/// Appends `cmd`'s record (its length, then the command) to `out`.
pub fn encode(cmd: &Cmd<Data<'_>>, out: &mut Vec<u8>) {
    let at = out.len();
    out.extend_from_slice(&[0; 4]);
    cmd.put(out);
    let len = (out.len() - at - 4) as u32;
    out[at..at + 4].copy_from_slice(&len.to_le_bytes());
}

/// A file's first bytes: [`MAGIC`] and this build's [`LIST_HASH`].
pub fn header() -> [u8; 16] {
    let mut h = [0; 16];
    h[..8].copy_from_slice(MAGIC);
    h[8..].copy_from_slice(&LIST_HASH.to_le_bytes());
    h
}

/// The commands of a file of commands (after its [`header`]), in order; each borrows its payloads
/// from `bytes`. A file written with another command list is refused.
pub fn commands(bytes: &[u8]) -> Result<Commands<'_>, WireError> {
    if bytes.len() < 16 || &bytes[..MAGIC.len()] != MAGIC {
        return Err(WireError::NotAStream);
    }
    let file = u64::from_le_bytes(bytes[8..16].try_into().expect("8 bytes"));
    if file != LIST_HASH {
        return Err(WireError::OtherList {
            file,
            this: LIST_HASH,
        });
    }
    Ok(Commands {
        r: Reader { bytes, at: 16 },
        failed: false,
    })
}

/// The commands of a file, one record at a time ([`commands`]).
pub struct Commands<'a> {
    r: Reader<'a>,
    failed: bool,
}

impl<'a> Iterator for Commands<'a> {
    type Item = Result<Cmd<Data<'a>>, WireError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.failed || self.r.at == self.r.bytes.len() {
            return None;
        }
        let item = (|| {
            let start = self.r.at;
            let len = u32::get(&mut self.r)? as usize;
            let end = self
                .r
                .at
                .checked_add(len)
                .filter(|&end| end <= self.r.bytes.len())
                .ok_or(WireError::Truncated { at: start })?;
            let tag = u8::get(&mut self.r)?;
            let cmd = Cmd::get(tag, &mut self.r)?;
            if self.r.at != end {
                return Err(WireError::Length { at: start });
            }
            cmd.check()
                .map_err(|why| WireError::Payload { at: start, why })?;
            Ok(cmd)
        })();
        self.failed = item.is_err();
        Some(item)
    }
}

/// Every command of a file of commands carried out by `backend`, in order (the sync ones' replies
/// dropped); how many there were.
pub fn replay(bytes: &[u8], backend: &mut dyn Backend) -> Result<u64, WireError> {
    let mut n = 0;
    for cmd in commands(bytes)? {
        backend.execute(cmd?);
        n += 1;
    }
    Ok(n)
}

/// The 64-bit FNV-1a hash of `bytes` continued from `hash` (start with [`HASH_START`]).
pub(crate) fn hash(mut hash: u64, bytes: &[u8]) -> u64 {
    for &b in bytes {
        hash ^= u64::from(b);
        hash = hash.wrapping_mul(0x0100_0000_01b3);
    }
    hash
}

pub(crate) const HASH_START: u64 = 0xcbf2_9ce4_8422_2325;
