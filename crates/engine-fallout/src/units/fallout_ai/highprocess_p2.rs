//! `fallout/ai/highprocess.cpp` (Xbox PDB source unit), part 2: its functions from `008d8720` up to
//! (not including) `008d9780` in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The unit's shared layouts and helpers are in
//! [`super::highprocess`]; anything public there may be used here.
//!
//! Like the unit's first block, these are the process classes' one-field
//! accessors. The `MiddleHighProcess` ones use the Xbox PDB field names and
//! PC offsets (checked against the code); the doc comment of each names the
//! field and offset. Byte getters return the raw byte (`MOV AL`).

#[allow(unused_imports)]
use super::highprocess::*;
#[allow(unused_imports)]
use crate::prelude::*;

/// By-value removal from a `BSSimpleList` (`00905330`): `this` is the list
/// head, the argument the address of a word holding the item.
const LIST_REMOVE_ITEM: u32 = 0x0090_5330;
/// `NiPointer` getter (`00559450`): the first word of `this`.
const NI_POINTER_GET: u32 = 0x0055_9450;
/// `BSSoundHandle` assignment (`00418900`): `this` = destination, source.
const SOUND_HANDLE_ASSIGN: u32 = 0x0041_8900;
/// `BSSoundHandle` destructor (`00483710`).
const SOUND_HANDLE_DESTROY: u32 = 0x0048_3710;

/// Setting getter (`00403e20`): `this` = a game setting object; returns a pointer to its float.
const SETTING_FLOAT_POINTER: u32 = 0x0040_3e20;
/// The game setting object `SetDetectionModifierTimer` reads.
const DETECTION_MODIFIER_SETTING: u32 = 0x011c_d4fc;

// Translated from 008d8720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::SetIsSummonedCreature` (Xbox PDB) stores its argument in `bSummonedCreature` at +0x18b.
pub fn middle_high_process_set_is_summoned_creature(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x18b, value);
}

// Translated from 008d87e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetAll3DUpdateFlags` (Xbox PDB) returns `cUpdate3DModel` at +0x18c.
pub fn middle_high_process_get_all_3d_update_flags(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 396)
}

// Translated from 008d8800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetAttachedArrowList` (Xbox PDB) returns `pAttachedArrowList (BSSimpleList<ArrowProjectile *>*)` at +0x1ac.
pub fn middle_high_process_get_attached_arrow_list(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(this.addr() + 428))
}

// Translated from 008d8820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::SetIronSights` (Xbox PDB) stores its argument in `bIronSights` at +0x228.
pub fn middle_high_process_set_iron_sights(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x228, value);
}

// Translated from 008d8840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetIronSights` (Xbox PDB) returns `bIronSights` at +0x228.
pub fn middle_high_process_get_iron_sights(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 552)
}

// Translated from 008d8860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::IsSustainedFire` (Xbox PDB) returns `bSustainedFire` at +0x1d9.
pub fn middle_high_process_is_sustained_fire(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 473)
}

// Translated from 008d8880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::SetSustainedFire` (Xbox PDB) stores its argument in `bSustainedFire` at +0x1d9.
pub fn middle_high_process_set_sustained_fire(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x1d9, value);
}

// Translated from 008d88a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::SetBeginIdlesPlayed` (Xbox PDB) stores its argument in `bPlayedBeginIdles` at +0x19c.
pub fn middle_high_process_set_begin_idles_played(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x19c, value);
}

// Translated from 008d88c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetBeginIdlesPlayed` (Xbox PDB) returns `bPlayedBeginIdles` at +0x19c.
pub fn middle_high_process_get_begin_idles_played(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 412)
}

// Translated from 008d88e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::SetEndIdlesPlayed` (Xbox PDB) stores its argument in `bPlayedEndIdles` at +0x19d.
pub fn middle_high_process_set_end_idles_played(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x19d, value);
}

// Translated from 008d8900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetEndIdlesPlayed` (Xbox PDB) returns `bPlayedEndIdles` at +0x19d.
pub fn middle_high_process_get_end_idles_played(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 413)
}

// Translated from 008d8920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetCreatureLipSynchAnim` (Xbox PDB) returns `pCreatureLipsynchAnim (LipSynchAnim*)` at +0x1a0.
pub fn middle_high_process_get_creature_lip_synch_anim(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(this.addr() + 416))
}

// Translated from 008d8940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetCreatureLipSynchStartTime` (Xbox PDB) returns `iCreatureLipsynchStartTime` at +0x1a4.
pub fn middle_high_process_get_creature_lip_synch_start_time(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 420)
}

// Translated from 008d89c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetRadiationMagicDelta` (Xbox PDB) returns `fRadiationMagicDelta` at +0x238.
pub fn middle_high_process_get_radiation_magic_delta(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + 568)
}

// Translated from 008d89e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::SetRadiationWaterDelta` (Xbox PDB) stores its argument in `fRadiationWaterDelta` at +0x23c.
pub fn middle_high_process_set_radiation_water_delta(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_f32(this.addr() + 0x23c, x87_quiet(value));
}

// Translated from 008d8a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetRadiationWaterDelta` (Xbox PDB) returns `fRadiationWaterDelta` at +0x23c.
pub fn middle_high_process_get_radiation_water_delta(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + 572)
}

// Translated from 008d8a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::SetRadiationDelta` (Xbox PDB) stores its argument in `fRadiationDelta` at +0x234.
pub fn middle_high_process_set_radiation_delta(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_f32(this.addr() + 0x234, x87_quiet(value));
}

// Translated from 008d8a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetLastAttackHitData` (Xbox PDB) returns `pLastAttackHitData (HitData*)` at +0x254.
pub fn middle_high_process_get_last_attack_hit_data(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(this.addr() + 596))
}

// Translated from 008d8a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetWeaponConditionStage` (Xbox PDB) returns `iWeaponConditionStage` at +0x244.
pub fn middle_high_process_get_weapon_condition_stage(e: &mut Engine, this: Ptr) -> i32 {
    e.mem.u32(this.addr() + 580) as i32
}

// Translated from 008d8a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::SetWeaponConditionStage` (Xbox PDB) stores its argument in `iWeaponConditionStage` at +0x244.
pub fn middle_high_process_set_weapon_condition_stage(e: &mut Engine, this: Ptr, value: i32) {
    e.mem.set_u32(this.addr() + 0x244, value as u32);
}

// Translated from 008d8aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::SetFaceNode` (Xbox PDB) stores its argument in `pFaceNode (BSFaceGenNiNode*)` at +0x248.
pub fn middle_high_process_set_face_node(e: &mut Engine, this: Ptr, value: Ptr) {
    e.mem.set_u32(this.addr() + 0x248, value.addr());
}

// Translated from 008d8ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetFaceSkinnedNode` (Xbox PDB) returns `pFaceNodeSkinned (BSFaceGenNiNode*)` at +0x24c.
pub fn middle_high_process_get_face_skinned_node(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(this.addr() + 588))
}

// Translated from 008d8ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::SetFaceSkinnedNode` (Xbox PDB) stores its argument in `pFaceNodeSkinned (BSFaceGenNiNode*)` at +0x24c.
pub fn middle_high_process_set_face_skinned_node(e: &mut Engine, this: Ptr, value: Ptr) {
    e.mem.set_u32(this.addr() + 0x24c, value.addr());
}

// Translated from 008d8b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetHeadAnims` (Xbox PDB) returns `pAnimFace (NiAVObject*)` at +0x250.
pub fn middle_high_process_get_head_anims(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(this.addr() + 592))
}

// Translated from 008d8b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetCurrentFurniture` (Xbox PDB) returns `pCurrentFurniture (TESObjectREFR*)` at +0x140.
pub fn middle_high_process_get_current_furniture(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(this.addr() + 320))
}

// Translated from 008d8be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetCurrentFurnitureIndex` (Xbox PDB) returns `cCurrentFurnitureIndex` at +0x144.
pub fn middle_high_process_get_current_furniture_index(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 324)
}

// Translated from 008d8c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetTorsoNode` (Xbox PDB) returns `pTorsoNode (NiAVObject*)` at +0x21c.
pub fn middle_high_process_get_torso_node(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(this.addr() + 540))
}

// Translated from 008d8c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::SetAnimActionSuccess` (Xbox PDB) stores its argument in `iAnimActionSuccess` at +0x22a.
pub fn middle_high_process_set_anim_action_success(e: &mut Engine, this: Ptr, value: i16) {
    e.mem.set_u16(this.addr() + 0x22a, value as u16);
}

// Translated from 008d8740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::Set3DUpdateFlag` (Xbox PDB) ORs the mask into `cUpdate3DModel` at +0x18c.
pub fn middle_high_process_set_3d_update_flag(e: &mut Engine, this: Ptr, mask: u32) {
    let flags = e.mem.u8(this.addr() + 0x18c) as u32;
    e.mem.set_u8(this.addr() + 0x18c, (flags | mask) as u8);
}

// Translated from 008d8770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::Clear3DUpdateFlag` (Xbox PDB) updates `cUpdate3DModel` at +0x18c. The code
/// multiplies the flags by the complemented mask (`IMUL`, not `AND`) and stores the low byte;
/// reproduced as the code does it.
pub fn middle_high_process_clear_3d_update_flag(e: &mut Engine, this: Ptr, mask: u32) {
    let flags = e.mem.u8(this.addr() + 0x18c) as u32;
    e.mem
        .set_u8(this.addr() + 0x18c, flags.wrapping_mul(!mask) as u8);
}

// Translated from 008d87a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::ClearAll3DUpdateFlags` (Xbox PDB) zeroes `cUpdate3DModel` at +0x18c.
pub fn middle_high_process_clear_all_3d_update_flags(e: &mut Engine, this: Ptr) {
    e.mem.set_u8(this.addr() + 0x18c, 0);
}

// Translated from 008d87c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::Get3DUpdateFlag` (Xbox PDB): whether any bit of the mask is set in
/// `cUpdate3DModel` at +0x18c.
pub fn middle_high_process_get_3d_update_flag(e: &mut Engine, this: Ptr, mask: u32) -> bool {
    (e.mem.u8(this.addr() + 0x18c) as u32 & mask) != 0
}

// Translated from 008d8960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::AddRadiationMagicDelta` (Xbox PDB) adds its argument to
/// `fRadiationMagicDelta` at +0x238 (x87: computed in `f64`, stored as `f32`).
pub fn middle_high_process_add_radiation_magic_delta(e: &mut Engine, this: Ptr, delta: f32) {
    let current = e.mem.f32(this.addr() + 0x238) as f64;
    e.mem
        .set_f32(this.addr() + 0x238, (current + delta as f64) as f32);
}

// Translated from 008d8990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::RemoveRadiationMagicDelta` (Xbox PDB) subtracts its argument from
/// `fRadiationMagicDelta` at +0x238 (x87: computed in `f64`, stored as `f32`).
pub fn middle_high_process_remove_radiation_magic_delta(e: &mut Engine, this: Ptr, delta: f32) {
    let current = e.mem.f32(this.addr() + 0x238) as f64;
    e.mem
        .set_f32(this.addr() + 0x238, (current - delta as f64) as f32);
}

// Translated from 008d8b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetSitSleepState` (Xbox PDB): `cSitSleepState` at +0x13d, plus one when
/// virtual slot 0x618 reports bit 3 (0x8) and the state is 4 or 9.
pub fn middle_high_process_get_sit_sleep_state(e: &mut Engine, this: Ptr) -> u32 {
    let flags = e.vcall(this.addr(), 0x618, &args![]).u32();
    let state = e.mem.u8(this.addr() + 0x13d) as u32;
    if flags & 0x8 != 0 && (state == 4 || state == 9) {
        state + 1
    } else {
        state
    }
}

// Translated from 008d8ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::ClearCurrentFurniture` (Xbox PDB): removes the reference from the list
/// embedded at +0xd0 (`00905330`, by value) and, if it is `pCurrentFurniture` (+0x140), clears that.
pub fn middle_high_process_clear_current_furniture(e: &mut Engine, this: Ptr, furniture: Ptr) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), furniture.addr());
        e.call(LIST_REMOVE_ITEM, &args![this.byte_add(0xd0), slot]);
    });
    if e.mem.u32(this.addr() + 0x140) == furniture.addr() {
        e.mem.set_u32(this.addr() + 0x140, 0);
    }
}

// Translated from 008d8c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetLightingShaderProperty` (Xbox PDB): the
/// `NiPointer<BSShaderPPLightingProperty>` `pLightingProperty` at +0x220, read through the
/// pointer getter (`00559450`).
pub fn middle_high_process_get_lighting_shader_property(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(NI_POINTER_GET, &args![this.byte_add(0x220)]).ptr()
}

// Translated from 008d8c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::RemoveSpokenToActor` (Xbox PDB): removes the actor from the list
/// `pLastSpokeToList` (+0x264, `BSSimpleList<Actor *>*`) with the by-value list removal
/// (`00905330`).
pub fn high_process_remove_spoken_to_actor(e: &mut Engine, this: Ptr<HighProcess>, actor: Ptr) {
    let list = e.get(this, HighProcess::pLastSpokeToList);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), actor.addr());
        e.call(LIST_REMOVE_ITEM, &args![list, slot]);
    });
}

// Translated from 008d8c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetNumberGuardsArresting` (Xbox PDB) returns the word at +0x39c
/// (`iNumberGuardsPersuing` in the Xbox PDB layout).
pub fn high_process_get_number_guards_arresting(e: &mut Engine, this: Ptr<HighProcess>) -> u32 {
    e.get(this, HighProcess::iNumberGuardsPersuing)
}

// Translated from 008d8ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::ModNumberGuardsArresting` (Xbox PDB) adds its argument (wrapping) to the word at
/// +0x39c.
pub fn high_process_mod_number_guards_arresting(
    e: &mut Engine,
    this: Ptr<HighProcess>,
    delta: u32,
) {
    let count = e.get(this, HighProcess::iNumberGuardsPersuing);
    e.set(
        this,
        HighProcess::iNumberGuardsPersuing,
        count.wrapping_add(delta),
    );
}

// Translated from 008d8cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SetNumberGuardsArresting` (Xbox PDB) stores its argument in `iNumberGuardsPersuing` at +0x39c.
pub fn high_process_set_number_guards_arresting(e: &mut Engine, this: Ptr, value: i32) {
    e.mem.set_u32(this.addr() + 0x39c, value as u32);
}

// Translated from 008d8cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetAutomaticFireAtLeastOnce` (Xbox PDB) returns `bAutomaticFireAtLeastOne` at +0x2f4.
pub fn high_process_get_automatic_fire_at_least_once(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x2f4)
}

// Translated from 008d8d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SetAutomaticFireAtLeastOnce` (Xbox PDB) stores its argument in `bAutomaticFireAtLeastOne` at +0x2f4.
pub fn high_process_set_automatic_fire_at_least_once(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x2f4, value);
}

// Translated from 008d8da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::IsDoingSayTo` (Xbox PDB) returns `bIsDoingSayTo` at +0x459.
pub fn high_process_is_doing_say_to(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x459)
}

// Translated from 008d8dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SetDoingSayTo` (Xbox PDB) stores its argument in `bIsDoingSayTo` at +0x459.
pub fn high_process_set_doing_say_to(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x459, value);
}

// Translated from 008d8de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetLastSpeakingEmotion` (Xbox PDB) returns `eLastSpeakingEmotion (DIALOGUE_EMOTION)` at +0x36c.
pub fn high_process_get_last_speaking_emotion(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x36c)
}

// Translated from 008d8e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SetLastSpeakingEmotion` (Xbox PDB) stores its argument in `eLastSpeakingEmotion (DIALOGUE_EMOTION)` at +0x36c.
pub fn high_process_set_last_speaking_emotion(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0x36c, value);
}

// Translated from 008d8e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetLastGreeted` (Xbox PDB) returns `pGreetActor (TESObjectREFR*)` at +0x30c.
pub fn high_process_get_last_greeted(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(this.addr() + 0x30c))
}

// Translated from 008d8e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SetLastGreeted` (Xbox PDB) stores its argument in `pGreetActor (TESObjectREFR*)` at +0x30c.
pub fn high_process_set_last_greeted(e: &mut Engine, this: Ptr, value: Ptr) {
    e.mem.set_u32(this.addr() + 0x30c, value.addr());
}

// Translated from 008d8f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetGreetingFlag` (Xbox PDB) returns `bGreetingFlag` at +0x32c.
pub fn high_process_get_greeting_flag(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x32c)
}

// Translated from 008d8f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SetGreetingFlag` (Xbox PDB) stores its argument in `bGreetingFlag` at +0x32c.
pub fn high_process_set_greeting_flag(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x32c, value);
}

// Translated from 008d8f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetSoundDelay` (Xbox PDB) returns `fSoundDelay` at +0x310.
pub fn high_process_get_sound_delay(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + 0x310)
}

// Translated from 008d8f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SetSoundDelay` (Xbox PDB) stores its argument in `fSoundDelay` at +0x310.
pub fn high_process_set_sound_delay(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_f32(this.addr() + 0x310, x87_quiet(value));
}

// Translated from 008d8f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetGreetingTimer` (Xbox PDB) returns `fGreetingTimer` at +0x330.
pub fn high_process_get_greeting_timer(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + 0x330)
}

// Translated from 008d8fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SetGreetingTimer` (Xbox PDB) stores its argument in `fGreetingTimer` at +0x330.
pub fn high_process_set_greeting_timer(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_f32(this.addr() + 0x330, x87_quiet(value));
}

// Translated from 008d8fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetDetectAlert` (Xbox PDB) returns `bWeaponAlertDrawn` at +0x349.
pub fn high_process_get_detect_alert(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x349)
}

// Translated from 008d8ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SetDetectAlert` (Xbox PDB) stores its argument in `bWeaponAlertDrawn` at +0x349.
pub fn high_process_set_detect_alert(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x349, value);
}

// Translated from 008d9010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetIdleTimer` (Xbox PDB) returns `fIdleTimer` at +0x334.
pub fn high_process_get_idle_timer(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + 0x334)
}

// Translated from 008d9030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SetIdleTimer` (Xbox PDB) stores its argument in `fIdleTimer` at +0x334.
pub fn high_process_set_idle_timer(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_f32(this.addr() + 0x334, x87_quiet(value));
}

// Translated from 008d9050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::ContinuingPackageforPC` (Xbox PDB) returns `bContinuingPackageforPC` at +0x374.
pub fn high_process_continuing_packagefor_pc(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x374)
}

// Translated from 008d9070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SetContinuingPackage` (Xbox PDB) stores its argument in `bContinuingPackageforPC` at +0x374.
pub fn high_process_set_continuing_package(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x374, value);
}

// Translated from 008d9090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetAwarePlayerTimer` (Xbox PDB) returns `fAwarePlayerTimer` at +0x34c.
pub fn high_process_get_aware_player_timer(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + 0x34c)
}

// Translated from 008d90e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SetBreathTimer` (Xbox PDB) stores its argument in `fBreathTimer` at +0x33c.
pub fn high_process_set_breath_timer(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_f32(this.addr() + 0x33c, x87_quiet(value));
}

// Translated from 008d9100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetBreathTimer` (Xbox PDB) returns `fBreathTimer` at +0x33c.
pub fn high_process_get_breath_timer(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + 0x33c)
}

// Translated from 008d9120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SetPackageEndTimerValue` (Xbox PDB) stores its argument in `fScriptPackageEndTime` at +0x378.
pub fn high_process_set_package_end_timer_value(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_f32(this.addr() + 0x378, x87_quiet(value));
}

// Translated from 008d9140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetPackageEndTimerValue` (Xbox PDB) returns `fScriptPackageEndTime` at +0x378.
pub fn high_process_get_package_end_timer_value(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + 0x378)
}

// Translated from 008d9160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SetAnimationActiveFlag` (Xbox PDB) stores its argument in `bActivateAnim` at +0x375.
pub fn high_process_set_animation_active_flag(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x375, value);
}

// Translated from 008d9180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetMovementStoped` (Xbox PDB) returns `bStop` at +0x3a0.
pub fn high_process_get_movement_stoped(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x3a0)
}

// Translated from 008d91c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SetCurrentlyReanimating` (Xbox PDB) stores its argument in `bCurrentlyReanimating` at +0x3b9.
pub fn high_process_set_currently_reanimating(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x3b9, value);
}

// Translated from 008d91e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetCurrentlyReanimating` (Xbox PDB) returns `bCurrentlyReanimating` at +0x3b9.
pub fn high_process_get_currently_reanimating(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x3b9)
}

// Translated from 008d9200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetWaitingforLipFile` (Xbox PDB) returns `bWaitingForLipFile` at +0x3d0.
pub fn high_process_get_waitingfor_lip_file(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x3d0)
}

// Translated from 008d9220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SetWaitingforLipFile` (Xbox PDB) stores its argument in `bWaitingForLipFile` at +0x3d0.
pub fn high_process_set_waitingfor_lip_file(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x3d0, value);
}

// Translated from 008d9240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetLipAnim` (Xbox PDB) returns `pLipSynicAnim (LipSynchAnim*)` at +0x3cc.
pub fn high_process_get_lip_anim(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(this.addr() + 0x3cc))
}

// Translated from 008d8d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SaveWeaponLastPos` (Xbox PDB) stores the three coordinates in `WeaponLastPos (NiPoint3)` at +0x300.
pub fn high_process_save_weapon_last_pos(e: &mut Engine, this: Ptr, x: f32, y: f32, z: f32) {
    e.mem.set_f32(this.addr() + 0x300, x);
    e.mem.set_f32(this.addr() + 0x304, y);
    e.mem.set_f32(this.addr() + 0x308, z);
}

// Translated from 008d8d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetWeaponLastPos` (Xbox PDB) returns the address of `WeaponLastPos (NiPoint3)` at +0x300.
pub fn high_process_get_weapon_last_pos(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr() + 0x300)
}

// Translated from 008d8d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::ResetSearchChatterTimer` (Xbox PDB) sets `fIdleChatterTimer` at +0x298 to zero.
pub fn high_process_reset_search_chatter_timer(e: &mut Engine, this: Ptr) {
    e.mem.set_f32(this.addr() + 0x298, 0.0);
}

// Translated from 008d8e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetSoundHandle` (Xbox PDB) copies the `index`th `BSSoundHandle` of
/// `SoundHandle` (12-byte elements from +0x314) into `result` with the handle assignment
/// (`00418900`) and returns `result`.
pub fn high_process_get_sound_handle(e: &mut Engine, this: Ptr, result: Ptr, index: u32) -> Ptr {
    let slot = this
        .addr()
        .wrapping_add(index.wrapping_mul(0xc))
        .wrapping_add(0x314);
    e.call(SOUND_HANDLE_ASSIGN, &args![result, slot]);
    result
}

// Translated from 008d8ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SetSoundHandle` (Xbox PDB) assigns the by-value `BSSoundHandle` (three
/// words) to the `index`th element of `SoundHandle` (12-byte elements from +0x314) with the
/// handle assignment (`00418900`), then destroys the argument copy (`00483710`).
/// C++ exception unwinding is not translated.
pub fn high_process_set_sound_handle(
    e: &mut Engine,
    this: Ptr,
    index: u32,
    handle_0: u32,
    handle_1: u32,
    handle_2: u32,
) {
    let slot = this
        .addr()
        .wrapping_add(index.wrapping_mul(0xc))
        .wrapping_add(0x314);
    e.with_stack(12, |e, copy| {
        e.mem.set_u32(copy.addr(), handle_0);
        e.mem.set_u32(copy.addr() + 4, handle_1);
        e.mem.set_u32(copy.addr() + 8, handle_2);
        e.call(SOUND_HANDLE_ASSIGN, &args![slot, copy]);
        e.call(SOUND_HANDLE_DESTROY, &args![copy]);
    });
}

// Translated from 008d90b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::ModAwarePlayerTimer` (Xbox PDB) adds `delta` to `fAwarePlayerTimer` at +0x34c.
pub fn high_process_mod_aware_player_timer(e: &mut Engine, this: Ptr, delta: f32) {
    let timer = e.mem.f32(this.addr() + 0x34c);
    e.mem.set_f32(
        this.addr() + 0x34c,
        (f64::from(timer) + f64::from(delta)) as f32,
    );
}

// Translated from 008d91a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::ClearMovementStoped` (Xbox PDB) clears `bStop` at +0x3a0.
pub fn high_process_clear_movement_stoped(e: &mut Engine, this: Ptr) {
    e.mem.set_u8(this.addr() + 0x3a0, 0);
}

// Translated from 008d9260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SetLipAnim` (Xbox PDB) stores its argument in `pLipSynicAnim (LipSynchAnim*)` at +0x3cc.
pub fn high_process_set_lip_anim(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0x3cc, value);
}

// Translated from 008d9280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SetLipFileFailed` (Xbox PDB) stores its argument in `bLipFileFailed` at +0x3d1.
pub fn high_process_set_lip_file_failed(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x3d1, value);
}

// Translated from 008d92a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetLipFileFailed` (Xbox PDB) returns `bLipFileFailed` at +0x3d1.
pub fn high_process_get_lip_file_failed(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x3d1)
}

// Translated from 008d92c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetLipQuequed` (Xbox PDB) returns `bLipQuequed` at +0x348.
pub fn high_process_get_lip_quequed(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x348)
}

// Translated from 008d92e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SetLipQuequed` (Xbox PDB) stores its argument in `bLipQuequed` at +0x348.
pub fn high_process_set_lip_quequed(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x348, value);
}

// Translated from 008d9300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetCurrentMuzzleFlash` (Xbox PDB) returns `pCurrentMuzzleFlash (MuzzleFlash*)` at +0x3d4.
pub fn high_process_get_current_muzzle_flash(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(this.addr() + 0x3d4))
}

// Translated from 008d9320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetSocialTalkTimer` (Xbox PDB) returns `fCheckToTalkTimer` at +0x2c8.
pub fn high_process_get_social_talk_timer(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + 0x2c8)
}

// Translated from 008d9340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SetSocialTalkTimer` (Xbox PDB) stores its argument in `fCheckToTalkTimer` at +0x2c8.
pub fn high_process_set_social_talk_timer(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_f32(this.addr() + 0x2c8, x87_quiet(value));
}

// Translated from 008d9390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetActorsDetectionEvent` (Xbox PDB) returns `pActorsGeneratedDetectionEvent (DetectionEvent*)` at +0x3dc. The one stack argument is never read.
pub fn high_process_get_actors_detection_event(e: &mut Engine, this: Ptr, _unused_1: u32) -> Ptr {
    Ptr::new(e.mem.u32(this.addr() + 0x3dc))
}

// Translated from 008d93b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetAggroRadiusActorList` (Xbox PDB) returns the address of `AggroRadiusList (BSSimpleList<Actor *>)` at +0x38c.
pub fn high_process_get_aggro_radius_actor_list(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr() + 0x38c)
}

// Translated from 008d93d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetAvoidActorList` (Xbox PDB) returns the address of `AvoidActorList (BSSimpleList<Actor *>)` at +0x394.
pub fn high_process_get_avoid_actor_list(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr() + 0x394)
}

// Translated from 008d93f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetDetectionTimer` (Xbox PDB) returns `fDetectionTimer` at +0x2f8.
pub fn high_process_get_detection_timer(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + 0x2f8)
}

// Translated from 008d9410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SetCurrentProcessIdle` (Xbox PDB) stores its argument in `pIdleToPlay (TESIdleForm*)` at +0x350.
pub fn high_process_set_current_process_idle(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0x350, value);
}

// Translated from 008d9430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetFinishingCombatPackage` (Xbox PDB) returns `bFinishingCombatPackage` at +0x3e0.
pub fn high_process_get_finishing_combat_package(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x3e0)
}

// Translated from 008d9450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SetFinishingCombatPackage` (Xbox PDB) stores its argument in `bFinishingCombatPackage` at +0x3e0.
pub fn high_process_set_finishing_combat_package(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x3e0, value);
}

// Translated from 008d9490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SetDetectionTimer` (Xbox PDB) stores its argument in `fDetectionTimer` at +0x2f8.
pub fn high_process_set_detection_timer(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_f32(this.addr() + 0x2f8, x87_quiet(value));
}

// Translated from 008d94b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetActorLightLevel` (Xbox PDB) returns `fLightLevel` at +0x3c4.
pub fn high_process_get_actor_light_level(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + 0x3c4)
}

// Translated from 008d94d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SetActorLightLevel` (Xbox PDB) stores its argument in `fLightLevel` at +0x3c4.
pub fn high_process_set_actor_light_level(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_f32(this.addr() + 0x3c4, x87_quiet(value));
}

// Translated from 008d94f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetLightLevelTimer` (Xbox PDB) returns `fLightLevelTimer` at +0x3c8.
pub fn high_process_get_light_level_timer(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + 0x3c8)
}

// Translated from 008d9510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SetLightLevelTimer` (Xbox PDB) stores its argument in `fLightLevelTimer` at +0x3c8.
pub fn high_process_set_light_level_timer(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_f32(this.addr() + 0x3c8, x87_quiet(value));
}

// Translated from 008d9530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetTalkingtoPC` (Xbox PDB) returns `bDialoguewithPlayer` at +0x364.
pub fn high_process_get_talkingto_pc(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x364)
}

// Translated from 008d9550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SetTalkingtoPC` (Xbox PDB) stores its argument in `bDialoguewithPlayer` at +0x364.
pub fn high_process_set_talkingto_pc(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x364, value);
}

// Translated from 008d9590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetDetectionModifier` (Xbox PDB) returns `fDetectionModifer` at +0x3bc.
pub fn high_process_get_detection_modifier(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + 0x3bc)
}

// Translated from 008d95b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetDetectionModifierTimer` (Xbox PDB) returns `fDetectionModifierTimer` at +0x3c0.
pub fn high_process_get_detection_modifier_timer(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + 0x3c0)
}

// Translated from 008d9610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetFadeState` (Xbox PDB) returns `eFadeState (BaseProcess::FADE_STATE)` at +0x3e8.
pub fn high_process_get_fade_state(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x3e8)
}

// Translated from 008d9630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SetNeedTalkPlayer` (Xbox PDB) stores its argument in `bNeedTalkPlayer` at +0x445.
pub fn high_process_set_need_talk_player(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x445, value);
}

// Translated from 008d9650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetNeedTalkPlayer` (Xbox PDB) returns `bNeedTalkPlayer` at +0x445.
pub fn high_process_get_need_talk_player(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x445)
}

// Translated from 008d9670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetSubtitleItem` (Xbox PDB) returns `pSubtitleVoice (DialogueItem*)` at +0x454.
pub fn high_process_get_subtitle_item(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(this.addr() + 0x454))
}

// Translated from 008d9690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SetSubtitleItem` (Xbox PDB) stores its argument in `pSubtitleVoice (DialogueItem*)` at +0x454.
pub fn high_process_set_subtitle_item(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0x454, value);
}

// Translated from 008d9700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SetPlantedExplosive` (Xbox PDB) stores its argument in `bPlantedExplosive` at +0x444.
pub fn high_process_set_planted_explosive(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x444, value);
}

// Translated from 008d9720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetPlantedExplosive` (Xbox PDB) returns `bPlantedExplosive` at +0x444.
pub fn high_process_get_planted_explosive(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x444)
}

// Translated from 008d9740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetHeadTrackChangeTimer` (Xbox PDB) returns `fHeadTrackTargetTimer` at +0x418.
pub fn high_process_get_head_track_change_timer(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + 0x418)
}

// Translated from 008d9760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SetHeadTrackChangeTimer` (Xbox PDB) stores its argument in `fHeadTrackTargetTimer` at +0x418.
pub fn high_process_set_head_track_change_timer(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_f32(this.addr() + 0x418, x87_quiet(value));
}

// Translated from 008d9360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::AttackCallback` (Xbox PDB name from the map): adds its argument to the signed
/// 16-bit counter `sShotsFired` at +0x2c0 (sign-extended read, low half stored back).
pub fn high_process_attack_callback(e: &mut Engine, this: Ptr, count: i32) {
    let fired = e.mem.u16(this.addr() + 0x2c0) as i16 as i32;
    e.mem
        .set_u16(this.addr() + 0x2c0, fired.wrapping_add(count) as u16);
}

// Translated from 008d9470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::ResetSearchTimer` (Xbox PDB) clears `fEvaluateAcquireTimer` at +0x2e0.
pub fn high_process_reset_search_timer(e: &mut Engine, this: Ptr) {
    e.mem.set_f32(this.addr() + 0x2e0, 0.0);
}

// Translated from 008d9570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::SetDetectionModifierTimer` (Xbox PDB): stores in `fDetectionModifierTimer`
/// (+0x3c0) the float of the game setting at `011cd4fc`, read through the setting getter
/// (`00403e20`, which returns a pointer to the float).
pub fn high_process_set_detection_modifier_timer(e: &mut Engine, this: Ptr) {
    let setting = e
        .call(SETTING_FLOAT_POINTER, &args![DETECTION_MODIFIER_SETTING])
        .u32();
    let value = e.mem.f32(setting);
    e.mem.set_f32(this.addr() + 0x3c0, value);
}

// Translated from 008d95d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetCurrentProcessAnimIdle` (Xbox PDB): `spAnimIdleToPlay (NiPointer<AnimIdle>)`
/// at +0x354, read through the pointer getter (`00559450`).
pub fn high_process_get_current_process_anim_idle(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(NI_POINTER_GET, &args![this.byte_add(0x354)]).ptr()
}

// Translated from 008d95f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetCurrentProcessAnimIdleKF` (Xbox PDB): `spAnimIdleKF (NiPointer<KFModel>)` at
/// +0x35c, read through the pointer getter (`00559450`).
pub fn high_process_get_current_process_anim_idle_kf(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(NI_POINTER_GET, &args![this.byte_add(0x35c)]).ptr()
}

// Translated from 008d96b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::ClearPostAnimationActions` (Xbox PDB) clears `ePostAnimActon` at +0x424.
pub fn high_process_clear_post_animation_actions(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr() + 0x424, 0);
}

// Translated from 008d96d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::RemovePostAnimationAction` (Xbox PDB) clears the bits of `action` in
/// `ePostAnimActon` at +0x424.
pub fn high_process_remove_post_animation_action(e: &mut Engine, this: Ptr, action: u32) {
    let current = e.mem.u32(this.addr() + 0x424);
    e.mem.set_u32(this.addr() + 0x424, !action & current);
}

/// This part's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x008d8740, middle_high_process_set_3d_update_flag(Ptr, u32)),
        entry!(
            0x008d8770,
            middle_high_process_clear_3d_update_flag(Ptr, u32)
        ),
        entry!(
            0x008d87a0,
            middle_high_process_clear_all_3d_update_flags(Ptr)
        ),
        entry!(0x008d87c0, middle_high_process_get_3d_update_flag(Ptr, u32) -> bool),
        entry!(
            0x008d8960,
            middle_high_process_add_radiation_magic_delta(Ptr, f32)
        ),
        entry!(
            0x008d8990,
            middle_high_process_remove_radiation_magic_delta(Ptr, f32)
        ),
        entry!(0x008d8b20, middle_high_process_get_sit_sleep_state(Ptr) -> u32),
        entry!(
            0x008d8ba0,
            middle_high_process_clear_current_furniture(Ptr, Ptr)
        ),
        entry!(0x008d8c00, middle_high_process_get_lighting_shader_property(Ptr) -> Ptr),
        entry!(
            0x008d8c60,
            high_process_remove_spoken_to_actor(Ptr<HighProcess>, Ptr)
        ),
        entry!(
            0x008d8c80,
            high_process_get_number_guards_arresting(Ptr<HighProcess>) -> u32
        ),
        entry!(
            0x008d8ca0,
            high_process_mod_number_guards_arresting(Ptr<HighProcess>, u32)
        ),
        entry!(
            0x008d8720,
            middle_high_process_set_is_summoned_creature(Ptr, u8)
        ),
        entry!(0x008d87e0, middle_high_process_get_all_3d_update_flags(Ptr) -> u8),
        entry!(0x008d8800, middle_high_process_get_attached_arrow_list(Ptr) -> Ptr),
        entry!(0x008d8820, middle_high_process_set_iron_sights(Ptr, u8)),
        entry!(0x008d8840, middle_high_process_get_iron_sights(Ptr) -> u8),
        entry!(0x008d8860, middle_high_process_is_sustained_fire(Ptr) -> u8),
        entry!(0x008d8880, middle_high_process_set_sustained_fire(Ptr, u8)),
        entry!(
            0x008d88a0,
            middle_high_process_set_begin_idles_played(Ptr, u8)
        ),
        entry!(0x008d88c0, middle_high_process_get_begin_idles_played(Ptr) -> u8),
        entry!(
            0x008d88e0,
            middle_high_process_set_end_idles_played(Ptr, u8)
        ),
        entry!(0x008d8900, middle_high_process_get_end_idles_played(Ptr) -> u8),
        entry!(0x008d8920, middle_high_process_get_creature_lip_synch_anim(Ptr) -> Ptr),
        entry!(0x008d8940, middle_high_process_get_creature_lip_synch_start_time(Ptr) -> u32),
        entry!(0x008d89c0, middle_high_process_get_radiation_magic_delta(Ptr) -> f32),
        entry!(
            0x008d89e0,
            middle_high_process_set_radiation_water_delta(Ptr, f32)
        ),
        entry!(0x008d8a00, middle_high_process_get_radiation_water_delta(Ptr) -> f32),
        entry!(
            0x008d8a20,
            middle_high_process_set_radiation_delta(Ptr, f32)
        ),
        entry!(0x008d8a40, middle_high_process_get_last_attack_hit_data(Ptr) -> Ptr),
        entry!(0x008d8a60, middle_high_process_get_weapon_condition_stage(Ptr) -> i32),
        entry!(
            0x008d8a80,
            middle_high_process_set_weapon_condition_stage(Ptr, i32)
        ),
        entry!(0x008d8aa0, middle_high_process_set_face_node(Ptr, Ptr)),
        entry!(0x008d8ac0, middle_high_process_get_face_skinned_node(Ptr) -> Ptr),
        entry!(
            0x008d8ae0,
            middle_high_process_set_face_skinned_node(Ptr, Ptr)
        ),
        entry!(0x008d8b00, middle_high_process_get_head_anims(Ptr) -> Ptr),
        entry!(0x008d8b80, middle_high_process_get_current_furniture(Ptr) -> Ptr),
        entry!(0x008d8be0, middle_high_process_get_current_furniture_index(Ptr) -> u8),
        entry!(0x008d8c20, middle_high_process_get_torso_node(Ptr) -> Ptr),
        entry!(
            0x008d8c40,
            middle_high_process_set_anim_action_success(Ptr, i16)
        ),
        entry!(
            0x008d8cd0,
            high_process_set_number_guards_arresting(Ptr, i32)
        ),
        entry!(0x008d8cf0, high_process_get_automatic_fire_at_least_once(Ptr) -> u8),
        entry!(
            0x008d8d10,
            high_process_set_automatic_fire_at_least_once(Ptr, u8)
        ),
        entry!(0x008d8da0, high_process_is_doing_say_to(Ptr) -> u8),
        entry!(0x008d8dc0, high_process_set_doing_say_to(Ptr, u8)),
        entry!(0x008d8de0, high_process_get_last_speaking_emotion(Ptr) -> u32),
        entry!(0x008d8e00, high_process_set_last_speaking_emotion(Ptr, u32)),
        entry!(0x008d8e20, high_process_get_last_greeted(Ptr) -> Ptr),
        entry!(0x008d8e40, high_process_set_last_greeted(Ptr, Ptr)),
        entry!(0x008d8f10, high_process_get_greeting_flag(Ptr) -> u8),
        entry!(0x008d8f30, high_process_set_greeting_flag(Ptr, u8)),
        entry!(0x008d8f50, high_process_get_sound_delay(Ptr) -> f32),
        entry!(0x008d8f70, high_process_set_sound_delay(Ptr, f32)),
        entry!(0x008d8f90, high_process_get_greeting_timer(Ptr) -> f32),
        entry!(0x008d8fb0, high_process_set_greeting_timer(Ptr, f32)),
        entry!(0x008d8fd0, high_process_get_detect_alert(Ptr) -> u8),
        entry!(0x008d8ff0, high_process_set_detect_alert(Ptr, u8)),
        entry!(0x008d9010, high_process_get_idle_timer(Ptr) -> f32),
        entry!(0x008d9030, high_process_set_idle_timer(Ptr, f32)),
        entry!(0x008d9050, high_process_continuing_packagefor_pc(Ptr) -> u8),
        entry!(0x008d9070, high_process_set_continuing_package(Ptr, u8)),
        entry!(0x008d9090, high_process_get_aware_player_timer(Ptr) -> f32),
        entry!(0x008d90e0, high_process_set_breath_timer(Ptr, f32)),
        entry!(0x008d9100, high_process_get_breath_timer(Ptr) -> f32),
        entry!(
            0x008d9120,
            high_process_set_package_end_timer_value(Ptr, f32)
        ),
        entry!(0x008d9140, high_process_get_package_end_timer_value(Ptr) -> f32),
        entry!(0x008d9160, high_process_set_animation_active_flag(Ptr, u8)),
        entry!(0x008d9180, high_process_get_movement_stoped(Ptr) -> u8),
        entry!(0x008d91c0, high_process_set_currently_reanimating(Ptr, u8)),
        entry!(0x008d91e0, high_process_get_currently_reanimating(Ptr) -> u8),
        entry!(0x008d9200, high_process_get_waitingfor_lip_file(Ptr) -> u8),
        entry!(0x008d9220, high_process_set_waitingfor_lip_file(Ptr, u8)),
        entry!(0x008d9240, high_process_get_lip_anim(Ptr) -> Ptr),
        entry!(
            0x008d8d30,
            high_process_save_weapon_last_pos(Ptr, f32, f32, f32)
        ),
        entry!(0x008d8d60, high_process_get_weapon_last_pos(Ptr) -> Ptr),
        entry!(0x008d8d80, high_process_reset_search_chatter_timer(Ptr)),
        entry!(0x008d8e60, high_process_get_sound_handle(Ptr, Ptr, u32) -> Ptr),
        entry!(
            0x008d8ea0,
            high_process_set_sound_handle(Ptr, u32, u32, u32, u32)
        ),
        entry!(0x008d90b0, high_process_mod_aware_player_timer(Ptr, f32)),
        entry!(0x008d91a0, high_process_clear_movement_stoped(Ptr)),
        entry!(0x008d9260, high_process_set_lip_anim(Ptr, u32)),
        entry!(0x008d9280, high_process_set_lip_file_failed(Ptr, u8)),
        entry!(0x008d92a0, high_process_get_lip_file_failed(Ptr) -> u8),
        entry!(0x008d92c0, high_process_get_lip_quequed(Ptr) -> u8),
        entry!(0x008d92e0, high_process_set_lip_quequed(Ptr, u8)),
        entry!(0x008d9300, high_process_get_current_muzzle_flash(Ptr) -> Ptr),
        entry!(0x008d9320, high_process_get_social_talk_timer(Ptr) -> f32),
        entry!(0x008d9340, high_process_set_social_talk_timer(Ptr, f32)),
        entry!(0x008d9390, high_process_get_actors_detection_event(Ptr, u32) -> Ptr),
        entry!(0x008d93b0, high_process_get_aggro_radius_actor_list(Ptr) -> Ptr),
        entry!(0x008d93d0, high_process_get_avoid_actor_list(Ptr) -> Ptr),
        entry!(0x008d93f0, high_process_get_detection_timer(Ptr) -> f32),
        entry!(0x008d9410, high_process_set_current_process_idle(Ptr, u32)),
        entry!(0x008d9430, high_process_get_finishing_combat_package(Ptr) -> u8),
        entry!(
            0x008d9450,
            high_process_set_finishing_combat_package(Ptr, u8)
        ),
        entry!(0x008d9490, high_process_set_detection_timer(Ptr, f32)),
        entry!(0x008d94b0, high_process_get_actor_light_level(Ptr) -> f32),
        entry!(0x008d94d0, high_process_set_actor_light_level(Ptr, f32)),
        entry!(0x008d94f0, high_process_get_light_level_timer(Ptr) -> f32),
        entry!(0x008d9510, high_process_set_light_level_timer(Ptr, f32)),
        entry!(0x008d9530, high_process_get_talkingto_pc(Ptr) -> u8),
        entry!(0x008d9550, high_process_set_talkingto_pc(Ptr, u8)),
        entry!(0x008d9590, high_process_get_detection_modifier(Ptr) -> f32),
        entry!(0x008d95b0, high_process_get_detection_modifier_timer(Ptr) -> f32),
        entry!(0x008d9610, high_process_get_fade_state(Ptr) -> u32),
        entry!(0x008d9630, high_process_set_need_talk_player(Ptr, u8)),
        entry!(0x008d9650, high_process_get_need_talk_player(Ptr) -> u8),
        entry!(0x008d9670, high_process_get_subtitle_item(Ptr) -> Ptr),
        entry!(0x008d9690, high_process_set_subtitle_item(Ptr, u32)),
        entry!(0x008d9700, high_process_set_planted_explosive(Ptr, u8)),
        entry!(0x008d9720, high_process_get_planted_explosive(Ptr) -> u8),
        entry!(0x008d9740, high_process_get_head_track_change_timer(Ptr) -> f32),
        entry!(
            0x008d9760,
            high_process_set_head_track_change_timer(Ptr, f32)
        ),
        entry!(0x008d9360, high_process_attack_callback(Ptr, i32)),
        entry!(0x008d9470, high_process_reset_search_timer(Ptr)),
        entry!(0x008d9570, high_process_set_detection_modifier_timer(Ptr)),
        entry!(0x008d95d0, high_process_get_current_process_anim_idle(Ptr) -> Ptr),
        entry!(0x008d95f0, high_process_get_current_process_anim_idle_kf(Ptr) -> Ptr),
        entry!(0x008d96b0, high_process_clear_post_animation_actions(Ptr)),
        entry!(
            0x008d96d0,
            high_process_remove_post_animation_action(Ptr, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    fn process(e: &mut Engine) -> Ptr {
        Ptr::new(e.mem.alloc(0x500))
    }

    /// A word getter returns the word at `offset`.
    fn check_getter(addr: u32, offset: u32, value: u32) {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.mem.set_u32(this.addr() + offset, value);
        assert_eq!(e.call(addr, &args![this]).u32(), value);
    }

    /// A one-byte getter returns the byte at `offset` (and not its neighbour).
    fn check_byte_getter(addr: u32, offset: u32) {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.mem.set_u8(this.addr() + offset, 0x5a);
        e.mem.set_u8(this.addr() + offset + 1, 0xff);
        assert_eq!(e.call(addr, &args![this]).u8(), 0x5a);
    }

    /// A word setter stores its argument at `offset` and nothing around it.
    fn check_setter(addr: u32, offset: u32, value: u32) {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.call(addr, &args![this, value]);
        assert_eq!(e.mem.u32(this.addr() + offset), value);
        assert_eq!(e.mem.u32(this.addr() + offset + 4), 0);
        assert_eq!(e.mem.u32(this.addr() + offset - 4), 0);
    }

    /// A 16-bit setter stores the low half of the word.
    fn check_word16_setter(addr: u32, offset: u32) {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.call(addr, &args![this, -2i16]);
        assert_eq!(e.mem.u16(this.addr() + offset), 0xfffe);
        assert_eq!(e.mem.u16(this.addr() + offset + 2), 0);
    }

    /// A byte setter stores one byte at `offset`.
    fn check_byte_setter(addr: u32, offset: u32) {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.call(addr, &args![this, 0x78u8]);
        assert_eq!(e.mem.u8(this.addr() + offset), 0x78);
        assert_eq!(e.mem.u8(this.addr() + offset + 1), 0);
        assert_eq!(e.mem.u8(this.addr() + offset - 1), 0);
    }

    fn check_float_getter(addr: u32, offset: u32) {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.mem.set_f32(this.addr() + offset, 2.5);
        assert_eq!(e.call(addr, &args![this]).f32(), 2.5);
    }

    fn check_float_setter(addr: u32, offset: u32) {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.call(addr, &args![this, 1.25f32]);
        assert_eq!(e.mem.f32(this.addr() + offset), 1.25);
    }

    /// Doubles the by-value list removal, recording (list, item) per call.
    fn record_list_removals(e: &mut Engine) -> Rc<RefCell<Vec<(u32, u32)>>> {
        let seen: Rc<RefCell<Vec<(u32, u32)>>> = Rc::new(RefCell::new(vec![]));
        let log = seen.clone();
        e.register_double(LIST_REMOVE_ITEM, move |e, a| {
            log.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        seen
    }

    #[test]
    fn test_middle_high_process_set_3d_update_flag() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.mem.set_u8(this.addr() + 0x18c, 0x01);
        e.call(0x008d8740, &args![this, 0x04u32]);
        assert_eq!(e.mem.u8(this.addr() + 0x18c), 0x05);
        assert_eq!(e.mem.u8(this.addr() + 0x18d), 0);
    }

    #[test]
    fn test_middle_high_process_clear_3d_update_flag() {
        let mut e = Engine::new();
        let this = process(&mut e);
        // The code multiplies by the complemented mask: 1 * !1 = 0xfffffffe.
        e.mem.set_u8(this.addr() + 0x18c, 0x01);
        e.call(0x008d8770, &args![this, 0x01u32]);
        assert_eq!(e.mem.u8(this.addr() + 0x18c), 0xfe);
        e.mem.set_u8(this.addr() + 0x18c, 0x00);
        e.call(0x008d8770, &args![this, 0x01u32]);
        assert_eq!(e.mem.u8(this.addr() + 0x18c), 0x00);
        e.mem.set_u8(this.addr() + 0x18c, 0x03);
        e.call(0x008d8770, &args![this, 0xffff_ffffu32]);
        assert_eq!(e.mem.u8(this.addr() + 0x18c), 0x00, "x * !(-1) = 0");
    }

    #[test]
    fn test_middle_high_process_clear_all_3d_update_flags() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.mem.set_u8(this.addr() + 0x18c, 0xff);
        e.mem.set_u8(this.addr() + 0x18d, 0xff);
        e.call(0x008d87a0, &args![this]);
        assert_eq!(e.mem.u8(this.addr() + 0x18c), 0);
        assert_eq!(e.mem.u8(this.addr() + 0x18d), 0xff);
    }

    #[test]
    fn test_middle_high_process_get_3d_update_flag() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.mem.set_u8(this.addr() + 0x18c, 0x06);
        assert!(e.call(0x008d87c0, &args![this, 0x04u32]).bool());
        assert!(!e.call(0x008d87c0, &args![this, 0x01u32]).bool());
        assert!(!e.call(0x008d87c0, &args![this, 0x100u32]).bool());
    }

    #[test]
    fn test_middle_high_process_add_radiation_magic_delta() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.mem.set_f32(this.addr() + 0x238, 2.5);
        e.call(0x008d8960, &args![this, 1.25f32]);
        assert_eq!(e.mem.f32(this.addr() + 0x238), 3.75);
        assert_eq!(e.mem.f32(this.addr() + 0x23c), 0.0);
    }

    #[test]
    fn test_middle_high_process_remove_radiation_magic_delta() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.mem.set_f32(this.addr() + 0x238, 2.5);
        e.call(0x008d8990, &args![this, 1.25f32]);
        assert_eq!(e.mem.f32(this.addr() + 0x238), 1.25);
        assert_eq!(e.mem.f32(this.addr() + 0x23c), 0.0);
    }

    #[test]
    fn test_middle_high_process_get_sit_sleep_state() {
        let mut e = Engine::new();
        let this = process(&mut e);
        let flags = Rc::new(Cell::new(0u32));
        let seen = flags.clone();
        let mut slots = vec![0u32; 0x187];
        slots[0x618 / 4] = 0x0300_0000;
        e.put_vtable(0x0200_0000, &slots);
        e.mem.set_u32(this.addr(), 0x0200_0000);
        e.register_double(0x0300_0000, move |_, _| seen.get().into_ret());
        for (state, flag_word, expected) in [
            (4u8, 0x8u32, 5u32),
            (9, 0x8, 10),
            (4, 0x0, 4),
            (4, 0x7, 4),
            (5, 0x8, 5),
            (0, 0xff, 0),
        ] {
            e.mem.set_u8(this.addr() + 0x13d, state);
            flags.set(flag_word);
            assert_eq!(e.call(0x008d8b20, &args![this]).u32(), expected);
        }
    }

    #[test]
    fn test_middle_high_process_clear_current_furniture() {
        let mut e = Engine::new();
        let this = process(&mut e);
        let seen = record_list_removals(&mut e);
        e.mem.set_u32(this.addr() + 0x140, 0x1111);
        e.call(0x008d8ba0, &args![this, Ptr::<()>::new(0x2222)]);
        assert_eq!(*seen.borrow(), vec![(this.addr() + 0xd0, 0x2222)]);
        assert_eq!(
            e.mem.u32(this.addr() + 0x140),
            0x1111,
            "another reference stays"
        );
        e.call(0x008d8ba0, &args![this, Ptr::<()>::new(0x1111)]);
        assert_eq!(seen.borrow().len(), 2);
        assert_eq!(e.mem.u32(this.addr() + 0x140), 0);
    }

    #[test]
    fn test_middle_high_process_get_lighting_shader_property() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.register(NI_POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.mem.set_u32(this.addr() + 0x220, 0x4444);
        assert_eq!(e.call(0x008d8c00, &args![this]).u32(), 0x4444);
    }

    #[test]
    fn test_high_process_remove_spoken_to_actor() {
        let mut e = Engine::new();
        let this = process(&mut e);
        let seen = record_list_removals(&mut e);
        e.mem.set_u32(this.addr() + 0x264, 0x5000);
        e.call(0x008d8c60, &args![this, Ptr::<()>::new(0x6666)]);
        assert_eq!(*seen.borrow(), vec![(0x5000, 0x6666)]);
    }

    #[test]
    fn test_high_process_get_number_guards_arresting() {
        check_getter(0x008d8c80, 0x39c, 7);
    }

    #[test]
    fn test_high_process_mod_number_guards_arresting() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.mem.set_u32(this.addr() + 0x39c, 5);
        e.call(0x008d8ca0, &args![this, 3u32]);
        assert_eq!(e.mem.u32(this.addr() + 0x39c), 8);
        e.call(0x008d8ca0, &args![this, (-9i32) as u32]);
        assert_eq!(e.mem.u32(this.addr() + 0x39c), 0xffff_ffff);
    }

    #[test]
    fn test_middle_high_process_set_is_summoned_creature() {
        check_byte_setter(0x008d8720, 0x18b);
    }

    #[test]
    fn test_middle_high_process_get_all_3d_update_flags() {
        check_byte_getter(0x008d87e0, 0x18c);
    }

    #[test]
    fn test_middle_high_process_get_attached_arrow_list() {
        check_getter(0x008d8800, 0x1ac, 0x1234_5678);
    }

    #[test]
    fn test_middle_high_process_set_iron_sights() {
        check_byte_setter(0x008d8820, 0x228);
    }

    #[test]
    fn test_middle_high_process_get_iron_sights() {
        check_byte_getter(0x008d8840, 0x228);
    }

    #[test]
    fn test_middle_high_process_is_sustained_fire() {
        check_byte_getter(0x008d8860, 0x1d9);
    }

    #[test]
    fn test_middle_high_process_set_sustained_fire() {
        check_byte_setter(0x008d8880, 0x1d9);
    }

    #[test]
    fn test_middle_high_process_set_begin_idles_played() {
        check_byte_setter(0x008d88a0, 0x19c);
    }

    #[test]
    fn test_middle_high_process_get_begin_idles_played() {
        check_byte_getter(0x008d88c0, 0x19c);
    }

    #[test]
    fn test_middle_high_process_set_end_idles_played() {
        check_byte_setter(0x008d88e0, 0x19d);
    }

    #[test]
    fn test_middle_high_process_get_end_idles_played() {
        check_byte_getter(0x008d8900, 0x19d);
    }

    #[test]
    fn test_middle_high_process_get_creature_lip_synch_anim() {
        check_getter(0x008d8920, 0x1a0, 0x1234_5678);
    }

    #[test]
    fn test_middle_high_process_get_creature_lip_synch_start_time() {
        check_getter(0x008d8940, 0x1a4, 0x1234_5678);
    }

    #[test]
    fn test_middle_high_process_get_radiation_magic_delta() {
        check_float_getter(0x008d89c0, 0x238);
    }

    #[test]
    fn test_middle_high_process_set_radiation_water_delta() {
        check_float_setter(0x008d89e0, 0x23c);
    }

    #[test]
    fn test_middle_high_process_get_radiation_water_delta() {
        check_float_getter(0x008d8a00, 0x23c);
    }

    #[test]
    fn test_middle_high_process_set_radiation_delta() {
        check_float_setter(0x008d8a20, 0x234);
    }

    #[test]
    fn test_middle_high_process_get_last_attack_hit_data() {
        check_getter(0x008d8a40, 0x254, 0x1234_5678);
    }

    #[test]
    fn test_middle_high_process_get_weapon_condition_stage() {
        check_getter(0x008d8a60, 0x244, 0x1234_5678);
    }

    #[test]
    fn test_middle_high_process_set_weapon_condition_stage() {
        check_setter(0x008d8a80, 0x244, 0x1234_5678);
    }

    #[test]
    fn test_middle_high_process_set_face_node() {
        check_setter(0x008d8aa0, 0x248, 0x1234_5678);
    }

    #[test]
    fn test_middle_high_process_get_face_skinned_node() {
        check_getter(0x008d8ac0, 0x24c, 0x1234_5678);
    }

    #[test]
    fn test_middle_high_process_set_face_skinned_node() {
        check_setter(0x008d8ae0, 0x24c, 0x1234_5678);
    }

    #[test]
    fn test_middle_high_process_get_head_anims() {
        check_getter(0x008d8b00, 0x250, 0x1234_5678);
    }

    #[test]
    fn test_middle_high_process_get_current_furniture() {
        check_getter(0x008d8b80, 0x140, 0x1234_5678);
    }

    #[test]
    fn test_middle_high_process_get_current_furniture_index() {
        check_byte_getter(0x008d8be0, 0x144);
    }

    #[test]
    fn test_middle_high_process_get_torso_node() {
        check_getter(0x008d8c20, 0x21c, 0x1234_5678);
    }

    #[test]
    fn test_middle_high_process_set_anim_action_success() {
        check_word16_setter(0x008d8c40, 0x22a);
    }

    #[test]
    fn test_high_process_set_number_guards_arresting() {
        check_setter(0x008d8cd0, 0x39c, 0x1234_5678);
    }

    #[test]
    fn test_high_process_get_automatic_fire_at_least_once() {
        check_byte_getter(0x008d8cf0, 0x2f4);
    }

    #[test]
    fn test_high_process_set_automatic_fire_at_least_once() {
        check_byte_setter(0x008d8d10, 0x2f4);
    }

    #[test]
    fn test_high_process_is_doing_say_to() {
        check_byte_getter(0x008d8da0, 0x459);
    }

    #[test]
    fn test_high_process_set_doing_say_to() {
        check_byte_setter(0x008d8dc0, 0x459);
    }

    #[test]
    fn test_high_process_get_last_speaking_emotion() {
        check_getter(0x008d8de0, 0x36c, 0x1234_5678);
    }

    #[test]
    fn test_high_process_set_last_speaking_emotion() {
        check_setter(0x008d8e00, 0x36c, 0x1234_5678);
    }

    #[test]
    fn test_high_process_get_last_greeted() {
        check_getter(0x008d8e20, 0x30c, 0x1234_5678);
    }

    #[test]
    fn test_high_process_set_last_greeted() {
        check_setter(0x008d8e40, 0x30c, 0x1234_5678);
    }

    #[test]
    fn test_high_process_get_greeting_flag() {
        check_byte_getter(0x008d8f10, 0x32c);
    }

    #[test]
    fn test_high_process_set_greeting_flag() {
        check_byte_setter(0x008d8f30, 0x32c);
    }

    #[test]
    fn test_high_process_get_sound_delay() {
        check_float_getter(0x008d8f50, 0x310);
    }

    #[test]
    fn test_high_process_set_sound_delay() {
        check_float_setter(0x008d8f70, 0x310);
    }

    #[test]
    fn test_high_process_get_greeting_timer() {
        check_float_getter(0x008d8f90, 0x330);
    }

    #[test]
    fn test_high_process_set_greeting_timer() {
        check_float_setter(0x008d8fb0, 0x330);
    }

    #[test]
    fn test_high_process_get_detect_alert() {
        check_byte_getter(0x008d8fd0, 0x349);
    }

    #[test]
    fn test_high_process_set_detect_alert() {
        check_byte_setter(0x008d8ff0, 0x349);
    }

    #[test]
    fn test_high_process_get_idle_timer() {
        check_float_getter(0x008d9010, 0x334);
    }

    #[test]
    fn test_high_process_set_idle_timer() {
        check_float_setter(0x008d9030, 0x334);
    }

    #[test]
    fn test_high_process_continuing_packagefor_pc() {
        check_byte_getter(0x008d9050, 0x374);
    }

    #[test]
    fn test_high_process_set_continuing_package() {
        check_byte_setter(0x008d9070, 0x374);
    }

    #[test]
    fn test_high_process_get_aware_player_timer() {
        check_float_getter(0x008d9090, 0x34c);
    }

    #[test]
    fn test_high_process_set_breath_timer() {
        check_float_setter(0x008d90e0, 0x33c);
    }

    #[test]
    fn test_high_process_get_breath_timer() {
        check_float_getter(0x008d9100, 0x33c);
    }

    #[test]
    fn test_high_process_set_package_end_timer_value() {
        check_float_setter(0x008d9120, 0x378);
    }

    #[test]
    fn test_high_process_get_package_end_timer_value() {
        check_float_getter(0x008d9140, 0x378);
    }

    #[test]
    fn test_high_process_set_animation_active_flag() {
        check_byte_setter(0x008d9160, 0x375);
    }

    #[test]
    fn test_high_process_get_movement_stoped() {
        check_byte_getter(0x008d9180, 0x3a0);
    }

    #[test]
    fn test_high_process_set_currently_reanimating() {
        check_byte_setter(0x008d91c0, 0x3b9);
    }

    #[test]
    fn test_high_process_get_currently_reanimating() {
        check_byte_getter(0x008d91e0, 0x3b9);
    }

    #[test]
    fn test_high_process_get_waitingfor_lip_file() {
        check_byte_getter(0x008d9200, 0x3d0);
    }

    #[test]
    fn test_high_process_set_waitingfor_lip_file() {
        check_byte_setter(0x008d9220, 0x3d0);
    }

    #[test]
    fn test_high_process_get_lip_anim() {
        check_getter(0x008d9240, 0x3cc, 0x1234_5678);
    }

    #[test]
    fn test_high_process_save_weapon_last_pos() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.call(0x008d8d30, &args![this, 1.5f32, -2.5f32, 3.25f32]);
        assert_eq!(e.mem.f32(this.addr() + 0x300), 1.5);
        assert_eq!(e.mem.f32(this.addr() + 0x304), -2.5);
        assert_eq!(e.mem.f32(this.addr() + 0x308), 3.25);
        assert_eq!(e.mem.u32(this.addr() + 0x30c), 0);
        assert_eq!(e.mem.u32(this.addr() + 0x2fc), 0);
    }

    #[test]
    fn test_high_process_get_weapon_last_pos() {
        let mut e = Engine::new();
        let this = process(&mut e);
        assert_eq!(e.call(0x008d8d60, &args![this]).u32(), this.addr() + 0x300);
    }

    #[test]
    fn test_high_process_reset_search_chatter_timer() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.mem.set_f32(this.addr() + 0x298, 9.0);
        e.call(0x008d8d80, &args![this]);
        assert_eq!(e.mem.f32(this.addr() + 0x298), 0.0);
    }

    type SoundCalls = Rc<RefCell<Vec<(u32, u32, [u32; 3])>>>;

    /// Doubles the sound handle assignment and destructor, recording the calls.
    fn record_sound_handle_calls(e: &mut Engine) -> SoundCalls {
        let seen: SoundCalls = Rc::new(RefCell::new(vec![]));
        let log = seen.clone();
        e.register_double(SOUND_HANDLE_ASSIGN, move |e, a| {
            let words = [e.mem.u32(a[1]), e.mem.u32(a[1] + 4), e.mem.u32(a[1] + 8)];
            log.borrow_mut().push((SOUND_HANDLE_ASSIGN, a[0], words));
            a[0].into_ret()
        });
        let log = seen.clone();
        e.register_double(SOUND_HANDLE_DESTROY, move |_e, a| {
            log.borrow_mut().push((SOUND_HANDLE_DESTROY, a[0], [0; 3]));
            Ret::default()
        });
        seen
    }

    #[test]
    fn test_high_process_get_sound_handle() {
        let mut e = Engine::new();
        let this = process(&mut e);
        let out = Ptr::<()>::new(e.mem.alloc(12));
        let seen = record_sound_handle_calls(&mut e);
        e.mem.set_u32(this.addr() + 0x314 + 2 * 12, 7);
        e.mem.set_u32(this.addr() + 0x314 + 2 * 12 + 4, 8);
        e.mem.set_u32(this.addr() + 0x314 + 2 * 12 + 8, 9);
        let ret = e.call(0x008d8e60, &args![this, out, 2u32]).u32();
        assert_eq!(ret, out.addr());
        assert_eq!(
            *seen.borrow(),
            vec![(SOUND_HANDLE_ASSIGN, out.addr(), [7, 8, 9])]
        );
    }

    #[test]
    fn test_high_process_set_sound_handle() {
        let mut e = Engine::new();
        let this = process(&mut e);
        let seen = record_sound_handle_calls(&mut e);
        e.call(0x008d8ea0, &args![this, 1u32, 4u32, 5u32, 6u32]);
        let log = seen.borrow();
        assert_eq!(log.len(), 2);
        assert_eq!(
            log[0],
            (SOUND_HANDLE_ASSIGN, this.addr() + 0x314 + 12, [4, 5, 6])
        );
        assert_eq!(log[1].0, SOUND_HANDLE_DESTROY);
        assert_ne!(log[1].1, 0);
    }

    #[test]
    fn test_high_process_mod_aware_player_timer() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.mem.set_f32(this.addr() + 0x34c, 1.5);
        e.call(0x008d90b0, &args![this, 2.25f32]);
        assert_eq!(e.mem.f32(this.addr() + 0x34c), 3.75);
        e.call(0x008d90b0, &args![this, -4.0f32]);
        assert_eq!(e.mem.f32(this.addr() + 0x34c), -0.25);
    }

    #[test]
    fn test_high_process_clear_movement_stoped() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.mem.set_u8(this.addr() + 0x3a0, 1);
        e.mem.set_u8(this.addr() + 0x3a1, 0xff);
        e.call(0x008d91a0, &args![this]);
        assert_eq!(e.mem.u8(this.addr() + 0x3a0), 0);
        assert_eq!(e.mem.u8(this.addr() + 0x3a1), 0xff);
    }

    #[test]
    fn test_high_process_set_lip_anim() {
        check_setter(0x008d9260, 0x3cc, 0x1234_5678);
    }

    #[test]
    fn test_high_process_set_lip_file_failed() {
        check_byte_setter(0x008d9280, 0x3d1);
    }

    #[test]
    fn test_high_process_get_lip_file_failed() {
        check_byte_getter(0x008d92a0, 0x3d1);
    }

    #[test]
    fn test_high_process_get_lip_quequed() {
        check_byte_getter(0x008d92c0, 0x348);
    }

    #[test]
    fn test_high_process_set_lip_quequed() {
        check_byte_setter(0x008d92e0, 0x348);
    }

    #[test]
    fn test_high_process_get_current_muzzle_flash() {
        check_getter(0x008d9300, 0x3d4, 0x1234_5678);
    }

    #[test]
    fn test_high_process_get_social_talk_timer() {
        check_float_getter(0x008d9320, 0x2c8);
    }

    #[test]
    fn test_high_process_set_social_talk_timer() {
        check_float_setter(0x008d9340, 0x2c8);
    }

    #[test]
    fn test_high_process_get_actors_detection_event() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.mem.set_u32(this.addr() + 0x3dc, 0x4321);
        assert_eq!(e.call(0x008d9390, &args![this, 99u32]).u32(), 0x4321);
    }

    #[test]
    fn test_high_process_get_aggro_radius_actor_list() {
        let mut e = Engine::new();
        let this = process(&mut e);
        assert_eq!(e.call(0x008d93b0, &args![this]).u32(), this.addr() + 0x38c);
    }

    #[test]
    fn test_high_process_get_avoid_actor_list() {
        let mut e = Engine::new();
        let this = process(&mut e);
        assert_eq!(e.call(0x008d93d0, &args![this]).u32(), this.addr() + 0x394);
    }

    #[test]
    fn test_high_process_get_detection_timer() {
        check_float_getter(0x008d93f0, 0x2f8);
    }

    #[test]
    fn test_high_process_set_current_process_idle() {
        check_setter(0x008d9410, 0x350, 0x1234_5678);
    }

    #[test]
    fn test_high_process_get_finishing_combat_package() {
        check_byte_getter(0x008d9430, 0x3e0);
    }

    #[test]
    fn test_high_process_set_finishing_combat_package() {
        check_byte_setter(0x008d9450, 0x3e0);
    }

    #[test]
    fn test_high_process_set_detection_timer() {
        check_float_setter(0x008d9490, 0x2f8);
    }

    #[test]
    fn test_high_process_get_actor_light_level() {
        check_float_getter(0x008d94b0, 0x3c4);
    }

    #[test]
    fn test_high_process_set_actor_light_level() {
        check_float_setter(0x008d94d0, 0x3c4);
    }

    #[test]
    fn test_high_process_get_light_level_timer() {
        check_float_getter(0x008d94f0, 0x3c8);
    }

    #[test]
    fn test_high_process_set_light_level_timer() {
        check_float_setter(0x008d9510, 0x3c8);
    }

    #[test]
    fn test_high_process_get_talkingto_pc() {
        check_byte_getter(0x008d9530, 0x364);
    }

    #[test]
    fn test_high_process_set_talkingto_pc() {
        check_byte_setter(0x008d9550, 0x364);
    }

    #[test]
    fn test_high_process_get_detection_modifier() {
        check_float_getter(0x008d9590, 0x3bc);
    }

    #[test]
    fn test_high_process_get_detection_modifier_timer() {
        check_float_getter(0x008d95b0, 0x3c0);
    }

    #[test]
    fn test_high_process_get_fade_state() {
        check_getter(0x008d9610, 0x3e8, 0x1234_5678);
    }

    #[test]
    fn test_high_process_set_need_talk_player() {
        check_byte_setter(0x008d9630, 0x445);
    }

    #[test]
    fn test_high_process_get_need_talk_player() {
        check_byte_getter(0x008d9650, 0x445);
    }

    #[test]
    fn test_high_process_get_subtitle_item() {
        check_getter(0x008d9670, 0x454, 0x1234_5678);
    }

    #[test]
    fn test_high_process_set_subtitle_item() {
        check_setter(0x008d9690, 0x454, 0x1234_5678);
    }

    #[test]
    fn test_high_process_set_planted_explosive() {
        check_byte_setter(0x008d9700, 0x444);
    }

    #[test]
    fn test_high_process_get_planted_explosive() {
        check_byte_getter(0x008d9720, 0x444);
    }

    #[test]
    fn test_high_process_get_head_track_change_timer() {
        check_float_getter(0x008d9740, 0x418);
    }

    #[test]
    fn test_high_process_set_head_track_change_timer() {
        check_float_setter(0x008d9760, 0x418);
    }

    #[test]
    fn test_high_process_attack_callback() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.mem.set_u16(this.addr() + 0x2c0, 0xfffe);
        e.call(0x008d9360, &args![this, 5u32]);
        assert_eq!(e.mem.u16(this.addr() + 0x2c0), 3);
        e.call(0x008d9360, &args![this, 0xffff_fff0u32]);
        assert_eq!(e.mem.u16(this.addr() + 0x2c0), 0xfff3);
        assert_eq!(e.mem.u16(this.addr() + 0x2c2), 0);
    }

    #[test]
    fn test_high_process_reset_search_timer() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.mem.set_f32(this.addr() + 0x2e0, 9.0);
        e.mem.set_f32(this.addr() + 0x2e4, 9.0);
        e.call(0x008d9470, &args![this]);
        assert_eq!(e.mem.f32(this.addr() + 0x2e0), 0.0);
        assert_eq!(e.mem.f32(this.addr() + 0x2e4), 9.0);
    }

    #[test]
    fn test_high_process_set_detection_modifier_timer() {
        let mut e = Engine::new();
        let this = process(&mut e);
        let value = e.mem.alloc(4);
        e.mem.set_f32(value, 12.5);
        let seen = Rc::new(Cell::new(0u32));
        let log = seen.clone();
        e.register_double(SETTING_FLOAT_POINTER, move |_e, a| {
            log.set(a[0]);
            value.into_ret()
        });
        e.call(0x008d9570, &args![this]);
        assert_eq!(seen.get(), DETECTION_MODIFIER_SETTING);
        assert_eq!(e.mem.f32(this.addr() + 0x3c0), 12.5);
    }

    #[test]
    fn test_high_process_get_current_process_anim_idle() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.register(NI_POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.mem.set_u32(this.addr() + 0x354, 0x5151);
        assert_eq!(e.call(0x008d95d0, &args![this]).u32(), 0x5151);
    }

    #[test]
    fn test_high_process_get_current_process_anim_idle_kf() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.register(NI_POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.mem.set_u32(this.addr() + 0x35c, 0x6262);
        assert_eq!(e.call(0x008d95f0, &args![this]).u32(), 0x6262);
    }

    #[test]
    fn test_high_process_clear_post_animation_actions() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.mem.set_u32(this.addr() + 0x424, 0xff);
        e.mem.set_u32(this.addr() + 0x428, 0xff);
        e.call(0x008d96b0, &args![this]);
        assert_eq!(e.mem.u32(this.addr() + 0x424), 0);
        assert_eq!(e.mem.u32(this.addr() + 0x428), 0xff);
    }

    #[test]
    fn test_high_process_remove_post_animation_action() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.mem.set_u32(this.addr() + 0x424, 0b1111);
        e.call(0x008d96d0, &args![this, 0b0101u32]);
        assert_eq!(e.mem.u32(this.addr() + 0x424), 0b1010);
    }
}
