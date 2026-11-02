//! `Havok SDK/hkatwojointsiksolver.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! SSE code computes lane by lane in `f32`; x87 code computes in `f64` and rounds when a `float`
//! is stored. `RSQRTSS` and `RCPPS` are the host CPU's estimates.

use crate::prelude::*;
use crate::units::havok_sdk::hkakeyframehierarchyutility::{
    add4, mul4, rotate_and_move, rotate_half, sub4,
};
use crate::units::havok_sdk::hkpboxmotion::{cross, mul};
use crate::units::havok_sdk::hkpconvexradiusbuilder::{normalising_scale, splat};
use crate::units::havok_sdk::hkpconvexradiusviewer::time_stamp_low;
use crate::units::havok_sdk::hkpsphereshape::{
    copy_vec, load_bits, load_vec, profile_record, refined_reciprocal, shufps, store_bits,
    store_vec, sum3, unpckhps, Vec4, HALF_F32, PROFILE_SLOT, THREE_F32, TLS_GET_VALUE, ZERO_F32,
};
use crate::x87::rsqrt_estimate;

/// The CRT square root (`00ec6040`, argument in ST0, answers in ST0).
const X87_SQRT: u32 = 0x00ec_6040;
/// `0.5` as a `double` and the two `double` bounds on the dot product (about `-0.999999` and
/// `0.999999`) of `00cdb910`.
const HALF_F64: u32 = 0x0101_1588;
const LOWER_BOUND: u32 = 0x010c_b650;
const UPPER_BOUND: u32 = 0x0109_deb0;
/// The vector `(0, 0, 0, 1)`.
const IDENTITY_ROTATION: u32 = 0x010c_71b0;
/// The function `00cb2700` (`hkquaternion.cpp`, `RET 4`, no name in the engine map): `this` =
/// result, `(vector)`; `00cdb910` hands the result to it when the two directions are (nearly)
/// opposite.
const OPPOSITE_DIRECTIONS: u32 = 0x00cb_2700;

// Translated from 00cdb910 (decompiled, FalloutNV.exe 1.4.0.525)
/// A fractional rotation between two directions (thiscall, `RET 0xc`; no name in the engine
/// map): `this` receives the quaternion that rotates `vector_b` towards `vector_a` by the
/// fraction `t` of the way. With `d` the dot product of the two vectors (the first three
/// lanes, `z + (y + x)`), `h = (((1 - t) + t * d) + 1) * 0.5` is computed on the x87 stack and
/// rounded to a `float`; when `h <= 0` or `d` is below the lower bound (a double) the result is
/// left to `00cb2700` (`this`, `vector_a`). When `d` is above the upper bound the result is the
/// vector `(0, 0, 0, 1)` at `010c71b0`. Otherwise `s` is the CRT square root of `h`
/// (`00ec6040`, rounded to a `float`), `q = (t * 0.5) / s` is rounded to a `float`, and the
/// result is the vector `(cross * q, s)` (`cross` = `vector_b x vector_a`) normalised by the
/// refined `RSQRTSS` scale, zero when the length is zero, in all four lanes.
pub fn fn_00cdb910(e: &mut Engine, this: Ptr, t: f32, vector_a: u32, vector_b: u32) {
    let this = this.addr();
    let a = load_vec(e, vector_a);
    let b = load_vec(e, vector_b);
    let d = sum3(mul(b, a));
    let t64 = f64::from(t);
    let x = ((1.0 - t64) + t64 * f64::from(d)) as f32;
    let half = e.mem.f64(HALF_F64);
    let h = ((1.0 + f64::from(x)) * half) as f32;
    if h <= 0.0 {
        e.call(OPPOSITE_DIRECTIONS, &args![this, vector_a]);
        return;
    }
    if f64::from(d) < e.mem.f64(LOWER_BOUND) {
        e.call(OPPOSITE_DIRECTIONS, &args![this, vector_a]);
        return;
    }
    if f64::from(d) > e.mem.f64(UPPER_BOUND) {
        let identity = load_bits(e, IDENTITY_ROTATION);
        store_bits(e, this, identity);
        return;
    }
    let root = e.call(X87_SQRT, &args![f64::from(h)]).f64() as f32;
    let factor = ((t64 * half) / f64::from(root)) as f32;
    let scaled = mul(cross(a, b), splat(factor));
    let rotation = [scaled[0], scaled[1], scaled[2], root];
    let squares = mul(rotation, rotation);
    let sum = (squares[3] + squares[1]) + (squares[2] + squares[0]);
    let zero = e.mem.f32(ZERO_F32);
    let three = e.mem.f32(THREE_F32);
    let half32 = e.mem.f32(HALF_F32);
    let scale = normalising_scale(sum, zero, half32, three);
    store_vec(e, this, mul(splat(scale), rotation));
}

// `hkaTwoJointsIkSolver::solve` (00cdbae0) and what it needs.
//
// `hkaTwoJointsIkSolver::Setup` is not in the Xbox PDB's type list; its fields are read off the
// code: `+0`, `+2`, `+4` the 16-bit indices of the first joint, the second joint and the end bone,
// `+6` and `+8` two more bone indices (negative when unused) whose transforms are kept relative
// to the first, respectively the second joint, `+0x10` the hinge axis in the second joint's
// space, `+0x20` the lower and `+0x24` the upper bound of the hinge cosine, `+0x28`, `+0x2c` and
// `+0x30` the gains of the first joint, the second joint and the end bone, `+0x40` the end
// target, `+0x50` the end target rotation, `+0x60` the end bone offset, `+0x70` the end bone
// rotation offset and the bytes `+0x80` (fix the end position) and `+0x81` (fix the end
// rotation). The pose is laid out as `hkapose.rs` describes.

/// `hkaPose::calculateBoneModelSpace` (`00cda180`, `ECX` = the pose, `(bone)`, answers the
/// transform's address) and `00cdad80` (`ECX` = the pose, `(bone, keep_children)`, answers the
/// address of the bone's model transform after making it the truth).
const CALCULATE_BONE_MODEL_SPACE: u32 = 0x00cd_a180;
const ACCESS_BONE_MODEL_SPACE: u32 = 0x00cd_ad80;
/// `hkQuaternion::setMul` (`00c66320`, `this` = the result, `(a, b)`).
const QUATERNION_SET_MUL: u32 = 0x00c6_6320;
/// `hkVector4::setTransformedPos_ov2` (`00c7f7a0`, `this` = the result, `(transform, vector)`).
const SET_TRANSFORMED_POS: u32 = 0x00c7_f7a0;
/// `005611c0` (no name in the engine map, `this` only): forwards to `005611e0`, which scales
/// the four lanes of `this` by factors it computes from it.
const SCALE_QUATERNION: u32 = 0x0056_11c0;
/// `hkMath::acos` (`00c74a70`, cdecl, one float, the answer in `ST0`).
const ACOS: u32 = 0x00c7_4a70;
/// `hkQuaternion::setAxisAngle` (`00cb2450`, `this` = the result, `(axis, angle)`).
const SET_AXIS_ANGLE: u32 = 0x00cb_2450;
/// `hkQuaternion::setSlerp` (`00cb2850`, `this` = the result, `(q0, q1, t)`).
const SET_SLERP: u32 = 0x00cb_2850;
/// `00c74ac0` (no name in the engine map, `this` = the result, `(a, b)`): a product of the two
/// quaternions.
const QUATERNION_PRODUCT: u32 = 0x00c7_4ac0;
/// `00c6cd30` (no name in the engine map, `this` = the destination, `(source)`): copies a
/// 0x30-byte transform.
const COPY_TRANSFORM: u32 = 0x00c6_cd30;
/// `1.0` as a `float`, the `double` `-2.0`, four times `-0.5`, four times `1.0`, the four sign
/// bits, the sign bits of the three lowest lanes and the all-ones mask of the three lowest lanes.
const ONE_F32: u32 = 0x0101_2068;
const MINUS_TWO_F64: u32 = 0x0106_d4c8;
const MINUS_HALF_LANES: u32 = 0x010c_71e0;
const ONE_LANES: u32 = 0x010c_7170;
const SIGN_BITS: u32 = 0x010c_44d0;
const SIGN_BITS_XYZ: u32 = 0x010c_4c00;
const MASK_XYZ: u32 = 0x010c_4c10;
/// The profiling labels: `"LtTWOJOINTSOLVER"` and `"StINIT"` (the start record),
/// `"StFIXLENGTH"`, `"StFIXDIRECTION"` and the closing `"lt"`.
const LABEL_SOLVER: u32 = 0x010c_b67c;
const LABEL_INIT: u32 = 0x010c_b674;
const LABEL_FIX_LENGTH: u32 = 0x010c_b668;
const LABEL_FIX_DIRECTION: u32 = 0x010c_b658;
const LABEL_END: u32 = 0x0107_6f38;
/// The solver's frame (`AND ESP,-16; SUB ESP,0x174`): the offsets below are from its aligned
/// base, as the code addresses them.
const FRAME_SIZE: u32 = 0x190;

/// Appends the 16-byte start record (`first` at `+0`, the time stamp at `+4`, `second` at
/// `+0xc`) to the thread's profiling stream while its cursor is below its limit.
fn profile_start_record(e: &mut Engine, first: u32, second: u32) {
    let slot: u32 = e.global(PROFILE_SLOT);
    let stream = e.call(TLS_GET_VALUE, &args![slot]).u32();
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, first);
        e.mem.set_u32(cursor + 0xc, second);
        e.mem.set_u32(cursor + 4, time_stamp_low());
        e.mem.set_u32(stream + 4, cursor + 0x10);
    }
}

/// The bone index at `setup + offset` (`MOVSX`).
fn setup_bone(e: &Engine, setup: u32, offset: u32) -> i32 {
    i32::from(e.mem.i16(setup + offset))
}

/// The address of the flag word of `bone` (`pose + 0x1c`).
fn flag_word(e: &Engine, pose: u32, bone: i32) -> u32 {
    e.mem
        .u32(pose + 0x1c)
        .wrapping_add((bone as u32).wrapping_mul(4))
}

/// `bone * 0x30`, the offset of a transform in the arrays.
fn transform_offset(bone: i32) -> u32 {
    (bone as u32).wrapping_mul(0x30)
}

/// The inlined `getBoneModelSpace`: `calculateBoneModelSpace` when the bone's flag byte has
/// bit 2, else the bone's entry of the model array (`pose + 0x10`).
fn bone_model(e: &mut Engine, pose: u32, bone: i32) -> u32 {
    if e.mem.u8(flag_word(e, pose, bone)) & 2 != 0 {
        e.call(CALCULATE_BONE_MODEL_SPACE, &args![pose, bone]).u32()
    } else {
        e.mem.u32(pose + 0x10).wrapping_add(transform_offset(bone))
    }
}

fn bits_xor(v: Vec4, mask: [u32; 4]) -> Vec4 {
    std::array::from_fn(|lane| f32::from_bits(v[lane].to_bits() ^ mask[lane]))
}

fn bits_and(v: Vec4, mask: [u32; 4]) -> Vec4 {
    std::array::from_fn(|lane| f32::from_bits(v[lane].to_bits() & mask[lane]))
}

/// The half of the inverse rotation of `v` by `q`, as the code spells it with `SHUFPS`: the
/// dot product `z + (y + x)` of `v * q`, `cross = v.yzx * q.zxy - v.zxy * q.yzx`, then
/// `cross * w + ((w * w - 0.5) * v + dot * q)` (the caller doubles it).
fn inverse_rotate_half(e: &Engine, q: Vec4, v: Vec4) -> Vec4 {
    let dot = sum3(mul4(v, q));
    let cross = sub4(
        mul4(shufps(v, v, 0xc9), shufps(q, q, 0xd2)),
        mul4(shufps(v, v, 0xd2), shufps(q, q, 0xc9)),
    );
    let w = shufps(q, q, 0xff);
    let scaled = mul4(cross, w);
    let half = mul4(add4(mul4(w, w), load_vec(e, MINUS_HALF_LANES)), v);
    add4(scaled, add4(half, mul4(splat(dot), q)))
}

/// The reciprocal of a scale: lanes `(x, y, z, 1)` (`UNPCKHPS` with the ones, `SHUFPS 0xc4`),
/// `RCPPS` and one Newton step `(r + r) - (r * x) * r`, the fourth lane masked to zero.
fn inverse_scale(e: &Engine, scale: Vec4) -> Vec4 {
    let lanes = shufps(scale, unpckhps(scale, load_vec(e, ONE_LANES)), 0xc4);
    let reciprocal: Vec4 = std::array::from_fn(|lane| refined_reciprocal(lanes[lane]));
    bits_and(reciprocal, load_bits(e, MASK_XYZ))
}

/// The inlined inverse of `parent` applied to `child` (the twist bones' start, `00cdbb97` and
/// `00cdbd53`): `frame + 0x50` gets the negated double of the parent's translation turned back
/// by its rotation, `frame + 0x60` the inverse rotation (the three lowest sign bits flipped) and
/// `frame + 0x70` the inverse scale; `out` gets the child's translation turned by the inverse
/// rotation, doubled, plus `frame + 0x50`, `out + 0x10` `setMul(frame + 0x60, child rotation)`
/// and `out + 0x20` the child's scale times the inverse scale.
fn relative_transform(e: &mut Engine, frame: u32, parent: u32, child: u32, out: u32) {
    let rotation = load_vec(e, parent + 0x10);
    let half = inverse_rotate_half(e, rotation, load_vec(e, parent));
    let negated = bits_xor(add4(half, half), load_bits(e, SIGN_BITS));
    let inverse_rotation = bits_xor(load_vec(e, parent + 0x10), load_bits(e, SIGN_BITS_XYZ));
    let scale = inverse_scale(e, load_vec(e, parent + 0x20));
    store_vec(e, frame + 0x70, scale);
    let moved = rotate_and_move(e, inverse_rotation, load_vec(e, child), negated);
    store_vec(e, frame + 0x50, negated);
    store_vec(e, frame + 0x60, inverse_rotation);
    store_vec(e, out, moved);
    e.call(
        QUATERNION_SET_MUL,
        &args![out + 0x10, frame + 0x60, child + 0x10],
    );
    let scaled = mul4(load_vec(e, child + 0x20), load_vec(e, frame + 0x70));
    store_vec(e, out + 0x20, scaled);
}

/// The inlined `model * relative` of the twist bones' end (`00cdcb79` and `00cdcf2e`):
/// `frame + 0x50` the relative translation turned by the model rotation, doubled, plus the
/// model translation; `frame + 0x60` `setMul(model rotation, relative rotation)`; `frame + 0x70`
/// the model scale times the relative scale.
fn compose_transform(e: &mut Engine, frame: u32, model: u32, relative: u32) {
    let moved = rotate_and_move(
        e,
        load_vec(e, model + 0x10),
        load_vec(e, relative),
        load_vec(e, model),
    );
    store_vec(e, frame + 0x50, moved);
    e.call(
        QUATERNION_SET_MUL,
        &args![frame + 0x60, model + 0x10, relative + 0x10],
    );
    let scaled = mul4(load_vec(e, model + 0x20), load_vec(e, relative + 0x20));
    store_vec(e, frame + 0x70, scaled);
}

/// How the walk copies a model transform into the local one for a bone without a parent (a
/// branch the walk's own parent test excludes): inline, or through `00c6cd30`.
#[derive(Clone, Copy, PartialEq)]
enum OrphanCopy {
    Inline,
    ByCall,
}

/// The local transform of `index` (offset `offset`) from its model transform and its parent's
/// (`parent`): the parent's inverse as in `relative_transform`, kept at `frame + 0xc0` (the
/// translation term), `+0xd0` (the inverse rotation) and `+0xe0` (the inverse scale).
fn update_local(e: &mut Engine, frame: u32, pose: u32, offset: u32, parent: i32) {
    let model = e.mem.u32(pose + 0x10);
    let parent_block = if e.mem.u8(flag_word(e, pose, parent)) & 2 != 0 {
        e.call(CALCULATE_BONE_MODEL_SPACE, &args![pose, parent])
            .u32()
    } else {
        model.wrapping_add(transform_offset(parent))
    };
    let rotation = load_vec(e, parent_block + 0x10);
    let half = inverse_rotate_half(e, rotation, load_vec(e, parent_block));
    let negated = bits_xor(add4(half, half), load_bits(e, SIGN_BITS));
    let inverse_rotation = bits_xor(
        load_vec(e, parent_block + 0x10),
        load_bits(e, SIGN_BITS_XYZ),
    );
    let scale = inverse_scale(e, load_vec(e, parent_block + 0x20));
    store_vec(e, frame + 0xe0, scale);
    let own = model.wrapping_add(offset);
    let local = e.mem.u32(pose + 4).wrapping_add(offset);
    let moved = rotate_and_move(e, inverse_rotation, load_vec(e, own), negated);
    store_vec(e, frame + 0xc0, negated);
    store_vec(e, frame + 0xd0, inverse_rotation);
    store_vec(e, local, moved);
    e.call(
        QUATERNION_SET_MUL,
        &args![local + 0x10, frame + 0xd0, own + 0x10],
    );
    let scaled = mul4(load_vec(e, own + 0x20), load_vec(e, frame + 0xe0));
    store_vec(e, local + 0x20, scaled);
}

/// The inlined `setBoneModelSpace` with propagation that ends each fix: `bone` gets bit 4; every
/// later bone whose parent (16-bit index, not `-1`) carries bit 4 has its local transform
/// brought up to date when its bit 1 is set (`update_local`, then bit 1 cleared), gets bit 4 and
/// clears the model-space byte (`pose + 0x28`); then every later bone with bit 4 swaps it for
/// bit 2. The model transform of `bone` becomes `frame + 0x50`, `+0x60`, `+0x70`, its flag word
/// 1, and the local-space byte (`pose + 0x29`) is cleared.
fn set_bone_model_space(e: &mut Engine, frame: u32, pose: u32, bone: i32, orphan: OrphanCopy) {
    let count = e.mem.u32(e.mem.u32(pose) + 0x1c) as i32;
    let at = flag_word(e, pose, bone);
    let word = e.mem.u32(at);
    e.mem.set_u32(at, word | 4);
    let mut index = bone.wrapping_add(1);
    while index < count {
        let parents = e.mem.u32(e.mem.u32(pose) + 0xc);
        let parent = e
            .mem
            .u16(parents.wrapping_add((index as u32).wrapping_mul(2)));
        if parent != 0xffff && e.mem.u8(flag_word(e, pose, i32::from(parent as i16))) & 4 != 0 {
            if e.mem.u8(flag_word(e, pose, index)) & 1 != 0 {
                let offset = transform_offset(index);
                if parent == 0xffff {
                    // Never taken: the test above already excludes a missing parent.
                    let model = e.mem.u32(pose + 0x10).wrapping_add(offset);
                    let local = e.mem.u32(pose + 4).wrapping_add(offset);
                    if orphan == OrphanCopy::ByCall {
                        e.call(COPY_TRANSFORM, &args![local, model]);
                    } else {
                        for part in [0, 0x10, 0x20] {
                            copy_vec(e, model + part, local + part);
                        }
                    }
                } else {
                    update_local(e, frame, pose, offset, i32::from(parent as i16));
                }
                let at = flag_word(e, pose, index);
                let word = e.mem.u32(at);
                e.mem.set_u32(at, word & 0xffff_fffe);
            }
            let at = flag_word(e, pose, index);
            let word = e.mem.u32(at);
            e.mem.set_u32(at, word | 4);
            e.mem.set_u8(pose + 0x28, 0);
        }
        index += 1;
    }
    let mut index = bone.wrapping_add(1);
    while index < count {
        let at = flag_word(e, pose, index);
        let word = e.mem.u32(at);
        if word & 4 != 0 {
            e.mem.set_u32(at, (word & 0xffff_fffb) | 2);
        }
        index += 1;
    }
    let model = e.mem.u32(pose + 0x10).wrapping_add(transform_offset(bone));
    for part in [0, 0x10, 0x20] {
        copy_vec(e, frame + 0x50 + part, model + part);
    }
    let at = flag_word(e, pose, bone);
    e.mem.set_u32(at, 1);
    e.mem.set_u8(pose + 0x29, 0);
}

/// The x87 clamp of the hinge cosine to the upper bound (`FCOM`, `TEST AH,0x41`): the bound
/// only when the cosine is above it (a NaN cosine stays).
fn clamp_to_upper(cosine: f64, upper: f64) -> f64 {
    if cosine > upper {
        upper
    } else {
        cosine
    }
}

/// The x87 clamp to the lower bound (`FCOM`, `TEST AH,5`, `JP`): the bound only when the
/// value is below it (unordered keeps the value).
fn clamp_to_lower(value: f64, lower: f64) -> f64 {
    if value < lower {
        lower
    } else {
        value
    }
}

/// The length fix (`00cdbf10`): bends the second joint about the hinge axis so that the end can
/// reach the target, then rotates the second joint's model transform by that angle.
fn fix_length(e: &mut Engine, frame: u32, setup: u32, pose: u32) {
    profile_record(e, LABEL_FIX_LENGTH);
    let (first_bone, second_bone) = (setup_bone(e, setup, 0), setup_bone(e, setup, 2));
    let first = bone_model(e, pose, first_bone);
    let second = bone_model(e, pose, second_bone);
    let end_bone = setup_bone(e, setup, 4);
    let end = bone_model(e, pose, end_bone);
    let (end_position, target) = if e.mem.u8(setup + 0x81) == 0 {
        copy_vec(e, setup + 0x40, frame + 0x120);
        e.call(SET_TRANSFORMED_POS, &args![frame + 0x40, end, setup + 0x60]);
        (load_vec(e, frame + 0x40), load_vec(e, frame + 0x120))
    } else {
        // The target moved back by the end bone offset: the offset turned back by the rotation
        // offset, doubled, then turned by the target rotation, doubled.
        let half = inverse_rotate_half(e, load_vec(e, setup + 0x70), load_vec(e, setup + 0x60));
        let offset = add4(half, half);
        let turned = rotate_half(e, load_vec(e, setup + 0x50), offset);
        let target = sub4(load_vec(e, setup + 0x40), add4(turned, turned));
        let end_position = load_vec(e, end);
        store_vec(e, frame + 0x120, target);
        store_vec(e, frame + 0x40, end_position);
        (end_position, target)
    };
    let second_position = load_vec(e, second);
    let first_position = load_vec(e, first);
    let first_to_second = sub4(second_position, first_position);
    let second_to_end = sub4(end_position, second_position);
    let first_to_target = sub4(target, first_position);
    store_vec(e, frame + 0x30, first_to_second);
    store_vec(e, frame + 0x130, second_to_end);
    store_vec(e, frame + 0x80, first_to_target);

    // The hinge axis in model space, from a scaled copy of the second joint's rotation.
    let second = bone_model(e, pose, second_bone);
    copy_vec(e, second + 0x10, frame + 0x20);
    e.call(SCALE_QUATERNION, &args![frame + 0x20]);
    let half = rotate_half(e, load_vec(e, frame + 0x20), load_vec(e, setup + 0x10));
    let axis = add4(half, half);
    // Each limb split into its part along the axis and the rest (the dot products go through
    // a float in the frame, `FLD`/`FSTP`).
    let along_first = x87_quiet(sum3(mul4(axis, first_to_second)));
    let first_along = mul4(splat(along_first), axis);
    let first_across = sub4(first_to_second, first_along);
    let along_second = x87_quiet(sum3(mul4(axis, second_to_end)));
    let second_along = mul4(splat(along_second), axis);
    let second_across = sub4(second_to_end, second_along);
    let along = sub4(first_along, second_along);
    store_vec(e, frame + 0xb0, first_across);
    store_vec(e, frame + 0xa0, second_across);
    let along_squares = mul4(along, along);
    let upper = x87_quiet(e.mem.f32(setup + 0x24));
    let along_length = sum3(along_squares);
    let target_squares = mul4(first_to_target, first_to_target);
    let target_length = sum3(target_squares);
    let reach = (f64::from(x87_quiet(target_length)) - f64::from(x87_quiet(along_length))) as f32;
    let first_squares = mul4(first_across, first_across);
    let first_length = sum3(first_squares);
    store_vec(e, frame + 0x80, first_squares);
    let first_length_x87 = f64::from(x87_quiet(first_length));
    let partial = f64::from(x87_quiet(reach)) - first_length_x87;
    let second_squares = mul4(second_across, second_across);
    let second_length = sum3(second_squares);
    let numerator = partial - f64::from(x87_quiet(second_length));
    store_vec(e, frame + 0x130, second_squares);
    let first_root = e.call(X87_SQRT, &args![first_length_x87]).f64() as f32;
    let denominator = f64::from(x87_quiet(first_root)) * e.mem.f64(MINUS_TWO_F64);
    let second_root = e
        .call(X87_SQRT, &args![f64::from(x87_quiet(second_length))])
        .f64() as f32;
    let cosine = (numerator / (f64::from(x87_quiet(second_root)) * denominator)) as f32;
    let cosine = clamp_to_upper(f64::from(x87_quiet(cosine)), f64::from(upper)) as f32;
    let lower = x87_quiet(e.mem.f32(setup + 0x20));
    let cosine = clamp_to_lower(f64::from(x87_quiet(cosine)), f64::from(lower)) as f32;

    // The current angle between the two limbs (across the axis), from their unit vectors.
    let zero = e.mem.f32(ZERO_F32);
    let three = e.mem.f32(THREE_F32);
    let half_f32 = e.mem.f32(HALF_F32);
    let first_scale = normalising_scale(sum3(load_vec(e, frame + 0x80)), zero, half_f32, three);
    store_vec(e, frame + 0x90, [half_f32, 0.0, 0.0, 0.0]);
    let second_sum = sum3(load_vec(e, frame + 0x130));
    let first_unit = mul4(splat(first_scale), load_vec(e, frame + 0xb0));
    store_vec(e, frame + 0x170, [three, 0.0, 0.0, 0.0]);
    let first_unit = bits_xor(first_unit, load_bits(e, SIGN_BITS_XYZ));
    let second_scale = normalising_scale(second_sum, zero, half_f32, three);
    let second_unit = mul4(splat(second_scale), load_vec(e, frame + 0xa0));
    let current = sum3(mul4(first_unit, second_unit));
    store_vec(e, frame + 0x80, [zero, 0.0, 0.0, 0.0]);
    let second_model = e
        .call(ACCESS_BONE_MODEL_SPACE, &args![pose, second_bone, 1u32])
        .u32();
    let rotation = second_model + 0x10;
    copy_vec(e, rotation, frame + 0x20);
    let half = rotate_half(e, load_vec(e, frame + 0x20), load_vec(e, setup + 0x10));
    let axis = add4(half, half);
    let estimate = rsqrt_estimate(sum3(mul4(axis, axis)));
    store_vec(e, frame + 0xa0, mul4(splat(estimate), axis));
    let wanted = e.call(ACOS, &args![x87_quiet(cosine).to_bits()]).f64() as f32;
    let now = e.call(ACOS, &args![x87_quiet(current).to_bits()]).f64();
    let difference = (f64::from(x87_quiet(wanted)) - now) as f32;
    let gain = x87_quiet(e.mem.f32(setup + 0x2c));
    let angle = (f64::from(x87_quiet(difference)) * f64::from(gain)) as f32;
    e.call(
        SET_AXIS_ANGLE,
        &args![frame + 0xb0, frame + 0xa0, x87_quiet(angle).to_bits()],
    );
    e.call(
        QUATERNION_SET_MUL,
        &args![rotation, frame + 0xb0, frame + 0x20],
    );
    e.call(SCALE_QUATERNION, &args![rotation]);
}

/// The direction fix (`00cdc5e8`): turns the first joint so that the direction to the end moves
/// towards the direction to the target by the first joint's gain (`00cdb910`).
fn fix_direction(e: &mut Engine, frame: u32, setup: u32, pose: u32) {
    profile_record(e, LABEL_FIX_DIRECTION);
    let first_bone = setup_bone(e, setup, 0);
    let first = bone_model(e, pose, first_bone);
    let end_bone = setup_bone(e, setup, 4);
    let end_position = if e.mem.u8(setup + 0x81) != 0 {
        let end = bone_model(e, pose, end_bone);
        let position = load_vec(e, end);
        store_vec(e, frame + 0x30, position);
        position
    } else {
        let end = bone_model(e, pose, end_bone);
        e.call(SET_TRANSFORMED_POS, &args![frame + 0x30, end, setup + 0x60]);
        load_vec(e, frame + 0x30)
    };
    let gain = x87_quiet(e.mem.f32(setup + 0x28));
    let to_end = sub4(end_position, load_vec(e, first));
    let three = load_vec(e, frame + 0x170)[0];
    let half = load_vec(e, frame + 0x90)[0];
    let zero = load_vec(e, frame + 0x80)[0];
    let scale = normalising_scale(sum3(mul4(to_end, to_end)), zero, half, three);
    store_vec(e, frame + 0xa0, mul4(splat(scale), to_end));
    let to_target = sub4(load_vec(e, frame + 0x120), load_vec(e, first));
    let scale = normalising_scale(sum3(mul4(to_target, to_target)), zero, half, three);
    store_vec(e, frame + 0x90, mul4(splat(scale), to_target));
    fn_00cdb910(e, Ptr::new(frame + 0xb0), gain, frame + 0xa0, frame + 0x90);
    let first_model = e
        .call(ACCESS_BONE_MODEL_SPACE, &args![pose, first_bone, 1u32])
        .u32();
    let rotation = first_model + 0x10;
    copy_vec(e, rotation, frame + 0x20);
    e.call(
        QUATERNION_SET_MUL,
        &args![rotation, frame + 0xb0, frame + 0x20],
    );
    e.call(SCALE_QUATERNION, &args![rotation]);
}

/// The body after the early test, in the solver's frame (aligned base `frame`).
fn solve_in_frame(e: &mut Engine, frame: u32, setup: u32, pose: u32) {
    // The two relative transforms start as the identity: translation zero, rotation
    // (0, 0, 0, 1), scale 1.0 in all lanes.
    let identity = load_bits(e, IDENTITY_ROTATION);
    let one = e.mem.f32(ONE_F32);
    for base in [frame + 0xf0, frame + 0x140] {
        store_vec(e, base, [0.0; 4]);
        store_bits(e, base + 0x10, identity);
        store_vec(e, base + 0x20, splat(one));
    }
    let first_twist = setup_bone(e, setup, 6);
    if first_twist >= 0 {
        let twist = bone_model(e, pose, first_twist);
        let joint = setup_bone(e, setup, 0);
        let joint = bone_model(e, pose, joint);
        relative_transform(e, frame, joint, twist, frame + 0xf0);
    }
    let second_twist = setup_bone(e, setup, 8);
    if second_twist >= 0 {
        let twist = bone_model(e, pose, second_twist);
        let joint = setup_bone(e, setup, 2);
        let joint = bone_model(e, pose, joint);
        relative_transform(e, frame, joint, twist, frame + 0x140);
    }
    if e.mem.u8(setup + 0x80) != 0 {
        fix_length(e, frame, setup, pose);
        fix_direction(e, frame, setup, pose);
    }
    if e.mem.u8(setup + 0x81) != 0 {
        e.call(
            QUATERNION_PRODUCT,
            &args![frame + 0x90, setup + 0x50, setup + 0x70],
        );
        let end_bone = setup_bone(e, setup, 4);
        let end = bone_model(e, pose, end_bone);
        let gain = x87_quiet(e.mem.f32(setup + 0x30));
        for part in [0, 0x10, 0x20] {
            copy_vec(e, end + part, frame + 0x50 + part);
        }
        e.call(
            SET_SLERP,
            &args![frame + 0x60, end + 0x10, frame + 0x90, gain.to_bits()],
        );
        set_bone_model_space(e, frame, pose, end_bone, OrphanCopy::Inline);
    }
    if setup_bone(e, setup, 6) >= 0 {
        let joint = setup_bone(e, setup, 0);
        let joint = bone_model(e, pose, joint);
        compose_transform(e, frame, joint, frame + 0xf0);
        let twist = setup_bone(e, setup, 6);
        set_bone_model_space(e, frame, pose, twist, OrphanCopy::Inline);
    }
    if setup_bone(e, setup, 8) >= 0 {
        let joint = setup_bone(e, setup, 2);
        let joint = bone_model(e, pose, joint);
        compose_transform(e, frame, joint, frame + 0x140);
        let twist = setup_bone(e, setup, 8);
        set_bone_model_space(e, frame, pose, twist, OrphanCopy::ByCall);
    }
    profile_record(e, LABEL_END);
}

// Translated from 00cdbae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaTwoJointsIkSolver::solve` (Xbox PDB name per the engine map; cdecl `(result, setup,
/// pose)`, the `hkResult` returned through `result`, which is also the answer in `EAX`).
///
/// Writes the start record (`"LtTWOJOINTSOLVER"`, `"StINIT"`) to the profiling stream. When
/// neither the position byte (`setup + 0x80`) nor the rotation byte (`+0x81`) is set, nothing
/// else happens. Otherwise, in a 16-byte aligned frame:
///
/// - Each twist bone (`+6`, `+8`, when not negative) has its transform relative to the first
///   (respectively the second) joint taken: the inverse of the joint's model transform times the
///   twist bone's (both through `calculateBoneModelSpace` when the bone's flag has bit 2). An
///   unused one stays the identity.
/// - With the position byte, the length fix: the target (`+0x40`, or with the rotation byte the
///   target moved back by the end offset turned by the offset rotation and the target rotation)
///   and the end position (`setTransformedPos` of the end bone and the offset `+0x60`, or the end
///   bone's translation with the rotation byte); the hinge axis (`+0x10`) turned by the second
///   joint's rotation (a copy scaled by `005611c0`); both limbs split along the axis; the wanted
///   hinge cosine `(|target|^2 - along^2 - a^2 - b^2) / (-2 * a * b)` on the x87 stack, clamped
///   to the upper (`+0x24`) and the lower bound (`+0x20`); the angle `acos(wanted) -
///   acos(current)` times the second joint's gain (`+0x2c`) around the axis (`setAxisAngle`)
///   multiplied into the second joint's model rotation (`00cdad80`, `setMul`, `005611c0`). Then
///   the direction fix: the unit directions from the first joint to the end and to the target,
///   `00cdb910` with the first joint's gain (`+0x28`) multiplied into the first joint's model
///   rotation the same way. The fixes are marked `"StFIXLENGTH"` and `"StFIXDIRECTION"`.
/// - With the rotation byte, the end bone's model rotation becomes `setSlerp` from its own to the
///   product (`00c74ac0`) of the target rotation (`+0x50`) and the rotation offset (`+0x70`) by
///   the end gain (`+0x30`), set with propagation to the later bones.
/// - Each twist bone gets its joint's model transform times its relative transform, set with
///   propagation.
///
/// Ends with the `"lt"` record. The byte at `result` always receives 1.
pub fn hka_two_joints_ik_solver_solve(e: &mut Engine, result: u32, setup: u32, pose: u32) -> u32 {
    profile_start_record(e, LABEL_SOLVER, LABEL_INIT);
    if e.mem.u8(setup + 0x80) != 0 || e.mem.u8(setup + 0x81) != 0 {
        e.with_stack(FRAME_SIZE, |e, block| {
            let frame = (block.addr() + 0xf) & !0xf;
            solve_in_frame(e, frame, setup, pose);
        });
    }
    e.mem.set_u8(result, 1);
    result
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cd_b910, fn_00cdb910(Ptr, f32, u32, u32)),
        entry!(
            0x00cd_bae0,
            hka_two_joints_ik_solver_solve(u32, u32, u32) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::FRAC_1_SQRT_2;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_f64(HALF_F64, 0.5);
        e.mem.set_f64(LOWER_BOUND, -0.999999);
        e.mem.set_f64(UPPER_BOUND, 0.999999);
        e.mem.set_f32(ZERO_F32, 0.0);
        e.mem.set_f32(THREE_F32, 3.0);
        e.mem.set_f32(HALF_F32, 0.5);
        for (lane, value) in [0.0f32, 0.0, 0.0, 1.0].into_iter().enumerate() {
            e.mem.set_f32(IDENTITY_ROTATION + 4 * lane as u32, value);
        }
        e.call_log = Some(vec![]);
        e.register_double(X87_SQRT, |_, a| {
            let value = f64::from_bits((u64::from(a[1]) << 32) | u64::from(a[0]));
            Ret {
                st0: value.sqrt(),
                ..Ret::default()
            }
        });
        e.register_double(OPPOSITE_DIRECTIONS, |_, _| Ret::default());
        e
    }

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(called, _)| *called == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    fn vector(e: &mut Engine, lanes: [f32; 4]) -> u32 {
        let at = e.mem.alloc(16);
        store_vec(e, at, lanes);
        at
    }

    #[test]
    fn opposite_directions_are_left_to_the_other_function() {
        let mut e = engine();
        let a = vector(&mut e, [1.0, 0.0, 0.0, 0.0]);
        let b = vector(&mut e, [-1.0, 0.0, 0.0, 0.0]);
        let out = vector(&mut e, [9.0; 4]);
        // Half way: h = 0.5 is positive, the dot product -1 is below the lower bound.
        e.call(0x00cd_b910, &args![out, 0.5f32, a, b]);
        assert_eq!(calls(&e, OPPOSITE_DIRECTIONS), vec![vec![out, a]]);
        assert!(calls(&e, X87_SQRT).is_empty());
        assert_eq!(load_vec(&e, out), [9.0; 4]);
        // The whole way: h = 0, the same answer.
        e.call(0x00cd_b910, &args![out, 1.0f32, a, b]);
        assert_eq!(calls(&e, OPPOSITE_DIRECTIONS).len(), 2);
    }

    #[test]
    fn a_negative_h_is_left_to_the_other_function_too() {
        let mut e = engine();
        let a = vector(&mut e, [1.0, 0.0, 0.0, 0.0]);
        let b = vector(&mut e, [0.0, 1.0, 0.0, 0.0]);
        let out = vector(&mut e, [9.0; 4]);
        // t = 4: x = -3 + 0 = -3, h = (1 - 3) * 0.5 = -1.
        e.call(0x00cd_b910, &args![out, 4.0f32, a, b]);
        assert_eq!(calls(&e, OPPOSITE_DIRECTIONS), vec![vec![out, a]]);
    }

    #[test]
    fn parallel_directions_give_the_identity() {
        let mut e = engine();
        let a = vector(&mut e, [0.0, 0.0, 1.0, 0.0]);
        let b = vector(&mut e, [0.0, 0.0, 1.0, 0.0]);
        let out = vector(&mut e, [9.0; 4]);
        e.call(0x00cd_b910, &args![out, 0.5f32, a, b]);
        assert_eq!(load_vec(&e, out), [0.0, 0.0, 0.0, 1.0]);
        assert!(calls(&e, OPPOSITE_DIRECTIONS).is_empty());
        assert!(calls(&e, X87_SQRT).is_empty());
    }

    #[test]
    fn a_quarter_turn_gives_the_half_angle_quaternion() {
        let mut e = engine();
        let a = vector(&mut e, [1.0, 0.0, 0.0, 0.0]);
        let b = vector(&mut e, [0.0, 1.0, 0.0, 0.0]);
        let out = vector(&mut e, [9.0; 4]);
        // d = 0, t = 1: h = 0.5 and s = sqrt(0.5); the cross term in lane 2 is
        // b.y * a.x - b.x * a.y = 1.
        e.call(0x00cd_b910, &args![out, 1.0f32, a, b]);
        assert_eq!(
            calls(&e, X87_SQRT),
            vec![vec![
                0.5f64.to_bits() as u32,
                (0.5f64.to_bits() >> 32) as u32
            ]]
        );
        let result = load_vec(&e, out);
        let expected = [0.0, 0.0, FRAC_1_SQRT_2, FRAC_1_SQRT_2];
        for lane in 0..4 {
            assert!(
                (result[lane] - expected[lane]).abs() < 1e-5,
                "lane {lane}: {result:?}"
            );
        }
    }

    /// The solver's test rig: a pose of six bones (parents `-1, 0, 1, 2, 3, 0`) with identity
    /// rotations, unit scales and the given translations; a setup with the joints 1 and 2, the
    /// end bone 3, no twist bones, the hinge axis z, the target `(1, 1, 0)`, the bounds -1 and 1
    /// and the gains 1, 1 and 0.25; a profiling stream; and doubles for every callee outside
    /// this file.
    struct Rig {
        pose: u32,
        setup: u32,
        records: u32,
        result: u32,
    }

    fn rig(e: &mut Engine, positions: [Vec4; 6]) -> Rig {
        store_vec(e, MINUS_HALF_LANES, [-0.5; 4]);
        store_vec(e, ONE_LANES, [1.0; 4]);
        store_bits(e, SIGN_BITS, [0x8000_0000; 4]);
        store_bits(e, SIGN_BITS_XYZ, [0x8000_0000, 0x8000_0000, 0x8000_0000, 0]);
        store_bits(e, MASK_XYZ, [u32::MAX, u32::MAX, u32::MAX, 0]);
        e.mem.set_f32(ONE_F32, 1.0);
        e.mem.set_f64(MINUS_TWO_F64, -2.0);
        let skeleton = e.mem.alloc(0x20);
        let parents = e.mem.alloc(12);
        for (bone, parent) in [-1i16, 0, 1, 2, 3, 0].into_iter().enumerate() {
            e.mem.set_i16(parents + 2 * bone as u32, parent);
        }
        e.mem.set_u32(skeleton + 0xc, parents);
        e.mem.set_u32(skeleton + 0x1c, 6);
        let local = e.mem.alloc(6 * 0x30);
        let models = e.mem.alloc(6 * 0x30);
        let flags = e.mem.alloc(6 * 4);
        for (bone, position) in positions.into_iter().enumerate() {
            let at = models + 0x30 * bone as u32;
            store_vec(e, at, position);
            store_vec(e, at + 0x10, [0.0, 0.0, 0.0, 1.0]);
            store_vec(e, at + 0x20, [1.0; 4]);
        }
        let pose = e.mem.alloc(0x38);
        e.mem.set_u32(pose, skeleton);
        e.mem.set_u32(pose + 4, local);
        e.mem.set_u32(pose + 0x10, models);
        e.mem.set_u32(pose + 0x1c, flags);
        e.mem.set_u8(pose + 0x28, 1);
        e.mem.set_u8(pose + 0x29, 1);
        let setup = e.mem.alloc(0x90);
        for (offset, bone) in [(0, 1i16), (2, 2), (4, 3), (6, -1), (8, -1)] {
            e.mem.set_i16(setup + offset, bone);
        }
        store_vec(e, setup + 0x10, [0.0, 0.0, 1.0, 0.0]);
        for (offset, value) in [
            (0x20, -1.0f32),
            (0x24, 1.0),
            (0x28, 1.0),
            (0x2c, 1.0),
            (0x30, 0.25),
        ] {
            e.mem.set_f32(setup + offset, value);
        }
        store_vec(e, setup + 0x40, [1.0, 1.0, 0.0, 0.0]);
        store_vec(e, setup + 0x50, [0.0, 0.0, 0.0, 1.0]);
        store_vec(e, setup + 0x70, [0.0, 0.0, 0.0, 1.0]);
        let stream = e.mem.alloc(0x10);
        let records = e.mem.alloc(0x100);
        e.mem.set_u32(stream + 4, records);
        e.mem.set_u32(stream + 0xc, records + 0x100);
        e.set_global(PROFILE_SLOT, 7u32);
        e.register_double(TLS_GET_VALUE, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        // Both pose accessors answer the bone's entry of the model array.
        for address in [CALCULATE_BONE_MODEL_SPACE, ACCESS_BONE_MODEL_SPACE] {
            e.register_double(address, |e, a| Ret {
                eax: e.mem.u32(a[0] + 0x10) + a[1] * 0x30,
                ..Ret::default()
            });
        }
        e.register_double(QUATERNION_SET_MUL, |e, a| {
            let product = hamilton(load_vec(e, a[1]), load_vec(e, a[2]));
            store_vec(e, a[0], product);
            Ret::default()
        });
        // The transformed position stands in as the translation plus the vector.
        e.register_double(SET_TRANSFORMED_POS, |e, a| {
            let moved = add4(load_vec(e, a[1]), load_vec(e, a[2]));
            store_vec(e, a[0], moved);
            Ret::default()
        });
        e.register_double(SCALE_QUATERNION, |_, _| Ret::default());
        e.register_double(ACOS, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])).acos(),
            ..Ret::default()
        });
        e.register_double(SET_AXIS_ANGLE, |e, a| {
            let half = f64::from(f32::from_bits(a[2])) / 2.0;
            let axis = load_vec(e, a[1]);
            let sine = half.sin() as f32;
            let turn = [
                axis[0] * sine,
                axis[1] * sine,
                axis[2] * sine,
                half.cos() as f32,
            ];
            store_vec(e, a[0], turn);
            Ret::default()
        });
        e.register_double(SET_SLERP, |e, a| {
            copy_vec(e, a[2], a[0]);
            Ret::default()
        });
        e.register_double(QUATERNION_PRODUCT, |e, a| {
            store_vec(e, a[0], [0.0, 0.0, 0.6, 0.8]);
            Ret::default()
        });
        e.register_double(COPY_TRANSFORM, |_, _| Ret::default());
        let result = e.mem.alloc(4);
        Rig {
            pose,
            setup,
            records,
            result,
        }
    }

    /// The Hamilton product `a * b` of two quaternions `(x, y, z, w)`.
    fn hamilton(a: Vec4, b: Vec4) -> Vec4 {
        [
            a[3] * b[0] + b[3] * a[0] + (a[1] * b[2] - a[2] * b[1]),
            a[3] * b[1] + b[3] * a[1] + (a[2] * b[0] - a[0] * b[2]),
            a[3] * b[2] + b[3] * a[2] + (a[0] * b[1] - a[1] * b[0]),
            a[3] * b[3] - (a[0] * b[0] + a[1] * b[1] + a[2] * b[2]),
        ]
    }

    fn model_of(e: &Engine, rig: &Rig, bone: u32) -> u32 {
        e.mem.u32(rig.pose + 0x10) + bone * 0x30
    }

    fn flags_of(e: &Engine, rig: &Rig, bone: u32) -> u32 {
        e.mem.u32(e.mem.u32(rig.pose + 0x1c) + 4 * bone)
    }

    fn assert_near(actual: Vec4, expected: Vec4, tolerance: f32) {
        for (lane, (got, wanted)) in actual.iter().zip(expected).enumerate() {
            assert!(
                (got - wanted).abs() <= tolerance,
                "lane {lane}: {actual:?} against {expected:?}"
            );
        }
    }

    fn solve(e: &mut Engine, rig: &Rig) -> u32 {
        e.call(0x00cd_bae0, &args![rig.result, rig.setup, rig.pose])
            .u32()
    }

    const CHAIN: [Vec4; 6] = [
        [0.0; 4],
        [0.0; 4],
        [1.0, 0.0, 0.0, 0.0],
        [2.0, 0.0, 0.0, 0.0],
        [3.0, 0.0, 0.0, 0.0],
        [0.0, 5.0, 0.0, 0.0],
    ];

    #[test]
    fn solve_without_either_fix_only_writes_the_start_record() {
        let mut e = engine();
        let rig = rig(&mut e, CHAIN);
        assert_eq!(solve(&mut e, &rig), rig.result);
        assert_eq!(e.mem.u8(rig.result), 1);
        let called: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(address, _)| *address)
            .collect();
        assert_eq!(called, vec![0x00cd_bae0, TLS_GET_VALUE]);
        assert_eq!(e.mem.u32(rig.records), LABEL_SOLVER);
        assert_eq!(e.mem.u32(rig.records + 0xc), LABEL_INIT);
        assert_eq!(e.mem.u32(rig.records + 0x10), 0);
        assert_eq!(e.mem.u8(rig.pose + 0x28), 1);
        assert_eq!(flags_of(&e, &rig, 3), 0);
    }

    #[test]
    fn solve_fixes_the_length_then_the_direction() {
        let mut e = engine();
        let rig = rig(&mut e, CHAIN);
        e.mem.set_u8(rig.setup + 0x80, 1);
        // The wanted cosine (-0.0 for this right angle) is clamped to the upper bound -0.5.
        e.mem.set_f32(rig.setup + 0x24, -0.5);
        assert_eq!(solve(&mut e, &rig), rig.result);
        assert_eq!(e.mem.u8(rig.result), 1);

        let end = model_of(&e, &rig, 3);
        let moves = calls(&e, SET_TRANSFORMED_POS);
        assert_eq!(moves.len(), 2);
        let frame = moves[0][0] - 0x40;
        assert_eq!(moves[0], vec![frame + 0x40, end, rig.setup + 0x60]);
        assert_eq!(moves[1], vec![frame + 0x30, end, rig.setup + 0x60]);
        assert_eq!(
            calls(&e, ACCESS_BONE_MODEL_SPACE),
            vec![vec![rig.pose, 2, 1], vec![rig.pose, 1, 1]]
        );
        let angles = calls(&e, ACOS);
        assert_eq!(angles.len(), 2);
        assert_eq!(angles[0], vec![(-0.5f32).to_bits()]);
        assert!((f32::from_bits(angles[1][0]) + 1.0).abs() < 1e-5);
        let turns = calls(&e, SET_AXIS_ANGLE);
        assert_eq!(turns.len(), 1);
        assert_eq!(turns[0][..2], [frame + 0xb0, frame + 0xa0]);
        assert!((f32::from_bits(turns[0][2]) + std::f32::consts::FRAC_PI_3).abs() < 1e-3);
        assert_eq!(
            calls(&e, QUATERNION_SET_MUL),
            vec![
                vec![model_of(&e, &rig, 2) + 0x10, frame + 0xb0, frame + 0x20],
                vec![model_of(&e, &rig, 1) + 0x10, frame + 0xb0, frame + 0x20],
            ]
        );
        assert_eq!(calls(&e, SCALE_QUATERNION).len(), 3);
        assert_eq!(calls(&e, X87_SQRT).len(), 3);
        assert!(calls(&e, CALCULATE_BONE_MODEL_SPACE).is_empty());
        assert!(calls(&e, SET_SLERP).is_empty());
        // The second joint turns by -60 degrees about the hinge, the first by +45 degrees
        // towards the target.
        let second = load_vec(&e, model_of(&e, &rig, 2) + 0x10);
        assert_near(second, [0.0, 0.0, -0.5, 0.866_025_4], 1e-3);
        let first = load_vec(&e, model_of(&e, &rig, 1) + 0x10);
        assert_near(first, [0.0, 0.0, 0.382_683_4, 0.923_879_5], 1e-3);
        for (offset, label) in [
            (0, LABEL_SOLVER),
            (0xc, LABEL_INIT),
            (0x10, LABEL_FIX_LENGTH),
            (0x1c, LABEL_FIX_DIRECTION),
            (0x28, LABEL_END),
        ] {
            assert_eq!(e.mem.u32(rig.records + offset), label);
        }
    }

    #[test]
    fn solve_with_both_fixes_and_twist_bones_propagates() {
        let mut e = engine();
        let rig = rig(&mut e, CHAIN);
        e.mem.set_u8(rig.setup + 0x80, 1);
        e.mem.set_u8(rig.setup + 0x81, 1);
        e.mem.set_i16(rig.setup + 6, 4);
        e.mem.set_i16(rig.setup + 8, 5);
        // The wanted cosine -0.0 is raised to the lower bound 0.5; half the second gain.
        e.mem.set_f32(rig.setup + 0x20, 0.5);
        e.mem.set_f32(rig.setup + 0x2c, 0.5);
        let flags = e.mem.u32(rig.pose + 0x1c);
        e.mem.set_u32(flags + 2 * 4, 2);
        e.mem.set_u32(flags + 4 * 4, 1);
        solve(&mut e, &rig);
        assert_eq!(e.mem.u8(rig.result), 1);

        // The second joint's flag has bit 2: the accessor runs for it at the start, twice in the
        // length fix and once for the second twist bone.
        assert_eq!(
            calls(&e, CALCULATE_BONE_MODEL_SPACE),
            vec![vec![rig.pose, 2]; 4]
        );
        assert!(calls(&e, SET_TRANSFORMED_POS).is_empty());
        assert!(calls(&e, COPY_TRANSFORM).is_empty());
        assert_eq!(calls(&e, ACOS)[0], vec![0.5f32.to_bits()]);
        let products = calls(&e, QUATERNION_PRODUCT);
        assert_eq!(products.len(), 1);
        let frame = products[0][0] - 0x90;
        assert_eq!(products[0][1..], [rig.setup + 0x50, rig.setup + 0x70]);
        assert_eq!(
            calls(&e, SET_SLERP),
            vec![vec![
                frame + 0x60,
                model_of(&e, &rig, 3) + 0x10,
                frame + 0x90,
                0.25f32.to_bits()
            ]]
        );

        // The end bone keeps its translation and takes the slerped rotation; bone 4 (its child,
        // stale local transform) got its local transform from the end bone's old transform.
        assert_eq!(load_vec(&e, model_of(&e, &rig, 3)), [2.0, 0.0, 0.0, 0.0]);
        assert_eq!(
            load_vec(&e, model_of(&e, &rig, 3) + 0x10),
            [0.0, 0.0, 0.6, 0.8]
        );
        let local = e.mem.u32(rig.pose + 4) + 4 * 0x30;
        assert_eq!(load_vec(&e, local), [1.0, 0.0, 0.0, 0.0]);
        assert_near(load_vec(&e, local + 0x10), [0.0, 0.0, 0.0, 1.0], 0.0);
        assert_near(load_vec(&e, local + 0x20), [1.0, 1.0, 1.0, 0.0], 1e-3);
        // The twist bones follow their joints: bone 4 the first joint's 45 degree turn, bone 5
        // the second joint's -60 degree turn (the current angle is acos of nearly -1, where the
        // host's RSQRTSS estimate moves the answer by a few 1e-4).
        assert_near(
            load_vec(&e, model_of(&e, &rig, 4)),
            [2.121_320_3, 2.121_320_3, 0.0, 0.0],
            2e-3,
        );
        assert_near(
            load_vec(&e, model_of(&e, &rig, 5)),
            [4.830_127, 3.366_025_4, 0.0, 0.0],
            1e-2,
        );
        for bone in [3, 4, 5] {
            assert_eq!(flags_of(&e, &rig, bone), 1, "bone {bone}");
        }
        assert_eq!(flags_of(&e, &rig, 2), 2);
        assert_eq!(e.mem.u8(rig.pose + 0x28), 0);
        assert_eq!(e.mem.u8(rig.pose + 0x29), 0);
    }
}
