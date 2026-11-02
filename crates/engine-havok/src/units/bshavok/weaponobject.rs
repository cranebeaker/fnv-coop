//! `BSHavok/weaponobject.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `WeaponObject` (Xbox PDB, 0x2c bytes, vtable `010c5c1c`) is a collision object of a weapon
//! node: the node it belongs to is the word at `+8` and the Havok body wrapper is reached
//! through the node's collision object (`+0x10`). Floats: x87 `FLD`/`FSTP` copies go through
//! [`x87_quiet`]; the x87 arithmetic is done in `f64` and rounded to `f32` where the exe stores
//! to memory; the SSE code runs in `f32`. `RSQRTSS` is the host CPU's estimate
//! (`rsqrt_estimate`). The functions that carry an `FS:[0]` exception frame or a `/GS` cookie
//! check (`00ec408c`) are marked `Translation incomplete`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpsphereshape::{inverse_length, store_vec, sum3, Vec4};

use super::bhkentity::{
    enter_scope, release_ni_object, HEAP_ALLOCATE, INTERLOCKED_DECREMENT, INTERLOCKED_INCREMENT,
    MEMORY_MANAGER, MEMORY_ROUTER,
};

/// The base constructor (`00c87c30`) and the constraint initializer (`bhkConstraint::Init`,
/// `00c89dd0`, `this` plus the data pointer) used by `fn_00c88380`, the two instance counters it
/// raises and the vtables it stores.
const OBJECT_CONSTRUCT: u32 = 0x00c8_7c30;
const CONSTRAINT_INIT: u32 = 0x00c8_9dd0;
const INSTANCE_COUNT_BASE: u32 = 0x0126_8188;
const INSTANCE_COUNT_CLASS: u32 = 0x0126_8318;
const BASE_VTABLE: u32 = 0x0108_febc;
const CONSTRAINT_VTABLE: u32 = 0x010c_5ce4;
/// The vtable of the constraint info (`010c5dc0`) and the one it ends with (`0108ffb4`).
const CINFO_VTABLE: u32 = 0x010c_5dc0;
const CINFO_ENDED_VTABLE: u32 = 0x0108_ffb4;
/// The vtable of a `WeaponObject`.
const WEAPON_VTABLE: u32 = 0x010c_5c1c;
/// The base object functions (`this` plus the stated words): `00c81600` the constructor,
/// `00c81580` the clone step `(source, new object)`, `00c81550` the destructor tail and
/// `00c817d0` `bhkBlendCollisionObject::UpdateWorldData` (Xbox PDB).
const BLEND_CONSTRUCT: u32 = 0x00c8_1600;
const BLEND_CLONE: u32 = 0x00c8_1580;
const BLEND_DESTRUCT: u32 = 0x00c8_1550;
const BLEND_UPDATE_WORLD_DATA: u32 = 0x00c8_17d0;
/// The comparisons `UpdateWorldData` makes (`00439090` on the translation, answering true when
/// the code must copy; `004d9ae0` on the rotation, answering true when it is equal; both
/// unnamed in the Xbox PDB), and the globals it copies from: the rotation (9 words) and the
/// translation (3 words).
const TRANSLATION_DIFFERS: u32 = 0x0043_9090;
const ROTATION_EQUAL: u32 = 0x004d_9ae0;
const ROTATION_SOURCE: u32 = 0x011a_9448;
const TRANSLATION_SOURCE: u32 = 0x011f_426c;
/// `NiTransform` helpers: `MakeIdentity` (`00a86bf0`, `this` = the transform) and the product
/// (`0062c250`, `this` = the left factor, `(result, right factor)`, answers the result).
const MAKE_IDENTITY: u32 = 0x00a8_6bf0;
const TRANSFORM_MULTIPLY: u32 = 0x0062_c250;
/// `bhkUtilFunctions::FindNextCollisionObject` (Xbox PDB, `00c802d0`, cdecl `(node)`),
/// `bhkUtilFunctions::GatherRigidBodies` (`00c80930`, cdecl `(node, &array)`) and the RTTI cast
/// (`00653290`, `this` = the object, `(rtti)`; `00653270` is the cdecl form `(rtti, object)`).
const FIND_NEXT_COLLISION_OBJECT: u32 = 0x00c8_02d0;
const GATHER_RIGID_BODIES: u32 = 0x00c8_0930;
const RTTI_CAST: u32 = 0x0065_3290;
const RTTI_CAST_STATIC: u32 = 0x0065_3270;
/// The RTTI of the update target (`012043f8`), of the blend collision object class
/// (`01267e64`) and of the controller class (`012681f0`).
const RTTI_UPDATE_TARGET: u32 = 0x0120_43f8;
const RTTI_BLEND_COLLISION: u32 = 0x0126_7e64;
const RTTI_CONTROLLER: u32 = 0x0126_81f0;
/// Conversions (cdecl): `00624ad0` `(out, havok transform)` builds a `NiTransform` (the scale
/// word `+0x30` is left to the caller), `00a6df40` is `NiQuaternion::FromRotation` (Xbox PDB,
/// `this` = the quaternion, `(matrix)`), `004a3e00` `(out, point)` turns a `NiPoint3` into a
/// Havok vector, `0056df80` is `NI2HK` `(out, NiTransform)` and `005611c0` scales a Havok
/// vector in place (`this` = the vector).
const HAVOK_TO_NI_TRANSFORM: u32 = 0x0062_4ad0;
const QUATERNION_FROM_ROTATION: u32 = 0x00a6_df40;
const POINT_TO_HAVOK: u32 = 0x004a_3e00;
const NI_TO_HAVOK_TRANSFORM: u32 = 0x0056_df80;
const SCALE_VECTOR: u32 = 0x0056_11c0;
/// The lock object (`01267b80`: owner word, count word `01267b84`) taken with `0040fbf0`
/// (`this` plus a word) around the position update.
const LOCK_OBJECT: u32 = 0x0126_7b80;
const LOCK_COUNT: u32 = 0x0126_7b84;
const LOCK_ACQUIRE: u32 = 0x0040_fbf0;
/// `bhkWorld::RemoveObjects` (`00c69ee0`, cdecl `(node, 1, 1)`), `bhkWorld::SetMotion`
/// (`00c6a350`, cdecl five words), `NiObjectNET::GetController` (`00a5c570`, `this` = the node,
/// `(rtti)`) and `RemoveController` (`00a5c480`, `this` = the node, `(controller)`).
const REMOVE_OBJECTS: u32 = 0x00c6_9ee0;
const SET_MOTION: u32 = 0x00c6_a350;
const GET_CONTROLLER: u32 = 0x00a5_c570;
const REMOVE_CONTROLLER: u32 = 0x00a5_c480;
/// The string interner (`00a5b690`, cdecl `(text)`), the empty string word (`0109b220`),
/// `BSUtilities::GetObjectByName` (`00c4b470`, cdecl `(root, &name, 0)`), the lookup by plain
/// text (`00c4b310`, cdecl `(root, text, 0)`), `_atexit` (`00ec658f`) and the texts and
/// handle word used by `UpdateConstraints` and `UpdateChain`.
const INTERN_STRING: u32 = 0x00a5_b690;
const EMPTY_STRING: u32 = 0x0109_b220;
const GET_OBJECT_BY_NAME: u32 = 0x00c4_b470;
const FIND_BY_TEXT: u32 = 0x00c4_b310;
const AT_EXIT: u32 = 0x00ec_658f;
const WEAPON_TEXT: u32 = 0x0101_3be8;
const BACKPACK_TEXT: u32 = 0x0101_f5cc;
const CHAIN_TEXT: u32 = 0x010c_5dd8;
const BACKPACK_HANDLE: u32 = 0x0126_8174;
const BACKPACK_GUARD: u32 = 0x0126_8178;
const BACKPACK_DESTRUCTOR: u32 = 0x00fd_e170;
/// The memory manager's `Deallocate` (`00aa4060`, `this` = the manager, a pointer).
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// The cleanup run on a list node before it is freed (`004ee840`, `this` = the node).
const NODE_CLEANUP: u32 = 0x004e_e840;
/// `NiAVObject::Update`-like call of `UpdateChain` (`00a59c60`, `this` = the object, the
/// address of a 9-byte block: a float 0.0 and five zero bytes).
const UPDATE_WITH_DATA: u32 = 0x00a5_9c60;
/// `bhkNiCollisionObject::SetWorldObject` (Xbox PDB, `00c6b980`, `this` plus the object) and
/// `NiAVObject::SetCollisionObject` (Xbox PDB, `0062bc90`, `this` = the node plus the object).
const SET_WORLD_OBJECT: u32 = 0x00c6_b980;
const SET_COLLISION_OBJECT: u32 = 0x0062_bc90;
/// The motion-mode predicate (`00c8cce0`, cdecl `(&mode)`), `bhkBlendController`'s
/// constructor (`00c9aeb0`, `this`) and the size of that object.
const MODE_PREDICATE: u32 = 0x00c8_cce0;
const BLEND_CONTROLLER_CONSTRUCT: u32 = 0x00c9_aeb0;
const BLEND_CONTROLLER_SIZE: u32 = 0x5c;
/// Floats `Constrain` stores into the hinge data. `bhkWorldObject::GetProperty` (`00c85ef0`,
/// cdecl `(out, object, key)`, answers `out`) and the key `UpdateChildRB3D` passes.
const LIMIT_MINIMUM: u32 = 0x0105_0de4;
const LIMIT_MAXIMUM: u32 = 0x0101_e2bc;
const GET_PROPERTY: u32 = 0x00c8_5ef0;
const PROPERTY_KEY: u32 = 0x0126_7b70;
/// `Constrain` pieces (see the function): the RTTI nodes of the two kinds of attachment it
/// removes, the constraint info steps and the body functions.
const RTTI_CONSTRAINT: u32 = 0x0126_818c;
const RTTI_WORLD_OBJECT: u32 = 0x0126_8158;
const CINFO_CREATE_DATA: u32 = 0x00ca_a3f0;
const CINFO_SET_BODIES: u32 = 0x009b_a320;
const CINFO_SET_DATA: u32 = 0x00c8_9ac0;
const BODY_VALUE: u32 = 0x0051_7630;
const RENORMALIZE: u32 = 0x00cb_2fc0;
const SET_IN_WORLD_SPACE: u32 = 0x00cb_8b60;
const ADD_ACTION_CONSTRAINT: u32 = 0x00c8_ef80;
const COLLISION_FILTER_OF: u32 = 0x0043_b4f0;
const SET_COLLISION_FILTER: u32 = 0x0062_6cb0;
const WORLD_OF: u32 = 0x0062_1480;
const BODY_IN_WORLD: u32 = 0x008c_ffa0;
const ADD_TO_ENTITY: u32 = 0x0056_d2c0;
const ADD_ACTION_CONSTRAINT_TO_WORLD: u32 = 0x00c8_dac0;
const CONSTRAINT_GET_HK_WORLD: u32 = 0x00c8_9a00;
const WORLD_OBJECT_GET_HK_WORLD: u32 = 0x00c8_7a40;

/// The inverse length of `(x, y, z, 0)` the exe's way (`RSQRTSS` estimate refined once, `0`
/// when the squared length is the float at 0x01011d78), and the vector scaled by it.
fn normalized_axis(e: &Engine, x: f32, y: f32, z: f32) -> Vec4 {
    let vector: Vec4 = [x, y, z, 0.0];
    let squares = vector.map(|lane| lane * lane);
    let sum = sum3(squares);
    let inverse = inverse_length(e, sum);
    vector.map(|lane| lane * inverse)
}

/// Copies `words` dwords bit for bit.
fn copy_words(e: &mut Engine, from: u32, to: u32, words: u32) {
    for word in 0..words {
        let value = e.mem.u32(from + 4 * word);
        e.mem.set_u32(to + 4 * word, value);
    }
}

// Translated from 00c88380 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c88380 - the FS:[0] exception frame is not modeled.
/// `fn_00c88380` (unnamed in the Xbox PDB): takes `this` and the address of a constraint info
/// (or 0). Runs the base constructor (`00c87c30`), clears the word at `+0xC`, raises the
/// instance counter at 0x01268188, stores the vtable 0x010c5ce4, runs `bhkConstraint::Init`
/// (`00c89dd0`) with the info's data pointer (the word at info `+4`; 0 for no info) and raises
/// the instance counter at 0x01268318 (next to the RTTI word 0x0126831c of
/// `bhkLimitedHingeConstraint`, to which this class belongs: unconfirmed). Answers `this`.
pub fn fn_00c88380(e: &mut Engine, this: u32, info: u32) -> u32 {
    e.call(OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this + 0xc, 0);
    e.mem.set_u32(this, BASE_VTABLE);
    let count = e.mem.u32(INSTANCE_COUNT_BASE);
    e.mem.set_u32(INSTANCE_COUNT_BASE, count.wrapping_add(1));
    e.mem.set_u32(this, CONSTRAINT_VTABLE);
    let data = if info != 0 { info + 4 } else { 0 };
    e.call(CONSTRAINT_INIT, &args![this, data]);
    let count = e.mem.u32(INSTANCE_COUNT_CLASS);
    e.mem.set_u32(INSTANCE_COUNT_CLASS, count.wrapping_add(1));
    this
}

// Translated from 00c88430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00c88430` (unnamed in the Xbox PDB): sets the words of a constraint info: `+4` to 0,
/// `+8` to 1, `+0xC` and `+0x10` to 0 and the vtable 0x010c5dc0. Answers `this`.
pub fn fn_00c88430(e: &mut Engine, this: u32) -> u32 {
    e.mem.set_u32(this + 4, 0);
    e.mem.set_u32(this + 8, 1);
    e.mem.set_u32(this + 0xc, 0);
    e.mem.set_u32(this + 0x10, 0);
    e.mem.set_u32(this, CINFO_VTABLE);
    this
}

// Translated from 00c88450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `WeaponObject::UpdateWorldData` (Xbox PDB): takes one word. With a node (the word at `+8`),
/// when `00439090` (on the node's translation at `+0x58` against the three words at
/// 0x011f426c) answers true or `004d9ae0` (on the rotation at `+0x34` against the nine words
/// at 0x011a9448) answers false, copies the nine rotation words and the three translation
/// words from those globals into the node. Then runs `bhkBlendCollisionObject::UpdateWorldData`
/// (`00c817d0`) with the word.
pub fn weapon_object_update_world_data(e: &mut Engine, this: u32, update: u32) {
    let node = e.mem.u32(this + 8);
    if node != 0 {
        let differs = e
            .call(TRANSLATION_DIFFERS, &args![node + 0x58, TRANSLATION_SOURCE])
            .bool();
        if differs
            || !e
                .call(ROTATION_EQUAL, &args![node + 0x34, ROTATION_SOURCE])
                .bool()
        {
            copy_words(e, ROTATION_SOURCE, node + 0x34, 9);
            copy_words(e, TRANSLATION_SOURCE, node + 0x58, 3);
        }
    }
    e.call(BLEND_UPDATE_WORLD_DATA, &args![this, update]);
}

// Translated from 00c884c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GetTransformRecursive` (Xbox PDB): cdecl `(node, stop, out)`. Nothing for no node. At the
/// `stop` node `out` becomes the identity (`NiTransform::MakeIdentity`, `00a86bf0`). Otherwise
/// the parent (the word at `+0x18`) is handled first, then the node's transform (`+0x34`) is
/// multiplied onto `out` (`0062c250`) and the 13-word result is copied to `out`.
pub fn get_transform_recursive(e: &mut Engine, node: u32, stop: u32, out: u32) {
    if node == 0 {
        return;
    }
    if node == stop {
        e.call(MAKE_IDENTITY, &args![out]);
        return;
    }
    let parent = e.mem.u32(node + 0x18);
    get_transform_recursive(e, parent, stop, out);
    e.with_stack(0x34, |e, result| {
        let product = e
            .call(TRANSFORM_MULTIPLY, &args![out, result, node + 0x34])
            .u32();
        copy_words(e, product, out, 13);
    });
}

// Translated from 00c88520 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c88520 - the /GS cookie check (00ec408c) is not modeled.
/// `WeaponObject::UpdateRigidBodyTransforms` (Xbox PDB): cdecl `(node, root)`. Finds the first
/// collision object of `node` (`FindNextCollisionObject`, `00c802d0`; its node is the word at
/// `+8`, its body the word at `+0x10`) and the body of `root`'s collision field (the word at
/// `+0x1C` cast to the RTTI 0x012043f8, body at `+0x10`). When both bodies, the node and
/// `root`'s parent (`+0x18`) exist: gets the source body's transform through its virtual slot
/// 0xF4, builds a `NiTransform` from it (`00624ad0`) whose scale is `root`'s float at
/// `+0x98`, multiplies the transform that leads from the node up to `root`
/// (`GetTransformRecursive`) onto it, turns the product's rotation into a quaternion
/// (`NiQuaternion::FromRotation`, `00a6df40`) and its translation into a Havok vector
/// (`004a3e00`), reorders the quaternion to (y, z, w, x) lanes (through the x87 stack), scales
/// it in place (`005611c0`) and, holding the lock at 0x01267b80
/// (`0040fbf0`), hands the translation and the quaternion to the destination body's virtual
/// slot 0xE8. The lock is let go by lowering the count at 0x01267b84, which clears the owner
/// word 0x01267b80 when it reaches 0.
pub fn weapon_object_update_rigid_body_transforms(e: &mut Engine, node: u32, root: u32) {
    let mut collision = 0;
    let mut target = 0;
    if node != 0 {
        collision = e.call(FIND_NEXT_COLLISION_OBJECT, &args![node]).u32();
        if collision != 0 {
            target = e.mem.u32(collision + 8);
        }
    }
    let body = if collision != 0 {
        e.mem.u32(collision + 0x10)
    } else {
        0
    };
    let field = if root != 0 { e.mem.u32(root + 0x1c) } else { 0 };
    let cast = if field != 0 {
        e.call(RTTI_CAST, &args![field, RTTI_UPDATE_TARGET]).u32()
    } else {
        0
    };
    let source_body = if cast != 0 { e.mem.u32(cast + 0x10) } else { 0 };
    if body == 0 || source_body == 0 || target == 0 || root == 0 || e.mem.u32(root + 0x18) == 0 {
        return;
    }
    e.with_stack(0x130, |e, scratch| {
        let frame = scratch.addr();
        let havok_transform = frame + 0xd4;
        let ni_transform = frame + 0x18;
        let chain = frame + 0x80;
        let product = frame + 0x4c;
        let quaternion = frame + 8;
        let position = frame + 0xc4;
        let rotation = frame + 0xb4;
        e.vcall(source_body, 0xf4, &args![havok_transform]);
        e.call(HAVOK_TO_NI_TRANSFORM, &args![ni_transform, havok_transform]);
        let scale = x87_quiet_bits(e.mem.u32(root + 0x98));
        e.mem.set_u32(frame + 0x48, scale);
        get_transform_recursive(e, target, root, chain);
        e.call(TRANSFORM_MULTIPLY, &args![ni_transform, product, chain]);
        e.call(QUATERNION_FROM_ROTATION, &args![quaternion, product]);
        e.call(POINT_TO_HAVOK, &args![position, product + 0x24]);
        for (lane, source) in [(0, 1), (1, 2), (2, 3), (3, 0)] {
            let value = x87_quiet_bits(e.mem.u32(quaternion + 4 * source));
            e.mem.set_u32(rotation + 4 * lane, value);
        }
        e.call(SCALE_VECTOR, &args![rotation]);
        e.call(LOCK_ACQUIRE, &args![LOCK_OBJECT, 0u32]);
        e.vcall(body, 0xe8, &args![position, rotation]);
        let count = e.mem.u32(LOCK_COUNT).wrapping_sub(1);
        e.mem.set_u32(LOCK_COUNT, count);
        if count == 0 {
            e.mem.set_u32(LOCK_OBJECT, 0);
        }
    });
}

// Translated from 00c886c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c886c0 - the FS:[0] exception frame (unwinding that restores the scope word) is not modeled.
/// `WeaponObject::Kill` (Xbox PDB, `this` in ECX): with the allocation-scope word set to
/// 0x10 and a node (the word at `+8`): `bhkWorld::RemoveObjects` (`00c69ee0`) on
/// `(node, 1, 1)`, then removes the node's controller of the RTTI 0x012681f0
/// (`GetController`, `00a5c570`; `RemoveController`, `00a5c480`) when it has one. The scope
/// word is put back.
pub fn weapon_object_kill(e: &mut Engine, this: u32) {
    let (word, saved) = enter_scope(e);
    let node = e.mem.u32(this + 8);
    if node != 0 {
        e.call(REMOVE_OBJECTS, &args![node, 1u32, 1u32]);
        let controller = e.call(GET_CONTROLLER, &args![node, RTTI_CONTROLLER]).u32();
        if controller != 0 {
            e.call(REMOVE_CONTROLLER, &args![node, controller]);
        }
    }
    e.mem.set_u32(word, saved);
}

// Translated from 00c88760 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c88760 - the FS:[0] exception frame (unwinding that restores the scope word) is not modeled.
/// `WeaponObject::CreateClone` (Xbox PDB): takes one word. With the allocation-scope word set
/// to 0x10, allocates 0x2c bytes (`00aa13e0`); when that gave a block, runs the constructor
/// `00c81600` on it and stores the vtable 0x010c5c1c. Runs `00c81580` on `(this, block or 0,
/// the word)` and answers the block (or 0). The scope word is put back.
pub fn weapon_object_create_clone(e: &mut Engine, this: u32, source: u32) -> u32 {
    let (word, saved) = enter_scope(e);
    let block = e.call(HEAP_ALLOCATE, &args![0x2cu32]).u32();
    if block != 0 {
        e.call(BLEND_CONSTRUCT, &args![block]);
        e.mem.set_u32(block, WEAPON_VTABLE);
    }
    e.call(BLEND_CLONE, &args![this, block, source]);
    e.mem.set_u32(word, saved);
    block
}

// Translated from 00c88810 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c88810 - the FS:[0] exception frame is not modeled.
/// `WeaponObject::UpdateChain` (Xbox PDB): cdecl, takes an object (its caller pushes one more
/// word, which the function does not read). Gets the object's owner through virtual slot 0xC;
/// with one, interns the text "Chain01" (`00a5b690`; the handle is released by lowering the
/// count 8 bytes before it unless it is the empty string at 0x0109b220), looks the name up
/// through the owner's virtual slot 0x9C, and when that finds an object: runs `00a59c60` on the
/// object with a 9-byte block of zeros (a float 0.0 and five zero bytes), then walks the
/// children of the found object's parent (the word at `+0x18`; the array pointer at `+0xA0`,
/// the 16-bit count at `+0xA6`): for each with a collision field (`+0x1C`) runs that field's
/// virtual slot 0xA8 and `bhkWorld::SetMotion` (`00c6a350`) on `(child, 1, 0, 1, 1)`.
pub fn weapon_object_update_chain(e: &mut Engine, this: u32) {
    let owner = e.vcall(this, 0xc, &args![]).u32();
    if owner == 0 {
        return;
    }
    let handle = e.call(INTERN_STRING, &args![CHAIN_TEXT]).u32();
    let (found, released) = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), handle);
        let found = e.vcall(owner, 0x9c, &args![slot]).u32();
        (found, e.mem.u32(slot.addr()))
    });
    let empty = e.mem.u32(EMPTY_STRING);
    if released != empty {
        e.call(INTERLOCKED_DECREMENT, &args![released.wrapping_sub(8)]);
    }
    if found == 0 {
        return;
    }
    e.with_stack(0x10, |e, block| {
        e.mem.set_f32(block.addr(), 0.0);
        for byte in 0..5 {
            e.mem.set_u8(block.addr() + 4 + byte, 0);
        }
        e.call(UPDATE_WITH_DATA, &args![this, block]);
    });
    let parent = e.mem.u32(found + 0x18);
    if parent == 0 {
        return;
    }
    let mut index = 0u32;
    while index < u32::from(e.mem.u16(parent + 0xa6)) {
        let children = e.mem.u32(parent + 0xa0);
        let child = e.mem.u32(children + 4 * index);
        if child != 0 {
            let field = e.mem.u32(child + 0x1c);
            if field != 0 {
                e.vcall(field, 0xa8, &args![]);
                e.call(SET_MOTION, &args![child, 1u32, 0u32, 1u32, 1u32]);
            }
        }
        index += 1;
    }
}

// Translated from 00c88930 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c88930 - the FS:[0] exception frame is not modeled.
/// `WeaponObject::~WeaponObject` (Xbox PDB, `this` in ECX): stores the vtable 0x010c5c1c, runs
/// `WeaponObject::Kill` and then the base destructor tail `00c81550`.
pub fn weapon_object_destructor(e: &mut Engine, this: u32) {
    e.mem.set_u32(this, WEAPON_VTABLE);
    weapon_object_kill(e, this);
    e.call(BLEND_DESTRUCT, &args![this]);
}

// Translated from 00c88990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00c88990` (unnamed in the Xbox PDB, `this` in ECX): erases the element held by a list
/// node (word 0 the counted object, word 1 the next node). With a next node the object of the
/// next node is taken over (the old one released: decrement of its count at `+4`, virtual slot
/// 4 when it reaches 0; the new one counted), the node's next pointer becomes the next node's
/// next, and the next node is cleared, cleaned up (`004ee840`) and given back to the memory
/// manager (`00aa4060`). Without one the held object is released and the word cleared.
pub fn fn_00c88990(e: &mut Engine, this: u32) {
    let next = e.mem.u32(this + 4);
    if next != 0 {
        let held = e.mem.u32(this);
        let after = e.mem.u32(next + 4);
        e.mem.set_u32(this + 4, after);
        if held != e.mem.u32(next) {
            if held != 0 {
                release_ni_object(e, held);
            }
            let taken = e.mem.u32(next);
            e.mem.set_u32(this, taken);
            if taken != 0 {
                e.call(INTERLOCKED_INCREMENT, &args![taken + 4]);
            }
        }
        e.mem.set_u32(next + 4, 0);
        e.call(NODE_CLEANUP, &args![next]);
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, next]);
        return;
    }
    let held = e.mem.u32(this);
    if held != 0 {
        release_ni_object(e, held);
        e.mem.set_u32(this, 0);
    }
}

/// Whether the RTTI chain the object reports through its virtual slot 8 contains `rtti`.
fn is_kind_of(e: &mut Engine, object: u32, rtti: u32) -> bool {
    let mut node = e.vcall(object, 8, &args![]).u32();
    while node != 0 {
        if node == rtti {
            return true;
        }
        node = e.mem.u32(node + 4);
    }
    false
}

/// The first part of `Constrain`: walks the list of attachments of the body (the node
/// embedded at body `+0x14`) and erases them (`fn_00c88990`). An attachment of the
/// constraint RTTI 0x0126818c whose Havok constraint's virtual slot 0x20 answers more than
/// 0x64, 0x12 or 0x13 is kept and the walk moves on; any other one is erased, after its
/// virtual slot 0xA0 ran when it has a Havok world (`00c89a00` for the constraint kind,
/// `00c87a40` for the kind of the RTTI 0x01268158, which is the only other one given that
/// call). Answers whether one was kept.
fn drop_attachments(e: &mut Engine, body: u32) -> bool {
    let mut kept = false;
    let mut node = body + 0x14;
    while node != 0 {
        let item = e.mem.u32(node);
        if item == 0 {
            break;
        }
        if is_kind_of(e, item, RTTI_CONSTRAINT) {
            let owner = e.mem.u32(item + 8);
            let holder = if owner != 0 {
                e.mem.u32(owner + 0xc)
            } else {
                let list = e.mem.u32(item + 0xc);
                if list != 0 {
                    e.mem.u32(list)
                } else {
                    0
                }
            };
            let first = e.vcall(holder, 0x20, &args![]).i32();
            let keep = first > 0x64
                || e.vcall(holder, 0x20, &args![]).i32() == 0x12
                || e.vcall(holder, 0x20, &args![]).i32() == 0x13;
            if keep {
                node = e.mem.u32(node + 4);
                kept = true;
                continue;
            }
            if e.call(CONSTRAINT_GET_HK_WORLD, &args![item]).u32() != 0 {
                e.vcall(item, 0xa0, &args![]);
            }
        } else if is_kind_of(e, item, RTTI_WORLD_OBJECT)
            && e.call(WORLD_OBJECT_GET_HK_WORLD, &args![item]).u32() != 0
        {
            e.vcall(item, 0xa0, &args![]);
        }
        fn_00c88990(e, node);
    }
    kept
}

// Translated from 00c88a40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c88a40 - the FS:[0] exception frame (unwinding that restores the scope word) and the /GS cookie check (00ec408c) are not modeled.
/// `WeaponObject::Constrain` (Xbox PDB): cdecl `(create, filter, node_a, node_b, mode)`;
/// `create` and `mode` are tested as bytes. With the allocation-scope word set to 0x10 and both
/// nodes: `UpdateRigidBodyTransforms` on the two, then the collision objects of both
/// (`FindNextCollisionObject`); when both exist, erases the attachments of the first body
/// (the word at collision object `+0x10`, see `drop_attachments`). When `create` is set, a
/// limited hinge constraint is built between the two bodies: the constraint info is
/// initialised (vtable 0x010c5dc0, `+8` = 1) and gets its data (`00caa3f0`); the first body's
/// virtual slot 0xE4 is called with 4 when `mode` is set, else with 1 or 4 chosen from
/// the predicate `00c8cce0` over the two bodies' values (`00517630`) and the first body's
/// controller (`00a5c570`, RTTI 0x012681f0, last entry of its array at `+0x3c`, count at
/// `+0x48`): 1 when its float `+4` is below 1.0. The info takes the bodies (`009ba320`), the
/// two bodies' 64-byte transforms (at body `+8`, `+0xF0`) are renormalized (`00cb2fc0`), the
/// data gets its three floats (0x01050de4 at `+0xC4`, 0x0101e2bc at `+0xC8` and 1.0 at
/// `+0xBC`), the first node's axis (floats at `+0x68`, `+0x74`, `+0x80` of the first collision
/// object's node) is normalized and `setInWorldSpace` (`00cb8b60`) places the hinge. A
/// 0x10-byte constraint object (`fn_00c88380`) is attached to the first body
/// (`00c8ef80`), whose collision filter is rebuilt from both bodies' filters (`0043b4f0`)
/// and `filter`, set with `00626cb0`. The world of the second body (`00621480` of its
/// virtual slot 0x94, or the field at `+0x24` of the second collision object cast to the RTTI
/// 0x01267e64) receives the first body (`0056d2c0`, or `00c8dac0` when the first body has no
/// world either). When an attachment was kept, `UpdateChain` runs on the first node. The
/// constraint info is finished (`SetData(0)`, vtable 0x0108ffb4). The scope word is put back.
pub fn weapon_object_constrain(
    e: &mut Engine,
    create: u32,
    filter: u32,
    node_a: u32,
    node_b: u32,
    mode: u32,
) {
    let (word, saved) = enter_scope(e);
    if node_a != 0 && node_b != 0 {
        weapon_object_update_rigid_body_transforms(e, node_a, node_b);
        let object_a = e.call(FIND_NEXT_COLLISION_OBJECT, &args![node_a]).u32();
        let object_b = e.call(FIND_NEXT_COLLISION_OBJECT, &args![node_b]).u32();
        if object_a != 0 && object_b != 0 {
            let body_a = e.mem.u32(object_a + 0x10);
            let body_b = e.mem.u32(object_b + 0x10);
            let kept = drop_attachments(e, body_a);
            if create as u8 != 0 {
                build_hinge(
                    e,
                    filter,
                    node_a,
                    mode,
                    (object_a, object_b),
                    (body_a, body_b),
                    kept,
                );
            }
        }
    }
    e.mem.set_u32(word, saved);
}

/// The second part of `Constrain`; see there.
fn build_hinge(
    e: &mut Engine,
    filter: u32,
    node_a: u32,
    mode: u32,
    objects: (u32, u32),
    bodies: (u32, u32),
    kept: bool,
) {
    let (object_a, object_b) = objects;
    let (body_a, body_b) = bodies;
    e.with_stack(0x150, |e, scratch| {
        let frame = scratch.addr();
        let info = frame + 0x40;
        let cell_a = frame + 0x2c;
        let cell_b = frame + 0x60;
        let transform_a = frame + 0x74;
        let transform_b = frame + 0xb4;
        let pivot = frame + 0xa4;
        let axis = frame + 0x64;
        let ni_out = frame + 0xf4;
        e.mem.set_u32(info + 4, 0);
        e.mem.set_u32(info + 8, 1);
        e.mem.set_u32(info + 0xc, 0);
        e.mem.set_u32(info + 0x10, 0);
        e.mem.set_u32(info, CINFO_VTABLE);
        e.call(CINFO_CREATE_DATA, &args![info]);
        if mode as u8 != 0 {
            e.vcall(body_a, 0xe4, &args![4u32]);
        } else {
            let value_a = e.call(BODY_VALUE, &args![body_a]).u32();
            e.mem.set_u32(cell_a, value_a);
            let value_b = e.call(BODY_VALUE, &args![body_b]).u32();
            e.mem.set_u32(cell_b, value_b);
            let second = e.call(MODE_PREDICATE, &args![cell_b]).bool();
            let first = e.call(MODE_PREDICATE, &args![cell_a]).bool();
            if first != second {
                let node = e.mem.u32(object_b + 8);
                let controller = e.call(GET_CONTROLLER, &args![node, RTTI_CONTROLLER]).u32();
                let entries = if controller != 0 {
                    e.mem.u32(controller + 0x48)
                } else {
                    0
                };
                let new_mode = if entries != 0 {
                    let array = e.mem.u32(controller + 0x3c);
                    let last = e
                        .mem
                        .f32(array.wrapping_add(entries.wrapping_mul(12)).wrapping_sub(8));
                    if 1.0f64 > f64::from(last) {
                        1u32
                    } else {
                        4
                    }
                } else {
                    let value = e.call(BODY_VALUE, &args![body_b]).u32();
                    e.mem.set_u32(cell_a, value);
                    if e.call(MODE_PREDICATE, &args![cell_a]).bool() {
                        1
                    } else {
                        4
                    }
                };
                e.vcall(body_a, 0xe4, &args![new_mode]);
            }
        }
        e.call(CINFO_SET_BODIES, &args![info, body_a, body_b]);
        let rigid_a = if body_a != 0 {
            e.mem.u32(body_a + 8)
        } else {
            0
        };
        copy_words(e, rigid_a + 0xf0, transform_a, 16);
        let rigid_b = if body_b != 0 {
            e.mem.u32(body_b + 8)
        } else {
            0
        };
        copy_words(e, rigid_b + 0xf0, transform_b, 16);
        e.call(NI_TO_HAVOK_TRANSFORM, &args![ni_out, node_a + 0x68]);
        e.call(RENORMALIZE, &args![transform_a]);
        e.call(RENORMALIZE, &args![transform_b]);
        let data = e.mem.u32(info + 4);
        let minimum = x87_quiet_bits(e.mem.u32(LIMIT_MINIMUM));
        e.mem.set_u32(data + 0xc4, minimum);
        let maximum = x87_quiet_bits(e.mem.u32(LIMIT_MAXIMUM));
        e.mem.set_u32(data + 0xc8, maximum);
        e.mem.set_f32(data + 0xbc, 1.0);
        let axis_node = e.mem.u32(object_a + 8);
        let components: Vec<f32> = [0x68, 0x74, 0x80]
            .into_iter()
            .map(|offset| x87_quiet(e.mem.f32(axis_node + offset)))
            .collect();
        let unit = normalized_axis(e, components[0], components[1], components[2]);
        store_vec(e, axis, unit);
        e.call(
            SET_IN_WORLD_SPACE,
            &args![data, transform_a, transform_b, pivot, axis],
        );
        let block = e.call(HEAP_ALLOCATE, &args![0x10u32]).u32();
        let constraint = if block != 0 {
            fn_00c88380(e, block, info)
        } else {
            0
        };
        e.call(ADD_ACTION_CONSTRAINT, &args![body_a, constraint]);
        let filter_a = frame + 0x28;
        let filter_b = frame + 0x34;
        e.call(COLLISION_FILTER_OF, &args![body_a, filter_a]);
        let second_filter = e.call(COLLISION_FILTER_OF, &args![body_b, filter_b]).u32();
        let combined = (e.mem.u32(filter_a) & 0xe080)
            | (filter & 0xffff_1400)
            | (e.mem.u32(second_filter) & 0x7f)
            | 0x1400;
        e.call(SET_COLLISION_FILTER, &args![body_a, combined]);
        let body_b_world = e.vcall(body_b, 0x94, &args![]).u32();
        let mut target = e.call(WORLD_OF, &args![body_b_world]).u32();
        if target == 0 {
            let cast = e
                .call(RTTI_CAST_STATIC, &args![RTTI_BLEND_COLLISION, object_b])
                .u32();
            if cast != 0 {
                target = e.mem.u32(cast + 0x24);
            }
        }
        if target != 0 && e.call(BODY_IN_WORLD, &args![body_b]).bool() {
            if e.vcall(body_a, 0x94, &args![]).u32() == 0 {
                e.call(ADD_TO_ENTITY, &args![target, body_a]);
            } else {
                e.call(ADD_ACTION_CONSTRAINT_TO_WORLD, &args![body_a, target]);
            }
        }
        if kept {
            weapon_object_update_chain(e, node_a);
        }
        e.mem.set_u32(info, CINFO_ENDED_VTABLE);
        e.call(CINFO_SET_DATA, &args![info, 0u32]);
    });
}

// Translated from 00c89050 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c89050 - the FS:[0] exception frame and the one-time guard of the "Backpack" name (whose cleanup is registered with _atexit) are not modeled.
/// `WeaponObject::UpdateConstraints` (Xbox PDB): `(create, filter, root, mode)`, four words
/// (the register `ECX` the caller sets is not read). Runs `Constrain` twice on `(create,
/// filter, found, ancestor, mode)`: first with the object the plain-text lookup (`00c4b310`)
/// finds for "Weapon" under `root`, then, the name "Backpack" being interned once into the
/// word 0x01268174 (guard bit 0 of 0x01268178; `_atexit` registers `00fde170`), with the
/// object `GetObjectByName` (`00c4b470`) finds for it. The second node is the first ancestor
/// (via the word at `+0x18`) of the found object whose collision field (`+0x1C`) is empty or
/// has an object (virtual slot 0x3C, called twice) whose word at `+0x10` is 0; the walk gives
/// up with 0 at `root`, at an ancestor that has such a field with another answer, and when
/// the found object has no parent. Each run needs both nodes.
pub fn weapon_object_update_constraints(
    e: &mut Engine,
    create: u32,
    filter: u32,
    root: u32,
    mode: u32,
) {
    let weapon = e.call(FIND_BY_TEXT, &args![root, WEAPON_TEXT, 0u32]).u32();
    constrain_found(e, (create, filter, root, mode), weapon);
    let guard = e.mem.u32(BACKPACK_GUARD);
    if guard & 1 == 0 {
        e.mem.set_u32(BACKPACK_GUARD, guard | 1);
        let handle = e.call(INTERN_STRING, &args![BACKPACK_TEXT]).u32();
        e.mem.set_u32(BACKPACK_HANDLE, handle);
        e.call(AT_EXIT, &args![BACKPACK_DESTRUCTOR]);
    }
    let backpack = e
        .call(GET_OBJECT_BY_NAME, &args![root, BACKPACK_HANDLE, 0u32])
        .u32();
    constrain_found(e, (create, filter, root, mode), backpack);
}

/// One run of `UpdateConstraints`: finds the ancestor for the found object and constrains.
fn constrain_found(e: &mut Engine, words: (u32, u32, u32, u32), found: u32) {
    let (create, filter, root, mode) = words;
    let mut ancestor = if found != 0 {
        e.mem.u32(found + 0x18)
    } else {
        0
    };
    while ancestor != 0 {
        let field = e.mem.u32(ancestor + 0x1c);
        if field != 0 {
            if e.vcall(field, 0x3c, &args![]).u32() == 0 {
                break;
            }
            let inner = e.vcall(field, 0x3c, &args![]).u32();
            if e.mem.u32(inner + 0x10) != 0 {
                break;
            }
        }
        if ancestor == root {
            ancestor = 0;
            break;
        }
        ancestor = e.mem.u32(ancestor + 0x18);
    }
    if ancestor == root {
        ancestor = 0;
    }
    if found != 0 && ancestor != 0 {
        weapon_object_constrain(e, create, filter, found, ancestor, mode);
    }
}

// Translated from 00c891c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c891c0 - the FS:[0] exception frame is not modeled.
/// `WeaponObject::UpdateChildRB3D` (Xbox PDB, `this` in ECX): with a node (the word at `+8`),
/// takes the body of its first collision object (`FindNextCollisionObject`, word `+0x10`),
/// gathers the node's rigid bodies into a local array (`GatherRigidBodies`, `00c80930`, cdecl
/// `(node, &array)`; the array starts as data 0, size 0, capacity word 0x80000000) and, while
/// that body exists, for each gathered object that is not the first one's own (word `+8`) and
/// whose word at `+8` is 0 sets bit 0 of the word at `+0x30` of the object the property
/// 0x01267b70 of it points to (`bhkWorldObject::GetProperty`, `00c85ef0`). The array's block
/// is given back through the memory router's allocator (`00c85750`, `+0x10`, virtual slot 0x10
/// with the block and `capacity * 4` bytes) when its capacity word is not negative.
pub fn weapon_object_update_child_rb3d(e: &mut Engine, this: u32) {
    let node = e.mem.u32(this + 8);
    if node == 0 {
        return;
    }
    let collision = e.call(FIND_NEXT_COLLISION_OBJECT, &args![node]).u32();
    let body = if collision != 0 {
        e.mem.u32(collision + 0x10)
    } else {
        0
    };
    e.with_stack(0x20, |e, block| {
        let array = block.addr() + 8;
        let output = block.addr();
        e.mem.set_u32(array, 0);
        e.mem.set_u32(array + 4, 0);
        e.mem.set_u32(array + 8, 0x8000_0000);
        e.call(GATHER_RIGID_BODIES, &args![node, array]);
        let mut index = 0u32;
        if (e.mem.u32(array + 4) as i32) > 0 && body != 0 {
            let mut data = e.mem.u32(array);
            loop {
                if body == 0 {
                    break;
                }
                let own = e.mem.u32(body + 8);
                let object = e.mem.u32(data + 4 * index);
                if object != own && e.mem.u32(object + 8) == 0 {
                    let holder = e
                        .call(GET_PROPERTY, &args![output, object, PROPERTY_KEY])
                        .u32();
                    let target = e.mem.u32(holder);
                    let flags_owner = e.mem.u32(target + 8);
                    let flags = e.mem.u32(flags_owner + 0x30);
                    e.mem.set_u32(flags_owner + 0x30, flags | 1);
                    data = e.mem.u32(array);
                }
                index += 1;
                if (index as i32) >= (e.mem.u32(array + 4) as i32) {
                    break;
                }
            }
        }
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        e.mem.set_u32(array + 4, 0);
        let capacity = e.mem.u32(array + 8);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(array);
            e.vcall(allocator, 0x10, &args![data, (capacity & 0x3fff_ffff) * 4]);
        }
    });
}

// Translated from 00c892c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c892c0 - the FS:[0] exception frame (unwinding that restores the scope word and releases the references) is not modeled.
/// `WeaponObject::Init` (Xbox PDB): five words `(first, root, third, collision, last)`. With
/// the allocation-scope word set to 0x10, does nothing unless `root`, `third` and `collision`
/// are set. Otherwise sets bits 0x41 of the 16-bit word at `+0xC`; takes the fields at `+0x18`,
/// `+0x1C`, `+0x20`, `+0x24` and `+0x28` from `collision` cast to the RTTI 0x01267e64 (the
/// floats through the x87 stack), or, without one, 1.0, 1.0 and 4 for the first three.
/// Takes a counted reference to the object at collision `+0x10`, clears the world object
/// of `collision` (`00c6b980`), attaches this object to the node at collision `+8`
/// (`0062bc90`), sets the world object here, runs `UpdateConstraints` on `(1, first, root,
/// last)`, makes the fields `+0x20`, `+0x18` and `+0x1C` 1, 0.0 and 0.0 when the predicate
/// `00c8cce0` accepts the motion mode (byte `+0xE8` of the body at `+8` of the object at
/// `+0x10`, or 5) and, giving the node a `bhkBlendController` (0x5c bytes, `00c9aeb0`) unless
/// it has the controller of the RTTI 0x012681f0, sets bit 8 in the controller's 16-bit word
/// at `+8` and runs its virtual slot 0x98 on the node. The reference is released (virtual slot
/// 4 when it was the last) and the scope word put back.
pub fn weapon_object_init(
    e: &mut Engine,
    this: u32,
    first: u32,
    root: u32,
    third: u32,
    collision: u32,
    last: u32,
) {
    let (word, saved) = enter_scope(e);
    if root == 0 || third == 0 || collision == 0 {
        e.mem.set_u32(word, saved);
        return;
    }
    let flags = e.mem.u16(this + 0xc);
    e.mem.set_u16(this + 0xc, flags | 0x41);
    let cast = e
        .call(RTTI_CAST, &args![collision, RTTI_BLEND_COLLISION])
        .u32();
    if cast != 0 {
        for offset in [0x18, 0x1c] {
            let value = x87_quiet_bits(e.mem.u32(cast + offset));
            e.mem.set_u32(this + offset, value);
        }
        for offset in [0x24, 0x28, 0x20] {
            let value = e.mem.u32(cast + offset);
            e.mem.set_u32(this + offset, value);
        }
    } else {
        e.mem.set_u32(this + 0x20, 4);
        e.mem.set_f32(this + 0x18, 1.0);
        e.mem.set_f32(this + 0x1c, 1.0);
    }
    let reference = e.mem.u32(collision + 0x10);
    if reference != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![reference + 4]);
    }
    e.call(SET_WORLD_OBJECT, &args![collision, 0u32]);
    let node = e.mem.u32(collision + 8);
    e.call(SET_COLLISION_OBJECT, &args![node, this]);
    e.call(SET_WORLD_OBJECT, &args![this, reference]);
    weapon_object_update_constraints(e, 1, first, root, last);
    let held = e.mem.u32(this + 0x10);
    if held != 0 {
        let body = e.mem.u32(held + 8);
        let mode_value = if body != 0 {
            u32::from(e.mem.u8(body + 0xe8))
        } else {
            5
        };
        let accepted = e.with_stack(4, |e, cell| {
            e.mem.set_u32(cell.addr(), mode_value);
            e.call(MODE_PREDICATE, &args![cell]).bool()
        });
        if accepted {
            e.mem.set_u32(this + 0x20, 1);
            e.mem.set_f32(this + 0x18, 0.0);
            e.mem.set_f32(this + 0x1c, 0.0);
        }
    }
    let mut controller = e.call(GET_CONTROLLER, &args![node, RTTI_CONTROLLER]).u32();
    if controller == 0 {
        let block = e.call(HEAP_ALLOCATE, &args![BLEND_CONTROLLER_SIZE]).u32();
        controller = if block != 0 {
            e.call(BLEND_CONTROLLER_CONSTRUCT, &args![block]).u32()
        } else {
            0
        };
    }
    let controller_flags = e.mem.u16(controller + 8);
    e.mem.set_u16(controller + 8, controller_flags | 8);
    e.vcall(controller, 0x98, &args![node]);
    if reference != 0 {
        release_ni_object(e, reference);
    }
    e.mem.set_u32(word, saved);
}

// Translated from 00c894d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c894d0 - the FS:[0] exception frame is not modeled.
/// `WeaponObject::WeaponObject` (Xbox PDB): `this` plus five words. Runs the base constructor
/// (`00c81600`), stores the vtable 0x010c5c1c and runs `WeaponObject::Init` on the five words.
/// Answers `this`.
pub fn weapon_object_weapon_object(
    e: &mut Engine,
    this: u32,
    first: u32,
    root: u32,
    third: u32,
    collision: u32,
    last: u32,
) -> u32 {
    e.call(BLEND_CONSTRUCT, &args![this]);
    e.mem.set_u32(this, WEAPON_VTABLE);
    weapon_object_init(e, this, first, root, third, collision, last);
    this
}

// Translated from 00c89540 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c89540 - the FS:[0] exception frame is not modeled.
/// `WeaponObject::CreateWeaponObject` (Xbox PDB): cdecl, four words `(first, root, node,
/// last)`. With the allocation-scope word set to 0x10 and a `node`: finds its first collision
/// object (`FindNextCollisionObject`); with one, allocates 0x2c bytes (`00aa13e0`) and, when
/// that gave a block, constructs a `WeaponObject` on it from `(first, root, node, the
/// collision object, last)`. Answers the object, or 0. The scope word is put back.
pub fn weapon_object_create_weapon_object(
    e: &mut Engine,
    first: u32,
    root: u32,
    node: u32,
    last: u32,
) -> u32 {
    let (word, saved) = enter_scope(e);
    let mut result = 0;
    if node != 0 {
        let collision = e.call(FIND_NEXT_COLLISION_OBJECT, &args![node]).u32();
        if collision != 0 {
            let block = e.call(HEAP_ALLOCATE, &args![0x2cu32]).u32();
            if block != 0 {
                result = weapon_object_weapon_object(e, block, first, root, node, collision, last);
            }
        }
    }
    e.mem.set_u32(word, saved);
    result
}

// Translated from 00c88410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkLimitedHingeConstraint::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00caa140`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x10 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn bhk_limited_hinge_constraint_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00ca_a140;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x10u32]);
    }
    this
}

// Translated from 00c88a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `WeaponObject::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00c88930`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x2c bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn weapon_object_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00c8_8930;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x2cu32]);
    }
    this
}

// Translated from 00c88370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `WeaponObject::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_817c.
pub fn weapon_object_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_817c
}

// Translated from 00c88400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkLimitedHingeConstraint::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_831c.
pub fn bhk_limited_hinge_constraint_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_831c
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c88370, weapon_object_get_r_t_t_i() -> u32),
        entry!(0x00c88400, bhk_limited_hinge_constraint_get_r_t_t_i() -> u32),
        entry!(0x00c88410, bhk_limited_hinge_constraint_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00c88a20, weapon_object_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00c88380, fn_00c88380(u32, u32) -> u32),
        entry!(0x00c88430, fn_00c88430(u32) -> u32),
        entry!(0x00c88450, weapon_object_update_world_data(u32, u32)),
        entry!(0x00c884c0, get_transform_recursive(u32, u32, u32)),
        entry!(
            0x00c88520,
            weapon_object_update_rigid_body_transforms(u32, u32)
        ),
        entry!(0x00c886c0, weapon_object_kill(u32)),
        entry!(0x00c88760, weapon_object_create_clone(u32, u32) -> u32),
        entry!(0x00c88810, weapon_object_update_chain(u32)),
        entry!(0x00c88930, weapon_object_destructor(u32)),
        entry!(0x00c88990, fn_00c88990(u32)),
        entry!(0x00c88a40, weapon_object_constrain(u32, u32, u32, u32, u32)),
        entry!(
            0x00c89050,
            weapon_object_update_constraints(u32, u32, u32, u32)
        ),
        entry!(0x00c891c0, weapon_object_update_child_rb3d(u32)),
        entry!(0x00c892c0, weapon_object_init(u32, u32, u32, u32, u32, u32)),
        entry!(0x00c894d0, weapon_object_weapon_object(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00c89540, weapon_object_create_weapon_object(u32, u32, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bhk_limited_hinge_constraint_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00ca_a140, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00c88410, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00c88410, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x10]]);
        assert_eq!(calls_to(&e, 0x00ca_a140), vec![vec![this]; 2]);
    }

    #[test]
    fn weapon_object_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00c8_8930, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00c88a20, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00c88a20, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x2c]]);
        assert_eq!(calls_to(&e, 0x00c8_8930), vec![vec![this]; 2]);
    }

    #[test]
    fn weapon_object_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c8_8370, &args![]).u32(), 0x0126_817c);
    }

    #[test]
    fn bhk_limited_hinge_constraint_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c8_8400, &args![]).u32(), 0x0126_831c);
    }
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use crate::units::bshavok::bhkentity::testing::{
        calls_to, interlocked_doubles, logging_engine, ni_object,
    };
    use crate::units::bshavok::bhkentity::SCOPE_WORD;
    use crate::units::havok_sdk::hkpsphereshape::{HALF_F32, THREE_F32, ZERO_F32};

    /// The double of the virtual slot `s` of the test objects is at this base plus `s`.
    const SLOT_BASE: u32 = 0x0900_0000;
    /// Fixed memory the doubles write into, the hinge data, the memory router and the vtable
    /// of the controllers.
    const RECORD: u32 = 0x0900_1000;
    const HINGE_DATA: u32 = 0x0900_2000;
    const ROUTER_OBJECT: u32 = 0x0900_3000;
    const CONTROLLER_VTABLE: u32 = 0x0900_4000;
    /// The world the `Constrain` doubles answer (kept at `RECORD + 0x20`).
    const WORLD: u32 = 0xdead_0001;

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// Answers the word at `address`.
    fn answer_word(e: &Engine, address: u32) -> Ret {
        ret(e.mem.u32(address))
    }

    /// Puts the double of virtual slot `slot` into the vtable of `object`.
    fn with_slot(e: &mut Engine, object: u32, slot: u32) {
        let vtable = e.mem.u32(object);
        e.mem.set_u32(vtable + slot, SLOT_BASE + slot);
        e.register(SLOT_BASE + slot, nothing);
    }

    /// An object of `size` bytes whose vtable has doubles in the given slots.
    fn object_with_slots(e: &mut Engine, size: u32, slots: &[u32]) -> u32 {
        let object = e.mem.alloc(size);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(object, vtable);
        for &slot in slots {
            with_slot(e, object, slot);
        }
        object
    }

    fn scope_word(e: &mut Engine) -> u32 {
        e.tls() + SCOPE_WORD
    }

    #[test]
    fn the_constraint_object_constructor_counts_instances_and_initializes() {
        let mut e = logging_engine();
        e.register(OBJECT_CONSTRUCT, nothing);
        e.register(CONSTRAINT_INIT, nothing);
        e.map(INSTANCE_COUNT_BASE, 4);
        e.map(INSTANCE_COUNT_CLASS, 4);
        e.mem.set_u32(INSTANCE_COUNT_BASE, 5);
        e.mem.set_u32(INSTANCE_COUNT_CLASS, 7);
        let this = e.mem.alloc(0x10);
        let info = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0xc, 0x99);
        assert_eq!(e.call(0x00c8_8380, &args![this, info]).u32(), this);
        assert_eq!(e.mem.u32(this), CONSTRAINT_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(INSTANCE_COUNT_BASE), 6);
        assert_eq!(e.mem.u32(INSTANCE_COUNT_CLASS), 8);
        assert_eq!(calls_to(&e, OBJECT_CONSTRUCT), [[this]]);
        assert_eq!(calls_to(&e, CONSTRAINT_INIT), [[this, info + 4]]);
        e.call(0x00c8_8380, &args![this, 0u32]);
        assert_eq!(calls_to(&e, CONSTRAINT_INIT)[1], [this, 0]);
    }

    #[test]
    fn the_constraint_info_starts_with_one_in_its_second_word() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        for offset in [4, 0xc, 0x10] {
            e.mem.set_u32(this + offset, 0x55);
        }
        assert_eq!(e.call(0x00c8_8430, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), CINFO_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), 1);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0);
    }

    fn world_data_setup() -> (Engine, u32, u32) {
        let mut e = logging_engine();
        e.register(TRANSLATION_DIFFERS, |_, _| Ret::default());
        e.register(ROTATION_EQUAL, |_, _| ret(1));
        e.register(BLEND_UPDATE_WORLD_DATA, nothing);
        e.map(ROTATION_SOURCE, 36);
        e.map(TRANSLATION_SOURCE, 12);
        for word in 0..9 {
            e.mem.set_u32(ROTATION_SOURCE + 4 * word, 0x100 + word);
        }
        for word in 0..3 {
            e.mem.set_u32(TRANSLATION_SOURCE + 4 * word, 0x200 + word);
        }
        let node = e.mem.alloc(0x80);
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 8, node);
        (e, this, node)
    }

    #[test]
    fn world_data_is_left_alone_when_nothing_changed() {
        let (mut e, this, node) = world_data_setup();
        e.call(0x00c8_8450, &args![this, 0x55u32]);
        assert_eq!(
            calls_to(&e, TRANSLATION_DIFFERS),
            [[node + 0x58, TRANSLATION_SOURCE]]
        );
        assert_eq!(
            calls_to(&e, ROTATION_EQUAL),
            [[node + 0x34, ROTATION_SOURCE]]
        );
        assert_eq!(calls_to(&e, BLEND_UPDATE_WORLD_DATA), [[this, 0x55]]);
        assert_eq!(e.mem.u32(node + 0x34), 0);
        assert_eq!(e.mem.u32(node + 0x58), 0);
    }

    #[test]
    fn a_moved_translation_copies_the_rotation_and_translation() {
        let (mut e, this, node) = world_data_setup();
        e.register(TRANSLATION_DIFFERS, |_, _| ret(1));
        e.call(0x00c8_8450, &args![this, 0x55u32]);
        assert!(calls_to(&e, ROTATION_EQUAL).is_empty());
        for word in 0..9 {
            assert_eq!(e.mem.u32(node + 0x34 + 4 * word), 0x100 + word);
        }
        for word in 0..3 {
            assert_eq!(e.mem.u32(node + 0x58 + 4 * word), 0x200 + word);
        }
        assert_eq!(calls_to(&e, BLEND_UPDATE_WORLD_DATA), [[this, 0x55]]);
    }

    #[test]
    fn a_different_rotation_copies_too() {
        let (mut e, this, node) = world_data_setup();
        e.register(ROTATION_EQUAL, |_, _| Ret::default());
        e.call(0x00c8_8450, &args![this, 0u32]);
        assert_eq!(e.mem.u32(node + 0x34), 0x100);
        assert_eq!(e.mem.u32(node + 0x5c), 0x201);
    }

    #[test]
    fn without_a_node_only_the_base_update_runs() {
        let (mut e, this, _) = world_data_setup();
        e.mem.set_u32(this + 8, 0);
        e.call(0x00c8_8450, &args![this, 9u32]);
        assert!(calls_to(&e, TRANSLATION_DIFFERS).is_empty());
        assert_eq!(calls_to(&e, BLEND_UPDATE_WORLD_DATA), [[this, 9]]);
    }

    #[test]
    fn the_transform_chain_is_multiplied_from_the_stop_node_down() {
        let mut e = logging_engine();
        e.register(MAKE_IDENTITY, |e, a| {
            for word in 0..13 {
                e.mem.set_u32(a[0] + 4 * word, 1);
            }
            Ret::default()
        });
        e.register(TRANSFORM_MULTIPLY, |e, a| {
            for word in 0..13 {
                let left = e.mem.u32(a[0] + 4 * word);
                let right = e.mem.u32(a[2] + 4 * word);
                e.mem.set_u32(a[1] + 4 * word, left * 10 + right);
            }
            ret(a[1])
        });
        let root = e.mem.alloc(0x80);
        let first = e.mem.alloc(0x80);
        let second = e.mem.alloc(0x80);
        e.mem.set_u32(first + 0x18, root);
        e.mem.set_u32(second + 0x18, first);
        e.mem.set_u32(first + 0x34, 3);
        e.mem.set_u32(second + 0x34, 5);
        let out = e.mem.alloc(0x40);
        e.call(0x00c8_84c0, &args![0u32, root, out]);
        assert!(calls_to(&e, MAKE_IDENTITY).is_empty());
        e.call(0x00c8_84c0, &args![second, root, out]);
        assert_eq!(calls_to(&e, MAKE_IDENTITY), [[out]]);
        assert_eq!(calls_to(&e, TRANSFORM_MULTIPLY).len(), 2);
        // ((1 * 10 + 3) * 10 + 5) in the first word, (1 * 10) * 10 in the others.
        assert_eq!(e.mem.u32(out), 135);
        assert_eq!(e.mem.u32(out + 4), 100);
        assert_eq!(e.mem.u32(out + 0x30), 100);
        e.call(0x00c8_84c0, &args![root, root, out]);
        assert_eq!(e.mem.u32(out), 1);
    }

    /// The doubles of `UpdateRigidBodyTransforms`; answers the engine, the node, the root, the
    /// destination body and the source body.
    fn rigid_setup() -> (Engine, u32, u32, u32, u32) {
        let mut e = logging_engine();
        e.register(FIND_NEXT_COLLISION_OBJECT, |e, a| {
            answer_word(e, a[0] + 0x30)
        });
        e.register(RTTI_CAST, |_, a| ret(a[0]));
        e.register(HAVOK_TO_NI_TRANSFORM, nothing);
        e.register(TRANSFORM_MULTIPLY, |e, a| {
            let scale = e.mem.u32(a[0] + 0x30);
            e.mem.set_u32(RECORD + 0x10, scale);
            ret(a[1])
        });
        e.register(MAKE_IDENTITY, nothing);
        e.register(QUATERNION_FROM_ROTATION, |e, a| {
            for (lane, value) in [1.0f32, 2.0, 3.0, 4.0].into_iter().enumerate() {
                e.mem.set_f32(a[0] + 4 * lane as u32, value);
            }
            Ret::default()
        });
        e.register(POINT_TO_HAVOK, nothing);
        e.register(SCALE_VECTOR, nothing);
        e.register(LOCK_ACQUIRE, nothing);
        e.map(RECORD, 0x40);
        e.map(LOCK_OBJECT, 8);
        e.mem.set_u32(LOCK_OBJECT, 0x77);
        e.mem.set_u32(LOCK_COUNT, 2);
        let body = object_with_slots(&mut e, 0x100, &[0xe8]);
        e.register(SLOT_BASE + 0xe8, |e, a| {
            for lane in 0..4 {
                let value = e.mem.u32(a[2] + 4 * lane);
                e.mem.set_u32(RECORD + 4 * lane, value);
            }
            Ret::default()
        });
        let source_body = object_with_slots(&mut e, 0x100, &[0xf4]);
        let collision = e.mem.alloc(0x20);
        let target = e.mem.alloc(0x80);
        let node = e.mem.alloc(0x80);
        let root = e.mem.alloc(0xa0);
        let field = e.mem.alloc(0x20);
        let parent = e.mem.alloc(0x20);
        e.mem.set_u32(node + 0x30, collision);
        e.mem.set_u32(collision + 8, target);
        e.mem.set_u32(collision + 0x10, body);
        e.mem.set_u32(target + 0x18, root);
        e.mem.set_u32(root + 0x18, parent);
        e.mem.set_u32(root + 0x1c, field);
        e.mem.set_f32(root + 0x98, 2.5);
        e.mem.set_u32(field + 0x10, source_body);
        (e, node, root, body, source_body)
    }

    #[test]
    fn rigid_body_transforms_reach_the_body_as_position_and_rotation() {
        let (mut e, node, root, body, source_body) = rigid_setup();
        e.call(0x00c8_8520, &args![node, root]);
        assert_eq!(calls_to(&e, FIND_NEXT_COLLISION_OBJECT), [[node]]);
        let field = e.mem.u32(root + 0x1c);
        assert_eq!(calls_to(&e, RTTI_CAST), [[field, RTTI_UPDATE_TARGET]]);
        let source = calls_to(&e, SLOT_BASE + 0xf4);
        assert_eq!(source.len(), 1);
        assert_eq!(source[0][0], source_body);
        let multiplies = calls_to(&e, TRANSFORM_MULTIPLY);
        assert_eq!(multiplies.len(), 2);
        let outer = &multiplies[1];
        assert_eq!(outer[1] - outer[0], 0x34);
        assert_eq!(outer[2] - outer[0], 0x68);
        assert_eq!(multiplies[0][0], outer[2]);
        let target = e.mem.u32(e.mem.u32(node + 0x30) + 8);
        assert_eq!(multiplies[0][2], target + 0x34);
        assert_eq!(calls_to(&e, MAKE_IDENTITY), [[outer[2]]]);
        assert_eq!(e.mem.u32(RECORD + 0x10), 2.5f32.to_bits());
        let hand_over = calls_to(&e, SLOT_BASE + 0xe8);
        assert_eq!(hand_over.len(), 1);
        assert_eq!(hand_over[0][0], body);
        assert_eq!(hand_over[0][1], hand_over[0][2] + 0x10);
        let rotation: Vec<f32> = (0..4).map(|lane| e.mem.f32(RECORD + 4 * lane)).collect();
        assert_eq!(rotation, [2.0, 3.0, 4.0, 1.0]);
        assert_eq!(calls_to(&e, LOCK_ACQUIRE), [[LOCK_OBJECT, 0]]);
        assert_eq!(e.mem.u32(LOCK_COUNT), 1);
        assert_eq!(e.mem.u32(LOCK_OBJECT), 0x77);
        e.call(0x00c8_8520, &args![node, root]);
        assert_eq!(e.mem.u32(LOCK_COUNT), 0);
        assert_eq!(e.mem.u32(LOCK_OBJECT), 0);
    }

    #[test]
    fn rigid_body_transforms_need_both_bodies_and_a_parent() {
        let (mut e, node, root, _, _) = rigid_setup();
        e.mem.set_u32(root + 0x18, 0);
        e.call(0x00c8_8520, &args![node, root]);
        e.call(0x00c8_8520, &args![0u32, root]);
        e.call(0x00c8_8520, &args![node, 0u32]);
        assert!(calls_to(&e, LOCK_ACQUIRE).is_empty());
        assert!(calls_to(&e, SLOT_BASE + 0xe8).is_empty());
        assert_eq!(calls_to(&e, FIND_NEXT_COLLISION_OBJECT), [[node], [node]]);
    }

    #[test]
    fn kill_removes_the_world_objects_and_the_controller() {
        let mut e = logging_engine();
        e.register(REMOVE_OBJECTS, nothing);
        e.register(GET_CONTROLLER, |e, a| answer_word(e, a[0] + 0x40));
        e.register(REMOVE_CONTROLLER, nothing);
        let word = scope_word(&mut e);
        e.mem.set_u32(word, 7);
        let node = e.mem.alloc(0x80);
        let controller = e.mem.alloc(0x10);
        let this = e.mem.alloc(0x30);
        e.call(0x00c8_86c0, &args![this]);
        assert!(calls_to(&e, REMOVE_OBJECTS).is_empty());
        e.mem.set_u32(this + 8, node);
        e.call(0x00c8_86c0, &args![this]);
        assert_eq!(calls_to(&e, REMOVE_OBJECTS), [[node, 1, 1]]);
        assert_eq!(calls_to(&e, GET_CONTROLLER), [[node, RTTI_CONTROLLER]]);
        assert!(calls_to(&e, REMOVE_CONTROLLER).is_empty());
        e.mem.set_u32(node + 0x40, controller);
        e.call(0x00c8_86c0, &args![this]);
        assert_eq!(calls_to(&e, REMOVE_CONTROLLER), [[node, controller]]);
        assert_eq!(e.mem.u32(word), 7);
    }

    #[test]
    fn a_clone_is_a_new_weapon_object_when_the_allocation_succeeds() {
        let mut e = logging_engine();
        e.register(HEAP_ALLOCATE, |e, _| {
            let block = e.mem.alloc(0x2c);
            ret(block)
        });
        e.register(BLEND_CONSTRUCT, nothing);
        e.register(BLEND_CLONE, nothing);
        let word = scope_word(&mut e);
        e.mem.set_u32(word, 7);
        let this = e.mem.alloc(0x30);
        let block = e.call(0x00c8_8760, &args![this, 0x66u32]).u32();
        assert_ne!(block, 0);
        assert_eq!(e.mem.u32(block), WEAPON_VTABLE);
        assert_eq!(calls_to(&e, HEAP_ALLOCATE), [[0x2c]]);
        assert_eq!(calls_to(&e, BLEND_CONSTRUCT), [[block]]);
        assert_eq!(calls_to(&e, BLEND_CLONE), [[this, block, 0x66]]);
        assert_eq!(e.mem.u32(word), 7);
        e.register(HEAP_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00c8_8760, &args![this, 0x67u32]).u32(), 0);
        assert_eq!(calls_to(&e, BLEND_CONSTRUCT).len(), 1);
        assert_eq!(calls_to(&e, BLEND_CLONE)[1], [this, 0, 0x67]);
    }

    #[test]
    fn the_chain_wakes_the_children_of_the_found_objects_parent() {
        let mut e = logging_engine();
        interlocked_doubles(&mut e);
        e.register(INTERN_STRING, |e, _| answer_word(e, RECORD));
        e.register(UPDATE_WITH_DATA, nothing);
        e.register(SET_MOTION, nothing);
        e.map(RECORD, 0x40);
        e.map(EMPTY_STRING, 4);
        e.mem.set_u32(EMPTY_STRING, 0x1234);
        let text = e.mem.alloc(0x10);
        e.mem.set_u32(text, 1);
        e.mem.set_u32(RECORD, text + 8);
        let this = object_with_slots(&mut e, 0x100, &[0xc]);
        let owner = object_with_slots(&mut e, 0x100, &[0x9c]);
        e.register(SLOT_BASE + 0xc, |e, a| answer_word(e, a[0] + 0x20));
        e.register(SLOT_BASE + 0x9c, |e, a| answer_word(e, a[0] + 0x20));
        let found = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x20, owner);
        e.mem.set_u32(owner + 0x20, found);
        let parent = e.mem.alloc(0xb0);
        let children = e.mem.alloc(0x10);
        let first = e.mem.alloc(0x40);
        let second = e.mem.alloc(0x40);
        let field = object_with_slots(&mut e, 0x100, &[0xa8]);
        e.mem.set_u32(found + 0x18, parent);
        e.mem.set_u16(parent + 0xa6, 3);
        e.mem.set_u32(parent + 0xa0, children);
        e.mem.set_u32(children, first);
        e.mem.set_u32(children + 4, second);
        e.mem.set_u32(first + 0x1c, field);
        e.call(0x00c8_8810, &args![this, 0u32]);
        assert_eq!(calls_to(&e, INTERN_STRING), [[CHAIN_TEXT]]);
        assert_eq!(calls_to(&e, SLOT_BASE + 0x9c).len(), 1);
        assert_eq!(e.mem.u32(text), 0);
        let wake = calls_to(&e, UPDATE_WITH_DATA);
        assert_eq!(wake.len(), 1);
        assert_eq!(wake[0][0], this);
        assert_eq!(calls_to(&e, SLOT_BASE + 0xa8), [[field]]);
        assert_eq!(calls_to(&e, SET_MOTION), [[first, 1, 0, 1, 1]]);
        // The empty string is not released, and an unknown name stops the walk.
        e.mem.set_u32(RECORD, 0x1234);
        e.mem.set_u32(owner + 0x20, 0);
        e.call(0x00c8_8810, &args![this, 0u32]);
        assert_eq!(calls_to(&e, INTERN_STRING).len(), 2);
        assert_eq!(calls_to(&e, UPDATE_WITH_DATA).len(), 1);
        assert_eq!(e.mem.u32(text), 0);
        // Without an owner nothing else happens.
        e.mem.set_u32(this + 0x20, 0);
        e.call(0x00c8_8810, &args![this, 0u32]);
        assert_eq!(calls_to(&e, INTERN_STRING).len(), 2);
    }

    #[test]
    fn the_destructor_kills_the_object_and_runs_the_base_destructor() {
        let mut e = logging_engine();
        e.register(BLEND_DESTRUCT, nothing);
        let this = e.mem.alloc(0x30);
        e.call(0x00c8_8930, &args![this]);
        assert_eq!(e.mem.u32(this), WEAPON_VTABLE);
        assert_eq!(calls_to(&e, BLEND_DESTRUCT), [[this]]);
    }

    fn erase_setup() -> Engine {
        let mut e = logging_engine();
        interlocked_doubles(&mut e);
        e.register(NODE_CLEANUP, nothing);
        e.register(MEMORY_DEALLOCATE, nothing);
        e.register(SLOT_BASE + 4, nothing);
        e
    }

    #[test]
    fn erasing_a_node_takes_over_the_next_object() {
        let mut e = erase_setup();
        let held = ni_object(&mut e, 2, SLOT_BASE + 4);
        let taken = ni_object(&mut e, 1, SLOT_BASE + 4);
        let this = e.mem.alloc(0x10);
        let next = e.mem.alloc(0x10);
        let after = e.mem.alloc(0x10);
        e.mem.set_u32(this, held);
        e.mem.set_u32(this + 4, next);
        e.mem.set_u32(next, taken);
        e.mem.set_u32(next + 4, after);
        e.call(0x00c8_8990, &args![this]);
        assert_eq!(e.mem.u32(this), taken);
        assert_eq!(e.mem.u32(this + 4), after);
        assert_eq!(e.mem.u32(held + 4), 1);
        assert_eq!(e.mem.u32(taken + 4), 2);
        assert_eq!(e.mem.u32(next + 4), 0);
        assert_eq!(calls_to(&e, NODE_CLEANUP), [[next]]);
        assert_eq!(calls_to(&e, MEMORY_DEALLOCATE), [[MEMORY_MANAGER, next]]);
        assert!(calls_to(&e, SLOT_BASE + 4).is_empty());
    }

    #[test]
    fn erasing_over_the_same_object_keeps_the_counts() {
        let mut e = erase_setup();
        let held = ni_object(&mut e, 2, SLOT_BASE + 4);
        let this = e.mem.alloc(0x10);
        let next = e.mem.alloc(0x10);
        e.mem.set_u32(this, held);
        e.mem.set_u32(this + 4, next);
        e.mem.set_u32(next, held);
        e.call(0x00c8_8990, &args![this]);
        assert_eq!(e.mem.u32(this), held);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(held + 4), 2);
        assert_eq!(calls_to(&e, NODE_CLEANUP), [[next]]);
    }

    #[test]
    fn erasing_the_last_node_releases_its_object() {
        let mut e = erase_setup();
        let held = ni_object(&mut e, 1, SLOT_BASE + 4);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, held);
        e.call(0x00c8_8990, &args![this]);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(calls_to(&e, SLOT_BASE + 4), [[held]]);
        assert!(calls_to(&e, NODE_CLEANUP).is_empty());
        e.call(0x00c8_8990, &args![this]);
        assert_eq!(calls_to(&e, SLOT_BASE + 4).len(), 1);
    }

    /// The nodes, bodies and collision objects `Constrain` works on, with the doubles for all
    /// its callees.
    struct Hinge {
        node_a: u32,
        node_b: u32,
        body_a: u32,
        body_b: u32,
        link: u32,
        collision_b: u32,
    }

    fn hinge_setup() -> (Engine, Hinge) {
        let mut e = logging_engine();
        interlocked_doubles(&mut e);
        e.register(NODE_CLEANUP, nothing);
        e.register(MEMORY_DEALLOCATE, nothing);
        e.register(SLOT_BASE + 4, nothing);
        e.register(FIND_NEXT_COLLISION_OBJECT, |e, a| {
            answer_word(e, a[0] + 0x30)
        });
        e.register(CONSTRAINT_GET_HK_WORLD, |_, _| ret(1));
        e.register(WORLD_OBJECT_GET_HK_WORLD, |_, _| ret(1));
        e.register(CINFO_CREATE_DATA, |e, a| {
            e.mem.set_u32(a[0] + 4, HINGE_DATA);
            Ret::default()
        });
        e.register(BODY_VALUE, |e, a| answer_word(e, a[0] + 0x34));
        e.register(MODE_PREDICATE, |e, a| ret(e.mem.u32(a[0]) & 1));
        e.register(GET_CONTROLLER, |e, a| answer_word(e, a[0] + 0x40));
        e.register(CINFO_SET_BODIES, nothing);
        e.register(NI_TO_HAVOK_TRANSFORM, nothing);
        e.register(RENORMALIZE, nothing);
        e.register(SET_IN_WORLD_SPACE, |e, a| {
            for lane in 0..4 {
                let value = e.mem.u32(a[4] + 4 * lane);
                e.mem.set_u32(RECORD + 4 * lane, value);
            }
            let first = e.mem.u32(a[1]);
            let last = e.mem.u32(a[1] + 0x3c);
            let second = e.mem.u32(a[2]);
            e.mem.set_u32(RECORD + 0x10, first);
            e.mem.set_u32(RECORD + 0x14, last);
            e.mem.set_u32(RECORD + 0x18, second);
            Ret::default()
        });
        e.register(HEAP_ALLOCATE, |e, _| {
            let block = e.mem.alloc(0x10);
            ret(block)
        });
        e.register(OBJECT_CONSTRUCT, nothing);
        e.register(CONSTRAINT_INIT, nothing);
        e.register(ADD_ACTION_CONSTRAINT, nothing);
        e.register(COLLISION_FILTER_OF, |e, a| {
            let filter = e.mem.u32(a[0] + 0x30);
            e.mem.set_u32(a[1], filter);
            ret(a[1])
        });
        e.register(SET_COLLISION_FILTER, nothing);
        e.register(WORLD_OF, |e, a| {
            if a[0] != 0 {
                answer_word(e, RECORD + 0x20)
            } else {
                Ret::default()
            }
        });
        e.register(RTTI_CAST_STATIC, |e, a| answer_word(e, a[1] + 0x30));
        e.register(BODY_IN_WORLD, |_, _| ret(1));
        e.register(ADD_TO_ENTITY, nothing);
        e.register(ADD_ACTION_CONSTRAINT_TO_WORLD, nothing);
        e.register(CINFO_SET_DATA, nothing);
        e.map(RECORD, 0x40);
        e.map(HINGE_DATA, 0x100);
        e.map(INSTANCE_COUNT_BASE, 4);
        e.map(INSTANCE_COUNT_CLASS, 4);
        e.map(LIMIT_MINIMUM, 4);
        e.map(LIMIT_MAXIMUM, 4);
        e.map(RTTI_CONSTRAINT, 8);
        e.map(RTTI_WORLD_OBJECT, 8);
        for (address, value) in [(ZERO_F32, 0.0f32), (THREE_F32, 3.0), (HALF_F32, 0.5)] {
            e.map(address, 4);
            e.mem.set_f32(address, value);
        }
        e.mem.set_f32(LIMIT_MINIMUM, -1.25);
        e.mem.set_f32(LIMIT_MAXIMUM, 1.75);
        e.mem.set_u32(RECORD + 0x20, WORLD);
        let rigid_a = e.mem.alloc(0x200);
        let rigid_b = e.mem.alloc(0x200);
        for word in 0..16 {
            e.mem.set_u32(rigid_a + 0xf0 + 4 * word, 0xa00 + word);
            e.mem.set_u32(rigid_b + 0xf0 + 4 * word, 0xb00 + word);
        }
        let body_a = object_with_slots(&mut e, 0x100, &[0xe4, 0x94]);
        let body_b = object_with_slots(&mut e, 0x100, &[0x94]);
        e.register(SLOT_BASE + 0x94, |e, a| answer_word(e, a[0] + 0x38));
        e.mem.set_u32(body_a + 8, rigid_a);
        e.mem.set_u32(body_b + 8, rigid_b);
        e.mem.set_u32(body_a + 0x30, 0xe0a0);
        e.mem.set_u32(body_b + 0x30, 0x1ff);
        e.mem.set_u32(body_b + 0x38, 0x5151);
        let node_a = object_with_slots(&mut e, 0x100, &[0xc]);
        let node_b = e.mem.alloc(0x100);
        let axis_node = e.mem.alloc(0x100);
        let link = e.mem.alloc(0x100);
        let collision_a = e.mem.alloc(0x40);
        let collision_b = e.mem.alloc(0x40);
        e.mem.set_f32(axis_node + 0x68, 3.0);
        e.mem.set_f32(axis_node + 0x80, 4.0);
        e.mem.set_u32(collision_a + 8, axis_node);
        e.mem.set_u32(collision_a + 0x10, body_a);
        e.mem.set_u32(collision_b + 8, link);
        e.mem.set_u32(collision_b + 0x10, body_b);
        e.mem.set_u32(node_a + 0x30, collision_a);
        e.mem.set_u32(node_b + 0x30, collision_b);
        let hinge = Hinge {
            node_a,
            node_b,
            body_a,
            body_b,
            link,
            collision_b,
        };
        (e, hinge)
    }

    /// A counted attachment object stored in the list node `node`: its virtual slot 8
    /// answers `rtti`, the holder of its constraint answers `kind` from slot 0x20.
    fn attach(e: &mut Engine, node: u32, rtti: u32, kind: u32) -> u32 {
        let item = ni_object(e, 1, SLOT_BASE + 4);
        with_slot(e, item, 8);
        with_slot(e, item, 0xa0);
        e.register(SLOT_BASE + 4, nothing);
        e.register(SLOT_BASE + 8, |e, a| answer_word(e, a[0] + 0x30));
        e.mem.set_u32(item + 0x30, rtti);
        let owner = e.mem.alloc(0x20);
        let holder = object_with_slots(e, 0x100, &[0x20]);
        e.register(SLOT_BASE + 0x20, |e, a| answer_word(e, a[0] + 0x30));
        e.mem.set_u32(holder + 0x30, kind);
        e.mem.set_u32(owner + 0xc, holder);
        e.mem.set_u32(item + 8, owner);
        e.mem.set_u32(node, item);
        item
    }

    #[test]
    fn constrain_without_both_nodes_only_restores_the_scope() {
        let (mut e, hinge) = hinge_setup();
        let word = scope_word(&mut e);
        e.mem.set_u32(word, 7);
        e.call(0x00c8_8a40, &args![1u32, 0u32, hinge.node_a, 0u32, 1u32]);
        e.call(0x00c8_8a40, &args![1u32, 0u32, 0u32, hinge.node_b, 1u32]);
        assert!(calls_to(&e, FIND_NEXT_COLLISION_OBJECT).is_empty());
        assert_eq!(e.mem.u32(word), 7);
    }

    #[test]
    fn constrain_erases_the_attachments_of_the_first_body() {
        let (mut e, hinge) = hinge_setup();
        let list = hinge.body_a + 0x14;
        let second_node = e.mem.alloc(0x10);
        e.mem.set_u32(list + 4, second_node);
        let constraint = attach(&mut e, list, RTTI_CONSTRAINT, 5);
        let world_object = attach(&mut e, second_node, RTTI_WORLD_OBJECT, 0);
        e.call(
            0x00c8_8a40,
            &args![0u32, 0u32, hinge.node_a, hinge.node_b, 0u32],
        );
        assert_eq!(
            calls_to(&e, SLOT_BASE + 0xa0),
            [[constraint], [world_object]]
        );
        assert_eq!(calls_to(&e, CONSTRAINT_GET_HK_WORLD), [[constraint]]);
        assert_eq!(calls_to(&e, WORLD_OBJECT_GET_HK_WORLD), [[world_object]]);
        assert_eq!(calls_to(&e, SLOT_BASE + 4), [[constraint]]);
        assert_eq!(calls_to(&e, NODE_CLEANUP), [[second_node]]);
        assert_eq!(e.mem.u32(list), 0);
        assert!(calls_to(&e, CINFO_CREATE_DATA).is_empty());
        assert_eq!(
            calls_to(&e, FIND_NEXT_COLLISION_OBJECT),
            [[hinge.node_a], [hinge.node_a], [hinge.node_b]]
        );
    }

    #[test]
    fn constrain_keeps_the_attachments_of_the_special_kinds() {
        for kind in [0x65, 0x12, 0x13] {
            let (mut e, hinge) = hinge_setup();
            let list = hinge.body_a + 0x14;
            let item = attach(&mut e, list, RTTI_CONSTRAINT, kind);
            e.call(
                0x00c8_8a40,
                &args![0u32, 0u32, hinge.node_a, hinge.node_b, 0u32],
            );
            assert_eq!(e.mem.u32(list), item, "kind {kind:#x}");
            assert!(calls_to(&e, SLOT_BASE + 0xa0).is_empty());
            e.call(
                0x00c8_8a40,
                &args![1u32, 0u32, hinge.node_a, hinge.node_b, 1u32],
            );
            assert_eq!(calls_to(&e, SLOT_BASE + 0xc), [[hinge.node_a]]);
        }
    }

    #[test]
    fn constrain_builds_a_hinge_between_the_two_bodies() {
        let (mut e, hinge) = hinge_setup();
        let word = scope_word(&mut e);
        e.mem.set_u32(word, 7);
        e.mem.set_u32(INSTANCE_COUNT_BASE, 1);
        e.mem.set_u32(INSTANCE_COUNT_CLASS, 2);
        e.call(
            0x00c8_8a40,
            &args![1u32, 0x0003_1c00u32, hinge.node_a, hinge.node_b, 1u32],
        );
        let created = calls_to(&e, CINFO_CREATE_DATA);
        assert_eq!(created.len(), 1);
        let info = created[0][0];
        assert_eq!(calls_to(&e, SLOT_BASE + 0xe4), [[hinge.body_a, 4]]);
        assert_eq!(
            calls_to(&e, CINFO_SET_BODIES),
            [[info, hinge.body_a, hinge.body_b]]
        );
        let renormalized = calls_to(&e, RENORMALIZE);
        assert_eq!(renormalized.len(), 2);
        assert_eq!(renormalized[1][0] - renormalized[0][0], 0x40);
        assert_eq!(e.mem.f32(HINGE_DATA + 0xc4), -1.25);
        assert_eq!(e.mem.f32(HINGE_DATA + 0xc8), 1.75);
        assert_eq!(e.mem.f32(HINGE_DATA + 0xbc), 1.0);
        let axis: Vec<f32> = (0..4).map(|lane| e.mem.f32(RECORD + 4 * lane)).collect();
        assert!((axis[0] - 0.6).abs() < 1e-3, "{axis:?}");
        assert_eq!(axis[1], 0.0);
        assert!((axis[2] - 0.8).abs() < 1e-3, "{axis:?}");
        assert_eq!(axis[3], 0.0);
        assert_eq!(e.mem.u32(RECORD + 0x10), 0xa00);
        assert_eq!(e.mem.u32(RECORD + 0x14), 0xa0f);
        assert_eq!(e.mem.u32(RECORD + 0x18), 0xb00);
        assert_eq!(e.mem.u32(INSTANCE_COUNT_BASE), 2);
        assert_eq!(e.mem.u32(INSTANCE_COUNT_CLASS), 3);
        let init = calls_to(&e, CONSTRAINT_INIT);
        assert_eq!(init.len(), 1);
        assert_eq!(init[0][1], info + 4);
        let attached = calls_to(&e, ADD_ACTION_CONSTRAINT);
        assert_eq!(attached, [[hinge.body_a, init[0][0]]]);
        assert_eq!(e.mem.u32(init[0][0]), CONSTRAINT_VTABLE);
        // (0xe0a0 & 0xe080) | (0x31c00 & 0xffff1400) | (0x1ff & 0x7f) | 0x1400
        assert_eq!(
            calls_to(&e, SET_COLLISION_FILTER),
            [[hinge.body_a, 0x0003_f4ff]]
        );
        assert_eq!(calls_to(&e, ADD_TO_ENTITY), [[WORLD, hinge.body_a]]);
        assert!(calls_to(&e, ADD_ACTION_CONSTRAINT_TO_WORLD).is_empty());
        assert!(calls_to(&e, SLOT_BASE + 0xc).is_empty());
        assert_eq!(calls_to(&e, CINFO_SET_DATA), [[info, 0]]);
        assert_eq!(e.mem.u32(word), 7);
    }

    #[test]
    fn constrain_finds_the_world_through_the_collision_object_when_the_body_has_none() {
        let (mut e, hinge) = hinge_setup();
        let cast = e.mem.alloc(0x40);
        let other_world = 0xbeef_0002;
        e.mem.set_u32(cast + 0x24, other_world);
        e.mem.set_u32(hinge.collision_b + 0x30, cast);
        e.mem.set_u32(hinge.body_b + 0x38, 0);
        e.mem.set_u32(hinge.body_a + 0x38, 0x4444);
        e.call(
            0x00c8_8a40,
            &args![1u32, 0u32, hinge.node_a, hinge.node_b, 1u32],
        );
        assert_eq!(
            calls_to(&e, RTTI_CAST_STATIC),
            [[RTTI_BLEND_COLLISION, hinge.collision_b]]
        );
        assert_eq!(
            calls_to(&e, ADD_ACTION_CONSTRAINT_TO_WORLD),
            [[hinge.body_a, other_world]]
        );
        assert!(calls_to(&e, ADD_TO_ENTITY).is_empty());
        assert_eq!(calls_to(&e, BODY_IN_WORLD), [[hinge.body_b]]);
        // A first body without a world is added to the entity instead.
        e.mem.set_u32(hinge.body_a + 0x38, 0);
        e.call(
            0x00c8_8a40,
            &args![1u32, 0u32, hinge.node_a, hinge.node_b, 1u32],
        );
        assert_eq!(calls_to(&e, ADD_TO_ENTITY), [[other_world, hinge.body_a]]);
        // Without any world nothing is added.
        e.mem.set_u32(cast + 0x24, 0);
        e.call(
            0x00c8_8a40,
            &args![1u32, 0u32, hinge.node_a, hinge.node_b, 1u32],
        );
        assert_eq!(calls_to(&e, ADD_TO_ENTITY).len(), 1);
        assert_eq!(calls_to(&e, ADD_ACTION_CONSTRAINT_TO_WORLD).len(), 1);
    }

    #[test]
    fn constrain_picks_the_motion_mode_from_the_bodies_when_no_mode_is_forced() {
        let (mut e, hinge) = hinge_setup();
        let run = |e: &mut Engine, hinge: &Hinge| -> Vec<Vec<u32>> {
            e.call_log = Some(vec![]);
            e.call(
                0x00c8_8a40,
                &args![1u32, 0u32, hinge.node_a, hinge.node_b, 0u32],
            );
            calls_to(e, SLOT_BASE + 0xe4)
        };
        // Both bodies agree: the mode is left alone.
        e.mem.set_u32(hinge.body_a + 0x34, 1);
        e.mem.set_u32(hinge.body_b + 0x34, 1);
        assert!(run(&mut e, &hinge).is_empty());
        // They differ and the second has no controller: decided by the second body's value.
        e.mem.set_u32(hinge.body_b + 0x34, 0);
        assert_eq!(run(&mut e, &hinge), [[hinge.body_a, 4]]);
        e.mem.set_u32(hinge.body_a + 0x34, 0);
        e.mem.set_u32(hinge.body_b + 0x34, 1);
        assert_eq!(run(&mut e, &hinge), [[hinge.body_a, 1]]);
        // With a controller the float of its last entry decides: below 1.0 gives 1.
        e.mem.set_u32(hinge.body_a + 0x34, 1);
        e.mem.set_u32(hinge.body_b + 0x34, 0);
        let controller = e.mem.alloc(0x80);
        let entries = e.mem.alloc(0x40);
        e.mem.set_u32(controller + 0x48, 2);
        e.mem.set_u32(controller + 0x3c, entries);
        e.mem.set_u32(hinge.link + 0x40, controller);
        e.mem.set_f32(entries + 16, 0.5);
        assert_eq!(run(&mut e, &hinge), [[hinge.body_a, 1]]);
        e.mem.set_f32(entries + 16, 1.0);
        assert_eq!(run(&mut e, &hinge), [[hinge.body_a, 4]]);
        e.mem.set_f32(entries + 16, 1.5);
        assert_eq!(run(&mut e, &hinge), [[hinge.body_a, 4]]);
        // A controller without entries is as good as none.
        e.mem.set_u32(controller + 0x48, 0);
        assert_eq!(run(&mut e, &hinge), [[hinge.body_a, 4]]);
    }

    #[test]
    fn update_constraints_constrains_the_weapon_and_the_backpack() {
        let mut e = logging_engine();
        e.register(FIND_BY_TEXT, |e, a| answer_word(e, a[0] + 0x30));
        e.register(GET_OBJECT_BY_NAME, |e, a| answer_word(e, a[0] + 0x34));
        e.register(INTERN_STRING, |_, _| ret(0x4242));
        e.register(AT_EXIT, nothing);
        e.register(FIND_NEXT_COLLISION_OBJECT, |_, _| Ret::default());
        e.register(RTTI_CAST, |_, _| Ret::default());
        e.map(BACKPACK_GUARD, 4);
        e.map(BACKPACK_HANDLE, 4);
        let root = e.mem.alloc(0x80);
        let weapon = e.mem.alloc(0x80);
        let backpack = e.mem.alloc(0x80);
        let first_parent = e.mem.alloc(0x80);
        let second_parent = e.mem.alloc(0x80);
        let inner = e.mem.alloc(0x20);
        let first_field = object_with_slots(&mut e, 0x100, &[0x3c]);
        let second_field = object_with_slots(&mut e, 0x100, &[0x3c]);
        e.register(SLOT_BASE + 0x3c, |e, a| answer_word(e, a[0] + 0x38));
        e.mem.set_u32(root + 0x30, weapon);
        e.mem.set_u32(root + 0x34, backpack);
        e.mem.set_u32(weapon + 0x18, first_parent);
        e.mem.set_u32(first_parent + 0x18, root);
        e.mem.set_u32(first_parent + 0x1c, first_field);
        e.mem.set_u32(backpack + 0x18, second_parent);
        e.mem.set_u32(second_parent + 0x18, root);
        e.mem.set_u32(second_parent + 0x1c, second_field);
        e.mem.set_u32(second_field + 0x38, inner);
        e.mem.set_u32(inner + 0x10, 1);
        e.call(0x00c8_9050, &args![0u32, 0x11u32, root, 1u32]);
        assert_eq!(calls_to(&e, FIND_BY_TEXT), [[root, WEAPON_TEXT, 0]]);
        assert_eq!(calls_to(&e, INTERN_STRING), [[BACKPACK_TEXT]]);
        assert_eq!(calls_to(&e, AT_EXIT), [[BACKPACK_DESTRUCTOR]]);
        assert_eq!(e.mem.u32(BACKPACK_HANDLE), 0x4242);
        assert_eq!(e.mem.u32(BACKPACK_GUARD) & 1, 1);
        assert_eq!(
            calls_to(&e, GET_OBJECT_BY_NAME),
            [[root, BACKPACK_HANDLE, 0]]
        );
        assert_eq!(
            calls_to(&e, FIND_NEXT_COLLISION_OBJECT),
            [
                [weapon],
                [weapon],
                [first_parent],
                [backpack],
                [backpack],
                [second_parent]
            ]
        );
        // The name is interned once; an ancestor whose collision object has an object with
        // no word at +0x10 is walked past, up to the root, which ends the search.
        e.call_log = Some(vec![]);
        e.mem.set_u32(inner + 0x10, 0);
        e.call(0x00c8_9050, &args![0u32, 0x11u32, root, 1u32]);
        assert!(calls_to(&e, INTERN_STRING).is_empty());
        assert_eq!(calls_to(&e, FIND_NEXT_COLLISION_OBJECT).len(), 3);
    }

    #[test]
    fn child_rigid_bodies_without_a_world_object_get_flagged() {
        let mut e = logging_engine();
        e.register(FIND_NEXT_COLLISION_OBJECT, |e, a| {
            answer_word(e, a[0] + 0x30)
        });
        e.register(GATHER_RIGID_BODIES, |e, a| {
            let data = e.mem.u32(a[0] + 0x34);
            let count = e.mem.u32(a[0] + 0x38);
            let capacity = e.mem.u32(a[0] + 0x3c);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 4, count);
            e.mem.set_u32(a[1] + 8, capacity);
            Ret::default()
        });
        e.register(GET_PROPERTY, |e, a| {
            let holder = e.mem.u32(a[1] + 0x30);
            e.mem.set_u32(a[0], holder);
            ret(a[0])
        });
        e.register(MEMORY_ROUTER, |_, _| ret(ROUTER_OBJECT));
        e.map(ROUTER_OBJECT, 0x20);
        let allocator = object_with_slots(&mut e, 0x100, &[0x10]);
        e.mem.set_u32(ROUTER_OBJECT + 0x10, allocator);
        let node = e.mem.alloc(0x80);
        let collision = e.mem.alloc(0x20);
        let body = e.mem.alloc(0x20);
        let own = e.mem.alloc(0x40);
        let data = e.mem.alloc(0x20);
        e.mem.set_u32(node + 0x30, collision);
        e.mem.set_u32(collision + 0x10, body);
        e.mem.set_u32(body + 8, own);
        e.mem.set_u32(node + 0x34, data);
        e.mem.set_u32(node + 0x38, 4);
        e.mem.set_u32(node + 0x3c, 4);
        let mut flagged = vec![];
        let mut items = vec![own];
        for _ in 0..3 {
            items.push(e.mem.alloc(0x40));
        }
        e.mem.set_u32(items[2] + 8, 0x77);
        for (index, item) in items.iter().enumerate() {
            e.mem.set_u32(data + 4 * index as u32, *item);
            let holder = e.mem.alloc(0x10);
            let target = e.mem.alloc(0x40);
            e.mem.set_u32(holder + 8, target);
            e.mem.set_u32(*item + 0x30, holder);
            flagged.push(target);
        }
        let this = e.mem.alloc(0x30);
        e.call(0x00c8_91c0, &args![this]);
        assert!(calls_to(&e, GATHER_RIGID_BODIES).is_empty());
        e.mem.set_u32(this + 8, node);
        e.call(0x00c8_91c0, &args![this]);
        let flags: Vec<u32> = flagged.iter().map(|t| e.mem.u32(t + 0x30)).collect();
        assert_eq!(flags, [0, 1, 0, 1]);
        let properties = calls_to(&e, GET_PROPERTY);
        assert_eq!(properties.len(), 2);
        assert_eq!(properties[0][1..], [items[1], PROPERTY_KEY]);
        assert_eq!(properties[1][1..], [items[3], PROPERTY_KEY]);
        assert_eq!(calls_to(&e, GATHER_RIGID_BODIES)[0][0], node);
        // The capacity word is not negative: the buffer goes back through the allocator.
        assert_eq!(calls_to(&e, SLOT_BASE + 0x10), [[allocator, data, 16]]);
        // A negative capacity word means the buffer is not owned.
        e.mem.set_u32(node + 0x3c, 0x8000_0004);
        e.call(0x00c8_91c0, &args![this]);
        assert_eq!(calls_to(&e, SLOT_BASE + 0x10).len(), 1);
        // Without a body of the first collision object nothing is flagged.
        e.mem.set_u32(collision + 0x10, 0);
        e.call(0x00c8_91c0, &args![this]);
        assert_eq!(calls_to(&e, GET_PROPERTY).len(), 2 + 2);
    }

    struct Init {
        this: u32,
        node: u32,
        collision: u32,
        reference: u32,
    }

    fn init_setup() -> (Engine, Init) {
        let mut e = logging_engine();
        interlocked_doubles(&mut e);
        e.register(RTTI_CAST, |e, a| answer_word(e, a[0] + 0x30));
        e.register(SET_WORLD_OBJECT, nothing);
        e.register(SET_COLLISION_OBJECT, nothing);
        e.register(FIND_BY_TEXT, |_, _| Ret::default());
        e.register(GET_OBJECT_BY_NAME, |_, _| Ret::default());
        e.register(MODE_PREDICATE, |e, a| ret(e.mem.u32(a[0]) & 1));
        e.register(GET_CONTROLLER, |e, a| answer_word(e, a[0] + 0x40));
        e.register(HEAP_ALLOCATE, |e, _| {
            let block = e.mem.alloc(0x5c);
            ret(block)
        });
        e.register(BLEND_CONTROLLER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], CONTROLLER_VTABLE);
            ret(a[0])
        });
        e.register(SLOT_BASE + 4, nothing);
        e.map(BACKPACK_GUARD, 4);
        e.mem.set_u32(BACKPACK_GUARD, 1);
        e.map(CONTROLLER_VTABLE, 0x100);
        e.mem.set_u32(CONTROLLER_VTABLE + 0x98, SLOT_BASE + 0x98);
        e.register(SLOT_BASE + 0x98, nothing);
        let reference = ni_object(&mut e, 2, SLOT_BASE + 4);
        let node = e.mem.alloc(0x80);
        let cast = e.mem.alloc(0x40);
        let collision = e.mem.alloc(0x40);
        e.mem.set_u32(collision + 8, node);
        e.mem.set_u32(collision + 0x10, reference);
        e.mem.set_u32(collision + 0x30, cast);
        e.mem.set_f32(cast + 0x18, 1.5);
        e.mem.set_f32(cast + 0x1c, 2.5);
        e.mem.set_u32(cast + 0x20, 7);
        e.mem.set_u32(cast + 0x24, 0x24);
        e.mem.set_u32(cast + 0x28, 0x28);
        let this = e.mem.alloc(0x30);
        (
            e,
            Init {
                this,
                node,
                collision,
                reference,
            },
        )
    }

    #[test]
    fn init_takes_over_the_collision_object_and_gives_the_node_a_controller() {
        let (mut e, init) = init_setup();
        let word = scope_word(&mut e);
        e.mem.set_u32(word, 7);
        e.call(
            0x00c8_92c0,
            &args![
                init.this,
                0x31u32,
                0x32u32,
                0x33u32,
                init.collision,
                0x35u32
            ],
        );
        assert_eq!(e.mem.u16(init.this + 0xc), 0x41);
        assert_eq!(e.mem.f32(init.this + 0x18), 1.5);
        assert_eq!(e.mem.f32(init.this + 0x1c), 2.5);
        assert_eq!(e.mem.u32(init.this + 0x20), 7);
        assert_eq!(e.mem.u32(init.this + 0x24), 0x24);
        assert_eq!(e.mem.u32(init.this + 0x28), 0x28);
        assert_eq!(
            calls_to(&e, RTTI_CAST),
            [[init.collision, RTTI_BLEND_COLLISION]]
        );
        assert_eq!(
            calls_to(&e, SET_WORLD_OBJECT),
            [[init.collision, 0], [init.this, init.reference]]
        );
        assert_eq!(calls_to(&e, SET_COLLISION_OBJECT), [[init.node, init.this]]);
        assert_eq!(calls_to(&e, FIND_BY_TEXT), [[0x32, WEAPON_TEXT, 0]]);
        assert_eq!(e.mem.u32(init.reference + 4), 2);
        assert!(calls_to(&e, SLOT_BASE + 4).is_empty());
        assert_eq!(calls_to(&e, GET_CONTROLLER), [[init.node, RTTI_CONTROLLER]]);
        assert_eq!(calls_to(&e, HEAP_ALLOCATE), [[0x5c]]);
        let built = calls_to(&e, BLEND_CONTROLLER_CONSTRUCT);
        assert_eq!(built.len(), 1);
        assert_eq!(e.mem.u16(built[0][0] + 8), 8);
        assert_eq!(calls_to(&e, SLOT_BASE + 0x98), [[built[0][0], init.node]]);
        assert_eq!(e.mem.u32(word), 7);
    }

    #[test]
    fn init_uses_defaults_without_a_blend_collision_object() {
        let (mut e, init) = init_setup();
        e.mem.set_u32(init.collision + 0x30, 0);
        let controller = e.mem.alloc(0x20);
        e.mem.set_u32(controller, CONTROLLER_VTABLE);
        e.mem.set_u16(controller + 8, 0x10);
        e.mem.set_u32(init.node + 0x40, controller);
        e.call(
            0x00c8_92c0,
            &args![init.this, 1u32, 2u32, 3u32, init.collision, 5u32],
        );
        assert_eq!(e.mem.u32(init.this + 0x20), 4);
        assert_eq!(e.mem.f32(init.this + 0x18), 1.0);
        assert_eq!(e.mem.f32(init.this + 0x1c), 1.0);
        assert_eq!(e.mem.u16(controller + 8), 0x18);
        assert!(calls_to(&e, HEAP_ALLOCATE).is_empty());
        assert_eq!(calls_to(&e, SLOT_BASE + 0x98), [[controller, init.node]]);
    }

    #[test]
    fn init_zeroes_the_blend_when_the_motion_mode_is_accepted() {
        let (mut e, init) = init_setup();
        let held = e.mem.alloc(0x20);
        let body = e.mem.alloc(0x100);
        e.mem.set_u32(init.this + 0x10, held);
        e.mem.set_u32(held + 8, body);
        e.mem.set_u8(body + 0xe8, 2);
        e.call(
            0x00c8_92c0,
            &args![init.this, 1u32, 2u32, 3u32, init.collision, 5u32],
        );
        assert_eq!(e.mem.u32(init.this + 0x20), 7);
        e.mem.set_u8(body + 0xe8, 3);
        e.call(
            0x00c8_92c0,
            &args![init.this, 1u32, 2u32, 3u32, init.collision, 5u32],
        );
        assert_eq!(e.mem.u32(init.this + 0x20), 1);
        assert_eq!(e.mem.f32(init.this + 0x18), 0.0);
        assert_eq!(e.mem.f32(init.this + 0x1c), 0.0);
        // Without a body the mode is 5, which is accepted too.
        e.mem.set_u32(held + 8, 0);
        e.mem.set_u32(init.this + 0x20, 9);
        e.call(
            0x00c8_92c0,
            &args![init.this, 1u32, 2u32, 3u32, init.collision, 5u32],
        );
        assert_eq!(e.mem.u32(init.this + 0x20), 1);
    }

    #[test]
    fn init_does_nothing_without_its_three_objects() {
        for missing in 0..3 {
            let (mut e, init) = init_setup();
            let mut words = [2u32, 3, init.collision];
            words[missing] = 0;
            e.call(
                0x00c8_92c0,
                &args![init.this, 1u32, words[0], words[1], words[2], 5u32],
            );
            assert_eq!(e.mem.u16(init.this + 0xc), 0);
            assert!(calls_to(&e, SET_WORLD_OBJECT).is_empty());
            assert!(calls_to(&e, GET_CONTROLLER).is_empty());
        }
    }

    #[test]
    fn the_constructor_runs_the_base_constructor_and_init() {
        let mut e = logging_engine();
        e.register(BLEND_CONSTRUCT, nothing);
        let this = e.mem.alloc(0x30);
        let result = e.call(0x00c8_94d0, &args![this, 1u32, 0u32, 3u32, 4u32, 5u32]);
        assert_eq!(result.u32(), this);
        assert_eq!(e.mem.u32(this), WEAPON_VTABLE);
        assert_eq!(calls_to(&e, BLEND_CONSTRUCT), [[this]]);
        assert_eq!(e.mem.u16(this + 0xc), 0);
    }

    #[test]
    fn a_weapon_object_is_made_only_for_a_node_with_a_collision_object() {
        let mut e = logging_engine();
        e.register(FIND_NEXT_COLLISION_OBJECT, |e, a| {
            answer_word(e, a[0] + 0x30)
        });
        e.register(HEAP_ALLOCATE, |e, _| {
            let block = e.mem.alloc(0x2c);
            ret(block)
        });
        e.register(BLEND_CONSTRUCT, nothing);
        let word = scope_word(&mut e);
        e.mem.set_u32(word, 7);
        let node = e.mem.alloc(0x80);
        let collision = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00c8_9540, &args![1u32, 0u32, 0u32, 4u32]).u32(), 0);
        assert!(calls_to(&e, FIND_NEXT_COLLISION_OBJECT).is_empty());
        assert_eq!(e.call(0x00c8_9540, &args![1u32, 0u32, node, 4u32]).u32(), 0);
        assert!(calls_to(&e, HEAP_ALLOCATE).is_empty());
        e.mem.set_u32(node + 0x30, collision);
        let made = e.call(0x00c8_9540, &args![1u32, 0u32, node, 4u32]).u32();
        assert_ne!(made, 0);
        assert_eq!(e.mem.u32(made), WEAPON_VTABLE);
        assert_eq!(calls_to(&e, HEAP_ALLOCATE), [[0x2c]]);
        assert_eq!(calls_to(&e, BLEND_CONSTRUCT), [[made]]);
        e.register(HEAP_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00c8_9540, &args![1u32, 0u32, node, 4u32]).u32(), 0);
        assert_eq!(calls_to(&e, BLEND_CONSTRUCT).len(), 1);
        assert_eq!(e.mem.u32(word), 7);
    }
}
