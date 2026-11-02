//! `BSHavok/bhkcollisionobject.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//! `bhkCollisionObject` (0x14 bytes, Xbox PDB) holds the scene-graph node it moves (`+8`) and
//! the Havok body (`spWorldObject`, `+0x10`); the Havok body's motion type is the byte at
//! `+0xe8` of its Havok object (`+8`), 5 meaning "no Havok object".
//!
//! The exception-unwinding frames (`FS:[0]`) and the `/GS` cookie checks (`00ec408c`) of the
//! functions marked `Translation incomplete` are not modeled.

#[allow(unused_imports)]
use crate::prelude::*;

/// `bhkCollisionObject` base constructor (`00c6b910`, thiscall, no stack arguments) and the
/// members copy `CopyMembers` (`00c6ba30`, thiscall: the clone and the process).
const BASE_CONSTRUCT: u32 = 0x00c6_b910;
const COPY_MEMBERS: u32 = 0x00c6_ba30;
/// The base `UpdateWorldData` (engine map name `bhkNiCollisionObject::UpdateWorldData`,
/// `00c6b670`, thiscall: one stack word) and the base `AddViewerStrings` (`00c6c430`).
const BASE_UPDATE_WORLD_DATA: u32 = 0x00c6_b670;
const BASE_ADD_VIEWER_STRINGS: u32 = 0x00c6_c430;
/// `operator new` (`00aa13e0`, cdecl size).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// A function of no effect called on a body (`00a29680`: one `RET`), and `00a68bf0` (thiscall, no
/// stack arguments), called on the scene-graph node (neither has an engine map name).
const NO_EFFECT: u32 = 0x00a2_9680;
const NODE_UPDATE: u32 = 0x00a6_8bf0;
/// `hkpRigidBody::setLinearVelocity` (engine map name, `005616d0`) and `setAngularVelocity`
/// (`00561800`) of the Havok object (thiscall: a vector), and the same for the body wrapper:
/// `00561690` / `005617c0`; `00560d80` / `00560e10` answer the body wrapper.s linear / angular
/// velocity vector.
const SET_LINEAR_VELOCITY_OBJECT: u32 = 0x0056_16d0;
const SET_ANGULAR_VELOCITY_OBJECT: u32 = 0x0056_1800;
const SET_LINEAR_VELOCITY: u32 = 0x0056_1690;
const SET_ANGULAR_VELOCITY: u32 = 0x0056_17c0;
const GET_LINEAR_VELOCITY: u32 = 0x0056_0d80;
const GET_ANGULAR_VELOCITY: u32 = 0x0056_0e10;
/// The body's rotation as a quaternion (`00560c20`, thiscall: the result) and translation
/// (`00560bc0`, thiscall: the result), the conversion of a quaternion to a rotation matrix
/// (`004f0180`, thiscall on the quaternion: the matrix), and `NiQuaternion::FromRotation`
/// (engine map name, `00a6df40`, thiscall on the quaternion: the rotation matrix).
const BODY_GET_ROTATION: u32 = 0x0056_0c20;
const BODY_GET_TRANSLATION: u32 = 0x0056_0bc0;
const QUATERNION_TO_MATRIX: u32 = 0x004f_0180;
const QUATERNION_FROM_ROTATION: u32 = 0x00a6_df40;
/// The conversion of a position to a Havok vector (`004a3e00`, cdecl: result, source) and of a
/// Havok vector to a point (`00458620`, cdecl: result, source); both answer the result.
const POSITION_TO_VECTOR: u32 = 0x004a_3e00;
const VECTOR_TO_POINT: u32 = 0x0045_8620;
/// `bhkRigidBody::ApplyHardKeyframe` (engine map name, `00c8e160`, thiscall on the body: the
/// position, the quaternion, the lock), the quaternion normalisation (`005611c0`, thiscall on
/// the quaternion) and `bhkWorldObject::GethkWorld` (engine map name, `00c87a40`).
const APPLY_HARD_KEYFRAME: u32 = 0x00c8_e160;
const NORMALIZE_QUATERNION: u32 = 0x0056_11c0;
const GET_HK_WORLD: u32 = 0x00c8_7a40;
/// Tests on a body or on the object (`0047b470`, thiscall on the collision object, answers a
/// bool), and a test on a motion type word (`00c8cce0`, cdecl: the address of the word,
/// answers a bool); `00517630` (thiscall on the body) answers a word for the second.
const OBJECT_TEST: u32 = 0x0047_b470;
const MOTION_TYPE_TEST: u32 = 0x00c8_cce0;
const BODY_WORD: u32 = 0x0051_7630;
/// `hkpEntity::activate` (engine map name, `00c9c1d0`, thiscall).
const ACTIVATE: u32 = 0x00c9_c1d0;
/// A dynamic cast (`00653290`, thiscall on the object: the RTTI; answers the object or 0) and
/// the RTTI of the target class (`012681c0`).
const DYNAMIC_CAST: u32 = 0x0065_3290;
const BODY_RTTI: u32 = 0x0126_81c0;
/// The recursive lock (`01267b80`: owner at `+0`, count at `+4`) and its acquire (`0040fbf0`,
/// thiscall: one unread stack word). Releasing is done in line: the count is lowered and the
/// owner cleared when it reaches 0.
const LOCK: u32 = 0x0126_7b80;
const LOCK_ACQUIRE: u32 = 0x0040_fbf0;
/// The shared zero vector (`01267e30`), the velocity epsilon (`float`, `01017d00`) and the
/// sign mask of four lanes (`01017d10`) the velocity checks use; and the default velocity
/// (three words, `011f426c`) `GetVelocity` answers without a body.
const ZERO_VECTOR: u32 = 0x0126_7e30;
const VELOCITY_EPSILON: u32 = 0x0101_7d00;
const ABS_MASK: u32 = 0x0101_7d10;
const DEFAULT_VELOCITY: u32 = 0x011f_426c;
/// The vtable stored in a new object (`0103046c`).
const OBJECT_VTABLE: u32 = 0x0103_046c;
/// `NiGetViewerString` (`00a7d330`, cdecl: name; `00a7d370`, cdecl: label and bool word), the
/// string array's resize (`0096ad30`) and store (`0096ae90`), the object's RTTI name pointer
/// (`bhkCollisionObject::ms_RTTI`, `012043f8`) and the label `010c3a30`.
const GET_VIEWER_STRING: u32 = 0x00a7_d330;
const GET_VIEWER_STRING_OF_BOOL: u32 = 0x00a7_d370;
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_SET_AT: u32 = 0x0096_ae90;
const COLLISION_OBJECT_RTTI: u32 = 0x0120_43f8;
const VIEWER_LABEL: u32 = 0x010c_3a30;
/// Per-thread word the Havok-allocating functions set to 0x10 (`TLS + 0x2b4`).
const SCOPE_WORD: u32 = 0x2b4;

layout! {
    /// `bhkCollisionObject` (Xbox PDB, 0x14 bytes).
    pub struct BhkCollisionObject: 0x14 {
        /// `m_pkSceneObject` (Xbox PDB): the scene-graph node.
        0x08 m_pkSceneObject: Ptr,
        /// `m_uFlags` (Xbox PDB).
        0x0C m_uFlags: u16,
        /// `spWorldObject` (Xbox PDB): the Havok body.
        0x10 spWorldObject: Ptr,
    }
}

/// Sets the per-thread word `TLS + 0x2b4` to 0x10, answering the thread block and the old value.
fn enter_scope(e: &mut Engine) -> (u32, u32) {
    let thread = e.tls();
    let saved = e.mem.u32(thread + SCOPE_WORD);
    e.mem.set_u32(thread + SCOPE_WORD, 0x10);
    (thread, saved)
}

fn leave_scope(e: &mut Engine, scope: (u32, u32)) {
    e.mem.set_u32(scope.0 + SCOPE_WORD, scope.1);
}

/// Stores the string pointer `text` in the string array: grows the array by its grow-by
/// (`0096ad30`) when the size (`+0xa`) has reached the capacity (`+8`), then stores through
/// `0096ae90` (array, index = the old size, address of the pointer).
fn store_text(e: &mut Engine, array: u32, text: u32) {
    let size = u32::from(e.mem.u16(array + 0xa));
    if size >= u32::from(e.mem.u16(array + 8)) {
        let grow_by = u32::from(e.mem.u16(array + 0xe));
        e.call(STRING_ARRAY_SET_SIZE, &args![array, grow_by + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRING_ARRAY_SET_AT, &args![array, size, slot]);
    });
}

/// The velocity checks of the exe (`SUBPS`, `ANDPS` with the sign mask, `CMPLEPS`,
/// `MOVMSKPS`): true when each of the first `lanes` lanes of the vector at `vector` differs
/// from the zero vector by no more than the epsilon (`float` at `01017d00`); a NaN lane is
/// not within.
fn velocity_within_epsilon(e: &Engine, vector: u32, lanes: u32) -> bool {
    let epsilon = e.mem.f32(VELOCITY_EPSILON);
    (0..lanes).all(|lane| {
        let difference = e.mem.f32(vector + 4 * lane) - e.mem.f32(ZERO_VECTOR + 4 * lane);
        let magnitude = f32::from_bits(difference.to_bits() & e.mem.u32(ABS_MASK + 4 * lane));
        magnitude <= epsilon
    })
}

/// Releases the recursive lock `01267b80` in line: lowers the count (`+4`) and clears the owner
/// (`+0`) when it reaches 0.
fn release_lock(e: &mut Engine) {
    let count = e.mem.u32(LOCK + 4).wrapping_sub(1);
    e.mem.set_u32(LOCK + 4, count);
    if count == 0 {
        e.mem.set_u32(LOCK, 0);
    }
}

/// Moves the body to the node's position and rotation: the position (`004a3e00` on the node's
/// translation), the rotation as the quaternion `(x, y, z, w)` of the `NiQuaternion`
/// `(w, x, y, z)` at `quaternion` (each word copied through `FLD`/`FSTP`) normalised
/// (`005611c0`), then, under the lock (`0040fbf0`, released in line), the body's virtual `+0xe8`
/// with both.
fn move_body(e: &mut Engine, body: u32, translation: u32, quaternion: u32) {
    e.with_stack(0x20, |e, vectors| {
        let position = vectors.addr();
        let rotation = position + 0x10;
        e.call(POSITION_TO_VECTOR, &args![position, translation]);
        for (target, source) in [(0, 4), (4, 8), (8, 0xc), (0xc, 0)] {
            let bits = x87_quiet_bits(e.mem.u32(quaternion + source));
            e.mem.set_u32(rotation + target, bits);
        }
        e.call(NORMALIZE_QUATERNION, &args![rotation]);
        e.call(LOCK_ACQUIRE, &args![LOCK, 0u32]);
        e.vcall(body, 0xe8, &args![position, rotation]);
    });
    release_lock(e);
}

/// `00c6c330`: the jump target of `00c65800`.
const JUMP_TARGET_00C65800: u32 = 0x00c6_c330;

/// `00c6bb70`: the jump target of `00c65810`.
const JUMP_TARGET_00C65810: u32 = 0x00c6_bb70;

/// `00c6c380`: the jump target of `00c65820`.
const JUMP_TARGET_00C65820: u32 = 0x00c6_c380;

/// `00c6b730`: the jump target of `00c87fa0`.
const JUMP_TARGET_00C87FA0: u32 = 0x00c6_b730;

/// `00c6b760`: the jump target of `00c87fb0`.
const JUMP_TARGET_00C87FB0: u32 = 0x00c6_b760;

// Translated from 00c65800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCollisionObject::LoadBinary` (Xbox PDB): a 5-byte jump to `00c6c330` (engine map: `bhkNiCollisionObject::LoadBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LoadBinary@bhkCollisionObject@@UAAXAAVNiStream@@@Z`.
pub fn bhk_collision_object_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00C65800, &args![this, stream]);
}

// Translated from 00c65810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCollisionObject::LinkObject` (Xbox PDB): a 5-byte jump to `00c6bb70` (engine map: `bhkNiCollisionObject::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@bhkCollisionObject@@UAAXAAVNiStream@@@Z`.
pub fn bhk_collision_object_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00C65810, &args![this, stream]);
}

// Translated from 00c65820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCollisionObject::SaveBinary` (Xbox PDB): a 5-byte jump to `00c6c380` (engine map: `bhkNiCollisionObject::SaveBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SaveBinary@bhkCollisionObject@@UAAXAAVNiStream@@@Z`.
pub fn bhk_collision_object_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00C65820, &args![this, stream]);
}

// Translated from 00c87fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCollisionObject::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00c6b730` (engine map: `bhkNiCollisionObject::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@bhkCollisionObject@@UAA_NAAVNiStream@@@Z`.
pub fn bhk_collision_object_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(JUMP_TARGET_00C87FA0, &args![this, stream]).bool()
}

// Translated from 00c87fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCollisionObject::IsEqual` (Xbox PDB): a 5-byte jump to `00c6b760` (engine map: `bhkNiCollisionObject::IsEqual`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?IsEqual@bhkCollisionObject@@UAA_NPAVNiObject@@@Z`.
pub fn bhk_collision_object_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(JUMP_TARGET_00C87FB0, &args![this, other]).bool()
}

// Translated from 00c65830 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c65830 - the FS:[0] exception-unwinding frame is not modeled.
/// Allocates (0x14 bytes, `operator new`) and constructs a collision object (unconfirmed name;
/// the Xbox PDB has a static `bhkCollisionObject::CreateObject`): the base constructor
/// (`00c6b910`) and the vtable `0103046c`. Answers the object, or 0 when the allocation failed.
pub fn fn_00c65830(e: &mut Engine) -> Ptr {
    let block = e.call(OPERATOR_NEW, &args![0x14u32]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    e.call(BASE_CONSTRUCT, &args![block]);
    e.mem.set_u32(block, OBJECT_VTABLE);
    Ptr::new(block)
}

// Translated from 00c65890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCollisionObject::UpdateWorldData` (Xbox PDB), `(update_data)`: when the Havok body has
/// bit 6 of its flags word (`+0x10`) set, only runs `00a68bf0` on the node (when there is one)
/// and ignores the argument. Otherwise it continues in the base `UpdateWorldData` (`00c6b670`)
/// with the argument. (Ghidra's bounds for this function run into the next one; the exe code is
/// the thirteen instructions ending in the jump.)
pub fn bhk_collision_object_update_world_data(
    e: &mut Engine,
    this: Ptr<BhkCollisionObject>,
    update_data: Ptr,
) {
    let body = e.get(this, BhkCollisionObject::spWorldObject).addr();
    if body != 0 && (e.mem.u32(body + 0x10) >> 6) & 1 != 0 {
        let node = e.get(this, BhkCollisionObject::m_pkSceneObject).addr();
        if node != 0 {
            e.call(NODE_UPDATE, &args![node]);
        }
        return;
    }
    e.call(BASE_UPDATE_WORLD_DATA, &args![this, update_data]);
}

// Translated from 00c658c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCollisionObject::ClearVelocities` (Xbox PDB): with a Havok body, for its Havok object's
/// linear velocity (`+0x1b0`) and then angular velocity (`+0x1c0`; the zero vector `01267e30`
/// when there is no Havok object): unless each of the first three lanes is within the epsilon
/// (`01017d00`) of zero (SSE code, NaN not within) and while the Havok object exists, sets it
/// to the zero vector (`005616d0`, `00561800`; `00a29680` runs on the body before and after).
pub fn bhk_collision_object_clear_velocities(e: &mut Engine, this: Ptr<BhkCollisionObject>) {
    let body = e.get(this, BhkCollisionObject::spWorldObject).addr();
    if body == 0 {
        return;
    }
    for (offset, setter) in [
        (0x1b0, SET_LINEAR_VELOCITY_OBJECT),
        (0x1c0, SET_ANGULAR_VELOCITY_OBJECT),
    ] {
        let object = e.mem.u32(body + 8);
        let vector = if object != 0 {
            object + offset
        } else {
            ZERO_VECTOR
        };
        if !velocity_within_epsilon(e, vector, 3) && object != 0 {
            e.call(NO_EFFECT, &args![body]);
            e.call(setter, &args![object, ZERO_VECTOR]);
            e.call(NO_EFFECT, &args![body]);
        }
    }
}

// Translated from 00c659a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCollisionObject::QKeyframe` (Xbox PDB): false without a Havok body; true when the body
/// has no Havok object or the Havok object's motion type (byte `+0xe8`) is 4 or 5; false
/// otherwise.
pub fn bhk_collision_object_q_keyframe(e: &mut Engine, this: Ptr<BhkCollisionObject>) -> bool {
    let body = e.get(this, BhkCollisionObject::spWorldObject).addr();
    if body == 0 {
        return false;
    }
    let object = e.mem.u32(body + 8);
    if object == 0 {
        return true;
    }
    matches!(e.mem.u8(object + 0xe8), 4 | 5)
}

// Translated from 00c659d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c659d0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkCollisionObject::CreateClone` (Xbox PDB), `(process)`: inside the `TLS + 0x2b4 = 0x10`
/// scope, creates a collision object as [`fn_00c65830`] does (0 when the allocation fails) and
/// runs `CopyMembers` (`00c6ba30`, with the clone and the process) on `this`. Answers the clone.
pub fn bhk_collision_object_create_clone(
    e: &mut Engine,
    this: Ptr<BhkCollisionObject>,
    process: Ptr,
) -> Ptr {
    let scope = enter_scope(e);
    let clone = fn_00c65830(e);
    e.call(COPY_MEMBERS, &args![this, clone, process]);
    leave_scope(e, scope);
    clone
}

// Translated from 00c65a80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c65a80 - the scale of the transform passed to SetTransform is never written (uninitialised stack in the exe; here 0).
/// `bhkCollisionObject::hktoNiTransform` (Xbox PDB): does nothing without a Havok body or Havok
/// object. When the Havok object's motion type is 5 and its `+8` is set, only runs `00a68bf0` on
/// the node (when there is one). Otherwise builds a `NiTransform` on the stack (the body's
/// rotation as a quaternion `00560c20`, turned into the matrix `004f0180`; its translation
/// `00560bc0` at the transform's `+0x24`) and gives it to `SetTransform` (virtual `+0xb8`).
pub fn bhk_collision_object_hk_to_ni_transform(e: &mut Engine, this: Ptr<BhkCollisionObject>) {
    let body = e.get(this, BhkCollisionObject::spWorldObject).addr();
    if body == 0 {
        return;
    }
    let object = e.mem.u32(body + 8);
    if object == 0 {
        return;
    }
    if e.mem.u8(object + 0xe8) == 5 && e.mem.u32(object + 8) != 0 {
        let node = e.get(this, BhkCollisionObject::m_pkSceneObject).addr();
        if node != 0 {
            e.call(NODE_UPDATE, &args![node]);
        }
        return;
    }
    // The quaternion (16 bytes) and the transform (0x34 bytes: matrix, translation, scale).
    e.with_stack(0x44, |e, frame| {
        let quaternion = frame.addr();
        let transform = quaternion + 0x10;
        e.call(BODY_GET_ROTATION, &args![body, quaternion]);
        e.call(QUATERNION_TO_MATRIX, &args![quaternion, transform]);
        e.call(BODY_GET_TRANSLATION, &args![body, transform + 0x24]);
        e.vcall(this.addr(), 0xb8, &args![transform]);
    });
}

// Translated from 00c65b00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c65b00 - the /GS cookie check is not modeled.
/// `bhkCollisionObject::NitohkTransform` (Xbox PDB): moves the Havok body to the scene-graph
/// node. Does nothing without a node (`00a68bf0` runs on a node first) or a body; for a body
/// whose motion type is 5 (or that has no Havok object) nothing happens when
/// `GethkWorld` (`00c87a40`) answers non-zero, and nothing without a Havok object. The node's
/// rotation (`+0x68`) becomes a `NiQuaternion` (`00a6df40`). Three cases follow:
///
/// * the object's bit 6 of `m_uFlags` is set, or the node's flags (`+0x30`) have any of the
///   bits 3, 4, 22 or 27 set and neither `00c8cce0` on the body's motion type nor
///   `0047b470` on the object answer true: [`move_body`] moves it by `SetPosAndRot`
///   (virtual `+0xe8`);
/// * the node's flags have none of those bits: the body's position (virtual `+0xd4`) and the
///   node's translation are compared; when they are more than 1.0 apart (squared distance
///   above 1.0, a NaN counts as near) the body is hard keyframed (`00c8e160`, with the
///   translation, the quaternion and the lock) if `0047b470` or `00c8cce0` answer true, and
///   moved otherwise; when they are near, the body's linear velocity and then angular
///   velocity (`00560d80`, `00560e10`) are set to zero (`00561690`, `005617c0`) unless all four
///   lanes are within the epsilon;
/// * the other cases with the flag bits set hard keyframe or move as above.
pub fn bhk_collision_object_ni_to_hk_transform(e: &mut Engine, this: Ptr<BhkCollisionObject>) {
    let node = e.get(this, BhkCollisionObject::m_pkSceneObject).addr();
    let body = e.get(this, BhkCollisionObject::spWorldObject).addr();
    if node == 0 {
        return;
    }
    e.call(NODE_UPDATE, &args![node]);
    if body == 0 {
        return;
    }
    let object = e.mem.u32(body + 8);
    let motion_type = if object != 0 {
        u32::from(e.mem.u8(object + 0xe8))
    } else {
        5
    };
    if motion_type == 5 && e.call(GET_HK_WORLD, &args![body]).u32() != 0 {
        return;
    }
    if e.mem.u32(body + 8) == 0 {
        return;
    }
    let rotation = node + 0x68;
    let translation = node + 0x8c;
    // The quaternion (16 bytes) and the word holding the motion type.
    e.with_stack(0x20, |e, frame| {
        let quaternion = frame.addr();
        let type_word = quaternion + 0x10;
        e.mem.set_u32(type_word, motion_type);
        e.call(QUATERNION_FROM_ROTATION, &args![quaternion, rotation]);
        let flags = e.mem.u8(this.addr() + 0xc);
        if (flags >> 6) & 1 != 0 {
            move_body(e, body, translation, quaternion);
            return;
        }
        let node_flags = e.mem.u32(node + 0x30);
        let flagged = [0x16, 0x1b, 3, 4]
            .into_iter()
            .any(|bit| (node_flags >> bit) & 1 != 0);
        if !flagged {
            let squared_distance = e.with_stack(0x20, |e, vectors| {
                let wanted = vectors.addr();
                let current = wanted + 0x10;
                e.call(POSITION_TO_VECTOR, &args![wanted, translation]);
                e.vcall(body, 0xd4, &args![current]);
                let squares: [f32; 4] = std::array::from_fn(|lane| {
                    let lane = 4 * lane as u32;
                    let difference = e.mem.f32(wanted + lane) - e.mem.f32(current + lane);
                    difference * difference
                });
                squares[2] + (squares[1] + squares[0])
            });
            let far = squared_distance > 1.0;
            if !far {
                // Near: zero the velocities that are not already (nearly) zero.
                let linear = e.call(GET_LINEAR_VELOCITY, &args![body]).u32();
                if !velocity_within_epsilon(e, linear, 4) {
                    e.call(SET_LINEAR_VELOCITY, &args![body, ZERO_VECTOR]);
                }
                let angular = e.call(GET_ANGULAR_VELOCITY, &args![body]).u32();
                if !velocity_within_epsilon(e, angular, 4) {
                    e.call(SET_ANGULAR_VELOCITY, &args![body, ZERO_VECTOR]);
                }
                return;
            }
        }
        if e.call(OBJECT_TEST, &args![this]).bool()
            || e.call(MOTION_TYPE_TEST, &args![type_word]).bool()
        {
            e.call(
                APPLY_HARD_KEYFRAME,
                &args![body, translation, quaternion, LOCK],
            );
            return;
        }
        move_body(e, body, translation, quaternion);
    });
}

// Translated from 00c65ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCollisionObject::GetVelocity` (Xbox PDB), `(out)`: stores the Havok object's linear
/// velocity (`+0x1b0`; the zero vector `01267e30` without a Havok object) converted to a point
/// (`00458620`) at `out`; without a body, the three words at `011f426c` instead. Answers `out`.
pub fn bhk_collision_object_get_velocity(
    e: &mut Engine,
    this: Ptr<BhkCollisionObject>,
    out: Ptr,
) -> Ptr {
    let body = e.get(this, BhkCollisionObject::spWorldObject).addr();
    if body == 0 {
        for word in 0..3 {
            let value = e.mem.u32(DEFAULT_VELOCITY + 4 * word);
            e.mem.set_u32(out.addr() + 4 * word, value);
        }
        return out;
    }
    let object = e.mem.u32(body + 8);
    let velocity = if object != 0 {
        object + 0x1b0
    } else {
        ZERO_VECTOR
    };
    e.call(VECTOR_TO_POINT, &args![out, velocity]);
    out
}

// Translated from 00c65f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCollisionObject::SetMotionType` (Xbox PDB), `(motion_type, object, update_flag)`: the
/// body is `object` cast by `00653290` to the class with RTTI `012681c0`, or, when there is no
/// such object, the collision object's own. Nothing happens without a body or when its motion
/// type (byte `+0xe8` of its Havok object, 5 without one) is already `motion_type`, or when
/// `motion_type` is 1 and `00c8cce0` answers true for the current type. Otherwise, under the
/// lock `01267b80`, the body's virtual `+0xe4` sets the type; on success `m_uFlags` bit 3 is set
/// or cleared by `00c8cce0` on the new type and, when `update_flag` is set, the word `00517630`
/// answers for the body goes through `00c8cce0` and, when true and the Havok object exists, it
/// is activated (`00c9c1d0`; `00a29680` runs on the body before and after).
pub fn bhk_collision_object_set_motion_type(
    e: &mut Engine,
    this: Ptr<BhkCollisionObject>,
    motion_type: u32,
    object: Ptr,
    update_flag: bool,
) {
    let mut body = 0;
    if object.addr() != 0 {
        body = e.call(DYNAMIC_CAST, &args![object, BODY_RTTI]).u32();
    }
    if body == 0 {
        body = e.get(this, BhkCollisionObject::spWorldObject).addr();
        if body == 0 {
            return;
        }
    }
    let current_type = |e: &Engine| {
        let hk = e.mem.u32(body + 8);
        if hk != 0 {
            u32::from(e.mem.u8(hk + 0xe8))
        } else {
            5
        }
    };
    let current = current_type(e);
    if current == motion_type {
        return;
    }
    if motion_type == 1 {
        let skip = e.with_stack(4, |e, word| {
            e.mem.set_u32(word.addr(), current);
            e.call(MOTION_TYPE_TEST, &args![word]).bool()
        });
        if skip {
            return;
        }
    }
    e.call(LOCK_ACQUIRE, &args![LOCK, 0u32]);
    let changed = e.vcall(body, 0xe4, &args![motion_type]).bool();
    release_lock(e);
    if !changed {
        return;
    }
    let new_type = current_type(e);
    let flagged = e.with_stack(4, |e, word| {
        e.mem.set_u32(word.addr(), new_type);
        e.call(MOTION_TYPE_TEST, &args![word]).bool()
    });
    let flags = e.get(this, BhkCollisionObject::m_uFlags);
    e.set(
        this,
        BhkCollisionObject::m_uFlags,
        if flagged { flags | 8 } else { flags & 0xfff7 },
    );
    if !update_flag {
        return;
    }
    let word = e.call(BODY_WORD, &args![body]).u32();
    let active = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), word);
        e.call(MOTION_TYPE_TEST, &args![slot]).bool()
    });
    if !active {
        return;
    }
    let hk = e.mem.u32(body + 8);
    if hk != 0 {
        e.call(NO_EFFECT, &args![body]);
        e.call(ACTIVATE, &args![hk]);
        e.call(NO_EFFECT, &args![body]);
    }
}

// Translated from 00c66060 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c66060 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkCollisionObject::AddViewerStrings` (Xbox PDB), `(strings)`: inside the
/// `TLS + 0x2b4 = 0x10` scope, the base strings (`00c6c430`), the class name
/// (`NiGetViewerString` of the first word of the RTTI at `012043f8`) and a bool string
/// (`00a7d370`, label `010c3a30`) that is 1 when `m_uFlags` has bit 5 set and bit 6 clear.
pub fn bhk_collision_object_add_viewer_strings(
    e: &mut Engine,
    this: Ptr<BhkCollisionObject>,
    strings: Ptr,
) {
    let scope = enter_scope(e);
    e.call(BASE_ADD_VIEWER_STRINGS, &args![this, strings]);
    let name = e.mem.u32(COLLISION_OBJECT_RTTI);
    let text = e.call(GET_VIEWER_STRING, &args![name]).u32();
    store_text(e, strings.addr(), text);
    let flags = e.get(this, BhkCollisionObject::m_uFlags);
    let value = u32::from(flags & 0x20 != 0 && (flags >> 6) & 1 == 0);
    let text = e
        .call(GET_VIEWER_STRING_OF_BOOL, &args![VIEWER_LABEL, value])
        .u32();
    store_text(e, strings.addr(), text);
    leave_scope(e, scope);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c6_5830, fn_00c65830() -> Ptr),
        entry!(
            0x00c6_5890,
            bhk_collision_object_update_world_data(Ptr<BhkCollisionObject>, Ptr)
        ),
        entry!(
            0x00c6_58c0,
            bhk_collision_object_clear_velocities(Ptr<BhkCollisionObject>)
        ),
        entry!(
            0x00c6_59a0,
            bhk_collision_object_q_keyframe(Ptr<BhkCollisionObject>) -> bool
        ),
        entry!(
            0x00c6_59d0,
            bhk_collision_object_create_clone(Ptr<BhkCollisionObject>, Ptr) -> Ptr
        ),
        entry!(
            0x00c6_5a80,
            bhk_collision_object_hk_to_ni_transform(Ptr<BhkCollisionObject>)
        ),
        entry!(
            0x00c6_5b00,
            bhk_collision_object_ni_to_hk_transform(Ptr<BhkCollisionObject>)
        ),
        entry!(
            0x00c6_5ef0,
            bhk_collision_object_get_velocity(Ptr<BhkCollisionObject>, Ptr) -> Ptr
        ),
        entry!(
            0x00c6_5f40,
            bhk_collision_object_set_motion_type(Ptr<BhkCollisionObject>, u32, Ptr, bool)
        ),
        entry!(
            0x00c6_6060,
            bhk_collision_object_add_viewer_strings(Ptr<BhkCollisionObject>, Ptr)
        ),
        entry!(0x00c6_5800, bhk_collision_object_load_binary(Ptr, Ptr)),
        entry!(0x00c6_5810, bhk_collision_object_link_object(Ptr, Ptr)),
        entry!(0x00c6_5820, bhk_collision_object_save_binary(Ptr, Ptr)),
        entry!(0x00c8_7fa0, bhk_collision_object_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00c8_7fb0, bhk_collision_object_is_equal(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_AREA: u32 = 0x7200_0000;
    const THIS_VTABLE: u32 = 0x7001_1000;
    const BODY_VTABLE: u32 = 0x7001_2000;
    const SET_TRANSFORM_SLOT: u32 = 0x7000_00b8;
    const GET_POSITION_SLOT: u32 = 0x7000_00d4;
    const SET_TYPE_SLOT: u32 = 0x7000_00e4;
    const MOVE_SLOT: u32 = 0x7000_00e8;
    // Cells of the test area the doubles read and write.
    const CURRENT_POSITION: u32 = TEST_AREA;
    const LOCK_COUNT_AT_MOVE: u32 = TEST_AREA + 0x10;
    const CAST_RESULT: u32 = TEST_AREA + 0x14;
    const SET_TYPE_SUCCEEDS: u32 = TEST_AREA + 0x18;
    const LOCK_COUNT_AT_SET_TYPE: u32 = TEST_AREA + 0x1c;
    const MOVED_ROTATION: u32 = TEST_AREA + 0x20;
    const MOVED_POSITION: u32 = TEST_AREA + 0x30;
    const OBJECT_TEST_RESULT: u32 = TEST_AREA + 0x40;
    const MOTION_TEST_RESULT: u32 = TEST_AREA + 0x44;
    const MOTION_TEST_WORD: u32 = TEST_AREA + 0x48;
    const LINEAR_VELOCITY: u32 = TEST_AREA + 0x50;
    const ANGULAR_VELOCITY: u32 = TEST_AREA + 0x60;
    const SCOPE_SEEN: u32 = TEST_AREA + 0x70;
    const BODY_WORD_RESULT: u32 = TEST_AREA + 0x74;

    fn logged_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(TEST_AREA, 0x100);
        e.call_log = Some(vec![]);
        e
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    fn order_of(e: &Engine, addresses: &[u32]) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|c| c.0)
            .filter(|a| addresses.contains(a))
            .collect()
    }

    fn clear_log(e: &mut Engine) {
        e.call_log = Some(vec![]);
    }

    fn set_vector(e: &mut Engine, address: u32, lanes: [f32; 4]) {
        for (lane, value) in lanes.into_iter().enumerate() {
            e.mem.set_f32(address + 4 * lane as u32, value);
        }
    }

    /// A collision object with a node (`+8`, translation `(10, 0, 0)` at its `+0x8c`) and a body
    /// (`+0x10`) whose Havok object (`+8`) has the motion type 2. The virtual slots of both
    /// objects are doubles: `SetTransform` (`+0xb8`), the body's position (`+0xd4`, copies the
    /// vector at `CURRENT_POSITION`), set motion type (`+0xe4`, succeeds when
    /// `SET_TYPE_SUCCEEDS` is non-zero) and `SetPosAndRot` (`+0xe8`, keeps its vectors and the
    /// lock count). The lock acquire double raises the count.
    struct Scene {
        this: u32,
        node: u32,
        body: u32,
        hk: u32,
    }

    fn scene(e: &mut Engine) -> Scene {
        let mut table = vec![0u32; 0xbc / 4];
        table[0xb8 / 4] = SET_TRANSFORM_SLOT;
        e.put_vtable(THIS_VTABLE, &table);
        let mut table = vec![0u32; 0xec / 4];
        table[0xd4 / 4] = GET_POSITION_SLOT;
        table[0xe4 / 4] = SET_TYPE_SLOT;
        table[0xe8 / 4] = MOVE_SLOT;
        e.put_vtable(BODY_VTABLE, &table);
        e.register(SET_TRANSFORM_SLOT, |_, _| Ret::default());
        e.register(GET_POSITION_SLOT, |e, a| {
            let bytes = e.mem.bytes(CURRENT_POSITION, 16);
            e.mem.write(a[1], &bytes);
            Ret::default()
        });
        e.register(SET_TYPE_SLOT, |e, a| {
            let lock_count = e.mem.u32(LOCK + 4);
            e.mem.set_u32(LOCK_COUNT_AT_SET_TYPE, lock_count);
            if e.mem.u32(SET_TYPE_SUCCEEDS) == 0 {
                return 0u32.into_ret();
            }
            let hk = e.mem.u32(a[0] + 8);
            if hk != 0 {
                e.mem.set_u8(hk + 0xe8, a[1] as u8);
            }
            1u32.into_ret()
        });
        e.register(MOVE_SLOT, |e, a| {
            let lock_count = e.mem.u32(LOCK + 4);
            e.mem.set_u32(LOCK_COUNT_AT_MOVE, lock_count);
            let position = e.mem.bytes(a[1], 16);
            e.mem.write(MOVED_POSITION, &position);
            let rotation = e.mem.bytes(a[2], 16);
            e.mem.write(MOVED_ROTATION, &rotation);
            Ret::default()
        });
        e.register(LOCK_ACQUIRE, |e, _| {
            let count = e.mem.u32(LOCK + 4);
            e.mem.set_u32(LOCK + 4, count + 1);
            e.mem.set_u32(LOCK, 1);
            Ret::default()
        });
        e.register(NODE_UPDATE, |_, _| Ret::default());
        let this = e.mem.alloc(0x14);
        let node = e.mem.alloc(0xa0);
        let body = e.mem.alloc(0x20);
        let hk = e.mem.alloc(0x200);
        e.mem.set_u32(this, THIS_VTABLE);
        e.mem.set_u32(this + 8, node);
        e.mem.set_u32(this + 0x10, body);
        e.mem.set_u32(body, BODY_VTABLE);
        e.mem.set_u32(body + 8, hk);
        e.mem.set_u8(hk + 0xe8, 2);
        set_vector(e, node + 0x8c, [10.0, 0.0, 0.0, 0.0]);
        Scene {
            this,
            node,
            body,
            hk,
        }
    }

    fn lock_is_free(e: &Engine) -> bool {
        e.mem.u32(LOCK) == 0 && e.mem.u32(LOCK + 4) == 0
    }

    #[test]
    fn fn_00c65830_allocates_and_constructs_a_collision_object() {
        let mut e = logged_engine();
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(BASE_CONSTRUCT, |_, _| Ret::default());
        let object = e.call(0x00c6_5830, &args![]).u32();
        assert_ne!(object, 0);
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x14]]);
        assert_eq!(calls_to(&e, BASE_CONSTRUCT), vec![vec![object]]);
        assert_eq!(e.mem.u32(object), OBJECT_VTABLE);
        // A failed allocation answers 0 and constructs nothing.
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00c6_5830, &args![]).u32(), 0);
        assert_eq!(calls_to(&e, BASE_CONSTRUCT).len(), 1);
    }

    #[test]
    fn bhk_collision_object_update_world_data_updates_the_node_for_a_flagged_body() {
        let mut e = logged_engine();
        let scene = scene(&mut e);
        e.register(BASE_UPDATE_WORLD_DATA, |_, _| Ret::default());
        e.mem.set_u32(scene.body + 0x10, 0x40);
        e.call(0x00c6_5890, &args![scene.this, 0x1234u32]);
        assert_eq!(calls_to(&e, NODE_UPDATE), vec![vec![scene.node]]);
        assert!(calls_to(&e, BASE_UPDATE_WORLD_DATA).is_empty());
        // Without a node nothing is updated.
        e.mem.set_u32(scene.this + 8, 0);
        e.call(0x00c6_5890, &args![scene.this, 0x1234u32]);
        assert_eq!(calls_to(&e, NODE_UPDATE).len(), 1);
        assert!(calls_to(&e, BASE_UPDATE_WORLD_DATA).is_empty());
        // Another flag, or no body, continues in the base function.
        e.mem.set_u32(scene.body + 0x10, 0xbf);
        e.call(0x00c6_5890, &args![scene.this, 0x1234u32]);
        e.mem.set_u32(scene.this + 0x10, 0);
        e.call(0x00c6_5890, &args![scene.this, 0x5678u32]);
        assert_eq!(
            calls_to(&e, BASE_UPDATE_WORLD_DATA),
            vec![vec![scene.this, 0x1234], vec![scene.this, 0x5678]]
        );
        assert_eq!(calls_to(&e, NODE_UPDATE).len(), 1);
    }

    #[test]
    fn bhk_collision_object_clear_velocities_zeroes_the_velocities_that_are_not_near_zero() {
        let mut e = logged_engine();
        let scene = scene(&mut e);
        e.register(NO_EFFECT, |_, _| Ret::default());
        e.register(SET_LINEAR_VELOCITY_OBJECT, |_, _| Ret::default());
        e.register(SET_ANGULAR_VELOCITY_OBJECT, |_, _| Ret::default());
        e.set_global(VELOCITY_EPSILON, 0.01f32);
        for lane in 0..4 {
            e.mem.set_u32(ABS_MASK + 4 * lane, 0x7fff_ffff);
        }
        set_vector(&mut e, scene.hk + 0x1b0, [0.5, 0.0, 0.0, 0.0]);
        set_vector(&mut e, scene.hk + 0x1c0, [0.0, 0.0, 0.0, 9.0]);
        e.call(0x00c6_58c0, &args![scene.this]);
        // Only the linear velocity is far from zero (the fourth lane is not looked at).
        assert_eq!(
            calls_to(&e, SET_LINEAR_VELOCITY_OBJECT),
            vec![vec![scene.hk, ZERO_VECTOR]]
        );
        assert!(calls_to(&e, SET_ANGULAR_VELOCITY_OBJECT).is_empty());
        assert_eq!(calls_to(&e, NO_EFFECT), vec![vec![scene.body]; 2]);
        let order = vec![NO_EFFECT, SET_LINEAR_VELOCITY_OBJECT, NO_EFFECT];
        assert_eq!(order_of(&e, &order), order);
        // A NaN lane is not within the epsilon.
        clear_log(&mut e);
        set_vector(&mut e, scene.hk + 0x1b0, [0.0, 0.0, 0.0, 0.0]);
        e.mem.set_u32(scene.hk + 0x1c8, 0x7fc0_0000);
        e.call(0x00c6_58c0, &args![scene.this]);
        assert!(calls_to(&e, SET_LINEAR_VELOCITY_OBJECT).is_empty());
        assert_eq!(
            calls_to(&e, SET_ANGULAR_VELOCITY_OBJECT),
            vec![vec![scene.hk, ZERO_VECTOR]]
        );
        // Without a Havok object, or without a body, nothing is called.
        clear_log(&mut e);
        e.mem.set_u32(scene.body + 8, 0);
        e.call(0x00c6_58c0, &args![scene.this]);
        e.mem.set_u32(scene.this + 0x10, 0);
        e.call(0x00c6_58c0, &args![scene.this]);
        assert!(calls_to(&e, NO_EFFECT).is_empty());
        assert!(calls_to(&e, SET_ANGULAR_VELOCITY_OBJECT).is_empty());
    }

    #[test]
    fn bhk_collision_object_q_keyframe_is_true_for_motion_types_4_and_5() {
        let mut e = logged_engine();
        let scene = scene(&mut e);
        for (motion_type, expected) in [(0u8, false), (2, false), (3, false), (4, true), (5, true)]
        {
            e.mem.set_u8(scene.hk + 0xe8, motion_type);
            assert_eq!(e.call(0x00c6_59a0, &args![scene.this]).bool(), expected);
        }
        // Without a Havok object: true; without a body: false.
        e.mem.set_u32(scene.body + 8, 0);
        assert!(e.call(0x00c6_59a0, &args![scene.this]).bool());
        e.mem.set_u32(scene.this + 0x10, 0);
        assert!(!e.call(0x00c6_59a0, &args![scene.this]).bool());
    }

    #[test]
    fn bhk_collision_object_create_clone_copies_the_members_inside_the_scope() {
        let mut e = logged_engine();
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(BASE_CONSTRUCT, |_, _| Ret::default());
        e.register(COPY_MEMBERS, |e, _| {
            let thread = e.tls();
            let scope = e.mem.u32(thread + SCOPE_WORD);
            e.mem.set_u32(SCOPE_SEEN, scope);
            Ret::default()
        });
        let thread = e.tls();
        e.mem.set_u32(thread + SCOPE_WORD, 0x77);
        let this = e.mem.alloc(0x14);
        let clone = e.call(0x00c6_59d0, &args![this, 0x9000u32]).u32();
        assert_ne!(clone, 0);
        assert_eq!(e.mem.u32(clone), OBJECT_VTABLE);
        assert_eq!(calls_to(&e, BASE_CONSTRUCT), vec![vec![clone]]);
        assert_eq!(calls_to(&e, COPY_MEMBERS), vec![vec![this, clone, 0x9000]]);
        assert_eq!(e.mem.u32(SCOPE_SEEN), 0x10);
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
        // A failed allocation still runs the copy, with 0, and answers 0.
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00c6_59d0, &args![this, 0x9000u32]).u32(), 0);
        assert_eq!(calls_to(&e, COPY_MEMBERS)[1], vec![this, 0, 0x9000]);
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
    }

    #[test]
    fn bhk_collision_object_hk_to_ni_transform_builds_the_transform_from_the_body() {
        let mut e = logged_engine();
        let scene = scene(&mut e);
        e.register(BODY_GET_ROTATION, |_, _| Ret::default());
        e.register(QUATERNION_TO_MATRIX, |_, _| Ret::default());
        e.register(BODY_GET_TRANSLATION, |_, _| Ret::default());
        e.call(0x00c6_5a80, &args![scene.this]);
        let rotation = calls_to(&e, BODY_GET_ROTATION);
        assert_eq!(rotation.len(), 1);
        assert_eq!(rotation[0][0], scene.body);
        let quaternion = rotation[0][1];
        let transform = quaternion + 0x10;
        assert_eq!(
            calls_to(&e, QUATERNION_TO_MATRIX),
            vec![vec![quaternion, transform]]
        );
        assert_eq!(
            calls_to(&e, BODY_GET_TRANSLATION),
            vec![vec![scene.body, transform + 0x24]]
        );
        assert_eq!(
            calls_to(&e, SET_TRANSFORM_SLOT),
            vec![vec![scene.this, transform]]
        );
        let order = vec![
            BODY_GET_ROTATION,
            QUATERNION_TO_MATRIX,
            BODY_GET_TRANSLATION,
            SET_TRANSFORM_SLOT,
        ];
        assert_eq!(order_of(&e, &order), order);
        assert!(calls_to(&e, NODE_UPDATE).is_empty());
        // A static body whose Havok object has the field at +8 only updates the node.
        clear_log(&mut e);
        e.mem.set_u8(scene.hk + 0xe8, 5);
        e.mem.set_u32(scene.hk + 8, 0x1000);
        e.call(0x00c6_5a80, &args![scene.this]);
        assert_eq!(calls_to(&e, NODE_UPDATE), vec![vec![scene.node]]);
        assert!(calls_to(&e, BODY_GET_ROTATION).is_empty());
        e.mem.set_u32(scene.this + 8, 0);
        e.call(0x00c6_5a80, &args![scene.this]);
        assert_eq!(calls_to(&e, NODE_UPDATE).len(), 1);
        // Type 5 without that field builds the transform as above.
        e.mem.set_u32(scene.hk + 8, 0);
        e.call(0x00c6_5a80, &args![scene.this]);
        assert_eq!(calls_to(&e, BODY_GET_ROTATION).len(), 1);
        // Without a Havok object or a body nothing happens.
        clear_log(&mut e);
        e.mem.set_u32(scene.body + 8, 0);
        e.call(0x00c6_5a80, &args![scene.this]);
        e.mem.set_u32(scene.this + 0x10, 0);
        e.call(0x00c6_5a80, &args![scene.this]);
        assert!(order_of(&e, &order).is_empty());
    }

    /// The doubles `NitohkTransform` needs. The node rotation conversion writes `(1, 2, 3, 4)`.
    fn register_ni_to_hk_doubles(e: &mut Engine) {
        e.register(GET_HK_WORLD, |_, _| 0u32.into_ret());
        e.register(QUATERNION_FROM_ROTATION, |e, a| {
            for (lane, value) in [1.0f32, 2.0, 3.0, 4.0].into_iter().enumerate() {
                e.mem.set_f32(a[0] + 4 * lane as u32, value);
            }
            Ret::default()
        });
        e.register(POSITION_TO_VECTOR, |e, a| {
            for lane in 0..3 {
                let value = e.mem.u32(a[1] + 4 * lane);
                e.mem.set_u32(a[0] + 4 * lane, value);
            }
            e.mem.set_u32(a[0] + 12, 0);
            a[0].into_ret()
        });
        e.register(NORMALIZE_QUATERNION, |_, _| Ret::default());
        e.register(OBJECT_TEST, |e, _| e.mem.u32(OBJECT_TEST_RESULT).into_ret());
        e.register(MOTION_TYPE_TEST, |e, a| {
            let word = e.mem.u32(a[0]);
            e.mem.set_u32(MOTION_TEST_WORD, word);
            e.mem.u32(MOTION_TEST_RESULT).into_ret()
        });
        e.register(APPLY_HARD_KEYFRAME, |_, _| Ret::default());
        e.register(GET_LINEAR_VELOCITY, |_, _| LINEAR_VELOCITY.into_ret());
        e.register(GET_ANGULAR_VELOCITY, |_, _| ANGULAR_VELOCITY.into_ret());
        e.register(SET_LINEAR_VELOCITY, |_, _| Ret::default());
        e.register(SET_ANGULAR_VELOCITY, |_, _| Ret::default());
        e.set_global(VELOCITY_EPSILON, 0.01f32);
        for lane in 0..4 {
            e.mem.set_u32(ABS_MASK + 4 * lane, 0x7fff_ffff);
        }
    }

    fn moved_rotation(e: &Engine) -> [f32; 4] {
        std::array::from_fn(|lane| e.mem.f32(MOVED_ROTATION + 4 * lane as u32))
    }

    #[test]
    fn bhk_collision_object_ni_to_hk_transform_moves_or_keyframes_the_body() {
        let mut e = logged_engine();
        let scene = scene(&mut e);
        register_ni_to_hk_doubles(&mut e);
        // No node: nothing at all.
        e.mem.set_u32(scene.this + 8, 0);
        e.call(0x00c6_5b00, &args![scene.this]);
        assert!(calls_to(&e, NODE_UPDATE).is_empty());
        e.mem.set_u32(scene.this + 8, scene.node);
        // No body: only the node is updated.
        e.mem.set_u32(scene.this + 0x10, 0);
        e.call(0x00c6_5b00, &args![scene.this]);
        assert_eq!(calls_to(&e, NODE_UPDATE), vec![vec![scene.node]]);
        assert!(calls_to(&e, GET_HK_WORLD).is_empty());
        e.mem.set_u32(scene.this + 0x10, scene.body);
        // Motion type 5 with a world: nothing more. A body without Havok object counts as 5.
        clear_log(&mut e);
        e.mem.set_u8(scene.hk + 0xe8, 5);
        e.register(GET_HK_WORLD, |_, _| 1u32.into_ret());
        e.call(0x00c6_5b00, &args![scene.this]);
        assert_eq!(calls_to(&e, GET_HK_WORLD), vec![vec![scene.body]]);
        assert!(calls_to(&e, QUATERNION_FROM_ROTATION).is_empty());
        e.mem.set_u32(scene.body + 8, 0);
        e.call(0x00c6_5b00, &args![scene.this]);
        assert_eq!(calls_to(&e, GET_HK_WORLD).len(), 2);
        assert!(calls_to(&e, QUATERNION_FROM_ROTATION).is_empty());
        // Type 5 without a world, and no Havok object: nothing more.
        e.register(GET_HK_WORLD, |_, _| 0u32.into_ret());
        e.call(0x00c6_5b00, &args![scene.this]);
        assert!(calls_to(&e, QUATERNION_FROM_ROTATION).is_empty());
        e.mem.set_u32(scene.body + 8, scene.hk);
        e.mem.set_u8(scene.hk + 0xe8, 2);

        // The object's flag bit 6: moved at once, with the rotation (x, y, z, w) = (2, 3, 4, 1)
        // and the position of the node translation, under the lock.
        clear_log(&mut e);
        e.mem.set_u16(scene.this + 0xc, 0x40);
        e.call(0x00c6_5b00, &args![scene.this]);
        let rotation = calls_to(&e, QUATERNION_FROM_ROTATION);
        assert_eq!(rotation.len(), 1);
        assert_eq!(rotation[0][1], scene.node + 0x68);
        assert_eq!(moved_rotation(&e), [2.0, 3.0, 4.0, 1.0]);
        assert_eq!(e.mem.f32(MOVED_POSITION), 10.0);
        assert_eq!(e.mem.u32(LOCK_COUNT_AT_MOVE), 1);
        assert!(lock_is_free(&e));
        assert_eq!(calls_to(&e, NORMALIZE_QUATERNION).len(), 1);
        assert_eq!(calls_to(&e, LOCK_ACQUIRE), vec![vec![LOCK, 0]]);
        let order = vec![
            QUATERNION_FROM_ROTATION,
            POSITION_TO_VECTOR,
            NORMALIZE_QUATERNION,
            LOCK_ACQUIRE,
            MOVE_SLOT,
        ];
        assert_eq!(order_of(&e, &order), order);
        assert!(calls_to(&e, GET_POSITION_SLOT).is_empty());

        // No flags: far away (the body is at the origin), neither test true: moved.
        e.mem.set_u16(scene.this + 0xc, 0);
        clear_log(&mut e);
        e.call(0x00c6_5b00, &args![scene.this]);
        assert_eq!(calls_to(&e, GET_POSITION_SLOT).len(), 1);
        assert_eq!(calls_to(&e, MOVE_SLOT).len(), 1);
        assert!(calls_to(&e, APPLY_HARD_KEYFRAME).is_empty());
        assert_eq!(calls_to(&e, OBJECT_TEST), vec![vec![scene.this]]);
        assert_eq!(e.mem.u32(MOTION_TEST_WORD), 2);
        assert!(lock_is_free(&e));
        // The object test true: hard keyframed with the node translation, the quaternion and
        // the lock (not released here).
        clear_log(&mut e);
        e.mem.set_u32(OBJECT_TEST_RESULT, 1);
        e.call(0x00c6_5b00, &args![scene.this]);
        assert!(calls_to(&e, MOVE_SLOT).is_empty());
        let keyframe = calls_to(&e, APPLY_HARD_KEYFRAME);
        assert_eq!(keyframe.len(), 1);
        assert_eq!(keyframe[0][0], scene.body);
        assert_eq!(keyframe[0][1], scene.node + 0x8c);
        assert_eq!(keyframe[0][3], LOCK);
        assert!(lock_is_free(&e));
        // The motion type test true instead.
        clear_log(&mut e);
        e.mem.set_u32(OBJECT_TEST_RESULT, 0);
        e.mem.set_u32(MOTION_TEST_RESULT, 1);
        e.call(0x00c6_5b00, &args![scene.this]);
        assert_eq!(calls_to(&e, APPLY_HARD_KEYFRAME).len(), 1);
        assert!(calls_to(&e, MOVE_SLOT).is_empty());
        e.mem.set_u32(MOTION_TEST_RESULT, 0);

        // Near (the body is at the node): the velocities that are not near zero are zeroed.
        clear_log(&mut e);
        set_vector(&mut e, CURRENT_POSITION, [10.0, 0.0, 0.0, 0.0]);
        set_vector(&mut e, LINEAR_VELOCITY, [0.5, 0.0, 0.0, 0.0]);
        set_vector(&mut e, ANGULAR_VELOCITY, [0.0, 0.0, 0.0, 0.0]);
        e.call(0x00c6_5b00, &args![scene.this]);
        assert!(calls_to(&e, MOVE_SLOT).is_empty());
        assert!(calls_to(&e, OBJECT_TEST).is_empty());
        assert_eq!(
            calls_to(&e, SET_LINEAR_VELOCITY),
            vec![vec![scene.body, ZERO_VECTOR]]
        );
        assert!(calls_to(&e, SET_ANGULAR_VELOCITY).is_empty());
        // A NaN counts as near, and the fourth lane of the angular velocity is looked at.
        clear_log(&mut e);
        e.mem.set_u32(CURRENT_POSITION, 0x7fc0_0000);
        set_vector(&mut e, LINEAR_VELOCITY, [0.0, 0.0, 0.0, 0.0]);
        set_vector(&mut e, ANGULAR_VELOCITY, [0.0, 0.0, 0.0, 2.0]);
        e.call(0x00c6_5b00, &args![scene.this]);
        assert!(calls_to(&e, SET_LINEAR_VELOCITY).is_empty());
        assert_eq!(
            calls_to(&e, SET_ANGULAR_VELOCITY),
            vec![vec![scene.body, ZERO_VECTOR]]
        );
        assert!(calls_to(&e, MOVE_SLOT).is_empty());

        // The node flag bit 3 (or 4, 22, 27): no distance test, straight to the tests.
        for bit in [3u32, 4, 22, 27] {
            clear_log(&mut e);
            set_vector(&mut e, CURRENT_POSITION, [10.0, 0.0, 0.0, 0.0]);
            e.mem.set_u32(scene.node + 0x30, 1 << bit);
            e.call(0x00c6_5b00, &args![scene.this]);
            assert!(calls_to(&e, GET_POSITION_SLOT).is_empty());
            assert_eq!(calls_to(&e, MOVE_SLOT).len(), 1);
            e.mem.set_u32(MOTION_TEST_RESULT, 1);
            e.call(0x00c6_5b00, &args![scene.this]);
            assert_eq!(calls_to(&e, APPLY_HARD_KEYFRAME).len(), 1);
            e.mem.set_u32(MOTION_TEST_RESULT, 0);
        }
        // A bit that is not one of those leaves the distance test.
        clear_log(&mut e);
        e.mem.set_u32(scene.node + 0x30, 1 << 5);
        e.call(0x00c6_5b00, &args![scene.this]);
        assert_eq!(calls_to(&e, GET_POSITION_SLOT).len(), 1);
    }

    #[test]
    fn bhk_collision_object_get_velocity_converts_the_linear_velocity() {
        let mut e = logged_engine();
        let scene = scene(&mut e);
        e.register(VECTOR_TO_POINT, |_, a| a[0].into_ret());
        let out = e.mem.alloc(0x10);
        assert_eq!(e.call(0x00c6_5ef0, &args![scene.this, out]).u32(), out);
        assert_eq!(
            calls_to(&e, VECTOR_TO_POINT),
            vec![vec![out, scene.hk + 0x1b0]]
        );
        // No Havok object: the zero vector.
        e.mem.set_u32(scene.body + 8, 0);
        assert_eq!(e.call(0x00c6_5ef0, &args![scene.this, out]).u32(), out);
        assert_eq!(calls_to(&e, VECTOR_TO_POINT)[1], vec![out, ZERO_VECTOR]);
        // No body: the three default words.
        for (word, value) in [0x11u32, 0x22, 0x33].into_iter().enumerate() {
            e.mem.set_u32(DEFAULT_VELOCITY + 4 * word as u32, value);
        }
        e.mem.set_u32(scene.this + 0x10, 0);
        assert_eq!(e.call(0x00c6_5ef0, &args![scene.this, out]).u32(), out);
        assert_eq!(calls_to(&e, VECTOR_TO_POINT).len(), 2);
        let words: Vec<u32> = (0..3).map(|word| e.mem.u32(out + 4 * word)).collect();
        assert_eq!(words, vec![0x11, 0x22, 0x33]);
    }

    #[test]
    fn bhk_collision_object_set_motion_type_changes_the_type_under_the_lock() {
        let mut e = logged_engine();
        let scene = scene(&mut e);
        e.register(DYNAMIC_CAST, |e, _| e.mem.u32(CAST_RESULT).into_ret());
        e.register(MOTION_TYPE_TEST, |e, a| {
            u32::from(matches!(e.mem.u32(a[0]), 4 | 5)).into_ret()
        });
        e.register(BODY_WORD, |e, _| e.mem.u32(BODY_WORD_RESULT).into_ret());
        e.register(NO_EFFECT, |_, _| Ret::default());
        e.register(ACTIVATE, |_, _| Ret::default());
        e.mem.set_u32(SET_TYPE_SUCCEEDS, 1);
        e.mem.set_u16(scene.this + 0xc, 0xfff7);
        let call = |e: &mut Engine, motion_type: u32, object: u32, update: u32| {
            e.call(0x00c6_5f40, &args![scene.this, motion_type, object, update]);
        };
        // The type is already the wanted one: nothing.
        call(&mut e, 2, 0, 0);
        assert!(calls_to(&e, LOCK_ACQUIRE).is_empty());
        // Type 1 is refused when the current type passes the 00c8cce0 test (here 4 or 5).
        e.mem.set_u8(scene.hk + 0xe8, 4);
        call(&mut e, 1, 0, 0);
        assert!(calls_to(&e, LOCK_ACQUIRE).is_empty());
        // From 4 to 2: set under the lock, bit 3 cleared (2 does not pass the test).
        e.mem.set_u16(scene.this + 0xc, 0xffff);
        call(&mut e, 2, 0, 0);
        assert_eq!(calls_to(&e, LOCK_ACQUIRE), vec![vec![LOCK, 0]]);
        assert_eq!(calls_to(&e, SET_TYPE_SLOT), vec![vec![scene.body, 2]]);
        assert_eq!(e.mem.u32(LOCK_COUNT_AT_SET_TYPE), 1);
        assert!(lock_is_free(&e));
        assert_eq!(e.mem.u8(scene.hk + 0xe8), 2);
        assert_eq!(e.mem.u16(scene.this + 0xc), 0xfff7);
        // Type 1 from 2 goes through; type 4 sets the bit.
        call(&mut e, 1, 0, 0);
        assert_eq!(e.mem.u16(scene.this + 0xc), 0xfff7);
        call(&mut e, 4, 0, 0);
        assert_eq!(e.mem.u8(scene.hk + 0xe8), 4);
        assert_eq!(e.mem.u16(scene.this + 0xc), 0xffff);
        assert!(calls_to(&e, NO_EFFECT).is_empty());
        // A refusal by the body leaves the flags and the type as they were.
        e.mem.set_u32(SET_TYPE_SUCCEEDS, 0);
        e.mem.set_u16(scene.this + 0xc, 0xfff7);
        call(&mut e, 3, 0, 1);
        assert_eq!(e.mem.u16(scene.this + 0xc), 0xfff7);
        assert_eq!(e.mem.u8(scene.hk + 0xe8), 4);
        assert!(lock_is_free(&e));
        assert!(calls_to(&e, BODY_WORD).is_empty());
        // With the update flag: the body word tested; when it passes, the Havok object is
        // activated between the two no-effect calls.
        e.mem.set_u32(SET_TYPE_SUCCEEDS, 1);
        e.mem.set_u32(BODY_WORD_RESULT, 3);
        call(&mut e, 2, 0, 1);
        assert_eq!(calls_to(&e, BODY_WORD), vec![vec![scene.body]]);
        assert!(calls_to(&e, ACTIVATE).is_empty());
        clear_log(&mut e);
        e.mem.set_u32(BODY_WORD_RESULT, 4);
        call(&mut e, 3, 0, 1);
        assert_eq!(calls_to(&e, ACTIVATE), vec![vec![scene.hk]]);
        assert_eq!(calls_to(&e, NO_EFFECT), vec![vec![scene.body]; 2]);
        let order = vec![NO_EFFECT, ACTIVATE, NO_EFFECT];
        assert_eq!(order_of(&e, &order), order);
        // A cast object replaces the object's own body; a type 5 object is the Havok-less case.
        clear_log(&mut e);
        let other = e.mem.alloc(0x20);
        e.mem.set_u32(other, BODY_VTABLE);
        e.mem.set_u32(CAST_RESULT, other);
        e.mem.set_u16(scene.this + 0xc, 0);
        call(&mut e, 2, 0x4000, 0);
        assert_eq!(calls_to(&e, DYNAMIC_CAST), vec![vec![0x4000, BODY_RTTI]]);
        assert_eq!(calls_to(&e, SET_TYPE_SLOT), vec![vec![other, 2]]);
        assert_eq!(e.mem.u16(scene.this + 0xc), 8);
        // No cast result: the object's own body; neither: nothing.
        clear_log(&mut e);
        e.mem.set_u32(CAST_RESULT, 0);
        call(&mut e, 2, 0x4000, 0);
        assert_eq!(calls_to(&e, SET_TYPE_SLOT).len(), 1);
        e.mem.set_u32(scene.this + 0x10, 0);
        call(&mut e, 3, 0x4000, 0);
        call(&mut e, 3, 0, 0);
        assert_eq!(calls_to(&e, SET_TYPE_SLOT).len(), 1);
    }

    #[test]
    fn bhk_collision_object_add_viewer_strings_adds_the_class_name_and_the_flag() {
        let mut e = logged_engine();
        e.register(BASE_ADD_VIEWER_STRINGS, |e, _| {
            let thread = e.tls();
            let scope = e.mem.u32(thread + SCOPE_WORD);
            e.mem.set_u32(SCOPE_SEEN, scope);
            Ret::default()
        });
        e.register(GET_VIEWER_STRING, |_, _| 0x1111u32.into_ret());
        e.register(GET_VIEWER_STRING_OF_BOOL, |_, a| (0x2000 + a[1]).into_ret());
        e.register(STRING_ARRAY_SET_SIZE, |e, a| {
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        e.register(STRING_ARRAY_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(base + a[1] * 4, value);
            e.mem.set_u16(a[0] + 0xa, (a[1] + 1) as u16);
            Ret::default()
        });
        e.mem.set_u32(COLLISION_OBJECT_RTTI, 0x4444);
        let thread = e.tls();
        e.mem.set_u32(thread + SCOPE_WORD, 0x77);
        let this = e.mem.alloc(0x14);
        // Bit 5 set and bit 6 clear: 1. Bit 6 set: 0. Bit 5 clear: 0.
        for (flags, expected) in [
            (0x20u16, 0x2001u32),
            (0x60, 0x2000),
            (0x40, 0x2000),
            (0, 0x2000),
        ] {
            clear_log(&mut e);
            e.mem.set_u16(this + 0xc, flags);
            let array = e.mem.alloc(0x10);
            let base = e.mem.alloc(0x40);
            e.mem.set_u32(array + 4, base);
            e.mem.set_u16(array + 0xe, 4);
            e.call(0x00c6_6060, &args![this, array]);
            assert_eq!(
                calls_to(&e, BASE_ADD_VIEWER_STRINGS),
                vec![vec![this, array]]
            );
            assert_eq!(e.mem.u32(SCOPE_SEEN), 0x10);
            assert_eq!(calls_to(&e, GET_VIEWER_STRING), vec![vec![0x4444]]);
            assert_eq!(
                calls_to(&e, GET_VIEWER_STRING_OF_BOOL),
                vec![vec![VIEWER_LABEL, expected - 0x2000]]
            );
            assert_eq!(calls_to(&e, STRING_ARRAY_SET_SIZE), vec![vec![array, 4]]);
            assert_eq!(e.mem.u32(base), 0x1111);
            assert_eq!(e.mem.u32(base + 4), expected);
            assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
            let order = vec![
                BASE_ADD_VIEWER_STRINGS,
                GET_VIEWER_STRING,
                STRING_ARRAY_SET_AT,
                GET_VIEWER_STRING_OF_BOOL,
                STRING_ARRAY_SET_AT,
            ];
            assert_eq!(order_of(&e, &order), order);
        }
    }

    #[test]
    fn thunk_00c65800_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00c6_5800;
        const TARGET: u32 = 0x00c6_c330;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    #[test]
    fn thunk_00c65810_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00c6_5810;
        const TARGET: u32 = 0x00c6_bb70;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    #[test]
    fn thunk_00c65820_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00c6_5820;
        const TARGET: u32 = 0x00c6_c380;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    #[test]
    fn thunk_00c87fa0_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00c8_7fa0;
        const TARGET: u32 = 0x00c6_b730;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }

    #[test]
    fn thunk_00c87fb0_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00c8_7fb0;
        const TARGET: u32 = 0x00c6_b760;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }
}
