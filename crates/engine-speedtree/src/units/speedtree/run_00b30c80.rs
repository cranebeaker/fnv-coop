//! `SpeedTree/run_00b30c80` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `CIndexedGeometry` member functions the engine map gives no name: the release of the strip
//! arrays of one detail level and of all of them, and the two accessors that give the address of
//! a vertex's coordinates and texture coordinates in the geometry's float vectors. The layout
//! of the class is the one of `indexedgeometry.rs` (the PC `std::vector` is 0x18 bytes).

#[allow(unused_imports)]
use crate::prelude::*;

use super::branch::{vector_at, vector_len, INVALID_PARAMETER, OPERATOR_DELETE};
use super::indexedgeometry::CIndexedGeometry;

/// `CIndexedGeometry::GetNumStrips`-style helper `00b0a9a0` (`thiscall`, `RET 4`: the detail
/// level as a sign-extended word; the count comes back in `AX`).
const STRIP_COUNT: u32 = 0x00b0_a9a0;
/// `std::vector<unsigned short *>::clear` of the strip vector's elements (`00b32350`,
/// `thiscall`) and the clear of the strip vector itself (`00b32120`, `thiscall`).
const CLEAR_STRIP_ELEMENT: u32 = 0x00b3_2350;
const CLEAR_STRIPS: u32 = 0x00b3_2120;
/// The offset of the strip vector (`m_vStrips`) in the PC geometry.
const STRIPS: u32 = 0x58;

// Translated from 00b30c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees the strip arrays of one detail level and clears their pointers (the engine map gives
/// it no name; `thiscall`, one argument, the level; `RET 4`): while the index is below the
/// level's strip count (`00b0a9a0`, asked each round and taken as an unsigned word), the pointer
/// at that index of the level's vector in the strip vector (`m_vStrips`, `+0x58`, 0x18-byte
/// elements of pointer vectors) is released with `operator delete` and then set to zero. Both
/// indexes are checked as the `std::vector` code checks them (a level or index past the end
/// takes the invalid-parameter path, `00ec7c56`).
pub fn fn_00b30c80(e: &mut Engine, this: Ptr<CIndexedGeometry>, level: u16) {
    let base = this.addr();
    let level = u32::from(level);
    let mut index = 0i32;
    loop {
        let count = i32::from(e.call(STRIP_COUNT, &args![this, level]).u16());
        if index >= count {
            break;
        }
        let strips = vector_at(e, base + STRIPS, level, 0x18);
        let slot = vector_at(e, strips, index as u32, 4);
        let array = e.mem.u32(slot);
        e.call(OPERATOR_DELETE, &args![array]);
        let strips = vector_at(e, base + STRIPS, level, 0x18);
        let slot = vector_at(e, strips, index as u32, 4);
        e.mem.set_u32(slot, 0);
        index = index.wrapping_add(1);
    }
}

// Translated from 00b30da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees every strip array of every detail level and empties the vectors (the engine map gives
/// it no name; `fastcall`, `this` only): for each level of the strip vector (`+0x58`), each
/// pointer of its vector is released with `operator delete` and set to zero, then the level's
/// vector is cleared (`00b32350`); at the end the strip vector itself is cleared (`00b32120`).
/// The indexes are checked as the `std::vector` code checks them (`00ec7c56` on a failure).
pub fn fn_00b30da0(e: &mut Engine, this: Ptr<CIndexedGeometry>) {
    let strips_vector = this.addr() + STRIPS;
    let mut level = 0u32;
    while level < vector_len(e, strips_vector, 0x18) {
        let mut index = 0u32;
        loop {
            let strips = vector_at(e, strips_vector, level, 0x18);
            if index >= vector_len(e, strips, 4) {
                break;
            }
            let strips = vector_at(e, strips_vector, level, 0x18);
            let slot = vector_at(e, strips, index, 4);
            let array = e.mem.u32(slot);
            e.call(OPERATOR_DELETE, &args![array]);
            let strips = vector_at(e, strips_vector, level, 0x18);
            let slot = vector_at(e, strips, index, 4);
            e.mem.set_u32(slot, 0);
            index = index.wrapping_add(1);
        }
        let strips = vector_at(e, strips_vector, level, 0x18);
        e.call(CLEAR_STRIP_ELEMENT, &args![strips]);
        level = level.wrapping_add(1);
    }
    e.call(CLEAR_STRIPS, &args![strips_vector]);
}

// Translated from 00b30f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the coordinates of vertex `index` in the coordinate vector (`m_vCoords`,
/// `+0x88`; the engine map gives it no name; `thiscall`, one argument, `RET 4`): the vector has
/// to hold at least one float (else the invalid-parameter path, `00ec7c56`); the address is
/// the vector's first pointer plus twelve bytes per vertex.
pub fn fn_00b30f80(e: &mut Engine, this: Ptr<CIndexedGeometry>, index: u32) -> u32 {
    let vector = this.addr() + 0x88;
    let floats = (e
        .mem
        .u32(vector + 0x10)
        .wrapping_sub(e.mem.u32(vector + 0x0C)) as i32
        >> 2) as u32;
    if floats == 0 {
        e.call(INVALID_PARAMETER, &args![]);
    }
    e.mem
        .u32(vector + 0x0C)
        .wrapping_add(index.wrapping_mul(3).wrapping_mul(4))
}

// Translated from 00b30fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the texture coordinates of vertex `index` in the first texture-coordinate
/// vector (`m_vTexCoords0`, `+0x100`; the engine map gives it no name; `thiscall`, one argument,
/// `RET 4`): twice the index has to be below the vector's float count (else the
/// invalid-parameter path, `00ec7c56`); the address is the vector's first pointer plus eight
/// bytes per vertex.
pub fn fn_00b30fd0(e: &mut Engine, this: Ptr<CIndexedGeometry>, index: u32) -> u32 {
    let vector = this.addr() + 0x100;
    let doubled = index << 1;
    let floats = (e
        .mem
        .u32(vector + 0x10)
        .wrapping_sub(e.mem.u32(vector + 0x0C)) as i32
        >> 2) as u32;
    if doubled >= floats {
        e.call(INVALID_PARAMETER, &args![]);
    }
    e.mem
        .u32(vector + 0x0C)
        .wrapping_add(doubled.wrapping_mul(4))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b30c80, fn_00b30c80(Ptr<CIndexedGeometry>, u16)),
        entry!(0x00b30da0, fn_00b30da0(Ptr<CIndexedGeometry>)),
        entry!(0x00b30f80, fn_00b30f80(Ptr<CIndexedGeometry>, u32) -> u32),
        entry!(0x00b30fd0, fn_00b30fd0(Ptr<CIndexedGeometry>, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The proxy word the test vectors carry.
    const PROXY: u32 = 0x0090_0000;

    /// An engine with the exe's data range mapped and the callees outside the file silenced.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0020_0000);
        for addr in [
            STRIP_COUNT,
            CLEAR_STRIP_ELEMENT,
            CLEAR_STRIPS,
            INVALID_PARAMETER,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e
    }

    /// Sets the vector header at `at` over `count` words at `data`.
    fn word_vector(e: &mut Engine, at: u32, data: u32, count: u32) {
        e.mem.set_u32(at, PROXY);
        e.mem.set_u32(at + 0x0C, data);
        e.mem.set_u32(at + 0x10, data + 4 * count);
        e.mem.set_u32(at + 0x14, data + 4 * count);
    }

    /// A geometry whose strip vector has one pointer vector per entry of `levels`, each holding
    /// freshly allocated blocks; returns the geometry and the blocks.
    fn geometry(e: &mut Engine, levels: &[u32]) -> (Ptr<CIndexedGeometry>, Vec<Vec<u32>>) {
        let this = e.new_object::<CIndexedGeometry>();
        let outer = e.mem.alloc(0x18 * levels.len() as u32);
        let mut all = Vec::new();
        for (i, count) in levels.iter().enumerate() {
            let blocks: Vec<u32> = (0..*count).map(|_| e.mem.alloc(16)).collect();
            let array = e.mem.alloc(4 * count);
            for (j, block) in blocks.iter().enumerate() {
                e.mem.set_u32(array + 4 * j as u32, *block);
            }
            word_vector(e, outer + 0x18 * i as u32, array, *count);
            all.push(blocks);
        }
        let n = levels.len() as u32;
        e.mem.set_u32(this.addr() + 0x58, PROXY);
        e.mem.set_u32(this.addr() + 0x58 + 0x0C, outer);
        e.mem.set_u32(this.addr() + 0x58 + 0x10, outer + 0x18 * n);
        e.mem.set_u32(this.addr() + 0x58 + 0x14, outer + 0x18 * n);
        (this, all)
    }

    #[test]
    fn freeing_a_levels_strips_frees_the_counted_arrays_and_clears_their_slots() {
        let mut e = engine();
        let (this, blocks) = geometry(&mut e, &[1, 3]);
        // The level has three arrays but the strip count says two.
        e.register(STRIP_COUNT, |_, _| Ret {
            eax: 2,
            ..Ret::default()
        });
        e.call(0x00b3_0c80, &args![this, 1u32]);
        assert_eq!(e.mem.block_size(blocks[1][0]), None);
        assert_eq!(e.mem.block_size(blocks[1][1]), None);
        assert_eq!(e.mem.block_size(blocks[1][2]), Some(16));
        // The other level is untouched.
        assert_eq!(e.mem.block_size(blocks[0][0]), Some(16));
        let outer = e.mem.u32(this.addr() + 0x58 + 0x0C);
        let array = e.mem.u32(outer + 0x18 + 0x0C);
        assert_eq!(e.mem.u32(array), 0);
        assert_eq!(e.mem.u32(array + 4), 0);
        assert_eq!(e.mem.u32(array + 8), blocks[1][2]);
    }

    #[test]
    fn freeing_every_strip_clears_the_levels_and_the_strip_vector() {
        let mut e = engine();
        let (this, blocks) = geometry(&mut e, &[2, 1]);
        e.call_log = Some(vec![]);
        e.call(0x00b3_0da0, &args![this]);
        let log = e.call_log.take().unwrap();
        for block in blocks.iter().flatten() {
            assert_eq!(e.mem.block_size(*block), None);
        }
        let outer = e.mem.u32(this.addr() + 0x58 + 0x0C);
        let cleared: Vec<Vec<u32>> = log
            .iter()
            .filter(|(a, _)| *a == CLEAR_STRIP_ELEMENT)
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(cleared, vec![vec![outer], vec![outer + 0x18]]);
        let strips: Vec<Vec<u32>> = log
            .iter()
            .filter(|(a, _)| *a == CLEAR_STRIPS)
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(strips, vec![vec![this.addr() + 0x58]]);
        // Every slot of the first level is zero.
        let array = e.mem.u32(outer + 0x0C);
        assert_eq!((e.mem.u32(array), e.mem.u32(array + 4)), (0, 0));
    }

    /// Makes the float vector header at `at` hold `count` floats.
    fn float_vector(e: &mut Engine, at: u32, count: u32) -> u32 {
        let data = e.mem.alloc(4 * count.max(1));
        word_vector(e, at, data, count);
        data
    }

    #[test]
    fn the_coordinate_address_is_twelve_bytes_per_vertex() {
        let mut e = engine();
        let this = e.new_object::<CIndexedGeometry>();
        let data = float_vector(&mut e, this.addr() + 0x88, 6);
        e.call_log = Some(vec![]);
        let address = e.call(0x00b3_0f80, &args![this, 2u32]).u32();
        assert_eq!(address, data + 24);
        assert!(e
            .call_log
            .take()
            .unwrap()
            .iter()
            .all(|(a, _)| *a != INVALID_PARAMETER));
        // An empty coordinate vector takes the invalid-parameter path.
        let empty = e.new_object::<CIndexedGeometry>();
        e.call_log = Some(vec![]);
        e.call(0x00b3_0f80, &args![empty, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log.iter().filter(|(a, _)| *a == INVALID_PARAMETER).count(),
            1
        );
    }

    #[test]
    fn the_texture_coordinate_address_is_eight_bytes_per_vertex() {
        let mut e = engine();
        let this = e.new_object::<CIndexedGeometry>();
        let data = float_vector(&mut e, this.addr() + 0x100, 8);
        e.call_log = Some(vec![]);
        let address = e.call(0x00b3_0fd0, &args![this, 3u32]).u32();
        assert_eq!(address, data + 24);
        assert!(e
            .call_log
            .take()
            .unwrap()
            .iter()
            .all(|(a, _)| *a != INVALID_PARAMETER));
        // Twice the index must be below the float count.
        e.call_log = Some(vec![]);
        e.call(0x00b3_0fd0, &args![this, 4u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log.iter().filter(|(a, _)| *a == INVALID_PARAMETER).count(),
            1
        );
    }
}
