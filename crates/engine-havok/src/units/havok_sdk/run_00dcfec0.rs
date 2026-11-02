//! `Havok SDK/run_00dcfec0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkgeometryprocessing::{
    add4, compare_flags, load_vec4, mul4, rotate_c9, rotate_d2, store_vec4, sub4, sum3, Vec4,
};

/// `hkMath_atan2fApproximation` (Xbox PDB, `00cbff80`, cdecl: two `float`s as words); answers a
/// `float` in `ST0`.
const ATAN2_APPROXIMATION: u32 = 0x00cb_ff80;
/// The `double` below which a length is taken as zero (`010d33d0`).
const LENGTH_LIMIT: u32 = 0x010d_33d0;

// Translated from 00dcfec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: `a`, `b`, `c`, `d` (16-byte vectors), `out_axis` (a pointer
/// to a 16-byte vector), `out_angle` (a pointer to a `float`)). Normalizes the sum `a + b` (all
/// four lanes; the length is the `SQRTSS` of the squares of lanes 0 to 2 summed in the code's
/// order): when the length is above `1e-16` (the `double` at `010d33d0`; a NaN is not) the axis
/// is the sum times `1 / length` (the division on the x87 stack, stored as a `float`, then
/// `MULPS`), otherwise it is `b`. With `u = rot_d2(d) * rot_c9(axis) - rot_c9(d) * rot_d2(axis)`
/// and `v = rot_c9(u) * rot_d2(axis) - rot_d2(u) * rot_c9(axis)` (the exe's `SHUFPS` 0xc9/0xd2
/// forms), the angle is `atan2fApproximation(c . u, c . v)` (lanes 0 to 2 summed in the code's
/// order, passed through `FLD`/`FSTP` as `float` bits), stored as a `float` at `out_angle`, and
/// the axis is stored at `out_axis`.
pub fn fn_00dcfec0(e: &mut Engine, a: u32, b: u32, c: u32, d: u32, out_axis: u32, out_angle: u32) {
    let vb = load_vec4(e, b);
    let va = load_vec4(e, a);
    let sum = add4(va, vb);
    let length = sum3(mul4(sum, sum)).sqrt();
    let limit = e.mem.f64(LENGTH_LIMIT);
    let flags = compare_flags(x87_load32(length.to_bits()), limit);
    let axis: Vec4 = if flags & 0x41 == 0 {
        let inverse = f32::from_bits(x87_store32(x87_div(1.0, x87_load32(length.to_bits()))));
        mul4([inverse; 4], sum)
    } else {
        vb
    };
    let vd = load_vec4(e, d);
    let rotated_axis = (rotate_c9(axis), rotate_d2(axis));
    let first = mul4(rotate_c9(vd), rotated_axis.1);
    let u = sub4(mul4(rotate_d2(vd), rotated_axis.0), first);
    let second = mul4(rotate_d2(u), rotated_axis.0);
    let third = mul4(rotate_c9(u), rotated_axis.1);
    let vc = load_vec4(e, c);
    let v = sub4(third, second);
    let along_u = sum3(mul4(vc, u));
    let along_v = sum3(mul4(vc, v));
    let first_argument = x87_quiet_bits(along_u.to_bits());
    let second_argument = x87_quiet_bits(along_v.to_bits());
    let angle = e
        .call(ATAN2_APPROXIMATION, &args![first_argument, second_argument])
        .f64();
    e.mem.set_u32(out_angle, x87_store32(angle));
    store_vec4(e, out_axis, axis);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00dcfec0,
        fn_00dcfec0(u32, u32, u32, u32, u32, u32)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn vector(e: &mut Engine, lanes: Vec4) -> u32 {
        let at = e.mem.alloc(16);
        store_vec4(e, at, lanes);
        at
    }

    /// Runs the function with an arctangent double that logs its two arguments and answers
    /// 0.75; answers the axis, the angle and the logged arguments.
    fn run(a: Vec4, b: Vec4, c: Vec4, d: Vec4) -> (Vec4, f32, Vec<f32>) {
        let mut e = Engine::new();
        e.map(LENGTH_LIMIT, 8);
        e.mem.set_f64(LENGTH_LIMIT, 1e-16);
        let log: Rc<RefCell<Vec<f32>>> = Rc::default();
        let seen = Rc::clone(&log);
        e.register_double(ATAN2_APPROXIMATION, move |_, w| {
            seen.borrow_mut()
                .extend(w.iter().map(|word| f32::from_bits(*word)));
            Ret {
                st0: 0.75,
                ..Ret::default()
            }
        });
        let (a, b, c, d) = (
            vector(&mut e, a),
            vector(&mut e, b),
            vector(&mut e, c),
            vector(&mut e, d),
        );
        let axis = e.mem.alloc(16);
        let angle = e.mem.alloc(4);
        e.call(0x00dc_fec0, &args![a, b, c, d, axis, angle]);
        let calls = log.borrow().clone();
        (load_vec4(&e, axis), e.mem.f32(angle), calls)
    }

    #[test]
    fn the_axis_is_the_normalized_sum_and_the_angle_comes_from_two_dot_products() {
        let (axis, angle, args) = run(
            [0.0, 0.0, 0.5, 0.0],
            [0.0, 0.0, 0.5, 0.0],
            [2.0, 3.0, 4.0, 0.0],
            [1.0, 0.0, 0.0, 0.0],
        );
        assert_eq!(axis, [0.0, 0.0, 1.0, 0.0]);
        // u = (0, 1, 0, 0) and v = (1, 0, 0, 0): the dot products with (2, 3, 4) are 3 and 2.
        assert_eq!(args, vec![3.0, 2.0]);
        assert_eq!(angle, 0.75);
    }

    #[test]
    fn a_zero_sum_keeps_the_second_vector_as_the_axis() {
        let (axis, _, args) = run(
            [0.0, 0.0, -2.0, 0.0],
            [0.0, 0.0, 2.0, 0.0],
            [2.0, 3.0, 4.0, 0.0],
            [1.0, 0.0, 0.0, 0.0],
        );
        assert_eq!(axis, [0.0, 0.0, 2.0, 0.0]);
        // u = (0, 2, 0, 0) and v = (4, 0, 0, 0).
        assert_eq!(args, vec![6.0, 8.0]);
    }

    #[test]
    fn a_nan_sum_is_not_normalized() {
        let (axis, _, _) = run(
            [f32::NAN, 0.0, 0.0, 0.0],
            [1.0, 2.0, 3.0, 4.0],
            [0.0; 4],
            [0.0; 4],
        );
        assert_eq!(axis, [1.0, 2.0, 3.0, 4.0]);
    }
}
