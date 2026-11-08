//! The co-save's file format (`.nvse`), as xNVSE 6.4.9 writes and reads it (`Serialization.cpp`
//! (xNVSE)); no engine here.
//!
//! ```text
//! Header       u32 signature "NVSE" (the bytes), u32 format 1, u16 6, u16 4, u32 0x040020D0,
//!              u32 the number of plugins written
//! per plugin   u32 opcode base (0x1400 for xNVSE itself), u32 chunks, u32 length (of what
//!              follows, chunk headers included)
//!   per chunk  u32 type (a multi-character literal: 'MODS' is stored as the bytes "SDOM"),
//!              u32 version, u32 length, then the data
//! ```
//!
//! A plugin that wrote no chunk is left out (and not counted). The reader is xNVSE's stateful
//! one: [`Reader::next_plugin`] reads plugin headers while 12 bytes remain and stops at one of
//! length 0; [`Reader::next_record`] skips what is left of the open chunk and reads the next
//! chunk header of the plugin; the record reads give 0 (or nothing, for 8 bytes) when the chunk
//! has fewer bytes left than asked for, and read past the file's end as xNVSE's `std::out_of_range`
//! ([`OutOfRange`]), which ends the load where it is.

/// `Header::kSignature` (`MACRO_SWAP32('NVSE')`): the bytes "NVSE".
pub const SIGNATURE: u32 = u32::from_le_bytes(*b"NVSE");
/// `Header::kVersion`.
pub const FORMAT_VERSION: u32 = 1;
/// `NVSE_VERSION_INTEGER`, `NVSE_VERSION_INTEGER_MINOR`.
pub const NVSE_VERSION: (u16, u16) = (6, 4);
/// `RUNTIME_VERSION` for 1.4.0.525 (`MAKE_NEW_VEGAS_VERSION(4, 0, 525)`).
pub const RUNTIME_VERSION: u32 = 0x0400_20d0;
/// `kNvseOpcodeBase`: xNVSE's own plugin header.
pub const CORE_OPCODE_BASE: u32 = 0x1400;
/// The header's and each plugin and chunk header's sizes.
pub const HEADER_SIZE: usize = 20;
pub const PLUGIN_HEADER_SIZE: usize = 12;
pub const CHUNK_HEADER_SIZE: usize = 12;

/// A chunk type as the C++ multi-character literal `'MODS'` makes it.
pub const fn tag(t: &[u8; 4]) -> u32 {
    u32::from_be_bytes(*t)
}

/// A chunk to write.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Chunk {
    pub kind: u32,
    pub version: u32,
    pub data: Vec<u8>,
}

/// A plugin's chunks to write.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Plugin {
    pub opcode_base: u32,
    pub chunks: Vec<Chunk>,
}

/// Another plugin's section as a co-save held it: its header's opcode base and chunk count, and
/// its bytes after the header (chunk headers included), kept as they were.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Kept {
    pub opcode_base: u32,
    pub chunks: u32,
    pub data: Vec<u8>,
}

/// [`write_with`] with no kept section.
pub fn write(plugins: &[Plugin]) -> Vec<u8> {
    write_with(plugins, &[])
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Serialization.cpp: HandleSaveGame,
// OpenRecord, FlushWriteChunk.
/// The file's bytes: the header, then each plugin that has chunks, then the kept sections of
/// other plugins byte for byte (the fixes preset's; xNVSE writes only loaded plugins).
pub fn write_with(plugins: &[Plugin], kept: &[Kept]) -> Vec<u8> {
    let mut out = vec![0; HEADER_SIZE];
    let mut count = 0u32;
    for p in plugins.iter().filter(|p| !p.chunks.is_empty()) {
        count += 1;
        let length: usize = p
            .chunks
            .iter()
            .map(|c| CHUNK_HEADER_SIZE + c.data.len())
            .sum();
        out.extend_from_slice(&p.opcode_base.to_le_bytes());
        out.extend_from_slice(&(p.chunks.len() as u32).to_le_bytes());
        out.extend_from_slice(&(length as u32).to_le_bytes());
        for c in &p.chunks {
            out.extend_from_slice(&c.kind.to_le_bytes());
            out.extend_from_slice(&c.version.to_le_bytes());
            out.extend_from_slice(&(c.data.len() as u32).to_le_bytes());
            out.extend_from_slice(&c.data);
        }
    }
    for k in kept {
        count += 1;
        out.extend_from_slice(&k.opcode_base.to_le_bytes());
        out.extend_from_slice(&k.chunks.to_le_bytes());
        out.extend_from_slice(&(k.data.len() as u32).to_le_bytes());
        out.extend_from_slice(&k.data);
    }
    out[0..4].copy_from_slice(&SIGNATURE.to_le_bytes());
    out[4..8].copy_from_slice(&FORMAT_VERSION.to_le_bytes());
    out[8..10].copy_from_slice(&NVSE_VERSION.0.to_le_bytes());
    out[10..12].copy_from_slice(&NVSE_VERSION.1.to_le_bytes());
    out[12..16].copy_from_slice(&RUNTIME_VERSION.to_le_bytes());
    out[16..20].copy_from_slice(&count.to_le_bytes());
    out
}

/// A chunk's data being written (`WriteRecord8`, ...).
#[derive(Default)]
pub struct ChunkWriter {
    pub data: Vec<u8>,
}

impl ChunkWriter {
    pub fn u8(&mut self, v: u8) {
        self.data.push(v);
    }
    pub fn u16(&mut self, v: u16) {
        self.data.extend_from_slice(&v.to_le_bytes());
    }
    pub fn u32(&mut self, v: u32) {
        self.data.extend_from_slice(&v.to_le_bytes());
    }
    pub fn f64(&mut self, v: f64) {
        self.data.extend_from_slice(&v.to_le_bytes());
    }
    pub fn bytes(&mut self, b: &[u8]) {
        self.data.extend_from_slice(b);
    }
}

/// A read past the end of the file (`ValidateOffset`'s `std::out_of_range`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OutOfRange;

/// Why a file is not read at all (`HandleLoadGame`'s checks before the plugins).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refused {
    /// Shorter than its header (the header read throws).
    Short,
    Signature(u32),
    VersionInvalid(u32),
    VersionTooNew(u32),
}

/// A plugin header (`PluginHeader`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PluginHeader {
    pub opcode_base: u32,
    pub chunks: u32,
    pub length: u32,
}

/// xNVSE's reader over a whole file.
pub struct Reader<'a> {
    buf: &'a [u8],
    pos: usize,
    /// The plugin being read: its chunks not yet started.
    chunks_left: u32,
    /// The open chunk's bytes not yet read, if one is open.
    chunk_left: Option<u32>,
}

impl<'a> Reader<'a> {
    // Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Serialization.cpp: HandleLoadGame
    // (the header's checks).
    pub fn open(buf: &'a [u8]) -> Result<Reader<'a>, Refused> {
        if buf.len() < HEADER_SIZE {
            return Err(Refused::Short);
        }
        let word = |at: usize| u32::from_le_bytes(buf[at..at + 4].try_into().unwrap());
        let signature = word(0);
        if signature != SIGNATURE {
            return Err(Refused::Signature(signature));
        }
        let version = word(4);
        if version == 0 {
            return Err(Refused::VersionInvalid(version));
        }
        if version > FORMAT_VERSION {
            return Err(Refused::VersionTooNew(version));
        }
        Ok(Reader {
            buf,
            pos: HEADER_SIZE,
            chunks_left: 0,
            chunk_left: None,
        })
    }

    pub fn offset(&self) -> usize {
        self.pos
    }

    /// `SetOffset` (reading: no resize, a later read past the end throws).
    pub fn set_offset(&mut self, at: usize) {
        self.pos = at;
    }

    fn take(&mut self, n: usize) -> Result<&'a [u8], OutOfRange> {
        if self.pos + n > self.buf.len() {
            return Err(OutOfRange);
        }
        let s = &self.buf[self.pos..self.pos + n];
        self.pos += n;
        Ok(s)
    }

    fn word(&mut self) -> Result<u32, OutOfRange> {
        Ok(u32::from_le_bytes(self.take(4)?.try_into().unwrap()))
    }

    /// The next plugin header while at least 12 bytes remain (`GetRemain() >= sizeof`); `None`
    /// at the end, or at a header of length 0 (xNVSE: "cosave header has size 0", stop).
    pub fn next_plugin(&mut self) -> Result<Option<PluginHeader>, OutOfRange> {
        if self.buf.len().saturating_sub(self.pos) < PLUGIN_HEADER_SIZE {
            return Ok(None);
        }
        let h = PluginHeader {
            opcode_base: self.word()?,
            chunks: self.word()?,
            length: self.word()?,
        };
        if h.length == 0 {
            return Ok(None);
        }
        self.chunks_left = h.chunks;
        self.chunk_left = None;
        Ok(Some(h))
    }

    /// A plugin's data as it is (for keeping it); the read is checked as `Skip` checks it.
    pub fn take_plugin(&mut self, n: u32) -> Result<Vec<u8>, OutOfRange> {
        self.take(n as usize).map(<[u8]>::to_vec)
    }

    /// `Skip(length, true)`: past a plugin's data (the read is checked).
    pub fn skip(&mut self, n: u32) -> Result<(), OutOfRange> {
        self.take(n as usize).map(|_| ())
    }

    /// Ends the plugin: no chunk open, none left (`s_chunkOpen = false`).
    pub fn end_plugin(&mut self) {
        self.chunks_left = 0;
        self.chunk_left = None;
    }

    /// For a plugin that had no data: its callback reads no record.
    pub fn no_records(&mut self) {
        self.end_plugin();
    }

    // Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Serialization.cpp:
    // GetNextRecordInfo, FlushReadRecord.
    /// The next chunk's type, version and length; what is left of the open one is skipped.
    pub fn next_record(&mut self) -> Result<Option<(u32, u32, u32)>, OutOfRange> {
        if let Some(left) = self.chunk_left.take() {
            if left != 0 {
                self.skip(left)?;
            }
        }
        if self.chunks_left == 0 {
            return Ok(None);
        }
        self.chunks_left -= 1;
        let kind = self.word()?;
        let version = self.word()?;
        let length = self.word()?;
        self.chunk_left = Some(length);
        Ok(Some((kind, version, length)))
    }

    /// The open chunk's bytes not yet read (0 with none open): a count of items that each take
    /// at least a byte cannot be larger.
    pub fn left(&self) -> u32 {
        self.chunk_left.unwrap_or(0)
    }

    fn take_record(&mut self, n: u32) -> Result<Option<&'a [u8]>, OutOfRange> {
        if self.left() < n {
            return Ok(None);
        }
        let s = self.take(n as usize)?;
        self.chunk_left = Some(self.left() - n);
        Ok(Some(s))
    }

    /// `ReadRecord8`: 0 when the chunk has no byte left.
    pub fn read8(&mut self) -> Result<u8, OutOfRange> {
        Ok(self.take_record(1)?.map_or(0, |s| s[0]))
    }

    pub fn read16(&mut self) -> Result<u16, OutOfRange> {
        Ok(self
            .take_record(2)?
            .map_or(0, |s| u16::from_le_bytes(s.try_into().unwrap())))
    }

    pub fn read32(&mut self) -> Result<u32, OutOfRange> {
        Ok(self
            .take_record(4)?
            .map_or(0, |s| u32::from_le_bytes(s.try_into().unwrap())))
    }

    /// `ReadRecord64`: nothing (the caller's value stays) with fewer than 8 bytes left.
    pub fn read64(&mut self) -> Result<Option<f64>, OutOfRange> {
        Ok(self
            .take_record(8)?
            .map(|s| f64::from_le_bytes(s.try_into().unwrap())))
    }

    /// `ReadRecordData`: up to `n` bytes, as many as the chunk has.
    pub fn read_data(&mut self, n: u32) -> Result<Vec<u8>, OutOfRange> {
        let n = n.min(self.left());
        let s = self.take(n as usize)?;
        self.chunk_left = Some(self.left() - n);
        Ok(s.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The layout: header, plugin header with the chunks' total, chunk headers, the multi-char
    /// type's bytes.
    #[test]
    fn writes_xnvse_layout() {
        let bytes = write(&[
            Plugin {
                opcode_base: CORE_OPCODE_BASE,
                chunks: vec![
                    Chunk {
                        kind: tag(b"MODS"),
                        version: 0,
                        data: vec![1, 2, 0, b'a', b'b'],
                    },
                    Chunk {
                        kind: tag(b"ARVS"),
                        version: 2,
                        data: Vec::new(),
                    },
                ],
            },
            Plugin {
                opcode_base: 0x2200,
                chunks: Vec::new(),
            },
        ]);
        let mut want = b"NVSE".to_vec();
        want.extend([1, 0, 0, 0, 6, 0, 4, 0, 0xd0, 0x20, 0, 4, 1, 0, 0, 0]);
        want.extend([0, 0x14, 0, 0, 2, 0, 0, 0, 29, 0, 0, 0]);
        want.extend(b"SDOM");
        want.extend([0, 0, 0, 0, 5, 0, 0, 0, 1, 2, 0, b'a', b'b']);
        want.extend(b"SVRA");
        want.extend([2, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(bytes, want);
    }

    /// Reading: records give 0 past their chunk's end, the open chunk's rest is skipped, a read
    /// past the file's end is `OutOfRange`.
    #[test]
    fn reads_records_as_xnvse() {
        let bytes = write(&[Plugin {
            opcode_base: CORE_OPCODE_BASE,
            chunks: vec![
                Chunk {
                    kind: tag(b"STVR"),
                    version: 0,
                    data: vec![5, 9, 0, 0, 0],
                },
                Chunk {
                    kind: tag(b"STVE"),
                    version: 0,
                    data: vec![],
                },
            ],
        }]);
        let mut r = Reader::open(&bytes).unwrap();
        let p = r.next_plugin().unwrap().unwrap();
        assert_eq!((p.opcode_base, p.chunks, p.length), (0x1400, 2, 29));
        assert_eq!(r.next_record().unwrap(), Some((tag(b"STVR"), 0, 5)));
        assert_eq!(r.read8().unwrap(), 5);
        assert_eq!(r.read16().unwrap(), 9);
        assert_eq!(r.read32().unwrap(), 0, "2 bytes left: 0, nothing read");
        assert_eq!(r.read64().unwrap(), None);
        assert_eq!(r.next_record().unwrap(), Some((tag(b"STVE"), 0, 0)));
        assert_eq!(r.next_record().unwrap(), None);
        assert_eq!(r.next_plugin().unwrap(), None);
        assert_eq!(Reader::open(b"NVSX").err(), Some(Refused::Short));
        let mut bad = bytes.clone();
        bad[4] = 2;
        assert_eq!(Reader::open(&bad).err(), Some(Refused::VersionTooNew(2)));
        // A chunk header claiming more than the file has: the skip throws.
        let mut cut = bytes.clone();
        cut.truncate(HEADER_SIZE + PLUGIN_HEADER_SIZE + CHUNK_HEADER_SIZE + 2);
        let mut r = Reader::open(&cut).unwrap();
        r.next_plugin().unwrap();
        r.next_record().unwrap();
        assert_eq!(r.next_record(), Err(OutOfRange));
    }
}
