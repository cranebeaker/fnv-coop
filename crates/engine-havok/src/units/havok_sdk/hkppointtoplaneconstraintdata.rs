//! `Havok SDK/hkppointtoplaneconstraintdata.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpPointToPlaneConstraintData` keeps its constraint atoms from `+0x10` (`0x94` bytes of
//! them), a rotation (three rows) at `+0x20`, the pivot in body A's space at `+0x50`, the
//! plane's basis (three rows) at `+0x60`, the pivot in body B's space at `+0x90` and, in the
//! byte at `+0xa2`, the index of the row of the basis that holds the plane normal.

use std::cmp::Ordering;

use super::hkpsphereshape::{inverse_length, load_vec, shufps, store_vec, sum3, zip_with, Vec4};

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkpConstraintData::getConstraintInfoUtil` (`00d21a60`, cdecl `(atoms, size, info)`).
const GET_CONSTRAINT_INFO_UTIL: u32 = 0x00d2_1a60;
/// `hkVector4::setTransformedInversePos` (`00c7f690`, `this` = the result, `(transform, vector)`).
const VECTOR_SET_TRANSFORMED_INVERSE_POS: u32 = 0x00c7_f690;
/// The unnamed thiscall function at `004b4ab0` (`RET 8`): `this` receives the vector rotated by
/// the three rows at its first word, the vector being the second word.
const ROTATE_BY_MATRIX: u32 = 0x004b_4ab0;
/// `hkRotation::isOrthonormal` (`00cb2e70`, `this` = the rotation, `(result byte, epsilon)`,
/// `RET 8`); it returns the pointer to the result byte.
const IS_ORTHONORMAL: u32 = 0x00cb_2e70;
/// The `float` tolerance `isValid` passes to `isOrthonormal` (`01017d00`).
const ORTHONORMAL_EPSILON: u32 = 0x0101_7d00;

const ROTATION: u32 = 0x20;
const PIVOT_A: u32 = 0x50;
const BASIS: u32 = 0x60;
const PIVOT_B: u32 = 0x90;
const NORMAL_ROW: u32 = 0xa2;

/// `(a.yzx * b.zxy) - (a.zxy * b.yzx)`, the way the code forms the cross product `a x b`
/// (`SHUFPS` 0xC9 and 0xD2 with the multiplies in this order).
fn cross_yzx(a: Vec4, b: Vec4) -> Vec4 {
    let plus = zip_with(shufps(a, a, 0xc9), shufps(b, b, 0xd2), |x, y| x * y);
    let minus = zip_with(shufps(a, a, 0xd2), shufps(b, b, 0xc9), |x, y| x * y);
    zip_with(plus, minus, |x, y| x - y)
}

/// The rotation at `+0x20` becomes the identity (the three rows, each with a zero fourth lane).
fn reset_rotation(e: &mut Engine, data: u32) {
    for row in 0..3 {
        store_vec(e, data + ROTATION + 0x10 * row, [0.0; 4]);
        e.mem.set_f32(data + ROTATION + 0x10 * row + 4 * row, 1.0);
    }
}

/// The tail both `setInBodySpace` and the world-space variant share, once the normal is stored
/// in the basis row `index` (the byte at `+0xa2`): the row `(index + 1) % 3` becomes a vector
/// perpendicular to the normal, built from the normal's two components that the comparisons
/// of the absolute values pick (the `float`s go through the x87, so signalling NaNs are
/// quieted), then normalised with the refined `RSQRTSS`; the row `(index + 2) % 3` becomes the
/// cross product of the normal and that row. Returns `(index + 2) / 3` (the quotient the last
/// `IDIV` leaves in `EAX`).
// `(index + 2) / 3` is the exe's own add-then-divide.
#[allow(clippy::manual_div_ceil)]
fn complete_basis(e: &mut Engine, data: u32, index: u32) -> u32 {
    let normal = data + BASIS + 0x10 * index;
    let quiet: [f32; 3] = [0, 1, 2].map(|lane| x87_quiet(e.mem.f32(normal + 4 * lane)));
    let magnitude = quiet.map(f32::abs);
    let greater = |a: f32, b: f32| a.partial_cmp(&b) == Some(Ordering::Greater);
    let (smallest, mut chosen, other) = if greater(magnitude[0], magnitude[1]) {
        (magnitude[1], 1usize, 0usize)
    } else {
        (magnitude[0], 0usize, 1usize)
    };
    // Equal and unordered values count as "not greater" in both comparisons.
    if !greater(smallest, magnitude[2]) {
        chosen = 2;
    }
    let row = data + BASIS + 0x10 * ((index + 1) % 3);
    let mut perpendicular = [0.0f32; 4];
    perpendicular[other] = quiet[chosen];
    perpendicular[chosen] = -quiet[other];
    store_vec(e, row, perpendicular);
    let inverse = inverse_length(
        e,
        sum3(zip_with(perpendicular, perpendicular, |a, b| a * b)),
    );
    let unit = zip_with([inverse; 4], perpendicular, |a, b| a * b);
    store_vec(e, row, unit);
    let normal_vector = load_vec(e, normal);
    let unit = load_vec(e, row);
    store_vec(
        e,
        data + BASIS + 0x10 * ((index + 2) % 3),
        cross_yzx(normal_vector, unit),
    );
    (index + 2) / 3
}

// Translated from 00d84760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPointToPlaneConstraintData::getConstraintInfo` (named in the engine map): the shared
/// `getConstraintInfoUtil` on the atoms at `this + 0x10`, `0x94` bytes of them, filling `info`.
pub fn point_to_plane_get_constraint_info(e: &mut Engine, this: Ptr, info: u32) {
    e.call(
        GET_CONSTRAINT_INFO_UTIL,
        &args![this.addr() + 0x10, 0x94u32, info],
    );
}

// Translated from 00d84780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPointToPlaneConstraintData::getRuntimeInfo` (named in the engine map): `(size, count)`
/// is written to `info` as `(8, 1)` when `flag` (the low byte of the first word) is non-zero
/// and as `(0, 0)` otherwise.
pub fn point_to_plane_get_runtime_info(e: &mut Engine, _this: Ptr, flag: u8, info: Ptr) {
    let (size, count) = if flag != 0 { (8, 1) } else { (0, 0) };
    e.mem.set_u32(info.addr() + 4, count);
    e.mem.set_u32(info.addr(), size);
}

// Translated from 00d847b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPointToPlaneConstraintData::getType` (named in the engine map): the constraint type, 14.
pub fn point_to_plane_get_type(_e: &mut Engine, _this: Ptr) -> u32 {
    0xe
}

// Translated from 00d847c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An unnamed function of the unit (the engine map has no name for it; it is the world-space
/// counterpart of `setInBodySpace`, but that identity is unconfirmed). `transform_a` and
/// `transform_b` are the bodies' transforms, `pivot` a point and `normal` a direction in world
/// space. The pivot is carried into body A's space (`+0x50`) and body B's (`+0x90`) with
/// `setTransformedInversePos`, the rotation at `+0x20` becomes the identity, the normal
/// rotated by `transform_b` (`004b4ab0`) goes in the basis row the byte at `+0xa2` names, and
/// the rest of the basis is completed. Returns `(index + 2) / 3`.
pub fn fn_00d847c0(
    e: &mut Engine,
    this: Ptr,
    transform_a: u32,
    transform_b: u32,
    pivot: u32,
    normal: u32,
) -> u32 {
    let data = this.addr();
    e.call(
        VECTOR_SET_TRANSFORMED_INVERSE_POS,
        &args![data + PIVOT_A, transform_a, pivot],
    );
    e.call(
        VECTOR_SET_TRANSFORMED_INVERSE_POS,
        &args![data + PIVOT_B, transform_b, pivot],
    );
    reset_rotation(e, data);
    let index = u32::from(e.mem.u8(data + NORMAL_ROW));
    e.call(
        ROTATE_BY_MATRIX,
        &args![data + BASIS + 0x10 * index, transform_b, normal],
    );
    complete_basis(e, data, index)
}

// Translated from 00d849a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPointToPlaneConstraintData::setInBodySpace` (named in the engine map): copies the
/// pivots `pivot_a` and `pivot_b` (16 bytes each) to `+0x50` and `+0x90`, makes the rotation at
/// `+0x20` the identity, stores `normal` in the basis row the byte at `+0xa2` names and
/// completes the basis. Returns `(index + 2) / 3`.
pub fn point_to_plane_set_in_body_space(
    e: &mut Engine,
    this: Ptr,
    pivot_a: u32,
    pivot_b: u32,
    normal: u32,
) -> u32 {
    let data = this.addr();
    let a = load_vec(e, pivot_a);
    store_vec(e, data + PIVOT_A, a);
    let b = load_vec(e, pivot_b);
    store_vec(e, data + PIVOT_B, b);
    reset_rotation(e, data);
    let index = u32::from(e.mem.u8(data + NORMAL_ROW));
    let n = load_vec(e, normal);
    store_vec(e, data + BASIS + 0x10 * index, n);
    complete_basis(e, data, index)
}

// Translated from 00d84b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPointToPlaneConstraintData::isValid` (named in the engine map): the byte at `valid`
/// becomes 1 when both the rotation at `+0x20` and the basis at `+0x60` are orthonormal to
/// within the tolerance at `01017d00` (`hkRotation::isOrthonormal`), else 0. Returns `valid`.
pub fn point_to_plane_is_valid(e: &mut Engine, this: Ptr, valid: u32) -> u32 {
    let data = this.addr();
    let ok = e.with_stack(8, |e, scratch| {
        let result = scratch.addr();
        let epsilon = x87_quiet(e.global::<f32>(ORTHONORMAL_EPSILON));
        let got = e
            .call(
                IS_ORTHONORMAL,
                &args![data + ROTATION, result + 2, epsilon.to_bits()],
            )
            .u32();
        if e.mem.u8(got) == 0 {
            return false;
        }
        let epsilon = x87_quiet(e.global::<f32>(ORTHONORMAL_EPSILON));
        let got = e
            .call(
                IS_ORTHONORMAL,
                &args![data + BASIS, result + 3, epsilon.to_bits()],
            )
            .u32();
        e.mem.u8(got) != 0
    });
    e.mem.set_u8(valid, u8::from(ok));
    valid
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d8_4760, point_to_plane_get_constraint_info(Ptr, u32)),
        entry!(0x00d8_4780, point_to_plane_get_runtime_info(Ptr, u8, Ptr)),
        entry!(0x00d8_47b0, point_to_plane_get_type(Ptr) -> u32),
        entry!(0x00d8_47c0, fn_00d847c0(Ptr, u32, u32, u32, u32) -> u32),
        entry!(0x00d8_49a0, point_to_plane_set_in_body_space(Ptr, u32, u32, u32) -> u32),
        entry!(0x00d8_4b70, point_to_plane_is_valid(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod a05_point_to_plane_tests {
    use super::*;

    fn world() -> Engine {
        let mut e = Engine::new();
        for page in [0x0101_1000, 0x0101_7000, 0x0101_6000] {
            e.map(page, 0x1000);
        }
        e.mem.set_f32(0x0101_1d78, 0.0);
        e.mem.set_f32(0x0101_7718, 3.0);
        e.mem.set_f32(0x0101_6248, 0.5);
        e.mem.set_f32(ORTHONORMAL_EPSILON, 0.001);
        e.call_log = Some(vec![]);
        e
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    fn row(e: &Engine, at: u32) -> [f32; 4] {
        [0, 1, 2, 3].map(|lane| e.mem.f32(at + 4 * lane))
    }

    fn close(got: [f32; 4], want: [f32; 4]) {
        for lane in 0..4 {
            assert!(
                (got[lane] - want[lane]).abs() < 2e-6,
                "{got:?} differs from {want:?}"
            );
        }
    }

    #[test]
    fn constraint_info_runtime_info_and_type() {
        let mut e = world();
        e.register_double(GET_CONSTRAINT_INFO_UTIL, |_, _| Ret::default());
        let data = e.mem.alloc(0xb0);
        let info = e.mem.alloc(0x10);
        e.call(0x00d8_4760, &args![data, info]);
        assert_eq!(
            calls_to(&e, GET_CONSTRAINT_INFO_UTIL),
            vec![vec![data + 0x10, 0x94, info]]
        );
        e.call(0x00d8_4780, &args![data, 1u32, info]);
        assert_eq!((e.mem.u32(info), e.mem.u32(info + 4)), (8, 1));
        // Only the low byte of the flag word counts.
        e.call(0x00d8_4780, &args![data, 0x100u32, info]);
        assert_eq!((e.mem.u32(info), e.mem.u32(info + 4)), (0, 0));
        assert_eq!(e.call(0x00d8_47b0, &args![data]).u32(), 0xe);
    }

    #[test]
    fn body_space_basis_for_a_normal_along_z() {
        let mut e = world();
        let data = e.mem.alloc(0xb0);
        let (a, b, normal) = (e.mem.alloc(0x10), e.mem.alloc(0x10), e.mem.alloc(0x10));
        for lane in 0..4 {
            e.mem.set_f32(a + 4 * lane, 1.0 + lane as f32);
            e.mem.set_f32(b + 4 * lane, 5.0 + lane as f32);
        }
        e.mem.set_f32(normal + 8, 1.0);
        e.mem.set_u8(data + 0xa2, 0);
        let result = e.call(0x00d8_49a0, &args![data, a, b, normal]).u32();
        assert_eq!(result, 0);
        assert_eq!(row(&e, data + 0x50), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(row(&e, data + 0x90), [5.0, 6.0, 7.0, 8.0]);
        assert_eq!(row(&e, data + 0x20), [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(row(&e, data + 0x30), [0.0, 1.0, 0.0, 0.0]);
        assert_eq!(row(&e, data + 0x40), [0.0, 0.0, 1.0, 0.0]);
        assert_eq!(row(&e, data + 0x60), [0.0, 0.0, 1.0, 0.0]);
        close(row(&e, data + 0x70), [0.0, 1.0, 0.0, 0.0]);
        close(row(&e, data + 0x80), [-1.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn body_space_basis_picks_components_by_magnitude_and_wraps_the_rows() {
        let mut e = world();
        let data = e.mem.alloc(0xb0);
        let (a, b, normal) = (e.mem.alloc(0x10), e.mem.alloc(0x10), e.mem.alloc(0x10));
        // |x| > |y| and |z| < the smaller: the first comparison fails, so the pair is (1, 0),
        // and `smallest > z` holds, so the chosen component stays 1.
        e.mem.set_f32(normal, 3.0);
        e.mem.set_f32(normal + 4, 2.0);
        e.mem.set_f32(normal + 8, 1.0);
        e.mem.set_u8(data + 0xa2, 2);
        let result = e.call(0x00d8_49a0, &args![data, a, b, normal]).u32();
        assert_eq!(result, 1);
        assert_eq!(row(&e, data + 0x80), [3.0, 2.0, 1.0, 0.0]);
        // other = 0, chosen = 1: (x, y) = (normal.y, -normal.x) = (2, -3), normalised.
        let length = 13.0f32.sqrt();
        close(
            row(&e, data + 0x60),
            [2.0 / length, -3.0 / length, 0.0, 0.0],
        );
        // normal x unit = (2*0 - 1*(-3/l), 1*(2/l) - 3*0, 3*(-3/l) - 2*(2/l)) = (3, 2, -13) / l.
        close(
            row(&e, data + 0x70),
            [3.0 / length, 2.0 / length, -13.0 / length, 0.0],
        );
    }

    #[test]
    fn a_tie_on_the_last_component_picks_it() {
        let mut e = world();
        let data = e.mem.alloc(0xb0);
        let (a, b, normal) = (e.mem.alloc(0x10), e.mem.alloc(0x10), e.mem.alloc(0x10));
        // |x| == |y| == |z|: x <= y keeps the pair (0, 1), `smallest <= z` picks z.
        for lane in 0..3 {
            e.mem.set_f32(normal + 4 * lane, -1.0);
        }
        e.mem.set_u8(data + 0xa2, 1);
        e.call(0x00d8_49a0, &args![data, a, b, normal]);
        // other = 1, chosen = 2: row (index + 1) % 3 = 2 gets (0, normal.z, -normal.y) = (0, -1, 1).
        let root = 0.5f32.sqrt();
        close(row(&e, data + 0x80), [0.0, -root, root, 0.0]);
        assert_eq!(row(&e, data + 0x70), [-1.0, -1.0, -1.0, 0.0]);
    }

    #[test]
    fn a_zero_normal_leaves_a_zero_basis() {
        let mut e = world();
        let data = e.mem.alloc(0xb0);
        let (a, b, normal) = (e.mem.alloc(0x10), e.mem.alloc(0x10), e.mem.alloc(0x10));
        e.mem.set_u8(data + 0xa2, 0);
        e.call(0x00d8_49a0, &args![data, a, b, normal]);
        assert_eq!(row(&e, data + 0x70), [0.0; 4]);
        assert_eq!(row(&e, data + 0x80), [0.0; 4]);
    }

    #[test]
    fn world_space_variant_transforms_the_pivot_twice_and_rotates_the_normal() {
        let mut e = world();
        e.register_double(VECTOR_SET_TRANSFORMED_INVERSE_POS, |e, args| {
            // Marks the result with the transform's address.
            e.mem.set_u32(args[0], args[1]);
            Ret::default()
        });
        e.register_double(ROTATE_BY_MATRIX, |e, args| {
            for (lane, value) in [0.0f32, 0.0, 2.0, 0.0].into_iter().enumerate() {
                e.mem.set_f32(args[0] + 4 * lane as u32, value);
            }
            Ret::default()
        });
        let data = e.mem.alloc(0xb0);
        e.mem.set_u8(data + 0xa2, 0);
        let (ta, tb, pivot, normal) = (
            e.mem.alloc(0x40),
            e.mem.alloc(0x40),
            e.mem.alloc(0x10),
            e.mem.alloc(0x10),
        );
        let result = e
            .call(0x00d8_47c0, &args![data, ta, tb, pivot, normal])
            .u32();
        assert_eq!(result, 0);
        assert_eq!(
            calls_to(&e, VECTOR_SET_TRANSFORMED_INVERSE_POS),
            vec![vec![data + 0x50, ta, pivot], vec![data + 0x90, tb, pivot]]
        );
        assert_eq!(
            calls_to(&e, ROTATE_BY_MATRIX),
            vec![vec![data + 0x60, tb, normal]]
        );
        assert_eq!(row(&e, data + 0x30), [0.0, 1.0, 0.0, 0.0]);
        assert_eq!(row(&e, data + 0x60), [0.0, 0.0, 2.0, 0.0]);
        close(row(&e, data + 0x70), [0.0, 1.0, 0.0, 0.0]);
        close(row(&e, data + 0x80), [-2.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn is_valid_needs_both_rotations_to_pass() {
        let mut e = world();
        // The double reports the result recorded for the rotation it is asked about.
        e.register_double(IS_ORTHONORMAL, |e, args| {
            let verdict = e.mem.u8(args[0] + 0x30);
            e.mem.set_u8(args[1], verdict);
            Ret {
                eax: args[1],
                ..Ret::default()
            }
        });
        let data = e.mem.alloc(0xb0);
        let valid = e.mem.alloc(4);
        // The verdicts live 0x30 bytes after each rotation.
        for (first, second, want) in [(1u8, 1u8, 1u8), (1, 0, 0), (0, 1, 0), (0, 0, 0)] {
            e.mem.set_u8(data + 0x50, first);
            e.mem.set_u8(data + 0x90, second);
            e.mem.set_u8(valid, 7);
            let returned = e.call(0x00d8_4b70, &args![data, valid]).u32();
            assert_eq!(returned, valid);
            assert_eq!(e.mem.u8(valid), want);
        }
        let calls = calls_to(&e, IS_ORTHONORMAL);
        assert_eq!(calls.len(), 6);
        assert_eq!(calls[0][0], data + 0x20);
        assert_eq!(calls[1][0], data + 0x60);
        assert_eq!(calls[0][2], 0.001f32.to_bits());
    }
}
