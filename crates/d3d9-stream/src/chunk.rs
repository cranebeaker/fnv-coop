//! A run of commands handed to the render thread at once: the commands with their payloads as
//! places in the chunk's own memory (copied there when recorded), reused once carried out.

use crate::cmd::{Cmd, Data};

/// Where a payload lies in its chunk: a byte offset in the arena, or (with [`BIG`]) the index of
/// a block of its own.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Span {
    at: u32,
    len: u32,
}

/// Marks a [`Span`] of a block of its own.
const BIG: u32 = 1 << 31;

/// Payloads from this size up get a block of their own (zeroed by the allocator, so not written
/// twice), not the arena, which is kept and reused.
const BIG_BYTES: u32 = 64 * 1024;

/// Commands and their payloads.
#[derive(Default)]
pub(crate) struct Chunk {
    cmds: Vec<Cmd<Span>>,
    /// Small payloads, each 4-aligned.
    arena: Vec<u32>,
    bigs: Vec<Vec<u8>>,
    /// The bytes it holds, payloads and commands (for the queue's limit).
    pub(crate) bytes: usize,
    /// The presents in it.
    pub(crate) presents: u64,
    /// Its last command is sync: the render thread keeps its reply.
    pub(crate) reply_wanted: bool,
    /// `NV_D3D9_STREAM_CHECK`: the recording side's hash after its last command.
    pub(crate) check: Option<u64>,
}

impl Chunk {
    pub(crate) fn len(&self) -> usize {
        self.cmds.len()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.cmds.is_empty()
    }

    /// Appends `cmd`, its payloads copied in.
    pub(crate) fn push(&mut self, cmd: Cmd<Data<'_>>) {
        if matches!(cmd, Cmd::Present { .. }) {
            self.presents += 1;
        }
        let Chunk {
            arena, bigs, bytes, ..
        } = self;
        let packed = cmd.map(|d| store(arena, bigs, bytes, d));
        self.cmds.push(packed);
        self.bytes += std::mem::size_of::<Cmd<Span>>();
    }

    /// Command `i`, its payloads borrowed from the chunk.
    pub(crate) fn get(&self, i: usize) -> Cmd<Data<'_>> {
        self.cmds[i].map(|s| self.data(s))
    }

    fn data(&self, s: Span) -> Data<'_> {
        if s.at & BIG != 0 {
            Data::Bytes(&self.bigs[(s.at & !BIG) as usize])
        } else {
            Data::Bytes(&words_as_bytes(&self.arena)[s.at as usize..(s.at + s.len) as usize])
        }
    }

    /// Empty again, keeping its memory (the large blocks are freed).
    pub(crate) fn clear(&mut self) {
        self.cmds.clear();
        self.arena.clear();
        self.bigs.clear();
        self.bytes = 0;
        self.presents = 0;
        self.reply_wanted = false;
        self.check = None;
    }
}

fn store(arena: &mut Vec<u32>, bigs: &mut Vec<Vec<u8>>, bytes: &mut usize, d: Data<'_>) -> Span {
    let len = d.len();
    *bytes += len as usize;
    if len >= BIG_BYTES {
        let mut block = vec![0u8; len as usize];
        d.copy_to(0, &mut block);
        bigs.push(block);
        return Span {
            at: BIG | (bigs.len() - 1) as u32,
            len,
        };
    }
    let at = arena.len();
    match d {
        // Words lying aligned (constants, matrices): one copy.
        #[cfg(target_endian = "little")]
        Data::Bytes(b) if b.len() % 4 == 0 && b.as_ptr() as usize % 4 == 0 => {
            // SAFETY: aligned, a whole number of words, and every bit pattern is a word.
            arena.extend_from_slice(unsafe {
                std::slice::from_raw_parts(b.as_ptr().cast::<u32>(), b.len() / 4)
            });
        }
        _ => {
            arena.resize(at + (len as usize).div_ceil(4), 0);
            d.copy_to(0, &mut words_as_bytes_mut(&mut arena[at..])[..len as usize]);
        }
    }
    Span {
        at: 4 * at as u32,
        len,
    }
}

fn words_as_bytes(w: &[u32]) -> &[u8] {
    // SAFETY: `4 * len` initialised bytes; bytes have no alignment or validity requirement.
    unsafe { std::slice::from_raw_parts(w.as_ptr().cast(), 4 * w.len()) }
}

fn words_as_bytes_mut(w: &mut [u32]) -> &mut [u8] {
    // SAFETY: as above, and every byte pattern written is a valid word.
    unsafe { std::slice::from_raw_parts_mut(w.as_mut_ptr().cast(), 4 * w.len()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_command_stays_small() {
        // About 7,000 a frame are written into chunks.
        assert!(
            std::mem::size_of::<Cmd<Span>>() <= 48,
            "{} bytes",
            std::mem::size_of::<Cmd<Span>>()
        );
    }

    #[test]
    fn payloads_come_back_as_given_small_and_large() {
        let small: Vec<u8> = (0..37u8).collect();
        let large: Vec<u8> = (0..BIG_BYTES + 3).map(|i| (i * 7) as u8).collect();
        let read = |offset: u32, into: &mut [u8]| {
            for (i, b) in into.iter_mut().enumerate() {
                *b = (offset as usize + i) as u8 ^ 0x5a;
            }
        };
        let mut c = Chunk::default();
        c.push(Cmd::SetTransform {
            device: 1,
            state: 2,
            matrix: Data::Bytes(&small),
        });
        c.push(Cmd::UploadBuffer {
            buffer: 3,
            offset: 4,
            flags: 0,
            data: Data::Bytes(&large),
        });
        c.push(Cmd::UploadSurface {
            surface: 5,
            row: 6,
            rows: 7,
            data: Data::Read {
                len: 42,
                read: &read,
            },
        });
        assert_eq!(c.len(), 3);
        match c.get(0) {
            Cmd::SetTransform { matrix, .. } => {
                assert_eq!(&*matrix.bytes(), &small[..]);
                assert_eq!(matrix.to_words().len(), 10);
            }
            other => panic!("{other:?}"),
        }
        match c.get(1) {
            Cmd::UploadBuffer { data, offset, .. } => {
                assert_eq!(offset, 4);
                assert_eq!(&*data.bytes(), &large[..]);
            }
            other => panic!("{other:?}"),
        }
        match c.get(2) {
            Cmd::UploadSurface { data, .. } => {
                let want: Vec<u8> = (0..42u8).map(|i| i ^ 0x5a).collect();
                assert_eq!(&*data.bytes(), &want[..]);
            }
            other => panic!("{other:?}"),
        }
        c.clear();
        assert!(c.is_empty());
        assert_eq!(c.bytes, 0);
    }
}
