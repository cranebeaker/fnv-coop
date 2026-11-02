//! `fallout/ai/highprocess.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The first block of this unit is the process classes' one-field accessors
//! (`BaseProcess` > `LowProcess` > `MiddleLowProcess` > `MiddleHighProcess` >
//! `HighProcess`): each reads or writes one field of `this`. The field names
//! and offsets are the Xbox PDB's, checked against the PC code (they agree up
//! to `HighProcess::ePostAnimActon`). The classes' own layouts are not
//! declared here; the offsets are named constants.
//!
//! The two message-queue functions at the end are `BSTCommonMessageQueue`
//! template instances: a try-lock (compare-exchange of the word at +4 from 0
//! to 1) around the virtual `Push` / `Pop`.

#[allow(unused_imports)]
use crate::prelude::*;

/// `InterlockedCompareExchange(target, old, new)` wrapper (`0043b460`),
/// returning the previous value.
const COMPARE_EXCHANGE: u32 = 0x0043_b460;
/// `Error` (`0040fbe0`): a five-byte stub called around the locked section
/// (takes no arguments).
const ERROR_NO_OP: u32 = 0x0040_fbe0;

// Translated from 00407800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::SetSavedAcquireObject` (Xbox PDB) stores its argument in `pSavedAcquireObject (ObjectstoAcquire*)` at +0x68.
pub fn middle_high_process_set_saved_acquire_object(e: &mut Engine, this: Ptr, value: Ptr) {
    e.mem.set_u32(this.addr() + 0x68, value.addr());
}

// Translated from 004727d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetPostAnimationActions` (Xbox PDB) returns `ePostAnimActon (BaseProcess::POSTANIM_ACTION)` at +0x424.
pub fn high_process_get_post_animation_actions(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x424)
}

// Translated from 00502430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetFireNode` (Xbox PDB) returns `pFireNode (NiAVObject*)` at +0x130.
pub fn middle_high_process_get_fire_node(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(this.addr() + 0x130))
}

// Translated from 00502c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleLowProcess::SetPackageEvaluateHour` (Xbox PDB) stores its argument in `iHourPackageEvaluated` at +0xb4.
pub fn middle_low_process_set_package_evaluate_hour(e: &mut Engine, this: Ptr, value: i32) {
    e.mem.set_u32(this.addr() + 0xb4, value as u32);
}

// Translated from 00505e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::SetLastIdlePlayed` (Xbox PDB) stores its argument in `pLastIdlePlayed (TESIdleForm*)` at +0x10c.
pub fn middle_high_process_set_last_idle_played(e: &mut Engine, this: Ptr, value: Ptr) {
    e.mem.set_u32(this.addr() + 0x10c, value.addr());
}

// Translated from 005075f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::SetDeathTime` (Xbox PDB) stores its argument in `fDeathTime` at +0xa8.
pub fn low_process_set_death_time(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_f32(this.addr() + 0xa8, x87_quiet(value));
}

// Translated from 0051f510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetDialogTarget` (Xbox PDB) returns `pDialogTarget (MobileObject*)` at +0x370.
pub fn high_process_get_dialog_target(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(this.addr() + 0x370))
}

// Translated from 0051f530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; its body stores its argument in `pDialogTarget (MobileObject*)` at +0x370.
pub fn fn_0051f530(e: &mut Engine, this: Ptr, value: Ptr) {
    e.mem.set_u32(this.addr() + 0x370, value.addr());
}

// Translated from 0051f550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetHeadNode` (Xbox PDB) returns `pHeadNode (NiAVObject*)` at +0x218.
pub fn middle_high_process_get_head_node(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(this.addr() + 0x218))
}

// Translated from 00521630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetLastHitData` (Xbox PDB) returns `pLastHitData (HitData*)` at +0x240.
pub fn middle_high_process_get_last_hit_data(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(this.addr() + 0x240))
}

// Translated from 00521650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; its body stores its argument in `bIronSights (bool; the store is a whole word, so it also covers iAnimActionSuccess at +0x22a)` at +0x228.
pub fn fn_00521650(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0x228, value);
}

// Translated from 00521670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; its body stores its argument in `iMovementAnimations` at +0x22c.
pub fn fn_00521670(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0x22c, value);
}

// Translated from 00521690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; its body stores its argument in `pListItemstoEquipUnequip (BSSimpleList<QueuedItem *>*)` at +0x230.
pub fn fn_00521690(e: &mut Engine, this: Ptr, value: Ptr) {
    e.mem.set_u32(this.addr() + 0x230, value.addr());
}

// Translated from 005216b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; its body stores its argument in `fRadiationDelta (copied as a raw word)` at +0x234.
pub fn fn_005216b0(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0x234, value);
}

// Translated from 005216d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; its body stores its argument in `fRadiationMagicDelta (copied as a raw word)` at +0x238.
pub fn fn_005216d0(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0x238, value);
}

// Translated from 005216f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; its body stores its argument in `fRadiationWaterDelta (copied as a raw word)` at +0x23c.
pub fn fn_005216f0(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0x23c, value);
}

// Translated from 00521710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::SetHeadAnims` (Xbox PDB) stores its argument in `pAnimFace (NiAVObject*)` at +0x250.
pub fn middle_high_process_set_head_anims(e: &mut Engine, this: Ptr, value: Ptr) {
    e.mem.set_u32(this.addr() + 0x250, value.addr());
}

// Translated from 005224a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetFaceNode` (Xbox PDB) returns `pFaceNode (BSFaceGenNiNode*)` at +0x248.
pub fn middle_high_process_get_face_node(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(this.addr() + 0x248))
}

// Translated from 00566950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetForceNextUpdate` (Xbox PDB) returns `bForceNextUpdate` at +0x18d.
pub fn middle_high_process_get_force_next_update(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0x18d) != 0
}

// Translated from 005a29b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetLastIdlePlayed` (Xbox PDB) returns `pLastIdlePlayed (TESIdleForm*)` at +0x10c.
pub fn middle_high_process_get_last_idle_played(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(this.addr() + 0x10c))
}

// Translated from 005a8060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::SetNumberOfItemsActivated` (Xbox PDB) stores its argument in `iNumberItemsActivate` at +0x58.
pub fn low_process_set_number_of_items_activated(e: &mut Engine, this: Ptr, value: i32) {
    e.mem.set_u32(this.addr() + 0x58, value as u32);
}

// Translated from 005c8a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::SetBSBound` (Xbox PDB) stores its argument in `pBSBound (BSBound*)` at +0x224.
pub fn middle_high_process_set_bs_bound(e: &mut Engine, this: Ptr, value: Ptr) {
    e.mem.set_u32(this.addr() + 0x224, value.addr());
}

// Translated from 005ce8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::IsCurrentWeaponMine` (Xbox PDB) returns `bWeaponMine` at +0x125.
pub fn middle_high_process_is_current_weapon_mine(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0x125) != 0
}

// Translated from 005f7590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetCurrentAmmo` (Xbox PDB) returns `pAmmo (ItemChange*)` at +0x118.
pub fn middle_high_process_get_current_ammo(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(this.addr() + 0x118))
}

// Translated from 005f8bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::SetCommandingActor` (Xbox PDB) stores its argument in `pCommandingActor (Actor*)` at +0x158.
pub fn middle_high_process_set_commanding_actor(e: &mut Engine, this: Ptr, value: Ptr) {
    e.mem.set_u32(this.addr() + 0x158, value.addr());
}

// Translated from 005f9b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetCommandingActor` (Xbox PDB) returns `pCommandingActor (Actor*)` at +0x158.
pub fn middle_high_process_get_commanding_actor(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(this.addr() + 0x158))
}

// Translated from 00602150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetAnimation` (Xbox PDB) returns `pAnimation (Animation*)` at +0x1c0.
pub fn middle_high_process_get_animation(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(this.addr() + 0x1c0))
}

// Translated from 006214b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::GetCurrentProcessIdle` (Xbox PDB) returns `pIdleToPlay (TESIdleForm*)` at +0x350.
pub fn high_process_get_current_process_idle(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(this.addr() + 0x350))
}

// Translated from 006286d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::GetNumberOfItemsActivated` (Xbox PDB) returns `iNumberItemsActivate` at +0x58.
pub fn low_process_get_number_of_items_activated(e: &mut Engine, this: Ptr) -> i32 {
    e.mem.u32(this.addr() + 0x58) as i32
}

// Translated from 00645c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetScriptRefractionPower` (Xbox PDB) returns `fScriptRefractPower` at +0x174.
pub fn middle_high_process_get_script_refraction_power(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + 0x174)
}

// Translated from 00649b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetIsSummonedCreature` (Xbox PDB) returns `bSummonedCreature` at +0x18b.
pub fn middle_high_process_get_is_summoned_creature(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0x18b) != 0
}

// Translated from 006546c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::SetIdleDoneOnce` (Xbox PDB) stores its argument in `bDoneOnce` at +0xe0.
pub fn middle_high_process_set_idle_done_once(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0xe0, value);
}

// Translated from 006733e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::GetSecondGenericLocation` (Xbox PDB) returns `pGenericSecondLocation (TESObjectREFR*)` at +0x48.
pub fn low_process_get_second_generic_location(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(this.addr() + 0x48))
}

// Translated from 00673400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::SetSecondGenericLocation` (Xbox PDB) stores its argument in `pGenericSecondLocation (TESObjectREFR*)` at +0x48.
pub fn low_process_set_second_generic_location(e: &mut Engine, this: Ptr, value: Ptr) {
    e.mem.set_u32(this.addr() + 0x48, value.addr());
}

// Translated from 007ac9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::SetWeaponEnchantmentVisuals` (Xbox PDB) stores its argument in `pCurrentWeaponEffect (TESEffectShader*)` at +0x16c.
pub fn middle_high_process_set_weapon_enchantment_visuals(e: &mut Engine, this: Ptr, value: Ptr) {
    e.mem.set_u32(this.addr() + 0x16c, value.addr());
}

// Translated from 008041a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::GetGenericLocation` (Xbox PDB) returns `pGenericLocation (TESObjectREFR*)` at +0x44.
pub fn low_process_get_generic_location(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(this.addr() + 0x44))
}

// Translated from 00812870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::GetEssentialDownTimer` (Xbox PDB) returns `fEssentialDownTimer` at +0xa4.
pub fn low_process_get_essential_down_timer(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + 0xa4)
}

// Translated from 0087d7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::SetCreatureLipSynchStartTime` (Xbox PDB) stores its argument in `iCreatureLipsynchStartTime` at +0x1a4.
pub fn middle_high_process_set_creature_lip_synch_start_time(
    e: &mut Engine,
    this: Ptr,
    value: u32,
) {
    e.mem.set_u32(this.addr() + 0x1a4, value);
}

// Translated from 006ec0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTCommonMessageQueue<ActorPathingMessage>::TryPush` (Xbox PDB): takes
/// the queue's lock (compare-exchange of the word at +4 from 0 to 1); when
/// it is free, calls the virtual `Push` (vtable +0x14) with `message` and
/// releases the lock (1 back to 0). Returns `Push`'s result, or false when
/// the lock was taken.
pub fn bst_common_message_queue_actor_pathing_message_try_push(
    e: &mut Engine,
    this: Ptr,
    message: Ptr,
) -> bool {
    try_locked_vcall(e, this, 0x14, message)
}

// Translated from 006ec390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTCommonMessageQueue<ActorPathingMessage>::TryPop` (Xbox PDB): the same
/// try-lock around the virtual `Pop` (vtable +0x18).
pub fn bst_common_message_queue_actor_pathing_message_try_pop(
    e: &mut Engine,
    this: Ptr,
    message: Ptr,
) -> bool {
    try_locked_vcall(e, this, 0x18, message)
}

/// The body both `Try` functions share.
fn try_locked_vcall(e: &mut Engine, this: Ptr, slot: u32, message: Ptr) -> bool {
    let lock = this.addr() + 4;
    let previous = e.call(COMPARE_EXCHANGE, &args![lock, 0u32, 1u32]).u32();
    if previous != 0 {
        return false;
    }
    e.call(ERROR_NO_OP, &args![]);
    let result = e.vcall(this.addr(), slot, &args![message]).bool();
    e.call(ERROR_NO_OP, &args![]);
    e.call(COMPARE_EXCHANGE, &args![lock, 1u32, 0u32]);
    result
}

// ---- Save and load helpers, the constructor and the `LowProcess` flags -------------

/// `BGSSaveGameBuffer::SaveFormID_ov2(buffer, form, 0)` (`00865df0`).
const SAVE_FORM_ID: u32 = 0x0086_5df0;
/// `BGSSaveGameBuffer::Save(buffer, data, size, 0)` (`00865e50`): writes
/// `size` bytes read from `data`.
const SAVE_BYTES: u32 = 0x0086_5e50;
/// `BGSLoadGameBuffer::Load(buffer, destination, size)` (`00864980`).
const LOAD_BYTES: u32 = 0x0086_4980;
/// `BGSLoadGameBuffer::LoadFormID_ov2(buffer, destination)` (`008648e0`).
const LOAD_FORM_ID_OV2: u32 = 0x0086_48e0;
/// `BGSLoadGameBuffer::LoadFormID(buffer)` (`008648a0`): the form ID read.
const LOAD_FORM_ID: u32 = 0x0086_48a0;
/// `CombatTimeStamp::SaveGame(stamp, buffer)` and `LoadGame(stamp, buffer)`
/// (`009a5f90`, `009a5ff0`).
const COMBAT_TIME_STAMP_SAVE: u32 = 0x009a_5f90;
const COMBAT_TIME_STAMP_LOAD: u32 = 0x009a_5ff0;
/// Reads the `float` a `CombatTimeStamp` holds (`006a7f50`, returned in
/// ST0).
const TIME_STAMP_VALUE: u32 = 0x006a_7f50;
/// `-FLT_MAX` is the "no time" value; the constant stored at this address is
/// `FLT_MAX` (`7f7fffff`).
const MAX_FLOAT: u32 = 0x0108_7820;
/// Form lookup by form ID (`004839c0`): null when there is no such form.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `__RTDynamicCast(object, 0, source type, target type, 0)` (`00ec43fb`).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `TypeDescriptor` of `TESForm`, the source type of the form casts here.
const TYPE_TES_FORM: u32 = 0x0118_3028;
/// `TypeDescriptor` of `Actor`.
const TYPE_ACTOR: u32 = 0x0118_46d4;
/// `TypeDescriptor` of `TESObjectREFR`.
const TYPE_TES_OBJECT_REFR: u32 = 0x0118_41cc;
/// `TypeDescriptor` of `TESObjectSTAT` (`.?AVTESObjectSTAT@@`), the target
/// type the form cast of `008d7420` names.
const TYPE_TES_OBJECT_STAT: u32 = 0x0118_6568;
/// `TESObjectREFR::SetTargeted(reference, on)` (`00564db0`).
const SET_TARGETED: u32 = 0x0056_4db0;
/// The save/load of the base part of the object `008d73b0` / `008d7420`
/// handle (`006dc890` / `006dc910`, each `(this, buffer)`).
const BASE_SAVE: u32 = 0x006d_c890;
const BASE_LOAD: u32 = 0x006d_c910;

/// `HighProcess` constructor callees.
/// The base constructor (`MiddleHighProcess`, `00913fe0`), the list
/// constructor `BSSimpleList::BSSimpleList` (`0096a2d0`, returns its
/// object), `operator new(size)` (`00401000`) and `NiPoint3`'s default
/// constructor (`006815c0`).
const BASE_CONSTRUCTOR: u32 = 0x0091_3fe0;
const LIST_CONSTRUCTOR: u32 = 0x0096_a2d0;
const OPERATOR_NEW: u32 = 0x0040_1000;
const POINT_CONSTRUCTOR: u32 = 0x0068_15c0;
/// `__ehvec_ctor(array, element size, count, constructor, destructor)`
/// (`00ec782f`, cdecl).
const VECTOR_CONSTRUCT: u32 = 0x00ec_782f;
/// `NiPointer` initialisation from a pointer (`00633c90`) and assignment
/// (`0066b0d0`), `NiPointer<ActorPathingMessageQueue>` initialisation
/// (`006ebf60`), the `NiPointer<KFModel>` assignment (`0044b070`) and the
/// function `0061cc40(this, 0)`, which stores its argument in the word at +0x28 (`LowProcess::eLevel` in the PDB; the engine map gives the address a `testopicinfo.cpp` setter, since the linker folded identical code).
const NI_POINTER_INIT: u32 = 0x0063_3c90;
const NI_POINTER_SET: u32 = 0x0066_b0d0;
const QUEUE_POINTER_INIT: u32 = 0x006e_bf60;
const KF_MODEL_POINTER_SET: u32 = 0x0044_b070;
const STORE_WORD_AT_0X28: u32 = 0x0061_cc40;
/// Element constructors and destructors the array constructors are given:
/// `BSSoundHandle` (`0041a250`, `00483710`), `NiPointer<AnimIdle>`
/// (`006694e0`, `0045cec0`) and `NiPointer<KFModel>` (`008d98c0`,
/// `0044b030`).
const SOUND_HANDLE_CONSTRUCTOR: u32 = 0x0041_a250;
const SOUND_HANDLE_DESTRUCTOR: u32 = 0x0048_3710;
const ANIM_IDLE_POINTER_CONSTRUCTOR: u32 = 0x0066_94e0;
const ANIM_IDLE_POINTER_DESTRUCTOR: u32 = 0x0045_cec0;
const KF_MODEL_POINTER_CONSTRUCTOR: u32 = 0x008d_98c0;
const KF_MODEL_POINTER_DESTRUCTOR: u32 = 0x0044_b030;
/// Pointer to the `float` a game setting holds (`00403e20(setting)`).
/// The game clock `float` (`00435dd0`, returned in ST0): the global at `011f1bf0`.
const CLOCK: u32 = 0x0043_5dd0;
const SETTING_FLOAT: u32 = 0x0040_3e20;
/// `CombatFormulas::GetRandomBetween(first, second)` (`006465f0`, cdecl).
const RANDOM_BETWEEN: u32 = 0x0064_65f0;
/// The `HighProcess` vtable.
const HIGH_PROCESS_VTABLE: u32 = 0x0108_7864;
/// Game settings the constructor draws its two random timers from (the
/// first and second argument of `GetRandomBetween` for `fIdleChatterTimer`;
/// the same for `fCheckToTalkTimer`), and the one the clear-talk-to-list
/// timer starts at.
const IDLE_CHATTER_FIRST: u32 = 0x011c_d008;
const IDLE_CHATTER_SECOND: u32 = 0x011c_d18c;
const CHECK_TO_TALK_FIRST: u32 = 0x011c_d328;
const CHECK_TO_TALK_SECOND: u32 = 0x011c_d038;
const CLEAR_TALK_TO_LIST: u32 = 0x011c_dcd8;
/// Globals the constructor copies: the three words of the default weapon
/// position (`NiPoint3`), the breath timer, the two head-track / take-back
/// timers (one `float`), and the running detection counter.
const DEFAULT_WEAPON_POSITION: u32 = 0x011f_426c;
const DEFAULT_BREATH_TIMER: u32 = 0x0101_7868;
const DEFAULT_HEAD_TRACK_TIMER: u32 = 0x0101_2054;
const DETECTION_COUNTER: u32 = 0x011e_01e0;
/// `LowProcess` flag bits (`m_uFlags`, byte at +0x30).
const FLAG_TARGET_ACTIVATED: u8 = 0x01;
const FLAG_ACTION_COMPLETE: u8 = 0x02;
const FLAG_AGGRESSOR: u8 = 0x04;
const FLAG_ALERT: u8 = 0x08;
const FLAG_FOLLOWER: u8 = 0x10;
const FLAG_PACKAGE_DONE_ONCE: u8 = 0x20;
const FLAG_LOCKED_LOCATION: u8 = 0x80;

layout! {
    /// `DetectionState` (Xbox PDB), size 0x24.
    pub struct DetectionState: 0x24 {
        /// `pActor` (Xbox PDB): `Actor*` (the save writes its form ID).
        0x00 pActor: Ptr,
        /// `eDetectionLevel` (Xbox PDB): one byte in the save, a whole word
        /// in memory.
        0x04 eDetectionLevel: i32,
        /// `iLevel` (Xbox PDB).
        0x08 iLevel: i32,
        /// `LastPositionDetected` (Xbox PDB): `NiPoint3`, three words.
        0x0c lastPositionDetectedX: u32,
        /// `fLastTimeDetected` (Xbox PDB): `CombatTimeStamp`.
        0x18 fLastTimeDetected: u32,
        /// `b360view` (Xbox PDB).
        0x1c b360view: u8,
        /// `bcombat` (Xbox PDB).
        0x1d bcombat: u8,
        /// `bLineofSight` (Xbox PDB).
        0x1e bLineofSight: u8,
        /// `bEvaluated` (Xbox PDB).
        0x1f bEvaluated: u8,
        /// `iDetectedEventLevel` (Xbox PDB).
        0x20 iDetectedEventLevel: u32,
    }

    /// `DetectionEvent` (Xbox PDB), size 0x1c.
    pub struct DetectionEvent: 0x1c {
        /// `iActionValue` (Xbox PDB).
        0x00 iActionValue: u32,
        /// `Location` (Xbox PDB): `NiPoint3`.
        0x04 Location: u32,
        /// `fTimeStamp` (Xbox PDB): `CombatTimeStamp`.
        0x10 fTimeStamp: u32,
        /// `eEventType` (Xbox PDB).
        0x14 eEventType: u32,
        /// `pRef` (Xbox PDB): `TESObjectREFR*`.
        0x18 pRef: Ptr,
    }

    /// `LowProcess` (Xbox PDB), the fields `0x8d7dc0` to `0x8d80c0` use.
    pub struct LowProcess: 0xb4 {
        /// `m_uFlags` (Xbox PDB).
        0x30 m_uFlags: u8,
        /// `pItemBeingUsed` (Xbox PDB): `TESBoundObject*`.
        0x34 pItemBeingUsed: Ptr,
        /// `pGenericLocation` (Xbox PDB): `TESObjectREFR*`.
        0x44 pGenericLocation: Ptr,
        /// `fEssentialDownTimer` (Xbox PDB).
        0xa4 fEssentialDownTimer: f32,
    }

    /// `CombatTimer` (Xbox PDB, `LowProcess::fCombatDelayTimer`): the time
    /// the timer was started and its delay.
    pub struct CombatTimer: 0x8 {
        /// The game time at the start.
        0x00 fStartTime: f32,
        /// The delay.
        0x04 fDelay: f32,
    }

    /// `MiddleHighProcess` (Xbox PDB), the fields `0x8d8100` to `0x8d81a0`
    /// use.
    pub struct MiddleHighProcess: 0x1c0 {
        /// `pCurrentSpell` (Xbox PDB): `MagicItem*`.
        0x160 pCurrentSpell: Ptr,
        /// `bCheckMagicNode` (Xbox PDB).
        0x168 bCheckMagicNode: bool,
        /// `pActiveEffectList` (Xbox PDB): `BSSimpleList<ActiveEffect *>*`.
        0x1b8 pActiveEffectList: Ptr,
        /// `pDesiredTarget` (Xbox PDB): `MagicTarget*`.
        0x1bc pDesiredTarget: Ptr,
    }

    /// `HighProcess` (Xbox PDB), size 0x46c: every field its constructor
    /// sets (PC offsets equal the PDB's).
    pub struct HighProcess: 0x46c {
        /// `iHourPackageEvaluated` (`MiddleLowProcess`).
        0x0b4 iHourPackageEvaluated: i32,
        /// `pDetectedActorList`: `BSSimpleList<DetectionState *>*`.
        0x25c pDetectedActorList: Ptr,
        /// `pActorsWhoDetectMeList`.
        0x260 pActorsWhoDetectMeList: Ptr,
        /// `pLastSpokeToList`: `BSSimpleList<Actor *>*`.
        0x264 pLastSpokeToList: Ptr,
        /// `pThreadDetectList`.
        0x268 pThreadDetectList: Ptr,
        /// `pTempActorsWhoDetectMeList`.
        0x26c pTempActorsWhoDetectMeList: Ptr,
        /// `bEvaluateDetection`.
        0x270 bEvaluateDetection: u8,
        /// `fDetectListTimer`.
        0x294 fDetectListTimer: f32,
        /// `fIdleChatterTimer`.
        0x298 fIdleChatterTimer: f32,
        /// `bSayGoodByePlayer`.
        0x29c bSayGoodByePlayer: u8,
        /// `bProcessGreetSayTo`.
        0x29d bProcessGreetSayTo: u8,
        /// `fClearTalkToListTimer`.
        0x2a0 fClearTalkToListTimer: f32,
        /// `plastDetected`.
        0x2a4 plastDetected: Ptr,
        /// `fTalkTimer`.
        0x2a8 fTalkTimer: f32,
        /// `pPathLookAtTarget`.
        0x2ac pPathLookAtTarget: Ptr,
        /// `fMaxAlpha`.
        0x2b0 fMaxAlpha: f32,
        /// `fPackageEvalTimer`.
        0x2b4 fPackageEvalTimer: f32,
        /// `fUseItemTimer`.
        0x2b8 fUseItemTimer: f32,
        /// `fHoldAttackTimer`.
        0x2bc fHoldAttackTimer: f32,
        /// `sShotsFired`.
        0x2c0 sShotsFired: i16,
        /// `sShotsToFire`.
        0x2c2 sShotsToFire: i16,
        /// `sBurstsFired`.
        0x2c4 sBurstsFired: i16,
        /// `bCheckDeadTalk`.
        0x2c6 bCheckDeadTalk: u8,
        /// `bSkippedUpdate`.
        0x2c7 bSkippedUpdate: u8,
        /// `fCheckToTalkTimer`.
        0x2c8 fCheckToTalkTimer: f32,
        /// `pNode`: `NiNode*`.
        0x2cc pNode: Ptr,
        /// `fDelayTimer`.
        0x2d0 fDelayTimer: f32,
        /// `fDistanceMoved`.
        0x2d4 fDistanceMoved: f32,
        /// `fTurnTime`.
        0x2d8 fTurnTime: f32,
        /// `cLastTurnDir`.
        0x2dc cLastTurnDir: u8,
        /// `fEvaluateAcquireTimer`.
        0x2e0 fEvaluateAcquireTimer: f32,
        /// `pBoneLOD`.
        0x2e4 pBoneLOD: Ptr,
        /// `iLastBoneLOD`.
        0x2e8 iLastBoneLOD: u32,
        /// `sAnimAction`.
        0x2ec sAnimAction: i16,
        /// `pAnimSeq`.
        0x2f0 pAnimSeq: Ptr,
        /// `bAutomaticFireAtLeastOne`.
        0x2f4 bAutomaticFireAtLeastOne: u8,
        /// `fDetectionTimer`.
        0x2f8 fDetectionTimer: f32,
        /// `iLastDetection`.
        0x2fc iLastDetection: i16,
        /// `pGreetActor`.
        0x30c pGreetActor: Ptr,
        /// `fSoundDelay`.
        0x310 fSoundDelay: f32,
        /// `bGreetingFlag`.
        0x32c bGreetingFlag: u8,
        /// `fGreetingTimer`.
        0x330 fGreetingTimer: f32,
        /// `fIdleTimer`.
        0x334 fIdleTimer: f32,
        /// `fDetectGreetTimer`.
        0x338 fDetectGreetTimer: f32,
        /// `fBreathTimer`.
        0x33c fBreathTimer: f32,
        /// `bHeadTrack`.
        0x340 bHeadTrack: u8,
        /// `fVoiceTimer`.
        0x344 fVoiceTimer: f32,
        /// `bLipQuequed`.
        0x348 bLipQuequed: u8,
        /// `bWeaponAlertDrawn`.
        0x349 bWeaponAlertDrawn: u8,
        /// `fAwarePlayerTimer`.
        0x34c fAwarePlayerTimer: f32,
        /// `pIdleToPlay`.
        0x350 pIdleToPlay: Ptr,
        /// `bDialoguewithPlayer`.
        0x364 bDialoguewithPlayer: u8,
        /// `pGreetTopic`.
        0x368 pGreetTopic: Ptr,
        /// `pDialogTarget`.
        0x370 pDialogTarget: Ptr,
        /// `bContinuingPackageforPC`.
        0x374 bContinuingPackageforPC: u8,
        /// `bActivateAnim`.
        0x375 bActivateAnim: u8,
        /// `fScriptPackageEndTime`.
        0x378 fScriptPackageEndTime: f32,
        /// `fHealthBarAlphaValue`.
        0x37c fHealthBarAlphaValue: f32,
        /// `fActorHealthPercentage`.
        0x384 fActorHealthPercentage: f32,
        /// `fHealthBarEmittanceValue`.
        0x388 fHealthBarEmittanceValue: f32,
        /// `iNumberGuardsPersuing`.
        0x39c iNumberGuardsPersuing: u32,
        /// `bStop`.
        0x3a0 bStop: u8,
        /// `fReEquipArmorTimer`.
        0x3a4 fReEquipArmorTimer: f32,
        /// `bUnequippedArmorToSwim`.
        0x3a8 bUnequippedArmorToSwim: u8,
        /// `iHasHealingSpell`.
        0x3ac iHasHealingSpell: u32,
        /// `iHasHealingPotion`.
        0x3b0 iHasHealingPotion: u32,
        /// `pLeveledSpellList`.
        0x3b4 pLeveledSpellList: Ptr,
        /// `cLastTurn`.
        0x3b8 cLastTurn: u8,
        /// `bCurrentlyReanimating`.
        0x3b9 bCurrentlyReanimating: u8,
        /// `fDetectionModifer`.
        0x3bc fDetectionModifer: f32,
        /// `fDetectionModifierTimer`.
        0x3c0 fDetectionModifierTimer: f32,
        /// `fLightLevel`.
        0x3c4 fLightLevel: f32,
        /// `fLightLevelTimer`.
        0x3c8 fLightLevelTimer: f32,
        /// `pLipSynicAnim`.
        0x3cc pLipSynicAnim: Ptr,
        /// `bWaitingForLipFile`.
        0x3d0 bWaitingForLipFile: u8,
        /// `bLipFileFailed`.
        0x3d1 bLipFileFailed: u8,
        /// `pCurrentMuzzleFlash`.
        0x3d4 pCurrentMuzzleFlash: Ptr,
        /// `iDetectionCounter`.
        0x3d8 iDetectionCounter: u32,
        /// `pActorsGeneratedDetectionEvent`.
        0x3dc pActorsGeneratedDetectionEvent: Ptr,
        /// `bFinishingCombatPackage`.
        0x3e0 bFinishingCombatPackage: u8,
        /// `m_pSayToDialogueTopic`.
        0x3e4 m_pSayToDialogueTopic: Ptr,
        /// `eFadeState`.
        0x3e8 eFadeState: u32,
        /// `fFadeAlpha`.
        0x3ec fFadeAlpha: f32,
        /// `pTeleportFadeRef`.
        0x3f0 pTeleportFadeRef: Ptr,
        /// `pMoveToFadeStruct`.
        0x3f4 pMoveToFadeStruct: Ptr,
        /// `pLastTarget`.
        0x41c pLastTarget: Ptr,
        /// `bForceRotate`.
        0x420 bForceRotate: u8,
        /// `ePostAnimActon`.
        0x424 ePostAnimActon: u32,
        /// `pActorValueCache`.
        0x428 pActorValueCache: Ptr,
        /// `fCachedActorHeight`.
        0x42c fCachedActorHeight: f32,
        /// `eSpecialIdleType`.
        0x430 eSpecialIdleType: u32,
        /// `fHighestRadiation`.
        0x440 fHighestRadiation: f32,
        /// `fRadiationTimer`.
        0x43c fRadiationTimer: f32,
        /// `bPlantedExplosive`.
        0x444 bPlantedExplosive: u8,
        /// `bNeedTalkPlayer`.
        0x445 bNeedTalkPlayer: u8,
        /// `fTakeBackTimer`.
        0x448 fTakeBackTimer: f32,
        /// `pListOfAvoidAreas`.
        0x44c pListOfAvoidAreas: Ptr,
        /// `fAvoidWaitTimer`.
        0x450 fAvoidWaitTimer: f32,
        /// `pSubtitleVoice`.
        0x454 pSubtitleVoice: Ptr,
        /// `bHiding`.
        0x458 bHiding: u8,
        /// `bIsDoingSayTo`.
        0x459 bIsDoingSayTo: u8,
    }
}

/// `HighProcess::fHeadTrackTargetTimer` and the arrays of head-tracking
/// targets: `TESObjectREFR*[6]` at +0x3f8 and `bool[6]` flags at +0x410.
const HEAD_TRACKING_TARGETS: u32 = 0x3f8;
const HEAD_TRACKING_TARGET_FLAGS: u32 = 0x410;
/// `fHeadTrackTargetTimer` (+0x418).
const HEAD_TRACK_TARGET_TIMER: u32 = 0x418;

/// `__RTDynamicCast` of a form ID's form to `target`: the form
/// (`004839c0`), or null if there is none, converted from `TESForm`.
fn form_cast(e: &mut Engine, form_id: u32, target: u32) -> Ptr {
    let form = e.call(LOOKUP_FORM, &args![form_id]).ptr::<()>();
    e.call(
        RT_DYNAMIC_CAST,
        &args![
            form,
            0i32,
            Ptr::<()>::new(TYPE_TES_FORM),
            Ptr::<()>::new(target),
            0i32
        ],
    )
    .ptr::<()>()
}

/// The shared tail of `008d7340` / `008d74a0`: replaces the form ID in the
/// word at `slot` with the `TESObjectREFR` it names (or null) and marks that
/// reference as targeted.
fn resolve_targeted_reference(e: &mut Engine, slot: u32) {
    let form_id = e.mem.u32(slot);
    let reference = if form_id == 0 {
        Ptr::<()>::NULL
    } else {
        form_cast(e, form_id, TYPE_TES_OBJECT_REFR)
    };
    e.mem.set_u32(slot, reference.addr());
    if !reference.is_null() {
        e.call(SET_TARGETED, &args![reference, 1u32]);
    }
}

// Translated from 008d6fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; its body copies a `DetectionState` field by
/// field from `source` into `this` (the assignment operator): the actor,
/// level and flag bytes, then, only when the source's time stamp is
/// greater than `-FLT_MAX` (read through `006a7f50`), the last detected
/// position and the time stamp.
pub fn fn_008d6fc0(e: &mut Engine, this: Ptr<DetectionState>, source: Ptr<DetectionState>) {
    let v = e.get(source, DetectionState::pActor);
    e.set(this, DetectionState::pActor, v);
    let v = e.get(source, DetectionState::eDetectionLevel);
    e.set(this, DetectionState::eDetectionLevel, v);
    let v = e.get(source, DetectionState::bLineofSight);
    e.set(this, DetectionState::bLineofSight, v);
    let v = e.get(source, DetectionState::iLevel);
    e.set(this, DetectionState::iLevel, v);
    let v = e.get(source, DetectionState::b360view);
    e.set(this, DetectionState::b360view, v);
    let v = e.get(source, DetectionState::bcombat);
    e.set(this, DetectionState::bcombat, v);
    let v = e.get(source, DetectionState::iDetectedEventLevel);
    e.set(this, DetectionState::iDetectedEventLevel, v);
    let v = e.get(source, DetectionState::bEvaluated);
    e.set(this, DetectionState::bEvaluated, v);
    let stamp = e.call(TIME_STAMP_VALUE, &args![source.addr() + 0x18]).f64();
    let limit = -f64::from(e.global::<f32>(MAX_FLOAT));
    // `-FLT_MAX < stamp` (false for a NaN stamp).
    if limit < stamp {
        for word in 0..3 {
            let offset = 0x0c + 4 * word;
            let v = e.mem.u32(source.addr() + offset);
            e.mem.set_u32(this.addr() + offset, v);
        }
        let v = e.get(source, DetectionState::fLastTimeDetected);
        e.set(this, DetectionState::fLastTimeDetected, v);
    }
}

// Translated from 008d7070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetectionState::SaveGame` (Xbox PDB): writes the actor's form ID, the
/// detection level as one byte, the level word, the three position words,
/// the time stamp (`CombatTimeStamp::SaveGame`) and the flag bytes and the
/// detected-event level, in that order.
pub fn detection_state_save_game(e: &mut Engine, this: Ptr<DetectionState>, buffer: Ptr) {
    let actor = e.get(this, DetectionState::pActor);
    e.call(SAVE_FORM_ID, &args![buffer, actor, 0u32]);
    // The level is saved as a byte copied to a local.
    let level = e.get(this, DetectionState::eDetectionLevel) as u8;
    e.with_stack(4, |e, local| {
        e.mem.set_u8(local.addr(), level);
        e.call(SAVE_BYTES, &args![buffer, local, 1u32, 0u32]);
    });
    e.call(SAVE_BYTES, &args![buffer, this.addr() + 0x08, 4u32, 0u32]);
    e.call(
        SAVE_BYTES,
        &args![buffer, this.addr() + 0x0c, 0x0cu32, 0u32],
    );
    e.call(COMBAT_TIME_STAMP_SAVE, &args![this.addr() + 0x18, buffer]);
    e.call(SAVE_BYTES, &args![buffer, this.addr() + 0x1e, 1u32, 0u32]);
    e.call(SAVE_BYTES, &args![buffer, this.addr() + 0x1c, 1u32, 0u32]);
    e.call(SAVE_BYTES, &args![buffer, this.addr() + 0x1d, 1u32, 0u32]);
    e.call(SAVE_BYTES, &args![buffer, this.addr() + 0x20, 4u32, 0u32]);
    e.call(SAVE_BYTES, &args![buffer, this.addr() + 0x1f, 1u32, 0u32]);
}

// Translated from 008d7140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetectionState::LoadGame` (Xbox PDB): the reverse of `SaveGame`. The
/// form ID goes into the actor word (resolved later), the level byte is
/// sign-extended into the level word, and the last flag byte (`bEvaluated`)
/// is only read from saves whose version (the buffer's virtual slot 0) is
/// at least 8.
pub fn detection_state_load_game(e: &mut Engine, this: Ptr<DetectionState>, buffer: Ptr) {
    e.call(LOAD_FORM_ID_OV2, &args![buffer, this]);
    let level = e.with_stack(4, |e, local| {
        e.mem.set_u8(local.addr(), 0);
        e.call(LOAD_BYTES, &args![buffer, local, 1u32]);
        e.mem.u8(local.addr())
    });
    e.set(
        this,
        DetectionState::eDetectionLevel,
        i32::from(level as i8),
    );
    e.call(LOAD_BYTES, &args![buffer, this.addr() + 0x08, 4u32]);
    e.call(LOAD_BYTES, &args![buffer, this.addr() + 0x0c, 0x0cu32]);
    e.call(COMBAT_TIME_STAMP_LOAD, &args![this.addr() + 0x18, buffer]);
    e.call(LOAD_BYTES, &args![buffer, this.addr() + 0x1e, 1u32]);
    e.call(LOAD_BYTES, &args![buffer, this.addr() + 0x1c, 1u32]);
    e.call(LOAD_BYTES, &args![buffer, this.addr() + 0x1d, 1u32]);
    e.call(LOAD_BYTES, &args![buffer, this.addr() + 0x20, 4u32]);
    let version = e.vcall(buffer.addr(), 0, &args![]).u32() as u8;
    if version >= 8 {
        e.call(LOAD_BYTES, &args![buffer, this.addr() + 0x1f, 1u32]);
    }
}

// Translated from 008d7220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; its body replaces the form ID stored in the
/// word `this` points to with the `Actor` that form is (null when the ID is
/// 0 or the form is no `Actor`).
pub fn fn_008d7220(e: &mut Engine, this: Ptr) {
    let form_id = e.mem.u32(this.addr());
    let actor = if form_id == 0 {
        Ptr::<()>::NULL
    } else {
        form_cast(e, form_id, TYPE_ACTOR)
    };
    e.mem.set_u32(this.addr(), actor.addr());
}

// Translated from 008d7270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetectionEvent::SaveGame` (Xbox PDB): writes the action value, the three
/// location words, the time stamp (`CombatTimeStamp::SaveGame`), the event
/// type and the reference's form ID.
pub fn detection_event_save_game(e: &mut Engine, this: Ptr<DetectionEvent>, buffer: Ptr) {
    e.call(SAVE_BYTES, &args![buffer, this, 4u32, 0u32]);
    e.call(
        SAVE_BYTES,
        &args![buffer, this.addr() + 0x04, 0x0cu32, 0u32],
    );
    e.call(COMBAT_TIME_STAMP_SAVE, &args![this.addr() + 0x10, buffer]);
    e.call(SAVE_BYTES, &args![buffer, this.addr() + 0x14, 4u32, 0u32]);
    let reference = e.get(this, DetectionEvent::pRef);
    e.call(SAVE_FORM_ID, &args![buffer, reference, 0u32]);
}

// Translated from 008d72e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; its body is `DetectionEvent`'s load: the
/// reverse of `008d7270` (the reference's form ID lands in the `pRef` word).
pub fn fn_008d72e0(e: &mut Engine, this: Ptr<DetectionEvent>, buffer: Ptr) {
    e.call(LOAD_BYTES, &args![buffer, this, 4u32]);
    e.call(LOAD_BYTES, &args![buffer, this.addr() + 0x04, 0x0cu32]);
    e.call(COMBAT_TIME_STAMP_LOAD, &args![this.addr() + 0x10, buffer]);
    e.call(LOAD_BYTES, &args![buffer, this.addr() + 0x14, 4u32]);
    e.call(LOAD_FORM_ID_OV2, &args![buffer, this.addr() + 0x18]);
}

// Translated from 008d7340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; its body turns the form ID in a
/// `DetectionEvent`'s `pRef` word into the `TESObjectREFR` it names (null
/// when the ID is 0 or no such reference) and, when there is one, marks it
/// targeted (`TESObjectREFR::SetTargeted(1)`).
pub fn fn_008d7340(e: &mut Engine, this: Ptr<DetectionEvent>) {
    resolve_targeted_reference(e, this.addr() + 0x18);
}

// Translated from 008d73b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; its body saves an object whose base part is
/// saved by `006dc890`: then the words at +0x24 and +0x2c, and the form IDs
/// of the forms at +0x28 and +0x30.
pub fn fn_008d73b0(e: &mut Engine, this: Ptr, buffer: Ptr) {
    e.call(BASE_SAVE, &args![this, buffer]);
    e.call(SAVE_BYTES, &args![buffer, this.addr() + 0x24, 4u32, 0u32]);
    e.call(SAVE_BYTES, &args![buffer, this.addr() + 0x2c, 4u32, 0u32]);
    let first = e.mem.u32(this.addr() + 0x28);
    e.call(SAVE_FORM_ID, &args![buffer, first, 0u32]);
    let second = e.mem.u32(this.addr() + 0x30);
    e.call(SAVE_FORM_ID, &args![buffer, second, 0u32]);
}

// Translated from 008d7420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; its body is the load matching `008d73b0`: the
/// base part (`006dc910`), the words at +0x24 and +0x2c, the form read as a
/// form ID and cast (`TESObjectSTAT`) into the word at +0x28, and the form
/// ID of the one at +0x30 into its word.
pub fn fn_008d7420(e: &mut Engine, this: Ptr, buffer: Ptr) {
    e.call(BASE_LOAD, &args![this, buffer]);
    e.call(LOAD_BYTES, &args![buffer, this.addr() + 0x24, 4u32]);
    e.call(LOAD_BYTES, &args![buffer, this.addr() + 0x2c, 4u32]);
    let form_id = e.call(LOAD_FORM_ID, &args![buffer]).u32();
    let form = form_cast(e, form_id, TYPE_TES_OBJECT_STAT);
    e.mem.set_u32(this.addr() + 0x28, form.addr());
    e.call(LOAD_FORM_ID_OV2, &args![buffer, this.addr() + 0x30]);
}

// Translated from 008d74a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; its body does for the word at +0x30 of the
/// object `008d7420` loads what `008d7340` does for a `DetectionEvent`:
/// form ID to `TESObjectREFR` (or null), marked targeted.
pub fn fn_008d74a0(e: &mut Engine, this: Ptr) {
    resolve_targeted_reference(e, this.addr() + 0x30);
}

// Translated from 008d7510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::HighProcess` (Xbox PDB): the constructor. Runs the
/// `MiddleHighProcess` constructor, installs the vtable, constructs the
/// embedded lists, arrays and `NiPointer`s, allocates the five detection
/// lists, draws two random timers from game settings and sets every field
/// to its starting value; the detection counter global is read, incremented,
/// and the process's own share is that value modulo 12. C++ exception
/// unwinding (the `try` states that destroy the sub-objects) is not
/// translated. Returns `this`.
pub fn high_process_high_process(e: &mut Engine, this: Ptr<HighProcess>) -> Ptr<HighProcess> {
    let base = this.addr();
    e.call(BASE_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(base, HIGH_PROCESS_VTABLE);
    // The four `StartCombatStates` lists: AggroList, GroupsToHelpList,
    // TargetToAddList, SpectatorList.
    for offset in [0x274u32, 0x27c, 0x284, 0x28c] {
        e.call(LIST_CONSTRUCTOR, &args![base + offset]);
    }
    e.call(POINT_CONSTRUCTOR, &args![base + 0x300]);
    e.call(
        VECTOR_CONSTRUCT,
        &args![
            base + 0x314,
            0x0cu32,
            2u32,
            SOUND_HANDLE_CONSTRUCTOR,
            SOUND_HANDLE_DESTRUCTOR
        ],
    );
    e.call(
        VECTOR_CONSTRUCT,
        &args![
            base + 0x354,
            4u32,
            2u32,
            ANIM_IDLE_POINTER_CONSTRUCTOR,
            ANIM_IDLE_POINTER_DESTRUCTOR
        ],
    );
    e.call(
        VECTOR_CONSTRUCT,
        &args![
            base + 0x35c,
            4u32,
            2u32,
            KF_MODEL_POINTER_CONSTRUCTOR,
            KF_MODEL_POINTER_DESTRUCTOR
        ],
    );
    e.call(NI_POINTER_INIT, &args![base + 0x380, 0u32]);
    e.call(LIST_CONSTRUCTOR, &args![base + 0x38c]);
    e.call(LIST_CONSTRUCTOR, &args![base + 0x394]);
    e.call(NI_POINTER_INIT, &args![base + 0x434, 0u32]);
    e.call(NI_POINTER_INIT, &args![base + 0x45c, 0u32]);
    e.call(NI_POINTER_INIT, &args![base + 0x460, 0u32]);
    e.call(QUEUE_POINTER_INIT, &args![base + 0x464, 0u32]);
    e.call(QUEUE_POINTER_INIT, &args![base + 0x468, 0u32]);

    e.set(this, HighProcess::iHourPackageEvaluated, -1);
    e.set(this, HighProcess::pNode, Ptr::NULL);
    e.set(this, HighProcess::fDelayTimer, 0.0);
    e.set(this, HighProcess::fDistanceMoved, 0.0);
    e.set(this, HighProcess::fPackageEvalTimer, 0.0);
    e.set(this, HighProcess::fDetectionTimer, 0.0);
    e.set(this, HighProcess::fDetectListTimer, 0.0);
    e.set(this, HighProcess::iLastDetection, -1);
    e.set(this, HighProcess::plastDetected, Ptr::NULL);
    e.set(this, HighProcess::pBoneLOD, Ptr::NULL);
    e.set(this, HighProcess::iLastBoneLOD, 0xffff_fffe);
    e.set(this, HighProcess::sAnimAction, -1);
    e.set(this, HighProcess::pAnimSeq, Ptr::NULL);
    e.set(this, HighProcess::bAutomaticFireAtLeastOne, 0);
    e.set(this, HighProcess::pGreetActor, Ptr::NULL);
    e.set(this, HighProcess::bCheckDeadTalk, 0);
    e.set(this, HighProcess::bPlantedExplosive, 0);
    // `WeaponLastPos` (+0x300) is a copy of three words of a global.
    for word in 0..3 {
        let v = e.mem.u32(DEFAULT_WEAPON_POSITION + 4 * word);
        e.mem.set_u32(base + 0x300 + 4 * word, v);
    }
    e.set(this, HighProcess::fSoundDelay, 0.0);
    e.set(this, HighProcess::bGreetingFlag, 0);
    e.set(this, HighProcess::fGreetingTimer, 0.0);
    e.set(this, HighProcess::bHeadTrack, 1);
    e.set(this, HighProcess::bWeaponAlertDrawn, 0);
    e.set(this, HighProcess::pGreetTopic, Ptr::NULL);
    e.set(this, HighProcess::fIdleTimer, 1.0);
    e.set(this, HighProcess::bDialoguewithPlayer, 0);
    e.set(this, HighProcess::bContinuingPackageforPC, 0);
    e.set(this, HighProcess::fAwarePlayerTimer, 0.0);
    e.set(this, HighProcess::fDetectGreetTimer, 0.0);
    e.set(this, HighProcess::fEvaluateAcquireTimer, 0.0);
    e.set(this, HighProcess::fHoldAttackTimer, 0.0);
    e.set(this, HighProcess::fUseItemTimer, 0.0);
    e.set(this, HighProcess::sShotsFired, 0);
    e.set(this, HighProcess::sShotsToFire, -1);
    e.set(this, HighProcess::sBurstsFired, 0);
    // The five detection lists (8-byte lists allocated on the heap).
    for field in [
        HighProcess::pDetectedActorList,
        HighProcess::pLastSpokeToList,
        HighProcess::pThreadDetectList,
        HighProcess::pActorsWhoDetectMeList,
        HighProcess::pTempActorsWhoDetectMeList,
    ] {
        let list = e.call(OPERATOR_NEW, &args![8u32]).ptr::<()>();
        let list = if list.is_null() {
            list
        } else {
            e.call(LIST_CONSTRUCTOR, &args![list]).ptr::<()>()
        };
        e.set(this, field, list);
    }
    let breath = e.global::<f32>(DEFAULT_BREATH_TIMER);
    e.set(this, HighProcess::fBreathTimer, breath);
    e.set(this, HighProcess::fTalkTimer, 0.0);
    e.set(this, HighProcess::fTurnTime, 0.0);
    e.set(this, HighProcess::fVoiceTimer, 0.0);
    e.set(this, HighProcess::cLastTurnDir, 0);
    e.set(this, HighProcess::eFadeState, 1);
    e.set(this, HighProcess::fFadeAlpha, 0.0);
    e.set(this, HighProcess::fMaxAlpha, 1.0);
    e.set(this, HighProcess::pTeleportFadeRef, Ptr::NULL);
    e.set(this, HighProcess::pMoveToFadeStruct, Ptr::NULL);
    e.set(this, HighProcess::bActivateAnim, 0);
    e.set(this, HighProcess::fScriptPackageEndTime, 0.0);
    // The two random timers: `GetRandomBetween(low setting, high setting)`.
    let timer = random_between_settings(e, IDLE_CHATTER_FIRST, IDLE_CHATTER_SECOND);
    e.set(this, HighProcess::fIdleChatterTimer, timer);
    let timer = random_between_settings(e, CHECK_TO_TALK_FIRST, CHECK_TO_TALK_SECOND);
    e.set(this, HighProcess::fCheckToTalkTimer, timer);
    e.set(this, HighProcess::fHealthBarAlphaValue, 0.0);
    e.set(this, HighProcess::fActorHealthPercentage, 1.0);
    e.set(this, HighProcess::fHealthBarEmittanceValue, 0.0);
    e.call(NI_POINTER_SET, &args![base + 0x380, 0u32]);
    e.set(this, HighProcess::iNumberGuardsPersuing, 0);
    e.set(this, HighProcess::bStop, 0);
    e.set(this, HighProcess::fReEquipArmorTimer, 0.0);
    e.set(this, HighProcess::bUnequippedArmorToSwim, 0);
    e.set(this, HighProcess::iHasHealingSpell, 0xffff_ffff);
    e.set(this, HighProcess::iHasHealingPotion, 0xffff_ffff);
    e.set(this, HighProcess::pLeveledSpellList, Ptr::NULL);
    e.set(this, HighProcess::cLastTurn, 0);
    e.set(this, HighProcess::bCurrentlyReanimating, 0);
    for i in 0..6 {
        e.mem.set_u32(base + HEAD_TRACKING_TARGETS + 4 * i, 0);
        e.mem.set_u8(base + HEAD_TRACKING_TARGET_FLAGS + i, 0);
    }
    e.set(this, HighProcess::pLastTarget, Ptr::NULL);
    e.set(this, HighProcess::bForceRotate, 0);
    e.set(this, HighProcess::fDetectionModifer, 0.0);
    e.set(this, HighProcess::fDetectionModifierTimer, 0.0);
    e.set(this, HighProcess::bSayGoodByePlayer, 0);
    e.set(this, HighProcess::pLipSynicAnim, Ptr::NULL);
    e.set(this, HighProcess::bWaitingForLipFile, 0);
    e.set(this, HighProcess::pDialogTarget, Ptr::NULL);
    e.set(this, HighProcess::bSkippedUpdate, 0);
    e.set(this, HighProcess::bLipFileFailed, 0);
    e.set(this, HighProcess::pCurrentMuzzleFlash, Ptr::NULL);
    e.set(this, HighProcess::ePostAnimActon, 0);
    e.set(this, HighProcess::fRadiationTimer, 0.0);
    e.set(this, HighProcess::fCachedActorHeight, 0.0);
    e.set(this, HighProcess::fHighestRadiation, 0.0);
    e.set(this, HighProcess::pActorValueCache, Ptr::NULL);
    e.set(this, HighProcess::pIdleToPlay, Ptr::NULL);
    e.set(this, HighProcess::eSpecialIdleType, 2);
    e.call(NI_POINTER_SET, &args![base + 0x354, 0u32]);
    e.call(NI_POINTER_SET, &args![base + 0x358, 0u32]);
    e.call(KF_MODEL_POINTER_SET, &args![base + 0x35c, 0u32]);
    e.call(KF_MODEL_POINTER_SET, &args![base + 0x360, 0u32]);
    let head_track = e.global::<f32>(DEFAULT_HEAD_TRACK_TIMER);
    e.mem.set_f32(base + HEAD_TRACK_TARGET_TIMER, head_track);
    e.set(this, HighProcess::pDialogTarget, Ptr::NULL);
    e.set(this, HighProcess::bEvaluateDetection, 0);
    e.set(this, HighProcess::bProcessGreetSayTo, 0);
    e.set(this, HighProcess::bLipQuequed, 0);
    e.set(this, HighProcess::fLightLevel, 0.0);
    e.set(this, HighProcess::fLightLevelTimer, 0.0);
    e.set(this, HighProcess::bNeedTalkPlayer, 0);
    e.set(this, HighProcess::fTakeBackTimer, head_track);
    // The detection counter: this process takes the current value and the
    // global advances; the share is the value modulo 12.
    let counter = e.global::<u32>(DETECTION_COUNTER);
    e.set(this, HighProcess::iDetectionCounter, counter);
    e.set_global(DETECTION_COUNTER, counter.wrapping_add(1));
    e.set(this, HighProcess::iDetectionCounter, counter % 12);
    e.set(this, HighProcess::pActorsGeneratedDetectionEvent, Ptr::NULL);
    e.set(this, HighProcess::bHiding, 0);
    e.set(this, HighProcess::pListOfAvoidAreas, Ptr::NULL);
    e.set(this, HighProcess::fAvoidWaitTimer, 0.0);
    e.set(this, HighProcess::bFinishingCombatPackage, 0);
    e.call(STORE_WORD_AT_0X28, &args![this, 0u32]);
    e.set(this, HighProcess::bIsDoingSayTo, 0);
    e.set(this, HighProcess::pSubtitleVoice, Ptr::NULL);
    e.set(this, HighProcess::pPathLookAtTarget, Ptr::NULL);
    e.call(NI_POINTER_SET, &args![base + 0x434, 0u32]);
    let setting = e.call(SETTING_FLOAT, &args![CLEAR_TALK_TO_LIST]).u32();
    let clear_talk_to = e.mem.f32(setting);
    e.set(this, HighProcess::fClearTalkToListTimer, clear_talk_to);
    e.set(this, HighProcess::m_pSayToDialogueTopic, Ptr::NULL);
    this
}

/// `GetRandomBetween(*first, *second)` of two game settings, as the
/// constructor calls it: the second setting is read first, then the first,
/// then the call.
fn random_between_settings(e: &mut Engine, first: u32, second: u32) -> f32 {
    let second_pointer = e.call(SETTING_FLOAT, &args![second]).u32();
    let second_value = e.mem.f32(second_pointer);
    let first_pointer = e.call(SETTING_FLOAT, &args![first]).u32();
    let first_value = e.mem.f32(first_pointer);
    e.call(RANDOM_BETWEEN, &args![first_value, second_value])
        .f32()
}

// Translated from 008d7dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::SetItemBeingUsed` (Xbox PDB) stores its argument in
/// `pItemBeingUsed (TESBoundObject*)` at +0x34.
pub fn low_process_set_item_being_used(e: &mut Engine, this: Ptr<LowProcess>, item: Ptr) {
    e.set(this, LowProcess::pItemBeingUsed, item);
}

// Translated from 008d7e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; its body sets (`on` nonzero) or clears the
/// bits of `mask` in `LowProcess::m_uFlags` (+0x30).
pub fn fn_008d7e00(e: &mut Engine, this: Ptr<LowProcess>, on: u8, mask: u8) {
    let flags = e.get(this, LowProcess::m_uFlags);
    let flags = if on != 0 { flags | mask } else { flags & !mask };
    e.set(this, LowProcess::m_uFlags, flags);
}

// Translated from 008d7e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; its body tests whether any bit of `mask` is set
/// in `LowProcess::m_uFlags` (+0x30).
pub fn fn_008d7e60(e: &mut Engine, this: Ptr<LowProcess>, mask: u8) -> bool {
    e.get(this, LowProcess::m_uFlags) & mask != 0
}

// Translated from 008d7de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::SetCurrentActionComplete` (Xbox PDB) sets or clears flag bit
/// 0x02.
pub fn low_process_set_current_action_complete(e: &mut Engine, this: Ptr<LowProcess>, on: u8) {
    fn_008d7e00(e, this, on, FLAG_ACTION_COMPLETE);
}

// Translated from 008d7e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::GetCurrentActionComplete` (Xbox PDB) tests flag bit 0x02.
pub fn low_process_get_current_action_complete(e: &mut Engine, this: Ptr<LowProcess>) -> bool {
    fn_008d7e60(e, this, FLAG_ACTION_COMPLETE)
}

// Translated from 008d7e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::IsAFollower` (Xbox PDB) tests flag bit 0x10.
pub fn low_process_is_a_follower(e: &mut Engine, this: Ptr<LowProcess>) -> bool {
    fn_008d7e60(e, this, FLAG_FOLLOWER)
}

// Translated from 008d7ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::SetFollower` (Xbox PDB) sets or clears flag bit 0x10.
pub fn low_process_set_follower(e: &mut Engine, this: Ptr<LowProcess>, on: u8) {
    fn_008d7e00(e, this, on, FLAG_FOLLOWER);
}

// Translated from 008d7ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::GetIsAggressor` (Xbox PDB) tests flag bit 0x04.
pub fn low_process_get_is_aggressor(e: &mut Engine, this: Ptr<LowProcess>) -> bool {
    fn_008d7e60(e, this, FLAG_AGGRESSOR)
}

// Translated from 008d7ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::SetIsAggressor` (Xbox PDB) sets or clears flag bit 0x04.
pub fn low_process_set_is_aggressor(e: &mut Engine, this: Ptr<LowProcess>, on: u8) {
    fn_008d7e00(e, this, on, FLAG_AGGRESSOR);
}

// Translated from 008d7f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::SetEssentialDownTimer` (Xbox PDB) stores its argument in
/// `fEssentialDownTimer` at +0xa4.
pub fn low_process_set_essential_down_timer(e: &mut Engine, this: Ptr<LowProcess>, value: f32) {
    e.set(this, LowProcess::fEssentialDownTimer, x87_quiet(value));
}

// Translated from 008d7f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; its body starts a `CombatTimer`: the start
/// time is the game clock `float` that `00435dd0` returns, the delay is the
/// argument.
pub fn fn_008d7f40(e: &mut Engine, this: Ptr<CombatTimer>, delay: f32) {
    let now = e.call(CLOCK, &args![]).f32();
    e.set(this, CombatTimer::fStartTime, now);
    e.set(this, CombatTimer::fDelay, x87_quiet(delay));
}

// Translated from 008d7f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::SetCombatDelayTimer` (Xbox PDB) starts the `CombatTimer` at
/// +0x38 with the given delay.
pub fn low_process_set_combat_delay_timer(e: &mut Engine, this: Ptr<LowProcess>, delay: f32) {
    fn_008d7f40(e, Ptr::new(this.addr() + 0x38), delay);
}

// Translated from 008d7f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; its body says whether a `CombatTimer` has run
/// out: the game clock (`00435dd0`) minus the start time is greater than the
/// delay (false for a NaN). The difference is computed in `f64`, the x87
/// code's extended precision.
pub fn fn_008d7f80(e: &mut Engine, this: Ptr<CombatTimer>) -> bool {
    let now = e.call(CLOCK, &args![]).f64();
    let elapsed = now - f64::from(e.get(this, CombatTimer::fStartTime));
    f64::from(e.get(this, CombatTimer::fDelay)) < elapsed
}

// Translated from 008d7f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::CheckCombatDelayTimer` (Xbox PDB): whether the
/// `CombatTimer` at +0x38 has run out.
pub fn low_process_check_combat_delay_timer(e: &mut Engine, this: Ptr<LowProcess>) -> bool {
    fn_008d7f80(e, Ptr::new(this.addr() + 0x38))
}

// Translated from 008d7fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::IsTargetActivated` (Xbox PDB) tests flag bit 0x01.
pub fn low_process_is_target_activated(e: &mut Engine, this: Ptr<LowProcess>) -> bool {
    fn_008d7e60(e, this, FLAG_TARGET_ACTIVATED)
}

// Translated from 008d7fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::SetTargetActivated` (Xbox PDB) sets or clears flag bit 0x01.
pub fn low_process_set_target_activated(e: &mut Engine, this: Ptr<LowProcess>, on: u8) {
    fn_008d7e00(e, this, on, FLAG_TARGET_ACTIVATED);
}

// Translated from 008d8000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::IsPackageDoneOnce` (Xbox PDB) tests flag bit 0x20.
pub fn low_process_is_package_done_once(e: &mut Engine, this: Ptr<LowProcess>) -> bool {
    fn_008d7e60(e, this, FLAG_PACKAGE_DONE_ONCE)
}

// Translated from 008d8020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::SetPackageDoneOnce` (Xbox PDB) sets or clears flag bit 0x20.
pub fn low_process_set_package_done_once(e: &mut Engine, this: Ptr<LowProcess>, on: u8) {
    fn_008d7e00(e, this, on, FLAG_PACKAGE_DONE_ONCE);
}

// Translated from 008d8040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::SetGenericLocation` (Xbox PDB) stores its argument in
/// `pGenericLocation (TESObjectREFR*)` at +0x44.
pub fn low_process_set_generic_location(e: &mut Engine, this: Ptr<LowProcess>, location: Ptr) {
    e.set(this, LowProcess::pGenericLocation, location);
}

// Translated from 008d8060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::GetLockedLocation` (Xbox PDB) tests flag bit 0x80.
pub fn low_process_get_locked_location(e: &mut Engine, this: Ptr<LowProcess>) -> bool {
    fn_008d7e60(e, this, FLAG_LOCKED_LOCATION)
}

// Translated from 008d8080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::SetLockedLocation` (Xbox PDB) sets or clears flag bit 0x80.
pub fn low_process_set_locked_location(e: &mut Engine, this: Ptr<LowProcess>, on: u8) {
    fn_008d7e00(e, this, on, FLAG_LOCKED_LOCATION);
}

// Translated from 008d80a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::GetAlert` (Xbox PDB) tests flag bit 0x08.
pub fn low_process_get_alert(e: &mut Engine, this: Ptr<LowProcess>) -> bool {
    fn_008d7e60(e, this, FLAG_ALERT)
}

// Translated from 008d80c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::SetAlert` (Xbox PDB) sets or clears flag bit 0x08.
pub fn low_process_set_alert(e: &mut Engine, this: Ptr<LowProcess>, on: u8) {
    fn_008d7e00(e, this, on, FLAG_ALERT);
}

// Translated from 008d8100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetActiveEffectList` (Xbox PDB) returns
/// `pActiveEffectList (BSSimpleList<ActiveEffect *>*)` at +0x1b8.
pub fn middle_high_process_get_active_effect_list(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
) -> Ptr {
    e.get(this, MiddleHighProcess::pActiveEffectList)
}

// Translated from 008d8120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetCurrentSpell` (Xbox PDB) returns
/// `pCurrentSpell (MagicItem*)` at +0x160.
pub fn middle_high_process_get_current_spell(e: &mut Engine, this: Ptr<MiddleHighProcess>) -> Ptr {
    e.get(this, MiddleHighProcess::pCurrentSpell)
}

// Translated from 008d8140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::SetCurrentSpell` (Xbox PDB) stores its argument in
/// `pCurrentSpell (MagicItem*)` at +0x160.
pub fn middle_high_process_set_current_spell(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    spell: Ptr,
) {
    e.set(this, MiddleHighProcess::pCurrentSpell, spell);
}

// Translated from 008d8160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetDesiredTarget` (Xbox PDB) returns
/// `pDesiredTarget (MagicTarget*)` at +0x1bc.
pub fn middle_high_process_get_desired_target(e: &mut Engine, this: Ptr<MiddleHighProcess>) -> Ptr {
    e.get(this, MiddleHighProcess::pDesiredTarget)
}

// Translated from 008d8180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::SetDesiredTarget` (Xbox PDB) stores its argument in
/// `pDesiredTarget (MagicTarget*)` at +0x1bc.
pub fn middle_high_process_set_desired_target(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
    target: Ptr,
) {
    e.set(this, MiddleHighProcess::pDesiredTarget, target);
}

// Translated from 008d81a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetShouldCheckMagicNode` (Xbox PDB) returns
/// `bCheckMagicNode` at +0x168.
pub fn middle_high_process_get_should_check_magic_node(
    e: &mut Engine,
    this: Ptr<MiddleHighProcess>,
) -> bool {
    e.get(this, MiddleHighProcess::bCheckMagicNode)
}

// Translated from 008d81c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::SetShouldCheckMagicNode` (Xbox PDB) stores the low byte of its argument in `bCheckMagicNode` at +0x168.
pub fn middle_high_process_set_should_check_magic_node(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x168, value);
}

// Translated from 008d81e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetCurrentWeapon` (Xbox PDB) returns `pWeapon (ItemChange*)` at +0x114.
pub fn middle_high_process_get_current_weapon(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(this.addr() + 0x114))
}

// Translated from 008d8200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::IsCurrentWeaponThrown` (Xbox PDB) returns `bWeaponThrown` at +0x126.
pub fn middle_high_process_is_current_weapon_thrown(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0x126) != 0
}

// Translated from 008d8220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::IsCurrentWeaponGrenade` (Xbox PDB) returns `bWeaponGrenade` at +0x124.
pub fn middle_high_process_is_current_weapon_grenade(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0x124) != 0
}

// Translated from 008d8270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::IsHeavyBodyArmorWorn` (Xbox PDB) returns `bHeavyArmorWorn` at +0x127.
pub fn middle_high_process_is_heavy_body_armor_worn(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0x127) != 0
}

// Translated from 008d8290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::IsPowerBodyArmorWorn` (Xbox PDB) returns `bPowerArmorWorn` at +0x128.
pub fn middle_high_process_is_power_body_armor_worn(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0x128) != 0
}

// Translated from 008d82b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::IsPowerArmorHelmetWorn` (Xbox PDB) returns `bPowerArmorHelmet` at +0x129.
pub fn middle_high_process_is_power_armor_helmet_worn(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0x129) != 0
}

// Translated from 008d82d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::HasBackPackWorn` (Xbox PDB) returns `bHasBackPackWorn` at +0x12a.
pub fn middle_high_process_has_back_pack_worn(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0x12a) != 0
}

// Translated from 008d82f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::SetFireNode` (Xbox PDB) stores its argument in `pFireNode (NiAVObject*)` at +0x130.
pub fn middle_high_process_set_fire_node(e: &mut Engine, this: Ptr, value: Ptr) {
    e.mem.set_u32(this.addr() + 0x130, value.addr());
}

// Translated from 008d8330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetAimLooking` (Xbox PDB) returns `fAimLooking` at +0x1d0.
pub fn middle_high_process_get_aim_looking(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + 0x1d0)
}

// Translated from 008d8380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::SetShotsToFire` (Xbox PDB) stores the low byte of its argument in `cShotsToFire` at +0x1d8.
pub fn middle_high_process_set_shots_to_fire(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x1d8, value);
}

// Translated from 008d83a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetShotsToFire` (Xbox PDB) returns `cShotsToFire` at +0x1d8.
pub fn middle_high_process_get_shots_to_fire(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x1d8)
}

// Translated from 008d83c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetAutomaticShotDelay` (Xbox PDB) returns `fAutomaticShotDelay` at +0x1d4.
pub fn middle_high_process_get_automatic_shot_delay(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + 0x1d4)
}

// Translated from 008d8500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::SetAcquireObject` (Xbox PDB) stores its argument in `pAcquireObject (ObjectstoAcquire*)` at +0x64.
pub fn middle_high_process_set_acquire_object(e: &mut Engine, this: Ptr, value: Ptr) {
    e.mem.set_u32(this.addr() + 0x64, value.addr());
}

// Translated from 008d8520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetSavedAcquireObject` (Xbox PDB) returns `pSavedAcquireObject (ObjectstoAcquire*)` at +0x68.
pub fn middle_high_process_get_saved_acquire_object(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(this.addr() + 0x68))
}

// Translated from 008d8540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetKnockState` (Xbox PDB) returns `cKnockState` at +0x13c, sign-extended from a signed byte.
pub fn middle_high_process_get_knock_state(e: &mut Engine, this: Ptr) -> i32 {
    e.mem.u8(this.addr() + 0x13c) as i8 as i32
}

// Translated from 008d8560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::SetKnockState` (Xbox PDB) stores the low byte of its argument in `cKnockState` at +0x13c.
pub fn middle_high_process_set_knock_state(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x13c, value);
}

// Translated from 008d8580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetCurrentPackageSpell` (Xbox PDB) returns `pCurrentPackageSpell (MagicItem*)` at +0x164.
pub fn middle_high_process_get_current_package_spell(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(this.addr() + 0x164))
}

// Translated from 008d85a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::SetCurrentPackageSpell` (Xbox PDB) stores its argument in `pCurrentPackageSpell (MagicItem*)` at +0x164.
pub fn middle_high_process_set_current_package_spell(e: &mut Engine, this: Ptr, value: Ptr) {
    e.mem.set_u32(this.addr() + 0x164, value.addr());
}

// Translated from 008d85c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::SetLastBoundWeapon` (Xbox PDB) stores its argument in `pLastBoundWeapon (TESObjectWEAP*)` at +0x15c.
pub fn middle_high_process_set_last_bound_weapon(e: &mut Engine, this: Ptr, value: Ptr) {
    e.mem.set_u32(this.addr() + 0x15c, value.addr());
}

// Translated from 008d85e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetLastBoundWeapon` (Xbox PDB) returns `pLastBoundWeapon (TESObjectWEAP*)` at +0x15c.
pub fn middle_high_process_get_last_bound_weapon(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(this.addr() + 0x15c))
}

// Translated from 008d8600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetShouldRefreshMagicShadersEffects` (Xbox PDB) returns `bRefreshMagicShaderEffects` at +0x17c.
pub fn middle_high_process_get_should_refresh_magic_shaders_effects(
    e: &mut Engine,
    this: Ptr,
) -> bool {
    e.mem.u8(this.addr() + 0x17c) != 0
}

// Translated from 008d8620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::SetAlphaMult` (Xbox PDB) stores its argument in `fAlphaMult` at +0x170.
pub fn middle_high_process_set_alpha_mult(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_f32(this.addr() + 0x170, x87_quiet(value));
}

// Translated from 008d8640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::SetScriptRefractionPower` (Xbox PDB) stores its argument in `fScriptRefractPower` at +0x174.
pub fn middle_high_process_set_script_refraction_power(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_f32(this.addr() + 0x174, x87_quiet(value));
}

// Translated from 008d8660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetIdleDoneOnce` (Xbox PDB) returns `bDoneOnce` at +0xe0.
pub fn middle_high_process_get_idle_done_once(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0xe0) != 0
}

// Translated from 008d86c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetPickpocketed` (Xbox PDB) returns `bPickPocketed` at +0x188.
pub fn middle_high_process_get_pickpocketed(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0x188) != 0
}

// Translated from 008d86e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::SetPickpocketed` (Xbox PDB) stores the low byte of its argument in `bPickPocketed` at +0x188.
pub fn middle_high_process_set_pickpocketed(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x188, value);
}

// Translated from 008d8700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::SetForceNextUpdate` (Xbox PDB) stores the low byte of its argument in `bForceNextUpdate` at +0x18d.
pub fn middle_high_process_set_force_next_update(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x18d, value);
}

/// The `ActorPackage` that `MiddleHighProcess` embeds as `RunOncePackage`
/// (Xbox PDB) at +0xe4: the functions below pass its address on.
const RUN_ONCE_PACKAGE_OFFSET: u32 = 0xe4;
/// `spCharController` (`NiPointer<bhkCharacterController>`, Xbox PDB) at +0x138.
const CHAR_CONTROLLER_OFFSET: u32 = 0x138;
/// Reads the first word of an object (`00559450`; the `NiPointer` getter).
const READ_FIRST_WORD: u32 = 0x0055_9450;
/// Stores its argument at +0xc of the object (`0041fd00`).
const WRITE_WORD_AT_0C: u32 = 0x0041_fd00;
/// Reads the word at +0xc of the object (`0084e3a0`).
const READ_WORD_AT_0C: u32 = 0x0084_e3a0;
/// Adds its argument to the word at +0xc of the object (`0049bb50`).
const ADD_TO_WORD_AT_0C: u32 = 0x0049_bb50;
/// Reads the word at +4 of the object (`00726070`).
const READ_WORD_AT_04: u32 = 0x0072_6070;
/// `NiPointer<..>::operator=(const NiPointer&)` (`006e5cc0`): takes the
/// address of the source pointer.
const NI_POINTER_ASSIGN: u32 = 0x006e_5cc0;
/// `NiPointer` destructor (`0045cec0`): releases the pointer it holds.
const NI_POINTER_DESTROY: u32 = 0x0045_cec0;
/// The double `pi / 180` (`01023128`) that `AddGunDrift` multiplies by.
const DEGREES_TO_RADIANS: u32 = 0x0102_3128;

// Translated from 008d8240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::IsCurrentWeaponGrenadeOrMine` (Xbox PDB): true when
/// any of `bWeaponGrenade` (+0x124), `bWeaponMine` (+0x125) or
/// `bWeaponThrown` (+0x126) is set.
pub fn middle_high_process_is_current_weapon_grenade_or_mine(e: &mut Engine, this: Ptr) -> bool {
    let grenade = e.mem.u8(this.addr() + 0x124) as u32;
    let mine = e.mem.u8(this.addr() + 0x125) as u32;
    let thrown = e.mem.u8(this.addr() + 0x126) as u32;
    (grenade | mine | thrown) != 0
}

// Translated from 008d8310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::IsRunningRunOnce` (Xbox PDB): true when the virtual
/// call at vtable offset 0x20c returns non-zero.
pub fn middle_high_process_is_running_run_once(e: &mut Engine, this: Ptr) -> bool {
    e.vcall(this.addr(), 0x20c, &args![]).u32() != 0
}

// Translated from 008d8350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::AddGunDrift` (Xbox PDB): subtracts
/// `amount * (pi / 180)` (the double at `01023128`) from `fAimLooking`
/// (+0x1d0), rounding to `float` at the store.
pub fn middle_high_process_add_gun_drift(e: &mut Engine, this: Ptr, amount: f32) {
    let factor = e.global::<f64>(DEGREES_TO_RADIANS);
    let current = e.mem.f32(this.addr() + 0x1d0) as f64;
    let result = current - amount as f64 * factor;
    e.mem.set_f32(this.addr() + 0x1d0, result as f32);
}

// Translated from 008d83e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map: returns `00559450(&RunOncePackage)`, the first word
/// of the embedded package at +0xe4.
pub fn fn_008d83e0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(
        READ_FIRST_WORD,
        &args![this.addr() + RUN_ONCE_PACKAGE_OFFSET],
    )
    .u32()
}

// Translated from 008d8400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::SetRunOnceProcedureIndex` (Xbox PDB): calls
/// `0041fd00(&RunOncePackage, value)`.
pub fn middle_high_process_set_run_once_procedure_index(e: &mut Engine, this: Ptr, value: u32) {
    e.call(
        WRITE_WORD_AT_0C,
        &args![this.addr() + RUN_ONCE_PACKAGE_OFFSET, value],
    );
}

// Translated from 008d8420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetRunOnceProcedureIndex` (Xbox PDB): returns
/// `0084e3a0(&RunOncePackage)`.
pub fn middle_high_process_get_run_once_procedure_index(e: &mut Engine, this: Ptr) -> u32 {
    e.call(
        READ_WORD_AT_0C,
        &args![this.addr() + RUN_ONCE_PACKAGE_OFFSET],
    )
    .u32()
}

// Translated from 008d8440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::AddToRunOnceProcedureIndex` (Xbox PDB): calls
/// `0049bb50(&RunOncePackage, value)`.
pub fn middle_high_process_add_to_run_once_procedure_index(e: &mut Engine, this: Ptr, value: u32) {
    e.call(
        ADD_TO_WORD_AT_0C,
        &args![this.addr() + RUN_ONCE_PACKAGE_OFFSET, value],
    );
}

// Translated from 008d8460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetRunOncePackageInstanceData` (Xbox PDB): returns
/// `00726070(&RunOncePackage)`.
pub fn middle_high_process_get_run_once_package_instance_data(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(
        e.call(
            READ_WORD_AT_04,
            &args![this.addr() + RUN_ONCE_PACKAGE_OFFSET],
        )
        .u32(),
    )
}

// Translated from 008d8480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::GetCharController` (Xbox PDB): returns
/// `00559450(&spCharController)`, the controller held by the `NiPointer`
/// at +0x138.
pub fn middle_high_process_get_char_controller(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(
        e.call(
            READ_FIRST_WORD,
            &args![this.addr() + CHAR_CONTROLLER_OFFSET],
        )
        .u32(),
    )
}

// Translated from 008d84a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::SetCharController` (Xbox PDB): assigns the
/// `NiPointer` at +0x138 from a temporary `NiPointer` that holds `value`
/// (`006e5cc0`), then destroys the temporary (`0045cec0`). C++ exception
/// unwinding of the temporary is not translated.
pub fn middle_high_process_set_char_controller(e: &mut Engine, this: Ptr, value: Ptr) {
    e.with_stack(4, |e, temporary| {
        e.mem.set_u32(temporary.addr(), value.addr());
        e.call(
            NI_POINTER_ASSIGN,
            &args![this.addr() + CHAR_CONTROLLER_OFFSET, temporary],
        );
        e.call(NI_POINTER_DESTROY, &args![temporary]);
    });
}

// Translated from 008d8680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::ClearFaceAnimationData` (Xbox PDB) sets
/// `m_pFaceAnimationData` (+0x178) to null.
pub fn middle_high_process_clear_face_animation_data(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr() + 0x178, 0);
}

// Translated from 008d86a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiddleHighProcess::SetRefreshFlareFlags` (Xbox PDB) sets
/// `bRefreshFlareFlags` (+0x180) to 1.
pub fn middle_high_process_set_refresh_flare_flags(e: &mut Engine, this: Ptr) {
    e.mem.set_u8(this.addr() + 0x180, 1);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x008d6fc0,
            fn_008d6fc0(Ptr<DetectionState>, Ptr<DetectionState>)
        ),
        entry!(
            0x008d7070,
            detection_state_save_game(Ptr<DetectionState>, Ptr)
        ),
        entry!(
            0x008d7140,
            detection_state_load_game(Ptr<DetectionState>, Ptr)
        ),
        entry!(0x008d7220, fn_008d7220(Ptr)),
        entry!(
            0x008d7270,
            detection_event_save_game(Ptr<DetectionEvent>, Ptr)
        ),
        entry!(0x008d72e0, fn_008d72e0(Ptr<DetectionEvent>, Ptr)),
        entry!(0x008d7340, fn_008d7340(Ptr<DetectionEvent>)),
        entry!(0x008d73b0, fn_008d73b0(Ptr, Ptr)),
        entry!(0x008d7420, fn_008d7420(Ptr, Ptr)),
        entry!(0x008d74a0, fn_008d74a0(Ptr)),
        entry!(
            0x008d7510,
            high_process_high_process(Ptr<HighProcess>) -> Ptr<HighProcess>
        ),
        entry!(
            0x008d7dc0,
            low_process_set_item_being_used(Ptr<LowProcess>, Ptr)
        ),
        entry!(
            0x008d7de0,
            low_process_set_current_action_complete(Ptr<LowProcess>, u8)
        ),
        entry!(0x008d7e00, fn_008d7e00(Ptr<LowProcess>, u8, u8)),
        entry!(
            0x008d7e40,
            low_process_get_current_action_complete(Ptr<LowProcess>) -> bool
        ),
        entry!(0x008d7e60, fn_008d7e60(Ptr<LowProcess>, u8) -> bool),
        entry!(
            0x008d7e80,
            low_process_is_a_follower(Ptr<LowProcess>) -> bool
        ),
        entry!(0x008d7ea0, low_process_set_follower(Ptr<LowProcess>, u8)),
        entry!(
            0x008d7ec0,
            low_process_get_is_aggressor(Ptr<LowProcess>) -> bool
        ),
        entry!(
            0x008d7ee0,
            low_process_set_is_aggressor(Ptr<LowProcess>, u8)
        ),
        entry!(
            0x008d7f00,
            low_process_set_essential_down_timer(Ptr<LowProcess>, f32)
        ),
        entry!(
            0x008d7f20,
            low_process_set_combat_delay_timer(Ptr<LowProcess>, f32)
        ),
        entry!(0x008d7f40, fn_008d7f40(Ptr<CombatTimer>, f32)),
        entry!(
            0x008d7f60,
            low_process_check_combat_delay_timer(Ptr<LowProcess>) -> bool
        ),
        entry!(0x008d7f80, fn_008d7f80(Ptr<CombatTimer>) -> bool),
        entry!(
            0x008d7fc0,
            low_process_is_target_activated(Ptr<LowProcess>) -> bool
        ),
        entry!(
            0x008d7fe0,
            low_process_set_target_activated(Ptr<LowProcess>, u8)
        ),
        entry!(
            0x008d8000,
            low_process_is_package_done_once(Ptr<LowProcess>) -> bool
        ),
        entry!(
            0x008d8020,
            low_process_set_package_done_once(Ptr<LowProcess>, u8)
        ),
        entry!(
            0x008d8040,
            low_process_set_generic_location(Ptr<LowProcess>, Ptr)
        ),
        entry!(
            0x008d8060,
            low_process_get_locked_location(Ptr<LowProcess>) -> bool
        ),
        entry!(
            0x008d8080,
            low_process_set_locked_location(Ptr<LowProcess>, u8)
        ),
        entry!(0x008d80a0, low_process_get_alert(Ptr<LowProcess>) -> bool),
        entry!(0x008d80c0, low_process_set_alert(Ptr<LowProcess>, u8)),
        entry!(
            0x008d8100,
            middle_high_process_get_active_effect_list(Ptr<MiddleHighProcess>) -> Ptr
        ),
        entry!(
            0x008d8120,
            middle_high_process_get_current_spell(Ptr<MiddleHighProcess>) -> Ptr
        ),
        entry!(
            0x008d8140,
            middle_high_process_set_current_spell(Ptr<MiddleHighProcess>, Ptr)
        ),
        entry!(
            0x008d8160,
            middle_high_process_get_desired_target(Ptr<MiddleHighProcess>) -> Ptr
        ),
        entry!(
            0x008d8180,
            middle_high_process_set_desired_target(Ptr<MiddleHighProcess>, Ptr)
        ),
        entry!(
            0x008d81a0,
            middle_high_process_get_should_check_magic_node(Ptr<MiddleHighProcess>) -> bool
        ),
        entry!(
            0x00407800,
            middle_high_process_set_saved_acquire_object(Ptr, Ptr)
        ),
        entry!(0x004727d0, high_process_get_post_animation_actions(Ptr) -> u32),
        entry!(0x00502430, middle_high_process_get_fire_node(Ptr) -> Ptr),
        entry!(
            0x00502c90,
            middle_low_process_set_package_evaluate_hour(Ptr, i32)
        ),
        entry!(
            0x00505e50,
            middle_high_process_set_last_idle_played(Ptr, Ptr)
        ),
        entry!(0x005075f0, low_process_set_death_time(Ptr, f32)),
        entry!(0x0051f510, high_process_get_dialog_target(Ptr) -> Ptr),
        entry!(0x0051f530, fn_0051f530(Ptr, Ptr)),
        entry!(0x0051f550, middle_high_process_get_head_node(Ptr) -> Ptr),
        entry!(0x00521630, middle_high_process_get_last_hit_data(Ptr) -> Ptr),
        entry!(0x00521650, fn_00521650(Ptr, u32)),
        entry!(0x00521670, fn_00521670(Ptr, u32)),
        entry!(0x00521690, fn_00521690(Ptr, Ptr)),
        entry!(0x005216b0, fn_005216b0(Ptr, u32)),
        entry!(0x005216d0, fn_005216d0(Ptr, u32)),
        entry!(0x005216f0, fn_005216f0(Ptr, u32)),
        entry!(0x00521710, middle_high_process_set_head_anims(Ptr, Ptr)),
        entry!(0x005224a0, middle_high_process_get_face_node(Ptr) -> Ptr),
        entry!(0x00566950, middle_high_process_get_force_next_update(Ptr) -> bool),
        entry!(0x005a29b0, middle_high_process_get_last_idle_played(Ptr) -> Ptr),
        entry!(
            0x005a8060,
            low_process_set_number_of_items_activated(Ptr, i32)
        ),
        entry!(0x005c8a30, middle_high_process_set_bs_bound(Ptr, Ptr)),
        entry!(0x005ce8f0, middle_high_process_is_current_weapon_mine(Ptr) -> bool),
        entry!(0x005f7590, middle_high_process_get_current_ammo(Ptr) -> Ptr),
        entry!(
            0x005f8bd0,
            middle_high_process_set_commanding_actor(Ptr, Ptr)
        ),
        entry!(0x005f9b50, middle_high_process_get_commanding_actor(Ptr) -> Ptr),
        entry!(0x00602150, middle_high_process_get_animation(Ptr) -> Ptr),
        entry!(0x006214b0, high_process_get_current_process_idle(Ptr) -> Ptr),
        entry!(0x006286d0, low_process_get_number_of_items_activated(Ptr) -> i32),
        entry!(0x00645c40, middle_high_process_get_script_refraction_power(Ptr) -> f32),
        entry!(0x00649b80, middle_high_process_get_is_summoned_creature(Ptr) -> bool),
        entry!(0x006546c0, middle_high_process_set_idle_done_once(Ptr, u8)),
        entry!(0x006733e0, low_process_get_second_generic_location(Ptr) -> Ptr),
        entry!(
            0x00673400,
            low_process_set_second_generic_location(Ptr, Ptr)
        ),
        entry!(
            0x007ac9d0,
            middle_high_process_set_weapon_enchantment_visuals(Ptr, Ptr)
        ),
        entry!(0x008041a0, low_process_get_generic_location(Ptr) -> Ptr),
        entry!(0x00812870, low_process_get_essential_down_timer(Ptr) -> f32),
        entry!(
            0x0087d7d0,
            middle_high_process_set_creature_lip_synch_start_time(Ptr, u32)
        ),
        entry!(0x006ec0d0, bst_common_message_queue_actor_pathing_message_try_push(Ptr, Ptr) -> bool),
        entry!(0x006ec390, bst_common_message_queue_actor_pathing_message_try_pop(Ptr, Ptr) -> bool),
        entry!(
            0x008d81c0,
            middle_high_process_set_should_check_magic_node(Ptr, u8)
        ),
        entry!(0x008d81e0, middle_high_process_get_current_weapon(Ptr) -> Ptr),
        entry!(0x008d8200, middle_high_process_is_current_weapon_thrown(Ptr) -> bool),
        entry!(0x008d8220, middle_high_process_is_current_weapon_grenade(Ptr) -> bool),
        entry!(0x008d8270, middle_high_process_is_heavy_body_armor_worn(Ptr) -> bool),
        entry!(0x008d8290, middle_high_process_is_power_body_armor_worn(Ptr) -> bool),
        entry!(0x008d82b0, middle_high_process_is_power_armor_helmet_worn(Ptr) -> bool),
        entry!(0x008d82d0, middle_high_process_has_back_pack_worn(Ptr) -> bool),
        entry!(0x008d82f0, middle_high_process_set_fire_node(Ptr, Ptr)),
        entry!(0x008d8330, middle_high_process_get_aim_looking(Ptr) -> f32),
        entry!(0x008d8380, middle_high_process_set_shots_to_fire(Ptr, u8)),
        entry!(0x008d83a0, middle_high_process_get_shots_to_fire(Ptr) -> u8),
        entry!(0x008d83c0, middle_high_process_get_automatic_shot_delay(Ptr) -> f32),
        entry!(0x008d8500, middle_high_process_set_acquire_object(Ptr, Ptr)),
        entry!(0x008d8520, middle_high_process_get_saved_acquire_object(Ptr) -> Ptr),
        entry!(0x008d8540, middle_high_process_get_knock_state(Ptr) -> i32),
        entry!(0x008d8560, middle_high_process_set_knock_state(Ptr, u8)),
        entry!(0x008d8580, middle_high_process_get_current_package_spell(Ptr) -> Ptr),
        entry!(
            0x008d85a0,
            middle_high_process_set_current_package_spell(Ptr, Ptr)
        ),
        entry!(
            0x008d85c0,
            middle_high_process_set_last_bound_weapon(Ptr, Ptr)
        ),
        entry!(0x008d85e0, middle_high_process_get_last_bound_weapon(Ptr) -> Ptr),
        entry!(0x008d8600, middle_high_process_get_should_refresh_magic_shaders_effects(Ptr) -> bool),
        entry!(0x008d8620, middle_high_process_set_alpha_mult(Ptr, f32)),
        entry!(
            0x008d8640,
            middle_high_process_set_script_refraction_power(Ptr, f32)
        ),
        entry!(0x008d8660, middle_high_process_get_idle_done_once(Ptr) -> bool),
        entry!(0x008d86c0, middle_high_process_get_pickpocketed(Ptr) -> bool),
        entry!(0x008d86e0, middle_high_process_set_pickpocketed(Ptr, u8)),
        entry!(
            0x008d8700,
            middle_high_process_set_force_next_update(Ptr, u8)
        ),
        entry!(0x008d8240, middle_high_process_is_current_weapon_grenade_or_mine(Ptr) -> bool),
        entry!(0x008d8310, middle_high_process_is_running_run_once(Ptr) -> bool),
        entry!(0x008d8350, middle_high_process_add_gun_drift(Ptr, f32)),
        entry!(0x008d83e0, fn_008d83e0(Ptr) -> u32),
        entry!(
            0x008d8400,
            middle_high_process_set_run_once_procedure_index(Ptr, u32)
        ),
        entry!(0x008d8420, middle_high_process_get_run_once_procedure_index(Ptr) -> u32),
        entry!(
            0x008d8440,
            middle_high_process_add_to_run_once_procedure_index(Ptr, u32)
        ),
        entry!(0x008d8460, middle_high_process_get_run_once_package_instance_data(Ptr) -> Ptr),
        entry!(0x008d8480, middle_high_process_get_char_controller(Ptr) -> Ptr),
        entry!(
            0x008d84a0,
            middle_high_process_set_char_controller(Ptr, Ptr)
        ),
        entry!(
            0x008d8680,
            middle_high_process_clear_face_animation_data(Ptr)
        ),
        entry!(0x008d86a0, middle_high_process_set_refresh_flare_flags(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A zeroed object big enough for every class here (`HighProcess` is
    /// 0x46c bytes).
    fn process(e: &mut Engine) -> Ptr {
        Ptr::new(e.mem.alloc(0x500))
    }

    #[test]
    fn test_middle_high_process_set_saved_acquire_object() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.call(0x00407800, &args![this, Ptr::<()>::new(0x1234_5678)]);
        assert_eq!(e.mem.u32(this.addr() + 0x68), 0x1234_5678);
        assert_eq!(
            e.mem.u32(this.addr() + 0x68 + 8),
            0,
            "only the one field is written"
        );
    }

    #[test]
    fn test_high_process_get_post_animation_actions() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.mem.set_u32(this.addr() + 0x424, 0x1234_5678);
        let r = e.call(0x004727d0, &args![this]);
        assert_eq!(r.u32(), 0x1234_5678);
    }

    #[test]
    fn test_middle_high_process_get_fire_node() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.mem.set_u32(this.addr() + 0x130, 0x1234_5678);
        let r = e.call(0x00502430, &args![this]);
        assert_eq!(r.u32(), 0x1234_5678);
    }

    #[test]
    fn test_middle_low_process_set_package_evaluate_hour() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.call(0x00502c90, &args![this, -5i32]);
        assert_eq!(e.mem.u32(this.addr() + 0xb4), -5i32 as u32);
        assert_eq!(
            e.mem.u32(this.addr() + 0xb4 + 8),
            0,
            "only the one field is written"
        );
    }

    #[test]
    fn test_middle_high_process_set_last_idle_played() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.call(0x00505e50, &args![this, Ptr::<()>::new(0x1234_5678)]);
        assert_eq!(e.mem.u32(this.addr() + 0x10c), 0x1234_5678);
        assert_eq!(
            e.mem.u32(this.addr() + 0x10c + 8),
            0,
            "only the one field is written"
        );
    }

    #[test]
    fn test_low_process_set_death_time() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.call(0x005075f0, &args![this, 2.5f32]);
        assert_eq!(e.mem.f32(this.addr() + 0xa8), 2.5);
        assert_eq!(
            e.mem.u32(this.addr() + 0xa8 + 8),
            0,
            "only the one field is written"
        );
    }

    #[test]
    fn test_high_process_get_dialog_target() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.mem.set_u32(this.addr() + 0x370, 0x1234_5678);
        let r = e.call(0x0051f510, &args![this]);
        assert_eq!(r.u32(), 0x1234_5678);
    }

    #[test]
    fn test_fn_0051f530() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.call(0x0051f530, &args![this, Ptr::<()>::new(0x1234_5678)]);
        assert_eq!(e.mem.u32(this.addr() + 0x370), 0x1234_5678);
        assert_eq!(
            e.mem.u32(this.addr() + 0x370 + 8),
            0,
            "only the one field is written"
        );
    }

    #[test]
    fn test_middle_high_process_get_head_node() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.mem.set_u32(this.addr() + 0x218, 0x1234_5678);
        let r = e.call(0x0051f550, &args![this]);
        assert_eq!(r.u32(), 0x1234_5678);
    }

    #[test]
    fn test_middle_high_process_get_last_hit_data() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.mem.set_u32(this.addr() + 0x240, 0x1234_5678);
        let r = e.call(0x00521630, &args![this]);
        assert_eq!(r.u32(), 0x1234_5678);
    }

    #[test]
    fn test_fn_00521650() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.call(0x00521650, &args![this, 0x1234_5678u32]);
        assert_eq!(e.mem.u32(this.addr() + 0x228), 0x1234_5678);
        assert_eq!(
            e.mem.u32(this.addr() + 0x228 + 8),
            0,
            "only the one field is written"
        );
    }

    #[test]
    fn test_fn_00521670() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.call(0x00521670, &args![this, 0x1234_5678u32]);
        assert_eq!(e.mem.u32(this.addr() + 0x22c), 0x1234_5678);
        assert_eq!(
            e.mem.u32(this.addr() + 0x22c + 8),
            0,
            "only the one field is written"
        );
    }

    #[test]
    fn test_fn_00521690() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.call(0x00521690, &args![this, Ptr::<()>::new(0x1234_5678)]);
        assert_eq!(e.mem.u32(this.addr() + 0x230), 0x1234_5678);
        assert_eq!(
            e.mem.u32(this.addr() + 0x230 + 8),
            0,
            "only the one field is written"
        );
    }

    #[test]
    fn test_fn_005216b0() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.call(0x005216b0, &args![this, 0x1234_5678u32]);
        assert_eq!(e.mem.u32(this.addr() + 0x234), 0x1234_5678);
        assert_eq!(
            e.mem.u32(this.addr() + 0x234 + 8),
            0,
            "only the one field is written"
        );
    }

    #[test]
    fn test_fn_005216d0() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.call(0x005216d0, &args![this, 0x1234_5678u32]);
        assert_eq!(e.mem.u32(this.addr() + 0x238), 0x1234_5678);
        assert_eq!(
            e.mem.u32(this.addr() + 0x238 + 8),
            0,
            "only the one field is written"
        );
    }

    #[test]
    fn test_fn_005216f0() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.call(0x005216f0, &args![this, 0x1234_5678u32]);
        assert_eq!(e.mem.u32(this.addr() + 0x23c), 0x1234_5678);
        assert_eq!(
            e.mem.u32(this.addr() + 0x23c + 8),
            0,
            "only the one field is written"
        );
    }

    #[test]
    fn test_middle_high_process_set_head_anims() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.call(0x00521710, &args![this, Ptr::<()>::new(0x1234_5678)]);
        assert_eq!(e.mem.u32(this.addr() + 0x250), 0x1234_5678);
        assert_eq!(
            e.mem.u32(this.addr() + 0x250 + 8),
            0,
            "only the one field is written"
        );
    }

    #[test]
    fn test_middle_high_process_get_face_node() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.mem.set_u32(this.addr() + 0x248, 0x1234_5678);
        let r = e.call(0x005224a0, &args![this]);
        assert_eq!(r.u32(), 0x1234_5678);
    }

    #[test]
    fn test_middle_high_process_get_force_next_update() {
        let mut e = Engine::new();
        let this = process(&mut e);
        assert!(!e.call(0x00566950, &args![this]).bool());
        e.mem.set_u8(this.addr() + 0x18d, 1);
        assert!(e.call(0x00566950, &args![this]).bool());
    }

    #[test]
    fn test_middle_high_process_get_last_idle_played() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.mem.set_u32(this.addr() + 0x10c, 0x1234_5678);
        let r = e.call(0x005a29b0, &args![this]);
        assert_eq!(r.u32(), 0x1234_5678);
    }

    #[test]
    fn test_low_process_set_number_of_items_activated() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.call(0x005a8060, &args![this, -5i32]);
        assert_eq!(e.mem.u32(this.addr() + 0x58), -5i32 as u32);
        assert_eq!(
            e.mem.u32(this.addr() + 0x58 + 8),
            0,
            "only the one field is written"
        );
    }

    #[test]
    fn test_middle_high_process_set_bs_bound() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.call(0x005c8a30, &args![this, Ptr::<()>::new(0x1234_5678)]);
        assert_eq!(e.mem.u32(this.addr() + 0x224), 0x1234_5678);
        assert_eq!(
            e.mem.u32(this.addr() + 0x224 + 8),
            0,
            "only the one field is written"
        );
    }

    #[test]
    fn test_middle_high_process_is_current_weapon_mine() {
        let mut e = Engine::new();
        let this = process(&mut e);
        assert!(!e.call(0x005ce8f0, &args![this]).bool());
        e.mem.set_u8(this.addr() + 0x125, 1);
        assert!(e.call(0x005ce8f0, &args![this]).bool());
    }

    #[test]
    fn test_middle_high_process_get_current_ammo() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.mem.set_u32(this.addr() + 0x118, 0x1234_5678);
        let r = e.call(0x005f7590, &args![this]);
        assert_eq!(r.u32(), 0x1234_5678);
    }

    #[test]
    fn test_middle_high_process_set_commanding_actor() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.call(0x005f8bd0, &args![this, Ptr::<()>::new(0x1234_5678)]);
        assert_eq!(e.mem.u32(this.addr() + 0x158), 0x1234_5678);
        assert_eq!(
            e.mem.u32(this.addr() + 0x158 + 8),
            0,
            "only the one field is written"
        );
    }

    #[test]
    fn test_middle_high_process_get_commanding_actor() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.mem.set_u32(this.addr() + 0x158, 0x1234_5678);
        let r = e.call(0x005f9b50, &args![this]);
        assert_eq!(r.u32(), 0x1234_5678);
    }

    #[test]
    fn test_middle_high_process_get_animation() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.mem.set_u32(this.addr() + 0x1c0, 0x1234_5678);
        let r = e.call(0x00602150, &args![this]);
        assert_eq!(r.u32(), 0x1234_5678);
    }

    #[test]
    fn test_high_process_get_current_process_idle() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.mem.set_u32(this.addr() + 0x350, 0x1234_5678);
        let r = e.call(0x006214b0, &args![this]);
        assert_eq!(r.u32(), 0x1234_5678);
    }

    #[test]
    fn test_low_process_get_number_of_items_activated() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.mem.set_u32(this.addr() + 0x58, 0x1234_5678);
        let r = e.call(0x006286d0, &args![this]);
        assert_eq!(r.u32(), 0x1234_5678);
    }

    #[test]
    fn test_middle_high_process_get_script_refraction_power() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.mem.set_f32(this.addr() + 0x174, 2.5);
        assert_eq!(e.call(0x00645c40, &args![this]).f32(), 2.5);
    }

    #[test]
    fn test_middle_high_process_get_is_summoned_creature() {
        let mut e = Engine::new();
        let this = process(&mut e);
        assert!(!e.call(0x00649b80, &args![this]).bool());
        e.mem.set_u8(this.addr() + 0x18b, 1);
        assert!(e.call(0x00649b80, &args![this]).bool());
    }

    #[test]
    fn test_middle_high_process_set_idle_done_once() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.call(0x006546c0, &args![this, 1u8]);
        assert_eq!(e.mem.u8(this.addr() + 0xe0), 1);
        assert_eq!(
            e.mem.u32(this.addr() + 0xe0 + 8),
            0,
            "only the one field is written"
        );
    }

    #[test]
    fn test_low_process_get_second_generic_location() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.mem.set_u32(this.addr() + 0x48, 0x1234_5678);
        let r = e.call(0x006733e0, &args![this]);
        assert_eq!(r.u32(), 0x1234_5678);
    }

    #[test]
    fn test_low_process_set_second_generic_location() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.call(0x00673400, &args![this, Ptr::<()>::new(0x1234_5678)]);
        assert_eq!(e.mem.u32(this.addr() + 0x48), 0x1234_5678);
        assert_eq!(
            e.mem.u32(this.addr() + 0x48 + 8),
            0,
            "only the one field is written"
        );
    }

    #[test]
    fn test_middle_high_process_set_weapon_enchantment_visuals() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.call(0x007ac9d0, &args![this, Ptr::<()>::new(0x1234_5678)]);
        assert_eq!(e.mem.u32(this.addr() + 0x16c), 0x1234_5678);
        assert_eq!(
            e.mem.u32(this.addr() + 0x16c + 8),
            0,
            "only the one field is written"
        );
    }

    #[test]
    fn test_low_process_get_generic_location() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.mem.set_u32(this.addr() + 0x44, 0x1234_5678);
        let r = e.call(0x008041a0, &args![this]);
        assert_eq!(r.u32(), 0x1234_5678);
    }

    #[test]
    fn test_low_process_get_essential_down_timer() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.mem.set_f32(this.addr() + 0xa4, 2.5);
        assert_eq!(e.call(0x00812870, &args![this]).f32(), 2.5);
    }

    #[test]
    fn test_middle_high_process_set_creature_lip_synch_start_time() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.call(0x0087d7d0, &args![this, 0x1234_5678u32]);
        assert_eq!(e.mem.u32(this.addr() + 0x1a4), 0x1234_5678);
        assert_eq!(
            e.mem.u32(this.addr() + 0x1a4 + 8),
            0,
            "only the one field is written"
        );
    }

    /// A queue object (lock word at +4, vtable at +0) whose virtual slot at
    /// `slot` is a double answering `answer`; the compare-exchange is the
    /// real operation on memory, and the stub is a no-op.
    fn queue_with_slot(e: &mut Engine, slot: u32, answer: u32) -> Ptr {
        let queue = Ptr::new(e.mem.alloc(8));
        let mut slots = [0u32; 8];
        slots[(slot / 4) as usize] = 0x7000_0000;
        e.put_vtable(0x7100_0000, &slots);
        e.mem.set_u32(queue.addr(), 0x7100_0000);
        e.register(COMPARE_EXCHANGE, |e, a| {
            let old = e.mem.u32(a[0]);
            if old == a[1] {
                e.mem.set_u32(a[0], a[2]);
            }
            old.into_ret()
        });
        e.register(ERROR_NO_OP, |_, _| Ret::default());
        e.register_double(0x7000_0000, move |e, a| {
            // The lock is held while the virtual runs.
            assert_eq!(e.mem.u32(a[0] + 4), 1);
            Ret {
                eax: answer,
                ..Ret::default()
            }
        });
        queue
    }

    #[test]
    fn test_try_push_calls_push_under_the_lock() {
        let mut e = Engine::new();
        let queue = queue_with_slot(&mut e, 0x14, 1);
        e.call_log = Some(vec![]);
        let message = Ptr::<()>::new(0x2222);
        let r = e.call(0x006ec0d0, &args![queue, message]);
        assert!(r.bool());
        assert_eq!(e.mem.u32(queue.addr() + 4), 0, "lock released");
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(0x7000_0000, vec![queue.addr(), 0x2222])));
    }

    #[test]
    fn test_try_push_returns_false_when_locked() {
        let mut e = Engine::new();
        let queue = queue_with_slot(&mut e, 0x14, 1);
        e.mem.set_u32(queue.addr() + 4, 1);
        e.call_log = Some(vec![]);
        let r = e.call(0x006ec0d0, &args![queue, Ptr::<()>::new(0x2222)]);
        assert!(!r.bool());
        assert_eq!(e.mem.u32(queue.addr() + 4), 1, "lock untouched");
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(a, _)| *a == 0x7000_0000));
    }

    #[test]
    fn test_try_pop_calls_pop_under_the_lock() {
        let mut e = Engine::new();
        let queue = queue_with_slot(&mut e, 0x18, 0);
        e.call_log = Some(vec![]);
        let r = e.call(0x006ec390, &args![queue, Ptr::<()>::new(0x3333)]);
        assert!(!r.bool(), "Pop's own answer is passed through");
        assert_eq!(e.mem.u32(queue.addr() + 4), 0);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(0x7000_0000, vec![queue.addr(), 0x3333])));
    }

    #[test]
    fn test_try_pop_returns_false_when_locked() {
        let mut e = Engine::new();
        let queue = queue_with_slot(&mut e, 0x18, 1);
        e.mem.set_u32(queue.addr() + 4, 1);
        assert!(!e
            .call(0x006ec390, &args![queue, Ptr::<()>::new(0x3333)])
            .bool());
    }

    // ---- Tests of 008d6fc0 .. 008d81a0 ----------------------------------------

    use std::cell::RefCell;
    use std::rc::Rc;

    /// Doubles that do nothing and return nothing.
    fn double_nothing(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    /// Doubles of constructors: they return their first argument.
    fn double_this(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, a| Ret {
                eax: a[0],
                ..Ret::default()
            });
        }
    }

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// The argument words of every logged call to `addr`.
    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// The addresses of the logged calls, in order, without the first (the
    /// call the test makes).
    fn call_order(e: &Engine) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .skip(1)
            .map(|(a, _)| *a)
            .collect()
    }

    /// A double of the buffer's byte writer that keeps what it was asked to
    /// write.
    fn record_saved_bytes(e: &mut Engine) -> Rc<RefCell<Vec<Vec<u8>>>> {
        let saved = Rc::new(RefCell::new(vec![]));
        let keep = saved.clone();
        e.register_double(SAVE_BYTES, move |e, a| {
            keep.borrow_mut().push(e.mem.bytes(a[1], a[2]));
            Ret::default()
        });
        saved
    }

    #[test]
    fn test_fn_008d6fc0_copies_the_position_when_the_stamp_is_set() {
        let mut e = Engine::new();
        e.map(MAX_FLOAT, 4);
        e.set_global(MAX_FLOAT, f32::MAX);
        e.register(TIME_STAMP_VALUE, |_, _| Ret {
            st0: 12.5,
            ..Ret::default()
        });
        let source = e.new_object::<DetectionState>();
        let this = e.new_object::<DetectionState>();
        e.set(source, DetectionState::pActor, Ptr::<()>::new(0x1234));
        e.set(source, DetectionState::eDetectionLevel, -3);
        e.set(source, DetectionState::iLevel, 77);
        e.set(source, DetectionState::b360view, 1u8);
        e.set(source, DetectionState::bcombat, 2u8);
        e.set(source, DetectionState::bLineofSight, 3u8);
        e.set(source, DetectionState::bEvaluated, 4u8);
        e.set(source, DetectionState::iDetectedEventLevel, 9);
        e.mem.set_u32(source.addr() + 0x0c, 0xa);
        e.mem.set_u32(source.addr() + 0x10, 0xb);
        e.mem.set_u32(source.addr() + 0x14, 0xc);
        e.set(source, DetectionState::fLastTimeDetected, 0xd);
        e.call_log = Some(vec![]);
        e.call(0x008d6fc0, &args![this, source]);
        assert_eq!(
            calls_to(&e, TIME_STAMP_VALUE),
            vec![vec![source.addr() + 0x18]]
        );
        assert_eq!(e.get(this, DetectionState::pActor).addr(), 0x1234);
        assert_eq!(e.get(this, DetectionState::eDetectionLevel), -3);
        assert_eq!(e.get(this, DetectionState::iLevel), 77);
        assert_eq!(e.get(this, DetectionState::b360view), 1);
        assert_eq!(e.get(this, DetectionState::bcombat), 2);
        assert_eq!(e.get(this, DetectionState::bLineofSight), 3);
        assert_eq!(e.get(this, DetectionState::bEvaluated), 4);
        assert_eq!(e.get(this, DetectionState::iDetectedEventLevel), 9);
        assert_eq!(e.mem.u32(this.addr() + 0x0c), 0xa);
        assert_eq!(e.mem.u32(this.addr() + 0x10), 0xb);
        assert_eq!(e.mem.u32(this.addr() + 0x14), 0xc);
        assert_eq!(e.get(this, DetectionState::fLastTimeDetected), 0xd);
    }

    #[test]
    fn test_fn_008d6fc0_keeps_the_position_when_the_stamp_is_unset() {
        for stamp in [-f64::from(f32::MAX), f64::NAN] {
            let mut e = Engine::new();
            e.map(MAX_FLOAT, 4);
            e.set_global(MAX_FLOAT, f32::MAX);
            e.register_double(TIME_STAMP_VALUE, move |_, _| Ret {
                st0: stamp,
                ..Ret::default()
            });
            let source = e.new_object::<DetectionState>();
            let this = e.new_object::<DetectionState>();
            e.set(source, DetectionState::iLevel, 77);
            e.mem.set_u32(source.addr() + 0x0c, 0xa);
            e.set(source, DetectionState::fLastTimeDetected, 0xd);
            e.call(0x008d6fc0, &args![this, source]);
            assert_eq!(
                e.get(this, DetectionState::iLevel),
                77,
                "the rest is copied"
            );
            assert_eq!(e.mem.u32(this.addr() + 0x0c), 0, "the position is not");
            assert_eq!(e.get(this, DetectionState::fLastTimeDetected), 0);
        }
    }

    #[test]
    fn test_detection_state_save_game() {
        let mut e = Engine::new();
        let saved = record_saved_bytes(&mut e);
        double_nothing(&mut e, &[SAVE_FORM_ID, COMBAT_TIME_STAMP_SAVE]);
        let this = e.new_object::<DetectionState>();
        let buffer = Ptr::<()>::new(0x5000);
        e.set(this, DetectionState::pActor, Ptr::<()>::new(0x1111));
        e.set(this, DetectionState::eDetectionLevel, 0x105);
        e.set(this, DetectionState::iLevel, 0x2233_4455);
        e.mem.set_u32(this.addr() + 0x0c, 0x0102_0304);
        e.set(this, DetectionState::bLineofSight, 6u8);
        e.set(this, DetectionState::b360view, 7u8);
        e.set(this, DetectionState::bcombat, 8u8);
        e.set(this, DetectionState::iDetectedEventLevel, 0x99);
        e.set(this, DetectionState::bEvaluated, 9u8);
        e.call_log = Some(vec![]);
        e.call(0x008d7070, &args![this, buffer]);
        let a = this.addr();
        let log = e.call_log.clone().unwrap();
        assert_eq!(log[1], (SAVE_FORM_ID, vec![0x5000, 0x1111, 0]));
        assert_eq!(log[3], (SAVE_BYTES, vec![0x5000, a + 0x08, 4, 0]));
        assert_eq!(log[4], (SAVE_BYTES, vec![0x5000, a + 0x0c, 0x0c, 0]));
        assert_eq!(log[5], (COMBAT_TIME_STAMP_SAVE, vec![a + 0x18, 0x5000]));
        assert_eq!(log[6], (SAVE_BYTES, vec![0x5000, a + 0x1e, 1, 0]));
        assert_eq!(log[7], (SAVE_BYTES, vec![0x5000, a + 0x1c, 1, 0]));
        assert_eq!(log[8], (SAVE_BYTES, vec![0x5000, a + 0x1d, 1, 0]));
        assert_eq!(log[9], (SAVE_BYTES, vec![0x5000, a + 0x20, 4, 0]));
        assert_eq!(log[10], (SAVE_BYTES, vec![0x5000, a + 0x1f, 1, 0]));
        assert_eq!(log.len(), 11);
        let saved = saved.borrow();
        assert_eq!(saved[0], vec![5], "the level goes out as one byte");
        assert_eq!(saved[1], 0x2233_4455u32.to_le_bytes());
        assert_eq!(saved[2][..4], 0x0102_0304u32.to_le_bytes());
        assert_eq!(saved[3], vec![6]);
        assert_eq!(saved[4], vec![7]);
        assert_eq!(saved[5], vec![8]);
        assert_eq!(saved[6], 0x99u32.to_le_bytes());
        assert_eq!(saved[7], vec![9]);
    }

    /// A load buffer whose virtual slot 0 answers `version`, and a byte reader
    /// that fills what it reads with `0xff`.
    fn load_buffer(e: &mut Engine, version: u32) -> Ptr {
        let buffer = Ptr::new(e.mem.alloc(8));
        e.put_vtable(0x7100_0000, &[0x7200_0000]);
        e.mem.set_u32(buffer.addr(), 0x7100_0000);
        e.register_double(0x7200_0000, move |_, _| word(version));
        e.register(LOAD_BYTES, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[1] + i, 0xff);
            }
            Ret::default()
        });
        double_nothing(e, &[LOAD_FORM_ID_OV2, COMBAT_TIME_STAMP_LOAD]);
        buffer
    }

    #[test]
    fn test_detection_state_load_game_reads_the_evaluated_flag_from_version_8() {
        let mut e = Engine::new();
        let buffer = load_buffer(&mut e, 8);
        let this = e.new_object::<DetectionState>();
        e.call_log = Some(vec![]);
        e.call(0x008d7140, &args![this, buffer]);
        let a = this.addr();
        assert_eq!(calls_to(&e, LOAD_FORM_ID_OV2), vec![vec![buffer.addr(), a]]);
        assert_eq!(
            calls_to(&e, COMBAT_TIME_STAMP_LOAD),
            vec![vec![a + 0x18, buffer.addr()]]
        );
        assert_eq!(calls_to(&e, LOAD_BYTES).len(), 8);
        assert_eq!(
            e.get(this, DetectionState::eDetectionLevel),
            -1,
            "sign-extended"
        );
        assert_eq!(e.get(this, DetectionState::iLevel), -1);
        assert_eq!(e.get(this, DetectionState::bLineofSight), 0xff);
        assert_eq!(e.get(this, DetectionState::b360view), 0xff);
        assert_eq!(e.get(this, DetectionState::bcombat), 0xff);
        assert_eq!(
            e.get(this, DetectionState::iDetectedEventLevel),
            0xffff_ffff
        );
        assert_eq!(e.get(this, DetectionState::bEvaluated), 0xff);
    }

    #[test]
    fn test_detection_state_load_game_skips_the_evaluated_flag_before_version_8() {
        let mut e = Engine::new();
        let buffer = load_buffer(&mut e, 7);
        let this = e.new_object::<DetectionState>();
        e.call_log = Some(vec![]);
        e.call(0x008d7140, &args![this, buffer]);
        assert_eq!(calls_to(&e, LOAD_BYTES).len(), 7);
        assert_eq!(e.get(this, DetectionState::bEvaluated), 0);
        assert_eq!(
            e.get(this, DetectionState::iDetectedEventLevel),
            0xffff_ffff
        );
    }

    /// Doubles for the form lookup and the dynamic cast; the cast gives
    /// `cast_result`.
    fn form_cast_doubles(e: &mut Engine, cast_result: u32) {
        e.register(LOOKUP_FORM, |_, a| word(a[0] + 0x1000));
        e.register_double(RT_DYNAMIC_CAST, move |_, _| word(cast_result));
        double_nothing(e, &[SET_TARGETED]);
    }

    #[test]
    fn test_fn_008d7220_turns_a_form_id_into_an_actor() {
        let mut e = Engine::new();
        form_cast_doubles(&mut e, 0x6000);
        let slot = Ptr::<()>::new(e.mem.alloc(4));
        e.mem.set_u32(slot.addr(), 0x42);
        e.call_log = Some(vec![]);
        e.call(0x008d7220, &args![slot]);
        assert_eq!(e.mem.u32(slot.addr()), 0x6000);
        assert_eq!(calls_to(&e, LOOKUP_FORM), vec![vec![0x42]]);
        assert_eq!(
            calls_to(&e, RT_DYNAMIC_CAST),
            vec![vec![0x1042, 0, TYPE_TES_FORM, TYPE_ACTOR, 0]]
        );
    }

    #[test]
    fn test_fn_008d7220_keeps_null_and_drops_a_failed_cast() {
        let mut e = Engine::new();
        form_cast_doubles(&mut e, 0);
        let slot = Ptr::<()>::new(e.mem.alloc(4));
        e.call_log = Some(vec![]);
        e.call(0x008d7220, &args![slot]);
        assert_eq!(e.mem.u32(slot.addr()), 0);
        assert!(
            calls_to(&e, LOOKUP_FORM).is_empty(),
            "id 0 is not looked up"
        );
        e.mem.set_u32(slot.addr(), 0x42);
        e.call(0x008d7220, &args![slot]);
        assert_eq!(e.mem.u32(slot.addr()), 0, "the form is no Actor");
    }

    #[test]
    fn test_detection_event_save_game() {
        let mut e = Engine::new();
        let saved = record_saved_bytes(&mut e);
        double_nothing(&mut e, &[SAVE_FORM_ID, COMBAT_TIME_STAMP_SAVE]);
        let this = e.new_object::<DetectionEvent>();
        e.set(this, DetectionEvent::iActionValue, 0x0a0b_0c0d);
        e.set(this, DetectionEvent::Location, 0x1111);
        e.set(this, DetectionEvent::eEventType, 0x22);
        e.set(this, DetectionEvent::pRef, Ptr::<()>::new(0x3333));
        e.call_log = Some(vec![]);
        e.call(0x008d7270, &args![this, Ptr::<()>::new(0x5000)]);
        let a = this.addr();
        let log = e.call_log.clone().unwrap();
        assert_eq!(log[1], (SAVE_BYTES, vec![0x5000, a, 4, 0]));
        assert_eq!(log[2], (SAVE_BYTES, vec![0x5000, a + 4, 0x0c, 0]));
        assert_eq!(log[3], (COMBAT_TIME_STAMP_SAVE, vec![a + 0x10, 0x5000]));
        assert_eq!(log[4], (SAVE_BYTES, vec![0x5000, a + 0x14, 4, 0]));
        assert_eq!(log[5], (SAVE_FORM_ID, vec![0x5000, 0x3333, 0]));
        assert_eq!(log.len(), 6);
        let saved = saved.borrow();
        assert_eq!(saved[0], 0x0a0b_0c0du32.to_le_bytes());
        assert_eq!(saved[1][..4], 0x1111u32.to_le_bytes());
        assert_eq!(saved[2], 0x22u32.to_le_bytes());
    }

    #[test]
    fn test_fn_008d72e0() {
        let mut e = Engine::new();
        let buffer = load_buffer(&mut e, 8);
        let this = e.new_object::<DetectionEvent>();
        e.call_log = Some(vec![]);
        e.call(0x008d72e0, &args![this, buffer]);
        let a = this.addr();
        assert_eq!(
            call_order(&e),
            vec![
                LOAD_BYTES,
                LOAD_BYTES,
                COMBAT_TIME_STAMP_LOAD,
                LOAD_BYTES,
                LOAD_FORM_ID_OV2
            ]
        );
        assert_eq!(
            calls_to(&e, LOAD_BYTES),
            vec![
                vec![buffer.addr(), a, 4],
                vec![buffer.addr(), a + 4, 0x0c],
                vec![buffer.addr(), a + 0x14, 4]
            ]
        );
        assert_eq!(
            calls_to(&e, COMBAT_TIME_STAMP_LOAD),
            vec![vec![a + 0x10, buffer.addr()]]
        );
        assert_eq!(
            calls_to(&e, LOAD_FORM_ID_OV2),
            vec![vec![buffer.addr(), a + 0x18]]
        );
    }

    #[test]
    fn test_fn_008d7340_resolves_and_targets_the_reference() {
        let mut e = Engine::new();
        form_cast_doubles(&mut e, 0x6000);
        let this = e.new_object::<DetectionEvent>();
        e.set(this, DetectionEvent::pRef, Ptr::<()>::new(0x42));
        e.call_log = Some(vec![]);
        e.call(0x008d7340, &args![this]);
        assert_eq!(e.get(this, DetectionEvent::pRef).addr(), 0x6000);
        assert_eq!(
            calls_to(&e, RT_DYNAMIC_CAST),
            vec![vec![0x1042, 0, TYPE_TES_FORM, TYPE_TES_OBJECT_REFR, 0]]
        );
        assert_eq!(calls_to(&e, SET_TARGETED), vec![vec![0x6000, 1]]);
    }

    #[test]
    fn test_fn_008d7340_leaves_null_and_failed_casts_untargeted() {
        let mut e = Engine::new();
        form_cast_doubles(&mut e, 0);
        let this = e.new_object::<DetectionEvent>();
        e.call_log = Some(vec![]);
        e.call(0x008d7340, &args![this]);
        assert!(calls_to(&e, LOOKUP_FORM).is_empty());
        e.set(this, DetectionEvent::pRef, Ptr::<()>::new(0x42));
        e.call(0x008d7340, &args![this]);
        assert_eq!(e.get(this, DetectionEvent::pRef).addr(), 0);
        assert!(calls_to(&e, SET_TARGETED).is_empty());
    }

    #[test]
    fn test_fn_008d73b0() {
        let mut e = Engine::new();
        let saved = record_saved_bytes(&mut e);
        double_nothing(&mut e, &[BASE_SAVE, SAVE_FORM_ID]);
        let this = Ptr::<()>::new(e.mem.alloc(0x40));
        e.mem.set_u32(this.addr() + 0x24, 0xaa);
        e.mem.set_u32(this.addr() + 0x28, 0x1001);
        e.mem.set_u32(this.addr() + 0x2c, 0xbb);
        e.mem.set_u32(this.addr() + 0x30, 0x1002);
        e.call_log = Some(vec![]);
        e.call(0x008d73b0, &args![this, Ptr::<()>::new(0x5000)]);
        let a = this.addr();
        let log = e.call_log.clone().unwrap();
        assert_eq!(log[1], (BASE_SAVE, vec![a, 0x5000]));
        assert_eq!(log[2], (SAVE_BYTES, vec![0x5000, a + 0x24, 4, 0]));
        assert_eq!(log[3], (SAVE_BYTES, vec![0x5000, a + 0x2c, 4, 0]));
        assert_eq!(log[4], (SAVE_FORM_ID, vec![0x5000, 0x1001, 0]));
        assert_eq!(log[5], (SAVE_FORM_ID, vec![0x5000, 0x1002, 0]));
        assert_eq!(log.len(), 6);
        assert_eq!(saved.borrow()[0], 0xaau32.to_le_bytes());
        assert_eq!(saved.borrow()[1], 0xbbu32.to_le_bytes());
    }

    #[test]
    fn test_fn_008d7420() {
        let mut e = Engine::new();
        let buffer = load_buffer(&mut e, 8);
        double_nothing(&mut e, &[BASE_LOAD]);
        e.register(LOAD_FORM_ID, |_, _| word(0x77));
        form_cast_doubles(&mut e, 0x6000);
        let this = Ptr::<()>::new(e.mem.alloc(0x40));
        e.call_log = Some(vec![]);
        e.call(0x008d7420, &args![this, buffer]);
        let a = this.addr();
        assert_eq!(calls_to(&e, BASE_LOAD), vec![vec![a, buffer.addr()]]);
        assert_eq!(
            calls_to(&e, LOAD_BYTES),
            vec![
                vec![buffer.addr(), a + 0x24, 4],
                vec![buffer.addr(), a + 0x2c, 4]
            ]
        );
        assert_eq!(calls_to(&e, LOOKUP_FORM), vec![vec![0x77]]);
        assert_eq!(
            calls_to(&e, RT_DYNAMIC_CAST),
            vec![vec![0x1077, 0, TYPE_TES_FORM, TYPE_TES_OBJECT_STAT, 0]]
        );
        assert_eq!(e.mem.u32(a + 0x28), 0x6000);
        assert_eq!(
            calls_to(&e, LOAD_FORM_ID_OV2),
            vec![vec![buffer.addr(), a + 0x30]]
        );
    }

    #[test]
    fn test_fn_008d74a0() {
        let mut e = Engine::new();
        form_cast_doubles(&mut e, 0x6000);
        let this = Ptr::<()>::new(e.mem.alloc(0x40));
        e.mem.set_u32(this.addr() + 0x30, 0x42);
        e.call_log = Some(vec![]);
        e.call(0x008d74a0, &args![this]);
        assert_eq!(e.mem.u32(this.addr() + 0x30), 0x6000);
        assert_eq!(calls_to(&e, SET_TARGETED), vec![vec![0x6000, 1]]);
        e.mem.set_u32(this.addr() + 0x30, 0);
        e.call(0x008d74a0, &args![this]);
        assert_eq!(e.mem.u32(this.addr() + 0x30), 0);
        assert_eq!(calls_to(&e, SET_TARGETED).len(), 1, "none for null");
    }

    /// Everything the `HighProcess` constructor calls, as doubles, and the
    /// globals and settings it reads.
    fn constructor_world(e: &mut Engine) {
        double_nothing(
            e,
            &[
                BASE_CONSTRUCTOR,
                POINT_CONSTRUCTOR,
                VECTOR_CONSTRUCT,
                NI_POINTER_INIT,
                NI_POINTER_SET,
                QUEUE_POINTER_INIT,
                KF_MODEL_POINTER_SET,
                STORE_WORD_AT_0X28,
            ],
        );
        double_this(e, &[LIST_CONSTRUCTOR]);
        e.register(OPERATOR_NEW, |e, a| word(e.mem.alloc(a[0])));
        // A setting's `float` sits 0x10 bytes after the setting's address.
        e.map(0x011c_d000, 0x1000);
        e.register(SETTING_FLOAT, |_, a| word(a[0] + 0x10));
        for (setting, value) in [
            (IDLE_CHATTER_FIRST, 1.0f32),
            (IDLE_CHATTER_SECOND, 2.0),
            (CHECK_TO_TALK_SECOND, 3.0),
            (CHECK_TO_TALK_FIRST, 4.0),
            (CLEAR_TALK_TO_LIST, 9.5),
        ] {
            e.mem.set_f32(setting + 0x10, value);
        }
        // `GetRandomBetween(first, second)` answers `first * 1000 + second`.
        e.register(RANDOM_BETWEEN, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])) * 1000.0 + f64::from(f32::from_bits(a[1])),
            ..Ret::default()
        });
        e.map(DEFAULT_WEAPON_POSITION, 12);
        for (i, v) in [0x11u32, 0x22, 0x33].into_iter().enumerate() {
            e.mem.set_u32(DEFAULT_WEAPON_POSITION + 4 * i as u32, v);
        }
        e.map(DEFAULT_BREATH_TIMER, 4);
        e.set_global(DEFAULT_BREATH_TIMER, 0.75f32);
        e.map(DEFAULT_HEAD_TRACK_TIMER, 4);
        e.set_global(DEFAULT_HEAD_TRACK_TIMER, 0.5f32);
        e.map(DETECTION_COUNTER, 4);
        e.set_global(DETECTION_COUNTER, 25u32);
    }

    #[test]
    fn test_high_process_high_process_calls_in_the_game_order() {
        let mut e = Engine::new();
        constructor_world(&mut e);
        let this = e.new_object::<HighProcess>();
        e.call_log = Some(vec![]);
        let r = e.call(0x008d7510, &args![this]);
        assert_eq!(r.ptr::<()>().addr(), this.addr(), "returns this");
        let a = this.addr();
        assert_eq!(
            call_order(&e),
            vec![
                BASE_CONSTRUCTOR,
                LIST_CONSTRUCTOR,
                LIST_CONSTRUCTOR,
                LIST_CONSTRUCTOR,
                LIST_CONSTRUCTOR,
                POINT_CONSTRUCTOR,
                VECTOR_CONSTRUCT,
                VECTOR_CONSTRUCT,
                VECTOR_CONSTRUCT,
                NI_POINTER_INIT,
                LIST_CONSTRUCTOR,
                LIST_CONSTRUCTOR,
                NI_POINTER_INIT,
                NI_POINTER_INIT,
                NI_POINTER_INIT,
                QUEUE_POINTER_INIT,
                QUEUE_POINTER_INIT,
                OPERATOR_NEW,
                LIST_CONSTRUCTOR,
                OPERATOR_NEW,
                LIST_CONSTRUCTOR,
                OPERATOR_NEW,
                LIST_CONSTRUCTOR,
                OPERATOR_NEW,
                LIST_CONSTRUCTOR,
                OPERATOR_NEW,
                LIST_CONSTRUCTOR,
                SETTING_FLOAT,
                SETTING_FLOAT,
                RANDOM_BETWEEN,
                SETTING_FLOAT,
                SETTING_FLOAT,
                RANDOM_BETWEEN,
                NI_POINTER_SET,
                NI_POINTER_SET,
                NI_POINTER_SET,
                KF_MODEL_POINTER_SET,
                KF_MODEL_POINTER_SET,
                STORE_WORD_AT_0X28,
                NI_POINTER_SET,
                SETTING_FLOAT,
            ]
        );
        assert_eq!(
            calls_to(&e, LIST_CONSTRUCTOR)[..4],
            [
                vec![a + 0x274],
                vec![a + 0x27c],
                vec![a + 0x284],
                vec![a + 0x28c]
            ]
        );
        assert_eq!(calls_to(&e, BASE_CONSTRUCTOR), vec![vec![a]]);
        assert_eq!(
            calls_to(&e, VECTOR_CONSTRUCT),
            vec![
                vec![a + 0x314, 0x0c, 2, 0x0041_a250, 0x0048_3710],
                vec![a + 0x354, 4, 2, 0x0066_94e0, 0x0045_cec0],
                vec![a + 0x35c, 4, 2, 0x008d_98c0, 0x0044_b030],
            ]
        );
        assert_eq!(
            calls_to(&e, NI_POINTER_INIT),
            vec![
                vec![a + 0x380, 0],
                vec![a + 0x434, 0],
                vec![a + 0x45c, 0],
                vec![a + 0x460, 0]
            ]
        );
        assert_eq!(
            calls_to(&e, NI_POINTER_SET),
            vec![
                vec![a + 0x380, 0],
                vec![a + 0x354, 0],
                vec![a + 0x358, 0],
                vec![a + 0x434, 0]
            ]
        );
        assert_eq!(
            calls_to(&e, KF_MODEL_POINTER_SET),
            vec![vec![a + 0x35c, 0], vec![a + 0x360, 0]]
        );
        assert_eq!(calls_to(&e, STORE_WORD_AT_0X28), vec![vec![a, 0]]);
        // The settings are read second argument first.
        assert_eq!(
            calls_to(&e, SETTING_FLOAT),
            vec![
                vec![IDLE_CHATTER_SECOND],
                vec![IDLE_CHATTER_FIRST],
                vec![CHECK_TO_TALK_SECOND],
                vec![CHECK_TO_TALK_FIRST],
                vec![CLEAR_TALK_TO_LIST]
            ]
        );
    }

    #[test]
    fn test_high_process_high_process_sets_the_starting_values() {
        let mut e = Engine::new();
        constructor_world(&mut e);
        let this = e.new_object::<HighProcess>();
        // Fill the object so that every field the constructor sets shows.
        for i in 0..0x46c / 4 {
            e.mem.set_u32(this.addr() + 4 * i, 0xcccc_cccc);
        }
        e.call(0x008d7510, &args![this]);
        let a = this.addr();
        assert_eq!(e.mem.u32(a), HIGH_PROCESS_VTABLE);
        assert_eq!(e.get(this, HighProcess::iHourPackageEvaluated), -1);
        assert_eq!(e.get(this, HighProcess::iLastBoneLOD), 0xffff_fffe);
        assert_eq!(e.get(this, HighProcess::sAnimAction), -1);
        assert_eq!(e.get(this, HighProcess::iLastDetection), -1);
        assert_eq!(e.get(this, HighProcess::sShotsFired), 0);
        assert_eq!(e.get(this, HighProcess::sShotsToFire), -1);
        assert_eq!(e.get(this, HighProcess::sBurstsFired), 0);
        assert_eq!(e.get(this, HighProcess::iHasHealingSpell), 0xffff_ffff);
        assert_eq!(e.get(this, HighProcess::iHasHealingPotion), 0xffff_ffff);
        assert_eq!(e.get(this, HighProcess::bHeadTrack), 1);
        assert_eq!(e.get(this, HighProcess::bStop), 0);
        assert_eq!(e.get(this, HighProcess::bHiding), 0);
        assert_eq!(e.get(this, HighProcess::bIsDoingSayTo), 0);
        assert_eq!(e.get(this, HighProcess::bNeedTalkPlayer), 0);
        assert_eq!(e.get(this, HighProcess::bPlantedExplosive), 0);
        assert_eq!(e.get(this, HighProcess::fIdleTimer), 1.0);
        assert_eq!(e.get(this, HighProcess::fMaxAlpha), 1.0);
        assert_eq!(e.get(this, HighProcess::fActorHealthPercentage), 1.0);
        assert_eq!(e.get(this, HighProcess::fDelayTimer), 0.0);
        assert_eq!(e.get(this, HighProcess::fCachedActorHeight), 0.0);
        assert_eq!(e.get(this, HighProcess::eFadeState), 1);
        assert_eq!(e.get(this, HighProcess::eSpecialIdleType), 2);
        assert_eq!(e.get(this, HighProcess::ePostAnimActon), 0);
        assert_eq!(e.get(this, HighProcess::pNode).addr(), 0);
        assert_eq!(e.get(this, HighProcess::pDialogTarget).addr(), 0);
        assert_eq!(e.get(this, HighProcess::pSubtitleVoice).addr(), 0);
        assert_eq!(e.get(this, HighProcess::m_pSayToDialogueTopic).addr(), 0);
        assert_eq!(e.get(this, HighProcess::pLastTarget).addr(), 0);
        // Copied globals and settings.
        assert_eq!(e.mem.u32(a + 0x300), 0x11);
        assert_eq!(e.mem.u32(a + 0x304), 0x22);
        assert_eq!(e.mem.u32(a + 0x308), 0x33);
        assert_eq!(e.get(this, HighProcess::fBreathTimer), 0.75);
        assert_eq!(e.mem.f32(a + HEAD_TRACK_TARGET_TIMER), 0.5);
        assert_eq!(e.get(this, HighProcess::fTakeBackTimer), 0.5);
        assert_eq!(e.get(this, HighProcess::fIdleChatterTimer), 1002.0);
        assert_eq!(e.get(this, HighProcess::fCheckToTalkTimer), 4003.0);
        assert_eq!(e.get(this, HighProcess::fClearTalkToListTimer), 9.5);
        // The head-tracking arrays are cleared.
        for i in 0..6 {
            assert_eq!(e.mem.u32(a + HEAD_TRACKING_TARGETS + 4 * i), 0);
            assert_eq!(e.mem.u8(a + HEAD_TRACKING_TARGET_FLAGS + i), 0);
        }
        // Five distinct detection lists were allocated and constructed.
        let lists = [
            HighProcess::pDetectedActorList,
            HighProcess::pActorsWhoDetectMeList,
            HighProcess::pLastSpokeToList,
            HighProcess::pThreadDetectList,
            HighProcess::pTempActorsWhoDetectMeList,
        ]
        .map(|field| e.get(this, field).addr());
        for (i, list) in lists.iter().enumerate() {
            assert_ne!(*list, 0);
            assert_eq!(e.mem.block_size(*list), Some(8));
            assert!(!lists[..i].contains(list));
        }
    }

    #[test]
    fn test_high_process_high_process_takes_the_detection_counter_modulo_12() {
        let mut e = Engine::new();
        constructor_world(&mut e);
        let this = e.new_object::<HighProcess>();
        e.call(0x008d7510, &args![this]);
        assert_eq!(e.get(this, HighProcess::iDetectionCounter), 1, "25 % 12");
        assert_eq!(e.global::<u32>(DETECTION_COUNTER), 26);
        let other = e.new_object::<HighProcess>();
        e.set_global(DETECTION_COUNTER, 11u32);
        e.call(0x008d7510, &args![other]);
        assert_eq!(e.get(other, HighProcess::iDetectionCounter), 11);
        assert_eq!(e.global::<u32>(DETECTION_COUNTER), 12);
        e.call(0x008d7510, &args![other]);
        assert_eq!(e.get(other, HighProcess::iDetectionCounter), 0, "12 % 12");
    }

    #[test]
    fn test_low_process_set_item_being_used() {
        let mut e = Engine::new();
        let this = e.new_object::<LowProcess>();
        e.call(0x008d7dc0, &args![this, Ptr::<()>::new(0x1234_5678)]);
        assert_eq!(e.mem.u32(this.addr() + 0x34), 0x1234_5678);
        assert_eq!(e.mem.u32(this.addr() + 0x38), 0, "only the one field");
    }

    /// Setting through `setter` (a one-flag setter) sets and clears `mask` of
    /// the flags byte at +0x30 and nothing else.
    fn check_flag_setter(setter: u32, mask: u8) {
        let mut e = Engine::new();
        let this = e.new_object::<LowProcess>();
        e.mem.set_u8(this.addr() + 0x30, 0x40);
        e.mem.set_u8(this.addr() + 0x31, 0x55);
        e.call(setter, &args![this, 1u8]);
        assert_eq!(e.mem.u8(this.addr() + 0x30), 0x40 | mask);
        e.call(setter, &args![this, 0u8]);
        assert_eq!(e.mem.u8(this.addr() + 0x30), 0x40);
        e.mem.set_u8(this.addr() + 0x30, 0xff);
        e.call(setter, &args![this, 0u8]);
        assert_eq!(e.mem.u8(this.addr() + 0x30), !mask);
        e.call(setter, &args![this, 0x80u8]);
        assert_eq!(e.mem.u8(this.addr() + 0x30), 0xff, "any nonzero sets");
        assert_eq!(e.mem.u8(this.addr() + 0x31), 0x55);
    }

    /// `getter` answers whether `mask` is set in the flags byte.
    fn check_flag_getter(getter: u32, mask: u8) {
        let mut e = Engine::new();
        let this = e.new_object::<LowProcess>();
        e.mem.set_u8(this.addr() + 0x30, !mask);
        assert!(!e.call(getter, &args![this]).bool());
        e.mem.set_u8(this.addr() + 0x30, mask);
        assert!(e.call(getter, &args![this]).bool());
        e.mem.set_u8(this.addr() + 0x30, 0xff);
        assert!(e.call(getter, &args![this]).bool());
    }

    #[test]
    fn test_fn_008d7e00() {
        let mut e = Engine::new();
        let this = e.new_object::<LowProcess>();
        e.mem.set_u8(this.addr() + 0x30, 0b0101);
        e.call(0x008d7e00, &args![this, 1u8, 0b1010u8]);
        assert_eq!(e.mem.u8(this.addr() + 0x30), 0b1111);
        e.call(0x008d7e00, &args![this, 0u8, 0b0110u8]);
        assert_eq!(e.mem.u8(this.addr() + 0x30), 0b1001);
    }

    #[test]
    fn test_fn_008d7e60() {
        let mut e = Engine::new();
        let this = e.new_object::<LowProcess>();
        e.mem.set_u8(this.addr() + 0x30, 0b0101);
        assert!(e.call(0x008d7e60, &args![this, 0b0100u8]).bool());
        assert!(e.call(0x008d7e60, &args![this, 0b1101u8]).bool(), "any bit");
        assert!(!e.call(0x008d7e60, &args![this, 0b1010u8]).bool());
        assert_eq!(e.call(0x008d7e60, &args![this, 0b0001u8]).u32(), 1);
    }

    #[test]
    fn test_low_process_set_current_action_complete() {
        check_flag_setter(0x008d7de0, 0x02);
    }

    #[test]
    fn test_low_process_get_current_action_complete() {
        check_flag_getter(0x008d7e40, 0x02);
    }

    #[test]
    fn test_low_process_is_a_follower() {
        check_flag_getter(0x008d7e80, 0x10);
    }

    #[test]
    fn test_low_process_set_follower() {
        check_flag_setter(0x008d7ea0, 0x10);
    }

    #[test]
    fn test_low_process_get_is_aggressor() {
        check_flag_getter(0x008d7ec0, 0x04);
    }

    #[test]
    fn test_low_process_set_is_aggressor() {
        check_flag_setter(0x008d7ee0, 0x04);
    }

    #[test]
    fn test_low_process_set_essential_down_timer() {
        let mut e = Engine::new();
        let this = e.new_object::<LowProcess>();
        e.call(0x008d7f00, &args![this, 2.5f32]);
        assert_eq!(e.mem.f32(this.addr() + 0xa4), 2.5);
        assert_eq!(e.mem.u32(this.addr() + 0xa8), 0, "only the one field");
    }

    /// The game clock double: ST0 = `now`.
    fn clock_at(e: &mut Engine, now: f64) {
        e.register_double(CLOCK, move |_, _| Ret {
            st0: now,
            ..Ret::default()
        });
    }

    #[test]
    fn test_fn_008d7f40_starts_a_timer() {
        let mut e = Engine::new();
        clock_at(&mut e, 12.5);
        let timer = e.new_object::<CombatTimer>();
        e.call(0x008d7f40, &args![timer, 3.25f32]);
        assert_eq!(e.get(timer, CombatTimer::fStartTime), 12.5);
        assert_eq!(e.get(timer, CombatTimer::fDelay), 3.25);
    }

    #[test]
    fn test_low_process_set_combat_delay_timer() {
        let mut e = Engine::new();
        clock_at(&mut e, 7.0);
        let this = e.new_object::<LowProcess>();
        e.call(0x008d7f20, &args![this, 1.5f32]);
        assert_eq!(e.mem.f32(this.addr() + 0x38), 7.0);
        assert_eq!(e.mem.f32(this.addr() + 0x3c), 1.5);
    }

    /// Whether a timer started at 4.0 with `delay` has run out at time 10.0.
    fn timer_has_run_out(delay: f32) -> bool {
        let mut e = Engine::new();
        clock_at(&mut e, 10.0);
        let timer = e.new_object::<CombatTimer>();
        e.set(timer, CombatTimer::fStartTime, 4.0);
        e.set(timer, CombatTimer::fDelay, delay);
        e.call(0x008d7f80, &args![timer]).bool()
    }

    #[test]
    fn test_fn_008d7f80() {
        assert!(timer_has_run_out(5.0), "6 elapsed > 5");
        assert!(timer_has_run_out(-1.0));
        assert!(!timer_has_run_out(6.0), "equal is not over");
        assert!(!timer_has_run_out(7.0));
        assert!(!timer_has_run_out(f32::NAN));
    }

    #[test]
    fn test_low_process_check_combat_delay_timer() {
        let mut e = Engine::new();
        clock_at(&mut e, 10.0);
        let this = e.new_object::<LowProcess>();
        e.mem.set_f32(this.addr() + 0x38, 4.0);
        e.mem.set_f32(this.addr() + 0x3c, 5.0);
        assert!(e.call(0x008d7f60, &args![this]).bool());
        e.mem.set_f32(this.addr() + 0x3c, 6.5);
        assert!(!e.call(0x008d7f60, &args![this]).bool());
    }

    #[test]
    fn test_low_process_is_target_activated() {
        check_flag_getter(0x008d7fc0, 0x01);
    }

    #[test]
    fn test_low_process_set_target_activated() {
        check_flag_setter(0x008d7fe0, 0x01);
    }

    #[test]
    fn test_low_process_is_package_done_once() {
        check_flag_getter(0x008d8000, 0x20);
    }

    #[test]
    fn test_low_process_set_package_done_once() {
        check_flag_setter(0x008d8020, 0x20);
    }

    #[test]
    fn test_low_process_set_generic_location() {
        let mut e = Engine::new();
        let this = e.new_object::<LowProcess>();
        e.call(0x008d8040, &args![this, Ptr::<()>::new(0x1234_5678)]);
        assert_eq!(e.mem.u32(this.addr() + 0x44), 0x1234_5678);
        assert_eq!(e.mem.u32(this.addr() + 0x48), 0, "only the one field");
    }

    #[test]
    fn test_low_process_get_locked_location() {
        check_flag_getter(0x008d8060, 0x80);
    }

    #[test]
    fn test_low_process_set_locked_location() {
        check_flag_setter(0x008d8080, 0x80);
    }

    #[test]
    fn test_low_process_get_alert() {
        check_flag_getter(0x008d80a0, 0x08);
    }

    #[test]
    fn test_low_process_set_alert() {
        check_flag_setter(0x008d80c0, 0x08);
    }

    #[test]
    fn test_middle_high_process_get_active_effect_list() {
        let mut e = Engine::new();
        let this = e.new_object::<MiddleHighProcess>();
        e.mem.set_u32(this.addr() + 0x1b8, 0x1234_5678);
        assert_eq!(e.call(0x008d8100, &args![this]).u32(), 0x1234_5678);
    }

    #[test]
    fn test_middle_high_process_get_current_spell() {
        let mut e = Engine::new();
        let this = e.new_object::<MiddleHighProcess>();
        e.mem.set_u32(this.addr() + 0x160, 0x1234_5678);
        assert_eq!(e.call(0x008d8120, &args![this]).u32(), 0x1234_5678);
    }

    #[test]
    fn test_middle_high_process_set_current_spell() {
        let mut e = Engine::new();
        let this = e.new_object::<MiddleHighProcess>();
        e.call(0x008d8140, &args![this, Ptr::<()>::new(0x1234_5678)]);
        assert_eq!(e.mem.u32(this.addr() + 0x160), 0x1234_5678);
        assert_eq!(e.mem.u32(this.addr() + 0x164), 0, "only the one field");
    }

    #[test]
    fn test_middle_high_process_get_desired_target() {
        let mut e = Engine::new();
        let this = e.new_object::<MiddleHighProcess>();
        e.mem.set_u32(this.addr() + 0x1bc, 0x1234_5678);
        assert_eq!(e.call(0x008d8160, &args![this]).u32(), 0x1234_5678);
    }

    #[test]
    fn test_middle_high_process_set_desired_target() {
        let mut e = Engine::new();
        let this = e.new_object::<MiddleHighProcess>();
        e.call(0x008d8180, &args![this, Ptr::<()>::new(0x1234_5678)]);
        assert_eq!(e.mem.u32(this.addr() + 0x1bc), 0x1234_5678);
        assert_eq!(e.mem.u32(this.addr() + 0x1b8), 0, "only the one field");
    }

    #[test]
    fn test_middle_high_process_get_should_check_magic_node() {
        let mut e = Engine::new();
        let this = e.new_object::<MiddleHighProcess>();
        assert!(!e.call(0x008d81a0, &args![this]).bool());
        e.mem.set_u8(this.addr() + 0x168, 1);
        assert!(e.call(0x008d81a0, &args![this]).bool());
    }
    #[test]
    fn test_middle_high_process_set_should_check_magic_node() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.call(0x008d81c0, &args![this, 0x1234_5677u32]);
        assert_eq!(e.mem.u8(this.addr() + 0x168), 0x77);
        assert_eq!(
            e.mem.u8(this.addr() + 0x168 + 1),
            0,
            "only the one byte is written"
        );
    }

    #[test]
    fn test_middle_high_process_get_current_weapon() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.mem.set_u32(this.addr() + 0x114, 0x1234_5678);
        assert_eq!(e.call(0x008d81e0, &args![this]).u32(), 0x1234_5678);
    }

    #[test]
    fn test_middle_high_process_is_current_weapon_thrown() {
        let mut e = Engine::new();
        let this = process(&mut e);
        assert!(!e.call(0x008d8200, &args![this]).bool());
        e.mem.set_u8(this.addr() + 0x126, 1);
        assert!(e.call(0x008d8200, &args![this]).bool());
    }

    #[test]
    fn test_middle_high_process_is_current_weapon_grenade() {
        let mut e = Engine::new();
        let this = process(&mut e);
        assert!(!e.call(0x008d8220, &args![this]).bool());
        e.mem.set_u8(this.addr() + 0x124, 1);
        assert!(e.call(0x008d8220, &args![this]).bool());
    }

    #[test]
    fn test_middle_high_process_is_heavy_body_armor_worn() {
        let mut e = Engine::new();
        let this = process(&mut e);
        assert!(!e.call(0x008d8270, &args![this]).bool());
        e.mem.set_u8(this.addr() + 0x127, 1);
        assert!(e.call(0x008d8270, &args![this]).bool());
    }

    #[test]
    fn test_middle_high_process_is_power_body_armor_worn() {
        let mut e = Engine::new();
        let this = process(&mut e);
        assert!(!e.call(0x008d8290, &args![this]).bool());
        e.mem.set_u8(this.addr() + 0x128, 1);
        assert!(e.call(0x008d8290, &args![this]).bool());
    }

    #[test]
    fn test_middle_high_process_is_power_armor_helmet_worn() {
        let mut e = Engine::new();
        let this = process(&mut e);
        assert!(!e.call(0x008d82b0, &args![this]).bool());
        e.mem.set_u8(this.addr() + 0x129, 1);
        assert!(e.call(0x008d82b0, &args![this]).bool());
    }

    #[test]
    fn test_middle_high_process_has_back_pack_worn() {
        let mut e = Engine::new();
        let this = process(&mut e);
        assert!(!e.call(0x008d82d0, &args![this]).bool());
        e.mem.set_u8(this.addr() + 0x12a, 1);
        assert!(e.call(0x008d82d0, &args![this]).bool());
    }

    #[test]
    fn test_middle_high_process_set_fire_node() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.call(0x008d82f0, &args![this, Ptr::<()>::new(0x1234_5678)]);
        assert_eq!(e.mem.u32(this.addr() + 0x130), 0x1234_5678);
        assert_eq!(
            e.mem.u32(this.addr() + 0x130 + 4),
            0,
            "only the one field is written"
        );
    }

    #[test]
    fn test_middle_high_process_get_aim_looking() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.mem.set_f32(this.addr() + 0x1d0, 1.5);
        assert_eq!(e.call(0x008d8330, &args![this]).f32(), 1.5);
    }

    #[test]
    fn test_middle_high_process_set_shots_to_fire() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.call(0x008d8380, &args![this, 0x1234_5677u32]);
        assert_eq!(e.mem.u8(this.addr() + 0x1d8), 0x77);
        assert_eq!(
            e.mem.u8(this.addr() + 0x1d8 + 1),
            0,
            "only the one byte is written"
        );
    }

    #[test]
    fn test_middle_high_process_get_shots_to_fire() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.mem.set_u8(this.addr() + 0x1d8, 7);
        assert_eq!(e.call(0x008d83a0, &args![this]).u32() & 0xff, 7);
    }

    #[test]
    fn test_middle_high_process_get_automatic_shot_delay() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.mem.set_f32(this.addr() + 0x1d4, 1.5);
        assert_eq!(e.call(0x008d83c0, &args![this]).f32(), 1.5);
    }

    #[test]
    fn test_middle_high_process_set_acquire_object() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.call(0x008d8500, &args![this, Ptr::<()>::new(0x1234_5678)]);
        assert_eq!(e.mem.u32(this.addr() + 0x64), 0x1234_5678);
        assert_eq!(
            e.mem.u32(this.addr() + 0x64 + 4),
            0,
            "only the one field is written"
        );
    }

    #[test]
    fn test_middle_high_process_get_saved_acquire_object() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.mem.set_u32(this.addr() + 0x68, 0x1234_5678);
        assert_eq!(e.call(0x008d8520, &args![this]).u32(), 0x1234_5678);
    }

    #[test]
    fn test_middle_high_process_get_knock_state() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.mem.set_u8(this.addr() + 0x13c, 0x05);
        assert_eq!(e.call(0x008d8540, &args![this]).u32(), 5);
        e.mem.set_u8(this.addr() + 0x13c, 0xff);
        assert_eq!(e.call(0x008d8540, &args![this]).u32() as i32, -1);
    }

    #[test]
    fn test_middle_high_process_set_knock_state() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.call(0x008d8560, &args![this, 0x1234_5677u32]);
        assert_eq!(e.mem.u8(this.addr() + 0x13c), 0x77);
        assert_eq!(
            e.mem.u8(this.addr() + 0x13c + 1),
            0,
            "only the one byte is written"
        );
    }

    #[test]
    fn test_middle_high_process_get_current_package_spell() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.mem.set_u32(this.addr() + 0x164, 0x1234_5678);
        assert_eq!(e.call(0x008d8580, &args![this]).u32(), 0x1234_5678);
    }

    #[test]
    fn test_middle_high_process_set_current_package_spell() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.call(0x008d85a0, &args![this, Ptr::<()>::new(0x1234_5678)]);
        assert_eq!(e.mem.u32(this.addr() + 0x164), 0x1234_5678);
        assert_eq!(
            e.mem.u32(this.addr() + 0x164 + 4),
            0,
            "only the one field is written"
        );
    }

    #[test]
    fn test_middle_high_process_set_last_bound_weapon() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.call(0x008d85c0, &args![this, Ptr::<()>::new(0x1234_5678)]);
        assert_eq!(e.mem.u32(this.addr() + 0x15c), 0x1234_5678);
        assert_eq!(
            e.mem.u32(this.addr() + 0x15c + 4),
            0,
            "only the one field is written"
        );
    }

    #[test]
    fn test_middle_high_process_get_last_bound_weapon() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.mem.set_u32(this.addr() + 0x15c, 0x1234_5678);
        assert_eq!(e.call(0x008d85e0, &args![this]).u32(), 0x1234_5678);
    }

    #[test]
    fn test_middle_high_process_get_should_refresh_magic_shaders_effects() {
        let mut e = Engine::new();
        let this = process(&mut e);
        assert!(!e.call(0x008d8600, &args![this]).bool());
        e.mem.set_u8(this.addr() + 0x17c, 1);
        assert!(e.call(0x008d8600, &args![this]).bool());
    }

    #[test]
    fn test_middle_high_process_set_alpha_mult() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.call(0x008d8620, &args![this, 2.5f32]);
        assert_eq!(e.mem.f32(this.addr() + 0x170), 2.5);
        assert_eq!(
            e.mem.u32(this.addr() + 0x170 + 4),
            0,
            "only the one field is written"
        );
    }

    #[test]
    fn test_middle_high_process_set_script_refraction_power() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.call(0x008d8640, &args![this, 2.5f32]);
        assert_eq!(e.mem.f32(this.addr() + 0x174), 2.5);
        assert_eq!(
            e.mem.u32(this.addr() + 0x174 + 4),
            0,
            "only the one field is written"
        );
    }

    #[test]
    fn test_middle_high_process_get_idle_done_once() {
        let mut e = Engine::new();
        let this = process(&mut e);
        assert!(!e.call(0x008d8660, &args![this]).bool());
        e.mem.set_u8(this.addr() + 0xe0, 1);
        assert!(e.call(0x008d8660, &args![this]).bool());
    }

    #[test]
    fn test_middle_high_process_get_pickpocketed() {
        let mut e = Engine::new();
        let this = process(&mut e);
        assert!(!e.call(0x008d86c0, &args![this]).bool());
        e.mem.set_u8(this.addr() + 0x188, 1);
        assert!(e.call(0x008d86c0, &args![this]).bool());
    }

    #[test]
    fn test_middle_high_process_set_pickpocketed() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.call(0x008d86e0, &args![this, 0x1234_5677u32]);
        assert_eq!(e.mem.u8(this.addr() + 0x188), 0x77);
        assert_eq!(
            e.mem.u8(this.addr() + 0x188 + 1),
            0,
            "only the one byte is written"
        );
    }

    #[test]
    fn test_middle_high_process_set_force_next_update() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.call(0x008d8700, &args![this, 0x1234_5677u32]);
        assert_eq!(e.mem.u8(this.addr() + 0x18d), 0x77);
        assert_eq!(
            e.mem.u8(this.addr() + 0x18d + 1),
            0,
            "only the one byte is written"
        );
    }

    #[test]
    fn test_middle_high_process_is_current_weapon_grenade_or_mine() {
        for offset in [0x124u32, 0x125, 0x126] {
            let mut e = Engine::new();
            let this = process(&mut e);
            assert!(!e.call(0x008d8240, &args![this]).bool());
            e.mem.set_u8(this.addr() + offset, 1);
            assert!(
                e.call(0x008d8240, &args![this]).bool(),
                "flag at {offset:#x}"
            );
        }
    }

    #[test]
    fn test_middle_high_process_is_running_run_once() {
        for (result, expected) in [(0u32, false), (7, true), (0x100, true)] {
            let mut e = Engine::new();
            let this = process(&mut e);
            e.put_vtable(0x0200_0000, &[0; 0x84]);
            e.mem.set_u32(0x0200_0000 + 0x20c, 0x0200_1000);
            e.mem.set_u32(this.addr(), 0x0200_0000);
            e.register_double(0x0200_1000, move |_, _| word(result));
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x008d8310, &args![this]).bool(), expected);
            assert_eq!(e.call_log.as_ref().unwrap().len(), 2, "entry + vcall");
        }
    }
    #[test]

    fn test_middle_high_process_add_gun_drift() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.mem.map(0x0102_3128, 8);
        e.set_global(0x0102_3128, std::f64::consts::PI / 180.0);
        e.mem.set_f32(this.addr() + 0x1d0, 1.0);
        e.call(0x008d8350, &args![this, 10.0f32]);
        let expected = (1.0f64 - 10.0f64 * (std::f64::consts::PI / 180.0)) as f32;
        assert_eq!(e.mem.f32(this.addr() + 0x1d0), expected);
    }

    /// Doubles for the package getters at +0xe4 / the `NiPointer` at +0x138
    /// that record the object address they receive.
    fn record_receiver(e: &mut Engine, address: u32, result: u32) {
        e.register_double(address, move |e, a| {
            e.mem.set_u32(0x0300_0000, a[0]);
            e.mem.set_u32(0x0300_0004, a.get(1).copied().unwrap_or(0));
            word(result)
        });
    }

    fn calling_setup(e: &mut Engine) -> Ptr {
        e.mem.map(0x0300_0000, 8);
        process(e)
    }

    #[test]
    fn test_fn_008d83e0() {
        let mut e = Engine::new();
        let this = calling_setup(&mut e);
        record_receiver(&mut e, 0x0055_9450, 0x1234_5678);
        assert_eq!(e.call(0x008d83e0, &args![this]).u32(), 0x1234_5678);
        assert_eq!(e.mem.u32(0x0300_0000), this.addr() + 0xe4);
    }

    #[test]
    fn test_middle_high_process_set_run_once_procedure_index() {
        let mut e = Engine::new();
        let this = calling_setup(&mut e);
        record_receiver(&mut e, 0x0041_fd00, 0);
        e.call(0x008d8400, &args![this, 5u32]);
        assert_eq!(e.mem.u32(0x0300_0000), this.addr() + 0xe4);
        assert_eq!(e.mem.u32(0x0300_0004), 5);
    }

    #[test]
    fn test_middle_high_process_get_run_once_procedure_index() {
        let mut e = Engine::new();
        let this = calling_setup(&mut e);
        record_receiver(&mut e, 0x0084_e3a0, 9);
        assert_eq!(e.call(0x008d8420, &args![this]).u32(), 9);
        assert_eq!(e.mem.u32(0x0300_0000), this.addr() + 0xe4);
    }

    #[test]
    fn test_middle_high_process_add_to_run_once_procedure_index() {
        let mut e = Engine::new();
        let this = calling_setup(&mut e);
        record_receiver(&mut e, 0x0049_bb50, 0);
        e.call(0x008d8440, &args![this, 3u32]);
        assert_eq!(e.mem.u32(0x0300_0000), this.addr() + 0xe4);
        assert_eq!(e.mem.u32(0x0300_0004), 3);
    }

    #[test]
    fn test_middle_high_process_get_run_once_package_instance_data() {
        let mut e = Engine::new();
        let this = calling_setup(&mut e);
        record_receiver(&mut e, 0x0072_6070, 0x2222_0000);
        assert_eq!(e.call(0x008d8460, &args![this]).u32(), 0x2222_0000);
        assert_eq!(e.mem.u32(0x0300_0000), this.addr() + 0xe4);
    }

    #[test]
    fn test_middle_high_process_get_char_controller() {
        let mut e = Engine::new();
        let this = calling_setup(&mut e);
        record_receiver(&mut e, 0x0055_9450, 0x3333_0000);
        assert_eq!(e.call(0x008d8480, &args![this]).u32(), 0x3333_0000);
        assert_eq!(e.mem.u32(0x0300_0000), this.addr() + 0x138);
    }

    #[test]
    fn test_middle_high_process_set_char_controller() {
        let mut e = Engine::new();
        let this = calling_setup(&mut e);
        // The assign double stores the source pointer's value into the
        // destination `NiPointer`, as the real one does.
        e.register(0x006e_5cc0, |e, a| {
            let source = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], source);
            word(a[0])
        });
        e.register(0x0045_cec0, |e, a| {
            e.mem.set_u32(0x0300_0004, e.mem.u32(a[0]));
            word(0)
        });
        e.call(0x008d84a0, &args![this, Ptr::<()>::new(0x4444_0000)]);
        assert_eq!(e.mem.u32(this.addr() + 0x138), 0x4444_0000);
        assert_eq!(
            e.mem.u32(0x0300_0004),
            0x4444_0000,
            "the temporary is destroyed after the assignment"
        );
    }

    #[test]
    fn test_middle_high_process_clear_face_animation_data() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.mem.set_u32(this.addr() + 0x178, 0x1234_5678);
        e.mem.set_u32(this.addr() + 0x17c, 0x1111);
        e.call(0x008d8680, &args![this]);
        assert_eq!(e.mem.u32(this.addr() + 0x178), 0);
        assert_eq!(e.mem.u32(this.addr() + 0x17c), 0x1111, "only the one field");
    }

    #[test]
    fn test_middle_high_process_set_refresh_flare_flags() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.call(0x008d86a0, &args![this]);
        assert_eq!(e.mem.u8(this.addr() + 0x180), 1);
        assert_eq!(e.mem.u8(this.addr() + 0x181), 0, "only the one byte");
    }
}
