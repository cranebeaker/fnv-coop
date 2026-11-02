//! `Havok SDK/hkphingeconstraintdata.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpHingeConstraintData` (Xbox PDB; vtable `010c91ec`) keeps its atoms from `+0x10` (type 2 at
//! `+0x10`, a second atom of type 12 at `+0xA0`). What the code does with the rest: each body has
//! three direction rows (`+0x20/+0x30/+0x40` for body A, `+0x60/+0x70/+0x80` for body B) and a
//! pivot (`+0x50`, `+0x90`); the notified-body mask is the byte at `+0xA6` and the maximum linear
//! impulse the float at `+0xA8`.
//!
//! The SSE code normalises with `RSQRTSS` (the host's estimate, refined once the way the code
//! does) and the x87 code (`FLD`/`FABS`/`FCHS`/`FSTP`) quiets a signalling NaN at each load.
//!
//! None of the functions has a /GS cookie or an exception frame.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpboxmotion::cross;
use crate::units::havok_sdk::hkpbvagent::LARGE_FLOAT;
use crate::units::havok_sdk::hkpsphereshape::{
    inverse_length, load_vec, store_vec, sum3, zip_with, Vec4,
};

/// `hkpConstraintData::getConstraintInfoUtil` (`00d21a60`, cdecl: atoms, size, info).
pub(crate) const CONSTRAINT_INFO_UTIL: u32 = 0x00d2_1a60;
/// `hkRotation::isOrthonormal` (`00cb2e70`, `this` the rotation, then the byte to fill and the
/// tolerance as a float; answers the byte's address).
pub(crate) const IS_ORTHONORMAL: u32 = 0x00cb_2e70;
/// `hkUFloat8::operator=` (`00ca9360`, `this` the byte buffer, then the address of a float).
pub(crate) const U_FLOAT8_ASSIGN: u32 = 0x00ca_9360;
/// `hkVector4::setTransformedInversePos` (`00c7f690`, `this` the output, then the transform
/// and the vector).
pub(crate) const SET_TRANSFORMED_INVERSE_POS: u32 = 0x00c7_f690;
/// The call at `004b4ab0` is a thunk to `004b4ad0` (`this` the output, then a transform and a
/// vector); its Xbox PDB name is not known (unconfirmed: it rotates the vector by the
/// transform's rotation).
pub(crate) const ROTATE_VECTOR: u32 = 0x004b_4ab0;
/// Vtable of the constraint data, and the tolerance `isValid` passes (a float).
const VTABLE: u32 = 0x010c_91ec;
pub(crate) const ORTHONORMAL_TOLERANCE: u32 = 0x0101_7d00;

/// `v` times `scale` in all four lanes (`SHUFPS 0` of the scale, `MULPS`).
pub(crate) fn scaled(v: Vec4, scale: f32) -> Vec4 {
    zip_with([scale; 4], v, |s, c| s * c)
}

/// The axis made unit length: the sum of squares of the first three lanes, one refined
/// `RSQRTSS` and a multiply of all four lanes.
pub(crate) fn normalised(e: &Engine, v: Vec4) -> Vec4 {
    let squares = zip_with(v, v, |a, b| a * b);
    scaled(v, inverse_length(e, sum3(squares)))
}

/// The vector the code builds beside an axis: the absolute values of `x`, `y` and `z` go
/// through the x87 stack (a signalling NaN is quieted), `index_a` and `index_b` start as the
/// larger and smaller of `x` and `y` (`x > y` strictly), `index_c` is 2 unless that smaller
/// one is greater than `z` (then it is `index_b`), and the result is zero but for
/// `v[index_a] = axis[index_c]` and `v[index_c] = -axis[index_a]`.
pub(crate) fn beside(axis: Vec4) -> Vec4 {
    let [x, y, z] = [0, 1, 2].map(|lane| x87_quiet(axis[lane]).abs());
    let (index_a, index_b, smaller) = if x > y { (0, 1, y) } else { (1, 0, x) };
    let index_c = if smaller > z { index_b } else { 2 };
    let mut v = [0.0f32; 4];
    v[index_a] = x87_quiet(axis[index_c]);
    v[index_c] = -x87_quiet(axis[index_a]);
    v
}

// Translated from 00cbe190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpHingeConstraintData::setMaxLinearImpulse` (Xbox PDB): the float at `+0xA8`.
pub fn hkp_hinge_constraint_data_set_max_linear_impulse(e: &mut Engine, this: Ptr, impulse: f32) {
    e.mem.set_f32(this.addr() + 0xa8, x87_quiet(impulse));
}

// Translated from 00cbe1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpHingeConstraintData::getMaxLinearImpulse` (Xbox PDB).
pub fn hkp_hinge_constraint_data_get_max_linear_impulse(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr() + 0xa8))
}

// Translated from 00cbe1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpHingeConstraintData::setBodyToNotify` (Xbox PDB): the mask byte at `+0xA6` is `1 << index`
/// (the shift count is the low five bits and the shift is 8 bits wide). Answers `this`
/// (`EAX`, left over from the code).
pub fn hkp_hinge_constraint_data_set_body_to_notify(e: &mut Engine, this: Ptr, index: u32) -> Ptr {
    let shift = index & 0x1f;
    let mask = if shift < 8 { 1u8 << shift } else { 0 };
    e.mem.set_u8(this.addr() + 0xa6, mask);
    this
}

// Translated from 00cbe1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpHingeConstraintData::getNotifiedBodyIndex` (Xbox PDB): the mask byte at `+0xA6` shifted
/// right once.
pub fn hkp_hinge_constraint_data_get_notified_body_index(e: &mut Engine, this: Ptr) -> u32 {
    u32::from(e.mem.u8(this.addr() + 0xa6)) >> 1
}

// Translated from 00cbe1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpHingeConstraintData::getRuntimeInfo` (Xbox PDB): only the low byte of the first word is
/// tested; set it answers `{0x28, 5}` (`+0` the size in bytes, `+4` the number of solver
/// results), clear `{0, 0}`.
pub fn hkp_hinge_constraint_data_get_runtime_info(
    e: &mut Engine,
    _this: Ptr,
    want_runtime: u8,
    info: Ptr,
) {
    let (size, results) = if want_runtime != 0 { (0x28, 5) } else { (0, 0) };
    e.mem.set_u32(info.addr() + 4, results);
    e.mem.set_u32(info.addr(), size);
}

// Translated from 00cbe210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpHingeConstraintData::getConstraintInfo` (Xbox PDB): `getConstraintInfoUtil` over the
/// atoms at `+0x10`, `0x9C` bytes of them.
pub fn hkp_hinge_constraint_data_get_constraint_info(e: &mut Engine, this: Ptr, info: Ptr) {
    e.call(
        CONSTRAINT_INFO_UTIL,
        &args![this.addr() + 0x10, 0x9cu32, info],
    );
}

// Translated from 00cbe230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpHingeConstraintData::getType` (Xbox PDB): 1.
pub fn hkp_hinge_constraint_data_get_type(_e: &mut Engine, _this: Ptr) -> u32 {
    1
}

// Translated from 00cbe240 (decompiled, FalloutNV.exe 1.4.0.525)
/// The unnamed function of the hinge data between `getType` and `setInBodySpace` (the shape
/// suggests a world-space setter; unconfirmed). `transform_a` and `transform_b` are the bodies
/// transforms, `pivot` a point and `axis` a direction (an aligned `hkVector4`, not normalised).
/// The second row is the normalised vector built beside the axis ([`beside`]) and the third
/// the cross product of the axis and the second. Each row is rotated by each transform
/// (`004b4ab0`) into the rows at `+0x20/+0x30/+0x40` (body A) and `+0x60/+0x70/+0x80` (body
/// B), and `setTransformedInversePos` puts the pivot at `+0x50` and `+0x90`.
pub fn fn_00cbe240(
    e: &mut Engine,
    this: Ptr,
    transform_a: Ptr,
    transform_b: Ptr,
    pivot: Ptr,
    axis: Ptr,
) {
    let me = this.addr();
    let axis_vec = load_vec(e, axis.addr());
    let second = normalised(e, beside(axis_vec));
    let third = cross(axis_vec, second);
    // The two vectors live in the frame (`esp + 0x20` and `esp + 0x30`).
    e.with_stack(0x20, |e, frame| {
        let second_at = frame.addr();
        let third_at = frame.addr() + 0x10;
        store_vec(e, second_at, second);
        store_vec(e, third_at, third);
        rotate_rows(
            e,
            me,
            [transform_a, transform_b],
            pivot,
            [axis.addr(), second_at, third_at],
        );
    });
}

/// The calls that put each body's three rows and pivot in place: every row goes through the
/// rotation thunk `004b4ab0` into `+0x20/+0x30/+0x40` (body A) and `+0x60/+0x70/+0x80`
/// (body B), then `setTransformedInversePos` puts the pivot at `+0x50` and `+0x90`.
pub(crate) fn rotate_rows(
    e: &mut Engine,
    this: u32,
    transforms: [Ptr; 2],
    pivot: Ptr,
    rows: [u32; 3],
) {
    for (offset, transform) in [(0x20, transforms[0]), (0x60, transforms[1])] {
        for (n, row) in rows.into_iter().enumerate() {
            let target = this + offset + 0x10 * n as u32;
            e.call(ROTATE_VECTOR, &args![target, transform, row]);
        }
        e.call(
            SET_TRANSFORMED_INVERSE_POS,
            &args![this + offset + 0x30, transform, pivot],
        );
    }
}

// Translated from 00cbe400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpHingeConstraintData::setInBodySpace` (Xbox PDB): the pivots are copied to `+0x50`
/// and `+0x90`; each axis is normalised into its first row (`+0x20`, `+0x60`), a vector built
/// beside it ([`beside`], on the stored axis) is normalised into the second row (`+0x30`,
/// `+0x70`) and the cross product of the two rows is the third (`+0x40`, `+0x80`).
pub fn hkp_hinge_constraint_data_set_in_body_space(
    e: &mut Engine,
    this: Ptr,
    pivot_a: Ptr,
    pivot_b: Ptr,
    axis_a: Ptr,
    axis_b: Ptr,
) {
    let me = this.addr();
    let pivot = load_vec(e, pivot_a.addr());
    store_vec(e, me + 0x50, pivot);
    let pivot = load_vec(e, pivot_b.addr());
    store_vec(e, me + 0x90, pivot);
    for (row, axis) in [(0x20, axis_a), (0x60, axis_b)] {
        let first = normalised(e, load_vec(e, axis.addr()));
        store_vec(e, me + row, first);
        let second = normalised(e, beside(first));
        store_vec(e, me + row + 0x10, second);
        store_vec(e, me + row + 0x20, cross(first, second));
    }
}

// Translated from 00cbe750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpHingeConstraintData::isValid` (Xbox PDB): `isOrthonormal` (tolerance from `01017d00`)
/// of the rows at `+0x20` and at `+0x60`; the byte at `result` is 1 when both answer true and
/// 0 as soon as one answers false. Answers `result` (`EAX`).
pub fn hkp_hinge_constraint_data_is_valid(e: &mut Engine, this: Ptr, result: Ptr) -> Ptr {
    let me = this.addr();
    let tolerance = x87_quiet_bits(e.global::<u32>(ORTHONORMAL_TOLERANCE));
    e.with_stack(4, |e, flags| {
        let first = e
            .call(
                IS_ORTHONORMAL,
                &args![me + 0x20, flags.addr() + 2, tolerance],
            )
            .u32();
        if e.mem.u8(first) != 0 {
            let second = e
                .call(
                    IS_ORTHONORMAL,
                    &args![me + 0x60, flags.addr() + 3, tolerance],
                )
                .u32();
            if e.mem.u8(second) != 0 {
                e.mem.set_u8(result.addr(), 1);
                return result;
            }
        }
        e.mem.set_u8(result.addr(), 0);
        result
    })
}

// Translated from 00cbe7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpHingeConstraintData::hkpHingeConstraintData` (Xbox PDB): the reference-count word
/// (`+6`) is 1, `+8` is 0, the vtable is set, the atoms are built (`+0x10`) and both transforms'
/// rotations are the identity with zero pivots; the byte at `+0xA2` is cleared. Answers `this`.
pub fn hkp_hinge_constraint_data_construct(e: &mut Engine, this: Ptr) -> Ptr {
    let me = this.addr();
    e.mem.set_u16(me + 6, 1);
    e.mem.set_u32(me + 8, 0);
    e.mem.set_u32(me, VTABLE);
    hkp_hinge_constraint_data_atoms_construct(e, Ptr::new(me + 0x10));
    for row in [0x20, 0x60] {
        for offset in [0, 0x10, 0x20, 0x30] {
            store_vec(e, me + row + offset, [0.0; 4]);
        }
        for diagonal in [0, 0x14, 0x28] {
            e.mem.set_f32(me + row + diagonal, 1.0);
        }
    }
    e.mem.set_u8(me + 0xa2, 0);
    this
}

// Translated from 00cbe820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpHingeConstraintData::Atoms::Atoms` (Xbox PDB): the setup atom's type is 2; the
/// ball-socket atom at `+0x90` has type 12, `+0x94` is 5, the byte at `+0x96` is 0, the
/// `hkUFloat8` at `+0x97` is set to 1.0 and the float at `+0x98` to the large float
/// (`0104da48`). Answers `this`.
pub fn hkp_hinge_constraint_data_atoms_construct(e: &mut Engine, this: Ptr) -> Ptr {
    let me = this.addr();
    e.mem.set_u16(me, 2);
    e.mem.set_u16(me + 0x90, 0xc);
    e.mem.set_u16(me + 0x94, 5);
    e.mem.set_u8(me + 0x96, 0);
    e.with_stack(4, |e, one| {
        e.mem.set_f32(one.addr(), 1.0);
        e.call(U_FLOAT8_ASSIGN, &args![me + 0x97, one]);
    });
    let large = x87_quiet_bits(e.global::<u32>(LARGE_FLOAT));
    e.mem.set_u32(me + 0x98, large);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00cb_e190,
            hkp_hinge_constraint_data_set_max_linear_impulse(Ptr, f32)
        ),
        entry!(
            0x00cb_e1a0,
            hkp_hinge_constraint_data_get_max_linear_impulse(Ptr) -> f32
        ),
        entry!(
            0x00cb_e1b0,
            hkp_hinge_constraint_data_set_body_to_notify(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00cb_e1d0,
            hkp_hinge_constraint_data_get_notified_body_index(Ptr) -> u32
        ),
        entry!(
            0x00cb_e1e0,
            hkp_hinge_constraint_data_get_runtime_info(Ptr, u8, Ptr)
        ),
        entry!(
            0x00cb_e210,
            hkp_hinge_constraint_data_get_constraint_info(Ptr, Ptr)
        ),
        entry!(0x00cb_e230, hkp_hinge_constraint_data_get_type(Ptr) -> u32),
        entry!(0x00cb_e240, fn_00cbe240(Ptr, Ptr, Ptr, Ptr, Ptr)),
        entry!(
            0x00cb_e400,
            hkp_hinge_constraint_data_set_in_body_space(Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00cb_e750,
            hkp_hinge_constraint_data_is_valid(Ptr, Ptr) -> Ptr
        ),
        entry!(0x00cb_e7b0, hkp_hinge_constraint_data_construct(Ptr) -> Ptr),
        entry!(
            0x00cb_e820,
            hkp_hinge_constraint_data_atoms_construct(Ptr) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, new_log, Log,
    };
    use crate::units::havok_sdk::hkpsphereshape::{HALF_F32, THREE_F32, ZERO_F32};
    use std::rc::Rc;

    const TOLERANCE_BITS: u32 = 0x3a83_126f;

    /// The constants the normaliser reads from the exe data.
    fn set_constants(e: &mut Engine) {
        e.set_global(ZERO_F32, 0.0f32);
        e.set_global(THREE_F32, 3.0f32);
        e.set_global(HALF_F32, 0.5f32);
    }

    fn assert_close(actual: Vec4, expected: Vec4) {
        for lane in 0..4 {
            assert!(
                (actual[lane] - expected[lane]).abs() < 1e-5,
                "lane {lane}: {actual:?} against {expected:?}"
            );
        }
    }

    /// A double at `address` that logs its arguments and the four words of the vector at
    /// argument 2 as the call sees them.
    fn vector_double(e: &mut Engine, log: &Log, address: u32) {
        let log = Rc::clone(log);
        e.register_double(address, move |e, a| {
            let mut words = a.to_vec();
            words.extend((0..4).map(|lane| e.mem.u32(a[2] + 4 * lane)));
            log.borrow_mut().push((address, words));
            Ret::default()
        });
    }

    fn vector_words(words: &[u32]) -> Vec4 {
        std::array::from_fn(|lane| f32::from_bits(words[3 + lane]))
    }

    fn dot(a: Vec4, b: Vec4) -> f32 {
        a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
    }

    #[test]
    fn max_linear_impulse_is_stored_and_read_through_the_x87_stack_00cbe190() {
        let mut e = engine();
        let this = e.mem.alloc(0x100);
        e.call(0x00cb_e190, &args![this, 2.5f32.to_bits()]);
        assert_eq!(e.mem.f32(this + 0xa8), 2.5);
        assert_eq!(e.call(0x00cb_e1a0, &args![this]).f32(), 2.5);
        e.call(0x00cb_e190, &args![this, 0x7f80_0001u32]);
        assert_eq!(e.mem.u32(this + 0xa8), 0x7fc0_0001);
    }

    #[test]
    fn body_to_notify_sets_the_mask_and_the_index_reads_it_back_00cbe1b0() {
        let mut e = engine();
        let this = e.mem.alloc(0x100);
        for (index, mask) in [(0u32, 1u8), (1, 2), (7, 0x80), (8, 0), (33, 2)] {
            assert_eq!(e.call(0x00cb_e1b0, &args![this, index]).u32(), this);
            assert_eq!(e.mem.u8(this + 0xa6), mask, "index {index}");
        }
        e.mem.set_u8(this + 0xa6, 0x80);
        assert_eq!(e.call(0x00cb_e1d0, &args![this]).u32(), 0x40);
        e.mem.set_u8(this + 0xa6, 1);
        assert_eq!(e.call(0x00cb_e1d0, &args![this]).u32(), 0);
    }

    #[test]
    fn runtime_info_depends_on_the_flag_byte_00cbe1e0() {
        let mut e = engine();
        let this = e.mem.alloc(0x100);
        let info = e.mem.alloc(8);
        e.mem.write(info, &[0xff; 8]);
        e.call(0x00cb_e1e0, &args![this, 1u32, info]);
        assert_eq!((e.mem.u32(info), e.mem.u32(info + 4)), (0x28, 5));
        e.mem.write(info, &[0xff; 8]);
        e.call(0x00cb_e1e0, &args![this, 0u32, info]);
        assert_eq!((e.mem.u32(info), e.mem.u32(info + 4)), (0, 0));
    }

    #[test]
    fn constraint_info_goes_through_the_util_over_the_atoms_00cbe210() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, CONSTRAINT_INFO_UTIL, 0);
        let this = e.mem.alloc(0x100);
        let info = e.mem.alloc(0x20);
        e.call(0x00cb_e210, &args![this, info]);
        assert_eq!(
            calls(&log, CONSTRAINT_INFO_UTIL),
            vec![vec![this + 0x10, 0x9c, info]]
        );
    }

    #[test]
    fn the_type_is_one_00cbe230() {
        let mut e = engine();
        assert_eq!(e.call(0x00cb_e230, &args![0u32]).u32(), 1);
    }

    #[test]
    fn beside_follows_the_three_branches_and_nan() {
        // x <= y: the pair starts as (1, 0); z does not beat x.
        assert_eq!(beside([1.0, 2.0, 3.0, 9.0]), [0.0, 3.0, -2.0, 0.0]);
        // x > y and the smaller one (y) beats z: the third index is the smaller one.
        assert_eq!(beside([3.0, 2.0, 1.0, 9.0]), [2.0, -3.0, 0.0, 0.0]);
        // x > y and z is not below y.
        assert_eq!(beside([3.0, 1.0, 2.0, 9.0]), [2.0, 0.0, -3.0, 0.0]);
        // NaN never compares greater: the pair is (1, 0) and the third index is 2.
        let v = beside([f32::NAN, 1.0, 2.0, 0.0]);
        assert_eq!((v[0], v[1], v[2]), (0.0, 2.0, -1.0));
    }

    #[test]
    fn the_unnamed_setter_rotates_the_axis_and_the_two_built_rows_00cbe240() {
        let mut e = engine();
        set_constants(&mut e);
        let log = new_log();
        vector_double(&mut e, &log, ROTATE_VECTOR);
        vector_double(&mut e, &log, SET_TRANSFORMED_INVERSE_POS);
        let this = e.mem.alloc(0x100);
        let axis = e.mem.alloc(0x10);
        store_vec(&mut e, axis, [0.0, 0.0, 2.0, 0.0]);
        let pivot = e.mem.alloc(0x10);
        let (transform_a, transform_b) = (0x1000u32, 0x2000u32);
        e.call(
            0x00cb_e240,
            &args![this, transform_a, transform_b, pivot, axis],
        );
        let rotations = calls(&log, ROTATE_VECTOR);
        assert_eq!(rotations.len(), 6);
        let outputs: Vec<u32> = rotations.iter().map(|c| c[0]).collect();
        assert_eq!(
            outputs,
            [0x20, 0x30, 0x40, 0x60, 0x70, 0x80].map(|o| this + o)
        );
        let transforms: Vec<u32> = rotations.iter().map(|c| c[1]).collect();
        assert_eq!(
            transforms,
            [
                transform_a,
                transform_a,
                transform_a,
                transform_b,
                transform_b,
                transform_b
            ]
        );
        assert_eq!(rotations[0][2], axis);
        for half in [0, 3] {
            assert_eq!(vector_words(&rotations[half]), [0.0, 0.0, 2.0, 0.0]);
            assert_close(vector_words(&rotations[half + 1]), [0.0, 1.0, 0.0, 0.0]);
            assert_close(vector_words(&rotations[half + 2]), [-2.0, 0.0, 0.0, 0.0]);
        }
        let pivots = calls(&log, SET_TRANSFORMED_INVERSE_POS);
        let heads: Vec<[u32; 3]> = pivots.iter().map(|c| [c[0], c[1], c[2]]).collect();
        assert_eq!(
            heads,
            [
                [this + 0x50, transform_a, pivot],
                [this + 0x90, transform_b, pivot]
            ]
        );
    }

    #[test]
    fn the_unnamed_setter_keeps_the_rows_orthonormal_for_a_slanted_axis_00cbe240() {
        let mut e = engine();
        set_constants(&mut e);
        let log = new_log();
        vector_double(&mut e, &log, ROTATE_VECTOR);
        vector_double(&mut e, &log, SET_TRANSFORMED_INVERSE_POS);
        let this = e.mem.alloc(0x100);
        let axis = e.mem.alloc(0x10);
        store_vec(&mut e, axis, [3.0, 2.0, 1.0, 0.0]);
        let pivot = e.mem.alloc(0x10);
        e.call(0x00cb_e240, &args![this, 0x1000u32, 0x2000u32, pivot, axis]);
        let rotations = calls(&log, ROTATE_VECTOR);
        let first = vector_words(&rotations[0]);
        let second = vector_words(&rotations[1]);
        let third = vector_words(&rotations[2]);
        assert_eq!(first, [3.0, 2.0, 1.0, 0.0]);
        let root = 13f32.sqrt();
        assert_close(second, [2.0 / root, -3.0 / root, 0.0, 0.0]);
        assert_close(third, cross(first, second));
        assert!(dot(first, second).abs() < 1e-5);
        assert!(dot(second, third).abs() < 1e-5);
    }

    #[test]
    fn body_space_setup_normalises_the_axes_and_copies_the_pivots_00cbe400() {
        let mut e = engine();
        set_constants(&mut e);
        let this = e.mem.alloc(0x100);
        let [pivot_a, pivot_b, axis_a, axis_b] = [0; 4].map(|_| e.mem.alloc(0x10));
        store_vec(&mut e, pivot_a, [1.0, 2.0, 3.0, 4.0]);
        store_vec(&mut e, pivot_b, [5.0, 6.0, 7.0, 8.0]);
        store_vec(&mut e, axis_a, [0.0, 0.0, 2.0, 0.5]);
        store_vec(&mut e, axis_b, [3.0, 2.0, 1.0, 0.0]);
        e.call(0x00cb_e400, &args![this, pivot_a, pivot_b, axis_a, axis_b]);
        assert_eq!(load_vec(&e, this + 0x50), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(load_vec(&e, this + 0x90), [5.0, 6.0, 7.0, 8.0]);
        assert_close(load_vec(&e, this + 0x20), [0.0, 0.0, 1.0, 0.25]);
        assert_close(load_vec(&e, this + 0x30), [0.0, 1.0, 0.0, 0.0]);
        assert_close(load_vec(&e, this + 0x40), [-1.0, 0.0, 0.0, 0.0]);
        let root = 14f32.sqrt();
        let first = load_vec(&e, this + 0x60);
        assert_close(first, [3.0 / root, 2.0 / root, 1.0 / root, 0.0]);
        let second = load_vec(&e, this + 0x70);
        assert!((dot(second, second) - 1.0).abs() < 1e-5);
        assert!(dot(first, second).abs() < 1e-5);
        assert_close(load_vec(&e, this + 0x80), cross(first, second));
    }

    /// An `isOrthonormal` double that answers the bytes of `answers` in turn.
    fn orthonormal_double(e: &mut Engine, log: &Log, answers: Vec<u8>) {
        let log = Rc::clone(log);
        let mut answers = answers.into_iter();
        e.register_double(IS_ORTHONORMAL, move |e, a| {
            log.borrow_mut().push((IS_ORTHONORMAL, a.to_vec()));
            e.mem.set_u8(a[1], answers.next().unwrap());
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
    }

    #[test]
    fn valid_needs_both_rows_orthonormal_00cbe750() {
        for (answers, expected, asked) in [
            (vec![1u8, 1], 1u8, 2usize),
            (vec![1, 0], 0, 2),
            (vec![0, 1], 0, 1),
        ] {
            let mut e = engine();
            e.set_global(ORTHONORMAL_TOLERANCE, TOLERANCE_BITS);
            let log = new_log();
            orthonormal_double(&mut e, &log, answers);
            let this = e.mem.alloc(0x100);
            let result = e.mem.alloc(4);
            e.mem.set_u8(result, 0x55);
            assert_eq!(e.call(0x00cb_e750, &args![this, result]).u32(), result);
            assert_eq!(e.mem.u8(result), expected);
            let seen = calls(&log, IS_ORTHONORMAL);
            assert_eq!(seen.len(), asked);
            assert_eq!(seen[0][0], this + 0x20);
            assert_eq!(seen[0][2], TOLERANCE_BITS);
            if asked == 2 {
                assert_eq!(seen[1][0], this + 0x60);
                assert_eq!(seen[1][2], TOLERANCE_BITS);
                assert_eq!(seen[1][1], seen[0][1] + 1);
            }
        }
    }

    #[test]
    fn valid_quiets_a_signalling_tolerance_00cbe750() {
        let mut e = engine();
        e.set_global(ORTHONORMAL_TOLERANCE, 0x7f80_0001u32);
        let log = new_log();
        orthonormal_double(&mut e, &log, vec![0]);
        let this = e.mem.alloc(0x100);
        let result = e.mem.alloc(4);
        e.call(0x00cb_e750, &args![this, result]);
        assert_eq!(calls(&log, IS_ORTHONORMAL)[0][2], 0x7fc0_0001);
    }

    #[test]
    fn atoms_are_filled_in_with_a_unit_ufloat8_and_the_large_float_00cbe820() {
        let mut e = engine();
        e.set_global(LARGE_FLOAT, 0x7f7f_ffeeu32);
        let log = new_log();
        let sink = Rc::clone(&log);
        e.register_double(U_FLOAT8_ASSIGN, move |e, a| {
            sink.borrow_mut()
                .push((U_FLOAT8_ASSIGN, vec![a[0], e.mem.u32(a[1])]));
            Ret::default()
        });
        let this = e.mem.alloc(0xc0);
        e.mem.write(this, &[0xcd; 0xc0]);
        assert_eq!(e.call(0x00cb_e820, &args![this]).u32(), this);
        assert_eq!(
            calls(&log, U_FLOAT8_ASSIGN),
            vec![vec![this + 0x97, 1.0f32.to_bits()]]
        );
        assert_eq!(e.mem.u16(this), 2);
        assert_eq!(e.mem.u16(this + 0x90), 0xc);
        assert_eq!(e.mem.u16(this + 0x94), 5);
        assert_eq!(e.mem.u8(this + 0x96), 0);
        assert_eq!(e.mem.u32(this + 0x98), 0x7f7f_ffee);
        assert_eq!(e.mem.u16(this + 2), 0xcdcd);
        assert_eq!(e.mem.u16(this + 0x92), 0xcdcd);
        assert_eq!(e.mem.u8(this + 0x9c), 0xcd);
    }

    #[test]
    fn construct_builds_identity_rows_and_zero_pivots_00cbe7b0() {
        let mut e = engine();
        e.set_global(LARGE_FLOAT, 0x7f7f_ffeeu32);
        e.register_double(U_FLOAT8_ASSIGN, |_, _| Ret::default());
        let this = e.mem.alloc(0x100);
        e.mem.write(this, &[0xcd; 0x100]);
        assert_eq!(e.call(0x00cb_e7b0, &args![this]).u32(), this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!(e.mem.u16(this + 0x10), 2);
        assert_eq!(e.mem.u8(this + 0xa2), 0);
        assert_eq!(e.mem.u8(this + 0xa3), 0xcd);
        for row in [0x20, 0x60] {
            assert_eq!(load_vec(&e, this + row), [1.0, 0.0, 0.0, 0.0]);
            assert_eq!(load_vec(&e, this + row + 0x10), [0.0, 1.0, 0.0, 0.0]);
            assert_eq!(load_vec(&e, this + row + 0x20), [0.0, 0.0, 1.0, 0.0]);
            assert_eq!(load_vec(&e, this + row + 0x30), [0.0; 4]);
        }
        assert_eq!(e.mem.u8(this + 0xe), 0xcd);
    }
}
