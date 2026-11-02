//! `Havok SDK/run_00ce0c40` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// A C runtime function that takes its argument in `ST0` and returns its result in `ST0`
/// (`00ec9650`; unnamed; from the clamp in `00ce0c40` it is the arc sine). In the uniform
/// form its argument is a leading `f64` (two words).
const ARC_FUNCTION: u32 = 0x00ec_9650;
/// The float `1.0` (`01012068`).
const ONE: u32 = 0x0101_2068;
/// The two floats returned when the magnitude is at least 1: `0101ff38` for a positive and
/// `01016b78` for any other argument (the values are 1.5707964 and -1.5707964).
const POSITIVE_RESULT: u32 = 0x0101_ff38;
const OTHER_RESULT: u32 = 0x0101_6b78;

// Translated from 00ce0c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// cdecl `(x)`, result in `ST0`: the magnitude `|x|` is rounded to a float (`FABS`, `FSTP`,
/// `FLD`) and compared with `1.0`; below it (or unordered, so also for a NaN) the C runtime
/// function `00ec9650` is called with `x` (as loaded on the x87 stack) and its result is
/// rounded to a float; otherwise the result is the float at `0101ff38` when `0 < x` and the one
/// at `01016b78` when not, each copied through the x87 stack.
pub fn fn_00ce0c40(e: &mut Engine, x: f32) -> f32 {
    let loaded = x87_load32(x.to_bits());
    let magnitude = x87_load32(x87_store32(loaded.abs()));
    let one: f32 = e.global(ONE);
    // FCOMP then TEST AH,1: taken when the magnitude is below 1.0 or unordered.
    if !(magnitude >= f64::from(one)) {
        let result = e.call(ARC_FUNCTION, &args![loaded]).f64();
        return f32::from_bits(x87_store32(result));
    }
    let bits = if 0.0 < loaded {
        e.mem.u32(POSITIVE_RESULT)
    } else {
        e.mem.u32(OTHER_RESULT)
    };
    f32::from_bits(x87_quiet_bits(bits))
}

// Translated from 00ce0ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (a quaternion product): `this` in `ECX` (a quaternion of four floats,
/// 16-byte aligned) and one stack word, `other` (`RET 4`). Replaces `this` by the product
/// `this * other` in single precision: with `a` = `this` and `b` = `other`, the vector part is
/// `b.w * a.v + (a.w * b.v + a Ã— b)` and the last lane `a.w * b.w - (a.z * b.z + (a.y * b.y + a.x * b.x))`
/// (the order the SSE code adds in).
pub fn fn_00ce0ca0(e: &mut Engine, this: u32, other: u32) {
    let a: [f32; 4] = std::array::from_fn(|i| e.mem.f32(this + 4 * i as u32));
    let b: [f32; 4] = std::array::from_fn(|i| e.mem.f32(other + 4 * i as u32));
    let cross = |i: usize| -> f32 {
        let (j, k) = ((i + 1) % 3, (i + 2) % 3);
        a[j] * b[k] - a[k] * b[j]
    };
    let vector: [f32; 3] = std::array::from_fn(|i| b[3] * a[i] + (a[3] * b[i] + cross(i)));
    let products: [f32; 4] = std::array::from_fn(|i| a[i] * b[i]);
    let sum = products[2] + (products[1] + products[0]);
    for (i, value) in vector.iter().enumerate() {
        e.mem.set_f32(this + 4 * i as u32, *value);
    }
    e.mem.set_f32(this + 12, products[3] - sum);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ce_0c40, fn_00ce0c40(f32) -> f32),
        entry!(0x00ce_0ca0, fn_00ce0ca0(u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn rig() -> (Engine, Rc<RefCell<Vec<f64>>>) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_f32(ONE, 1.0);
        e.mem.set_u32(POSITIVE_RESULT, 0x3fc9_0fdb);
        e.mem.set_u32(OTHER_RESULT, 0xbfc9_0fdb);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(ARC_FUNCTION, move |_, a| {
            let x = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            sink.borrow_mut().push(x);
            Ret {
                st0: x.asin(),
                ..Ret::default()
            }
        });
        (e, seen)
    }

    /// Below 1 the library function runs on the argument and its float-rounded result comes
    /// back; at 1 and above the constants answer by sign (`0 < x`).
    #[test]
    fn small_magnitudes_call_the_library_and_large_ones_clamp() {
        let (mut e, seen) = rig();
        let got = e.call(0x00ce_0c40, &args![0.5f32]).f32();
        assert_eq!(got, 0.5f64.asin() as f32);
        assert_eq!(*seen.borrow(), vec![0.5]);
        let got = e.call(0x00ce_0c40, &args![-0.25f32]).f32();
        assert_eq!(got, (-0.25f64).asin() as f32);
        for (x, expected) in [
            (1.0f32, 0x3fc9_0fdbu32),
            (7.5, 0x3fc9_0fdb),
            (-1.0, 0xbfc9_0fdb),
            (f32::NEG_INFINITY, 0xbfc9_0fdb),
            (f32::INFINITY, 0x3fc9_0fdb),
        ] {
            let got = e.call(0x00ce_0c40, &args![x]).f32();
            assert_eq!(got.to_bits(), expected, "x = {x}");
        }
        assert_eq!(seen.borrow().len(), 2);
    }

    /// A NaN is unordered against 1.0, so it takes the library path (and its signalling
    /// form is quieted by the load).
    #[test]
    fn a_nan_goes_to_the_library() {
        let (mut e, seen) = rig();
        e.call(0x00ce_0c40, &args![f32::from_bits(0x7f80_0001)]);
        assert_eq!(seen.borrow().len(), 1);
        assert!(seen.borrow()[0].is_nan());
    }
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;

    fn set_quaternion(e: &mut Engine, values: [f32; 4]) -> u32 {
        let at = e.mem.alloc(0x10);
        for (i, value) in values.iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, *value);
        }
        at
    }

    fn quaternion(e: &Engine, at: u32) -> [f32; 4] {
        std::array::from_fn(|i| e.mem.f32(at + 4 * i as u32))
    }

    /// (1, 2, 3, 4) * (5, 6, 7, 8): cross (-4, 8, -4), vector (24, 48, 48), scalar 32 - 38.
    #[test]
    fn fn_00ce0ca0_multiplies_two_quaternions() {
        let mut e = Engine::new();
        let a = set_quaternion(&mut e, [1.0, 2.0, 3.0, 4.0]);
        let b = set_quaternion(&mut e, [5.0, 6.0, 7.0, 8.0]);
        e.call(0x00ce_0ca0, &args![a, b]);
        assert_eq!(quaternion(&e, a), [24.0, 48.0, 48.0, -6.0]);
        assert_eq!(quaternion(&e, b), [5.0, 6.0, 7.0, 8.0]);
    }

    /// The identity (0, 0, 0, 1) on either side leaves the other quaternion unchanged.
    #[test]
    fn fn_00ce0ca0_has_the_identity_on_both_sides() {
        let mut e = Engine::new();
        let q = [0.5, -0.25, 0.125, 2.0];
        let a = set_quaternion(&mut e, q);
        let identity = set_quaternion(&mut e, [0.0, 0.0, 0.0, 1.0]);
        e.call(0x00ce_0ca0, &args![a, identity]);
        assert_eq!(quaternion(&e, a), q);
        let b = set_quaternion(&mut e, [0.0, 0.0, 0.0, 1.0]);
        let other = set_quaternion(&mut e, q);
        e.call(0x00ce_0ca0, &args![b, other]);
        assert_eq!(quaternion(&e, b), q);
    }
}
