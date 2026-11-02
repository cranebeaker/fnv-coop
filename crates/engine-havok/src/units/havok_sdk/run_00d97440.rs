//! `Havok SDK/run_00d97440` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::havok_sdk::run_00d6bf60::{addps, load_vec, mulps};
use crate::x87::fistp;

/// The lower and upper start of the bisection (`float`s at `01017b78` and `01050cb0`), the
/// `float` `1.0` (`01012068`, lane 0 is broadcast), the `double` halving factor (`01011588`)
/// and the `double` offset (`01035718`) the result has taken off.
const LOWER_START: u32 = 0x0101_7b78;
const UPPER_START: u32 = 0x0105_0cb0;
const ONE: u32 = 0x0101_2068;
const HALF: u32 = 0x0101_1588;
const OFFSET: u32 = 0x0103_5718;
/// The 16-byte vectors added in the loops: `01267e30` (both) and `011b1bd0` (the first
/// function only).
const FIRST_ADDEND: u32 = 0x0126_7e30;
const SECOND_ADDEND: u32 = 0x011b_1bd0;

/// The value the x87 stack gives back through `FLD`/`FSTP` of a result: `FSTP float` then
/// `FLD float` quiets a signalling NaN.
fn return_value(bits: u32) -> f32 {
    f32::from_bits(x87_quiet_bits(bits))
}

/// `FLD float [hi]; FADD float [lo]` and the multiplication by the halving factor, stored as a
/// float: the next middle value, as bits.
fn middle(e: &Engine, hi: u32, lo: u32) -> u32 {
    let sum = x87_add(x87_load32(hi), x87_m32(lo));
    x87_store32(x87_mul(sum, e.mem.f64(HALF)))
}

/// The result both functions end with: `((hi + lo) * half) - offset` as a float.
fn bisection_result(e: &Engine, hi: u32, lo: u32) -> f32 {
    let sum = x87_add(x87_load32(hi), x87_m32(lo));
    let product = x87_mul(e.mem.f64(HALF), sum);
    return_value(x87_store32(x87_sub(product, e.mem.f64(OFFSET))))
}

// Translated from 00d97440 (decompiled, FalloutNV.exe 1.4.0.525)
/// No arguments, the result in `ST0`: 23 bisection steps between the floats at `01017b78` and
/// `01050cb0`. Each step takes `mid = (hi + lo) * 0.5` (x87, stored as a float), builds
/// `(broadcast(mid) + [01267e30]) * broadcast(1.0) + [011b1bd0]` with SSE and looks at the low
/// 16 bits of `bits(lane 0) >> 6`: below `0xb` the middle becomes the lower bound, otherwise the
/// upper. The answer is `((hi + lo) * 0.5) - 11.0` (the double at `01035718`). The exe's name for
/// it is unknown (the unit has none).
pub fn fn_00d97440(e: &mut Engine) -> f32 {
    let mut lo = x87_quiet_bits(e.mem.u32(LOWER_START));
    let mut hi = x87_quiet_bits(e.mem.u32(UPPER_START));
    let one = f32::from_bits(e.mem.u32(ONE));
    let first = load_vec(e, FIRST_ADDEND);
    let second = load_vec(e, SECOND_ADDEND);
    for _ in 0..0x17 {
        let mid = middle(e, hi, lo);
        let lanes = [f32::from_bits(mid); 4];
        let moved = addps(lanes, first);
        let scaled = mulps(moved, [one; 4]);
        let shifted = addps(scaled, second);
        let selector = (shifted[0].to_bits() >> 6) as u16;
        let quiet = x87_quiet_bits(mid);
        if selector < 0xb {
            lo = quiet;
        } else {
            hi = quiet;
        }
    }
    bisection_result(e, hi, lo)
}

// Translated from 00d974f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No arguments, the result in `ST0`: as `00d97440` but without the `011b1bd0` addend and with an
/// integer test: the four lanes of `(broadcast(mid) + [01267e30]) * broadcast(1.0)` are each
/// rounded with `FISTP` (nearest, ties to even; the integer indefinite for NaN or out of range) and
/// lane 0 is compared, unsigned, with `0xb` (the first step count minus 12, set once before the loop): below it
/// the middle becomes the lower bound, otherwise the upper. The other three integers are only
/// stored. The result is `((hi + lo) * 0.5) - 11.0`. The exe's name for it is unknown.
pub fn fn_00d974f0(e: &mut Engine) -> f32 {
    let mut lo = x87_quiet_bits(e.mem.u32(LOWER_START));
    let mut hi = x87_quiet_bits(e.mem.u32(UPPER_START));
    let one = f32::from_bits(e.mem.u32(ONE));
    // `LEA ECX,[EAX - 0xc]` runs once, before the loop (00d9751e): the limit stays 0x17 - 0xc.
    let limit = 0x17u32.wrapping_sub(0xc);
    for _ in 0..0x17u32 {
        let first = load_vec(e, FIRST_ADDEND);
        let mid = middle(e, hi, lo);
        let lanes = [f32::from_bits(mid); 4];
        let moved = addps(lanes, first);
        let scaled = mulps(moved, [one; 4]);
        // FLD / FSTP / FLD / FISTP of each lane: the stored integers are never read again but
        // lane 0.
        let integers: Vec<u32> = scaled
            .iter()
            .map(|lane| fistp(f64::from(x87_quiet(*lane))) as u32)
            .collect();
        let quiet = x87_quiet_bits(mid);
        if integers[0] < limit {
            lo = quiet;
        } else {
            hi = quiet;
        }
    }
    let sum = x87_add(x87_load32(hi), x87_m32(lo));
    let product = x87_mul(sum, e.mem.f64(HALF));
    return_value(x87_store32(x87_sub(product, e.mem.f64(OFFSET))))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d9_7440, fn_00d97440() -> f32),
        entry!(0x00d9_74f0, fn_00d974f0() -> f32),
    ]
}

#[cfg(test)]
mod a09_tests {
    use super::*;

    /// The constants as the exe holds them: 10.0, 11.0, 1.0, 0.5 and 11.0 (the last two as
    /// doubles), a zero first addend and 196608.0 in each lane of the second.
    fn engine(second_addend: f32) -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(LOWER_START, 10.0f32);
        e.set_global(UPPER_START, 11.0f32);
        e.set_global(ONE, 1.0f32);
        e.set_global(HALF, 0.5f64);
        e.set_global(OFFSET, 11.0f64);
        for lane in 0..4 {
            e.set_global(SECOND_ADDEND + 4 * lane, second_addend);
        }
        e
    }

    /// The bisection worked out with plain arithmetic: `take_low` says for each step number
    /// (1-based) whether the middle becomes the lower bound.
    fn expected(take_low: impl Fn(f32, u32) -> bool) -> f32 {
        let (mut lo, mut hi) = (10.0f32, 11.0f32);
        for step in (1..=0x17u32).rev() {
            let mid = ((f64::from(hi) + f64::from(lo)) * 0.5) as f32;
            if take_low(mid, step) {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        (((f64::from(hi) + f64::from(lo)) * 0.5) - 11.0) as f32
    }

    #[test]
    fn the_first_bisection_keeps_raising_the_lower_bound_while_the_low_bits_stay_below_0xb() {
        let mut e = engine(196608.0);
        let result = e.call(0x00d9_7440, &args![]).f32();
        // Adding 196608.0 leaves `round(mid * 64)` in the mantissa, so the selector is 10 until
        // `mid` rounds up to 11.0.
        let want = expected(|mid, _| (((mid + 196_608.0f32).to_bits() >> 6) as u16) < 0xb);
        assert_eq!(result.to_bits(), want.to_bits());
    }

    #[test]
    fn the_first_bisection_lowers_the_upper_bound_when_the_selector_is_large() {
        // A zero addend: the selector is taken from the bits of `mid` itself (0x0a00 for 10.5).
        let mut e = engine(0.0);
        let result = e.call(0x00d9_7440, &args![]).f32();
        let want = expected(|_, _| false);
        assert_eq!(result.to_bits(), want.to_bits());
        assert!(result < -0.99 && result > -1.01);
    }

    #[test]
    fn the_second_bisection_compares_the_rounded_middle_with_the_step_limit() {
        let mut e = engine(0.0);
        let result = e.call(0x00d9_74f0, &args![]).f32();
        // Round-to-nearest-even of the middle against 0xb (unsigned), set once before the loop.
        let want = expected(|mid, _step| {
            let rounded = fistp(f64::from(mid)) as u32;
            rounded < 0xb
        });
        assert_eq!(result.to_bits(), want.to_bits());
    }

    #[test]
    fn the_second_bisection_uses_the_first_addend() {
        let mut e = engine(0.0);
        for lane in 0..4 {
            e.set_global(FIRST_ADDEND + 4 * lane, 100.0f32);
        }
        let result = e.call(0x00d9_74f0, &args![]).f32();
        let want = expected(|mid, _step| {
            let rounded = fistp(f64::from(mid + 100.0)) as u32;
            rounded < 0xb
        });
        assert_eq!(result.to_bits(), want.to_bits());
    }

    #[test]
    fn a_nan_bound_stays_nan_in_the_result() {
        let mut e = engine(0.0);
        e.set_global(UPPER_START, 0x7f80_0001u32);
        let result = e.call(0x00d9_74f0, &args![]).f32();
        assert!(result.is_nan());
        assert_eq!(result.to_bits() & 0x0040_0000, 0x0040_0000);
    }
}
