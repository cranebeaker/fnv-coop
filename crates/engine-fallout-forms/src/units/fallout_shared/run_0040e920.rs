//! `fallout shared/run_0040e920` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Three helpers on a three-word record that matches the Xbox PDB's
//! `ContainerItemExtra` (`pOwnerForm` +0, a union of `pOwnerGlobal` /
//! `iOwnerRank` +4, `fHealthMult` +8); the engine map gives them no names,
//! so the class is unconfirmed.

#[allow(unused_imports)]
use crate::prelude::*;

/// `1.0` (`double`).
const DOUBLE_ONE: u32 = 0x0101_2070;

// Translated from 0040e920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether two records differ (`thiscall (this; other)`): true without
/// `other`, when the words at +0 or +4 differ, or when the `float`s at +8
/// are not equal (`FUCOMPP`, unordered counts as different).
pub fn fn_0040e920(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if other.addr() == 0 {
        return true;
    }
    if e.mem.u32(this.addr()) != e.mem.u32(other.addr()) {
        return true;
    }
    if e.mem.u32(this.addr() + 4) != e.mem.u32(other.addr() + 4) {
        return true;
    }
    let mine = x87_load32(e.mem.u32(this.addr() + 8));
    let theirs = x87_load32(e.mem.u32(other.addr() + 8));
    // TEST AH,0x44 / JNP: only an equal compare (C3 alone) answers false.
    mine != theirs
}

// Translated from 0040e980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies a record (`thiscall (this; other)`): with `other`, its words at +0
/// and +4 and its `float` at +8 (through the x87 stack).
pub fn fn_0040e980(e: &mut Engine, this: Ptr, other: Ptr) {
    if other.addr() == 0 {
        return;
    }
    let owner = e.mem.u32(other.addr());
    e.mem.set_u32(this.addr(), owner);
    let word = e.mem.u32(other.addr() + 4);
    e.mem.set_u32(this.addr() + 4, word);
    let value = x87_quiet_bits(e.mem.u32(other.addr() + 8));
    e.mem.set_u32(this.addr() + 8, value);
}

// Translated from 0040e9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether a record holds nothing (`thiscall`): no word at +0 and the
/// `float` at +8 equal to 1.0 (the `double` at `01012070`).
pub fn fn_0040e9c0(e: &mut Engine, this: Ptr) -> bool {
    if e.mem.u32(this.addr()) != 0 {
        return false;
    }
    let value = x87_load32(e.mem.u32(this.addr() + 8));
    let one: f64 = e.global(DOUBLE_ONE);
    // FCOMP, TEST AH,0x44 / JNP: true only for an equal compare.
    value == one
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0040_e920, fn_0040e920(Ptr, Ptr) -> bool),
        entry!(0x0040_e980, fn_0040e980(Ptr, Ptr)),
        entry!(0x0040_e9c0, fn_0040e9c0(Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record(e: &mut Engine, owner: u32, word: u32, value: f32) -> u32 {
        let p = e.mem.alloc(0xc);
        e.mem.set_u32(p, owner);
        e.mem.set_u32(p + 4, word);
        e.mem.set_f32(p + 8, value);
        p
    }

    #[test]
    fn fn_0040e920_compares_each_field() {
        let mut e = Engine::new();
        let a = record(&mut e, 1, 2, 0.5);
        let same = record(&mut e, 1, 2, 0.5);
        let owner = record(&mut e, 3, 2, 0.5);
        let word = record(&mut e, 1, 4, 0.5);
        let value = record(&mut e, 1, 2, 0.75);
        let nan = record(&mut e, 1, 2, f32::NAN);
        let differs = |e: &mut Engine, b: u32| e.call(0x0040_e920, &args![a, b]).bool();
        assert!(!differs(&mut e, same));
        assert!(differs(&mut e, 0));
        assert!(differs(&mut e, owner));
        assert!(differs(&mut e, word));
        assert!(differs(&mut e, value));
        assert!(differs(&mut e, nan));
    }

    #[test]
    fn fn_0040e980_copies_the_record() {
        let mut e = Engine::new();
        let a = record(&mut e, 0, 0, 0.0);
        let b = record(&mut e, 7, 8, 1.5);
        e.call(0x0040_e980, &args![a, b]);
        assert_eq!(e.mem.bytes(a, 0xc), e.mem.bytes(b, 0xc));
        e.mem.set_u32(b + 8, 0x7f80_0001);
        e.call(0x0040_e980, &args![a, b]);
        assert_eq!(e.mem.u32(a + 8), 0x7fc0_0001, "a signalling NaN is quieted");
        e.call(0x0040_e980, &args![a, 0u32]);
        assert_eq!(e.mem.u32(a), 7);
    }

    #[test]
    fn fn_0040e9c0_is_true_for_no_owner_and_a_multiplier_of_one() {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x100);
        e.set_global(DOUBLE_ONE, 1.0f64);
        let empty = record(&mut e, 0, 5, 1.0);
        let owned = record(&mut e, 1, 0, 1.0);
        let damaged = record(&mut e, 0, 0, 0.5);
        assert!(e.call(0x0040_e9c0, &args![empty]).bool());
        assert!(!e.call(0x0040_e9c0, &args![owned]).bool());
        assert!(!e.call(0x0040_e9c0, &args![damaged]).bool());
    }
}
