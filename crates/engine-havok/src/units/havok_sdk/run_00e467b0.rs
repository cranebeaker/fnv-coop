//! `Havok SDK/run_00e467b0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00e467b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 0xc` (`this` is not read; no reference to it was found in the exe's data):
/// writes the byte 1 to `out`. The other two stack words are not read.
/// Answers `out`: the exe leaves it in `EAX` at every `RET` (the hkBool result returned
/// through a pointer), and virtual call sites test the flag through `EAX` (e.g. 00c90f6a).
pub fn fn_00e467b0(e: &mut Engine, _this: Ptr, out: u32, _unused_2: u32, _unused_3: u32) -> u32 {
    e.mem.set_u8(out, 1);
    out
}

// Translated from 00e467c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Virtual, `RET 4`: does nothing; the stack word is not read.
pub fn fn_00e467c0(_e: &mut Engine, _this: Ptr, _unused_1: u32) {}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e4_67b0, fn_00e467b0(Ptr, u32, u32, u32) -> u32),
        entry!(0x00e4_67c0, fn_00e467c0(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_one_is_written_to_the_output() {
        let mut e = Engine::new();
        let out = e.mem.alloc(8);
        e.mem.set_u32(out, 0xaabb_ccdd);
        e.call(0x00e4_67b0, &args![0x1000u32, out, 5u32, 6u32]);
        assert_eq!(e.mem.u32(out), 0xaabb_cc01);
    }

    #[test]
    fn fn_00e467b0_answers_its_result_pointer() {
        let mut e = Engine::new();
        let out = e.mem.alloc(8);
        e.mem.set_u32(out, 0xaabb_ccdd);
        // The exe leaves the result pointer in EAX at every RET; callers read through it.
        assert_eq!(
            e.call(0x00e4_67b0, &args![0x1000u32, out, 5u32, 6u32])
                .u32(),
            args![out][0]
        );
    }

    #[test]
    fn empty_virtual_changes_nothing() {
        let mut e = Engine::new();
        let ret = e.call(0x00e4_67c0, &args![0x1000u32, 5u32]);
        assert_eq!(ret.u32(), 0);
    }
}
