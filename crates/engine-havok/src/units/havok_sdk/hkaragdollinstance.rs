//! `Havok SDK/hkaragdollinstance.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkaRagdollInstance` (Xbox PDB name) is read off the code: `+0` the vtable, `+4` the 16-bit
//! size of the block, `+6` the 16-bit reference count, `+8` an `hkArray` of rigid bodies (data,
//! size `+0xc`, capacity word `+0x10`), `+0x14` an `hkArray` of constraints (size `+0x18`,
//! capacity word `+0x1c`), `+0x20` an `hkArray` of 32-bit rigid body indices, one per skeleton
//! bone (`-1` for none; size `+0x24`, capacity word `+0x28`), and `+0x2c` the skeleton (reference
//! counted; `+0xc` the 16-bit parent index of every bone, `+0x1c` the bone count, `+0x24` the
//! reference pose of 0x30-byte transforms). The rigid bodies are `hkpRigidBody`s (`+8` the world,
//! `+0xe0` the motion, `+0xf0` the transform, `+0x1b0` and `+0x1c0` the linear and angular
//! velocity). A transform of a pose is 0x30 bytes: the translation, the rotation quaternion and
//! the scale, each a 16-byte vector. Nothing here has an exception frame.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkakeyframehierarchyutility::{
    add4, mul4, rotate_and_move, rotate_half, splat, sub4, within_tolerance,
};
use crate::units::havok_sdk::hkpsphereshape::{
    dot3, load_bits, load_vec, refined_reciprocal, shufps, store_vec, unpckhps, Vec4,
};

/// The memory router (`0x00c8_5750`): its field `+0` is the bump cursor of the stack-like arena and
/// its field `+0x10` the allocator, whose virtual functions at `+4` (allocate a block), `+0xc`
/// (allocate through a size in memory, which is rounded in place), `+0x10` (free `(data, bytes)`)
/// this unit uses.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkReferencedObject::addReference` and `removeReference` (`00c90510`, `00c905b0`, `ECX` = the
/// object).
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl `(allocator, array, capacity, element size)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// `hkQuaternion::setMul` (`00c66320`, `this` = the result, `(a, b)`), the quaternion product
/// `00c74ac0` (`this` = the result, `(a, b)`), the unnamed in-place rescaling of a quaternion
/// (`005611c0`, `ECX` = the quaternion), the unnamed routine that answers the unit axis of a
/// quaternion (`00c74c90`, `ECX` = the quaternion, `(out)`) and the one that answers its angle on
/// the x87 stack (`00c74c20`, `ECX` = the quaternion).
const QUATERNION_SET_MUL: u32 = 0x00c6_6320;
const QUATERNION_PRODUCT: u32 = 0x00c7_4ac0;
const QUATERNION_RESCALE: u32 = 0x0056_11c0;
const QUATERNION_AXIS: u32 = 0x00c7_4c90;
const QUATERNION_ANGLE: u32 = 0x00c7_4c20;
/// `hkQsTransform::setFromTransformNoScale` (`00ce5070`, `ECX` = the result, `(transform)`) and
/// the unnamed routine that multiplies the reference pose scales of a bone and its ancestors
/// into a vector (`00cdd600`, cdecl `(skeleton, reference pose, bone, out)`).
const SET_FROM_TRANSFORM_NO_SCALE: u32 = 0x00ce_5070;
const ACCUMULATE_SCALE: u32 = 0x00cd_d600;
/// `hkpMotion::approxTransformAt` (`00c86490`, `ECX` = the motion, `(time, out)`), the world's
/// `getCurrentTime` (`00c91020`, `ECX` = the world, answers the time on the x87 stack),
/// `hkpRigidBody::setPositionAndRotation` (`00ca8860`, `ECX` = the body, `(position, rotation)`),
/// the activation of an entity (`00c9c1d0`, `ECX` = the entity), `hkpWorld::addEntityBatch`
/// (`00c94bd0`, `ECX` = the world, `(entities, count, flag)`), `addConstraint` (`00c97d30`,
/// `ECX` = the world, `(constraint)`), `updateCollisionFilterOnEntity` (`00c946e0`, `ECX` = the
/// world, `(entity, flag, flag)`) and `hkpConstraintInstance::clone` (`00cf3fa0`, `ECX` = the
/// constraint, `(first, second, world)`).
const MOTION_APPROX_TRANSFORM_AT: u32 = 0x00c8_6490;
const WORLD_CURRENT_TIME: u32 = 0x00c9_1020;
const SET_POSITION_AND_ROTATION: u32 = 0x00ca_8860;
const ENTITY_ACTIVATE: u32 = 0x00c9_c1d0;
const ADD_ENTITY_BATCH: u32 = 0x00c9_4bd0;
const ADD_CONSTRAINT: u32 = 0x00c9_7d30;
const UPDATE_COLLISION_FILTER: u32 = 0x00c9_46e0;
const CLONE_CONSTRAINT: u32 = 0x00cf_3fa0;
/// This unit's own routines, which the translations call the way the code does.
const POSE_FROM_RIGID_BODIES: u32 = 0x00ce_1760;
const POSE_FROM_CURRENT_BODIES: u32 = 0x00ce_18d0;
const BONE_POSE_FROM_BODIES: u32 = 0x00ce_2000;
const BONE_POSE_AT_TIME: u32 = 0x00ce_2150;
const POSE_AT_TIME: u32 = 0x00ce_2430;
const ASSIGN_BODIES_AND_CONSTRAINTS: u32 = 0x00ce_22c0;
const CONSTRUCTOR: u32 = 0x00ce_2510;
/// The vtable written by the constructor, and the one the destructor leaves.
const VTABLE: u32 = 0x010c_b830;
const BASE_VTABLE: u32 = 0x0102_e388;
/// The root transform of `00ce18d0` (`01268700`), the zero vector the velocities are compared with
/// (`01267e30`), the identity quaternion vector (`010c71b0`), the float that fills the scale of a
/// root (`01012068`), the four ones (`010c7170`), the sign bits (`010c44d0`), the sign bits of the
/// three lowest lanes (`010c4c00`), the all-ones mask of the three lowest lanes (`010c4c10`) and
/// the smallest rotation that counts as one (`0104da4c`).
const ROOT_TRANSFORM: u32 = 0x0126_8700;
const ZERO_VELOCITY: u32 = 0x0126_7e30;
const IDENTITY_QUATERNION: u32 = 0x010c_71b0;
const SCALE_FILL: u32 = 0x0101_2068;
const ONE_LANES: u32 = 0x010c_7170;
const SIGN_BITS: u32 = 0x010c_44d0;
const SIGN_BITS_XYZ: u32 = 0x010c_4c00;
const MASK_XYZ: u32 = 0x010c_4c10;
const SMALLEST_ROTATION: u32 = 0x0104_da4c;
/// The bit that marks the storage of an `hkArray` as not owned, and the capacity mask.
const NOT_OWNED: u32 = 0x8000_0000;
const CAPACITY_MASK: u32 = 0x3fff_ffff;

// Translated from 00ce29b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00ce_2600`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00ce29b0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00ce_2600, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// The allocator the memory router points at.
fn router_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// The number of bones: the skeleton's field `+0x1c`.
fn bone_count(e: &Engine, this: u32) -> i32 {
    e.mem.u32(e.mem.u32(this + 0x2c) + 0x1c) as i32
}

/// The address of element `index` of an array of 0x30-byte transforms at `base`.
fn element(base: u32, index: i32) -> u32 {
    base.wrapping_add((index as u32).wrapping_mul(0x30))
}

/// The rigid body of `bone`, or 0 when its index is negative or its entry is empty.
fn bone_entity(e: &Engine, this: u32, bone: i32) -> u32 {
    let index = e.mem.i32(
        e.mem
            .u32(this + 0x20)
            .wrapping_add((bone as u32).wrapping_mul(4)),
    );
    if index < 0 {
        0
    } else {
        e.mem.u32(
            e.mem
                .u32(this + 8)
                .wrapping_add((index as u32).wrapping_mul(4)),
        )
    }
}

/// The 16-bit parent index of `bone` in the skeleton, sign extended (`MOVSX`).
fn parent_of(e: &Engine, this: u32, bone: i32) -> i32 {
    let parents = e.mem.u32(e.mem.u32(this + 0x2c) + 0xc);
    i32::from(
        e.mem
            .u16(parents.wrapping_add((bone as u32).wrapping_mul(2))) as i16,
    )
}

fn xor_bits(v: Vec4, mask: [u32; 4]) -> Vec4 {
    std::array::from_fn(|lane| f32::from_bits(v[lane].to_bits() ^ mask[lane]))
}

/// The three vectors that undo the transform at `transform` (translation, rotation, scale): the
/// translation turned by the rotation, doubled and negated (all four sign bits flipped), the
/// rotation with the sign bits of its three lowest lanes flipped, and the reciprocal of the scale
/// (`RCPPS` of its lanes 0 to 2 and 1.0, one Newton step, then the three lowest lanes kept, so
/// lane 3 is zero).
pub(crate) fn inverse_transform(e: &Engine, transform: u32) -> [Vec4; 3] {
    let rotation = load_vec(e, transform + 0x10);
    let translation = load_vec(e, transform);
    let half = rotate_half(e, rotation, translation);
    let negated = xor_bits(add4(half, half), load_bits(e, SIGN_BITS));
    let inverse_rotation = xor_bits(rotation, load_bits(e, SIGN_BITS_XYZ));
    let scale = load_vec(e, transform + 0x20);
    let ones = load_vec(e, ONE_LANES);
    let lanes = shufps(scale, unpckhps(scale, ones), 0xc4);
    let mask = load_bits(e, MASK_XYZ);
    let inverse_scale: Vec4 = std::array::from_fn(|lane| {
        f32::from_bits(refined_reciprocal(lanes[lane]).to_bits() & mask[lane])
    });
    [negated, inverse_rotation, inverse_scale]
}

/// The transform `pose` of a bone moved by `transform` into `frame` (translation, rotation, scale):
/// the translation is the pose's turned by the rotation of `transform` and moved by its own, the
/// rotation `setMul` of the two rotations (rescaled in the end by `005611c0`) and the scale the
/// product of the two.
fn place_in_world(e: &mut Engine, frame: u32, transform: u32, pose: u32) {
    let moved = rotate_and_move(
        e,
        load_vec(e, transform + 0x10),
        load_vec(e, pose),
        load_vec(e, transform),
    );
    store_vec(e, frame, moved);
    e.call(
        QUATERNION_SET_MUL,
        &args![frame + 0x10, transform + 0x10, pose + 0x10],
    );
    let scale = mul4(load_vec(e, transform + 0x20), load_vec(e, pose + 0x20));
    store_vec(e, frame + 0x20, scale);
    e.call(QUATERNION_RESCALE, &args![frame + 0x10]);
}

/// The body's motion, at `+0xe0` of the entity, is asked (virtual function at `offset`) to take
/// `velocity` after the entity is activated, when `velocity` is not within the tolerance of
/// `current`.
fn drive_velocity(e: &mut Engine, entity: u32, velocity: u32, current: Vec4, offset: u32) {
    if !within_tolerance(e, load_vec(e, velocity), current) {
        e.call(ENTITY_ACTIVATE, &args![entity]);
        e.vcall(entity + 0xe0, offset, &args![velocity]);
    }
}

/// A child's transform from its parent's (translation, rotation and scale at `parent`) and the
/// reference pose of the bone (`rest`): the parent's translation plus the reference translation
/// turned by the parent's rotation, `setMul` of the parent's and the reference rotation, and the
/// product of the scales.
fn compose_on_parent(e: &mut Engine, parent: u32, rest: u32, out: u32) {
    let moved = rotate_and_move(
        e,
        load_vec(e, parent + 0x10),
        load_vec(e, rest),
        load_vec(e, parent),
    );
    store_vec(e, out, moved);
    e.call(
        QUATERNION_SET_MUL,
        &args![out + 0x10, parent + 0x10, rest + 0x10],
    );
    let scale = mul4(load_vec(e, rest + 0x20), load_vec(e, parent + 0x20));
    store_vec(e, out + 0x20, scale);
}

/// What `00ce2000` and `00ce2150` write for a bone that has no rigid body and no parent: the
/// translation zero, the identity rotation and the scale filled with the float at `01012068`.
fn store_root_frame(e: &mut Engine, frame: u32) {
    let identity = load_vec(e, IDENTITY_QUATERNION);
    store_vec(e, frame, [0.0; 4]);
    let fill = e.mem.f32(SCALE_FILL);
    store_vec(e, frame + 0x10, identity);
    store_vec(e, frame + 0x20, splat(fill));
}

// Translated from 00ce1680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaRagdollInstance::addToWorld` (Xbox PDB), `ECX` = the instance, `(world, update_filters)`.
/// Answers 1 and does nothing when the first rigid body is already in a world (its field `+8`
/// is not zero). Otherwise adds all rigid bodies as a batch, then the constraints one by one and,
/// when `update_filters` is not zero, updates the collision filter of every rigid body. Answers 0.
pub fn hka_ragdoll_instance_add_to_world(
    e: &mut Engine,
    this: Ptr,
    world: Ptr,
    update_filters: u8,
) -> u32 {
    let (this, world) = (this.addr(), world.addr());
    let first = e.mem.u32(e.mem.u32(this + 8));
    if first != 0 && e.mem.u32(first + 8) != 0 {
        return 1;
    }
    let bodies = e.mem.u32(this + 8);
    let count = e.mem.u32(this + 0xc);
    e.call(ADD_ENTITY_BATCH, &args![world, bodies, count, 1u32]);
    let mut k = 0u32;
    while (k as i32) < e.mem.i32(this + 0x18) {
        let constraint = e.mem.u32(e.mem.u32(this + 0x14).wrapping_add(4 * k));
        e.call(ADD_CONSTRAINT, &args![world, constraint]);
        k += 1;
    }
    if update_filters != 0 {
        let count = e.mem.i32(this + 0xc);
        let mut k = 0i32;
        while k < count {
            let body = e.mem.u32(e.mem.u32(this + 8).wrapping_add(4 * k as u32));
            e.call(UPDATE_COLLISION_FILTER, &args![world, body, 1u32, 0u32]);
            k += 1;
        }
    }
    0
}

// Translated from 00ce1760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed, `ECX` = the instance, `(out, root, rest)`: the pose of every skeleton bone into the
/// array of transforms `out`. `rest` is the reference pose (the skeleton's when null) and `root`
/// the transform a bone without parent starts from. A bone that has a rigid body gets that
/// body's transform without scale (`00ce5070` on the body's `+0xf0`) and its scale from `00cdd600`;
/// any other bone gets its parent's transform (in `out`, or `root`) composed with its reference
/// transform (the rotation through `setMul` on a copy of the parent's) and the product of the
/// scales.
pub fn fn_00ce1760(e: &mut Engine, this: Ptr, out: Ptr, root: Ptr, rest: Ptr) {
    let (this, out, root) = (this.addr(), out.addr(), root.addr());
    let mut rest = rest.addr();
    if rest == 0 {
        rest = e.mem.u32(e.mem.u32(this + 0x2c) + 0x24);
    }
    let mut bone = 0i32;
    while bone < bone_count(e, this) {
        let target = element(out, bone);
        let entity = bone_entity(e, this, bone);
        if entity != 0 {
            e.call(SET_FROM_TRANSFORM_NO_SCALE, &args![target, entity + 0xf0]);
            let skeleton = e.mem.u32(this + 0x2c);
            let reference = e.mem.u32(skeleton + 0x24);
            e.call(
                ACCUMULATE_SCALE,
                &args![skeleton, reference, bone, target + 0x20],
            );
        } else {
            let parent = parent_of(e, this, bone);
            let source = if parent > -1 {
                element(out, parent)
            } else {
                root
            };
            let reference = element(rest, bone);
            let rotation = load_vec(e, source + 0x10);
            let scale = load_vec(e, source + 0x20);
            let moved = rotate_and_move(e, rotation, load_vec(e, reference), load_vec(e, source));
            e.with_stack(0x10, |e, copy| {
                store_vec(e, copy.addr(), rotation);
                store_vec(e, target, moved);
                e.call(
                    QUATERNION_SET_MUL,
                    &args![target + 0x10, copy.addr(), reference + 0x10],
                );
            });
            let scaled = mul4(load_vec(e, reference + 0x20), scale);
            store_vec(e, target + 0x20, scaled);
        }
        bone += 1;
    }
}

// Translated from 00ce18d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed, `ECX` = the instance, `(out)`: `00ce1760` with the root transform at `01268700` and the
/// skeleton's reference pose.
pub fn fn_00ce18d0(e: &mut Engine, this: Ptr, out: Ptr) {
    e.call(
        POSE_FROM_RIGID_BODIES,
        &args![this, out, ROOT_TRANSFORM, 0u32],
    );
}

// Translated from 00ce18f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaRagdollInstance::getPoseModelSpace` (Xbox PDB), `ECX` = the instance, `(out, transform)`:
/// the pose of `00ce18d0` into `out`, then every transform of it moved by the inverse of
/// `transform`: the translation is turned by the inverse rotation (the rotation with the sign
/// bits of its three lowest lanes flipped) and moved by the negated double of the turned
/// translation of `transform`, the rotation is `setMul` of the inverse rotation and the bone's,
/// and the scale is multiplied by the reciprocal of the scale of `transform` (`RCPPS` and one
/// Newton step, lanes 0 to 2, lane 3 zero).
pub fn hka_ragdoll_instance_get_pose_model_space(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
    transform: Ptr,
) {
    let (out, transform) = (out.addr(), transform.addr());
    e.call(POSE_FROM_CURRENT_BODIES, &args![this, out]);
    let count = bone_count(e, this.addr());
    let [negated, inverse_rotation, inverse_scale] = inverse_transform(e, transform);
    e.with_stack(0x30, |e, frame| {
        let frame = frame.addr();
        store_vec(e, frame, negated);
        store_vec(e, frame + 0x10, inverse_rotation);
        store_vec(e, frame + 0x20, inverse_scale);
        for bone in 0..count.max(0) {
            let target = element(out, bone);
            let half = rotate_half(e, inverse_rotation, load_vec(e, target));
            store_vec(e, target, add4(add4(half, half), negated));
            e.call(
                QUATERNION_SET_MUL,
                &args![target + 0x10, frame + 0x10, target + 0x10],
            );
            let scaled = mul4(load_vec(e, frame + 0x20), load_vec(e, target + 0x20));
            store_vec(e, target + 0x20, scaled);
        }
    });
}

// Translated from 00ce1ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed, `ECX` = the instance, `(pose, transform)`: puts every rigid body where the pose
/// (an array of transforms, one per skeleton bone) moved by `transform` says
/// (`place_in_world`, then `setPositionAndRotation`), and gives the body's motion the zero
/// velocity at `01267e30` (after activating the body) when its linear velocity (`+0x1b0`) or its
/// angular velocity (`+0x1c0`) is not within the tolerance of it. The bone count is read once.
pub fn fn_00ce1ad0(e: &mut Engine, this: Ptr, pose: Ptr, transform: Ptr) {
    let (this, pose, transform) = (this.addr(), pose.addr(), transform.addr());
    let count = bone_count(e, this);
    e.with_stack(0x30, |e, frame| {
        let frame = frame.addr();
        for bone in 0..count.max(0) {
            let entity = bone_entity(e, this, bone);
            if entity == 0 {
                continue;
            }
            place_in_world(e, frame, transform, element(pose, bone));
            e.call(
                SET_POSITION_AND_ROTATION,
                &args![entity, frame, frame + 0x10],
            );
            let linear = load_vec(e, entity + 0x1b0);
            drive_velocity(e, entity, ZERO_VELOCITY, linear, 0x40);
            let angular = load_vec(e, entity + 0x1c0);
            drive_velocity(e, entity, ZERO_VELOCITY, angular, 0x44);
        }
    });
}

// Translated from 00ce1ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed, `ECX` = the instance, `(pose_a, transform_a, pose_b, transform_b, time_step)`: puts
/// every rigid body where the second pose moved by its transform says, and gives its motion the
/// velocities that lead there from the first pose in `time_step`: the linear one is the change of
/// translation times the reciprocal of the step (`1 / step` on the x87 stack), the angular one
/// the axis of the rotation between the two (`00c74ac0`, rescaled by `005611c0`) times its angle
/// (`00c74c20`) times the reciprocal, or zero when the length of its three lowest lanes is not
/// greater than the float at `0104da4c`. Each velocity is given (after activating the body) when
/// it is not within the tolerance of the one the body has. The bone count is read once.
pub fn fn_00ce1ca0(
    e: &mut Engine,
    this: Ptr,
    pose_a: Ptr,
    transform_a: Ptr,
    pose_b: Ptr,
    transform_b: Ptr,
    time_step: f32,
) {
    let (this, pose_a, transform_a) = (this.addr(), pose_a.addr(), transform_a.addr());
    let (pose_b, transform_b) = (pose_b.addr(), transform_b.addr());
    let count = bone_count(e, this);
    let inverse = (1.0f64 / f64::from(x87_quiet(time_step))) as f32;
    e.with_stack(0xa0, |e, frame| {
        let frame = frame.addr();
        let (angular_velocity, rotation_between) = (frame, frame + 0x10);
        let (world_b, linear_velocity, world_a) = (frame + 0x20, frame + 0x60, frame + 0x70);
        for bone in 0..count.max(0) {
            let entity = bone_entity(e, this, bone);
            if entity == 0 {
                continue;
            }
            place_in_world(e, world_a, transform_a, element(pose_a, bone));
            place_in_world(e, world_b, transform_b, element(pose_b, bone));
            e.call(
                SET_POSITION_AND_ROTATION,
                &args![entity, world_b, world_b + 0x10],
            );
            let change = sub4(load_vec(e, world_b), load_vec(e, world_a));
            store_vec(e, linear_velocity, mul4(splat(inverse), change));
            let linear = load_vec(e, entity + 0x1b0);
            drive_velocity(e, entity, linear_velocity, linear, 0x40);
            e.call(
                QUATERNION_PRODUCT,
                &args![rotation_between, world_b + 0x10, world_a + 0x10],
            );
            e.call(QUATERNION_RESCALE, &args![rotation_between]);
            let length = dot3(load_vec(e, rotation_between), load_vec(e, rotation_between)).sqrt();
            let velocity = if x87_quiet(e.mem.f32(SMALLEST_ROTATION)) < length {
                e.call(QUATERNION_AXIS, &args![rotation_between, angular_velocity]);
                let angle = e.call(QUATERNION_ANGLE, &args![rotation_between]).f64();
                let rate = (angle * f64::from(inverse)) as f32;
                mul4(splat(rate), load_vec(e, angular_velocity))
            } else {
                [0.0; 4]
            };
            store_vec(e, angular_velocity, velocity);
            let angular = load_vec(e, entity + 0x1c0);
            drive_velocity(e, entity, angular_velocity, angular, 0x44);
        }
    });
}

// Translated from 00ce2000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed, `ECX` = the instance, `(bone, out)`: the transform of one bone into `out` the way
/// `00ce1760` makes it, but walking up to the parents by recursion: a bone with a rigid body gets
/// its transform from the body, any other the one of its parent (the zero translation, the
/// identity rotation and the scale `01012068` for a root) composed with its reference transform.
pub fn fn_00ce2000(e: &mut Engine, this: Ptr, bone: i32, out: Ptr) {
    let (this, out) = (this.addr(), out.addr());
    let entity = bone_entity(e, this, bone);
    if entity != 0 {
        e.call(SET_FROM_TRANSFORM_NO_SCALE, &args![out, entity + 0xf0]);
        let skeleton = e.mem.u32(this + 0x2c);
        let reference = e.mem.u32(skeleton + 0x24);
        e.call(
            ACCUMULATE_SCALE,
            &args![skeleton, reference, bone, out + 0x20],
        );
        return;
    }
    let parent = parent_of(e, this, bone);
    e.with_stack(0x30, |e, frame| {
        let frame = frame.addr();
        if parent >= 0 {
            e.call(BONE_POSE_FROM_BODIES, &args![this, parent, frame]);
        } else {
            store_root_frame(e, frame);
        }
        let reference = element(e.mem.u32(e.mem.u32(this + 0x2c) + 0x24), bone);
        compose_on_parent(e, frame, reference, out);
    });
}

// Translated from 00ce2150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed, `ECX` = the instance, `(bone, time, out)`: `00ce2000` at a point in time: a bone with
/// a rigid body gets the body's transform at `time` (`hkpMotion::approxTransformAt` of the
/// motion at `+0xe0`, through `00ce5070`; `time` is copied through the x87 stack) and its scale
/// from `00cdd600`; the parents are found by recursion with the same time.
pub fn fn_00ce2150(e: &mut Engine, this: Ptr, bone: i32, time: f32, out: Ptr) {
    let (this, out) = (this.addr(), out.addr());
    let time = x87_quiet(time);
    let entity = bone_entity(e, this, bone);
    if entity != 0 {
        e.with_stack(0x40, |e, transform| {
            e.call(
                MOTION_APPROX_TRANSFORM_AT,
                &args![entity + 0xe0, time, transform.addr()],
            );
            e.call(SET_FROM_TRANSFORM_NO_SCALE, &args![out, transform.addr()]);
        });
        let skeleton = e.mem.u32(this + 0x2c);
        let reference = e.mem.u32(skeleton + 0x24);
        e.call(
            ACCUMULATE_SCALE,
            &args![skeleton, reference, bone, out + 0x20],
        );
        return;
    }
    let parent = parent_of(e, this, bone);
    e.with_stack(0x30, |e, frame| {
        let frame = frame.addr();
        if parent >= 0 {
            e.call(BONE_POSE_AT_TIME, &args![this, parent, time, frame]);
        } else {
            store_root_frame(e, frame);
        }
        let reference = element(e.mem.u32(e.mem.u32(this + 0x2c) + 0x24), bone);
        compose_on_parent(e, frame, reference, out);
    });
}

// Translated from 00ce2430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed, `ECX` = the instance, `(time, out)`: `00ce2150` for every skeleton bone at `time`,
/// into the array of transforms `out`. The bone count is read once.
pub fn fn_00ce2430(e: &mut Engine, this: Ptr, time: f32, out: Ptr) {
    let count = bone_count(e, this.addr());
    for bone in 0..count.max(0) {
        e.call(
            BONE_POSE_AT_TIME,
            &args![this, bone, x87_quiet(time), element(out.addr(), bone)],
        );
    }
}

// Translated from 00ce26f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed, `ECX` = the instance, `(out)`: the pose of every bone into `out`: at the world's
/// current time (`00c91020`) through `00ce2430` when the first rigid body is in a world, and from
/// the bodies as they are (`00ce18d0`) otherwise.
pub fn fn_00ce26f0(e: &mut Engine, this: Ptr, out: Ptr) {
    let first = e.mem.u32(e.mem.u32(this.addr() + 8));
    let world = if first != 0 { e.mem.u32(first + 8) } else { 0 };
    if world == 0 {
        e.call(POSE_FROM_CURRENT_BODIES, &args![this, out]);
        return;
    }
    let time = e.call(WORLD_CURRENT_TIME, &args![world]).f32();
    e.call(POSE_AT_TIME, &args![this, time, out]);
}

/// The array copy `00ce22c0` and `00ce2510` do for each of their arrays: the array at `this +
/// field` (data, size `+4`, capacity word `+8`) is given the elements of the array at `source`
/// (data, size `+4`). When its capacity is smaller than the source's size, the old storage is
/// freed through the allocator (virtual function `+0x10`, unless the storage is not owned) and new
/// storage of 4 bytes per element is taken (virtual function `+0xc`, which rounds the byte size in
/// memory; the new capacity is that size divided by 4, rounded toward zero).
fn assign_array(e: &mut Engine, this: u32, field: u32, source: u32) {
    let allocator = router_allocator(e);
    let capacity = e.mem.u32(this + field + 8);
    let wanted = e.mem.i32(source + 4);
    let held = (capacity & CAPACITY_MASK) as i32;
    if held < wanted {
        if capacity as i32 >= 0 {
            let data = e.mem.u32(this + field);
            let bytes = (held as u32).wrapping_mul(4);
            e.vcall(allocator, 0x10, &args![data, bytes]);
        }
        let bytes = (wanted as u32).wrapping_mul(4);
        let (block, rounded) = e.with_stack(4, |e, size| {
            e.mem.set_u32(size.addr(), bytes);
            let block = e.vcall(allocator, 0xc, &args![size]).u32();
            (block, e.mem.i32(size.addr()))
        });
        e.mem.set_u32(this + field, block);
        let slots = rounded.wrapping_add((rounded >> 31) & 3) >> 2;
        e.mem.set_u32(this + field + 8, slots as u32);
    }
    let size = e.mem.u32(source + 4);
    let target = e.mem.u32(this + field);
    e.mem.set_u32(this + field + 4, size);
    let from = e.mem.u32(source);
    for k in 0..(size as i32).max(0) as u32 {
        let word = e.mem.u32(from.wrapping_add(4 * k));
        e.mem.set_u32(target.wrapping_add(4 * k), word);
    }
}

/// `addReference` on every non-null entry of the array at `this + field`, the size read again for
/// every entry.
fn add_references(e: &mut Engine, this: u32, field: u32) {
    let mut k = 0u32;
    while (k as i32) < e.mem.i32(this + field + 4) {
        let object = e.mem.u32(e.mem.u32(this + field).wrapping_add(4 * k));
        if object != 0 {
            e.call(ADD_REFERENCE, &args![object]);
        }
        k += 1;
    }
}

/// `removeReference` on every entry of the array at `this + field`, the size read again for every
/// entry.
fn remove_references(e: &mut Engine, this: u32, field: u32) {
    let mut k = 0u32;
    while (k as i32) < e.mem.i32(this + field + 4) {
        let object = e.mem.u32(e.mem.u32(this + field).wrapping_add(4 * k));
        e.call(REMOVE_REFERENCE, &args![object]);
        k += 1;
    }
}

/// What the destructor does to each of its arrays: size zero, the storage freed (data and
/// `capacity * 4` bytes through the allocator's function `+0x10`) unless it is not owned, then
/// data null and the capacity word just the not-owned bit.
fn release_array(e: &mut Engine, this: u32, field: u32) {
    let allocator = router_allocator(e);
    let capacity = e.mem.u32(this + field + 8);
    e.mem.set_u32(this + field + 4, 0);
    if capacity & NOT_OWNED == 0 {
        let bytes = (capacity & CAPACITY_MASK).wrapping_mul(4);
        let data = e.mem.u32(this + field);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(this + field, 0);
    e.mem.set_u32(this + field + 8, NOT_OWNED);
}

// Translated from 00ce22c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed, `ECX` = the instance, `(bodies, constraints, skeleton)`: takes a reference to the new
/// skeleton (when not null), gives up the old one (when not null) and keeps the new; copies the
/// two source arrays (data pointer, size) into the instance's rigid body and constraint arrays
/// (`assign_array`); and adds a reference to every non-null rigid body and constraint.
pub fn fn_00ce22c0(e: &mut Engine, this: Ptr, bodies: Ptr, constraints: Ptr, skeleton: Ptr) {
    let (this, skeleton) = (this.addr(), skeleton.addr());
    if skeleton != 0 {
        e.call(ADD_REFERENCE, &args![skeleton]);
    }
    let old = e.mem.u32(this + 0x2c);
    if old != 0 {
        e.call(REMOVE_REFERENCE, &args![old]);
    }
    e.mem.set_u32(this + 0x2c, skeleton);
    assign_array(e, this, 8, bodies.addr());
    assign_array(e, this, 0x14, constraints.addr());
    add_references(e, this, 8);
    add_references(e, this, 0x14);
}

// Translated from 00ce2510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaRagdollInstance::hkaRagdollInstance` (Xbox PDB), `ECX` = the instance, `(bodies,
/// constraints, skeleton, mapping)`: the constructor. Sets the vtable and a reference count of 1,
/// empties the three arrays and the skeleton, runs `00ce22c0` and then copies the array `mapping`
/// (data pointer, size) into the rigid body index array. Answers the instance.
pub fn hka_ragdoll_instance_constructor(
    e: &mut Engine,
    this: Ptr,
    bodies: Ptr,
    constraints: Ptr,
    skeleton: Ptr,
    mapping: Ptr,
) -> Ptr {
    let at = this.addr();
    e.mem.set_u32(at, VTABLE);
    e.mem.set_u16(at + 6, 1);
    for (offset, word) in [
        (0x8, 0),
        (0xc, 0),
        (0x10, NOT_OWNED),
        (0x1c, NOT_OWNED),
        (0x14, 0),
        (0x18, 0),
        (0x28, NOT_OWNED),
        (0x20, 0),
        (0x24, 0),
        (0x2c, 0),
    ] {
        e.mem.set_u32(at + offset, word);
    }
    e.call(
        ASSIGN_BODIES_AND_CONSTRAINTS,
        &args![this, bodies, constraints, skeleton],
    );
    assign_array(e, at, 0x20, mapping.addr());
    this
}

// Translated from 00ce25f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed, `ECX` = the instance, `(_unused_1)`: sets the vtable only. Answers the instance.
pub fn fn_00ce25f0(e: &mut Engine, this: Ptr, _unused_1: u32) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE);
    this
}

// Translated from 00ce2600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (the destructor), `ECX` = the instance: sets the vtable, gives up the reference to every
/// rigid body and constraint (none may be null) and to the skeleton (when not null), frees the
/// three arrays (`release_array`: the index array, then the constraints, then the rigid bodies)
/// and sets the base vtable `0102e388`.
pub fn fn_00ce2600(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    e.mem.set_u32(this, VTABLE);
    remove_references(e, this, 8);
    remove_references(e, this, 0x14);
    let skeleton = e.mem.u32(this + 0x2c);
    if skeleton != 0 {
        e.call(REMOVE_REFERENCE, &args![skeleton]);
    }
    e.mem.set_u32(this + 0x2c, 0);
    release_array(e, this, 0x20);
    release_array(e, this, 0x14);
    release_array(e, this, 8);
    e.mem.set_u32(this, BASE_VTABLE);
}

/// A stack-like `hkArray` of `count` 4-byte elements (data, size, capacity word, and `+0xc` the
/// arena position it started at): the router's cursor is moved on by `count * 4` rounded up to
/// a multiple of 128, the array takes the old cursor as its data with a capacity of `count` and the
/// not-owned bit, and `_reserve` is asked for more room (twice the capacity, at least `count`)
/// when that capacity is smaller than `count` (not the case unless `count` is negative or
/// larger than the mask). Ends with the size `count`.
fn begin_stack_array(e: &mut Engine, array: u32, count: i32) {
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 4, 0);
    e.mem.set_u32(array + 8, NOT_OWNED);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let cursor = e.mem.u32(router);
    let rounded = (count as u32).wrapping_mul(4).wrapping_add(0x7f) & 0xffff_ff80;
    e.mem.set_u32(router, cursor.wrapping_add(rounded));
    e.mem.set_u32(array, cursor);
    e.mem.set_u32(array + 8, count as u32 | NOT_OWNED);
    e.mem.set_u32(array + 0xc, cursor);
    let allocator = router_allocator(e);
    let held = (e.mem.u32(array + 8) & CAPACITY_MASK) as i32;
    if held < count {
        let wanted = held.wrapping_add(held).max(count);
        e.call(ARRAY_RESERVE, &args![allocator, array, wanted, 4u32]);
    }
    e.mem.set_u32(array + 4, count as u32);
}

/// `removeReference` on the first `count` entries of a stack-like array (none may be null).
fn release_clones(e: &mut Engine, array: u32, count: i32) {
    let mut k = 0i32;
    while k < count {
        let object = e.mem.u32(e.mem.u32(array).wrapping_add(4 * k as u32));
        e.call(REMOVE_REFERENCE, &args![object]);
        k += 1;
    }
}

/// What `00ce2730` does to one of its stack-like arrays at the end: the size is zeroed when the
/// array still uses its starting storage, the router's cursor goes back to where the array
/// started, and storage `_reserve` took from the allocator (capacity word without the not-owned
/// bit) is freed again. `forget` says to also reset the data pointer and the capacity word (the
/// second array).
fn end_stack_array(e: &mut Engine, array: u32, forget: bool) {
    let started = e.mem.u32(array + 0xc);
    if started == e.mem.u32(array) {
        e.mem.set_u32(array + 4, 0);
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, started);
    let allocator = router_allocator(e);
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if capacity as i32 >= 0 {
        let bytes = (capacity & CAPACITY_MASK).wrapping_mul(4);
        let data = e.mem.u32(array);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    if forget {
        e.mem.set_u32(array, 0);
        e.mem.set_u32(array + 8, NOT_OWNED);
    }
}

// Translated from 00ce2730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaRagdollInstance::clone` (Xbox PDB), `ECX` = the instance, `(world)`: clones every rigid
/// body (virtual function `+0x18`) into a stack-like array and every constraint (`00cf3fa0`, with
/// the clone of the constraint's `i + 1`th body and the clone of that body's parent) into a
/// second, takes 0x30 bytes from the allocator (function `+4`, with the 16-bit size 0x30 at `+4`),
/// builds the new instance there with the constructor `00ce2510` (the clones, the skeleton and
/// this instance's rigid body index array), gives up the references the clones started with,
/// and puts the router's cursor back. Answers the new instance.
pub fn hka_ragdoll_instance_clone(e: &mut Engine, this: Ptr, world: Ptr) -> Ptr {
    let (this, world) = (this.addr(), world.addr());
    let bodies = e.mem.u32(this + 0xc) as i32;
    let constraints = e.mem.u32(this + 0x18) as i32;
    e.with_stack(0x20, |e, frame| {
        let (first, second) = (frame.addr(), frame.addr() + 0x10);
        begin_stack_array(e, first, bodies);
        begin_stack_array(e, second, constraints);
        let mut k = 0i32;
        while k < bodies {
            let body = e.mem.u32(e.mem.u32(this + 8).wrapping_add(4 * k as u32));
            let slot = e.mem.u32(first).wrapping_add(4 * k as u32);
            let clone = e.vcall(body, 0x18, &args![]).u32();
            e.mem.set_u32(slot, clone);
            k += 1;
        }
        let mut k = 0i32;
        while k < constraints {
            let clones = e.mem.u32(first);
            let next = e.mem.u32(clones.wrapping_add(4 * k as u32 + 4));
            let parent = parent_of(e, this, k + 1);
            let parent_clone = e
                .mem
                .u32(clones.wrapping_add((parent as u32).wrapping_mul(4)));
            let original = e.mem.u32(e.mem.u32(this + 0x14).wrapping_add(4 * k as u32));
            let slot = e.mem.u32(second).wrapping_add(4 * k as u32);
            let clone = e
                .call(
                    CLONE_CONSTRAINT,
                    &args![original, next, parent_clone, world],
                )
                .u32();
            e.mem.set_u32(slot, clone);
            k += 1;
        }
        let allocator = router_allocator(e);
        let block = e.vcall(allocator, 4, &args![0x30u32]).u32();
        e.mem.set_u16(block + 4, 0x30);
        let skeleton = e.mem.u32(this + 0x2c);
        let instance = e
            .call(
                CONSTRUCTOR,
                &args![block, first, second, skeleton, this + 0x20],
            )
            .u32();
        release_clones(e, first, bodies);
        release_clones(e, second, constraints);
        end_stack_array(e, second, true);
        end_stack_array(e, first, false);
        Ptr::new(instance)
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ce_1680, hka_ragdoll_instance_add_to_world(Ptr, Ptr, u8) -> u32),
        entry!(0x00ce_1760, fn_00ce1760(Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00ce_18d0, fn_00ce18d0(Ptr, Ptr)),
        entry!(
            0x00ce_18f0,
            hka_ragdoll_instance_get_pose_model_space(Ptr, Ptr, Ptr)
        ),
        entry!(0x00ce_1ad0, fn_00ce1ad0(Ptr, Ptr, Ptr)),
        entry!(0x00ce_1ca0, fn_00ce1ca0(Ptr, Ptr, Ptr, Ptr, Ptr, f32)),
        entry!(0x00ce_2000, fn_00ce2000(Ptr, i32, Ptr)),
        entry!(0x00ce_2150, fn_00ce2150(Ptr, i32, f32, Ptr)),
        entry!(0x00ce_22c0, fn_00ce22c0(Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00ce_2430, fn_00ce2430(Ptr, f32, Ptr)),
        entry!(0x00ce_2510, hka_ragdoll_instance_constructor(Ptr, Ptr, Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00ce_25f0, fn_00ce25f0(Ptr, u32) -> Ptr),
        entry!(0x00ce_2600, fn_00ce2600(Ptr)),
        entry!(0x00ce_26f0, fn_00ce26f0(Ptr, Ptr)),
        entry!(0x00ce_2730, hka_ragdoll_instance_clone(Ptr, Ptr) -> Ptr),
        entry!(0x00ce_29b0, deleting_destructor_00ce29b0(Ptr, u8) -> Ptr),
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
    fn deleting_destructor_00ce29b0_frees_only_when_bit_zero_is_set() {
        check(0x00ce_29b0, 0x00ce_2600);
    }

    use crate::units::havok_sdk::hkpsphereshape::store_bits;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const LINEAR_DOUBLE: u32 = 0x0900_0040;
    const ANGULAR_DOUBLE: u32 = 0x0900_0044;
    const CLONE_BODY_DOUBLE: u32 = 0x0900_0018;
    const ALLOCATE_DOUBLE: u32 = 0x0900_0004;
    const SIZED_ALLOCATE_DOUBLE: u32 = 0x0900_000c;
    const FREE_DOUBLE: u32 = 0x0900_0010;

    fn entry(address: u32, words: &[u32]) -> (u32, Vec<u32>) {
        (address, words.to_vec())
    }

    fn bits(v: Vec4) -> Vec<u32> {
        v.iter().map(|lane| lane.to_bits()).collect()
    }

    fn near(a: Vec4, b: Vec4) -> bool {
        a.iter().zip(b).all(|(x, y)| (x - y).abs() <= 1.0e-6)
    }

    /// The exe constants the translations read: -0.5, the identity quaternion, the scale fill 0.5,
    /// ones, the sign masks, the tolerance 0.001 with its absolute-value mask, the zero velocity
    /// and the smallest rotation 0.0001.
    fn exe_constants(e: &mut Engine) {
        store_vec(e, 0x010c_71e0, [-0.5; 4]);
        store_vec(e, 0x010c_71b0, [0.0, 0.0, 0.0, 1.0]);
        e.mem.set_f32(0x0101_2068, 0.5);
        store_vec(e, 0x010c_7170, [1.0; 4]);
        store_bits(e, 0x010c_44d0, [0x8000_0000; 4]);
        store_bits(e, 0x010c_4c00, [0x8000_0000, 0x8000_0000, 0x8000_0000, 0]);
        store_bits(e, 0x010c_4c10, [0xffff_ffff, 0xffff_ffff, 0xffff_ffff, 0]);
        e.mem.set_f32(0x0101_7d00, 0.001);
        store_bits(e, 0x0101_7d10, [0x7fff_ffff; 4]);
        store_vec(e, 0x0126_7e30, [0.0; 4]);
        e.mem.set_f32(0x0104_da4c, 0.0001);
    }

    /// A double that logs the words it is called with.
    fn log_calls(e: &mut Engine, log: &Log, address: u32) {
        let log = Rc::clone(log);
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            Ret::default()
        });
    }

    /// `setMul` as the sum of its two rotations.
    fn add_quaternions(e: &mut Engine) {
        e.register_double(QUATERNION_SET_MUL, |e, a| {
            let sum = add4(load_vec(e, a[1]), load_vec(e, a[2]));
            store_vec(e, a[0], sum);
            Ret::default()
        });
    }

    /// The motion functions (slots `0x40` and `0x44`), logging the motion and the velocity.
    fn motion_doubles(e: &mut Engine, log: &Log) -> u32 {
        for address in [LINEAR_DOUBLE, ANGULAR_DOUBLE] {
            let log = Rc::clone(log);
            e.register_double(address, move |e, a| {
                let mut words = vec![a[0]];
                words.extend(bits(load_vec(e, a[1])));
                log.borrow_mut().push((address, words));
                Ret::default()
            });
        }
        let vtable = e.mem.alloc(0x80);
        e.mem.set_u32(vtable + 0x40, LINEAR_DOUBLE);
        e.mem.set_u32(vtable + 0x44, ANGULAR_DOUBLE);
        vtable
    }

    struct Rig {
        instance: u32,
        skeleton: u32,
        reference: u32,
        entities: Vec<u32>,
    }

    /// An instance over a skeleton with a bone for each `(parent, rigid body index)`, with
    /// `entity_count` rigid bodies (0x200 bytes each) whose motion has the vtable `motion`. Bone
    /// `k` has the reference translation `(1, 2, 3) * 10^k`, the identity rotation and the scale
    /// `k + 1`.
    fn make_rig(e: &mut Engine, bones: &[(u16, i32)], entity_count: u32, motion: u32) -> Rig {
        let count = bones.len() as u32;
        let instance = e.mem.alloc(0x40);
        let skeleton = e.mem.alloc(0x40);
        let parents = e.mem.alloc(2 * count);
        let indices = e.mem.alloc(4 * count);
        let reference = e.mem.alloc(0x30 * count);
        for (k, (parent, body)) in bones.iter().enumerate() {
            let k = k as u32;
            e.mem.set_u16(parents + 2 * k, *parent);
            e.mem.set_i32(indices + 4 * k, *body);
            let unit = 10f32.powi(k as i32);
            let at = reference + 0x30 * k;
            store_vec(e, at, [unit, 2.0 * unit, 3.0 * unit, 0.0]);
            store_vec(e, at + 0x10, [0.0, 0.0, 0.0, 1.0]);
            store_vec(e, at + 0x20, [k as f32 + 1.0; 4]);
        }
        let data = e.mem.alloc(4 * entity_count.max(1));
        let mut entities = Vec::new();
        for k in 0..entity_count {
            let entity = e.mem.alloc(0x200);
            e.mem.set_u32(entity + 0xe0, motion);
            e.mem.set_u32(data + 4 * k, entity);
            entities.push(entity);
        }
        e.mem.set_u32(skeleton + 0xc, parents);
        e.mem.set_u32(skeleton + 0x1c, count);
        e.mem.set_u32(skeleton + 0x24, reference);
        e.mem.set_u32(instance + 8, data);
        e.mem.set_u32(instance + 0xc, entity_count);
        e.mem.set_u32(instance + 0x20, indices);
        e.mem.set_u32(instance + 0x24, count);
        e.mem.set_u32(instance + 0x2c, skeleton);
        Rig {
            instance,
            skeleton,
            reference,
            entities,
        }
    }

    fn put_transform(e: &mut Engine, at: u32, t: Vec4, q: Vec4, s: Vec4) {
        store_vec(e, at, t);
        store_vec(e, at + 0x10, q);
        store_vec(e, at + 0x20, s);
    }

    fn get_transform(e: &Engine, at: u32) -> [Vec4; 3] {
        [
            load_vec(e, at),
            load_vec(e, at + 0x10),
            load_vec(e, at + 0x20),
        ]
    }

    #[test]
    fn adding_to_a_world_that_already_has_the_first_body_does_nothing() {
        let mut e = engine();
        let log = Log::default();
        let rig = make_rig(&mut e, &[(0xffff, 0)], 2, 0);
        e.mem.set_u32(rig.entities[0] + 8, 0x7000);
        for address in [ADD_ENTITY_BATCH, ADD_CONSTRAINT, UPDATE_COLLISION_FILTER] {
            log_calls(&mut e, &log, address);
        }
        let answer = e
            .call(0x00ce_1680, &args![rig.instance, 0x5000u32, 1u32])
            .u32();
        assert_eq!(answer, 1);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn adding_to_a_world_adds_the_bodies_the_constraints_and_updates_the_filters() {
        let mut e = engine();
        let log = Log::default();
        let rig = make_rig(&mut e, &[(0xffff, 0)], 2, 0);
        let constraints = e.mem.alloc(8);
        e.mem.set_u32(constraints, 0xc1);
        e.mem.set_u32(constraints + 4, 0xc2);
        e.mem.set_u32(rig.instance + 0x14, constraints);
        e.mem.set_u32(rig.instance + 0x18, 2);
        for address in [ADD_ENTITY_BATCH, ADD_CONSTRAINT, UPDATE_COLLISION_FILTER] {
            log_calls(&mut e, &log, address);
        }
        let data = e.mem.u32(rig.instance + 8);
        let (a, b) = (rig.entities[0], rig.entities[1]);
        let answer = e
            .call(0x00ce_1680, &args![rig.instance, 0x5000u32, 0u32])
            .u32();
        assert_eq!(answer, 0);
        assert_eq!(
            log.borrow()[..],
            [
                entry(ADD_ENTITY_BATCH, &[0x5000, data, 2, 1]),
                entry(ADD_CONSTRAINT, &[0x5000, 0xc1]),
                entry(ADD_CONSTRAINT, &[0x5000, 0xc2]),
            ]
        );
        log.borrow_mut().clear();
        e.call(0x00ce_1680, &args![rig.instance, 0x5000u32, 7u32]);
        assert_eq!(
            log.borrow()[3..],
            [
                entry(UPDATE_COLLISION_FILTER, &[0x5000, a, 1, 0]),
                entry(UPDATE_COLLISION_FILTER, &[0x5000, b, 1, 0]),
            ]
        );
        assert_eq!(log.borrow().len(), 5);
    }

    /// Doubles for the callees of the pose builders: `setMul` adds, `00ce5070` writes the
    /// translation `(5, 6, 7)`, the identity rotation and the scale 1, `00cdd600` writes the scale
    /// 3, and all three log their words.
    fn body_doubles(e: &mut Engine, log: &Log) {
        let l = Rc::clone(log);
        e.register_double(SET_FROM_TRANSFORM_NO_SCALE, move |e, a| {
            l.borrow_mut()
                .push((SET_FROM_TRANSFORM_NO_SCALE, a.to_vec()));
            put_transform(
                e,
                a[0],
                [5.0, 6.0, 7.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
                [1.0; 4],
            );
            Ret::default()
        });
        let l = Rc::clone(log);
        e.register_double(ACCUMULATE_SCALE, move |e, a| {
            l.borrow_mut().push((ACCUMULATE_SCALE, a.to_vec()));
            store_vec(e, a[3], [3.0; 4]);
            Ret::default()
        });
    }

    #[test]
    fn the_pose_takes_bodies_where_there_are_and_composes_the_rest_from_the_parent() {
        let mut e = engine();
        exe_constants(&mut e);
        let log = Log::default();
        let rig = make_rig(&mut e, &[(0xffff, -1), (0, 0), (1, -1)], 1, 0);
        add_quaternions(&mut e);
        body_doubles(&mut e, &log);
        let root = e.mem.alloc(0x30);
        put_transform(
            &mut e,
            root,
            [1000.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
            [2.0, 2.0, 2.0, 1.0],
        );
        let out = e.mem.alloc(0x90);
        e.call(0x00ce_1760, &args![rig.instance, out, root, 0u32]);
        let spin = [0.0, 0.0, 0.0, 2.0];
        assert_eq!(
            get_transform(&e, out),
            [[1001.0, 2.0, 3.0, 0.0], spin, [2.0, 2.0, 2.0, 1.0]]
        );
        assert_eq!(
            get_transform(&e, out + 0x30),
            [[5.0, 6.0, 7.0, 0.0], [0.0, 0.0, 0.0, 1.0], [3.0; 4]]
        );
        assert_eq!(
            get_transform(&e, out + 0x60),
            [[105.0, 206.0, 307.0, 0.0], spin, [9.0; 4]]
        );
        let body = rig.entities[0];
        assert_eq!(
            log.borrow()[..],
            [
                entry(SET_FROM_TRANSFORM_NO_SCALE, &[out + 0x30, body + 0xf0]),
                entry(
                    ACCUMULATE_SCALE,
                    &[rig.skeleton, rig.reference, 1, out + 0x50]
                ),
            ]
        );
    }

    #[test]
    fn the_pose_takes_the_given_reference_pose_and_the_copy_of_the_parent_rotation() {
        let mut e = engine();
        exe_constants(&mut e);
        let rig = make_rig(&mut e, &[(0xffff, -1), (0, -1)], 0, 0);
        let seen: Rc<RefCell<Vec<(Vec<u32>, Vec<u32>)>>> = Rc::default();
        let s = Rc::clone(&seen);
        e.register_double(QUATERNION_SET_MUL, move |e, a| {
            s.borrow_mut().push((a.to_vec(), bits(load_vec(e, a[1]))));
            store_vec(e, a[0], [9.0; 4]);
            Ret::default()
        });
        let root = e.mem.alloc(0x30);
        put_transform(&mut e, root, [0.0; 4], [0.5, 0.5, 0.5, 0.5], [1.0; 4]);
        let rest = e.mem.alloc(0x60);
        put_transform(&mut e, rest, [1.0, 0.0, 0.0, 0.0], [0.0; 4], [4.0; 4]);
        put_transform(&mut e, rest + 0x30, [0.0; 4], [0.0; 4], [5.0; 4]);
        let out = e.mem.alloc(0x60);
        e.call(0x00ce_1760, &args![rig.instance, out, root, rest]);
        // A quarter turn about (1, 1, 1) / sqrt 3 takes x to y.
        assert!(near(load_vec(&e, out), [0.0, 1.0, 0.0, 0.5]));
        assert_eq!(load_vec(&e, out + 0x20), [4.0; 4]);
        assert_eq!(load_vec(&e, out + 0x50), [20.0; 4]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 2);
        // `this`, the copy of the parent rotation (not the parent itself) and the reference rotation.
        assert_eq!(seen[0].0[0], out + 0x10);
        assert_ne!(seen[0].0[1], root + 0x10);
        assert_eq!(seen[0].0[2], rest + 0x10);
        assert_eq!(seen[0].1, bits([0.5; 4]));
        assert_eq!(seen[1].0[0], out + 0x40);
        assert_ne!(seen[1].0[1], out + 0x10);
        assert_eq!(seen[1].0[2], rest + 0x40);
        assert_eq!(seen[1].1, bits([9.0; 4]));
    }

    #[test]
    fn the_current_pose_is_the_pose_over_the_root_transform() {
        let mut e = engine();
        let log = Log::default();
        log_calls(&mut e, &log, POSE_FROM_RIGID_BODIES);
        e.call(0x00ce_18d0, &args![0x1000u32, 0x2000u32]);
        assert_eq!(
            log.borrow()[..],
            [entry(
                POSE_FROM_RIGID_BODIES,
                &[0x1000, 0x2000, 0x0126_8700, 0]
            )]
        );
    }

    #[test]
    fn the_model_space_pose_moves_every_bone_by_the_inverse_transform() {
        let mut e = engine();
        exe_constants(&mut e);
        let rig = make_rig(&mut e, &[(0xffff, -1), (0, -1)], 0, 0);
        let out = e.mem.alloc(0x60);
        e.register_double(POSE_FROM_CURRENT_BODIES, move |e, a| {
            assert_eq!(a[1], out);
            put_transform(
                e,
                out,
                [1.0, 2.0, 3.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
                [2.0, 2.0, 2.0, 1.0],
            );
            put_transform(
                e,
                out + 0x30,
                [4.0, 5.0, 6.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
                [4.0, 4.0, 4.0, 1.0],
            );
            Ret::default()
        });
        let seen: Rc<RefCell<Vec<(Vec<u32>, Vec<u32>)>>> = Rc::default();
        let s = Rc::clone(&seen);
        e.register_double(QUATERNION_SET_MUL, move |e, a| {
            s.borrow_mut().push((a.to_vec(), bits(load_vec(e, a[1]))));
            let sum = add4(load_vec(e, a[1]), load_vec(e, a[2]));
            store_vec(e, a[0], sum);
            Ret::default()
        });
        let transform = e.mem.alloc(0x30);
        put_transform(
            &mut e,
            transform,
            [10.0, 20.0, 30.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
            [2.0, 4.0, 8.0, 1.0],
        );
        e.call(0x00ce_18f0, &args![rig.instance, out, transform]);
        let first = get_transform(&e, out);
        assert!(near(first[0], [-9.0, -18.0, -27.0, 0.0]), "{first:?}");
        assert_eq!(first[1], [0.0, 0.0, 0.0, 2.0]);
        assert!(near(first[2], [1.0, 0.5, 0.25, 0.0]), "{first:?}");
        let second = get_transform(&e, out + 0x30);
        assert!(near(second[0], [-6.0, -15.0, -24.0, 0.0]), "{second:?}");
        assert!(near(second[2], [2.0, 1.0, 0.5, 0.0]), "{second:?}");
        // The inverse rotation (the sign bits of x, y and z flipped) is a copy on the stack, the
        // same one for both bones; the result goes into the bone's own rotation.
        let seen = seen.borrow();
        let inverse = bits([-0.0, -0.0, -0.0, 1.0]);
        assert_eq!((seen[0].0[0], seen[0].0[2]), (out + 0x10, out + 0x10));
        assert_eq!(seen[0].1, inverse);
        assert_eq!((seen[1].0[0], seen[1].0[2]), (out + 0x40, out + 0x40));
        assert_eq!(seen[1].1, inverse);
        assert_eq!(seen[0].0[1], seen[1].0[1]);
    }

    /// The doubles of the placing functions: `setMul` adds, the rescaling halves the quaternion
    /// and `setPositionAndRotation` logs the body and the position and rotation it is given.
    fn placing_doubles(e: &mut Engine, log: &Log) {
        add_quaternions(e);
        e.register_double(QUATERNION_RESCALE, |e, a| {
            let halved = mul4(load_vec(e, a[0]), [0.5; 4]);
            store_vec(e, a[0], halved);
            Ret::default()
        });
        let l = Rc::clone(log);
        e.register_double(SET_POSITION_AND_ROTATION, move |e, a| {
            let mut words = vec![a[0]];
            words.extend(bits(load_vec(e, a[1])));
            words.extend(bits(load_vec(e, a[2])));
            l.borrow_mut().push((SET_POSITION_AND_ROTATION, words));
            Ret::default()
        });
        log_calls(e, log, ENTITY_ACTIVATE);
    }

    #[test]
    fn placing_the_bodies_sets_their_transform_and_stops_the_ones_that_move() {
        let mut e = engine();
        exe_constants(&mut e);
        let log = Log::default();
        let motion = motion_doubles(&mut e, &log);
        placing_doubles(&mut e, &log);
        let rig = make_rig(&mut e, &[(0xffff, 0), (0, 1), (1, -1)], 2, motion);
        let (a, b) = (rig.entities[0], rig.entities[1]);
        // The first body has a NaN in its angular velocity, the second a moving linear one and
        // a small angular one.
        store_vec(&mut e, a + 0x1c0, [f32::NAN, 0.0, 0.0, 0.0]);
        store_vec(&mut e, b + 0x1b0, [0.0, 5.0, 0.0, 0.0]);
        store_vec(&mut e, b + 0x1c0, [0.0005, 0.0, 0.0, 0.0]);
        let pose = e.mem.alloc(0x90);
        put_transform(
            &mut e,
            pose,
            [1.0, 2.0, 3.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
            [1.0; 4],
        );
        put_transform(
            &mut e,
            pose + 0x30,
            [4.0, 5.0, 6.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
            [3.0; 4],
        );
        let transform = e.mem.alloc(0x30);
        put_transform(
            &mut e,
            transform,
            [10.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
            [2.0; 4],
        );
        e.call(0x00ce_1ad0, &args![rig.instance, pose, transform]);
        let rotation = bits([0.0, 0.0, 0.0, 1.0]);
        let place = |body: u32, position: Vec4| {
            let mut words = vec![body];
            words.extend(bits(position));
            words.extend(rotation.clone());
            (SET_POSITION_AND_ROTATION, words)
        };
        let zero = bits([0.0; 4]);
        let mut angular = vec![a + 0xe0];
        angular.extend(&zero);
        let mut linear = vec![b + 0xe0];
        linear.extend(&zero);
        assert_eq!(
            log.borrow()[..],
            [
                place(a, [11.0, 2.0, 3.0, 0.0]),
                entry(ENTITY_ACTIVATE, &[a]),
                (ANGULAR_DOUBLE, angular),
                place(b, [14.0, 5.0, 6.0, 0.0]),
                entry(ENTITY_ACTIVATE, &[b]),
                (LINEAR_DOUBLE, linear),
            ]
        );
        assert_eq!(load_vec(&e, pose + 0x60), [0.0; 4]);
    }

    #[test]
    fn placing_the_bodies_between_two_poses_gives_them_the_velocities_that_lead_there() {
        let mut e = engine();
        exe_constants(&mut e);
        let log = Log::default();
        let motion = motion_doubles(&mut e, &log);
        placing_doubles(&mut e, &log);
        let rig = make_rig(&mut e, &[(0xffff, 0), (0, 1)], 2, motion);
        let (a, b) = (rig.entities[0], rig.entities[1]);
        let l = Rc::clone(&log);
        let calls = Rc::new(RefCell::new(0));
        e.register_double(QUATERNION_PRODUCT, move |e, x| {
            let mut words = bits(load_vec(e, x[1]));
            words.extend(bits(load_vec(e, x[2])));
            l.borrow_mut().push((QUATERNION_PRODUCT, words));
            let mut calls = calls.borrow_mut();
            let answer = if *calls == 0 {
                [0.6, 0.0, 0.0, 0.8]
            } else {
                [0.0, 0.0, 0.0, 1.0]
            };
            *calls += 1;
            store_vec(e, x[0], answer);
            Ret::default()
        });
        e.register_double(QUATERNION_AXIS, |e, x| {
            store_vec(e, x[1], [1.0, 0.0, 0.0, 0.0]);
            Ret::default()
        });
        e.register_double(QUATERNION_ANGLE, |_, _| Ret {
            st0: 0.5,
            ..Ret::default()
        });
        let poses = |e: &mut Engine, translations: [Vec4; 2], first_rotation: Vec4| {
            let pose = e.mem.alloc(0x60);
            put_transform(e, pose, translations[0], first_rotation, [1.0; 4]);
            put_transform(
                e,
                pose + 0x30,
                translations[1],
                [0.0, 0.0, 0.0, 1.0],
                [1.0; 4],
            );
            pose
        };
        let pose_a = poses(
            &mut e,
            [[0.0; 4], [1.0, 1.0, 1.0, 0.0]],
            [0.0, 0.0, 0.0, 1.0],
        );
        let pose_b = poses(
            &mut e,
            [[1.0, 2.0, 3.0, 0.0], [1.0, 1.0, 1.0, 0.0]],
            [0.0, 0.0, 2.0, 1.0],
        );
        let transform = e.mem.alloc(0x30);
        put_transform(&mut e, transform, [0.0; 4], [0.0, 0.0, 0.0, 1.0], [1.0; 4]);
        e.call(
            0x00ce_1ca0,
            &args![rig.instance, pose_a, transform, pose_b, transform, 0.5f32],
        );
        let place = |body: u32, position: Vec4, rotation: Vec4| {
            let mut words = vec![body];
            words.extend(bits(position));
            words.extend(bits(rotation));
            (SET_POSITION_AND_ROTATION, words)
        };
        let product = |second: Vec4| {
            let mut words = bits(second);
            words.extend(bits([0.0, 0.0, 0.0, 1.0]));
            (QUATERNION_PRODUCT, words)
        };
        let mut linear = vec![a + 0xe0];
        linear.extend(bits([2.0, 4.0, 6.0, 0.0]));
        let mut angular = vec![a + 0xe0];
        angular.extend(bits([1.0, 0.0, 0.0, 0.0]));
        assert_eq!(
            log.borrow()[..],
            [
                place(a, [1.0, 2.0, 3.0, 0.0], [0.0, 0.0, 1.0, 1.0]),
                entry(ENTITY_ACTIVATE, &[a]),
                (LINEAR_DOUBLE, linear),
                product([0.0, 0.0, 1.0, 1.0]),
                entry(ENTITY_ACTIVATE, &[a]),
                (ANGULAR_DOUBLE, angular),
                place(b, [1.0, 1.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]),
                product([0.0, 0.0, 0.0, 1.0]),
            ]
        );
    }

    #[test]
    fn a_step_that_is_not_a_number_gives_velocities_that_are_never_within_the_tolerance() {
        let mut e = engine();
        exe_constants(&mut e);
        let log = Log::default();
        let motion = motion_doubles(&mut e, &log);
        placing_doubles(&mut e, &log);
        e.register_double(QUATERNION_PRODUCT, |e, x| {
            store_vec(e, x[0], [0.0, 0.0, 0.0, 1.0]);
            Ret::default()
        });
        let rig = make_rig(&mut e, &[(0xffff, 0)], 1, motion);
        let a = rig.entities[0];
        let pose = e.mem.alloc(0x30);
        put_transform(&mut e, pose, [0.0; 4], [0.0, 0.0, 0.0, 1.0], [1.0; 4]);
        let transform = e.mem.alloc(0x30);
        put_transform(&mut e, transform, [0.0; 4], [0.0, 0.0, 0.0, 1.0], [1.0; 4]);
        // A signalling NaN is quieted by the x87 load before the division.
        let step = f32::from_bits(0x7f80_0001);
        e.call(
            0x00ce_1ca0,
            &args![rig.instance, pose, transform, pose, transform, step],
        );
        let log = log.borrow();
        let nan = |words: &[u32]| words[1..].iter().all(|w| f32::from_bits(*w).is_nan());
        let linear = log.iter().find(|(address, _)| *address == LINEAR_DOUBLE);
        let linear = linear.expect("the linear velocity is given");
        assert_eq!(linear.1[0], a + 0xe0);
        assert!(nan(&linear.1), "{:?}", linear.1);
        assert!(log.iter().any(|(address, _)| *address == ENTITY_ACTIVATE));
        // No rotation: the angular velocity is zero and within the tolerance.
        assert!(log.iter().all(|(address, _)| *address != ANGULAR_DOUBLE));
    }

    /// Doubles for the single-bone pose builders: `00ce5070` writes the translation `(5, 6, 7)`,
    /// the identity rotation and the scale 1 and logs `(out, source, first word of source)`,
    /// `00cdd600` writes the scale 3, `00c86490` writes the word 0x1234 into the transform it is
    /// given and logs its words, and `setMul` adds.
    fn single_bone_doubles(e: &mut Engine, log: &Log) {
        add_quaternions(e);
        let l = Rc::clone(log);
        e.register_double(SET_FROM_TRANSFORM_NO_SCALE, move |e, a| {
            let first = e.mem.u32(a[1]);
            l.borrow_mut()
                .push((SET_FROM_TRANSFORM_NO_SCALE, vec![a[0], a[1], first]));
            put_transform(
                e,
                a[0],
                [5.0, 6.0, 7.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
                [1.0; 4],
            );
            Ret::default()
        });
        let l = Rc::clone(log);
        e.register_double(ACCUMULATE_SCALE, move |e, a| {
            l.borrow_mut().push((ACCUMULATE_SCALE, a.to_vec()));
            store_vec(e, a[3], [3.0; 4]);
            Ret::default()
        });
        let l = Rc::clone(log);
        e.register_double(MOTION_APPROX_TRANSFORM_AT, move |e, a| {
            l.borrow_mut()
                .push((MOTION_APPROX_TRANSFORM_AT, vec![a[0], a[1]]));
            e.mem.set_u32(a[2], 0x1234);
            Ret::default()
        });
    }

    #[test]
    fn a_bone_pose_comes_from_its_body_or_from_the_parent_by_recursion() {
        let mut e = engine();
        exe_constants(&mut e);
        let log = Log::default();
        single_bone_doubles(&mut e, &log);
        let rig = make_rig(&mut e, &[(0xffff, -1), (0, 0), (1, -1)], 1, 0);
        let body = rig.entities[0];
        let out = e.mem.alloc(0x30);
        // A bone without body or parent starts from the zero translation, the identity rotation
        // and the scale 0.5.
        e.call(0x00ce_2000, &args![rig.instance, 0u32, out]);
        assert_eq!(
            get_transform(&e, out),
            [[1.0, 2.0, 3.0, 0.0], [0.0, 0.0, 0.0, 2.0], [0.5; 4]]
        );
        assert!(log.borrow().is_empty());
        // A bone with a body takes the body's transform (without scale) and the accumulated scale.
        e.call(0x00ce_2000, &args![rig.instance, 1u32, out]);
        assert_eq!(
            get_transform(&e, out),
            [[5.0, 6.0, 7.0, 0.0], [0.0, 0.0, 0.0, 1.0], [3.0; 4]]
        );
        assert_eq!(
            log.borrow()[..],
            [
                entry(SET_FROM_TRANSFORM_NO_SCALE, &[out, body + 0xf0, 0]),
                entry(
                    ACCUMULATE_SCALE,
                    &[rig.skeleton, rig.reference, 1, out + 0x20]
                ),
            ]
        );
        // A child of that bone is composed with it.
        log.borrow_mut().clear();
        e.call(0x00ce_2000, &args![rig.instance, 2u32, out]);
        assert_eq!(
            get_transform(&e, out),
            [[105.0, 206.0, 307.0, 0.0], [0.0, 0.0, 0.0, 2.0], [9.0; 4]]
        );
        assert_eq!(log.borrow().len(), 2);
    }

    #[test]
    fn a_bone_pose_at_a_time_asks_the_motion_for_the_transform_then() {
        let mut e = engine();
        exe_constants(&mut e);
        let log = Log::default();
        single_bone_doubles(&mut e, &log);
        let rig = make_rig(&mut e, &[(0xffff, -1), (0, 0), (1, -1)], 1, 0);
        let body = rig.entities[0];
        let out = e.mem.alloc(0x30);
        e.call(0x00ce_2150, &args![rig.instance, 0u32, 1.5f32, out]);
        assert_eq!(
            get_transform(&e, out),
            [[1.0, 2.0, 3.0, 0.0], [0.0, 0.0, 0.0, 2.0], [0.5; 4]]
        );
        assert!(log.borrow().is_empty());
        // The time is copied through the x87 stack: a signalling NaN comes out quiet.
        let signalling = f32::from_bits(0x7f80_0001);
        e.call(0x00ce_2150, &args![rig.instance, 2u32, signalling, out]);
        assert_eq!(
            get_transform(&e, out),
            [[105.0, 206.0, 307.0, 0.0], [0.0, 0.0, 0.0, 2.0], [9.0; 4]]
        );
        let log = log.borrow();
        assert_eq!(log.len(), 3);
        assert_eq!(
            log[0],
            entry(MOTION_APPROX_TRANSFORM_AT, &[body + 0xe0, 0x7fc0_0001])
        );
        // The transform the motion filled in is what `00ce5070` is given.
        assert_eq!(log[1].0, SET_FROM_TRANSFORM_NO_SCALE);
        assert_eq!(log[1].1[2], 0x1234);
        assert_eq!(log[2].0, ACCUMULATE_SCALE);
        assert_eq!(log[2].1[1..3], [rig.reference, 1]);
    }

    #[test]
    fn the_pose_at_a_time_asks_for_every_bone() {
        let mut e = engine();
        let log = Log::default();
        log_calls(&mut e, &log, BONE_POSE_AT_TIME);
        let rig = make_rig(&mut e, &[(0xffff, -1), (0, -1), (1, -1)], 0, 0);
        e.call(
            0x00ce_2430,
            &args![rig.instance, f32::from_bits(0x7f80_0001), 0x8000u32],
        );
        assert_eq!(
            log.borrow()[..],
            [
                entry(BONE_POSE_AT_TIME, &[rig.instance, 0, 0x7fc0_0001, 0x8000]),
                entry(BONE_POSE_AT_TIME, &[rig.instance, 1, 0x7fc0_0001, 0x8030]),
                entry(BONE_POSE_AT_TIME, &[rig.instance, 2, 0x7fc0_0001, 0x8060]),
            ]
        );
        e.mem.set_u32(rig.skeleton + 0x1c, 0);
        log.borrow_mut().clear();
        e.call(0x00ce_2430, &args![rig.instance, 1.0f32, 0x8000u32]);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn the_pose_takes_the_current_time_of_the_world_the_first_body_is_in() {
        let mut e = engine();
        let log = Log::default();
        log_calls(&mut e, &log, POSE_FROM_CURRENT_BODIES);
        log_calls(&mut e, &log, POSE_AT_TIME);
        let l = Rc::clone(&log);
        e.register_double(WORLD_CURRENT_TIME, move |_, a| {
            l.borrow_mut().push((WORLD_CURRENT_TIME, a.to_vec()));
            Ret {
                st0: 1.5,
                ..Ret::default()
            }
        });
        // No body at all: the array's first entry is null.
        let rig = make_rig(&mut e, &[(0xffff, -1)], 0, 0);
        e.call(0x00ce_26f0, &args![rig.instance, 0x9000u32]);
        // A body that is in no world.
        let rig2 = make_rig(&mut e, &[(0xffff, 0)], 1, 0);
        e.call(0x00ce_26f0, &args![rig2.instance, 0x9000u32]);
        assert_eq!(
            log.borrow()[..],
            [
                entry(POSE_FROM_CURRENT_BODIES, &[rig.instance, 0x9000]),
                entry(POSE_FROM_CURRENT_BODIES, &[rig2.instance, 0x9000]),
            ]
        );
        log.borrow_mut().clear();
        e.mem.set_u32(rig2.entities[0] + 8, 0x7000);
        e.call(0x00ce_26f0, &args![rig2.instance, 0x9000u32]);
        assert_eq!(
            log.borrow()[..],
            [
                entry(WORLD_CURRENT_TIME, &[0x7000]),
                entry(POSE_AT_TIME, &[rig2.instance, 0x3fc0_0000, 0x9000]),
            ]
        );
    }

    struct Heap {
        allocator: u32,
        arena: u32,
        router: u32,
    }

    /// A memory router whose arena starts at `arena` and whose allocator logs every call:
    /// `allocate(size)` hands out a block, `allocate through a size in memory` rounds the size up
    /// to a multiple of 16, and `free(data, bytes)`.
    fn heap(e: &mut Engine, log: &Log) -> Heap {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE_DOUBLE);
        e.mem.set_u32(vtable + 0xc, SIZED_ALLOCATE_DOUBLE);
        e.mem.set_u32(vtable + 0x10, FREE_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let arena = e.mem.alloc(0x400);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router, arena);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let l = Rc::clone(log);
        e.register_double(ALLOCATE_DOUBLE, move |e, a| {
            l.borrow_mut().push((ALLOCATE_DOUBLE, a.to_vec()));
            Ret {
                eax: e.mem.alloc(a[1]),
                ..Ret::default()
            }
        });
        let l = Rc::clone(log);
        e.register_double(SIZED_ALLOCATE_DOUBLE, move |e, a| {
            let size = e.mem.u32(a[1]);
            l.borrow_mut()
                .push((SIZED_ALLOCATE_DOUBLE, vec![a[0], size]));
            let rounded = (size + 15) & !15;
            e.mem.set_u32(a[1], rounded);
            Ret {
                eax: e.mem.alloc(rounded),
                ..Ret::default()
            }
        });
        log_calls(e, log, FREE_DOUBLE);
        Heap {
            allocator,
            arena,
            router,
        }
    }

    fn put_array(e: &mut Engine, words: &[u32]) -> u32 {
        let data = e.mem.alloc(4 * words.len().max(1) as u32);
        for (k, word) in words.iter().enumerate() {
            e.mem.set_u32(data + 4 * k as u32, *word);
        }
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, words.len() as u32);
        array
    }

    fn words_of(e: &Engine, data: u32, count: u32) -> Vec<u32> {
        (0..count).map(|k| e.mem.u32(data + 4 * k)).collect()
    }

    #[test]
    fn assigning_bodies_and_constraints_grows_the_arrays_and_takes_references() {
        let mut e = engine();
        let log = Log::default();
        let heap = heap(&mut e, &log);
        log_calls(&mut e, &log, ADD_REFERENCE);
        log_calls(&mut e, &log, REMOVE_REFERENCE);
        let instance = e.mem.alloc(0x40);
        let old_bodies = e.mem.alloc(4);
        for (offset, word) in [
            (0x8, old_bodies),
            (0xc, 1),
            (0x10, 1),
            (0x14, 0),
            (0x18, 0),
            (0x1c, NOT_OWNED),
            (0x2c, 0x7001),
        ] {
            e.mem.set_u32(instance + offset, word);
        }
        let bodies = put_array(&mut e, &[0xb1, 0, 0xb3]);
        let constraints = put_array(&mut e, &[0xc1]);
        e.call(
            0x00ce_22c0,
            &args![instance, bodies, constraints, 0x7002u32],
        );
        assert_eq!(
            log.borrow()[..],
            [
                entry(ADD_REFERENCE, &[0x7002]),
                entry(REMOVE_REFERENCE, &[0x7001]),
                entry(FREE_DOUBLE, &[heap.allocator, old_bodies, 4]),
                entry(SIZED_ALLOCATE_DOUBLE, &[heap.allocator, 12]),
                entry(SIZED_ALLOCATE_DOUBLE, &[heap.allocator, 4]),
                entry(ADD_REFERENCE, &[0xb1]),
                entry(ADD_REFERENCE, &[0xb3]),
                entry(ADD_REFERENCE, &[0xc1]),
            ]
        );
        let (data, size, capacity) = (
            e.mem.u32(instance + 8),
            e.mem.u32(instance + 0xc),
            e.mem.u32(instance + 0x10),
        );
        assert_eq!((size, capacity), (3, 4));
        assert_eq!(words_of(&e, data, 3), [0xb1, 0, 0xb3]);
        let data = e.mem.u32(instance + 0x14);
        assert_eq!(e.mem.u32(instance + 0x18), 1);
        assert_eq!(e.mem.u32(instance + 0x1c), 4);
        assert_eq!(words_of(&e, data, 1), [0xc1]);
        assert_eq!(e.mem.u32(instance + 0x2c), 0x7002);
    }

    #[test]
    fn assigning_into_room_that_is_big_enough_only_copies() {
        let mut e = engine();
        let log = Log::default();
        heap(&mut e, &log);
        log_calls(&mut e, &log, ADD_REFERENCE);
        log_calls(&mut e, &log, REMOVE_REFERENCE);
        let instance = e.mem.alloc(0x40);
        let room = e.mem.alloc(32);
        let more = e.mem.alloc(32);
        for (offset, word) in [(0x8, room), (0x10, 8), (0x14, more), (0x1c, 0x8000_0008)] {
            e.mem.set_u32(instance + offset, word);
        }
        let bodies = put_array(&mut e, &[0xb1, 0xb2]);
        let constraints = put_array(&mut e, &[]);
        e.call(0x00ce_22c0, &args![instance, bodies, constraints, 0u32]);
        assert_eq!(
            log.borrow()[..],
            [entry(ADD_REFERENCE, &[0xb1]), entry(ADD_REFERENCE, &[0xb2])]
        );
        assert_eq!(e.mem.u32(instance + 8), room);
        assert_eq!(words_of(&e, room, 2), [0xb1, 0xb2]);
        assert_eq!(e.mem.u32(instance + 0x14), more);
        assert_eq!(e.mem.u32(instance + 0x18), 0);
        assert_eq!(e.mem.u32(instance + 0x2c), 0);
    }

    #[test]
    fn construction_empties_the_instance_before_assigning_and_copies_the_mapping() {
        let mut e = engine();
        let log = Log::default();
        let heap = heap(&mut e, &log);
        let snapshot: Rc<RefCell<Vec<u32>>> = Rc::default();
        let s = Rc::clone(&snapshot);
        let l = Rc::clone(&log);
        e.register_double(ASSIGN_BODIES_AND_CONSTRAINTS, move |e, a| {
            l.borrow_mut()
                .push((ASSIGN_BODIES_AND_CONSTRAINTS, a.to_vec()));
            *s.borrow_mut() = words_of(e, a[0], 12);
            Ret::default()
        });
        let instance = e.mem.alloc(0x40);
        for k in 0..16 {
            e.mem.set_u32(instance + 4 * k, 0xffff_ffff);
        }
        let mapping = put_array(&mut e, &[7, 8]);
        let answer = e
            .call(
                0x00ce_2510,
                &args![instance, 0x1000u32, 0x2000u32, 0x3000u32, mapping],
            )
            .u32();
        assert_eq!(answer, instance);
        assert_eq!(
            snapshot.borrow()[..],
            [
                0x010c_b830,
                0x0001_ffff,
                0,
                0,
                NOT_OWNED,
                0,
                0,
                NOT_OWNED,
                0,
                0,
                NOT_OWNED,
                0
            ]
        );
        assert_eq!(
            log.borrow()[..],
            [
                entry(
                    ASSIGN_BODIES_AND_CONSTRAINTS,
                    &[instance, 0x1000, 0x2000, 0x3000]
                ),
                entry(SIZED_ALLOCATE_DOUBLE, &[heap.allocator, 8]),
            ]
        );
        let data = e.mem.u32(instance + 0x20);
        assert_eq!(words_of(&e, data, 2), [7, 8]);
        assert_eq!(e.mem.u32(instance + 0x24), 2);
        assert_eq!(e.mem.u32(instance + 0x28), 4);
    }

    #[test]
    fn the_small_constructor_only_sets_the_vtable() {
        let mut e = engine();
        let instance = e.mem.alloc(0x40);
        e.mem.set_u32(instance, 1);
        let answer = e.call(0x00ce_25f0, &args![instance, 0x1234u32]).u32();
        assert_eq!(answer, instance);
        assert_eq!(e.mem.u32(instance), 0x010c_b830);
    }

    #[test]
    fn the_destructor_gives_up_references_and_frees_what_it_owns() {
        let mut e = engine();
        let log = Log::default();
        let heap = heap(&mut e, &log);
        log_calls(&mut e, &log, REMOVE_REFERENCE);
        let instance = e.mem.alloc(0x40);
        let bodies = put_array(&mut e, &[0xb1, 0xb2]);
        let constraints = put_array(&mut e, &[0xc1]);
        let mapping = put_array(&mut e, &[1, 2, 3]);
        let words = |array: u32| (e.mem.u32(array), e.mem.u32(array + 4));
        let (bodies_data, _) = words(bodies);
        let (mapping_data, _) = words(mapping);
        for (offset, word) in [
            (0x8, bodies_data),
            (0xc, 2),
            (0x10, 2),
            (0x14, e.mem.u32(constraints)),
            (0x18, 1),
            (0x1c, 0x8000_0001),
            (0x20, mapping_data),
            (0x24, 3),
            (0x28, 4),
            (0x2c, 0x7001),
        ] {
            e.mem.set_u32(instance + offset, word);
        }
        e.call(0x00ce_2600, &args![instance]);
        assert_eq!(
            log.borrow()[..],
            [
                entry(REMOVE_REFERENCE, &[0xb1]),
                entry(REMOVE_REFERENCE, &[0xb2]),
                entry(REMOVE_REFERENCE, &[0xc1]),
                entry(REMOVE_REFERENCE, &[0x7001]),
                entry(FREE_DOUBLE, &[heap.allocator, mapping_data, 16]),
                entry(FREE_DOUBLE, &[heap.allocator, bodies_data, 8]),
            ]
        );
        for field in [0x8, 0x14, 0x20] {
            assert_eq!(e.mem.u32(instance + field), 0);
            assert_eq!(e.mem.u32(instance + field + 4), 0);
            assert_eq!(e.mem.u32(instance + field + 8), NOT_OWNED);
        }
        assert_eq!(e.mem.u32(instance + 0x2c), 0);
        assert_eq!(e.mem.u32(instance), 0x0102_e388);
    }

    #[test]
    fn cloning_clones_the_bodies_and_constraints_and_builds_the_instance_from_them() {
        let mut e = engine();
        let log = Log::default();
        let heap = heap(&mut e, &log);
        log_calls(&mut e, &log, ADD_REFERENCE);
        log_calls(&mut e, &log, REMOVE_REFERENCE);
        let rig = make_rig(&mut e, &[(0xffff, 0), (0, 1)], 2, 0);
        let (first, second) = (rig.entities[0], rig.entities[1]);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x18, CLONE_BODY_DOUBLE);
        for entity in [first, second] {
            e.mem.set_u32(entity, vtable);
        }
        let l = Rc::clone(&log);
        let made = Rc::new(RefCell::new(0xb0));
        e.register_double(CLONE_BODY_DOUBLE, move |_, a| {
            l.borrow_mut().push((CLONE_BODY_DOUBLE, a.to_vec()));
            *made.borrow_mut() += 1;
            Ret {
                eax: *made.borrow(),
                ..Ret::default()
            }
        });
        let l = Rc::clone(&log);
        e.register_double(CLONE_CONSTRAINT, move |_, a| {
            l.borrow_mut().push((CLONE_CONSTRAINT, a.to_vec()));
            Ret {
                eax: 0xd1,
                ..Ret::default()
            }
        });
        let constraints = e.mem.alloc(4);
        e.mem.set_u32(constraints, 0xc0);
        e.mem.set_u32(rig.instance + 0x14, constraints);
        e.mem.set_u32(rig.instance + 0x18, 1);
        let clone = e.call(0x00ce_2730, &args![rig.instance, 0x5000u32]).u32();
        let a = heap.allocator;
        assert_eq!(
            log.borrow()[..],
            [
                entry(CLONE_BODY_DOUBLE, &[first]),
                entry(CLONE_BODY_DOUBLE, &[second]),
                entry(CLONE_CONSTRAINT, &[0xc0, 0xb2, 0xb1, 0x5000]),
                entry(ALLOCATE_DOUBLE, &[a, 0x30]),
                entry(ADD_REFERENCE, &[rig.skeleton]),
                entry(SIZED_ALLOCATE_DOUBLE, &[a, 8]),
                entry(SIZED_ALLOCATE_DOUBLE, &[a, 4]),
                entry(ADD_REFERENCE, &[0xb1]),
                entry(ADD_REFERENCE, &[0xb2]),
                entry(ADD_REFERENCE, &[0xd1]),
                entry(SIZED_ALLOCATE_DOUBLE, &[a, 8]),
                entry(REMOVE_REFERENCE, &[0xb1]),
                entry(REMOVE_REFERENCE, &[0xb2]),
                entry(REMOVE_REFERENCE, &[0xd1]),
            ]
        );
        assert_eq!(e.mem.u16(clone + 4), 0x30);
        assert_eq!(e.mem.u32(clone), 0x010c_b830);
        assert_eq!(words_of(&e, e.mem.u32(clone + 8), 2), [0xb1, 0xb2]);
        assert_eq!(words_of(&e, e.mem.u32(clone + 0x14), 1), [0xd1]);
        assert_eq!(words_of(&e, e.mem.u32(clone + 0x20), 2), [0, 1]);
        assert_eq!(e.mem.u32(clone + 0x2c), rig.skeleton);
        // The arena is back where it started.
        assert_eq!(e.mem.u32(heap.router), heap.arena);
    }
}
