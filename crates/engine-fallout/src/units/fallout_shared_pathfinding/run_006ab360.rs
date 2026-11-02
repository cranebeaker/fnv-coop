//! `fallout shared/pathfinding/run_006ab360` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006ab360 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the member at `this + 0xB4` (`ECX` only, no stack
/// words).
pub fn fn_006ab360(_e: &mut Engine, this: Ptr) -> Ptr {
    this.byte_add(0xb4)
}

// Translated from 006ab380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the three words at `this + 0x3c` (a point) into `result`
/// (`thiscall`; one word; `RET 4`) and returns `result`. The engine map
/// gives it no name.
pub fn fn_006ab380(e: &mut Engine, this: Ptr, result: Ptr) -> Ptr {
    for offset in [0, 4, 8] {
        let word = e.mem.u32(this.addr() + 0x3c + offset);
        e.mem.set_u32(result.addr() + offset, word);
    }
    result
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006a_b360, fn_006ab360(Ptr) -> Ptr),
        entry!(0x006a_b380, fn_006ab380(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod b028_tests {
    use super::*;

    #[test]
    fn fn_006ab360_returns_this_plus_0xb4() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x006a_b360, &args![0x4000u32]).u32(), 0x40b4);
    }

    #[test]
    fn fn_006ab380_copies_the_point_at_0x3c() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x48);
        for i in 0..3u32 {
            e.mem.set_u32(this + 0x3c + 4 * i, 0x7f80_0001 + i);
        }
        let result = e.mem.alloc(0xc);
        assert_eq!(e.call(0x006a_b380, &args![this, result]).u32(), result);
        // Integer moves: the words come across unchanged, NaNs included.
        for i in 0..3u32 {
            assert_eq!(e.mem.u32(result + 4 * i), 0x7f80_0001 + i);
        }
    }
}
