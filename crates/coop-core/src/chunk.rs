//! Splitting big blobs (snapshots) into reliable-channel sized chunks, and
//! putting them back together.

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::ids::Seq;

/// Most chunks one blob may use (with 1 KiB chunks: 64 MiB).
pub const MAX_CHUNKS: u16 = u16::MAX;

/// One piece of a blob.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Chunk {
    /// Blob id (the snapshot's send sequence number).
    pub id: Seq,
    /// Position of this piece.
    pub index: u16,
    /// Number of pieces.
    pub total: u16,
    /// Bytes.
    pub data: Vec<u8>,
}

/// Splits `bytes` into pieces of at most `max` bytes (at least one piece).
pub fn split(id: Seq, bytes: &[u8], max: usize) -> Result<Vec<Chunk>> {
    let max = max.max(1);
    let n = bytes.len().div_ceil(max).max(1);
    if n > MAX_CHUNKS as usize {
        return Err(Error::TooLarge {
            len: bytes.len(),
            max: max * MAX_CHUNKS as usize,
        });
    }
    let mut out: Vec<Chunk> = bytes
        .chunks(max)
        .enumerate()
        .map(|(i, c)| Chunk {
            id,
            index: i as u16,
            total: n as u16,
            data: c.to_vec(),
        })
        .collect();
    if out.is_empty() {
        out.push(Chunk {
            id,
            index: 0,
            total: 1,
            data: Vec::new(),
        });
    }
    Ok(out)
}

/// Collects chunks of the newest blob id; older ids are dropped.
#[derive(Debug, Default)]
pub struct Reassembler {
    id: Option<Seq>,
    parts: Vec<Option<Vec<u8>>>,
    got: usize,
}

impl Reassembler {
    /// New, empty.
    pub fn new() -> Self {
        Self::default()
    }

    /// Feeds one chunk. Returns the whole blob when the last piece arrives.
    /// Duplicate pieces are ignored; a chunk of a newer blob restarts the
    /// assembly; a chunk of an older blob is ignored.
    pub fn push(&mut self, c: Chunk) -> Result<Option<Vec<u8>>> {
        if c.total == 0 || c.index >= c.total {
            return Err(Error::Chunk("index out of range"));
        }
        match self.id {
            Some(cur) if cur == c.id => {
                if self.parts.is_empty() {
                    return Ok(None); // already completed
                }
                if self.parts.len() != c.total as usize {
                    return Err(Error::Chunk("total changed"));
                }
            }
            Some(cur) if !c.id.is_newer_than(cur) => return Ok(None),
            _ => {
                self.id = Some(c.id);
                self.parts = vec![None; c.total as usize];
                self.got = 0;
            }
        }
        let slot = &mut self.parts[c.index as usize];
        if slot.is_none() {
            *slot = Some(c.data);
            self.got += 1;
        }
        if self.got == self.parts.len() {
            let mut out = Vec::new();
            for p in self.parts.drain(..) {
                out.extend_from_slice(&p.unwrap_or_default());
            }
            self.got = 0;
            // keep `id` so late duplicates of this blob are ignored, not restarted
            self.parts = Vec::new();
            return Ok(Some(out));
        }
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn single_and_empty() {
        let c = split(Seq(1), b"abc", 1024).unwrap();
        assert_eq!(c.len(), 1);
        let c = split(Seq(1), b"", 1024).unwrap();
        assert_eq!(c.len(), 1);
        let mut r = Reassembler::new();
        assert_eq!(r.push(c[0].clone()).unwrap(), Some(vec![]));
    }

    #[test]
    fn bad_chunks() {
        let mut r = Reassembler::new();
        assert!(r
            .push(Chunk {
                id: Seq(1),
                index: 0,
                total: 0,
                data: vec![]
            })
            .is_err());
        assert!(r
            .push(Chunk {
                id: Seq(1),
                index: 2,
                total: 2,
                data: vec![]
            })
            .is_err());
        r.push(Chunk {
            id: Seq(1),
            index: 0,
            total: 2,
            data: vec![1],
        })
        .unwrap();
        assert!(r
            .push(Chunk {
                id: Seq(1),
                index: 1,
                total: 3,
                data: vec![1]
            })
            .is_err());
    }

    #[test]
    fn newer_blob_restarts_older_ignored() {
        let mut r = Reassembler::new();
        r.push(Chunk {
            id: Seq(5),
            index: 0,
            total: 2,
            data: vec![1],
        })
        .unwrap();
        r.push(Chunk {
            id: Seq(6),
            index: 0,
            total: 2,
            data: vec![2],
        })
        .unwrap();
        assert_eq!(
            r.push(Chunk {
                id: Seq(5),
                index: 1,
                total: 2,
                data: vec![9]
            })
            .unwrap(),
            None
        );
        let out = r
            .push(Chunk {
                id: Seq(6),
                index: 1,
                total: 2,
                data: vec![3],
            })
            .unwrap();
        assert_eq!(out, Some(vec![2, 3]));
        // late duplicate of finished blob does not restart
        assert_eq!(
            r.push(Chunk {
                id: Seq(6),
                index: 0,
                total: 2,
                data: vec![2]
            })
            .unwrap(),
            None
        );
    }

    proptest! {
        #[test]
        fn any_order_with_dups_reassembles(
            data in proptest::collection::vec(any::<u8>(), 0..5000),
            max in 1usize..700,
            seed in any::<u64>(),
        ) {
            let mut chunks = split(Seq(3), &data, max).unwrap();
            let mut dups = chunks.clone();
            chunks.append(&mut dups);
            // deterministic shuffle
            let mut s = seed | 1;
            for i in (1..chunks.len()).rev() {
                s ^= s << 13; s ^= s >> 7; s ^= s << 17;
                chunks.swap(i, (s % (i as u64 + 1)) as usize);
            }
            let mut r = Reassembler::new();
            let mut done = None;
            for c in chunks {
                if let Some(b) = r.push(c).unwrap() { prop_assert!(done.is_none()); done = Some(b); }
            }
            prop_assert_eq!(done.unwrap(), data);
        }
    }
}
