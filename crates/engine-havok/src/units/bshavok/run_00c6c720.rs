//! `BSHavok/run_00c6c720` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `float` the slots of the table are initialised with (`FLT_MIN`, `1.1754944e-38`);
/// a slot that still holds it has not been written yet.
const UNSET_SLOT: u32 = 0x0102_efc4;
/// The `double` 0.5.
const HALF: u32 = 0x0101_1588;

// Translated from 00c6c720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (class unconfirmed). The object keeps a table of `float`s from +0x50. Slot `index`
/// takes `value` when it still holds the unset marker (`FLT_MIN`, compared exactly: a NaN is
/// not equal to it); otherwise it becomes the average of its old value and `value`.
pub fn fn_00c6c720(e: &mut Engine, this: Ptr, index: u32, value: f32) {
    let slot = this
        .addr()
        .wrapping_add(0x50)
        .wrapping_add(index.wrapping_mul(4));
    let unset: f32 = e.global(UNSET_SLOT);
    let old = e.mem.f32(slot);
    if old == unset {
        // FLD / FSTP: a signalling NaN comes back quiet.
        e.mem.set_f32(slot, x87_quiet(value));
        return;
    }
    // FADD into a stack float (rounded to single), then FMUL by the double 0.5.
    let sum = (old as f64 + value as f64) as f32;
    let half: f64 = e.global(HALF);
    e.mem.set_f32(slot, (sum as f64 * half) as f32);
}

// Translated from 00c6c760 (decompiled, FalloutNV.exe 1.4.0.525)
/// A virtual method that does nothing (the linker folded it into the slot of many vtables):
/// `this` and three stack words, none read.
pub fn fn_00c6c760(_e: &mut Engine, _this: Ptr, _unused_2: u32, _unused_3: u32, _unused_4: u32) {}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c6c720, fn_00c6c720(Ptr, u32, f32)),
        entry!(0x00c6c760, fn_00c6c760(Ptr, u32, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0102_e000, 0x1000);
        e.map(0x0101_1000, 0x1000);
        e.set_global(UNSET_SLOT, f32::MIN_POSITIVE);
        e.set_global(HALF, 0.5f64);
        e
    }

    #[test]
    fn an_unset_slot_takes_the_value() {
        let mut e = engine();
        let this = e.mem.alloc(0x80);
        e.mem.set_f32(this + 0x50 + 8, f32::MIN_POSITIVE);
        e.call(0x00c6_c720, &args![this, 2u32, 3.5f32]);
        assert_eq!(e.mem.f32(this + 0x58), 3.5);
        // The neighbours are untouched.
        assert_eq!(e.mem.f32(this + 0x5c), 0.0);
    }

    #[test]
    fn a_set_slot_becomes_the_average() {
        let mut e = engine();
        let this = e.mem.alloc(0x80);
        e.mem.set_f32(this + 0x50, 2.0);
        e.call(0x00c6_c720, &args![this, 0u32, 5.0f32]);
        assert_eq!(e.mem.f32(this + 0x50), 3.5);
    }

    #[test]
    fn a_nan_slot_is_not_unset_and_a_signalling_value_is_quieted() {
        let mut e = engine();
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0x50, 0x7fc0_0000);
        e.call(0x00c6_c720, &args![this, 0u32, 5.0f32]);
        assert!(e.mem.f32(this + 0x50).is_nan());
        e.mem.set_f32(this + 0x54, f32::MIN_POSITIVE);
        e.call(0x00c6_c720, &args![this, 1u32, f32::from_bits(0x7f80_0001)]);
        assert_eq!(e.mem.u32(this + 0x54), 0x7fc0_0001);
    }

    #[test]
    fn the_folded_virtual_method_does_nothing() {
        let mut e = engine();
        let ret = e.call(0x00c6_c760, &args![1u32, 2u32, 3u32, 4u32]);
        assert_eq!(ret, Ret::default());
    }
}
