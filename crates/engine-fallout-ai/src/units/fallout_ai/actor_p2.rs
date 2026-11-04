//! `fallout/ai/actor.cpp` (Xbox PDB source unit), part 2: its functions from `00884990` up to
//! (not including) `00891d70` in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The unit's shared layouts and helpers are in
//! [`super::actor`]; anything public there may be used here.
//!
//! This session covers `00884990` to `00885d70`: the actor's swimming state,
//! its cached values (radius, width, length, walk and run speed, the
//! `CachedValuesOwner` virtuals that `Actor` overrides through its secondary
//! vtable at `Actor + 0xa8`), its height, and the follow-package speed rules.
//!
//! The second block (b0148p2) covers `00885d90` to `0088b150`: two more speed
//! formulas, the process-level change, three large per-frame updates
//! (`00886360`, `00886cb0` and `Actor::Update`, `00888b50`), the ragdoll and
//! character-proxy accessors, `ResetLoadedAnimations`,
//! `UpdateAnimationMovement` and `UpdateActor3DPosition`. Their exception
//! frames (SEH) are not translated.
//!
//! The third block (b0148p2, second session) covers `0088b4e0` to `00891d30`,
//! the end of this part's range: the regeneration and restore functions,
//! `LineOfSight` and its cone test, `EquipObject`/`UnEquipObject` with their
//! queued variants, the quick chair and bed placement, the hit effect
//! functions (`0088e1e0`, `0088e8d0`, `0088fb00`), `DoTrap`,
//! `UpdateWeaponConditionEffects`, `DamageEquipment` and the small inventory
//! queries. The range is done: every function from `00884990` up to
//! `00891d70` is translated.
//!
//! x87 note: the game computes in extended precision and stores `float`
//! results. The translations compute in `f64` and round to `f32` where the
//! code stores a `float`.
//!
//! Virtual slots: the PC `Actor` vtable (`01084254`) has its slots 4 bytes
//! higher than the Xbox PDB's from the bounds getters on, so a PC slot is
//! described by its offset and, where the vtable contents confirm it, by the
//! Xbox PDB name of the slot four bytes lower.

#[allow(unused_imports)]
use super::actor::*;
#[allow(unused_imports)]
use crate::prelude::*;

/// Returns the actor's flag word (`00884990` copies bit `0x800` into
/// `bSwimming`; `004997b0`, `00884750` and the follow code test other bits).
const GET_FLAG_WORD: u32 = 0x0088_46e0;
/// Tells whether the process has a cached-values block (`pCachedValues != 0`).
const HAS_CACHED_VALUES: u32 = 0x0088_4920;
/// ORs a mask into `CachedValues::iFlags`.
const CACHED_VALUES_ADD_FLAGS: u32 = 0x0088_48c0;
/// Process-level function that forwards a `CachedValues::iFlags` mask to
/// `00884970` when the process has a cached-values block.
const PROCESS_FORWARD_FLAG_MASK: u32 = 0x0088_4940;
/// `Actor::GetCurrentPackageTarget` (Xbox PDB): the reference the actor's
/// current package targets, or null.
const GET_CURRENT_PACKAGE_TARGET: u32 = 0x0088_1650;
/// `MobileObject::GetCurrentPackage` (Xbox PDB).
const GET_CURRENT_PACKAGE: u32 = 0x0093_44a0;
/// `MobileObject::GetCharController` (Xbox PDB).
const GET_CHAR_CONTROLLER: u32 = 0x0093_06d0;
/// `TESObjectREFR::GetScale` (Xbox PDB), a `float` in ST0.
const GET_SCALE: u32 = 0x0056_7400;
/// `TESObjectREFR::GetDistanceFromReference` (Xbox PDB): reference, two
/// more words, `float` in ST0.
const GET_DISTANCE_FROM_REFERENCE: u32 = 0x0057_23b0;
/// `TESObjectREFR::GetWorldSpace` (Xbox PDB).
const GET_WORLD_SPACE: u32 = 0x0057_5d70;
/// `TESObjectREFR::GetRelevantWaterHeight` (Xbox PDB), `float` in ST0.
const GET_RELEVANT_WATER_HEIGHT: u32 = 0x0057_b0a0;
/// `TESObjectCELL::GetWaterHeight` (Xbox PDB), `float` in ST0.
const CELL_GET_WATER_HEIGHT: u32 = 0x0054_71e0;
/// The reference's parent cell (`ECX` = reference).
const GET_PARENT_CELL: u32 = 0x008d_6f30;
/// Cell test (`ECX` = cell, `bool` in `AL`) the follow code applies to the
/// two actors' cells to decide whether they are in an interior.
const CELL_TEST_INTERIOR: u32 = 0x0042_5fd0;
/// `Actor::GetEyeLevel` (Xbox PDB), `float` in ST0.
const GET_EYE_LEVEL: u32 = 0x008b_e940;
/// `Actor::GetArmorBeingWorn` (Xbox PDB): actor and a slot number.
const GET_ARMOR_BEING_WORN: u32 = 0x0089_1b90;
/// `Actor::GetCurrentSpeed` (Xbox PDB), `float` in ST0.
const GET_CURRENT_SPEED: u32 = 0x008a_0b10;
/// `cdecl`, two `float` arguments, `float` in ST0: returns one of them (the
/// follow code uses it to keep a distance from going below zero and the
/// cached extents above a minimum, so it is a maximum).
const FLOAT_HELPER_MAX: u32 = 0x0040_4010;
/// `cdecl`, two `float` arguments, `float` in ST0: the other of the pair
/// (the follow code uses it as a minimum).
const FLOAT_HELPER_MIN: u32 = 0x0040_ebd0;
/// `cdecl` (`float*` value, `float` lower, `float` upper): forces `*value`
/// into `[lower, upper]` (the upper bound is applied first).
const CLAMP_FLOAT: u32 = 0x0053_30e0;
/// Returns the address of the `float` held by the settings object in `ECX`
/// (a static zero when `ECX` is null).
const SETTING_VALUE_ADDRESS: u32 = 0x0040_3e20;
/// `cdecl`, one `float`: the larger of the value minus a setting and the
/// value times another setting.
const FOLLOW_RADIUS_NEAR: u32 = 0x0064_3f20;
/// `cdecl`, one `float`: returns its argument unchanged.
const FOLLOW_RADIUS_MID: u32 = 0x0064_3f90;
/// `AiFormulas::GetFollowRadiusWalk` (Xbox PDB), `cdecl`, one `float`.
const FOLLOW_RADIUS_WALK: u32 = 0x0064_3f70;
/// `AiFormulas::GetFollowRadiusMatchSpeed` (Xbox PDB), `cdecl`, one `float`.
const FOLLOW_RADIUS_MATCH_SPEED: u32 = 0x0064_3fd0;
/// Actor-level test (`bool` in `AL`) that the follow code requires before
/// it forces a run.
const FOLLOW_MODE_ALLOWED: u32 = 0x0088_4750;
/// `BGSEntryPoint::HandleEntryPoint` (Xbox PDB), `cdecl`: entry point
/// number, actor, `float*` value.
const HANDLE_ENTRY_POINT: u32 = 0x005e_58f0;
/// The walk speed formula: `cdecl`, seven words (`ActorValueOwner*`, two
/// item values, four flags), `float` in ST0.
const CALCULATE_WALK_SPEED_FORMULA: u32 = 0x0064_7d10;
/// The run speed formula: `cdecl`, seven words, `float` in ST0.
const CALCULATE_RUN_SPEED_FORMULA: u32 = 0x0064_7f00;
/// Flag test of the actor used by the speed formulas (`ECX` = actor,
/// `bool` in `AL`).
const SPEED_FLAG_TEST: u32 = 0x0049_97b0;
/// `ECX` = item entry, returns its word at +0x08.
const ITEM_VALUE: u32 = 0x0044_ddc0;
/// `ECX` = item, one stack word; called on the worn armor after its value
/// is read.
const ITEM_RELEASE: u32 = 0x0044_59e0;
/// `ECX` = the object `Actor` virtual `+0x1e4` returned, one stack word
/// (`1`); returns an object or null.
const ANIMATION_LOOKUP: u32 = 0x0049_1040;
/// Named `Animation::ZeroGlobalTransform` in the engine map; takes the
/// object `00491040` returned in `ECX` and returns an object.
const ANIMATION_STEP: u32 = 0x0048_f7f0;
/// Tests the object `0048f7f0` returned (`ECX`), `bool` in `AL`.
const ANIMATION_TEST: u32 = 0x005f_4d60;
/// Returns a kind number for the object `0048f7f0` returned (`ECX`).
const ANIMATION_KIND: u32 = 0x005f_2420;
/// Returns a state number for the character controller (`ECX`).
const CONTROLLER_STATE: u32 = 0x005c_0880;
/// `ImpactMixer::PlayJump` (Xbox PDB), `cdecl`: actor and a word.
const PLAY_JUMP: u32 = 0x0083_8780;
/// `MobileObject` call (actor in `ECX`) whose non-zero answer makes
/// `00884aa0` play the jump sound.
const JUMP_TEST: u32 = 0x0093_0640;
/// `Actor::StopMoving` (Xbox PDB).
const STOP_MOVING: u32 = 0x008b_3ab0;
/// `ECX` = the object process virtual `+0x22c` returned; a type number.
const OBJECT_TYPE: u32 = 0x0041_ca90;
/// Actor-level test (`bool` in `AL`) the swim check ends with.
const SWIM_TEST_FINAL: u32 = 0x0087_f350;
/// Actor-level test (`bool` in `AL`) the swim check applies before asking
/// whether the controller is fleeing.
const SWIM_TEST_COMBAT: u32 = 0x0087_f3b0;
/// `CombatController::IsFleeing` (Xbox PDB).
const COMBAT_CONTROLLER_IS_FLEEING: u32 = 0x0098_1990;
/// Test of the combat controller (`ECX`) the swim check applies.
const COMBAT_CONTROLLER_TEST: u32 = 0x0047_c850;
/// Test of the object process virtual `+0x22c` returned (`ECX`), used by
/// the swim check for creatures.
const SWIM_OBJECT_TEST: u32 = 0x0067_a560;
/// `BGSSaveFormBuffer::GetForm` (Xbox PDB): the form behind a reference
/// (`ECX`).
const SAVE_FORM_BUFFER_GET_FORM: u32 = 0x007a_f430;
/// Returns the form type byte (`TESForm::cFormType`, +0x04) of the form in
/// `ECX`.
const FORM_TYPE: u32 = 0x0040_1170;
/// Pointer global holding the player character (`005732d3` passes it as
/// the `this` of `008859e0`).
const PLAYER_CHARACTER: u32 = 0x011d_ea3c;
/// Process-level test (`ECX` = process): non-zero means the actor height is
/// not cached in the process.
const CACHED_HEIGHT_BYPASS: u32 = 0x0045_cd60;

/// `0.0` as a `double` (the unset value of the cached actor height).
const DOUBLE_ZERO: u32 = 0x0101_2060;
/// `1.0` as a `double`.
const DOUBLE_ONE: u32 = 0x0101_2070;
/// `0.5` as a `double`.
const DOUBLE_HALF: u32 = 0x0101_1588;
/// `0.57` (the `float`, widened) as a `double`: the sneak-height factor.
const SNEAK_HEIGHT_FACTOR: u32 = 0x0108_4830;
/// `1.25` as a `double`.
const FOLLOW_CATCH_UP_FACTOR: u32 = 0x0102_1798;
/// `32.0` as a `float`: the least extent before halving into a radius.
const MINIMUM_EXTENT: u32 = 0x0101_e340;

/// Settings objects whose `float` the follow code reads through
/// [`SETTING_VALUE_ADDRESS`]: the speed limit factors by the actor's flag
/// state and the interpolation range.
const SETTING_UPPER_FACTOR_ALL_FLAGS: u32 = 0x011c_d744;
const SETTING_UPPER_FACTOR_OTHERWISE: u32 = 0x011c_d9b4;
const SETTING_LOWER_FACTOR_ALL_FLAGS: u32 = 0x011c_d5f0;
const SETTING_LOWER_FACTOR_OTHERWISE: u32 = 0x011c_df9c;
const SETTING_INTERPOLATION_FACTOR: u32 = 0x011c_d354;

/// `CachedValues::iFlags` masks that `Actor`'s cached-value setters use.
const CACHED_RADIUS_FLAG: u32 = 0x1;
const CACHED_WIDTH_FLAG: u32 = 0x2;
const CACHED_LENGTH_FLAG: u32 = 0x4;
const CACHED_WALK_SPEED_FLAG: u32 = 0x1000;
const CACHED_RUN_SPEED_FLAG: u32 = 0x2000;
const CACHED_FORWARD_LENGTH_FLAG: u32 = 0x8000;

/// `Actor` virtual `+0x458`: `CalculateWalkSpeed` (Xbox PDB name of the slot
/// at `+0x454`; the vtable at `01084254` holds `00885a50` there).
const VSLOT_CALCULATE_WALK_SPEED: u32 = 0x458;
/// `Actor` virtual `+0x45c`: `CalculateRunSpeed` (Xbox PDB, `+0x458`);
/// holds `00885bf0`.
const VSLOT_CALCULATE_RUN_SPEED: u32 = 0x45c;
/// `Actor` virtual `+0x100`: `IsActor` (Xbox PDB).
const VSLOT_IS_ACTOR: u32 = 0x100;
/// `Actor` virtual `+0x1dc`: writes a 12-byte vector into its argument and
/// returns it (the upper corner of the bounds, since the height is its z
/// minus `+0x1d8`'s).
const VSLOT_BOUNDS_UPPER: u32 = 0x1dc;
/// `Actor` virtual `+0x1d8`: the lower corner of the bounds.
const VSLOT_BOUNDS_LOWER: u32 = 0x1d8;
/// `Actor` virtual `+0x1e4`: returns an animation object (the Xbox PDB's
/// `GetAnimation` is at `+0x1e0`).
const VSLOT_GET_ANIMATION: u32 = 0x1e4;
/// `Actor` virtual `+0x218` (Xbox PDB `IsCreature`, which sits below the
/// shift).
const VSLOT_IS_CREATURE: u32 = 0x218;
/// `Actor` virtual `+0x358` (Xbox PDB `+0x354`; the name is not confirmed).
const VSLOT_FLAG_0X358: u32 = 0x358;
/// `Actor` virtual `+0x428`: `GetCombatController` (Xbox PDB `+0x424`);
/// `IsAllowedToSwim` passes its result to `CombatController::IsFleeing`.
const VSLOT_GET_COMBAT_CONTROLLER: u32 = 0x428;
/// Process virtual `+0x22c`: returns the object `00884ae0` types.
const PROCESS_VSLOT_OBJECT: u32 = 0x22c;
/// Process virtual `+0x454`: a flag the speed formulas negate.
const PROCESS_VSLOT_FLAG: u32 = 0x454;
/// Process virtual `+0x148`: returns the item the speed formulas weigh.
const PROCESS_VSLOT_ITEM: u32 = 0x148;
/// `CachedValuesOwner` virtual `+0x38` (`CalculateCachedWalkSpeed`) and
/// `+0x3c` (`CalculateCachedRunSpeed`), Xbox PDB.
const OWNER_VSLOT_WALK_SPEED: u32 = 0x38;
const OWNER_VSLOT_RUN_SPEED: u32 = 0x3c;

/// How far the `CachedValuesOwner` subobject sits inside an `Actor` on PC.
const CACHED_VALUES_OWNER_OFFSET: u32 = 0xa8;
/// How far the `ActorValueOwner` subobject sits inside an `Actor` on PC.
const ACTOR_VALUE_OWNER_OFFSET: u32 = 0xa4;

/// `Actor::GetArmorBeingWorn`'s slot for the speed formulas.
const SPEED_ARMOR_SLOT: u32 = 2;

/// Bits of the flag word `GET_FLAG_WORD` returns, as the follow code tests
/// them (meanings not confirmed), and of the movement mode word.
const FLAGS_MATCHING_PACE_MASK: u32 = 0x20f;
const TARGET_FLAGS_HOLD_MASK: u32 = 0xf;
const TARGET_FLAGS_MODE_MASK: u32 = 0x500;
const TARGET_FLAGS_MODE_VALUE: u32 = 0x400;
const TARGET_FLAGS_BIT_200: u32 = 0x200;
const MODE_BIT_200: u32 = 0x200;
const MODE_BIT_100: u32 = 0x100;
const MODE_BIT_800: u32 = 0x800;
/// `TESForm::iFormFlags` bit tested by `00885a30`.
const FORM_FLAG_20000000: u32 = 0x2000_0000;

layout! {
    /// `BaseProcess` (Xbox PDB), size 0x30.
    pub struct BaseProcess: 0x30 {
        /// `pCachedValues` (Xbox PDB).
        0x2c pCachedValues: Ptr<CachedValues>,
    }

    /// `HighProcess` (Xbox PDB), size 0x46c: only the cached height.
    pub struct HighProcess: 0x46c {
        /// `fCachedActorHeight` (Xbox PDB).
        0x42c fCachedActorHeight: f32,
    }

    /// `CachedValues` (Xbox PDB), 0x48 bytes.
    pub struct CachedValues: 0x48 {
        /// `fCachedRadius` (Xbox PDB).
        0x00 fCachedRadius: f32,
        /// `fCachedWidth` (Xbox PDB).
        0x04 fCachedWidth: f32,
        /// `fCachedLength` (Xbox PDB).
        0x08 fCachedLength: f32,
        /// `fCachedForwardLength` (Xbox PDB).
        0x0c fCachedForwardLength: f32,
        /// `fCachedWalkSpeed` (Xbox PDB).
        0x38 fCachedWalkSpeed: f32,
        /// `fCachedRunSpeed` (Xbox PDB).
        0x3c fCachedRunSpeed: f32,
        /// `iFlags` (Xbox PDB): which cached values are valid.
        0x44 iFlags: u32,
    }

    /// `CachedValuesOwner` (Xbox PDB), the 4-byte secondary base at
    /// `Actor + 0xa8`.
    pub struct CachedValuesOwner: 0x4 {}
}

/// The `Actor` an `Actor + 0xa8` subobject pointer belongs to.
fn actor_of_owner(owner: Ptr<CachedValuesOwner>) -> Ptr<Actor> {
    Ptr::new(owner.addr().wrapping_sub(CACHED_VALUES_OWNER_OFFSET))
}

/// The `float` a settings object holds, read through
/// [`SETTING_VALUE_ADDRESS`].
fn setting_value(e: &mut Engine, setting: u32) -> f32 {
    let address = e.call(SETTING_VALUE_ADDRESS, &args![setting]).u32();
    e.mem.f32(address)
}

// ---------------------------------------------------------------------------
// Callees, globals and slots of the second block (b0148p2). Meanings are
// given only where the body of the callee or its use confirms them.

/// Reads `[ECX + 0x68]`: the actor's process (`Actor::pCurrentProcess`).
const GET_PROCESS: u32 = 0x008d_8520;
/// An empty function (`ret`); the game calls it on scope objects (a global
/// or a stack temporary in `ECX`).
const EMPTY_FUNCTION: u32 = 0x0048_3710;
const SCOPE_OBJECT_UPDATE: u32 = 0x011d_f69d;
const SCOPE_OBJECT_0X11DF71D: u32 = 0x011d_f71d;
const SCOPE_OBJECT_0X11DF69F: u32 = 0x011d_f69f;
const SCOPE_OBJECT_0X11DF69C: u32 = 0x011d_f69c;
/// `ExtraDataList` of the reference (same function as `extra_data_list`).
const GET_EXTRA_DATA_LIST_ADDRESS: u32 = 0x005d_43c0;
/// Returns the `float` frame step (`ST0`) of the settings object.
const UPDATE_STEP_SETTING: u32 = 0x0084_d030;
const UPDATE_STEP_OBJECT: u32 = 0x011f_6394;
/// The `Calendar` singleton object.
const CALENDAR: u32 = 0x011d_e7b8;
const CALENDAR_GET_DAY: u32 = 0x0086_7d60;
const CALENDAR_GET_MONTH: u32 = 0x0086_7ef0;
const CALENDAR_GET_HOUR: u32 = 0x0086_7da0;
const CALENDAR_GET_TIME_SCALE: u32 = 0x0086_7950;
const CALENDAR_GET_TIME: u32 = 0x0086_7e30;
/// The process-level number of the process in `ECX`.
const PROCESS_GET_LEVEL: u32 = 0x0045_cd60;
/// Returns the address of the `int` an integer setting object holds.
const GLOBAL_WORD_ADDRESS: u32 = 0x0043_d4d0;
const SETTING_INT_VALUE: u32 = 0x0044_ddc0;
const SETTING_INT_FN: u32 = 0x0040_3df0;
const SETTING_VALUE_POINTER: u32 = 0x0040_8d60;
const MODE_SETTING: u32 = 0x011c_3ea4;
const VATS_STATE_OBJECT: u32 = 0x011f_2250;
const VATS_TARGET_ACTOR: u32 = 0x011f_21cc;
const VATS_TARGET_UPDATE_MULT: u32 = 0x009c_8d60;
const PROCESS_LISTS: u32 = 0x011e_0e80;
const TES_POINTER: u32 = 0x011d_ea10;
const PLAYER_UPDATE_NESTING: u32 = 0x011e_07a8;
const SCALE_VECTOR: u32 = 0x011f_426c;
const HEAD_TARGET_A: u32 = 0x011e_07d4;
const HEAD_TARGET_B: u32 = 0x011e_07d0;
const FOLLOWER_BARK_COUNTDOWN: u32 = 0x011e_07e8;
const FOLLOWER_BARK_RESET: u32 = 0x0101_3dc0;
const SKIP_SET_POSITION: u32 = 0x011e_0550;
const PROCESSING_ENABLED: u32 = 0x011f_1220;
const INTERRUPT_UPDATES_ENABLED: u32 = 0x011f_122c;
const MANAGER_011DE45C: u32 = 0x011d_e45c;
const COMBAT_OBJECT_POINTER: u32 = 0x011d_e45c;
const SAVE_LOAD_GAME: u32 = 0x011d_df38;
const KF_GROUP_TABLE: u32 = 0x0118_a838;
const GLOBAL_VALUE_011C6260: u32 = 0x011c_6260;
const GLOBAL_FLAG_012677A3: u32 = 0x0126_77a3;
const GLOBAL_FLAG_011F3E60: u32 = 0x011f_3e60;
const MOVER_SETTING: u32 = 0x011d_f6b0;
const ESSENTIAL_SETTING: u32 = 0x011e_0888;
const MONTH_SETTING_A: u32 = 0x011d_058c;
const MONTH_SETTING_B: u32 = 0x011d_0c3c;
const ANIMATION_SPEED_SETTING: u32 = 0x011d_0fa8;
const EYE_DIRECTION: u32 = 0x011a_9484;
const ANIMATION_GROUP_KEY: u32 = 0x0119_9760;
const SPEED_SETTING_21C_SET_TRUE: u32 = 0x011f_2b10;
const SPEED_SETTING_21C_SET_FALSE: u32 = 0x011f_2c10;
const SPEED_SETTING_21C_CLEAR_TRUE: u32 = 0x011f_2ac4;
const SPEED_SETTING_21C_CLEAR_FALSE: u32 = 0x011f_2ad8;
const WATER_DAMAGE_SETTING_A: u32 = 0x011d_0460;
const WATER_DAMAGE_SETTING_B: u32 = 0x011d_1464;
const SPLASH_SETTING_A: u32 = 0x011d_1458;
const SPLASH_SETTING_B: u32 = 0x011d_12c0;
const SPLASH_SOUND_NAME: u32 = 0x0108_48a0;
const DROWNING_SOUND_NAME: u32 = 0x0108_48b8;
const BAD_INIT_ANIMATION_FORMAT: u32 = 0x0108_4840;
/// Constants (`float` or `double`) read through memory.
const UPDATE_TIME_LIMIT: u32 = 0x0108_48d0;
const SLEEP_UPDATE_TIME: u32 = 0x0108_4838;
const WATER_THRESHOLD_A: u32 = 0x0101_6408;
const WATER_THRESHOLD_B: u32 = 0x0104_ee98;
const WATER_THRESHOLD_C: u32 = 0x0107_3828;
const WATER_HEIGHT_UNSET: u32 = 0x0102_41b0;
const NO_WATER_HEIGHT: u32 = 0x0101_5f5c;
const WAKE_TIME_FACTOR: u32 = 0x0108_4898;
const HOURS_PER_DAY: u32 = 0x0103_5838;
const MILLISECONDS_PER_SECOND: u32 = 0x0101_7b70;
const DROWN_TIME_FACTOR: u32 = 0x0101_6ff0;
const PROCESS_TIMER_UNSET: u32 = 0x0102_31b0;
const DEAD_BODY_ALARM_RESET: u32 = 0x0101_7718;
const VOLUME_CAP: u32 = 0x0101_7718;
const CONTROLLER_VALUE_PLAYER: u32 = 0x011a_32d8;
const CONTROLLER_VALUE_OTHER: u32 = 0x0104_f4b0;
const CONTROLLER_VALUE_CREATURE: u32 = 0x0101_e2bc;
const CONTROLLER_LIMIT_B: u32 = 0x0101_a6b0;
const CONTROLLER_LIMIT_C: u32 = 0x0108_48b0;
const CONTROLLER_DIVISOR: u32 = 0x0101_e2c0;
const CONTROLLER_FACTOR_A: u32 = 0x0101_ffa0;
const CONTROLLER_OFFSET_A: u32 = 0x0102_0998;
const CONTROLLER_PUSH_DOWN: u32 = 0x0101_712c;
const CONTROLLER_PUSH_UP: u32 = 0x0102_caf8;
const MENU_ID_0X3F0: u32 = 0x3f0;
const ANIMATION_SIZE: u32 = 0x13c;
/// Speed formulas (`cdecl`).
const SPEED_FORMULA_00647F50: u32 = 0x0064_7f50;
const SPEED_FORMULA_00647F90: u32 = 0x0064_7f90;
const SPEED_FORMULA_00647FD0: u32 = 0x0064_7fd0;
const ACTOR_SPEED_VALUE_008A0C60: u32 = 0x008a_0c60;
/// Virtual slots of the actor.
const VSLOT_DESTROY: u32 = 0x10;
const VSLOT_UPDATE_HOOK_0X140: u32 = 0x140;
const VSLOT_GET_3D: u32 = 0x1d0;
const VSLOT_GET_POSITION_0X1F4: u32 = 0x1f4;
const VSLOT_FOLLOW_0X20C: u32 = 0x20c;
const VSLOT_KIND_0X214: u32 = 0x214;
const VSLOT_ANIMATION_FLAG_0X21C: u32 = 0x21c;
const VSLOT_IS_PROCESS_FLAG_0X22C: u32 = 0x22c;
const VSLOT_PACKAGE_TEST_0X230: u32 = 0x230;
const VSLOT_EVALUATE_PACKAGE_TEST_0X26C: u32 = 0x26c;
const VSLOT_0X2A0: u32 = 0x2a0;
const VSLOT_GET_HEADING_0X2BC: u32 = 0x2bc;
const VSLOT_FLAG_0X2E8: u32 = 0x2e8;
const VSLOT_DAMAGE_0X338: u32 = 0x338;
const VSLOT_ACTOR_FLAG_0X360: u32 = 0x360;
const VSLOT_ACTOR_FORM_TEST_0X38C: u32 = 0x38c;
const VSLOT_0X3AC: u32 = 0x3ac;
const VSLOT_UPDATE_0X44C: u32 = 0x44c;
const VSLOT_UPDATE_0X494: u32 = 0x494;
const VSLOT_UPDATE_0X4C8: u32 = 0x4c8;
const VSLOT_UPDATE_0X4CC: u32 = 0x4cc;
const VSLOT_0XD4: u32 = 0xd4;
/// Virtual slots of the process.
const PROCESS_VSLOT_STORED_DAY: u32 = 0x30;
const PROCESS_VSLOT_0XE0: u32 = 0xe0;
const PROCESS_VSLOT_0XE4: u32 = 0xe4;
const PROCESS_VSLOT_0XFC: u32 = 0xfc;
const PROCESS_VSLOT_0X128: u32 = 0x128;
const PROCESS_VSLOT_0X27C: u32 = 0x27c;
const PROCESS_VSLOT_0X294: u32 = 0x294;
const PROCESS_VSLOT_0X2F0: u32 = 0x2f0;
const PROCESS_VSLOT_0X2F4: u32 = 0x2f4;
const PROCESS_VSLOT_0X2F8: u32 = 0x2f8;
const PROCESS_VSLOT_0X2FC: u32 = 0x2fc;
const PROCESS_VSLOT_0X300: u32 = 0x300;
const PROCESS_VSLOT_0X35C: u32 = 0x35c;
const PROCESS_VSLOT_0X40C: u32 = 0x40c;
const PROCESS_VSLOT_0X41C: u32 = 0x41c;
const PROCESS_VSLOT_0X424: u32 = 0x424;
const PROCESS_VSLOT_0X428: u32 = 0x428;
const PROCESS_VSLOT_0X464: u32 = 0x464;
const PROCESS_VSLOT_0X48C: u32 = 0x48c;
const PROCESS_VSLOT_0X4B0: u32 = 0x4b0;
const PROCESS_VSLOT_0X4B4: u32 = 0x4b4;
const PROCESS_VSLOT_0X4BC: u32 = 0x4bc;
const PROCESS_VSLOT_0X5F8: u32 = 0x5f8;
const PROCESS_VSLOT_0X610: u32 = 0x610;
const PROCESS_VSLOT_0X618: u32 = 0x618;
const PROCESS_VSLOT_WAIT_TIME: u32 = 0x69c;
const PROCESS_VSLOT_MUZZLE_FLASH: u32 = 0x6b8;
const PROCESS_VSLOT_0X6FC: u32 = 0x6fc;
const PROCESS_VSLOT_0X704: u32 = 0x704;
const PROCESS_VSLOT_0X740: u32 = 0x740;
const PROCESS_VSLOT_0X744: u32 = 0x744;
const PROCESS_VSLOT_0X748: u32 = 0x748;
const PROCESS_VSLOT_0X74C: u32 = 0x74c;
const PROCESS_VSLOT_0X750: u32 = 0x750;
const PROCESS_VSLOT_0X760: u32 = 0x760;
const FACE_VSLOT_0XC8: u32 = 0xc8;
const FACE_VSLOT_0XD4: u32 = 0xd4;
const FACE_VSLOT_0XD8: u32 = 0xd8;
const MOVER_VSLOT_MODE: u32 = 0x1c;
const MAGIC_VSLOT_0X10: u32 = 0x10;
/// Functions.
const REFERENCE_DISABLE: u32 = 0x0057_4400;
const GET_DESIRED_PROCESS_LEVEL: u32 = 0x0093_34b0;
const PACKAGE_TYPE: u32 = 0x0096_11e0;
const PACKAGE_LOCATION_OF: u32 = 0x0055_b980;
const PACKAGE_RUNNING_OF: u32 = 0x0071_7e50;
const PACKAGE_IS_INTERRUPT: u32 = 0x0067_8610;
const PACKAGE_TEST_0X67A4F0: u32 = 0x0067_a4f0;
const GET_REF_PERSISTS: u32 = 0x0056_53d0;
const ACTOR_SKIP_UPDATE_TEST: u32 = 0x0044_0da0;
const ACTOR_ROTATE_TO_TRACK_TEST: u32 = 0x008a_40e0;
const ACTOR_RESET_FLOATS: u32 = 0x008b_bdb0;
const ACTOR_GET_LIFE_STATE: u32 = 0x004f_8960;
const ACTOR_GET_IN_COMBAT: u32 = 0x0049_3bb0;
const ACTOR_GET_CURRENT_WEAPON: u32 = 0x008a_1710;
const ACTOR_BLOCKED_TEST: u32 = 0x0043_7bd0;
const ACTOR_RANGE_TEST: u32 = 0x0089_4900;
const ACTOR_GET_BLOCKED: u32 = 0x0089_4d60;
const ACTOR_SET_BLOCK: u32 = 0x0089_4cc0;
const ACTOR_MOVER_FLAG_TEST: u32 = 0x005c_e8f0;
const ACTOR_GET_ESSENTIAL: u32 = 0x0087_f3d0;
const ACTOR_FLAG_TEST_0087F4A0: u32 = 0x0087_f4a0;
const ACTOR_TEST_0X87F5C0: u32 = 0x0087_f5c0;
const ACTOR_TEST_0X87F620: u32 = 0x0087_f620;
const ACTOR_TEST_0X87F570: u32 = 0x0087_f570;
const ACTOR_TEST_0X87F390: u32 = 0x0087_f390;
const ACTOR_TEST_0X8A6840: u32 = 0x008a_6840;
const ACTOR_TEST_0X5A2030: u32 = 0x005a_2030;
const ACTOR_IS_SURFACING: u32 = 0x008a_7a40;
const ACTOR_INITIATE_SURFACE_PACKAGE: u32 = 0x0089_84c0;
const ACTOR_SET_LIFE_STATE: u32 = 0x008a_1800;
const ACTOR_RESTORE_FULL_HEALTH: u32 = 0x008a_0960;
const ACTOR_TRIGGER_PAIN: u32 = 0x008a_7d50;
const ACTOR_TIMED_UPDATE: u32 = 0x008b_3230;
const ACTOR_END_OF_UPDATE: u32 = 0x008c_1470;
const ACTOR_UPDATE_MAGIC: u32 = 0x008c_3c40;
const ACTOR_EVALUATE_PACKAGE: u32 = 0x008a_6ce0;
const ACTOR_EVALUATE_FLAG: u32 = 0x0088_6b10;
const ACTOR_SET_MOVE_MODE: u32 = 0x008b_39f0;
const ACTOR_CLEAR_MOVE_MODE: u32 = 0x008b_3a80;
const ACTOR_GET_ENDURANCE: u32 = 0x008b_e7a0;
const ACTOR_GET_RADIATION_RESISTANCE_MULT: u32 = 0x008c_4330;
const ACTOR_MOVEMENT_STEP: u32 = 0x008b_cdf0;
const ACTOR_MOVER_TEST_0X9DCB90: u32 = 0x009d_cb90;
const ACTOR_MOVER_RESPONSE: u32 = 0x0089_85d0;
const ACTOR_PRE_ANIMATION_STEP: u32 = 0x0056_7050;
const ACTOR_AFTER_DEATH_ANIMATION: u32 = 0x0089_f580;
const ACTOR_LIGHTING_FORCE_TEST: u32 = 0x0049_38e0;
const ACTOR_QUEUE_ANIMATION_0X8B28C0: u32 = 0x008b_28c0;
const ACTOR_RAGDOLL_FACE_STEP: u32 = 0x0088_8970;
const ACTOR_DELETE_TEST: u32 = 0x008c_51f0;
const ACTOR_UNLOAD_PREPARE: u32 = 0x008c_2b60;
const ACTOR_DISPEL_ALL: u32 = 0x0082_4970;
const ACTOR_CAST_PERMANENT_MAGIC: u32 = 0x008c_26e0;
const ACTOR_MOVER_TEST: u32 = 0x009d_ccf0;
const MANAGER_REMOVE_ACTOR: u32 = 0x008d_0370;
const SAVE_LOAD_UNLOAD_FORM: u32 = 0x0084_9730;
const ACTOR_ROTATION_VECTOR: u32 = 0x0043_0830;
const POSITION_VECTOR_OF: u32 = 0x0043_6aa0;
const GET_FACE_ANIMATION_DATA: u32 = 0x008a_dcb0;
const GET_FORCE_NEXT_UPDATE: u32 = 0x0056_6950;
const TRIGGER_FOLLOWER_BARK: u32 = 0x008d_5cb0;
const PLAYER_STATE_TEST: u32 = 0x004e_af60;
const PLAYER_SLEEPING_OR_RESTING: u32 = 0x0094_df60;
const PLAYER_IN_COMBAT: u32 = 0x0095_3c50;
const PLAYER_GET_ANIMATION: u32 = 0x0095_0a60;
const PLAYER_TEST_0X524D10: u32 = 0x0052_4d10;
const PLAYER_TEST_0X9549A0: u32 = 0x0095_49a0;
const PLAYER_UPDATE_0X952290: u32 = 0x0095_2290;
const PLAYER_RADIATION_0X968730: u32 = 0x0096_8730;
const ADD_ACTOR_TO_TEMP_CHANGE_LIST: u32 = 0x0096_e870;
const TIME_NOW_LOW: u32 = 0x0052_6100;
const TES_IS_CELL_LOADED: u32 = 0x0045_11e0;
const CELL_TEST_0X450FF0: u32 = 0x0045_0ff0;
const CELL_CONTAINS_POSITION: u32 = 0x0055_0200;
const CELL_GET_WORLD_SPACE: u32 = 0x0054_ddd0;
const CELL_GET_WATER_TYPE: u32 = 0x0054_7770;
const MOVE_REF_TO_NEW_SPACE: u32 = 0x0057_3800;
const WATER_FORM_GET_DANGEROUS: u32 = 0x0058_0060;
const WATER_TYPE_COUNT: u32 = 0x0077_8950;
const TES_WATER_HEIGHT_AT: u32 = 0x0045_cbc0;
const SWIM_BREATH_TIME: u32 = 0x0064_8a10;
const DAMAGE_RATE: u32 = 0x0064_8a50;
const FTOL2: u32 = 0x00ec_62c0;
const EXTRA_GET_MERCHANT_CONTAINER: u32 = 0x0042_1400;
const MERCHANT_CONTAINER_TEST: u32 = 0x0056_ae60;
const EXTRA_LIST_DAY_CHANGED: u32 = 0x0042_e040;
const EXTRA_LIST_SET_DAY: u32 = 0x0041_d7b0;
const EXTRA_GET_DISMEMBERMENT: u32 = 0x0042_e8c0;
const EXTRA_GET_SOUND: u32 = 0x0041_8890;
const EXTRA_SET_SOUND: u32 = 0x0041_a800;
const EXTRA_GET_WEAPON_ATTACK_SOUND: u32 = 0x0041_8a00;
const EXTRA_SET_WEAPON_ATTACK_SOUND: u32 = 0x0041_a540;
const EXTRA_GET_CREATURE_AWAKE_SOUND: u32 = 0x0041_8940;
const EXTRA_SET_CREATURE_AWAKE_SOUND: u32 = 0x0041_a090;
const SOUND_HANDLE_INIT: u32 = 0x0041_a250;
const SOUND_HANDLE_ASSIGN: u32 = 0x0041_8900;
const SOUND_HANDLE_IS_PLAYING: u32 = 0x00ad_8930;
const SOUND_HANDLE_IS_VALID: u32 = 0x00ad_8ce0;
const SOUND_HANDLE_PLAY: u32 = 0x00ad_8830;
const SOUND_HANDLE_STOP: u32 = 0x00ad_88f0;
const SOUND_HANDLE_RELEASE: u32 = 0x00ad_8d10;
const SOUND_HANDLE_SET_VOLUME: u32 = 0x00ad_89e0;
const SOUND_HANDLE_SET_POSITION: u32 = 0x00ad_8b60;
const SOUND_HANDLE_SET_POSITION_PTR: u32 = 0x0068_a7d0;
const SOUND_HANDLE_SET_OBJECT_TO_FOLLOW: u32 = 0x00ad_8f20;
const SOUND_HANDLE_SET_0X4F15A0: u32 = 0x004f_15a0;
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
const AUDIO_GET_SOUND_HANDLE_BY_FILENAME: u32 = 0x00ad_7480;
const AUDIO_GET_SOUND_HANDLE_BY_NAME: u32 = 0x00ad_7550;
const SOUND_FLAGS_OF: u32 = 0x005e_39b0;
const PICK_CREATURE_SOUND: u32 = 0x005f_92d0;
const PLAY_SOUND_BY_EDITOR_NAME: u32 = 0x0093_3270;
const PLAY_SPLASH_EFFECTS: u32 = 0x0062_eb50;
const CREATE_SPLASH_EFFECT: u32 = 0x0068_9210;
const RANDOM_ANGLE: u32 = 0x004a_4240;
const OBJECT_SIZE_0X87CE50: u32 = 0x0087_ce50;
const CLAMP_MIN_4DD150: u32 = 0x004d_d150;
const WEAPON_OBJECT_OF: u32 = 0x008b_ffa0;
const WEAPON_OBJECT_TEST: u32 = 0x0051_1840;
const WEAPON_SOUND_NAME: u32 = 0x0051_1840;
const WEAPON_SOUND_ITEM: u32 = 0x004e_75d0;
const WEAPON_TYPE_0X51F5F0: u32 = 0x0051_f5f0;
const WEAPON_TEST_0X524B40: u32 = 0x0052_4b40;
const GET_LAST_BOUND_WEAPON: u32 = 0x008d_85e0;
const MUZZLE_FLASH_UPDATE: u32 = 0x009b_b080;
const HIGH_PROCESS_FADE_UPDATE: u32 = 0x008f_ec10;
const IS_IN_MENU_MODE: u32 = 0x0070_2360;
const IS_MENU_ID_VISIBLE: u32 = 0x0070_2680;
const REFERENCE_FLAG_TEST_0X200000: u32 = 0x0057_6d30;
const REFERENCE_TEST_0X579670: u32 = 0x0057_9670;
const REFERENCE_FLAG_0X50D4A0: u32 = 0x0050_d4a0;
const SET_ANIMATION: u32 = 0x0057_2e50;
const BUILD_KF_FILE_LIST: u32 = 0x0056_6970;
const INIT_ANIMATION: u32 = 0x0048_ffd0;
const HAS_KF_FILES: u32 = 0x0047_fd90;
const GET_MODEL: u32 = 0x0057_15d0;
const ADD_SPECIAL_ANIMATIONS: u32 = 0x0049_0330;
const ANIMATION_CONSTRUCTOR: u32 = 0x0048_f810;
const OPERATOR_NEW_FN: u32 = 0x0040_1000;
const LOG_MESSAGE: u32 = 0x005b_5e40;
const STRING_COPY: u32 = 0x0040_6d30;
const STRING_FIND_LAST: u32 = 0x0040_ab30;
const ITEM_GROUP_INDEX: u32 = 0x0044_6390;
const ACTOR_ANIMATION_GROUP_TEST: u32 = 0x008a_6970;
const RESET_ANIMS_SHOULD_QUEUE: u32 = 0x008c_7aa0;
const TASK_QUEUE_INTERFACE: u32 = 0x0045_37b0;
const QUEUE_ACTOR_RESET_LOADED_ANIMS: u32 = 0x0087_b850;
const ANIMATION_GROUP_LOADED: u32 = 0x0049_4710;
const ANIMATION_GROUP_FIELD: u32 = 0x0043_01b0;
const ANIM_GROUP_GET_TYPE: u32 = 0x005f_2440;
const IS_POWER_ATTACK_ACTION: u32 = 0x005f_2670;
const IS_ATTACK_ACTION: u32 = 0x005f_2540;
const ANIMATION_FORCE_SECTION: u32 = 0x0049_55c0;
const ANIMATION_UPDATE: u32 = 0x0049_1180;
const ANIMATION_UPDATE_MOVEMENT: u32 = 0x0049_3900;
const ANIMATION_UPDATE_MOVEMENT_NO_WORLD_UPDATE: u32 = 0x0049_39d0;
const ANIMATION_UPDATE_SCENE_GRAPH_NO_CONTROLLER: u32 = 0x0049_3bd0;
const ANIMATION_SET_SPEED_SCALE: u32 = 0x0088_b030;
const ANIMATION_BLEND_OUT: u32 = 0x0049_94f0;
const ANIMATION_ENTRY_KIND: u32 = 0x0080_41a0;
const ANIMATION_ENTRY_TEST_0X4937E0: u32 = 0x0049_37e0;
const ANIMATION_FIELD_0X70F490: u32 = 0x0070_f490;
const ANIMATION_SPECIAL_IDLE_FREE: u32 = 0x0049_8910;
const ANIMATION_CLEAR_GROUP: u32 = 0x0049_6080;
const ANIMATION_FUNCTION_0X4964D0: u32 = 0x0049_64d0;
const ANIMATION_TIME: u32 = 0x0045_3700;
const ANIMATION_SCALE_VECTOR: u32 = 0x0049_4390;
const ANIMATION_SET_FRACTION: u32 = 0x0049_bdc0;
const LIST_FRACTION_AT: u32 = 0x004d_50e0;
const LIST_COUNT: u32 = 0x0055_9450;
const WORD_AT_START: u32 = 0x0055_9450;
const WORD_AT_0XC: u32 = 0x0084_e3a0;
const ENTRY_FLOAT_08100: u32 = 0x0050_8100;
const ENTRY_SET_VALUE_98ADB0: u32 = 0x0098_adb0;
const PICK_ANIMATIONS: u32 = 0x0089_5110;
const PICK_COUNT_OVERRIDE_BYTE: u32 = 0x0088_8960;
const SET_PICK_COUNT_OVERRIDE: u32 = 0x00c6_6230;
const CONSOLIDATE_SIM_ISLANDS: u32 = 0x0062_c430;
const DO_RAGDOLL_ANIM: u32 = 0x00c7_d630;
const RAGDOLL_TEST_0X4955A0: u32 = 0x0049_55a0;
const PROXY_OF: u32 = 0x004a_e750;
const SET_FLOAT_ON_TARGET: u32 = 0x0093_14d0;
const SET_MOTOR_ACTIVE: u32 = 0x00cb_8fe0;
const BYTE_STORE: u32 = 0x0062_2570;
const SUBOBJECT_SET_VALUE: u32 = 0x004a_3f10;
const FLAG_SET_OR_CLEAR: u32 = 0x0062_9670;
const BLEND_OBJECT_OF: u32 = 0x0043_fcd0;
const SET_BIP_TRANSFORM: u32 = 0x00c8_21b0;
const MATRIX_CONSTRUCT: u32 = 0x0068_15c0;
const MATRIX_MAKE_ROTATION: u32 = 0x004a_0c90;
const MATRIX_MAKE_X_ROTATION: u32 = 0x0052_4ac0;
const MATRIX_MAKE_Y_ROTATION: u32 = 0x0043_f850;
const MATRIX_MULTIPLY: u32 = 0x0043_f8d0;
const MULTIPLY_MATRIX_BY_RACE: u32 = 0x0056_fac0;
const SET_WORLD_POSITION: u32 = 0x0044_0460;
const SET_WORLD_ROTATION: u32 = 0x0043_fa80;
const VECTOR_TIMES_SCALAR: u32 = 0x004a_3760;
const VECTOR_TIMES_MATRIX: u32 = 0x004b_3ae0;
const VECTOR_PLUS_VECTOR: u32 = 0x0043_9e90;
const NI_POINT3_SET: u32 = 0x0041_6870;
const OBJECT_POSITION_0X8C: u32 = 0x0045_bb80;
const CONTROLLER_SET_0X520: u32 = 0x0088_b0a0;
const CONTROLLER_FLAG_0X88B0C0: u32 = 0x0088_b0c0;
const CONTROLLER_SPEED_0X88B0F0: u32 = 0x0088_b0f0;
const CONTROLLER_PITCH: u32 = 0x0056_7730;
const CONTROLLER_ROLL: u32 = 0x0056_7750;
const CONTROLLER_SET_TARGET: u32 = 0x00c6_d6b0;
const SET_VELOCITY_MODIFIER: u32 = 0x00c6_d4d0;
const CHARACTER_PROXY_POSITION: u32 = 0x0062_1a40;
const CONTROLLER_FUNCTION_0X4A3E90: u32 = 0x004a_3e90;
const INDEXED_ELEMENT_ADDRESS: u32 = 0x0056_0d30;
const FIND_NEXT_COLLISION_OBJECT: u32 = 0x00c8_02d0;
const COLLISION_RIGID_BODY: u32 = 0x006f_a820;
const RIGID_BODY_IS_ACTIVE: u32 = 0x0056_09b0;
const SHADOW_SCENE_NODE: u32 = 0x0045_0b80;
const UPDATE_OBJECT_LIGHTING: u32 = 0x00b5_d9f0;
const SHADOW_SCENE_NODE_UPDATE_0XB5F080: u32 = 0x00b5_f080;
const CELL_UPDATE_REFERENCE: u32 = 0x0054_a070;
const GET_LINKED_OBJECT: u32 = 0x0057_25f0;
const GET_HEADING: u32 = 0x008b_d7b0;
const SET_LOCATION_ON_REFERENCE_Z: u32 = 0x0057_5b70;
const GET_BIP_BLEND_VALUE: u32 = 0x00c8_2580;
const PROCESS_FLOAT_0X7DF1F0: u32 = 0x007d_f1f0;
const CLEAR_INTERPOLATORS: u32 = 0x0057_1560;
const SCOPE_OBJECT_0X11DF69E: u32 = 0x011d_f69e;
const GET_CURRENT_PROCESS_TYPE: u32 = 0x0093_1850;
const MOVER_VSLOT_0X20: u32 = 0x20;
const MOVER_VSLOT_0X24: u32 = 0x24;
const VSLOT_0X48: u32 = 0x48;
const VSLOT_0X4B4: u32 = 0x4b4;
const VSLOT_0X250: u32 = 0x250;
const ACTOR_GET_ANIMATION: u32 = 0x008b_70d0;
const SET_LOCATION_ON_REFERENCE: u32 = 0x0057_5830;
const MATRIX_TIMES_VECTOR: u32 = 0x004b_4500;
const ACTOR_FUNCTION_0X8B00C0: u32 = 0x008b_00c0;
const PROCESS_VSLOT_0X28C: u32 = 0x28c;
const GET_WORLD_BOUND: u32 = 0x0043_d450;
const VECTOR_MINUS_VECTOR: u32 = 0x0043_9ef0;
const VECTOR_LENGTH: u32 = 0x0045_7990;
const SKELETON_VSLOT_0X10: u32 = 0x10;
const LOD_DIVISOR: u32 = 0x007d_1d00;
const SKELETON_RADIUS: u32 = 0x006d_2c20;
const GET_ACCUMULATOR: u32 = 0x00b4_f5c0;
const ACCUMULATOR_TEST: u32 = 0x004b_6e70;
const ACCUMULATOR_UPDATE: u32 = 0x00b6_5f60;
const GLOBAL_BYTE_FN_525420: u32 = 0x0052_5420;
const TURRET_SET_FLAG_0X810610: u32 = 0x0081_0610;
const ACTOR_TURRET_BEHAVIOR: u32 = 0x0087_eeb0;
const GLOBAL_WORD_FN_87E9A0: u32 = 0x0087_e9a0;
const RAGDOLL_TEST_0X552490: u32 = 0x0055_2490;
const DISABLE_RAGDOLL_ANIM: u32 = 0x00c7_c150;
const ACTOR_SHAPE_0X931ED0: u32 = 0x0093_1ed0;
const SHAPE_RESULT_0X4A3A20: u32 = 0x004a_3a20;
const PENETRATION_UPDATE: u32 = 0x00ca_2ad0;
const DO_KNOCK_DOWN: u32 = 0x00c9_b670;
const FIND_COLLISION_OBJECT: u32 = 0x004a_de00;
const COLLISION_FUNCTION_0X6838B0: u32 = 0x0068_38b0;
const FIND_ENTRY: u32 = 0x0065_3270;
const ENTRY_TABLE_A: u32 = 0x0126_7e64;
const ENTRY_TABLE_B: u32 = 0x0126_83ec;
const BODY_LIST_0X7D6BB0: u32 = 0x007d_6bb0;
const BODY_SHAPE_0X517630: u32 = 0x0051_7630;
const WORLD_TEST_0XC8CCE0: u32 = 0x00c8_cce0;
const SET_MOTION: u32 = 0x00c6_a350;
const BODY_VSLOT_0X94: u32 = 0x94;
const BODY_FUNCTION_0X621480: u32 = 0x0062_1480;
const ADD_ACT_CON_TO_WORLD: u32 = 0x00c8_dac0;
const BODY_FLAGS_0X43B4F0: u32 = 0x0043_b4f0;
const BODY_KIND_0X43B4D0: u32 = 0x0043_b4d0;
const COLLISION_SET_0XC804D0: u32 = 0x00c8_04d0;
const BODY_ACTIVATE_0X561580: u32 = 0x0056_1580;
const NEXT_LIST_NODE: u32 = 0x0072_6070;
const ANGLE_0X5B9E80: u32 = 0x005b_9e80;
const ANGLE_0X5C53D0: u32 = 0x005c_53d0;
const QUATERNION_SET: u32 = 0x0055_32a0;
const NODE_VSLOT_0X44: u32 = 0x44;
const HANDLE_FLOAT_0X9A1260: u32 = 0x009a_1260;
const HANDLE_FLOAT_0X644A50: u32 = 0x0064_4a50;
const QUEUE_COLLISION_SYNC: u32 = 0x0087_ab50;
const COLLISION_SYNCHRONIZE: u32 = 0x00c6_c3d0;

// Translated from 00884990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies bit `0x800` of the actor's flag word (`008846e0`) into the
/// `bSwimming` byte.
pub fn fn_00884990(e: &mut Engine, this: Ptr<Actor>) {
    let flags = e.call(GET_FLAG_WORD, &args![this]).u32();
    e.set(this, Actor::bSwimming, flags & 0x800 != 0);
}

// Translated from 008849c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when the actor's animation says so (the object behind virtual
/// `+0x1e4` leads to an object that passes `005f4d60`, or whose kind from
/// `005f2420` is `0xe3`, `0xe5` or `0xf1` to `0xf4`), or else when the
/// character controller's state (`005c0880`) is 2.
pub fn fn_008849c0(e: &mut Engine, this: Ptr<Actor>) -> bool {
    let animation = e.vcall(this.addr(), VSLOT_GET_ANIMATION, &args![]).u32();
    if animation != 0 && e.call(ANIMATION_LOOKUP, &args![animation, 1u32]).u32() != 0 {
        let entry = e.call(ANIMATION_LOOKUP, &args![animation, 1u32]).u32();
        let step = e.call(ANIMATION_STEP, &args![entry]).u32();
        if e.call(ANIMATION_TEST, &args![step]).bool() {
            return true;
        }
        let entry = e.call(ANIMATION_LOOKUP, &args![animation, 1u32]).u32();
        let step = e.call(ANIMATION_STEP, &args![entry]).u32();
        let kind = e.call(ANIMATION_KIND, &args![step]).u32();
        let index = kind.wrapping_sub(0xe3);
        // The compiler's switch (`00884a2b`) on `kind - 0xe3` (`0..=0x11`, an unsigned compare):
        // kinds `0xe3`, `0xe5` and `0xf1` to `0xf4` answer true, the others carry on.
        if matches!(index, 0 | 2 | 14..=17) {
            return true;
        }
    }
    let controller = e.call(GET_CHAR_CONTROLLER, &args![this]).u32();
    controller != 0 && e.call(CONTROLLER_STATE, &args![controller]).i32() == 2
}

// Translated from 00884aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `00930640` on the actor and, when it answers non-zero, plays the
/// jump sound (`ImpactMixer::PlayJump(actor, 0)`); returns the answer.
pub fn fn_00884aa0(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    let result = e.call(JUMP_TEST, &args![this]).u32();
    if result != 0 {
        e.call(PLAY_JUMP, &args![this, 0u32]);
    }
    result
}

// Translated from 00884ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when the process's virtual `+0x22c` object has type `0x12` or
/// `0x15` (read by `0041ca90`); false without a process or object.
pub fn fn_00884ae0(e: &mut Engine, this: Ptr<Actor>) -> bool {
    let process = e.get(this, Actor::pCurrentProcess);
    let object = if process.is_null() {
        0
    } else {
        e.vcall(process.addr(), PROCESS_VSLOT_OBJECT, &args![])
            .u32()
    };
    if object == 0 {
        return false;
    }
    e.call(OBJECT_TYPE, &args![object]).i32() == 0x12
        || e.call(OBJECT_TYPE, &args![object]).i32() == 0x15
}

// Translated from 00884b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::IsAllowedToSwim` (Xbox PDB). True when the flag byte at `+0x14c`
/// is set. Otherwise, when `00884ae0` holds: a combat controller (virtual
/// `+0x428`) that is neither accompanied (`0087f3b0`) nor fleeing, or that
/// fails `0047c850`, forbids it; else the answer is `0087f350`'s. When
/// `00884ae0` does not hold: creatures (virtual `+0x218`) may swim if the
/// process's `+0x22c` object passes `0067a560`; everything else gets
/// `0087f350`'s answer.
pub fn actor_is_allowed_to_swim(e: &mut Engine, this: Ptr<Actor>) -> bool {
    if e.get(this, Actor::bInWater) {
        return true;
    }
    if fn_00884ae0(e, this) {
        let controller = e
            .vcall(this.addr(), VSLOT_GET_COMBAT_CONTROLLER, &args![])
            .u32();
        if controller != 0 {
            if !e.call(SWIM_TEST_COMBAT, &args![this]).bool()
                && !e
                    .call(COMBAT_CONTROLLER_IS_FLEEING, &args![controller])
                    .bool()
            {
                return false;
            }
            if !e.call(COMBAT_CONTROLLER_TEST, &args![controller]).bool() {
                return false;
            }
        }
        return e.call(SWIM_TEST_FINAL, &args![this]).bool();
    }
    if e.vcall(this.addr(), VSLOT_IS_CREATURE, &args![]).bool() {
        let process = e.get(this, Actor::pCurrentProcess);
        let object = if process.is_null() {
            0
        } else {
            e.vcall(process.addr(), PROCESS_VSLOT_OBJECT, &args![])
                .u32()
        };
        object != 0 && e.call(SWIM_OBJECT_TEST, &args![object]).bool()
    } else {
        e.call(SWIM_TEST_FINAL, &args![this]).bool()
    }
}

// Translated from 00884c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards to `Actor::StopMoving` (Xbox PDB).
pub fn fn_00884c60(e: &mut Engine, this: Ptr<Actor>) {
    e.call(STOP_MOVING, &args![this]);
}

// Translated from 00884c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CachedValuesOwner::CalculateCachedWalkSpeed` (Xbox PDB name of slot
/// `+0x38`, which `01084144` fills with this function) as `Actor`
/// implements it. `this` is the `CachedValuesOwner` subobject: asks the
/// actor for its walk speed (virtual `+0x458`) and, when its process has a
/// cached-values block, stores the value there.
pub fn fn_00884c80(e: &mut Engine, this: Ptr<CachedValuesOwner>) -> f32 {
    let actor = actor_of_owner(this);
    let speed = e
        .vcall(actor.addr(), VSLOT_CALCULATE_WALK_SPEED, &args![])
        .f32();
    let process = e.get(actor, Actor::pCurrentProcess);
    if !process.is_null() && e.call(HAS_CACHED_VALUES, &args![process]).bool() {
        fn_00884ce0(e, process.cast(), speed);
    }
    speed
}

// Translated from 00884ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the walk speed in the process's cached values and marks it valid
/// (`iFlags |= 0x1000`).
pub fn fn_00884ce0(e: &mut Engine, this: Ptr<BaseProcess>, speed: f32) {
    let cached = e.get(this, BaseProcess::pCachedValues);
    if !cached.is_null() {
        e.set(cached, CachedValues::fCachedWalkSpeed, speed);
        e.call(
            CACHED_VALUES_ADD_FLAGS,
            &args![cached, CACHED_WALK_SPEED_FLAG],
        );
    }
}

// Translated from 00884d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CachedValuesOwner::CalculateCachedRunSpeed` (Xbox PDB name of slot
/// `+0x3c`) as `Actor` implements it: like `00884c80` with the run speed
/// (virtual `+0x45c`).
pub fn fn_00884d20(e: &mut Engine, this: Ptr<CachedValuesOwner>) -> f32 {
    let actor = actor_of_owner(this);
    let speed = e
        .vcall(actor.addr(), VSLOT_CALCULATE_RUN_SPEED, &args![])
        .f32();
    let process = e.get(actor, Actor::pCurrentProcess);
    if !process.is_null() && e.call(HAS_CACHED_VALUES, &args![process]).bool() {
        fn_00884d80(e, process.cast(), speed);
    }
    speed
}

// Translated from 00884d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the run speed in the process's cached values and marks it valid
/// (`iFlags |= 0x2000`).
pub fn fn_00884d80(e: &mut Engine, this: Ptr<BaseProcess>, speed: f32) {
    let cached = e.get(this, BaseProcess::pCachedValues);
    if !cached.is_null() {
        e.set(cached, CachedValues::fCachedRunSpeed, speed);
        e.call(
            CACHED_VALUES_ADD_FLAGS,
            &args![cached, CACHED_RUN_SPEED_FLAG],
        );
    }
}

// Translated from 00884dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetWalkSpeed` (Xbox PDB): the cached walk speed when the
/// process has a cached-values block (`00884e30`, with the actor's
/// `CachedValuesOwner` subobject), else the actor's virtual `+0x458`.
pub fn actor_get_walk_speed(e: &mut Engine, this: Ptr<Actor>) -> f32 {
    let process = e.get(this, Actor::pCurrentProcess);
    if !process.is_null() && e.call(HAS_CACHED_VALUES, &args![process]).bool() {
        let owner = if this.is_null() {
            Ptr::NULL
        } else {
            Ptr::new(this.addr() + CACHED_VALUES_OWNER_OFFSET)
        };
        return fn_00884e30(e, process.cast(), owner);
    }
    e.vcall(this.addr(), VSLOT_CALCULATE_WALK_SPEED, &args![])
        .f32()
}

// Translated from 00884e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The process's walk speed: 0 without a cached-values block; the cached
/// value while its `0x1000` flag is set; otherwise the owner's virtual
/// `+0x38` (`CalculateCachedWalkSpeed`).
pub fn fn_00884e30(e: &mut Engine, this: Ptr<BaseProcess>, owner: Ptr<CachedValuesOwner>) -> f32 {
    let cached = e.get(this, BaseProcess::pCachedValues);
    if cached.is_null() {
        return 0.0;
    }
    if !fn_00884e90(e, cached, CACHED_WALK_SPEED_FLAG) {
        e.get(cached, CachedValues::fCachedWalkSpeed)
    } else {
        e.vcall(owner.addr(), OWNER_VSLOT_WALK_SPEED, &args![])
            .f32()
    }
}

// Translated from 00884e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when none of the `mask` bits are set in `iFlags`.
pub fn fn_00884e90(e: &mut Engine, this: Ptr<CachedValues>, mask: u32) -> bool {
    e.get(this, CachedValues::iFlags) & mask == 0
}

// Translated from 00884eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetRunSpeed` (Xbox PDB): like `GetWalkSpeed` with the run speed
/// (`00884f20`; fallback virtual `+0x45c`).
pub fn actor_get_run_speed(e: &mut Engine, this: Ptr<Actor>) -> f32 {
    let process = e.get(this, Actor::pCurrentProcess);
    if !process.is_null() && e.call(HAS_CACHED_VALUES, &args![process]).bool() {
        let owner = if this.is_null() {
            Ptr::NULL
        } else {
            Ptr::new(this.addr() + CACHED_VALUES_OWNER_OFFSET)
        };
        return fn_00884f20(e, process.cast(), owner);
    }
    e.vcall(this.addr(), VSLOT_CALCULATE_RUN_SPEED, &args![])
        .f32()
}

// Translated from 00884f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The process's run speed: 0 without a cached-values block; the cached
/// value while its `0x2000` flag is set; otherwise the owner's virtual
/// `+0x3c` (`CalculateCachedRunSpeed`).
pub fn fn_00884f20(e: &mut Engine, this: Ptr<BaseProcess>, owner: Ptr<CachedValuesOwner>) -> f32 {
    let cached = e.get(this, BaseProcess::pCachedValues);
    if cached.is_null() {
        return 0.0;
    }
    if !fn_00884e90(e, cached, CACHED_RUN_SPEED_FLAG) {
        e.get(cached, CachedValues::fCachedRunSpeed)
    } else {
        e.vcall(owner.addr(), OWNER_VSLOT_RUN_SPEED, &args![]).f32()
    }
}

// Translated from 00884f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the actor has a process, passes the run-speed mask `0x2000` and
/// then the walk-speed mask `0x1000` to the process-level `00884940`.
pub fn fn_00884f80(e: &mut Engine, this: Ptr<Actor>) {
    let process = e.get(this, Actor::pCurrentProcess);
    if !process.is_null() {
        e.call(
            PROCESS_FORWARD_FLAG_MASK,
            &args![process, CACHED_RUN_SPEED_FLAG],
        );
        e.call(
            PROCESS_FORWARD_FLAG_MASK,
            &args![process, CACHED_WALK_SPEED_FLAG],
        );
    }
}

// Translated from 00884fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::SetWalkSpeed` (Xbox PDB): stores the speed in the process's
/// cached values (`00884ce0`) when the actor has a process.
pub fn actor_set_walk_speed(e: &mut Engine, this: Ptr<Actor>, speed: f32) {
    let process = e.get(this, Actor::pCurrentProcess);
    if !process.is_null() {
        fn_00884ce0(e, process.cast(), speed);
    }
}

// Translated from 00884ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::SetRunSpeed` (Xbox PDB): as `SetWalkSpeed`, through `00884d80`.
pub fn actor_set_run_speed(e: &mut Engine, this: Ptr<Actor>, speed: f32) {
    let process = e.get(this, Actor::pCurrentProcess);
    if !process.is_null() {
        fn_00884d80(e, process.cast(), speed);
    }
}

/// Runs the two bounds virtuals into scratch vectors (upper corner at
/// `scratch`, lower corner 12 bytes on) and returns the pointers they
/// returned, upper corner first.
fn bounds_pointers(e: &mut Engine, actor: Ptr<Actor>, scratch: Ptr) -> (u32, u32) {
    let upper = e
        .vcall(actor.addr(), VSLOT_BOUNDS_UPPER, &args![scratch.addr()])
        .u32();
    let lower = e
        .vcall(
            actor.addr(),
            VSLOT_BOUNDS_LOWER,
            &args![scratch.addr() + 12],
        )
        .u32();
    (upper, lower)
}

// Translated from 00885020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CachedValuesOwner::CalculateCachedRadius` (Xbox PDB name of slot
/// `+0x00`) as `Actor` implements it. The larger (`00404010`) of the
/// bounds' x and y extents, scaled by the reference scale, at least `32.0`,
/// halved. Stored in the process's cached values (`00885110`) when it has
/// them.
pub fn fn_00885020(e: &mut Engine, this: Ptr<CachedValuesOwner>) -> f32 {
    let actor = actor_of_owner(this);
    let (width, depth) = e.with_stack(24, |e, scratch| {
        bounds_pointers(e, actor, scratch);
        let upper_x = e.mem.f32(scratch.addr());
        let lower_x = e.mem.f32(scratch.addr() + 12);
        let upper_y = e.mem.f32(scratch.addr() + 4);
        let lower_y = e.mem.f32(scratch.addr() + 16);
        (
            (upper_x as f64 - lower_x as f64) as f32,
            (upper_y as f64 - lower_y as f64) as f32,
        )
    });
    let extent = e.call(FLOAT_HELPER_MAX, &args![width, depth]).f32();
    let scale = e.call(GET_SCALE, &args![actor]).f32();
    let extent = (scale as f64 * extent as f64) as f32;
    let minimum: f32 = e.global(MINIMUM_EXTENT);
    let extent = e.call(FLOAT_HELPER_MAX, &args![extent, minimum]).f32();
    let half: f64 = e.global(DOUBLE_HALF);
    let radius = (extent as f64 * half) as f32;
    let process = e.get(actor, Actor::pCurrentProcess);
    if !process.is_null() && e.call(HAS_CACHED_VALUES, &args![process]).bool() {
        fn_00885110(e, process.cast(), radius);
    }
    radius
}

// Translated from 00885110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the radius in the process's cached values and marks it valid
/// (`iFlags |= 1`).
pub fn fn_00885110(e: &mut Engine, this: Ptr<BaseProcess>, radius: f32) {
    let cached = e.get(this, BaseProcess::pCachedValues);
    if !cached.is_null() {
        e.set(cached, CachedValues::fCachedRadius, radius);
        e.call(CACHED_VALUES_ADD_FLAGS, &args![cached, CACHED_RADIUS_FLAG]);
    }
}

// Translated from 00885140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CachedValuesOwner::CalculateCachedWidth` (Xbox PDB name of slot
/// `+0x04`) as `Actor` implements it: the bounds' x extent times the
/// reference scale, stored in the cached values (`008851e0`) when the
/// process has them.
pub fn fn_00885140(e: &mut Engine, this: Ptr<CachedValuesOwner>) -> f32 {
    let actor = actor_of_owner(this);
    let extent = e.with_stack(24, |e, scratch| {
        bounds_pointers(e, actor, scratch);
        let upper = e.mem.f32(scratch.addr());
        let lower = e.mem.f32(scratch.addr() + 12);
        (upper as f64 - lower as f64) as f32
    });
    let scale = e.call(GET_SCALE, &args![actor]).f32();
    let width = (scale as f64 * extent as f64) as f32;
    let process = e.get(actor, Actor::pCurrentProcess);
    if !process.is_null() && e.call(HAS_CACHED_VALUES, &args![process]).bool() {
        fn_008851e0(e, process.cast(), width);
    }
    width
}

// Translated from 008851e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the width in the process's cached values and marks it valid
/// (`iFlags |= 2`).
pub fn fn_008851e0(e: &mut Engine, this: Ptr<BaseProcess>, width: f32) {
    let cached = e.get(this, BaseProcess::pCachedValues);
    if !cached.is_null() {
        e.set(cached, CachedValues::fCachedWidth, width);
        e.call(CACHED_VALUES_ADD_FLAGS, &args![cached, CACHED_WIDTH_FLAG]);
    }
}

// Translated from 00885210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CachedValuesOwner::CalculateCachedLength` (Xbox PDB name of slot
/// `+0x08`) as `Actor` implements it: like `00885140` with the y extent,
/// stored through `008852b0`.
pub fn fn_00885210(e: &mut Engine, this: Ptr<CachedValuesOwner>) -> f32 {
    let actor = actor_of_owner(this);
    let extent = e.with_stack(24, |e, scratch| {
        bounds_pointers(e, actor, scratch);
        let upper = e.mem.f32(scratch.addr() + 4);
        let lower = e.mem.f32(scratch.addr() + 16);
        (upper as f64 - lower as f64) as f32
    });
    let scale = e.call(GET_SCALE, &args![actor]).f32();
    let length = (scale as f64 * extent as f64) as f32;
    let process = e.get(actor, Actor::pCurrentProcess);
    if !process.is_null() && e.call(HAS_CACHED_VALUES, &args![process]).bool() {
        fn_008852b0(e, process.cast(), length);
    }
    length
}

// Translated from 008852b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the length in the process's cached values and marks it valid
/// (`iFlags |= 4`).
pub fn fn_008852b0(e: &mut Engine, this: Ptr<BaseProcess>, length: f32) {
    let cached = e.get(this, BaseProcess::pCachedValues);
    if !cached.is_null() {
        e.set(cached, CachedValues::fCachedLength, length);
        e.call(CACHED_VALUES_ADD_FLAGS, &args![cached, CACHED_LENGTH_FLAG]);
    }
}

// Translated from 008852e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CachedValuesOwner::CalculateCachedForwardLength` (Xbox PDB name of slot
/// `+0x0c`) as `Actor` implements it: the upper bounds corner's y times the
/// reference scale, stored through `00885360`. Only the upper corner is
/// fetched.
pub fn fn_008852e0(e: &mut Engine, this: Ptr<CachedValuesOwner>) -> f32 {
    let actor = actor_of_owner(this);
    let upper_y = e.with_stack(12, |e, scratch| {
        e.vcall(actor.addr(), VSLOT_BOUNDS_UPPER, &args![scratch.addr()]);
        e.mem.f32(scratch.addr() + 4)
    });
    let scale = e.call(GET_SCALE, &args![actor]).f32();
    let length = (scale as f64 * upper_y as f64) as f32;
    let process = e.get(actor, Actor::pCurrentProcess);
    if !process.is_null() && e.call(HAS_CACHED_VALUES, &args![process]).bool() {
        fn_00885360(e, process.cast(), length);
    }
    length
}

// Translated from 00885360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the forward length in the process's cached values and marks it
/// valid (`iFlags |= 0x8000`).
pub fn fn_00885360(e: &mut Engine, this: Ptr<BaseProcess>, length: f32) {
    let cached = e.get(this, BaseProcess::pCachedValues);
    if !cached.is_null() {
        e.set(cached, CachedValues::fCachedForwardLength, length);
        e.call(
            CACHED_VALUES_ADD_FLAGS,
            &args![cached, CACHED_FORWARD_LENGTH_FLAG],
        );
    }
}

/// The reference scale times the height between the bounds corners
/// (`virtual +0x1dc` z minus `virtual +0x1d8` z), read through the pointers
/// the virtuals return.
fn scaled_bounds_height(e: &mut Engine, this: Ptr<Actor>) -> f32 {
    let (upper, lower) = e.with_stack(24, |e, scratch| {
        let (upper, lower) = bounds_pointers(e, this, scratch);
        (e.mem.f32(upper + 8), e.mem.f32(lower + 8))
    });
    let difference = upper as f64 - lower as f64;
    let scale = e.call(GET_SCALE, &args![this]).f32();
    (scale as f64 * difference) as f32
}

// Translated from 008853a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetHeight` (Xbox PDB): the reference scale times the bounds'
/// z extent. With a process that passes `0045cd60` the height is cached in
/// the process (`fCachedActorHeight`, `0.0` meaning unset), computed and
/// stored when unset; otherwise it is computed every time.
pub fn actor_get_height(e: &mut Engine, this: Ptr<Actor>) -> f32 {
    let process = e.get(this, Actor::pCurrentProcess);
    if !process.is_null() && e.call(CACHED_HEIGHT_BYPASS, &args![process]).u32() == 0 {
        let cached = fn_00885490(e, process.cast());
        let unset: f64 = e.global(DOUBLE_ZERO);
        if cached as f64 == unset {
            let height = scaled_bounds_height(e, this);
            fn_008854b0(e, process.cast(), height);
            return height;
        }
        return cached;
    }
    scaled_bounds_height(e, this)
}

// Translated from 00885490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HighProcess::fCachedActorHeight` (Xbox PDB name of the field at
/// `+0x42c`): reads it.
pub fn fn_00885490(e: &mut Engine, this: Ptr<HighProcess>) -> f32 {
    e.get(this, HighProcess::fCachedActorHeight)
}

// Translated from 008854b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes `HighProcess::fCachedActorHeight` (Xbox PDB).
pub fn fn_008854b0(e: &mut Engine, this: Ptr<HighProcess>, height: f32) {
    e.set(this, HighProcess::fCachedActorHeight, x87_quiet(height));
}

// Translated from 008854d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetSneakHeight` (Xbox PDB): the eye level (`008be940`) when
/// `use_eye_level` is set, else `GetHeight`, times `0.57`.
pub fn actor_get_sneak_height(e: &mut Engine, this: Ptr<Actor>, use_eye_level: bool) -> f32 {
    let base = if use_eye_level {
        e.call(GET_EYE_LEVEL, &args![this]).f32()
    } else {
        actor_get_height(e, this)
    };
    let factor: f64 = e.global(SNEAK_HEIGHT_FACTOR);
    (base as f64 * factor) as f32
}

// Translated from 00885520 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when `threshold <= fn_00885560(point.z, cell)`: how far the actor
/// would be submerged at the point's height, compared with a fraction.
/// `point` points at a vector whose z (+8) is used.
pub fn fn_00885520(
    e: &mut Engine,
    this: Ptr<Actor>,
    point: Ptr,
    cell: Ptr,
    threshold: f32,
) -> bool {
    let z = e.mem.f32(point.addr() + 8);
    let depth = fn_00885560(e, this, z, cell);
    threshold as f64 <= depth as f64
}

// Translated from 00885560 (decompiled, FalloutNV.exe 1.4.0.525)
/// How much of the actor's height is under water at height `z`: `0` when
/// `z` is not below the water, otherwise `(water - z) / height` capped at
/// `1.0`. The water height is the given cell's (`005471e0`) unless the cell
/// is null or the actor's own cell, when it is the actor's relevant water
/// height (`0057b0a0`).
pub fn fn_00885560(e: &mut Engine, this: Ptr<Actor>, z: f32, cell: Ptr) -> f32 {
    let mut result = 0.0f32;
    let water = if !cell.is_null() && cell.addr() != e.call(GET_PARENT_CELL, &args![this]).u32() {
        e.call(CELL_GET_WATER_HEIGHT, &args![cell]).f32()
    } else {
        e.call(GET_RELEVANT_WATER_HEIGHT, &args![this]).f32()
    };
    if (z as f64) < water as f64 {
        let height = actor_get_height(e, this);
        result = ((water as f64 - z as f64) / height as f64) as f32;
        let one: f64 = e.global(DOUBLE_ONE);
        if result as f64 > one {
            result = 1.0;
        }
    }
    result
}

// Translated from 008855f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::AdjustSpeedForFollowing` (Xbox PDB). `speed` points at the
/// `float` movement speed being adjusted, `follow_distance` is the
/// package's follow value (turned into radii by the follow functions),
/// `distance` the current distance to the followed actor.
///
/// Does nothing unless the current package targets another actor in the
/// same area (the same cell when either cell is an interior, else the same
/// world space). The target's speed is scaled by the ratio of the two
/// references' scales. Closer than the mid radius the new speed is
/// `max(0, distance - near) / (mid - near)` times the lesser of the old
/// speed and the target's; from the far radius on, when the flag word has
/// all of `0x20f`, it is the target's speed times `1.25`; otherwise it is
/// `1 + (distance - mid) / (far - mid) * setting` times the target's speed.
/// The result is clamped between the old speed times two settings.
pub fn actor_adjust_speed_for_following(
    e: &mut Engine,
    this: Ptr<Actor>,
    speed: Ptr,
    follow_distance: f32,
    distance: f32,
) {
    // The result is stored by the game but never read.
    e.call(GET_CURRENT_PACKAGE, &args![this]);
    let target = Ptr::<Actor>::new(e.call(GET_CURRENT_PACKAGE_TARGET, &args![this]).u32());
    if target.is_null()
        || !e.vcall(target.addr(), VSLOT_IS_ACTOR, &args![]).bool()
        || target == this
    {
        return;
    }
    let cell = e.call(GET_PARENT_CELL, &args![this]).u32();
    let target_cell = e.call(GET_PARENT_CELL, &args![target]).u32();
    // The game also fetches this actor's world space here, unused.
    e.call(GET_WORLD_SPACE, &args![this]);
    let target_world_space = e.call(GET_WORLD_SPACE, &args![target]).u32();
    let interior = (target_cell != 0 && e.call(CELL_TEST_INTERIOR, &args![target_cell]).bool())
        || (cell != 0 && e.call(CELL_TEST_INTERIOR, &args![cell]).bool());
    let same_area = if interior {
        target_cell == cell
    } else {
        target_world_space == e.call(GET_WORLD_SPACE, &args![this]).u32()
    };
    if !same_area {
        return;
    }

    let near = e.call(FOLLOW_RADIUS_NEAR, &args![follow_distance]).f32();
    let mid = e.call(FOLLOW_RADIUS_MID, &args![follow_distance]).f32();
    let far = e
        .call(FOLLOW_RADIUS_MATCH_SPEED, &args![follow_distance])
        .f32();
    let target_speed = if e.vcall(target.addr(), VSLOT_IS_ACTOR, &args![]).bool() {
        e.call(GET_CURRENT_SPEED, &args![target]).f32()
    } else {
        0.0
    };
    let target_scale = e.call(GET_SCALE, &args![target]).f32() as f64;
    let own_scale = e.call(GET_SCALE, &args![this]).f32() as f64;
    let follow_speed = ((target_scale / own_scale) * target_speed as f64) as f32;

    let flags = e.call(GET_FLAG_WORD, &args![this]).u32();
    let all_flags = flags & FLAGS_MATCHING_PACE_MASK == FLAGS_MATCHING_PACE_MASK;
    let upper_factor = setting_value(
        e,
        if all_flags {
            SETTING_UPPER_FACTOR_ALL_FLAGS
        } else {
            SETTING_UPPER_FACTOR_OTHERWISE
        },
    );
    let upper_limit = (e.mem.f32(speed.addr()) as f64 * upper_factor as f64) as f32;
    let lower_factor = setting_value(
        e,
        if all_flags {
            SETTING_LOWER_FACTOR_ALL_FLAGS
        } else {
            SETTING_LOWER_FACTOR_OTHERWISE
        },
    );
    let lower_limit = (e.mem.f32(speed.addr()) as f64 * lower_factor as f64) as f32;

    if mid as f64 > distance as f64 {
        let nearer = e
            .call(
                FLOAT_HELPER_MAX,
                &args![0.0f32, (distance as f64 - near as f64) as f32],
            )
            .f32();
        let span = (mid as f64 - near as f64) as f32;
        let current = e.mem.f32(speed.addr());
        let capped = e
            .call(FLOAT_HELPER_MIN, &args![current, follow_speed])
            .f32();
        e.mem.set_f32(
            speed.addr(),
            ((nearer as f64 / span as f64) * capped as f64) as f32,
        );
    } else if far as f64 <= distance as f64 && all_flags {
        let catch_up: f64 = e.global(FOLLOW_CATCH_UP_FACTOR);
        e.mem
            .set_f32(speed.addr(), (follow_speed as f64 * catch_up) as f32);
    } else {
        let past_mid = (distance as f64 - mid as f64) as f32;
        let range = (far as f64 - mid as f64) as f32;
        let factor = setting_value(e, SETTING_INTERPOLATION_FACTOR);
        let ratio = (past_mid as f64 / range as f64) * factor as f64;
        e.mem
            .set_f32(speed.addr(), ((ratio + 1.0) * follow_speed as f64) as f32);
    }
    e.call(CLAMP_FLOAT, &args![speed, lower_limit, upper_limit]);
}

// Translated from 008858c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::ModifyMoveModeForFollow` (Xbox PDB): adjusts the movement mode
/// word `mode` for a follow package. Without an actor target the mode is
/// unchanged. Closer than the walk radius with none of the target's low
/// four flag bits: bit `0x100` replaces `0x200`. Otherwise, if
/// `00884750` allows it and the target's flags `& 0x500 == 0x400` or has
/// bit `0x200`, the target is farther than the match-speed radius, or is
/// the player while `mode` has bit `0x800`: bit `0x200` replaces `0x100`.
/// The third stack word is not read.
pub fn actor_modify_move_mode_for_follow(
    e: &mut Engine,
    this: Ptr<Actor>,
    mode: u32,
    follow_distance: f32,
    _unused_3: f32,
) -> u32 {
    let target = e.call(GET_CURRENT_PACKAGE_TARGET, &args![this]).u32();
    if target == 0 || !e.vcall(target, VSLOT_IS_ACTOR, &args![]).bool() {
        return mode;
    }
    let distance = e
        .call(
            GET_DISTANCE_FROM_REFERENCE,
            &args![this, target, 0u32, 0u32],
        )
        .f32();
    let walk_radius = e.call(FOLLOW_RADIUS_WALK, &args![follow_distance]).f32();
    let match_radius = e
        .call(FOLLOW_RADIUS_MATCH_SPEED, &args![follow_distance])
        .f32();
    let target_flags = e.call(GET_FLAG_WORD, &args![target]).u32();
    if walk_radius as f64 > distance as f64 && target_flags & TARGET_FLAGS_HOLD_MASK == 0 {
        return (mode & !MODE_BIT_200) | MODE_BIT_100;
    }
    if e.call(FOLLOW_MODE_ALLOWED, &args![this]).bool()
        && (target_flags & TARGET_FLAGS_MODE_MASK == TARGET_FLAGS_MODE_VALUE
            || (match_radius as f64) < distance as f64
            || (target == e.global::<u32>(PLAYER_CHARACTER) && mode & MODE_BIT_800 != 0)
            || target_flags & TARGET_FLAGS_BIT_200 != 0)
    {
        return (mode & !MODE_BIT_100) | MODE_BIT_200;
    }
    mode
}

// Translated from 008859e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// True unless the form behind `reference` (`BGSSaveFormBuffer::GetForm`)
/// has form type `0x15`, `0x27` or `0x30` and is missing flag
/// `0x20000000` (`00885a30`). `this` is not read (callers pass the player
/// singleton).
pub fn fn_008859e0(e: &mut Engine, _unused_this: Ptr, reference: Ptr) -> bool {
    let form = e.call(SAVE_FORM_BUFFER_GET_FORM, &args![reference]).u32();
    let form_type = e.call(FORM_TYPE, &args![form]).u32();
    if form_type == 0x15 || form_type == 0x27 || form_type == 0x30 {
        let form = e.call(SAVE_FORM_BUFFER_GET_FORM, &args![reference]).u32();
        return fn_00885a30(e, Ptr::new(form));
    }
    true
}

// Translated from 00885a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when `TESForm::iFormFlags` (Xbox PDB, +0x08) has bit `0x20000000`.
pub fn fn_00885a30(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 8) & FORM_FLAG_20000000 != 0
}

// Translated from 00885a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::CalculateWalkSpeed` (Xbox PDB name of the slot at `+0x454`; the
/// Actor vtable holds this function at `+0x458`). Asks `00647d10` for the
/// speed from the actor's value owner (`Actor + 0xa4`), the values of the
/// process's item (virtual `+0x148`) and of the worn armor in slot 2, and
/// four flags; multiplied by the combat controller's float at `+0xd8`
/// (`fn_00885bd0`) when the actor has a controller.
pub fn actor_calculate_walk_speed(e: &mut Engine, this: Ptr<Actor>) -> f32 {
    let speed_flag = e.call(SPEED_FLAG_TEST, &args![this]).bool();
    let mut process_flag_clear = true;
    let mut item = 0u32;
    let process = e.get(this, Actor::pCurrentProcess);
    if !process.is_null() {
        process_flag_clear = !e.vcall(process.addr(), PROCESS_VSLOT_FLAG, &args![]).bool();
        item = e.vcall(process.addr(), PROCESS_VSLOT_ITEM, &args![]).u32();
    }
    let armor = e
        .call(GET_ARMOR_BEING_WORN, &args![this, SPEED_ARMOR_SLOT])
        .u32();
    let item_value = if item != 0 {
        e.call(ITEM_VALUE, &args![item]).u32()
    } else {
        0
    };
    let armor_value = if armor != 0 {
        e.call(ITEM_VALUE, &args![armor]).u32()
    } else {
        0
    };
    if armor != 0 {
        e.call(ITEM_RELEASE, &args![armor, 1u32]);
    }
    let value_owner = if this.is_null() {
        0
    } else {
        this.addr() + ACTOR_VALUE_OWNER_OFFSET
    };
    let flag_0x358 = e.vcall(this.addr(), VSLOT_FLAG_0X358, &args![]).bool();
    let creature = e.vcall(this.addr(), VSLOT_IS_CREATURE, &args![]).bool();
    let mut speed = e
        .call(
            CALCULATE_WALK_SPEED_FORMULA,
            &args![
                value_owner,
                item_value,
                armor_value,
                speed_flag,
                process_flag_clear,
                creature,
                flag_0x358
            ],
        )
        .f32();
    let controller = e
        .vcall(this.addr(), VSLOT_GET_COMBAT_CONTROLLER, &args![])
        .u32();
    if controller != 0 {
        let multiplier = fn_00885bd0(e, Ptr::new(controller));
        speed = (multiplier as f64 * speed as f64) as f32;
    }
    speed
}

// Translated from 00885bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the float at `+0xd8` of the combat controller (the Xbox PDB has
/// `CombatController::fWalkSpeedMult` at `+0xe8`; the PC layout is 0x10
/// shorter).
pub fn fn_00885bd0(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + 0xd8)
}

// Translated from 00885bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::CalculateRunSpeed` (Xbox PDB name of the slot at `+0x458`; the
/// Actor vtable holds this function at `+0x45c`). Like
/// `CalculateWalkSpeed` with `00647f00` (the flag arguments in another
/// order, and the `004997b0` test only as an argument), multiplied by the
/// combat controller's float at `+0xdc` (`fn_00885d70`), then passed
/// through `BGSEntryPoint::HandleEntryPoint(0x2a, actor, &speed)`.
pub fn actor_calculate_run_speed(e: &mut Engine, this: Ptr<Actor>) -> f32 {
    let mut process_flag_clear = true;
    let mut item = 0u32;
    let process = e.get(this, Actor::pCurrentProcess);
    if !process.is_null() {
        process_flag_clear = !e.vcall(process.addr(), PROCESS_VSLOT_FLAG, &args![]).bool();
        item = e.vcall(process.addr(), PROCESS_VSLOT_ITEM, &args![]).u32();
    }
    let armor = e
        .call(GET_ARMOR_BEING_WORN, &args![this, SPEED_ARMOR_SLOT])
        .u32();
    let item_value = if item != 0 {
        e.call(ITEM_VALUE, &args![item]).u32()
    } else {
        0
    };
    let armor_value = if armor != 0 {
        e.call(ITEM_VALUE, &args![armor]).u32()
    } else {
        0
    };
    if armor != 0 {
        e.call(ITEM_RELEASE, &args![armor, 1u32]);
    }
    let value_owner = if this.is_null() {
        0
    } else {
        this.addr() + ACTOR_VALUE_OWNER_OFFSET
    };
    let flag_0x358 = e.vcall(this.addr(), VSLOT_FLAG_0X358, &args![]).bool();
    let speed_flag = e.call(SPEED_FLAG_TEST, &args![this]).bool();
    let creature = e.vcall(this.addr(), VSLOT_IS_CREATURE, &args![]).bool();
    let mut speed = e
        .call(
            CALCULATE_RUN_SPEED_FORMULA,
            &args![
                value_owner,
                item_value,
                armor_value,
                process_flag_clear,
                creature,
                speed_flag,
                flag_0x358
            ],
        )
        .f32();
    let controller = e
        .vcall(this.addr(), VSLOT_GET_COMBAT_CONTROLLER, &args![])
        .u32();
    if controller != 0 {
        let multiplier = fn_00885d70(e, Ptr::new(controller));
        speed = (multiplier as f64 * speed as f64) as f32;
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_f32(slot.addr(), speed);
        e.call(HANDLE_ENTRY_POINT, &args![0x2au32, this, slot]);
        e.mem.f32(slot.addr())
    })
}

// Translated from 00885d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the float at `+0xdc` of the combat controller (the Xbox PDB has
/// `CombatController::fRunSpeedMult` at `+0xec`; the PC layout is 0x10
/// shorter).
pub fn fn_00885d70(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + 0xdc)
}

/// The common body of `00885d90` and `00885ed0`: the process virtuals
/// `+0x454` (negated) and `+0x148` give the flag and the item, the worn
/// armor (slot 2) and the item give two values, and `formula` (`cdecl`, six
/// words: the `ActorValueOwner`, the two values, the flag, `IsCreature`,
/// the `+0x358` flag) gives the speed, a `float` in ST0.
fn weighted_speed(e: &mut Engine, this: Ptr<Actor>, formula: u32) -> f32 {
    let mut process_flag_clear = true;
    let mut item = 0u32;
    let process = e.get(this, Actor::pCurrentProcess);
    if !process.is_null() {
        process_flag_clear = !e.vcall(process.addr(), PROCESS_VSLOT_FLAG, &args![]).bool();
        item = e.vcall(process.addr(), PROCESS_VSLOT_ITEM, &args![]).u32();
    }
    let armor = e
        .call(GET_ARMOR_BEING_WORN, &args![this, SPEED_ARMOR_SLOT])
        .u32();
    let item_value = if item != 0 {
        e.call(ITEM_VALUE, &args![item]).u32()
    } else {
        0
    };
    let armor_value = if armor != 0 {
        e.call(ITEM_VALUE, &args![armor]).u32()
    } else {
        0
    };
    if armor != 0 {
        e.call(ITEM_RELEASE, &args![armor, 1u32]);
    }
    let value_owner = if this.is_null() {
        0
    } else {
        this.addr() + ACTOR_VALUE_OWNER_OFFSET
    };
    let flag_0x358 = e.vcall(this.addr(), VSLOT_FLAG_0X358, &args![]).bool();
    let creature = e.vcall(this.addr(), VSLOT_IS_CREATURE, &args![]).bool();
    e.call(
        formula,
        &args![
            value_owner,
            item_value,
            armor_value,
            process_flag_clear,
            creature,
            flag_0x358
        ],
    )
    .f32()
}

// Translated from 00885d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// A speed from the actor values through `00647f50` (the flag arguments as
/// in `CalculateWalkSpeed`, without its `004997b0` test and its
/// post-processing). The engine map has no name for it.
pub fn fn_00885d90(e: &mut Engine, this: Ptr<Actor>) -> f32 {
    weighted_speed(e, this, SPEED_FORMULA_00647F50)
}

// Translated from 00885ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The twin of `00885d90` with `00647f90` as the formula.
pub fn fn_00885ed0(e: &mut Engine, this: Ptr<Actor>) -> f32 {
    weighted_speed(e, this, SPEED_FORMULA_00647F90)
}

// Translated from 00886010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Zero when the `+0x358` virtual says so; otherwise `00647fd0` applied to
/// the actor values and the actor's `008a0c60` value (a `float` in ST0).
pub fn fn_00886010(e: &mut Engine, this: Ptr<Actor>) -> f32 {
    if e.vcall(this.addr(), VSLOT_FLAG_0X358, &args![]).bool() {
        return 0.0;
    }
    let value = e.call(ACTOR_SPEED_VALUE_008A0C60, &args![this]).f32();
    let value_owner = if this.is_null() {
        0
    } else {
        this.addr() + ACTOR_VALUE_OWNER_OFFSET
    };
    e.call(SPEED_FORMULA_00647FD0, &args![value_owner, value])
        .f32()
}

// Translated from 00886080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetCurrentPackageLocation` (Xbox PDB): the package location of
/// the object the process virtual `+0x27c` returns, or 0 without a process
/// or object (`0055b980` reads it).
pub fn actor_get_current_package_location(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    let process = e.get(this, Actor::pCurrentProcess);
    if process.is_null() {
        return 0;
    }
    let object = e.vcall(process.addr(), PROCESS_VSLOT_0X27C, &args![]).u32();
    if object == 0 {
        return 0;
    }
    e.call(PACKAGE_LOCATION_OF, &args![object]).u32()
}

// Translated from 008860d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves the actor to the process level `MobileObject::GetDesiredProcessLevel`
/// (`009334b0`) asks for. True when the actor stays (the level already is the
/// desired one, or it is handled and kept); false when the process is
/// missing or the reference was deleted or unloaded. The meanings of the
/// process virtual `+0x610` values 5 and 6 and of the actor virtuals
/// `+0x240` to `+0x24c` are not confirmed.
pub fn fn_008860d0(e: &mut Engine, this: Ptr<Actor>) -> bool {
    let process = e.get(this, Actor::pCurrentProcess);
    if process.is_null() {
        return false;
    }
    let current_level = e.call(CACHED_HEIGHT_BYPASS, &args![process]).u32();
    let desired_level = e.call(GET_DESIRED_PROCESS_LEVEL, &args![this]).u32();
    if current_level == desired_level {
        return true;
    }
    if current_level == 0 && e.vcall(process.addr(), PROCESS_VSLOT_0X610, &args![]).u32() != 0 {
        if e.vcall(process.addr(), PROCESS_VSLOT_0X610, &args![]).u32() == 6 {
            if !this.is_null() {
                e.vcall(this.addr(), VSLOT_DESTROY, &args![1u32]);
            }
            return false;
        }
        if e.vcall(process.addr(), PROCESS_VSLOT_0X610, &args![]).u32() == 5 {
            e.call(REFERENCE_DISABLE, &args![this]);
            return false;
        }
    }
    if desired_level != 0 {
        let package = e.vcall(process.addr(), PROCESS_VSLOT_0X27C, &args![]).u32();
        if package != 0
            && e.call(PACKAGE_TYPE, &args![package]).u32() == 8
            && e.vcall(process.addr(), PROCESS_VSLOT_0X128, &args![]).u32()
                == e.mem.u32(PLAYER_CHARACTER)
        {
            let mover = e.get(this, Actor::pActorMover);
            if !e.call(ACTOR_MOVER_TEST, &args![mover]).bool() {
                e.call(STOP_MOVING, &args![this]);
            }
        }
    }
    let mut unload = false;
    match desired_level {
        0 => {
            e.vcall(this.addr(), 0x240, &args![]);
        }
        1 => {
            e.vcall(this.addr(), 0x24c, &args![]);
        }
        2 => {
            e.vcall(this.addr(), 0x248, &args![]);
            unload = !e.call(GET_REF_PERSISTS, &args![this]).bool();
        }
        3 => {
            e.vcall(this.addr(), 0x244, &args![]);
            unload = !e.call(GET_REF_PERSISTS, &args![this]).bool();
        }
        _ => {}
    }
    if !unload {
        return true;
    }
    if !e.call(ACTOR_DELETE_TEST, &args![this]).bool()
        && e.call(GET_PARENT_CELL, &args![this]).u32() != 0
    {
        e.call(ACTOR_UNLOAD_PREPARE, &args![this]);
        e.call(ACTOR_DISPEL_ALL, &args![this.addr() + 0x94]);
        e.call(ACTOR_CAST_PERMANENT_MAGIC, &args![this, 0u32]);
        let manager = e.mem.u32(MANAGER_011DE45C);
        e.call(MANAGER_REMOVE_ACTOR, &args![manager, this]);
        let save_load = e.mem.u32(SAVE_LOAD_GAME);
        e.call(SAVE_LOAD_UNLOAD_FORM, &args![save_load, this, 0u32]);
    }
    if !this.is_null() {
        e.vcall(this.addr(), VSLOT_DESTROY, &args![1u32]);
    }
    false
}

// Translated from 00886360 (decompiled, FalloutNV.exe 1.4.0.525)
/// A per-frame step of an actor's process: timers, the merchant container
/// reset on a new day, the package evaluation, the dead-body alarm and the
/// check that moves a reference out of an unloaded cell. `delta` is the
/// frame time. It returns at once when `00440da0` says so or the actor is
/// not marked `bProcessMe`. Not named in the engine map; the meanings of most
/// callees are not confirmed.
pub fn fn_00886360(e: &mut Engine, this: Ptr<Actor>, delta: f32) {
    if e.call(ACTOR_SKIP_UPDATE_TEST, &args![this]).bool() {
        return;
    }
    if !e.get(this, Actor::bProcessMe) {
        return;
    }
    let player = e.mem.u32(PLAYER_CHARACTER);
    e.call(EMPTY_FUNCTION, &args![SCOPE_OBJECT_0X11DF69F]);
    e.vcall(this.addr(), VSLOT_UPDATE_HOOK_0X140, &args![]);
    let rotate = e.call(ACTOR_ROTATE_TO_TRACK_TEST, &args![this, 0u32]).u8();
    // Actor::bShouldRotateToTrack (Xbox PDB) +0x15d, stored as the raw byte.
    e.mem.set_u8(this.addr() + 0x15d, rotate);
    let step = e
        .call(UPDATE_STEP_SETTING, &args![UPDATE_STEP_OBJECT])
        .f32();
    let step = vats_scaled(e, this, step);
    let action_timer = e.get(this, Actor::fTimeronAction);
    if action_timer > 0.0 {
        e.set(
            this,
            Actor::fTimeronAction,
            (action_timer as f64 - step as f64) as f32,
        );
    } else {
        e.set(this, Actor::iActionValue, 0);
    }
    let process = e.call(GET_PROCESS, &args![this]).u32();
    if process != 0
        && !e
            .vcall(this.addr(), VSLOT_IS_PROCESS_FLAG_0X22C, &args![0u32])
            .bool()
    {
        let process = e.call(GET_PROCESS, &args![this]).u32();
        let stored_day = e.vcall(process, PROCESS_VSLOT_STORED_DAY, &args![]).u32();
        let day = e.call(CALENDAR_GET_DAY, &args![CALENDAR]).u8() as i8 as i32 as u32;
        if stored_day == day {
            // Actor::bContainerReset (Xbox PDB) +0x175.
            e.mem.set_u8(this.addr() + 0x175, 0);
        } else {
            if e.mem.u8(this.addr() + 0x175) == 0 {
                let month = e.call(CALENDAR_GET_MONTH, &args![CALENDAR]).u32();
                let first = e.call(GLOBAL_WORD_ADDRESS, &args![MONTH_SETTING_A]).u32();
                let matches = month == e.mem.u32(first) || {
                    let second = e.call(GLOBAL_WORD_ADDRESS, &args![MONTH_SETTING_B]).u32();
                    month == e.mem.u32(second)
                };
                if matches {
                    let lists = e.call(GET_EXTRA_DATA_LIST_ADDRESS, &args![this]).u32();
                    let merchant = e.call(EXTRA_GET_MERCHANT_CONTAINER, &args![lists]).u32();
                    if merchant != 0 {
                        let flag = e.call(MERCHANT_CONTAINER_TEST, &args![merchant]).bool();
                        e.vcall(merchant, 0x208, &args![!flag]);
                    }
                }
            }
            e.mem.set_u8(this.addr() + 0x175, 1);
        }
    }
    let process = e.call(GET_PROCESS, &args![this]).u32();
    let level = e.call(PROCESS_GET_LEVEL, &args![process]).u32();
    if level != 0 || e.call(PLAYER_SLEEPING_OR_RESTING, &args![player]).bool() {
        let mut time = delta;
        if e.call(PLAYER_SLEEPING_OR_RESTING, &args![player]).bool() {
            time = e.global::<f32>(SLEEP_UPDATE_TIME);
        } else if e.call(ACTOR_GET_IN_COMBAT, &args![this]).bool() {
            time = 0.0;
        }
        e.call(ACTOR_TIMED_UPDATE, &args![this, time]);
        e.call(ACTOR_UPDATE_MAGIC, &args![this, 0u32, 0u32]);
    }
    let not_processed = e
        .vcall(this.addr(), VSLOT_IS_PROCESS_FLAG_0X22C, &args![0u32])
        .bool();
    let dead = e.get(this, Actor::bDeadFlag);
    if not_processed
        || dead
        || e.mem.u8(PROCESSING_ENABLED) == 0
        || e.call(ACTOR_GET_LIFE_STATE, &args![this]).u32() == 6
    {
        if e.vcall(this.addr(), VSLOT_IS_PROCESS_FLAG_0X22C, &args![0u32])
            .bool()
        {
            dead_body_step(e, this, player, step);
        }
    } else {
        package_step(e, this, player, delta);
    }
    let mode = e.call(GLOBAL_WORD_ADDRESS, &args![MODE_SETTING]).u32();
    if e.mem.u32(mode) == 1 {
        let process = e.call(GET_PROCESS, &args![this]).u32();
        if e.call(PROCESS_GET_LEVEL, &args![process]).u32() == 0 {
            let object = e.mem.u32(COMBAT_OBJECT_POINTER);
            if !e.call(COMBAT_CONTROLLER_TEST, &args![object]).bool() {
                e.call(ACTOR_UPDATE_MAGIC, &args![this, 0u32, 0u32]);
            }
        }
    }
    if this.addr() != player && e.call(SAVE_FORM_BUFFER_GET_FORM, &args![this]).u32() != 0 {
        let process = e.call(GET_PROCESS, &args![this]).u32();
        if e.call(PROCESS_GET_LEVEL, &args![process]).u32() != 0 {
            move_out_of_cell(e, this);
        }
    }
    e.call(EMPTY_FUNCTION, &args![SCOPE_OBJECT_0X11DF69F]);
}

/// The part of `00886360` for an actor that is dead or not processed (its
/// first branch): while the virtual `+0x22c` holds, either the dead-body
/// timers run down and a killed-by-the-player actor tells the player's
/// process, or an essential-flag actor is handled; then the actor may be
/// queued on the temporary change list.
fn dead_body_step(e: &mut Engine, this: Ptr<Actor>, player: u32, step: f32) {
    let state = e.call(ACTOR_GET_LIFE_STATE, &args![this]).u32();
    if state != 1 {
        let process = e.get(this, Actor::pCurrentProcess);
        e.vcall(process.addr(), 0x28, &args![]);
        let body_timer = e.get(this, Actor::fCheckMyDeadBodyTimer);
        e.set(
            this,
            Actor::fCheckMyDeadBodyTimer,
            (body_timer as f64 - step as f64) as f32,
        );
        let alarm = e.get(this, Actor::fDeadBodyAlarm);
        let alarm = (alarm as f64 - step as f64) as f32;
        e.set(this, Actor::fDeadBodyAlarm, alarm);
        if alarm <= 0.0 {
            if e.get(this, Actor::pMyKiller).addr() == player {
                let player_process = e.call(GET_PROCESS, &args![player]).u32();
                let seen = e.vcall(player_process, 0xf8, &args![player]).u32();
                if seen == 0 {
                    let player_process = e.call(GET_PROCESS, &args![player]).u32();
                    let position = e
                        .vcall(this.addr(), VSLOT_GET_POSITION_0X1F4, &args![])
                        .u32();
                    let x = e.mem.u32(position);
                    let y = e.mem.u32(position + 4);
                    let z = e.mem.u32(position + 8);
                    e.vcall(
                        player_process,
                        PROCESS_VSLOT_0XFC,
                        &args![player, x, y, z, 1u32, 4u32, 0u32],
                    );
                }
            }
            let reset = e.global::<f32>(DEAD_BODY_ALARM_RESET);
            e.set(this, Actor::fDeadBodyAlarm, reset);
        }
    } else if e.call(ACTOR_GET_LIFE_STATE, &args![this]).u32() == 1 {
        let setting = e
            .call(SETTING_VALUE_POINTER, &args![ESSENTIAL_SETTING])
            .u32();
        if e.mem.u8(setting) != 0 && e.call(ACTOR_GET_ESSENTIAL, &args![this]).bool() {
            e.vcall(this.addr(), 0x324, &args![0u32, 0u32, 1u32]);
        } else {
            let process = e.get(this, Actor::pCurrentProcess);
            e.vcall(process.addr(), 0x10, &args![this]);
        }
    }
    if !e.call(ACTOR_GET_ESSENTIAL, &args![this]).bool()
        && e.call(ACTOR_FLAG_TEST_0087F4A0, &args![this]).bool()
    {
        let cell = e.call(GET_PARENT_CELL, &args![this]).u32();
        // 00886968 MOV ECX,[0x011dea10]: the TES object the pointer holds.
        let tes = e.mem.u32(TES_POINTER);
        if !e.call(TES_IS_CELL_LOADED, &args![tes, cell, 0u32]).bool() {
            let process = e.call(GET_PROCESS, &args![this]).u32();
            if process != 0 {
                let wait = e.vcall(process, PROCESS_VSLOT_WAIT_TIME, &args![]).f32();
                if wait != 0.0 {
                    let wait = e.vcall(process, PROCESS_VSLOT_WAIT_TIME, &args![]).f32();
                    let now_low = e.call(TIME_NOW_LOW, &args![]).u32();
                    let now = e.call(CALENDAR_GET_TIME, &args![CALENDAR]).u32();
                    if (now_low as f64 + wait as f64) < now as f64 {
                        e.call(ADD_ACTOR_TO_TEMP_CHANGE_LIST, &args![PROCESS_LISTS, this]);
                    }
                }
            }
        }
    }
}

/// The part of `00886360` for an actor that is alive and processed (its
/// second branch): the movement-mover check, the day change, the package
/// evaluation and the process's own update.
fn package_step(e: &mut Engine, this: Ptr<Actor>, player: u32, delta: f32) {
    let setting = e.call(SETTING_VALUE_POINTER, &args![MOVER_SETTING]).u32();
    if e.mem.u8(setting) != 0 {
        let process = process_of(e, this);
        if !e.vcall(process, PROCESS_VSLOT_0X35C, &args![]).bool() {
            let mover = e.get(this, Actor::pActorMover);
            if !mover.is_null() && e.call(ACTOR_MOVER_TEST_0X9DCB90, &args![mover]).bool() {
                e.call(ACTOR_MOVER_RESPONSE, &args![this]);
            }
        }
    }
    let process = process_of(e, this);
    let stored_day = e.vcall(process, PROCESS_VSLOT_STORED_DAY, &args![]).u8();
    let day = e.call(CALENDAR_GET_DAY, &args![CALENDAR]).u8();
    if day as i8 != stored_day as i8 {
        let lists = e.call(GET_EXTRA_DATA_LIST_ADDRESS, &args![this]).u32();
        e.call(EXTRA_LIST_DAY_CHANGED, &args![lists]);
    }
    if e.vcall(this.addr(), VSLOT_EVALUATE_PACKAGE_TEST_0X26C, &args![])
        .bool()
    {
        let flag = e.call(ACTOR_EVALUATE_FLAG, &args![this]).u8();
        e.call(ACTOR_EVALUATE_PACKAGE, &args![this, 1u32, flag as u32]);
    }
    if !e
        .vcall(this.addr(), VSLOT_PACKAGE_TEST_0X230, &args![])
        .bool()
    {
        let mut update = false;
        if e.mem.u8(INTERRUPT_UPDATES_ENABLED) == 0 {
            let process = process_of(e, this);
            let package = e.vcall(process, PROCESS_VSLOT_OBJECT, &args![]).u32();
            if package != 0 {
                let process = process_of(e, this);
                let package = e.vcall(process, PROCESS_VSLOT_OBJECT, &args![]).u32();
                update = e.call(PACKAGE_IS_INTERRUPT, &args![package]).bool();
            }
        } else {
            update = true;
        }
        if update {
            let process = process_of(e, this);
            let package = e.vcall(process, PROCESS_VSLOT_OBJECT, &args![]).u32();
            let mut skip = false;
            if package != 0 {
                let process = process_of(e, this);
                let package = e.vcall(process, PROCESS_VSLOT_OBJECT, &args![]).u32();
                skip = e.call(OBJECT_TYPE, &args![package]).u32() == 0x1b;
            }
            if !skip {
                let process = process_of(e, this);
                if delta == 0.0 || this.addr() == player {
                    e.vcall(process, 0x10, &args![this]);
                } else {
                    e.vcall(process, 0xc, &args![this, delta]);
                }
            }
        }
    }
    let day = e.call(CALENDAR_GET_DAY, &args![CALENDAR]).u8() as u32;
    let lists = e.call(GET_EXTRA_DATA_LIST_ADDRESS, &args![this]).u32();
    e.call(EXTRA_LIST_SET_DAY, &args![lists, day]);
}

// Translated from 00886b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns `Actor::bResetAI` (Xbox PDB, +0x146 on PC).
pub fn fn_00886b10(e: &mut Engine, this: Ptr<Actor>) -> bool {
    e.get(this, Actor::bResetAI)
}

// Translated from 00886b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the actor has a process and the process virtual `+0x40c` says six
/// (or, failing that, the virtual `+0x22c` or the process virtual `+0x6fc`
/// say so): blends the actor's bip transform (`bhkBlendCollisionObject::
/// SetBipTransform`, `00c821b0`), clears a flag on the character controller
/// and rotates the 3-vector at `vector` by the actor's heading (a
/// z-rotation matrix built from the `float` at `+0x30` of the actor's
/// `+0x24` vector). Returns true when it did so. The meanings of the
/// virtuals `+0x230`, `+0x40c` and `+0x6fc` are not confirmed.
pub fn fn_00886b30(e: &mut Engine, this: Ptr<Actor>, vector: Ptr) -> bool {
    let process = e.call(GET_PROCESS, &args![this]).u32();
    if process == 0 {
        return false;
    }
    let mut proceed = false;
    if e.vcall(this.addr(), VSLOT_PACKAGE_TEST_0X230, &args![])
        .bool()
    {
        let process = e.call(GET_PROCESS, &args![this]).u32();
        if e.vcall(process, PROCESS_VSLOT_0X40C, &args![]).u32() != 6 {
            proceed = true;
        }
    }
    if !proceed {
        if e.vcall(this.addr(), VSLOT_IS_PROCESS_FLAG_0X22C, &args![0u32])
            .bool()
        {
            proceed = true;
        } else {
            let process = e.call(GET_PROCESS, &args![this]).u32();
            if e.vcall(process, PROCESS_VSLOT_0X6FC, &args![]).bool() {
                proceed = true;
            }
        }
    }
    if !proceed {
        return false;
    }
    let not_flag = !e
        .vcall(this.addr(), VSLOT_IS_PROCESS_FLAG_0X22C, &args![0u32])
        .bool();
    let blend = e.call(BLEND_OBJECT_OF, &args![this]).u32();
    e.call(SET_BIP_TRANSFORM, &args![blend, vector, not_flag]);
    let controller = e.call(GET_CHAR_CONTROLLER, &args![this]).u32();
    if controller != 0 {
        fn_00886c80(e, Ptr::new(controller), true);
    }
    let rotation = e.call(ACTOR_ROTATION_VECTOR, &args![this]).u32();
    let heading = e.mem.f32(rotation + 8);
    e.with_stack(0x30, |e, base| {
        let matrix = base.addr();
        let result = base.addr() + 0x24;
        e.call(MATRIX_CONSTRUCT, &args![matrix]);
        e.call(MATRIX_MAKE_ROTATION, &args![matrix, heading]);
        let rotated = e
            .call(VECTOR_TIMES_MATRIX, &args![result, vector, matrix])
            .u32();
        for offset in [0, 4, 8] {
            let word = e.mem.u32(rotated + offset);
            e.mem.set_u32(vector.addr() + offset, word);
        }
    });
    true
}

// Translated from 00886c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears the bit `0x1000` of the flag word at `+0x410` of the
/// character controller (`00629670`: `flag` set ORs the mask into the word
/// at +4 of that sub-object, otherwise `006296b0` clears it).
pub fn fn_00886c80(e: &mut Engine, this: Ptr, flag: bool) {
    e.call(
        FLAG_SET_OR_CLEAR,
        &args![this.addr() + 0x410, 0x1000u32, flag],
    );
}

// Translated from 008879d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same as `00886c80` with the bit `0x40000`.
pub fn fn_008879d0(e: &mut Engine, this: Ptr, flag: bool) {
    e.call(
        FLAG_SET_OR_CLEAR,
        &args![this.addr() + 0x410, 0x40000u32, flag],
    );
}

// Translated from 00887a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the global word at `011c6260`.
pub fn fn_00887a00(e: &mut Engine) -> u32 {
    e.mem.u32(GLOBAL_VALUE_011C6260)
}

// Translated from 00887a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The object behind `this`'s `004ae750` pointer, passed through `0084e3a0`
/// (which reads its word at +0xc); without it, the word the pointer at
/// `this + 0xc` points to.
pub fn fn_00887a10(e: &mut Engine, this: Ptr) -> u32 {
    let proxy = e.call(PROXY_OF, &args![this]).u32();
    if proxy != 0 {
        return e.call(WORD_AT_0XC, &args![proxy]).u32();
    }
    let link = e.mem.u32(this.addr() + 0xc);
    if link == 0 {
        return 0;
    }
    // The pointer is adjusted by -4 and the word at +4 of that is read.
    e.mem.u32(link)
}

// Translated from 00887a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Hands the object `00887a10` finds a 4-byte value built from `flag`
/// (`00622570` stores the byte) and the `004ae750` pointer of `this`
/// (`hkpLimitedHingeConstraintData::setMotorActive` in the engine map).
pub fn fn_00887a80(e: &mut Engine, this: Ptr, flag: bool) {
    let target = fn_00887a10(e, this);
    if target == 0 {
        return;
    }
    let value = e.with_stack(4, |e, slot| {
        e.call(BYTE_STORE, &args![slot, flag]);
        e.mem.u32(slot.addr())
    });
    let proxy = e.call(PROXY_OF, &args![this]).u32();
    e.call(SET_MOTOR_ACTIVE, &args![target, proxy, value]);
}

// Translated from 00887ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Passes `value` to `009314d0` of the object `00887a10` finds, if any.
pub fn fn_00887ac0(e: &mut Engine, this: Ptr, value: f32) {
    let target = fn_00887a10(e, this);
    if target != 0 {
        e.call(SET_FLOAT_ON_TARGET, &args![target, value]);
    }
}

// Translated from 00887af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards `value` to `00887b20` of the `004ae750` pointer of `this`.
pub fn fn_00887af0(e: &mut Engine, this: Ptr, value: u32) {
    let proxy = e.call(PROXY_OF, &args![this]).u32();
    fn_00887b20(e, Ptr::new(proxy), value);
}

// Translated from 00887b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Passes `value` to `004a3f10` of the sub-object at +0x40.
pub fn fn_00887b20(e: &mut Engine, this: Ptr, value: u32) {
    e.call(SUBOBJECT_SET_VALUE, &args![this.addr() + 0x40, value]);
}

// Translated from 00887b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards the byte `value` to `00887b70` of the `004ae750` pointer of
/// `this`.
pub fn fn_00887b40(e: &mut Engine, this: Ptr, value: u8) {
    let proxy = e.call(PROXY_OF, &args![this]).u32();
    fn_00887b70(e, Ptr::new(proxy), value);
}

// Translated from 00887b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the byte `value` at +0x5c.
pub fn fn_00887b70(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x5c, value);
}

/// The frame time scaled by the VATS multiplier when the VATS state is 4
/// and `this` is the VATS target (the common prologue of several updates):
/// `value * 009c8d60()`, rounded to `float`.
fn vats_scaled(e: &mut Engine, this: Ptr<Actor>, value: f32) -> f32 {
    if e.call(SETTING_INT_VALUE, &args![VATS_STATE_OBJECT]).u32() == 4
        && this.addr() == e.mem.u32(VATS_TARGET_ACTOR)
    {
        let multiplier = e
            .call(VATS_TARGET_UPDATE_MULT, &args![VATS_STATE_OBJECT])
            .f32();
        return (multiplier as f64 * value as f64) as f32;
    }
    value
}

/// Moves the reference to the world space of its cell when its position is
/// outside the cell (the tail `00887b90` and `00886360` share): the cell,
/// the position and the three tests, then `MoveRefToNewSpace(this, 0,
/// worldspace)`.
fn move_out_of_cell(e: &mut Engine, this: Ptr<Actor>) {
    let cell = e.call(GET_PARENT_CELL, &args![this]).u32();
    let position = e
        .vcall(this.addr(), VSLOT_GET_POSITION_0X1F4, &args![])
        .u32();
    let copy = [
        e.mem.u32(position),
        e.mem.u32(position + 4),
        e.mem.u32(position + 8),
    ];
    if cell == 0 || e.call(CELL_TEST_INTERIOR, &args![cell]).bool() {
        return;
    }
    let inside = e.with_stack(12, |e, slot| {
        for (i, word) in copy.iter().enumerate() {
            e.mem.set_u32(slot.addr() + 4 * i as u32, *word);
        }
        e.call(CELL_CONTAINS_POSITION, &args![cell, slot]).bool()
    });
    if !inside {
        let world_space = e.call(CELL_GET_WORLD_SPACE, &args![cell]).u32();
        e.call(MOVE_REF_TO_NEW_SPACE, &args![this, 0u32, world_space]);
    }
}

// Translated from 00887b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// A per-frame step for a non-player actor's process (muzzle flash update,
/// the cell check and the fade update), `delta` the frame time. It does
/// nothing when `00576d30` (a flag of the reference) says so. The second
/// stack word is not read.
pub fn fn_00887b90(e: &mut Engine, this: Ptr<Actor>, delta: f32, _unused_1: u32) {
    if e.call(REFERENCE_FLAG_TEST_0X200000, &args![this]).bool() {
        return;
    }
    let delta = vats_scaled(e, this, delta);
    let process = e.get(this, Actor::pCurrentProcess);
    if !process.is_null()
        && !e
            .call(IS_MENU_ID_VISIBLE, &args![MENU_ID_0X3F0, 0u32])
            .bool()
    {
        e.vcall(process.addr(), PROCESS_VSLOT_0X464, &args![this]);
        if e.vcall(process.addr(), PROCESS_VSLOT_MUZZLE_FLASH, &args![])
            .u32()
            != 0
        {
            let flash = e
                .vcall(process.addr(), PROCESS_VSLOT_MUZZLE_FLASH, &args![])
                .u32();
            e.call(MUZZLE_FLASH_UPDATE, &args![flash, delta, this]);
        }
    }
    if e.call(SAVE_FORM_BUFFER_GET_FORM, &args![this]).u32() != 0 {
        move_out_of_cell(e, this);
    }
    let process = e.get(this, Actor::pCurrentProcess);
    if !process.is_null() && e.call(PROCESS_GET_LEVEL, &args![process]).u32() == 0 {
        e.call(HIGH_PROCESS_FADE_UPDATE, &args![process, this]);
    }
}

// Translated from 00887d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::ResetLoadedAnimations` (Xbox PDB): for a non-player actor with
/// a 3D object, either queues the reset on the task queue
/// (`TaskQueueInterface::QueueActorResetLoadedAnims`) when `008c7aa0`
/// says so, or replaces the actor's animation with a fresh one: clears the
/// process's three entries (virtual `+0x424` for 0 to 2), picks the
/// animation set from the process's item, builds the KF file list, inits
/// the new animation, logs when that fails and, for an actor not in life
/// state 2 whose base form has KF files, adds the special animations of
/// the model's folder. The exception frame is not translated.
pub fn actor_reset_loaded_animations(e: &mut Engine, this: Ptr<Actor>) {
    if this.addr() == e.mem.u32(PLAYER_CHARACTER) {
        return;
    }
    if e.vcall(this.addr(), VSLOT_GET_3D, &args![]).u32() == 0 {
        return;
    }
    if e.call(RESET_ANIMS_SHOULD_QUEUE, &args![]).bool() {
        let queue = e.call(TASK_QUEUE_INTERFACE, &args![]).u32();
        e.call(QUEUE_ACTOR_RESET_LOADED_ANIMS, &args![queue, this]);
        return;
    }
    e.call(SET_ANIMATION, &args![this, 0u32]);
    let process = e.get(this, Actor::pCurrentProcess);
    if !process.is_null() {
        e.vcall(process.addr(), PROCESS_VSLOT_0X424, &args![0u32, 0u32]);
        e.vcall(process.addr(), PROCESS_VSLOT_0X424, &args![1u32, 0u32]);
        e.vcall(process.addr(), PROCESS_VSLOT_0X424, &args![2u32, 0u32]);
    }
    let mut kf_group = 0u32;
    if e.call(GET_PROCESS, &args![this]).u32() != 0 {
        let process = e.call(GET_PROCESS, &args![this]).u32();
        let level = e.call(PROCESS_GET_LEVEL, &args![process]).i32();
        if (0..=1).contains(&level) {
            let process = e.call(GET_PROCESS, &args![this]).u32();
            if e.vcall(process, PROCESS_VSLOT_ITEM, &args![]).u32() != 0 {
                let item = e.vcall(process, PROCESS_VSLOT_ITEM, &args![]).u32();
                let value = e.call(ITEM_VALUE, &args![item]).u32();
                let index = e.call(ITEM_GROUP_INDEX, &args![value]).i32();
                kf_group = e
                    .mem
                    .u32(KF_GROUP_TABLE.wrapping_add((index as u32).wrapping_mul(4)));
            } else if e.call(ACTOR_ANIMATION_GROUP_TEST, &args![this]).bool() {
                kf_group = 1;
            }
        }
    }
    let raw = e.call(OPERATOR_NEW_FN, &args![ANIMATION_SIZE]).u32();
    let animation = if raw != 0 {
        e.call(ANIMATION_CONSTRUCTOR, &args![raw]).u32()
    } else {
        0
    };
    let new_animation = e.call(SET_ANIMATION, &args![this, animation]).u32();
    let kf_list = e.call(BUILD_KF_FILE_LIST, &args![this, kf_group]).u32();
    let base = base_form(e, this).addr();
    let animation_slot = e.vcall(this.addr(), VSLOT_GET_3D, &args![]).u32();
    let not_flag = !e
        .vcall(this.addr(), VSLOT_IS_PROCESS_FLAG_0X22C, &args![0u32])
        .bool();
    let item = e.vcall(animation_slot, 0xc, &args![]).u32();
    let ok = e
        .call(
            INIT_ANIMATION,
            &args![new_animation, kf_list, item, this, not_flag],
        )
        .bool();
    if !ok {
        let form_id = e.call(WORD_AT_0XC, &args![base]).u32();
        let name = e.vcall(base, 0x130, &args![]).u32();
        e.call(
            LOG_MESSAGE,
            &args![BAD_INIT_ANIMATION_FORMAT, name, form_id],
        );
    }
    let life_state = e.get(this, Actor::eLifeState);
    if life_state != 2 && e.call(HAS_KF_FILES, &args![base + 0xc4]).bool() {
        let model = e.call(GET_MODEL, &args![this]).u32();
        e.with_stack(0x104, |e, buffer| {
            e.call(STRING_COPY, &args![buffer, 0x104u32, model]);
            let slash = e.call(STRING_FIND_LAST, &args![buffer, 0x5cu32]).u32();
            if slash != 0 {
                e.mem.set_u8(slash, 0);
                let running = e.call(PACKAGE_RUNNING_OF, &args![base + 0xc4]).u32();
                e.call(
                    ADD_SPECIAL_ANIMATIONS,
                    &args![new_animation, running, buffer],
                );
            }
        });
    }
}

// Translated from 00886cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A per-frame physics and 3D step of an actor: refreshes the swim flag
/// (`00884990`), moves a non-player actor along its mover's path
/// (`SetLocationOnReference`), updates the level-of-detail flag of the
/// shader accumulator, plays out the death ragdoll or knockdown once
/// `bDeadFlag` is set, and, for an actor with the turret flag (`+0x1b0`),
/// re-syncs the collision objects of its 3D object with the havok world
/// (two passes with the globals of `00887a00` and `0087e9a0`). `delta` is
/// the frame time (the VATS-scaled step when it is 0 and the process type
/// is 0); the second stack word is not read. Not named in the engine map;
/// meanings of most callees and slots are not confirmed.
pub fn fn_00886cb0(e: &mut Engine, this: Ptr<Actor>, delta: f32, _unused_2: u32) {
    let player = e.mem.u32(PLAYER_CHARACTER);
    e.call(EMPTY_FUNCTION, &args![SCOPE_OBJECT_0X11DF69E]);
    let mut delta = delta;
    if delta == 0.0 && e.call(GET_CURRENT_PROCESS_TYPE, &args![this]).u32() == 0 {
        if e.call(SETTING_INT_VALUE, &args![VATS_STATE_OBJECT]).u32() == 4
            && this.addr() == e.mem.u32(VATS_TARGET_ACTOR)
        {
            let step = e
                .call(UPDATE_STEP_SETTING, &args![UPDATE_STEP_OBJECT])
                .f64();
            let multiplier = e
                .call(VATS_TARGET_UPDATE_MULT, &args![VATS_STATE_OBJECT])
                .f64();
            delta = (multiplier * step) as f32;
        } else {
            delta = e
                .call(UPDATE_STEP_SETTING, &args![UPDATE_STEP_OBJECT])
                .f32();
        }
    }
    fn_00884990(e, this);
    if this.addr() != player {
        follow_mover_path(e, this, player, delta);
    }
    if e.call(ACTOR_GET_IN_COMBAT, &args![this]).bool() {
        e.call(ACTOR_FUNCTION_0X8B00C0, &args![this]);
    }
    let process = e.get(this, Actor::pCurrentProcess);
    let turret_object = if process.is_null() {
        0
    } else {
        e.vcall(process.addr(), PROCESS_VSLOT_0X28C, &args![]).u32()
    };
    if this.addr() != player {
        let skeleton = e.vcall(this.addr(), VSLOT_GET_3D, &args![]).u32();
        if this.addr() != player && skeleton != 0 {
            level_of_detail(e, this, player, skeleton, turret_object);
        }
    }
    if turret_object != 0 {
        let flag = e.call(GLOBAL_BYTE_FN_525420, &args![]).u8();
        e.call(TURRET_SET_FLAG_0X810610, &args![turret_object, flag as u32]);
    }
    if e.get(this, Actor::bDeadFlag) && e.call(GET_PROCESS, &args![this]).u32() != 0 {
        let collision = e.call(BLEND_OBJECT_OF, &args![this]).u32();
        if collision != 0 {
            dead_ragdoll(e, this, collision);
        }
    }
    if e.call(ACTOR_TURRET_BEHAVIOR, &args![this]).bool()
        && !e
            .vcall(this.addr(), VSLOT_IS_PROCESS_FLAG_0X22C, &args![0u32])
            .bool()
        && !e.call(ACTOR_BLOCKED_TEST, &args![this]).bool()
    {
        let skeleton = e.vcall(this.addr(), VSLOT_GET_3D, &args![]).u32();
        let mut changed = false;
        let key = e.call(GLOBAL_WORD_FN_87E9A0, &args![]).u32();
        changed |= resync_collision(e, this, skeleton, key, false);
        let key = fn_00887a00(e);
        changed |= resync_collision(e, this, skeleton, key, true);
        if changed {
            e.vcall(this.addr(), VSLOT_0X48, &args![4u32]);
        }
    }
    e.call(EMPTY_FUNCTION, &args![SCOPE_OBJECT_0X11DF69E]);
}

/// The path part of [`fn_00886cb0`] for a non-player actor: the mover's
/// next position (virtuals `+0x20`, `+0x24` of `pActorMover`, after
/// `00886b30`) and either `Actor` virtual `+0x250` or a plain
/// `SetLocationOnReference` when the cell is not ready.
fn follow_mover_path(e: &mut Engine, this: Ptr<Actor>, player: u32, delta: f32) {
    e.with_stack(0x80, |e, base| {
        let vector = base.addr();
        let matrix = base.addr() + 0x10;
        let scratch_a = base.addr() + 0x40;
        let scratch_b = base.addr() + 0x50;
        let scratch_c = base.addr() + 0x60;
        let mut extra = 0u32;
        for i in 0..3 {
            let word = e.mem.u32(SCALE_VECTOR + 4 * i);
            e.mem.set_u32(vector + 4 * i, word);
        }
        if !e.vcall(this.addr(), VSLOT_0X4B4, &args![]).bool()
            && !fn_00886b30(e, this, Ptr::new(vector))
        {
            let mover = e.get(this, Actor::pActorMover);
            extra = e.vcall(mover.addr(), MOVER_VSLOT_0X20, &args![]).u32();
            let mover = e.get(this, Actor::pActorMover);
            if !e
                .vcall(mover.addr(), MOVER_VSLOT_0X24, &args![vector])
                .bool()
            {
                let animation = e.call(ACTOR_GET_ANIMATION, &args![this]).u32();
                if animation != 0 {
                    e.call(
                        ANIMATION_SCALE_VECTOR,
                        &args![animation, vector, this, 0u32, 1u32],
                    );
                }
            }
        }
        let cell = e.call(GET_PARENT_CELL, &args![this]).u32();
        if cell == 0
            || !e.call(CELL_TEST_0X450FF0, &args![cell]).bool()
            || e.call(PLAYER_SLEEPING_OR_RESTING, &args![player]).bool()
        {
            e.call(MATRIX_CONSTRUCT, &args![matrix]);
            let rotation = e.call(ACTOR_ROTATION_VECTOR, &args![this]).u32();
            let heading = e.mem.f32(rotation + 8);
            e.call(MATRIX_MAKE_ROTATION, &args![matrix, heading]);
            let rotated = e
                .call(MATRIX_TIMES_VECTOR, &args![matrix, scratch_a, vector])
                .u32();
            let position = e
                .vcall(this.addr(), VSLOT_GET_POSITION_0X1F4, &args![])
                .u32();
            let moved = e
                .call(VECTOR_PLUS_VECTOR, &args![position, scratch_b, rotated])
                .u32();
            e.call(SET_LOCATION_ON_REFERENCE, &args![this, moved]);
        } else {
            e.vcall(this.addr(), VSLOT_0X250, &args![delta, vector, extra]);
        }
        let _ = scratch_c;
    });
}

/// The shader accumulator part of [`fn_00886cb0`]: works out whether the
/// actor is far enough to drop out of detail (the distance from the head
/// target to the world bound, less the bound's radius scaled by 1.25) and
/// tells the accumulator and the turret object.
fn level_of_detail(e: &mut Engine, this: Ptr<Actor>, player: u32, skeleton: u32, turret: u32) {
    e.with_stack(0x80, |e, base| {
        let target = base.addr();
        let scaled = base.addr() + 0x10;
        let sum = base.addr() + 0x20;
        let bound = base.addr() + 0x30;
        let difference = base.addr() + 0x40;
        e.call(MATRIX_CONSTRUCT, &args![target]);
        let head_a = e.mem.u32(HEAD_TARGET_A);
        let source = if e.call(PLAYER_STATE_TEST, &args![player]).bool() && head_a != 0 {
            e.call(OBJECT_POSITION_0X8C, &args![head_a]).u32()
        } else if e.mem.u32(HEAD_TARGET_B) != 0 {
            let head_b = e.mem.u32(HEAD_TARGET_B);
            e.call(OBJECT_POSITION_0X8C, &args![head_b]).u32()
        } else {
            let eye = e.call(GET_EYE_LEVEL, &args![player]).f32();
            let scaled_vector = e
                .call(VECTOR_TIMES_SCALAR, &args![scaled, eye, EYE_DIRECTION])
                .u32();
            let position = e.call(POSITION_VECTOR_OF, &args![player]).u32();
            e.call(VECTOR_PLUS_VECTOR, &args![position, sum, scaled_vector])
                .u32()
        };
        for i in 0..3 {
            let word = e.mem.u32(source + 4 * i);
            e.mem.set_u32(target + 4 * i, word);
        }
        let world_bound = e.call(GET_WORLD_BOUND, &args![skeleton]).u32();
        for i in 0..4 {
            let word = e.mem.u32(world_bound + 4 * i);
            e.mem.set_u32(bound + 4 * i, word);
        }
        let same = e.call(MATRIX_CONSTRUCT, &args![bound]).u32();
        let delta = e
            .call(VECTOR_MINUS_VECTOR, &args![target, difference, same])
            .u32();
        let distance = e.call(VECTOR_LENGTH, &args![delta]).f64();
        let radius = e.call(UPDATE_STEP_SETTING, &args![bound]).f64();
        let lod = (distance - radius * e.global::<f64>(FOLLOW_CATCH_UP_FACTOR)) as f32;
        let mut far = false;
        let owner = e.vcall(skeleton, SKELETON_VSLOT_0X10, &args![]).u32();
        if owner != 0 {
            let divisor = e.call(LOD_DIVISOR, &args![3u32]).f64();
            let quotient = (lod as f64 / divisor) as f32;
            let reference = e.call(SKELETON_RADIUS, &args![owner]).f64();
            far = reference < quotient as f64;
        }
        let accumulator = e.call(GET_ACCUMULATOR, &args![]).u32();
        if accumulator != 0 && e.call(ACCUMULATOR_TEST, &args![accumulator]).u32() != 0 {
            let value = if far { false } else { lod as f64 > 0.0 };
            let id = e.call(WORD_AT_0XC, &args![this]).u32();
            e.call(ACCUMULATOR_UPDATE, &args![accumulator, id, bound, value]);
        }
        if turret != 0 {
            fn_008879d0(e, Ptr::new(turret), far);
        }
    });
}

/// The death part of [`fn_00886cb0`]: with `collision` the actor's blend
/// collision object, stops the ragdoll animation, hands the penetration
/// utility the actor's shape, sets life state 1 and either knocks the
/// body down (an actor whose form test `+0x38c` fails) or plays the death
/// animation section, then clears `bDeadFlag`.
fn dead_ragdoll(e: &mut Engine, this: Ptr<Actor>, collision: u32) {
    let mut alive_form = true;
    let mut actor_form = 0u32;
    let form = e.call(SAVE_FORM_BUFFER_GET_FORM, &args![this]).u32();
    if e.call(FORM_TYPE, &args![form]).u32() == 0x2b {
        actor_form = this.addr();
    }
    if actor_form != 0
        && !e
            .vcall(actor_form, VSLOT_ACTOR_FORM_TEST_0X38C, &args![])
            .bool()
    {
        alive_form = false;
    }
    let ragdoll = e.get(this, Actor::pRagdollController);
    if !ragdoll.is_null() && e.call(RAGDOLL_TEST_0X552490, &args![ragdoll]).bool() {
        let ragdoll = e.get(this, Actor::pRagdollController);
        e.call(DISABLE_RAGDOLL_ANIM, &args![ragdoll, 1u32]);
    }
    let penetration = e.get(this, Actor::pPenetrationDetection);
    if !penetration.is_null() {
        let shape = e.with_stack(4, |e, slot| {
            let made = e.call(ACTOR_SHAPE_0X931ED0, &args![this, slot]).u32();
            e.call(SHAPE_RESULT_0X4A3A20, &args![made]).u32()
        });
        let penetration = e.get(this, Actor::pPenetrationDetection);
        e.call(PENETRATION_UPDATE, &args![penetration, collision, shape]);
    }
    e.call(ACTOR_SET_LIFE_STATE, &args![this, 1u32]);
    e.vcall(this.addr(), VSLOT_0X2A0, &args![]);
    if alive_form {
        e.with_stack(0x60, |e, base| {
            let matrix = base.addr();
            let direction = base.addr() + 0x24;
            let scratch = base.addr() + 0x30;
            e.call(MATRIX_CONSTRUCT, &args![matrix]);
            let heading = e
                .vcall(this.addr(), VSLOT_GET_HEADING_0X2BC, &args![0u32])
                .f32();
            e.call(MATRIX_MAKE_ROTATION, &args![matrix, heading]);
            e.call(NI_POINT3_SET, &args![direction, 0.0f32, 1.0f32, 0.0f32]);
            let rotated = e
                .call(MATRIX_TIMES_VECTOR, &args![matrix, scratch, direction])
                .u32();
            for i in 0..3 {
                let word = e.mem.u32(rotated + 4 * i);
                e.mem.set_u32(direction + 4 * i, word);
            }
            e.call(
                DO_KNOCK_DOWN,
                &args![collision, direction, 1u32, 0.0f32, 0u32],
            );
        });
        e.vcall(this.addr(), VSLOT_0X48, &args![4u32]);
        let process = e.get(this, Actor::pCurrentProcess);
        e.vcall(process.addr(), 0x28, &args![]);
    } else {
        let animation = e.vcall(this.addr(), VSLOT_GET_ANIMATION, &args![]).u32();
        if animation != 0
            && e.call(ANIMATION_GROUP_LOADED, &args![animation, 0xe0u32])
                .bool()
        {
            e.call(
                ANIMATION_FORCE_SECTION,
                &args![
                    animation,
                    0x14u32,
                    0xe0u32,
                    0xffff_ffffu32,
                    0.0f32,
                    0xffff_ffffu32
                ],
            );
            let entry = e.call(ANIMATION_LOOKUP, &args![animation, 1u32]).u32();
            if entry != 0 {
                let value = e.call(ENTRY_FLOAT_08100, &args![entry]).f32();
                e.call(ENTRY_SET_VALUE_98ADB0, &args![entry, value]);
                let value = e.call(ENTRY_FLOAT_08100, &args![entry]).f32();
                e.call(ANIMATION_UPDATE, &args![animation, this, 0.0f32, value]);
                e.call(ANIMATION_UPDATE_MOVEMENT, &args![animation, this]);
            }
            e.call(ACTOR_AFTER_DEATH_ANIMATION, &args![this]);
        }
    }
    e.set(this, Actor::bDeadFlag, false);
}

/// One pass of the collision re-sync of [`fn_00886cb0`]: finds the
/// collision object of `skeleton` for `key` (`004ade00`), adds its rigid
/// body to the world when it is not there, rotates a matching entry of
/// the body's list (`second` selects the variant that clamps the float at
/// `this + 0x24` to the limits of the object `00887a10` finds for the
/// node's `+0x44` handle, then hands the flag (`00887a80`) and the clamped
/// float (`00887ac0`) to that handle) and synchronises. Returns true when
/// the rigid body was added.
fn resync_collision(
    e: &mut Engine,
    this: Ptr<Actor>,
    skeleton: u32,
    key: u32,
    second: bool,
) -> bool {
    let collision = e.call(FIND_COLLISION_OBJECT, &args![skeleton, key]).u32();
    if collision == 0 {
        return false;
    }
    e.call(COLLISION_FUNCTION_0X6838B0, &args![collision]);
    let name = e.call(COLLISION_FUNCTION_0X6838B0, &args![collision]).u32();
    let entry = e.call(FIND_ENTRY, &args![ENTRY_TABLE_A, name]).u32();
    if entry == 0 {
        return false;
    }
    let mut added = false;
    let body = e.call(COLLISION_RIGID_BODY, &args![entry]).u32();
    let mut list = e.call(BODY_LIST_0X7D6BB0, &args![body]).u32();
    let shape = e.call(BODY_SHAPE_0X517630, &args![body]).u32();
    let in_world = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), shape);
        e.call(WORLD_TEST_0XC8CCE0, &args![slot]).bool()
    });
    if !in_world {
        e.call(SET_MOTION, &args![collision, 1u32, 1u32, 1u32, 1u32]);
        let word = e.vcall(body, BODY_VSLOT_0X94, &args![]).u32();
        let made = e.call(BODY_FUNCTION_0X621480, &args![word]).u32();
        e.call(ADD_ACT_CON_TO_WORLD, &args![body, made]);
        added = true;
    }
    let kind = e.with_stack(4, |e, slot| {
        e.call(BODY_FLAGS_0X43B4F0, &args![body, slot]);
        e.call(BODY_KIND_0X43B4D0, &args![slot]).u32()
    });
    if kind == 0x1d {
        e.call(COLLISION_SET_0XC804D0, &args![skeleton, 8u32]);
    }
    e.call(BODY_ACTIVATE_0X561580, &args![body, 1u32]);
    while list != 0 {
        let same = e.call(MATRIX_CONSTRUCT, &args![list]).u32();
        if e.call(WORD_AT_START, &args![same]).u32() == 0 {
            break;
        }
        let same = e.call(MATRIX_CONSTRUCT, &args![list]).u32();
        let node = e.call(WORD_AT_START, &args![same]).u32();
        if !second {
            let found = e.call(FIND_ENTRY, &args![ENTRY_TABLE_B, node]).u32();
            if found != 0 {
                let heading = e
                    .vcall(this.addr(), VSLOT_GET_HEADING_0X2BC, &args![0u32])
                    .f32();
                let first = e
                    .call(ANGLE_0X5B9E80, &args![heading, 0.0f32, 0.0f32])
                    .f32();
                let second_value = e.call(ANGLE_0X5C53D0, &args![heading, first]).f32();
                e.with_stack(0x10, |e, out| {
                    e.call(
                        QUATERNION_SET,
                        &args![out, second_value, first, 0.0f32, 0.0f32],
                    );
                    fn_00887b40(e, Ptr::new(found), 1);
                    fn_00887af0(e, Ptr::new(found), out.addr());
                });
                break;
            }
        } else {
            let handle = e.vcall(node, NODE_VSLOT_0X44, &args![]).u32();
            if handle != 0 {
                let target = fn_00887a10(e, Ptr::new(handle));
                let value = e.mem.f32(this.addr() + 0x24);
                let upper = e.call(HANDLE_FLOAT_0X9A1260, &args![target]).f32();
                let lower = e.call(HANDLE_FLOAT_0X644A50, &args![target]).f32();
                let clamped = e.with_stack(4, |e, slot| {
                    e.mem.set_f32(slot.addr(), value);
                    e.call(CLAMP_FLOAT, &args![slot, lower, upper]);
                    e.mem.f32(slot.addr())
                });
                // 0088791d / 00887932 MOV ECX,[EBP-0x1a4]: both callees get
                // the handle (they run `00887a10` on it themselves), not the
                // `00887a10` result kept at [EBP-0x1a8].
                fn_00887a80(e, Ptr::new(handle), true);
                fn_00887ac0(e, Ptr::new(handle), clamped);
                break;
            }
        }
        list = e.call(NEXT_LIST_NODE, &args![list]).u32();
    }
    if e.call(RESET_ANIMS_SHOULD_QUEUE, &args![]).bool() {
        let queue = e.call(TASK_QUEUE_INTERFACE, &args![]).u32();
        e.call(QUEUE_COLLISION_SYNC, &args![queue, this, collision]);
    } else {
        e.call(COLLISION_SYNCHRONIZE, &args![collision, 2u32]);
    }
    added
}

// Translated from 00888070 (decompiled, FalloutNV.exe 1.4.0.525)
/// The per-frame animation step of an actor with a 3D object: advances the
/// process's high-level animation state, the idle-wait counter of the
/// process, the dying animation (`bDeadFlag` is cleared once it is
/// played), the process virtual `+0x740` update and
/// `UpdateAnimationMovement`, then `Actor::PickAnimations(1.0, speed)`
/// with one of four settings. `delta` is the frame time passed on; the
/// meanings of the process virtuals `+0x740` to `+0x750` are not
/// confirmed.
pub fn fn_00888070(e: &mut Engine, this: Ptr<Actor>, delta: f32) {
    e.call(EMPTY_FUNCTION, &args![SCOPE_OBJECT_0X11DF69C]);
    let skeleton = e.vcall(this.addr(), VSLOT_GET_3D, &args![]).u32();
    if skeleton != 0 {
        let skeleton = e.vcall(this.addr(), VSLOT_GET_3D, &args![]).u32();
        if e.call(PACKAGE_TYPE, &args![skeleton]).u32() != 0 {
            animation_step(e, this, delta);
        }
    }
    e.call(EMPTY_FUNCTION, &args![SCOPE_OBJECT_0X11DF69C]);
}

/// The body of `00888070` once the actor has a 3D object whose `009611e0`
/// answer is non-zero.
fn animation_step(e: &mut Engine, this: Ptr<Actor>, delta: f32) {
    e.call(ACTOR_PRE_ANIMATION_STEP, &args![this]);
    let process = e.get(this, Actor::pCurrentProcess);
    e.vcall(process.addr(), PROCESS_VSLOT_0X41C, &args![this]);
    let animation = e.vcall(this.addr(), VSLOT_GET_ANIMATION, &args![]).u32();
    if e.vcall(this.addr(), VSLOT_ANIMATION_FLAG_0X21C, &args![])
        .bool()
        && animation != 0
        && e.call(ANIMATION_GROUP_LOADED, &args![animation, 0xe1u32])
            .bool()
        && e.call(GET_PROCESS, &args![this]).u32() != 0
    {
        let process = e.call(GET_PROCESS, &args![this]).u32();
        if e.vcall(process, PROCESS_VSLOT_0X748, &args![]).u32() != 0 {
            let mut count = 0u32;
            let process = e.call(GET_PROCESS, &args![this]).u32();
            let have = e.vcall(process, PROCESS_VSLOT_0X750, &args![]).u32();
            let wanted = animation_time_in_milliseconds(e, animation);
            if wanted as u32 > have {
                let process = e.call(GET_PROCESS, &args![this]).u32();
                let have = e.vcall(process, PROCESS_VSLOT_0X750, &args![]).u32();
                let wanted = animation_time_in_milliseconds(e, animation);
                count = (wanted as u32).wrapping_sub(have) / 0x21;
            }
            let process = e.call(GET_PROCESS, &args![this]).u32();
            let list = e.vcall(process, PROCESS_VSLOT_0X748, &args![]).u32();
            let fraction = e.call(LIST_FRACTION_AT, &args![list, count]).f32();
            // 00888229..00888230: PUSH ECX; FSTP [ESP]; CALL 0049bdc0 (`RET 4`): the fraction
            // is the one word; `count` (PUSH EDX at 00888211) was 004d50e0's.
            e.call(ANIMATION_SET_FRACTION, &args![animation, fraction]);
            let process = e.call(GET_PROCESS, &args![this]).u32();
            if e.vcall(process, PROCESS_VSLOT_0X748, &args![]).u32() != 0 {
                let process = e.call(GET_PROCESS, &args![this]).u32();
                let list = e.vcall(process, PROCESS_VSLOT_0X748, &args![]).u32();
                if count > e.call(LIST_COUNT, &args![list]).u32() {
                    let process = e.call(GET_PROCESS, &args![this]).u32();
                    e.vcall(process, PROCESS_VSLOT_0X744, &args![0u32]);
                    let process = e.call(GET_PROCESS, &args![this]).u32();
                    e.vcall(process, PROCESS_VSLOT_0X74C, &args![0u32]);
                }
            }
        }
    }
    if e.get(this, Actor::bDeadFlag) && e.call(GET_PROCESS, &args![this]).u32() != 0 {
        let collision = e.call(BLEND_OBJECT_OF, &args![this]).u32();
        if collision != 0 {
            let mut alive_form = true;
            let mut actor_form = 0u32;
            let form = e.call(SAVE_FORM_BUFFER_GET_FORM, &args![this]).u32();
            if e.call(FORM_TYPE, &args![form]).u32() == 0x2b {
                actor_form = this.addr();
            }
            if actor_form != 0
                && !e
                    .vcall(actor_form, VSLOT_ACTOR_FORM_TEST_0X38C, &args![])
                    .bool()
            {
                alive_form = false;
            }
            if !alive_form
                && animation != 0
                && e.call(ANIMATION_GROUP_LOADED, &args![animation, 0xe0u32])
                    .bool()
            {
                e.call(ACTOR_SET_LIFE_STATE, &args![this, 1u32]);
                e.vcall(this.addr(), VSLOT_0X2A0, &args![]);
                e.call(
                    ANIMATION_FORCE_SECTION,
                    &args![
                        animation,
                        0x14u32,
                        0xe0u32,
                        0xffff_ffffu32,
                        0.0f32,
                        0xffff_ffffu32
                    ],
                );
                let entry = e.call(ANIMATION_LOOKUP, &args![animation, 1u32]).u32();
                if entry != 0 {
                    let value = e.call(ENTRY_FLOAT_08100, &args![entry]).f32();
                    e.call(ENTRY_SET_VALUE_98ADB0, &args![entry, value]);
                    let value = e.call(ENTRY_FLOAT_08100, &args![entry]).f32();
                    e.call(ANIMATION_UPDATE, &args![animation, this, 0.0f32, value]);
                    e.call(ANIMATION_UPDATE_MOVEMENT, &args![animation, this]);
                }
                e.call(ACTOR_AFTER_DEATH_ANIMATION, &args![this]);
                e.set(this, Actor::bDeadFlag, false);
            }
        }
    }
    let process = e.get(this, Actor::pCurrentProcess);
    if !process.is_null() {
        let step = e
            .call(UPDATE_STEP_SETTING, &args![UPDATE_STEP_OBJECT])
            .f32();
        if e.call(SETTING_INT_VALUE, &args![VATS_STATE_OBJECT]).u32() == 4
            && this.addr() == e.mem.u32(VATS_TARGET_ACTOR)
        {
            let step = e
                .call(UPDATE_STEP_SETTING, &args![UPDATE_STEP_OBJECT])
                .f64();
            let multiplier = e
                .call(VATS_TARGET_UPDATE_MULT, &args![VATS_STATE_OBJECT])
                .f64();
            let scaled = (multiplier * step) as f32;
            e.vcall(process.addr(), PROCESS_VSLOT_0X740, &args![this, scaled]);
        } else {
            e.vcall(process.addr(), PROCESS_VSLOT_0X740, &args![this, step]);
        }
    }
    if !e
        .vcall(this.addr(), VSLOT_IS_PROCESS_FLAG_0X22C, &args![0u32])
        .bool()
    {
        let process = e.call(GET_PROCESS, &args![this]).u32();
        if e.vcall(process, PROCESS_VSLOT_FLAG, &args![]).bool()
            && animation != 0
            && e.call(ANIMATION_LOOKUP, &args![animation, 4u32]).u32() == 0
        {
            e.call(
                ACTOR_QUEUE_ANIMATION_0X8B28C0,
                &args![this, 0x11u32, animation],
            );
        }
    }
    actor_update_animation_movement(e, this, Ptr::new(animation), delta);
    let flag_0x21c = e
        .vcall(this.addr(), VSLOT_ANIMATION_FLAG_0X21C, &args![])
        .bool();
    let flag_0x493bb0 = e.call(ACTOR_GET_IN_COMBAT, &args![this]).bool();
    let setting = match (flag_0x21c, flag_0x493bb0) {
        (false, true) => SPEED_SETTING_21C_CLEAR_TRUE,
        (false, false) => SPEED_SETTING_21C_CLEAR_FALSE,
        (true, true) => SPEED_SETTING_21C_SET_TRUE,
        (true, false) => SPEED_SETTING_21C_SET_FALSE,
    };
    let speed = setting_value(e, setting);
    e.call(PICK_ANIMATIONS, &args![this, 1.0f32, speed]);
}

/// `Animation::fTime`-style value `453700` (a `float` at +0xd0 of the
/// animation) times 1000, truncated toward zero to 64 bits (the code
/// switches the x87 rounding mode to truncate), as a `u64`.
fn animation_time_in_milliseconds(e: &mut Engine, animation: u32) -> u64 {
    let seconds = e.call(ANIMATION_TIME, &args![animation]).f64();
    let scaled = seconds * e.global::<f64>(MILLISECONDS_PER_SECOND);
    (scaled as i64) as u64
}

// Translated from 008885a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `Animation::UpdateMovementNoWorldUpdate(this)` on the actor's
/// animation (virtual `+0x1e4`), if there is one.
pub fn fn_008885a0(e: &mut Engine, this: Ptr<Actor>) {
    let animation = e.vcall(this.addr(), VSLOT_GET_ANIMATION, &args![]).u32();
    if animation != 0 {
        e.call(
            ANIMATION_UPDATE_MOVEMENT_NO_WORLD_UPDATE,
            &args![animation, this],
        );
    }
}

// Translated from 008885e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::UpdateAnimationMovement` (Xbox PDB): moves the actor by its
/// animation's movement, `animation` being the actor's animation (`this`
/// does nothing without one). The player is counted in a global nesting
/// byte (`011e07a8`) while it runs. The second stack word (the frame time
/// `00888070` passes) is not read.
pub fn actor_update_animation_movement(
    e: &mut Engine,
    this: Ptr<Actor>,
    animation: Ptr,
    _unused_2: f32,
) {
    let player = e.mem.u32(PLAYER_CHARACTER);
    e.call(EMPTY_FUNCTION, &args![SCOPE_OBJECT_0X11DF71D]);
    if this.addr() == player {
        let count = e.mem.u8(PLAYER_UPDATE_NESTING);
        e.mem.set_u8(PLAYER_UPDATE_NESTING, count.wrapping_add(1));
    }
    if !animation.is_null() {
        let is_free_player =
            this.addr() == player && !e.call(PLAYER_STATE_TEST, &args![player]).bool();
        if this.addr() == player || mode_setting_is_one(e) {
            e.call(
                ANIMATION_UPDATE_MOVEMENT_NO_WORLD_UPDATE,
                &args![animation, this],
            );
        }
        let blocked = e.call(ACTOR_BLOCKED_TEST, &args![this]).bool()
            || e.vcall(this.addr(), VSLOT_IS_PROCESS_FLAG_0X22C, &args![0u32])
                .bool()
            || e.vcall(this.addr(), VSLOT_FLAG_0X2E8, &args![]).bool()
            || e.vcall(this.addr(), VSLOT_PACKAGE_TEST_0X230, &args![])
                .bool();
        let updates =
            !blocked && !e.call(REFERENCE_TEST_0X579670, &args![this]).bool() && !is_free_player;
        let mut step = e
            .call(UPDATE_STEP_SETTING, &args![UPDATE_STEP_OBJECT])
            .f32();
        step = vats_scaled(e, this, step);
        if updates {
            e.vcall(this.addr(), VSLOT_UPDATE_0X4C8, &args![step]);
            e.vcall(this.addr(), VSLOT_UPDATE_0X4CC, &args![step]);
            e.vcall(this.addr(), VSLOT_UPDATE_0X494, &args![]);
        }
        if this.addr() == player || mode_setting_is_one(e) {
            let saved = e.call(PICK_COUNT_OVERRIDE_BYTE, &args![]).u8();
            if this.addr() == player {
                e.call(SET_PICK_COUNT_OVERRIDE, &args![1u32]);
            }
            if !is_free_player
                && e.call(ACTOR_MOVEMENT_STEP, &args![this, updates, step])
                    .bool()
            {
                e.call(ACTOR_RAGDOLL_FACE_STEP, &args![this, 1u32]);
            }
            if this.addr() == player {
                e.call(SET_PICK_COUNT_OVERRIDE, &args![saved as u32]);
                e.call(CONSOLIDATE_SIM_ISLANDS, &args![this]);
            }
        } else {
            e.call(ACTOR_MOVEMENT_STEP, &args![this, updates, step]);
        }
        e.call(
            ANIMATION_UPDATE_SCENE_GRAPH_NO_CONTROLLER,
            &args![animation],
        );
        let kind = e.vcall(this.addr(), VSLOT_KIND_0X214, &args![]).u32();
        let index = kind.wrapping_sub(3);
        // The compiler's switch on the kind less 3 (`0..=7`): kinds 3, 5, 8 and 10 rescale the
        // actor; the others leave.
        if matches!(index, 0 | 2 | 5 | 7) {
            let player_animation = e.call(PLAYER_GET_ANIMATION, &args![player, 1u32]).u32();
            if animation.addr() != player_animation {
                let mut vector = [
                    e.mem.u32(SCALE_VECTOR),
                    e.mem.u32(SCALE_VECTOR + 4),
                    e.mem.u32(SCALE_VECTOR + 8),
                ];
                let scale = e.call(GET_SCALE, &args![this]).f32();
                if scale != 1.0 {
                    // 008888b6: `0050d4a0` takes no stack word; the pushed 0 is
                    // the last word of `00494390` (RET 0x10).
                    let flag = e.call(REFERENCE_FLAG_0X50D4A0, &args![this]).u8();
                    e.with_stack(12, |e, slot| {
                        for (i, word) in vector.iter().enumerate() {
                            e.mem.set_u32(slot.addr() + 4 * i as u32, *word);
                        }
                        e.call(
                            ANIMATION_SCALE_VECTOR,
                            &args![animation, slot, this, flag as u32, 0u32],
                        );
                        for (i, word) in vector.iter_mut().enumerate() {
                            *word = e.mem.u32(slot.addr() + 4 * i as u32);
                        }
                    });
                    let z = f32::from_bits(vector[2]);
                    if z != 0.0 {
                        let offset = (z as f64 - z as f64 * scale as f64) as f32;
                        let position = e
                            .vcall(this.addr(), VSLOT_GET_POSITION_0X1F4, &args![])
                            .u32();
                        let height = e.mem.f32(position + 8);
                        let new_height = (height as f64 - offset as f64) as f32;
                        e.call(SET_LOCATION_ON_REFERENCE_Z, &args![this, new_height]);
                    }
                }
            }
        }
    }
    if this.addr() == player {
        let count = e.mem.u8(PLAYER_UPDATE_NESTING);
        e.mem.set_u8(PLAYER_UPDATE_NESTING, count.wrapping_sub(1));
    }
    e.call(EMPTY_FUNCTION, &args![SCOPE_OBJECT_0X11DF71D]);
}

// Translated from 00888960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the global byte at `012677a3`.
pub fn fn_00888960(e: &mut Engine) -> u8 {
    e.mem.u8(GLOBAL_FLAG_012677A3)
}

// Translated from 00888970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs the ragdoll controller's ragdoll animation (`00c7d630(1)`), then
/// copies the controller's two floats at `+0x19c`/`+0x1a0` (or zeros, when
/// the byte at `+0xb3` is clear) into the face animation data's `+0x150`
/// and `+0x154`, and resets the controller's `+0x23c` when `004955a0` says
/// so. The stack word is not read.
pub fn fn_00888970(e: &mut Engine, this: Ptr<Actor>, _unused_1: u32) {
    let controller = e.get(this, Actor::pRagdollController);
    e.call(DO_RAGDOLL_ANIM, &args![controller, 1u32]);
    let face = e.call(GET_FACE_ANIMATION_DATA, &args![this]).u32();
    if face != 0 {
        if fn_00888a50(e, controller) {
            let (first, second) = e.with_stack(8, |e, slot| {
                let first = slot.addr();
                let second = slot.addr() + 4;
                fn_00888a70(e, controller, Ptr::new(first), Ptr::new(second));
                (e.mem.f32(first), e.mem.f32(second))
            });
            fn_00888a20(e, Ptr::new(face), first, second);
        } else {
            fn_00888a20(e, Ptr::new(face), 0.0, 0.0);
        }
    }
    if !e.call(RAGDOLL_TEST_0X4955A0, &args![controller]).bool() {
        fn_00888aa0(e, controller);
    }
}

// Translated from 00888a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores two floats at `+0x150` and `+0x154`.
pub fn fn_00888a20(e: &mut Engine, this: Ptr, first: f32, second: f32) {
    e.mem.set_f32(this.addr() + 0x150, x87_quiet(first));
    e.mem.set_f32(this.addr() + 0x154, x87_quiet(second));
}

// Translated from 00888a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the byte at `+0xb3` of the ragdoll controller.
pub fn fn_00888a50(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0xb3) != 0
}

// Translated from 00888a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the floats at `+0x19c` and `+0x1a0` of the controller to the two
/// pointers.
pub fn fn_00888a70(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr) {
    let a = e.mem.f32(this.addr() + 0x19c);
    e.mem.set_f32(first.addr(), a);
    let b = e.mem.f32(this.addr() + 0x1a0);
    e.mem.set_f32(second.addr(), b);
}

// Translated from 00888aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the `float` at `+0x23c` of the ragdoll controller.
pub fn fn_00888aa0(e: &mut Engine, this: Ptr) {
    e.mem.set_f32(this.addr() + 0x23c, 0.0);
}

// Translated from 00888ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Process virtual `+0x2f0` with the actor and `value`.
pub fn fn_00888ac0(e: &mut Engine, this: Ptr<Actor>, value: f32) {
    let process = e.get(this, Actor::pCurrentProcess);
    e.vcall(process.addr(), PROCESS_VSLOT_0X2F0, &args![this, value]);
}

// Translated from 00888af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Process virtual `+0x2f4`, whose `ST0` it answers unchanged (the exe ends on that call). The
/// function is slot `+0x2b4` of `Actor`'s vtable, the `MobileObject` slot the Xbox PDB names
/// `GetDetectionTimer` (`float`; Xbox slot `+0x2b0`, the `-4` the bounded `Actor` slots show),
/// and `MobileObject`'s own implementation (`0087d880`) answers `FLDZ`.
pub fn fn_00888af0(e: &mut Engine, this: Ptr<Actor>) -> f64 {
    let process = e.get(this, Actor::pCurrentProcess);
    e.vcall(process.addr(), PROCESS_VSLOT_0X2F4, &args![]).f64()
}

// Translated from 00888b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Process virtual `+0x2f8`.
pub fn fn_00888b20(e: &mut Engine, this: Ptr<Actor>) {
    let process = e.get(this, Actor::pCurrentProcess);
    e.vcall(process.addr(), PROCESS_VSLOT_0X2F8, &args![]);
}

/// The locals of `Actor::Update` (`00888b50`) that its sections share.
struct UpdateFrame {
    this: Ptr<Actor>,
    delta: f32,
    player: u32,
    /// The actor's 3D object (virtual `+0x1d0`).
    skeleton: u32,
    /// `Actor::GetFaceAnimationData` (`008adcb0`).
    face: u32,
    /// The actor's animation (virtual `+0x1e4`).
    animation: u32,
    /// `Actor::bInCombat` as `00493bb0` reads it (cleared for the player).
    in_combat: bool,
    /// `Actor::GetCurrentWeapon` (`008a1710`).
    weapon: u32,
    /// The process read at the start (`Actor::pCurrentProcess`).
    process: u32,
    /// The `BSSoundHandle` on the stack.
    sound: u32,
    /// The character controller (`MobileObject::GetCharController`).
    controller: u32,
    /// True for the player when `004eaf60` says so.
    player_flag: bool,
    /// The actor's parent cell.
    cell: u32,
}

// Translated from 00888b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::Update` (Xbox PDB): the actor's per-frame update with the frame
/// time `delta`: caches the sit/sleep state, scales the animation speed,
/// positions the actor's head target, updates the face data, the process
/// state (life state 6 healing), the weapon attack sound, the movement
/// mode, the 3D position, swimming and wading (the water height, breath,
/// drowning damage, splash effects and the controller's mode), the
/// creature sounds and the animation block flags. It does nothing
/// (apart from the cleanup) when the actor has no 3D object or cell, or
/// the cell fails `00450ff0`, and only wakes the process (virtual `+0x5f8`)
/// when `delta >= 900`. The exception frame is not translated. The
/// meaning of most callees and virtual slots is not confirmed; they are
/// described by what they take and return.
pub fn actor_update(e: &mut Engine, this: Ptr<Actor>, delta: f32) {
    e.call(EMPTY_FUNCTION, &args![SCOPE_OBJECT_UPDATE]);
    e.call(ACTOR_RESET_FLOATS, &args![this]);
    let process = e.call(GET_PROCESS, &args![this]).u32();
    if process != 0 {
        let state = e.vcall(process, PROCESS_VSLOT_0X4BC, &args![]).u32();
        e.set(this, Actor::cCurrentSitSleepState, state);
    }
    let skeleton = e.vcall(this.addr(), VSLOT_GET_3D, &args![]).u32();
    let face = e.call(GET_FACE_ANIMATION_DATA, &args![this]).u32();
    let animation = e.vcall(this.addr(), VSLOT_GET_ANIMATION, &args![]).u32();
    let in_combat = e.call(ACTOR_GET_IN_COMBAT, &args![this]).bool();
    let weapon = e.call(ACTOR_GET_CURRENT_WEAPON, &args![this]).u32();
    e.with_stack(0x10, |e, sound| {
        e.call(SOUND_HANDLE_INIT, &args![sound]);
        let cell = e.call(GET_PARENT_CELL, &args![this]).u32();
        let mut frame = UpdateFrame {
            this,
            delta,
            player: e.mem.u32(PLAYER_CHARACTER),
            skeleton,
            face,
            animation,
            in_combat,
            weapon,
            process,
            sound: sound.addr(),
            controller: 0,
            player_flag: false,
            cell,
        };
        if skeleton != 0 && cell != 0 && e.call(CELL_TEST_0X450FF0, &args![cell]).bool() {
            let limit = e.global::<f64>(UPDATE_TIME_LIMIT);
            if (delta as f64) >= limit {
                if process != 0 {
                    e.vcall(process, PROCESS_VSLOT_0X5F8, &args![1u32]);
                }
            } else {
                update_main(e, &mut frame);
            }
        }
        e.call(EMPTY_FUNCTION, &args![SCOPE_OBJECT_UPDATE]);
        e.call(EMPTY_FUNCTION, &args![sound.addr()]);
    });
}

/// The body of `Actor::Update` once the actor has a 3D object and a cell.
fn update_main(e: &mut Engine, f: &mut UpdateFrame) {
    let this = f.this;
    e.vcall(this.addr(), VSLOT_UPDATE_0X44C, &args![]);
    update_animation_speed(e, f);
    f.player_flag = this.addr() == f.player && !e.call(PLAYER_STATE_TEST, &args![f.player]).bool();
    f.controller = e.call(GET_CHAR_CONTROLLER, &args![this]).u32();
    if f.controller != 0 {
        update_controller_target(e, f);
    }
    update_face_and_life(e, f);
    update_weapon_sound(e, f);
    update_movement_mode(e, f);
    update_position(e, f);
    if !f.player_flag {
        update_water_and_sounds(e, f);
    }
    // 0088afcc
    e.call(ACTOR_TIMED_UPDATE, &args![this, f.delta]);
    e.call(ACTOR_END_OF_UPDATE, &args![this]);
}

/// `0088b030` on the actor's animation: `1 / [011d0fa8]` for a non-player
/// actor whose actor value `0x33` is positive, else `1.0`.
fn update_animation_speed(e: &mut Engine, f: &UpdateFrame) {
    let this = f.this;
    if this.addr() == f.player {
        return;
    }
    let owner = this.addr() + ACTOR_VALUE_OWNER_OFFSET;
    let value = e.vcall(owner, 8, &args![0x33u32]).i32();
    if value > 0 && e.vcall(this.addr(), VSLOT_GET_ANIMATION, &args![]).u32() != 0 {
        let setting = setting_value(e, ANIMATION_SPEED_SETTING);
        let scale = (1.0f64 / setting as f64) as f32;
        let animation = e.vcall(this.addr(), VSLOT_GET_ANIMATION, &args![]).u32();
        e.call(ANIMATION_SET_SPEED_SCALE, &args![animation, scale]);
        return;
    }
    if e.vcall(this.addr(), VSLOT_GET_ANIMATION, &args![]).u32() != 0 {
        let animation = e.vcall(this.addr(), VSLOT_GET_ANIMATION, &args![]).u32();
        e.call(ANIMATION_SET_SPEED_SCALE, &args![animation, 1.0f32]);
    }
}

/// The character controller part of `Actor::Update`: gives the controller
/// the point the actor's head looks at (for a non-player actor) and marks
/// whether the current attack is a power attack with a type below `0xa8`.
fn update_controller_target(e: &mut Engine, f: &UpdateFrame) {
    let this = f.this;
    let player = f.player;
    let controller = f.controller;
    if this.addr() != player {
        e.with_stack(0x24, |e, base| {
            let target = base.addr();
            let scaled = base.addr() + 0xc;
            let sum = base.addr() + 0x18;
            e.call(MATRIX_CONSTRUCT, &args![target]);
            let source = if e.call(PLAYER_STATE_TEST, &args![player]).bool()
                && e.mem.u32(HEAD_TARGET_A) != 0
            {
                let object = e.mem.u32(HEAD_TARGET_A);
                Some(e.call(OBJECT_POSITION_0X8C, &args![object]).u32())
            } else if e.mem.u32(HEAD_TARGET_B) != 0 {
                let object = e.mem.u32(HEAD_TARGET_B);
                Some(e.call(OBJECT_POSITION_0X8C, &args![object]).u32())
            } else {
                None
            };
            let source = match source {
                Some(address) => address,
                None => {
                    let eye = e.call(GET_EYE_LEVEL, &args![player]).f32();
                    let scaled_vector = e
                        .call(VECTOR_TIMES_SCALAR, &args![scaled, eye, EYE_DIRECTION])
                        .u32();
                    let position = e.call(POSITION_VECTOR_OF, &args![player]).u32();
                    e.call(VECTOR_PLUS_VECTOR, &args![position, sum, scaled_vector])
                        .u32()
                }
            };
            for offset in [0, 4, 8] {
                let word = e.mem.u32(source + offset);
                e.mem.set_u32(target + offset, word);
            }
            e.call(
                CONTROLLER_SET_TARGET,
                &args![controller, target, f.skeleton],
            );
        });
    }
    let mut power_attack = false;
    if f.animation != 0 {
        let group = e
            .call(ANIMATION_GROUP_FIELD, &args![f.animation, 4u32])
            .u16();
        if e.call(IS_POWER_ATTACK_ACTION, &args![group as u32]).bool()
            && e.call(ANIM_GROUP_GET_TYPE, &args![group as u32]).i32() < 0xa8
        {
            power_attack = true;
        } else {
            let group = e
                .call(ANIMATION_GROUP_FIELD, &args![f.animation, 2u32])
                .u16();
            if e.call(IS_POWER_ATTACK_ACTION, &args![group as u32]).bool()
                && e.call(ANIM_GROUP_GET_TYPE, &args![group as u32]).i32() < 0xa8
            {
                power_attack = true;
            }
        }
    }
    let controller_now = e.call(GET_CHAR_CONTROLLER, &args![this]).u32();
    let target = if controller_now != 0 {
        controller_now + 0x410
    } else {
        0
    };
    e.with_stack(4, |e, flag| {
        e.mem.set_u8(flag.addr(), power_attack as u8);
        fn_0088b070(e, Ptr::new(target), flag);
    });
}

/// The process and face part of `Actor::Update`: the face animation data's
/// two virtuals (`+0xd4`, `+0xd8`), the process virtual `+0x704`, and, in
/// life state 6, healing the actor and restoring its health once the
/// process says it is rested.
fn update_face_and_life(e: &mut Engine, f: &UpdateFrame) {
    let this = f.this;
    let face = f.face;
    let process = e.get(this, Actor::pCurrentProcess);
    if !process.is_null() {
        e.vcall(process.addr(), PROCESS_VSLOT_0X704, &args![this]);
        if e.get(this, Actor::cCurrentSitSleepState) == 9
            || e.call(ACTOR_BLOCKED_TEST, &args![this]).bool()
        {
            if face != 0 && !e.vcall(face, FACE_VSLOT_0XD4, &args![]).bool() {
                e.vcall(face, FACE_VSLOT_0XD8, &args![1u32, 0u32]);
            }
        } else if face != 0
            && !e
                .vcall(this.addr(), VSLOT_IS_PROCESS_FLAG_0X22C, &args![0u32])
                .bool()
            && e.vcall(face, FACE_VSLOT_0XD4, &args![]).bool()
        {
            e.vcall(face, FACE_VSLOT_0XD8, &args![0u32, 0u32]);
        }
        if e.call(ACTOR_GET_LIFE_STATE, &args![this]).u32() == 6 {
            let level = e.vcall(process.addr(), PROCESS_VSLOT_0X40C, &args![]).u32();
            let level_ok =
                level == 3 || e.vcall(process.addr(), PROCESS_VSLOT_0X40C, &args![]).u32() == 1;
            if level_ok {
                if e.call(GET_FORCE_NEXT_UPDATE, &args![this]).bool() {
                    let player = e.mem.u32(PLAYER_CHARACTER);
                    if player == 0 || !e.call(PLAYER_IN_COMBAT, &args![player, 0u32]).bool() {
                        e.vcall(process.addr(), PROCESS_VSLOT_0XE0, &args![]);
                    }
                    let health = e.vcall(process.addr(), PROCESS_VSLOT_0XE4, &args![]).f64();
                    if health <= 0.0 {
                        e.call(ACTOR_SET_LIFE_STATE, &args![this, 0u32]);
                        e.call(ACTOR_RESTORE_FULL_HEALTH, &args![this]);
                        e.call(TRIGGER_FOLLOWER_BARK, &args![this, 0xbu32]);
                    }
                } else {
                    e.vcall(process.addr(), PROCESS_VSLOT_0XE0, &args![]);
                    let health = e.vcall(process.addr(), PROCESS_VSLOT_0XE4, &args![]).f64();
                    if health <= 0.0 {
                        e.call(ACTOR_SET_LIFE_STATE, &args![this, 0u32]);
                        e.call(ACTOR_RESTORE_FULL_HEALTH, &args![this]);
                    }
                }
            }
        }
    }
}

/// The weapon attack sound part of `Actor::Update`: an actor that is not
/// in the state `vcall(+0x22c)` tests and has an animation and a weapon
/// keeps the attack sound of its extra data in step with the attack
/// animation: stops it when the attack ends, or starts it for a looping
/// attack animation.
fn update_weapon_sound(e: &mut Engine, f: &UpdateFrame) {
    let this = f.this;
    let animation = f.animation;
    let weapon = f.weapon;
    let sound = f.sound;
    if e.vcall(this.addr(), VSLOT_IS_PROCESS_FLAG_0X22C, &args![0u32])
        .bool()
        || animation == 0
        || weapon == 0
    {
        return;
    }
    let weapon_object = e.call(WEAPON_OBJECT_OF, &args![weapon]).u32();
    if weapon_object == 0 {
        return;
    }
    let weapon_object = e.call(WEAPON_OBJECT_OF, &args![weapon]).u32();
    if e.call(WEAPON_OBJECT_TEST, &args![weapon_object]).u32() == 0 {
        return;
    }
    if e.call(GET_LAST_BOUND_WEAPON, &args![weapon]).u32() == 0x2d
        && e.call(WEAPON_TYPE_0X51F5F0, &args![weapon]).u32() != 0x50
    {
        return;
    }
    let in_range = e.call(ACTOR_RANGE_TEST, &args![this]).bool();
    let attack_state = if !in_range
        || e.vcall(this.addr(), VSLOT_PACKAGE_TEST_0X230, &args![])
            .bool()
    {
        true
    } else if !e.call(ACTOR_RANGE_TEST, &args![this]).bool() {
        false
    } else {
        e.call(ANIMATION_FIELD_0X70F490, &args![animation, 4u32])
            .i32()
            > 1
    };
    if attack_state {
        let lists = e.call(GET_EXTRA_DATA_LIST_ADDRESS, &args![this]).u32();
        e.call(EXTRA_GET_WEAPON_ATTACK_SOUND, &args![lists, sound]);
        if e.call(SOUND_HANDLE_IS_PLAYING, &args![sound]).bool() {
            e.call(SOUND_HANDLE_STOP, &args![sound]);
            if e.call(WEAPON_TEST_0X524B40, &args![weapon]).bool() {
                let entry = e.call(ANIMATION_LOOKUP, &args![animation, 4u32]).u32();
                if entry != 0 {
                    let entry = e.call(ANIMATION_LOOKUP, &args![animation, 4u32]).u32();
                    let step = e.call(ANIMATION_STEP, &args![entry]).u32();
                    if e.call(ANIMATION_ENTRY_TEST_0X4937E0, &args![step]).bool() {
                        let entry = e.call(ANIMATION_LOOKUP, &args![animation, 4u32]).u32();
                        if e.call(ANIMATION_ENTRY_KIND, &args![entry]).u32() == 1 {
                            if this.addr() == f.player {
                                let first =
                                    e.call(PLAYER_GET_ANIMATION, &args![f.player, 1u32]).u32();
                                e.call(ANIMATION_BLEND_OUT, &args![first, 4u32, 0u32]);
                                let second =
                                    e.call(PLAYER_GET_ANIMATION, &args![f.player, 0u32]).u32();
                                e.call(ANIMATION_BLEND_OUT, &args![second, 4u32, 0u32]);
                            } else {
                                e.call(ANIMATION_BLEND_OUT, &args![animation, 4u32, 0u32]);
                            }
                        }
                    }
                }
            }
        }
        let lists = e.call(GET_EXTRA_DATA_LIST_ADDRESS, &args![this]).u32();
        e.call(EXTRA_SET_WEAPON_ATTACK_SOUND, &args![lists, sound]);
        return;
    }
    if !e.call(ACTOR_RANGE_TEST, &args![this]).bool() {
        return;
    }
    if e.call(ANIMATION_LOOKUP, &args![animation, 4u32]).u32() == 0 {
        return;
    }
    let group = e.call(ANIMATION_GROUP_FIELD, &args![animation, 4u32]).u16();
    if !e.call(IS_ATTACK_ACTION, &args![group as u32]).bool() {
        return;
    }
    let entry = e.call(ANIMATION_LOOKUP, &args![animation, 4u32]).u32();
    if e.call(ANIMATION_ENTRY_KIND, &args![entry]).u32() != 1 {
        return;
    }
    if e.call(ANIMATION_FIELD_0X70F490, &args![animation, 4u32])
        .i32()
        >= 1
    {
        return;
    }
    let lists = e.call(GET_EXTRA_DATA_LIST_ADDRESS, &args![this]).u32();
    e.call(EXTRA_GET_WEAPON_ATTACK_SOUND, &args![lists, sound]);
    if !e.call(SOUND_HANDLE_IS_VALID, &args![sound]).bool() {
        start_weapon_attack_sound(e, f);
    } else if !e.call(SOUND_HANDLE_IS_PLAYING, &args![sound]).bool() {
        e.call(SOUND_HANDLE_PLAY, &args![sound, 0u32]);
    }
    let lists = e.call(GET_EXTRA_DATA_LIST_ADDRESS, &args![this]).u32();
    e.call(EXTRA_SET_WEAPON_ATTACK_SOUND, &args![lists, sound]);
}

/// Creates the sound handle for the weapon attack sound of `Actor::Update`
/// (the weapon's sound file with flags depending on whether the actor is
/// the player), follows the actor's 3D object (or the virtual `+0x20c`
/// object) and sets its position.
fn start_weapon_attack_sound(e: &mut Engine, f: &UpdateFrame) {
    let this = f.this;
    let weapon = f.weapon;
    let mut follow = e.vcall(this.addr(), VSLOT_FOLLOW_0X20C, &args![]).u32();
    if follow == 0 {
        follow = e.vcall(this.addr(), VSLOT_GET_3D, &args![]).u32();
    }
    // The item is the frame's 0x24-byte local at `EBP-0x184` (`004e75d0` copies nine words into
    // it) and the handle a separate 0xc-byte one at `EBP-0x190`, so the frame holds both, the
    // handle after the whole item. (With the handle at +0x10 of a 0x20-byte block, the item's
    // copy ran over the handle and 4 bytes past the end.)
    e.with_stack(0x30, |e, scratch| {
        let item_scratch = scratch.addr();
        let handle_scratch = scratch.addr() + 0x24;
        let sound_object = e.call(WEAPON_OBJECT_OF, &args![weapon]).u32();
        let item = e
            .call(WEAPON_SOUND_ITEM, &args![sound_object, item_scratch])
            .u32();
        let word = e.mem.u32(item + 4);
        let sound_flags = e.call(SOUND_FLAGS_OF, &args![word]).u32();
        let mut flags: u32 = if this.addr() != f.player {
            0xf800_0001u32.wrapping_add(0x0800_0001)
        } else {
            0x0800_0001
        };
        if this.addr() == f.player && !e.call(PLAYER_TEST_0X524D10, &args![f.player]).bool() {
            flags |= 0x4_0000;
        }
        flags |= 0x1_0000;
        let first = e.call(WEAPON_OBJECT_OF, &args![weapon]).u32();
        let third = e.call(WEAPON_OBJECT_OF, &args![weapon]).u32();
        let name = e.call(WEAPON_SOUND_NAME, &args![third]).u32();
        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
        let made = e
            .call(
                AUDIO_GET_SOUND_HANDLE_BY_FILENAME,
                &args![
                    audio,
                    handle_scratch,
                    name,
                    sound_flags | flags | 0x4000_0000,
                    first
                ],
            )
            .u32();
        e.call(SOUND_HANDLE_ASSIGN, &args![f.sound, made]);
        e.call(EMPTY_FUNCTION, &args![handle_scratch]);
    });
    e.call(SOUND_HANDLE_SET_0X4F15A0, &args![f.sound, 1u32]);
    e.call(SOUND_HANDLE_SET_OBJECT_TO_FOLLOW, &args![f.sound, follow]);
    let position = e
        .vcall(this.addr(), VSLOT_GET_POSITION_0X1F4, &args![])
        .u32();
    e.call(SOUND_HANDLE_SET_POSITION_PTR, &args![f.sound, position]);
}

/// The movement mode part of `Actor::Update` for a non-player actor (the
/// player clears `in_combat` instead): the blocked flag, and the
/// `ActorMover` flags (`0x400`) kept in step with the follow and weapon
/// conditions.
fn update_movement_mode(e: &mut Engine, f: &mut UpdateFrame) {
    let this = f.this;
    let player = f.player;
    if this.addr() == player {
        f.in_combat = false;
        return;
    }
    let mut package = 0u32;
    if !e.get(this, Actor::pCurrentProcess).is_null() {
        let process = e.call(GET_PROCESS, &args![this]).u32();
        package = e.vcall(process, PROCESS_VSLOT_0X27C, &args![]).u32();
    }
    if f.in_combat {
        return;
    }
    let typed = package != 0
        && (e.call(OBJECT_TYPE, &args![package]).u32() == 8
            || e.call(OBJECT_TYPE, &args![package]).u32() == 0x10);
    if !typed && e.call(ACTOR_GET_BLOCKED, &args![this]).bool() {
        e.call(ACTOR_SET_BLOCK, &args![this, 0u32]);
    }
    let follow_state = e.call(ACTOR_MOVER_FLAG_TEST, &args![this]).bool()
        || (package != 0 && e.call(PACKAGE_TEST_0X67A4F0, &args![package]).bool())
        || (e.call(GET_FORCE_NEXT_UPDATE, &args![this]).bool()
            && e.call(SPEED_FLAG_TEST, &args![player]).bool()
            && e.call(PLAYER_TEST_0X9549A0, &args![player, this]).bool());
    if follow_state {
        if !e.call(SPEED_FLAG_TEST, &args![this]).bool() {
            fn_00884f80(e, this);
        }
        let mover = e.get(this, Actor::pActorMover);
        if !mover.is_null() {
            let mode = e.vcall(mover.addr(), MOVER_VSLOT_MODE, &args![]).u32();
            if mode & 0x400 == 0 {
                let counter = e.global::<f32>(FOLLOWER_BARK_COUNTDOWN);
                if counter <= 0.0 {
                    e.call(TRIGGER_FOLLOWER_BARK, &args![this, 0xcu32]);
                    let reset = e.global::<f32>(FOLLOWER_BARK_RESET);
                    e.set_global(FOLLOWER_BARK_COUNTDOWN, reset);
                }
            }
        }
        let mover = e.get(this, Actor::pActorMover);
        let mode = e.vcall(mover.addr(), MOVER_VSLOT_MODE, &args![]).u32();
        e.call(ACTOR_SET_MOVE_MODE, &args![this, mode | 0x400]);
        return;
    }
    let mover = e.get(this, Actor::pActorMover);
    let clear = e.call(SPEED_FLAG_TEST, &args![this]).bool()
        || (e.call(GET_FORCE_NEXT_UPDATE, &args![this]).bool()
            && !mover.is_null()
            && e.vcall(mover.addr(), MOVER_VSLOT_MODE, &args![]).u32() & 0x400 != 0);
    if clear {
        fn_00884f80(e, this);
        let word = e.call(GET_FLAG_WORD, &args![this]).u32();
        e.call(ACTOR_SET_MOVE_MODE, &args![this, word & !0x400u32]);
    }
}

/// The position part of `Actor::Update`: the player's 3D object is placed
/// at the actor's position (`00440460`), any other actor goes through
/// [`actor_update_actor_3d_position`] and the process virtual `+0x428`;
/// then the lighting of the extra scene node is refreshed and the
/// `ProcessLists` flag byte is cleared.
fn update_position(e: &mut Engine, f: &UpdateFrame) {
    let this = f.this;
    if f.player_flag {
        let position = e.call(POSITION_VECTOR_OF, &args![this]).u32();
        e.call(SET_WORLD_POSITION, &args![f.skeleton, position]);
        e.call(PLAYER_UPDATE_0X952290, &args![f.player]);
    } else {
        actor_update_actor_3d_position(e, this);
        let process = e.get(this, Actor::pCurrentProcess);
        e.vcall(process.addr(), PROCESS_VSLOT_0X428, &args![this]);
    }
    let link = e.call(GET_LINKED_OBJECT, &args![this]).u32();
    if link != 0 && e.call(WORD_AT_START, &args![link]).u32() != 0 {
        let word = e.call(WORD_AT_START, &args![link]).u32();
        let node = e.call(SHADOW_SCENE_NODE, &args![0u32]).u32();
        e.call(SHADOW_SCENE_NODE_UPDATE_0XB5F080, &args![node, word]);
    }
    fn_0088b020(e, 0);
}

/// The rest of `Actor::Update` for an actor other than the player: the
/// swim and water section (when it has a character controller and no
/// menu is open) and the animation block flags.
fn update_water_and_sounds(e: &mut Engine, f: &UpdateFrame) {
    let this = f.this;
    if f.controller != 0
        && !e.call(IS_IN_MENU_MODE, &args![]).bool()
        && !e
            .vcall(this.addr(), VSLOT_IS_PROCESS_FLAG_0X22C, &args![0u32])
            .bool()
    {
        swim_and_water(e, f);
    }
    animation_block_flags(e, f);
}

/// The process of `this` as read from `+0x68`.
fn process_of(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    e.get(this, Actor::pCurrentProcess).addr()
}

/// The swim and water section of `Actor::Update`.
fn swim_and_water(e: &mut Engine, f: &UpdateFrame) {
    let this = f.this;
    let position = e.call(POSITION_VECTOR_OF, &args![this]).u32();
    let z = e.mem.f32(position + 8);
    let depth = fn_00885560(e, this, z, Ptr::new(f.cell));
    let in_water = depth as f64 >= e.global::<f64>(WATER_THRESHOLD_A);
    let wading = depth as f64 >= e.global::<f64>(WATER_THRESHOLD_B);
    let deep = depth as f64 >= e.global::<f64>(WATER_THRESHOLD_C);
    let endurance = e.call(ACTOR_GET_ENDURANCE, &args![this]).f64();
    let ticks = e.call(FTOL2, &args![endurance]).u32();
    let breath_time = e.call(SWIM_BREATH_TIME, &args![ticks]).f32();
    let was_in_water = e.get(this, Actor::bInWater);
    let just_entered = !was_in_water && in_water;
    e.set(this, Actor::bInWater, in_water);
    let mut breathing = true;
    if depth as f64 <= 0.0 {
        breathing = e.call(ACTOR_TEST_0X87F620, &args![this]).bool();
    } else if deep {
        let magic = this.addr() + 0x94;
        if !e.vcall(magic, MAGIC_VSLOT_0X10, &args![]).bool()
            && !e.call(ACTOR_TEST_0X87F5C0, &args![this]).bool()
        {
            breathing = false;
        }
    }
    if breathing {
        let process = process_of(e, this);
        e.vcall(process, PROCESS_VSLOT_0X2FC, &args![breath_time]);
    } else {
        drowning_step(e, f, breath_time);
    }
    let creature_sound = creature_sound_step(e, f);
    let _ = just_entered;
    controller_mode_step(e, f, creature_sound, in_water, wading, just_entered);
    water_effects(e, f, in_water, wading, creature_sound);
}

/// The breath and drowning part of the swim section: the process's
/// breath timer (virtual `+0x300`) runs down by the frame step; when it
/// is spent the actor takes drowning damage and the pain sound
/// plays; otherwise the timer is capped by the breath time, and an actor
/// with the surfacing conditions starts the surface package.
fn drowning_step(e: &mut Engine, f: &UpdateFrame, breath_time: f32) {
    let this = f.this;
    let player = f.player;
    let process = process_of(e, this);
    let timer = e.vcall(process, PROCESS_VSLOT_0X300, &args![]).f32();
    let step = e
        .call(UPDATE_STEP_SETTING, &args![UPDATE_STEP_OBJECT])
        .f64();
    let mut remaining = (timer as f64 - step) as f32;
    if remaining < 0.0 {
        if e.vcall(this.addr(), VSLOT_ACTOR_FLAG_0X360, &args![])
            .bool()
        {
            let owner = this.addr() + ACTOR_VALUE_OWNER_OFFSET;
            let value = e.vcall(owner, 0, &args![0x10u32]).u32();
            let damage_rate = e.call(DAMAGE_RATE, &args![value]).f64();
            let step = e
                .call(UPDATE_STEP_SETTING, &args![UPDATE_STEP_OBJECT])
                .f64();
            let damage = (step * damage_rate) as f32;
            e.vcall(
                this.addr(),
                VSLOT_DAMAGE_0X338,
                &args![damage, 0.0f32, 0u32],
            );
            e.with_stack(0x10, |e, handle| {
                e.call(SOUND_HANDLE_INIT, &args![handle]);
                let lists = e.call(GET_EXTRA_DATA_LIST_ADDRESS, &args![this]).u32();
                e.call(EXTRA_GET_SOUND, &args![lists, handle]);
                if !e.call(SOUND_HANDLE_IS_VALID, &args![handle]).bool() {
                    e.with_stack(0x10, |e, scratch| {
                        let made = e
                            .call(
                                PLAY_SOUND_BY_EDITOR_NAME,
                                &args![this, scratch, DROWNING_SOUND_NAME, 0u32, 0x102u32, 0u32],
                            )
                            .u32();
                        e.call(SOUND_HANDLE_ASSIGN, &args![handle, made]);
                        e.call(EMPTY_FUNCTION, &args![scratch]);
                    });
                    let lists = e.call(GET_EXTRA_DATA_LIST_ADDRESS, &args![this]).u32();
                    e.call(EXTRA_SET_SOUND, &args![lists, handle]);
                }
                e.call(ACTOR_TRIGGER_PAIN, &args![this, 0u32, 1u32]);
                e.call(EMPTY_FUNCTION, &args![handle]);
            });
        }
        remaining = 0.0;
    } else if this.addr() != player {
        if breath_time < remaining {
            remaining = breath_time;
        }
        let speed = fn_00885d90(e, this);
        let water = e.call(GET_RELEVANT_WATER_HEIGHT, &args![this]).f64();
        let position = e
            .vcall(this.addr(), VSLOT_GET_POSITION_0X1F4, &args![])
            .u32();
        let height = e.mem.f32(position + 8);
        let depth = (water - height as f64) as f32;
        let ratio = (depth as f64 / speed as f64) as f32;
        let scaled = (ratio as f64 * e.global::<f64>(DROWN_TIME_FACTOR)) as f32;
        let one = e.global::<f64>(DOUBLE_ONE);
        if (remaining as f64) < scaled as f64 + one
            && !e.call(ACTOR_TEST_0X87F570, &args![this]).bool()
            && !e.call(ACTOR_IS_SURFACING, &args![this]).bool()
        {
            e.call(ACTOR_INITIATE_SURFACE_PACKAGE, &args![this]);
        }
    }
    let process = process_of(e, this);
    e.vcall(process, PROCESS_VSLOT_0X2FC, &args![remaining]);
}

/// The creature sound part of the swim section: while the actor passes
/// `vcall(+0x21c)`, its creature "awake" sound (`PickCreatureSound` with 11)
/// is created, positioned and played, or stopped. Returns the byte
/// `0087f390` gave (the creature flag the rest of the section uses).
fn creature_sound_step(e: &mut Engine, f: &UpdateFrame) -> bool {
    let this = f.this;
    let mut creature = false;
    if !e
        .vcall(this.addr(), VSLOT_ANIMATION_FLAG_0X21C, &args![])
        .bool()
    {
        return creature;
    }
    creature = e.call(ACTOR_TEST_0X87F390, &args![this]).bool();
    let form = e.call(SAVE_FORM_BUFFER_GET_FORM, &args![this]).u32();
    let sounds = if form != 0 {
        e.call(PICK_CREATURE_SOUND, &args![form, 0xbu32]).u32()
    } else {
        0
    };
    if sounds == 0 || e.vcall(this.addr(), VSLOT_GET_3D, &args![]).u32() == 0 {
        return creature;
    }
    let awake = !e.call(ACTOR_BLOCKED_TEST, &args![this]).bool()
        && !e
            .vcall(this.addr(), VSLOT_IS_PROCESS_FLAG_0X22C, &args![0u32])
            .bool();
    e.with_stack(0x10, |e, handle| {
        e.call(SOUND_HANDLE_INIT, &args![handle]);
        let lists = e.call(GET_EXTRA_DATA_LIST_ADDRESS, &args![this]).u32();
        e.call(EXTRA_GET_CREATURE_AWAKE_SOUND, &args![lists, handle]);
        if awake && !e.call(SOUND_HANDLE_IS_PLAYING, &args![handle]).bool() {
            if !e.call(SOUND_HANDLE_IS_VALID, &args![handle]).bool() {
                // The item is the frame's 0x24-byte local at `EBP-0x1c0`: `004e75d0` copies
                // nine words into it and answers its address, and the word at +4 is read
                // through that address, so the read is made while it is there. (A 0x10-byte
                // block scoped to `004e75d0` alone was written 0x14 bytes past its end and read
                // after it was freed.)
                let word = e.with_stack(0x24, |e, scratch| {
                    let item = e.call(WEAPON_SOUND_ITEM, &args![sounds, scratch]).u32();
                    e.mem.u32(item + 4)
                });
                let flags = e.call(SOUND_FLAGS_OF, &args![word]).u32() | 2;
                let name = e.call(WEAPON_SOUND_NAME, &args![sounds]).u32();
                let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
                e.with_stack(0x10, |e, scratch| {
                    let made = e
                        .call(
                            AUDIO_GET_SOUND_HANDLE_BY_FILENAME,
                            &args![audio, scratch, name, flags, sounds],
                        )
                        .u32();
                    e.call(SOUND_HANDLE_ASSIGN, &args![handle, made]);
                    e.call(EMPTY_FUNCTION, &args![scratch]);
                });
                let lists = e.call(GET_EXTRA_DATA_LIST_ADDRESS, &args![this]).u32();
                e.call(EXTRA_SET_CREATURE_AWAKE_SOUND, &args![lists, handle]);
                let follow = e.vcall(this.addr(), VSLOT_GET_3D, &args![]).u32();
                e.call(SOUND_HANDLE_SET_OBJECT_TO_FOLLOW, &args![handle, follow]);
            }
            if !e.call(SOUND_HANDLE_IS_PLAYING, &args![handle]).bool() {
                let position = e
                    .vcall(this.addr(), VSLOT_GET_POSITION_0X1F4, &args![])
                    .u32();
                e.call(SOUND_HANDLE_SET_POSITION_PTR, &args![handle, position]);
                e.call(SOUND_HANDLE_PLAY, &args![handle, 0u32]);
            }
        } else if e.call(SOUND_HANDLE_IS_PLAYING, &args![handle]).bool()
            && (!awake
                || e.vcall(this.addr(), VSLOT_IS_PROCESS_FLAG_0X22C, &args![0u32])
                    .bool())
        {
            e.call(SOUND_HANDLE_STOP, &args![handle]);
            e.call(SOUND_HANDLE_RELEASE, &args![handle]);
        }
        e.call(EMPTY_FUNCTION, &args![handle]);
    });
    creature
}

/// Sets the character controller's move mode word and the process's
/// cached-values flags the way `Actor::Update` does after a move mode
/// change: when the process has a cached-values block, `0x1000` and
/// `0x2000` are forwarded to it.
fn forward_cached_flags(e: &mut Engine, this: Ptr<Actor>) {
    let process = process_of(e, this);
    if process != 0 && e.call(HAS_CACHED_VALUES, &args![process]).bool() {
        let process = process_of(e, this);
        e.call(PROCESS_FORWARD_FLAG_MASK, &args![process, 0x1000u32]);
        let process = process_of(e, this);
        e.call(PROCESS_FORWARD_FLAG_MASK, &args![process, 0x2000u32]);
    }
}

/// `ActorMover` virtual `+0x1c` (the mode word) of the actor's mover.
fn mover_mode(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    let mover = e.get(this, Actor::pActorMover);
    e.vcall(mover.addr(), MOVER_VSLOT_MODE, &args![]).u32()
}

/// Stores the water height the call `TES::GetWaterHeight`-like `0045cbc0`
/// gives for the actor's position into the float at +8 of the object at
/// `this + 0x64`.
fn store_water_height(e: &mut Engine, this: Ptr<Actor>) {
    let position = e
        .vcall(this.addr(), VSLOT_GET_POSITION_0X1F4, &args![])
        .u32();
    let tes = e.mem.u32(TES_POINTER);
    let height = e
        .call(TES_WATER_HEIGHT_AT, &args![tes, position, 0u32])
        .f32();
    let target = e.mem.u32(this.addr() + 0x64);
    e.mem.set_f32(target + 8, height);
}

/// The controller mode part of the swim section of `Actor::Update`
/// (`00889ffa` to `0088a7f0`): the controller's `0088b110` value, then for
/// creatures the water-height and wading mode handling and the
/// velocity modifier, and for the others the wading, leaving-water and
/// splash handling.
fn controller_mode_step(
    e: &mut Engine,
    f: &UpdateFrame,
    creature: bool,
    in_water: bool,
    wading: bool,
    just_entered: bool,
) {
    let this = f.this;
    let player = f.player;
    let controller = f.controller;
    let value_address = if creature {
        CONTROLLER_VALUE_CREATURE
    } else if this.addr() == player && !e.call(PLAYER_TEST_0X524D10, &args![player]).bool() {
        CONTROLLER_VALUE_PLAYER
    } else {
        CONTROLLER_VALUE_OTHER
    };
    let value = e.global::<f32>(value_address);
    fn_0088b110(e, Ptr::new(controller), value);
    if creature {
        creature_mode(e, f, in_water, wading);
    } else {
        other_mode(e, f, wading, just_entered);
    }
}

/// The creature branch of [`controller_mode_step`].
fn creature_mode(e: &mut Engine, f: &UpdateFrame, in_water: bool, wading: bool) {
    let this = f.this;
    let controller = f.controller;
    if e.call(CONTROLLER_STATE, &args![controller]).u32() == 5 {
        let target = e.mem.u32(this.addr() + 0x64);
        if e.mem.f32(target + 8) as f64 == e.global::<f64>(WATER_HEIGHT_UNSET) {
            store_water_height(e, this);
        }
    }
    if wading {
        let mode = mover_mode(e, this);
        e.call(ACTOR_SET_MOVE_MODE, &args![this, mode | 0x800]);
    } else {
        e.call(ACTOR_CLEAR_MOVE_MODE, &args![this, 0x800u32]);
    }
    forward_cached_flags(e, this);
    if e.call(CONTROLLER_STATE, &args![controller]).u32() == 5 && !in_water {
        e.call(CONTROLLER_SET_0X520, &args![controller, 0u32]);
        let target = e.mem.u32(this.addr() + 0x64);
        let height = e.global::<f32>(NO_WATER_HEIGHT);
        e.mem.set_f32(target + 8, height);
    } else if e.call(CONTROLLER_STATE, &args![controller]).u32() != 5 && in_water {
        let value = indexed_float(e, controller + 0x4f0, 2);
        if (value as f64) < e.global::<f64>(DOUBLE_ONE) {
            let water = e.call(GET_RELEVANT_WATER_HEIGHT, &args![this]).f64();
            if water == e.global::<f64>(WATER_HEIGHT_UNSET) {
                store_water_height(e, this);
            }
            e.call(CONTROLLER_SET_0X520, &args![controller, 5u32]);
        }
    }
    if e.call(CONTROLLER_STATE, &args![controller]).u32() == 5 {
        let limit = e.global::<f64>(WATER_THRESHOLD_A);
        let current = e.mem.f32(controller + 0x524);
        if (current as f64) < limit && in_water && !wading {
            let base = fn_0088b130(e, Ptr::new(controller));
            let low = indexed_float(e, base, 2);
            let proxy = e.call(CHARACTER_PROXY_POSITION, &args![controller]).u32();
            let high = indexed_float(e, proxy, 2);
            let factor = e.global::<f64>(CONTROLLER_FACTOR_A);
            let blend = ((high as f64 - low as f64)
                + (low as f64 * factor * e.mem.f32(controller + 0x55c) as f64))
                as f32;
            let reference = (e.mem.f32(controller + 0x53c) as f64
                - e.global::<f64>(CONTROLLER_OFFSET_A)) as f32;
            let drop = e
                .call(CONTROLLER_FUNCTION_0X4A3E90, &args![reference])
                .f64();
            let excess = (blend as f64 - drop) as f32;
            if (excess as f64) < factor {
                velocity_modifier(e, controller, CONTROLLER_PUSH_DOWN);
            } else if excess as f64 > factor {
                velocity_modifier(e, controller, CONTROLLER_PUSH_UP);
            }
        }
    }
}

/// `bhkCharacterController::SetVelocityModifier(controller, (0, 0, z), 1.0)`
/// with `z` the float at `z_address`.
fn velocity_modifier(e: &mut Engine, controller: u32, z_address: u32) {
    let z = e.global::<f32>(z_address);
    e.with_stack(0xc, |e, vector| {
        let made = e
            .call(NI_POINT3_SET, &args![vector, 0.0f32, 0.0f32, z])
            .u32();
        e.call(SET_VELOCITY_MODIFIER, &args![controller, made, 1.0f32]);
    });
}

/// The branch of [`controller_mode_step`] for the other actors.
fn other_mode(e: &mut Engine, f: &UpdateFrame, wading: bool, just_entered: bool) {
    let this = f.this;
    let controller = f.controller;
    if wading {
        let mode = mover_mode(e, this);
        e.call(ACTOR_SET_MOVE_MODE, &args![this, mode | 0x800]);
        e.call(CONTROLLER_SET_0X520, &args![controller, 5u32]);
        e.call(ACTOR_TEST_0X8A6840, &args![this, 0u32]);
        forward_cached_flags(e, this);
        return;
    }
    if e.call(ACTOR_TEST_0X5A2030, &args![this]).bool()
        || e.call(GET_FLAG_WORD, &args![this]).u32() & 0x800 != 0
        || e.call(CONTROLLER_STATE, &args![controller]).u32() == 5
    {
        if e.call(CONTROLLER_FLAG_0X88B0C0, &args![controller]).bool() {
            e.call(ACTOR_CLEAR_MOVE_MODE, &args![this, 0x800u32]);
            e.call(CONTROLLER_SET_0X520, &args![controller, 0u32]);
            forward_cached_flags(e, this);
        }
        return;
    }
    if just_entered {
        splash_sound_and_effects(e, f);
    }
}

/// The splash part of [`other_mode`]: when the actor has just entered the
/// water and the controller's velocity is high enough, plays the splash
/// sound at the actor's position and, over non-dangerous water, the
/// splash particle effects.
fn splash_sound_and_effects(e: &mut Engine, f: &UpdateFrame) {
    let this = f.this;
    let controller = f.controller;
    let speed = e.call(CONTROLLER_SPEED_0X88B0F0, &args![controller]).f64();
    let fast_enough = speed > e.global::<f64>(DOUBLE_HALF);
    if !fast_enough {
        return;
    }
    let velocity = indexed_float(e, controller + 0x4f0, 2);
    let slow_enough = (velocity as f64) < e.global::<f64>(CONTROLLER_LIMIT_B);
    if !slow_enough {
        return;
    }
    let velocity = indexed_float(e, controller + 0x4f0, 2);
    let mut volume = 1.0f32;
    if (velocity as f64) > e.global::<f64>(CONTROLLER_LIMIT_C) {
        volume = (velocity as f64 * e.global::<f64>(CONTROLLER_LIMIT_B)
            / e.global::<f64>(CONTROLLER_DIVISOR)) as f32;
    }
    let cap = e.global::<f32>(VOLUME_CAP);
    volume = e.call(CLAMP_MIN_4DD150, &args![volume, cap]).f32();
    e.with_stack(0x10, |e, handle| {
        e.call(SOUND_HANDLE_INIT, &args![handle]);
        if e.vcall(this.addr(), VSLOT_GET_3D, &args![]).u32() == 0 {
            e.call(EMPTY_FUNCTION, &args![handle]);
            return;
        }
        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
        let made = e.with_stack(0x10, |e, scratch| {
            let made = e
                .call(
                    AUDIO_GET_SOUND_HANDLE_BY_NAME,
                    &args![audio, scratch, SPLASH_SOUND_NAME, 0x102u32],
                )
                .u32();
            e.call(SOUND_HANDLE_ASSIGN, &args![handle, made]);
            e.call(EMPTY_FUNCTION, &args![scratch]);
            made
        });
        let _ = made;
        if e.call(SOUND_HANDLE_IS_PLAYING, &args![handle]).bool() {
            e.call(EMPTY_FUNCTION, &args![handle]);
            return;
        }
        let position = e
            .vcall(this.addr(), VSLOT_GET_POSITION_0X1F4, &args![])
            .u32();
        let (x, y, z) = (
            e.mem.u32(position),
            e.mem.u32(position + 4),
            e.mem.u32(position + 8),
        );
        e.call(SOUND_HANDLE_SET_POSITION, &args![handle, x, y, z]);
        e.call(SOUND_HANDLE_SET_VOLUME, &args![handle, volume]);
        e.call(SOUND_HANDLE_PLAY, &args![handle, 0u32]);
        if f.cell != 0 {
            let water_type = e.call(CELL_GET_WATER_TYPE, &args![f.cell]).u32();
            if water_type != 0 {
                let water_type = e.call(CELL_GET_WATER_TYPE, &args![f.cell]).u32();
                if !e.call(WATER_FORM_GET_DANGEROUS, &args![water_type]).bool() {
                    splash_particles(e, f, [x, y, z]);
                }
            }
        }
        e.call(EMPTY_FUNCTION, &args![handle]);
    });
}

/// `TESWaterListener::PlaySplashEffects` for the splash at `position`
/// (the water height replaces its z), with a random rotation.
fn splash_particles(e: &mut Engine, f: &UpdateFrame, position: [u32; 3]) {
    let this = f.this;
    let water_height = e.call(GET_RELEVANT_WATER_HEIGHT, &args![this]).f32();
    let at_water = [position[0], position[1], water_height.to_bits()];
    let matrix_words = e.with_stack(0x24, |e, matrix| {
        e.call(MATRIX_CONSTRUCT, &args![matrix]);
        let angle = e.call(RANDOM_ANGLE, &args![]).f32();
        e.call(MATRIX_MAKE_ROTATION, &args![matrix, angle]);
        let mut words = [0u32; 9];
        for (i, word) in words.iter_mut().enumerate() {
            *word = e.mem.u32(matrix.addr() + 4 * i as u32);
        }
        words
    });
    let rate_address = e
        .call(SETTING_VALUE_ADDRESS, &args![SPLASH_SETTING_A])
        .u32();
    let rate = e.mem.f32(rate_address);
    let count = e.call(SETTING_INT_FN, &args![SPLASH_SETTING_B]).u32();
    let mut arguments = args![f.cell, 1.0f32, count];
    for word in matrix_words {
        arguments.push(word);
    }
    for word in at_water {
        arguments.push(word);
    }
    arguments.push(rate.to_bits());
    arguments.push(1);
    arguments.push(0);
    e.call(CREATE_SPLASH_EFFECT, &arguments);
    let object = e.vcall(this.addr(), VSLOT_GET_3D, &args![]).u32();
    let size = e.call(OBJECT_SIZE_0X87CE50, &args![object]).f32();
    e.call(
        PLAY_SPLASH_EFFECTS,
        &args![size, position[0], position[1], position[2]],
    );
}

/// The damage and breath part of the swim section (`0088a7f0` to
/// `0088aaad`): the player's process virtual `+0x760`, the drowning
/// damage of dangerous water, and the ProcessLists flag byte.
fn water_effects(e: &mut Engine, f: &UpdateFrame, in_water: bool, wading: bool, creature: bool) {
    let this = f.this;
    let player = f.player;
    if this.addr() == player && f.process != 0 {
        e.vcall(f.process, PROCESS_VSLOT_0X760, &args![0.0f32]);
    }
    if !in_water || creature {
        fn_0088b050(e, Ptr::new(PROCESS_LISTS), 0);
        return;
    }
    if f.cell == 0 {
        return;
    }
    let water_type = e.call(CELL_GET_WATER_TYPE, &args![f.cell]).u32();
    if water_type != 0 {
        let water_type = e.call(CELL_GET_WATER_TYPE, &args![f.cell]).u32();
        if e.call(WATER_FORM_GET_DANGEROUS, &args![water_type]).bool() {
            let sub = water_type + 0x24;
            let rate = e.vcall(sub, 0x10, &args![]).u16();
            let step = e
                .call(UPDATE_STEP_SETTING, &args![UPDATE_STEP_OBJECT])
                .f64();
            let damage = (step * rate as f64) as f32;
            if damage as f64 > 0.0 {
                e.vcall(
                    this.addr(),
                    VSLOT_DAMAGE_0X338,
                    &args![damage, 0.0f32, 0u32],
                );
                e.call(ACTOR_TRIGGER_PAIN, &args![this, 1u32, 1u32]);
            }
        }
    }
    if water_type == 0 {
        return;
    }
    let layers = e.call(WATER_TYPE_COUNT, &args![water_type]).i32();
    if layers <= 0 {
        return;
    }
    if !e
        .vcall(this.addr(), VSLOT_ACTOR_FLAG_0X360, &args![])
        .bool()
        && !e
            .vcall(this.addr(), VSLOT_ANIMATION_FLAG_0X21C, &args![])
            .bool()
    {
        return;
    }
    let mut amount = layers as f32;
    let factor = if wading || e.call(GET_FLAG_WORD, &args![this]).u32() & 0x800 != 0 {
        setting_value(e, WATER_DAMAGE_SETTING_A)
    } else {
        setting_value(e, WATER_DAMAGE_SETTING_B)
    };
    amount = (amount as f64 * factor as f64) as f32;
    let resistance = e
        .call(ACTOR_GET_RADIATION_RESISTANCE_MULT, &args![this])
        .f32();
    amount = (amount as f64 * resistance as f64) as f32;
    if this.addr() == player && f.process != 0 {
        e.vcall(f.process, PROCESS_VSLOT_0X760, &args![amount]);
        e.call(PLAYER_RADIATION_0X968730, &args![player]);
    }
    let step = e
        .call(UPDATE_STEP_SETTING, &args![UPDATE_STEP_OBJECT])
        .f64();
    let scaled = (step * amount as f64) as f32;
    if this.addr() == player {
        fn_0088b050(e, Ptr::new(PROCESS_LISTS), (scaled as f64 > 0.0) as u8);
    }
    e.vcall(this.addr(), VSLOT_0X3AC, &args![0x36u32, scaled, 0u32]);
}

/// The last part of `Actor::Update` for an actor that is not the player
/// (`0088aaad` to `0088afcc`): when it is dead-animated (virtual `+0x2e8`)
/// the animation block is managed (a blend out, the face animation data,
/// clearing the interpolators), otherwise the process timer `+0x4b0` is
/// run down.
fn animation_block_flags(e: &mut Engine, f: &UpdateFrame) {
    let this = f.this;
    let player = f.player;
    let animation = f.animation;
    let face = f.face;
    if !(e.vcall(this.addr(), VSLOT_FLAG_0X2E8, &args![]).bool() && animation != 0) {
        if e.vcall(this.addr(), VSLOT_IS_PROCESS_FLAG_0X22C, &args![0u32])
            .bool()
        {
            let process = process_of(e, this);
            let value = e.vcall(process, PROCESS_VSLOT_0X4B0, &args![]).f64();
            if value == e.global::<f64>(PROCESS_TIMER_UNSET) {
                let process = process_of(e, this);
                e.vcall(process, PROCESS_VSLOT_0X4B4, &args![0.0f32]);
            }
            let process = process_of(e, this);
            let value = e.vcall(process, PROCESS_VSLOT_0X4B0, &args![]).f64();
            let step = e
                .call(UPDATE_STEP_SETTING, &args![UPDATE_STEP_OBJECT])
                .f64();
            let remaining = (value - step) as f32;
            let process = process_of(e, this);
            e.vcall(process, PROCESS_VSLOT_0X4B4, &args![remaining]);
        }
        return;
    }
    let mut waking = true;
    let group = e.call(ANIMATION_GROUP_FIELD, &args![animation, 1u32]).u16();
    let is_dying = e.call(ANIM_GROUP_GET_TYPE, &args![group as u32]).i32() == 0xe0;
    if waking && !is_dying {
        let process = e.call(GET_PROCESS, &args![this]).u32();
        let current = e.call(PROCESS_FLOAT_0X7DF1F0, &args![process]).f32();
        let hour = e.call(CALENDAR_GET_HOUR, &args![CALENDAR]).f32();
        let difference = if current as f64 > hour as f64 {
            (current as f64 + e.global::<f64>(HOURS_PER_DAY) - hour as f64) as f32
        } else {
            (hour as f64 - current as f64) as f32
        };
        let scale = e.call(CALENDAR_GET_TIME_SCALE, &args![CALENDAR]).f64();
        waking = (difference as f64) < scale * e.global::<f64>(WAKE_TIME_FACTOR);
    }
    let owner = e.call(WORD_AT_0XC, &args![animation]).u32();
    let object = if owner != 0 {
        e.vcall(owner, 0xc, &args![]).u32()
    } else {
        0
    };
    if object != 0 && !is_dying {
        let blend = e.call(GET_BIP_BLEND_VALUE, &args![object]).f64();
        if blend <= 0.0 {
            if this.addr() == player {
                let first = e.call(PLAYER_GET_ANIMATION, &args![player, 0u32]).u32();
                free_animation(e, first);
                let second = e.call(PLAYER_GET_ANIMATION, &args![player, 1u32]).u32();
                free_animation(e, second);
            } else {
                free_animation(e, animation);
            }
            let process = process_of(e, this);
            e.vcall(process, PROCESS_VSLOT_0X294, &args![this]);
        }
    }
    if face != 0 && !e.vcall(face, FACE_VSLOT_0XD4, &args![]).bool() {
        let process = process_of(e, this);
        let start = if process != 0 {
            let made = e.with_stack(0x10, |e, scratch| {
                let made = e
                    .vcall(process, PROCESS_VSLOT_0X48C, &args![scratch, 0u32])
                    .u32();
                let valid = e.call(SOUND_HANDLE_IS_VALID, &args![made]).bool();
                e.call(EMPTY_FUNCTION, &args![scratch]);
                valid
            });
            !made
        } else {
            true
        };
        if start {
            e.vcall(face, FACE_VSLOT_0XD8, &args![1u32, 0u32]);
        }
    }
    if is_dying {
        return;
    }
    if waking && face != 0 && e.vcall(face, FACE_VSLOT_0XC8, &args![]).bool() {
        return;
    }
    let mut interpolate = true;
    if e.call(ANIMATION_GROUP_LOADED, &args![animation, 0xe0u32])
        .bool()
    {
        let key = e.mem.u32(ANIMATION_GROUP_KEY);
        if e.call(ANIMATION_LOOKUP, &args![animation, key]).u32() != 0 {
            interpolate = false;
        } else {
            let process = e.call(GET_PROCESS, &args![this]).u32();
            if e.vcall(process, PROCESS_VSLOT_0X618, &args![]).u32() & 0x20 != 0 {
                interpolate = false;
            }
        }
    }
    e.vcall(this.addr(), VSLOT_0XD4, &args![1u32]);
    if interpolate {
        e.call(CLEAR_INTERPOLATORS, &args![this]);
    }
}

/// `Animation::SpecialIdleFree(1, 0)`, `Animation::ClearGroup(0x14, 0.0)`
/// and `004964d0` on one animation (the sequence `Actor::Update` repeats).
fn free_animation(e: &mut Engine, animation: u32) {
    e.call(ANIMATION_SPECIAL_IDLE_FREE, &args![animation, 1u32, 0u32]);
    e.call(ANIMATION_CLEAR_GROUP, &args![animation, 0x14u32, 0.0f32]);
    e.call(ANIMATION_FUNCTION_0X4964D0, &args![animation]);
}

/// The `float` at element `index` of the vector-like object `base`
/// (`00560d30` gives the element's address).
fn indexed_float(e: &mut Engine, base: u32, index: u32) -> f32 {
    let address = e.call(INDEXED_ELEMENT_ADDRESS, &args![base, index]).u32();
    e.mem.f32(address)
}
// Translated from 0088b020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the byte into the global at `011f3e60` (`cdecl`).
pub fn fn_0088b020(e: &mut Engine, value: u8) {
    e.mem.set_u8(GLOBAL_FLAG_011F3E60, value);
}

// Translated from 0088b050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the byte at +0x15d of `this` (`Actor::Update` calls it on the
/// `ProcessLists` object `011e0e80`).
pub fn fn_0088b050(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x15d, value);
}

// Translated from 0088b070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the byte `*flag` to `+0x69` of the character controller and
/// clears the word at `+0x6c` when it is zero.
pub fn fn_0088b070(e: &mut Engine, this: Ptr, flag: Ptr) {
    let value = e.mem.u8(flag.addr());
    e.mem.set_u8(this.addr() + 0x69, value);
    if value == 0 {
        e.mem.set_u32(this.addr() + 0x6c, 0);
    }
}

// Translated from 0088b110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the `float` at `+0x560`.
pub fn fn_0088b110(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_f32(this.addr() + 0x560, x87_quiet(value));
}

// Translated from 0088b130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the address `this + 0x570`.
pub fn fn_0088b130(_e: &mut Engine, this: Ptr) -> u32 {
    this.addr() + 0x570
}

// Translated from 0088b150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::UpdateActor3DPosition` (Xbox PDB): places the actor's 3D object
/// at its position (unless `00440460` is skipped by the global at
/// `011e0550`) and builds its rotation from the heading (`GetHeading` for
/// the player, virtual `+0x2bc` otherwise) and the character controller's
/// two extra rotations, then, for a non-player actor outside the
/// player's sleep or rest, updates the lighting of the 3D object through
/// `ShadowSceneNode::UpdateObjectLighting` and tracks the result in
/// `bLightingUpdatedNonMoving`.
pub fn actor_update_actor_3d_position(e: &mut Engine, this: Ptr<Actor>) {
    let object = e.vcall(this.addr(), VSLOT_GET_3D, &args![]).u32();
    if object == 0 {
        return;
    }
    let controller = e.call(GET_CHAR_CONTROLLER, &args![this]).u32();
    let player = e.mem.u32(PLAYER_CHARACTER);
    let position = e.call(POSITION_VECTOR_OF, &args![this]).u32();
    let vector = [
        e.mem.u32(position),
        e.mem.u32(position + 4),
        e.mem.u32(position + 8),
    ];
    let heading = if this.addr() == player {
        e.call(GET_HEADING, &args![this, 1u32]).f32()
    } else {
        e.vcall(this.addr(), VSLOT_GET_HEADING_0X2BC, &args![1u32])
            .f32()
    };
    // Eight 9-word matrices live on the engine's stack: the rotation, the
    // second rotation, three product results and the race-adjusted copy.
    e.with_stack(0xf0, |e, base| {
        let rotation = base.addr();
        let second = base.addr() + 0x24;
        let product = base.addr() + 0x48;
        let adjusted = base.addr() + 0x6c;
        let position_copy = base.addr() + 0x90;
        for (i, word) in vector.iter().enumerate() {
            e.mem.set_u32(position_copy + 4 * i as u32, *word);
        }
        if e.mem.u8(SKIP_SET_POSITION) == 0 {
            e.call(SET_WORLD_POSITION, &args![object, position_copy]);
        }
        e.call(MATRIX_CONSTRUCT, &args![rotation]);
        e.call(MATRIX_MAKE_ROTATION, &args![rotation, heading]);
        if controller != 0 {
            e.call(MATRIX_CONSTRUCT, &args![second]);
            let pitch = e.call(CONTROLLER_PITCH, &args![controller]).f32();
            if pitch != 0.0 {
                let pitch = e.call(CONTROLLER_PITCH, &args![controller]).f32();
                e.call(MATRIX_MAKE_X_ROTATION, &args![second, pitch]);
                let result = e
                    .call(MATRIX_MULTIPLY, &args![rotation, product, second])
                    .u32();
                copy_words(e, result, rotation, 9);
            }
            let roll = e.call(CONTROLLER_ROLL, &args![controller]).f32();
            if roll != 0.0 {
                let roll = e.call(CONTROLLER_ROLL, &args![controller]).f32();
                e.call(MATRIX_MAKE_Y_ROTATION, &args![second, roll]);
                let result = e
                    .call(MATRIX_MULTIPLY, &args![rotation, product, second])
                    .u32();
                copy_words(e, result, rotation, 9);
            }
        }
        let result = e
            .call(MULTIPLY_MATRIX_BY_RACE, &args![this, adjusted, rotation])
            .u32();
        e.call(SET_WORLD_ROTATION, &args![object, result]);
    });
    if this.addr() == player || e.call(PLAYER_SLEEPING_OR_RESTING, &args![player]).bool() {
        return;
    }
    if e.vcall(this.addr(), VSLOT_GET_3D, &args![]).u32() == 0 {
        return;
    }
    let lists = e.call(GET_EXTRA_DATA_LIST_ADDRESS, &args![this]).u32();
    let dismembered = e.call(EXTRA_GET_DISMEMBERMENT, &args![lists]).u32() != 0;
    let cell = e.call(GET_PARENT_CELL, &args![this]).u32();
    e.call(CELL_UPDATE_REFERENCE, &args![cell, this, 1u32, dismembered]);
    if e.call(ACTOR_LIGHTING_FORCE_TEST, &args![this]).bool() {
        update_object_lighting(e, this, true);
        e.set(this, Actor::bLightingUpdatedNonMoving, false);
        return;
    }
    let mut updated_moving = false;
    if e.vcall(this.addr(), VSLOT_IS_PROCESS_FLAG_0X22C, &args![0u32])
        .bool()
        || e.vcall(this.addr(), VSLOT_FLAG_0X2E8, &args![]).bool()
    {
        let object = e.vcall(this.addr(), VSLOT_GET_3D, &args![]).u32();
        let collision = e.call(FIND_NEXT_COLLISION_OBJECT, &args![object]).u32();
        let rigid_body = if collision != 0 {
            e.call(COLLISION_RIGID_BODY, &args![collision]).u32()
        } else {
            0
        };
        if rigid_body != 0 && e.call(RIGID_BODY_IS_ACTIVE, &args![rigid_body]).bool() {
            update_object_lighting(e, this, true);
            e.set(this, Actor::bLightingUpdatedNonMoving, false);
            updated_moving = true;
        }
    }
    if !e.get(this, Actor::bLightingUpdatedNonMoving) && !updated_moving {
        update_object_lighting(e, this, false);
        e.set(this, Actor::bLightingUpdatedNonMoving, true);
    }
}

/// `ShadowSceneNode::UpdateObjectLighting(object, flag)` (Xbox PDB) on the
/// scene node `00450b80(0)` returns, for the actor's 3D object.
fn update_object_lighting(e: &mut Engine, this: Ptr<Actor>, flag: bool) {
    let object = e.vcall(this.addr(), VSLOT_GET_3D, &args![]).u32();
    let node = e.call(SHADOW_SCENE_NODE, &args![0u32]).u32();
    e.call(UPDATE_OBJECT_LIGHTING, &args![node, object, flag]);
}

/// Copies `count` words (the game's `rep movsd`).
fn copy_words(e: &mut Engine, from: u32, to: u32, count: u32) {
    for i in 0..count {
        let word = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, word);
    }
}

/// True when the integer setting at `011c3ea4` (read through `0043d4d0`)
/// is 1.
fn mode_setting_is_one(e: &mut Engine) -> bool {
    let mode = e.call(GLOBAL_WORD_ADDRESS, &args![MODE_SETTING]).u32();
    e.mem.u32(mode) == 1
}

// ---------------------------------------------------------------------
// Third block (b0148p2, second session): `0088b4e0` up to `00891d30`.
// ---------------------------------------------------------------------

/// Tells whether the actor's `cCurrentSitSleepState` (+0x1ac) equals 9
/// (`SETZ AL`); `RestoreHealth` passes it to the regeneration formula.
const SIT_SLEEP_STATE_IS_NINE: u32 = 0x0057_9670;
/// `ECX` = actor, `float` in ST0: the base value `RestoreHealth` feeds to the
/// health regeneration formula (the map has no name for it).
const HEALTH_BASE_VALUE: u32 = 0x008b_e6d0;
/// `cdecl`, a `float` and a flag word, `float` in ST0: the health
/// regeneration rate.
const HEALTH_REGENERATION_FORMULA: u32 = 0x0064_8b00;
/// `cdecl`, one `float`, `float` in ST0: the fatigue regeneration rate.
const FATIGUE_REGENERATION_FORMULA: u32 = 0x0064_8050;
/// `cdecl`, no argument, `float` in ST0: the action point regeneration rate.
const ACTION_POINT_REGENERATION_FORMULA: u32 = 0x0066_dda0;
/// `Actor::GetFatigue` (Xbox PDB), `float` in ST0.
const GET_FATIGUE: u32 = 0x0089_3500;
/// `ECX` = extra data list: the word at +0x0c of its extra data of type
/// `0x29`, or 0 without it.
const EXTRA_LIST_WORD_0X29: u32 = 0x0041_8250;
/// Table of words indexed by the number `0041ca90` returns for an object.
const OBJECT_TYPE_TABLE: u32 = 0x0119_bcb0;
/// Settings object whose float `00403e20` addresses; `0088b850` feeds the
/// float to `CombatFormulas::CalcWeaponReach`.
const WEAPON_REACH_SETTING: u32 = 0x011c_f1e0;
/// `CombatFormulas::CalcWeaponReach` (Xbox PDB), `cdecl`, one `float`,
/// `float` in ST0.
const CALC_WEAPON_REACH: u32 = 0x0064_63e0;
/// `ECX` = actor value number (stack: a mask), `bool` in `AL`: tells whether
/// the actor value has the flag mask (`RestoreActorValue` tests `0x200`).
/// Cdecl: index, mask.
const ACTOR_VALUE_HAS_FLAG_MASK: u32 = 0x0040_6d70;
/// Actor value number restored by `RestoreHealth`.
const RESTORED_VALUE_HEALTH: u32 = 0x10;
/// Actor value number restored by `RestoreFatigue`.
const RESTORED_VALUE_FATIGUE: u32 = 0x16;
/// Actor value number restored by `RestoreActionPoints`.
const RESTORED_VALUE_ACTION_POINTS: u32 = 0x0c;
/// The `Actor` virtual `RestoreActorValue` calls with (index, amount, 0).
const VSLOT_APPLY_VALUE_CHANGE: u32 = 0x3ac;
/// First argument of `HandleEntryPoint` for the health regeneration.
const ENTRY_POINT_HEALTH_REGENERATION: u32 = 0x0c;
/// First argument of `HandleEntryPoint` for the action point regeneration.
const ENTRY_POINT_ACTION_POINT_REGENERATION: u32 = 0x27;
/// Settings object whose int (`0044ddc0`) blocks the action point
/// regeneration while it is non-zero.
const ACTION_POINT_BLOCK_OBJECT: u32 = 0x011f_2250;
/// Virtual `+0x14` of the sub-object at `Actor + 0xa4`: the current value of
/// an actor value (the index is the stack word).
const OWNER_VSLOT_CURRENT_VALUE: u32 = 0x14;
/// Virtual `+0x04` of the sub-object at `Actor + 0xa4`: a `float` per actor
/// value index.
const OWNER_VSLOT_VALUE: u32 = 0x04;

// Translated from 0088b4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the `0x29` extra data word of the actor's extra data list (the
/// result is dropped) and returns what `008d8520` answers: the actor's
/// process (`+0x68`). The map has no name for it.
pub fn fn_0088b4e0(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    let list = e.call(GET_EXTRA_DATA_LIST_ADDRESS, &args![this]).u32();
    e.call(EXTRA_LIST_WORD_0X29, &args![list]);
    e.call(GET_PROCESS, &args![this]).u32()
}

// Translated from 0088b510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::RestoreHealth` (Xbox PDB): restores `amount` times the
/// regeneration rate, plus the health regeneration entry point value (also
/// scaled by `amount`) when the actor virtual `+0x360` holds.
pub fn actor_restore_health(e: &mut Engine, this: Ptr<Actor>, amount: f32) {
    let flag = e.call(SIT_SLEEP_STATE_IS_NINE, &args![this]).u8() as u32;
    let base = e.call(HEALTH_BASE_VALUE, &args![this]).f32();
    let rate = e
        .call(HEALTH_REGENERATION_FORMULA, &args![base, flag])
        .f32();
    let restored = (rate as f64 * amount as f64) as f32;
    let mut bonus = 0.0f32;
    if e.vcall(this.addr(), VSLOT_ACTOR_FLAG_0X360, &args![])
        .bool()
    {
        let player = e.mem.u32(PLAYER_CHARACTER);
        bonus = e.with_stack(4, |e, local| {
            e.mem.set_f32(local.addr(), 0.0);
            e.call(
                HANDLE_ENTRY_POINT,
                &args![ENTRY_POINT_HEALTH_REGENERATION, player, local],
            );
            e.mem.f32(local.addr())
        });
        bonus = (bonus as f64 * amount as f64) as f32;
    }
    actor_restore_actor_value(
        e,
        this,
        RESTORED_VALUE_HEALTH,
        (restored as f64 + bonus as f64) as f32,
    );
}

// Translated from 0088b5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::RestoreFatigue` (Xbox PDB): restores `amount` times the
/// endurance based rate; when that lifts a negative fatigue to zero or above
/// and the owner's current value of the fatigue actor value is still
/// negative, restores enough to bring it to one.
pub fn actor_restore_fatigue(e: &mut Engine, this: Ptr<Actor>, amount: f32) {
    let endurance = e.call(ACTOR_GET_ENDURANCE, &args![this]).f32();
    let rate = e
        .call(FATIGUE_REGENERATION_FORMULA, &args![endurance])
        .f32();
    let restored = (rate as f64 * amount as f64) as f32;
    let fatigue = e.call(GET_FATIGUE, &args![this]).f32();
    actor_restore_actor_value(e, this, RESTORED_VALUE_FATIGUE, restored);
    let after = (fatigue as f64 + restored as f64) as f32;
    let zero: f64 = e.global(DOUBLE_ZERO);
    if (fatigue as f64) < zero && (after as f64) >= zero {
        let owner = this.addr() + ACTOR_VALUE_OWNER_OFFSET;
        let current = e
            .vcall(
                owner,
                OWNER_VSLOT_CURRENT_VALUE,
                &args![RESTORED_VALUE_FATIGUE],
            )
            .f32();
        if (current as f64) < zero {
            let one: f64 = e.global(DOUBLE_ONE);
            let missing = (-(current as f64) + one) as f32;
            actor_restore_actor_value(e, this, RESTORED_VALUE_FATIGUE, missing);
        }
    }
}

// Translated from 0088b660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::RestoreActionPoints` (Xbox PDB): when the actor virtual `+0x360`
/// holds and the settings object `011f2250` reads zero, restores `amount`
/// times the regeneration rate, adjusted by the action point entry point
/// (with the item the process virtual `+0x148` yields, if any) and
/// multiplied by the owner's value for actor value `0x0c`.
pub fn actor_restore_action_points(e: &mut Engine, this: Ptr<Actor>, amount: f32) {
    if !e
        .vcall(this.addr(), VSLOT_ACTOR_FLAG_0X360, &args![])
        .bool()
    {
        return;
    }
    if e.call(ITEM_VALUE, &args![ACTION_POINT_BLOCK_OBJECT]).u32() != 0 {
        return;
    }
    let rate = e.call(ACTION_POINT_REGENERATION_FORMULA, &args![]).f32();
    let scaled = (rate as f64 * amount as f64) as f32;
    let mut item = 0u32;
    let process = e.get(this, Actor::pCurrentProcess);
    if !process.is_null() && e.vcall(process.addr(), PROCESS_VSLOT_ITEM, &args![]).u32() != 0 {
        let held = e.vcall(process.addr(), PROCESS_VSLOT_ITEM, &args![]).u32();
        item = e.call(ITEM_VALUE, &args![held]).u32();
    }
    let adjusted = e.with_stack(4, |e, local| {
        e.mem.set_f32(local.addr(), scaled);
        e.call(
            HANDLE_ENTRY_POINT,
            &args![ENTRY_POINT_ACTION_POINT_REGENERATION, this, item, local],
        );
        e.mem.f32(local.addr())
    });
    let owner = this.addr() + ACTOR_VALUE_OWNER_OFFSET;
    let multiplier = e
        .vcall(
            owner,
            OWNER_VSLOT_VALUE,
            &args![RESTORED_VALUE_ACTION_POINTS],
        )
        .f32();
    let result = (adjusted as f64 * multiplier as f64) as f32;
    actor_restore_actor_value(e, this, RESTORED_VALUE_ACTION_POINTS, result);
}

// Translated from 0088b740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::RestoreActorValue` (Xbox PDB): applies a positive `amount`
/// through the actor virtual `+0x3ac` when the owner's current value of
/// `index` is below zero; otherwise applies a negative `amount` when the
/// actor value `index` has the flag mask `0x200`.
pub fn actor_restore_actor_value(e: &mut Engine, this: Ptr<Actor>, index: u32, amount: f32) {
    let zero: f64 = e.global(DOUBLE_ZERO);
    if amount as f64 > zero {
        let owner = this.addr() + ACTOR_VALUE_OWNER_OFFSET;
        let current = e
            .vcall(owner, OWNER_VSLOT_CURRENT_VALUE, &args![index])
            .f32();
        if (current as f64) < zero {
            e.vcall(
                this.addr(),
                VSLOT_APPLY_VALUE_CHANGE,
                &args![index, amount, 0u32],
            );
            return;
        }
    }
    if e.call(ACTOR_VALUE_HAS_FLAG_MASK, &args![index, 0x200u32])
        .bool()
        && (amount as f64) < zero
    {
        e.vcall(
            this.addr(),
            VSLOT_APPLY_VALUE_CHANGE,
            &args![index, amount, 0u32],
        );
    }
}

// Translated from 0088b7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The entry of the table at `0119bcb0` for the object the process
/// virtual `+0x22c` returns, or 0 without a process or an object. The map
/// has no name for it.
pub fn fn_0088b7f0(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    let process = e.get(this, Actor::pCurrentProcess);
    if process.is_null() {
        return 0;
    }
    if e.vcall(process.addr(), PROCESS_VSLOT_OBJECT, &args![])
        .u32()
        == 0
    {
        return 0;
    }
    let object = e
        .vcall(process.addr(), PROCESS_VSLOT_OBJECT, &args![])
        .u32();
    let kind = e.call(OBJECT_TYPE, &args![object]).u32();
    e.mem
        .u32(OBJECT_TYPE_TABLE.wrapping_add(kind.wrapping_mul(4)))
}

// Translated from 0088b850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatFormulas::CalcWeaponReach` of the float in the settings object
/// `011cf1e0` (a `float` in ST0). `this` is not read. The map has no name
/// for it.
pub fn fn_0088b850(e: &mut Engine, _this: Ptr) -> f32 {
    let value_address = e
        .call(SETTING_VALUE_ADDRESS, &args![WEAPON_REACH_SETTING])
        .u32();
    let value = e.mem.f32(value_address);
    e.call(CALC_WEAPON_REACH, &args![value]).f32()
}

// ---- line of sight (0088b880, 0088c240, 0088c570, 0088c600) ----

/// `Actor` setting object whose float is the view cone width in degrees.
const VIEW_CONE_DEGREES_SETTING: u32 = 0x011c_d668;
/// Byte flag: when non-zero `LineOfSight` answers at once (and reports 2).
const LINE_OF_SIGHT_OVERRIDE_FLAG: u32 = 0x011d_f678;
/// Counter that `LineOfSight` increments once per ray.
const RAY_COUNTER: u32 = 0x011d_ea34;
/// `float`: the margin `LineOfSight` applies to the two corners of its box.
const RAY_BOX_MARGIN: u32 = 0x011d_f6bc;
/// `float`: the numerator of the distance scale the query gets.
const RAY_DISTANCE_NUMERATOR: u32 = 0x011a_32dc;
/// The three `float` fractions of the target height the rays aim at.
const RAY_HEIGHT_FRACTIONS: [u32; 3] = [0x0101_6264, 0x0101_6248, 0x0101_622c];
/// What `LineOfSight` writes to its result for the ray number 0, 1 and 2.
const RAY_RESULT_CODES: [u32; 3] = [2, 1, 0];
/// `double`: distance up to which `LineOfSight` answers true at once (2.0).
const SIGHT_CLOSE_DISTANCE: u32 = 0x0101_1590;
/// `double` factor applied to the height for the eye point (0.75).
const SIGHT_HEIGHT_FACTOR: u32 = 0x0101_de30;
/// `double`: the squared-length limit `0088c240` compares with (25.0).
const SIGHT_SQUARED_LIMIT: u32 = 0x0104_f2f0;
/// `double`: degrees to radians.
const DEGREES_TO_RADIANS_FACTOR: u32 = 0x0102_3128;
/// The collision layer number put into the ray filter.
const RAY_FILTER_LAYER: u32 = 0x25;

/// Reference virtual `+0xfc` (tested before the reference becomes the ray's
/// ignored object).
const REFERENCE_VSLOT_0XFC: u32 = 0xfc;
/// Base form virtual `+0x180`: non-zero when the base form has a process
/// based position (`Actor` process virtual `+0x6d4` is then asked).
const BASE_FORM_VSLOT_0X180: u32 = 0x180;
/// Process virtual `+0x2cc`: the process's own line of sight test
/// (actor, reference, 0, flag).
const PROCESS_VSLOT_LINE_OF_SIGHT: u32 = 0x2cc;
/// Process virtual `+0x6d4`: an object with a position (`0045bb80`).
const PROCESS_VSLOT_POSITION_OBJECT_A: u32 = 0x6d4;
/// Process virtual `+0x6d8`: a second such object.
const PROCESS_VSLOT_POSITION_OBJECT_B: u32 = 0x6d8;
/// `Actor` virtual `+0x4b4`: when true the ray query gets a distance scale.
const VSLOT_RAY_SCALE_FLAG: u32 = 0x4b4;
/// Process virtual `+0x504`: finds the entry `Has360LineOfSight` reads.
const PROCESS_VSLOT_0X504: u32 = 0x504;
/// Virtual `+0x94` of the cell's object, and the `+0x60` and `+0x68` of what
/// `006286d0` makes of its result.
const CELL_OBJECT_VSLOT_0X94: u32 = 0x94;
const COUNTED_OBJECT_VSLOT_COUNT: u32 = 0x60;
const COUNTED_OBJECT_VSLOT_FILL: u32 = 0x68;

/// `ECX` = the reference: the address of its position (`this + 0x30`).
/// (Already named `POSITION_VECTOR_OF` above.)
/// `ECX` = an object: the address of its position (`this + 0x8c`).
const POSITION_OF_OBJECT: u32 = 0x0045_bb80;
/// `ECX` = actor: its base form (`BGSSaveFormBuffer::GetForm` folded into
/// `004181e0`).
const BASE_FORM_OF: u32 = 0x0041_81e0;
/// `ECX` = vector storage: returns `ECX` (constructs nothing).
const VECTOR_CONSTRUCT: u32 = 0x0068_15c0;
/// `cdecl`, destination and source `float[3]`: copies three floats and
/// clears the fourth.
const VECTOR_COPY: u32 = 0x0055_3fc0;
/// `ECX` = vector, one stack word: the address of its component.
const VECTOR_COMPONENT_ADDRESS: u32 = 0x0056_0d30;
/// `ECX` = vector: its squared length, a `float` in ST0.
const VECTOR_LENGTH_SQUARED: u32 = 0x004a_7290;
/// `GetZAngleFromVector` (map name), `cdecl`, one vector pointer, `float`
/// in ST0.
const GET_Z_ANGLE_FROM_VECTOR: u32 = 0x004b_13c0;
/// `cdecl` (`float` a, `float` b, `float*` flag), `float` in ST0: the angle
/// difference `b - a` brought into range, with a flag stored through the
/// pointer.
const ANGLE_DIFFERENCE: u32 = 0x004b_15e0;
/// `cdecl`, one `float`, `float` in ST0: wraps the C runtime function
/// `00ec6cde` (the use here makes it the magnitude).
const FLOAT_MAGNITUDE: u32 = 0x0040_8840;
/// `ECX` = box (32 bytes), stack (point, point): sets min and max to the
/// two points.
const BOX_INIT: u32 = 0x0062_8950;
/// `ECX` = destination, stack (a, b): per component minimum.
const BOX_MIN: u32 = 0x0062_85e0;
/// `ECX` = destination, stack (a, b): per component maximum.
const BOX_MAX: u32 = 0x0062_8640;
/// `ECX` = vector, one `float` on the stack: all four lanes from the float
/// and zeros.
const VECTOR_FROM_FLOAT: u32 = 0x005d_bf20;
/// `ECX` = vector, one vector on the stack: lane-wise multiplication.
const VECTOR_MULTIPLY: u32 = 0x0062_7920;
/// `ECX` = the cell's object (`004543c0`), checked before and after use.
const CELL_OBJECT_TOUCH: u32 = 0x0062_19c0;
/// `ECX` = cell: its object (interior or exterior).
const CELL_OBJECT_OF: u32 = 0x0045_43c0;
/// `ECX` = object: its word at +0x58 (the map names the folded body
/// `LowProcess::GetNumberOfItemsActivated`).
const OBJECT_WORD_0X58: u32 = 0x0062_86d0;
/// `cdecl` (count, 0): a block for `count` entries.
const ALLOCATE_ENTRIES: u32 = 0x006e_7620;
/// `cdecl`, one block: frees what `006e7620` returned.
const FREE_ENTRIES: u32 = 0x005e_0b60;
/// `ECX` = ray pick data, one stack word: stores it at +0xa0.
const PICK_SET_ENTRIES: u32 = 0x0050_0a20;
/// `ECX` = ray pick data (0xb0 bytes): constructor.
const PICK_CONSTRUCT: u32 = 0x004a_3c20;
/// `ECX` = ray query (0x80 bytes), stack (0x25, reference): constructor.
const QUERY_CONSTRUCT: u32 = 0x0062_a190;
/// `ECX` = ray query, no stack: destructor.
const QUERY_DESTRUCT: u32 = 0x0059_cee0;
/// `ECX` = pick data, one stack word: stores the query (+0xa4).
const PICK_SET_QUERY: u32 = 0x0059_ceb0;
/// `ECX` = pick data, one stack word (vector): sets the ray start.
const PICK_SET_START: u32 = 0x004a_3da0;
/// `ECX` = pick data, one stack word (vector): sets the ray end.
const PICK_SET_END: u32 = 0x004a_3eb0;
/// `ECX` = pick data, one stack word: stores the filter word at +0x24.
const PICK_SET_FILTER: u32 = 0x004a_3f70;
/// `ECX` = pick data: the byte at +0xac, true when the ray was blocked.
const PICK_BLOCKED: u32 = 0x0063_2ce0;
/// `ECX` = pick data, one stack word (vector): fills the vector from the
/// pick result.
const PICK_RESULT_POINT: u32 = 0x005d_be60;
/// `ECX` = actor, one stack word (out): the filter word of its character
/// controller.
const CHAR_CONTROLLER_FILTER: u32 = 0x0093_1ed0;
/// `ECX` = word storage, one stack word: stores it (`std::locale::id`).
const STORE_WORD: u32 = 0x008c_71b0;
/// `ECX` = filter word storage, one stack word: replaces the low 7 bits.
const FILTER_SET_LAYER: u32 = 0x004a_39f0;
/// `ECX` = filter word: its high 16 bits.
const FILTER_GET_GROUP: u32 = 0x004a_3a20;
/// `ECX` = filter word storage, one stack word: replaces the high 16 bits.
const FILTER_SET_GROUP: u32 = 0x0059_ce80;
/// `ECX` = ray query, one `float` stack word: stores it at +0x74.
const QUERY_SET_DISTANCE_SCALE: u32 = 0x006d_3b00;
/// `TES::Pick` (Xbox PDB): `ECX` = the `TES` object, one stack word (the
/// pick data); returns the 3D object hit, or 0.
const TES_PICK: u32 = 0x0045_8420;
/// `TESObjectREFR::FindReferenceFor3D` (Xbox PDB), `cdecl`, one 3D object.
const FIND_REFERENCE_FOR_3D: u32 = 0x0056_f930;

/// Starts the sight origin: `origin` (a vector) becomes the position of the
/// object the process offers (`Actor` process virtual `+0x6d4`, only for
/// actors whose base form virtual `+0x180` answers non-zero and that are not
/// the player), else the actor's own position raised by its eye point
/// (`SneakHeight` when the sneak test holds; otherwise the eye level or
/// three quarters of the height). Returns the object, or 0. Shared by
/// `0088b880` and `0088c240`.
fn sight_origin(e: &mut Engine, this: Ptr<Actor>, eye_level: bool, origin: u32) -> u32 {
    e.call(VECTOR_CONSTRUCT, &args![origin]);
    let mut object = 0u32;
    let base = e.call(BASE_FORM_OF, &args![this]).u32();
    let has_process_position = e.vcall(base, BASE_FORM_VSLOT_0X180, &args![]).u32() != 0;
    if has_process_position && this.addr() != e.mem.u32(PLAYER_CHARACTER) {
        let process = e.mem.u32(this.addr() + 0x68);
        object = e
            .vcall(process, PROCESS_VSLOT_POSITION_OBJECT_A, &args![])
            .u32();
    }
    let source = if object != 0 {
        e.call(POSITION_OF_OBJECT, &args![object]).u32()
    } else {
        e.call(POSITION_VECTOR_OF, &args![this]).u32()
    };
    for word in 0..3 {
        let value = e.mem.u32(source + 4 * word);
        e.mem.set_u32(origin + 4 * word, value);
    }
    let height = if e.call(SPEED_FLAG_TEST, &args![this]).bool() {
        actor_get_sneak_height(e, this, eye_level)
    } else if eye_level {
        e.call(GET_EYE_LEVEL, &args![this]).f32()
    } else {
        let factor: f64 = e.global(SIGHT_HEIGHT_FACTOR);
        (actor_get_height(e, this) as f64 * factor) as f32
    };
    if object == 0 {
        let z = e.mem.f32(origin + 8);
        e.mem.set_f32(origin + 8, (z as f64 + height as f64) as f32);
    }
    object
}

/// Puts the ray filter word together at `filter` (`std::locale::id` storage
/// in the game's code): the layer `0x25` in the low bits, the group the
/// actor's character controller reports in the high 16 bits.
fn build_ray_filter(e: &mut Engine, this: Ptr<Actor>, controller_word: u32, filter: u32) -> u32 {
    e.call(CHAR_CONTROLLER_FILTER, &args![this, controller_word]);
    e.call(STORE_WORD, &args![filter, 0u32]);
    e.call(FILTER_SET_LAYER, &args![filter, RAY_FILTER_LAYER]);
    let group = e.call(FILTER_GET_GROUP, &args![controller_word]).u32();
    e.call(FILTER_SET_GROUP, &args![filter, group]);
    e.mem.u32(filter)
}

// Translated from 0088b880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::LineOfSight` (Xbox PDB): whether the actor sees `target`. The
/// stack parameters are (`cast_ray`, `target`, `eye_level`, `result_code`,
/// `ignore_view_cone`); names of the flags are by use. Answers false without
/// a target with 3D; true when the target is within the close distance;
/// without `cast_ray` it asks the process (virtual `+0x2cc`); otherwise it
/// checks the view cone (unless ignored), the override flag (`result_code`
/// is then 2), and casts up to three rays from the sight origin at heights
/// of the target (`result_code` receives 3 first, then the code of the ray
/// that was clear: 2, 1 or 0). The SEH frame is not translated.
pub fn actor_line_of_sight(
    e: &mut Engine,
    this: Ptr<Actor>,
    cast_ray: bool,
    target: Ptr,
    eye_level: bool,
    result_code: Ptr,
    ignore_view_cone: bool,
) -> bool {
    if target.is_null() {
        return false;
    }
    if e.vcall(target.addr(), VSLOT_GET_3D, &args![]).u32() == 0 {
        return false;
    }
    e.call(ACTOR_GET_IN_COMBAT, &args![this]);
    if e.vcall(target.addr(), VSLOT_IS_PROCESS_FLAG_0X22C, &args![0u32])
        .bool()
        && !e
            .vcall(this.addr(), VSLOT_ACTOR_FLAG_0X360, &args![])
            .bool()
        && !cast_ray
    {
        return false;
    }
    let reference = if e.vcall(target.addr(), VSLOT_IS_ACTOR, &args![]).bool() {
        target.addr()
    } else {
        0
    };
    let distance = e
        .call(
            GET_DISTANCE_FROM_REFERENCE,
            &args![this, target, 0u32, 0u32],
        )
        .f32();
    let close: f64 = e.global(SIGHT_CLOSE_DISTANCE);
    if distance as f64 <= close {
        // The game also sets its own copy of the last stack flag to 1 here.
        return true;
    }
    if !cast_ray {
        let process = e.get(this, Actor::pCurrentProcess);
        if process.is_null() {
            return false;
        }
        return e
            .vcall(
                process.addr(),
                PROCESS_VSLOT_LINE_OF_SIGHT,
                &args![this, reference, 0u32, ignore_view_cone],
            )
            .bool();
    }
    if !result_code.is_null() {
        e.mem.set_u32(result_code.addr(), 3);
    }
    if !ignore_view_cone {
        let degrees_address = e
            .call(SETTING_VALUE_ADDRESS, &args![VIEW_CONE_DEGREES_SETTING])
            .u32();
        let to_radians: f64 = e.global(DEGREES_TO_RADIANS_FACTOR);
        let cone = (e.mem.f32(degrees_address) as f64 * to_radians) as f32;
        let position = e.call(POSITION_VECTOR_OF, &args![target]).u32();
        if !actor_is_point_in_view_cone(e, this, Ptr::new(position), cone) {
            return false;
        }
    }
    if e.mem.u8(LINE_OF_SIGHT_OVERRIDE_FLAG) != 0 {
        if !result_code.is_null() {
            e.mem.set_u32(result_code.addr(), 2);
        }
        return true;
    }
    let cell = e.call(GET_PARENT_CELL, &args![this]).u32();
    if cell == 0 {
        return false;
    }
    // The game's frame holds the pick data, the query, the box and the
    // vectors; 0x2c0 bytes cover every slot (the offsets below are the
    // distances under its frame pointer).
    e.with_stack(0x2c0, |e, frame| {
        let frame_pointer = frame.addr() + 0x2b0;
        line_of_sight_rays(
            e,
            frame_pointer,
            this,
            target,
            reference,
            eye_level,
            result_code,
            cell,
        )
    })
}

/// The ray part of `LineOfSight`; `frame_pointer` is the end of the scratch
/// frame the game's function keeps on its stack.
#[allow(clippy::too_many_arguments)]
fn line_of_sight_rays(
    e: &mut Engine,
    frame_pointer: u32,
    this: Ptr<Actor>,
    target: Ptr,
    reference: u32,
    eye_level: bool,
    result_code: Ptr,
    cell: u32,
) -> bool {
    let under = |distance: u32| frame_pointer - distance;
    let start = under(0x38);
    let pick = under(0x100);
    let query = under(0x190);
    let end = under(0x19c);
    let filter_source = under(0x1a0);
    let filter = under(0x1a4);
    let point = under(0x220);
    let box_min = under(0x240);
    let box_max = under(0x230);
    let player = e.mem.u32(PLAYER_CHARACTER);
    let tes = e.mem.u32(TES_POINTER);

    sight_origin(e, this, eye_level, start);
    if !e.call(TES_IS_CELL_LOADED, &args![tes, cell, 1u32]).bool() {
        return false;
    }
    e.call(PICK_CONSTRUCT, &args![pick]);
    let ignored = if !target.is_null()
        && e.vcall(target.addr(), REFERENCE_VSLOT_0XFC, &args![])
            .bool()
    {
        target.addr()
    } else {
        0
    };
    e.call(QUERY_CONSTRUCT, &args![query, RAY_FILTER_LAYER, ignored]);
    e.call(PICK_SET_QUERY, &args![pick, query]);
    let target_position = e.call(POSITION_VECTOR_OF, &args![target]).u32();
    for word in 0..3 {
        let value = e.mem.u32(target_position + 4 * word);
        e.mem.set_u32(end + 4 * word, value);
    }
    e.call(PICK_SET_START, &args![pick, start]);
    let filter_word = build_ray_filter(e, this, filter_source, filter);
    e.call(PICK_SET_FILTER, &args![pick, filter_word]);

    let scale = e.call(GET_SCALE, &args![target]).f32() as f64;
    let upper = e
        .vcall(target.addr(), VSLOT_BOUNDS_UPPER, &args![under(0x1b0)])
        .u32();
    let lower = e
        .vcall(target.addr(), VSLOT_BOUNDS_LOWER, &args![under(0x1c4)])
        .u32();
    let extent = ((e.mem.f32(upper + 8) as f64 - e.mem.f32(lower + 8) as f64) * scale) as f32;
    let fractions = RAY_HEIGHT_FRACTIONS.map(|address| e.mem.f32(address));
    let mut aim_objects = [0u32; 3];
    if reference != 0 {
        let base = e.call(BASE_FORM_OF, &args![reference]).u32();
        if e.vcall(base, BASE_FORM_VSLOT_0X180, &args![]).u32() != 0 {
            let process = e.call(GET_PROCESS, &args![reference]).u32();
            aim_objects[0] = e
                .vcall(process, PROCESS_VSLOT_POSITION_OBJECT_A, &args![])
                .u32();
            let process = e.call(GET_PROCESS, &args![reference]).u32();
            aim_objects[1] = e
                .vcall(process, PROCESS_VSLOT_POSITION_OBJECT_B, &args![])
                .u32();
        }
    }
    let base_height = e.mem.f32(end + 8);
    let mut rays = 1;
    e.vcall(this.addr(), VSLOT_GET_COMBAT_CONTROLLER, &args![]);
    if (reference != 0 && reference == player)
        || reference == e.mem.u32(this.addr() + 0x128)
        || this.addr() == player
    {
        rays = 3;
    }
    let mut entries = 0u32;
    let cell_object = e.call(CELL_OBJECT_OF, &args![cell]).u32();
    e.call(VECTOR_CONSTRUCT, &args![point]);
    e.call(VECTOR_COPY, &args![point, start]);
    e.call(BOX_INIT, &args![box_min, point, point]);
    for ray in 0..rays {
        if aim_objects[ray] != 0 {
            let position = e.call(POSITION_OF_OBJECT, &args![aim_objects[ray]]).u32();
            e.call(VECTOR_COPY, &args![point, position]);
        } else {
            e.call(VECTOR_COPY, &args![point, end]);
            let height = (extent as f64 * fractions[ray] as f64 + base_height as f64) as f32;
            let component = e.call(VECTOR_COMPONENT_ADDRESS, &args![point, 2u32]).u32();
            e.mem.set_f32(component, height);
        }
        e.call(BOX_MIN, &args![box_min, box_min, point]);
        e.call(BOX_MAX, &args![box_max, box_max, point]);
    }
    let margin: f32 = e.global(RAY_BOX_MARGIN);
    e.call(VECTOR_FROM_FLOAT, &args![under(0x260), margin]);
    e.call(VECTOR_MULTIPLY, &args![box_min, under(0x260)]);
    e.call(VECTOR_FROM_FLOAT, &args![under(0x270), margin]);
    e.call(VECTOR_MULTIPLY, &args![box_max, under(0x270)]);
    e.call(CELL_OBJECT_TOUCH, &args![cell_object]);
    let counted = e.vcall(cell_object, CELL_OBJECT_VSLOT_0X94, &args![]).u32();
    let counted = e.call(OBJECT_WORD_0X58, &args![counted]).u32();
    let count = e.vcall(counted, COUNTED_OBJECT_VSLOT_COUNT, &args![]).i32();
    if count > 0 {
        entries = e.call(ALLOCATE_ENTRIES, &args![count as u32, 0u32]).u32();
        if entries != 0 {
            let counted = e.vcall(cell_object, CELL_OBJECT_VSLOT_0X94, &args![]).u32();
            let counted = e.call(OBJECT_WORD_0X58, &args![counted]).u32();
            e.vcall(counted, COUNTED_OBJECT_VSLOT_FILL, &args![box_min, entries]);
            e.call(PICK_SET_ENTRIES, &args![pick, entries]);
        }
    }
    e.call(CELL_OBJECT_TOUCH, &args![cell_object]);
    let mut seen = false;
    for ray in 0..rays {
        if aim_objects[ray] != 0 {
            let position = e.call(POSITION_OF_OBJECT, &args![aim_objects[ray]]).u32();
            e.call(PICK_SET_END, &args![pick, position]);
        } else {
            let height = (extent as f64 * fractions[ray] as f64 + base_height as f64) as f32;
            e.mem.set_f32(end + 8, height);
            e.call(PICK_SET_END, &args![pick, end]);
        }
        let counter = e.mem.u32(RAY_COUNTER);
        e.mem.set_u32(RAY_COUNTER, counter.wrapping_add(1));
        if e.vcall(this.addr(), VSLOT_RAY_SCALE_FLAG, &args![]).bool() {
            let delta = e
                .call(VECTOR_MINUS_VECTOR, &args![end, under(0x28c), start])
                .u32();
            let length = e.call(VECTOR_LENGTH, &args![delta]).f32();
            let numerator: f32 = e.global(RAY_DISTANCE_NUMERATOR);
            let scale = (numerator as f64 / length as f64) as f32;
            e.call(QUERY_SET_DISTANCE_SCALE, &args![query, scale]);
        } else {
            e.call(QUERY_SET_DISTANCE_SCALE, &args![query, 0.0f32]);
        }
        let hit = e.call(TES_PICK, &args![tes, pick]).u32();
        let hit_reference = if hit != 0 {
            e.call(FIND_REFERENCE_FOR_3D, &args![hit]).u32()
        } else {
            0
        };
        if !e.call(PICK_BLOCKED, &args![pick]).bool()
            && (hit == 0 || hit_reference == target.addr())
        {
            seen = true;
            if !result_code.is_null() {
                e.mem.set_u32(result_code.addr(), RAY_RESULT_CODES[ray]);
            }
            break;
        }
    }
    if entries != 0 {
        e.call(FREE_ENTRIES, &args![entries]);
    }
    e.call(QUERY_DESTRUCT, &args![query]);
    seen
}

// Translated from 0088c240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the actor sees the point `point`: the view cone first (unless
/// `ignore_view_cone`), then one ray from the sight origin to the point in
/// the actor's cell; true when the ray hits nothing, or when what it hits
/// is closer than 5 units (squared length below 25) from the hit point.
/// The map has no name for it; the SEH frame is not translated.
pub fn fn_0088c240(
    e: &mut Engine,
    this: Ptr<Actor>,
    point: Ptr,
    eye_level: bool,
    ignore_view_cone: bool,
) -> bool {
    if !ignore_view_cone {
        let degrees_address = e
            .call(SETTING_VALUE_ADDRESS, &args![VIEW_CONE_DEGREES_SETTING])
            .u32();
        let to_radians: f64 = e.global(DEGREES_TO_RADIANS_FACTOR);
        let cone = (e.mem.f32(degrees_address) as f64 * to_radians) as f32;
        if !actor_is_point_in_view_cone(e, this, point, cone) {
            return false;
        }
    }
    let cell = e.call(GET_PARENT_CELL, &args![this]).u32();
    if cell == 0 {
        return false;
    }
    e.with_stack(0x1c0, |e, frame| {
        let frame_pointer = frame.addr() + 0x1b0;
        let under = |distance: u32| frame_pointer - distance;
        let start = under(0x2c);
        let pick = under(0x100);
        let query = under(0x180);
        let tes = e.mem.u32(TES_POINTER);
        sight_origin(e, this, eye_level, start);
        if !e.call(TES_IS_CELL_LOADED, &args![tes, cell, 1u32]).bool() {
            return false;
        }
        e.call(PICK_CONSTRUCT, &args![pick]);
        e.call(QUERY_CONSTRUCT, &args![query, RAY_FILTER_LAYER, this]);
        e.call(PICK_SET_QUERY, &args![pick, query]);
        e.call(PICK_SET_START, &args![pick, start]);
        e.call(PICK_SET_END, &args![pick, point]);
        let filter_word = build_ray_filter(e, this, under(0x184), under(0x188));
        e.call(PICK_SET_FILTER, &args![pick, filter_word]);
        let hit = e.call(TES_PICK, &args![tes, pick]).u32();
        let seen = if hit == 0 {
            true
        } else {
            let hit_point = under(0x19c);
            let difference = under(0x1a8);
            e.call(VECTOR_CONSTRUCT, &args![hit_point]);
            e.call(PICK_RESULT_POINT, &args![pick, hit_point]);
            e.call(VECTOR_MINUS_VECTOR, &args![hit_point, difference, point]);
            let squared = e.call(VECTOR_LENGTH_SQUARED, &args![difference]).f32();
            let limit: f64 = e.global(SIGHT_SQUARED_LIMIT);
            (squared as f64) < limit
        };
        e.call(QUERY_DESTRUCT, &args![query]);
        seen
    })
}

// Translated from 0088c570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::IsPointInViewCone` (Xbox PDB): true when the angle between the
/// actor's heading (virtual `+0x2bc`) and the direction to `point` is less
/// than half of `cone` (radians).
pub fn actor_is_point_in_view_cone(
    e: &mut Engine,
    this: Ptr<Actor>,
    point: Ptr,
    cone: f32,
) -> bool {
    let this_position = e.call(POSITION_VECTOR_OF, &args![this]).u32();
    e.with_stack(0x20, |e, scratch| {
        let direction = scratch.addr();
        let flag = scratch.addr() + 0x10;
        e.call(VECTOR_MINUS_VECTOR, &args![point, direction, this_position]);
        let angle = e.call(GET_Z_ANGLE_FROM_VECTOR, &args![direction]).f32();
        let heading = e
            .vcall(this.addr(), VSLOT_GET_HEADING_0X2BC, &args![0u32])
            .f32();
        let difference = e.call(ANGLE_DIFFERENCE, &args![heading, angle, flag]).f32();
        let magnitude = e.call(FLOAT_MAGNITUDE, &args![difference]).f32();
        let half: f64 = e.global(DOUBLE_HALF);
        cone as f64 * half > magnitude as f64
    })
}

// Translated from 0088c600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::Has360LineOfSight` (Xbox PDB): the byte at +0x1c of the entry
/// the process virtual `+0x504` returns for `argument`; false without a
/// process or an entry.
pub fn actor_has_360_line_of_sight(e: &mut Engine, this: Ptr<Actor>, argument: u32) -> u8 {
    let process = e.get(this, Actor::pCurrentProcess);
    if process.is_null() {
        return 0;
    }
    let entry = e
        .vcall(process.addr(), PROCESS_VSLOT_0X504, &args![argument, 0u32])
        .u32();
    if entry == 0 {
        return 0;
    }
    e.mem.u8(entry + 0x1c)
}

// ---- equipping (0088c650 up to 0088e1e0) ----

/// `Script::SetActionFlag` (Xbox PDB), `cdecl`: (object, extra data list,
/// flag number).
const SET_ACTION_FLAG: u32 = 0x005a_c750;
/// `cdecl` with `ECX` = a message setting object, stack (0, icon path or 0,
/// 0, `float` duration, 0): builds a message and returns it.
const CREATE_MESSAGE: u32 = 0x0040_3df0;
/// The same shape as [`CREATE_MESSAGE`], used by `UnEquipObject`.
const CREATE_MESSAGE_ALT: u32 = 0x004c_69f0;
/// `cdecl`, six words: shows the message `CREATE_MESSAGE` returned.
const SHOW_MESSAGE: u32 = 0x0070_52f0;
/// The icon path `Interface\Icons\Message Icons\glow_message_vaultboy_sad.dds`.
const MESSAGE_ICON_PATH: u32 = 0x0102_08a0;
/// `float` (2.0 per the decompiler): the message duration.
const MESSAGE_DURATION: u32 = 0x0101_62c0;
/// Message setting object of the power armor refusal.
const POWER_ARMOR_MESSAGE: u32 = 0x011d_3540;
/// Message setting object shown for items that cannot be worn and for
/// unsupported types.
const CANNOT_WEAR_MESSAGE: u32 = 0x011d_42ac;
/// Message setting object shown for a broken item.
const BROKEN_ITEM_MESSAGE: u32 = 0x011d_213c;
/// Message setting object shown when the player cannot use a consumable.
const CANNOT_USE_MESSAGE: u32 = 0x011d_39fc;
/// Message setting object shown (without icon) when the equip step failed.
const EQUIP_FAILED_MESSAGE: u32 = 0x011d_201c;
/// Message setting object shown when an item cannot be taken off.
const CANNOT_REMOVE_MESSAGE: u32 = 0x011d_3864;
/// `ExtraDataList::GetCanNotWear` (Xbox PDB): `ECX` = the list.
const EXTRA_LIST_CAN_NOT_WEAR: u32 = 0x0041_8b10;
/// `ExtraDataList::GetWorn` (Xbox PDB): `ECX` = the list, one stack word.
const EXTRA_LIST_GET_WORN: u32 = 0x0041_8ab0;
/// `ExtraDataList::GetContainerChanges` (Xbox PDB): `ECX` = the list.
const EXTRA_LIST_GET_CONTAINER_CHANGES: u32 = 0x0041_8520;
/// `ExtraDataList::GetWeaponModSlotActive` (Xbox PDB): `ECX` = the list,
/// one stack word.
const EXTRA_LIST_WEAPON_MOD_SLOT_ACTIVE: u32 = 0x0041_8c00;
/// `BaseExtraList::GetExtraData` (Xbox PDB): `ECX` = the list, one stack
/// word (the extra data type).
const BASE_EXTRA_GET_DATA: u32 = 0x0041_0220;
/// `InventoryChanges::GetInventoryChanges` (Xbox PDB), `cdecl`, the actor.
const INVENTORY_CHANGES_OF: u32 = 0x004b_f220;
/// `InventoryChanges::GetObjectCount` (Xbox PDB): `ECX` = the changes, one
/// stack word (the form).
const INVENTORY_OBJECT_COUNT: u32 = 0x004c_8f30;
/// `InventoryChanges::GetWornItem` (Xbox PDB): `ECX` = the changes, stack
/// (slot, 0).
const INVENTORY_WORN_ITEM: u32 = 0x004c_8c10;
/// `TESBipedModelForm::GetFormAsBipedModel` (Xbox PDB), `cdecl`, the form.
const BIPED_MODEL_OF: u32 = 0x0048_0db0;
/// `TESBipedModelForm::IsPowerArmor` (Xbox PDB): `ECX` = the biped model.
const BIPED_IS_POWER_ARMOR: u32 = 0x0048_0d10;
/// `TESBipedModelForm::FillsBipedSlot` (Xbox PDB): `ECX` = the biped model,
/// stack (slot, 0, 0).
const BIPED_FILLS_SLOT: u32 = 0x0048_0af0;
/// Offset of the biped model inside an armor form (`ECX = form + 0x70`).
const ARMOR_BIPED_MODEL_OFFSET: u32 = 0x70;
/// `TESHealthForm::GetFormHealth` (Xbox PDB), `cdecl`, the form.
const FORM_HEALTH: u32 = 0x0048_73d0;
/// `ECX` = form: a flag test (it keeps `EquipObject` from cutting a stack of
/// weapons to one; `DamageEquipment` applies it to the weapon).
const FORM_FLAG_TEST_4C0BF0: u32 = 0x004c_0bf0;
/// `ItemChange::HasModEffectActive_ov2` (Xbox PDB): `ECX` = the item, one
/// stack word.
const ITEM_HAS_MOD_EFFECT_ACTIVE: u32 = 0x004b_da70;
/// `TESObjectWEAP::GetFormClipRounds` (Xbox PDB): `ECX` = the form, one
/// stack word.
const FORM_CLIP_ROUNDS: u32 = 0x004f_e160;
/// `cdecl` (clip rounds, count): the count adjusted for the clip size.
const COUNT_FOR_CLIPS: u32 = 0x004a_8f20;
/// No arguments: a number; above zero `EquipObject` notifies `008ce180`.
const AMMO_NOTIFY_LEVEL: u32 = 0x0057_0f60;
/// `cdecl` (form, count, actor, 1, 1): ammo notification.
const AMMO_NOTIFY: u32 = 0x008c_e180;
/// `ItemChange::ItemChange` (Xbox PDB): `ECX` = storage, stack (form,
/// count).
const ITEM_CHANGE_CONSTRUCT: u32 = 0x004b_c550;
/// Actor virtual `+0x3c4`: called after an item is equipped or removed.
const VSLOT_AFTER_EQUIP_CHANGE: u32 = 0x3c4;
/// Process virtual `+0x168`: takes the equipped `ItemChange` (or 0).
const PROCESS_VSLOT_SET_EQUIPPED_CHANGE: u32 = 0x168;
/// Actor virtual `+0x17c`: takes (form, extra list, 1, 0, 0, 0, 0, 0, 1, 0);
/// `EquipObject` calls it for a book that can be read and `DamageEquipment`
/// for a weapon of the explosive kind.
const VSLOT_USE_ITEM_0X17C: u32 = 0x17c;
/// Process virtual `+0x178`: queues an equip or a removal (actor, equip
/// flag, form, count, extra list, three flags, three words, flag).
const PROCESS_VSLOT_QUEUE_EQUIP: u32 = 0x178;
/// Process virtual `+0x458`: (actor, 0), used when nothing is held.
const PROCESS_VSLOT_0X458: u32 = 0x458;
/// Process virtual `+0x14c`: the second held item (ammo).
const PROCESS_VSLOT_AMMO_ITEM: u32 = 0x14c;
/// Process virtual `+0x3e4`: a state number (7 makes `UnEquipObject` clear
/// the block).
const PROCESS_VSLOT_STATE: u32 = 0x3e4;
/// Actor virtual `+0x3ec`: re-equips a weapon (form, 0, flag, 1).
const VSLOT_EQUIP_WEAPON_STEP: u32 = 0x3ec;
/// Actor virtual `+0x390`.
const VSLOT_0X390: u32 = 0x390;
/// Actor virtual `+0x218`.
const VSLOT_TEST_0X218: u32 = 0x218;
/// Actor virtual `+0x384`: (1, `float`).
const VSLOT_0X384: u32 = 0x384;
/// Actor virtual `+0x1e8`: the actor's biped data.
const VSLOT_GET_BIPED: u32 = 0x1e8;
/// Form virtual `+0x94`: a test the eating and ammo code makes first.
const FORM_VSLOT_0X94: u32 = 0x94;
/// `PlayerCharacter::GetBiped` (Xbox PDB): `ECX` = player, one stack word.
const PLAYER_GET_BIPED: u32 = 0x0095_0b00;
/// `ECX` = the biped data, one stack word: the entry of slot `n`
/// (`this + 0x2c + 16 * n`).
const BIPED_SLOT_ENTRY: u32 = 0x0088_e0f0;
/// `__RTDynamicCast` (CRT), `cdecl`: (object, 0, source type, target type,
/// 0).
const RT_DYNAMIC_CAST_FN: u32 = 0x00ec_43fb;
/// The two run-time type descriptors `fn_0088db20` casts between.
const CAST_SOURCE_TYPE: u32 = 0x0118_3028;
const CAST_TARGET_TYPE: u32 = 0x0118_3978;
/// `ECX` = the biped data, stack (slot, 1, 0): clears a slot.
const BIPED_CLEAR_SLOT: u32 = 0x004a_aff0;
/// `ECX` = the worn entry: its first word.
const WORN_FIRST_WORD: u32 = 0x0055_9450;
/// `ECX` = form, one stack word (a slot bit): a number the weapon mod
/// code compares with 2.
const FORM_WEAPON_MOD_KIND: u32 = 0x004b_d880;
/// `Actor::ReloadTargets` (Xbox PDB): `ECX` = actor, one stack word.
const ACTOR_RELOAD_TARGETS: u32 = 0x008b_0b00;
/// `ECX` = changes, stack (form, count, actor, extra list, 0, flag):
/// applies the equip to the inventory changes (RET 0x18).
const INVENTORY_APPLY_EQUIP: u32 = 0x004b_ffe0;
/// `TESNPC::InitWornObject` (Xbox PDB): `ECX` = the base form, stack
/// (actor, biped data, item).
const NPC_INIT_WORN_OBJECT: u32 = 0x0060_61b0;
/// `TESEnchantableForm::GetFormEnchanting` (Xbox PDB), `cdecl`, the form.
const FORM_ENCHANTING: u32 = 0x004b_e330;
/// `MagicItem::Preload` (Xbox PDB): `ECX` = enchantment + 0x18, one stack
/// word.
const MAGIC_ITEM_PRELOAD: u32 = 0x0040_a420;
/// `MagicItem::Unload` (Xbox PDB): `ECX` = enchantment + 0x18, one stack
/// word.
const MAGIC_ITEM_UNLOAD: u32 = 0x0040_b800;
/// No arguments, `bool`: a game condition the refraction update needs.
const REFRACTION_CONDITION: u32 = 0x005b_9b00;
/// `Actor::IsRefractive` (Xbox PDB): `ECX` = actor.
const ACTOR_IS_REFRACTIVE: u32 = 0x008c_51c0;
/// `ExtraDataList::GetRefractionPropertyExtra` (Xbox PDB): `ECX` = list.
const EXTRA_LIST_REFRACTION: u32 = 0x0042_2820;
/// `ECX` = actor + 0x94, stack (form, 1): drops the effects of the form.
const MAGIC_TARGET_REMOVE_EFFECTS: u32 = 0x0082_48e0;
/// `MagicTarget::UpdateTarget` (Xbox PDB): `ECX` = player + 0x94, one
/// `float` stack word.
const MAGIC_TARGET_UPDATE: u32 = 0x0082_3c40;
/// `PlayerCharacter::RemoveQueuedEnchantment` (Xbox PDB): `ECX` = player,
/// stack (form, extra list).
const PLAYER_REMOVE_QUEUED_ENCHANTMENT: u32 = 0x0095_eb10;
/// `ECX` = player, stack (form, extra list): the step after a worn item
/// was equipped by the player.
const PLAYER_NOTIFY_EQUIPPED: u32 = 0x0095_ea30;
/// `ECX` = actor, stack (form, extra list): the same step for the others.
const ACTOR_NOTIFY_EQUIPPED: u32 = 0x008c_2630;
/// `ECX` = actor, stack (form, count or 0, flag): the notification the
/// player sees.
const ACTOR_EQUIP_NOTIFICATION: u32 = 0x008a_ded0;
/// `ECX` = actor: steps run after an equip.
const ACTOR_EQUIP_STEP_A: u32 = 0x008b_be00;
const ACTOR_EQUIP_STEP_B: u32 = 0x008a_cba0;
const ACTOR_EQUIP_STEP_C: u32 = 0x008c_1940;
const ACTOR_EQUIP_STEP_D: u32 = 0x008c_17c0;
/// `ECX` = player: a step both weapon cases of `EquipObject` run.
const PLAYER_EQUIP_STEP: u32 = 0x0096_27f0;
/// `PlayerCharacter::AmmoSwapHelper` (Xbox PDB): `ECX` = player, stack
/// (0, 1).
const PLAYER_AMMO_SWAP_HELPER: u32 = 0x0094_62c0;
/// `TESDataHandler::GetSound_ov2` (Xbox PDB): `ECX` = the data handler,
/// one stack word (the editor name).
const DATA_HANDLER_GET_SOUND: u32 = 0x0046_16c0;
/// The pointer to the data handler object.
const DATA_HANDLER_POINTER: u32 = 0x011c_3f2c;
/// The sound name `ITMTorchHeldEquip`.
const TORCH_EQUIP_SOUND_NAME: u32 = 0x0108_48d8;
/// `ECX` = sound, no stack: a word `00933150` takes.
const SOUND_PLAY_ARGUMENT: u32 = 0x0084_e3a0;
/// `ECX` = actor, stack (out handle, word, 0, flags, 1): plays a sound.
const ACTOR_PLAY_SOUND: u32 = 0x0093_3150;
/// `ECX` = actor, stack (form, extra list): whether the form can be used
/// now (RET 8).
const ACTOR_CAN_USE_FORM: u32 = 0x0080_f7d0;
/// `Actor::Eat` (Xbox PDB): `ECX` = actor, stack (form, extra list, flag).
const ACTOR_EAT: u32 = 0x008c_1de0;
/// `Actor::DrinkPotion` (Xbox PDB): `ECX` = actor, stack (form, extra list,
/// flag).
const ACTOR_DRINK_POTION: u32 = 0x008c_1f80;
/// `TESObjectBOOK::Read` (Xbox PDB): `ECX` = book, one stack word (actor).
const BOOK_READ: u32 = 0x0051_5040;
/// `ECX` = book: a number, -1 when the book cannot be read.
const BOOK_SKILL_NUMBER: u32 = 0x0051_4fd0;
/// `ECX` = form: non-zero for a food item.
const POTION_FOOD_TEST: u32 = 0x0040_30b0;
/// `MiscStatManager::Increment` (Xbox PDB), `cdecl`: the statistic number.
const MISC_STAT_INCREMENT: u32 = 0x004d_5c60;
/// `BGSDefaultObjectManager::GetDefaultObject` (Xbox PDB), `cdecl`.
const GET_DEFAULT_OBJECT: u32 = 0x0058_db10;
/// `AlchemyItem::IsWater` (Xbox PDB): `ECX` = form.
const POTION_IS_WATER: u32 = 0x0040_36d0;
/// `EffectItemList::CanBePoison` (Xbox PDB): `ECX` = potion + 0x3c.
const POTION_CAN_BE_POISON: u32 = 0x0040_5da0;
/// `cdecl` (5 or 9, 1, potion, 0, 0, 0, 0): a notification about the
/// potion being used.
const POTION_NOTIFY: u32 = 0x005f_5950;
/// `ECX` = player, stack (potion): the poison choice.
const PLAYER_POISON_CHOICE: u32 = 0x0095_e6f0;
/// `Interface::GetPipboy` (Xbox PDB).
const GET_PIPBOY: u32 = 0x0070_5990;
/// `ECX` = pipboy: refreshes it.
const PIPBOY_REFRESH: u32 = 0x007f_a990;
/// `ECX` = save/load object: tests bit 2 of the word at +0x244.
const SAVE_LOAD_BIT_2_TEST: u32 = 0x0042_ce10;
/// `ECX` = sound handle storage: construct (`SOUND_HANDLE_INIT`), release
/// and fade out are the `SOUND_HANDLE_*` constants above.
const SOUND_HANDLE_FADE_OUT_AND_RELEASE: u32 = 0x00ad_8da0;
/// `ExtraDataList::GetWeaponIdleSound` (Xbox PDB): `ECX` = list, one stack
/// word (the handle).
const EXTRA_GET_WEAPON_IDLE_SOUND: u32 = 0x0041_89c0;
/// `ExtraDataList::SetWeaponIdleSound` (Xbox PDB): `ECX` = list, one stack
/// word (the handle).
const EXTRA_SET_WEAPON_IDLE_SOUND: u32 = 0x0041_a3e0;

/// Form type numbers `EquipObject` and `UnEquipObject` switch on. The class
/// names the callees carry give the meaning: 0x18 armor (the biped model is
/// asked `IsPowerArmor`), 0x19 book (`TESObjectBOOK::Read`), 0x1d food
/// (`Actor::Eat`), 0x1e light (the torch sound), 0x28 weapon, 0x29 ammo,
/// 0x2f potion (`AlchemyItem::IsWater`). 0x1a and 0x33 are not named.
const FORM_TYPE_ARMOR: u32 = 0x18;
const FORM_TYPE_BOOK: u32 = 0x19;
const FORM_TYPE_0X1A: u32 = 0x1a;
const FORM_TYPE_FOOD: u32 = 0x1d;
const FORM_TYPE_LIGHT: u32 = 0x1e;
const FORM_TYPE_WEAPON: u32 = 0x28;
const FORM_TYPE_AMMO: u32 = 0x29;
const FORM_TYPE_POTION: u32 = 0x2f;
const FORM_TYPE_0X33: u32 = 0x33;
/// Extra data type of the item health (+0x0c is a `float`).
const EXTRA_DATA_HEALTH: u32 = 0x25;
/// Action flag number `EquipObject` sets with `SetActionFlag`.
const ACTION_FLAG_USED: u32 = 2;
/// Action flag number `fn_0088e110` sets.
const ACTION_FLAG_REMOVED: u32 = 8;
/// Number of biped slots the equip code walks.
const BIPED_SLOT_COUNT: u32 = 0x14;

/// Shows a message: `creator` (`CREATE_MESSAGE` or `CREATE_MESSAGE_ALT`)
/// with the setting object and the icon (a path address, or 0), then
/// `SHOW_MESSAGE`.
fn show_message(e: &mut Engine, creator: u32, setting: u32, icon: u32) {
    let duration: f32 = e.global(MESSAGE_DURATION);
    // 0088c911: the creator (00403df0 / 004c69f0, plain `RET`) takes only the
    // setting (ECX); the five pushed words are the rest of the cdecl `007052f0`
    // (ADD ESP,0x18).
    let message = e.call(creator, &args![setting]).u32();
    e.call(
        SHOW_MESSAGE,
        &args![message, 0u32, icon, 0u32, duration, 0u32],
    );
}

// Translated from 0088c650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::QueueEquipObject` (Xbox PDB): form types 0x19, 0x1d and 0x2f are
/// equipped at once; for 0x18, 0x28, 0x29 and 0x33 the equip happens at once
/// when the actor has no process, its process level is above 1, it is the
/// player in a menu, or the save/load object's flag `2` is set; otherwise
/// the equip is queued in the process (virtual `+0x178`). Other types do
/// nothing.
#[allow(clippy::too_many_arguments)]
pub fn actor_queue_equip_object(
    e: &mut Engine,
    this: Ptr<Actor>,
    item: Ptr,
    count: i32,
    extra: Ptr,
    flag_a: u8,
    flag_b: u8,
    flag_c: u8,
) {
    match e.call(FORM_TYPE, &args![item]).u32() {
        FORM_TYPE_ARMOR | FORM_TYPE_WEAPON | FORM_TYPE_AMMO | FORM_TYPE_0X33 => {
            let process = e.get(this, Actor::pCurrentProcess);
            let at_once = process.is_null()
                || e.call(PROCESS_GET_LEVEL, &args![process]).i32() > 1
                || (this.addr() == e.mem.u32(PLAYER_CHARACTER)
                    && e.call(IS_IN_MENU_MODE, &args![]).bool())
                || {
                    let save_load = e.mem.u32(SAVE_LOAD_GAME);
                    e.call(SAVE_LOAD_BIT_2_TEST, &args![save_load]).bool()
                };
            if at_once {
                actor_equip_object(e, this, item, count, extra, flag_a, flag_b, flag_c);
            } else {
                e.vcall(
                    process.addr(),
                    PROCESS_VSLOT_QUEUE_EQUIP,
                    &args![
                        this, 1u32, item, count, extra, flag_a, flag_b, 0u32, 0u32, 0u32, flag_c
                    ],
                );
            }
        }
        FORM_TYPE_BOOK | FORM_TYPE_FOOD | FORM_TYPE_POTION => {
            actor_equip_object(e, this, item, count, extra, flag_a, flag_b, flag_c);
        }
        _ => {}
    }
}

// Translated from 0088c790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::QueueUnEquipObject` (Xbox PDB): queues the removal in the process
/// (virtual `+0x178`) when the actor has a process at level 0 or 1 and is
/// not the player in a menu; otherwise removes at once.
#[allow(clippy::too_many_arguments)]
pub fn actor_queue_un_equip_object(
    e: &mut Engine,
    this: Ptr<Actor>,
    item: Ptr,
    count: i32,
    extra: Ptr,
    flag_a: u8,
    flag_b: u8,
    flag_c: u8,
) {
    let process = e.get(this, Actor::pCurrentProcess);
    if !process.is_null()
        && e.call(PROCESS_GET_LEVEL, &args![process]).i32() <= 1
        && (this.addr() != e.mem.u32(PLAYER_CHARACTER) || !e.call(IS_IN_MENU_MODE, &args![]).bool())
    {
        e.vcall(
            process.addr(),
            PROCESS_VSLOT_QUEUE_EQUIP,
            &args![this, 0u32, item, count, extra, 0u32, 0u32, 0u32, flag_a, flag_b, flag_c],
        );
        return;
    }
    actor_un_equip_object(e, this, item, count, extra, flag_a, flag_b, flag_c);
}

/// The torch part of `EquipObject`: for a form of type 0x1e whose sound
/// `ITMTorchHeldEquip` exists, plays it (flags 0x120 when the player is in a
/// menu, 0x102 otherwise).
fn equip_torch_sound(e: &mut Engine, this: Ptr<Actor>, item: Ptr, is_player: bool) {
    if e.call(FORM_TYPE, &args![item]).u32() != FORM_TYPE_LIGHT {
        return;
    }
    let handler = e.mem.u32(DATA_HANDLER_POINTER);
    let sound = e
        .call(
            DATA_HANDLER_GET_SOUND,
            &args![handler, TORCH_EQUIP_SOUND_NAME],
        )
        .u32();
    if sound == 0 {
        return;
    }
    e.with_stack(0x40, |e, frame| {
        let handle = frame.addr();
        let out = frame.addr() + 0x20;
        e.call(SOUND_HANDLE_INIT, &args![handle]);
        let flags = if is_player && e.call(IS_IN_MENU_MODE, &args![]).bool() {
            0x120u32
        } else {
            0x102u32
        };
        let argument = e.call(SOUND_PLAY_ARGUMENT, &args![sound]).u32();
        e.call(
            ACTOR_PLAY_SOUND,
            &args![this, out, argument, 0u32, flags, 1u32],
        );
        e.call(EMPTY_FUNCTION, &args![out]);
        e.call(EMPTY_FUNCTION, &args![handle]);
    });
}

/// The statistics `EquipObject` counts for a potion used by the player:
/// 9 for a food item, else 6, 8, 7 or 0x1a when the potion is one of four
/// default objects (0, 3, 2, 0x15), else 0x13 for water.
fn count_potion_statistic(e: &mut Engine, potion: Ptr) {
    if e.call(POTION_FOOD_TEST, &args![potion]).u32() != 0 {
        e.call(MISC_STAT_INCREMENT, &args![9u32]);
        return;
    }
    for (default_object, statistic) in [(0u32, 6u32), (3, 8), (2, 7), (0x15, 0x1a)] {
        if potion.addr() == e.call(GET_DEFAULT_OBJECT, &args![default_object]).u32() {
            e.call(MISC_STAT_INCREMENT, &args![statistic]);
            return;
        }
    }
    if e.call(POTION_IS_WATER, &args![potion]).bool() {
        e.call(MISC_STAT_INCREMENT, &args![0x13u32]);
    }
}

// Translated from 0088c830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::EquipObject` (Xbox PDB). Uses or equips `item` according to its
/// form type (see the `FORM_TYPE_*` constants): armor and weapons refuse
/// broken items (health from the extra data `0x25` or the form), books are
/// read, food eaten, potions drunk (or offered as poison to the player),
/// ammo swapped, and light, `0x1a` and armor go through `fn_0088db20`. The
/// player gets a message for each refusal. `flag_b` lets an item that cannot
/// be worn through, `flag_a` selects the other notify step for the player
/// and `flag_c` asks for the final notification. The SEH frame is not
/// translated.
#[allow(clippy::too_many_arguments)]
pub fn actor_equip_object(
    e: &mut Engine,
    this: Ptr<Actor>,
    item: Ptr,
    mut count: i32,
    extra: Ptr,
    flag_a: u8,
    flag_b: u8,
    flag_c: u8,
) {
    let player = e.mem.u32(PLAYER_CHARACTER);
    let is_player = this.addr() == player;
    let mut unsupported = false;
    let mut action_flag_pending = true;
    let mut notify_flag = 0u32;
    fn_00884f80(e, this);
    if e.call(FORM_TYPE, &args![item]).u32() == FORM_TYPE_AMMO
        && e.vcall(item.addr(), FORM_VSLOT_0X94, &args![]).bool()
    {
        e.call(SET_ACTION_FLAG, &args![this, extra, ACTION_FLAG_USED]);
        let list = e.call(GET_EXTRA_DATA_LIST_ADDRESS, &args![this]).u32();
        e.call(SET_ACTION_FLAG, &args![item, list, ACTION_FLAG_USED]);
        return;
    }
    if is_player
        && fn_0088d2d0(e, Ptr::new(player)) == 0
        && !item.is_null()
        && e.call(FORM_TYPE, &args![item]).u32() == FORM_TYPE_ARMOR
        && e.call(
            BIPED_IS_POWER_ARMOR,
            &args![item.addr() + ARMOR_BIPED_MODEL_OFFSET],
        )
        .bool()
    {
        show_message(e, CREATE_MESSAGE, POWER_ARMOR_MESSAGE, MESSAGE_ICON_PATH);
        return;
    }
    if !extra.is_null() && e.call(EXTRA_LIST_CAN_NOT_WEAR, &args![extra]).bool() && flag_b == 0 {
        if is_player {
            show_message(e, CREATE_MESSAGE, CANNOT_WEAR_MESSAGE, MESSAGE_ICON_PATH);
        }
        return;
    }
    let changes = e.call(INVENTORY_CHANGES_OF, &args![this]).u32();
    let held_count = e.call(INVENTORY_OBJECT_COUNT, &args![changes, item]).i32();
    if e.call(FORM_TYPE, &args![item]).u32() != FORM_TYPE_AMMO
        && e.call(FORM_TYPE, &args![item]).u32() != FORM_TYPE_WEAPON
        && !extra.is_null()
        && e.call(EXTRA_LIST_GET_WORN, &args![extra, 0u32]).bool()
    {
        return;
    }
    e.call(BIPED_MODEL_OF, &args![item]);
    if held_count < 1 {
        return;
    }
    if count < 1 && e.call(FORM_TYPE, &args![item]).u32() == FORM_TYPE_AMMO {
        count = held_count;
    }
    let mut health_data = 0u32;
    if !extra.is_null() {
        health_data = e
            .call(BASE_EXTRA_GET_DATA, &args![extra, EXTRA_DATA_HEALTH])
            .u32();
    }
    let health = if health_data != 0 {
        e.mem.f32(health_data + 0xc)
    } else {
        e.call(FORM_HEALTH, &args![item]).u32() as f32
    };
    let zero: f64 = e.global(DOUBLE_ZERO);
    let item_type = e.call(FORM_TYPE, &args![item]).u32();
    // The shared tail of armor, light and `0x1a`: the use test, the equip
    // step and the notifications.
    let mut equip_step = false;
    match item_type {
        FORM_TYPE_WEAPON => {
            if count > 1 && !e.call(FORM_FLAG_TEST_4C0BF0, &args![item]).bool() {
                count = 1;
            }
            if health as f64 > zero {
                fn_0088db20(e, this, item, count, extra, flag_b);
            } else if is_player {
                show_message(e, CREATE_MESSAGE, BROKEN_ITEM_MESSAGE, MESSAGE_ICON_PATH);
                action_flag_pending = false;
            }
            if is_player {
                e.call(PLAYER_EQUIP_STEP, &args![player]);
                e.call(PLAYER_AMMO_SWAP_HELPER, &args![player, 0u32, 1u32]);
            }
            e.vcall(this.addr(), VSLOT_AFTER_EQUIP_CHANGE, &args![]);
            e.call(ACTOR_EQUIP_STEP_A, &args![this]);
            e.call(ACTOR_EQUIP_STEP_B, &args![this]);
            e.call(ACTOR_EQUIP_STEP_C, &args![this]);
            e.call(ACTOR_EQUIP_STEP_D, &args![this]);
        }
        FORM_TYPE_0X33 => {
            fn_0088db20(e, this, item, count, extra, flag_b);
            e.call(PLAYER_EQUIP_STEP, &args![player]);
            e.vcall(this.addr(), VSLOT_AFTER_EQUIP_CHANGE, &args![]);
        }
        FORM_TYPE_ARMOR => {
            if health as f64 > zero || health.is_nan() {
                e.vcall(this.addr(), VSLOT_AFTER_EQUIP_CHANGE, &args![]);
                equip_torch_sound(e, this, item, is_player);
                equip_step = true;
            } else {
                if e.vcall(this.addr(), VSLOT_ACTOR_FLAG_0X360, &args![])
                    .bool()
                {
                    show_message(e, CREATE_MESSAGE, BROKEN_ITEM_MESSAGE, MESSAGE_ICON_PATH);
                }
                action_flag_pending = false;
            }
        }
        FORM_TYPE_LIGHT => {
            equip_torch_sound(e, this, item, is_player);
            equip_step = true;
        }
        FORM_TYPE_0X1A => equip_step = true,
        FORM_TYPE_AMMO => {
            let process = e.call(GET_PROCESS, &args![this]).u32();
            let held = e.vcall(process, PROCESS_VSLOT_ITEM, &args![]).u32();
            let held_form = if held == 0 {
                0
            } else {
                e.call(ITEM_VALUE, &args![held]).u32()
            };
            if held_form != 0
                && !(10..=13).contains(&e.call(ITEM_GROUP_INDEX, &args![held_form]).u32())
            {
                let mod_active = e.call(ITEM_HAS_MOD_EFFECT_ACTIVE, &args![held, 2u32]).u8() as u32;
                let rounds = e
                    .call(FORM_CLIP_ROUNDS, &args![held_form, mod_active])
                    .u32();
                count = e.call(COUNT_FOR_CLIPS, &args![rounds, count]).i32();
            }
            if e.call(AMMO_NOTIFY_LEVEL, &args![]).i32() > 0 {
                e.call(AMMO_NOTIFY, &args![item, count, this, 1u32, 1u32]);
            }
            let storage = e.call(OPERATOR_NEW_FN, &args![0xcu32]).u32();
            let change = if storage == 0 {
                0
            } else {
                e.call(ITEM_CHANGE_CONSTRUCT, &args![storage, item, count])
                    .u32()
            };
            let process = e.get(this, Actor::pCurrentProcess);
            e.vcall(
                process.addr(),
                PROCESS_VSLOT_SET_EQUIPPED_CHANGE,
                &args![change],
            );
        }
        FORM_TYPE_FOOD => {
            e.call(SET_ACTION_FLAG, &args![this, extra, ACTION_FLAG_USED]);
            if !e.vcall(item.addr(), FORM_VSLOT_0X94, &args![]).bool() {
                e.call(ACTOR_EAT, &args![this, item, extra, !is_player]);
                notify_flag = 1;
                e.call(
                    ACTOR_EQUIP_NOTIFICATION,
                    &args![this, item, 1u32, notify_flag],
                );
            } else if is_player {
                show_message(e, CREATE_MESSAGE, CANNOT_USE_MESSAGE, MESSAGE_ICON_PATH);
            }
            return;
        }
        FORM_TYPE_BOOK => {
            if e.vcall(this.addr(), VSLOT_ACTOR_FLAG_0X360, &args![])
                .bool()
                && e.call(BOOK_SKILL_NUMBER, &args![item]).i32() != -1
                && e.call(BOOK_READ, &args![item, this]).bool()
            {
                e.vcall(
                    this.addr(),
                    VSLOT_USE_ITEM_0X17C,
                    &args![item, extra, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32],
                );
                e.call(ACTOR_EQUIP_NOTIFICATION, &args![this, item, 1u32, 1u32]);
                return;
            }
        }
        FORM_TYPE_POTION => {
            notify_flag = 1;
            if !item.is_null() {
                if e.vcall(this.addr(), VSLOT_ACTOR_FLAG_0X360, &args![])
                    .bool()
                {
                    count_potion_statistic(e, item);
                }
                if e.call(POTION_CAN_BE_POISON, &args![item.addr() + 0x3c])
                    .bool()
                {
                    if is_player {
                        e.call(PLAYER_POISON_CHOICE, &args![player, item]);
                        return;
                    }
                } else {
                    for notification in [5u32, 9] {
                        e.call(
                            POTION_NOTIFY,
                            &args![notification, 1u32, item, 0u32, 0u32, 0u32, 0u32],
                        );
                    }
                    e.call(SET_ACTION_FLAG, &args![this, extra, ACTION_FLAG_USED]);
                    if e.call(ACTOR_DRINK_POTION, &args![this, item, extra, !is_player])
                        .bool()
                    {
                        e.call(
                            ACTOR_EQUIP_NOTIFICATION,
                            &args![this, item, 1u32, notify_flag],
                        );
                    }
                    return;
                }
            }
        }
        _ => unsupported = true,
    }
    if equip_step {
        if e.call(ACTOR_CAN_USE_FORM, &args![this, item, extra]).bool() {
            if count > 1 {
                count = 1;
            }
            if fn_0088db20(e, this, item, count, extra, flag_b) != 0 {
                if is_player && flag_a == 0 {
                    e.call(PLAYER_NOTIFY_EQUIPPED, &args![player, item, extra]);
                } else {
                    e.call(ACTOR_NOTIFY_EQUIPPED, &args![this, item, extra]);
                }
                let biped = e.call(BIPED_MODEL_OF, &args![item]).u32();
                if biped != 0
                    && e.call(BIPED_FILLS_SLOT, &args![biped, 6u32, 0u32, 0u32])
                        .bool()
                {
                    let pipboy = e.call(GET_PIPBOY, &args![]).u32();
                    if pipboy != 0 {
                        e.call(PIPBOY_REFRESH, &args![pipboy]);
                    }
                }
            }
        } else if is_player {
            show_message(e, CREATE_MESSAGE, EQUIP_FAILED_MESSAGE, 0);
            return;
        }
    }
    if unsupported && is_player {
        show_message(e, CREATE_MESSAGE, CANNOT_WEAR_MESSAGE, MESSAGE_ICON_PATH);
    }
    if is_player && flag_c != 0 {
        e.call(
            ACTOR_EQUIP_NOTIFICATION,
            &args![this, item, 1u32, notify_flag],
        );
    }
    let combat_object = e.mem.u32(COMBAT_OBJECT_POINTER);
    if !e.call(COMBAT_CONTROLLER_TEST, &args![combat_object]).bool() && action_flag_pending {
        e.call(SET_ACTION_FLAG, &args![this, extra, ACTION_FLAG_USED]);
        let list = e.call(GET_EXTRA_DATA_LIST_ADDRESS, &args![this]).u32();
        e.call(SET_ACTION_FLAG, &args![item, list, ACTION_FLAG_USED]);
    }
}

// Translated from 0088d2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at +0x7c7 of the object (the player in the callers).
pub fn fn_0088d2d0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x7c7)
}

/// `InventoryChanges` removal (`004c0cf0`): `ECX` = the changes, stack
/// (out byte pointer, form, count, actor, extra list, worn flag, flag, 0, 0),
/// `RET 0x24`; returns a `bool` in `AL`.
const INVENTORY_REMOVE: u32 = 0x004c_0cf0;

/// `ECX` = player, one stack byte: stores it at +0x64a and +0x64c (the
/// byte `PLAYER_STATE_TEST` reads back).
const PLAYER_STATE_SET: u32 = 0x005c_4210;
/// `TESFurniture::CanSitOn` (Xbox PDB): `ECX` = the furniture form.
const FURNITURE_CAN_SIT_ON: u32 = 0x0050_93f0;
/// `TESFurniture::CanSleepOn` (Xbox PDB): `ECX` = the furniture form.
const FURNITURE_CAN_SLEEP_ON: u32 = 0x0050_9420;
/// `ECX` = marker record: the `ushort` at +0x0c divided by 1000, a `float`
/// in ST0.
const MARKER_ANGLE: u32 = 0x0056_8650;
/// `ECX` = marker record: the byte at +0x0e.
const MARKER_NUMBER: u32 = 0x0050_94b0;
/// `TESFurniture::GetMarkerTargetOffset` (Xbox PDB): `ECX` = the form,
/// stack (out vector, marker number, `float` scale).
const FURNITURE_MARKER_TARGET_OFFSET: u32 = 0x0050_9920;
/// `ECX` = the form, one stack word (the marker number): a `float` in ST0
/// that is added to the actor's heading.
const FURNITURE_MARKER_HEIGHT: u32 = 0x0050_99c0;
/// `TESObjectREFR::SetMarkerUsed` (Xbox PDB): `ECX` = the furniture
/// reference, stack (marker user, 1).
const SET_MARKER_USED: u32 = 0x0056_8020;
/// `bhkCharacterController::SetPosition` (Xbox PDB): `ECX` = controller,
/// one stack word (the position vector).
const CONTROLLER_SET_POSITION: u32 = 0x0056_20e0;
/// `ECX` = actor, one `float` stack word: adds it to the actor's heading
/// (calls the actor virtuals `+0x2bc` and `+0x2c4`).
const ACTOR_ADD_TO_HEADING: u32 = 0x0093_1d30;
/// `ECX` = reference: its full name (a lookup of the reference and `GetFullName`).
const REFERENCE_FULL_NAME: u32 = 0x0055_d520;
/// The format `"%s went to sit at %s and had no animation\n"`.
const NO_ANIMATION_FORMAT: u32 = 0x0108_48ec;
/// Actor virtual `+0x2c4`: sets the heading (one `float`).
const VSLOT_SET_ROTATION: u32 = 0x2c4;
/// Actor virtual `+0x2a8`: sets the position (a vector pointer).
const VSLOT_SET_POSITION: u32 = 0x2a8;
/// Process virtual `+0x284`: one word.
const PROCESS_VSLOT_0X284: u32 = 0x284;
/// Process virtual `+0x4c0`: sets the furniture state (actor, state,
/// furniture, user byte).
const PROCESS_VSLOT_SET_FURNITURE_STATE: u32 = 0x4c0;
/// Process virtual `+0x4d8`: (actor) true when the furniture animation is
/// ready.
const PROCESS_VSLOT_FURNITURE_ANIMATION_READY: u32 = 0x4d8;
/// Process virtual `+0x4bc`: non-zero when the actor is on furniture.
const PROCESS_VSLOT_ON_FURNITURE: u32 = 0x4bc;
/// Process virtual `+0x71c`: one word.
const PROCESS_VSLOT_0X71C: u32 = 0x71c;
/// Process virtual `+0x84`: (actor).
const PROCESS_VSLOT_0X84: u32 = 0x84;
/// `ECX` = animation: a step before clearing the group when the actor
/// leaves furniture.
const ANIMATION_STEP_0X4974A0: u32 = 0x0049_74a0;
/// `Actor::GetAnimGroup` (Xbox PDB): `ECX` = actor, stack (0, held, 1,
/// animation); the group number is the low 16 bits.
const ACTOR_GET_ANIM_GROUP: u32 = 0x0089_7910;
/// `Animation::PlayGroup` (Xbox PDB): `ECX` = animation, stack (group, 1, -1,
/// -1).
const ANIMATION_PLAY_GROUP: u32 = 0x0049_4740;
/// `MobileObject::SetChaseBip` (Xbox PDB): `ECX` = actor, one stack word.
const SET_CHASE_BIP: u32 = 0x0093_1fb0;

// Translated from 0088d2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::PutActorInChairBedQuick` (Xbox PDB): puts the actor on the marker
/// of `furniture` at once. `marker` points at a marker record (position at
/// +0, an angle in thousandths at +0x0c, the marker number at +0x0e).
/// Returns 0 without an animation or character controller or when the
/// furniture form can neither be sat nor slept on; also 0 (after resetting
/// the furniture state) when the process has no animation for it; else 1.
/// With `sleeping` the actor goes to state 6 then 9 instead of 1 then 4.
pub fn actor_put_actor_in_chair_bed_quick(
    e: &mut Engine,
    this: Ptr<Actor>,
    furniture: Ptr,
    marker: Ptr,
    marker_user: u32,
    sleeping: u8,
) -> u8 {
    let player = e.mem.u32(PLAYER_CHARACTER);
    let is_player = this.addr() == player;
    let saved_state = e.call(PLAYER_STATE_TEST, &args![player]).u8();
    if is_player {
        e.call(PLAYER_STATE_SET, &args![player, 1u32]);
    }
    let animation = e.vcall(this.addr(), VSLOT_GET_ANIMATION, &args![]).u32();
    if animation == 0 || e.call(GET_CHAR_CONTROLLER, &args![this]).u32() == 0 {
        return 0;
    }
    let form = e.call(SAVE_FORM_BUFFER_GET_FORM, &args![furniture]).u32();
    if !e.call(FURNITURE_CAN_SIT_ON, &args![form]).bool()
        && !e.call(FURNITURE_CAN_SLEEP_ON, &args![form]).bool()
    {
        return 0;
    }
    let angle = e.call(MARKER_ANGLE, &args![marker]).f32();
    e.vcall(this.addr(), VSLOT_SET_ROTATION, &args![angle]);
    let scale = e.call(GET_SCALE, &args![this]).f32();
    let marker_number = e.call(MARKER_NUMBER, &args![marker]).u32();
    e.with_stack(0x60, |e, frame| {
        // The game's locals: the rotation matrix (9 words), the offset
        // vector and two result vectors.
        let matrix = frame.addr();
        let offset = frame.addr() + 0x30;
        let rotated = frame.addr() + 0x40;
        let sum = frame.addr() + 0x50;
        e.call(
            FURNITURE_MARKER_TARGET_OFFSET,
            &args![form, offset, marker_number, scale],
        );
        e.call(MATRIX_CONSTRUCT, &args![matrix]);
        let angle = e.call(MARKER_ANGLE, &args![marker]).f32();
        e.call(MATRIX_MAKE_ROTATION, &args![matrix, angle]);
        let result = e
            .call(MATRIX_TIMES_VECTOR, &args![matrix, rotated, offset])
            .u32();
        for word in 0..3 {
            let value = e.mem.u32(result + 4 * word);
            e.mem.set_u32(offset + 4 * word, value);
        }
        let process = e.get(this, Actor::pCurrentProcess);
        e.vcall(process.addr(), PROCESS_VSLOT_0X284, &args![2u32]);
        e.call(SET_MARKER_USED, &args![furniture, marker_user, 1u32]);
        e.call(MATRIX_CONSTRUCT, &args![marker]);
        let position = e
            .call(VECTOR_PLUS_VECTOR, &args![marker, sum, offset])
            .u32();
        e.vcall(this.addr(), VSLOT_SET_POSITION, &args![position]);
        e.call(MATRIX_CONSTRUCT, &args![marker]);
        let controller = e.call(GET_CHAR_CONTROLLER, &args![this]).u32();
        e.call(CONTROLLER_SET_POSITION, &args![controller, marker]);
        let animation = e.vcall(this.addr(), VSLOT_GET_ANIMATION, &args![]).u32();
        e.call(ANIMATION_CLEAR_GROUP, &args![animation, 0x14u32, 0.0f32]);
        if is_player {
            let animation = e.call(PLAYER_GET_ANIMATION, &args![player, 1u32]).u32();
            e.call(ANIMATION_CLEAR_GROUP, &args![animation, 0x14u32, 0.0f32]);
        }
        let marker_number = e.call(MARKER_NUMBER, &args![marker]).u32();
        let form = e.call(SAVE_FORM_BUFFER_GET_FORM, &args![furniture]).u32();
        let height = e
            .call(FURNITURE_MARKER_HEIGHT, &args![form, marker_number])
            .f32();
        e.call(ACTOR_ADD_TO_HEADING, &args![this, height]);
        let user_byte = marker_user & 0xff;
        let process = e.get(this, Actor::pCurrentProcess);
        if sleeping != 0 {
            e.vcall(
                process.addr(),
                PROCESS_VSLOT_SET_FURNITURE_STATE,
                &args![this, 6u32, furniture, user_byte],
            );
            let process = e.get(this, Actor::pCurrentProcess);
            e.vcall(process.addr(), PROCESS_VSLOT_0X284, &args![1u32]);
        } else {
            e.vcall(
                process.addr(),
                PROCESS_VSLOT_SET_FURNITURE_STATE,
                &args![this, 1u32, furniture, user_byte],
            );
        }
        let process = e.get(this, Actor::pCurrentProcess);
        let placed;
        if e.vcall(
            process.addr(),
            PROCESS_VSLOT_FURNITURE_ANIMATION_READY,
            &args![this],
        )
        .bool()
        {
            placed = 1u8;
            let process = e.get(this, Actor::pCurrentProcess);
            if sleeping != 0 {
                let face = e.call(GET_FACE_ANIMATION_DATA, &args![this]).u32();
                if face != 0 {
                    e.vcall(face, FACE_VSLOT_0XD8, &args![1u32, 0u32]);
                }
                e.vcall(
                    process.addr(),
                    PROCESS_VSLOT_SET_FURNITURE_STATE,
                    &args![this, 9u32, furniture, user_byte],
                );
            } else {
                e.vcall(
                    process.addr(),
                    PROCESS_VSLOT_SET_FURNITURE_STATE,
                    &args![this, 4u32, furniture, user_byte],
                );
            }
        } else {
            placed = 0u8;
            let furniture_name = e.call(REFERENCE_FULL_NAME, &args![furniture]).u32();
            let actor_name = e.call(REFERENCE_FULL_NAME, &args![this]).u32();
            e.call(
                LOG_MESSAGE,
                &args![NO_ANIMATION_FORMAT, actor_name, furniture_name],
            );
            let process = e.get(this, Actor::pCurrentProcess);
            e.vcall(
                process.addr(),
                PROCESS_VSLOT_SET_FURNITURE_STATE,
                &args![this, 0u32, 0u32, 0x7fu32],
            );
        }
        if is_player && saved_state == 0 {
            e.call(PLAYER_STATE_SET, &args![player, 0u32]);
        }
        placed
    })
}

// Translated from 0088d640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetOutofFurnitureQuick` (Xbox PDB): when the process virtual
/// `+0x4bc` says the actor is on furniture, clears (or blends out) the
/// furniture animation group, plays the group the actor leaves with (not for
/// the player), resets the process (virtuals `+0x71c` and `+0x84`) and the
/// chase bip.
pub fn actor_get_out_of_furniture_quick(e: &mut Engine, this: Ptr<Actor>) {
    let process = e.get(this, Actor::pCurrentProcess);
    if e.vcall(process.addr(), PROCESS_VSLOT_ON_FURNITURE, &args![])
        .u32()
        == 0
    {
        return;
    }
    let player = e.mem.u32(PLAYER_CHARACTER);
    let is_player = this.addr() == player;
    let saved_state = e.call(PLAYER_STATE_TEST, &args![player]).u8();
    if is_player {
        e.call(PLAYER_STATE_SET, &args![player, 1u32]);
    }
    let animation = e.vcall(this.addr(), VSLOT_GET_ANIMATION, &args![]).u32();
    if animation != 0 {
        let save_load = e.mem.u32(SAVE_LOAD_GAME);
        if e.call(SAVE_LOAD_BIT_2_TEST, &args![save_load]).bool()
            || e.call(PLAYER_SLEEPING_OR_RESTING, &args![player]).bool()
        {
            e.call(ANIMATION_STEP_0X4974A0, &args![animation]);
            e.call(ANIMATION_CLEAR_GROUP, &args![animation, 0x14u32, 0.0f32]);
        } else {
            e.call(ANIMATION_BLEND_OUT, &args![animation, 0x14u32, 0u32]);
        }
        let process = e.get(this, Actor::pCurrentProcess);
        let held = e.vcall(process.addr(), PROCESS_VSLOT_ITEM, &args![]).u32();
        let group = e
            .call(
                ACTOR_GET_ANIM_GROUP,
                &args![this, 0u32, held, 1u32, animation],
            )
            .u16();
        if is_player {
            let player_animation = e.call(PLAYER_GET_ANIMATION, &args![player, 1u32]).u32();
            e.call(
                ANIMATION_CLEAR_GROUP,
                &args![player_animation, 0x14u32, 0.0f32],
            );
        } else if e.call(ANIM_GROUP_GET_TYPE, &args![group as u32]).u32() == 0 {
            e.call(
                ANIMATION_PLAY_GROUP,
                &args![
                    animation,
                    group as u32,
                    1u32,
                    0xffff_ffffu32,
                    0xffff_ffffu32
                ],
            );
        }
        let process = e.get(this, Actor::pCurrentProcess);
        e.vcall(process.addr(), PROCESS_VSLOT_0X71C, &args![0u32]);
    }
    let process = e.get(this, Actor::pCurrentProcess);
    e.vcall(process.addr(), PROCESS_VSLOT_0X84, &args![this]);
    e.call(SET_CHASE_BIP, &args![this, 0u32]);
    if is_player && saved_state == 0 {
        e.call(PLAYER_STATE_SET, &args![player, 0u32]);
    }
}

// Translated from 0088d7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::UnEquipObject` (Xbox PDB): takes `item` off. Refuses (with a
/// message for the player) an extra data list that cannot be removed while
/// the actor virtual `+0x22c` is false; armor (0x18, after virtual `+0x3c4`)
/// and type 0x1a drop their magic effects, tell the player's menu and go
/// through `fn_0088e110`; weapons (0x28) go through it too and then release
/// the idle and attack sounds, reset the process (virtuals `+0x458`,
/// `+0x14c`, `+0x168`, `+0x3e4`) and refresh the actor. Returns the result
/// of `fn_0088e110` (1 for a null item in that function), 0 when refused or
/// for other types. The fifth stack word is not read. The SEH frame is not
/// translated.
#[allow(clippy::too_many_arguments)]
pub fn actor_un_equip_object(
    e: &mut Engine,
    this: Ptr<Actor>,
    item: Ptr,
    count: i32,
    extra: Ptr,
    flag_a: u8,
    _unused_4: u8,
    flag_c: u8,
) -> u8 {
    let player = e.mem.u32(PLAYER_CHARACTER);
    let is_player = this.addr() == player;
    let mut result = 0u8;
    fn_00884f80(e, this);
    if !extra.is_null()
        && e.call(EXTRA_LIST_CAN_NOT_WEAR, &args![extra]).bool()
        && !e
            .vcall(this.addr(), VSLOT_IS_PROCESS_FLAG_0X22C, &args![0u32])
            .bool()
    {
        if is_player {
            show_message(e, CREATE_MESSAGE_ALT, CANNOT_REMOVE_MESSAGE, 0);
        }
        return result;
    }
    let worn = !extra.is_null() && e.call(EXTRA_LIST_GET_WORN, &args![extra, 1u32]).bool();
    if item.is_null() {
        return 0;
    }
    let item_type = e.call(FORM_TYPE, &args![item]).u32();
    match item_type {
        FORM_TYPE_ARMOR | FORM_TYPE_0X1A => {
            if item_type == FORM_TYPE_ARMOR {
                e.vcall(this.addr(), VSLOT_AFTER_EQUIP_CHANGE, &args![]);
            }
            e.call(
                MAGIC_TARGET_REMOVE_EFFECTS,
                &args![this.addr() + 0x94, item, 1u32],
            );
            if is_player {
                if e.call(IS_IN_MENU_MODE, &args![]).bool()
                    && !e.call(PLAYER_SLEEPING_OR_RESTING, &args![player]).bool()
                {
                    e.call(MAGIC_TARGET_UPDATE, &args![player + 0x94, 0.0f32]);
                }
                e.call(
                    PLAYER_REMOVE_QUEUED_ENCHANTMENT,
                    &args![player, item, extra],
                );
                if flag_c != 0 {
                    e.call(ACTOR_EQUIP_NOTIFICATION, &args![this, item, 0u32, 0u32]);
                }
            }
            result = fn_0088e110(e, this, item, count, extra, worn as u8, flag_a);
        }
        FORM_TYPE_WEAPON => {
            result = fn_0088e110(e, this, item, count, extra, worn as u8, flag_a);
            e.with_stack(0x20, |e, handle| {
                let handle = handle.addr();
                e.call(SOUND_HANDLE_INIT, &args![handle]);
                let list = e.call(GET_EXTRA_DATA_LIST_ADDRESS, &args![this]).u32();
                e.call(EXTRA_GET_WEAPON_IDLE_SOUND, &args![list, handle]);
                release_sound_handle(e, handle);
                let list = e.call(GET_EXTRA_DATA_LIST_ADDRESS, &args![this]).u32();
                e.call(EXTRA_SET_WEAPON_IDLE_SOUND, &args![list, handle]);
                let list = e.call(GET_EXTRA_DATA_LIST_ADDRESS, &args![this]).u32();
                e.call(EXTRA_GET_WEAPON_ATTACK_SOUND, &args![list, handle]);
                release_sound_handle(e, handle);
                let list = e.call(GET_EXTRA_DATA_LIST_ADDRESS, &args![this]).u32();
                e.call(EXTRA_SET_WEAPON_ATTACK_SOUND, &args![list, handle]);
                let process = e.get(this, Actor::pCurrentProcess);
                if !process.is_null() {
                    e.vcall(process.addr(), PROCESS_VSLOT_0X458, &args![this, 0u32]);
                    if e.vcall(process.addr(), PROCESS_VSLOT_AMMO_ITEM, &args![])
                        .u32()
                        != 0
                    {
                        e.vcall(
                            process.addr(),
                            PROCESS_VSLOT_SET_EQUIPPED_CHANGE,
                            &args![0u32],
                        );
                    }
                    if e.vcall(process.addr(), PROCESS_VSLOT_STATE, &args![]).u32() == 7 {
                        e.call(ACTOR_SET_BLOCK, &args![this, 0u32]);
                    }
                }
                e.call(ACTOR_EQUIP_STEP_A, &args![this]);
                e.call(ACTOR_EQUIP_STEP_C, &args![this]);
                e.call(ACTOR_EQUIP_STEP_D, &args![this]);
                if is_player && flag_c != 0 {
                    e.call(ACTOR_EQUIP_NOTIFICATION, &args![this, item, 0u32, 0u32]);
                }
                e.call(EMPTY_FUNCTION, &args![handle]);
            });
        }
        _ => {}
    }
    result
}

/// Releases a sound handle: fades it out (500) when it is playing, else
/// releases it at once.
fn release_sound_handle(e: &mut Engine, handle: u32) {
    if e.call(SOUND_HANDLE_IS_PLAYING, &args![handle]).bool() {
        e.call(SOUND_HANDLE_FADE_OUT_AND_RELEASE, &args![handle, 500u32]);
    } else {
        e.call(SOUND_HANDLE_RELEASE, &args![handle]);
    }
}

/// The first word of the worn entry, then the word it points to (the game
/// reaches it through `00559450` and the identity function `006815c0`).
fn worn_entry_word(e: &mut Engine, worn: u32) -> u32 {
    let first = e.call(WORN_FIRST_WORD, &args![worn]).u32();
    let pointer = e.call(MATRIX_CONSTRUCT, &args![first]).u32();
    e.mem.u32(pointer)
}

// Translated from 0088db20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes off whatever occupies the biped slots of `item` (each removal
/// through `UnEquipObject`), then updates the held items (weapons and the
/// `0x33` type: remove the held item and re-equip; ammo: remove the held
/// ammo), applies the equip to the inventory changes (`004bffe0`), and
/// finishes with the NPC worn object, the enchantment preload and the
/// refraction update. Returns 0 when a worn item that cannot be taken off
/// blocks it (unless the combat object test holds), else 1. The map has no
/// name for it; the SEH frame is not translated.
pub fn fn_0088db20(
    e: &mut Engine,
    this: Ptr<Actor>,
    item: Ptr,
    count: i32,
    extra: Ptr,
    flag: u8,
) -> u8 {
    let player = e.mem.u32(PLAYER_CHARACTER);
    let is_player = this.addr() == player;
    let biped_data = if is_player {
        e.call(PLAYER_GET_BIPED, &args![player, 0u32]).u32()
    } else {
        e.vcall(this.addr(), VSLOT_GET_BIPED, &args![]).u32()
    };
    let biped = e.call(BIPED_MODEL_OF, &args![item]).u32();
    e.call(INVENTORY_CHANGES_OF, &args![this]);
    let mut blocked = false;
    if biped != 0 && biped_data != 0 {
        for slot in 0..BIPED_SLOT_COUNT {
            if !e
                .call(BIPED_FILLS_SLOT, &args![biped, slot, 0u32, 0u32])
                .bool()
            {
                continue;
            }
            for inner in 0..BIPED_SLOT_COUNT {
                let equipped = e.call(BIPED_SLOT_ENTRY, &args![biped_data, inner]).u32();
                let cast = e
                    .call(
                        RT_DYNAMIC_CAST_FN,
                        &args![equipped, 0u32, CAST_SOURCE_TYPE, CAST_TARGET_TYPE, 0u32],
                    )
                    .u32();
                if cast == 0
                    || !e
                        .call(BIPED_FILLS_SLOT, &args![cast, slot, 0u32, 0u32])
                        .bool()
                    || e.call(FORM_TYPE, &args![equipped]).u32() == 0xc
                {
                    continue;
                }
                let list = e.call(GET_EXTRA_DATA_LIST_ADDRESS, &args![this]).u32();
                let changes = e.call(EXTRA_LIST_GET_CONTAINER_CHANGES, &args![list]).u32();
                let worn = e
                    .call(INVENTORY_WORN_ITEM, &args![changes, slot, 0u32])
                    .u32();
                let mut worn_extra = 0u32;
                if worn != 0
                    && e.call(WORN_FIRST_WORD, &args![worn]).u32() != 0
                    && worn_entry_word(e, worn) != 0
                {
                    let list = worn_entry_word(e, worn);
                    if e.call(EXTRA_LIST_CAN_NOT_WEAR, &args![list]).bool() {
                        blocked = true;
                    }
                    worn_extra = worn_entry_word(e, worn);
                }
                if !blocked {
                    let _ = actor_un_equip_object(
                        e,
                        this,
                        Ptr::new(equipped),
                        1,
                        Ptr::new(worn_extra),
                        0,
                        0,
                        1,
                    );
                    for removed_slot in 0..BIPED_SLOT_COUNT {
                        if e.call(BIPED_FILLS_SLOT, &args![cast, removed_slot, 0u32, 0u32])
                            .bool()
                        {
                            e.call(
                                BIPED_CLEAR_SLOT,
                                &args![biped_data, removed_slot, 1u32, 0u32],
                            );
                        }
                    }
                }
                if worn != 0 {
                    e.call(ITEM_RELEASE, &args![worn, 1u32]);
                }
            }
        }
    }
    let combat_object = e.mem.u32(COMBAT_OBJECT_POINTER);
    if blocked && !e.call(COMBAT_CONTROLLER_TEST, &args![combat_object]).bool() {
        return 0;
    }
    let process = e.get(this, Actor::pCurrentProcess);
    if !process.is_null() {
        match e.call(FORM_TYPE, &args![item]).u32() {
            FORM_TYPE_WEAPON | FORM_TYPE_0X33 => {
                let held = e.vcall(process.addr(), PROCESS_VSLOT_ITEM, &args![]).u32();
                if held == 0 {
                    if is_player {
                        e.vcall(process.addr(), PROCESS_VSLOT_0X458, &args![this, 0u32]);
                    }
                } else {
                    let mut held_blocks = false;
                    if e.call(ITEM_VALUE, &args![held]).u32() != item.addr()
                        && e.call(WORN_FIRST_WORD, &args![held]).u32() != 0
                        && worn_entry_word(e, held) != 0
                    {
                        let list = worn_entry_word(e, held);
                        if e.call(EXTRA_LIST_CAN_NOT_WEAR, &args![list]).bool() {
                            held_blocks = true;
                        }
                    }
                    let held_extra = worn_entry_word(e, held);
                    let held_item = e.call(ITEM_VALUE, &args![held]).u32();
                    let _ = actor_un_equip_object(
                        e,
                        this,
                        Ptr::new(held_item),
                        1,
                        Ptr::new(held_extra),
                        0,
                        0,
                        0,
                    );
                    if held_blocks && !e.call(COMBAT_CONTROLLER_TEST, &args![combat_object]).bool()
                    {
                        return 0;
                    }
                    if e.vcall(this.addr(), VSLOT_0X390, &args![]).u32() == 0 {
                        e.call(ACTOR_RELOAD_TARGETS, &args![this, 0u32]);
                    }
                }
                let mut mod_active = 0u32;
                if !extra.is_null() {
                    for mask in [1u32, 2, 4] {
                        if e.call(FORM_WEAPON_MOD_KIND, &args![item, mask]).u32() == 2
                            && e.call(EXTRA_LIST_WEAPON_MOD_SLOT_ACTIVE, &args![extra, mask])
                                .bool()
                        {
                            mod_active = 1;
                            break;
                        }
                    }
                }
                e.vcall(
                    this.addr(),
                    VSLOT_EQUIP_WEAPON_STEP,
                    &args![item, 0u32, mod_active, 1u32],
                );
            }
            FORM_TYPE_AMMO => {
                let held = e
                    .vcall(process.addr(), PROCESS_VSLOT_AMMO_ITEM, &args![])
                    .u32();
                if held != 0 && e.call(ITEM_VALUE, &args![held]).u32() != item.addr() {
                    let held_extra = worn_entry_word(e, held);
                    let held_item = e.call(ITEM_VALUE, &args![held]).u32();
                    let _ = actor_un_equip_object(
                        e,
                        this,
                        Ptr::new(held_item),
                        1,
                        Ptr::new(held_extra),
                        0,
                        0,
                        1,
                    );
                }
            }
            _ => {}
        }
    }
    let changes = e.call(INVENTORY_CHANGES_OF, &args![this]).u32();
    e.call(
        INVENTORY_APPLY_EQUIP,
        &args![changes, item, count, this, extra, 0u32, flag],
    );
    if e.vcall(this.addr(), VSLOT_TEST_0X218, &args![]).bool()
        && !is_player
        && e.call(FORM_TYPE, &args![item]).u32() != FORM_TYPE_WEAPON
    {
        let data = e.vcall(this.addr(), VSLOT_GET_BIPED, &args![]).u32();
        if data != 0 {
            let form = e.call(SAVE_FORM_BUFFER_GET_FORM, &args![this]).u32();
            e.call(NPC_INIT_WORN_OBJECT, &args![form, this, data, item]);
        }
    }
    e.call(FORM_TYPE, &args![item]);
    let enchanting = e.call(FORM_ENCHANTING, &args![item]).u32();
    if enchanting != 0 {
        e.call(MAGIC_ITEM_PRELOAD, &args![enchanting + 0x18, 0u32]);
    }
    if e.call(REFRACTION_CONDITION, &args![]).bool()
        && e.call(ACTOR_IS_REFRACTIVE, &args![this]).bool()
    {
        let list = e.call(GET_EXTRA_DATA_LIST_ADDRESS, &args![this]).u32();
        let property = e.call(EXTRA_LIST_REFRACTION, &args![list]).u32();
        let value = e.mem.f32(property + 0xc);
        e.vcall(this.addr(), VSLOT_0X384, &args![1u32, value]);
    }
    1
}

// Translated from 0088e0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The entry of slot `slot` of the biped data: the word at
/// `this + 0x2c + 16 * slot`.
pub fn fn_0088e0f0(e: &mut Engine, this: Ptr, slot: u32) -> u32 {
    e.mem.u32(this.addr() + 0x2c + (slot << 4))
}

// Translated from 0088e110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes `item` from the actor's container changes (`004c0cf0`) after
/// setting the removal action flag (unless the combat object test holds),
/// and unloads the item's enchantment. Returns 1 for a null item, else what
/// `004c0cf0` answers (0 without container changes). The map has no name
/// for it.
pub fn fn_0088e110(
    e: &mut Engine,
    this: Ptr<Actor>,
    item: Ptr,
    count: i32,
    extra: Ptr,
    worn: u8,
    flag: u8,
) -> u8 {
    if item.is_null() {
        return 1;
    }
    let mut result = 0u8;
    let list = e.call(GET_EXTRA_DATA_LIST_ADDRESS, &args![this]).u32();
    let changes = e.call(EXTRA_LIST_GET_CONTAINER_CHANGES, &args![list]).u32();
    if changes != 0 {
        let combat_object = e.mem.u32(COMBAT_OBJECT_POINTER);
        if !e.call(COMBAT_CONTROLLER_TEST, &args![combat_object]).bool() {
            e.call(SET_ACTION_FLAG, &args![this, extra, ACTION_FLAG_REMOVED]);
            let list = e.call(GET_EXTRA_DATA_LIST_ADDRESS, &args![this]).u32();
            e.call(SET_ACTION_FLAG, &args![item, list, ACTION_FLAG_REMOVED]);
        }
        result = e.with_stack(4, |e, local| {
            e.mem.set_u8(local.addr(), 1);
            e.call(
                INVENTORY_REMOVE,
                &args![changes, local, item, count, this, extra, worn, flag, 0u32, 0u32],
            )
            .u8()
        });
    }
    let enchanting = e.call(FORM_ENCHANTING, &args![item]).u32();
    if enchanting != 0 {
        e.call(MAGIC_ITEM_UNLOAD, &args![enchanting + 0x18, 1u32]);
    }
    result
}

// ---- small accessors and inventory queries (008905f0, 00891060 up to 00891d30) ----

/// `ECX` = vector, one stack word (a 16-byte vector): copies it into the
/// vector in `ECX`.
const VECTOR_ASSIGN: u32 = 0x004a_3c90;
/// `ECX` = vector, one stack word (the destination): fills it with the
/// first component of `ECX` in all four lanes; returns the destination.
const VECTOR_SPLAT_FIRST: u32 = 0x0056_1240;
/// `ECX` = flag word at the object's +0x10, stack (mask, set flag): sets or
/// clears the mask bits.
const FLAG_WORD_SET_OR_CLEAR: u32 = 0x0062_0760;
/// `ECX` = flag word, one stack word: the bits of the mask that are set.
const FLAG_WORD_AND: u32 = 0x008c_1be0;
/// A `dword` global returned by `00891350`.
const NODE_KEY_GLOBAL_011C61EC: u32 = 0x011c_61ec;
/// A flag byte that makes `fn_00891be0` answer 5.
const WEIGHT_OVERRIDE_FLAG: u32 = 0x0052_5420;
/// Actor virtual `+0x21c`: selects the actor-based weight in
/// `fn_00891be0`.
const VSLOT_WEIGHT_SOURCE: u32 = 0x21c;
/// `ECX` = form: a `float` in ST0 (the actor-based weight).
const FORM_ACTOR_WEIGHT: u32 = 0x0082_1660;
/// `TESWeightForm::GetFormWeight` (Xbox PDB), `cdecl` (form, flag), `float`
/// in ST0.
const FORM_WEIGHT: u32 = 0x0048_ebc0;
/// `ECX` = player: a flag byte `GetFormWeight` takes.
const PLAYER_WEIGHT_FLAG: u32 = 0x004d_1360;
/// `ECX` = changes, stack (base form, `float*`, item, 0): a query of the
/// inventory changes (`RET 0x10`).
const INVENTORY_QUERY_ITEM: u32 = 0x004c_7400;
/// `ECX` = changes, stack (base form, 0) (`RET 8`).
const INVENTORY_QUERY_BASE: u32 = 0x004c_6f60;
/// `InventoryChanges::GetBestFood` (Xbox PDB): `ECX` = the changes.
const INVENTORY_GET_BEST_FOOD: u32 = 0x004c_afe0;

// Translated from 008905f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at +0x2c of the object. The map has no name for it.
pub fn fn_008905f0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x2c)
}

// Translated from 00891060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `cdecl` (out, a, b): `out = (a.x * b.x, a.y, a.z, a.w)` (a `MULSS` on the
/// first lane of the vector `a`), stored through `004a3c90`; returns `out`.
/// The map has no name for it.
pub fn fn_00891060(e: &mut Engine, out: Ptr, a: Ptr, b: Ptr) -> Ptr {
    e.call(MATRIX_CONSTRUCT, &args![b]);
    let b_first = e.mem.f32(b.addr());
    e.call(MATRIX_CONSTRUCT, &args![a]);
    e.with_stack(0x20, |e, scratch| {
        let result = scratch.addr();
        let product = e.mem.f32(a.addr()) * b_first;
        e.mem.set_f32(result, product);
        for word in 1..4 {
            let value = e.mem.u32(a.addr() + 4 * word);
            e.mem.set_u32(result + 4 * word, value);
        }
        e.call(VECTOR_ASSIGN, &args![out, result]);
    });
    out
}

// Translated from 008910c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `this -= splat(x.first) * y`, lane by lane in `float`, for the four-lane
/// vectors `this`, `y` (the first stack word is `x`, the vector whose first
/// component is spread by `00561240`). The map has no name for it.
pub fn fn_008910c0(e: &mut Engine, this: Ptr, x: Ptr, y: Ptr) {
    e.with_stack(0x20, |e, scratch| {
        let spread = scratch.addr();
        e.call(VECTOR_SPLAT_FIRST, &args![x, spread]);
        for lane in 0..4 {
            let factor = e.mem.f32(spread + 4 * lane);
            let multiplier = e.mem.f32(y.addr() + 4 * lane);
            let current = e.mem.f32(this.addr() + 4 * lane);
            e.mem
                .set_f32(this.addr() + 4 * lane, current - factor * multiplier);
        }
    });
}

// Translated from 00891130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`flag` non-zero) or clears bit 1 of the flag word at +0x10.
pub fn fn_00891130(e: &mut Engine, this: Ptr, flag: u8) {
    e.call(
        FLAG_WORD_SET_OR_CLEAR,
        &args![this.addr() + 0x10, 1u32, flag],
    );
}

// Translated from 00891150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tells whether bit 1 of the flag word at +0x10 is set.
pub fn fn_00891150(e: &mut Engine, this: Ptr) -> bool {
    e.call(FLAG_WORD_AND, &args![this.addr() + 0x10, 1u32])
        .u32()
        != 0
}

// Translated from 00891170 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address `this + 0x20` (an embedded member).
pub fn fn_00891170(_e: &mut Engine, this: Ptr) -> u32 {
    this.addr() + 0x20
}

// Translated from 00891350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the `dword` global at `011c61ec`.
pub fn fn_00891350(e: &mut Engine) -> u32 {
    e.mem.u32(NODE_KEY_GLOBAL_011C61EC)
}

// Translated from 00891b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tells whether bit 3 of the byte at +0x100 is set.
pub fn fn_00891b70(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0x100) & 8 != 0
}

// Translated from 00891b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetArmorBeingWorn` (Xbox PDB): the worn item of slot `slot` in the
/// actor's container changes (`InventoryChanges::GetWornItem`), or 0
/// without changes.
pub fn actor_get_armor_being_worn(e: &mut Engine, this: Ptr<Actor>, slot: u32) -> u32 {
    let list = e.call(GET_EXTRA_DATA_LIST_ADDRESS, &args![this]).u32();
    let changes = e.call(EXTRA_LIST_GET_CONTAINER_CHANGES, &args![list]).u32();
    if changes == 0 {
        return 0;
    }
    e.call(INVENTORY_WORN_ITEM, &args![changes, slot, 1u32])
        .u32()
}

// Translated from 00891be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A weight as an integer: 5 when the flag byte `525420` reads true; else,
/// for an actor the virtual `+0x21c` selects, the truncated
/// `00821660` of its base form (0 without one); else, for a non-null
/// `item`, the truncated `GetFormWeight` of the item's value with the
/// player flag; else 0. The map has no name for it.
pub fn fn_00891be0(e: &mut Engine, this: Ptr<Actor>, item: Ptr) -> u32 {
    if e.call(WEIGHT_OVERRIDE_FLAG, &args![]).bool() {
        return 5;
    }
    let mut result = 0u32;
    if e.vcall(this.addr(), VSLOT_WEIGHT_SOURCE, &args![]).bool() {
        let form = e.call(SAVE_FORM_BUFFER_GET_FORM, &args![this]).u32();
        if form != 0 {
            let weight = e.call(FORM_ACTOR_WEIGHT, &args![form]).f64();
            result = e.call(FTOL2, &args![weight]).u32();
        }
    } else if !item.is_null() {
        let player = e.mem.u32(PLAYER_CHARACTER);
        let flag = e.call(PLAYER_WEIGHT_FLAG, &args![player]).u8() as u32;
        let value = e.call(ITEM_VALUE, &args![item]).u32();
        let weight = e.call(FORM_WEIGHT, &args![value, flag]).f64();
        result = e.call(FTOL2, &args![weight]).u32();
    }
    result
}

// Translated from 00891c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Asks the actor's container changes (`004c7400`) about `item`, with the
/// actor's base form; 0 without changes. The map has no name for it.
pub fn fn_00891c80(e: &mut Engine, this: Ptr<Actor>, item: Ptr) -> u32 {
    let list = e.call(GET_EXTRA_DATA_LIST_ADDRESS, &args![this]).u32();
    let changes = e.call(EXTRA_LIST_GET_CONTAINER_CHANGES, &args![list]).u32();
    if changes == 0 {
        return 0;
    }
    e.with_stack(4, |e, out| {
        e.mem.set_f32(out.addr(), 0.0);
        let base = e.call(BASE_FORM_OF, &args![this]).u32();
        e.call(INVENTORY_QUERY_ITEM, &args![changes, base, out, item, 0u32])
            .u32()
    })
}

// Translated from 00891ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Asks the actor's container changes (`004c6f60`) with the actor's base
/// form; 0 without changes. The map has no name for it.
pub fn fn_00891ce0(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    let list = e.call(GET_EXTRA_DATA_LIST_ADDRESS, &args![this]).u32();
    let changes = e.call(EXTRA_LIST_GET_CONTAINER_CHANGES, &args![list]).u32();
    if changes == 0 {
        return 0;
    }
    let base = e.call(BASE_FORM_OF, &args![this]).u32();
    e.call(INVENTORY_QUERY_BASE, &args![changes, base, 0u32])
        .u32()
}

// Translated from 00891d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetBestFoodItem` (Xbox PDB): `InventoryChanges::GetBestFood` of
/// the actor's container changes, or 0 without changes.
pub fn actor_get_best_food_item(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    let list = e.call(GET_EXTRA_DATA_LIST_ADDRESS, &args![this]).u32();
    let changes = e.call(EXTRA_LIST_GET_CONTAINER_CHANGES, &args![list]).u32();
    if changes == 0 {
        return 0;
    }
    e.call(INVENTORY_GET_BEST_FOOD, &args![changes]).u32()
}

// ---- weapon condition and equipment damage (00891190, 00891360) ----

/// `Actor::IsWeaponDrawn` (Xbox PDB): `ECX` = actor.
const ACTOR_IS_WEAPON_DRAWN: u32 = 0x008a_16d0;
/// `PlayerCharacter` call (`ECX` = player, one stack word, 0 or 1): the 3D
/// root object the weapon condition code walks.
const PLAYER_ROOT_OBJECT: u32 = 0x0095_0bb0;
/// Process virtual `+0x368`: (actor) the condition number of the weapon.
const PROCESS_VSLOT_WEAPON_CONDITION: u32 = 0x368;
/// Process virtual `+0x788`: the condition number last applied.
const PROCESS_VSLOT_LAST_CONDITION: u32 = 0x788;
/// Process virtual `+0x78c`: (condition) stores the applied condition.
const PROCESS_VSLOT_SET_LAST_CONDITION: u32 = 0x78c;
/// Process virtual `+0x6b8`: an object whose damage stage nodes follow the
/// condition.
const PROCESS_VSLOT_DAMAGE_STAGE_OBJECT: u32 = 0x6b8;
/// `ECX` = the process virtual `+0x6b8` object: the destructible form behind
/// it.
const DESTRUCTIBLE_OF_OBJECT: u32 = 0x0043_b230;
/// `BGSDestructibleObjectForm::UpdateDamageStageNodes` (Xbox PDB), `cdecl`:
/// (node or form, condition).
const UPDATE_DAMAGE_STAGE_NODES: u32 = 0x0047_6ec0;
/// `cdecl` (root object, `dword`): finds a node of the root.
const FIND_NODE_BY_VALUE: u32 = 0x004a_de00;
/// `cdecl` (root object, name): finds a node by name.
const FIND_NODE_BY_NAME: u32 = 0x004a_ae30;
/// The node name `Backpack`.
const BACKPACK_NAME: u32 = 0x0101_f5cc;
/// `ItemChange::GetItemHealth` (Xbox PDB): `ECX` = the item change, one
/// stack word (0 for the health value, 1 for the percentage), `float` in
/// ST0.
const ITEM_GET_HEALTH: u32 = 0x004b_cdb0;
/// `ItemChange::SetItemHealth` (Xbox PDB): `ECX` = the item change, stack
/// (`float` health, container changes, extra data, 1).
const ITEM_SET_HEALTH: u32 = 0x004b_d030;
/// `ECX` = weapon, one stack word (the actor): the ammo form the weapon
/// uses for that actor, or the actor's word at +0x118.
const WEAPON_AMMO_FOR_ACTOR: u32 = 0x0052_5a90;
/// `ECX` = ammo form: a flag test (`GetFormFlag(2)`).
const AMMO_FLAG_TEST: u32 = 0x004f_d360;
/// `ECX` = ammo form: its word at +0x84.
const AMMO_PROJECTILE: u32 = 0x004f_d3c0;
/// `ECX` = weapon: tests bit 0x80 of the byte at +0x100 (true when clear).
const WEAPON_FLAG_NOT_80: u32 = 0x0047_bcf0;
/// `ECX` = weapon: tests bit 0x20 of the byte at +0x100.
const WEAPON_FLAG_20: u32 = 0x0046_e8c0;
/// No arguments, `bool`: whether `DamageEquipment` writes its debug line.
const DAMAGE_DEBUG_FLAG: u32 = 0x005b_6f70;
/// `cdecl`: formats and prints a line (format, then its arguments).
const PRINT_LINE: u32 = 0x0070_3c00;
/// `MapMarkerData::GetLocationName` (Xbox PDB, folded): `ECX` = armor + 0x30.
const ARMOR_LOCATION_NAME: u32 = 0x0040_8da0;
/// The format `"%.20s's %s takes %.2f points of damage (%.2f/%.2f)!"`.
const DAMAGE_LINE_FORMAT: u32 = 0x0108_49cc;
/// `double` (25.0): the percentage below which `DamageEquipment` warns the
/// player.
const PERCENTAGE_LIMIT: u32 = 0x0104_f2f0;
/// The text address the damage messages pass.
const DAMAGE_MESSAGE_TEXT: u32 = 0x0108_49c0;
/// `float` (the message duration of the damage messages).
const DAMAGE_MESSAGE_DURATION: u32 = 0x0101_e570;
/// Message setting object shown when the weapon drops below 25 percent.
const WEAPON_DAMAGED_MESSAGE: u32 = 0x011d_45c4;
/// Message setting object shown when the armor drops below 25 percent.
const ARMOR_DAMAGED_MESSAGE: u32 = 0x011d_3234;
/// Message setting object shown when the player's weapon is destroyed.
const WEAPON_DESTROYED_MESSAGE: u32 = 0x011d_291c;
/// `BSAudio::StopMovingSounds` (Xbox PDB): `ECX` = audio, stack (root, `float`
/// 0.0, 0).
const AUDIO_STOP_MOVING_SOUNDS: u32 = 0x00ad_8570;
/// `VATS::QuitVATSPlayback_ov2` (Xbox PDB): `ECX` = the VATS object, stack
/// (form, extra data).
const VATS_QUIT_PLAYBACK: u32 = 0x009c_8a90;
/// `ECX` = object: the address of its matrix (`this + 0x68`).
const OBJECT_MATRIX: u32 = 0x0046_1130;
/// The identity matrix (nine `float`s).
const IDENTITY_MATRIX_ADDRESS: u32 = 0x011a_9448;
/// `NiMatrix3::ToEulerAnglesZXY` (Xbox PDB): `ECX` = matrix, stack (three
/// `float*`).
const MATRIX_TO_EULER_ZXY: u32 = 0x00a5_9400;
/// `cdecl` (projectile, 0, 0, cell, position x, y, z, matrix of nine
/// words): launches the ammo's projectile from a destroyed explosive weapon.
const LAUNCH_PROJECTILE: u32 = 0x009a_c9c0;
/// `cdecl` (actor, word): a step after the actor's virtual `+0x3cc` placed
/// the dropped weapon.
const ACTOR_AFTER_DROP: u32 = 0x0056_acb0;
/// `ECX` = combat controller, one stack word.
const COMBAT_CONTROLLER_STEP: u32 = 0x0097_f6d0;
/// Actor virtual `+0x3cc`: (form, extra data, 1, position, euler out) places
/// a dropped item; returns a word.
const VSLOT_PLACE_DROPPED_ITEM: u32 = 0x3cc;
/// Actor virtual `+0x1f4`: the actor's position.
const VSLOT_POSITION_0X1F4: u32 = 0x1f4;
/// Process virtual `+0x190`, `+0x198`: (biped data) the object the dropped
/// weapon (or armor) comes from.
const PROCESS_VSLOT_DROP_SOURCE_WEAPON: u32 = 0x190;
const PROCESS_VSLOT_DROP_SOURCE_ARMOR: u32 = 0x198;
/// Process virtual `+0x450`: one word.
const PROCESS_VSLOT_0X450: u32 = 0x450;
/// Entry point number `DamageEquipment` hands to `HandleEntryPoint`.
const ENTRY_POINT_EQUIPMENT_DAMAGE: u32 = 0x44;
/// Object-type numbers of the acquired object that stop the damage.
const OBJECT_TYPE_NO_DAMAGE_A: u32 = 8;
const OBJECT_TYPE_NO_DAMAGE_B: u32 = 0x10;

// Translated from 00891190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::UpdateWeaponConditionEffects` (Xbox PDB): with a process and a
/// current weapon, asks the process for the weapon condition (virtual
/// `+0x368`); unless `force` it stops when that equals the last one applied
/// (`+0x788`). Otherwise stores it (`+0x78c`) and updates the damage stage
/// nodes of the process object, of the actor's 3D root (and its `Backpack`
/// node) and, for the player, of the second root.
pub fn actor_update_weapon_condition_effects(e: &mut Engine, this: Ptr<Actor>, force: u8) {
    let process = e.get(this, Actor::pCurrentProcess);
    if process.is_null() {
        return;
    }
    if e.call(ACTOR_GET_CURRENT_WEAPON, &args![this]).u32() == 0 {
        return;
    }
    let condition = e
        .vcall(process.addr(), PROCESS_VSLOT_WEAPON_CONDITION, &args![this])
        .u32();
    if force == 0
        && e.vcall(process.addr(), PROCESS_VSLOT_LAST_CONDITION, &args![])
            .u32()
            == condition
    {
        return;
    }
    e.vcall(
        process.addr(),
        PROCESS_VSLOT_SET_LAST_CONDITION,
        &args![condition],
    );
    let stage_object = e
        .vcall(process.addr(), PROCESS_VSLOT_DAMAGE_STAGE_OBJECT, &args![])
        .u32();
    if stage_object != 0 {
        let destructible = e.call(DESTRUCTIBLE_OF_OBJECT, &args![stage_object]).u32();
        e.call(UPDATE_DAMAGE_STAGE_NODES, &args![destructible, condition]);
    }
    let player = e.mem.u32(PLAYER_CHARACTER);
    let root = if this.addr() == player {
        e.call(PLAYER_ROOT_OBJECT, &args![player, 0u32]).u32()
    } else {
        e.vcall(this.addr(), VSLOT_GET_3D, &args![]).u32()
    };
    update_condition_nodes(e, root, condition);
    if this.addr() == player {
        let second_root = e.call(PLAYER_ROOT_OBJECT, &args![player, 1u32]).u32();
        update_condition_nodes(e, second_root, condition);
    }
}

/// Looks up the node `00891350` names and the node `Backpack` under `root`
/// and updates their damage stage nodes (the game passes the first node for
/// both updates, which is kept).
fn update_condition_nodes(e: &mut Engine, root: u32, condition: u32) {
    let key = fn_00891350(e);
    let node = e.call(FIND_NODE_BY_VALUE, &args![root, key]).u32();
    if node != 0 {
        e.call(UPDATE_DAMAGE_STAGE_NODES, &args![node, condition]);
    }
    let backpack = e.call(FIND_NODE_BY_NAME, &args![root, BACKPACK_NAME]).u32();
    if backpack != 0 {
        e.call(UPDATE_DAMAGE_STAGE_NODES, &args![node, condition]);
    }
}

/// Shows a damage message: `CREATE_MESSAGE` with (2, 0, text, duration, 0).
fn show_damage_message(e: &mut Engine, setting: u32) {
    let duration: f32 = e.global(DAMAGE_MESSAGE_DURATION);
    // 0089167d: `00403df0` takes only the setting (ECX); the five pushed words
    // are the rest of the cdecl `007052f0` (ADD ESP,0x18).
    let message = e.call(CREATE_MESSAGE, &args![setting]).u32();
    e.call(
        SHOW_MESSAGE,
        &args![message, 2u32, 0u32, DAMAGE_MESSAGE_TEXT, duration, 0u32],
    );
}

// Translated from 00891360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::DamageEquipment` (Xbox PDB): applies `amount` of damage (after the
/// entry point `0x44`) to the item change `entry` of an equipped weapon or
/// armor. Returns 0 when nothing is damaged (a null entry, no damage, an
/// acquired object of type 8 or 0x10, a weapon whose ammo rules forbid it)
/// or the item survives; 1 when the item is gone (then a weapon is dropped,
/// thrown or its projectile launched, or the item is removed, and the
/// player is told). The third stack word is not read.
pub fn actor_damage_equipment(
    e: &mut Engine,
    this: Ptr<Actor>,
    entry: Ptr,
    amount: f32,
    _unused_2: u32,
) -> u8 {
    let zero: f64 = e.global(DOUBLE_ZERO);
    if entry.is_null() || (amount as f64) <= zero {
        return 0;
    }
    let process = e.call(GET_PROCESS, &args![this]).u32();
    let acquired = if process != 0 {
        e.vcall(process, PROCESS_VSLOT_0X27C, &args![]).u32()
    } else {
        0
    };
    if acquired != 0
        && (e.call(OBJECT_TYPE, &args![acquired]).u32() == OBJECT_TYPE_NO_DAMAGE_A
            || e.call(OBJECT_TYPE, &args![acquired]).u32() == OBJECT_TYPE_NO_DAMAGE_B)
    {
        return 0;
    }
    let mut weapon = 0u32;
    if e.call(ITEM_VALUE, &args![entry]).u32() != 0 {
        let form = e.call(ITEM_VALUE, &args![entry]).u32();
        if e.call(FORM_TYPE, &args![form]).u32() == FORM_TYPE_WEAPON {
            weapon = e.call(ITEM_VALUE, &args![entry]).u32();
        }
    }
    if weapon != 0
        && e.vcall(this.addr(), VSLOT_ACTOR_FLAG_0X360, &args![])
            .bool()
        && e.call(FORM_FLAG_TEST_4C0BF0, &args![weapon]).bool()
        && e.call(WEAPON_AMMO_FOR_ACTOR, &args![weapon, this]).u32() != 0
    {
        let ammo = e.call(WEAPON_AMMO_FOR_ACTOR, &args![weapon, this]).u32();
        if e.call(AMMO_FLAG_TEST, &args![ammo]).bool() {
            return 0;
        }
    }
    if weapon != 0 && !e.call(WEAPON_FLAG_NOT_80, &args![weapon]).bool() {
        return 0;
    }
    let mut damage = e.with_stack(4, |e, local| {
        e.mem.set_f32(local.addr(), amount);
        e.call(
            HANDLE_ENTRY_POINT,
            &args![ENTRY_POINT_EQUIPMENT_DAMAGE, this, local],
        );
        e.mem.f32(local.addr())
    });
    if weapon != 0
        && !e
            .vcall(this.addr(), VSLOT_ACTOR_FLAG_0X360, &args![])
            .bool()
        && e.call(FORM_FLAG_TEST_4C0BF0, &args![weapon]).bool()
        && e.call(WEAPON_AMMO_FOR_ACTOR, &args![weapon, this]).u32() != 0
    {
        let ammo = e.call(WEAPON_AMMO_FOR_ACTOR, &args![weapon, this]).u32();
        if e.call(AMMO_FLAG_TEST, &args![ammo]).bool() {
            damage = e.call(ITEM_GET_HEALTH, &args![entry, 0u32]).f32();
        }
    }
    let mut armor = 0u32;
    if e.call(ITEM_VALUE, &args![entry]).u32() != 0 {
        let form = e.call(ITEM_VALUE, &args![entry]).u32();
        if e.call(FORM_TYPE, &args![form]).u32() == FORM_TYPE_ARMOR {
            armor = e.call(ITEM_VALUE, &args![entry]).u32();
        }
    }
    let health_before = e.call(ITEM_GET_HEALTH, &args![entry, 0u32]).f32();
    let mut remaining = (health_before as f64 - damage as f64) as f32;
    let one: f64 = e.global(DOUBLE_ONE);
    if (remaining as f64) < one {
        remaining = 0.0;
    }
    if e.call(DAMAGE_DEBUG_FLAG, &args![]).bool() && armor != 0 {
        let health = e.call(ITEM_GET_HEALTH, &args![entry, 0u32]).f64();
        let location = e.call(ARMOR_LOCATION_NAME, &args![armor + 0x30]).u32();
        let name = e.call(REFERENCE_FULL_NAME, &args![this]).u32();
        e.call(
            PRINT_LINE,
            &args![
                DAMAGE_LINE_FORMAT,
                name,
                location,
                damage as f64,
                remaining as f64,
                health
            ],
        );
    }
    e.vcall(this.addr(), VSLOT_ACTOR_FLAG_0X360, &args![]);
    let health_percentage = e.call(ITEM_GET_HEALTH, &args![entry, 1u32]).f32();
    let keeps_item = weapon != 0 && e.call(FORM_FLAG_TEST_4C0BF0, &args![weapon]).bool();
    if !keeps_item {
        let extra = worn_entry_word(e, entry.addr());
        let list = e.call(GET_EXTRA_DATA_LIST_ADDRESS, &args![this]).u32();
        let changes = e.call(EXTRA_LIST_GET_CONTAINER_CHANGES, &args![list]).u32();
        e.call(
            ITEM_SET_HEALTH,
            &args![entry, remaining, changes, extra, 1u32],
        );
        let limit: f64 = e.global(PERCENTAGE_LIMIT);
        if health_percentage as f64 >= limit
            && (e.call(ITEM_GET_HEALTH, &args![entry, 1u32]).f32() as f64) < limit
        {
            if weapon != 0
                && e.vcall(this.addr(), VSLOT_ACTOR_FLAG_0X360, &args![])
                    .bool()
            {
                show_damage_message(e, WEAPON_DAMAGED_MESSAGE);
            } else if armor != 0
                && e.vcall(this.addr(), VSLOT_ACTOR_FLAG_0X360, &args![])
                    .bool()
            {
                show_damage_message(e, ARMOR_DAMAGED_MESSAGE);
            }
        }
        if armor != 0 {
            e.vcall(this.addr(), VSLOT_AFTER_EQUIP_CHANGE, &args![]);
        }
        if weapon != 0 {
            e.call(ACTOR_EQUIP_STEP_A, &args![this]);
        }
        actor_update_weapon_condition_effects(e, this, 0);
        if remaining as f64 > zero {
            return 0;
        }
    }
    if weapon == 0 {
        return 1;
    }
    let destroyed_at_once = e
        .vcall(this.addr(), VSLOT_ACTOR_FLAG_0X360, &args![])
        .bool()
        || fn_00891b70(e, Ptr::new(weapon))
        || {
            let form = e.call(ITEM_VALUE, &args![entry]).u32();
            e.vcall(form, FORM_VSLOT_0X94, &args![]).bool()
        }
        || e.call(WEAPON_FLAG_20, &args![weapon]).bool();
    if destroyed_at_once {
        remove_destroyed_weapon(e, this, entry, weapon);
        return 1;
    }
    drop_destroyed_weapon(e, this, entry, weapon, armor);
    1
}

/// The end of `DamageEquipment` for a weapon that is gone and must vanish:
/// the player is told, the moving sounds of the actor stop, and either the
/// VATS playback ends (the player's weapon, with the VATS object in state 4)
/// or the weapon is unequipped.
fn remove_destroyed_weapon(e: &mut Engine, this: Ptr<Actor>, entry: Ptr, weapon: u32) {
    let player = e.mem.u32(PLAYER_CHARACTER);
    if this.addr() == player {
        let root = e.call(PLAYER_ROOT_OBJECT, &args![player, 1u32]).u32();
        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
        e.call(AUDIO_STOP_MOVING_SOUNDS, &args![audio, root, 0.0f32, 0u32]);
        show_damage_message(e, WEAPON_DESTROYED_MESSAGE);
    }
    let root = e.call(BLEND_OBJECT_OF, &args![this]).u32();
    let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
    e.call(AUDIO_STOP_MOVING_SOUNDS, &args![audio, root, 0.0f32, 0u32]);
    if e.vcall(this.addr(), VSLOT_ACTOR_FLAG_0X360, &args![])
        .bool()
        && weapon != 0
        && e.call(ITEM_VALUE, &args![VATS_STATE_OBJECT]).u32() == 4
    {
        let extra = worn_entry_word(e, entry.addr());
        let form = e.call(ITEM_VALUE, &args![entry]).u32();
        e.call(VATS_QUIT_PLAYBACK, &args![VATS_STATE_OBJECT, form, extra]);
        return;
    }
    let extra = worn_entry_word(e, entry.addr());
    let form = e.call(ITEM_VALUE, &args![entry]).u32();
    actor_un_equip_object(e, this, Ptr::new(form), 1, Ptr::new(extra), 0, 1, 1);
}

/// The branch of `DamageEquipment` where the destroyed weapon stays in the
/// world: it takes the position and matrix of the process's drop source (or
/// the actor's own), and either launches the ammo's projectile (explosive
/// weapons) or places the dropped item (when the weapon is drawn) and tells
/// the follower code; a drawn weapon is finally put away.
fn drop_destroyed_weapon(e: &mut Engine, this: Ptr<Actor>, entry: Ptr, weapon: u32, armor: u32) {
    e.with_stack(0x70, |e, frame| {
        // The game's locals: Euler angles (3 words), the matrix (9 words)
        // and the position (3 words).
        let euler = frame.addr();
        let matrix = frame.addr() + 0x0c;
        let position = frame.addr() + 0x30;
        let source_root = e.vcall(this.addr(), VSLOT_GET_BIPED, &args![]).u32();
        let process = e.get(this, Actor::pCurrentProcess);
        let source = if armor != 0 {
            e.vcall(
                process.addr(),
                PROCESS_VSLOT_DROP_SOURCE_ARMOR,
                &args![source_root],
            )
            .u32()
        } else {
            e.vcall(
                process.addr(),
                PROCESS_VSLOT_DROP_SOURCE_WEAPON,
                &args![source_root],
            )
            .u32()
        };
        let source_position = if source != 0 {
            e.call(OBJECT_POSITION_0X8C, &args![source]).u32()
        } else {
            e.vcall(this.addr(), VSLOT_POSITION_0X1F4, &args![]).u32()
        };
        for word in 0..3 {
            let value = e.mem.u32(source_position + 4 * word);
            e.mem.set_u32(position + 4 * word, value);
        }
        let source_matrix = if source != 0 {
            e.call(OBJECT_MATRIX, &args![source]).u32()
        } else {
            IDENTITY_MATRIX_ADDRESS
        };
        for word in 0..9 {
            let value = e.mem.u32(source_matrix + 4 * word);
            e.mem.set_u32(matrix + 4 * word, value);
        }
        let explosive = e.call(FORM_FLAG_TEST_4C0BF0, &args![weapon]).bool()
            && e.call(WEAPON_AMMO_FOR_ACTOR, &args![weapon, this]).u32() != 0
            && {
                let ammo = e.call(WEAPON_AMMO_FOR_ACTOR, &args![weapon, this]).u32();
                e.call(AMMO_FLAG_TEST, &args![ammo]).bool()
            };
        if explosive {
            let extra = worn_entry_word(e, entry.addr());
            let form = e.call(ITEM_VALUE, &args![entry]).u32();
            e.vcall(
                this.addr(),
                VSLOT_USE_ITEM_0X17C,
                &args![form, extra, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32],
            );
            let cell = e.call(GET_PARENT_CELL, &args![this]).u32();
            let ammo = e.call(WEAPON_AMMO_FOR_ACTOR, &args![weapon, this]).u32();
            let projectile = e.call(AMMO_PROJECTILE, &args![ammo]).u32();
            let mut words = vec![projectile, 0, 0, cell];
            for word in 0..3 {
                words.push(e.mem.u32(position + 4 * word));
            }
            for word in 0..9 {
                words.push(e.mem.u32(matrix + 4 * word));
            }
            e.call(LAUNCH_PROJECTILE, &words);
        } else if e.call(ACTOR_IS_WEAPON_DRAWN, &args![this]).bool() {
            e.call(MATRIX_CONSTRUCT, &args![euler]);
            e.call(
                MATRIX_TO_EULER_ZXY,
                &args![matrix, euler + 8, euler, euler + 4],
            );
            let extra = worn_entry_word(e, entry.addr());
            let form = e.call(ITEM_VALUE, &args![entry]).u32();
            let placed = e
                .vcall(
                    this.addr(),
                    VSLOT_PLACE_DROPPED_ITEM,
                    &args![form, extra, 1u32, position, euler],
                )
                .u32();
            e.call(ACTOR_AFTER_DROP, &args![this, placed]);
            let controller = e
                .vcall(this.addr(), VSLOT_GET_COMBAT_CONTROLLER, &args![])
                .u32();
            if controller != 0 {
                e.call(COMBAT_CONTROLLER_STEP, &args![controller, 0u32]);
            }
            if e.call(GET_FORCE_NEXT_UPDATE, &args![this]).bool() {
                e.call(TRIGGER_FOLLOWER_BARK, &args![this, 8u32]);
            }
        }
        if e.call(ACTOR_IS_WEAPON_DRAWN, &args![this]).bool() {
            let process = e.get(this, Actor::pCurrentProcess);
            e.vcall(process.addr(), PROCESS_VSLOT_0X450, &args![0u32]);
            let animation = e.vcall(this.addr(), VSLOT_GET_ANIMATION, &args![]).u32();
            if animation != 0 {
                e.call(ANIMATION_BLEND_OUT, &args![animation, 4u32, 0u32]);
                e.call(ANIMATION_BLEND_OUT, &args![animation, 2u32, 0u32]);
            }
        }
    });
}

// ---- impact sounds (0088e1e0) ----

/// `ECX` = actor: a flag byte that adds the secondary impact sounds.
const IMPACT_SECONDARY_FLAG: u32 = 0x008b_a410;
/// No arguments: the listener object whose position `0045bb80` reads.
const LISTENER_OBJECT: u32 = 0x0052_4c90;
/// `ECX` = process: the object `fn_0088e1e0` takes the sound set from
/// (named `MiddleHighProcess::GetFaceSkinnedNode` in the map, folded).
const IMPACT_SOUND_SOURCE: u32 = 0x008d_8ac0;
/// `ECX` = a sound set, one stack word (the material number): its entry.
const SOUND_SET_ENTRY: u32 = 0x0058_e9d0;
/// `ECX` = a sound set entry: the first sound.
const SOUND_ENTRY_FIRST: u32 = 0x0068_a810;
/// `ECX` = a sound set entry: the second sound.
const SOUND_ENTRY_SECOND: u32 = 0x0068_a830;
/// `ECX` = a creature form: its sound set.
const CREATURE_SOUND_SET: u32 = 0x005f_9b30;
/// `ECX` = a sound, one stack word (the output): its record, whose byte at
/// +1 is the distance in hundreds of units the sound is heard at.
const SOUND_RECORD: u32 = 0x004e_75d0;
/// The global word `fn_0088e1e0` uses as the sound source when the impact
/// names no reference and the target is not a creature.
const IMPACT_DEFAULT_SOURCE: u32 = 0x011c_a278;
/// The sound flags the impact sounds are played with.
const IMPACT_SOUND_FLAGS: u32 = 0x4102;
/// Virtual `+0x21c` of the impact target: a true answer selects the creature
/// sounds.
const VSLOT_TARGET_TEST_0X21C: u32 = 0x21c;
/// Virtual `+0x58` of `base form + 0x30`: the material number.
const BASE_FORM_PART_VSLOT_MATERIAL: u32 = 0x58;
/// Material number used for the secondary impact sounds.
const SECONDARY_MATERIAL: u32 = 4;

/// Plays `sound` through the handle at `handle` at `position` when its
/// record's distance (the byte at +1 times 100) exceeds `distance`.
fn play_impact_sound(
    e: &mut Engine,
    handle: u32,
    scratch: u32,
    sound: u32,
    distance: f32,
    position: [u32; 3],
) {
    let record = e.call(SOUND_RECORD, &args![sound, scratch]).u32();
    let limit = e.mem.u8(record + 1) as i32 * 100;
    let in_range = (distance as f64) < limit as f64;
    if !in_range {
        return;
    }
    let name = e.call(WEAPON_SOUND_NAME, &args![sound]).u32();
    let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
    let made = e
        .call(
            AUDIO_GET_SOUND_HANDLE_BY_FILENAME,
            &args![audio, scratch + 0x30, name, IMPACT_SOUND_FLAGS, sound],
        )
        .u32();
    e.call(SOUND_HANDLE_ASSIGN, &args![handle, made]);
    e.call(EMPTY_FUNCTION, &args![scratch + 0x30]);
    e.call(
        SOUND_HANDLE_SET_POSITION,
        &args![
            handle,
            f32::from_bits(position[0]),
            f32::from_bits(position[1]),
            f32::from_bits(position[2])
        ],
    );
    e.call(SOUND_HANDLE_PLAY, &args![handle, 0u32]);
}

// Translated from 0088e1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Plays the impact sounds of a hit described by `impact` (the target
/// reference at +0, the attacker at +4, the process object at +8, an
/// explicit sound source at +0x30, the position at +0x38). The sound set
/// comes from the explicit source, from the target when it is a creature
/// (the creature sound number 9 and its set), or from the global default;
/// the sounds are played at the hit position when the distance to the
/// listener is below the sound's range. With the secondary flag the
/// material-4 sounds of the same set follow. The map has no name for it;
/// the SEH frame is not translated.
pub fn fn_0088e1e0(e: &mut Engine, this: Ptr<Actor>, impact: Ptr) {
    let player = e.mem.u32(PLAYER_CHARACTER);
    let hit = impact.addr();
    let secondary = e.call(IMPACT_SECONDARY_FLAG, &args![this]).u8() != 0;
    e.with_stack(0x100, |e, frame| {
        let position = frame.addr();
        let direction = frame.addr() + 0x10;
        let difference = frame.addr() + 0x20;
        let handle = frame.addr() + 0x30;
        let scratch = frame.addr() + 0x40;
        for word in 0..3 {
            let value = e.mem.u32(hit + 0x38 + 4 * word);
            e.mem.set_u32(position + 4 * word, value);
            let value = e.mem.u32(SCALE_VECTOR + 4 * word);
            e.mem.set_u32(direction + 4 * word, value);
        }
        let mut distance = 0.0f32;
        if e.mem.u32(hit + 4) != player {
            let listener = e.call(LISTENER_OBJECT, &args![]).u32();
            let listener_position = e.call(OBJECT_POSITION_0X8C, &args![listener]).u32();
            let result = e
                .call(
                    VECTOR_MINUS_VECTOR,
                    &args![position, difference, listener_position],
                )
                .u32();
            for word in 0..3 {
                let value = e.mem.u32(result + 4 * word);
                e.mem.set_u32(direction + 4 * word, value);
            }
            distance = e.call(VECTOR_LENGTH, &args![direction]).f32();
        } else {
            let player_position = e.vcall(player, VSLOT_POSITION_0X1F4, &args![]).u32();
            for word in 0..3 {
                let value = e.mem.u32(player_position + 4 * word);
                e.mem.set_u32(position + 4 * word, value);
            }
        }
        let base = e.call(BASE_FORM_OF, &args![this]).u32();
        let material = e
            .vcall(base + 0x30, BASE_FORM_PART_VSLOT_MATERIAL, &args![])
            .u32();
        let mut source = 0u32;
        let mut is_creature = false;
        e.call(SOUND_HANDLE_INIT, &args![handle]);
        let target = e.mem.u32(hit);
        let explicit_source = e.mem.u32(hit + 0x30);
        if explicit_source != 0 {
            source = explicit_source;
        } else if target != 0 && e.vcall(target, VSLOT_TARGET_TEST_0X21C, &args![]).bool() {
            is_creature = true;
        } else if target != 0 && !e.vcall(target, VSLOT_TARGET_TEST_0X21C, &args![]).bool() {
            source = e.mem.u32(IMPACT_DEFAULT_SOURCE);
        } else {
            e.call(EMPTY_FUNCTION, &args![handle]);
            return;
        }
        let mut set_entry = 0u32;
        let mut first = 0u32;
        let mut second = 0u32;
        let mut third = 0u32;
        let mut sound_set = 0u32;
        if is_creature {
            let form = e.call(SAVE_FORM_BUFFER_GET_FORM, &args![target]).u32();
            third = if form != 0 {
                e.call(PICK_CREATURE_SOUND, &args![form, 9u32]).u32()
            } else {
                0
            };
            sound_set = if form != 0 {
                e.call(CREATURE_SOUND_SET, &args![form]).u32()
            } else {
                0
            };
            set_entry = if sound_set != 0 {
                e.call(SOUND_SET_ENTRY, &args![sound_set, material]).u32()
            } else {
                0
            };
        } else if source != 0 {
            sound_set = e.call(IMPACT_SOUND_SOURCE, &args![source]).u32();
            set_entry = if sound_set != 0 {
                e.call(SOUND_SET_ENTRY, &args![sound_set, material]).u32()
            } else {
                0
            };
        }
        if set_entry != 0 {
            first = e.call(SOUND_ENTRY_FIRST, &args![set_entry]).u32();
            second = e.call(SOUND_ENTRY_SECOND, &args![set_entry]).u32();
        }
        let position_words = [
            e.mem.u32(position),
            e.mem.u32(position + 4),
            e.mem.u32(position + 8),
        ];
        for sound in [first, second, third] {
            if sound != 0 {
                play_impact_sound(e, handle, scratch, sound, distance, position_words);
            }
        }
        if secondary {
            set_entry = if sound_set != 0 {
                e.call(SOUND_SET_ENTRY, &args![sound_set, SECONDARY_MATERIAL])
                    .u32()
            } else {
                0
            };
            if set_entry != 0 {
                first = e.call(SOUND_ENTRY_FIRST, &args![set_entry]).u32();
                if first != 0 {
                    play_impact_sound(e, handle, scratch, first, distance, position_words);
                }
                second = e.call(SOUND_ENTRY_SECOND, &args![set_entry]).u32();
                if second != 0 {
                    play_impact_sound(e, handle, scratch, second, distance, position_words);
                }
            }
        }
        e.call(EMPTY_FUNCTION, &args![handle]);
    });
}

// ---- hit effects (0088e8d0) ----

/// `cdecl` scope timer constructor: `ECX` = the timer, stack (0x32, 1, source
/// file, line); its destructor is `TIMER_SCOPE_DESTROY`.
const TIMER_SCOPE_CONSTRUCT: u32 = 0x0040_4eb0;
/// `ECX` = the scope timer: destructor.
const TIMER_SCOPE_DESTROY: u32 = 0x0040_4ee0;
/// The source file name string `...\Fallout\AI\Actor.cpp`.
const ACTOR_SOURCE_FILE_NAME: u32 = 0x0108_4918;
/// `ECX` = actor: a flag byte (the variant of `008ba410` for the hits of
/// types 1 and 2).
const HIT_FLAG_MELEE: u32 = 0x008b_a470;
/// `ECX` = actor (`Actor` code at `0087f260`): must be true for hit effects.
const HIT_EFFECTS_ALLOWED: u32 = 0x0087_f260;
/// `cdecl`, one `float`: a whole number of points from a damage value.
const DAMAGE_POINTS: u32 = 0x0064_7ae0;
/// `cdecl` (points, `float` 0.0, 0): a player statistic step.
const PLAYER_STATISTIC_STEP: u32 = 0x004e_0330;
/// `ECX` = the combat manager, stack (0, 0): the number of combatants.
const COMBATANT_COUNT: u32 = 0x0099_31c0;
/// The pointer to the combat manager.
const COMBAT_MANAGER_POINTER: u32 = 0x011f_1958;
/// Setting object whose word (`0043d4d0`) is the combatant limit.
const COMBATANT_LIMIT_OBJECT: u32 = 0x011d_f780;
/// Setting object whose float is the distance beyond which hit effects are
/// tested for visibility.
const HIT_EFFECT_RANGE_OBJECT: u32 = 0x011c_e26c;
/// `cdecl` (position x, y, z, listener object, `float`): visibility test.
const HIT_EFFECT_VISIBLE: u32 = 0x004b_61d0;
/// `float`: the last argument of the visibility test.
const HIT_VISIBILITY_ARGUMENT: u32 = 0x0101_e340;
/// `cdecl` (min, max), `float` in ST0: a random number.
const RANDOM_FLOAT: u32 = 0x0047_6b70;
/// `cdecl`, one `float`, `float` in ST0: turned into a byte by `_ftol2`.
const RANDOM_BYTE_SOURCE: u32 = 0x0040_6cc0;
/// `float`: the largest random number for the hit byte (and the duration of
/// the damage messages, the same constant).
const HIT_RANDOM_MAX: u32 = 0x0101_e570;
/// The smallest and largest components of the random direction offset.
const HIT_OFFSET_MIN: u32 = 0x0107_5a9c;
const HIT_OFFSET_MAX: u32 = 0x0103_0ff0;
/// `float`: scale applied to the hit direction.
const HIT_DIRECTION_SCALE: u32 = 0x0101_62c0;
/// `cdecl` (out, `float` factor, vector): `out = factor * vector`.
const VECTOR_SCALE_INTO: u32 = 0x004a_3760;
/// `ECX` = vector: normalizes it (zero below `1e-6`).
const VECTOR_NORMALIZE: u32 = 0x004a_0c10;
/// No arguments: the sound set source used when a hit has none.
const DEFAULT_SOUND_SET_SOURCE: u32 = 0x005c_02f0;
/// `ECX` = a sound set entry: its sound.
const SOUND_SET_ENTRY_SOUND: u32 = 0x0067_33e0;
/// `ECX` = decal definition: the smallest and largest scale.
const DECAL_SCALE_MIN: u32 = 0x004a_40a0;
const DECAL_SCALE_MAX: u32 = 0x004a_40c0;
/// `cdecl` (`float` scale, 1): makes a decal.
const DECAL_CREATE: u32 = 0x0062_2a70;
/// `ECX` = decal, one stack word: sets its flag.
const DECAL_SET_FLAG: u32 = 0x0062_33c0;
/// `DecalCaster::Add` (Xbox PDB): `ECX` = decal, one stack word (the cell's
/// object).
const DECAL_CASTER_ADD: u32 = 0x0062_31a0;
/// `ECX` = collector (0x118 bytes): constructor.
const COLLECTOR_CONSTRUCT: u32 = 0x0062_d350;
/// `ECX` = collector: destructor.
const COLLECTOR_DESTROY: u32 = 0x0062_d4b0;
/// `ECX` = decal, stack (position, collector): collects the hit objects.
const DECAL_COLLECT: u32 = 0x0062_3090;
/// `ECX` = collector: its list (`this + 8`).
const COLLECTOR_LIST: u32 = 0x0041_3f40;
/// `ECX` = list: its count (the word at +4).
const LIST_ENTRY_COUNT: u32 = 0x0072_6070;
/// `ECX` = list, one stack word: the entry (the word at +8 is the
/// collidable).
const LIST_ENTRY: u32 = 0x0062_e150;
/// `bhkUtilFunctions::GetNiAVObject` (Xbox PDB), `cdecl`: the 3D object of a
/// collidable.
const COLLIDABLE_3D_OBJECT: u32 = 0x00c7_fa90;
/// `ECX` = actor, one stack word (a 3D object): the number of its body part.
const ACTOR_BODY_PART_NUMBER: u32 = 0x008b_3ef0;
/// `ECX` = hit record (0x78 bytes): constructor.
const HIT_RECORD_CONSTRUCT: u32 = 0x004a_37b0;
/// `ECX` = cell object, stack (hit record, kind, flag): adds the hit to the
/// cell (`RET 0xc`).
const CELL_ADD_HIT: u32 = 0x004a_3fe0;
/// `ECX` = hit-owner object, one stack word (actor): its collision record
/// (`RET 4`).
const OBJECT_COLLISION_RECORD: u32 = 0x009c_3ee0;
/// `GetAVObjectForCollidable` (map name), `cdecl`.
const AV_OBJECT_FOR_COLLIDABLE: u32 = 0x004b_5820;
/// `cdecl` (cell, 1.0, sound name, direction x, y, z, position x, y, z,
/// 1.0, 7, 3D object): plays the blood sound and spurt.
const PLAY_HIT_SPURT: u32 = 0x0068_90b0;
/// `ECX` = vector, one stack word: stores the negated vector there and
/// returns it.
const VECTOR_NEGATE_INTO: u32 = 0x004a_0bd0;
/// `BGSBodyPartData::GetBodyPart` (Xbox PDB): `ECX` = body part data, one
/// stack word.
const BODY_PART_OF: u32 = 0x005e_50f0;
/// `ECX` = body part: its sound set source.
const BODY_PART_SOUND_SOURCE: u32 = 0x004f_d400;
/// Setting object whose float is the distance limit for the second effect.
const HIT_SECOND_RANGE_OBJECT: u32 = 0x011c_ef8c;
/// No arguments, `float` in ST0: the distance the second effect compares
/// with its limit.
const HIT_SECOND_DISTANCE: u32 = 0x005c_5420;
/// `float`: 32.0 for the second decal's scale (the code's `0101e340`).
const HIT_SECOND_DECAL_SCALE: u32 = 0x0101_e340;
/// `float` added along the hit normal for the second ray.
const HIT_RAY_REACH: u32 = 0x0102_31a0;
/// `ECX` = vector, stack (out, `float` factor): `out = factor * vector`.
const VECTOR_TIMES_FLOAT_INTO: u32 = 0x0045_bb20;
/// The filter layer of the second ray.
const HIT_RAY_FILTER_LAYER: u32 = 0x27;
/// `ECX` = ray pick data, one stack word: sets the ray hit collector.
const PICK_SET_COLLECTOR: u32 = 0x004a_3fb0;
/// `ECX` = ray hit collector (0x330 bytes): constructor.
const RAY_COLLECTOR_CONSTRUCT: u32 = 0x004a_3a70;
/// `ECX` = ray hit collector: destructor.
const RAY_COLLECTOR_DESTROY: u32 = 0x004a_3bc0;
/// Virtual `+0xc8` of the cell's object: casts the ray of the pick data.
const CELL_OBJECT_VSLOT_CAST_RAY: u32 = 0xc8;
/// `ECX` = ray pick data: its result list holder (`this + 0xa8`).
const PICK_RESULT_HOLDER: u32 = 0x008c_dd90;
/// `ECX` = the result list, one stack word (an index): the record
/// (`index * 0x60 + [this]`).
const PICK_RESULT_RECORD: u32 = 0x004a_46b0;
/// `cdecl` (out vector, record): writes the record's normal into the vector.
const PICK_RECORD_NORMAL: u32 = 0x004a_3970;
/// `ECX` = the result's 3D object: its collision filter word.
const NODE_FILTER_WORD: u32 = 0x0043_b540;
/// `ECX` = `TES`, one stack word (a position): the object found there.
const TES_OBJECT_AT_POINT: u32 = 0x0045_7620;
/// `ECX` = object: its parent (the word at +0x18).
const OBJECT_PARENT: u32 = 0x0096_11e0;
/// `cdecl` (reference, 1): tests the reference's form type.
const REFERENCE_FORM_TEST: u32 = 0x004a_1060;
/// `ECX` = `TES`, stack (position x, y, z): the cell object at a point.
const TES_CELL_AT_POINT: u32 = 0x0045_19d0;
/// `ECX` = decal definition: its color word (the first of its fields), a
/// word that is read as `0x00RRGGBB`.
const DECAL_COLOR_WORD: u32 = 0x004a_4220;
/// Decal definition getters used when the second hit record is filled.
const DECAL_FIELD_BYTE_A: u32 = 0x004a_4100;
const DECAL_FIELD_FLOAT_A: u32 = 0x004a_41c0;
const DECAL_FIELD_BYTE_B: u32 = 0x004a_41e0;
const DECAL_FIELD_BYTE_C: u32 = 0x004a_4180;
const DECAL_FIELD_BYTE_D: u32 = 0x004a_4140;
const DECAL_FIELD_FLOAT_B: u32 = 0x004a_40e0;
const DECAL_FIELD_FLOAT_C: u32 = 0x009a_9350;
const DECAL_FIELD_FLOAT_D: u32 = 0x0059_8040;
/// `double` (255.0 per its use): divisor of the color bytes.
const COLOR_BYTE_DIVISOR: u32 = 0x0101_e568;
/// `float` stored in the second hit record.
const HIT_RECORD_FLOAT: u32 = 0x0101_e574;
/// `double`: factor applied to the scale of hits on armor.
const ARMOR_HIT_SCALE_FACTOR: u32 = 0x0101_6ff0;
/// Virtual `+0x224` of the hit's second object.
const HIT_OBJECT_VSLOT_0X224: u32 = 0x224;
/// Virtual `+0x14` of the sound set sound's member at +0x18: a pointer to a
/// string.
const SOUND_MEMBER_VSLOT_NAME: u32 = 0x14;

/// Virtual `+0x08` of the actor value owner (`target + 0xa4`): a number per
/// actor value index (the hit code tests index `0x37` for a positive value).
const OWNER_VSLOT_0X08: u32 = 0x08;
/// `ECX` = object: the address `this + 0x10`.
const OBJECT_PLUS_0X10: u32 = 0x0046_0140;

/// Reads the three words of a vector.
fn vector_words(e: &Engine, address: u32) -> [u32; 3] {
    [
        e.mem.u32(address),
        e.mem.u32(address + 4),
        e.mem.u32(address + 8),
    ]
}

/// Writes three words.
fn set_vector_words(e: &mut Engine, address: u32, words: [u32; 3]) {
    for (i, word) in words.iter().enumerate() {
        e.mem.set_u32(address + 4 * i as u32, *word);
    }
}

// Translated from 0088e8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Hit effects of an attack on the actor. `hit` is the hit record (target
/// reference at +0, second object at +8, kind at +0x10, point scale at
/// +0x14, explicit source +0x30, position at +0x38, direction at +0x44).
/// When the actor allows it (`0087f260`) and `damage` is positive, it
/// counts the player's points, decides from the combat load and the distance
/// whether to show anything, then makes the impact decal on the cell, plays
/// the blood sound and spurt of the sound set and, unless the hit is on
/// armor of types 1 and 2, casts a second ray and puts a second decal on
/// what it finds. The map has no name for it. The tail the game keeps after
/// the second loop (a debug marker) is unreachable in its code and is not
/// translated; the SEH frame is not translated either.
pub fn fn_0088e8d0(e: &mut Engine, this: Ptr<Actor>, damage: f32, hit: Ptr) {
    // The game's frame is 0x84c bytes; every local that is passed by address
    // lives in this block at its distance below the frame pointer.
    e.with_stack(0x890, |e, frame| {
        let frame_pointer = frame.addr() + 0x858;
        let timer = frame_pointer - 0x1c;
        e.call(
            TIMER_SCOPE_CONSTRUCT,
            &args![timer, 0x32u32, 1u32, ACTOR_SOURCE_FILE_NAME, 0x1f69u32],
        );
        run_hit_effects(e, frame_pointer, this, damage, hit.addr());
        e.call(TIMER_SCOPE_DESTROY, &args![timer]);
    });
}

fn run_hit_effects(e: &mut Engine, frame_pointer: u32, this: Ptr<Actor>, damage: f32, hit: u32) {
    let under = |distance: u32| frame_pointer - distance;
    let player = e.mem.u32(PLAYER_CHARACTER);
    let hit_kind = e.mem.u32(hit + 0x10);
    let melee_kind = hit != 0 && (hit_kind == 1 || hit_kind == 2);
    let material_flag = if melee_kind {
        e.call(HIT_FLAG_MELEE, &args![this]).u8()
    } else {
        e.call(IMPACT_SECONDARY_FLAG, &args![this]).u8()
    } != 0;
    let position = under(0x30);
    set_vector_words(e, position, vector_words(e, hit + 0x38));
    let target = e.mem.u32(hit);
    if !e.call(HIT_EFFECTS_ALLOWED, &args![this]).bool() {
        return;
    }
    let zero: f64 = e.global(DOUBLE_ZERO);
    if damage as f64 <= zero {
        return;
    }
    if this.addr() == player {
        let hit_points = e.mem.f32(hit + 0x14);
        let points = e.call(DAMAGE_POINTS, &args![hit_points]).i32();
        if points > 0 {
            e.call(PLAYER_STATISTIC_STEP, &args![points, 0.0f32, 0u32]);
        }
    }
    let armor_hit = target != 0
        && e.vcall(target + 0xa4, OWNER_VSLOT_0X08, &args![0x37u32])
            .i32()
            > 0;
    let listener = e.call(LISTENER_OBJECT, &args![]).u32();
    let listener_position = e.call(OBJECT_POSITION_0X8C, &args![listener]).u32();
    let difference = under(0x4c);
    e.call(
        VECTOR_MINUS_VECTOR,
        &args![position, difference, listener_position],
    );
    if !hit_is_visible(e, this, target, position, difference) {
        return;
    }
    let ray_kind = 0xffff_ffffu32;
    let angle = e.call(RANDOM_ANGLE, &args![]).f32();
    let random_max: f32 = e.global(HIT_RANDOM_MAX);
    let random = e.call(RANDOM_FLOAT, &args![0.0f32, random_max]).f32();
    let source_value = e.call(RANDOM_BYTE_SOURCE, &args![random]).f64();
    let random_byte = e.call(FTOL2, &args![source_value]).u8();
    let material = if material_flag {
        4
    } else {
        let base = e.call(BASE_FORM_OF, &args![this]).u32();
        e.vcall(base + 0x30, BASE_FORM_PART_VSLOT_MATERIAL, &args![])
            .u32()
    };
    let weapon = if target != 0 {
        e.call(ACTOR_GET_CURRENT_WEAPON, &args![target]).u32()
    } else {
        0
    };
    let source = if weapon != 0 {
        e.call(IMPACT_SOUND_SOURCE, &args![weapon]).u32()
    } else {
        0
    };
    let mut entry = if source != 0 {
        e.call(SOUND_SET_ENTRY, &args![source, material]).u32()
    } else {
        0
    };
    let mut hit_sound = if entry != 0 {
        e.call(SOUND_SET_ENTRY_SOUND, &args![entry]).u32()
    } else {
        0
    };
    let root = e.vcall(this.addr(), VSLOT_GET_3D, &args![]).u32();
    let direction_a = under(0xa4);
    let direction_b = under(0xb0);
    let point = under(0xbc);
    e.call(MATRIX_CONSTRUCT, &args![direction_a]);
    e.call(MATRIX_CONSTRUCT, &args![direction_b]);
    e.call(MATRIX_CONSTRUCT, &args![point]);
    let offset_min: f32 = e.global(HIT_OFFSET_MIN);
    let offset_max: f32 = e.global(HIT_OFFSET_MAX);
    for component in 0..3 {
        let value = e.call(RANDOM_FLOAT, &args![offset_min, offset_max]).f32();
        e.mem.set_f32(direction_b + 4 * component, value);
    }
    let direction_scale: f32 = e.global(HIT_DIRECTION_SCALE);
    let scaled = e
        .call(
            VECTOR_SCALE_INTO,
            &args![under(0xc8), direction_scale, hit + 0x44],
        )
        .u32();
    let sum = e
        .call(VECTOR_PLUS_VECTOR, &args![direction_b, under(0xd4), scaled])
        .u32();
    set_vector_words(e, direction_a, vector_words(e, sum));
    e.call(VECTOR_NORMALIZE, &args![direction_a]);
    set_vector_words(e, point, vector_words(e, position));
    if entry == 0 && hit_sound == 0 {
        let default_source = e.call(DEFAULT_SOUND_SET_SOURCE, &args![]).u32();
        entry = e
            .call(SOUND_SET_ENTRY, &args![default_source, material])
            .u32();
        hit_sound = if entry != 0 {
            e.call(SOUND_SET_ENTRY_SOUND, &args![entry]).u32()
        } else {
            0
        };
    }
    let mut decal_made = false;
    if root != 0
        && hit_sound != 0
        && !((hit_kind == 1 || hit_kind == 2)
            && e.vcall(this.addr(), VSLOT_TEST_0X218, &args![]).bool())
    {
        decal_made = true;
        let scale_max = e.call(DECAL_SCALE_MAX, &args![entry]).f32();
        let scale_min = e.call(DECAL_SCALE_MIN, &args![entry]).f32();
        let scale = e.call(RANDOM_FLOAT, &args![scale_min, scale_max]).f32();
        let decal = e.call(DECAL_CREATE, &args![scale, 1u32]).u32();
        e.call(DECAL_SET_FLAG, &args![decal, 1u32]);
        let cell = e.call(GET_PARENT_CELL, &args![this]).u32();
        let cell_object = if cell != 0 {
            e.call(CELL_OBJECT_OF, &args![cell]).u32()
        } else {
            0
        };
        if cell_object != 0 {
            e.call(DECAL_CASTER_ADD, &args![decal, cell_object]);
            let collector = under(0x208);
            e.call(COLLECTOR_CONSTRUCT, &args![collector]);
            e.call(DECAL_COLLECT, &args![decal, point, collector]);
            let list = e.call(COLLECTOR_LIST, &args![collector]).u32();
            let count = e.call(LIST_ENTRY_COUNT, &args![list]).i32();
            if count != 0 {
                let record = under(0x290);
                e.call(HIT_RECORD_CONSTRUCT, &args![record]);
                set_vector_words(e, record, vector_words(e, point));
                set_vector_words(e, record + 0xc, vector_words(e, direction_a));
                e.mem.set_f32(record + 0x38, scale);
                e.mem.set_f32(record + 0x3c, scale);
                e.mem.set_u32(record + 0x28, root);
                e.mem.set_u32(record + 0x34, ray_kind);
                e.mem.set_f32(record + 0x44, angle);
                e.mem.set_u8(record + 0x70, random_byte);
                e.mem.set_u32(record + 0x30, hit_sound);
                for index in 0..count {
                    let list = e.call(COLLECTOR_LIST, &args![collector]).u32();
                    let list_entry = e.call(LIST_ENTRY, &args![list, index as u32]).u32();
                    let collidable = e.mem.u32(list_entry + 8);
                    if collidable != 0 {
                        let object = e.call(COLLIDABLE_3D_OBJECT, &args![collidable]).u32();
                        let reference = e.call(FIND_REFERENCE_FOR_3D, &args![object]).u32();
                        if reference != 0 && reference == this.addr() {
                            let part = e.call(ACTOR_BODY_PART_NUMBER, &args![this, object]).u32();
                            let flags = e.mem.u32(record + 0x6c);
                            e.mem
                                .set_u32(record + 0x6c, (1u32 << ((part + 1) & 31)) | flags);
                        }
                    }
                }
                let flags = e.mem.u32(record + 0x6c);
                e.mem.set_u32(record + 0x6c, flags & 0xffff_fffe);
                let cell = e.call(GET_PARENT_CELL, &args![this]).u32();
                let creature =
                    target != 0 && e.vcall(target, VSLOT_ACTOR_FLAG_0X360, &args![]).bool();
                e.call(CELL_ADD_HIT, &args![cell, record, 2u32, creature as u32]);
            }
            e.call(COLLECTOR_DESTROY, &args![collector]);
        }
    }
    // The blood sound and spurt.
    let mut spurt = false;
    if entry != 0 {
        let member = entry + 0x18;
        if e.vcall(member, SOUND_MEMBER_VSLOT_NAME, &args![]).u32() != 0 {
            let name_pointer = e.vcall(member, SOUND_MEMBER_VSLOT_NAME, &args![]).u32();
            if e.mem.i8(name_pointer) != 0 {
                spurt = true;
                let mut collidable_3d = 0u32;
                let second = e.mem.u32(hit + 8);
                if second != 0 && e.vcall(second, HIT_OBJECT_VSLOT_0X224, &args![]).bool() {
                    let record = e.call(OBJECT_COLLISION_RECORD, &args![second, this]).u32();
                    if record != 0 && e.mem.u32(record + 0x1c) != 0 {
                        let inner = e.mem.u32(record + 0x1c);
                        let word = e.call(OBJECT_PLUS_0X10, &args![inner]).u32();
                        collidable_3d = e.call(AV_OBJECT_FOR_COLLIDABLE, &args![word]).u32();
                    }
                }
                let negated = e
                    .call(VECTOR_NEGATE_INTO, &args![direction_a, under(0x2c0)])
                    .u32();
                let negated_words = vector_words(e, negated);
                let point_words = vector_words(e, point);
                let name = e.vcall(member, SOUND_MEMBER_VSLOT_NAME, &args![]).u32();
                let cell = e.call(GET_PARENT_CELL, &args![this]).u32();
                e.call(
                    PLAY_HIT_SPURT,
                    &args![
                        cell,
                        1.0f32,
                        name,
                        negated_words[0],
                        negated_words[1],
                        negated_words[2],
                        point_words[0],
                        point_words[1],
                        point_words[2],
                        1.0f32,
                        7u32,
                        collidable_3d
                    ],
                );
            }
        }
    }
    if !spurt && !decal_made {
        return;
    }
    second_hit_decal(
        e,
        frame_pointer,
        this,
        hit,
        material,
        material_flag,
        armor_hit,
        angle,
        random_byte,
    );
}

/// Whether the hit is shown: always when `525420` says so; otherwise when the
/// combat is below its limit or either party is the player's, or the hit is
/// within the range setting, or the visibility test finds it.
fn hit_is_visible(
    e: &mut Engine,
    this: Ptr<Actor>,
    target: u32,
    position: u32,
    difference: u32,
) -> bool {
    if e.call(WEIGHT_OVERRIDE_FLAG, &args![]).bool() {
        return true;
    }
    let manager = e.mem.u32(COMBAT_MANAGER_POINTER);
    let combatants = e.call(COMBATANT_COUNT, &args![manager, 0u32, 0u32]).u32();
    let limit_address = e
        .call(GLOBAL_WORD_ADDRESS, &args![COMBATANT_LIMIT_OBJECT])
        .u32();
    let limit = e.mem.u32(limit_address);
    if limit < combatants
        && !e
            .vcall(this.addr(), VSLOT_ACTOR_FLAG_0X360, &args![])
            .bool()
        && target != 0
        && !e.vcall(target, VSLOT_ACTOR_FLAG_0X360, &args![]).bool()
    {
        return true;
    }
    let distance = e.call(VECTOR_LENGTH, &args![difference]).f64();
    let range_address = e
        .call(SETTING_VALUE_ADDRESS, &args![HIT_EFFECT_RANGE_OBJECT])
        .u32();
    let range = e.mem.f32(range_address);
    if range as f64 >= distance {
        return true;
    }
    let listener = e.call(LISTENER_OBJECT, &args![]).u32();
    let argument: f32 = e.global(HIT_VISIBILITY_ARGUMENT);
    let words = vector_words(e, position);
    !e.call(
        HIT_EFFECT_VISIBLE,
        &args![words[0], words[1], words[2], listener, argument],
    )
    .bool()
}

/// The second half of `0088e8d0`: finds the sound set entry of the hit body
/// part, casts a short ray along the hit direction from the hit point and
/// puts a decal on the first object of the actor the ray meets.
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn second_hit_decal(
    e: &mut Engine,
    frame_pointer: u32,
    this: Ptr<Actor>,
    hit: u32,
    material: u32,
    material_flag: bool,
    armor_hit: bool,
    angle: f32,
    random_byte: u8,
) {
    let under = |distance: u32| frame_pointer - distance;
    let point = under(0xbc);
    let direction_a = under(0xa4);
    let hit_kind = e.mem.u32(hit + 0x10);
    let base = e.call(BASE_FORM_OF, &args![this]).u32();
    let body_data = e.vcall(base, BASE_FORM_VSLOT_0X180, &args![]).u32();
    let part = if body_data != 0 {
        e.call(BODY_PART_OF, &args![body_data, hit_kind]).u32()
    } else {
        0
    };
    let source = if part != 0 {
        e.call(BODY_PART_SOUND_SOURCE, &args![part]).u32()
    } else {
        0
    };
    let entry = if source != 0 {
        e.call(SOUND_SET_ENTRY, &args![source, material]).u32()
    } else {
        0
    };
    let hit_sound = if entry != 0 {
        e.call(SOUND_SET_ENTRY_SOUND, &args![entry]).u32()
    } else {
        0
    };
    if material_flag || entry == 0 {
        return;
    }
    let distance = e.call(HIT_SECOND_DISTANCE, &args![]).f64();
    let limit_address = e
        .call(SETTING_VALUE_ADDRESS, &args![HIT_SECOND_RANGE_OBJECT])
        .u32();
    let limit = e.mem.f32(limit_address);
    if limit as f64 <= distance {
        return;
    }
    let decal_scale: f32 = e.global(HIT_SECOND_DECAL_SCALE);
    let second_decal = e.call(DECAL_CREATE, &args![decal_scale, 1u32]).u32();
    e.call(DECAL_SET_FLAG, &args![second_decal, 0u32]);
    let cell = e.call(GET_PARENT_CELL, &args![this]).u32();
    let cell_object = if cell != 0 {
        e.call(CELL_OBJECT_OF, &args![cell]).u32()
    } else {
        0
    };
    if cell_object == 0 {
        return;
    }
    let ray_direction = under(0x300);
    set_vector_words(e, ray_direction, vector_words(e, direction_a));
    let half: f64 = e.global(DOUBLE_HALF);
    let z = e.mem.f32(ray_direction + 8);
    e.mem.set_f32(ray_direction + 8, (z as f64 - half) as f32);
    e.call(VECTOR_NORMALIZE, &args![ray_direction]);
    let reach: f32 = e.global(HIT_RAY_REACH);
    let ray_vector = e
        .call(
            VECTOR_TIMES_FLOAT_INTO,
            &args![ray_direction, under(0x30c), reach],
        )
        .u32();
    let ray_end = under(0x318);
    e.call(VECTOR_PLUS_VECTOR, &args![point, ray_end, ray_vector]);
    let pick = under(0x3d0);
    let filter = under(0x3d4);
    e.call(PICK_CONSTRUCT, &args![pick]);
    e.call(STORE_WORD, &args![filter, 0u32]);
    e.call(FILTER_SET_LAYER, &args![filter, HIT_RAY_FILTER_LAYER]);
    let filter_word = e.mem.u32(filter);
    e.call(PICK_SET_FILTER, &args![pick, filter_word]);
    e.call(PICK_SET_START, &args![pick, point]);
    e.call(PICK_SET_END, &args![pick, ray_end]);
    let collector = under(0x700);
    e.call(RAY_COLLECTOR_CONSTRUCT, &args![collector]);
    e.call(PICK_SET_COLLECTOR, &args![pick, collector]);
    e.vcall(cell_object, CELL_OBJECT_VSLOT_CAST_RAY, &args![pick]);
    let holder = e.call(PICK_RESULT_HOLDER, &args![pick]).u32();
    let results = e.call(OBJECT_PLUS_0X10, &args![holder]).u32();
    let count = e.call(LIST_ENTRY_COUNT, &args![results]).i32();
    let tes = e.mem.u32(TES_POINTER);
    let mut done = false;
    let mut index = 0i32;
    while !done && index < count {
        let record = pick_result(e, pick, index);
        let node = e.mem.u32(record + 0x50);
        if node != 0 {
            let to_hit = under(0x720);
            e.call(VECTOR_MINUS_VECTOR, &args![ray_end, to_hit, point]);
            let record = pick_result(e, pick, index);
            let fraction = e.mem.f32(record + 0x10);
            let scaled = e
                .call(VECTOR_SCALE_INTO, &args![under(0x72c), fraction, to_hit])
                .u32();
            let hit_point = under(0x738);
            e.call(VECTOR_PLUS_VECTOR, &args![point, hit_point, scaled]);
            let normal = under(0x744);
            e.call(MATRIX_CONSTRUCT, &args![normal]);
            let record = pick_result(e, pick, index);
            e.call(PICK_RECORD_NORMAL, &args![normal, record]);
            let node_word = e.call(NODE_FILTER_WORD, &args![node]).u32();
            let node_filter = under(0x748);
            let stored = e.call(STORE_WORD, &args![node_filter, node_word]).u32();
            let group = e.call(FILTER_GET_GROUP, &args![stored]).u32();
            let actor_node = group == 1;
            let mut found;
            if actor_node {
                found = e.call(TES_OBJECT_AT_POINT, &args![tes, hit_point]).u32();
                if found != 0 && e.call(OBJECT_PARENT, &args![found]).u32() != 0 {
                    found = e.call(OBJECT_PARENT, &args![found]).u32();
                }
            } else {
                found = e.call(COLLIDABLE_3D_OBJECT, &args![node]).u32();
            }
            let reference = e.call(FIND_REFERENCE_FOR_3D, &args![found]).u32();
            if !actor_node
                && !(reference != 0 && e.call(REFERENCE_FORM_TEST, &args![reference, 1u32]).bool())
            {
                index += 1;
                continue;
            }
            let words = vector_words(e, hit_point);
            let decal_cell = e
                .call(TES_CELL_AT_POINT, &args![tes, words[0], words[1], words[2]])
                .u32();
            let scale_max = e.call(DECAL_SCALE_MAX, &args![entry]).f32();
            let scale_min = e.call(DECAL_SCALE_MIN, &args![entry]).f32();
            let scale = e.call(RANDOM_FLOAT, &args![scale_min, scale_max]).f32();
            let record = under(0x7e0);
            e.call(HIT_RECORD_CONSTRUCT, &args![record]);
            set_vector_words(e, record, vector_words(e, hit_point));
            set_vector_words(e, record + 0xc, vector_words(e, normal));
            e.mem.set_f32(record + 0x38, scale);
            e.mem.set_f32(record + 0x3c, scale);
            let extra_float: f32 = e.global(HIT_RECORD_FLOAT);
            e.mem.set_f32(record + 0x40, extra_float);
            e.mem.set_f32(record + 0x44, angle);
            e.mem.set_u8(record + 0x70, random_byte);
            e.mem.set_u32(record + 0x28, found);
            e.mem.set_u32(record + 0x30, hit_sound);
            let byte = e.call(DECAL_FIELD_BYTE_A, &args![entry]).u8();
            e.mem.set_u8(record + 0x73, byte);
            let value = e.call(DECAL_FIELD_FLOAT_A, &args![entry]).f32();
            e.mem.set_f32(record + 0x4c, value);
            let byte = e.call(DECAL_FIELD_BYTE_B, &args![entry]).u8();
            e.mem.set_u8(record + 0x76, byte);
            let byte = e.call(DECAL_FIELD_BYTE_C, &args![entry]).u8();
            e.mem.set_u8(record + 0x74, byte);
            let byte = e.call(DECAL_FIELD_BYTE_D, &args![entry]).u8();
            e.mem.set_u8(record + 0x75, byte);
            let value = e.call(DECAL_FIELD_FLOAT_B, &args![entry]).f32();
            e.mem.set_f32(record + 0x54, value);
            let value = e.call(DECAL_FIELD_FLOAT_C, &args![entry]).f32();
            e.mem.set_f32(record + 0x58, value);
            let value = e.call(DECAL_FIELD_FLOAT_D, &args![entry]).f32();
            e.mem.set_f32(record + 0x5c, value);
            let divisor: f64 = e.global(COLOR_BYTE_DIVISOR);
            let color = e.call(DECAL_COLOR_WORD, &args![entry]).u32();
            let red = (((color >> 16) & 0xff) as f64 / divisor) as f32;
            let color = e.call(DECAL_COLOR_WORD, &args![entry]).u32();
            let green = ((((color & 0xffff) >> 8) & 0xff) as f64 / divisor) as f32;
            let color = e.call(DECAL_COLOR_WORD, &args![entry]).u32();
            let blue = ((color & 0xff) as f64 / divisor) as f32;
            let colour_vector = e
                .call(NI_POINT3_SET, &args![under(0x804), blue, green, red])
                .u32();
            set_vector_words(e, record + 0x60, vector_words(e, colour_vector));
            if armor_hit {
                let factor: f64 = e.global(ARMOR_HIT_SCALE_FACTOR);
                for offset in [0x38u32, 0x3c] {
                    let value = e.mem.f32(record + offset);
                    e.mem
                        .set_f32(record + offset, (value as f64 * factor) as f32);
                }
            }
            e.call(CELL_ADD_HIT, &args![decal_cell, record, 1u32, 0u32]);
            done = true;
        }
        index += 1;
    }
    e.call(RAY_COLLECTOR_DESTROY, &args![collector]);
    let _ = second_decal;
}

/// The record `index` of the pick data's result list.
fn pick_result(e: &mut Engine, pick: u32, index: i32) -> u32 {
    let holder = e.call(PICK_RESULT_HOLDER, &args![pick]).u32();
    let list = e.call(OBJECT_PLUS_0X10, &args![holder]).u32();
    e.call(PICK_RESULT_RECORD, &args![list, index as u32]).u32()
}

// ---- weapon hit effects (0088fb00) ----

/// The run-time type descriptors `fn_0088fb00` casts the base form between.
const CREATURE_CAST_SOURCE_TYPE: u32 = 0x0118_46e8;
const CREATURE_CAST_TARGET_TYPE: u32 = 0x0118_3a00;
/// `TESCreature::IsHumanoidCreature` (Xbox PDB): `ECX` = the creature.
const CREATURE_IS_HUMANOID: u32 = 0x005f_bf20;
/// Setting object whose byte stops `fn_0088fb00` for creatures.
const CREATURE_HIT_SETTING_OBJECT: u32 = 0x011d_f7f8;
/// Process virtual `+0x1b0`: when true no weapon hit effect is made.
const PROCESS_VSLOT_0X1B0: u32 = 0x1b0;
/// Process virtual `+0x190` (shared with the drop code): (biped data) the
/// weapon node of the actor.
const PROCESS_VSLOT_WEAPON_NODE: u32 = 0x190;
/// `ECX` = held item: a flag test that stops the effect.
const HELD_ITEM_FLAG_TEST: u32 = 0x004c_0c30;
/// Setting object whose float is compared with the damage.
const WEAPON_HIT_DAMAGE_SETTING: u32 = 0x011d_f748;
/// `ECX` = the biped data, one stack word: the node `this + 8 + 8 * index`.
const BIPED_NODE_BY_INDEX: u32 = 0x004a_b230;
/// `Actor::GetWeaponPosition` (Xbox PDB): `ECX` = actor, stack (out vector,
/// flag): the weapon position; returns the vector.
const GET_WEAPON_POSITION: u32 = 0x008a_5340;
/// `ECX` = vector, stack (out, `float` divisor): `out = vector / divisor`.
const VECTOR_DIVIDE_BY_FLOAT: u32 = 0x0053_d280;
/// `ECX` = node: a `float` in ST0 (the word at +0x98).
const NODE_FLOAT_0X98: u32 = 0x008d_01e0;
/// `ECX` = player: the animation owner the weapon hit code asks.
const PLAYER_ANIMATION_OWNER: u32 = 0x0095_0a10;
/// `ECX` = node, one stack word: the owner of the extra data `43b4a0` looks
/// up.
const NODE_EXTRA_OWNER_COUNT: u32 = 0x0045_bc00;
/// `ECX` = node, one stack word: a node of the extra data.
const NODE_EXTRA_OWNER: u32 = 0x0043_b4a0;
/// `ECX` = fixed string storage, one stack word (a C string): constructs it.
const FIXED_STRING_FROM: u32 = 0x0043_8170;
/// `ECX` = fixed string: releases it.
const FIXED_STRING_RELEASE: u32 = 0x0043_81b0;
/// `NiObjectNET::GetExtraData` (Xbox PDB): `ECX` = the node, one stack word
/// (the name).
const NODE_GET_EXTRA_DATA: u32 = 0x00a5_bdd0;
/// No arguments: the default extra data name (a global).
const DEFAULT_EXTRA_DATA_NAME: u32 = 0x004a_b220;
/// `cdecl` (minimum, maximum): a random integer.
const RANDOM_INTEGER: u32 = 0x0094_4460;
/// The string `ModelSwapNode`'s sibling name the code looks the extra data
/// up by (`DVPG`-prefixed at `01020504`).
const EXTRA_DATA_NAME_STRING: u32 = 0x0102_0504;
/// Process virtual `+0x1b8` through `Actor::GetAnimation`: (named above as
/// `ACTOR_GET_ANIMATION`).
/// Number of the animation lookup argument the weapon hit code uses.
const WEAPON_HIT_ANIMATION_KIND: u32 = 4;

// Translated from 0088fb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Hit effects of a weapon strike: when the damage exceeds the larger of 1.0
/// and the setting `011df748`, and the attacker `attacker` has a held weapon
/// with a node, it puts a decal (the sound set entry of the hit body part of
/// the attacker) at the weapon's position, twice: on the weapon node and on
/// the node `[-0x5c]` stands for (the player's biped node, in first
/// person). `hit` is the hit record (target at +0, kind at +0x10, direction
/// at +0x44). The map has no name for it; the SEH frame is not translated.
pub fn fn_0088fb00(e: &mut Engine, this: Ptr<Actor>, damage: f32, hit: Ptr, attacker: Ptr) {
    // The game's frame is 0x218 bytes plus the saved registers.
    e.with_stack(0x240, |e, frame| {
        let frame_pointer = frame.addr() + 0x228;
        weapon_hit_effects(e, frame_pointer, this, damage, hit.addr(), attacker.addr());
    });
}

#[allow(clippy::too_many_lines)]
fn weapon_hit_effects(
    e: &mut Engine,
    frame_pointer: u32,
    this: Ptr<Actor>,
    damage: f32,
    hit: u32,
    attacker: u32,
) {
    let under = |distance: u32| frame_pointer - distance;
    let player = e.mem.u32(PLAYER_CHARACTER);
    let base = e.call(BASE_FORM_OF, &args![this]).u32();
    let cast = e
        .call(
            RT_DYNAMIC_CAST_FN,
            &args![
                base,
                0u32,
                CREATURE_CAST_SOURCE_TYPE,
                CREATURE_CAST_TARGET_TYPE,
                0u32
            ],
        )
        .u32();
    let skip_setting = (cast != 0 && e.call(CREATURE_IS_HUMANOID, &args![cast]).bool())
        || e.vcall(this.addr(), VSLOT_TEST_0X218, &args![]).bool();
    if skip_setting {
        let setting = e
            .call(SETTING_VALUE_POINTER, &args![CREATURE_HIT_SETTING_OBJECT])
            .u32();
        if e.mem.u8(setting) != 0 {
            return;
        }
    }
    let target = e.mem.u32(hit);
    let armor_hit = target != 0
        && e.vcall(target + 0xa4, OWNER_VSLOT_0X08, &args![0x37u32])
            .i32()
            > 0;
    let ray_kind = 0xffff_ffffu32;
    let process = e.call(GET_PROCESS, &args![this]).u32();
    let hit_kind = e.mem.u32(hit + 0x10);
    let melee_kind = hit != 0 && (hit_kind == 1 || hit_kind == 2);
    let material_flag = if melee_kind {
        e.call(HIT_FLAG_MELEE, &args![attacker]).u8()
    } else {
        e.call(IMPACT_SECONDARY_FLAG, &args![attacker]).u8()
    } != 0;
    let material = if material_flag {
        4
    } else {
        let base = e.call(BASE_FORM_OF, &args![attacker]).u32();
        e.vcall(base + 0x30, BASE_FORM_PART_VSLOT_MATERIAL, &args![])
            .u32()
    };
    let body_data = if attacker != 0 {
        let base = e.call(BASE_FORM_OF, &args![attacker]).u32();
        e.vcall(base, BASE_FORM_VSLOT_0X180, &args![]).u32()
    } else {
        0
    };
    let part = if body_data != 0 {
        e.call(BODY_PART_OF, &args![body_data, hit_kind]).u32()
    } else {
        0
    };
    let source = if part != 0 {
        e.call(BODY_PART_SOUND_SOURCE, &args![part]).u32()
    } else {
        0
    };
    let entry = if source != 0 {
        e.call(SOUND_SET_ENTRY, &args![source, material]).u32()
    } else {
        0
    };
    let hit_sound = if entry != 0 {
        e.call(SOUND_SET_ENTRY_SOUND, &args![entry]).u32()
    } else {
        0
    };
    if hit == 0 || attacker == 0 || hit_sound == 0 || material_flag {
        return;
    }
    if e.vcall(process, PROCESS_VSLOT_0X1B0, &args![]).bool() {
        return;
    }
    let mut weapon_node = if process != 0 {
        let biped = e.vcall(this.addr(), VSLOT_GET_BIPED, &args![]).u32();
        e.vcall(process, PROCESS_VSLOT_WEAPON_NODE, &args![biped])
            .u32()
    } else {
        0
    };
    let own_process = e.get(this, Actor::pCurrentProcess);
    let held = e
        .vcall(own_process.addr(), PROCESS_VSLOT_ITEM, &args![])
        .u32();
    let held_form = if held != 0 {
        e.call(ITEM_VALUE, &args![held]).u32()
    } else {
        0
    };
    if weapon_node == 0 || held_form == 0 {
        return;
    }
    if e.call(HELD_ITEM_FLAG_TEST, &args![held_form]).bool() {
        return;
    }
    let setting_address = e
        .call(SETTING_VALUE_ADDRESS, &args![WEAPON_HIT_DAMAGE_SETTING])
        .u32();
    let setting = e.mem.f32(setting_address);
    let limit = e.call(FLOAT_HELPER_MAX, &args![1.0f32, setting]).f32();
    let strong_enough = damage as f64 > limit as f64;
    if !strong_enough {
        return;
    }
    let start = under(0x58);
    let player_direction = under(0x78);
    let direction = under(0x4c);
    let mut second_node = 0u32;
    e.call(MATRIX_CONSTRUCT, &args![start]);
    e.call(MATRIX_CONSTRUCT, &args![player_direction]);
    e.call(MATRIX_CONSTRUCT, &args![under(0x6c)]);
    if this.addr() == player {
        let biped = e.call(PLAYER_GET_BIPED, &args![player, 1u32]).u32();
        let player_node = e.call(BIPED_NODE_BY_INDEX, &args![biped, 1u32]).u32();
        let weapon_position = e
            .call(GET_WEAPON_POSITION, &args![this, under(0x150), 1u32])
            .u32();
        let node_position = e.call(OBJECT_POSITION_0X8C, &args![weapon_node]).u32();
        let offset = e
            .call(
                VECTOR_MINUS_VECTOR,
                &args![node_position, under(0x15c), weapon_position],
            )
            .u32();
        let duration: f32 = e.global(HIT_DIRECTION_SCALE);
        let divided = e
            .call(
                VECTOR_DIVIDE_BY_FLOAT,
                &args![offset, under(0x168), duration],
            )
            .u32();
        set_vector_words(e, start, vector_words(e, divided));
        second_node = if e.call(PLAYER_STATE_TEST, &args![player]).bool() {
            player_node
        } else {
            weapon_node
        };
        weapon_node = if e.call(PLAYER_STATE_TEST, &args![player]).bool() {
            weapon_node
        } else {
            player_node
        };
        let bound = e.call(GET_WORLD_BOUND, &args![weapon_node]).u32();
        let bound_buffer = under(0x12c);
        for word in 0..4 {
            let value = e.mem.u32(bound + 4 * word);
            e.mem.set_u32(bound_buffer + 4 * word, value);
        }
        let _ = e.call(UPDATE_STEP_SETTING, &args![bound_buffer]).f64();
        let _ = e.call(NODE_FLOAT_0X98, &args![weapon_node]).f64();
        let node_position = e.call(OBJECT_POSITION_0X8C, &args![weapon_node]).u32();
        let handle = e.call(MATRIX_CONSTRUCT, &args![bound_buffer]).u32();
        e.call(
            VECTOR_MINUS_VECTOR,
            &args![handle, under(0x114), node_position],
        );
        let mut animation_flag = 0u32;
        let animation_owner = e.call(PLAYER_ANIMATION_OWNER, &args![player]).u32();
        if animation_owner != 0 {
            let animation = e
                .call(
                    ANIMATION_LOOKUP,
                    &args![animation_owner, WEAPON_HIT_ANIMATION_KIND],
                )
                .u32();
            if animation != 0 {
                let step = e.call(ANIMATION_STEP, &args![animation]).u32();
                if step != 0 {
                    animation_flag = fn_008905f0(e, Ptr::new(step)) as u32;
                }
            }
        }
        let extra = if e
            .call(NODE_EXTRA_OWNER_COUNT, &args![weapon_node, 0u32])
            .u32()
            != 0
        {
            let name = e
                .call(
                    FIXED_STRING_FROM,
                    &args![under(0x16c), EXTRA_DATA_NAME_STRING],
                )
                .u32();
            let owner = e.call(NODE_EXTRA_OWNER, &args![weapon_node, 0u32]).u32();
            let found = e.call(NODE_GET_EXTRA_DATA, &args![owner, name]).u32();
            e.call(FIXED_STRING_RELEASE, &args![under(0x16c)]);
            found
        } else {
            0
        };
        let extra = if extra == 0 {
            let default_name = e.call(DEFAULT_EXTRA_DATA_NAME, &args![]).u32();
            e.call(NODE_GET_EXTRA_DATA, &args![weapon_node, default_name])
                .u32()
        } else {
            extra
        };
        if extra == 0 || (animation_flag as i32) >= e.mem.u16(extra + 0x10) as i32 {
            return;
        }
        let table_entry = e.mem.u32(extra + 0x14 + 4 * animation_flag);
        let count = e.mem.u16(table_entry) as u32;
        let index = e
            .call(RANDOM_INTEGER, &args![0u32, count.wrapping_sub(1)])
            .u32();
        set_vector_words(
            e,
            start,
            vector_words(e, table_entry + index.wrapping_mul(12) + 4),
        );
        set_vector_words(
            e,
            player_direction,
            vector_words(e, table_entry + index.wrapping_mul(12) + 0x64),
        );
    } else {
        let animation = e.call(ACTOR_GET_ANIMATION, &args![this]).u32();
        if animation != 0 {
            let lookup = e
                .call(
                    ANIMATION_LOOKUP,
                    &args![animation, WEAPON_HIT_ANIMATION_KIND],
                )
                .u32();
            if lookup != 0 {
                let lookup = e
                    .call(
                        ANIMATION_LOOKUP,
                        &args![animation, WEAPON_HIT_ANIMATION_KIND],
                    )
                    .u32();
                let step = e.call(ANIMATION_STEP, &args![lookup]).u32();
                let _ = fn_008905f0(e, Ptr::new(step));
                let duration: f32 = e.global(HIT_DIRECTION_SCALE);
                let node_position = e.call(OBJECT_POSITION_0X8C, &args![weapon_node]).u32();
                let weapon_position = e
                    .call(GET_WEAPON_POSITION, &args![this, under(0x178), 0u32])
                    .u32();
                let offset = e
                    .call(
                        VECTOR_MINUS_VECTOR,
                        &args![weapon_position, under(0x184), node_position],
                    )
                    .u32();
                let divided = e
                    .call(
                        VECTOR_DIVIDE_BY_FLOAT,
                        &args![offset, under(0x190), duration],
                    )
                    .u32();
                let node_position = e.call(OBJECT_POSITION_0X8C, &args![weapon_node]).u32();
                let sum = e
                    .call(
                        VECTOR_PLUS_VECTOR,
                        &args![node_position, under(0x19c), divided],
                    )
                    .u32();
                set_vector_words(e, start, vector_words(e, sum));
            }
        }
    }
    // The common part: a random byte, the angle and the decal scale.
    let random_max: f32 = e.global(HIT_RANDOM_MAX);
    let random = e.call(RANDOM_FLOAT, &args![0.0f32, random_max]).f32();
    let source_value = e.call(RANDOM_BYTE_SOURCE, &args![random]).f64();
    let random_byte = e.call(FTOL2, &args![source_value]).u8();
    let angle = e.call(RANDOM_ANGLE, &args![]).f32();
    let scale_max = e.call(DECAL_SCALE_MAX, &args![entry]).f32();
    let scale_min = e.call(DECAL_SCALE_MIN, &args![entry]).f32();
    let scale = e.call(RANDOM_FLOAT, &args![scale_min, scale_max]).f32();
    e.call(MATRIX_CONSTRUCT, &args![direction]);
    if this.addr() == player {
        set_vector_words(e, direction, vector_words(e, player_direction));
    } else {
        let negated = e
            .call(VECTOR_NEGATE_INTO, &args![hit + 0x44, under(0x1a8)])
            .u32();
        set_vector_words(e, direction, vector_words(e, negated));
    }
    let record = under(0xfc);
    e.call(HIT_RECORD_CONSTRUCT, &args![record]);
    set_vector_words(e, record, vector_words(e, start));
    set_vector_words(e, record + 0xc, vector_words(e, direction));
    e.mem.set_u32(record + 0x28, weapon_node);
    e.mem.set_u32(record + 0x34, ray_kind);
    e.mem.set_u32(record + 0x2c, second_node);
    e.mem.set_f32(record + 0x44, angle);
    e.mem.set_u8(record + 0x70, random_byte);
    e.mem.set_u32(record + 0x30, hit_sound);
    e.mem.set_f32(record + 0x38, scale);
    e.mem.set_f32(record + 0x3c, scale);
    let extra_float: f32 = e.global(HIT_RECORD_FLOAT);
    e.mem.set_f32(record + 0x40, extra_float);
    let value = e.call(DECAL_FIELD_FLOAT_B, &args![entry]).f32();
    e.mem.set_f32(record + 0x54, value);
    e.mem.set_u8(record + 0x72, 0);
    let value = e.call(DECAL_FIELD_FLOAT_D, &args![entry]).f32();
    e.mem.set_f32(record + 0x5c, value);
    let value = e.call(DECAL_FIELD_FLOAT_C, &args![entry]).f32();
    e.mem.set_f32(record + 0x58, value);
    e.mem.set_u8(record + 0x77, 1);
    let byte = e.call(DECAL_FIELD_BYTE_C, &args![entry]).u8();
    e.mem.set_u8(record + 0x74, byte);
    let byte = e.call(DECAL_FIELD_BYTE_D, &args![entry]).u8();
    e.mem.set_u8(record + 0x75, byte);
    let divisor: f64 = e.global(COLOR_BYTE_DIVISOR);
    let color = e.call(DECAL_COLOR_WORD, &args![entry]).u32();
    let red = (((color >> 16) & 0xff) as f64 / divisor) as f32;
    let color = e.call(DECAL_COLOR_WORD, &args![entry]).u32();
    let green = ((((color & 0xffff) >> 8) & 0xff) as f64 / divisor) as f32;
    let color = e.call(DECAL_COLOR_WORD, &args![entry]).u32();
    let blue = ((color & 0xff) as f64 / divisor) as f32;
    let colour_vector = e
        .call(NI_POINT3_SET, &args![under(0x1b4), blue, green, red])
        .u32();
    set_vector_words(e, record + 0x60, vector_words(e, colour_vector));
    e.mem.set_u8(record + 0x78, 1);
    if armor_hit {
        let factor: f64 = e.global(ARMOR_HIT_SCALE_FACTOR);
        for offset in [0x38u32, 0x3c] {
            let value = e.mem.f32(record + offset);
            e.mem
                .set_f32(record + offset, (value as f64 * factor) as f32);
        }
    }
    let cell = e.call(GET_PARENT_CELL, &args![this]).u32();
    e.call(CELL_ADD_HIT, &args![cell, record, 1u32, 0u32]);
    e.mem.set_u32(record + 0x28, second_node);
    let cell = e.call(GET_PARENT_CELL, &args![this]).u32();
    e.call(CELL_ADD_HIT, &args![cell, record, 1u32, 0u32]);
}

// ---- traps (00890610) ----

/// `Script::FindVariable` (Xbox PDB): `ECX` = script, stack (name, `u32*` index
/// out); true (`AL`, signed) when found.
const SCRIPT_FIND_VARIABLE: u32 = 0x005a_c6a0;
/// `ScriptLocals::GetVariable` (Xbox PDB): `ECX` = locals, stack (index, 0);
/// a `float` in ST0.
const LOCALS_GET_VARIABLE: u32 = 0x005a_9140;
/// `ScriptLocals::SetVariable` (Xbox PDB): `ECX` = locals, stack (index,
/// `double` value).
const LOCALS_SET_VARIABLE: u32 = 0x005a_9290;
/// `ExtraDataList::GetScript` (Xbox PDB): `ECX` = the list.
const EXTRA_LIST_GET_SCRIPT: u32 = 0x0041_8800;
/// `ExtraDataList::GetScriptLocals` (Xbox PDB): `ECX` = the list.
const EXTRA_LIST_GET_SCRIPT_LOCALS: u32 = 0x0041_8830;
/// `ECX` = player: the trap owner a player's own trap is recognised by.
const PLAYER_TRAP_OWNER: u32 = 0x0089_f4e0;
/// `ECX` = actor: the level (`u16`) the levelled trap damage multiplies.
const ACTOR_LEVEL: u32 = 0x0087_f9f0;
/// `cdecl`, one `float`, `float` in ST0: the value times a global factor
/// (`011c582c`).
const SCALE_BY_GLOBAL_FACTOR: u32 = 0x004a_3e90;
/// `ECX` = trap owner entry, one stack word: `entry + 8 + ...` (the
/// referenced form): `cdecl`, one word, a collision body: `004b59f0`.
const COLLISION_BODY_OF_FORM: u32 = 0x004b_59f0;
/// `ECX` = vector, one stack word (out): the vector's length-like value
/// (`630b40`, a lane square root); returns `out`.
const VECTOR_ROOT_INTO: u32 = 0x0063_0b40;
/// `ECX` = vector: its length (`float` in ST0).
const VECTOR_LENGTH_FN: u32 = 0x0045_86d0;
/// `ECX` = vector (four lanes), one stack word (another vector): subtracts.
const VECTOR_SUBTRACT_ASSIGN: u32 = 0x0096_1190;
/// `cdecl` (out, source): converts a vector (`004a3e00`, each component
/// through `004a3e90`).
const VECTOR_CONVERT: u32 = 0x004a_3e00;
/// `cdecl` (out, source): a vector derived from another (`00458620`).
const VECTOR_DERIVE: u32 = 0x0045_8620;
/// `ECX` = character controller: its linear velocity (`bhkCharacterProxy::
/// GethkLinearVelocity` in the map).
const CONTROLLER_LINEAR_VELOCITY: u32 = 0x0080_7080;
/// `ECX` = collision body: a position-like object (`560dc0`) and a
/// second (`4b4ec0`) the code feeds `004a3f10`.
const BODY_FIRST_VECTOR: u32 = 0x0056_0dc0;
const BODY_SECOND_VECTOR: u32 = 0x004b_4ec0;
/// `cdecl`, one word: a number derived from the collision body (`004b5a80`).
const BODY_NUMBER: u32 = 0x004b_5a80;
/// `ECX` = hit record (`HitData::HitData` in the map): constructor.
const HIT_DATA_CONSTRUCT: u32 = 0x009b_4d90;
/// `ECX` = collision record: constructor (`006240d0`).
const COLLISION_RECORD_CONSTRUCT: u32 = 0x0062_40d0;
/// `ImpactMixer::PlayCollisionSound` (Xbox PDB), `cdecl`, one record.
const PLAY_COLLISION_SOUND: u32 = 0x0083_7550;
/// `bhkWorld::SetMotion` (Xbox PDB), `cdecl` (object, 1, 1, 0, 1).
const WORLD_SET_MOTION: u32 = 0x00c6_a350;
/// `bhkForceController::RemoveForce` (Xbox PDB), `cdecl`, one object.
const REMOVE_FORCE: u32 = 0x00cb_9650;
/// `ECX` = vector: multiplies by a vector made from its argument chain
/// (`0062aea0`).
const VECTOR_SCALE_BY_CONSTANT: u32 = 0x0062_aea0;
/// `TESHavokUtilities::AddExplosionImpulse` (Xbox PDB), `cdecl` (object,
/// vector, `float`, 0).
const ADD_EXPLOSION_IMPULSE: u32 = 0x0062_b660;
/// `bhkCharacterController::SetVelocityModifier` (Xbox PDB): `ECX` =
/// controller, stack (vector, `float`).
const SET_VELOCITY_MODIFIER_FN: u32 = 0x00c6_d4d0;
/// `ECX` = vector, one stack word (`float`): multiplies its components.
const VECTOR_TIMES_SCALAR_ASSIGN: u32 = 0x0043_9180;
/// `ECX` = the Y of the trap record: second vector lookup (`00891170`) is
/// `fn_00891170`.
/// The `Actor` virtual `+0x338`: (damage, `float` 0.0, 0).
const VSLOT_APPLY_DAMAGE: u32 = 0x338;
/// Owner virtual `+0x0c`: a `float` per actor value.
const OWNER_VSLOT_VALUE_0X0C: u32 = 0x0c;
/// Settings for the trap code.
const TRAP_LEVEL_FACTOR_DIVISOR: u32 = 0x0101_7a40;
const TRAP_IMPULSE_LIMIT: u32 = 0x0101_79e0;
const TRAP_PUSH_LIMIT: u32 = 0x0101_7b70;
const TRAP_PUSH_LIMIT_VALUE: u32 = 0x0101_3974;
const TRAP_IMPULSE_LIMIT_VALUE: u32 = 0x0102_226c;
const TRAP_PUSH_TO_IMPULSE_DIVISOR: u32 = 0x0102_0758;
const TRAP_SPEED_SCALE_LIMIT: u32 = 0x0101_1590;
/// `double` (2.0): the divisor of the actor height for the trap hit height.
const TRAP_HALF_HEIGHT_DIVISOR: u32 = 0x0101_1590;
/// `ECX` = the word stored by `008c71b0`: its kind (compared with 0x10).
const KIND_OF_WORD: u32 = 0x0043_b4d0;
const TRAP_SPEED_SCALE_VALUE: u32 = 0x0101_62c0;
const TRAP_FIXED_FLOAT: u32 = 0x0101_6248;
const TRAP_DEFAULT_RANGE: u32 = 0x0101_7b78;
const TRAP_FLOAT_SCALE: u32 = 0x0102_36e0;
const TRAP_VECTOR_SCALAR: u32 = 0x0101_7718;
const TRAP_DIRECTION_VECTOR: u32 = 0x011a_9484;
/// The script variable names the trap code reads.
const TRAP_DAMAGE_NAME: u32 = 0x0108_49b4;
const TRAP_LEVELLED_DAMAGE_NAME: u32 = 0x0108_49a4;
const TRAP_PUSH_BACK_NAME: u32 = 0x0108_4994;
const TRAP_MIN_VELOCITY_NAME: u32 = 0x0108_4980;
const TRAP_CONTINUOUS_NAME: u32 = 0x0108_4970;
const TRAP_DEATH_PUSH_BACK_NAME: u32 = 0x0108_495c;
const TRAP_HAS_HIT_NAME: u32 = 0x0108_4950;
/// Kind number (`0x10`) of the form whose trap hurts through the health
/// actor value.
const TRAP_KIND_HEALTH: u32 = 0x10;
/// Kind number (`0x24`) `00890610` hands to the collision record.
const COLLISION_RECORD_KIND: u32 = 0x24;
/// Actor value index whose owner value the death trap damage uses.
const TRAP_ACTOR_VALUE: u32 = 0x13;

/// Looks `name` up in the script; the index when found.
fn find_script_variable(e: &mut Engine, script: u32, name: u32) -> Option<u32> {
    e.with_stack(4, |e, out| {
        e.mem.set_u32(out.addr(), 0);
        let found = e.call(SCRIPT_FIND_VARIABLE, &args![script, name, out]).eax as u8 as i8 != 0;
        found.then(|| e.mem.u32(out.addr()))
    })
}

/// Reads a script variable as a `float`.
fn script_float(e: &mut Engine, locals: u32, index: u32) -> f32 {
    e.call(LOCALS_GET_VARIABLE, &args![locals, index, 0u32])
        .f32()
}

// Translated from 00890610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::DoTrap` (Xbox PDB): the trap `trap_data`, hit by `trap` (the
/// trap record whose member at +4 refers to the trap reference), hurts the
/// actor. The damage and the pushes come from the trap reference's script
/// variables: `fTrapDamage`, `fLevelledDamage` (times the actor's level),
/// `fTrapPushBack`, `fTrapMinVelocity`, `bTrapContinuous` and
/// `fTrapDeathPushBack`; `bTrapHasHit` is set to 1 afterwards. Damage is
/// scaled by the actor's speed against the minimum velocity, reduced by the
/// damage resistance of the actor value 0x13 for kind 0x10 hits, applied
/// through the actor virtual `+0x338`; a hit effect record is built for
/// `fn_0088e8d0`, pain triggered, and the actor is pushed (or exploded when
/// it has just died). The SEH-free frame is plain; the map name is by the
/// Xbox PDB.
#[allow(clippy::too_many_lines)]
pub fn actor_do_trap(e: &mut Engine, this: Ptr<Actor>, trap: Ptr, trap_data: Ptr) {
    // The game's frame is 0x240 bytes below the frame pointer.
    e.with_stack(0x270, |e, frame| {
        let frame_pointer = frame.addr() + 0x250;
        do_trap(e, frame_pointer, this, trap.addr(), trap_data);
    });
}

#[allow(clippy::too_many_lines)]
fn do_trap(e: &mut Engine, frame_pointer: u32, this: Ptr<Actor>, trap: u32, trap_data: Ptr) {
    let under = |distance: u32| frame_pointer - distance;
    let player = e.mem.u32(PLAYER_CHARACTER);
    let zero: f64 = e.global(DOUBLE_ZERO);
    let mut damage = 0.0f32;
    let mut push_force = 0.0f32;
    let mut impulse = 0.0f32;
    let mut min_velocity = 0.0f32;
    let mut continuous = false;
    let mut speed_scale = 1.0f32;
    let trap_reference = e.call(SOUND_PLAY_ARGUMENT, &args![trap + 4]).u32();
    let list = e
        .call(GET_EXTRA_DATA_LIST_ADDRESS, &args![trap_reference])
        .u32();
    let script = e.call(EXTRA_LIST_GET_SCRIPT, &args![list]).u32();
    if this.addr() == player
        && trap_reference != 0
        && trap_reference == e.call(PLAYER_TRAP_OWNER, &args![player]).u32()
    {
        return;
    }
    if script != 0 {
        let locals = e.call(EXTRA_LIST_GET_SCRIPT_LOCALS, &args![list]).u32();
        if locals != 0 {
            if let Some(index) = find_script_variable(e, script, TRAP_DAMAGE_NAME) {
                damage = script_float(e, locals, index);
            }
            if let Some(index) = find_script_variable(e, script, TRAP_LEVELLED_DAMAGE_NAME) {
                let level = e.call(ACTOR_LEVEL, &args![this]).u16() as f64;
                let value = script_float(e, locals, index);
                damage = (value as f64 * level + damage as f64) as f32;
            }
            if let Some(index) = find_script_variable(e, script, TRAP_PUSH_BACK_NAME) {
                push_force = script_float(e, locals, index);
            }
            if let Some(index) = find_script_variable(e, script, TRAP_MIN_VELOCITY_NAME) {
                min_velocity = script_float(e, locals, index);
                if (min_velocity as f64) < zero {
                    min_velocity = 0.0;
                } else {
                    min_velocity = e.call(SCALE_BY_GLOBAL_FACTOR, &args![min_velocity]).f32();
                }
            }
            if let Some(index) = find_script_variable(e, script, TRAP_CONTINUOUS_NAME) {
                let value = script_float(e, locals, index);
                continuous = (value as f64) != zero;
            }
            if let Some(index) = find_script_variable(e, script, TRAP_DEATH_PUSH_BACK_NAME) {
                impulse = script_float(e, locals, index);
            } else {
                let divisor: f64 = e.global(TRAP_PUSH_TO_IMPULSE_DIVISOR);
                impulse = (push_force as f64 / divisor) as f32;
            }
        }
    }
    if !continuous && fn_00891150(e, trap_data) {
        return;
    }
    let controller = e.call(GET_CHAR_CONTROLLER, &args![this]).u32();
    let trap_form = e.call(ITEM_VALUE, &args![trap + 4]).u32();
    let body = e.call(COLLISION_BODY_OF_FORM, &args![trap_form]).u32();
    let velocity = under(0x80);
    let position = under(0x90);
    let direction = under(0xa0);
    e.call(MATRIX_CONSTRUCT, &args![velocity]);
    e.call(MATRIX_CONSTRUCT, &args![position]);
    if body != 0 {
        if !fn_00891150(e, trap_data) {
            let source = e.call(POSITION_VECTOR_OF, &args![trap_data]).u32();
            e.call(SUBOBJECT_SET_VALUE, &args![velocity, source]);
            let member = fn_00891170(e, trap_data);
            e.call(SUBOBJECT_SET_VALUE, &args![position, member]);
        } else {
            let first = e.call(BODY_FIRST_VECTOR, &args![body]).u32();
            e.call(SUBOBJECT_SET_VALUE, &args![velocity, first]);
            let second = e.call(BODY_SECOND_VECTOR, &args![body]).u32();
            e.call(SUBOBJECT_SET_VALUE, &args![position, second]);
        }
        e.call(MATRIX_CONSTRUCT, &args![direction]);
        let mut skip_damage = false;
        if (min_velocity as f64) != zero {
            let rooted = e
                .call(VECTOR_ROOT_INTO, &args![velocity, under(0xb0)])
                .u32();
            let length = e.call(VECTOR_LENGTH_FN, &args![rooted]).f32();
            if min_velocity as f64 > length as f64 {
                damage = 0.0;
                skip_damage = true;
            }
        }
        if !skip_damage && controller != 0 {
            let linear = e.call(CONTROLLER_LINEAR_VELOCITY, &args![controller]).u32();
            e.call(SUBOBJECT_SET_VALUE, &args![direction, linear]);
            e.call(VECTOR_SUBTRACT_ASSIGN, &args![direction, velocity]);
            let range = if (min_velocity as f64) != zero {
                min_velocity
            } else {
                e.global(TRAP_DEFAULT_RANGE)
            };
            let rooted = e
                .call(VECTOR_ROOT_INTO, &args![direction, under(0xd0)])
                .u32();
            let length = e.call(VECTOR_LENGTH_FN, &args![rooted]).f32();
            speed_scale = (length as f64 / range as f64) as f32;
            let limit: f64 = e.global(TRAP_SPEED_SCALE_LIMIT);
            if speed_scale as f64 > limit {
                speed_scale = e.global(TRAP_SPEED_SCALE_VALUE);
            }
        }
    } else {
        if !fn_00891150(e, trap_data) {
            let member = fn_00891170(e, trap_data);
            e.call(SUBOBJECT_SET_VALUE, &args![position, member]);
        } else {
            let inner = e.call(ITEM_VALUE, &args![trap + 4]).u32();
            let form = e.call(ITEM_VALUE, &args![inner]).u32();
            let offset = e.call(POSITION_VECTOR_OF, &args![form]).u32();
            e.call(SUBOBJECT_SET_VALUE, &args![position, offset]);
        }
        let own_position = e.vcall(this.addr(), VSLOT_POSITION_0X1F4, &args![]).u32();
        e.call(VECTOR_CONVERT, &args![velocity, own_position]);
        e.call(VECTOR_SUBTRACT_ASSIGN, &args![velocity, position]);
        let component = e
            .call(VECTOR_COMPONENT_ADDRESS, &args![velocity, 2u32])
            .u32();
        let height = actor_get_height(e, this);
        let half_height = (height as f64 / e.global::<f64>(TRAP_HALF_HEIGHT_DIVISOR)) as f32;
        let scaled = e.call(SCALE_BY_GLOBAL_FACTOR, &args![half_height]).f32();
        let z = e.mem.f32(component);
        e.mem.set_f32(component, (z as f64 + scaled as f64) as f32);
    }
    let was_not_dead = !e
        .vcall(this.addr(), VSLOT_IS_PROCESS_FLAG_0X22C, &args![0u32])
        .bool();
    let trap_form = e.call(ITEM_VALUE, &args![trap + 4]).u32();
    let form_word = e.call(NODE_FILTER_WORD, &args![trap_form]).u32();
    let kind_word = under(0xe4);
    e.call(STORE_WORD, &args![kind_word, form_word]);
    let trap_kind = e.call(KIND_OF_WORD, &args![kind_word]).u32();
    if trap_kind == TRAP_KIND_HEALTH {
        let owner = this.addr() + ACTOR_VALUE_OWNER_OFFSET;
        let resisted = e
            .vcall(owner, OWNER_VSLOT_VALUE_0X0C, &args![TRAP_ACTOR_VALUE])
            .f32();
        let divisor: f64 = e.global(TRAP_LEVEL_FACTOR_DIVISOR);
        let fraction = (resisted as f64 / divisor) as f32;
        let capped = e.call(FLOAT_HELPER_MIN, &args![fraction, 1.0f32]).f32();
        let factor = (1.0 - capped as f64) as f32;
        damage = (damage as f64 * factor as f64) as f32;
    } else {
        let capped = e.call(FLOAT_HELPER_MIN, &args![0.0f32, 1.0f32]).f32();
        let factor = (1.0 - capped as f64) as f32;
        damage = (damage as f64 * factor as f64) as f32;
    }
    if damage.is_nan() || (damage as f64) <= zero {
        finish_trap(e, trap_data);
        return;
    }
    if continuous {
        let setting = e
            .call(UPDATE_STEP_SETTING, &args![UPDATE_STEP_OBJECT])
            .f32();
        damage = (setting as f64 * speed_scale as f64 * damage as f64) as f32;
    }
    e.vcall(
        this.addr(),
        VSLOT_APPLY_DAMAGE,
        &args![damage, 0.0f32, 0u32],
    );
    let height = actor_get_height(e, this);
    let half_height = (height as f64 * e.global::<f64>(DOUBLE_HALF)) as f32;
    let eye = e
        .call(
            VECTOR_TIMES_FLOAT_INTO,
            &args![TRAP_DIRECTION_VECTOR, under(0xfc), half_height],
        )
        .u32();
    let root = e.vcall(this.addr(), VSLOT_GET_3D, &args![]).u32();
    let root_position = e.call(OBJECT_POSITION_0X8C, &args![root]).u32();
    let hit_position = under(0x108);
    e.call(VECTOR_PLUS_VECTOR, &args![root_position, hit_position, eye]);
    let derived = under(0x114);
    e.call(MATRIX_CONSTRUCT, &args![derived]);
    e.call(VECTOR_DERIVE, &args![derived, position]);
    if e.call(KIND_OF_WORD, &args![kind_word]).u32() != TRAP_KIND_HEALTH {
        let hit_data = under(0x180);
        e.call(HIT_DATA_CONSTRUCT, &args![hit_data]);
        e.mem.set_u32(hit_data + 8, 0);
        set_vector_words(e, hit_data + 0x38, vector_words(e, hit_position));
        let difference = e
            .call(
                VECTOR_MINUS_VECTOR,
                &args![hit_position, under(0x18c), derived],
            )
            .u32();
        set_vector_words(e, hit_data + 0x44, vector_words(e, difference));
        e.mem.set_u32(hit_data, 0);
        e.mem.set_u32(hit_data + 0x10, 0xffff_ffff);
        let double_damage = (damage as f64 + damage as f64) as f32;
        fn_0088e8d0(e, this, double_damage, Ptr::new(hit_data));
        if !continuous
            && body != 0
            && e.call(GET_PARENT_CELL, &args![trap_data]).u32() != COLLISION_RECORD_KIND
        {
            let derived_again = e.call(VECTOR_DERIVE, &args![under(0x19c), position]).u32();
            let reach = vector_words(e, derived_again);
            let collision_number = e.call(BODY_NUMBER, &args![body]).u32();
            let record = under(0x1c4);
            e.call(COLLISION_RECORD_CONSTRUCT, &args![record]);
            let fixed: f32 = e.global(TRAP_FIXED_FLOAT);
            e.mem.set_f32(record + 0x10, fixed);
            e.mem.set_u32(record + 0x1c, 0);
            e.mem.set_u32(record + 0x20, collision_number);
            e.mem.set_u8(record + 0x14, COLLISION_RECORD_KIND as u8);
            let kind = e.call(GET_PARENT_CELL, &args![trap_data]).u8();
            e.mem.set_u8(record + 0x15, kind);
            set_vector_words(e, record, reach);
            let scalar: f32 = e.global(TRAP_VECTOR_SCALAR);
            let scalar_vector = e
                .call(VECTOR_FROM_FLOAT, &args![under(0x1e0), scalar])
                .u32();
            let rooted = e
                .call(VECTOR_ROOT_INTO, &args![velocity, under(0x1f0)])
                .u32();
            let rooted = fn_00891060(
                e,
                Ptr::new(under(0x200)),
                Ptr::new(rooted),
                Ptr::new(scalar_vector),
            );
            let speed = e.call(VECTOR_LENGTH_FN, &args![rooted.addr()]).f32();
            e.mem.set_f32(record + 0xc, speed);
            e.mem.set_u32(record + 0x18, this.addr() + collision_number);
            e.call(PLAY_COLLISION_SOUND, &args![record]);
        }
    }
    e.call(ACTOR_TRIGGER_PAIN, &args![this, 1u32, 1u32]);
    let dead_now = was_not_dead
        && e.vcall(this.addr(), VSLOT_IS_PROCESS_FLAG_0X22C, &args![0u32])
            .bool();
    if push_force as f64 > zero {
        let limit: f64 = e.global(TRAP_IMPULSE_LIMIT);
        if impulse as f64 > limit {
            impulse = e.global(TRAP_IMPULSE_LIMIT_VALUE);
        }
        if dead_now && this.addr() != player {
            let root = e.vcall(this.addr(), VSLOT_GET_3D, &args![]).u32();
            e.call(WORLD_SET_MOTION, &args![root, 1u32, 1u32, 0u32, 1u32]);
            let root = e.vcall(this.addr(), VSLOT_GET_3D, &args![]).u32();
            e.call(REMOVE_FORCE, &args![root]);
            e.call(VECTOR_SCALE_BY_CONSTANT, &args![velocity]);
            let factor: f32 = e.global(TRAP_FLOAT_SCALE);
            let spread = e
                .call(VECTOR_FROM_FLOAT, &args![under(0x220), factor])
                .u32();
            fn_008910c0(e, Ptr::new(position), Ptr::new(spread), Ptr::new(velocity));
            let root = e.vcall(this.addr(), VSLOT_GET_3D, &args![]).u32();
            e.call(ADD_EXPLOSION_IMPULSE, &args![root, position, impulse, 0u32]);
        } else if controller != 0 {
            let limit: f64 = e.global(TRAP_PUSH_LIMIT);
            if push_force as f64 > limit {
                push_force = e.global(TRAP_PUSH_LIMIT_VALUE);
            }
            if continuous {
                let setting = e
                    .call(UPDATE_STEP_SETTING, &args![UPDATE_STEP_OBJECT])
                    .f32();
                push_force = (setting as f64 * speed_scale as f64 * push_force as f64) as f32;
            }
            let form = e.call(ITEM_VALUE, &args![trap + 4]).u32();
            if form != 0 {
                let push_vector = under(0x230);
                e.call(MATRIX_CONSTRUCT, &args![push_vector]);
                e.call(VECTOR_DERIVE, &args![push_vector, velocity]);
                e.call(VECTOR_NORMALIZE, &args![push_vector]);
                e.call(VECTOR_TIMES_SCALAR_ASSIGN, &args![push_vector, push_force]);
                let modifier: f32 = e.global(TRAP_FIXED_FLOAT);
                e.call(
                    SET_VELOCITY_MODIFIER_FN,
                    &args![controller, push_vector, modifier],
                );
            }
        }
    }
    if script != 0 {
        let locals = e.call(EXTRA_LIST_GET_SCRIPT_LOCALS, &args![list]).u32();
        if locals != 0 {
            if let Some(index) = find_script_variable(e, script, TRAP_HAS_HIT_NAME) {
                e.call(LOCALS_SET_VARIABLE, &args![locals, index, 1.0f64]);
            }
        }
    }
    finish_trap(e, trap_data);
}

/// The common end of `DoTrap`: sets bit 1 of the trap record's flag word.
fn finish_trap(e: &mut Engine, trap_data: Ptr) {
    fn_00891130(e, trap_data, 1);
}

/// This part's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00884990, fn_00884990(Ptr<Actor>)),
        entry!(0x008849c0, fn_008849c0(Ptr<Actor>) -> bool),
        entry!(0x00884aa0, fn_00884aa0(Ptr<Actor>) -> u32),
        entry!(0x00884ae0, fn_00884ae0(Ptr<Actor>) -> bool),
        entry!(0x00884b50, actor_is_allowed_to_swim(Ptr<Actor>) -> bool),
        entry!(0x00884c60, fn_00884c60(Ptr<Actor>)),
        entry!(0x00884c80, fn_00884c80(Ptr<CachedValuesOwner>) -> f32),
        entry!(0x00884ce0, fn_00884ce0(Ptr<BaseProcess>, f32)),
        entry!(0x00884d20, fn_00884d20(Ptr<CachedValuesOwner>) -> f32),
        entry!(0x00884d80, fn_00884d80(Ptr<BaseProcess>, f32)),
        entry!(0x00884dc0, actor_get_walk_speed(Ptr<Actor>) -> f32),
        entry!(
            0x00884e30,
            fn_00884e30(Ptr<BaseProcess>, Ptr<CachedValuesOwner>) -> f32
        ),
        entry!(0x00884e90, fn_00884e90(Ptr<CachedValues>, u32) -> bool),
        entry!(0x00884eb0, actor_get_run_speed(Ptr<Actor>) -> f32),
        entry!(
            0x00884f20,
            fn_00884f20(Ptr<BaseProcess>, Ptr<CachedValuesOwner>) -> f32
        ),
        entry!(0x00884f80, fn_00884f80(Ptr<Actor>)),
        entry!(0x00884fc0, actor_set_walk_speed(Ptr<Actor>, f32)),
        entry!(0x00884ff0, actor_set_run_speed(Ptr<Actor>, f32)),
        entry!(0x00885020, fn_00885020(Ptr<CachedValuesOwner>) -> f32),
        entry!(0x00885110, fn_00885110(Ptr<BaseProcess>, f32)),
        entry!(0x00885140, fn_00885140(Ptr<CachedValuesOwner>) -> f32),
        entry!(0x008851e0, fn_008851e0(Ptr<BaseProcess>, f32)),
        entry!(0x00885210, fn_00885210(Ptr<CachedValuesOwner>) -> f32),
        entry!(0x008852b0, fn_008852b0(Ptr<BaseProcess>, f32)),
        entry!(0x008852e0, fn_008852e0(Ptr<CachedValuesOwner>) -> f32),
        entry!(0x00885360, fn_00885360(Ptr<BaseProcess>, f32)),
        entry!(0x008853a0, actor_get_height(Ptr<Actor>) -> f32),
        entry!(0x00885490, fn_00885490(Ptr<HighProcess>) -> f32),
        entry!(0x008854b0, fn_008854b0(Ptr<HighProcess>, f32)),
        entry!(0x008854d0, actor_get_sneak_height(Ptr<Actor>, bool) -> f32),
        entry!(0x00885520, fn_00885520(Ptr<Actor>, Ptr, Ptr, f32) -> bool),
        entry!(0x00885560, fn_00885560(Ptr<Actor>, f32, Ptr) -> f32),
        entry!(
            0x008855f0,
            actor_adjust_speed_for_following(Ptr<Actor>, Ptr, f32, f32)
        ),
        entry!(
            0x008858c0,
            actor_modify_move_mode_for_follow(Ptr<Actor>, u32, f32, f32) -> u32
        ),
        entry!(0x008859e0, fn_008859e0(Ptr, Ptr) -> bool),
        entry!(0x00885a30, fn_00885a30(Ptr) -> bool),
        entry!(0x00885a50, actor_calculate_walk_speed(Ptr<Actor>) -> f32),
        entry!(0x00885bd0, fn_00885bd0(Ptr) -> f32),
        entry!(0x00885bf0, actor_calculate_run_speed(Ptr<Actor>) -> f32),
        entry!(0x00885d70, fn_00885d70(Ptr) -> f32),
        entry!(0x00885d90, fn_00885d90(Ptr<Actor>) -> f32),
        entry!(0x00885ed0, fn_00885ed0(Ptr<Actor>) -> f32),
        entry!(0x00886010, fn_00886010(Ptr<Actor>) -> f32),
        entry!(
            0x00886080,
            actor_get_current_package_location(Ptr<Actor>) -> u32
        ),
        entry!(0x008860d0, fn_008860d0(Ptr<Actor>) -> bool),
        entry!(0x00886360, fn_00886360(Ptr<Actor>, f32)),
        entry!(0x00886b10, fn_00886b10(Ptr<Actor>) -> bool),
        entry!(0x00886b30, fn_00886b30(Ptr<Actor>, Ptr) -> bool),
        entry!(0x00886c80, fn_00886c80(Ptr, bool)),
        entry!(0x00886cb0, fn_00886cb0(Ptr<Actor>, f32, u32)),
        entry!(0x008879d0, fn_008879d0(Ptr, bool)),
        entry!(0x00887a00, fn_00887a00() -> u32),
        entry!(0x00887a10, fn_00887a10(Ptr) -> u32),
        entry!(0x00887a80, fn_00887a80(Ptr, bool)),
        entry!(0x00887ac0, fn_00887ac0(Ptr, f32)),
        entry!(0x00887af0, fn_00887af0(Ptr, u32)),
        entry!(0x00887b20, fn_00887b20(Ptr, u32)),
        entry!(0x00887b40, fn_00887b40(Ptr, u8)),
        entry!(0x00887b70, fn_00887b70(Ptr, u8)),
        entry!(0x00887b90, fn_00887b90(Ptr<Actor>, f32, u32)),
        entry!(0x00887d00, actor_reset_loaded_animations(Ptr<Actor>)),
        entry!(0x00888070, fn_00888070(Ptr<Actor>, f32)),
        entry!(0x008885a0, fn_008885a0(Ptr<Actor>)),
        entry!(
            0x008885e0,
            actor_update_animation_movement(Ptr<Actor>, Ptr, f32)
        ),
        entry!(0x00888960, fn_00888960() -> u8),
        entry!(0x00888970, fn_00888970(Ptr<Actor>, u32)),
        entry!(0x00888a20, fn_00888a20(Ptr, f32, f32)),
        entry!(0x00888a50, fn_00888a50(Ptr) -> bool),
        entry!(0x00888a70, fn_00888a70(Ptr, Ptr, Ptr)),
        entry!(0x00888aa0, fn_00888aa0(Ptr)),
        entry!(0x00888ac0, fn_00888ac0(Ptr<Actor>, f32)),
        entry!(0x00888af0, fn_00888af0(Ptr<Actor>) -> f64),
        entry!(0x00888b20, fn_00888b20(Ptr<Actor>)),
        entry!(0x00888b50, actor_update(Ptr<Actor>, f32)),
        entry!(0x0088b020, fn_0088b020(u8)),
        entry!(0x0088b050, fn_0088b050(Ptr, u8)),
        entry!(0x0088b070, fn_0088b070(Ptr, Ptr)),
        entry!(0x0088b110, fn_0088b110(Ptr, f32)),
        entry!(0x0088b130, fn_0088b130(Ptr) -> u32),
        entry!(0x0088b150, actor_update_actor_3d_position(Ptr<Actor>)),
        entry!(0x0088b4e0, fn_0088b4e0(Ptr<Actor>) -> u32),
        entry!(0x0088b510, actor_restore_health(Ptr<Actor>, f32)),
        entry!(0x0088b5a0, actor_restore_fatigue(Ptr<Actor>, f32)),
        entry!(0x0088b660, actor_restore_action_points(Ptr<Actor>, f32)),
        entry!(0x0088b740, actor_restore_actor_value(Ptr<Actor>, u32, f32)),
        entry!(0x0088b7f0, fn_0088b7f0(Ptr<Actor>) -> u32),
        entry!(0x0088b850, fn_0088b850(Ptr) -> f32),
        entry!(
            0x0088b880,
            actor_line_of_sight(Ptr<Actor>, bool, Ptr, bool, Ptr, bool) -> bool
        ),
        entry!(0x0088c240, fn_0088c240(Ptr<Actor>, Ptr, bool, bool) -> bool),
        entry!(
            0x0088c570,
            actor_is_point_in_view_cone(Ptr<Actor>, Ptr, f32) -> bool
        ),
        entry!(
            0x0088c600,
            actor_has_360_line_of_sight(Ptr<Actor>, u32) -> u8
        ),
        entry!(
            0x0088c650,
            actor_queue_equip_object(Ptr<Actor>, Ptr, i32, Ptr, u8, u8, u8)
        ),
        entry!(
            0x0088c790,
            actor_queue_un_equip_object(Ptr<Actor>, Ptr, i32, Ptr, u8, u8, u8)
        ),
        entry!(
            0x0088c830,
            actor_equip_object(Ptr<Actor>, Ptr, i32, Ptr, u8, u8, u8)
        ),
        entry!(0x0088d2d0, fn_0088d2d0(Ptr) -> u8),
        entry!(
            0x0088d2f0,
            actor_put_actor_in_chair_bed_quick(Ptr<Actor>, Ptr, Ptr, u32, u8) -> u8
        ),
        entry!(0x0088d640, actor_get_out_of_furniture_quick(Ptr<Actor>)),
        entry!(
            0x0088d7d0,
            actor_un_equip_object(Ptr<Actor>, Ptr, i32, Ptr, u8, u8, u8) -> u8
        ),
        entry!(0x0088db20, fn_0088db20(Ptr<Actor>, Ptr, i32, Ptr, u8) -> u8),
        entry!(0x0088e0f0, fn_0088e0f0(Ptr, u32) -> u32),
        entry!(
            0x0088e110,
            fn_0088e110(Ptr<Actor>, Ptr, i32, Ptr, u8, u8) -> u8
        ),
        entry!(0x008905f0, fn_008905f0(Ptr) -> u8),
        entry!(0x00891060, fn_00891060(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x008910c0, fn_008910c0(Ptr, Ptr, Ptr)),
        entry!(0x00891130, fn_00891130(Ptr, u8)),
        entry!(0x00891150, fn_00891150(Ptr) -> bool),
        entry!(0x00891170, fn_00891170(Ptr) -> u32),
        entry!(0x00891350, fn_00891350() -> u32),
        entry!(0x00891b70, fn_00891b70(Ptr) -> bool),
        entry!(
            0x00891b90,
            actor_get_armor_being_worn(Ptr<Actor>, u32) -> u32
        ),
        entry!(0x00891be0, fn_00891be0(Ptr<Actor>, Ptr) -> u32),
        entry!(0x00891c80, fn_00891c80(Ptr<Actor>, Ptr) -> u32),
        entry!(0x00891ce0, fn_00891ce0(Ptr<Actor>) -> u32),
        entry!(0x00891d30, actor_get_best_food_item(Ptr<Actor>) -> u32),
        entry!(
            0x00891190,
            actor_update_weapon_condition_effects(Ptr<Actor>, u8)
        ),
        entry!(
            0x00891360,
            actor_damage_equipment(Ptr<Actor>, Ptr, f32, u32) -> u8
        ),
        entry!(0x0088e1e0, fn_0088e1e0(Ptr<Actor>, Ptr)),
        entry!(0x0088e8d0, fn_0088e8d0(Ptr<Actor>, f32, Ptr)),
        entry!(0x0088fb00, fn_0088fb00(Ptr<Actor>, f32, Ptr, Ptr)),
        entry!(0x00890610, actor_do_trap(Ptr<Actor>, Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const ACTOR_TABLE: u32 = 0x0200_0000;
    const OWNER_TABLE: u32 = 0x0200_2000;
    const PROCESS_TABLE: u32 = 0x0200_4000;

    fn eax(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn st0(value: f32) -> Ret {
        Ret {
            st0: value as f64,
            ..Ret::default()
        }
    }

    /// An engine with room for the three vtables the tests build and the
    /// exe data the code reads.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for table in [ACTOR_TABLE, OWNER_TABLE, PROCESS_TABLE] {
            e.map(table, 0x600);
        }
        for page in [
            0x0101_1000,
            0x0101_2000,
            0x0101_e000,
            0x0102_1000,
            0x0108_4000,
            0x011d_e000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(DOUBLE_ZERO, 0.0f64);
        e.set_global(DOUBLE_ONE, 1.0f64);
        e.set_global(DOUBLE_HALF, 0.5f64);
        e.set_global(MINIMUM_EXTENT, 32.0f32);
        e.set_global(FOLLOW_CATCH_UP_FACTOR, 1.25f64);
        e.set_global(SNEAK_HEIGHT_FACTOR, 0.5f64);
        e
    }

    /// A function standing in for `addr` that answers `ret` to any call.
    fn double(e: &mut Engine, addr: u32, ret: Ret) {
        e.register_double(addr, move |_, _| ret);
    }

    fn slot_target(table: u32, offset: u32) -> u32 {
        0x0300_0000 + (table - 0x0200_0000) + offset
    }

    /// Puts a function answering `ret` into a vtable slot.
    fn slot(e: &mut Engine, table: u32, offset: u32, ret: Ret) {
        let target = slot_target(table, offset);
        e.mem.set_u32(table + offset, target);
        double(e, target, ret);
    }

    fn new_actor(e: &mut Engine) -> Ptr<Actor> {
        let actor = e.new_object::<Actor>();
        e.mem.set_u32(actor.addr(), ACTOR_TABLE);
        e.mem
            .set_u32(actor.addr() + CACHED_VALUES_OWNER_OFFSET, OWNER_TABLE);
        actor
    }

    fn owner_of(actor: Ptr<Actor>) -> Ptr<CachedValuesOwner> {
        Ptr::new(actor.addr() + CACHED_VALUES_OWNER_OFFSET)
    }

    /// Gives the actor a process (a `HighProcess`-sized block with a vtable).
    fn with_process(e: &mut Engine, actor: Ptr<Actor>) -> Ptr<BaseProcess> {
        let process = Ptr::<BaseProcess>::new(e.mem.alloc(HighProcess::SIZE));
        e.mem.set_u32(process.addr(), PROCESS_TABLE);
        e.set(actor, Actor::pCurrentProcess, process.cast());
        process
    }

    fn with_cached_values(e: &mut Engine, process: Ptr<BaseProcess>) -> Ptr<CachedValues> {
        let cached = e.new_object::<CachedValues>();
        e.set(process, BaseProcess::pCachedValues, cached);
        cached
    }

    /// Doubles for the process-level cached-values helpers.
    fn cached_value_helpers(e: &mut Engine) {
        e.register(HAS_CACHED_VALUES, |e, a| {
            eax((e.mem.u32(a[0] + 0x2c) != 0) as u32)
        });
        e.register(CACHED_VALUES_ADD_FLAGS, |e, a| {
            let flags = e.mem.u32(a[0] + 0x44);
            e.mem.set_u32(a[0] + 0x44, flags | a[1]);
            Ret::default()
        });
    }

    fn logged(e: &Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log.clone().unwrap()
    }

    fn logged_addresses(e: &Engine) -> Vec<u32> {
        logged(e).iter().map(|entry| entry.0).collect()
    }

    /// The bounds virtuals write (6, 10, 8) and (2, 4, 3) and return their
    /// argument; the scale is `scale`.
    fn bounds_and_scale(e: &mut Engine, scale: f32) {
        for (offset, values) in [
            (VSLOT_BOUNDS_UPPER, [6.0f32, 10.0, 8.0]),
            (VSLOT_BOUNDS_LOWER, [2.0f32, 4.0, 3.0]),
        ] {
            let target = slot_target(ACTOR_TABLE, offset);
            e.mem.set_u32(ACTOR_TABLE + offset, target);
            e.register_double(target, move |e, a| {
                for (i, value) in values.iter().enumerate() {
                    e.mem.set_f32(a[1] + 4 * i as u32, *value);
                }
                eax(a[1])
            });
        }
        double(e, GET_SCALE, st0(scale));
        e.register(FLOAT_HELPER_MAX, |_, a| {
            st0(f32::from_bits(a[0]).max(f32::from_bits(a[1])))
        });
    }

    #[test]
    fn swimming_byte_follows_flag_0x800() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        double(&mut e, GET_FLAG_WORD, eax(0x800));
        fn_00884990(&mut e, actor);
        assert!(e.get(actor, Actor::bSwimming));
        double(&mut e, GET_FLAG_WORD, eax(0x7ff));
        fn_00884990(&mut e, actor);
        assert!(!e.get(actor, Actor::bSwimming));
    }

    fn animation_actor(e: &mut Engine, found: u32, passes: u32, kind: u32) -> Ptr<Actor> {
        let actor = new_actor(e);
        slot(e, ACTOR_TABLE, VSLOT_GET_ANIMATION, eax(0x5000));
        double(e, ANIMATION_LOOKUP, eax(found));
        double(e, ANIMATION_STEP, eax(0x7000));
        double(e, ANIMATION_TEST, eax(passes));
        double(e, ANIMATION_KIND, eax(kind));
        double(e, GET_CHAR_CONTROLLER, eax(0));
        actor
    }

    #[test]
    fn animation_kinds_in_the_switch_answer_true() {
        for kind in [0xe3, 0xe5, 0xf1, 0xf2, 0xf3, 0xf4] {
            let mut e = engine();
            let actor = animation_actor(&mut e, 0x6000, 0, kind);
            assert!(fn_008849c0(&mut e, actor), "kind {kind:#x}");
        }
        for kind in [0xe4, 0xe6, 0xf0, 0xf5, 0x100, 0x0] {
            let mut e = engine();
            let actor = animation_actor(&mut e, 0x6000, 0, kind);
            assert!(!fn_008849c0(&mut e, actor), "kind {kind:#x}");
        }
    }

    #[test]
    fn animation_test_or_controller_state_answers_true() {
        let mut e = engine();
        let actor = animation_actor(&mut e, 0x6000, 1, 0);
        assert!(fn_008849c0(&mut e, actor));

        // No animation entry: only the controller counts.
        let mut e = engine();
        let actor = animation_actor(&mut e, 0, 1, 0xe3);
        double(&mut e, GET_CHAR_CONTROLLER, eax(0x8000));
        double(&mut e, CONTROLLER_STATE, eax(2));
        assert!(fn_008849c0(&mut e, actor));
        double(&mut e, CONTROLLER_STATE, eax(1));
        assert!(!fn_008849c0(&mut e, actor));
    }

    #[test]
    fn jump_sound_plays_only_when_the_test_answers() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        double(&mut e, JUMP_TEST, eax(5));
        double(&mut e, PLAY_JUMP, Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(fn_00884aa0(&mut e, actor), 5);
        assert_eq!(logged(&e)[1], (PLAY_JUMP, vec![actor.addr(), 0]));

        double(&mut e, JUMP_TEST, eax(0));
        e.call_log = Some(vec![]);
        assert_eq!(fn_00884aa0(&mut e, actor), 0);
        assert_eq!(logged_addresses(&e), vec![JUMP_TEST]);
    }

    #[test]
    fn object_type_0x12_or_0x15_counts() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        e.register(OBJECT_TYPE, |_, a| eax(a[0]));
        assert!(!fn_00884ae0(&mut e, actor), "no process");
        with_process(&mut e, actor);
        slot(&mut e, PROCESS_TABLE, PROCESS_VSLOT_OBJECT, eax(0));
        assert!(!fn_00884ae0(&mut e, actor), "no object");
        for (object, expected) in [(0x12, true), (0x15, true), (0x13, false)] {
            slot(&mut e, PROCESS_TABLE, PROCESS_VSLOT_OBJECT, eax(object));
            assert_eq!(fn_00884ae0(&mut e, actor), expected, "type {object:#x}");
        }
    }

    #[test]
    fn swimming_is_allowed_when_the_flag_byte_is_set() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        e.set(actor, Actor::bInWater, true);
        e.call_log = Some(vec![]);
        assert!(actor_is_allowed_to_swim(&mut e, actor));
        assert!(logged(&e).is_empty());
    }

    #[test]
    fn swimming_with_a_combat_controller_needs_each_test() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        with_process(&mut e, actor);
        slot(&mut e, PROCESS_TABLE, PROCESS_VSLOT_OBJECT, eax(0x12));
        e.register(OBJECT_TYPE, |_, a| eax(a[0]));
        slot(
            &mut e,
            ACTOR_TABLE,
            VSLOT_GET_COMBAT_CONTROLLER,
            eax(0x9000),
        );
        double(&mut e, SWIM_TEST_FINAL, eax(1));

        // Neither accompanied nor fleeing: forbidden.
        double(&mut e, SWIM_TEST_COMBAT, eax(0));
        double(&mut e, COMBAT_CONTROLLER_IS_FLEEING, eax(0));
        double(&mut e, COMBAT_CONTROLLER_TEST, eax(1));
        assert!(!actor_is_allowed_to_swim(&mut e, actor));
        // Fleeing passes the first test; the second one decides.
        double(&mut e, COMBAT_CONTROLLER_IS_FLEEING, eax(1));
        assert!(actor_is_allowed_to_swim(&mut e, actor));
        double(&mut e, COMBAT_CONTROLLER_TEST, eax(0));
        assert!(!actor_is_allowed_to_swim(&mut e, actor));
        // Without a controller only the final test counts.
        slot(&mut e, ACTOR_TABLE, VSLOT_GET_COMBAT_CONTROLLER, eax(0));
        assert!(actor_is_allowed_to_swim(&mut e, actor));
        double(&mut e, SWIM_TEST_FINAL, eax(0));
        assert!(!actor_is_allowed_to_swim(&mut e, actor));
    }

    #[test]
    fn swimming_without_the_object_type_depends_on_creatures() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        with_process(&mut e, actor);
        slot(&mut e, PROCESS_TABLE, PROCESS_VSLOT_OBJECT, eax(0x13));
        e.register(OBJECT_TYPE, |_, a| eax(a[0]));
        double(&mut e, SWIM_TEST_FINAL, eax(1));
        double(&mut e, SWIM_OBJECT_TEST, eax(1));
        // Not a creature: the final test decides.
        slot(&mut e, ACTOR_TABLE, VSLOT_IS_CREATURE, eax(0));
        assert!(actor_is_allowed_to_swim(&mut e, actor));
        // A creature: the object test decides.
        slot(&mut e, ACTOR_TABLE, VSLOT_IS_CREATURE, eax(1));
        assert!(actor_is_allowed_to_swim(&mut e, actor));
        double(&mut e, SWIM_OBJECT_TEST, eax(0));
        assert!(!actor_is_allowed_to_swim(&mut e, actor));
    }

    #[test]
    fn stop_moving_is_forwarded() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        double(&mut e, STOP_MOVING, Ret::default());
        e.call_log = Some(vec![]);
        fn_00884c60(&mut e, actor);
        assert_eq!(logged(&e), vec![(STOP_MOVING, vec![actor.addr()])]);
    }

    #[test]
    fn cached_walk_speed_is_stored_when_the_process_has_values() {
        let mut e = engine();
        cached_value_helpers(&mut e);
        let actor = new_actor(&mut e);
        let process = with_process(&mut e, actor);
        let cached = with_cached_values(&mut e, process);
        slot(&mut e, ACTOR_TABLE, VSLOT_CALCULATE_WALK_SPEED, st0(3.5));
        assert_eq!(fn_00884c80(&mut e, owner_of(actor)), 3.5);
        assert_eq!(e.get(cached, CachedValues::fCachedWalkSpeed), 3.5);
        assert_eq!(e.get(cached, CachedValues::iFlags), CACHED_WALK_SPEED_FLAG);

        // Without a cached-values block the speed is only returned.
        let plain = new_actor(&mut e);
        with_process(&mut e, plain);
        assert_eq!(fn_00884c80(&mut e, owner_of(plain)), 3.5);
    }

    #[test]
    fn walk_speed_setter_marks_the_flag() {
        let mut e = engine();
        double(&mut e, CACHED_VALUES_ADD_FLAGS, Ret::default());
        let process = e.new_object::<BaseProcess>();
        fn_00884ce0(&mut e, process, 2.0);
        let cached = e.new_object::<CachedValues>();
        e.set(process, BaseProcess::pCachedValues, cached);
        e.call_log = Some(vec![]);
        fn_00884ce0(&mut e, process, 2.0);
        assert_eq!(e.get(cached, CachedValues::fCachedWalkSpeed), 2.0);
        assert_eq!(
            logged(&e),
            vec![(CACHED_VALUES_ADD_FLAGS, vec![cached.addr(), 0x1000])]
        );
    }

    #[test]
    fn cached_run_speed_is_stored_when_the_process_has_values() {
        let mut e = engine();
        cached_value_helpers(&mut e);
        let actor = new_actor(&mut e);
        let process = with_process(&mut e, actor);
        let cached = with_cached_values(&mut e, process);
        slot(&mut e, ACTOR_TABLE, VSLOT_CALCULATE_RUN_SPEED, st0(7.25));
        assert_eq!(fn_00884d20(&mut e, owner_of(actor)), 7.25);
        assert_eq!(e.get(cached, CachedValues::fCachedRunSpeed), 7.25);
        assert_eq!(e.get(cached, CachedValues::iFlags), CACHED_RUN_SPEED_FLAG);
    }

    #[test]
    fn run_speed_setter_marks_the_flag() {
        let mut e = engine();
        double(&mut e, CACHED_VALUES_ADD_FLAGS, Ret::default());
        let process = e.new_object::<BaseProcess>();
        let cached = e.new_object::<CachedValues>();
        e.set(process, BaseProcess::pCachedValues, cached);
        e.call_log = Some(vec![]);
        fn_00884d80(&mut e, process, 9.0);
        assert_eq!(e.get(cached, CachedValues::fCachedRunSpeed), 9.0);
        assert_eq!(
            logged(&e),
            vec![(CACHED_VALUES_ADD_FLAGS, vec![cached.addr(), 0x2000])]
        );
    }

    #[test]
    fn get_walk_speed_uses_the_cache_or_the_virtual() {
        let mut e = engine();
        cached_value_helpers(&mut e);
        let actor = new_actor(&mut e);
        slot(&mut e, ACTOR_TABLE, VSLOT_CALCULATE_WALK_SPEED, st0(1.5));
        // No process: the virtual.
        assert_eq!(actor_get_walk_speed(&mut e, actor), 1.5);
        // A process without cached values: the virtual.
        let process = with_process(&mut e, actor);
        assert_eq!(actor_get_walk_speed(&mut e, actor), 1.5);
        // With cached values and the flag set: the cached value.
        let cached = with_cached_values(&mut e, process);
        e.set(cached, CachedValues::fCachedWalkSpeed, 6.0);
        e.set(cached, CachedValues::iFlags, CACHED_WALK_SPEED_FLAG);
        assert_eq!(actor_get_walk_speed(&mut e, actor), 6.0);
        // Flag clear: the owner's virtual +0x38.
        e.set(cached, CachedValues::iFlags, 0);
        slot(&mut e, OWNER_TABLE, OWNER_VSLOT_WALK_SPEED, st0(2.5));
        assert_eq!(actor_get_walk_speed(&mut e, actor), 2.5);
    }

    #[test]
    fn process_walk_speed_is_zero_without_cached_values() {
        let mut e = engine();
        let process = e.new_object::<BaseProcess>();
        assert_eq!(fn_00884e30(&mut e, process, Ptr::NULL), 0.0);
    }

    #[test]
    fn flag_test_is_true_when_no_mask_bit_is_set() {
        let mut e = engine();
        let cached = e.new_object::<CachedValues>();
        e.set(cached, CachedValues::iFlags, 0x1000);
        assert!(!fn_00884e90(&mut e, cached, 0x1000));
        assert!(fn_00884e90(&mut e, cached, 0x2000));
        assert!(!fn_00884e90(&mut e, cached, 0x3000));
    }

    #[test]
    fn get_run_speed_uses_the_cache_or_the_virtual() {
        let mut e = engine();
        cached_value_helpers(&mut e);
        let actor = new_actor(&mut e);
        slot(&mut e, ACTOR_TABLE, VSLOT_CALCULATE_RUN_SPEED, st0(4.5));
        assert_eq!(actor_get_run_speed(&mut e, actor), 4.5);
        let process = with_process(&mut e, actor);
        assert_eq!(actor_get_run_speed(&mut e, actor), 4.5);
        let cached = with_cached_values(&mut e, process);
        e.set(cached, CachedValues::fCachedRunSpeed, 8.0);
        e.set(cached, CachedValues::iFlags, CACHED_RUN_SPEED_FLAG);
        assert_eq!(actor_get_run_speed(&mut e, actor), 8.0);
        e.set(cached, CachedValues::iFlags, 0);
        slot(&mut e, OWNER_TABLE, OWNER_VSLOT_RUN_SPEED, st0(5.5));
        assert_eq!(actor_get_run_speed(&mut e, actor), 5.5);
    }

    #[test]
    fn process_run_speed_is_zero_without_cached_values() {
        let mut e = engine();
        let process = e.new_object::<BaseProcess>();
        assert_eq!(fn_00884f20(&mut e, process, Ptr::NULL), 0.0);
    }

    #[test]
    fn speed_masks_are_forwarded_run_first() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        double(&mut e, PROCESS_FORWARD_FLAG_MASK, Ret::default());
        e.call_log = Some(vec![]);
        fn_00884f80(&mut e, actor);
        assert!(logged(&e).is_empty(), "no process, no calls");
        let process = with_process(&mut e, actor);
        fn_00884f80(&mut e, actor);
        assert_eq!(
            logged(&e),
            vec![
                (PROCESS_FORWARD_FLAG_MASK, vec![process.addr(), 0x2000]),
                (PROCESS_FORWARD_FLAG_MASK, vec![process.addr(), 0x1000]),
            ]
        );
    }

    #[test]
    fn speed_setters_need_a_process() {
        let mut e = engine();
        cached_value_helpers(&mut e);
        let actor = new_actor(&mut e);
        actor_set_walk_speed(&mut e, actor, 1.0);
        actor_set_run_speed(&mut e, actor, 1.0);
        let process = with_process(&mut e, actor);
        let cached = with_cached_values(&mut e, process);
        actor_set_walk_speed(&mut e, actor, 2.0);
        actor_set_run_speed(&mut e, actor, 3.0);
        assert_eq!(e.get(cached, CachedValues::fCachedWalkSpeed), 2.0);
        assert_eq!(e.get(cached, CachedValues::fCachedRunSpeed), 3.0);
        assert_eq!(e.get(cached, CachedValues::iFlags), 0x3000);
    }

    #[test]
    fn cached_radius_is_half_the_larger_extent_but_at_least_16() {
        let mut e = engine();
        cached_value_helpers(&mut e);
        let actor = new_actor(&mut e);
        let process = with_process(&mut e, actor);
        let cached = with_cached_values(&mut e, process);
        // Extents 4 and 6 times scale 1: below the minimum of 32.
        bounds_and_scale(&mut e, 1.0);
        assert_eq!(fn_00885020(&mut e, owner_of(actor)), 16.0);
        assert_eq!(e.get(cached, CachedValues::fCachedRadius), 16.0);
        assert_eq!(e.get(cached, CachedValues::iFlags), 1);
        // Times scale 8: 48, half of it.
        bounds_and_scale(&mut e, 8.0);
        assert_eq!(fn_00885020(&mut e, owner_of(actor)), 24.0);
    }

    #[test]
    fn radius_setter_marks_the_flag() {
        let mut e = engine();
        cached_value_helpers(&mut e);
        let process = e.new_object::<BaseProcess>();
        fn_00885110(&mut e, process, 1.0);
        let cached = e.new_object::<CachedValues>();
        e.set(process, BaseProcess::pCachedValues, cached);
        fn_00885110(&mut e, process, 12.0);
        assert_eq!(e.get(cached, CachedValues::fCachedRadius), 12.0);
        assert_eq!(e.get(cached, CachedValues::iFlags), 1);
    }

    #[test]
    fn cached_width_is_the_scaled_x_extent() {
        let mut e = engine();
        cached_value_helpers(&mut e);
        let actor = new_actor(&mut e);
        let process = with_process(&mut e, actor);
        let cached = with_cached_values(&mut e, process);
        bounds_and_scale(&mut e, 2.0);
        assert_eq!(fn_00885140(&mut e, owner_of(actor)), 8.0);
        assert_eq!(e.get(cached, CachedValues::fCachedWidth), 8.0);
        assert_eq!(e.get(cached, CachedValues::iFlags), 2);
    }

    #[test]
    fn width_setter_marks_the_flag() {
        let mut e = engine();
        cached_value_helpers(&mut e);
        let process = e.new_object::<BaseProcess>();
        fn_008851e0(&mut e, process, 1.0);
        let cached = e.new_object::<CachedValues>();
        e.set(process, BaseProcess::pCachedValues, cached);
        fn_008851e0(&mut e, process, 3.0);
        assert_eq!(e.get(cached, CachedValues::fCachedWidth), 3.0);
        assert_eq!(e.get(cached, CachedValues::iFlags), 2);
    }

    #[test]
    fn cached_length_is_the_scaled_y_extent() {
        let mut e = engine();
        cached_value_helpers(&mut e);
        let actor = new_actor(&mut e);
        let process = with_process(&mut e, actor);
        let cached = with_cached_values(&mut e, process);
        bounds_and_scale(&mut e, 2.0);
        assert_eq!(fn_00885210(&mut e, owner_of(actor)), 12.0);
        assert_eq!(e.get(cached, CachedValues::fCachedLength), 12.0);
        assert_eq!(e.get(cached, CachedValues::iFlags), 4);
    }

    #[test]
    fn length_setter_marks_the_flag() {
        let mut e = engine();
        cached_value_helpers(&mut e);
        let process = e.new_object::<BaseProcess>();
        fn_008852b0(&mut e, process, 1.0);
        let cached = e.new_object::<CachedValues>();
        e.set(process, BaseProcess::pCachedValues, cached);
        fn_008852b0(&mut e, process, 5.0);
        assert_eq!(e.get(cached, CachedValues::fCachedLength), 5.0);
        assert_eq!(e.get(cached, CachedValues::iFlags), 4);
    }

    #[test]
    fn cached_forward_length_uses_only_the_upper_corner() {
        let mut e = engine();
        cached_value_helpers(&mut e);
        let actor = new_actor(&mut e);
        let process = with_process(&mut e, actor);
        let cached = with_cached_values(&mut e, process);
        bounds_and_scale(&mut e, 2.0);
        e.call_log = Some(vec![]);
        assert_eq!(fn_008852e0(&mut e, owner_of(actor)), 20.0);
        assert!(!logged_addresses(&e).contains(&slot_target(ACTOR_TABLE, VSLOT_BOUNDS_LOWER)));
        assert_eq!(e.get(cached, CachedValues::fCachedForwardLength), 20.0);
        assert_eq!(e.get(cached, CachedValues::iFlags), 0x8000);
    }

    #[test]
    fn forward_length_setter_marks_the_flag() {
        let mut e = engine();
        cached_value_helpers(&mut e);
        let process = e.new_object::<BaseProcess>();
        fn_00885360(&mut e, process, 1.0);
        let cached = e.new_object::<CachedValues>();
        e.set(process, BaseProcess::pCachedValues, cached);
        fn_00885360(&mut e, process, 7.0);
        assert_eq!(e.get(cached, CachedValues::fCachedForwardLength), 7.0);
        assert_eq!(e.get(cached, CachedValues::iFlags), 0x8000);
    }

    #[test]
    fn height_is_computed_without_a_process_and_cached_with_one() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        bounds_and_scale(&mut e, 2.0);
        // No process: (8 - 3) * 2 every time.
        assert_eq!(actor_get_height(&mut e, actor), 10.0);

        // A process that bypasses the cache: also computed.
        let process = with_process(&mut e, actor);
        double(&mut e, CACHED_HEIGHT_BYPASS, eax(1));
        assert_eq!(actor_get_height(&mut e, actor), 10.0);
        assert_eq!(e.mem.f32(process.addr() + 0x42c), 0.0);

        // Caching process, unset: computed and stored.
        double(&mut e, CACHED_HEIGHT_BYPASS, eax(0));
        assert_eq!(actor_get_height(&mut e, actor), 10.0);
        assert_eq!(e.mem.f32(process.addr() + 0x42c), 10.0);

        // Set: returned as stored, bounds not asked.
        e.mem.set_f32(process.addr() + 0x42c, 7.0);
        e.call_log = Some(vec![]);
        assert_eq!(actor_get_height(&mut e, actor), 7.0);
        assert_eq!(logged_addresses(&e), vec![CACHED_HEIGHT_BYPASS]);
    }

    #[test]
    fn cached_height_accessors_use_offset_0x42c() {
        let mut e = engine();
        let process = Ptr::<HighProcess>::new(e.mem.alloc(HighProcess::SIZE));
        fn_008854b0(&mut e, process, 1.75);
        assert_eq!(e.mem.f32(process.addr() + 0x42c), 1.75);
        assert_eq!(fn_00885490(&mut e, process), 1.75);
    }

    #[test]
    fn sneak_height_is_a_fraction_of_height_or_eye_level() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        bounds_and_scale(&mut e, 2.0);
        double(&mut e, GET_EYE_LEVEL, st0(4.0));
        assert_eq!(actor_get_sneak_height(&mut e, actor, false), 5.0);
        assert_eq!(actor_get_sneak_height(&mut e, actor, true), 2.0);
    }

    #[test]
    fn submersion_is_depth_over_height_capped_at_one() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        bounds_and_scale(&mut e, 2.0);
        double(&mut e, GET_RELEVANT_WATER_HEIGHT, st0(10.0));
        double(&mut e, GET_PARENT_CELL, eax(0x9000));
        assert_eq!(
            fn_00885560(&mut e, actor, 4.0, Ptr::NULL),
            (6.0f64 / 10.0) as f32
        );
        assert_eq!(fn_00885560(&mut e, actor, -5.0, Ptr::NULL), 1.0);
        assert_eq!(fn_00885560(&mut e, actor, 10.0, Ptr::NULL), 0.0);
        assert_eq!(fn_00885560(&mut e, actor, 12.0, Ptr::NULL), 0.0);
    }

    #[test]
    fn submersion_uses_the_cell_water_height_for_another_cell() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        bounds_and_scale(&mut e, 2.0);
        double(&mut e, GET_RELEVANT_WATER_HEIGHT, st0(10.0));
        double(&mut e, CELL_GET_WATER_HEIGHT, st0(5.0));
        double(&mut e, GET_PARENT_CELL, eax(0x9000));
        // Another cell: its water, 5 - 0 over 10.
        assert_eq!(fn_00885560(&mut e, actor, 0.0, Ptr::new(0x9100)), 0.5);
        // The actor's own cell: the relevant water height, 10 - 0 over 10.
        assert_eq!(fn_00885560(&mut e, actor, 0.0, Ptr::new(0x9000)), 1.0);
    }

    #[test]
    fn submersion_test_compares_the_threshold() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        bounds_and_scale(&mut e, 2.0);
        double(&mut e, GET_RELEVANT_WATER_HEIGHT, st0(10.0));
        let point = Ptr::new(e.mem.alloc(12));
        e.mem.set_f32(point.addr() + 8, 5.0);
        // Depth is 0.5.
        assert!(fn_00885520(&mut e, actor, point, Ptr::NULL, 0.25));
        assert!(fn_00885520(&mut e, actor, point, Ptr::NULL, 0.5));
        assert!(!fn_00885520(&mut e, actor, point, Ptr::NULL, 0.75));
    }

    const WORLD_SPACE: u32 = 0xa000;

    /// Two actors in the same world space whose current package targets
    /// the second, plus the doubles the follow code calls.
    fn follow_setup(e: &mut Engine) -> (Ptr<Actor>, Ptr<Actor>) {
        let actor = new_actor(e);
        let target = new_actor(e);
        slot(e, ACTOR_TABLE, VSLOT_IS_ACTOR, eax(1));
        double(e, GET_CURRENT_PACKAGE, eax(0xb000));
        double(e, GET_CURRENT_PACKAGE_TARGET, eax(target.addr()));
        double(e, GET_PARENT_CELL, eax(0x9000));
        double(e, GET_WORLD_SPACE, eax(WORLD_SPACE));
        double(e, CELL_TEST_INTERIOR, eax(0));
        double(e, FOLLOW_RADIUS_NEAR, st0(2.0));
        double(e, FOLLOW_RADIUS_MID, st0(6.0));
        double(e, FOLLOW_RADIUS_WALK, st0(3.0));
        double(e, FOLLOW_RADIUS_MATCH_SPEED, st0(10.0));
        double(e, GET_CURRENT_SPEED, st0(4.0));
        double(e, GET_SCALE, st0(1.0));
        double(e, GET_FLAG_WORD, eax(0x20f));
        e.register(FLOAT_HELPER_MAX, |_, a| {
            st0(f32::from_bits(a[0]).max(f32::from_bits(a[1])))
        });
        e.register(FLOAT_HELPER_MIN, |_, a| {
            st0(f32::from_bits(a[0]).min(f32::from_bits(a[1])))
        });
        e.register(SETTING_VALUE_ADDRESS, |e, a| {
            let value = match a[0] {
                SETTING_UPPER_FACTOR_ALL_FLAGS => 2.0f32,
                SETTING_UPPER_FACTOR_OTHERWISE => 3.0,
                SETTING_LOWER_FACTOR_ALL_FLAGS => 0.5,
                SETTING_LOWER_FACTOR_OTHERWISE => 0.25,
                SETTING_INTERPOLATION_FACTOR => 0.5,
                other => panic!("unexpected setting {other:08x}"),
            };
            let address = e.mem.alloc(4);
            e.mem.set_f32(address, value);
            eax(address)
        });
        e.register(CLAMP_FLOAT, |e, a| {
            let (lower, upper) = (f32::from_bits(a[1]), f32::from_bits(a[2]));
            let value = e.mem.f32(a[0]);
            if value > upper {
                e.mem.set_f32(a[0], upper);
            } else if value < lower {
                e.mem.set_f32(a[0], lower);
            }
            Ret::default()
        });
        (actor, target)
    }

    fn adjusted_speed(e: &mut Engine, actor: Ptr<Actor>, speed: f32, distance: f32) -> f32 {
        let cell = Ptr::new(e.mem.alloc(4));
        e.mem.set_f32(cell.addr(), speed);
        actor_adjust_speed_for_following(e, actor, cell, 1.0, distance);
        e.mem.f32(cell.addr())
    }

    #[test]
    fn following_inside_the_mid_radius_ramps_from_the_near_radius() {
        let mut e = engine();
        let (actor, _) = follow_setup(&mut e);
        // nearer = 5 - 2, span = 6 - 2, lesser speed = min(2, 4): 3/4 * 2,
        // inside the clamp [2 * 0.5, 2 * 2].
        assert_eq!(adjusted_speed(&mut e, actor, 2.0, 5.0), 1.5);
        // Clamped to the lower limit 2 * 0.5.
        assert_eq!(adjusted_speed(&mut e, actor, 2.0, 3.0), 1.0);
    }

    #[test]
    fn following_past_the_far_radius_catches_up_when_all_flags_are_set() {
        let mut e = engine();
        let (actor, _) = follow_setup(&mut e);
        // 4 * 1.25 inside [2, 8].
        assert_eq!(adjusted_speed(&mut e, actor, 4.0, 12.0), 5.0);
        // The clamp's upper limit 2 * 2 wins over 5.
        assert_eq!(adjusted_speed(&mut e, actor, 2.0, 12.0), 4.0);
    }

    #[test]
    fn following_between_the_radii_interpolates() {
        let mut e = engine();
        let (actor, _) = follow_setup(&mut e);
        // (1 + (8 - 6) / (10 - 6) * 0.5) * 4.
        assert_eq!(adjusted_speed(&mut e, actor, 4.0, 8.0), 5.0);
        // Without all flags the far radius interpolates too:
        // (1 + (12 - 6) / 4 * 0.5) * 4 = 7, inside [1, 12].
        double(&mut e, GET_FLAG_WORD, eax(0x20e));
        assert_eq!(adjusted_speed(&mut e, actor, 4.0, 12.0), 7.0);
    }

    #[test]
    fn following_scales_the_target_speed_by_the_scale_ratio() {
        let mut e = engine();
        let (actor, target) = follow_setup(&mut e);
        e.register_double(GET_SCALE, move |_, a| {
            st0(if a[0] == target.addr() { 3.0 } else { 1.5 })
        });
        // Target speed 4 * (3 / 1.5) = 8: (1 + 0.25) * 8 = 10, inside the
        // clamp [4, 16] for the old speed 8.
        assert_eq!(adjusted_speed(&mut e, actor, 8.0, 8.0), 10.0);
    }

    #[test]
    fn following_needs_an_actor_target_in_the_same_area() {
        let mut e = engine();
        let (actor, target) = follow_setup(&mut e);
        // No target.
        double(&mut e, GET_CURRENT_PACKAGE_TARGET, eax(0));
        assert_eq!(adjusted_speed(&mut e, actor, 4.0, 12.0), 4.0);
        // The actor itself.
        double(&mut e, GET_CURRENT_PACKAGE_TARGET, eax(actor.addr()));
        assert_eq!(adjusted_speed(&mut e, actor, 4.0, 12.0), 4.0);
        // Not an actor.
        double(&mut e, GET_CURRENT_PACKAGE_TARGET, eax(target.addr()));
        slot(&mut e, ACTOR_TABLE, VSLOT_IS_ACTOR, eax(0));
        assert_eq!(adjusted_speed(&mut e, actor, 4.0, 12.0), 4.0);
        slot(&mut e, ACTOR_TABLE, VSLOT_IS_ACTOR, eax(1));
        // Another world space.
        e.register_double(GET_WORLD_SPACE, move |_, a| {
            eax(if a[0] == target.addr() {
                0xc000
            } else {
                WORLD_SPACE
            })
        });
        assert_eq!(adjusted_speed(&mut e, actor, 4.0, 12.0), 4.0);
        // Interior cells decide by cell instead: equal cells match whatever
        // the world spaces say, different cells differ.
        double(&mut e, CELL_TEST_INTERIOR, eax(1));
        assert_eq!(adjusted_speed(&mut e, actor, 4.0, 12.0), 5.0);
        e.register_double(GET_PARENT_CELL, move |_, a| {
            eax(if a[0] == target.addr() {
                0x9100
            } else {
                0x9000
            })
        });
        assert_eq!(adjusted_speed(&mut e, actor, 4.0, 12.0), 4.0);
    }

    #[test]
    fn follow_mode_is_decided_by_distance_and_flags() {
        let mut e = engine();
        let (actor, target) = follow_setup(&mut e);
        double(&mut e, FOLLOW_MODE_ALLOWED, eax(1));
        e.set_global(PLAYER_CHARACTER, 0u32);
        double(&mut e, GET_DISTANCE_FROM_REFERENCE, st0(5.0));
        double(&mut e, GET_FLAG_WORD, eax(0));
        // Walk radius 3 < distance 5 < match radius 10: unchanged.
        assert_eq!(
            actor_modify_move_mode_for_follow(&mut e, actor, 0x300, 1.0, 0.0),
            0x300
        );
        // Closer than the walk radius with no low flag bits: 0x100 replaces
        // 0x200.
        double(&mut e, GET_DISTANCE_FROM_REFERENCE, st0(2.0));
        assert_eq!(
            actor_modify_move_mode_for_follow(&mut e, actor, 0x2ff, 1.0, 0.0),
            0x1ff
        );
        // ... but not with a low flag bit set.
        double(&mut e, GET_FLAG_WORD, eax(1));
        assert_eq!(
            actor_modify_move_mode_for_follow(&mut e, actor, 0x2ff, 1.0, 0.0),
            0x2ff
        );
        // Farther than the match radius: 0x200 replaces 0x100.
        double(&mut e, GET_DISTANCE_FROM_REFERENCE, st0(11.0));
        assert_eq!(
            actor_modify_move_mode_for_follow(&mut e, actor, 0x1ff, 1.0, 0.0),
            0x2ff
        );
        // ... unless the follow code is not allowed to.
        double(&mut e, FOLLOW_MODE_ALLOWED, eax(0));
        assert_eq!(
            actor_modify_move_mode_for_follow(&mut e, actor, 0x1ff, 1.0, 0.0),
            0x1ff
        );
        // The target's own flags.
        double(&mut e, FOLLOW_MODE_ALLOWED, eax(1));
        double(&mut e, GET_DISTANCE_FROM_REFERENCE, st0(5.0));
        for flags in [0x400u32, 0x200] {
            double(&mut e, GET_FLAG_WORD, eax(flags));
            assert_eq!(
                actor_modify_move_mode_for_follow(&mut e, actor, 0x100, 1.0, 0.0),
                0x200,
                "flags {flags:#x}"
            );
        }
        double(&mut e, GET_FLAG_WORD, eax(0x500));
        assert_eq!(
            actor_modify_move_mode_for_follow(&mut e, actor, 0x100, 1.0, 0.0),
            0x100
        );
        // The player as the target with mode bit 0x800.
        double(&mut e, GET_FLAG_WORD, eax(0));
        e.set_global(PLAYER_CHARACTER, target.addr());
        assert_eq!(
            actor_modify_move_mode_for_follow(&mut e, actor, 0x900, 1.0, 0.0),
            0xa00
        );
        assert_eq!(
            actor_modify_move_mode_for_follow(&mut e, actor, 0x100, 1.0, 0.0),
            0x100
        );
    }

    #[test]
    fn follow_mode_is_unchanged_without_an_actor_target() {
        let mut e = engine();
        let (actor, _) = follow_setup(&mut e);
        double(&mut e, GET_CURRENT_PACKAGE_TARGET, eax(0));
        assert_eq!(
            actor_modify_move_mode_for_follow(&mut e, actor, 0x2ff, 1.0, 0.0),
            0x2ff
        );
        let other = new_actor(&mut e);
        double(&mut e, GET_CURRENT_PACKAGE_TARGET, eax(other.addr()));
        slot(&mut e, ACTOR_TABLE, VSLOT_IS_ACTOR, eax(0));
        assert_eq!(
            actor_modify_move_mode_for_follow(&mut e, actor, 0x2ff, 1.0, 0.0),
            0x2ff
        );
    }

    #[test]
    fn form_flag_is_checked_only_for_three_form_types() {
        let mut e = engine();
        e.register(SAVE_FORM_BUFFER_GET_FORM, |_, a| eax(a[0]));
        e.register(FORM_TYPE, |e, a| eax(e.mem.u8(a[0] + 4) as u32));
        let form = Ptr::new(e.mem.alloc(16));
        for (form_type, flags, expected) in [
            (0x15u8, 0x2000_0000u32, true),
            (0x15, 0, false),
            (0x27, 0, false),
            (0x30, 0x2000_0000, true),
            (0x30, 0, false),
            (0x14, 0, true),
            (0x00, 0, true),
        ] {
            e.mem.set_u8(form.addr() + 4, form_type);
            e.mem.set_u32(form.addr() + 8, flags);
            assert_eq!(
                fn_008859e0(&mut e, Ptr::NULL, form),
                expected,
                "type {form_type:#x} flags {flags:#x}"
            );
        }
    }

    #[test]
    fn form_flag_bit_20000000() {
        let mut e = engine();
        let form = Ptr::new(e.mem.alloc(16));
        assert!(!fn_00885a30(&mut e, form));
        e.mem.set_u32(form.addr() + 8, 0x2000_0000);
        assert!(fn_00885a30(&mut e, form));
        e.mem.set_u32(form.addr() + 8, 0xdfff_ffff);
        assert!(!fn_00885a30(&mut e, form));
    }

    /// Doubles for what both speed calculations call.
    fn speed_formula_setup(e: &mut Engine) -> Ptr<Actor> {
        let actor = new_actor(e);
        with_process(e, actor);
        slot(e, PROCESS_TABLE, PROCESS_VSLOT_FLAG, eax(0));
        slot(e, PROCESS_TABLE, PROCESS_VSLOT_ITEM, eax(0x111));
        slot(e, ACTOR_TABLE, VSLOT_FLAG_0X358, eax(1));
        slot(e, ACTOR_TABLE, VSLOT_IS_CREATURE, eax(0));
        double(e, SPEED_FLAG_TEST, eax(1));
        double(e, GET_ARMOR_BEING_WORN, eax(0x222));
        e.register(ITEM_VALUE, |_, a| eax(a[0] + 1));
        double(e, ITEM_RELEASE, Ret::default());
        double(e, CALCULATE_WALK_SPEED_FORMULA, st0(4.0));
        double(e, CALCULATE_RUN_SPEED_FORMULA, st0(4.0));
        slot(e, ACTOR_TABLE, VSLOT_GET_COMBAT_CONTROLLER, eax(0));
        actor
    }

    #[test]
    fn walk_speed_formula_gets_the_actor_values_in_order() {
        let mut e = engine();
        let actor = speed_formula_setup(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(actor_calculate_walk_speed(&mut e, actor), 4.0);
        let log = logged(&e);
        let formula = log
            .iter()
            .find(|entry| entry.0 == CALCULATE_WALK_SPEED_FORMULA)
            .unwrap();
        // value owner, item value, armor value, 004997b0, process flag
        // negated, creature, virtual +0x358.
        assert_eq!(
            formula.1,
            vec![actor.addr() + 0xa4, 0x112, 0x223, 1, 1, 0, 1]
        );
        assert!(log.contains(&(ITEM_RELEASE, vec![0x222, 1])));
        assert!(log.contains(&(GET_ARMOR_BEING_WORN, vec![actor.addr(), 2])));
    }

    #[test]
    fn walk_speed_is_scaled_by_the_combat_controller() {
        let mut e = engine();
        let actor = speed_formula_setup(&mut e);
        let controller = Ptr::new(e.mem.alloc(0x100));
        e.mem.set_f32(controller.addr() + 0xd8, 1.5);
        slot(
            &mut e,
            ACTOR_TABLE,
            VSLOT_GET_COMBAT_CONTROLLER,
            eax(controller.addr()),
        );
        assert_eq!(actor_calculate_walk_speed(&mut e, actor), 6.0);
        assert_eq!(fn_00885bd0(&mut e, controller), 1.5);
    }

    #[test]
    fn walk_speed_without_items_passes_zero_values() {
        let mut e = engine();
        let actor = speed_formula_setup(&mut e);
        slot(&mut e, PROCESS_TABLE, PROCESS_VSLOT_ITEM, eax(0));
        slot(&mut e, PROCESS_TABLE, PROCESS_VSLOT_FLAG, eax(1));
        double(&mut e, GET_ARMOR_BEING_WORN, eax(0));
        e.call_log = Some(vec![]);
        actor_calculate_walk_speed(&mut e, actor);
        let log = logged(&e);
        assert!(!log.iter().any(|entry| entry.0 == ITEM_VALUE));
        assert!(!log.iter().any(|entry| entry.0 == ITEM_RELEASE));
        let formula = log
            .iter()
            .find(|entry| entry.0 == CALCULATE_WALK_SPEED_FORMULA)
            .unwrap();
        assert_eq!(formula.1, vec![actor.addr() + 0xa4, 0, 0, 1, 0, 0, 1]);
    }

    #[test]
    fn run_speed_goes_through_the_entry_point() {
        let mut e = engine();
        let actor = speed_formula_setup(&mut e);
        e.register(HANDLE_ENTRY_POINT, |e, a| {
            let value = e.mem.f32(a[2]);
            e.mem.set_f32(a[2], value + 1.0);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(actor_calculate_run_speed(&mut e, actor), 5.0);
        let log = logged(&e);
        let formula = log
            .iter()
            .find(|entry| entry.0 == CALCULATE_RUN_SPEED_FORMULA)
            .unwrap();
        // value owner, item value, armor value, process flag negated,
        // creature, 004997b0, virtual +0x358.
        assert_eq!(
            formula.1,
            vec![actor.addr() + 0xa4, 0x112, 0x223, 1, 0, 1, 1]
        );
        let entry = log
            .iter()
            .find(|entry| entry.0 == HANDLE_ENTRY_POINT)
            .unwrap();
        assert_eq!(entry.1[0..2], [0x2a, actor.addr()]);
    }

    #[test]
    fn run_speed_is_scaled_by_the_combat_controller_before_the_entry_point() {
        let mut e = engine();
        let actor = speed_formula_setup(&mut e);
        let controller = Ptr::new(e.mem.alloc(0x100));
        e.mem.set_f32(controller.addr() + 0xdc, 0.5);
        slot(
            &mut e,
            ACTOR_TABLE,
            VSLOT_GET_COMBAT_CONTROLLER,
            eax(controller.addr()),
        );
        e.register(HANDLE_ENTRY_POINT, |e, a| {
            let value = e.mem.f32(a[2]);
            e.mem.set_f32(a[2], value + 1.0);
            Ret::default()
        });
        assert_eq!(actor_calculate_run_speed(&mut e, actor), 3.0);
        assert_eq!(fn_00885d70(&mut e, controller), 0.5);
    }

    // -----------------------------------------------------------------
    // Tests of the second block (b0148p2)

    const OTHER_TABLE: u32 = 0x0200_6000;
    const DEFAULT_TARGET: u32 = 0x0300_f000;
    const SCRATCH_FLOAT: u32 = 0x0101_2100;

    /// Callees of the original block that the new functions also call.
    const EARLIER_CALLEES: [u32; 25] = [
        GET_FLAG_WORD,
        SPEED_FLAG_TEST,
        GET_ARMOR_BEING_WORN,
        ITEM_VALUE,
        ITEM_RELEASE,
        HAS_CACHED_VALUES,
        PROCESS_FORWARD_FLAG_MASK,
        STOP_MOVING,
        OBJECT_TYPE,
        GET_EYE_LEVEL,
        CLAMP_FLOAT,
        GET_PARENT_CELL,
        CELL_TEST_INTERIOR,
        SAVE_FORM_BUFFER_GET_FORM,
        FORM_TYPE,
        GET_RELEVANT_WATER_HEIGHT,
        GET_SCALE,
        ANIMATION_LOOKUP,
        ANIMATION_STEP,
        GET_CHAR_CONTROLLER,
        CACHED_HEIGHT_BYPASS,
        FOLLOW_MODE_ALLOWED,
        HANDLE_ENTRY_POINT,
        CONTROLLER_STATE,
        CELL_GET_WATER_HEIGHT,
    ];

    const CODE_CALLEES: [u32; 285] = [
        0x00401000, 0x00403df0, 0x00406d30, 0x00408d60, 0x0040ab30, 0x00416870, 0x00418890,
        0x00418900, 0x00418940, 0x00418a00, 0x0041a090, 0x0041a250, 0x0041a540, 0x0041a800,
        0x0041d7b0, 0x00421400, 0x0042e040, 0x0042e8c0, 0x004301b0, 0x00430830, 0x00436aa0,
        0x00437bd0, 0x00439e90, 0x00439ef0, 0x0043b4d0, 0x0043b4f0, 0x0043d450, 0x0043d4d0,
        0x0043f850, 0x0043f8d0, 0x0043fa80, 0x0043fcd0, 0x00440460, 0x00440da0, 0x00446390,
        0x0044ddc0, 0x00450b80, 0x00450ff0, 0x004511e0, 0x00453700, 0x004537b0, 0x00453a70,
        0x00457990, 0x0045bb80, 0x0045cbc0, 0x0045cd60, 0x0047fd90, 0x00483710, 0x0048f810,
        0x0048ffd0, 0x00490330, 0x00491180, 0x004937e0, 0x004938e0, 0x00493900, 0x004939d0,
        0x00493bb0, 0x00493bd0, 0x00494390, 0x00494710, 0x004955a0, 0x004955c0, 0x00496080,
        0x004964d0, 0x00498910, 0x004994f0, 0x0049bdc0, 0x004a0c90, 0x004a3760, 0x004a3a20,
        0x004a3e90, 0x004a3f10, 0x004a4240, 0x004ade00, 0x004ae750, 0x004b3ae0, 0x004b4500,
        0x004b6e70, 0x004d50e0, 0x004dd150, 0x004e75d0, 0x004eaf60, 0x004f15a0, 0x004f8960,
        0x00508100, 0x0050d4a0, 0x00511840, 0x00517630, 0x0051f5f0, 0x00524ac0, 0x00524b40,
        0x00524d10, 0x00525420, 0x00526100, 0x00547770, 0x0054a070, 0x0054ddd0, 0x00550200,
        0x00552490, 0x005532a0, 0x00559450, 0x0055b980, 0x005609b0, 0x00560d30, 0x00561580,
        0x005653d0, 0x00566950, 0x00566970, 0x00567050, 0x00567730, 0x00567750, 0x0056ae60,
        0x0056fac0, 0x00571560, 0x005715d0, 0x005725f0, 0x00572e50, 0x00573800, 0x00574400,
        0x00575830, 0x00575b70, 0x00576d30, 0x00579670, 0x00580060, 0x005a2030, 0x005b5e40,
        0x005b9e80, 0x005c53d0, 0x005ce8f0, 0x005d43c0, 0x005e39b0, 0x005f2440, 0x005f2540,
        0x005f2670, 0x005f92d0, 0x00621480, 0x00621a40, 0x00622570, 0x00629670, 0x0062c430,
        0x0062eb50, 0x00644a50, 0x00647f50, 0x00647f90, 0x00647fd0, 0x00648a10, 0x00648a50,
        0x00653270, 0x00678610, 0x0067a4f0, 0x006815c0, 0x006838b0, 0x00689210, 0x0068a7d0,
        0x006d2c20, 0x006fa820, 0x00702360, 0x00702680, 0x0070f490, 0x00717e50, 0x00726070,
        0x00778950, 0x007d1d00, 0x007d6bb0, 0x007df1f0, 0x008041a0, 0x00810610, 0x00824970,
        0x00849730, 0x0084d030, 0x0084e3a0, 0x00867950, 0x00867d60, 0x00867da0, 0x00867e30,
        0x00867ef0, 0x0087ab50, 0x0087b850, 0x0087ce50, 0x0087e9a0, 0x0087eeb0, 0x0087f390,
        0x0087f3d0, 0x0087f4a0, 0x0087f570, 0x0087f5c0, 0x0087f620, 0x00886b10, 0x00888960,
        0x00888970, 0x0088b030, 0x0088b0a0, 0x0088b0c0, 0x0088b0f0, 0x00894900, 0x00894cc0,
        0x00894d60, 0x00895110, 0x008984c0, 0x008985d0, 0x0089f580, 0x008a0960, 0x008a0c60,
        0x008a1710, 0x008a1800, 0x008a40e0, 0x008a6840, 0x008a6970, 0x008a6ce0, 0x008a7a40,
        0x008a7d50, 0x008adcb0, 0x008b00c0, 0x008b28c0, 0x008b3230, 0x008b39f0, 0x008b3a80,
        0x008b70d0, 0x008bbdb0, 0x008bcdf0, 0x008bd7b0, 0x008be7a0, 0x008bffa0, 0x008c1470,
        0x008c26e0, 0x008c2b60, 0x008c3c40, 0x008c4330, 0x008c51f0, 0x008c7aa0, 0x008d0370,
        0x008d5cb0, 0x008d8520, 0x008d85e0, 0x008fec10, 0x009314d0, 0x00931850, 0x00931ed0,
        0x00933270, 0x009334b0, 0x0094df60, 0x00950a60, 0x00952290, 0x00953c50, 0x009549a0,
        0x009611e0, 0x00968730, 0x0096e870, 0x0098adb0, 0x009a1260, 0x009bb080, 0x009c8d60,
        0x009dcb90, 0x009dccf0, 0x00ad7480, 0x00ad7550, 0x00ad8830, 0x00ad88f0, 0x00ad8930,
        0x00ad89e0, 0x00ad8b60, 0x00ad8ce0, 0x00ad8d10, 0x00ad8f20, 0x00b4f5c0, 0x00b5d9f0,
        0x00b5f080, 0x00b65f60, 0x00c66230, 0x00c6a350, 0x00c6c3d0, 0x00c6d4d0, 0x00c6d6b0,
        0x00c7c150, 0x00c7d630, 0x00c802d0, 0x00c804d0, 0x00c821b0, 0x00c82580, 0x00c8cce0,
        0x00c8dac0, 0x00c9b670, 0x00ca2ad0, 0x00cb8fe0, 0x00ec62c0,
    ];
    const GLOBAL_ADDRESSES: [u32; 70] = [
        0x01013dc0, 0x01015f5c, 0x01016408, 0x01016ff0, 0x0101712c, 0x01017718, 0x01017b70,
        0x0101a6b0, 0x0101e2bc, 0x0101e2c0, 0x0101ffa0, 0x01020998, 0x010231b0, 0x010241b0,
        0x0102caf8, 0x01035838, 0x0104ee98, 0x0104f4b0, 0x01073828, 0x01084838, 0x01084840,
        0x01084898, 0x010848a0, 0x010848b0, 0x010848b8, 0x010848d0, 0x0118a838, 0x01199760,
        0x011a32d8, 0x011a9484, 0x011c3ea4, 0x011c6260, 0x011d0460, 0x011d058c, 0x011d0c3c,
        0x011d0fa8, 0x011d12c0, 0x011d1458, 0x011d1464, 0x011ddf38, 0x011de45c, 0x011de7b8,
        0x011dea10, 0x011df69c, 0x011df69d, 0x011df69e, 0x011df69f, 0x011df6b0, 0x011df71d,
        0x011e0550, 0x011e07a8, 0x011e07d0, 0x011e07d4, 0x011e07e8, 0x011e0888, 0x011e0e80,
        0x011f1220, 0x011f122c, 0x011f21cc, 0x011f2250, 0x011f2ac4, 0x011f2ad8, 0x011f2b10,
        0x011f2c10, 0x011f3e60, 0x011f426c, 0x011f6394, 0x012677a3, 0x01267e64, 0x012683ec,
    ];

    /// An engine where every callee of the second block answers zero, the
    /// vtables have default slots and the globals' pages exist.
    fn engine2() -> Engine {
        let mut e = engine();
        for table in [ACTOR_TABLE, OWNER_TABLE, PROCESS_TABLE, OTHER_TABLE] {
            e.map(table, 0x800);
            for offset in (0..0x800).step_by(4) {
                e.mem.set_u32(table + offset, DEFAULT_TARGET);
            }
        }
        e.register(DEFAULT_TARGET, |_, _| Ret::default());
        for address in GLOBAL_ADDRESSES {
            e.map(address, 0x10);
        }
        for address in CODE_CALLEES.iter().chain(EARLIER_CALLEES.iter()) {
            e.register(*address, |_, _| Ret::default());
        }
        e.register(GET_PROCESS, |e, a| eax(e.mem.u32(a[0] + 0x68)));
        e.register(SETTING_VALUE_ADDRESS, |_, _| eax(SCRATCH_FLOAT));
        e.register(EMPTY_FUNCTION, |_, _| Ret::default());
        e
    }

    fn actor2(e: &mut Engine) -> Ptr<Actor> {
        let actor = new_actor(e);
        e.mem.set_u32(actor.addr() + 0x94, OTHER_TABLE);
        e.mem
            .set_u32(actor.addr() + ACTOR_VALUE_OWNER_OFFSET, OTHER_TABLE);
        actor
    }

    fn set_player(e: &mut Engine, player: Ptr<Actor>) {
        e.mem.set_u32(PLAYER_CHARACTER, player.addr());
    }

    fn args_of(e: &Engine, address: u32) -> Vec<u32> {
        logged(e)
            .into_iter()
            .find(|entry| entry.0 == address)
            .unwrap_or_else(|| panic!("{address:08x} was not called"))
            .1
    }

    fn was_called(e: &Engine, address: u32) -> bool {
        logged(e).iter().any(|entry| entry.0 == address)
    }

    #[test]
    fn weighted_speed_passes_the_six_formula_words() {
        let mut e = engine2();
        let actor = speed_formula_setup(&mut e);
        double(&mut e, SPEED_FORMULA_00647F50, st0(4.0));
        e.call_log = Some(vec![]);
        assert_eq!(fn_00885d90(&mut e, actor), 4.0);
        assert_eq!(
            args_of(&e, SPEED_FORMULA_00647F50),
            vec![actor.addr() + 0xa4, 0x112, 0x223, 1, 0, 1]
        );
    }

    #[test]
    fn second_weighted_speed_uses_the_other_formula() {
        let mut e = engine2();
        let actor = speed_formula_setup(&mut e);
        double(&mut e, SPEED_FORMULA_00647F90, st0(5.0));
        e.call_log = Some(vec![]);
        assert_eq!(fn_00885ed0(&mut e, actor), 5.0);
        assert!(was_called(&e, SPEED_FORMULA_00647F90));
    }

    #[test]
    fn speed_00886010_is_zero_when_the_virtual_says_so() {
        let mut e = engine2();
        let actor = actor2(&mut e);
        slot(&mut e, ACTOR_TABLE, VSLOT_FLAG_0X358, eax(1));
        assert_eq!(fn_00886010(&mut e, actor), 0.0);
    }

    #[test]
    fn speed_00886010_applies_the_formula_to_the_actor_value() {
        let mut e = engine2();
        let actor = actor2(&mut e);
        double(&mut e, ACTOR_SPEED_VALUE_008A0C60, st0(2.5));
        double(&mut e, SPEED_FORMULA_00647FD0, st0(7.0));
        e.call_log = Some(vec![]);
        assert_eq!(fn_00886010(&mut e, actor), 7.0);
        let formula = args_of(&e, SPEED_FORMULA_00647FD0);
        assert_eq!(formula, vec![actor.addr() + 0xa4, 2.5f32.to_bits()]);
    }

    #[test]
    fn package_location_needs_a_process_and_an_object() {
        let mut e = engine2();
        let actor = actor2(&mut e);
        assert_eq!(actor_get_current_package_location(&mut e, actor), 0);
        with_process(&mut e, actor);
        assert_eq!(actor_get_current_package_location(&mut e, actor), 0);
        slot(&mut e, PROCESS_TABLE, PROCESS_VSLOT_0X27C, eax(0x1234));
        double(&mut e, PACKAGE_LOCATION_OF, eax(0x99));
        assert_eq!(actor_get_current_package_location(&mut e, actor), 0x99);
    }

    #[test]
    fn process_level_without_a_process_is_false() {
        let mut e = engine2();
        let actor = actor2(&mut e);
        assert!(!fn_008860d0(&mut e, actor));
    }

    #[test]
    fn process_level_already_desired_is_true() {
        let mut e = engine2();
        let actor = actor2(&mut e);
        with_process(&mut e, actor);
        double(&mut e, PROCESS_GET_LEVEL, eax(2));
        double(&mut e, GET_DESIRED_PROCESS_LEVEL, eax(2));
        assert!(fn_008860d0(&mut e, actor));
    }

    #[test]
    fn process_level_unloads_a_non_persistent_reference() {
        let mut e = engine2();
        let actor = actor2(&mut e);
        with_process(&mut e, actor);
        double(&mut e, PROCESS_GET_LEVEL, eax(3));
        double(&mut e, GET_DESIRED_PROCESS_LEVEL, eax(2));
        double(&mut e, GET_REF_PERSISTS, eax(0));
        double(&mut e, GET_PARENT_CELL, eax(0x5000));
        e.call_log = Some(vec![]);
        assert!(!fn_008860d0(&mut e, actor));
        let log = logged(&e);
        assert!(was_called(&e, ACTOR_CAST_PERMANENT_MAGIC));
        assert!(was_called(&e, SAVE_LOAD_UNLOAD_FORM));
        assert_eq!(args_of(&e, ACTOR_DISPEL_ALL), vec![actor.addr() + 0x94]);
        assert!(log.iter().any(|entry| entry.0 == DEFAULT_TARGET));
    }

    #[test]
    fn process_level_keeps_a_persistent_reference() {
        let mut e = engine2();
        let actor = actor2(&mut e);
        with_process(&mut e, actor);
        double(&mut e, PROCESS_GET_LEVEL, eax(3));
        double(&mut e, GET_DESIRED_PROCESS_LEVEL, eax(3));
        double(&mut e, GET_REF_PERSISTS, eax(1));
        assert!(fn_008860d0(&mut e, actor));
    }

    #[test]
    fn update_returns_at_once_when_the_actor_is_not_processed() {
        let mut e = engine2();
        let actor = actor2(&mut e);
        e.call_log = Some(vec![]);
        fn_00886360(&mut e, actor, 0.1);
        assert_eq!(logged(&e).len(), 1);
        double(&mut e, ACTOR_SKIP_UPDATE_TEST, eax(1));
        e.set(actor, Actor::bProcessMe, true);
        fn_00886360(&mut e, actor, 0.1);
        assert!(!was_called(&e, EMPTY_FUNCTION));
    }

    #[test]
    fn update_evaluates_the_package_of_a_living_actor() {
        let mut e = engine2();
        let actor = actor2(&mut e);
        let other = actor2(&mut e);
        set_player(&mut e, other);
        with_process(&mut e, actor);
        e.set(actor, Actor::bProcessMe, true);
        e.mem.set_u8(PROCESSING_ENABLED, 1);
        double(&mut e, UPDATE_STEP_SETTING, st0(0.5));
        double(&mut e, SETTING_INT_VALUE, eax(1));
        double(&mut e, GLOBAL_WORD_ADDRESS, eax(SCRATCH_FLOAT));
        double(&mut e, SETTING_VALUE_POINTER, eax(SCRATCH_FLOAT));
        slot(
            &mut e,
            ACTOR_TABLE,
            VSLOT_EVALUATE_PACKAGE_TEST_0X26C,
            eax(1),
        );
        double(&mut e, ACTOR_EVALUATE_FLAG, eax(1));
        e.call_log = Some(vec![]);
        fn_00886360(&mut e, actor, 0.25);
        assert_eq!(
            args_of(&e, ACTOR_EVALUATE_PACKAGE),
            vec![actor.addr(), 1, 1]
        );
        assert!(was_called(&e, EXTRA_LIST_SET_DAY));
    }

    #[test]
    fn update_runs_the_dead_body_timers_down() {
        let mut e = engine2();
        let actor = actor2(&mut e);
        let other = actor2(&mut e);
        set_player(&mut e, other);
        with_process(&mut e, actor);
        e.set(actor, Actor::bProcessMe, true);
        e.set(actor, Actor::bDeadFlag, true);
        e.set(actor, Actor::fCheckMyDeadBodyTimer, 5.0);
        e.set(actor, Actor::fDeadBodyAlarm, 1.0);
        slot(&mut e, ACTOR_TABLE, VSLOT_IS_PROCESS_FLAG_0X22C, eax(1));
        double(&mut e, UPDATE_STEP_SETTING, st0(0.5));
        double(&mut e, SETTING_INT_VALUE, eax(1));
        double(&mut e, GLOBAL_WORD_ADDRESS, eax(SCRATCH_FLOAT));
        double(&mut e, ACTOR_GET_ESSENTIAL, eax(1));
        fn_00886360(&mut e, actor, 0.25);
        assert_eq!(e.get(actor, Actor::fCheckMyDeadBodyTimer), 4.5);
        assert_eq!(e.get(actor, Actor::fDeadBodyAlarm), 0.5);
    }

    #[test]
    fn update_tells_the_player_when_the_dead_alarm_goes_off() {
        let mut e = engine2();
        let actor = actor2(&mut e);
        let player = actor2(&mut e);
        set_player(&mut e, player);
        with_process(&mut e, actor);
        with_process(&mut e, player);
        e.set(actor, Actor::bProcessMe, true);
        e.set(actor, Actor::bDeadFlag, true);
        e.set(actor, Actor::fDeadBodyAlarm, 0.25);
        e.set(actor, Actor::pMyKiller, player.cast());
        slot(&mut e, ACTOR_TABLE, VSLOT_IS_PROCESS_FLAG_0X22C, eax(1));
        double(&mut e, UPDATE_STEP_SETTING, st0(0.5));
        double(&mut e, SETTING_INT_VALUE, eax(1));
        double(&mut e, GLOBAL_WORD_ADDRESS, eax(SCRATCH_FLOAT));
        double(&mut e, ACTOR_GET_ESSENTIAL, eax(1));
        e.set_global(DEAD_BODY_ALARM_RESET, 3.0f32);
        slot(&mut e, PROCESS_TABLE, PROCESS_VSLOT_0XFC, Ret::default());
        let position = e.mem.alloc(16);
        e.mem.set_f32(position, 1.0);
        slot(&mut e, ACTOR_TABLE, VSLOT_GET_POSITION_0X1F4, eax(position));
        e.call_log = Some(vec![]);
        fn_00886360(&mut e, actor, 0.25);
        let target = slot_target(PROCESS_TABLE, PROCESS_VSLOT_0XFC);
        let process = e.get(player, Actor::pCurrentProcess).addr();
        assert_eq!(
            args_of(&e, target),
            vec![process, player.addr(), 1.0f32.to_bits(), 0, 0, 1, 4, 0]
        );
        assert_eq!(e.get(actor, Actor::fDeadBodyAlarm), 3.0);
    }

    #[test]
    fn reset_ai_flag_is_returned() {
        let mut e = engine2();
        let actor = actor2(&mut e);
        assert!(!fn_00886b10(&mut e, actor));
        e.set(actor, Actor::bResetAI, true);
        assert!(fn_00886b10(&mut e, actor));
    }

    #[test]
    fn bip_transform_needs_a_process() {
        let mut e = engine2();
        let actor = actor2(&mut e);
        let vector = Ptr::new(e.mem.alloc(12));
        assert!(!fn_00886b30(&mut e, actor, vector));
    }

    #[test]
    fn bip_transform_rotates_the_vector_by_the_heading() {
        let mut e = engine2();
        let actor = actor2(&mut e);
        with_process(&mut e, actor);
        slot(&mut e, ACTOR_TABLE, VSLOT_PACKAGE_TEST_0X230, eax(0));
        slot(&mut e, ACTOR_TABLE, VSLOT_IS_PROCESS_FLAG_0X22C, eax(1));
        let vector = Ptr::new(e.mem.alloc(12));
        let result = e.mem.alloc(12);
        e.mem.set_f32(result, 9.0);
        e.mem.set_f32(result + 4, 8.0);
        e.mem.set_f32(result + 8, 7.0);
        let rotation = e.mem.alloc(16);
        e.mem.set_f32(rotation + 8, 0.5);
        double(&mut e, ACTOR_ROTATION_VECTOR, eax(rotation));
        double(&mut e, VECTOR_TIMES_MATRIX, eax(result));
        double(&mut e, BLEND_OBJECT_OF, eax(0x777));
        let controller = e.mem.alloc(0x500);
        double(&mut e, GET_CHAR_CONTROLLER, eax(controller));
        e.call_log = Some(vec![]);
        assert!(fn_00886b30(&mut e, actor, vector));
        assert_eq!(e.mem.f32(vector.addr() + 4), 8.0);
        assert_eq!(
            args_of(&e, SET_BIP_TRANSFORM),
            vec![0x777, vector.addr(), 0]
        );
        assert_eq!(
            args_of(&e, FLAG_SET_OR_CLEAR),
            vec![controller + 0x410, 0x1000, 1]
        );
        assert_eq!(args_of(&e, MATRIX_MAKE_ROTATION)[1], 0.5f32.to_bits());
    }

    #[test]
    fn controller_flag_setters_pass_their_masks() {
        let mut e = engine2();
        e.call_log = Some(vec![]);
        fn_00886c80(&mut e, Ptr::new(0x5000), true);
        assert_eq!(args_of(&e, FLAG_SET_OR_CLEAR), vec![0x5410, 0x1000, 1]);
        e.call_log = Some(vec![]);
        fn_008879d0(&mut e, Ptr::new(0x5000), false);
        assert_eq!(args_of(&e, FLAG_SET_OR_CLEAR), vec![0x5410, 0x40000, 0]);
    }

    #[test]
    fn global_word_is_returned() {
        let mut e = engine2();
        e.mem.set_u32(GLOBAL_VALUE_011C6260, 0x1357);
        assert_eq!(fn_00887a00(&mut e), 0x1357);
    }

    #[test]
    fn proxy_value_prefers_the_proxy_object() {
        let mut e = engine2();
        let object = Ptr::<()>::new(e.mem.alloc(0x20));
        double(&mut e, WORD_AT_0XC, eax(0x42));
        double(&mut e, PROXY_OF, eax(0x9000));
        assert_eq!(fn_00887a10(&mut e, object.cast()), 0x42);
    }

    #[test]
    fn proxy_value_falls_back_to_the_linked_word() {
        let mut e = engine2();
        let object = e.mem.alloc(0x20);
        assert_eq!(fn_00887a10(&mut e, Ptr::new(object)), 0);
        let target = e.mem.alloc(8);
        e.mem.set_u32(target, 0x5566);
        e.mem.set_u32(object + 0xc, target);
        assert_eq!(fn_00887a10(&mut e, Ptr::new(object)), 0x5566);
    }

    #[test]
    fn motor_flag_builds_the_value_on_the_stack() {
        let mut e = engine2();
        let object = e.mem.alloc(0x20);
        let target = e.mem.alloc(8);
        e.mem.set_u32(object + 0xc, target);
        e.register(BYTE_STORE, |e, a| {
            e.mem.set_u8(a[0], a[1] as u8);
            Ret::default()
        });
        double(&mut e, PROXY_OF, eax(0x9000));
        e.mem.set_u32(target, 0x5566);
        double(&mut e, WORD_AT_0XC, eax(0x4321));
        e.call_log = Some(vec![]);
        fn_00887a80(&mut e, Ptr::new(object), true);
        let call = args_of(&e, SET_MOTOR_ACTIVE);
        assert_eq!(call[0], 0x4321);
        assert_eq!(call[1], 0x9000);
        assert_eq!(call[2] & 0xff, 1);
    }

    #[test]
    fn float_setter_forwards_to_the_target() {
        let mut e = engine2();
        let object = e.mem.alloc(0x20);
        let target = e.mem.alloc(8);
        e.mem.set_u32(object + 0xc, target);
        e.mem.set_u32(target, 0x1111);
        e.call_log = Some(vec![]);
        fn_00887ac0(&mut e, Ptr::new(object), 2.0);
        assert_eq!(
            args_of(&e, SET_FLOAT_ON_TARGET),
            vec![0x1111, 2.0f32.to_bits()]
        );
    }

    #[test]
    fn proxy_forwarders_reach_their_targets() {
        let mut e = engine2();
        double(&mut e, PROXY_OF, eax(0x6000));
        e.call_log = Some(vec![]);
        fn_00887af0(&mut e, Ptr::new(0x5000), 77);
        assert_eq!(args_of(&e, SUBOBJECT_SET_VALUE), vec![0x6040, 77]);
        let proxy = e.mem.alloc(0x80);
        e.register(PROXY_OF, |_, _| eax(0x7000));
        fn_00887b20(&mut e, Ptr::new(proxy), 5);
        assert_eq!(args_of(&e, SUBOBJECT_SET_VALUE), vec![0x6040, 77]);
        fn_00887b70(&mut e, Ptr::new(proxy), 3);
        assert_eq!(e.mem.u8(proxy + 0x5c), 3);
        e.mem.map(0x7000, 0x100);
        fn_00887b40(&mut e, Ptr::new(0x5000), 9);
        assert_eq!(e.mem.u8(0x705c), 9);
    }

    #[test]
    fn muzzle_step_skips_flagged_references() {
        let mut e = engine2();
        let actor = actor2(&mut e);
        double(&mut e, REFERENCE_FLAG_TEST_0X200000, eax(1));
        e.call_log = Some(vec![]);
        fn_00887b90(&mut e, actor, 0.1, 0);
        assert_eq!(logged(&e).len(), 1);
    }

    #[test]
    fn muzzle_step_updates_the_flash_and_fades() {
        let mut e = engine2();
        let actor = actor2(&mut e);
        let other = actor2(&mut e);
        set_player(&mut e, other);
        with_process(&mut e, actor);
        slot(
            &mut e,
            PROCESS_TABLE,
            PROCESS_VSLOT_MUZZLE_FLASH,
            eax(0xf1a5),
        );
        double(&mut e, SETTING_INT_VALUE, eax(1));
        double(&mut e, PROCESS_GET_LEVEL, eax(0));
        e.call_log = Some(vec![]);
        fn_00887b90(&mut e, actor, 0.5, 0);
        assert_eq!(
            args_of(&e, MUZZLE_FLASH_UPDATE),
            vec![0xf1a5, 0.5f32.to_bits(), actor.addr()]
        );
        let process = e.get(actor, Actor::pCurrentProcess).addr();
        assert_eq!(
            args_of(&e, HIGH_PROCESS_FADE_UPDATE),
            vec![process, actor.addr()]
        );
    }

    #[test]
    fn reset_loaded_animations_ignores_the_player() {
        let mut e = engine2();
        let actor = actor2(&mut e);
        set_player(&mut e, actor);
        e.call_log = Some(vec![]);
        actor_reset_loaded_animations(&mut e, actor);
        assert_eq!(logged(&e).len(), 0);
    }

    #[test]
    fn reset_loaded_animations_can_be_queued() {
        let mut e = engine2();
        let actor = actor2(&mut e);
        let other = actor2(&mut e);
        set_player(&mut e, other);
        slot(&mut e, ACTOR_TABLE, VSLOT_GET_3D, eax(0x1000));
        double(&mut e, RESET_ANIMS_SHOULD_QUEUE, eax(1));
        double(&mut e, TASK_QUEUE_INTERFACE, eax(0x8888));
        e.call_log = Some(vec![]);
        actor_reset_loaded_animations(&mut e, actor);
        assert_eq!(
            args_of(&e, QUEUE_ACTOR_RESET_LOADED_ANIMS),
            vec![0x8888, actor.addr()]
        );
        assert!(!was_called(&e, SET_ANIMATION));
    }

    #[test]
    fn reset_loaded_animations_builds_a_new_animation() {
        let mut e = engine2();
        let actor = actor2(&mut e);
        let other = actor2(&mut e);
        set_player(&mut e, other);
        with_process(&mut e, actor);
        slot(&mut e, ACTOR_TABLE, VSLOT_GET_3D, eax(0x1000));
        double(&mut e, OPERATOR_NEW_FN, eax(0x2000));
        double(&mut e, ANIMATION_CONSTRUCTOR, eax(0x2000));
        double(&mut e, SET_ANIMATION, eax(0x2000));
        double(&mut e, BUILD_KF_FILE_LIST, eax(0x3000));
        double(&mut e, INIT_ANIMATION, eax(1));
        double(&mut e, PROCESS_GET_LEVEL, eax(2));
        e.mem.map(0x5000, 0x1000);
        e.mem.set_u32(0x5100 + 0xc, 0xbeef);
        let skeleton = e.mem.alloc(0x40);
        e.mem.set_u32(skeleton, OTHER_TABLE);
        slot(&mut e, ACTOR_TABLE, VSLOT_GET_3D, eax(skeleton));
        e.register(0x0041_81e0, |_, _| eax(0x5100));
        slot(&mut e, ACTOR_TABLE, VSLOT_IS_PROCESS_FLAG_0X22C, eax(0));
        e.set(actor, Actor::eLifeState, 2);
        e.call_log = Some(vec![]);
        actor_reset_loaded_animations(&mut e, actor);
        let log = logged(&e);
        assert_eq!(
            log.iter().filter(|entry| entry.0 == SET_ANIMATION).count(),
            2
        );
        assert_eq!(args_of(&e, OPERATOR_NEW_FN), vec![ANIMATION_SIZE]);
        let init = args_of(&e, INIT_ANIMATION);
        assert_eq!(init[0], 0x2000);
        assert_eq!(init[1], 0x3000);
        assert_eq!(init[3], actor.addr());
        assert_eq!(init[4], 1);
    }

    #[test]
    fn animation_step_ignores_an_actor_without_a_3d_object() {
        let mut e = engine2();
        let actor = actor2(&mut e);
        e.call_log = Some(vec![]);
        fn_00888070(&mut e, actor, 0.1);
        assert!(!was_called(&e, PICK_ANIMATIONS));
    }

    #[test]
    fn animation_step_picks_the_animations() {
        let mut e = engine2();
        let actor = actor2(&mut e);
        let other = actor2(&mut e);
        set_player(&mut e, other);
        with_process(&mut e, actor);
        slot(&mut e, ACTOR_TABLE, VSLOT_GET_3D, eax(0x1000));
        double(&mut e, PACKAGE_TYPE, eax(1));
        double(&mut e, SETTING_INT_VALUE, eax(1));
        double(&mut e, UPDATE_STEP_SETTING, st0(0.25));
        e.set_global(0x011f_2ac4, 0.0f32);
        e.set_global(0x011f_2ad8, 6.0f32);
        double(&mut e, SETTING_VALUE_ADDRESS, eax(0x011f_2ad8));
        slot(&mut e, ACTOR_TABLE, VSLOT_ANIMATION_FLAG_0X21C, eax(0));
        e.call_log = Some(vec![]);
        fn_00888070(&mut e, actor, 0.1);
        assert_eq!(
            args_of(&e, PICK_ANIMATIONS),
            vec![actor.addr(), 1.0f32.to_bits(), 6.0f32.to_bits()]
        );
        let process = e.get(actor, Actor::pCurrentProcess).addr();
        assert_eq!(args_of(&e, ACTOR_PRE_ANIMATION_STEP), vec![actor.addr()]);
        let _ = process;
    }

    #[test]
    fn animation_step_finishes_the_death_animation() {
        let mut e = engine2();
        let actor = actor2(&mut e);
        let other = actor2(&mut e);
        set_player(&mut e, other);
        with_process(&mut e, actor);
        slot(&mut e, ACTOR_TABLE, VSLOT_GET_3D, eax(0x1000));
        slot(&mut e, ACTOR_TABLE, VSLOT_GET_ANIMATION, eax(0x4000));
        double(&mut e, PACKAGE_TYPE, eax(1));
        double(&mut e, SETTING_INT_VALUE, eax(1));
        double(&mut e, BLEND_OBJECT_OF, eax(0x777));
        double(&mut e, ANIMATION_GROUP_LOADED, eax(1));
        double(&mut e, SAVE_FORM_BUFFER_GET_FORM, eax(0x600));
        double(&mut e, FORM_TYPE, eax(0x2b));
        double(&mut e, GLOBAL_WORD_ADDRESS, eax(SCRATCH_FLOAT));
        slot(&mut e, ACTOR_TABLE, VSLOT_ACTOR_FORM_TEST_0X38C, eax(0));
        e.set(actor, Actor::bDeadFlag, true);
        e.call_log = Some(vec![]);
        fn_00888070(&mut e, actor, 0.1);
        assert_eq!(args_of(&e, ACTOR_SET_LIFE_STATE), vec![actor.addr(), 1]);
        assert_eq!(
            args_of(&e, ANIMATION_FORCE_SECTION),
            vec![0x4000, 0x14, 0xe0, 0xffff_ffff, 0, 0xffff_ffff]
        );
        assert!(!e.get(actor, Actor::bDeadFlag));
    }

    /// 00888211..00888230: the count is pushed for `004d50e0` (`RET 4`), and
    /// `0049bdc0` (`RET 4`) gets only the fraction `004d50e0` left in ST0.
    #[test]
    fn animation_step_hands_0049bdc0_only_the_fraction() {
        let mut e = engine2();
        let actor = actor2(&mut e);
        let other = actor2(&mut e);
        set_player(&mut e, other);
        with_process(&mut e, actor);
        slot(&mut e, ACTOR_TABLE, VSLOT_GET_3D, eax(0x1000));
        slot(&mut e, ACTOR_TABLE, VSLOT_GET_ANIMATION, eax(0x4000));
        slot(&mut e, ACTOR_TABLE, VSLOT_ANIMATION_FLAG_0X21C, eax(1));
        slot(&mut e, PROCESS_TABLE, PROCESS_VSLOT_0X748, eax(0x5000));
        double(&mut e, PACKAGE_TYPE, eax(1));
        double(&mut e, ANIMATION_GROUP_LOADED, eax(1));
        double(&mut e, ANIMATION_TIME, st0(0.1));
        double(&mut e, LIST_FRACTION_AT, st0(0.75));
        double(&mut e, GLOBAL_WORD_ADDRESS, eax(SCRATCH_FLOAT));
        e.set_global(MILLISECONDS_PER_SECOND, 1000.0f64);
        e.call_log = Some(vec![]);
        fn_00888070(&mut e, actor, 0.1);
        assert_eq!(args_of(&e, LIST_FRACTION_AT), vec![0x5000, 100 / 0x21]);
        assert_eq!(
            args_of(&e, ANIMATION_SET_FRACTION),
            vec![0x4000, 0.75f32.to_bits()]
        );
    }

    #[test]
    fn movement_update_needs_an_animation() {
        let mut e = engine2();
        let actor = actor2(&mut e);
        e.call_log = Some(vec![]);
        fn_008885a0(&mut e, actor);
        assert!(!was_called(&e, ANIMATION_UPDATE_MOVEMENT_NO_WORLD_UPDATE));
        slot(&mut e, ACTOR_TABLE, VSLOT_GET_ANIMATION, eax(0x4000));
        e.call_log = Some(vec![]);
        fn_008885a0(&mut e, actor);
        assert_eq!(
            args_of(&e, ANIMATION_UPDATE_MOVEMENT_NO_WORLD_UPDATE),
            vec![0x4000, actor.addr()]
        );
    }

    #[test]
    fn animation_movement_counts_the_player_while_it_runs() {
        let mut e = engine2();
        let actor = actor2(&mut e);
        set_player(&mut e, actor);
        e.mem.set_u8(PLAYER_UPDATE_NESTING, 4);
        e.call_log = Some(vec![]);
        actor_update_animation_movement(&mut e, actor, Ptr::new(0), 0.0);
        assert_eq!(e.mem.u8(PLAYER_UPDATE_NESTING), 4);
        assert_eq!(logged(&e).len(), 2);
    }

    #[test]
    fn animation_movement_updates_the_scene_graph_and_scales() {
        let mut e = engine2();
        let actor = actor2(&mut e);
        let other = actor2(&mut e);
        set_player(&mut e, other);
        double(&mut e, SETTING_INT_VALUE, eax(1));
        double(&mut e, UPDATE_STEP_SETTING, st0(0.5));
        double(&mut e, GLOBAL_WORD_ADDRESS, eax(SCRATCH_FLOAT));
        slot(&mut e, ACTOR_TABLE, VSLOT_KIND_0X214, eax(3));
        double(&mut e, PLAYER_GET_ANIMATION, eax(0x7777));
        double(&mut e, GET_SCALE, st0(2.0));
        e.set_global(DOUBLE_ONE, 1.0f64);
        e.register(ANIMATION_SCALE_VECTOR, |e, a| {
            e.mem.set_f32(a[1] + 8, 4.0);
            Ret::default()
        });
        let position = e.mem.alloc(16);
        e.mem.set_f32(position + 8, 10.0);
        slot(&mut e, ACTOR_TABLE, VSLOT_GET_POSITION_0X1F4, eax(position));
        e.call_log = Some(vec![]);
        actor_update_animation_movement(&mut e, actor, Ptr::new(0x4000), 0.0);
        assert!(was_called(&e, ANIMATION_UPDATE_SCENE_GRAPH_NO_CONTROLLER));
        // The offset 4 - 4 * 2 = -4 lowers the position by that much.
        assert_eq!(
            args_of(&e, SET_LOCATION_ON_REFERENCE_Z),
            vec![actor.addr(), 14.0f32.to_bits()]
        );
    }

    #[test]
    fn small_accessors_of_the_ragdoll_controller() {
        let mut e = engine2();
        let controller = Ptr::<()>::new(e.mem.alloc(0x300));
        e.mem.set_u8(GLOBAL_FLAG_012677A3, 7);
        assert_eq!(fn_00888960(&mut e), 7);
        e.mem.set_u8(controller.addr() + 0xb3, 1);
        assert!(fn_00888a50(&mut e, controller));
        e.mem.set_f32(controller.addr() + 0x19c, 1.5);
        e.mem.set_f32(controller.addr() + 0x1a0, 2.5);
        let out = e.mem.alloc(8);
        fn_00888a70(&mut e, controller, Ptr::new(out), Ptr::new(out + 4));
        assert_eq!(e.mem.f32(out), 1.5);
        assert_eq!(e.mem.f32(out + 4), 2.5);
        let face = Ptr::<()>::new(e.mem.alloc(0x200));
        fn_00888a20(&mut e, face, 3.0, 4.0);
        assert_eq!(e.mem.f32(face.addr() + 0x150), 3.0);
        assert_eq!(e.mem.f32(face.addr() + 0x154), 4.0);
        e.mem.set_f32(controller.addr() + 0x23c, 9.0);
        fn_00888aa0(&mut e, controller);
        assert_eq!(e.mem.f32(controller.addr() + 0x23c), 0.0);
    }

    #[test]
    fn ragdoll_face_step_copies_the_controller_floats() {
        let mut e = engine2();
        let actor = actor2(&mut e);
        let controller = e.mem.alloc(0x300);
        e.set(actor, Actor::pRagdollController, Ptr::new(controller));
        e.mem.set_u8(controller + 0xb3, 1);
        e.mem.set_f32(controller + 0x19c, 1.5);
        e.mem.set_f32(controller + 0x1a0, 2.5);
        let face = e.mem.alloc(0x200);
        double(&mut e, GET_FACE_ANIMATION_DATA, eax(face));
        double(&mut e, RAGDOLL_TEST_0X4955A0, eax(0));
        e.mem.set_f32(controller + 0x23c, 9.0);
        e.call_log = Some(vec![]);
        fn_00888970(&mut e, actor, 1);
        assert_eq!(args_of(&e, DO_RAGDOLL_ANIM), vec![controller, 1]);
        assert_eq!(e.mem.f32(face + 0x150), 1.5);
        assert_eq!(e.mem.f32(face + 0x154), 2.5);
        assert_eq!(e.mem.f32(controller + 0x23c), 0.0);
        e.mem.set_u8(controller + 0xb3, 0);
        fn_00888970(&mut e, actor, 1);
        assert_eq!(e.mem.f32(face + 0x150), 0.0);
    }

    #[test]
    fn process_forwarders_use_their_slots() {
        let mut e = engine2();
        let actor = actor2(&mut e);
        with_process(&mut e, actor);
        slot(&mut e, PROCESS_TABLE, PROCESS_VSLOT_0X2F0, Ret::default());
        slot(&mut e, PROCESS_TABLE, PROCESS_VSLOT_0X2F4, Ret::default());
        slot(&mut e, PROCESS_TABLE, PROCESS_VSLOT_0X2F8, Ret::default());
        e.call_log = Some(vec![]);
        fn_00888ac0(&mut e, actor, 1.5);
        fn_00888af0(&mut e, actor);
        fn_00888b20(&mut e, actor);
        let log = logged(&e);
        let process = e.get(actor, Actor::pCurrentProcess).addr();
        assert_eq!(log.len(), 3);
        assert_eq!(log[0].1, vec![process, actor.addr(), 1.5f32.to_bits()]);
        assert_eq!(log[1].1, vec![process]);
        assert_eq!(log[2].1, vec![process]);
    }

    #[test]
    fn fn_00888af0_answers_the_process_st0() {
        let mut e = engine2();
        let actor = actor2(&mut e);
        with_process(&mut e, actor);
        let timer = Ret {
            st0: 2.25,
            ..Ret::default()
        };
        slot(&mut e, PROCESS_TABLE, PROCESS_VSLOT_0X2F4, timer);
        assert_eq!(fn_00888af0(&mut e, actor), 2.25);
    }

    #[test]
    fn update_stops_at_the_cell_checks() {
        let mut e = engine2();
        let actor = actor2(&mut e);
        let other = actor2(&mut e);
        set_player(&mut e, other);
        with_process(&mut e, actor);
        slot(&mut e, PROCESS_TABLE, PROCESS_VSLOT_0X4BC, eax(5));
        e.call_log = Some(vec![]);
        actor_update(&mut e, actor, 0.1);
        assert_eq!(e.get(actor, Actor::cCurrentSitSleepState), 5);
        assert!(!was_called(&e, ACTOR_TIMED_UPDATE));
    }

    #[test]
    fn update_wakes_the_process_for_a_long_frame() {
        let mut e = engine2();
        let actor = actor2(&mut e);
        let other = actor2(&mut e);
        set_player(&mut e, other);
        with_process(&mut e, actor);
        slot(&mut e, ACTOR_TABLE, VSLOT_GET_3D, eax(0x1000));
        double(&mut e, GET_PARENT_CELL, eax(0x5000));
        double(&mut e, CELL_TEST_0X450FF0, eax(1));
        e.set_global(UPDATE_TIME_LIMIT, 900.0f64);
        slot(&mut e, PROCESS_TABLE, PROCESS_VSLOT_0X5F8, Ret::default());
        e.call_log = Some(vec![]);
        actor_update(&mut e, actor, 1000.0);
        let process = e.get(actor, Actor::pCurrentProcess).addr();
        let wake = slot_target(PROCESS_TABLE, PROCESS_VSLOT_0X5F8);
        assert_eq!(args_of(&e, wake), vec![process, 1]);
        assert!(!was_called(&e, ACTOR_TIMED_UPDATE));
    }

    fn update_setup(e: &mut Engine) -> Ptr<Actor> {
        let actor = actor2(e);
        let other = actor2(e);
        set_player(e, other);
        with_process(e, actor);
        slot(e, ACTOR_TABLE, VSLOT_GET_3D, eax(0x1000));
        double(e, GET_PARENT_CELL, eax(0x5000));
        double(e, CELL_TEST_0X450FF0, eax(1));
        e.set_global(UPDATE_TIME_LIMIT, 900.0f64);
        double(e, SETTING_INT_VALUE, eax(1));
        double(e, UPDATE_STEP_SETTING, st0(0.5));
        double(e, GLOBAL_WORD_ADDRESS, eax(SCRATCH_FLOAT));
        double(e, ACTOR_GET_LIFE_STATE, eax(1));
        e.set_global(DOUBLE_ONE, 1.0f64);
        let position = e.mem.alloc(16);
        double(e, POSITION_VECTOR_OF, eax(position));
        double(e, VECTOR_PLUS_VECTOR, eax(position));
        slot(e, ACTOR_TABLE, VSLOT_GET_POSITION_0X1F4, eax(position));
        actor
    }

    #[test]
    fn update_runs_the_whole_frame_for_a_plain_actor() {
        let mut e = engine2();
        let actor = update_setup(&mut e);
        let player = e.mem.u32(PLAYER_CHARACTER);
        with_process(&mut e, Ptr::new(player));
        e.mem.set_f32(SCRATCH_FLOAT, 4.0);
        e.call_log = Some(vec![]);
        actor_update(&mut e, actor, 0.1);
        assert_eq!(
            args_of(&e, ACTOR_TIMED_UPDATE),
            vec![actor.addr(), 0.1f32.to_bits()]
        );
        assert!(was_called(&e, ACTOR_END_OF_UPDATE));
        let log = logged(&e);
        assert_eq!(
            log.iter().filter(|entry| entry.0 == EMPTY_FUNCTION).count(),
            3
        );
    }

    #[test]
    fn update_scales_the_animation_speed_by_the_setting() {
        let mut e = engine2();
        let actor = update_setup(&mut e);
        let player = e.mem.u32(PLAYER_CHARACTER);
        with_process(&mut e, Ptr::new(player));
        e.mem.set_f32(SCRATCH_FLOAT, 4.0);
        slot(&mut e, OTHER_TABLE, 8, eax(3));
        slot(&mut e, ACTOR_TABLE, VSLOT_GET_ANIMATION, eax(0x4000));
        e.call_log = Some(vec![]);
        actor_update(&mut e, actor, 0.1);
        assert_eq!(
            args_of(&e, ANIMATION_SET_SPEED_SCALE),
            vec![0x4000, 0.25f32.to_bits()]
        );
    }

    #[test]
    fn update_drowns_the_actor_when_the_breath_is_spent() {
        let mut e = engine2();
        let actor = update_setup(&mut e);
        let player = e.mem.u32(PLAYER_CHARACTER);
        with_process(&mut e, Ptr::new(player));
        let controller = e.mem.alloc(0x600);
        double(&mut e, GET_CHAR_CONTROLLER, eax(controller));
        double(&mut e, IS_IN_MENU_MODE, eax(0));
        double(&mut e, ACTOR_GET_LIFE_STATE, eax(1));
        // The actor is in deep water.
        e.set_global(WATER_THRESHOLD_A, 1.0f64);
        e.set_global(WATER_THRESHOLD_B, 2.0f64);
        e.set_global(WATER_THRESHOLD_C, 3.0f64);
        slot(&mut e, ACTOR_TABLE, VSLOT_ACTOR_FLAG_0X360, eax(0));
        slot(&mut e, PROCESS_TABLE, PROCESS_VSLOT_0X300, st0(0.1));
        slot(&mut e, PROCESS_TABLE, PROCESS_VSLOT_0X2FC, Ret::default());
        double(&mut e, ACTOR_TEST_0X87F5C0, eax(0));
        let position = e.mem.alloc(16);
        slot(&mut e, ACTOR_TABLE, VSLOT_GET_POSITION_0X1F4, eax(position));
        double(&mut e, POSITION_VECTOR_OF, eax(position));
        double(&mut e, VECTOR_PLUS_VECTOR, eax(position));
        e.call_log = Some(vec![]);
        actor_update(&mut e, actor, 0.1);
        let target = slot_target(PROCESS_TABLE, PROCESS_VSLOT_0X2FC);
        let process = e.get(actor, Actor::pCurrentProcess).addr();
        assert_eq!(args_of(&e, target), vec![process, 0]);
        assert!(was_called(&e, ACTOR_TIMED_UPDATE));
    }

    /// `Actor::Update` (`00888b50`) follows `world::frame::ai_stage`'s model
    /// (docs/FRAME_SKELETON.md, PR 6): a person without and with a
    /// character controller (in water), a long step, no 3D, the player in
    /// first person.
    #[test]
    fn update_follows_the_frame_model() {
        use world::frame::ai_stage::{follows, function, AiState, Callee};
        let model = function(0x0088_8b50).expect("modelled");
        // The virtual calls the model names, by the slot they go through.
        let slots = [
            (ACTOR_TABLE, VSLOT_UPDATE_0X44C),
            (PROCESS_TABLE, PROCESS_VSLOT_0X5F8),
            (PROCESS_TABLE, PROCESS_VSLOT_0X704),
            (PROCESS_TABLE, PROCESS_VSLOT_0X428),
        ];
        let callees = |e: &Engine| -> Vec<Callee> {
            logged(e)
                .into_iter()
                .map(|(a, _)| {
                    slots
                        .iter()
                        .find(|&&(t, s)| slot_target(t, s) == a)
                        .map_or(Callee::Direct(a), |&(_, s)| Callee::Virtual(s))
                })
                .collect()
        };
        // Called as Rust functions, so not in the log: the power attack flag,
        // the 3D's placement, the flag byte and the water's depth; and the
        // placement's own `00440460` (the first-person player's is checked
        // below).
        let ignore = [
            Callee::Direct(0x0088_b070),
            Callee::Direct(0x0088_b150),
            Callee::Direct(0x0044_0460),
            Callee::Direct(0x0088_b020),
            Callee::Direct(0x0088_5560),
        ];
        let prepare = |e: &mut Engine| {
            for (t, s) in slots {
                slot(e, t, s, Ret::default());
            }
        };
        let npc = AiState {
            actor_controller: false,
            ..AiState::default()
        };
        // A person.
        let mut e = engine2();
        let actor = update_setup(&mut e);
        let player = e.mem.u32(PLAYER_CHARACTER);
        with_process(&mut e, Ptr::new(player));
        prepare(&mut e);
        e.call_log = Some(vec![]);
        actor_update(&mut e, actor, 0.1);
        follows(model, &npc, &callees(&e), &ignore).expect("a person");
        // A long step only wakes the process.
        let long = AiState {
            actor_long_step: true,
            ..npc
        };
        let mut e = engine2();
        let actor = update_setup(&mut e);
        prepare(&mut e);
        e.call_log = Some(vec![]);
        actor_update(&mut e, actor, 1000.0);
        follows(model, &long, &callees(&e), &ignore).expect("a long step");
        // No 3D: nothing but the reset.
        let not_ready = AiState {
            actor_ready: false,
            ..npc
        };
        let mut e = engine2();
        let actor = actor2(&mut e);
        let other = actor2(&mut e);
        set_player(&mut e, other);
        with_process(&mut e, actor);
        prepare(&mut e);
        e.call_log = Some(vec![]);
        actor_update(&mut e, actor, 0.1);
        follows(model, &not_ready, &callees(&e), &ignore).expect("no 3D");
        // The player in first person (+0x64a clear): its 3D put at its
        // position.
        let player_state = AiState {
            actor_is_player: true,
            ..npc
        };
        let mut e = engine2();
        let actor = update_setup(&mut e);
        set_player(&mut e, actor);
        prepare(&mut e);
        double(&mut e, PLAYER_STATE_TEST, eax(0));
        e.call_log = Some(vec![]);
        actor_update(&mut e, actor, 0.1);
        follows(model, &player_state, &callees(&e), &ignore).expect("the player");
        assert!(was_called(&e, SET_WORLD_POSITION));
        // A person with a controller, in deep water.
        let swimmer = AiState::default();
        let mut e = engine2();
        let actor = update_setup(&mut e);
        let player = e.mem.u32(PLAYER_CHARACTER);
        with_process(&mut e, Ptr::new(player));
        prepare(&mut e);
        let controller = e.mem.alloc(0x600);
        double(&mut e, GET_CHAR_CONTROLLER, eax(controller));
        double(&mut e, IS_IN_MENU_MODE, eax(0));
        e.set_global(WATER_THRESHOLD_A, 1.0f64);
        e.set_global(WATER_THRESHOLD_B, 2.0f64);
        e.set_global(WATER_THRESHOLD_C, 3.0f64);
        slot(&mut e, ACTOR_TABLE, VSLOT_ACTOR_FLAG_0X360, eax(0));
        slot(&mut e, PROCESS_TABLE, PROCESS_VSLOT_0X300, st0(0.1));
        slot(&mut e, PROCESS_TABLE, PROCESS_VSLOT_0X2FC, Ret::default());
        double(&mut e, ACTOR_TEST_0X87F5C0, eax(0));
        e.call_log = Some(vec![]);
        actor_update(&mut e, actor, 0.1);
        follows(model, &swimmer, &callees(&e), &ignore).expect("a swimmer");
        assert!(was_called(&e, CONTROLLER_SET_TARGET));
    }

    /// `004e75d0` as the exe has it: copies the nine words at `this + 0x44` to `out` and
    /// answers `out`.
    fn sound_item_copies_nine_words(e: &mut Engine) {
        e.register(WEAPON_SOUND_ITEM, |e, a| {
            for i in 0..9 {
                let word = e.mem.u32(a[0] + 0x44 + 4 * i);
                e.mem.set_u32(a[1] + 4 * i, word);
            }
            eax(a[1])
        });
    }

    /// A sound whose nine item words are `0x100`, `0x101`, ...
    fn sound_with_item(e: &mut Engine) -> u32 {
        let sound = e.mem.alloc(0x68);
        for i in 0..9 {
            e.mem.set_u32(sound + 0x44 + 4 * i, 0x100 + i);
        }
        sound
    }

    /// The frame `Actor::Update` sections see, for `this`.
    fn update_frame(this: Ptr<Actor>, player: u32, weapon: u32, sound: u32) -> UpdateFrame {
        UpdateFrame {
            this,
            delta: 0.1,
            player,
            skeleton: 0x1000,
            face: 0,
            animation: 0,
            in_combat: false,
            weapon,
            process: 0,
            sound,
            controller: 0,
            player_flag: false,
            cell: 0,
        }
    }

    /// The creature "awake" sound's item is the frame's 0x24-byte local: `004e75d0` copies all
    /// nine words into it and the word at +4 is read while it is there. With the freed-block
    /// check a 0x10-byte block faults on the copy, and one freed before the read gives the
    /// fill word.
    #[test]
    fn creature_sound_reads_the_flags_word_from_the_whole_item() {
        let mut e = engine2();
        e.mem.enable_freed_check(true);
        let actor = actor2(&mut e);
        slot(&mut e, ACTOR_TABLE, VSLOT_ANIMATION_FLAG_0X21C, eax(1));
        slot(&mut e, ACTOR_TABLE, VSLOT_GET_3D, eax(0x1000));
        double(&mut e, SAVE_FORM_BUFFER_GET_FORM, eax(0x4400));
        let sounds = sound_with_item(&mut e);
        double(&mut e, PICK_CREATURE_SOUND, eax(sounds));
        sound_item_copies_nine_words(&mut e);
        e.call_log = Some(vec![]);
        creature_sound_step(&mut e, &update_frame(actor, 0, 0, 0));
        assert_eq!(args_of(&e, SOUND_FLAGS_OF), vec![0x101]);
        assert!(was_called(&e, AUDIO_GET_SOUND_HANDLE_BY_FILENAME));
    }

    /// The weapon attack sound's item (0x24 bytes) and handle (0xc bytes) are separate locals:
    /// `004e75d0` copies the whole item without reaching the handle or past the frame. With
    /// the freed-block check a frame too small for both faults on the copy.
    #[test]
    fn weapon_attack_sound_keeps_the_handle_past_the_item() {
        let mut e = engine2();
        e.mem.enable_freed_check(true);
        let actor = actor2(&mut e);
        let player = actor2(&mut e);
        let sound_object = sound_with_item(&mut e);
        double(&mut e, WEAPON_OBJECT_OF, eax(sound_object));
        sound_item_copies_nine_words(&mut e);
        let handle = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        start_weapon_attack_sound(&mut e, &update_frame(actor, player.addr(), 0x7700, handle));
        assert_eq!(args_of(&e, SOUND_FLAGS_OF), vec![0x101]);
        let item = args_of(&e, WEAPON_SOUND_ITEM)[1];
        let handle_scratch = args_of(&e, AUDIO_GET_SOUND_HANDLE_BY_FILENAME)[1];
        assert!(
            handle_scratch >= item + 0x24,
            "{handle_scratch:08x} {item:08x}"
        );
    }

    #[test]
    fn character_update_clears_the_dead_flag_and_kicks_the_ragdoll() {
        let mut e = engine2();
        let actor = actor2(&mut e);
        let player = actor2(&mut e);
        set_player(&mut e, player);
        with_process(&mut e, actor);
        e.set(actor, Actor::bDeadFlag, true);
        double(&mut e, BLEND_OBJECT_OF, eax(0x777));
        double(&mut e, SAVE_FORM_BUFFER_GET_FORM, eax(0x600));
        double(&mut e, FORM_TYPE, eax(0x2b));
        double(&mut e, GET_CURRENT_PROCESS_TYPE, eax(1));
        slot(&mut e, ACTOR_TABLE, VSLOT_ACTOR_FORM_TEST_0X38C, eax(1));
        let rotated = e.mem.alloc(12);
        double(&mut e, MATRIX_TIMES_VECTOR, eax(rotated));
        slot(&mut e, ACTOR_TABLE, VSLOT_GET_POSITION_0X1F4, eax(rotated));
        let cell = 0x5000;
        double(&mut e, GET_PARENT_CELL, eax(cell));
        double(&mut e, GET_FLAG_WORD, eax(0x800));
        set_player(&mut e, actor);
        e.call_log = Some(vec![]);
        fn_00886cb0(&mut e, actor, 0.1, 0);
        assert!(e.get(actor, Actor::bSwimming));
        assert_eq!(args_of(&e, ACTOR_SET_LIFE_STATE), vec![actor.addr(), 1]);
        let knock = args_of(&e, DO_KNOCK_DOWN);
        assert_eq!(knock[0], 0x777);
        assert_eq!(knock[2], 1);
        assert!(!e.get(actor, Actor::bDeadFlag));
    }

    #[test]
    fn character_update_scales_the_step_for_the_vats_target() {
        let mut e = engine2();
        let actor = actor2(&mut e);
        let other = actor2(&mut e);
        set_player(&mut e, other);
        let mover = e.mem.alloc(0x40);
        e.mem.set_u32(mover, OTHER_TABLE);
        e.set(actor, Actor::pActorMover, Ptr::new(mover));
        double(&mut e, GET_CURRENT_PROCESS_TYPE, eax(0));
        double(&mut e, SETTING_INT_VALUE, eax(4));
        e.mem.set_u32(VATS_TARGET_ACTOR, actor.addr());
        double(&mut e, UPDATE_STEP_SETTING, st0(0.5));
        double(&mut e, VATS_TARGET_UPDATE_MULT, st0(0.25));
        double(&mut e, GET_FLAG_WORD, eax(0));
        double(&mut e, GET_PARENT_CELL, eax(0x5000));
        double(&mut e, CELL_TEST_0X450FF0, eax(1));
        slot(&mut e, ACTOR_TABLE, VSLOT_0X250, Ret::default());
        e.call_log = Some(vec![]);
        fn_00886cb0(&mut e, actor, 0.0, 0);
        let call = args_of(&e, slot_target(ACTOR_TABLE, VSLOT_0X250));
        assert_eq!(call[0], actor.addr());
        assert_eq!(call[1], 0.125f32.to_bits());
        assert_eq!(call[3], 0);
    }

    #[test]
    fn character_update_resyncs_the_collision_of_a_turret() {
        let mut e = engine2();
        let actor = actor2(&mut e);
        set_player(&mut e, actor);
        slot(&mut e, ACTOR_TABLE, VSLOT_GET_3D, eax(0x1000));
        e.mem.set_u8(actor.addr() + 0x1b0, 1);
        double(&mut e, ACTOR_TURRET_BEHAVIOR, eax(1));
        double(&mut e, GET_FLAG_WORD, eax(0));
        double(&mut e, FIND_COLLISION_OBJECT, eax(0x9100));
        double(&mut e, FIND_ENTRY, eax(0x9200));
        double(&mut e, COLLISION_RIGID_BODY, eax(0x9300));
        double(&mut e, BODY_LIST_0X7D6BB0, eax(0));
        double(&mut e, WORLD_TEST_0XC8CCE0, eax(0));
        double(&mut e, BODY_FUNCTION_0X621480, eax(0x9400));
        e.mem.map(0x9000, 0x1000);
        e.mem.set_u32(0x9300, OTHER_TABLE);
        e.call_log = Some(vec![]);
        fn_00886cb0(&mut e, actor, 0.1, 0);
        assert_eq!(args_of(&e, SET_MOTION), vec![0x9100, 1, 1, 1, 1]);
        assert_eq!(args_of(&e, COLLISION_SYNCHRONIZE), vec![0x9100, 2]);
    }

    #[test]
    fn second_collision_pass_hands_the_handle_to_the_motor_and_float_setters() {
        let mut e = engine2();
        let actor = actor2(&mut e);
        e.mem.set_f32(actor.addr() + 0x24, 2.0);
        double(&mut e, FIND_COLLISION_OBJECT, eax(0x9100));
        double(&mut e, FIND_ENTRY, eax(0x9200));
        double(&mut e, COLLISION_RIGID_BODY, eax(0x9300));
        double(&mut e, BODY_LIST_0X7D6BB0, eax(0x9500));
        double(&mut e, WORLD_TEST_0XC8CCE0, eax(1));
        // The list's node answers a handle whose `00887a10` object (no
        // proxy: the word the +0xc link points to) is `target`.
        let target = 0x5566;
        let link = e.mem.alloc(8);
        e.mem.set_u32(link, target);
        let handle = e.mem.alloc(0x20);
        e.mem.set_u32(handle + 0xc, link);
        let node = e.mem.alloc(0x10);
        e.mem.set_u32(node, OTHER_TABLE);
        slot(&mut e, OTHER_TABLE, NODE_VSLOT_0X44, eax(handle));
        double(&mut e, WORD_AT_START, eax(node));
        double(&mut e, HANDLE_FLOAT_0X9A1260, st0(3.0));
        double(&mut e, HANDLE_FLOAT_0X644A50, st0(1.0));
        e.register(CLAMP_FLOAT, |e, a| {
            e.mem.set_f32(a[0], 1.5);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        resync_collision(&mut e, actor, 0x1000, 7, true);
        // The limits come from the `00887a10` object (ECX = [EBP-0x1a8]).
        assert_eq!(args_of(&e, HANDLE_FLOAT_0X9A1260), vec![target]);
        assert_eq!(args_of(&e, HANDLE_FLOAT_0X644A50), vec![target]);
        let clamp = args_of(&e, CLAMP_FLOAT);
        assert_eq!(clamp[1], 1.0f32.to_bits());
        assert_eq!(clamp[2], 3.0f32.to_bits());
        // 0088791d / 00887932: `00887a80` and `00887ac0` get the handle
        // ([EBP-0x1a4]) and resolve it themselves, so every `004ae750`
        // lookup is of the handle.
        let proxies: Vec<Vec<u32>> = logged(&e)
            .into_iter()
            .filter(|entry| entry.0 == PROXY_OF)
            .map(|entry| entry.1)
            .collect();
        assert_eq!(proxies, vec![vec![handle]; 4]);
        let motor = args_of(&e, SET_MOTOR_ACTIVE);
        assert_eq!(motor[0], target);
        assert_eq!(motor[1], 0);
        assert_eq!(
            args_of(&e, SET_FLOAT_ON_TARGET),
            vec![target, 1.5f32.to_bits()]
        );
        assert_eq!(args_of(&e, COLLISION_SYNCHRONIZE), vec![0x9100, 2]);
    }

    #[test]
    fn small_setters_store_their_values() {
        let mut e = engine2();
        fn_0088b020(&mut e, 5);
        assert_eq!(e.mem.u8(GLOBAL_FLAG_011F3E60), 5);
        let object = e.mem.alloc(0x600);
        fn_0088b050(&mut e, Ptr::new(object), 3);
        assert_eq!(e.mem.u8(object + 0x15d), 3);
        let flag = e.mem.alloc(4);
        e.mem.set_u8(flag, 1);
        e.mem.set_u32(object + 0x6c, 9);
        fn_0088b070(&mut e, Ptr::new(object), Ptr::new(flag));
        assert_eq!(e.mem.u8(object + 0x69), 1);
        assert_eq!(e.mem.u32(object + 0x6c), 9);
        e.mem.set_u8(flag, 0);
        fn_0088b070(&mut e, Ptr::new(object), Ptr::new(flag));
        assert_eq!(e.mem.u8(object + 0x69), 0);
        assert_eq!(e.mem.u32(object + 0x6c), 0);
        fn_0088b110(&mut e, Ptr::new(object), 2.5);
        assert_eq!(e.mem.f32(object + 0x560), 2.5);
        assert_eq!(fn_0088b130(&mut e, Ptr::new(object)), object + 0x570);
    }

    #[test]
    fn position_update_does_nothing_without_a_3d_object() {
        let mut e = engine2();
        let actor = actor2(&mut e);
        e.call_log = Some(vec![]);
        actor_update_actor_3d_position(&mut e, actor);
        assert_eq!(logged(&e).len(), 1);
    }

    #[test]
    fn position_update_places_the_object_and_refreshes_the_lighting() {
        let mut e = engine2();
        let actor = actor2(&mut e);
        let other = actor2(&mut e);
        set_player(&mut e, other);
        slot(&mut e, ACTOR_TABLE, VSLOT_GET_3D, eax(0x1000));
        slot(&mut e, ACTOR_TABLE, VSLOT_GET_HEADING_0X2BC, st0(0.75));
        let position = e.mem.alloc(16);
        double(&mut e, POSITION_VECTOR_OF, eax(position));
        e.mem.set_f32(position + 4, 3.0);
        let result = e.mem.alloc(0x40);
        double(&mut e, MULTIPLY_MATRIX_BY_RACE, eax(result));
        double(&mut e, ACTOR_LIGHTING_FORCE_TEST, eax(0));
        double(&mut e, SHADOW_SCENE_NODE, eax(0xaaaa));
        e.mem.set_u8(SKIP_SET_POSITION, 0);
        e.call_log = Some(vec![]);
        actor_update_actor_3d_position(&mut e, actor);
        let place = args_of(&e, SET_WORLD_POSITION);
        assert_eq!(place[0], 0x1000);
        assert_eq!(args_of(&e, MATRIX_MAKE_ROTATION)[1], 0.75f32.to_bits());
        assert_eq!(args_of(&e, SET_WORLD_ROTATION), vec![0x1000, result]);
        assert_eq!(args_of(&e, UPDATE_OBJECT_LIGHTING), vec![0xaaaa, 0x1000, 0]);
        assert!(e.get(actor, Actor::bLightingUpdatedNonMoving));
    }

    #[test]
    fn position_update_forces_the_lighting_when_asked() {
        let mut e = engine2();
        let actor = actor2(&mut e);
        let other = actor2(&mut e);
        set_player(&mut e, other);
        slot(&mut e, ACTOR_TABLE, VSLOT_GET_3D, eax(0x1000));
        let position = e.mem.alloc(16);
        double(&mut e, POSITION_VECTOR_OF, eax(position));
        let result = e.mem.alloc(0x40);
        double(&mut e, MULTIPLY_MATRIX_BY_RACE, eax(result));
        double(&mut e, ACTOR_LIGHTING_FORCE_TEST, eax(1));
        double(&mut e, SHADOW_SCENE_NODE, eax(0xaaaa));
        e.set(actor, Actor::bLightingUpdatedNonMoving, true);
        e.call_log = Some(vec![]);
        actor_update_actor_3d_position(&mut e, actor);
        assert_eq!(args_of(&e, UPDATE_OBJECT_LIGHTING), vec![0xaaaa, 0x1000, 1]);
        assert!(!e.get(actor, Actor::bLightingUpdatedNonMoving));
    }

    // ---- third block (0088b4e0 and later) ----

    type CallRecord = std::rc::Rc<std::cell::RefCell<Vec<Vec<u32>>>>;

    /// Registers a double at `addr` that answers `ret` and records the
    /// argument words of each call.
    fn record(e: &mut Engine, addr: u32, ret: Ret) -> CallRecord {
        let log: CallRecord = Default::default();
        let sink = log.clone();
        e.register_double(addr, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            ret
        });
        log
    }

    /// Puts a recording function into a vtable slot of `table`.
    fn record_slot(e: &mut Engine, table: u32, offset: u32, ret: Ret) -> CallRecord {
        let target = slot_target(table, offset);
        e.mem.set_u32(table + offset, target);
        record(e, target, ret)
    }

    fn engine3() -> Engine {
        let mut e = engine2();
        for page in [0x0119_b000u32, 0x011c_f000, 0x011f_2000] {
            if !e.mem.is_mapped(page) {
                e.map(page, 0x1000);
            }
        }
        e
    }

    #[test]
    fn extra_list_word_is_read_and_the_process_is_returned() {
        let mut e = engine3();
        let actor = actor2(&mut e);
        e.mem.set_u32(actor.addr() + 0x68, 0x4444);
        double(
            &mut e,
            GET_EXTRA_DATA_LIST_ADDRESS,
            eax(actor.addr() + 0x44),
        );
        let word = record(&mut e, EXTRA_LIST_WORD_0X29, eax(7));
        assert_eq!(fn_0088b4e0(&mut e, actor), 0x4444);
        assert_eq!(*word.borrow(), vec![vec![actor.addr() + 0x44]]);
    }

    fn health_setup(e: &mut Engine, applies: bool) -> (Ptr<Actor>, CallRecord, CallRecord) {
        let actor = actor2(e);
        slot(e, ACTOR_TABLE, VSLOT_ACTOR_FLAG_0X360, eax(applies as u32));
        slot(e, OTHER_TABLE, OWNER_VSLOT_CURRENT_VALUE, st0(-5.0));
        let applied = record_slot(e, ACTOR_TABLE, VSLOT_APPLY_VALUE_CHANGE, Ret::default());
        double(e, SIT_SLEEP_STATE_IS_NINE, eax(0xffff_ff01));
        double(e, HEALTH_BASE_VALUE, st0(10.0));
        let formula = record(e, HEALTH_REGENERATION_FORMULA, st0(2.0));
        e.mem.set_u32(PLAYER_CHARACTER, 0x7777);
        (actor, applied, formula)
    }

    #[test]
    fn restore_health_adds_the_scaled_entry_point_value() {
        let mut e = engine3();
        let (actor, applied, formula) = health_setup(&mut e, true);
        e.register(HANDLE_ENTRY_POINT, |e, a| {
            assert_eq!((a[0], a[1]), (0x0c, 0x7777));
            assert_eq!(e.mem.f32(a[2]), 0.0);
            e.mem.set_f32(a[2], 3.0);
            Ret::default()
        });
        actor_restore_health(&mut e, actor, 0.5);
        assert_eq!(*formula.borrow(), vec![vec![10.0f32.to_bits(), 1]]);
        assert_eq!(
            *applied.borrow(),
            vec![vec![actor.addr(), 0x10, 2.5f32.to_bits(), 0]]
        );
    }

    #[test]
    fn restore_health_without_the_virtual_uses_only_the_rate() {
        let mut e = engine3();
        let (actor, applied, _) = health_setup(&mut e, false);
        actor_restore_health(&mut e, actor, 0.5);
        assert_eq!(
            *applied.borrow(),
            vec![vec![actor.addr(), 0x10, 1.0f32.to_bits(), 0]]
        );
    }

    fn fatigue_setup(e: &mut Engine, fatigue: f32, current: f32) -> (Ptr<Actor>, CallRecord) {
        let actor = actor2(e);
        double(e, ACTOR_GET_ENDURANCE, st0(6.0));
        double(e, FATIGUE_REGENERATION_FORMULA, st0(4.0));
        double(e, GET_FATIGUE, st0(fatigue));
        slot(e, OTHER_TABLE, OWNER_VSLOT_CURRENT_VALUE, st0(current));
        double(e, ACTOR_VALUE_HAS_FLAG_MASK, eax(0));
        let applied = record_slot(e, ACTOR_TABLE, VSLOT_APPLY_VALUE_CHANGE, Ret::default());
        (actor, applied)
    }

    #[test]
    fn restore_fatigue_tops_up_to_one_when_it_reaches_zero_but_is_negative() {
        let mut e = engine3();
        let (actor, applied) = fatigue_setup(&mut e, -2.0, -1.0);
        actor_restore_fatigue(&mut e, actor, 0.5);
        assert_eq!(
            *applied.borrow(),
            vec![
                vec![actor.addr(), 0x16, 2.0f32.to_bits(), 0],
                vec![actor.addr(), 0x16, 2.0f32.to_bits(), 0],
            ]
        );
    }

    #[test]
    fn restore_fatigue_restores_once_when_it_stays_negative_or_was_not() {
        let mut e = engine3();
        let (actor, applied) = fatigue_setup(&mut e, -3.0, -1.0);
        actor_restore_fatigue(&mut e, actor, 0.5);
        assert_eq!(applied.borrow().len(), 1);
        let mut e = engine3();
        let (actor, applied) = fatigue_setup(&mut e, 1.0, -1.0);
        actor_restore_fatigue(&mut e, actor, 0.5);
        assert_eq!(applied.borrow().len(), 1);
    }

    fn action_point_setup(e: &mut Engine, flag: bool, blocked: u32) -> (Ptr<Actor>, CallRecord) {
        let actor = actor2(e);
        slot(e, ACTOR_TABLE, VSLOT_ACTOR_FLAG_0X360, eax(flag as u32));
        e.register_double(ITEM_VALUE, move |_, a| {
            eax(if a[0] == ACTION_POINT_BLOCK_OBJECT {
                blocked
            } else {
                a[0] + 1
            })
        });
        double(e, ACTION_POINT_REGENERATION_FORMULA, st0(4.0));
        slot(e, OTHER_TABLE, OWNER_VSLOT_VALUE, st0(3.0));
        slot(e, OTHER_TABLE, OWNER_VSLOT_CURRENT_VALUE, st0(-1.0));
        let applied = record_slot(e, ACTOR_TABLE, VSLOT_APPLY_VALUE_CHANGE, Ret::default());
        (actor, applied)
    }

    #[test]
    fn restore_action_points_scales_by_the_entry_point_and_the_owner_value() {
        let mut e = engine3();
        let (actor, applied) = action_point_setup(&mut e, true, 0);
        let process = with_process(&mut e, actor);
        record_slot(&mut e, PROCESS_TABLE, PROCESS_VSLOT_ITEM, eax(0x5000));
        let entry = record(&mut e, HANDLE_ENTRY_POINT, Ret::default());
        e.register_double(HANDLE_ENTRY_POINT, {
            let entry = entry.clone();
            move |e, a| {
                entry.borrow_mut().push(a.to_vec());
                assert_eq!(e.mem.f32(a[3]), 2.0);
                e.mem.set_f32(a[3], 5.0);
                Ret::default()
            }
        });
        let _ = process;
        actor_restore_action_points(&mut e, actor, 0.5);
        assert_eq!(entry.borrow()[0][..3], [0x27, actor.addr(), 0x5001]);
        assert_eq!(
            *applied.borrow(),
            vec![vec![actor.addr(), 0x0c, 15.0f32.to_bits(), 0]]
        );
    }

    #[test]
    fn restore_action_points_without_an_item_passes_zero() {
        let mut e = engine3();
        let (actor, _) = action_point_setup(&mut e, true, 0);
        let entry = record(&mut e, HANDLE_ENTRY_POINT, Ret::default());
        actor_restore_action_points(&mut e, actor, 0.5);
        assert_eq!(entry.borrow()[0][..3], [0x27, actor.addr(), 0]);
    }

    #[test]
    fn restore_action_points_needs_the_virtual_and_a_clear_setting() {
        let mut e = engine3();
        let (actor, applied) = action_point_setup(&mut e, false, 0);
        actor_restore_action_points(&mut e, actor, 0.5);
        assert!(applied.borrow().is_empty());
        let mut e = engine3();
        let (actor, applied) = action_point_setup(&mut e, true, 1);
        actor_restore_action_points(&mut e, actor, 0.5);
        assert!(applied.borrow().is_empty());
    }

    #[test]
    fn restore_actor_value_applies_positive_amounts_to_negative_values() {
        let mut e = engine3();
        let actor = actor2(&mut e);
        slot(&mut e, OTHER_TABLE, OWNER_VSLOT_CURRENT_VALUE, st0(-1.0));
        let applied = record_slot(
            &mut e,
            ACTOR_TABLE,
            VSLOT_APPLY_VALUE_CHANGE,
            Ret::default(),
        );
        double(&mut e, ACTOR_VALUE_HAS_FLAG_MASK, eax(0));
        actor_restore_actor_value(&mut e, actor, 5, 2.0);
        assert_eq!(
            *applied.borrow(),
            vec![vec![actor.addr(), 5, 2.0f32.to_bits(), 0]]
        );
        // A non-negative current value and no flag: nothing.
        slot(&mut e, OTHER_TABLE, OWNER_VSLOT_CURRENT_VALUE, st0(0.0));
        actor_restore_actor_value(&mut e, actor, 5, 2.0);
        assert_eq!(applied.borrow().len(), 1);
    }

    #[test]
    fn restore_actor_value_applies_negative_amounts_only_with_the_flag() {
        let mut e = engine3();
        let actor = actor2(&mut e);
        slot(&mut e, OTHER_TABLE, OWNER_VSLOT_CURRENT_VALUE, st0(-1.0));
        let applied = record_slot(
            &mut e,
            ACTOR_TABLE,
            VSLOT_APPLY_VALUE_CHANGE,
            Ret::default(),
        );
        let flag = record(&mut e, ACTOR_VALUE_HAS_FLAG_MASK, eax(0));
        actor_restore_actor_value(&mut e, actor, 5, -2.0);
        assert!(applied.borrow().is_empty());
        assert_eq!(*flag.borrow(), vec![vec![5, 0x200]]);
        double(&mut e, ACTOR_VALUE_HAS_FLAG_MASK, eax(1));
        actor_restore_actor_value(&mut e, actor, 5, -2.0);
        assert_eq!(
            *applied.borrow(),
            vec![vec![actor.addr(), 5, (-2.0f32).to_bits(), 0]]
        );
        // A positive amount with the flag but a non-negative value is dropped.
        slot(&mut e, OTHER_TABLE, OWNER_VSLOT_CURRENT_VALUE, st0(3.0));
        actor_restore_actor_value(&mut e, actor, 5, 2.0);
        assert_eq!(applied.borrow().len(), 1);
    }

    #[test]
    fn object_table_entry_needs_a_process_and_an_object() {
        let mut e = engine3();
        let actor = actor2(&mut e);
        assert_eq!(fn_0088b7f0(&mut e, actor), 0);
        let process = with_process(&mut e, actor);
        record_slot(&mut e, PROCESS_TABLE, PROCESS_VSLOT_OBJECT, eax(0));
        assert_eq!(fn_0088b7f0(&mut e, actor), 0);
        record_slot(&mut e, PROCESS_TABLE, PROCESS_VSLOT_OBJECT, eax(0x6000));
        double(&mut e, OBJECT_TYPE, eax(3));
        e.mem.set_u32(OBJECT_TYPE_TABLE + 12, 0xbeef);
        assert_eq!(fn_0088b7f0(&mut e, actor), 0xbeef);
        let _ = process;
    }

    #[test]
    fn weapon_reach_uses_the_setting_float() {
        let mut e = engine3();
        e.mem.set_f32(SCRATCH_FLOAT, 12.5);
        let setting = record(&mut e, SETTING_VALUE_ADDRESS, eax(SCRATCH_FLOAT));
        let reach = record(&mut e, CALC_WEAPON_REACH, st0(99.0));
        assert_eq!(fn_0088b850(&mut e, Ptr::new(0)), 99.0);
        assert_eq!(*setting.borrow(), vec![vec![WEAPON_REACH_SETTING]]);
        assert_eq!(*reach.borrow(), vec![vec![12.5f32.to_bits()]]);
    }

    // ---- line of sight ----

    const TARGET_TABLE: u32 = 0x0200_8000;
    const FORM_TABLE: u32 = 0x0200_a000;
    const CELL_OBJECT_TABLE: u32 = 0x0200_c000;
    const COUNTED_TABLE: u32 = 0x0200_e000;

    fn new_table(e: &mut Engine, table: u32) {
        e.map(table, 0x800);
        for offset in (0..0x800).step_by(4) {
            e.mem.set_u32(table + offset, DEFAULT_TARGET);
        }
    }

    fn object_with(e: &mut Engine, table: u32) -> u32 {
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object, table);
        object
    }

    fn map_pages(e: &mut Engine, pages: &[u32]) {
        for page in pages {
            if !e.mem.is_mapped(*page) {
                e.map(*page, 0x1000);
            }
        }
    }

    /// Doubles for everything `LineOfSight` and its relatives call, with
    /// vectors that really move words around. Returns (actor, target).
    fn sight_world(e: &mut Engine) -> (Ptr<Actor>, u32) {
        map_pages(
            e,
            &[
                0x011c_d000,
                0x011d_f000,
                0x011a_3000,
                0x0101_6000,
                0x0101_d000,
                0x0104_f000,
                0x0102_3000,
            ],
        );
        for table in [TARGET_TABLE, FORM_TABLE, CELL_OBJECT_TABLE, COUNTED_TABLE] {
            new_table(e, table);
        }
        e.set_global(VIEW_CONE_DEGREES_SETTING, 0.0f32);
        e.mem.set_f32(SCRATCH_FLOAT, 90.0);
        e.set_global(DEGREES_TO_RADIANS_FACTOR, std::f64::consts::PI / 180.0);
        e.set_global(SIGHT_CLOSE_DISTANCE, 2.0f64);
        e.set_global(SIGHT_HEIGHT_FACTOR, 0.75f64);
        e.set_global(SIGHT_SQUARED_LIMIT, 25.0f64);
        e.set_global(RAY_BOX_MARGIN, 1.5f32);
        e.set_global(RAY_DISTANCE_NUMERATOR, 100.0f32);
        e.mem.set_u8(LINE_OF_SIGHT_OVERRIDE_FLAG, 0);
        e.mem.set_u32(RAY_COUNTER, 0);
        for (address, value) in RAY_HEIGHT_FRACTIONS.iter().zip([0.75f32, 0.5, 0.25]) {
            e.mem.set_f32(*address, value);
        }
        let actor = actor2(e);
        let target = object_with(e, TARGET_TABLE);
        let form = object_with(e, FORM_TABLE);
        let actor_position = e.mem.alloc(16);
        let target_position = e.mem.alloc(16);
        e.mem.set_f32(actor_position + 8, 100.0);
        e.mem.set_f32(target_position + 8, 50.0);
        let actor_address = actor.addr();
        e.register_double(POSITION_VECTOR_OF, move |_, a| {
            eax(if a[0] == actor_address {
                actor_position
            } else {
                target_position
            })
        });
        e.register(VECTOR_CONSTRUCT, |_, a| eax(a[0]));
        e.register(VECTOR_COPY, |e, a| {
            for word in 0..3 {
                let value = e.mem.u32(a[1] + 4 * word);
                e.mem.set_u32(a[0] + 4 * word, value);
            }
            eax(a[0])
        });
        e.register(VECTOR_COMPONENT_ADDRESS, |_, a| eax(a[0] + 4 * a[1]));
        e.register(VECTOR_MINUS_VECTOR, |e, a| {
            for word in 0..3 {
                let value = e.mem.f32(a[0] + 4 * word) - e.mem.f32(a[2] + 4 * word);
                e.mem.set_f32(a[1] + 4 * word, value);
            }
            eax(a[1])
        });
        e.register(FLOAT_MAGNITUDE, |_, a| st0(f32::from_bits(a[0]).abs()));
        e.register(GET_Z_ANGLE_FROM_VECTOR, |_, _| st0(0.5));
        e.register(ANGLE_DIFFERENCE, |_, a| {
            st0(f32::from_bits(a[1]) - f32::from_bits(a[0]))
        });
        bounds_and_scale(e, 2.0);
        slot(e, ACTOR_TABLE, VSLOT_GET_HEADING_0X2BC, st0(0.25));
        double(e, ACTOR_GET_IN_COMBAT, eax(0));
        double(e, SPEED_FLAG_TEST, eax(0));
        double(e, GET_EYE_LEVEL, st0(6.0));
        double(e, GET_SCALE, st0(2.0));
        double(e, GET_DISTANCE_FROM_REFERENCE, st0(10.0));
        double(e, GET_PARENT_CELL, eax(0xc000));
        double(e, BASE_FORM_OF, eax(form));
        double(e, TES_IS_CELL_LOADED, eax(1));
        double(e, PICK_BLOCKED, eax(0));
        double(e, TES_PICK, eax(0));
        double(e, FIND_REFERENCE_FOR_3D, eax(0));
        e.mem.set_u32(TES_POINTER, 0x7000);
        e.mem.set_u32(PLAYER_CHARACTER, 0);
        slot_at(e, TARGET_TABLE, VSLOT_GET_3D, eax(0x5555));
        slot_at(e, TARGET_TABLE, VSLOT_IS_PROCESS_FLAG_0X22C, eax(0));
        slot_at(e, TARGET_TABLE, VSLOT_IS_ACTOR, eax(0));
        slot_at(e, TARGET_TABLE, REFERENCE_VSLOT_0XFC, eax(0));
        let bounds_upper = e.mem.alloc(16);
        let bounds_lower = e.mem.alloc(16);
        e.mem.set_f32(bounds_upper + 8, 20.0);
        e.mem.set_f32(bounds_lower + 8, 4.0);
        slot_at(e, TARGET_TABLE, VSLOT_BOUNDS_UPPER, eax(bounds_upper));
        slot_at(e, TARGET_TABLE, VSLOT_BOUNDS_LOWER, eax(bounds_lower));
        slot_at(e, FORM_TABLE, BASE_FORM_VSLOT_0X180, eax(0));
        let cell_object = object_with(e, CELL_OBJECT_TABLE);
        let inner = e.mem.alloc(0x100);
        let counted = object_with(e, COUNTED_TABLE);
        double(e, CELL_OBJECT_OF, eax(cell_object));
        double(e, CELL_OBJECT_TOUCH, Ret::default());
        slot_at(e, CELL_OBJECT_TABLE, CELL_OBJECT_VSLOT_0X94, eax(inner));
        double(e, OBJECT_WORD_0X58, eax(counted));
        slot_at(e, COUNTED_TABLE, COUNTED_OBJECT_VSLOT_COUNT, eax(0));
        slot_at(e, COUNTED_TABLE, COUNTED_OBJECT_VSLOT_FILL, Ret::default());
        for address in [
            PICK_CONSTRUCT,
            QUERY_CONSTRUCT,
            QUERY_DESTRUCT,
            PICK_SET_QUERY,
            PICK_SET_START,
            PICK_SET_END,
            PICK_SET_FILTER,
            PICK_SET_ENTRIES,
            CHAR_CONTROLLER_FILTER,
            STORE_WORD,
            FILTER_SET_LAYER,
            FILTER_SET_GROUP,
            VECTOR_FROM_FLOAT,
            VECTOR_MULTIPLY,
            BOX_INIT,
            BOX_MIN,
            BOX_MAX,
            QUERY_SET_DISTANCE_SCALE,
            FREE_ENTRIES,
            PICK_RESULT_POINT,
        ] {
            double(e, address, Ret::default());
        }
        double(e, FILTER_GET_GROUP, eax(0x1234));
        double(e, ALLOCATE_ENTRIES, eax(0x6000));
        (actor, target)
    }

    /// Like `slot`, for a table whose slots were not filled by `engine2`.
    fn slot_at(e: &mut Engine, table: u32, offset: u32, ret: Ret) {
        slot(e, table, offset, ret);
    }

    fn rays_world() -> (Engine, Ptr<Actor>, u32) {
        let mut e = engine3();
        let (actor, target) = sight_world(&mut e);
        // Anything tested by the view cone: 0.5 - 0.25 = 0.25 rad, cone of
        // 90 degrees: well inside.
        (e, actor, target)
    }

    #[test]
    fn line_of_sight_needs_a_target_with_3d() {
        let (mut e, actor, target) = rays_world();
        assert!(!actor_line_of_sight(
            &mut e,
            actor,
            true,
            Ptr::new(0),
            false,
            Ptr::new(0),
            false
        ));
        slot_at(&mut e, TARGET_TABLE, VSLOT_GET_3D, eax(0));
        assert!(!actor_line_of_sight(
            &mut e,
            actor,
            true,
            Ptr::new(target),
            false,
            Ptr::new(0),
            false
        ));
    }

    #[test]
    fn line_of_sight_without_the_ray_flag_needs_the_virtual_to_say_so() {
        let (mut e, actor, target) = rays_world();
        slot_at(&mut e, TARGET_TABLE, VSLOT_IS_PROCESS_FLAG_0X22C, eax(1));
        slot(&mut e, ACTOR_TABLE, VSLOT_ACTOR_FLAG_0X360, eax(0));
        assert!(!actor_line_of_sight(
            &mut e,
            actor,
            false,
            Ptr::new(target),
            false,
            Ptr::new(0),
            false
        ));
        // With the actor virtual true the process is asked instead.
        slot(&mut e, ACTOR_TABLE, VSLOT_ACTOR_FLAG_0X360, eax(1));
        assert!(!actor_line_of_sight(
            &mut e,
            actor,
            false,
            Ptr::new(target),
            false,
            Ptr::new(0),
            false
        ));
        with_process(&mut e, actor);
        slot_at(&mut e, TARGET_TABLE, VSLOT_IS_ACTOR, eax(1));
        let asked = record_slot(
            &mut e,
            PROCESS_TABLE,
            PROCESS_VSLOT_LINE_OF_SIGHT,
            eax(0xffff_ff01),
        );
        assert!(actor_line_of_sight(
            &mut e,
            actor,
            false,
            Ptr::new(target),
            false,
            Ptr::new(0),
            true
        ));
        assert_eq!(
            *asked.borrow(),
            vec![vec![
                e.get(actor, Actor::pCurrentProcess).addr(),
                actor.addr(),
                target,
                0,
                1
            ]]
        );
    }

    #[test]
    fn line_of_sight_is_true_at_once_within_two_units() {
        let (mut e, actor, target) = rays_world();
        double(&mut e, GET_DISTANCE_FROM_REFERENCE, st0(2.0));
        e.call_log = Some(vec![]);
        assert!(actor_line_of_sight(
            &mut e,
            actor,
            true,
            Ptr::new(target),
            false,
            Ptr::new(0),
            false
        ));
        assert!(!logged_addresses(&e).contains(&TES_PICK));
        double(&mut e, GET_DISTANCE_FROM_REFERENCE, st0(2.5));
        let result = e.mem.alloc(4);
        assert!(actor_line_of_sight(
            &mut e,
            actor,
            true,
            Ptr::new(target),
            false,
            Ptr::new(result),
            false
        ));
        assert_eq!(e.mem.u32(result), 2);
    }

    #[test]
    fn line_of_sight_reports_3_when_the_view_cone_fails() {
        let (mut e, actor, target) = rays_world();
        // Heading 0.25, angle 0.5: difference 0.25 rad; a cone of 0.2 rad
        // (11.46 degrees) is too narrow.
        e.mem.set_f32(SCRATCH_FLOAT, 11.4592);
        let result = e.mem.alloc(4);
        assert!(!actor_line_of_sight(
            &mut e,
            actor,
            true,
            Ptr::new(target),
            false,
            Ptr::new(result),
            false
        ));
        assert_eq!(e.mem.u32(result), 3);
        // Ignoring the cone goes on to the rays, which hit nothing.
        assert!(actor_line_of_sight(
            &mut e,
            actor,
            true,
            Ptr::new(target),
            false,
            Ptr::new(result),
            true
        ));
        assert_eq!(e.mem.u32(result), 2);
    }

    #[test]
    fn line_of_sight_override_flag_reports_2() {
        let (mut e, actor, target) = rays_world();
        e.mem.set_u8(LINE_OF_SIGHT_OVERRIDE_FLAG, 1);
        let result = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        assert!(actor_line_of_sight(
            &mut e,
            actor,
            true,
            Ptr::new(target),
            false,
            Ptr::new(result),
            true
        ));
        assert_eq!(e.mem.u32(result), 2);
        assert!(!logged_addresses(&e).contains(&GET_PARENT_CELL));
    }

    #[test]
    fn line_of_sight_fails_without_a_loaded_cell() {
        let (mut e, actor, target) = rays_world();
        double(&mut e, GET_PARENT_CELL, eax(0));
        assert!(!actor_line_of_sight(
            &mut e,
            actor,
            true,
            Ptr::new(target),
            false,
            Ptr::new(0),
            true
        ));
        double(&mut e, GET_PARENT_CELL, eax(0xc000));
        double(&mut e, TES_IS_CELL_LOADED, eax(0));
        assert!(!actor_line_of_sight(
            &mut e,
            actor,
            true,
            Ptr::new(target),
            false,
            Ptr::new(0),
            true
        ));
    }

    #[test]
    fn line_of_sight_casts_three_rays_for_the_player_and_reports_the_clear_one() {
        let (mut e, actor, target) = rays_world();
        e.mem.set_u32(PLAYER_CHARACTER, actor.addr());
        let ends = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let sink = ends.clone();
        e.register_double(PICK_SET_END, move |e, a| {
            sink.borrow_mut().push((a[1], e.mem.f32(a[1] + 8)));
            Ret::default()
        });
        let starts = record(&mut e, PICK_SET_START, Ret::default());
        // The first two rays hit another object, the third nothing.
        let mut hits = vec![0x4444u32, 0x4444, 0].into_iter();
        e.register_double(TES_PICK, move |_, _| eax(hits.next().unwrap()));
        double(&mut e, FIND_REFERENCE_FOR_3D, eax(0x9999));
        let result = e.mem.alloc(4);
        assert!(actor_line_of_sight(
            &mut e,
            actor,
            true,
            Ptr::new(target),
            true,
            Ptr::new(result),
            true
        ));
        assert_eq!(e.mem.u32(result), 0);
        // Extent: (20 - 4) * scale 2 = 32 on top of the target's z of 50.
        let heights: Vec<f32> = ends.borrow().iter().map(|entry| entry.1).collect();
        assert_eq!(heights, vec![74.0, 66.0, 58.0]);
        assert_eq!(e.mem.u32(RAY_COUNTER), 3);
        // The start is the actor's position raised by the eye level.
        let start = starts.borrow()[0][1];
        assert_eq!(e.mem.f32(start + 8), 106.0);
    }

    #[test]
    fn line_of_sight_casts_one_ray_for_other_actors_and_stops_at_a_hit_on_the_target() {
        let (mut e, actor, target) = rays_world();
        let mut hits = vec![0x4444u32].into_iter();
        e.register_double(TES_PICK, move |_, _| eax(hits.next().unwrap()));
        double(&mut e, FIND_REFERENCE_FOR_3D, eax(target));
        let result = e.mem.alloc(4);
        assert!(actor_line_of_sight(
            &mut e,
            actor,
            true,
            Ptr::new(target),
            false,
            Ptr::new(result),
            true
        ));
        assert_eq!(e.mem.u32(result), 2);
        assert_eq!(e.mem.u32(RAY_COUNTER), 1);
        // A hit on something else fails the single ray.
        double(&mut e, TES_PICK, eax(0x4444));
        double(&mut e, FIND_REFERENCE_FOR_3D, eax(0x9999));
        assert!(!actor_line_of_sight(
            &mut e,
            actor,
            true,
            Ptr::new(target),
            false,
            Ptr::new(result),
            true
        ));
        // A blocked ray fails even without a hit.
        double(&mut e, TES_PICK, eax(0));
        double(&mut e, PICK_BLOCKED, eax(1));
        assert!(!actor_line_of_sight(
            &mut e,
            actor,
            true,
            Ptr::new(target),
            false,
            Ptr::new(result),
            true
        ));
    }

    #[test]
    fn line_of_sight_scales_the_query_and_fills_the_entries() {
        let (mut e, actor, target) = rays_world();
        slot(&mut e, ACTOR_TABLE, VSLOT_RAY_SCALE_FLAG, eax(1));
        slot_at(&mut e, COUNTED_TABLE, COUNTED_OBJECT_VSLOT_COUNT, eax(3));
        let filled = record_slot(
            &mut e,
            COUNTED_TABLE,
            COUNTED_OBJECT_VSLOT_FILL,
            Ret::default(),
        );
        double(&mut e, VECTOR_LENGTH, st0(4.0));
        let scale = record(&mut e, QUERY_SET_DISTANCE_SCALE, Ret::default());
        let entries = record(&mut e, PICK_SET_ENTRIES, Ret::default());
        let freed = record(&mut e, FREE_ENTRIES, Ret::default());
        assert!(actor_line_of_sight(
            &mut e,
            actor,
            true,
            Ptr::new(target),
            false,
            Ptr::new(0),
            true
        ));
        assert_eq!(scale.borrow()[0][1], 25.0f32.to_bits());
        assert_eq!(entries.borrow()[0][1], 0x6000);
        assert_eq!(freed.borrow()[0], vec![0x6000]);
        assert_eq!(filled.borrow()[0][2], 0x6000);
    }

    #[test]
    fn line_of_sight_aims_at_the_process_objects_of_an_actor_target() {
        let (mut e, actor, target) = rays_world();
        slot_at(&mut e, TARGET_TABLE, VSLOT_IS_ACTOR, eax(1));
        with_process(&mut e, actor);
        let target_process = object_with(&mut e, PROCESS_TABLE);
        double(&mut e, GET_PROCESS, eax(target_process));
        slot_at(&mut e, FORM_TABLE, BASE_FORM_VSLOT_0X180, eax(1));
        let aim = e.mem.alloc(0x100);
        e.mem.set_f32(aim + 0x8c + 8, 77.0);
        slot_at(
            &mut e,
            PROCESS_TABLE,
            PROCESS_VSLOT_POSITION_OBJECT_A,
            eax(aim),
        );
        double(&mut e, POSITION_OF_OBJECT, eax(aim + 0x8c));
        let ends = record(&mut e, PICK_SET_END, Ret::default());
        assert!(actor_line_of_sight(
            &mut e,
            actor,
            true,
            Ptr::new(target),
            false,
            Ptr::new(0),
            true
        ));
        assert_eq!(ends.borrow()[0][1], aim + 0x8c);
    }

    #[test]
    fn point_sight_ray_is_clear_without_a_hit_or_with_a_near_one() {
        let (mut e, actor, _) = rays_world();
        let point = e.mem.alloc(16);
        e.mem.set_f32(point + 8, 10.0);
        e.register(VECTOR_LENGTH_SQUARED, |e, a| {
            let sum: f32 = (0..3).map(|i| e.mem.f32(a[0] + 4 * i).powi(2)).sum();
            st0(sum)
        });
        assert!(fn_0088c240(&mut e, actor, Ptr::new(point), false, true));
        // A hit whose point is within 5 units of the end counts, a farther
        // one does not.
        double(&mut e, TES_PICK, eax(0x4444));
        e.register_double(PICK_RESULT_POINT, |e, a| {
            e.mem.set_f32(a[1] + 8, 12.0);
            Ret::default()
        });
        assert!(fn_0088c240(&mut e, actor, Ptr::new(point), false, true));
        e.register_double(PICK_RESULT_POINT, |e, a| {
            e.mem.set_f32(a[1] + 8, 20.0);
            Ret::default()
        });
        assert!(!fn_0088c240(&mut e, actor, Ptr::new(point), false, true));
    }

    #[test]
    fn point_sight_needs_the_cone_and_a_loaded_cell() {
        let (mut e, actor, _) = rays_world();
        let point = e.mem.alloc(16);
        e.mem.set_f32(SCRATCH_FLOAT, 11.4592);
        assert!(!fn_0088c240(&mut e, actor, Ptr::new(point), false, false));
        assert!(fn_0088c240(&mut e, actor, Ptr::new(point), false, true));
        double(&mut e, GET_PARENT_CELL, eax(0));
        assert!(!fn_0088c240(&mut e, actor, Ptr::new(point), false, true));
        double(&mut e, GET_PARENT_CELL, eax(0xc000));
        double(&mut e, TES_IS_CELL_LOADED, eax(0));
        assert!(!fn_0088c240(&mut e, actor, Ptr::new(point), false, true));
    }

    #[test]
    fn point_sight_raises_the_start_by_the_sneak_height_when_sneaking() {
        let (mut e, actor, _) = rays_world();
        let point = e.mem.alloc(16);
        double(&mut e, SPEED_FLAG_TEST, eax(1));
        let starts = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let sink = starts.clone();
        e.register_double(PICK_SET_START, move |e, a| {
            sink.borrow_mut().push(e.mem.f32(a[1] + 8));
            Ret::default()
        });
        // The eye level (6.0) times the sneak factor (0.5) on top of z=100.
        assert!(fn_0088c240(&mut e, actor, Ptr::new(point), true, true));
        assert_eq!(*starts.borrow(), vec![103.0]);
        // Standing: the eye level itself.
        double(&mut e, SPEED_FLAG_TEST, eax(0));
        assert!(fn_0088c240(&mut e, actor, Ptr::new(point), true, true));
        assert_eq!(starts.borrow()[1], 106.0);
    }

    #[test]
    fn view_cone_compares_half_the_cone_with_the_angle_difference() {
        let (mut e, actor, _) = rays_world();
        let point = e.mem.alloc(16);
        // Difference 0.5 - 0.25 = 0.25.
        assert!(actor_is_point_in_view_cone(
            &mut e,
            actor,
            Ptr::new(point),
            0.6
        ));
        assert!(!actor_is_point_in_view_cone(
            &mut e,
            actor,
            Ptr::new(point),
            0.5
        ));
        assert!(!actor_is_point_in_view_cone(
            &mut e,
            actor,
            Ptr::new(point),
            0.4
        ));
    }

    #[test]
    fn has_360_line_of_sight_reads_the_entry_byte() {
        let (mut e, actor, _) = rays_world();
        assert_eq!(actor_has_360_line_of_sight(&mut e, actor, 7), 0);
        with_process(&mut e, actor);
        let asked = record_slot(&mut e, PROCESS_TABLE, PROCESS_VSLOT_0X504, eax(0));
        assert_eq!(actor_has_360_line_of_sight(&mut e, actor, 7), 0);
        assert_eq!(asked.borrow()[0][1..], [7, 0]);
        let entry = e.mem.alloc(0x40);
        e.mem.set_u8(entry + 0x1c, 3);
        record_slot(&mut e, PROCESS_TABLE, PROCESS_VSLOT_0X504, eax(entry));
        assert_eq!(actor_has_360_line_of_sight(&mut e, actor, 7), 3);
    }

    // ---- equipping ----

    /// Every callee of the equip functions gets a double that answers zero;
    /// the tests override the few they care about.
    const EQUIP_CALLEES: &[u32] = &[
        PROCESS_FORWARD_FLAG_MASK,
        SET_ACTION_FLAG,
        CREATE_MESSAGE,
        CREATE_MESSAGE_ALT,
        SHOW_MESSAGE,
        EXTRA_LIST_CAN_NOT_WEAR,
        EXTRA_LIST_GET_WORN,
        EXTRA_LIST_GET_CONTAINER_CHANGES,
        EXTRA_LIST_WEAPON_MOD_SLOT_ACTIVE,
        BASE_EXTRA_GET_DATA,
        INVENTORY_CHANGES_OF,
        INVENTORY_OBJECT_COUNT,
        INVENTORY_WORN_ITEM,
        BIPED_MODEL_OF,
        BIPED_IS_POWER_ARMOR,
        BIPED_FILLS_SLOT,
        FORM_HEALTH,
        FORM_FLAG_TEST_4C0BF0,
        ITEM_HAS_MOD_EFFECT_ACTIVE,
        FORM_CLIP_ROUNDS,
        COUNT_FOR_CLIPS,
        AMMO_NOTIFY_LEVEL,
        AMMO_NOTIFY,
        ITEM_CHANGE_CONSTRUCT,
        PLAYER_GET_BIPED,
        BIPED_SLOT_ENTRY,
        RT_DYNAMIC_CAST_FN,
        BIPED_CLEAR_SLOT,
        WORN_FIRST_WORD,
        FORM_WEAPON_MOD_KIND,
        ACTOR_RELOAD_TARGETS,
        INVENTORY_APPLY_EQUIP,
        NPC_INIT_WORN_OBJECT,
        FORM_ENCHANTING,
        MAGIC_ITEM_PRELOAD,
        MAGIC_ITEM_UNLOAD,
        REFRACTION_CONDITION,
        ACTOR_IS_REFRACTIVE,
        EXTRA_LIST_REFRACTION,
        MAGIC_TARGET_REMOVE_EFFECTS,
        MAGIC_TARGET_UPDATE,
        PLAYER_REMOVE_QUEUED_ENCHANTMENT,
        PLAYER_NOTIFY_EQUIPPED,
        ACTOR_NOTIFY_EQUIPPED,
        ACTOR_EQUIP_NOTIFICATION,
        ACTOR_EQUIP_STEP_A,
        ACTOR_EQUIP_STEP_B,
        ACTOR_EQUIP_STEP_C,
        ACTOR_EQUIP_STEP_D,
        PLAYER_EQUIP_STEP,
        PLAYER_AMMO_SWAP_HELPER,
        DATA_HANDLER_GET_SOUND,
        SOUND_PLAY_ARGUMENT,
        ACTOR_PLAY_SOUND,
        ACTOR_CAN_USE_FORM,
        ACTOR_EAT,
        ACTOR_DRINK_POTION,
        BOOK_READ,
        BOOK_SKILL_NUMBER,
        POTION_FOOD_TEST,
        MISC_STAT_INCREMENT,
        GET_DEFAULT_OBJECT,
        POTION_IS_WATER,
        POTION_CAN_BE_POISON,
        POTION_NOTIFY,
        PLAYER_POISON_CHOICE,
        GET_PIPBOY,
        PIPBOY_REFRESH,
        SAVE_LOAD_BIT_2_TEST,
        INVENTORY_REMOVE,
        COMBAT_CONTROLLER_TEST,
        ITEM_RELEASE,
        IS_IN_MENU_MODE,
        PLAYER_SLEEPING_OR_RESTING,
        EXTRA_GET_WEAPON_IDLE_SOUND,
        EXTRA_SET_WEAPON_IDLE_SOUND,
        EXTRA_GET_WEAPON_ATTACK_SOUND,
        EXTRA_SET_WEAPON_ATTACK_SOUND,
        SOUND_HANDLE_INIT,
        SOUND_HANDLE_IS_PLAYING,
        SOUND_HANDLE_RELEASE,
        SOUND_HANDLE_FADE_OUT_AND_RELEASE,
        EMPTY_FUNCTION,
        ACTOR_SET_BLOCK,
    ];

    /// An equip world: an actor, an item of form type `item_type` (written at
    /// +4 where the real `FORM_TYPE` reads it), and doubles for every callee.
    fn equip_world(e: &mut Engine, item_type: u8) -> (Ptr<Actor>, Ptr) {
        map_pages(
            e,
            &[
                0x0101_6000,
                0x011d_d000,
                0x011c_3000,
                0x0108_4000,
                0x0119_b000,
            ],
        );
        new_table(e, FORM_TABLE);
        e.set_global(MESSAGE_DURATION, 2.0f32);
        for address in EQUIP_CALLEES.iter().copied() {
            double(e, address, Ret::default());
        }
        e.register(FORM_TYPE, |e, a| eax(e.mem.u8(a[0] + 4) as u32));
        e.register(GET_EXTRA_DATA_LIST_ADDRESS, |_, a| eax(a[0] + 0x44));
        e.register(GET_PROCESS, |e, a| eax(e.mem.u32(a[0] + 0x68)));
        e.register(ITEM_VALUE, |e, a| eax(e.mem.u32(a[0] + 8)));
        e.register(ITEM_GROUP_INDEX, |_, _| eax(0));
        double(e, CREATE_MESSAGE, eax(0x5151));
        double(e, CREATE_MESSAGE_ALT, eax(0x5252));
        double(e, INVENTORY_CHANGES_OF, eax(0xa000));
        double(e, INVENTORY_OBJECT_COUNT, eax(5));
        double(e, FORM_HEALTH, eax(100));
        double(e, ACTOR_CAN_USE_FORM, eax(1));
        let actor = actor2(e);
        e.mem.set_u32(PLAYER_CHARACTER, 0);
        e.mem.set_u32(COMBAT_OBJECT_POINTER, 0x7777);
        e.mem.set_u32(SAVE_LOAD_GAME, 0x6666);
        let item = e.mem.alloc(0x200);
        e.mem.set_u32(item, FORM_TABLE);
        e.mem.set_u8(item + 4, item_type);
        (actor, Ptr::new(item))
    }

    fn make_player(e: &mut Engine, actor: Ptr<Actor>) {
        e.mem.set_u32(PLAYER_CHARACTER, actor.addr());
        // The player byte `fn_0088d2d0` reads lies past the small test actor.
        let page = (actor.addr() + 0x7c7) & !0xfff;
        for page in [page, page + 0x1000] {
            if !e.mem.is_mapped(page) {
                e.map(page, 0x1000);
            }
        }
    }

    /// Messages shown so far, as (setting object, icon): the creator takes only
    /// the setting (ECX); the icon is the third word of `SHOW_MESSAGE`.
    fn shown_messages(created: &CallRecord, shown: &CallRecord) -> Vec<(u32, u32)> {
        let created = created.borrow();
        let shown = shown.borrow();
        assert_eq!(created.len(), shown.len());
        created
            .iter()
            .zip(shown.iter())
            .map(|(made, call)| {
                assert_eq!(made.len(), 1);
                assert_eq!(call.len(), 6);
                (made[0], call[2])
            })
            .collect()
    }

    fn equip(e: &mut Engine, actor: Ptr<Actor>, item: Ptr, count: i32, extra: u32, flags: [u8; 3]) {
        actor_equip_object(
            e,
            actor,
            item,
            count,
            Ptr::new(extra),
            flags[0],
            flags[1],
            flags[2],
        );
    }

    #[test]
    fn restricted_ammo_use_sets_the_flags_and_stops() {
        let mut e = engine3();
        let (actor, item) = equip_world(&mut e, 0x29);
        slot_at(&mut e, FORM_TABLE, FORM_VSLOT_0X94, eax(1));
        let flags = record(&mut e, SET_ACTION_FLAG, Ret::default());
        equip(&mut e, actor, item, 1, 0x1234, [0, 0, 0]);
        assert_eq!(
            *flags.borrow(),
            vec![
                vec![actor.addr(), 0x1234, 2],
                vec![item.addr(), actor.addr() + 0x44, 2]
            ]
        );
    }

    #[test]
    fn power_armor_is_refused_for_the_player_unless_the_flag_byte_is_set() {
        let mut e = engine3();
        let (actor, item) = equip_world(&mut e, 0x18);
        make_player(&mut e, actor);
        let flag_page = (actor.addr() + 0x7c7) & !0xfff;
        for page in [flag_page, flag_page + 0x1000] {
            if !e.mem.is_mapped(page) {
                e.map(page, 0x1000);
            }
        }
        let tested = record(&mut e, BIPED_IS_POWER_ARMOR, eax(1));
        let created = record(&mut e, CREATE_MESSAGE, eax(0x5151));
        let shown = record(&mut e, SHOW_MESSAGE, Ret::default());
        let applied = record(&mut e, INVENTORY_APPLY_EQUIP, Ret::default());
        equip(&mut e, actor, item, 1, 0, [0, 0, 0]);
        assert_eq!(*tested.borrow(), vec![vec![item.addr() + 0x70]]);
        assert_eq!(
            shown_messages(&created, &shown),
            vec![(POWER_ARMOR_MESSAGE, MESSAGE_ICON_PATH)]
        );
        assert_eq!(shown.borrow()[0][4], 2.0f32.to_bits());
        assert_eq!(shown.borrow()[0][0], 0x5151);
        assert!(applied.borrow().is_empty());
        // A non-zero byte at +0x7c7 of the player lets the armor through.
        e.mem.set_u8(actor.addr() + 0x7c7, 1);
        equip(&mut e, actor, item, 1, 0, [0, 0, 0]);
        assert_eq!(created.borrow().len(), 1);
        assert_eq!(applied.borrow().len(), 1);
    }

    #[test]
    fn items_that_cannot_be_worn_are_refused_unless_the_flag_is_given() {
        let mut e = engine3();
        let (actor, item) = equip_world(&mut e, 0x18);
        make_player(&mut e, actor);
        let flag_page = (actor.addr() + 0x7c7) & !0xfff;
        for page in [flag_page, flag_page + 0x1000] {
            if !e.mem.is_mapped(page) {
                e.map(page, 0x1000);
            }
        }
        double(&mut e, EXTRA_LIST_CAN_NOT_WEAR, eax(1));
        let created = record(&mut e, CREATE_MESSAGE, eax(0x5151));
        let shown = record(&mut e, SHOW_MESSAGE, Ret::default());
        let applied = record(&mut e, INVENTORY_APPLY_EQUIP, Ret::default());
        equip(&mut e, actor, item, 1, 0x1234, [0, 0, 0]);
        assert_eq!(
            shown_messages(&created, &shown),
            vec![(CANNOT_WEAR_MESSAGE, MESSAGE_ICON_PATH)]
        );
        assert!(applied.borrow().is_empty());
        // flag_b lets it through; another actor gets no message.
        equip(&mut e, actor, item, 1, 0x1234, [0, 1, 0]);
        assert_eq!(applied.borrow().len(), 1);
        e.mem.set_u32(PLAYER_CHARACTER, 0);
        equip(&mut e, actor, item, 1, 0x1234, [0, 0, 0]);
        assert_eq!(created.borrow().len(), 1);
        assert_eq!(applied.borrow().len(), 1);
    }

    #[test]
    fn worn_items_and_missing_stock_stop_the_equip() {
        let mut e = engine3();
        let (actor, item) = equip_world(&mut e, 0x18);
        let applied = record(&mut e, INVENTORY_APPLY_EQUIP, Ret::default());
        double(&mut e, EXTRA_LIST_GET_WORN, eax(1));
        equip(&mut e, actor, item, 1, 0x1234, [0, 0, 0]);
        assert!(applied.borrow().is_empty());
        // Weapons ignore the worn test.
        e.mem.set_u8(item.addr() + 4, 0x28);
        equip(&mut e, actor, item, 1, 0x1234, [0, 0, 0]);
        assert_eq!(applied.borrow().len(), 1);
        // No stock: nothing happens.
        double(&mut e, INVENTORY_OBJECT_COUNT, eax(0));
        equip(&mut e, actor, item, 1, 0x1234, [0, 0, 0]);
        assert_eq!(applied.borrow().len(), 1);
    }

    #[test]
    fn weapons_check_their_health_and_run_the_follow_up_steps() {
        let mut e = engine3();
        let (actor, item) = equip_world(&mut e, 0x28);
        make_player(&mut e, actor);
        let flag_page = (actor.addr() + 0x7c7) & !0xfff;
        let _ = flag_page;
        let after = record_slot(
            &mut e,
            ACTOR_TABLE,
            VSLOT_AFTER_EQUIP_CHANGE,
            Ret::default(),
        );
        let applied = record(&mut e, INVENTORY_APPLY_EQUIP, Ret::default());
        let step = record(&mut e, PLAYER_EQUIP_STEP, Ret::default());
        let swap = record(&mut e, PLAYER_AMMO_SWAP_HELPER, Ret::default());
        let steps: Vec<CallRecord> = [
            ACTOR_EQUIP_STEP_A,
            ACTOR_EQUIP_STEP_B,
            ACTOR_EQUIP_STEP_C,
            ACTOR_EQUIP_STEP_D,
        ]
        .iter()
        .map(|address| record(&mut e, *address, Ret::default()))
        .collect();
        let flags = record(&mut e, SET_ACTION_FLAG, Ret::default());
        equip(&mut e, actor, item, 3, 0, [0, 0, 0]);
        assert_eq!(applied.borrow()[0][2..4], [1, actor.addr()]);
        assert_eq!(after.borrow().len(), 1);
        assert_eq!(step.borrow().len(), 1);
        assert_eq!(swap.borrow()[0][1..], [0, 1]);
        assert!(steps.iter().all(|step| step.borrow().len() == 1));
        assert_eq!(flags.borrow().len(), 2);
        // A stack that is kept together keeps its count.
        double(&mut e, FORM_FLAG_TEST_4C0BF0, eax(1));
        equip(&mut e, actor, item, 3, 0, [0, 0, 0]);
        assert_eq!(applied.borrow()[1][2..4], [3, actor.addr()]);
        // Broken: the message for the player, no equip, no use flag.
        double(&mut e, FORM_HEALTH, eax(0));
        let created = record(&mut e, CREATE_MESSAGE, eax(0x5151));
        let shown = record(&mut e, SHOW_MESSAGE, Ret::default());
        let before = applied.borrow().len();
        flags.borrow_mut().clear();
        equip(&mut e, actor, item, 1, 0, [0, 0, 0]);
        assert_eq!(applied.borrow().len(), before);
        assert_eq!(
            shown_messages(&created, &shown),
            vec![(BROKEN_ITEM_MESSAGE, MESSAGE_ICON_PATH)]
        );
        assert!(flags.borrow().is_empty());
    }

    #[test]
    fn the_extra_data_health_replaces_the_form_health() {
        let mut e = engine3();
        let (actor, item) = equip_world(&mut e, 0x28);
        let health = e.mem.alloc(0x20);
        e.mem.set_f32(health + 0xc, 0.0);
        e.register_double(BASE_EXTRA_GET_DATA, move |_, a| {
            assert_eq!(a[1], 0x25);
            eax(health)
        });
        let applied = record(&mut e, INVENTORY_APPLY_EQUIP, Ret::default());
        equip(&mut e, actor, item, 1, 0x1234, [0, 0, 0]);
        assert!(applied.borrow().is_empty());
        e.mem.set_f32(health + 0xc, 5.0);
        equip(&mut e, actor, item, 1, 0x1234, [0, 0, 0]);
        assert_eq!(applied.borrow().len(), 1);
    }

    #[test]
    fn armor_with_health_equips_and_notifies_the_player() {
        let mut e = engine3();
        let (actor, item) = equip_world(&mut e, 0x18);
        make_player(&mut e, actor);
        let flag_page = (actor.addr() + 0x7c7) & !0xfff;
        for page in [flag_page, flag_page + 0x1000] {
            if !e.mem.is_mapped(page) {
                e.map(page, 0x1000);
            }
        }
        let applied = record(&mut e, INVENTORY_APPLY_EQUIP, Ret::default());
        let player_notify = record(&mut e, PLAYER_NOTIFY_EQUIPPED, Ret::default());
        let other_notify = record(&mut e, ACTOR_NOTIFY_EQUIPPED, Ret::default());
        let after = record_slot(
            &mut e,
            ACTOR_TABLE,
            VSLOT_AFTER_EQUIP_CHANGE,
            Ret::default(),
        );
        equip(&mut e, actor, item, 4, 0x1234, [0, 0, 0]);
        assert_eq!(applied.borrow()[0][2..4], [1, actor.addr()]);
        assert_eq!(
            *player_notify.borrow(),
            vec![vec![actor.addr(), item.addr(), 0x1234]]
        );
        assert!(other_notify.borrow().is_empty());
        assert_eq!(after.borrow().len(), 1);
        // With flag_a, or for another actor, the other step is used.
        equip(&mut e, actor, item, 4, 0x1234, [1, 0, 0]);
        assert_eq!(other_notify.borrow().len(), 1);
        e.mem.set_u32(PLAYER_CHARACTER, 0);
        equip(&mut e, actor, item, 4, 0x1234, [0, 0, 0]);
        assert_eq!(other_notify.borrow().len(), 2);
    }

    #[test]
    fn armor_refreshes_the_pipboy_when_it_fills_slot_6() {
        let mut e = engine3();
        let (actor, item) = equip_world(&mut e, 0x18);
        double(&mut e, BIPED_MODEL_OF, eax(0x4040));
        e.register(BIPED_FILLS_SLOT, |_, a| {
            eax((a[0] == 0x4040 && a[1] == 6) as u32)
        });
        double(&mut e, GET_PIPBOY, eax(0x3030));
        let refreshed = record(&mut e, PIPBOY_REFRESH, Ret::default());
        equip(&mut e, actor, item, 1, 0, [0, 0, 0]);
        assert_eq!(*refreshed.borrow(), vec![vec![0x3030]]);
    }

    #[test]
    fn broken_armor_is_refused_and_leaves_the_use_flag_alone() {
        let mut e = engine3();
        let (actor, item) = equip_world(&mut e, 0x18);
        double(&mut e, FORM_HEALTH, eax(0));
        slot(&mut e, ACTOR_TABLE, VSLOT_ACTOR_FLAG_0X360, eax(1));
        let created = record(&mut e, CREATE_MESSAGE, eax(0x5151));
        let shown = record(&mut e, SHOW_MESSAGE, Ret::default());
        let applied = record(&mut e, INVENTORY_APPLY_EQUIP, Ret::default());
        let flags = record(&mut e, SET_ACTION_FLAG, Ret::default());
        equip(&mut e, actor, item, 1, 0, [0, 0, 0]);
        assert_eq!(
            shown_messages(&created, &shown),
            vec![(BROKEN_ITEM_MESSAGE, MESSAGE_ICON_PATH)]
        );
        assert!(applied.borrow().is_empty());
        assert!(flags.borrow().is_empty());
        slot(&mut e, ACTOR_TABLE, VSLOT_ACTOR_FLAG_0X360, eax(0));
        equip(&mut e, actor, item, 1, 0, [0, 0, 0]);
        assert_eq!(created.borrow().len(), 1);
    }

    #[test]
    fn unusable_forms_show_the_failure_message_to_the_player() {
        let mut e = engine3();
        let (actor, item) = equip_world(&mut e, 0x18);
        make_player(&mut e, actor);
        let flag_page = (actor.addr() + 0x7c7) & !0xfff;
        for page in [flag_page, flag_page + 0x1000] {
            if !e.mem.is_mapped(page) {
                e.map(page, 0x1000);
            }
        }
        double(&mut e, ACTOR_CAN_USE_FORM, eax(0));
        let created = record(&mut e, CREATE_MESSAGE, eax(0x5151));
        let shown = record(&mut e, SHOW_MESSAGE, Ret::default());
        let flags = record(&mut e, SET_ACTION_FLAG, Ret::default());
        equip(&mut e, actor, item, 1, 0, [0, 0, 0]);
        assert_eq!(
            shown_messages(&created, &shown),
            vec![(EQUIP_FAILED_MESSAGE, 0)]
        );
        assert!(flags.borrow().is_empty());
        // The light type goes the same way, silently for another actor.
        e.mem.set_u8(item.addr() + 4, 0x1e);
        e.mem.set_u32(PLAYER_CHARACTER, 0);
        equip(&mut e, actor, item, 1, 0, [0, 0, 0]);
        assert_eq!(created.borrow().len(), 1);
        assert_eq!(flags.borrow().len(), 2);
    }

    #[test]
    fn lights_play_the_torch_sound_and_then_equip() {
        let mut e = engine3();
        let (actor, item) = equip_world(&mut e, 0x1e);
        make_player(&mut e, actor);
        e.mem.set_u32(DATA_HANDLER_POINTER, 0x2020);
        e.register_double(DATA_HANDLER_GET_SOUND, |_, a| {
            assert_eq!(a, [0x2020, TORCH_EQUIP_SOUND_NAME]);
            eax(0x3131)
        });
        double(&mut e, SOUND_PLAY_ARGUMENT, eax(0x4242));
        double(&mut e, SOUND_HANDLE_INIT, Ret::default());
        let played = record(&mut e, ACTOR_PLAY_SOUND, Ret::default());
        let destroyed = record(&mut e, EMPTY_FUNCTION, Ret::default());
        let applied = record(&mut e, INVENTORY_APPLY_EQUIP, Ret::default());
        let flag_page = (actor.addr() + 0x7c7) & !0xfff;
        let _ = flag_page;
        equip(&mut e, actor, item, 1, 0, [0, 0, 0]);
        assert_eq!(played.borrow()[0][0], actor.addr());
        assert_eq!(played.borrow()[0][2..], [0x4242, 0, 0x102, 1]);
        assert_eq!(destroyed.borrow().len(), 2);
        assert_eq!(applied.borrow().len(), 1);
        // In a menu the flags are 0x120.
        double(&mut e, IS_IN_MENU_MODE, eax(1));
        equip(&mut e, actor, item, 1, 0, [0, 0, 0]);
        assert_eq!(played.borrow()[1][4], 0x120);
        // Without the sound nothing is played.
        double(&mut e, DATA_HANDLER_GET_SOUND, eax(0));
        equip(&mut e, actor, item, 1, 0, [0, 0, 0]);
        assert_eq!(played.borrow().len(), 2);
        assert_eq!(applied.borrow().len(), 3);
    }

    #[test]
    fn type_0x1a_skips_the_sound_and_the_health() {
        let mut e = engine3();
        let (actor, item) = equip_world(&mut e, 0x1a);
        double(&mut e, FORM_HEALTH, eax(0));
        let played = record(&mut e, ACTOR_PLAY_SOUND, Ret::default());
        let applied = record(&mut e, INVENTORY_APPLY_EQUIP, Ret::default());
        equip(&mut e, actor, item, 9, 0, [0, 0, 0]);
        assert!(played.borrow().is_empty());
        // The count is limited to one.
        assert_eq!(applied.borrow()[0][2..4], [1, actor.addr()]);
    }

    #[test]
    fn books_are_read_when_the_actor_and_book_allow_it() {
        let mut e = engine3();
        let (actor, item) = equip_world(&mut e, 0x19);
        slot(&mut e, ACTOR_TABLE, VSLOT_ACTOR_FLAG_0X360, eax(1));
        double(&mut e, BOOK_SKILL_NUMBER, eax(3));
        double(&mut e, BOOK_READ, eax(1));
        let read = record_slot(&mut e, ACTOR_TABLE, VSLOT_USE_ITEM_0X17C, Ret::default());
        let notified = record(&mut e, ACTOR_EQUIP_NOTIFICATION, Ret::default());
        let flags = record(&mut e, SET_ACTION_FLAG, Ret::default());
        equip(&mut e, actor, item, 1, 0x1234, [0, 0, 0]);
        assert_eq!(
            read.borrow()[0][1..],
            [item.addr(), 0x1234, 1, 0, 0, 0, 0, 0, 1, 0]
        );
        assert_eq!(
            *notified.borrow(),
            vec![vec![actor.addr(), item.addr(), 1, 1]]
        );
        assert!(flags.borrow().is_empty());
        // A book that cannot be read falls through to the use flags.
        double(&mut e, BOOK_SKILL_NUMBER, eax(0xffff_ffff));
        equip(&mut e, actor, item, 1, 0x1234, [0, 0, 0]);
        assert_eq!(read.borrow().len(), 1);
        assert_eq!(flags.borrow().len(), 2);
    }

    #[test]
    fn food_is_eaten_unless_the_form_says_no() {
        let mut e = engine3();
        let (actor, item) = equip_world(&mut e, 0x1d);
        make_player(&mut e, actor);
        let flag_page = (actor.addr() + 0x7c7) & !0xfff;
        let _ = flag_page;
        let eaten = record(&mut e, ACTOR_EAT, Ret::default());
        let notified = record(&mut e, ACTOR_EQUIP_NOTIFICATION, Ret::default());
        let flags = record(&mut e, SET_ACTION_FLAG, Ret::default());
        equip(&mut e, actor, item, 1, 0x1234, [0, 0, 0]);
        assert_eq!(
            *eaten.borrow(),
            vec![vec![actor.addr(), item.addr(), 0x1234, 0]]
        );
        assert_eq!(
            *notified.borrow(),
            vec![vec![actor.addr(), item.addr(), 1, 1]]
        );
        assert_eq!(flags.borrow().len(), 1);
        e.mem.set_u32(PLAYER_CHARACTER, 0);
        equip(&mut e, actor, item, 1, 0x1234, [0, 0, 0]);
        assert_eq!(eaten.borrow()[1][3], 1);
        // The form refuses: only the player is told.
        slot_at(&mut e, FORM_TABLE, FORM_VSLOT_0X94, eax(1));
        let created = record(&mut e, CREATE_MESSAGE, eax(0x5151));
        let shown = record(&mut e, SHOW_MESSAGE, Ret::default());
        equip(&mut e, actor, item, 1, 0x1234, [0, 0, 0]);
        assert!(created.borrow().is_empty());
        make_player(&mut e, actor);
        equip(&mut e, actor, item, 1, 0x1234, [0, 0, 0]);
        assert_eq!(
            shown_messages(&created, &shown),
            vec![(CANNOT_USE_MESSAGE, MESSAGE_ICON_PATH)]
        );
        assert_eq!(eaten.borrow().len(), 2);
    }

    #[test]
    fn potions_are_drunk_and_counted() {
        let mut e = engine3();
        let (actor, item) = equip_world(&mut e, 0x2f);
        slot(&mut e, ACTOR_TABLE, VSLOT_ACTOR_FLAG_0X360, eax(1));
        e.register(GET_DEFAULT_OBJECT, |_, _| eax(0x1111));
        let stats = record(&mut e, MISC_STAT_INCREMENT, Ret::default());
        let drunk = record(&mut e, ACTOR_DRINK_POTION, eax(1));
        let notified = record(&mut e, POTION_NOTIFY, Ret::default());
        let shown = record(&mut e, ACTOR_EQUIP_NOTIFICATION, Ret::default());
        equip(&mut e, actor, item, 1, 0x1234, [0, 0, 0]);
        assert_eq!(
            *drunk.borrow(),
            vec![vec![actor.addr(), item.addr(), 0x1234, 1]]
        );
        assert_eq!(notified.borrow().len(), 2);
        assert_eq!(notified.borrow()[0], vec![5, 1, item.addr(), 0, 0, 0, 0]);
        assert_eq!(notified.borrow()[1][0], 9);
        assert!(stats.borrow().is_empty());
        assert_eq!(*shown.borrow(), vec![vec![actor.addr(), item.addr(), 1, 1]]);
        // Statistics: food, the default objects and water.
        double(&mut e, POTION_FOOD_TEST, eax(1));
        equip(&mut e, actor, item, 1, 0x1234, [0, 0, 0]);
        assert_eq!(stats.borrow().last().unwrap(), &vec![9]);
        double(&mut e, POTION_FOOD_TEST, eax(0));
        let potion = item.addr();
        e.register_double(GET_DEFAULT_OBJECT, move |_, a| {
            eax(if a[0] == 2 { potion } else { 0x1111 })
        });
        equip(&mut e, actor, item, 1, 0x1234, [0, 0, 0]);
        assert_eq!(stats.borrow().last().unwrap(), &vec![7]);
        e.register(GET_DEFAULT_OBJECT, |_, _| eax(0x1111));
        double(&mut e, POTION_IS_WATER, eax(1));
        equip(&mut e, actor, item, 1, 0x1234, [0, 0, 0]);
        assert_eq!(stats.borrow().last().unwrap(), &vec![0x13]);
        // A drink that fails is not announced.
        double(&mut e, ACTOR_DRINK_POTION, eax(0));
        let before = shown.borrow().len();
        equip(&mut e, actor, item, 1, 0x1234, [0, 0, 0]);
        assert_eq!(shown.borrow().len(), before);
    }

    #[test]
    fn poison_is_offered_to_the_player_only() {
        let mut e = engine3();
        let (actor, item) = equip_world(&mut e, 0x2f);
        make_player(&mut e, actor);
        double(&mut e, POTION_CAN_BE_POISON, eax(1));
        let choice = record(&mut e, PLAYER_POISON_CHOICE, Ret::default());
        let drunk = record(&mut e, ACTOR_DRINK_POTION, Ret::default());
        equip(&mut e, actor, item, 1, 0x1234, [0, 0, 0]);
        assert_eq!(*choice.borrow(), vec![vec![actor.addr(), item.addr()]]);
        assert!(drunk.borrow().is_empty());
        // Another actor goes on to the common end: use flags and no drink.
        e.mem.set_u32(PLAYER_CHARACTER, 0);
        let flags = record(&mut e, SET_ACTION_FLAG, Ret::default());
        equip(&mut e, actor, item, 1, 0x1234, [0, 0, 0]);
        assert_eq!(choice.borrow().len(), 1);
        assert!(drunk.borrow().is_empty());
        assert_eq!(flags.borrow().len(), 2);
    }

    #[test]
    fn ammo_is_swapped_through_an_item_change() {
        let mut e = engine3();
        let (actor, item) = equip_world(&mut e, 0x29);
        let process = with_process(&mut e, actor);
        let held = e.mem.alloc(0x20);
        let held_form = e.mem.alloc(0x20);
        e.mem.set_u32(held + 8, held_form);
        record_slot(&mut e, PROCESS_TABLE, PROCESS_VSLOT_ITEM, eax(held));
        double(&mut e, ITEM_HAS_MOD_EFFECT_ACTIVE, eax(1));
        let rounds = record(&mut e, FORM_CLIP_ROUNDS, eax(30));
        let clips = record(&mut e, COUNT_FOR_CLIPS, eax(60));
        double(&mut e, AMMO_NOTIFY_LEVEL, eax(1));
        let notified = record(&mut e, AMMO_NOTIFY, Ret::default());
        double(&mut e, OPERATOR_NEW_FN, eax(0x8800));
        let built = record(&mut e, ITEM_CHANGE_CONSTRUCT, eax(0x8808));
        let equipped = record_slot(
            &mut e,
            PROCESS_TABLE,
            PROCESS_VSLOT_SET_EQUIPPED_CHANGE,
            Ret::default(),
        );
        equip(&mut e, actor, item, 2, 0, [0, 0, 0]);
        assert_eq!(*rounds.borrow(), vec![vec![held_form, 1]]);
        assert_eq!(*clips.borrow(), vec![vec![30, 2]]);
        assert_eq!(
            *notified.borrow(),
            vec![vec![item.addr(), 60, actor.addr(), 1, 1]]
        );
        assert_eq!(*built.borrow(), vec![vec![0x8800, item.addr(), 60]]);
        assert_eq!(*equipped.borrow(), vec![vec![process.addr(), 0x8808]]);
        // Groups 10 to 13 keep the count.
        e.register(ITEM_GROUP_INDEX, |_, _| eax(11));
        equip(&mut e, actor, item, 2, 0, [0, 0, 0]);
        assert_eq!(clips.borrow().len(), 1);
        assert_eq!(built.borrow()[1][2], 2);
        // An empty hand and a failed allocation give a null change.
        record_slot(&mut e, PROCESS_TABLE, PROCESS_VSLOT_ITEM, eax(0));
        double(&mut e, OPERATOR_NEW_FN, eax(0));
        equip(&mut e, actor, item, 2, 0, [0, 0, 0]);
        assert_eq!(equipped.borrow()[2][1], 0);
    }

    #[test]
    fn unsupported_types_tell_the_player_and_set_the_use_flags() {
        let mut e = engine3();
        let (actor, item) = equip_world(&mut e, 0x30);
        make_player(&mut e, actor);
        let created = record(&mut e, CREATE_MESSAGE, eax(0x5151));
        let shown = record(&mut e, SHOW_MESSAGE, Ret::default());
        let notified = record(&mut e, ACTOR_EQUIP_NOTIFICATION, Ret::default());
        let flags = record(&mut e, SET_ACTION_FLAG, Ret::default());
        equip(&mut e, actor, item, 1, 0x1234, [0, 0, 1]);
        assert_eq!(
            shown_messages(&created, &shown),
            vec![(CANNOT_WEAR_MESSAGE, MESSAGE_ICON_PATH)]
        );
        assert_eq!(
            *notified.borrow(),
            vec![vec![actor.addr(), item.addr(), 1, 0]]
        );
        assert_eq!(flags.borrow().len(), 2);
        // The combat object test suppresses the flags.
        double(&mut e, COMBAT_CONTROLLER_TEST, eax(1));
        equip(&mut e, actor, item, 1, 0x1234, [0, 0, 0]);
        assert_eq!(flags.borrow().len(), 2);
    }

    #[test]
    fn queued_equips_wait_for_the_process_unless_it_is_urgent() {
        let mut e = engine3();
        let (actor, item) = equip_world(&mut e, 0x28);
        let applied = record(&mut e, INVENTORY_APPLY_EQUIP, Ret::default());
        // No process: at once.
        actor_queue_equip_object(&mut e, actor, item, 1, Ptr::new(0x1234), 1, 0, 1);
        assert_eq!(applied.borrow().len(), 1);
        let process = with_process(&mut e, actor);
        double(&mut e, PROCESS_GET_LEVEL, eax(1));
        let queued = record_slot(
            &mut e,
            PROCESS_TABLE,
            PROCESS_VSLOT_QUEUE_EQUIP,
            Ret::default(),
        );
        actor_queue_equip_object(&mut e, actor, item, 3, Ptr::new(0x1234), 1, 0, 1);
        assert_eq!(
            *queued.borrow(),
            vec![vec![
                process.addr(),
                actor.addr(),
                1,
                item.addr(),
                3,
                0x1234,
                1,
                0,
                0,
                0,
                0,
                1
            ]]
        );
        assert_eq!(applied.borrow().len(), 1);
        // A high level, the player in a menu or the save/load bit equip now.
        double(&mut e, PROCESS_GET_LEVEL, eax(2));
        actor_queue_equip_object(&mut e, actor, item, 3, Ptr::new(0x1234), 1, 0, 1);
        assert_eq!(applied.borrow().len(), 2);
        double(&mut e, PROCESS_GET_LEVEL, eax(1));
        make_player(&mut e, actor);
        let flag_page = (actor.addr() + 0x7c7) & !0xfff;
        let _ = flag_page;
        double(&mut e, IS_IN_MENU_MODE, eax(1));
        actor_queue_equip_object(&mut e, actor, item, 3, Ptr::new(0x1234), 1, 0, 1);
        assert_eq!(applied.borrow().len(), 3);
        double(&mut e, IS_IN_MENU_MODE, eax(0));
        double(&mut e, SAVE_LOAD_BIT_2_TEST, eax(1));
        actor_queue_equip_object(&mut e, actor, item, 3, Ptr::new(0x1234), 1, 0, 1);
        assert_eq!(applied.borrow().len(), 4);
        assert_eq!(queued.borrow().len(), 1);
    }

    #[test]
    fn queued_equips_of_food_are_immediate_and_other_types_are_ignored() {
        let mut e = engine3();
        let (actor, item) = equip_world(&mut e, 0x1d);
        with_process(&mut e, actor);
        let queued = record_slot(
            &mut e,
            PROCESS_TABLE,
            PROCESS_VSLOT_QUEUE_EQUIP,
            Ret::default(),
        );
        let eaten = record(&mut e, ACTOR_EAT, Ret::default());
        double(&mut e, PROCESS_GET_LEVEL, eax(0));
        actor_queue_equip_object(&mut e, actor, item, 1, Ptr::new(0), 0, 0, 0);
        assert_eq!(eaten.borrow().len(), 1);
        e.mem.set_u8(item.addr() + 4, 0x30);
        actor_queue_equip_object(&mut e, actor, item, 1, Ptr::new(0), 0, 0, 0);
        assert!(queued.borrow().is_empty());
        assert_eq!(eaten.borrow().len(), 1);
    }

    #[test]
    fn queued_removals_wait_for_a_low_level_process() {
        let mut e = engine3();
        let (actor, item) = equip_world(&mut e, 0x28);
        let removed = record(&mut e, INVENTORY_REMOVE, eax(1));
        double(&mut e, EXTRA_LIST_GET_CONTAINER_CHANGES, eax(0x5050));
        // No process: removed at once.
        actor_queue_un_equip_object(&mut e, actor, item, 1, Ptr::new(0), 1, 2, 3);
        assert_eq!(removed.borrow().len(), 1);
        let process = with_process(&mut e, actor);
        double(&mut e, PROCESS_GET_LEVEL, eax(1));
        let queued = record_slot(
            &mut e,
            PROCESS_TABLE,
            PROCESS_VSLOT_QUEUE_EQUIP,
            Ret::default(),
        );
        actor_queue_un_equip_object(&mut e, actor, item, 2, Ptr::new(0x1234), 1, 2, 3);
        assert_eq!(
            *queued.borrow(),
            vec![vec![
                process.addr(),
                actor.addr(),
                0,
                item.addr(),
                2,
                0x1234,
                0,
                0,
                0,
                1,
                2,
                3
            ]]
        );
        assert_eq!(removed.borrow().len(), 1);
        double(&mut e, PROCESS_GET_LEVEL, eax(2));
        actor_queue_un_equip_object(&mut e, actor, item, 2, Ptr::new(0x1234), 1, 2, 3);
        assert_eq!(removed.borrow().len(), 2);
        double(&mut e, PROCESS_GET_LEVEL, eax(0));
        make_player(&mut e, actor);
        double(&mut e, IS_IN_MENU_MODE, eax(1));
        actor_queue_un_equip_object(&mut e, actor, item, 2, Ptr::new(0x1234), 1, 2, 3);
        assert_eq!(removed.borrow().len(), 3);
        assert_eq!(queued.borrow().len(), 1);
    }

    #[test]
    fn the_player_byte_is_read_from_offset_0x7c7() {
        let mut e = engine3();
        let object = e.mem.alloc(0x800);
        e.mem.set_u8(object + 0x7c7, 9);
        assert_eq!(fn_0088d2d0(&mut e, Ptr::new(object)), 9);
    }

    #[test]
    fn biped_slot_entries_are_16_bytes_apart() {
        let mut e = engine3();
        let data = e.mem.alloc(0x200);
        e.mem.set_u32(data + 0x2c + 0x30, 0xabcd);
        assert_eq!(fn_0088e0f0(&mut e, Ptr::new(data), 3), 0xabcd);
    }

    #[test]
    fn removal_marks_the_item_and_asks_the_inventory() {
        let mut e = engine3();
        let (actor, item) = equip_world(&mut e, 0x28);
        assert_eq!(
            fn_0088e110(&mut e, actor, Ptr::new(0), 1, Ptr::new(0), 0, 0),
            1
        );
        // Without container changes only the enchantment is looked at.
        let unloaded = record(&mut e, MAGIC_ITEM_UNLOAD, Ret::default());
        double(&mut e, FORM_ENCHANTING, eax(0x6000));
        assert_eq!(
            fn_0088e110(&mut e, actor, item, 1, Ptr::new(0x1234), 1, 1),
            0
        );
        assert_eq!(*unloaded.borrow(), vec![vec![0x6018, 1]]);
        double(&mut e, EXTRA_LIST_GET_CONTAINER_CHANGES, eax(0x5050));
        let flags = record(&mut e, SET_ACTION_FLAG, Ret::default());
        e.register_double(INVENTORY_REMOVE, |e, a| {
            assert_eq!(e.mem.u8(a[1]), 1);
            eax(1)
        });
        let result = fn_0088e110(&mut e, actor, item, 4, Ptr::new(0x1234), 1, 0);
        assert_eq!(result, 1);
        assert_eq!(
            *flags.borrow(),
            vec![
                vec![actor.addr(), 0x1234, 8],
                vec![item.addr(), actor.addr() + 0x44, 8]
            ]
        );
        // With the combat object test the flags are not set.
        double(&mut e, COMBAT_CONTROLLER_TEST, eax(1));
        flags.borrow_mut().clear();
        fn_0088e110(&mut e, actor, item, 4, Ptr::new(0x1234), 1, 0);
        assert!(flags.borrow().is_empty());
        double(&mut e, FORM_ENCHANTING, eax(0));
        let before = unloaded.borrow().len();
        fn_0088e110(&mut e, actor, item, 4, Ptr::new(0x1234), 1, 0);
        assert_eq!(unloaded.borrow().len(), before);
    }

    #[test]
    fn removal_passes_the_nine_words_to_the_inventory() {
        let mut e = engine3();
        let (actor, item) = equip_world(&mut e, 0x28);
        double(&mut e, EXTRA_LIST_GET_CONTAINER_CHANGES, eax(0x5050));
        let removed = record(&mut e, INVENTORY_REMOVE, eax(0));
        fn_0088e110(&mut e, actor, item, 4, Ptr::new(0x1234), 7, 8);
        let call = removed.borrow()[0].clone();
        assert_eq!(call[0], 0x5050);
        assert_eq!(
            call[2..],
            [item.addr(), 4, actor.addr(), 0x1234, 7, 8, 0, 0]
        );
    }

    #[test]
    fn unequip_refuses_items_that_cannot_be_worn_off() {
        let mut e = engine3();
        let (actor, item) = equip_world(&mut e, 0x18);
        make_player(&mut e, actor);
        double(&mut e, EXTRA_LIST_CAN_NOT_WEAR, eax(1));
        let created = record(&mut e, CREATE_MESSAGE_ALT, eax(0x5252));
        let shown = record(&mut e, SHOW_MESSAGE, Ret::default());
        let removed = record(&mut e, INVENTORY_REMOVE, eax(1));
        assert_eq!(
            actor_un_equip_object(&mut e, actor, item, 1, Ptr::new(0x1234), 0, 0, 0),
            0
        );
        assert_eq!(
            shown_messages(&created, &shown),
            vec![(CANNOT_REMOVE_MESSAGE, 0)]
        );
        assert_eq!(
            *shown.borrow(),
            vec![vec![0x5252, 0, 0, 0, 2.0f32.to_bits(), 0]]
        );
        assert!(removed.borrow().is_empty());
        // The virtual +0x22c lets it through.
        slot(&mut e, ACTOR_TABLE, VSLOT_IS_PROCESS_FLAG_0X22C, eax(1));
        double(&mut e, EXTRA_LIST_GET_CONTAINER_CHANGES, eax(0x5050));
        actor_un_equip_object(&mut e, actor, item, 1, Ptr::new(0x1234), 0, 0, 0);
        assert_eq!(removed.borrow().len(), 1);
    }

    #[test]
    fn unequipping_armor_drops_the_effects_and_removes() {
        let mut e = engine3();
        let (actor, item) = equip_world(&mut e, 0x18);
        make_player(&mut e, actor);
        double(&mut e, EXTRA_LIST_GET_CONTAINER_CHANGES, eax(0x5050));
        double(&mut e, EXTRA_LIST_GET_WORN, eax(1));
        double(&mut e, IS_IN_MENU_MODE, eax(1));
        let after = record_slot(
            &mut e,
            ACTOR_TABLE,
            VSLOT_AFTER_EQUIP_CHANGE,
            Ret::default(),
        );
        let effects = record(&mut e, MAGIC_TARGET_REMOVE_EFFECTS, Ret::default());
        let update = record(&mut e, MAGIC_TARGET_UPDATE, Ret::default());
        let queued = record(&mut e, PLAYER_REMOVE_QUEUED_ENCHANTMENT, Ret::default());
        let notified = record(&mut e, ACTOR_EQUIP_NOTIFICATION, Ret::default());
        let removed = record(&mut e, INVENTORY_REMOVE, eax(1));
        let result = actor_un_equip_object(&mut e, actor, item, 2, Ptr::new(0x1234), 5, 0, 1);
        assert_eq!(result, 1);
        assert_eq!(after.borrow().len(), 1);
        assert_eq!(
            *effects.borrow(),
            vec![vec![actor.addr() + 0x94, item.addr(), 1]]
        );
        assert_eq!(*update.borrow(), vec![vec![actor.addr() + 0x94, 0]]);
        assert_eq!(
            *queued.borrow(),
            vec![vec![actor.addr(), item.addr(), 0x1234]]
        );
        assert_eq!(
            *notified.borrow(),
            vec![vec![actor.addr(), item.addr(), 0, 0]]
        );
        // worn = 1 and flag_a = 5 reach the inventory call.
        assert_eq!(removed.borrow()[0][6..8], [1, 5]);
        // Sleeping or resting suppresses the target update.
        double(&mut e, PLAYER_SLEEPING_OR_RESTING, eax(1));
        actor_un_equip_object(&mut e, actor, item, 2, Ptr::new(0x1234), 5, 0, 0);
        assert_eq!(update.borrow().len(), 1);
        assert_eq!(notified.borrow().len(), 1);
        // Type 0x1a skips the actor virtual.
        e.mem.set_u8(item.addr() + 4, 0x1a);
        actor_un_equip_object(&mut e, actor, item, 2, Ptr::new(0x1234), 5, 0, 0);
        assert_eq!(after.borrow().len(), 2);
    }

    #[test]
    fn unequipping_other_types_and_nothing_does_nothing() {
        let mut e = engine3();
        let (actor, item) = equip_world(&mut e, 0x30);
        let removed = record(&mut e, INVENTORY_REMOVE, eax(1));
        assert_eq!(
            actor_un_equip_object(&mut e, actor, item, 1, Ptr::new(0), 0, 0, 0),
            0
        );
        assert_eq!(
            actor_un_equip_object(&mut e, actor, Ptr::new(0), 1, Ptr::new(0), 0, 0, 0),
            0
        );
        assert!(removed.borrow().is_empty());
    }

    #[test]
    fn unequipping_a_weapon_releases_the_sounds_and_resets_the_process() {
        let mut e = engine3();
        let (actor, item) = equip_world(&mut e, 0x28);
        let process = with_process(&mut e, actor);
        make_player(&mut e, actor);
        double(&mut e, EXTRA_LIST_GET_CONTAINER_CHANGES, eax(0x5050));
        double(&mut e, INVENTORY_REMOVE, eax(1));
        double(&mut e, SOUND_HANDLE_INIT, Ret::default());
        double(&mut e, EXTRA_GET_WEAPON_ATTACK_SOUND, Ret::default());
        double(&mut e, EXTRA_SET_WEAPON_ATTACK_SOUND, Ret::default());
        double(&mut e, EXTRA_SET_WEAPON_IDLE_SOUND, Ret::default());
        let playing = std::rc::Rc::new(std::cell::RefCell::new(vec![1u32, 0]));
        let order = playing.clone();
        e.register_double(SOUND_HANDLE_IS_PLAYING, move |_, _| {
            eax(order.borrow_mut().remove(0))
        });
        let faded = record(&mut e, SOUND_HANDLE_FADE_OUT_AND_RELEASE, Ret::default());
        let released = record(&mut e, SOUND_HANDLE_RELEASE, Ret::default());
        let destroyed = record(&mut e, EMPTY_FUNCTION, Ret::default());
        record_slot(&mut e, PROCESS_TABLE, PROCESS_VSLOT_0X458, Ret::default());
        record_slot(&mut e, PROCESS_TABLE, PROCESS_VSLOT_AMMO_ITEM, eax(1));
        let reset = record_slot(
            &mut e,
            PROCESS_TABLE,
            PROCESS_VSLOT_SET_EQUIPPED_CHANGE,
            Ret::default(),
        );
        record_slot(&mut e, PROCESS_TABLE, PROCESS_VSLOT_STATE, eax(7));
        let block = record(&mut e, ACTOR_SET_BLOCK, Ret::default());
        let notified = record(&mut e, ACTOR_EQUIP_NOTIFICATION, Ret::default());
        let steps: Vec<CallRecord> = [ACTOR_EQUIP_STEP_A, ACTOR_EQUIP_STEP_C, ACTOR_EQUIP_STEP_D]
            .iter()
            .map(|address| record(&mut e, *address, Ret::default()))
            .collect();
        let result = actor_un_equip_object(&mut e, actor, item, 1, Ptr::new(0x1234), 0, 0, 1);
        assert_eq!(result, 1);
        assert_eq!(faded.borrow().len(), 1);
        assert_eq!(faded.borrow()[0][1], 500);
        assert_eq!(released.borrow().len(), 1);
        assert_eq!(*reset.borrow(), vec![vec![process.addr(), 0]]);
        assert_eq!(*block.borrow(), vec![vec![actor.addr(), 0]]);
        assert_eq!(
            *notified.borrow(),
            vec![vec![actor.addr(), item.addr(), 0, 0]]
        );
        assert!(steps.iter().all(|step| step.borrow().len() == 1));
        assert_eq!(destroyed.borrow().len(), 1);
    }

    /// A worn entry whose first word points at a list whose first word is
    /// an extra data list, as `fn_0088db20` walks it.
    fn worn_entry(e: &mut Engine) -> (u32, u32) {
        let worn = e.mem.alloc(0x20);
        let worn_list = e.mem.alloc(0x20);
        let worn_extra = e.mem.alloc(0x20);
        e.mem.set_u32(worn, worn_list);
        e.mem.set_u32(worn_list, worn_extra);
        e.register(WORN_FIRST_WORD, |e, a| eax(e.mem.u32(a[0])));
        e.register(MATRIX_CONSTRUCT, |_, a| eax(a[0]));
        (worn, worn_extra)
    }

    /// An armor world where slot 3 of the new item is occupied by entry 5
    /// of the biped data (an object that casts to 0x4141).
    fn occupied_slot_world(e: &mut Engine) -> (Ptr<Actor>, Ptr, u32, u32) {
        let (actor, item) = equip_world(e, 0x18);
        let data = e.mem.alloc(0x400);
        slot(e, ACTOR_TABLE, VSLOT_GET_BIPED, eax(data));
        double(e, BIPED_MODEL_OF, eax(0x4040));
        e.register(BIPED_FILLS_SLOT, |_, a| {
            eax(((a[0] == 0x4040 || a[0] == 0x4141) && a[1] == 3) as u32)
        });
        let occupant = e.mem.alloc(0x40);
        e.mem.set_u8(occupant + 4, 0x18);
        e.mem.set_u32(data + 0x2c + 5 * 16, occupant);
        e.register(BIPED_SLOT_ENTRY, |e, a| {
            eax(e.mem.u32(a[0] + 0x2c + a[1] * 16))
        });
        e.register_double(RT_DYNAMIC_CAST_FN, move |_, a| {
            assert_eq!(a[1..], [0, CAST_SOURCE_TYPE, CAST_TARGET_TYPE, 0]);
            eax(if a[0] == occupant { 0x4141 } else { 0 })
        });
        double(e, EXTRA_LIST_GET_CONTAINER_CHANGES, eax(0x5050));
        (actor, item, data, occupant)
    }

    #[test]
    fn a_biped_world_removes_what_occupies_the_slots() {
        let mut e = engine3();
        let (actor, item, data, occupant) = occupied_slot_world(&mut e);
        let (worn, worn_extra) = worn_entry(&mut e);
        let wanted = record(&mut e, INVENTORY_WORN_ITEM, eax(worn));
        let removed = record(&mut e, INVENTORY_REMOVE, eax(1));
        let cleared = record(&mut e, BIPED_CLEAR_SLOT, Ret::default());
        let released = record(&mut e, ITEM_RELEASE, Ret::default());
        let applied = record(&mut e, INVENTORY_APPLY_EQUIP, Ret::default());
        assert_eq!(fn_0088db20(&mut e, actor, item, 1, Ptr::new(0), 0), 1);
        assert_eq!(wanted.borrow()[0][1..], [3, 0]);
        // The occupant is removed with the worn entry's extra data.
        assert_eq!(removed.borrow().len(), 1);
        assert_eq!(removed.borrow()[0][2], occupant);
        assert_eq!(removed.borrow()[0][5], worn_extra);
        assert_eq!(*cleared.borrow(), vec![vec![data, 3, 1, 0]]);
        assert_eq!(*released.borrow(), vec![vec![worn, 1]]);
        assert_eq!(applied.borrow().len(), 1);
    }

    #[test]
    fn a_worn_item_that_cannot_come_off_blocks_the_equip() {
        let mut e = engine3();
        let (actor, item, _, _) = occupied_slot_world(&mut e);
        let (worn, _) = worn_entry(&mut e);
        double(&mut e, INVENTORY_WORN_ITEM, eax(worn));
        double(&mut e, EXTRA_LIST_CAN_NOT_WEAR, eax(1));
        let removed = record(&mut e, INVENTORY_REMOVE, eax(1));
        let applied = record(&mut e, INVENTORY_APPLY_EQUIP, Ret::default());
        assert_eq!(fn_0088db20(&mut e, actor, item, 1, Ptr::new(0), 0), 0);
        assert!(removed.borrow().is_empty());
        assert!(applied.borrow().is_empty());
        // The combat object test overrides the block.
        double(&mut e, COMBAT_CONTROLLER_TEST, eax(1));
        assert_eq!(fn_0088db20(&mut e, actor, item, 1, Ptr::new(0), 0), 1);
        assert_eq!(applied.borrow().len(), 1);
    }

    #[test]
    fn the_player_uses_the_player_biped_data() {
        let mut e = engine3();
        let (actor, item) = equip_world(&mut e, 0x18);
        make_player(&mut e, actor);
        double(&mut e, BIPED_MODEL_OF, eax(0x4040));
        let asked = record(&mut e, PLAYER_GET_BIPED, eax(0));
        assert_eq!(fn_0088db20(&mut e, actor, item, 1, Ptr::new(0), 0), 1);
        assert_eq!(*asked.borrow(), vec![vec![actor.addr(), 0]]);
    }

    #[test]
    fn a_held_weapon_is_taken_off_before_the_weapon_step() {
        let mut e = engine3();
        let (actor, item) = equip_world(&mut e, 0x28);
        make_player(&mut e, actor);
        with_process(&mut e, actor);
        let (held, held_extra) = worn_entry(&mut e);
        let held_form = e.mem.alloc(0x40);
        e.mem.set_u8(held_form + 4, 0x28);
        e.mem.set_u32(held + 8, held_form);
        record_slot(&mut e, PROCESS_TABLE, PROCESS_VSLOT_ITEM, eax(held));
        double(&mut e, EXTRA_LIST_GET_CONTAINER_CHANGES, eax(0x5050));
        let removed = record(&mut e, INVENTORY_REMOVE, eax(1));
        record_slot(&mut e, ACTOR_TABLE, VSLOT_0X390, eax(0));
        let reload = record(&mut e, ACTOR_RELOAD_TARGETS, Ret::default());
        e.register(FORM_WEAPON_MOD_KIND, |_, a| eax((a[1] == 2) as u32 * 2));
        let slot_active = record(&mut e, EXTRA_LIST_WEAPON_MOD_SLOT_ACTIVE, eax(1));
        let step = record_slot(&mut e, ACTOR_TABLE, VSLOT_EQUIP_WEAPON_STEP, Ret::default());
        assert_eq!(fn_0088db20(&mut e, actor, item, 1, Ptr::new(0x1234), 0), 1);
        assert_eq!(removed.borrow().len(), 1);
        assert_eq!(removed.borrow()[0][2], held_form);
        assert_eq!(removed.borrow()[0][5], held_extra);
        assert_eq!(*reload.borrow(), vec![vec![actor.addr(), 0]]);
        assert_eq!(*slot_active.borrow(), vec![vec![0x1234, 2]]);
        assert_eq!(
            *step.borrow(),
            vec![vec![actor.addr(), item.addr(), 0, 1, 1]]
        );
    }

    #[test]
    fn an_empty_hand_tells_the_process_only_for_the_player() {
        let mut e = engine3();
        let (actor, item) = equip_world(&mut e, 0x28);
        with_process(&mut e, actor);
        record_slot(&mut e, PROCESS_TABLE, PROCESS_VSLOT_ITEM, eax(0));
        let told = record_slot(&mut e, PROCESS_TABLE, PROCESS_VSLOT_0X458, Ret::default());
        fn_0088db20(&mut e, actor, item, 1, Ptr::new(0), 0);
        assert!(told.borrow().is_empty());
        make_player(&mut e, actor);
        fn_0088db20(&mut e, actor, item, 1, Ptr::new(0), 0);
        assert_eq!(told.borrow().len(), 1);
        assert_eq!(told.borrow()[0][1..], [actor.addr(), 0]);
    }

    #[test]
    fn held_ammo_is_removed_when_other_ammo_is_equipped() {
        let mut e = engine3();
        let (actor, item) = equip_world(&mut e, 0x29);
        with_process(&mut e, actor);
        let (held, _) = worn_entry(&mut e);
        let held_form = e.mem.alloc(0x40);
        e.mem.set_u8(held_form + 4, 0x28);
        e.mem.set_u32(held + 8, held_form);
        record_slot(&mut e, PROCESS_TABLE, PROCESS_VSLOT_AMMO_ITEM, eax(held));
        double(&mut e, EXTRA_LIST_GET_CONTAINER_CHANGES, eax(0x5050));
        let removed = record(&mut e, INVENTORY_REMOVE, eax(1));
        fn_0088db20(&mut e, actor, item, 1, Ptr::new(0), 0);
        assert_eq!(removed.borrow().len(), 1);
        assert_eq!(removed.borrow()[0][2], held_form);
        // The same ammo is left alone.
        e.mem.set_u32(held + 8, item.addr());
        fn_0088db20(&mut e, actor, item, 1, Ptr::new(0), 0);
        assert_eq!(removed.borrow().len(), 1);
    }

    #[test]
    fn the_equip_finishes_with_the_npc_object_the_enchantment_and_the_refraction() {
        let mut e = engine3();
        let (actor, item) = equip_world(&mut e, 0x18);
        slot(&mut e, ACTOR_TABLE, VSLOT_TEST_0X218, eax(1));
        let data = e.mem.alloc(0x400);
        slot(&mut e, ACTOR_TABLE, VSLOT_GET_BIPED, eax(data));
        double(&mut e, SAVE_FORM_BUFFER_GET_FORM, eax(0x7070));
        let npc = record(&mut e, NPC_INIT_WORN_OBJECT, Ret::default());
        double(&mut e, FORM_ENCHANTING, eax(0x6000));
        let preload = record(&mut e, MAGIC_ITEM_PRELOAD, Ret::default());
        double(&mut e, REFRACTION_CONDITION, eax(1));
        double(&mut e, ACTOR_IS_REFRACTIVE, eax(1));
        let property = e.mem.alloc(0x20);
        e.mem.set_f32(property + 0xc, 0.5);
        double(&mut e, EXTRA_LIST_REFRACTION, eax(property));
        let refract = record_slot(&mut e, ACTOR_TABLE, VSLOT_0X384, Ret::default());
        assert_eq!(fn_0088db20(&mut e, actor, item, 1, Ptr::new(0), 0), 1);
        assert_eq!(
            *npc.borrow(),
            vec![vec![0x7070, actor.addr(), data, item.addr()]]
        );
        assert_eq!(*preload.borrow(), vec![vec![0x6018, 0]]);
        assert_eq!(
            *refract.borrow(),
            vec![vec![actor.addr(), 1, 0.5f32.to_bits()]]
        );
        // Weapons skip the NPC object.
        e.mem.set_u8(item.addr() + 4, 0x28);
        fn_0088db20(&mut e, actor, item, 1, Ptr::new(0), 0);
        assert_eq!(npc.borrow().len(), 1);
    }

    /// An actor and a furniture reference for the placement tests; every
    /// callee answers zero except the few the placement reads.
    fn chair_world(e: &mut Engine) -> (Ptr<Actor>, Ptr) {
        map_pages(e, &[0x0101_6000, 0x011d_d000]);
        let actor = actor2(e);
        e.mem.set_u32(PLAYER_CHARACTER, 0);
        let furniture = e.mem.alloc(0x100);
        slot(e, ACTOR_TABLE, VSLOT_GET_ANIMATION, eax(0x5000));
        for address in [
            PLAYER_STATE_TEST,
            PLAYER_STATE_SET,
            FURNITURE_CAN_SLEEP_ON,
            SET_MARKER_USED,
            CONTROLLER_SET_POSITION,
            ANIMATION_CLEAR_GROUP,
            ACTOR_ADD_TO_HEADING,
            MATRIX_CONSTRUCT,
            MATRIX_MAKE_ROTATION,
            FURNITURE_MARKER_TARGET_OFFSET,
            LOG_MESSAGE,
            REFERENCE_FULL_NAME,
            PLAYER_GET_ANIMATION,
            GET_FACE_ANIMATION_DATA,
            ANIMATION_STEP_0X4974A0,
            ANIMATION_BLEND_OUT,
            ANIMATION_PLAY_GROUP,
            SET_CHASE_BIP,
            SAVE_LOAD_BIT_2_TEST,
            PLAYER_SLEEPING_OR_RESTING,
            ACTOR_GET_ANIM_GROUP,
            ANIM_GROUP_GET_TYPE,
        ] {
            double(e, address, Ret::default());
        }
        e.mem.set_u32(SAVE_LOAD_GAME, 0x6666);
        double(e, GET_CHAR_CONTROLLER, eax(0x5500));
        double(e, SAVE_FORM_BUFFER_GET_FORM, eax(0x7070));
        double(e, FURNITURE_CAN_SIT_ON, eax(1));
        double(e, GET_SCALE, st0(1.5));
        double(e, MARKER_ANGLE, st0(0.5));
        double(e, MARKER_NUMBER, eax(3));
        double(e, FURNITURE_MARKER_HEIGHT, st0(0.25));
        e.register(MATRIX_TIMES_VECTOR, |e, a| {
            for word in 0..3 {
                let value = e.mem.u32(a[2] + 4 * word);
                e.mem.set_u32(a[1] + 4 * word, value);
            }
            eax(a[1])
        });
        e.register(VECTOR_PLUS_VECTOR, |_, a| eax(a[1]));
        (actor, Ptr::new(furniture))
    }

    #[test]
    fn chair_and_bed_quick_placement_needs_an_animation_and_a_form_that_allows_it() {
        let mut e = engine3();
        let (actor, furniture) = chair_world(&mut e);
        slot(&mut e, ACTOR_TABLE, VSLOT_GET_ANIMATION, eax(0));
        let marker = e.mem.alloc(0x40);
        assert_eq!(
            actor_put_actor_in_chair_bed_quick(&mut e, actor, furniture, Ptr::new(marker), 7, 0),
            0
        );
        slot(&mut e, ACTOR_TABLE, VSLOT_GET_ANIMATION, eax(0x5000));
        double(&mut e, GET_CHAR_CONTROLLER, eax(0));
        assert_eq!(
            actor_put_actor_in_chair_bed_quick(&mut e, actor, furniture, Ptr::new(marker), 7, 0),
            0
        );
        double(&mut e, GET_CHAR_CONTROLLER, eax(0x5500));
        double(&mut e, FURNITURE_CAN_SIT_ON, eax(0));
        double(&mut e, FURNITURE_CAN_SLEEP_ON, eax(0));
        assert_eq!(
            actor_put_actor_in_chair_bed_quick(&mut e, actor, furniture, Ptr::new(marker), 7, 0),
            0
        );
    }

    #[test]
    fn chair_placement_walks_the_actor_to_the_marker_and_sits_it() {
        let mut e = engine3();
        let (actor, furniture) = chair_world(&mut e);
        let process = with_process(&mut e, actor);
        let marker = e.mem.alloc(0x40);
        let rotated = record_slot(&mut e, ACTOR_TABLE, VSLOT_SET_ROTATION, Ret::default());
        let positioned = record_slot(&mut e, ACTOR_TABLE, VSLOT_SET_POSITION, Ret::default());
        let offsets = record(&mut e, FURNITURE_MARKER_TARGET_OFFSET, Ret::default());
        let used = record(&mut e, SET_MARKER_USED, Ret::default());
        let state = record_slot(
            &mut e,
            PROCESS_TABLE,
            PROCESS_VSLOT_SET_FURNITURE_STATE,
            Ret::default(),
        );
        record_slot(
            &mut e,
            PROCESS_TABLE,
            PROCESS_VSLOT_FURNITURE_ANIMATION_READY,
            eax(1),
        );
        let heading = record(&mut e, ACTOR_ADD_TO_HEADING, Ret::default());
        let placed = actor_put_actor_in_chair_bed_quick(
            &mut e,
            actor,
            furniture,
            Ptr::new(marker),
            0x1ff,
            0,
        );
        assert_eq!(placed, 1);
        assert_eq!(
            *rotated.borrow(),
            vec![vec![actor.addr(), 0.5f32.to_bits()]]
        );
        assert_eq!(offsets.borrow()[0][0], 0x7070);
        assert_eq!(offsets.borrow()[0][2..], [3, 1.5f32.to_bits()]);
        assert_eq!(*used.borrow(), vec![vec![furniture.addr(), 0x1ff, 1]]);
        assert_eq!(positioned.borrow().len(), 1);
        assert_eq!(
            *heading.borrow(),
            vec![vec![actor.addr(), 0.25f32.to_bits()]]
        );
        // Sitting: state 1, then 4, with the user byte only.
        assert_eq!(
            *state.borrow(),
            vec![
                vec![process.addr(), actor.addr(), 1, furniture.addr(), 0xff],
                vec![process.addr(), actor.addr(), 4, furniture.addr(), 0xff],
            ]
        );
    }

    #[test]
    fn bed_placement_goes_through_the_sleeping_states() {
        let mut e = engine3();
        let (actor, furniture) = chair_world(&mut e);
        with_process(&mut e, actor);
        let marker = e.mem.alloc(0x40);
        let state = record_slot(
            &mut e,
            PROCESS_TABLE,
            PROCESS_VSLOT_SET_FURNITURE_STATE,
            Ret::default(),
        );
        let steps = record_slot(&mut e, PROCESS_TABLE, PROCESS_VSLOT_0X284, Ret::default());
        record_slot(
            &mut e,
            PROCESS_TABLE,
            PROCESS_VSLOT_FURNITURE_ANIMATION_READY,
            eax(1),
        );
        let face = e.mem.alloc(0x100);
        e.mem.set_u32(face, OTHER_TABLE);
        double(&mut e, GET_FACE_ANIMATION_DATA, eax(face));
        let face_call = record_slot(&mut e, OTHER_TABLE, FACE_VSLOT_0XD8, Ret::default());
        assert_eq!(
            actor_put_actor_in_chair_bed_quick(&mut e, actor, furniture, Ptr::new(marker), 5, 1),
            1
        );
        let states: Vec<u32> = state.borrow().iter().map(|call| call[2]).collect();
        assert_eq!(states, vec![6, 9]);
        let arguments: Vec<u32> = steps.borrow().iter().map(|call| call[1]).collect();
        assert_eq!(arguments, vec![2, 1]);
        assert_eq!(*face_call.borrow(), vec![vec![face, 1, 0]]);
    }

    #[test]
    fn placement_without_an_animation_logs_and_resets_the_state() {
        let mut e = engine3();
        let (actor, furniture) = chair_world(&mut e);
        let process = with_process(&mut e, actor);
        let marker = e.mem.alloc(0x40);
        make_player(&mut e, actor);
        e.register(REFERENCE_FULL_NAME, |_, a| eax(a[0] + 1));
        let log = record(&mut e, LOG_MESSAGE, Ret::default());
        let state = record_slot(
            &mut e,
            PROCESS_TABLE,
            PROCESS_VSLOT_SET_FURNITURE_STATE,
            Ret::default(),
        );
        record_slot(
            &mut e,
            PROCESS_TABLE,
            PROCESS_VSLOT_FURNITURE_ANIMATION_READY,
            eax(0),
        );
        let player_state = record(&mut e, PLAYER_STATE_SET, Ret::default());
        assert_eq!(
            actor_put_actor_in_chair_bed_quick(&mut e, actor, furniture, Ptr::new(marker), 5, 0),
            0
        );
        assert_eq!(
            *log.borrow(),
            vec![vec![
                NO_ANIMATION_FORMAT,
                actor.addr() + 1,
                furniture.addr() + 1
            ]]
        );
        assert_eq!(
            state.borrow().last().unwrap(),
            &vec![process.addr(), actor.addr(), 0, 0, 0x7f]
        );
        // The player's state byte is set first and cleared at the end.
        let values: Vec<u32> = player_state.borrow().iter().map(|call| call[1]).collect();
        assert_eq!(values, vec![1, 0]);
    }

    #[test]
    fn leaving_furniture_needs_the_process_to_say_so() {
        let mut e = engine3();
        let (actor, _) = chair_world(&mut e);
        with_process(&mut e, actor);
        record_slot(&mut e, PROCESS_TABLE, PROCESS_VSLOT_ON_FURNITURE, eax(0));
        let reset = record_slot(&mut e, PROCESS_TABLE, PROCESS_VSLOT_0X84, Ret::default());
        actor_get_out_of_furniture_quick(&mut e, actor);
        assert!(reset.borrow().is_empty());
        record_slot(&mut e, PROCESS_TABLE, PROCESS_VSLOT_ON_FURNITURE, eax(1));
        let chase = record(&mut e, SET_CHASE_BIP, Ret::default());
        actor_get_out_of_furniture_quick(&mut e, actor);
        assert_eq!(reset.borrow().len(), 1);
        assert_eq!(chase.borrow()[0][1], 0);
    }

    #[test]
    fn leaving_furniture_blends_out_and_plays_the_group_for_others() {
        let mut e = engine3();
        let (actor, _) = chair_world(&mut e);
        let process = with_process(&mut e, actor);
        record_slot(&mut e, PROCESS_TABLE, PROCESS_VSLOT_ON_FURNITURE, eax(1));
        record_slot(&mut e, PROCESS_TABLE, PROCESS_VSLOT_ITEM, eax(0x4400));
        let reset = record_slot(&mut e, PROCESS_TABLE, PROCESS_VSLOT_0X71C, Ret::default());
        let blend = record(&mut e, ANIMATION_BLEND_OUT, Ret::default());
        let group = record(&mut e, ACTOR_GET_ANIM_GROUP, eax(0xffff_0005));
        let kind = record(&mut e, ANIM_GROUP_GET_TYPE, eax(0));
        let play = record(&mut e, ANIMATION_PLAY_GROUP, Ret::default());
        actor_get_out_of_furniture_quick(&mut e, actor);
        assert_eq!(*blend.borrow(), vec![vec![0x5000, 0x14, 0]]);
        assert_eq!(
            *group.borrow(),
            vec![vec![actor.addr(), 0, 0x4400, 1, 0x5000]]
        );
        assert_eq!(*kind.borrow(), vec![vec![5]]);
        assert_eq!(
            *play.borrow(),
            vec![vec![0x5000, 5, 1, 0xffff_ffff, 0xffff_ffff]]
        );
        assert_eq!(*reset.borrow(), vec![vec![process.addr(), 0]]);
        // Another group type is not played.
        double(&mut e, ANIM_GROUP_GET_TYPE, eax(1));
        actor_get_out_of_furniture_quick(&mut e, actor);
        assert_eq!(play.borrow().len(), 1);
    }

    #[test]
    fn the_player_leaving_furniture_clears_the_group_instead() {
        let mut e = engine3();
        let (actor, _) = chair_world(&mut e);
        with_process(&mut e, actor);
        make_player(&mut e, actor);
        record_slot(&mut e, PROCESS_TABLE, PROCESS_VSLOT_ON_FURNITURE, eax(1));
        double(&mut e, SAVE_LOAD_BIT_2_TEST, eax(1));
        double(&mut e, PLAYER_GET_ANIMATION, eax(0x5600));
        let step = record(&mut e, ANIMATION_STEP_0X4974A0, Ret::default());
        let cleared = record(&mut e, ANIMATION_CLEAR_GROUP, Ret::default());
        let play = record(&mut e, ANIMATION_PLAY_GROUP, Ret::default());
        let player_state = record(&mut e, PLAYER_STATE_SET, Ret::default());
        actor_get_out_of_furniture_quick(&mut e, actor);
        assert_eq!(*step.borrow(), vec![vec![0x5000]]);
        assert_eq!(cleared.borrow().len(), 2);
        assert_eq!(cleared.borrow()[1][0], 0x5600);
        assert!(play.borrow().is_empty());
        let values: Vec<u32> = player_state.borrow().iter().map(|call| call[1]).collect();
        assert_eq!(values, vec![1, 0]);
    }

    // ---- small accessors and inventory queries ----

    #[test]
    fn small_byte_and_word_accessors() {
        let mut e = engine3();
        let object = e.mem.alloc(0x200);
        e.mem.set_u8(object + 0x2c, 7);
        assert_eq!(fn_008905f0(&mut e, Ptr::new(object)), 7);
        assert_eq!(fn_00891170(&mut e, Ptr::new(object)), object + 0x20);
        assert!(!fn_00891b70(&mut e, Ptr::new(object)));
        e.mem.set_u8(object + 0x100, 0x0c);
        assert!(fn_00891b70(&mut e, Ptr::new(object)));
        e.mem.set_u8(object + 0x100, 0x04);
        assert!(!fn_00891b70(&mut e, Ptr::new(object)));
        map_pages(&mut e, &[0x011c_6000]);
        e.mem.set_u32(NODE_KEY_GLOBAL_011C61EC, 0xcafe);
        assert_eq!(fn_00891350(&mut e), 0xcafe);
    }

    #[test]
    fn flag_word_wrappers_use_the_word_at_0x10() {
        let mut e = engine3();
        let object = e.mem.alloc(0x40);
        let set = record(&mut e, FLAG_WORD_SET_OR_CLEAR, Ret::default());
        fn_00891130(&mut e, Ptr::new(object), 1);
        fn_00891130(&mut e, Ptr::new(object), 0);
        assert_eq!(
            *set.borrow(),
            vec![vec![object + 0x10, 1, 1], vec![object + 0x10, 1, 0]]
        );
        e.register(FLAG_WORD_AND, |e, a| eax(e.mem.u32(a[0]) & a[1]));
        assert!(!fn_00891150(&mut e, Ptr::new(object)));
        e.mem.set_u32(object + 0x10, 3);
        assert!(fn_00891150(&mut e, Ptr::new(object)));
    }

    #[test]
    fn vector_product_keeps_the_other_lanes_of_the_first_vector() {
        let mut e = engine3();
        e.register(MATRIX_CONSTRUCT, |_, a| eax(a[0]));
        e.register(VECTOR_ASSIGN, |e, a| {
            for word in 0..4 {
                let value = e.mem.u32(a[1] + 4 * word);
                e.mem.set_u32(a[0] + 4 * word, value);
            }
            eax(a[0])
        });
        let out = e.mem.alloc(0x10);
        let a = e.mem.alloc(0x10);
        let b = e.mem.alloc(0x10);
        for (i, value) in [2.0f32, 3.0, 4.0, 5.0].iter().enumerate() {
            e.mem.set_f32(a + 4 * i as u32, *value);
        }
        for (i, value) in [10.0f32, 1.0, 1.0, 1.0].iter().enumerate() {
            e.mem.set_f32(b + 4 * i as u32, *value);
        }
        assert_eq!(
            fn_00891060(&mut e, Ptr::new(out), Ptr::new(a), Ptr::new(b)),
            Ptr::new(out)
        );
        let lanes: Vec<f32> = (0..4).map(|i| e.mem.f32(out + 4 * i)).collect();
        assert_eq!(lanes, vec![20.0, 3.0, 4.0, 5.0]);
    }

    #[test]
    fn vector_subtraction_scales_by_the_first_component() {
        let mut e = engine3();
        e.register(VECTOR_SPLAT_FIRST, |e, a| {
            let first = e.mem.f32(a[0]);
            for word in 0..4 {
                e.mem.set_f32(a[1] + 4 * word, first);
            }
            eax(a[1])
        });
        let this = e.mem.alloc(0x10);
        let x = e.mem.alloc(0x10);
        let y = e.mem.alloc(0x10);
        e.mem.set_f32(x, 2.0);
        for (i, value) in [10.0f32, 20.0, 30.0, 40.0].iter().enumerate() {
            e.mem.set_f32(this + 4 * i as u32, *value);
        }
        for (i, value) in [1.0f32, 2.0, 3.0, 4.0].iter().enumerate() {
            e.mem.set_f32(y + 4 * i as u32, *value);
        }
        fn_008910c0(&mut e, Ptr::new(this), Ptr::new(x), Ptr::new(y));
        let lanes: Vec<f32> = (0..4).map(|i| e.mem.f32(this + 4 * i)).collect();
        assert_eq!(lanes, vec![8.0, 16.0, 24.0, 32.0]);
    }

    fn container_world(e: &mut Engine) -> Ptr<Actor> {
        let actor = actor2(e);
        e.register(GET_EXTRA_DATA_LIST_ADDRESS, |_, a| eax(a[0] + 0x44));
        double(e, EXTRA_LIST_GET_CONTAINER_CHANGES, eax(0x5050));
        double(e, BASE_FORM_OF, eax(0x7070));
        actor
    }

    #[test]
    fn inventory_queries_need_container_changes() {
        let mut e = engine3();
        let actor = container_world(&mut e);
        let worn = record(&mut e, INVENTORY_WORN_ITEM, eax(0x9090));
        let item = record(&mut e, INVENTORY_QUERY_ITEM, eax(0x9191));
        let base = record(&mut e, INVENTORY_QUERY_BASE, eax(0x9292));
        let food = record(&mut e, INVENTORY_GET_BEST_FOOD, eax(0x9393));
        assert_eq!(actor_get_armor_being_worn(&mut e, actor, 4), 0x9090);
        assert_eq!(*worn.borrow(), vec![vec![0x5050, 4, 1]]);
        assert_eq!(fn_00891c80(&mut e, actor, Ptr::new(0x3030)), 0x9191);
        assert_eq!(item.borrow()[0][0..2], [0x5050, 0x7070]);
        assert_eq!(item.borrow()[0][3..], [0x3030, 0]);
        assert_eq!(fn_00891ce0(&mut e, actor), 0x9292);
        assert_eq!(*base.borrow(), vec![vec![0x5050, 0x7070, 0]]);
        assert_eq!(actor_get_best_food_item(&mut e, actor), 0x9393);
        assert_eq!(*food.borrow(), vec![vec![0x5050]]);
        double(&mut e, EXTRA_LIST_GET_CONTAINER_CHANGES, eax(0));
        assert_eq!(actor_get_armor_being_worn(&mut e, actor, 4), 0);
        assert_eq!(fn_00891c80(&mut e, actor, Ptr::new(0x3030)), 0);
        assert_eq!(fn_00891ce0(&mut e, actor), 0);
        assert_eq!(actor_get_best_food_item(&mut e, actor), 0);
        assert_eq!(worn.borrow().len(), 1);
        assert_eq!(item.borrow().len(), 1);
    }

    #[test]
    fn weight_value_comes_from_the_flag_the_actor_or_the_item() {
        let mut e = engine3();
        let actor = container_world(&mut e);
        e.register(FTOL2, |_, a| {
            eax(f64::from_bits(a[0] as u64 | (a[1] as u64) << 32) as i32 as u32)
        });
        double(&mut e, WEIGHT_OVERRIDE_FLAG, eax(1));
        assert_eq!(fn_00891be0(&mut e, actor, Ptr::new(0)), 5);
        double(&mut e, WEIGHT_OVERRIDE_FLAG, eax(0));
        assert_eq!(fn_00891be0(&mut e, actor, Ptr::new(0)), 0);
        // An actor the virtual selects uses its base form's weight.
        slot(&mut e, ACTOR_TABLE, VSLOT_WEIGHT_SOURCE, eax(1));
        double(&mut e, SAVE_FORM_BUFFER_GET_FORM, eax(0));
        assert_eq!(fn_00891be0(&mut e, actor, Ptr::new(0)), 0);
        double(&mut e, SAVE_FORM_BUFFER_GET_FORM, eax(0x7171));
        let asked = record(&mut e, FORM_ACTOR_WEIGHT, st0(12.75));
        assert_eq!(fn_00891be0(&mut e, actor, Ptr::new(0)), 12);
        assert_eq!(*asked.borrow(), vec![vec![0x7171]]);
        // Otherwise the item's value goes through GetFormWeight.
        slot(&mut e, ACTOR_TABLE, VSLOT_WEIGHT_SOURCE, eax(0));
        e.mem.set_u32(PLAYER_CHARACTER, 0x4040);
        double(&mut e, PLAYER_WEIGHT_FLAG, eax(0xffff_ff01));
        e.register(ITEM_VALUE, |_, a| eax(a[0] + 1));
        let weight = record(&mut e, FORM_WEIGHT, st0(7.9));
        assert_eq!(fn_00891be0(&mut e, actor, Ptr::new(0x100)), 7);
        assert_eq!(*weight.borrow(), vec![vec![0x101, 1]]);
    }

    // ---- weapon condition and equipment damage ----

    const CONDITION_CALLEES: &[u32] = &[
        ACTOR_GET_CURRENT_WEAPON,
        DESTRUCTIBLE_OF_OBJECT,
        UPDATE_DAMAGE_STAGE_NODES,
        FIND_NODE_BY_VALUE,
        FIND_NODE_BY_NAME,
        PLAYER_ROOT_OBJECT,
    ];

    fn condition_world(e: &mut Engine) -> (Ptr<Actor>, Ptr<BaseProcess>) {
        map_pages(e, &[0x011c_6000]);
        let actor = actor2(e);
        let process = with_process(e, actor);
        for address in CONDITION_CALLEES.iter().copied() {
            double(e, address, Ret::default());
        }
        e.mem.set_u32(PLAYER_CHARACTER, 0);
        e.mem.set_u32(NODE_KEY_GLOBAL_011C61EC, 0x6161);
        double(e, ACTOR_GET_CURRENT_WEAPON, eax(0x7000));
        record_slot(e, PROCESS_TABLE, PROCESS_VSLOT_WEAPON_CONDITION, eax(5));
        record_slot(e, PROCESS_TABLE, PROCESS_VSLOT_LAST_CONDITION, eax(5));
        (actor, process)
    }

    #[test]
    fn weapon_condition_does_nothing_without_a_process_or_a_weapon() {
        let mut e = engine3();
        let (actor, _) = condition_world(&mut e);
        let set = record_slot(
            &mut e,
            PROCESS_TABLE,
            PROCESS_VSLOT_SET_LAST_CONDITION,
            Ret::default(),
        );
        double(&mut e, ACTOR_GET_CURRENT_WEAPON, eax(0));
        actor_update_weapon_condition_effects(&mut e, actor, 1);
        assert!(set.borrow().is_empty());
        e.set(actor, Actor::pCurrentProcess, Ptr::new(0));
        double(&mut e, ACTOR_GET_CURRENT_WEAPON, eax(0x7000));
        actor_update_weapon_condition_effects(&mut e, actor, 1);
        assert!(set.borrow().is_empty());
    }

    #[test]
    fn weapon_condition_stops_when_it_is_unchanged_unless_forced() {
        let mut e = engine3();
        let (actor, process) = condition_world(&mut e);
        let set = record_slot(
            &mut e,
            PROCESS_TABLE,
            PROCESS_VSLOT_SET_LAST_CONDITION,
            Ret::default(),
        );
        actor_update_weapon_condition_effects(&mut e, actor, 0);
        assert!(set.borrow().is_empty());
        actor_update_weapon_condition_effects(&mut e, actor, 1);
        assert_eq!(*set.borrow(), vec![vec![process.addr(), 5]]);
        record_slot(&mut e, PROCESS_TABLE, PROCESS_VSLOT_LAST_CONDITION, eax(4));
        actor_update_weapon_condition_effects(&mut e, actor, 0);
        assert_eq!(set.borrow().len(), 2);
    }

    #[test]
    fn weapon_condition_updates_the_process_object_and_the_root_nodes() {
        let mut e = engine3();
        let (actor, _) = condition_world(&mut e);
        record_slot(
            &mut e,
            PROCESS_TABLE,
            PROCESS_VSLOT_DAMAGE_STAGE_OBJECT,
            eax(0x7100),
        );
        double(&mut e, DESTRUCTIBLE_OF_OBJECT, eax(0x7200));
        slot(&mut e, ACTOR_TABLE, VSLOT_GET_3D, eax(0x7300));
        let stage = record(&mut e, UPDATE_DAMAGE_STAGE_NODES, Ret::default());
        let by_value = record(&mut e, FIND_NODE_BY_VALUE, eax(0x7400));
        let by_name = record(&mut e, FIND_NODE_BY_NAME, eax(0x7500));
        actor_update_weapon_condition_effects(&mut e, actor, 1);
        // The process object, then the node, then (the game's quirk) the
        // node again for the backpack.
        assert_eq!(
            *stage.borrow(),
            vec![vec![0x7200, 5], vec![0x7400, 5], vec![0x7400, 5]]
        );
        assert_eq!(*by_value.borrow(), vec![vec![0x7300, 0x6161]]);
        assert_eq!(*by_name.borrow(), vec![vec![0x7300, BACKPACK_NAME]]);
        // Without nodes or a process object there are no updates.
        double(&mut e, FIND_NODE_BY_VALUE, eax(0));
        double(&mut e, FIND_NODE_BY_NAME, eax(0));
        stage.borrow_mut().clear();
        record_slot(
            &mut e,
            PROCESS_TABLE,
            PROCESS_VSLOT_DAMAGE_STAGE_OBJECT,
            eax(0),
        );
        actor_update_weapon_condition_effects(&mut e, actor, 1);
        assert!(stage.borrow().is_empty());
    }

    #[test]
    fn the_player_updates_both_roots() {
        let mut e = engine3();
        let (actor, _) = condition_world(&mut e);
        e.mem.set_u32(PLAYER_CHARACTER, actor.addr());
        e.register(PLAYER_ROOT_OBJECT, |_, a| eax(0x8000 + a[1]));
        let by_value = record(&mut e, FIND_NODE_BY_VALUE, eax(0x7400));
        double(&mut e, FIND_NODE_BY_NAME, eax(0));
        actor_update_weapon_condition_effects(&mut e, actor, 1);
        let roots: Vec<u32> = by_value.borrow().iter().map(|call| call[0]).collect();
        assert_eq!(roots, vec![0x8000, 0x8001]);
    }

    const DAMAGE_CALLEES: &[u32] = &[
        OBJECT_TYPE,
        WEAPON_AMMO_FOR_ACTOR,
        AMMO_FLAG_TEST,
        AMMO_PROJECTILE,
        WEAPON_FLAG_20,
        HANDLE_ENTRY_POINT,
        DAMAGE_DEBUG_FLAG,
        PRINT_LINE,
        ARMOR_LOCATION_NAME,
        REFERENCE_FULL_NAME,
        AUDIO_INSTANCE,
        AUDIO_STOP_MOVING_SOUNDS,
        VATS_QUIT_PLAYBACK,
        BLEND_OBJECT_OF,
        ACTOR_IS_WEAPON_DRAWN,
        OBJECT_POSITION_0X8C,
        OBJECT_MATRIX,
        MATRIX_TO_EULER_ZXY,
        LAUNCH_PROJECTILE,
        ACTOR_AFTER_DROP,
        COMBAT_CONTROLLER_STEP,
        GET_FORCE_NEXT_UPDATE,
        TRIGGER_FOLLOWER_BARK,
        ANIMATION_BLEND_OUT,
        GET_PARENT_CELL,
        MATRIX_CONSTRUCT,
        ACTOR_EQUIP_STEP_A,
        ACTOR_GET_CURRENT_WEAPON,
        PLAYER_ROOT_OBJECT,
        FIND_NODE_BY_VALUE,
        FIND_NODE_BY_NAME,
        UPDATE_DAMAGE_STAGE_NODES,
    ];

    /// An actor with a worn entry for an item of type `item_type` (0x28 for a
    /// weapon, 0x18 for armor); the item change health is `health` (and the
    /// percentage) and `ITEM_SET_HEALTH` writes it back.
    fn damage_world(
        e: &mut Engine,
        item_type: u8,
        health: f32,
    ) -> (Ptr<Actor>, Ptr, u32, std::rc::Rc<std::cell::Cell<f32>>) {
        let (actor, item) = equip_world(e, item_type);
        map_pages(e, &[0x011c_6000, 0x011f_2000, 0x0104_f000, 0x011a_9000]);
        e.set_global(DOUBLE_ONE, 1.0f64);
        e.set_global(PERCENTAGE_LIMIT, 25.0f64);
        e.set_global(DAMAGE_MESSAGE_DURATION, 3.0f32);
        for address in DAMAGE_CALLEES.iter().copied() {
            double(e, address, Ret::default());
        }
        e.mem.set_u32(NODE_KEY_GLOBAL_011C61EC, 0x6161);
        let (entry, _) = worn_entry(e);
        e.mem.set_u32(entry + 8, item.addr());
        let current = std::rc::Rc::new(std::cell::Cell::new(health));
        let reader = current.clone();
        e.register_double(ITEM_GET_HEALTH, move |_, _| st0(reader.get()));
        let writer = current.clone();
        e.register_double(ITEM_SET_HEALTH, move |_, a| {
            writer.set(f32::from_bits(a[1]));
            Ret::default()
        });
        double(e, WEAPON_FLAG_NOT_80, eax(1));
        double(e, EXTRA_LIST_GET_CONTAINER_CHANGES, eax(0x5050));
        (actor, item, entry, current)
    }

    fn damage(e: &mut Engine, actor: Ptr<Actor>, entry: u32, amount: f32) -> u8 {
        actor_damage_equipment(e, actor, Ptr::new(entry), amount, 0)
    }

    #[test]
    fn damage_needs_an_entry_and_a_positive_amount() {
        let mut e = engine3();
        let (actor, _, entry, health) = damage_world(&mut e, 0x18, 50.0);
        assert_eq!(damage(&mut e, actor, 0, 5.0), 0);
        assert_eq!(damage(&mut e, actor, entry, 0.0), 0);
        assert_eq!(damage(&mut e, actor, entry, -1.0), 0);
        assert_eq!(health.get(), 50.0);
    }

    #[test]
    fn damage_is_ignored_while_the_acquired_object_is_of_two_types() {
        let mut e = engine3();
        let (actor, _, entry, health) = damage_world(&mut e, 0x18, 50.0);
        with_process(&mut e, actor);
        record_slot(&mut e, PROCESS_TABLE, PROCESS_VSLOT_0X27C, eax(0x7700));
        for kind in [8u32, 0x10] {
            double(&mut e, OBJECT_TYPE, eax(kind));
            assert_eq!(damage(&mut e, actor, entry, 5.0), 0);
        }
        assert_eq!(health.get(), 50.0);
        double(&mut e, OBJECT_TYPE, eax(3));
        damage(&mut e, actor, entry, 5.0);
        assert_eq!(health.get(), 45.0);
    }

    #[test]
    fn armor_loses_health_and_survives() {
        let mut e = engine3();
        let (actor, _, entry, health) = damage_world(&mut e, 0x18, 80.0);
        let set = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        e.register_double(ITEM_SET_HEALTH, {
            let set = set.clone();
            let health = health.clone();
            move |_, a| {
                set.borrow_mut().push(a.to_vec());
                health.set(f32::from_bits(a[1]));
                Ret::default()
            }
        });
        let entry_point = record(&mut e, HANDLE_ENTRY_POINT, Ret::default());
        let after = record_slot(
            &mut e,
            ACTOR_TABLE,
            VSLOT_AFTER_EQUIP_CHANGE,
            Ret::default(),
        );
        assert_eq!(damage(&mut e, actor, entry, 12.5), 0);
        assert_eq!(health.get(), 67.5);
        // SetItemHealth(entry; health, changes, extra data, 1)
        let call: Vec<u32> = set.borrow()[0].clone();
        assert_eq!(call[0..3], [entry, 67.5f32.to_bits(), 0x5050]);
        assert_eq!(call[4], 1);
        assert_eq!(entry_point.borrow()[0][0..2], [0x44, actor.addr()]);
        assert_eq!(after.borrow().len(), 1);
    }

    #[test]
    fn the_entry_point_can_change_the_damage() {
        let mut e = engine3();
        let (actor, _, entry, health) = damage_world(&mut e, 0x18, 80.0);
        e.register(HANDLE_ENTRY_POINT, |e, a| {
            let value = e.mem.f32(a[2]);
            e.mem.set_f32(a[2], value * 2.0);
            Ret::default()
        });
        damage(&mut e, actor, entry, 10.0);
        assert_eq!(health.get(), 60.0);
    }

    #[test]
    fn health_below_one_becomes_zero_and_destroys_armor() {
        let mut e = engine3();
        let (actor, _, entry, health) = damage_world(&mut e, 0x18, 10.0);
        assert_eq!(damage(&mut e, actor, entry, 9.5), 1);
        assert_eq!(health.get(), 0.0);
    }

    #[test]
    fn armor_below_the_warning_percentage_tells_the_player() {
        let mut e = engine3();
        let (actor, _, entry, _) = damage_world(&mut e, 0x18, 30.0);
        slot(&mut e, ACTOR_TABLE, VSLOT_ACTOR_FLAG_0X360, eax(1));
        let created = record(&mut e, CREATE_MESSAGE, eax(0x5151));
        let shown = record(&mut e, SHOW_MESSAGE, Ret::default());
        damage(&mut e, actor, entry, 10.0);
        assert_eq!(*created.borrow(), vec![vec![ARMOR_DAMAGED_MESSAGE]]);
        assert_eq!(
            *shown.borrow(),
            vec![vec![0x5151, 2, 0, DAMAGE_MESSAGE_TEXT, 3.0f32.to_bits(), 0]]
        );
        // Starting under the limit: no second message.
        damage(&mut e, actor, entry, 1.0);
        assert_eq!(created.borrow().len(), 1);
        // Not the player: no message.
        let (actor, _, entry, _) = damage_world(&mut e, 0x18, 30.0);
        slot(&mut e, ACTOR_TABLE, VSLOT_ACTOR_FLAG_0X360, eax(0));
        let created = record(&mut e, CREATE_MESSAGE, eax(0x5151));
        damage(&mut e, actor, entry, 10.0);
        assert!(created.borrow().is_empty());
    }

    #[test]
    fn armor_damage_writes_a_debug_line_when_asked() {
        let mut e = engine3();
        let (actor, item, entry, _) = damage_world(&mut e, 0x18, 50.0);
        double(&mut e, DAMAGE_DEBUG_FLAG, eax(1));
        e.register(ARMOR_LOCATION_NAME, |_, a| eax(a[0] + 1));
        e.register(REFERENCE_FULL_NAME, |_, a| eax(a[0] + 2));
        let printed = record(&mut e, PRINT_LINE, Ret::default());
        damage(&mut e, actor, entry, 8.0);
        let call = printed.borrow()[0].clone();
        assert_eq!(
            call[0..3],
            [DAMAGE_LINE_FORMAT, actor.addr() + 2, item.addr() + 0x31]
        );
        let doubles: Vec<f64> = (0..3)
            .map(|i| f64::from_bits(call[3 + 2 * i] as u64 | (call[4 + 2 * i] as u64) << 32))
            .collect();
        assert_eq!(doubles, vec![8.0, 42.0, 50.0]);
    }

    #[test]
    fn a_weapon_updates_its_condition_and_survives_with_health() {
        let mut e = engine3();
        let (actor, _, entry, health) = damage_world(&mut e, 0x28, 40.0);
        let step = record(&mut e, ACTOR_EQUIP_STEP_A, Ret::default());
        assert_eq!(damage(&mut e, actor, entry, 10.0), 0);
        assert_eq!(health.get(), 30.0);
        assert_eq!(step.borrow().len(), 1);
    }

    #[test]
    fn a_weapon_that_must_not_be_damaged_is_left_alone() {
        let mut e = engine3();
        let (actor, _, entry, health) = damage_world(&mut e, 0x28, 40.0);
        double(&mut e, WEAPON_FLAG_NOT_80, eax(0));
        assert_eq!(damage(&mut e, actor, entry, 10.0), 0);
        assert_eq!(health.get(), 40.0);
    }

    #[test]
    fn the_player_weapon_with_flagged_ammo_is_not_damaged() {
        let mut e = engine3();
        let (actor, _, entry, health) = damage_world(&mut e, 0x28, 40.0);
        slot(&mut e, ACTOR_TABLE, VSLOT_ACTOR_FLAG_0X360, eax(1));
        double(&mut e, FORM_FLAG_TEST_4C0BF0, eax(1));
        double(&mut e, WEAPON_AMMO_FOR_ACTOR, eax(0x4400));
        double(&mut e, AMMO_FLAG_TEST, eax(1));
        assert_eq!(damage(&mut e, actor, entry, 10.0), 0);
        assert_eq!(health.get(), 40.0);
    }

    #[test]
    fn a_weapon_without_a_drawn_state_that_is_gone_ends_with_one() {
        let mut e = engine3();
        let (actor, _, entry, health) = damage_world(&mut e, 0x28, 5.0);
        with_process(&mut e, actor);
        let position = e.mem.alloc(0x10);
        slot(&mut e, ACTOR_TABLE, VSLOT_POSITION_0X1F4, eax(position));
        let result = damage(&mut e, actor, entry, 10.0);
        assert_eq!(health.get(), 0.0);
        assert_eq!(result, 1);
    }

    #[test]
    fn a_destroyed_weapon_ends_the_vats_playback() {
        let mut e = engine3();
        let (actor, _, entry, _) = damage_world(&mut e, 0x28, 5.0);
        slot(&mut e, ACTOR_TABLE, VSLOT_ACTOR_FLAG_0X360, eax(1));
        e.register(ITEM_VALUE, |e, a| {
            eax(if a[0] == VATS_STATE_OBJECT {
                4
            } else {
                e.mem.u32(a[0] + 8)
            })
        });
        let quit = record(&mut e, VATS_QUIT_PLAYBACK, Ret::default());
        let stopped = record(&mut e, AUDIO_STOP_MOVING_SOUNDS, Ret::default());
        assert_eq!(damage(&mut e, actor, entry, 10.0), 1);
        assert_eq!(quit.borrow().len(), 1);
        assert_eq!(quit.borrow()[0][0], VATS_STATE_OBJECT);
        assert_eq!(stopped.borrow().len(), 1);
    }

    #[test]
    fn a_destroyed_weapon_of_the_player_is_announced_and_unequipped() {
        let mut e = engine3();
        let (actor, _, entry, _) = damage_world(&mut e, 0x28, 5.0);
        make_player(&mut e, actor);
        slot(&mut e, ACTOR_TABLE, VSLOT_ACTOR_FLAG_0X360, eax(1));
        let created = record(&mut e, CREATE_MESSAGE, eax(0x5151));
        let shown = record(&mut e, SHOW_MESSAGE, Ret::default());
        let stopped = record(&mut e, AUDIO_STOP_MOVING_SOUNDS, Ret::default());
        let removed = record(&mut e, INVENTORY_REMOVE, eax(1));
        e.register(PLAYER_ROOT_OBJECT, |_, a| eax(0x8000 + a[1]));
        assert_eq!(damage(&mut e, actor, entry, 10.0), 1);
        assert_eq!(*created.borrow(), vec![vec![WEAPON_DESTROYED_MESSAGE]]);
        assert_eq!(
            *shown.borrow(),
            vec![vec![0x5151, 2, 0, DAMAGE_MESSAGE_TEXT, 3.0f32.to_bits(), 0]]
        );
        assert_eq!(stopped.borrow().len(), 2);
        assert_eq!(stopped.borrow()[0][1..], [0x8001, 0, 0]);
        assert_eq!(removed.borrow().len(), 1);
    }

    #[test]
    fn a_destroyed_weapon_that_stays_is_dropped_when_drawn() {
        let mut e = engine3();
        let (actor, _, entry, _) = damage_world(&mut e, 0x28, 5.0);
        with_process(&mut e, actor);
        double(&mut e, ACTOR_IS_WEAPON_DRAWN, eax(1));
        let source = e.mem.alloc(0x100);
        let position = e.mem.alloc(0x10);
        e.mem.set_f32(position, 1.0);
        e.mem.set_f32(position + 8, 3.0);
        let matrix = e.mem.alloc(0x40);
        e.mem.set_f32(matrix, 2.0);
        slot(&mut e, ACTOR_TABLE, VSLOT_GET_BIPED, eax(0x9900));
        record_slot(
            &mut e,
            PROCESS_TABLE,
            PROCESS_VSLOT_DROP_SOURCE_WEAPON,
            eax(source),
        );
        double(&mut e, OBJECT_POSITION_0X8C, eax(position));
        double(&mut e, OBJECT_MATRIX, eax(matrix));
        let euler = record(&mut e, MATRIX_TO_EULER_ZXY, Ret::default());
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let sink = seen.clone();
        let target = slot_target(ACTOR_TABLE, VSLOT_PLACE_DROPPED_ITEM);
        e.mem
            .set_u32(ACTOR_TABLE + VSLOT_PLACE_DROPPED_ITEM, target);
        e.register_double(target, move |e, a| {
            sink.borrow_mut()
                .push((a.to_vec(), e.mem.f32(a[4]), e.mem.f32(a[4] + 8)));
            eax(0x6600)
        });
        let after = record(&mut e, ACTOR_AFTER_DROP, Ret::default());
        slot(
            &mut e,
            ACTOR_TABLE,
            VSLOT_GET_COMBAT_CONTROLLER,
            eax(0x4400),
        );
        let controller = record(&mut e, COMBAT_CONTROLLER_STEP, Ret::default());
        double(&mut e, GET_FORCE_NEXT_UPDATE, eax(1));
        let bark = record(&mut e, TRIGGER_FOLLOWER_BARK, Ret::default());
        let tail = record_slot(&mut e, PROCESS_TABLE, PROCESS_VSLOT_0X450, Ret::default());
        slot(&mut e, ACTOR_TABLE, VSLOT_GET_ANIMATION, eax(0x5000));
        let blend = record(&mut e, ANIMATION_BLEND_OUT, Ret::default());
        assert_eq!(damage(&mut e, actor, entry, 10.0), 1);
        assert_eq!(euler.borrow().len(), 1);
        // vcall 0x3cc: (actor, form, extra, 1, position, euler)
        let (call, x, z) = seen.borrow()[0].clone();
        assert_eq!(call[3], 1);
        assert_eq!((x, z), (1.0, 3.0));
        assert_eq!(*after.borrow(), vec![vec![actor.addr(), 0x6600]]);
        assert_eq!(*controller.borrow(), vec![vec![0x4400, 0]]);
        assert_eq!(*bark.borrow(), vec![vec![actor.addr(), 8]]);
        assert_eq!(tail.borrow().len(), 1);
        assert_eq!(
            *blend.borrow(),
            vec![vec![0x5000, 4, 0], vec![0x5000, 2, 0]]
        );
    }

    #[test]
    fn an_explosive_weapon_launches_its_projectile() {
        let mut e = engine3();
        let (actor, _, entry, health) = damage_world(&mut e, 0x28, 5.0);
        with_process(&mut e, actor);
        double(&mut e, FORM_FLAG_TEST_4C0BF0, eax(1));
        double(&mut e, WEAPON_AMMO_FOR_ACTOR, eax(0x4400));
        double(&mut e, AMMO_FLAG_TEST, eax(1));
        double(&mut e, AMMO_PROJECTILE, eax(0x4500));
        double(&mut e, GET_PARENT_CELL, eax(0xc000));
        slot(&mut e, ACTOR_TABLE, VSLOT_GET_BIPED, eax(0x9900));
        record_slot(
            &mut e,
            PROCESS_TABLE,
            PROCESS_VSLOT_DROP_SOURCE_WEAPON,
            eax(0),
        );
        let position = e.mem.alloc(0x10);
        e.mem.set_f32(position + 4, 7.0);
        slot(&mut e, ACTOR_TABLE, VSLOT_POSITION_0X1F4, eax(position));
        e.mem.set_f32(IDENTITY_MATRIX_ADDRESS, 1.0);
        e.mem.set_f32(IDENTITY_MATRIX_ADDRESS + 16, 1.0);
        e.mem.set_f32(IDENTITY_MATRIX_ADDRESS + 32, 1.0);
        let used = record_slot(&mut e, ACTOR_TABLE, VSLOT_USE_ITEM_0X17C, Ret::default());
        let launched = record(&mut e, LAUNCH_PROJECTILE, Ret::default());
        let set = record(&mut e, ITEM_SET_HEALTH, Ret::default());
        assert_eq!(damage(&mut e, actor, entry, 10.0), 1);
        assert!(set.borrow().is_empty());
        assert_eq!(health.get(), 5.0);
        assert_eq!(used.borrow()[0][3..], [1, 0, 0, 0, 0, 0, 1, 0]);
        let call = launched.borrow()[0].clone();
        assert_eq!(call.len(), 16);
        assert_eq!(call[0..4], [0x4500, 0, 0, 0xc000]);
        assert_eq!(f32::from_bits(call[5]), 7.0);
        assert_eq!(f32::from_bits(call[7]), 1.0);
        assert_eq!(f32::from_bits(call[11]), 1.0);
        assert_eq!(f32::from_bits(call[15]), 1.0);
    }

    // ---- impact sounds ----

    /// A world for `fn_0088e1e0`: the listener at the origin, a sound set
    /// whose entry for material `m` has the sounds `0x7100 + m` and
    /// `0x7200 + m`, records that let sounds be heard up to 300 units.
    fn impact_world(e: &mut Engine, secondary: bool) -> (Ptr<Actor>, u32) {
        map_pages(e, &[0x011f_4000, 0x011c_a000]);
        new_table(e, FORM_TABLE);
        let actor = actor2(e);
        e.mem.set_u32(PLAYER_CHARACTER, 0x4040);
        for word in 0..3 {
            e.mem.set_u32(SCALE_VECTOR + 4 * word, 0);
        }
        e.mem.set_u32(IMPACT_DEFAULT_SOURCE, 0x7400);
        double(e, IMPACT_SECONDARY_FLAG, eax(secondary as u32));
        let base = e.mem.alloc(0x100);
        e.mem.set_u32(base + 0x30, FORM_TABLE);
        slot_at(e, FORM_TABLE, BASE_FORM_PART_VSLOT_MATERIAL, eax(7));
        double(e, BASE_FORM_OF, eax(base));
        double(e, SOUND_HANDLE_INIT, Ret::default());
        double(e, EMPTY_FUNCTION, Ret::default());
        double(e, LISTENER_OBJECT, eax(0x6000));
        let listener_position = e.mem.alloc(0x10);
        double(e, OBJECT_POSITION_0X8C, eax(listener_position));
        e.register(VECTOR_MINUS_VECTOR, |e, a| {
            for word in 0..3 {
                let value = e.mem.f32(a[0] + 4 * word) - e.mem.f32(a[2] + 4 * word);
                e.mem.set_f32(a[1] + 4 * word, value);
            }
            eax(a[1])
        });
        e.register(VECTOR_LENGTH, |e, a| {
            let sum: f32 = (0..3).map(|i| e.mem.f32(a[0] + 4 * i).powi(2)).sum();
            st0(sum.sqrt())
        });
        double(e, IMPACT_SOUND_SOURCE, eax(0x7500));
        e.register(SOUND_SET_ENTRY, |_, a| eax(a[1]));
        e.register(SOUND_ENTRY_FIRST, |_, a| eax(0x7100 + a[0]));
        e.register(SOUND_ENTRY_SECOND, |_, a| eax(0x7200 + a[0]));
        e.register(SOUND_RECORD, |e, a| {
            e.mem.set_u8(a[1] + 1, 3);
            eax(a[1])
        });
        e.register(WEAPON_SOUND_NAME, |_, a| eax(a[0] + 0x10000));
        double(e, AUDIO_INSTANCE, eax(0x6100));
        e.register(AUDIO_GET_SOUND_HANDLE_BY_FILENAME, |_, a| eax(a[2] + 1));
        for address in [
            SOUND_HANDLE_ASSIGN,
            SOUND_HANDLE_SET_POSITION,
            SOUND_HANDLE_PLAY,
        ] {
            double(e, address, Ret::default());
        }
        let hit = e.mem.alloc(0x80);
        e.mem.set_u32(hit + 4, 0x5050);
        e.mem.set_u32(hit + 0x30, 0x7400);
        e.mem.set_f32(hit + 0x38, 100.0);
        (actor, hit)
    }

    #[test]
    fn impact_sounds_play_both_sounds_of_the_entry_within_range() {
        let mut e = engine3();
        let (actor, hit) = impact_world(&mut e, false);
        let entries = record(&mut e, SOUND_SET_ENTRY, eax(7));
        let sounds = record(&mut e, AUDIO_GET_SOUND_HANDLE_BY_FILENAME, eax(0x5555));
        let assigned = record(&mut e, SOUND_HANDLE_ASSIGN, Ret::default());
        let placed = record(&mut e, SOUND_HANDLE_SET_POSITION, Ret::default());
        let played = record(&mut e, SOUND_HANDLE_PLAY, Ret::default());
        fn_0088e1e0(&mut e, actor, Ptr::new(hit));
        // The sound set object comes from the explicit source, the entry
        // from the material 7 the base form part reports.
        assert_eq!(*entries.borrow(), vec![vec![0x7500, 7]]);
        let files: Vec<(u32, u32, u32)> = sounds
            .borrow()
            .iter()
            .map(|call| (call[2], call[3], call[4]))
            .collect();
        assert_eq!(
            files,
            vec![(0x17107, 0x4102, 0x7107), (0x17207, 0x4102, 0x7207)]
        );
        assert_eq!(assigned.borrow().len(), 2);
        assert_eq!(assigned.borrow()[0][1], 0x5555);
        // The sounds sit at the hit position: 100 units away is within 300.
        assert_eq!(placed.borrow().len(), 2);
        assert_eq!(placed.borrow()[0][1..], [100.0f32.to_bits(), 0, 0]);
        assert_eq!(played.borrow().len(), 2);
        assert_eq!(played.borrow()[0][1], 0);
    }

    #[test]
    fn impact_sounds_beyond_their_range_are_silent() {
        let mut e = engine3();
        let (actor, hit) = impact_world(&mut e, false);
        e.mem.set_f32(hit + 0x38, 300.0);
        let played = record(&mut e, SOUND_HANDLE_PLAY, Ret::default());
        fn_0088e1e0(&mut e, actor, Ptr::new(hit));
        assert!(played.borrow().is_empty());
    }

    #[test]
    fn impacts_by_the_player_use_the_player_position_and_distance_zero() {
        let mut e = engine3();
        let (actor, hit) = impact_world(&mut e, false);
        let player_position = e.mem.alloc(0x10);
        e.mem.set_f32(player_position, 5.0);
        let target = slot_target(ACTOR_TABLE, VSLOT_POSITION_0X1F4);
        let player_object = e.mem.alloc(0x10);
        e.mem.set_u32(player_object, ACTOR_TABLE);
        e.mem.set_u32(ACTOR_TABLE + VSLOT_POSITION_0X1F4, target);
        e.register_double(target, move |_, _| eax(player_position));
        e.mem.set_u32(PLAYER_CHARACTER, player_object);
        e.mem.set_u32(hit + 4, player_object);
        // A position 1000 units away would be out of range for others.
        e.mem.set_f32(hit + 0x38, 1000.0);
        let placed = record(&mut e, SOUND_HANDLE_SET_POSITION, Ret::default());
        fn_0088e1e0(&mut e, actor, Ptr::new(hit));
        assert_eq!(placed.borrow().len(), 2);
        assert_eq!(placed.borrow()[0][1], 5.0f32.to_bits());
    }

    #[test]
    fn creature_targets_add_the_creature_sound() {
        let mut e = engine3();
        let (actor, hit) = impact_world(&mut e, false);
        e.mem.set_u32(hit + 0x30, 0);
        let target = e.mem.alloc(0x100);
        e.mem.set_u32(target, FORM_TABLE);
        slot_at(&mut e, FORM_TABLE, VSLOT_TARGET_TEST_0X21C, eax(1));
        e.mem.set_u32(hit, target);
        double(&mut e, SAVE_FORM_BUFFER_GET_FORM, eax(0x7600));
        let picked = record(&mut e, PICK_CREATURE_SOUND, eax(0x7300));
        double(&mut e, CREATURE_SOUND_SET, eax(0x7500));
        let sounds = record(&mut e, AUDIO_GET_SOUND_HANDLE_BY_FILENAME, eax(0x5555));
        fn_0088e1e0(&mut e, actor, Ptr::new(hit));
        assert_eq!(*picked.borrow(), vec![vec![0x7600, 9]]);
        let played: Vec<u32> = sounds.borrow().iter().map(|call| call[4]).collect();
        assert_eq!(played, vec![0x7107, 0x7207, 0x7300]);
    }

    #[test]
    fn non_creature_targets_use_the_default_source() {
        let mut e = engine3();
        let (actor, hit) = impact_world(&mut e, false);
        e.mem.set_u32(hit + 0x30, 0);
        let target = e.mem.alloc(0x100);
        e.mem.set_u32(target, FORM_TABLE);
        e.mem.set_u32(hit, target);
        let sources = record(&mut e, IMPACT_SOUND_SOURCE, eax(0x7500));
        fn_0088e1e0(&mut e, actor, Ptr::new(hit));
        assert_eq!(*sources.borrow(), vec![vec![0x7400]]);
    }

    #[test]
    fn impacts_without_a_source_or_target_play_nothing() {
        let mut e = engine3();
        let (actor, hit) = impact_world(&mut e, false);
        e.mem.set_u32(hit + 0x30, 0);
        let played = record(&mut e, SOUND_HANDLE_PLAY, Ret::default());
        let destroyed = record(&mut e, EMPTY_FUNCTION, Ret::default());
        fn_0088e1e0(&mut e, actor, Ptr::new(hit));
        assert!(played.borrow().is_empty());
        assert_eq!(destroyed.borrow().len(), 1);
    }

    #[test]
    fn the_secondary_flag_adds_the_material_4_sounds() {
        let mut e = engine3();
        let (actor, hit) = impact_world(&mut e, true);
        let sounds = record(&mut e, AUDIO_GET_SOUND_HANDLE_BY_FILENAME, eax(0x5555));
        fn_0088e1e0(&mut e, actor, Ptr::new(hit));
        let played: Vec<u32> = sounds.borrow().iter().map(|call| call[4]).collect();
        assert_eq!(played, vec![0x7107, 0x7207, 0x7104, 0x7204]);
    }

    // ---- hit effects ----

    const HIT_CALLEES: &[u32] = &[
        TIMER_SCOPE_CONSTRUCT,
        TIMER_SCOPE_DESTROY,
        HIT_FLAG_MELEE,
        IMPACT_SECONDARY_FLAG,
        HIT_EFFECTS_ALLOWED,
        DAMAGE_POINTS,
        PLAYER_STATISTIC_STEP,
        LISTENER_OBJECT,
        OBJECT_POSITION_0X8C,
        WEIGHT_OVERRIDE_FLAG,
        COMBATANT_COUNT,
        GLOBAL_WORD_ADDRESS,
        SETTING_VALUE_ADDRESS,
        HIT_EFFECT_VISIBLE,
        RANDOM_ANGLE,
        RANDOM_FLOAT,
        RANDOM_BYTE_SOURCE,
        FTOL2,
        BASE_FORM_OF,
        ACTOR_GET_CURRENT_WEAPON,
        IMPACT_SOUND_SOURCE,
        SOUND_SET_ENTRY,
        SOUND_SET_ENTRY_SOUND,
        MATRIX_CONSTRUCT,
        VECTOR_SCALE_INTO,
        VECTOR_NORMALIZE,
        DEFAULT_SOUND_SET_SOURCE,
        DECAL_SCALE_MIN,
        DECAL_SCALE_MAX,
        DECAL_CREATE,
        DECAL_SET_FLAG,
        GET_PARENT_CELL,
        CELL_OBJECT_OF,
        DECAL_CASTER_ADD,
        COLLECTOR_CONSTRUCT,
        COLLECTOR_DESTROY,
        DECAL_COLLECT,
        COLLECTOR_LIST,
        LIST_ENTRY_COUNT,
        LIST_ENTRY,
        HIT_RECORD_CONSTRUCT,
        COLLIDABLE_3D_OBJECT,
        FIND_REFERENCE_FOR_3D,
        ACTOR_BODY_PART_NUMBER,
        CELL_ADD_HIT,
        OBJECT_COLLISION_RECORD,
        OBJECT_PLUS_0X10,
        AV_OBJECT_FOR_COLLIDABLE,
        VECTOR_NEGATE_INTO,
        PLAY_HIT_SPURT,
        BODY_PART_OF,
        BODY_PART_SOUND_SOURCE,
        HIT_SECOND_DISTANCE,
        PICK_CONSTRUCT,
        STORE_WORD,
        FILTER_SET_LAYER,
        FILTER_GET_GROUP,
        PICK_SET_FILTER,
        PICK_SET_START,
        PICK_SET_END,
        RAY_COLLECTOR_CONSTRUCT,
        RAY_COLLECTOR_DESTROY,
        PICK_SET_COLLECTOR,
        PICK_RESULT_HOLDER,
        PICK_RESULT_RECORD,
        PICK_RECORD_NORMAL,
        NODE_FILTER_WORD,
        TES_OBJECT_AT_POINT,
        OBJECT_PARENT,
        REFERENCE_FORM_TEST,
        TES_CELL_AT_POINT,
        DECAL_COLOR_WORD,
        DECAL_FIELD_BYTE_A,
        DECAL_FIELD_FLOAT_A,
        DECAL_FIELD_BYTE_B,
        DECAL_FIELD_BYTE_C,
        DECAL_FIELD_BYTE_D,
        DECAL_FIELD_FLOAT_B,
        DECAL_FIELD_FLOAT_C,
        DECAL_FIELD_FLOAT_D,
    ];

    /// A hit world: the actor is the target of a hit at (100, 0, 0) from
    /// another party, the listener sits at the origin; sound set entries and
    /// decals are doubles that answer fixed values.
    fn hit_world(e: &mut Engine) -> (Ptr<Actor>, u32) {
        map_pages(
            e,
            &[
                0x011f_1000,
                0x011d_f000,
                0x011c_e000,
                0x0101_e000,
                0x0101_6000,
                0x0107_5000,
                0x0103_0000,
                0x0102_3000,
                0x011d_e000,
                0x0101_1000,
            ],
        );
        new_table(e, FORM_TABLE);
        let actor = actor2(e);
        e.mem.set_u32(PLAYER_CHARACTER, 0x4040);
        e.mem.set_u32(TES_POINTER, 0x7000);
        e.mem.set_u32(COMBAT_MANAGER_POINTER, 0x7100);
        for address in HIT_CALLEES.iter().copied() {
            double(e, address, Ret::default());
        }
        e.register(HIT_EFFECTS_ALLOWED, |_, _| eax(1));
        e.register(GLOBAL_WORD_ADDRESS, |_, _| eax(0x011d_f790));
        e.register(SETTING_VALUE_ADDRESS, |_, _| eax(0x011c_e270));
        e.mem.set_f32(0x011c_e270, 50.0);
        e.register(LISTENER_OBJECT, |_, _| eax(0x6000));
        let listener_position = e.mem.alloc(0x10);
        double(e, OBJECT_POSITION_0X8C, eax(listener_position));
        e.register(VECTOR_MINUS_VECTOR, |e, a| {
            for word in 0..3 {
                let value = e.mem.f32(a[0] + 4 * word) - e.mem.f32(a[2] + 4 * word);
                e.mem.set_f32(a[1] + 4 * word, value);
            }
            eax(a[1])
        });
        e.register(VECTOR_PLUS_VECTOR, |e, a| {
            for word in 0..3 {
                let value = e.mem.f32(a[0] + 4 * word) + e.mem.f32(a[2] + 4 * word);
                e.mem.set_f32(a[1] + 4 * word, value);
            }
            eax(a[1])
        });
        e.register(VECTOR_LENGTH, |e, a| {
            let sum: f32 = (0..3).map(|i| e.mem.f32(a[0] + 4 * i).powi(2)).sum();
            st0(sum.sqrt())
        });
        e.register(VECTOR_SCALE_INTO, |e, a| {
            let factor = f32::from_bits(a[1]);
            for word in 0..3 {
                let value = factor * e.mem.f32(a[2] + 4 * word);
                e.mem.set_f32(a[0] + 4 * word, value);
            }
            eax(a[0])
        });
        e.register(MATRIX_CONSTRUCT, |_, a| eax(a[0]));
        e.register(RANDOM_FLOAT, |_, a| st0(f32::from_bits(a[0])));
        e.register(RANDOM_BYTE_SOURCE, |_, a| st0(f32::from_bits(a[0])));
        e.register(FTOL2, |_, _| eax(77));
        e.register(STORE_WORD, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            eax(a[0])
        });
        e.register(FILTER_GET_GROUP, |e, a| eax(e.mem.u32(a[0]) >> 16));
        e.register(DECAL_CREATE, |_, _| eax(0x7300));
        e.register(CELL_OBJECT_OF, |_, a| eax(a[0] + 0x1000));
        e.register(GET_PARENT_CELL, |_, _| eax(0xc000));
        e.register(OBJECT_PLUS_0X10, |_, a| eax(a[0] + 0x10));
        double(e, SOUND_HANDLE_INIT, Ret::default());
        slot_at(e, FORM_TABLE, BASE_FORM_PART_VSLOT_MATERIAL, eax(7));
        slot_at(e, FORM_TABLE, BASE_FORM_VSLOT_0X180, eax(0));
        let base = e.mem.alloc(0x100);
        e.mem.set_u32(base, FORM_TABLE);
        e.mem.set_u32(base + 0x30, FORM_TABLE);
        double(e, BASE_FORM_OF, eax(base));
        e.mem.set_f32(HIT_OFFSET_MIN, 0.5);
        e.mem.set_f32(HIT_OFFSET_MAX, 0.5);
        e.mem.set_f32(HIT_DIRECTION_SCALE, 2.0);
        e.mem.set_f32(HIT_RANDOM_MAX, 9.0);
        e.set_global(DOUBLE_ZERO, 0.0f64);
        e.set_global(DOUBLE_HALF, 0.5f64);
        e.set_global(COLOR_BYTE_DIVISOR, 255.0f64);
        e.set_global(ARMOR_HIT_SCALE_FACTOR, 2.0f64);
        e.mem.set_f32(HIT_RECORD_FLOAT, 4.5);
        e.mem.set_f32(HIT_REACH_FOR_TESTS, 3.0);
        let hit = e.mem.alloc(0x100);
        e.mem.set_f32(hit + 0x38, 100.0);
        e.mem.set_f32(hit + 0x44, 1.0);
        e.mem.set_u32(hit + 0x10, 3);
        (actor, hit)
    }

    /// Where the tests keep the ray reach constant (the game's `010231a0`).
    const HIT_REACH_FOR_TESTS: u32 = HIT_RAY_REACH;

    /// Makes the target actor carry a weapon whose sound set has the entry
    /// `0x7500` with a sound `0x7600`; gives the impact source a decal
    /// definition entry with a name (so the spurt plays) and a root object.
    fn armed_target(e: &mut Engine, hit: u32) -> u32 {
        let target = e.mem.alloc(0x100);
        e.mem.set_u32(target, FORM_TABLE);
        e.mem.set_u32(target + 0xa4, OTHER_TABLE);
        map_pages(e, &[0x7000]);
        e.mem.set_u32(0x7518, OTHER_TABLE);
        e.mem.set_u32(hit, target);
        double(e, ACTOR_GET_CURRENT_WEAPON, eax(0x7400));
        double(e, IMPACT_SOUND_SOURCE, eax(0x7410));
        double(e, SOUND_SET_ENTRY, eax(0x7500));
        double(e, SOUND_SET_ENTRY_SOUND, eax(0x7600));
        target
    }

    #[test]
    fn hit_effects_need_the_actor_to_allow_them_and_positive_damage() {
        let mut e = engine3();
        let (actor, hit) = hit_world(&mut e);
        e.register(HIT_EFFECTS_ALLOWED, |_, _| eax(0));
        e.call_log = Some(vec![]);
        fn_0088e8d0(&mut e, actor, 5.0, Ptr::new(hit));
        let addresses = logged_addresses(&e);
        assert!(addresses.contains(&TIMER_SCOPE_CONSTRUCT));
        assert!(addresses.contains(&TIMER_SCOPE_DESTROY));
        assert!(!addresses.contains(&LISTENER_OBJECT));
        e.register(HIT_EFFECTS_ALLOWED, |_, _| eax(1));
        e.call_log = Some(vec![]);
        fn_0088e8d0(&mut e, actor, 0.0, Ptr::new(hit));
        assert!(!logged_addresses(&e).contains(&LISTENER_OBJECT));
    }

    #[test]
    fn the_scope_timer_gets_the_source_line() {
        let mut e = engine3();
        let (actor, hit) = hit_world(&mut e);
        let timer = record(&mut e, TIMER_SCOPE_CONSTRUCT, Ret::default());
        fn_0088e8d0(&mut e, actor, 0.0, Ptr::new(hit));
        let call = timer.borrow()[0].clone();
        assert_eq!(call[1..], [0x32, 1, ACTOR_SOURCE_FILE_NAME, 0x1f69]);
    }

    #[test]
    fn the_player_hit_counts_points() {
        let mut e = engine3();
        let (actor, hit) = hit_world(&mut e);
        e.mem.set_u32(PLAYER_CHARACTER, actor.addr());
        e.mem.set_f32(hit + 0x14, 12.5);
        e.register(DAMAGE_POINTS, |_, a| {
            assert_eq!(a[0], 12.5f32.to_bits());
            eax(4)
        });
        let step = record(&mut e, PLAYER_STATISTIC_STEP, Ret::default());
        fn_0088e8d0(&mut e, actor, 5.0, Ptr::new(hit));
        assert_eq!(*step.borrow(), vec![vec![4, 0.0f32.to_bits(), 0]]);
    }

    fn visibility_world(e: &mut Engine) -> (Ptr<Actor>, u32, CallRecord) {
        let (actor, hit) = hit_world(e);
        e.register(WEIGHT_OVERRIDE_FLAG, |_, _| eax(0));
        e.register(GLOBAL_WORD_ADDRESS, |_, _| eax(0x011d_f790));
        e.mem.set_u32(0x011d_f790, 5);
        e.register(COMBATANT_COUNT, |_, _| eax(3));
        e.register(SETTING_VALUE_ADDRESS, |_, _| eax(0x011c_e270));
        e.mem.set_f32(0x011c_e270, 50.0);
        let visible = record(e, HIT_EFFECT_VISIBLE, eax(0));
        let allowed = record(e, DECAL_CREATE, eax(0x7300));
        let _ = allowed;
        (actor, hit, visible)
    }

    #[test]
    fn a_far_hit_is_shown_only_when_the_visibility_test_fails_to_hide_it() {
        let mut e = engine3();
        let (actor, hit, visible) = visibility_world(&mut e);
        let created = record(&mut e, RANDOM_ANGLE, st0(0.0));
        // Distance 100 > range 50: the test decides, and it answers 0, so the
        // hit is shown.
        fn_0088e8d0(&mut e, actor, 5.0, Ptr::new(hit));
        assert_eq!(visible.borrow().len(), 1);
        assert_eq!(visible.borrow()[0][0..3], [100.0f32.to_bits(), 0, 0]);
        assert_eq!(visible.borrow()[0][3], 0x6000);
        assert_eq!(created.borrow().len(), 1);
        // The test says "hidden": nothing more happens.
        double(&mut e, HIT_EFFECT_VISIBLE, eax(1));
        fn_0088e8d0(&mut e, actor, 5.0, Ptr::new(hit));
        assert_eq!(created.borrow().len(), 1);
    }

    #[test]
    fn a_near_hit_or_a_busy_combat_is_shown_without_the_visibility_test() {
        let mut e = engine3();
        let (actor, hit, visible) = visibility_world(&mut e);
        let created = record(&mut e, RANDOM_ANGLE, st0(0.0));
        e.mem.set_f32(0x011c_e270, 500.0);
        fn_0088e8d0(&mut e, actor, 5.0, Ptr::new(hit));
        assert!(visible.borrow().is_empty());
        assert_eq!(created.borrow().len(), 1);
        // More combatants than the limit, with a target that is not the
        // player's: shown regardless of the distance.
        e.mem.set_f32(0x011c_e270, 1.0);
        let target = armed_target(&mut e, hit);
        e.mem.set_u32(hit + 0x10, 3);
        let _ = target;
        e.register(COMBATANT_COUNT, |_, _| eax(9));
        fn_0088e8d0(&mut e, actor, 5.0, Ptr::new(hit));
        assert!(visible.borrow().is_empty());
        assert_eq!(created.borrow().len(), 2);
        // The override flag shows it too.
        e.register(COMBATANT_COUNT, |_, _| eax(0));
        e.register(WEIGHT_OVERRIDE_FLAG, |_, _| eax(1));
        fn_0088e8d0(&mut e, actor, 5.0, Ptr::new(hit));
        assert_eq!(created.borrow().len(), 3);
    }

    /// Snapshots a hit record when `CELL_ADD_HIT` is called.
    type HitSnapshots = std::rc::Rc<std::cell::RefCell<Vec<(Vec<u32>, Vec<u32>)>>>;

    fn snapshot_hits(e: &mut Engine) -> HitSnapshots {
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let sink = seen.clone();
        e.register_double(CELL_ADD_HIT, move |e, a| {
            let words: Vec<u32> = (0..0x20).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
            sink.borrow_mut().push((a.to_vec(), words));
            Ret::default()
        });
        seen
    }

    #[test]
    fn the_first_decal_is_cast_on_the_cell_and_marks_the_hit_body_parts() {
        let mut e = engine3();
        let (actor, hit) = hit_world(&mut e);
        e.register(WEIGHT_OVERRIDE_FLAG, |_, _| eax(1));
        let target = armed_target(&mut e, hit);
        slot_at(&mut e, FORM_TABLE, VSLOT_ACTOR_FLAG_0X360, eax(1));
        e.register(DECAL_SCALE_MIN, |_, _| st0(1.5));
        e.register(DECAL_SCALE_MAX, |_, _| st0(2.5));
        e.register(RANDOM_FLOAT, |_, a| {
            // The decal scale is the random number between its bounds; the
            // offsets and the byte source pass their first argument.
            st0(f32::from_bits(a[0]))
        });
        let seen = snapshot_hits(&mut e);
        let collected = record(&mut e, DECAL_COLLECT, Ret::default());
        let caster = record(&mut e, DECAL_CASTER_ADD, Ret::default());
        e.register(LIST_ENTRY_COUNT, |_, _| eax(2));
        let first = e.mem.alloc(0x20);
        let second = e.mem.alloc(0x20);
        e.mem.set_u32(first + 8, 0x8100);
        e.mem.set_u32(second + 8, 0x8200);
        e.register_double(LIST_ENTRY, move |_, a| {
            eax(if a[1] == 0 { first } else { second })
        });
        let this_address = actor.addr();
        e.register(COLLIDABLE_3D_OBJECT, |_, a| eax(a[0] + 1));
        e.register_double(FIND_REFERENCE_FOR_3D, move |_, a| {
            eax(if a[0] == 0x8101 { this_address } else { 0x9999 })
        });
        e.register(ACTOR_BODY_PART_NUMBER, |_, _| eax(3));
        // The objects after the first decal do not matter here: stop the
        // blood sound and the second decal.
        double(&mut e, HIT_SECOND_DISTANCE, st0(1000.0));
        e.register(SETTING_VALUE_ADDRESS, |_, _| eax(0x011c_e270));
        e.mem.set_f32(0x011c_e270, 1.0);
        let root = e.mem.alloc(0x10);
        let _ = root;
        slot(&mut e, ACTOR_TABLE, VSLOT_GET_3D, eax(0x5555));
        fn_0088e8d0(&mut e, actor, 5.0, Ptr::new(hit));
        let _ = target;
        // The decal goes to the cell's object, collecting around the hit
        // point.
        assert_eq!(*caster.borrow(), vec![vec![0x7300, 0xc000 + 0x1000]]);
        assert_eq!(collected.borrow()[0][0], 0x7300);
        // One cell call for the record; kind 2; the target is "player-like".
        let (call, words) = seen.borrow()[0].clone();
        assert_eq!(call[0], 0xc000);
        assert_eq!(call[2..], [2, 1]);
        // Position (100, 0, 0) and the normalized direction, the scale
        // twice, the root, the ray kind and the random byte.
        assert_eq!(f32::from_bits(words[0]), 100.0);
        assert_eq!(words[0x38 / 4], 1.5f32.to_bits());
        assert_eq!(words[0x3c / 4], 1.5f32.to_bits());
        assert_eq!(words[0x28 / 4], 0x5555);
        assert_eq!(words[0x34 / 4], 0xffff_ffff);
        assert_eq!(words[0x30 / 4], 0x7600);
        // Body part 3 + 1 = bit 4 is set, bit 0 stays clear.
        assert_eq!(words[0x6c / 4], 1 << 4);
        assert_eq!(words[0x70 / 4] & 0xff, 77);
    }

    #[test]
    fn a_hit_on_a_creature_needs_no_decal_and_makes_no_cell_call() {
        let mut e = engine3();
        let (actor, hit) = hit_world(&mut e);
        e.register(WEIGHT_OVERRIDE_FLAG, |_, _| eax(1));
        armed_target(&mut e, hit);
        slot(&mut e, ACTOR_TABLE, VSLOT_GET_3D, eax(0x5555));
        // Kind 1 with the actor virtual +0x218 true skips the decal.
        e.mem.set_u32(hit + 0x10, 1);
        slot(&mut e, ACTOR_TABLE, VSLOT_TEST_0X218, eax(1));
        let seen = snapshot_hits(&mut e);
        let created = record(&mut e, DECAL_CREATE, eax(0x7300));
        double(&mut e, HIT_SECOND_DISTANCE, st0(1000.0));
        e.register(SETTING_VALUE_ADDRESS, |_, _| eax(0x011c_e270));
        e.mem.set_f32(0x011c_e270, 1.0);
        fn_0088e8d0(&mut e, actor, 5.0, Ptr::new(hit));
        assert!(created.borrow().is_empty());
        assert!(seen.borrow().is_empty());
    }

    #[test]
    fn the_blood_spurt_plays_with_the_negated_direction() {
        let mut e = engine3();
        let (actor, hit) = hit_world(&mut e);
        e.register(WEIGHT_OVERRIDE_FLAG, |_, _| eax(1));
        armed_target(&mut e, hit);
        slot(&mut e, ACTOR_TABLE, VSLOT_GET_3D, eax(0));
        let name = e.mem.alloc(0x10);
        e.mem.set_u8(name, b'b');
        let member_table = 0x0200_a000;
        // The entry 0x7500 + 0x18 is the member whose virtual +0x14 yields
        // the name pointer.
        let member = 0x7518u32;
        e.mem.set_u32(member, member_table);
        e.mem
            .set_u32(member_table + 0x14, slot_target(member_table, 0x14));
        e.register_double(slot_target(member_table, 0x14), move |_, _| eax(name));
        e.register(VECTOR_NEGATE_INTO, |e, a| {
            for word in 0..3 {
                let value = -e.mem.f32(a[0] + 4 * word);
                e.mem.set_f32(a[1] + 4 * word, value);
            }
            eax(a[1])
        });
        let played = record(&mut e, PLAY_HIT_SPURT, Ret::default());
        double(&mut e, HIT_SECOND_DISTANCE, st0(1000.0));
        e.register(SETTING_VALUE_ADDRESS, |_, _| eax(0x011c_e270));
        e.mem.set_f32(0x011c_e270, 1.0);
        fn_0088e8d0(&mut e, actor, 5.0, Ptr::new(hit));
        let call = played.borrow()[0].clone();
        assert_eq!(call[0], 0xc000);
        assert_eq!(call[1], 1.0f32.to_bits());
        assert_eq!(call[2], name);
        // The direction (normalized by the double: here untouched) negated;
        // the point; 1.0, 7 and the collidable's 3D object (none).
        assert_eq!(call[6], 100.0f32.to_bits());
        assert_eq!(call[9..], [1.0f32.to_bits(), 7, 0]);
    }

    /// The hit records the second decal added (kind 1), not the first (kind 2).
    fn second_only(seen: &HitSnapshots) -> Vec<(Vec<u32>, Vec<u32>)> {
        seen.borrow()
            .iter()
            .filter(|entry| entry.0[2] == 1)
            .cloned()
            .collect()
    }

    /// The world for the second decal: the hit body part has a sound set
    /// entry; a ray along the hit direction meets one object of the actor.
    fn second_decal_world(e: &mut Engine) -> (Ptr<Actor>, u32) {
        let (actor, hit) = hit_world(e);
        e.register(WEIGHT_OVERRIDE_FLAG, |_, _| eax(1));
        armed_target(e, hit);
        slot(e, ACTOR_TABLE, VSLOT_GET_3D, eax(0x5555));
        let cell_object = e.mem.alloc(0x10);
        e.mem.set_u32(cell_object, OTHER_TABLE);
        e.register_double(CELL_OBJECT_OF, move |_, _| eax(cell_object));
        let list_entry = e.mem.alloc(0x20);
        e.register_double(LIST_ENTRY, move |_, _| eax(list_entry));
        slot_at(e, FORM_TABLE, BASE_FORM_VSLOT_0X180, eax(1));
        double(e, BODY_PART_OF, eax(0x7700));
        double(e, BODY_PART_SOUND_SOURCE, eax(0x7710));
        double(e, HIT_SECOND_DISTANCE, st0(5.0));
        e.register(SETTING_VALUE_ADDRESS, |_, _| eax(0x011c_e270));
        e.mem.set_f32(0x011c_e270, 50.0);
        e.register(VECTOR_TIMES_FLOAT_INTO, |e, a| {
            let factor = f32::from_bits(a[2]);
            for word in 0..3 {
                let value = factor * e.mem.f32(a[0] + 4 * word);
                e.mem.set_f32(a[1] + 4 * word, value);
            }
            eax(a[1])
        });
        e.register(DECAL_SCALE_MIN, |_, _| st0(1.5));
        e.register(DECAL_SCALE_MAX, |_, _| st0(2.5));
        e.register(PICK_RESULT_HOLDER, |_, a| eax(a[0] + 0xa8));
        e.register(LIST_ENTRY_COUNT, |_, _| eax(1));
        let record = e.mem.alloc(0x100);
        e.mem.set_u32(record + 0x50, 0x8800);
        e.mem.set_f32(record + 0x10, 0.5);
        e.register_double(PICK_RESULT_RECORD, move |_, _| eax(record));
        e.register(PICK_RECORD_NORMAL, |e, a| {
            e.mem.set_f32(a[0], 0.0);
            e.mem.set_f32(a[0] + 4, 1.0);
            e.mem.set_f32(a[0] + 8, 0.0);
            Ret::default()
        });
        double(e, NODE_FILTER_WORD, eax(0x0001_0000));
        double(e, TES_OBJECT_AT_POINT, eax(0x8000));
        double(e, FIND_REFERENCE_FOR_3D, eax(0x9000));
        double(e, TES_CELL_AT_POINT, eax(0xd000));
        double(e, DECAL_FIELD_BYTE_A, eax(9));
        e.register(DECAL_COLOR_WORD, |_, _| eax(0x00ff_8040));
        e.register(NI_POINT3_SET, |e, a| {
            for word in 0..3 {
                e.mem.set_u32(a[0] + 4 * word, a[1 + word as usize]);
            }
            eax(a[0])
        });
        (actor, hit)
    }

    #[test]
    fn the_second_decal_is_put_where_the_ray_meets_the_actor() {
        let mut e = engine3();
        let (actor, hit) = second_decal_world(&mut e);
        let seen = snapshot_hits(&mut e);
        let ends = record(&mut e, PICK_SET_END, Ret::default());
        let collector = record(&mut e, RAY_COLLECTOR_DESTROY, Ret::default());
        slot(&mut e, OTHER_TABLE, OWNER_VSLOT_0X08, eax(0));
        fn_0088e8d0(&mut e, actor, 5.0, Ptr::new(hit));
        assert_eq!(collector.borrow().len(), 1);
        // The ray end is the point plus the reach (3.0) along the direction
        // with its z lowered by one half.
        let end = ends.borrow()[0][1];
        assert_eq!(second_only(&seen).len(), 1);
        let (call, words) = second_only(&seen)[0].clone();
        assert_eq!(call[0], 0xd000);
        assert_eq!(call[2..], [1, 0]);
        let _ = end;
        // Hit point: 100 + 0.5 * 7.5 = 103.75 along x.
        assert_eq!(f32::from_bits(words[0]), 103.75);
        assert_eq!(f32::from_bits(words[1]), 0.75);
        // The normal comes from the pick record.
        assert_eq!(f32::from_bits(words[4]), 1.0);
        assert_eq!(words[0x38 / 4], 1.5f32.to_bits());
        assert_eq!(words[0x28 / 4], 0x8000);
        assert_eq!(words[0x30 / 4], 0x7600);
        assert_eq!(words[0x70 / 4] & 0xff, 77);
        assert_eq!(words[0x70 / 4] >> 24, 9);
        // The colour bytes 0xff, 0x80, 0x40 as (blue, green, red) fractions.
        assert_eq!(f32::from_bits(words[0x60 / 4]), 0x40 as f32 / 255.0);
        assert_eq!(f32::from_bits(words[0x64 / 4]), 0x80 as f32 / 255.0);
        assert_eq!(f32::from_bits(words[0x68 / 4]), 1.0);
    }

    #[test]
    fn a_hit_on_armor_enlarges_the_second_decal() {
        let mut e = engine3();
        let (actor, hit) = second_decal_world(&mut e);
        let seen = snapshot_hits(&mut e);
        slot(&mut e, OTHER_TABLE, OWNER_VSLOT_0X08, eax(1));
        fn_0088e8d0(&mut e, actor, 5.0, Ptr::new(hit));
        let (_, words) = second_only(&seen)[0].clone();
        assert_eq!(words[0x38 / 4], 3.0f32.to_bits());
        assert_eq!(words[0x3c / 4], 3.0f32.to_bits());
    }

    #[test]
    fn the_second_decal_needs_the_range_a_target_and_a_non_actor_object_check() {
        let mut e = engine3();
        let (actor, hit) = second_decal_world(&mut e);
        let seen = snapshot_hits(&mut e);
        // Too far: the limit setting is below the distance.
        e.mem.set_f32(0x011c_e270, 1.0);
        fn_0088e8d0(&mut e, actor, 5.0, Ptr::new(hit));
        assert!(second_only(&seen).is_empty());
        e.mem.set_f32(0x011c_e270, 50.0);
        // No sound set entry for the body part: nothing.
        double(&mut e, BODY_PART_OF, eax(0));
        fn_0088e8d0(&mut e, actor, 5.0, Ptr::new(hit));
        assert!(second_only(&seen).is_empty());
        double(&mut e, BODY_PART_OF, eax(0x7700));
        // A non-actor node whose reference fails the form test is skipped.
        double(&mut e, NODE_FILTER_WORD, eax(0));
        double(&mut e, COLLIDABLE_3D_OBJECT, eax(0x8200));
        double(&mut e, REFERENCE_FORM_TEST, eax(0));
        fn_0088e8d0(&mut e, actor, 5.0, Ptr::new(hit));
        assert!(second_only(&seen).is_empty());
        double(&mut e, REFERENCE_FORM_TEST, eax(1));
        fn_0088e8d0(&mut e, actor, 5.0, Ptr::new(hit));
        assert_eq!(second_only(&seen).len(), 1);
        assert_eq!(second_only(&seen)[0].1[0x28 / 4], 0x8200);
    }

    // ---- weapon hit effects ----

    /// A weapon hit world: the actor has a process holding a weapon, the
    /// weapon node sits at (10, 0, 0) and the weapon at (20, 0, 0); the
    /// attacker's body part has a sound set entry.
    fn weapon_hit_world(e: &mut Engine) -> (Ptr<Actor>, u32, u32, u32) {
        let (actor, hit) = hit_world(e);
        armed_target(e, hit);
        double(e, RT_DYNAMIC_CAST_FN, eax(0));
        let step_object = e.mem.alloc(0x40);
        double(e, ACTOR_GET_ANIMATION, eax(0x5500));
        double(e, ANIMATION_LOOKUP, eax(0x5600));
        double(e, ANIMATION_STEP, eax(step_object));
        double(e, PLAYER_STATE_TEST, eax(0));
        let attacker = e.mem.alloc(0x100);
        slot_at(e, FORM_TABLE, BASE_FORM_VSLOT_0X180, eax(1));
        double(e, BODY_PART_OF, eax(0x7700));
        double(e, BODY_PART_SOUND_SOURCE, eax(0x7710));
        let process = with_process(e, actor);
        let _ = process;
        record_slot(e, PROCESS_TABLE, PROCESS_VSLOT_0X1B0, eax(0));
        let node = e.mem.alloc(0x100);
        record_slot(e, PROCESS_TABLE, PROCESS_VSLOT_WEAPON_NODE, eax(node));
        let held = e.mem.alloc(0x20);
        let held_form = e.mem.alloc(0x20);
        e.mem.set_u32(held + 8, held_form);
        record_slot(e, PROCESS_TABLE, PROCESS_VSLOT_ITEM, eax(held));
        e.register(ITEM_VALUE, |e, a| eax(e.mem.u32(a[0] + 8)));
        double(e, HELD_ITEM_FLAG_TEST, eax(0));
        slot(e, ACTOR_TABLE, VSLOT_GET_BIPED, eax(0x9900));
        e.register(FLOAT_HELPER_MAX, |_, a| {
            st0(f32::from_bits(a[0]).max(f32::from_bits(a[1])))
        });
        let node_position = e.mem.alloc(0x10);
        e.mem.set_f32(node_position, 10.0);
        let weapon_position = e.mem.alloc(0x10);
        e.mem.set_f32(weapon_position, 20.0);
        e.register_double(OBJECT_POSITION_0X8C, move |_, _| eax(node_position));
        e.register_double(GET_WEAPON_POSITION, move |_, _| eax(weapon_position));
        e.register(VECTOR_DIVIDE_BY_FLOAT, |e, a| {
            let divisor = f32::from_bits(a[2]);
            for word in 0..3 {
                let value = e.mem.f32(a[0] + 4 * word) / divisor;
                e.mem.set_f32(a[1] + 4 * word, value);
            }
            eax(a[1])
        });
        e.register(VECTOR_NEGATE_INTO, |e, a| {
            for word in 0..3 {
                let value = -e.mem.f32(a[0] + 4 * word);
                e.mem.set_f32(a[1] + 4 * word, value);
            }
            eax(a[1])
        });
        e.mem.set_f32(hit + 0x44, 0.0);
        e.mem.set_f32(hit + 0x48, 1.0);
        e.register(DECAL_COLOR_WORD, |_, _| eax(0x00ff_8040));
        e.register(NI_POINT3_SET, |e, a| {
            for word in 0..3 {
                e.mem.set_u32(a[0] + 4 * word, a[1 + word as usize]);
            }
            eax(a[0])
        });
        (actor, hit, attacker, node)
    }

    fn weapon_hit(e: &mut Engine, actor: Ptr<Actor>, damage: f32, hit: u32, attacker: u32) {
        fn_0088fb00(e, actor, damage, Ptr::new(hit), Ptr::new(attacker));
    }

    #[test]
    fn weapon_hits_place_two_decals_at_the_weapon_for_other_actors() {
        let mut e = engine3();
        let (actor, hit, attacker, node) = weapon_hit_world(&mut e);
        let seen = snapshot_hits(&mut e);
        weapon_hit(&mut e, actor, 60.0, hit, attacker);
        assert_eq!(seen.borrow().len(), 2);
        let (call, words) = seen.borrow()[0].clone();
        assert_eq!(call[0], 0xc000);
        assert_eq!(call[2..], [1, 0]);
        // The start is the node position plus the offset to the weapon
        // divided by 2: 10 + (20 - 10) / 2.
        assert_eq!(f32::from_bits(words[0]), 15.0);
        // The direction is the negated hit direction (0, -1, 0).
        assert_eq!(f32::from_bits(words[4]), -1.0);
        assert_eq!(words[0x28 / 4], node);
        assert_eq!(words[0x2c / 4], 0);
        assert_eq!(words[0x30 / 4], 0x7600);
        assert_eq!(words[0x34 / 4], 0xffff_ffff);
        assert_eq!(words[0x70 / 4] & 0xff, 77);
        assert_eq!(words[0x78 / 4] & 0xff, 1);
        assert_eq!(f32::from_bits(words[0x60 / 4]), 0x40 as f32 / 255.0);
        // The second call uses the second node (none here).
        assert_eq!(seen.borrow()[1].1[0x28 / 4], 0);
    }

    #[test]
    fn weapon_hits_need_enough_damage() {
        let mut e = engine3();
        let (actor, hit, attacker, _) = weapon_hit_world(&mut e);
        let seen = snapshot_hits(&mut e);
        // The setting is 50: damage of 50 or less does nothing.
        weapon_hit(&mut e, actor, 50.0, hit, attacker);
        assert!(seen.borrow().is_empty());
        weapon_hit(&mut e, actor, 50.5, hit, attacker);
        assert_eq!(seen.borrow().len(), 2);
    }

    #[test]
    fn weapon_hits_are_skipped_when_a_party_is_missing_or_blocks() {
        let mut e = engine3();
        let (actor, hit, attacker, _) = weapon_hit_world(&mut e);
        let seen = snapshot_hits(&mut e);
        weapon_hit(&mut e, actor, 60.0, hit, 0);
        assert!(seen.borrow().is_empty());
        // The process virtual +0x1b0 blocks.
        record_slot(&mut e, PROCESS_TABLE, PROCESS_VSLOT_0X1B0, eax(1));
        weapon_hit(&mut e, actor, 60.0, hit, attacker);
        assert!(seen.borrow().is_empty());
        record_slot(&mut e, PROCESS_TABLE, PROCESS_VSLOT_0X1B0, eax(0));
        // So does the held item flag, or a missing weapon node.
        double(&mut e, HELD_ITEM_FLAG_TEST, eax(1));
        weapon_hit(&mut e, actor, 60.0, hit, attacker);
        assert!(seen.borrow().is_empty());
        double(&mut e, HELD_ITEM_FLAG_TEST, eax(0));
        record_slot(&mut e, PROCESS_TABLE, PROCESS_VSLOT_WEAPON_NODE, eax(0));
        weapon_hit(&mut e, actor, 60.0, hit, attacker);
        assert!(seen.borrow().is_empty());
    }

    #[test]
    fn weapon_hits_on_humanoid_creatures_obey_the_setting_byte() {
        let mut e = engine3();
        let (actor, hit, attacker, _) = weapon_hit_world(&mut e);
        let seen = snapshot_hits(&mut e);
        double(&mut e, RT_DYNAMIC_CAST_FN, eax(0x7800));
        double(&mut e, CREATURE_IS_HUMANOID, eax(1));
        e.register(SETTING_VALUE_POINTER, |e, _| {
            e.mem.set_u8(0x011d_f7f8, 1);
            eax(0x011d_f7f8)
        });
        weapon_hit(&mut e, actor, 60.0, hit, attacker);
        assert!(seen.borrow().is_empty());
        e.register(SETTING_VALUE_POINTER, |e, _| {
            e.mem.set_u8(0x011d_f7f8, 0);
            eax(0x011d_f7f8)
        });
        weapon_hit(&mut e, actor, 60.0, hit, attacker);
        assert_eq!(seen.borrow().len(), 2);
    }

    #[test]
    fn the_player_weapon_hit_reads_the_start_from_the_animation_table() {
        let mut e = engine3();
        let (actor, hit, attacker, node) = weapon_hit_world(&mut e);
        e.mem.set_u32(PLAYER_CHARACTER, actor.addr());
        let page = (actor.addr() + 0x7c7) & !0xfff;
        for page in [page, page + 0x1000] {
            if !e.mem.is_mapped(page) {
                e.map(page, 0x1000);
            }
        }
        let seen = snapshot_hits(&mut e);
        double(&mut e, PLAYER_GET_BIPED, eax(0x6400));
        double(&mut e, BIPED_NODE_BY_INDEX, eax(0x6300));
        double(&mut e, PLAYER_STATE_TEST, eax(1));
        let bound = e.mem.alloc(0x10);
        double(&mut e, GET_WORLD_BOUND, eax(bound));
        double(&mut e, UPDATE_STEP_SETTING, st0(2.0));
        double(&mut e, NODE_FLOAT_0X98, st0(4.0));
        double(&mut e, PLAYER_ANIMATION_OWNER, eax(0x5500));
        let animation = e.mem.alloc(0x40);
        double(&mut e, ANIMATION_LOOKUP, eax(0x5600));
        double(&mut e, ANIMATION_STEP, eax(animation));
        // The animation says table 1.
        e.mem.set_u8(animation + 0x2c, 1);
        double(&mut e, NODE_EXTRA_OWNER_COUNT, eax(0));
        double(&mut e, DEFAULT_EXTRA_DATA_NAME, eax(0x1234));
        let table = e.mem.alloc(0x200);
        let extra = e.mem.alloc(0x40);
        e.mem.set_u16(extra + 0x10, 2);
        e.mem.set_u32(extra + 0x14, 0);
        e.mem.set_u32(extra + 0x18, table);
        e.mem.set_u16(table, 3);
        e.mem.set_f32(table + 12 + 4, 1.5);
        e.mem.set_f32(table + 12 + 0x64, 2.5);
        let asked = record(&mut e, NODE_GET_EXTRA_DATA, eax(extra));
        let random = record(&mut e, RANDOM_INTEGER, eax(1));
        weapon_hit(&mut e, actor, 60.0, hit, attacker);
        assert_eq!(*asked.borrow(), vec![vec![node, 0x1234]]);
        assert_eq!(*random.borrow(), vec![vec![0, 2]]);
        assert_eq!(seen.borrow().len(), 2);
        let (_, words) = seen.borrow()[0].clone();
        assert_eq!(f32::from_bits(words[0]), 1.5);
        // The player's direction is the table's direction.
        assert_eq!(f32::from_bits(words[3]), 2.5);
        // First person: the second node is the player's node.
        assert_eq!(words[0x28 / 4], node);
        assert_eq!(words[0x2c / 4], 0x6300);
        assert_eq!(seen.borrow()[1].1[0x28 / 4], 0x6300);
    }

    #[test]
    fn the_player_weapon_hit_stops_without_a_table_entry() {
        let mut e = engine3();
        let (actor, hit, attacker, _) = weapon_hit_world(&mut e);
        e.mem.set_u32(PLAYER_CHARACTER, actor.addr());
        let page = (actor.addr() + 0x7c7) & !0xfff;
        for page in [page, page + 0x1000] {
            if !e.mem.is_mapped(page) {
                e.map(page, 0x1000);
            }
        }
        let seen = snapshot_hits(&mut e);
        double(&mut e, PLAYER_GET_BIPED, eax(0x6400));
        double(&mut e, BIPED_NODE_BY_INDEX, eax(0x6300));
        let bound = e.mem.alloc(0x10);
        double(&mut e, GET_WORLD_BOUND, eax(bound));
        double(&mut e, NODE_EXTRA_OWNER_COUNT, eax(0));
        double(&mut e, UPDATE_STEP_SETTING, st0(2.0));
        double(&mut e, NODE_FLOAT_0X98, st0(4.0));
        double(&mut e, PLAYER_ANIMATION_OWNER, eax(0));
        double(&mut e, DEFAULT_EXTRA_DATA_NAME, eax(0x1234));
        double(&mut e, NODE_GET_EXTRA_DATA, eax(0));
        weapon_hit(&mut e, actor, 60.0, hit, attacker);
        assert!(seen.borrow().is_empty());
    }

    // ---- traps ----

    const TRAP_CALLEES: &[u32] = &[
        TIMER_SCOPE_CONSTRUCT,
        TIMER_SCOPE_DESTROY,
        HIT_FLAG_MELEE,
        IMPACT_SECONDARY_FLAG,
        HIT_EFFECTS_ALLOWED,
        SOUND_PLAY_ARGUMENT,
        GET_EXTRA_DATA_LIST_ADDRESS,
        EXTRA_LIST_GET_SCRIPT,
        EXTRA_LIST_GET_SCRIPT_LOCALS,
        PLAYER_TRAP_OWNER,
        ACTOR_LEVEL,
        SCALE_BY_GLOBAL_FACTOR,
        GET_CHAR_CONTROLLER,
        COLLISION_BODY_OF_FORM,
        POSITION_VECTOR_OF,
        SUBOBJECT_SET_VALUE,
        MATRIX_CONSTRUCT,
        BODY_FIRST_VECTOR,
        BODY_SECOND_VECTOR,
        VECTOR_ROOT_INTO,
        VECTOR_LENGTH_FN,
        CONTROLLER_LINEAR_VELOCITY,
        VECTOR_SUBTRACT_ASSIGN,
        VECTOR_CONVERT,
        VECTOR_COMPONENT_ADDRESS,
        STORE_WORD,
        NODE_FILTER_WORD,
        KIND_OF_WORD,
        FLOAT_HELPER_MIN,
        UPDATE_STEP_SETTING,
        VECTOR_TIMES_FLOAT_INTO,
        OBJECT_POSITION_0X8C,
        VECTOR_PLUS_VECTOR,
        VECTOR_MINUS_VECTOR,
        VECTOR_DERIVE,
        HIT_DATA_CONSTRUCT,
        COLLISION_RECORD_CONSTRUCT,
        BODY_NUMBER,
        VECTOR_FROM_FLOAT,
        PLAY_COLLISION_SOUND,
        ACTOR_TRIGGER_PAIN,
        WORLD_SET_MOTION,
        REMOVE_FORCE,
        VECTOR_SCALE_BY_CONSTANT,
        ADD_EXPLOSION_IMPULSE,
        SET_VELOCITY_MODIFIER_FN,
        VECTOR_TIMES_SCALAR_ASSIGN,
        VECTOR_NORMALIZE,
        LOCALS_SET_VARIABLE,
        GET_PARENT_CELL,
        FLAG_WORD_AND,
        FLAG_WORD_SET_OR_CLEAR,
    ];

    type Variables = std::rc::Rc<std::cell::RefCell<Vec<(u32, f32)>>>;

    /// A trap world: the script of the trap reference defines the variables
    /// of `variables` (name address, value); the trap record `trap` holds
    /// its reference's form in the member at +4; the actor is not the
    /// player and has the character controller 0x5500.
    fn trap_world(e: &mut Engine, variables: &[(u32, f32)]) -> (Ptr<Actor>, u32, Ptr, Variables) {
        map_pages(
            e,
            &[
                0x0101_7000,
                0x0101_3000,
                0x0102_2000,
                0x0102_0000,
                0x0101_1000,
                0x0101_6000,
                0x011a_9000,
                0x011f_6000,
                0x011c_5000,
                0x0102_3000,
            ],
        );
        new_table(e, FORM_TABLE);
        let actor = actor2(e);
        bounds_and_scale(e, 1.0);
        e.mem.set_u32(PLAYER_CHARACTER, 0x4040);
        for address in TRAP_CALLEES.iter().copied() {
            double(e, address, Ret::default());
        }
        e.set_global(DOUBLE_ZERO, 0.0f64);
        e.set_global(DOUBLE_HALF, 0.5f64);
        e.set_global(TRAP_PUSH_TO_IMPULSE_DIVISOR, 10.0f64);
        e.set_global(TRAP_SPEED_SCALE_LIMIT, 2.0f64);
        e.set_global(TRAP_LEVEL_FACTOR_DIVISOR, 100.0f64);
        e.set_global(TRAP_IMPULSE_LIMIT, 1000.0f64);
        e.set_global(TRAP_PUSH_LIMIT, 1000.0f64);
        let trap = e.mem.alloc(0x40);
        let trap_form = e.mem.alloc(0x40);
        e.mem.set_u32(trap + 4 + 8, trap_form);
        let trap_data = e.mem.alloc(0x40);
        e.register(SOUND_PLAY_ARGUMENT, |_, _| eax(0x7100));
        e.register(GET_EXTRA_DATA_LIST_ADDRESS, |_, a| eax(a[0] + 0x44));
        e.register(EXTRA_LIST_GET_SCRIPT, |_, _| eax(0x7200));
        e.register(EXTRA_LIST_GET_SCRIPT_LOCALS, |_, _| eax(0x7300));
        e.register(ITEM_VALUE, |e, a| eax(e.mem.u32(a[0] + 8)));
        e.register(FLAG_WORD_AND, |e, a| eax(e.mem.u32(a[0]) & a[1]));
        e.register(VECTOR_COMPONENT_ADDRESS, |_, a| eax(a[0] + 4 * a[1]));
        e.register(MATRIX_CONSTRUCT, |_, a| eax(a[0]));
        e.register(STORE_WORD, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            eax(a[0])
        });
        e.register(KIND_OF_WORD, |e, a| eax(e.mem.u32(a[0])));
        e.register(FLOAT_HELPER_MIN, |_, a| {
            st0(f32::from_bits(a[0]).min(f32::from_bits(a[1])))
        });
        e.register(VECTOR_FROM_FLOAT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            eax(a[0])
        });
        let list: Variables = Default::default();
        let stored = list.clone();
        for (name, value) in variables {
            stored.borrow_mut().push((*name, *value));
        }
        let finder = list.clone();
        e.register_double(SCRIPT_FIND_VARIABLE, move |e, a| {
            let found = finder.borrow().iter().any(|entry| entry.0 == a[1]);
            if found {
                e.mem.set_u32(a[2], a[1]);
            }
            eax(found as u32)
        });
        let reader = list.clone();
        e.register_double(LOCALS_GET_VARIABLE, move |_, a| {
            let value = reader
                .borrow()
                .iter()
                .find(|entry| entry.0 == a[1])
                .map(|entry| entry.1)
                .unwrap_or(0.0);
            st0(value)
        });
        e.register(VECTOR_MINUS_VECTOR, |_, a| eax(a[1]));
        e.register(VECTOR_PLUS_VECTOR, |_, a| eax(a[1]));
        e.register(VECTOR_TIMES_FLOAT_INTO, |_, a| eax(a[1]));
        e.register(VECTOR_DERIVE, |_, a| eax(a[0]));
        e.register(VECTOR_ROOT_INTO, |_, a| eax(a[1]));
        let buffer = e.mem.alloc(0x40);
        e.register_double(POSITION_VECTOR_OF, move |_, _| eax(buffer));
        e.register_double(OBJECT_POSITION_0X8C, move |_, _| eax(buffer));
        e.register(ACTOR_LEVEL, |_, _| eax(5));
        e.register(SCALE_BY_GLOBAL_FACTOR, |_, a| {
            st0(f32::from_bits(a[0]) * 2.0)
        });
        double(e, GET_CHAR_CONTROLLER, eax(0x5500));
        (actor, trap, Ptr::new(trap_data), list)
    }

    fn do_trap_with(e: &mut Engine, actor: Ptr<Actor>, trap: u32, data: Ptr) {
        actor_do_trap(e, actor, Ptr::new(trap), data);
    }

    #[test]
    fn the_players_own_trap_does_not_hurt_the_player() {
        let mut e = engine3();
        let (actor, trap, data, _) = trap_world(&mut e, &[(TRAP_DAMAGE_NAME, 9.0)]);
        e.mem.set_u32(PLAYER_CHARACTER, actor.addr());
        let page = (actor.addr() + 0x7c7) & !0xfff;
        let _ = page;
        double(&mut e, PLAYER_TRAP_OWNER, eax(0x7100));
        let damaged = record_slot(&mut e, ACTOR_TABLE, VSLOT_APPLY_DAMAGE, Ret::default());
        let finished = record(&mut e, FLAG_WORD_SET_OR_CLEAR, Ret::default());
        do_trap_with(&mut e, actor, trap, data);
        assert!(damaged.borrow().is_empty());
        assert!(finished.borrow().is_empty());
    }

    #[test]
    fn a_trap_applies_its_scripted_damage_with_the_levelled_part() {
        let mut e = engine3();
        let (actor, trap, data, _) = trap_world(
            &mut e,
            &[(TRAP_DAMAGE_NAME, 9.0), (TRAP_LEVELLED_DAMAGE_NAME, 2.0)],
        );
        let damaged = record_slot(&mut e, ACTOR_TABLE, VSLOT_APPLY_DAMAGE, Ret::default());
        let pain = record(&mut e, ACTOR_TRIGGER_PAIN, Ret::default());
        let finished = record(&mut e, FLAG_WORD_SET_OR_CLEAR, Ret::default());
        slot(&mut e, ACTOR_TABLE, VSLOT_IS_PROCESS_FLAG_0X22C, eax(0));
        do_trap_with(&mut e, actor, trap, data);
        // 9 + 2 * level 5, with no resistance (actor value 0x13 gives 0).
        assert_eq!(
            *damaged.borrow(),
            vec![vec![actor.addr(), 19.0f32.to_bits(), 0, 0]]
        );
        assert_eq!(*pain.borrow(), vec![vec![actor.addr(), 1, 1]]);
        assert_eq!(*finished.borrow(), vec![vec![data.addr() + 0x10, 1, 1]]);
    }

    #[test]
    fn health_traps_are_reduced_by_the_resistance_value() {
        let mut e = engine3();
        let (actor, trap, data, _) = trap_world(&mut e, &[(TRAP_DAMAGE_NAME, 10.0)]);
        // The form word's kind is 0x10: the owner's value 0x13 is 40 percent.
        e.mem.set_u32(trap + 4 + 8, 0);
        let form = e.mem.alloc(0x20);
        e.mem.set_u32(trap + 4 + 8, form);
        e.mem.set_u32(form + 8, TRAP_KIND_HEALTH);
        e.register(NODE_FILTER_WORD, |e, a| eax(e.mem.u32(a[0] + 8)));
        slot(&mut e, OTHER_TABLE, OWNER_VSLOT_VALUE_0X0C, st0(40.0));
        let damaged = record_slot(&mut e, ACTOR_TABLE, VSLOT_APPLY_DAMAGE, Ret::default());
        do_trap_with(&mut e, actor, trap, data);
        // 10 * (1 - 0.4) = 6.
        assert_eq!(damaged.borrow()[0][1], 6.0f32.to_bits());
    }

    #[test]
    fn a_trap_without_damage_only_marks_its_record() {
        let mut e = engine3();
        let (actor, trap, data, _) = trap_world(&mut e, &[(TRAP_DAMAGE_NAME, 0.0)]);
        let damaged = record_slot(&mut e, ACTOR_TABLE, VSLOT_APPLY_DAMAGE, Ret::default());
        let finished = record(&mut e, FLAG_WORD_SET_OR_CLEAR, Ret::default());
        do_trap_with(&mut e, actor, trap, data);
        assert!(damaged.borrow().is_empty());
        assert_eq!(finished.borrow().len(), 1);
    }

    #[test]
    fn a_trap_that_already_hit_is_ignored_unless_continuous() {
        let mut e = engine3();
        let (actor, trap, data, _) = trap_world(&mut e, &[(TRAP_DAMAGE_NAME, 7.0)]);
        e.mem.set_u32(data.addr() + 0x10, 1);
        let damaged = record_slot(&mut e, ACTOR_TABLE, VSLOT_APPLY_DAMAGE, Ret::default());
        let finished = record(&mut e, FLAG_WORD_SET_OR_CLEAR, Ret::default());
        do_trap_with(&mut e, actor, trap, data);
        assert!(damaged.borrow().is_empty());
        assert!(finished.borrow().is_empty());
        let (actor, trap, data, _) = trap_world(
            &mut e,
            &[(TRAP_DAMAGE_NAME, 7.0), (TRAP_CONTINUOUS_NAME, 1.0)],
        );
        e.mem.set_u32(data.addr() + 0x10, 1);
        e.register(UPDATE_STEP_SETTING, |_, _| st0(1.0));
        let damaged = record_slot(&mut e, ACTOR_TABLE, VSLOT_APPLY_DAMAGE, Ret::default());
        do_trap_with(&mut e, actor, trap, data);
        assert_eq!(damaged.borrow()[0][1], 7.0f32.to_bits());
    }

    #[test]
    fn the_script_marks_the_trap_as_having_hit() {
        let mut e = engine3();
        let (actor, trap, data, _) =
            trap_world(&mut e, &[(TRAP_DAMAGE_NAME, 3.0), (TRAP_HAS_HIT_NAME, 0.0)]);
        let set = record(&mut e, LOCALS_SET_VARIABLE, Ret::default());
        do_trap_with(&mut e, actor, trap, data);
        let call = set.borrow()[0].clone();
        assert_eq!(call[0..2], [0x7300, TRAP_HAS_HIT_NAME]);
        assert_eq!(f64::from_bits(call[2] as u64 | (call[3] as u64) << 32), 1.0);
    }

    #[test]
    fn the_hit_record_of_a_non_health_trap_is_built_and_sent_on() {
        let mut e = engine3();
        let (actor, trap, data, _) = trap_world(&mut e, &[(TRAP_DAMAGE_NAME, 4.0)]);
        // Kind 0 (not 0x10): the hit record goes through the 0088e8d0 path,
        // which needs its own callees; stop it at the gate.
        e.register(HIT_EFFECTS_ALLOWED, |_, _| eax(0));
        for address in [
            TIMER_SCOPE_CONSTRUCT,
            TIMER_SCOPE_DESTROY,
            HIT_FLAG_MELEE,
            IMPACT_SECONDARY_FLAG,
        ] {
            double(&mut e, address, Ret::default());
        }
        let construct = record(&mut e, HIT_DATA_CONSTRUCT, Ret::default());
        let timer = record(&mut e, TIMER_SCOPE_CONSTRUCT, Ret::default());
        do_trap_with(&mut e, actor, trap, data);
        assert_eq!(construct.borrow().len(), 1);
        assert_eq!(timer.borrow().len(), 1);
    }

    #[test]
    fn the_minimum_velocity_cancels_damage_for_slow_actors() {
        let mut e = engine3();
        let (actor, trap, data, _) = trap_world(
            &mut e,
            &[(TRAP_DAMAGE_NAME, 4.0), (TRAP_MIN_VELOCITY_NAME, 3.0)],
        );
        // The collision body exists; the velocity length is 1 (below 3 * 2).
        double(&mut e, COLLISION_BODY_OF_FORM, eax(0x7400));
        e.register(VECTOR_LENGTH_FN, |_, _| st0(1.0));
        let damaged = record_slot(&mut e, ACTOR_TABLE, VSLOT_APPLY_DAMAGE, Ret::default());
        do_trap_with(&mut e, actor, trap, data);
        assert!(damaged.borrow().is_empty());
    }

    #[test]
    fn a_dying_actor_is_blown_apart_by_the_death_push() {
        let mut e = engine3();
        let (actor, trap, data, _) = trap_world(
            &mut e,
            &[
                (TRAP_DAMAGE_NAME, 4.0),
                (TRAP_PUSH_BACK_NAME, 20.0),
                (TRAP_DEATH_PUSH_BACK_NAME, 8.0),
            ],
        );
        // Not dead before, dead after: the process virtual +0x22c answers
        // 0 then 1.
        let answers = std::rc::Rc::new(std::cell::RefCell::new(vec![0u32, 1]));
        let order = answers.clone();
        let target = slot_target(ACTOR_TABLE, VSLOT_IS_PROCESS_FLAG_0X22C);
        e.mem
            .set_u32(ACTOR_TABLE + VSLOT_IS_PROCESS_FLAG_0X22C, target);
        e.register_double(target, move |_, _| {
            let mut list = order.borrow_mut();
            eax(if list.len() > 1 {
                list.remove(0)
            } else {
                list[0]
            })
        });
        slot(&mut e, ACTOR_TABLE, VSLOT_GET_3D, eax(0x5555));
        let motion = record(&mut e, WORLD_SET_MOTION, Ret::default());
        let impulse = record(&mut e, ADD_EXPLOSION_IMPULSE, Ret::default());
        let removed = record(&mut e, REMOVE_FORCE, Ret::default());
        do_trap_with(&mut e, actor, trap, data);
        assert_eq!(*motion.borrow(), vec![vec![0x5555, 1, 1, 0, 1]]);
        assert_eq!(*removed.borrow(), vec![vec![0x5555]]);
        assert_eq!(impulse.borrow()[0][0], 0x5555);
        assert_eq!(impulse.borrow()[0][2], 8.0f32.to_bits());
    }

    #[test]
    fn a_living_actor_with_a_controller_gets_a_velocity_modifier() {
        let mut e = engine3();
        let (actor, trap, data, _) = trap_world(
            &mut e,
            &[(TRAP_DAMAGE_NAME, 4.0), (TRAP_PUSH_BACK_NAME, 20.0)],
        );
        let modifier = record(&mut e, SET_VELOCITY_MODIFIER_FN, Ret::default());
        let scaled = record(&mut e, VECTOR_TIMES_SCALAR_ASSIGN, Ret::default());
        do_trap_with(&mut e, actor, trap, data);
        assert_eq!(modifier.borrow().len(), 1);
        assert_eq!(modifier.borrow()[0][0], 0x5500);
        assert_eq!(scaled.borrow()[0][1], 20.0f32.to_bits());
    }
}
