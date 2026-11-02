//! `Havok SDK/hkswepttransform.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions
//! in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkSweptTransform` (Xbox PDB, size 0x50; layout from the PC code): the centre of mass at the
//! start (`+0`) and at the end (`+0x10`) of the step, the start time in the `w` lane of the
//! first (`+0xC`) and the inverse of the step length in the `w` lane of the second (`+0x1C`),
//! the rotation quaternions at the start (`+0x20`) and the end (`+0x30`) and the centre of mass
//! in the body's frame (`+0x40`).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkquaternion::{
    add_ps, load_vec, mul_ps, refined_rsqrt, shufps, store_vec, sub_ps, sum_four,
};

/// `hkRotation::set` (`00cb2d90`, `this` = the rotation's 0x30 bytes, `(quaternion)`) and the
/// image's vector of ones (`010ec7f0`).
const ROTATION_SET: u32 = 0x00cb_2d90;
const VECTOR_ONES: u32 = 0x010e_c7f0;

// Translated from 00cf22d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSweptTransform::approxTransformAt` (Xbox PDB), `(time, transform)`: with `a = ((time -
/// start time) * inverse step length)` (x87, stored as a float) the interpolated and normalized
/// rotation `(1 - a) * q0 + q1 * a` (the refined reciprocal square root of its square sum) goes
/// into the transform's rotation columns (`hkRotation::set`, `00cb2d90`) and the translation at
/// `+0x30` becomes `((1 - a) * c0 + c1 * a)` minus the centre of mass rotated by the columns,
/// `y * column1 + x * column0 + z * column2`.
pub fn hk_swept_transform_approx_transform_at(e: &mut Engine, this: Ptr, time: f32, out: Ptr) {
    let start = e.mem.u32(this.addr() + 0xc);
    let inverse = e.mem.u32(this.addr() + 0x1c);
    let alpha = x87_store32(x87_mul(
        x87_sub(x87_load32(time.to_bits()), x87_m32(start)),
        x87_m32(inverse),
    ));
    let a = [f32::from_bits(alpha); 4];
    let ones = load_vec(e, VECTOR_ONES);
    let rotation = add_ps(
        mul_ps(sub_ps(ones, a), load_vec(e, this.addr() + 0x20)),
        mul_ps(load_vec(e, this.addr() + 0x30), a),
    );
    let scale = refined_rsqrt(e, sum_four(mul_ps(rotation, rotation)));
    let rotation = mul_ps([scale; 4], rotation);
    e.with_stack(0x10, |e, quaternion| {
        store_vec(e, quaternion.addr(), rotation);
        e.call(ROTATION_SET, &args![out, quaternion]);
    });
    let ones = load_vec(e, VECTOR_ONES);
    let centre = add_ps(
        mul_ps(sub_ps(ones, a), load_vec(e, this.addr())),
        mul_ps(load_vec(e, this.addr() + 0x10), a),
    );
    store_vec(e, out.addr() + 0x30, centre);
    let offset = load_vec(e, this.addr() + 0x40);
    let column0 = load_vec(e, out.addr());
    let column1 = load_vec(e, out.addr() + 0x10);
    let by_y = mul_ps(shufps(offset, offset, 0x55), column1);
    let by_x = mul_ps(shufps(offset, offset, 0), column0);
    let sum = add_ps(by_y, by_x);
    let by_z = mul_ps(shufps(offset, offset, 0xaa), load_vec(e, out.addr() + 0x20));
    let rotated = add_ps(sum, by_z);
    let translation = sub_ps(load_vec(e, out.addr() + 0x30), rotated);
    store_vec(e, out.addr() + 0x30, translation);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00cf_22d0,
        hk_swept_transform_approx_transform_at(Ptr, f32, Ptr)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Vec4 = [f32; 4];

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0020_0000);
        store_vec(&mut e, VECTOR_ONES, [1.0; 4]);
        // The constants of the refined reciprocal square root.
        e.mem.set_f32(0x0101_1d78, 0.0);
        e.mem.set_f32(0x0101_7718, 3.0);
        e.mem.set_f32(0x0101_6248, 0.5);
        e
    }

    fn close(actual: Vec4, expected: Vec4) {
        for lane in 0..4 {
            assert!(
                (actual[lane] - expected[lane]).abs() < 1e-4,
                "lane {lane}: {actual:?} against {expected:?}"
            );
        }
    }

    /// A swept transform: start and end centres of mass (the `w` lanes are the start time and
    /// the inverse step length), the two rotations and the body-frame offset.
    fn swept(e: &mut Engine, rotations: [Vec4; 2], offset: Vec4) -> u32 {
        let this = e.mem.alloc(0x50);
        store_vec(e, this, [1.0, 2.0, 3.0, 2.0]);
        store_vec(e, this + 0x10, [5.0, 6.0, 7.0, 0.5]);
        store_vec(e, this + 0x20, rotations[0]);
        store_vec(e, this + 0x30, rotations[1]);
        store_vec(e, this + 0x40, offset);
        this
    }

    /// `hkRotation::set` as a double: logs the quaternion it is given and writes the identity
    /// columns.
    fn rotation_double(e: &mut Engine) -> Rc<RefCell<Vec<Vec4>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&log);
        e.register_double(ROTATION_SET, move |e, a| {
            seen.borrow_mut().push(load_vec(e, a[1]));
            store_vec(e, a[0], [1.0, 0.0, 0.0, 0.0]);
            store_vec(e, a[0] + 0x10, [0.0, 1.0, 0.0, 0.0]);
            store_vec(e, a[0] + 0x20, [0.0, 0.0, 1.0, 0.0]);
            Ret::default()
        });
        log
    }

    #[test]
    fn approx_transform_at_the_end_of_the_step() {
        let mut e = engine();
        let log = rotation_double(&mut e);
        let identity = [0.0, 0.0, 0.0, 1.0];
        let this = swept(&mut e, [identity, identity], [1.0, 0.0, 0.0, 0.0]);
        let out = e.mem.alloc(0x40);
        // time 4: a = (4 - 2) * 0.5 = 1, the end centre of mass; the body-frame offset
        // (1, 0, 0) along column 0 is subtracted.
        e.call(0x00cf_22d0, &args![this, 4.0f32, out]);
        close(log.borrow()[0], identity);
        close(load_vec(&e, out + 0x30), [4.0, 6.0, 7.0, 0.5]);
    }

    #[test]
    fn approx_transform_in_the_middle_interpolates_and_normalizes_the_rotation() {
        let mut e = engine();
        let log = rotation_double(&mut e);
        let this = swept(
            &mut e,
            [[0.0, 0.0, 0.0, 1.0], [0.0, 0.0, 1.0, 0.0]],
            [0.0, 2.0, 0.0, 0.0],
        );
        let out = e.mem.alloc(0x40);
        // time 3: a = 0.5.
        e.call(0x00cf_22d0, &args![this, 3.0f32, out]);
        let h = std::f32::consts::FRAC_1_SQRT_2;
        close(log.borrow()[0], [0.0, 0.0, h, h]);
        // Centre of mass halfway: (3, 4, 5, 1.25); the offset (0, 2, 0) along column 1.
        close(load_vec(&e, out + 0x30), [3.0, 2.0, 5.0, 1.25]);
    }
}
