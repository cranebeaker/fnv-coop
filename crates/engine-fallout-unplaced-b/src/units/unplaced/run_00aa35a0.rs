//! `(unplaced)/run_00aa35a0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// `NiFixedAllocator` (Xbox PDB layout; these offsets match the PC code):
// m_stBlockSize +0x00, m_ucNumBlocks +0x04, m_pkChunks +0x08,
// m_stNumChunks +0x0C, m_stMaxNumChunks +0x10, m_pkAllocChunk +0x14,
// m_pkDeallocChunk +0x18, m_pkEmptyChunk +0x1C, m_kCriticalSection +0x80.
// A chunk is 8 bytes: m_pucData +0, m_ucFirstAvailableBlock +4,
// m_ucBlocksAvailable +5.

/// `EnterCriticalSection` and `LeaveCriticalSection` (import slots).
const IMPORT_ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const IMPORT_LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
/// Reallocates through the Ni allocator (`00aa11b0`, cdecl: block, size).
const NI_REALLOC: u32 = 0x00aa_11b0;
/// Allocates aligned through the Ni allocator (`00aa1130`, cdecl: size,
/// alignment).
const NI_ALIGNED_ALLOC: u32 = 0x00aa_1130;
/// Frees a block through the Ni allocator (`00aa1190`, cdecl; null is
/// ignored).
const NI_FREE_00AA1190: u32 = 0x00aa_1190;
/// Swaps two chunks (`00aa3530`, cdecl).
const SWAP_CHUNKS: u32 = 0x00aa_3530;

/// Grows the chunk array to hold `needed` chunks when its capacity (+0x10)
/// is smaller (`00aa11b0` to `needed * 8` bytes).
fn reserve_chunks(e: &mut Engine, this: u32, needed: u32) {
    if needed > e.mem.u32(this + 0x10) {
        let chunks = e.mem.u32(this + 8);
        let grown = e.call(NI_REALLOC, &[chunks, needed.wrapping_mul(8)]).u32();
        e.mem.set_u32(this + 8, grown);
        e.mem.set_u32(this + 0x10, needed);
    }
}

// Translated from 00aa35a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Allocates a block (thiscall; called by `00aa3810`; the engine map has no
/// name), inside the critical section at +0x80: drops the allocation chunk
/// (+0x14) when it is full, takes the kept empty chunk (+0x1C, then
/// cleared) when there is one, and with no allocation chunk takes the first
/// chunk with a free block; with none, adds a chunk (the array grown through
/// `00aa11b0` as needed, twice as the exe checks it twice) whose data
/// (`00aa1130`, block size * blocks, alignment 4) has each block's first byte
/// set to the next block's index, first free 0 and all blocks free, and
/// makes it the allocation and the first chunk the deallocation chunk.
/// Answers the allocation chunk's first free block (whose first byte becomes
/// the chunk's first free index, one fewer free), or null when it has none.
/// The new chunk's padding bytes +6 and +7 get two uninitialized bytes of
/// the exe's stack; they are left as they are here.
pub fn fn_00aa35a0(e: &mut Engine, this: u32) -> u32 {
    let section = this + 0x80;
    e.call(IMPORT_ENTER_CRITICAL_SECTION, &[section]);
    let current = e.mem.u32(this + 0x14);
    if current != 0 && e.mem.u8(current + 5) == 0 {
        e.mem.set_u32(this + 0x14, 0);
    }
    let empty = e.mem.u32(this + 0x1c);
    if empty != 0 {
        e.mem.set_u32(this + 0x14, empty);
        e.mem.set_u32(this + 0x1c, 0);
    }
    if e.mem.u32(this + 0x14) == 0 {
        let count = e.mem.u32(this + 0xc);
        let mut index = 0;
        while index < count {
            let chunk = e.mem.u32(this + 8).wrapping_add(index * 8);
            if e.mem.u8(chunk + 5) != 0 {
                e.mem.set_u32(this + 0x14, chunk);
                break;
            }
            index += 1;
        }
        if e.mem.u32(this + 0x14) == 0 {
            reserve_chunks(e, this, count.wrapping_add(1));
            let blocks = e.mem.u8(this + 4);
            let block_size = e.mem.u32(this);
            let data = e
                .call(
                    NI_ALIGNED_ALLOC,
                    &[u32::from(blocks).wrapping_mul(block_size), 4],
                )
                .u32();
            let mut next = 0u8;
            let mut at = data;
            while next != blocks {
                next = next.wrapping_add(1);
                e.mem.set_u8(at, next);
                at = at.wrapping_add(block_size);
            }
            let slot = e.mem.u32(this + 0xc);
            reserve_chunks(e, this, slot.wrapping_add(1));
            let chunks = e.mem.u32(this + 8);
            let chunk = chunks.wrapping_add(slot.wrapping_mul(8));
            e.mem.set_u32(chunk, data);
            e.mem.set_u8(chunk + 4, 0);
            e.mem.set_u8(chunk + 5, blocks);
            let count = e.mem.u32(this + 0xc).wrapping_add(1);
            e.mem.set_u32(this + 0xc, count);
            let chunks = e.mem.u32(this + 8);
            let last = chunks.wrapping_add(count.wrapping_mul(8)).wrapping_sub(8);
            e.mem.set_u32(this + 0x14, last);
            e.mem.set_u32(this + 0x18, chunks);
        }
    }
    let chunk = e.mem.u32(this + 0x14);
    let available = e.mem.u8(chunk + 5);
    if available == 0 {
        e.call(IMPORT_LEAVE_CRITICAL_SECTION, &[section]);
        return 0;
    }
    let block = u32::from(e.mem.u8(chunk + 4))
        .wrapping_mul(e.mem.u32(this))
        .wrapping_add(e.mem.u32(chunk));
    let next = e.mem.u8(block);
    e.mem.set_u8(chunk + 4, next);
    e.mem.set_u8(chunk + 5, available.wrapping_sub(1));
    e.call(IMPORT_LEAVE_CRITICAL_SECTION, &[section]);
    block
}

// Translated from 00aa36f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns `block` to the deallocation chunk (thiscall; called by
/// `00aa3830` after `00aa34b0` found the chunk; the engine map has no
/// name): the block's first byte gets the chunk's first free index, which
/// becomes the block's index (`(block - data) / block size`, its low byte),
/// and the chunk's free count grows. When the chunk is then entirely free
/// and an empty chunk (+0x1C) is already kept, one of the two goes: if the
/// chunk is the last one, the empty chunk becomes the deallocation chunk;
/// otherwise the empty chunk is swapped with the last (`00aa3530`) unless it
/// is the last; then the last chunk's data is freed (`00aa1190`), the chunk
/// count drops and the allocation chunk (+0x14) becomes the deallocation
/// chunk. An entirely free deallocation chunk is then kept as the empty
/// chunk.
pub fn fn_00aa36f0(e: &mut Engine, this: u32, block: u32) {
    let chunk = e.mem.u32(this + 0x18);
    let first_free = e.mem.u8(chunk + 4);
    let block_size = e.mem.u32(this);
    e.mem.set_u8(block, first_free);
    // `DIV`: a block size of 0 is a divide fault in the exe.
    let index = block.wrapping_sub(e.mem.u32(chunk)) / block_size;
    let available = e.mem.u8(chunk + 5).wrapping_add(1);
    e.mem.set_u8(chunk + 5, available);
    e.mem.set_u8(chunk + 4, index as u8);
    let mut chunk = e.mem.u32(this + 0x18);
    if e.mem.u8(chunk + 5) != e.mem.u8(this + 4) {
        return;
    }
    let empty = e.mem.u32(this + 0x1c);
    if empty != 0 {
        let count = e.mem.u32(this + 0xc);
        let last = e
            .mem
            .u32(this + 8)
            .wrapping_add(count.wrapping_mul(8))
            .wrapping_sub(8);
        if last == chunk {
            e.mem.set_u32(this + 0x18, empty);
        } else if last != empty {
            e.call(SWAP_CHUNKS, &[empty, last]);
        }
        let data = e.mem.u32(last);
        e.call(NI_FREE_00AA1190, &[data]);
        chunk = e.mem.u32(this + 0x18);
        let count = e.mem.u32(this + 0xc).wrapping_sub(1);
        e.mem.set_u32(this + 0xc, count);
        e.mem.set_u32(this + 0x14, chunk);
    }
    e.mem.set_u32(this + 0x1c, chunk);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00aa_35a0, fn_00aa35a0(u32) -> u32),
        entry!(0x00aa_36f0, fn_00aa36f0(u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An allocator of 4 blocks of 0x10 bytes with chunks whose data start
    /// at `data[i]`; returns it and its chunk array.
    fn allocator(e: &mut Engine, data: &[u32]) -> (u32, u32) {
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this, 0x10);
        e.mem.set_u8(this + 4, 4);
        let chunks = e.mem.alloc(8 * data.len() as u32 + 8);
        for (i, d) in data.iter().enumerate() {
            e.mem.set_u32(chunks + 8 * i as u32, *d);
        }
        e.mem.set_u32(this + 8, chunks);
        e.mem.set_u32(this + 0xc, data.len() as u32);
        e.mem.set_u32(this + 0x10, data.len() as u32 + 1);
        (this, chunks)
    }

    fn allocation_engine() -> Engine {
        let mut e = Engine::new();
        e.register(IMPORT_ENTER_CRITICAL_SECTION, |_, _| 0u32.into_ret());
        e.register(IMPORT_LEAVE_CRITICAL_SECTION, |_, _| 0u32.into_ret());
        e.register(NI_ALIGNED_ALLOC, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_REALLOC, |e, a| {
            let grown = e.mem.alloc(a[1]);
            if a[0] != 0 {
                let old = e.mem.bytes(a[0], a[1] - 8);
                e.mem.write(grown, &old);
            }
            grown.into_ret()
        });
        e
    }

    #[test]
    fn allocation_uses_a_free_chunk() {
        let mut e = allocation_engine();
        let data = e.mem.alloc(0x40);
        let (this, chunks) = allocator(&mut e, &[0x1000, data]);
        e.mem.set_u8(chunks + 12, 2);
        e.mem.set_u8(chunks + 13, 2);
        e.mem.set_u8(data + 0x20, 3);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00aa_35a0, &[this]).u32(), data + 0x20);
        assert_eq!(e.mem.u32(this + 0x14), chunks + 8);
        assert_eq!((e.mem.u8(chunks + 12), e.mem.u8(chunks + 13)), (3, 1));
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (IMPORT_ENTER_CRITICAL_SECTION, vec![this + 0x80]));
        assert_eq!(
            log.last().unwrap(),
            &(IMPORT_LEAVE_CRITICAL_SECTION, vec![this + 0x80])
        );
        // The kept empty chunk is taken first and the slot cleared.
        e.mem.set_u32(this + 0x1c, chunks + 8);
        e.mem.set_u32(this + 0x14, chunks);
        assert_eq!(e.call(0x00aa_35a0, &[this]).u32(), data + 0x30);
        assert_eq!(e.mem.u32(this + 0x1c), 0);
    }

    #[test]
    fn allocation_adds_a_chunk_when_all_are_full() {
        let mut e = allocation_engine();
        let (this, _) = allocator(&mut e, &[0x1000]);
        e.mem.set_u32(this + 0x10, 1);
        e.call_log = Some(vec![]);
        let block = e.call(0x00aa_35a0, &[this]).u32();
        let chunks = e.mem.u32(this + 8);
        let added = chunks + 8;
        let data = e.mem.u32(added);
        assert_eq!(block, data);
        assert_eq!(e.mem.u32(this + 0xc), 2);
        assert_eq!(e.mem.u32(this + 0x10), 2);
        assert_eq!(e.mem.u32(this + 0x14), added);
        assert_eq!(e.mem.u32(this + 0x18), chunks);
        assert_eq!((e.mem.u8(added + 4), e.mem.u8(added + 5)), (1, 3));
        let links: Vec<u8> = (0..4).map(|i| e.mem.u8(data + 0x10 * i)).collect();
        assert_eq!(links, vec![1, 2, 3, 4]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(NI_ALIGNED_ALLOC, vec![0x40, 4])));
        let reallocs: Vec<_> = log.iter().filter(|(a, _)| *a == NI_REALLOC).collect();
        assert_eq!(reallocs.len(), 1);
        assert_eq!(reallocs[0].1[1], 16);
    }

    #[test]
    fn freed_blocks_link_into_their_chunk() {
        let mut e = Engine::new();
        e.register(NI_FREE_00AA1190, |_, _| 0u32.into_ret());
        let data = e.mem.alloc(0x40);
        let (this, chunks) = allocator(&mut e, &[data]);
        e.mem.set_u32(this + 0x18, chunks);
        e.mem.set_u8(chunks + 4, 3);
        e.mem.set_u8(chunks + 5, 1);
        e.call(0x00aa_36f0, &[this, data + 0x20]);
        assert_eq!(e.mem.u8(data + 0x20), 3);
        assert_eq!((e.mem.u8(chunks + 4), e.mem.u8(chunks + 5)), (2, 2));
        assert_eq!(e.mem.u32(this + 0x1c), 0);
        // Freeing the rest empties it: kept as the empty chunk.
        e.mem.set_u8(chunks + 5, 3);
        e.call(0x00aa_36f0, &[this, data]);
        assert_eq!(e.mem.u32(this + 0x1c), chunks);
        assert_eq!(e.mem.u32(this + 0xc), 1);
    }

    #[test]
    fn a_second_empty_chunk_frees_the_last() {
        let mut e = Engine::new();
        e.register(NI_FREE_00AA1190, |_, _| 0u32.into_ret());
        e.register(SWAP_CHUNKS, |_, a| a[0].into_ret());
        // The emptied chunk is not the last: the kept empty chunk is swapped
        // with the last, whose data is freed.
        let data = e.mem.alloc(0x40);
        let (this, chunks) = allocator(&mut e, &[0x1000, data, 0x3000]);
        e.mem.set_u8(chunks + 13, 3);
        e.mem.set_u32(this + 0x18, chunks + 8);
        e.mem.set_u32(this + 0x1c, chunks);
        e.call_log = Some(vec![]);
        e.call(0x00aa_36f0, &[this, data + 0x10]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (SWAP_CHUNKS, vec![chunks, chunks + 16]),
                (NI_FREE_00AA1190, vec![0x3000])
            ]
        );
        assert_eq!(e.mem.u32(this + 0xc), 2);
        assert_eq!(e.mem.u32(this + 0x14), chunks + 8);
        assert_eq!(e.mem.u32(this + 0x1c), chunks + 8);
        // The emptied chunk is the last: the empty one takes its place.
        let data = e.mem.alloc(0x40);
        let (this, chunks) = allocator(&mut e, &[0x1000, data]);
        e.mem.set_u8(chunks + 13, 3);
        e.mem.set_u32(this + 0x18, chunks + 8);
        e.mem.set_u32(this + 0x1c, chunks);
        e.call_log = Some(vec![]);
        e.call(0x00aa_36f0, &[this, data]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(NI_FREE_00AA1190, vec![data])]);
        assert_eq!(e.mem.u32(this + 0xc), 1);
        assert_eq!(e.mem.u32(this + 0x18), chunks);
        assert_eq!(e.mem.u32(this + 0x14), chunks);
        assert_eq!(e.mem.u32(this + 0x1c), chunks);
    }
}
