//! `NiXenonRenderer/run_00e6e730` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Two functions `00e75a70` calls (one after the other) on the same object: each empties two
//! of its `NiTArray`s the way `NiTArray::RemoveAll` does, inlined.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiTArray;

/// `NiTArray::RemoveAll` as the two functions inline it: every element below `m_usSize`
/// cleared (the index is 16-bit, as the exe's `AX`), then `m_usSize` and `m_usESize` set to 0.
fn remove_all(e: &mut Engine, array: Ptr<NiTArray>) {
    let mut index: u16 = 0;
    while index < e.get(array, NiTArray::m_usSize) {
        let elements = e.get(array, NiTArray::m_pBase);
        e.mem
            .set_u32(elements.wrapping_add(u32::from(index) * 4), 0);
        index = index.wrapping_add(1);
    }
    e.set(array, NiTArray::m_usSize, 0);
    e.set(array, NiTArray::m_usESize, 0);
}

// Translated from 00e6e730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): empties the `NiTArray`s at +0xad4
/// and +0xae4 of `this` (elements cleared up to the size, size and element count set to 0).
pub fn fn_00e6e730(e: &mut Engine, this: Ptr) {
    remove_all(e, this.byte_add(0xad4).cast());
    remove_all(e, this.byte_add(0xae4).cast());
}

// Translated from 00e6e7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): empties the `NiTArray`s at +0xaf4
/// and +0xb04 of `this`, as [`fn_00e6e730`] does with the two before them.
pub fn fn_00e6e7b0(e: &mut Engine, this: Ptr) {
    remove_all(e, this.byte_add(0xaf4).cast());
    remove_all(e, this.byte_add(0xb04).cast());
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e6_e730, fn_00e6e730(Ptr)),
        entry!(0x00e6_e7b0, fn_00e6e7b0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An object with arrays at the four offsets, each of capacity 4 with `sizes[i]` used
    /// entries and every element and the free counts filled with markers; answers the object
    /// and the element blocks.
    fn object(e: &mut Engine, sizes: [u16; 4]) -> (u32, [u32; 4]) {
        let this = e.mem.alloc(0xb20);
        let mut blocks = [0u32; 4];
        for (i, offset) in [0xad4u32, 0xae4, 0xaf4, 0xb04].iter().enumerate() {
            let block = e.mem.alloc(0x10);
            for slot in 0..4 {
                e.mem.set_u32(block + 4 * slot, 0x0e00_0000 + slot);
            }
            let array = this + offset;
            e.mem.set_u32(array + 4, block);
            e.mem.set_u16(array + 8, 4);
            e.mem.set_u16(array + 0xa, sizes[i]);
            e.mem.set_u16(array + 0xc, sizes[i]);
            e.mem.set_u16(array + 0xe, 7);
            blocks[i] = block;
        }
        (this, blocks)
    }

    fn elements(e: &Engine, block: u32) -> Vec<u32> {
        (0..4).map(|slot| e.mem.u32(block + 4 * slot)).collect()
    }

    #[test]
    fn fn_00e6e730_empties_the_first_two_arrays() {
        let mut e = Engine::new();
        let (this, blocks) = object(&mut e, [3, 0, 2, 2]);
        e.call(0x00e6_e730, &args![this]);
        assert_eq!(elements(&e, blocks[0]), vec![0, 0, 0, 0x0e00_0003]);
        assert_eq!(
            elements(&e, blocks[1]),
            vec![0x0e00_0000, 0x0e00_0001, 0x0e00_0002, 0x0e00_0003]
        );
        for offset in [0xad4u32, 0xae4] {
            assert_eq!(e.mem.u16(this + offset + 0xa), 0);
            assert_eq!(e.mem.u16(this + offset + 0xc), 0);
            assert_eq!(e.mem.u16(this + offset + 0x8), 4);
            assert_eq!(e.mem.u16(this + offset + 0xe), 7);
        }
        // The other two are left alone.
        assert_eq!(e.mem.u16(this + 0xaf4 + 0xa), 2);
        assert_eq!(e.mem.u16(this + 0xb04 + 0xa), 2);
    }

    #[test]
    fn fn_00e6e7b0_empties_the_last_two_arrays() {
        let mut e = Engine::new();
        let (this, blocks) = object(&mut e, [2, 2, 4, 1]);
        e.call(0x00e6_e7b0, &args![this]);
        assert_eq!(elements(&e, blocks[2]), vec![0, 0, 0, 0]);
        assert_eq!(
            elements(&e, blocks[3]),
            vec![0, 0x0e00_0001, 0x0e00_0002, 0x0e00_0003]
        );
        for offset in [0xaf4u32, 0xb04] {
            assert_eq!(e.mem.u16(this + offset + 0xa), 0);
            assert_eq!(e.mem.u16(this + offset + 0xc), 0);
        }
        assert_eq!(e.mem.u16(this + 0xad4 + 0xa), 2);
        assert_eq!(e.mem.u16(this + 0xae4 + 0xa), 2);
    }
}
