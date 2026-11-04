//! `fallout/magic/reanimateeffect.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Sets (`on`) or clears `bit` of the word at `this + 0x1c`.
fn set_flag(e: &mut Engine, this: Ptr, bit: u32, on: bool) {
    let flags = e.mem.u32(this.addr() + 0x1c);
    let flags = if on { flags | bit } else { flags & !bit };
    e.mem.set_u32(this.addr() + 0x1c, flags);
}

// Translated from 00826b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: a `bool`): sets bit 1 (value 2) of the
/// word at `this + 0x1c` when the flag is set, clears it otherwise. What the bit means is
/// not confirmed.
pub fn fn_00826b40(e: &mut Engine, this: Ptr, on: bool) {
    set_flag(e, this, 2, on);
}

// Translated from 00826b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: a `bool`): sets bit 2 (value 4) of the
/// word at `this + 0x1c` when the flag is set, clears it otherwise. What the bit means is
/// not confirmed.
pub fn fn_00826b90(e: &mut Engine, this: Ptr, on: bool) {
    set_flag(e, this, 4, on);
}

layout! {
    /// `ReanimateEffect` (Xbox PDB), 0x70 bytes (`Copy` allocates 0x70): an
    /// `ActiveEffect` (0x48) and the reanimation's state; the PC code uses
    /// the PDB's offsets.
    pub struct ReanimateEffect: 0x70 {
        /// `ActiveEffect::fElapsedSeconds` (Xbox PDB).
        0x04 fElapsedSeconds: u32,
        /// `ActiveEffect::pCaster` (Xbox PDB).
        0x28 pCaster: u32,
        /// `pBody` (Xbox PDB): `bhkRigidBody*`.
        0x48 pBody: u32,
        /// `iState` (Xbox PDB).
        0x4C iState: i32,
        /// `fSaveTime` (Xbox PDB).
        0x50 fSaveTime: u32,
    }
}

/// `StartPos` (`NiPoint3`, +0x54) and `StartRot` (`NiQuaternion`, +0x60).
const START_POS: u32 = 0x54;
const START_ROT: u32 = 0x60;
/// `ReanimateEffect`'s vtable (`0107c06c`).
const REANIMATE_EFFECT_VTABLE: u32 = 0x0107_c06c;
/// `ActiveEffect`'s constructor body (`00803d30`, `thiscall`, three words).
const ACTIVE_EFFECT_CONSTRUCT: u32 = 0x0080_3d30;
/// `006815c0` (`thiscall`): answers `this`.
const SELF_006815C0: u32 = 0x0068_15c0;
/// `00416870` (`thiscall`, three floats): fills a point.
const SET_POINT_00416870: u32 = 0x0041_6870;
/// The four words of the identity rotation the constructor copies
/// (`011a9ea4`).
const IDENTITY_ROTATION: u32 = 0x011a_9ea4;
/// `operator new` (`00401000`, `cdecl`: the size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `0045cd60` (`thiscall`): the effect's (or a process's) word the code
/// tests as the caster; `MagicCaster::GetCasterAsActor` (engine map,
/// `00815410`).
const EFFECT_CASTER_0045CD60: u32 = 0x0045_cd60;
const GET_CASTER_AS_ACTOR: u32 = 0x0081_5410;
/// `ActiveEffect::Dispel` (engine map, `00804210`, `thiscall`, one word).
const DISPEL: u32 = 0x0080_4210;
/// `0059bb30` (`thiscall`): the effect's magic target;
/// `MagicTarget::GetTargetAsActor` (engine map, `00822b40`).
const EFFECT_TARGET_0059BB30: u32 = 0x0059_bb30;
const GET_TARGET_AS_ACTOR: u32 = 0x0082_2b40;
/// `Actor::SetLifeState` (engine map, `008a1800`, one word),
/// `Actor::SetGhost` (`008acf10`, one word), `00493bb0` (returns a byte),
/// `MiddleHighProcess::GetSavedAcquireObject` (`008d8520`: the process),
/// `0045bb80` (the 3D's position), `TESObjectREFR::SetLocationOnReference`
/// (`00575830`, one word), `008a6840` (one word),
/// `TESObjectREFR::InitAnimation` (`005659f0`),
/// `Actor::InitLightingPropertyPtr` (`008b0bd0`, one word), all `thiscall`
/// on the actor.
const SET_LIFE_STATE: u32 = 0x008a_1800;
const SET_GHOST: u32 = 0x008a_cf10;
const ACTOR_TEST_00493BB0: u32 = 0x0049_3bb0;
const PROCESS_OF: u32 = 0x008d_8520;
const POSITION_0045BB80: u32 = 0x0045_bb80;
const SET_LOCATION_ON_REFERENCE: u32 = 0x0057_5830;
const ACTOR_00826941_008A6840: u32 = 0x008a_6840;
const INIT_ANIMATION: u32 = 0x0056_59f0;
const INIT_LIGHTING_PROPERTY_PTR: u32 = 0x008b_0bd0;
/// `TESPackage::CreatePackage` (engine map, `00670b90`, `cdecl`, one word),
/// `TESPackage::SetPackType` (`00670fc0`, `thiscall`, one word), `00984f60`
/// and `0067a3a0` (`thiscall`, one word).
const CREATE_PACKAGE: u32 = 0x0067_0b90;
const SET_PACK_TYPE: u32 = 0x0067_0fc0;
const PACKAGE_SET_00984F60: u32 = 0x0098_4f60;
const PACKAGE_SET_0067A3A0: u32 = 0x0067_a3a0;
/// `bhkWorld::SetMotion` (engine map, `00c6a350`, `cdecl`, five words).
const SET_MOTION: u32 = 0x00c6_a350;
/// The fixed string constructor `00438170` (`thiscall`, one word) and
/// destructor `004381b0`, and the bone name `"Bip01 Spine2"` (`0101f618`).
const FIXED_STRING_CONSTRUCT: u32 = 0x0043_8170;
const FIXED_STRING_DESTRUCT: u32 = 0x0043_81b0;
const SPINE_BONE: u32 = 0x0101_f618;
/// `bhkCollisionObject::GetbhkCollisionObject` (engine map, `0043b610`,
/// `cdecl`) and `006fa820` (`thiscall`: the collision object's body).
const GET_COLLISION_OBJECT: u32 = 0x0043_b610;
const COLLISION_BODY_006FA820: u32 = 0x006f_a820;
/// `bhkRigidBody::GetPosition` / `GetRotation` (engine map, `00560bc0` /
/// `00560c20`, `thiscall`, one word: where to write).
const GET_POSITION: u32 = 0x0056_0bc0;
const GET_ROTATION: u32 = 0x0056_0c20;
/// `005b5e40` (`cdecl`, one word: a message) and the two messages
/// (`0107c0c8` from `Start`, `0107c110` from `FinishInitLoadGame`).
const PRINT_005B5E40: u32 = 0x005b_5e40;
const NO_BONE_MESSAGE: u32 = 0x0107_c0c8;
const NO_BONE_LOAD_MESSAGE: u32 = 0x0107_c110;
/// The `double`s 5.0 (`01020998`), 4.0 (`0101db80`), 2.0 (`01011590`) and
/// 1.0 (`01012070`), the `float`s 100.0 (`01016410`) and 0.5 (`01016248`),
/// and the `float` global at `011dcb54`.
const RISE_TIME: u32 = 0x0102_0998;
const RISE_DIVISOR: u32 = 0x0101_db80;
const STAND_TIME: u32 = 0x0101_1590;
const ONE_DOUBLE: u32 = 0x0101_2070;
const RISE_HEIGHT: u32 = 0x0101_6410;
const HALF: u32 = 0x0101_6248;
const ANGLE_011DCB54: u32 = 0x011d_cb54;
/// `0040ebd0` (`cdecl`, two floats: the smaller), `004a3e00` (`cdecl`, two
/// words: copies a point), `00560d30` (`thiscall`, one word: the address of
/// a coordinate), `004a3e90` (`cdecl`, one float), `005dbf20` (`thiscall`,
/// one float) and `005dbf70` (`thiscall`, three words): the rise's
/// interpolation.
const MIN_0040EBD0: u32 = 0x0040_ebd0;
const COPY_POINT_004A3E00: u32 = 0x004a_3e00;
const COORDINATE_00560D30: u32 = 0x0056_0d30;
const SCALE_004A3E90: u32 = 0x004a_3e90;
const SET_FRACTION_005DBF20: u32 = 0x005d_bf20;
const LERP_005DBF70: u32 = 0x005d_bf70;
/// `00439ef0` (`thiscall`, two words: the difference of two points) and
/// `atan2` (`004b5550`, `cdecl`, two floats).
const SUBTRACT_00439EF0: u32 = 0x0043_9ef0;
const ATAN2: u32 = 0x004b_5550;
/// `NiMatrix3::FromEulerAnglesZYX` (engine map, `00a59780`, `thiscall`,
/// three floats), `0056df00` and `006240d0` (`thiscall`: construct),
/// `0056dff0` and `00561500` (`cdecl`, two words: convert),
/// `hkQuaternion::set` (`00cb26e0`, `thiscall`, one word),
/// `hkQuaternion::setSlerp` (`00cb2850`, `thiscall`, three words) and
/// `00c8de20` (`thiscall` on the body, four words).
const FROM_EULER_ANGLES_ZYX: u32 = 0x00a5_9780;
const CONSTRUCT_0056DF00: u32 = 0x0056_df00;
const CONVERT_0056DFF0: u32 = 0x0056_dff0;
const QUATERNION_CONSTRUCT_006240D0: u32 = 0x0062_40d0;
const CONVERT_00561500: u32 = 0x0056_1500;
const QUATERNION_SET: u32 = 0x00cb_26e0;
const QUATERNION_SET_SLERP: u32 = 0x00cb_2850;
const BODY_MOVE_00C8DE20: u32 = 0x00c8_de20;
/// `Actor::ReloadTargets` (engine map, `008b0b00`, one word),
/// `Animation::PlayGroup` (`00494740`, four words), `00c9bdf0` (`cdecl`,
/// two words), `005d43c0` (`thiscall`: the actor's extra data list) and
/// `ExtraDataList::SetRagDollData` (`0041d490`, one word).
const RELOAD_TARGETS: u32 = 0x008b_0b00;
const PLAY_GROUP: u32 = 0x0049_4740;
const RAGDOLL_00C9BDF0: u32 = 0x00c9_bdf0;
const EXTRA_LIST_005D43C0: u32 = 0x005d_43c0;
const SET_RAGDOLL_DATA: u32 = 0x0041_d490;
/// `BGSSaveFormBuffer::GetForm` (engine map's name for `007af430`: the
/// reference's base form), `__RTDynamicCast` (`00ec43fb`) and the type
/// descriptors `01183108` (`TESBoundObject`), `01183a1c` (`TESNPC`),
/// `011841cc` (`TESObjectREFR`) and `011a0f14` (`Character`).
const BASE_FORM_007AF430: u32 = 0x007a_f430;
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_BOUND_OBJECT: u32 = 0x0118_3108;
const TYPE_TES_NPC: u32 = 0x0118_3a1c;
const TYPE_TES_OBJECT_REFR: u32 = 0x0118_41cc;
const TYPE_CHARACTER: u32 = 0x011a_0f14;
/// `TESNPC::SwapEyes` (engine map, `0060a950`, one word) with the word at
/// `011cb600`, and `Actor::GetFaceAnimationData` (`008adcb0`).
const SWAP_EYES: u32 = 0x0060_a950;
const EYES_011CB600: u32 = 0x011c_b600;
const GET_FACE_ANIMATION_DATA: u32 = 0x008a_dcb0;
/// `CommandEffect::InitCommandedActor` (engine map, `0080b850`, `cdecl`),
/// `0080b8f0` (`cdecl`) and `CommandEffect::CleanupCommandedActor`
/// (`0080b9e0`, `cdecl`, three words).
const INIT_COMMANDED_ACTOR: u32 = 0x0080_b850;
const COMMANDED_UPDATE_0080B8F0: u32 = 0x0080_b8f0;
const CLEANUP_COMMANDED_ACTOR: u32 = 0x0080_b9e0;
/// `004226e0` (`thiscall`, returns a byte) on the object at `011c3f2c`.
const GAME_TEST_004226E0: u32 = 0x0042_26e0;
const GAME_011C3F2C: u32 = 0x011c_3f2c;
/// `Actor::Kill` (engine map, `0089d900`, `thiscall`, two words).
const KILL: u32 = 0x0089_d900;
/// `00608d80` (`thiscall`, returns a byte) and `004f8960` (`thiscall`).
const ACTOR_TEST_00608D80: u32 = 0x0060_8d80;
const ACTOR_VALUE_004F8960: u32 = 0x004f_8960;
/// `ActiveEffect::GetSaveSize`, `SaveGame`, `LoadGame` (engine map,
/// `00804f80`, `008050d0`, `00805330`), `ActiveEffect::FinishInitLoadGame`
/// (`00805760`), all `thiscall` with one word; `004534f0` (`thiscall`, one
/// word, does nothing).
const ACTIVE_EFFECT_GET_SAVE_SIZE: u32 = 0x0080_4f80;
const ACTIVE_EFFECT_SAVE_GAME: u32 = 0x0080_50d0;
const ACTIVE_EFFECT_LOAD_GAME: u32 = 0x0080_5330;
const ACTIVE_EFFECT_FINISH_INIT_LOAD_GAME: u32 = 0x0080_5760;
const REVERT_BASE_004534F0: u32 = 0x0045_34f0;
/// The save/load game object (`011de45c`) and its write and read
/// (`008579b0` / `008579e0`, `thiscall`: buffer, size).
const SAVE_LOAD_GAME: u32 = 0x011d_e45c;
const SAVE_WRITE: u32 = 0x0085_79b0;
const LOAD_READ: u32 = 0x0085_79e0;
/// The states `iState` steps through.
const STATE_START: i32 = 0;
const STATE_RISING: i32 = 10;
const STATE_RAGDOLL: i32 = 0x14;
const STATE_STANDING: i32 = 0x1e;
const STATE_WAKING: i32 = 0x28;
const STATE_COMMANDED: i32 = 0x32;

// Translated from 008265b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ReanimateEffect`'s constructor (`thiscall`, `RET 0xC`: caster, magic
/// item, effect item): the `ActiveEffect` base (`00803d30`), the vtable,
/// `StartPos` and `StartRot` constructed (`006815c0`), `pBody` 0,
/// `fSaveTime` 0.0, `iState` 0, `StartPos` from a local point of zeros
/// (`00416870`), `StartRot` from the four words at `011a9ea4`. Returns
/// `this`.
// Translation incomplete: 008265b0 - the FS:[0] exception frame (unwinding the base on a throw) is not translated
pub fn fn_008265b0(
    e: &mut Engine,
    this: Ptr<ReanimateEffect>,
    caster: u32,
    spell: u32,
    effect: u32,
) -> Ptr<ReanimateEffect> {
    let base = this.addr();
    e.call(ACTIVE_EFFECT_CONSTRUCT, &args![this, caster, spell, effect]);
    e.mem.set_u32(base, REANIMATE_EFFECT_VTABLE);
    e.call(SELF_006815C0, &args![base.wrapping_add(START_POS)]);
    e.call(SELF_006815C0, &args![base.wrapping_add(START_ROT)]);
    e.set(this, ReanimateEffect::pBody, 0);
    e.set(this, ReanimateEffect::fSaveTime, 0);
    e.set(this, ReanimateEffect::iState, 0);
    e.with_stack(0xc, |e, point| {
        let filled = e
            .call(SET_POINT_00416870, &args![point, 0u32, 0u32, 0u32])
            .u32();
        for i in 0..3 {
            let word = e.mem.u32(filled.wrapping_add(4 * i));
            e.mem.set_u32(base.wrapping_add(START_POS + 4 * i), word);
        }
    });
    for i in 0..4 {
        let word = e.mem.u32(IDENTITY_ROTATION + 4 * i);
        e.mem.set_u32(base.wrapping_add(START_ROT + 4 * i), word);
    }
    this
}

// Translated from 008266a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ReanimateEffect::Copy` (engine map; `thiscall`): a new 0x70-byte
/// `ReanimateEffect` (`00401000`, `008265b0` with the caster +0x28, magic
/// item +8 and effect item +0xC, or 0), handed to `this`'s slot +0x44
/// (`CopyData`), then `pBody`, `iState`, `fSaveTime` (through `FLD`/`FSTP`),
/// `StartPos` and `StartRot` copied into it (a failed allocation writes
/// through the null copy, as the code does). Returns the copy.
// Translation incomplete: 008266a0 - the FS:[0] exception frame (freeing the block when the constructor throws) is not translated
pub fn reanimate_effect_copy(e: &mut Engine, this: Ptr<ReanimateEffect>) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x70u32]).u32();
    let copy = if block != 0 {
        let base = this.addr();
        let caster = e.mem.u32(base.wrapping_add(0x28));
        let spell = e.mem.u32(base.wrapping_add(8));
        let effect = e.mem.u32(base.wrapping_add(0xc));
        fn_008265b0(e, Ptr::new(block), caster, spell, effect).addr()
    } else {
        0
    };
    e.vcall(this.addr(), 0x44, &args![copy]);
    let target = Ptr::<ReanimateEffect>::new(copy);
    let body = e.get(this, ReanimateEffect::pBody);
    e.set(target, ReanimateEffect::pBody, body);
    let state = e.get(this, ReanimateEffect::iState);
    e.set(target, ReanimateEffect::iState, state);
    let time = x87_quiet_bits(e.get(this, ReanimateEffect::fSaveTime));
    e.set(target, ReanimateEffect::fSaveTime, time);
    for i in 0..3 {
        let word = e.mem.u32(this.addr().wrapping_add(START_POS + 4 * i));
        e.mem.set_u32(copy.wrapping_add(START_POS + 4 * i), word);
    }
    for i in 0..4 {
        let word = e.mem.u32(this.addr().wrapping_add(START_ROT + 4 * i));
        e.mem.set_u32(copy.wrapping_add(START_ROT + 4 * i), word);
    }
    copy
}

/// The effect's target as an actor (`0059bb30`, then
/// `MagicTarget::GetTargetAsActor`), or 0.
fn target_actor(e: &mut Engine, this: u32) -> u32 {
    if e.call(EFFECT_TARGET_0059BB30, &args![this]).u32() == 0 {
        return 0;
    }
    let target = e.call(EFFECT_TARGET_0059BB30, &args![this]).u32();
    e.call(GET_TARGET_AS_ACTOR, &args![target]).u32()
}

/// The effect's caster as an actor (`0045cd60`, then
/// `MagicCaster::GetCasterAsActor`), or 0.
fn caster_actor(e: &mut Engine, this: u32) -> u32 {
    if e.call(EFFECT_CASTER_0045CD60, &args![this]).u32() == 0 {
        return 0;
    }
    let caster = e.call(EFFECT_CASTER_0045CD60, &args![this]).u32();
    e.call(GET_CASTER_AS_ACTOR, &args![caster]).u32()
}

/// Finds the `"Bip01 Spine2"` bone of `node` through its slot +0x9C, with
/// the name as a fixed string on the stack.
fn spine_bone(e: &mut Engine, node: u32) -> u32 {
    e.with_stack(4, |e, name| {
        let made = e
            .call(FIXED_STRING_CONSTRUCT, &args![name, SPINE_BONE])
            .u32();
        let bone = e.vcall(node, 0x9c, &args![made]).u32();
        e.call(FIXED_STRING_DESTRUCT, &args![name]);
        bone
    })
}

/// Takes the body of `bone`'s collision object as `pBody` and calls its
/// slot +0xE4 with 4; `false` when the bone has no collision object.
fn take_body(e: &mut Engine, this: Ptr<ReanimateEffect>, bone: u32) -> bool {
    let collision = e.call(GET_COLLISION_OBJECT, &args![bone]).u32();
    if collision == 0 {
        return false;
    }
    let body = e.call(COLLISION_BODY_006FA820, &args![collision]).u32();
    e.set(this, ReanimateEffect::pBody, body);
    let body = e.get(this, ReanimateEffect::pBody);
    e.vcall(body, 0xe4, &args![4u32]);
    true
}

// Translated from 008267b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ReanimateEffect::Start` (engine map; `thiscall`). Without a caster
/// (`0045cd60`) the effect is dispelled with 1. Without a target actor, or
/// when its slot +0x22C (with 0) answers false, it is dispelled with 0.
/// Otherwise, on the actor: slot +0xD4 with 0, `SetLifeState(4)`, slot
/// +0x434 with 0 when `00493bb0` answers true, the process's slot +0x6F8
/// with 1 when `0045cd60` on the process answers 0, `SetGhost(1)`, slot
/// +0x2C0 with the caster's slot +0x2C, `SetLocationOnReference` at its
/// 3D's position (slot +0x1D0) when it has 3D, slot +0x1C4, the process's
/// slot +0x628 with 0, `008a6840` with 0, slot +0x3AC with 0x10 and the
/// actor value 0x10's base (slot 0 of +0xA4, with 0) minus its current
/// value (slot +0xC), `InitAnimation` and `InitLightingPropertyPtr` when
/// it has 3D; a new package of type 0x1B (`CreatePackage`, `SetPackType`,
/// both flags of this file set, `00984f60` with 0x19, `0067a3a0` with 1)
/// given through slot +0x2F4 (with 1, 1). With 3D: `bhkWorld::SetMotion`
/// (1, 1, 0, 1), then the `"Bip01 Spine2"` bone's body becomes `pBody`
/// (slot +0xE4 with 4) and gives `StartPos` and `StartRot`; a missing bone
/// prints the message at `0107c0c8`.
// Translation incomplete: 008267b0 - the FS:[0] exception frame (destroying the fixed string on a throw) is not translated
pub fn reanimate_effect_start(e: &mut Engine, this: Ptr<ReanimateEffect>) {
    let base = this.addr();
    if e.call(EFFECT_CASTER_0045CD60, &args![base]).u32() == 0 {
        e.call(DISPEL, &args![base, 1u32]);
        return;
    }
    let actor = target_actor(e, base);
    if actor == 0 || !e.vcall(actor, 0x22c, &args![0u32]).bool() {
        e.call(DISPEL, &args![base, 0u32]);
        return;
    }
    e.vcall(actor, 0xd4, &args![0u32]);
    e.call(SET_LIFE_STATE, &args![actor, 4u32]);
    if e.call(ACTOR_TEST_00493BB0, &args![actor]).bool() {
        e.vcall(actor, 0x434, &args![0u32]);
    }
    let process = e.call(PROCESS_OF, &args![actor]).u32();
    if e.call(EFFECT_CASTER_0045CD60, &args![process]).u32() == 0 {
        e.vcall(process, 0x6f8, &args![1u32]);
    }
    e.call(SET_GHOST, &args![actor, 1u32]);
    let caster = e.call(EFFECT_CASTER_0045CD60, &args![base]).u32();
    let heading = e.vcall(caster, 0x2c, &[]).u32();
    e.vcall(actor, 0x2c0, &args![heading]);
    if e.vcall(actor, 0x1d0, &[]).u32() != 0 {
        let node = e.vcall(actor, 0x1d0, &[]).u32();
        let position = e.call(POSITION_0045BB80, &args![node]).u32();
        e.call(SET_LOCATION_ON_REFERENCE, &args![actor, position]);
    }
    e.vcall(actor, 0x1c4, &[]);
    let process = e.call(PROCESS_OF, &args![actor]).u32();
    e.vcall(process, 0x628, &args![0u32]);
    e.call(ACTOR_00826941_008A6840, &args![actor, 0u32]);
    let values = actor.wrapping_add(0xa4);
    let base_value = e.vcall(values, 0, &args![0x10u32, 0u32]).i32();
    let current = e.vcall(values, 0xc, &args![0x10u32]).st0;
    let damage = x87_store32(x87_sub(f64::from(base_value), current));
    e.vcall(actor, 0x3ac, &args![0x10u32, damage]);
    if e.vcall(actor, 0x1d0, &[]).u32() != 0 {
        e.call(INIT_ANIMATION, &args![actor]);
        let node = e.vcall(actor, 0x1d0, &[]).u32();
        e.call(INIT_LIGHTING_PROPERTY_PTR, &args![actor, node]);
    }
    let package = e.call(CREATE_PACKAGE, &args![0x1bu32]).u32();
    e.call(SET_PACK_TYPE, &args![package, 0x1bu32]);
    fn_00826b40(e, Ptr::new(package), true);
    fn_00826b90(e, Ptr::new(package), true);
    e.call(PACKAGE_SET_00984F60, &args![package, 0x19u32]);
    e.call(PACKAGE_SET_0067A3A0, &args![package, 1u32]);
    e.vcall(actor, 0x2f4, &args![package, 1u32, 1u32]);
    let node = e.vcall(actor, 0x1d0, &[]).u32();
    if node == 0 {
        return;
    }
    e.call(SET_MOTION, &args![node, 1u32, 1u32, 0u32, 1u32]);
    let bone = spine_bone(e, node);
    if bone == 0 {
        e.call(PRINT_005B5E40, &args![NO_BONE_MESSAGE]);
        return;
    }
    if take_body(e, this, bone) {
        let body = e.get(this, ReanimateEffect::pBody);
        e.call(GET_POSITION, &args![body, base.wrapping_add(START_POS)]);
        let body = e.get(this, ReanimateEffect::pBody);
        e.call(GET_ROTATION, &args![body, base.wrapping_add(START_ROT)]);
    }
}

/// `fElapsedSeconds - fSaveTime` as the x87 computes it (`FLD`, `FSUB`).
fn time_in_state(e: &Engine, this: Ptr<ReanimateEffect>) -> f64 {
    let elapsed = e.get(this, ReanimateEffect::fElapsedSeconds);
    let saved = e.get(this, ReanimateEffect::fSaveTime);
    x87_sub(x87_load32(elapsed), x87_m32(saved))
}

/// The rising state's step of `ReanimateEffect::Update` (`00826cd6` to
/// `00826f77`): the locals at their `EBP` offsets in `frame` (`ebp` is
/// `EBP`).
fn rise(
    e: &mut Engine,
    this: Ptr<ReanimateEffect>,
    actor: u32,
    caster: u32,
    seconds: u32,
    ebp: u32,
) {
    let base = this.addr();
    let local = |offset: u32| ebp.wrapping_sub(offset);
    let process = e.call(PROCESS_OF, &args![actor]).u32();
    if e.call(EFFECT_CASTER_0045CD60, &args![process]).u32() == 0 {
        e.vcall(process, 0x6f8, &args![1u32]);
    }
    if time_in_state(e, this) > e.global::<f64>(RISE_TIME) {
        e.set(this, ReanimateEffect::iState, STATE_RAGDOLL);
    }
    let fraction = x87_store32(x87_div(
        time_in_state(e, this),
        e.global::<f64>(RISE_DIVISOR),
    ));
    e.mem.set_u32(local(0x20), fraction);
    let fraction = x87_store32(
        e.call(
            MIN_0040EBD0,
            &args![1.0f32.to_bits(), x87_quiet_bits(fraction)],
        )
        .st0,
    );
    e.mem.set_u32(local(0x24), fraction);
    let (low, high) = (local(0x50), local(0x40));
    e.call(SELF_006815C0, &args![high]);
    e.call(SELF_006815C0, &args![low]);
    e.call(
        COPY_POINT_004A3E00,
        &args![high, base.wrapping_add(START_POS)],
    );
    e.call(
        COPY_POINT_004A3E00,
        &args![low, base.wrapping_add(START_POS)],
    );
    let z = e.call(COORDINATE_00560D30, &args![high, 2u32]).u32();
    let height = x87_quiet_bits(e.global::<u32>(RISE_HEIGHT));
    let lift = e.call(SCALE_004A3E90, &args![height]).st0;
    let lifted = x87_store32(x87_add(lift, x87_m32(e.mem.u32(z))));
    e.mem.set_u32(z, lifted);
    let position = local(0x70);
    e.call(SELF_006815C0, &args![position]);
    let blend = local(0x80);
    e.call(
        SET_FRACTION_005DBF20,
        &args![blend, x87_quiet_bits(fraction)],
    );
    e.call(LERP_005DBF70, &args![position, low, high, blend]);
    let body_position = local(0x8c);
    e.call(SELF_006815C0, &args![body_position]);
    let body = e.get(this, ReanimateEffect::pBody);
    e.call(GET_POSITION, &args![body, body_position]);
    let caster_position = e.vcall(caster, 0x1f4, &[]).u32();
    let direction = local(0x98);
    e.call(
        SUBTRACT_00439EF0,
        &args![caster_position, direction, body_position],
    );
    e.mem.set_u32(local(0x90), 0);
    let x = x87_quiet_bits(e.mem.u32(direction));
    let y = x87_quiet_bits(e.mem.u32(local(0x94)));
    let angle = e.call(ATAN2, &args![y, x]).st0;
    let heading = x87_store32(-angle);
    e.mem.set_u32(local(0x9c), heading);
    let angle = x87_quiet_bits(e.global::<u32>(ANGLE_011DCB54));
    e.mem.set_u32(local(0xa0), angle);
    let half = x87_quiet_bits(e.global::<u32>(HALF));
    e.mem.set_u32(local(0xa4), half);
    let rotation = local(0xc8);
    e.call(SELF_006815C0, &args![rotation]);
    let heading = x87_quiet_bits(e.mem.u32(local(0x9c)));
    let half = x87_quiet_bits(e.mem.u32(local(0xa4)));
    let angle = x87_quiet_bits(e.mem.u32(local(0xa0)));
    e.call(
        FROM_EULER_ANGLES_ZYX,
        &args![rotation, heading, half, angle],
    );
    let target_rotation = local(0x100);
    e.call(CONSTRUCT_0056DF00, &args![target_rotation]);
    e.call(CONVERT_0056DFF0, &args![target_rotation, rotation]);
    let start = local(0x110);
    e.call(QUATERNION_CONSTRUCT_006240D0, &args![start]);
    e.call(
        CONVERT_00561500,
        &args![start, base.wrapping_add(START_ROT)],
    );
    let end = local(0x120);
    fn_00827200(e, Ptr::new(end), target_rotation);
    let blended = local(0x130);
    e.call(QUATERNION_CONSTRUCT_006240D0, &args![blended]);
    let fraction = x87_quiet_bits(e.mem.u32(local(0x24)));
    e.call(QUATERNION_SET_SLERP, &args![blended, start, end, fraction]);
    let inverse = x87_store32(x87_div(1.0, x87_m32(seconds)));
    e.mem.set_u32(local(0x134), inverse);
    let body = e.get(this, ReanimateEffect::pBody);
    e.call(
        BODY_MOVE_00C8DE20,
        &args![
            body,
            position,
            blended,
            x87_quiet_bits(inverse),
            1.0f32.to_bits()
        ],
    );
    let caster = e.call(EFFECT_CASTER_0045CD60, &args![base]).u32();
    let heading = e.vcall(caster, 0x2c, &[]).u32();
    e.vcall(actor, 0x2c0, &args![heading]);
}

// Translated from 00826be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ReanimateEffect::Update` (engine map; `thiscall`, `RET 4`: the elapsed
/// seconds). Without a caster actor the effect is dispelled with 1; with
/// no `pBody` or no target actor nothing happens; a target whose slot
/// +0x22C (with 0) answers true dispels it with 0. Otherwise `iState`
/// steps: 0 sets `fSaveTime` 0.0 and goes to 10 (rising, at once); while
/// rising (process slot +0x6F8 with 1 when `0045cd60` on the process is 0;
/// after more than 5 seconds the state becomes 0x14) the body is moved to
/// the start position raised by 100 and the heading towards the caster,
/// interpolated by `min(1, t / 4)`, through `00c8de20` with `1 / seconds`,
/// and the actor's heading follows the caster's; at 0x14, once the actor
/// has a biped (slot +0x1E4) and 3D, it goes to 0x1E (`ReloadTargets`,
/// `PlayGroup`, `00c9bdf0`, process slot +0x6F8 with 0, the rag doll data
/// cleared, `fSaveTime` set); after 2 seconds at 0x1E to 0x28 (`SetMotion`
/// 4, an NPC's eyes swapped, face animation slot +0xD8 with 0, 0); after 1
/// second at 0x28 to 0x32 (`SetLifeState(4)`, `SetGhost(0)`,
/// `InitCommandedActor`); at 0x32 the process's slot +0x644 with 0 and
/// `0080b8f0`.
///
/// The stack frame is modelled as a block of the frame's size, the locals
/// at their `EBP` offsets.
// Translation incomplete: 00826be0 - the /GS stack cookie check (00ec408c) is not translated
pub fn reanimate_effect_update(e: &mut Engine, this: Ptr<ReanimateEffect>, seconds: f32) {
    let base = this.addr();
    let actor = target_actor(e, base);
    let caster = caster_actor(e, base);
    if caster == 0 {
        e.call(DISPEL, &args![base, 1u32]);
        return;
    }
    if e.get(this, ReanimateEffect::pBody) == 0 || actor == 0 {
        return;
    }
    if e.vcall(actor, 0x22c, &args![0u32]).bool() {
        e.call(DISPEL, &args![base, 0u32]);
        return;
    }
    if e.get(this, ReanimateEffect::iState) == STATE_START {
        e.set(this, ReanimateEffect::fSaveTime, 0);
        e.set(this, ReanimateEffect::iState, STATE_RISING);
    }
    let state = e.get(this, ReanimateEffect::iState);
    if state == STATE_RISING {
        e.with_stack(0x160, |e, frame| {
            let ebp = frame.addr().wrapping_add(0x160);
            rise(e, this, actor, caster, seconds.to_bits(), ebp);
        });
    } else if state == STATE_RAGDOLL {
        let biped = e.vcall(actor, 0x1e4, &[]).u32();
        let node = e.vcall(actor, 0x1d0, &[]).u32();
        if node != 0 && biped != 0 {
            e.set(this, ReanimateEffect::iState, STATE_STANDING);
            e.call(RELOAD_TARGETS, &args![actor, 1u32]);
            e.call(
                PLAY_GROUP,
                &args![biped, 0u32, 0u32, 0xffff_ffffu32, 0xffff_ffffu32],
            );
            e.call(RAGDOLL_00C9BDF0, &args![node, 0u32]);
            let process = e.call(PROCESS_OF, &args![actor]).u32();
            if e.call(EFFECT_CASTER_0045CD60, &args![process]).u32() == 0 {
                e.vcall(process, 0x6f8, &args![0u32]);
            }
            let list = e.call(EXTRA_LIST_005D43C0, &args![actor]).u32();
            e.call(SET_RAGDOLL_DATA, &args![list, 0u32]);
            let elapsed = x87_quiet_bits(e.get(this, ReanimateEffect::fElapsedSeconds));
            e.set(this, ReanimateEffect::fSaveTime, elapsed);
        }
    } else if state == STATE_STANDING {
        if time_in_state(e, this) > e.global::<f64>(STAND_TIME) {
            let elapsed = x87_quiet_bits(e.get(this, ReanimateEffect::fElapsedSeconds));
            e.set(this, ReanimateEffect::fSaveTime, elapsed);
            e.set(this, ReanimateEffect::iState, STATE_WAKING);
            let node = e.vcall(actor, 0x1d0, &[]).u32();
            if node != 0 {
                e.call(SET_MOTION, &args![node, 4u32, 1u32, 0u32, 1u32]);
            }
            let form = e.call(BASE_FORM_007AF430, &args![actor]).u32();
            let npc = e
                .call(
                    DYNAMIC_CAST,
                    &args![form, 0u32, TYPE_TES_BOUND_OBJECT, TYPE_TES_NPC, 0u32],
                )
                .u32();
            if npc != 0 {
                let eyes = e.mem.u32(EYES_011CB600);
                e.call(SWAP_EYES, &args![npc, eyes]);
            }
            let face = e.call(GET_FACE_ANIMATION_DATA, &args![actor]).u32();
            if face != 0 {
                e.vcall(face, 0xd8, &args![0u32, 0u32]);
            }
        }
    } else if state == STATE_WAKING {
        if time_in_state(e, this) > e.global::<f64>(ONE_DOUBLE) {
            e.set(this, ReanimateEffect::iState, STATE_COMMANDED);
            e.call(SET_LIFE_STATE, &args![actor, 4u32]);
            e.call(SET_GHOST, &args![actor, 0u32]);
            e.call(INIT_COMMANDED_ACTOR, &args![actor, caster]);
        }
    } else if state == STATE_COMMANDED {
        let process = e.call(PROCESS_OF, &args![actor]).u32();
        e.vcall(process, 0x644, &args![0u32]);
        e.call(COMMANDED_UPDATE_0080B8F0, &args![actor, caster]);
    }
}

// Translated from 00827200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`): constructs `this` (`006815c0`) and sets it
/// from the word (`hkQuaternion::set`, `00cb26e0`). Returns `this`.
pub fn fn_00827200(e: &mut Engine, this: Ptr, source: u32) -> Ptr {
    e.call(SELF_006815C0, &args![this]);
    e.call(QUATERNION_SET, &args![this, source]);
    this
}

// Translated from 00827230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ReanimateEffect::Finish` (engine map; `thiscall`): nothing when
/// `004226e0` on `011c3f2c` answers true or there is no target actor.
/// Before the commanded state (0x32): `SetGhost(0)`, the body's slot +0xE4
/// with 1, `Actor::Kill(0, 0.0)`; in it, `Kill` only when the actor's slot
/// +0x22C (with 0) answers false. Then `CleanupCommandedActor` with the
/// caster actor when there is one.
pub fn reanimate_effect_finish(e: &mut Engine, this: Ptr<ReanimateEffect>) {
    let base = this.addr();
    let game = e.mem.u32(GAME_011C3F2C);
    if e.call(GAME_TEST_004226E0, &args![game]).bool() {
        return;
    }
    let actor = target_actor(e, base);
    if actor == 0 {
        return;
    }
    if e.get(this, ReanimateEffect::iState) < STATE_COMMANDED {
        e.call(SET_GHOST, &args![actor, 0u32]);
        if e.get(this, ReanimateEffect::pBody) != 0 {
            let body = e.get(this, ReanimateEffect::pBody);
            e.vcall(body, 0xe4, &args![1u32]);
        }
        e.call(KILL, &args![actor, 0u32, 0u32]);
    } else if !e.vcall(actor, 0x22c, &args![0u32]).bool() {
        e.call(KILL, &args![actor, 0u32, 0u32]);
    }
    let caster = caster_actor(e, base);
    if caster != 0 {
        e.call(CLEANUP_COMMANDED_ACTOR, &args![actor, caster, 0u32]);
    }
}

// Translated from 00827340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ReanimateEffect::CheckTarget` (engine map; `thiscall`, `RET 4`: the
/// magic target): true when the target's slot +4 answers a `Character`
/// (`__RTDynamicCast` from `TESObjectREFR`) whose slot +0x22C (with 0)
/// answers true, `00608d80` answers false and `004f8960` is not 6.
pub fn reanimate_effect_check_target(e: &mut Engine, _this: Ptr, target: u32) -> u8 {
    let reference = e.vcall(target, 4, &[]).u32();
    let character = e
        .call(
            DYNAMIC_CAST,
            &args![reference, 0u32, TYPE_TES_OBJECT_REFR, TYPE_CHARACTER, 0u32],
        )
        .u32();
    if character != 0
        && e.vcall(character, 0x22c, &args![0u32]).bool()
        && !e.call(ACTOR_TEST_00608D80, &args![character]).bool()
        && e.call(ACTOR_VALUE_004F8960, &args![character]).u32() != 6
    {
        return 1;
    }
    0
}

// Translated from 008273c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, three words: caster, magic item, effect item): a new
/// 0x70-byte `ReanimateEffect` (`00401000`, then `008265b0`), or 0 when the
/// allocation fails.
// Translation incomplete: 008273c0 - the FS:[0] exception frame (freeing the block when the constructor throws) is not translated
pub fn fn_008273c0(e: &mut Engine, caster: u32, spell: u32, effect: u32) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x70u32]).u32();
    if block != 0 {
        return fn_008265b0(e, Ptr::new(block), caster, spell, effect).addr();
    }
    0
}

// Translated from 00827440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ReanimateEffect::GetSaveSize` (engine map; `thiscall`, `RET 4`):
/// `ActiveEffect::GetSaveSize` plus 4, 4, 0xC and 0x10 (a 16-bit sum).
pub fn reanimate_effect_get_save_size(e: &mut Engine, this: Ptr, argument: u32) -> u16 {
    let size = e
        .call(ACTIVE_EFFECT_GET_SAVE_SIZE, &args![this, argument])
        .u16();
    size.wrapping_add(4)
        .wrapping_add(4)
        .wrapping_add(0xc)
        .wrapping_add(0x10)
}

/// The four members `SaveGame` and `LoadGame` write or read: `iState`,
/// `fSaveTime`, `StartPos`, `StartRot`.
const SAVED_MEMBERS: [(u32, u32); 4] = [(0x4c, 4), (0x50, 4), (START_POS, 0xc), (START_ROT, 0x10)];

// Translated from 008274a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ReanimateEffect::SaveGame` (engine map; `thiscall`, `RET 4`):
/// `ActiveEffect::SaveGame`, then `iState`, `fSaveTime`, `StartPos` and
/// `StartRot` written (`008579b0` on the save/load game object).
pub fn reanimate_effect_save_game(e: &mut Engine, this: Ptr, argument: u32) {
    e.call(ACTIVE_EFFECT_SAVE_GAME, &args![this, argument]);
    for (offset, size) in SAVED_MEMBERS {
        let save_load = e.mem.u32(SAVE_LOAD_GAME);
        e.call(
            SAVE_WRITE,
            &args![save_load, this.addr().wrapping_add(offset), size],
        );
    }
}

// Translated from 00827510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ReanimateEffect::LoadGame` (engine map; `thiscall`, `RET 4`):
/// `ActiveEffect::LoadGame`, then `iState`, `fSaveTime`, `StartPos` and
/// `StartRot` read (`008579e0` on the save/load game object).
pub fn reanimate_effect_load_game(e: &mut Engine, this: Ptr, argument: u32) {
    e.call(ACTIVE_EFFECT_LOAD_GAME, &args![this, argument]);
    for (offset, size) in SAVED_MEMBERS {
        let save_load = e.mem.u32(SAVE_LOAD_GAME);
        e.call(
            LOAD_READ,
            &args![save_load, this.addr().wrapping_add(offset), size],
        );
    }
}

// Translated from 00827580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ReanimateEffect::FinishInitLoadGame` (engine map; `thiscall`, `RET 4`:
/// the actor): `ActiveEffect::FinishInitLoadGame`, then, when the actor
/// has 3D (slot +0x1D0): its face animation's slot +0xD8 with (1, 1)
/// before the waking state (0x28) and (0, 0) from it,
/// `bhkWorld::SetMotion` (1, 1, 0, 1) before the standing state (0x1E),
/// and the `"Bip01 Spine2"` bone's body as `pBody` (slot +0xE4 with 4); a
/// missing bone prints the message at `0107c110`.
// Translation incomplete: 00827580 - the FS:[0] exception frame (destroying the fixed string on a throw) is not translated
pub fn reanimate_effect_finish_init_load_game(
    e: &mut Engine,
    this: Ptr<ReanimateEffect>,
    actor: u32,
) {
    e.call(ACTIVE_EFFECT_FINISH_INIT_LOAD_GAME, &args![this, actor]);
    let node = e.vcall(actor, 0x1d0, &[]).u32();
    if node == 0 {
        return;
    }
    let face = e.call(GET_FACE_ANIMATION_DATA, &args![actor]).u32();
    if face != 0 {
        if e.get(this, ReanimateEffect::iState) < STATE_WAKING {
            e.vcall(face, 0xd8, &args![1u32, 1u32]);
        } else {
            e.vcall(face, 0xd8, &args![0u32, 0u32]);
        }
    }
    if e.get(this, ReanimateEffect::iState) < STATE_STANDING {
        e.call(SET_MOTION, &args![node, 1u32, 1u32, 0u32, 1u32]);
    }
    let bone = spine_bone(e, node);
    if bone == 0 {
        e.call(PRINT_005B5E40, &args![NO_BONE_LOAD_MESSAGE]);
        return;
    }
    take_body(e, this, bone);
}

// Translated from 008276e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ReanimateEffect::Revert` (engine map; `thiscall`, `RET 4`: the
/// reference): `004534f0`, then, before the commanded state (0x32), the
/// body's slot +0xE4 with 1 when the reference has 3D (slot +0x1D0) and
/// there is a body.
pub fn reanimate_effect_revert(e: &mut Engine, this: Ptr<ReanimateEffect>, reference: u32) {
    e.call(REVERT_BASE_004534F0, &args![this, reference]);
    if e.get(this, ReanimateEffect::iState) < STATE_COMMANDED
        && reference != 0
        && e.vcall(reference, 0x1d0, &[]).u32() != 0
        && e.get(this, ReanimateEffect::pBody) != 0
    {
        let body = e.get(this, ReanimateEffect::pBody);
        e.vcall(body, 0xe4, &args![1u32]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0082_6b40, fn_00826b40(Ptr, bool)),
        entry!(0x0082_6b90, fn_00826b90(Ptr, bool)),
        entry!(
            0x0082_65b0,
            fn_008265b0(Ptr<ReanimateEffect>, u32, u32, u32) -> Ptr<ReanimateEffect>
        ),
        entry!(
            0x0082_66a0,
            reanimate_effect_copy(Ptr<ReanimateEffect>) -> u32
        ),
        entry!(0x0082_67b0, reanimate_effect_start(Ptr<ReanimateEffect>)),
        entry!(
            0x0082_6be0,
            reanimate_effect_update(Ptr<ReanimateEffect>, f32)
        ),
        entry!(0x0082_7200, fn_00827200(Ptr, u32) -> Ptr),
        entry!(0x0082_7230, reanimate_effect_finish(Ptr<ReanimateEffect>)),
        entry!(0x0082_7340, reanimate_effect_check_target(Ptr, u32) -> u8),
        entry!(0x0082_73c0, fn_008273c0(u32, u32, u32) -> u32),
        entry!(0x0082_7440, reanimate_effect_get_save_size(Ptr, u32) -> u16),
        entry!(0x0082_74a0, reanimate_effect_save_game(Ptr, u32)),
        entry!(0x0082_7510, reanimate_effect_load_game(Ptr, u32)),
        entry!(
            0x0082_7580,
            reanimate_effect_finish_init_load_game(Ptr<ReanimateEffect>, u32)
        ),
        entry!(
            0x0082_76e0,
            reanimate_effect_revert(Ptr<ReanimateEffect>, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(address: u32, bit: u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x1c, 0x8000_0001);
        e.call(address, &[this, 1]);
        assert_eq!(e.mem.u32(this + 0x1c), 0x8000_0001 | bit);
        // Only the low byte of the flag word counts.
        e.call(address, &[this, 0x100]);
        assert_eq!(e.mem.u32(this + 0x1c), 0x8000_0001);
        e.mem.set_u32(this + 0x1c, 0xffff_ffff);
        e.call(address, &[this, 0]);
        assert_eq!(e.mem.u32(this + 0x1c), !bit);
    }

    #[test]
    fn fn_00826b40_sets_or_clears_bit_value_2() {
        check(0x0082_6b40, 2);
    }

    #[test]
    fn fn_00826b90_sets_or_clears_bit_value_4() {
        check(0x0082_6b90, 4);
    }
}

#[cfg(test)]
mod tests_n048 {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn zero(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    fn run(e: &mut Engine, addr: u32, words: &[u32]) -> (Ret, Log) {
        e.call_log = Some(vec![]);
        let r = e.call(addr, words);
        (r, e.call_log.take().unwrap())
    }

    /// An object whose vtable sends each listed slot to `code + slot`, a
    /// double answering the paired word.
    fn object(e: &mut Engine, code: u32, slots: &[(u32, u32)]) -> u32 {
        let size = slots.iter().map(|s| s.0).max().unwrap_or(0) + 4;
        let vtable = e.mem.alloc(size);
        for &(slot, answer) in slots {
            e.mem.set_u32(vtable + slot, code + slot);
            e.register_double(code + slot, move |_, _| ret(answer));
        }
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object, vtable);
        object
    }

    const ACTOR: u32 = 0x0300_0000;
    const CASTER: u32 = 0x0310_0000;
    const PROCESS: u32 = 0x0320_0000;
    const BODY: u32 = 0x0330_0000;
    const NODE: u32 = 0x0340_0000;
    const VALUES: u32 = 0x0350_0000;
    const FACE: u32 = 0x0360_0000;
    const REFERENCE: u32 = 0x0370_0000;

    #[test]
    fn constructor_clears_the_state() {
        let mut e = Engine::new();
        zero(&mut e, &[ACTIVE_EFFECT_CONSTRUCT]);
        e.register(SELF_006815C0, |_, a| ret(a[0]));
        e.register(SET_POINT_00416870, |e, a| {
            for i in 0..3 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            ret(a[0])
        });
        e.map(IDENTITY_ROTATION, 16);
        for i in 0..4 {
            e.mem.set_u32(IDENTITY_ROTATION + 4 * i, 0x100 + i);
        }
        let this = e.new_object::<ReanimateEffect>();
        e.mem.write(this.addr(), &[0xffu8; 0x70]);
        let (r, log) = run(&mut e, 0x0082_65b0, &args![this, 1u32, 2u32, 3u32]);
        assert_eq!(r.u32(), this.addr());
        assert_eq!(
            calls(&log, ACTIVE_EFFECT_CONSTRUCT),
            vec![vec![this.addr(), 1, 2, 3]]
        );
        assert_eq!(e.mem.u32(this.addr()), REANIMATE_EFFECT_VTABLE);
        assert_eq!(e.get(this, ReanimateEffect::pBody), 0);
        assert_eq!(e.get(this, ReanimateEffect::iState), 0);
        assert_eq!(e.get(this, ReanimateEffect::fSaveTime), 0);
        for i in 0..3 {
            assert_eq!(e.mem.u32(this.addr() + START_POS + 4 * i), 0);
        }
        for i in 0..4 {
            assert_eq!(e.mem.u32(this.addr() + START_ROT + 4 * i), 0x100 + i);
        }
    }

    #[test]
    fn copy_and_maker_copy_the_state() {
        let mut e = Engine::new();
        zero(&mut e, &[ACTIVE_EFFECT_CONSTRUCT, SELF_006815C0]);
        e.register(SET_POINT_00416870, |_, a| ret(a[0]));
        e.map(IDENTITY_ROTATION, 16);
        let block = e.mem.alloc(0x70);
        e.register_double(OPERATOR_NEW, move |_, _| ret(block));
        let this = object(&mut e, 0x0380_0000, &[(0x44, 0)]);
        e.mem.set_u32(this + 0x28, 0xca);
        e.mem.set_u32(this + 8, 0x5b);
        e.mem.set_u32(this + 0xc, 0xef);
        for i in 0..10 {
            e.mem.set_u32(this + 0x48 + 4 * i, 0x200 + i);
        }
        e.mem.set_u32(this + 0x50, 0x7f80_0001);
        let (r, log) = run(&mut e, 0x0082_66a0, &args![this]);
        assert_eq!(r.u32(), block);
        assert_eq!(calls(&log, OPERATOR_NEW), vec![vec![0x70]]);
        assert_eq!(
            calls(&log, ACTIVE_EFFECT_CONSTRUCT),
            vec![vec![block, 0xca, 0x5b, 0xef]]
        );
        assert_eq!(calls(&log, 0x0380_0044), vec![vec![this, block]]);
        for i in 0..10 {
            let expected = if i == 2 { 0x7fc0_0001 } else { 0x200 + i };
            assert_eq!(e.mem.u32(block + 0x48 + 4 * i), expected);
        }
        let (r, log) = run(&mut e, 0x0082_73c0, &args![4u32, 5u32, 6u32]);
        assert_eq!(r.u32(), block);
        assert_eq!(
            calls(&log, ACTIVE_EFFECT_CONSTRUCT),
            vec![vec![block, 4, 5, 6]]
        );
        e.register(OPERATOR_NEW, |_, _| ret(0));
        assert_eq!(e.call(0x0082_73c0, &args![4u32, 5u32, 6u32]).u32(), 0);
    }

    /// The effect, its target actor (`ACTOR` slots), its caster (`CASTER`
    /// slots), the actor's process (`PROCESS`), 3D (`NODE`) and body.
    struct World {
        effect: Ptr<ReanimateEffect>,
        actor: u32,
        caster: u32,
        process: u32,
        node: u32,
        body: u32,
    }

    fn world(e: &mut Engine, knocked: u32, with_3d: bool) -> World {
        let node = object(e, NODE, &[(0x9c, 0x0b0e)]);
        let shown = if with_3d { node } else { 0 };
        let actor = object(
            e,
            ACTOR,
            &[
                (0xd4, 0),
                (0x22c, knocked),
                (0x434, 0),
                (0x2c0, 0),
                (0x1d0, shown),
                (0x1c4, 0),
                (0x3ac, 0),
                (0x2f4, 0),
                (0x1e4, 0xb1),
            ],
        );
        // The actor value owner is embedded at +0xA4.
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(actor + 0xa4, vtable);
        e.mem.set_u32(vtable, VALUES);
        e.mem.set_u32(vtable + 0xc, VALUES + 0xc);
        e.register(VALUES, |_, _| ret(100));
        e.register(VALUES + 0xc, |_, _| Ret {
            st0: 60.0,
            ..Ret::default()
        });
        let caster = object(e, CASTER, &[(0x2c, 0x4e4e), (0x1f4, 0)]);
        let process = object(e, PROCESS, &[(0x6f8, 0), (0x628, 0), (0x644, 0)]);
        let body = object(e, BODY, &[(0xe4, 0)]);
        let effect = e.new_object::<ReanimateEffect>();
        let this = effect.addr();
        let target = e.mem.alloc(4);
        e.register_double(EFFECT_TARGET_0059BB30, move |_, _| ret(target));
        e.register_double(GET_TARGET_AS_ACTOR, move |_, _| ret(actor));
        e.register_double(EFFECT_CASTER_0045CD60, move |_, a| {
            ret(if a[0] == this { caster } else { 0 })
        });
        e.register_double(GET_CASTER_AS_ACTOR, move |_, _| ret(caster));
        e.register_double(PROCESS_OF, move |_, _| ret(process));
        zero(
            e,
            &[
                DISPEL,
                SET_LIFE_STATE,
                SET_GHOST,
                ACTOR_TEST_00493BB0,
                SET_LOCATION_ON_REFERENCE,
                ACTOR_00826941_008A6840,
                INIT_ANIMATION,
                INIT_LIGHTING_PROPERTY_PTR,
                SET_PACK_TYPE,
                PACKAGE_SET_00984F60,
                PACKAGE_SET_0067A3A0,
                SET_MOTION,
                FIXED_STRING_DESTRUCT,
                GET_POSITION,
                GET_ROTATION,
                PRINT_005B5E40,
            ],
        );
        e.register(POSITION_0045BB80, |_, a| ret(a[0] + 0x30));
        let package = e.mem.alloc(0x20);
        e.register_double(CREATE_PACKAGE, move |_, _| ret(package));
        e.register(FIXED_STRING_CONSTRUCT, |_, a| ret(a[0]));
        e.register(GET_COLLISION_OBJECT, |_, a| ret(a[0] + 1));
        e.register_double(COLLISION_BODY_006FA820, move |_, _| ret(body));
        World {
            effect,
            actor,
            caster,
            process,
            node,
            body,
        }
    }

    #[test]
    fn start_dispels_without_caster_or_knocked_target() {
        let mut e = Engine::new();
        let w = world(&mut e, 0, true);
        let this = w.effect.addr();
        let (_, log) = run(&mut e, 0x0082_67b0, &args![this]);
        assert_eq!(calls(&log, DISPEL), vec![vec![this, 0]]);
        e.register(EFFECT_CASTER_0045CD60, |_, _| ret(0));
        let (_, log) = run(&mut e, 0x0082_67b0, &args![this]);
        assert_eq!(calls(&log, DISPEL), vec![vec![this, 1]]);
    }

    #[test]
    fn start_raises_the_body_and_records_its_start() {
        let mut e = Engine::new();
        let w = world(&mut e, 1, true);
        let this = w.effect.addr();
        let (_, log) = run(&mut e, 0x0082_67b0, &args![this]);
        assert!(calls(&log, DISPEL).is_empty());
        assert_eq!(calls(&log, ACTOR + 0xd4), vec![vec![w.actor, 0]]);
        assert_eq!(calls(&log, SET_LIFE_STATE), vec![vec![w.actor, 4]]);
        assert!(calls(&log, ACTOR + 0x434).is_empty());
        assert_eq!(calls(&log, PROCESS + 0x6f8), vec![vec![w.process, 1]]);
        assert_eq!(calls(&log, SET_GHOST), vec![vec![w.actor, 1]]);
        assert_eq!(calls(&log, ACTOR + 0x2c0), vec![vec![w.actor, 0x4e4e]]);
        assert_eq!(
            calls(&log, SET_LOCATION_ON_REFERENCE),
            vec![vec![w.actor, w.node + 0x30]]
        );
        assert_eq!(calls(&log, PROCESS + 0x628), vec![vec![w.process, 0]]);
        // 100 - 60 = 40.
        assert_eq!(
            calls(&log, ACTOR + 0x3ac),
            vec![vec![w.actor, 0x10, 40.0f32.to_bits()]]
        );
        assert_eq!(calls(&log, CREATE_PACKAGE), vec![vec![0x1b]]);
        let package = calls(&log, SET_PACK_TYPE)[0][0];
        assert_eq!(e.mem.u32(package + 0x1c) & 6, 6);
        assert_eq!(
            calls(&log, ACTOR + 0x2f4),
            vec![vec![w.actor, package, 1, 1]]
        );
        assert_eq!(calls(&log, SET_MOTION), vec![vec![w.node, 1, 1, 0, 1]]);
        assert_eq!(e.get(w.effect, ReanimateEffect::pBody), w.body);
        assert_eq!(calls(&log, BODY + 0xe4), vec![vec![w.body, 4]]);
        assert_eq!(calls(&log, GET_POSITION), vec![vec![w.body, this + 0x54]]);
        assert_eq!(calls(&log, GET_ROTATION), vec![vec![w.body, this + 0x60]]);
        assert!(calls(&log, PRINT_005B5E40).is_empty());
        // No bone: the message.
        e.register(NODE + 0x9c, |_, _| ret(0));
        let (_, log) = run(&mut e, 0x0082_67b0, &args![this]);
        assert_eq!(calls(&log, PRINT_005B5E40), vec![vec![NO_BONE_MESSAGE]]);
        assert!(calls(&log, GET_POSITION).is_empty());
    }

    #[test]
    fn start_without_3d_stops_after_the_package() {
        let mut e = Engine::new();
        let w = world(&mut e, 1, false);
        e.register(ACTOR_TEST_00493BB0, |_, _| ret(1));
        let (_, log) = run(&mut e, 0x0082_67b0, &args![w.effect]);
        assert_eq!(calls(&log, ACTOR + 0x434), vec![vec![w.actor, 0]]);
        assert!(calls(&log, SET_LOCATION_ON_REFERENCE).is_empty());
        assert!(calls(&log, INIT_ANIMATION).is_empty());
        assert_eq!(calls(&log, ACTOR + 0x2f4).len(), 1);
        assert!(calls(&log, SET_MOTION).is_empty());
    }

    fn update_rig(e: &mut Engine) {
        zero(
            e,
            &[
                SELF_006815C0,
                COPY_POINT_004A3E00,
                SET_FRACTION_005DBF20,
                LERP_005DBF70,
                SUBTRACT_00439EF0,
                FROM_EULER_ANGLES_ZYX,
                CONSTRUCT_0056DF00,
                CONVERT_0056DFF0,
                QUATERNION_CONSTRUCT_006240D0,
                CONVERT_00561500,
                QUATERNION_SET,
                QUATERNION_SET_SLERP,
                BODY_MOVE_00C8DE20,
                RELOAD_TARGETS,
                PLAY_GROUP,
                RAGDOLL_00C9BDF0,
                SET_RAGDOLL_DATA,
                SWAP_EYES,
                INIT_COMMANDED_ACTOR,
                COMMANDED_UPDATE_0080B8F0,
            ],
        );
        for (addr, value) in [
            (RISE_TIME, 5.0f64),
            (RISE_DIVISOR, 4.0),
            (STAND_TIME, 2.0),
            (ONE_DOUBLE, 1.0),
        ] {
            e.map(addr, 8);
            e.set_global::<f64>(addr, value);
        }
        e.map(RISE_HEIGHT, 4);
        e.set_global::<u32>(RISE_HEIGHT, 100.0f32.to_bits());
        e.map(HALF, 4);
        e.set_global::<u32>(HALF, 0.5f32.to_bits());
        e.map(ANGLE_011DCB54, 4);
        e.map(EYES_011CB600, 4);
        e.mem.set_u32(EYES_011CB600, 0xe7e5);
        e.register(MIN_0040EBD0, |_, a| {
            let (x, y) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
            Ret {
                st0: f64::from(if x < y { x } else { y }),
                ..Ret::default()
            }
        });
        let coordinate = e.mem.alloc(4);
        e.mem.set_u32(coordinate, 1.5f32.to_bits());
        e.register_double(COORDINATE_00560D30, move |_, _| ret(coordinate));
        e.register(SCALE_004A3E90, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])),
            ..Ret::default()
        });
        e.register(ATAN2, |_, _| Ret {
            st0: 0.25,
            ..Ret::default()
        });
        e.register(EXTRA_LIST_005D43C0, |_, a| ret(a[0] + 0x40));
    }

    #[test]
    fn update_dispels_or_waits() {
        let mut e = Engine::new();
        let w = world(&mut e, 0, true);
        update_rig(&mut e);
        let this = w.effect.addr();
        // No body yet: nothing.
        let (_, log) = run(&mut e, 0x0082_6be0, &args![this, 0.5f32.to_bits()]);
        assert!(calls(&log, DISPEL).is_empty());
        assert_eq!(e.get(w.effect, ReanimateEffect::iState), 0);
        e.set(w.effect, ReanimateEffect::pBody, w.body);
        e.register(ACTOR + 0x22c, |_, _| ret(1));
        let (_, log) = run(&mut e, 0x0082_6be0, &args![this, 0.5f32.to_bits()]);
        assert_eq!(calls(&log, DISPEL), vec![vec![this, 0]]);
        e.register(GET_CASTER_AS_ACTOR, |_, _| ret(0));
        let (_, log) = run(&mut e, 0x0082_6be0, &args![this, 0.5f32.to_bits()]);
        assert_eq!(calls(&log, DISPEL), vec![vec![this, 1]]);
    }

    #[test]
    fn update_rises_towards_the_caster() {
        let mut e = Engine::new();
        let w = world(&mut e, 0, true);
        update_rig(&mut e);
        let this = w.effect.addr();
        e.set(w.effect, ReanimateEffect::pBody, w.body);
        e.set(w.effect, ReanimateEffect::fSaveTime, 7.0f32.to_bits());
        e.set(w.effect, ReanimateEffect::fElapsedSeconds, 2.0f32.to_bits());
        let (_, log) = run(&mut e, 0x0082_6be0, &args![this, 0.5f32.to_bits()]);
        // State 0 starts the rise at once, `fSaveTime` 0.
        assert_eq!(e.get(w.effect, ReanimateEffect::iState), STATE_RISING);
        assert_eq!(e.get(w.effect, ReanimateEffect::fSaveTime), 0);
        assert_eq!(calls(&log, PROCESS + 0x6f8), vec![vec![w.process, 1]]);
        // min(1, 2 / 4) = 0.5.
        assert_eq!(
            calls(&log, MIN_0040EBD0),
            vec![vec![1.0f32.to_bits(), 0.5f32.to_bits()]]
        );
        assert_eq!(calls(&log, SET_FRACTION_005DBF20)[0][1], 0.5f32.to_bits());
        assert_eq!(calls(&log, COORDINATE_00560D30)[0][1], 2);
        assert_eq!(calls(&log, SCALE_004A3E90), vec![vec![100.0f32.to_bits()]]);
        assert_eq!(calls(&log, CASTER + 0x1f4), vec![vec![w.caster]]);
        let euler = calls(&log, FROM_EULER_ANGLES_ZYX);
        assert_eq!(euler[0][1..], [(-0.25f32).to_bits(), 0.5f32.to_bits(), 0]);
        assert_eq!(calls(&log, QUATERNION_SET_SLERP)[0][3], 0.5f32.to_bits());
        let moved = calls(&log, BODY_MOVE_00C8DE20);
        assert_eq!(moved.len(), 1);
        assert_eq!(moved[0][0], w.body);
        assert_eq!(moved[0][3], 2.0f32.to_bits());
        assert_eq!(moved[0][4], 1.0f32.to_bits());
        assert_eq!(calls(&log, ACTOR + 0x2c0), vec![vec![w.actor, 0x4e4e]]);
        // More than 5 seconds: the rag doll state.
        e.set(w.effect, ReanimateEffect::fElapsedSeconds, 5.5f32.to_bits());
        let (_, log) = run(&mut e, 0x0082_6be0, &args![this, 0.5f32.to_bits()]);
        assert_eq!(e.get(w.effect, ReanimateEffect::iState), STATE_RAGDOLL);
        assert_eq!(
            calls(&log, MIN_0040EBD0),
            vec![vec![1.0f32.to_bits(), 1.375f32.to_bits()]]
        );
    }

    #[test]
    fn update_steps_through_the_later_states() {
        let mut e = Engine::new();
        let w = world(&mut e, 0, true);
        update_rig(&mut e);
        let this = w.effect.addr();
        e.set(w.effect, ReanimateEffect::pBody, w.body);
        e.set(w.effect, ReanimateEffect::iState, STATE_RAGDOLL);
        e.set(w.effect, ReanimateEffect::fElapsedSeconds, 3.0f32.to_bits());
        let (_, log) = run(&mut e, 0x0082_6be0, &args![this, 0.5f32.to_bits()]);
        assert_eq!(e.get(w.effect, ReanimateEffect::iState), STATE_STANDING);
        assert_eq!(calls(&log, RELOAD_TARGETS), vec![vec![w.actor, 1]]);
        assert_eq!(
            calls(&log, PLAY_GROUP),
            vec![vec![0xb1, 0, 0, 0xffff_ffff, 0xffff_ffff]]
        );
        assert_eq!(calls(&log, RAGDOLL_00C9BDF0), vec![vec![w.node, 0]]);
        assert_eq!(calls(&log, PROCESS + 0x6f8), vec![vec![w.process, 0]]);
        assert_eq!(calls(&log, SET_RAGDOLL_DATA), vec![vec![w.actor + 0x40, 0]]);
        assert_eq!(
            e.get(w.effect, ReanimateEffect::fSaveTime),
            3.0f32.to_bits()
        );
        // Standing: not yet after 2 seconds exactly.
        e.set(w.effect, ReanimateEffect::fElapsedSeconds, 5.0f32.to_bits());
        let (_, log) = run(&mut e, 0x0082_6be0, &args![this, 0.5f32.to_bits()]);
        assert_eq!(e.get(w.effect, ReanimateEffect::iState), STATE_STANDING);
        assert!(calls(&log, SET_MOTION).is_empty());
        e.set(w.effect, ReanimateEffect::fElapsedSeconds, 5.5f32.to_bits());
        let npc = e.mem.alloc(4);
        e.register(BASE_FORM_007AF430, |_, a| ret(a[0] + 8));
        e.register_double(DYNAMIC_CAST, move |_, _| ret(npc));
        let face = object(&mut e, FACE, &[(0xd8, 0)]);
        e.register_double(GET_FACE_ANIMATION_DATA, move |_, _| ret(face));
        let (_, log) = run(&mut e, 0x0082_6be0, &args![this, 0.5f32.to_bits()]);
        assert_eq!(e.get(w.effect, ReanimateEffect::iState), STATE_WAKING);
        assert_eq!(
            e.get(w.effect, ReanimateEffect::fSaveTime),
            5.5f32.to_bits()
        );
        assert_eq!(calls(&log, SET_MOTION), vec![vec![w.node, 4, 1, 0, 1]]);
        assert_eq!(
            calls(&log, DYNAMIC_CAST),
            vec![vec![w.actor + 8, 0, TYPE_TES_BOUND_OBJECT, TYPE_TES_NPC, 0]]
        );
        assert_eq!(calls(&log, SWAP_EYES), vec![vec![npc, 0xe7e5]]);
        assert_eq!(calls(&log, FACE + 0xd8), vec![vec![face, 0, 0]]);
        // Waking: more than one second later, commanded.
        e.set(
            w.effect,
            ReanimateEffect::fElapsedSeconds,
            6.75f32.to_bits(),
        );
        let (_, log) = run(&mut e, 0x0082_6be0, &args![this, 0.5f32.to_bits()]);
        assert_eq!(e.get(w.effect, ReanimateEffect::iState), STATE_COMMANDED);
        assert_eq!(calls(&log, SET_LIFE_STATE), vec![vec![w.actor, 4]]);
        assert_eq!(calls(&log, SET_GHOST), vec![vec![w.actor, 0]]);
        assert_eq!(
            calls(&log, INIT_COMMANDED_ACTOR),
            vec![vec![w.actor, w.caster]]
        );
        let (_, log) = run(&mut e, 0x0082_6be0, &args![this, 0.5f32.to_bits()]);
        assert_eq!(calls(&log, PROCESS + 0x644), vec![vec![w.process, 0]]);
        assert_eq!(
            calls(&log, COMMANDED_UPDATE_0080B8F0),
            vec![vec![w.actor, w.caster]]
        );
    }

    #[test]
    fn finish_kills_and_releases_the_actor() {
        let mut e = Engine::new();
        let w = world(&mut e, 1, true);
        zero(&mut e, &[KILL, CLEANUP_COMMANDED_ACTOR]);
        e.map(GAME_011C3F2C, 4);
        e.register(GAME_TEST_004226E0, |_, _| ret(0));
        let this = w.effect.addr();
        e.set(w.effect, ReanimateEffect::pBody, w.body);
        let (_, log) = run(&mut e, 0x0082_7230, &args![this]);
        assert_eq!(calls(&log, SET_GHOST), vec![vec![w.actor, 0]]);
        assert_eq!(calls(&log, BODY + 0xe4), vec![vec![w.body, 1]]);
        assert_eq!(calls(&log, KILL), vec![vec![w.actor, 0, 0]]);
        assert_eq!(
            calls(&log, CLEANUP_COMMANDED_ACTOR),
            vec![vec![w.actor, w.caster, 0]]
        );
        // Commanded and still knocked out: no kill.
        e.set(w.effect, ReanimateEffect::iState, STATE_COMMANDED);
        let (_, log) = run(&mut e, 0x0082_7230, &args![this]);
        assert!(calls(&log, KILL).is_empty());
        assert!(calls(&log, SET_GHOST).is_empty());
        e.register(ACTOR + 0x22c, |_, _| ret(0));
        let (_, log) = run(&mut e, 0x0082_7230, &args![this]);
        assert_eq!(calls(&log, KILL).len(), 1);
        e.register(GAME_TEST_004226E0, |_, _| ret(1));
        let (_, log) = run(&mut e, 0x0082_7230, &args![this]);
        assert!(calls(&log, KILL).is_empty());
        assert!(calls(&log, CLEANUP_COMMANDED_ACTOR).is_empty());
    }

    #[test]
    fn check_target_wants_a_knocked_out_character() {
        let mut e = Engine::new();
        let character = object(&mut e, ACTOR, &[(0x22c, 1)]);
        let target = object(&mut e, REFERENCE, &[(4, 0x4ef)]);
        e.register_double(DYNAMIC_CAST, move |_, _| ret(character));
        e.register(ACTOR_TEST_00608D80, |_, _| ret(0));
        e.register(ACTOR_VALUE_004F8960, |_, _| ret(5));
        let (r, log) = run(&mut e, 0x0082_7340, &args![0u32, target]);
        assert_eq!(r.u32(), 1);
        assert_eq!(
            calls(&log, DYNAMIC_CAST),
            vec![vec![0x4ef, 0, TYPE_TES_OBJECT_REFR, TYPE_CHARACTER, 0]]
        );
        e.register(ACTOR_VALUE_004F8960, |_, _| ret(6));
        assert_eq!(e.call(0x0082_7340, &args![0u32, target]).u32(), 0);
        e.register(ACTOR_VALUE_004F8960, |_, _| ret(5));
        e.register(ACTOR_TEST_00608D80, |_, _| ret(1));
        assert_eq!(e.call(0x0082_7340, &args![0u32, target]).u32(), 0);
        e.register(DYNAMIC_CAST, |_, _| ret(0));
        assert_eq!(e.call(0x0082_7340, &args![0u32, target]).u32(), 0);
    }

    #[test]
    fn save_size_save_and_load() {
        let mut e = Engine::new();
        e.register(ACTIVE_EFFECT_GET_SAVE_SIZE, |_, _| ret(0x1_0040));
        let (r, _) = run(&mut e, 0x0082_7440, &args![0x10u32, 0x20u32]);
        assert_eq!(r.u32() & 0xffff, 0x40 + 0x24);
        e.register(ACTIVE_EFFECT_GET_SAVE_SIZE, |_, _| ret(0xfff0));
        assert_eq!(
            e.call(0x0082_7440, &args![0x10u32, 0x20u32]).u32() & 0xffff,
            0x14
        );
        zero(
            &mut e,
            &[
                ACTIVE_EFFECT_SAVE_GAME,
                ACTIVE_EFFECT_LOAD_GAME,
                SAVE_WRITE,
                LOAD_READ,
            ],
        );
        e.map(SAVE_LOAD_GAME, 4);
        e.mem.set_u32(SAVE_LOAD_GAME, 0x5a1d);
        let expected = vec![
            vec![0x5a1d, 0x104c, 4],
            vec![0x5a1d, 0x1050, 4],
            vec![0x5a1d, 0x1054, 0xc],
            vec![0x5a1d, 0x1060, 0x10],
        ];
        let (_, log) = run(&mut e, 0x0082_74a0, &args![0x1000u32, 0x77u32]);
        assert_eq!(
            calls(&log, ACTIVE_EFFECT_SAVE_GAME),
            vec![vec![0x1000, 0x77]]
        );
        assert_eq!(calls(&log, SAVE_WRITE), expected);
        let (_, log) = run(&mut e, 0x0082_7510, &args![0x1000u32, 0x77u32]);
        assert_eq!(
            calls(&log, ACTIVE_EFFECT_LOAD_GAME),
            vec![vec![0x1000, 0x77]]
        );
        assert_eq!(calls(&log, LOAD_READ), expected);
    }

    #[test]
    fn finish_init_load_game_takes_the_body_again() {
        let mut e = Engine::new();
        let w = world(&mut e, 1, true);
        zero(&mut e, &[ACTIVE_EFFECT_FINISH_INIT_LOAD_GAME]);
        let face = object(&mut e, FACE, &[(0xd8, 0)]);
        e.register_double(GET_FACE_ANIMATION_DATA, move |_, _| ret(face));
        let this = w.effect.addr();
        e.set(w.effect, ReanimateEffect::iState, STATE_RAGDOLL);
        let (_, log) = run(&mut e, 0x0082_7580, &args![this, w.actor]);
        assert_eq!(
            calls(&log, ACTIVE_EFFECT_FINISH_INIT_LOAD_GAME),
            vec![vec![this, w.actor]]
        );
        assert_eq!(calls(&log, FACE + 0xd8), vec![vec![face, 1, 1]]);
        assert_eq!(calls(&log, SET_MOTION), vec![vec![w.node, 1, 1, 0, 1]]);
        assert_eq!(e.get(w.effect, ReanimateEffect::pBody), w.body);
        assert_eq!(calls(&log, BODY + 0xe4), vec![vec![w.body, 4]]);
        e.set(w.effect, ReanimateEffect::iState, STATE_WAKING);
        let (_, log) = run(&mut e, 0x0082_7580, &args![this, w.actor]);
        assert_eq!(calls(&log, FACE + 0xd8), vec![vec![face, 0, 0]]);
        assert!(calls(&log, SET_MOTION).is_empty());
        e.register(NODE + 0x9c, |_, _| ret(0));
        let (_, log) = run(&mut e, 0x0082_7580, &args![this, w.actor]);
        assert_eq!(
            calls(&log, PRINT_005B5E40),
            vec![vec![NO_BONE_LOAD_MESSAGE]]
        );
        // Without 3D only the base runs.
        let w = world(&mut e, 1, false);
        let (_, log) = run(&mut e, 0x0082_7580, &args![w.effect, w.actor]);
        assert!(calls(&log, GET_FACE_ANIMATION_DATA).is_empty());
    }

    #[test]
    fn revert_releases_the_body_before_command() {
        let mut e = Engine::new();
        let w = world(&mut e, 1, true);
        zero(&mut e, &[REVERT_BASE_004534F0]);
        let this = w.effect.addr();
        e.set(w.effect, ReanimateEffect::pBody, w.body);
        let (_, log) = run(&mut e, 0x0082_76e0, &args![this, w.actor]);
        assert_eq!(calls(&log, REVERT_BASE_004534F0), vec![vec![this, w.actor]]);
        assert_eq!(calls(&log, BODY + 0xe4), vec![vec![w.body, 1]]);
        let (_, log) = run(&mut e, 0x0082_76e0, &args![this, 0u32]);
        assert!(calls(&log, BODY + 0xe4).is_empty());
        e.set(w.effect, ReanimateEffect::iState, STATE_COMMANDED);
        let (_, log) = run(&mut e, 0x0082_76e0, &args![this, w.actor]);
        assert!(calls(&log, BODY + 0xe4).is_empty());
    }

    #[test]
    fn quaternion_from_rotation() {
        let mut e = Engine::new();
        e.register(SELF_006815C0, |_, a| ret(a[0]));
        zero(&mut e, &[QUATERNION_SET]);
        let (r, log) = run(&mut e, 0x0082_7200, &args![0x40u32, 0x80u32]);
        assert_eq!(r.u32(), 0x40);
        assert_eq!(calls(&log, SELF_006815C0), vec![vec![0x40]]);
        assert_eq!(calls(&log, QUATERNION_SET), vec![vec![0x40, 0x80]]);
    }
}
