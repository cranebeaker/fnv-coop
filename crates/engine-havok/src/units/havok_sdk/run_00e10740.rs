//! `Havok SDK/run_00e10740` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00e10740 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a "less than" comparison for a sort, cdecl, two stack words (nothing
/// popped): answers 1 when the float at +4 of `b` is greater than the float at +4 of `a`
/// (`FCOMPP` with the exe's `TEST AH, 0x41` / `JNZ`: equal and unordered answer 0), else 0. Ghidra made no function here; the address is pushed at 00e0fd23.
pub fn fn_00e10740(e: &mut Engine, a: u32, b: u32) -> u32 {
    u32::from(e.mem.f32(b + 4) > e.mem.f32(a + 4))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e1_0740, fn_00e10740(u32, u32) -> u32)]
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;

    #[test]
    fn fn_00e10740_answers_one_only_for_a_greater_second_float() {
        let mut e = Engine::new();
        for (a, b, expected) in [
            (1.0f32, 2.0f32, 1),
            (2.0, 1.0, 0),
            (2.0, 2.0, 0),
            (f32::NAN, 1.0, 0),
            (1.0, f32::NAN, 0),
        ] {
            let first = e.mem.alloc(0x10);
            let second = e.mem.alloc(0x10);
            e.mem.set_f32(first + 4, a);
            e.mem.set_f32(second + 4, b);
            assert_eq!(e.call(0x00e1_0740, &args![first, second]).u32(), expected);
        }
    }
}
