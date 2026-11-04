//! `fallout/ai/run_009a4960` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `006a7f50` (unnamed, `navmeshsearch.cpp` per the engine map): `thiscall`, returns the
/// float at `this` in `ST0`.
const READ_FLOAT: u32 = 0x006a_7f50;
/// `2.0` (`double`, `0x01011590`).
const TWO: u32 = 0x0101_1590;

// Translated from 009a4960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): whether `value` lies between the
/// floats at `+8` and `+0xc` (both read through `006a7f50`), ends included: false when
/// `value < low` or unordered (`FCOMPP`, `C0`), otherwise true only when `value < high`
/// or `value == high` (`C0` or `C3` alone; `JP` after `TEST AH,0x41`).
pub fn fn_009a4960(e: &mut Engine, this: Ptr, value: f32) -> bool {
    let low = e.call(READ_FLOAT, &args![this.addr() + 0x8]).f64();
    let value = x87_load32(value.to_bits());
    if !(value >= low) {
        return false;
    }
    let high = e.call(READ_FLOAT, &args![this.addr() + 0xc]).f64();
    // Exactly one of C0 (below) and C3 (equal) set; an unordered compare sets both
    // (parity even, false), as `<=` is false for a NaN.
    value <= high
}

// Translated from 009a49b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`): `this`'s float at `+0x10` times 2.0
/// (`0x01011590`) divided by a count: with `actor`, the integer its value owner at `+0xa4`
/// gives for 0x10 (slot `+8`) as a float; without, `fallback`. Rounded to a float.
pub fn fn_009a49b0(e: &mut Engine, this: Ptr, actor: u32, fallback: f32) -> f32 {
    let divisor = if actor != 0 {
        let owner = actor + 0xa4;
        let count = e.vcall(owner, 0x8, &args![0x10u32]).i32();
        x87_store32(f64::from(count))
    } else {
        x87_quiet_bits(fallback.to_bits())
    };
    let base = x87_load32(e.mem.u32(this.addr() + 0x10));
    let two = f64::from_bits(e.mem.u64(TWO));
    let product = x87_mul(base, two);
    let result = x87_store32(x87_div(product, x87_m32(divisor)));
    f32::from_bits(x87_quiet_bits(result))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009a_4960, fn_009a4960(Ptr, f32) -> bool),
        entry!(0x009a_49b0, fn_009a49b0(Ptr, u32, f32) -> f32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_009a4960_tests_the_closed_range() {
        let mut e = Engine::new();
        e.register(READ_FLOAT, |e, a| Ret {
            st0: f64::from(e.mem.f32(a[0])),
            ..Ret::default()
        });
        let this = e.mem.alloc(0x10);
        e.mem.set_f32(this + 8, 1.0);
        e.mem.set_f32(this + 0xc, 3.0);
        let inside = |e: &mut Engine, v: f32| e.call(0x009a_4960, &args![this, v]).bool();
        assert!(!inside(&mut e, 0.5));
        assert!(inside(&mut e, 1.0));
        assert!(inside(&mut e, 2.0));
        assert!(inside(&mut e, 3.0));
        assert!(!inside(&mut e, 3.5));
        assert!(!inside(&mut e, f32::NAN));
        e.mem.set_u32(this + 0xc, 0x7fc0_0000);
        assert!(!inside(&mut e, 2.0));
    }

    #[test]
    fn fn_009a49b0_divides_twice_the_float() {
        let mut e = Engine::new();
        e.map(TWO & !0xfff, 0x1000);
        e.mem.set_u64(TWO, 2.0f64.to_bits());
        let this = e.mem.alloc(0x14);
        e.mem.set_f32(this + 0x10, 3.0);
        assert_eq!(e.call(0x009a_49b0, &args![this, 0u32, 4.0f32]).f32(), 1.5);
        // With an actor: its value owner's slot +8 for 0x10 gives 3.
        e.put_vtable(0x7000, &[0, 0, 0x7100]);
        e.register(0x7100, |_, a| Ret {
            eax: if a[1] == 0x10 { 3 } else { 0 },
            ..Ret::default()
        });
        let actor = e.mem.alloc(0xa8);
        e.mem.set_u32(actor + 0xa4, 0x7000);
        assert_eq!(e.call(0x009a_49b0, &args![this, actor, 4.0f32]).f32(), 2.0);
    }
}
