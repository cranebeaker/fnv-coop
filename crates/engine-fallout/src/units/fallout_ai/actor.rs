//! `fallout/ai/actor.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! This is the unit's main file: the `Actor` layout and the helpers shared with
//! the part files (`actor_p2.rs` ... `actor_p7.rs`) are declared here. The
//! first block of this file (batch b0148) covers the two functions of the
//! range that sit far from the rest (`004b4880`, `00575610`) and `0087d4a0` up
//! to `0087f620`: the constructor and destructor, the ragdoll set-up and a run
//! of one-line accessors.
//!
//! Notes for the next session (this file's range is `00000000` up to
//! `00884990`, translated in blocks of 40 functions in address order):
//! - Block 1 (b0148) ends with `0087f620`; block 2 (`0087f660` up to
//!   `00881360`: editor location, disposition modifiers, the actor-value
//!   change functions) ends with `00881360`. Block 3 (`00881450` up to
//!   `00884970`: package getters, the four process transitions `00881d30`,
//!   `00882b90`, `00883240`, `00883800`, and the movement predicates) closes
//!   this file's range: no function of `00000000 .. 00884990` is open.
//! - Virtual-slot arities used by block 3 were checked against the vtables
//!   (the `HighProcess` vtable at `01087864`, the `Actor` vtable at `01084254`):
//!   e.g. process `+0x160` takes 3 words, `+0x6c8` 2, `+0x24` 2, `+0x4c0` 4;
//!   actor `+0x1d0`, `+0x1f4`, `+0x1e4` take none.
//! - Functions at `0x0088xxxx` whose `this` is the actor-value owner
//!   sub-object (`Actor + 0xa4`) take it as a plain `Ptr` and subtract `0xa4`.
//! - `004181e0` (the engine map calls it `BGSSaveFormBuffer::GetForm`, the
//!   linker folded it) returns the reference's base form; [`base_form`] wraps
//!   it. `005d43c0` returns the reference's `ExtraDataList`
//!   ([`extra_data_list`]).
//! - The Xbox PDB's `Actor` is 0x1c4 bytes with `TESForm` at 0x28; on PC the
//!   whole class sits `0x10` lower (0x1b4 bytes), so every field offset below is
//!   the PDB's minus `0x10`.
//! - This unit's code is debug-style (every argument in a stack slot); the
//!   decompiler hangs pushed words on the wrong call, so every translation was
//!   read from the disassembly. A `PUSH x` before a nested call whose own `RET`
//!   takes no argument belongs to the OUTER call.
//! - The one-line wrappers `0087f350`, `0087f370`, `0087f390`, `0087f3b0`,
//!   `0087f4a0` and `0087f4c0` all forward to a `TESActorBase` predicate of the
//!   actor's base form; their names are the addresses because the map has none.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSSimpleList;

layout! {
    /// `Actor` (Xbox PDB), `0x1b4` bytes on PC (the PDB's `0x1c4` minus the `0x10`
    /// bytes of `TESForm` the PC build lacks). Offsets are the PC ones; only
    /// the fields the translations use are listed.
    pub struct Actor: 0x1b4 {
        /// `pCurrentProcess` (Xbox PDB): `BaseProcess *`.
        0x68 pCurrentProcess: Ptr,
        /// `fUpdateTargetTimer` (Xbox PDB), randomised by the initializer.
        0x74 fUpdateTargetTimer: f32,
        /// `bIgnoreChangeAnimationCall` (Xbox PDB), cleared by the initializer.
        0x7E bIgnoreChangeAnimationCall: u8,
        /// `pRagdollController` (Xbox PDB): `bhkRagdollController *`.
        0xAC pRagdollController: Ptr,
        /// `pPenetrationDetection` (Xbox PDB): `bhkRagdollPenetrationUtil *`.
        0xB0 pPenetrationDetection: Ptr,
        /// `ePersuasionEmotion` (Xbox PDB).
        0xB4 ePersuasionEmotion: u32,
        /// `fEmotionValue` (Xbox PDB).
        0xB8 fEmotionValue: f32,
        /// `bProcessMe` (Xbox PDB).
        0xBC bProcessMe: bool,
        /// `pMyKiller` (Xbox PDB).
        0xC0 pMyKiller: Ptr,
        /// `bMurderAlarm` (Xbox PDB).
        0xC4 bMurderAlarm: bool,
        /// `fCheckMyDeadBodyTimer` (Xbox PDB).
        0xC8 fCheckMyDeadBodyTimer: f32,
        /// `fDeadBodyAlarm` (Xbox PDB).
        0xCC fDeadBodyAlarm: f32,
        /// `bBlockPostAnim` (Xbox PDB).
        0xF0 bBlockPostAnim: bool,
        /// `bReloadTargetQueued` (Xbox PDB).
        0xF1 bReloadTargetQueued: bool,
        /// `DispModifierList` (Xbox PDB `+0x10c`): `BSSimpleList<DispositionModifier *>`,
        /// the list head embedded in the actor.
        0xFC DispModifierList: Inline<BSSimpleList>,
        /// `bInCombat` (Xbox PDB).
        0x104 bInCombat: bool,
        /// `eLifeState` (Xbox PDB).
        0x108 eLifeState: u32,
        /// `eCriticalStage` (Xbox PDB).
        0x10C eCriticalStage: u32,
        /// `eQueuedattack` (Xbox PDB).
        0x110 eQueuedattack: u32,
        /// `fLastUpdate` (Xbox PDB).
        0x114 fLastUpdate: f32,
        /// `bDeadFlag` (Xbox PDB).
        0x118 bDeadFlag: bool,
        /// `iVisFlags` (Xbox PDB).
        0x11C iVisFlags: u32,
        /// `iLastSeenTime` (Xbox PDB).
        0x120 iLastSeenTime: u32,
        /// `bForceRun` (Xbox PDB).
        0x124 bForceRun: bool,
        /// `bForceSneak` (Xbox PDB).
        0x125 bForceSneak: bool,
        /// `bForceUpdateQuestTarget` (Xbox PDB).
        0x126 bForceUpdateQuestTarget: bool,
        /// `bSearchingInCombat` (Xbox PDB).
        0x127 bSearchingInCombat: bool,
        /// `pCurrentCombatTarget` (Xbox PDB).
        0x128 pCurrentCombatTarget: Ptr,
        /// `pCurrentCombatTargetArray` (Xbox PDB).
        0x12C pCurrentCombatTargetArray: Ptr,
        /// `pCurrentCombatMemberArray` (Xbox PDB).
        0x130 pCurrentCombatMemberArray: Ptr,
        /// `bAttackOnNextTheft` (Xbox PDB).
        0x134 bAttackOnNextTheft: bool,
        /// `iThiefCrimeStamp` (Xbox PDB).
        0x138 iThiefCrimeStamp: u32,
        /// `iMinorCrimes` (Xbox PDB).
        0x13C iMinorCrimes: u32,
        /// `iMajorCrimes` (Xbox PDB).
        0x140 iMajorCrimes: u32,
        /// `bIgnoreCrime` (Xbox PDB).
        0x144 bIgnoreCrime: bool,
        /// `bEVPBuffered` (Xbox PDB).
        0x145 bEVPBuffered: bool,
        /// `bResetAI` (Xbox PDB).
        0x146 bResetAI: bool,
        /// `pTemplateActorBase` (Xbox PDB).
        0x148 pTemplateActorBase: Ptr,
        /// `bInWater` (Xbox PDB).
        0x14C bInWater: bool,
        /// `bSwimming` (Xbox PDB).
        0x14D bSwimming: bool,
        /// `bUpdateLighting` (Xbox PDB).
        0x14E bUpdateLighting: bool,
        /// `iActionValue` (Xbox PDB).
        0x150 iActionValue: u32,
        /// `fTimeronAction` (Xbox PDB).
        0x154 fTimeronAction: f32,
        /// `fHeadTrackTimer` (Xbox PDB).
        0x158 fHeadTrackTimer: f32,
        /// `bWasInFrustum` (Xbox PDB).
        0x15C bWasInFrustum: bool,
        /// `bShouldRotateToTrack` (Xbox PDB).
        0x15D bShouldRotateToTrack: bool,
        /// `fEditorLocZRot` (Xbox PDB).
        0x16C fEditorLocZRot: f32,
        /// `pEditorLocForm` (Xbox PDB).
        0x170 pEditorLocForm: Ptr,
        /// `bSetOnDeath` (Xbox PDB).
        0x174 bSetOnDeath: bool,
        /// `bContainerReset` (Xbox PDB).
        0x175 bContainerReset: bool,
        /// `bFootIKInRange` (Xbox PDB).
        0x18C bFootIKInRange: bool,
        /// `bPlayerTeammate` (Xbox PDB).
        0x18D bPlayerTeammate: bool,
        /// `bLightingUpdatedNonMoving` (Xbox PDB).
        0x18E bLightingUpdatedNonMoving: bool,
        /// `pActorMover` (Xbox PDB).
        0x190 pActorMover: Ptr,
        /// `pLastHitData` (Xbox PDB).
        0x194 pLastHitData: Ptr,
        /// `pInitialPackage` (Xbox PDB).
        0x198 pInitialPackage: Ptr,
        /// `pContinuousBeamPersistant` (Xbox PDB).
        0x1A0 pContinuousBeamPersistant: Ptr,
        /// `iEmotion` (Xbox PDB).
        0x1A4 iEmotion: u32,
        /// `iEmotionValue` (Xbox PDB).
        0x1A8 iEmotionValue: u32,
        /// `cCurrentSitSleepState` (Xbox PDB).
        0x1AC cCurrentSitSleepState: u32,
        /// `bTurretBehavior` (Xbox PDB).
        0x1B0 bTurretBehavior: u8,
        /// `bForceHitReaction` (Xbox PDB).
        0x1B1 bForceHitReaction: bool,
    }

    /// `DispositionModifier` (Xbox PDB), 8 bytes: one entry of
    /// [`Actor::DispModifierList`].
    pub struct DispositionModifier: 0x08 {
        /// `modifieramount` (Xbox PDB).
        0x00 modifieramount: i32,
        /// `pactormodified` (Xbox PDB): `Actor *`.
        0x04 pactormodified: Ptr,
    }

    /// `bhkRagdollController` (Xbox PDB, `0x2d0` bytes there; the PC size is not
    /// confirmed). Only the fields that `actor.cpp` reaches are listed, at PC
    /// offsets; the PDB's layout differs from the PC one past `+0x60`, so the
    /// fields past it are named by their offset.
    pub struct RagdollController: 0x2d0 {
        /// `bInitRagdollAnim` (Xbox PDB), the flag `0087ea20` reads.
        0x42 bInitRagdollAnim: u8,
        /// `iGroup` (Xbox PDB), set by `0087ea40`.
        0x5C iGroup: u32,
        /// The word `0087ea80` sets from `bhkCharacterProxy::operator P` (PC
        /// only; the PDB has `spSkeletonDebug` at `+0x90`).
        0x90 pCharacterProxy: Ptr,
        /// Byte set by `0087ea60` (PC offset; the PDB has other fields here).
        0x224 bFlag224: u8,
        /// Byte `0087e9d0` reads (PC offset).
        0x254 bFlag254: u8,
        /// Byte `0087e9d0` writes (PC offset).
        0x255 bFlag255: u8,
    }
}

// ---------------------------------------------------------------------------
// Callees outside this file

/// `TESObjectREFR`'s base form (`BGSSaveFormBuffer::GetForm` in the engine map:
/// the linker folded it with `TESForm`'s getter).
const GET_BASE_FORM: u32 = 0x0041_81e0;
/// `007af430`: the same getter under another address (`GetForm`).
const GET_FORM: u32 = 0x007a_f430;
/// `005d43c0`: the reference's `ExtraDataList`.
const GET_EXTRA_DATA_LIST: u32 = 0x005d_43c0;
/// `MobileObject::Load3D(bool)`.
const MOBILE_OBJECT_LOAD3D: u32 = 0x0093_35b0;
/// `Actor::ClearInCombat(bool)`.
const CLEAR_IN_COMBAT: u32 = 0x008a_08e0;
/// `Actor::StopCombat(bool)`.
const STOP_COMBAT: u32 = 0x008a_06c0;
/// Sets or clears the bits `mask` of the word at `this + 0x30`: `(this, set, mask)`.
const SET_FLAG_BITS: u32 = 0x0043_b370;
/// `operator new(size)` (cdecl).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// Allocates `size` bytes from the engine's memory manager (cdecl).
const ALLOCATE_BLOCK: u32 = 0x0055_3300;
/// Releases a memory-manager block (cdecl).
const FREE_BLOCK: u32 = 0x0055_3380;
/// `ExtraDataList::ExtraDataList()` (`0x20` bytes).
const EXTRA_DATA_LIST_CONSTRUCTOR: u32 = 0x0041_0360;
/// `__RTDynamicCast(object, 0, source type, target type, 0)`.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// Returns a pointer to a setting's value byte (`ECX` is the setting).
const SETTING_VALUE_POINTER: u32 = 0x0040_8d60;
/// `TESObjectREFR::Set3D(node, flag)`.
const SET_3D: u32 = 0x0057_02e0;
/// `PlayerCharacter::RemoveActorFromPlayercombatList`.
const REMOVE_FROM_PLAYER_COMBAT_LIST: u32 = 0x0093_a660;
/// `PlayerCharacter::RemovePerceivedActor`.
const REMOVE_PERCEIVED_ACTOR: u32 = 0x0096_7350;

/// `ProcessLists *` (the singleton object itself).
const PROCESS_LISTS: u32 = 0x011e_0e80;
/// The `LipSyncBackgroundManager` singleton.
const LIP_SYNC_MANAGER: u32 = 0x011f_11e0;
/// `PlayerCharacter *`.
const PLAYER_POINTER: u32 = 0x011d_ea3c;
/// A global pointer to the object that `004226e0` is called on.
const SHUTDOWN_CHECK_OBJECT_POINTER: u32 = 0x011c_3f2c;
/// `ModelLoader *`.
const MODEL_LOADER_POINTER: u32 = 0x011c_3b3c;
/// The global word `0087e9a0` returns.
const GLOBAL_VALUE_011C625C: u32 = 0x011c_625c;
/// The identity matrix the ragdoll set-up copies (9 floats).
const IDENTITY_MATRIX: u32 = 0x011a_9448;
/// `0.01745329238474369` (`double`), degrees to radians.
const DEGREES_TO_RADIANS: u32 = 0x0102_3128;
/// The format `"%s_%08X_%08X"` and the error and file-name strings of the
/// ragdoll set-up.
const RAGDOLL_NAME_FORMAT: u32 = 0x0108_4820;
const RAGDOLL_ERROR_CONTROLLER: u32 = 0x0108_47ec;
const RAGDOLL_ERROR_LOOK_IK: u32 = 0x0108_47bc;
const RAGDOLL_ERROR_FOOT_IK: u32 = 0x0108_4784;
const PRIME_TEXT: u32 = 0x0108_47b4;
const DEATH_POSE_FILE: u32 = 0x0108_4778;

// ---------------------------------------------------------------------------
// Helpers shared with the part files

/// `TESObjectREFR`'s base form (`004181e0`).
pub(crate) fn base_form(e: &mut Engine, this: Ptr<Actor>) -> Ptr {
    e.call(GET_BASE_FORM, &args![this]).ptr()
}

/// The reference's `ExtraDataList` (`005d43c0`).
pub(crate) fn extra_data_list(e: &mut Engine, this: Ptr<Actor>) -> Ptr {
    e.call(GET_EXTRA_DATA_LIST, &args![this]).ptr()
}

// ---------------------------------------------------------------------------
// Translations

// Translated from 004b4880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTransform::InvertNonUniform` (Xbox PDB): writes into `result` the inverse
/// of the transform `this` (rotation at `+0`, translate at `+0x24`, scale at
/// `+0x30`): the transposed rotation, `1 / scale`, and the translation
/// `scale' * (R' * (-t))` computed with the helpers `004a0bd0`, `004b4500` and
/// `004a3760`.
pub fn ni_transform_invert_non_uniform(e: &mut Engine, this: Ptr, result: Ptr) {
    // Locals: the transposed matrix (0x24 bytes) and three point temporaries.
    e.with_stack(0x80, |e, frame| {
        let matrix = frame;
        let negated_slot = frame.byte_add(0x30);
        let rotated_slot = frame.byte_add(0x40);
        let scaled_slot = frame.byte_add(0x50);
        let transposed: Ptr = e.call(0x0047_68c0, &args![this, matrix]).ptr();
        for i in 0..9 {
            let word = e.mem.u32(transposed.addr() + 4 * i);
            e.mem.set_u32(result.addr() + 4 * i, word);
        }
        let scale = e.mem.f32(this.addr() + 0x30);
        let inverse = (1.0f64 / scale as f64) as f32;
        e.mem.set_f32(result.addr() + 0x30, inverse);
        let translate = this.byte_add(0x24);
        let negated: Ptr = e.call(0x004a_0bd0, &args![translate, negated_slot]).ptr();
        let rotated: Ptr = e
            .call(0x004b_4500, &args![result, rotated_slot, negated])
            .ptr();
        let scale_now = e.mem.f32(result.addr() + 0x30);
        let scaled: Ptr = e
            .call(0x004a_3760, &args![scaled_slot, scale_now, rotated])
            .ptr();
        for i in 0..3 {
            let word = e.mem.u32(scaled.addr() + 4 * i);
            e.mem.set_u32(result.addr() + 0x24 + 4 * i, word);
        }
    });
}

// Translated from 00575610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetItemCountinContainer` (Xbox PDB): the count of `item` in the
/// actor's inventory changes (0 when it has none).
pub fn actor_get_item_countin_container(e: &mut Engine, this: Ptr<Actor>, item: Ptr) -> u32 {
    let changes: Ptr = e.call(0x004b_f220, &args![this]).ptr();
    if changes.is_null() {
        0
    } else {
        e.call(0x004c_8f30, &args![changes, item]).u32()
    }
}

// Translated from 0087d4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::Actor` (Xbox PDB): constructs the base `MobileObject`, the parts at
/// `+0x88`, `+0x94`, `+0xa4` and `+0xa8`, installs the six vtables, builds the
/// two modifier lists (`+0xd0`, `+0xe0`), the two simple lists (`+0xf4`,
/// `+0xfc`) and the point at `+0x160`, then runs the initializer `0087d890`.
/// The compiler's exception-unwinding frame is not translated.
pub fn actor_constructor(e: &mut Engine, this: Ptr<Actor>, add_to_process_lists: bool) -> Ptr {
    e.call(0x0092_eb50, &args![this]);
    e.call(0x0081_5300, &args![this.byte_add(0x88)]);
    e.call(0x0082_2a40, &args![this.byte_add(0x94)]);
    e.call(0x005f_78e0, &args![this.byte_add(0xa4)]);
    fn_0087d860(e, this.byte_add(0xa8));
    e.mem.set_u32(this.addr(), 0x0108_4254);
    e.mem.set_u32(this.addr() + 0x18, 0x0108_4248);
    e.mem.set_u32(this.addr() + 0x88, 0x0108_41f4);
    e.mem.set_u32(this.addr() + 0x94, 0x0108_41bc);
    e.mem.set_u32(this.addr() + 0xa4, 0x0108_418c);
    e.mem.set_u32(this.addr() + 0xa8, 0x0108_4144);
    e.call(0x0093_7000, &args![this.byte_add(0xd0)]);
    e.call(0x0093_7000, &args![this.byte_add(0xe0)]);
    e.call(0x0096_a2d0, &args![this.byte_add(0xf4)]);
    e.call(0x0096_a2d0, &args![this.byte_add(0xfc)]);
    e.call(0x0068_15c0, &args![this.byte_add(0x160)]);
    fn_0087d890(e, this, add_to_process_lists);
    this.cast()
}

// Translated from 0087d860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the small object at `Actor + 0xa8`: installs its vtable
/// (`01084734`) and returns it.
pub fn fn_0087d860(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0x0108_4734);
    this.cast()
}

// Translated from 0087d890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor`'s field initializer (called by the constructor): creates the
/// process object (`0xb4` bytes, `00906dc0`), registers the actor with
/// `ProcessLists` when asked (`add_to_process_lists`), and gives every field
/// its starting value, in the order of the exe.
pub fn fn_0087d890(e: &mut Engine, this: Ptr<Actor>, add_to_process_lists: bool) {
    e.call(0x0046_a010, &args![this, 1u32]);
    let block = e.call(OPERATOR_NEW, &args![0xb4u32]).u32();
    let process: Ptr = if block != 0 {
        e.call(0x0090_6dc0, &args![block]).ptr()
    } else {
        Ptr::NULL
    };
    e.set(this, Actor::pCurrentProcess, process);
    if add_to_process_lists {
        if e.call(0x008c_7aa0, &args![]).bool() {
            e.call(0x0096_e870, &args![PROCESS_LISTS, this]);
        } else {
            e.call(
                0x0096_d450,
                &args![PROCESS_LISTS, this, 3u32, 0u32, 0u32, 0u32],
            );
        }
    }
    let default_last_update: f32 = e.global(0x0101_2054);
    e.set(this, Actor::fLastUpdate, default_last_update);
    e.set(this, Actor::iLastSeenTime, 0);
    e.set(this, Actor::eLifeState, 0);
    e.set(this, Actor::eCriticalStage, 0);
    e.set(this, Actor::bDeadFlag, false);
    e.set(this, Actor::ePersuasionEmotion, 8);
    e.set(this, Actor::bProcessMe, true);
    e.set(this, Actor::bMurderAlarm, false);
    e.set(this, Actor::fEmotionValue, 0.0);
    e.set(this, Actor::bForceSneak, false);
    e.set(this, Actor::bForceRun, false);
    e.set(this, Actor::pMyKiller, Ptr::NULL);
    // MobileObject's word at +0x70 (PDB +0x80, `pDialogueItemTarget`).
    e.mem.set_u32(this.addr() + 0x70, 0);
    e.set(this, Actor::pTemplateActorBase, Ptr::NULL);
    e.set(this, Actor::bForceUpdateQuestTarget, true);
    e.set(this, Actor::bInWater, false);
    e.set(this, Actor::fHeadTrackTimer, 0.0);
    e.set(this, Actor::bWasInFrustum, true);
    // The random delay before the first target update: `RandomFloat(low,
    // high)` of two game settings (`00403e20` returns a pointer to a
    // setting's value; the setting at `011df810` is the first argument).
    let high_pointer = e.call(0x0040_3e20, &args![0x011d_f828u32]).u32();
    let high = e.mem.f32(high_pointer);
    let low_pointer = e.call(0x0040_3e20, &args![0x011d_f810u32]).u32();
    let low = e.mem.f32(low_pointer);
    let random = e.call(0x0047_6b70, &args![low, high]).f32();
    e.set(this, Actor::fUpdateTargetTimer, random);
    e.set(this, Actor::iVisFlags, 0);
    e.set(this, Actor::pEditorLocForm, Ptr::NULL);
    e.set(this, Actor::fEditorLocZRot, 0.0);
    // `EditorLocCoord` (a `NiPoint3` at +0x160) starts as the three words at 011f426c.
    for i in 0..3 {
        let word: u32 = e.global(0x011f_426c + 4 * i);
        e.mem.set_u32(this.addr() + 0x160 + 4 * i, word);
    }
    e.set(this, Actor::fCheckMyDeadBodyTimer, 0.0);
    let default_dead_body_alarm: f32 = e.global(0x0101_7718);
    e.set(this, Actor::fDeadBodyAlarm, default_dead_body_alarm);
    e.set(this, Actor::bSetOnDeath, false);
    e.set(this, Actor::bContainerReset, true);
    e.set(this, Actor::eQueuedattack, 0xff);
    e.vcall(this.addr(), 0x4d0, &args![]);
    e.call(CLEAR_IN_COMBAT, &args![this, 0u32]);
    e.set(this, Actor::bSwimming, false);
    e.set(this, Actor::pRagdollController, Ptr::NULL);
    let block = e.call(ALLOCATE_BLOCK, &args![0x30u32]).u32();
    let detection: Ptr = if block != 0 {
        e.call(0x00ca_0dc0, &args![block]).ptr()
    } else {
        Ptr::NULL
    };
    e.set(this, Actor::pPenetrationDetection, detection);
    e.set(this, Actor::bFootIKInRange, false);
    e.call(0x008b_bdb0, &args![this]);
    e.call(0x008b_be00, &args![this]);
    e.vcall(this.addr(), 0x3c4, &args![]);
    e.call(0x008c_4400, &args![this]);
    e.set(this, Actor::iActionValue, 0);
    e.set(this, Actor::fTimeronAction, 0.0);
    e.set(this, Actor::bReloadTargetQueued, false);
    e.call(0x0093_7090, &args![this.byte_add(0xe0), 1u32]);
    e.set(this, Actor::bEVPBuffered, false);
    e.set(this, Actor::bResetAI, false);
    e.set(this, Actor::bIgnoreChangeAnimationCall, 0);
    e.set(this, Actor::bBlockPostAnim, false);
    e.set(this, Actor::iEmotion, 0);
    e.set(this, Actor::iEmotionValue, 0);
    e.set(this, Actor::bPlayerTeammate, false);
    e.set(this, Actor::bSearchingInCombat, false);
    e.set(this, Actor::pCurrentCombatTarget, Ptr::NULL);
    e.set(this, Actor::pCurrentCombatTargetArray, Ptr::NULL);
    e.set(this, Actor::pCurrentCombatMemberArray, Ptr::NULL);
    e.set(this, Actor::cCurrentSitSleepState, 0);
    e.set(this, Actor::bLightingUpdatedNonMoving, false);
    e.set(this, Actor::pLastHitData, Ptr::NULL);
    e.set(this, Actor::bShouldRotateToTrack, true);
    e.set(this, Actor::pInitialPackage, Ptr::NULL);
    e.set(this, Actor::bIgnoreCrime, false);
    e.set(this, Actor::bTurretBehavior, 0);
    e.set(this, Actor::pContinuousBeamPersistant, Ptr::NULL);
    e.set(this, Actor::bForceHitReaction, false);
    e.set(this, Actor::iMajorCrimes, 0);
    e.set(this, Actor::iMinorCrimes, 0);
}

// Translated from 0087dcb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::~Actor` (Xbox PDB), the body: puts the vtables back, deletes the
/// ragdoll controller and the penetration utility, releases the sound handle of
/// the reference's extra data, switches the NPC radio off, and, unless the
/// shutdown check `004077c0` says the world is going away, takes the actor out
/// of the player's combat and perceived lists, the process lists, the lip-sync
/// manager, its magic, and its process; then runs the destructors of the
/// members and the `MobileObject` base. The scalar-deleting wrapper and the
/// exception-unwinding frame are not translated.
pub fn actor_destructor(e: &mut Engine, this: Ptr<Actor>) {
    e.mem.set_u32(this.addr(), 0x0108_4254);
    e.mem.set_u32(this.addr() + 0x18, 0x0108_4248);
    e.mem.set_u32(this.addr() + 0x88, 0x0108_41f4);
    e.mem.set_u32(this.addr() + 0x94, 0x0108_41bc);
    e.mem.set_u32(this.addr() + 0xa4, 0x0108_418c);
    e.mem.set_u32(this.addr() + 0xa8, 0x0108_4144);
    let ragdoll = e.get(this, Actor::pRagdollController);
    if !ragdoll.is_null() {
        e.vcall(ragdoll.addr(), 0, &args![1u32]);
    }
    let detection = e.get(this, Actor::pPenetrationDetection);
    if !detection.is_null() {
        e.vcall(detection.addr(), 0, &args![1u32]);
    }
    // The sound handle of the extra data: constructed, fetched, released if
    // valid and destroyed at the end.
    let handle: Ptr = Ptr::new(e.mem.alloc(0x10));
    e.call(0x0041_a250, &args![handle]);
    let list = extra_data_list(e, this);
    e.call(0x0041_8890, &args![list, handle]);
    if e.call(0x00ad_8ce0, &args![handle]).bool() {
        e.call(0x00ad_8d10, &args![handle]);
    }
    e.call(0x0083_5980, &args![this]);
    if !e.call(0x0040_77c0, &args![this]).bool() {
        let player = e.global::<u32>(PLAYER_POINTER);
        e.call(REMOVE_FROM_PLAYER_COMBAT_LIST, &args![player, this]);
        e.call(REMOVE_PERCEIVED_ACTOR, &args![player, this]);
        e.call(0x0096_e6f0, &args![PROCESS_LISTS, this]);
        e.call(0x0087_fd20, &args![this]);
        e.call(0x0090_6050, &args![LIP_SYNC_MANAGER, this]);
        e.call(0x0097_8660, &args![PROCESS_LISTS, this]);
        let shutdown_object = e.global::<u32>(SHUTDOWN_CHECK_OBJECT_POINTER);
        if !e.call(0x0042_26e0, &args![shutdown_object]).bool() {
            e.call(0x0082_4970, &args![this.byte_add(0x94)]);
            e.call(0x0066_e100, &args![this]);
            e.call(0x0092_ed60, &args![this, 1u32]);
            e.call(0x0088_1680, &args![this, 0u32]);
            e.call(0x0097_6680, &args![PROCESS_LISTS, this]);
            e.call(0x0096_f600, &args![PROCESS_LISTS, this, 1u32]);
            e.call(0x0097_5320, &args![PROCESS_LISTS, this]);
            let process = e.get(this, Actor::pCurrentProcess);
            if !process.is_null() {
                let kind = e.call(0x0045_cd60, &args![process]).i32();
                if kind == 1 || e.call(0x0045_cd60, &args![process]).i32() == 0 {
                    let accumulator = e.call(0x00b4_f5c0, &args![]).u32();
                    if accumulator != 0 {
                        let value = e.call(0x0084_e3a0, &args![this]).u32();
                        e.call(0x00b6_6050, &args![accumulator, value]);
                    }
                    e.call(0x008c_30a0, &args![this]);
                }
                e.call(0x008c_51f0, &args![this]);
            }
        }
        e.call(0x0096_f400, &args![PROCESS_LISTS, this]);
        if e.get(this, Actor::bInCombat) {
            e.call(STOP_COMBAT, &args![this, 0u32]);
        }
        e.call(0x008c_5090, &args![this]);
        e.call(0x0093_70b0, &args![this.byte_add(0xd0)]);
        e.call(0x0093_70b0, &args![this.byte_add(0xe0)]);
        e.call(0x008b_3180, &args![this]);
        e.call(SET_3D, &args![this, 0u32, 0u32]);
    }
    e.call(0x0048_3710, &args![handle]);
    e.mem.free(handle.addr());
    e.call(0x0046_ffb0, &args![this.byte_add(0xfc)]);
    e.call(0x0046_ffb0, &args![this.byte_add(0xf4)]);
    e.call(0x0093_7030, &args![this.byte_add(0xe0)]);
    e.call(0x0093_7030, &args![this.byte_add(0xd0)]);
    e.call(0x0082_2a80, &args![this.byte_add(0x94)]);
    e.call(0x0081_53b0, &args![this.byte_add(0x88)]);
    e.call(0x0092_ec40, &args![this]);
}

// Translated from 0087e060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Wrapper around `MobileObject::Load3D(flag)`: when it returns a node, hands
/// it to the process (virtual `+0x6c8`, with the base form's virtual `+0x180`
/// result), clears the node's `0x800000` bits unless the setting at `011c7664`
/// is on, and tells `008b7360` about it when `008b7b70` is true. Returns the
/// node.
pub fn fn_0087e060(e: &mut Engine, this: Ptr<Actor>, flag: bool) -> Ptr {
    let node: Ptr = e.call(MOBILE_OBJECT_LOAD3D, &args![this, flag]).ptr();
    if !node.is_null() {
        let process = e.get(this, Actor::pCurrentProcess);
        if !process.is_null() {
            let base = base_form(e, this);
            let model = e.vcall(base.addr(), 0x180, &args![]).u32();
            e.vcall(process.addr(), 0x6c8, &args![node, model]);
        }
        let setting = e.call(SETTING_VALUE_POINTER, &args![0x011c_7664u32]).u32();
        if e.mem.u8(setting) == 0 {
            fn_0087e100(e, node, false);
        }
        if e.call(0x008b_7b70, &args![this]).bool() {
            e.call(0x008b_7360, &args![this, 1u32, 0u32]);
        }
    }
    node
}

// Translated from 0087e100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`flag` true) or clears the `0x800000` bits of the node's flag word at
/// `+0x30` (`0043b370(node, !flag, 0x800000)`: the helper's second argument is
/// the "set" request, so a false `flag` asks for them to be set).
pub fn fn_0087e100(e: &mut Engine, node: Ptr, flag: bool) {
    e.call(SET_FLAG_BITS, &args![node, !flag, 0x0080_0000u32]);
}

// Translated from 0087e130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::CreateRagdollInstance` (Xbox PDB). `biped` is the word given to
/// `004ade00` and to the controller's constructor (`bipedanim.cpp`);
/// `replace_existing` deletes the current controller first; `init_iks` runs the
/// look-at and foot IK set-up over the model's nodes; `unique_name` appends the
/// form id and reference to the model name.
///
/// Builds the ragdoll controller (`0x2c0` bytes, `00c7f060`), initializes its
/// look-at and foot IK (when asked), its pose matching from the `Death.psa`
/// file next to the model, and its character proxy. An actor whose base is
/// immobile and which is not in a world space only gets its turret flag set
/// (`0087e960`) and returns. The exception-unwinding frame and the stack
/// cookie check are not translated.
pub fn actor_create_ragdoll_instance(
    e: &mut Engine,
    this: Ptr<Actor>,
    biped: Ptr,
    replace_existing: bool,
    init_iks: bool,
    unique_name: bool,
) {
    let base = base_form(e, this);
    let node: Ptr = e.vcall(base.addr(), 0x180, &args![]).ptr();
    let world_space: Ptr = if node.is_null() {
        Ptr::NULL
    } else {
        e.call(0x004f_d380, &args![node]).ptr()
    };
    let existing = e.get(this, Actor::pRagdollController);
    if !existing.is_null() && replace_existing {
        e.vcall(existing.addr(), 0, &args![1u32]);
    }
    let global_word = fn_0087e9a0(e);
    let found = e.call(0x004a_de00, &args![biped, global_word]).u32();
    if found != 0 {
        let base_again = base_form(e, this);
        if e.call(0x005f_0c80, &args![base_again]).bool() && world_space.is_null() {
            fn_0087e960(e, this, true);
            return;
        }
    }
    let form: Ptr = e.call(GET_FORM, &args![this]).ptr();
    let model_file = e.call(0x0050_fd90, &args![form, this]).u32();
    let form_again: Ptr = e.call(GET_FORM, &args![this]).ptr();
    let lod_mult = e.call(0x0045_c6b0, &args![form_again]).u32();
    let loader = e.global::<u32>(MODEL_LOADER_POINTER);
    let loaded = e
        .call(
            0x0044_7080,
            &args![loader, model_file, lod_mult, 1u32, 0u32, 0u32, 1u32],
        )
        .u32();
    // Every local of the exe's frame gets a slot of its own here.
    let frame: Ptr = Ptr::new(e.mem.alloc(0x1000));
    let name_buffer = frame; // char[0x104]: the model file name
    let path_buffer = frame.byte_add(0x120); // char[0x104]: the directory of the model
    let matrix = frame.byte_add(0x240); // NiMatrix3 (0x24)
    let matrix_result = frame.byte_add(0x280); // NiMatrix3 out (0x24)
    let bit_set = frame.byte_add(0x2c0); // QuickBitSet (8)
    let array = frame.byte_add(0x2e0); // BSSimpleArray (0x10)
    let slot_word = frame.byte_add(0x300); // a word passed by address
    let biped_out = frame.byte_add(0x310); // out word of 00931ed0
    let pose_file = frame.byte_add(0x320); // object built by 00438170
    let pose_file_flag = frame.byte_add(0x330); // object built by 00633c90

    let form_third: Ptr = e.call(GET_FORM, &args![this]).ptr();
    e.call(0x0050_fdf0, &args![form_third, this, name_buffer]);
    if unique_name {
        let old = e.get(this, Actor::pRagdollController);
        e.call(
            0x0040_6d00,
            &args![
                name_buffer,
                0x104u32,
                RAGDOLL_NAME_FORMAT,
                name_buffer,
                this,
                old
            ],
        );
    }
    let block = e.call(ALLOCATE_BLOCK, &args![0x2c0u32]).u32();
    let controller: Ptr = if block != 0 {
        e.call(0x00c7_f060, &args![block, biped, loaded, name_buffer])
            .ptr()
    } else {
        Ptr::NULL
    };
    e.set(this, Actor::pRagdollController, controller);
    if fn_0087ea20(e, controller) == 0 {
        let name = ragdoll_error_name(e, this);
        e.call(0x005b_5e40, &args![RAGDOLL_ERROR_CONTROLLER, name]);
        e.mem.free(frame.addr());
        return;
    }
    e.call(0x0068_15c0, &args![matrix]);
    for i in 0..9 {
        let word: u32 = e.global(IDENTITY_MATRIX + 4 * i);
        e.mem.set_u32(matrix.addr() + 4 * i, word);
    }
    e.call(0x0056_fac0, &args![this, matrix_result, matrix]);
    let controller = e.get(this, Actor::pRagdollController);
    let basis = e.call(0x004e_a950, &args![controller]).u32();
    e.call(0x004b_46a0, &args![matrix, basis]);
    if init_iks {
        e.call(0x0096_a2d0, &args![bit_set]);
        e.call(0x005e_5320, &args![node, bit_set]);
        e.call(0x005e_ede0, &args![array]);
        let mut look_ik_done = false;
        while !e.call(0x0082_56d0, &args![bit_set]).bool() {
            let current_slot = e.call(0x0068_15c0, &args![bit_set]).u32();
            let current = Ptr::new(e.mem.u32(current_slot));
            e.call(0x0063_f7b0, &args![bit_set]);
            if fn_0087e920(e, current) {
                let part = e.call(0x0063_d040, &args![current]).u32();
                e.mem.set_u32(slot_word.addr(), part);
                e.call(0x007c_b2e0, &args![array, slot_word]);
            }
            if !look_ik_done && fn_0087e940(e, current) {
                if !e.vcall(this.addr(), 0x21c, &args![]).bool() {
                    let controller = e.get(this, Actor::pRagdollController);
                    e.call(0x004e_4660, &args![controller, 1u32]);
                }
                look_ik_done = true;
                let part = e.call(0x0063_d040, &args![current]).u32();
                let controller = e.get(this, Actor::pRagdollController);
                e.call(0x00c7_de60, &args![controller, part]);
                let degrees = e.call(0x0051_b860, &args![current]).f32();
                let factor: f64 = e.global(DEGREES_TO_RADIANS);
                let radians = (degrees as f64 * factor) as f32;
                let controller = e.get(this, Actor::pRagdollController);
                e.call(0x0060_7810, &args![controller, 1u32, radians]);
                let controller = e.get(this, Actor::pRagdollController);
                e.call(0x005b_a4f0, &args![controller, 1u32]);
                let controller = e.get(this, Actor::pRagdollController);
                if !e.call(0x005b_a540, &args![controller]).bool() {
                    let name = ragdoll_error_name(e, this);
                    e.call(0x005b_5e40, &args![RAGDOLL_ERROR_LOOK_IK, name]);
                }
            }
        }
        let node_name = e.vcall(node.addr(), 0x130, &args![]).u32();
        if e.call(0x00ec_7750, &args![node_name, PRIME_TEXT]).u32() != 0 {
            let controller = e.get(this, Actor::pRagdollController);
            e.call(0x005c_6b40, &args![controller, 1u32]);
            let controller = e.get(this, Actor::pRagdollController);
            fn_0087ea60(e, controller, 0);
        }
        let controller = e.get(this, Actor::pRagdollController);
        e.call(0x00c7_c3a0, &args![controller, array]);
        let controller = e.get(this, Actor::pRagdollController);
        e.call(0x005b_a130, &args![controller, 1u32]);
        let controller = e.get(this, Actor::pRagdollController);
        e.call(0x005b_a310, &args![controller, 1u32]);
        let controller = e.get(this, Actor::pRagdollController);
        if !e.call(0x005b_a180, &args![controller]).bool()
            && e.call(0x0044_ddc0, &args![array]).u32() != 0
        {
            let name = ragdoll_error_name(e, this);
            e.call(0x005b_5e40, &args![RAGDOLL_ERROR_FOOT_IK, name]);
        }
        let controller = e.get(this, Actor::pRagdollController);
        if e.call(0x005b_a360, &args![controller]).bool() {
            e.call(0x008b_c300, &args![this]);
        }
        e.call(0x005e_ee10, &args![array]);
        e.call(0x0046_ffb0, &args![bit_set]);
    }
    e.call(0x0093_1ed0, &args![this, biped_out]);
    let group = e.call(0x004a_3a20, &args![biped_out]).u32();
    let controller = e.get(this, Actor::pRagdollController);
    fn_0087ea40(e, controller, group);
    // The model's file path, cut after its last backslash, plus "Death.psa".
    let model_path = e.vcall(node.addr() + 0x18, 0x14, &args![]).u32();
    e.call(0x0040_6d30, &args![path_buffer, 0x104u32, model_path]);
    let last_slash = e.call(0x0040_ab30, &args![path_buffer, 0x5cu32]).u32();
    if last_slash != 0 {
        e.mem.set_u8(last_slash + 1, 0);
        e.call(0x0040_6d50, &args![path_buffer, 0x104u32, DEATH_POSE_FILE]);
        e.call(0x0043_8170, &args![pose_file, path_buffer]);
        e.call(0x0063_3c90, &args![pose_file_flag, 0u32]);
        let pose = e.call(0x0043_b1b0, &args![pose_file]).u32();
        let controller = e.get(this, Actor::pRagdollController);
        e.call(
            0x00c7_b1f0,
            &args![
                controller,
                pose,
                0xffff_ffffu32,
                0xffff_ffffu32,
                0xffff_ffffu32
            ],
        );
        e.call(0x0045_cec0, &args![pose_file_flag]);
        e.call(0x0043_81b0, &args![pose_file]);
    }
    if !world_space.is_null() && this.addr() != e.global::<u32>(PLAYER_POINTER) {
        let area = fn_0087eaa0(e, world_space);
        let controller = e.get(this, Actor::pRagdollController);
        e.call(0x00c7_5ad0, &args![controller, area]);
    }
    let controller = e.get(this, Actor::pRagdollController);
    fn_0087e9d0(e, controller, true);
    let character_controller = e.call(0x0093_06d0, &args![this]).u32();
    if character_controller != 0 {
        let proxy = fn_0087e980(e, Ptr::new(character_controller));
        let controller = e.get(this, Actor::pRagdollController);
        fn_0087ea80(e, controller, Ptr::new(proxy));
    }
    e.mem.free(frame.addr());
}

/// The name an error message of the ragdoll set-up gives the actor: the
/// virtual `+0x130` when `00474cb0` says it has one, `0055d520` otherwise.
fn ragdoll_error_name(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    if e.call(0x0047_4cb0, &args![this]).u32() != 0 {
        e.vcall(this.addr(), 0x130, &args![]).u32()
    } else {
        e.call(0x0055_d520, &args![this]).u32()
    }
}

// Translated from 0087e920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bit `4` of the byte at `+0x60` of a model node (`NiAVObject`).
pub fn fn_0087e920(e: &mut Engine, node: Ptr) -> bool {
    e.mem.u8(node.addr() + 0x60) & 4 != 0
}

// Translated from 0087e940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bit `0x20` of the byte at `+0x60` of a model node (`NiAVObject`).
pub fn fn_0087e940(e: &mut Engine, node: Ptr) -> bool {
    e.mem.u8(node.addr() + 0x60) & 0x20 != 0
}

// Translated from 0087e960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `bTurretBehavior` (Xbox PDB `+0x1c0`).
pub fn fn_0087e960(e: &mut Engine, this: Ptr<Actor>, value: bool) {
    e.set(this, Actor::bTurretBehavior, value as u8);
}

// Translated from 0087e980 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word `00559450` reads at `+0x594` of the character controller (its
/// first word: the pointer stored there).
pub fn fn_0087e980(e: &mut Engine, character_controller: Ptr) -> u32 {
    e.call(0x0055_9450, &args![character_controller.byte_add(0x594)])
        .u32()
}

// Translated from 0087e9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The global word at `011c625c`.
pub fn fn_0087e9a0(e: &mut Engine) -> u32 {
    e.global(GLOBAL_VALUE_011C625C)
}

// Translated from 0087e9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases a memory-manager block (`00553380`, cdecl).
pub fn fn_0087e9b0(e: &mut Engine, block: Ptr) {
    e.call(FREE_BLOCK, &args![block]);
}

// Translated from 0087e9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// On a `bhkRagdollController`: sets the byte at `+0x255` to `[+0x254] && flag`.
pub fn fn_0087e9d0(e: &mut Engine, this: Ptr, flag: bool) {
    let this: Ptr<RagdollController> = this.cast();
    let on = e.get(this, RagdollController::bFlag254) != 0 && flag;
    e.set(this, RagdollController::bFlag255, on as u8);
}

// Translated from 0087ea20 (decompiled, FalloutNV.exe 1.4.0.525)
/// On a `bhkRagdollController`: the byte at `+0x42` (`bInitRagdollAnim`).
pub fn fn_0087ea20(e: &mut Engine, this: Ptr) -> u8 {
    e.get(
        this.cast::<RagdollController>(),
        RagdollController::bInitRagdollAnim,
    )
}

// Translated from 0087ea40 (decompiled, FalloutNV.exe 1.4.0.525)
/// On a `bhkRagdollController`: sets `iGroup` (`+0x5c`).
pub fn fn_0087ea40(e: &mut Engine, this: Ptr, group: u32) {
    e.set(
        this.cast::<RagdollController>(),
        RagdollController::iGroup,
        group,
    );
}

// Translated from 0087ea60 (decompiled, FalloutNV.exe 1.4.0.525)
/// On a `bhkRagdollController`: sets the byte at `+0x224`.
pub fn fn_0087ea60(e: &mut Engine, this: Ptr, value: u8) {
    e.set(
        this.cast::<RagdollController>(),
        RagdollController::bFlag224,
        value,
    );
}

// Translated from 0087ea80 (decompiled, FalloutNV.exe 1.4.0.525)
/// On a `bhkRagdollController`: stores at `+0x90` the pointer
/// `bhkCharacterProxy::operator P` (`004ae750`) makes of `proxy`.
pub fn fn_0087ea80(e: &mut Engine, this: Ptr, proxy: Ptr) {
    let pointer: Ptr = e.call(0x004a_e750, &args![proxy]).ptr();
    e.set(
        this.cast::<RagdollController>(),
        RagdollController::pCharacterProxy,
        pointer,
    );
}

// Translated from 0087eaa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address `0x40` bytes into the object.
pub fn fn_0087eaa0(_e: &mut Engine, this: Ptr) -> Ptr {
    this.byte_add(0x40)
}

// Translated from 0087eac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Puts the actor on `package`, or takes it off (`flag_a`): `flag_a` true is the
/// "leaving" path (the process's virtual `+0x210`/`+0x284`, `+0x48` of the
/// actor), false the "starting" one (the process's `+0x1e0` timer made
/// negative, `+0x230`, the sit/sleep checks). `flag_b` marks the package as
/// created (`TESPackage::SetIsCreated`). Does nothing without a process.
pub fn fn_0087eac0(e: &mut Engine, this: Ptr<Actor>, package: Ptr, flag_a: bool, flag_b: bool) {
    let process = e.get(this, Actor::pCurrentProcess);
    if process.is_null() {
        return;
    }
    if e.call(0x0096_11e0, &args![package]).i32() == -1 {
        let target = e.call(0x0056_9b80, &args![this]).u32();
        e.call(0x0067_77b0, &args![package, target]);
    }
    // Leaving this block (the label `ec07` of the exe) skips the rest of the
    // sit/sleep handling.
    'handled: {
        if package_kind(e, package) == 0x1c
            || package_kind(e, package) == 0x1b
            || package_kind(e, package) == 0x1a
        {
            break 'handled;
        }
        if e.vcall(this.addr(), 0x214, &args![]).u32() == 0 {
            break 'handled;
        }
        if (package_kind(e, package) == 0x15 || package_kind(e, package) == 0x1c)
            && e.vcall(this.addr(), 0x214, &args![]).u32() == 4
        {
            break 'handled;
        }
        let leave_furniture = e.vcall(this.addr(), 0x214, &args![]).u32() == 4
            || e.vcall(this.addr(), 0x214, &args![]).u32() == 9
            || {
                let acquire = e.call(0x008d_8520, &args![this]).u32();
                e.vcall(acquire, 0x4d4, &args![]).u32() != 0 && {
                    let acquire_again = e.call(0x008d_8520, &args![this]).u32();
                    let object = e.vcall(acquire_again, 0x4d4, &args![]).u32();
                    e.mem.u8(object + 0xe) > 0x14
                }
            };
        if leave_furniture {
            e.vcall(this.addr(), 0x418, &args![]);
            if flag_a {
                return;
            }
        } else {
            e.call(0x0088_d640, &args![this]);
        }
    }
    if !package.is_null() && flag_b {
        e.call(0x0067_4d70, &args![package, 1u32]);
    }
    let process_address = process.addr();
    if !flag_a {
        let timer = e.vcall(process_address, 0x1e0, &args![]).f32();
        let zero: f64 = e.global(0x0101_2060);
        if timer as f64 > zero {
            let timer = e.vcall(process_address, 0x1e0, &args![]).f32();
            let factor: f64 = e.global(0x0101_a6b0);
            let negated = (timer as f64 * factor) as f32;
            e.vcall(process_address, 0x1e4, &args![negated]);
            e.vcall(process_address, 0x214, &args![]);
            e.vcall(process_address, 0x294, &args![this]);
            let player = e.global::<u32>(PLAYER_POINTER);
            e.vcall(process_address, 0x628, &args![player]);
        }
        if package.is_null() || package_kind(e, package) != 0x12 {
            e.call(CLEAR_IN_COMBAT, &args![this, 1u32]);
        }
        e.vcall(process_address, 0x230, &args![package, this]);
        e.call(0x0045_34f0, &args![this, package]);
        if !e.call(0x0049_3bb0, &args![this]).bool() && e.call(0x008b_70d0, &args![this]).u32() != 0
        {
            let state = e.vcall(this.addr(), 0x214, &args![]).u32();
            let sitting_or_sleeping = state == 0
                || e.vcall(this.addr(), 0x214, &args![]).u32() == 9
                || e.vcall(this.addr(), 0x214, &args![]).u32() == 4;
            if sitting_or_sleeping && package_kind(e, package) != 0x1c {
                e.vcall(process_address, 0x614, &args![0x800u32]);
            }
        }
    } else {
        e.vcall(process_address, 0x210, &args![package, this]);
        e.vcall(process_address, 0x284, &args![0u32]);
        let mut notify = true;
        if package_kind(e, package) == 0x1c && fn_0087eed0(e, package) != 0 {
            notify = false;
        }
        if notify {
            e.vcall(process_address, 0x294, &args![this]);
        }
        e.vcall(this.addr(), 0x48, &args![0u32]);
    }
    e.vcall(process_address, 0x12c, &args![0u32]);
    e.vcall(process_address, 0x244, &args![]);
    if !e.call(0x0049_3bb0, &args![this]).bool() && fn_0087eeb0(e, this) == 0 {
        e.call(0x008b_3d30, &args![this]);
    }
    e.call(0x008a_8f60, &args![this]);
}

/// The package's type word (`0041ca90`).
fn package_kind(e: &mut Engine, package: Ptr) -> u32 {
    e.call(0x0041_ca90, &args![package]).u32()
}

// Translated from 0087eeb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bTurretBehavior` (Xbox PDB `+0x1c0`).
pub fn fn_0087eeb0(e: &mut Engine, this: Ptr<Actor>) -> u8 {
    e.get(this, Actor::bTurretBehavior)
}

// Translated from 0087eed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `+0xbe` of a `TESPackage`.
pub fn fn_0087eed0(e: &mut Engine, package: Ptr) -> u8 {
    e.mem.u8(package.addr() + 0xbe)
}

// Translated from 0087eef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Gives the actor the items its base form's container lists: builds a
/// snapshot of the base form's contents (`00487f70`, over the component the
/// base form's `+0x30` word leads to), and for every entry adds the item
/// (virtual `+0x190`) with the right count and, when the item has a script or
/// an entry health value, an extra-data list carrying them. Finally lets the
/// inventory changes of the reference's extra data update (`004d1960`).
pub fn fn_0087eef0(e: &mut Engine, this: Ptr<Actor>) {
    let base = base_form(e, this);
    let source = e.call(0x0044_1110, &args![base.byte_add(0x30)]).u32();
    if source != 0 {
        e.with_stack(0x40, |e, container| {
            e.call(0x0048_1610, &args![container]);
            let flags = e.call(0x0087_f9f0, &args![this]).u16();
            e.call(
                0x0048_7f70,
                &args![
                    Ptr::<()>::new(source).byte_add(0x30),
                    flags,
                    1u32,
                    container,
                    0u32
                ],
            );
            let mut node = e.call(0x0071_7e50, &args![container]).u32();
            while node != 0 {
                let data = e.call(0x0068_15c0, &args![node]).u32();
                let entry = e.mem.u32(data);
                if entry == 0 {
                    break;
                }
                add_default_inventory_entry(e, this, node, entry);
                node = e.call(0x0072_6070, &args![node]).u32();
            }
            e.call(0x0048_1680, &args![container]);
        });
    }
    let list = extra_data_list(e, this);
    let changes = e.call(0x0041_8520, &args![list]).u32();
    if changes != 0 {
        e.call(0x004d_1960, &args![changes]);
    }
}

/// One entry of the default inventory: `entry` is the container entry (count
/// at `+0`, item at `+4`, health description at `+8`); `node` is the list node
/// it came from.
fn add_default_inventory_entry(e: &mut Engine, this: Ptr<Actor>, node: u32, entry: u32) {
    let mut form = 0;
    let item = e.mem.u32(entry + 4);
    if item != 0 && e.vcall(item, 0xe4, &args![]).bool() {
        let data = e.call(0x0068_15c0, &args![node]).u32();
        let entry_again = e.mem.u32(data);
        form = e.mem.u32(entry_again + 4);
    }
    if form == 0 {
        return;
    }
    let script = e.call(0x0048_26d0, &args![item]).u32();
    let count = e.mem.u32(entry);
    let health_entry = e.mem.u32(entry + 8);
    if script != 0 {
        for _ in 0..count {
            let extra = new_extra_data_list(e);
            e.call(0x0041_9ad0, &args![extra, 1u32]);
            if extra != 0 && e.call(0x0041_8800, &args![extra]).u32() == 0 {
                e.call(0x0041_9ed0, &args![extra, script]);
                let attached = e.call(0x0041_8800, &args![extra]).u32();
                let value = e.call(0x005a_bf60, &args![attached]).u32();
                e.call(0x0041_9f80, &args![extra, value]);
            }
            if health_entry != 0 {
                let health = e.call(0x0048_72e0, &args![item]).u32();
                e.call(0x0040_ea20, &args![health_entry, extra, health]);
            }
            e.vcall(this.addr(), 0x190, &args![form, extra, 1u32]);
        }
    } else if health_entry != 0 {
        let extra = new_extra_data_list(e);
        e.call(0x0041_9ad0, &args![extra, count as u16]);
        let health = e.call(0x0048_72e0, &args![item]).u32();
        e.call(0x0040_ea20, &args![health_entry, extra, health]);
        e.vcall(this.addr(), 0x190, &args![form, extra, count]);
    } else {
        e.vcall(this.addr(), 0x190, &args![form, 0u32, count]);
    }
}

/// `new ExtraDataList` (`0x20` bytes), null when the allocation fails.
fn new_extra_data_list(e: &mut Engine) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x20u32]).u32();
    if block != 0 {
        e.call(EXTRA_DATA_LIST_CONSTRUCTOR, &args![block]).u32()
    } else {
        0
    }
}

// Translated from 0087f200 (decompiled, FalloutNV.exe 1.4.0.525)
/// False when the base form's component at `+0x30` says (virtual `+0x44`) yes
/// or `008ace90` says yes; true otherwise.
pub fn fn_0087f200(e: &mut Engine, this: Ptr<Actor>) -> bool {
    let base = base_form(e, this);
    if e.vcall(base.addr() + 0x30, 0x44, &args![]).bool() {
        return false;
    }
    !e.call(0x008a_ce90, &args![this]).bool()
}

// Translated from 0087f260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the actor may play a random idle. False when (the base is a
/// humanoid creature, or the actor's virtual `+0x218` is true) and (the
/// setting at `011df7f8` is on or the actor's virtual `+0x1a0(0)` is true);
/// false when `005a29b0` finds a last idle; false when the base form's
/// component at `+0x30` has virtual `+0x50` and virtual `+0x48` both true;
/// true otherwise.
pub fn fn_0087f260(e: &mut Engine, this: Ptr<Actor>) -> bool {
    let base = base_form(e, this);
    let as_creature = e
        .call(
            RT_DYNAMIC_CAST,
            &args![base, 0u32, 0x0118_46e8u32, 0x0118_3a00u32, 0u32],
        )
        .u32();
    let humanoid_creature = as_creature != 0 && e.call(0x005f_bf20, &args![as_creature]).bool();
    if humanoid_creature || e.vcall(this.addr(), 0x218, &args![]).bool() {
        let setting = e.call(SETTING_VALUE_POINTER, &args![0x011d_f7f8u32]).u32();
        if e.mem.u8(setting) != 0 || e.vcall(this.addr(), 0x1a0, &args![0u32]).bool() {
            return false;
        }
    }
    if e.call(0x005a_29b0, &args![this]).u32() != 0 {
        return false;
    }
    let base = base_form(e, this);
    if e.vcall(base.addr() + 0x30, 0x50, &args![]).bool() {
        let base = base_form(e, this);
        if e.vcall(base.addr() + 0x30, 0x48, &args![]).bool() {
            return false;
        }
    }
    true
}

// Translated from 0087f350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `005f0b50` (a `TESActorBase` predicate) on the actor's base form.
pub fn fn_0087f350(e: &mut Engine, this: Ptr<Actor>) -> bool {
    let base = base_form(e, this);
    e.call(0x005f_0b50, &args![base]).bool()
}

// Translated from 0087f370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `005f0bf0` (a `TESActorBase` predicate) on the actor's base form.
pub fn fn_0087f370(e: &mut Engine, this: Ptr<Actor>) -> bool {
    let base = base_form(e, this);
    e.call(0x005f_0bf0, &args![base]).bool()
}

// Translated from 0087f390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `005f0c40` (a `TESActorBase` predicate) on the actor's base form.
pub fn fn_0087f390(e: &mut Engine, this: Ptr<Actor>) -> bool {
    let base = base_form(e, this);
    e.call(0x005f_0c40, &args![base]).bool()
}

// Translated from 0087f3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `005f0ba0` (a `TESActorBase` predicate) on the actor's base form.
pub fn fn_0087f3b0(e: &mut Engine, this: Ptr<Actor>) -> bool {
    let base = base_form(e, this);
    e.call(0x005f_0ba0, &args![base]).bool()
}

// Translated from 0087f3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetEssential` (Xbox PDB): whether the actor is essential. Uses the
/// leveled-creature original base (`004216f0` on the extra data list) when it
/// has one, else the base form; false without a base. True while the player
/// exists, `00566950` is true for the actor and `004d1360` on the player is
/// false; otherwise `0087f480` on the base form's component at `+0x30` or
/// `008c1b30` on the actor.
pub fn actor_get_essential(e: &mut Engine, this: Ptr<Actor>) -> bool {
    let mut base = base_form(e, this);
    let list = extra_data_list(e, this);
    if e.call(0x0042_16f0, &args![list]).u32() != 0 {
        let list = extra_data_list(e, this);
        base = e.call(0x0042_16f0, &args![list]).ptr();
    }
    if base.is_null() {
        return false;
    }
    let player = e.global::<u32>(PLAYER_POINTER);
    if player != 0
        && e.call(0x0056_6950, &args![this]).bool()
        && !e.call(0x004d_1360, &args![player]).bool()
    {
        return true;
    }
    fn_0087f480(e, base.byte_add(0x30)) || e.call(0x008c_1b30, &args![this]).bool()
}

// Translated from 0087f480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit `2` of the flag word at `+4` of the object is set
/// (`00461580(this, 2)`).
pub fn fn_0087f480(e: &mut Engine, this: Ptr) -> bool {
    e.call(0x0046_1580, &args![this, 2u32]).bool()
}

// Translated from 0087f4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `0056af00` (bit `8` of the flag word) on the base form's component at `+0x30`.
pub fn fn_0087f4a0(e: &mut Engine, this: Ptr<Actor>) -> bool {
    let base = base_form(e, this);
    e.call(0x0056_af00, &args![base.byte_add(0x30)]).bool()
}

// Translated from 0087f4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESActorBase::GetSex` (`005f0cc0`) on the actor's base form: 0 or 1, or -1
/// when the base is not an NPC.
pub fn fn_0087f4c0(e: &mut Engine, this: Ptr<Actor>) -> i32 {
    let base = base_form(e, this);
    e.call(0x005f_0cc0, &args![base]).i32()
}

// Translated from 0087f4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The base form's virtual `+0x144`, with its result.
pub fn fn_0087f4e0(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    let base = base_form(e, this);
    e.vcall(base.addr(), 0x144, &args![]).u32()
}

// Translated from 0087f510 (decompiled, FalloutNV.exe 1.4.0.525)
/// 1 when the actor's virtual `+0x218` is false; otherwise the byte at `+0xc`
/// of the extra data of type `0x4e` when the reference has it, else
/// `005d2780` on the base form's component at `+0x30`.
pub fn fn_0087f510(e: &mut Engine, this: Ptr<Actor>) -> u8 {
    if !e.vcall(this.addr(), 0x218, &args![]).bool() {
        return 1;
    }
    let list = extra_data_list(e, this);
    let extra = e.call(0x0041_0220, &args![list, 0x4eu32]).u32();
    if extra != 0 {
        e.mem.u8(extra + 0xc)
    } else {
        let base = base_form(e, this);
        e.call(0x005d_2780, &args![base.byte_add(0x30)]).u8()
    }
}

// Translated from 0087f570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::SwimsOnly` (Xbox PDB): true when `0087f370` is false, `0087f350` is
/// true and `0087f390` is false.
pub fn actor_swims_only(e: &mut Engine, this: Ptr<Actor>) -> bool {
    !fn_0087f370(e, this) && fn_0087f350(e, this) && !fn_0087f390(e, this)
}

// Translated from 0087f5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when the actor's virtual `+0x218` is false and it only swims;
/// otherwise whether the actor-value owner at `+0xa4` (virtual `+8`) gives a
/// non-zero value for `0x35`.
pub fn fn_0087f5c0(e: &mut Engine, this: Ptr<Actor>) -> bool {
    if !e.vcall(this.addr(), 0x218, &args![]).bool() && actor_swims_only(e, this) {
        return true;
    }
    e.vcall(this.addr() + 0xa4, 8, &args![0x35u32]).u32() != 0
}

// Translated from 0087f620 (decompiled, FalloutNV.exe 1.4.0.525)
/// False when the actor's virtual `+0x218` is false and it only swims, true
/// otherwise.
pub fn fn_0087f620(e: &mut Engine, this: Ptr<Actor>) -> bool {
    e.vcall(this.addr(), 0x218, &args![]).bool() || !actor_swims_only(e, this)
}

// ---------------------------------------------------------------------------
// Block 2 (batch b0148, continued): `0087f660` up to `00881360`

/// `_ftol2_sse` (`00ec62c0`): truncates the value the game holds in `ST0`; the
/// uniform form takes it as a leading `f64` argument.
const FLOAT_TO_INT: u32 = 0x00ec_62c0;
/// The form-type byte of a form (`00401170`, the byte at `+4`).
const FORM_TYPE: u32 = 0x0040_1170;
/// Bit 0 of the byte at `+0x24` of an object (`00425fd0`).
const FLAG_BIT_0_AT_0X24: u32 = 0x0042_5fd0;
/// A list node's own address (`006815c0`): the item is the word at the result.
const LIST_NODE_ITEM_SLOT: u32 = 0x0068_15c0;
/// A list node's next pointer (`00726070`, the word at `+4`).
const LIST_NODE_NEXT: u32 = 0x0072_6070;
/// Removes the first node holding the item whose address is passed (a
/// pointer to a local holding the item): `(list, &item)`.
const LIST_REMOVE_ITEM: u32 = 0x0090_5330;
/// Appends the item whose address is passed: `(list, &item)`.
const LIST_APPEND_ITEM: u32 = 0x005a_e3d0;
/// Empties a list (`00470470`).
const LIST_CLEAR: u32 = 0x0047_0470;
/// Releases a block allocated with `operator new` (`00401030`, cdecl).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The actor's process (`008d8520`: the word at `+0x68`; the engine map names
/// it `MiddleHighProcess::GetSavedAcquireObject` because the code is folded).
const ACTOR_PROCESS: u32 = 0x008d_8520;
/// `bool` test of actor value `(index)`, true when the value is a plain
/// stat that has a base entry (`0066ee10`, cdecl).
const ACTOR_VALUE_HAS_BASE: u32 = 0x0066_ee10;
/// `(index, mask)`: whether the actor-value flag word has a bit of `mask`
/// (`00406d70`, cdecl).
const ACTOR_VALUE_HAS_FLAG: u32 = 0x0040_6d70;
/// Tells the actor value `(owner, index, base, value, other owner)` changed
/// (`0066ee50`, cdecl, five words).
const ACTOR_VALUE_CHANGED: u32 = 0x0066_ee50;
/// `(index)` whether `index` is within `0 ..= 0x2d` (`0047f060`, cdecl).
const ACTOR_VALUE_IS_STAT: u32 = 0x0047_f060;
/// Offset of the actor-value owner sub-object inside an `Actor`.
const ACTOR_VALUE_OWNER: u32 = 0xa4;
/// Offset of the editor location (three `float`s) inside an `Actor`.
const EDITOR_LOCATION: u32 = 0x160;

/// `_ftol2_sse` on a value in `ST0`.
fn float_to_int(e: &mut Engine, value: f64) -> i32 {
    e.call(FLOAT_TO_INT, &args![value]).i32()
}

/// Copies three words (a `NiPoint3`).
fn copy_point(e: &mut Engine, from: u32, to: u32) {
    for i in 0..3 {
        let word = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, word);
    }
}

/// The actor's process (`008d8520`).
fn actor_process(e: &mut Engine, this: Ptr<Actor>) -> Ptr {
    e.call(ACTOR_PROCESS, &args![this]).ptr()
}

/// `owner + 0xa4` of a pointer that may be null (null stays null).
fn owner_of(actor: u32) -> u32 {
    if actor != 0 {
        actor + ACTOR_VALUE_OWNER
    } else {
        0
    }
}

/// The base value of actor value `index` (the owner's virtual `+0xc`), or
/// `0.0` when `0066ee10` says the value has none.
fn base_actor_value(e: &mut Engine, this: Ptr<Actor>, index: u32) -> f32 {
    if e.call(ACTOR_VALUE_HAS_BASE, &args![index]).bool() {
        e.vcall(this.addr() + ACTOR_VALUE_OWNER, 0xc, &args![index])
            .f32()
    } else {
        0.0
    }
}

/// The report every actor-value change ends with: `0066ee50(owner of this,
/// index, base, value, owner of source)`.
fn report_actor_value_change(
    e: &mut Engine,
    this: Ptr<Actor>,
    index: u32,
    base: f32,
    value: f32,
    source: u32,
) {
    e.call(
        ACTOR_VALUE_CHANGED,
        &args![owner_of(this.addr()), index, base, value, owner_of(source)],
    );
}

/// The item of the first node of the disposition-modifier list whose actor
/// (`DispositionModifier::pactormodified`) is `target`, or 0.
fn find_disposition_modifier(e: &mut Engine, this: Ptr<Actor>, target: u32) -> u32 {
    let mut node = this.at(Actor::DispModifierList).cast::<()>();
    while !node.is_null() {
        let slot = e.call(LIST_NODE_ITEM_SLOT, &args![node]).u32();
        let item = e.mem.u32(slot);
        if item == 0 {
            break;
        }
        if e.mem.u32(item + 4) == target {
            return item;
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).ptr();
    }
    0
}

// Translated from 0087f660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::CheckBreathTimer` (Xbox PDB): false when the actor has a process
/// whose virtual `+0x300` (the breath timer) is smaller than the swim breath
/// time (`00648a10` of the integer part of `008be7a0`); true otherwise.
pub fn actor_check_breath_timer(e: &mut Engine, this: Ptr<Actor>) -> bool {
    let mut result = true;
    let level = e.call(0x008b_e7a0, &args![this]).f64();
    let level = float_to_int(e, level);
    let limit = e.call(0x0064_8a10, &args![level]).f32();
    let process = e.get(this, Actor::pCurrentProcess);
    if !process.is_null() {
        let timer = e.vcall(process.addr(), 0x300, &args![]).f32();
        if limit > timer {
            result = false;
        }
    }
    result
}

// Translated from 0087f6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The result of `00408da0` on the data at `+0x18` of `004ac110`'s record of
/// the actor's form (`MapMarkerData::GetLocationName` in the decompiler's
/// naming), 0 when the actor has no form (virtual `+0x218` false) or the
/// record's check `0048cee0` fails.
pub fn fn_0087f6c0(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    let mut form = 0;
    if e.vcall(this.addr(), 0x218, &args![]).bool() {
        form = e.call(GET_FORM, &args![this]).u32();
    }
    if form != 0 {
        let record = e.call(0x004a_c110, &args![form]).u32();
        let data = if record != 0 { record + 0x18 } else { 0 };
        if data != 0 && e.call(0x0048_cee0, &args![data]).u32() != 0 {
            return e.call(0x0040_8da0, &args![data]).u32();
        }
    }
    0
}

// Translated from 0087f750 (decompiled, FalloutNV.exe 1.4.0.525)
/// The editor location form when it is set and its form type is `0x41`, else null.
pub fn fn_0087f750(e: &mut Engine, this: Ptr<Actor>) -> Ptr {
    let form = e.get(this, Actor::pEditorLocForm);
    if !form.is_null() && e.call(FORM_TYPE, &args![form]).u32() == 0x41 {
        form
    } else {
        Ptr::NULL
    }
}

// Translated from 0087f7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The editor location form when it is set, its form type is `0x39` and
/// `00425fd0` is true for it, else null.
pub fn fn_0087f7a0(e: &mut Engine, this: Ptr<Actor>) -> Ptr {
    let mut form = Ptr::NULL;
    let location = e.get(this, Actor::pEditorLocForm);
    if !location.is_null() && e.call(FORM_TYPE, &args![location]).u32() == 0x39 {
        form = location;
    }
    if !form.is_null() && !e.call(FLAG_BIT_0_AT_0X24, &args![form]).bool() {
        form = Ptr::NULL;
    }
    form
}

// Translated from 0087f800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Records the actor's current place as its editor location: the position
/// (`00436aa0`, three floats) into `+0x160`, the z rotation (virtual `+0x2bc`
/// with argument 0) into `fEditorLocZRot`, and the form: `008d6f30` when it
/// passes `00425fd0`, else `00575d70` (`TESObjectREFR::GetWorldSpace`).
pub fn fn_0087f800(e: &mut Engine, this: Ptr<Actor>) {
    let position = e.call(0x0043_6aa0, &args![this]).u32();
    copy_point(e, position, this.addr() + EDITOR_LOCATION);
    let z_rotation = e.vcall(this.addr(), 0x2bc, &args![0u32]).f32();
    e.set(this, Actor::fEditorLocZRot, z_rotation);
    if e.call(0x008d_6f30, &args![this]).u32() != 0 {
        let candidate = e.call(0x008d_6f30, &args![this]).ptr::<()>();
        if e.call(FLAG_BIT_0_AT_0X24, &args![candidate]).bool() {
            let form = e.call(0x008d_6f30, &args![this]).ptr();
            e.set(this, Actor::pEditorLocForm, form);
            return;
        }
    }
    let form = e.call(0x0057_5d70, &args![this]).ptr();
    e.set(this, Actor::pEditorLocForm, form);
}

// Translated from 0087f890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the editor location from `position` and `rotation`; the form is
/// `candidate` when it is set and passes `00425fd0`, else `fallback`.
pub fn fn_0087f890(
    e: &mut Engine,
    this: Ptr<Actor>,
    fallback: Ptr,
    candidate: Ptr,
    position: Ptr,
    rotation: f32,
) {
    copy_point(e, position.addr(), this.addr() + EDITOR_LOCATION);
    e.set(this, Actor::fEditorLocZRot, rotation);
    if !candidate.is_null() && e.call(FLAG_BIT_0_AT_0X24, &args![candidate]).bool() {
        e.set(this, Actor::pEditorLocForm, candidate);
    } else {
        e.set(this, Actor::pEditorLocForm, fallback);
    }
}

// Translated from 0087f900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the actor has an editor location form.
pub fn fn_0087f900(e: &mut Engine, this: Ptr<Actor>) -> bool {
    !e.get(this, Actor::pEditorLocForm).is_null()
}

// Translated from 0087f920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the editor location back: false when the actor has none; otherwise
/// the position goes to `position`, `(0, 0, z rotation)` to `rotation` and the
/// form to the word at `form`, and the result is true. The fourth word the
/// function pops is never read.
pub fn fn_0087f920(
    e: &mut Engine,
    this: Ptr<Actor>,
    position: Ptr,
    rotation: Ptr,
    form: Ptr,
    _unused_3: u32,
) -> bool {
    let location = e.get(this, Actor::pEditorLocForm);
    if location.is_null() {
        return false;
    }
    copy_point(e, this.addr() + EDITOR_LOCATION, position.addr());
    e.mem.set_f32(rotation.addr(), 0.0);
    e.mem.set_f32(rotation.addr() + 4, 0.0);
    let z_rotation = e.get(this, Actor::fEditorLocZRot);
    e.mem.set_f32(rotation.addr() + 8, z_rotation);
    e.mem.set_u32(form.addr(), location.addr());
    true
}

// Translated from 0087f990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `005bb4d0` on the base form's component at `+0x90`.
pub fn fn_0087f990(e: &mut Engine, this: Ptr<Actor>) -> bool {
    let base = base_form(e, this);
    e.call(0x005b_b4d0, &args![base.byte_add(0x90)]).bool()
}

// Translated from 0087f9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The actor-value owner's virtual `+0` (an integer) for actor value `0x15`,
/// as a `float`.
pub fn fn_0087f9c0(e: &mut Engine, this: Ptr<Actor>) -> f32 {
    let value = e
        .vcall(this.addr() + ACTOR_VALUE_OWNER, 0, &args![0x15u32])
        .i32();
    value as f32
}

// Translated from 0087f9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `0047ded0` on the base form's component at `+0x30`.
pub fn fn_0087f9f0(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    let base = base_form(e, this);
    e.call(0x0047_ded0, &args![base.byte_add(0x30)]).u32()
}

// Translated from 0087fa10 (decompiled, FalloutNV.exe 1.4.0.525)
/// With a process: its virtual `+0x118` with `flag`, then its virtual `+0x4a0`.
pub fn fn_0087fa10(e: &mut Engine, this: Ptr<Actor>, flag: bool) {
    let process = e.get(this, Actor::pCurrentProcess);
    if !process.is_null() {
        e.vcall(process.addr(), 0x118, &args![flag]);
        e.vcall(process.addr(), 0x4a0, &args![]);
    }
}

// Translated from 0087fa60 (decompiled, FalloutNV.exe 1.4.0.525)
/// With a process: its virtual `+0x120` with `flag`.
pub fn fn_0087fa60(e: &mut Engine, this: Ptr<Actor>, flag: bool) {
    let process = e.get(this, Actor::pCurrentProcess);
    if !process.is_null() {
        e.vcall(process.addr(), 0x120, &args![flag]);
    }
}

// Translated from 0087faa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::EndMovement` (Xbox PDB): with a process, its virtual `+0x294` with
/// the actor.
pub fn actor_end_movement(e: &mut Engine, this: Ptr<Actor>) {
    let process = e.get(this, Actor::pCurrentProcess);
    if !process.is_null() {
        e.vcall(process.addr(), 0x294, &args![this]);
    }
}

// Translated from 0087fad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes from the disposition-modifier list the entry for `target`
/// (`DispositionModifier::pactormodified`), if there is one.
pub fn fn_0087fad0(e: &mut Engine, this: Ptr<Actor>, target: u32) {
    let item = find_disposition_modifier(e, this, target);
    if item != 0 {
        let list = this.at(Actor::DispModifierList);
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), item);
            e.call(LIST_REMOVE_ITEM, &args![list, slot]);
        });
    }
}

// Translated from 0087fb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `amount` to the disposition modifier the actor keeps toward `target`,
/// only when `target` is the player. Sets the actor's flag bit `0x80000`
/// (virtual `+0x48`). When the actor already has an entry for the player,
/// the amount is first limited so that the actor's disposition (virtual
/// `+0x344` with the player and 0) plus the amount stays within `0 ..= 100`
/// (a sum below 0 turns the amount into `-disposition`; a sum above 100
/// keeps only the part up to 100, or 0), then it is added to the entry.
/// Otherwise a new entry `{ amount, target }` is allocated and appended
/// (`00564db0(target, 1)` is called on the player first).
pub fn fn_0087fb40(e: &mut Engine, this: Ptr<Actor>, target: Ptr, amount: f32) {
    let player = e.global::<u32>(PLAYER_POINTER);
    if target.addr() != player {
        return;
    }
    let mut amount = amount;
    e.vcall(this.addr(), 0x48, &args![0x80000u32]);
    let entry = find_disposition_modifier(e, this, target.addr());
    if entry != 0 {
        let entry: Ptr<DispositionModifier> = Ptr::new(entry);
        let disposition = e.vcall(this.addr(), 0x344, &args![target, 0u32]).i32();
        let sum = disposition as f64 + amount as f64;
        let zero: f64 = e.global(0x0101_2060);
        let hundred: f64 = e.global(0x0101_7a40);
        if sum < zero {
            amount = (amount as f64 - sum) as f32;
        } else if sum > hundred {
            let total = float_to_int(e, amount as f64).wrapping_add(disposition);
            let excess = total.wrapping_sub(100);
            if amount as f64 > excess as f64 {
                amount = (amount as f64 - excess as f64) as f32;
            } else {
                amount = 0.0;
            }
        }
        let delta = float_to_int(e, amount as f64);
        let current = e.get(entry, DispositionModifier::modifieramount);
        e.set(
            entry,
            DispositionModifier::modifieramount,
            current.wrapping_add(delta),
        );
    } else {
        let item = e.call(OPERATOR_NEW, &args![8u32]).u32();
        let delta = float_to_int(e, amount as f64);
        let item: Ptr<DispositionModifier> = Ptr::new(item);
        e.set(item, DispositionModifier::modifieramount, delta);
        e.set(item, DispositionModifier::pactormodified, target);
        e.call(0x0056_4db0, &args![target, 1u32]);
        let list = this.at(Actor::DispModifierList);
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), item.addr());
            e.call(LIST_APPEND_ITEM, &args![list, slot]);
        });
    }
}

// Translated from 0087fcb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The disposition modifier the actor keeps toward `target` as a `float`, 0
/// when it has none.
pub fn fn_0087fcb0(e: &mut Engine, this: Ptr<Actor>, target: u32) -> f32 {
    let entry = find_disposition_modifier(e, this, target);
    if entry != 0 {
        e.mem.i32(entry) as f32
    } else {
        0.0
    }
}

// Translated from 0087fd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::ClearDispositionModifiers` (Xbox PDB): frees every entry of the
/// disposition-modifier list, then empties the list (`00470470`).
pub fn actor_clear_disposition_modifiers(e: &mut Engine, this: Ptr<Actor>) {
    let list = this.at(Actor::DispModifierList);
    let mut node = list.cast::<()>();
    while !node.is_null() {
        let slot = e.call(LIST_NODE_ITEM_SLOT, &args![node]).u32();
        let item = e.mem.u32(slot);
        if item == 0 {
            break;
        }
        e.call(OPERATOR_DELETE, &args![item]);
        node = e.call(LIST_NODE_NEXT, &args![node]).ptr();
    }
    e.call(LIST_CLEAR, &args![list]);
}

// Translated from 0087fd90 (decompiled, FalloutNV.exe 1.4.0.525)
/// A score of the actor towards `other`, clamped to `0 ..= 100` (it looks
/// like the actor's disposition towards `other`; the virtual `+0x344` the
/// modifier code above calls with `(other, 0)` has this shape). Returns 0
/// without `other` and 100 at once when `other` is the actor's own target
/// form. `cached` (may be null) is a block of six words that stands in for the
/// values the function otherwise reads from the actor: `+0`, `+4`, `+8`
/// (the target form), `+0xc`, `+0x10` and `+0x14`.
///
/// Two paths follow: the first (when the actor is not a creature-like
/// actor, virtual `+0x218` false, and it has an owner reference or a saved
/// acquire object, or `00608d80` is true) and the second both read the
/// values of `other`'s actor-value owner and combine them with
/// `00642a60`; the result goes through `BGSEntryPoint::HandleEntryPoint`
/// (`005e58f0`, entry point `0xe`) and is clamped. `00642a60` is `cdecl`
/// with eleven words (`ADD ESP,0x2c`): the flags word, `0066ef20(owner, 8)`,
/// the skill word, the owner's virtual `+8` of `0x17`, the `0048bf50` value,
/// the `008b7fe0` result, its first out word, the word built from
/// `008a16d0`, a local that stays 0, the truncated virtual `+0x464` value and the
/// actor's virtual `+0x21c` byte.
pub fn fn_0087fd90(e: &mut Engine, this: Ptr<Actor>, other: Ptr<Actor>, cached: Ptr) -> i32 {
    let mut result: i32 = 0;
    if other.is_null() {
        return result;
    }
    let mut saved_acquire = 0u32;
    let cached_37c;
    let virtual_37c = e.vcall(this.addr(), 0x37c, &args![]).u32();
    let mut target_form = 0u32;
    let base_of_this;
    let flags_word;
    let skill_word;
    let owner_reference;
    let mut other_base = base_form(e, other).addr();
    if other_base != 0 {
        let component = e.call(0x005d_8a70, &args![other_base + 0x30]).u32();
        if e.call(0x0082_56d0, &args![component]).bool() {
            other_base = base_form(e, other).addr();
        }
    }
    if !cached.is_null() {
        cached_37c = e.mem.u32(cached.addr());
        owner_reference = e.mem.u32(cached.addr() + 0xc);
        flags_word = e.mem.u32(cached.addr() + 0x10);
        skill_word = e.mem.u32(cached.addr() + 0x14);
        target_form = e.mem.u32(cached.addr() + 8);
        base_of_this = e.mem.u32(cached.addr() + 4);
    } else {
        cached_37c = e.vcall(this.addr(), 0x37c, &args![]).u32();
        owner_reference = e.call(0x0056_7790, &args![this]).u32();
        base_of_this = base_form(e, this).addr();
        flags_word = e.call(0x0047_d3d0, &args![base_of_this + 0x30]).u16() as u32;
        skill_word = e
            .vcall(this.addr() + ACTOR_VALUE_OWNER, 8, &args![3u32])
            .u32();
        if e.vcall(this.addr(), 0x218, &args![]).bool() {
            target_form = e.call(GET_FORM, &args![this]).u32();
        }
    }
    let process = actor_process(e, this);
    if !process.is_null() {
        saved_acquire = e.vcall(process.addr(), 0x52c, &args![]).u32();
    }
    if e.call(GET_FORM, &args![other]).u32() == target_form {
        return 100;
    }
    let creature_like = e.vcall(this.addr(), 0x218, &args![]).bool();
    let first_path = (!creature_like && (owner_reference != 0 || saved_acquire != 0))
        || e.call(0x0060_8d80, &args![this]).bool();
    if first_path {
        if target_form == 0 {
            if owner_reference != 0 && e.call(FORM_TYPE, &args![owner_reference]).u32() == 0x2a {
                target_form = owner_reference;
            }
            if target_form == 0 && saved_acquire != 0 {
                let form = e.call(GET_FORM, &args![saved_acquire]).u32();
                if e.call(FORM_TYPE, &args![form]).u32() == 0x2a {
                    target_form = e.call(GET_FORM, &args![saved_acquire]).u32();
                }
            }
        }
        if target_form != 0 && e.call(GET_FORM, &args![other]).u32() == target_form {
            return 100;
        }
        if target_form != 0 && other_base != 0 {
            // Locals the compiler keeps (an unused copy of `other_base` among them).
            let mut cached_value = 0u32;
            if e.vcall(other.addr(), 0x218, &args![]).bool() {
                cached_value = e
                    .call(0x0048_bf50, &args![cached_37c + 0x40, virtual_37c])
                    .u32();
            }
            let (outcome, out_first) = e.with_stack(12, |e, outs| {
                e.mem.set_u32(outs.addr(), 0xffff_ffff);
                e.mem.set_u32(outs.addr() + 4, 0);
                e.mem.set_u32(outs.addr() + 8, 0);
                let outcome = e
                    .call(
                        0x008b_7fe0,
                        &args![this, other, outs, outs.byte_add(4), outs.byte_add(8)],
                    )
                    .u32();
                (outcome, e.mem.u32(outs.addr()))
            });
            let mut hostile = 0u32;
            if e.call(0x008a_16d0, &args![other]).bool() {
                if !e.call(0x0049_3bb0, &args![other]).bool() {
                    hostile = 1;
                } else {
                    let combat_target = e.vcall(this.addr(), 0x428, &args![]).u32();
                    if combat_target != 0 && e.call(0x0097_fa10, &args![combat_target, this]).bool()
                    {
                        hostile = 1;
                    }
                }
            }
            let player = e.global::<u32>(PLAYER_POINTER);
            let actor_in_high = if target_form == e.call(GET_FORM, &args![player]).u32() {
                player
            } else {
                e.call(0x0097_0a20, &args![PROCESS_LISTS, target_form, 0u32])
                    .u32()
            };
            let mut other_value = 0i32;
            if actor_in_high != 0 {
                let value = e.vcall(actor_in_high, 0x464, &args![other]).f64();
                other_value = float_to_int(e, value);
            }
            // [EBP-0x3c]: set to 0 at 008800e1 and never written again.
            let zero_word = 0u32;
            let flag_21c = e.vcall(this.addr(), 0x21c, &args![]).u8();
            // 0088015e..008801aa: the owner's virtual +8 takes only 0x17; the
            // words pushed before it are 00642a60's arguments 5..11.
            let owner_value = e
                .vcall(other.addr() + ACTOR_VALUE_OWNER, 8, &args![0x17u32])
                .u32();
            let clamped = e
                .call(0x0066_ef20, &args![other.addr() + ACTOR_VALUE_OWNER, 8u32])
                .u32();
            result = e
                .call(
                    0x0064_2a60,
                    &args![
                        flags_word,
                        clamped,
                        skill_word,
                        owner_value,
                        cached_value,
                        outcome,
                        out_first,
                        hostile,
                        zero_word,
                        other_value,
                        flag_21c as u32
                    ],
                )
                .i32();
        }
    } else {
        if target_form == 0
            && owner_reference != 0
            && e.call(FORM_TYPE, &args![owner_reference]).u32() == 0x2a
        {
            target_form = owner_reference;
        }
        if target_form != 0 && e.call(GET_FORM, &args![other]).u32() == target_form {
            return 100;
        }
        let other_base_again = base_form(e, other).addr();
        if other_base_again != 0 && base_of_this != 0 {
            let mut cached_value = 0u32;
            if cached_37c != 0 && virtual_37c != 0 {
                cached_value = e
                    .call(0x0048_bf50, &args![cached_37c + 0x40, virtual_37c])
                    .u32();
            }
            let (outcome, out_first) = e.with_stack(12, |e, outs| {
                e.mem.set_u32(outs.addr(), 0xffff_ffff);
                e.mem.set_u32(outs.addr() + 4, 0);
                e.mem.set_u32(outs.addr() + 8, 0);
                let outcome = e
                    .call(
                        0x008b_7fe0,
                        &args![this, other, outs, outs.byte_add(4), outs.byte_add(8)],
                    )
                    .u32();
                (outcome, e.mem.u32(outs.addr()))
            });
            let weapon_drawn = e.call(0x008a_16d0, &args![other]).u8();
            let value = e.vcall(this.addr(), 0x464, &args![other]).f64();
            let other_value = float_to_int(e, value);
            // [EBP-0x6c]: set to 0 at 00880281 and never written again.
            let zero_word = 0u32;
            let flag_21c = e.vcall(this.addr(), 0x21c, &args![]).u8();
            // 008802c3..0088030f: as on the first path, the words pushed before
            // the owner's virtual +8 (which takes only 0x17) are 00642a60's
            // arguments 5..11.
            let owner_value = e
                .vcall(other.addr() + ACTOR_VALUE_OWNER, 8, &args![0x17u32])
                .u32();
            let clamped = e
                .call(0x0066_ef20, &args![other.addr() + ACTOR_VALUE_OWNER, 8u32])
                .u32();
            result = e
                .call(
                    0x0064_2a60,
                    &args![
                        flags_word,
                        clamped,
                        skill_word,
                        owner_value,
                        cached_value,
                        outcome,
                        out_first,
                        weapon_drawn as u32,
                        zero_word,
                        other_value,
                        flag_21c as u32
                    ],
                )
                .i32();
        }
    }
    let mut score = result as f32;
    e.with_stack(4, |e, cell| {
        e.mem.set_f32(cell.addr(), score);
        e.call(0x005e_58f0, &args![0xeu32, this, other, cell]);
        score = e.mem.f32(cell.addr());
    });
    result = float_to_int(e, score as f64);
    result.clamp(0, 100)
}

// Translated from 00880370 (decompiled, FalloutNV.exe 1.4.0.525)
/// On the actor-value owner `this` (the sub-object at `Actor + 0xa4`): the
/// virtual `+4` value of actor value `index`, passed through `00404040` and
/// truncated.
pub fn fn_00880370(e: &mut Engine, this: Ptr, index: u32) -> i32 {
    let value = e.vcall(this.addr(), 4, &args![index]).f32();
    let adjusted = e.call(0x0040_4040, &args![value]).f64();
    float_to_int(e, adjusted)
}

// Translated from 008803a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// On the actor-value owner `this` (the sub-object at `Actor + 0xa4`): the
/// value of actor value `index` once the modifiers apply. The base value is
/// the integer result of the base form component's (`+0x100`) virtual `+8`;
/// the actor's virtual `+0x48c` gives the modifier and a flag. Which of the
/// two (or their sum) is returned depends on `0047f060`, the actor-value flag
/// words (`00406d70` with `0x1000`, `0x800`, `0x80`, `0x40`), the actor's
/// virtual `+0x21c` and `+0x360`, `00566950`, and the result of `0066ed60`.
pub fn fn_008803a0(e: &mut Engine, this: Ptr, index: u32) -> f32 {
    let actor: Ptr<Actor> = Ptr::new(this.addr().wrapping_sub(ACTOR_VALUE_OWNER));
    let component = base_form(e, actor).addr() + 0x100;
    let mut base_value = e.vcall(component, 8, &args![index]).i32() as f32;
    let (modifier, flag) = e.with_stack(4, |e, flag| {
        e.mem.set_u8(flag.addr(), 0);
        let modifier = e.vcall(actor.addr(), 0x48c, &args![index, flag]).f32();
        (modifier, e.mem.u8(flag.addr()) != 0)
    });
    if flag && !e.call(ACTOR_VALUE_IS_STAT, &args![index]).bool() {
        return modifier;
    }
    if e.vcall(actor.addr(), 0x21c, &args![]).bool()
        && e.call(ACTOR_VALUE_HAS_FLAG, &args![index, 0x1000u32])
            .bool()
    {
        return base_value;
    }
    let combine = e.call(ACTOR_VALUE_HAS_FLAG, &args![index, 0x800u32]).bool()
        || e.vcall(actor.addr(), 0x360, &args![]).bool()
        || e.call(ACTOR_VALUE_HAS_FLAG, &args![index, 0x80u32]).bool()
        || (e.call(0x0056_6950, &args![actor]).bool() && index == 0x10);
    if !combine {
        return base_value;
    }
    if flag && e.call(ACTOR_VALUE_IS_STAT, &args![index]).bool() {
        base_value = modifier;
    }
    // 00880509..00880523: the owner handed on is `this` itself (the `ActorValueOwner` part),
    // or 0 when the actor it belongs to is null.
    let owner = if actor.addr() == 0 { 0 } else { this.addr() };
    let (accepted, result) = e.with_stack(4, |e, cell| {
        e.mem.set_f32(cell.addr(), 0.0);
        let accepted = e.call(0x0066_ed60, &args![owner, index, cell]).bool();
        (accepted, e.mem.f32(cell.addr()))
    });
    if !accepted {
        return base_value;
    }
    if e.call(ACTOR_VALUE_HAS_FLAG, &args![index, 0x40u32]).bool() || flag {
        return (result as f64 + base_value as f64) as f32;
    }
    result
}

// Translated from 00880580 (decompiled, FalloutNV.exe 1.4.0.525)
/// On the actor-value owner `this` (the sub-object at `Actor + 0xa4`): with a
/// process, the process's virtual `+0x398` called with (the actor's base
/// form, `index`, the actor); without, the owner's own virtual `+0` with
/// `index`.
pub fn fn_00880580(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let actor: Ptr<Actor> = Ptr::new(this.addr().wrapping_sub(ACTOR_VALUE_OWNER));
    if !e.get(actor, Actor::pCurrentProcess).is_null() {
        let process = actor_process(e, actor);
        let base = base_form(e, actor);
        e.vcall(process.addr(), 0x398, &args![base, index, actor])
            .u32()
    } else {
        e.vcall(this.addr(), 0, &args![index]).u32()
    }
}

// Translated from 008805f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `float` counterpart of `00880580`: with a process, its virtual `+0x39c`
/// result; without, the owner's virtual `+0` (an integer) as a `float`.
pub fn fn_008805f0(e: &mut Engine, this: Ptr, index: u32) -> f32 {
    let actor: Ptr<Actor> = Ptr::new(this.addr().wrapping_sub(ACTOR_VALUE_OWNER));
    if !e.get(actor, Actor::pCurrentProcess).is_null() {
        let process = actor_process(e, actor);
        let base = base_form(e, actor);
        e.vcall(process.addr(), 0x39c, &args![base, index, actor])
            .f32()
    } else {
        e.vcall(this.addr(), 0, &args![index]).i32() as f32
    }
}

// Translated from 00880660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `009376e0` (the modifier list at `+0xe0`) with (`kind`, `value`), as a `float`.
pub fn fn_00880660(e: &mut Engine, this: Ptr<Actor>, kind: u8, value: u32) -> f32 {
    e.call(0x0093_76e0, &args![this.byte_add(0xe0), kind as u32, value])
        .f32()
}

// Translated from 00880690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `009375e0` (the modifier list at `+0xe0`) with (`kind`, `amount`), then sets
/// the actor's flag bit `0x400000` (virtual `+0x48`).
pub fn fn_00880690(e: &mut Engine, this: Ptr<Actor>, kind: u8, amount: f32) {
    e.call(
        0x0093_75e0,
        &args![this.byte_add(0xe0), kind as u32, amount],
    );
    e.vcall(this.addr(), 0x48, &args![0x0040_0000u32]);
}

// Translated from 008806d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The actor's virtual `+0x394` with (`index`, `value` as a `float`).
pub fn fn_008806d0(e: &mut Engine, this: Ptr<Actor>, index: u32, value: i32) {
    e.vcall(this.addr(), 0x394, &args![index, value as f32]);
}

// Translated from 00880700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Changes actor value `index` by `amount` (a `float`) for the actor: the
/// base value is read first (`0` unless `0066ee10`). With the flag `0x100`
/// (`00406d70`) the amount is clamped: a negative amount becomes 0 and,
/// otherwise, an amount not below the maximum of `0066e920`'s record
/// (`+0x98`) becomes that maximum minus 1. Then the actor's virtual `+0x490`
/// is called with (`index`, amount), the process's virtual `+0x3b0` with
/// `index` when the actor's virtual `+0x360` is false and it has a process,
/// and `0066ee50` reports the change.
pub fn fn_00880700(e: &mut Engine, this: Ptr<Actor>, index: u32, amount: f32) {
    let base = base_actor_value(e, this, index);
    let mut amount = amount;
    if e.call(ACTOR_VALUE_HAS_FLAG, &args![index, 0x100u32]).bool() {
        let zero: f64 = e.global(0x0101_2060);
        if (amount as f64) < zero {
            amount = 0.0;
        } else {
            let record = e.call(0x0066_e920, &args![index]).u32();
            if record != 0 {
                let maximum = e.mem.i32(record + 0x98) as f64;
                if amount as f64 >= maximum {
                    let one: f64 = e.global(0x0101_2070);
                    amount = (maximum - one) as f32;
                }
            }
        }
    }
    e.vcall(this.addr(), 0x490, &args![index, amount]);
    if !e.vcall(this.addr(), 0x360, &args![]).bool() && !actor_process(e, this).is_null() {
        let process = actor_process(e, this);
        e.vcall(process.addr(), 0x3b0, &args![index]);
    }
    e.call(
        ACTOR_VALUE_CHANGED,
        &args![owner_of(this.addr()), index, base, amount, 0u32],
    );
}

// Translated from 00880850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `00880890` on (`index`, `value` as a `float`, `source`), passed through
/// `00404040` and truncated.
pub fn fn_00880850(e: &mut Engine, this: Ptr, index: u32, value: i32, source: u32) -> i32 {
    let adjusted = fn_00880890(e, this, index, value as f32, source);
    let rounded = e.call(0x0040_4040, &args![adjusted]).f64();
    float_to_int(e, rounded)
}

// Translated from 00880890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns its second word, a `float`, unchanged (the first and third words
/// are never read).
pub fn fn_00880890(_e: &mut Engine, _this: Ptr, _unused_0: u32, value: f32, _unused_2: u32) -> f32 {
    value
}

// Translated from 008808a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::DifficultyLevelAdjustHealthModifier` (Xbox PDB): `modifier` times
/// `00648cb0(difficulty, 0x10, actor's virtual +0x360)`, where the difficulty
/// is the player's `+0x7b8` (`005be4d0`). The second word is never read.
pub fn actor_difficulty_level_adjust_health_modifier(
    e: &mut Engine,
    this: Ptr<Actor>,
    modifier: f32,
    _unused_1: u32,
) -> f32 {
    let flag = e.vcall(this.addr(), 0x360, &args![]).bool();
    let player = e.global::<u32>(PLAYER_POINTER);
    let difficulty = e.call(0x005b_e4d0, &args![player]).u32();
    let factor = e.call(0x0064_8cb0, &args![difficulty, 0x10u32, flag]).f32();
    (modifier as f64 * factor as f64) as f32
}

// Translated from 008808f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `0047f060(index)`; the second word is never read.
pub fn fn_008808f0(e: &mut Engine, _this: Ptr, index: u32, _unused_1: u32) -> u32 {
    e.call(ACTOR_VALUE_IS_STAT, &args![index]).u32()
}

// Translated from 00880910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `004bfc80` on the container changes (`00418520`) of the reference's extra
/// data list; false when it has none.
pub fn fn_00880910(e: &mut Engine, this: Ptr<Actor>) -> bool {
    let mut result = false;
    let list = extra_data_list(e, this);
    let changes = e.call(0x0041_8520, &args![list]).u32();
    if changes != 0 {
        result = e.call(0x004b_fc80, &args![changes]).bool();
    }
    result
}

// Translated from 00880950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Changes actor value `index` by the integer `value` (first adjusted by
/// `00880850`). Does nothing for index `0x16` with a negative value unless the
/// actor's virtual `+0x38c` is true, nor for values with the flag `0x100`.
/// Calls the process's virtual `+0x3a4` with (actor, `index`, value), for
/// index `0x10` with a negative value the actor's virtual `+0x4b8` with
/// (`source`, value as a `float`), sets the actor's flag bit `0x100000`
/// (virtual `+0x48`), and reports the change with `0066ee50`.
pub fn fn_00880950(e: &mut Engine, this: Ptr<Actor>, index: u32, value: i32, source: Ptr<Actor>) {
    if index == 0x16 && value < 0 && !e.vcall(this.addr(), 0x38c, &args![]).bool() {
        return;
    }
    if e.call(ACTOR_VALUE_HAS_FLAG, &args![index, 0x100u32]).bool() {
        return;
    }
    let value = fn_00880850(e, this.cast(), index, value, source.addr());
    let base = base_actor_value(e, this, index);
    if !e.get(this, Actor::pCurrentProcess).is_null() {
        let process = actor_process(e, this);
        e.vcall(process.addr(), 0x3a4, &args![this, index, value]);
    }
    if index == 0x10 && value < 0 {
        e.vcall(this.addr(), 0x4b8, &args![source, value as f32]);
    }
    e.vcall(this.addr(), 0x48, &args![0x0010_0000u32]);
    fn_008808f0(e, this.cast(), index, 0);
    report_actor_value_change(e, this, index, base, value as f32, source.addr());
}

// Translated from 00880ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `float` counterpart of `00880950` (adjusted by `00880890`; process
/// virtual `+0x3a0`; the negative test is `amount < 0.0`).
pub fn fn_00880ad0(e: &mut Engine, this: Ptr<Actor>, index: u32, amount: f32, source: Ptr<Actor>) {
    let zero: f64 = e.global(0x0101_2060);
    if index == 0x16 && (amount as f64) < zero && !e.vcall(this.addr(), 0x38c, &args![]).bool() {
        return;
    }
    if e.call(ACTOR_VALUE_HAS_FLAG, &args![index, 0x100u32]).bool() {
        return;
    }
    let amount = fn_00880890(e, this.cast(), index, amount, source.addr());
    let base = base_actor_value(e, this, index);
    if !e.get(this, Actor::pCurrentProcess).is_null() {
        let process = actor_process(e, this);
        e.vcall(process.addr(), 0x3a0, &args![this, index, amount]);
    }
    if index == 0x10 && (amount as f64) < zero {
        e.vcall(this.addr(), 0x4b8, &args![source, amount]);
    }
    e.vcall(this.addr(), 0x48, &args![0x0010_0000u32]);
    fn_008808f0(e, this.cast(), index, 0);
    report_actor_value_change(e, this, index, base, amount, source.addr());
}

// Translated from 00880c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like `00880950`, but the change goes to the modifier list at `+0xd0`
/// (`00937480(kind = index, value as a float, 2)`), the flag bit set is
/// `0x800000` and the process call is its virtual `+0x3b0` with `index`
/// (after the flag bit, with no `0x3a4` call before).
pub fn fn_00880c70(e: &mut Engine, this: Ptr<Actor>, index: u32, value: i32, source: Ptr<Actor>) {
    if index == 0x16 && value < 0 && !e.vcall(this.addr(), 0x38c, &args![]).bool() {
        return;
    }
    if e.call(ACTOR_VALUE_HAS_FLAG, &args![index, 0x100u32]).bool() {
        return;
    }
    let value = fn_00880850(e, this.cast(), index, value, source.addr());
    let base = base_actor_value(e, this, index);
    e.call(
        0x0093_7480,
        &args![this.byte_add(0xd0), index as u8 as u32, value as f32, 2u32],
    );
    if index == 0x10 && value < 0 {
        e.vcall(this.addr(), 0x4b8, &args![source, value as f32]);
    }
    e.vcall(this.addr(), 0x48, &args![0x0080_0000u32]);
    fn_008808f0(e, this.cast(), index, 0);
    if !e.get(this, Actor::pCurrentProcess).is_null() {
        let process = actor_process(e, this);
        e.vcall(process.addr(), 0x3b0, &args![index]);
    }
    report_actor_value_change(e, this, index, base, value as f32, source.addr());
}

// Translated from 00880e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `float` counterpart of `00880c70` (adjusted by `00880890`).
pub fn fn_00880e00(e: &mut Engine, this: Ptr<Actor>, index: u32, amount: f32, source: Ptr<Actor>) {
    let zero: f64 = e.global(0x0101_2060);
    if index == 0x16 && (amount as f64) < zero && !e.vcall(this.addr(), 0x38c, &args![]).bool() {
        return;
    }
    if e.call(ACTOR_VALUE_HAS_FLAG, &args![index, 0x100u32]).bool() {
        return;
    }
    let amount = fn_00880890(e, this.cast(), index, amount, source.addr());
    let base = base_actor_value(e, this, index);
    e.call(
        0x0093_7480,
        &args![this.byte_add(0xd0), index as u8 as u32, amount, 2u32],
    );
    if index == 0x10 && (amount as f64) < zero {
        e.vcall(this.addr(), 0x4b8, &args![source, amount]);
    }
    e.vcall(this.addr(), 0x48, &args![0x0080_0000u32]);
    fn_008808f0(e, this.cast(), index, 0);
    if !e.get(this, Actor::pCurrentProcess).is_null() {
        let process = actor_process(e, this);
        e.vcall(process.addr(), 0x3b0, &args![index]);
    }
    report_actor_value_change(e, this, index, base, amount, source.addr());
}

// Translated from 00880fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like `00880950` with the process's virtual `+0x3ac` and the flag bit `0x200000`.
pub fn fn_00880fb0(e: &mut Engine, this: Ptr<Actor>, index: u32, value: i32, source: Ptr<Actor>) {
    if index == 0x16 && value < 0 && !e.vcall(this.addr(), 0x38c, &args![]).bool() {
        return;
    }
    if e.call(ACTOR_VALUE_HAS_FLAG, &args![index, 0x100u32]).bool() {
        return;
    }
    let value = fn_00880850(e, this.cast(), index, value, source.addr());
    let base = base_actor_value(e, this, index);
    if !e.get(this, Actor::pCurrentProcess).is_null() {
        let process = actor_process(e, this);
        e.vcall(process.addr(), 0x3ac, &args![this, index, value]);
    }
    if index == 0x10 && value < 0 {
        e.vcall(this.addr(), 0x4b8, &args![source, value as f32]);
    }
    e.vcall(this.addr(), 0x48, &args![0x0020_0000u32]);
    fn_008808f0(e, this.cast(), index, 0);
    report_actor_value_change(e, this, index, base, value as f32, source.addr());
}

// Translated from 00881130 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `float` counterpart of `00880fb0` (adjusted by `00880890`; process
/// virtual `+0x3a8`). For index `0x16` with a negative amount the change is
/// also bounded by the float setting at `011d2664` (read through `00403e20`):
/// nothing happens unless the setting is below the base value, and an amount
/// that would take base plus amount to the setting or below becomes setting
/// minus base.
pub fn fn_00881130(e: &mut Engine, this: Ptr<Actor>, index: u32, amount: f32, source: Ptr<Actor>) {
    let zero: f64 = e.global(0x0101_2060);
    if index == 0x16 && (amount as f64) < zero && !e.vcall(this.addr(), 0x38c, &args![]).bool() {
        return;
    }
    if e.call(ACTOR_VALUE_HAS_FLAG, &args![index, 0x100u32]).bool() {
        return;
    }
    let mut amount = fn_00880890(e, this.cast(), index, amount, source.addr());
    let base = base_actor_value(e, this, index);
    if index == 0x16 && (amount as f64) < zero {
        let limit_slot = e.call(0x0040_3e20, &args![0x011d_2664u32]).u32();
        let limit = e.mem.f32(limit_slot);
        if limit as f64 >= base as f64 {
            return;
        }
        let sum = (base as f64 + amount as f64) as f32;
        let limit_slot = e.call(0x0040_3e20, &args![0x011d_2664u32]).u32();
        let limit = e.mem.f32(limit_slot);
        if limit as f64 > sum as f64 {
            let limit_slot = e.call(0x0040_3e20, &args![0x011d_2664u32]).u32();
            let limit = e.mem.f32(limit_slot);
            amount = (limit as f64 - base as f64) as f32;
        }
    }
    if !e.get(this, Actor::pCurrentProcess).is_null() {
        let process = actor_process(e, this);
        e.vcall(process.addr(), 0x3a8, &args![this, index, amount]);
    }
    if index == 0x10 && (amount as f64) < zero {
        e.vcall(this.addr(), 0x4b8, &args![source, amount]);
    }
    e.vcall(this.addr(), 0x48, &args![0x0020_0000u32]);
    fn_008808f0(e, this.cast(), index, 0);
    report_actor_value_change(e, this, index, base, amount, source.addr());
}

// Translated from 00881330 (decompiled, FalloutNV.exe 1.4.0.525)
/// The actor's virtual `+0x3b4` with (`index`, `value` as a `float`).
pub fn fn_00881330(e: &mut Engine, this: Ptr<Actor>, index: u32, value: i32) {
    e.vcall(this.addr(), 0x3b4, &args![index, value as f32]);
}

// Translated from 00881360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `amount` to the current value of actor value `index` (the owner's
/// virtual `+4`) through the actor's virtual `+0x394`, then reports the
/// change (`0066ee50`) with the base value read before; does nothing for
/// values with the flag `0x100`.
pub fn fn_00881360(e: &mut Engine, this: Ptr<Actor>, index: u32, amount: f32) {
    if e.call(ACTOR_VALUE_HAS_FLAG, &args![index, 0x100u32]).bool() {
        return;
    }
    let base = base_actor_value(e, this, index);
    let current = e
        .vcall(this.addr() + ACTOR_VALUE_OWNER, 4, &args![index])
        .f32();
    let new_value = (current as f64 + amount as f64) as f32;
    e.vcall(this.addr(), 0x394, &args![index, new_value]);
    fn_008808f0(e, this.cast(), index, 0);
    e.call(
        ACTOR_VALUE_CHANGED,
        &args![owner_of(this.addr()), index, base, amount, 0u32],
    );
}

// ---------------------------------------------------------------------------
// Block 3 (batch b0148, continued): `00881450` up to `00884970`
//
// The three process transitions (`00881d30`, `00882b90`, `00883240`,
// `00883800`) carry C++ exception frames in the exe; the unwinding is not
// translated.

/// `TESAIForm::GetCurrentPackage(reference)` (decompiler name, cdecl).
const REFERENCE_CURRENT_PACKAGE: u32 = 0x0047_f520;
/// `TESAIForm::GetMissedPackages(reference, a, b, hour)` (decompiler name, cdecl).
const REFERENCE_MISSED_PACKAGES: u32 = 0x0047_f590;
/// The calendar singleton (`ECX` of the two calendar getters below).
const CALENDAR_OBJECT: u32 = 0x011d_e7b8;
/// `Calendar::GetHour` (Xbox PDB), result in `ST0`.
const CALENDAR_GET_HOUR: u32 = 0x0086_7da0;
/// The calendar function whose word result `00881d10` stores as the actor's
/// last-seen time.
const CALENDAR_TIME_STAMP: u32 = 0x0086_7e30;
/// `TESPackage::IsInterruptPackage` (decompiler name).
const IS_INTERRUPT_PACKAGE: u32 = 0x0067_8610;
/// `ExtraDataList::GetPackageExtra` (decompiler name): the package an
/// `ExtraDataList` holds.
const EXTRA_PACKAGE: u32 = 0x0041_cb10;
/// `Actor::IsFleeing(flag)` (decompiler name).
const IS_FLEEING: u32 = 0x008a_6650;
/// Returns the word at `+8` of its `this` (`0044ddc0`), here `process + 4`.
const WORD_AT_8: u32 = 0x0044_ddc0;
/// The level of a process (`0045cd60`): 0 high, 1 middle high, 2 middle low, 3 low
/// (the order in which the four transitions below use it).
const PROCESS_LEVEL: u32 = 0x0045_cd60;
/// `MobileObject::GetCurrentProcessType` (Xbox PDB), on the actor.
const CURRENT_PROCESS_TYPE: u32 = 0x0093_1850;
/// `MobileObject::GetCurrentPackage` (Xbox PDB), on the actor.
const MOBILE_OBJECT_PACKAGE: u32 = 0x0093_44a0;
/// `Actor::SetLifeState(state)` (Xbox PDB).
const SET_LIFE_STATE: u32 = 0x008a_1800;
/// `Actor::RestoreFullHealthAndConditions` (decompiler name).
const RESTORE_FULL_HEALTH: u32 = 0x008a_0960;
/// `Actor::IsAlarmed` (decompiler name).
const IS_ALARMED: u32 = 0x008a_61b0;
/// `Actor::IsContinuingPackageforPC` (decompiler name).
const IS_CONTINUING_PACKAGE_FOR_PC: u32 = 0x008a_69d0;
/// `Actor::UnlockLockDoorsProcedure` (Xbox PDB).
const UNLOCK_LOCK_DOORS_PROCEDURE: u32 = 0x008a_6a40;
/// Takes the actor, a process level and a flag (`008a0680`).
const MOVE_TO_PROCESS_LEVEL: u32 = 0x008a_0680;
/// `Actor::UpdateAlpha` (Xbox PDB).
const UPDATE_ALPHA: u32 = 0x008c_4640;
/// `Actor::CastPermanentMagic(flag)` (Xbox PDB).
const CAST_PERMANENT_MAGIC: u32 = 0x008c_26e0;
/// `Actor::DoDeathStuff` (decompiler name).
const DO_DEATH_STUFF: u32 = 0x008b_01c0;
/// `TESObjectREFR::RunScript` (decompiler name).
const RUN_SCRIPT: u32 = 0x0056_5870;
/// A query on the actor whose result 6 means "dead" in the transitions (`004f8960`).
const ACTOR_STATE_QUERY: u32 = 0x004f_8960;
/// The parent cell of a reference (`008d6f30`).
const PARENT_CELL: u32 = 0x008d_6f30;
/// The world space of a reference (`00575d70`, decompiler name `GetWorldSpace`).
const WORLD_SPACE: u32 = 0x0057_5d70;
/// `ExtraDataList::GetContainerChanges` (Xbox PDB).
const GET_CONTAINER_CHANGES: u32 = 0x0041_8520;
/// `InventoryChanges::GetWornItem(slot, flag)` (Xbox PDB).
const GET_WORN_ITEM: u32 = 0x004c_8c10;
/// The reference's name text (`0055d520`).
const REFERENCE_NAME: u32 = 0x0055_d520;
/// `strstr(text, pattern)` (CRT, cdecl).
const STRSTR: u32 = 0x00ec_7750;
/// The text `"Lily"` the transitions look for in a reference's name.
const LILY_TEXT: u32 = 0x0102_fdc8;
/// `TESCreature::InitDefaultWorn(actor, a, b, c)` (Xbox PDB).
const INIT_DEFAULT_WORN: u32 = 0x005f_9e00;
/// The object `0040fbf0(flag)` and `0040fba0()` are called on.
const GUARD_OBJECT_011F11A0: u32 = 0x011f_11a0;
/// Enters the guard object with a flag.
const GUARD_ENTER: u32 = 0x0040_fbf0;
/// Leaves the guard object.
const GUARD_LEAVE: u32 = 0x0040_fba0;
/// A global pointer to an object tested with `0042ce10`.
const SHARED_OBJECT_POINTER_011DDF38: u32 = 0x011d_df38;
/// A global pointer to an object tested with `0047c850`.
const SHARED_OBJECT_POINTER_011DE45C: u32 = 0x011d_e45c;
/// `0042ce10`: a flag test of the object at [`SHARED_OBJECT_POINTER_011DDF38`].
const SHARED_OBJECT_TEST: u32 = 0x0042_ce10;
/// A test of the actor (`00576d30`).
const REFERENCE_TEST: u32 = 0x0057_6d30;
/// `0047c850`: the always-false function several actor slots point to.
const ALWAYS_FALSE: u32 = 0x0047_c850;
/// `004226e0`: tested on the object at [`SHUTDOWN_CHECK_OBJECT_POINTER`].
const SHUTDOWN_CHECK: u32 = 0x0042_26e0;
/// `ProcessLists::RemoveReference(actor, level)` (Xbox PDB).
const PROCESS_LISTS_REMOVE_REFERENCE: u32 = 0x0096_d470;
/// `ProcessLists::AddReference(actor, level, a, b, c)` (Xbox PDB).
const PROCESS_LISTS_ADD_REFERENCE: u32 = 0x0096_d450;
/// `(actor)` on the process lists (`0096e2f0`).
const PROCESS_LISTS_REMOVE_ACTOR: u32 = 0x0096_e2f0;
/// `ProcessLists::RemoveActorCloseToPlayer(actor)` (decompiler name).
const PROCESS_LISTS_REMOVE_CLOSE_TO_PLAYER: u32 = 0x0096_e6f0;
/// `(actor)` on the process lists (`00973cb0`).
const PROCESS_LISTS_REFRESH_ACTOR: u32 = 0x0097_3cb0;
/// `(actor)` on the process lists (`0096e2b0`).
const PROCESS_LISTS_NOTE_ACTOR: u32 = 0x0096_e2b0;
/// `BSShaderManager::GetAccumulator` (decompiler name).
const GET_ACCUMULATOR: u32 = 0x00b4_f5c0;
/// `(accumulator, value)` (`00b66050`).
const ACCUMULATOR_ADD: u32 = 0x00b6_6050;
/// The shader value of an actor (`0084e3a0`).
const ACTOR_SHADER_VALUE: u32 = 0x0084_e3a0;
/// `MagicTarget::Dispel` (decompiler name), on `actor + 0x94`.
const MAGIC_TARGET_DISPEL: u32 = 0x0082_4970;
/// `(actor)` (`008c2b60`).
const ACTOR_CLEAN_UP_A: u32 = 0x008c_2b60;
/// `(actor)` (`008c30a0`).
const ACTOR_CLEAN_UP_B: u32 = 0x008c_30a0;
/// `(process)` (`008e56b0`).
const PROCESS_RESET: u32 = 0x008e_56b0;
/// `(process, float)` (`00693d50`).
const PROCESS_SET_TIME: u32 = 0x0069_3d50;
/// `(actor)` (`00437bb0`).
const ACTOR_TEST_00437BB0: u32 = 0x0043_7bb0;
/// `HighProcess::HighProcess` (Xbox PDB), object size `0x46c`.
const HIGH_PROCESS_CONSTRUCTOR: u32 = 0x008d_7510;
/// Constructor of the low process (object size `0xb4`).
const LOW_PROCESS_CONSTRUCTOR: u32 = 0x0090_6dc0;
/// Constructor of the middle-low process (object size `0xc8`).
const MIDDLE_LOW_PROCESS_CONSTRUCTOR: u32 = 0x0092_c950;
/// Constructor of the middle-high process (object size `0x25c`).
const MIDDLE_HIGH_PROCESS_CONSTRUCTOR: u32 = 0x0091_3fe0;
/// `(actor)` (`00483710`).
const ACTOR_NOTE_FAILURE: u32 = 0x0048_3710;
/// `(process, actor)` (`008da1a0`).
const HIGH_PROCESS_SET_ACTOR: u32 = 0x008d_a1a0;
/// `(process, actor)` (`008e4e50`).
const HIGH_PROCESS_INIT_ACTOR: u32 = 0x008e_4e50;
/// `MiddleHighProcess::GetForceNextUpdate` (Xbox PDB), called on the actor.
const GET_FORCE_NEXT_UPDATE: u32 = 0x0056_6950;
/// A test of a package (`0067a460`).
const PACKAGE_TEST_0067A460: u32 = 0x0067_a460;
/// `TESPackage::GetIsCreated` (decompiler name).
const PACKAGE_GET_IS_CREATED: u32 = 0x0067_4d40;
/// `(cell, actor)` (`00546ca0`).
const CELL_TEST_ACTOR: u32 = 0x0054_6ca0;
/// `TESPackage::GetLocationReference(actor)` (decompiler name).
const PACKAGE_LOCATION_REFERENCE: u32 = 0x0067_6140;
/// `TESPackage::GetLocationCell(actor)` (decompiler name).
const PACKAGE_LOCATION_CELL: u32 = 0x0067_5c20;
/// `TESPackage::GetLocationCoord(out, actor)` (decompiler name).
const PACKAGE_LOCATION_COORD: u32 = 0x0067_5de0;
/// The package's location (`0055b980`).
const PACKAGE_LOCATION: u32 = 0x0055_b980;
/// `PackageLocation::GetLocType` (decompiler name).
const PACKAGE_LOCATION_TYPE: u32 = 0x0067_8ca0;
/// `(package, actor)` (`00676280`).
const PACKAGE_LOCATION_CHECK: u32 = 0x0067_6280;
/// A no-op "constructor" that returns its `this` (`006815c0`).
const POINT_CONSTRUCTOR: u32 = 0x0068_15c0;
/// The global the location form is compared with.
const LOCATION_FORM_POINTER_011CA248: u32 = 0x011c_a248;
/// Length of a `NiPoint3` (`00457990`), result in `ST0`.
const POINT_LENGTH: u32 = 0x0045_7990;
/// `NiPoint3` subtraction `this - other` into `out` (`00439ef0`).
const POINT_SUBTRACT: u32 = 0x0043_9ef0;
/// `NiPoint3` addition `this + other` into `out` (`00439e90`).
const POINT_ADD: u32 = 0x0043_9e90;
/// `NiPoint3` scaled by a float into `out` (`0045bb20`).
const POINT_SCALE: u32 = 0x0045_bb20;
/// Length of the horizontal vector (`00457910`), result in `ST0`.
const POINT_LENGTH_2D: u32 = 0x0045_7910;
/// Squared length of a `NiPoint3` (`004a7290`), result in `ST0`.
const POINT_LENGTH_SQUARED: u32 = 0x004a_7290;
/// `Actor::GetRadius` (Xbox PDB), result in `ST0`.
const ACTOR_GET_RADIUS: u32 = 0x008b_e1b0;
/// `Pathing::FindClosestPointOnNavmesh(worldspace, cell, position, out)`.
const FIND_CLOSEST_POINT_ON_NAVMESH: u32 = 0x006d_6f80;
/// `PathingLocation::PathingLocation(position, actor)`.
const PATHING_LOCATION_CONSTRUCTOR: u32 = 0x006d_ce10;
/// `PathingLocation::~PathingLocation`.
const PATHING_LOCATION_DESTRUCTOR: u32 = 0x004f_f7e0;
/// Constructor of the local `0x18`-byte object `00881860` fills (`006a0480`).
const PATH_POINT_CONSTRUCTOR: u32 = 0x006a_0480;
/// `(location, radius, out)` (`006d4570`, cdecl).
const PATHING_FIND_POINT: u32 = 0x006d_4570;
/// `PathingRequestClosePoint::PathingRequestClosePoint`.
const REQUEST_CONSTRUCTOR: u32 = 0x006e_3f30;
/// `(request, actor)` (`006e29f0`).
const REQUEST_SET_ACTOR: u32 = 0x006e_29f0;
/// `(request, float)` (`00507610`).
const REQUEST_SET_RADIUS: u32 = 0x0050_7610;
/// `(request, float)` (`006e5ee0`).
const REQUEST_SET_OUTER_RADIUS: u32 = 0x006e_5ee0;
/// `(request, position)` (`006d33c0`, cdecl).
const REQUEST_SUBMIT: u32 = 0x006d_33c0;
/// `PathingRequestClosePoint` destructor (`006dad70`).
const REQUEST_DESTRUCTOR: u32 = 0x006d_ad70;
/// Returns the singleton `0087b650` and `0087b190` are called on (`004537b0`).
const PATHING_SINGLETON: u32 = 0x0045_37b0;
/// `(singleton, actor, position)` (`0087b650`).
const PATHING_ORDER_MOVE: u32 = 0x0087_b650;
/// `(singleton, actor)` (`0087b190`).
const PATHING_ORDER_FINISH: u32 = 0x0087_b190;
/// Whether the pathing singleton is usable (`008c7aa0`).
const PATHING_AVAILABLE: u32 = 0x008c_7aa0;
/// Tests of the actor in `Actor::CanMove` (`00437bf0`, `00437bd0`).
const ACTOR_BLOCK_TEST_A: u32 = 0x0043_7bf0;
const ACTOR_BLOCK_TEST_B: u32 = 0x0043_7bd0;
/// `PlayerCharacter::IsPipboyActive` (decompiler name).
const IS_PIPBOY_ACTIVE: u32 = 0x0096_7ae0;
/// The action animation of the actor (`008a7570`, decompiler name `GetAnimAction`).
const GET_ANIM_ACTION: u32 = 0x008a_7570;
/// Flag test on a base form's data (`00461580(this, mask)`).
const BASE_FLAG_TEST: u32 = 0x0046_1580;
/// Barter gold of a base form's data (`0047d3f0`), a `short`.
const BARTER_GOLD: u32 = 0x0047_d3f0;
/// The class of an actor base (`00502430`).
const ACTOR_BASE_CLASS: u32 = 0x0050_2430;
/// The last-seen interval (`00526100`).
const LAST_SEEN_INTERVAL: u32 = 0x0052_6100;
/// The reference's encounter zone (`00567d20`).
const ENCOUNTER_ZONE: u32 = 0x0056_7d20;
/// Whether the encounter zone has its flag set (`00526320`).
const ENCOUNTER_ZONE_FLAG: u32 = 0x0052_6320;
/// `bhkRagdollController::DisableRagdollAnim(flag)` (Xbox PDB).
const DISABLE_RAGDOLL_ANIM: u32 = 0x00c7_c150;
/// `bhkBlendController::DoKnockDown` (Xbox PDB).
const DO_KNOCK_DOWN: u32 = 0x00c9_b670;
/// `(actor)` (`0089f580`).
const ACTOR_AFTER_ANIMATION: u32 = 0x0089_f580;
/// `(actor, out)` (`00931ed0`).
const ACTOR_POSITION_OBJECT: u32 = 0x0093_1ed0;
/// `(object)` (`004a3a20`).
const POSITION_OBJECT_VALUE: u32 = 0x004a_3a20;
/// `(controller, node, value)` (`00ca2ad0`).
const RAGDOLL_SET_POSITION: u32 = 0x00ca_2ad0;
/// `Animation::GroupLoaded(group)` (Xbox PDB).
const ANIMATION_GROUP_LOADED: u32 = 0x0049_4710;
/// `Animation::ForceSection` (Xbox PDB).
const ANIMATION_FORCE_SECTION: u32 = 0x0049_55c0;
/// `(animation, flag)` (`00491040`).
const ANIMATION_CURRENT: u32 = 0x0049_1040;
/// `Animation::Update` (Xbox PDB).
const ANIMATION_UPDATE: u32 = 0x0049_1180;
/// `Animation::UpdateMovement` (Xbox PDB).
const ANIMATION_UPDATE_MOVEMENT: u32 = 0x0049_3900;
/// `Animation::ZeroGlobalTransform` (Xbox PDB).
const ANIMATION_ZERO_GLOBAL_TRANSFORM: u32 = 0x0048_f7f0;
/// `TESAnimGroup::GetTime` (Xbox PDB).
const ANIM_GROUP_GET_TIME: u32 = 0x005f_3780;
/// `(animation)` returning a float (`00508100`).
const ANIMATION_TIME: u32 = 0x0050_8100;
/// `(animation, float)` (`0098adb0`).
const ANIMATION_SET_TIME: u32 = 0x0098_adb0;
/// Sets the angle of a local matrix: `(matrix, float)` (`004a0c90`).
const MATRIX_SET_ANGLE: u32 = 0x004a_0c90;
/// Builds a vector: `(vector, x, y, z)` (`00416870`).
const VECTOR_BUILD: u32 = 0x0041_6870;
/// Rotates a vector by a matrix: `(matrix, out, vector)` (`004b4500`).
const MATRIX_TRANSFORM: u32 = 0x004b_4500;
/// The actor's 3D node (`0043fcd0`; also actor slot `0x1d0`).
const GET_3D_NODE: u32 = 0x0043_fcd0;
/// The form-type byte of a form (`00401170`), same as [`FORM_TYPE`].
const GET_FORM_TYPE: u32 = 0x0040_1170;
/// `ExtraDataList::GetRagDollData` (Xbox PDB).
const GET_RAGDOLL_DATA: u32 = 0x0041_d6d0;
/// `ExtraDataList::GetSavedHavokData` (Xbox PDB).
const GET_SAVED_HAVOK_DATA: u32 = 0x0042_2b90;
/// `(flag)` returning an object (`00450b80`, cdecl).
const OBJECT_GETTER: u32 = 0x0045_0b80;
/// `(object, value)` (`00b5cbd0`).
const OBJECT_SET_VALUE: u32 = 0x00b5_cbd0;
/// `(actor mover)` (`009dc7f0`).
const MOVER_UPDATE_A: u32 = 0x009d_c7f0;
/// `(actor mover)` (`009dc780`).
const MOVER_UPDATE_B: u32 = 0x009d_c780;
/// `(actor, flag)` (`008a6840`).
const ACTOR_SET_FLAG: u32 = 0x008a_6840;
/// `(worn item, flag)` (`004459e0`).
const WORN_ITEM_UPDATE: u32 = 0x0044_59e0;
/// `(actor)` (`008c0050`).
const ACTOR_TEST_008C0050: u32 = 0x008c_0050;
/// `TESDataHandler::EnumReferencesCloseToPoint` (decompiler name), on the
/// object at [`SHUTDOWN_CHECK_OBJECT_POINTER`].
const ENUM_REFERENCES_CLOSE_TO_POINT: u32 = 0x0046_f280;
/// The callback `00883800` passes to that enumeration.
const ENUM_CALLBACK: u32 = 0x0090_d480;
/// `(actor, object)` (`00891170`): returns an object whose `+0x10` is tested.
const NOTE_LOOKUP: u32 = 0x0089_1170;
/// The object `00891170` is called with.
const NOTE_OBJECT_011F426C: u32 = 0x011f_426c;
/// Test of the `+0x10` part of that object (`004390c0`).
const NOTE_TEST: u32 = 0x0043_90c0;
/// `(actor, float)` (`00575770`).
const ACTOR_SET_FLOAT: u32 = 0x0057_5770;
/// Returns a pointer to a float of the reference (`00430830`); `FLT_MAX` means unset.
const REFERENCE_FLOAT_POINTER: u32 = 0x0043_0830;
/// `FLT_MAX` as a `double`.
const FLT_MAX_DOUBLE: u32 = 0x0102_31b0;
/// `-1.0f`.
const MINUS_ONE_FLOAT: u32 = 0x0101_2054;
/// `0.0` as a `double`.
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// `1.0` as a `double`.
const ONE_DOUBLE: u32 = 0x0101_2070;
/// `0.5` as a `double`.
const HALF_DOUBLE: u32 = 0x0101_1588;
/// `1.5` as a `double`.
const ONE_AND_HALF_DOUBLE: u32 = 0x0101_6ff0;
/// `3.0` as a `double`.
const THREE_DOUBLE: u32 = 0x0102_1928;
/// `5000.0f`.
const ENUM_RADIUS_FLOAT: u32 = 0x0103_0020;
/// `CombatFormulas::GetBodyPartCondition(owner, part, flag)` (Xbox PDB), result in `ST0`.
const GET_BODY_PART_CONDITION: u32 = 0x0064_6800;

/// The actor's current process (`this + 0x68`).
fn process_of(e: &Engine, this: Ptr<Actor>) -> u32 {
    e.get(this, Actor::pCurrentProcess).addr()
}

/// The level of `process` (`0045cd60`).
fn process_level(e: &mut Engine, process: u32) -> i32 {
    e.call(PROCESS_LEVEL, &args![process]).i32()
}

/// The type word of a package (`0041ca90`).
fn package_type(e: &mut Engine, package: u32) -> u32 {
    e.call(PACKAGE_TYPE, &args![package]).u32()
}

/// `0041ca90`, the package type word (same function as [`package_kind`]).
const PACKAGE_TYPE: u32 = 0x0041_ca90;

/// The object `ECX` points at in `004226e0(*SHUTDOWN_CHECK_OBJECT_POINTER)`.
fn shutdown_check(e: &mut Engine) -> bool {
    let object = e.global::<u32>(SHUTDOWN_CHECK_OBJECT_POINTER);
    e.call(SHUTDOWN_CHECK, &args![object]).bool()
}

/// `Calendar::GetHour` of the calendar singleton.
fn calendar_hour(e: &mut Engine) -> f64 {
    e.call(CALENDAR_GET_HOUR, &args![CALENDAR_OBJECT]).f64()
}

/// The constant 'double' stored at `addr`.
fn double_at(e: &Engine, addr: u32) -> f64 {
    e.global::<f64>(addr)
}

/// Replaces the actor's process with `new_process`: `new_process.vcall(4)(old)`,
/// the old process's deleting destructor, and the store into `this + 0x68`.
fn swap_process(e: &mut Engine, this: Ptr<Actor>, new_process: u32) {
    let old = process_of(e, this);
    e.vcall(new_process, 4, &args![old]);
    if old != 0 {
        e.vcall(old, 0, &args![1u32]);
    }
    e.set(this, Actor::pCurrentProcess, Ptr::new(new_process));
}

/// Allocates `size` bytes with `operator new` and runs `constructor` on them
/// (null stays null).
fn construct_process(e: &mut Engine, size: u32, constructor: u32) -> u32 {
    let memory = e.call(OPERATOR_NEW, &args![size]).u32();
    if memory != 0 {
        e.call(constructor, &args![memory]).u32()
    } else {
        0
    }
}

/// Ends an interrupt package when the actor's package is an interrupt package
/// that the actor does not need any more; the opening test the process
/// transitions share (`008d8520` process, its virtual `+0x27c` package of type
/// `0x1c`, then its virtual `+0x2d8`).
fn end_interrupt_if_requested(e: &mut Engine, this: Ptr<Actor>) {
    if actor_process(e, this).is_null() {
        return;
    }
    let process = actor_process(e, this).addr();
    let package = e.vcall(process, 0x27c, &args![]).u32();
    if package != 0 && package_type(e, package) == 0x1c {
        let process = actor_process(e, this).addr();
        if e.vcall(process, 0x2d8, &args![]).bool() {
            actor_end_interrupt_package(e, this, false);
        }
    }
}

// Translated from 00881450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetPackage` (Xbox PDB): the reference's current package
/// (`0047f520(reference)`).
pub fn actor_get_package(e: &mut Engine, this: Ptr<Actor>) -> Ptr {
    e.call(REFERENCE_CURRENT_PACKAGE, &args![this]).ptr()
}

// Translated from 00881470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetMissedPacks` (Xbox PDB): `0047f590(this, a, b, hour)` with the
/// calendar's hour as a `float`.
pub fn actor_get_missed_packs(e: &mut Engine, this: Ptr<Actor>, arg_a: u32, arg_b: f32) {
    let hour = calendar_hour(e) as f32;
    e.call(REFERENCE_MISSED_PACKAGES, &args![this, arg_a, arg_b, hour]);
}

// Translated from 008814b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetCurrentEditorPackage` (Xbox PDB): without a process 0; otherwise
/// the process's current package (virtual `+0x22c`) unless it is missing or an
/// interrupt package, in which case the package of the reference's extra data.
pub fn actor_get_current_editor_package(e: &mut Engine, this: Ptr<Actor>) -> Ptr {
    let process = process_of(e, this);
    if process == 0 {
        return Ptr::new(0);
    }
    let package = e.vcall(process, 0x22c, &args![]).u32();
    if package != 0 && !e.call(IS_INTERRUPT_PACKAGE, &args![package]).bool() {
        return Ptr::new(package);
    }
    let list = extra_data_list(e, this);
    e.call(EXTRA_PACKAGE, &args![list]).ptr()
}

// Translated from 00881510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetPackageSetAsPcurrent` (Xbox PDB): the process's current package
/// (virtual `+0x22c`), replaced by the package of the reference's extra data
/// when it is an interrupt package; 0 without a process.
pub fn actor_get_package_set_as_pcurrent(e: &mut Engine, this: Ptr<Actor>) -> Ptr {
    let process = process_of(e, this);
    let mut package = 0;
    if process != 0 {
        package = e.vcall(process, 0x22c, &args![]).u32();
    }
    if package != 0 && e.call(IS_INTERRUPT_PACKAGE, &args![package]).bool() {
        let list = extra_data_list(e, this);
        package = e.call(EXTRA_PACKAGE, &args![list]).u32();
    }
    Ptr::new(package)
}

// Translated from 00881570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::IsRunningRunOnce` (Xbox PDB): the process's virtual `+0x35c`
/// (the process is not checked).
pub fn actor_is_running_run_once(e: &mut Engine, this: Ptr<Actor>) -> bool {
    let process = process_of(e, this);
    e.vcall(process, 0x35c, &args![]).bool()
}

// Translated from 008815a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Without a process 0. With one: when `00493bb0` is false, or the actor is
/// running a run-once package and is not fleeing, or its virtual `+0x42c` gives
/// 0, the process's virtual `+0x128`; otherwise the result of the actor's
/// virtual `+0x42c`.
pub fn fn_008815a0(e: &mut Engine, this: Ptr<Actor>) -> Ptr {
    let process = process_of(e, this);
    if process == 0 {
        return Ptr::new(0);
    }
    if e.call(ANIMATION_FLAG_TEST, &args![this]).bool() {
        let use_actor_slot =
            !actor_is_running_run_once(e, this) || e.call(IS_FLEEING, &args![this, 0u32]).bool();
        if use_actor_slot {
            let value = e.vcall(this.addr(), 0x42c, &args![]).u32();
            if value != 0 {
                return Ptr::new(value);
            }
        }
    }
    e.vcall(process, 0x128, &args![]).ptr()
}

/// `00493bb0(actor)`: a test of the actor (the engine map puts it in `animation.cpp`).
const ANIMATION_FLAG_TEST: u32 = 0x0049_3bb0;

// Translated from 00881620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::SetCurrentTarget` (Xbox PDB): with a process, its virtual `+0x12c`
/// with `target`.
pub fn actor_set_current_target(e: &mut Engine, this: Ptr<Actor>, target: u32) {
    let process = process_of(e, this);
    if process != 0 {
        e.vcall(process, 0x12c, &args![target]);
    }
}

// Translated from 00881650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetCurrentPackageTarget` (Xbox PDB): without a process 0, else the
/// word at `+8` of the block at `process + 4` (`0044ddc0`).
pub fn actor_get_current_package_target(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    let process = process_of(e, this);
    if process == 0 {
        return 0;
    }
    e.call(WORD_AT_8, &args![process + 4]).u32()
}

// Translated from 00881680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::EndInterruptPackage` (Xbox PDB): with a process, drops the current
/// interrupt package. `keep_idle` skips the actor's virtual `+0x288` call and
/// keeps the running type-`0x1c` package.
pub fn actor_end_interrupt_package(e: &mut Engine, this: Ptr<Actor>, keep_idle: bool) {
    let process = process_of(e, this);
    if process == 0 {
        return;
    }
    if !keep_idle {
        e.vcall(this.addr(), 0x288, &args![]);
    }
    let process = process_of(e, this);
    let package = e.vcall(process, 0x22c, &args![]).u32();
    let player = e.global::<u32>(PLAYER_POINTER);
    // PlayerCharacter +0x208: the package the player is waiting on.
    if package == e.mem.u32(player + 0x208) {
        let player = e.global::<u32>(PLAYER_POINTER);
        e.mem.set_u32(player + 0x208, 0);
    }
    let mut ended = false;
    let current = e.vcall(process, 0x20c, &args![]).u32();
    if current != 0
        && (!keep_idle || package_type(e, current) != 0x1c)
        && package_type(e, current) != 0x1a
    {
        e.vcall(process, 0x214, &args![]);
        ended = true;
    }
    if package != 0 && e.call(IS_INTERRUPT_PACKAGE, &args![package]).bool() {
        if package_type(e, package) == 0x17 {
            fn_00881830(e, Ptr::new(package), -1);
        }
        e.call(CLEAR_IN_COMBAT, &args![this, 1u32]);
        e.vcall(this.addr(), 0x4c, &args![0u32]);
        let process = process_of(e, this);
        e.vcall(process, 0x234, &args![]);
        e.vcall(process, 0x714, &args![this]);
        let after = e.vcall(process, 0x22c, &args![]).u32();
        if after == 0 {
            let object = e.global::<u32>(SHUTDOWN_CHECK_OBJECT_POINTER);
            if !e.call(SHUTDOWN_CHECK, &args![object]).bool() {
                e.vcall(process, 0x24, &args![this, 0u32]);
            }
        }
        ended = true;
    }
    if ended {
        actor_end_movement(e, this);
    }
}

// Translated from 00881830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `delta` to the word at `+0x98` of `this` (a package; wraps).
pub fn fn_00881830(e: &mut Engine, this: Ptr, delta: i32) {
    let value = e.mem.u32(this.addr() + 0x98);
    e.mem
        .set_u32(this.addr() + 0x98, value.wrapping_add(delta as u32));
}

// Translated from 00881860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves the actor toward a navmesh point near it; true when it did. With
/// `force` false it does nothing for an actor that cannot move, only swims or is
/// blocked by its virtual `+0x4b4`. The package (virtual `+0x22c` of the
/// process) can veto the move; otherwise the actor's position is snapped to the
/// navmesh, pulled away from the player when it is closer than 1.5 times the
/// sum of their radii (with a close-point request for the player), and the
/// move is ordered through the pathing singleton or the actor's virtual `+0x2a8`.
pub fn fn_00881860(e: &mut Engine, this: Ptr<Actor>, force: bool) -> bool {
    let blocked = !actor_can_move(e, this)
        || actor_swims_only(e, this)
        || e.vcall(this.addr(), 0x4b4, &args![]).bool();
    if blocked && !force {
        return false;
    }
    let mut proceed = true;
    let process = process_of(e, this);
    let package = if process != 0 {
        e.vcall(process, 0x22c, &args![]).u32()
    } else {
        0
    };
    if package != 0 {
        let minus_one = e.global::<f32>(MINUS_ONE_FLOAT);
        let accepted = e
            .vcall(package, 0x13c, &args![this, 0u32, minus_one, 0u32])
            .bool();
        if accepted {
            let mut vetoed = false;
            if package_type(e, package) == 6 {
                let reference = e
                    .call(PACKAGE_LOCATION_REFERENCE, &args![package, this])
                    .u32();
                if reference != 0 {
                    let reference = e
                        .call(PACKAGE_LOCATION_REFERENCE, &args![package, this])
                        .u32();
                    let form = e.call(GET_FORM, &args![reference]).u32();
                    if form == e.global::<u32>(LOCATION_FORM_POINTER_011CA248) {
                        proceed = false;
                        vetoed = true;
                    }
                }
            }
            if !vetoed && e.call(PACKAGE_LOCATION, &args![package]).u32() != 0 {
                let location = e.call(PACKAGE_LOCATION, &args![package]).u32();
                if e.call(PACKAGE_LOCATION_TYPE, &args![location]).i32() == 3
                    && e.call(PACKAGE_LOCATION_CHECK, &args![package, this]).u32() == 0
                {
                    proceed = false;
                }
            }
        }
    }
    if !proceed {
        return false;
    }
    // The game's locals, laid out as in its frame (offsets from the frame's
    // bottom at ebp - 0x160).
    e.with_stack(0x160, |e, frame| {
        let at = |ebp_offset: u32| frame.addr() + 0x160 - ebp_offset;
        let pathing_location = at(0x74);
        let path_point = at(0x4c);
        let path_point_position = at(0x40);
        let target = at(0x34);
        let to_player = at(0x28);
        let direction = at(0x84);
        let scaled = at(0x148);
        let sum = at(0x154);
        let request = at(0x13c);

        let position = e.vcall(this.addr(), 0x1f4, &args![]).u32();
        copy_point(e, position, target);
        let position = e.vcall(this.addr(), 0x1f4, &args![]).u32();
        let cell = e.call(PARENT_CELL, &args![this]).u32();
        let world_space = e.call(WORLD_SPACE, &args![this]).u32();
        let found = e
            .call(
                FIND_CLOSEST_POINT_ON_NAVMESH,
                &args![world_space, cell, position, target],
            )
            .bool();
        if found {
            e.call(
                PATHING_LOCATION_CONSTRUCTOR,
                &args![pathing_location, target, this],
            );
            e.call(PATH_POINT_CONSTRUCTOR, &args![path_point]);
            let radius = e.call(ACTOR_GET_RADIUS, &args![this]).f32();
            let has_point = e
                .call(
                    PATHING_FIND_POINT,
                    &args![pathing_location, radius, path_point],
                )
                .bool();
            if has_point {
                e.call(
                    POINT_SUBTRACT,
                    &args![target, direction, path_point_position],
                );
                e.mem.set_f32(direction + 8, 0.0);
                let length = e.call(POINT_LENGTH_2D, &args![direction]).f32();
                let radius = e.call(ACTOR_GET_RADIUS, &args![this]).f64();
                if (length as f64) < radius {
                    let radius = e.call(ACTOR_GET_RADIUS, &args![this]).f32();
                    let scaled_point = e.call(POINT_SCALE, &args![direction, scaled, radius]).u32();
                    let moved = e
                        .call(POINT_ADD, &args![path_point_position, sum, scaled_point])
                        .u32();
                    copy_point(e, moved, target);
                }
            }
            e.call(PATHING_LOCATION_DESTRUCTOR, &args![pathing_location]);
        }
        let player = e.global::<u32>(PLAYER_POINTER);
        let player_position = e.vcall(player, 0x1f4, &args![]).u32();
        e.call(POINT_SUBTRACT, &args![target, to_player, player_position]);
        let player_radius = e.call(ACTOR_GET_RADIUS, &args![player]).f64();
        let own_radius = e.call(ACTOR_GET_RADIUS, &args![this]).f64();
        let reach = (own_radius + player_radius) as f32;
        let distance = e.call(POINT_LENGTH_SQUARED, &args![to_player]).f64();
        let one_and_half = double_at(e, ONE_AND_HALF_DOUBLE);
        let limit = reach as f64 * reach as f64 * one_and_half * one_and_half;
        if limit > distance {
            e.call(REQUEST_CONSTRUCTOR, &args![request]);
            e.call(REQUEST_SET_ACTOR, &args![request, player]);
            let inner = (reach as f64 * one_and_half) as f32;
            e.call(REQUEST_SET_RADIUS, &args![request, inner]);
            let outer = (reach as f64 * double_at(e, THREE_DOUBLE)) as f32;
            e.call(REQUEST_SET_OUTER_RADIUS, &args![request, outer]);
            e.call(REQUEST_SUBMIT, &args![request, target]);
            e.call(REQUEST_DESTRUCTOR, &args![request]);
        }
        if e.call(PATHING_AVAILABLE, &args![]).bool() {
            let singleton = e.call(PATHING_SINGLETON, &args![]).u32();
            e.call(PATHING_ORDER_MOVE, &args![singleton, this, target]);
            let singleton = e.call(PATHING_SINGLETON, &args![]).u32();
            e.call(PATHING_ORDER_FINISH, &args![singleton, this]);
        } else {
            e.vcall(this.addr(), 0x2a8, &args![target]);
        }
    });
    true
}

// Translated from 00881c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the actor may be seen again at time stamp `time`: true when it was
/// never seen (`+0x120` is 0); false when `time` is not later; otherwise true
/// once the elapsed time exceeds the interval of `00526100`, unless the
/// reference's encounter zone has its flag.
pub fn fn_00881c90(e: &mut Engine, this: Ptr<Actor>, time: u32) -> bool {
    let last_seen = e.get(this, Actor::iLastSeenTime);
    if last_seen == 0 {
        return true;
    }
    if time > last_seen {
        let elapsed = time.wrapping_sub(last_seen);
        let interval = e.call(LAST_SEEN_INTERVAL, &args![]).u32();
        if elapsed > interval {
            let zone = e.call(ENCOUNTER_ZONE, &args![this]).u32();
            if zone != 0 && e.call(ENCOUNTER_ZONE_FLAG, &args![zone]).bool() {
                return false;
            }
            return true;
        }
    }
    false
}

// Translated from 00881d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the calendar's time stamp (`00867e30` of the calendar singleton) in
/// `iLastSeenTime`.
pub fn fn_00881d10(e: &mut Engine, this: Ptr<Actor>) {
    let stamp = e.call(CALENDAR_TIME_STAMP, &args![CALENDAR_OBJECT]).u32();
    e.set(this, Actor::iLastSeenTime, stamp);
}

/// `Actor::GetAnimation` (Xbox PDB).
const GET_ANIMATION: u32 = 0x008b_70d0;

/// Enters the guard object (`0040fbf0(flag)`).
fn guard_enter(e: &mut Engine, flag: u32) {
    e.call(GUARD_ENTER, &args![GUARD_OBJECT_011F11A0, flag]);
}

/// Leaves the guard object (`0040fba0()`).
fn guard_leave(e: &mut Engine) {
    e.call(GUARD_LEAVE, &args![GUARD_OBJECT_011F11A0]);
}

/// The test the transitions end with: the process's virtual `+0x31c`, or a
/// current package (virtual `+0x22c`) that `0067a460` accepts; when it holds,
/// the process's virtual `+0x450` with 1. `package` is the package already
/// read by the caller (`0` to read it from the process).
fn request_process_update(e: &mut Engine, this: Ptr<Actor>, package: Option<u32>) {
    let process = process_of(e, this);
    let mut wanted = e.vcall(process, 0x31c, &args![]).bool();
    if !wanted {
        wanted = match package {
            Some(package) => package != 0 && e.call(PACKAGE_TEST_0067A460, &args![package]).bool(),
            None => {
                e.vcall(process, 0x22c, &args![]).u32() != 0 && {
                    let package = e.vcall(process, 0x22c, &args![]).u32();
                    e.call(PACKAGE_TEST_0067A460, &args![package]).bool()
                }
            }
        };
    }
    if wanted {
        let process = process_of(e, this);
        e.vcall(process, 0x450, &args![1u32]);
    }
}

/// The equip step of the transitions: when the reference has container
/// changes, the worn item of slot 5 goes to the process's virtual `+0x160`.
fn worn_item(e: &mut Engine, changes: u32) -> u32 {
    e.call(GET_WORN_ITEM, &args![changes, 5u32, 0u32]).u32()
}

// Translated from 00881d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Puts the actor into the high process (the first of the four process
/// transitions `00881d30`, `00882b90`, `00883240`, `00883800`; always true).
///
/// An actor without a process, or already in the high process (level 0), only
/// has its worn item refreshed. Otherwise the process is replaced with a new
/// `HighProcess` (`0x46c` bytes), the actor re-registers in the process
/// lists, equips its worn item, and an actor flagged `bDeadFlag` with a loaded
/// 3D is woken from its ragdoll or knock-down state.
pub fn fn_00881d30(e: &mut Engine, this: Ptr<Actor>) -> bool {
    let old_process = process_of(e, this);
    if old_process == 0 || process_level(e, old_process) == 0 {
        // No process, or already high.
        let process = process_of(e, this);
        if process != 0 && e.vcall(this.addr(), 0x1d0, &args![]).u32() != 0 {
            let list = extra_data_list(e, this);
            let changes = e.call(GET_CONTAINER_CHANGES, &args![list]).u32();
            if changes != 0 {
                let mut with_node = false;
                if e.call(GET_FORCE_NEXT_UPDATE, &args![this]).bool()
                    && e.vcall(this.addr(), 0x21c, &args![]).bool()
                {
                    let name = e.call(REFERENCE_NAME, &args![this]).u32();
                    if e.call(STRSTR, &args![name, LILY_TEXT]).u32() != 0 {
                        let node = e.vcall(this.addr(), 0x1d0, &args![]).u32();
                        let worn = worn_item(e, changes);
                        let process = process_of(e, this);
                        e.vcall(process, 0x160, &args![worn, node, 1u32]);
                        with_node = true;
                    }
                }
                if !with_node {
                    let worn = worn_item(e, changes);
                    let process = process_of(e, this);
                    e.vcall(process, 0x160, &args![worn, 0u32, 1u32]);
                }
            }
            request_process_update(e, this, None);
        }
        return true;
    }

    // A lower process: build a high one.
    let ignore_flag = {
        let object = e.global::<u32>(SHARED_OBJECT_POINTER_011DDF38);
        e.call(SHARED_OBJECT_TEST, &args![object]).bool()
            || e.call(REFERENCE_TEST, &args![this]).bool()
    };
    e.mem.set_u8(this.addr() + 0x83, 0);
    if e.call(CURRENT_PROCESS_TYPE, &args![this]).i32() != 1 {
        let process = actor_process(e, this).addr();
        e.vcall(process, 0x84, &args![this]);
    }
    let animation_flag = e.call(ANIMATION_FLAG_TEST, &args![this]).bool();
    if !animation_flag && process_of(e, this) != 0 && !ignore_flag {
        let process = process_of(e, this);
        e.vcall(process, 0x24, &args![this, 1u32]);
    }
    // The result (a "fleeing" flag) is stored by the game and never read.
    e.call(IS_FLEEING, &args![this, 0u32]);
    guard_enter(e, 0);
    if !e.vcall(this.addr(), 0x22c, &args![0u32]).bool() {
        e.call(PROCESS_LISTS_NOTE_ACTOR, &args![PROCESS_LISTS, this]);
    }
    let mut level = 3;
    let mut package = 0;
    if process_of(e, this) != 0 {
        let process = process_of(e, this);
        level = process_level(e, process);
        package = e.vcall(process, 0x22c, &args![]).u32();
        e.call(
            PROCESS_LISTS_REMOVE_REFERENCE,
            &args![PROCESS_LISTS, this, level as u32],
        );
    }
    let mut wants_move = false;
    let mut flag_12 = false;
    let float_pointer = e.call(REFERENCE_FLOAT_POINTER, &args![this]).u32();
    let value = e.mem.f32(float_pointer) as f64;
    if value == double_at(e, FLT_MAX_DOUBLE) {
        wants_move = true;
        e.call(ACTOR_SET_FLOAT, &args![this, 0.0f32]);
        let owner = this.addr() + ACTOR_VALUE_OWNER;
        if e.vcall(owner, 4, &args![0x10u32]).f64() > double_at(e, ZERO_DOUBLE) {
            flag_12 = true;
        }
    } else {
        let note = e.call(NOTE_LOOKUP, &args![this]).u32();
        // 00882116: `00891170` takes no stack word; the pushed note object is
        // the word of `004390c0` (RET 4).
        if e.call(NOTE_TEST, &args![note + 0x10, NOTE_OBJECT_011F426C])
            .bool()
        {
            wants_move = true;
        }
    }
    if !wants_move {
        let object = e.global::<u32>(SHARED_OBJECT_POINTER_011DDF38);
        if !e.call(SHARED_OBJECT_TEST, &args![object]).bool() && (2..=3).contains(&level) {
            wants_move = true;
        }
    }
    let mut failed = true;
    if wants_move && fn_00881860(e, this, flag_12) {
        failed = false;
    }
    if failed {
        e.call(ACTOR_NOTE_FAILURE, &args![this]);
    }

    let new_process = construct_process(e, 0x46c, HIGH_PROCESS_CONSTRUCTOR);
    swap_process(e, this, new_process);
    if e.vcall(this.addr(), 0x1d0, &args![]).u32() != 0 {
        let form = base_form(e, this).addr();
        let first = e.vcall(form, 0x180, &args![]).u32();
        let second = e.vcall(this.addr(), 0x1d0, &args![]).u32();
        let process = process_of(e, this);
        e.vcall(process, 0x6c8, &args![second, first]);
    }
    e.call(
        PROCESS_LISTS_ADD_REFERENCE,
        &args![PROCESS_LISTS, this, 0u32, 0u32, 0u32, 0u32],
    );
    guard_leave(e);
    e.call(HIGH_PROCESS_SET_ACTOR, &args![new_process, this]);
    e.vcall(new_process, 0x58, &args![]);
    let list = extra_data_list(e, this);
    let changes = e.call(GET_CONTAINER_CHANGES, &args![list]).u32();
    if changes != 0 {
        if e.call(GET_FORCE_NEXT_UPDATE, &args![this]).bool() {
            let name = e.call(REFERENCE_NAME, &args![this]).u32();
            if e.call(STRSTR, &args![name, LILY_TEXT]).u32() != 0
                && e.vcall(this.addr(), 0x21c, &args![]).bool()
            {
                let form = e.call(GET_FORM, &args![this]).u32();
                e.call(INIT_DEFAULT_WORN, &args![form, this, 1u32, 1u32, 1u32]);
            }
        }
        let worn = worn_item(e, changes);
        if level != 1 {
            let process = process_of(e, this);
            e.vcall(process, 0x160, &args![worn, 0u32, 0u32]);
        }
    }
    request_process_update(e, this, Some(package));
    let process = process_of(e, this);
    e.vcall(process, 0x48, &args![]);
    e.call(UNLOCK_LOCK_DOORS_PROCEDURE, &args![this]);
    if animation_flag {
        e.call(MOVE_TO_PROCESS_LEVEL, &args![this, level as u32, 0u32]);
    }
    let node = e.call(GET_3D_NODE, &args![this]).u32();
    let mut may_ragdoll = true;
    let mut character = 0;
    let form = e.call(GET_FORM, &args![this]).u32();
    if e.call(GET_FORM_TYPE, &args![form]).u32() == 0x2b {
        character = this.addr();
    }
    if character != 0 && node != 0 && !e.vcall(character, 0x38c, &args![]).bool() {
        may_ragdoll = false;
    }
    if e.vcall(this.addr(), 0x22c, &args![1u32]).bool() && may_ragdoll {
        let list = extra_data_list(e, this);
        if e.call(GET_RAGDOLL_DATA, &args![list]).u32() == 0 {
            let list = extra_data_list(e, this);
            if e.call(GET_SAVED_HAVOK_DATA, &args![list]).u32() == 0 {
                e.set(this, Actor::bDeadFlag, true);
            }
        }
    }
    e.call(HIGH_PROCESS_INIT_ACTOR, &args![new_process, this]);
    e.vcall(this.addr(), 0x1fc, &args![0u32]);
    if e.get(this, Actor::bDeadFlag) && node != 0 {
        wake_from_death_pose(e, this, node);
    }
    if node != 0 {
        e.call(UPDATE_ALPHA, &args![this]);
    }
    let process = actor_process(e, this).addr();
    let current = e.call(MOBILE_OBJECT_PACKAGE, &args![this]).u32();
    if current != 0 && package_type(e, current) == 6 {
        e.vcall(process, 0x284, &args![0u32]);
    }
    if !e.vcall(this.addr(), 0x22c, &args![0u32]).bool() && (level == 3 || level == 2) {
        e.call(CAST_PERMANENT_MAGIC, &args![this, 0u32]);
    }
    if process != 0 && e.vcall(process, 0x360, &args![]).bool() {
        let target = e.vcall(process, 0x128, &args![]).u32();
        if target != 0 {
            let target = e.vcall(process, 0x128, &args![]).u32();
            if e.vcall(target, 0x100, &args![]).bool() {
                let target = e.vcall(process, 0x128, &args![]).u32();
                if target != 0 && e.call(CURRENT_PROCESS_TYPE, &args![target]).i32() != 0 {
                    e.vcall(process, 0x364, &args![0u32]);
                    e.call(
                        PROCESS_LISTS_REMOVE_REFERENCE,
                        &args![PROCESS_LISTS, this, 0u32],
                    );
                    e.vcall(process, 0x364, &args![1u32]);
                }
            }
        }
    }
    if e.vcall(this.addr(), 0x1d0, &args![]).u32() != 0 {
        let value = e.vcall(this.addr(), 0x1d0, &args![]).u32();
        let object = e.call(OBJECT_GETTER, &args![0u32]).u32();
        e.call(OBJECT_SET_VALUE, &args![object, value]);
    }
    let mover = e.get(this, Actor::pActorMover).addr();
    e.call(MOVER_UPDATE_A, &args![mover]);
    let process = actor_process(e, this).addr();
    if e.vcall(process, 0x20c, &args![]).u32() == 0 {
        let process = actor_process(e, this).addr();
        if !e.vcall(process, 0x228, &args![this]).bool() && !ignore_flag {
            let process = actor_process(e, this).addr();
            let package = e.vcall(process, 0x22c, &args![]).u32();
            if package != 0 {
                let process = actor_process(e, this).addr();
                let attached = e.call(ANIMATION_FLAG_TEST, &args![this]).u8() as u32;
                e.vcall(process, 0xb0, &args![this, package, attached]);
            }
        }
    }
    let process = process_of(e, this);
    e.vcall(process, 0x7a8, &args![this]);
    true
}

/// The part of `00881d30` that wakes an actor flagged `bDeadFlag` whose 3D
/// `node` is loaded: forces the death-pose animation section when the
/// animation has it, or knocks the node down, then clears the flag.
fn wake_from_death_pose(e: &mut Engine, this: Ptr<Actor>, node: u32) {
    e.vcall(this.addr(), 0x2a0, &args![]);
    let animation = e.call(GET_ANIMATION, &args![this]).u32();
    let controller = e.get(this, Actor::pRagdollController).addr();
    if controller != 0 {
        e.call(DISABLE_RAGDOLL_ANIM, &args![controller, 1u32]);
    }
    let penetration = e.get(this, Actor::pPenetrationDetection).addr();
    if penetration != 0 {
        e.with_stack(0x20, |e, scratch| {
            let object = e.call(ACTOR_POSITION_OBJECT, &args![this, scratch]).u32();
            let value = e.call(POSITION_OBJECT_VALUE, &args![object]).u32();
            e.call(RAGDOLL_SET_POSITION, &args![penetration, node, value]);
        });
    }
    if animation != 0
        && e.call(ANIMATION_GROUP_LOADED, &args![animation, 0xe0u32])
            .bool()
    {
        e.call(
            ANIMATION_FORCE_SECTION,
            &args![animation, 0x14u32, 0xe0u32, -1i32, 0.0f32, -1i32],
        );
        let current = e.call(ANIMATION_CURRENT, &args![animation, 1u32]).u32();
        if current != 0 {
            if !e.vcall(this.addr(), 0x38c, &args![]).bool() {
                let time = e.call(ANIMATION_TIME, &args![current]).f32();
                e.call(ANIMATION_SET_TIME, &args![current, time]);
                let time = e.call(ANIMATION_TIME, &args![current]).f32();
                e.call(ANIMATION_UPDATE, &args![animation, this, 0.0f32, time]);
                e.call(ANIMATION_UPDATE_MOVEMENT, &args![animation, this]);
            } else {
                let group = e
                    .call(ANIMATION_ZERO_GLOBAL_TRANSFORM, &args![current])
                    .u32();
                // 008826d3: the pushed 1 is the word of `005f3780` (RET 4).
                let time = e.call(ANIM_GROUP_GET_TIME, &args![group, 1u32]).f32();
                e.call(ANIMATION_UPDATE, &args![animation, this, 0.0f32, time]);
                e.call(ANIMATION_UPDATE_MOVEMENT, &args![animation, this]);
                knock_down(e, this, node, true);
            }
        }
        e.call(ACTOR_AFTER_ANIMATION, &args![this]);
        e.call(SET_LIFE_STATE, &args![this, 1u32]);
    } else if e.get(this, Actor::bDeadFlag) {
        e.call(SET_LIFE_STATE, &args![this, 1u32]);
        knock_down(e, this, node, false);
    }
    let process = process_of(e, this);
    e.vcall(process, 0x28, &args![]);
    e.set(this, Actor::bDeadFlag, false);
}

/// The knock-down both branches of [`wake_from_death_pose`] end with: builds
/// a vector (0, 1, 0) rotated by a z angle and calls
/// `DoKnockDown(node, vector, 1, 0.0, 1)`, then the actor's virtual `+0x48`
/// with 4. The angle is the float at `+8` of `00430830` (and the node a fresh
/// `0043fcd0`) when `from_reference_float`, else the actor's virtual `+0x2bc`
/// result and the caller's `node`.
fn knock_down(e: &mut Engine, this: Ptr<Actor>, node: u32, from_reference_float: bool) {
    // The rotation is a whole 0x24-byte NiMatrix3 (004a0c90 writes all nine floats), so the
    // vector and the transform's result follow it. The exe keeps them apart the same way:
    // matrix EBP-0x70, vector EBP-0x7c, result EBP-0xd4 in the first branch of 00881d30;
    // EBP-0xa0, EBP-0xac and EBP-0xe0 in the second.
    e.with_stack(0x40, |e, frame| {
        let matrix = frame.addr();
        let vector = frame.addr() + 0x24;
        let rotated = frame.addr() + 0x30;
        e.call(POINT_CONSTRUCTOR, &args![matrix]);
        let angle = if from_reference_float {
            let pointer = e.call(REFERENCE_FLOAT_POINTER, &args![this]).u32();
            e.mem.f32(pointer + 8)
        } else {
            e.vcall(this.addr(), 0x2bc, &args![0u32]).f32()
        };
        e.call(MATRIX_SET_ANGLE, &args![matrix, angle]);
        e.call(VECTOR_BUILD, &args![vector, 0.0f32, 1.0f32, 0.0f32]);
        let result = e
            .call(MATRIX_TRANSFORM, &args![matrix, rotated, vector])
            .u32();
        copy_point(e, result, vector);
        let node = if from_reference_float {
            e.call(GET_3D_NODE, &args![this]).u32()
        } else {
            node
        };
        e.call(DO_KNOCK_DOWN, &args![node, vector, 1u32, 0.0f32, 1u32]);
        e.vcall(this.addr(), 0x48, &args![4u32]);
    });
}

/// The opening the low, middle-low and middle-high transitions share: when the
/// actor is in the high process (level 0), stamps its last-seen time, resets
/// its process (`reset_process` is true for the low and middle-low
/// transitions) and registers the shader value with the accumulator.
fn leave_high_process(e: &mut Engine, this: Ptr<Actor>, reset_process: bool) {
    if process_of(e, this) != 0 {
        let process = process_of(e, this);
        if process_level(e, process) == 0 {
            fn_00881d10(e, this);
            if reset_process {
                let process = process_of(e, this);
                e.call(PROCESS_RESET, &args![process]);
            }
            let accumulator = e.call(GET_ACCUMULATOR, &args![]).u32();
            if accumulator != 0 {
                let value = e.call(ACTOR_SHADER_VALUE, &args![this]).u32();
                e.call(ACCUMULATOR_ADD, &args![accumulator, value]);
            }
        }
    }
}

/// The "killed or not" step of the low-process transitions: with the actor's
/// virtual `+0x2e8` true, runs the death hooks; otherwise an actor whose
/// virtual `+0x22c(0)` is true and which is in the high process gets its
/// virtual `+0xd4` with 1.
fn run_death_step(e: &mut Engine, this: Ptr<Actor>) {
    if e.vcall(this.addr(), 0x2e8, &args![]).bool() {
        e.call(DO_DEATH_STUFF, &args![this]);
        let process = process_of(e, this);
        e.vcall(process, 0x354, &args![this]);
        e.call(RUN_SCRIPT, &args![this]);
    } else if e.vcall(this.addr(), 0x22c, &args![0u32]).bool() {
        let process = process_of(e, this);
        if process_level(e, process) == 0 {
            e.vcall(this.addr(), 0xd4, &args![1u32]);
        }
    }
}

/// Resets the actor's life state when `004f8960` says 6: `SetLifeState(0)`, the
/// process's virtual `+0xe8` with `0.0`, optionally the full health restore,
/// and the process's virtual `+0x420`.
fn reset_dead_state(e: &mut Engine, this: Ptr<Actor>, restore_health: bool, call_420: bool) {
    if e.call(ACTOR_STATE_QUERY, &args![this]).i32() == 6 {
        e.call(SET_LIFE_STATE, &args![this, 0u32]);
        let process = process_of(e, this);
        e.vcall(process, 0xe8, &args![0.0f32]);
        if restore_health {
            e.call(RESTORE_FULL_HEALTH, &args![this]);
        }
        if call_420 {
            let process = process_of(e, this);
            e.vcall(process, 0x420, &args![this]);
        }
    }
}

/// Ends the interrupt package when the actor has a package that is an
/// interrupt package and the actor is neither flagged (`animation_flag`) nor
/// alarmed.
fn end_interrupt_unless_flagged(e: &mut Engine, this: Ptr<Actor>, animation_flag: bool) {
    let process = process_of(e, this);
    if process == 0 {
        return;
    }
    let package = e.vcall(process, 0x22c, &args![]).u32();
    if package == 0 {
        return;
    }
    let package = e.vcall(process, 0x22c, &args![]).u32();
    if e.call(IS_INTERRUPT_PACKAGE, &args![package]).bool()
        && !animation_flag
        && !e.call(IS_ALARMED, &args![this]).bool()
    {
        actor_end_interrupt_package(e, this, false);
    }
}

/// The scheduling step the three lower transitions end with, for process
/// `level` (3, 2 or 1) and `step` hours (the constant subtracted from the
/// calendar hour): re-times the new process and moves the actor between the
/// process lists, or hands it to `008a0680`.
fn schedule_in_process_lists(
    e: &mut Engine,
    this: Ptr<Actor>,
    old_level: i32,
    new_level: u32,
    animation_flag: bool,
    leave_hours: f64,
) {
    if animation_flag {
        e.call(
            MOVE_TO_PROCESS_LEVEL,
            &args![this, old_level as u32, new_level],
        );
    } else {
        let mut due = false;
        if e.vcall(this.addr(), 0x160, &args![]).bool() && e.mem.u8(this.addr() + 0x126) != 0 {
            due = true;
        } else if actor_is_following(e, this) || actor_is_escorter_behind(e, this) {
            let process = actor_process(e, this).addr();
            let player = e.global::<u32>(PLAYER_POINTER);
            if e.vcall(process, 0x128, &args![]).u32() == player {
                due = true;
            }
        }
        if due {
            let hour = calendar_hour(e);
            let time = (hour - leave_hours) as f32;
            let process = process_of(e, this);
            e.call(PROCESS_SET_TIME, &args![process, time]);
            e.call(
                PROCESS_LISTS_REMOVE_REFERENCE,
                &args![PROCESS_LISTS, this, new_level],
            );
            e.call(
                PROCESS_LISTS_ADD_REFERENCE,
                &args![PROCESS_LISTS, this, new_level, 0u32, 0u32, 0u32],
            );
        }
    }
}

// Translated from 00882b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Puts the actor into the low process (level 3; always true; the fourth of
/// the transitions after `00881d30`). Nothing happens for an actor already
/// there. Otherwise the actor leaves the high process, ends interrupt
/// packages, runs the death or dismiss steps, is re-registered in the process
/// lists with a new low process (`0xb4` bytes) and, when no shutdown is
/// pending, rescheduled; it ends with the process's virtual `+0x7a8` and the
/// actor's virtual `+0x4c` with `0x100000`.
pub fn fn_00882b90(e: &mut Engine, this: Ptr<Actor>) -> bool {
    let process = process_of(e, this);
    if process != 0 && process_level(e, process) == 3 {
        return true;
    }
    leave_high_process(e, this, true);
    e.set(this, Actor::cCurrentSitSleepState, 0);
    end_interrupt_if_requested(e, this);
    if e.vcall(this.addr(), 0x274, &args![0u32]).bool()
        && !e.call(ACTOR_TEST_00437BB0, &args![this]).bool()
    {
        e.vcall(this.addr(), 0x434, &args![0u32]);
    }
    guard_enter(e, 0);
    e.call(PROCESS_LISTS_REMOVE_ACTOR, &args![PROCESS_LISTS, this]);
    e.call(
        PROCESS_LISTS_REMOVE_CLOSE_TO_PLAYER,
        &args![PROCESS_LISTS, this],
    );
    reset_dead_state(e, this, true, true);
    let process = process_of(e, this);
    if process != 0 && e.vcall(process, 0x22c, &args![]).u32() != 0 {
        let package = e.vcall(process, 0x22c, &args![]).u32();
        if package_type(e, package) == 0x1c {
            let target = e.vcall(process, 0x128, &args![]).u32();
            if target != e.global::<u32>(PLAYER_POINTER) {
                actor_end_interrupt_package(e, this, false);
            }
        }
    }
    if e.call(IS_ALARMED, &args![this]).bool() && !e.vcall(this.addr(), 0x304, &args![]).bool() {
        let player = e.global::<u32>(PLAYER_POINTER);
        let cell = e.call(PARENT_CELL, &args![player]).u32();
        let in_cell = cell != 0 && {
            let cell = e.call(PARENT_CELL, &args![player]).u32();
            e.call(FLAG_BIT_0_AT_0X24, &args![cell]).bool()
        };
        if !in_cell {
            actor_end_interrupt_package(e, this, false);
        }
    }
    let process = process_of(e, this);
    let level = process_level(e, process);
    e.call(ACTOR_CLEAN_UP_A, &args![this]);
    if level == 1 || level == 0 {
        e.call(ACTOR_CLEAN_UP_B, &args![this]);
    }
    e.call(MAGIC_TARGET_DISPEL, &args![this.addr() + 0x94]);
    if !shutdown_check(e) {
        let object = e.global::<u32>(SHARED_OBJECT_POINTER_011DE45C);
        if !e.call(ALWAYS_FALSE, &args![object]).bool() {
            e.call(PROCESS_LISTS_REFRESH_ACTOR, &args![PROCESS_LISTS, this]);
        }
    }
    run_death_step(e, this);
    e.call(
        PROCESS_LISTS_REMOVE_REFERENCE,
        &args![PROCESS_LISTS, this, level as u32],
    );
    let continuing = e.call(IS_CONTINUING_PACKAGE_FOR_PC, &args![this]).bool();
    let animation_flag = e.call(ANIMATION_FLAG_TEST, &args![this]).bool();
    end_interrupt_unless_flagged(e, this, animation_flag);
    let new_process = construct_process(e, 0xb4, LOW_PROCESS_CONSTRUCTOR);
    let process = process_of(e, this);
    if e.vcall(process, 0x4c8, &args![]).u32() != 0 {
        e.vcall(process, 0x540, &args![0u32, 0u32, 0u32]);
    }
    swap_process(e, this, new_process);
    e.call(
        PROCESS_LISTS_ADD_REFERENCE,
        &args![PROCESS_LISTS, this, 3u32, 1u32, 0u32, 0u32],
    );
    if !animation_flag && continuing {
        let process = process_of(e, this);
        e.vcall(process, 0x24, &args![this, 0u32]);
    }
    if e.global::<u32>(SHUTDOWN_CHECK_OBJECT_POINTER) != 0 && !shutdown_check(e) {
        if continuing {
            let process = process_of(e, this);
            e.vcall(process, 0x24, &args![this, 0u32]);
        }
        schedule_in_process_lists(e, this, level, 3, animation_flag, double_at(e, ONE_DOUBLE));
        e.mem.set_u8(this.addr() + 0x126, 1);
    }
    guard_leave(e);
    if level == 0 {
        let mover = e.get(this, Actor::pActorMover).addr();
        e.call(MOVER_UPDATE_B, &args![mover]);
    }
    let process = process_of(e, this);
    e.vcall(process, 0x7a8, &args![this]);
    e.vcall(this.addr(), 0x4c, &args![0x0010_0000u32]);
    true
}

// Translated from 00883240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Puts the actor into the middle-low process (level 2; always true). Like
/// `00882b90`, with a new `0xc8`-byte process, a package type-5 check
/// (virtual `+0x284`) and half an hour of lead time; no health restore, and it
/// stops early for an actor already at level 2.
pub fn fn_00883240(e: &mut Engine, this: Ptr<Actor>) -> bool {
    e.call(
        PROCESS_LISTS_REMOVE_CLOSE_TO_PLAYER,
        &args![PROCESS_LISTS, this],
    );
    e.set(this, Actor::cCurrentSitSleepState, 0);
    let process = process_of(e, this);
    if process != 0 && process_level(e, process) == 2 {
        return true;
    }
    leave_high_process(e, this, true);
    end_interrupt_if_requested(e, this);
    let process = process_of(e, this);
    let level = process_level(e, process);
    e.call(ACTOR_CLEAN_UP_A, &args![this]);
    if level == 1 || level == 0 {
        e.call(ACTOR_CLEAN_UP_B, &args![this]);
    }
    e.call(MAGIC_TARGET_DISPEL, &args![this.addr() + 0x94]);
    run_death_step(e, this);
    guard_enter(e, 0);
    e.call(
        PROCESS_LISTS_REMOVE_REFERENCE,
        &args![PROCESS_LISTS, this, level as u32],
    );
    e.call(PROCESS_LISTS_REMOVE_ACTOR, &args![PROCESS_LISTS, this]);
    if !shutdown_check(e) {
        let object = e.global::<u32>(SHARED_OBJECT_POINTER_011DE45C);
        if !e.call(ALWAYS_FALSE, &args![object]).bool() {
            e.call(PROCESS_LISTS_REFRESH_ACTOR, &args![PROCESS_LISTS, this]);
        }
        let process = process_of(e, this);
        if process != 0 && e.vcall(process, 0x22c, &args![]).u32() != 0 {
            let package = e.vcall(process, 0x22c, &args![]).u32();
            if package_type(e, package) == 5 {
                e.vcall(process, 0x284, &args![0u32]);
            }
        }
    }
    reset_dead_state(e, this, true, true);
    let continuing = e.call(IS_CONTINUING_PACKAGE_FOR_PC, &args![this]).bool();
    let new_process = construct_process(e, 0xc8, MIDDLE_LOW_PROCESS_CONSTRUCTOR);
    let animation_flag = e.call(ANIMATION_FLAG_TEST, &args![this]).bool();
    end_interrupt_unless_flagged(e, this, animation_flag);
    swap_process(e, this, new_process);
    e.call(
        PROCESS_LISTS_ADD_REFERENCE,
        &args![PROCESS_LISTS, this, 2u32, 1u32, 0u32, 0u32],
    );
    if !animation_flag && continuing {
        let process = process_of(e, this);
        e.vcall(process, 0x24, &args![this, 0u32]);
    }
    if e.global::<u32>(SHUTDOWN_CHECK_OBJECT_POINTER) != 0 && !shutdown_check(e) {
        if continuing {
            let process = process_of(e, this);
            e.vcall(process, 0x24, &args![this, 0u32]);
        }
        schedule_in_process_lists(e, this, level, 2, animation_flag, double_at(e, HALF_DOUBLE));
        e.mem.set_u8(this.addr() + 0x126, 1);
    }
    guard_leave(e);
    if level == 0 {
        let mover = e.get(this, Actor::pActorMover).addr();
        e.call(MOVER_UPDATE_B, &args![mover]);
    }
    let process = process_of(e, this);
    e.vcall(process, 0x7a8, &args![this]);
    true
}

// Translated from 00883800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Puts the actor into the middle-high process (level 1; always true). Builds
/// a `0x25c`-byte process, re-equips the worn item and, when the actor is
/// scheduled for it, searches for references within 5000 units of the player's
/// follower or escorter.
pub fn fn_00883800(e: &mut Engine, this: Ptr<Actor>) -> bool {
    e.call(
        PROCESS_LISTS_REMOVE_CLOSE_TO_PLAYER,
        &args![PROCESS_LISTS, this],
    );
    let process = process_of(e, this);
    if process != 0 && process_level(e, process) == 1 {
        return true;
    }
    leave_high_process(e, this, false);
    end_interrupt_if_requested(e, this);
    let process = process_of(e, this);
    let current = e.vcall(process, 0x20c, &args![]).u32();
    if current != 0 && e.call(PACKAGE_GET_IS_CREATED, &args![current]).bool() {
        let process = process_of(e, this);
        let package = e.vcall(process, 0x22c, &args![]).u32();
        if package != 0 {
            if !e.call(IS_ALARMED, &args![this]).bool()
                && fn_008840f0(e, Ptr::new(package))
                && fn_008840d0(e, Ptr::new(package))
                && e.call(PARENT_CELL, &args![this]).u32() != 0
            {
                let cell = e.call(PARENT_CELL, &args![this]).u32();
                if e.call(CELL_TEST_ACTOR, &args![cell, this]).bool() {
                    actor_end_interrupt_package(e, this, false);
                }
            }
        } else if e.call(IS_ALARMED, &args![this]).bool()
            && !e.vcall(this.addr(), 0x304, &args![]).bool()
        {
            let player = e.global::<u32>(PLAYER_POINTER);
            let cell = e.call(PARENT_CELL, &args![player]).u32();
            if !e.call(FLAG_BIT_0_AT_0X24, &args![cell]).bool() {
                actor_end_interrupt_package(e, this, false);
            }
        }
    }
    let process = process_of(e, this);
    let level = process_level(e, process);
    if e.vcall(this.addr(), 0x2e8, &args![]).bool() {
        e.call(DO_DEATH_STUFF, &args![this]);
        let process = process_of(e, this);
        e.vcall(process, 0x354, &args![this]);
        e.call(RUN_SCRIPT, &args![this]);
    } else {
        let mut high = false;
        if e.vcall(this.addr(), 0x22c, &args![1u32]).bool() {
            let process = process_of(e, this);
            if process_level(e, process) == 0 {
                e.vcall(this.addr(), 0xd4, &args![1u32]);
                high = true;
            }
        }
        if !high {
            reset_dead_state(e, this, false, true);
        }
    }
    let kind = e.vcall(this.addr(), 0x214, &args![]).u32();
    if kind != 0 {
        let mut proceed = false;
        if e.vcall(this.addr(), 0x214, &args![]).u32() == 4
            || e.vcall(this.addr(), 0x214, &args![]).u32() == 9
            || {
                let process = process_of(e, this);
                e.vcall(process, 0x4c8, &args![]).u32() == 0
            }
        {
            let package = e.call(MOBILE_OBJECT_PACKAGE, &args![this]).u32();
            if package != 0 {
                let package = e.call(MOBILE_OBJECT_PACKAGE, &args![this]).u32();
                if package_type(e, package) == 0x1a {
                    proceed = true;
                }
            }
        } else {
            proceed = true;
        }
        if proceed {
            let process = process_of(e, this);
            e.vcall(process, 0x2ac, &args![this]);
            let process = process_of(e, this);
            e.vcall(process, 0x214, &args![]);
        }
    }
    if e.vcall(this.addr(), 0x214, &args![]).u32() != 0
        && e.vcall(this.addr(), 0x214, &args![]).u32() != 4
        && e.vcall(this.addr(), 0x214, &args![]).u32() != 9
        && {
            let process = process_of(e, this);
            e.vcall(process, 0x4c8, &args![]).u32() == 0
        }
    {
        let process = process_of(e, this);
        e.vcall(process, 0x4c0, &args![this, 0u32, 0u32, 0x7fu32]);
        e.set(this, Actor::cCurrentSitSleepState, 0);
    }
    if !shutdown_check(e) {
        let object = e.global::<u32>(SHARED_OBJECT_POINTER_011DE45C);
        if !e.call(ALWAYS_FALSE, &args![object]).bool() {
            e.call(PROCESS_LISTS_REFRESH_ACTOR, &args![PROCESS_LISTS, this]);
        }
    }
    guard_enter(e, 0);
    e.call(
        PROCESS_LISTS_REMOVE_REFERENCE,
        &args![PROCESS_LISTS, this, level as u32],
    );
    e.call(PROCESS_LISTS_REMOVE_ACTOR, &args![PROCESS_LISTS, this]);
    reset_dead_state(e, this, true, false);
    let continuing = e.call(IS_CONTINUING_PACKAGE_FOR_PC, &args![this]).bool();
    let new_process = construct_process(e, 0x25c, MIDDLE_HIGH_PROCESS_CONSTRUCTOR);
    swap_process(e, this, new_process);
    if e.vcall(this.addr(), 0x1d0, &args![]).u32() != 0 {
        let form = base_form(e, this).addr();
        let first = e.vcall(form, 0x180, &args![]).u32();
        let second = e.vcall(this.addr(), 0x1d0, &args![]).u32();
        let process = process_of(e, this);
        e.vcall(process, 0x6c8, &args![second, first]);
    }
    let process = process_of(e, this);
    if e.vcall(process, 0x31c, &args![]).bool() {
        e.call(ACTOR_SET_FLAG, &args![this, 1u32]);
    }
    e.call(
        PROCESS_LISTS_ADD_REFERENCE,
        &args![PROCESS_LISTS, this, 1u32, 1u32, 0u32, 0u32],
    );
    guard_leave(e);
    let list = extra_data_list(e, this);
    let changes = e.call(GET_CONTAINER_CHANGES, &args![list]).u32();
    if changes != 0 {
        let worn = worn_item(e, changes);
        if level != 0 {
            let process = process_of(e, this);
            e.vcall(process, 0x160, &args![worn, 0u32, 0u32]);
        } else if worn != 0 {
            e.call(WORN_ITEM_UPDATE, &args![worn, 1u32]);
        }
    }
    if e.global::<u32>(SHUTDOWN_CHECK_OBJECT_POINTER) != 0 && !shutdown_check(e) {
        if continuing {
            let process = process_of(e, this);
            e.vcall(process, 0x24, &args![this, 0u32]);
        }
        let animation_flag = e.call(ANIMATION_FLAG_TEST, &args![this]).bool();
        if animation_flag {
            e.call(MOVE_TO_PROCESS_LEVEL, &args![this, level as u32, 1u32]);
        } else {
            let mut due = false;
            if e.vcall(this.addr(), 0x160, &args![]).bool() && e.mem.u8(this.addr() + 0x126) != 0 {
                due = true;
            } else if actor_is_following(e, this) || actor_is_escorter_behind(e, this) {
                let process = actor_process(e, this).addr();
                if e.vcall(process, 0x128, &args![]).u32() == e.global::<u32>(PLAYER_POINTER) {
                    due = true;
                }
            }
            if due {
                let time = (calendar_hour(e) - double_at(e, HALF_DOUBLE)) as f32;
                let process = process_of(e, this);
                e.call(PROCESS_SET_TIME, &args![process, time]);
                e.call(
                    PROCESS_LISTS_REMOVE_REFERENCE,
                    &args![PROCESS_LISTS, this, 1u32],
                );
                let kind = e.call(CURRENT_PROCESS_TYPE, &args![this]).u32();
                e.call(
                    PROCESS_LISTS_ADD_REFERENCE,
                    &args![PROCESS_LISTS, this, kind, 0u32, 0u32, 0u32],
                );
            }
        }
        e.mem.set_u8(this.addr() + 0x126, 1);
        if e.call(ACTOR_TEST_008C0050, &args![this]).bool() {
            let mut follower = 0;
            let process = process_of(e, this);
            let target = e.vcall(process, 0x128, &args![]).u32();
            if target != 0 && e.vcall(target, 0x100, &args![]).bool() {
                follower = target;
            }
            if follower != 0 && !e.vcall(follower, 0x448, &args![]).bool() {
                actor_end_interrupt_package(e, this, false);
                let radius = e.global::<f32>(ENUM_RADIUS_FLOAT);
                let first = e.vcall(this.addr(), 0x1f4, &args![]).u32();
                let second = e.vcall(this.addr(), 0x1f4, &args![]).u32();
                let cell = e.call(PARENT_CELL, &args![this]).u32();
                let handler = e.global::<u32>(SHUTDOWN_CHECK_OBJECT_POINTER);
                e.call(
                    ENUM_REFERENCES_CLOSE_TO_POINT,
                    &args![
                        handler,
                        cell,
                        second,
                        radius,
                        first,
                        radius,
                        ENUM_CALLBACK,
                        this
                    ],
                );
            }
        }
    }
    if !e.vcall(this.addr(), 0x22c, &args![0u32]).bool() && (level == 3 || level == 2) {
        e.call(CAST_PERMANENT_MAGIC, &args![this, 0u32]);
    }
    if level == 0 {
        let mover = e.get(this, Actor::pActorMover).addr();
        e.call(MOVER_UPDATE_B, &args![mover]);
    }
    let process = process_of(e, this);
    e.vcall(process, 0x7a8, &args![this]);
    true
}

// Translated from 008840d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bit 0 of the flag word at `+0x1c` of `this` (a package).
pub fn fn_008840d0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0x1c) & 1 != 0
}

// Translated from 008840f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bit `0x200` of the flag word at `+0x1c` of `this` (a package).
pub fn fn_008840f0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0x1c) & 0x200 != 0
}

// Translated from 00884110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::IsEscorterBehind` (Xbox PDB): true when the actor's process has an
/// escort package (type 2, virtual `+0x27c`) whose target (virtual `+0x128`) is
/// the player, and, unless the package's location cell differs from the
/// player's (both interior), the player's offset from its escort location is
/// shorter than the actor's offset from its own.
pub fn actor_is_escorter_behind(e: &mut Engine, this: Ptr<Actor>) -> bool {
    let process = process_of(e, this);
    if process == 0 {
        return false;
    }
    let package = e.vcall(process, 0x27c, &args![]).u32();
    let target = e.vcall(process, 0x128, &args![]).u32();
    if package == 0 || package_type(e, package) != 2 || target != e.global::<u32>(PLAYER_POINTER) {
        return false;
    }
    e.with_stack(0x60, |e, frame| {
        let own_offset = frame.addr();
        let target_offset = frame.addr() + 0x0c;
        let coord_a = frame.addr() + 0x18;
        let temp_a = frame.addr() + 0x24;
        let coord_b = frame.addr() + 0x30;
        let temp_b = frame.addr() + 0x3c;
        e.call(POINT_CONSTRUCTOR, &args![own_offset]);
        e.call(POINT_CONSTRUCTOR, &args![target_offset]);
        if target == 0 {
            return false;
        }
        let coord = e
            .call(PACKAGE_LOCATION_COORD, &args![package, coord_a, this])
            .u32();
        let position = e.vcall(this.addr(), 0x1f4, &args![]).u32();
        let offset = e
            .call(POINT_SUBTRACT, &args![position, temp_a, coord])
            .u32();
        copy_point(e, offset, own_offset);
        let coord = e
            .call(PACKAGE_LOCATION_COORD, &args![package, coord_b, this])
            .u32();
        let position = e.vcall(target, 0x1f4, &args![]).u32();
        let offset = e
            .call(POINT_SUBTRACT, &args![position, temp_b, coord])
            .u32();
        copy_point(e, offset, target_offset);
        let player = e.global::<u32>(PLAYER_POINTER);
        if e.call(PARENT_CELL, &args![player]).u32() != 0 {
            let cell = e.call(PARENT_CELL, &args![player]).u32();
            if e.call(FLAG_BIT_0_AT_0X24, &args![cell]).bool()
                && e.call(PACKAGE_LOCATION_CELL, &args![package, this]).u32() != 0
            {
                let location_cell = e.call(PACKAGE_LOCATION_CELL, &args![package, this]).u32();
                if e.call(FLAG_BIT_0_AT_0X24, &args![location_cell]).bool() {
                    let location_cell = e.call(PACKAGE_LOCATION_CELL, &args![package, this]).u32();
                    let player_cell = e.call(PARENT_CELL, &args![player]).u32();
                    if location_cell != player_cell {
                        return false;
                    }
                }
            }
        }
        let own_length = e.call(POINT_LENGTH, &args![own_offset]).f64();
        let target_length = e.call(POINT_LENGTH, &args![target_offset]).f64();
        target_length < own_length
    })
}

// Translated from 008842c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::IsFollowing` (Xbox PDB): the process's package (virtual `+0x27c`)
/// is of type 1 and was not created at run time (`0067 4d40`).
pub fn actor_is_following(e: &mut Engine, this: Ptr<Actor>) -> bool {
    let process = process_of(e, this);
    if process == 0 {
        return false;
    }
    let package = e.vcall(process, 0x27c, &args![]).u32();
    package != 0
        && package_type(e, package) == 1
        && !e.call(PACKAGE_GET_IS_CREATED, &args![package]).bool()
}

// Translated from 00884320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetBarterGoldBase` (Xbox PDB): the `short` barter gold of the base
/// form's data at `+0x30` (`0047d3f0`).
pub fn actor_get_barter_gold_base(e: &mut Engine, this: Ptr<Actor>) -> u16 {
    let form = base_form(e, this).addr();
    e.call(BARTER_GOLD, &args![form + 0x30]).u16()
}

// Translated from 00884350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetClass` (Xbox PDB): 0 unless the actor's virtual `+0x218` is true
/// and it has a form, whose class (`00502430`) is returned.
pub fn actor_get_class(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    if e.vcall(this.addr(), 0x218, &args![]).bool() {
        let form = e.call(GET_FORM, &args![this]).u32();
        if form != 0 {
            return e.call(ACTOR_BASE_CLASS, &args![form]).u32();
        }
    }
    0
}

// Translated from 008843a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::CanMove` (Xbox PDB): false when the actor's virtual `+0x234` or
/// `+0x22c(0)` is true, its action animation is 12 or 13, its virtual
/// `+0x230` is true, either block test (`00437bf0`, `00437bd0`) holds, its
/// virtual `+0x214` is nonzero, or it is the player with the Pip-Boy active.
pub fn actor_can_move(e: &mut Engine, this: Ptr<Actor>) -> bool {
    if e.vcall(this.addr(), 0x234, &args![]).bool()
        || e.vcall(this.addr(), 0x22c, &args![0u32]).bool()
    {
        return false;
    }
    let action = e.call(GET_ANIM_ACTION, &args![this]).i32();
    if (12..=13).contains(&action) {
        return false;
    }
    if e.vcall(this.addr(), 0x230, &args![]).bool() {
        return false;
    }
    if e.call(ACTOR_BLOCK_TEST_A, &args![this]).bool()
        || e.call(ACTOR_BLOCK_TEST_B, &args![this]).bool()
    {
        return false;
    }
    if e.vcall(this.addr(), 0x214, &args![]).u32() != 0 {
        return false;
    }
    let player = e.global::<u32>(PLAYER_POINTER);
    if this.addr() == player && e.call(IS_PIPBOY_ACTIVE, &args![player]).bool() {
        return false;
    }
    true
}

// Translated from 00884480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::CanSpeak` (Xbox PDB): false when the actor's virtual `+0x234` or
/// `+0x22c(0)` or `+0x230` is true, or `00437bd0` holds.
pub fn actor_can_speak(e: &mut Engine, this: Ptr<Actor>) -> bool {
    if e.vcall(this.addr(), 0x234, &args![]).bool()
        || e.vcall(this.addr(), 0x22c, &args![0u32]).bool()
    {
        return false;
    }
    if e.vcall(this.addr(), 0x230, &args![]).bool() {
        return false;
    }
    !e.call(ACTOR_BLOCK_TEST_B, &args![this]).bool()
}

// Translated from 008844f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when `004f8960` says 2 or 1, or, when `exclude_six` is false, 6
/// (the actor's slot `+0x22c`).
pub fn fn_008844f0(e: &mut Engine, this: Ptr<Actor>, exclude_six: bool) -> bool {
    let kind = |e: &mut Engine| e.call(ACTOR_STATE_QUERY, &args![this]).i32();
    if exclude_six {
        kind(e) == 2 || kind(e) == 1
    } else {
        kind(e) == 2 || kind(e) == 1 || kind(e) == 6
    }
}

// Translated from 00884560 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when the actor has a process whose virtual `+0x40c` is nonzero.
pub fn fn_00884560(e: &mut Engine, this: Ptr<Actor>) -> bool {
    let process = process_of(e, this);
    process != 0 && e.vcall(process, 0x40c, &args![]).u32() != 0
}

// Translated from 008845a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::CanKnockDown` (Xbox PDB): false when the actor's virtual `+0x22c(1)`
/// or `+0x4b4` is true, its base form's data (at `+0x30`) forbids it
/// (`00884690`), its process's virtual `+0x3e4` is 9 or `0x11`, or its virtual
/// `+0x1e4` result passes `008846c0`.
pub fn actor_can_knock_down(e: &mut Engine, this: Ptr<Actor>) -> bool {
    if e.vcall(this.addr(), 0x22c, &args![1u32]).bool() {
        return false;
    }
    if e.vcall(this.addr(), 0x4b4, &args![]).bool() {
        return false;
    }
    let form = base_form(e, this).addr();
    if !fn_00884690(e, Ptr::new(form + 0x30)) {
        return false;
    }
    let process = process_of(e, this);
    if process != 0 {
        if e.vcall(process, 0x3e4, &args![]).i32() == 9 {
            return false;
        }
        let process = process_of(e, this);
        if e.vcall(process, 0x3e4, &args![]).i32() == 0x11 {
            return false;
        }
    }
    if e.vcall(this.addr(), 0x1e4, &args![]).u32() != 0 {
        let value = e.vcall(this.addr(), 0x1e4, &args![]).u32();
        if fn_008846c0(e, Ptr::new(value)) {
            return false;
        }
    }
    true
}

// Translated from 00884690 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when `00461580(this, 0x4000000)` is false (a flag test of a base-form
/// data block).
pub fn fn_00884690(e: &mut Engine, this: Ptr) -> bool {
    !e.call(BASE_FLAG_TEST, &args![this, 0x0400_0000u32]).bool()
}

// Translated from 008846c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when the `u16` at `+0x122` of `this` is not `0xff`.
pub fn fn_008846c0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u16(this.addr() + 0x122) != 0xff
}

// Translated from 008846e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The actor mover's virtual `+0x20` (the movement flags), 0 without a mover.
pub fn fn_008846e0(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    let mover = e.get(this, Actor::pActorMover).addr();
    if mover == 0 {
        0
    } else {
        e.vcall(mover, 0x20, &args![]).u32()
    }
}

// Translated from 00884730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::IsRunning` (Xbox PDB): bit `0x200` of the movement flags
/// (`008846e0`).
pub fn actor_is_running(e: &mut Engine, this: Ptr<Actor>) -> bool {
    fn_008846e0(e, this) & 0x200 != 0
}

// Translated from 00884750 (decompiled, FalloutNV.exe 1.4.0.525)
/// False when `0047c850` holds, the action animation is 7, or the actor is the
/// player and its process's virtual `+0x404` is true; true otherwise.
pub fn fn_00884750(e: &mut Engine, this: Ptr<Actor>) -> bool {
    if e.call(ALWAYS_FALSE, &args![this]).bool() {
        return false;
    }
    if e.call(GET_ANIM_ACTION, &args![this]).i32() == 7 {
        return false;
    }
    let process = process_of(e, this);
    if this.addr() == e.global::<u32>(PLAYER_POINTER)
        && process != 0
        && e.vcall(process, 0x404, &args![]).bool()
    {
        return false;
    }
    true
}

// Translated from 008847c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `this` is the sub-object at `Actor + 0xa8`. True when both of the actor
/// value owner's (`this - 4`, null for a null actor) body part conditions 29
/// and 30 are `0.0`, or when the owner's virtual `+8` with `0x48` is nonzero;
/// the result is also written into the process's flag block (`00884880`).
pub fn fn_008847c0(e: &mut Engine, this: Ptr) -> bool {
    let owner = if this.addr() == 0xa8 {
        0
    } else {
        this.addr() - 4
    };
    let zero = double_at(e, ZERO_DOUBLE);
    let mut result = false;
    let first = e
        .call(GET_BODY_PART_CONDITION, &args![owner, 0x1du32, 0u32])
        .f64();
    let mut check_slot = true;
    if first == zero {
        let owner = if this.addr() == 0xa8 {
            0
        } else {
            this.addr() - 4
        };
        let second = e
            .call(GET_BODY_PART_CONDITION, &args![owner, 0x1eu32, 0u32])
            .f64();
        if second == zero {
            result = true;
            check_slot = false;
        }
    }
    if check_slot && e.vcall(this.addr() - 4, 8, &args![0x48u32]).u32() != 0 {
        result = true;
    }
    let process = e.mem.u32(this.addr() - 0x40);
    if process != 0 {
        fn_00884880(e, Ptr::new(process), result as u8);
    }
    result
}

// Translated from 00884880 (decompiled, FalloutNV.exe 1.4.0.525)
/// With a flag block at `+0x2c`, stores `value` in its byte at `+0x40` and sets
/// bit `0x4000` of its word at `+0x44` (`008848c0`).
pub fn fn_00884880(e: &mut Engine, this: Ptr, value: u8) {
    let block = e.mem.u32(this.addr() + 0x2c);
    if block != 0 {
        e.mem.set_u8(block + 0x40, value);
        fn_008848c0(e, Ptr::new(block), 0x4000);
    }
}

// Translated from 008848c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the bits `mask` in the word at `+0x44` of `this`.
pub fn fn_008848c0(e: &mut Engine, this: Ptr, mask: u32) {
    let flags = e.mem.u32(this.addr() + 0x44);
    e.mem.set_u32(this.addr() + 0x44, flags | mask);
}

// Translated from 008848e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// With a process that has a flag block (`+0x2c`), clears bit `0x4000` of its
/// flags (`00884940`).
pub fn fn_008848e0(e: &mut Engine, this: Ptr<Actor>) {
    let process = process_of(e, this);
    if process != 0 && fn_00884920(e, Ptr::new(process)) {
        fn_00884940(e, Ptr::new(process), 0x4000);
    }
}

// Translated from 00884920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the word at `+0x2c` of `this` is nonzero.
pub fn fn_00884920(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0x2c) != 0
}

// Translated from 00884940 (decompiled, FalloutNV.exe 1.4.0.525)
/// With a flag block at `+0x2c`, clears the bits `mask` of it (`00884970`).
pub fn fn_00884940(e: &mut Engine, this: Ptr, mask: u32) {
    let block = e.mem.u32(this.addr() + 0x2c);
    if block != 0 {
        fn_00884970(e, Ptr::new(block), mask);
    }
}

// Translated from 00884970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the bits `mask` in the word at `+0x44` of `this`.
pub fn fn_00884970(e: &mut Engine, this: Ptr, mask: u32) {
    let flags = e.mem.u32(this.addr() + 0x44);
    e.mem.set_u32(this.addr() + 0x44, !mask & flags);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004b4880, ni_transform_invert_non_uniform(Ptr, Ptr)),
        entry!(
            0x00575610,
            actor_get_item_countin_container(Ptr<Actor>, Ptr) -> u32
        ),
        entry!(0x0087d4a0, actor_constructor(Ptr<Actor>, bool) -> Ptr),
        entry!(0x0087d860, fn_0087d860(Ptr) -> Ptr),
        entry!(0x0087d890, fn_0087d890(Ptr<Actor>, bool)),
        entry!(0x0087dcb0, actor_destructor(Ptr<Actor>)),
        entry!(0x0087e060, fn_0087e060(Ptr<Actor>, bool) -> Ptr),
        entry!(0x0087e100, fn_0087e100(Ptr, bool)),
        entry!(
            0x0087e130,
            actor_create_ragdoll_instance(Ptr<Actor>, Ptr, bool, bool, bool)
        ),
        entry!(0x0087e920, fn_0087e920(Ptr) -> bool),
        entry!(0x0087e940, fn_0087e940(Ptr) -> bool),
        entry!(0x0087e960, fn_0087e960(Ptr<Actor>, bool)),
        entry!(0x0087e980, fn_0087e980(Ptr) -> u32),
        entry!(0x0087e9a0, fn_0087e9a0() -> u32),
        entry!(0x0087e9b0, fn_0087e9b0(Ptr)),
        entry!(0x0087e9d0, fn_0087e9d0(Ptr, bool)),
        entry!(0x0087ea20, fn_0087ea20(Ptr) -> u8),
        entry!(0x0087ea40, fn_0087ea40(Ptr, u32)),
        entry!(0x0087ea60, fn_0087ea60(Ptr, u8)),
        entry!(0x0087ea80, fn_0087ea80(Ptr, Ptr)),
        entry!(0x0087eaa0, fn_0087eaa0(Ptr) -> Ptr),
        entry!(0x0087eac0, fn_0087eac0(Ptr<Actor>, Ptr, bool, bool)),
        entry!(0x0087eeb0, fn_0087eeb0(Ptr<Actor>) -> u8),
        entry!(0x0087eed0, fn_0087eed0(Ptr) -> u8),
        entry!(0x0087eef0, fn_0087eef0(Ptr<Actor>)),
        entry!(0x0087f200, fn_0087f200(Ptr<Actor>) -> bool),
        entry!(0x0087f260, fn_0087f260(Ptr<Actor>) -> bool),
        entry!(0x0087f350, fn_0087f350(Ptr<Actor>) -> bool),
        entry!(0x0087f370, fn_0087f370(Ptr<Actor>) -> bool),
        entry!(0x0087f390, fn_0087f390(Ptr<Actor>) -> bool),
        entry!(0x0087f3b0, fn_0087f3b0(Ptr<Actor>) -> bool),
        entry!(0x0087f3d0, actor_get_essential(Ptr<Actor>) -> bool),
        entry!(0x0087f480, fn_0087f480(Ptr) -> bool),
        entry!(0x0087f4a0, fn_0087f4a0(Ptr<Actor>) -> bool),
        entry!(0x0087f4c0, fn_0087f4c0(Ptr<Actor>) -> i32),
        entry!(0x0087f4e0, fn_0087f4e0(Ptr<Actor>) -> u32),
        entry!(0x0087f510, fn_0087f510(Ptr<Actor>) -> u8),
        entry!(0x0087f570, actor_swims_only(Ptr<Actor>) -> bool),
        entry!(0x0087f5c0, fn_0087f5c0(Ptr<Actor>) -> bool),
        entry!(0x0087f620, fn_0087f620(Ptr<Actor>) -> bool),
        entry!(0x0087f660, actor_check_breath_timer(Ptr<Actor>) -> bool),
        entry!(0x0087f6c0, fn_0087f6c0(Ptr<Actor>) -> u32),
        entry!(0x0087f750, fn_0087f750(Ptr<Actor>) -> Ptr),
        entry!(0x0087f7a0, fn_0087f7a0(Ptr<Actor>) -> Ptr),
        entry!(0x0087f800, fn_0087f800(Ptr<Actor>)),
        entry!(0x0087f890, fn_0087f890(Ptr<Actor>, Ptr, Ptr, Ptr, f32)),
        entry!(0x0087f900, fn_0087f900(Ptr<Actor>) -> bool),
        entry!(
            0x0087f920,
            fn_0087f920(Ptr<Actor>, Ptr, Ptr, Ptr, u32) -> bool
        ),
        entry!(0x0087f990, fn_0087f990(Ptr<Actor>) -> bool),
        entry!(0x0087f9c0, fn_0087f9c0(Ptr<Actor>) -> f32),
        entry!(0x0087f9f0, fn_0087f9f0(Ptr<Actor>) -> u32),
        entry!(0x0087fa10, fn_0087fa10(Ptr<Actor>, bool)),
        entry!(0x0087fa60, fn_0087fa60(Ptr<Actor>, bool)),
        entry!(0x0087faa0, actor_end_movement(Ptr<Actor>)),
        entry!(0x0087fad0, fn_0087fad0(Ptr<Actor>, u32)),
        entry!(0x0087fb40, fn_0087fb40(Ptr<Actor>, Ptr, f32)),
        entry!(0x0087fcb0, fn_0087fcb0(Ptr<Actor>, u32) -> f32),
        entry!(0x0087fd20, actor_clear_disposition_modifiers(Ptr<Actor>)),
        entry!(0x0087fd90, fn_0087fd90(Ptr<Actor>, Ptr<Actor>, Ptr) -> i32),
        entry!(0x00880370, fn_00880370(Ptr, u32) -> i32),
        entry!(0x008803a0, fn_008803a0(Ptr, u32) -> f32),
        entry!(0x00880580, fn_00880580(Ptr, u32) -> u32),
        entry!(0x008805f0, fn_008805f0(Ptr, u32) -> f32),
        entry!(0x00880660, fn_00880660(Ptr<Actor>, u8, u32) -> f32),
        entry!(0x00880690, fn_00880690(Ptr<Actor>, u8, f32)),
        entry!(0x008806d0, fn_008806d0(Ptr<Actor>, u32, i32)),
        entry!(0x00880700, fn_00880700(Ptr<Actor>, u32, f32)),
        entry!(0x00880850, fn_00880850(Ptr, u32, i32, u32) -> i32),
        entry!(0x00880890, fn_00880890(Ptr, u32, f32, u32) -> f32),
        entry!(
            0x008808a0,
            actor_difficulty_level_adjust_health_modifier(Ptr<Actor>, f32, u32) -> f32
        ),
        entry!(0x008808f0, fn_008808f0(Ptr, u32, u32) -> u32),
        entry!(0x00880910, fn_00880910(Ptr<Actor>) -> bool),
        entry!(0x00880950, fn_00880950(Ptr<Actor>, u32, i32, Ptr<Actor>)),
        entry!(0x00880ad0, fn_00880ad0(Ptr<Actor>, u32, f32, Ptr<Actor>)),
        entry!(0x00880c70, fn_00880c70(Ptr<Actor>, u32, i32, Ptr<Actor>)),
        entry!(0x00880e00, fn_00880e00(Ptr<Actor>, u32, f32, Ptr<Actor>)),
        entry!(0x00880fb0, fn_00880fb0(Ptr<Actor>, u32, i32, Ptr<Actor>)),
        entry!(0x00881130, fn_00881130(Ptr<Actor>, u32, f32, Ptr<Actor>)),
        entry!(0x00881330, fn_00881330(Ptr<Actor>, u32, i32)),
        entry!(0x00881360, fn_00881360(Ptr<Actor>, u32, f32)),
        entry!(0x00881450, actor_get_package(Ptr<Actor>) -> Ptr),
        entry!(0x00881470, actor_get_missed_packs(Ptr<Actor>, u32, f32)),
        entry!(
            0x008814b0,
            actor_get_current_editor_package(Ptr<Actor>) -> Ptr
        ),
        entry!(
            0x00881510,
            actor_get_package_set_as_pcurrent(Ptr<Actor>) -> Ptr
        ),
        entry!(0x00881570, actor_is_running_run_once(Ptr<Actor>) -> bool),
        entry!(0x008815a0, fn_008815a0(Ptr<Actor>) -> Ptr),
        entry!(0x00881620, actor_set_current_target(Ptr<Actor>, u32)),
        entry!(
            0x00881650,
            actor_get_current_package_target(Ptr<Actor>) -> u32
        ),
        entry!(0x00881680, actor_end_interrupt_package(Ptr<Actor>, bool)),
        entry!(0x00881830, fn_00881830(Ptr, i32)),
        entry!(0x00881860, fn_00881860(Ptr<Actor>, bool) -> bool),
        entry!(0x00881c90, fn_00881c90(Ptr<Actor>, u32) -> bool),
        entry!(0x00881d10, fn_00881d10(Ptr<Actor>)),
        entry!(0x00881d30, fn_00881d30(Ptr<Actor>) -> bool),
        entry!(0x00882b90, fn_00882b90(Ptr<Actor>) -> bool),
        entry!(0x00883240, fn_00883240(Ptr<Actor>) -> bool),
        entry!(0x00883800, fn_00883800(Ptr<Actor>) -> bool),
        entry!(0x008840d0, fn_008840d0(Ptr) -> bool),
        entry!(0x008840f0, fn_008840f0(Ptr) -> bool),
        entry!(0x00884110, actor_is_escorter_behind(Ptr<Actor>) -> bool),
        entry!(0x008842c0, actor_is_following(Ptr<Actor>) -> bool),
        entry!(0x00884320, actor_get_barter_gold_base(Ptr<Actor>) -> u16),
        entry!(0x00884350, actor_get_class(Ptr<Actor>) -> u32),
        entry!(0x008843a0, actor_can_move(Ptr<Actor>) -> bool),
        entry!(0x00884480, actor_can_speak(Ptr<Actor>) -> bool),
        entry!(0x008844f0, fn_008844f0(Ptr<Actor>, bool) -> bool),
        entry!(0x00884560, fn_00884560(Ptr<Actor>) -> bool),
        entry!(0x008845a0, actor_can_knock_down(Ptr<Actor>) -> bool),
        entry!(0x00884690, fn_00884690(Ptr) -> bool),
        entry!(0x008846c0, fn_008846c0(Ptr) -> bool),
        entry!(0x008846e0, fn_008846e0(Ptr<Actor>) -> u32),
        entry!(0x00884730, actor_is_running(Ptr<Actor>) -> bool),
        entry!(0x00884750, fn_00884750(Ptr<Actor>) -> bool),
        entry!(0x008847c0, fn_008847c0(Ptr) -> bool),
        entry!(0x00884880, fn_00884880(Ptr, u8)),
        entry!(0x008848c0, fn_008848c0(Ptr, u32)),
        entry!(0x008848e0, fn_008848e0(Ptr<Actor>)),
        entry!(0x00884920, fn_00884920(Ptr) -> bool),
        entry!(0x00884940, fn_00884940(Ptr, u32)),
        entry!(0x00884970, fn_00884970(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    fn ret(value: u32) -> Ret {
        value.into_ret()
    }

    /// An engine whose listed callees are doubles returning zero.
    fn engine(addrs: &[u32]) -> Engine {
        let mut e = Engine::new();
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
        e
    }

    /// The address a test vtable slot jumps to.
    fn slot_target(vtable: u32, offset: u32) -> u32 {
        0x6000_0000 + (vtable & 0x00ff_ffff) + offset
    }

    /// An object of `size` bytes whose vtable (at `vtable`, a page-aligned
    /// address) has the listed slots, each a double returning zero.
    fn object(e: &mut Engine, size: u32, vtable: u32, offsets: &[u32]) -> Ptr {
        e.map(vtable, 0x1000);
        for offset in offsets {
            let target = slot_target(vtable, *offset);
            e.mem.set_u32(vtable + offset, target);
            e.register(target, |_, _| Ret::default());
        }
        let object = Ptr::new(e.mem.alloc(size));
        e.mem.set_u32(object.addr(), vtable);
        object
    }

    /// The addresses called so far (the test's own top-level call included).
    fn called(e: &Engine) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(addr, _)| *addr)
            .collect()
    }

    /// The argument words of the first call to `addr`.
    fn args_of(e: &Engine, addr: u32) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .find(|(a, _)| *a == addr)
            .unwrap_or_else(|| panic!("{addr:08x} was not called"))
            .1
            .clone()
    }

    fn map_pages(e: &mut Engine, pages: &[u32]) {
        for page in pages {
            e.map(*page, 0x1000);
        }
    }

    // ---- 004b4880 ----

    #[test]
    fn invert_non_uniform_transposes_inverts_the_scale_and_rebuilds_the_translation() {
        let mut e = Engine::new();
        let transposed = e.mem.alloc(0x24);
        for i in 0..9 {
            e.mem.set_u32(transposed + 4 * i, 0x100 + i);
        }
        let scaled = e.mem.alloc(0x10);
        for i in 0..3 {
            e.mem.set_u32(scaled + 4 * i, 0x200 + i);
        }
        e.register_double(0x0047_68c0, move |_, _| ret(transposed));
        e.register(0x004a_0bd0, |_, _| ret(0x7001));
        e.register(0x004b_4500, |_, _| ret(0x7002));
        e.register_double(0x004a_3760, move |_, _| ret(scaled));
        let source = e.mem.alloc(0x40);
        e.mem.set_f32(source + 0x30, 4.0);
        let result = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(
            0x004b_4880,
            &args![Ptr::<()>::new(source), Ptr::<()>::new(result)],
        );
        for i in 0..9 {
            assert_eq!(e.mem.u32(result + 4 * i), 0x100 + i);
        }
        assert_eq!(e.mem.f32(result + 0x30), 0.25);
        for i in 0..3 {
            assert_eq!(e.mem.u32(result + 0x24 + 4 * i), 0x200 + i);
        }
        // The translation goes through the helpers: negate the translate at
        // +0x24 of the source, rotate it by the result, scale by 1/scale.
        let negate = args_of(&e, 0x004a_0bd0);
        assert_eq!(negate[0], source + 0x24);
        let rotate = args_of(&e, 0x004b_4500);
        assert_eq!((rotate[0], rotate[2]), (result, 0x7001));
        let scale = args_of(&e, 0x004a_3760);
        assert_eq!((scale[1], scale[2]), (0.25f32.to_bits(), 0x7002));
    }

    // ---- 00881d30 (knock-down) ----

    /// The knock-down's rotation matrix is a whole NiMatrix3: building the (0, 1, 0)
    /// vector does not write over its second row before the transform reads it.
    #[test]
    fn knock_down_keeps_the_whole_rotation_matrix_apart_from_the_vector() {
        let mut e = engine(&[DO_KNOCK_DOWN, POINT_CONSTRUCTOR]);
        let vtable = e.mem.alloc(0x400);
        e.mem.set_u32(vtable + 0x2bc, 0x0f20_0001);
        e.mem.set_u32(vtable + 0x48, 0x0f20_0002);
        e.register(0x0f20_0001, |_, _| Ret {
            st0: 0.5,
            ..Ret::default()
        });
        e.register(0x0f20_0002, |_, _| Ret::default());
        let actor = e.mem.alloc(0x200);
        e.mem.set_u32(actor, vtable);
        // The matrix gets nine distinct words; the vector its three floats.
        e.register(MATRIX_SET_ANGLE, |e, a| {
            for i in 0..9 {
                e.mem.set_u32(a[0] + 4 * i, 0x100 + i);
            }
            Ret::default()
        });
        e.register(VECTOR_BUILD, |e, a| {
            for i in 0..3 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            Ret::default()
        });
        let seen = Rc::new(RefCell::new(vec![]));
        let record = seen.clone();
        e.register_double(MATRIX_TRANSFORM, move |e, a| {
            record
                .borrow_mut()
                .extend((0..9).map(|i| e.mem.u32(a[0] + 4 * i)));
            ret(a[2])
        });
        knock_down(&mut e, Ptr::new(actor), 0x1234, false);
        assert_eq!(
            *seen.borrow(),
            (0..9).map(|i| 0x100 + i).collect::<Vec<_>>()
        );
    }

    // ---- 00575610 ----

    #[test]
    fn get_item_countin_container_is_zero_without_inventory_changes() {
        let mut e = engine(&[0x004b_f220]);
        e.register(0x004c_8f30, |_, _| ret(9));
        let actor: Ptr<Actor> = e.new_object();
        let item = Ptr::<()>::new(0x1234);
        assert_eq!(e.call(0x0057_5610, &args![actor, item]).u32(), 0);
    }

    #[test]
    fn get_item_countin_container_asks_the_inventory_changes() {
        let mut e = engine(&[]);
        e.register(0x004b_f220, |_, _| ret(0x5000));
        e.register(0x004c_8f30, |_, a| ret(a[0] + a[1]));
        let actor: Ptr<Actor> = e.new_object();
        let item = Ptr::<()>::new(7);
        assert_eq!(e.call(0x0057_5610, &args![actor, item]).u32(), 0x5007);
    }

    // ---- 0087d860 ----

    #[test]
    fn fn_0087d860_installs_its_vtable() {
        let mut e = Engine::new();
        let part = Ptr::<()>::new(e.mem.alloc(8));
        assert_eq!(e.call(0x0087_d860, &args![part]).ptr::<()>(), part);
        assert_eq!(e.mem.u32(part.addr()), 0x0108_4734);
    }

    // ---- 0087d890 and 0087d4a0 ----

    /// Doubles for everything the initializer calls, and the pages that hold
    /// the constants and settings it reads.
    fn init_engine(process_block: u32) -> Engine {
        let mut e = engine(&[
            0x0046_a010,
            0x008a_08e0,
            0x008b_bdb0,
            0x008b_be00,
            0x008c_4400,
            0x0093_7090,
            0x0096_e870,
            0x0096_d450,
            0x0092_eb50,
            0x0081_5300,
            0x0082_2a40,
            0x005f_78e0,
            0x0093_7000,
            0x0096_a2d0,
            0x0068_15c0,
        ]);
        map_pages(
            &mut e,
            &[0x0101_2000, 0x0101_7000, 0x011f_4000, 0x011d_f000],
        );
        e.set_global(0x0101_2054, -1.0f32);
        e.set_global(0x0101_7718, 3.0f32);
        e.set_global(0x011f_426c, 0x1111u32);
        e.set_global(0x011f_4270, 0x2222u32);
        e.set_global(0x011f_4274, 0x3333u32);
        e.set_global(0x011d_f810, 2.0f32);
        e.set_global(0x011d_f828, 5.0f32);
        e.register_double(0x0040_1000, move |_, _| ret(process_block));
        e.register(0x0090_6dc0, |_, a| ret(a[0]));
        e.register(0x0055_3300, |e, _| ret(e.mem.alloc(0x30)));
        e.register(0x00ca_0dc0, |_, a| ret(a[0]));
        // Settings: the value pointer is the setting's own address.
        e.register(0x0040_3e20, |_, a| ret(a[0]));
        // RandomFloat(low, high): encodes the argument order.
        e.register(0x0047_6b70, |_, a| {
            (f32::from_bits(a[0]) * 10.0 + f32::from_bits(a[1])).into_ret()
        });
        e
    }

    fn init_actor(e: &mut Engine) -> Ptr<Actor> {
        let actor = object(e, 0x1b4, 0x0200_0000, &[0x4d0, 0x3c4]);
        // Stale values the initializer must overwrite.
        for offset in (0x68..0x1b4).step_by(4) {
            e.mem.set_u32(actor.addr() + offset, 0xdead_beef);
        }
        actor.cast()
    }

    #[test]
    fn initializer_sets_every_field_and_registers_with_the_process_lists() {
        let mut e = init_engine(0x5000);
        let actor = init_actor(&mut e);
        e.register(0x008c_7aa0, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(0x0087_d890, &args![actor, true]);
        assert_eq!(e.get(actor, Actor::pCurrentProcess), Ptr::new(0x5000));
        assert_eq!(e.get(actor, Actor::fLastUpdate), -1.0);
        assert_eq!(e.get(actor, Actor::ePersuasionEmotion), 8);
        assert!(e.get(actor, Actor::bProcessMe));
        assert_eq!(e.get(actor, Actor::eQueuedattack), 0xff);
        assert_eq!(e.get(actor, Actor::fDeadBodyAlarm), 3.0);
        // RandomFloat(setting at 011df810, setting at 011df828).
        assert_eq!(e.get(actor, Actor::fUpdateTargetTimer), 25.0);
        assert_eq!(e.mem.u32(actor.addr() + 0x160), 0x1111);
        assert_eq!(e.mem.u32(actor.addr() + 0x168), 0x3333);
        assert!(e.get(actor, Actor::bShouldRotateToTrack));
        assert!(e.get(actor, Actor::bContainerReset));

        assert_eq!(e.get(actor, Actor::pContinuousBeamPersistant), Ptr::NULL);
        assert_eq!(e.get(actor, Actor::pLastHitData), Ptr::NULL);
        assert_eq!(e.get(actor, Actor::pInitialPackage), Ptr::NULL);
        assert_eq!(e.get(actor, Actor::iMinorCrimes), 0);
        assert_eq!(e.get(actor, Actor::pRagdollController), Ptr::NULL);
        assert!(!e.get(actor, Actor::pPenetrationDetection).is_null());
        // Not in the temp-change list: the plain AddReference(actor, 3, 0, 0, 0).
        assert_eq!(
            args_of(&e, 0x0096_d450),
            vec![PROCESS_LISTS, actor.addr(), 3, 0, 0, 0]
        );
        let calls = called(&e);
        assert!(!calls.contains(&0x0096_e870));
        // The clear-in-combat call comes after the virtual at +0x4d0.
        let pos = |addr| calls.iter().position(|a| *a == addr).unwrap();
        assert!(pos(0x008a_08e0) > pos(slot_target(0x0200_0000, 0x4d0)));
        assert!(pos(0x008b_bdb0) > pos(0x00ca_0dc0));
        assert_eq!(args_of(&e, 0x0093_7090), vec![actor.addr() + 0xe0, 1]);
    }

    #[test]
    fn initializer_uses_the_temp_change_list_and_survives_failed_allocations() {
        let mut e = init_engine(0);
        let actor = init_actor(&mut e);
        e.register(0x008c_7aa0, |_, _| ret(1));
        e.register(0x0055_3300, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(0x0087_d890, &args![actor, true]);
        assert_eq!(e.get(actor, Actor::pCurrentProcess), Ptr::NULL);
        assert_eq!(e.get(actor, Actor::pPenetrationDetection), Ptr::NULL);
        assert_eq!(args_of(&e, 0x0096_e870), vec![PROCESS_LISTS, actor.addr()]);
        assert!(!called(&e).contains(&0x0096_d450));
        assert!(!called(&e).contains(&0x0090_6dc0));
        assert!(!called(&e).contains(&0x00ca_0dc0));
    }

    #[test]
    fn initializer_skips_the_process_lists_when_not_asked() {
        let mut e = init_engine(0x5000);
        let actor = init_actor(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0087_d890, &args![actor, false]);
        let calls = called(&e);
        assert!(!calls.contains(&0x0096_e870) && !calls.contains(&0x0096_d450));
        assert_eq!(args_of(&e, 0x0046_a010), vec![actor.addr(), 1]);
    }

    #[test]
    fn constructor_builds_the_parts_installs_the_vtables_and_initializes() {
        let mut e = init_engine(0x5000);
        e.register(0x008c_7aa0, |_, _| ret(0));
        let actor = Ptr::<Actor>::new(e.mem.alloc(0x1b4));
        // The initializer calls the virtuals at +0x4d0 and +0x3c4 of the new
        // vtable (01084254), which the test supplies.
        e.map(0x0108_4000, 0x1000);
        e.register(0x7000_0001, |_, _| Ret::default());
        e.mem.set_u32(0x0108_4254 + 0x4d0, 0x7000_0001);
        e.mem.set_u32(0x0108_4254 + 0x3c4, 0x7000_0001);
        e.call_log = Some(vec![]);
        let back = e.call(0x0087_d4a0, &args![actor, true]).ptr::<Actor>();
        assert_eq!(back, actor);
        for (offset, vtable) in [
            (0x00, 0x0108_4254),
            (0x18, 0x0108_4248),
            (0x88, 0x0108_41f4),
            (0x94, 0x0108_41bc),
            (0xa4, 0x0108_418c),
            (0xa8, 0x0108_4144),
        ] {
            assert_eq!(e.mem.u32(actor.addr() + offset), vtable);
        }
        let calls = called(&e);
        let pos = |addr| calls.iter().position(|a| *a == addr).unwrap();
        // Base first, parts in offset order, the initializer last.
        assert_eq!(args_of(&e, 0x0092_eb50), vec![actor.addr()]);
        assert!(pos(0x0092_eb50) < pos(0x0081_5300));
        assert!(pos(0x0081_5300) < pos(0x0082_2a40));
        assert!(pos(0x0082_2a40) < pos(0x005f_78e0));
        assert!(pos(0x005f_78e0) < pos(0x0093_7000));
        assert!(pos(0x0096_a2d0) < pos(0x0068_15c0));
        assert!(pos(0x0068_15c0) < pos(0x0046_a010));
        assert_eq!(e.mem.u32(actor.addr() + 0xa8), 0x0108_4144);
    }

    // ---- 0087dcb0 ----

    const DESTRUCTOR_CALLEES: [u32; 38] = [
        0x0041_a250,
        0x0041_8890,
        0x00ad_8ce0,
        0x00ad_8d10,
        0x0083_5980,
        0x0040_77c0,
        REMOVE_FROM_PLAYER_COMBAT_LIST,
        REMOVE_PERCEIVED_ACTOR,
        0x0096_e6f0,
        0x0087_fd20,
        0x0090_6050,
        0x0097_8660,
        0x0042_26e0,
        0x0082_4970,
        0x0066_e100,
        0x0092_ed60,
        0x0088_1680,
        0x0097_6680,
        0x0096_f600,
        0x0097_5320,
        0x0045_cd60,
        0x00b4_f5c0,
        0x0084_e3a0,
        0x00b6_6050,
        0x008c_30a0,
        0x008c_51f0,
        0x0096_f400,
        STOP_COMBAT,
        0x008c_5090,
        0x0093_70b0,
        0x008b_3180,
        SET_3D,
        0x0048_3710,
        0x0046_ffb0,
        0x0093_7030,
        0x0082_2a80,
        0x0081_53b0,
        0x0092_ec40,
    ];

    fn destructor_setup() -> (Engine, Ptr<Actor>, Ptr, Ptr) {
        let mut e = engine(&DESTRUCTOR_CALLEES);
        e.register(GET_EXTRA_DATA_LIST, |_, a| ret(a[0] + 0x44));
        let actor: Ptr<Actor> = object(&mut e, 0x1b4, 0x0200_0000, &[]).cast();
        let ragdoll = object(&mut e, 0x10, 0x0201_0000, &[0]);
        let detection = object(&mut e, 0x10, 0x0202_0000, &[0]);
        e.set(actor, Actor::pRagdollController, ragdoll);
        e.set(actor, Actor::pPenetrationDetection, detection);
        e.map(0x011d_e000, 0x1000);
        e.map(0x011c_3000, 0x1000);
        e.set_global(PLAYER_POINTER, 0x7777u32);
        e.set_global(SHUTDOWN_CHECK_OBJECT_POINTER, 0x8888u32);
        (e, actor, ragdoll, detection)
    }

    #[test]
    fn destructor_takes_the_actor_out_of_every_list_and_runs_the_member_destructors() {
        let (mut e, actor, ragdoll, detection) = destructor_setup();
        let process = Ptr::<()>::new(e.mem.alloc(0x40));
        e.set(actor, Actor::pCurrentProcess, process);
        e.set(actor, Actor::bInCombat, true);
        e.register(0x0045_cd60, |_, _| ret(1));
        e.register(0x00b4_f5c0, |_, _| ret(0x9000));
        e.register(0x0084_e3a0, |_, _| ret(0x9100));
        e.register(0x00ad_8ce0, |_, _| ret(1));
        e.call_log = Some(vec![]);
        e.call(0x0087_dcb0, &args![actor]);
        for (offset, vtable) in [
            (0x00, 0x0108_4254),
            (0x18, 0x0108_4248),
            (0x88, 0x0108_41f4),
            (0x94, 0x0108_41bc),
            (0xa4, 0x0108_418c),
            (0xa8, 0x0108_4144),
        ] {
            assert_eq!(e.mem.u32(actor.addr() + offset), vtable);
        }
        let calls = called(&e);
        let pos = |addr| calls.iter().position(|a| *a == addr).unwrap();
        // Both objects are deleted through their first virtual with flag 1.
        assert_eq!(
            args_of(&e, slot_target(0x0201_0000, 0)),
            vec![ragdoll.addr(), 1]
        );
        assert_eq!(
            args_of(&e, slot_target(0x0202_0000, 0)),
            vec![detection.addr(), 1]
        );
        // The sound handle is released because it is valid.
        assert!(calls.contains(&0x00ad_8d10));
        assert_eq!(args_of(&e, 0x0041_8890)[0], actor.addr() + 0x44);
        assert_eq!(
            args_of(&e, REMOVE_FROM_PLAYER_COMBAT_LIST),
            vec![0x7777, actor.addr()]
        );
        assert_eq!(
            args_of(&e, REMOVE_PERCEIVED_ACTOR),
            vec![0x7777, actor.addr()]
        );
        assert_eq!(
            args_of(&e, 0x0096_f600),
            vec![PROCESS_LISTS, actor.addr(), 1]
        );
        assert_eq!(args_of(&e, 0x0042_26e0), vec![0x8888]);
        // The process kind is 1: the accumulator is told, then 008c30a0 runs.
        assert_eq!(args_of(&e, 0x00b6_6050), vec![0x9000, 0x9100]);
        assert!(pos(0x00b6_6050) < pos(0x008c_30a0));
        assert!(pos(0x008c_30a0) < pos(0x008c_51f0));
        assert_eq!(args_of(&e, STOP_COMBAT), vec![actor.addr(), 0]);
        assert_eq!(args_of(&e, SET_3D), vec![actor.addr(), 0, 0]);
        assert_eq!(args_of(&e, 0x0093_70b0), vec![actor.addr() + 0xd0]);
        // Member destructors in reverse order of construction; base last.
        assert!(pos(0x0046_ffb0) > pos(SET_3D));
        assert!(pos(0x0093_7030) < pos(0x0082_2a80));
        assert!(pos(0x0082_2a80) < pos(0x0081_53b0));
        assert_eq!(*calls.last().unwrap(), 0x0092_ec40);
    }

    #[test]
    fn destructor_skips_the_lists_when_the_shutdown_check_says_so() {
        let (mut e, actor, _, _) = destructor_setup();
        e.register(0x0040_77c0, |_, _| ret(1));
        e.call_log = Some(vec![]);
        e.call(0x0087_dcb0, &args![actor]);
        let calls = called(&e);
        for skipped in [
            REMOVE_FROM_PLAYER_COMBAT_LIST,
            0x0096_e6f0,
            0x0042_26e0,
            SET_3D,
            0x0096_f400,
        ] {
            assert!(!calls.contains(&skipped), "{skipped:08x} must be skipped");
        }
        // The members are still taken apart.
        assert!(calls.contains(&0x0046_ffb0) && calls.contains(&0x0092_ec40));
    }

    #[test]
    fn destructor_only_does_part_of_the_magic_teardown_while_shutting_down() {
        let (mut e, actor, _, _) = destructor_setup();
        e.register(0x0042_26e0, |_, _| ret(1));
        e.call_log = Some(vec![]);
        e.call(0x0087_dcb0, &args![actor]);
        let calls = called(&e);
        assert!(calls.contains(&0x0096_e6f0));
        assert!(!calls.contains(&0x0082_4970) && !calls.contains(&0x0097_6680));
        assert!(calls.contains(&0x0096_f400) && calls.contains(&SET_3D));
    }

    // ---- 0087e060 and 0087e100 ----

    #[test]
    fn load3d_wrapper_returns_null_without_a_node() {
        let mut e = engine(&[MOBILE_OBJECT_LOAD3D]);
        let actor: Ptr<Actor> = e.new_object();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0087_e060, &args![actor, true]).u32(), 0);
        assert_eq!(called(&e), vec![0x0087_e060, MOBILE_OBJECT_LOAD3D]);
    }

    #[test]
    fn load3d_wrapper_hands_the_node_to_the_process_and_sets_the_flag_bits() {
        let mut e = engine(&[SET_FLAG_BITS, 0x008b_7360]);
        e.register(MOBILE_OBJECT_LOAD3D, |_, _| ret(0x4000));
        e.register(GET_BASE_FORM, |e, a| ret(e.mem.u32(a[0] + 0x20)));
        e.register(0x008b_7b70, |_, _| ret(1));
        // The setting at 011c7664 holds 0.
        e.map(0x011c_7000, 0x1000);
        e.register(SETTING_VALUE_POINTER, |_, _| ret(0x011c_7800));
        let actor: Ptr<Actor> = object(&mut e, 0x1b4, 0x0200_0000, &[]).cast();
        let base = object(&mut e, 0x40, 0x0203_0000, &[0x180]);
        e.register(slot_target(0x0203_0000, 0x180), |_, _| ret(0x6100));
        e.mem.set_u32(actor.addr() + 0x20, base.addr());
        let process = object(&mut e, 0x10, 0x0204_0000, &[0x6c8]);
        e.set(actor, Actor::pCurrentProcess, process);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0087_e060, &args![actor, false]).u32(), 0x4000);
        assert_eq!(
            args_of(&e, slot_target(0x0204_0000, 0x6c8)),
            vec![process.addr(), 0x4000, 0x6100]
        );
        // The setting is off: the node's bits are asked to be set.
        assert_eq!(args_of(&e, SET_FLAG_BITS), vec![0x4000, 1, 0x0080_0000]);
        assert_eq!(args_of(&e, 0x008b_7360), vec![actor.addr(), 1, 0]);
    }

    #[test]
    fn load3d_wrapper_leaves_the_flag_bits_when_the_setting_is_on() {
        let mut e = engine(&[SET_FLAG_BITS, 0x008b_7b70]);
        e.register(MOBILE_OBJECT_LOAD3D, |_, _| ret(0x4000));
        e.map(0x011c_7000, 0x1000);
        e.mem.set_u8(0x011c_7800, 1);
        e.register(SETTING_VALUE_POINTER, |_, _| ret(0x011c_7800));
        let actor: Ptr<Actor> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x0087_e060, &args![actor, true]);
        let calls = called(&e);
        assert!(!calls.contains(&SET_FLAG_BITS) && !calls.contains(&0x008b_7360));
    }

    #[test]
    fn fn_0087e100_asks_for_the_bits_to_be_set_when_the_flag_is_false() {
        let mut e = engine(&[SET_FLAG_BITS]);
        e.call_log = Some(vec![]);
        e.call(0x0087_e100, &args![Ptr::<()>::new(0x4000), false]);
        assert_eq!(args_of(&e, SET_FLAG_BITS), vec![0x4000, 1, 0x0080_0000]);
        e.call_log = Some(vec![]);
        e.call(0x0087_e100, &args![Ptr::<()>::new(0x4000), true]);
        assert_eq!(args_of(&e, SET_FLAG_BITS), vec![0x4000, 0, 0x0080_0000]);
    }

    // ---- 0087e130 ----

    const RAGDOLL_CALLEES: [u32; 50] = [
        0x004f_d380,
        0x004a_de00,
        0x005f_0c80,
        0x0050_fd90,
        0x0045_c6b0,
        0x0044_7080,
        0x0050_fdf0,
        0x0040_6d00,
        0x0056_fac0,
        0x004e_a950,
        0x004b_46a0,
        0x0096_a2d0,
        0x005e_5320,
        0x005e_ede0,
        0x0063_f7b0,
        0x007c_b2e0,
        0x004e_4660,
        0x00c7_de60,
        0x0051_b860,
        0x0060_7810,
        0x005b_a4f0,
        0x005b_a540,
        0x005c_6b40,
        0x00c7_c3a0,
        0x005b_a130,
        0x005b_a310,
        0x005b_a180,
        0x0044_ddc0,
        0x005b_a360,
        0x008b_c300,
        0x005e_ee10,
        0x0046_ffb0,
        0x0093_1ed0,
        0x004a_3a20,
        0x0040_6d30,
        0x0040_6d50,
        0x0043_8170,
        0x0063_3c90,
        0x0043_b1b0,
        0x00c7_b1f0,
        0x0045_cec0,
        0x0043_81b0,
        0x00c7_5ad0,
        0x0093_06d0,
        0x0047_4cb0,
        0x0055_d520,
        0x005b_5e40,
        0x0063_d040,
        0x00ec_7750,
        0x0040_ab30,
    ];

    struct Ragdoll {
        e: Engine,
        actor: Ptr<Actor>,
        controller: Ptr,
    }

    /// An actor whose base form (`GetBaseForm` result) is `node`'s owner: the
    /// base form's virtual `+0x180` gives `node`, whose virtuals `+0x130` and
    /// (at `+0x18`) `+0x14` give names. The controller built by `00c7f060` is
    /// `controller`, with `bInitRagdollAnim` = `controller_ok`.
    fn ragdoll_setup(controller_ok: bool) -> Ragdoll {
        let mut e = engine(&RAGDOLL_CALLEES);
        map_pages(
            &mut e,
            &[
                0x011a_9000,
                0x011c_6000,
                0x0102_3000,
                0x011d_e000,
                0x011c_3000,
            ],
        );
        e.set_global(PLAYER_POINTER, 0x7777u32);
        e.set_global(MODEL_LOADER_POINTER, 0x6666u32);
        e.set_global(DEGREES_TO_RADIANS, 0.5f64);
        for i in 0..9u32 {
            e.set_global(IDENTITY_MATRIX + 4 * i, 0x300 + i);
        }
        let node = object(&mut e, 0x40, 0x0210_0000, &[0x130]);
        e.mem.set_u32(node.addr() + 0x18, 0x0211_0000);
        e.map(0x0211_0000, 0x1000);
        e.mem.set_u32(0x0211_0000 + 0x14, 0x6001_1001);
        e.register(0x6001_1001, |e, _| ret(e.mem.alloc(0x40)));
        let base = object(&mut e, 0x40, 0x0212_0000, &[0x180]);
        e.register_double(slot_target(0x0212_0000, 0x180), {
            let node = node.addr();
            move |_, _| ret(node)
        });
        e.register_double(GET_BASE_FORM, move |_, _| ret(base.addr()));
        e.register(GET_FORM, |_, a| ret(a[0] + 0x20));
        let controller = Ptr::<()>::new(e.mem.alloc(0x300));
        e.mem.set_u8(controller.addr() + 0x42, controller_ok as u8);
        e.register_double(0x00c7_f060, {
            let controller = controller.addr();
            move |_, _| ret(controller)
        });
        e.register(ALLOCATE_BLOCK, |e, a| ret(e.mem.alloc(a[0])));
        e.register(0x0068_15c0, |_, a| ret(a[0]));
        e.register(0x004a_de00, |_, _| ret(0));
        let actor: Ptr<Actor> = object(&mut e, 0x1b4, 0x0213_0000, &[0x130, 0x21c]).cast();
        e.register(slot_target(0x0213_0000, 0x130), |_, _| ret(0xa100));
        // Strings the ragdoll code copies.
        e.register(0x0040_6d30, |e, a| {
            e.mem
                .set_cstr(a[0], b"Data\\Meshes\\Characters\\Skeleton.nif");
            Ret::default()
        });
        e.register(0x0040_ab30, |e, a| {
            let text = e.mem.cstr(a[0]);
            let index = text.iter().rposition(|byte| *byte == b'\\').unwrap();
            ret(a[0] + index as u32)
        });
        Ragdoll {
            e,
            actor,
            controller,
        }
    }

    #[test]
    fn ragdoll_of_an_immobile_actor_outside_a_world_space_only_sets_the_turret_flag() {
        let mut r = ragdoll_setup(true);
        r.e.register(0x004a_de00, |_, _| ret(1));
        r.e.register(0x005f_0c80, |_, _| ret(1));
        // No world space: 004fd380 gives null.
        r.e.call_log = Some(vec![]);
        r.e.call(
            0x0087_e130,
            &args![r.actor, Ptr::<()>::new(0x5555), false, true, false],
        );
        assert_eq!(r.e.get(r.actor, Actor::bTurretBehavior), 1);
        let calls = called(&r.e);
        assert!(!calls.contains(&GET_FORM) && !calls.contains(&0x00c7_f060));
        assert_eq!(args_of(&r.e, 0x004a_de00), vec![0x5555, 0]);
    }

    #[test]
    fn ragdoll_replaces_the_existing_controller_and_reports_a_failed_initialization() {
        let mut r = ragdoll_setup(false);
        let old = object(&mut r.e, 0x20, 0x0214_0000, &[0]);
        r.e.set(r.actor, Actor::pRagdollController, old);
        r.e.register(0x0047_4cb0, |_, _| ret(1));
        r.e.call_log = Some(vec![]);
        r.e.call(
            0x0087_e130,
            &args![r.actor, Ptr::<()>::new(0x5555), true, true, true],
        );
        assert_eq!(
            args_of(&r.e, slot_target(0x0214_0000, 0)),
            vec![old.addr(), 1]
        );
        // The new controller is stored, the name has the id and old pointer.
        assert_eq!(r.e.get(r.actor, Actor::pRagdollController), r.controller);
        let format = args_of(&r.e, 0x0040_6d00);
        assert_eq!(format[1..3], [0x104, RAGDOLL_NAME_FORMAT]);
        assert_eq!(format[4..], [r.actor.addr(), old.addr()]);
        // ModelLoader::LoadFile(file, lod, 1, 0, 0, 1) on the loader pointer.
        assert_eq!(args_of(&r.e, 0x0044_7080), vec![0x6666, 0, 0, 1, 0, 0, 1]);
        // The error names the actor through its virtual +0x130.
        assert_eq!(
            args_of(&r.e, 0x005b_5e40),
            vec![RAGDOLL_ERROR_CONTROLLER, 0xa100]
        );
        assert!(!called(&r.e).contains(&0x0056_fac0));
    }

    #[test]
    fn ragdoll_without_iks_sets_the_group_pose_matching_and_the_character_proxy() {
        let mut r = ragdoll_setup(true);
        let world = Ptr::<()>::new(0x8000);
        r.e.register(0x004f_d380, |_, _| ret(0x8000));
        r.e.register(0x004a_3a20, |_, _| ret(0xabc));
        r.e.register(0x0043_b1b0, |_, _| ret(0xf00d));
        r.e.register(0x0093_06d0, |e, _| {
            let controller = e.mem.alloc(0x600);
            e.mem.set_u32(controller + 0x594, 0xcafe);
            ret(controller)
        });
        r.e.register(0x0055_9450, |e, a| ret(e.mem.u32(a[0])));
        r.e.register(0x004a_e750, |_, a| ret(a[0] + 1));
        r.e.register(0x004e_a950, |_, _| ret(0x6a6a));
        r.e.call_log = Some(vec![]);
        r.e.call(
            0x0087_e130,
            &args![r.actor, Ptr::<()>::new(0x5555), false, false, false],
        );
        let calls = called(&r.e);
        // No IK set-up.
        assert!(!calls.contains(&0x0096_a2d0) && !calls.contains(&0x00c7_c3a0));
        // The identity matrix is copied before MultipleMatrixByRace and the
        // inverse is taken against the controller's matrix.
        let multiply = args_of(&r.e, 0x0056_fac0);
        assert_eq!(multiply[0], r.actor.addr());
        for i in 0..9u32 {
            assert_eq!(r.e.mem.u32(multiply[2] + 4 * i), 0x300 + i);
        }
        assert_eq!(args_of(&r.e, 0x004b_46a0), vec![multiply[2], 0x6a6a]);
        // The group comes from 004a3a20, the pose file path ends in Death.psa.
        assert_eq!(
            r.e.mem.u32(r.controller.addr() + 0x5c),
            0xabc,
            "iGroup (+0x5c)"
        );
        let path = args_of(&r.e, 0x0043_8170)[1];
        assert_eq!(r.e.mem.cstr(path), b"Data\\Meshes\\Characters\\");
        assert_eq!(
            args_of(&r.e, 0x0040_6d50),
            vec![path, 0x104, DEATH_POSE_FILE]
        );
        assert_eq!(
            args_of(&r.e, 0x00c7_b1f0),
            vec![r.controller.addr(), 0xf00d, u32::MAX, u32::MAX, u32::MAX]
        );
        // A world space and a non-player actor: the area at +0x40 is given.
        assert_eq!(
            args_of(&r.e, 0x00c7_5ad0),
            vec![r.controller.addr(), world.addr() + 0x40]
        );
        // The character controller's word at +0x594 goes through 004ae750 into +0x90.
        assert_eq!(r.e.mem.u32(r.controller.addr() + 0x90), 0xcaff);
        // bFlag254 is zero, so +0x255 is zero.
        assert_eq!(r.e.mem.u8(r.controller.addr() + 0x255), 0);
    }

    #[test]
    fn ragdoll_walks_the_nodes_for_look_ik_and_foot_ik() {
        let mut r = ragdoll_setup(true);
        r.e.set_global(PLAYER_POINTER, r.actor.addr());
        // Two nodes: the first has bit 4 (collected), the second bit 0x20
        // (look IK); the Prime name is found; foot IK fails and complains.
        let first = r.e.mem.alloc(0x80);
        r.e.mem.set_u8(first + 0x60, 4);
        let second = r.e.mem.alloc(0x80);
        r.e.mem.set_u8(second + 0x60, 0x20);
        let queue = Rc::new(RefCell::new(vec![second, first]));
        let advanced = Rc::new(Cell::new(0u32));
        r.e.register_double(0x0082_56d0, {
            let queue = queue.clone();
            move |e, a| match queue.borrow_mut().pop() {
                Some(node) => {
                    e.mem.set_u32(a[0], node);
                    ret(0)
                }
                None => ret(1),
            }
        });
        r.e.register_double(0x0063_f7b0, {
            let advanced = advanced.clone();
            move |_, _| {
                advanced.set(advanced.get() + 1);
                Ret::default()
            }
        });
        r.e.register(0x0063_d040, |_, a| ret(a[0] + 0x1000));
        r.e.register(0x0051_b860, |_, _| 90.0f32.into_ret());
        r.e.register(0x005b_a540, |_, _| ret(0));
        r.e.register(0x00ec_7750, |_, _| ret(1));
        r.e.register(0x0044_ddc0, |_, _| ret(1));
        r.e.register(0x005b_a360, |_, _| ret(1));
        r.e.register(0x0047_4cb0, |_, _| ret(0));
        r.e.register(0x0055_d520, |_, _| ret(0xbeef));
        r.e.call_log = Some(vec![]);
        r.e.call(
            0x0087_e130,
            &args![r.actor, Ptr::<()>::new(0x5555), false, true, false],
        );
        let log = r.e.call_log.clone().unwrap();
        let calls = called(&r.e);
        assert_eq!(advanced.get(), 2);
        // The first node is added to the array through its part (+0x1000).
        let add = log.iter().find(|(a, _)| *a == 0x007c_b2e0).unwrap();
        assert_eq!(r.e.mem.u32(add.1[1]), first + 0x1000);
        // Look IK is initialized once, from the second node: the virtual
        // +0x21c is false so the controller is asked first; the angle is
        // 90 degrees times the factor.
        assert_eq!(
            log.iter().filter(|(a, _)| *a == 0x00c7_de60).count(),
            1,
            "look IK once"
        );
        assert_eq!(args_of(&r.e, 0x004e_4660), vec![r.controller.addr(), 1]);
        assert_eq!(
            args_of(&r.e, 0x0060_7810),
            vec![r.controller.addr(), 1, 45.0f32.to_bits()]
        );
        // Both IK errors name the actor through 0055d520 (00474cb0 is zero).
        let errors: Vec<_> = log.iter().filter(|(a, _)| *a == 0x005b_5e40).collect();
        assert_eq!(errors.len(), 2);
        assert_eq!(errors[0].1, vec![RAGDOLL_ERROR_LOOK_IK, 0xbeef]);
        assert_eq!(errors[1].1, vec![RAGDOLL_ERROR_FOOT_IK, 0xbeef]);
        // "Prime" in the name clears the byte at +0x224 and asks for +0x5c6b40.
        assert_eq!(args_of(&r.e, 0x005c_6b40), vec![r.controller.addr(), 1]);
        assert_eq!(args_of(&r.e, 0x00ec_7750)[1], PRIME_TEXT);
        // The foot IK result 005ba360 is true: the actor is told (008bc300).
        assert_eq!(args_of(&r.e, 0x008b_c300), vec![r.actor.addr()]);
        // The player skips the area call (no world space here either).
        assert!(!calls.contains(&0x00c7_5ad0));
        let position = |addr| calls.iter().position(|a| *a == addr).unwrap();
        assert!(position(0x005e_ee10) < position(0x0093_1ed0));
    }
}

#[cfg(test)]
mod tests_accessors {
    use super::*;

    fn ret(value: u32) -> Ret {
        value.into_ret()
    }

    fn engine(addrs: &[u32]) -> Engine {
        let mut e = Engine::new();
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
        e
    }

    fn slot_target(vtable: u32, offset: u32) -> u32 {
        0x6000_0000 + (vtable & 0x00ff_ffff) + offset
    }

    fn object(e: &mut Engine, size: u32, vtable: u32, offsets: &[u32]) -> Ptr {
        e.map(vtable, 0x1000);
        for offset in offsets {
            let target = slot_target(vtable, *offset);
            e.mem.set_u32(vtable + offset, target);
            e.register(target, |_, _| Ret::default());
        }
        let object = Ptr::new(e.mem.alloc(size));
        e.mem.set_u32(object.addr(), vtable);
        object
    }

    fn called(e: &Engine) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(addr, _)| *addr)
            .collect()
    }

    fn args_of(e: &Engine, addr: u32) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .find(|(a, _)| *a == addr)
            .unwrap_or_else(|| panic!("{addr:08x} was not called"))
            .1
            .clone()
    }

    /// An actor whose base form (the result of `004181e0`) is `base`.
    fn actor_with_base(e: &mut Engine, base: Ptr) -> Ptr<Actor> {
        let actor = object(
            e,
            0x1b4,
            0x0200_0000,
            &[0x218, 0x1a0, 0x214, 0x418, 0x48, 0x190, 0x4d4],
        );
        e.register_double(GET_BASE_FORM, move |_, _| ret(base.addr()));
        e.register(GET_EXTRA_DATA_LIST, |_, a| ret(a[0] + 0x44));
        actor.cast()
    }

    // ---- 0087e920 / 0087e940 ----

    #[test]
    fn node_flag_tests_read_bits_4_and_0x20_of_the_byte_at_0x60() {
        let mut e = Engine::new();
        let node = Ptr::<()>::new(e.mem.alloc(0x80));
        assert!(!e.call(0x0087_e920, &args![node]).bool());
        assert!(!e.call(0x0087_e940, &args![node]).bool());
        e.mem.set_u8(node.addr() + 0x60, 0x04);
        assert!(e.call(0x0087_e920, &args![node]).bool());
        assert!(!e.call(0x0087_e940, &args![node]).bool());
        e.mem.set_u8(node.addr() + 0x60, 0x20);
        assert!(!e.call(0x0087_e920, &args![node]).bool());
        assert!(e.call(0x0087_e940, &args![node]).bool());
    }

    // ---- 0087e960 / 0087eeb0 ----

    #[test]
    fn turret_flag_is_written_and_read_at_0x1b0() {
        let mut e = Engine::new();
        let actor: Ptr<Actor> = e.new_object();
        e.call(0x0087_e960, &args![actor, true]);
        assert_eq!(e.mem.u8(actor.addr() + 0x1b0), 1);
        assert_eq!(e.call(0x0087_eeb0, &args![actor]).u8(), 1);
        e.call(0x0087_e960, &args![actor, false]);
        assert_eq!(e.call(0x0087_eeb0, &args![actor]).u8(), 0);
    }

    // ---- 0087e980, 0087e9a0, 0087e9b0 ----

    #[test]
    fn character_controller_word_comes_from_the_helper_at_0x594() {
        let mut e = Engine::new();
        e.register(0x0055_9450, |_, a| ret(a[0] ^ 1));
        let result = e.call(0x0087_e980, &args![Ptr::<()>::new(0x1000)]).u32();
        assert_eq!(result, (0x1000 + 0x594) ^ 1);
    }

    #[test]
    fn fn_0087e9a0_reads_its_global() {
        let mut e = Engine::new();
        e.map(0x011c_6000, 0x1000);
        e.set_global(GLOBAL_VALUE_011C625C, 0x1234u32);
        assert_eq!(e.call(0x0087_e9a0, &args![]).u32(), 0x1234);
    }

    #[test]
    fn fn_0087e9b0_releases_the_block() {
        let mut e = engine(&[FREE_BLOCK]);
        e.call_log = Some(vec![]);
        e.call(0x0087_e9b0, &args![Ptr::<()>::new(0x4444)]);
        assert_eq!(args_of(&e, FREE_BLOCK), vec![0x4444]);
    }

    // ---- ragdoll controller accessors ----

    #[test]
    fn controller_flag_255_needs_flag_254_and_the_argument() {
        let mut e = Engine::new();
        let controller = Ptr::<()>::new(e.mem.alloc(0x300));
        e.call(0x0087_e9d0, &args![controller, true]);
        assert_eq!(e.mem.u8(controller.addr() + 0x255), 0);
        e.mem.set_u8(controller.addr() + 0x254, 1);
        e.call(0x0087_e9d0, &args![controller, true]);
        assert_eq!(e.mem.u8(controller.addr() + 0x255), 1);
        e.call(0x0087_e9d0, &args![controller, false]);
        assert_eq!(e.mem.u8(controller.addr() + 0x255), 0);
    }

    #[test]
    fn controller_setters_and_getters_use_their_offsets() {
        let mut e = Engine::new();
        let controller = Ptr::<()>::new(e.mem.alloc(0x300));
        e.mem.set_u8(controller.addr() + 0x42, 7);
        assert_eq!(e.call(0x0087_ea20, &args![controller]).u8(), 7);
        e.call(0x0087_ea40, &args![controller, 0x99u32]);
        assert_eq!(e.mem.u32(controller.addr() + 0x5c), 0x99);
        e.call(0x0087_ea60, &args![controller, 3u8]);
        assert_eq!(e.mem.u8(controller.addr() + 0x224), 3);
    }

    #[test]
    fn controller_character_proxy_goes_through_the_pointer_operator() {
        let mut e = Engine::new();
        e.register(0x004a_e750, |_, a| {
            ret(if a[0] == 0 { 0 } else { a[0] + 8 })
        });
        let controller = Ptr::<()>::new(e.mem.alloc(0x300));
        e.call(0x0087_ea80, &args![controller, Ptr::<()>::new(0x100)]);
        assert_eq!(e.mem.u32(controller.addr() + 0x90), 0x108);
    }

    #[test]
    fn fn_0087eaa0_is_the_address_0x40_in() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x0087_eaa0, &args![Ptr::<()>::new(0x1000)]).u32(),
            0x1040
        );
    }

    // ---- 0087eed0 ----

    #[test]
    fn package_byte_at_0xbe() {
        let mut e = Engine::new();
        let package = Ptr::<()>::new(e.mem.alloc(0xc0));
        e.mem.set_u8(package.addr() + 0xbe, 5);
        assert_eq!(e.call(0x0087_eed0, &args![package]).u8(), 5);
    }

    // ---- 0087eac0 ----

    const PACKAGE_CALLEES: [u32; 12] = [
        0x0096_11e0,
        0x0056_9b80,
        0x0067_77b0,
        0x008d_8520,
        0x0088_d640,
        0x0067_4d70,
        CLEAR_IN_COMBAT,
        0x0045_34f0,
        0x0049_3bb0,
        0x008b_70d0,
        0x008b_3d30,
        0x008a_8f60,
    ];

    struct Package {
        e: Engine,
        actor: Ptr<Actor>,
        process: Ptr,
        package: Ptr,
    }

    fn package_setup(state: u32, kind: u32) -> Package {
        let mut e = engine(&PACKAGE_CALLEES);
        e.map(0x0101_2000, 0x1000);
        e.map(0x0101_a000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        e.set_global(0x0101_2060, 0.0f64);
        e.set_global(0x0101_a6b0, -1.0f64);
        e.set_global(PLAYER_POINTER, 0x7777u32);
        let actor = actor_with_base(&mut e, Ptr::NULL);
        e.register_double(slot_target(0x0200_0000, 0x214), move |_, _| ret(state));
        let process = object(
            &mut e,
            0x40,
            0x0201_0000,
            &[
                0x1e0, 0x1e4, 0x214, 0x294, 0x628, 0x230, 0x614, 0x210, 0x284, 0x12c, 0x244,
            ],
        );
        e.set(actor, Actor::pCurrentProcess, process);
        e.register(0x0096_11e0, |_, _| ret(5));
        e.register_double(0x0041_ca90, move |_, _| ret(kind));
        let package = Ptr::<()>::new(e.mem.alloc(0xc0));
        e.call_log = Some(vec![]);
        Package {
            e,
            actor,
            process,
            package,
        }
    }

    #[test]
    fn package_change_does_nothing_without_a_process() {
        let mut p = package_setup(0, 5);
        p.e.set(p.actor, Actor::pCurrentProcess, Ptr::NULL);
        p.e.call(0x0087_eac0, &args![p.actor, p.package, false, true]);
        assert_eq!(called(&p.e), vec![0x0087_eac0]);
    }

    #[test]
    fn package_start_adjusts_the_timer_and_clears_combat() {
        let mut p = package_setup(0, 5);
        p.e.register(slot_target(0x0201_0000, 0x1e0), |_, _| 2.0f32.into_ret());
        // The package type is not yet known (-1): it is calculated for the actor.
        p.e.register(0x0096_11e0, |_, _| ret(u32::MAX));
        p.e.register(0x0056_9b80, |_, _| ret(0x4242));
        p.e.register(0x008b_70d0, |_, _| ret(1));
        p.e.call(0x0087_eac0, &args![p.actor, p.package, false, true]);
        let calls = called(&p.e);
        assert_eq!(args_of(&p.e, 0x0067_77b0), vec![p.package.addr(), 0x4242]);
        // State 0 skips the sit/sleep handling; the package is marked created.
        assert!(!calls.contains(&0x0088_d640));
        assert_eq!(args_of(&p.e, 0x0067_4d70), vec![p.package.addr(), 1]);
        // The timer is positive: it is negated and the process told.
        assert_eq!(
            args_of(&p.e, slot_target(0x0201_0000, 0x1e4)),
            vec![p.process.addr(), (-2.0f32).to_bits()]
        );
        assert_eq!(
            args_of(&p.e, slot_target(0x0201_0000, 0x294)),
            vec![p.process.addr(), p.actor.addr()]
        );
        assert_eq!(
            args_of(&p.e, slot_target(0x0201_0000, 0x628)),
            vec![p.process.addr(), 0x7777]
        );
        assert_eq!(args_of(&p.e, CLEAR_IN_COMBAT), vec![p.actor.addr(), 1]);
        assert_eq!(
            args_of(&p.e, slot_target(0x0201_0000, 0x230)),
            vec![p.process.addr(), p.package.addr(), p.actor.addr()]
        );
        // The actor has an animation and state 0: the process gets 0x800.
        assert_eq!(
            args_of(&p.e, slot_target(0x0201_0000, 0x614)),
            vec![p.process.addr(), 0x800]
        );
        // The look-at target is cleared (not a turret, 00493bb0 false).
        assert!(calls.contains(&0x008b_3d30));
        assert_eq!(*calls.last().unwrap(), 0x008a_8f60);
    }

    #[test]
    fn package_start_keeps_combat_for_a_type_0x12_package_and_skips_a_zero_timer() {
        let mut p = package_setup(0, 0x12);
        p.e.register(0x0049_3bb0, |_, _| ret(1));
        p.e.call(0x0087_eac0, &args![p.actor, p.package, false, false]);
        let calls = called(&p.e);
        assert!(!calls.contains(&CLEAR_IN_COMBAT));
        assert!(!calls.contains(&slot_target(0x0201_0000, 0x1e4)));
        assert!(!calls.contains(&0x0067_4d70));
        assert!(!calls.contains(&0x008b_3d30));
        assert!(!calls.contains(&slot_target(0x0201_0000, 0x614)));
    }

    #[test]
    fn package_leave_from_a_furniture_state_returns_early() {
        // State 4: the actor leaves its furniture, and with flag_a set the
        // function ends there.
        let mut p = package_setup(4, 5);
        p.e.call(0x0087_eac0, &args![p.actor, p.package, true, true]);
        let calls = called(&p.e);
        assert!(calls.contains(&slot_target(0x0200_0000, 0x418)));
        assert!(!calls.contains(&0x008a_8f60));
        assert!(!calls.contains(&slot_target(0x0201_0000, 0x210)));
    }

    #[test]
    fn package_leave_tells_the_process_unless_the_package_is_type_0x1c_with_its_flag() {
        let mut p = package_setup(0, 0x1c);
        p.e.mem.set_u8(p.package.addr() + 0xbe, 1);
        p.e.call(0x0087_eac0, &args![p.actor, p.package, true, false]);
        let calls = called(&p.e);
        assert_eq!(
            args_of(&p.e, slot_target(0x0201_0000, 0x210)),
            vec![p.process.addr(), p.package.addr(), p.actor.addr()]
        );
        assert_eq!(
            args_of(&p.e, slot_target(0x0201_0000, 0x284)),
            vec![p.process.addr(), 0]
        );
        assert!(!calls.contains(&slot_target(0x0201_0000, 0x294)));
        assert_eq!(
            args_of(&p.e, slot_target(0x0200_0000, 0x48)),
            vec![p.actor.addr(), 0]
        );
        assert!(calls.contains(&slot_target(0x0201_0000, 0x12c)));
        assert_eq!(*calls.last().unwrap(), 0x008a_8f60);
    }

    #[test]
    fn package_furniture_check_uses_the_acquire_object_when_not_sitting() {
        // State 2 and a non-null package kind that is not special: the
        // acquire object's virtual +0x4d4 decides; its byte at +0xe above
        // 0x14 sends the actor out of the furniture with the virtual +0x418.
        let mut p = package_setup(2, 5);
        let acquire = object(&mut p.e, 0x20, 0x0220_0000, &[0x4d4]);
        let object_row = Ptr::<()>::new(p.e.mem.alloc(0x20));
        p.e.mem.set_u8(object_row.addr() + 0xe, 0x15);
        let row = object_row.addr();
        p.e.register_double(slot_target(0x0220_0000, 0x4d4), move |_, _| ret(row));
        let acquire_address = acquire.addr();
        p.e.register_double(0x008d_8520, move |_, _| ret(acquire_address));
        p.e.call(0x0087_eac0, &args![p.actor, p.package, false, false]);
        let calls = called(&p.e);
        assert!(calls.contains(&slot_target(0x0200_0000, 0x418)));
        assert!(!calls.contains(&0x0088_d640));
        // With a low byte the actor is taken out of furniture quickly instead.
        let mut p = package_setup(2, 5);
        let acquire = object(&mut p.e, 0x20, 0x0220_0000, &[0x4d4]);
        let object_row = Ptr::<()>::new(p.e.mem.alloc(0x20));
        p.e.mem.set_u8(object_row.addr() + 0xe, 0x10);
        let row = object_row.addr();
        p.e.register_double(slot_target(0x0220_0000, 0x4d4), move |_, _| ret(row));
        let acquire_address = acquire.addr();
        p.e.register_double(0x008d_8520, move |_, _| ret(acquire_address));
        p.e.call(0x0087_eac0, &args![p.actor, p.package, false, false]);
        let calls = called(&p.e);
        assert!(calls.contains(&0x0088_d640));
        assert!(!calls.contains(&slot_target(0x0200_0000, 0x418)));
    }

    // ---- 0087eef0 ----

    #[test]
    fn default_inventory_without_a_source_only_updates_the_inventory_changes() {
        let mut e = engine(&[0x0044_1110, 0x004d_1960]);
        let base = Ptr::new(e.mem.alloc(0x40));
        let actor = actor_with_base(&mut e, base);
        e.register(0x0041_8520, |_, _| ret(0x6600));
        e.call_log = Some(vec![]);
        e.call(0x0087_eef0, &args![actor]);
        assert_eq!(
            called(&e),
            vec![
                0x0087_eef0,
                GET_BASE_FORM,
                0x0044_1110,
                GET_EXTRA_DATA_LIST,
                0x0041_8520,
                0x004d_1960
            ]
        );
        assert_eq!(args_of(&e, 0x004d_1960), vec![0x6600]);
    }

    #[test]
    fn default_inventory_adds_each_entry_with_its_count_and_extra_data() {
        let mut e = engine(&[
            0x0048_1610,
            0x0048_7f70,
            0x0048_1680,
            0x0041_9ad0,
            0x0041_9ed0,
            0x0041_9f80,
            0x0040_ea20,
            0x004d_1960,
            0x0087_f9f0,
        ]);
        let base = Ptr::<()>::new(e.mem.alloc(0x80));
        let actor = actor_with_base(&mut e, base);
        e.register(0x0044_1110, |_, _| ret(0x2000));
        e.register(0x0041_8520, |_, _| ret(0));
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        e.register(EXTRA_DATA_LIST_CONSTRUCTOR, |_, a| ret(a[0]));
        e.register(0x0068_15c0, |_, a| ret(a[0]));
        // Three list nodes (the first word of a node is its entry pointer);
        // entry layout: count, item, health description.
        let item = object(&mut e, 0x20, 0x0230_0000, &[0xe4]);
        e.register(slot_target(0x0230_0000, 0xe4), |_, _| ret(1));
        let plain = e.mem.alloc(0x10);
        let health = e.mem.alloc(0x10);
        let scripted = e.mem.alloc(0x10);
        let nodes: Vec<u32> = (0..3).map(|_| e.mem.alloc(0x10)).collect();
        e.mem.set_u32(nodes[0], plain);
        e.mem.set_u32(nodes[1], health);
        e.mem.set_u32(nodes[2], scripted);
        for (entry, count, description) in
            [(plain, 3u32, 0u32), (health, 2, 0x9000), (scripted, 2, 0)]
        {
            e.mem.set_u32(entry, count);
            e.mem.set_u32(entry + 4, item.addr());
            e.mem.set_u32(entry + 8, description);
        }
        // The list: node i -> node i+1; the container snapshot starts at nodes[0].
        let table = nodes.clone();
        e.register_double(0x0071_7e50, move |_, _| ret(table[0]));
        let next = nodes.clone();
        e.register_double(0x0072_6070, move |_, a| {
            let at = next.iter().position(|n| *n == a[0]).unwrap();
            ret(next.get(at + 1).copied().unwrap_or(0))
        });
        // Only the third entry's item has a script.
        let scripted_entry = scripted;
        let _ = scripted_entry;
        let calls_to_script = std::rc::Rc::new(std::cell::Cell::new(0));
        let counter = calls_to_script.clone();
        e.register_double(0x0048_26d0, move |_, _| {
            counter.set(counter.get() + 1);
            // The first two entries have no script, the third has one.
            ret(if counter.get() == 3 { 0x5c5c } else { 0 })
        });
        e.register(0x0041_8800, |_, _| ret(0));
        e.register(0x005a_bf60, |_, _| ret(0x7a7a));
        e.register(0x0048_72e0, |_, _| ret(0x4e4e));
        e.call_log = Some(vec![]);
        e.call(0x0087_eef0, &args![actor]);
        let log = e.call_log.clone().unwrap();
        let adds: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == slot_target(0x0200_0000, 0x190))
            .map(|(_, args)| args.clone())
            .collect();
        // plain: no extra data, count 3; health: extra data with count 2;
        // scripted: two extra data lists with count 1 each.
        assert_eq!(adds.len(), 4);
        assert_eq!(adds[0][2..], [0, 3]);
        assert_ne!(adds[1][2], 0);
        assert_eq!(adds[1][3], 2);
        assert_eq!(adds[2][3], 1);
        assert_eq!(adds[3][3], 1);
        // The health entry's description is applied with the item's health form.
        let health_call = log.iter().find(|(a, _)| *a == 0x0040_ea20).unwrap();
        assert_eq!((health_call.1[0], health_call.1[2]), (0x9000, 0x4e4e));
        // Scripted entries get the script attached to each extra data list.
        let set_script: Vec<_> = log.iter().filter(|(a, _)| *a == 0x0041_9ed0).collect();
        assert_eq!(set_script.len(), 2);
        assert_eq!(set_script[0].1[1], 0x5c5c);
        assert_eq!(args_of(&e, 0x0041_9ad0)[1], 2); // the health entry's SetCount(2)
    }

    // ---- the base form predicates ----

    #[test]
    fn fn_0087f200_is_false_when_either_check_says_yes() {
        for (virtual_says, helper_says, expected) in [
            (false, false, true),
            (true, false, false),
            (false, true, false),
        ] {
            let mut e = engine(&[]);
            let base = object(&mut e, 0x80, 0x0240_0000, &[0x44]);
            // The virtual is on the component at base + 0x30.
            e.mem.set_u32(base.addr() + 0x30, 0x0241_0000);
            e.map(0x0241_0000, 0x1000);
            e.mem.set_u32(0x0241_0000 + 0x44, 0x7000_0044);
            e.register_double(0x7000_0044, move |_, _| ret(virtual_says as u32));
            let actor = actor_with_base(&mut e, base);
            e.register_double(0x008a_ce90, move |_, _| ret(helper_says as u32));
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0087_f200, &args![actor]).bool(), expected);
            // The helper is only asked when the virtual says no.
            assert_eq!(called(&e).contains(&0x008a_ce90), !virtual_says);
        }
    }

    fn essential_engine(base_form_value: u32) -> (Engine, Ptr<Actor>) {
        let mut e = engine(&[]);
        let actor = actor_with_base(&mut e, Ptr::new(base_form_value));
        e.map(0x011d_e000, 0x1000);
        e.set_global(PLAYER_POINTER, 0);
        e.register(0x0042_16f0, |_, _| ret(0));
        e.register(0x0056_6950, |_, _| ret(0));
        e.register(0x004d_1360, |_, _| ret(0));
        e.register(0x008c_1b30, |_, _| ret(0));
        e.register(0x0046_1580, |e, a| {
            ret(((e.mem.u32(a[0] + 4) & a[1]) != 0) as u32)
        });
        (e, actor)
    }

    #[test]
    fn essential_is_false_without_a_base_form() {
        let (mut e, actor) = essential_engine(0);
        assert!(!e.call(0x0087_f3d0, &args![actor]).bool());
    }

    #[test]
    fn essential_follows_the_base_flag_or_the_actor_check() {
        let (mut e, actor) = essential_engine(0);
        let base = e.mem.alloc(0x80);
        e.register_double(GET_BASE_FORM, move |_, _| ret(base));
        assert!(!e.call(0x0087_f3d0, &args![actor]).bool());
        // Bit 2 of the word at +4 of the component at base + 0x30.
        e.mem.set_u32(base + 0x34, 2);
        assert!(e.call(0x0087_f3d0, &args![actor]).bool());
        e.mem.set_u32(base + 0x34, 0);
        e.register(0x008c_1b30, |_, _| ret(1));
        assert!(e.call(0x0087_f3d0, &args![actor]).bool());
    }

    #[test]
    fn essential_prefers_the_original_leveled_base() {
        let (mut e, actor) = essential_engine(0);
        let original = e.mem.alloc(0x80);
        e.mem.set_u32(original + 0x34, 2);
        e.register_double(0x0042_16f0, move |_, _| ret(original));
        assert!(e.call(0x0087_f3d0, &args![actor]).bool());
    }

    #[test]
    fn essential_is_true_while_the_player_forces_the_next_update() {
        let (mut e, actor) = essential_engine(0);
        let base = e.mem.alloc(0x80);
        e.register_double(GET_BASE_FORM, move |_, _| ret(base));
        e.set_global(PLAYER_POINTER, 0x7777u32);
        e.register(0x0056_6950, |_, _| ret(1));
        assert!(e.call(0x0087_f3d0, &args![actor]).bool());
        // Unless the player check says otherwise.
        e.register(0x004d_1360, |_, _| ret(1));
        assert!(!e.call(0x0087_f3d0, &args![actor]).bool());
    }

    #[test]
    fn fn_0087f260_checks_creature_state_settings_idle_and_base_virtuals() {
        // Plain actor: nothing blocks.
        let mut e = engine(&[]);
        let base = object(&mut e, 0x80, 0x0240_0000, &[]);
        let component = 0x0241_0000u32;
        e.mem.set_u32(base.addr() + 0x30, component);
        e.map(component, 0x1000);
        e.mem.set_u32(component + 0x50, 0x7000_0050);
        e.mem.set_u32(component + 0x48, 0x7000_0048);
        e.register(0x7000_0050, |_, _| ret(1));
        e.register(0x7000_0048, |_, _| ret(0));
        let actor = actor_with_base(&mut e, base);
        e.register(RT_DYNAMIC_CAST, |_, _| ret(0));
        e.register(0x005f_bf20, |_, _| ret(0));
        e.register(0x005a_29b0, |_, _| ret(0));
        e.map(0x011d_f000, 0x1000);
        e.register(SETTING_VALUE_POINTER, |_, _| ret(0x011d_f800));
        e.call_log = Some(vec![]);
        assert!(e.call(0x0087_f260, &args![actor]).bool());
        assert_eq!(
            args_of(&e, RT_DYNAMIC_CAST),
            vec![base.addr(), 0, 0x0118_46e8, 0x0118_3a00, 0]
        );
        // A last idle forbids it.
        e.register(0x005a_29b0, |_, _| ret(1));
        assert!(!e.call(0x0087_f260, &args![actor]).bool());
        e.register(0x005a_29b0, |_, _| ret(0));
        // Both base virtuals true forbid it.
        e.register(0x7000_0048, |_, _| ret(1));
        assert!(!e.call(0x0087_f260, &args![actor]).bool());
        e.register(0x7000_0048, |_, _| ret(0));
        // A humanoid creature with the setting on forbids it; with the
        // setting off, the actor's virtual +0x1a0(0) decides.
        e.register(RT_DYNAMIC_CAST, |_, _| ret(0x5000));
        e.register(0x005f_bf20, |_, _| ret(1));
        assert!(e.call(0x0087_f260, &args![actor]).bool());
        e.register(slot_target(0x0200_0000, 0x1a0), |_, _| ret(1));
        assert!(!e.call(0x0087_f260, &args![actor]).bool());
        e.register(slot_target(0x0200_0000, 0x1a0), |_, _| ret(0));
        e.mem.set_u8(0x011d_f800, 1);
        assert!(!e.call(0x0087_f260, &args![actor]).bool());
    }

    #[test]
    fn creature_actor_value_wrappers_forward_to_the_base_form_predicates() {
        for (address, helper) in [
            (0x0087_f350u32, 0x005f_0b50u32),
            (0x0087_f370, 0x005f_0bf0),
            (0x0087_f390, 0x005f_0c40),
            (0x0087_f3b0, 0x005f_0ba0),
        ] {
            let mut e = engine(&[]);
            let actor = actor_with_base(&mut e, Ptr::new(0x3300));
            e.register(helper, |_, a| ret((a[0] == 0x3300) as u32));
            assert!(e.call(address, &args![actor]).bool(), "{address:08x}");
            let other = actor_with_base(&mut e, Ptr::new(0x3301));
            assert!(!e.call(address, &args![other]).bool());
        }
    }

    #[test]
    fn fn_0087f480_tests_form_flag_bit_2() {
        let mut e = Engine::new();
        e.register(0x0046_1580, |_, a| ret((a[1] == 2) as u32));
        assert!(e.call(0x0087_f480, &args![Ptr::<()>::new(0x100)]).bool());
    }

    #[test]
    fn fn_0087f4a0_asks_the_component_at_0x30() {
        let mut e = engine(&[]);
        let actor = actor_with_base(&mut e, Ptr::new(0x2000));
        e.register(0x0056_af00, |_, a| ret((a[0] == 0x2030) as u32));
        assert!(e.call(0x0087_f4a0, &args![actor]).bool());
    }

    #[test]
    fn fn_0087f4c0_returns_the_sex_of_the_base() {
        let mut e = engine(&[]);
        let actor = actor_with_base(&mut e, Ptr::new(0x2000));
        e.register(0x005f_0cc0, |_, _| ret(u32::MAX));
        assert_eq!(e.call(0x0087_f4c0, &args![actor]).i32(), -1);
        e.register(0x005f_0cc0, |_, _| ret(1));
        assert_eq!(e.call(0x0087_f4c0, &args![actor]).i32(), 1);
    }

    #[test]
    fn fn_0087f4e0_calls_the_base_forms_virtual_0x144() {
        let mut e = engine(&[]);
        let base = object(&mut e, 0x40, 0x0240_0000, &[0x144]);
        e.register(slot_target(0x0240_0000, 0x144), |_, a| ret(a[0] + 1));
        let actor = actor_with_base(&mut e, base);
        assert_eq!(e.call(0x0087_f4e0, &args![actor]).u32(), base.addr() + 1);
    }

    #[test]
    fn fn_0087f510_uses_the_extra_data_or_the_component() {
        let mut e = engine(&[]);
        let actor = actor_with_base(&mut e, Ptr::new(0x2000));
        // Virtual +0x218 false: the answer is 1.
        assert_eq!(e.call(0x0087_f510, &args![actor]).u8(), 1);
        e.register(slot_target(0x0200_0000, 0x218), |_, _| ret(1));
        // Without the extra data, 005d2780 on the component at base + 0x30.
        e.register(0x0041_0220, |_, _| ret(0));
        e.register(0x005d_2780, |_, a| ret((a[0] == 0x2030) as u32 * 7));
        assert_eq!(e.call(0x0087_f510, &args![actor]).u8(), 7);
        // With it, the byte at +0xc.
        let extra = e.mem.alloc(0x10);
        e.mem.set_u8(extra + 0xc, 9);
        e.register_double(0x0041_0220, move |_, _| ret(extra));
        assert_eq!(e.call(0x0087_f510, &args![actor]).u8(), 9);
    }

    #[test]
    fn swims_only_and_its_users() {
        let mut e = engine(&[]);
        let actor = actor_with_base(&mut e, Ptr::new(0x2000));
        // 0087f370 false, 0087f350 true, 0087f390 false: swims only.
        e.register(0x005f_0bf0, |_, _| ret(0));
        e.register(0x005f_0b50, |_, _| ret(1));
        e.register(0x005f_0c40, |_, _| ret(0));
        assert!(e.call(0x0087_f570, &args![actor]).bool());
        // A flying creature does not (0087f390 true).
        e.register(0x005f_0c40, |_, _| ret(1));
        assert!(!e.call(0x0087_f570, &args![actor]).bool());
        e.register(0x005f_0c40, |_, _| ret(0));
        // 0087f620: false only for a (virtual +0x218 false) swimmer.
        assert!(!e.call(0x0087_f620, &args![actor]).bool());
        e.register(slot_target(0x0200_0000, 0x218), |_, _| ret(1));
        assert!(e.call(0x0087_f620, &args![actor]).bool());
    }

    #[test]
    fn fn_0087f5c0_falls_back_to_the_actor_value() {
        let mut e = engine(&[]);
        let actor = actor_with_base(&mut e, Ptr::new(0x2000));
        e.register(0x005f_0bf0, |_, _| ret(0));
        e.register(0x005f_0b50, |_, _| ret(1));
        e.register(0x005f_0c40, |_, _| ret(0));
        // Virtual +0x218 false and a swimmer: true without asking the owner.
        assert!(e.call(0x0087_f5c0, &args![actor]).bool());
        // Otherwise the actor-value owner at +0xa4 (virtual +8, argument 0x35).
        e.register(slot_target(0x0200_0000, 0x218), |_, _| ret(1));
        e.map(0x0250_0000, 0x1000);
        e.mem.set_u32(actor.addr() + 0xa4, 0x0250_0000);
        e.mem.set_u32(0x0250_0000 + 8, 0x7000_0008);
        let owner = actor.addr() + 0xa4;
        e.register_double(0x7000_0008, move |_, a| {
            ret((a[0] == owner && a[1] == 0x35) as u32)
        });
        assert!(e.call(0x0087_f5c0, &args![actor]).bool());
    }
}

#[cfg(test)]
mod tests_block2 {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The argument words of each call a double saw.
    type Log = Rc<RefCell<Vec<Vec<u32>>>>;

    const VTABLE: u32 = 0x0200_0000;
    const OWNER_VTABLE: u32 = 0x0201_0000;
    const PROCESS_VTABLE: u32 = 0x0202_0000;

    /// An engine with the pages of the globals the tests set mapped.
    fn new_engine() -> Engine {
        let mut e = Engine::new();
        for page in [0x0101_2000, 0x0101_7000, 0x011d_e000] {
            e.map(page, 0x1000);
        }
        e
    }

    fn ret(value: u32) -> Ret {
        value.into_ret()
    }

    fn slot(vtable: u32, offset: u32) -> u32 {
        0x6000_0000 + (vtable & 0x00ff_ffff) + offset
    }

    /// A double over `addr` that returns `value` and logs the argument words.
    fn record(e: &mut Engine, addr: u32, value: Ret) -> Log {
        let log: Log = Rc::new(RefCell::new(vec![]));
        let seen = log.clone();
        e.register_double(addr, move |_, a| {
            seen.borrow_mut().push(a.to_vec());
            value
        });
        log
    }

    /// Doubles returning zero over every address.
    fn stub(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    /// An object of `size` bytes whose vtable (at `vtable`) has the listed
    /// slots, each a double returning zero.
    fn object(e: &mut Engine, size: u32, vtable: u32, offsets: &[u32]) -> Ptr {
        e.map(vtable, 0x1000);
        for offset in offsets {
            let target = slot(vtable, *offset);
            e.mem.set_u32(vtable + offset, target);
            e.register(target, |_, _| Ret::default());
        }
        let object = Ptr::new(e.mem.alloc(size));
        e.mem.set_u32(object.addr(), vtable);
        object
    }

    /// An actor whose vtable has the listed slots.
    fn actor(e: &mut Engine, slots: &[u32]) -> Ptr<Actor> {
        object(e, 0x1b4, VTABLE, slots).cast()
    }

    /// Gives the actor a process with the listed virtual slots.
    fn give_process(e: &mut Engine, actor: Ptr<Actor>, slots: &[u32]) -> Ptr {
        let process = object(e, 0x400, PROCESS_VTABLE, slots);
        e.set(actor, Actor::pCurrentProcess, process);
        e.register(ACTOR_PROCESS, |e, a| ret(e.mem.u32(a[0] + 0x68)));
        process
    }

    /// Gives the actor an actor-value owner at `+0xa4` with the listed slots.
    fn give_owner(e: &mut Engine, actor: Ptr<Actor>, slots: &[u32]) -> u32 {
        e.map(OWNER_VTABLE, 0x1000);
        for offset in slots {
            let target = slot(OWNER_VTABLE, *offset);
            e.mem.set_u32(OWNER_VTABLE + offset, target);
            e.register(target, |_, _| Ret::default());
        }
        e.mem.set_u32(actor.addr() + 0xa4, OWNER_VTABLE);
        actor.addr() + 0xa4
    }

    /// Makes `base` the actor's base form.
    fn give_base(e: &mut Engine, base: u32) {
        e.register_double(GET_BASE_FORM, move |_, _| ret(base));
    }

    /// `_ftol2_sse`: truncation of the `f64` in the first two words.
    fn install_float_to_int(e: &mut Engine) {
        e.register(FLOAT_TO_INT, |_, a| {
            let value = f64::from_bits(a[0] as u64 | (a[1] as u64) << 32);
            ret(value as i32 as u32)
        });
    }

    /// The disposition-modifier list of `actor` with the given entries
    /// `(amount, target)`; returns the item addresses. Installs the two node
    /// helpers.
    fn modifier_list(e: &mut Engine, actor: Ptr<Actor>, entries: &[(i32, u32)]) -> Vec<u32> {
        e.register(LIST_NODE_ITEM_SLOT, |_, a| ret(a[0]));
        e.register(LIST_NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        let mut items = vec![];
        let mut node = actor.addr() + 0xfc;
        for (i, (amount, target)) in entries.iter().enumerate() {
            let item = e.mem.alloc(8);
            e.mem.set_i32(item, *amount);
            e.mem.set_u32(item + 4, *target);
            e.mem.set_u32(node, item);
            if i + 1 < entries.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
            items.push(item);
        }
        items
    }

    // ---- 0087f660 ----

    #[test]
    fn check_breath_timer_is_false_only_when_the_swim_time_exceeds_the_timer() {
        let mut e = new_engine();
        install_float_to_int(&mut e);
        let actor = actor(&mut e, &[]);
        e.register(0x008b_e7a0, |_, _| 7.9f32.into_ret());
        // 00648a10 gives twice the integer level: 14.0.
        e.register(0x0064_8a10, |_, a| ((a[0] as i32) as f32 * 2.0).into_ret());
        assert!(e.call(0x0087_f660, &args![actor]).bool());
        give_process(&mut e, actor, &[0x300]);
        e.register(slot(PROCESS_VTABLE, 0x300), |_, _| 20.0f32.into_ret());
        assert!(e.call(0x0087_f660, &args![actor]).bool());
        e.register(slot(PROCESS_VTABLE, 0x300), |_, _| 10.0f32.into_ret());
        assert!(!e.call(0x0087_f660, &args![actor]).bool());
    }

    // ---- 0087f6c0 ----

    #[test]
    fn location_name_needs_a_form_a_record_and_a_passing_check() {
        let mut e = new_engine();
        let actor = actor(&mut e, &[0x218]);
        e.register(GET_FORM, |_, _| ret(0x5000));
        e.register(0x004a_c110, |_, _| ret(0x6000));
        e.register(0x0048_cee0, |_, _| ret(1));
        let name = record(&mut e, 0x0040_8da0, ret(0x777));
        assert_eq!(e.call(0x0087_f6c0, &args![actor]).u32(), 0);
        e.register(slot(VTABLE, 0x218), |_, _| ret(1));
        assert_eq!(e.call(0x0087_f6c0, &args![actor]).u32(), 0x777);
        assert_eq!(name.borrow()[0], vec![0x6018]);
        e.register(0x0048_cee0, |_, _| ret(0));
        assert_eq!(e.call(0x0087_f6c0, &args![actor]).u32(), 0);
        e.register(0x004a_c110, |_, _| ret(0));
        assert_eq!(e.call(0x0087_f6c0, &args![actor]).u32(), 0);
    }

    // ---- 0087f750 / 0087f7a0 ----

    #[test]
    fn editor_location_form_getters_check_the_form_type() {
        let mut e = new_engine();
        let actor = actor(&mut e, &[]);
        e.register(FORM_TYPE, |_, _| ret(0x41));
        e.register(FLAG_BIT_0_AT_0X24, |_, _| ret(1));
        assert_eq!(e.call(0x0087_f750, &args![actor]).u32(), 0);
        e.set(actor, Actor::pEditorLocForm, Ptr::<()>::new(0x4444));
        assert_eq!(e.call(0x0087_f750, &args![actor]).u32(), 0x4444);
        assert_eq!(e.call(0x0087_f7a0, &args![actor]).u32(), 0);
        e.register(FORM_TYPE, |_, _| ret(0x39));
        assert_eq!(e.call(0x0087_f750, &args![actor]).u32(), 0);
        assert_eq!(e.call(0x0087_f7a0, &args![actor]).u32(), 0x4444);
        e.register(FLAG_BIT_0_AT_0X24, |_, _| ret(0));
        assert_eq!(e.call(0x0087_f7a0, &args![actor]).u32(), 0);
    }

    // ---- 0087f800 ----

    #[test]
    fn editor_location_is_taken_from_the_actor() {
        let mut e = new_engine();
        let actor = actor(&mut e, &[0x2bc]);
        let position = e.mem.alloc(12);
        for i in 0..3 {
            e.mem.set_f32(position + 4 * i, 1.0 + i as f32);
        }
        e.register_double(0x0043_6aa0, move |_, _| ret(position));
        let rotation = record(&mut e, slot(VTABLE, 0x2bc), 2.5f32.into_ret());
        e.register(0x008d_6f30, |_, _| ret(0x9000));
        e.register(FLAG_BIT_0_AT_0X24, |_, _| ret(1));
        e.register(0x0057_5d70, |_, _| ret(0x9100));
        e.call(0x0087_f800, &args![actor]);
        assert_eq!(e.mem.f32(actor.addr() + 0x160 + 8), 3.0);
        assert_eq!(e.get(actor, Actor::fEditorLocZRot), 2.5);
        assert_eq!(e.get(actor, Actor::pEditorLocForm).addr(), 0x9000);
        assert_eq!(rotation.borrow()[0], vec![actor.addr(), 0]);
        // The candidate fails the check: the world space form is used.
        e.register(FLAG_BIT_0_AT_0X24, |_, _| ret(0));
        e.call(0x0087_f800, &args![actor]);
        assert_eq!(e.get(actor, Actor::pEditorLocForm).addr(), 0x9100);
        // No candidate at all.
        e.register(0x008d_6f30, |_, _| ret(0));
        e.set(actor, Actor::pEditorLocForm, Ptr::<()>::new(1));
        e.call(0x0087_f800, &args![actor]);
        assert_eq!(e.get(actor, Actor::pEditorLocForm).addr(), 0x9100);
    }

    // ---- 0087f890 ----

    #[test]
    fn editor_location_is_set_from_the_arguments() {
        let mut e = new_engine();
        let actor = actor(&mut e, &[]);
        let position = e.mem.alloc(12);
        for i in 0..3 {
            e.mem.set_f32(position + 4 * i, 4.0 + i as f32);
        }
        e.register(FLAG_BIT_0_AT_0X24, |_, _| ret(1));
        let words = args![actor, 0x1000u32, 0x2000u32, position, 0.5f32];
        e.call(0x0087_f890, &words);
        assert_eq!(e.mem.f32(actor.addr() + 0x160 + 8), 6.0);
        assert_eq!(e.get(actor, Actor::fEditorLocZRot), 0.5);
        assert_eq!(e.get(actor, Actor::pEditorLocForm).addr(), 0x2000);
        e.register(FLAG_BIT_0_AT_0X24, |_, _| ret(0));
        e.call(0x0087_f890, &words);
        assert_eq!(e.get(actor, Actor::pEditorLocForm).addr(), 0x1000);
        e.set(actor, Actor::pEditorLocForm, Ptr::<()>::new(1));
        let words = args![actor, 0x1000u32, 0u32, position, 0.5f32];
        e.call(0x0087_f890, &words);
        assert_eq!(e.get(actor, Actor::pEditorLocForm).addr(), 0x1000);
    }

    // ---- 0087f900 / 0087f920 ----

    #[test]
    fn editor_location_can_be_tested_and_read_back() {
        let mut e = new_engine();
        let actor = actor(&mut e, &[]);
        let out = e.mem.alloc(0x20);
        let words = args![actor, out, out + 0x10, out + 0x1c, 0u32];
        assert!(!e.call(0x0087_f900, &args![actor]).bool());
        assert!(!e.call(0x0087_f920, &words).bool());
        e.set(actor, Actor::pEditorLocForm, Ptr::<()>::new(0x4444));
        e.set(actor, Actor::fEditorLocZRot, 3.0f32);
        for i in 0..3 {
            e.mem.set_f32(actor.addr() + 0x160 + 4 * i, 1.0 + i as f32);
        }
        assert!(e.call(0x0087_f900, &args![actor]).bool());
        assert!(e.call(0x0087_f920, &words).bool());
        assert_eq!(e.mem.f32(out + 8), 3.0);
        let rotation = (
            e.mem.f32(out + 0x10),
            e.mem.f32(out + 0x14),
            e.mem.f32(out + 0x18),
        );
        assert_eq!(rotation, (0.0, 0.0, 3.0));
        assert_eq!(e.mem.u32(out + 0x1c), 0x4444);
    }

    // ---- 0087f990 / 0087f9f0 ----

    #[test]
    fn base_form_component_predicates() {
        let mut e = new_engine();
        let actor = actor(&mut e, &[]);
        give_base(&mut e, 0x3000);
        let first = record(&mut e, 0x005b_b4d0, ret(1));
        let second = record(&mut e, 0x0047_ded0, ret(0x1234));
        assert!(e.call(0x0087_f990, &args![actor]).bool());
        assert_eq!(first.borrow()[0], vec![0x3090]);
        assert_eq!(e.call(0x0087_f9f0, &args![actor]).u32(), 0x1234);
        assert_eq!(second.borrow()[0], vec![0x3030]);
    }

    // ---- 0087f9c0 ----

    #[test]
    fn owner_value_for_actor_value_0x15_is_returned_as_a_float() {
        let mut e = new_engine();
        let actor = actor(&mut e, &[]);
        give_owner(&mut e, actor, &[0]);
        let seen = record(&mut e, slot(OWNER_VTABLE, 0), ret((-3i32) as u32));
        assert_eq!(e.call(0x0087_f9c0, &args![actor]).f32(), -3.0);
        assert_eq!(seen.borrow()[0], vec![actor.addr() + 0xa4, 0x15]);
    }

    // ---- 0087fa10 / 0087fa60 / 0087faa0 ----

    #[test]
    fn process_forwarders_need_a_process() {
        let mut e = new_engine();
        let actor = actor(&mut e, &[]);
        e.call(0x0087_fa10, &args![actor, true]);
        e.call(0x0087_fa60, &args![actor, true]);
        e.call(0x0087_faa0, &args![actor]);
        let process = give_process(&mut e, actor, &[0x118, 0x120, 0x294, 0x4a0]);
        let first = record(&mut e, slot(PROCESS_VTABLE, 0x118), Ret::default());
        let second = record(&mut e, slot(PROCESS_VTABLE, 0x4a0), Ret::default());
        let third = record(&mut e, slot(PROCESS_VTABLE, 0x120), Ret::default());
        let fourth = record(&mut e, slot(PROCESS_VTABLE, 0x294), Ret::default());
        e.call(0x0087_fa10, &args![actor, true]);
        e.call(0x0087_fa60, &args![actor, false]);
        e.call(0x0087_faa0, &args![actor]);
        assert_eq!(first.borrow()[0], vec![process.addr(), 1]);
        assert_eq!(second.borrow()[0], vec![process.addr()]);
        assert_eq!(third.borrow()[0], vec![process.addr(), 0]);
        assert_eq!(fourth.borrow()[0], vec![process.addr(), actor.addr()]);
    }

    // ---- 0087fad0 ----

    #[test]
    fn removing_a_disposition_modifier_hands_the_item_to_the_list_remover() {
        let mut e = new_engine();
        let actor = actor(&mut e, &[]);
        let items = modifier_list(&mut e, actor, &[(5, 0x100), (6, 0x200)]);
        let seen: Rc<RefCell<Vec<(u32, u32)>>> = Rc::new(RefCell::new(vec![]));
        let log = seen.clone();
        e.register_double(LIST_REMOVE_ITEM, move |e, a| {
            log.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        e.call(0x0087_fad0, &args![actor, 0x200u32]);
        assert_eq!(*seen.borrow(), vec![(actor.addr() + 0xfc, items[1])]);
        e.call(0x0087_fad0, &args![actor, 0x300u32]);
        assert_eq!(seen.borrow().len(), 1);
    }

    // ---- 0087fb40 ----

    #[test]
    fn disposition_modifier_is_added_for_the_player_and_limited_to_0_100() {
        let mut e = new_engine();
        install_float_to_int(&mut e);
        e.set_global(0x0101_7a40, 100.0f64);
        e.set_global(PLAYER_POINTER, 0x7000u32);
        let actor = actor(&mut e, &[0x48, 0x344]);
        let flags = record(&mut e, slot(VTABLE, 0x48), Ret::default());
        // The target is not the player: nothing happens.
        e.call(0x0087_fb40, &args![actor, 0x7100u32, 5.0f32]);
        assert!(flags.borrow().is_empty());
        // No entry yet: a new one is allocated and appended.
        modifier_list(&mut e, actor, &[]);
        let block = e.mem.alloc(8);
        e.register_double(OPERATOR_NEW, move |_, _| ret(block));
        let hold = record(&mut e, 0x0056_4db0, Ret::default());
        let appended: Rc<RefCell<Vec<u32>>> = Rc::new(RefCell::new(vec![]));
        let log = appended.clone();
        e.register_double(LIST_APPEND_ITEM, move |e, a| {
            log.borrow_mut().push(e.mem.u32(a[1]));
            Ret::default()
        });
        e.call(0x0087_fb40, &args![actor, 0x7000u32, 12.75f32]);
        assert_eq!(flags.borrow()[0], vec![actor.addr(), 0x80000]);
        assert_eq!(hold.borrow()[0], vec![0x7000, 1]);
        assert_eq!(*appended.borrow(), vec![block]);
        assert_eq!((e.mem.i32(block), e.mem.u32(block + 4)), (12, 0x7000));
        // An entry exists (amount 10): the amount is limited by the disposition.
        let items = modifier_list(&mut e, actor, &[(10, 0x7000)]);
        // (disposition, amount, expected entry): 60 - 80 < 0 turns the amount
        // into -60; within range adds it; above 100 keeps only the part up to
        // 100; already at 100 adds 0.
        let cases = [
            (60i32, -80.0f32, -50),
            (60, 30.0, 40),
            (90, 30.0, 20),
            (100, 30.0, 10),
        ];
        for (disposition, amount, expected) in cases {
            e.mem.set_i32(items[0], 10);
            e.register_double(slot(VTABLE, 0x344), move |_, _| ret(disposition as u32));
            e.call(0x0087_fb40, &args![actor, 0x7000u32, amount]);
            assert_eq!(e.mem.i32(items[0]), expected, "{disposition} {amount}");
        }
    }

    // ---- 0087fcb0 ----

    #[test]
    fn disposition_modifier_toward_an_actor_is_its_amount_or_zero() {
        let mut e = new_engine();
        let actor = actor(&mut e, &[]);
        modifier_list(&mut e, actor, &[(5, 0x100), (-7, 0x200)]);
        assert_eq!(e.call(0x0087_fcb0, &args![actor, 0x200u32]).f32(), -7.0);
        assert_eq!(e.call(0x0087_fcb0, &args![actor, 0x100u32]).f32(), 5.0);
        assert_eq!(e.call(0x0087_fcb0, &args![actor, 0x300u32]).f32(), 0.0);
    }

    // ---- 0087fd20 ----

    #[test]
    fn clearing_the_disposition_modifiers_frees_each_entry_then_clears_the_list() {
        let mut e = new_engine();
        let actor = actor(&mut e, &[]);
        let items = modifier_list(&mut e, actor, &[(1, 0x100), (2, 0x200)]);
        let freed = record(&mut e, OPERATOR_DELETE, Ret::default());
        let cleared = record(&mut e, LIST_CLEAR, Ret::default());
        e.call(0x0087_fd20, &args![actor]);
        assert_eq!(*freed.borrow(), vec![vec![items[0]], vec![items[1]]]);
        assert_eq!(*cleared.borrow(), vec![vec![actor.addr() + 0xfc]]);
    }

    // ---- 0087fd90 ----

    /// The doubles the score function needs, all returning zero except the
    /// ones that matter to the cases; `GetForm` of an object is its address + 1.
    fn score_fixture(e: &mut Engine) -> (Ptr<Actor>, Ptr<Actor>) {
        install_float_to_int(e);
        stub(
            e,
            &[
                0x005d_8a70,
                0x0082_56d0,
                0x0060_8d80,
                0x0048_bf50,
                0x008b_7fe0,
                0x008a_16d0,
                0x0049_3bb0,
                0x0097_fa10,
                0x0097_0a20,
                0x0066_ef20,
                0x0056_7790,
                0x0047_d3d0,
                ACTOR_PROCESS,
                0x005e_58f0,
                0x0064_2a60,
                FORM_TYPE,
            ],
        );
        e.set_global(PLAYER_POINTER, 0x7000u32);
        let this = actor(e, &[0x37c, 0x218, 0x21c, 0x428, 0x464]);
        let other: Ptr<Actor> = object(e, 0x1b4, 0x0203_0000, &[0x218]).cast();
        e.map(OWNER_VTABLE, 0x1000);
        e.mem.set_u32(other.addr() + 0xa4, OWNER_VTABLE);
        e.mem.set_u32(OWNER_VTABLE + 8, slot(OWNER_VTABLE, 8));
        e.register(slot(OWNER_VTABLE, 8), |_, _| ret(0x55));
        give_base(e, 0x3000);
        e.register(GET_FORM, |_, a| ret(a[0] + 1));
        (this, other)
    }

    #[test]
    fn score_is_zero_without_a_target_and_100_for_the_actors_own_form() {
        let mut e = new_engine();
        let (this, other) = score_fixture(&mut e);
        assert_eq!(e.call(0x0087_fd90, &args![this, 0u32, 0u32]).i32(), 0);
        // The cached block names the target form: GetForm(other) == other + 1.
        let cached = e.mem.alloc(0x18);
        e.mem.set_u32(cached + 8, other.addr() + 1);
        assert_eq!(e.call(0x0087_fd90, &args![this, other, cached]).i32(), 100);
    }

    #[test]
    fn score_combines_the_owner_values_and_is_clamped() {
        let mut e = new_engine();
        let (this, other) = score_fixture(&mut e);
        // First path: not creature-like (virtual +0x218 false) with an owner reference.
        let cached = e.mem.alloc(0x18);
        e.mem.set_u32(cached + 4, 0x3300); // base of this
        e.mem.set_u32(cached + 8, 0x4400); // target form
        e.mem.set_u32(cached + 0xc, 0x5500); // owner reference
        e.mem.set_u32(cached + 0x10, 0x66);
        e.mem.set_u32(cached + 0x14, 0x77);
        let combine = record(&mut e, 0x0064_2a60, ret(150));
        let seen = Rc::new(RefCell::new(0.0f32));
        let entry_point_input = seen.clone();
        e.register_double(0x005e_58f0, move |e, a| {
            *entry_point_input.borrow_mut() = e.mem.f32(a[3]);
            Ret::default()
        });
        assert_eq!(e.call(0x0087_fd90, &args![this, other, cached]).i32(), 100);
        // 00642a60's eleven words: flags word, 0066ef20 value, skill word,
        // owner value, then 0048bf50 value, 008b7fe0 result, its first out
        // word (-1 as set), the 008a16d0 word, a zero local, the +0x464 value
        // and the +0x21c byte.
        assert_eq!(
            combine.borrow()[0],
            vec![0x66, 0, 0x77, 0x55, 0, 0, 0xffff_ffff, 0, 0, 0, 0]
        );
        assert_eq!(*seen.borrow(), 150.0);
        // A negative score is raised to 0.
        e.register(0x0064_2a60, |_, _| ret((-20i32) as u32));
        assert_eq!(e.call(0x0087_fd90, &args![this, other, cached]).i32(), 0);
        e.register(0x0064_2a60, |_, _| ret(42));
        assert_eq!(e.call(0x0087_fd90, &args![this, other, cached]).i32(), 42);
        // The entry point may change the score.
        e.register(0x005e_58f0, |e, a| {
            e.mem.set_f32(a[3], 7.9);
            Ret::default()
        });
        assert_eq!(e.call(0x0087_fd90, &args![this, other, cached]).i32(), 7);
        // Second path: creature-like (virtual +0x218 true), no owner reference.
        e.register(0x005e_58f0, |_, _| Ret::default());
        e.register(slot(VTABLE, 0x218), |_, _| ret(1));
        e.mem.set_u32(cached + 0xc, 0);
        e.register(0x0064_2a60, |_, _| ret(61));
        assert_eq!(e.call(0x0087_fd90, &args![this, other, cached]).i32(), 61);
    }

    #[test]
    fn score_passes_eleven_words_to_00642a60_in_exe_order() {
        let mut e = new_engine();
        let (this, other) = score_fixture(&mut e);
        let cached = e.mem.alloc(0x18);
        e.mem.set_u32(cached, 0x9000); // cached +0x37c value
        e.mem.set_u32(cached + 4, 0x3300); // base of this
        e.mem.set_u32(cached + 8, 0x4400); // target form
        e.mem.set_u32(cached + 0xc, 0x5500); // owner reference
        e.mem.set_u32(cached + 0x10, 0x66);
        e.mem.set_u32(cached + 0x14, 0x77);
        e.register(slot(VTABLE, 0x37c), |_, _| ret(0x88));
        let cache_lookup = record(&mut e, 0x0048_bf50, ret(0x11));
        let this_addr = this.addr();
        e.register_double(0x008b_7fe0, move |e, a| {
            assert_eq!(a[0], this_addr);
            e.mem.set_u32(a[2], 0x33);
            ret(0x22)
        });
        e.register(0x0066_ef20, |_, _| ret(0x44));
        // Other's virtual +0x218 true: the first path reads 0048bf50.
        e.register(slot(0x0203_0000, 0x218), |_, _| ret(1));
        e.register(0x008a_16d0, |_, _| ret(0x0301));
        e.register(0x0049_3bb0, |_, _| ret(0));
        // 00970a20 finds `this`, whose virtual +0x464 gives 12.75.
        e.register_double(0x0097_0a20, move |_, _| ret(this_addr));
        e.register(slot(VTABLE, 0x464), |_, _| Ret {
            st0: 12.75,
            ..Ret::default()
        });
        e.register(slot(VTABLE, 0x21c), |_, _| ret(0x0201));
        let combine = record(&mut e, 0x0064_2a60, ret(50));
        assert_eq!(e.call(0x0087_fd90, &args![this, other, cached]).i32(), 50);
        // Second path: this creature-like (virtual +0x218 true), no owner
        // reference; the +0x21c byte is now 0 (AL of 0x100).
        e.register(slot(VTABLE, 0x218), |_, _| ret(1));
        e.register(slot(VTABLE, 0x21c), |_, _| ret(0x0100));
        e.mem.set_u32(cached + 0xc, 0);
        assert_eq!(e.call(0x0087_fd90, &args![this, other, cached]).i32(), 50);
        assert_eq!(
            *cache_lookup.borrow(),
            vec![vec![0x9040, 0x88], vec![0x9040, 0x88]]
        );
        assert_eq!(
            *combine.borrow(),
            vec![
                // 008801aa: [EBP-0x24], 0066ef20, [EBP-0x20], owner +8,
                // [EBP-0x48], [EBP-0x38], [EBP-0x50], [EBP-0x4c], [EBP-0x3c],
                // [EBP-0x54], +0x21c byte.
                vec![0x66, 0x44, 0x77, 0x55, 0x11, 0x22, 0x33, 1, 0, 12, 1],
                // 0088030f: [EBP-0x24], 0066ef20, [EBP-0x20], owner +8,
                // [EBP-0x74], [EBP-0x68], [EBP-0x7c], [EBP-0x78], [EBP-0x6c],
                // [EBP-0x80], +0x21c byte.
                vec![0x66, 0x44, 0x77, 0x55, 0x11, 0x22, 0x33, 1, 0, 12, 0],
            ]
        );
    }

    // ---- 00880370 ----

    #[test]
    fn owner_value_is_adjusted_and_truncated() {
        let mut e = new_engine();
        install_float_to_int(&mut e);
        let owner = Ptr::<()>::new(e.mem.alloc(8));
        e.map(OWNER_VTABLE, 0x1000);
        e.mem.set_u32(owner.addr(), OWNER_VTABLE);
        e.mem.set_u32(OWNER_VTABLE + 4, slot(OWNER_VTABLE, 4));
        e.register(slot(OWNER_VTABLE, 4), |_, a| {
            ((a[1] as f32) * 0.5 + 0.75).into_ret()
        });
        e.register(0x0040_4040, |_, a| (f32::from_bits(a[0]) * 2.0).into_ret());
        // virtual +4(6) = 3.75; doubled 7.5; truncated 7.
        assert_eq!(e.call(0x0088_0370, &args![owner, 6u32]).i32(), 7);
    }

    // ---- 008803a0 ----

    #[test]
    fn modified_owner_value_picks_base_modifier_or_sum() {
        let mut e = new_engine();
        let actor = actor(&mut e, &[0x48c, 0x21c, 0x360]);
        let owner = give_owner(&mut e, actor, &[]);
        // The base form's component at +0x100 has virtual +8 giving 10.
        let base = e.mem.alloc(0x200);
        e.map(0x0204_0000, 0x1000);
        e.mem.set_u32(0x0204_0000 + 8, slot(0x0204_0000, 8));
        e.mem.set_u32(base + 0x100, 0x0204_0000);
        give_base(&mut e, base);
        e.register(slot(0x0204_0000, 8), |_, _| ret(10));
        e.register(slot(VTABLE, 0x48c), |e, a| {
            e.mem.set_u8(a[2], 1);
            4.5f32.into_ret()
        });
        stub(
            &mut e,
            &[
                ACTOR_VALUE_IS_STAT,
                ACTOR_VALUE_HAS_FLAG,
                0x0056_6950,
                0x0066_ed60,
            ],
        );
        // The modifier flag is set and the index is not a stat: the modifier.
        assert_eq!(e.call(0x0088_03a0, &args![owner, 3u32]).f32(), 4.5);
        // A stat index, nothing special: the base value.
        e.register(ACTOR_VALUE_IS_STAT, |_, _| ret(1));
        assert_eq!(e.call(0x0088_03a0, &args![owner, 3u32]).f32(), 10.0);
        // Flag 0x800 set: 0066ed60 gives 2.0; with the modifier flag the base
        // becomes the modifier 4.5 and the result is the sum.
        e.register(ACTOR_VALUE_HAS_FLAG, |_, a| ret((a[1] == 0x800) as u32));
        e.register(0x0066_ed60, |e, a| {
            e.mem.set_f32(a[2], 2.0);
            ret(1)
        });
        assert_eq!(e.call(0x0088_03a0, &args![owner, 3u32]).f32(), 6.5);
        // The `ActorValueOwner` part (`this`) goes on to 0066ed60, not the actor.
        let expected = owner;
        e.register_double(0x0066_ed60, move |e, a| {
            assert_eq!(a[0], expected, "the owner, not the actor");
            e.mem.set_f32(a[2], 2.0);
            ret(1)
        });
        assert_eq!(e.call(0x0088_03a0, &args![owner, 3u32]).f32(), 6.5);
        // 0066ed60 refuses: the base value (here the modifier replaced it).
        e.register(0x0066_ed60, |_, _| ret(0));
        assert_eq!(e.call(0x0088_03a0, &args![owner, 3u32]).f32(), 4.5);
    }

    // ---- 00880580 / 008805f0 ----

    #[test]
    fn owner_thunks_use_the_process_when_there_is_one() {
        let mut e = new_engine();
        let actor = actor(&mut e, &[]);
        let owner = give_owner(&mut e, actor, &[0]);
        e.register(slot(OWNER_VTABLE, 0), |_, _| ret(9));
        give_base(&mut e, 0x3000);
        assert_eq!(e.call(0x0088_0580, &args![owner, 4u32]).u32(), 9);
        assert_eq!(e.call(0x0088_05f0, &args![owner, 4u32]).f32(), 9.0);
        let process = give_process(&mut e, actor, &[0x398, 0x39c]);
        let first = record(&mut e, slot(PROCESS_VTABLE, 0x398), ret(77));
        let second = record(&mut e, slot(PROCESS_VTABLE, 0x39c), 1.5f32.into_ret());
        assert_eq!(e.call(0x0088_0580, &args![owner, 4u32]).u32(), 77);
        assert_eq!(e.call(0x0088_05f0, &args![owner, 4u32]).f32(), 1.5);
        let expected = vec![process.addr(), 0x3000, 4, actor.addr()];
        assert_eq!(first.borrow()[0], expected);
        assert_eq!(second.borrow()[0], expected);
    }

    // ---- 00880660 / 00880690 / 008806d0 ----

    #[test]
    fn modifier_list_accessors_at_0xe0() {
        let mut e = new_engine();
        let actor = actor(&mut e, &[0x48, 0x394]);
        let getter = record(&mut e, 0x0093_76e0, 2.5f32.into_ret());
        let setter = record(&mut e, 0x0093_75e0, Ret::default());
        let flags = record(&mut e, slot(VTABLE, 0x48), Ret::default());
        let change = record(&mut e, slot(VTABLE, 0x394), Ret::default());
        assert_eq!(e.call(0x0088_0660, &args![actor, 3u8, 9u32]).f32(), 2.5);
        assert_eq!(getter.borrow()[0], vec![actor.addr() + 0xe0, 3, 9]);
        e.call(0x0088_0690, &args![actor, 4u8, 1.5f32]);
        let expected = vec![actor.addr() + 0xe0, 4, 1.5f32.to_bits()];
        assert_eq!(setter.borrow()[0], expected);
        assert_eq!(flags.borrow()[0], vec![actor.addr(), 0x400000]);
        e.call(0x0088_06d0, &args![actor, 7u32, -2i32]);
        let expected = vec![actor.addr(), 7, (-2.0f32).to_bits()];
        assert_eq!(change.borrow()[0], expected);
    }

    // ---- 00880700 ----

    #[test]
    fn float_actor_value_change_clamps_with_flag_0x100() {
        let mut e = new_engine();
        e.set_global(0x0101_2070, 1.0f64);
        let actor = actor(&mut e, &[0x490, 0x360]);
        give_owner(&mut e, actor, &[0xc]);
        give_process(&mut e, actor, &[0x3b0]);
        e.register(ACTOR_VALUE_HAS_BASE, |_, _| ret(1));
        e.register(slot(OWNER_VTABLE, 0xc), |_, _| 8.0f32.into_ret());
        e.register(ACTOR_VALUE_HAS_FLAG, |_, a| ret((a[1] == 0x100) as u32));
        let record_block = e.mem.alloc(0x100);
        e.mem.set_i32(record_block + 0x98, 50);
        e.register_double(0x0066_e920, move |_, _| ret(record_block));
        let change = record(&mut e, slot(VTABLE, 0x490), Ret::default());
        let process_call = record(&mut e, slot(PROCESS_VTABLE, 0x3b0), Ret::default());
        let report = record(&mut e, ACTOR_VALUE_CHANGED, Ret::default());
        // Negative: 0.
        e.call(0x0088_0700, &args![actor, 5u32, -3.0f32]);
        assert_eq!(change.borrow()[0], vec![actor.addr(), 5, 0]);
        // Above the maximum: maximum - 1.
        e.call(0x0088_0700, &args![actor, 5u32, 60.0f32]);
        assert_eq!(change.borrow()[1][2], 49.0f32.to_bits());
        // Below the maximum: unchanged.
        e.call(0x0088_0700, &args![actor, 5u32, 20.0f32]);
        assert_eq!(change.borrow()[2][2], 20.0f32.to_bits());
        assert_eq!(process_call.borrow().len(), 3);
        let expected = vec![
            actor.addr() + 0xa4,
            5,
            8.0f32.to_bits(),
            20.0f32.to_bits(),
            0,
        ];
        assert_eq!(report.borrow()[2], expected);
    }

    // ---- 00880850 / 00880890 ----

    #[test]
    fn adjustment_helpers_pass_the_value_through() {
        let mut e = new_engine();
        install_float_to_int(&mut e);
        e.register(0x0040_4040, |_, a| (f32::from_bits(a[0]) + 0.5).into_ret());
        let words = args![0u32, 1u32, 2.5f32, 3u32];
        assert_eq!(e.call(0x0088_0890, &words).f32(), 2.5);
        assert_eq!(e.call(0x0088_0850, &args![0u32, 1u32, 4i32, 3u32]).i32(), 4);
    }

    // ---- 008808a0 ----

    #[test]
    fn health_modifier_is_scaled_by_the_difficulty_factor() {
        let mut e = new_engine();
        let actor = actor(&mut e, &[0x360]);
        e.set_global(PLAYER_POINTER, 0x7000u32);
        e.register(slot(VTABLE, 0x360), |_, _| ret(1));
        let level = record(&mut e, 0x005b_e4d0, ret(2));
        let factor = record(&mut e, 0x0064_8cb0, 1.5f32.into_ret());
        assert_eq!(e.call(0x0088_08a0, &args![actor, 4.0f32, 0u32]).f32(), 6.0);
        assert_eq!(level.borrow()[0], vec![0x7000]);
        assert_eq!(factor.borrow()[0], vec![2, 0x10, 1]);
    }

    // ---- 008808f0 / 00880910 ----

    #[test]
    fn small_forwarders() {
        let mut e = new_engine();
        let actor = actor(&mut e, &[]);
        let stat = record(&mut e, ACTOR_VALUE_IS_STAT, ret(1));
        assert_eq!(e.call(0x0088_08f0, &args![actor, 12u32, 0u32]).u32(), 1);
        assert_eq!(stat.borrow()[0], vec![12]);
        e.register(GET_EXTRA_DATA_LIST, |_, a| ret(a[0] + 0x44));
        e.register(0x0041_8520, |_, _| ret(0));
        e.register(0x004b_fc80, |_, _| ret(1));
        assert!(!e.call(0x0088_0910, &args![actor]).bool());
        e.register(0x0041_8520, |_, _| ret(0x8000));
        assert!(e.call(0x0088_0910, &args![actor]).bool());
    }

    // ---- 00880950 and its three siblings ----

    /// An actor with everything the integer and float change functions use:
    /// (actor, source, report log, flag-bit log, hit log, modifier-list log).
    /// Index `0x99` has the flag `0x100`.
    fn change_fixture(e: &mut Engine) -> (Ptr<Actor>, Ptr<Actor>, Log, Log, Log, Log) {
        install_float_to_int(e);
        e.set_global(0x0101_2070, 1.0f64);
        let slots = [0x38c, 0x4b8, 0x48, 0x3a4, 0x3a0, 0x3ac, 0x3a8, 0x3b0];
        let this = actor(e, &slots);
        let source: Ptr<Actor> = object(e, 0x1b4, 0x0205_0000, &[]).cast();
        give_owner(e, this, &[0xc]);
        give_process(e, this, &[0x3a4, 0x3a0, 0x3ac, 0x3a8, 0x3b0]);
        e.register(ACTOR_VALUE_HAS_BASE, |_, _| ret(1));
        e.register(slot(OWNER_VTABLE, 0xc), |_, _| 3.0f32.into_ret());
        e.register(ACTOR_VALUE_HAS_FLAG, |_, a| {
            ret((a[1] == 0x100 && a[0] == 0x99) as u32)
        });
        e.register(0x0040_4040, |_, a| f32::from_bits(a[0]).into_ret());
        e.register(ACTOR_VALUE_IS_STAT, |_, _| ret(1));
        let report = record(e, ACTOR_VALUE_CHANGED, Ret::default());
        let flags = record(e, slot(VTABLE, 0x48), Ret::default());
        let hit = record(e, slot(VTABLE, 0x4b8), Ret::default());
        let list = record(e, 0x0093_7480, Ret::default());
        (this, source, report, flags, hit, list)
    }

    #[test]
    fn integer_change_calls_the_process_then_reports() {
        let mut e = new_engine();
        let (this, source, report, flags, hit, _) = change_fixture(&mut e);
        let process_call = record(&mut e, slot(PROCESS_VTABLE, 0x3a4), Ret::default());
        e.call(0x0088_0950, &args![this, 0x10u32, -4i32, source]);
        assert_eq!(
            process_call.borrow()[0][1..],
            [this.addr(), 0x10, (-4i32) as u32]
        );
        let expected = vec![this.addr(), source.addr(), (-4.0f32).to_bits()];
        assert_eq!(hit.borrow()[0], expected);
        assert_eq!(flags.borrow()[0], vec![this.addr(), 0x100000]);
        let expected = vec![
            this.addr() + 0xa4,
            0x10,
            3.0f32.to_bits(),
            (-4.0f32).to_bits(),
            source.addr() + 0xa4,
        ];
        assert_eq!(report.borrow()[0], expected);
        // The flag 0x100 index is ignored; index 0x16 with a negative value too.
        e.call(0x0088_0950, &args![this, 0x99u32, 4i32, source]);
        e.call(0x0088_0950, &args![this, 0x16u32, -4i32, source]);
        assert_eq!(report.borrow().len(), 1);
        e.register(slot(VTABLE, 0x38c), |_, _| ret(1));
        e.call(0x0088_0950, &args![this, 0x16u32, -4i32, source]);
        assert_eq!(report.borrow().len(), 2);
    }

    #[test]
    fn float_change_calls_the_process_then_reports() {
        let mut e = new_engine();
        let (this, source, report, flags, hit, _) = change_fixture(&mut e);
        let process_call = record(&mut e, slot(PROCESS_VTABLE, 0x3a0), Ret::default());
        e.call(0x0088_0ad0, &args![this, 0x10u32, -4.5f32, source]);
        let expected = [this.addr(), 0x10, (-4.5f32).to_bits()];
        assert_eq!(process_call.borrow()[0][1..], expected);
        let expected = vec![this.addr(), source.addr(), (-4.5f32).to_bits()];
        assert_eq!(hit.borrow()[0], expected);
        assert_eq!(flags.borrow()[0], vec![this.addr(), 0x100000]);
        assert_eq!(report.borrow()[0][3], (-4.5f32).to_bits());
        e.call(0x0088_0ad0, &args![this, 0x16u32, -1.0f32, source]);
        e.call(0x0088_0ad0, &args![this, 0x99u32, 1.0f32, source]);
        assert_eq!(report.borrow().len(), 1);
        // A non-negative amount does not call the hit function.
        e.call(0x0088_0ad0, &args![this, 0x10u32, 2.0f32, source]);
        assert_eq!(hit.borrow().len(), 1);
    }

    #[test]
    fn integer_change_to_the_modifier_list_at_0xd0() {
        let mut e = new_engine();
        let (this, source, report, flags, _, list) = change_fixture(&mut e);
        let process_call = record(&mut e, slot(PROCESS_VTABLE, 0x3b0), Ret::default());
        e.call(0x0088_0c70, &args![this, 0x10u32, 7i32, source]);
        let expected = vec![this.addr() + 0xd0, 0x10, 7.0f32.to_bits(), 2];
        assert_eq!(list.borrow()[0], expected);
        assert_eq!(flags.borrow()[0], vec![this.addr(), 0x800000]);
        assert_eq!(process_call.borrow()[0][1], 0x10);
        assert_eq!(report.borrow()[0][3], 7.0f32.to_bits());
        e.call(0x0088_0c70, &args![this, 0x99u32, 7i32, source]);
        assert_eq!(report.borrow().len(), 1);
    }

    #[test]
    fn float_change_to_the_modifier_list_at_0xd0() {
        let mut e = new_engine();
        let (this, source, report, flags, hit, list) = change_fixture(&mut e);
        let process_call = record(&mut e, slot(PROCESS_VTABLE, 0x3b0), Ret::default());
        e.call(0x0088_0e00, &args![this, 0x10u32, -2.5f32, source]);
        let expected = vec![this.addr() + 0xd0, 0x10, (-2.5f32).to_bits(), 2];
        assert_eq!(list.borrow()[0], expected);
        assert_eq!(flags.borrow()[0], vec![this.addr(), 0x800000]);
        assert_eq!(hit.borrow().len(), 1);
        assert_eq!(process_call.borrow()[0][1], 0x10);
        assert_eq!(report.borrow()[0][3], (-2.5f32).to_bits());
        e.call(0x0088_0e00, &args![this, 0x99u32, 1.0f32, source]);
        assert_eq!(report.borrow().len(), 1);
    }

    #[test]
    fn integer_change_variant_0x3ac_sets_flag_0x200000() {
        let mut e = new_engine();
        let (this, source, report, flags, _, _) = change_fixture(&mut e);
        let process_call = record(&mut e, slot(PROCESS_VTABLE, 0x3ac), Ret::default());
        e.call(0x0088_0fb0, &args![this, 0x20u32, 5i32, source]);
        assert_eq!(process_call.borrow()[0][1..], [this.addr(), 0x20, 5]);
        assert_eq!(flags.borrow()[0], vec![this.addr(), 0x200000]);
        assert_eq!(report.borrow()[0][3], 5.0f32.to_bits());
        e.call(0x0088_0fb0, &args![this, 0x99u32, 5i32, source]);
        assert_eq!(report.borrow().len(), 1);
    }

    #[test]
    fn float_change_variant_0x3a8_bounds_index_0x16_by_the_setting() {
        let mut e = new_engine();
        let (this, source, report, flags, _, _) = change_fixture(&mut e);
        let process_call = record(&mut e, slot(PROCESS_VTABLE, 0x3a8), Ret::default());
        // The setting (float at 0x011d2664 through 00403e20) is 1.0; the base is 3.0.
        let setting = e.mem.alloc(4);
        e.mem.set_f32(setting, 1.0);
        e.register_double(0x0040_3e20, move |_, _| ret(setting));
        e.call(0x0088_1130, &args![this, 0x20u32, 2.0f32, source]);
        let expected = [this.addr(), 0x20, 2.0f32.to_bits()];
        assert_eq!(process_call.borrow()[0][1..], expected);
        assert_eq!(flags.borrow()[0], vec![this.addr(), 0x200000]);
        // Index 0x16, amount -1: 3 - 1 = 2 stays above the setting: unchanged.
        e.register(slot(VTABLE, 0x38c), |_, _| ret(1));
        e.call(0x0088_1130, &args![this, 0x16u32, -1.0f32, source]);
        assert_eq!(process_call.borrow()[1][3], (-1.0f32).to_bits());
        // Amount -5: 3 - 5 is below the setting, so the amount becomes 1 - 3 = -2.
        e.call(0x0088_1130, &args![this, 0x16u32, -5.0f32, source]);
        assert_eq!(process_call.borrow()[2][3], (-2.0f32).to_bits());
        // The setting above the base: nothing happens.
        e.mem.set_f32(setting, 4.0);
        let before = report.borrow().len();
        e.call(0x0088_1130, &args![this, 0x16u32, -1.0f32, source]);
        assert_eq!(report.borrow().len(), before);
        // The flag 0x100 index is ignored.
        e.call(0x0088_1130, &args![this, 0x99u32, 1.0f32, source]);
        assert_eq!(report.borrow().len(), before);
    }

    // ---- 00881330 / 00881360 ----

    #[test]
    fn forwarding_value_changes() {
        let mut e = new_engine();
        let actor = actor(&mut e, &[0x3b4, 0x394]);
        give_owner(&mut e, actor, &[0xc, 4]);
        let direct = record(&mut e, slot(VTABLE, 0x3b4), Ret::default());
        e.call(0x0088_1330, &args![actor, 3u32, 9i32]);
        assert_eq!(direct.borrow()[0], vec![actor.addr(), 3, 9.0f32.to_bits()]);
        e.register(ACTOR_VALUE_HAS_BASE, |_, _| ret(1));
        e.register(ACTOR_VALUE_HAS_FLAG, |_, a| ret((a[0] == 0x99) as u32));
        e.register(ACTOR_VALUE_IS_STAT, |_, _| ret(1));
        e.register(slot(OWNER_VTABLE, 0xc), |_, _| 3.0f32.into_ret());
        e.register(slot(OWNER_VTABLE, 4), |_, _| 10.0f32.into_ret());
        let set = record(&mut e, slot(VTABLE, 0x394), Ret::default());
        let report = record(&mut e, ACTOR_VALUE_CHANGED, Ret::default());
        e.call(0x0088_1360, &args![actor, 7u32, 2.5f32]);
        assert_eq!(set.borrow()[0], vec![actor.addr(), 7, 12.5f32.to_bits()]);
        let expected = vec![
            actor.addr() + 0xa4,
            7,
            3.0f32.to_bits(),
            2.5f32.to_bits(),
            0,
        ];
        assert_eq!(report.borrow()[0], expected);
        e.call(0x0088_1360, &args![actor, 0x99u32, 2.5f32]);
        assert_eq!(set.borrow().len(), 1);
    }
}

#[cfg(test)]
mod tests_block3 {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const ACTOR_VTABLE: u32 = 0x0200_0000;
    const PROCESS_VTABLE: u32 = 0x0201_0000;
    const OBJECT_VTABLE: u32 = 0x0202_0000;
    const TARGET_VTABLE: u32 = 0x0203_0000;
    const MOVER_VTABLE: u32 = 0x0204_0000;

    /// The address and argument words of every call a double saw, in order.
    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// The address a test vtable slot jumps to.
    fn target(vtable: u32, offset: u32) -> u32 {
        0x6000_0000 + (vtable & 0x00ff_ffff) + offset
    }

    fn ret(value: u32) -> Ret {
        value.into_ret()
    }

    fn float_ret(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    /// Every exe function the translations of this block call by address.
    const CALLEES: &[u32] = &[
        REFERENCE_CURRENT_PACKAGE,
        REFERENCE_MISSED_PACKAGES,
        CALENDAR_GET_HOUR,
        CALENDAR_TIME_STAMP,
        IS_INTERRUPT_PACKAGE,
        EXTRA_PACKAGE,
        IS_FLEEING,
        WORD_AT_8,
        PROCESS_LEVEL,
        CURRENT_PROCESS_TYPE,
        MOBILE_OBJECT_PACKAGE,
        SET_LIFE_STATE,
        RESTORE_FULL_HEALTH,
        IS_ALARMED,
        IS_CONTINUING_PACKAGE_FOR_PC,
        UNLOCK_LOCK_DOORS_PROCEDURE,
        MOVE_TO_PROCESS_LEVEL,
        UPDATE_ALPHA,
        CAST_PERMANENT_MAGIC,
        DO_DEATH_STUFF,
        RUN_SCRIPT,
        ACTOR_STATE_QUERY,
        PARENT_CELL,
        WORLD_SPACE,
        GET_CONTAINER_CHANGES,
        GET_WORN_ITEM,
        REFERENCE_NAME,
        STRSTR,
        INIT_DEFAULT_WORN,
        GUARD_ENTER,
        GUARD_LEAVE,
        SHARED_OBJECT_TEST,
        REFERENCE_TEST,
        ALWAYS_FALSE,
        SHUTDOWN_CHECK,
        PROCESS_LISTS_REMOVE_REFERENCE,
        PROCESS_LISTS_ADD_REFERENCE,
        PROCESS_LISTS_REMOVE_ACTOR,
        PROCESS_LISTS_REMOVE_CLOSE_TO_PLAYER,
        PROCESS_LISTS_REFRESH_ACTOR,
        PROCESS_LISTS_NOTE_ACTOR,
        GET_ACCUMULATOR,
        ACCUMULATOR_ADD,
        ACTOR_SHADER_VALUE,
        MAGIC_TARGET_DISPEL,
        ACTOR_CLEAN_UP_A,
        ACTOR_CLEAN_UP_B,
        PROCESS_RESET,
        PROCESS_SET_TIME,
        ACTOR_TEST_00437BB0,
        HIGH_PROCESS_CONSTRUCTOR,
        LOW_PROCESS_CONSTRUCTOR,
        MIDDLE_LOW_PROCESS_CONSTRUCTOR,
        MIDDLE_HIGH_PROCESS_CONSTRUCTOR,
        ACTOR_NOTE_FAILURE,
        HIGH_PROCESS_SET_ACTOR,
        HIGH_PROCESS_INIT_ACTOR,
        GET_FORCE_NEXT_UPDATE,
        PACKAGE_TEST_0067A460,
        PACKAGE_GET_IS_CREATED,
        CELL_TEST_ACTOR,
        PACKAGE_LOCATION_REFERENCE,
        PACKAGE_LOCATION_CELL,
        PACKAGE_LOCATION_COORD,
        PACKAGE_LOCATION,
        PACKAGE_LOCATION_TYPE,
        PACKAGE_LOCATION_CHECK,
        POINT_CONSTRUCTOR,
        POINT_LENGTH,
        POINT_SUBTRACT,
        POINT_ADD,
        POINT_SCALE,
        POINT_LENGTH_2D,
        POINT_LENGTH_SQUARED,
        ACTOR_GET_RADIUS,
        FIND_CLOSEST_POINT_ON_NAVMESH,
        PATHING_LOCATION_CONSTRUCTOR,
        PATHING_LOCATION_DESTRUCTOR,
        PATH_POINT_CONSTRUCTOR,
        PATHING_FIND_POINT,
        REQUEST_CONSTRUCTOR,
        REQUEST_SET_ACTOR,
        REQUEST_SET_RADIUS,
        REQUEST_SET_OUTER_RADIUS,
        REQUEST_SUBMIT,
        REQUEST_DESTRUCTOR,
        PATHING_SINGLETON,
        PATHING_ORDER_MOVE,
        PATHING_ORDER_FINISH,
        PATHING_AVAILABLE,
        ACTOR_BLOCK_TEST_A,
        ACTOR_BLOCK_TEST_B,
        IS_PIPBOY_ACTIVE,
        GET_ANIM_ACTION,
        BASE_FLAG_TEST,
        BARTER_GOLD,
        ACTOR_BASE_CLASS,
        LAST_SEEN_INTERVAL,
        ENCOUNTER_ZONE,
        ENCOUNTER_ZONE_FLAG,
        DISABLE_RAGDOLL_ANIM,
        DO_KNOCK_DOWN,
        ACTOR_AFTER_ANIMATION,
        ACTOR_POSITION_OBJECT,
        POSITION_OBJECT_VALUE,
        RAGDOLL_SET_POSITION,
        ANIMATION_GROUP_LOADED,
        ANIMATION_FORCE_SECTION,
        ANIMATION_CURRENT,
        ANIMATION_UPDATE,
        ANIMATION_UPDATE_MOVEMENT,
        ANIMATION_ZERO_GLOBAL_TRANSFORM,
        ANIM_GROUP_GET_TIME,
        ANIMATION_TIME,
        ANIMATION_SET_TIME,
        MATRIX_SET_ANGLE,
        VECTOR_BUILD,
        MATRIX_TRANSFORM,
        GET_3D_NODE,
        GET_FORM_TYPE,
        GET_RAGDOLL_DATA,
        GET_SAVED_HAVOK_DATA,
        OBJECT_GETTER,
        OBJECT_SET_VALUE,
        MOVER_UPDATE_A,
        MOVER_UPDATE_B,
        ACTOR_SET_FLAG,
        WORN_ITEM_UPDATE,
        ACTOR_TEST_008C0050,
        ENUM_REFERENCES_CLOSE_TO_POINT,
        NOTE_LOOKUP,
        NOTE_TEST,
        ACTOR_SET_FLOAT,
        REFERENCE_FLOAT_POINTER,
        GET_BODY_PART_CONDITION,
        PACKAGE_TYPE,
        ANIMATION_FLAG_TEST,
        GET_ANIMATION,
        ACTOR_PROCESS,
        GET_EXTRA_DATA_LIST,
        GET_BASE_FORM,
        GET_FORM,
        OPERATOR_NEW,
        CLEAR_IN_COMBAT,
        FLAG_BIT_0_AT_0X24,
        0x005f_0b50,
        0x005f_0bf0,
        0x005f_0c40,
    ];

    /// An actor with a process, doubles over every callee and over every
    /// virtual slot of the test vtables (all returning zero until a test says
    /// otherwise), and a log of every call the doubles see.
    struct Rig {
        e: Engine,
        log: Log,
        actor: Ptr<Actor>,
        process: u32,
    }

    impl Rig {
        fn new() -> Rig {
            let mut e = Engine::new();
            for page in [
                0x0101_1000,
                0x0101_2000,
                0x0101_6000,
                0x0102_1000,
                0x0102_3000,
                0x0103_0000,
                0x011c_3000,
                0x011c_a000,
                0x011d_d000,
                0x011d_e000,
            ] {
                e.map(page, 0x1000);
            }
            let actor = Ptr::new(e.mem.alloc(0x1b4));
            let process = e.mem.alloc(0x400);
            let mut rig = Rig {
                e,
                log: Rc::default(),
                actor,
                process,
            };
            for vtable in [
                ACTOR_VTABLE,
                PROCESS_VTABLE,
                OBJECT_VTABLE,
                TARGET_VTABLE,
                MOVER_VTABLE,
            ] {
                rig.add_vtable(vtable);
            }
            for addr in CALLEES {
                rig.stub(*addr, Ret::default());
            }
            rig.e
                .register_double(ACTOR_PROCESS, |e, a| ret(e.mem.u32(a[0] + 0x68)));
            rig.e.mem.set_u32(actor.addr(), ACTOR_VTABLE);
            rig.e.mem.set_u32(actor.addr() + 0xa4, OBJECT_VTABLE);
            rig.e.mem.set_u32(process, PROCESS_VTABLE);
            rig.e.set(actor, Actor::pCurrentProcess, Ptr::new(process));
            rig
        }

        /// Maps a vtable page whose slots all log and return zero.
        fn add_vtable(&mut self, vtable: u32) {
            self.e.map(vtable, 0x1000);
            for offset in (0..0x800).step_by(4) {
                self.vslot(vtable, offset, Ret::default());
            }
        }

        /// A double over `addr` that logs its calls and returns `value`.
        fn stub(&mut self, addr: u32, value: Ret) {
            let log = self.log.clone();
            self.e.register_double(addr, move |_, a| {
                log.borrow_mut().push((addr, a.to_vec()));
                value
            });
        }

        /// A double that returns the values in turn (the last one again and again).
        fn stub_seq(&mut self, addr: u32, values: Vec<Ret>) {
            let log = self.log.clone();
            let mut index = 0;
            self.e.register_double(addr, move |_, a| {
                log.borrow_mut().push((addr, a.to_vec()));
                let value = values[index.min(values.len() - 1)];
                index += 1;
                value
            });
        }

        /// Makes slot `offset` of `vtable` log and return `value`.
        fn vslot(&mut self, vtable: u32, offset: u32, value: Ret) {
            self.e.mem.set_u32(vtable + offset, target(vtable, offset));
            self.stub(target(vtable, offset), value);
        }

        fn vslot_seq(&mut self, vtable: u32, offset: u32, values: Vec<Ret>) {
            self.e.mem.set_u32(vtable + offset, target(vtable, offset));
            self.stub_seq(target(vtable, offset), values);
        }

        /// An object of `size` bytes with `vtable`.
        fn object(&mut self, vtable: u32, size: u32) -> u32 {
            let object = self.e.mem.alloc(size);
            self.e.mem.set_u32(object, vtable);
            object
        }

        /// The argument words of every call to `addr`.
        fn calls(&self, addr: u32) -> Vec<Vec<u32>> {
            self.log
                .borrow()
                .iter()
                .filter(|(a, _)| *a == addr)
                .map(|(_, args)| args.clone())
                .collect()
        }

        /// The argument words of every call of slot `offset` of `vtable`.
        fn vcalls(&self, vtable: u32, offset: u32) -> Vec<Vec<u32>> {
            self.calls(target(vtable, offset))
        }

        /// The addresses of all calls so far, in order.
        fn order(&self) -> Vec<u32> {
            self.log.borrow().iter().map(|(a, _)| *a).collect()
        }

        fn clear(&self) {
            self.log.borrow_mut().clear();
        }

        /// Sets the process level the doubles report.
        fn level(&mut self, level: u32) {
            self.stub(PROCESS_LEVEL, ret(level));
        }
    }

    #[test]
    fn get_package_forwards_the_reference() {
        let mut r = Rig::new();
        let a = r.actor;
        r.stub(REFERENCE_CURRENT_PACKAGE, ret(0x1234));
        assert_eq!(actor_get_package(&mut r.e, a), Ptr::new(0x1234));
        assert_eq!(r.calls(REFERENCE_CURRENT_PACKAGE), vec![vec![a.addr()]]);
    }

    #[test]
    fn missed_packs_passes_the_calendar_hour_as_a_float() {
        let mut r = Rig::new();
        let a = r.actor;
        r.stub(CALENDAR_GET_HOUR, float_ret(13.5));
        actor_get_missed_packs(&mut r.e, a, 7, 2.5);
        assert_eq!(
            r.calls(REFERENCE_MISSED_PACKAGES),
            vec![vec![a.addr(), 7, 2.5f32.to_bits(), 13.5f32.to_bits()]]
        );
        assert_eq!(r.calls(CALENDAR_GET_HOUR), vec![vec![CALENDAR_OBJECT]]);
    }

    #[test]
    fn current_editor_package_falls_back_to_the_extra_data() {
        let mut r = Rig::new();
        let a = r.actor;
        r.stub(GET_EXTRA_DATA_LIST, ret(0x500));
        r.stub(EXTRA_PACKAGE, ret(0x900));
        r.vslot(PROCESS_VTABLE, 0x22c, ret(0x700));
        // A normal package is the answer.
        r.stub(IS_INTERRUPT_PACKAGE, ret(0));
        assert_eq!(actor_get_current_editor_package(&mut r.e, a).addr(), 0x700);
        // An interrupt package is replaced by the extra data's.
        r.stub(IS_INTERRUPT_PACKAGE, ret(1));
        assert_eq!(actor_get_current_editor_package(&mut r.e, a).addr(), 0x900);
        assert_eq!(r.calls(EXTRA_PACKAGE), vec![vec![0x500]]);
        // No package at all gives the extra data's too.
        r.vslot(PROCESS_VTABLE, 0x22c, ret(0));
        assert_eq!(actor_get_current_editor_package(&mut r.e, a).addr(), 0x900);
        // Without a process there is nothing.
        r.e.set(a, Actor::pCurrentProcess, Ptr::new(0));
        assert_eq!(actor_get_current_editor_package(&mut r.e, a).addr(), 0);
    }

    #[test]
    fn package_set_as_pcurrent_replaces_only_interrupt_packages() {
        let mut r = Rig::new();
        let a = r.actor;
        r.stub(GET_EXTRA_DATA_LIST, ret(0x500));
        r.stub(EXTRA_PACKAGE, ret(0x900));
        r.vslot(PROCESS_VTABLE, 0x22c, ret(0x700));
        r.stub(IS_INTERRUPT_PACKAGE, ret(0));
        assert_eq!(actor_get_package_set_as_pcurrent(&mut r.e, a).addr(), 0x700);
        r.stub(IS_INTERRUPT_PACKAGE, ret(1));
        assert_eq!(actor_get_package_set_as_pcurrent(&mut r.e, a).addr(), 0x900);
        // No package: the interrupt test is not even made.
        r.vslot(PROCESS_VTABLE, 0x22c, ret(0));
        r.clear();
        assert_eq!(actor_get_package_set_as_pcurrent(&mut r.e, a).addr(), 0);
        assert!(r.calls(IS_INTERRUPT_PACKAGE).is_empty());
        // No process.
        r.e.set(a, Actor::pCurrentProcess, Ptr::new(0));
        assert_eq!(actor_get_package_set_as_pcurrent(&mut r.e, a).addr(), 0);
    }

    #[test]
    fn is_running_run_once_asks_the_process() {
        let mut r = Rig::new();
        let a = r.actor;
        assert!(!actor_is_running_run_once(&mut r.e, a));
        r.vslot(PROCESS_VTABLE, 0x35c, ret(1));
        assert!(actor_is_running_run_once(&mut r.e, a));
    }

    #[test]
    fn fn_008815a0_picks_between_the_actor_and_the_process() {
        let mut r = Rig::new();
        let a = r.actor;
        r.vslot(PROCESS_VTABLE, 0x128, ret(0xaa));
        r.vslot(ACTOR_VTABLE, 0x42c, ret(0xbb));
        // The flag test fails: the process's answer.
        assert_eq!(fn_008815a0(&mut r.e, a).addr(), 0xaa);
        // The flag holds and the package is not run-once: the actor's answer.
        r.stub(ANIMATION_FLAG_TEST, ret(1));
        assert_eq!(fn_008815a0(&mut r.e, a).addr(), 0xbb);
        // Run-once and not fleeing: the process's answer again.
        r.vslot(PROCESS_VTABLE, 0x35c, ret(1));
        assert_eq!(fn_008815a0(&mut r.e, a).addr(), 0xaa);
        assert_eq!(r.calls(IS_FLEEING).last(), Some(&vec![a.addr(), 0]));
        // Run-once but fleeing: the actor's.
        r.stub(IS_FLEEING, ret(1));
        assert_eq!(fn_008815a0(&mut r.e, a).addr(), 0xbb);
        // The actor's answer is zero: back to the process's.
        r.vslot(ACTOR_VTABLE, 0x42c, ret(0));
        assert_eq!(fn_008815a0(&mut r.e, a).addr(), 0xaa);
        // No process.
        r.e.set(a, Actor::pCurrentProcess, Ptr::new(0));
        assert_eq!(fn_008815a0(&mut r.e, a).addr(), 0);
    }

    #[test]
    fn set_current_target_needs_a_process() {
        let mut r = Rig::new();
        let a = r.actor;
        actor_set_current_target(&mut r.e, a, 0x55);
        assert_eq!(r.vcalls(PROCESS_VTABLE, 0x12c), vec![vec![r.process, 0x55]]);
        r.e.set(a, Actor::pCurrentProcess, Ptr::new(0));
        actor_set_current_target(&mut r.e, a, 0x66);
        assert_eq!(r.vcalls(PROCESS_VTABLE, 0x12c).len(), 1);
    }

    #[test]
    fn current_package_target_reads_behind_the_process() {
        let mut r = Rig::new();
        let a = r.actor;
        r.stub(WORD_AT_8, ret(0x42));
        assert_eq!(actor_get_current_package_target(&mut r.e, a), 0x42);
        assert_eq!(r.calls(WORD_AT_8), vec![vec![r.process + 4]]);
        r.e.set(a, Actor::pCurrentProcess, Ptr::new(0));
        assert_eq!(actor_get_current_package_target(&mut r.e, a), 0);
    }

    #[test]
    fn end_interrupt_package_ends_an_interrupt_package() {
        let mut r = Rig::new();
        let a = r.actor;
        let player = r.object(ACTOR_VTABLE, 0x300);
        let package = r.e.mem.alloc(0xa0);
        r.e.mem.set_u32(package + 0x98, 10);
        r.e.mem.set_u32(player + 0x208, package);
        r.e.set_global::<u32>(PLAYER_POINTER, player);
        // The process's package is the interrupt package (type 0x17), later none.
        r.vslot_seq(PROCESS_VTABLE, 0x22c, vec![ret(package), ret(0)]);
        r.vslot(PROCESS_VTABLE, 0x20c, ret(0x800));
        r.stub(IS_INTERRUPT_PACKAGE, ret(1));
        r.stub_seq(PACKAGE_TYPE, vec![ret(0x10), ret(0x17)]);
        actor_end_interrupt_package(&mut r.e, a, false);
        // The player's pending package is cleared, the package's counter lowered.
        assert_eq!(r.e.mem.u32(player + 0x208), 0);
        assert_eq!(r.e.mem.u32(package + 0x98), 9);
        let order = r.order();
        let position = |addr: u32| order.iter().position(|x| *x == addr).unwrap();
        assert!(position(target(ACTOR_VTABLE, 0x288)) < position(target(PROCESS_VTABLE, 0x22c)));
        assert_eq!(r.vcalls(PROCESS_VTABLE, 0x214), vec![vec![r.process]]);
        assert_eq!(r.calls(CLEAR_IN_COMBAT), vec![vec![a.addr(), 1]]);
        assert_eq!(r.vcalls(ACTOR_VTABLE, 0x4c), vec![vec![a.addr(), 0]]);
        assert_eq!(
            r.vcalls(PROCESS_VTABLE, 0x714),
            vec![vec![r.process, a.addr()]]
        );
        // No package left: with no shutdown pending the process gets the actor again.
        assert_eq!(
            r.vcalls(PROCESS_VTABLE, 0x24),
            vec![vec![r.process, a.addr(), 0]]
        );
        // And the movement ends.
        assert_eq!(
            r.vcalls(PROCESS_VTABLE, 0x294),
            vec![vec![r.process, a.addr()]]
        );
    }

    #[test]
    fn end_interrupt_package_keeps_the_idle_package_when_asked() {
        let mut r = Rig::new();
        let a = r.actor;
        let player = r.object(ACTOR_VTABLE, 0x300);
        r.e.set_global::<u32>(PLAYER_POINTER, player);
        r.vslot(PROCESS_VTABLE, 0x20c, ret(0x800));
        r.stub(PACKAGE_TYPE, ret(0x1c));
        actor_end_interrupt_package(&mut r.e, a, true);
        // Type 0x1c with the flag set is left alone; the actor's +0x288 is skipped.
        assert!(r.vcalls(ACTOR_VTABLE, 0x288).is_empty());
        assert!(r.vcalls(PROCESS_VTABLE, 0x214).is_empty());
        assert!(r.vcalls(PROCESS_VTABLE, 0x294).is_empty());
        // Without the flag the same package is dropped.
        actor_end_interrupt_package(&mut r.e, a, false);
        assert_eq!(r.vcalls(ACTOR_VTABLE, 0x288).len(), 1);
        assert_eq!(r.vcalls(PROCESS_VTABLE, 0x214).len(), 1);
        assert_eq!(r.vcalls(PROCESS_VTABLE, 0x294).len(), 1);
        // Type 0x1a is never dropped.
        r.stub(PACKAGE_TYPE, ret(0x1a));
        actor_end_interrupt_package(&mut r.e, a, false);
        assert_eq!(r.vcalls(PROCESS_VTABLE, 0x214).len(), 1);
        // No process, nothing at all.
        r.clear();
        r.e.set(a, Actor::pCurrentProcess, Ptr::new(0));
        actor_end_interrupt_package(&mut r.e, a, false);
        assert!(r.order().is_empty());
    }

    #[test]
    fn fn_00881830_adds_to_the_counter() {
        let mut e = Engine::new();
        let package = Ptr::new(e.mem.alloc(0xa0));
        e.mem.set_u32(package.addr() + 0x98, 5);
        fn_00881830(&mut e, package, -1);
        assert_eq!(e.mem.u32(package.addr() + 0x98), 4);
        fn_00881830(&mut e, package, 10);
        assert_eq!(e.mem.u32(package.addr() + 0x98), 14);
        e.mem.set_u32(package.addr() + 0x98, 0);
        fn_00881830(&mut e, package, -1);
        assert_eq!(e.mem.u32(package.addr() + 0x98), u32::MAX);
    }

    /// An actor that may move, with the position `(1, 2, 3)` behind its
    /// virtual `+0x1f4`; returns the position's address.
    fn mobile_rig(r: &mut Rig) -> u32 {
        let position = r.e.mem.alloc(12);
        for (i, v) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            r.e.mem.set_f32(position + 4 * i as u32, *v);
        }
        r.vslot(ACTOR_VTABLE, 0x1f4, ret(position));
        let player = r.object(ACTOR_VTABLE, 0x300);
        r.e.set_global::<u32>(PLAYER_POINTER, player);
        r.stub(ACTOR_GET_RADIUS, float_ret(1.0));
        r.e.mem.set_f64(ONE_AND_HALF_DOUBLE, 1.5);
        r.e.mem.set_f64(THREE_DOUBLE, 3.0);
        position
    }

    #[test]
    fn fn_00881860_refuses_an_actor_that_cannot_move() {
        let mut r = Rig::new();
        let a = r.actor;
        mobile_rig(&mut r);
        r.vslot(ACTOR_VTABLE, 0x234, ret(1));
        assert!(!fn_00881860(&mut r.e, a, false));
        assert!(r.calls(PARENT_CELL).is_empty());
        // `force` goes on anyway (and with no navmesh point just orders the move).
        assert!(fn_00881860(&mut r.e, a, true));
        assert_eq!(r.calls(PARENT_CELL).len(), 1);
    }

    #[test]
    fn fn_00881860_lets_the_package_veto() {
        let mut r = Rig::new();
        let a = r.actor;
        mobile_rig(&mut r);
        let package = r.object(OBJECT_VTABLE, 0x100);
        r.vslot(PROCESS_VTABLE, 0x22c, ret(package));
        r.e.mem.set_f32(MINUS_ONE_FLOAT, -1.0);
        r.vslot(OBJECT_VTABLE, 0x13c, ret(1));
        r.stub(PACKAGE_TYPE, ret(6));
        r.stub(PACKAGE_LOCATION_REFERENCE, ret(0x77));
        r.stub(GET_FORM, ret(0x1111));
        r.e.set_global::<u32>(LOCATION_FORM_POINTER_011CA248, 0x1111);
        assert!(!fn_00881860(&mut r.e, a, false));
        assert_eq!(
            r.vcalls(OBJECT_VTABLE, 0x13c),
            vec![vec![package, a.addr(), 0, (-1.0f32).to_bits(), 0]]
        );
        // A different form does not veto.
        r.e.set_global::<u32>(LOCATION_FORM_POINTER_011CA248, 0x2222);
        r.stub(PACKAGE_LOCATION, ret(0));
        assert!(fn_00881860(&mut r.e, a, false));
        // A location of type 3 that the package rejects does.
        r.stub(PACKAGE_LOCATION, ret(0x88));
        r.stub(PACKAGE_LOCATION_TYPE, ret(3));
        r.stub(PACKAGE_LOCATION_CHECK, ret(0));
        assert!(!fn_00881860(&mut r.e, a, false));
        r.stub(PACKAGE_LOCATION_CHECK, ret(1));
        assert!(fn_00881860(&mut r.e, a, false));
    }

    #[test]
    fn fn_00881860_orders_the_move_to_the_snapped_point() {
        let mut r = Rig::new();
        let a = r.actor;
        let position = mobile_rig(&mut r);
        // The navmesh has a point; the path finder finds a close one at distance
        // (3, 4) horizontally, inside the radius 1.0? No: farther, so no push.
        r.stub(FIND_CLOSEST_POINT_ON_NAVMESH, ret(1));
        r.stub(PATHING_FIND_POINT, ret(1));
        r.stub(POINT_LENGTH_2D, float_ret(5.0));
        // The player is far away: no close-point request.
        r.stub(POINT_LENGTH_SQUARED, float_ret(100.0));
        // The 0x2a8 double reads the three position words it is given.
        let seen = Rc::new(RefCell::new(vec![]));
        let seen_in = seen.clone();
        let slot = target(ACTOR_VTABLE, 0x2a8);
        r.e.register_double(slot, move |e, args| {
            seen_in.borrow_mut().push([
                e.mem.f32(args[1]),
                e.mem.f32(args[1] + 4),
                e.mem.f32(args[1] + 8),
            ]);
            Ret::default()
        });
        assert!(fn_00881860(&mut r.e, a, false));
        assert_eq!(*seen.borrow(), vec![[1.0, 2.0, 3.0]]);
        assert_eq!(
            r.calls(FIND_CLOSEST_POINT_ON_NAVMESH)[0][2],
            position,
            "the actor's position is the third argument"
        );
        assert_eq!(r.calls(PATHING_LOCATION_DESTRUCTOR).len(), 1);
        assert!(r.calls(REQUEST_CONSTRUCTOR).is_empty());
        assert!(r.calls(PATHING_ORDER_MOVE).is_empty());
    }

    #[test]
    fn fn_00881860_pulls_away_from_the_player() {
        let mut r = Rig::new();
        let a = r.actor;
        mobile_rig(&mut r);
        r.stub(FIND_CLOSEST_POINT_ON_NAVMESH, ret(1));
        r.stub(PATHING_FIND_POINT, ret(1));
        // The horizontal distance 0.5 is inside the radius 1.0: the point is
        // pushed out by the radius.
        r.stub(POINT_LENGTH_2D, float_ret(0.5));
        let scaled = r.e.mem.alloc(12);
        let moved = r.e.mem.alloc(12);
        for (i, v) in [7.0f32, 8.0, 9.0].iter().enumerate() {
            r.e.mem.set_f32(moved + 4 * i as u32, *v);
        }
        r.stub(POINT_SCALE, ret(scaled));
        r.stub(POINT_ADD, ret(moved));
        // The player is close: radii 1 + 1, squared 4 * 2.25 = 9 > 4.
        r.stub(POINT_LENGTH_SQUARED, float_ret(4.0));
        r.stub(PATHING_AVAILABLE, ret(1));
        r.stub(PATHING_SINGLETON, ret(0x5000));
        let ordered = Rc::new(RefCell::new(vec![]));
        let ordered_in = ordered.clone();
        r.e.register_double(PATHING_ORDER_MOVE, move |e, args| {
            ordered_in.borrow_mut().push([
                e.mem.f32(args[2]),
                e.mem.f32(args[2] + 4),
                e.mem.f32(args[2] + 8),
            ]);
            Ret::default()
        });
        assert!(fn_00881860(&mut r.e, a, false));
        assert_eq!(*ordered.borrow(), vec![[7.0, 8.0, 9.0]]);
        assert_eq!(r.calls(POINT_SCALE)[0][2], 1.0f32.to_bits());
        let player = r.e.global::<u32>(PLAYER_POINTER);
        let request = r.calls(REQUEST_CONSTRUCTOR)[0][0];
        assert_eq!(r.calls(REQUEST_SET_ACTOR), vec![vec![request, player]]);
        assert_eq!(
            r.calls(REQUEST_SET_RADIUS),
            vec![vec![request, 3.0f32.to_bits()]]
        );
        assert_eq!(
            r.calls(REQUEST_SET_OUTER_RADIUS),
            vec![vec![request, 6.0f32.to_bits()]]
        );
        assert_eq!(r.calls(REQUEST_SUBMIT)[0][0], request);
        assert_eq!(r.calls(REQUEST_DESTRUCTOR), vec![vec![request]]);
        // The move goes through the pathing singleton, with the pushed-out point.
        assert_eq!(r.calls(PATHING_ORDER_FINISH), vec![vec![0x5000, a.addr()]]);
        assert!(r.vcalls(ACTOR_VTABLE, 0x2a8).is_empty());
    }

    #[test]
    fn fn_00881c90_waits_for_the_interval() {
        let mut r = Rig::new();
        let a = r.actor;
        r.stub(LAST_SEEN_INTERVAL, ret(100));
        // Never seen: always true.
        assert!(fn_00881c90(&mut r.e, a, 5));
        r.e.set(a, Actor::iLastSeenTime, 1000);
        // Not later than the last time: false.
        assert!(!fn_00881c90(&mut r.e, a, 1000));
        assert!(!fn_00881c90(&mut r.e, a, 900));
        // Later but within the interval: false.
        assert!(!fn_00881c90(&mut r.e, a, 1100));
        // Beyond the interval: true unless the encounter zone is flagged.
        assert!(fn_00881c90(&mut r.e, a, 1101));
        r.stub(ENCOUNTER_ZONE, ret(0x30));
        assert!(fn_00881c90(&mut r.e, a, 1101));
        r.stub(ENCOUNTER_ZONE_FLAG, ret(1));
        assert!(!fn_00881c90(&mut r.e, a, 1101));
        assert_eq!(r.calls(ENCOUNTER_ZONE_FLAG).last(), Some(&vec![0x30]));
    }

    #[test]
    fn fn_00881d10_stores_the_calendar_stamp() {
        let mut r = Rig::new();
        let a = r.actor;
        r.stub(CALENDAR_TIME_STAMP, ret(0x1234));
        fn_00881d10(&mut r.e, a);
        assert_eq!(r.e.get(a, Actor::iLastSeenTime), 0x1234);
        assert_eq!(r.calls(CALENDAR_TIME_STAMP), vec![vec![CALENDAR_OBJECT]]);
    }

    #[test]
    fn fn_00881d30_refreshes_the_worn_item_of_a_high_actor() {
        let mut r = Rig::new();
        let a = r.actor;
        r.level(0);
        r.vslot(ACTOR_VTABLE, 0x1d0, ret(0x55));
        r.stub(GET_CONTAINER_CHANGES, ret(0x66));
        r.stub(GET_WORN_ITEM, ret(0x99));
        r.vslot(PROCESS_VTABLE, 0x31c, ret(1));
        assert!(fn_00881d30(&mut r.e, a));
        assert_eq!(r.calls(GET_WORN_ITEM), vec![vec![0x66, 5, 0]]);
        assert_eq!(
            r.vcalls(PROCESS_VTABLE, 0x160),
            vec![vec![r.process, 0x99, 0, 1]]
        );
        assert_eq!(r.vcalls(PROCESS_VTABLE, 0x450), vec![vec![r.process, 1]]);
        // Nothing replaced the process.
        assert_eq!(r.e.get(a, Actor::pCurrentProcess), Ptr::new(r.process));
        assert!(r.calls(HIGH_PROCESS_CONSTRUCTOR).is_empty());
    }

    #[test]
    fn fn_00881d30_gives_lily_her_3d_node() {
        let mut r = Rig::new();
        let a = r.actor;
        r.level(0);
        r.vslot(ACTOR_VTABLE, 0x1d0, ret(0x55));
        r.stub(GET_CONTAINER_CHANGES, ret(0x66));
        r.stub(GET_WORN_ITEM, ret(0x99));
        r.stub(GET_FORCE_NEXT_UPDATE, ret(1));
        r.vslot(ACTOR_VTABLE, 0x21c, ret(1));
        r.stub(REFERENCE_NAME, ret(0x7000));
        r.stub(STRSTR, ret(0x7002));
        // A package the process accepts: the same update step.
        r.vslot(PROCESS_VTABLE, 0x22c, ret(0x700));
        r.stub(PACKAGE_TEST_0067A460, ret(1));
        assert!(fn_00881d30(&mut r.e, a));
        assert_eq!(r.calls(STRSTR), vec![vec![0x7000, LILY_TEXT]]);
        assert_eq!(
            r.vcalls(PROCESS_VTABLE, 0x160),
            vec![vec![r.process, 0x99, 0x55, 1]]
        );
        assert_eq!(r.calls(PACKAGE_TEST_0067A460), vec![vec![0x700]]);
        assert_eq!(r.vcalls(PROCESS_VTABLE, 0x450), vec![vec![r.process, 1]]);
        // Without a process there is nothing to do.
        r.clear();
        r.e.set(a, Actor::pCurrentProcess, Ptr::new(0));
        assert!(fn_00881d30(&mut r.e, a));
        assert!(r.calls(GET_CONTAINER_CHANGES).is_empty());
    }

    /// A rig for the transitions: a new process of `size` bytes comes from
    /// `operator new` and its constructor; returns its address.
    fn new_process(r: &mut Rig, constructor: u32) -> u32 {
        let new_process = r.object(PROCESS_VTABLE, 0x500);
        r.stub(OPERATOR_NEW, ret(0x6666));
        r.stub(constructor, ret(new_process));
        new_process
    }

    #[test]
    fn fn_00881d30_builds_a_high_process_for_a_low_actor() {
        let mut r = Rig::new();
        let a = r.actor;
        let high = new_process(&mut r, HIGH_PROCESS_CONSTRUCTOR);
        r.level(3);
        // The reference float is not the "unset" marker, the note object says no:
        // level 3 asks for a move, which the actor (virtual +0x234) cannot do.
        let float = r.e.mem.alloc(16);
        r.e.mem.set_f32(float, 1.0);
        r.stub(REFERENCE_FLOAT_POINTER, ret(float));
        r.e.mem.set_f64(FLT_MAX_DOUBLE, f32::MAX as f64);
        r.vslot(ACTOR_VTABLE, 0x234, ret(1));
        r.vslot(PROCESS_VTABLE, 0x22c, ret(0x700));
        assert!(fn_00881d30(&mut r.e, a));
        // The old process is removed from the lists at level 3 and destroyed,
        // the new one installed and the actor added again at level 0.
        assert_eq!(
            r.calls(PROCESS_LISTS_REMOVE_REFERENCE)[0],
            vec![PROCESS_LISTS, a.addr(), 3]
        );
        assert_eq!(r.calls(OPERATOR_NEW), vec![vec![0x46c]]);
        assert_eq!(r.calls(HIGH_PROCESS_CONSTRUCTOR), vec![vec![0x6666]]);
        assert_eq!(r.vcalls(PROCESS_VTABLE, 4), vec![vec![high, r.process]]);
        assert_eq!(r.vcalls(PROCESS_VTABLE, 0), vec![vec![r.process, 1]]);
        assert_eq!(r.e.get(a, Actor::pCurrentProcess), Ptr::new(high));
        assert_eq!(
            r.calls(PROCESS_LISTS_ADD_REFERENCE),
            vec![vec![PROCESS_LISTS, a.addr(), 0, 0, 0, 0]]
        );
        // The move failed: the failure is noted.
        assert_eq!(r.calls(ACTOR_NOTE_FAILURE), vec![vec![a.addr()]]);
        assert_eq!(r.calls(HIGH_PROCESS_SET_ACTOR), vec![vec![high, a.addr()]]);
        assert_eq!(r.calls(HIGH_PROCESS_INIT_ACTOR), vec![vec![high, a.addr()]]);
        // It ends with the new process's virtual +0x7a8.
        assert_eq!(r.vcalls(PROCESS_VTABLE, 0x7a8), vec![vec![high, a.addr()]]);
        // The guard was entered and left once around the swap.
        assert_eq!(r.calls(GUARD_ENTER), vec![vec![GUARD_OBJECT_011F11A0, 0]]);
        assert_eq!(r.calls(GUARD_LEAVE), vec![vec![GUARD_OBJECT_011F11A0]]);
    }

    #[test]
    fn fn_00881d30_wakes_a_flagged_actor_with_a_knock_down() {
        let mut r = Rig::new();
        let a = r.actor;
        let high = new_process(&mut r, HIGH_PROCESS_CONSTRUCTOR);
        mobile_rig(&mut r);
        r.level(3);
        let float = r.e.mem.alloc(16);
        r.e.mem.set_f32(float, f32::MAX);
        r.e.mem.set_f64(FLT_MAX_DOUBLE, f32::MAX as f64);
        r.stub(REFERENCE_FLOAT_POINTER, ret(float));
        // A 3D node exists, so the dead flag (set below) is acted on; the actor
        // has no animation, so it is knocked down from its own rotation.
        r.stub(GET_3D_NODE, ret(0x333));
        r.e.set(a, Actor::bDeadFlag, true);
        r.vslot(ACTOR_VTABLE, 0x2bc, float_ret(0.5));
        let rotated = r.e.mem.alloc(12);
        for (i, v) in [0.0f32, 0.0, 1.0].iter().enumerate() {
            r.e.mem.set_f32(rotated + 4 * i as u32, *v);
        }
        r.stub(MATRIX_TRANSFORM, ret(rotated));
        // The vector DoKnockDown gets is the rotated one.
        let seen = Rc::new(RefCell::new(vec![]));
        let seen_in = seen.clone();
        r.e.register_double(DO_KNOCK_DOWN, move |e, args| {
            seen_in.borrow_mut().push((
                args.to_vec(),
                [
                    e.mem.f32(args[1]),
                    e.mem.f32(args[1] + 4),
                    e.mem.f32(args[1] + 8),
                ],
            ));
            Ret::default()
        });
        assert!(fn_00881d30(&mut r.e, a));
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].0[0], 0x333);
        assert_eq!(&seen[0].0[2..], &[1, 0.0f32.to_bits(), 1]);
        assert_eq!(seen[0].1, [0.0, 0.0, 1.0]);
        assert_eq!(r.calls(MATRIX_SET_ANGLE)[0][1], 0.5f32.to_bits());
        assert_eq!(r.calls(SET_LIFE_STATE), vec![vec![a.addr(), 1]]);
        assert_eq!(r.vcalls(ACTOR_VTABLE, 0x48), vec![vec![a.addr(), 4]]);
        // The flag is cleared and the new process told.
        assert!(!r.e.get(a, Actor::bDeadFlag));
        assert_eq!(r.vcalls(PROCESS_VTABLE, 0x28), vec![vec![high]]);
        assert_eq!(r.calls(UPDATE_ALPHA), vec![vec![a.addr()]]);
    }

    #[test]
    fn fn_00881d30_forces_the_death_pose_section() {
        let mut r = Rig::new();
        let a = r.actor;
        let high = new_process(&mut r, HIGH_PROCESS_CONSTRUCTOR);
        mobile_rig(&mut r);
        r.level(3);
        let float = r.e.mem.alloc(16);
        r.e.mem.set_f32(float, f32::MAX);
        r.e.mem.set_f64(FLT_MAX_DOUBLE, f32::MAX as f64);
        r.stub(REFERENCE_FLOAT_POINTER, ret(float));
        r.stub(GET_3D_NODE, ret(0x333));
        r.e.set(a, Actor::bDeadFlag, true);
        r.stub(GET_ANIMATION, ret(0x4000));
        r.stub(ANIMATION_GROUP_LOADED, ret(1));
        r.stub(ANIMATION_CURRENT, ret(0x4400));
        r.stub(ANIMATION_TIME, float_ret(0.25));
        // The actor's virtual +0x38c is false: it updates at the current time.
        r.e.set(a, Actor::pRagdollController, Ptr::new(0x8000));
        assert!(fn_00881d30(&mut r.e, a));
        assert_eq!(r.calls(DISABLE_RAGDOLL_ANIM), vec![vec![0x8000, 1]]);
        assert_eq!(
            r.calls(ANIMATION_FORCE_SECTION),
            vec![vec![
                0x4000,
                0x14,
                0xe0,
                u32::MAX,
                0.0f32.to_bits(),
                u32::MAX
            ]]
        );
        assert_eq!(r.calls(ANIMATION_CURRENT), vec![vec![0x4000, 1]]);
        assert_eq!(
            r.calls(ANIMATION_UPDATE),
            vec![vec![0x4000, a.addr(), 0.0f32.to_bits(), 0.25f32.to_bits()]]
        );
        assert_eq!(
            r.calls(ANIMATION_SET_TIME),
            vec![vec![0x4400, 0.25f32.to_bits()]]
        );
        assert_eq!(
            r.calls(ANIMATION_UPDATE_MOVEMENT),
            vec![vec![0x4000, a.addr()]]
        );
        assert_eq!(r.calls(ACTOR_AFTER_ANIMATION), vec![vec![a.addr()]]);
        assert_eq!(r.calls(SET_LIFE_STATE), vec![vec![a.addr(), 1]]);
        // No knock-down on this path.
        assert!(r.calls(DO_KNOCK_DOWN).is_empty());
        assert_eq!(r.vcalls(PROCESS_VTABLE, 0x28), vec![vec![high]]);
        assert!(!r.e.get(a, Actor::bDeadFlag));
    }

    /// A rig for the three lower transitions: the actor is in the high process,
    /// the new process comes from `constructor`.
    fn transition_rig(constructor: u32) -> (Rig, u32) {
        let mut r = Rig::new();
        let new = new_process(&mut r, constructor);
        r.level(0);
        r.stub(CALENDAR_TIME_STAMP, ret(77));
        r.stub(GET_ACCUMULATOR, ret(0x9000));
        r.stub(ACTOR_SHADER_VALUE, ret(0x31));
        r.stub(ACTOR_STATE_QUERY, ret(0));
        (r, new)
    }

    #[test]
    fn fn_00882b90_stops_for_an_actor_that_is_low_already() {
        let mut r = Rig::new();
        let a = r.actor;
        r.level(3);
        assert!(fn_00882b90(&mut r.e, a));
        assert_eq!(r.order(), vec![PROCESS_LEVEL]);
    }

    #[test]
    fn fn_00882b90_moves_a_high_actor_to_the_low_process() {
        let (mut r, low) = transition_rig(LOW_PROCESS_CONSTRUCTOR);
        let a = r.actor;
        // Dead-body handling: dead actor runs the death hooks.
        r.vslot(ACTOR_VTABLE, 0x2e8, ret(1));
        r.stub(IS_CONTINUING_PACKAGE_FOR_PC, ret(1));
        assert!(fn_00882b90(&mut r.e, a));
        // The high actor is stamped, its process reset and its shader value
        // handed to the accumulator.
        assert_eq!(r.calls(CALENDAR_TIME_STAMP).len(), 1);
        assert_eq!(r.e.get(a, Actor::iLastSeenTime), 77);
        assert_eq!(r.calls(PROCESS_RESET), vec![vec![r.process]]);
        assert_eq!(r.calls(ACCUMULATOR_ADD), vec![vec![0x9000, 0x31]]);
        // The death hooks.
        assert_eq!(r.calls(DO_DEATH_STUFF), vec![vec![a.addr()]]);
        assert_eq!(
            r.vcalls(PROCESS_VTABLE, 0x354),
            vec![vec![r.process, a.addr()]]
        );
        assert_eq!(r.calls(RUN_SCRIPT), vec![vec![a.addr()]]);
        // Removed from the lists at level 0, then added at level 3.
        assert_eq!(
            r.calls(PROCESS_LISTS_REMOVE_REFERENCE),
            vec![vec![PROCESS_LISTS, a.addr(), 0]]
        );
        assert_eq!(r.calls(OPERATOR_NEW), vec![vec![0xb4]]);
        assert_eq!(r.e.get(a, Actor::pCurrentProcess), Ptr::new(low));
        assert_eq!(
            r.calls(PROCESS_LISTS_ADD_REFERENCE),
            vec![vec![PROCESS_LISTS, a.addr(), 3, 1, 0, 0]]
        );
        // A continuing package for the player: the new process gets the actor.
        assert_eq!(r.vcalls(PROCESS_VTABLE, 0x24), vec![vec![low, a.addr(), 0]]);
        // The old high actor's mover is told.
        assert_eq!(r.calls(ACTOR_CLEAN_UP_A), vec![vec![a.addr()]]);
        assert_eq!(r.calls(ACTOR_CLEAN_UP_B), vec![vec![a.addr()]]);
        assert_eq!(r.calls(MAGIC_TARGET_DISPEL), vec![vec![a.addr() + 0x94]]);
        // And the ending: the process's +0x7a8 and the actor's +0x4c(0x100000).
        assert_eq!(r.vcalls(PROCESS_VTABLE, 0x7a8), vec![vec![low, a.addr()]]);
        assert_eq!(
            r.vcalls(ACTOR_VTABLE, 0x4c),
            vec![vec![a.addr(), 0x0010_0000]]
        );
    }

    #[test]
    fn fn_00882b90_reschedules_a_follower_one_hour_early() {
        let (mut r, low) = transition_rig(LOW_PROCESS_CONSTRUCTOR);
        let a = r.actor;
        // A shutdown object exists and is not shutting down.
        r.e.set_global::<u32>(SHUTDOWN_CHECK_OBJECT_POINTER, 0x1234);
        r.e.mem.set_f64(ONE_DOUBLE, 1.0);
        r.stub(CALENDAR_GET_HOUR, float_ret(10.0));
        // The actor's +0x160 holds and the force flag is set: it is due.
        r.vslot(ACTOR_VTABLE, 0x160, ret(1));
        r.e.mem.set_u8(a.addr() + 0x126, 1);
        assert!(fn_00882b90(&mut r.e, a));
        assert_eq!(r.calls(PROCESS_SET_TIME), vec![vec![low, 9.0f32.to_bits()]]);
        // The actor leaves and re-enters the lists at level 3.
        assert_eq!(
            r.calls(PROCESS_LISTS_REMOVE_REFERENCE).last(),
            Some(&vec![PROCESS_LISTS, a.addr(), 3])
        );
        assert_eq!(
            r.calls(PROCESS_LISTS_ADD_REFERENCE).last(),
            Some(&vec![PROCESS_LISTS, a.addr(), 3, 0, 0, 0])
        );
        assert_eq!(r.e.mem.u8(a.addr() + 0x126), 1);
        // With the animation flag, the move goes to `008a0680` instead.
        let (mut r, _low) = transition_rig(LOW_PROCESS_CONSTRUCTOR);
        let a = r.actor;
        r.e.set_global::<u32>(SHUTDOWN_CHECK_OBJECT_POINTER, 0x1234);
        r.stub(ANIMATION_FLAG_TEST, ret(1));
        assert!(fn_00882b90(&mut r.e, a));
        assert_eq!(r.calls(MOVE_TO_PROCESS_LEVEL), vec![vec![a.addr(), 0, 3]]);
        assert!(r.calls(PROCESS_SET_TIME).is_empty());
    }

    #[test]
    fn fn_00882b90_ends_the_idle_package_of_a_stranded_actor() {
        let (mut r, _low) = transition_rig(LOW_PROCESS_CONSTRUCTOR);
        let a = r.actor;
        let player = r.object(ACTOR_VTABLE, 0x300);
        r.e.set_global::<u32>(PLAYER_POINTER, player);
        // The actor's package is the idle one (0x1c) and its target is not the
        // player: the package ends.
        r.vslot(PROCESS_VTABLE, 0x22c, ret(0x700));
        r.vslot(PROCESS_VTABLE, 0x128, ret(0x1111));
        r.stub(PACKAGE_TYPE, ret(0x1c));
        assert!(fn_00882b90(&mut r.e, a));
        assert!(!r.vcalls(ACTOR_VTABLE, 0x288).is_empty());
        // 4f8960 == 6: the actor is restored first.
        let (mut r, _low) = transition_rig(LOW_PROCESS_CONSTRUCTOR);
        let a = r.actor;
        r.stub(ACTOR_STATE_QUERY, ret(6));
        assert!(fn_00882b90(&mut r.e, a));
        assert_eq!(r.calls(SET_LIFE_STATE), vec![vec![a.addr(), 0]]);
        assert_eq!(r.calls(RESTORE_FULL_HEALTH), vec![vec![a.addr()]]);
        assert_eq!(
            r.vcalls(PROCESS_VTABLE, 0xe8),
            vec![vec![r.process, 0.0f32.to_bits()]]
        );
        assert_eq!(
            r.vcalls(PROCESS_VTABLE, 0x420),
            vec![vec![r.process, a.addr()]]
        );
    }

    #[test]
    fn fn_00883240_stops_for_an_actor_that_is_middle_low_already() {
        let mut r = Rig::new();
        let a = r.actor;
        r.level(2);
        r.e.set(a, Actor::cCurrentSitSleepState, 5);
        assert!(fn_00883240(&mut r.e, a));
        assert_eq!(r.e.get(a, Actor::cCurrentSitSleepState), 0);
        assert_eq!(
            r.order(),
            vec![PROCESS_LISTS_REMOVE_CLOSE_TO_PLAYER, PROCESS_LEVEL]
        );
    }

    #[test]
    fn fn_00883240_moves_a_high_actor_to_the_middle_low_process() {
        let (mut r, new) = transition_rig(MIDDLE_LOW_PROCESS_CONSTRUCTOR);
        let a = r.actor;
        assert!(fn_00883240(&mut r.e, a));
        assert_eq!(r.calls(OPERATOR_NEW), vec![vec![0xc8]]);
        assert_eq!(r.e.get(a, Actor::pCurrentProcess), Ptr::new(new));
        assert_eq!(r.calls(PROCESS_RESET), vec![vec![r.process]]);
        assert_eq!(
            r.calls(PROCESS_LISTS_REMOVE_REFERENCE),
            vec![vec![PROCESS_LISTS, a.addr(), 0]]
        );
        assert_eq!(
            r.calls(PROCESS_LISTS_ADD_REFERENCE),
            vec![vec![PROCESS_LISTS, a.addr(), 2, 1, 0, 0]]
        );
        assert_eq!(r.vcalls(PROCESS_VTABLE, 0x7a8), vec![vec![new, a.addr()]]);
        // No +0x4c(0x100000) at the end of this transition.
        assert!(r.vcalls(ACTOR_VTABLE, 0x4c).is_empty());
    }

    #[test]
    fn fn_00883240_reschedules_with_half_an_hour_of_lead() {
        let (mut r, new) = transition_rig(MIDDLE_LOW_PROCESS_CONSTRUCTOR);
        let a = r.actor;
        r.e.set_global::<u32>(SHUTDOWN_CHECK_OBJECT_POINTER, 0x1234);
        r.e.mem.set_f64(HALF_DOUBLE, 0.5);
        r.stub(CALENDAR_GET_HOUR, float_ret(10.0));
        r.vslot(ACTOR_VTABLE, 0x160, ret(1));
        r.e.mem.set_u8(a.addr() + 0x126, 1);
        // The idle package type 5 of the old process is stopped.
        r.vslot(PROCESS_VTABLE, 0x22c, ret(0x700));
        r.stub(PACKAGE_TYPE, ret(5));
        r.stub(IS_CONTINUING_PACKAGE_FOR_PC, ret(1));
        assert!(fn_00883240(&mut r.e, a));
        assert_eq!(r.vcalls(PROCESS_VTABLE, 0x284), vec![vec![r.process, 0]]);
        assert_eq!(r.calls(PROCESS_SET_TIME), vec![vec![new, 9.5f32.to_bits()]]);
        assert_eq!(
            r.calls(PROCESS_LISTS_ADD_REFERENCE).last(),
            Some(&vec![PROCESS_LISTS, a.addr(), 2, 0, 0, 0])
        );
        // The continuing package gives the actor to the new process twice.
        assert_eq!(r.vcalls(PROCESS_VTABLE, 0x24).len(), 2);
    }

    #[test]
    fn fn_00883800_stops_for_an_actor_that_is_middle_high_already() {
        let mut r = Rig::new();
        let a = r.actor;
        r.level(1);
        assert!(fn_00883800(&mut r.e, a));
        assert_eq!(
            r.order(),
            vec![PROCESS_LISTS_REMOVE_CLOSE_TO_PLAYER, PROCESS_LEVEL]
        );
    }

    #[test]
    fn fn_00883800_moves_a_high_actor_to_the_middle_high_process() {
        let (mut r, new) = transition_rig(MIDDLE_HIGH_PROCESS_CONSTRUCTOR);
        let a = r.actor;
        r.stub(GET_CONTAINER_CHANGES, ret(0x66));
        r.stub(GET_WORN_ITEM, ret(0x99));
        r.vslot(PROCESS_VTABLE, 0x31c, ret(1));
        assert!(fn_00883800(&mut r.e, a));
        // No process reset in this transition.
        assert!(r.calls(PROCESS_RESET).is_empty());
        assert_eq!(r.calls(OPERATOR_NEW), vec![vec![0x25c]]);
        assert_eq!(r.e.get(a, Actor::pCurrentProcess), Ptr::new(new));
        assert_eq!(
            r.calls(PROCESS_LISTS_ADD_REFERENCE),
            vec![vec![PROCESS_LISTS, a.addr(), 1, 1, 0, 0]]
        );
        assert_eq!(r.calls(ACTOR_SET_FLAG), vec![vec![a.addr(), 1]]);
        // The old level was 0: the worn item is refreshed in place.
        assert_eq!(r.calls(WORN_ITEM_UPDATE), vec![vec![0x99, 1]]);
        assert_eq!(r.vcalls(PROCESS_VTABLE, 0x7a8), vec![vec![new, a.addr()]]);
        // The mover is told as the actor leaves the high process.
        assert_eq!(r.calls(MOVER_UPDATE_B).len(), 1);
    }

    #[test]
    fn fn_00883800_hands_a_follower_to_the_enumeration() {
        let (mut r, new) = transition_rig(MIDDLE_HIGH_PROCESS_CONSTRUCTOR);
        let a = r.actor;
        let player = r.object(TARGET_VTABLE, 0x300);
        r.e.set_global::<u32>(PLAYER_POINTER, player);
        r.e.set_global::<u32>(SHUTDOWN_CHECK_OBJECT_POINTER, 0x1234);
        r.e.mem.set_f64(HALF_DOUBLE, 0.5);
        r.e.mem.set_f32(ENUM_RADIUS_FLOAT, 5000.0);
        r.stub(CALENDAR_GET_HOUR, float_ret(10.0));
        r.stub(CURRENT_PROCESS_TYPE, ret(1));
        // The actor follows the player: its process's target is the player.
        r.vslot(PROCESS_VTABLE, 0x27c, ret(0x700));
        r.stub(PACKAGE_TYPE, ret(1));
        r.vslot(PROCESS_VTABLE, 0x128, ret(player));
        r.vslot(TARGET_VTABLE, 0x100, ret(1));
        r.stub(ACTOR_TEST_008C0050, ret(1));
        let position = r.e.mem.alloc(12);
        r.vslot(ACTOR_VTABLE, 0x1f4, ret(position));
        r.stub(PARENT_CELL, ret(0xc0de));
        assert!(fn_00883800(&mut r.e, a));
        // Due: re-timed half an hour early and re-added at the process type.
        assert_eq!(r.calls(PROCESS_SET_TIME), vec![vec![new, 9.5f32.to_bits()]]);
        assert_eq!(
            r.calls(PROCESS_LISTS_ADD_REFERENCE).last(),
            Some(&vec![PROCESS_LISTS, a.addr(), 1, 0, 0, 0])
        );
        // The player's follower is not in a state to be skipped: the
        // enumeration runs around the actor.
        let radius = 5000.0f32.to_bits();
        assert_eq!(
            r.calls(ENUM_REFERENCES_CLOSE_TO_POINT),
            vec![vec![
                0x1234,
                0xc0de,
                position,
                radius,
                position,
                radius,
                ENUM_CALLBACK,
                a.addr()
            ]]
        );
        assert_eq!(r.e.mem.u8(a.addr() + 0x126), 1);
    }

    #[test]
    fn fn_00883800_ends_interrupt_packages_of_a_created_package() {
        let (mut r, _new) = transition_rig(MIDDLE_HIGH_PROCESS_CONSTRUCTOR);
        let a = r.actor;
        let player = r.object(ACTOR_VTABLE, 0x300);
        r.e.set_global::<u32>(PLAYER_POINTER, player);
        // The process's current package (+0x20c) was created at run time; the
        // package it runs has the two flags and the actor's cell holds it.
        r.vslot(PROCESS_VTABLE, 0x20c, ret(0x800));
        r.stub(PACKAGE_GET_IS_CREATED, ret(1));
        let package = r.e.mem.alloc(0x40);
        r.e.mem.set_u32(package + 0x1c, 0x201);
        r.vslot(PROCESS_VTABLE, 0x22c, ret(package));
        r.stub(PARENT_CELL, ret(0xc0de));
        r.stub(CELL_TEST_ACTOR, ret(1));
        assert!(fn_00883800(&mut r.e, a));
        assert_eq!(r.calls(CELL_TEST_ACTOR), vec![vec![0xc0de, a.addr()]]);
        assert!(!r.vcalls(ACTOR_VTABLE, 0x288).is_empty());
        // Without a package: an alarmed actor outside the player's interior ends it.
        let (mut r, _new) = transition_rig(MIDDLE_HIGH_PROCESS_CONSTRUCTOR);
        let a = r.actor;
        let player = r.object(ACTOR_VTABLE, 0x300);
        r.e.set_global::<u32>(PLAYER_POINTER, player);
        r.vslot(PROCESS_VTABLE, 0x20c, ret(0x800));
        r.stub(PACKAGE_GET_IS_CREATED, ret(1));
        r.stub(IS_ALARMED, ret(1));
        r.stub(PARENT_CELL, ret(0xc0de));
        r.stub(FLAG_BIT_0_AT_0X24, ret(0));
        assert!(fn_00883800(&mut r.e, a));
        assert!(!r.vcalls(ACTOR_VTABLE, 0x288).is_empty());
    }

    #[test]
    fn package_flag_tests_read_the_flag_word() {
        let mut e = Engine::new();
        let package = Ptr::new(e.mem.alloc(0x40));
        assert!(!fn_008840d0(&mut e, package));
        assert!(!fn_008840f0(&mut e, package));
        e.mem.set_u32(package.addr() + 0x1c, 1);
        assert!(fn_008840d0(&mut e, package));
        assert!(!fn_008840f0(&mut e, package));
        e.mem.set_u32(package.addr() + 0x1c, 0x200);
        assert!(!fn_008840d0(&mut e, package));
        assert!(fn_008840f0(&mut e, package));
    }

    /// An escort package (type 2) whose target is the player.
    fn escort_rig() -> (Rig, u32, u32) {
        let mut r = Rig::new();
        let player = r.object(TARGET_VTABLE, 0x300);
        r.e.set_global::<u32>(PLAYER_POINTER, player);
        r.vslot(PROCESS_VTABLE, 0x27c, ret(0x700));
        r.vslot(PROCESS_VTABLE, 0x128, ret(player));
        r.stub(PACKAGE_TYPE, ret(2));
        let own_position = r.e.mem.alloc(12);
        let player_position = r.e.mem.alloc(12);
        r.vslot(ACTOR_VTABLE, 0x1f4, ret(own_position));
        r.vslot(TARGET_VTABLE, 0x1f4, ret(player_position));
        let offset = r.e.mem.alloc(12);
        r.stub(POINT_SUBTRACT, ret(offset));
        r.stub_seq(POINT_LENGTH, vec![float_ret(10.0), float_ret(4.0)]);
        (r, player, offset)
    }

    #[test]
    fn is_escorter_behind_compares_the_two_offsets() {
        let (mut r, _player, _offset) = escort_rig();
        let a = r.actor;
        // The player's offset (4) is shorter than the actor's (10).
        assert!(actor_is_escorter_behind(&mut r.e, a));
        // The other way round it is not.
        r.stub_seq(POINT_LENGTH, vec![float_ret(3.0), float_ret(4.0)]);
        assert!(!actor_is_escorter_behind(&mut r.e, a));
        // Equal is not "behind" either.
        r.stub_seq(POINT_LENGTH, vec![float_ret(4.0), float_ret(4.0)]);
        assert!(!actor_is_escorter_behind(&mut r.e, a));
    }

    #[test]
    fn is_escorter_behind_needs_the_escort_package_and_the_player() {
        let (mut r, _player, _offset) = escort_rig();
        let a = r.actor;
        // Another package type.
        r.stub(PACKAGE_TYPE, ret(3));
        assert!(!actor_is_escorter_behind(&mut r.e, a));
        r.stub(PACKAGE_TYPE, ret(2));
        // Another target.
        r.vslot(PROCESS_VTABLE, 0x128, ret(0x1234));
        assert!(!actor_is_escorter_behind(&mut r.e, a));
        // No package.
        let (mut r, _player, _offset) = escort_rig();
        let a = r.actor;
        r.vslot(PROCESS_VTABLE, 0x27c, ret(0));
        assert!(!actor_is_escorter_behind(&mut r.e, a));
        // No process.
        r.e.set(a, Actor::pCurrentProcess, Ptr::new(0));
        assert!(!actor_is_escorter_behind(&mut r.e, a));
    }

    #[test]
    fn is_escorter_behind_gives_up_when_the_package_cell_is_not_the_players() {
        let (mut r, _player, _offset) = escort_rig();
        let a = r.actor;
        r.stub(PARENT_CELL, ret(0xc0de));
        r.stub(FLAG_BIT_0_AT_0X24, ret(1));
        r.stub(PACKAGE_LOCATION_CELL, ret(0xbeef));
        assert!(!actor_is_escorter_behind(&mut r.e, a));
        // The same cell: the length comparison decides.
        r.stub(PACKAGE_LOCATION_CELL, ret(0xc0de));
        assert!(actor_is_escorter_behind(&mut r.e, a));
    }

    #[test]
    fn is_following_needs_an_uncreated_following_package() {
        let mut r = Rig::new();
        let a = r.actor;
        r.vslot(PROCESS_VTABLE, 0x27c, ret(0x700));
        r.stub(PACKAGE_TYPE, ret(1));
        assert!(actor_is_following(&mut r.e, a));
        r.stub(PACKAGE_GET_IS_CREATED, ret(1));
        assert!(!actor_is_following(&mut r.e, a));
        r.stub(PACKAGE_GET_IS_CREATED, ret(0));
        r.stub(PACKAGE_TYPE, ret(2));
        assert!(!actor_is_following(&mut r.e, a));
        r.vslot(PROCESS_VTABLE, 0x27c, ret(0));
        r.stub(PACKAGE_TYPE, ret(1));
        assert!(!actor_is_following(&mut r.e, a));
        r.e.set(a, Actor::pCurrentProcess, Ptr::new(0));
        assert!(!actor_is_following(&mut r.e, a));
    }

    #[test]
    fn barter_gold_base_reads_the_base_data() {
        let mut r = Rig::new();
        let a = r.actor;
        r.stub(GET_BASE_FORM, ret(0x4000));
        r.stub(BARTER_GOLD, ret(0xabcd_0123));
        assert_eq!(actor_get_barter_gold_base(&mut r.e, a), 0x0123);
        assert_eq!(r.calls(BARTER_GOLD), vec![vec![0x4030]]);
    }

    #[test]
    fn get_class_needs_a_form_and_the_flag() {
        let mut r = Rig::new();
        let a = r.actor;
        r.stub(GET_FORM, ret(0x4000));
        r.stub(ACTOR_BASE_CLASS, ret(0x5000));
        assert_eq!(actor_get_class(&mut r.e, a), 0);
        r.vslot(ACTOR_VTABLE, 0x218, ret(1));
        assert_eq!(actor_get_class(&mut r.e, a), 0x5000);
        assert_eq!(r.calls(ACTOR_BASE_CLASS), vec![vec![0x4000]]);
        r.stub(GET_FORM, ret(0));
        assert_eq!(actor_get_class(&mut r.e, a), 0);
    }

    #[test]
    fn can_move_checks_every_blocker() {
        let mut r = Rig::new();
        let a = r.actor;
        let player = r.object(ACTOR_VTABLE, 0x300);
        r.e.set_global::<u32>(PLAYER_POINTER, player);
        assert!(actor_can_move(&mut r.e, a));
        for offset in [0x234, 0x22c, 0x230, 0x214] {
            r.vslot(ACTOR_VTABLE, offset, ret(1));
            assert!(!actor_can_move(&mut r.e, a), "slot {offset:#x}");
            r.vslot(ACTOR_VTABLE, offset, ret(0));
        }
        for action in [12, 13] {
            r.stub(GET_ANIM_ACTION, ret(action));
            assert!(!actor_can_move(&mut r.e, a), "action {action}");
        }
        for action in [11, 14] {
            r.stub(GET_ANIM_ACTION, ret(action));
            assert!(actor_can_move(&mut r.e, a), "action {action}");
        }
        r.stub(GET_ANIM_ACTION, ret(0));
        r.stub(ACTOR_BLOCK_TEST_A, ret(1));
        assert!(!actor_can_move(&mut r.e, a));
        r.stub(ACTOR_BLOCK_TEST_A, ret(0));
        r.stub(ACTOR_BLOCK_TEST_B, ret(1));
        assert!(!actor_can_move(&mut r.e, a));
        r.stub(ACTOR_BLOCK_TEST_B, ret(0));
        // The player with the Pip-Boy up cannot move.
        r.stub(IS_PIPBOY_ACTIVE, ret(1));
        assert!(actor_can_move(&mut r.e, a));
        r.e.set_global::<u32>(PLAYER_POINTER, a.addr());
        assert!(!actor_can_move(&mut r.e, a));
        assert_eq!(r.calls(IS_PIPBOY_ACTIVE).last(), Some(&vec![a.addr()]));
    }

    #[test]
    fn can_speak_checks_the_blockers() {
        let mut r = Rig::new();
        let a = r.actor;
        assert!(actor_can_speak(&mut r.e, a));
        for offset in [0x234, 0x22c, 0x230] {
            r.vslot(ACTOR_VTABLE, offset, ret(1));
            assert!(!actor_can_speak(&mut r.e, a), "slot {offset:#x}");
            r.vslot(ACTOR_VTABLE, offset, ret(0));
        }
        r.stub(ACTOR_BLOCK_TEST_B, ret(1));
        assert!(!actor_can_speak(&mut r.e, a));
        // The movement-only test does not matter for speech.
        r.stub(ACTOR_BLOCK_TEST_B, ret(0));
        r.stub(ACTOR_BLOCK_TEST_A, ret(1));
        assert!(actor_can_speak(&mut r.e, a));
    }

    #[test]
    fn fn_008844f0_accepts_states_one_two_and_maybe_six() {
        let mut r = Rig::new();
        let a = r.actor;
        for (state, strict, loose) in [
            (0, false, false),
            (1, true, true),
            (2, true, true),
            (6, false, true),
            (7, false, false),
        ] {
            r.stub(ACTOR_STATE_QUERY, ret(state));
            assert_eq!(fn_008844f0(&mut r.e, a, true), strict, "state {state}");
            assert_eq!(fn_008844f0(&mut r.e, a, false), loose, "state {state}");
        }
    }

    #[test]
    fn fn_00884560_asks_the_process() {
        let mut r = Rig::new();
        let a = r.actor;
        assert!(!fn_00884560(&mut r.e, a));
        r.vslot(PROCESS_VTABLE, 0x40c, ret(0x20));
        assert!(fn_00884560(&mut r.e, a));
        r.e.set(a, Actor::pCurrentProcess, Ptr::new(0));
        assert!(!fn_00884560(&mut r.e, a));
    }

    #[test]
    fn can_knock_down_checks_every_blocker() {
        let mut r = Rig::new();
        let a = r.actor;
        r.stub(GET_BASE_FORM, ret(0x4000));
        assert!(actor_can_knock_down(&mut r.e, a));
        assert_eq!(r.calls(BASE_FLAG_TEST), vec![vec![0x4030, 0x0400_0000]]);
        for offset in [0x22c, 0x4b4] {
            r.vslot(ACTOR_VTABLE, offset, ret(1));
            assert!(!actor_can_knock_down(&mut r.e, a), "slot {offset:#x}");
            r.vslot(ACTOR_VTABLE, offset, ret(0));
        }
        // The base data's flag forbids it.
        r.stub(BASE_FLAG_TEST, ret(1));
        assert!(!actor_can_knock_down(&mut r.e, a));
        r.stub(BASE_FLAG_TEST, ret(0));
        // The process's virtual +0x3e4 (9 and 0x11 forbid it).
        for state in [9, 0x11] {
            r.vslot(PROCESS_VTABLE, 0x3e4, ret(state));
            assert!(!actor_can_knock_down(&mut r.e, a), "state {state}");
        }
        r.vslot(PROCESS_VTABLE, 0x3e4, ret(5));
        assert!(actor_can_knock_down(&mut r.e, a));
        // The actor's +0x1e4 object with the u16 at +0x122 not 0xff forbids it.
        let object = r.e.mem.alloc(0x140);
        r.vslot(ACTOR_VTABLE, 0x1e4, ret(object));
        r.e.mem.set_u16(object + 0x122, 0xff);
        assert!(actor_can_knock_down(&mut r.e, a));
        r.e.mem.set_u16(object + 0x122, 0x10);
        assert!(!actor_can_knock_down(&mut r.e, a));
        // Without a process the process test is skipped.
        r.e.mem.set_u16(object + 0x122, 0xff);
        r.e.set(a, Actor::pCurrentProcess, Ptr::new(0));
        assert!(actor_can_knock_down(&mut r.e, a));
    }

    #[test]
    fn fn_00884690_negates_the_flag_test() {
        let mut r = Rig::new();
        assert!(fn_00884690(&mut r.e, Ptr::new(0x4030)));
        r.stub(BASE_FLAG_TEST, ret(1));
        assert!(!fn_00884690(&mut r.e, Ptr::new(0x4030)));
        assert_eq!(
            r.calls(BASE_FLAG_TEST).last(),
            Some(&vec![0x4030, 0x0400_0000])
        );
    }

    #[test]
    fn fn_008846c0_compares_the_halfword_with_0xff() {
        let mut e = Engine::new();
        let object = Ptr::new(e.mem.alloc(0x140));
        e.mem.set_u16(object.addr() + 0x122, 0xff);
        assert!(!fn_008846c0(&mut e, object));
        e.mem.set_u16(object.addr() + 0x122, 0x100);
        assert!(fn_008846c0(&mut e, object));
        e.mem.set_u16(object.addr() + 0x122, 0);
        assert!(fn_008846c0(&mut e, object));
    }

    #[test]
    fn fn_008846e0_and_is_running_read_the_mover() {
        let mut r = Rig::new();
        let a = r.actor;
        // Without a mover: zero.
        assert_eq!(fn_008846e0(&mut r.e, a), 0);
        assert!(!actor_is_running(&mut r.e, a));
        let mover = r.object(MOVER_VTABLE, 0x100);
        r.e.set(a, Actor::pActorMover, Ptr::new(mover));
        r.vslot(MOVER_VTABLE, 0x20, ret(0x0300));
        assert_eq!(fn_008846e0(&mut r.e, a), 0x300);
        assert!(actor_is_running(&mut r.e, a));
        r.vslot(MOVER_VTABLE, 0x20, ret(0x0100));
        assert!(!actor_is_running(&mut r.e, a));
    }

    #[test]
    fn fn_00884750_blocks_for_the_stub_the_action_and_the_player() {
        let mut r = Rig::new();
        let a = r.actor;
        let player = r.object(ACTOR_VTABLE, 0x300);
        r.e.set_global::<u32>(PLAYER_POINTER, player);
        assert!(fn_00884750(&mut r.e, a));
        r.stub(ALWAYS_FALSE, ret(1));
        assert!(!fn_00884750(&mut r.e, a));
        r.stub(ALWAYS_FALSE, ret(0));
        r.stub(GET_ANIM_ACTION, ret(7));
        assert!(!fn_00884750(&mut r.e, a));
        r.stub(GET_ANIM_ACTION, ret(0));
        // The process's virtual +0x404 only matters for the player.
        r.vslot(PROCESS_VTABLE, 0x404, ret(1));
        assert!(fn_00884750(&mut r.e, a));
        r.e.set_global::<u32>(PLAYER_POINTER, a.addr());
        assert!(!fn_00884750(&mut r.e, a));
        r.e.set(a, Actor::pCurrentProcess, Ptr::new(0));
        assert!(fn_00884750(&mut r.e, a));
    }

    #[test]
    fn fn_008847c0_reports_dead_legs_and_marks_the_process() {
        let mut r = Rig::new();
        let a = r.actor;
        let process_flags = r.e.mem.alloc(0x60);
        r.e.mem.set_u32(r.process + 0x2c, process_flags);
        r.e.mem.set_f64(ZERO_DOUBLE, 0.0);
        let this = Ptr::new(a.addr() + 0xa8);
        // Both leg conditions are zero: crippled, without asking the owner.
        r.stub(GET_BODY_PART_CONDITION, float_ret(0.0));
        assert!(fn_008847c0(&mut r.e, this));
        assert_eq!(
            r.calls(GET_BODY_PART_CONDITION),
            vec![
                vec![a.addr() + 0xa4, 0x1d, 0],
                vec![a.addr() + 0xa4, 0x1e, 0]
            ]
        );
        assert!(r.vcalls(OBJECT_VTABLE, 8).is_empty());
        assert_eq!(r.e.mem.u8(process_flags + 0x40), 1);
        assert_eq!(r.e.mem.u32(process_flags + 0x44), 0x4000);
        // One non-zero condition falls back to the owner's virtual +8 (0x48).
        r.stub_seq(
            GET_BODY_PART_CONDITION,
            vec![float_ret(0.0), float_ret(5.0)],
        );
        assert!(!fn_008847c0(&mut r.e, this));
        assert_eq!(
            r.vcalls(OBJECT_VTABLE, 8),
            vec![vec![a.addr() + 0xa4, 0x48]]
        );
        assert_eq!(r.e.mem.u8(process_flags + 0x40), 0);
        r.vslot(OBJECT_VTABLE, 8, ret(1));
        r.stub(GET_BODY_PART_CONDITION, float_ret(2.0));
        assert!(fn_008847c0(&mut r.e, this));
        assert_eq!(r.e.mem.u8(process_flags + 0x40), 1);
        // Without a process flag block nothing is written.
        r.e.mem.set_u32(r.process + 0x2c, 0);
        assert!(fn_008847c0(&mut r.e, this));
    }

    #[test]
    fn flag_block_helpers_set_and_clear_bits() {
        let mut e = Engine::new();
        let block = Ptr::new(e.mem.alloc(0x60));
        let holder = Ptr::new(e.mem.alloc(0x40));
        // 00884920: is there a block?
        assert!(!fn_00884920(&mut e, holder));
        e.mem.set_u32(holder.addr() + 0x2c, block.addr());
        assert!(fn_00884920(&mut e, holder));
        // 008848c0 sets, 00884970 clears.
        fn_008848c0(&mut e, block, 0x4001);
        assert_eq!(e.mem.u32(block.addr() + 0x44), 0x4001);
        fn_00884970(&mut e, block, 0x4000);
        assert_eq!(e.mem.u32(block.addr() + 0x44), 0x0001);
        // 00884880 stores the byte and sets 0x4000.
        fn_00884880(&mut e, holder, 7);
        assert_eq!(e.mem.u8(block.addr() + 0x40), 7);
        assert_eq!(e.mem.u32(block.addr() + 0x44), 0x4001);
        // 00884940 clears through the block.
        fn_00884940(&mut e, holder, 0x4001);
        assert_eq!(e.mem.u32(block.addr() + 0x44), 0);
        // Without a block, 00884880 and 00884940 do nothing.
        e.mem.set_u32(holder.addr() + 0x2c, 0);
        fn_00884880(&mut e, holder, 9);
        fn_00884940(&mut e, holder, 0xffff);
        assert_eq!(e.mem.u8(block.addr() + 0x40), 7);
    }

    #[test]
    fn fn_008848e0_clears_the_flag_of_the_process_block() {
        let mut r = Rig::new();
        let a = r.actor;
        let block = r.e.mem.alloc(0x60);
        r.e.mem.set_u32(block + 0x44, 0x4003);
        r.e.mem.set_u32(r.process + 0x2c, block);
        fn_008848e0(&mut r.e, a);
        assert_eq!(r.e.mem.u32(block + 0x44), 0x0003);
        // No block on the process, or no process: nothing changes.
        r.e.mem.set_u32(r.process + 0x2c, 0);
        fn_008848e0(&mut r.e, a);
        r.e.set(a, Actor::pCurrentProcess, Ptr::new(0));
        fn_008848e0(&mut r.e, a);
        assert_eq!(r.e.mem.u32(block + 0x44), 0x0003);
    }
}
