//! `Havok SDK/hkqstransform.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkQsTransform` (not in the Xbox PDB; layout from the PC code) is three 16-byte vectors: the
//! translation at `+0`, the rotation quaternion at `+0x10` and the scale at `+0x20`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkquaternion::{load_vec, mul_ps, store_vec, sub_ps, sum_four, Vec4};

/// `hkQuaternion::set` (`00cb26e0`, `this` = the quaternion, `(rotation)`) and
/// `hkRotation::set` (`00cb2d90`, `this` = the rotation, `(quaternion)`).
const QUATERNION_SET: u32 = 0x00cb_26e0;
const ROTATION_SET: u32 = 0x00cb_2d90;
/// The image's vector of ones (`010c7170`), the sign mask of the absolute value (`01017d10`, four
/// words) and the float `-1.0` (`01012054`).
const VECTOR_ONES: u32 = 0x010c_7170;
const ABS_MASK: u32 = 0x0101_7d10;
const FLOAT_MINUS_ONE: u32 = 0x0101_2054;

// Translated from 00ce5070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkQsTransform::setFromTransformNoScale` (Xbox PDB), `(transform)`: the rotation of a 0x40-byte
/// transform (three columns and the translation at `+0x30`) goes to the quaternion at `+0x10`
/// (`hkQuaternion::set`), the translation to `+0`, and the scale at `+0x20` becomes the image's
/// vector of ones.
pub fn hk_qs_transform_set_from_transform_no_scale(e: &mut Engine, this: Ptr, transform: Ptr) {
    e.call(QUATERNION_SET, &args![this.addr() + 0x10, transform]);
    let translation = load_vec(e, transform.addr() + 0x30);
    store_vec(e, this.addr(), translation);
    let ones = load_vec(e, VECTOR_ONES);
    store_vec(e, this.addr() + 0x20, ones);
}

// Translated from 00ce50b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: writes the transform into a 0x40-byte one (`(transform)`): the
/// rotation columns from the quaternion at `+0x10` (`hkRotation::set`) and the translation at
/// `+0x30`.
pub fn fn_00ce50b0(e: &mut Engine, this: Ptr, transform: Ptr) {
    e.call(ROTATION_SET, &args![transform, this.addr() + 0x10]);
    let translation = load_vec(e, this.addr());
    store_vec(e, transform.addr() + 0x30, translation);
}

// Translated from 00ce50e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map, `(out)`: stores through `out` a byte that is 1 when every one of
/// the 12 floats of the transform except lanes 3 and 11 (the unused lane of the translation and
/// of the scale) has a finite value (the exponent bits are not all ones, after the `FLD`/`FSTP`
/// copy), else 0. Returns `out`.
pub fn fn_00ce50e0(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    for index in 0..12u32 {
        if index == 3 || index == 11 {
            continue;
        }
        let bits = x87_quiet_bits(e.mem.u32(this.addr() + 4 * index));
        if bits & 0x7f80_0000 == 0x7f80_0000 {
            e.mem.set_u8(out.addr(), 0);
            return out;
        }
    }
    e.mem.set_u8(out.addr(), 1);
    out
}

/// `CMPLEPS` of `|a - b|` (the absolute-value mask on the bits) against `epsilon` in every lane.
fn within(e: &Engine, a: Vec4, b: Vec4, epsilon: f32) -> [bool; 4] {
    let mask: [u32; 4] = std::array::from_fn(|lane| e.mem.u32(ABS_MASK + 4 * lane as u32));
    let difference = sub_ps(a, b);
    std::array::from_fn(|lane| f32::from_bits(difference[lane].to_bits() & mask[lane]) <= epsilon)
}

// Translated from 00ce5120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkQsTransform::isApproximatelyEqual` (Xbox PDB), `(out, other, epsilon)`: stores through
/// `out` a byte that is 1 when the first three lanes of the translations (`+0`) and of the
/// scales (`+0x20`) differ by at most `epsilon` (a NaN never does) and the quaternions
/// (`+0x10`), this one first negated when their dot product is below 0 (`(p3 + p1) + (p2 + p0)`
/// of the lane products), differ by at most `epsilon` in all four lanes. Returns `out`.
pub fn hk_qs_transform_is_approximately_equal(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
    other: Ptr,
    epsilon: f32,
) -> Ptr {
    let first = within(
        e,
        load_vec(e, this.addr()),
        load_vec(e, other.addr()),
        epsilon,
    );
    let second = within(
        e,
        load_vec(e, this.addr() + 0x20),
        load_vec(e, other.addr() + 0x20),
        epsilon,
    );
    if !(first[..3].iter().all(|lane| *lane) && second[..3].iter().all(|lane| *lane)) {
        e.mem.set_u8(out.addr(), 0);
        return out;
    }
    let mut quaternion = load_vec(e, this.addr() + 0x10);
    let target = load_vec(e, other.addr() + 0x10);
    if sum_four(mul_ps(quaternion, target)) < 0.0 {
        let minus_one = e.mem.f32(FLOAT_MINUS_ONE);
        quaternion = mul_ps([minus_one; 4], quaternion);
    }
    let close = within(e, quaternion, target, epsilon);
    e.mem
        .set_u8(out.addr(), u8::from(close.iter().all(|lane| *lane)));
    out
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00ce_5070,
            hk_qs_transform_set_from_transform_no_scale(Ptr, Ptr)
        ),
        entry!(0x00ce_50b0, fn_00ce50b0(Ptr, Ptr)),
        entry!(0x00ce_50e0, fn_00ce50e0(Ptr, Ptr) -> Ptr),
        entry!(0x00ce_5120, hk_qs_transform_is_approximately_equal(Ptr, Ptr, Ptr, f32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0020_0000);
        store_vec(&mut e, VECTOR_ONES, [1.0; 4]);
        for lane in 0..4 {
            e.mem.set_u32(ABS_MASK + 4 * lane, 0x7fff_ffff);
        }
        e.mem.set_f32(FLOAT_MINUS_ONE, -1.0);
        e
    }

    /// A transform at a fresh block: translation, quaternion, scale.
    fn transform(e: &mut Engine, translation: Vec4, quaternion: Vec4, scale: Vec4) -> u32 {
        let block = e.mem.alloc(0x40);
        store_vec(e, block, translation);
        store_vec(e, block + 0x10, quaternion);
        store_vec(e, block + 0x20, scale);
        block
    }

    #[test]
    fn set_from_transform_no_scale_takes_the_rotation_and_the_translation() {
        let mut e = engine();
        let log = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&log);
        e.register_double(QUATERNION_SET, move |_, a| {
            seen.borrow_mut().push((a[0], a[1]));
            Ret::default()
        });
        let source = e.mem.alloc(0x40);
        store_vec(&mut e, source + 0x30, [1.0, 2.0, 3.0, 4.0]);
        let this = transform(&mut e, [0.0; 4], [0.0; 4], [7.0; 4]);
        e.call(0x00ce_5070, &args![this, source]);
        assert_eq!(*log.borrow(), vec![(this + 0x10, source)]);
        assert_eq!(load_vec(&e, this), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(load_vec(&e, this + 0x20), [1.0; 4]);
    }

    #[test]
    fn fn_00ce50b0_writes_the_rotation_and_the_translation() {
        let mut e = engine();
        let log = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&log);
        e.register_double(ROTATION_SET, move |_, a| {
            seen.borrow_mut().push((a[0], a[1]));
            Ret::default()
        });
        let this = transform(&mut e, [5.0, 6.0, 7.0, 8.0], [0.0; 4], [1.0; 4]);
        let target = e.mem.alloc(0x40);
        e.call(0x00ce_50b0, &args![this, target]);
        assert_eq!(*log.borrow(), vec![(target, this + 0x10)]);
        assert_eq!(load_vec(&e, target + 0x30), [5.0, 6.0, 7.0, 8.0]);
    }

    #[test]
    fn fn_00ce50e0_checks_the_ten_used_lanes_for_finite_values() {
        let mut e = engine();
        let out = e.mem.alloc(4);
        let this = transform(&mut e, [1.0; 4], [0.5; 4], [1.0; 4]);
        assert_eq!(e.call(0x00ce_50e0, &args![this, out]).u32(), out);
        assert_eq!(e.mem.u8(out), 1);
        // Lanes 3 and 11 are not looked at.
        e.mem.set_f32(this + 12, f32::INFINITY);
        e.mem.set_f32(this + 44, f32::NAN);
        e.call(0x00ce_50e0, &args![this, out]);
        assert_eq!(e.mem.u8(out), 1);
        // Any other lane that is infinite or NaN gives 0.
        for lane in [0u32, 2, 4, 7, 8, 10] {
            let broken = transform(&mut e, [1.0; 4], [0.5; 4], [1.0; 4]);
            e.mem.set_u32(broken + 4 * lane, 0x7f80_0001);
            e.call(0x00ce_50e0, &args![broken, out]);
            assert_eq!(e.mem.u8(out), 0, "lane {lane}");
        }
        let infinite = transform(&mut e, [1.0; 4], [0.5; 4], [1.0; 4]);
        e.mem.set_f32(infinite + 20, f32::NEG_INFINITY);
        e.call(0x00ce_50e0, &args![infinite, out]);
        assert_eq!(e.mem.u8(out), 0);
    }

    fn equal(e: &mut Engine, a: u32, b: u32, epsilon: f32) -> bool {
        let out = e.mem.alloc(4);
        let got = e.call(0x00ce_5120, &args![a, out, b, epsilon]).u32();
        assert_eq!(got, out);
        e.mem.u8(out) != 0
    }

    #[test]
    fn is_approximately_equal_compares_the_three_vectors() {
        let mut e = engine();
        let h = std::f32::consts::FRAC_1_SQRT_2;
        let a = transform(
            &mut e,
            [1.0, 2.0, 3.0, 0.0],
            [0.0, 0.0, h, h],
            [1.0, 1.0, 1.0, 1.0],
        );
        let same = transform(
            &mut e,
            [1.0, 2.0, 3.0, 9.0],
            [0.0, 0.0, h, h],
            [1.0, 1.0, 1.0, 5.0],
        );
        // The fourth lane of the translation and of the scale is not compared.
        assert!(equal(&mut e, a, same, 0.01));
        let moved = transform(&mut e, [1.0, 2.0, 3.1, 0.0], [0.0, 0.0, h, h], [1.0; 4]);
        assert!(!equal(&mut e, a, moved, 0.01));
        assert!(equal(&mut e, a, moved, 0.2));
        let scaled = transform(
            &mut e,
            [1.0, 2.0, 3.0, 0.0],
            [0.0, 0.0, h, h],
            [1.0, 1.5, 1.0, 1.0],
        );
        assert!(!equal(&mut e, a, scaled, 0.01));
        let turned = transform(&mut e, [1.0, 2.0, 3.0, 0.0], [0.0, 0.0, 0.0, 1.0], [1.0; 4]);
        assert!(!equal(&mut e, a, turned, 0.01));
        // The opposite sign of a quaternion is the same rotation.
        let flipped = transform(&mut e, [1.0, 2.0, 3.0, 0.0], [0.0, 0.0, -h, -h], [1.0; 4]);
        assert!(equal(&mut e, a, flipped, 0.01));
        // A NaN is never within the tolerance.
        let broken = transform(
            &mut e,
            [f32::NAN, 2.0, 3.0, 0.0],
            [0.0, 0.0, h, h],
            [1.0; 4],
        );
        assert!(!equal(&mut e, a, broken, 1.0));
    }
}
