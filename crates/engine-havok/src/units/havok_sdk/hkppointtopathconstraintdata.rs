//! `Havok SDK/hkppointtopathconstraintdata.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00deef10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00de_e7d0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00deef10(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00de_e7d0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

use super::hkpsphereshape::{
    inverse_length, load_bits, load_vec, shufps, store_vec, sum3, zip_with, Vec4, SIGN_MASK,
};

// `hkpPointToPathConstraintData` (the vtable `010da004` is that class by its RTTI): vtable at
// +0, reference count word at +6, a count at +8 (0), the constraint atoms from +0x0C (a bridge
// atom, initialised by `00d21ce0`), the path (`hkpParametricCurve`, reference counted) at +0x18,
// a float at +0x1C (the maximum friction force), a byte at +0x20 (the orientation type), the
// transform of body A's pivot at +0x30 (three rotation vectors and a translation) and the
// transform for the path at +0x70; 0xB0 bytes in all.
const POINT_TO_PATH_VTABLE: u32 = 0x010d_a004;
const BASE_VTABLE: u32 = 0x0102_e388;
const PATH: u32 = 0x18;
const MAX_FRICTION: u32 = 0x1c;
const ORIENTATION_TYPE: u32 = 0x20;
const PIVOT_TRANSFORM: u32 = 0x30;
const PATH_TRANSFORM: u32 = 0x70;

/// `hkReferencedObject` add and remove reference (`this` only).
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `hkpBridgeConstraintAtom::init` (Xbox PDB): `this`, the constraint data.
const BRIDGE_ATOM_INIT: u32 = 0x00d2_1ce0;
/// `hkVector4::setTransformedPos` (Xbox PDB, `this`, transform, position),
/// `hkVector4::setTransformedInversePos` (`this`, transform, position) and
/// `hkVector4::setRotatedDir` (`this`, transform, direction).
const VECTOR_SET_TRANSFORMED_POS: u32 = 0x00c7_f640;
const VECTOR_SET_TRANSFORMED_INVERSE_POS: u32 = 0x00c7_f690;
const VECTOR_SET_ROTATED_DIR: u32 = 0x00c7_f700;
/// `hkTransform::setMul` (Xbox PDB, `this`, a, b: `this = a * b`) and
/// `hkTransform::setMulInverseMul` (`this`, a, b).
const TRANSFORM_SET_MUL: u32 = 0x00cb_2240;
const TRANSFORM_SET_MUL_INVERSE_MUL: u32 = 0x00cb_22f0;
/// The jacobian schema header record (`00d6f2b0`, cdecl: the query input, the output, a word
/// and a byte).
const HEADER_RECORD: u32 = 0x00d6_f2b0;
/// `hk1dLinearFrictionBuildJacobian`, `hk1dLinearBilateralConstraintBuildJacobian`,
/// `hk1dAngularBilateralConstraintBuildJacobian` and `hk1dLinearLimitBuildJacobian` (Xbox PDB;
/// each cdecl: a block of vectors on the stack, the query input and the output).
const LINEAR_FRICTION_JACOBIAN: u32 = 0x00dc_f100;
const LINEAR_BILATERAL_JACOBIAN: u32 = 0x00dc_f280;
const ANGULAR_BILATERAL_JACOBIAN: u32 = 0x00dc_ee80;
const LINEAR_LIMIT_JACOBIAN: u32 = 0x00dc_fa30;

/// `(a.yzx * b.zxy) - (a.zxy * b.yzx)`: the cross product `a x b` the way the code forms it
/// (`SHUFPS` 0xC9 and 0xD2 with the multiplies in this order).
fn cross_yzx(a: Vec4, b: Vec4) -> Vec4 {
    let plus = zip_with(shufps(a, a, 0xc9), shufps(b, b, 0xd2), |x, y| x * y);
    let minus = zip_with(shufps(a, a, 0xd2), shufps(b, b, 0xc9), |x, y| x * y);
    zip_with(plus, minus, |x, y| x - y)
}

/// Copies 16 bytes (a `MOVAPS` load and store).
fn copy_vector(e: &mut Engine, from: u32, to: u32) {
    let v = load_bits(e, from);
    for (lane, word) in v.into_iter().enumerate() {
        e.mem.set_u32(to + 4 * lane as u32, word);
    }
}

// Translated from 00dee720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPointToPathConstraintData::getRuntimeInfo` (Xbox PDB): neither `this` nor the first
/// word is read. Sets the info's runtime size (+0) to 0x3C and its solver result count (+4) to
/// 7.
pub fn point_to_path_get_runtime_info(e: &mut Engine, _this: Ptr, _unused_1: u32, info: u32) {
    e.mem.set_u32(info + 4, 7);
    e.mem.set_u32(info, 0x3c);
}

// Translated from 00dee740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPointToPathConstraintData::getType` (Xbox PDB): 3.
pub fn point_to_path_get_type(_e: &mut Engine, _this: Ptr) -> u32 {
    3
}

// Translated from 00dee750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPointToPathConstraintData::hkpPointToPathConstraintData` (Xbox PDB): installs the
/// vtable, reference count 1, a zero count at +8, the word 1 at +0xC (the atoms' first
/// field), no path, a maximum friction of 0, orientation type 1, identity rotations with zero
/// translations at +0x30 and +0x70, then initialises the bridge atom at +0xC with `this`
/// (`hkpBridgeConstraintAtom::init`). Returns `this`.
pub fn point_to_path_construct(e: &mut Engine, this: Ptr) -> Ptr {
    let data = this.addr();
    e.mem.set_u32(data, POINT_TO_PATH_VTABLE);
    e.mem.set_u16(data + 6, 1);
    e.mem.set_u32(data + 8, 0);
    e.mem.set_u16(data + 0xc, 1);
    e.mem.set_f32(data + MAX_FRICTION, 0.0);
    e.mem.set_u32(data + PATH, 0);
    e.mem.set_u8(data + ORIENTATION_TYPE, 1);
    for transform in [PIVOT_TRANSFORM, PATH_TRANSFORM] {
        for row in 0..4 {
            store_vec(e, data + transform + 0x10 * row, [0.0; 4]);
        }
        for row in 0..3 {
            e.mem.set_f32(data + transform + 0x10 * row + 4 * row, 1.0);
        }
    }
    e.call(BRIDGE_ATOM_INIT, &args![data + 0xc, data]);
    this
}

// Translated from 00dee7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPointToPathConstraintData::~hkpPointToPathConstraintData` (the base destructor of the
/// deleting destructor at `00deef10`): installs the vtable, removes the reference to the path
/// when there is one, installs the base vtable.
pub fn point_to_path_destruct(e: &mut Engine, this: Ptr) {
    let data = this.addr();
    let path = e.mem.u32(data + PATH);
    e.mem.set_u32(data, POINT_TO_PATH_VTABLE);
    if path != 0 {
        e.call(REMOVE_REFERENCE, &args![path]);
    }
    e.mem.set_u32(data, BASE_VTABLE);
}

// Translated from 00dee7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPointToPathConstraintData::getConstraintInfo` (Xbox PDB): the info gets 0 at +0, 0x180
/// at +4, 7 at +8, 8 at +0xC, the atoms (`this + 0xC`) at +0x10 and their size 0xC at +0x14.
pub fn point_to_path_get_constraint_info(e: &mut Engine, this: Ptr, info: u32) {
    e.mem.set_u32(info + 0x10, this.addr() + 0xc);
    e.mem.set_u32(info + 0x14, 0xc);
    e.mem.set_u32(info, 0);
    e.mem.set_u32(info + 4, 0x180);
    e.mem.set_u32(info + 8, 7);
    e.mem.set_u32(info + 0xc, 8);
}

// Translated from 00dee820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPointToPathConstraintData::calcPivot` (Xbox PDB): `hkVector4::setTransformedPos` with
/// `this` = the second argument (the output vector), the first argument (a transform) and the
/// vector at `this + 0x60`.
pub fn point_to_path_calc_pivot(e: &mut Engine, this: Ptr, transform: u32, output: u32) {
    e.call(
        VECTOR_SET_TRANSFORMED_POS,
        &args![output, transform, this.addr() + 0x60],
    );
}

// Translated from 00dee840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPointToPathConstraintData::isValid` (Xbox PDB): the byte at `valid` becomes 1 when there
/// is a path (+0x18) and the orientation type (+0x20) is non-zero, else 0. Returns `valid`.
pub fn point_to_path_is_valid(e: &mut Engine, this: Ptr, valid: u32) -> u32 {
    let data = this.addr();
    let ok = e.mem.u32(data + PATH) != 0 && e.mem.u8(data + ORIENTATION_TYPE) != 0;
    e.mem.set_u8(valid, u8::from(ok));
    valid
}

// Translated from 00dee870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPointToPathConstraintData::setInWorldSpace` (Xbox PDB): `transform_a` and `transform_b`
/// are the two bodies' transforms, `pivot` the pivot in world space, `path` the curve (a
/// reference is added to it and removed from the previous one) and `path_transform` the
/// curve's transform (copied to +0x70). With `T1 = b * path_transform` the pivot is mapped
/// into the curve's frame, the curve's nearest point (virtual +0x10, parameter 0.0), tangent
/// (+0x14) and binormal (+0x24) there form a rotation (tangent, binormal, tangent x
/// binormal) with the nearest point as translation, which is carried back to world space and
/// to body A's frame (`inverse(a) * (b * (path_transform * frame))`) into +0x30.
pub fn point_to_path_set_in_world_space(
    e: &mut Engine,
    this: Ptr,
    transform_a: u32,
    transform_b: u32,
    pivot: u32,
    path: u32,
    path_transform: u32,
) {
    let data = this.addr();
    if path != 0 {
        e.call(ADD_REFERENCE, &args![path]);
    }
    let old = e.mem.u32(data + PATH);
    if old != 0 {
        e.call(REMOVE_REFERENCE, &args![old]);
    }
    e.mem.set_u32(data + PATH, path);
    for row in 0..4 {
        copy_vector(
            e,
            path_transform + 0x10 * row,
            data + PATH_TRANSFORM + 0x10 * row,
        );
    }
    // The frame of the original: `m` + 0x10 the nearest point, +0xC its parameter, +0x20 the
    // binormal, +0x30 the tangent, +0x40 the result transform, +0x80 the frame (tangent,
    // binormal, normal, point), +0xC0 and +0x100 temporary transforms.
    e.with_stack(0x140, |e, frame| {
        let m = frame.addr();
        let (point, binormal, tangent, result, matrix) =
            (m + 0x10, m + 0x20, m + 0x30, m + 0x40, m + 0x80);
        let (first, second) = (m + 0xc0, m + 0x100);
        e.call(
            TRANSFORM_SET_MUL,
            &args![first, transform_b, path_transform],
        );
        e.call(
            VECTOR_SET_TRANSFORMED_INVERSE_POS,
            &args![point, first, pivot],
        );
        let curve = e.mem.u32(data + PATH);
        let t = e.vcall(curve, 0x10, &args![0.0f32, point, point]).f32();
        e.mem.set_f32(m + 0xc, t);
        let curve = e.mem.u32(data + PATH);
        let t = x87_quiet(e.mem.f32(m + 0xc));
        e.vcall(curve, 0x14, &args![t, tangent]);
        let curve = e.mem.u32(data + PATH);
        let t = x87_quiet(e.mem.f32(m + 0xc));
        e.vcall(curve, 0x24, &args![t, binormal]);
        let tangent_vector = load_vec(e, tangent);
        let binormal_vector = load_vec(e, binormal);
        let normal = cross_yzx(tangent_vector, binormal_vector);
        copy_vector(e, binormal, matrix + 0x10);
        let point_vector = load_vec(e, point);
        store_vec(e, matrix, tangent_vector);
        store_vec(e, matrix + 0x20, normal);
        store_vec(e, matrix + 0x30, point_vector);
        e.call(TRANSFORM_SET_MUL, &args![second, path_transform, matrix]);
        e.call(TRANSFORM_SET_MUL, &args![first, transform_b, second]);
        e.call(
            TRANSFORM_SET_MUL_INVERSE_MUL,
            &args![result, transform_a, first],
        );
        for row in 0..4 {
            copy_vector(e, result + 0x10 * row, data + PIVOT_TRANSFORM + 0x10 * row);
        }
    });
}

// Translated from 00deea00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the path and the two pivots (no engine-map name; the shape of `setInBodySpace`): adds a
/// reference to a non-null `path` and removes the previous one, stores it, resets the
/// transforms at +0x30 and +0x70 to identity rotations and zero translations, then sets the
/// translations at +0x60 and +0xA0 from the vectors `pivot_a` and `pivot_b`.
pub fn fn_00deea00(e: &mut Engine, this: Ptr, pivot_a: u32, pivot_b: u32, path: u32) {
    let data = this.addr();
    if path != 0 {
        e.call(ADD_REFERENCE, &args![path]);
    }
    let old = e.mem.u32(data + PATH);
    if old != 0 {
        e.call(REMOVE_REFERENCE, &args![old]);
    }
    e.mem.set_u32(data + PATH, path);
    for transform in [PIVOT_TRANSFORM, PATH_TRANSFORM] {
        for row in 0..4 {
            store_vec(e, data + transform + 0x10 * row, [0.0; 4]);
        }
        for row in 0..3 {
            e.mem.set_f32(data + transform + 0x10 * row + 4 * row, 1.0);
        }
    }
    copy_vector(e, pivot_a, data + PIVOT_TRANSFORM + 0x30);
    copy_vector(e, pivot_b, data + PATH_TRANSFORM + 0x30);
}

// Translated from 00deea90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPointToPathConstraintData::buildJacobian` (Xbox PDB). `input` is the query input
/// (transforms of the two bodies at +0x38 and +0x3C, a record at +0x64 whose float at +0x38
/// is the curve parameter) and `output` the query output. Writes the jacobian header record
/// (8), maps body B's pivot into the curve's frame and finds the nearest point (curve
/// virtual +0x10, which updates the parameter), its tangent (+0x14) and a unit vector
/// perpendicular to it (built from the two largest tangent components; its inverse length is
/// the refined `RSQRTSS`). With the curve's curvature-like virtual (+0x20, the parameter
/// passed) negated and scaled onto the tangent a position is formed; a positive maximum friction
/// (+0x1C) adds a friction jacobian. Two linear bilateral jacobians follow (the perpendicular
/// and tangent-times-perpendicular directions). An orientation type above 1 adds two angular
/// jacobians and type 3 two more (with the curve's binormal, virtual +0x24). Unless the curve
/// says (virtual +0x28) it is a closed loop, a linear limit jacobian is built from the
/// curve's two end parameters (virtuals +0x18 and +0x1C, each through +0x20).
///
/// The curve is read through its virtuals with the unconfirmed names above.
pub fn point_to_path_build_jacobian(e: &mut Engine, this: Ptr, input: u32, output: u32) {
    let data = this.addr();
    let record = e.mem.u32(input + 0x64);
    e.call(HEADER_RECORD, &args![input, output, record, 8u32]);
    e.with_stack(0x180, |e, frame| {
        let m = frame.addr();
        let (nearest, tangent_world) = (m + 0x20, m + 0x40);
        let (first, second) = (m + 0xa0, m + 0x130);
        // `second` is the transform of body A's pivot, `first` of body B's.
        e.call(
            TRANSFORM_SET_MUL,
            &args![second, e.mem.u32(input + 0x3c), data + PATH_TRANSFORM],
        );
        e.call(
            TRANSFORM_SET_MUL,
            &args![first, e.mem.u32(input + 0x38), data + PIVOT_TRANSFORM],
        );
        let record = e.mem.u32(input + 0x64);
        let pivot = m + 0xd0;
        e.call(
            VECTOR_SET_TRANSFORMED_INVERSE_POS,
            &args![nearest, second, pivot],
        );
        let curve = e.mem.u32(data + PATH);
        let hint = x87_quiet(e.mem.f32(record + 0x38));
        let parameter = e.vcall(curve, 0x10, &args![hint, nearest, nearest]).f32();
        e.mem.set_f32(record + 0x38, parameter);
        let point = m + 0xf0;
        e.call(VECTOR_SET_TRANSFORMED_POS, &args![point, second, nearest]);
        let curve = e.mem.u32(data + PATH);
        let parameter = x87_quiet(e.mem.f32(record + 0x38));
        e.vcall(curve, 0x14, &args![parameter, nearest]);
        e.call(
            VECTOR_SET_ROTATED_DIR,
            &args![tangent_world, second, nearest],
        );
        // The two other components than the smallest of |tangent|, as indices `k` and `j`.
        let tangent = load_vec(e, tangent_world);
        let abs_x = x87_quiet(tangent[0]).abs();
        let abs_y = x87_quiet(tangent[1]).abs();
        let abs_z = x87_quiet(tangent[2]).abs();
        let mut smallest = abs_x;
        let (mut k, mut j, mut index) = (1usize, 2usize, 0usize);
        if abs_x > abs_y {
            smallest = abs_y;
            k = 0;
            index = 1;
        }
        if smallest > abs_z {
            j = index;
        }
        let mut perpendicular = [0.0f32; 4];
        perpendicular[k] = x87_quiet(tangent[j]);
        perpendicular[j] = -x87_quiet(tangent[k]);
        store_vec(e, nearest, perpendicular);
        let parameter = x87_quiet(e.mem.f32(record + 0x38));
        let inverse = inverse_length(
            e,
            sum3(zip_with(perpendicular, perpendicular, |a, b| a * b)),
        );
        let unit = zip_with([inverse; 4], perpendicular, |a, b| a * b);
        let bilateral = cross_yzx(tangent, unit);
        let curve = e.mem.u32(data + PATH);
        store_vec(e, nearest, unit);
        store_vec(e, m + 0x90, bilateral);
        let factor = -e.vcall(curve, 0x20, &args![parameter]).f32();
        let tangent = load_vec(e, tangent_world);
        let pivot_vector = load_vec(e, pivot);
        let scaled = zip_with([factor; 4], tangent, |a, b| a * b);
        let target = zip_with(scaled, pivot_vector, |a, b| a + b);
        store_vec(e, m + 0xe0, target);
        if 0.0 < f64::from(e.mem.f32(data + MAX_FRICTION)) {
            // The friction block: position, direction, the friction force and the record.
            let friction = x87_quiet(e.mem.f32(data + MAX_FRICTION));
            e.mem.set_f32(m + 0x70, friction);
            store_vec(e, m + 0x60, tangent);
            store_vec(e, m + 0x50, target);
            e.mem.set_u32(m + 0x74, record);
            e.call(LINEAR_FRICTION_JACOBIAN, &args![m + 0x50, input, output]);
        }
        let block = m + 0x100;
        copy_vector(e, pivot, block);
        copy_vector(e, point, block + 0x10);
        copy_vector(e, nearest, block + 0x20);
        e.call(LINEAR_BILATERAL_JACOBIAN, &args![block, input, output]);
        copy_vector(e, m + 0x90, block + 0x20);
        e.call(LINEAR_BILATERAL_JACOBIAN, &args![block, input, output]);
        let orientation = e.mem.i8(data + ORIENTATION_TYPE);
        if orientation > 1 {
            let angular = m + 0x50;
            store_vec(e, angular, tangent);
            copy_vector(e, m + 0xc0, angular + 0x20);
            copy_vector(e, m + 0xb0, angular + 0x10);
            e.call(ANGULAR_BILATERAL_JACOBIAN, &args![angular, input, output]);
            copy_vector(e, m + 0xb0, angular + 0x20);
            let row_two = load_bits(e, m + 0xc0);
            let mask = load_bits(e, SIGN_MASK);
            for lane in 0..4 {
                e.mem.set_u32(
                    angular + 0x10 + 4 * lane,
                    row_two[lane as usize] ^ mask[lane as usize],
                );
            }
            e.call(ANGULAR_BILATERAL_JACOBIAN, &args![angular, input, output]);
            if e.mem.u8(data + ORIENTATION_TYPE) == 3 {
                let curve = e.mem.u32(data + PATH);
                let parameter = x87_quiet(e.mem.f32(record + 0x38));
                e.vcall(curve, 0x24, &args![parameter, m + 0x90]);
                e.call(VECTOR_SET_ROTATED_DIR, &args![nearest, second, m + 0x90]);
                copy_vector(e, m + 0xc0, angular);
                let negated = load_bits(e, nearest);
                let mask = load_bits(e, SIGN_MASK);
                for lane in 0..4 {
                    e.mem.set_u32(
                        angular + 0x20 + 4 * lane,
                        negated[lane as usize] ^ mask[lane as usize],
                    );
                }
                copy_vector(e, m + 0xa0, angular + 0x10);
                e.call(ANGULAR_BILATERAL_JACOBIAN, &args![angular, input, output]);
            }
        }
        let curve = e.mem.u32(data + PATH);
        let flag = e.with_stack(4, |e, out| {
            let flag_pointer = e.vcall(curve, 0x28, &args![out]).u32();
            e.mem.u8(flag_pointer)
        });
        if flag == 0 {
            let curve = e.mem.u32(data + PATH);
            copy_vector(e, pivot, m + 0x50);
            copy_vector(e, m + 0xe0, m + 0x60);
            let start = e.vcall(curve, 0x18, &args![]).f32();
            let end_0 = e.vcall(curve, 0x20, &args![start]).f32();
            e.mem.set_f32(m + 0x80, end_0);
            let curve = e.mem.u32(data + PATH);
            let stop = e.vcall(curve, 0x1c, &args![]).f32();
            let end_1 = e.vcall(curve, 0x20, &args![stop]).f32();
            e.mem.set_f32(m + 0x84, end_1);
            copy_vector(e, m + 0x40, m + 0x70);
            e.call(LINEAR_LIMIT_JACOBIAN, &args![m + 0x50, input, output]);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00de_ef10, deleting_destructor_00deef10(Ptr, u8) -> Ptr),
        entry!(0x00de_e720, point_to_path_get_runtime_info(Ptr, u32, u32)),
        entry!(0x00de_e740, point_to_path_get_type(Ptr) -> u32),
        entry!(0x00de_e750, point_to_path_construct(Ptr) -> Ptr),
        entry!(0x00de_e7d0, point_to_path_destruct(Ptr)),
        entry!(0x00de_e7f0, point_to_path_get_constraint_info(Ptr, u32)),
        entry!(0x00de_e820, point_to_path_calc_pivot(Ptr, u32, u32)),
        entry!(0x00de_e840, point_to_path_is_valid(Ptr, u32) -> u32),
        entry!(
            0x00de_e870,
            point_to_path_set_in_world_space(Ptr, u32, u32, u32, u32, u32)
        ),
        entry!(0x00de_ea00, fn_00deea00(Ptr, u32, u32, u32)),
        entry!(0x00de_ea90, point_to_path_build_jacobian(Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00deef10_frees_only_when_bit_zero_is_set() {
        check(0x00de_ef10, 0x00de_e7d0);
    }
}

#[cfg(test)]
mod a05_point_to_path_tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    const SLOT_10: u32 = 0x0900_0510;
    const SLOT_14: u32 = 0x0900_0514;
    const SLOT_18: u32 = 0x0900_0518;
    const SLOT_1C: u32 = 0x0900_051c;
    const SLOT_20: u32 = 0x0900_0520;
    const SLOT_24: u32 = 0x0900_0524;
    const SLOT_28: u32 = 0x0900_0528;

    fn world() -> Engine {
        let mut e = Engine::new();
        for page in [0x0101_1000, 0x0101_7000, 0x0101_6000, 0x010c_4000] {
            e.map(page, 0x1000);
        }
        e.mem.set_f32(0x0101_1d78, 0.0);
        e.mem.set_f32(0x0101_7718, 3.0);
        e.mem.set_f32(0x0101_6248, 0.5);
        for lane in 0..4 {
            e.mem.set_u32(SIGN_MASK + 4 * lane, 0x8000_0000);
        }
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

    /// A path object whose vtable has doubles at slots 0x10 to 0x28.
    fn path_object(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x40);
        for (slot, double) in [
            (0x10, SLOT_10),
            (0x14, SLOT_14),
            (0x18, SLOT_18),
            (0x1c, SLOT_1C),
            (0x20, SLOT_20),
            (0x24, SLOT_24),
            (0x28, SLOT_28),
        ] {
            e.mem.set_u32(vtable + slot, double);
        }
        let path = e.mem.alloc(0x40);
        e.mem.set_u32(path, vtable);
        path
    }

    fn data_object(e: &mut Engine) -> u32 {
        e.mem.alloc(0xb0)
    }

    #[test]
    fn runtime_type_and_constraint_info() {
        let mut e = world();
        let data = data_object(&mut e);
        let info = e.mem.alloc(0x20);
        e.call(0x00de_e720, &args![data, 1u32, info]);
        assert_eq!((e.mem.u32(info), e.mem.u32(info + 4)), (0x3c, 7));
        assert_eq!(e.call(0x00de_e740, &args![data]).u32(), 3);
        let info = e.mem.alloc(0x20);
        e.call(0x00de_e7f0, &args![data, info]);
        let words: Vec<u32> = (0..6).map(|i| e.mem.u32(info + 4 * i)).collect();
        assert_eq!(words, vec![0, 0x180, 7, 8, data + 0xc, 0xc]);
    }

    #[test]
    fn construct_installs_identity_transforms_and_the_bridge_atom() {
        let mut e = world();
        e.register(BRIDGE_ATOM_INIT, |_, _| Ret::default());
        let data = data_object(&mut e);
        let got = e.call(0x00de_e750, &args![data]).u32();
        assert_eq!(got, data);
        assert_eq!(e.mem.u32(data), POINT_TO_PATH_VTABLE);
        assert_eq!(e.mem.u16(data + 6), 1);
        assert_eq!(e.mem.u16(data + 0xc), 1);
        assert_eq!(e.mem.u32(data + 0x18), 0);
        assert_eq!(e.mem.f32(data + 0x1c), 0.0);
        assert_eq!(e.mem.u8(data + 0x20), 1);
        for base in [0x30, 0x70] {
            assert_eq!(load_vec(&e, data + base), [1.0, 0.0, 0.0, 0.0]);
            assert_eq!(load_vec(&e, data + base + 0x10), [0.0, 1.0, 0.0, 0.0]);
            assert_eq!(load_vec(&e, data + base + 0x20), [0.0, 0.0, 1.0, 0.0]);
            assert_eq!(load_vec(&e, data + base + 0x30), [0.0; 4]);
        }
        assert_eq!(calls_to(&e, BRIDGE_ATOM_INIT), vec![vec![data + 0xc, data]]);
    }

    #[test]
    fn destruct_removes_the_path_reference() {
        let mut e = world();
        e.register(REMOVE_REFERENCE, |_, _| Ret::default());
        let data = data_object(&mut e);
        e.call(0x00de_e7d0, &args![data]);
        assert!(calls_to(&e, REMOVE_REFERENCE).is_empty());
        assert_eq!(e.mem.u32(data), BASE_VTABLE);
        e.mem.set_u32(data + 0x18, 0x4444);
        e.call(0x00de_e7d0, &args![data]);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE), vec![vec![0x4444]]);
    }

    #[test]
    fn calc_pivot_transforms_the_pivot_into_the_output() {
        let mut e = world();
        e.register(VECTOR_SET_TRANSFORMED_POS, |_, _| Ret::default());
        let data = data_object(&mut e);
        e.call(0x00de_e820, &args![data, 0x1000u32, 0x2000u32]);
        assert_eq!(
            calls_to(&e, VECTOR_SET_TRANSFORMED_POS),
            vec![vec![0x2000, 0x1000, data + 0x60]]
        );
    }

    #[test]
    fn is_valid_needs_a_path_and_an_orientation_type() {
        let mut e = world();
        let data = data_object(&mut e);
        let out = e.mem.alloc(8);
        for (path, orientation, expected) in [(0u32, 1u8, 0u8), (0x77, 0, 0), (0x77, 1, 1)] {
            e.mem.set_u32(data + 0x18, path);
            e.mem.set_u8(data + 0x20, orientation);
            e.mem.set_u8(out, 0xaa);
            assert_eq!(e.call(0x00de_e840, &args![data, out]).u32(), out);
            assert_eq!(e.mem.u8(out), expected);
        }
    }

    #[test]
    fn the_body_space_setter_stores_the_path_and_the_pivots() {
        let mut e = world();
        e.register(ADD_REFERENCE, |_, _| Ret::default());
        e.register(REMOVE_REFERENCE, |_, _| Ret::default());
        let data = data_object(&mut e);
        e.mem.set_u32(data + 0x18, 0x4444);
        let pivot_a = e.mem.alloc(0x10);
        store_vec(&mut e, pivot_a, [1.0, 2.0, 3.0, 4.0]);
        let pivot_b = e.mem.alloc(0x10);
        store_vec(&mut e, pivot_b, [5.0, 6.0, 7.0, 8.0]);
        e.call(0x00de_ea00, &args![data, pivot_a, pivot_b, 0x5555u32]);
        assert_eq!(calls_to(&e, ADD_REFERENCE), vec![vec![0x5555]]);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE), vec![vec![0x4444]]);
        assert_eq!(e.mem.u32(data + 0x18), 0x5555);
        assert_eq!(load_vec(&e, data + 0x30), [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, data + 0x60), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(load_vec(&e, data + 0xa0), [5.0, 6.0, 7.0, 8.0]);
        assert_eq!(load_vec(&e, data + 0x80), [0.0, 1.0, 0.0, 0.0]);
        // No path: no reference is added and none removed (the old one is 0x5555 now).
        e.call(0x00de_ea00, &args![data, pivot_a, pivot_b, 0u32]);
        assert_eq!(calls_to(&e, ADD_REFERENCE).len(), 1);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE).len(), 2);
        assert_eq!(e.mem.u32(data + 0x18), 0);
    }

    #[test]
    fn set_in_world_space_builds_the_frame_from_the_path() {
        let mut e = world();
        e.register(ADD_REFERENCE, |_, _| Ret::default());
        e.register(REMOVE_REFERENCE, |_, _| Ret::default());
        // Every multiply copies its second operand to the output, so the result is the frame.
        e.register(TRANSFORM_SET_MUL, |e, a| {
            for word in 0..16 {
                let value = e.mem.u32(a[2] + 4 * word);
                e.mem.set_u32(a[0] + 4 * word, value);
            }
            Ret::default()
        });
        e.register(TRANSFORM_SET_MUL_INVERSE_MUL, |e, a| {
            for word in 0..16 {
                let value = e.mem.u32(a[2] + 4 * word);
                e.mem.set_u32(a[0] + 4 * word, value);
            }
            Ret::default()
        });
        e.register(VECTOR_SET_TRANSFORMED_INVERSE_POS, |e, a| {
            store_vec(e, a[0], [1.0, 2.0, 3.0, 0.0]);
            Ret::default()
        });
        e.register(SLOT_10, |_, _| Ret {
            st0: 0.25,
            ..Ret::default()
        });
        e.register(SLOT_14, |e, a| {
            store_vec(e, a[2], [0.0, 1.0, 0.0, 0.0]);
            Ret::default()
        });
        e.register(SLOT_24, |e, a| {
            store_vec(e, a[2], [0.0, 0.0, 1.0, 0.0]);
            Ret::default()
        });
        let path = path_object(&mut e);
        let data = data_object(&mut e);
        e.mem.set_u32(data + 0x18, 0x4444);
        let path_transform = e.mem.alloc(0x40);
        for word in 0..16 {
            e.mem
                .set_f32(path_transform + 4 * word, 100.0 + word as f32);
        }
        e.call(
            0x00de_e870,
            &args![data, 0x1000u32, 0x2000u32, 0x3000u32, path, path_transform],
        );
        assert_eq!(calls_to(&e, ADD_REFERENCE), vec![vec![path]]);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE), vec![vec![0x4444]]);
        assert_eq!(e.mem.u32(data + 0x18), path);
        // The transform for the path was copied.
        assert_eq!(e.mem.f32(data + 0x70), 100.0);
        assert_eq!(e.mem.f32(data + 0xac), 115.0);
        // The curve was asked for the nearest point at parameter 0.0, then the tangent and the
        // binormal at the parameter it returned.
        let nearest = calls_to(&e, SLOT_10);
        assert_eq!(nearest.len(), 1);
        assert_eq!(nearest[0][1], 0);
        assert_eq!(nearest[0][2], nearest[0][3]);
        assert_eq!(calls_to(&e, SLOT_14)[0][1], 0.25f32.to_bits());
        assert_eq!(calls_to(&e, SLOT_24)[0][1], 0.25f32.to_bits());
        // The frame: tangent, binormal, tangent x binormal, the nearest point.
        assert_eq!(load_vec(&e, data + 0x30), [0.0, 1.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, data + 0x40), [0.0, 0.0, 1.0, 0.0]);
        assert_eq!(load_vec(&e, data + 0x50), [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, data + 0x60), [1.0, 2.0, 3.0, 0.0]);
        // The multiplies ran in the original's order with the right operands.
        let multiplies = calls_to(&e, TRANSFORM_SET_MUL);
        assert_eq!(multiplies.len(), 3);
        assert_eq!(multiplies[0][1..], [0x2000, path_transform]);
        assert_eq!(multiplies[1][1], path_transform);
        assert_eq!(multiplies[2][1], 0x2000);
        assert_eq!(multiplies[2][2], multiplies[1][0]);
        assert_eq!(multiplies[2][0], multiplies[0][0]);
        let inverse = calls_to(&e, TRANSFORM_SET_MUL_INVERSE_MUL);
        assert_eq!(inverse[0][1..], [0x1000, multiplies[0][0]]);
        assert_eq!(
            calls_to(&e, VECTOR_SET_TRANSFORMED_INVERSE_POS)[0][1..],
            [multiplies[0][0], 0x3000]
        );
    }

    /// What a jacobian builder saw: its address and the words of its block when called.
    type Seen = Rc<RefCell<Vec<(u32, Vec<f32>)>>>;

    struct Setup {
        tangent: [f32; 3],
        orientation: i8,
        friction: f32,
        closed: bool,
    }

    /// Runs `buildJacobian` with doubles for everything it calls. The multiplies write body
    /// B's transform (rotation rows scaled 11, 12, 13; translation `(7, 8, 9)`); the path's
    /// virtuals return fixed numbers. Returns what each jacobian builder was given.
    fn run(setup: Setup) -> (Engine, Seen, u32, u32) {
        let mut e = world();
        e.register(HEADER_RECORD, |_, _| Ret::default());
        e.register(TRANSFORM_SET_MUL, |e, a| {
            store_vec(e, a[0], [11.0, 0.0, 0.0, 0.0]);
            store_vec(e, a[0] + 0x10, [0.0, 12.0, 0.0, 0.0]);
            store_vec(e, a[0] + 0x20, [0.0, 0.0, 13.0, 0.0]);
            store_vec(e, a[0] + 0x30, [7.0, 8.0, 9.0, 0.0]);
            Ret::default()
        });
        e.register(VECTOR_SET_TRANSFORMED_INVERSE_POS, |_, _| Ret::default());
        e.register(VECTOR_SET_TRANSFORMED_POS, |e, a| {
            store_vec(e, a[0], [3.0, 3.0, 3.0, 0.0]);
            Ret::default()
        });
        let calls = Rc::new(Cell::new(0));
        let rotated = Rc::clone(&calls);
        let [x, y, z] = setup.tangent;
        e.register_double(VECTOR_SET_ROTATED_DIR, move |e, a| {
            let first = rotated.get() == 0;
            rotated.set(rotated.get() + 1);
            // The first call is the world tangent, the second (orientation 3) the binormal.
            store_vec(
                e,
                a[0],
                if first {
                    [x, y, z, 0.0]
                } else {
                    [0.0, 0.0, 1.0, 0.0]
                },
            );
            Ret::default()
        });
        e.register(SLOT_10, |_, _| Ret {
            st0: 0.5,
            ..Ret::default()
        });
        e.register(SLOT_14, |e, a| {
            store_vec(e, a[2], [0.25, 0.5, 0.75, 0.0]);
            Ret::default()
        });
        e.register(SLOT_18, |_, _| Ret {
            st0: 1.5,
            ..Ret::default()
        });
        e.register(SLOT_1C, |_, _| Ret {
            st0: 2.5,
            ..Ret::default()
        });
        let slot_20 = Rc::new(Cell::new(0));
        let counter = Rc::clone(&slot_20);
        e.register_double(SLOT_20, move |_, a| {
            counter.set(counter.get() + 1);
            // The first call is the one whose result is negated into the position factor.
            let st0 = if counter.get() == 1 {
                -2.0
            } else {
                f64::from(f32::from_bits(a[1])) * 10.0
            };
            Ret {
                st0,
                ..Ret::default()
            }
        });
        e.register(SLOT_24, |e, a| {
            store_vec(e, a[2], [0.0, 1.0, 0.0, 0.0]);
            Ret::default()
        });
        let closed = setup.closed;
        e.register_double(SLOT_28, move |e, a| {
            e.mem.set_u8(a[1], u8::from(closed));
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        let seen: Seen = Rc::new(RefCell::new(Vec::new()));
        for (address, words) in [
            (LINEAR_FRICTION_JACOBIAN, 10),
            (LINEAR_BILATERAL_JACOBIAN, 12),
            (ANGULAR_BILATERAL_JACOBIAN, 12),
            (LINEAR_LIMIT_JACOBIAN, 14),
        ] {
            let log = Rc::clone(&seen);
            e.register_double(address, move |e, a| {
                let block = (0..words).map(|i| e.mem.f32(a[0] + 4 * i)).collect();
                log.borrow_mut().push((address, block));
                Ret::default()
            });
        }
        let path = path_object(&mut e);
        let data = data_object(&mut e);
        e.mem.set_u32(data + 0x18, path);
        e.mem.set_f32(data + 0x1c, setup.friction);
        e.mem.set_u8(data + 0x20, setup.orientation as u8);
        let record = e.mem.alloc(0x40);
        e.mem.set_f32(record + 0x38, 0.125);
        let input = e.mem.alloc(0x70);
        e.mem.set_u32(input + 0x38, 0x6000);
        e.mem.set_u32(input + 0x3c, 0x6100);
        e.mem.set_u32(input + 0x64, record);
        let output = e.mem.alloc(0x10);
        e.call(0x00de_ea90, &args![data, input, output]);
        (e, seen, record, input)
    }

    fn blocks(seen: &Seen, address: u32) -> Vec<Vec<f32>> {
        seen.borrow()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, block)| block.clone())
            .collect()
    }

    fn close(actual: f32, expected: f32) {
        assert!(
            (actual - expected).abs() <= 2e-6,
            "{actual} is not close to {expected}"
        );
    }

    #[test]
    fn build_jacobian_with_friction_orientation_three_and_an_open_path() {
        let (e, seen, record, input) = run(Setup {
            tangent: [1.0, 0.0, 0.0],
            orientation: 3,
            friction: 0.75,
            closed: false,
        });
        // The header record, then the curve was asked for the nearest point with the
        // parameter stored in the record, which now holds the parameter it returned.
        assert_eq!(calls_to(&e, HEADER_RECORD)[0][2..], [record, 8]);
        assert_eq!(calls_to(&e, HEADER_RECORD)[0][0], input);
        assert_eq!(calls_to(&e, SLOT_10)[0][1], 0.125f32.to_bits());
        assert_eq!(e.mem.f32(record + 0x38), 0.5);
        // Friction: position `2 * tangent + pivot`, the tangent, the force and the record.
        let friction = blocks(&seen, LINEAR_FRICTION_JACOBIAN);
        assert_eq!(friction.len(), 1);
        assert_eq!(friction[0][..4], [9.0, 8.0, 9.0, 0.0]);
        assert_eq!(friction[0][4..8], [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(friction[0][8], 0.75);
        assert_eq!(friction[0][9].to_bits(), record);
        // Two linear jacobians: the pivot, the nearest point in world space and the unit
        // perpendicular (0, 0, -1), then the cross of the tangent with it (0, 1, 0).
        let linear = blocks(&seen, LINEAR_BILATERAL_JACOBIAN);
        assert_eq!(linear.len(), 2);
        assert_eq!(linear[0][..8], [7.0, 8.0, 9.0, 0.0, 3.0, 3.0, 3.0, 0.0]);
        close(linear[0][8], 0.0);
        close(linear[0][9], 0.0);
        close(linear[0][10], -1.0);
        assert_eq!(linear[1][..8], linear[0][..8]);
        close(linear[1][8], 0.0);
        close(linear[1][9], 1.0);
        close(linear[1][10], 0.0);
        // Three angular jacobians: (tangent, row 1, row 2), (tangent, -row 2, row 1) and, from
        // the binormal, (row 2, row 0, minus the world binormal).
        let angular = blocks(&seen, ANGULAR_BILATERAL_JACOBIAN);
        assert_eq!(angular.len(), 3);
        assert_eq!(angular[0][..4], [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(angular[0][4..8], [0.0, 12.0, 0.0, 0.0]);
        assert_eq!(angular[0][8..12], [0.0, 0.0, 13.0, 0.0]);
        assert_eq!(angular[1][4..8], [-0.0, -0.0, -13.0, -0.0]);
        assert_eq!(angular[1][8..12], [0.0, 12.0, 0.0, 0.0]);
        assert_eq!(angular[2][..4], [0.0, 0.0, 13.0, 0.0]);
        assert_eq!(angular[2][4..8], [11.0, 0.0, 0.0, 0.0]);
        assert_eq!(angular[2][8..12], [-0.0, -0.0, -1.0, -0.0]);
        // The limit: pivot, position, tangent and the two ends through virtual +0x20.
        let limit = blocks(&seen, LINEAR_LIMIT_JACOBIAN);
        assert_eq!(limit.len(), 1);
        assert_eq!(limit[0][..4], [7.0, 8.0, 9.0, 0.0]);
        assert_eq!(limit[0][4..8], [9.0, 8.0, 9.0, 0.0]);
        assert_eq!(limit[0][8..12], [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(limit[0][12..14], [15.0, 25.0]);
        let order: Vec<u32> = seen.borrow().iter().map(|(a, _)| *a).collect();
        assert_eq!(
            order,
            vec![
                LINEAR_FRICTION_JACOBIAN,
                LINEAR_BILATERAL_JACOBIAN,
                LINEAR_BILATERAL_JACOBIAN,
                ANGULAR_BILATERAL_JACOBIAN,
                ANGULAR_BILATERAL_JACOBIAN,
                ANGULAR_BILATERAL_JACOBIAN,
                LINEAR_LIMIT_JACOBIAN
            ]
        );
    }

    #[test]
    fn build_jacobian_without_friction_orientation_or_limit() {
        let (_, seen, _, _) = run(Setup {
            tangent: [0.2, 1.0, 0.5],
            orientation: 1,
            friction: 0.0,
            closed: true,
        });
        // |x| <= |y| and |x| <= |z|... the smallest is x, so the perpendicular is built from
        // y and z: (0, z, -y) / length.
        assert!(blocks(&seen, LINEAR_FRICTION_JACOBIAN).is_empty());
        assert!(blocks(&seen, ANGULAR_BILATERAL_JACOBIAN).is_empty());
        assert!(blocks(&seen, LINEAR_LIMIT_JACOBIAN).is_empty());
        let linear = blocks(&seen, LINEAR_BILATERAL_JACOBIAN);
        assert_eq!(linear.len(), 2);
        let length = (0.5f32 * 0.5 + 1.0).sqrt();
        close(linear[0][8], 0.0);
        close(linear[0][9], 0.5 / length);
        close(linear[0][10], -1.0 / length);
    }

    #[test]
    fn build_jacobian_picks_the_two_largest_tangent_components() {
        // |x| > |y| and |y| > |z|: the smallest is z, so the perpendicular is (y, -x, 0).
        let (_, seen, _, _) = run(Setup {
            tangent: [1.0, 0.5, 0.2],
            orientation: 2,
            friction: -1.0,
            closed: true,
        });
        let linear = blocks(&seen, LINEAR_BILATERAL_JACOBIAN);
        let length = (1.0f32 + 0.25).sqrt();
        close(linear[0][8], 0.5 / length);
        close(linear[0][9], -1.0 / length);
        close(linear[0][10], 0.0);
        // Orientation 2: two angular jacobians, no friction (negative force), no binormal.
        assert_eq!(blocks(&seen, ANGULAR_BILATERAL_JACOBIAN).len(), 2);
        assert!(blocks(&seen, LINEAR_FRICTION_JACOBIAN).is_empty());
    }
}
