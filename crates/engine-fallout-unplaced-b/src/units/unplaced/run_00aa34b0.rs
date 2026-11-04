//! `(unplaced)/run_00aa34b0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// `NiFixedAllocator` (Xbox PDB layout; these offsets match the PC code):
// m_stBlockSize +0x00, m_ucNumBlocks +0x04, m_pkChunks +0x08,
// m_stNumChunks +0x0C, m_pkDeallocChunk +0x18. A chunk is 8 bytes:
// m_pucData +0, m_ucFirstAvailableBlock +4, m_ucBlocksAvailable +5.

// Translated from 00aa34b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds the chunk whose blocks hold `address` (thiscall; called by
/// `00aa3830` in `NiSystem/nistandardallocator.cpp`; the engine map has no
/// name): searches down from the deallocation chunk (+0x18) and up from the
/// chunk after it, one step each in turn, a chunk matching when `data <=
/// address < data + block size * blocks`; null when both ends are passed.
/// With both the deallocation chunk and the next one null from the start,
/// the exe loops forever, and so does this.
pub fn fn_00aa34b0(e: &mut Engine, this: u32, address: u32) -> u32 {
    let mut down = e.mem.u32(this + 0x18);
    let first = e.mem.u32(this + 8);
    let span = u32::from(e.mem.u8(this + 4)).wrapping_mul(e.mem.u32(this));
    let count = e.mem.u32(this + 0xc);
    let end = first.wrapping_add(count.wrapping_mul(8));
    let mut up = down.wrapping_add(8);
    if up == end {
        up = 0;
    }
    let holds = |e: &Engine, chunk: u32| {
        let data = e.mem.u32(chunk);
        data <= address && address < data.wrapping_add(span)
    };
    loop {
        if down != 0 {
            if holds(e, down) {
                return down;
            }
            if down == first {
                down = 0;
                if up == 0 {
                    return 0;
                }
            } else {
                down = down.wrapping_sub(8);
            }
        }
        if up == 0 {
            continue;
        }
        if holds(e, up) {
            return up;
        }
        up = up.wrapping_add(8);
        if up == end {
            up = 0;
            if down == 0 {
                return 0;
            }
        }
    }
}

// Translated from 00aa3530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Swaps two 8-byte chunks (cdecl; called by `00aa36f0`) unless they
/// are the same one; answers `first` (still in EAX).
pub fn fn_00aa3530(e: &mut Engine, first: u32, second: u32) -> u32 {
    if first != second {
        let data = e.mem.u32(first);
        let state = e.mem.u32(first + 4);
        let word = e.mem.u32(second);
        e.mem.set_u32(first, word);
        let word = e.mem.u32(second + 4);
        e.mem.set_u32(first + 4, word);
        e.mem.set_u32(second + 4, state);
        e.mem.set_u32(second, data);
    }
    first
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00aa_34b0, fn_00aa34b0(u32, u32) -> u32),
        entry!(0x00aa_3530, fn_00aa3530(u32, u32) -> u32),
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
        let chunks = e.mem.alloc(8 * data.len() as u32);
        for (i, d) in data.iter().enumerate() {
            e.mem.set_u32(chunks + 8 * i as u32, *d);
        }
        e.mem.set_u32(this + 8, chunks);
        e.mem.set_u32(this + 0xc, data.len() as u32);
        (this, chunks)
    }

    #[test]
    fn chunks_are_found_around_the_hint() {
        let mut e = Engine::new();
        let (this, chunks) = allocator(&mut e, &[0x1000, 0x2000, 0x3000, 0x4000]);
        e.mem.set_u32(this + 0x18, chunks + 8);
        let find = |e: &mut Engine, a: u32| e.call(0x00aa_34b0, &[this, a]).u32();
        assert_eq!(find(&mut e, 0x2000), chunks + 8);
        assert_eq!(find(&mut e, 0x103f), chunks);
        assert_eq!(find(&mut e, 0x3010), chunks + 16);
        assert_eq!(find(&mut e, 0x4020), chunks + 24);
        assert_eq!(find(&mut e, 0x4040), 0);
        assert_eq!(find(&mut e, 0x0fff), 0);
        // The hint at the last chunk: only the downward search runs.
        e.mem.set_u32(this + 0x18, chunks + 24);
        assert_eq!(find(&mut e, 0x1000), chunks);
        assert_eq!(find(&mut e, 0x5000), 0);
    }

    #[test]
    fn chunks_swap_unless_the_same() {
        let mut e = Engine::new();
        let a = e.mem.alloc(8);
        let b = e.mem.alloc(8);
        e.mem.set_u32(a, 1);
        e.mem.set_u32(a + 4, 2);
        e.mem.set_u32(b, 3);
        e.mem.set_u32(b + 4, 4);
        assert_eq!(e.call(0x00aa_3530, &[a, b]).u32(), a);
        assert_eq!((e.mem.u32(a), e.mem.u32(a + 4)), (3, 4));
        assert_eq!((e.mem.u32(b), e.mem.u32(b + 4)), (1, 2));
        assert_eq!(e.call(0x00aa_3530, &[a, a]).u32(), a);
        assert_eq!((e.mem.u32(a), e.mem.u32(a + 4)), (3, 4));
    }
}
