//! `NiMain/run_00a92660` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00a92660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the four floats at `this` equal the four at `other`, compared
/// in order (x87 equality: a NaN is never equal); the engine map has no
/// name. Answers 1 or 0 in `EAX`.
pub fn fn_00a92660(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    for i in 0..4 {
        let left = e.mem.f32(this.addr() + 4 * i);
        let right = e.mem.f32(other.addr() + 4 * i);
        if right != left {
            return false;
        }
    }
    true
}

// Translated from 00a926b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The inequality of four floats (`thiscall`, the other four; `RET 4`; the engine map has no
/// name): 1 as soon as one pair differs (x87 `FUCOMPP`, a NaN differs from everything),
/// otherwise 0, in `EAX`.
pub fn fn_00a926b0(e: &mut Engine, this: Ptr, other: Ptr) -> u32 {
    for i in 0..4 {
        let left = x87_load32(e.mem.u32(this.addr() + 4 * i));
        let right = x87_load32(e.mem.u32(other.addr() + 4 * i));
        if right != left {
            return 1;
        }
    }
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a9_2660, fn_00a92660(Ptr, Ptr) -> bool),
        entry!(0x00a9_26b0, fn_00a926b0(Ptr, Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn four_floats_compare_equal_only_when_all_match() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        let other = e.mem.alloc(0x10);
        for i in 0..4 {
            e.mem.set_f32(this + 4 * i, i as f32);
            e.mem.set_f32(other + 4 * i, i as f32);
        }
        assert_eq!(e.call(0x00a9_2660, &args![this, other]).u32(), 1);
        e.mem.set_f32(other + 12, 9.0);
        assert_eq!(e.call(0x00a9_2660, &args![this, other]).u32(), 0);
        e.mem.set_f32(other + 12, 3.0);
        e.mem.set_f32(this, f32::NAN);
        e.mem.set_f32(other, f32::NAN);
        assert_eq!(e.call(0x00a9_2660, &args![this, other]).u32(), 0);
        // Signed zeros are equal.
        e.mem.set_f32(this, -0.0);
        e.mem.set_f32(other, 0.0);
        assert_eq!(e.call(0x00a9_2660, &args![this, other]).u32(), 1);
    }
}

#[cfg(test)]
mod inequality_tests {
    use super::*;

    #[test]
    fn four_floats_differ_when_any_pair_does() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        let other = e.mem.alloc(0x10);
        for i in 0..4 {
            e.mem.set_f32(this + 4 * i, i as f32);
            e.mem.set_f32(other + 4 * i, i as f32);
        }
        assert_eq!(e.call(0x00a9_26b0, &args![this, other]).u32(), 0);
        for i in 0..4 {
            e.mem.set_f32(other + 4 * i, 9.0);
            assert_eq!(e.call(0x00a9_26b0, &args![this, other]).u32(), 1);
            e.mem.set_f32(other + 4 * i, i as f32);
        }
        e.mem.set_f32(this + 8, f32::NAN);
        e.mem.set_f32(other + 8, f32::NAN);
        assert_eq!(e.call(0x00a9_26b0, &args![this, other]).u32(), 1);
    }
}
