//! `Havok SDK/run_00d98390` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d98390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, no stack words: initialises an array of four 0x50-byte output blocks (each 0x50
/// apart from `this`): the float 1.0 at `+0x10`, `0xffffffff` at `+0x14` and `+0x20` and 0 at
/// `+0x40` of every block (the exe's `ECX` is the array; `hkpcpukdtreeraycastjob.rs` calls it as
/// `OUTPUT_ARRAY_INIT`). Returns `this`. The class is unconfirmed.
pub fn fn_00d98390(e: &mut Engine, this: Ptr) -> Ptr {
    let array = this.addr();
    for block in 0..4u32 {
        let base = array.wrapping_add(0x50 * block);
        e.mem.set_f32(base + 0x10, 1.0);
        e.mem.set_u32(base + 0x14, 0xffff_ffff);
        e.mem.set_u32(base + 0x40, 0);
        e.mem.set_u32(base + 0x20, 0xffff_ffff);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d9_8390, fn_00d98390(Ptr) -> Ptr)]
}

#[cfg(test)]
mod a09_tests {
    use super::*;

    #[test]
    fn every_block_gets_one_point_zero_and_the_markers() {
        let mut e = Engine::new();
        let array = e.mem.alloc(0x150);
        for word in 0..0x54 {
            e.mem.set_u32(array + 4 * word, 0x1234_5678);
        }
        assert_eq!(e.call(0x00d9_8390, &args![array]).u32(), array);
        for block in 0..4u32 {
            let base = array + 0x50 * block;
            assert_eq!(e.mem.u32(base + 0x10), 1.0f32.to_bits());
            assert_eq!(e.mem.u32(base + 0x14), 0xffff_ffff);
            assert_eq!(e.mem.u32(base + 0x20), 0xffff_ffff);
            assert_eq!(e.mem.u32(base + 0x40), 0);
            // The other words of the block are not touched.
            assert_eq!(e.mem.u32(base + 0x18), 0x1234_5678);
            assert_eq!(e.mem.u32(base + 0x44), 0x1234_5678);
        }
    }
}
