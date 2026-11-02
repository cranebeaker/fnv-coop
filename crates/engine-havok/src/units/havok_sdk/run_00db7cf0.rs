//! `Havok SDK/run_00db7cf0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00db7cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`: `this`, `index`, and a second stack word the
/// code never reads, `_unused_1`). Answers the word at `index` of the array whose pointer is
/// at `this + 0x10`.
pub fn fn_00db7cf0(e: &mut Engine, this: u32, index: u32, _unused_1: u32) -> u32 {
    let array = e.mem.u32(this + 0x10);
    e.mem.u32(array.wrapping_add(index.wrapping_mul(4)))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00db7cf0, fn_00db7cf0(u32, u32, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_word_at_the_index_of_the_array_is_answered() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0x10, array);
        for i in 0..4 {
            e.mem.set_u32(array + 4 * i, 0x100 + i);
        }
        assert_eq!(e.call(0x00db_7cf0, &args![this, 2u32, 0u32]).u32(), 0x102);
        assert_eq!(e.call(0x00db_7cf0, &args![this, 0u32, 7u32]).u32(), 0x100);
    }
}
