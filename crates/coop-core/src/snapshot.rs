//! Versioned, compressed full-state snapshot (join and fallback path).
//!
//! Layout: magic `CSN`, version byte, codec byte (0 raw, 1 lz4), raw length
//! (u32 LE), payload. The payload is the postcard encoding of [`Snapshot`].
//! The codec byte leaves room for zstd later without a version bump.

use serde::{Deserialize, Serialize};

use crate::delta::{SnapEntry, StateStore};
use crate::error::{Error, Result};
use crate::ids::{Rev, Tick};

/// Current snapshot format version.
pub const SNAPSHOT_VERSION: u8 = 1;
/// Upper bound on a decompressed snapshot.
pub const MAX_SNAPSHOT_BYTES: usize = 64 * 1024 * 1024;
const MAGIC: &[u8; 3] = b"CSN";
const HEADER: usize = 3 + 1 + 1 + 4;
const CODEC_RAW: u8 = 0;
const CODEC_LZ4: u8 = 1;

/// Full copy of the replicated state.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Snapshot {
    /// Host tick of capture.
    pub tick: Tick,
    /// Revision of capture.
    pub rev: Rev,
    /// Every record, ascending by key.
    pub entries: Vec<SnapEntry>,
}

impl Snapshot {
    /// Captures a store.
    pub fn from_store(store: &StateStore, tick: Tick) -> Self {
        Snapshot {
            tick,
            rev: store.rev(),
            entries: store.entries(),
        }
    }

    /// Builds a store whose baseline is the snapshot revision.
    pub fn into_store(self) -> StateStore {
        let mut s = StateStore::new();
        s.load(self.entries, self.rev);
        s
    }

    /// Serialises and compresses.
    pub fn encode(&self) -> Result<Vec<u8>> {
        let raw = postcard::to_allocvec(self)?;
        if raw.len() > MAX_SNAPSHOT_BYTES {
            return Err(Error::TooLarge {
                len: raw.len(),
                max: MAX_SNAPSHOT_BYTES,
            });
        }
        let packed = lz4_flex::block::compress(&raw);
        let (codec, body) = if packed.len() < raw.len() {
            (CODEC_LZ4, packed)
        } else {
            (CODEC_RAW, raw.clone())
        };
        let mut out = Vec::with_capacity(HEADER + body.len());
        out.extend_from_slice(MAGIC);
        out.push(SNAPSHOT_VERSION);
        out.push(codec);
        out.extend_from_slice(&(raw.len() as u32).to_le_bytes());
        out.extend_from_slice(&body);
        Ok(out)
    }

    /// Parses and decompresses, validating every length before allocating.
    pub fn decode(bytes: &[u8]) -> Result<Snapshot> {
        if bytes.len() < HEADER || &bytes[..3] != MAGIC {
            return Err(Error::BadMagic);
        }
        if bytes[3] != SNAPSHOT_VERSION {
            return Err(Error::Version(bytes[3]));
        }
        let raw_len = u32::from_le_bytes([bytes[5], bytes[6], bytes[7], bytes[8]]) as usize;
        if raw_len > MAX_SNAPSHOT_BYTES {
            return Err(Error::TooLarge {
                len: raw_len,
                max: MAX_SNAPSHOT_BYTES,
            });
        }
        let body = &bytes[HEADER..];
        let raw = match bytes[4] {
            CODEC_RAW => {
                if body.len() != raw_len {
                    return Err(Error::Decompress("raw length mismatch".into()));
                }
                body.to_vec()
            }
            CODEC_LZ4 => lz4_flex::block::decompress(body, raw_len)
                .map_err(|e| Error::Decompress(e.to_string()))?,
            other => return Err(Error::Version(other)),
        };
        let (snap, rest) = postcard::take_from_bytes::<Snapshot>(&raw)?;
        if !rest.is_empty() {
            return Err(Error::Trailing);
        }
        if snap.entries.iter().any(|e| !e.field.is_sane()) {
            return Err(Error::Chunk("insane field"));
        }
        Ok(snap)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::FormId;
    use crate::schema::Field;

    fn store(n: u32) -> StateStore {
        let mut s = StateStore::new();
        for i in 0..n {
            s.write_local(
                FormId(0x0100_0000 + i),
                Field::QuestStage {
                    stage: (i % 50) as u16,
                },
            );
            s.write_local(FormId(0x0100_0000 + i), Field::QuestFlags { flags: i });
        }
        s
    }

    #[test]
    fn roundtrip_preserves_entries_and_revs() {
        let s = store(500);
        let bytes = Snapshot::from_store(&s, Tick(7)).encode().unwrap();
        let back = Snapshot::decode(&bytes).unwrap();
        assert_eq!(back.tick, Tick(7));
        let s2 = back.into_store();
        assert_eq!(s2.entries(), s.entries());
        assert_eq!(s2.rev(), s.rev());
        assert_eq!(s2.base(), s.rev());
    }

    #[test]
    fn compresses_repetitive_state() {
        let s = store(2000);
        let snap = Snapshot::from_store(&s, Tick(0));
        let raw = postcard::to_allocvec(&snap).unwrap();
        let enc = snap.encode().unwrap();
        assert!(enc.len() < raw.len());
    }

    #[test]
    fn empty_roundtrip() {
        let bytes = Snapshot::from_store(&StateStore::new(), Tick(0))
            .encode()
            .unwrap();
        assert!(Snapshot::decode(&bytes).unwrap().entries.is_empty());
    }

    #[test]
    fn rejects_garbage() {
        assert!(matches!(Snapshot::decode(b"xx"), Err(Error::BadMagic)));
        assert!(matches!(
            Snapshot::decode(b"XXX\x01\x00\0\0\0\0"),
            Err(Error::BadMagic)
        ));
        let mut b = Snapshot::from_store(&store(10), Tick(0)).encode().unwrap();
        b[3] = 99;
        assert!(matches!(Snapshot::decode(&b), Err(Error::Version(99))));
        let mut b = Snapshot::from_store(&store(10), Tick(0)).encode().unwrap();
        b[5..9].copy_from_slice(&u32::MAX.to_le_bytes());
        assert!(matches!(Snapshot::decode(&b), Err(Error::TooLarge { .. })));
        let b = Snapshot::from_store(&store(50), Tick(0)).encode().unwrap();
        assert!(Snapshot::decode(&b[..b.len() - 3]).is_err());
    }

    #[test]
    fn corrupted_bytes_never_panic() {
        let b = Snapshot::from_store(&store(30), Tick(0)).encode().unwrap();
        for i in 0..b.len() {
            let mut c = b.clone();
            c[i] ^= 0xA5;
            let _ = Snapshot::decode(&c);
        }
    }
}
