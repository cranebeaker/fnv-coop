//! `Havok SDK/run_00d10890` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The table the function fills: 31 entries of six floats from `01269210`.
const TABLE_01269210: u32 = 0x0126_9210;
/// The double `3.1415927` (a float's value) the angle is measured from.
const PI_DOUBLE: u32 = 0x0101_ff40;
/// The double step between two entries (`2 * pi / 30`, a float's value).
const ANGLE_STEP: u32 = 0x010c_d5a8;
/// The float `3.1415927` and the float `-1.0`.
const PI_FLOAT: u32 = 0x0102_b3c8;
const MINUS_ONE: u32 = 0x0101_2054;
/// The CRT helpers (value in and out in `ST0`; the other translations use them as cosine and sine).
const CRT_FIRST_TRIG: u32 = 0x00ec_9f70;
const CRT_SECOND_TRIG: u32 = 0x00ec_a0a0;

/// A float result as the code stores it: `FSTP dword` to a stack slot, `FLD dword` back, and
/// `FSTP dword` into the table.
fn float_bits_through_stack(value: f64) -> u32 {
    x87_store32(x87_load32(x87_store32(value)))
}

/// A CRT helper called with `value` in `ST0`, its result stored as a float through the stack.
fn crt_float_bits(e: &mut Engine, helper: u32, value: f64) -> u32 {
    float_bits_through_stack(e.call(helper, &args![value]).f64())
}

// Translated from 00d10890 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, two float words `low`, `high`): fills the table of 31 entries of six floats
/// at `01269210`. Entry `i` is made for the angle `f = float(i * step - pi)` (the constants are
/// doubles in the exe's data; `f` is rounded to a float). With `f < 0` (`g = float(pi - |f|)`,
/// `h = float(g - low)`): `1.0`, `0.0`, `first(h > high ? high : (0 < h ? h : 0))`,
/// `first(0 < h ? h : 0)`, `first(g)`, `second(g)`. Otherwise (also for a NaN `f`), with
/// `p = float(f + low)` and `q = float(pi - high)`: `first(f)`, `second(f)`,
/// `first(p < pi ? p : pi_float)`, `first(q > p ? q : (p < pi ? p : pi_float))`, `-1.0` (the float
/// at `01012054`) and `0.0`. `first` and `second` are the CRT helpers `00ec9f70` and `00eca0a0`
/// (the other translations use them as cosine and sine), each result stored as a float.
pub fn fn_00d10890(e: &mut Engine, low: f32, high: f32) {
    let low_bits = low.to_bits();
    let high = x87_load32(high.to_bits());
    let pi = e.global::<f64>(PI_DOUBLE);
    let step = e.global::<f64>(ANGLE_STEP);
    for index in 0..31u32 {
        let entry = TABLE_01269210 + index * 0x18;
        let angle = x87_load32(x87_store32(x87_sub(x87_mul(f64::from(index), step), pi)));
        if angle < 0.0 {
            let magnitude = x87_load32(x87_store32(angle.abs()));
            let reflected_bits = x87_store32(x87_sub(pi, magnitude));
            let reflected = x87_load32(reflected_bits);
            let remaining = x87_load32(x87_store32(x87_sub(reflected, x87_m32(low_bits))));
            e.mem.set_u32(entry, 1.0f32.to_bits());
            e.mem.set_u32(entry + 4, 0);
            let first_argument = if remaining > high {
                high
            } else if remaining > 0.0 {
                remaining
            } else {
                0.0
            };
            let bits = crt_float_bits(e, CRT_FIRST_TRIG, first_argument);
            e.mem.set_u32(entry + 8, bits);
            let clamped = if remaining > 0.0 { remaining } else { 0.0 };
            let bits = crt_float_bits(e, CRT_FIRST_TRIG, clamped);
            e.mem.set_u32(entry + 0xc, bits);
            let bits = crt_float_bits(e, CRT_FIRST_TRIG, reflected);
            e.mem.set_u32(entry + 0x10, bits);
            let bits = crt_float_bits(e, CRT_SECOND_TRIG, reflected);
            e.mem.set_u32(entry + 0x14, bits);
        } else {
            let bits = crt_float_bits(e, CRT_FIRST_TRIG, angle);
            e.mem.set_u32(entry, bits);
            let bits = crt_float_bits(e, CRT_SECOND_TRIG, angle);
            e.mem.set_u32(entry + 4, bits);
            let sum = x87_add(angle, x87_m32(low_bits));
            let sum_bits = x87_store32(sum);
            let shifted = x87_load32(sum_bits);
            let pi_float = x87_load32(e.mem.u32(PI_FLOAT));
            let limited = if shifted < pi { shifted } else { pi_float };
            let bits = crt_float_bits(e, CRT_FIRST_TRIG, x87_load32(x87_store32(limited)));
            e.mem.set_u32(entry + 8, bits);
            let other = x87_load32(x87_store32(x87_sub(pi, high)));
            let second_argument = if other > shifted {
                other
            } else if shifted < pi {
                shifted
            } else {
                pi_float
            };
            let bits = crt_float_bits(e, CRT_FIRST_TRIG, second_argument);
            e.mem.set_u32(entry + 0xc, bits);
            let minus_one = x87_store32(x87_load32(e.mem.u32(MINUS_ONE)));
            e.mem.set_u32(entry + 0x10, minus_one);
            e.mem.set_u32(entry + 0x14, 0);
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d1_0890, fn_00d10890(f32, f32))]
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;

    /// The CRT helpers as `value + 100` and `value + 200`, so the argument each entry was
    /// computed from can be read back.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_u64(PI_DOUBLE, 0x4009_21fb_6000_0000);
        e.mem.set_u64(ANGLE_STEP, 0x3fca_ceea_0000_0000);
        e.mem.set_u32(PI_FLOAT, 0x4049_0fdb);
        e.mem.set_u32(MINUS_ONE, 0xbf80_0000);
        fn shifted(a: &[u32], add: f64) -> Ret {
            Ret {
                st0: f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) + add,
                ..Ret::default()
            }
        }
        e.register(CRT_FIRST_TRIG, |_, a| shifted(a, 100.0));
        e.register(CRT_SECOND_TRIG, |_, a| shifted(a, 200.0));
        e
    }

    fn float_of(value: f64, add: f64) -> u32 {
        ((value + add) as f32).to_bits()
    }

    #[test]
    fn fn_00d10890_fills_the_table_for_both_signs_of_the_angle() {
        let (low, high) = (0.25f32, 0.5f32);
        let mut e = engine();
        e.call(0x00d1_0890, &args![low, high]);
        let pi = f64::from_bits(0x4009_21fb_6000_0000);
        let step = f64::from_bits(0x3fca_ceea_0000_0000);
        let pi_float = f64::from(f32::from_bits(0x4049_0fdb));
        for index in 0..31u32 {
            let entry = TABLE_01269210 + index * 0x18;
            let angle = (f64::from(index) * step - pi) as f32;
            let words: Vec<u32> = (0..6).map(|k| e.mem.u32(entry + k * 4)).collect();
            if angle < 0.0 {
                let reflected = (pi - f64::from(angle.abs())) as f32;
                let remaining = (f64::from(reflected) - f64::from(low)) as f32;
                let first = if remaining > high {
                    f64::from(high)
                } else if remaining > 0.0 {
                    f64::from(remaining)
                } else {
                    0.0
                };
                let clamped = f64::from(remaining.max(0.0));
                let expected = [
                    1.0f32.to_bits(),
                    0,
                    float_of(first, 100.0),
                    float_of(clamped, 100.0),
                    float_of(f64::from(reflected), 100.0),
                    float_of(f64::from(reflected), 200.0),
                ];
                assert_eq!(words, expected, "entry {index}");
            } else {
                let angle = f64::from(angle);
                let shifted = (angle + f64::from(low)) as f32;
                let limited = if f64::from(shifted) < pi {
                    f64::from(shifted)
                } else {
                    pi_float
                };
                let other = (pi - f64::from(high)) as f32;
                let second = if other > shifted {
                    f64::from(other)
                } else {
                    limited
                };
                let expected = [
                    float_of(angle, 100.0),
                    float_of(angle, 200.0),
                    float_of(limited, 100.0),
                    float_of(second, 100.0),
                    (-1.0f32).to_bits(),
                    0,
                ];
                assert_eq!(words, expected, "entry {index}");
            }
        }
    }
}
