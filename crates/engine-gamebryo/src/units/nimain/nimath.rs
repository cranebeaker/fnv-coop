//! `NiMain/nimath.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiFastATan2`'s constants: the `float` pi/2 (`011ab3f8`), the `float`
/// answered for a zero ratio and a non-positive `x` (`0109e238`), the
/// `double` added or taken off in the left half-plane (`0101ff40`), and the
/// polynomial's `double` coefficients (`0109e260`, `0109e258`,
/// `0109e250`, `0109e248`, `0109e240`).
const HALF_PI: u32 = 0x011a_b3f8;
const ZERO_RATIO_ANSWER: u32 = 0x0109_e238;
const HALF_TURN: u32 = 0x0101_ff40;
const COEFFICIENT_0: u32 = 0x0109_e260;
const COEFFICIENT_1: u32 = 0x0109_e258;
const COEFFICIENT_2: u32 = 0x0109_e250;
const COEFFICIENT_3: u32 = 0x0109_e248;
const COEFFICIENT_4: u32 = 0x0109_e240;

/// A `double` memory operand (`FADD`/`FSUB qword`): the plain bits.
fn double_operand(e: &Engine, address: u32) -> f64 {
    f64::from_bits(e.global::<u64>(address))
}

/// `FSTP dword` then `FLD dword` of the same slot.
fn through_float(value: f64) -> f64 {
    x87_load32(x87_store32(value))
}

// Translated from 00a811f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFastATan2` (Xbox PDB; `cdecl (y, x)`, the result in ST0): an
/// approximation of `atan2(y, x)`. Both zero: 0. With `|x| < |y|` (an
/// ordered, strict comparison) the ratio `x / y` is taken and the base is
/// `+pi/2` for a positive ratio, `-pi/2` for a negative one; a zero (or
/// NaN) ratio answers `+pi/2` for a positive `y`, else `-pi/2`. Otherwise
/// the ratio is `y / x` with base 0; a zero ratio answers 0 for a positive
/// `x`, else the constant at `0109e238`. The polynomial `r * (((((c0 * s -
/// c1) * s + c2) * s - c3) * s + c4)` with `s = r * r` (each step stored
/// as a `float`) is taken off a nonzero base or answered as it is; then
/// the half turn is taken off for a negative `y` and negative `x`, or added
/// for a positive `y` and negative `x`. Every comparison follows the exe's
/// flag tests (a NaN takes the branch its flags give).
pub fn ni_fast_a_tan2(e: &mut Engine, y: f32, x: f32) -> f32 {
    let x = x87_load32(x.to_bits());
    let y = x87_load32(y.to_bits());
    if x == 0.0 && y == 0.0 {
        return 0.0;
    }
    let absolute_y = through_float(y.abs());
    let absolute_x = through_float(x.abs());
    let half_pi = x87_load32(e.global::<u32>(HALF_PI));
    let (ratio, base) = if absolute_x < absolute_y {
        let ratio = through_float(x87_div(x, y));
        if ratio > 0.0 {
            (ratio, half_pi)
        } else if ratio < 0.0 {
            (ratio, -half_pi)
        } else {
            // A zero or NaN ratio.
            let answer = if 0.0 < y { half_pi } else { -half_pi };
            return f32::from_bits(x87_store32(answer));
        }
    } else {
        let ratio = through_float(x87_div(y, x));
        if ratio == 0.0 {
            if x > 0.0 {
                return 0.0;
            }
            let answer = x87_load32(e.global::<u32>(ZERO_RATIO_ANSWER));
            return f32::from_bits(x87_store32(answer));
        }
        (ratio, 0.0)
    };
    let square = through_float(x87_mul(ratio, ratio));
    let mut term = through_float(x87_mul(x87_load64(e.global::<u64>(COEFFICIENT_0)), square));
    term = through_float(x87_sub(term, double_operand(e, COEFFICIENT_1)));
    term = through_float(x87_mul(term, square));
    term = through_float(x87_add(term, double_operand(e, COEFFICIENT_2)));
    term = through_float(x87_mul(term, square));
    let term_bits = x87_store32(x87_sub(term, double_operand(e, COEFFICIENT_3)));
    let term_bits = x87_store32(x87_mul(square, x87_m32(term_bits)));
    let term_bits = x87_store32(x87_add(
        x87_load32(term_bits),
        double_operand(e, COEFFICIENT_4),
    ));
    let mut result = x87_store32(x87_mul(ratio, x87_m32(term_bits)));
    let base = through_float(base);
    if base != 0.0 {
        result = x87_store32(x87_sub(base, x87_m32(result)));
    }
    let half_turn = x87_load64(e.global::<u64>(HALF_TURN));
    if 0.0 > y && 0.0 > x {
        result = x87_store32(x87_sub(x87_load32(result), half_turn));
    }
    if y > 0.0 && 0.0 > x {
        result = x87_store32(x87_add(half_turn, x87_m32(result)));
    }
    f32::from_bits(result)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a8_11f0, ni_fast_a_tan2(f32, f32) -> f32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An engine with the approximation's constants (the classic rational
    /// coefficients, pi and pi/2).
    fn atan_engine() -> Engine {
        let mut e = Engine::new();
        e.map(HALF_PI, 4);
        e.set_global(HALF_PI, std::f32::consts::FRAC_PI_2);
        e.map(ZERO_RATIO_ANSWER, 4);
        e.set_global(ZERO_RATIO_ANSWER, std::f32::consts::PI);
        e.map(HALF_TURN, 8);
        e.set_global(HALF_TURN, f64::from(std::f32::consts::PI));
        e.map(COEFFICIENT_4, 0x28);
        e.set_global(COEFFICIENT_0, 0.020_835_1_f64);
        e.set_global(COEFFICIENT_1, 0.085_133_f64);
        e.set_global(COEFFICIENT_2, 0.180_141_f64);
        e.set_global(COEFFICIENT_3, 0.330_299_5_f64);
        e.set_global(COEFFICIENT_4, 0.999_866_f64);
        e
    }

    fn atan(e: &mut Engine, y: f32, x: f32) -> f32 {
        e.call(0x00a8_11f0, &args![y, x]).f32()
    }

    #[test]
    fn fast_atan2_covers_each_quadrant_and_axis() {
        let mut e = atan_engine();
        let pi = std::f32::consts::PI;
        let cases = [
            (1.0f32, 1.0f32, pi / 4.0),
            (1.0, -1.0, 3.0 * pi / 4.0),
            (-1.0, -1.0, -3.0 * pi / 4.0),
            (-1.0, 1.0, -pi / 4.0),
            (2.0, 1.0, 1.107_148_7),
            (-2.0, 1.0, -1.107_148_7),
            (2.0, -1.0, 2.034_444),
            (0.5, 3.0, 0.165_148_68),
        ];
        for (y, x, expected) in cases {
            let answer = atan(&mut e, y, x);
            assert!((answer - expected).abs() < 1e-4, "{y} {x}: {answer}");
        }
    }

    #[test]
    fn fast_atan2_handles_zero_ratios() {
        let mut e = atan_engine();
        let half_pi = std::f32::consts::FRAC_PI_2;
        assert_eq!(atan(&mut e, 0.0, 0.0), 0.0);
        assert_eq!(atan(&mut e, 0.0, 2.0), 0.0);
        assert_eq!(atan(&mut e, 0.0, -2.0), std::f32::consts::PI);
        // |x| < |y| with x = 0: the ratio is zero, the answer +-pi/2.
        assert_eq!(atan(&mut e, 3.0, 0.0), half_pi);
        assert_eq!(atan(&mut e, -3.0, 0.0), -half_pi);
        // A NaN x: |x| < |y| is false; the ratio NaN goes through the polynomial.
        assert!(atan(&mut e, 1.0, f32::NAN).is_nan());
    }
}
