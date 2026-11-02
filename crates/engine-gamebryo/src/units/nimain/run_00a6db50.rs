//! `NiMain/run_00a6db50` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Three small functions on a value of four consecutive `float`s (the class is not named in
//! the engine map; the code only shows four floats at +0, +4, +8, +0xc).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00a6db50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor from four floats: each goes through `FLD`/`FSTP` into +0, +4, +8, +0xc (a
/// signalling NaN is stored quiet). Returns `this` (`EAX`).
pub fn fn_00a6db50(
    e: &mut Engine,
    this: Ptr,
    first: f32,
    second: f32,
    third: f32,
    fourth: f32,
) -> Ptr {
    let base = this.addr();
    for (i, value) in [first, second, third, fourth].into_iter().enumerate() {
        e.mem
            .set_u32(base + 4 * i as u32, x87_quiet_bits(value.to_bits()));
    }
    this
}

// Translated from 00a6db70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sum: for each of the four floats, `out[i] = other[i] + this[i]` (`FLD` of `other`, `FADD`
/// of `this`, `FSTP` into `out`). Returns `out` (the value returned by hidden pointer).
pub fn fn_00a6db70(e: &mut Engine, this: Ptr, out: Ptr, other: Ptr) -> Ptr {
    for i in 0..4u32 {
        let sum = x87_add(
            x87_load32(e.mem.u32(other.addr() + 4 * i)),
            x87_m32(e.mem.u32(this.addr() + 4 * i)),
        );
        e.mem.set_u32(out.addr() + 4 * i, x87_store32(sum));
    }
    out
}

// Translated from 00a6dba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Difference: for each of the four floats, `out[i] = this[i] - other[i]` (`FLD` of `this`,
/// `FSUB` of `other`, `FSTP` into `out`). Returns `out` (the value returned by hidden
/// pointer).
pub fn fn_00a6dba0(e: &mut Engine, this: Ptr, out: Ptr, other: Ptr) -> Ptr {
    for i in 0..4u32 {
        let difference = x87_sub(
            x87_load32(e.mem.u32(this.addr() + 4 * i)),
            x87_m32(e.mem.u32(other.addr() + 4 * i)),
        );
        e.mem.set_u32(out.addr() + 4 * i, x87_store32(difference));
    }
    out
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a6_db50, fn_00a6db50(Ptr, f32, f32, f32, f32) -> Ptr),
        entry!(0x00a6_db70, fn_00a6db70(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00a6_dba0, fn_00a6dba0(Ptr, Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod b004_tests {
    use super::*;

    fn floats(e: &Engine, p: u32) -> Vec<u32> {
        (0..4).map(|i| e.mem.u32(p + 4 * i)).collect()
    }

    fn put(e: &mut Engine, values: [f32; 4]) -> u32 {
        let p = e.mem.alloc(0x10);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_u32(p + 4 * i as u32, v.to_bits());
        }
        p
    }

    #[test]
    fn constructor_stores_the_four_floats_quieting_a_signalling_nan() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        let r = e.call(
            0x00a6_db50,
            &[this, 1.5f32.to_bits(), 0x7f80_0001, (-2.0f32).to_bits(), 0],
        );
        assert_eq!(r.u32(), this);
        assert_eq!(
            floats(&e, this),
            vec![1.5f32.to_bits(), 0x7fc0_0001, (-2.0f32).to_bits(), 0]
        );
    }

    #[test]
    fn sum_and_difference_go_component_by_component() {
        let mut e = Engine::new();
        let a = put(&mut e, [1.0, 2.0, 3.0, 4.0]);
        let b = put(&mut e, [0.5, -1.0, 10.0, 0.25]);
        let out = e.mem.alloc(0x10);
        assert_eq!(e.call(0x00a6_db70, &args![a, out, b]).u32(), out);
        let want: Vec<u32> = [1.5f32, 1.0, 13.0, 4.25]
            .iter()
            .map(|v| v.to_bits())
            .collect();
        assert_eq!(floats(&e, out), want);
        assert_eq!(e.call(0x00a6_dba0, &args![a, out, b]).u32(), out);
        let want: Vec<u32> = [0.5f32, 3.0, -7.0, 3.75]
            .iter()
            .map(|v| v.to_bits())
            .collect();
        assert_eq!(floats(&e, out), want);
    }

    #[test]
    fn sum_and_difference_keep_the_x87_nan_choice() {
        let mut e = Engine::new();
        let a = put(&mut e, [f32::from_bits(0x7fc0_0001), 1.0, 1.0, 1.0]);
        let b = put(&mut e, [f32::from_bits(0xffc0_0002), 1.0, 1.0, 1.0]);
        let out = e.mem.alloc(0x10);
        e.call(0x00a6_db70, &args![a, out, b]);
        let expected = x87_store32(x87_add(x87_load32(0xffc0_0002), x87_m32(0x7fc0_0001)));
        assert_eq!(e.mem.u32(out), expected);
        e.call(0x00a6_dba0, &args![a, out, b]);
        let expected = x87_store32(x87_sub(x87_load32(0x7fc0_0001), x87_m32(0xffc0_0002)));
        assert_eq!(e.mem.u32(out), expected);
    }
}
