//! `Havok SDK/hkpmousespringaction.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Runs `f` on an exe-shaped frame: a function that aligns `ESP` (`AND ESP,0xfffffff0`) and
/// then reserves `below` bytes (its `SUB ESP` and pushes) has its frame `below` bytes under a
/// 16-byte boundary; `f` gets the frame's lowest address.
fn with_aligned_frame<R>(e: &mut Engine, below: u32, f: impl FnOnce(&mut Engine, u32) -> R) -> R {
    e.with_stack(below + 0x10, |e, raw| {
        let boundary = (raw.addr() + below + 0xf) & !0xf;
        f(e, boundary - below)
    })
}

/// The frame base [`with_aligned_frame`] picks for a block at `raw` (tests).
#[cfg(test)]
fn aligned_frame_at(raw: u32, below: u32) -> u32 {
    ((raw + below + 0xf) & !0xf) - below
}

// Translated from 00cbb8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpUnaryAction::getClassType` (Xbox PDB): takes no arguments and returns the constant 0x0126_8a14.
pub fn hkp_unary_action_get_class_type(_e: &mut Engine) -> u32 {
    0x0126_8a14
}

/// The memory router (`00c85750`): its field `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The vtable of `hkpMouseSpringAction` (`010c8e94`).
const MOUSE_SPRING_VTABLE: u32 = 0x010c_8e94;
/// The vtable stored in the collector the constructor hands to the collision function
/// (`010c476c`).
const COLLECTOR_VTABLE: u32 = 0x010c_476c;
/// The `hkpUnaryAction` constructor (`00d2b260`: `this`, `(entity, 0)`), its
/// `entityRemovedCallback` (`00d2b1f0`, one word) and its destructor (`00d2b2b0`).
const UNARY_ACTION_CONSTRUCT: u32 = 0x00d2_b260;
const UNARY_ACTION_ENTITY_REMOVED: u32 = 0x00d2_b1f0;
const UNARY_ACTION_DESTRUCT: u32 = 0x00d2_b2b0;
/// This unit's constructor (`00cbb4e0`: `this`, the entity word).
const MOUSE_SPRING_CONSTRUCT: u32 = 0x00cb_b4e0;
/// `hkStringPtr::operator=` (`00cb0c10`, one word) and the string it is given.
const STRING_ASSIGN: u32 = 0x00cb_0c10;
const ACTION_NAME: u32 = 0x011b_1f3c;
/// `hkpMouseSpringAction::setMousePosition` (`00cbb150`, one word).
const SPRING_SET_MOUSE_POSITION: u32 = 0x00cb_b150;
/// The float constants of the constructors: 0.5, 0.3, 250.0 and 0.95.
const DAMPING_DEFAULT: u32 = 0x0101_6248;
const ELASTICITY_DEFAULT: u32 = 0x0101_7870;
const MAX_FORCE_DEFAULT: u32 = 0x0101_b2d0;
const LIMIT_DEFAULT: u32 = 0x0101_ffc0;
/// `hkpSphereShape` constructor (`00cd8160`, `this`, radius), `hkMatrix3::getIdentity`
/// (`00cd4b50`, returns the matrix), `hkpCollidable` constructor (`00cd2d80`) and the
/// `BoundingVolumeData` constructor it runs (`00d01a60`).
const SPHERE_CONSTRUCT: u32 = 0x00cd_8160;
const GET_IDENTITY: u32 = 0x00cd_4b50;
const COLLIDABLE_CONSTRUCT: u32 = 0x00cd_2d80;
const BOUNDING_VOLUME_CONSTRUCT: u32 = 0x00d0_1a60;
/// The float the collector keeps at +4 and +0x2c (`0104da48`) and the one the collidable
/// keeps at +0x4c (`01012054`).
const COLLECTOR_CONSTANT: u32 = 0x0104_da48;
const COLLIDABLE_CONSTANT: u32 = 0x0101_2054;

/// The allocator of the memory router.
fn allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// Makes the word array of `action` (data at +0x54, size at +0x58, capacity word at +0x5c: low
/// 30 bits the capacity, bit 31 set when the storage is not owned) hold `count` words copied
/// from `source`. When the capacity is below `count` the old storage is freed through
/// `allocator` (slot 0x10, `capacity * 4` bytes) unless bit 31 is set, and new storage is
/// requested (slot 0xC with a pointer to the byte count, which comes back as the granted
/// size; the new capacity is that size divided by four, rounded towards zero).
fn assign_words(e: &mut Engine, action: u32, allocator: u32, count: i32, source: u32) {
    let flags = e.mem.u32(action + 0x5c);
    let capacity = (flags & 0x3fff_ffff) as i32;
    if capacity < count {
        if (flags as i32) >= 0 {
            let data = e.mem.u32(action + 0x54);
            e.vcall(
                allocator,
                0x10,
                &args![data, (capacity as u32).wrapping_mul(4)],
            );
        }
        e.with_stack(4, |e, request| {
            e.mem.set_i32(request.addr(), count.wrapping_mul(4));
            let block = e.vcall(allocator, 0xc, &args![request]).u32();
            e.mem.set_u32(action + 0x54, block);
            let granted = e.mem.i32(request.addr());
            e.mem.set_i32(
                action + 0x5c,
                granted.wrapping_add((granted >> 31) & 3) >> 2,
            );
        });
    }
    let data = e.mem.u32(action + 0x54);
    e.mem.set_i32(action + 0x58, count);
    for index in 0..count.max(0) as u32 {
        let word = e.mem.u32(source + 4 * index);
        e.mem.set_u32(data + 4 * index, word);
    }
}

// Translated from 00cbb1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMouseSpringAction::setMaxRelativeForce` (Xbox PDB): stores the float at +0x48 (copied
/// through the x87 stack).
pub fn hkp_mouse_spring_action_set_max_relative_force(e: &mut Engine, this: Ptr, force: u32) {
    e.mem.set_u32(this.addr() + 0x48, x87_quiet_bits(force));
}

// Translated from 00cbb1c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMouseSpringAction::entityRemovedCallback` (Xbox PDB): runs the `hkpUnaryAction`
/// callback with `entity`, then clears the word at +0x18.
pub fn hkp_mouse_spring_action_entity_removed_callback(e: &mut Engine, this: Ptr, entity: u32) {
    e.call(UNARY_ACTION_ENTITY_REMOVED, &args![this, entity]);
    e.mem.set_u32(this.addr() + 0x18, 0);
}

// Translated from 00cbb4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMouseSpringAction::hkpMouseSpringAction` (Xbox PDB): runs the `hkpUnaryAction`
/// constructor with `(entity, 0)`, then sets the floats at +0x40, +0x44, +0x48 and +0x4C from
/// the constants `01016248`, `01017870`, `0101b2d0` and `0101ffc0` (copied through the x87
/// stack), the vtable `010c8e94`, an empty array at +0x54 (data 0, size 0, capacity word
/// `0x80000000`), zeros in the vectors at +0x20 and +0x30, `0xFFFFFFFF` at +0x50 and the name
/// string at +0x14. Returns `this`.
pub fn hkp_mouse_spring_action_hkp_mouse_spring_action(
    e: &mut Engine,
    this: Ptr,
    entity: u32,
) -> Ptr {
    let base = this.addr();
    e.call(UNARY_ACTION_CONSTRUCT, &args![this, entity, 0u32]);
    for (offset, constant) in [
        (0x40, DAMPING_DEFAULT),
        (0x44, ELASTICITY_DEFAULT),
        (0x48, MAX_FORCE_DEFAULT),
        (0x4c, LIMIT_DEFAULT),
    ] {
        let bits = x87_quiet_bits(e.mem.u32(constant));
        e.mem.set_u32(base + offset, bits);
    }
    e.mem.set_u32(base, MOUSE_SPRING_VTABLE);
    e.mem.set_u32(base + 0x54, 0);
    e.mem.set_u32(base + 0x58, 0);
    e.mem.set_u32(base + 0x5c, 0x8000_0000);
    for offset in (0..0x20).step_by(4) {
        e.mem.set_u32(base + 0x20 + offset, 0);
    }
    e.mem.set_u32(base + 0x50, 0xffff_ffff);
    e.call(STRING_ASSIGN, &args![base + 0x14, ACTION_NAME]);
    this
}

// Translated from 00cbb560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMouseSpringAction::hkpMouseSpringAction_ov2` (Xbox PDB; `this` in `ECX`, `RET 0x1c`):
/// runs the `hkpUnaryAction` constructor with `(entity, 0)`, sets the vtable and copies the
/// vectors `first_vector` and `second_vector` to +0x20 and +0x30, the three floats to +0x40,
/// +0x44 and +0x4C (through the x87 stack) and the constant `0101b2d0` to +0x48; the array at
/// +0x54 is empty with the capacity word `0x80000000`. When `words` (an array object: data
/// pointer at +0, count at +4) is not null its words are copied into the array at +0x54.
/// +0x50 gets `0xFFFFFFFF`. When the world at `entity + 8` is not null a sphere shape (radius
/// 1.0) and an identity-oriented transform at `second_vector` build a collidable on the stack;
/// the collision function in the world's table (`world[0x80] + 0x9a8 + 0x14 * index`, with
/// `index` the byte at `((shape type + 0xD) << 5) + entity[0x10][0xC]` of that table) is called
/// with the collidable, `entity + 0x10`, a copy of the words the world keeps at `world[0x78]`
/// and a collector; when the collector's count (+0x30) is not zero its word at +0x5C goes to
/// +0x50. Finally the name string is assigned at +0x14. Returns `this`.
pub fn hkp_mouse_spring_action_hkp_mouse_spring_action_ov2(
    e: &mut Engine,
    this: Ptr,
    first_vector: Ptr,
    second_vector: Ptr,
    first_float: u32,
    second_float: u32,
    third_float: u32,
    entity: u32,
    words: u32,
) -> Ptr {
    let base = this.addr();
    e.call(UNARY_ACTION_CONSTRUCT, &args![this, entity, 0u32]);
    e.mem.set_u32(base, MOUSE_SPRING_VTABLE);
    for offset in (0..0x10).step_by(4) {
        let first = e.mem.u32(first_vector.addr() + offset);
        e.mem.set_u32(base + 0x20 + offset, first);
        let second = e.mem.u32(second_vector.addr() + offset);
        e.mem.set_u32(base + 0x30 + offset, second);
    }
    e.mem.set_u32(base + 0x40, x87_quiet_bits(first_float));
    e.mem.set_u32(base + 0x44, x87_quiet_bits(second_float));
    e.mem.set_u32(base + 0x4c, x87_quiet_bits(third_float));
    e.mem.set_u32(base + 0x54, 0);
    let constant = x87_quiet_bits(e.mem.u32(MAX_FORCE_DEFAULT));
    e.mem.set_u32(base + 0x58, 0);
    e.mem.set_u32(base + 0x5c, 0x8000_0000);
    e.mem.set_u32(base + 0x48, constant);
    if words != 0 {
        let allocator = allocator(e);
        let count = e.mem.i32(words + 4);
        let source = e.mem.u32(words);
        assign_words(e, base, allocator, count, source);
    }
    let world = e.mem.u32(entity + 8);
    e.mem.set_u32(base + 0x50, 0xffff_ffff);
    if world != 0 {
        e.with_stack(0x190, |e, frame| {
            // Frame (offsets from the aligned frame base): `+0x10` a copy of the world's
            // words, `+0x60` the sphere shape, `+0x80` the transform (three columns and the
            // position), `+0xC0` the collector, `+0x140` the collidable.
            let f = frame.addr();
            let (copy, sphere, transform, collector, collidable) =
                (f + 0x10, f + 0x60, f + 0x80, f + 0xc0, f + 0x140);
            e.call(SPHERE_CONSTRUCT, &args![sphere, 1.0f32]);
            let identity = e.call(GET_IDENTITY, &args![]).u32();
            for offset in (0..0x30).step_by(4) {
                let word = e.mem.u32(identity + offset);
                e.mem.set_u32(transform + offset, word);
            }
            for offset in (0..0x10).step_by(4) {
                let word = e.mem.u32(second_vector.addr() + offset);
                e.mem.set_u32(transform + 0x30 + offset, word);
            }
            e.call(
                COLLIDABLE_CONSTRUCT,
                &args![collidable, sphere, transform, 0u32],
            );
            let constant = x87_quiet_bits(e.mem.u32(COLLECTOR_CONSTANT));
            let table = e.mem.u32(world + 0x80);
            let shape_type = e.mem.u32(sphere + 0xc);
            let owner = e.mem.u32(entity + 0x10);
            let index = shape_type
                .wrapping_add(0xd)
                .wrapping_shl(5)
                .wrapping_add(e.mem.u32(owner + 0xc));
            let kind = u32::from(e.mem.u8(table.wrapping_add(index)));
            let function = e.mem.u32(
                table
                    .wrapping_add(kind.wrapping_mul(0x14))
                    .wrapping_add(0x9a8),
            );
            let source = e.mem.u32(world + 0x78);
            for offset in (0..0x18).step_by(4) {
                let word = e.mem.u32(source + offset);
                e.mem.set_u32(copy + offset, word);
            }
            for (to, from) in [(0x20, 0x20), (0x30, 0x30), (0x40, 0x40)] {
                for offset in (0..0x10).step_by(4) {
                    let word = e.mem.u32(source + from + offset);
                    e.mem.set_u32(copy + to + offset, word);
                }
            }
            e.mem.set_u32(collector + 0x2c, constant);
            e.mem.set_u32(collector + 4, constant);
            e.mem.set_u32(collector, COLLECTOR_VTABLE);
            e.mem.set_u32(collector + 0x30, 0);
            e.call(function, &args![collidable, entity + 0x10, copy, collector]);
            if e.mem.u32(collector + 0x30) != 0 {
                let hit = e.mem.u32(collector + 0x5c);
                e.mem.set_u32(base + 0x50, hit);
            }
        });
    }
    e.call(STRING_ASSIGN, &args![base + 0x14, ACTION_NAME]);
    this
}

// Translated from 00cbb790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMouseSpringAction::clone` (Xbox PDB): `first_array` and `second_array` are array objects
/// (data pointer at +0, element count at +4; their identities are unconfirmed). Returns 0
/// unless the first holds exactly one element and the second none. Otherwise allocates 0x60
/// bytes from the memory router's allocator (virtual +4), stores 0x60 as the halfword at +4,
/// runs the constructor with the entity word the first element's pointer points to, copies
/// the vectors at +0x20 and +0x30, the floats at +0x40..+0x4C (through the x87 stack), the
/// words at +0x10 and +0x50 and the word array at +0x54 (see `assign_words`), and returns the
/// new action.
pub fn hkp_mouse_spring_action_clone(
    e: &mut Engine,
    this: Ptr,
    first_array: Ptr,
    second_array: Ptr,
) -> u32 {
    let base = this.addr();
    if e.mem.u32(first_array.addr() + 4) != 1 || e.mem.u32(second_array.addr() + 4) != 0 {
        return 0;
    }
    let block_allocator = allocator(e);
    let block = e.vcall(block_allocator, 4, &args![0x60u32]).u32();
    e.mem.set_u16(block + 4, 0x60);
    let elements = e.mem.u32(first_array.addr());
    let entity = e.mem.u32(elements);
    let copy = e.call(MOUSE_SPRING_CONSTRUCT, &args![block, entity]).u32();
    for offset in (0..0x20).step_by(4) {
        let word = e.mem.u32(base + 0x20 + offset);
        e.mem.set_u32(copy + 0x20 + offset, word);
    }
    for offset in [0x40, 0x44, 0x48, 0x4c] {
        let bits = x87_quiet_bits(e.mem.u32(base + offset));
        e.mem.set_u32(copy + offset, bits);
    }
    let user_data = e.mem.u32(base + 0x10);
    e.mem.set_u32(copy + 0x10, user_data);
    let hit = e.mem.u32(base + 0x50);
    e.mem.set_u32(copy + 0x50, hit);
    let array_allocator = allocator(e);
    let count = e.mem.i32(base + 0x58);
    let source = e.mem.u32(base + 0x54);
    assign_words(e, copy, array_allocator, count, source);
    copy
}

// Translated from 00cbb8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMouseSpringAction::~hkpMouseSpringAction` with the deleting flag (Xbox PDB name
/// `_scalar_deleting_destructor_`): sets the array size at +0x58 to 0, frees the array storage
/// (allocator slot 0x10, `capacity * 4` bytes) unless bit 31 of the capacity word at +0x5C is
/// set, clears the data pointer and sets the capacity word to `0x80000000`, runs the
/// `hkpUnaryAction` destructor and, when bit 0 of `flags` is set, returns the block to the
/// allocator (slot 8) with the halfword at +4 as its size. Returns `this`.
pub fn hkp_mouse_spring_action_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u8,
) -> Ptr {
    let base = this.addr();
    let array_allocator = allocator(e);
    let capacity_word = e.mem.u32(base + 0x5c);
    e.mem.set_u32(base + 0x58, 0);
    if (capacity_word as i32) >= 0 {
        let data = e.mem.u32(base + 0x54);
        e.vcall(
            array_allocator,
            0x10,
            &args![data, (capacity_word & 0x3fff_ffff).wrapping_mul(4)],
        );
    }
    e.mem.set_u32(base + 0x54, 0);
    e.mem.set_u32(base + 0x5c, 0x8000_0000);
    e.call(UNARY_ACTION_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        let block_allocator = allocator(e);
        let size = u32::from(e.mem.u16(base + 4));
        e.vcall(block_allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00cd2d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCollidable::hkpCollidable` (Xbox PDB; `this` in `ECX`, `RET 0xc`): stores `first` at +0
/// and `second` at +8, `-1` at +4 and as the byte at +0x1A, zero at +0xC, +0x14 and +0x1C and
/// as the byte at +0x10, the low byte of `third` at +0x18, runs the `BoundingVolumeData`
/// constructor on `this + 0x20`, stores the constant at `01012054` (through the x87 stack) at
/// +0x4C and clears the byte at +0x11 and the halfword at +0x12. The byte at +0x19 is `0x7F`
/// while the constructor runs and `0xEC` afterwards. Returns `this`.
pub fn hkp_collidable_hkp_collidable(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    third: u32,
) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base + 8, second);
    e.mem.set_u32(base, first);
    e.mem.set_u32(base + 0xc, 0);
    e.mem.set_u32(base + 4, 0xffff_ffff);
    e.mem.set_u8(base + 0x10, 0);
    e.mem.set_u8(base + 0x1a, 0xff);
    e.mem.set_u32(base + 0x14, 0);
    e.mem.set_u8(base + 0x18, third as u8);
    e.mem.set_u32(base + 0x1c, 0);
    e.mem.set_u8(base + 0x19, 0x7f);
    e.call(BOUNDING_VOLUME_CONSTRUCT, &args![base + 0x20]);
    let constant = x87_quiet_bits(e.mem.u32(COLLIDABLE_CONSTANT));
    e.mem.set_u32(base + 0x4c, constant);
    e.mem.set_u8(base + 0x19, 0xec);
    e.mem.set_u8(base + 0x11, 0);
    e.mem.set_u16(base + 0x12, 0);
    this
}

// Translated from 00d66bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The engine map names this `hkpMouseSpringAction::setMousePosition`, but `this` is the
/// mouse picking viewer (fastcall, `RET 4`): when the action pointer at `this + 0x2C` is not
/// null and its word at +8 is not zero, it jumps to `00cbb150` with the action as `this` and
/// the same stack word `position`.
pub fn hkp_mouse_spring_action_set_mouse_position(e: &mut Engine, this: Ptr, position: u32) {
    let action = e.mem.u32(this.addr() + 0x2c);
    if action != 0 && e.mem.u32(action + 8) != 0 {
        e.call(SPRING_SET_MOUSE_POSITION, &args![action, position]);
    }
}

use super::hkquaternion::{add_ps, load_vec, mul_ps, store_vec, sub_ps, Vec4};

/// Callees of `applyAction`, all with `this` first: `hkVector4::setTransformedPos` (`00c7f640`,
/// `this` = the result, `(transform, point)`), the entity function at `00ca86c0` (no map name; an
/// answer of 0 means the entity own motion at +0xe0 is used, otherwise the answer is the motion),
/// `hkMatrix3::setCrossSkewSymmetric` (`00cd4cc0`, `(vector)`), `hkMatrix3::setMul_ov2`
/// (`00cd55b0`, `this` = the result, `(first, second)`), `hkMatrix3::sub` (`00cd5180`, `(other)`),
/// `hkMatrix3::invert` (`00cd4f10`, `(epsilon)`, answers 0 on success), `hkpRigidBody::
/// setLinearVelocity` (`005616d0`) and `setAngularVelocity` (`00561800`), the entity function at
/// `00ca0c40` (no map name, `(point, result)`), `hkVector4::setMul3` (`00c7f750`, `this` = the
/// result, `(matrix, vector)`), `hkpMotion::getMass` (`00c86430`, the mass in `ST0`) and
/// `hkpEntity::activate` (`00c9c1d0`).
const VECTOR_SET_TRANSFORMED_POS: u32 = 0x00c7_f640;
const ENTITY_GET_MOTION_00CA86C0: u32 = 0x00ca_86c0;
const MATRIX_SET_CROSS_SKEW_SYMMETRIC: u32 = 0x00cd_4cc0;
const MATRIX_SET_MUL: u32 = 0x00cd_55b0;
const MATRIX_SUB: u32 = 0x00cd_5180;
const MATRIX_INVERT: u32 = 0x00cd_4f10;
const RIGID_BODY_SET_LINEAR_VELOCITY: u32 = 0x0056_16d0;
const RIGID_BODY_SET_ANGULAR_VELOCITY: u32 = 0x0056_1800;
const ENTITY_POINT_QUERY_00CA0C40: u32 = 0x00ca_0c40;
const VECTOR_SET_MUL3: u32 = 0x00c7_f750;
const MOTION_GET_MASS: u32 = 0x00c8_6430;
const ENTITY_ACTIVATE: u32 = 0x00c9_c1d0;
/// `_CIsqrt` (`00ec6040`, the argument in `ST0`, a leading `f64`).
const CI_SQRT: u32 = 0x00ec_6040;
/// The singularity limit handed to `invert` (a `float` constant of the image) and the vector
/// of four sign bits the impulse is negated with.
const INVERT_EPSILON: u32 = 0x0106_c288;
const NEGATION_MASK: u32 = 0x010c_44d0;

// Translated from 00cbb1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMouseSpringAction::applyAction` (Xbox PDB), thiscall with one stack word, the step info
/// (`+8` the time step, `+0xc` the inverse step in this build; read as floats): pulls the point
/// of the body at `+0x20` (in body space) towards the mouse position at `+0x30`. The body is
/// the entity at `+0x18`. With `P` the transformed point, `D = P - mouse position`, `R = P -
/// centre of mass` (entity `+0x140`), `S` the float at `+0xcc` of the body motion (the motion
/// from `00ca86c0` or the entity own at `+0xe0`), the matrix `S * I - (R x)(inertia)(R x)` style
/// product is built with `setCrossSkewSymmetric`, the motion virtual function at `+0x28`
/// (inertia), `setMul_ov2` (twice) and `sub`, and inverted; when `invert` answers non-zero
/// nothing more happens. Otherwise both velocities of the entity are scaled by the object
/// damping (`+0x4c`), `00ca0c40` fills `R` from `P`, the impulse is
/// `inverse * (springDamping(+0x40) * R + (elasticity(+0x44) * step[0xc]) * D)` negated,
/// limited to a length of `step[8] * mass * maxRelativeForce(+0x48)` through `_CIsqrt`, applied
/// to the motion (virtual function `+0x50`, `(impulse, P)`) after `activate`, and every callback
/// of the array at `+0x54` (size at `+0x58`) is called as `(this, step info, impulse)`.
pub fn hkp_mouse_spring_action_apply_action(e: &mut Engine, this: Ptr, step: Ptr) {
    let (this, step) = (this.addr(), step.addr());
    // Locals of the exe frame, at their offsets from its base.
    // The exe aligns ESP, then `SUB ESP,0x174`: the frame base is 0x174 bytes under the 16-byte
    // boundary (00cbb1e3..00cbb1e6), its three pushes below it.
    with_aligned_frame(e, 0x174, |e, frame| {
        let at = |offset: u32| frame + offset;
        let (impulse, point, relative, matrix_inverse, direction) =
            (at(0x14), at(0x24), at(0x34), at(0x54), at(0x44));
        let (difference, skew, product_a, inertia, product_b) =
            (at(0xa4), at(0xb4), at(0xe4), at(0x114), at(0x144));
        let entity = e.mem.u32(this + 0x18);
        e.call(
            VECTOR_SET_TRANSFORMED_POS,
            &args![point, entity + 0xf0, this + 0x20],
        );
        let mouse = load_vec(e, this + 0x30);
        let transformed = load_vec(e, point);
        store_vec(e, difference, sub_ps(transformed, mouse));
        let mut motion = e.call(ENTITY_GET_MOTION_00CA86C0, &args![entity]).u32();
        if motion != 0 {
            motion = e.call(ENTITY_GET_MOTION_00CA86C0, &args![entity]).u32();
        } else {
            motion = entity + 0xe0;
        }
        let fourth_lane = e.mem.u32(motion + 0xc0 + 0xc);
        let centre = load_vec(e, entity + 0x140);
        store_vec(e, relative, sub_ps(load_vec(e, point), centre));
        e.call(MATRIX_SET_CROSS_SKEW_SYMMETRIC, &args![skew, relative]);
        e.vcall(motion, 0x28, &args![inertia]);
        // The diagonal matrix with the loaded float on its diagonal.
        let diagonal = x87_quiet_bits(fourth_lane);
        for (column, lanes) in [
            (0u32, [diagonal, 0, 0, 0]),
            (0x10, [0, diagonal, 0, 0]),
            (0x20, [0, 0, diagonal, 0]),
        ] {
            for (lane, bits) in lanes.into_iter().enumerate() {
                e.mem
                    .set_u32(matrix_inverse + column + 4 * lane as u32, bits);
            }
        }
        e.call(MATRIX_SET_MUL, &args![product_a, skew, inertia]);
        e.call(MATRIX_SET_MUL, &args![product_b, product_a, skew]);
        e.call(MATRIX_SUB, &args![matrix_inverse, product_b]);
        let epsilon = x87_quiet_bits(e.mem.u32(INVERT_EPSILON));
        let failed = e.call(MATRIX_INVERT, &args![matrix_inverse, epsilon]).u32() != 0;
        if failed {
            return;
        }
        let object_damping = e.mem.f32(this + 0x4c);
        let linear = mul_ps([object_damping; 4], load_vec(e, entity + 0x1b0));
        let linear_at = at(0x84);
        store_vec(e, linear_at, linear);
        e.call(RIGID_BODY_SET_LINEAR_VELOCITY, &args![entity, linear_at]);
        let object_damping = e.mem.f32(this + 0x4c);
        let angular = mul_ps([object_damping; 4], load_vec(e, entity + 0x1c0));
        let angular_at = at(0x94);
        store_vec(e, angular_at, angular);
        e.call(RIGID_BODY_SET_ANGULAR_VELOCITY, &args![entity, angular_at]);
        e.call(ENTITY_POINT_QUERY_00CA0C40, &args![entity, point, relative]);
        let elasticity_step = f32::from_bits(x87_store32(x87_mul(
            x87_load32(e.mem.u32(this + 0x44)),
            x87_m32(e.mem.u32(step + 0xc)),
        )));
        let scaled_difference = mul_ps([elasticity_step; 4], load_vec(e, difference));
        store_vec(e, direction, scaled_difference);
        let spring_damping = e.mem.f32(this + 0x40);
        let from_relative = mul_ps([spring_damping; 4], load_vec(e, relative));
        store_vec(e, direction, add_ps(from_relative, scaled_difference));
        e.call(VECTOR_SET_MUL3, &args![impulse, matrix_inverse, direction]);
        let mask = load_vec(e, NEGATION_MASK).map(|lane| lane.to_bits());
        let negated: Vec4 = {
            let lanes = load_vec(e, impulse);
            std::array::from_fn(|lane| f32::from_bits(lanes[lane].to_bits() ^ mask[lane]))
        };
        store_vec(e, impulse, negated);
        let squares = mul_ps(negated, negated);
        let length_squared = squares[2] + (squares[1] + squares[0]);
        let mass = x87_store32(e.call(MOTION_GET_MASS, &args![motion]).f64());
        let limit = x87_store32(x87_mul(
            x87_mul(x87_load32(e.mem.u32(step + 8)), x87_m32(mass)),
            x87_m32(e.mem.u32(this + 0x48)),
        ));
        let limit_loaded = x87_load32(limit);
        let limit_squared = x87_mul(limit_loaded, limit_loaded);
        // `FCOM`: only an ordered "longer than the limit" scales the impulse.
        if x87_load32(length_squared.to_bits()) > limit_squared {
            let root = x87_store32(
                e.call(CI_SQRT, &args![x87_load32(length_squared.to_bits())])
                    .f64(),
            );
            let inverse = x87_store32(x87_div(1.0, x87_load32(root)));
            let factor = f32::from_bits(x87_store32(x87_mul(x87_load32(inverse), x87_m32(limit))));
            let lanes = load_vec(e, impulse);
            store_vec(e, impulse, mul_ps([factor; 4], lanes));
        }
        e.call(ENTITY_ACTIVATE, &args![entity]);
        e.vcall(entity + 0xe0, 0x50, &args![impulse, point]);
        let mut index = 0u32;
        while (index as i32) < (e.mem.u32(this + 0x58) as i32) {
            let callbacks = e.mem.u32(this + 0x54);
            let callback = e.mem.u32(callbacks.wrapping_add(index * 4));
            e.call(callback, &args![this, step, impulse]);
            index += 1;
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cb_b1e0, hkp_mouse_spring_action_apply_action(Ptr, Ptr)),
        entry!(0x00cbb8a0, hkp_unary_action_get_class_type() -> u32),
        entry!(
            0x00cb_b1b0,
            hkp_mouse_spring_action_set_max_relative_force(Ptr, u32)
        ),
        entry!(
            0x00cb_b1c0,
            hkp_mouse_spring_action_entity_removed_callback(Ptr, u32)
        ),
        entry!(
            0x00cb_b4e0,
            hkp_mouse_spring_action_hkp_mouse_spring_action(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00cb_b560,
            hkp_mouse_spring_action_hkp_mouse_spring_action_ov2(
                Ptr, Ptr, Ptr, u32, u32, u32, u32, u32
            ) -> Ptr
        ),
        entry!(
            0x00cb_b790,
            hkp_mouse_spring_action_clone(Ptr, Ptr, Ptr) -> u32
        ),
        entry!(
            0x00cb_b8b0,
            hkp_mouse_spring_action_scalar_deleting_destructor(Ptr, u8) -> Ptr
        ),
        entry!(0x00cd_2d80, hkp_collidable_hkp_collidable(Ptr, u32, u32, u32) -> Ptr),
        entry!(
            0x00d6_6bf0,
            hkp_mouse_spring_action_set_mouse_position(Ptr, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hkp_unary_action_get_class_type_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cb_b8a0, &args![]).u32(), 0x0126_8a14);
    }

    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const BLOCK_ALLOC: u32 = 0x0900_0031;
    const BLOCK_FREE: u32 = 0x0900_0032;
    const ARRAY_ALLOC: u32 = 0x0900_0033;
    const ARRAY_FREE: u32 = 0x0900_0034;
    const COLLISION_FUNCTION: u32 = 0x0900_0035;

    /// An engine with the exe data range mapped, the constants the constructors read and an
    /// allocator whose slots 4, 8, 0xC and 0x10 are doubles that log `(address, arguments)`
    /// (without the allocator itself).
    fn setup() -> (Engine, Log) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(DAMPING_DEFAULT, 0.5f32);
        e.set_global(ELASTICITY_DEFAULT, 0.3f32);
        e.set_global(MAX_FORCE_DEFAULT, 250.0f32);
        e.set_global(LIMIT_DEFAULT, 0.95f32);
        e.set_global(COLLECTOR_CONSTANT, 3.5f32);
        e.set_global(COLLIDABLE_CONSTANT, -1.0f32);
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, BLOCK_ALLOC);
        e.mem.set_u32(vtable + 8, BLOCK_FREE);
        e.mem.set_u32(vtable + 0xc, ARRAY_ALLOC);
        e.mem.set_u32(vtable + 0x10, ARRAY_FREE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(BLOCK_ALLOC, move |e, a| {
            sink.borrow_mut().push((BLOCK_ALLOC, a[1..].to_vec()));
            Ret {
                eax: e.mem.alloc(a[1]),
                ..Ret::default()
            }
        });
        let sink = Rc::clone(&log);
        e.register_double(ARRAY_ALLOC, move |e, a| {
            let requested = e.mem.u32(a[1]);
            sink.borrow_mut().push((ARRAY_ALLOC, vec![requested]));
            // The allocator grants a little more than asked.
            e.mem.set_u32(a[1], requested + 6);
            Ret {
                eax: e.mem.alloc(requested + 8),
                ..Ret::default()
            }
        });
        for address in [BLOCK_FREE, ARRAY_FREE] {
            let sink = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a[1..].to_vec()));
                Ret::default()
            });
        }
        (e, log)
    }

    /// Doubles over callees that log their arguments and return 0.
    fn log_callees(e: &mut Engine, log: &Log, addresses: &[u32]) {
        for &address in addresses {
            let sink = Rc::clone(log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
    }

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    fn set_words(e: &mut Engine, address: u32, words: &[u32]) {
        for (index, word) in words.iter().enumerate() {
            e.mem.set_u32(address + 4 * index as u32, *word);
        }
    }

    fn words(e: &Engine, address: u32, count: u32) -> Vec<u32> {
        (0..count)
            .map(|index| e.mem.u32(address + 4 * index))
            .collect()
    }

    #[test]
    fn set_max_relative_force_stores_the_quieted_float() {
        let mut e = Engine::new();
        let action = e.mem.alloc(0x60);
        e.call(0x00cb_b1b0, &args![action, 12.5f32.to_bits()]);
        assert_eq!(e.mem.f32(action + 0x48), 12.5);
        e.call(0x00cb_b1b0, &args![action, 0x7f80_0001u32]);
        assert_eq!(e.mem.u32(action + 0x48), 0x7fc0_0001);
    }

    #[test]
    fn entity_removed_callback_forwards_and_clears_the_word() {
        let (mut e, log) = setup();
        log_callees(&mut e, &log, &[UNARY_ACTION_ENTITY_REMOVED]);
        let action = e.mem.alloc(0x60);
        e.mem.set_u32(action + 0x18, 0x1234);
        e.call(0x00cb_b1c0, &args![action, 0x55u32]);
        assert_eq!(
            calls_to(&log, UNARY_ACTION_ENTITY_REMOVED),
            vec![vec![action, 0x55]]
        );
        assert_eq!(e.mem.u32(action + 0x18), 0);
    }

    #[test]
    fn the_constructor_sets_the_defaults() {
        let (mut e, log) = setup();
        log_callees(&mut e, &log, &[UNARY_ACTION_CONSTRUCT, STRING_ASSIGN]);
        let action = e.mem.alloc(0x60);
        for offset in (0..0x60).step_by(4) {
            e.mem.set_u32(action + offset, 0xdead_beef);
        }
        assert_eq!(e.call(0x00cb_b4e0, &args![action, 0x77u32]).u32(), action);
        assert_eq!(
            calls_to(&log, UNARY_ACTION_CONSTRUCT),
            vec![vec![action, 0x77, 0]]
        );
        assert_eq!(
            calls_to(&log, STRING_ASSIGN),
            vec![vec![action + 0x14, ACTION_NAME]]
        );
        assert_eq!(e.mem.u32(action), MOUSE_SPRING_VTABLE);
        assert_eq!(e.mem.f32(action + 0x40), 0.5);
        assert_eq!(e.mem.f32(action + 0x44), 0.3);
        assert_eq!(e.mem.f32(action + 0x48), 250.0);
        assert_eq!(e.mem.f32(action + 0x4c), 0.95);
        assert_eq!(words(&e, action + 0x20, 8), [0; 8]);
        assert_eq!(e.mem.u32(action + 0x50), 0xffff_ffff);
        assert_eq!(words(&e, action + 0x54, 3), [0, 0, 0x8000_0000]);
        assert_eq!(e.mem.u32(action + 0x10), 0xdead_beef);
        assert_eq!(e.mem.u32(action + 0x18), 0xdead_beef);
    }

    /// Everything the second constructor calls, with the unary constructor, the string
    /// assignment and the sphere constructor logged.
    fn ov2_setup() -> (Engine, Log, u32) {
        let (mut e, log) = setup();
        log_callees(
            &mut e,
            &log,
            &[UNARY_ACTION_CONSTRUCT, STRING_ASSIGN, SPHERE_CONSTRUCT],
        );
        let action = e.mem.alloc(0x60);
        (e, log, action)
    }

    fn vectors(e: &mut Engine) -> (u32, u32) {
        let first = e.mem.alloc(0x10);
        let second = e.mem.alloc(0x10);
        set_words(
            e,
            first,
            &[1.0f32.to_bits(), 2.0f32.to_bits(), 3.0f32.to_bits(), 4],
        );
        set_words(
            e,
            second,
            &[5.0f32.to_bits(), 6.0f32.to_bits(), 7.0f32.to_bits(), 8],
        );
        (first, second)
    }

    #[test]
    fn the_second_constructor_copies_its_arguments_without_a_world() {
        let (mut e, log, action) = ov2_setup();
        let (first, second) = vectors(&mut e);
        let entity = e.mem.alloc(0x20);
        let result = e.call(
            0x00cb_b560,
            &args![
                action,
                first,
                second,
                0.25f32.to_bits(),
                0x7f80_0001u32,
                0.75f32.to_bits(),
                entity,
                0u32
            ],
        );
        assert_eq!(result.u32(), action);
        assert_eq!(
            calls_to(&log, UNARY_ACTION_CONSTRUCT),
            vec![vec![action, entity, 0]]
        );
        assert_eq!(e.mem.u32(action), MOUSE_SPRING_VTABLE);
        assert_eq!(
            words(&e, action + 0x20, 8),
            [
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                3.0f32.to_bits(),
                4,
                5.0f32.to_bits(),
                6.0f32.to_bits(),
                7.0f32.to_bits(),
                8
            ]
        );
        assert_eq!(e.mem.f32(action + 0x40), 0.25);
        assert_eq!(e.mem.u32(action + 0x44), 0x7fc0_0001);
        assert_eq!(e.mem.f32(action + 0x48), 250.0);
        assert_eq!(e.mem.f32(action + 0x4c), 0.75);
        assert_eq!(e.mem.u32(action + 0x50), 0xffff_ffff);
        assert_eq!(words(&e, action + 0x54, 3), [0, 0, 0x8000_0000]);
        assert!(calls_to(&log, SPHERE_CONSTRUCT).is_empty());
        assert_eq!(
            calls_to(&log, STRING_ASSIGN),
            vec![vec![action + 0x14, ACTION_NAME]]
        );
    }

    /// An array object of `values` words.
    fn word_array(e: &mut Engine, values: &[u32]) -> u32 {
        let array = e.mem.alloc(8);
        let data = e.mem.alloc(4 * values.len() as u32 + 4);
        set_words(e, data, values);
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, values.len() as u32);
        array
    }

    #[test]
    fn the_second_constructor_copies_the_word_array() {
        let (mut e, log, action) = ov2_setup();
        let (first, second) = vectors(&mut e);
        let entity = e.mem.alloc(0x20);
        let array = word_array(&mut e, &[10, 20, 30]);
        e.call(
            0x00cb_b560,
            &args![action, first, second, 0u32, 0u32, 0u32, entity, array],
        );
        // The empty array has bit 31 set: nothing is freed; 12 bytes are requested and 18
        // granted: the capacity word is 18 / 4 = 4.
        assert!(calls_to(&log, ARRAY_FREE).is_empty());
        assert_eq!(calls_to(&log, ARRAY_ALLOC), vec![vec![12]]);
        assert_eq!(e.mem.u32(action + 0x58), 3);
        assert_eq!(e.mem.u32(action + 0x5c), 4);
        assert_eq!(words(&e, e.mem.u32(action + 0x54), 3), [10, 20, 30]);
    }

    #[test]
    fn the_second_constructor_leaves_big_enough_storage_alone() {
        let (mut e, log, action) = ov2_setup();
        let (first, second) = vectors(&mut e);
        let entity = e.mem.alloc(0x20);
        let array = word_array(&mut e, &[]);
        e.call(
            0x00cb_b560,
            &args![action, first, second, 0u32, 0u32, 0u32, entity, array],
        );
        assert!(calls_to(&log, ARRAY_ALLOC).is_empty());
        assert_eq!(e.mem.u32(action + 0x58), 0);
        assert_eq!(e.mem.u32(action + 0x5c), 0x8000_0000);
    }

    /// A world with a collision function table: shape type 3 and `owner[0xC] = 5` select the
    /// byte at table `+ 0x205` (here 2), whose function is the word at table `+ 0x9d0`.
    struct WorldSetup {
        action: u32,
        entity: u32,
        first: u32,
        second: u32,
        seen: Rc<RefCell<Vec<Vec<u32>>>>,
    }

    fn world_setup(e: &mut Engine, log: &Log, action: u32, leave_count: bool) -> WorldSetup {
        let (first, second) = vectors(e);
        let world = e.mem.alloc(0x100);
        let table = e.mem.alloc(0x1000);
        e.mem.set_u32(world + 0x80, table);
        e.mem.set_u8(table + 0x205, 2);
        e.mem.set_u32(table + 0x9a8 + 2 * 0x14, COLLISION_FUNCTION);
        let source = e.mem.alloc(0x50);
        for offset in (0..0x50).step_by(4) {
            e.mem.set_u32(source + offset, 0x100 + offset);
        }
        e.mem.set_u32(world + 0x78, source);
        let owner = e.mem.alloc(0x20);
        e.mem.set_u32(owner + 0xc, 5);
        let entity = e.mem.alloc(0x20);
        e.mem.set_u32(entity + 8, world);
        e.mem.set_u32(entity + 0x10, owner);
        let identity = e.mem.alloc(0x30);
        for offset in (0..0x30).step_by(4) {
            e.mem.set_u32(identity + offset, 0x500 + offset);
        }
        e.register_double(GET_IDENTITY, move |_, _| Ret {
            eax: identity,
            ..Ret::default()
        });
        let sink = Rc::clone(log);
        e.register_double(SPHERE_CONSTRUCT, move |e, a| {
            sink.borrow_mut().push((SPHERE_CONSTRUCT, a.to_vec()));
            e.mem.set_u32(a[0] + 0xc, 3);
            Ret::default()
        });
        log_callees(e, log, &[COLLIDABLE_CONSTRUCT]);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(COLLISION_FUNCTION, move |e, a| {
            let mut snapshot = a.to_vec();
            snapshot.extend(words(e, a[2], 0x14));
            snapshot.extend(words(e, a[3], 0x14));
            sink.borrow_mut().push(snapshot);
            if !leave_count {
                e.mem.set_u32(a[3] + 0x30, 1);
                e.mem.set_u32(a[3] + 0x5c, 0x77);
            }
            Ret::default()
        });
        WorldSetup {
            action,
            entity,
            first,
            second,
            seen,
        }
    }

    fn run_world_case(leave_count: bool) -> (Engine, Log, WorldSetup) {
        let (mut e, log, action) = ov2_setup();
        let w = world_setup(&mut e, &log, action, leave_count);
        e.call(
            0x00cb_b560,
            &args![w.action, w.first, w.second, 0u32, 0u32, 0u32, w.entity, 0u32],
        );
        (e, log, w)
    }

    #[test]
    fn the_second_constructor_calls_the_collision_function_of_the_world() {
        let (e, log, w) = run_world_case(false);
        assert_eq!(e.mem.u32(w.action + 0x50), 0x77);
        let sphere = calls_to(&log, SPHERE_CONSTRUCT);
        assert_eq!(sphere.len(), 1);
        assert_eq!(sphere[0][1], 1.0f32.to_bits());
        let collidable = calls_to(&log, COLLIDABLE_CONSTRUCT);
        assert_eq!(collidable.len(), 1);
        assert_eq!(collidable[0][1], sphere[0][0]);
        assert_eq!(collidable[0][2], sphere[0][0] + 0x20);
        assert_eq!(collidable[0][3], 0);
        let seen = w.seen.borrow();
        assert_eq!(seen.len(), 1);
        let call = &seen[0];
        assert_eq!(call[0], collidable[0][0]);
        assert_eq!(call[1], w.entity + 0x10);
        // The copy: six words, a gap, then three vectors.
        let copy = &call[4..24];
        assert_eq!(copy[..6], [0x100, 0x104, 0x108, 0x10c, 0x110, 0x114]);
        assert_eq!(copy[8..12], [0x120, 0x124, 0x128, 0x12c]);
        assert_eq!(copy[12..16], [0x130, 0x134, 0x138, 0x13c]);
        assert_eq!(copy[16..20], [0x140, 0x144, 0x148, 0x14c]);
        // The collector: vtable, the constant at +4 and +0x2c, a zero count.
        let collector = &call[24..44];
        assert_eq!(collector[0], COLLECTOR_VTABLE);
        assert_eq!(collector[1], 3.5f32.to_bits());
        assert_eq!(collector[0x2c / 4], 3.5f32.to_bits());
        assert_eq!(collector[0x30 / 4], 0);
    }

    #[test]
    fn the_second_constructor_builds_the_transform_of_the_collidable() {
        let (e, log, _) = run_world_case(false);
        let transform = calls_to(&log, COLLIDABLE_CONSTRUCT)[0][2];
        let expected: Vec<u32> = (0..12u32)
            .map(|index| 0x500 + 4 * index)
            .chain([5.0f32.to_bits(), 6.0f32.to_bits(), 7.0f32.to_bits(), 8])
            .collect();
        assert_eq!(words(&e, transform, 16), expected);
    }

    #[test]
    fn the_second_constructor_keeps_the_default_when_the_collector_is_empty() {
        let (e, _, w) = run_world_case(true);
        assert_eq!(e.mem.u32(w.action + 0x50), 0xffff_ffff);
    }

    #[test]
    fn clone_builds_a_copy_of_the_action() {
        let (mut e, log) = setup();
        log_callees(&mut e, &log, &[UNARY_ACTION_CONSTRUCT, STRING_ASSIGN]);
        let action = e.mem.alloc(0x60);
        set_words(&mut e, action + 0x20, &[1, 2, 3, 4, 5, 6, 7, 8]);
        e.mem.set_f32(action + 0x40, 0.25);
        e.mem.set_u32(action + 0x44, 0x7f80_0002);
        e.mem.set_f32(action + 0x48, 8.0);
        e.mem.set_f32(action + 0x4c, 9.0);
        e.mem.set_u32(action + 0x10, 0xabcd);
        e.mem.set_u32(action + 0x50, 0x42);
        let data = e.mem.alloc(16);
        set_words(&mut e, data, &[11, 22]);
        e.mem.set_u32(action + 0x54, data);
        e.mem.set_u32(action + 0x58, 2);
        e.mem.set_u32(action + 0x5c, 2);
        let first = word_array(&mut e, &[0x1234]);
        let second = word_array(&mut e, &[]);
        let first_data = e.mem.u32(first);
        e.mem.set_u32(first_data, 0x1234);
        let copy = e.call(0x00cb_b790, &args![action, first, second]).u32();
        assert_ne!(copy, 0);
        assert_eq!(calls_to(&log, BLOCK_ALLOC), vec![vec![0x60]]);
        assert_eq!(e.mem.u16(copy + 4), 0x60);
        assert_eq!(
            calls_to(&log, UNARY_ACTION_CONSTRUCT),
            vec![vec![copy, 0x1234, 0]]
        );
        assert_eq!(words(&e, copy + 0x20, 8), [1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(e.mem.f32(copy + 0x40), 0.25);
        assert_eq!(e.mem.u32(copy + 0x44), 0x7fc0_0002);
        assert_eq!(e.mem.f32(copy + 0x48), 8.0);
        assert_eq!(e.mem.f32(copy + 0x4c), 9.0);
        assert_eq!(e.mem.u32(copy + 0x10), 0xabcd);
        assert_eq!(e.mem.u32(copy + 0x50), 0x42);
        // Capacity 0 < 2 words: 8 bytes requested, 14 granted, capacity 3.
        assert_eq!(calls_to(&log, ARRAY_ALLOC), vec![vec![8]]);
        assert_eq!(e.mem.u32(copy + 0x58), 2);
        assert_eq!(e.mem.u32(copy + 0x5c), 3);
        assert_eq!(words(&e, e.mem.u32(copy + 0x54), 2), [11, 22]);
    }

    #[test]
    fn clone_returns_zero_unless_the_arrays_hold_one_and_none() {
        let (mut e, _) = setup();
        let action = e.mem.alloc(0x60);
        let none = word_array(&mut e, &[]);
        let one = word_array(&mut e, &[1]);
        let two = word_array(&mut e, &[1, 2]);
        assert_eq!(e.call(0x00cb_b790, &args![action, none, none]).u32(), 0);
        assert_eq!(e.call(0x00cb_b790, &args![action, two, none]).u32(), 0);
        assert_eq!(e.call(0x00cb_b790, &args![action, one, one]).u32(), 0);
    }

    #[test]
    fn the_deleting_destructor_frees_owned_storage_and_the_block() {
        let (mut e, log) = setup();
        log_callees(&mut e, &log, &[UNARY_ACTION_DESTRUCT]);
        let action = e.mem.alloc(0x60);
        e.mem.set_u16(action + 4, 0x60);
        e.mem.set_u32(action + 0x54, 0x7000);
        e.mem.set_u32(action + 0x58, 5);
        e.mem.set_u32(action + 0x5c, 0x4000_0006);
        let result = e.call(0x00cb_b8b0, &args![action, 1u8]).u32();
        assert_eq!(result, action);
        assert_eq!(calls_to(&log, ARRAY_FREE), vec![vec![0x7000, 24]]);
        assert_eq!(calls_to(&log, UNARY_ACTION_DESTRUCT), vec![vec![action]]);
        assert_eq!(calls_to(&log, BLOCK_FREE), vec![vec![action, 0x60]]);
        assert_eq!(words(&e, action + 0x54, 3), [0, 0, 0x8000_0000]);
    }

    #[test]
    fn the_destructor_without_the_flag_keeps_the_block_and_borrowed_storage() {
        let (mut e, log) = setup();
        log_callees(&mut e, &log, &[UNARY_ACTION_DESTRUCT]);
        let action = e.mem.alloc(0x60);
        e.mem.set_u32(action + 0x54, 0x7000);
        e.mem.set_u32(action + 0x58, 5);
        e.mem.set_u32(action + 0x5c, 0x8000_0003);
        e.call(0x00cb_b8b0, &args![action, 0xfeu8]);
        assert!(calls_to(&log, ARRAY_FREE).is_empty());
        assert!(calls_to(&log, BLOCK_FREE).is_empty());
        assert_eq!(calls_to(&log, UNARY_ACTION_DESTRUCT), vec![vec![action]]);
        assert_eq!(words(&e, action + 0x54, 3), [0, 0, 0x8000_0000]);
    }

    #[test]
    fn the_collidable_constructor_sets_its_fields() {
        let (mut e, log) = setup();
        let sink = Rc::clone(&log);
        e.register_double(BOUNDING_VOLUME_CONSTRUCT, move |e, a| {
            // The byte at +0x19 is still 0x7f while the bounding volume is built.
            sink.borrow_mut().push((
                BOUNDING_VOLUME_CONSTRUCT,
                vec![a[0], u32::from(e.mem.u8(a[0] - 0x20 + 0x19))],
            ));
            Ret::default()
        });
        let collidable = e.mem.alloc(0x60);
        for offset in (0..0x60).step_by(4) {
            e.mem.set_u32(collidable + offset, 0xdead_beef);
        }
        let result = e.call(
            0x00cd_2d80,
            &args![collidable, 0x11u32, 0x22u32, 0x1234_5601u32],
        );
        assert_eq!(result.u32(), collidable);
        assert_eq!(
            calls_to(&log, BOUNDING_VOLUME_CONSTRUCT),
            vec![vec![collidable + 0x20, 0x7f]]
        );
        assert_eq!(e.mem.u32(collidable), 0x11);
        assert_eq!(e.mem.u32(collidable + 4), 0xffff_ffff);
        assert_eq!(e.mem.u32(collidable + 8), 0x22);
        assert_eq!(e.mem.u32(collidable + 0xc), 0);
        assert_eq!(e.mem.u8(collidable + 0x10), 0);
        assert_eq!(e.mem.u8(collidable + 0x11), 0);
        assert_eq!(e.mem.u16(collidable + 0x12), 0);
        assert_eq!(e.mem.u32(collidable + 0x14), 0);
        assert_eq!(e.mem.u8(collidable + 0x18), 1);
        assert_eq!(e.mem.u8(collidable + 0x19), 0xec);
        assert_eq!(e.mem.u8(collidable + 0x1a), 0xff);
        assert_eq!(e.mem.u32(collidable + 0x1c), 0);
        assert_eq!(e.mem.f32(collidable + 0x4c), -1.0);
        assert_eq!(e.mem.u32(collidable + 0x50), 0xdead_beef);
    }

    #[test]
    fn set_mouse_position_forwards_to_a_live_action() {
        let (mut e, log) = setup();
        log_callees(&mut e, &log, &[SPRING_SET_MOUSE_POSITION]);
        let viewer = e.mem.alloc(0x40);
        e.call(0x00d6_6bf0, &args![viewer, 0x55u32]);
        let action = e.mem.alloc(0x20);
        e.mem.set_u32(viewer + 0x2c, action);
        e.call(0x00d6_6bf0, &args![viewer, 0x56u32]);
        e.mem.set_u32(action + 8, 1);
        e.call(0x00d6_6bf0, &args![viewer, 0x57u32]);
        assert_eq!(
            calls_to(&log, SPRING_SET_MOUSE_POSITION),
            vec![vec![action, 0x57]]
        );
    }
}

#[cfg(test)]
mod apply_action_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Seen = Rc<RefCell<Vec<(u32, Vec<u32>, Vec<f32>)>>>;

    const MOTION_VTABLE: u32 = 0x0c00_0000;
    const MOTION_INERTIA: u32 = 0x0c00_0100;
    const MOTION_IMPULSE: u32 = 0x0c00_0110;
    const CALLBACK: u32 = 0x0c00_0200;

    /// Records a call's words and the four floats at the address in its word `at` (when given).
    fn watch(e: &mut Engine, seen: &Seen, address: u32, at: Option<usize>, answer: Ret) {
        let seen = Rc::clone(seen);
        e.register_double(address, move |e, a| {
            let floats = at
                .map(|i| (0..4).map(|lane| e.mem.f32(a[i] + 4 * lane)).collect())
                .unwrap_or_default();
            seen.borrow_mut().push((address, a.to_vec(), floats));
            answer
        });
    }

    fn found(seen: &Seen, address: u32) -> Vec<(Vec<u32>, Vec<f32>)> {
        seen.borrow()
            .iter()
            .filter(|(a, _, _)| *a == address)
            .map(|(_, words, floats)| (words.clone(), floats.clone()))
            .collect()
    }

    struct Scene {
        e: Engine,
        seen: Seen,
        action: u32,
        entity: u32,
        step: u32,
    }

    fn scene(invert_answer: u32) -> Scene {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        let seen: Seen = Rc::new(RefCell::new(Vec::new()));
        let entity = e.mem.alloc(0x300);
        let action = e.mem.alloc(0x80);
        let step = e.mem.alloc(0x20);
        e.mem.set_u32(action + 0x18, entity);
        e.mem.set_f32(action + 0x40, 2.0);
        e.mem.set_f32(action + 0x44, 0.5);
        e.mem.set_f32(action + 0x48, 1.0);
        e.mem.set_f32(action + 0x4c, 0.5);
        for (lane, value) in [0.5f32, 0.5, 0.5, 0.0].into_iter().enumerate() {
            e.mem.set_f32(action + 0x30 + 4 * lane as u32, value);
        }
        e.mem.set_f32(step + 8, 0.5);
        e.mem.set_f32(step + 0xc, 2.0);
        for (lane, value) in [1.0f32, 2.0, 3.0, 0.0].into_iter().enumerate() {
            e.mem.set_f32(entity + 0x1b0 + 4 * lane as u32, value);
        }
        e.mem.set_f32(entity + 0xe0 + 0xcc, 0.5);
        e.mem.set_u32(entity + 0xe0, MOTION_VTABLE);
        e.put_vtable(MOTION_VTABLE, &[0; 0x15]);
        e.mem.set_u32(MOTION_VTABLE + 0x28, MOTION_INERTIA);
        e.mem.set_u32(MOTION_VTABLE + 0x50, MOTION_IMPULSE);
        e.mem.set_u32(INVERT_EPSILON, 0x33d6_bf95);
        for lane in 0..4 {
            e.mem.set_u32(NEGATION_MASK + 4 * lane, 0x8000_0000);
        }
        watch(&mut e, &seen, MOTION_INERTIA, None, Ret::default());
        watch(&mut e, &seen, MOTION_IMPULSE, Some(1), Ret::default());
        watch(
            &mut e,
            &seen,
            ENTITY_GET_MOTION_00CA86C0,
            None,
            Ret::default(),
        );
        watch(
            &mut e,
            &seen,
            MATRIX_SET_CROSS_SKEW_SYMMETRIC,
            None,
            Ret::default(),
        );
        watch(&mut e, &seen, MATRIX_SET_MUL, None, Ret::default());
        watch(&mut e, &seen, MATRIX_SUB, Some(0), Ret::default());
        watch(
            &mut e,
            &seen,
            MATRIX_INVERT,
            None,
            Ret {
                eax: invert_answer,
                ..Ret::default()
            },
        );
        watch(
            &mut e,
            &seen,
            RIGID_BODY_SET_LINEAR_VELOCITY,
            Some(1),
            Ret::default(),
        );
        watch(
            &mut e,
            &seen,
            RIGID_BODY_SET_ANGULAR_VELOCITY,
            Some(1),
            Ret::default(),
        );
        watch(&mut e, &seen, ENTITY_ACTIVATE, None, Ret::default());
        watch(
            &mut e,
            &seen,
            MOTION_GET_MASS,
            None,
            Ret {
                st0: 2.0,
                ..Ret::default()
            },
        );
        let sqrt_seen = Rc::clone(&seen);
        e.register_double(CI_SQRT, move |_, a| {
            sqrt_seen
                .borrow_mut()
                .push((CI_SQRT, a.to_vec(), Vec::new()));
            Ret {
                st0: f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32).sqrt(),
                ..Ret::default()
            }
        });
        e.register(VECTOR_SET_TRANSFORMED_POS, |e, a| {
            for (lane, value) in [1.0f32, 2.0, 3.0, 0.0].into_iter().enumerate() {
                e.mem.set_f32(a[0] + 4 * lane as u32, value);
            }
            Ret::default()
        });
        e.register(ENTITY_POINT_QUERY_00CA0C40, |e, a| {
            for lane in 0..3 {
                e.mem.set_f32(a[2] + 4 * lane, 9.0);
            }
            e.mem.set_f32(a[2] + 12, 0.0);
            Ret::default()
        });
        let direction_seen = Rc::clone(&seen);
        e.register_double(VECTOR_SET_MUL3, move |e, a| {
            let floats = (0..4).map(|lane| e.mem.f32(a[2] + 4 * lane)).collect();
            direction_seen
                .borrow_mut()
                .push((VECTOR_SET_MUL3, a.to_vec(), floats));
            // The impulse the inverse matrix gives: (3, 4, 0, 0).
            for (lane, value) in [3.0f32, 4.0, 0.0, 0.0].into_iter().enumerate() {
                e.mem.set_f32(a[0] + 4 * lane as u32, value);
            }
            Ret::default()
        });
        Scene {
            e,
            seen,
            action,
            entity,
            step,
        }
    }

    #[test]
    fn applying_the_spring_limits_and_applies_the_impulse() {
        let mut s = scene(0);
        let callbacks = s.e.mem.alloc(8);
        s.e.mem.set_u32(callbacks, CALLBACK);
        s.e.mem.set_u32(s.action + 0x54, callbacks);
        s.e.mem.set_u32(s.action + 0x58, 1);
        let callback_seen = Rc::clone(&s.seen);
        s.e.register_double(CALLBACK, move |e, a| {
            let floats = (0..4).map(|lane| e.mem.f32(a[2] + 4 * lane)).collect();
            callback_seen
                .borrow_mut()
                .push((CALLBACK, a.to_vec(), floats));
            Ret::default()
        });
        s.e.call(0x00cb_b1e0, &args![s.action, s.step]);
        let (entity, seen) = (s.entity, &s.seen);
        // The body has no motion of its own from `00ca86c0` (answer 0): its embedded one is used.
        assert_eq!(found(seen, ENTITY_GET_MOTION_00CA86C0).len(), 1);
        assert_eq!(found(seen, MOTION_INERTIA).len(), 1);
        // Both velocities are scaled by the object damping 0.5.
        assert_eq!(found(seen, RIGID_BODY_SET_LINEAR_VELOCITY)[0].0[0], entity);
        assert_eq!(
            found(seen, RIGID_BODY_SET_LINEAR_VELOCITY)[0].1,
            vec![0.5, 1.0, 1.5, 0.0]
        );
        // direction = 2 * R(9) + (0.5 * 2.0) * (P - mouse) = 18 + 0.5, 18 + 1.5, 18 + 2.5.
        assert_eq!(
            found(seen, VECTOR_SET_MUL3)[0].1,
            vec![18.5, 19.5, 20.5, 0.0]
        );
        // The diagonal of the matrix is the float at +0xcc of the motion (0.5) when `sub` runs.
        assert_eq!(found(seen, MATRIX_SUB)[0].0.len(), 2);
        assert_eq!(found(seen, MATRIX_SUB)[0].1, vec![0.5, 0.0, 0.0, 0.0]);
        // The impulse (3, 4) is negated and cut to the length limit 0.5 * 2 * 1 = 1: (-0.6, -0.8).
        let impulse = &found(seen, MOTION_IMPULSE)[0];
        assert_eq!(impulse.0[0], entity + 0xe0);
        let expected = [-0.6f32, -0.8, 0.0, 0.0];
        for lane in 0..4 {
            assert!(
                (impulse.1[lane] - expected[lane]).abs() < 1e-6,
                "{:?}",
                impulse.1
            );
        }
        // The callbacks get the action, the step info and the impulse.
        let called = found(seen, CALLBACK);
        assert_eq!(called.len(), 1);
        assert_eq!(called[0].0[..2], [s.action, s.step]);
        assert_eq!(called[0].1, impulse.1);
        // `activate` ran before the impulse; the epsilon was handed to `invert`.
        let order: Vec<u32> = seen
            .borrow()
            .iter()
            .map(|(a, _, _)| *a)
            .filter(|a| [ENTITY_ACTIVATE, MOTION_IMPULSE, MATRIX_INVERT].contains(a))
            .collect();
        assert_eq!(order, vec![MATRIX_INVERT, ENTITY_ACTIVATE, MOTION_IMPULSE]);
        assert_eq!(found(seen, MATRIX_INVERT)[0].0[1], 0x33d6_bf95);
        // Squared length 25 against the squared limit 1: the root is taken once.
        assert_eq!(found(seen, CI_SQRT).len(), 1);
    }

    #[test]
    fn the_spring_locals_sit_in_the_aligned_exe_frame() {
        let mut s = scene(0);
        let callbacks = s.e.mem.alloc(8);
        s.e.mem.set_u32(callbacks, CALLBACK);
        s.e.mem.set_u32(s.action + 0x54, callbacks);
        s.e.mem.set_u32(s.action + 0x58, 1);
        let impulses = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&impulses);
        s.e.register_double(CALLBACK, move |_, a| {
            sink.borrow_mut().push(a[2]);
            Ret::default()
        });
        let raw = s.e.next_stack_local(0x184);
        s.e.call(0x00cb_b1e0, &args![s.action, s.step]);
        // The impulse is at +0x14 of the base 0x174 bytes under the boundary.
        assert_eq!(
            *impulses.borrow(),
            vec![aligned_frame_at(raw, 0x174) + 0x14]
        );
    }

    #[test]
    fn a_short_impulse_is_not_scaled() {
        let mut s = scene(0);
        // Limit 0.5 * 2 * 10 = 10 against a length of 5.
        s.e.mem.set_f32(s.action + 0x48, 10.0);
        s.e.call(0x00cb_b1e0, &args![s.action, s.step]);
        let impulse = &found(&s.seen, MOTION_IMPULSE)[0];
        assert_eq!(impulse.1, vec![-3.0, -4.0, 0.0, 0.0]);
        assert!(found(&s.seen, CI_SQRT).is_empty());
    }

    #[test]
    fn a_singular_matrix_stops_before_any_velocity_changes() {
        let mut s = scene(1);
        s.e.call(0x00cb_b1e0, &args![s.action, s.step]);
        assert_eq!(found(&s.seen, MATRIX_INVERT).len(), 1);
        assert!(found(&s.seen, RIGID_BODY_SET_LINEAR_VELOCITY).is_empty());
        assert!(found(&s.seen, ENTITY_ACTIVATE).is_empty());
        assert!(found(&s.seen, MOTION_IMPULSE).is_empty());
    }
}
