//! `Havok SDK/hkaskeletonutils.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkaSkeletonUtils` (Xbox PDB name) works on a skeleton and on poses, arrays of 0x30-byte
//! transforms (translation `+0`, rotation quaternion `+0x10`, scale `+0x20`, four lanes each).
//! The fields of the skeleton the functions read are not in the Xbox PDB, so they are described
//! by offset (unconfirmed): the parent bone index of every bone as an `i16` array behind the
//! pointer at `+0xc`, the bones behind the pointer at `+0x18` (8 bytes each: name pointer with
//! the low bit masked off, a byte at `+4`), the bone count at `+0x1c` and the reference pose
//! behind the pointer at `+0x24`. SSE code computes lane by lane in `f32`; a sum of three squares
//! is `z + (y + x)` and a sum of four lanes is `(w + y) + (z + x)` the way the `SHUFPS`/`ADDPS`
//! sequences add them; `RSQRTSS`/`RSQRTPS` are the host CPU's estimates. The pose functions
//! rotate with the unrolled quaternion sequence (`rotate_twice`, shared with
//! `hkaskeletonmapperutils`).
//!
//! The functions `00cde3d0` and `00cde450` use the memory router (`00c85750`): the second
//! function keeps a byte buffer on the router's local stack (an `hkLocalArray`: data, size,
//! capacity and flags, then the saved stack pointer) and gives it back at the end.
//!
//! Not translated: none of these functions has a C++ exception frame.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkaskeletonmapperutils::{
    inverse_compose, rotate_twice, IDENTITY_QUATERNION, QUATERNION_SET_MUL, ROTATION_BIAS,
};
use crate::units::havok_sdk::hkp3axissweep::{ARRAY_GROW, ARRAY_RESERVE, MEMORY_ROUTER};
use crate::units::havok_sdk::hkpboxmotion::{add, cross, mul, splat, sub};
use crate::units::havok_sdk::hkpconvexradiusbuilder::normalising_scale;
use crate::units::havok_sdk::hkpsphereshape::{
    load_bits, load_vec, store_bits, store_vec, Vec4, HALF_F32, ONES, THREE_F32, ZERO_F32,
};
use crate::x87::rsqrt_estimate;

/// The skeleton: parent indices (`i16` array pointer), bones (pointer), bone count, reference
/// pose (pointer).
const SKELETON_PARENTS: u32 = 0xc;
const SKELETON_BONES: u32 = 0x18;
const SKELETON_BONE_COUNT: u32 = 0x1c;
const SKELETON_REFERENCE_POSE: u32 = 0x24;
/// The size of a transform.
const TRANSFORM_SIZE: i32 = 0x30;
/// `hkString::strCasecmp` (`00cb0fe0`), the comparison `findBoneWithName` uses when it is given
/// none.
const STRING_COMPARE: u32 = 0x00cb_0fe0;
/// The identity transform global `hkQsTransform::getIdentity` returns.
const IDENTITY_TRANSFORM: u32 = 0x0126_8700;
/// The vector of ones (second copy), the vector of `-1.0` (as a `float`) and the `float` the
/// rotation length is compared with.
const ONES_INTERPOLATE: u32 = 0x010e_c7f0;
const MINUS_ONE: u32 = 0x0101_2054;
const LENGTH_EPSILON: u32 = 0x0104_da4c;
/// The vectors of `3.0` and `0.5` the four-wide normalisation uses.
const THREE_VECTOR: u32 = 0x010c_71c0;
const HALF_VECTOR: u32 = 0x010c_71d0;
/// `memset(buffer, value, count)` (`00ec61c0`, cdecl).
const MEMSET: u32 = 0x00ec_61c0;
/// The quaternion normaliser (`005611c0`, `this` = the quaternion).
const NORMALIZE: u32 = 0x0056_11c0;
/// The array insertion `00cde920` (thiscall, `RET 0x10`): `this` = the `i16` array, `(allocator,
/// index, source, count)`.
const ARRAY_INSERT: u32 = 0x00cd_e920;
/// `hkaSkeletonUtils`'s own helpers: the interpolation of two transforms and the array push.
const INTERPOLATE: u32 = 0x00cd_e740;
const PUSH_BACK: u32 = 0x00cd_e940;
const TRANSFORM_LOCAL_TO_WORLD: u32 = 0x00cd_dad0;
const TRANSFORM_WORLD_TO_LOCAL: u32 = 0x00cd_dd20;

/// Sign-extends a 16-bit value.
fn sign_extend(value: u16) -> i32 {
    i32::from(value as i16)
}

/// The pointer to transform `index` of an array.
fn transform_at(base: u32, index: i32) -> u32 {
    base.wrapping_add(index.wrapping_mul(TRANSFORM_SIZE) as u32)
}

/// The parent bone index of `index`, as the 16-bit word in the skeleton's parent array.
fn parent_word(e: &Engine, skeleton: u32, index: i32) -> u16 {
    let parents = e.mem.u32(skeleton + SKELETON_PARENTS);
    e.mem
        .u16(parents.wrapping_add((index as u32).wrapping_mul(2)))
}

/// The sum of four lanes the `SHUFPS 0x4e`/`ADDPS`/`SHUFPS 0xb1`/`ADDPS` sequence leaves in lane
/// 0: `(w + y) + (z + x)`.
fn sum4(v: Vec4) -> f32 {
    (v[3] + v[1]) + (v[2] + v[0])
}

// Translated from 00cdd390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaSkeletonUtils::findBoneWithName` (Xbox PDB name; cdecl): the index of the first bone
/// whose name (the pointer with its low bit cleared) `compare(name, bone name)` answers zero
/// for, or `0xffffffff`. `compare` is `hkString::strCasecmp` (`00cb0fe0`) when zero. The index
/// is counted in 16 bits.
pub fn hka_skeleton_utils_find_bone_with_name(
    e: &mut Engine,
    skeleton: Ptr,
    name: u32,
    compare: u32,
) -> u32 {
    let skeleton = skeleton.addr();
    let compare = if compare == 0 {
        STRING_COMPARE
    } else {
        compare
    };
    let count = e.mem.i32(skeleton + SKELETON_BONE_COUNT);
    let mut index: u16 = 0;
    if count <= 0 {
        return u32::MAX;
    }
    loop {
        let bones = e.mem.u32(skeleton + SKELETON_BONES);
        let bone_name = e
            .mem
            .u32(bones.wrapping_add((sign_extend(index) as u32).wrapping_mul(8)))
            & 0xffff_fffe;
        if e.call(compare, &args![name, bone_name]).u32() == 0 {
            return u32::from(index);
        }
        index = index.wrapping_add(1);
        if sign_extend(index) >= count {
            return u32::MAX;
        }
    }
}

// Translated from 00cdd410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Marks the bones below a bone (no name in the engine map; cdecl): the first `count` bytes of
/// `buffer` are cleared (when `count` is positive), the byte of bone `count` is set, and for
/// every later bone the byte is 1 when its parent index is not negative and the parent's byte is
/// not zero, else 0. At the end the byte of bone `count` is set to `last` (the byte is also
/// set when `count + 1` is not below the bone count).
pub fn fn_00cdd410(e: &mut Engine, skeleton: Ptr, count: i32, buffer: u32, last: u8) {
    let skeleton = skeleton.addr();
    if count > 0 {
        e.call(MEMSET, &args![buffer, 0u32, count]);
    }
    let first = count.wrapping_add(1);
    let marker = buffer.wrapping_add(count as u32);
    e.mem.set_u8(marker, 1);
    let mut bone = first;
    while bone < e.mem.i32(skeleton + SKELETON_BONE_COUNT) {
        let parent = parent_word(e, skeleton, bone);
        let marked =
            parent as i16 >= 0 && e.mem.u8(buffer.wrapping_add(sign_extend(parent) as u32)) != 0;
        e.mem
            .set_u8(buffer.wrapping_add(bone as u32), u8::from(marked));
        bone += 1;
    }
    e.mem.set_u8(marker, last);
}

// Translated from 00cdd480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Interpolates `count` transform pairs with a weight each (no name in the engine map; cdecl):
/// for every `i`, `00cde740(from_a[i], from_b[i], (weights[i], 0, 0, 0), out[i])`.
pub fn fn_00cdd480(e: &mut Engine, count: u32, from_a: u32, from_b: u32, weights: u32, out: u32) {
    for i in 0..count {
        let weight = e.mem.u32(weights.wrapping_add(i.wrapping_mul(4)));
        let offset = i.wrapping_mul(0x30);
        e.with_stack(0x10, |e, vector| {
            store_bits(e, vector.addr(), [weight, 0, 0, 0]);
            e.call(
                INTERPOLATE,
                &args![
                    from_a.wrapping_add(offset),
                    from_b.wrapping_add(offset),
                    vector.addr(),
                    out.wrapping_add(offset)
                ],
            );
        });
    }
}

// Translated from 00cdd4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Interpolates `count` transform pairs with one weight (no name in the engine map; cdecl):
/// for every `i`, `00cde740(from_a[i], from_b[i], (weight, 0, 0, 0), out[i])`. The weight is
/// passed as a `float`.
pub fn fn_00cdd4f0(e: &mut Engine, count: u32, from_a: u32, from_b: u32, weight: f32, out: u32) {
    e.with_stack(0x10, |e, vector| {
        store_bits(e, vector.addr(), [weight.to_bits(), 0, 0, 0]);
        for i in 0..count {
            let offset = i.wrapping_mul(0x30);
            e.call(
                INTERPOLATE,
                &args![
                    from_a.wrapping_add(offset),
                    from_b.wrapping_add(offset),
                    vector.addr(),
                    out.wrapping_add(offset)
                ],
            );
        }
    });
}

// Translated from 00cdd550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Interpolates the transforms picked by `count` bone indices with one weight (no name in the
/// engine map; cdecl): for every `i` the offset is `indices[i] * 0x30` (the index as an `i16`)
/// and `00cde740(from_a + offset, from_b + offset, (weight, 0, 0, 0), out + offset)` is called.
pub fn fn_00cdd550(
    e: &mut Engine,
    count: u32,
    indices: u32,
    from_a: u32,
    from_b: u32,
    weight: f32,
    out: u32,
) {
    e.with_stack(0x10, |e, vector| {
        store_bits(e, vector.addr(), [weight.to_bits(), 0, 0, 0]);
        for i in 0..count {
            let index = e.mem.i16(indices.wrapping_add(i.wrapping_mul(2)));
            let offset = transform_at(0, i32::from(index));
            e.call(
                INTERPOLATE,
                &args![
                    from_a.wrapping_add(offset),
                    from_b.wrapping_add(offset),
                    vector.addr(),
                    out.wrapping_add(offset)
                ],
            );
        }
    });
}

// Translated from 00cdd600 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scale of a bone in model space (no name in the engine map; cdecl): the product of the
/// scales of the bone and of all its ancestors (each factor multiplied from the left, starting
/// from a vector of ones, stored at `out`); `index == -1` stores the ones.
pub fn fn_00cdd600(e: &mut Engine, skeleton: Ptr, transforms: u32, index: i32, out: u32) {
    let skeleton = skeleton.addr();
    let mut product = load_vec(e, ONES);
    let mut index = index;
    if index != -1 {
        let parents = e.mem.u32(skeleton + SKELETON_PARENTS);
        loop {
            let scale = load_vec(e, transform_at(transforms, index).wrapping_add(0x20));
            product = mul(scale, product);
            index = sign_extend(
                e.mem
                    .u16(parents.wrapping_add((index as u32).wrapping_mul(2))),
            );
            if index == -1 {
                break;
            }
        }
    }
    store_vec(e, out, product);
}

// Translated from 00cdd660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Normalises the rotation quaternion (`+0x10`) of `count` transforms in place (no name in the
/// engine map; cdecl). Groups of four go through `RSQRTPS` and the Newton step
/// `(3 - (x * r) * r) * (0.5 * r)` with no guard for a zero length (a zero quaternion becomes
/// NaN); the remaining `count & 3` transforms go through `RSQRTSS` and the guarded scale of the
/// scalar normaliser (zero when the squared length is zero). The squared length of a quaternion
/// is `(w + y) + (z + x)` of its squares; the `FLD`/`FSTP` copies in the code carry sums, which
/// are never signalling NaNs.
pub fn fn_00cdd660(e: &mut Engine, transforms: u32, count: i32) {
    let groups = (count.wrapping_add((count >> 31) & 3) >> 2) as u32;
    let mut cursor = transforms;
    for _ in 0..groups {
        let quaternions: [Vec4; 4] =
            std::array::from_fn(|k| load_vec(e, cursor.wrapping_add(0x10 + 0x30 * k as u32)));
        let squares = quaternions.map(|q| sum4(mul(q, q)));
        let estimates = squares.map(rsqrt_estimate);
        let three = load_vec(e, THREE_VECTOR);
        let half = load_vec(e, HALF_VECTOR);
        let scales: Vec4 = std::array::from_fn(|k| {
            let r = estimates[k];
            (three[k] - (r * squares[k]) * r) * (half[k] * r)
        });
        for (k, quaternion) in quaternions.into_iter().enumerate() {
            let at = cursor.wrapping_add(0x10 + 0x30 * k as u32);
            store_vec(e, at, mul([scales[k]; 4], quaternion));
        }
        cursor = cursor.wrapping_add(0xc0);
    }
    let mut tail = (count as u32 & 0x8000_0003) as i32;
    if tail < 0 {
        tail = (tail.wrapping_sub(1) | -4).wrapping_add(1);
    }
    if tail != 0 {
        let zero = e.mem.f32(ZERO_F32);
        let three = e.mem.f32(THREE_F32);
        let half = e.mem.f32(HALF_F32);
        let mut at = cursor.wrapping_add(0x10);
        for _ in 0..tail as u32 {
            let quaternion = load_vec(e, at);
            let scale = normalising_scale(sum4(mul(quaternion, quaternion)), zero, half, three);
            store_vec(e, at, mul([scale; 4], quaternion));
            at = at.wrapping_add(0x30);
        }
    }
}

/// The translation and rotation of a local transform in the space of its parent (the part
/// `transformLocalPoseToWorldPose` and its relatives share): `out.translation = rotate(
/// parent.rotation, local.translation) + parent.translation` (the unrolled quaternion rotation,
/// doubled) and `out.rotation = parent.rotation * local.rotation` (`00c66320`, `this` the
/// result, `(parent rotation, local rotation)`). `local` may be `out`.
fn compose_translation_and_rotation(e: &mut Engine, parent: u32, local: u32, out: u32) {
    let bias = load_vec(e, ROTATION_BIAS);
    let rotation = load_vec(e, parent + 0x10);
    let translation = load_vec(e, local);
    let moved = rotate_twice(cross(rotation, translation), translation, rotation, bias);
    let parent_translation = load_vec(e, parent);
    store_vec(e, out, add(moved, parent_translation));
    e.call(
        QUATERNION_SET_MUL,
        &args![out + 0x10, parent + 0x10, local + 0x10],
    );
}

// Translated from 00cddad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaSkeletonUtils::transformLocalPoseToWorldPose` (Xbox PDB name; cdecl): for every bone `i`
/// the parent transform is `root` when the parent index (the `i16` in `parents`) is -1, else
/// the already computed `out[parent]`; `out[i]` is the local pose composed with it: translation
/// `rotate(parent.rotation, local.translation) + parent.translation`, rotation
/// `parent.rotation * local.rotation` (`00c66320`), scale `local.scale * parent.scale`.
pub fn hka_skeleton_utils_transform_local_pose_to_world_pose(
    e: &mut Engine,
    count: i32,
    parents: u32,
    root: u32,
    local: u32,
    out: u32,
) {
    for i in 0..count {
        let parent_index = sign_extend(e.mem.u16(parents.wrapping_add((i as u32).wrapping_mul(2))));
        let parent = if parent_index == -1 {
            root
        } else {
            transform_at(out, parent_index)
        };
        let local_i = transform_at(local, i);
        let out_i = transform_at(out, i);
        compose_translation_and_rotation(e, parent, local_i, out_i);
        let scale = mul(load_vec(e, local_i + 0x20), load_vec(e, parent + 0x20));
        store_vec(e, out_i + 0x20, scale);
    }
}

// Translated from 00cddbf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The model-space transform of one bone (no name in the engine map; cdecl): `transforms[index]`
/// is copied to `out`, then for every ancestor (the `i16` parent indices from `parents`, while
/// not negative) `out` becomes the transform composed with the ancestor's: translation
/// `rotate(ancestor.rotation, out.translation) + ancestor.translation`, rotation
/// `ancestor.rotation * out.rotation` (`00c66320`), scale `ancestor.scale * out.scale`.
pub fn fn_00cddbf0(e: &mut Engine, index: i32, parents: u32, transforms: u32, out: u32) {
    let source = transform_at(transforms, index);
    for offset in [0, 0x10, 0x20] {
        let word = load_bits(e, source + offset);
        store_bits(e, out + offset, word);
    }
    let mut ancestor = sign_extend(
        e.mem
            .u16(parents.wrapping_add((index as u32).wrapping_mul(2))),
    );
    while ancestor >= 0 {
        let parent = transform_at(transforms, ancestor);
        compose_translation_and_rotation(e, parent, out, out);
        let scale = mul(load_vec(e, parent + 0x20), load_vec(e, out + 0x20));
        store_vec(e, out + 0x20, scale);
        ancestor = sign_extend(
            e.mem
                .u16(parents.wrapping_add((ancestor as u32).wrapping_mul(2))),
        );
    }
}

// Translated from 00cddcf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaSkeletonUtils::transformLocalPoseToModelPose` (Xbox PDB name; cdecl): the world-pose
/// transform (`00cddad0`) with the identity transform global (`01268700`) as the root.
pub fn hka_skeleton_utils_transform_local_pose_to_model_pose(
    e: &mut Engine,
    count: i32,
    parents: u32,
    local: u32,
    out: u32,
) {
    e.call(
        TRANSFORM_LOCAL_TO_WORLD,
        &args![count, parents, IDENTITY_TRANSFORM, local, out],
    );
}

// Translated from 00cddd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaSkeletonUtils::transformWorldPoseToLocalPose` (Xbox PDB name; cdecl): for every bone `i`
/// the parent transform is `root` when the parent index (the `i16` in `parents`) is -1, else
/// `world[parent]`; `out[i]` is `inverse(parent) * world[i]` (translation, rotation through
/// `00c66320`, scale `world.scale * (1 / parent.scale)` as in `computeMappingTransform` mode 0).
pub fn hka_skeleton_utils_transform_world_pose_to_local_pose(
    e: &mut Engine,
    count: i32,
    parents: u32,
    root: u32,
    world: u32,
    out: u32,
) {
    for i in 0..count {
        let parent_index = sign_extend(e.mem.u16(parents.wrapping_add((i as u32).wrapping_mul(2))));
        let parent = if parent_index == -1 {
            root
        } else {
            transform_at(world, parent_index)
        };
        inverse_compose(e, parent, transform_at(world, i), transform_at(out, i));
    }
}

// Translated from 00cddef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaSkeletonUtils::transformModelPoseToLocalPose` (Xbox PDB name; cdecl): the world-pose
/// to local-pose conversion (`00cddd20`) with the identity transform global (`01268700`) as the
/// root.
pub fn hka_skeleton_utils_transform_model_pose_to_local_pose(
    e: &mut Engine,
    count: i32,
    parents: u32,
    model: u32,
    out: u32,
) {
    e.call(
        TRANSFORM_WORLD_TO_LOCAL,
        &args![count, parents, IDENTITY_TRANSFORM, model, out],
    );
}

// Translated from 00cddf20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Composes one transform with an array (no name in the engine map; cdecl): for every `i`,
/// `out[i]` is `base` composed with `poses[i]`: translation `rotate(base.rotation,
/// poses.translation) + base.translation`, rotation `base.rotation * poses.rotation`
/// (`00c66320`), scale `poses.scale * base.scale`.
pub fn fn_00cddf20(e: &mut Engine, count: i32, base: u32, poses: u32, out: u32) {
    for i in 0..count {
        let pose = transform_at(poses, i);
        let out_i = transform_at(out, i);
        compose_translation_and_rotation(e, base, pose, out_i);
        let scale = mul(load_vec(e, pose + 0x20), load_vec(e, base + 0x20));
        store_vec(e, out_i + 0x20, scale);
    }
}

// Translated from 00cde010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Inverse composition of one transform with an array (no name in the engine map; cdecl): for
/// every `i`, `out[i] = inverse(base) * poses[i]` (the mode 0 of `computeMappingTransform`).
pub fn fn_00cde010(e: &mut Engine, count: i32, base: u32, poses: u32, out: u32) {
    for i in 0..count {
        inverse_compose(e, base, transform_at(poses, i), transform_at(out, i));
    }
}

// Translated from 00cde1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Puts the bones that have their flag byte set (the byte at `+4` of the bone) at the reference
/// pose translation (no name in the engine map; cdecl): for every such bone `i`, `poses[i]`'s
/// translation becomes the reference pose translation when the parent word is `0xffff`, else
/// `rotate(poses[parent].rotation, reference.translation) + poses[parent].translation`.
pub fn fn_00cde1d0(e: &mut Engine, skeleton: Ptr, poses: u32) {
    let skeleton = skeleton.addr();
    let count = e.mem.i32(skeleton + SKELETON_BONE_COUNT);
    for i in 0..count {
        let bones = e.mem.u32(skeleton + SKELETON_BONES);
        let flag = e.mem.u8(bones
            .wrapping_add((i as u32).wrapping_mul(8))
            .wrapping_add(4));
        let parent = parent_word(e, skeleton, i);
        if flag == 0 {
            continue;
        }
        let reference = e.mem.u32(skeleton + SKELETON_REFERENCE_POSE);
        let reference = transform_at(reference, i);
        let out_i = transform_at(poses, i);
        if parent == 0xffff {
            let translation = load_bits(e, reference);
            store_bits(e, out_i, translation);
        } else {
            let parent_pose = transform_at(poses, sign_extend(parent));
            let bias = load_vec(e, ROTATION_BIAS);
            let translation = load_vec(e, reference);
            let rotation = load_vec(e, parent_pose + 0x10);
            let moved = rotate_twice(cross(rotation, translation), translation, rotation, bias);
            store_vec(e, out_i, moved);
            let sum = add(load_vec(e, out_i), load_vec(e, parent_pose));
            store_vec(e, out_i, sum);
        }
    }
}

// Translated from 00cde2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like `00cde1d0`, but reading the translation from another pose and writing a third (no name
/// in the engine map; cdecl): for every bone with its flag byte set, `out[i]`'s translation is
/// `poses[i]`'s translation when the parent word is `0xffff`, else
/// `rotate(out[parent].rotation, poses[i].translation) + out[parent].translation`.
pub fn fn_00cde2d0(e: &mut Engine, skeleton: Ptr, poses: u32, out: u32) {
    let skeleton = skeleton.addr();
    let count = e.mem.i32(skeleton + SKELETON_BONE_COUNT);
    for i in 0..count {
        let bones = e.mem.u32(skeleton + SKELETON_BONES);
        let flag = e.mem.u8(bones
            .wrapping_add((i as u32).wrapping_mul(8))
            .wrapping_add(4));
        let parent = parent_word(e, skeleton, i);
        if flag == 0 {
            continue;
        }
        let pose = transform_at(poses, i);
        let out_i = transform_at(out, i);
        if parent == 0xffff {
            let translation = load_bits(e, pose);
            store_bits(e, out_i, translation);
        } else {
            let parent_out = transform_at(out, sign_extend(parent));
            let bias = load_vec(e, ROTATION_BIAS);
            let translation = load_vec(e, pose);
            let rotation = load_vec(e, parent_out + 0x10);
            let moved = rotate_twice(cross(rotation, translation), translation, rotation, bias);
            let parent_translation = load_vec(e, parent_out);
            store_vec(e, out_i, add(moved, parent_translation));
        }
    }
}

// Translated from 00cde3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaSkeletonUtils::getBoneChain` (Xbox PDB name; cdecl, the code is written as `void
/// getBoneChain(bool* found, skeleton, stop, start, hkArray<i16>* chain)`): pushes the bone
/// words onto `chain` (through `00cde940`, the allocator being the field `+0x10` of the memory
/// router) from `start` up its parents while the bone (a 16-bit signed value) is positive and
/// is not `stop`; then pushes `stop` as well and stores in `found` whether the bone the walk
/// ended on is not negative. `stop` and `start` are the raw words of the arguments (only the
/// low 16 bits are used; the push reads them from memory, as the exe passes the address of the
/// argument slots).
/// Answers `found` (`EAX` at the `RET`; callers test the flag through it, 00ce37ca).
pub fn hka_skeleton_utils_get_bone_chain(
    e: &mut Engine,
    found: u32,
    skeleton: Ptr,
    stop: u32,
    start: u32,
    chain: u32,
) -> u32 {
    let skeleton = skeleton.addr();
    e.with_stack(8, |e, slots| {
        let start_slot = slots.addr();
        let stop_slot = slots.addr() + 4;
        e.mem.set_u32(stop_slot, stop);
        let mut bone = start & 0xffff;
        e.mem.set_u32(start_slot, bone);
        if (bone as u16 as i16) > 0 {
            loop {
                if bone as u16 == stop as u16 {
                    break;
                }
                let router = e.call(MEMORY_ROUTER, &args![]).u32();
                let allocator = e.mem.u32(router + 0x10);
                e.call(PUSH_BACK, &args![chain, allocator, 0u32, start_slot]);
                bone = u32::from(parent_word(e, skeleton, sign_extend(bone as u16)));
                e.mem.set_u32(start_slot, bone);
                if (bone as u16 as i16) <= 0 {
                    break;
                }
            }
        }
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        e.call(PUSH_BACK, &args![chain, allocator, 0u32, stop_slot]);
        e.mem.set_u8(found, u8::from(bone as u16 as i16 >= 0));
    });
    found
}

/// `hkArray<i16>::pushBack`: the room for one more element is made through `00c90940` when the
/// size equals the capacity (the allocator is the field `+0x10` of the memory router), then the
/// word is stored and the size incremented.
fn push_back_word(e: &mut Engine, array: u32, value: u16) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_GROW, &args![allocator, array, 2u32]);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    e.mem
        .set_u16(data.wrapping_add(size.wrapping_mul(2)), value);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

// Translated from 00cde450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaSkeletonUtils::getDescendants` (Xbox PDB name; cdecl): pushes onto the `hkArray<i16>`
/// `out` the bones below `bone` (and `bone` itself first when `include_bone`). A byte per bone
/// is kept in an `hkLocalArray` on the memory router's local stack (the router's stack pointer,
/// `[router]`, moves up by the bone count rounded up to 128 bytes and goes back at the end):
/// the buffer is cleared, `bone`'s byte set, and every later bone whose parent word is not
/// `0xffff` and whose parent's byte is set gets its byte set and is pushed. If the local array
/// had to leave the router's stack (the capacity flag's top bit clear) the memory is given back
/// through the allocator's slot `0x10` with `(data, capacity)`.
pub fn hka_skeleton_utils_get_descendants(
    e: &mut Engine,
    skeleton: Ptr,
    bone: u32,
    out: u32,
    include_bone: bool,
) {
    let skeleton = skeleton.addr();
    e.with_stack(0x10, |e, local| {
        // The local array: data, size, capacity and flags; then the saved stack pointer.
        let local = local.addr();
        let (data_at, size_at, flags_at, saved_at) = (local, local + 4, local + 8, local + 12);
        e.mem.set_u32(data_at, 0);
        e.mem.set_u32(size_at, 0);
        e.mem.set_u32(flags_at, 0x8000_0000);
        let count = e.mem.i32(skeleton + SKELETON_BONE_COUNT);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let saved = e.mem.u32(router);
        let reserved = (count as u32).wrapping_add(0x7f) & 0xffff_ff80;
        e.mem.set_u32(router, saved.wrapping_add(reserved));
        e.mem.set_u32(flags_at, count as u32 | 0x8000_0000);
        let count = e.mem.i32(skeleton + SKELETON_BONE_COUNT);
        e.mem.set_u32(data_at, saved);
        e.mem.set_u32(saved_at, saved);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = (e.mem.u32(flags_at) & 0x3fff_ffff) as i32;
        if capacity < count {
            let doubled = capacity.wrapping_add(capacity);
            let wanted = if count < doubled { doubled } else { count };
            e.call(ARRAY_RESERVE, &args![allocator, local, wanted, 1u32]);
        }
        let size = e.mem.i32(size_at);
        let data = e.mem.u32(data_at);
        let missing = count.wrapping_sub(size);
        if missing > 0 {
            e.call(
                MEMSET,
                &args![data.wrapping_add(size as u32), 0u32, missing],
            );
        }
        e.mem.set_u32(size_at, count as u32);
        let mut index = bone;
        if include_bone {
            push_back_word(e, out, index as u16);
        }
        let marks = e.mem.u32(data_at);
        e.mem
            .set_u8(marks.wrapping_add(sign_extend(index as u16) as u32), 1);
        index = u32::from((index as u16).wrapping_add(1));
        let mut current = sign_extend(index as u16);
        let mut marks = e.mem.u32(data_at);
        if current < e.mem.i32(skeleton + SKELETON_BONE_COUNT) {
            loop {
                let parent = parent_word(e, skeleton, current);
                if parent != 0xffff && e.mem.u8(marks.wrapping_add(sign_extend(parent) as u32)) != 0
                {
                    e.mem.set_u8(marks.wrapping_add(current as u32), 1);
                    push_back_word(e, out, index as u16);
                    marks = e.mem.u32(data_at);
                }
                index = index.wrapping_add(1);
                current = sign_extend(index as u16);
                if current >= e.mem.i32(skeleton + SKELETON_BONE_COUNT) {
                    break;
                }
            }
        }
        if saved == marks {
            e.mem.set_u32(size_at, 0);
        }
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        e.mem.set_u32(router, saved);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let flags = e.mem.u32(flags_at);
        e.mem.set_u32(size_at, 0);
        if (flags as i32) >= 0 {
            let data = e.mem.u32(data_at);
            e.vcall(allocator, 0x10, &args![data, flags & 0x3fff_ffff]);
        }
    });
}

// Translated from 00cde740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Interpolates two transforms (no name in the engine map; cdecl): with `t` the first lane of
/// the vector at `weight`, `out.translation = (1 - t) * from.translation + to.translation * t`
/// and `out.scale = (1 - t) * from.scale + to.scale * t`. The rotation is
/// `(1 - t) * from.rotation + t * to.rotation'`, where `to.rotation'` is `to.rotation` negated
/// when `0.0 > dot(from.rotation, to.rotation)` (the four-lane sum `(w + y) + (z + x)`). When
/// the squared length of the result rotation (the same sum) is below the `float` at `0104da4c`
/// the rotation is set to the identity quaternion from `010c71b0`, else it is normalised by
/// `005611c0` (`this` = the rotation).
pub fn fn_00cde740(e: &mut Engine, from: u32, to: u32, weight: u32, out: u32) {
    let t = splat(load_vec(e, weight), 0);
    let ones = load_vec(e, ONES_INTERPOLATE);
    let from_translation = load_vec(e, from);
    let to_translation = load_vec(e, to);
    let translation = add(mul(sub(ones, t), from_translation), mul(to_translation, t));
    store_vec(e, out, translation);
    let from_rotation = load_vec(e, from + 0x10);
    let to_rotation = load_vec(e, to + 0x10);
    let dot = sum4(mul(from_rotation, to_rotation));
    let minus_one = e.mem.f32(MINUS_ONE);
    let to_rotation = if 0.0 > dot {
        mul([minus_one; 4], to_rotation)
    } else {
        to_rotation
    };
    let t = splat(load_vec(e, weight), 0);
    let ones = load_vec(e, ONES_INTERPOLATE);
    let rotation = add(mul(sub(ones, t), from_rotation), mul(t, to_rotation));
    store_vec(e, out + 0x10, rotation);
    let t = splat(load_vec(e, weight), 0);
    let ones = load_vec(e, ONES_INTERPOLATE);
    let scale = add(
        mul(sub(ones, t), load_vec(e, from + 0x20)),
        mul(load_vec(e, to + 0x20), t),
    );
    store_vec(e, out + 0x20, scale);
    let result = load_vec(e, out + 0x10);
    let epsilon = e.mem.f32(LENGTH_EPSILON);
    if epsilon > sum4(mul(result, result)) {
        let identity = load_bits(e, IDENTITY_QUATERNION);
        store_bits(e, out + 0x10, identity);
    } else {
        e.call(NORMALIZE, &args![out + 0x10]);
    }
}

// Translated from 00cde940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Array push helper (no name in the engine map; thiscall, `RET 0xc`): `this` is the array,
/// `(allocator, index, source)`; calls `00cde920` with the same `this` and one element (count
/// 1).
pub fn fn_00cde940(e: &mut Engine, this: Ptr, allocator: u32, index: u32, source: u32) {
    e.call(ARRAY_INSERT, &args![this, allocator, index, source, 1u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00cd_d390,
            hka_skeleton_utils_find_bone_with_name(Ptr, u32, u32) -> u32
        ),
        entry!(0x00cd_d410, fn_00cdd410(Ptr, i32, u32, u8)),
        entry!(0x00cd_d480, fn_00cdd480(u32, u32, u32, u32, u32)),
        entry!(0x00cd_d4f0, fn_00cdd4f0(u32, u32, u32, f32, u32)),
        entry!(0x00cd_d550, fn_00cdd550(u32, u32, u32, u32, f32, u32)),
        entry!(0x00cd_d600, fn_00cdd600(Ptr, u32, i32, u32)),
        entry!(0x00cd_d660, fn_00cdd660(u32, i32)),
        entry!(
            0x00cd_dad0,
            hka_skeleton_utils_transform_local_pose_to_world_pose(i32, u32, u32, u32, u32)
        ),
        entry!(0x00cd_dbf0, fn_00cddbf0(i32, u32, u32, u32)),
        entry!(
            0x00cd_dcf0,
            hka_skeleton_utils_transform_local_pose_to_model_pose(i32, u32, u32, u32)
        ),
        entry!(
            0x00cd_dd20,
            hka_skeleton_utils_transform_world_pose_to_local_pose(i32, u32, u32, u32, u32)
        ),
        entry!(
            0x00cd_def0,
            hka_skeleton_utils_transform_model_pose_to_local_pose(i32, u32, u32, u32)
        ),
        entry!(0x00cd_df20, fn_00cddf20(i32, u32, u32, u32)),
        entry!(0x00cd_e010, fn_00cde010(i32, u32, u32, u32)),
        entry!(0x00cd_e1d0, fn_00cde1d0(Ptr, u32)),
        entry!(0x00cd_e2d0, fn_00cde2d0(Ptr, u32, u32)),
        entry!(
            0x00cd_e3d0,
            hka_skeleton_utils_get_bone_chain(u32, Ptr, u32, u32, u32) -> u32
        ),
        entry!(
            0x00cd_e450,
            hka_skeleton_utils_get_descendants(Ptr, u32, u32, bool)
        ),
        entry!(0x00cd_e740, fn_00cde740(u32, u32, u32, u32)),
        entry!(0x00cd_e940, fn_00cde940(Ptr, u32, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkaskeletonmapperutils::{CONJUGATE_MASK, XYZ_MASK};
    use crate::units::havok_sdk::hkpsphereshape::SIGN_MASK;

    const ROTATION: Vec4 = [0.0, 0.0, 0.6, 0.8];
    const IDENTITY: Vec4 = [0.0, 0.0, 0.0, 1.0];

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        store_vec(&mut e, ONES, [1.0; 4]);
        store_vec(&mut e, ONES_INTERPOLATE, [1.0; 4]);
        e.mem.set_f32(MINUS_ONE, -1.0);
        e.mem.set_f32(LENGTH_EPSILON, f32::from_bits(0x3400_0000));
        store_vec(&mut e, THREE_VECTOR, [3.0; 4]);
        store_vec(&mut e, HALF_VECTOR, [0.5; 4]);
        store_vec(&mut e, ROTATION_BIAS, [-0.5; 4]);
        store_bits(&mut e, IDENTITY_QUATERNION, [0, 0, 0, 0x3f80_0000]);
        e.mem.set_f32(ZERO_F32, 0.0);
        e.mem.set_f32(HALF_F32, 0.5);
        e.mem.set_f32(THREE_F32, 3.0);
        store_bits(&mut e, SIGN_MASK, [0x8000_0000; 4]);
        store_bits(
            &mut e,
            CONJUGATE_MASK,
            [0x8000_0000, 0x8000_0000, 0x8000_0000, 0],
        );
        store_bits(&mut e, XYZ_MASK, [u32::MAX, u32::MAX, u32::MAX, 0]);
        e.call_log = Some(vec![]);
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

    /// `00c66320` as a double: the result (`this`) becomes the first operand.
    fn install_set_mul(e: &mut Engine) {
        e.register_double(QUATERNION_SET_MUL, |e, a| {
            let left = load_vec(e, a[1]);
            store_vec(e, a[0], left);
            Ret::default()
        });
    }

    fn put_transform(e: &mut Engine, at: u32, translation: Vec4, rotation: Vec4, scale: Vec4) {
        store_vec(e, at, translation);
        store_vec(e, at + 0x10, rotation);
        store_vec(e, at + 0x20, scale);
    }

    fn get_transform(e: &Engine, at: u32) -> [Vec4; 3] {
        [
            load_vec(e, at),
            load_vec(e, at + 0x10),
            load_vec(e, at + 0x20),
        ]
    }

    fn assert_close(got: Vec4, want: Vec4, tolerance: f32) {
        for lane in 0..4 {
            assert!(
                (got[lane] - want[lane]).abs() <= tolerance,
                "lane {lane}: {got:?} against {want:?}"
            );
        }
    }

    /// A skeleton with the parent words, the flag byte of every bone and `count` bones.
    fn make_skeleton(e: &mut Engine, parents: &[u16], flags: &[u8]) -> u32 {
        let skeleton = e.mem.alloc(0x30);
        let parent_array = e.mem.alloc(2 * parents.len() as u32);
        let bones = e.mem.alloc(8 * parents.len() as u32);
        for (i, parent) in parents.iter().enumerate() {
            e.mem.set_u16(parent_array + 2 * i as u32, *parent);
            e.mem.set_u8(bones + 8 * i as u32 + 4, flags[i]);
        }
        e.mem.set_u32(skeleton + SKELETON_PARENTS, parent_array);
        e.mem.set_u32(skeleton + SKELETON_BONES, bones);
        e.mem
            .set_u32(skeleton + SKELETON_BONE_COUNT, parents.len() as u32);
        skeleton
    }

    /// An `hkArray<i16>` with room for `capacity` words.
    fn make_array(e: &mut Engine, capacity: u32) -> u32 {
        let array = e.mem.alloc(12);
        let data = e.mem.alloc(2 * capacity);
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 8, capacity | 0x8000_0000);
        array
    }

    fn array_words(e: &Engine, array: u32) -> Vec<u16> {
        let data = e.mem.u32(array);
        (0..e.mem.u32(array + 4))
            .map(|i| e.mem.u16(data + 2 * i))
            .collect()
    }

    /// The memory router (`00c85750`) as a double: its block holds the local stack pointer
    /// (`+0`, at `stack`) and the allocator (`+0x10`).
    fn install_router(e: &mut Engine, stack: u32, allocator: u32) -> u32 {
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router, stack);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        router
    }

    /// `00cde920` as a double: the word at `source` is appended to the array in `this`.
    fn install_array_insert(e: &mut Engine) {
        e.register_double(ARRAY_INSERT, |e, a| {
            let (array, source) = (a[0], a[3]);
            assert_eq!((a[2], a[4]), (0, 1));
            let value = e.mem.u16(source);
            let data = e.mem.u32(array);
            let size = e.mem.u32(array + 4);
            e.mem.set_u16(data + 2 * size, value);
            e.mem.set_u32(array + 4, size + 1);
            Ret::default()
        });
    }

    #[test]
    fn find_bone_with_name_compares_the_names_without_their_low_bit() {
        let mut e = engine();
        let skeleton = make_skeleton(&mut e, &[0xffff, 0, 1], &[0; 3]);
        let bones = e.mem.u32(skeleton + SKELETON_BONES);
        for (i, name) in [0x0500_0000u32, 0x0500_0100 | 1, 0x0500_0200]
            .into_iter()
            .enumerate()
        {
            e.mem.set_u32(bones + 8 * i as u32, name);
        }
        let compare = 0x0900_0100;
        e.register_double(compare, |_, a| Ret {
            eax: u32::from(a[1] != 0x0500_0100),
            ..Ret::default()
        });
        let found = e
            .call(0x00cd_d390, &args![skeleton, 0x0600_0000u32, compare])
            .u32();
        assert_eq!(found, 1);
        let seen = calls(&e, compare);
        assert_eq!(
            seen,
            vec![
                vec![0x0600_0000, 0x0500_0000],
                vec![0x0600_0000, 0x0500_0100]
            ]
        );
        // The first bone matches: the index is 0.
        e.register_double(compare, |_, _| Ret::default());
        assert_eq!(
            e.call(0x00cd_d390, &args![skeleton, 0x0600_0000u32, compare])
                .u32(),
            0
        );
        // No bone matches.
        e.register_double(compare, |_, _| Ret {
            eax: 7,
            ..Ret::default()
        });
        assert_eq!(
            e.call(0x00cd_d390, &args![skeleton, 0x0600_0000u32, compare])
                .u32(),
            u32::MAX
        );
        // Without a bone nothing is compared.
        e.mem.set_u32(skeleton + SKELETON_BONE_COUNT, 0);
        assert_eq!(
            e.call(0x00cd_d390, &args![skeleton, 0x0600_0000u32, compare])
                .u32(),
            u32::MAX
        );
    }

    #[test]
    fn find_bone_with_name_defaults_to_the_case_insensitive_compare() {
        let mut e = engine();
        let skeleton = make_skeleton(&mut e, &[0xffff, 0], &[0; 2]);
        let bones = e.mem.u32(skeleton + SKELETON_BONES);
        e.mem.set_u32(bones, 0x0500_0000);
        e.mem.set_u32(bones + 8, 0x0500_0100);
        e.register_double(STRING_COMPARE, |_, a| Ret {
            eax: u32::from(a[1] != 0x0500_0100),
            ..Ret::default()
        });
        let found = e
            .call(0x00cd_d390, &args![skeleton, 0x0600_0000u32, 0u32])
            .u32();
        assert_eq!(found, 1);
        assert_eq!(
            calls(&e, STRING_COMPARE),
            vec![
                vec![0x0600_0000, 0x0500_0000],
                vec![0x0600_0000, 0x0500_0100]
            ]
        );
    }

    #[test]
    fn fn_00cdd410_marks_the_bones_below_a_bone() {
        let mut e = engine();
        let skeleton = make_skeleton(&mut e, &[0xffff, 0, 1, 0, 2], &[0; 5]);
        let buffer = e.mem.alloc(16);
        for i in 0..16 {
            e.mem.set_u8(buffer + i, 0xaa);
        }
        e.call(0x00cd_d410, &args![skeleton, 1u32, buffer, 0u32]);
        let bytes: Vec<u8> = (0..7).map(|i| e.mem.u8(buffer + i)).collect();
        assert_eq!(bytes, vec![0, 0, 1, 0, 1, 0xaa, 0xaa]);
        assert_eq!(calls(&e, MEMSET), vec![vec![buffer, 0, 1]]);
        // A count of zero clears nothing; the byte of bone 0 is set and then given the flag.
        e.call(0x00cd_d410, &args![skeleton, 0u32, buffer, 1u32]);
        let bytes: Vec<u8> = (0..5).map(|i| e.mem.u8(buffer + i)).collect();
        assert_eq!(bytes, vec![1, 1, 1, 1, 1]);
        assert_eq!(calls(&e, MEMSET).len(), 1);
        // The last bone: the loop is not entered and the byte becomes the flag.
        e.call(0x00cd_d410, &args![skeleton, 4u32, buffer, 0u32]);
        let bytes: Vec<u8> = (0..6).map(|i| e.mem.u8(buffer + i)).collect();
        assert_eq!(bytes, vec![0, 0, 0, 0, 0, 0xaa]);
    }

    /// The interpolation `00cde740` as a double that writes what it was given into `out`:
    /// the first lane of the weight vector, the two sources and the weight vector's address.
    fn install_interpolation_record(e: &mut Engine) {
        e.register_double(INTERPOLATE, |e, a| {
            let weight = load_bits(e, a[2]);
            assert_eq!(&weight[1..], &[0, 0, 0]);
            store_bits(e, a[3], [weight[0], a[0], a[1], 0]);
            Ret::default()
        });
    }

    #[test]
    fn fn_00cdd480_interpolates_each_pair_with_its_weight() {
        let mut e = engine();
        install_interpolation_record(&mut e);
        let weights = e.mem.alloc(8);
        e.mem.set_f32(weights, 0.25);
        e.mem.set_f32(weights + 4, 0.75);
        let (a, b, out) = (0x0500_0000u32, 0x0500_1000u32, e.mem.alloc(0x60));
        e.call(0x00cd_d480, &args![2u32, a, b, weights, out]);
        assert_eq!(load_bits(&e, out), [0.25f32.to_bits(), a, b, 0]);
        assert_eq!(
            load_bits(&e, out + 0x30),
            [0.75f32.to_bits(), a + 0x30, b + 0x30, 0]
        );
        assert_eq!(calls(&e, INTERPOLATE).len(), 2);
        e.call(0x00cd_d480, &args![0u32, a, b, weights, out]);
        assert_eq!(calls(&e, INTERPOLATE).len(), 2);
    }

    #[test]
    fn fn_00cdd4f0_interpolates_each_pair_with_one_weight() {
        let mut e = engine();
        install_interpolation_record(&mut e);
        let (a, b, out) = (0x0500_0000u32, 0x0500_1000u32, e.mem.alloc(0x60));
        e.call(0x00cd_d4f0, &args![2u32, a, b, 0.5f32, out]);
        assert_eq!(load_bits(&e, out), [0.5f32.to_bits(), a, b, 0]);
        assert_eq!(
            load_bits(&e, out + 0x30),
            [0.5f32.to_bits(), a + 0x30, b + 0x30, 0]
        );
        e.call(0x00cd_d4f0, &args![0u32, a, b, 0.5f32, out]);
        assert_eq!(calls(&e, INTERPOLATE).len(), 2);
    }

    #[test]
    fn fn_00cdd550_interpolates_the_transforms_picked_by_index() {
        let mut e = engine();
        install_interpolation_record(&mut e);
        let indices = e.mem.alloc(4);
        e.mem.set_i16(indices, 2);
        e.mem.set_i16(indices + 2, 0);
        let (a, b, out) = (0x0500_0000u32, 0x0500_1000u32, e.mem.alloc(0x90));
        e.call(0x00cd_d550, &args![2u32, indices, a, b, 0.125f32, out]);
        assert_eq!(
            load_bits(&e, out + 0x60),
            [0.125f32.to_bits(), a + 0x60, b + 0x60, 0]
        );
        assert_eq!(load_bits(&e, out), [0.125f32.to_bits(), a, b, 0]);
        e.call(0x00cd_d550, &args![0u32, indices, a, b, 0.125f32, out]);
        assert_eq!(calls(&e, INTERPOLATE).len(), 2);
    }

    #[test]
    fn fn_00cdd600_multiplies_the_scales_up_the_parents() {
        let mut e = engine();
        let skeleton = make_skeleton(&mut e, &[0xffff, 0, 1], &[0; 3]);
        let transforms = e.mem.alloc(0x90);
        let scales = [
            [2.0, 3.0, 4.0, 0.0],
            [1.0, 2.0, 1.0, 5.0],
            [3.0, 3.0, 3.0, 1.0],
        ];
        for (i, scale) in scales.into_iter().enumerate() {
            store_vec(&mut e, transforms + 0x30 * i as u32 + 0x20, scale);
        }
        let out = e.mem.alloc(16);
        e.call(0x00cd_d600, &args![skeleton, transforms, 2i32, out]);
        assert_eq!(load_vec(&e, out), [6.0, 18.0, 12.0, 0.0]);
        e.call(0x00cd_d600, &args![skeleton, transforms, 1i32, out]);
        assert_eq!(load_vec(&e, out), [2.0, 6.0, 4.0, 0.0]);
        e.call(0x00cd_d600, &args![skeleton, transforms, -1i32, out]);
        assert_eq!(load_vec(&e, out), [1.0; 4]);
    }

    #[test]
    fn fn_00cdd660_normalises_the_rotations_in_groups_of_four_and_a_tail() {
        let mut e = engine();
        let transforms = e.mem.alloc(0x30 * 6);
        let rotations = [
            [0.0, 0.0, 0.0, 2.0],
            [0.0, 3.0, 0.0, 4.0],
            [0.0, 0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0, 0.0],
            [2.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ];
        for (i, rotation) in rotations.into_iter().enumerate() {
            let at = transforms + 0x30 * i as u32;
            put_transform(&mut e, at, [1.0, 2.0, 3.0, 4.0], rotation, [5.0; 4]);
        }
        e.call(0x00cd_d660, &args![transforms, 6i32]);
        fn rotation_of(e: &Engine, transforms: u32, i: u32) -> Vec4 {
            load_vec(e, transforms + 0x30 * i + 0x10)
        }
        assert_close(rotation_of(&e, transforms, 0), [0.0, 0.0, 0.0, 1.0], 1e-3);
        assert_close(rotation_of(&e, transforms, 1), [0.0, 0.6, 0.0, 0.8], 1e-3);
        // A zero length in a group of four is not guarded: its reciprocal root is infinite.
        assert!(rotation_of(&e, transforms, 2)
            .iter()
            .all(|lane| lane.is_nan()));
        assert_close(rotation_of(&e, transforms, 3), [1.0, 0.0, 0.0, 0.0], 1e-3);
        // The tail goes through the guarded scalar step.
        assert_close(rotation_of(&e, transforms, 4), [1.0, 0.0, 0.0, 0.0], 1e-3);
        assert_eq!(rotation_of(&e, transforms, 5), [0.0; 4]);
        for i in 0..6 {
            let at = transforms + 0x30 * i;
            assert_eq!(load_vec(&e, at), [1.0, 2.0, 3.0, 4.0]);
            assert_eq!(load_vec(&e, at + 0x20), [5.0; 4]);
        }
        // Nothing to do for a count of zero.
        e.call(0x00cd_d660, &args![transforms, 0i32]);
        assert_eq!(rotation_of(&e, transforms, 5), [0.0; 4]);
    }

    #[test]
    fn local_pose_to_world_pose_composes_with_the_parent_or_the_root() {
        let mut e = engine();
        install_set_mul(&mut e);
        let parents = e.mem.alloc(4);
        e.mem.set_i16(parents, -1);
        e.mem.set_i16(parents + 2, 0);
        let (root, local, out) = (e.mem.alloc(0x30), e.mem.alloc(0x60), e.mem.alloc(0x60));
        put_transform(&mut e, root, [10.0, 0.0, 0.0, 0.0], ROTATION, [2.0; 4]);
        put_transform(
            &mut e,
            local,
            [1.0, 2.0, 3.0, 0.0],
            IDENTITY,
            [3.0, 1.0, 1.0, 1.0],
        );
        put_transform(
            &mut e,
            local + 0x30,
            [1.0, 0.0, 0.0, 0.0],
            IDENTITY,
            [1.0, 2.0, 3.0, 4.0],
        );
        e.call(0x00cd_dad0, &args![2i32, parents, root, local, out]);
        let first = get_transform(&e, out);
        // Lane 3 holds `dot * q.w` (1.8 * 0.8, doubled): the unrolled rotation does not clear it.
        assert_close(first[0], [8.36, 1.52, 3.0, 2.88], 1e-5);
        assert_eq!(first[1], ROTATION);
        assert_eq!(first[2], [6.0, 2.0, 2.0, 2.0]);
        let second = get_transform(&e, out + 0x30);
        assert_close(second[0], [8.64, 2.48, 3.0, 2.88], 1e-5);
        assert_eq!(second[2], [6.0, 4.0, 6.0, 8.0]);
        assert_eq!(
            calls(&e, QUATERNION_SET_MUL),
            vec![
                vec![out + 0x10, root + 0x10, local + 0x10],
                vec![out + 0x40, out + 0x10, local + 0x40],
            ]
        );
        // No bone, no work.
        e.call(0x00cd_dad0, &args![0i32, parents, root, local, out]);
        assert_eq!(calls(&e, QUATERNION_SET_MUL).len(), 2);
    }

    #[test]
    fn fn_00cddbf0_composes_a_bone_with_all_its_ancestors() {
        let mut e = engine();
        install_set_mul(&mut e);
        let parents = e.mem.alloc(6);
        for (i, parent) in [-1i16, 0, 1].into_iter().enumerate() {
            e.mem.set_i16(parents + 2 * i as u32, parent);
        }
        let (transforms, out) = (e.mem.alloc(0x90), e.mem.alloc(0x30));
        put_transform(&mut e, transforms, [0.0, 0.0, 3.0, 0.0], IDENTITY, [2.0; 4]);
        put_transform(
            &mut e,
            transforms + 0x30,
            [0.0, 2.0, 0.0, 0.0],
            IDENTITY,
            [3.0; 4],
        );
        put_transform(
            &mut e,
            transforms + 0x60,
            [1.0, 0.0, 0.0, 0.0],
            ROTATION,
            [1.0, 2.0, 3.0, 4.0],
        );
        e.call(0x00cd_dbf0, &args![2i32, parents, transforms, out]);
        let result = get_transform(&e, out);
        assert_close(result[0], [1.0, 2.0, 3.0, 0.0], 1e-6);
        // The doubled `setMul` leaves the first operand: the rotation of the last ancestor.
        assert_eq!(result[1], IDENTITY);
        assert_eq!(result[2], [6.0, 12.0, 18.0, 24.0]);
        assert_eq!(
            calls(&e, QUATERNION_SET_MUL),
            vec![
                vec![out + 0x10, transforms + 0x40, out + 0x10],
                vec![out + 0x10, transforms + 0x10, out + 0x10],
            ]
        );
        // A bone without a parent is only copied.
        e.call(0x00cd_dbf0, &args![0i32, parents, transforms, out]);
        assert_eq!(get_transform(&e, out), get_transform(&e, transforms));
        assert_eq!(calls(&e, QUATERNION_SET_MUL).len(), 2);
    }

    #[test]
    fn model_pose_conversions_use_the_identity_transform_as_the_root() {
        let mut e = engine();
        install_set_mul(&mut e);
        put_transform(&mut e, IDENTITY_TRANSFORM, [0.0; 4], IDENTITY, [1.0; 4]);
        let parents = e.mem.alloc(2);
        e.mem.set_i16(parents, -1);
        let (pose, out) = (e.mem.alloc(0x30), e.mem.alloc(0x30));
        put_transform(
            &mut e,
            pose,
            [1.0, 2.0, 3.0, 0.0],
            IDENTITY,
            [2.0, 4.0, 8.0, 0.0],
        );
        e.call(0x00cd_dcf0, &args![1i32, parents, pose, out]);
        assert_eq!(
            calls(&e, TRANSFORM_LOCAL_TO_WORLD),
            vec![vec![1, parents, IDENTITY_TRANSFORM, pose, out]]
        );
        let result = get_transform(&e, out);
        assert_eq!(result[0], [1.0, 2.0, 3.0, 0.0]);
        assert_eq!(result[2], [2.0, 4.0, 8.0, 0.0]);
        let back = e.mem.alloc(0x30);
        e.call(0x00cd_def0, &args![1i32, parents, pose, back]);
        assert_eq!(
            calls(&e, TRANSFORM_WORLD_TO_LOCAL),
            vec![vec![1, parents, IDENTITY_TRANSFORM, pose, back]]
        );
        let result = get_transform(&e, back);
        assert_eq!(result[0], [1.0, 2.0, 3.0, 0.0]);
        assert_close(result[2], [2.0, 4.0, 8.0, 0.0], 1e-4);
    }

    #[test]
    fn world_pose_to_local_pose_removes_the_parent() {
        let mut e = engine();
        install_set_mul(&mut e);
        let parents = e.mem.alloc(4);
        e.mem.set_i16(parents, -1);
        e.mem.set_i16(parents + 2, 0);
        let (root, world, out) = (e.mem.alloc(0x30), e.mem.alloc(0x60), e.mem.alloc(0x60));
        put_transform(&mut e, root, [0.0; 4], IDENTITY, [1.0; 4]);
        put_transform(
            &mut e,
            world,
            [1.0, 2.0, 3.0, 0.0],
            IDENTITY,
            [2.0, 2.0, 2.0, 0.0],
        );
        put_transform(
            &mut e,
            world + 0x30,
            [4.0, 6.0, 8.0, 0.0],
            IDENTITY,
            [4.0, 2.0, 8.0, 0.0],
        );
        e.call(0x00cd_dd20, &args![2i32, parents, root, world, out]);
        let first = get_transform(&e, out);
        assert_eq!(first[0], [1.0, 2.0, 3.0, 0.0]);
        assert_close(first[2], [2.0, 2.0, 2.0, 0.0], 1e-4);
        let second = get_transform(&e, out + 0x30);
        assert_eq!(second[0], [3.0, 4.0, 5.0, 0.0]);
        assert_eq!(second[1], [-0.0, -0.0, -0.0, 1.0]);
        assert_close(second[2], [2.0, 1.0, 4.0, 0.0], 1e-4);
        let sets = calls(&e, QUATERNION_SET_MUL);
        assert_eq!(sets.len(), 2);
        assert_eq!((sets[1][0], sets[1][2]), (out + 0x40, world + 0x40));
        e.call(0x00cd_dd20, &args![0i32, parents, root, world, out]);
        assert_eq!(calls(&e, QUATERNION_SET_MUL).len(), 2);
    }

    #[test]
    fn fn_00cddf20_composes_one_transform_with_each_pose() {
        let mut e = engine();
        install_set_mul(&mut e);
        let (base, poses, out) = (e.mem.alloc(0x30), e.mem.alloc(0x60), e.mem.alloc(0x60));
        put_transform(&mut e, base, [10.0, 0.0, 0.0, 0.0], ROTATION, [2.0; 4]);
        put_transform(
            &mut e,
            poses,
            [1.0, 0.0, 0.0, 0.0],
            IDENTITY,
            [1.0, 2.0, 3.0, 4.0],
        );
        put_transform(
            &mut e,
            poses + 0x30,
            [0.0, 1.0, 0.0, 0.0],
            IDENTITY,
            [5.0, 6.0, 7.0, 8.0],
        );
        e.call(0x00cd_df20, &args![2i32, base, poses, out]);
        let first = get_transform(&e, out);
        assert_close(first[0], [10.28, 0.96, 0.0, 0.0], 1e-5);
        assert_eq!(first[2], [2.0, 4.0, 6.0, 8.0]);
        let second = get_transform(&e, out + 0x30);
        assert_close(second[0], [9.04, 0.28, 0.0, 0.0], 1e-5);
        assert_eq!(second[2], [10.0, 12.0, 14.0, 16.0]);
        assert_eq!(
            calls(&e, QUATERNION_SET_MUL),
            vec![
                vec![out + 0x10, base + 0x10, poses + 0x10],
                vec![out + 0x40, base + 0x10, poses + 0x40],
            ]
        );
        e.call(0x00cd_df20, &args![0i32, base, poses, out]);
        assert_eq!(calls(&e, QUATERNION_SET_MUL).len(), 2);
    }

    #[test]
    fn fn_00cde010_removes_one_transform_from_each_pose() {
        let mut e = engine();
        install_set_mul(&mut e);
        let (base, poses, out) = (e.mem.alloc(0x30), e.mem.alloc(0x60), e.mem.alloc(0x60));
        put_transform(&mut e, base, [1.0, 2.0, 3.0, 0.0], IDENTITY, [2.0; 4]);
        put_transform(
            &mut e,
            poses,
            [2.0, 2.0, 2.0, 0.0],
            IDENTITY,
            [4.0, 4.0, 4.0, 0.0],
        );
        put_transform(
            &mut e,
            poses + 0x30,
            [5.0, 5.0, 5.0, 0.0],
            IDENTITY,
            [8.0, 6.0, 2.0, 0.0],
        );
        e.call(0x00cd_e010, &args![2i32, base, poses, out]);
        let first = get_transform(&e, out);
        assert_eq!(first[0], [1.0, 0.0, -1.0, 0.0]);
        assert_close(first[2], [2.0, 2.0, 2.0, 0.0], 1e-4);
        let second = get_transform(&e, out + 0x30);
        assert_eq!(second[0], [4.0, 3.0, 2.0, 0.0]);
        assert_close(second[2], [4.0, 3.0, 1.0, 0.0], 1e-4);
        let sets = calls(&e, QUATERNION_SET_MUL);
        assert_eq!(sets.len(), 2);
        assert_eq!((sets[1][0], sets[1][2]), (out + 0x40, poses + 0x40));
        e.call(0x00cd_e010, &args![0i32, base, poses, out]);
        assert_eq!(calls(&e, QUATERNION_SET_MUL).len(), 2);
    }

    /// A skeleton of four bones (bone 0 is not flagged, bone 3 has no parent) and the poses
    /// the two reference-pose functions work on.
    fn flagged_skeleton(e: &mut Engine) -> u32 {
        let skeleton = make_skeleton(e, &[0xffff, 0, 1, 0xffff], &[0, 1, 1, 1]);
        let reference = e.mem.alloc(0xc0);
        store_vec(e, reference + 0x30, [1.0, 0.0, 0.0, 0.0]);
        store_vec(e, reference + 0x60, [0.0, 1.0, 0.0, 0.0]);
        store_vec(e, reference + 0x90, [7.0, 8.0, 9.0, 0.0]);
        e.mem.set_u32(skeleton + SKELETON_REFERENCE_POSE, reference);
        skeleton
    }

    #[test]
    fn fn_00cde1d0_places_the_flagged_bones_by_their_reference_translation() {
        let mut e = engine();
        let skeleton = flagged_skeleton(&mut e);
        let poses = e.mem.alloc(0xc0);
        put_transform(&mut e, poses, [5.0, 0.0, 0.0, 0.0], ROTATION, [1.0; 4]);
        for i in 1..4 {
            put_transform(&mut e, poses + 0x30 * i, [9.0; 4], ROTATION, [1.0; 4]);
        }
        e.call(0x00cd_e1d0, &args![skeleton, poses]);
        assert_eq!(load_vec(&e, poses), [5.0, 0.0, 0.0, 0.0]);
        assert_close(load_vec(&e, poses + 0x30), [5.28, 0.96, 0.0, 0.0], 1e-5);
        assert_close(load_vec(&e, poses + 0x60), [4.32, 1.24, 0.0, 0.0], 1e-5);
        assert_eq!(load_vec(&e, poses + 0x90), [7.0, 8.0, 9.0, 0.0]);
    }

    #[test]
    fn fn_00cde2d0_writes_the_flagged_bones_to_another_pose() {
        let mut e = engine();
        let skeleton = flagged_skeleton(&mut e);
        let (poses, out) = (e.mem.alloc(0xc0), e.mem.alloc(0xc0));
        store_vec(&mut e, poses + 0x30, [1.0, 0.0, 0.0, 0.0]);
        store_vec(&mut e, poses + 0x60, [0.0, 1.0, 0.0, 0.0]);
        store_vec(&mut e, poses + 0x90, [7.0, 8.0, 9.0, 0.0]);
        put_transform(&mut e, out, [5.0, 0.0, 0.0, 0.0], ROTATION, [1.0; 4]);
        for i in 1..4 {
            put_transform(&mut e, out + 0x30 * i, [9.0; 4], ROTATION, [1.0; 4]);
        }
        e.call(0x00cd_e2d0, &args![skeleton, poses, out]);
        assert_eq!(load_vec(&e, out), [5.0, 0.0, 0.0, 0.0]);
        assert_close(load_vec(&e, out + 0x30), [5.28, 0.96, 0.0, 0.0], 1e-5);
        assert_close(load_vec(&e, out + 0x60), [4.32, 1.24, 0.0, 0.0], 1e-5);
        assert_eq!(load_vec(&e, out + 0x90), [7.0, 8.0, 9.0, 0.0]);
    }

    #[test]
    fn get_bone_chain_pushes_the_bones_up_to_the_stop_bone() {
        let mut e = engine();
        install_router(&mut e, 0x0500_0000, 0x1234);
        install_array_insert(&mut e);
        let skeleton = make_skeleton(&mut e, &[0xffff, 0, 1, 2], &[0; 4]);
        let found = e.mem.alloc(4);
        let chain = make_array(&mut e, 8);
        e.call(0x00cd_e3d0, &args![found, skeleton, 1u32, 3u32, chain]);
        assert_eq!(array_words(&e, chain), vec![3, 2, 1]);
        assert_eq!(e.mem.u8(found), 1);
        let inserts = calls(&e, ARRAY_INSERT);
        assert_eq!(inserts.len(), 3);
        assert_eq!(inserts[0][..3], [chain, 0x1234, 0]);
        // The walk ends on the root bone, which is not the stop bone.
        let chain = make_array(&mut e, 8);
        e.call(0x00cd_e3d0, &args![found, skeleton, 7u32, 2u32, chain]);
        assert_eq!(array_words(&e, chain), vec![2, 1, 7]);
        assert_eq!(e.mem.u8(found), 1);
        // The walk leaves the skeleton through a negative parent.
        e.mem.set_u8(found, 1);
        let chain = make_array(&mut e, 8);
        e.mem
            .set_u16(e.mem.u32(skeleton + SKELETON_PARENTS) + 2, 0xffff);
        e.call(0x00cd_e3d0, &args![found, skeleton, 9u32, 2u32, chain]);
        assert_eq!(array_words(&e, chain), vec![2, 1, 9]);
        assert_eq!(e.mem.u8(found), 0);
        // A start bone that is not positive pushes only the stop bone.
        e.mem.set_u8(found, 1);
        let chain = make_array(&mut e, 8);
        e.call(0x00cd_e3d0, &args![found, skeleton, 5u32, 0xffffu32, chain]);
        assert_eq!(array_words(&e, chain), vec![5]);
        assert_eq!(e.mem.u8(found), 0);
        let chain = make_array(&mut e, 8);
        e.call(0x00cd_e3d0, &args![found, skeleton, 5u32, 0u32, chain]);
        assert_eq!(array_words(&e, chain), vec![5]);
        assert_eq!(e.mem.u8(found), 1);
    }

    #[test]
    fn get_bone_chain_answers_its_flag_pointer() {
        let mut e = engine();
        install_router(&mut e, 0x0500_0000, 0x1234);
        install_array_insert(&mut e);
        let skeleton = make_skeleton(&mut e, &[0xffff, 0, 1, 2], &[0; 4]);
        let found = e.mem.alloc(4);
        let chain = make_array(&mut e, 8);
        // EAX is `found` at the RET; the callers test the flag through it (00ce37ca).
        let answered = e
            .call(0x00cd_e3d0, &args![found, skeleton, 1u32, 3u32, chain])
            .u32();
        assert_eq!(answered, found);
    }

    #[test]
    fn get_descendants_marks_and_pushes_the_bones_below() {
        let mut e = engine();
        let stack = e.mem.alloc(0x400);
        for i in 0..0x400 {
            e.mem.set_u8(stack + i, 0xff);
        }
        let router = install_router(&mut e, stack, 0x1234);
        let skeleton = make_skeleton(&mut e, &[0xffff, 0, 0, 1, 2, 3], &[0; 6]);
        let out = make_array(&mut e, 8);
        e.call(0x00cd_e450, &args![skeleton, 1u32, out, true]);
        assert_eq!(array_words(&e, out), vec![1, 3, 5]);
        // The router's stack pointer is back and the marks were cleared first.
        assert_eq!(e.mem.u32(router), stack);
        let marks: Vec<u8> = (0..8).map(|i| e.mem.u8(stack + i)).collect();
        assert_eq!(marks, vec![0, 1, 0, 1, 0, 1, 0xff, 0xff]);
        assert_eq!(calls(&e, MEMSET), vec![vec![stack, 0, 6]]);
        // Without the bone itself, and from a bone with no children.
        let out = make_array(&mut e, 8);
        e.call(0x00cd_e450, &args![skeleton, 1u32, out, false]);
        assert_eq!(array_words(&e, out), vec![3, 5]);
        let out = make_array(&mut e, 8);
        e.call(0x00cd_e450, &args![skeleton, 4u32, out, true]);
        assert_eq!(array_words(&e, out), vec![4]);
        // The parent word 0xffff is skipped even past the first bone.
        e.mem
            .set_u16(e.mem.u32(skeleton + SKELETON_PARENTS) + 8, 0xffff);
        let out = make_array(&mut e, 8);
        e.call(0x00cd_e450, &args![skeleton, 0u32, out, false]);
        assert_eq!(array_words(&e, out), vec![1, 2, 3, 5]);
    }

    #[test]
    fn get_descendants_grows_a_full_output_array_and_frees_a_moved_stack_array() {
        let mut e = engine();
        let stack = e.mem.alloc(0x400);
        let router = install_router(&mut e, stack, 0x2000_0000);
        // A vtable for the allocator whose slot 0x10 records its arguments.
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, 0x0900_0200);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, vtable);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(0x0900_0200, |_, _| Ret::default());
        let skeleton = make_skeleton(&mut e, &[0xffff, 0, 1], &[0; 3]);
        // The array to push to is full: the growth goes through `00c90940` (here a double).
        let out = make_array(&mut e, 1);
        e.mem.set_u32(out + 4, 1);
        e.register_double(ARRAY_GROW, |e, a| {
            let new_data = e.mem.alloc(16);
            let old_data = e.mem.u32(a[1]);
            e.mem.set_u16(new_data, e.mem.u16(old_data));
            e.mem.set_u32(a[1], new_data);
            e.mem.set_u32(a[1] + 8, 8);
            Ret::default()
        });
        e.call(0x00cd_e450, &args![skeleton, 0u32, out, true]);
        assert_eq!(calls(&e, ARRAY_GROW), vec![vec![allocator, out, 2]]);
        assert_eq!(array_words(&e, out), vec![0, 0, 1, 2]);
        // A local array that left the router's stack is freed with (data, capacity). The
        // reserve double moves it to the heap and the memset double shrinks the skeleton so that
        // the huge byte count of the call is never walked.
        let big = make_skeleton(&mut e, &[0xffff, 0, 1], &[0; 3]);
        e.mem.set_u32(big + SKELETON_BONE_COUNT, 0x4000_0003);
        let moved = e.mem.alloc(16);
        e.register_double(ARRAY_RESERVE, move |e, a| {
            e.mem.set_u32(a[1], moved);
            e.mem.set_u32(a[1] + 8, 0x10);
            Ret::default()
        });
        e.register_double(MEMSET, move |e, _| {
            e.mem.set_u32(big + SKELETON_BONE_COUNT, 3);
            Ret::default()
        });
        let out = make_array(&mut e, 8);
        e.call(0x00cd_e450, &args![big, 1u32, out, false]);
        assert_eq!(calls(&e, ARRAY_RESERVE).len(), 1);
        let reserved = calls(&e, ARRAY_RESERVE);
        assert_eq!(
            (reserved[0][0], reserved[0][2], reserved[0][3]),
            (allocator, 0x4000_0003, 1)
        );
        assert_eq!(calls(&e, 0x0900_0200), vec![vec![allocator, moved, 0x10]]);
        assert_eq!(array_words(&e, out), vec![2]);
    }

    #[test]
    fn fn_00cde740_interpolates_two_transforms() {
        let mut e = engine();
        e.register_double(NORMALIZE, |_, _| Ret::default());
        let (from, to, out, weight) = (
            e.mem.alloc(0x30),
            e.mem.alloc(0x30),
            e.mem.alloc(0x30),
            e.mem.alloc(16),
        );
        put_transform(&mut e, from, [0.0; 4], IDENTITY, [1.0, 2.0, 3.0, 4.0]);
        put_transform(
            &mut e,
            to,
            [4.0, 8.0, 12.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [5.0, 6.0, 7.0, 8.0],
        );
        store_vec(&mut e, weight, [0.25, 9.0, 9.0, 9.0]);
        e.call(0x00cd_e740, &args![from, to, weight, out]);
        let result = get_transform(&e, out);
        assert_eq!(result[0], [1.0, 2.0, 3.0, 0.0]);
        assert_eq!(result[1], [0.0, 0.0, 0.25, 0.75]);
        assert_eq!(result[2], [2.0, 3.0, 4.0, 5.0]);
        assert_eq!(calls(&e, NORMALIZE), vec![vec![out + 0x10]]);
    }

    #[test]
    fn fn_00cde740_negates_the_target_rotation_when_the_dot_is_negative() {
        let mut e = engine();
        e.register_double(NORMALIZE, |_, _| Ret::default());
        let (from, to, out, weight) = (
            e.mem.alloc(0x30),
            e.mem.alloc(0x30),
            e.mem.alloc(0x30),
            e.mem.alloc(16),
        );
        put_transform(&mut e, from, [0.0; 4], IDENTITY, [1.0; 4]);
        put_transform(&mut e, to, [0.0; 4], [0.0, 0.0, 0.0, -1.0], [1.0; 4]);
        store_vec(&mut e, weight, [0.5, 0.0, 0.0, 0.0]);
        e.call(0x00cd_e740, &args![from, to, weight, out]);
        assert_eq!(load_vec(&e, out + 0x10), [0.0, 0.0, 0.0, 1.0]);
        // A NaN dot is not negative: the target is used as it is.
        put_transform(&mut e, to, [0.0; 4], [f32::NAN, 0.0, 0.0, -1.0], [1.0; 4]);
        e.call(0x00cd_e740, &args![from, to, weight, out]);
        assert!(load_vec(&e, out + 0x10)[0].is_nan());
        assert_eq!(calls(&e, NORMALIZE).len(), 2);
    }

    #[test]
    fn fn_00cde740_sets_the_identity_for_a_short_rotation() {
        let mut e = engine();
        e.register_double(NORMALIZE, |_, _| Ret::default());
        let (from, to, out, weight) = (
            e.mem.alloc(0x30),
            e.mem.alloc(0x30),
            e.mem.alloc(0x30),
            e.mem.alloc(16),
        );
        put_transform(&mut e, from, [0.0; 4], [0.0; 4], [1.0; 4]);
        put_transform(&mut e, to, [0.0; 4], [0.0; 4], [1.0; 4]);
        store_vec(&mut e, weight, [0.5, 0.0, 0.0, 0.0]);
        e.call(0x00cd_e740, &args![from, to, weight, out]);
        assert_eq!(load_vec(&e, out + 0x10), IDENTITY);
        assert!(calls(&e, NORMALIZE).is_empty());
    }

    #[test]
    fn fn_00cde940_pushes_one_element_through_the_array_insert() {
        let mut e = engine();
        e.register_double(ARRAY_INSERT, |_, _| Ret::default());
        e.call(
            0x00cd_e940,
            &args![0x0500_0000u32, 7u32, 0u32, 0x0500_0100u32],
        );
        assert_eq!(
            calls(&e, ARRAY_INSERT),
            vec![vec![0x0500_0000, 7, 0, 0x0500_0100, 1]]
        );
    }
}
