//! `fallout/ai/combat/combatprocedureattackmelee.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006ecd40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the word at `+4` of `this` (`thiscall`, one word). The
/// engine map's name `CombatProcedureAttackMelee::Initialize` belongs to one
/// of the methods folded onto this shared body; `TESQuest::Load` (`0060d104`)
/// uses it on its component at `+0x18` to store the word read from a chunk.
pub fn fn_006ecd40(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr().wrapping_add(4), value);
}

/// `006ecd40` (named `CombatProcedureAttackMelee::Initialize` in the engine map): the base
/// body the wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const STORE_TARGET_006ECD40: u32 = 0x006e_cd40;

// Translated from 009cc4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (vtable word 01090c0c of `CombatProcedureAttackMelee`, between
/// `Update` and `Shutdown`): forwards `this` and one word to `006ecd40`, which stores the word
/// at `this+4`; that body's translation (`units/fallout_ai/combatprocedureattackmelee.rs`)
/// returns nothing, and so does this wrapper.
pub fn fn_009cc4b0(e: &mut Engine, this: Ptr, value: u32) {
    e.call(STORE_TARGET_006ECD40, &args![this, value]);
}

// Translated from 009cdac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the float at +0x60 (loaded into ST0).
pub fn fn_009cdac0(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr().wrapping_add(0x60)))
}

// Translated from 009cdae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the float at +0x64 (loaded into ST0).
pub fn fn_009cdae0(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr().wrapping_add(0x64)))
}

layout! {
    /// `CombatProcedureAttackMelee` (Xbox PDB), 0x88 bytes; the offsets are
    /// the ones the PC code uses.
    pub struct CombatProcedureAttackMelee: 0x88 {
        /// `pCombatController` (Xbox PDB, `CombatProcedure`): `CombatController*`.
        0x04 pCombatController: Ptr,
        /// `eManeuverState` (Xbox PDB): 0 deciding, 1 holding, 2 dodging, 3
        /// moving in, 4 backing away (the cases of `Update`'s switch).
        0x10 eManeuverState: u32,
        /// `eCombatState` (Xbox PDB): 0 attacking, 1 blocking, 2 waiting, 3
        /// deciding (the cases of `009cf570`).
        0x14 eCombatState: u32,
        /// `eLastCombatState` (Xbox PDB).
        0x18 eLastCombatState: u32,
        /// `eCurrentAttack` (Xbox PDB): `ANIM_GROUP_ENUM`, 0xff for none.
        0x1C eCurrentAttack: u32,
        /// `bRangedAllyWantsToShoot` (Xbox PDB).
        0x50 bRangedAllyWantsToShoot: bool,
        /// `iAdvanceMoveMode` (Xbox PDB).
        0x54 iAdvanceMoveMode: i32,
        /// `fCurrentAttackReach` (Xbox PDB): negative until computed.
        0x58 fCurrentAttackReach: f32,
        /// `fFacingTargetDistance` (Xbox PDB): set to -1 each update,
        /// computed on demand.
        0x5C fFacingTargetDistance: f32,
    }
}

/// The embedded members of `CombatProcedureAttackMelee` (Xbox PDB names):
/// the `CombatTimer`s (8 bytes each), the `CombatTimeStamp` and the
/// `CombatMovementRestrictions`.
const MELEE_TIMER: u32 = 0x20;
const MANEUVER_TIMER: u32 = 0x28;
const DODGE_TIMER: u32 = 0x30;
const ADVANCE_TIMER: u32 = 0x38;
const CLOSE_TIMER: u32 = 0x40;
const TARGET_UNREACHABLE_TIMER: u32 = 0x48;
const ATTACK_PICK_FAILED_TIMESTAMP: u32 = 0x60;
const MOVEMENT_RESTRICTIONS: u32 = 0x64;

layout! {
    /// `CombatMovementRestrictions` (Xbox PDB), 0x24 bytes (embedded at
    /// `+0x64` of `CombatProcedureAttackMelee`; the PC code uses the same
    /// offsets).
    pub struct CombatMovementRestrictions: 0x24 {
        /// `bReset` (Xbox PDB).
        0x00 bReset: bool,
        /// `bInitialized` (Xbox PDB).
        0x01 bInitialized: bool,
        /// `cCurrentUpdate` (Xbox PDB): the direction checked next (0..3).
        0x04 cCurrentUpdate: u32,
        /// `sRestrictedMoves` (Xbox PDB): a bit per blocked direction
        /// (1 forward, 2 backward, 4 left, 8 right, as the callers test them).
        0x08 sRestrictedMoves: u16,
        /// `bHasMoveLeftAnim` (Xbox PDB).
        0x0A bHasMoveLeftAnim: bool,
        /// `bHasMoveRightAnim` (Xbox PDB).
        0x0B bHasMoveRightAnim: bool,
        /// `bHasMoveForwardAnim` (Xbox PDB).
        0x0C bHasMoveForwardAnim: bool,
        /// `bHasMoveBackwardAnim` (Xbox PDB).
        0x0D bHasMoveBackwardAnim: bool,
    }
}

/// `Location` (`NiPoint3`) and `UpdateTimer` (`CombatTimer`) of
/// `CombatMovementRestrictions` (Xbox PDB).
const RESTRICTIONS_LOCATION: u32 = 0x10;
const RESTRICTIONS_UPDATE_TIMER: u32 = 0x1c;

// ---- Callees outside this file (called by address) ----

/// `CombatProcedure::GetAttacker` / `GetTarget` (Xbox PDB).
const GET_ATTACKER: u32 = 0x0099_6de0;
const GET_TARGET: u32 = 0x0099_6e00;
/// `CombatProcedure::SetMoveMode` (Xbox PDB, one word).
const SET_MOVE_MODE: u32 = 0x0099_6e20;
/// `CombatProcedure::SetLookAtTarget` (Xbox PDB, two words) and
/// `ClearLookAtTarget`.
const SET_LOOK_AT_TARGET: u32 = 0x0099_6e60;
const CLEAR_LOOK_AT_TARGET: u32 = 0x0099_6ea0;
/// `CombatProcedure::SaveGame` / `LoadGame` (Xbox PDB).
const BASE_SAVE_GAME: u32 = 0x0099_6ed0;
const BASE_LOAD_GAME: u32 = 0x0099_6ef0;
/// `009caac0` (one word): stores the failure reason at `+0xc` and the
/// status 1 at `+8`.
const FAIL_PROCEDURE: u32 = 0x009c_aac0;
/// `0044edb0` (`__thiscall`): the word at `+0x10` (here the maneuver
/// state; the engine map's name `BaseProcess::GetCurrentProcedureIndex` is
/// one of the methods folded onto the body).
const WORD_AT_10: u32 = 0x0044_edb0;
/// `00825c00` (`__thiscall`): the word at `+0x14` (here the combat state).
const WORD_AT_14: u32 = 0x0082_5c00;
/// `00483710` (`__thiscall`, no stack words): returns at once. The update
/// calls it on the object at `011f24e4` (its static initializer names it
/// `"Combat"`), on `this` and on the controller.
const EMPTY_HOOK: u32 = 0x0048_3710;
const COMBAT_HOOK_OBJECT: u32 = 0x011f_24e4;
/// `00586150` (`__thiscall` on the controller): the word at `+0x9c` (the
/// combat state).
const GET_COMBAT_STATE: u32 = 0x0058_6150;
/// `00980070` (`__thiscall` on the controller): the word at `+0xf4` (its
/// combat style).
const CONTROLLER_STYLE: u32 = 0x0098_0070;
/// `007031c0` (`__thiscall` on the controller): the word at `+0xf0` (its
/// current weapon record).
const CONTROLLER_WEAPON: u32 = 0x0070_31c0;
/// `0097ae90` (`__thiscall` on the controller): its actor.
const CONTROLLER_ACTOR: u32 = 0x0097_ae90;
/// `CombatController::IsSwitchingWeapons` (Xbox PDB).
const IS_SWITCHING_WEAPONS: u32 = 0x0098_1970;
/// `0097f2d0`, `00981900`, `009818b0` (`__thiscall` on the controller, `AL`):
/// tests of its combat state (`0097f2f0` and `009373f0` on `+0x9c`).
const CONTROLLER_TEST_F2D0: u32 = 0x0097_f2d0;
const CONTROLLER_TEST_1900: u32 = 0x0098_1900;
const CONTROLLER_TEST_18B0: u32 = 0x0098_18b0;
/// `009818e0` (`__thiscall` on the controller, one word).
const CONTROLLER_SET_18E0: u32 = 0x0098_18e0;
/// `0097fc70` (`__thiscall` on the controller, no stack words).
const CONTROLLER_VALUE_FC70: u32 = 0x0097_fc70;
/// `CombatController::DeterminePowerAttackDeltas` (Xbox PDB).
const DETERMINE_POWER_ATTACK_DELTAS: u32 = 0x0097_ff60;
/// `009800e0` (`__thiscall` on the controller, two words, `AL`).
const CONTROLLER_ATTACK_CHECK: u32 = 0x0098_00e0;
/// `008d8220` (engine map: `MiddleHighProcess::IsCurrentWeaponGrenade`, a
/// folded body): `__thiscall` on the controller, `AL`.
const CONTROLLER_TEST_8220: u32 = 0x008d_8220;
/// `0097fb40` (`__thiscall` on the combat state, one word): copies the word
/// at `+0x9c` to the pointer and returns it.
const STATE_TIMESTAMP_COPY: u32 = 0x0097_fb40;
/// `0097fa50` (`__thiscall` on a `CombatTimer`, `AL`): whether its target
/// time is not 0.
const TIMER_RUNNING: u32 = 0x0097_fa50;
/// `008d7f40` (`__thiscall` on a `CombatTimer`, a `float`): starts it.
const TIMER_START: u32 = 0x008d_7f40;
/// `008d7f80` (`__thiscall` on a `CombatTimer`, `AL`): whether it ran out.
const TIMER_EXPIRED: u32 = 0x008d_7f80;
/// `00910b10` (`__thiscall` on a `CombatTimer`): sets its target time to 0
/// (the engine map's name `TESWeightForm::InitializeDataComponent` belongs
/// to a folded method).
const TIMER_CLEAR: u32 = 0x0091_0b10;
/// `00435e00` (`__thiscall` on a timer or time stamp): the time since its
/// start, in `ST0`.
const TIMER_ELAPSED: u32 = 0x0043_5e00;
/// `006b9130` (`__thiscall` on a `CombatTimer`): its duration, in `ST0`.
const TIMER_DURATION: u32 = 0x006b_9130;
/// `00906f60` (`__thiscall` on a `CombatTimer`): its constructor.
const TIMER_CONSTRUCT: u32 = 0x0090_6f60;
/// `CombatTimer::SaveGame` / `LoadGame` and `CombatTimeStamp::SaveGame` /
/// `LoadGame` (Xbox PDB).
const TIMER_SAVE: u32 = 0x009a_5eb0;
const TIMER_LOAD: u32 = 0x009a_5f20;
const TIMESTAMP_SAVE: u32 = 0x009a_5f90;
const TIMESTAMP_LOAD: u32 = 0x009a_5ff0;
/// `00435dd0` (no words): the current time, in `ST0`; `00435de0`
/// (`__thiscall` on a time stamp, a `float`): stores it.
const TIME_NOW: u32 = 0x0043_5dd0;
const TIMESTAMP_SET: u32 = 0x0043_5de0;
/// `00403e20` (`__thiscall` on a setting): the address of its value.
const SETTING_VALUE: u32 = 0x0040_3e20;
/// `00487f50` (no words): a random `u32`.
const RANDOM_U32: u32 = 0x0048_7f50;
/// `00944460` (`__cdecl`, two words).
const RANDOM_RANGE: u32 = 0x0094_4460;
/// `CombatFormulas::GetRandomBetween` (Xbox PDB): `__cdecl`, two floats,
/// `ST0`.
const RANDOM_BETWEEN: u32 = 0x0064_65f0;
/// `00404010` (`__cdecl`, two floats, `ST0`): the first when the second
/// is less, else the second.
const FLOAT_MAX: u32 = 0x0040_4010;
/// `0040ebd0` (`__cdecl`, two floats, `ST0`).
const FLOAT_PAIR_EBD0: u32 = 0x0040_ebd0;
/// `_ftol2_sse` (`00ec62c0`): `ST0` (a leading `f64`) truncated to `EAX`.
const FTOL: u32 = 0x00ec_62c0;
/// `00403d30` (`__cdecl`, three words: block, byte, count) and `_memset`
/// (`00ec61c0`).
const FILL_BYTES: u32 = 0x0040_3d30;
const MEMSET: u32 = 0x00ec_61c0;
/// `00406d00` (`__cdecl`, buffer, size, format, ...) and `00406d30`
/// (`__cdecl`, buffer, size, text).
const PRINT_TO_BUFFER: u32 = 0x0040_6d00;
const COPY_TO_BUFFER: u32 = 0x0040_6d30;
/// `005b6f70` (no words, `AL`): whether the debug messages below are
/// printed; `00703c00` and `005b5e40` (`__cdecl`, a format and its words)
/// print them.
const DEBUG_MESSAGES_ON: u32 = 0x005b_6f70;
const DEBUG_PRINT: u32 = 0x0070_3c00;
const DEBUG_WARNING: u32 = 0x005b_5e40;
/// `0055d520` (`__thiscall` on a reference): its name.
const REFERENCE_NAME: u32 = 0x0055_d520;
/// `0084e3a0` (`__thiscall` on a form): its form id.
const FORM_ID: u32 = 0x0084_e3a0;
/// `CombatDialogueManager::StartDialogue` (Xbox PDB, four words) on the
/// manager at `011f1708`.
const START_DIALOGUE: u32 = 0x0098_3970;
const DIALOGUE_MANAGER: u32 = 0x011f_1708;
/// The player reference (`PlayerCharacter*` global at `011dea3c`, as other
/// translations read it).
const PLAYER: u32 = 0x011d_ea3c;
/// The word global `011a8244` the attack choice passes to `00495f50`.
const ATTACK_GROUP_WORD: u32 = 0x011a_8244;
/// The default (zero) `NiPoint3` at `011f426c`.
const DEFAULT_POINT: u32 = 0x011f_426c;
/// Power attack groups by direction (five words at `011a5ac8`), the
/// maneuver names (`011a5aa4`) and the combat state names (`011a5ab8`).
const POWER_ATTACK_BY_KIND: u32 = 0x011a_5ac8;
const MANEUVER_NAMES: u32 = 0x011a_5aa4;
const COMBAT_STATE_NAMES: u32 = 0x011a_5ab8;

/// `Actor` and `TESObjectREFR` methods (Xbox PDB names unless noted).
const GET_ANIM_ACTION: u32 = 0x008a_7570;
const GET_BLOCKED: u32 = 0x0089_4d60;
const SET_BLOCK: u32 = 0x0089_4cc0;
const QUEUE_ATTACK: u32 = 0x0089_35f0;
const STOP_MOVING_ACTOR: u32 = 0x008b_3ab0;
const IS_PATH_VALID: u32 = 0x008b_3960;
const IS_PATHING: u32 = 0x008b_3bd0;
const IS_WAITING_ON_PATH: u32 = 0x008b_3bf0;
const IS_PATHING_COMPLETE: u32 = 0x008b_3bb0;
const GET_CURRENT_PATHFINDING_GOAL: u32 = 0x008b_3840;
const SET_PATHFINDING_GOAL: u32 = 0x008b_3630;
const BUILD_REQUEST: u32 = 0x008b_3880;
const FORCE_MOVE_MODE: u32 = 0x008b_3a20;
const GET_FATIGUE_PERCENTAGE: u32 = 0x0089_3530;
const GET_WALK_SPEED: u32 = 0x0088_4dc0;
const GET_ANIM_GROUP: u32 = 0x0089_7910;
const GET_ANIM_GROUP_DURATION: u32 = 0x0089_7b50;
const GET_CURRENT_WEAPON: u32 = 0x008a_1710;
const IS_WEAPON_DRAWN: u32 = 0x008a_16d0;
const GET_RADIUS: u32 = 0x008b_e1b0;
const GET_SCALE: u32 = 0x0056_7400;
const GET_WORLD_LOCATION: u32 = 0x0057_5650;
const GET_WORLD_SPACE: u32 = 0x0057_5d70;
const GET_SAVED_ACQUIRE_OBJECT: u32 = 0x008d_8520;
/// Unnamed `__thiscall` helpers on an actor (no stack words): `008b3b90`
/// (a path test, `AL`), `008b3a50`, `008b3c70` (`AL`), `008846e0` (the
/// movement flags), `004938e0` (`AL`), `00894900` (`AL`: anim action 2..6),
/// `00493bb0` (the byte at `+0x104`), `005a2030` (the byte at `+0x14d`),
/// `00437bd0`, `00437bf0` (`AL`), `00579670` (`AL`), `008be280`,
/// `008be350` (`ST0`), `008d6f30`, `00525420` (`AL`).
const ACTOR_PATH_TEST: u32 = 0x008b_3b90;
const ACTOR_CLEAR_8B3A50: u32 = 0x008b_3a50;
const ACTOR_TEST_8B3C70: u32 = 0x008b_3c70;
const ACTOR_MOVE_FLAGS: u32 = 0x0088_46e0;
const ACTOR_TEST_4938E0: u32 = 0x0049_38e0;
const ACTOR_IS_ATTACKING: u32 = 0x0089_4900;
const ACTOR_BYTE_104: u32 = 0x0049_3bb0;
const ACTOR_BYTE_14D: u32 = 0x005a_2030;
const ACTOR_TEST_437BD0: u32 = 0x0043_7bd0;
const ACTOR_TEST_437BF0: u32 = 0x0043_7bf0;
const ACTOR_TEST_579670: u32 = 0x0057_9670;
const ACTOR_WIDTH: u32 = 0x008b_e280;
const ACTOR_LENGTH: u32 = 0x008b_e350;
const ACTOR_CELL_8D6F30: u32 = 0x008d_6f30;
const PROCEDURE_TEST_525420: u32 = 0x0052_5420;
/// The actor's `ActorValueOwner` (at `+0xa4`):
/// `ActorValueOwner::GetClampedActorValue` (Xbox PDB, one word) and its
/// vtable slot `+0xc` (one word, `ST0`).
const ACTOR_VALUE_OWNER: u32 = 0xa4;
const GET_CLAMPED_ACTOR_VALUE: u32 = 0x0066_ef20;
const SLOT_GET_ACTOR_VALUE: u32 = 0xc;
/// Actor vtable slots the procedure calls (no stack words unless noted).
const SLOT_ANIMATION: u32 = 0x1e4;
const SLOT_POSITION: u32 = 0x1f4;
const SLOT_TARGET_TEST_1D0: u32 = 0x1d0;
const SLOT_TEST_218: u32 = 0x218;
const SLOT_TEST_22C: u32 = 0x22c;
const SLOT_TEST_230: u32 = 0x230;
const SLOT_TEST_234: u32 = 0x234;
const SLOT_VELOCITY: u32 = 0x2ac;
const SLOT_HEADING: u32 = 0x2bc;
const SLOT_COMBAT_CONTROLLER: u32 = 0x428;
/// Slots of the object `GetSavedAcquireObject` returns.
const SLOT_ITEM_148: u32 = 0x148;
const SLOT_TEST_3F8: u32 = 0x3f8;
const SLOT_HAS_MOVE_ANIM: u32 = 0x408;
const SLOT_TEST_610: u32 = 0x610;
/// The debug-text object's print slot (`__cdecl` through the vtable).
const SLOT_PRINT: u32 = 0x14;

/// Animation helpers (on the object of the actor's slot `+0x1e4`):
/// `004301b0` (one word, `AX`), `0070f490` (one word), `00491040` (one
/// word), `00495e40` (one word, `AL`), `00495f50` (three words, `AL`),
/// `00508070` (`ST0`), `Animation::GroupLoaded` (Xbox PDB, one word).
const ANIM_GROUP_OF: u32 = 0x0043_01b0;
const ANIM_STATE_OF: u32 = 0x0070_f490;
const ANIM_SEQUENCE_OF: u32 = 0x0049_1040;
const ANIM_DIRECTION: u32 = 0x0049_5e40;
const ANIM_ATTACK_CHOICE: u32 = 0x0049_5f50;
const ANIM_SPEED: u32 = 0x0050_8070;
const ANIM_GROUP_LOADED: u32 = 0x0049_4710;
/// `LowProcess::GetGenericLocation` (engine map name of `008041a0`, a
/// folded body).
const SEQUENCE_STATE: u32 = 0x0080_41a0;
/// `TESAnimGroup::IsAttackAction`, `IsPowerAttackAction`, `GetType` (Xbox
/// PDB, `__cdecl`, one word) and `005f25d0` (`__cdecl`, one word, `AL`).
const IS_ATTACK_ACTION: u32 = 0x005f_2540;
const IS_POWER_ATTACK_ACTION: u32 = 0x005f_2670;
const ANIM_GROUP_TYPE: u32 = 0x005f_2440;
const ANIM_GROUP_TEST: u32 = 0x005f_25d0;
/// `0051f5f0` (`__thiscall` on a weapon record): the byte at `+0x11d`.
const WEAPON_ATTACK_GROUP: u32 = 0x0051_f5f0;
/// `004c0c30` (`__thiscall` on a weapon): whether its type byte is 3..13.
const WEAPON_TYPE_TEST: u32 = 0x004c_0c30;
/// `ItemChange::GetItemHealth` (Xbox PDB, one word, `ST0`).
const GET_ITEM_HEALTH: u32 = 0x004b_cdb0;
/// `004a8ae0` (engine map: `TESActorBaseData::GetFatigue`, a folded body):
/// `__thiscall` on the `CombatMovementRestrictions`, its restricted moves
/// (`AX`).
const RESTRICTED_MOVES: u32 = 0x004a_8ae0;
/// `009373f0` (`__thiscall` on the `CombatMovementRestrictions`, `AL`).
const RESTRICTIONS_TEST_9373F0: u32 = 0x0093_73f0;
/// `0097f2f0` (`__thiscall` on the combat state, `AL`).
const COMBAT_STATE_TEST_F2F0: u32 = 0x0097_f2f0;

/// Combat style getters (`__thiscall` on the style, `ST0` unless noted).
const STYLE_SCORE_5524B0: u32 = 0x0055_24b0;
const STYLE_FLAG_TEST: u32 = 0x0099_28c0;
const STYLE_HAS_ATTACK: u32 = 0x0050_5b80;
const STYLE_GRENADE_SCALE: u32 = 0x006d_2c20;
const STYLE_CHANCE_73CB80: u32 = 0x0073_cb80;
const STYLE_CHANCE_505C00: u32 = 0x0050_5c00;
const STYLE_CHANCE_505C20: u32 = 0x0050_5c20;
const STYLE_CHANCE_6A9560: u32 = 0x006a_9560;
const STYLE_CHANCE_9E8670: u32 = 0x009e_8670;
const STYLE_DODGE_MIN_639B20: u32 = 0x0063_9b20;
const STYLE_DODGE_51B860: u32 = 0x0051_b860;
const STYLE_DODGE_9A7C40: u32 = 0x009a_7c40;
const STYLE_DODGE_69EF80: u32 = 0x0069_ef80;
const STYLE_HOLD_6A78F0: u32 = 0x006a_78f0;
const STYLE_HOLD_9B0BD0: u32 = 0x009b_0bd0;
const STYLE_BLOCK_TIME_A0: u32 = 0x0045_65f0;
const STYLE_BLOCK_TIME_A4: u32 = 0x0081_2870;

/// `CombatFormulas` helpers of the attack decisions (`__cdecl`).
const FORMULA_ATTACK_SCORE: u32 = 0x0064_4680;
const FORMULA_BLOCK_SCORE: u32 = 0x0064_4580;
const FORMULA_POWER_ATTACK: u32 = 0x0064_4810;
const FORMULA_DODGE_CHANCE: u32 = 0x0064_4990;
const FORMULA_DODGE_DIRECTION: u32 = 0x0064_4a70;

/// `CombatUtilities` (Xbox PDB, `__cdecl`).
const GET_TARGET_FIRING_LOCATION: u32 = 0x009a_8600;
const COMPUTE_FACE_TARGET_LOCATION: u32 = 0x009a_68b0;
const COMPUTE_FACE_TARGET_DISTANCE: u32 = 0x009a_6830;
const GET_WEAPON_REACH: u32 = 0x009a_69c0;
const GET_DISTANCE_BETWEEN_FACING_ACTORS: u32 = 0x009a_64d0;
const GET_ATTACKER_TARGET_EXTENT_SUM: u32 = 0x009a_6770;
const CALCULATE_LOS: u32 = 0x009a_7940;
const IS_ACTOR_USING_RANGED: u32 = 0x009a_96c0;
/// `009a6ae0` (`__cdecl`, three words, `AL`).
const UTILITY_TEST_9A6AE0: u32 = 0x009a_6ae0;
/// `CombatState::AddUnreachableLocation` (Xbox PDB).
const ADD_UNREACHABLE_LOCATION: u32 = 0x009a_0ec0;

/// Point and matrix helpers: `006815c0` (a constructor that returns
/// `this`), `00416870` (`NiPoint3` from three floats), `00439e90` (sum into
/// the out point), `00439ef0` (difference into the out point), `004a7290`
/// (squared length, `ST0`), `00589850` and `00457990` (length, `ST0`),
/// `00460140` (`__thiscall`, a point of the object), `004a0c90` (a matrix
/// from a heading `float`), `004b4500` (matrix times point into the out
/// point).
const POINT_CONSTRUCT: u32 = 0x0068_15c0;
const POINT_SET: u32 = 0x0041_6870;
const POINT_SUM: u32 = 0x0043_9e90;
const POINT_DIFFERENCE: u32 = 0x0043_9ef0;
const POINT_LENGTH_SQUARED: u32 = 0x004a_7290;
const POINT_LENGTH: u32 = 0x0058_9850;
const POINT_LENGTH_457990: u32 = 0x0045_7990;
const RESTRICTIONS_POINT: u32 = 0x0046_0140;
const MATRIX_FROM_HEADING: u32 = 0x004a_0c90;
const MATRIX_TIMES_POINT: u32 = 0x004b_4500;

/// Pathing (Xbox PDB names): `PathingRequest::PathingRequest` and its
/// destructor `006e2620`; `PathingLocation::PathingLocation_ov3` (from an
/// actor), `_ov4` (from a point and an actor), `IsValidLocation`,
/// `GetWorldspace`, `GetCell`, the destructor `004ff7e0`, `005c3420` (its
/// point into the out point); `Pathing::CheckLineOfSight_ov2`.
const PATHING_REQUEST_CONSTRUCT: u32 = 0x006e_2420;
const PATHING_REQUEST_DESTRUCT: u32 = 0x006e_2620;
const PATHING_LOCATION_FROM_ACTOR: u32 = 0x006d_cd70;
const PATHING_LOCATION_FROM_POINT: u32 = 0x006d_ce10;
const PATHING_LOCATION_IS_VALID: u32 = 0x006d_ed70;
const PATHING_LOCATION_WORLDSPACE: u32 = 0x0044_1110;
const PATHING_LOCATION_CELL: u32 = 0x006d_d4f0;
const PATHING_LOCATION_POINT: u32 = 0x005c_3420;
const PATHING_LOCATION_DESTRUCT: u32 = 0x004f_f7e0;
const CHECK_LINE_OF_SIGHT: u32 = 0x006d_7490;
/// The avoid-node array: `00aa13e0` (allocate), `006e3850` (array
/// constructor), `006b37b0` / `006b37e0` (its smart pointer's constructor
/// and destructor), `00559450` (the pointer's object), `006915d0` (an avoid
/// node from a point and two floats), `BSSimpleArray<PathingAvoidNode>::Add`.
const ALLOCATE: u32 = 0x00aa_13e0;
const AVOID_ARRAY_CONSTRUCT: u32 = 0x006e_3850;
const AVOID_POINTER_CONSTRUCT: u32 = 0x006b_37b0;
const AVOID_POINTER_DESTRUCT: u32 = 0x006b_37e0;
const POINTER_GET: u32 = 0x0055_9450;
const AVOID_NODE_CONSTRUCT: u32 = 0x0069_15d0;
const AVOID_ARRAY_ADD: u32 = 0x0090_5450;

/// `BGSSaveGameBuffer` / `BGSLoadGameBuffer` byte writers and readers.
const SAVE_BYTES: u32 = 0x0086_5e50;
const LOAD_BYTES: u32 = 0x0086_4980;

// ---- Settings (named by their static initializers) and constants ----

const SETTING_COMBAT_DISTANCE: u32 = 0x011c_f84c;
const SETTING_COMBAT_STEP_ADVANCE_DISTANCE: u32 = 0x011c_eea8;
const SETTING_COMBAT_MAX_HOLD_SCORE: u32 = 0x011c_ea38;
const SETTING_COMBAT_MIN_ENGAGE_DISTANCE: u32 = 0x011c_eb98;
const SETTING_AI_DODGE_WALK_CHANCE: u32 = 0x011c_e380;
const SETTING_COMBAT_ADVANCE_NORMAL_ATTACK_CHANCE: u32 = 0x011c_f27c;
const SETTING_MOVEMENT_NEAR_TARGET_AVOID_COST: u32 = 0x011c_f120;
const SETTING_MOVEMENT_NEAR_TARGET_AVOID_RADIUS: u32 = 0x011c_f4e0;
const SETTING_AI_UPDATE_MOVEMENT_RESTRICTIONS_DISTANCE: u32 = 0x011c_f968;
const SETTING_MELEE_MOVEMENT_RESTRICTIONS_UPDATE_TIME: u32 = 0x011c_ee60;

/// `float` constants of the exe: -1, 2, 3, 5, 10, 32, 64.
const MINUS_ONE_FLOAT: u32 = 0x0101_2054;
const TWO_FLOAT: u32 = 0x0101_62c0;
const THREE_FLOAT: u32 = 0x0101_7718;
const FIVE_FLOAT: u32 = 0x0101_712c;
const TEN_FLOAT: u32 = 0x0101_7b78;
const THIRTY_TWO_FLOAT: u32 = 0x0101_e340;
const SIXTY_FOUR_FLOAT: u32 = 0x0101_e6ec;
/// `double` constants of the exe: 0.5, 1, 1.25, 2, 2.25, 5, 10, 25, 100,
/// and the factor at `01090d68` (0.66, read from the exe), and the `double`
/// 0 at `01012060` the compares with 0 read.
const HALF_DOUBLE: u32 = 0x0101_1588;
const ZERO_DOUBLE: u32 = 0x0101_2060;
const ONE_DOUBLE: u32 = 0x0101_2070;
const ONE_AND_A_QUARTER_DOUBLE: u32 = 0x0102_1798;
const TWO_DOUBLE: u32 = 0x0101_1590;
const REACH_FACTOR_DOUBLE: u32 = 0x0108_d6e8;
const FIVE_DOUBLE: u32 = 0x0102_0998;
const TEN_DOUBLE: u32 = 0x0102_0758;
const TWENTY_FIVE_DOUBLE: u32 = 0x0104_f2f0;
const HUNDRED_DOUBLE: u32 = 0x0101_7a40;
const POWER_ATTACK_MARGIN_DOUBLE: u32 = 0x0109_0d68;

// ---- Strings ----

const TEXT_COULD_NOT_REACH_TARGET: u32 = 0x0109_0c48;
const TEXT_NO_DODGE_ANIMATION: u32 = 0x0109_0c60;
const TEXT_BACK: u32 = 0x0109_0c9c;
const TEXT_FOREWARD: u32 = 0x0109_0ca4;
const TEXT_RIGHT: u32 = 0x0109_0cb0;
const TEXT_LEFT: u32 = 0x0109_0cb8;
const TEXT_NO_ROOM_TO_DODGE: u32 = 0x0109_0cc0;
const TEXT_SELECTS_POWER_ATTACK: u32 = 0x0109_0ce4;
const TEXT_NO: u32 = 0x0109_0d04;
const TEXT_BACKWARD: u32 = 0x0109_0d08;
const TEXT_FORWARD: u32 = 0x0109_0d14;
const TEXT_NORMAL: u32 = 0x0109_0d1c;
const TEXT_POWER_ATTACK_UNUSABLE: u32 = 0x0109_0d24;
const TEXT_RESET_RESTRICTIONS: u32 = 0x0109_0d70;
const TEXT_RESTRICTIONS: u32 = 0x0109_0d8c;
const TEXT_MELEE: u32 = 0x0109_0db8;
const TEXT_UNREACHABLE_TIMER: u32 = 0x0109_0dc8;
const TEXT_RIGHT_POWER: u32 = 0x0109_0dec;
const TEXT_LEFT_POWER: u32 = 0x0109_0df8;
const TEXT_BACK_POWER: u32 = 0x0109_0e04;
const TEXT_FORWARD_POWER: u32 = 0x0109_0e10;
const TEXT_NORMAL_POWER: u32 = 0x0109_0e20;
const TEXT_MANEUVER: u32 = 0x0109_0e30;
const TEXT_TIMER: u32 = 0x0109_0e40;
const TEXT_NAMED_TIMER: u32 = 0x0109_0e4c;
const TEXT_SPACE_RIGHT: u32 = 0x0109_0e5c;
const TEXT_SPACE_LEFT: u32 = 0x0109_0e64;
const TEXT_SPACE_BACKWARD: u32 = 0x0109_0e6c;
const TEXT_SPACE_FORWARD: u32 = 0x0109_0e78;
const TEXT_EMPTY: u32 = 0x0101_1584;
const TEXT_N: u32 = 0x0101_7b64;
const TEXT_Y: u32 = 0x0101_7b68;

/// The bit pattern of the `float` a setting holds.
fn setting_bits(e: &mut Engine, setting: u32) -> u32 {
    let value = e.call(SETTING_VALUE, &args![setting]).u32();
    e.mem.u32(value)
}

/// A `double` constant of the exe (as a memory operand).
fn double_at(e: &Engine, addr: u32) -> f64 {
    e.global::<f64>(addr)
}

/// A random number below 100 as the `float` the code makes of it
/// (`DIV` by 100, then `FILD qword` of the remainder and `FSTP dword`).
fn random_percent(e: &mut Engine) -> u32 {
    let random = e.call(RANDOM_U32, &args![]).u32();
    x87_store32(f64::from(random % 100))
}

fn controller_of(e: &Engine, this: Ptr<CombatProcedureAttackMelee>) -> u32 {
    e.get(this, CombatProcedureAttackMelee::pCombatController)
        .addr()
}

fn restrictions_of(this: Ptr<CombatProcedureAttackMelee>) -> Ptr<CombatMovementRestrictions> {
    Ptr::new(this.addr() + MOVEMENT_RESTRICTIONS)
}

/// The combat style of the procedure's controller (`00980070`).
fn style_of(e: &mut Engine, this: Ptr<CombatProcedureAttackMelee>) -> u32 {
    let controller = controller_of(e, this);
    e.call(CONTROLLER_STYLE, &args![controller]).u32()
}

/// A style getter's `float` (`ST0`) stored as a `float`.
fn style_float(e: &mut Engine, this: Ptr<CombatProcedureAttackMelee>, getter: u32) -> u32 {
    let style = style_of(e, this);
    x87_store32(e.call(getter, &args![style]).f64())
}

/// `CombatFormulas::GetRandomBetween` of two `float` bit patterns, stored
/// as a `float`.
fn random_between(e: &mut Engine, low: u32, high: u32) -> u32 {
    x87_store32(e.call(RANDOM_BETWEEN, &args![low, high]).f64())
}

/// Any of the target tests the attack decisions combine: its slots
/// `+0x234` and `+0x230`, then `00437bd0`, `00579670` and `00437bf0`, in
/// that order (the first that holds ends it).
fn target_is_busy(e: &mut Engine, target: u32) -> bool {
    e.vcall(target, SLOT_TEST_234, &args![]).bool()
        || e.vcall(target, SLOT_TEST_230, &args![]).bool()
        || e.call(ACTOR_TEST_437BD0, &args![target]).bool()
        || e.call(ACTOR_TEST_579670, &args![target]).bool()
        || e.call(ACTOR_TEST_437BF0, &args![target]).bool()
}

/// Whether the attacker's animation (slot `+0x1e4`) has a sequence `2`
/// whose state (`008041a0`) is not 1: the test `fn_009ccfb0` and
/// `fn_009cd150` start with (each step asks for the attacker again).
fn attacker_sequence_busy(e: &mut Engine, this: Ptr<CombatProcedureAttackMelee>) -> bool {
    let attacker = e.call(GET_ATTACKER, &args![this]).u32();
    if e.vcall(attacker, SLOT_ANIMATION, &args![]).u32() == 0 {
        return false;
    }
    let attacker = e.call(GET_ATTACKER, &args![this]).u32();
    let animation = e.vcall(attacker, SLOT_ANIMATION, &args![]).u32();
    if e.call(ANIM_SEQUENCE_OF, &args![animation, 2u32]).u32() == 0 {
        return false;
    }
    let attacker = e.call(GET_ATTACKER, &args![this]).u32();
    let animation = e.vcall(attacker, SLOT_ANIMATION, &args![]).u32();
    let sequence = e.call(ANIM_SEQUENCE_OF, &args![animation, 2u32]).u32();
    e.call(SEQUENCE_STATE, &args![sequence]).u32() != 1
}

/// Releases the block and ends blocking: the tail `fn_009cd150` shares
/// (`SetBlock(false)` when blocked, `009818e0(controller, 0)`, and the
/// combat state 3 when it was 1).
fn stop_blocking(e: &mut Engine, this: Ptr<CombatProcedureAttackMelee>, attacker: u32) {
    if e.call(GET_BLOCKED, &args![attacker]).bool() {
        e.call(SET_BLOCK, &args![attacker, 0u32]);
    }
    let controller = controller_of(e, this);
    e.call(CONTROLLER_SET_18E0, &args![controller, 0u32]);
    if e.call(WORD_AT_14, &args![this]).u32() == 1 {
        fn_009cf6d0(e, this, 3);
    }
}

// Translated from 009cc240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureAttackMelee::Update` (Xbox PDB): resets the facing
/// distance, updates the movement restrictions and the combat state
/// (`009cf570`); while moving in or backing away it watches the path and,
/// once the target-unreachable timer ran out, marks the target's location
/// unreachable and fails ("Could not reach target"). Then it runs the
/// maneuver state's step (0..4) and looks at the target (its firing
/// location) unless it is attacking within reach.
pub fn update(e: &mut Engine, this: Ptr<CombatProcedureAttackMelee>) {
    let t = this.addr();
    let attacker = e.call(GET_ATTACKER, &args![t]).u32();
    let target = e.call(GET_TARGET, &args![t]).u32();
    let minus_one = x87_quiet_bits(e.global::<u32>(MINUS_ONE_FLOAT));
    e.mem.set_u32(t + 0x5c, minus_one);
    e.call(EMPTY_HOOK, &args![COMBAT_HOOK_OBJECT]);
    let controller = controller_of(e, this);
    fn_009d0610(e, restrictions_of(this), controller);
    fn_009cf570(e, this);
    e.call(EMPTY_HOOK, &args![t]);

    let unreachable_timer = t + TARGET_UNREACHABLE_TIMER;
    let maneuver = e.call(WORD_AT_10, &args![t]).u32();
    let moving = maneuver == 3 || e.call(WORD_AT_10, &args![t]).u32() == 4;
    if moving {
        let path_failed = e.call(ACTOR_PATH_TEST, &args![attacker]).bool();
        if path_failed && !e.call(IS_PATH_VALID, &args![attacker]).bool() {
            if !e.call(TIMER_RUNNING, &args![unreachable_timer]).bool() {
                let two = x87_quiet_bits(e.global::<u32>(TWO_FLOAT));
                e.call(TIMER_START, &args![unreachable_timer, two]);
            } else if e.call(TIMER_EXPIRED, &args![unreachable_timer]).bool() {
                let controller = controller_of(e, this);
                let combat_state = e.call(GET_COMBAT_STATE, &args![controller]).u32();
                fn_009cc490(e, combat_state, 2);
                e.with_stack(0x10, |e, location| {
                    let location = e.call(GET_WORLD_LOCATION, &args![target, location]).u32();
                    let controller = controller_of(e, this);
                    let combat_state = e.call(GET_COMBAT_STATE, &args![controller]).u32();
                    e.call(ADD_UNREACHABLE_LOCATION, &args![combat_state, location]);
                });
                e.call(FAIL_PROCEDURE, &args![t, TEXT_COULD_NOT_REACH_TARGET]);
            }
        } else if e.call(IS_PATH_VALID, &args![attacker]).bool() {
            e.call(TIMER_CLEAR, &args![unreachable_timer]);
        }
    } else {
        e.call(TIMER_CLEAR, &args![unreachable_timer]);
    }

    // The compiler's switch on the maneuver state (table `009cc474`).
    match e.call(WORD_AT_10, &args![t]).u32() {
        0 => fn_009cd430(e, this),
        1 => fn_009cde10(e, this),
        2 => handle_dodge(e, this),
        3 => fn_009cdf80(e, this),
        4 => fn_009ce1d0(e, this),
        _ => {}
    }

    let look = if e.get(this, CombatProcedureAttackMelee::eCombatState) != 3
        || e.get(this, CombatProcedureAttackMelee::eManeuverState) == 3
    {
        true
    } else {
        let facing = get_facing_target_distance(e, this).to_bits();
        let reach = get_current_attack_reach(e, this).to_bits();
        let margin = x87_sub(x87_load32(facing), x87_m32(reach));
        let distance = setting_bits(e, SETTING_COMBAT_DISTANCE);
        let scale = e.call(GET_SCALE, &args![attacker]).f64();
        let limit = x87_mul(
            x87_mul(scale, x87_m32(distance)),
            double_at(e, REACH_FACTOR_DOUBLE),
        );
        // FCOMP margin; TEST AH,0x41; JNZ: looks when the limit is greater.
        limit > margin
    };
    if look {
        e.with_stack(0xc, |e, point| {
            e.call(GET_TARGET_FIRING_LOCATION, &args![point, target, 2u32]);
            e.call(SET_LOOK_AT_TARGET, &args![t, point, 4u32]);
        });
    } else {
        e.call(CLEAR_LOOK_AT_TARGET, &args![t]);
    }
    e.call(EMPTY_HOOK, &args![COMBAT_HOOK_OBJECT]);
}

// Translated from 009cc490 (decompiled, FalloutNV.exe 1.4.0.525)
/// ORs `mask` into the word at `+4` (`__thiscall`, one word; `Update`
/// uses it on the controller's combat state).
pub fn fn_009cc490(e: &mut Engine, this: u32, mask: u32) {
    let word = e.mem.u32(this + 4);
    e.mem.set_u32(this + 4, word | mask);
}

// Translated from 009cc4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureAttackMelee::Shutdown` (Xbox PDB): clears the look-at
/// target, stops the attacker and releases its block.
pub fn shutdown(e: &mut Engine, this: Ptr<CombatProcedureAttackMelee>) {
    let attacker = e.call(GET_ATTACKER, &args![this]).u32();
    e.call(CLEAR_LOOK_AT_TARGET, &args![this]);
    e.call(STOP_MOVING_ACTOR, &args![attacker]);
    e.call(SET_BLOCK, &args![attacker, 0u32]);
}

// Translated from 009cc510 (decompiled, FalloutNV.exe 1.4.0.525)
/// The attack decision of the combat state 3 (unnamed in the map): with a
/// target within reach, it scores attacking (`00644680`, plus 100 when the
/// combat state's time stamp is older than 5 and the score not 0, plus 100
/// against a non-player target whose byte `+0x104` is clear), blocking
/// (`00644580`) and holding (100 minus both, at least
/// `fCombatMaxHoldScore`), zeroing the scores the actors' states rule out,
/// and draws a number below their sum (100 when the sum is not positive):
/// below the attack score it picks an attack (`009ccb20`), starts the
/// combat dialogue and the attack; below attack plus block it blocks;
/// otherwise it waits (combat state 2) for a random time.
pub fn fn_009cc510(e: &mut Engine, this: Ptr<CombatProcedureAttackMelee>) {
    let t = this.addr();
    let attacker = e.call(GET_ATTACKER, &args![t]).u32();
    let target = e.call(GET_TARGET, &args![t]).u32();
    if e.call(WORD_AT_14, &args![t]).u32() != 3 || target == 0 {
        return;
    }
    let controller = controller_of(e, this);
    if e.call(IS_SWITCHING_WEAPONS, &args![controller]).bool() {
        return;
    }
    e.set(this, CombatProcedureAttackMelee::eCurrentAttack, 0xff);
    let facing = get_facing_target_distance(e, this);
    let reach = get_current_attack_reach(e, this);
    let animation = e.vcall(attacker, SLOT_ANIMATION, &args![]).u32();
    let attacker_action = e.call(GET_ANIM_ACTION, &args![attacker]).u32();
    let target_action = e.call(GET_ANIM_ACTION, &args![target]).u32();
    let group = e.call(ANIM_GROUP_OF, &args![animation, 4u32]).u16();
    let attacking = e.call(IS_ATTACK_ACTION, &args![u32::from(group)]).bool();
    let attack_state_3 = e.call(ANIM_STATE_OF, &args![animation, 4u32]).u32() == 3;
    let attacker_recoiling = attacker_action == 8 || attacker_action == 0xa;
    let target_attacking = e.call(ACTOR_IS_ATTACKING, &args![target]).bool();
    let target_action_4 = target_action == 4;
    let target_recoiling = target_action == 8 || target_action == 0xa;
    let target_byte_104 = e.call(ACTOR_BYTE_104, &args![target]).u8() != 0;
    let target_blocked = e.call(GET_BLOCKED, &args![target]).bool();
    if !fn_009cefd0(e, this, facing, reach, 0) {
        return;
    }

    let slot_230 = e.vcall(target, SLOT_TEST_230, &args![]).bool();
    let controller = controller_of(e, this);
    let no_weapon = e.call(CONTROLLER_WEAPON, &args![controller]).u32() == 0;
    let value = e
        .call(
            GET_CLAMPED_ACTOR_VALUE,
            &args![attacker + ACTOR_VALUE_OWNER, 0xbu32],
        )
        .u32();
    let controller = controller_of(e, this);
    let fc70 = e.call(CONTROLLER_VALUE_FC70, &args![controller]).u32();
    let style = style_of(e, this);
    let mut attack_score = x87_store32(
        e.call(
            FORMULA_ATTACK_SCORE,
            &args![
                style,
                fc70,
                value,
                u32::from(target_attacking),
                u32::from(no_weapon),
                u32::from(target_recoiling),
                u32::from(slot_230),
                u32::from(target_blocked)
            ],
        )
        .f64(),
    );
    let since = e.with_stack(4, |e, stamp| {
        let controller = controller_of(e, this);
        let combat_state = e.call(GET_COMBAT_STATE, &args![controller]).u32();
        let copy = e
            .call(STATE_TIMESTAMP_COPY, &args![combat_state, stamp])
            .u32();
        e.call(TIMER_ELAPSED, &args![copy]).f64()
    });
    let hundred = double_at(e, HUNDRED_DOUBLE);
    // FCOMP 5.0; TEST AH,0x41; JNZ: only when the time is greater.
    if since > double_at(e, FIVE_DOUBLE) && x87_load32(attack_score) != double_at(e, ZERO_DOUBLE) {
        attack_score = x87_store32(x87_add(x87_load32(attack_score), hundred));
    }
    if !target_byte_104 && target != e.global::<u32>(PLAYER) {
        attack_score = x87_store32(x87_add(x87_load32(attack_score), hundred));
    }
    let style = style_of(e, this);
    let mut block_score = x87_store32(
        e.call(
            FORMULA_BLOCK_SCORE,
            &args![style, 0x32u32, 0x32u32, 0u32, u32::from(target_attacking)],
        )
        .f64(),
    );
    let target_busy = target_is_busy(e, target);
    let weapon = e.call(GET_CURRENT_WEAPON, &args![target]).u32();
    let zero_block = target_busy
        || !e.call(IS_WEAPON_DRAWN, &args![target]).bool()
        || (weapon != 0 && e.call(WEAPON_TYPE_TEST, &args![weapon]).bool());
    if zero_block {
        block_score = 0;
    }
    let can_attack = if attacking && x87_load32(attack_score) > double_at(e, ZERO_DOUBLE) {
        true
    } else {
        let attacker = e.call(GET_ATTACKER, &args![t]).u32();
        let object = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![attacker]).u32();
        e.vcall(object, SLOT_TEST_3F8, &args![]).bool()
    };
    let keep = can_attack && {
        let controller = controller_of(e, this);
        e.call(CONTROLLER_TEST_F2D0, &args![controller]).bool()
    };
    if !keep {
        attack_score = 0;
        let controller = controller_of(e, this);
        if !e.call(CONTROLLER_TEST_F2D0, &args![controller]).bool() {
            block_score = 0;
        }
    }
    if (attacking && !attack_state_3)
        || e.call(WORD_AT_10, &args![t]).u32() == 4
        || attacker_recoiling
    {
        block_score = 0;
    }
    if !e.call(ANIM_GROUP_LOADED, &args![animation, 0xaau32]).bool() {
        block_score = 0;
    }
    let max_hold = setting_bits(e, SETTING_COMBAT_MAX_HOLD_SCORE);
    let rest = x87_store32(x87_sub(
        hundred,
        x87_add(x87_load32(attack_score), x87_m32(block_score)),
    ));
    let mut hold_score = x87_store32(e.call(FLOAT_MAX, &args![rest, max_hold]).f64());
    if attacking && !attack_state_3 {
        hold_score = 0;
    }
    if e.get(this, CombatProcedureAttackMelee::eLastCombatState) == 2 {
        hold_score = 0;
    }
    let total = x87_add(
        x87_add(x87_load32(attack_score), x87_m32(block_score)),
        x87_m32(hold_score),
    );
    let total = e.call(FTOL, &args![total]).i32();
    let range = if total > 0 { total as u32 } else { 100 };
    let random = e.call(RANDOM_U32, &args![]).u32();
    let roll = x87_store32(f64::from(random % range));

    // FLD roll; FLD attack; FCOMPP; TEST AH,0x41; JNZ: attacks when the
    // attack score is greater.
    if x87_load32(attack_score) > x87_load32(roll) {
        let controller = controller_of(e, this);
        if e.call(CONTROLLER_TEST_1900, &args![controller]).bool() {
            e.call(SET_BLOCK, &args![attacker, 0u32]);
            if e.call(WORD_AT_14, &args![t]).u32() == 1 {
                fn_009cf6d0(e, this, 3);
            }
        }
        e.with_stack(8, |e, flags| {
            let shout = flags.addr();
            let power = shout + 4;
            let attack = fn_009ccb20(
                e,
                this,
                attacking,
                facing,
                reach,
                target_attacking,
                target_action_4,
                target_recoiling,
                Ptr::new(shout),
                Ptr::new(power),
            );
            if attack == 0xff {
                return;
            }
            let target = e.call(GET_TARGET, &args![t]).u32();
            if e.vcall(target, SLOT_TARGET_TEST_1D0, &args![]).u32() == 0 {
                return;
            }
            let manager = e.global::<u32>(DIALOGUE_MANAGER);
            let controller = controller_of(e, this);
            let topic = u32::from(e.mem.u8(shout) != 0);
            e.call(
                START_DIALOGUE,
                &args![manager, controller, 2u32, topic, 0u32],
            );
            let power_flag = u32::from(e.mem.u8(power));
            start_attack(e, this, attack, power_flag);
        });
    } else if x87_load32(block_score) > double_at(e, ZERO_DOUBLE)
        && x87_load32(roll) < x87_add(x87_load32(attack_score), x87_m32(block_score))
    {
        e.call(SET_BLOCK, &args![attacker, 1u32]);
        fn_009cf6d0(e, this, 1);
    } else if e.get(this, CombatProcedureAttackMelee::eLastCombatState) != 2 {
        fn_009cf6d0(e, this, 2);
        let high = style_float(e, this, STYLE_BLOCK_TIME_A4);
        let low = style_float(e, this, STYLE_BLOCK_TIME_A0);
        let wait = random_between(e, low, high);
        e.call(TIMER_START, &args![t + MELEE_TIMER, wait]);
    }
}

// Translated from 009ccb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Picks the attack (an `ANIM_GROUP_ENUM`, 0xff for none; unnamed in the
/// map; eight stack words). `*shout` gets whether the attacker may shout
/// (`00644810`, or 0 when its slot `+0x218` and byte `+0x14d` say so) and
/// `*power` whether the attack is a power attack. Not attacking yet: a
/// power attack when shouting (`SelectPowerAttack`), else the weapon's
/// attack group, else group `0x1a` or `0x20` (by `00495f50`). Attacking
/// with the animation state 3: a power attack, else the direction's power
/// group (`00495e40`: `0x33..0x38` give `0x26..0x44` in steps of 6,
/// `0x6c` gives `0x1a`), else the weapon's group or `0x20`. While backing
/// away the reach grows by `fCombatStepAdvanceDistance` (and by 100 in 2 of
/// 100 draws).
pub fn fn_009ccb20(
    e: &mut Engine,
    this: Ptr<CombatProcedureAttackMelee>,
    attacking: bool,
    facing: f32,
    reach: f32,
    target_attacking: bool,
    target_action_4: bool,
    target_recoiling: bool,
    shout: Ptr<u8>,
    power: Ptr<u8>,
) -> u32 {
    let t = this.addr();
    let attacker = e.call(GET_ATTACKER, &args![t]).u32();
    let target = e.call(GET_TARGET, &args![t]).u32();
    e.mem.set_u8(power.addr(), 0);
    let mut attack = 0xffu32;
    let controller = controller_of(e, this);
    let weapon = e.call(CONTROLLER_WEAPON, &args![controller]).u32();
    if target == 0 {
        return attack;
    }
    let no_shout = e.vcall(attacker, SLOT_TEST_218, &args![]).bool()
        && e.call(ACTOR_BYTE_14D, &args![attacker]).bool();
    if no_shout {
        e.mem.set_u8(shout.addr(), 0);
    } else {
        let slot_230 = e.vcall(target, SLOT_TEST_230, &args![]).bool();
        let fatigue = e.call(GET_FATIGUE_PERCENTAGE, &args![attacker]).f64();
        let style = style_of(e, this);
        let value = e
            .call(
                FORMULA_POWER_ATTACK,
                &args![
                    style,
                    x87_store32(fatigue),
                    u32::from(target_recoiling),
                    u32::from(slot_230),
                    0u32
                ],
            )
            .u8();
        e.mem.set_u8(shout.addr(), value);
    }
    let mut reach = reach.to_bits();
    if e.call(WORD_AT_10, &args![t]).u32() == 4 {
        let step = setting_bits(e, SETTING_COMBAT_STEP_ADVANCE_DISTANCE);
        reach = x87_store32(x87_add(x87_load32(reach), x87_m32(step)));
        let random = e.call(RANDOM_U32, &args![]).u32();
        if random % 100 < 2 {
            reach = x87_store32(x87_add(x87_load32(reach), double_at(e, HUNDRED_DOUBLE)));
        }
    }
    let in_reach = fn_009cefd0(e, this, facing, f32::from_bits(reach), 0);
    if !attacking {
        if e.mem.u8(shout.addr()) != 0 {
            attack = select_power_attack(
                e,
                this,
                facing,
                f32::from_bits(reach),
                u32::from(target_attacking),
                u32::from(target_action_4),
                0,
            );
        }
        if attack == 0xff && in_reach {
            attack = if weapon == 0 {
                0xff
            } else {
                e.call(WEAPON_ATTACK_GROUP, &args![weapon]).u32()
            };
            if attack == 0xff {
                let group = e.global::<u32>(ATTACK_GROUP_WORD);
                let animation = e.vcall(attacker, SLOT_ANIMATION, &args![]).u32();
                let choice = e
                    .call(ANIM_ATTACK_CHOICE, &args![animation, 1u32, group, 0u32])
                    .u8() as i8;
                attack = if choice as i32 == 0x6c { 0x1a } else { 0x20 };
            }
        }
        return attack;
    }
    let animation = e.vcall(attacker, SLOT_ANIMATION, &args![]).u32();
    if e.call(ANIM_STATE_OF, &args![animation, 4u32]).u32() != 3 {
        return attack;
    }
    if e.mem.u8(shout.addr()) != 0 {
        attack = select_power_attack(
            e,
            this,
            facing,
            f32::from_bits(reach),
            u32::from(target_attacking),
            u32::from(target_action_4),
            0,
        );
    }
    if attack != 0xff || !in_reach {
        return attack;
    }
    let attacker = e.call(GET_ATTACKER, &args![t]).u32();
    let animation = e.vcall(attacker, SLOT_ANIMATION, &args![]).u32();
    let direction = e.call(ANIM_DIRECTION, &args![animation, 0u32]).u8() as i8;
    // The compiler's switch on `direction - 0x33` (byte map `009cce30`,
    // jump table `009cce10`).
    attack = match (direction as i32).wrapping_sub(0x33) as u32 {
        0 => 0x26,
        1 => 0x2c,
        2 => 0x32,
        3 => 0x38,
        4 => 0x3e,
        5 => 0x44,
        0x39 => 0x1a,
        _ => {
            if weapon != 0 && e.call(WEAPON_ATTACK_GROUP, &args![weapon]).u32() != 0xff {
                e.call(WEAPON_ATTACK_GROUP, &args![weapon]).u32()
            } else {
                0x20
            }
        }
    };
    e.mem.set_u8(power.addr(), 1);
    attack
}

// Translated from 009cce70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureAttackMelee::StartAttack` (Xbox PDB): unless the attack
/// is 0xff or the combat state is already 0, queues the attack on the
/// attacker; when it was queued the combat state becomes 0 (attacking) and
/// the attack the current one. The second stack word is not read.
pub fn start_attack(
    e: &mut Engine,
    this: Ptr<CombatProcedureAttackMelee>,
    attack: u32,
    _unused_2: u32,
) {
    if attack == 0xff || e.call(WORD_AT_14, &args![this]).u32() == 0 {
        return;
    }
    let attacker = e.call(GET_ATTACKER, &args![this]).u32();
    if e.call(QUEUE_ATTACK, &args![attacker, attack]).bool() {
        fn_009cf6d0(e, this, 0);
        e.set(this, CombatProcedureAttackMelee::eCurrentAttack, attack);
    }
}

// Translated from 009cced0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureAttackMelee::HandleAttack` (Xbox PDB), the combat state
/// 0: while the attacker's animation still plays an attack action, a
/// finished swing (state 3) goes back to deciding (combat state 3 and
/// `009cc510`), and a weapon with an attack group that `005f25d0` accepts
/// queues that attack; without an attack action the combat state goes back
/// to 3.
pub fn handle_attack(e: &mut Engine, this: Ptr<CombatProcedureAttackMelee>) {
    let t = this.addr();
    if e.call(WORD_AT_14, &args![t]).u32() != 0 {
        return;
    }
    let attacker = e.call(GET_ATTACKER, &args![t]).u32();
    let _target = e.call(GET_TARGET, &args![t]).u32();
    let animation = e.vcall(attacker, SLOT_ANIMATION, &args![]).u32();
    let group = e.call(ANIM_GROUP_OF, &args![animation, 4u32]).u16();
    if !e.call(IS_ATTACK_ACTION, &args![u32::from(group)]).bool() {
        fn_009cf6d0(e, this, 3);
        return;
    }
    let animation = e.vcall(attacker, SLOT_ANIMATION, &args![]).u32();
    if e.call(ANIM_STATE_OF, &args![animation, 4u32]).u32() == 3 {
        fn_009cf6d0(e, this, 3);
        fn_009cc510(e, this);
    }
    let controller = controller_of(e, this);
    let weapon = e.call(CONTROLLER_WEAPON, &args![controller]).u32();
    if weapon != 0 {
        let group = e.call(WEAPON_ATTACK_GROUP, &args![weapon]).u32();
        if e.call(ANIM_GROUP_TEST, &args![group]).bool() {
            let group = e.call(WEAPON_ATTACK_GROUP, &args![weapon]).u32();
            e.call(QUEUE_ATTACK, &args![attacker, group]);
        }
    }
}

// Translated from 009ccfb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The combat state 1, blocking (unnamed in the map): back to deciding
/// when the attacker no longer blocks; with the item of its saved acquire
/// object (slot `+0x148`): a broken item (health at most 0) or a
/// recoiling target (anim action 8) ends the block unless the attacker's
/// sequence is busy; otherwise `fn_009cd150` decides whether to keep it.
pub fn fn_009ccfb0(e: &mut Engine, this: Ptr<CombatProcedureAttackMelee>) {
    let t = this.addr();
    if e.call(WORD_AT_14, &args![t]).u32() != 1 {
        return;
    }
    let attacker = e.call(GET_ATTACKER, &args![t]).u32();
    let target = e.call(GET_TARGET, &args![t]).u32();
    if !e.call(GET_BLOCKED, &args![attacker]).bool() {
        fn_009cf6d0(e, this, 3);
    }
    let object = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![attacker]).u32();
    let item = e.vcall(object, SLOT_ITEM_148, &args![]).u32();
    if item == 0 {
        return;
    }
    let busy = attacker_sequence_busy(e, this);
    let health = x87_store32(e.call(GET_ITEM_HEALTH, &args![item, 0u32]).f64());
    // FCOMP 0.0; TEST AH,0x41; JP: a health below or equal to 0.
    if x87_load32(health) <= double_at(e, ZERO_DOUBLE) && !busy {
        e.call(SET_BLOCK, &args![attacker, 0u32]);
        fn_009cf6d0(e, this, 3);
        return;
    }
    let target_recoiling = target != 0 && e.call(GET_ANIM_ACTION, &args![target]).u32() == 8;
    if target_recoiling && !busy {
        e.call(SET_BLOCK, &args![attacker, 0u32]);
        fn_009cf6d0(e, this, 3);
        fn_009cf570(e, this);
        return;
    }
    if !busy {
        fn_009cd150(e, this);
    }
}

// Translated from 009cd150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether to keep blocking (unnamed in the map): nothing while the
/// attacker's sequence is busy, or when the controller's test `00981900`
/// fails and the combat state is not 1. Without a target, or with the
/// target beyond reach, busy, swinging (anim action 4) or blocked by its
/// own controller, the block ends; otherwise it ends in 5 of 100 draws
/// unless the target attacks, and only when the attacker no longer blocks.
pub fn fn_009cd150(e: &mut Engine, this: Ptr<CombatProcedureAttackMelee>) {
    let t = this.addr();
    if attacker_sequence_busy(e, this) {
        return;
    }
    let attacker = e.call(GET_ATTACKER, &args![t]).u32();
    let target = e.call(GET_TARGET, &args![t]).u32();
    let controller = controller_of(e, this);
    if !e.call(CONTROLLER_TEST_1900, &args![controller]).bool()
        && e.call(WORD_AT_14, &args![t]).u32() != 1
    {
        return;
    }
    if target == 0 {
        stop_blocking(e, this, attacker);
        return;
    }
    let target_attacking = e.call(ACTOR_IS_ATTACKING, &args![target]).bool();
    let target_swinging = e.call(GET_ANIM_ACTION, &args![target]).u32() == 4;
    let mut controller_blocks = false;
    if e.vcall(target, SLOT_COMBAT_CONTROLLER, &args![]).u32() != 0 {
        let target_controller = e.vcall(target, SLOT_COMBAT_CONTROLLER, &args![]).u32();
        controller_blocks = e
            .call(CONTROLLER_TEST_1900, &args![target_controller])
            .bool();
    }
    let facing = get_facing_target_distance(e, this);
    let reach = get_current_attack_reach(e, this);
    // A local the function clears and never sets.
    let keep_anyway = false;
    let busy = target_is_busy(e, target);
    // FLD facing; FLD reach; FCOMPP; TEST AH,5; JNP: ends when the reach is
    // less than the facing distance.
    let ends = if x87_load32(reach.to_bits()) < x87_load32(facing.to_bits())
        || busy
        || target_swinging
        || controller_blocks
    {
        true
    } else {
        let random = e.call(RANDOM_U32, &args![]).u32();
        if random % 100 < 5 && (!target_attacking || keep_anyway) {
            true
        } else {
            !e.call(GET_BLOCKED, &args![attacker]).bool()
        }
    };
    if ends {
        stop_blocking(e, this, attacker);
    }
}

// Translated from 009cd3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The combat state 2, waiting (unnamed in the map): back to deciding
/// (combat state 3) once the melee timer ran out.
pub fn fn_009cd3d0(e: &mut Engine, this: Ptr<CombatProcedureAttackMelee>) {
    if e.call(WORD_AT_14, &args![this]).u32() != 2 {
        return;
    }
    if e.call(TIMER_EXPIRED, &args![this.addr() + MELEE_TIMER])
        .bool()
    {
        fn_009cf6d0(e, this, 3);
    }
}

// Translated from 009cd410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Back to the maneuver state 0 (`009cf660`).
pub fn fn_009cd410(e: &mut Engine, this: Ptr<CombatProcedureAttackMelee>) {
    fn_009cf660(e, this, 0);
}

// Translated from 009cd430 (decompiled, FalloutNV.exe 1.4.0.525)
/// The maneuver state 0 (unnamed in the map): within reach and when the
/// controller's test `0097f2d0` fails and `009a6ae0` agrees, it backs away
/// (`009ce190`); while blocking it decides on the block (`fn_009cd150`);
/// outside the attack range it backs away when the margin exceeds
/// `fCombatMinEngageDistance` or the target moves away, else moves in
/// (`009cdf00`); otherwise `ManeuverDecision`. A branch on a local that is
/// always 0 (the power-attack check inside the minimum engage distance) is
/// kept as the exe has it.
pub fn fn_009cd430(e: &mut Engine, this: Ptr<CombatProcedureAttackMelee>) {
    let t = this.addr();
    if e.call(WORD_AT_10, &args![t]).u32() != 0 {
        return;
    }
    let attacker = e.call(GET_ATTACKER, &args![t]).u32();
    let target = e.call(GET_TARGET, &args![t]).u32();
    let facing = get_facing_target_distance(e, this);
    let reach = get_current_attack_reach(e, this);
    if fn_009cefd0(e, this, facing, reach, 0) {
        let controller = controller_of(e, this);
        if !e.call(CONTROLLER_TEST_F2D0, &args![controller]).bool()
            && e.call(UTILITY_TEST_9A6AE0, &args![attacker, target, 0u32])
                .bool()
        {
            fn_009ce190(e, this);
            return;
        }
    }
    let controller = controller_of(e, this);
    if e.call(CONTROLLER_TEST_1900, &args![controller]).bool()
        || e.call(WORD_AT_14, &args![t]).u32() == 1
    {
        fn_009cd150(e, this);
        return;
    }
    // A local the function sets to 0 and never changes.
    let check_power_attack = false;
    if outside_attack_range(e, this, facing, reach, 0) {
        let margin = x87_sub(x87_load32(facing.to_bits()), x87_m32(reach.to_bits()));
        let minimum = setting_bits(e, SETTING_COMBAT_MIN_ENGAGE_DISTANCE);
        // FLD minimum; FCOMP margin; TEST AH,5; JNP: backs away when the
        // minimum is less.
        if x87_load32(minimum) < margin || is_target_moving_away(e, this) {
            fn_009ce190(e, this);
        } else {
            fn_009cdf00(e, this);
        }
        return;
    }
    if check_power_attack {
        let minimum = setting_bits(e, SETTING_COMBAT_MIN_ENGAGE_DISTANCE);
        if x87_load32(minimum) > x87_load32(facing.to_bits()) {
            let animation = e.vcall(attacker, SLOT_ANIMATION, &args![]).u32();
            let group = e.call(ANIM_GROUP_OF, &args![animation, 4u32]).u16();
            if !e
                .call(IS_POWER_ATTACK_ACTION, &args![u32::from(group)])
                .bool()
            {
                fn_009cdf00(e, this);
            }
            return;
        }
    }
    maneuver_decision(e, this);
}

// Translated from 009cd5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureAttackMelee::ManeuverDecision` (Xbox PDB): with a
/// target, scores dodging (`00644990`, at least 0, plus 25 when a ranged
/// ally wants to shoot, plus 10 while waiting, minus 100 against a
/// non-player target whose byte `+0x104` is clear, 0 when the attacker's
/// byte `+0x14d` is set) and compares it with a random number below 100.
/// Dodging: the direction (`00644a70`) picks the animation group (3
/// forward/back, 4 for 2, 5 left, 6 right) and its duration range; with
/// that group's animation it forces the move mode (`| 0x200` when the
/// target idles and `fAIDodgeWalkChance` is below 0, else `| 0x100`),
/// starts the maneuver and dodge timers and enters the maneuver state 2;
/// without one it reports the missing animation. Not dodging (and not
/// already holding): holds for a random time (maneuver state 1).
pub fn maneuver_decision(e: &mut Engine, this: Ptr<CombatProcedureAttackMelee>) {
    let t = this.addr();
    let attacker = e.call(GET_ATTACKER, &args![t]).u32();
    let target = e.call(GET_TARGET, &args![t]).u32();
    if target == 0 {
        return;
    }
    let target_idle = e.call(GET_ANIM_ACTION, &args![target]).u32() == 2;
    let target_ranged = e.call(IS_ACTOR_USING_RANGED, &args![target]).bool();
    let fatigue = e.call(GET_FATIGUE_PERCENTAGE, &args![attacker]).f64();
    let walk_speed = e.call(GET_WALK_SPEED, &args![attacker]).f64();
    let walk_speed = e.call(FTOL, &args![walk_speed]).u32();
    let style = style_of(e, this);
    let mut chance = x87_store32(
        e.call(
            FORMULA_DODGE_CHANCE,
            &args![
                style,
                walk_speed,
                x87_store32(fatigue),
                u32::from(target_idle)
            ],
        )
        .f64(),
    );
    // FCOMP 0.0; TEST AH,5; JP: below 0 becomes 0.
    if x87_load32(chance) < double_at(e, ZERO_DOUBLE) {
        chance = 0;
    }
    if e.get(this, CombatProcedureAttackMelee::bRangedAllyWantsToShoot) {
        chance = x87_store32(x87_add(
            x87_load32(chance),
            double_at(e, TWENTY_FIVE_DOUBLE),
        ));
    }
    if e.call(WORD_AT_14, &args![t]).u32() == 2 {
        chance = x87_store32(x87_add(x87_load32(chance), double_at(e, TEN_DOUBLE)));
    }
    if target != e.global::<u32>(PLAYER) && e.call(ACTOR_BYTE_104, &args![target]).u8() == 0 {
        chance = x87_store32(x87_sub(x87_load32(chance), double_at(e, HUNDRED_DOUBLE)));
    }
    if e.call(ACTOR_BYTE_14D, &args![attacker]).bool() {
        chance = 0;
    }
    let roll = random_percent(e);
    // FLD roll; FLD chance; FCOMPP; TEST AH,1; JNZ: dodges unless the chance
    // is less (or unordered).
    if x87_load32(chance) >= x87_load32(roll) {
        let attacker_idle = e.call(GET_ANIM_ACTION, &args![attacker]).u32() == 2;
        let facing = get_facing_target_distance(e, this);
        let reach = get_current_attack_reach(e, this);
        let moves = e
            .call(RESTRICTED_MOVES, &args![t + MOVEMENT_RESTRICTIONS])
            .u16();
        let style = style_of(e, this);
        let ally = e.get(this, CombatProcedureAttackMelee::bRangedAllyWantsToShoot);
        let direction = e
            .call(
                FORMULA_DODGE_DIRECTION,
                &args![
                    style,
                    u32::from(attacker_idle),
                    u32::from(target_idle),
                    facing,
                    reach,
                    u32::from(ally),
                    u32::from(target_ranged),
                    u32::from(moves)
                ],
            )
            .u16();
        if direction == 0 {
            return;
        }
        let attacker2 = e.call(GET_ATTACKER, &args![t]).u32();
        if e.vcall(attacker2, SLOT_ANIMATION, &args![]).u32() == 0 {
            return;
        }
        let mut group = 3u32;
        let duration;
        match direction & 0xf {
            2 => {
                group = 4;
                let high = style_float(e, this, STYLE_DODGE_51B860);
                let low = style_float(e, this, STYLE_DODGE_9A7C40);
                duration = random_between(e, low, high);
            }
            4 => {
                group = 5;
                let _allowed = fn_009cfbd0(e, this, 4);
                let style = style_of(e, this);
                let high = fn_009cdac0(e, Ptr::new(style)).to_bits();
                let low = style_float(e, this, STYLE_DODGE_MIN_639B20);
                duration = random_between(e, low, high);
            }
            8 => {
                group = 6;
                let _allowed = fn_009cfbd0(e, this, 8);
                let style = style_of(e, this);
                let high = fn_009cdac0(e, Ptr::new(style)).to_bits();
                let low = style_float(e, this, STYLE_DODGE_MIN_639B20);
                duration = random_between(e, low, high);
            }
            _ => {
                let high = style_float(e, this, STYLE_DODGE_69EF80);
                let style = style_of(e, this);
                let low = fn_009cdae0(e, Ptr::new(style)).to_bits();
                duration = random_between(e, low, high);
            }
        }
        let attacker3 = e.call(GET_ATTACKER, &args![t]).u32();
        let anim_group = e
            .call(GET_ANIM_GROUP, &args![attacker3, group, 0u32, 0u32, 0u32])
            .u16();
        if e.call(ANIM_GROUP_TYPE, &args![u32::from(anim_group)]).u32() == group {
            let length = x87_store32(
                e.call(GET_ANIM_GROUP_DURATION, &args![attacker, group])
                    .f64(),
            );
            let duration = x87_store32(e.call(FLOAT_MAX, &args![length, duration]).f64());
            let zero = 0.0f64;
            let walk = target_idle && {
                let walk_chance = setting_bits(e, SETTING_AI_DODGE_WALK_CHANCE);
                // FLD 0; FLD chance; FCOMPP; TEST AH,5; JP: walks when the
                // setting is below 0.
                x87_load32(walk_chance) < zero
            };
            let mode = if walk {
                u32::from(direction) | 0x200
            } else {
                u32::from(direction) | 0x100
            };
            e.call(FORCE_MOVE_MODE, &args![attacker, mode]);
            e.call(TIMER_START, &args![t + MANEUVER_TIMER, duration]);
            e.call(TIMER_START, &args![t + DODGE_TIMER, length]);
            fn_009cf660(e, this, 2);
        } else if e.call(DEBUG_MESSAGES_ON, &args![]).bool() {
            let name = if direction & 4 != 0 {
                TEXT_LEFT
            } else if direction & 8 != 0 {
                TEXT_RIGHT
            } else if direction & 1 != 0 {
                TEXT_FOREWARD
            } else {
                TEXT_BACK
            };
            let actor_name = e.call(REFERENCE_NAME, &args![attacker]).u32();
            e.call(
                DEBUG_PRINT,
                &args![TEXT_NO_DODGE_ANIMATION, actor_name, name],
            );
        }
        return;
    }
    if e.call(WORD_AT_10, &args![t]).u32() == 1 {
        return;
    }
    let high = style_float(e, this, STYLE_HOLD_6A78F0);
    let low = style_float(e, this, STYLE_HOLD_9B0BD0);
    let hold = random_between(e, low, high);
    e.call(TIMER_START, &args![t + MANEUVER_TIMER, hold]);
    stop_moving(e, this);
    fn_009cf660(e, this, 1);
}

// Translated from 009cdb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureAttackMelee::HandleDodge` (Xbox PDB), the maneuver
/// state 2: ends when the maneuver timer runs out or the attacker stops
/// (`004938e0`); a blocked direction stops it ("has run out of room to
/// dodge"). Sideways dodges outside the attack range back away once the
/// dodge timer ran out, and add or drop the forward bit by the distance;
/// forward dodges end inside the minimum engage distance once the dodge
/// timer ran out. Against an idle target a walking dodge may turn into a
/// run (`fAIDodgeWalkChance`), otherwise a walk back into a run; then the
/// combat dialogue starts.
pub fn handle_dodge(e: &mut Engine, this: Ptr<CombatProcedureAttackMelee>) {
    let t = this.addr();
    if e.call(WORD_AT_10, &args![t]).u32() != 2 {
        return;
    }
    if e.call(TIMER_EXPIRED, &args![t + MANEUVER_TIMER]).bool() {
        fn_009cd410(e, this);
        fn_009cfc00(e, this);
        return;
    }
    let attacker = e.call(GET_ATTACKER, &args![t]).u32();
    let target = e.call(GET_TARGET, &args![t]).u32();
    let mode = e.call(ACTOR_MOVE_FLAGS, &args![attacker]).u32();
    if !e.call(ACTOR_TEST_4938E0, &args![attacker]).bool() {
        fn_009cd410(e, this);
        return;
    }
    e.call(EMPTY_HOOK, &args![t]);
    check_movement_restrictions_distance(e, this);
    let target_action = if target == 0 {
        u32::MAX
    } else {
        e.call(GET_ANIM_ACTION, &args![target]).u32()
    };
    if target_action == 2 && e.call(WORD_AT_14, &args![t]).u32() == 2 {
        fn_009cf6d0(e, this, 3);
        fn_009cf570(e, this);
    }
    if !fn_009cdde0(e, restrictions_of(this), mode as u16) {
        stop_moving(e, this);
        if e.call(DEBUG_MESSAGES_ON, &args![]).bool() {
            let name = e.call(REFERENCE_NAME, &args![attacker]).u32();
            e.call(DEBUG_PRINT, &args![TEXT_NO_ROOM_TO_DODGE, name]);
        }
        fn_009cd410(e, this);
        return;
    }
    let facing = get_facing_target_distance(e, this);
    let reach = get_current_attack_reach(e, this);
    if mode & 4 != 0 || mode & 8 != 0 {
        // FLD facing; FLD reach; FCOMPP; TEST AH,5; JP.
        if x87_load32(reach.to_bits()) < x87_load32(facing.to_bits()) {
            if outside_attack_range(e, this, facing, reach, 0) {
                if e.call(TIMER_EXPIRED, &args![t + DODGE_TIMER]).bool() {
                    stop_moving(e, this);
                    fn_009ce190(e, this);
                    return;
                }
            } else {
                e.call(FORCE_MOVE_MODE, &args![attacker, mode | 1]);
            }
        } else {
            let minimum = setting_bits(e, SETTING_COMBAT_MIN_ENGAGE_DISTANCE);
            if x87_load32(minimum) > x87_load32(facing.to_bits()) && mode & 1 != 0 {
                e.call(FORCE_MOVE_MODE, &args![attacker, mode & 0xffff_fffe]);
            }
        }
    } else if mode & 1 != 0 {
        let minimum = setting_bits(e, SETTING_COMBAT_MIN_ENGAGE_DISTANCE);
        // FLD facing; FLD minimum; FCOMPP; TEST AH,1; JNZ.
        if x87_load32(minimum) >= x87_load32(facing.to_bits())
            && e.call(TIMER_EXPIRED, &args![t + DODGE_TIMER]).bool()
        {
            stop_moving(e, this);
            fn_009cd410(e, this);
            return;
        }
    }
    if target_action == 2 {
        let roll = random_percent(e);
        if mode & 0x100 != 0 {
            let controller = controller_of(e, this);
            if !e.call(CONTROLLER_TEST_1900, &args![controller]).bool() {
                let walk_chance = setting_bits(e, SETTING_AI_DODGE_WALK_CHANCE);
                if x87_load32(walk_chance) < x87_load32(roll) {
                    e.call(
                        FORCE_MOVE_MODE,
                        &args![attacker, (mode | 0x200) & 0xffff_feff],
                    );
                }
            }
        }
    } else if mode & 0x200 != 0 {
        e.call(
            FORCE_MOVE_MODE,
            &args![attacker, (mode | 0x100) & 0xffff_fdff],
        );
    }
    let manager = e.global::<u32>(DIALOGUE_MANAGER);
    let controller = controller_of(e, this);
    e.call(
        START_DIALOGUE,
        &args![manager, controller, 2u32, 0u32, 0u32],
    );
}

// Translated from 009cdde0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether none of `moves` is restricted (`sRestrictedMoves & moves == 0`;
/// `__thiscall` on the `CombatMovementRestrictions`, one word; `EAX` is 0
/// or 1).
pub fn fn_009cdde0(e: &mut Engine, this: Ptr<CombatMovementRestrictions>, moves: u16) -> bool {
    e.get(this, CombatMovementRestrictions::sRestrictedMoves) & moves == 0
}

// Translated from 009cde10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The maneuver state 1, holding (unnamed in the map): back to deciding
/// when the maneuver timer ran out, or after half a second outside the
/// attack range; when the target attacks or a ranged ally wants to shoot,
/// a waiting combat state goes back to deciding and the maneuver is
/// decided again.
pub fn fn_009cde10(e: &mut Engine, this: Ptr<CombatProcedureAttackMelee>) {
    let t = this.addr();
    if e.call(WORD_AT_10, &args![t]).u32() != 1 {
        return;
    }
    let target = e.call(GET_TARGET, &args![t]).u32();
    if e.call(TIMER_EXPIRED, &args![t + MANEUVER_TIMER]).bool() {
        fn_009cd410(e, this);
        return;
    }
    let elapsed = e.call(TIMER_ELAPSED, &args![t + MANEUVER_TIMER]).f64();
    if elapsed > double_at(e, HALF_DOUBLE) {
        let facing = get_facing_target_distance(e, this);
        let reach = get_current_attack_reach(e, this);
        if outside_attack_range(e, this, facing, reach, 0) {
            fn_009cd410(e, this);
            return;
        }
    }
    let react = (target != 0 && e.call(ACTOR_IS_ATTACKING, &args![target]).bool())
        || e.get(this, CombatProcedureAttackMelee::bRangedAllyWantsToShoot);
    if react {
        if e.call(WORD_AT_14, &args![t]).u32() == 2 {
            fn_009cf6d0(e, this, 3);
            fn_009cf570(e, this);
        }
        maneuver_decision(e, this);
    }
}

// Translated from 009cdf00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Starts moving in (maneuver state 3, unnamed in the map): move mode 1,
/// the face-target location at the attack reach, a path there
/// (`009ce5b0`) and the first step of the state (`009cdf80`).
pub fn fn_009cdf00(e: &mut Engine, this: Ptr<CombatProcedureAttackMelee>) {
    let t = this.addr();
    fn_009cf660(e, this, 3);
    let attacker = e.call(GET_ATTACKER, &args![t]).u32();
    let target = e.call(GET_TARGET, &args![t]).u32();
    e.call(SET_MOVE_MODE, &args![t, 1u32]);
    e.with_stack(0xc, |e, location| {
        let reach = get_current_attack_reach(e, this);
        let position = e.vcall(target, SLOT_POSITION, &args![]).u32();
        e.call(
            COMPUTE_FACE_TARGET_LOCATION,
            &args![location, attacker, target, position, reach],
        );
        fn_009ce5b0(e, this, location.addr());
    });
    fn_009cdf80(e, this);
}

/// The test `fn_009cdf80` and `fn_009ce1d0` share: with a path goal (and
/// not waiting on the path), whether the distance from the goal to the
/// target minus their extents is within reach.
fn goal_within_reach(
    e: &mut Engine,
    this: Ptr<CombatProcedureAttackMelee>,
    attacker: u32,
    target: u32,
    reach: f32,
    goal: u32,
    difference: u32,
) -> bool {
    if !e
        .call(GET_CURRENT_PATHFINDING_GOAL, &args![attacker, goal])
        .bool()
    {
        return false;
    }
    let extent = x87_store32(
        e.call(GET_ATTACKER_TARGET_EXTENT_SUM, &args![attacker, target])
            .f64(),
    );
    let position = e.vcall(target, SLOT_POSITION, &args![]).u32();
    let difference = e
        .call(POINT_DIFFERENCE, &args![goal, difference, position])
        .u32();
    let length = e.call(POINT_LENGTH, &args![difference]).f64();
    let distance = x87_store32(x87_sub(length, x87_m32(extent)));
    fn_009cefd0(e, this, f32::from_bits(distance), reach, 0)
}

// Translated from 009cdf80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The maneuver state 3, moving in (unnamed in the map): back to deciding
/// when the attacker stops pathing; beyond twice the minimum engage
/// distance it backs away when attacking; outside the attack range (with
/// the controller's test `0097f2d0`) it re-paths to the face-target
/// location unless it waits on its path or the goal is within reach;
/// within reach it decides again, or goes on with the combat state when
/// it may back away (move flag 2) inside the minimum engage distance.
pub fn fn_009cdf80(e: &mut Engine, this: Ptr<CombatProcedureAttackMelee>) {
    let t = this.addr();
    if e.call(WORD_AT_10, &args![t]).u32() != 3 {
        return;
    }
    let attacker = e.call(GET_ATTACKER, &args![t]).u32();
    let target = e.call(GET_TARGET, &args![t]).u32();
    let facing = get_facing_target_distance(e, this);
    let reach = get_current_attack_reach(e, this);
    let mode = e.call(ACTOR_MOVE_FLAGS, &args![attacker]).u16();
    if !e.call(IS_PATHING, &args![attacker]).bool() {
        fn_009cd410(e, this);
        return;
    }
    e.with_stack(0x30, |e, frame| {
        let location = frame.addr();
        let goal = location + 0xc;
        let difference = location + 0x18;
        let reach2 = get_current_attack_reach(e, this);
        let position = e.vcall(target, SLOT_POSITION, &args![]).u32();
        e.call(
            COMPUTE_FACE_TARGET_LOCATION,
            &args![location, attacker, target, position, reach2],
        );
        let minimum = setting_bits(e, SETTING_COMBAT_MIN_ENGAGE_DISTANCE);
        let twice = x87_add(x87_load32(minimum), x87_load32(minimum));
        if x87_load32(facing.to_bits()) > twice {
            if e.call(WORD_AT_14, &args![t]).u32() == 0 {
                fn_009ce190(e, this);
            }
            return;
        }
        if outside_attack_range(e, this, facing, reach, 0) && {
            let controller = controller_of(e, this);
            e.call(CONTROLLER_TEST_F2D0, &args![controller]).bool()
        } {
            e.call(POINT_CONSTRUCT, &args![goal]);
            let done = e.call(IS_WAITING_ON_PATH, &args![attacker]).bool()
                || goal_within_reach(e, this, attacker, target, reach, goal, difference);
            if !done {
                fn_009ce5b0(e, this, location);
            }
            return;
        }
        if fn_009cefd0(e, this, facing, reach, 0) {
            let go_on = mode & 2 != 0 && {
                let minimum = setting_bits(e, SETTING_COMBAT_MIN_ENGAGE_DISTANCE);
                x87_load32(minimum) > x87_load32(facing.to_bits())
            };
            if go_on {
                fn_009cf570(e, this);
            } else {
                fn_009cd410(e, this);
            }
        }
    });
}

// Translated from 009ce190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Starts backing away (maneuver state 4, unnamed in the map): move mode
/// 3, the maneuver timer at 3 and the first step (`009ce1d0`).
pub fn fn_009ce190(e: &mut Engine, this: Ptr<CombatProcedureAttackMelee>) {
    let t = this.addr();
    fn_009cf660(e, this, 4);
    e.call(SET_MOVE_MODE, &args![t, 3u32]);
    let three = x87_quiet_bits(e.global::<u32>(THREE_FLOAT));
    e.call(TIMER_START, &args![t + MANEUVER_TIMER, three]);
    fn_009ce1d0(e, this);
}

// Translated from 009ce1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The maneuver state 4, backing away / advancing (unnamed in the map):
/// re-paths to the face-target location when outside the attack range or
/// when the controller's test `0097f2d0` fails (unless it waits on its
/// path, `008b3c70` holds, or the goal is within reach); once the path is
/// done it decides again (unless the target moves away); a blocked
/// attacker releases its block; within reach it moves in or decides again;
/// within twice the reach (with the attacker able to attack) it may start
/// a normal attack (`fCombatAdvanceNormalAttackChance`); otherwise, while
/// not attacking, it tries a power attack (`009cf090`).
pub fn fn_009ce1d0(e: &mut Engine, this: Ptr<CombatProcedureAttackMelee>) {
    let t = this.addr();
    if e.call(WORD_AT_10, &args![t]).u32() != 4 {
        return;
    }
    let attacker = e.call(GET_ATTACKER, &args![t]).u32();
    let target = e.call(GET_TARGET, &args![t]).u32();
    let facing = get_facing_target_distance(e, this);
    let reach = get_current_attack_reach(e, this);
    let moving_away = is_target_moving_away(e, this);
    let object = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![attacker]).u32();
    let repath = if outside_attack_range(e, this, facing, reach, 0) && {
        let controller = controller_of(e, this);
        e.call(CONTROLLER_TEST_F2D0, &args![controller]).bool()
    } {
        true
    } else {
        let controller = controller_of(e, this);
        !e.call(CONTROLLER_TEST_F2D0, &args![controller]).bool()
    };
    if repath {
        e.with_stack(0x30, |e, frame| {
            let location = frame.addr();
            let goal = location + 0xc;
            let difference = location + 0x18;
            let reach2 = get_current_attack_reach(e, this);
            let position = e.vcall(target, SLOT_POSITION, &args![]).u32();
            e.call(
                COMPUTE_FACE_TARGET_LOCATION,
                &args![location, attacker, target, position, reach2],
            );
            e.call(POINT_CONSTRUCT, &args![goal]);
            let done = e.call(IS_WAITING_ON_PATH, &args![attacker]).bool()
                || (!e.call(ACTOR_TEST_8B3C70, &args![attacker]).bool()
                    && goal_within_reach(e, this, attacker, target, reach, goal, difference));
            if !done {
                fn_009ce5b0(e, this, location);
            }
        });
    }
    if e.call(IS_PATHING_COMPLETE, &args![attacker]).bool() {
        if !moving_away {
            fn_009cd410(e, this);
        }
        return;
    }
    if e.call(GET_BLOCKED, &args![attacker]).bool() {
        e.call(SET_BLOCK, &args![attacker, 0u32]);
        return;
    }
    if fn_009cefd0(e, this, facing, reach, 0) && {
        let controller = controller_of(e, this);
        e.call(CONTROLLER_TEST_F2D0, &args![controller]).bool()
    } {
        let move_in = moving_away && {
            let minimum = setting_bits(e, SETTING_COMBAT_MIN_ENGAGE_DISTANCE);
            let twice = x87_add(x87_load32(minimum), x87_load32(minimum));
            // FLD facing (on top of twice); FCOMPP; TEST AH,5; JP: moves in
            // when the facing distance is less.
            x87_load32(facing.to_bits()) < twice
        };
        if move_in {
            fn_009cdf00(e, this);
        } else {
            fn_009cd410(e, this);
        }
        return;
    }
    let attack_now = e.call(WORD_AT_14, &args![t]).u32() != 0
        && {
            let twice = x87_store32(x87_add(
                x87_load32(reach.to_bits()),
                x87_load32(reach.to_bits()),
            ));
            fn_009cefd0(e, this, facing, f32::from_bits(twice), 0)
        }
        && {
            let controller = controller_of(e, this);
            e.call(CONTROLLER_TEST_F2D0, &args![controller]).bool()
        }
        && e.vcall(attacker, SLOT_TEST_218, &args![]).bool()
        && object != 0
        && e.vcall(object, SLOT_TEST_3F8, &args![]).bool();
    if attack_now {
        if e.call(UTILITY_TEST_9A6AE0, &args![attacker, target, 0u32])
            .bool()
        {
            let controller = controller_of(e, this);
            let _weapon = e.call(CONTROLLER_WEAPON, &args![controller]).u32();
            let random = e.call(RANDOM_U32, &args![]).u32();
            let roll = random % 100;
            let chance = setting_bits(e, SETTING_COMBAT_ADVANCE_NORMAL_ATTACK_CHANCE);
            let chance = x87_mul(x87_load32(chance), double_at(e, HUNDRED_DOUBLE));
            let limit = e.call(FTOL, &args![chance]).u32();
            if roll < limit {
                let group = e.global::<u32>(ATTACK_GROUP_WORD);
                let animation = e.vcall(attacker, SLOT_ANIMATION, &args![]).u32();
                let choice = e
                    .call(ANIM_ATTACK_CHOICE, &args![animation, 1u32, group, 0u32])
                    .u8() as i8;
                let attack = if choice as i32 == 0x6c { 0x1a } else { 0x20 };
                start_attack(e, this, attack, 0);
            }
        }
        return;
    }
    if e.get(this, CombatProcedureAttackMelee::eCombatState) != 0 {
        fn_009cf090(e, this, facing, reach);
    }
}

// Translated from 009ce560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureAttackMelee::GetCurrentAttackReach` (Xbox PDB): the
/// cached reach, computed from the controller's weapon
/// (`CombatUtilities::GetWeaponReach`) while it is below 0.
pub fn get_current_attack_reach(e: &mut Engine, this: Ptr<CombatProcedureAttackMelee>) -> f32 {
    let t = this.addr();
    let attacker = e.call(GET_ATTACKER, &args![t]).u32();
    // FCOMP 0.0; TEST AH,5; JP: computed only when below 0.
    if x87_load32(e.mem.u32(t + 0x58)) < double_at(e, ZERO_DOUBLE) {
        let controller = controller_of(e, this);
        let weapon = e.call(CONTROLLER_WEAPON, &args![controller]).u32();
        let reach = e.call(GET_WEAPON_REACH, &args![attacker, weapon]).f64();
        e.mem.set_u32(t + 0x58, x87_store32(reach));
    }
    f32::from_bits(x87_quiet_bits(e.mem.u32(t + 0x58)))
}

// Translated from 009ce5b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009ce5b0 - the compiler's FS:[0] exception frame (unwinding of the pathing request, the pathing location and the avoid-node pointer) is not translated
/// Sets a path to `point` (unnamed in the map): when the point is a valid
/// location for the attacker and the target is in its line of sight from
/// there, the request avoids the target (an avoid node of
/// `fMovementNearTargetAvoidRadius` / `fMovementNearTargetAvoidCost`) and
/// goes to the point within 5; otherwise it goes to the target within 10
/// (or its face-target distance when the combat state's test `0097f2f0`
/// holds). Then the request becomes the attacker's pathfinding goal.
///
/// Locals follow the disassembly's frame (offsets below `EBP`): the
/// request `-0xec`, the pathing location `-0x38`, the avoid-node pointer
/// `-0xf4`, the avoid node `-0x124`, the location's point `-0x130`.
pub fn fn_009ce5b0(e: &mut Engine, this: Ptr<CombatProcedureAttackMelee>, point: u32) {
    const FRAME: u32 = 0x140;
    let t = this.addr();
    e.with_stack(FRAME, |e, base| {
        let ebp = base.addr() + FRAME;
        let request = ebp - 0xec;
        let location = ebp - 0x38;
        let avoid = ebp - 0xf4;
        let node = ebp - 0x124;
        let location_point = ebp - 0x130;
        let attacker = e.call(GET_ATTACKER, &args![t]).u32();
        let target = e.call(GET_TARGET, &args![t]).u32();
        e.call(PATHING_REQUEST_CONSTRUCT, &args![request]);
        e.call(
            PATHING_LOCATION_FROM_POINT,
            &args![location, point, attacker],
        );
        let radius = e.call(GET_RADIUS, &args![attacker]).f64();
        let valid = e
            .call(
                PATHING_LOCATION_IS_VALID,
                &args![location, x87_store32(radius)],
            )
            .bool();
        let line_of_sight = valid && {
            let position = e.vcall(target, SLOT_POSITION, &args![]).u32();
            e.call(
                CALCULATE_LOS,
                &args![attacker, target, point, position, 0u32, 0x1eu32],
            )
            .bool()
        };
        if line_of_sight {
            let block = e.call(ALLOCATE, &args![0x18u32]).u32();
            let array = if block == 0 {
                0
            } else {
                e.call(AVOID_ARRAY_CONSTRUCT, &args![block]).u32()
            };
            e.call(AVOID_POINTER_CONSTRUCT, &args![avoid, array]);
            let cost = setting_bits(e, SETTING_MOVEMENT_NEAR_TARGET_AVOID_COST);
            let radius = setting_bits(e, SETTING_MOVEMENT_NEAR_TARGET_AVOID_RADIUS);
            let position = e.vcall(target, SLOT_POSITION, &args![]).u32();
            let avoid_node = e
                .call(AVOID_NODE_CONSTRUCT, &args![node, position, radius, cost])
                .u32();
            let array = e.call(POINTER_GET, &args![avoid]).u32();
            e.call(AVOID_ARRAY_ADD, &args![array, avoid_node]);
            let array = e.call(POINTER_GET, &args![avoid]).u32();
            let five = x87_quiet_bits(e.global::<u32>(FIVE_FLOAT));
            let worldspace = e.call(PATHING_LOCATION_WORLDSPACE, &args![location]).u32();
            let cell = e.call(PATHING_LOCATION_CELL, &args![location]).u32();
            let goal = e
                .call(PATHING_LOCATION_POINT, &args![location, location_point])
                .u32();
            e.call(
                BUILD_REQUEST,
                &args![attacker, request, goal, cell, worldspace, five, array],
            );
            e.call(AVOID_POINTER_DESTRUCT, &args![avoid]);
        } else {
            let mut distance = x87_quiet_bits(e.global::<u32>(TEN_FLOAT));
            let controller = controller_of(e, this);
            let combat_state = e.call(GET_COMBAT_STATE, &args![controller]).u32();
            if e.call(COMBAT_STATE_TEST_F2F0, &args![combat_state]).bool() {
                let reach = get_current_attack_reach(e, this);
                distance = x87_store32(
                    e.call(
                        COMPUTE_FACE_TARGET_DISTANCE,
                        &args![attacker, target, reach],
                    )
                    .f64(),
                );
            }
            let worldspace = e.call(GET_WORLD_SPACE, &args![target]).u32();
            let cell = e.call(ACTOR_CELL_8D6F30, &args![target]).u32();
            let position = e.vcall(target, SLOT_POSITION, &args![]).u32();
            e.call(
                BUILD_REQUEST,
                &args![attacker, request, position, cell, worldspace, distance, 0u32],
            );
        }
        e.call(SET_PATHFINDING_GOAL, &args![attacker, request]);
        e.call(PATHING_LOCATION_DESTRUCT, &args![location]);
        e.call(PATHING_REQUEST_DESTRUCT, &args![request]);
    });
}

// Translated from 009ce870 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009ce870 - the /GS cookie check (00ec408c) on return is not translated
/// `CombatProcedureAttackMelee::SelectPowerAttack` (Xbox PDB): the power
/// attack group (`0x5c + direction`, 0xff for none) for the distance
/// `facing` and the reach `reach`. Each of the five directions has a
/// distance and a weight in the controller (`009cef70`, `009cef90`, filled
/// by `DeterminePowerAttackDeltas` unless the controller's byte `+0xf9`
/// says they are current); a direction qualifies when its distance (times
/// the grenade scale when a grenade is out of reach) covers the target's
/// approach (`weight * velocity / speed + facing`) and either the scaled
/// reach reaches its distance or the approach is above the factor at
/// `01090d68` (0.66) of it. One candidate is taken; several are drawn
/// among (with a grenade out of reach and the style's flag 1 clear); else
/// the style's flag 1 clear picks by the situation (forward close in, back
/// far away, the sides by the free directions, normal) from the table at
/// `011a5ac8`; else a draw weighted by the style's attacks. An attack the
/// controller refuses (`009800e0`) is reported; the choice is printed when
/// debug messages are on. The last three stack words are not read.
pub fn select_power_attack(
    e: &mut Engine,
    this: Ptr<CombatProcedureAttackMelee>,
    facing: f32,
    reach: f32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    const FRAME: u32 = 0x100;
    let t = this.addr();
    e.with_stack(FRAME, |e, base| {
        let ebp = base.addr() + FRAME;
        let chosen = ebp - 0x24;
        let ids = ebp - 0x38;
        let point = ebp - 0x48;
        // The style's answer per attack group, at `ebp - 0xf0 + group`.
        let has_attack = ebp - 0xf0;
        let mut attack = 0xffu32;
        let mut count = 0u32;
        e.call(FILL_BYTES, &args![chosen, 0u32, 5u32]);
        let attacker = e.call(GET_ATTACKER, &args![t]).u32();
        let style = style_of(e, this);
        let controller = controller_of(e, this);
        if e.call(IS_SWITCHING_WEAPONS, &args![controller]).bool() {
            return 0xff;
        }
        let controller = controller_of(e, this);
        if !fn_009cefb0(e, controller) {
            let controller = controller_of(e, this);
            e.call(DETERMINE_POWER_ATTACK_DELTAS, &args![controller]);
        }
        let controller = controller_of(e, this);
        let grenade_out_of_reach = e.call(CONTROLLER_TEST_8220, &args![controller]).bool()
            && !fn_009cefd0(e, this, facing, reach, 0);
        let scale = if grenade_out_of_reach {
            x87_store32(e.call(STYLE_GRENADE_SCALE, &args![style]).f64())
        } else {
            1.0f32.to_bits()
        };
        let animation = e.vcall(attacker, SLOT_ANIMATION, &args![]).u32();
        let mut speed = x87_store32(e.call(ANIM_SPEED, &args![animation]).f64());
        // FCOMP 0.0; TEST AH,0x41; JP: at most 0 becomes 1.
        if x87_load32(speed) <= double_at(e, ZERO_DOUBLE) {
            speed = 1.0f32.to_bits();
        }
        let velocity = x87_store32(get_moving_target_velocity(e, this));
        let scaled_reach = x87_store32(x87_mul(x87_load32(reach.to_bits()), x87_m32(scale)));
        let owner = attacker + ACTOR_VALUE_OWNER;
        let both_values = e.vcall(owner, SLOT_GET_ACTOR_VALUE, &args![0x1du32]).f64()
            > double_at(e, ZERO_DOUBLE)
            && e.vcall(owner, SLOT_GET_ACTOR_VALUE, &args![0x1eu32]).f64()
                > double_at(e, ZERO_DOUBLE);
        let one = double_at(e, ONE_DOUBLE);
        for i in 0..5u32 {
            let controller = controller_of(e, this);
            let distance = fn_009cef70(e, controller, i).to_bits();
            if x87_load32(distance) > one && !both_values {
                continue;
            }
            let scaled = x87_store32(x87_mul(x87_load32(distance), x87_m32(scale)));
            let range = x87_store32(e.call(FLOAT_MAX, &args![scaled, scaled_reach]).f64());
            let controller = controller_of(e, this);
            let weight = fn_009cef90(e, controller, i).to_bits();
            // FCOMP 0.0; TEST AH,5; JP: a weight below 0 is skipped.
            if x87_load32(weight) < double_at(e, ZERO_DOUBLE) {
                continue;
            }
            let group = i + 0x5c;
            if e.call(STYLE_FLAG_TEST, &args![style, 1u32]).bool()
                && !e.call(STYLE_HAS_ATTACK, &args![style, group]).bool()
            {
                continue;
            }
            let approach = x87_store32(x87_add(
                x87_div(
                    x87_mul(x87_load32(weight), x87_m32(velocity)),
                    x87_m32(speed),
                ),
                x87_m32(facing.to_bits()),
            ));
            // FLD approach; FLD range; FCOMPP; TEST AH,1; JNZ: skipped when
            // the range is less (or unordered).
            if !(x87_load32(range) >= x87_load32(approach)) {
                continue;
            }
            let take = if x87_load32(scaled_reach) >= x87_load32(distance) {
                true
            } else {
                let margin = x87_mul(x87_load32(range), double_at(e, POWER_ATTACK_MARGIN_DOUBLE));
                x87_load32(approach) > margin
            };
            if take {
                e.mem.set_u32(ids + count * 4, group);
                count += 1;
                e.mem.set_u8(chosen + i, 1);
            }
        }
        if count == 0 {
            return 0xff;
        }
        for j in count..5 {
            e.mem.set_u32(ids + j * 4, 0xff);
        }
        if count == 1 {
            attack = e.mem.u32(ids);
        } else if count as i32 > 1
            && grenade_out_of_reach
            && !e.call(STYLE_FLAG_TEST, &args![style, 1u32]).bool()
        {
            let pick = e.call(RANDOM_RANGE, &args![0u32, count]).u32();
            attack = e.mem.u32(ids.wrapping_add(pick.wrapping_mul(4)));
        }
        if attack == 0xff {
            if !e.call(STYLE_FLAG_TEST, &args![style, 1u32]).bool() {
                let mut kind = 5u32;
                let forward_close = e.mem.u8(chosen + 2) != 0 && {
                    let minimum = setting_bits(e, SETTING_COMBAT_MIN_ENGAGE_DISTANCE);
                    x87_load32(minimum) > x87_load32(facing.to_bits())
                };
                if forward_close {
                    kind = 2;
                } else if e.mem.u8(chosen + 1) != 0
                    && x87_mul(
                        x87_load32(reach.to_bits()),
                        double_at(e, ONE_AND_A_QUARTER_DOUBLE),
                    ) < x87_load32(facing.to_bits())
                {
                    kind = 1;
                } else {
                    let moves = e.call(ACTOR_MOVE_FLAGS, &args![attacker]).u16();
                    if e.mem.u8(chosen + 3) != 0 && moves & 4 != 0 {
                        kind = 3;
                    } else if e.mem.u8(chosen + 4) != 0 && moves & 8 != 0 {
                        kind = 4;
                    } else if e.mem.u8(chosen) != 0 {
                        kind = 0;
                    }
                }
                if kind != 5 {
                    attack = e.mem.u32(POWER_ATTACK_BY_KIND + kind * 4);
                }
            }
            if attack == 0xff {
                for k in 0..5u32 {
                    let has = e.call(STYLE_HAS_ATTACK, &args![style, k + 0x5c]).u8();
                    e.mem.set_u8(has_attack + 0x5c + k, has);
                }
                let mut total = 0u32;
                for j in 0..count {
                    let group = e.mem.u32(ids + j * 4);
                    let has = e.mem.u8(has_attack.wrapping_add(group));
                    total = total.wrapping_add(u32::from(has));
                }
                if total > 0 {
                    let pick = e.call(RANDOM_RANGE, &args![0u32, total]).u32();
                    let mut sum = 0u32;
                    for j in 0..count {
                        let group = e.mem.u32(ids + j * 4);
                        let has = e.mem.u8(has_attack.wrapping_add(group));
                        sum = sum.wrapping_add(u32::from(has));
                        if pick <= sum {
                            attack = group;
                            break;
                        }
                    }
                }
            }
        }
        e.call(POINT_CONSTRUCT, &args![point]);
        let controller = controller_of(e, this);
        if !e
            .call(CONTROLLER_ATTACK_CHECK, &args![controller, attack, point])
            .bool()
        {
            let form_id = e.call(FORM_ID, &args![attacker]).u32();
            let name = e.call(REFERENCE_NAME, &args![attacker]).u32();
            e.call(
                DEBUG_WARNING,
                &args![TEXT_POWER_ATTACK_UNUSABLE, name, form_id, attack],
            );
        }
        let result = attack;
        let announced = true;
        if e.call(DEBUG_MESSAGES_ON, &args![]).bool() && announced {
            let text = match result {
                0x5c => TEXT_NORMAL,
                0x5d => TEXT_FORWARD,
                0x5e => TEXT_BACKWARD,
                0x5f => TEXT_LEFT,
                0x60 => TEXT_RIGHT,
                _ => TEXT_NO,
            };
            let name = e.call(REFERENCE_NAME, &args![attacker]).u32();
            e.call(DEBUG_PRINT, &args![TEXT_SELECTS_POWER_ATTACK, name, text]);
        }
        result
    })
}

// Translated from 009cef70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The float at `+0xfc + 4 * index` (`__thiscall`, one word, `ST0`; the
/// power attack distances of the controller `SelectPowerAttack` reads).
pub fn fn_009cef70(e: &mut Engine, this: u32, index: u32) -> f32 {
    let at = this.wrapping_add(index.wrapping_mul(4)).wrapping_add(0xfc);
    f32::from_bits(x87_quiet_bits(e.mem.u32(at)))
}

// Translated from 009cef90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The float at `+0x110 + 4 * index` (`__thiscall`, one word, `ST0`; the
/// power attack weights of the controller).
pub fn fn_009cef90(e: &mut Engine, this: u32, index: u32) -> f32 {
    let at = this.wrapping_add(index.wrapping_mul(4)).wrapping_add(0x110);
    f32::from_bits(x87_quiet_bits(e.mem.u32(at)))
}

// Translated from 009cefb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `+0xf9` (`__thiscall`, `AL`; on the controller, whether its
/// power attack values are current).
pub fn fn_009cefb0(e: &mut Engine, this: u32) -> bool {
    e.mem.u8(this + 0xf9) != 0
}

// Translated from 009cefd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `distance` is within `reach` (`distance <= reach + 0`, false for
/// NaN). It also asks for the attacker and the style's byte `005524b0`
/// (compared with 100), whose results it does not use; the third stack
/// word is not read.
pub fn fn_009cefd0(
    e: &mut Engine,
    this: Ptr<CombatProcedureAttackMelee>,
    distance: f32,
    reach: f32,
    _unused_3: u32,
) -> bool {
    let _attacker = e.call(GET_ATTACKER, &args![this]).u32();
    let style = style_of(e, this);
    let _skilled = e.call(STYLE_SCORE_5524B0, &args![style]).u8() as i8 >= 100;
    let limit = x87_add(x87_load32(reach.to_bits()), x87_m32(0.0f32.to_bits()));
    // FLD distance; FCOMPP; TEST AH,0x41; JP: within when less or equal.
    x87_load32(distance.to_bits()) <= limit
}

// Translated from 009cf030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureAttackMelee::OutsideAttackRange` (Xbox PDB): whether
/// `distance >= reach + 0` (false for NaN); the same unused reads as
/// `009cefd0`, and the third stack word is not read.
pub fn outside_attack_range(
    e: &mut Engine,
    this: Ptr<CombatProcedureAttackMelee>,
    distance: f32,
    reach: f32,
    _unused_3: u32,
) -> bool {
    let _attacker = e.call(GET_ATTACKER, &args![this]).u32();
    let style = style_of(e, this);
    let _skilled = e.call(STYLE_SCORE_5524B0, &args![style]).u8() as i8 >= 100;
    let limit = x87_add(x87_load32(reach.to_bits()), x87_m32(0.0f32.to_bits()));
    // FLD distance; FCOMPP; TEST AH,1; JNZ: outside unless less (or
    // unordered).
    x87_load32(distance.to_bits()) >= limit
}

// Translated from 009cf090 (decompiled, FalloutNV.exe 1.4.0.525)
/// A power attack while closing in (unnamed in the map; two floats): when
/// deciding, at least a second after the last failed pick, with the
/// controller's test `008d8220`, the acquire object's slot `+0x3f8`, no
/// power attack playing, the attacker able to shout, `0097f2d0` and
/// `009a6ae0`: picks a power attack, checks it with `009cf420` and the
/// style's chance (byte `+0xad`), and requires its score to beat the
/// block and hold scores. With the target in line of sight it starts the
/// attack and decides the maneuver again; otherwise it stamps the failed
/// pick time.
pub fn fn_009cf090(e: &mut Engine, this: Ptr<CombatProcedureAttackMelee>, facing: f32, reach: f32) {
    let t = this.addr();
    if e.call(WORD_AT_14, &args![t]).u32() != 3 {
        return;
    }
    let attacker = e.call(GET_ATTACKER, &args![t]).u32();
    let target = e.call(GET_TARGET, &args![t]).u32();
    if target == 0 {
        return;
    }
    let since = e
        .call(TIMER_ELAPSED, &args![t + ATTACK_PICK_FAILED_TIMESTAMP])
        .f64();
    // FCOMP 1.0; TEST AH,5; JP: goes on unless less than 1.
    if since < double_at(e, ONE_DOUBLE) {
        return;
    }
    let controller = controller_of(e, this);
    if !e.call(CONTROLLER_TEST_8220, &args![controller]).bool() {
        return;
    }
    let object = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![attacker]).u32();
    if !e.vcall(object, SLOT_TEST_3F8, &args![]).bool() {
        return;
    }
    let power_playing = if e.vcall(attacker, SLOT_ANIMATION, &args![]).u32() == 0 {
        false
    } else {
        let animation = e.vcall(attacker, SLOT_ANIMATION, &args![]).u32();
        let group = e.call(ANIM_GROUP_OF, &args![animation, 4u32]).u16();
        e.call(IS_POWER_ATTACK_ACTION, &args![u32::from(group)])
            .bool()
    };
    if power_playing {
        return;
    }
    if e.vcall(attacker, SLOT_TEST_218, &args![]).bool()
        && e.call(ACTOR_BYTE_14D, &args![attacker]).bool()
    {
        return;
    }
    let controller = controller_of(e, this);
    if !e.call(CONTROLLER_TEST_F2D0, &args![controller]).bool() {
        return;
    }
    if !e
        .call(UTILITY_TEST_9A6AE0, &args![attacker, target, 0u32])
        .bool()
    {
        return;
    }
    let target_action = e.call(GET_ANIM_ACTION, &args![target]).u32();
    let target_attacking = e.call(ACTOR_IS_ATTACKING, &args![target]).bool();
    let target_action_4 = target_action == 4;
    let target_recoiling = target_action == 8 || target_action == 0xa;
    let target_blocked = e.call(GET_BLOCKED, &args![target]).bool();
    let attack = select_power_attack(
        e,
        this,
        facing,
        reach,
        u32::from(target_attacking),
        u32::from(target_action_4),
        1,
    );
    if attack == 0xff {
        return;
    }
    if !fn_009cf420(e, this, attack) {
        return;
    }
    let roll = (e.call(RANDOM_U32, &args![]).u32() % 100) as i32;
    let style = style_of(e, this);
    let chance = i32::from(fn_009cf400(e, style));
    if roll >= chance {
        return;
    }
    let slot_230 = e.vcall(target, SLOT_TEST_230, &args![]).bool();
    let controller = controller_of(e, this);
    let no_weapon = e.call(CONTROLLER_WEAPON, &args![controller]).u32() == 0;
    let value = e
        .call(
            GET_CLAMPED_ACTOR_VALUE,
            &args![attacker + ACTOR_VALUE_OWNER, 0xbu32],
        )
        .u32();
    let controller = controller_of(e, this);
    let fc70 = e.call(CONTROLLER_VALUE_FC70, &args![controller]).u32();
    let style = style_of(e, this);
    let attack_score = x87_store32(
        e.call(
            FORMULA_ATTACK_SCORE,
            &args![
                style,
                fc70,
                value,
                u32::from(target_attacking),
                u32::from(no_weapon),
                u32::from(target_recoiling),
                u32::from(slot_230),
                u32::from(target_blocked)
            ],
        )
        .f64(),
    );
    let style = style_of(e, this);
    let block_score = x87_store32(
        e.call(
            FORMULA_BLOCK_SCORE,
            &args![style, 0x32u32, 0x32u32, 0u32, u32::from(target_attacking)],
        )
        .f64(),
    );
    let max_hold = setting_bits(e, SETTING_COMBAT_MAX_HOLD_SCORE);
    let rest = x87_store32(x87_sub(
        double_at(e, HUNDRED_DOUBLE),
        x87_add(x87_load32(attack_score), x87_m32(block_score)),
    ));
    let hold_score = x87_store32(e.call(FLOAT_MAX, &args![rest, max_hold]).f64());
    // FLD attack; FLD other; FCOMPP; TEST AH,5; JP: only when the other
    // score is less.
    if !(x87_load32(block_score) < x87_load32(attack_score)) {
        return;
    }
    if !(x87_load32(hold_score) < x87_load32(attack_score)) {
        return;
    }
    let target_position = e.vcall(target, SLOT_POSITION, &args![]).u32();
    let attacker_position = e.vcall(attacker, SLOT_POSITION, &args![]).u32();
    if e.call(
        CALCULATE_LOS,
        &args![
            attacker,
            target,
            attacker_position,
            target_position,
            0u32,
            0x1eu32
        ],
    )
    .bool()
    {
        start_attack(e, this, attack, 0);
        fn_009cf660(e, this, 0);
    } else {
        e.with_stack(4, |e, stamp| {
            let now = e.call(TIME_NOW, &args![]).f64();
            e.call(TIMESTAMP_SET, &args![stamp, x87_store32(now)]);
            let value = e.mem.u32(stamp.addr());
            e.mem.set_u32(t + ATTACK_PICK_FAILED_TIMESTAMP, value);
        });
    }
}

// Translated from 009cf400 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `+0xad` (`__thiscall`, `AL`; on the combat style, the power
/// attack chance the callers compare with a draw below 100).
pub fn fn_009cf400(e: &mut Engine, this: u32) -> i8 {
    e.mem.u8(this + 0xad) as i8
}

// Translated from 009cf420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the style allows the power attack `attack` (unnamed in the
/// map): only when the style's byte `005524b0` is above 0; then, when its
/// flag 1 (`009928c0`) is set, the group's own chance byte (`0073cb80`,
/// `00505c00`, `00505c20`, `006a9560`, `009e8670` for `0x5c..0x60`) must
/// be above 0.
pub fn fn_009cf420(e: &mut Engine, this: Ptr<CombatProcedureAttackMelee>, attack: u32) -> bool {
    let _attacker = e.call(GET_ATTACKER, &args![this]).u32();
    let mut allowed = false;
    let style = style_of(e, this);
    let score = e.call(STYLE_SCORE_5524B0, &args![style]).u8() as i8;
    if f64::from(score) > double_at(e, ZERO_DOUBLE) {
        allowed = true;
        let style = style_of(e, this);
        if e.call(STYLE_FLAG_TEST, &args![style, 1u32]).bool() {
            // The compiler's switch on `attack - 0x5c` (table `009cf55c`).
            let getter = match attack.wrapping_sub(0x5c) {
                0 => Some(STYLE_CHANCE_73CB80),
                1 => Some(STYLE_CHANCE_505C00),
                2 => Some(STYLE_CHANCE_505C20),
                3 => Some(STYLE_CHANCE_6A9560),
                4 => Some(STYLE_CHANCE_9E8670),
                _ => None,
            };
            if let Some(getter) = getter {
                let style = style_of(e, this);
                allowed = (e.call(getter, &args![style]).u8() as i8) > 0;
            }
        }
    }
    allowed
}

// Translated from 009cf570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs the combat state's step (unnamed in the map; switch table
/// `009cf64c`): 0 `HandleAttack`, 1 blocking (`009ccfb0`), 2 waiting
/// (`009cd3d0`), 3 the attack decision (`009cc510`) unless the target's
/// acquire object has an item (slot `+0x610`), the controller's test
/// `009818b0` holds, or the target's slot `+0x22c` (with 0) refuses.
pub fn fn_009cf570(e: &mut Engine, this: Ptr<CombatProcedureAttackMelee>) {
    let t = this.addr();
    let target = e.call(GET_TARGET, &args![t]).u32();
    match e.call(WORD_AT_14, &args![t]).u32() {
        0 => handle_attack(e, this),
        1 => fn_009ccfb0(e, this),
        2 => fn_009cd3d0(e, this),
        3 => {
            let item = if target != 0 && e.call(GET_SAVED_ACQUIRE_OBJECT, &args![target]).u32() != 0
            {
                let object = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![target]).u32();
                e.vcall(object, SLOT_TEST_610, &args![]).u32()
            } else {
                0
            };
            let controller = controller_of(e, this);
            if e.call(CONTROLLER_TEST_18B0, &args![controller]).bool() || item != 0 {
                return;
            }
            if target != 0 && e.vcall(target, SLOT_TEST_22C, &args![0u32]).bool() {
                return;
            }
            fn_009cc510(e, this);
        }
        _ => {}
    }
}

// Translated from 009cf660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the maneuver state (unnamed in the map; one word). Leaving the
/// backing-away state 4 for 0 is refused while the target moves away; a
/// change into or out of state 4 calls `00483710` on the controller; every
/// change resets the movement restrictions unless the attacker is pathing
/// (`009cfc00`).
pub fn fn_009cf660(e: &mut Engine, this: Ptr<CombatProcedureAttackMelee>, state: u32) {
    let current = e.get(this, CombatProcedureAttackMelee::eManeuverState);
    if current != state {
        if state == 0 && current == 4 && is_target_moving_away(e, this) {
            return;
        }
        if state == 4 || e.get(this, CombatProcedureAttackMelee::eManeuverState) == 4 {
            let controller = controller_of(e, this);
            e.call(EMPTY_HOOK, &args![controller]);
        }
        fn_009cfc00(e, this);
    }
    e.set(this, CombatProcedureAttackMelee::eManeuverState, state);
}

// Translated from 009cf6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the combat state (unnamed in the map; one word), keeping the
/// previous one in `eLastCombatState`.
pub fn fn_009cf6d0(e: &mut Engine, this: Ptr<CombatProcedureAttackMelee>, state: u32) {
    let current = e.get(this, CombatProcedureAttackMelee::eCombatState);
    e.set(this, CombatProcedureAttackMelee::eLastCombatState, current);
    e.set(this, CombatProcedureAttackMelee::eCombatState, state);
}

/// Timer text into `buffer`: `00406d00` with the timer's elapsed time and
/// duration as doubles (the duration is asked for first, as the exe pushes
/// it first).
fn print_timer(e: &mut Engine, buffer: u32, format: u32, text: Option<u32>, timer: u32) {
    let duration = e.call(TIMER_DURATION, &args![timer]).f64();
    let elapsed = e.call(TIMER_ELAPSED, &args![timer]).f64();
    let mut words = args![buffer, 0x64u32, format];
    if let Some(text) = text {
        words.extend(args![text]);
    }
    words.extend(args![elapsed, duration]);
    e.call(PRINT_TO_BUFFER, &words);
}

// Translated from 009cf700 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009cf700 - the /GS cookie check (00ec408c) on return is not translated
/// `CombatProcedureAttackMelee::DisplayDebugText` (Xbox PDB): prints the
/// maneuver (with the dodge direction and the maneuver timer), the combat
/// state (with the current attack or the melee timer), the
/// target-unreachable timer while it runs, the restricted directions
/// (when some are restricted and the attacker is not pathing) and whether
/// the restrictions were reset, through the text object's print slot.
pub fn display_debug_text(
    e: &mut Engine,
    this: Ptr<CombatProcedureAttackMelee>,
    text: u32,
    first: u32,
    second: u32,
) {
    const FRAME: u32 = 0x80;
    let t = this.addr();
    e.with_stack(FRAME, |e, base| {
        let ebp = base.addr() + FRAME;
        let buffer = ebp - 0x78;
        let attacker = e.call(GET_ATTACKER, &args![t]).u32();
        let moves = e.call(ACTOR_MOVE_FLAGS, &args![attacker]).u16();
        let empty = e.mem.u8(TEXT_EMPTY);
        e.mem.set_u8(buffer, empty);
        e.call(MEMSET, &args![buffer + 1, 0u32, 0x63u32]);
        match e.get(this, CombatProcedureAttackMelee::eManeuverState) {
            1 => print_timer(e, buffer, TEXT_TIMER, None, t + MANEUVER_TIMER),
            2 => {
                let direction = if moves & 1 != 0 {
                    TEXT_SPACE_FORWARD
                } else if moves & 2 != 0 {
                    TEXT_SPACE_BACKWARD
                } else if moves & 4 != 0 {
                    TEXT_SPACE_LEFT
                } else if moves & 8 != 0 {
                    TEXT_SPACE_RIGHT
                } else {
                    TEXT_EMPTY
                };
                print_timer(
                    e,
                    buffer,
                    TEXT_NAMED_TIMER,
                    Some(direction),
                    t + MANEUVER_TIMER,
                );
            }
            _ => e.mem.set_u8(buffer, 0),
        }
        let maneuver = e.get(this, CombatProcedureAttackMelee::eManeuverState);
        let name = e
            .mem
            .u32(MANEUVER_NAMES.wrapping_add(maneuver.wrapping_mul(4)));
        e.vcall(
            text,
            SLOT_PRINT,
            &args![first, second, TEXT_MANEUVER, name, buffer],
        );
        match e.get(this, CombatProcedureAttackMelee::eCombatState) {
            0 => {
                let attack = match e.get(this, CombatProcedureAttackMelee::eCurrentAttack) {
                    0x1a => TEXT_LEFT,
                    0x20 => TEXT_RIGHT,
                    0x5c => TEXT_NORMAL_POWER,
                    0x5d => TEXT_FORWARD_POWER,
                    0x5e => TEXT_BACK_POWER,
                    0x5f => TEXT_LEFT_POWER,
                    0x60 => TEXT_RIGHT_POWER,
                    _ => TEXT_EMPTY,
                };
                e.call(COPY_TO_BUFFER, &args![buffer, 0x64u32, attack]);
            }
            2 => print_timer(e, buffer, TEXT_TIMER, None, t + MELEE_TIMER),
            _ => e.mem.set_u8(buffer, 0),
        }
        if e.call(TIMER_RUNNING, &args![t + TARGET_UNREACHABLE_TIMER])
            .bool()
        {
            let duration = e
                .call(TIMER_DURATION, &args![t + TARGET_UNREACHABLE_TIMER])
                .f64();
            let elapsed = e
                .call(TIMER_ELAPSED, &args![t + TARGET_UNREACHABLE_TIMER])
                .f64();
            e.vcall(
                text,
                SLOT_PRINT,
                &args![first, second, TEXT_UNREACHABLE_TIMER, elapsed, duration],
            );
        }
        let state = e.get(this, CombatProcedureAttackMelee::eCombatState);
        let name = e
            .mem
            .u32(COMBAT_STATE_NAMES.wrapping_add(state.wrapping_mul(4)));
        e.vcall(
            text,
            SLOT_PRINT,
            &args![first, second, TEXT_MELEE, name, buffer],
        );
        let restrictions = restrictions_of(this);
        if e.call(RESTRICTED_MOVES, &args![restrictions]).u16() != 0
            && !e.call(IS_PATHING, &args![attacker]).bool()
        {
            let flag = |e: &mut Engine, moves: u16| {
                if fn_009cdde0(e, restrictions, moves) {
                    TEXT_Y
                } else {
                    TEXT_N
                }
            };
            let backward = flag(e, 2);
            let forward = flag(e, 1);
            let right = flag(e, 8);
            let left = flag(e, 4);
            e.vcall(
                text,
                SLOT_PRINT,
                &args![
                    first,
                    second,
                    TEXT_RESTRICTIONS,
                    left,
                    right,
                    forward,
                    backward
                ],
            );
        }
        if e.call(RESTRICTIONS_TEST_9373F0, &args![restrictions])
            .bool()
        {
            e.vcall(
                text,
                SLOT_PRINT,
                &args![first, second, TEXT_RESET_RESTRICTIONS],
            );
        }
    });
}

// Translated from 009cfbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether none of `moves` is restricted (`009cdde0` on the movement
/// restrictions; one word).
pub fn fn_009cfbd0(e: &mut Engine, this: Ptr<CombatProcedureAttackMelee>, moves: u16) -> bool {
    fn_009cdde0(e, restrictions_of(this), moves)
}

// Translated from 009cfc00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets the movement restrictions (`009cfc40` with 1) unless the
/// attacker is pathing.
pub fn fn_009cfc00(e: &mut Engine, this: Ptr<CombatProcedureAttackMelee>) {
    let attacker = e.call(GET_ATTACKER, &args![this]).u32();
    if !e.call(IS_PATHING, &args![attacker]).bool() {
        fn_009cfc40(e, restrictions_of(this), 1);
    }
}

// Translated from 009cfc40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `bReset` of the `CombatMovementRestrictions` (one stack word, not
/// read).
pub fn fn_009cfc40(e: &mut Engine, this: Ptr<CombatMovementRestrictions>, _unused_1: u32) {
    e.set(this, CombatMovementRestrictions::bReset, true);
}

// Translated from 009cfc60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureAttackMelee::StopMoving` (Xbox PDB): `008b3a50` and
/// `Actor::StopMoving` on the attacker, then the restrictions distance
/// check.
pub fn stop_moving(e: &mut Engine, this: Ptr<CombatProcedureAttackMelee>) {
    let attacker = e.call(GET_ATTACKER, &args![this]).u32();
    e.call(ACTOR_CLEAR_8B3A50, &args![attacker]);
    e.call(STOP_MOVING_ACTOR, &args![attacker]);
    check_movement_restrictions_distance(e, this);
}

// Translated from 009cfc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureAttackMelee::CheckMovementRestrictionsDistance` (Xbox
/// PDB): unless `00525420` holds, resets the restrictions (`009cfc00`) once
/// the attacker is at least `fAIUpdateMovementRestrictionsDistance` from
/// where they were computed (squared distances).
pub fn check_movement_restrictions_distance(e: &mut Engine, this: Ptr<CombatProcedureAttackMelee>) {
    if e.call(PROCEDURE_TEST_525420, &args![this]).bool() {
        return;
    }
    let attacker = e.call(GET_ATTACKER, &args![this]).u32();
    let squared = e.with_stack(0xc, |e, difference| {
        let position = e.vcall(attacker, SLOT_POSITION, &args![]).u32();
        let location = e
            .call(RESTRICTIONS_POINT, &args![restrictions_of(this)])
            .u32();
        e.call(POINT_DIFFERENCE, &args![location, difference, position]);
        e.call(POINT_LENGTH_SQUARED, &args![difference]).f64()
    });
    let first = setting_bits(e, SETTING_AI_UPDATE_MOVEMENT_RESTRICTIONS_DISTANCE);
    let second = setting_bits(e, SETTING_AI_UPDATE_MOVEMENT_RESTRICTIONS_DISTANCE);
    let limit = x87_mul(x87_load32(first), x87_m32(second));
    // FCOMP squared; TEST AH,0x41; JP: resets when the limit is less or
    // equal.
    if limit <= squared {
        fn_009cfc00(e, this);
    }
}

// Translated from 009cfd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureAttackMelee::GetFacingTargetDistance` (Xbox PDB): the
/// cached distance, computed
/// (`CombatUtilities::GetDistanceBetweenFacingActors`) while it is below
/// 0.
pub fn get_facing_target_distance(e: &mut Engine, this: Ptr<CombatProcedureAttackMelee>) -> f32 {
    let t = this.addr();
    // FCOMP 0.0; TEST AH,5; JP: computed only when below 0.
    if x87_load32(e.mem.u32(t + 0x5c)) < double_at(e, ZERO_DOUBLE) {
        let target = e.call(GET_TARGET, &args![t]).u32();
        let attacker = e.call(GET_ATTACKER, &args![t]).u32();
        let distance = e
            .call(
                GET_DISTANCE_BETWEEN_FACING_ACTORS,
                &args![attacker, target, 0u32],
            )
            .f64();
        e.mem.set_u32(t + 0x5c, x87_store32(distance));
    }
    f32::from_bits(x87_quiet_bits(e.mem.u32(t + 0x5c)))
}

// Translated from 009cfd70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureAttackMelee::IsTargetMovingAway` (Xbox PDB): with a
/// moving target (`004938e0`), whether the attacker is at least the facing
/// distance from where the target is heading (its position plus its
/// horizontal velocity, slot `+0x2ac`), compared squared.
pub fn is_target_moving_away(e: &mut Engine, this: Ptr<CombatProcedureAttackMelee>) -> bool {
    let t = this.addr();
    e.with_stack(0x30, |e, frame| {
        // A copy of the default point the function makes and never reads.
        let unused_point = frame.addr();
        let velocity = unused_point + 0xc;
        let heading = velocity + 0xc;
        let difference = heading + 0xc;
        for i in 0..3 {
            let word = e.mem.u32(DEFAULT_POINT + i * 4);
            e.mem.set_u32(unused_point + i * 4, word);
        }
        for i in 0..3 {
            let word = e.mem.u32(DEFAULT_POINT + i * 4);
            e.mem.set_u32(velocity + i * 4, word);
        }
        let target = e.call(GET_TARGET, &args![t]).u32();
        if target == 0 {
            return false;
        }
        let target = e.call(GET_TARGET, &args![t]).u32();
        if !e.call(ACTOR_TEST_4938E0, &args![target]).bool() {
            return false;
        }
        let target = e.call(GET_TARGET, &args![t]).u32();
        e.vcall(target, SLOT_VELOCITY, &args![velocity]);
        e.mem.set_u32(velocity + 8, 0.0f32.to_bits());
        let target = e.call(GET_TARGET, &args![t]).u32();
        let position = e.vcall(target, SLOT_POSITION, &args![]).u32();
        e.call(POINT_SUM, &args![position, heading, velocity]);
        let facing = get_facing_target_distance(e, this).to_bits();
        let attacker = e.call(GET_ATTACKER, &args![t]).u32();
        let position = e.vcall(attacker, SLOT_POSITION, &args![]).u32();
        e.call(POINT_DIFFERENCE, &args![position, difference, heading]);
        let squared = x87_store32(e.call(POINT_LENGTH_SQUARED, &args![difference]).f64());
        let limit = x87_mul(x87_load32(facing), x87_m32(facing));
        // FLD squared; FCOMPP; TEST AH,1; JNZ: away unless less.
        x87_load32(squared) >= limit
    })
}

// Translated from 009cfe90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureAttackMelee::GetMovingTargetVelocity` (Xbox PDB): the
/// length of the target's velocity (slot `+0x2ac`, `ST0`), 0 without a
/// moving target (`004938e0`).
pub fn get_moving_target_velocity(e: &mut Engine, this: Ptr<CombatProcedureAttackMelee>) -> f64 {
    let t = this.addr();
    let target = e.call(GET_TARGET, &args![t]).u32();
    if target == 0 {
        return 0.0;
    }
    let target = e.call(GET_TARGET, &args![t]).u32();
    if !e.call(ACTOR_TEST_4938E0, &args![target]).bool() {
        return 0.0;
    }
    e.with_stack(0xc, |e, velocity| {
        for i in 0..3 {
            let word = e.mem.u32(DEFAULT_POINT + i * 4);
            e.mem.set_u32(velocity.addr() + i * 4, word);
        }
        let target = e.call(GET_TARGET, &args![t]).u32();
        e.vcall(target, SLOT_VELOCITY, &args![velocity]);
        e.call(POINT_LENGTH, &args![velocity]).f64()
    })
}

// Translated from 009cff10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatMovementRestrictions`' constructor (unnamed in the map): clears
/// the flags and the moves, copies the default point into `Location`,
/// constructs `UpdateTimer` and returns `this`.
pub fn fn_009cff10(
    e: &mut Engine,
    this: Ptr<CombatMovementRestrictions>,
) -> Ptr<CombatMovementRestrictions> {
    e.set(this, CombatMovementRestrictions::bReset, false);
    e.set(this, CombatMovementRestrictions::bInitialized, false);
    e.set(this, CombatMovementRestrictions::cCurrentUpdate, 0);
    e.set(this, CombatMovementRestrictions::sRestrictedMoves, 0);
    e.set(this, CombatMovementRestrictions::bHasMoveLeftAnim, false);
    e.set(this, CombatMovementRestrictions::bHasMoveRightAnim, false);
    e.set(this, CombatMovementRestrictions::bHasMoveForwardAnim, false);
    e.set(
        this,
        CombatMovementRestrictions::bHasMoveBackwardAnim,
        false,
    );
    for i in 0..3 {
        let word = e.mem.u32(DEFAULT_POINT + i * 4);
        e.mem
            .set_u32(this.addr() + RESTRICTIONS_LOCATION + i * 4, word);
    }
    e.call(
        TIMER_CONSTRUCT,
        &args![this.addr() + RESTRICTIONS_UPDATE_TIMER],
    );
    this
}

/// What one direction of `fn_009cff90` uses: where its point lives in the
/// frame (`EBP - point`), the out points of the rotation and the sum, the
/// move bit that lengthens it, the offset of the coordinate that grows
/// (+0 x, +4 y) and whether it shrinks, the animation flag (offset in the
/// restrictions) and the restriction bit.
struct Direction {
    point: u32,
    rotated: u32,
    summed: u32,
    move_bit: u32,
    coordinate: u32,
    subtract: bool,
    anim_flag: u32,
    bit: u16,
}

// Translated from 009cff90 (decompiled, FalloutNV.exe 1.4.0.525)
/// One update step of the `CombatMovementRestrictions` (unnamed in the
/// map; two words: the actor and the direction 0..3): on the first call it
/// asks the actor's acquire object (slot `+0x408`) which move animations
/// exist; while not reset it waits for the actor to stop pathing and, for
/// direction 0, to move `fAIUpdateMovementRestrictionsDistance` (returning
/// 4). Then it stores the actor's position and checks the direction (0
/// back, 1 forward, 2 left, 3 right; switch table `009d0518`) one body
/// length (the width for the sides; both at most by `0040ebd0` with 64,
/// made equal when within a factor of two) away, lengthened when the actor
/// already moves that way and rotated by the actor's heading, by line of
/// sight (`009d0590`, only when the animation exists), and records the
/// result (`009d0530`). Returns 1.
pub fn fn_009cff90(
    e: &mut Engine,
    this: Ptr<CombatMovementRestrictions>,
    actor: u32,
    direction: u32,
) -> u32 {
    const FRAME: u32 = 0x100;
    let r = this.addr();
    if !e.get(this, CombatMovementRestrictions::bInitialized) {
        for (offset, animation) in [(0xau32, 5u32), (0xb, 6), (0xc, 3), (0xd, 4)] {
            let object = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![actor]).u32();
            let has = e.vcall(object, SLOT_HAS_MOVE_ANIM, &args![animation]).u8();
            e.mem.set_u8(r + offset, has);
        }
        e.set(this, CombatMovementRestrictions::bInitialized, true);
    }
    if !e.get(this, CombatMovementRestrictions::bReset) && e.call(IS_PATHING, &args![actor]).bool()
    {
        return 4;
    }
    e.with_stack(FRAME, |e, base| {
        let ebp = base.addr() + FRAME;
        let matrix = ebp - 0x24;
        if !e.get(this, CombatMovementRestrictions::bReset) && direction == 0 {
            let moved = ebp - 0x40;
            let position = e.vcall(actor, SLOT_POSITION, &args![]).u32();
            e.call(
                POINT_DIFFERENCE,
                &args![r + RESTRICTIONS_LOCATION, moved, position],
            );
            let distance = e.call(POINT_LENGTH_457990, &args![moved]).f64();
            let limit = setting_bits(e, SETTING_AI_UPDATE_MOVEMENT_RESTRICTIONS_DISTANCE);
            // FLD limit; FCOMP distance; TEST AH,0x41; JNZ: waits while the
            // limit is greater.
            if x87_load32(limit) > distance {
                return 4;
            }
        }
        let position = e.vcall(actor, SLOT_POSITION, &args![]).u32();
        for i in 0..3 {
            let word = e.mem.u32(position + i * 4);
            e.mem.set_u32(r + RESTRICTIONS_LOCATION + i * 4, word);
        }
        e.call(POINT_CONSTRUCT, &args![matrix]);
        let heading = e.vcall(actor, SLOT_HEADING, &args![0u32]).f64();
        e.call(MATRIX_FROM_HEADING, &args![matrix, x87_store32(heading)]);
        let mut width = x87_store32(e.call(ACTOR_WIDTH, &args![actor]).f64());
        // FCOMP 0.0; TEST AH,0x44; JP: 0 becomes 32.
        if x87_load32(width) == double_at(e, ZERO_DOUBLE) {
            width = x87_quiet_bits(e.global::<u32>(THIRTY_TWO_FLOAT));
        }
        let mut length = x87_store32(e.call(ACTOR_LENGTH, &args![actor]).f64());
        if x87_load32(length) == double_at(e, ZERO_DOUBLE) {
            length = width;
        }
        let ratio = x87_store32(x87_div(x87_load32(width), x87_m32(length)));
        if x87_load32(ratio) > double_at(e, HALF_DOUBLE)
            && x87_load32(ratio) < double_at(e, TWO_DOUBLE)
        {
            width = x87_store32(e.call(FLOAT_MAX, &args![width, length]).f64());
            length = width;
        }
        let sixty_four = x87_quiet_bits(e.global::<u32>(SIXTY_FOUR_FLOAT));
        width = x87_store32(e.call(FLOAT_PAIR_EBD0, &args![width, sixty_four]).f64());
        let sixty_four = x87_quiet_bits(e.global::<u32>(SIXTY_FOUR_FLOAT));
        length = x87_store32(e.call(FLOAT_PAIR_EBD0, &args![length, sixty_four]).f64());
        let mode = e.call(ACTOR_MOVE_FLAGS, &args![actor]).u32();
        let zero = 0.0f32.to_bits();
        let (x, y, size, step) = match direction {
            0 => (
                zero,
                x87_store32(-x87_load32(length)),
                length,
                Direction {
                    point: 0x4c,
                    rotated: 0x7c,
                    summed: 0x88,
                    move_bit: 2,
                    coordinate: 4,
                    subtract: true,
                    anim_flag: 0xd,
                    bit: 2,
                },
            ),
            1 => (
                zero,
                length,
                length,
                Direction {
                    point: 0x58,
                    rotated: 0x94,
                    summed: 0xa0,
                    move_bit: 1,
                    coordinate: 4,
                    subtract: false,
                    anim_flag: 0xc,
                    bit: 1,
                },
            ),
            2 => (
                x87_store32(-x87_load32(width)),
                zero,
                width,
                Direction {
                    point: 0x64,
                    rotated: 0xac,
                    summed: 0xb8,
                    move_bit: 4,
                    coordinate: 0,
                    subtract: true,
                    anim_flag: 0xa,
                    bit: 4,
                },
            ),
            3 => (
                width,
                zero,
                width,
                Direction {
                    point: 0x70,
                    rotated: 0xc4,
                    summed: 0xd0,
                    move_bit: 8,
                    coordinate: 0,
                    subtract: false,
                    anim_flag: 0xb,
                    bit: 8,
                },
            ),
            _ => return 1,
        };
        let point = ebp - step.point;
        e.call(POINT_SET, &args![point, x, y, zero]);
        if mode & step.move_bit != 0 {
            let at = point + step.coordinate;
            let value = if step.subtract {
                x87_sub(x87_load32(e.mem.u32(at)), x87_m32(size))
            } else {
                x87_add(x87_load32(e.mem.u32(at)), x87_m32(size))
            };
            e.mem.set_u32(at, x87_store32(value));
        }
        let rotated = e
            .call(
                MATRIX_TIMES_POINT,
                &args![matrix, ebp - step.rotated, point],
            )
            .u32();
        let moved = e
            .call(
                POINT_SUM,
                &args![rotated, ebp - step.summed, r + RESTRICTIONS_LOCATION],
            )
            .u32();
        for i in 0..3 {
            let word = e.mem.u32(moved + i * 4);
            e.mem.set_u32(point + i * 4, word);
        }
        let clear = if e.mem.u8(r + step.anim_flag) != 0 {
            fn_009d0590(e, this, actor, point)
        } else {
            false
        };
        fn_009d0530(e, this, step.bit, clear);
        1
    })
}

// Translated from 009d0530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears (`allowed`) or sets the restriction bits `moves` (two words);
/// `AX` holds the new `sRestrictedMoves` at the return.
pub fn fn_009d0530(
    e: &mut Engine,
    this: Ptr<CombatMovementRestrictions>,
    moves: u16,
    allowed: bool,
) -> u16 {
    let current = e.get(this, CombatMovementRestrictions::sRestrictedMoves);
    let new = if allowed {
        current & !moves
    } else {
        current | moves
    };
    e.set(this, CombatMovementRestrictions::sRestrictedMoves, new);
    new
}

// Translated from 009d0590 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009d0590 - the compiler's FS:[0] exception frame (unwinding of the pathing location) is not translated
/// Whether `point` is in line of sight from the actor's pathing location
/// (`Pathing::CheckLineOfSight_ov2` with 0, 0; two words).
pub fn fn_009d0590(
    e: &mut Engine,
    _this: Ptr<CombatMovementRestrictions>,
    actor: u32,
    point: u32,
) -> bool {
    e.with_stack(0x28, |e, location| {
        e.call(PATHING_LOCATION_FROM_ACTOR, &args![location, actor]);
        let clear = e
            .call(CHECK_LINE_OF_SIGHT, &args![location, point, 0u32, 0u32])
            .bool();
        e.call(PATHING_LOCATION_DESTRUCT, &args![location]);
        clear
    })
}

// Translated from 009d0610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Updates the `CombatMovementRestrictions` (one word: the controller):
/// while the update timer ran out or a round is under way, checks the
/// next direction (`009cff90` with the controller's actor) and advances by
/// its result; after the fourth it starts a new round
/// (`fMeleeMovementRestrictionsUpdateTime`).
pub fn fn_009d0610(e: &mut Engine, this: Ptr<CombatMovementRestrictions>, controller: u32) {
    let r = this.addr();
    if e.call(TIMER_EXPIRED, &args![r + RESTRICTIONS_UPDATE_TIMER])
        .bool()
        || e.get(this, CombatMovementRestrictions::cCurrentUpdate) > 0
    {
        let direction = e.get(this, CombatMovementRestrictions::cCurrentUpdate);
        let actor = e.call(CONTROLLER_ACTOR, &args![controller]).u32();
        let step = fn_009cff90(e, this, actor, direction);
        let current = e.get(this, CombatMovementRestrictions::cCurrentUpdate);
        e.set(
            this,
            CombatMovementRestrictions::cCurrentUpdate,
            step.wrapping_add(current),
        );
    }
    if e.get(this, CombatMovementRestrictions::cCurrentUpdate) >= 4 {
        e.set(this, CombatMovementRestrictions::bReset, false);
        e.set(this, CombatMovementRestrictions::cCurrentUpdate, 0);
        let time = setting_bits(e, SETTING_MELEE_MOVEMENT_RESTRICTIONS_UPDATE_TIME);
        e.call(
            TIMER_START,
            &args![r + RESTRICTIONS_UPDATE_TIMER, x87_quiet_bits(time)],
        );
    }
}

/// The timers `SaveGame` and `LoadGame` write and read, in order.
const SAVED_TIMERS: [u32; 6] = [
    MELEE_TIMER,
    MANEUVER_TIMER,
    DODGE_TIMER,
    ADVANCE_TIMER,
    CLOSE_TIMER,
    TARGET_UNREACHABLE_TIMER,
];

// Translated from 009d0690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureAttackMelee::SaveGame` (Xbox PDB): the base class, the
/// four state words, the six timers, the time stamp, the ally flag and the
/// advance move mode.
pub fn save_game(e: &mut Engine, this: Ptr<CombatProcedureAttackMelee>, buffer: u32) {
    let t = this.addr();
    e.call(BASE_SAVE_GAME, &args![t, buffer]);
    for offset in [0x10u32, 0x14, 0x18, 0x1c] {
        e.call(SAVE_BYTES, &args![buffer, t + offset, 4u32, 0u32]);
    }
    for timer in SAVED_TIMERS {
        e.call(TIMER_SAVE, &args![t + timer, buffer]);
    }
    e.call(
        TIMESTAMP_SAVE,
        &args![t + ATTACK_PICK_FAILED_TIMESTAMP, buffer],
    );
    e.call(SAVE_BYTES, &args![buffer, t + 0x50, 1u32, 0u32]);
    e.call(SAVE_BYTES, &args![buffer, t + 0x54, 4u32, 0u32]);
}

// Translated from 009d0790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureAttackMelee::LoadGame` (Xbox PDB): the mirror of
/// `SaveGame`; the time stamp is only in saves of version 18 or later (the
/// buffer's version is its virtual function at slot 0).
pub fn load_game(e: &mut Engine, this: Ptr<CombatProcedureAttackMelee>, buffer: u32) {
    let t = this.addr();
    e.call(BASE_LOAD_GAME, &args![t, buffer]);
    for offset in [0x10u32, 0x14, 0x18, 0x1c] {
        e.call(LOAD_BYTES, &args![buffer, t + offset, 4u32]);
    }
    for timer in SAVED_TIMERS {
        e.call(TIMER_LOAD, &args![t + timer, buffer]);
    }
    if e.vcall(buffer, 0, &args![]).u8() >= 0x12 {
        e.call(
            TIMESTAMP_LOAD,
            &args![t + ATTACK_PICK_FAILED_TIMESTAMP, buffer],
        );
    }
    e.call(LOAD_BYTES, &args![buffer, t + 0x50, 1u32]);
    e.call(LOAD_BYTES, &args![buffer, t + 0x54, 4u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006ecd40, fn_006ecd40(Ptr, u32)),
        entry!(0x009c_c4b0, fn_009cc4b0(Ptr, u32)),
        entry!(0x009c_dac0, fn_009cdac0(Ptr) -> f32),
        entry!(0x009c_dae0, fn_009cdae0(Ptr) -> f32),
        entry!(0x009c_c240, update(Ptr<CombatProcedureAttackMelee>)),
        entry!(0x009c_c490, fn_009cc490(u32, u32)),
        entry!(0x009c_c4d0, shutdown(Ptr<CombatProcedureAttackMelee>)),
        entry!(0x009c_c510, fn_009cc510(Ptr<CombatProcedureAttackMelee>)),
        entry!(
            0x009c_cb20,
            fn_009ccb20(
                Ptr<CombatProcedureAttackMelee>,
                bool,
                f32,
                f32,
                bool,
                bool,
                bool,
                Ptr<u8>,
                Ptr<u8>,
            ) -> u32
        ),
        entry!(
            0x009c_ce70,
            start_attack(Ptr<CombatProcedureAttackMelee>, u32, u32)
        ),
        entry!(0x009c_ced0, handle_attack(Ptr<CombatProcedureAttackMelee>)),
        entry!(0x009c_cfb0, fn_009ccfb0(Ptr<CombatProcedureAttackMelee>)),
        entry!(0x009c_d150, fn_009cd150(Ptr<CombatProcedureAttackMelee>)),
        entry!(0x009c_d3d0, fn_009cd3d0(Ptr<CombatProcedureAttackMelee>)),
        entry!(0x009c_d410, fn_009cd410(Ptr<CombatProcedureAttackMelee>)),
        entry!(0x009c_d430, fn_009cd430(Ptr<CombatProcedureAttackMelee>)),
        entry!(
            0x009c_d5e0,
            maneuver_decision(Ptr<CombatProcedureAttackMelee>)
        ),
        entry!(0x009c_db00, handle_dodge(Ptr<CombatProcedureAttackMelee>)),
        entry!(
            0x009c_dde0,
            fn_009cdde0(Ptr<CombatMovementRestrictions>, u16) -> bool
        ),
        entry!(0x009c_de10, fn_009cde10(Ptr<CombatProcedureAttackMelee>)),
        entry!(0x009c_df00, fn_009cdf00(Ptr<CombatProcedureAttackMelee>)),
        entry!(0x009c_df80, fn_009cdf80(Ptr<CombatProcedureAttackMelee>)),
        entry!(0x009c_e190, fn_009ce190(Ptr<CombatProcedureAttackMelee>)),
        entry!(0x009c_e1d0, fn_009ce1d0(Ptr<CombatProcedureAttackMelee>)),
        entry!(
            0x009c_e560,
            get_current_attack_reach(Ptr<CombatProcedureAttackMelee>) -> f32
        ),
        entry!(
            0x009c_e5b0,
            fn_009ce5b0(Ptr<CombatProcedureAttackMelee>, u32)
        ),
        entry!(
            0x009c_e870,
            select_power_attack(Ptr<CombatProcedureAttackMelee>, f32, f32, u32, u32, u32) -> u32
        ),
        entry!(0x009c_ef70, fn_009cef70(u32, u32) -> f32),
        entry!(0x009c_ef90, fn_009cef90(u32, u32) -> f32),
        entry!(0x009c_efb0, fn_009cefb0(u32) -> bool),
        entry!(
            0x009c_efd0,
            fn_009cefd0(Ptr<CombatProcedureAttackMelee>, f32, f32, u32) -> bool
        ),
        entry!(
            0x009c_f030,
            outside_attack_range(Ptr<CombatProcedureAttackMelee>, f32, f32, u32) -> bool
        ),
        entry!(
            0x009c_f090,
            fn_009cf090(Ptr<CombatProcedureAttackMelee>, f32, f32)
        ),
        entry!(0x009c_f400, fn_009cf400(u32) -> i8),
        entry!(
            0x009c_f420,
            fn_009cf420(Ptr<CombatProcedureAttackMelee>, u32) -> bool
        ),
        entry!(0x009c_f570, fn_009cf570(Ptr<CombatProcedureAttackMelee>)),
        entry!(
            0x009c_f660,
            fn_009cf660(Ptr<CombatProcedureAttackMelee>, u32)
        ),
        entry!(
            0x009c_f6d0,
            fn_009cf6d0(Ptr<CombatProcedureAttackMelee>, u32)
        ),
        entry!(
            0x009c_f700,
            display_debug_text(Ptr<CombatProcedureAttackMelee>, u32, u32, u32)
        ),
        entry!(
            0x009c_fbd0,
            fn_009cfbd0(Ptr<CombatProcedureAttackMelee>, u16) -> bool
        ),
        entry!(0x009c_fc00, fn_009cfc00(Ptr<CombatProcedureAttackMelee>)),
        entry!(
            0x009c_fc40,
            fn_009cfc40(Ptr<CombatMovementRestrictions>, u32)
        ),
        entry!(0x009c_fc60, stop_moving(Ptr<CombatProcedureAttackMelee>)),
        entry!(
            0x009c_fc90,
            check_movement_restrictions_distance(Ptr<CombatProcedureAttackMelee>)
        ),
        entry!(
            0x009c_fd20,
            get_facing_target_distance(Ptr<CombatProcedureAttackMelee>) -> f32
        ),
        entry!(
            0x009c_fd70,
            is_target_moving_away(Ptr<CombatProcedureAttackMelee>) -> bool
        ),
        entry!(
            0x009c_fe90,
            get_moving_target_velocity(Ptr<CombatProcedureAttackMelee>) -> f64
        ),
        entry!(
            0x009c_ff10,
            fn_009cff10(Ptr<CombatMovementRestrictions>) -> Ptr<CombatMovementRestrictions>
        ),
        entry!(
            0x009c_ff90,
            fn_009cff90(Ptr<CombatMovementRestrictions>, u32, u32) -> u32
        ),
        entry!(
            0x009d_0530,
            fn_009d0530(Ptr<CombatMovementRestrictions>, u16, bool) -> u16
        ),
        entry!(
            0x009d_0590,
            fn_009d0590(Ptr<CombatMovementRestrictions>, u32, u32) -> bool
        ),
        entry!(
            0x009d_0610,
            fn_009d0610(Ptr<CombatMovementRestrictions>, u32)
        ),
        entry!(0x009d_0690, save_game(Ptr<CombatProcedureAttackMelee>, u32)),
        entry!(0x009d_0790, load_game(Ptr<CombatProcedureAttackMelee>, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_the_word_at_4() {
        let mut e = Engine::new();
        let block = e.mem.alloc(12);
        e.mem.write(block, &[0xa5; 12]);
        e.call(0x006e_cd40, &args![block, 0x1234_5678u32]);
        assert_eq!(e.mem.u32(block + 4), 0x1234_5678);
        assert_eq!(e.mem.u32(block), 0xa5a5_a5a5);
        assert_eq!(e.mem.u32(block + 8), 0xa5a5_a5a5);
    }

    #[test]
    fn forwarder_fn_009cc4b0_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(STORE_TARGET_006ECD40, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x009c_c4b0, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == STORE_TARGET_006ECD40)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }

    #[test]
    fn fn_009cdac0_and_009cdae0_load_the_floats_at_60_and_64() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x68);
        e.mem.set_f32(this + 0x60, 2.5);
        e.mem.set_u32(this + 0x64, 0xff80_0001);
        assert_eq!(e.call(0x009c_dac0, &args![this]).f32(), 2.5);
        let loaded = e.call(0x009c_dae0, &args![this]).st0;
        assert_eq!((loaded as f32).to_bits(), 0xffc0_0001);
    }

    use std::cell::RefCell;
    use std::collections::{HashMap, VecDeque};
    use std::rc::Rc;

    type Log = Vec<(u32, Vec<u32>)>;

    /// Every fake object's vtable: slot `n` holds `SLOT_BASE + n`, a double.
    const FAKE_VTABLE: u32 = 0x7e00_0000;
    const SLOT_BASE: u32 = 0x7e10_0000;
    const SLOTS: u32 = 0x800;
    /// Where the setting double puts each setting's value.
    const SETTING_CELLS: u32 = 0x7e20_0000;
    const SETTING_BASE: u32 = 0x011c_0000;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn float(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    /// What the doubles answer: a queue that is used up first, then a
    /// fixed answer, then 0.
    #[derive(Default)]
    struct Script {
        queues: HashMap<u32, VecDeque<Ret>>,
        fixed: HashMap<u32, Ret>,
    }

    struct Rig {
        e: Engine,
        script: Rc<RefCell<Script>>,
    }

    impl Rig {
        fn new() -> Rig {
            let mut e = Engine::new();
            e.map(FAKE_VTABLE, SLOTS);
            for slot in (0..SLOTS).step_by(4) {
                e.mem.set_u32(FAKE_VTABLE + slot, SLOT_BASE + slot);
            }
            e.map(SETTING_CELLS, 0x4_0000);
            exe_data(&mut e);
            let mut rig = Rig {
                e,
                script: Rc::default(),
            };
            for &addr in CALLEES {
                rig.double(addr);
            }
            for slot in (0..SLOTS).step_by(4) {
                rig.double(SLOT_BASE + slot);
            }
            rig.e.register(SETTING_VALUE, |_, a| {
                ret(SETTING_CELLS + (a[0] - SETTING_BASE))
            });
            rig
        }

        fn double(&mut self, addr: u32) {
            let script = Rc::clone(&self.script);
            self.e.register_double(addr, move |_, _| {
                let mut script = script.borrow_mut();
                if let Some(answer) = script.queues.get_mut(&addr).and_then(VecDeque::pop_front) {
                    return answer;
                }
                script.fixed.get(&addr).copied().unwrap_or_default()
            });
        }

        fn answer(&self, addr: u32, value: u32) {
            self.script.borrow_mut().fixed.insert(addr, ret(value));
        }

        fn answer_float(&self, addr: u32, value: f64) {
            self.script.borrow_mut().fixed.insert(addr, float(value));
        }

        fn queue(&self, addr: u32, values: &[u32]) {
            self.script
                .borrow_mut()
                .queues
                .insert(addr, values.iter().map(|&v| ret(v)).collect());
        }

        /// The answer of the fake vtable's slot `slot`.
        fn slot(&self, slot: u32, value: u32) {
            self.answer(SLOT_BASE + slot, value);
        }

        fn setting(&mut self, setting: u32, value: f32) {
            self.e
                .mem
                .set_f32(SETTING_CELLS + (setting - SETTING_BASE), value);
        }

        /// A zeroed block whose first word is the fake vtable.
        fn object(&mut self, size: u32) -> u32 {
            let block = self.e.mem.alloc(size);
            self.e.mem.set_u32(block, FAKE_VTABLE);
            block
        }

        /// An actor: a fake object whose `ActorValueOwner` (`+0xa4`) has the
        /// fake vtable too.
        fn actor(&mut self) -> u32 {
            let actor = self.object(0x200);
            self.e.mem.set_u32(actor + 0xa4, FAKE_VTABLE);
            actor
        }

        fn run(&mut self, addr: u32, words: &[u32]) -> (Ret, Log) {
            self.e.call_log = Some(vec![]);
            let result = self.e.call(addr, words);
            (result, self.e.call_log.take().unwrap())
        }
    }

    fn calls_to(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn slot_calls(log: &Log, slot: u32) -> Vec<Vec<u32>> {
        calls_to(log, SLOT_BASE + slot)
    }

    fn bits(value: f32) -> u32 {
        value.to_bits()
    }

    /// The `float` and `double` constants of the exe the translations read
    /// (their bit patterns, read from the exe with the Ghidra server).
    const EXE_FLOATS: [(u32, u32); 22] = [
        (0x0101_2054, 0xbf80_0000),
        (0x0101_3ea4, 0x3c23_d70a),
        (0x0101_622c, 0x3e80_0000),
        (0x0101_6248, 0x3f00_0000),
        (0x0101_62c0, 0x4000_0000),
        (0x0101_712c, 0x40a0_0000),
        (0x0101_7718, 0x4040_0000),
        (0x0101_7b78, 0x4120_0000),
        (0x0101_e2bc, 0x3dcc_cccd),
        (0x0101_e340, 0x4200_0000),
        (0x0101_e57c, 0x4180_0000),
        (0x0101_e580, 0x4170_0000),
        (0x0101_e6ec, 0x4280_0000),
        (0x0101_e704, 0x4300_0000),
        (0x0101_ff50, 0x40c9_0fdb),
        (0x0102_2958, 0x461c_4000),
        (0x0103_2adc, 0x4380_0000),
        (0x0104_e0e8, 0x41c0_0000),
        (0x0105_0b98, 0x43c0_0000),
        (0x0105_36b8, 0x4340_0000),
        (0x0109_0e90, 0x7f7f_ffff),
        (0x0109_14d0, 0x7f7f_ffff),
    ];
    const EXE_DOUBLES: [(u32, u64); 22] = [
        (0x0101_1588, 0x3fe0_0000_0000_0000),
        (0x0101_1590, 0x4000_0000_0000_0000),
        (0x0101_2060, 0x0000_0000_0000_0000),
        (0x0101_2070, 0x3ff0_0000_0000_0000),
        (0x0101_7a10, 0x40b0_0000_0000_0000),
        (0x0101_7a40, 0x4059_0000_0000_0000),
        (0x0101_db80, 0x4010_0000_0000_0000),
        (0x0101_de30, 0x3fe8_0000_0000_0000),
        (0x0101_ecb8, 0x4090_0000_0000_0000),
        (0x0102_0758, 0x4024_0000_0000_0000),
        (0x0102_0998, 0x4014_0000_0000_0000),
        (0x0102_1798, 0x3ff4_0000_0000_0000),
        (0x0102_3128, 0x3f91_df46_a000_0000),
        (0x0102_31d8, 0x4070_0000_0000_0000),
        (0x0102_f248, 0x404c_a5dc_0000_0000),
        (0x0104_f2f0, 0x4039_0000_0000_0000),
        (0x0108_cb08, 0x4122_0000_0000_0000),
        (0x0108_d6e8, 0x4002_0000_0000_0000),
        (0x0108_e058, 0x40f0_0000_0000_0000),
        (0x0109_0d68, 0x3fe5_1eb8_6000_0000),
        (0x0109_1138, 0x4110_0000_0000_0000),
        (0x0109_1b28, 0x4083_8800_0000_0000),
    ];

    /// Maps the exe data the translations read (constants, globals) and
    /// writes the constants' values; the globals start out zeroed.
    fn exe_data(e: &mut Engine) {
        for page in [
            0x011a_5000u32,
            0x011a_8000,
            0x011d_e000,
            0x011f_1000,
            0x011f_4000,
        ] {
            e.map(page, 0x1000);
        }
        for (addr, value) in EXE_FLOATS {
            e.map(addr & !0xfff, 0x1000);
            e.mem.set_u32(addr, value);
        }
        for (addr, value) in EXE_DOUBLES {
            e.map(addr & !0xfff, 0x1000);
            e.mem.set_u64(addr, value);
        }
        // The empty string at `01011584` and the strings the code copies.
        e.mem.set_u8(0x0101_1584, 0);
    }

    /// Every function outside the file that the new translations call by
    /// address (the others are direct calls).
    const CALLEES: &[u32] = &[
        GET_ATTACKER,
        GET_TARGET,
        SET_MOVE_MODE,
        SET_LOOK_AT_TARGET,
        CLEAR_LOOK_AT_TARGET,
        BASE_SAVE_GAME,
        BASE_LOAD_GAME,
        FAIL_PROCEDURE,
        WORD_AT_10,
        WORD_AT_14,
        EMPTY_HOOK,
        GET_COMBAT_STATE,
        CONTROLLER_STYLE,
        CONTROLLER_WEAPON,
        CONTROLLER_ACTOR,
        IS_SWITCHING_WEAPONS,
        CONTROLLER_TEST_F2D0,
        CONTROLLER_TEST_1900,
        CONTROLLER_TEST_18B0,
        CONTROLLER_SET_18E0,
        CONTROLLER_VALUE_FC70,
        DETERMINE_POWER_ATTACK_DELTAS,
        CONTROLLER_ATTACK_CHECK,
        CONTROLLER_TEST_8220,
        STATE_TIMESTAMP_COPY,
        TIMER_RUNNING,
        TIMER_START,
        TIMER_EXPIRED,
        TIMER_CLEAR,
        TIMER_ELAPSED,
        TIMER_DURATION,
        TIMER_CONSTRUCT,
        TIMER_SAVE,
        TIMER_LOAD,
        TIMESTAMP_SAVE,
        TIMESTAMP_LOAD,
        TIME_NOW,
        TIMESTAMP_SET,
        RANDOM_U32,
        RANDOM_RANGE,
        RANDOM_BETWEEN,
        FLOAT_MAX,
        FLOAT_PAIR_EBD0,
        FTOL,
        FILL_BYTES,
        MEMSET,
        PRINT_TO_BUFFER,
        COPY_TO_BUFFER,
        DEBUG_MESSAGES_ON,
        DEBUG_PRINT,
        DEBUG_WARNING,
        REFERENCE_NAME,
        FORM_ID,
        START_DIALOGUE,
        GET_ANIM_ACTION,
        GET_BLOCKED,
        SET_BLOCK,
        QUEUE_ATTACK,
        STOP_MOVING_ACTOR,
        IS_PATH_VALID,
        IS_PATHING,
        IS_WAITING_ON_PATH,
        IS_PATHING_COMPLETE,
        GET_CURRENT_PATHFINDING_GOAL,
        SET_PATHFINDING_GOAL,
        BUILD_REQUEST,
        FORCE_MOVE_MODE,
        GET_FATIGUE_PERCENTAGE,
        GET_WALK_SPEED,
        GET_ANIM_GROUP,
        GET_ANIM_GROUP_DURATION,
        GET_CURRENT_WEAPON,
        IS_WEAPON_DRAWN,
        GET_RADIUS,
        GET_SCALE,
        GET_WORLD_LOCATION,
        GET_WORLD_SPACE,
        GET_SAVED_ACQUIRE_OBJECT,
        ACTOR_PATH_TEST,
        ACTOR_CLEAR_8B3A50,
        ACTOR_TEST_8B3C70,
        ACTOR_MOVE_FLAGS,
        ACTOR_TEST_4938E0,
        ACTOR_IS_ATTACKING,
        ACTOR_BYTE_104,
        ACTOR_BYTE_14D,
        ACTOR_TEST_437BD0,
        ACTOR_TEST_437BF0,
        ACTOR_TEST_579670,
        ACTOR_WIDTH,
        ACTOR_LENGTH,
        ACTOR_CELL_8D6F30,
        PROCEDURE_TEST_525420,
        GET_CLAMPED_ACTOR_VALUE,
        ANIM_GROUP_OF,
        ANIM_STATE_OF,
        ANIM_SEQUENCE_OF,
        ANIM_DIRECTION,
        ANIM_ATTACK_CHOICE,
        ANIM_SPEED,
        ANIM_GROUP_LOADED,
        SEQUENCE_STATE,
        IS_ATTACK_ACTION,
        IS_POWER_ATTACK_ACTION,
        ANIM_GROUP_TYPE,
        ANIM_GROUP_TEST,
        WEAPON_ATTACK_GROUP,
        WEAPON_TYPE_TEST,
        GET_ITEM_HEALTH,
        RESTRICTED_MOVES,
        RESTRICTIONS_TEST_9373F0,
        COMBAT_STATE_TEST_F2F0,
        STYLE_SCORE_5524B0,
        STYLE_FLAG_TEST,
        STYLE_HAS_ATTACK,
        STYLE_GRENADE_SCALE,
        STYLE_CHANCE_73CB80,
        STYLE_CHANCE_505C00,
        STYLE_CHANCE_505C20,
        STYLE_CHANCE_6A9560,
        STYLE_CHANCE_9E8670,
        STYLE_DODGE_MIN_639B20,
        STYLE_DODGE_51B860,
        STYLE_DODGE_9A7C40,
        STYLE_DODGE_69EF80,
        STYLE_HOLD_6A78F0,
        STYLE_HOLD_9B0BD0,
        STYLE_BLOCK_TIME_A0,
        STYLE_BLOCK_TIME_A4,
        FORMULA_ATTACK_SCORE,
        FORMULA_BLOCK_SCORE,
        FORMULA_POWER_ATTACK,
        FORMULA_DODGE_CHANCE,
        FORMULA_DODGE_DIRECTION,
        GET_TARGET_FIRING_LOCATION,
        COMPUTE_FACE_TARGET_LOCATION,
        COMPUTE_FACE_TARGET_DISTANCE,
        GET_WEAPON_REACH,
        GET_DISTANCE_BETWEEN_FACING_ACTORS,
        GET_ATTACKER_TARGET_EXTENT_SUM,
        CALCULATE_LOS,
        IS_ACTOR_USING_RANGED,
        UTILITY_TEST_9A6AE0,
        ADD_UNREACHABLE_LOCATION,
        POINT_CONSTRUCT,
        POINT_SET,
        POINT_SUM,
        POINT_DIFFERENCE,
        POINT_LENGTH_SQUARED,
        POINT_LENGTH,
        POINT_LENGTH_457990,
        RESTRICTIONS_POINT,
        MATRIX_FROM_HEADING,
        MATRIX_TIMES_POINT,
        PATHING_REQUEST_CONSTRUCT,
        PATHING_REQUEST_DESTRUCT,
        PATHING_LOCATION_FROM_ACTOR,
        PATHING_LOCATION_FROM_POINT,
        PATHING_LOCATION_IS_VALID,
        PATHING_LOCATION_WORLDSPACE,
        PATHING_LOCATION_CELL,
        PATHING_LOCATION_POINT,
        PATHING_LOCATION_DESTRUCT,
        CHECK_LINE_OF_SIGHT,
        ALLOCATE,
        AVOID_ARRAY_CONSTRUCT,
        AVOID_POINTER_CONSTRUCT,
        AVOID_POINTER_DESTRUCT,
        POINTER_GET,
        AVOID_NODE_CONSTRUCT,
        AVOID_ARRAY_ADD,
        SAVE_BYTES,
        LOAD_BYTES,
    ];

    /// A procedure with a controller, an attacker and a target (all fake
    /// objects), the controller's style, the attacker's animation, the
    /// acquire object and a position point; the facing distance and the
    /// reach are cached (2 and 3).
    struct World {
        rig: Rig,
        this: Ptr<CombatProcedureAttackMelee>,
        t: u32,
        controller: u32,
        attacker: u32,
        target: u32,
        style: u32,
        animation: u32,
        position: u32,
    }

    fn world() -> World {
        let mut rig = Rig::new();
        let t = rig.e.mem.alloc(0x88);
        let this = Ptr::<CombatProcedureAttackMelee>::new(t);
        let controller = rig.object(0x200);
        rig.e.set(
            this,
            CombatProcedureAttackMelee::pCombatController,
            Ptr::new(controller),
        );
        let attacker = rig.actor();
        let target = rig.actor();
        let style = rig.e.mem.alloc(0x100);
        let animation = rig.object(0x40);
        let object = rig.object(0x40);
        let position = rig.e.mem.alloc(0xc);
        rig.answer(GET_ATTACKER, attacker);
        rig.answer(GET_TARGET, target);
        rig.answer(CONTROLLER_STYLE, style);
        rig.answer(GET_SAVED_ACQUIRE_OBJECT, object);
        // The state getters read the fields, as the exe's do.
        rig.e
            .register(WORD_AT_10, |e, a| ret(e.mem.u32(a[0] + 0x10)));
        rig.e
            .register(WORD_AT_14, |e, a| ret(e.mem.u32(a[0] + 0x14)));
        rig.slot(SLOT_ANIMATION, animation);
        rig.slot(SLOT_POSITION, position);
        rig.e.mem.set_f32(t + 0x5c, 2.0);
        rig.e.mem.set_f32(t + 0x58, 3.0);
        World {
            rig,
            this,
            t,
            controller,
            attacker,
            target,
            style,
            animation,
            position,
        }
    }

    #[test]
    fn update_watches_the_path_runs_the_maneuver_and_looks_at_the_target() {
        let mut w = world();
        let t = w.t;
        // Holding (maneuver 1) with the maneuver timer run out; the
        // restrictions' timer has not.
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eManeuverState, 1);
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eCombatState, 9);
        w.rig.queue(TIMER_EXPIRED, &[0, 1]);
        let (_, log) = w.rig.run(0x009c_c240, &args![t]);
        assert_eq!(w.rig.e.mem.u32(t + 0x5c), (-1.0f32).to_bits());
        assert_eq!(
            calls_to(&log, EMPTY_HOOK),
            vec![vec![COMBAT_HOOK_OBJECT], vec![t], vec![COMBAT_HOOK_OBJECT]]
        );
        assert_eq!(calls_to(&log, TIMER_CLEAR), vec![vec![t + 0x48]]);
        assert_eq!(
            calls_to(&log, TIMER_EXPIRED),
            vec![vec![t + 0x64 + 0x1c], vec![t + 0x28]]
        );
        // Combat state 0 (the field): looks at the target.
        let firing = calls_to(&log, GET_TARGET_FIRING_LOCATION);
        assert_eq!(firing.len(), 1);
        assert_eq!(firing[0][1..], [w.target, 2]);
        assert_eq!(
            calls_to(&log, SET_LOOK_AT_TARGET),
            vec![vec![t, firing[0][0], 4]]
        );

        // Moving in with a failed, invalid path and the unreachable timer
        // run out: the location is marked and the procedure fails.
        let mut w = world();
        let t = w.t;
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eManeuverState, 3);
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eCombatState, 9);
        w.rig.answer(ACTOR_PATH_TEST, 1);
        w.rig.answer(TIMER_RUNNING, 1);
        w.rig.queue(TIMER_EXPIRED, &[0, 1]);
        let combat_state = w.rig.e.mem.alloc(8);
        w.rig.answer(GET_COMBAT_STATE, combat_state);
        w.rig.answer(GET_WORLD_LOCATION, 0x4444);
        w.rig.answer(GET_SCALE, 0);
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eCombatState, 3);
        w.rig.e.mem.set_f32(t + 0x58, 1.0);
        w.rig.answer_float(GET_DISTANCE_BETWEEN_FACING_ACTORS, 5.0);
        w.rig.answer_float(GET_SCALE, 1.0);
        w.rig.setting(SETTING_COMBAT_DISTANCE, 1.0);
        let (_, log) = w.rig.run(0x009c_c240, &args![t]);
        assert_eq!(w.rig.e.mem.u32(combat_state + 4), 2);
        assert_eq!(
            calls_to(&log, ADD_UNREACHABLE_LOCATION),
            vec![vec![combat_state, 0x4444]]
        );
        assert_eq!(
            calls_to(&log, FAIL_PROCEDURE),
            vec![vec![t, TEXT_COULD_NOT_REACH_TARGET]]
        );
        assert!(calls_to(&log, TIMER_CLEAR).is_empty());
        // The facing distance (5) minus the reach (1) is beyond 2.25 times
        // the combat distance: no looking.
        assert_eq!(calls_to(&log, CLEAR_LOOK_AT_TARGET), vec![vec![t]]);
        assert!(calls_to(&log, SET_LOOK_AT_TARGET).is_empty());
    }

    #[test]
    fn fn_009cc490_ors_the_mask_into_the_word_at_4() {
        let mut w = world();
        let block = w.rig.e.mem.alloc(8);
        w.rig.e.mem.set_u32(block + 4, 0x10);
        w.rig.run(0x009c_c490, &args![block, 3u32]);
        assert_eq!(w.rig.e.mem.u32(block + 4), 0x13);
    }

    #[test]
    fn shutdown_stops_the_attacker() {
        let mut w = world();
        let (_, log) = w.rig.run(0x009c_c4d0, &args![w.t]);
        assert_eq!(calls_to(&log, CLEAR_LOOK_AT_TARGET), vec![vec![w.t]]);
        assert_eq!(calls_to(&log, STOP_MOVING_ACTOR), vec![vec![w.attacker]]);
        assert_eq!(calls_to(&log, SET_BLOCK), vec![vec![w.attacker, 0]]);
    }

    /// The attack decision with the scores `attack`, `block`, the hold
    /// score from `FLOAT_MAX` and the draw `random`.
    fn decision(attack: f64, block: f64, hold: f64, random: u32) -> (World, Log) {
        let mut w = world();
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eCombatState, 3);
        w.rig.answer_float(FORMULA_ATTACK_SCORE, attack);
        w.rig.answer_float(FORMULA_BLOCK_SCORE, block);
        w.rig.answer_float(FLOAT_MAX, hold);
        w.rig.answer(IS_WEAPON_DRAWN, 1);
        w.rig.answer(CONTROLLER_TEST_F2D0, 1);
        w.rig.answer(ANIM_GROUP_LOADED, 1);
        w.rig.slot(SLOT_TEST_3F8, 1);
        w.rig.answer(ACTOR_BYTE_104, 1);
        w.rig.answer(FTOL, (attack + block + hold) as u32);
        w.rig.answer(RANDOM_U32, random);
        let (_, log) = w.rig.run(0x009c_c510, &args![w.t]);
        (w, log)
    }

    #[test]
    fn fn_009cc510_attacks_blocks_or_waits_by_the_draw() {
        // A draw below the attack score: the attack (the weapon's group)
        // is picked and started.
        let (w, log) = {
            let mut w = world();
            w.rig
                .e
                .set(w.this, CombatProcedureAttackMelee::eCombatState, 3);
            w.rig.answer_float(FORMULA_ATTACK_SCORE, 50.0);
            w.rig.answer(CONTROLLER_TEST_F2D0, 1);
            w.rig.slot(SLOT_TEST_3F8, 1);
            w.rig.answer(ACTOR_BYTE_104, 1);
            w.rig.answer(FTOL, 50);
            w.rig.answer(RANDOM_U32, 10);
            w.rig.answer(CONTROLLER_WEAPON, 0x900);
            w.rig.answer(WEAPON_ATTACK_GROUP, 0x21);
            w.rig.slot(SLOT_TARGET_TEST_1D0, 1);
            w.rig.answer(QUEUE_ATTACK, 1);
            let manager = 0x6100;
            w.rig.e.mem.set_u32(DIALOGUE_MANAGER, manager);
            w.rig
                .e
                .set(w.this, CombatProcedureAttackMelee::eCombatState, 3);
            let (_, log) = w.rig.run(0x009c_c510, &args![w.t]);
            (w, log)
        };
        let scores = calls_to(&log, FORMULA_ATTACK_SCORE);
        assert_eq!(scores.len(), 1);
        assert_eq!(scores[0][0], w.style);
        assert_eq!(
            calls_to(&log, FORMULA_BLOCK_SCORE),
            vec![vec![w.style, 0x32, 0x32, 0, 0]]
        );
        assert_eq!(
            calls_to(&log, START_DIALOGUE),
            vec![vec![0x6100, w.controller, 2, 0, 0]]
        );
        assert_eq!(calls_to(&log, QUEUE_ATTACK), vec![vec![w.attacker, 0x21]]);
        assert_eq!(
            w.rig
                .e
                .get(w.this, CombatProcedureAttackMelee::eCurrentAttack),
            0x21
        );
        assert_eq!(
            w.rig
                .e
                .get(w.this, CombatProcedureAttackMelee::eCombatState),
            0
        );
        // The weapon is not drawn: no block score.
        assert_eq!(calls_to(&log, FTOL), vec![args![50.0f64]]);

        // A draw between the attack and attack plus block: blocks.
        let (w, log) = decision(10.0, 30.0, 0.0, 20);
        assert_eq!(calls_to(&log, SET_BLOCK), vec![vec![w.attacker, 1]]);
        assert_eq!(
            w.rig
                .e
                .get(w.this, CombatProcedureAttackMelee::eCombatState),
            1
        );
        assert_eq!(
            calls_to(&log, FLOAT_MAX),
            vec![vec![bits(60.0), 0]],
            "the hold score is 100 minus the others, at least fCombatMaxHoldScore"
        );

        // A draw above both: waits a random time.
        let (w, log) = decision(10.0, 30.0, 60.0, 90);
        assert_eq!(
            w.rig
                .e
                .get(w.this, CombatProcedureAttackMelee::eCombatState),
            2
        );
        let starts = calls_to(&log, TIMER_START);
        assert_eq!(starts.len(), 1);
        assert_eq!(starts[0][0], w.t + 0x20);
        assert_eq!(
            calls_to(&log, STYLE_BLOCK_TIME_A4),
            vec![vec![w.style]],
            "the wait is drawn between the style's two times"
        );

        // Not deciding: nothing.
        let mut w = world();
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eCombatState, 1);
        let (_, log) = w.rig.run(0x009c_c510, &args![w.t]);
        assert!(calls_to(&log, IS_SWITCHING_WEAPONS).is_empty());
    }

    /// `fn_009ccb20` with the given flags; returns the attack, the shout
    /// and the power bytes and the log.
    fn pick(w: &mut World, attacking: bool) -> (u32, u8, u8, Log) {
        let flags = w.rig.e.mem.alloc(8);
        let (result, log) = w.rig.run(
            0x009c_cb20,
            &args![
                w.t,
                attacking,
                bits(2.0),
                bits(3.0),
                true,
                false,
                true,
                flags,
                flags + 4
            ],
        );
        (
            result.u32(),
            w.rig.e.mem.u8(flags),
            w.rig.e.mem.u8(flags + 4),
            log,
        )
    }

    #[test]
    fn fn_009ccb20_picks_the_weapon_group_or_the_direction_power_attack() {
        // Not attacking, no shout, in reach: the weapon's attack group.
        let mut w = world();
        w.rig.answer(CONTROLLER_WEAPON, 0x900);
        w.rig.answer(WEAPON_ATTACK_GROUP, 0x21);
        let (attack, shout, power, log) = pick(&mut w, false);
        assert_eq!((attack, shout, power), (0x21, 0, 0));
        let formula = calls_to(&log, FORMULA_POWER_ATTACK);
        assert_eq!(formula.len(), 1);
        assert_eq!(formula[0][0], w.style);
        assert_eq!(formula[0][2..], [1, 0, 0]);

        // No weapon group: the animation's choice (0x6c gives 0x1a).
        let mut w = world();
        w.rig.answer(ANIM_ATTACK_CHOICE, 0x6c);
        w.rig.e.mem.set_u32(ATTACK_GROUP_WORD, 7);
        let (attack, _, _, log) = pick(&mut w, false);
        assert_eq!(attack, 0x1a);
        assert_eq!(
            calls_to(&log, ANIM_ATTACK_CHOICE),
            vec![vec![w.animation, 1, 7, 0]]
        );

        // Attacking with the swing done: the direction 0x35 gives 0x32 and
        // marks a power attack.
        let mut w = world();
        w.rig.answer(ANIM_STATE_OF, 3);
        w.rig.answer(ANIM_DIRECTION, 0x35);
        let (attack, _, power, _) = pick(&mut w, true);
        assert_eq!((attack, power), (0x32, 1));

        // An unknown direction falls back to 0x20.
        let mut w = world();
        w.rig.answer(ANIM_STATE_OF, 3);
        w.rig.answer(ANIM_DIRECTION, 0x10);
        let (attack, _, power, _) = pick(&mut w, true);
        assert_eq!((attack, power), (0x20, 1));

        // Attacking mid-swing: nothing. Without a target: nothing.
        let mut w = world();
        let (attack, _, _, _) = pick(&mut w, true);
        assert_eq!(attack, 0xff);
        let mut w = world();
        w.rig.answer(GET_TARGET, 0);
        let (attack, _, _, log) = pick(&mut w, false);
        assert_eq!(attack, 0xff);
        assert!(calls_to(&log, FORMULA_POWER_ATTACK).is_empty());

        // Backing away the reach grows by the step distance.
        let mut w = world();
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eManeuverState, 4);
        w.rig.setting(SETTING_COMBAT_STEP_ADVANCE_DISTANCE, 1.5);
        w.rig.answer(RANDOM_U32, 50);
        w.rig.e.mem.set_f32(w.t + 0x5c, 4.0);
        w.rig.answer(CONTROLLER_WEAPON, 0x900);
        w.rig.answer(WEAPON_ATTACK_GROUP, 0x22);
        // A facing distance of 4 is beyond 3 but within 4.5.
        let flags = w.rig.e.mem.alloc(8);
        let (result, _) = w.rig.run(
            0x009c_cb20,
            &args![
                w.t,
                false,
                bits(4.0),
                bits(3.0),
                false,
                false,
                false,
                flags,
                flags + 4
            ],
        );
        assert_eq!(result.u32(), 0x22);
    }

    #[test]
    fn start_attack_queues_the_attack() {
        let mut w = world();
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eCombatState, 3);
        w.rig.answer(QUEUE_ATTACK, 1);
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eCombatState, 3);
        let (_, log) = w.rig.run(0x009c_ce70, &args![w.t, 0x26u32, 9u32]);
        assert_eq!(calls_to(&log, QUEUE_ATTACK), vec![vec![w.attacker, 0x26]]);
        assert_eq!(
            w.rig
                .e
                .get(w.this, CombatProcedureAttackMelee::eCombatState),
            0
        );
        assert_eq!(
            w.rig
                .e
                .get(w.this, CombatProcedureAttackMelee::eLastCombatState),
            3
        );
        assert_eq!(
            w.rig
                .e
                .get(w.this, CombatProcedureAttackMelee::eCurrentAttack),
            0x26
        );
        let (_, log) = w.rig.run(0x009c_ce70, &args![w.t, 0xffu32, 0u32]);
        assert!(calls_to(&log, QUEUE_ATTACK).is_empty());
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eCombatState, 0);
        let (_, log) = w.rig.run(0x009c_ce70, &args![w.t, 0x26u32, 0u32]);
        assert!(calls_to(&log, QUEUE_ATTACK).is_empty());
    }

    #[test]
    fn handle_attack_follows_the_swing() {
        let mut w = world();
        // The attack decision stops at once (switching weapons).
        w.rig.answer(IS_SWITCHING_WEAPONS, 1);
        w.rig.answer(IS_ATTACK_ACTION, 1);
        w.rig.answer(ANIM_STATE_OF, 3);
        w.rig.answer(CONTROLLER_WEAPON, 0x900);
        w.rig.answer(WEAPON_ATTACK_GROUP, 0x21);
        w.rig.answer(ANIM_GROUP_TEST, 1);
        let (_, log) = w.rig.run(0x009c_ced0, &args![w.t]);
        assert_eq!(calls_to(&log, ANIM_GROUP_OF)[0], vec![w.animation, 4]);
        assert_eq!(
            w.rig
                .e
                .get(w.this, CombatProcedureAttackMelee::eCombatState),
            3
        );
        assert_eq!(calls_to(&log, ANIM_GROUP_TEST), vec![vec![0x21]]);
        assert_eq!(calls_to(&log, QUEUE_ATTACK), vec![vec![w.attacker, 0x21]]);

        // No attack action any more: back to deciding, nothing queued.
        let mut w = world();
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eCombatState, 0);
        let (_, log) = w.rig.run(0x009c_ced0, &args![w.t]);
        assert_eq!(
            w.rig
                .e
                .get(w.this, CombatProcedureAttackMelee::eCombatState),
            3
        );
        assert!(calls_to(&log, ANIM_STATE_OF).is_empty());

        // Not attacking: nothing.
        let mut w = world();
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eCombatState, 2);
        let (_, log) = w.rig.run(0x009c_ced0, &args![w.t]);
        assert!(calls_to(&log, GET_ATTACKER).is_empty());
    }

    #[test]
    fn fn_009ccfb0_ends_a_block_on_a_broken_item_or_a_recoiling_target() {
        let mut w = world();
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eCombatState, 1);
        w.rig.answer(GET_BLOCKED, 1);
        w.rig.slot(SLOT_ITEM_148, 0x800);
        w.rig.answer_float(GET_ITEM_HEALTH, 0.0);
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eCombatState, 1);
        let (_, log) = w.rig.run(0x009c_cfb0, &args![w.t]);
        assert_eq!(calls_to(&log, GET_ITEM_HEALTH), vec![vec![0x800, 0]]);
        assert_eq!(calls_to(&log, SET_BLOCK), vec![vec![w.attacker, 0]]);
        assert_eq!(
            w.rig
                .e
                .get(w.this, CombatProcedureAttackMelee::eCombatState),
            3
        );

        // A sound item, a target in anim action 8: ends and decides again.
        let mut w = world();
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eCombatState, 1);
        w.rig.answer(GET_BLOCKED, 1);
        w.rig.slot(SLOT_ITEM_148, 0x800);
        w.rig.answer_float(GET_ITEM_HEALTH, 50.0);
        w.rig.answer(GET_ANIM_ACTION, 8);
        // Keeps 009cf570 from running the attack decision.
        w.rig.answer(CONTROLLER_TEST_18B0, 1);
        let (_, log) = w.rig.run(0x009c_cfb0, &args![w.t]);
        assert_eq!(calls_to(&log, SET_BLOCK), vec![vec![w.attacker, 0]]);
        assert_eq!(
            w.rig
                .e
                .get(w.this, CombatProcedureAttackMelee::eCombatState),
            3
        );

        // The attacker's sequence busy: keeps the block.
        let mut w = world();
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eCombatState, 1);
        w.rig.answer(GET_BLOCKED, 1);
        w.rig.slot(SLOT_ITEM_148, 0x800);
        w.rig.answer(ANIM_SEQUENCE_OF, 0x700);
        w.rig.answer(SEQUENCE_STATE, 2);
        let (_, log) = w.rig.run(0x009c_cfb0, &args![w.t]);
        assert!(calls_to(&log, SET_BLOCK).is_empty());
        assert_eq!(calls_to(&log, SEQUENCE_STATE), vec![vec![0x700]]);

        // No item: nothing more after the blocked check.
        let mut w = world();
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eCombatState, 1);
        let (_, log) = w.rig.run(0x009c_cfb0, &args![w.t]);
        assert!(calls_to(&log, GET_ITEM_HEALTH).is_empty());
        assert_eq!(
            w.rig
                .e
                .get(w.this, CombatProcedureAttackMelee::eCombatState),
            3
        );
    }

    #[test]
    fn fn_009cd150_keeps_or_ends_the_block() {
        // The target beyond reach (facing 2 > reach 1): ends.
        let mut w = world();
        w.rig.answer(CONTROLLER_TEST_1900, 1);
        w.rig.answer(GET_BLOCKED, 1);
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eCombatState, 1);
        w.rig.e.mem.set_f32(w.t + 0x58, 1.0);
        let (_, log) = w.rig.run(0x009c_d150, &args![w.t]);
        assert_eq!(calls_to(&log, SET_BLOCK), vec![vec![w.attacker, 0]]);
        assert_eq!(
            calls_to(&log, CONTROLLER_SET_18E0),
            vec![vec![w.controller, 0]]
        );
        assert_eq!(
            w.rig
                .e
                .get(w.this, CombatProcedureAttackMelee::eCombatState),
            3
        );
        assert!(calls_to(&log, RANDOM_U32).is_empty());

        // Within reach, an attacking target, a low draw: keeps blocking
        // while blocked.
        let mut w = world();
        w.rig.answer(CONTROLLER_TEST_1900, 1);
        w.rig.answer(GET_BLOCKED, 1);
        w.rig.answer(ACTOR_IS_ATTACKING, 1);
        w.rig.answer(RANDOM_U32, 3);
        let (_, log) = w.rig.run(0x009c_d150, &args![w.t]);
        assert!(calls_to(&log, SET_BLOCK).is_empty());
        assert!(calls_to(&log, CONTROLLER_SET_18E0).is_empty());

        // The same with a target that does not attack: ends.
        let mut w = world();
        w.rig.answer(CONTROLLER_TEST_1900, 1);
        w.rig.answer(GET_BLOCKED, 1);
        w.rig.answer(RANDOM_U32, 3);
        let (_, log) = w.rig.run(0x009c_d150, &args![w.t]);
        assert_eq!(calls_to(&log, SET_BLOCK), vec![vec![w.attacker, 0]]);

        // No target: ends at once.
        let mut w = world();
        w.rig.answer(CONTROLLER_TEST_1900, 1);
        w.rig.answer(GET_TARGET, 0);
        let (_, log) = w.rig.run(0x009c_d150, &args![w.t]);
        assert_eq!(
            calls_to(&log, CONTROLLER_SET_18E0),
            vec![vec![w.controller, 0]]
        );
        assert!(calls_to(&log, GET_ANIM_ACTION).is_empty());

        // Not blocking and the test fails: nothing.
        let mut w = world();
        let (_, log) = w.rig.run(0x009c_d150, &args![w.t]);
        assert!(calls_to(&log, CONTROLLER_SET_18E0).is_empty());
    }

    #[test]
    fn fn_009cd3d0_and_fn_009cd410_step_the_states() {
        let mut w = world();
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eCombatState, 2);
        w.rig.answer(TIMER_EXPIRED, 1);
        let (_, log) = w.rig.run(0x009c_d3d0, &args![w.t]);
        assert_eq!(calls_to(&log, TIMER_EXPIRED), vec![vec![w.t + 0x20]]);
        assert_eq!(
            w.rig
                .e
                .get(w.this, CombatProcedureAttackMelee::eCombatState),
            3
        );
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eCombatState, 1);
        let (_, log) = w.rig.run(0x009c_d3d0, &args![w.t]);
        assert!(calls_to(&log, TIMER_EXPIRED).is_empty());

        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eManeuverState, 3);
        w.rig.answer(IS_PATHING, 1);
        w.rig.run(0x009c_d410, &args![w.t]);
        assert_eq!(
            w.rig
                .e
                .get(w.this, CombatProcedureAttackMelee::eManeuverState),
            0
        );
    }

    #[test]
    fn fn_009cd430_backs_away_moves_in_or_decides() {
        // In reach and 009a6ae0 agrees: backs away (maneuver 4).
        let mut w = world();
        w.rig.answer(UTILITY_TEST_9A6AE0, 1);
        w.rig.answer(IS_PATHING, 1);
        let (_, log) = w.rig.run(0x009c_d430, &args![w.t]);
        assert_eq!(
            calls_to(&log, UTILITY_TEST_9A6AE0),
            vec![vec![w.attacker, w.target, 0]]
        );
        assert_eq!(
            w.rig
                .e
                .get(w.this, CombatProcedureAttackMelee::eManeuverState),
            4
        );
        assert_eq!(calls_to(&log, SET_MOVE_MODE), vec![vec![w.t, 3]]);

        // Outside the range by more than the minimum engage distance:
        // backs away too.
        let mut w = world();
        w.rig.e.mem.set_f32(w.t + 0x5c, 10.0);
        w.rig.setting(SETTING_COMBAT_MIN_ENGAGE_DISTANCE, 5.0);
        w.rig.answer(IS_PATHING, 1);
        w.rig.run(0x009c_d430, &args![w.t]);
        assert_eq!(
            w.rig
                .e
                .get(w.this, CombatProcedureAttackMelee::eManeuverState),
            4
        );

        // Outside by less: moves in (maneuver 3).
        let mut w = world();
        w.rig.e.mem.set_f32(w.t + 0x5c, 4.0);
        w.rig.setting(SETTING_COMBAT_MIN_ENGAGE_DISTANCE, 5.0);
        w.rig.answer(IS_PATHING, 1);
        let (_, log) = w.rig.run(0x009c_d430, &args![w.t]);
        assert_eq!(
            w.rig
                .e
                .get(w.this, CombatProcedureAttackMelee::eManeuverState),
            3
        );
        assert_eq!(calls_to(&log, SET_MOVE_MODE), vec![vec![w.t, 1]]);

        // Within reach otherwise: the maneuver decision (holding here).
        let mut w = world();
        w.rig.answer(RANDOM_U32, 99);
        w.rig.answer(IS_PATHING, 1);
        w.rig.e.mem.set_u32(PLAYER, w.target);
        w.rig.run(0x009c_d430, &args![w.t]);
        assert_eq!(
            w.rig
                .e
                .get(w.this, CombatProcedureAttackMelee::eManeuverState),
            1
        );

        // Not in maneuver 0: nothing.
        let mut w = world();
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eManeuverState, 2);
        let (_, log) = w.rig.run(0x009c_d430, &args![w.t]);
        assert!(calls_to(&log, GET_ATTACKER).is_empty());
    }

    #[test]
    fn maneuver_decision_dodges_or_holds() {
        // A dodge chance of 50 against a draw of 30: dodges left with the
        // left animation found.
        let mut w = world();
        w.rig.e.mem.set_u32(PLAYER, w.target);
        w.rig.answer_float(FORMULA_DODGE_CHANCE, 50.0);
        w.rig.answer(RANDOM_U32, 30);
        w.rig.answer(FORMULA_DODGE_DIRECTION, 0x14);
        w.rig.answer(GET_ANIM_GROUP, 0x55);
        w.rig.answer(ANIM_GROUP_TYPE, 5);
        w.rig.answer_float(GET_ANIM_GROUP_DURATION, 0.75);
        w.rig.answer_float(RANDOM_BETWEEN, 1.5);
        w.rig.answer_float(FLOAT_MAX, 1.5);
        w.rig.answer(IS_PATHING, 1);
        w.rig.e.mem.set_f32(w.style + 0x60, 2.5);
        let (_, log) = w.rig.run(0x009c_d5e0, &args![w.t]);
        assert_eq!(
            calls_to(&log, GET_ANIM_GROUP),
            vec![vec![w.attacker, 5, 0, 0, 0]]
        );
        let between = calls_to(&log, RANDOM_BETWEEN);
        assert_eq!(between.len(), 1);
        assert_eq!(between[0][1], bits(2.5), "the style's float at +0x60");
        assert_eq!(calls_to(&log, FLOAT_MAX), vec![vec![bits(0.75), bits(1.5)]]);
        assert_eq!(
            calls_to(&log, FORCE_MOVE_MODE),
            vec![vec![w.attacker, 0x114]]
        );
        assert_eq!(
            calls_to(&log, TIMER_START),
            vec![vec![w.t + 0x28, bits(1.5)], vec![w.t + 0x30, bits(0.75)]]
        );
        assert_eq!(
            w.rig
                .e
                .get(w.this, CombatProcedureAttackMelee::eManeuverState),
            2
        );

        // No animation for the direction: reported when debug messages are on.
        let mut w = world();
        w.rig.e.mem.set_u32(PLAYER, w.target);
        w.rig.answer_float(FORMULA_DODGE_CHANCE, 50.0);
        w.rig.answer(FORMULA_DODGE_DIRECTION, 0x8);
        w.rig.answer(DEBUG_MESSAGES_ON, 1);
        w.rig.answer(REFERENCE_NAME, 0x5000);
        let (_, log) = w.rig.run(0x009c_d5e0, &args![w.t]);
        assert_eq!(
            calls_to(&log, DEBUG_PRINT),
            vec![vec![TEXT_NO_DODGE_ANIMATION, 0x5000, TEXT_RIGHT]]
        );
        assert_eq!(
            w.rig
                .e
                .get(w.this, CombatProcedureAttackMelee::eManeuverState),
            0
        );

        // A non-player target lowers the chance by 100: holds.
        let mut w = world();
        w.rig.answer_float(FORMULA_DODGE_CHANCE, 50.0);
        w.rig.answer(RANDOM_U32, 30);
        w.rig.answer(IS_PATHING, 1);
        w.rig.answer_float(RANDOM_BETWEEN, 4.0);
        let (_, log) = w.rig.run(0x009c_d5e0, &args![w.t]);
        assert!(calls_to(&log, FORMULA_DODGE_DIRECTION).is_empty());
        assert_eq!(
            calls_to(&log, TIMER_START),
            vec![vec![w.t + 0x28, bits(4.0)]]
        );
        assert_eq!(calls_to(&log, STOP_MOVING_ACTOR), vec![vec![w.attacker]]);
        assert_eq!(
            w.rig
                .e
                .get(w.this, CombatProcedureAttackMelee::eManeuverState),
            1
        );
    }

    #[test]
    fn handle_dodge_ends_or_turns_the_dodge() {
        // The maneuver timer ran out: back to 0.
        let mut w = world();
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eManeuverState, 2);
        w.rig.answer(TIMER_EXPIRED, 1);
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eManeuverState, 2);
        let (_, log) = w.rig.run(0x009c_db00, &args![w.t]);
        assert_eq!(
            w.rig
                .e
                .get(w.this, CombatProcedureAttackMelee::eManeuverState),
            0
        );
        assert!(calls_to(&log, ACTOR_MOVE_FLAGS).is_empty());

        // A restricted direction: out of room.
        let mut w = world();
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eManeuverState, 2);
        w.rig.answer(ACTOR_TEST_4938E0, 1);
        w.rig.answer(ACTOR_MOVE_FLAGS, 0x104);
        w.rig.answer(DEBUG_MESSAGES_ON, 1);
        w.rig.answer(REFERENCE_NAME, 0x5000);
        w.rig.answer(PROCEDURE_TEST_525420, 1);
        w.rig.e.set(
            Ptr::<CombatMovementRestrictions>::new(w.t + 0x64),
            CombatMovementRestrictions::sRestrictedMoves,
            4,
        );
        let (_, log) = w.rig.run(0x009c_db00, &args![w.t]);
        assert_eq!(
            calls_to(&log, DEBUG_PRINT),
            vec![vec![TEXT_NO_ROOM_TO_DODGE, 0x5000]]
        );
        assert!(calls_to(&log, START_DIALOGUE).is_empty());

        // A sideways dodge inside the reach adds the forward bit... beyond
        // the reach outside the range it keeps going; an idle target turns
        // a walk into a run.
        let mut w = world();
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eManeuverState, 2);
        w.rig.answer(ACTOR_TEST_4938E0, 1);
        w.rig.answer(ACTOR_MOVE_FLAGS, 0x104);
        w.rig.answer(PROCEDURE_TEST_525420, 1);
        w.rig.answer(GET_ANIM_ACTION, 2);
        w.rig.answer(RANDOM_U32, 50);
        w.rig.setting(SETTING_AI_DODGE_WALK_CHANCE, 0.25);
        w.rig.e.mem.set_f32(w.t + 0x5c, 2.5);
        w.rig.e.mem.set_f32(w.t + 0x58, 2.0);
        let manager = 0x6100;
        w.rig.e.mem.set_u32(DIALOGUE_MANAGER, manager);
        let (_, log) = w.rig.run(0x009c_db00, &args![w.t]);
        assert_eq!(
            calls_to(&log, FORCE_MOVE_MODE),
            vec![vec![w.attacker, 0x204]]
        );
        assert_eq!(
            calls_to(&log, START_DIALOGUE),
            vec![vec![manager, w.controller, 2, 0, 0]]
        );

        // Sideways within the attack range: the forward bit is added.
        let mut w = world();
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eManeuverState, 2);
        w.rig.answer(ACTOR_TEST_4938E0, 1);
        w.rig.answer(ACTOR_MOVE_FLAGS, 0x8);
        w.rig.answer(PROCEDURE_TEST_525420, 1);
        w.rig.e.mem.set_f32(w.t + 0x5c, 2.5);
        w.rig.e.mem.set_f32(w.t + 0x58, 2.4);
        // 2.5 is outside 2.4: the dodge timer has not run out, so it goes on.
        let (_, log) = w.rig.run(0x009c_db00, &args![w.t]);
        assert!(calls_to(&log, FORCE_MOVE_MODE).is_empty());
        assert_eq!(calls_to(&log, TIMER_EXPIRED).len(), 2);
    }

    #[test]
    fn fn_009cdde0_tests_the_restricted_moves() {
        let mut w = world();
        let r = w.t + 0x64;
        w.rig.e.mem.set_u16(r + 8, 0x6);
        assert!(!w.rig.run(0x009c_dde0, &args![r, 2u32]).0.bool());
        assert!(w.rig.run(0x009c_dde0, &args![r, 9u32]).0.bool());
        assert_eq!(w.rig.run(0x009c_dde0, &args![r, 9u32]).0.eax, 1);
    }

    #[test]
    fn fn_009cde10_holds_until_the_timer_or_the_range() {
        let mut w = world();
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eManeuverState, 1);
        w.rig.answer(TIMER_EXPIRED, 1);
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eManeuverState, 1);
        w.rig.run(0x009c_de10, &args![w.t]);
        assert_eq!(
            w.rig
                .e
                .get(w.this, CombatProcedureAttackMelee::eManeuverState),
            0
        );

        // Half a second in, outside the range: back to 0 too.
        let mut w = world();
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eManeuverState, 1);
        w.rig.answer_float(TIMER_ELAPSED, 0.75);
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eManeuverState, 1);
        w.rig.e.mem.set_f32(w.t + 0x5c, 4.0);
        let (_, log) = w.rig.run(0x009c_de10, &args![w.t]);
        assert_eq!(calls_to(&log, TIMER_ELAPSED), vec![vec![w.t + 0x28]]);
        assert_eq!(
            w.rig
                .e
                .get(w.this, CombatProcedureAttackMelee::eManeuverState),
            0
        );

        // A ranged ally wants to shoot: waiting turns into deciding and the
        // maneuver is decided again.
        let mut w = world();
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eManeuverState, 1);
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eCombatState, 2);
        w.rig.e.set(
            w.this,
            CombatProcedureAttackMelee::bRangedAllyWantsToShoot,
            true,
        );
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eCombatState, 2);
        let (_, log) = w.rig.run(0x009c_de10, &args![w.t]);
        assert_eq!(
            w.rig
                .e
                .get(w.this, CombatProcedureAttackMelee::eCombatState),
            3
        );
        assert_eq!(calls_to(&log, FORMULA_DODGE_CHANCE).len(), 1);
    }

    #[test]
    fn fn_009cdf00_starts_moving_in() {
        let mut w = world();
        w.rig.answer(IS_PATHING, 1);
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eManeuverState, 9);
        // Outside the range (4 against 3) with the controller's test: the
        // first step re-paths and stays moving in.
        w.rig.e.mem.set_f32(w.t + 0x5c, 4.0);
        w.rig.setting(SETTING_COMBAT_MIN_ENGAGE_DISTANCE, 5.0);
        w.rig.answer(CONTROLLER_TEST_F2D0, 1);
        let (_, log) = w.rig.run(0x009c_df00, &args![w.t]);
        assert_eq!(
            w.rig
                .e
                .get(w.this, CombatProcedureAttackMelee::eManeuverState),
            3
        );
        assert_eq!(calls_to(&log, SET_MOVE_MODE), vec![vec![w.t, 1]]);
        let face = calls_to(&log, COMPUTE_FACE_TARGET_LOCATION);
        assert_eq!(face.len(), 2);
        assert_eq!(face[0][1..], [w.attacker, w.target, w.position, bits(3.0)]);
        let from_point = calls_to(&log, PATHING_LOCATION_FROM_POINT);
        assert_eq!(from_point[0][1..], [face[0][0], w.attacker]);
        assert_eq!(calls_to(&log, SET_PATHFINDING_GOAL).len(), 2);
    }

    #[test]
    fn fn_009cdf80_moves_in_until_in_reach() {
        // Pathing, far beyond twice the minimum engage distance, attacking:
        // backs away.
        let mut w = world();
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eManeuverState, 3);
        w.rig.answer(IS_PATHING, 1);
        w.rig.setting(SETTING_COMBAT_MIN_ENGAGE_DISTANCE, 0.5);
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eManeuverState, 3);
        let (_, log) = w.rig.run(0x009c_df80, &args![w.t]);
        assert_eq!(
            w.rig
                .e
                .get(w.this, CombatProcedureAttackMelee::eManeuverState),
            4
        );
        assert_eq!(calls_to(&log, COMPUTE_FACE_TARGET_LOCATION).len(), 2);

        // Outside the range: re-paths unless the goal is within reach.
        let mut w = world();
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eManeuverState, 3);
        w.rig.answer(IS_PATHING, 1);
        w.rig.answer(CONTROLLER_TEST_F2D0, 1);
        w.rig.setting(SETTING_COMBAT_MIN_ENGAGE_DISTANCE, 5.0);
        w.rig.e.mem.set_f32(w.t + 0x5c, 4.0);
        let (_, log) = w.rig.run(0x009c_df80, &args![w.t]);
        assert_eq!(calls_to(&log, GET_CURRENT_PATHFINDING_GOAL).len(), 1);
        assert_eq!(calls_to(&log, SET_PATHFINDING_GOAL).len(), 1);

        // Within reach and free to back away inside the minimum distance:
        // goes on with the combat state.
        let mut w = world();
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eManeuverState, 3);
        w.rig.answer(IS_PATHING, 1);
        w.rig.answer(ACTOR_MOVE_FLAGS, 2);
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eCombatState, 2);
        w.rig.setting(SETTING_COMBAT_MIN_ENGAGE_DISTANCE, 5.0);
        let (_, log) = w.rig.run(0x009c_df80, &args![w.t]);
        assert_eq!(calls_to(&log, TIMER_EXPIRED), vec![vec![w.t + 0x20]]);

        // Not pathing: back to 0.
        let mut w = world();
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eManeuverState, 3);
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eManeuverState, 3);
        w.rig.answer(IS_PATHING, 0);
        let (_, log) = w.rig.run(0x009c_df80, &args![w.t]);
        assert!(calls_to(&log, COMPUTE_FACE_TARGET_LOCATION).is_empty());
    }

    #[test]
    fn fn_009ce190_starts_backing_away() {
        let mut w = world();
        w.rig.answer(IS_PATHING, 1);
        let (_, log) = w.rig.run(0x009c_e190, &args![w.t]);
        assert_eq!(
            w.rig
                .e
                .get(w.this, CombatProcedureAttackMelee::eManeuverState),
            4
        );
        assert_eq!(calls_to(&log, EMPTY_HOOK), vec![vec![w.controller]]);
        assert_eq!(calls_to(&log, SET_MOVE_MODE), vec![vec![w.t, 3]]);
        assert_eq!(
            calls_to(&log, TIMER_START),
            vec![vec![w.t + 0x28, bits(3.0)]]
        );
    }

    #[test]
    fn fn_009ce1d0_backs_away_and_may_attack() {
        // In reach with twice the reach clear and the draw below the
        // chance: a normal attack.
        let mut w = world();
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eManeuverState, 4);
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eCombatState, 3);
        w.rig.queue(CONTROLLER_TEST_F2D0, &[1, 1]);
        w.rig.e.mem.set_f32(w.t + 0x5c, 5.0);
        w.rig.slot(SLOT_TEST_218, 1);
        w.rig.slot(SLOT_TEST_3F8, 1);
        w.rig.answer(UTILITY_TEST_9A6AE0, 1);
        w.rig.answer(RANDOM_U32, 10);
        w.rig
            .setting(SETTING_COMBAT_ADVANCE_NORMAL_ATTACK_CHANCE, 0.5);
        w.rig.answer(FTOL, 50);
        w.rig.answer(ANIM_ATTACK_CHOICE, 1);
        w.rig.answer(QUEUE_ATTACK, 1);
        let (_, log) = w.rig.run(0x009c_e1d0, &args![w.t]);
        assert_eq!(calls_to(&log, FTOL), vec![args![50.0f64]]);
        assert_eq!(calls_to(&log, QUEUE_ATTACK), vec![vec![w.attacker, 0x20]]);

        // The path done: back to 0.
        let mut w = world();
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eManeuverState, 4);
        w.rig.answer(CONTROLLER_TEST_F2D0, 1);
        w.rig.answer(IS_PATHING_COMPLETE, 1);
        w.rig.answer(IS_PATHING, 1);
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eManeuverState, 4);
        let (_, log) = w.rig.run(0x009c_e1d0, &args![w.t]);
        assert!(calls_to(&log, SET_PATHFINDING_GOAL).is_empty());
        assert_eq!(
            w.rig
                .e
                .get(w.this, CombatProcedureAttackMelee::eManeuverState),
            0
        );

        // The controller's test fails: re-paths; 008b3c70 skips the goal
        // check.
        let mut w = world();
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eManeuverState, 4);
        w.rig.answer(ACTOR_TEST_8B3C70, 1);
        w.rig.answer(GET_BLOCKED, 1);
        let (_, log) = w.rig.run(0x009c_e1d0, &args![w.t]);
        assert!(calls_to(&log, GET_CURRENT_PATHFINDING_GOAL).is_empty());
        assert_eq!(calls_to(&log, SET_PATHFINDING_GOAL).len(), 1);
        assert_eq!(calls_to(&log, SET_BLOCK), vec![vec![w.attacker, 0]]);
    }

    #[test]
    fn get_current_attack_reach_caches_the_weapon_reach() {
        let mut w = world();
        w.rig.e.mem.set_f32(w.t + 0x58, -1.0);
        w.rig.answer(CONTROLLER_WEAPON, 0x900);
        w.rig.answer_float(GET_WEAPON_REACH, 1.75);
        let (result, log) = w.rig.run(0x009c_e560, &args![w.t]);
        assert_eq!(result.f32(), 1.75);
        assert_eq!(
            calls_to(&log, GET_WEAPON_REACH),
            vec![vec![w.attacker, 0x900]]
        );
        let (result, log) = w.rig.run(0x009c_e560, &args![w.t]);
        assert_eq!(result.f32(), 1.75);
        assert!(calls_to(&log, GET_WEAPON_REACH).is_empty());
    }

    #[test]
    fn fn_009ce5b0_paths_to_the_point_or_the_target() {
        // A valid location in line of sight: avoids the target.
        let mut w = world();
        let point = w.rig.e.mem.alloc(0xc);
        w.rig.answer(PATHING_LOCATION_IS_VALID, 1);
        w.rig.answer(CALCULATE_LOS, 1);
        w.rig.answer(ALLOCATE, 0x3000);
        w.rig.answer(AVOID_ARRAY_CONSTRUCT, 0x3000);
        w.rig.answer(AVOID_NODE_CONSTRUCT, 0x3100);
        w.rig.answer(POINTER_GET, 0x3000);
        w.rig.answer(PATHING_LOCATION_WORLDSPACE, 0x3200);
        w.rig.answer(PATHING_LOCATION_CELL, 0x3300);
        w.rig.answer(PATHING_LOCATION_POINT, 0x3400);
        w.rig.answer_float(GET_RADIUS, 20.0);
        w.rig.setting(SETTING_MOVEMENT_NEAR_TARGET_AVOID_COST, 7.0);
        w.rig
            .setting(SETTING_MOVEMENT_NEAR_TARGET_AVOID_RADIUS, 8.0);
        let (_, log) = w.rig.run(0x009c_e5b0, &args![w.t, point]);
        let request = calls_to(&log, PATHING_REQUEST_CONSTRUCT)[0][0];
        let location = calls_to(&log, PATHING_LOCATION_FROM_POINT)[0][0];
        assert_eq!(
            calls_to(&log, PATHING_LOCATION_IS_VALID),
            vec![vec![location, bits(20.0)]]
        );
        assert_eq!(
            calls_to(&log, CALCULATE_LOS),
            vec![vec![w.attacker, w.target, point, w.position, 0, 0x1e]]
        );
        let node = calls_to(&log, AVOID_NODE_CONSTRUCT);
        assert_eq!(node[0][1..], [w.position, bits(8.0), bits(7.0)]);
        assert_eq!(calls_to(&log, AVOID_ARRAY_ADD), vec![vec![0x3000, 0x3100]]);
        assert_eq!(
            calls_to(&log, BUILD_REQUEST),
            vec![vec![
                w.attacker,
                request,
                0x3400,
                0x3300,
                0x3200,
                bits(5.0),
                0x3000
            ]]
        );
        assert_eq!(
            calls_to(&log, SET_PATHFINDING_GOAL),
            vec![vec![w.attacker, request]]
        );
        assert_eq!(calls_to(&log, AVOID_POINTER_DESTRUCT).len(), 1);
        assert_eq!(
            calls_to(&log, PATHING_REQUEST_DESTRUCT),
            vec![vec![request]]
        );

        // Otherwise: to the target within the face-target distance.
        let mut w = world();
        let point = w.rig.e.mem.alloc(0xc);
        w.rig.answer(COMBAT_STATE_TEST_F2F0, 1);
        w.rig.answer_float(COMPUTE_FACE_TARGET_DISTANCE, 12.5);
        w.rig.answer(GET_WORLD_SPACE, 0x3200);
        w.rig.answer(ACTOR_CELL_8D6F30, 0x3300);
        let (_, log) = w.rig.run(0x009c_e5b0, &args![w.t, point]);
        let request = calls_to(&log, PATHING_REQUEST_CONSTRUCT)[0][0];
        assert_eq!(
            calls_to(&log, COMPUTE_FACE_TARGET_DISTANCE),
            vec![vec![w.attacker, w.target, bits(3.0)]]
        );
        assert_eq!(
            calls_to(&log, BUILD_REQUEST),
            vec![vec![
                w.attacker,
                request,
                w.position,
                0x3300,
                0x3200,
                bits(12.5),
                0
            ]]
        );
        assert!(calls_to(&log, ALLOCATE).is_empty());
    }

    #[test]
    fn select_power_attack_takes_the_qualifying_direction() {
        // Only direction 1 (forward) qualifies: distance 1 (no grenade),
        // weight 0, the scaled reach 3 covers it.
        let mut w = world();
        for i in 0..5 {
            w.rig.e.mem.set_f32(w.controller + 0xfc + i * 4, 5.0);
            w.rig.e.mem.set_f32(w.controller + 0x110 + i * 4, -1.0);
        }
        w.rig.e.mem.set_f32(w.controller + 0xfc + 4, 1.0);
        w.rig.e.mem.set_f32(w.controller + 0x110 + 4, 0.0);
        w.rig.answer_float(FLOAT_MAX, 3.0);
        w.rig.answer(CONTROLLER_ATTACK_CHECK, 1);
        w.rig.answer(DEBUG_MESSAGES_ON, 1);
        w.rig.answer(REFERENCE_NAME, 0x5000);
        let (result, log) = w.rig.run(
            0x009c_e870,
            &args![w.t, bits(2.0), bits(3.0), 0u32, 0u32, 0u32],
        );
        assert_eq!(result.u32(), 0x5d);
        assert_eq!(
            calls_to(&log, DETERMINE_POWER_ATTACK_DELTAS),
            vec![vec![w.controller]]
        );
        let fill = calls_to(&log, FILL_BYTES);
        assert_eq!(fill[0][1..], [0, 5]);
        assert_eq!(calls_to(&log, CONTROLLER_ATTACK_CHECK)[0][1], 0x5d);
        assert_eq!(
            calls_to(&log, DEBUG_PRINT),
            vec![vec![TEXT_SELECTS_POWER_ATTACK, 0x5000, TEXT_FORWARD]]
        );
        assert!(calls_to(&log, DEBUG_WARNING).is_empty());

        // Nothing qualifies: 0xff without asking the controller.
        let mut w = world();
        for i in 0..5 {
            w.rig.e.mem.set_f32(w.controller + 0x110 + i * 4, -1.0);
        }
        let (result, log) = w.rig.run(
            0x009c_e870,
            &args![w.t, bits(2.0), bits(3.0), 0u32, 0u32, 0u32],
        );
        assert_eq!(result.u32(), 0xff);
        assert!(calls_to(&log, CONTROLLER_ATTACK_CHECK).is_empty());

        // Two qualify (0 and 3) and the style's flag is clear: the table
        // picks by the situation (3, left, with the left move free).
        let mut w = world();
        for i in 0..5 {
            w.rig.e.mem.set_f32(w.controller + 0x110 + i * 4, -1.0);
        }
        w.rig.e.mem.set_f32(w.controller + 0x110, 0.0);
        w.rig.e.mem.set_f32(w.controller + 0x110 + 12, 0.0);
        w.rig.e.mem.set_u8(w.controller + 0xf9, 1);
        w.rig.answer_float(FLOAT_MAX, 3.0);
        w.rig.answer(ACTOR_MOVE_FLAGS, 4);
        w.rig.e.mem.set_u32(POWER_ATTACK_BY_KIND + 12, 0x5f);
        w.rig.answer(FORM_ID, 0x1234);
        w.rig.answer(REFERENCE_NAME, 0x5000);
        let (result, log) = w.rig.run(
            0x009c_e870,
            &args![w.t, bits(2.0), bits(3.0), 0u32, 0u32, 0u32],
        );
        assert_eq!(result.u32(), 0x5f);
        assert!(calls_to(&log, DETERMINE_POWER_ATTACK_DELTAS).is_empty());
        assert_eq!(
            calls_to(&log, DEBUG_WARNING),
            vec![vec![TEXT_POWER_ATTACK_UNUSABLE, 0x5000, 0x1234, 0x5f]]
        );

        // Switching weapons: 0xff at once.
        let mut w = world();
        w.rig.answer(IS_SWITCHING_WEAPONS, 1);
        let (result, log) = w.rig.run(
            0x009c_e870,
            &args![w.t, bits(2.0), bits(3.0), 0u32, 0u32, 0u32],
        );
        assert_eq!(result.u32(), 0xff);
        assert!(calls_to(&log, CONTROLLER_TEST_8220).is_empty());
    }

    #[test]
    fn select_power_attack_draws_by_the_style_weights() {
        // Directions 0 and 4 qualify, the style's flag 1 is set and allows
        // both; the weights (1 and 1) and a draw of 2 pick the second.
        let mut w = world();
        for i in 0..5 {
            w.rig.e.mem.set_f32(w.controller + 0x110 + i * 4, -1.0);
        }
        w.rig.e.mem.set_f32(w.controller + 0x110, 0.0);
        w.rig.e.mem.set_f32(w.controller + 0x110 + 16, 0.0);
        w.rig.e.mem.set_u8(w.controller + 0xf9, 1);
        w.rig.answer_float(FLOAT_MAX, 3.0);
        w.rig.answer(STYLE_FLAG_TEST, 1);
        w.rig.answer(STYLE_HAS_ATTACK, 1);
        w.rig.answer(RANDOM_RANGE, 2);
        w.rig.answer(CONTROLLER_ATTACK_CHECK, 1);
        let (result, log) = w.rig.run(
            0x009c_e870,
            &args![w.t, bits(2.0), bits(3.0), 0u32, 0u32, 0u32],
        );
        assert_eq!(result.u32(), 0x60);
        assert_eq!(calls_to(&log, RANDOM_RANGE), vec![vec![0, 2]]);
    }

    #[test]
    fn fn_009cef70_9cef90_and_9cefb0_read_the_controller() {
        let mut w = world();
        let c = w.controller;
        w.rig.e.mem.set_f32(c + 0xfc + 8, 1.5);
        w.rig.e.mem.set_f32(c + 0x110 + 16, 2.5);
        w.rig.e.mem.set_u8(c + 0xf9, 1);
        assert_eq!(w.rig.run(0x009c_ef70, &args![c, 2u32]).0.f32(), 1.5);
        assert_eq!(w.rig.run(0x009c_ef90, &args![c, 4u32]).0.f32(), 2.5);
        assert!(w.rig.run(0x009c_efb0, &args![c]).0.bool());
    }

    #[test]
    fn fn_009cefd0_and_outside_attack_range_compare_with_the_reach() {
        let mut w = world();
        let within = |w: &mut World, a: f32, b: f32| {
            w.rig
                .run(0x009c_efd0, &args![w.t, bits(a), bits(b), 0u32])
                .0
                .bool()
        };
        assert!(within(&mut w, 2.0, 3.0));
        assert!(within(&mut w, 3.0, 3.0));
        assert!(!within(&mut w, 3.5, 3.0));
        assert!(!within(&mut w, f32::NAN, 3.0));
        let outside = |w: &mut World, a: f32, b: f32| {
            w.rig
                .run(0x009c_f030, &args![w.t, bits(a), bits(b), 0u32])
                .0
                .bool()
        };
        assert!(outside(&mut w, 3.0, 3.0));
        assert!(outside(&mut w, 3.5, 3.0));
        assert!(!outside(&mut w, 2.0, 3.0));
        assert!(!outside(&mut w, f32::NAN, 3.0));
        let (_, log) = w
            .rig
            .run(0x009c_f030, &args![w.t, bits(1.0), bits(1.0), 0u32]);
        assert_eq!(calls_to(&log, STYLE_SCORE_5524B0), vec![vec![w.style]]);
    }

    #[test]
    fn fn_009cf090_starts_a_power_attack_in_sight() {
        let setup = |los: u32| {
            let mut w = world();
            w.rig
                .e
                .set(w.this, CombatProcedureAttackMelee::eCombatState, 3);
            w.rig.answer_float(TIMER_ELAPSED, 2.0);
            w.rig.answer(CONTROLLER_TEST_8220, 1);
            w.rig.slot(SLOT_TEST_3F8, 1);
            w.rig.answer(CONTROLLER_TEST_F2D0, 1);
            w.rig.answer(UTILITY_TEST_9A6AE0, 1);
            for i in 0..5 {
                w.rig.e.mem.set_f32(w.controller + 0x110 + i * 4, -1.0);
            }
            w.rig.e.mem.set_f32(w.controller + 0x110, 0.0);
            w.rig.e.mem.set_u8(w.controller + 0xf9, 1);
            w.rig.answer_float(FLOAT_MAX, 3.0);
            w.rig.answer(CONTROLLER_ATTACK_CHECK, 1);
            w.rig.answer(STYLE_SCORE_5524B0, 1);
            w.rig.e.mem.set_u8(w.style + 0xad, 50);
            w.rig.answer(RANDOM_U32, 20);
            w.rig.answer_float(FORMULA_ATTACK_SCORE, 60.0);
            w.rig.answer_float(FORMULA_BLOCK_SCORE, 10.0);
            w.rig.answer(CALCULATE_LOS, los);
            w.rig.answer(QUEUE_ATTACK, 1);
            w.rig.answer(IS_PATHING, 1);
            w
        };
        let mut w = setup(1);
        let (_, log) = w.rig.run(0x009c_f090, &args![w.t, bits(2.0), bits(3.0)]);
        assert_eq!(
            calls_to(&log, CALCULATE_LOS),
            vec![vec![w.attacker, w.target, w.position, w.position, 0, 0x1e]]
        );
        assert_eq!(calls_to(&log, QUEUE_ATTACK), vec![vec![w.attacker, 0x5c]]);
        assert_eq!(
            w.rig
                .e
                .get(w.this, CombatProcedureAttackMelee::eCombatState),
            0
        );

        // Out of sight: the failed pick is stamped.
        let mut w = setup(0);
        w.rig.answer_float(TIME_NOW, 42.0);
        let (_, log) = w.rig.run(0x009c_f090, &args![w.t, bits(2.0), bits(3.0)]);
        let set = calls_to(&log, TIMESTAMP_SET);
        assert_eq!(set.len(), 1);
        assert_eq!(set[0][1], bits(42.0));
        assert!(calls_to(&log, QUEUE_ATTACK).is_empty());

        // A draw at the style's chance: nothing.
        let mut w = setup(1);
        w.rig.answer(RANDOM_U32, 50);
        let (_, log) = w.rig.run(0x009c_f090, &args![w.t, bits(2.0), bits(3.0)]);
        assert!(calls_to(&log, FORMULA_ATTACK_SCORE).is_empty());

        // Less than a second after the last failed pick: nothing.
        let mut w = setup(1);
        w.rig.answer_float(TIMER_ELAPSED, 0.5);
        let (_, log) = w.rig.run(0x009c_f090, &args![w.t, bits(2.0), bits(3.0)]);
        assert!(calls_to(&log, CONTROLLER_TEST_8220).is_empty());
    }

    #[test]
    fn fn_009cf400_reads_the_chance_byte() {
        let mut w = world();
        w.rig.e.mem.set_u8(w.style + 0xad, 0xf0);
        assert_eq!(w.rig.run(0x009c_f400, &args![w.style]).0.eax as u8, 0xf0);
    }

    #[test]
    fn fn_009cf420_asks_the_style() {
        let mut w = world();
        assert!(!w.rig.run(0x009c_f420, &args![w.t, 0x5cu32]).0.bool());
        w.rig.answer(STYLE_SCORE_5524B0, 2);
        assert!(w.rig.run(0x009c_f420, &args![w.t, 0x5cu32]).0.bool());
        w.rig.answer(STYLE_FLAG_TEST, 1);
        let (result, log) = w.rig.run(0x009c_f420, &args![w.t, 0x5eu32]);
        assert!(!result.bool());
        assert_eq!(calls_to(&log, STYLE_CHANCE_505C20), vec![vec![w.style]]);
        w.rig.answer(STYLE_CHANCE_9E8670, 3);
        assert!(w.rig.run(0x009c_f420, &args![w.t, 0x60u32]).0.bool());
        // Not a power attack group: allowed.
        assert!(w.rig.run(0x009c_f420, &args![w.t, 0x20u32]).0.bool());
    }

    #[test]
    fn fn_009cf570_runs_the_combat_state() {
        // Deciding with nothing in the way: the attack decision runs.
        let mut w = world();
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eCombatState, 3);
        let (_, log) = w.rig.run(0x009c_f570, &args![w.t]);
        assert_eq!(slot_calls(&log, SLOT_TEST_22C), vec![vec![w.target, 0]]);
        assert_eq!(calls_to(&log, IS_SWITCHING_WEAPONS).len(), 1);

        // The target's acquire object has an item: nothing.
        let mut w = world();
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eCombatState, 3);
        w.rig.slot(SLOT_TEST_610, 0x800);
        let (_, log) = w.rig.run(0x009c_f570, &args![w.t]);
        assert!(calls_to(&log, IS_SWITCHING_WEAPONS).is_empty());

        // Waiting: its step.
        let mut w = world();
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eCombatState, 2);
        let (_, log) = w.rig.run(0x009c_f570, &args![w.t]);
        assert_eq!(calls_to(&log, TIMER_EXPIRED), vec![vec![w.t + 0x20]]);
    }

    #[test]
    fn fn_009cf660_sets_the_maneuver_state() {
        let mut w = world();
        let (_, log) = w.rig.run(0x009c_f660, &args![w.t, 4u32]);
        assert_eq!(calls_to(&log, EMPTY_HOOK), vec![vec![w.controller]]);
        assert_eq!(
            w.rig
                .e
                .get(w.this, CombatProcedureAttackMelee::eManeuverState),
            4
        );
        assert!(w.rig.e.get(
            Ptr::<CombatMovementRestrictions>::new(w.t + 0x64),
            CombatMovementRestrictions::bReset
        ));
        // Leaving 4 for 0 while the target moves away is refused.
        w.rig.answer(ACTOR_TEST_4938E0, 1);
        w.rig.e.mem.set_f32(w.t + 0x5c, 0.0);
        let (_, log) = w.rig.run(0x009c_f660, &args![w.t, 0u32]);
        assert!(calls_to(&log, EMPTY_HOOK).is_empty());
        assert_eq!(
            w.rig
                .e
                .get(w.this, CombatProcedureAttackMelee::eManeuverState),
            4
        );
        // The same state again: only the store.
        let (_, log) = w.rig.run(0x009c_f660, &args![w.t, 4u32]);
        assert!(calls_to(&log, IS_PATHING).is_empty());
    }

    #[test]
    fn fn_009cf6d0_keeps_the_last_combat_state() {
        let mut w = world();
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eCombatState, 2);
        w.rig.run(0x009c_f6d0, &args![w.t, 1u32]);
        assert_eq!(
            w.rig
                .e
                .get(w.this, CombatProcedureAttackMelee::eCombatState),
            1
        );
        assert_eq!(
            w.rig
                .e
                .get(w.this, CombatProcedureAttackMelee::eLastCombatState),
            2
        );
    }

    #[test]
    fn display_debug_text_prints_the_states() {
        let mut w = world();
        let text = w.rig.object(8);
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eManeuverState, 2);
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eCombatState, 0);
        w.rig
            .e
            .set(w.this, CombatProcedureAttackMelee::eCurrentAttack, 0x5e);
        w.rig.e.mem.set_u32(MANEUVER_NAMES + 8, 0x1111);
        w.rig.e.mem.set_u32(COMBAT_STATE_NAMES, 0x2222);
        w.rig.answer(ACTOR_MOVE_FLAGS, 4);
        w.rig.answer_float(TIMER_DURATION, 2.0);
        w.rig.answer_float(TIMER_ELAPSED, 0.5);
        w.rig.answer(RESTRICTED_MOVES, 1);
        w.rig.e.mem.set_u16(w.t + 0x64 + 8, 8);
        w.rig.answer(RESTRICTIONS_TEST_9373F0, 1);
        let (_, log) = w.rig.run(0x009c_f700, &args![w.t, text, 5u32, 6u32]);
        let printed = calls_to(&log, PRINT_TO_BUFFER);
        assert_eq!(printed.len(), 1);
        let buffer = printed[0][0];
        let mut expected = vec![buffer, 0x64, TEXT_NAMED_TIMER, TEXT_SPACE_LEFT];
        expected.extend(args![0.5f64, 2.0f64]);
        assert_eq!(printed[0], expected);
        assert_eq!(
            calls_to(&log, COPY_TO_BUFFER),
            vec![vec![buffer, 0x64, TEXT_BACK_POWER]]
        );
        assert_eq!(
            slot_calls(&log, SLOT_PRINT),
            vec![
                vec![text, 5, 6, TEXT_MANEUVER, 0x1111, buffer],
                vec![text, 5, 6, TEXT_MELEE, 0x2222, buffer],
                vec![
                    text,
                    5,
                    6,
                    TEXT_RESTRICTIONS,
                    TEXT_Y,
                    TEXT_N,
                    TEXT_Y,
                    TEXT_Y
                ],
                vec![text, 5, 6, TEXT_RESET_RESTRICTIONS],
            ]
        );
        assert_eq!(calls_to(&log, MEMSET), vec![vec![buffer + 1, 0, 0x63]]);
    }

    #[test]
    fn fn_009cfbd0_fn_009cfc00_and_fn_009cfc40_use_the_restrictions() {
        let mut w = world();
        let r = Ptr::<CombatMovementRestrictions>::new(w.t + 0x64);
        w.rig
            .e
            .set(r, CombatMovementRestrictions::sRestrictedMoves, 2);
        assert!(!w.rig.run(0x009c_fbd0, &args![w.t, 2u32]).0.bool());
        assert!(w.rig.run(0x009c_fbd0, &args![w.t, 1u32]).0.bool());
        w.rig.answer(IS_PATHING, 1);
        w.rig.run(0x009c_fc00, &args![w.t]);
        assert!(!w.rig.e.get(r, CombatMovementRestrictions::bReset));
        w.rig.answer(IS_PATHING, 0);
        w.rig.run(0x009c_fc00, &args![w.t]);
        assert!(w.rig.e.get(r, CombatMovementRestrictions::bReset));
        w.rig.e.set(r, CombatMovementRestrictions::bReset, false);
        w.rig.run(0x009c_fc40, &args![r, 0u32]);
        assert!(w.rig.e.get(r, CombatMovementRestrictions::bReset));
    }

    #[test]
    fn stop_moving_and_the_distance_check() {
        let mut w = world();
        w.rig.answer_float(POINT_LENGTH_SQUARED, 100.0);
        w.rig
            .setting(SETTING_AI_UPDATE_MOVEMENT_RESTRICTIONS_DISTANCE, 10.0);
        w.rig.answer(RESTRICTIONS_POINT, 0x3500);
        let (_, log) = w.rig.run(0x009c_fc60, &args![w.t]);
        assert_eq!(calls_to(&log, ACTOR_CLEAR_8B3A50), vec![vec![w.attacker]]);
        assert_eq!(calls_to(&log, STOP_MOVING_ACTOR), vec![vec![w.attacker]]);
        assert_eq!(calls_to(&log, RESTRICTIONS_POINT), vec![vec![w.t + 0x64]]);
        let difference = calls_to(&log, POINT_DIFFERENCE);
        assert_eq!(difference[0][0], 0x3500);
        assert_eq!(difference[0][2], w.position);
        // 10 * 10 is not less than 100: reset.
        assert!(w.rig.e.get(
            Ptr::<CombatMovementRestrictions>::new(w.t + 0x64),
            CombatMovementRestrictions::bReset
        ));

        let mut w = world();
        w.rig.answer_float(POINT_LENGTH_SQUARED, 99.0);
        w.rig
            .setting(SETTING_AI_UPDATE_MOVEMENT_RESTRICTIONS_DISTANCE, 10.0);
        w.rig.run(0x009c_fc90, &args![w.t]);
        assert!(!w.rig.e.get(
            Ptr::<CombatMovementRestrictions>::new(w.t + 0x64),
            CombatMovementRestrictions::bReset
        ));
        w.rig.answer(PROCEDURE_TEST_525420, 1);
        let (_, log) = w.rig.run(0x009c_fc90, &args![w.t]);
        assert!(calls_to(&log, GET_ATTACKER).is_empty());
    }

    #[test]
    fn get_facing_target_distance_caches_the_distance() {
        let mut w = world();
        w.rig.e.mem.set_f32(w.t + 0x5c, -1.0);
        w.rig.answer_float(GET_DISTANCE_BETWEEN_FACING_ACTORS, 6.5);
        let (result, log) = w.rig.run(0x009c_fd20, &args![w.t]);
        assert_eq!(result.f32(), 6.5);
        assert_eq!(
            calls_to(&log, GET_DISTANCE_BETWEEN_FACING_ACTORS),
            vec![vec![w.attacker, w.target, 0]]
        );
        let (_, log) = w.rig.run(0x009c_fd20, &args![w.t]);
        assert!(calls_to(&log, GET_DISTANCE_BETWEEN_FACING_ACTORS).is_empty());
    }

    #[test]
    fn is_target_moving_away_compares_the_heading_distance() {
        let mut w = world();
        w.rig.answer(ACTOR_TEST_4938E0, 1);
        w.rig.answer_float(POINT_LENGTH_SQUARED, 4.0);
        let (result, log) = w.rig.run(0x009c_fd70, &args![w.t]);
        // 4 is not less than 2 * 2.
        assert!(result.bool());
        let velocity = slot_calls(&log, SLOT_VELOCITY);
        assert_eq!(velocity.len(), 1);
        let sum = calls_to(&log, POINT_SUM);
        assert_eq!(sum[0][0], w.position);
        assert_eq!(sum[0][2], velocity[0][1]);
        let difference = calls_to(&log, POINT_DIFFERENCE);
        assert_eq!(difference[0][2], sum[0][1]);
        w.rig.answer_float(POINT_LENGTH_SQUARED, 3.9);
        assert!(!w.rig.run(0x009c_fd70, &args![w.t]).0.bool());
        w.rig.answer(ACTOR_TEST_4938E0, 0);
        let (result, log) = w.rig.run(0x009c_fd70, &args![w.t]);
        assert!(!result.bool());
        assert!(slot_calls(&log, SLOT_VELOCITY).is_empty());
    }

    #[test]
    fn get_moving_target_velocity_is_the_velocity_length() {
        let mut w = world();
        assert_eq!(w.rig.run(0x009c_fe90, &args![w.t]).0.f64(), 0.0);
        w.rig.answer(ACTOR_TEST_4938E0, 1);
        w.rig.answer_float(POINT_LENGTH, 7.25);
        let (result, log) = w.rig.run(0x009c_fe90, &args![w.t]);
        assert_eq!(result.f64(), 7.25);
        assert_eq!(slot_calls(&log, SLOT_VELOCITY).len(), 1);
    }

    #[test]
    fn fn_009cff10_constructs_the_restrictions() {
        let mut w = world();
        let block = w.rig.e.mem.alloc(0x24);
        w.rig.e.mem.write(block, &[0xcc; 0x24]);
        for (i, value) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            w.rig.e.mem.set_f32(DEFAULT_POINT + i as u32 * 4, *value);
        }
        let (result, log) = w.rig.run(0x009c_ff10, &args![block]);
        assert_eq!(result.u32(), block);
        assert_eq!(w.rig.e.mem.u16(block), 0);
        assert_eq!(w.rig.e.mem.u32(block + 4), 0);
        assert_eq!(w.rig.e.mem.u16(block + 8), 0);
        assert_eq!(w.rig.e.mem.u32(block + 0xa), 0);
        assert_eq!(w.rig.e.mem.f32(block + 0x14), 2.0);
        assert_eq!(calls_to(&log, TIMER_CONSTRUCT), vec![vec![block + 0x1c]]);
    }

    #[test]
    fn fn_009cff90_checks_one_direction() {
        let mut w = world();
        let r = w.t + 0x64;
        let actor = w.attacker;
        let summed = w.rig.e.mem.alloc(0xc);
        w.rig.e.mem.set_f32(summed + 4, 9.0);
        w.rig.answer(POINT_SUM, summed);
        w.rig.slot(SLOT_HAS_MOVE_ANIM, 1);
        w.rig.answer_float(ACTOR_WIDTH, 0.0);
        w.rig.answer_float(ACTOR_LENGTH, 40.0);
        w.rig.answer_float(FLOAT_PAIR_EBD0, 40.0);
        w.rig.answer(ACTOR_MOVE_FLAGS, 1);
        w.rig.answer(CHECK_LINE_OF_SIGHT, 1);
        w.rig.e.set(
            Ptr::<CombatMovementRestrictions>::new(r),
            CombatMovementRestrictions::bReset,
            true,
        );
        w.rig.e.set(
            Ptr::<CombatMovementRestrictions>::new(r),
            CombatMovementRestrictions::sRestrictedMoves,
            0xf,
        );
        let (result, log) = w.rig.run(0x009c_ff90, &args![r, actor, 1u32]);
        assert_eq!(result.u32(), 1);
        assert_eq!(
            slot_calls(&log, SLOT_HAS_MOVE_ANIM)
                .iter()
                .map(|c| c[1])
                .collect::<Vec<_>>(),
            vec![5, 6, 3, 4]
        );
        assert_eq!(w.rig.e.mem.u32(r + 0xa), 0x0101_0101);
        // The width was 0: 32; with the length 40 within a factor of two,
        // both become FLOAT_MAX's answer (0 here).
        let pair = calls_to(&log, FLOAT_PAIR_EBD0);
        assert_eq!(pair.len(), 2);
        // Forward: (0, length, 0), lengthened while moving forward.
        let set = calls_to(&log, POINT_SET);
        assert_eq!(set[0][1..], [0, bits(40.0), 0]);
        assert_eq!(
            calls_to(&log, CHECK_LINE_OF_SIGHT)[0][1],
            set[0][0],
            "the summed point is checked"
        );
        assert_eq!(w.rig.e.mem.f32(set[0][0] + 4), 9.0);
        // In sight: the forward bit is cleared.
        assert_eq!(w.rig.e.mem.u16(r + 8), 0xe);

        // Not reset and pathing: 4.
        let mut w = world();
        let r = w.t + 0x64;
        w.rig.answer(IS_PATHING, 1);
        let (result, log) = w.rig.run(0x009c_ff90, &args![r, w.attacker, 2u32]);
        assert_eq!(result.u32(), 4);
        assert!(calls_to(&log, POINT_SET).is_empty());
    }

    #[test]
    fn fn_009d0530_sets_or_clears_restrictions() {
        let mut w = world();
        let r = w.t + 0x64;
        w.rig.e.mem.set_u16(r + 8, 0x5);
        assert_eq!(w.rig.run(0x009d_0530, &args![r, 2u32, false]).0.u16(), 7);
        assert_eq!(w.rig.run(0x009d_0530, &args![r, 1u32, true]).0.u16(), 6);
        assert_eq!(w.rig.e.mem.u16(r + 8), 6);
    }

    #[test]
    fn fn_009d0590_checks_the_line_of_sight() {
        let mut w = world();
        w.rig.answer(CHECK_LINE_OF_SIGHT, 1);
        let (result, log) = w
            .rig
            .run(0x009d_0590, &args![w.t + 0x64, w.attacker, 0x3600u32]);
        assert!(result.bool());
        let location = calls_to(&log, PATHING_LOCATION_FROM_ACTOR)[0][0];
        assert_eq!(
            calls_to(&log, PATHING_LOCATION_FROM_ACTOR),
            vec![vec![location, w.attacker]]
        );
        assert_eq!(
            calls_to(&log, CHECK_LINE_OF_SIGHT),
            vec![vec![location, 0x3600, 0, 0]]
        );
        assert_eq!(
            calls_to(&log, PATHING_LOCATION_DESTRUCT),
            vec![vec![location]]
        );
    }

    #[test]
    fn fn_009d0610_steps_through_the_directions() {
        let mut w = world();
        let r = Ptr::<CombatMovementRestrictions>::new(w.t + 0x64);
        w.rig.answer(TIMER_EXPIRED, 1);
        w.rig.answer(CONTROLLER_ACTOR, w.attacker);
        w.rig.answer(IS_PATHING, 1);
        let (_, log) = w.rig.run(0x009d_0610, &args![r, w.controller]);
        // Pathing: the step answers 4, a new round starts.
        assert_eq!(calls_to(&log, CONTROLLER_ACTOR), vec![vec![w.controller]]);
        assert_eq!(
            w.rig.e.get(r, CombatMovementRestrictions::cCurrentUpdate),
            0
        );
        assert!(!w.rig.e.get(r, CombatMovementRestrictions::bReset));
        w.rig
            .setting(SETTING_MELEE_MOVEMENT_RESTRICTIONS_UPDATE_TIME, 1.5);
        let (_, log) = w.rig.run(0x009d_0610, &args![r, w.controller]);
        assert_eq!(
            calls_to(&log, TIMER_START),
            vec![vec![r.addr() + 0x1c, bits(1.5)]]
        );
        // Timer not run out, no round under way: nothing.
        w.rig.answer(TIMER_EXPIRED, 0);
        let (_, log) = w.rig.run(0x009d_0610, &args![r, w.controller]);
        assert!(calls_to(&log, CONTROLLER_ACTOR).is_empty());
        assert!(calls_to(&log, TIMER_START).is_empty());
    }

    #[test]
    fn save_and_load_game_cover_every_field() {
        let mut w = world();
        let t = w.t;
        let (_, log) = w.rig.run(0x009d_0690, &args![t, 0x40u32]);
        let expected: Log = vec![
            (0x009d_0690, vec![t, 0x40]),
            (BASE_SAVE_GAME, vec![t, 0x40]),
            (SAVE_BYTES, vec![0x40, t + 0x10, 4, 0]),
            (SAVE_BYTES, vec![0x40, t + 0x14, 4, 0]),
            (SAVE_BYTES, vec![0x40, t + 0x18, 4, 0]),
            (SAVE_BYTES, vec![0x40, t + 0x1c, 4, 0]),
            (TIMER_SAVE, vec![t + 0x20, 0x40]),
            (TIMER_SAVE, vec![t + 0x28, 0x40]),
            (TIMER_SAVE, vec![t + 0x30, 0x40]),
            (TIMER_SAVE, vec![t + 0x38, 0x40]),
            (TIMER_SAVE, vec![t + 0x40, 0x40]),
            (TIMER_SAVE, vec![t + 0x48, 0x40]),
            (TIMESTAMP_SAVE, vec![t + 0x60, 0x40]),
            (SAVE_BYTES, vec![0x40, t + 0x50, 1, 0]),
            (SAVE_BYTES, vec![0x40, t + 0x54, 4, 0]),
        ];
        assert_eq!(log, expected);

        let buffer = w.rig.object(8);
        w.rig.slot(0, 0x11);
        let (_, log) = w.rig.run(0x009d_0790, &args![t, buffer]);
        assert!(calls_to(&log, TIMESTAMP_LOAD).is_empty());
        assert_eq!(calls_to(&log, LOAD_BYTES).len(), 6);
        assert_eq!(calls_to(&log, TIMER_LOAD).len(), 6);
        w.rig.slot(0, 0x12);
        let (_, log) = w.rig.run(0x009d_0790, &args![t, buffer]);
        assert_eq!(calls_to(&log, TIMESTAMP_LOAD), vec![vec![t + 0x60, buffer]]);
        assert_eq!(
            calls_to(&log, LOAD_BYTES).last().unwrap(),
            &vec![buffer, t + 0x54, 4]
        );
    }
}
