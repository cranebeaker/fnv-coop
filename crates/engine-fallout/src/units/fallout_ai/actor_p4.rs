//! `fallout/ai/actor.cpp` (Xbox PDB source unit), part 4: its functions from `008a50d0` up to
//! (not including) `008b00c0` in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The unit's shared layouts and helpers are in
//! [`super::actor`]; anything public there may be used here.
//!
//! This file holds all the functions of the range: the first 40 (`008a50d0` to
//! `008a7760`), the second 40 (`008a7870` to `008ac6f0`) and the last 40
//! (`008ac810` to `008aff80`); the range is complete.
//!
//! Notes that apply to the whole file:
//!
//! - `Actor` offsets are the Xbox PDB ones minus `0x10` (the PC `TESForm` is
//!   `0x18` bytes), checked against the code that uses them. `+0x68` is
//!   `MobileObject::pCurrentProcess`, the actor's process; many functions here
//!   forward to a virtual method of that process. `008d8520` (named
//!   `MiddleHighProcess::GetSavedAcquireObject` in the engine map, a folded
//!   body) returns `this + 0x68`, and the code sometimes calls it and
//!   sometimes reads the field.
//! - Virtual calls use the PC byte offsets taken from the code. Slot names are
//!   given only where the Xbox PDB and the body agree.
//! - x87 note: `float` results are rounded to `f32` where the game stores a
//!   `float`; intermediate values are computed in `f64`.
//! - C++ exception-unwinding frames (`FS:[0]` chains) and the stack
//!   protector (`__security_check_cookie`) are not translated.

#[allow(unused_imports)]
use super::actor::*;
#[allow(unused_imports)]
use crate::prelude::*;

/// `MiddleHighProcess::GetSavedAcquireObject` (engine map name of `008d8520`):
/// the word at `this + 0x68`, the actor's process.
const GET_PROCESS: u32 = 0x008d_8520;
/// The player character (`011dea3c` holds the pointer).
const PLAYER_CHARACTER: u32 = 0x011d_ea3c;
/// The calendar object (`Calendar`, passed as `this` by address).
const CALENDAR: u32 = 0x011d_e7b8;
/// `Calendar::GetHour` (Xbox PDB): the hour in `ST0`.
const CALENDAR_GET_HOUR: u32 = 0x0086_7da0;
/// `1.0` (`double`).
const ONE: u32 = 0x0101_2070;
/// `0.0` (`double`).
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// `-1.0` (`float`).
const MINUS_ONE: u32 = 0x0101_2054;
/// `5000.0` (`float`): the radius of the reference searches.
const SEARCH_RADIUS: u32 = 0x0103_0020;
/// `0.01745329238474369` (`double`): one degree in radians.
const ONE_DEGREE: u32 = 0x0102_3128;
/// `pi` and `-pi` (`float`): the range `BSWrap` brings an angle into.
const PI: u32 = 0x0102_b3c8;
const MINUS_PI: u32 = 0x0106_b648;
/// A `float` the face-animation reset passes (`0.75`).
const FACE_FADE: u32 = 0x0101_6264;
/// Three words (a zero `NiPoint3`) used when there is no weapon position.
const ZERO_POINT: u32 = 0x011f_426c;
/// The `TESDataHandler` pointer (`011c3f2c`), `this` of the reference search.
const DATA_HANDLER: u32 = 0x011c_3f2c;
/// The `ModelLoader` pointer (`011c3b3c`).
const MODEL_LOADER: u32 = 0x011c_3b3c;
/// The global that remembers the actor `008a5580` is speaking for.
const SPEAKING_ACTOR: u32 = 0x011d_f680;
/// The pointer to the name `"Weapon"` (`01188b78`) the node lookup takes.
const WEAPON_NODE_NAME: u32 = 0x0118_8b78;
/// A table of `0x24`-byte entries (`011977e0`), indexed by `005f2420`; the
/// first word is compared with 2 by `SetAnimAction`.
const ANIM_GROUP_TABLE: u32 = 0x0119_77e0;
/// A table of pointers to tables of words (`011a3ff0`).
const PACKAGE_TYPE_TABLE: u32 = 0x011a_3ff0;
/// The cache (`0126817c`) `008a5f20` looks a collision object up in.
const COLLISION_CACHE: u32 = 0x0126_817c;
/// The type descriptors `__RTDynamicCast` is given by `008a7760`.
const TYPE_TES_PACKAGE: u32 = 0x0118_46a0;
const TYPE_BACK_UP_PACKAGE: u32 = 0x011a_32e0;
/// Names of the two landing sounds.
const LAND_HEAVY_SOUND: u32 = 0x0108_4dac;
const LAND_LIGHT_SOUND: u32 = 0x0108_4d98;

/// `GameSetting` objects `Actor::Move` reads through `00403e20`.
const SETTING_FALL_THRESHOLD: u32 = 0x011c_ff94;
const SETTING_FALL_MULT: u32 = 0x011d_1410;
const SETTING_FALL_EXPONENT: u32 = 0x011d_0d20;
const SETTING_HEAVY_THRESHOLD: u32 = 0x011d_f6ec;
const SETTING_LIMB_DAMAGE_MULT: u32 = 0x011d_108c;
const SETTING_LIMB_DAMAGE_CHANCE: u32 = 0x011d_10c8;

/// Call targets used more than once.
const SETTING_VALUE_POINTER: u32 = 0x0040_3e20;
const SOUND_HANDLE_INIT: u32 = 0x0041_a250;
const SOUND_HANDLE_ASSIGN: u32 = 0x0041_8900;
const SOUND_HANDLE_DESTRUCT: u32 = 0x0048_3710;
const FORM_TYPE: u32 = 0x0041_ca90;
const FORM_ID: u32 = 0x0084_e3a0;
const GET_PARENT_CELL: u32 = 0x008d_6f30;
const GET_ANIMATION: u32 = 0x008b_70d0;
const GET_FACE_ANIMATION_DATA: u32 = 0x008a_dcb0;
const GET_CURRENT_PACKAGE: u32 = 0x0093_44a0;
const GET_CURRENT_WEAPON: u32 = 0x008a_1710;
const ENUM_REFERENCES_CLOSE_TO_POINT: u32 = 0x0046_f280;
const IS_ALIVE_BODY: u32 = 0x0049_3bb0;
const GET_SAVE_FORM: u32 = 0x007a_f430;
const HOUR_TO_PROCESS: u32 = 0x0069_3d50;
const STOP_MOVING: u32 = 0x008b_3ab0;
const OPERATOR_NEW: u32 = 0x0040_1000;
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const INTERFACE_IN_DIALOG: u32 = 0x0070_50d0;
const SCRIPT_FLAG: u32 = 0x005b_7470;
const PACKAGE_CALLBACK_A: u32 = 0x0090_d570;
const PACKAGE_CALLBACK_B: u32 = 0x0090_d480;

layout! {
    /// `Actor` (Xbox PDB): the fields this file reads, at their PC offsets
    /// (Xbox PDB offset minus `0x10`). The PC size of the class is not known
    /// here; the declared size only has to cover the fields.
    pub struct Actor: 0x1b4 {
        /// `MobileObject::pCurrentProcess` (Xbox PDB `+0x78`): `BaseProcess*`.
        0x68 pCurrentProcess: Ptr,
        /// `MobileObject::bSpeakingDone` (Xbox PDB `+0x8c`).
        0x7c bSpeakingDone: u8,
        /// `MobileObject::bTalkingToPlayer` (Xbox PDB `+0x8d`).
        0x7d bTalkingToPlayer: u8,
        /// `MobileObject::bSoundFileDone` (Xbox PDB `+0x8f`).
        0x7f bSoundFileDone: u8,
        /// `MobileObject::bVoiceFileDone` (Xbox PDB `+0x90`).
        0x80 bVoiceFileDone: u8,
        /// `MobileObject::bUseEmotion` (Xbox PDB `+0x96`).
        0x86 bUseEmotion: u8,
        /// `Actor::pRagdollController` (Xbox PDB `+0xbc`).
        0xac pRagdollController: Ptr,
        /// `Actor::ePersuasionEmotion` (Xbox PDB `+0xc4`).
        0xb4 ePersuasionEmotion: u32,
        /// `Actor::bEVPBuffered` (Xbox PDB `+0x155`).
        0x145 bEVPBuffered: u8,
        /// `Actor::bResetAI` (Xbox PDB `+0x156`).
        0x146 bResetAI: u8,
        /// `Actor::pActorMover` (Xbox PDB `+0x1a0`).
        0x190 pActorMover: Ptr,
        /// `Actor::eCriticalStage` (Xbox PDB `+0x11c`).
        0x10c eCriticalStage: u32,
    }
}

/// `TESObjectREFR::pLoadedData` (Xbox PDB `+0x74`) at its PC offset.
const LOADED_DATA: u32 = 0x64;

/// The actor's process (`this + 0x68`).
fn process(e: &Engine, this: Ptr<Actor>) -> Ptr {
    e.get(this, Actor::pCurrentProcess)
}

/// `008d8520` as a call.
fn acquire_object(e: &mut Engine, this: Ptr<Actor>) -> Ptr {
    e.call(GET_PROCESS, &args![this]).ptr()
}

/// `Calendar::GetHour - 1.0`, as a `float`.
fn hour_before(e: &mut Engine) -> f32 {
    let hour = e.call(CALENDAR_GET_HOUR, &args![CALENDAR]).f32();
    let one: f64 = e.global(ONE);
    (hour as f64 - one) as f32
}

/// The value `00403e20` finds in a `GameSetting` object (a `float`).
fn setting_float(e: &mut Engine, setting: u32) -> f32 {
    let value = e.call(SETTING_VALUE_POINTER, &args![setting]).u32();
    e.mem.f32(value)
}

/// The value `0043d4d0` finds in a `GameSetting` object (an integer).
fn setting_int(e: &mut Engine, setting: u32) -> u32 {
    let value = e.call(0x0043_d4d0, &args![setting]).u32();
    e.mem.u32(value)
}

/// Copies `count` words.
fn copy_words(e: &mut Engine, to: u32, from: u32, count: u32) {
    for i in 0..count {
        let word = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, word);
    }
}

/// The reference search around the actor: `TESDataHandler::
/// EnumReferencesCloseToPoint` (`0046f280`) with the actor's cell, the
/// search radius (twice), two results of the actor's virtual method `+0x1f4`
/// and a callback that is given the actor.
fn enum_references_around(e: &mut Engine, this: Ptr<Actor>, callback: u32) {
    let radius: f32 = e.global(SEARCH_RADIUS);
    let handler: u32 = e.global(DATA_HANDLER);
    let first = e.vcall(this.addr(), 0x1f4, &args![]).u32();
    let second = e.vcall(this.addr(), 0x1f4, &args![]).u32();
    let cell = e.call(GET_PARENT_CELL, &args![this]).u32();
    e.call(
        ENUM_REFERENCES_CLOSE_TO_POINT,
        &args![handler, cell, second, radius, first, radius, callback, this],
    );
}

// Translated from 008a50d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a `float` in word 3 of `this` (`00560d30` is the address of word
/// `n` of the object).
pub fn fn_008a50d0(e: &mut Engine, this: Ptr, value: f32) {
    let word: Ptr = e.call(0x0056_0d30, &args![this, 3u32]).ptr();
    e.mem.set_f32(word.addr(), value);
}

// Translated from 008a50f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards to `008a5110`.
pub fn fn_008a50f0(e: &mut Engine, this: Ptr, source: Ptr) {
    fn_008a5110(e, this, source)
}

// Translated from 008a5110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the first three floats (x, y, z) of the 16-byte vector at `source`
/// into the 16-byte vector at `this`; the fourth float of `this` stays. The
/// code does it with SSE (`UNPCKHPS`, `SHUFPS`) on aligned loads.
pub fn fn_008a5110(e: &mut Engine, this: Ptr, source: Ptr) {
    copy_words(e, this.addr(), source.addr(), 3);
}

// Translated from 008a5170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 0 of the word at `this + 4` is set (`00621270` ands that word
/// with its argument, here `1`).
pub fn fn_008a5170(e: &mut Engine, this: Ptr) -> bool {
    e.call(0x0062_1270, &args![this, 1u32]).u32() != 0
}

// Translated from 008a5190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the global word at `011c6254`.
pub fn fn_008a5190(e: &mut Engine) -> u32 {
    e.global(0x011c_6254)
}

// Translated from 008a51a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The result of the process's virtual method `+0x77c`, 0 without a process.
pub fn fn_008a51a0(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    if acquire_object(e, this).is_null() {
        return 0;
    }
    let proc = acquire_object(e, this);
    e.vcall(proc.addr(), 0x77c, &args![]).u32()
}

// Translated from 008a51f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls the process's virtual method `+0x780` with `value`, if there is a
/// process.
pub fn fn_008a51f0(e: &mut Engine, this: Ptr<Actor>, value: u32) {
    if acquire_object(e, this).is_null() {
        return;
    }
    let proc = acquire_object(e, this);
    e.vcall(proc.addr(), 0x780, &args![value]);
}

// Translated from 008a5230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls the process's virtual method `+0x784`, if there is a process.
pub fn fn_008a5230(e: &mut Engine, this: Ptr<Actor>) {
    if acquire_object(e, this).is_null() {
        return;
    }
    let proc = acquire_object(e, this);
    e.vcall(proc.addr(), 0x784, &args![]);
}

// Translated from 008a5270 (decompiled, FalloutNV.exe 1.4.0.525)
/// The result of the process's virtual method `+0x770`, 0 without a process.
pub fn fn_008a5270(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    if acquire_object(e, this).is_null() {
        return 0;
    }
    let proc = acquire_object(e, this);
    e.vcall(proc.addr(), 0x770, &args![]).u32()
}

// Translated from 008a52c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls the process's virtual method `+0x774` with `value`, if there is a
/// process.
pub fn fn_008a52c0(e: &mut Engine, this: Ptr<Actor>, value: u32) {
    if acquire_object(e, this).is_null() {
        return;
    }
    let proc = acquire_object(e, this);
    e.vcall(proc.addr(), 0x774, &args![value]);
}

// Translated from 008a5300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls the process's virtual method `+0x778`, if there is a process.
pub fn fn_008a5300(e: &mut Engine, this: Ptr<Actor>) {
    if acquire_object(e, this).is_null() {
        return;
    }
    let proc = acquire_object(e, this);
    e.vcall(proc.addr(), 0x778, &args![]);
}

// Translated from 008a5340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetWeaponPosition` (Xbox PDB): writes into `result` the point
/// `(0, reach, 0)` transformed by the world transform of the actor's weapon
/// node, where `reach` is `CombatFormulas::CalcWeaponReach` of the current
/// weapon (0 without one). When `keep_world` is zero the transform is first
/// multiplied by the inverse of the actor's 3D root transform. `result` is
/// set to the zero point `011f426c` when the actor has no 3D or no weapon
/// node. Returns `result`.
///
/// The weapon node is the process's method `+0x190` of the current biped
/// (`+0x1e8`), or, without a process, the `"Weapon"` node found by
/// `004aae30` under the 3D root (the weapon query before it already reads the
/// process's vtable, so that branch is only reached with a process in the
/// game). Transforms are 13 words (`NiTransform`).
pub fn actor_get_weapon_position(
    e: &mut Engine,
    this: Ptr<Actor>,
    result: Ptr,
    keep_world: u8,
) -> Ptr {
    let root = e.vcall(this.addr(), 0x1d0, &args![]).u32();
    if root == 0 {
        copy_words(e, result.addr(), ZERO_POINT, 3);
        return result;
    }
    let proc = process(e, this);
    let weapon = e.vcall(proc.addr(), 0x148, &args![]).u32();
    let reach = if weapon != 0 {
        let form = e.call(0x0044_ddc0, &args![weapon]).u32();
        let base = e.call(0x0064_47f0, &args![form]).f32();
        e.call(0x0064_63e0, &args![base]).f32()
    } else {
        0.0f32
    };
    let proc = process(e, this);
    let node = if !proc.is_null() {
        let biped = e.vcall(this.addr(), 0x1e8, &args![]).u32();
        e.vcall(proc.addr(), 0x190, &args![biped]).u32()
    } else {
        let name: u32 = e.global(WEAPON_NODE_NAME);
        e.call(0x004a_ae30, &args![root, name]).u32()
    };
    if node == 0 {
        copy_words(e, result.addr(), ZERO_POINT, 3);
        return result;
    }
    // One block holds the working values:
    // 0x00 transform, 0x34 inverse, 0x68 product, 0x9c point, 0xa8 output.
    e.with_stack(0xb8, |e, block| {
        let transform = block.addr();
        let inverse = block.addr() + 0x34;
        let product = block.addr() + 0x68;
        let point = block.addr() + 0x9c;
        let output = block.addr() + 0xa8;
        let node_transform = e.call(0x0046_1130, &args![node]).u32();
        copy_words(e, transform, node_transform, 13);
        if keep_world == 0 {
            e.call(0x0047_6a80, &args![inverse]);
            let root_transform = e.call(0x0046_1130, &args![root]).u32();
            e.call(0x004b_4880, &args![root_transform, inverse]);
            let combined = e
                .call(0x0062_c250, &args![inverse, product, transform])
                .u32();
            copy_words(e, transform, combined, 13);
        }
        let made = e
            .call(0x0041_6870, &args![point, 0.0f32, reach, 0.0f32])
            .u32();
        e.call(0x0052_4c40, &args![transform, output, made]);
        copy_words(e, result.addr(), output, 3);
    });
    result
}

// Translated from 008a5530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls the actor's face animation data's virtual method `+0xb4` with
/// `(fade, 0, 1, 1, 1, 0)`, where `fade` is the `float` at `01016264`, if the
/// actor has face animation data (`Actor::GetFaceAnimationData`).
pub fn fn_008a5530(e: &mut Engine, this: Ptr<Actor>) {
    let face = e.call(GET_FACE_ANIMATION_DATA, &args![this]).u32();
    if face == 0 {
        return;
    }
    let fade: f32 = e.global(FACE_FADE);
    e.vcall(face, 0xb4, &args![fade, 0u32, 1u32, 1u32, 1u32, 0u32]);
}

/// The face animation data's mood call `+0x11c` shared by the branches of
/// `008a5580`: when the process does not say it is busy (method `+0x2e8`)
/// and the interface is in a dialogue, asks the actor's `ActorValueOwner`
/// subobject (`this + 0xa4`, method `+8`, `GetActorValue` in the Xbox PDB)
/// for value 4 and hands the answer to the face data.
fn face_mood_from_dialogue(e: &mut Engine, this: Ptr<Actor>, face: u32) {
    let busy = if !acquire_object(e, this).is_null() {
        let proc = acquire_object(e, this);
        e.vcall(proc.addr(), 0x2e8, &args![]).u32() != 0
    } else {
        false
    };
    if busy || !e.call(INTERFACE_IN_DIALOG, &args![]).bool() {
        return;
    }
    // 008a5acf..008a5af9: the 0 is pushed first and stays for the face call; GetActorValue
    // (Actor's ActorValueOwner slot +8, 00880580, RET 4) takes only the 4. The face call gets
    // (mood, 0): with one word the face data's 0064b400 (RET 8) stopped the game in Doc's
    // dialogue (the maintainer's couch report).
    let owner = this.addr() + 0xa4;
    let value = e.vcall(owner, 8, &args![4u32]).u32();
    e.vcall(face, 0x11c, &args![value, 0u32]);
}

/// The face animation data's reset call `+0xb4` with `(0.0, 1, 0, 0, 0, 0)`.
fn face_reset(e: &mut Engine, face: u32) {
    e.vcall(face, 0xb4, &args![0.0f32, 1u32, 0u32, 0u32, 0u32, 0u32]);
}

// Translated from 008a5580 (decompiled, FalloutNV.exe 1.4.0.525)
/// The actor's per-frame speech and movement update (the map has no name for
/// it). `delta` is the frame time, `speaker` the reference the actor speaks
/// for (it may be null) and `notify` says whether completion callbacks are
/// registered for the actor.
///
/// - Does nothing when the actor has no animation (virtual method `+0x1e4`).
/// - With a `speaker`, a process and `bSpeakingDone` set: records the actor in
///   `011df680`, sets `bTalkingToPlayer`, and either queues the speaker
///   form's sound (`00933150` and a completion callback, clearing
///   `bSoundFileDone` and `bSpeakingDone`) or, when the speech file is done,
///   starts the speech (virtual method `+0x284`, 13 words), registers the
///   completion callbacks and passes the speech length to the process
///   (`+0x318`).
/// - For an idle actor with a mover that is not rotating, requests a rotation
///   to the player's position (`ActorMover::RequestRotateActor`) when the
///   wrapped angle between the direction of the two positions and the
///   actor's rotation is more than one degree; then runs the move update
///   `+0x348` with the mover's `009c9900` flag raised around it.
/// - Updates the face animation data from the emotion state
///   (`ePersuasionEmotion`: 1 and 5 hand mood 6 and 5 to the face data; 8
///   either asks the actor value owner for the mood or resets the face,
///   depending on `005b7470`, the process and the interface).
/// - Keeps the special-idle package (type `0x1c`) pointing at the actor
///   while it speaks and restarts the idle (`+0x44` of the process) once the
///   idle is done playing.
pub fn fn_008a5580(e: &mut Engine, this: Ptr<Actor>, delta: f32, speaker: Ptr, notify: u8) {
    let animation = e.vcall(this.addr(), 0x1e4, &args![]).u32();
    let mut special_idle = Ptr::NULL;
    let proc = acquire_object(e, this);
    let package: Ptr = e.vcall(proc.addr(), 0x22c, &args![]).ptr();
    if !package.is_null() && e.call(FORM_TYPE, &args![package]).i32() == 0x1c {
        special_idle = package;
    }
    if animation == 0 {
        return;
    }
    // Two sound handles (12 bytes each) and a point.
    let scratch = e.mem.alloc(0x40);
    let handle = scratch;
    let queued = scratch + 0x10;
    let sound_state = scratch + 0x20;
    let mut speech_claimed = false;
    if !speaker.is_null()
        && !acquire_object(e, this).is_null()
        && e.get(this, Actor::bSpeakingDone) != 0
    {
        e.call(SOUND_HANDLE_INIT, &args![handle]);
        speech_claimed = true;
        e.set_global(SPEAKING_ACTOR, this.addr());
        e.set(this, Actor::bTalkingToPlayer, 1);
        let mut speaking = true;
        if !e.call(GET_SAVE_FORM, &args![speaker]).ptr::<()>().is_null() {
            let form = e.call(GET_SAVE_FORM, &args![speaker]).u32();
            let form_id = e.call(FORM_ID, &args![form]).u32();
            e.call(
                0x0093_3150,
                &args![this, queued, form_id, 0u32, 0x102u32, 1u32],
            );
            let manager = e.call(0x00ad_9060, &args![]).u32();
            let own_id = e.call(FORM_ID, &args![this]).u32();
            let sound = e.call(0x0055_9450, &args![queued]).u32();
            e.call(0x00ad_bfd0, &args![manager, sound, 0x0093_5cc0u32, own_id]);
            e.set(this, Actor::bSoundFileDone, 0);
            speaking = false;
            e.set(this, Actor::bSpeakingDone, 0);
            e.call(SOUND_HANDLE_DESTRUCT, &args![queued]);
        }
        let flag = e.call(0x0054_3c30, &args![speaker]).u8();
        fn_008a5cf0(e, this, flag);
        if speaking && e.get(this, Actor::bSoundFileDone) != 0 {
            let player: u32 = e.global(PLAYER_CHARACTER);
            let worldspace = e.call(0x0044_1110, &args![speaker]).u32();
            let owner = e.call(0x0096_11e0, &args![speaker]).u32();
            let owner_form = e.call(0x0068_15c0, &args![speaker]).u32();
            let owner_form = e.call(0x0040_48e0, &args![owner_form]).u32();
            let speaker_id = e.call(FORM_ID, &args![speaker]).u32();
            let base = e.call(0x0044_ddc0, &args![speaker]).u32();
            let name_field = e.call(0x0046_0140, &args![speaker]).u32();
            let name = e.call(0x0055_9450, &args![name_field]).u32();
            let length = e
                .vcall(
                    this.addr(),
                    0x284,
                    &args![
                        name, handle, base, speaker_id, owner_form, owner, worldspace, player,
                        1u32, 1u32, 0u32, 1u32, 1u32
                    ],
                )
                .f32();
            let manager = e.call(0x00ad_9060, &args![]).u32();
            let target = if notify != 0 { this } else { Ptr::NULL };
            if !target.is_null() {
                let id = e.call(FORM_ID, &args![target]).u32();
                let sound = e.call(0x0055_9450, &args![handle]).u32();
                e.call(0x00ad_c050, &args![manager, sound, 0x0093_5f00u32, id]);
                let id = e.call(FORM_ID, &args![target]).u32();
                let sound = e.call(0x0055_9450, &args![handle]).u32();
                e.call(0x00ad_bfd0, &args![manager, sound, 0x008b_c590u32, id]);
            }
            e.set(this, Actor::bVoiceFileDone, 0);
            let proc = acquire_object(e, this);
            e.vcall(proc.addr(), 0x318, &args![length]);
        }
        e.call(SOUND_HANDLE_DESTRUCT, &args![handle]);
    }

    let ragdoll = e.get(this, Actor::pRagdollController);
    if !ragdoll.is_null() {
        e.call(0x00c7_48b0, &args![ragdoll]);
    }

    let mover = e.get(this, Actor::pActorMover);
    if !mover.is_null()
        && e.vcall(this.addr(), 0x214, &args![]).u32() == 0
        && !e.call(IS_ALIVE_BODY, &args![this]).bool()
    {
        if !e.call(0x009d_db50, &args![mover]).bool() {
            // The direction from the player to the actor, as an angle.
            let own_position = e.vcall(this.addr(), 0x1f4, &args![]).u32();
            let player: u32 = e.global(PLAYER_CHARACTER);
            let player_position = e.vcall(player, 0x1f4, &args![]).u32();
            let offset = scratch + 0x30;
            e.call(0x0043_9ef0, &args![player_position, offset, own_position]);
            let direction = e.call(0x004b_13c0, &args![offset]).f32();
            let rotation = e.call(0x0043_0830, &args![this]).u32();
            let angle = (direction as f64 - e.mem.f32(rotation + 8) as f64) as f32;
            // The wrapped angle is kept in the last word of the scratch block.
            let wrapped = scratch + 0x3c;
            e.mem.set_f32(wrapped, angle);
            let low: f32 = e.global(MINUS_PI);
            let high: f32 = e.global(PI);
            e.call(0x004e_44f0, &args![wrapped, low, high]);
            let wrapped_angle = e.mem.f32(wrapped);
            let size = e.call(0x0040_8840, &args![wrapped_angle]).f64();
            let one_degree: f64 = e.global(ONE_DEGREE);
            if size > one_degree {
                let player_position = e.vcall(player, 0x1f4, &args![]).u32();
                let x = e.mem.u32(player_position);
                let y = e.mem.u32(player_position + 4);
                let z = e.mem.u32(player_position + 8);
                e.call(0x009d_ce20, &args![mover, x, y, z, 0u32]);
            }
        }
        e.call(0x009c_9900, &args![mover, 1u32]);
        e.vcall(this.addr(), 0x348, &args![delta, 0u32]);
        e.call(0x009c_9900, &args![mover, 0u32]);
    }

    e.vcall(this.addr(), 0x140, &args![]);

    if !acquire_object(e, this).is_null() {
        let proc = acquire_object(e, this);
        if e.call(0x0045_cd60, &args![proc]).u32() == 0 {
            let proc = acquire_object(e, this);
            let bone = e.call(0x0089_d620, &args![proc]).u32();
            if bone != 0 && e.call(0x005f_36f0, &args![bone]).u32() != 0 {
                e.call(0x00c5_2960, &args![bone, 0u32]);
            }
        }
    }

    if !acquire_object(e, this).is_null() {
        let proc = acquire_object(e, this);
        e.vcall(proc.addr(), 0x48c, &args![sound_state, 0u32]);
        let face = e.call(GET_FACE_ANIMATION_DATA, &args![this]).u32();
        let playing = e.call(0x00ad_8930, &args![sound_state]).bool();
        if !playing {
            if !special_idle.is_null() && e.mem.u32(special_idle.addr() + 0x94) != 0 {
                e.mem.set_u32(special_idle.addr() + 0x94, 0);
            }
            if face != 0 {
                let mood = e.get(this, Actor::ePersuasionEmotion);
                if mood == 8 {
                    if e.call(SCRIPT_FLAG, &args![]).bool() {
                        face_mood_from_dialogue(e, this, face);
                    } else {
                        face_reset(e, face);
                    }
                } else if mood == 1 {
                    e.vcall(face, 0x11c, &args![6u32, 0u32]);
                } else if mood == 5 {
                    e.vcall(face, 0x11c, &args![5u32, 0u32]);
                }
            }
        } else if face != 0 && e.get(this, Actor::ePersuasionEmotion) == 8 {
            let face = e.call(GET_FACE_ANIMATION_DATA, &args![this]).u32();
            if face != 0 {
                if e.call(SCRIPT_FLAG, &args![]).bool() {
                    face_mood_from_dialogue(e, this, face);
                } else {
                    face_reset(e, face);
                }
            }
        }
        e.call(SOUND_HANDLE_DESTRUCT, &args![sound_state]);
    }

    if speech_claimed && !special_idle.is_null() {
        e.mem.set_u32(special_idle.addr() + 0x94, this.addr());
    }
    if !special_idle.is_null() && e.call(0x0049_85f0, &args![animation]).bool() {
        let proc = acquire_object(e, this);
        if e.vcall(proc.addr(), 0x718, &args![]).u32() == 0 {
            let proc = acquire_object(e, this);
            e.vcall(
                proc.addr(),
                0x44,
                &args![this, 0u32, 2u32, 1u32, 0u32, 1u32],
            );
        }
    }
    e.mem.free(scratch);
}

// Translated from 008a5cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the byte in `bUseEmotion` (`+0x86`).
pub fn fn_008a5cf0(e: &mut Engine, this: Ptr<Actor>, value: u8) {
    e.set(this, Actor::bUseEmotion, value);
}

// Translated from 008a5d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// A per-frame update of the actor with frame time `delta`: relaxes the bone
/// level of detail of an idle process, runs the virtual methods `+0x268`,
/// `+0x350`, `+0x2f8` and `+0x178`, and updates the animation's queued scene
/// graph and the cell.
pub fn fn_008a5d10(e: &mut Engine, this: Ptr<Actor>, delta: f32) {
    let proc = acquire_object(e, this);
    let package = e.vcall(proc.addr(), 0x22c, &args![]).u32();
    if package != 0 {
        // The result (a package of type 0x1c) is not used afterwards.
        e.call(FORM_TYPE, &args![package]);
    }
    let animation = e.vcall(this.addr(), 0x1e4, &args![]).u32();
    if !proc.is_null() && e.call(0x0045_cd60, &args![proc]).u32() == 0 {
        let bone = e.call(0x0089_d620, &args![proc]).u32();
        if bone != 0 && e.call(0x005f_36f0, &args![bone]).u32() != 0 {
            e.call(0x00c5_2960, &args![bone, 0u32]);
        }
    }
    e.vcall(this.addr(), 0x268, &args![delta]);
    e.vcall(this.addr(), 0x350, &args![]);
    if !e.get(this, Actor::pRagdollController).is_null() {
        e.call(0x0088_8970, &args![this, 0u32]);
    }
    if animation != 0 {
        e.call(0x0049_3930, &args![animation]);
    }
    e.vcall(this.addr(), 0x2f8, &args![delta]);
    let cell = e.call(GET_PARENT_CELL, &args![this]).u32();
    if cell != 0 {
        e.call(0x0054_baf0, &args![cell]);
    }
    e.vcall(this.addr(), 0x178, &args![]);
}

// Translated from 008a5e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::SetAlert` (Xbox PDB): the process's virtual method `+0x320` with
/// `alert`, if there is a process.
pub fn actor_set_alert(e: &mut Engine, this: Ptr<Actor>, alert: u8) {
    let proc = process(e, this);
    if proc.is_null() {
        return;
    }
    e.vcall(proc.addr(), 0x320, &args![alert]);
}

// Translated from 008a5e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetAlert` (Xbox PDB): the process's virtual method `+0x31c`,
/// false without a process.
pub fn actor_get_alert(e: &mut Engine, this: Ptr<Actor>) -> bool {
    let proc = process(e, this);
    if proc.is_null() {
        return false;
    }
    e.vcall(proc.addr(), 0x31c, &args![]).bool()
}

// Translated from 008a5eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::SetHavokWeapon` (Xbox PDB): with a current weapon
/// (`Actor::GetCurrentWeapon`), calls `SetHavokWeapon_ov2` with 0 when the
/// ragdoll controller exists and `00552490` says so, else 1; returns its
/// result, 0 without a weapon.
pub fn actor_set_havok_weapon(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    if e.call(GET_CURRENT_WEAPON, &args![this]).u32() == 0 {
        return 0;
    }
    let ragdoll = e.get(this, Actor::pRagdollController);
    let flag: u8 = if !ragdoll.is_null() && e.call(0x0055_2490, &args![ragdoll]).bool() {
        0
    } else {
        1
    };
    actor_set_havok_weapon_ov2(e, this, flag)
}

// Translated from 008a5f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::SetHavokWeapon_ov2` (Xbox PDB): makes the physics weapon object of
/// the actor's current weapon, or updates it. `mode` is the byte the caller
/// derived (see [`actor_set_havok_weapon`]).
///
/// While the special state `008c7aa0` holds, only the task queue is told
/// (`TaskQueueInterface::QueueActorSetHavokWeapon`). Otherwise: takes the
/// actor's 3D (`00950bb0` for the player, `0043fcd0` for others), finds the
/// weapon node (`004ade00`), looks for an existing collision object
/// (`004b5260`, `00653270` in the cache `0126817c`), and either updates its
/// constraints (`00c89050`) or creates the weapon object (`00c89540`);
/// finally updates the child rigid bodies (`00c891c0`) when the actor is in
/// a state that needs it. Always returns 0.
pub fn actor_set_havok_weapon_ov2(e: &mut Engine, this: Ptr<Actor>, mode: u8) -> u32 {
    if e.call(0x008c_7aa0, &args![]).bool() {
        let queue = e.call(0x0045_37b0, &args![]).u32();
        e.call(0x0087_b6b0, &args![queue, this, mode]);
        return 0;
    }
    let player: u32 = e.global(PLAYER_CHARACTER);
    let root = if this.addr() == player {
        e.call(0x0095_0bb0, &args![player, 0u32]).u32()
    } else {
        e.call(0x0043_fcd0, &args![this]).u32()
    };
    if root == 0 || e.call(GET_CURRENT_WEAPON, &args![this]).u32() == 0 {
        return 0;
    }
    let name = e.call(0x0089_1350, &args![]).u32();
    let node = e.call(0x004a_de00, &args![root, name]).u32();
    if node == 0 {
        return 0;
    }
    let in_combat_use = (mode == 0
        && e.vcall(this.addr(), 0x22c, &args![0u32]).u8() == 0
        && e.vcall(this.addr(), 0x230, &args![]).u8() == 0)
        || {
            let ragdoll = e.get(this, Actor::pRagdollController);
            !ragdoll.is_null() && e.call(0x0089_d690, &args![ragdoll]).u8() != 0
        };
    let keep: u8 = in_combat_use as u8;
    let link = e.mem.alloc(4);
    e.call(0x0093_1ed0, &args![this, link]);
    let link_value = e.mem.u32(link);
    let collision = e.call(0x004b_5260, &args![node]).u32();
    let found = if collision != 0 {
        e.call(0x0065_3270, &args![COLLISION_CACHE, collision])
            .u32()
    } else {
        0
    };
    let mut weapon_object = found;
    // 008a608d / 008a60d3: `WeaponObject::UpdateConstraints` (`00c89050`, RET 0x10)
    // does not read the ECX (the weapon object) the caller sets; its four words are
    // the pushed ones.
    if weapon_object != 0 {
        e.call(0x00c8_9050, &args![mode, link_value, root, keep]);
    } else {
        weapon_object = e
            .call(0x00c8_9540, &args![link_value, root, node, keep])
            .u32();
        if weapon_object != 0 && mode == 0 {
            e.call(0x00c8_9050, &args![mode, link_value, root, keep]);
        }
    }
    e.mem.free(link);
    let needs_children = e.call(0x0043_7bd0, &args![this]).bool()
        || e.vcall(this.addr(), 0x22c, &args![0u32]).u8() != 0
        || e.vcall(this.addr(), 0x2e8, &args![]).u8() != 0
        || e.vcall(this.addr(), 0x230, &args![]).u8() != 0;
    if weapon_object != 0 && needs_children {
        e.call(0x00c8_91c0, &args![weapon_object]);
    }
    0
}

// Translated from 008a6170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the actor's current package (`MobileObject::GetCurrentPackage`)
/// has type `0x18`.
pub fn fn_008a6170(e: &mut Engine, this: Ptr<Actor>) -> bool {
    let package = e.call(GET_CURRENT_PACKAGE, &args![this]).u32();
    package != 0 && e.call(FORM_TYPE, &args![package]).i32() == 0x18
}

// Translated from 008a61b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::IsAlarmed` (Xbox PDB): whether the process's current package
/// (virtual method `+0x22c`) has type `0x15`.
pub fn actor_is_alarmed(e: &mut Engine, this: Ptr<Actor>) -> bool {
    let proc = process(e, this);
    let package = if proc.is_null() {
        0
    } else {
        let proc = process(e, this);
        e.vcall(proc.addr(), 0x22c, &args![]).u32()
    };
    package != 0 && e.call(FORM_TYPE, &args![package]).i32() == 0x15
}

// Translated from 008a6210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the current package qualifies: it has a valid index
/// (`009611e0`, not `-1`) and either `fn_008a6290` holds for the package or
/// the entry of table `011a3ff0` for that index and the process's method
/// `+0x280` is `0xd`.
pub fn fn_008a6210(e: &mut Engine, this: Ptr<Actor>) -> bool {
    let package = e.call(GET_CURRENT_PACKAGE, &args![this]).u32();
    if package == 0 || e.call(0x0096_11e0, &args![package]).i32() == -1 {
        return false;
    }
    let table_index = e.call(0x0096_11e0, &args![package]).u32();
    let proc = acquire_object(e, this);
    let key = e.vcall(proc.addr(), 0x280, &args![]).u32();
    if fn_008a6290(e, Ptr::new(package)) {
        return true;
    }
    let table = e.mem.u32(PACKAGE_TYPE_TABLE + table_index.wrapping_mul(4));
    e.mem.u32(table.wrapping_add(key.wrapping_mul(4))) == 0xd
}

// Translated from 008a6290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit `0x10000` of the word at `this + 0x1c` is set.
pub fn fn_008a6290(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0x1c) & 0x10000 != 0
}

// Translated from 008a62b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::Move` (Xbox PDB): moves the actor with `MobileObject::Move`
/// (`0092f260`, arguments `delta`, `arg_b`, `flags`) and, when that returned a
/// character controller that is on the ground, applies fall damage.
///
/// - An actor in dialogue with the player does not move unless the low six
///   bits of `flags` are set; one without loaded data does not move.
/// - The fall distance of the controller (`bhkCharacterController::
///   GetFallDistance`) beyond the setting at `011cff94` gives the damage
///   `pow(distance - threshold, exponent) * multiplier` (settings
///   `011d0d20`, `011d1410`), which is 0 for a base form `005f0c40` flags.
/// - Positive damage calls the virtual method `+0x338` when the actor is
///   not (`+0x360`) in the loaded state and the world space check allows
///   it, plays a landing sound (heavy above the setting `011df6ec`), and with
///   the chance of settings `011d10c8` damages the two leg values (`+0x3ac`
///   with `0x1d` and `0x1e`); then `Actor::TriggerPain(1, 1)`.
///
/// Returns what `MobileObject::Move` returned (0 when nothing moved).
pub fn actor_move(e: &mut Engine, this: Ptr<Actor>, delta: f32, arg_b: u32, flags: u32) -> Ptr {
    if e.call(0x0093_3840, &args![this]).bool() && flags & 0x3f == 0 {
        return Ptr::NULL;
    }
    if !process(e, this).is_null() {
        // The character controller; the result is not used afterwards.
        e.call(0x0093_06d0, &args![this]);
    }
    if e.mem.u32(this.addr() + LOADED_DATA) == 0 {
        return Ptr::NULL;
    }
    let moved: Ptr = e.call(0x0092_f260, &args![this, delta, arg_b, flags]).ptr();
    if moved.is_null() || !e.call(0x0087_cee0, &args![moved]).bool() {
        return moved;
    }
    let fall = e.call(0x00c7_0550, &args![moved]).f32();
    let limit = setting_float(e, SETTING_FALL_THRESHOLD);
    // `fall > limit`, false when either is not a number.
    if fall.partial_cmp(&limit) != Some(std::cmp::Ordering::Greater) {
        return moved;
    }
    let threshold = setting_float(e, SETTING_FALL_THRESHOLD);
    // `FSUB dword` (`008a6392`), at the main thread's precision.
    let excess = x87_sub(x87_load32(fall.to_bits()), x87_m32(threshold.to_bits())) as f32;
    let multiplier_at = e
        .call(SETTING_VALUE_POINTER, &args![SETTING_FALL_MULT])
        .u32();
    let exponent = setting_float(e, SETTING_FALL_EXPONENT);
    let scaled = e.call(0x004d_d130, &args![excess, exponent]).f64();
    // `FMUL dword` (`008a63c2`) on the power `004dd130` left in ST0.
    let mut damage = x87_mul(scaled, x87_m32(e.mem.u32(multiplier_at))) as f32;
    let form = e.call(0x0041_81e0, &args![this]).u32();
    if form != 0 && e.call(0x005f_0c40, &args![form]).bool() {
        damage = 0.0;
    }
    let zero: f64 = e.global(ZERO_DOUBLE);
    if (damage as f64).partial_cmp(&zero) != Some(std::cmp::Ordering::Greater) {
        return moved;
    }

    let mut hurt = true;
    if e.vcall(this.addr(), 0x360, &args![]).u8() == 0
        && e.vcall(this.addr(), 0x218, &args![]).u8() != 0
        && e.call(0x0057_5d70, &args![this]).u32() != 0
    {
        let world = e.call(0x0057_5d70, &args![this]).u32();
        if !e.call(0x0058_6320, &args![world]).bool() {
            hurt = false;
        }
    }
    if hurt
        && (e.vcall(this.addr(), 0x360, &args![]).u8() != 0
            || !e.call(0x0056_6950, &args![this]).bool())
    {
        e.vcall(this.addr(), 0x338, &args![damage, 0.0f32, 0u32]);
    }

    if e.vcall(this.addr(), 0x360, &args![]).u8() != 0 {
        let sound = e.mem.alloc(0x30);
        let named = sound + 0x10;
        e.call(SOUND_HANDLE_INIT, &args![sound]);
        let heavy = damage > setting_float(e, SETTING_HEAVY_THRESHOLD);
        let name = if heavy {
            LAND_HEAVY_SOUND
        } else {
            LAND_LIGHT_SOUND
        };
        let audio = e.call(0x0045_3a70, &args![]).u32();
        let found = e
            .call(0x00ad_7550, &args![audio, named, name, 0x101u32])
            .u32();
        e.call(SOUND_HANDLE_ASSIGN, &args![sound, found]);
        e.call(SOUND_HANDLE_DESTRUCT, &args![named]);
        e.call(0x00ad_8830, &args![sound, 0u32]);
        fn_008a6630(e, moved);
        let limb_damage =
            // `FMUL dword` (`008a6576`).
            x87_mul(
                x87_load32(damage.to_bits()),
                x87_m32(setting_float(e, SETTING_LIMB_DAMAGE_MULT).to_bits()),
            ) as f32;
        for value in [0x1du32, 0x1e] {
            let roll = e.call(0x0048_7f50, &args![]).u32() % 100;
            if roll < setting_int(e, SETTING_LIMB_DAMAGE_CHANCE) {
                e.vcall(this.addr(), 0x3ac, &args![value, -limb_damage, 0u32]);
            }
        }
        e.call(SOUND_HANDLE_DESTRUCT, &args![sound]);
        e.mem.free(sound);
    }
    e.call(0x008a_7d50, &args![this, 1u32, 1u32]);
    moved
}

// Translated from 008a6630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `006296b0` on the sub-object at `this + 0x410` with `0x80` (a flag
/// of the character controller).
pub fn fn_008a6630(e: &mut Engine, this: Ptr) {
    e.call(0x0062_96b0, &args![this.addr() + 0x410, 0x80u32]);
}

// Translated from 008a6650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::IsFleeing` (Xbox PDB): whether the actor's current package is a
/// flee package (type `0x16`, then for the player the process's method
/// `+0x234` runs and the answer is false), or, when `skip_checks` is 0, a
/// package of type `0xa`, or of type `0x13` while the process's method
/// `+0x280` is 1, or the actor's combat controller (virtual method `+0x428`)
/// is fleeing (`CombatController::IsFleeing`).
pub fn actor_is_fleeing(e: &mut Engine, this: Ptr<Actor>, skip_checks: u8) -> bool {
    let package = e.call(GET_CURRENT_PACKAGE, &args![this]).u32();
    if package == 0 {
        return false;
    }
    if e.call(FORM_TYPE, &args![package]).i32() == 0x16 {
        let player: u32 = e.global(PLAYER_CHARACTER);
        if player == this.addr() {
            let proc = e.call(GET_PROCESS, &args![player]).u32();
            e.vcall(proc, 0x234, &args![]);
            return false;
        }
        return true;
    }
    if skip_checks != 0 {
        return false;
    }
    if e.call(FORM_TYPE, &args![package]).i32() == 0x16 {
        return true;
    }
    if e.call(FORM_TYPE, &args![package]).i32() == 0xa {
        return true;
    }
    if e.call(FORM_TYPE, &args![package]).i32() == 0x13 {
        let proc = process(e, this);
        if e.vcall(proc.addr(), 0x280, &args![]).i32() == 1 {
            return true;
        }
    }
    let controller = e.vcall(this.addr(), 0x428, &args![]).u32();
    if controller == 0 {
        return false;
    }
    e.call(0x0098_1990, &args![controller]).bool()
}

// Translated from 008a6730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::RemoveFleeTarget` (Xbox PDB): when the current package is a flee
/// package (type `0x16`) that `fn_008a67d0` does not yet flag, removes
/// `target` from it (`009f1350`); if that makes `fn_008a67d0` true the actor
/// stops: its virtual method `+0x434` when `00493bb0` holds, else
/// `Actor::EndInterruptPackage(0)`.
pub fn actor_remove_flee_target(e: &mut Engine, this: Ptr<Actor>, target: u32) {
    let mut flee = Ptr::NULL;
    let package = e.call(GET_CURRENT_PACKAGE, &args![this]).u32();
    if package != 0 && e.call(FORM_TYPE, &args![package]).i32() == 0x16 {
        flee = e.call(GET_CURRENT_PACKAGE, &args![this]).ptr();
    }
    if flee.is_null() || fn_008a67d0(e, flee) {
        return;
    }
    e.call(0x009f_1350, &args![flee, target]);
    if !fn_008a67d0(e, flee) {
        return;
    }
    if e.call(IS_ALIVE_BODY, &args![this]).bool() {
        e.vcall(this.addr(), 0x434, &args![0u32]);
    } else {
        e.call(0x0088_1680, &args![this, 0u32]);
    }
}

// Translated from 008a67d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `008256d0` on the sub-object at `this + 0x98` (here a flee package's
/// list of targets).
pub fn fn_008a67d0(e: &mut Engine, this: Ptr) -> bool {
    e.call(0x0082_56d0, &args![this.addr() + 0x98]).bool()
}

// Translated from 008a67f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::IsTalking` (Xbox PDB): `bTalkingToPlayer`, else the process's
/// virtual method `+0x4b8` with the actor (false without a process).
pub fn actor_is_talking(e: &mut Engine, this: Ptr<Actor>) -> bool {
    if e.get(this, Actor::bTalkingToPlayer) != 0 {
        return true;
    }
    let proc = process(e, this);
    if proc.is_null() {
        return false;
    }
    let proc = process(e, this);
    e.vcall(proc.addr(), 0x4b8, &args![this]).bool()
}

// Translated from 008a6840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets a boolean state of the actor's process, probably the weapon-out state
/// (`fn_008a6970` reads it, process method `+0x450` writes it). Nothing
/// happens when
/// `drawn` is set and `008846e0` has flag `0x800`, for the player while the
/// iron sights are up, without a process, or when `drawn` equals
/// `fn_008a6970`. Otherwise: virtual method `+0x340` with `0xd`, the
/// process's method `+0x450` with `drawn`, and, for another actor than the
/// player whose process method `+0x148` returned 0, loads the animation
/// group `0x18` if it is missing (`ModelLoader::BuildKFFileList` and
/// `Animation::AddAnimationsFromList`).
pub fn fn_008a6840(e: &mut Engine, this: Ptr<Actor>, drawn: u8) {
    if drawn != 0 && e.call(0x0088_46e0, &args![this]).u32() & 0x800 != 0 {
        return;
    }
    let player: u32 = e.global(PLAYER_CHARACTER);
    if this.addr() == player && e.call(0x008b_bc10, &args![this]).bool() {
        return;
    }
    let proc = acquire_object(e, this);
    if proc.is_null() {
        return;
    }
    let current = fn_008a6970(e, this) as u8;
    if drawn == current {
        return;
    }
    e.vcall(this.addr(), 0x340, &args![0xdu32]);
    e.vcall(proc.addr(), 0x450, &args![drawn]);
    if this.addr() == player {
        return;
    }
    if e.vcall(proc.addr(), 0x148, &args![]).u32() != 0 || drawn == 0 {
        return;
    }
    let animation = e.call(GET_ANIMATION, &args![this]).u32();
    if animation == 0 {
        return;
    }
    let group = e.call(0x005f_2370, &args![0u32, 1u32, 0x18u32, 0u32]).u16();
    if e.call(0x0049_4710, &args![animation, group as u32]).bool() {
        return;
    }
    // 008a6933: `005715d0` takes no stack word; the pushed 1, 0, 1 are the
    // last three words of `00447330` (RET 0x10).
    let model = e.call(0x0057_15d0, &args![this]).u32();
    let loader: u32 = e.global(MODEL_LOADER);
    let list = e
        .call(0x0044_7330, &args![loader, model, 1u32, 0u32, 1u32])
        .u32();
    e.call(0x0049_0400, &args![animation, list]);
}

// Translated from 008a6970 (decompiled, FalloutNV.exe 1.4.0.525)
/// With `008846e0` flag `0x800`: for the player, quits the VATS playback when
/// the VATS mode is 4, and returns false; otherwise the process's virtual
/// method `+0x44c` (this assumes a process).
pub fn fn_008a6970(e: &mut Engine, this: Ptr<Actor>) -> bool {
    if e.call(0x0088_46e0, &args![this]).u32() & 0x800 != 0 {
        let player: u32 = e.global(PLAYER_CHARACTER);
        if this.addr() == player && e.call(0x0044_ddc0, &args![0x011f_2250u32]).u32() == 4 {
            e.call(0x009c_8950, &args![0x011f_2250u32, 0u32, 0u32]);
        }
        return false;
    }
    let proc = process(e, this);
    e.vcall(proc.addr(), 0x44c, &args![]).bool()
}

// Translated from 008a69d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::IsContinuingPackageforPC` (Xbox PDB): the process's virtual
/// method `+0x4e0`, false without a process.
pub fn actor_is_continuing_package_for_pc(e: &mut Engine, this: Ptr<Actor>) -> bool {
    let proc = process(e, this);
    if proc.is_null() {
        return false;
    }
    let proc = process(e, this);
    e.vcall(proc.addr(), 0x4e0, &args![]).bool()
}

// Translated from 008a6a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Turns the actor to `heading` (`MobileObject::SetHeading`) when `004f8960`
/// of the actor is 0 or 4.
pub fn fn_008a6a00(e: &mut Engine, this: Ptr<Actor>, heading: f32) {
    if e.call(0x004f_8960, &args![this]).u32() != 0 && e.call(0x004f_8960, &args![this]).u32() != 4
    {
        return;
    }
    e.call(0x0093_1b60, &args![this, heading]);
}

// Translated from 008a6a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::UnlockLockDoorsProcedure` (Xbox PDB): for an actor with a process
/// whose current package (process method `+0x22c`) is not an interrupt
/// package and which is in a cell, runs the package's door reference search
/// (callback `0090d480` when `005394a0` holds, else `0090d570` when
/// `00670ed0` does), then asks the package (virtual method `+0x13c` with the
/// actor, 0, `-1.0` and 0) and, when that succeeds and the process method
/// `+0x1bc` is false, searches again (`0090d480` when `00670f40` holds, and
/// the process method `+0x1c0` with 1; else `0090d570` when `00670f60`).
pub fn actor_unlock_lock_doors_procedure(e: &mut Engine, this: Ptr<Actor>) {
    if process(e, this).is_null() || e.vcall(this.addr(), 0x22c, &args![0u32]).u8() != 0 {
        return;
    }
    let global_object: u32 = e.global(0x011d_df38);
    if e.call(0x0042_ce10, &args![global_object]).bool() {
        return;
    }
    let proc = process(e, this);
    let package = e.vcall(proc.addr(), 0x22c, &args![]).u32();
    if package == 0
        || e.call(GET_PARENT_CELL, &args![this]).u32() == 0
        || e.call(0x0067_8610, &args![package]).bool()
    {
        return;
    }
    if e.call(0x0053_94a0, &args![package]).bool() {
        enum_references_around(e, this, PACKAGE_CALLBACK_B);
    } else if e.call(0x0067_0ed0, &args![package]).bool() {
        enum_references_around(e, this, PACKAGE_CALLBACK_A);
    }
    let minus_one: f32 = e.global(MINUS_ONE);
    let accepted = e
        .vcall(package, 0x13c, &args![this, 0u32, minus_one, 0u32])
        .bool();
    if !accepted {
        return;
    }
    let proc = acquire_object(e, this);
    if e.vcall(proc.addr(), 0x1bc, &args![]).bool() {
        return;
    }
    if e.call(0x0067_0f40, &args![package]).bool() {
        enum_references_around(e, this, PACKAGE_CALLBACK_B);
        let proc = acquire_object(e, this);
        e.vcall(proc.addr(), 0x1c0, &args![1u32]);
    } else if e.call(0x0067_0f60, &args![package]).bool() {
        enum_references_around(e, this, PACKAGE_CALLBACK_A);
    }
}

// Translated from 008a6ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::EvaluatePackage` (Xbox PDB): re-evaluates which package the actor
/// runs. With `now` zero the evaluation is only buffered (`bEVPBuffered` set,
/// `bResetAI` = `reset_ai`) and the process is told the hour before now.
/// With `now` set and `reset_ai` zero the actor stops moving and the process
/// re-starts its package (`+0x524`, `+0x118`, `+0x24`) when the actor has no
/// package change pending (method `+0x214` is 0, 4 or 9). With both set the
/// whole change is made: reference searches for the current package, removal
/// as a follower, `EndInterruptPackage`, leaving furniture, the process's
/// package switch (`+0x524`, `+0x4ec`, `+0x24`), the editor package's script
/// action flag, and the cleanup that depends on method `+0x214`
/// (`+0x418`, or `SetChaseBip`, `SetAnimAction` and clearing the animation
/// groups). Returns nothing; the two flags are cleared at the end.
pub fn actor_evaluate_package(e: &mut Engine, this: Ptr<Actor>, now: u8, reset_ai: u8) {
    if e.vcall(this.addr(), 0x22c, &args![1u32]).u8() != 0 {
        return;
    }
    if now == 0 {
        e.set(this, Actor::bEVPBuffered, 1);
        e.set(this, Actor::bResetAI, reset_ai);
        let proc = process(e, this);
        if !proc.is_null() {
            let hour = hour_before(e);
            let proc = process(e, this);
            e.call(HOUR_TO_PROCESS, &args![proc, hour]);
        }
        return;
    }
    if e.call(IS_ALIVE_BODY, &args![this]).bool() {
        e.vcall(this.addr(), 0x434, &args![0u32]);
    }
    if reset_ai == 0 {
        if e.vcall(this.addr(), 0x214, &args![]).u32() != 0
            && e.vcall(this.addr(), 0x214, &args![]).u32() != 4
            && e.vcall(this.addr(), 0x214, &args![]).u32() != 9
        {
            return;
        }
        e.call(STOP_MOVING, &args![this]);
        let proc = process(e, this);
        if !proc.is_null() {
            let proc = process(e, this);
            e.vcall(proc.addr(), 0x524, &args![this]);
            let proc = process(e, this);
            e.vcall(proc.addr(), 0x118, &args![1u32]);
            let proc = process(e, this);
            e.vcall(proc.addr(), 0x24, &args![this, 1u32]);
            let proc = process(e, this);
            if e.call(0x0045_cd60, &args![proc]).u32() != 0 {
                let hour = hour_before(e);
                let proc = process(e, this);
                e.call(HOUR_TO_PROCESS, &args![proc, hour]);
            }
        }
        e.set(this, Actor::bEVPBuffered, 0);
        e.set(this, Actor::bResetAI, 0);
        return;
    }

    if !acquire_object(e, this).is_null() {
        let proc = acquire_object(e, this);
        // The result is not used afterwards.
        e.vcall(proc.addr(), 0x20c, &args![]);
        let proc = acquire_object(e, this);
        if e.vcall(proc.addr(), 0x22c, &args![]).u32() != 0 {
            let proc = acquire_object(e, this);
            let package = e.vcall(proc.addr(), 0x22c, &args![]).u32();
            if e.call(0x0067_0f10, &args![package]).bool() {
                enum_references_around(e, this, PACKAGE_CALLBACK_A);
            }
            if e.call(0x0067_0ef0, &args![package]).bool() {
                enum_references_around(e, this, PACKAGE_CALLBACK_B);
            }
        }
        let proc = acquire_object(e, this);
        if e.vcall(proc.addr(), 0x360, &args![]).u8() != 0 {
            let mut follower = 0u32;
            let proc = acquire_object(e, this);
            if e.vcall(proc.addr(), 0x128, &args![]).u32() != 0 {
                let proc = acquire_object(e, this);
                let candidate = e.vcall(proc.addr(), 0x128, &args![]).u32();
                if e.vcall(candidate, 0x100, &args![]).u8() != 0 {
                    let proc = acquire_object(e, this);
                    follower = e.vcall(proc.addr(), 0x128, &args![]).u32();
                }
            }
            if follower != 0 {
                let list = e.call(0x005d_43c0, &args![follower]).u32();
                e.call(0x0042_2690, &args![list, this]);
            }
        }
        let editor_package = e.call(0x0088_14b0, &args![this]).ptr::<()>();
        e.call(0x0088_1680, &args![this, 0u32]);
        let proc = acquire_object(e, this);
        let old_package = e.vcall(proc.addr(), 0x22c, &args![]).ptr::<()>();
        let proc = acquire_object(e, this);
        if e.vcall(proc.addr(), 0x22c, &args![]).u32() != 0 {
            let proc = acquire_object(e, this);
            e.vcall(proc.addr(), 0x234, &args![]);
        }
        e.call(0x0088_d640, &args![this]);
        let proc = acquire_object(e, this);
        e.vcall(proc.addr(), 0x524, &args![this]);
        let proc = acquire_object(e, this);
        e.vcall(proc.addr(), 0x4ec, &args![0u32]);
        let proc = acquire_object(e, this);
        e.vcall(proc.addr(), 0x24, &args![this, 1u32]);
        let proc = acquire_object(e, this);
        let new_package = e.vcall(proc.addr(), 0x22c, &args![]).ptr::<()>();
        if !editor_package.is_null() && new_package != editor_package {
            let list = e.call(0x005d_43c0, &args![this]).u32();
            e.call(0x005a_c750, &args![editor_package, list, 0x800u32]);
            let proc = acquire_object(e, this);
            e.vcall(proc.addr(), 0x59c, &args![this, editor_package]);
            let proc = acquire_object(e, this);
            e.vcall(proc.addr(), 0x5a8, &args![0u32]);
        }
        if !old_package.is_null()
            && (e.call(FORM_TYPE, &args![old_package]).i32() == 4
                || e.call(0x0044_1b00, &args![old_package]).bool())
        {
            // The form's type (0x2a or 0x2b) is looked at but nothing is done
            // with it.
            let form = e.call(GET_SAVE_FORM, &args![this]).u32();
            e.call(0x0040_1170, &args![form]);
        }
        e.call(0x0087_faa0, &args![this]);
        let kind = e.vcall(this.addr(), 0x214, &args![]).u32();
        if kind != 0 {
            let mut finish = true;
            if kind == 4 {
                let proc = acquire_object(e, this);
                if e.vcall(proc.addr(), 0x22c, &args![]).u32() == 0 {
                    finish = false;
                }
            } else if kind != 9 {
                finish = false;
                e.call(0x0093_1fb0, &args![this, 0u32]);
                let proc = process(e, this);
                e.vcall(proc.addr(), 0x4c0, &args![this, 0u32, 0u32, 0x7fu32]);
                actor_set_anim_action(e, this, -1, 0);
                let animation = e.call(GET_ANIMATION, &args![this]).u32();
                if animation != 0 {
                    e.call(0x0049_6080, &args![animation, 0x14u32, 0.0f32]);
                    e.call(0x0049_8910, &args![animation, 1u32, 0u32]);
                }
            }
            if finish {
                e.vcall(this.addr(), 0x418, &args![]);
            }
        }
    }

    let proc = process(e, this);
    if !proc.is_null() {
        let proc = process(e, this);
        if e.call(0x0045_cd60, &args![proc]).u32() != 0 {
            let hour = hour_before(e);
            let proc = process(e, this);
            e.call(HOUR_TO_PROCESS, &args![proc, hour]);
        }
    }
    let proc = process(e, this);
    let package = if proc.is_null() {
        0
    } else {
        let proc = process(e, this);
        e.vcall(proc.addr(), 0x22c, &args![]).u32()
    };
    if package != 0 && e.call(0x0067_1d10, &args![package]).u32() != 0 {
        let count = e.call(0x0067_2930, &args![package]).u32();
        let proc = process(e, this);
        e.vcall(proc.addr(), 0x15c, &args![count]);
    }
    e.set(this, Actor::bEVPBuffered, 0);
    e.set(this, Actor::bResetAI, 0);
}

// Translated from 008a73e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::SetAnimAction` (Xbox PDB): sets the actor's animation action
/// (`action`; `sequence` is an animation sequence or 0). Before the process
/// is told (virtual method `+0x3ec`), clears the animation's upper-body
/// group 2 (`Animation::ClearGroup`, for the player also its first-person
/// animation) when it plays an animation group of type `0xaa`, unless the
/// current action is 4, the action is 7, the sequence's group entry in table
/// `011977e0` is 2, or the action is a weapon one (2, 4 to 6) that the weapon
/// form (`006450c0`) does not allow.
pub fn actor_set_anim_action(e: &mut Engine, this: Ptr<Actor>, action: i32, sequence: u32) {
    let animation = e.call(GET_ANIMATION, &args![this]).u32();
    let current = actor_get_anim_action(e, this);
    'clear: {
        if current == 4
            || animation == 0
            || e.call(0x0049_1040, &args![animation, 2u32]).u32() == 0
            || action == 7
        {
            break 'clear;
        }
        if sequence != 0 {
            let global = e.call(0x0048_f7f0, &args![sequence]).u32();
            let index = e.call(0x005f_2420, &args![global]).u32();
            if e.mem.u32(ANIM_GROUP_TABLE + index.wrapping_mul(0x24)) == 2 {
                break 'clear;
            }
        }
        let proc = process(e, this);
        let mut allowed = true;
        let weapon = e.vcall(proc.addr(), 0x148, &args![]).u32();
        let weapon_form = if weapon != 0 {
            let proc = process(e, this);
            let weapon = e.vcall(proc.addr(), 0x148, &args![]).u32();
            e.call(0x0044_ddc0, &args![weapon]).u32()
        } else {
            0
        };
        if (action == 2 || (action > 3 && action <= 6))
            && weapon_form != 0
            && !e.call(0x0064_50c0, &args![weapon_form]).bool()
        {
            allowed = false;
        }
        if allowed {
            let group = e.call(0x0043_01b0, &args![animation, 2u32]).u16();
            let group_type = e.call(0x005f_2440, &args![group as u32]).i32();
            if group_type == 0xaa {
                e.call(0x0049_6080, &args![animation, 2u32, 0.0f32]);
                let player: u32 = e.global(PLAYER_CHARACTER);
                if this.addr() == player {
                    let first_person = e.call(0x0095_0a60, &args![player, 1u32]).u32();
                    e.call(0x0049_6080, &args![first_person, 2u32, 0.0f32]);
                }
            }
        }
    }
    let proc = process(e, this);
    if !proc.is_null() {
        let proc = process(e, this);
        e.vcall(proc.addr(), 0x3ec, &args![action, sequence]);
    }
}

// Translated from 008a7570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetAnimAction` (Xbox PDB): the process's virtual method `+0x3e4`,
/// `-1` without a process.
pub fn actor_get_anim_action(e: &mut Engine, this: Ptr<Actor>) -> i32 {
    let proc = process(e, this);
    if proc.is_null() {
        return -1;
    }
    let proc = process(e, this);
    e.vcall(proc.addr(), 0x3e4, &args![]).i32()
}

/// The package changes shared by `008a75a0` and `008a7760`: nothing happens
/// when `00437bf0` or `00437bd0` hold or the current package already has
/// type `0x1a`; returns the process on which the new package is installed.
fn package_change_allowed(e: &mut Engine, this: Ptr<Actor>) -> bool {
    let proc = acquire_object(e, this);
    let package = e.vcall(proc.addr(), 0x27c, &args![]).u32();
    if e.call(0x0043_7bf0, &args![this]).bool() || e.call(0x0043_7bd0, &args![this]).bool() {
        return false;
    }
    !(package != 0 && e.call(FORM_TYPE, &args![package]).i32() == 0x1a)
}

// Translated from 008a75a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Makes the actor run a freshly created package of type `0x1a` located at
/// the actor itself: unless the change is not allowed, or the process method
/// `+0x4d0` returns `0x7f` (then the process method `+0x4c0` is called with
/// the actor, 0, 0 and `0x7f` instead), it creates the package
/// (`TESPackage::CreatePackage`, `SetPackType`), gives it a
/// `PackageLocation` referring to the actor, sets its data and installs it
/// with the virtual method `+0x2f4`.
pub fn fn_008a75a0(e: &mut Engine, this: Ptr<Actor>) {
    if !package_change_allowed(e, this) {
        return;
    }
    let proc = process(e, this);
    if e.vcall(proc.addr(), 0x4d0, &args![]).i32() == 0x7f {
        let proc = process(e, this);
        e.vcall(proc.addr(), 0x4c0, &args![this, 0u32, 0u32, 0x7fu32]);
        return;
    }
    let package = e.call(0x0067_0b90, &args![0x1au32]).u32();
    e.call(0x0067_0fc0, &args![package, 0x1au32]);
    e.call(0x0082_6b40, &args![package, 0u32]);
    e.call(0x0082_6b90, &args![package, 1u32]);
    let memory = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
    let location = if memory != 0 {
        e.call(0x0067_f030, &args![memory]).u32()
    } else {
        0
    };
    e.call(0x0067_f3c0, &args![location, this]);
    e.call(0x0067_1d30, &args![package, location]);
    if location != 0 {
        e.call(0x0067_0b30, &args![location, 1u32]);
    }
    e.call(0x0098_4f60, &args![package, 0x15u32]);
    let proc = acquire_object(e, this);
    e.vcall(proc.addr(), 0x28, &args![]);
    e.vcall(this.addr(), 0x2f4, &args![package, 1u32, 1u32]);
}

// Translated from 008a7760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Makes the actor run a freshly created package of type `0x27` (cast to the
/// back-up package type and initialized by `BackUpPackage::
/// InitializeBackUpPackage` with the actor, its cell and the player), under
/// the same conditions as `008a75a0`.
pub fn fn_008a7760(e: &mut Engine, this: Ptr<Actor>) {
    if !package_change_allowed(e, this) {
        return;
    }
    let package = e.call(0x0067_0b90, &args![0x27u32]).u32();
    e.call(0x0067_0fc0, &args![package, 0x27u32]);
    e.call(0x0082_6b40, &args![package, 0u32]);
    e.call(0x0082_6b90, &args![package, 1u32]);
    let back_up = e
        .call(
            DYNAMIC_CAST,
            &args![package, 0u32, TYPE_TES_PACKAGE, TYPE_BACK_UP_PACKAGE, 0u32],
        )
        .u32();
    if back_up != 0 {
        let player: u32 = e.global(PLAYER_CHARACTER);
        let cell = e.call(GET_PARENT_CELL, &args![this]).u32();
        e.call(0x009e_d0d0, &args![back_up, this, cell, player]);
    }
    e.call(0x0098_4f60, &args![package, 0x30u32]);
    let proc = acquire_object(e, this);
    e.vcall(proc.addr(), 0x28, &args![]);
    e.vcall(this.addr(), 0x2f4, &args![package, 1u32, 1u32]);
}

// ---------------------------------------------------------------------------
// Second session: `008a7870` up to `008ac6f0` (the next 40 functions).
// ---------------------------------------------------------------------------

/// The compiler helper that truncates `ST0` to an integer (`_ftol2`).
const FTOL: u32 = 0x00ec_62c0;
/// The value `00491040` finds for animation slot `slot` (a word).
const ANIMATION_SLOT_VALUE: u32 = 0x0049_1040;
/// The animation group id (`004301b0`) stored for slot `slot` (a `u16`).
const ANIMATION_SLOT_GROUP: u32 = 0x0043_01b0;
/// `Animation::ClearGroup` (Xbox PDB): `(group, float)`.
const ANIMATION_CLEAR_GROUP: u32 = 0x0049_6080;
/// `Animation::PlayGroup` (Xbox PDB): `(group, flag, -1, -1)` style words.
const ANIMATION_PLAY_GROUP: u32 = 0x0049_4740;
/// `Animation::BlendOut` (Xbox PDB): `(slot, 0)` as the code pushes them.
const ANIMATION_BLEND_OUT: u32 = 0x0049_94f0;
/// `Actor::GetAnimGroup` (Xbox PDB): four words after `this`, a `u16`.
const ACTOR_GET_ANIM_GROUP: u32 = 0x0089_7910;
/// `TESAnimGroup::GetType` (Xbox PDB): a `cdecl` function of one word.
const ANIM_GROUP_GET_TYPE: u32 = 0x005f_2440;
/// `PlayerCharacter::GetAnimation` (Xbox PDB).
const PLAYER_GET_ANIMATION: u32 = 0x0095_0a60;
/// The extra data list of a reference (`005d43c0`).
const GET_EXTRA_LIST: u32 = 0x005d_43c0;
/// `InventoryChanges::GetInventoryChanges` (Xbox PDB), a `cdecl` function.
const GET_INVENTORY_CHANGES: u32 = 0x004b_f220;
/// `InventoryChanges::GetInventoryItem` (Xbox PDB).
const GET_INVENTORY_ITEM: u32 = 0x004d_0650;
/// The count word of an inventory item (`00726070`, `this + 4`).
const ITEM_COUNT: u32 = 0x0072_6070;
/// `ItemChange::ItemChange` (Xbox PDB), the constructor.
const ITEM_CHANGE_CONSTRUCTOR: u32 = 0x004b_c550;
/// `ItemChange::HasModEffectActive_ov2` (Xbox PDB).
const ITEM_CHANGE_HAS_MOD: u32 = 0x004b_da70;
/// `TESObjectWEAP::GetFormClipRounds` (Xbox PDB).
const GET_FORM_CLIP_ROUNDS: u32 = 0x004f_e160;
/// Returns the word at `this + 8` (`0044ddc0`).
const WORD_AT_8: u32 = 0x0044_ddc0;
/// Returns its `this` (`006815c0`); the first word of a list node is its
/// item.
const SELF: u32 = 0x0068_15c0;
/// `bhkCharacterController::GetPosition` (Xbox PDB).
const CHARACTER_GET_POSITION: u32 = 0x0081_2b00;
/// The name `"Bip01 NonAccum"` (a string at `01064f70`).
const BONE_NON_ACCUM: u32 = 0x0106_4f70;
/// The `GameSetting` for the pain timer delay (`011d0694`).
const SETTING_PAIN_DELAY: u32 = 0x011d_0694;

/// `Animation::ClearGroup(group, 0.0)`.
fn clear_group(e: &mut Engine, anim: u32, group: u32) {
    e.call(ANIMATION_CLEAR_GROUP, &args![anim, group, 0.0f32]);
}

/// `Actor::GetAnimGroup` with the four words after `this`; a `u16`.
fn get_anim_group(e: &mut Engine, this: Ptr<Actor>, words: [u32; 4]) -> u16 {
    e.call(
        ACTOR_GET_ANIM_GROUP,
        &args![this, words[0], words[1], words[2], words[3]],
    )
    .u16()
}

// Translated from 008a7870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::IsEating` (Xbox PDB): the process's current package (virtual
/// method `+0x22c`) is of form type 3 and the entry of table `011a3ff0` for
/// the package's index (`009611e0`) and the process's method `+0x280` is 5.
pub fn actor_is_eating(e: &mut Engine, this: Ptr<Actor>) -> bool {
    let proc = process(e, this);
    if proc.addr() == 0 {
        return false;
    }
    let package = e.vcall(proc.addr(), 0x22c, &args![]).u32();
    if package == 0 || e.call(FORM_TYPE, &args![package]).i32() != 3 {
        return false;
    }
    let table_index = e.call(0x0096_11e0, &args![package]).u32();
    let key = e.vcall(proc.addr(), 0x280, &args![]).u32();
    let table = e.mem.u32(PACKAGE_TYPE_TABLE + table_index.wrapping_mul(4));
    e.mem.u32(table.wrapping_add(key.wrapping_mul(4))) == 5
}

// Translated from 008a78f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::ShouldSkipFallOutBehavior` (Xbox PDB): with the process's current
/// package (method `+0x22c`) nonzero and `0067a380` true, the answer for
/// `kind` is the negation of a package test (`kind` 0, 1, 2, 4, 5, 6, 7, 8
/// use `0067a850`, `0067a8d0`, `0067a950`, `0067aa50`, `0067aad0`,
/// `0067ab50`, `0067abd0`, `0067ac50`); kind 3 and above 8 are false.
pub fn actor_should_skip_fall_out_behavior(e: &mut Engine, this: Ptr<Actor>, kind: u32) -> bool {
    let proc = process(e, this);
    let package = e.vcall(proc.addr(), 0x22c, &args![]).u32();
    if package == 0 {
        return false;
    }
    if !e.call(0x0067_a380, &args![package]).bool() {
        return false;
    }
    let test = match kind {
        0 => 0x0067_a850,
        1 => 0x0067_a8d0,
        2 => 0x0067_a950,
        4 => 0x0067_aa50,
        5 => 0x0067_aad0,
        6 => 0x0067_ab50,
        7 => 0x0067_abd0,
        8 => 0x0067_ac50,
        _ => return false,
    };
    !e.call(test, &args![package]).bool()
}

// Translated from 008a7a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::IsSurfacing` (Xbox PDB): the process's package (method `+0x27c`)
/// is of form type `0x1d`.
pub fn actor_is_surfacing(e: &mut Engine, this: Ptr<Actor>) -> bool {
    let proc = process(e, this);
    if proc.addr() == 0 {
        return false;
    }
    let package = e.vcall(proc.addr(), 0x27c, &args![]).u32();
    package != 0 && e.call(FORM_TYPE, &args![package]).i32() == 0x1d
}

/// Whether the animation group in slot 2 is one of the types `0xe6` to
/// `0xeb`, which `fn_008a7a90` leaves running.
fn slot_two_is_kept(e: &mut Engine, anim: u32) -> bool {
    let group = e.call(ANIMATION_SLOT_GROUP, &args![anim, 2u32]).u16();
    let kind = e.call(ANIM_GROUP_GET_TYPE, &args![group as u32]).i32();
    (0xe6..=0xeb).contains(&kind)
}

/// Restarts the base group when slot 0 holds nothing:
/// `Animation::PlayGroup(Actor::GetAnimGroup(...), 0, -1, -1)`.
fn restart_base_group(e: &mut Engine, this: Ptr<Actor>, anim: u32) {
    if e.call(ANIMATION_SLOT_VALUE, &args![anim, 0u32]).u32() == 0 {
        let group = get_anim_group(e, this, [0, 0, 0, 0]);
        e.call(
            ANIMATION_PLAY_GROUP,
            &args![anim, group as u32, 0u32, u32::MAX, u32::MAX],
        );
    }
}

// Translated from 008a7a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unless the actor's virtual method `+0x214` is nonzero, clears the
/// animation groups 0, 1, 2 (unless it holds a type `0xe6` to `0xeb`
/// group), 3 and 7 of the actor's animation, restarts the base group, and
/// ends the anim action (`fn_008a73e0(-1, 0)`) when one of the slots cleared
/// held the value the process's method `+0x3e8` names. For the player the
/// same clearing is applied to `PlayerCharacter::GetAnimation(1)`.
pub fn fn_008a7a90(e: &mut Engine, this: Ptr<Actor>) {
    if e.vcall(this.addr(), 0x214, &args![]).u32() != 0 {
        return;
    }
    let mut cleared_current = false;
    let proc = process(e, this);
    let current = e.vcall(proc.addr(), 0x3e8, &args![]).u32();
    let anim = e.call(GET_ANIMATION, &args![this]).u32();
    if anim != 0 {
        let matches = |e: &mut Engine, slot: u32| {
            current != 0 && e.call(ANIMATION_SLOT_VALUE, &args![anim, slot]).u32() == current
        };
        if matches(e, 0) {
            cleared_current = true;
        }
        let slot_group = e.call(ANIMATION_SLOT_GROUP, &args![anim, 0u32]).u16();
        if slot_group != get_anim_group(e, this, [0, 0, 0, 0]) {
            clear_group(e, anim, 0);
        }
        if matches(e, 1) {
            cleared_current = true;
        }
        clear_group(e, anim, 1);
        if !slot_two_is_kept(e, anim) {
            if matches(e, 2) {
                cleared_current = true;
            }
            clear_group(e, anim, 2);
        }
        if matches(e, 3) {
            cleared_current = true;
        }
        clear_group(e, anim, 3);
        if matches(e, 7) {
            cleared_current = true;
        }
        clear_group(e, anim, 7);
        restart_base_group(e, this, anim);
    }
    if cleared_current {
        actor_set_anim_action(e, this, -1, 0);
    }
    let player = e.global::<u32>(PLAYER_CHARACTER);
    if this.addr() == player {
        let anim = e.call(PLAYER_GET_ANIMATION, &args![player, 1u32]).u32();
        if anim != 0 {
            let slot_group = e.call(ANIMATION_SLOT_GROUP, &args![anim, 0u32]).u16();
            if slot_group != get_anim_group(e, this, [0, 0, 0, 0]) {
                clear_group(e, anim, 0);
            }
            clear_group(e, anim, 1);
            if !slot_two_is_kept(e, anim) {
                clear_group(e, anim, 2);
            }
            clear_group(e, anim, 3);
            clear_group(e, anim, 7);
            restart_base_group(e, this, anim);
        }
    }
}

// Translated from 008a7d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::TriggerPain` (Xbox PDB), `start_dialogue` and `show_effect` being the two
/// byte arguments the code reads at `+8` and `+0xc`. Does nothing when the
/// actor's methods `+0x22c(0)` or `+0x230` are true, `00608d80` is true,
/// the actor has no process or the process's field `+0x28` is nonzero.
/// With `008c7aa0` true the pain is queued instead (`TaskQueueInterface::
/// QueueActorTriggerPain`, arguments the actor, `start_dialogue`, `show_effect`). Otherwise the
/// pain timer (`fn_008a7f40`, `fn_008a7f60`) is checked against game
/// setting `011d0694`, restarted, and for the player the get-hit image
/// space modifier (when `show_effect`) and a controller rumble (outside menu
/// mode) are triggered; `start_dialogue` starts combat dialogue.
#[allow(clippy::neg_cmp_op_on_partial_ord)] // the x87 comparisons treat NaN as "not less"
pub fn actor_trigger_pain(e: &mut Engine, this: Ptr<Actor>, start_dialogue: u8, show_effect: u8) {
    if e.vcall(this.addr(), 0x22c, &args![0u32]).bool() {
        return;
    }
    if e.vcall(this.addr(), 0x230, &args![]).bool() {
        return;
    }
    if e.call(0x0060_8d80, &args![this]).bool() {
        return;
    }
    let proc = acquire_object(e, this);
    if proc.addr() == 0 {
        return;
    }
    let proc = acquire_object(e, this);
    if e.call(0x0045_cd60, &args![proc]).u32() != 0 {
        return;
    }
    if e.call(0x008c_7aa0, &args![]).bool() {
        let tes = e.call(0x0045_37b0, &args![]).u32();
        e.call(
            0x0087_b7c0,
            &args![tes, this, start_dialogue as u32, show_effect as u32],
        );
        return;
    }
    let timer_owner = acquire_object(e, this);
    let field = e.mem.f32(this.addr() + 0x114);
    let timer = fn_008a7f60(e, timer_owner);
    let mut proceed = !(field < timer);
    if !proceed {
        let timer = fn_008a7f60(e, timer_owner);
        let delay = setting_float(e, SETTING_PAIN_DELAY);
        proceed = !(f64::from(field) > f64::from(timer) - f64::from(delay));
    }
    if !proceed {
        return;
    }
    let delay = setting_float(e, SETTING_PAIN_DELAY);
    let next = (f64::from(field) + f64::from(delay)) as f32;
    fn_008a7f40(e, timer_owner, next);
    if this.addr() == e.global::<u32>(PLAYER_CHARACTER) {
        if show_effect != 0 && e.call(WORD_AT_8, &args![0x011f_2250u32]).u32() == 0 {
            let strength = setting_float(e, 0x011c_f834);
            let modifier = e.call(0x005d_2860, &args![]).u32();
            e.call(0x0052_99a0, &args![modifier, strength, 0u32]);
        }
        if !e.call(0x0070_2360, &args![]).bool() {
            let devices: u32 = e.global(0x011d_ea0c);
            let controls = e.call(0x0087_7720, &args![devices]).u32();
            let scale = setting_float(e, 0x011d_0508);
            let one_scale: f64 = e.global(0x0101_7b70);
            let duration = e.call(FTOL, &args![f64::from(scale) * one_scale]).i32();
            let high = setting_float(e, 0x011d_0f48);
            let low = setting_float(e, 0x011d_0f48);
            e.call(
                0x00a2_55b0,
                &args![controls, low, high, duration, 0u32, 0u32, 0u32, 0u32],
            );
        }
    }
    if start_dialogue != 0 {
        let dialogue: u32 = e.global(0x011f_1708);
        e.call(
            0x0098_39b0,
            &args![dialogue, this, 0u32, 2u32, 2u32, 0u32, 0u32],
        );
    }
}

// Translated from 008a7f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a `float` at `this + 0x344` (the process's pain timer).
pub fn fn_008a7f40(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_f32(this.addr() + 0x344, x87_quiet(value));
}

// Translated from 008a7f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `float` at `this + 0x344` (the process's pain timer).
pub fn fn_008a7f60(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + 0x344)
}

// Translated from 008a7f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// A location of the actor: `ExtraDataList` query `0042ea20` on the actor's
/// extra list, else the second generic location (`006733e0`) of the
/// process's package (method `+0x22c`, called twice as the code does), else
/// the result of the virtual method `+0x188` of the object `004181e0`
/// finds.
pub fn fn_008a7f80(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    let list = e.call(GET_EXTRA_LIST, &args![this]).u32();
    let mut result = e.call(0x0042_ea20, &args![list]).u32();
    if result == 0 {
        let proc = process(e, this);
        if proc.addr() != 0 && e.vcall(proc.addr(), 0x22c, &args![]).u32() != 0 {
            let package = e.vcall(proc.addr(), 0x22c, &args![]).u32();
            result = e.call(0x0067_33e0, &args![package]).u32();
        }
        if result == 0 {
            let object = e.call(0x0041_81e0, &args![this]).u32();
            result = e.vcall(object, 0x188, &args![]).u32();
        }
    }
    result
}

// Translated from 008a8010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Hands `value` to the actor's extra list (`0042ea50`) and, when the actor's
/// method `+0x428` returns an object, calls `0097f3f0` on it.
pub fn fn_008a8010(e: &mut Engine, this: Ptr<Actor>, value: u32) {
    let list = e.call(GET_EXTRA_LIST, &args![this]).u32();
    e.call(0x0042_ea50, &args![list, value]);
    let object = e.vcall(this.addr(), 0x428, &args![]).u32();
    if object != 0 {
        e.call(0x0097_f3f0, &args![object]);
    }
}

// Translated from 008a8060 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the process's field `+0x28` (`0045cd60`) is below 2, creates a
/// package of type `0x1b` (`TESPackage::CreatePackage`), initialises it
/// (`00826b90(1)`, `TESPackage::CalculateProcedureType(0)`) and gives it to
/// the actor's method `+0x2f4` with two ones.
pub fn fn_008a8060(e: &mut Engine, this: Ptr<Actor>) {
    let proc = acquire_object(e, this);
    if e.call(0x0045_cd60, &args![proc]).i32() < 2 {
        let package = e.call(0x0067_0b90, &args![0x1bu32]).u32();
        e.call(0x0082_6b90, &args![package, 1u32]);
        e.call(0x0067_77b0, &args![package, 0u32]);
        e.vcall(this.addr(), 0x2f4, &args![package, 1u32, 1u32]);
    }
}

// Translated from 008a80c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::SetHasBeenEaten` (Xbox PDB): when the actor's method `+0x22c(0)`
/// is true, finds the extra data of type `0x5f` (adding it with
/// `ExtraDataList::AddDismembermentExtra` and flagging the actor with
/// method `+0x48(0x20000)` when missing) and passes `value` to `008a8150`.
pub fn actor_set_has_been_eaten(e: &mut Engine, this: Ptr<Actor>, value: u8) {
    if !e.vcall(this.addr(), 0x22c, &args![0u32]).bool() {
        return;
    }
    let list = e.call(GET_EXTRA_LIST, &args![this]).u32();
    let mut extra = e.call(0x0041_0220, &args![list, 0x5fu32]).u32();
    if extra == 0 {
        let list = e.call(GET_EXTRA_LIST, &args![this]).u32();
        e.call(0x0042_e820, &args![list]);
        let list = e.call(GET_EXTRA_LIST, &args![this]).u32();
        extra = e.call(0x0041_0220, &args![list, 0x5fu32]).u32();
        e.vcall(this.addr(), 0x48, &args![0x20000u32]);
    }
    if extra != 0 {
        e.call(0x008a_8150, &args![extra, value as u32]);
    }
}

// Translated from 008a8300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::IsCloseToGround` (Xbox PDB): true without a character
/// controller, model or `"Bip01 NonAccum"` node. Otherwise whether
/// `limit >= node height - controller height` (the node's vector is read
/// at `0045bb80`, the controller's position with `GetPosition`; a NaN
/// compares true).
#[allow(clippy::neg_cmp_op_on_partial_ord)] // a NaN compares true, as the x87 test does
pub fn actor_is_close_to_ground(e: &mut Engine, this: Ptr<Actor>, limit: f32) -> bool {
    let controller = e.call(0x0093_06d0, &args![this]).u32();
    if controller == 0 {
        return true;
    }
    let model = e.call(0x0043_fcd0, &args![this]).u32();
    if model == 0 {
        return true;
    }
    let node = e.call(0x004a_ae30, &args![model, BONE_NON_ACCUM]).u32();
    if node == 0 {
        return true;
    }
    let vector = e.call(0x0045_bb80, &args![node]).u32();
    let node_z = e.mem.f32(vector + 8);
    e.with_stack(0x10, |e, position| {
        e.call(SELF, &args![position]);
        e.call(CHARACTER_GET_POSITION, &args![controller, position]);
        let height = (f64::from(node_z) - f64::from(e.mem.f32(position.addr() + 8))) as f32;
        !(limit < height)
    })
}

// Translated from 008a83c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `004938c0` on `first` and then, whatever it answered, the actor's
/// method `+0x3e8` with `first`, `second` and `third`, whose byte is the
/// answer (the exe ends on that call and keeps its `AL`; the virtual slot's
/// `bool`). The fourth word is not read.
pub fn fn_008a83c0(
    e: &mut Engine,
    this: Ptr<Actor>,
    first: u32,
    second: u32,
    third: u8,
    _unused_4: u32,
) -> bool {
    e.call(0x0049_38c0, &args![first]);
    e.vcall(this.addr(), 0x3e8, &args![first, second, third as u32])
        .bool()
}

// Translated from 008a8420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reloads the actor's weapon with `weapon`'s ammunition (the name is not
/// in the Xbox PDB). `weapon` is the weapon form (`0` answers true at
/// once); the ammunition is `TESObjectWEAP::GetCurrentAmmo` (`00525980`), or
/// the form itself for form types 10, 11 and 13 (`00446390`). The actor's
/// inventory entry for it is found in `InventoryChanges`; when the process
/// holds no item change (method `+0x14c`) one is created and stored with
/// method `+0x168`. `mode` 2 plays the reload animation group when the clip
/// is not full, `mode` above 0 informs the combat controller (method
/// `+0x428`, `0097f7a0`), and the clip count is set with
/// `CombatProcedureAttackMelee::Initialize` (`006ecd40`); `mode` 1 starts
/// the group (or stores it with `fn_008a8820`). True when the item change
/// and the inventory item exist and the count is nonzero.
///
/// C++ exception frames are not translated.
pub fn fn_008a8420(e: &mut Engine, this: Ptr<Actor>, weapon: u32, mode: i32, flag: u8) -> bool {
    if weapon == 0 {
        return true;
    }
    let mut ammo = e.call(0x0052_5980, &args![weapon, this]).u32();
    if ammo == 0 {
        let is_ammo_kind =
            |e: &mut Engine, kind: i32| e.call(0x0044_6390, &args![weapon]).i32() == kind;
        if is_ammo_kind(e, 10) || is_ammo_kind(e, 0xb) || is_ammo_kind(e, 0xd) {
            ammo = weapon;
        }
    }
    if ammo == 0 {
        return false;
    }
    let act = e.vcall(this.addr(), 0x1e4, &args![]).u32();
    let changes = e.call(GET_INVENTORY_CHANGES, &args![this]).u32();
    let item = e
        .call(GET_INVENTORY_ITEM, &args![changes, ammo, 0u32])
        .u32();
    let mut count = if item != 0 {
        e.call(ITEM_COUNT, &args![item]).i32()
    } else {
        0
    };
    let initial_count = count;
    let proc = process(e, this);
    let mut change = e.vcall(proc.addr(), 0x14c, &args![]).u32();
    let held = e.vcall(proc.addr(), 0x148, &args![]).u32();
    if change == 0 && item != 0 {
        let block = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
        change = if block != 0 {
            let word = e.call(WORD_AT_8, &args![item]).u32();
            e.call(ITEM_CHANGE_CONSTRUCTOR, &args![block, word, initial_count])
                .u32()
        } else {
            0
        };
        let proc = process(e, this);
        e.vcall(proc.addr(), 0x168, &args![change]);
    }
    if mode == 2 && change != 0 {
        let modifier = if held != 0 {
            e.call(ITEM_CHANGE_HAS_MOD, &args![held, 2u32]).u8()
        } else {
            0
        };
        let rounds = e
            .call(GET_FORM_CLIP_ROUNDS, &args![weapon, modifier as u32])
            .i32();
        if rounds != e.call(ITEM_COUNT, &args![change]).i32()
            && initial_count != e.call(ITEM_COUNT, &args![change]).i32()
        {
            let group_id = e.call(0x0051_e2a0, &args![weapon, flag as u32]).i32();
            let group = get_anim_group(e, this, [group_id as u32, 0, 0, 0]);
            if e.call(ANIM_GROUP_GET_TYPE, &args![group as u32]).i32() == group_id {
                e.call(
                    ANIMATION_PLAY_GROUP,
                    &args![act, group as u32, 1u32, u32::MAX, u32::MAX],
                );
                let slot = e.call(ANIMATION_SLOT_VALUE, &args![act, 4u32]).u32();
                actor_set_anim_action(e, this, 9, slot);
            }
        }
    }
    if mode > 0 {
        let controller = e.vcall(this.addr(), 0x428, &args![]).u32();
        if controller != 0 {
            e.call(0x0097_f7a0, &args![controller, weapon, (mode == 2) as u32]);
        }
    }
    let result = if initial_count != 0 && item != 0 && change != 0 {
        let uses_ammo =
            actor_should_use_ammo(e, this, weapon) || e.call(0x004c_0bf0, &args![weapon]).bool();
        let mut take_rounds = !uses_ammo;
        if uses_ammo {
            let rounds = e
                .call(GET_FORM_CLIP_ROUNDS, &args![weapon, flag as u32])
                .i32();
            if rounds < initial_count {
                take_rounds = true;
            }
        }
        if take_rounds {
            count = e
                .call(GET_FORM_CLIP_ROUNDS, &args![weapon, flag as u32])
                .i32();
        } else {
            count = initial_count;
        }
        e.call(0x006e_cd40, &args![change, count]);
        if mode == 1 {
            let mut group_id = e.call(0x0051_e2a0, &args![weapon, flag as u32]).i32();
            if e.call(0x0049_38c0, &args![weapon]).bool() {
                group_id -= 0x17;
            }
            let group = get_anim_group(e, this, [group_id as u32, 0, 0, 0]);
            if e.call(ANIM_GROUP_GET_TYPE, &args![group as u32]).i32() == group_id {
                if e.call(0x0052_4b40, &args![weapon]).bool() {
                    e.call(ANIMATION_BLEND_OUT, &args![act, 5u32, 0u32]);
                    e.call(ANIMATION_BLEND_OUT, &args![act, 6u32, 0u32]);
                    e.call(
                        ANIMATION_PLAY_GROUP,
                        &args![act, group as u32, 1u32, u32::MAX, u32::MAX],
                    );
                    let slot = e.call(ANIMATION_SLOT_VALUE, &args![act, 4u32]).u32();
                    actor_set_anim_action(e, this, 9, slot);
                } else {
                    fn_008a8820(e, Ptr::new(act), group);
                }
            }
        }
        true
    } else {
        false
    };
    if item != 0 {
        e.call(0x0044_59e0, &args![item, 1u32]);
    }
    result
}

// Translated from 008a8820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a `u16` at `this + 0x122`.
pub fn fn_008a8820(e: &mut Engine, this: Ptr, value: u16) {
    e.mem.set_u16(this.addr() + 0x122, value);
}

// Translated from 008a8840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::QueueReload` (Xbox PDB): with a process, calls its method
/// `+0x614(2)`.
pub fn actor_queue_reload(e: &mut Engine, this: Ptr<Actor>) {
    let proc = process(e, this);
    if proc.addr() != 0 {
        e.vcall(proc.addr(), 0x614, &args![2u32]);
    }
}

// Translated from 008a8870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `Actor::GetAnimAction` is 9 or 15 to 17.
pub fn fn_008a8870(e: &mut Engine, this: Ptr<Actor>) -> bool {
    let action = actor_get_anim_action(e, this);
    action == 9 || (action > 0xe && action <= 0x11)
}

// Translated from 008a88b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The count (`00726070`) of the item change the process's method `+0x14c`
/// returns, or `-1` without one.
pub fn fn_008a88b0(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    let proc = process(e, this);
    let change = e.vcall(proc.addr(), 0x14c, &args![]).u32();
    if change == 0 {
        u32::MAX
    } else {
        e.call(ITEM_COUNT, &args![change]).u32()
    }
}

// Translated from 008a88f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetCurrentAmmoClipPercent` (Xbox PDB): the count of the current
/// item change (process method `+0x14c`) divided by the clip size of the
/// weapon (method `+0x148`, `GetFormClipRounds` with its mod effect), or
/// `1.0` without them or when the clip size is zero.
pub fn actor_get_current_ammo_clip_percent(e: &mut Engine, this: Ptr<Actor>) -> f32 {
    let proc = process(e, this);
    let change = e.vcall(proc.addr(), 0x14c, &args![]).u32();
    if change == 0 {
        return 1.0;
    }
    let held = e.vcall(proc.addr(), 0x148, &args![]).u32();
    if held == 0 {
        return 1.0;
    }
    let weapon = e.call(WORD_AT_8, &args![held]).u32();
    let modifier = e.call(ITEM_CHANGE_HAS_MOD, &args![held, 2u32]).u8();
    let rounds = e
        .call(GET_FORM_CLIP_ROUNDS, &args![weapon, modifier as u32])
        .i32();
    let clip = rounds as f32;
    let count = e.call(ITEM_COUNT, &args![change]).i32() as f32;
    let zero: f64 = e.global(ZERO_DOUBLE);
    if f64::from(clip) == zero {
        return 1.0;
    }
    (f64::from(count) / f64::from(clip)) as f32
}

// Translated from 008a8dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::ShouldUseAmmo` (Xbox PDB): true for the player; otherwise, for a
/// `weapon` form, true when its flag test `008a8e30` holds, or when
/// `0047bcf0` holds and the actor's `MiddleHighProcess::GetForceNextUpdate`
/// (`00566950`) does.
pub fn actor_should_use_ammo(e: &mut Engine, this: Ptr<Actor>, weapon: u32) -> bool {
    if this.addr() == e.global::<u32>(PLAYER_CHARACTER) {
        return true;
    }
    if weapon != 0 {
        if fn_008a8e30(e, Ptr::new(weapon)) {
            return true;
        }
        if !e.call(0x0047_bcf0, &args![weapon]).bool() {
            return false;
        }
        if e.call(0x0056_6950, &args![this]).bool() {
            return true;
        }
    }
    false
}

// Translated from 008a8e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 2 of the word at `this + 0x12c` is set.
pub fn fn_008a8e30(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0x12c) & 2 != 0
}

// Translated from 008a8e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::StopAttack` (Xbox PDB): when the process's method `+0x3e4` is 2
/// or 4, clears animation groups 4 and 2 (those of the player's two
/// animations, `PlayerCharacter::GetAnimation(1)` and `(0)`, or of the
/// process's method `+0x1b8`) and tells the process (`+0x3ec(-1, 0)`).
pub fn actor_stop_attack(e: &mut Engine, this: Ptr<Actor>) {
    let proc = process(e, this);
    if proc.addr() == 0 {
        return;
    }
    let state = e.vcall(proc.addr(), 0x3e4, &args![]).i32();
    if state != 2 && state != 4 {
        return;
    }
    let player = e.global::<u32>(PLAYER_CHARACTER);
    if this.addr() == player {
        let first = e.call(PLAYER_GET_ANIMATION, &args![player, 1u32]).u32();
        let second = e.call(PLAYER_GET_ANIMATION, &args![player, 0u32]).u32();
        clear_group(e, first, 4);
        clear_group(e, second, 4);
        clear_group(e, first, 2);
        clear_group(e, second, 2);
    } else {
        let anim = e.vcall(proc.addr(), 0x1b8, &args![]).u32();
        clear_group(e, anim, 4);
        clear_group(e, anim, 2);
    }
    let proc = process(e, this);
    e.vcall(proc.addr(), 0x3ec, &args![u32::MAX, 0u32]);
}

// Translated from 008a8f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks the list in the actor's extra data of type `0x1d` (`00422700`),
/// starting at its field `+0xc`, and for every entry whose actor's process
/// has a package (method `+0x27c`) of form type 7 calls the process's
/// method `+0x284(0)`. Stops at the first entry without an item.
pub fn fn_008a8f60(e: &mut Engine, this: Ptr<Actor>) {
    let list = e.call(GET_EXTRA_LIST, &args![this]).u32();
    let data = e.call(0x0042_2700, &args![list]).u32();
    if data == 0 {
        return;
    }
    let mut node = e.mem.u32(data + 0xc);
    while node != 0 {
        let item_slot = e.call(SELF, &args![node]).u32();
        if e.mem.u32(item_slot) == 0 {
            return;
        }
        let item_slot = e.call(SELF, &args![node]).u32();
        let target = e.mem.u32(item_slot);
        if e.call(GET_PROCESS, &args![target]).u32() != 0 {
            let proc = e.call(GET_PROCESS, &args![target]).u32();
            let package = e.vcall(proc, 0x27c, &args![]).u32();
            if package != 0 && e.call(FORM_TYPE, &args![package]).i32() == 7 {
                let proc = e.call(GET_PROCESS, &args![target]).u32();
                e.vcall(proc, 0x284, &args![0u32]);
            }
        }
        node = e.call(ITEM_COUNT, &args![node]).u32();
    }
}

/// `Actor::ClearInCombat` (Xbox PDB): `(this, 0)`.
const CLEAR_IN_COMBAT: u32 = 0x008a_08e0;

/// The save/load object (`TESSaveLoadGame`): the pointer stored at `011de45c`.
fn save_load_game(e: &Engine) -> u32 {
    e.global(0x011d_e45c)
}

/// The save version byte (`008df040`, `this + 0x80` of the save/load object).
fn save_version(e: &mut Engine) -> u32 {
    let owner = save_load_game(e);
    e.call(0x008d_f040, &args![owner]).u8() as u32
}

/// `TESSaveLoadGame::UseSaveGameBlocks` (Xbox PDB, `00862110`).
fn uses_save_blocks(e: &mut Engine) -> bool {
    let owner = save_load_game(e);
    e.call(0x0086_2110, &args![owner]).bool()
}

/// The write/read position of the save buffer (`00825c00`, `this + 0x14`).
fn save_position(e: &mut Engine) -> u32 {
    let owner = save_load_game(e);
    e.call(0x0082_5c00, &args![owner]).u32()
}

/// Whether a list node is empty (`008256d0`: both words are zero).
const NODE_IS_EMPTY: u32 = 0x0082_56d0;
/// The next node of a list node (`00726070` reads `this + 4`).
const NEXT_NODE: u32 = ITEM_COUNT;
/// The debug setting byte (`011de4e8`) that makes the save code log sizes.
const SAVE_LOG_SETTING: u32 = 0x011d_e4e8;
/// The source file name the save code reports in its messages.
const ACTOR_SOURCE_FILE: u32 = 0x0108_4918;
/// `BGSSaveFormBuffer`-style tests on a buffer: `428110` copies the word at
/// `buffer + 0x17` to its argument, `42ce30` the word at `buffer + 0x2c`, and
/// `004280f0` tests the copy against a mask.
const BUFFER_COPY_FLAGS_A: u32 = 0x0042_8110;
const BUFFER_COPY_FLAGS_B: u32 = 0x0042_ce30;
const FLAGS_TEST: u32 = 0x0042_80f0;
/// `Error` (Xbox PDB), a `cdecl` function with a format string.
const ERROR: u32 = 0x0040_fbe0;
/// The save layout error report of `005b5e40` (a `cdecl` format function).
const SAVELOAD_ERROR: u32 = 0x005b_5e40;
/// `TESForm::SaveGameDataOLD` (Xbox PDB): `(buffer owner, pointer, size)`.
const SAVE_GAME_DATA_OLD: u32 = 0x0048_4ce0;
/// `TESForm::SaveNumericID` (Xbox PDB).
const SAVE_NUMERIC_ID: u32 = 0x0048_4d20;
/// `TESForm::LoadGameDataOLD` (Xbox PDB).
const LOAD_GAME_DATA_OLD: u32 = 0x0048_4d00;
/// `TESForm::LoadNumericID` (Xbox PDB).
const LOAD_NUMERIC_ID: u32 = 0x0048_4d40;
/// The form id of a form (`0084e3a0` reads `this + 0xc`).
const FORM_ID_OF: u32 = 0x0084_e3a0;
/// Looks a form up by id (`004839c0`), a `cdecl` function.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// The `__RTDynamicCast` type descriptors the save code casts between.
const CAST_SOURCE_TYPE: u32 = 0x0118_3028;
const CAST_TO_011846D4: u32 = 0x0118_46d4;
const CAST_TO_011846E8: u32 = 0x0118_46e8;
const CAST_TO_01184920: u32 = 0x0118_4920;
const CAST_TO_01183060: u32 = 0x0118_3060;
const CAST_FROM_011A0D1C: u32 = 0x011a_0d1c;
const CAST_TO_011A146C: u32 = 0x011a_146c;
/// The text `"Lily"` (`0102fdc8`), searched for in the actor's name.
const NAME_FRAGMENT: u32 = 0x0102_fdc8;

/// Copies a flag word of `buffer` (`428110`) and tests `mask` in it.
fn buffer_flag_a(e: &mut Engine, buffer: u32, mask: u32) -> bool {
    e.with_stack(4, |e, copy| {
        let copy = e.call(BUFFER_COPY_FLAGS_A, &args![buffer, copy]).u32();
        e.call(FLAGS_TEST, &args![copy, mask]).bool()
    })
}

/// [`buffer_flag_a`] with the other flag word (`42ce30`).
fn buffer_flag_b(e: &mut Engine, buffer: u32, mask: u32) -> bool {
    e.with_stack(4, |e, copy| {
        let copy = e.call(BUFFER_COPY_FLAGS_B, &args![buffer, copy]).u32();
        e.call(FLAGS_TEST, &args![copy, mask]).bool()
    })
}

/// `__RTDynamicCast(LookupForm(id), 0, source, target, 0)`: the form with the
/// id, cast to `target`.
fn lookup_and_cast(e: &mut Engine, id: u32, target: u32) -> u32 {
    let form = e.call(LOOKUP_FORM, &args![id]).u32();
    e.call(
        DYNAMIC_CAST,
        &args![form, 0u32, CAST_SOURCE_TYPE, target, 0u32],
    )
    .u32()
}

/// Reports the save code's position in the form being saved: the message
/// `fmt_form` (with the form id, name and flags) when the buffer knows the
/// form being processed, else `fmt_none`. `diff` is the size or distance.
fn report_with_form(e: &mut Engine, fmt_form: u32, fmt_none: u32, diff: u32, line: u32) {
    let owner = save_load_game(e);
    let current = e.call(0x004f_d3e0, &args![owner]).u32();
    if current != 0 {
        let form_id = e.mem.u32(current);
        let flags = e.mem.u32(current + 5);
        let namer = e.call(LOOKUP_FORM, &args![form_id]).u32();
        let name = e.vcall(namer, 0x130, &args![]).u32();
        e.call(
            ERROR,
            &args![
                fmt_form,
                diff,
                form_id,
                name,
                flags,
                line,
                ACTOR_SOURCE_FILE
            ],
        );
    } else {
        e.call(ERROR, &args![fmt_none, diff, line, ACTOR_SOURCE_FILE]);
    }
}

// Translated from 008a9020 (decompiled, FalloutNV.exe 1.4.0.525)
/// The number of bytes `fn_008a9380` writes for the actor (the name is not
/// in the Xbox PDB; the code logs it as `GetSaveSize()`): `MobileObject`'s
/// size (`00931ff0`) plus the fields of the format, each part depending on
/// the save version (`008df040`) and the `flags`. The 16-bit sum wraps. The
/// parts the compiler masked with a constant zero are not translated (they
/// never ran).
pub fn fn_008a9020(e: &mut Engine, this: Ptr<Actor>, flags: u32) -> u16 {
    let base = e.call(0x0093_1ff0, &args![this, flags]).u16();
    let mut size: u16 = base;
    let start = size;
    if uses_save_blocks(e) {
        // The block tag (4 bytes) and its length (2).
        size = size.wrapping_add(6);
    }
    // Three bytes and a word.
    size = size.wrapping_add(7);
    if flags & 0x400 != 0 {
        size = size.wrapping_add(1);
    }
    if flags & 0x80000 != 0 {
        size = size.wrapping_add(2);
        let mut node = this.addr() + 0xfc;
        while node != 0 && !e.call(NODE_IS_EMPTY, &args![node]).bool() {
            let slot = e.call(SELF, &args![node]).u32();
            let item = e.mem.u32(slot);
            if e.mem.u32(item + 4) != 0 {
                size = size.wrapping_add(8);
            }
            node = e.call(NEXT_NODE, &args![node]).u32();
        }
    }
    size = size.wrapping_add(2);
    let count = e.call(0x005a_e380, &args![this.addr() + 0xf4]).u32();
    size = (size as u32).wrapping_add(count.wrapping_mul(8)) as u16;
    if save_version(e) >= 0x32 {
        size = size.wrapping_add(4);
    }
    if save_version(e) >= 0x3c {
        size = size.wrapping_add(4);
    }
    if save_version(e) >= 0x44 && flags & 0x80_0000 != 0 {
        let modifiers = e.call(0x0093_7ab0, &args![this.addr() + 0xd0]).u16();
        size = size.wrapping_add(modifiers);
    }
    if save_version(e) >= 0x45 {
        size = size.wrapping_add(5);
    }
    if save_version(e) >= 0x61 {
        size = size.wrapping_add(4);
    }
    if save_version(e) >= 0x65 {
        size = size.wrapping_add(4);
    }
    if save_version(e) >= 0x71 {
        size = size.wrapping_add(10);
    }
    if save_version(e) >= 0x73 {
        size = size.wrapping_add(1);
    }
    if save_version(e) >= 0x7b {
        size = size.wrapping_add(1);
    }
    let setting = e.call(0x0040_8d60, &args![SAVE_LOG_SETTING]).u32();
    if e.mem.u8(setting) != 0 {
        let diff = (size as u32).wrapping_sub(start as u32);
        report_with_form(e, 0x0101_2cb0, 0x0101_2c78, diff, 0x4c64);
    }
    size
}

/// The `(offset, size)` pairs of the actor fields `fn_008a9380` writes after
/// the (optional) block header and `MobileObject` part.
const SAVE_FIELDS_FIRST: [(u32, u32); 4] = [(0x114, 4), (0x124, 1), (0x125, 1), (0xbc, 1)];

// Translated from 008a9380 (decompiled, FalloutNV.exe 1.4.0.525)
/// The older save routine of the actor (the name is not in the Xbox PDB; its
/// sibling `Actor::SaveGame` is `008aaf40`), writing to the save/load
/// object: `MobileObject::SaveGame` (`00932070`), a block header when
/// `UseSaveGameBlocks`, the fields of the format by save version, and the
/// two lists at `this + 0xfc` and `this + 0xf4` with their counts patched
/// in afterwards. Writes the sizes it produced to the log when the debug
/// setting `011de4e8` is set. The parts the compiler masked with a constant
/// zero are not translated (they never ran).
pub fn fn_008a9380(e: &mut Engine, this: Ptr<Actor>, flags: u32) {
    e.call(0x0093_2070, &args![this, flags]);
    e.with_stack(0x40, |e, scratch| {
        let tag = scratch.addr();
        let zero_word = scratch.addr() + 4;
        let byte = scratch.addr() + 8;
        let count_one = scratch.addr() + 0xc;
        let id = scratch.addr() + 0x10;
        let count_two = scratch.addr() + 0x14;
        let mut block_size_slot = 0u32;
        let owner = save_load_game(e);
        let mut start = save_position(e);
        let setting = e.call(0x0040_8d60, &args![SAVE_LOG_SETTING]).u32();
        if e.mem.u8(setting) != 0 {
            start = save_position(e);
        }
        if uses_save_blocks(e) {
            e.mem.set_u32(tag, 0x424c_4f4b);
            e.call(0x0085_79b0, &args![owner, tag, 4u32]);
            block_size_slot = save_position(e);
            e.call(0x0085_79b0, &args![owner, zero_word, 2u32]);
        }
        for (offset, size) in SAVE_FIELDS_FIRST {
            e.call(SAVE_GAME_DATA_OLD, &args![this, this.addr() + offset, size]);
        }
        if flags & 0x400 != 0 {
            let state = e.call(0x004f_8960, &args![this]).u8();
            e.mem.set_u8(byte, state);
            e.call(SAVE_GAME_DATA_OLD, &args![this, byte, 1u32]);
        }
        if flags & 0x80000 != 0 {
            e.mem.set_u16(count_one, 0);
            let place = save_position(e);
            e.call(0x0085_79b0, &args![owner, count_one, 2u32]);
            let mut node = this.addr() + 0xfc;
            while node != 0 && !e.call(NODE_IS_EMPTY, &args![node]).bool() {
                let slot = e.call(SELF, &args![node]).u32();
                let item = e.mem.u32(slot);
                if e.mem.u32(item + 4) != 0 {
                    let form = e.mem.u32(item + 4);
                    let form_id = e.call(FORM_ID_OF, &args![form]).u32();
                    e.mem.set_u32(id, form_id);
                    e.call(SAVE_NUMERIC_ID, &args![this, id, 4u32]);
                    e.call(SAVE_GAME_DATA_OLD, &args![this, item, 4u32]);
                    let counted = e.mem.u16(count_one);
                    e.mem.set_u16(count_one, counted.wrapping_add(1));
                }
                node = e.call(NEXT_NODE, &args![node]).u32();
            }
            let counted = e.mem.u16(count_one);
            e.mem.set_u16(place, counted);
        }
        // The `flags & 0` block that would save a form id (`0085b170`) never ran.
        e.mem.set_u16(count_two, 0);
        let place = save_position(e);
        e.call(0x0085_79b0, &args![owner, count_two, 2u32]);
        let mut node = this.addr() + 0xf4;
        while node != 0 && !e.call(NODE_IS_EMPTY, &args![node]).bool() {
            let slot = e.call(SELF, &args![node]).u32();
            let entry = e.mem.u32(slot);
            let mut form_id = 0;
            if e.mem.u32(entry) != 0 {
                let form = e.mem.u32(entry);
                form_id = e.call(FORM_ID_OF, &args![form]).u32();
            }
            e.mem.set_u32(id, form_id);
            e.call(SAVE_NUMERIC_ID, &args![this, id, 4u32]);
            e.call(SAVE_GAME_DATA_OLD, &args![this, entry + 4, 4u32]);
            let counted = e.mem.u16(count_two);
            e.mem.set_u16(count_two, counted.wrapping_add(1));
            node = e.call(NEXT_NODE, &args![node]).u32();
        }
        let counted = e.mem.u16(count_two);
        e.mem.set_u16(place, counted);
        // A form reference at `this + offset`, saved as its form id (0 if none).
        let save_reference = |e: &mut Engine, offset: u32| {
            let mut form_id = 0;
            let form = e.mem.u32(this.addr() + offset);
            if form != 0 {
                form_id = e.call(FORM_ID_OF, &args![form]).u32();
            }
            e.mem.set_u32(id, form_id);
            e.call(SAVE_NUMERIC_ID, &args![this, id, 4u32]);
        };
        if save_version(e) >= 0x32 {
            save_reference(e, 0xc0);
        }
        if save_version(e) >= 0x3c {
            save_reference(e, 0x148);
        }
        if save_version(e) >= 0x44 && flags & 0x80_0000 != 0 {
            e.call(0x0093_78d0, &args![this.addr() + 0xd0]);
        }
        if save_version(e) >= 0x45 {
            e.call(0x0085_79b0, &args![owner, this.addr() + 0xc4, 1u32]);
            e.mem.set_u32(id, 0);
            e.call(0x0085_7a10, &args![owner, id, 4u32]);
        }
        if save_version(e) >= 0x61 {
            let mut form_id = 0;
            let form = e.mem.u32(this.addr() + 0x70);
            if form != 0 {
                form_id = e.call(FORM_ID_OF, &args![form]).u32();
            }
            e.mem.set_u32(id, form_id);
            e.call(0x0085_7a10, &args![owner, id, 4u32]);
        }
        if save_version(e) >= 0x65 {
            e.call(SAVE_GAME_DATA_OLD, &args![this, this.addr() + 0xc8, 4u32]);
        }
        if save_version(e) >= 0x71 {
            e.call(SAVE_GAME_DATA_OLD, &args![this, this.addr() + 0x126, 1u32]);
            e.call(SAVE_GAME_DATA_OLD, &args![this, this.addr() + 0x14c, 1u32]);
            e.call(SAVE_GAME_DATA_OLD, &args![this, this.addr() + 0x158, 4u32]);
            e.call(SAVE_GAME_DATA_OLD, &args![this, this.addr() + 0x74, 4u32]);
        }
        if save_version(e) >= 0x73 {
            e.call(SAVE_GAME_DATA_OLD, &args![this, this.addr() + 0x174, 1u32]);
        }
        if save_version(e) >= 0x7b {
            e.call(SAVE_GAME_DATA_OLD, &args![this, this.addr() + 0x118, 1u32]);
        }
        let setting = e.call(0x0040_8d60, &args![SAVE_LOG_SETTING]).u32();
        if e.mem.u8(setting) != 0 {
            let now = save_position(e);
            report_with_form(e, 0x0101_53a0, 0x0101_536c, now.wrapping_sub(start), 0x4d09);
        }
        if uses_save_blocks(e) {
            let now = save_position(e);
            if now > block_size_slot.wrapping_add(0xffff) {
                e.call(
                    SAVELOAD_ERROR,
                    &args![0x0101_5318u32, ACTOR_SOURCE_FILE, 0x4d09u32],
                );
            }
            e.mem
                .set_u16(block_size_slot, now.wrapping_sub(block_size_slot) as u16);
        }
    });
}

// Translated from 008a9940 (decompiled, FalloutNV.exe 1.4.0.525)
/// The older load routine of the actor (the name is not in the Xbox PDB;
/// the sibling `fn_008ab3a0` is the newer one). `flags` is the load flag
/// word the code tests, `extra_flags` the second word it hands on to
/// `MobileObject::LoadGame` (`009320e0`).
///
/// For form type `0x2a` or `0x2b` of the actor's base form (`007af430`,
/// `00401170`), when only `extra_flags` has `0x20`, the default worn items
/// are set up (`006047c0` or `005f9e00`). When only `extra_flags` has
/// `0x400` the actor's life state is reset (`008a1800`, `008acc80`) or its
/// ragdoll data restored (`00577330`). Then the fields of the format are read
/// from the save/load object in the order `fn_008a9380` wrote them, by save
/// version, with the two lists at `this + 0xfc` and `this + 0xf4` rebuilt;
/// block header and size mismatches are logged (`005b5e40`). The parts the
/// compiler masked with a constant zero are not translated (they never ran).
pub fn fn_008a9940(e: &mut Engine, this: Ptr<Actor>, flags: u32, extra_flags: u32) {
    if extra_flags & 0x20 != 0 && flags & 0x20 == 0 {
        let form = e.call(GET_SAVE_FORM, &args![this]).u32();
        let mut npc_base = 0;
        let mut creature_base = 0;
        let kind = e.call(0x0040_1170, &args![form]).i32();
        if kind == 0x2a {
            npc_base = form;
        } else if kind == 0x2b {
            creature_base = form;
        }
        let first_flag = 1u32;
        let mut second_flag = 1u32;
        let package = e.call(GET_CURRENT_PACKAGE, &args![this]).u32();
        if package != 0 && e.call(0x0044_1b00, &args![package]).bool() {
            second_flag = 0;
        }
        if npc_base != 0 {
            e.call(
                0x0060_47c0,
                &args![npc_base, this, first_flag, second_flag, 0u32, 1u32],
            );
        } else if creature_base != 0 {
            e.call(
                0x005f_9e00,
                &args![creature_base, this, first_flag, second_flag, 1u32],
            );
        }
    }
    if flags & 0x400 == 0 && extra_flags & 0x400 != 0 {
        let base = e.call(0x0041_81e0, &args![this]).u32();
        if !e.call(0x005f_0f50, &args![base]).bool() {
            e.call(0x008a_1800, &args![this, 0u32]);
            e.mem.set_u8(this.addr() + 0x118, 0);
            e.call(0x008a_cc80, &args![this]);
        } else {
            e.call(0x0057_7330, &args![this, 0u32]);
        }
    }
    let saved_state = e.call(0x004f_8960, &args![this]).u32();
    let saved_byte = e.mem.u8(this.addr() + 0x118);
    let handler: u32 = e.global(DATA_HANDLER);
    e.call(0x0055_0890, &args![handler, 1u32]);
    e.call(0x0093_20e0, &args![this, flags, extra_flags]);
    e.call(0x0055_0890, &args![handler, 0u32]);
    let owner = save_load_game(e);
    e.with_stack(0x40, |e, scratch| {
        let tag = scratch.addr();
        let size_word = scratch.addr() + 4;
        let byte = scratch.addr() + 8;
        let count_word = scratch.addr() + 0xc;
        let id = scratch.addr() + 0x10;
        let block_slot = scratch.addr() + 0x14;
        e.mem.set_u16(size_word, 0);
        let mut base = 0u32;
        if uses_save_blocks(e) {
            e.call(0x0085_79e0, &args![owner, tag, 4u32]);
            if e.mem.u32(tag) != 0x424c_4f4b {
                let current = e.call(0x004f_d3c0, &args![owner]).u32();
                if current != 0 {
                    let form_id = e.mem.u32(current);
                    let namer = e.call(LOOKUP_FORM, &args![form_id]).u32();
                    let flags_word = e.mem.u32(current + 5);
                    let version = e.mem.u8(current + 9) as u32;
                    let name = e.vcall(namer, 0x130, &args![]).u32();
                    e.call(
                        SAVELOAD_ERROR,
                        &args![
                            0x0101_5718u32,
                            ACTOR_SOURCE_FILE,
                            0x4d48u32,
                            form_id,
                            name,
                            version,
                            flags_word
                        ],
                    );
                } else {
                    let version = save_version(e);
                    e.call(
                        SAVELOAD_ERROR,
                        &args![0x0101_56a8u32, ACTOR_SOURCE_FILE, 0x4d48u32, version],
                    );
                }
            }
            base = save_position(e);
            e.call(0x0085_79e0, &args![owner, size_word, 2u32]);
        }
        e.mem.set_u32(this.addr() + 0x108, saved_state);
        e.mem.set_u8(this.addr() + 0x118, saved_byte);
        for (offset, size) in [(0x114u32, 4u32), (0x124, 1), (0x125, 1)] {
            e.call(LOAD_GAME_DATA_OLD, &args![this, this.addr() + offset, size]);
        }
        if save_version(e) >= 0x25 {
            e.call(LOAD_GAME_DATA_OLD, &args![this, this.addr() + 0xbc, 1u32]);
        }
        if flags & 0x400 != 0 {
            e.call(LOAD_GAME_DATA_OLD, &args![this, byte, 1u32]);
            let current_state = e.call(0x004f_8960, &args![this]).u32();
            let loaded = e.mem.u8(byte) as u32;
            if current_state != loaded {
                e.mem.set_u32(this.addr() + 0x108, loaded);
                let loaded_is_kept = loaded == 2 || loaded == 1 || loaded == 6;
                let was_kept = saved_state == 2 || saved_state == 1 || saved_state == 6;
                if !loaded_is_kept && was_kept {
                    e.call(0x008a_cc80, &args![this]);
                    e.mem.set_u8(this.addr() + 0x118, 0);
                }
            }
        }
        if flags & 0x80000 != 0 {
            e.call(0x0085_79e0, &args![owner, count_word, 2u32]);
            let count = e.mem.u16(count_word) as u32;
            let mut index = 0u32;
            while index < count {
                let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
                e.mem.set_u32(block_slot, block);
                e.call(LOAD_NUMERIC_ID, &args![this, id, 4u32]);
                let loaded_id = e.mem.u32(id);
                e.mem.set_u32(block + 4, loaded_id);
                e.call(LOAD_GAME_DATA_OLD, &args![this, block, 4u32]);
                e.call(0x005a_e3d0, &args![this.addr() + 0xfc, block_slot]);
                index += 1;
            }
        }
        // The `flags & 0` block that would load a form id never ran.
        if save_version(e) >= 0x14 {
            e.call(0x0085_79e0, &args![owner, count_word, 2u32]);
            let count = e.mem.u16(count_word) as u32;
            let mut index = 0u32;
            while index < count {
                let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
                e.mem.set_u32(block_slot, block);
                e.call(LOAD_NUMERIC_ID, &args![this, id, 4u32]);
                let loaded_id = e.mem.u32(id);
                e.mem.set_u32(block, loaded_id);
                e.call(LOAD_GAME_DATA_OLD, &args![this, block + 4, 4u32]);
                e.call(0x005a_e3d0, &args![this.addr() + 0xf4, block_slot]);
                index += 1;
            }
        }
        if save_version(e) >= 0x32 {
            e.call(LOAD_NUMERIC_ID, &args![this, id, 4u32]);
            let loaded_id = e.mem.u32(id);
            e.mem.set_u32(this.addr() + 0xc0, loaded_id);
        }
        if save_version(e) >= 0x3c {
            e.call(LOAD_NUMERIC_ID, &args![this, id, 4u32]);
            let loaded_id = e.mem.u32(id);
            let form = if loaded_id == 0 {
                0
            } else {
                lookup_and_cast(e, loaded_id, CAST_TO_011846E8)
            };
            e.mem.set_u32(this.addr() + 0x148, form);
        }
        if save_version(e) >= 0x44 && flags & 0x80_0000 != 0 {
            e.call(0x0093_79c0, &args![this.addr() + 0xd0]);
        }
        if save_version(e) >= 0x45 {
            e.call(0x0085_79e0, &args![owner, this.addr() + 0xc4, 1u32]);
            e.call(0x0085_7aa0, &args![owner, id, 4u32]);
        }
        if save_version(e) >= 0x61 {
            e.call(0x0085_7aa0, &args![owner, id, 4u32]);
            let loaded_id = e.mem.u32(id);
            e.mem.set_u32(this.addr() + 0x70, loaded_id);
        }
        if save_version(e) >= 0x65 {
            e.call(LOAD_GAME_DATA_OLD, &args![this, this.addr() + 0xc8, 4u32]);
        }
        if save_version(e) >= 0x71 {
            e.call(LOAD_GAME_DATA_OLD, &args![this, this.addr() + 0x126, 1u32]);
            e.call(LOAD_GAME_DATA_OLD, &args![this, this.addr() + 0x14c, 1u32]);
            e.call(LOAD_GAME_DATA_OLD, &args![this, this.addr() + 0x158, 4u32]);
            e.call(LOAD_GAME_DATA_OLD, &args![this, this.addr() + 0x74, 4u32]);
        }
        if save_version(e) >= 0x73 {
            e.call(LOAD_GAME_DATA_OLD, &args![this, this.addr() + 0x174, 1u32]);
        }
        if save_version(e) >= 0x7b {
            e.call(LOAD_GAME_DATA_OLD, &args![this, this.addr() + 0x118, 1u32]);
        }
        if uses_save_blocks(e) {
            let now = save_position(e);
            let current = e.call(0x004f_d3c0, &args![owner]).u32();
            let expected = (e.mem.u16(size_word) as u32).wrapping_add(base);
            let namer = if current != 0 {
                let form_id = e.mem.u32(current);
                e.call(LOOKUP_FORM, &args![form_id]).u32()
            } else {
                0
            };
            let (fmt_over_form, fmt_over, fmt_under_form, fmt_under) = (
                0x0101_5588u32,
                0x0101_54a0u32,
                0x0101_5500u32,
                0x0101_5440u32,
            );
            if now != expected {
                let over = now > expected;
                let diff = if over {
                    now.wrapping_sub(expected)
                } else {
                    expected.wrapping_sub(now)
                };
                if current != 0 {
                    let flags_word = e.mem.u32(current + 5);
                    let version = e.mem.u8(current + 9) as u32;
                    let name = e.vcall(namer, 0x130, &args![]).u32();
                    let form_id = e.mem.u32(current);
                    let fmt = if over { fmt_over_form } else { fmt_under_form };
                    e.call(
                        SAVELOAD_ERROR,
                        &args![
                            fmt,
                            diff,
                            ACTOR_SOURCE_FILE,
                            0x4dffu32,
                            form_id,
                            name,
                            version,
                            flags_word
                        ],
                    );
                } else {
                    let version = save_version(e);
                    let fmt = if over { fmt_over } else { fmt_under };
                    e.call(
                        SAVELOAD_ERROR,
                        &args![fmt, diff, ACTOR_SOURCE_FILE, 0x4dffu32, version],
                    );
                }
            }
        }
    });
}

/// The part of the animation clean-up `fn_008aa210` and `fn_008aa9a0` share:
/// when `anim` exists and `00496940` finds something in it, the object
/// (`00537bd0` of that, then method `+0x8c` with `00499b70`, then method
/// `+0xc` of the result) is handed to `004def90`.
fn animation_node(e: &mut Engine, anim: u32) -> u32 {
    let inner = e.call(0x0049_6940, &args![anim]).u32();
    let sink = e.call(0x0053_7bd0, &args![inner]).u32();
    let arg = e.call(0x0049_9b70, &args![]).u32();
    let object = e.vcall(sink, 0x8c, &args![arg]).u32();
    if object != 0 {
        e.vcall(object, 0xc, &args![]).u32()
    } else {
        0
    }
}

// Translated from 008aa210 (decompiled, FalloutNV.exe 1.4.0.525)
/// The actor's revert-game routine (the name is not in the Xbox PDB). When
/// `0055f5b0` of the save/load object holds, it hands the nodes of the
/// player's two animations (or the actor's) to `004def90` and destroys the
/// object at `this + 0x8c` (`0080fcc0(1)`). Then `MobileObject`'s
/// (`00932750`), emptying the list at `this + 0xfc` (with `flags & 0x80000`)
/// and `ModifierList::Revert` at `this + 0xd0` (with `flags & 0x800000`),
/// and, when `0055f5b0` holds, the actor's state is reset.
pub fn fn_008aa210(e: &mut Engine, this: Ptr<Actor>, flags: u32) {
    let owner = save_load_game(e);
    if e.call(0x0055_f5b0, &args![owner]).bool() {
        let player = e.global::<u32>(PLAYER_CHARACTER);
        let anim = if this.addr() == player {
            let first = e.call(PLAYER_GET_ANIMATION, &args![player, 1u32]).u32();
            if first != 0 && e.call(0x0049_6940, &args![first]).u32() != 0 {
                let node = animation_node(e, first);
                if node != 0 {
                    e.call(0x004d_ef90, &args![node]);
                }
            }
            e.call(PLAYER_GET_ANIMATION, &args![player, 0u32]).u32()
        } else {
            e.call(GET_ANIMATION, &args![this]).u32()
        };
        if anim != 0 && e.call(0x0049_6940, &args![anim]).u32() != 0 {
            let node = animation_node(e, anim);
            if node != 0 {
                e.call(0x004d_ef90, &args![node]);
            }
        }
        let object = e.mem.u32(this.addr() + 0x8c);
        if object != 0 {
            e.call(0x0080_fcc0, &args![object, 1u32]);
        }
        e.mem.set_u32(this.addr() + 0x8c, 0);
    }
    e.call(0x0093_2750, &args![this, flags]);
    if flags & 0x80000 != 0 {
        let node = this.addr() + 0xfc;
        while node != 0 && !e.call(NODE_IS_EMPTY, &args![node]).bool() {
            let slot = e.call(SELF, &args![node]).u32();
            let item = e.mem.u32(slot);
            e.call(0x0040_1030, &args![item]);
            e.call(0x0063_f7b0, &args![node]);
        }
    }
    if flags & 0x80_0000 != 0 {
        e.call(0x0093_7b30, &args![this.addr() + 0xd0]);
    }
    if e.call(0x0055_f5b0, &args![owner]).bool() {
        e.call(0x008a_5530, &args![this]);
        let face = e.call(0x008a_dcb0, &args![this]).u32();
        if face != 0 {
            e.vcall(face, 0xd8, &args![0u32, 1u32]);
        }
        e.call(0x008c_5090, &args![this]);
        e.mem.set_u8(this.addr() + 0xc4, 0);
        e.mem.set_u32(this.addr() + 0x11c, 0);
        e.mem.set_u8(this.addr() + 0x15c, 1);
        e.mem.set_u8(this.addr() + 0x174, 0);
        e.mem.set_u8(this.addr() + 0x14c, 0);
        e.mem.set_u8(this.addr() + 0x118, 0);
    }
}

/// Walks a list of entries at `list` (`this + 0xfc` or `this + 0xf4`) and
/// drops the entries for which `keep` answers false, as `fn_008aa4c0` does:
/// `00905330` (with the address of the entry in `slot`) removes the entry
/// that follows the previous kept node, or `0063f7b0` removes the head,
/// then the entry is freed (`00401030`).
fn drop_unresolved_entries(
    e: &mut Engine,
    list: u32,
    slot: u32,
    resolve: &mut dyn FnMut(&mut Engine, u32) -> bool,
) {
    let mut node = list;
    let mut previous = 0u32;
    while node != 0 && !e.call(NODE_IS_EMPTY, &args![node]).bool() {
        let cell = e.call(SELF, &args![node]).u32();
        let entry = e.mem.u32(cell);
        e.mem.set_u32(slot, entry);
        if !resolve(e, entry) {
            if previous == 0 {
                e.call(0x0063_f7b0, &args![node]);
            } else {
                e.call(0x0090_5330, &args![previous, slot]);
                node = e.call(NEXT_NODE, &args![previous]).u32();
            }
            let freed = e.mem.u32(slot);
            e.call(0x0040_1030, &args![freed]);
        } else {
            previous = node;
            node = e.call(NEXT_NODE, &args![node]).u32();
        }
    }
}

// Translated from 008aa4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The actor's init-load routine (`MobileObject::InitLoadGame` is
/// `00932490`; the name of this one is not in the Xbox PDB): after the
/// base routine, turns the saved form ids into pointers again. The list at
/// `this + 0xfc` (with `flags & 0x80000`) and, from save version 0x14, the
/// list at `this + 0xf4` lose the entries whose form no longer resolves; the
/// forms at `this + 0xc0` (from version 0x32) and `this + 0x70` (from 0x61)
/// are resolved too (`004839c0` and a `__RTDynamicCast`). Then the actor is
/// put back into its cell (method `+0x240`, or `TESObjectCELL::AddReference`
/// at the cell of its position) and, when it has no process and is neither
/// of the two kinds `00440d80` and `00440da0` test, gets a new process
/// (`00906dc0`) and the process level `009334b0` asks for.
///
/// C++ exception frames are not translated.
pub fn fn_008aa4c0(e: &mut Engine, this: Ptr<Actor>, flags: u32, arg: u32) {
    e.call(0x0093_2490, &args![this, flags, arg]);
    e.with_stack(0x10, |e, slot| {
        if flags & 0x80000 != 0 {
            drop_unresolved_entries(e, this.addr() + 0xfc, slot.addr(), &mut |e, entry| {
                let form = e.mem.u32(entry + 4);
                let resolved = lookup_and_cast(e, form, CAST_TO_011846D4);
                e.mem.set_u32(entry + 4, resolved);
                resolved != 0
            });
        }
        if save_version(e) >= 0x14 {
            drop_unresolved_entries(e, this.addr() + 0xf4, slot.addr(), &mut |e, entry| {
                let form = e.mem.u32(entry);
                if form != 0 {
                    let resolved = lookup_and_cast(e, form, CAST_TO_01183060);
                    e.mem.set_u32(entry, resolved);
                }
                e.mem.u32(entry) != 0
            });
        }
    });
    if save_version(e) >= 0x32 {
        let form = e.mem.u32(this.addr() + 0xc0);
        let resolved = if form != 0 {
            lookup_and_cast(e, form, CAST_TO_011846D4)
        } else {
            0
        };
        e.mem.set_u32(this.addr() + 0xc0, resolved);
    }
    save_version(e);
    if save_version(e) >= 0x61 {
        let form = e.mem.u32(this.addr() + 0x70);
        let resolved = if form != 0 {
            lookup_and_cast(e, form, CAST_TO_01184920)
        } else {
            0
        };
        e.mem.set_u32(this.addr() + 0x70, resolved);
    }
    let proc = process(e, this);
    if proc.addr() != 0 && e.call(0x0045_cd60, &args![proc]).u32() != 0 {
        let cell = e.call(GET_PARENT_CELL, &args![this]).u32();
        if cell != 0 {
            let cell = e.call(GET_PARENT_CELL, &args![this]).u32();
            if e.call(0x0045_0ff0, &args![cell]).bool() {
                e.vcall(this.addr(), 0x240, &args![]);
            }
        } else if e.call(0x0056_53d0, &args![this]).bool() {
            let world = e.call(0x0057_5d70, &args![this]).u32();
            if world != 0 {
                let position = e.vcall(this.addr(), 0x1f4, &args![]).u32();
                let x = e.mem.f32(position);
                let y = e.mem.f32(position + 4);
                let cell_x = e.call(0x0040_6d90, &args![x]).i32() >> 12;
                let cell_y = e.call(0x0040_6d90, &args![y]).i32() >> 12;
                let handler: u32 = e.global(DATA_HANDLER);
                let target = e
                    .call(
                        0x0046_1c20,
                        &args![handler, cell_x as u32, cell_y as u32, world, 0u32],
                    )
                    .u32();
                if target != 0 && e.call(0x0045_0ff0, &args![target]).bool() {
                    e.vcall(this.addr(), 0x240, &args![]);
                    let owner = save_load_game(e);
                    let previous = e.call(0x0047_c850, &args![owner]).u8();
                    e.call(0x0045_34f0, &args![owner, 0u32]);
                    e.call(0x0054_8230, &args![target, this, 0u32]);
                    e.call(0x0045_34f0, &args![owner, previous as u32]);
                }
            }
        }
    }
    if acquire_object(e, this).addr() == 0
        && !e.call(0x0044_0d80, &args![this]).bool()
        && !e.call(0x0044_0da0, &args![this]).bool()
    {
        let block = e.call(OPERATOR_NEW, &args![0xb4u32]).u32();
        let process = if block != 0 {
            e.call(0x0090_6dc0, &args![block]).u32()
        } else {
            0
        };
        e.mem.set_u32(this.addr() + 0x68, process);
        match e.call(0x0093_34b0, &args![this]).u32() {
            0 => {
                e.vcall(this.addr(), 0x240, &args![]);
            }
            1 => {
                e.vcall(this.addr(), 0x24c, &args![]);
            }
            2 => {
                e.vcall(this.addr(), 0x248, &args![]);
            }
            _ => {}
        }
    }
}

// Translated from 008aa9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The actor's finish-init-load routine (`MobileObject::FinishInitLoadGame`
/// is `00932520`; the name of this one is not in the Xbox PDB): tells the
/// process (method `+0x464`), runs the base routine, then, when the object
/// at `this + 0x88` (method `+0x34`) names an effect with a loaded model,
/// attaches a clone of it to the animation's node (`MagicProjectile::
/// Clone3D`, method `+0xdc`) and sets up a light for it (`0081bf80`,
/// `006ecd40`). Updates the alpha, the actor's speed method `+0x384` when
/// the process's `+0x5b4` is positive, the face animation data for dead or
/// talking actors, and re-initialises the animation when its group 3 is not
/// loaded.
///
/// C++ exception frames are not translated.
pub fn fn_008aa9a0(e: &mut Engine, this: Ptr<Actor>, first: u32, second: u32) {
    if acquire_object(e, this).addr() != 0 {
        let proc = acquire_object(e, this);
        e.vcall(proc.addr(), 0x464, &args![this]);
    }
    e.call(0x0093_2520, &args![this, first, second]);
    let holder = this.addr() + 0x88;
    if e.vcall(holder, 0x34, &args![]).u32() != 0 && e.call(0x0043_fcd0, &args![this]).u32() != 0 {
        let held = e.vcall(holder, 0x34, &args![]).u32();
        let effect = e.call(0x0040_a300, &args![held, 0u32]).u32();
        if effect != 0 && e.call(0x0048_cee0, &args![effect + 0x18]).u32() != 0 {
            let clone = e.call(0x0081_e440, &args![effect]).u32();
            let player = e.global::<u32>(PLAYER_CHARACTER);
            let anim = if this.addr() == player {
                if e.call(0x004e_af60, &args![player]).bool() {
                    e.call(PLAYER_GET_ANIMATION, &args![player, 0u32]).u32()
                } else {
                    e.call(PLAYER_GET_ANIMATION, &args![player, 1u32]).u32()
                }
            } else {
                e.call(GET_ANIMATION, &args![this]).u32()
            };
            if anim != 0 && e.call(0x0049_6940, &args![anim]).u32() != 0 {
                let inner = e.call(0x0049_6940, &args![anim]).u32();
                let sink = e.call(0x0053_7bd0, &args![inner]).u32();
                let arg = e.call(0x0049_9b70, &args![]).u32();
                let object = e.vcall(sink, 0x8c, &args![arg]).u32();
                let node = if object != 0 {
                    e.vcall(object, 0xc, &args![]).u32()
                } else {
                    0
                };
                if clone != 0 && node != 0 {
                    e.vcall(node, 0xdc, &args![clone, 1u32]);
                    if e.call(0x004f_d380, &args![effect]).u32() != 0
                        && !e.call(0x0052_5420, &args![]).bool()
                    {
                        let block = e.call(OPERATOR_NEW, &args![0x1cu32]).u32();
                        let light = if block != 0 {
                            let world = e.call(0x004f_d380, &args![effect]).u32();
                            let id = e.call(FORM_ID_OF, &args![world]).u32();
                            e.call(0x0081_bf80, &args![block, id, object]).u32()
                        } else {
                            0
                        };
                        e.call(0x006e_cd40, &args![holder, light]);
                    }
                }
            }
        }
    }
    e.call(0x008c_4640, &args![this]);
    let proc = process(e, this);
    if proc.addr() != 0 {
        let strength = e.vcall(proc.addr(), 0x5b4, &args![]).f32();
        let zero: f64 = e.global(ZERO_DOUBLE);
        if f64::from(strength) > zero {
            e.vcall(this.addr(), 0x384, &args![1u32, strength]);
        }
    }
    let mode = e.vcall(this.addr(), 0x214, &args![]).u32();
    if mode == 9
        || e.call(0x0043_7bd0, &args![this]).bool()
        || e.vcall(this.addr(), 0x22c, &args![0u32]).bool()
    {
        let face = e.call(0x008a_dcb0, &args![this]).u32();
        if face != 0 {
            e.vcall(face, 0xd8, &args![1u32, 1u32]);
        }
    }
    e.vcall(this.addr(), 0x3f4, &args![]);
    let player = e.global::<u32>(PLAYER_CHARACTER);
    if this.addr() != player
        && e.call(0x004f_8960, &args![this]).u32() != 2
        && e.call(0x004f_8960, &args![this]).u32() != 1
    {
        let anim = e.call(GET_ANIMATION, &args![this]).u32();
        if anim != 0 && !e.call(0x0049_4710, &args![anim, 3u32]).bool() {
            e.call(0x0056_59f0, &args![this]);
        }
    }
    let proc = process(e, this);
    if proc.addr() != 0 {
        e.vcall(proc.addr(), 0x7a8, &args![this]);
    }
}

// Translated from 008aad40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the actor keeps its state across the load (the name is not in the
/// Xbox PDB): false without a model (`0043fcd0`). With `flags & 0x400`, true
/// for state 1, 2 or 6 (`004f8960`); for state 4 also when a list entry of
/// the object at `this + 0x94` (method `+8`) casts to the type `011a146c`
/// with `0068a810` below `0x1e`. Otherwise true when the process's method
/// `+0x40c` answers 1, 3, 2, 4 or 5.
pub fn fn_008aad40(e: &mut Engine, this: Ptr<Actor>, flags: u32) -> bool {
    if e.call(0x0043_fcd0, &args![this]).u32() == 0 {
        return false;
    }
    if flags & 0x400 != 0 {
        if e.call(0x004f_8960, &args![this]).i32() == 1
            || e.call(0x004f_8960, &args![this]).i32() == 2
            || e.call(0x004f_8960, &args![this]).i32() == 6
        {
            return true;
        }
        if e.call(0x004f_8960, &args![this]).i32() == 4 {
            let mut node = e.vcall(this.addr() + 0x94, 8, &args![]).u32();
            while node != 0 && !e.call(NODE_IS_EMPTY, &args![node]).bool() {
                let slot = e.call(SELF, &args![node]).u32();
                let item = e.mem.u32(slot);
                let cast = e
                    .call(
                        DYNAMIC_CAST,
                        &args![item, 0u32, CAST_FROM_011A0D1C, CAST_TO_011A146C, 0u32],
                    )
                    .u32();
                if cast != 0 && e.call(0x0068_a810, &args![cast]).i32() < 0x1e {
                    return true;
                }
                node = e.call(NEXT_NODE, &args![node]).u32();
            }
        }
    }
    if acquire_object(e, this).addr() == 0 {
        return false;
    }
    for wanted in [1, 3, 2, 4, 5] {
        let proc = acquire_object(e, this);
        if e.vcall(proc.addr(), 0x40c, &args![]).i32() == wanted {
            return true;
        }
    }
    false
}

// Translated from 008aaee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Hands `buffer` to `00562140`; then, when `buffer`'s flag word (`428110`)
/// has `0x10000000` and the actor's method `+0x1e4` is zero, to `005621f0`
/// with the same mask.
pub fn fn_008aaee0(e: &mut Engine, this: Ptr<Actor>, buffer: u32) {
    e.call(0x0056_2140, &args![this, buffer]);
    if buffer_flag_a(e, buffer, 0x1000_0000) && e.vcall(this.addr(), 0x1e4, &args![]).u32() == 0 {
        e.call(0x0056_21f0, &args![buffer, 0x1000_0000u32]);
    }
}

/// `(offset, size)` of the fields `Actor::SaveGame` writes and
/// `fn_008ab3a0` reads after the clock, in the order of the format.
const FORMAT_FIELDS: [(u32, u32); 23] = [
    (0x124, 1),
    (0x125, 1),
    (0xbc, 1),
    (0xc4, 1),
    (0xc8, 4),
    (0x7d, 1),
    (0x110, 4),
    (0x118, 1),
    (0x126, 1),
    (0x145, 1),
    (0x146, 1),
    (0x14c, 1),
    (0x14d, 1),
    (0x150, 4),
    (0x154, 4),
    (0x158, 4),
    (0x174, 1),
    (0x175, 1),
    (0x18d, 1),
    (0x1a4, 4),
    (0x1a8, 4),
    (0xf0, 1),
    (0xf1, 1),
];
/// The field of save version 8.
const FORMAT_FIELDS_V8: [(u32, u32); 1] = [(0x10c, 4)];
/// The fields of save version 9.
const FORMAT_FIELDS_V9: [(u32, u32); 5] =
    [(0x134, 1), (0x138, 4), (0x144, 1), (0x13c, 4), (0x140, 4)];
/// The field of save version 13.
const FORMAT_FIELD_V13: (u32, u32) = (0x120, 4);
/// The object `ProcessLists::GetSystemTimeClock` (`0096d490`) is called on.
const PROCESS_LISTS_OBJECT: u32 = 0x011e_0e80;
/// `BGSSaveGameBuffer` write of `(address, size, 0)` (`00865e50`).
const BUFFER_WRITE: u32 = 0x0086_5e50;
/// `BGSSaveGameBuffer::SaveFormID_ov2` (Xbox PDB): `(form, 0)`.
const BUFFER_SAVE_FORM_ID: u32 = 0x0086_5df0;
/// `BGSLoadGameBuffer` read of `(address, size)` (`00864980`).
const BUFFER_READ: u32 = 0x0086_4980;
/// `BGSLoadGameBuffer::LoadFormID_ov2` (Xbox PDB): `(address)`.
const BUFFER_LOAD_FORM_ID: u32 = 0x0086_48e0;

// Translated from 008aaf40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::SaveGame` (Xbox PDB): saves the actor into the `BGSSaveGameBuffer`
/// `buffer` (`MobileObject::SaveGame_ov2` first): the time since the last
/// clock stamp (`ProcessLists::GetSystemTimeClock - this[0x114]`) and the
/// fields of the format with `00865e50(buffer, address, size, 0)`, the three
/// form references (`00865df0`), the state byte (flag `0x400`), the list at
/// `this + 0xfc` as a counted block (flag `0x80000`), the two modifier
/// lists (flags `0x800000` and `0x400000`) and the actor mover
/// (`this + 0x190`, method `+0x28`).
pub fn actor_save_game(e: &mut Engine, this: Ptr<Actor>, buffer: u32) {
    e.call(0x0093_2880, &args![this, buffer]);
    let clock = e.call(0x0096_d490, &args![PROCESS_LISTS_OBJECT]).f32();
    let elapsed = (f64::from(clock) - f64::from(e.mem.f32(this.addr() + 0x114))) as f32;
    e.with_stack(0x10, |e, scratch| {
        e.mem.set_f32(scratch.addr(), elapsed);
        e.call(BUFFER_WRITE, &args![buffer, scratch, 4u32, 0u32]);
    });
    let late = FORMAT_FIELDS_V8
        .iter()
        .chain(FORMAT_FIELDS_V9.iter())
        .chain(std::iter::once(&FORMAT_FIELD_V13));
    for (offset, size) in FORMAT_FIELDS.iter().chain(late) {
        e.call(
            BUFFER_WRITE,
            &args![buffer, this.addr() + offset, *size, 0u32],
        );
    }
    for offset in [0xc0u32, 0x148, 0x70] {
        let form = e.mem.u32(this.addr() + offset);
        e.call(BUFFER_SAVE_FORM_ID, &args![buffer, form, 0u32]);
    }
    if buffer_flag_a(e, buffer, 0x400) {
        let state = e.call(0x004f_8960, &args![this]).u8();
        e.with_stack(4, |e, scratch| {
            e.mem.set_u8(scratch.addr(), state);
            e.call(BUFFER_WRITE, &args![buffer, scratch, 1u32, 0u32]);
        });
    }
    if buffer_flag_a(e, buffer, 0x80000) {
        let mut count = 0u32;
        let place = e.call(0x0086_5f20, &args![buffer]).u32();
        let mut node = this.addr() + 0xfc;
        while node != 0 {
            let slot = e.call(SELF, &args![node]).u32();
            let item = e.mem.u32(slot);
            if item != 0 {
                let form = e.mem.u32(item + 4);
                e.call(BUFFER_SAVE_FORM_ID, &args![buffer, form, 0u32]);
                e.call(BUFFER_WRITE, &args![buffer, item, 4u32, 0u32]);
                count += 1;
            }
            node = e.call(NEXT_NODE, &args![node]).u32();
        }
        e.call(0x0086_5ff0, &args![buffer, count, place]);
    }
    if buffer_flag_a(e, buffer, 0x80_0000) {
        e.call(0x0093_7b50, &args![this.addr() + 0xd0, buffer]);
    }
    if buffer_flag_a(e, buffer, 0x40_0000) {
        e.call(0x0093_7b50, &args![this.addr() + 0xe0, buffer]);
    }
    let mover = e.mem.u32(this.addr() + 0x190);
    e.vcall(mover, 0x28, &args![buffer]);
}

// Translated from 008ab3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The actor's `BGSLoadGameBuffer` load routine (the name is not in the Xbox
/// PDB; it reads what `Actor::SaveGame`, `008aaf40`, wrote): the state
/// (`004f8960`) is kept across `MobileObject::LoadGame_ov2` (`00932a00`),
/// the clock stamp is turned back into a time difference, the fields of the
/// format are read with `00864980(buffer, address, size)`, the version 8, 9
/// and 13 fields by the buffer's version (virtual method `+0`), the form
/// references with `008648e0` and a cast, the counted list at `this + 0xfc`,
/// the modifier lists, and the actor mover (`this + 0x190`, method `+0x2c`).
/// Afterwards, with a model and a process, the equipment is compared with
/// the inventory: when a slot's item differs the process is told (method
/// `+0x468(1)`); an actor whose name contains `"Lily"` is checked for its
/// slot 5 item and weapon.
pub fn fn_008ab3a0(e: &mut Engine, this: Ptr<Actor>, buffer: u32) {
    let saved_state = e.call(0x004f_8960, &args![this]).u32();
    e.call(0x0093_2a00, &args![this, buffer]);
    e.mem.set_u32(this.addr() + 0x108, saved_state);
    e.call(0x0088_4f80, &args![this]);
    e.call(BUFFER_READ, &args![buffer, this.addr() + 0x114, 4u32]);
    let clock = e.call(0x0096_d490, &args![PROCESS_LISTS_OBJECT]).f32();
    let stamp = e.mem.f32(this.addr() + 0x114);
    e.mem.set_f32(
        this.addr() + 0x114,
        (f64::from(clock) - f64::from(stamp)) as f32,
    );
    for (offset, size) in FORMAT_FIELDS {
        e.call(BUFFER_READ, &args![buffer, this.addr() + offset, size]);
    }
    let version = |e: &mut Engine| e.vcall(buffer, 0, &args![]).u8() as u32;
    if version(e) >= 8 {
        for (offset, size) in FORMAT_FIELDS_V8 {
            e.call(BUFFER_READ, &args![buffer, this.addr() + offset, size]);
        }
    }
    if version(e) >= 9 {
        for (offset, size) in FORMAT_FIELDS_V9 {
            e.call(BUFFER_READ, &args![buffer, this.addr() + offset, size]);
        }
        e.call(0x008c_1ac0, &args![this, 0u32]);
        e.call(0x008c_1b50, &args![this, 0u32]);
    }
    if version(e) >= 0xd {
        let (offset, size) = FORMAT_FIELD_V13;
        e.call(BUFFER_READ, &args![buffer, this.addr() + offset, size]);
    }
    e.call(BUFFER_LOAD_FORM_ID, &args![buffer, this.addr() + 0xc0]);
    let id = e.call(0x0086_48a0, &args![buffer]).u32();
    let form = lookup_and_cast(e, id, CAST_TO_011846E8);
    e.mem.set_u32(this.addr() + 0x148, form);
    e.call(BUFFER_LOAD_FORM_ID, &args![buffer, this.addr() + 0x70]);
    if buffer_flag_a(e, buffer, 0x400) {
        let state = e.with_stack(4, |e, scratch| {
            e.mem.set_u8(scratch.addr(), 0);
            e.call(BUFFER_READ, &args![buffer, scratch, 1u32]);
            e.mem.u8(scratch.addr())
        });
        e.mem.set_u32(this.addr() + 0x108, state as u32);
    }
    if buffer_flag_a(e, buffer, 0x80000) {
        let count = e.call(0x0086_4a60, &args![buffer]).u32();
        e.with_stack(4, |e, slot| {
            let mut index = 0u32;
            while index < count {
                let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
                e.mem.set_u32(slot.addr(), block);
                e.call(BUFFER_LOAD_FORM_ID, &args![buffer, block + 4]);
                e.call(BUFFER_READ, &args![buffer, block, 4u32]);
                e.call(0x005a_e3d0, &args![this.addr() + 0xfc, slot]);
                index += 1;
            }
        });
    }
    if buffer_flag_a(e, buffer, 0x80_0000) {
        e.call(0x0093_7c60, &args![this.addr() + 0xd0, buffer]);
    }
    if buffer_flag_a(e, buffer, 0x40_0000) {
        e.call(0x0093_7c60, &args![this.addr() + 0xe0, buffer]);
    }
    let mover = e.mem.u32(this.addr() + 0x190);
    e.vcall(mover, 0x2c, &args![buffer]);
    if e.call(0x0043_fcd0, &args![this]).u32() != 0 && e.mem.u32(this.addr() + 0x68) != 0 {
        let equipment = e.vcall(this.addr(), 0x1e8, &args![]).u32();
        let changes = e.call(GET_INVENTORY_CHANGES, &args![this]).u32();
        if equipment != 0 && changes != 0 {
            let mut index = 0u32;
            while index < 0x13 {
                let mut slot_item = e.call(0x0088_e0f0, &args![equipment, index]).u32();
                let worn = e.call(0x004c_8c10, &args![changes, index, 0u32]).u32();
                let worn_form = if worn != 0 {
                    e.call(WORD_AT_8, &args![worn]).u32()
                } else {
                    0
                };
                if slot_item != 0
                    && (e.call(0x0040_1170, &args![slot_item]).i32() == 0xc
                        || e.vcall(slot_item, 0xf4, &args![]).bool())
                {
                    slot_item = 0;
                }
                if slot_item != worn_form {
                    let proc = process(e, this);
                    e.vcall(proc.addr(), 0x468, &args![1u32]);
                    break;
                }
                if worn != 0 {
                    e.call(0x0044_59e0, &args![worn, 1u32]);
                }
                index += 1;
            }
        } else if changes != 0
            && e.vcall(this.addr(), 0x21c, &args![]).bool()
            && e.call(0x0056_6950, &args![this]).bool()
        {
            let name = e.call(0x0055_d520, &args![this]).u32();
            if e.call(0x00ec_7750, &args![name, NAME_FRAGMENT]).u32() != 0 {
                let worn = e.call(0x004c_8c10, &args![changes, 5u32, 0u32]).u32();
                let worn_form = if worn != 0 {
                    e.call(WORD_AT_8, &args![worn]).u32()
                } else {
                    0
                };
                let proc = process(e, this);
                if e.vcall(proc.addr(), 0x148, &args![]).u32() != 0 {
                    let proc = process(e, this);
                    let weapon = e.vcall(proc.addr(), 0x148, &args![]).u32();
                    if e.call(WORD_AT_8, &args![weapon]).u32() != worn_form {
                        let proc = process(e, this);
                        e.vcall(proc.addr(), 0x468, &args![1u32]);
                    }
                }
            }
        }
    }
    if !e.vcall(this.addr(), 0x22c, &args![0u32]).bool() && e.mem.u32(this.addr() + 0x10c) != 0 {
        e.mem.set_u32(this.addr() + 0x10c, 0);
    }
}

// Translated from 008abc40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The actor's finish-init-load routine for the `BGSLoadGameBuffer` format
/// (the name is not in the Xbox PDB): `00932b70(buffer)`, then the forms at
/// `this + 0xc0` and `this + 0x70` and (with flag `0x80000`) the list at
/// `this + 0xfc` are resolved again with a cast; the combat state follows
/// the package (type `0x12` sets `this[0x104]`, else `ClearInCombat`), the
/// actor mover (`this + 0x190`, method `+0x30`) runs, `fn_008abfa0` is
/// handed the player when `this[0x18d]` is set, the default worn items are
/// set up (`006047c0` or `005f9e00`) unless flag `0x8000020` and the
/// animation is re-initialised when its group 3 is not loaded.
pub fn fn_008abc40(e: &mut Engine, this: Ptr<Actor>, buffer: u32) {
    e.call(0x0093_2b70, &args![this, buffer]);
    for (offset, target) in [(0xc0u32, CAST_TO_011846D4), (0x70, CAST_TO_01184920)] {
        let form = e.mem.u32(this.addr() + offset);
        let resolved = if form != 0 {
            lookup_and_cast(e, form, target)
        } else {
            0
        };
        e.mem.set_u32(this.addr() + offset, resolved);
    }
    if buffer_flag_a(e, buffer, 0x80000) {
        let mut node = this.addr() + 0xfc;
        while node != 0 {
            let slot = e.call(SELF, &args![node]).u32();
            let item = e.mem.u32(slot);
            if item != 0 {
                let form = e.mem.u32(item + 4);
                let resolved = if form != 0 {
                    lookup_and_cast(e, form, CAST_TO_011846D4)
                } else {
                    0
                };
                e.mem.set_u32(item + 4, resolved);
            }
            node = e.call(NEXT_NODE, &args![node]).u32();
        }
    }
    let proc = process(e, this);
    let package = if proc.addr() != 0 {
        e.vcall(proc.addr(), 0x22c, &args![]).u32()
    } else {
        0
    };
    if package != 0 && e.call(FORM_TYPE, &args![package]).i32() == 0x12 {
        e.mem.set_u8(this.addr() + 0x104, 1);
    } else {
        e.call(CLEAR_IN_COMBAT, &args![this, 0u32]);
    }
    let mover = e.mem.u32(this.addr() + 0x190);
    e.vcall(mover, 0x30, &args![buffer]);
    if e.mem.u8(this.addr() + 0x18d) != 0 {
        let player = e.global::<u32>(PLAYER_CHARACTER);
        fn_008abfa0(e, Ptr::new(player), this.addr());
    }
    if e.call(0x0042_ce90, &args![buffer]).bool()
        && !buffer_flag_a(e, buffer, 0x800_0020)
        && e.vcall(this.addr(), 0x1d0, &args![]).u32() != 0
    {
        if e.vcall(this.addr(), 0x21c, &args![]).bool() {
            let base = e.call(0x0041_81e0, &args![this]).u32();
            e.call(0x005f_9e00, &args![base, this, 1u32, 1u32, 1u32]);
        } else {
            let base = e.call(0x0041_81e0, &args![this]).u32();
            e.call(0x0060_47c0, &args![base, this, 1u32, 1u32, 0u32, 1u32]);
        }
    }
    if e.vcall(this.addr(), 0x1d0, &args![]).u32() != 0 {
        let player = e.global::<u32>(PLAYER_CHARACTER);
        if this.addr() != player
            && e.call(0x004f_8960, &args![this]).u32() != 2
            && e.call(0x004f_8960, &args![this]).u32() != 1
        {
            let anim = e.call(GET_ANIMATION, &args![this]).u32();
            if anim != 0 && !e.call(0x0049_4710, &args![anim, 3u32]).bool() {
                e.call(0x0088_7d00, &args![this]);
            }
        }
        e.call(0x008b_78c0, &args![this, 0u32]);
    }
    let proc = process(e, this);
    if proc.addr() != 0 && e.vcall(proc.addr(), 0x478, &args![]).bool() {
        e.vcall(proc.addr(), 0x464, &args![this]);
    }
    let ragdoll = e.mem.u32(this.addr() + 0xac);
    if ragdoll != 0 {
        e.call(0x00c7_c150, &args![ragdoll, 0u32]);
    }
    let controller = e.mem.u32(this.addr() + 0xb0);
    if controller != 0 {
        e.call(0x00ca_0cd0, &args![controller]);
    }
}

// Translated from 008abfa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds one to the counter at `this + 0xd68` and appends the address of the
/// argument (the stack word holding `value`) to the list at `this + 0x5fc`
/// (`005ae3d0`).
pub fn fn_008abfa0(e: &mut Engine, this: Ptr, value: u32) {
    let counter = e.mem.u32(this.addr() + 0xd68);
    e.mem.set_u32(this.addr() + 0xd68, counter.wrapping_add(1));
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), value);
        e.call(0x005a_e3d0, &args![this.addr() + 0x5fc, slot]);
    });
}

// Translated from 008abfe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The actor's last load step for the `BGSLoadGameBuffer` format (the name
/// is not in the Xbox PDB): `00932d60(buffer)`, the actor mover (method
/// `+0x34`), method `+0x2a0` when `+0x22c(1)` holds,
/// `Actor::UpdateActor3DPosition` and, when the actor has 3D (method
/// `+0x1d0`) and the buffer lacks flag `4`, the ragdoll is enabled for
/// dead or ragdolling actors (knock-down when it has no ragdoll data);
/// `Actor::UpdateAlpha`, the face animation data for dead or talking
/// actors, `Actor::ApplyCriticalStage` when `this[0x10c]` is set, and
/// `008c1470`, `Actor::UpdateDismemberedLimbVel`.
pub fn fn_008abfe0(e: &mut Engine, this: Ptr<Actor>, buffer: u32) {
    e.call(0x0093_2d60, &args![this, buffer]);
    let mover = e.mem.u32(this.addr() + 0x190);
    e.vcall(mover, 0x34, &args![buffer]);
    if e.vcall(this.addr(), 0x22c, &args![1u32]).bool() {
        e.vcall(this.addr(), 0x2a0, &args![]);
    }
    e.call(0x0088_b150, &args![this]);
    let node = e.vcall(this.addr(), 0x1d0, &args![]).u32();
    if node != 0 {
        if !buffer_flag_a(e, buffer, 4)
            && (e.vcall(this.addr(), 0x22c, &args![0u32]).bool()
                || e.vcall(this.addr(), 0x2e8, &args![]).bool())
        {
            let ragdoll = e.mem.u32(this.addr() + 0xac);
            if ragdoll != 0 {
                e.call(0x00c7_c150, &args![ragdoll, 1u32]);
            }
            let controller = e.mem.u32(this.addr() + 0xb0);
            if controller != 0 {
                e.with_stack(4, |e, out| {
                    let result = e.call(0x0093_1ed0, &args![this, out]).u32();
                    let value = e.call(0x004a_3a20, &args![result]).u32();
                    e.call(0x00ca_2ad0, &args![controller, node, value]);
                });
            }
            let list = e.call(GET_EXTRA_LIST, &args![this]).u32();
            if e.call(0x0041_d6d0, &args![list]).u32() == 0 {
                e.call(0x00c9_b670, &args![node, ZERO_POINT, 1u32, 0.0f32, 0u32]);
            }
        }
        e.with_stack(0x10, |e, local| {
            e.call(0x0043_d410, &args![local, 0.0f32, 0u32, 0u32]);
            e.call(0x00a5_9c60, &args![node, local]);
        });
    }
    e.call(0x008c_4640, &args![this]);
    let mode = e.vcall(this.addr(), 0x214, &args![]).u32();
    if mode == 9
        || e.call(0x0043_7bd0, &args![this]).bool()
        || e.vcall(this.addr(), 0x22c, &args![0u32]).bool()
    {
        let face = e.call(0x008a_dcb0, &args![this]).u32();
        if face != 0 {
            e.vcall(face, 0xd8, &args![1u32, 1u32]);
        }
    }
    if e.mem.u32(this.addr() + 0x10c) != 0 {
        e.call(0x008a_1a70, &args![this]);
    }
    e.call(0x008c_1470, &args![this]);
    e.call(0x008b_65f0, &args![this]);
}

// Translated from 008ac1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The actor's revert routine for the `BGSLoadGameBuffer` format (the name is
/// not in the Xbox PDB): removes the actor from all water, clears weapon and
/// dismembered limbs according to the buffer's flags, runs
/// `00932f60(buffer)`, resets the state fields to their defaults, destroys
/// the two objects at `this + 0x12c` and `this + 0x130` (virtual method `+0`
/// with `1`), clears the disposition modifiers, reverts the modifier lists
/// and the actor mover (method `+0x38`) and, with flag `0x400`, sets the
/// life state from the base form's health; ragdoll data is restored when the
/// actor should be a 3D ragdoll.
pub fn fn_008ac1e0(e: &mut Engine, this: Ptr<Actor>, buffer: u32) {
    e.call(0x0057_b520, &args![this, 1u32]);
    if buffer_flag_a(e, buffer, 0x800_0020) || buffer_flag_b(e, buffer, 0x800_0020) {
        e.call(0x0057_1b50, &args![this]);
        e.call(0x0045_34f0, &args![this, 1u32]);
        e.call(0x0048_3710, &args![this]);
        e.call(0x0048_3710, &args![this]);
    }
    if buffer_flag_b(e, buffer, 0x2_0000) && !buffer_flag_a(e, buffer, 0x2_0000) {
        e.call(0x008b_6820, &args![this]);
    }
    let ragdoll = e.mem.u32(this.addr() + 0xac);
    if ragdoll != 0 && e.call(0x0089_d690, &args![ragdoll]).bool() {
        e.call(0x00c7_b6a0, &args![ragdoll, 0u32]);
    }
    e.call(0x0093_2f60, &args![this, buffer]);
    e.call(0x008b_bdb0, &args![this]);
    e.call(0x008b_be00, &args![this]);
    e.call(CLEAR_IN_COMBAT, &args![this, 0u32]);
    e.call(0x0088_4f80, &args![this]);
    e.mem.set_f32(this.addr() + 0x114, 0.0);
    e.mem.set_u8(this.addr() + 0x124, 0);
    e.mem.set_u8(this.addr() + 0x125, 0);
    e.mem.set_u8(this.addr() + 0xbc, 1);
    e.mem.set_u8(this.addr() + 0xc4, 0);
    e.mem.set_f32(this.addr() + 0xc8, 0.0);
    e.mem.set_u8(this.addr() + 0x7d, 0);
    e.mem.set_u32(this.addr() + 0x110, 0xff);
    for offset in [0x118u32, 0x126, 0x145, 0x146, 0x14c, 0x14d] {
        e.mem.set_u8(this.addr() + offset, 0);
    }
    e.mem.set_u32(this.addr() + 0x150, 0);
    e.mem.set_f32(this.addr() + 0x154, 0.0);
    e.mem.set_f32(this.addr() + 0x158, 0.0);
    for offset in [0x174u32, 0x175, 0x18d, 0x18c, 0xf0, 0xf1] {
        e.mem.set_u8(this.addr() + offset, 0);
    }
    e.mem.set_u32(this.addr() + 0x1a4, 0);
    e.mem.set_u32(this.addr() + 0x1a8, 0);
    e.mem.set_u8(this.addr() + 0x127, 0);
    e.mem.set_u32(this.addr() + 0xc0, 0);
    e.mem.set_u32(this.addr() + 0x148, 0);
    e.mem.set_u32(this.addr() + 0x70, 0);
    e.mem.set_u8(this.addr() + 0x18e, 0);
    e.mem.set_u8(this.addr() + 0x127, 0);
    e.mem.set_u32(this.addr() + 0x128, 0);
    let object = e.mem.u32(this.addr() + 0x12c);
    if object != 0 {
        e.vcall(object, 0, &args![1u32]);
    }
    e.mem.set_u32(this.addr() + 0x12c, 0);
    let object = e.mem.u32(this.addr() + 0x130);
    if object != 0 {
        e.vcall(object, 0, &args![1u32]);
    }
    e.mem.set_u32(this.addr() + 0x130, 0);
    e.mem.set_u8(this.addr() + 0x134, 0);
    e.mem.set_u8(this.addr() + 0x144, 0);
    e.mem.set_u32(this.addr() + 0x138, 0);
    e.mem.set_u32(this.addr() + 0x13c, 0);
    e.mem.set_u32(this.addr() + 0x140, 0);
    e.call(0x008c_4400, &args![this]);
    if buffer_flag_b(e, buffer, 0x80000) {
        e.call(0x0087_fd20, &args![this]);
    }
    if buffer_flag_b(e, buffer, 0x80_0000) {
        e.call(0x0093_7d50, &args![this.addr() + 0xd0, buffer]);
    }
    if buffer_flag_b(e, buffer, 0x40_0000) {
        e.call(0x0093_7d50, &args![this.addr() + 0xe0, buffer]);
    }
    let mover = e.mem.u32(this.addr() + 0x190);
    e.vcall(mover, 0x38, &args![buffer]);
    if buffer_flag_b(e, buffer, 0x400) {
        if e.mem.u32(this.addr() + 0x108) != 0 {
            e.call(0x008a_cc80, &args![this]);
        }
        let base = e.call(0x0041_81e0, &args![this]).u32();
        let health = e.call(0x005f_0b00, &args![base]).i32();
        let zero: f64 = e.global(ZERO_DOUBLE);
        e.mem.set_u32(
            this.addr() + 0x108,
            if f64::from(health) <= zero { 2 } else { 0 },
        );
    }
    e.mem.set_u32(this.addr() + 0x10c, 0);
    if e.vcall(this.addr(), 0x1d0, &args![]).u32() != 0 {
        let node = e.vcall(this.addr(), 0x1d0, &args![]).u32();
        if e.call(0x0062_c3c0, &args![this, node]).bool() {
            e.call(0x0057_7330, &args![this, 0u32]);
        }
    }
}

// Translated from 008ac680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the actor's current package (process method `+0x22c`; for an
/// interrupt package, `00678610`, the package extra of the actor's extra
/// list instead) passes `008840d0`.
pub fn fn_008ac680(e: &mut Engine, this: Ptr<Actor>) -> bool {
    let proc = acquire_object(e, this);
    let mut package = e.vcall(proc.addr(), 0x22c, &args![]).u32();
    if package != 0 && e.call(0x0067_8610, &args![package]).bool() {
        let list = e.call(GET_EXTRA_LIST, &args![this]).u32();
        package = e.call(0x0041_cb10, &args![list]).u32();
    }
    package != 0 && e.call(0x0088_40d0, &args![package]).bool()
}

// Translated from 008ac6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::IsInCombatantFaction` (Xbox PDB): whether one of the factions of
/// the actor's base form (list at base `+0x30`) or of its extra faction
/// changes (extra data `0x5e`) that has not been expelled (`00437080`)
/// passes `0047d7e0` (flag `4` at `+0x34`).
pub fn actor_is_in_combatant_faction(e: &mut Engine, this: Ptr<Actor>) -> bool {
    let mut found = false;
    let base = e.call(0x0041_81e0, &args![this]).u32();
    let mut node = e.call(0x005d_8a70, &args![base + 0x30]).u32();
    let list = e.call(GET_EXTRA_LIST, &args![this]).u32();
    let changes = e.call(0x0042_e800, &args![list]).u32();
    let mut changes_node = 0;
    if changes != 0 {
        changes_node = e.mem.u32(changes + 0xc);
    }
    // One faction entry: counts when it is not expelled and passes the test.
    let qualifies = |e: &mut Engine, entry: u32| -> bool {
        let faction = e.mem.u32(entry);
        (changes == 0 || !e.call(0x0043_7080, &args![changes, faction]).bool())
            && e.call(0x0047_d7e0, &args![faction]).bool()
    };
    while node != 0 && !found {
        let cell = e.call(SELF, &args![node]).u32();
        let entry = e.mem.u32(cell);
        if entry != 0 && qualifies(e, entry) {
            found = true;
        }
        node = e.call(NEXT_NODE, &args![node]).u32();
    }
    while changes_node != 0 {
        let cell = e.call(SELF, &args![changes_node]).u32();
        if e.mem.u32(cell) == 0 || found {
            break;
        }
        let cell = e.call(SELF, &args![changes_node]).u32();
        let entry = e.mem.u32(cell);
        if entry != 0 && qualifies(e, entry) {
            found = true;
        }
        changes_node = e.call(NEXT_NODE, &args![changes_node]).u32();
    }
    found
}

/// A virtual method of the actor's process (`this + 0x68`, read afresh as the
/// code does); the process must exist.
fn process_vcall(e: &mut Engine, this: Ptr<Actor>, slot: u32, arguments: &[u32]) -> Ret {
    let proc = process(e, this);
    e.vcall(proc.addr(), slot, arguments)
}

// Translated from 008a89a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Uses up `amount` shots (`-1`: the process's method `+0x440`, a byte) of the
/// ammunition item change (process method `+0x14c`) for the weapon item
/// change (`+0x148`): the shots cost `00524b60(weapon form)` each, are at
/// most the count (`00726070`), and the rest is stored back (`006ecd40`).
/// When the actor should use ammo (`Actor::ShouldUseAmmo`) the actor's method
/// `+0x17c` removes the used shots. If nothing is left: the process's method
/// `+0x6b8` is cleared (`00560cf0(0.0)`); a weapon whose
/// `GetAmmoRegenRate` (`00709430` with the weapon's mod effect 6) is not
/// positive and for which the actor's method `+0x3ec` agrees takes the
/// remaining count of the ammunition item change; otherwise, when both item
/// changes name the same form, the biped weapon is removed (the player's two
/// bipeds, or the actor's method `+0x1e8`) and the process drops its weapon
/// (`+0x160(0, 0, 0)`, `fn_008a6840(0)`). Without shots left the ammunition
/// is unequipped (`Actor::QueueUnEquipObject`) and the combat controller
/// (`+0x428`) and a follower bark (`FollowerBarks::TriggerFollowerBark`, 7)
/// are told. Returns the count left.
pub fn fn_008a89a0(e: &mut Engine, this: Ptr<Actor>, amount: u32) -> u32 {
    if process(e, this).addr() == 0
        || process_vcall(e, this, 0x148, &[]).u32() == 0
        || process_vcall(e, this, 0x14c, &[]).u32() == 0
    {
        return 0;
    }
    let change = process_vcall(e, this, 0x14c, &[]).u32();
    let mut left = e.call(ITEM_COUNT, &args![change]).u32();
    let held = process_vcall(e, this, 0x148, &[]).u32();
    let weapon = e.call(WORD_AT_8, &args![held]).u32();
    let change = process_vcall(e, this, 0x14c, &[]).u32();
    let ammo = e.call(WORD_AT_8, &args![change]).u32();
    let mut used = if amount == u32::MAX {
        process_vcall(e, this, 0x440, &[]).u8() as u32
    } else {
        amount
    };
    if weapon != 0 {
        let per_shot = e.call(0x0052_4b60, &args![weapon]).u8() as u32;
        used = per_shot.wrapping_mul(used);
    }
    if used > left {
        used = left;
    }
    left = left.wrapping_sub(used);
    let change = process_vcall(e, this, 0x14c, &[]).u32();
    e.call(0x006e_cd40, &args![change, left]);
    if actor_should_use_ammo(e, this, weapon) {
        let change = process_vcall(e, this, 0x14c, &[]).u32();
        let form = e.call(WORD_AT_8, &args![change]).u32();
        e.vcall(
            this.addr(),
            0x17c,
            &args![form, 0u32, used, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32],
        );
    }
    let mut regenerated = false;
    if left == 0 {
        let drain = process_vcall(e, this, 0x6b8, &[]).u32();
        if drain != 0 {
            e.call(0x0056_0cf0, &args![drain, 0.0f32]);
        }
        if process_vcall(e, this, 0x148, &[]).u32() != 0 {
            let held = process_vcall(e, this, 0x148, &[]).u32();
            let modifier = e.call(ITEM_CHANGE_HAS_MOD, &args![held, 6u32]).u8();
            let held = process_vcall(e, this, 0x148, &[]).u32();
            let form = e.call(WORD_AT_8, &args![held]).u32();
            let rate = e.call(0x0070_9430, &args![form, modifier as u32]).f32();
            let zero: f64 = e.global(ZERO_DOUBLE);
            if f64::from(rate) <= zero {
                let held = process_vcall(e, this, 0x148, &[]).u32();
                let second_modifier = e.call(ITEM_CHANGE_HAS_MOD, &args![held, 2u32]).u8();
                let drawn = e.call(0x008a_16d0, &args![this]).u8();
                let held = process_vcall(e, this, 0x148, &[]).u32();
                let form = e.call(WORD_AT_8, &args![held]).u32();
                if e.vcall(
                    this.addr(),
                    0x3ec,
                    &args![form, drawn as u32, second_modifier as u32, 0u32],
                )
                .bool()
                {
                    let change = process_vcall(e, this, 0x14c, &[]).u32();
                    left = e.call(ITEM_COUNT, &args![change]).u32();
                    regenerated = true;
                }
            }
        }
        if !regenerated
            && process_vcall(e, this, 0x148, &[]).u32() != 0
            && process_vcall(e, this, 0x14c, &[]).u32() != 0
        {
            let held = process_vcall(e, this, 0x148, &[]).u32();
            let weapon_form = e.call(WORD_AT_8, &args![held]).u32();
            let change = process_vcall(e, this, 0x14c, &[]).u32();
            let ammo_form = e.call(WORD_AT_8, &args![change]).u32();
            if weapon_form == ammo_form {
                let player = e.global::<u32>(PLAYER_CHARACTER);
                if this.addr() == player {
                    let biped = e.call(0x0095_0b00, &args![player, 1u32]).u32();
                    e.call(0x004a_b5b0, &args![biped]);
                    let biped = e.call(0x0095_0b00, &args![player, 0u32]).u32();
                    e.call(0x004a_b5b0, &args![biped]);
                } else if e.vcall(this.addr(), 0x1e8, &args![]).u32() != 0 {
                    let biped = e.vcall(this.addr(), 0x1e8, &args![]).u32();
                    e.call(0x004a_b5b0, &args![biped]);
                }
                process_vcall(e, this, 0x160, &args![0u32, 0u32, 0u32]);
                fn_008a6840(e, this, 0);
            }
        }
    }
    if left == 0 {
        e.call(
            0x0088_c790,
            &args![this, ammo, 1u32, 0u32, 0u32, 0u32, 1u32],
        );
        let controller = e.vcall(this.addr(), 0x428, &args![]).u32();
        if controller != 0 {
            e.call(0x0097_f6d0, &args![controller, 0u32]);
        }
        if e.call(0x0056_6950, &args![this]).bool() {
            e.call(0x008d_5cb0, &args![this, 7u32]);
        }
    }
    left
}

/// `0.75` (`double`): the part of the height difference `008ac8b0` returns.
const HEIGHT_FACTOR: u32 = 0x0101_de30;
/// `100.0` (`double`): what `008ac9d0` divides the item health by.
const ITEM_HEALTH_DIVISOR: u32 = 0x0101_7a40;
/// `30.0` (`double`).
const THIRTY: u32 = 0x0101_db88;
/// `0.99` (`float`): the exponent of the combat strength.
const COMBAT_STRENGTH_EXPONENT: u32 = 0x0106_682c;

// Translated from 008ac810 (decompiled, FalloutNV.exe 1.4.0.525)
/// The object returned by the actor's virtual method `+0x428`, passed to
/// `004fb070`; 0 without that object.
pub fn fn_008ac810(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    let object = e.vcall(this.addr(), 0x428, &args![]).u32();
    if object == 0 {
        0
    } else {
        e.call(0x004f_b070, &args![object]).u32()
    }
}

// Translated from 008ac850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` (`008ac890`) in the object returned by the actor's
/// virtual method `+0x428`, when there is one.
pub fn fn_008ac850(e: &mut Engine, this: Ptr<Actor>, value: u32) {
    let object = e.vcall(this.addr(), 0x428, &args![]).u32();
    if object != 0 {
        fn_008ac890(e, Ptr::new(object), value);
    }
}

// Translated from 008ac890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the word at `this + 0x80`.
pub fn fn_008ac890(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0x80, value);
}

// Translated from 008ac8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A method of the actor's secondary base at `+0xa8` (`this`; the actor is
/// `this - 0xa8`): the height of the actor. Without a character controller
/// (virtual `+0x390` is zero) it is `0.75` of the difference of the `z`
/// of the points returned by virtual methods `+0x1dc` and `+0x1d8`;
/// otherwise the player's `0070c3b0` times the actor's scale (`00567400`).
/// The result is also stored in the actor's process (`008ac990`) when the
/// process passes `00884920`.
pub fn fn_008ac8b0(e: &mut Engine, this: Ptr) -> f32 {
    let actor = this.addr() - 0xa8;
    let value: f32 = if e.vcall(actor, 0x390, &args![]).u32() == 0 {
        let (upper, lower) = e.with_stack(0x20, |e, buffer| {
            let upper = e.vcall(actor, 0x1dc, &args![buffer.addr()]).u32();
            let lower = e.vcall(actor, 0x1d8, &args![buffer.addr() + 0x10]).u32();
            (e.mem.f32(upper + 8), e.mem.f32(lower + 8))
        });
        let factor: f64 = e.global(HEIGHT_FACTOR);
        ((f64::from(upper) - f64::from(lower)) * factor) as f32
    } else {
        let player = e.global::<u32>(PLAYER_CHARACTER);
        let player_value = e.call(0x0070_c3b0, &args![player]).f64();
        let scale = e.call(0x0056_7400, &args![actor]).f64();
        (scale * player_value) as f32
    };
    let process = e.mem.u32(actor + 0x68);
    if process != 0 && e.call(0x0088_4920, &args![process]).bool() {
        fn_008ac990(e, Ptr::new(process), value);
    }
    value
}

// Translated from 008ac990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the float at `+0x2c` of the object at `this + 0x2c` and
/// marks it changed (`008848c0` with `0x200`), when that object exists.
pub fn fn_008ac990(e: &mut Engine, this: Ptr, value: f32) {
    let object = e.mem.u32(this.addr() + 0x2c);
    if object != 0 {
        e.mem.set_f32(object + 0x2c, value);
        e.call(0x0088_48c0, &args![object, 0x200u32]);
    }
}

// Translated from 008ac9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A method of the actor's secondary base at `+0xa8` (`this`; the actor is
/// `this - 0xa8`): `00645380` of the actor's worn weapon, with the health
/// of the item (`ItemChange::GetItemHealth` over `100`) as a float. The
/// item is the process's current one (virtual `+0x148`) when the process
/// passes `0045cd60 <= 1`, else the weapon slot `5` of the actor's
/// inventory changes (`004c8c10`), which is released again (`004459e0`).
/// The result is also stored in the actor's process (`008acb70`) when the
/// process passes `00884920`.
pub fn fn_008ac9d0(e: &mut Engine, this: Ptr) -> f32 {
    let actor = this.addr() - 0xa8;
    let mut item_word = 0u32;
    let mut health = 1.0f32;
    let item;
    let process = e.call(GET_PROCESS, &args![actor]).u32();
    let held_process = if process != 0 {
        let process = e.call(GET_PROCESS, &args![actor]).u32();
        e.call(0x0045_cd60, &args![process]).i32() < 2
    } else {
        false
    };
    let hundred: f64 = e.global(ITEM_HEALTH_DIVISOR);
    if held_process {
        let process = e.call(GET_PROCESS, &args![actor]).u32();
        item = e.vcall(process, 0x148, &args![]).u32();
        if item != 0 {
            item_word = e.call(0x0044_ddc0, &args![item]).u32();
            let item_health = e.call(0x004b_cdb0, &args![item, 1u32]).f64();
            health = (item_health / hundred) as f32;
        }
    } else {
        let changes = e.call(0x004b_f220, &args![actor]).u32();
        let worn = e.call(0x004c_8c10, &args![changes, 5u32, 0u32]).u32();
        if worn != 0 {
            item_word = e.call(0x0044_ddc0, &args![worn]).u32();
            let item_health = e.call(0x004b_cdb0, &args![worn, 1u32]).f64();
            health = (item_health / hundred) as f32;
            e.call(0x0044_59e0, &args![worn, 1u32]);
        }
        item = 0;
    }
    let base = if actor == 0 { 0 } else { this.addr() - 4 };
    let value = e
        .call(
            0x0064_5380,
            &args![
                base,
                item_word,
                health,
                0u32,
                item,
                0u32,
                0u32,
                0xffff_ffffu32,
                0.0f32,
                0.0f32,
                0u32,
                0u32,
                0u32
            ],
        )
        .f32();
    let process = e.mem.u32(actor + 0x68);
    if process != 0 && e.call(0x0088_4920, &args![process]).bool() {
        fn_008acb70(e, Ptr::new(process), value);
    }
    value
}

// Translated from 008acb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the float at `+0x10` of the object at `this + 0x2c` and
/// marks it changed (`008848c0` with `8`), when that object exists.
pub fn fn_008acb70(e: &mut Engine, this: Ptr, value: f32) {
    let object = e.mem.u32(this.addr() + 0x2c);
    if object != 0 {
        e.mem.set_f32(object + 0x10, value);
        e.call(0x0088_48c0, &args![object, 8u32]);
    }
}

// Translated from 008acba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Marks the actor's process (`00884940` with `8`) when it exists and passes
/// `00884920`.
pub fn fn_008acba0(e: &mut Engine, this: Ptr<Actor>) {
    let process = process(e, this);
    if process.addr() != 0 && e.call(0x0088_4920, &args![process]).bool() {
        e.call(0x0088_4940, &args![process, 8u32]);
    }
}

// Translated from 008acbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::CalculateCombatStrength` (Xbox PDB): `damage` (the actor's
/// `Actor::GetWeaponDamagePerSecond`, `008be600`, when it is negative) times
/// the actor value `0x10` (through the vtable at `this + 0xa4`, slot `+8`)
/// divided by `1 - pow(virtual(+0x440) / 30, 0.99)`; `0040ebd0` is that
/// power function.
pub fn actor_calculate_combat_strength(e: &mut Engine, this: Ptr<Actor>, damage: f32) -> f32 {
    let mut damage = damage;
    let zero: f64 = e.global(ZERO_DOUBLE);
    if f64::from(damage) < zero {
        damage = e.call(0x008b_e600, &args![this]).f32();
    }
    let value = e.vcall(this.addr() + 0xa4, 8, &args![0x10u32]).i32() as f32;
    let thirty: f64 = e.global(THIRTY);
    let ratio = (e.vcall(this.addr(), 0x440, &args![]).f64() / thirty) as f32;
    let exponent: f32 = e.global(COMBAT_STRENGTH_EXPONENT);
    let power = e.call(0x0040_ebd0, &args![ratio, exponent]).f32();
    let factor = (f64::from(value) / (1.0 - f64::from(power))) as f32;
    (f64::from(damage) * f64::from(factor)) as f32
}

// Translated from 008acc80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Refreshes a loaded actor after a life-state change. The object from
/// `0043fcd0` (none: nothing happens) gets `00c9bdf0(object, 1.0)`; the face
/// animation data (`008adcb0`) its virtual method `+0xd8(0, 0)`;
/// `Actor::ReloadTargets(1)`; a character controller (`009306d0`) in a
/// cell (`008d6f30`) gets `bhkCharacterController::SetWorld` with the
/// world of the cell (`004543c0`); the ragdoll controller (`+0xac`)
/// `EnableRagdollAnim`; the animation from virtual `+0x1e4` updates its
/// scene graph, and so does the player's animation (`00950a60`) when this
/// is the player. Last, by the critical stage (`+0x10c`): `1` passes the
/// node of virtual `+0x1d0` to `00c80700`; `2` calls virtual `+0x1c4`, does
/// that, then `00450f90(object, 0)`; `4` calls `+0x1c4` and `00450f90`.
pub fn fn_008acc80(e: &mut Engine, this: Ptr<Actor>) {
    let object = e.call(0x0043_fcd0, &args![this]).u32();
    if object == 0 {
        return;
    }
    e.call(0x00c9_bdf0, &args![object, 1.0f32]);
    let face = actor_get_face_animation_data(e, this);
    if face != 0 {
        e.vcall(face, 0xd8, &args![0u32, 0u32]);
    }
    e.call(0x008b_0b00, &args![this, 1u32]);
    let controller = e.call(0x0093_06d0, &args![this]).u32();
    if controller != 0 && e.call(GET_PARENT_CELL, &args![this]).u32() != 0 {
        let cell = e.call(GET_PARENT_CELL, &args![this]).u32();
        let world = e.call(0x0045_43c0, &args![cell]).u32();
        e.call(0x00c7_0c90, &args![controller, world]);
    }
    let ragdoll = e.get(this, Actor::pRagdollController);
    if ragdoll.addr() != 0 {
        e.call(0x00c7_a8d0, &args![ragdoll]);
    }
    let animation = e.vcall(this.addr(), 0x1e4, &args![]).u32();
    if animation != 0 {
        e.call(0x0049_3bd0, &args![animation]);
        let player = e.global::<u32>(PLAYER_CHARACTER);
        if this.addr() == player {
            let flag = e.call(0x004e_af60, &args![player]).u8();
            let player_animation = e.call(0x0095_0a60, &args![player, u32::from(flag)]).u32();
            e.call(0x0049_3bd0, &args![player_animation]);
        }
    }
    match e.get(this, Actor::eCriticalStage) {
        1 => {
            let node = e.vcall(this.addr(), 0x1d0, &args![]).u32();
            e.call(0x00c8_0700, &args![node]);
        }
        2 => {
            e.vcall(this.addr(), 0x1c4, &args![]);
            let node = e.vcall(this.addr(), 0x1d0, &args![]).u32();
            e.call(0x00c8_0700, &args![node]);
            e.call(0x0045_0f90, &args![object, 0u32]);
        }
        4 => {
            e.vcall(this.addr(), 0x1c4, &args![]);
            e.call(0x0045_0f90, &args![object, 0u32]);
        }
        _ => {}
    }
}

// Translated from 008ace10 (decompiled, FalloutNV.exe 1.4.0.525)
/// With `set`, `Actor::SetLifeState(3)`; without it, `SetLifeState(0)` when
/// `004f8960` says the state is `3`.
pub fn fn_008ace10(e: &mut Engine, this: Ptr<Actor>, set: bool) {
    if set {
        e.call(0x008a_1800, &args![this, 3u32]);
    } else if e.call(0x004f_8960, &args![this]).i32() == 3 {
        e.call(0x008a_1800, &args![this, 0u32]);
    }
}

// Translated from 008ace50 (decompiled, FalloutNV.exe 1.4.0.525)
/// With `set`, `Actor::SetLifeState(5)` and `00884c60`; without it,
/// `SetLifeState(0)` when `004f8960` says the state is `5`.
pub fn fn_008ace50(e: &mut Engine, this: Ptr<Actor>, set: bool) {
    if set {
        e.call(0x008a_1800, &args![this, 5u32]);
        e.call(0x0088_4c60, &args![this]);
    } else if e.call(0x004f_8960, &args![this]).i32() == 5 {
        e.call(0x008a_1800, &args![this, 0u32]);
    }
}

// Translated from 008ace90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the ghost flag `0x10000000` is set: in the process's flag word
/// (`008aced0`) when the actor has a process, else `00418a80` of the
/// actor's extra data list.
pub fn fn_008ace90(e: &mut Engine, this: Ptr<Actor>) -> u8 {
    let process = process(e, this);
    if process.addr() != 0 {
        u8::from(fn_008aced0(e, process, 0x1000_0000))
    } else {
        let list = e.call(GET_EXTRA_LIST, &args![this]).u32();
        e.call(0x0041_8a80, &args![list]).u8()
    }
}

// Translated from 008aced0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the flag word at `+0x44` of the object at `this + 0x2c` has a
/// bit of `mask`; false without that object.
pub fn fn_008aced0(e: &mut Engine, this: Ptr, mask: u32) -> bool {
    let object = e.mem.u32(this.addr() + 0x2c);
    object != 0 && e.mem.u32(object + 0x44) & mask != 0
}

// Translated from 008acf10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::SetGhost` (Xbox PDB): sets the ghost state of the actor's extra
/// data list (`ExtraDataList::SetGhost`), sets or clears the flag
/// `0x10000000` of the process's flag word (`008acf70`) when it has a
/// process, then calls virtual `+0x48` with `0x80000000`.
pub fn actor_set_ghost(e: &mut Engine, this: Ptr<Actor>, ghost: u8) {
    let list = e.call(GET_EXTRA_LIST, &args![this]).u32();
    e.call(0x0041_a960, &args![list, u32::from(ghost)]);
    let process = process(e, this);
    if process.addr() != 0 {
        fn_008acf70(e, process, 0x1000_0000, ghost);
    }
    e.vcall(this.addr(), 0x48, &args![0x8000_0000u32]);
}

// Translated from 008acf70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `008acfa0(mask, set)` on the object at `this + 0x2c`, when there is one.
pub fn fn_008acf70(e: &mut Engine, this: Ptr, mask: u32, set: u8) {
    let object = e.mem.u32(this.addr() + 0x2c);
    if object != 0 {
        fn_008acfa0(e, Ptr::new(object), mask, set);
    }
}

// Translated from 008acfa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`set` nonzero) or clears the bits of `mask` in the word at
/// `this + 0x44`.
pub fn fn_008acfa0(e: &mut Engine, this: Ptr, mask: u32, set: u8) {
    let flags = e.mem.u32(this.addr() + 0x44);
    let flags = if set != 0 {
        flags | mask
    } else {
        flags & !mask
    };
    e.mem.set_u32(this.addr() + 0x44, flags);
}

/// `100.0` (`float`), copied into the collision-sound record by `008acff0`.
const COLLISION_SOUND_LOUDNESS: u32 = 0x0101_6410;
/// `0.5` (`float`), copied into the collision-sound record by `008acff0`.
const COLLISION_SOUND_HALF: u32 = 0x0101_6248;
/// The `TES` singleton pointer (`011dea10`).
const TES_SINGLETON: u32 = 0x011d_ea10;
/// The `GameSetting` (`011cd65c`) `008ad1c0` reads as the spacing of the
/// moved actors.
const SETTING_FOLLOWER_SPACING: u32 = 0x011c_d65c;
/// The `GameSetting` (`011cda7c`) `008ad970` turns into the search radius
/// of the package target.
const SETTING_SEARCH_DISTANCE: u32 = 0x011c_da7c;
/// `3.1415927410125732` (`double`, `pi` as a `float`).
const PI_AS_FLOAT: u32 = 0x0101_ff40;
/// The `float` `008ad1c0` passes when the cell is not loaded (`FLT_MAX`).
const FLOAT_MAX: u32 = 0x0101_6970;
/// The string `"Waiting"` (`01071db4`).
const WAITING: u32 = 0x0107_1db4;

/// The item of the list node `node`: `006815c0` returns the node, whose
/// first word is the item.
fn list_item(e: &mut Engine, node: u32) -> u32 {
    let at = e.call(0x0068_15c0, &args![node]).u32();
    e.mem.u32(at)
}

// Translated from 008acff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Plays the collision sound of the actor hitting something. When `index`
/// is zero and `hit` has a 3D node (virtual `+0x1d0`), `index` and `kind`
/// come from the node's first collision object (`004b5260`, `006fa820`,
/// `004ae6a0`, `006205a0`). It fills a record (`006240d0`) of the
/// position words, `100.0`, `0.5`, the byte `0x24`, `kind` and `this +
/// index`, finds the id of the item in the process's current weapon slot
/// (virtual `+0x148`, `0044ddc0`), and when `hit` answers its virtual
/// `+0x100` calls `00837520` with `hit`; otherwise, unless `kind` is `6`,
/// with the actor itself, and plays `ImpactMixer::PlayCollisionSound`
/// (`00837550`) for the record.
#[allow(clippy::too_many_arguments)]
pub fn fn_008acff0(
    e: &mut Engine,
    this: Ptr<Actor>,
    position_x: u32,
    position_y: u32,
    position_z: u32,
    hit: Ptr,
    index: u32,
    kind: u32,
) {
    let mut index = index;
    let mut kind = kind;
    if index == 0 && hit.addr() != 0 && e.vcall(hit.addr(), 0x1d0, &args![]).u32() != 0 {
        let node = e.vcall(hit.addr(), 0x1d0, &args![]).u32();
        let object = e.call(0x004b_5260, &args![node]).u32();
        if object != 0 {
            index = e.call(0x006f_a820, &args![object]).u32();
            let shape = e.call(0x004a_e6a0, &args![index]).u32();
            kind = e.call(0x0062_05a0, &args![shape]).u32();
        }
    }
    e.with_stack(0x28, |e, record| {
        e.call(0x0062_40d0, &args![record]);
        let loudness: f32 = e.global(COLLISION_SOUND_LOUDNESS);
        let half: f32 = e.global(COLLISION_SOUND_HALF);
        let at = record.addr();
        e.mem.set_f32(at + 0x0c, loudness);
        e.mem.set_u32(at + 0x1c, 0);
        e.mem.set_u32(at + 0x20, index);
        e.mem.set_u8(at + 0x14, 0x24);
        e.mem.set_u8(at + 0x15, kind as u8);
        e.mem.set_u32(at, position_x);
        e.mem.set_u32(at + 4, position_y);
        e.mem.set_u32(at + 8, position_z);
        e.mem.set_f32(at + 0x10, half);
        e.mem.set_u32(at + 0x18, this.addr().wrapping_add(index));
        let process = e.call(GET_PROCESS, &args![this]).u32();
        let held = if process != 0 {
            let process = e.call(GET_PROCESS, &args![this]).u32();
            e.vcall(process, 0x148, &args![]).u32()
        } else {
            0
        };
        let item_id = if held == 0 {
            0
        } else {
            e.call(0x0044_ddc0, &args![held]).u32()
        };
        if hit.addr() != 0 && e.vcall(hit.addr(), 0x100, &args![]).bool() {
            let value = if item_id == 0 {
                0xffff_ffff
            } else {
                e.call(0x0044_6390, &args![item_id]).u32()
            };
            e.call(
                0x0083_7520,
                &args![
                    this,
                    0.0f32,
                    0.0f32,
                    hit,
                    value,
                    0xffff_ffffu32,
                    0xffff_ffffu32,
                    0u32,
                    0u32
                ],
            );
            return;
        }
        if kind != 6 {
            let value = if item_id == 0 {
                0xffff_ffff
            } else {
                e.call(0x0044_6390, &args![item_id]).u32()
            };
            e.call(
                0x0083_7520,
                &args![
                    this,
                    0.0f32,
                    0.0f32,
                    this,
                    value,
                    0xffff_ffffu32,
                    0xffff_ffffu32,
                    1u32,
                    0u32
                ],
            );
        }
        e.call(0x0083_7550, &args![record]);
    });
}

// Translated from 008ad1c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves the actor and the actors that follow it (the `ExtraDataList`
/// object `00422700` finds in each actor's extra data list, transitively;
/// only those with a process) to `position` in the cell `cell` of the
/// world space `world_space`, one after the other along the direction of
/// `angle`, each spaced by the radius of the actor (`Actor::GetRadius`)
/// plus the `011cd65c` setting when `spaced`. Actors that are unavailable
/// (virtual `+0x22c`, `00440da0`, `fn_008a6210`, the player's byte
/// `+0x204`) are skipped, as are visible ones (`Actor::IsVisible(7)`)
/// unless `forced` (or the player is sleeping or resting), the actor is the
/// leader or the player, or has no 3D. An actor whose process is waiting
/// (`008c1740("Waiting") == 1.0` with `GetForceNextUpdate`) is skipped. For
/// each actor: leaves the furniture it sits on (`SetMarkerUsed`, process
/// virtual `+0x4c0` and the heading of the marker); if it is not the leader
/// and the cell is loaded, asks the pathing for a reachable spot at the
/// distance (`PathingRequest`, `006d0900`, `006e7e70`); places the actor
/// (`008c7aa0` decides between two ways), sets its location
/// (`00575830`), marks its movement range (`005757d0`, `00575770`),
/// `TESObjectREFR::MoveRefToNewSpace(cell, world_space)`, and for the
/// followers stops them (`ForceStopMoving`, `StopCombat`,
/// `EndInterruptPackage`, process virtual `+0x28`).
#[allow(clippy::too_many_arguments)]
pub fn fn_008ad1c0(
    e: &mut Engine,
    this: Ptr<Actor>,
    position: Ptr,
    angle: f32,
    cell: u32,
    world_space: u32,
    spaced: bool,
    forced: bool,
) {
    let leader = this.addr();
    // The frame the game keeps on its stack: the position copy (+0), the
    // direction (+0x10), the list (+0x20), the leader word (+0x30) and a
    // reference word (+0x34), then the pathing objects (+0x40 ...).
    e.with_stack(0x220, |e, frame| {
        let base = frame.addr();
        let pos = base;
        let direction = base + 0x10;
        let list = base + 0x20;
        let leader_word = base + 0x30;
        let reference_word = base + 0x34;
        for i in 0..3 {
            let word = e.mem.u32(position.addr() + 4 * i);
            e.mem.set_u32(pos + 4 * i, word);
        }
        let sine = e.call(0x004e_4470, &args![angle]).f64();
        let first = (-sine) as f32;
        let cosine = e.call(0x004e_44b0, &args![angle]).f64();
        let second = (-cosine) as f32;
        e.call(0x0041_6870, &args![direction, second, first, 0.0f32]);
        let mut travelled = 0.0f32;
        e.call(0x0096_a2d0, &args![list]);
        e.mem.set_u32(leader_word, leader);
        e.call(0x005a_e3d0, &args![list, leader_word]);
        // The leader and, transitively, the actors that follow it.
        let mut node = list;
        while node != 0 {
            let item = list_item(e, node);
            let extra_list = e.call(GET_EXTRA_LIST, &args![item]).u32();
            let followers = e.call(0x0042_2700, &args![extra_list]).u32();
            if followers != 0 {
                let mut inner = e.mem.u32(followers + 0xc);
                while inner != 0 {
                    let follower = list_item(e, inner);
                    e.mem.set_u32(reference_word, follower);
                    if follower != 0
                        && e.call(GET_PROCESS, &args![follower]).u32() != 0
                        && !e.call(0x005f_65d0, &args![list, reference_word]).bool()
                    {
                        e.call(0x0090_5820, &args![node, reference_word]);
                    }
                    inner = e.call(0x0072_6070, &args![inner]).u32();
                }
            }
            node = e.call(0x0072_6070, &args![node]).u32();
        }
        let player = e.global::<u32>(PLAYER_CHARACTER);
        let sleeping = forced || e.call(0x0094_df60, &args![player]).bool();
        let spacing = if spaced {
            setting_float(e, SETTING_FOLLOWER_SPACING)
        } else {
            0.0
        };
        let mut node = list;
        while node != 0 {
            let actor = list_item(e, node);
            let next = |e: &mut Engine, node: u32| e.call(0x0072_6070, &args![node]).u32();
            let actor_ptr: Ptr<Actor> = Ptr::new(actor);
            if e.vcall(actor, 0x22c, &args![0u32]).bool()
                || e.call(0x0044_0da0, &args![actor]).bool()
                || fn_008a6210(e, actor_ptr)
            {
                node = next(e, node);
                continue;
            }
            let player = e.global::<u32>(PLAYER_CHARACTER);
            if e.mem.u8(player + 0x204) != 0 {
                node = next(e, node);
                continue;
            }
            if !sleeping
                && actor != leader
                && actor != e.global::<u32>(PLAYER_CHARACTER)
                && e.vcall(actor, 0x1d0, &args![]).u32() != 0
                && e.call(0x008b_0190, &args![actor, 7u32]).bool()
            {
                node = next(e, node);
                continue;
            }
            e.call(0x0068_15c0, &args![base + 0x38]);
            let process = e.call(GET_PROCESS, &args![actor]).u32();
            if process == 0 {
                node = next(e, node);
                continue;
            }
            if e.call(0x0056_6950, &args![actor]).bool() {
                let waiting = e.call(0x008c_1740, &args![actor, WAITING]).f64();
                let one: f64 = e.global(ONE);
                if waiting == one {
                    node = next(e, node);
                    continue;
                }
            }
            if e.vcall(process, 0x4bc, &args![]).u32() != 0 {
                // Gets off the furniture it uses.
                let marker_id = e.vcall(process, 0x4c4, &args![]).u32();
                let furniture = e.vcall(process, 0x4c8, &args![]).u32();
                e.call(0x0056_8020, &args![furniture, marker_id, 0u32]);
                let animation = e.call(GET_ANIMATION, &args![actor]).u32();
                e.call(0x0049_74a0, &args![animation]);
                let furniture = e.vcall(process, 0x4c8, &args![]).u32();
                e.vcall(process, 0x4c0, &args![actor, 0u32, furniture, 0x7fu32]);
                let marker = e.vcall(process, 0x4d4, &args![]).u32();
                let marker_index = e.call(0x0050_94b0, &args![marker]).u32();
                let furniture = e.vcall(process, 0x4c8, &args![]).u32();
                let form = e.call(GET_SAVE_FORM, &args![furniture]).u32();
                let heading = e.call(0x0050_99c0, &args![form, marker_index]).f64();
                e.call(0x0093_1d30, &args![actor, (-heading) as f32]);
                let turned = e.vcall(actor, 0x2bc, &args![0u32]).f64();
                let pi: f64 = e.global(PI_AS_FLOAT);
                e.vcall(actor, 0x2c4, &args![(turned + pi) as f32]);
                let marker = e.vcall(process, 0x4d4, &args![]).u32();
                e.call(0x00c5_4550, &args![marker, 0.0f32]);
                let marker = e.vcall(process, 0x4d4, &args![]).u32();
                e.call(0x0056_8ab0, &args![marker, 0u32]);
                let marker = e.vcall(process, 0x4d4, &args![]).u32();
                e.call(0x0098_ddd0, &args![marker, ZERO_POINT]);
                let marker = e.vcall(process, 0x4d4, &args![]).u32();
                e.vcall(process, 0x540, &args![0u32, 0x7fu32, marker]);
                e.call(0x0093_1fb0, &args![actor, 0u32]);
            }
            let radius = e.call(0x008b_e1b0, &args![actor]).f32();
            if actor != leader {
                travelled =
                    ((f64::from(spacing) + f64::from(radius)) + f64::from(travelled)) as f32;
                if cell != 0 && {
                    let tes = e.global::<u32>(TES_SINGLETON);
                    e.call(0x0045_11e0, &args![tes, cell, 0u32]).bool()
                } {
                    let out = base + 0x40;
                    let target = base + 0x50;
                    let solution = base + 0x60;
                    let request = base + 0xb0;
                    let location = base + 0x170;
                    let found_a = base + 0x1b0;
                    let found_b = base + 0x1c0;
                    let scaled = e.call(0x0045_bb20, &args![direction, out, travelled]).u32();
                    e.call(0x0043_9e90, &args![pos, target, scaled]);
                    e.call(0x006e_7650, &args![solution]);
                    e.call(0x006e_2420, &args![request]);
                    e.call(0x006e_29f0, &args![request, leader]);
                    e.call(0x006e_2b50, &args![request, 1u32]);
                    let made = e.call(0x006d_ce10, &args![location, target, leader]).u32();
                    e.call(0x006d_1bc0, &args![request, made]);
                    e.call(0x004f_f7e0, &args![location]);
                    fn_008ad8f0(e, Ptr::new(request), 1);
                    e.call(0x0068_15c0, &args![found_b]);
                    if e.call(0x006d_0900, &args![request, solution]).bool()
                        && e.call(
                            0x006e_7e70,
                            &args![solution, 0.0f32, travelled, 1u32, found_a, found_b],
                        )
                        .bool()
                    {
                        for i in 0..3 {
                            let word = e.mem.u32(found_b + 4 * i);
                            e.mem.set_u32(pos + 4 * i, word);
                        }
                    }
                    e.call(0x006e_2620, &args![request]);
                    e.call(0x006e_7720, &args![solution]);
                }
            }
            if e.call(0x008c_7aa0, &args![]).bool() {
                let singleton = e.call(0x0045_37b0, &args![]).u32();
                e.call(0x0087_b650, &args![singleton, actor, pos]);
                let singleton = e.call(0x0045_37b0, &args![]).u32();
                e.call(0x0087_b190, &args![singleton, actor]);
            } else {
                e.vcall(actor, 0x2a8, &args![pos]);
            }
            e.call(0x0057_5830, &args![actor, pos]);
            travelled = (f64::from(travelled) + f64::from(radius)) as f32;
            let loaded = cell != 0 && {
                let tes = e.global::<u32>(TES_SINGLETON);
                e.call(0x0045_11e0, &args![tes, cell, 0u32]).bool()
            };
            if loaded {
                e.call(0x0057_57d0, &args![actor, angle]);
                e.call(0x0057_5770, &args![actor, 0.0f32]);
            } else {
                let largest: f32 = e.global(FLOAT_MAX);
                e.call(0x0057_5770, &args![actor, largest]);
            }
            e.call(0x0057_3800, &args![actor, cell, world_space]);
            if actor != leader {
                let follower_process = e.call(GET_PROCESS, &args![actor]).u32();
                e.call(0x008b_3ad0, &args![actor]);
                e.vcall(actor, 0x434, &args![0u32]);
                e.call(0x0088_1680, &args![actor, 0u32]);
                if follower_process != 0 {
                    e.vcall(follower_process, 0x28, &args![]);
                }
            }
            node = next(e, node);
        }
        e.call(0x0046_ffb0, &args![list]);
    });
}

// Translated from 008ad8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the byte `value` at `this + 0x9b`.
pub fn fn_008ad8f0(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x9b, value);
}

// Translated from 008ad910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_008ad1c0` of the actor to the place of `reference`: the world space
/// (`TESObjectREFR::GetWorldSpace`), the cell (`008d6f30`) and the angle
/// (`float` at `+8` of what `00430830` returns, its rotation about z) of the reference, and the
/// reference's own virtual `+0x1f4` as the position.
pub fn fn_008ad910(e: &mut Engine, this: Ptr<Actor>, reference: Ptr, spaced: bool, forced: bool) {
    let world_space = e.call(0x0057_5d70, &args![reference]).u32();
    let cell = e.call(GET_PARENT_CELL, &args![reference]).u32();
    let place = e.call(0x0043_0830, &args![reference]).u32();
    let angle = e.mem.f32(place + 8);
    let position = e.vcall(reference.addr(), 0x1f4, &args![]).u32();
    fn_008ad1c0(
        e,
        this,
        Ptr::new(position),
        angle,
        cell,
        world_space,
        spaced,
        forced,
    );
}

// Translated from 008ad970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Gives the actor a new search package: a `SearchPackage` (`009f66b0`, an
/// object of `0x1a4` bytes) of type `0x1e` with `00826b40(0)`,
/// `00826b90(0)`; a `PackageLocation` (`0x0c` bytes, `0067f030`) set to
/// the actor and handed over (`SetPackageLocation`, then freed); `0067a4a0(1)`;
/// a `PackageTarget` (`0x10` bytes, `0067ff70`); `00984f60(0x21)`; target
/// type `0`, target reference `reference` and a count from the setting
/// `011cda7c`. The actor's process gets virtual `+0xec`, `+0x28`, `+0xdc`
/// and the actor's virtual `+0x2f4(package, 1, 1)`; a nonzero `place`
/// (`NiPoint3::operator!=` against the zero point, `00439090`) is stored in
/// the package (`008adc20`), and with `flag` the process virtual `+0x288`
/// is called with the actor.
pub fn fn_008ad970(
    e: &mut Engine,
    this: Ptr<Actor>,
    reference: u32,
    place_x: u32,
    place_y: u32,
    place_z: u32,
    flag: bool,
) {
    let memory = e.call(OPERATOR_NEW, &args![0x1a4u32]).u32();
    let package = if memory == 0 {
        0
    } else {
        e.call(
            0x009f_66b0,
            &args![
                memory,
                place_x,
                place_y,
                place_z,
                reference,
                u32::from(flag)
            ],
        )
        .u32()
    };
    e.call(0x0067_0fc0, &args![package, 0x1eu32]);
    e.call(0x0082_6b40, &args![package, 0u32]);
    e.call(0x0082_6b90, &args![package, 0u32]);
    let memory = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
    let location = if memory == 0 {
        0
    } else {
        e.call(0x0067_f030, &args![memory]).u32()
    };
    e.call(0x0067_f3c0, &args![location, this]);
    e.call(0x0067_1d30, &args![package, location]);
    if location != 0 {
        e.call(0x0067_0b30, &args![location, 1u32]);
    }
    e.call(0x0067_a4a0, &args![package, 1u32]);
    let memory = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
    let target = if memory == 0 {
        0
    } else {
        e.call(0x0067_ff70, &args![memory]).u32()
    };
    e.call(0x0067_2fc0, &args![package, target]);
    e.call(0x0098_4f60, &args![package, 0x21u32]);
    let package_target = e.call(0x0067_1d10, &args![package]).u32();
    e.call(0x0068_00b0, &args![package_target, 0u32]);
    let package_target = e.call(0x0067_1d10, &args![package]).u32();
    e.call(0x0068_0110, &args![package_target, reference]);
    let distance = setting_float(e, SETTING_SEARCH_DISTANCE);
    let count = e.call(FTOL, &args![f64::from(distance)]).u32();
    let package_target = e.call(0x0067_1d10, &args![package]).u32();
    e.call(0x0040_3550, &args![package_target, count]);
    let process = acquire_object(e, this);
    e.vcall(process.addr(), 0xec, &args![]);
    let process = acquire_object(e, this);
    e.vcall(process.addr(), 0x28, &args![]);
    let process = acquire_object(e, this);
    e.vcall(process.addr(), 0xdc, &args![]);
    e.vcall(this.addr(), 0x2f4, &args![package, 1u32, 1u32]);
    let different = e.with_stack(12, |e, place| {
        e.mem.set_u32(place.addr(), place_x);
        e.mem.set_u32(place.addr() + 4, place_y);
        e.mem.set_u32(place.addr() + 8, place_z);
        e.call(0x0043_9090, &args![place, ZERO_POINT]).bool()
    });
    if different {
        fn_008adc20(e, Ptr::new(package), place_x, place_y, place_z);
    }
    if flag {
        let process = acquire_object(e, this);
        e.vcall(process.addr(), 0x288, &args![this, 1u32]);
    }
}

// Translated from 008adc20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores three words at `this + 0x94`.
pub fn fn_008adc20(e: &mut Engine, this: Ptr, first: u32, second: u32, third: u32) {
    e.mem.set_u32(this.addr() + 0x94, first);
    e.mem.set_u32(this.addr() + 0x98, second);
    e.mem.set_u32(this.addr() + 0x9c, third);
}

// Translated from 008adc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the actor has a process: its virtual `+0x160(0, 0, 0)` and then
/// `+0x168(0)`.
pub fn fn_008adc50(e: &mut Engine, this: Ptr<Actor>) {
    if acquire_object(e, this).addr() != 0 {
        let process = acquire_object(e, this);
        e.vcall(process.addr(), 0x160, &args![0u32, 0u32, 0u32]);
        let process = acquire_object(e, this);
        e.vcall(process.addr(), 0x168, &args![0u32]);
    }
}

// Translated from 008adcb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetFaceAnimationData` (Xbox PDB): the process's virtual `+0x144`
/// with the actor, or 0 without a process.
pub fn actor_get_face_animation_data(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    let process = process(e, this);
    if process.addr() == 0 {
        0
    } else {
        e.vcall(process.addr(), 0x144, &args![this]).u32()
    }
}

/// The type descriptor of `TESBoundObject` (`01183108`).
const TYPE_BOUND_OBJECT: u32 = 0x0118_3108;
/// The type descriptor of `BGSPickupPutdownSounds` (`011832d4`).
const TYPE_PICKUP_PUTDOWN_SOUNDS: u32 = 0x0118_32d4;
/// The sound names `Actor::GetPickUpSoundName` returns.
const SOUND_GUNS_BIG_UP: u32 = 0x0108_4e40;
const SOUND_GUNS_BIG_DOWN: u32 = 0x0108_4e2c;
const SOUND_GUNS_SMALL_UP: u32 = 0x0108_4e18;
const SOUND_GUNS_SMALL_DOWN: u32 = 0x0108_4e04;
const SOUND_MELEE_UP: u32 = 0x0108_4df4;
const SOUND_MELEE_DOWN: u32 = 0x0108_4de4;
const SOUND_GENERIC_UP: u32 = 0x0108_4dd4;
const SOUND_GENERIC_DOWN: u32 = 0x0108_4dc0;
/// The ring of five `NiPoint3` that `008ae4c0` hands out (`011df7a0`) and
/// the index of the one it used last (`011df880`).
const LOCATION_RING: u32 = 0x011d_f7a0;
const LOCATION_RING_INDEX: u32 = 0x011d_f880;
/// `-FLT_MAX` (`double`): what the water height is when there is no water.
const NO_WATER: u32 = 0x0102_41b0;
/// `5.0` (`double`).
const WATER_MARGIN: u32 = 0x0102_0998;
/// `0.75` (`double`), as `HEIGHT_FACTOR`.
const EYE_HEIGHT_FACTOR: u32 = HEIGHT_FACTOR;
/// `2.5` (`double`), `2.0` (`double`), `0.1`, `0.9` (`double`s) and `0.1`
/// (`float`) of `008ae000`.
const IMPULSE_FACTOR: u32 = 0x0101_8c00;
const TWO: u32 = 0x0101_1590;
const IMPULSE_FLOOR_DOUBLE: u32 = 0x0101_ffa0;
const IMPULSE_SLOPE: u32 = 0x0106_b9e8;
const IMPULSE_FLOOR_FLOAT: u32 = 0x0101_e2bc;
/// The `float`s `008ae000` scales its direction vectors by.
const IMPULSE_SCALE_HIT: u32 = 0x0101_8bfc;
const IMPULSE_SCALE_BODY: u32 = 0x0102_36e0;
/// The pointer (`011e07d0`) `Actor::GetEyeVector` uses for the player.
const PLAYER_EYE_SOURCE: u32 = 0x011e_07d0;

/// The three words at `at`.
fn point_at(e: &Engine, at: u32) -> [u32; 3] {
    [e.mem.u32(at), e.mem.u32(at + 4), e.mem.u32(at + 8)]
}

/// Stores three words at `at`.
fn set_point(e: &mut Engine, at: u32, point: [u32; 3]) {
    e.mem.set_u32(at, point[0]);
    e.mem.set_u32(at + 4, point[1]);
    e.mem.set_u32(at + 8, point[2]);
}

// Translated from 008adcf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetPickUpSoundName` (Xbox PDB): the name of the sound for picking
/// up (`up` is 1) or putting down `item`; `this` is not used. A form with a
/// `BGSPickupPutdownSounds` component (`__RTDynamicCast`) gives the name of
/// its pickup sound (virtual `+0x130` of the field at `+4`; with `flag` clear
/// and `up` nonzero) or putdown sound (`+8`; with both clear). Otherwise by
/// the form type byte: `0x28` (a weapon) by `BGSEquipType::GetEquipType`:
/// `0` the big gun sounds, `2` the small gun sounds, `3` the melee sounds;
/// `0x2f` with `flag` the name of the field at `+0xc8` (virtual `+0x130`),
/// or 0 without it. Everything else gets the generic names.
pub fn actor_get_pick_up_sound_name(
    e: &mut Engine,
    _this: Ptr<Actor>,
    item: Ptr,
    up: u8,
    flag: u8,
) -> u32 {
    if item.addr() == 0 {
        return 0;
    }
    let cast = e
        .call(
            DYNAMIC_CAST,
            &args![
                item,
                0u32,
                TYPE_BOUND_OBJECT,
                TYPE_PICKUP_PUTDOWN_SOUNDS,
                0u32
            ],
        )
        .u32();
    if flag == 0 && up != 0 && cast != 0 && e.call(0x0072_6070, &args![cast]).u32() != 0 {
        let sound = e.call(0x0072_6070, &args![cast]).u32();
        return e.vcall(sound, 0x130, &args![]).u32();
    }
    if flag == 0 && up == 0 && cast != 0 && e.call(0x0044_ddc0, &args![cast]).u32() != 0 {
        let sound = e.call(0x0044_ddc0, &args![cast]).u32();
        return e.vcall(sound, 0x130, &args![]).u32();
    }
    let kind = e.call(0x0040_1170, &args![item]).u32();
    if kind == 0x28 {
        match e.call(0x0047_9430, &args![item]).u32() {
            0 => {
                return if up != 0 {
                    SOUND_GUNS_BIG_UP
                } else {
                    SOUND_GUNS_BIG_DOWN
                }
            }
            2 => {
                return if up != 0 {
                    SOUND_GUNS_SMALL_UP
                } else {
                    SOUND_GUNS_SMALL_DOWN
                }
            }
            3 => {
                return if up != 0 {
                    SOUND_MELEE_UP
                } else {
                    SOUND_MELEE_DOWN
                }
            }
            _ => {}
        }
    } else if kind == 0x2f && flag != 0 {
        if e.call(0x009a_cce0, &args![item]).u32() == 0 {
            return 0;
        }
        let sound = e.call(0x009a_cce0, &args![item]).u32();
        return e.vcall(sound, 0x130, &args![]).u32();
    }
    if up == 1 {
        SOUND_GENERIC_UP
    } else {
        SOUND_GENERIC_DOWN
    }
}

// Translated from 008aded0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Plays the pickup or putdown sound of `item` (`actor_get_pick_up_sound_name`).
/// For the player the sound is the handle `BSAudio::GetSoundHandleByName`
/// finds (`0x121`), played unless it already plays; for others it is
/// `MobileObject::PlaySoundByEditorName(name, 0, 0x102, 1)`. Nothing for a
/// null `item`. (The exception frame is not translated.)
pub fn fn_008aded0(e: &mut Engine, this: Ptr<Actor>, item: Ptr, up: u8, flag: u8) {
    if item.addr() == 0 {
        return;
    }
    e.with_stack(0x10, |e, handle| {
        e.call(SOUND_HANDLE_INIT, &args![handle]);
        let name = actor_get_pick_up_sound_name(e, this, item, up, flag);
        if this.addr() == e.global::<u32>(PLAYER_CHARACTER) {
            let audio = e.call(0x0045_3a70, &args![]).u32();
            e.with_stack(0x10, |e, found| {
                let result = e
                    .call(0x00ad_7550, &args![audio, found, name, 0x121u32])
                    .u32();
                e.call(SOUND_HANDLE_ASSIGN, &args![handle, result]);
                e.call(SOUND_HANDLE_DESTRUCT, &args![found]);
            });
            if !e.call(0x00ad_8930, &args![handle]).bool() {
                e.call(0x00ad_8830, &args![handle, 0u32]);
            }
        } else {
            e.with_stack(0x10, |e, played| {
                let result = e
                    .call(
                        0x0093_3270,
                        &args![this, played, name, 0u32, 0x102u32, 1u32],
                    )
                    .u32();
                e.call(SOUND_HANDLE_ASSIGN, &args![handle, result]);
                e.call(SOUND_HANDLE_DESTRUCT, &args![played]);
            });
        }
        e.call(SOUND_HANDLE_DESTRUCT, &args![handle]);
    });
}

// Translated from 008ae000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Pushes the body of the explosion's target away. `data` is the explosion
/// record: target reference (`+4`), source (`+8`; `from_hit` when its
/// virtual `+0x224` holds), strength (`float` at `+0x14`), a record at
/// `+0x30` whose `008ae480`/`008ae4a0` give the radii, and the points at
/// `+0x38` and `+0x44`. The target must pass virtual `+0x21c` false or
/// `+0x38c` true and have 3D (`0043fcd0`). The impulse direction is built
/// from the points (normalised by `004a0c10`, scaled by one of two
/// `float`s), the strength by `006466e0(strength, from_hit)` times `2.5`
/// of the first radius and, for a hit within the second radius, a falloff
/// (`0.1` outside twice the radius, else `((2r^2 - d^2) / r^2) * 0.9 + 0.1`,
/// at least `0.1`, `00404010`). Then the body's forces are removed
/// (`bhkForceController::RemoveForce`), its motion set (`bhkWorld::SetMotion`)
/// and `TESHavokUtilities::AddExplosionImpulse(body, impulse, strength,
/// data)` applied.
pub fn fn_008ae000(e: &mut Engine, this: Ptr<Actor>, data: Ptr) {
    let record = data.addr();
    let target = e.mem.u32(record + 4);
    let base_strength = e.mem.f32(record + 0x14);
    let source = e.mem.u32(record + 8);
    let from_hit = source != 0 && e.vcall(source, 0x224, &args![]).bool();
    let node = e.vcall(this.addr(), 0x1d0, &args![]).u32();
    let zero = point_at(e, ZERO_POINT);
    e.with_stack(0x140, |e, frame| {
        let base = frame.addr();
        // `A`, `B` and `C` of the game's frame; the rest are scratch.
        let point_a = base;
        let point_b = base + 0x10;
        let direction = base + 0x20;
        set_point(e, point_a, zero);
        set_point(e, point_b, zero);
        if node != 0 {
            let process = acquire_object(e, this);
            let part = e.vcall(process.addr(), 0x460, &args![]).u32();
            let object = e.call(0x0046_1130, &args![node]).u32();
            let found = e.call(0x0052_4c40, &args![object, base + 0x30, part]).u32();
            let point = point_at(e, found);
            set_point(e, point_a, point);
        }
        if target == 0 {
            return;
        }
        if e.vcall(target, 0x21c, &args![]).bool() && !e.vcall(target, 0x38c, &args![]).bool() {
            return;
        }
        let body = e.call(0x0043_fcd0, &args![target]).u32();
        if body == 0 {
            return;
        }
        set_point(e, direction, zero);
        if from_hit {
            let hit_point = e
                .call(0x004a_0bd0, &args![record + 0x44, base + 0x40])
                .u32();
            let point = point_at(e, hit_point);
            set_point(e, direction, point);
            e.call(0x004a_0c10, &args![direction]);
            let scale: f32 = e.global(IMPULSE_SCALE_HIT);
            e.call(0x0043_9180, &args![direction, scale]);
            e.call(0x0063_c8a0, &args![direction, record + 0x38]);
        } else {
            if node != 0 {
                let weapon = actor_get_weapon_position(e, this, Ptr::new(base + 0x50), 0);
                let object = e.call(0x0046_1130, &args![node]).u32();
                let found = e
                    .call(0x0052_4c40, &args![object, base + 0x80, weapon])
                    .u32();
                let point = point_at(e, found);
                set_point(e, point_b, point);
            }
            let difference = e
                .call(0x0043_9ef0, &args![point_a, base + 0x90, point_b])
                .u32();
            let point = point_at(e, difference);
            set_point(e, direction, point);
            e.call(0x004a_0c10, &args![direction]);
            let hit_point = e
                .call(0x004a_0bd0, &args![record + 0x44, base + 0xc0])
                .u32();
            e.call(0x0063_c8a0, &args![direction, hit_point]);
            let scale: f32 = e.global(IMPULSE_SCALE_BODY);
            e.call(0x0043_9180, &args![direction, scale]);
            e.call(0x0063_c8a0, &args![direction, point_a]);
        }
        let mut strength = e
            .call(0x0064_66e0, &args![base_strength, u32::from(from_hit)])
            .f32();
        let radii = e.mem.u32(record + 0x30);
        if radii != 0 {
            let radius = fn_008ae480(e, Ptr::new(radii));
            let factor: f64 = e.global(IMPULSE_FACTOR);
            strength = (f64::from(radius) * factor) as f32;
            if from_hit {
                let zero_double: f64 = e.global(ZERO_DOUBLE);
                if f64::from(fn_008ae4a0(e, Ptr::new(radii))) > zero_double {
                    let body_point = e.call(0x0045_bb80, &args![body]).u32();
                    let body_point = point_at(e, body_point);
                    let from_body = base + 0xa0;
                    set_point(e, from_body, body_point);
                    let offset = base + 0xb0;
                    e.call(0x0043_9ef0, &args![from_body, offset, point_a]);
                    let x = e.mem.f32(offset);
                    let y = e.mem.f32(offset + 4);
                    let distance_squared =
                        (f64::from(x) * f64::from(x) + f64::from(y) * f64::from(y)) as f32;
                    let inner = fn_008ae4a0(e, Ptr::new(radii));
                    let inner_squared = (f64::from(inner) * f64::from(inner)) as f32;
                    if inner_squared < distance_squared {
                        let two: f64 = e.global(TWO);
                        let floor: f64 = e.global(IMPULSE_FLOOR_DOUBLE);
                        let doubled = f64::from(fn_008ae4a0(e, Ptr::new(radii))) * two;
                        if f64::from(distance_squared) <= doubled {
                            let slope: f64 = e.global(IMPULSE_SLOPE);
                            let falloff = ((f64::from(inner_squared) * two
                                - f64::from(distance_squared))
                                / f64::from(inner_squared)
                                * slope
                                + floor) as f32;
                            let least: f32 = e.global(IMPULSE_FLOOR_FLOAT);
                            let falloff = e.call(0x0040_4010, &args![falloff, least]).f32();
                            strength = (f64::from(strength) * f64::from(falloff)) as f32;
                        } else {
                            strength = (f64::from(strength) * floor) as f32;
                        }
                    }
                }
            }
        }
        let impulse = base + 0x100;
        let forces = base + 0xe0;
        e.call(0x00cb_9650, &args![body]);
        e.call(0x00c6_a350, &args![body, 1u32, 1u32, 0u32, 1u32]);
        e.call(0x0043_d410, &args![forces, 0.0f32, 0u32, 0u32]);
        e.call(0x00a5_9c60, &args![body, forces]);
        e.call(0x0068_15c0, &args![impulse]);
        e.call(0x004a_3e00, &args![impulse, direction]);
        e.call(0x0062_b660, &args![body, impulse, strength, data]);
    });
}

// Translated from 008ae480 (decompiled, FalloutNV.exe 1.4.0.525)
/// The float at `this + 0x1a8`.
pub fn fn_008ae480(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + 0x1a8)
}

// Translated from 008ae4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The float at `this + 0x1b8`.
pub fn fn_008ae4a0(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + 0x1b8)
}

// Translated from 008ae4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetLocationOnReference` (Xbox PDB): the position of the actor
/// (`00436aa0`), or, when `005a2030` holds and the water height
/// (`TESObjectREFR::GetRelevantWaterHeight`) is not `-FLT_MAX`, a copy of it
/// in the next of five slots of the ring at `011df7a0` whose `z` is raised by
/// `0.75` of the actor's height (`008853a0`) but at most to the water height
/// minus `5.0`, in a cell for which `004518e0` holds.
pub fn actor_get_location_on_reference(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    if !e.call(0x005a_2030, &args![this]).bool() {
        return e.call(0x0043_6aa0, &args![this]).u32();
    }
    let water = e.call(0x0057_b0a0, &args![this]).f64();
    let no_water: f64 = e.global(NO_WATER);
    if water == no_water {
        return e.call(0x0043_6aa0, &args![this]).u32();
    }
    let mut index = e.global::<u32>(LOCATION_RING_INDEX) as i32;
    index = index.wrapping_add(1);
    e.set_global(LOCATION_RING_INDEX, index as u32);
    if index >= 5 {
        e.set_global(LOCATION_RING_INDEX, 0u32);
    }
    let position = e.call(0x0043_6aa0, &args![this]).u32();
    let point = point_at(e, position);
    let slot = LOCATION_RING + 12 * e.global::<u32>(LOCATION_RING_INDEX);
    set_point(e, slot, point);
    let cell = e.call(GET_PARENT_CELL, &args![this]).u32();
    if cell != 0 && e.call(0x0045_18e0, &args![cell]).bool() {
        let margin: f64 = e.global(WATER_MARGIN);
        let ceiling = (e.call(0x0057_b0a0, &args![this]).f64() - margin) as f32;
        let height = e.call(0x0088_53a0, &args![this]).f64();
        let factor: f64 = e.global(EYE_HEIGHT_FACTOR);
        let rise = (height * factor) as f32;
        let slot = LOCATION_RING + 12 * e.global::<u32>(LOCATION_RING_INDEX);
        let top = f64::from(e.mem.f32(slot + 8)) + f64::from(rise);
        if f64::from(ceiling) <= top {
            e.mem.set_f32(slot + 8, ceiling);
        } else {
            e.mem.set_f32(slot + 8, top as f32);
        }
    }
    LOCATION_RING + 12 * e.global::<u32>(LOCATION_RING_INDEX)
}

// Translated from 008ae600 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `this + 0x148` (`Actor::pTemplateActorBase`).
pub fn fn_008ae600(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    e.mem.u32(this.addr() + 0x148)
}

// Translated from 008ae620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the word at `this + 0x148` (`Actor::pTemplateActorBase`).
pub fn fn_008ae620(e: &mut Engine, this: Ptr<Actor>, value: u32) {
    e.mem.set_u32(this.addr() + 0x148, value);
}

// Translated from 008ae640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::SetForceUpdateQuest` (Xbox PDB): stores the byte `value` at
/// `this + 0x126` (`bForceUpdateQuestTarget`).
pub fn actor_set_force_update_quest(e: &mut Engine, this: Ptr<Actor>, value: u8) {
    e.mem.set_u8(this.addr() + 0x126, value);
}

// Translated from 008afef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `this + 0x3a8`.
pub fn fn_008afef0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x3a8)
}

// Translated from 008aff10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `00621b00` of the member at `this + 0x54`: a `float` (in `ST0`).
pub fn fn_008aff10(e: &mut Engine, this: Ptr) -> f32 {
    e.call(0x0062_1b00, &args![this.addr() + 0x54]).f32()
}

// Translated from 008aff30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `bhkFilteredCdBodyCollector` (its RTTI name): the base
/// constructor `0062d350`, then the vtable at `01084e54`.
pub fn fn_008aff30(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0062_d350, &args![this]);
    e.mem.set_u32(this.addr(), 0x0108_4e54);
    this
}

// Translated from 008aff50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of `bhkFilteredCdBodyCollector`: the body
/// `00cb9900`, then `0062d470(this)` (the free) when bit 0 of `flags` is set.
pub fn fn_008aff50(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x00cb_9900, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0062_d470, &args![this]);
    }
    this
}

// Translated from 008aff80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetEyeVector` (Xbox PDB): writes the eye position to `position`
/// (the player's source `011e07d0`, through `0045bb80`, when there is one;
/// otherwise the actor's position `00436aa0` with `z` raised by
/// `Actor::GetEyeLevel`), adds the player's offset (`0070c410`) when `offset`
/// is set and `00524d10` of the player holds, then turns `direction`: a
/// rotation matrix about z from virtual `+0x2bc` (and for the player
/// multiplied by the `x` rotation of `MobileObject::GetLooking`, `0043f8d0`)
/// is applied to it (`00439f50(1, direction)`) and it is normalised
/// (`004a0c10`).
pub fn actor_get_eye_vector(
    e: &mut Engine,
    this: Ptr<Actor>,
    position: Ptr,
    direction: Ptr,
    offset: bool,
) {
    let player = e.global::<u32>(PLAYER_CHARACTER);
    let source = e.global::<u32>(PLAYER_EYE_SOURCE);
    if this.addr() == player && source != 0 {
        let at = e.call(0x0045_bb80, &args![source]).u32();
        let point = point_at(e, at);
        set_point(e, position.addr(), point);
    } else {
        let at = e.call(0x0043_6aa0, &args![this]).u32();
        let point = point_at(e, at);
        set_point(e, position.addr(), point);
        let eye = e.call(0x008b_e940, &args![this]).f64();
        let height = e.mem.f32(position.addr() + 8);
        e.mem
            .set_f32(position.addr() + 8, (eye + f64::from(height)) as f32);
    }
    if offset && this.addr() == player && e.call(0x0052_4d10, &args![player]).bool() {
        e.with_stack(0x10, |e, scratch| {
            let shift = e.call(0x0070_c410, &args![player, scratch]).u32();
            e.call(0x0063_c8a0, &args![position, shift]);
        });
    }
    e.with_stack(0xc0, |e, frame| {
        // The game's three 36-byte matrices: the rotation, the x rotation
        // and the product's scratch.
        let matrix = frame.addr();
        let x_rotation = frame.addr() + 0x30;
        let product = frame.addr() + 0x60;
        e.call(0x0068_15c0, &args![matrix]);
        let heading = e.vcall(this.addr(), 0x2bc, &args![0u32]).f32();
        e.call(0x004a_0c90, &args![matrix, heading]);
        if this.addr() == player {
            e.call(0x0068_15c0, &args![x_rotation]);
            let looking = e.call(0x0093_1d70, &args![this]).f32();
            e.call(0x0052_4ac0, &args![x_rotation, looking]);
            let result = e
                .call(0x0043_f8d0, &args![matrix, product, x_rotation])
                .u32();
            for i in 0..9 {
                let word = e.mem.u32(result + 4 * i);
                e.mem.set_u32(matrix + 4 * i, word);
            }
        }
        e.call(0x0043_9f50, &args![matrix, 1u32, direction]);
        e.call(0x004a_0c10, &args![direction]);
    });
}

/// The weapon form used when the actor holds no item (`011ca278`, a pointer
/// to the unarmed weapon).
const UNARMED_WEAPON: u32 = 0x011c_a278;
/// `3.0` (`double`): added to the weight `008ae660` sums.
const HIT_WEIGHT_BONUS: u32 = 0x0102_1928;
/// `255.0` (`double`): the divisor of the decal colour channels.
const COLOR_CHANNEL_MAX: u32 = 0x0101_e568;
/// `4.0` (`float`): the upper bound of the decal variant's random draw.
const DECAL_VARIANT_RANGE: u32 = 0x0101_e570;
/// The `float` `Actor::fn_008ae660` passes to `00963eb0`.
const HIT_SHAKE_STRENGTH: u32 = 0x0102_918c;
/// The `GameSetting` objects `008ae660` reads: the integer range of a weapon
/// with no form (`011d10d4`), the ranges for the weapon types `0` to `2` and
/// the rest, then the weight exponent, the impulse factor, the speed limit
/// and the boost for the body class `8`/`0x1d`.
const SETTING_RANGE_DEFAULT: u32 = 0x011d_10d4;
const SETTING_RANGE_KIND_0: u32 = 0x011d_0ec4;
const SETTING_RANGE_KIND_1: u32 = 0x011d_1128;
const SETTING_RANGE_KIND_2: u32 = 0x011d_0484;
const SETTING_RANGE_OTHER: u32 = 0x011d_1050;
const SETTING_WEIGHT_EXPONENT: u32 = 0x011c_f77c;
const SETTING_IMPULSE_FACTOR: u32 = 0x011c_fb90;
const SETTING_SPEED_LIMIT: u32 = 0x011c_f66c;
const SETTING_BODY_BOOST: u32 = 0x011c_eb08;
/// The `GameSetting` byte (`011df7f8`) that turns the humanoid effect off.
const SETTING_NO_HUMANOID_EFFECT: u32 = 0x011d_f7f8;
/// The object `00525430` is asked about (`011f2250`).
const OBJECT_011F2250: u32 = 0x011f_2250;
/// The type descriptors `008ae660` casts the actor's base form with.
const TYPE_BASE_FORM: u32 = 0x0118_46e8;
const TYPE_CREATURE: u32 = 0x0118_3a00;
/// `Script::SetActionFlag`.
const SET_ACTION_FLAG: u32 = 0x005a_c750;
/// `004519d0`: what the `TES` singleton finds at a position.
const TES_FIND_AT: u32 = 0x0045_19d0;
/// The pickup, putdown or impact sound getters of the impact record.
const IMPACT_SOUND_A: u32 = 0x0068_a810;
const IMPACT_SOUND_B: u32 = 0x0068_a830;

/// Plays the impact sound `getter` finds in `impact` (a file name object,
/// looked up as a sound with flags `0x4102`) at `position`, in `handle`.
fn play_impact_sound(
    e: &mut Engine,
    impact: u32,
    getter: u32,
    handle: u32,
    scratch: u32,
    position: [u32; 3],
) {
    let flags_source = e.call(getter, &args![impact]).u32();
    let name_object = e.call(getter, &args![impact]).u32();
    let file = e.call(0x0051_1840, &args![name_object]).u32();
    let audio = e.call(0x0045_3a70, &args![]).u32();
    let found = e
        .call(
            0x00ad_7480,
            &args![audio, scratch, file, 0x4102u32, flags_source],
        )
        .u32();
    e.call(SOUND_HANDLE_ASSIGN, &args![handle, found]);
    e.call(SOUND_HANDLE_DESTRUCT, &args![scratch]);
    e.call(
        0x00ad_8b60,
        &args![handle, position[0], position[1], position[2]],
    );
    e.call(0x00ad_8830, &args![handle, 0u32]);
}

// Translated from 008ae660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resolves a melee swing of the actor against what its weapon reaches;
/// `power_attack` selects the power attack multiplier. Returns true when
/// something was hit.
///
/// It finds the physics cell of the actor's cell (`004543c0`) and a hit
/// caster (`00626dd0`), casts a ray from the eye (`Actor::GetEyeVector`;
/// range from the `GameSetting` of the weapon type) and, for what it hits
/// (a reference with 3D, and the body its collision shape belongs to),
/// works out the damage of the swing: `ItemChange::GetModifiedAttackDamage`
/// of the held item (with the power attack multiplier of
/// `CombatFormulas::GetPowerAttackDamageMultiplier` unless virtual `+0x21c`
/// or `004997b0` say otherwise), or the fists (`CalcHandToHandDamage`, or
/// virtual `+0x444` for creatures), plus `00645c20` of the weapon form when
/// virtual `+0x360` and `00525430` hold.
///
/// With a physics body hit: an impulse (direction from the 3D point of the
/// actor to the weapon position, `GameSetting` factor, weight of the weapon
/// or of the worn armour, capped by the speed) through `0081f1e0`, the
/// collision sound (`fn_008acff0`), the action flags `0x80`/`0x100`
/// (`Script::SetActionFlag`), the decals of the material's impact record
/// (`00522ba0`; one per collider of the decal caster whose material
/// matches), an effect and the sounds of the record, and, for a reference
/// that answers virtual `+0x224` (or is a weapon form, `0x28`), its damage
/// (virtual `+0x144`). Without one, an actor that is hit (virtual `+0x100`,
/// body class `0x1e`) goes through `00837520`. For the player, hitting its
/// activation target also sets flag `0x10000000`.
/// (The exception-unwinding frame is not translated.)
pub fn fn_008ae660(e: &mut Engine, this: Ptr<Actor>, power_attack: u8) -> bool {
    let actor = this.addr();
    let mut hit_anything = false;
    let cell = e.call(GET_PARENT_CELL, &args![this]).u32();
    let physics_cell = if cell == 0 {
        0
    } else {
        e.call(0x0045_43c0, &args![cell]).u32()
    };
    let hit_caster = e.call(0x0062_6dd0, &args![0u32, 1u32]).u32();
    let held = if acquire_object(e, this).addr() == 0 {
        0
    } else {
        let process = acquire_object(e, this);
        e.vcall(process.addr(), 0x148, &args![]).u32()
    };
    let weapon_form = if held == 0 {
        e.global::<u32>(UNARMED_WEAPON)
    } else {
        e.call(0x0044_ddc0, &args![held]).u32()
    };
    let range_setting = e.call(0x0043_d4d0, &args![SETTING_RANGE_DEFAULT]).u32();
    let mut range = e.mem.u32(range_setting) as i32 as f32;
    if weapon_form != 0 {
        let setting = match e.call(0x0044_6390, &args![weapon_form]).u32() {
            0 => SETTING_RANGE_KIND_0,
            1 => SETTING_RANGE_KIND_1,
            2 => SETTING_RANGE_KIND_2,
            _ => SETTING_RANGE_OTHER,
        };
        range = setting_float(e, setting);
    }
    if physics_cell == 0 || hit_caster == 0 {
        return false;
    }
    // The locals the game keeps on its stack, at offsets of one block.
    e.with_stack(0x600, |e, frame| {
        let base = frame.addr();
        let eye_position = base;
        let eye_direction = base + 0x10;
        let scratch = base + 0x20;
        let point_a = base + 0x30;
        let point_b = base + 0x40;
        let direction = base + 0x50;
        let contact = base + 0x60;
        let out_a = base + 0x70;
        let out_weapon = base + 0x80;
        let out_b = base + 0x90;
        let push = base + 0xa0;
        let push_unit = base + 0xb0;
        let class_scratch_a = base + 0xc0;
        let class_scratch_b = base + 0xd0;
        let impulse = base + 0xe0;
        let normal = base + 0x100;
        let decal_direction = base + 0x110;
        let decal_origin = base + 0x120;
        let color = base + 0x130;
        let sink = base + 0x140;
        let reflected = base + 0x150;
        let reflected_out = base + 0x160;
        let sound_a = base + 0x170;
        let sound_b = base + 0x180;
        let handle = base + 0x1a0;
        let damage_slot = base + 0x1f0;
        let damage_scratch = base + 0x1fc;
        let collector = base + 0x200;
        let decal_info = base + 0x340;
        e.call(0x0068_15c0, &args![eye_position]);
        e.call(0x0068_15c0, &args![eye_direction]);
        actor_get_eye_vector(
            e,
            this,
            Ptr::new(eye_position),
            Ptr::new(eye_direction),
            false,
        );
        e.call(0x0062_6d20, &args![hit_caster, physics_cell]);
        let controller = e.call(0x0093_1ed0, &args![this, scratch]).u32();
        let wanted = e.call(0x004a_3a20, &args![controller]).u32();
        if fn_008afef0(e, Ptr::new(hit_caster)) != wanted {
            e.call(0x0062_6c50, &args![hit_caster, wanted]);
        }
        if !e
            .call(
                0x0062_6b30,
                &args![hit_caster, eye_position, eye_direction, range],
            )
            .bool()
        {
            return;
        }
        let caster = e.call(0x0068_15c0, &args![hit_caster]).u32();
        let first = e.call(0x0046_0140, &args![caster]).u32();
        let entry = e.call(0x0080_7770, &args![first, 0u32]).u32();
        let collidable = e.mem.u32(entry + 0x48);
        let hit_node = e.call(0x004b_5820, &args![collidable]).u32();
        let hit_ref = if hit_node == 0 {
            0
        } else {
            e.call(0x0056_f930, &args![hit_node]).u32()
        };
        let first = e.call(0x0046_0140, &args![caster]).u32();
        let entry = e.call(0x0080_7770, &args![first, 0u32]).u32();
        let collidable = e.mem.u32(entry + 0x48);
        let body_object = e.call(0x004b_59f0, &args![collidable]).u32();
        let hit_actor = if hit_ref != 0 && e.vcall(hit_ref, 0x100, &args![]).bool() {
            hit_ref
        } else {
            0
        };
        let hit_id = if weapon_form == 0 {
            0xffff_ffff
        } else {
            e.call(0x0044_6390, &args![weapon_form]).u32()
        };
        // The damage of the swing.
        let owner = if actor == 0 { 0 } else { actor + 0xa4 };
        let mut damage: f32;
        if held != 0 {
            let animation = e.vcall(actor, 0x1e4, &args![]).u32();
            let group = e.call(0x0043_01b0, &args![animation, 4u32]).u32() as u16;
            let group_type = e.call(0x005f_2440, &args![u32::from(group)]).u32();
            let mut multiplier = 1.0f32;
            if !e.vcall(actor, 0x21c, &args![]).bool()
                && !e.call(0x0049_97b0, &args![this]).bool()
                && power_attack != 0
            {
                let bound = e.call(0x008d_85e0, &args![weapon_form]).u32();
                let value = e.call(0x0066_ef20, &args![actor + 0xa4, bound]).u32();
                multiplier = e.call(0x0064_4520, &args![value, group_type]).f32();
            }
            damage = e
                .call(0x004b_df00, &args![held, owner, multiplier, 0u32])
                .f32();
        } else if e.vcall(actor, 0x21c, &args![]).bool() {
            damage = e.vcall(actor, 0x444, &args![]).i32() as f32;
        } else {
            e.mem.set_f32(damage_slot, 0.0);
            e.call(
                0x0064_6310,
                &args![owner, 0u32, damage_slot, damage_scratch],
            );
            damage = e.mem.f32(damage_slot);
        }
        if e.vcall(actor, 0x360, &args![]).bool()
            && e.call(0x0052_5430, &args![OBJECT_011F2250]).bool()
        {
            let mut extra = 0.0f32;
            if weapon_form != 0 {
                extra = f32::from(e.call(0x0064_5c20, &args![weapon_form]).u32() as u16);
            } else {
                let unarmed = e.global::<u32>(UNARMED_WEAPON);
                if unarmed != 0 {
                    extra = f32::from(e.call(0x0064_5c20, &args![unarmed]).u32() as u16);
                }
            }
            damage = (f64::from(damage) + f64::from(extra)) as f32;
        }
        let hit_owner = if weapon_form != 0 {
            weapon_form
        } else {
            e.global::<u32>(UNARMED_WEAPON)
        };
        if body_object != 0 {
            let body_ref = e.call(0x004b_5a80, &args![body_object]).u32();
            let mut humanoid_effect = true;
            e.call(0x0068_15c0, &args![point_a]);
            e.call(0x0068_15c0, &args![point_b]);
            let node = e.vcall(actor, 0x1d0, &args![]).u32();
            if node != 0 {
                let process = e.mem.u32(actor + 0x68);
                let part = e.vcall(process, 0x460, &args![]).u32();
                let found = e.call(0x0046_1130, &args![node]).u32();
                let at = e.call(0x0052_4c40, &args![found, out_a, part]).u32();
                let point = point_at(e, at);
                set_point(e, point_a, point);
                let weapon = actor_get_weapon_position(e, this, Ptr::new(out_weapon), 0);
                let found = e.call(0x0046_1130, &args![node]).u32();
                let at = e.call(0x0052_4c40, &args![found, out_b, weapon]).u32();
                let point = point_at(e, at);
                set_point(e, point_b, point);
            }
            e.call(0x0043_9ef0, &args![point_b, direction, point_a]);
            e.call(0x004a_0c10, &args![direction]);
            e.call(0x0068_15c0, &args![contact]);
            let first = e.call(0x0046_0140, &args![caster]).u32();
            let entry = e.call(0x0080_7770, &args![first, 0u32]).u32();
            let entry = e.call(0x0068_15c0, &args![entry]).u32();
            e.call(0x0045_8620, &args![contact, entry]);
            let mut weight;
            if weapon_form != 0 {
                weight = e.call(0x004b_e380, &args![weapon_form, 0u32]).f32();
            } else {
                let chest = e.call(0x0089_1b90, &args![this, 3u32]).u32();
                let hands = e.call(0x0089_1b90, &args![this, 4u32]).u32();
                if chest != 0 || hands != 0 {
                    weight = 0.0f32;
                    if chest != 0 {
                        let form = e.call(0x0044_ddc0, &args![chest]).u32();
                        let part = e.call(0x006b_9130, &args![form + 0x60]).f64();
                        weight = (part + f64::from(weight)) as f32;
                    }
                    if hands != 0 {
                        let form = e.call(0x0044_ddc0, &args![hands]).u32();
                        let part = e.call(0x006b_9130, &args![form + 0x60]).f64();
                        weight = (part + f64::from(weight)) as f32;
                    }
                } else {
                    weight = 0.0;
                    humanoid_effect = false;
                }
                if chest != 0 {
                    e.call(0x0044_59e0, &args![chest, 1u32]);
                }
                if hands != 0 {
                    e.call(0x0044_59e0, &args![hands, 1u32]);
                }
            }
            let bonus: f64 = e.global(HIT_WEIGHT_BONUS);
            weight = (f64::from(weight) + bonus) as f32;
            let first = e.call(0x0046_0140, &args![caster]).u32();
            let entry = e.call(0x0080_7770, &args![first, 0u32]).u32();
            let slot = e.mem.u32(entry + 0x4c);
            let mut material = 0u32;
            let class = e.call(0x0043_b4f0, &args![body_ref, class_scratch_a]).u32();
            let class = e.call(0x0043_b4d0, &args![class]).u32();
            if class == 0x11 {
                let tes = e.global::<u32>(TES_SINGLETON);
                material = e.call(0x0045_7880, &args![tes, contact]).u32();
            } else {
                let shape = e.call(0x004a_e6a0, &args![body_ref]).u32();
                if shape != 0 {
                    material = e.call(0x0062_05a0, &args![shape]).u32();
                    if slot != 0xffff_ffff {
                        let sub = e.vcall(shape, 0xcc, &args![]).u32();
                        if sub != 0 {
                            material = e.vcall(sub, 0xe0, &args![slot]).u32();
                        }
                    }
                }
            }
            let exponent = setting_float(e, SETTING_WEIGHT_EXPONENT);
            weight = e.call(0x0040_ebd0, &args![weight, exponent]).f32();
            let factor = setting_float(e, SETTING_IMPULSE_FACTOR);
            let scaled = e
                .call(0x0045_bb20, &args![direction, push_unit, factor])
                .u32();
            e.call(0x0045_bb20, &args![scaled, push, weight]);
            let speed = e.call(0x004b_4ea0, &args![body_object]).f32();
            let limit = setting_float(e, SETTING_SPEED_LIMIT);
            if limit > speed {
                let limit = setting_float(e, SETTING_SPEED_LIMIT);
                let ratio = (f64::from(speed) / f64::from(limit)) as f32;
                e.call(0x0043_9180, &args![push, ratio]);
            }
            let class = e.call(0x0043_b4f0, &args![body_ref, class_scratch_b]).u32();
            if e.call(0x0062_4070, &args![class]).bool() {
                let boost = setting_float(e, SETTING_BODY_BOOST);
                e.call(0x0043_9180, &args![push, boost]);
            }
            e.call(0x0068_15c0, &args![impulse]);
            e.call(0x004a_3e00, &args![impulse, push]);
            e.call(0x0055_33c0, &args![physics_cell]);
            let first = e.call(0x0046_0140, &args![caster]).u32();
            let entry = e.call(0x0080_7770, &args![first, 0u32]).u32();
            let entry = e.call(0x0068_15c0, &args![entry]).u32();
            e.call(0x0081_f1e0, &args![body_object, impulse, entry]);
            e.call(0x0055_33c0, &args![physics_cell]);
            let contact_words = point_at(e, contact);
            fn_008acff0(
                e,
                this,
                contact_words[0],
                contact_words[1],
                contact_words[2],
                Ptr::new(hit_ref),
                body_ref,
                material,
            );
            if hit_ref != 0 {
                if hit_actor != 0 {
                    fn_008a5300(e, Ptr::new(hit_actor));
                }
                let list = e.call(GET_EXTRA_LIST, &args![hit_ref]).u32();
                e.call(SET_ACTION_FLAG, &args![this, list, 0x80u32]);
                let list = e.call(GET_EXTRA_LIST, &args![hit_ref]).u32();
                e.call(SET_ACTION_FLAG, &args![hit_owner, list, 0x100u32]);
            }
            // The impact record of the material and its effect.
            let impact = e.call(0x0052_2ba0, &args![hit_owner, material]).u32();
            let effect = if impact == 0 {
                0
            } else {
                e.call(0x0067_33e0, &args![impact]).u32()
            };
            if effect != 0 {
                // One decal for each collider of the decal caster whose
                // material is the one that was hit.
                e.call(0x0068_15c0, &args![normal]);
                let first = e.call(0x0046_0140, &args![caster]).u32();
                let entry = e.call(0x0080_7770, &args![first, 0u32]).u32();
                let inner = e.call(0x0046_0140, &args![entry]).u32();
                e.call(0x0045_8620, &args![normal, inner]);
                let normal_words = point_at(e, normal);
                set_point(e, decal_direction, normal_words);
                e.call(0x004a_0c10, &args![decal_direction]);
                set_point(e, decal_origin, contact_words);
                let high = e.call(0x004a_40c0, &args![impact]).f32();
                let low = e.call(0x004a_40a0, &args![impact]).f32();
                let size = e.call(0x0047_6b70, &args![low, high]).f32();
                let decal = e.call(0x0062_2a70, &args![size, 1u32]).u32();
                e.call(0x0062_33c0, &args![decal, 0u32]);
                e.call(0x004a_1a10, &args![decal, 8u32, 0u32]);
                e.call(0x004a_1a10, &args![decal, 0x1du32, 0u32]);
                let decal_cell = e.call(GET_PARENT_CELL, &args![this]).u32();
                let decal_tes = if decal_cell == 0 {
                    0
                } else {
                    e.call(0x0045_43c0, &args![decal_cell]).u32()
                };
                if decal_tes != 0 {
                    e.call(0x0062_31a0, &args![decal, decal_tes]);
                    fn_008aff30(e, Ptr::new(collector));
                    e.call(0x0062_3090, &args![decal, contact, collector]);
                    let list = e.call(0x0041_3f40, &args![collector]).u32();
                    let count = e.call(0x0072_6070, &args![list]).i32();
                    let random_angle = e.call(0x004a_4240, &args![]).f32();
                    let range_end: f32 = e.global(DECAL_VARIANT_RANGE);
                    let drawn = e.call(0x0047_6b70, &args![0.0f32, range_end]).f32();
                    let variant_input = e.call(0x0040_6cc0, &args![drawn]).f64();
                    let variant = e.call(FTOL, &args![variant_input]).u32() as u8;
                    for index in 0..count {
                        let list = e.call(0x0041_3f40, &args![collector]).u32();
                        let item = e.call(0x0062_e150, &args![list, index as u32]).u32();
                        let object = e.mem.u32(item + 8);
                        if object == 0 {
                            continue;
                        }
                        let mut found_material = 0x24u32;
                        let list = e.call(0x0041_3f40, &args![collector]).u32();
                        let item = e.call(0x0062_e150, &args![list, index as u32]).u32();
                        let item_slot = e.mem.u32(item + 0xc);
                        let node = e.call(0x0055_9450, &args![object]).u32();
                        let part = e.call(0x004a_3a40, &args![node]).u32();
                        if part == 0 {
                            continue;
                        }
                        if item_slot == 0xffff_ffff {
                            found_material = e.call(0x0062_05a0, &args![part]).u32();
                        } else {
                            let sub = e.vcall(part, 0xcc, &args![]).u32();
                            if sub != 0 {
                                found_material = e.vcall(sub, 0xe0, &args![item_slot]).u32();
                            }
                        }
                        if found_material != material {
                            continue;
                        }
                        let key = e.call(0x0043_b540, &args![object]).u32();
                        let stored = e.call(0x008c_71b0, &args![sink, key]).u32();
                        let is_landscape = e.call(0x004a_3a20, &args![stored]).u32() == 1;
                        let target = if is_landscape {
                            let tes = e.global::<u32>(TES_SINGLETON);
                            let mut found = e.call(0x0045_7620, &args![tes, contact]).u32();
                            if found != 0 && e.call(0x0096_11e0, &args![found]).u32() != 0 {
                                found = e.call(0x0096_11e0, &args![found]).u32();
                            }
                            found
                        } else {
                            e.call(0x00c7_fa90, &args![object]).u32()
                        };
                        let target_ref = e.call(0x0056_f930, &args![target]).u32();
                        if !is_landscape
                            && (target_ref == 0
                                || !e.call(0x004a_1060, &args![target_ref, 1u32]).bool())
                        {
                            continue;
                        }
                        let tes = e.global::<u32>(TES_SINGLETON);
                        let spot = e
                            .call(
                                TES_FIND_AT,
                                &args![tes, contact_words[0], contact_words[1], contact_words[2]],
                            )
                            .u32();
                        e.call(0x004a_37b0, &args![decal_info]);
                        set_point(e, decal_info, contact_words);
                        let flat = point_at(e, decal_direction);
                        set_point(e, decal_info + 0xc, flat);
                        e.mem.set_f32(decal_info + 0x38, size);
                        e.mem.set_f32(decal_info + 0x3c, size);
                        let spread = fn_008aff10(e, Ptr::new(impact));
                        e.mem.set_f32(decal_info + 0x40, spread);
                        e.mem.set_u32(decal_info + 0x28, target);
                        e.mem.set_u32(decal_info + 0x30, effect);
                        e.mem.set_f32(decal_info + 0x44, random_angle);
                        e.mem.set_u8(decal_info + 0x70, variant);
                        let byte = e.call(0x004a_4140, &args![impact]).u8();
                        e.mem.set_u8(decal_info + 0x75, byte);
                        let byte = e.call(0x004a_4180, &args![impact]).u8();
                        e.mem.set_u8(decal_info + 0x74, byte);
                        let byte = e.call(0x004a_4100, &args![impact]).u8();
                        e.mem.set_u8(decal_info + 0x73, byte);
                        let value = e.call(0x004a_41c0, &args![impact]).f32();
                        e.mem.set_f32(decal_info + 0x4c, value);
                        let byte = e.call(0x004a_41e0, &args![impact]).u8();
                        e.mem.set_u8(decal_info + 0x76, byte);
                        let value = e.call(0x004a_40e0, &args![impact]).f32();
                        e.mem.set_f32(decal_info + 0x54, value);
                        let value = e.call(0x009a_9350, &args![impact]).f32();
                        e.mem.set_f32(decal_info + 0x58, value);
                        let divisor: f64 = e.global(COLOR_CHANNEL_MAX);
                        let packed = e.call(0x004a_4220, &args![impact]).u32();
                        let red = (f64::from((packed >> 16) as u8) / divisor) as f32;
                        let packed = e.call(0x004a_4220, &args![impact]).u32();
                        let green = (f64::from((packed as u16 >> 8) as u8) / divisor) as f32;
                        let packed = e.call(0x004a_4220, &args![impact]).u32();
                        let blue = (f64::from(packed as u8) / divisor) as f32;
                        let tint = e.call(0x0041_6870, &args![color, blue, green, red]).u32();
                        let tint = point_at(e, tint);
                        set_point(e, decal_info + 0x60, tint);
                        e.call(0x004a_3fe0, &args![spot, decal_info, 1u32, 0u32]);
                    }
                    e.call(0x00cb_9900, &args![collector]);
                }
            }
            if humanoid_effect && hit_ref != 0 && e.vcall(hit_ref, 0x100, &args![]).bool() {
                let base_form = e.call(0x0041_81e0, &args![this]).u32();
                let creature = e
                    .call(
                        DYNAMIC_CAST,
                        &args![base_form, 0u32, TYPE_BASE_FORM, TYPE_CREATURE, 0u32],
                    )
                    .u32();
                if (creature != 0 && e.call(0x005f_bf20, &args![creature]).bool())
                    || e.vcall(actor, 0x218, &args![]).bool()
                {
                    let setting = e
                        .call(0x0040_8d60, &args![SETTING_NO_HUMANOID_EFFECT])
                        .u32();
                    humanoid_effect = e.mem.u8(setting) == 0;
                }
            }
            // The effect name of the impact record, as a reflected spark.
            if humanoid_effect && impact != 0 {
                let name_object = impact + 0x18;
                let name = e.vcall(name_object, 0x14, &args![]).u32();
                if name != 0 {
                    let name = e.vcall(name_object, 0x14, &args![]).u32();
                    if e.mem.u8(name) as i8 != 0 {
                        let words = point_at(e, direction);
                        set_point(e, reflected, words);
                        let dot = e.call(0x004b_6190, &args![reflected, eye_direction]).f32();
                        let spread = e
                            .call(0x004a_3760, &args![reflected_out, dot, eye_direction])
                            .u32();
                        e.call(0x0045_78c0, &args![reflected, spread]);
                        e.call(0x004a_0c10, &args![reflected]);
                        let name = e.vcall(name_object, 0x14, &args![]).u32();
                        let radius = e.call(0x0050_8100, &args![impact]).f32();
                        let cell = e.call(GET_PARENT_CELL, &args![this]).u32();
                        let reflected_words = point_at(e, reflected);
                        e.call(
                            0x0068_90b0,
                            &args![
                                cell,
                                radius,
                                name,
                                reflected_words[0],
                                reflected_words[1],
                                reflected_words[2],
                                contact_words[0],
                                contact_words[1],
                                contact_words[2],
                                1.0f32,
                                7u32,
                                0u32
                            ],
                        );
                    }
                }
            }
            e.call(SOUND_HANDLE_INIT, &args![handle]);
            if impact != 0 && e.call(IMPACT_SOUND_A, &args![impact]).u32() != 0 {
                play_impact_sound(e, impact, IMPACT_SOUND_A, handle, sound_a, contact_words);
            }
            if impact != 0 && e.call(IMPACT_SOUND_B, &args![impact]).u32() != 0 {
                play_impact_sound(e, impact, IMPACT_SOUND_B, handle, sound_b, contact_words);
            }
            if hit_ref != 0 {
                let mut applies = e.vcall(hit_ref, 0x224, &args![]).bool();
                if !applies {
                    let form = e.call(GET_SAVE_FORM, &args![hit_ref]).u32();
                    applies = e.call(0x0040_1170, &args![form]).u32() == 0x28;
                }
                if applies {
                    if e.call(0x0051_7670, &args![body_object]).i32() < 4 {
                        let attacker = e.vcall(actor, 0x168, &args![]).u32();
                        e.vcall(hit_ref, 0x164, &args![attacker]);
                    }
                    e.vcall(hit_ref, 0x144, &args![damage, 0u32]);
                    if e.call(0x0047_7ba0, &args![hit_ref]).bool()
                        && actor == e.global::<u32>(PLAYER_CHARACTER)
                    {
                        let strength: f32 = e.global(HIT_SHAKE_STRENGTH);
                        let player = e.global::<u32>(PLAYER_CHARACTER);
                        e.call(0x0096_3eb0, &args![player, 10u32, strength, 0u32]);
                    }
                }
            }
            hit_anything = true;
            e.call(SOUND_HANDLE_DESTRUCT, &args![handle]);
        } else if actor != e.global::<u32>(PLAYER_CHARACTER) {
            // No physics body: an actor that was hit is damaged directly.
            let first = e.call(0x0046_0140, &args![caster]).u32();
            let entry = e.call(0x0080_7770, &args![first, 0u32]).u32();
            let collidable = e.mem.u32(entry + 0x48);
            let owner_body = e.call(0x004b_5950, &args![collidable]).u32();
            let other = if owner_body == 0 {
                0
            } else {
                e.call(0x004b_5a80, &args![owner_body]).u32()
            };
            if hit_actor != 0 && !e.vcall(hit_ref, 0x22c, &args![0u32]).bool() && other != 0 {
                let class = e.call(0x0043_b4f0, &args![other, sink]).u32();
                if e.call(0x0043_b4d0, &args![class]).u32() == 0x1e {
                    if e.vcall(hit_actor, 0x21c, &args![]).bool() {
                        e.call(
                            0x0083_7520,
                            &args![
                                this,
                                damage,
                                0.0f32,
                                hit_actor,
                                hit_id,
                                0xffff_ffffu32,
                                0xffff_ffffu32,
                                0u32,
                                0u32
                            ],
                        );
                    } else {
                        let armor = e.call(0x0089_d8b0, &args![hit_actor, 0u32]).u32();
                        if armor != 0 {
                            let form = e.call(0x0044_ddc0, &args![armor]).u32();
                            let flag = e.call(0x0051_4410, &args![form]).u8();
                            e.call(
                                0x0083_7520,
                                &args![
                                    this,
                                    damage,
                                    damage,
                                    hit_actor,
                                    hit_id,
                                    u32::from(flag),
                                    0xffff_ffffu32,
                                    1u32,
                                    0u32
                                ],
                            );
                            e.call(0x0044_59e0, &args![armor, 1u32]);
                        } else {
                            e.call(
                                0x0083_7520,
                                &args![
                                    this,
                                    damage,
                                    0.0f32,
                                    hit_actor,
                                    hit_id,
                                    0xffff_ffffu32,
                                    0xffff_ffffu32,
                                    1u32,
                                    0u32
                                ],
                            );
                        }
                    }
                    hit_anything = true;
                }
            }
        }
        if actor == e.global::<u32>(PLAYER_CHARACTER) {
            let hit_cell = if hit_ref != 0 {
                e.call(GET_PARENT_CELL, &args![hit_ref]).u32()
            } else {
                0
            };
            let hit_physics = if hit_cell != 0 {
                e.call(0x0045_43c0, &args![hit_cell]).u32()
            } else {
                0
            };
            let hit_form = if hit_physics != 0 {
                let form = e.call(GET_SAVE_FORM, &args![hit_physics]).u32();
                if form == 0 {
                    0
                } else {
                    form.wrapping_sub(8)
                }
            } else {
                0
            };
            if hit_form != 0
                && hit_ref == e.call(0x0070_3350, &args![]).u32()
                && e.call(0x0062_dee0, &args![hit_form, hit_ref]).bool()
            {
                let list = e.call(GET_EXTRA_LIST, &args![hit_ref]).u32();
                if list != 0 {
                    e.call(SET_ACTION_FLAG, &args![hit_ref, list, 0x1000_0000u32]);
                    hit_anything = true;
                }
            }
        }
    });
    hit_anything
}

/// This part's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x008a50d0, fn_008a50d0(Ptr, f32)),
        entry!(0x008a50f0, fn_008a50f0(Ptr, Ptr)),
        entry!(0x008a5110, fn_008a5110(Ptr, Ptr)),
        entry!(0x008a5170, fn_008a5170(Ptr) -> bool),
        entry!(0x008a5190, fn_008a5190() -> u32),
        entry!(0x008a51a0, fn_008a51a0(Ptr<Actor>) -> u32),
        entry!(0x008a51f0, fn_008a51f0(Ptr<Actor>, u32)),
        entry!(0x008a5230, fn_008a5230(Ptr<Actor>)),
        entry!(0x008a5270, fn_008a5270(Ptr<Actor>) -> u32),
        entry!(0x008a52c0, fn_008a52c0(Ptr<Actor>, u32)),
        entry!(0x008a5300, fn_008a5300(Ptr<Actor>)),
        entry!(
            0x008a5340,
            actor_get_weapon_position(Ptr<Actor>, Ptr, u8) -> Ptr
        ),
        entry!(0x008a5530, fn_008a5530(Ptr<Actor>)),
        entry!(0x008a5580, fn_008a5580(Ptr<Actor>, f32, Ptr, u8)),
        entry!(0x008a5cf0, fn_008a5cf0(Ptr<Actor>, u8)),
        entry!(0x008a5d10, fn_008a5d10(Ptr<Actor>, f32)),
        entry!(0x008a5e40, actor_set_alert(Ptr<Actor>, u8)),
        entry!(0x008a5e80, actor_get_alert(Ptr<Actor>) -> bool),
        entry!(0x008a5eb0, actor_set_havok_weapon(Ptr<Actor>) -> u32),
        entry!(
            0x008a5f20,
            actor_set_havok_weapon_ov2(Ptr<Actor>, u8) -> u32
        ),
        entry!(0x008a6170, fn_008a6170(Ptr<Actor>) -> bool),
        entry!(0x008a61b0, actor_is_alarmed(Ptr<Actor>) -> bool),
        entry!(0x008a6210, fn_008a6210(Ptr<Actor>) -> bool),
        entry!(0x008a6290, fn_008a6290(Ptr) -> bool),
        entry!(0x008a62b0, actor_move(Ptr<Actor>, f32, u32, u32) -> Ptr),
        entry!(0x008a6630, fn_008a6630(Ptr)),
        entry!(0x008a6650, actor_is_fleeing(Ptr<Actor>, u8) -> bool),
        entry!(0x008a6730, actor_remove_flee_target(Ptr<Actor>, u32)),
        entry!(0x008a67d0, fn_008a67d0(Ptr) -> bool),
        entry!(0x008a67f0, actor_is_talking(Ptr<Actor>) -> bool),
        entry!(0x008a6840, fn_008a6840(Ptr<Actor>, u8)),
        entry!(0x008a6970, fn_008a6970(Ptr<Actor>) -> bool),
        entry!(
            0x008a69d0,
            actor_is_continuing_package_for_pc(Ptr<Actor>) -> bool
        ),
        entry!(0x008a6a00, fn_008a6a00(Ptr<Actor>, f32)),
        entry!(0x008a6a40, actor_unlock_lock_doors_procedure(Ptr<Actor>)),
        entry!(0x008a6ce0, actor_evaluate_package(Ptr<Actor>, u8, u8)),
        entry!(0x008a73e0, actor_set_anim_action(Ptr<Actor>, i32, u32)),
        entry!(0x008a7570, actor_get_anim_action(Ptr<Actor>) -> i32),
        entry!(0x008a75a0, fn_008a75a0(Ptr<Actor>)),
        entry!(0x008a7760, fn_008a7760(Ptr<Actor>)),
        entry!(0x008a7870, actor_is_eating(Ptr<Actor>) -> bool),
        entry!(
            0x008a78f0,
            actor_should_skip_fall_out_behavior(Ptr<Actor>, u32) -> bool
        ),
        entry!(0x008a7a40, actor_is_surfacing(Ptr<Actor>) -> bool),
        entry!(0x008a7a90, fn_008a7a90(Ptr<Actor>)),
        entry!(0x008a7d50, actor_trigger_pain(Ptr<Actor>, u8, u8)),
        entry!(0x008a7f40, fn_008a7f40(Ptr, f32)),
        entry!(0x008a7f60, fn_008a7f60(Ptr) -> f32),
        entry!(0x008a7f80, fn_008a7f80(Ptr<Actor>) -> u32),
        entry!(0x008a8010, fn_008a8010(Ptr<Actor>, u32)),
        entry!(0x008a8060, fn_008a8060(Ptr<Actor>)),
        entry!(0x008a80c0, actor_set_has_been_eaten(Ptr<Actor>, u8)),
        entry!(
            0x008a8300,
            actor_is_close_to_ground(Ptr<Actor>, f32) -> bool
        ),
        entry!(
            0x008a83c0,
            fn_008a83c0(Ptr<Actor>, u32, u32, u8, u32) -> bool
        ),
        entry!(0x008a8420, fn_008a8420(Ptr<Actor>, u32, i32, u8) -> bool),
        entry!(0x008a8820, fn_008a8820(Ptr, u16)),
        entry!(0x008a8840, actor_queue_reload(Ptr<Actor>)),
        entry!(0x008a8870, fn_008a8870(Ptr<Actor>) -> bool),
        entry!(0x008a88b0, fn_008a88b0(Ptr<Actor>) -> u32),
        entry!(
            0x008a88f0,
            actor_get_current_ammo_clip_percent(Ptr<Actor>) -> f32
        ),
        entry!(0x008a89a0, fn_008a89a0(Ptr<Actor>, u32) -> u32),
        entry!(0x008a8dd0, actor_should_use_ammo(Ptr<Actor>, u32) -> bool),
        entry!(0x008a8e30, fn_008a8e30(Ptr) -> bool),
        entry!(0x008a8e50, actor_stop_attack(Ptr<Actor>)),
        entry!(0x008a8f60, fn_008a8f60(Ptr<Actor>)),
        entry!(0x008a9020, fn_008a9020(Ptr<Actor>, u32) -> u16),
        entry!(0x008a9380, fn_008a9380(Ptr<Actor>, u32)),
        entry!(0x008a9940, fn_008a9940(Ptr<Actor>, u32, u32)),
        entry!(0x008aa210, fn_008aa210(Ptr<Actor>, u32)),
        entry!(0x008aa4c0, fn_008aa4c0(Ptr<Actor>, u32, u32)),
        entry!(0x008aa9a0, fn_008aa9a0(Ptr<Actor>, u32, u32)),
        entry!(0x008aad40, fn_008aad40(Ptr<Actor>, u32) -> bool),
        entry!(0x008aaee0, fn_008aaee0(Ptr<Actor>, u32)),
        entry!(0x008aaf40, actor_save_game(Ptr<Actor>, u32)),
        entry!(0x008ab3a0, fn_008ab3a0(Ptr<Actor>, u32)),
        entry!(0x008abc40, fn_008abc40(Ptr<Actor>, u32)),
        entry!(0x008abfa0, fn_008abfa0(Ptr, u32)),
        entry!(0x008abfe0, fn_008abfe0(Ptr<Actor>, u32)),
        entry!(0x008ac1e0, fn_008ac1e0(Ptr<Actor>, u32)),
        entry!(0x008ac680, fn_008ac680(Ptr<Actor>) -> bool),
        entry!(
            0x008ac6f0,
            actor_is_in_combatant_faction(Ptr<Actor>) -> bool
        ),
        entry!(0x008ac810, fn_008ac810(Ptr<Actor>) -> u32),
        entry!(0x008ac850, fn_008ac850(Ptr<Actor>, u32)),
        entry!(0x008ac890, fn_008ac890(Ptr, u32)),
        entry!(0x008ac8b0, fn_008ac8b0(Ptr) -> f32),
        entry!(0x008ac990, fn_008ac990(Ptr, f32)),
        entry!(0x008ac9d0, fn_008ac9d0(Ptr) -> f32),
        entry!(0x008acb70, fn_008acb70(Ptr, f32)),
        entry!(0x008acba0, fn_008acba0(Ptr<Actor>)),
        entry!(
            0x008acbe0,
            actor_calculate_combat_strength(Ptr<Actor>, f32) -> f32
        ),
        entry!(0x008acc80, fn_008acc80(Ptr<Actor>)),
        entry!(0x008ace10, fn_008ace10(Ptr<Actor>, bool)),
        entry!(0x008ace50, fn_008ace50(Ptr<Actor>, bool)),
        entry!(0x008ace90, fn_008ace90(Ptr<Actor>) -> u8),
        entry!(0x008aced0, fn_008aced0(Ptr, u32) -> bool),
        entry!(0x008acf10, actor_set_ghost(Ptr<Actor>, u8)),
        entry!(0x008acf70, fn_008acf70(Ptr, u32, u8)),
        entry!(0x008acfa0, fn_008acfa0(Ptr, u32, u8)),
        entry!(
            0x008acff0,
            fn_008acff0(Ptr<Actor>, u32, u32, u32, Ptr, u32, u32)
        ),
        entry!(
            0x008ad1c0,
            fn_008ad1c0(Ptr<Actor>, Ptr, f32, u32, u32, bool, bool)
        ),
        entry!(0x008ad8f0, fn_008ad8f0(Ptr, u8)),
        entry!(0x008ad910, fn_008ad910(Ptr<Actor>, Ptr, bool, bool)),
        entry!(
            0x008ad970,
            fn_008ad970(Ptr<Actor>, u32, u32, u32, u32, bool)
        ),
        entry!(0x008adc20, fn_008adc20(Ptr, u32, u32, u32)),
        entry!(0x008adc50, fn_008adc50(Ptr<Actor>)),
        entry!(0x008adcb0, actor_get_face_animation_data(Ptr<Actor>) -> u32),
        entry!(
            0x008adcf0,
            actor_get_pick_up_sound_name(Ptr<Actor>, Ptr, u8, u8) -> u32
        ),
        entry!(0x008aded0, fn_008aded0(Ptr<Actor>, Ptr, u8, u8)),
        entry!(0x008ae000, fn_008ae000(Ptr<Actor>, Ptr)),
        entry!(0x008ae480, fn_008ae480(Ptr) -> f32),
        entry!(0x008ae4a0, fn_008ae4a0(Ptr) -> f32),
        entry!(
            0x008ae4c0,
            actor_get_location_on_reference(Ptr<Actor>) -> u32
        ),
        entry!(0x008ae600, fn_008ae600(Ptr<Actor>) -> u32),
        entry!(0x008ae620, fn_008ae620(Ptr<Actor>, u32)),
        entry!(0x008ae640, actor_set_force_update_quest(Ptr<Actor>, u8)),
        entry!(0x008afef0, fn_008afef0(Ptr) -> u32),
        entry!(0x008aff10, fn_008aff10(Ptr) -> f32),
        entry!(0x008aff30, fn_008aff30(Ptr) -> Ptr),
        entry!(0x008aff50, fn_008aff50(Ptr, u32) -> Ptr),
        entry!(0x008aff80, actor_get_eye_vector(Ptr<Actor>, Ptr, Ptr, bool)),
        entry!(0x008ae660, fn_008ae660(Ptr<Actor>, u8) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Vtables of the fake actor, process and other objects. Slot `s` of
    /// the table at `at` points at the fake function [`target`]`(at, s)`.
    const ACTOR_VT: u32 = 0x0900_0000;
    const PROCESS_VT: u32 = 0x0902_0000;
    const OBJECT_VT: u32 = 0x0904_0000;

    fn target(at: u32, slot: u32) -> u32 {
        at + 0x1_0000 + slot
    }

    /// A vtable at `at` with the given slots, each pointing at its fake
    /// function, which does nothing and returns 0 until a test replaces it.
    fn make_vtable(e: &mut Engine, at: u32, slots: &[u32]) {
        let len = slots.iter().max().unwrap() / 4 + 1;
        let mut words = vec![0u32; len as usize];
        for slot in slots {
            words[(*slot / 4) as usize] = target(at, *slot);
            e.register(target(at, *slot), |_, _| Ret::default());
        }
        e.put_vtable(at, &words);
    }

    /// Every actor vtable slot any test of this file calls.
    const ACTOR_SLOTS: &[u32] = &[
        0x13c, 0x140, 0x178, 0x1d0, 0x1e4, 0x1e8, 0x1f4, 0x214, 0x218, 0x22c, 0x230, 0x268, 0x27c,
        0x284, 0x2e8, 0x2f4, 0x2f8, 0x338, 0x340, 0x348, 0x350, 0x360, 0x3ac, 0x418, 0x428, 0x434,
        0x4c0, 0x390, 0x1d8, 0x1dc, 0x440, 0x1c4, 0x48, 0x2bc, 0x2c4, 0x2a8, 0x100, 0x144, 0x164,
        0x168, 0x21c, 0x224, 0x444,
    ];
    /// Every process vtable slot any test of this file calls.
    const PROCESS_SLOTS: &[u32] = &[
        0x24, 0x28, 0x44, 0x100, 0x118, 0x128, 0x148, 0x15c, 0x190, 0x1bc, 0x1c0, 0x20c, 0x22c,
        0x234, 0x27c, 0x280, 0x2e8, 0x318, 0x31c, 0x320, 0x360, 0x3e4, 0x3ec, 0x44c, 0x450, 0x48c,
        0x4b8, 0x4c0, 0x4d0, 0x4e0, 0x4ec, 0x524, 0x59c, 0x5a8, 0x718, 0x770, 0x774, 0x778, 0x77c,
        0x780, 0x784, 0x428, 0x434, 0x418, 0x2f4, 0x3ac, 0x338, 0x1f4, 0x13c, 0x11c, 0xb4, 0x144,
        0x4bc, 0x4c4, 0x4c8, 0x4d4, 0x540, 0xec, 0xdc, 0x288, 0x160, 0x168, 0x460,
    ];

    fn engine() -> Engine {
        let mut e = Engine::new();
        // The pages of the exe's data these functions read.
        for page in [
            0x0101_2000,
            0x0101_6000,
            0x0102_3000,
            0x0102_b000,
            0x0103_0000,
            0x0106_b000,
            0x0118_8000,
            0x0119_7000,
            0x011a_3000,
            0x011c_3000,
            0x011c_6000,
            0x011d_d000,
            0x011d_e000,
            0x011d_f000,
            0x011f_4000,
            0x0101_d000,
            0x0101_7000,
            0x0106_6000,
            0x0101_f000,
            0x011c_d000,
            0x0101_1000,
            0x0101_8000,
            0x0101_e000,
            0x0102_0000,
            0x0102_4000,
            0x011e_0000,
            0x011c_a000,
            0x011c_e000,
            0x011c_f000,
            0x011d_0000,
            0x011d_1000,
            0x0102_1000,
            0x0102_9000,
        ] {
            e.map(page, 0x1000);
        }
        make_vtable(&mut e, ACTOR_VT, ACTOR_SLOTS);
        make_vtable(&mut e, PROCESS_VT, PROCESS_SLOTS);
        make_vtable(
            &mut e,
            OBJECT_VT,
            &[
                0x8, 0x11c, 0xb4, 0x100, 0x13c, 0x128, 0x1f4, 0xd8, 0x1d0, 0x130, 0x21c, 0x38c,
                0x224, 0x144, 0x164, 0x22c,
            ],
        );
        // `008d8520`: the process field.
        e.register(GET_PROCESS, |e, a| e.mem.u32(a[0] + 0x68).into_ret());
        e
    }

    fn new_actor(e: &mut Engine) -> Ptr<Actor> {
        let actor: Ptr<Actor> = e.new_object();
        e.mem.set_u32(actor.addr(), ACTOR_VT);
        actor
    }

    /// An actor with a process.
    fn new_actor_with_process(e: &mut Engine) -> (Ptr<Actor>, Ptr) {
        let actor = new_actor(e);
        let proc = Ptr::new(e.mem.alloc(0x800));
        e.mem.set_u32(proc.addr(), PROCESS_VT);
        e.set(actor, Actor::pCurrentProcess, proc);
        (actor, proc)
    }

    fn new_object(e: &mut Engine) -> Ptr {
        let object: Ptr = Ptr::new(e.mem.alloc(0x100));
        e.mem.set_u32(object.addr(), OBJECT_VT);
        object
    }

    /// Makes the fake function behind a vtable slot return `value`.
    fn answer(e: &mut Engine, at: u32, slot: u32, value: u32) {
        e.register_double(target(at, slot), move |_, _| Ret {
            eax: value,
            ..Ret::default()
        });
    }

    /// Does nothing, returns 0.
    fn stub(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    /// Returns the calls recorded since `e.call_log = Some(vec![])`.
    fn calls(e: &mut Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log.take().unwrap_or_default()
    }

    /// The argument lists of the calls to `addr`.
    fn called(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    #[test]
    fn word_store_puts_the_float_in_word_three() {
        let mut e = engine();
        e.register(0x0056_0d30, |_, a| (a[0] + 4 * a[1]).into_ret());
        let object: Ptr = Ptr::new(e.mem.alloc(0x20));
        e.call(0x008a_50d0, &args![object, 2.5f32]);
        assert_eq!(e.mem.f32(object.addr() + 12), 2.5);
        assert_eq!(e.mem.f32(object.addr() + 8), 0.0);
    }

    #[test]
    fn vector_copy_keeps_the_fourth_float() {
        let mut e = engine();
        let source: Ptr = Ptr::new(e.mem.alloc(0x10));
        let dest: Ptr = Ptr::new(e.mem.alloc(0x10));
        for (i, v) in [1.0f32, 2.0, 3.0, 4.0].iter().enumerate() {
            e.mem.set_f32(source.addr() + 4 * i as u32, *v);
            e.mem.set_f32(dest.addr() + 4 * i as u32, 9.0);
        }
        e.call(0x008a_5110, &args![dest, source]);
        let words: Vec<f32> = (0..4).map(|i| e.mem.f32(dest.addr() + 4 * i)).collect();
        assert_eq!(words, vec![1.0, 2.0, 3.0, 9.0]);
        // The thunk forwards.
        e.mem.set_f32(dest.addr(), 0.0);
        e.call(0x008a_50f0, &args![dest, source]);
        assert_eq!(e.mem.f32(dest.addr()), 1.0);
    }

    #[test]
    fn flag_test_ands_word_one_with_one() {
        let mut e = engine();
        e.register(0x0062_1270, |e, a| (e.mem.u32(a[0] + 4) & a[1]).into_ret());
        let object: Ptr = Ptr::new(e.mem.alloc(0x10));
        assert!(!e.call(0x008a_5170, &args![object]).bool());
        e.mem.set_u32(object.addr() + 4, 3);
        assert!(e.call(0x008a_5170, &args![object]).bool());
        e.mem.set_u32(object.addr() + 4, 2);
        assert!(!e.call(0x008a_5170, &args![object]).bool());
    }

    #[test]
    fn global_word_is_returned() {
        let mut e = engine();
        e.set_global(0x011c_6254, 0x1234u32);
        assert_eq!(e.call(0x008a_5190, &args![]).u32(), 0x1234);
    }

    /// The six process forwarders: (address, slot, takes an argument,
    /// returns a value).
    const FORWARDERS: [(u32, u32, bool, bool); 6] = [
        (0x008a_51a0, 0x77c, false, true),
        (0x008a_51f0, 0x780, true, false),
        (0x008a_5230, 0x784, false, false),
        (0x008a_5270, 0x770, false, true),
        (0x008a_52c0, 0x774, true, false),
        (0x008a_5300, 0x778, false, false),
    ];

    #[test]
    fn process_forwarders_call_their_slot() {
        for (addr, slot, takes_argument, returns) in FORWARDERS {
            let mut e = engine();
            let (actor, proc) = new_actor_with_process(&mut e);
            answer(&mut e, PROCESS_VT, slot, 77);
            e.call_log = Some(vec![]);
            let words = if takes_argument {
                args![actor, 5u32]
            } else {
                args![actor]
            };
            let result = e.call(addr, &words).u32();
            let log = calls(&mut e);
            let slot_calls = called(&log, target(PROCESS_VT, slot));
            assert_eq!(slot_calls.len(), 1, "{addr:08x}");
            assert_eq!(slot_calls[0][0], proc.addr());
            if takes_argument {
                assert_eq!(slot_calls[0][1], 5);
            }
            if returns {
                assert_eq!(result, 77, "{addr:08x}");
            }
        }
    }

    #[test]
    fn process_forwarders_do_nothing_without_a_process() {
        for (addr, slot, takes_argument, _) in FORWARDERS {
            let mut e = engine();
            let actor = new_actor(&mut e);
            e.call_log = Some(vec![]);
            let words = if takes_argument {
                args![actor, 5u32]
            } else {
                args![actor]
            };
            let result = e.call(addr, &words).u32();
            let log = calls(&mut e);
            assert!(called(&log, target(PROCESS_VT, slot)).is_empty());
            assert_eq!(result, 0);
        }
    }

    /// A block of `n` floats.
    fn floats(e: &mut Engine, values: &[f32]) -> Ptr {
        let block: Ptr = Ptr::new(e.mem.alloc(4 * values.len().max(13) as u32));
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(block.addr() + 4 * i as u32, *v);
        }
        block
    }

    fn point(e: &Engine, at: Ptr) -> [f32; 3] {
        [
            e.mem.f32(at.addr()),
            e.mem.f32(at.addr() + 4),
            e.mem.f32(at.addr() + 8),
        ]
    }

    /// Doubles for the transform helpers of `Actor::GetWeaponPosition`:
    /// a transform's first three words stand for its translation.
    fn weapon_position_engine() -> (Engine, Ptr<Actor>, Ptr, Ptr) {
        let mut e = engine();
        let (actor, _) = new_actor_with_process(&mut e);
        let root = floats(&mut e, &[10.0, 20.0, 30.0]);
        let node = floats(&mut e, &[1.0, 2.0, 3.0]);
        answer(&mut e, ACTOR_VT, 0x1d0, root.addr());
        answer(&mut e, ACTOR_VT, 0x1e8, 0x55);
        answer(&mut e, PROCESS_VT, 0x148, 0x1234);
        answer(&mut e, PROCESS_VT, 0x190, node.addr());
        e.register(0x0044_ddc0, |_, a| a[0].into_ret());
        e.register(0x0064_47f0, |_, _| 10.0f32.into_ret());
        e.register(0x0064_63e0, |_, a| (f32::from_bits(a[0]) * 2.0).into_ret());
        // The node's transform is the node itself.
        e.register(0x0046_1130, |_, a| a[0].into_ret());
        e.register(0x0041_6870, |e, a| {
            for i in 1..4 {
                e.mem.set_u32(a[0] + 4 * (i - 1), a[i as usize]);
            }
            a[0].into_ret()
        });
        e.register(0x0052_4c40, |e, a| {
            for i in 0..3 {
                let sum = e.mem.f32(a[0] + 4 * i) + e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, sum);
            }
            Ret::default()
        });
        // The inverse is marked, and the product adds 100 to word 0.
        stub(&mut e, &[0x0047_6a80]);
        e.register(0x004b_4880, |e, a| {
            e.mem.set_f32(a[1], -1.0);
            Ret::default()
        });
        e.register(0x0062_c250, |e, a| {
            for i in 0..13 {
                let word = e.mem.u32(a[2] + 4 * i);
                e.mem.set_u32(a[1] + 4 * i, word);
            }
            let x = e.mem.f32(a[1]) + 100.0 * e.mem.f32(a[0]);
            e.mem.set_f32(a[1], x);
            a[1].into_ret()
        });
        e.set_global(ZERO_POINT, 7.0f32);
        e.set_global(ZERO_POINT + 4, 8.0f32);
        e.set_global(ZERO_POINT + 8, 9.0f32);
        let result: Ptr = Ptr::new(e.mem.alloc(0x10));
        (e, actor, result, root)
    }

    #[test]
    fn weapon_position_is_the_reach_point_through_the_node_transform() {
        let (mut e, actor, result, _) = weapon_position_engine();
        let back = e.call(0x008a_5340, &args![actor, result, 1u8]).ptr::<()>();
        assert_eq!(back, result);
        // reach 20; translation (1, 2, 3): (0 + 1, 20 + 2, 0 + 3).
        assert_eq!(point(&e, result), [1.0, 22.0, 3.0]);
    }

    #[test]
    fn weapon_position_in_root_space_multiplies_by_the_inverse() {
        let (mut e, actor, result, _) = weapon_position_engine();
        e.call_log = Some(vec![]);
        e.call(0x008a_5340, &args![actor, result, 0u8]);
        let log = calls(&mut e);
        let order: Vec<u32> = log
            .iter()
            .map(|(a, _)| *a)
            .filter(|a| [0x0047_6a80, 0x004b_4880, 0x0062_c250, 0x0052_4c40].contains(a))
            .collect();
        assert_eq!(
            order,
            vec![0x0047_6a80, 0x004b_4880, 0x0062_c250, 0x0052_4c40]
        );
        // The inverse is built from the root's transform (a double marks
        // its word 0 as -1): the product's word 0 is 1 + 100 * -1 = -99.
        assert_eq!(point(&e, result), [-99.0 + 0.0, 22.0, 3.0]);
    }

    #[test]
    fn weapon_position_without_weapon_has_no_reach() {
        let (mut e, actor, result, _) = weapon_position_engine();
        answer(&mut e, PROCESS_VT, 0x148, 0);
        e.call(0x008a_5340, &args![actor, result, 1u8]);
        assert_eq!(point(&e, result), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn weapon_position_falls_back_to_the_zero_point() {
        // No 3D.
        let (mut e, actor, result, _) = weapon_position_engine();
        answer(&mut e, ACTOR_VT, 0x1d0, 0);
        e.call(0x008a_5340, &args![actor, result, 1u8]);
        assert_eq!(point(&e, result), [7.0, 8.0, 9.0]);
        // No weapon node.
        let (mut e, actor, result, _) = weapon_position_engine();
        answer(&mut e, PROCESS_VT, 0x190, 0);
        e.call(0x008a_5340, &args![actor, result, 1u8]);
        assert_eq!(point(&e, result), [7.0, 8.0, 9.0]);
    }

    #[test]
    fn face_reset_passes_the_fade() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        let face = new_object(&mut e);
        e.set_global(FACE_FADE, 0.75f32);
        e.register(GET_FACE_ANIMATION_DATA, |e, _| {
            e.mem.u32(0x011d_f000).into_ret()
        });
        e.set_global(0x011d_f000, face.addr());
        e.call_log = Some(vec![]);
        e.call(0x008a_5530, &args![actor]);
        let log = calls(&mut e);
        let call = called(&log, target(OBJECT_VT, 0xb4));
        assert_eq!(
            call,
            vec![vec![face.addr(), 0.75f32.to_bits(), 0, 1, 1, 1, 0]]
        );
        // Without face data nothing is called.
        e.set_global(0x011d_f000, 0u32);
        e.call_log = Some(vec![]);
        e.call(0x008a_5530, &args![actor]);
        assert!(called(&calls(&mut e), target(OBJECT_VT, 0xb4)).is_empty());
    }

    #[test]
    fn use_emotion_byte_is_stored() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        e.call(0x008a_5cf0, &args![actor, 1u8]);
        assert_eq!(e.get(actor, Actor::bUseEmotion), 1);
        e.call(0x008a_5cf0, &args![actor, 0u8]);
        assert_eq!(e.get(actor, Actor::bUseEmotion), 0);
    }

    #[test]
    fn alert_goes_through_the_process() {
        let mut e = engine();
        let (actor, proc) = new_actor_with_process(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x008a_5e40, &args![actor, 1u8]);
        let log = calls(&mut e);
        assert_eq!(
            called(&log, target(PROCESS_VT, 0x320)),
            vec![vec![proc.addr(), 1]]
        );
        answer(&mut e, PROCESS_VT, 0x31c, 1);
        assert!(e.call(0x008a_5e80, &args![actor]).bool());
        answer(&mut e, PROCESS_VT, 0x31c, 0);
        assert!(!e.call(0x008a_5e80, &args![actor]).bool());
        // No process: nothing is called and the alert is off.
        let bare = new_actor(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x008a_5e40, &args![bare, 1u8]);
        assert!(!e.call(0x008a_5e80, &args![bare]).bool());
        let log = calls(&mut e);
        assert!(called(&log, target(PROCESS_VT, 0x320)).is_empty());
    }

    #[test]
    fn current_package_type_tests() {
        // 008a6170: type 0x18; 008a61b0: process package of type 0x15.
        let mut e = engine();
        let (actor, _) = new_actor_with_process(&mut e);
        let package: Ptr = Ptr::new(e.mem.alloc(0x40));
        e.register(FORM_TYPE, |e, a| (e.mem.i8(a[0] + 0x20) as i32).into_ret());
        e.register(GET_CURRENT_PACKAGE, |e, a| {
            let _ = a;
            e.mem.u32(0x011d_f000).into_ret()
        });
        e.set_global(0x011d_f000, package.addr());
        e.mem.set_u8(package.addr() + 0x20, 0x18);
        assert!(e.call(0x008a_6170, &args![actor]).bool());
        e.mem.set_u8(package.addr() + 0x20, 0x17);
        assert!(!e.call(0x008a_6170, &args![actor]).bool());
        e.set_global(0x011d_f000, 0u32);
        assert!(!e.call(0x008a_6170, &args![actor]).bool());

        answer(&mut e, PROCESS_VT, 0x22c, package.addr());
        e.mem.set_u8(package.addr() + 0x20, 0x15);
        assert!(e.call(0x008a_61b0, &args![actor]).bool());
        e.mem.set_u8(package.addr() + 0x20, 0x16);
        assert!(!e.call(0x008a_61b0, &args![actor]).bool());
        answer(&mut e, PROCESS_VT, 0x22c, 0);
        assert!(!e.call(0x008a_61b0, &args![actor]).bool());
        let bare = new_actor(&mut e);
        assert!(!e.call(0x008a_61b0, &args![bare]).bool());
    }

    #[test]
    fn package_flag_bit_is_0x10000_of_word_0x1c() {
        let mut e = engine();
        let object: Ptr = Ptr::new(e.mem.alloc(0x40));
        assert!(!e.call(0x008a_6290, &args![object]).bool());
        e.mem.set_u32(object.addr() + 0x1c, 0x1_0000);
        assert!(e.call(0x008a_6290, &args![object]).bool());
        e.mem.set_u32(object.addr() + 0x1c, 0xffff);
        assert!(!e.call(0x008a_6290, &args![object]).bool());
    }

    #[test]
    fn package_table_check() {
        let mut e = engine();
        let (actor, _) = new_actor_with_process(&mut e);
        let package: Ptr = Ptr::new(e.mem.alloc(0x40));
        e.register(GET_CURRENT_PACKAGE, |e, _| {
            e.mem.u32(0x011d_f000).into_ret()
        });
        e.register(0x0096_11e0, |e, a| e.mem.u32(a[0] + 0x18).into_ret());
        e.set_global(0x011d_f000, package.addr());
        // Table 1 holds the words (3, 0xd, 5).
        let table = floats(&mut e, &[]);
        e.mem.set_u32(table.addr(), 3);
        e.mem.set_u32(table.addr() + 4, 0xd);
        e.mem.set_u32(table.addr() + 8, 5);
        e.set_global(PACKAGE_TYPE_TABLE + 4, table.addr());
        e.mem.set_u32(package.addr() + 0x18, 1);
        // Process method +0x280 picks the entry.
        answer(&mut e, PROCESS_VT, 0x280, 1);
        assert!(e.call(0x008a_6210, &args![actor]).bool());
        answer(&mut e, PROCESS_VT, 0x280, 2);
        assert!(!e.call(0x008a_6210, &args![actor]).bool());
        // The flag in the package overrides the table.
        e.mem.set_u32(package.addr() + 0x1c, 0x1_0000);
        assert!(e.call(0x008a_6210, &args![actor]).bool());
        // Index -1 and no package are false.
        e.mem.set_u32(package.addr() + 0x18, u32::MAX);
        assert!(!e.call(0x008a_6210, &args![actor]).bool());
        e.set_global(0x011d_f000, 0u32);
        assert!(!e.call(0x008a_6210, &args![actor]).bool());
    }

    /// The addresses of `log`'s calls that are in `of`, in order.
    fn order(log: &[(u32, Vec<u32>)], of: &[u32]) -> Vec<u32> {
        log.iter()
            .map(|(a, _)| *a)
            .filter(|a| of.contains(a))
            .collect()
    }

    #[test]
    fn frame_update_runs_the_bone_relaxation_and_the_virtual_updates() {
        let mut e = engine();
        let (actor, proc) = new_actor_with_process(&mut e);
        let package: Ptr = Ptr::new(e.mem.alloc(0x40));
        let ragdoll: Ptr = Ptr::new(e.mem.alloc(0x40));
        e.set(actor, Actor::pRagdollController, ragdoll);
        answer(&mut e, PROCESS_VT, 0x22c, package.addr());
        answer(&mut e, ACTOR_VT, 0x1e4, 0x6000);
        e.register(FORM_TYPE, |_, _| 0x1c.into_ret());
        e.register(0x0045_cd60, |_, _| Ret::default());
        e.register(0x0089_d620, |_, _| 0x7000.into_ret());
        e.register(0x005f_36f0, |_, _| 1.into_ret());
        e.register(GET_PARENT_CELL, |_, _| 0x8000.into_ret());
        stub(
            &mut e,
            &[0x00c5_2960, 0x0088_8970, 0x0049_3930, 0x0054_baf0],
        );
        e.call_log = Some(vec![]);
        e.call(0x008a_5d10, &args![actor, 0.5f32]);
        let log = calls(&mut e);
        assert_eq!(called(&log, 0x00c5_2960), vec![vec![0x7000, 0]]);
        assert_eq!(called(&log, 0x0088_8970), vec![vec![actor.addr(), 0]]);
        assert_eq!(called(&log, 0x0049_3930), vec![vec![0x6000]]);
        assert_eq!(called(&log, 0x0054_baf0), vec![vec![0x8000]]);
        assert_eq!(
            called(&log, target(ACTOR_VT, 0x268)),
            vec![vec![actor.addr(), 0.5f32.to_bits()]]
        );
        assert_eq!(
            called(&log, target(ACTOR_VT, 0x2f8)),
            vec![vec![actor.addr(), 0.5f32.to_bits()]]
        );
        let slots = [0x268, 0x350, 0x2f8, 0x178].map(|s| target(ACTOR_VT, s));
        assert_eq!(order(&log, &slots), slots.to_vec());
        let _ = proc;
    }

    #[test]
    fn frame_update_skips_what_is_missing() {
        let mut e = engine();
        let (actor, _) = new_actor_with_process(&mut e);
        // The process is busy, so the bones stay; no animation, ragdoll or cell.
        e.register(0x0045_cd60, |_, _| 1.into_ret());
        stub(
            &mut e,
            &[
                0x00c5_2960,
                0x0088_8970,
                0x0049_3930,
                0x0054_baf0,
                GET_PARENT_CELL,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x008a_5d10, &args![actor, 0.5f32]);
        let log = calls(&mut e);
        for addr in [0x00c5_2960, 0x0088_8970, 0x0049_3930, 0x0054_baf0] {
            assert!(called(&log, addr).is_empty(), "{addr:08x}");
        }
        assert_eq!(called(&log, target(ACTOR_VT, 0x178)).len(), 1);
    }

    /// Doubles for `SetHavokWeapon_ov2`.
    fn havok_engine() -> (Engine, Ptr<Actor>, Ptr) {
        let mut e = engine();
        let (actor, _) = new_actor_with_process(&mut e);
        let root: Ptr = Ptr::new(e.mem.alloc(0x40));
        e.register(0x008c_7aa0, |_, _| Ret::default());
        e.register(0x0043_fcd0, |_, _| 0x1111.into_ret());
        e.register(GET_CURRENT_WEAPON, |_, _| 0x2222.into_ret());
        e.register(0x0089_1350, |_, _| 0x3333.into_ret());
        e.register(0x004a_de00, |_, a| (a[0] + a[1]).into_ret());
        e.register(0x0093_1ed0, |e, a| {
            e.mem.set_u32(a[1], 0x4444);
            Ret::default()
        });
        e.register(0x004b_5260, |_, _| 0x5555.into_ret());
        e.register(0x0065_3270, |_, _| Ret::default());
        e.register(0x00c8_9540, |_, _| 0x6666.into_ret());
        stub(
            &mut e,
            &[
                0x00c8_9050,
                0x00c8_91c0,
                0x0043_7bd0,
                0x0089_d690,
                0x0087_b6b0,
            ],
        );
        e.register(0x0045_37b0, |_, _| 0x7777.into_ret());
        (e, actor, root)
    }

    #[test]
    fn havok_weapon_is_only_queued_in_the_special_state() {
        let (mut e, actor, _) = havok_engine();
        e.register(0x008c_7aa0, |_, _| 1.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x008a_5f20, &args![actor, 1u8]).u32(), 0);
        let log = calls(&mut e);
        assert_eq!(
            called(&log, 0x0087_b6b0),
            vec![vec![0x7777, actor.addr(), 1]]
        );
        assert!(called(&log, 0x00c8_9540).is_empty());
    }

    #[test]
    fn havok_weapon_creates_the_object_and_updates_it_for_mode_zero() {
        let (mut e, actor, _) = havok_engine();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x008a_5f20, &args![actor, 0u8]).u32(), 0);
        let log = calls(&mut e);
        // The node is `4ade00(root, name)`.
        let node = 0x1111 + 0x3333;
        // Mode 0 with both actor checks false: the flag is 1.
        assert_eq!(
            called(&log, 0x00c8_9540),
            vec![vec![0x4444, 0x1111, node, 1]]
        );
        assert_eq!(called(&log, 0x00c8_9050), vec![vec![0, 0x4444, 0x1111, 1]]);
    }

    #[test]
    fn havok_weapon_creates_only_for_a_nonzero_mode() {
        let (mut e, actor, _) = havok_engine();
        e.call_log = Some(vec![]);
        e.call(0x008a_5f20, &args![actor, 1u8]);
        let log = calls(&mut e);
        let node = 0x1111 + 0x3333;
        // Mode 1, no ragdoll: the flag is 0.
        assert_eq!(
            called(&log, 0x00c8_9540),
            vec![vec![0x4444, 0x1111, node, 0]]
        );
        assert!(called(&log, 0x00c8_9050).is_empty());
    }

    #[test]
    fn havok_weapon_updates_an_existing_object_and_its_children() {
        let (mut e, actor, _) = havok_engine();
        e.register(0x0065_3270, |_, a| (a[1] + 1).into_ret());
        e.register(0x0043_7bd0, |_, _| 1.into_ret());
        // A ragdoll controller whose flag byte is set makes the flag 1.
        let ragdoll: Ptr = Ptr::new(e.mem.alloc(0x40));
        e.set(actor, Actor::pRagdollController, ragdoll);
        e.register(0x0089_d690, |_, _| 1.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x008a_5f20, &args![actor, 1u8]);
        let log = calls(&mut e);
        assert!(called(&log, 0x00c8_9540).is_empty());
        assert_eq!(called(&log, 0x00c8_9050), vec![vec![1, 0x4444, 0x1111, 1]]);
        assert_eq!(called(&log, 0x00c8_91c0), vec![vec![0x5556]]);
    }

    #[test]
    fn havok_weapon_stops_without_3d_weapon_or_node() {
        for missing in 0..3 {
            let (mut e, actor, _) = havok_engine();
            match missing {
                0 => e.register(0x0043_fcd0, |_, _| Ret::default()),
                1 => e.register(GET_CURRENT_WEAPON, |_, _| Ret::default()),
                _ => e.register(0x004a_de00, |_, _| Ret::default()),
            }
            e.call_log = Some(vec![]);
            e.call(0x008a_5f20, &args![actor, 1u8]);
            let log = calls(&mut e);
            assert!(called(&log, 0x00c8_9540).is_empty(), "{missing}");
        }
    }

    #[test]
    fn havok_weapon_of_the_player_uses_the_player_3d() {
        let (mut e, actor, _) = havok_engine();
        e.set_global(PLAYER_CHARACTER, actor.addr());
        e.register(0x0095_0bb0, |_, a| (a[0] ^ 0xff).into_ret());
        e.call_log = Some(vec![]);
        e.call(0x008a_5f20, &args![actor, 1u8]);
        let log = calls(&mut e);
        assert_eq!(called(&log, 0x0095_0bb0), vec![vec![actor.addr(), 0]]);
        assert!(called(&log, 0x0043_fcd0).is_empty());
    }

    #[test]
    fn set_havok_weapon_chooses_the_mode_from_the_ragdoll() {
        let (mut e, actor, _) = havok_engine();
        e.register(0x008c_7aa0, |_, _| 1.into_ret());
        // No ragdoll: 1.
        e.call_log = Some(vec![]);
        e.call(0x008a_5eb0, &args![actor]);
        let log = calls(&mut e);
        assert_eq!(
            called(&log, 0x0087_b6b0),
            vec![vec![0x7777, actor.addr(), 1]]
        );
        // A ragdoll that answers true: 0.
        let ragdoll: Ptr = Ptr::new(e.mem.alloc(0x40));
        e.set(actor, Actor::pRagdollController, ragdoll);
        e.register(0x0055_2490, |_, _| 1.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x008a_5eb0, &args![actor]);
        let log = calls(&mut e);
        assert_eq!(
            called(&log, 0x0087_b6b0),
            vec![vec![0x7777, actor.addr(), 0]]
        );
        // A ragdoll that answers false: 1.
        e.register(0x0055_2490, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x008a_5eb0, &args![actor]);
        let log = calls(&mut e);
        assert_eq!(
            called(&log, 0x0087_b6b0),
            vec![vec![0x7777, actor.addr(), 1]]
        );
        // No weapon: nothing is queued.
        e.register(GET_CURRENT_WEAPON, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x008a_5eb0, &args![actor]).u32(), 0);
        assert!(called(&calls(&mut e), 0x0087_b6b0).is_empty());
    }

    const KNOB: u32 = 0x011d_f000;
    const NO_SPEAKER: Ptr = Ptr::NULL;

    /// Doubles for `Actor::Move`: a fall of 300 against the threshold 100
    /// with exponent 3, multiplier 2 gives the damage `(200 * 3) * 2`.
    fn move_engine() -> (Engine, Ptr<Actor>, Ptr) {
        let mut e = engine();
        for page in [0x011c_f000, 0x011d_0000, 0x011d_1000] {
            e.map(page, 0x1000);
        }
        let (actor, _) = new_actor_with_process(&mut e);
        e.mem.set_u32(actor.addr() + LOADED_DATA, 1);
        let moved: Ptr = Ptr::new(e.mem.alloc(0x500));
        e.set_global(KNOB, moved.addr());
        e.register(0x0093_3840, |_, _| Ret::default());
        e.register(0x0092_f260, |e, _| e.mem.u32(KNOB).into_ret());
        e.register(0x0087_cee0, |_, _| 1.into_ret());
        e.register(0x00c7_0550, |_, _| 300.0f32.into_ret());
        e.register(SETTING_VALUE_POINTER, |_, a| (a[0] + 4).into_ret());
        e.register(0x0043_d4d0, |_, a| (a[0] + 4).into_ret());
        e.register(0x004d_d130, |_, a| {
            (f32::from_bits(a[0]) as f64 * f32::from_bits(a[1]) as f64).into_ret()
        });
        e.set_global(SETTING_FALL_THRESHOLD + 4, 100.0f32);
        e.set_global(SETTING_FALL_MULT + 4, 2.0f32);
        e.set_global(SETTING_FALL_EXPONENT + 4, 3.0f32);
        e.set_global(SETTING_HEAVY_THRESHOLD + 4, 500.0f32);
        e.set_global(SETTING_LIMB_DAMAGE_MULT + 4, 0.5f32);
        e.set_global(SETTING_LIMB_DAMAGE_CHANCE + 4, 50u32);
        e.register(0x0041_81e0, |_, _| Ret::default());
        e.register(0x0056_6950, |_, _| Ret::default());
        e.register(0x0045_3a70, |_, _| 0x9999.into_ret());
        e.register(0x00ad_7550, |_, _| 0xaaaa.into_ret());
        e.register(0x0048_7f50, |_, _| 205.into_ret());
        stub(
            &mut e,
            &[
                0x0093_06d0,
                0x005f_0c40,
                0x0057_5d70,
                0x0058_6320,
                SOUND_HANDLE_INIT,
                SOUND_HANDLE_ASSIGN,
                SOUND_HANDLE_DESTRUCT,
                0x00ad_8830,
                0x0062_96b0,
                0x008a_7d50,
            ],
        );
        (e, actor, moved)
    }

    #[test]
    fn move_applies_fall_damage_and_hurts() {
        let (mut e, actor, moved) = move_engine();
        e.call_log = Some(vec![]);
        let result = e
            .call(0x008a_62b0, &args![actor, 0.5f32, 7u32, 1u32])
            .ptr::<()>();
        assert_eq!(result, moved);
        let log = calls(&mut e);
        assert_eq!(
            called(&log, 0x0092_f260),
            vec![vec![actor.addr(), 0.5f32.to_bits(), 7, 1]]
        );
        // The fall passes the exponent: 200 ^ 3 stand-in, times 2.
        assert_eq!(
            called(&log, 0x004d_d130),
            vec![vec![200.0f32.to_bits(), 3.0f32.to_bits()]]
        );
        assert_eq!(
            called(&log, target(ACTOR_VT, 0x338)),
            vec![vec![actor.addr(), 1200.0f32.to_bits(), 0, 0]]
        );
        assert_eq!(called(&log, 0x008a_7d50), vec![vec![actor.addr(), 1, 1]]);
        // The actor is not in its loaded state: no sound.
        assert!(called(&log, SOUND_HANDLE_INIT).is_empty());
    }

    #[test]
    fn move_skips_the_hurt_call_when_the_world_space_check_says_so() {
        let (mut e, actor, _) = move_engine();
        answer(&mut e, ACTOR_VT, 0x218, 1);
        e.register(0x0057_5d70, |_, _| 0x1234.into_ret());
        e.register(0x0058_6320, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x008a_62b0, &args![actor, 0.5f32, 7u32, 1u32]);
        let log = calls(&mut e);
        assert!(called(&log, target(ACTOR_VT, 0x338)).is_empty());
        assert_eq!(called(&log, 0x008a_7d50).len(), 1);
        // A world space that answers true lets it through again.
        e.register(0x0058_6320, |_, _| 1.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x008a_62b0, &args![actor, 0.5f32, 7u32, 1u32]);
        assert_eq!(called(&calls(&mut e), target(ACTOR_VT, 0x338)).len(), 1);
    }

    #[test]
    fn move_in_the_loaded_state_plays_the_landing_sound_and_hurts_the_legs() {
        let (mut e, actor, moved) = move_engine();
        answer(&mut e, ACTOR_VT, 0x360, 1);
        e.call_log = Some(vec![]);
        e.call(0x008a_62b0, &args![actor, 0.5f32, 7u32, 1u32]);
        let log = calls(&mut e);
        // 1200 is above the heavy threshold 500.
        assert_eq!(called(&log, 0x00ad_7550)[0][2..], [LAND_HEAVY_SOUND, 0x101]);
        assert_eq!(called(&log, SOUND_HANDLE_ASSIGN).len(), 1);
        assert_eq!(called(&log, 0x00ad_8830)[0][1], 0);
        assert_eq!(
            called(&log, 0x0062_96b0),
            vec![vec![moved.addr() + 0x410, 0x80]]
        );
        // Random 205 % 100 = 5 is below the chance 50: both legs, -600.
        assert_eq!(
            called(&log, target(ACTOR_VT, 0x3ac)),
            vec![
                vec![actor.addr(), 0x1d, (-600.0f32).to_bits(), 0],
                vec![actor.addr(), 0x1e, (-600.0f32).to_bits(), 0]
            ]
        );
        // The hurt call also runs in the loaded state.
        assert_eq!(called(&log, target(ACTOR_VT, 0x338)).len(), 1);
    }

    #[test]
    fn move_light_landing_and_no_leg_damage_when_the_roll_fails() {
        let (mut e, actor, _) = move_engine();
        answer(&mut e, ACTOR_VT, 0x360, 1);
        e.set_global(SETTING_HEAVY_THRESHOLD + 4, 5000.0f32);
        e.set_global(SETTING_LIMB_DAMAGE_CHANCE + 4, 5u32);
        e.call_log = Some(vec![]);
        e.call(0x008a_62b0, &args![actor, 0.5f32, 7u32, 1u32]);
        let log = calls(&mut e);
        assert_eq!(called(&log, 0x00ad_7550)[0][2..], [LAND_LIGHT_SOUND, 0x101]);
        assert!(called(&log, target(ACTOR_VT, 0x3ac)).is_empty());
    }

    #[test]
    fn move_without_damage() {
        // Below the threshold.
        let (mut e, actor, moved) = move_engine();
        e.register(0x00c7_0550, |_, _| 50.0f32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x008a_62b0, &args![actor, 0.5f32, 7u32, 1u32])
                .ptr::<()>(),
            moved
        );
        assert!(called(&calls(&mut e), 0x008a_7d50).is_empty());
        // A base form that the check flags takes the damage away.
        let (mut e, actor, moved) = move_engine();
        e.register(0x0041_81e0, |_, _| 0x1.into_ret());
        e.register(0x005f_0c40, |_, _| 1.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x008a_62b0, &args![actor, 0.5f32, 7u32, 1u32])
                .ptr::<()>(),
            moved
        );
        let log = calls(&mut e);
        assert!(called(&log, 0x008a_7d50).is_empty());
        assert!(called(&log, target(ACTOR_VT, 0x338)).is_empty());
        // The mover is not on the ground.
        let (mut e, actor, moved) = move_engine();
        e.register(0x0087_cee0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x008a_62b0, &args![actor, 0.5f32, 7u32, 1u32])
                .ptr::<()>(),
            moved
        );
        assert!(called(&calls(&mut e), 0x00c7_0550).is_empty());
    }

    #[test]
    fn move_gives_up_early() {
        // In dialogue with the player, without the low flag bits.
        let (mut e, actor, _) = move_engine();
        e.register(0x0093_3840, |_, _| 1.into_ret());
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x008a_62b0, &args![actor, 0.5f32, 7u32, 0x40u32])
            .ptr::<()>()
            .is_null());
        assert!(called(&calls(&mut e), 0x0092_f260).is_empty());
        // With a low flag bit it moves.
        e.call_log = Some(vec![]);
        e.call(0x008a_62b0, &args![actor, 0.5f32, 7u32, 0x01u32]);
        assert_eq!(called(&calls(&mut e), 0x0092_f260).len(), 1);
        // No loaded data.
        let (mut e, actor, _) = move_engine();
        e.mem.set_u32(actor.addr() + LOADED_DATA, 0);
        assert!(e
            .call(0x008a_62b0, &args![actor, 0.5f32, 7u32, 1u32])
            .ptr::<()>()
            .is_null());
        // Nothing moved.
        let (mut e, actor, _) = move_engine();
        e.set_global(KNOB, 0u32);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x008a_62b0, &args![actor, 0.5f32, 7u32, 1u32])
            .ptr::<()>()
            .is_null());
        assert!(called(&calls(&mut e), 0x0087_cee0).is_empty());
    }

    #[test]
    fn character_controller_flag_call() {
        let mut e = engine();
        e.register(0x0062_96b0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x008a_6630, &args![0x1000u32]);
        assert_eq!(
            called(&calls(&mut e), 0x0062_96b0),
            vec![vec![0x1410, 0x80]]
        );
    }

    /// An actor whose current package is a fresh block of the given type.
    fn actor_with_package(e: &mut Engine, kind: u8) -> (Ptr<Actor>, Ptr, Ptr) {
        let (actor, proc) = new_actor_with_process(e);
        let package: Ptr = Ptr::new(e.mem.alloc(0x100));
        e.mem.set_u8(package.addr() + 0x20, kind);
        e.register(FORM_TYPE, |e, a| (e.mem.i8(a[0] + 0x20) as i32).into_ret());
        e.register(GET_CURRENT_PACKAGE, |e, _| e.mem.u32(KNOB).into_ret());
        e.set_global(KNOB, package.addr());
        (actor, proc, package)
    }

    #[test]
    fn fleeing_by_package_type() {
        let mut e = engine();
        let (actor, _, package) = actor_with_package(&mut e, 0x16);
        e.set_global(PLAYER_CHARACTER, 0u32);
        // A flee package: fleeing, whatever the flag.
        assert!(e.call(0x008a_6650, &args![actor, 0u8]).bool());
        assert!(e.call(0x008a_6650, &args![actor, 1u8]).bool());
        // Package type 0xa counts only without the flag.
        e.mem.set_u8(package.addr() + 0x20, 0xa);
        assert!(e.call(0x008a_6650, &args![actor, 0u8]).bool());
        assert!(!e.call(0x008a_6650, &args![actor, 1u8]).bool());
        // Type 0x13 counts while the process method +0x280 is 1.
        e.mem.set_u8(package.addr() + 0x20, 0x13);
        answer(&mut e, PROCESS_VT, 0x280, 1);
        assert!(e.call(0x008a_6650, &args![actor, 0u8]).bool());
        answer(&mut e, PROCESS_VT, 0x280, 2);
        assert!(!e.call(0x008a_6650, &args![actor, 0u8]).bool());
        // No package.
        e.set_global(KNOB, 0u32);
        assert!(!e.call(0x008a_6650, &args![actor, 0u8]).bool());
    }

    #[test]
    fn fleeing_follows_the_combat_controller() {
        let mut e = engine();
        let (actor, _, _) = actor_with_package(&mut e, 0x05);
        e.register(0x0098_1990, |_, a| (a[0] == 0x4242).into_ret());
        // No controller.
        assert!(!e.call(0x008a_6650, &args![actor, 0u8]).bool());
        // A controller that is fleeing.
        answer(&mut e, ACTOR_VT, 0x428, 0x4242);
        assert!(e.call(0x008a_6650, &args![actor, 0u8]).bool());
        // One that is not.
        answer(&mut e, ACTOR_VT, 0x428, 0x4243);
        assert!(!e.call(0x008a_6650, &args![actor, 0u8]).bool());
    }

    #[test]
    fn the_player_with_a_flee_package_is_not_fleeing_and_the_process_is_told() {
        let mut e = engine();
        let (actor, proc, _) = actor_with_package(&mut e, 0x16);
        e.set_global(PLAYER_CHARACTER, actor.addr());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x008a_6650, &args![actor, 0u8]).bool());
        let log = calls(&mut e);
        assert_eq!(
            called(&log, target(PROCESS_VT, 0x234)),
            vec![vec![proc.addr()]]
        );
    }

    /// Doubles for `RemoveFleeTarget`: a flee package whose "has targets"
    /// byte (+0x98 + 0) is set by the removal.
    fn remove_target_engine(kind: u8) -> (Engine, Ptr<Actor>, Ptr) {
        let mut e = engine();
        let (actor, _, package) = actor_with_package(&mut e, kind);
        e.register(0x0082_56d0, |e, a| e.mem.u8(a[0]).into_ret());
        e.register(0x009f_1350, |e, a| {
            e.mem.set_u8(a[0] + 0x98, 1);
            Ret::default()
        });
        e.register(IS_ALIVE_BODY, |_, _| Ret::default());
        stub(&mut e, &[0x0088_1680]);
        (e, actor, package)
    }

    #[test]
    fn removing_a_flee_target_stops_the_actor() {
        let (mut e, actor, package) = remove_target_engine(0x16);
        e.call_log = Some(vec![]);
        e.call(0x008a_6730, &args![actor, 0x77u32]);
        let log = calls(&mut e);
        assert_eq!(called(&log, 0x009f_1350), vec![vec![package.addr(), 0x77]]);
        // Not a living body: the interrupt package ends.
        assert_eq!(called(&log, 0x0088_1680), vec![vec![actor.addr(), 0]]);
        assert!(called(&log, target(ACTOR_VT, 0x434)).is_empty());

        // A body that `00493bb0` accepts is stopped through +0x434 instead.
        let (mut e, actor, _) = remove_target_engine(0x16);
        e.register(IS_ALIVE_BODY, |_, _| 1.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x008a_6730, &args![actor, 0x77u32]);
        let log = calls(&mut e);
        assert_eq!(
            called(&log, target(ACTOR_VT, 0x434)),
            vec![vec![actor.addr(), 0]]
        );
        assert!(called(&log, 0x0088_1680).is_empty());
    }

    #[test]
    fn removing_a_flee_target_does_nothing_when_not_applicable() {
        // Not a flee package.
        let (mut e, actor, _) = remove_target_engine(0x15);
        e.call_log = Some(vec![]);
        e.call(0x008a_6730, &args![actor, 0x77u32]);
        assert!(called(&calls(&mut e), 0x009f_1350).is_empty());
        // Already flagged.
        let (mut e, actor, package) = remove_target_engine(0x16);
        e.mem.set_u8(package.addr() + 0x98, 1);
        e.call_log = Some(vec![]);
        e.call(0x008a_6730, &args![actor, 0x77u32]);
        assert!(called(&calls(&mut e), 0x009f_1350).is_empty());
        // The removal does not flag it: the actor is left alone.
        let (mut e, actor, _) = remove_target_engine(0x16);
        e.register(0x009f_1350, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x008a_6730, &args![actor, 0x77u32]);
        let log = calls(&mut e);
        assert!(called(&log, 0x0088_1680).is_empty());
    }

    #[test]
    fn list_check_looks_at_the_sub_object() {
        let mut e = engine();
        e.register(0x0082_56d0, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        assert!(e.call(0x008a_67d0, &args![0x1000u32]).bool());
        assert_eq!(called(&calls(&mut e), 0x0082_56d0), vec![vec![0x1098]]);
    }

    #[test]
    fn talking_is_the_flag_or_the_process_answer() {
        let mut e = engine();
        let (actor, proc) = new_actor_with_process(&mut e);
        answer(&mut e, PROCESS_VT, 0x4b8, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x008a_67f0, &args![actor]).bool());
        let log = calls(&mut e);
        assert_eq!(
            called(&log, target(PROCESS_VT, 0x4b8)),
            vec![vec![proc.addr(), actor.addr()]]
        );
        answer(&mut e, PROCESS_VT, 0x4b8, 0);
        assert!(!e.call(0x008a_67f0, &args![actor]).bool());
        e.set(actor, Actor::bTalkingToPlayer, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x008a_67f0, &args![actor]).bool());
        assert!(called(&calls(&mut e), target(PROCESS_VT, 0x4b8)).is_empty());
        let bare = new_actor(&mut e);
        assert!(!e.call(0x008a_67f0, &args![bare]).bool());
    }

    #[test]
    fn continuing_package_for_the_player_is_the_process_answer() {
        let mut e = engine();
        let (actor, _) = new_actor_with_process(&mut e);
        answer(&mut e, PROCESS_VT, 0x4e0, 1);
        assert!(e.call(0x008a_69d0, &args![actor]).bool());
        answer(&mut e, PROCESS_VT, 0x4e0, 0);
        assert!(!e.call(0x008a_69d0, &args![actor]).bool());
        let bare = new_actor(&mut e);
        assert!(!e.call(0x008a_69d0, &args![bare]).bool());
    }

    #[test]
    fn heading_is_set_only_in_modes_zero_and_four() {
        for (mode, runs) in [(0u32, true), (4, true), (1, false), (5, false)] {
            let mut e = engine();
            let actor = new_actor(&mut e);
            e.register(0x004f_8960, |e, _| e.mem.u32(KNOB).into_ret());
            e.set_global(KNOB, mode);
            stub(&mut e, &[0x0093_1b60]);
            e.call_log = Some(vec![]);
            e.call(0x008a_6a00, &args![actor, 1.5f32]);
            let log = calls(&mut e);
            let seen = called(&log, 0x0093_1b60);
            if runs {
                assert_eq!(seen, vec![vec![actor.addr(), 1.5f32.to_bits()]], "{mode}");
            } else {
                assert!(seen.is_empty(), "{mode}");
            }
        }
    }

    /// The flags word `008846e0` answers for an actor in these tests.
    const ACTOR_FLAGS_WORD: u32 = 0x1b0;

    /// Doubles for the weapon-drawn change (`008a6840`, `008a6970`).
    fn draw_engine() -> (Engine, Ptr<Actor>, Ptr) {
        let mut e = engine();
        e.map(0x011f_2000, 0x1000);
        let (actor, proc) = new_actor_with_process(&mut e);
        e.register(0x0088_46e0, |e, a| {
            e.mem.u32(a[0] + ACTOR_FLAGS_WORD).into_ret()
        });
        e.register(0x008b_bc10, |e, _| e.mem.u32(KNOB).into_ret());
        e.register(GET_ANIMATION, |_, _| 0x500.into_ret());
        e.register(0x005f_2370, |_, _| 0x66.into_ret());
        e.register(0x0049_4710, |e, _| e.mem.u32(KNOB + 4).into_ret());
        e.register(0x0057_15d0, |_, _| 0x77.into_ret());
        e.register(0x0044_7330, |_, _| 0x99.into_ret());
        e.set_global(MODEL_LOADER, 0x88u32);
        e.register(0x0044_ddc0, |e, a| e.mem.u32(a[0] + 8).into_ret());
        stub(&mut e, &[0x0049_0400, 0x009c_8950]);
        (e, actor, proc)
    }

    #[test]
    fn drawing_a_weapon_loads_the_animation_group_once() {
        let (mut e, actor, proc) = draw_engine();
        e.call_log = Some(vec![]);
        e.call(0x008a_6840, &args![actor, 1u8]);
        let log = calls(&mut e);
        assert_eq!(
            called(&log, target(ACTOR_VT, 0x340)),
            vec![vec![actor.addr(), 0xd]]
        );
        assert_eq!(
            called(&log, target(PROCESS_VT, 0x450)),
            vec![vec![proc.addr(), 1]]
        );
        assert_eq!(called(&log, 0x005f_2370), vec![vec![0, 1, 0x18, 0]]);
        assert_eq!(called(&log, 0x0049_4710), vec![vec![0x500, 0x66]]);
        assert_eq!(called(&log, 0x0057_15d0), vec![vec![actor.addr()]]);
        assert_eq!(called(&log, 0x0044_7330), vec![vec![0x88, 0x77, 1, 0, 1]]);
        assert_eq!(called(&log, 0x0049_0400), vec![vec![0x500, 0x99]]);
        // Already loaded.
        e.set_global(KNOB + 4, 1u32);
        e.call_log = Some(vec![]);
        e.call(0x008a_6840, &args![actor, 1u8]);
        assert!(called(&calls(&mut e), 0x0049_0400).is_empty());
    }

    #[test]
    fn drawing_does_not_load_for_holstering_the_player_or_an_armed_process() {
        // Holstering (current state 1).
        let (mut e, actor, _) = draw_engine();
        answer(&mut e, PROCESS_VT, 0x44c, 1);
        e.call_log = Some(vec![]);
        e.call(0x008a_6840, &args![actor, 0u8]);
        let log = calls(&mut e);
        assert_eq!(called(&log, target(PROCESS_VT, 0x450)).len(), 1);
        assert!(called(&log, 0x0049_0400).is_empty());
        // The player.
        let (mut e, actor, _) = draw_engine();
        e.set_global(PLAYER_CHARACTER, actor.addr());
        e.call_log = Some(vec![]);
        e.call(0x008a_6840, &args![actor, 1u8]);
        let log = calls(&mut e);
        assert_eq!(called(&log, target(PROCESS_VT, 0x450)).len(), 1);
        assert!(called(&log, 0x0049_0400).is_empty());
        // A process that already has something (method +0x148).
        let (mut e, actor, _) = draw_engine();
        answer(&mut e, PROCESS_VT, 0x148, 0x42);
        e.call_log = Some(vec![]);
        e.call(0x008a_6840, &args![actor, 1u8]);
        assert!(called(&calls(&mut e), 0x0049_0400).is_empty());
    }

    #[test]
    fn drawing_is_refused_in_the_blocked_states() {
        // Same state: nothing.
        let (mut e, actor, _) = draw_engine();
        answer(&mut e, PROCESS_VT, 0x44c, 1);
        e.call_log = Some(vec![]);
        e.call(0x008a_6840, &args![actor, 1u8]);
        assert!(called(&calls(&mut e), target(ACTOR_VT, 0x340)).is_empty());
        // Flag 0x800 blocks drawing but not holstering.
        let (mut e, actor, _) = draw_engine();
        e.mem.set_u32(actor.addr() + ACTOR_FLAGS_WORD, 0x800);
        e.call_log = Some(vec![]);
        e.call(0x008a_6840, &args![actor, 1u8]);
        assert!(called(&calls(&mut e), target(ACTOR_VT, 0x340)).is_empty());
        // The player with the iron sights up.
        let (mut e, actor, _) = draw_engine();
        e.set_global(PLAYER_CHARACTER, actor.addr());
        e.set_global(KNOB, 1u32);
        e.call_log = Some(vec![]);
        e.call(0x008a_6840, &args![actor, 1u8]);
        assert!(called(&calls(&mut e), target(ACTOR_VT, 0x340)).is_empty());
        // No process.
        let (mut e, _, _) = draw_engine();
        let bare = new_actor(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x008a_6840, &args![bare, 1u8]);
        assert!(called(&calls(&mut e), target(ACTOR_VT, 0x340)).is_empty());
    }

    #[test]
    fn drawn_state_is_the_process_answer_unless_the_flag_blocks_it() {
        let (mut e, actor, _) = draw_engine();
        answer(&mut e, PROCESS_VT, 0x44c, 1);
        assert!(e.call(0x008a_6970, &args![actor]).bool());
        answer(&mut e, PROCESS_VT, 0x44c, 0);
        assert!(!e.call(0x008a_6970, &args![actor]).bool());
        // Flag 0x800: false; the player in VATS mode 4 quits the playback.
        e.mem.set_u32(actor.addr() + ACTOR_FLAGS_WORD, 0x800);
        answer(&mut e, PROCESS_VT, 0x44c, 1);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x008a_6970, &args![actor]).bool());
        assert!(called(&calls(&mut e), 0x009c_8950).is_empty());
        e.set_global(PLAYER_CHARACTER, actor.addr());
        e.mem.set_u32(0x011f_2250 + 8, 4);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x008a_6970, &args![actor]).bool());
        assert_eq!(
            called(&calls(&mut e), 0x009c_8950),
            vec![vec![0x011f_2250, 0, 0]]
        );
        e.mem.set_u32(0x011f_2250 + 8, 3);
        e.call_log = Some(vec![]);
        e.call(0x008a_6970, &args![actor]);
        assert!(called(&calls(&mut e), 0x009c_8950).is_empty());
    }

    /// Doubles for `UnlockLockDoorsProcedure`: the actor's method +0x1f4 returns
    /// 1, then 2, ...
    fn doors_engine() -> (Engine, Ptr<Actor>, Ptr, Ptr) {
        let mut e = engine();
        let (actor, proc) = new_actor_with_process(&mut e);
        let package = new_object(&mut e);
        answer(&mut e, PROCESS_VT, 0x22c, package.addr());
        let mut n = 0;
        e.register_double(target(ACTOR_VT, 0x1f4), move |_, _| {
            n += 1;
            Ret {
                eax: n,
                ..Ret::default()
            }
        });
        e.set_global(0x011d_df38, 0x5000u32);
        e.set_global(DATA_HANDLER, 0x7000u32);
        e.set_global(SEARCH_RADIUS, 5000.0f32);
        e.set_global(MINUS_ONE, -1.0f32);
        e.register(0x0042_ce10, |_, _| Ret::default());
        e.register(GET_PARENT_CELL, |_, _| 0x8000.into_ret());
        stub(
            &mut e,
            &[
                0x0067_8610,
                0x0053_94a0,
                0x0067_0ed0,
                0x0067_0f40,
                0x0067_0f60,
                ENUM_REFERENCES_CLOSE_TO_POINT,
            ],
        );
        answer(&mut e, OBJECT_VT, 0x13c, 0);
        (e, actor, proc, package)
    }

    #[test]
    fn doors_search_with_the_door_callback_then_ask_the_package() {
        let (mut e, actor, _, package) = doors_engine();
        e.register(0x0053_94a0, |_, _| 1.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x008a_6a40, &args![actor]);
        let log = calls(&mut e);
        let radius = 5000.0f32.to_bits();
        assert_eq!(
            called(&log, ENUM_REFERENCES_CLOSE_TO_POINT),
            vec![vec![
                0x7000,
                0x8000,
                2,
                radius,
                1,
                radius,
                PACKAGE_CALLBACK_B,
                actor.addr()
            ]]
        );
        // The package is asked with the actor, 0, -1.0 and 0; it refuses.
        assert_eq!(
            called(&log, target(OBJECT_VT, 0x13c)),
            vec![vec![
                package.addr(),
                actor.addr(),
                0,
                (-1.0f32).to_bits(),
                0
            ]]
        );
    }

    #[test]
    fn doors_second_search_depends_on_the_package_answer() {
        let (mut e, actor, proc, _) = doors_engine();
        e.register(0x0067_0ed0, |_, _| 1.into_ret());
        e.register(0x0067_0f40, |_, _| 1.into_ret());
        answer(&mut e, OBJECT_VT, 0x13c, 1);
        e.call_log = Some(vec![]);
        e.call(0x008a_6a40, &args![actor]);
        let log = calls(&mut e);
        let searches = called(&log, ENUM_REFERENCES_CLOSE_TO_POINT);
        assert_eq!(searches.len(), 2);
        assert_eq!(searches[0][6], PACKAGE_CALLBACK_A);
        assert_eq!(searches[1][6], PACKAGE_CALLBACK_B);
        assert_eq!(
            called(&log, target(PROCESS_VT, 0x1c0)),
            vec![vec![proc.addr(), 1]]
        );

        // The second kind of search runs without the process call.
        let (mut e, actor, _, _) = doors_engine();
        e.register(0x0067_0f60, |_, _| 1.into_ret());
        answer(&mut e, OBJECT_VT, 0x13c, 1);
        e.call_log = Some(vec![]);
        e.call(0x008a_6a40, &args![actor]);
        let log = calls(&mut e);
        let searches = called(&log, ENUM_REFERENCES_CLOSE_TO_POINT);
        assert_eq!(searches.len(), 1);
        assert_eq!(searches[0][6], PACKAGE_CALLBACK_A);
        assert!(called(&log, target(PROCESS_VT, 0x1c0)).is_empty());

        // The process says it is busy (method +0x1bc): no second search.
        let (mut e, actor, _, _) = doors_engine();
        e.register(0x0067_0f40, |_, _| 1.into_ret());
        answer(&mut e, OBJECT_VT, 0x13c, 1);
        answer(&mut e, PROCESS_VT, 0x1bc, 1);
        e.call_log = Some(vec![]);
        e.call(0x008a_6a40, &args![actor]);
        assert!(called(&calls(&mut e), ENUM_REFERENCES_CLOSE_TO_POINT).is_empty());
    }

    #[test]
    fn doors_gates() {
        // No process.
        let (mut e, actor, _, _) = doors_engine();
        e.set(actor, Actor::pCurrentProcess, Ptr::NULL);
        e.call_log = Some(vec![]);
        e.call(0x008a_6a40, &args![actor]);
        assert!(called(&calls(&mut e), target(OBJECT_VT, 0x13c)).is_empty());
        // The actor's method +0x22c (with 0) holds.
        let (mut e, actor, _, _) = doors_engine();
        answer(&mut e, ACTOR_VT, 0x22c, 1);
        e.call_log = Some(vec![]);
        e.call(0x008a_6a40, &args![actor]);
        assert!(called(&calls(&mut e), target(OBJECT_VT, 0x13c)).is_empty());
        // The global object's check holds.
        let (mut e, actor, _, _) = doors_engine();
        e.register(0x0042_ce10, |_, a| (a[0] == 0x5000).into_ret());
        e.call_log = Some(vec![]);
        e.call(0x008a_6a40, &args![actor]);
        assert!(called(&calls(&mut e), target(OBJECT_VT, 0x13c)).is_empty());
        // No package, no cell, an interrupt package.
        let (mut e, actor, _, _) = doors_engine();
        answer(&mut e, PROCESS_VT, 0x22c, 0);
        e.call_log = Some(vec![]);
        e.call(0x008a_6a40, &args![actor]);
        assert!(called(&calls(&mut e), target(OBJECT_VT, 0x13c)).is_empty());
        let (mut e, actor, _, _) = doors_engine();
        e.register(GET_PARENT_CELL, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x008a_6a40, &args![actor]);
        assert!(called(&calls(&mut e), target(OBJECT_VT, 0x13c)).is_empty());
        let (mut e, actor, _, _) = doors_engine();
        e.register(0x0067_8610, |_, _| 1.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x008a_6a40, &args![actor]);
        assert!(called(&calls(&mut e), target(OBJECT_VT, 0x13c)).is_empty());
    }

    /// Makes a fake function answer the given values in turn, then the last
    /// one again.
    fn answer_in_turn(e: &mut Engine, at: u32, slot: u32, values: &[u32]) {
        let values = values.to_vec();
        let mut next = 0;
        e.register_double(target(at, slot), move |_, _| {
            let value = values[next.min(values.len() - 1)];
            next += 1;
            Ret {
                eax: value,
                ..Ret::default()
            }
        });
    }

    /// Doubles for `Actor::EvaluatePackage`.
    fn evaluate_engine() -> (Engine, Ptr<Actor>, Ptr, Ptr) {
        let mut e = engine();
        let (actor, proc) = new_actor_with_process(&mut e);
        let package = new_object(&mut e);
        e.register(CALENDAR_GET_HOUR, |_, _| 10.5f32.into_ret());
        e.set_global(ONE, 1.0f64);
        e.set_global(DATA_HANDLER, 0x7000u32);
        e.set_global(SEARCH_RADIUS, 5000.0f32);
        e.register(IS_ALIVE_BODY, |_, _| Ret::default());
        e.register(0x0045_cd60, |_, _| 1.into_ret());
        e.register(0x005d_43c0, |_, a| (a[0] + 0x44).into_ret());
        e.register(GET_PARENT_CELL, |_, _| 0x8000.into_ret());
        e.register(FORM_TYPE, |e, a| (e.mem.i8(a[0] + 0x20) as i32).into_ret());
        e.register(GET_ANIMATION, |_, _| 0x500.into_ret());
        stub(
            &mut e,
            &[
                HOUR_TO_PROCESS,
                STOP_MOVING,
                ENUM_REFERENCES_CLOSE_TO_POINT,
                0x0067_0f10,
                0x0067_0ef0,
                0x0042_2690,
                0x0088_14b0,
                0x0088_1680,
                0x0088_d640,
                0x005a_c750,
                0x0044_1b00,
                GET_SAVE_FORM,
                0x0040_1170,
                0x0087_faa0,
                0x0067_1d10,
                0x0067_2930,
                0x0093_1fb0,
                0x0049_6080,
                0x0049_8910,
                0x0049_1040,
            ],
        );
        answer(&mut e, PROCESS_VT, 0x22c, package.addr());
        (e, actor, proc, package)
    }

    #[test]
    fn evaluating_is_buffered_when_not_immediate() {
        let (mut e, actor, proc, _) = evaluate_engine();
        e.call_log = Some(vec![]);
        e.call(0x008a_6ce0, &args![actor, 0u8, 1u8]);
        let log = calls(&mut e);
        assert_eq!(e.get(actor, Actor::bEVPBuffered), 1);
        assert_eq!(e.get(actor, Actor::bResetAI), 1);
        // The process is told the hour before now: 10.5 - 1.
        assert_eq!(
            called(&log, HOUR_TO_PROCESS),
            vec![vec![proc.addr(), 9.5f32.to_bits()]]
        );
        // Without a process only the flags are set.
        let bare = new_actor(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x008a_6ce0, &args![bare, 0u8, 0u8]);
        assert!(called(&calls(&mut e), HOUR_TO_PROCESS).is_empty());
        assert_eq!(e.get(bare, Actor::bEVPBuffered), 1);
        assert_eq!(e.get(bare, Actor::bResetAI), 0);
    }

    #[test]
    fn evaluating_does_nothing_when_the_actor_vetoes_it() {
        let (mut e, actor, _, _) = evaluate_engine();
        answer(&mut e, ACTOR_VT, 0x22c, 1);
        e.call_log = Some(vec![]);
        e.call(0x008a_6ce0, &args![actor, 1u8, 1u8]);
        let log = calls(&mut e);
        assert_eq!(
            called(&log, target(ACTOR_VT, 0x22c)),
            vec![vec![actor.addr(), 1]]
        );
        assert_eq!(log.len(), 2);
        assert_eq!(e.get(actor, Actor::bEVPBuffered), 0);
    }

    #[test]
    fn evaluating_without_reset_restarts_the_package_if_nothing_is_pending() {
        let (mut e, actor, proc, _) = evaluate_engine();
        e.set(actor, Actor::bEVPBuffered, 1);
        e.set(actor, Actor::bResetAI, 1);
        e.call_log = Some(vec![]);
        e.call(0x008a_6ce0, &args![actor, 1u8, 0u8]);
        let log = calls(&mut e);
        assert_eq!(called(&log, STOP_MOVING), vec![vec![actor.addr()]]);
        assert_eq!(
            called(&log, target(PROCESS_VT, 0x524)),
            vec![vec![proc.addr(), actor.addr()]]
        );
        assert_eq!(
            called(&log, target(PROCESS_VT, 0x118)),
            vec![vec![proc.addr(), 1]]
        );
        assert_eq!(
            called(&log, target(PROCESS_VT, 0x24)),
            vec![vec![proc.addr(), actor.addr(), 1]]
        );
        assert_eq!(
            called(&log, HOUR_TO_PROCESS),
            vec![vec![proc.addr(), 9.5f32.to_bits()]]
        );
        assert_eq!(e.get(actor, Actor::bEVPBuffered), 0);
        assert_eq!(e.get(actor, Actor::bResetAI), 0);

        // A pending change (method +0x214 = 7) leaves everything alone.
        let (mut e, actor, _, _) = evaluate_engine();
        e.set(actor, Actor::bEVPBuffered, 1);
        answer(&mut e, ACTOR_VT, 0x214, 7);
        e.call_log = Some(vec![]);
        e.call(0x008a_6ce0, &args![actor, 1u8, 0u8]);
        assert!(called(&calls(&mut e), STOP_MOVING).is_empty());
        assert_eq!(e.get(actor, Actor::bEVPBuffered), 1);
        // 4 and 9 do not count as pending.
        for kind in [0, 4, 9] {
            let (mut e, actor, _, _) = evaluate_engine();
            answer(&mut e, ACTOR_VT, 0x214, kind);
            e.call_log = Some(vec![]);
            e.call(0x008a_6ce0, &args![actor, 1u8, 0u8]);
            assert_eq!(called(&calls(&mut e), STOP_MOVING).len(), 1, "{kind}");
        }
    }

    #[test]
    fn evaluating_with_reset_switches_the_package() {
        let (mut e, actor, proc, package) = evaluate_engine();
        // The old package has type 4; the editor package differs from the new.
        let old: Ptr = Ptr::new(e.mem.alloc(0x40));
        e.mem.set_u8(old.addr() + 0x20, 4);
        let new_package: Ptr = Ptr::new(e.mem.alloc(0x40));
        let tail: Ptr = Ptr::new(e.mem.alloc(0x40));
        let editor: Ptr = Ptr::new(e.mem.alloc(0x40));
        let follower = new_object(&mut e);
        answer_in_turn(
            &mut e,
            PROCESS_VT,
            0x22c,
            &[
                package.addr(),
                package.addr(),
                old.addr(),
                old.addr(),
                new_package.addr(),
                tail.addr(),
            ],
        );
        answer(&mut e, PROCESS_VT, 0x360, 1);
        answer(&mut e, PROCESS_VT, 0x128, follower.addr());
        answer(&mut e, OBJECT_VT, 0x100, 1);
        e.register(0x0067_0f10, |_, _| 1.into_ret());
        e.register(0x0067_0ef0, |_, _| 1.into_ret());
        e.register_double(0x0088_14b0, move |_, _| 0x4a00.into_ret());
        e.register(0x0067_1d10, |_, _| 1.into_ret());
        e.register(0x0067_2930, |_, _| 3.into_ret());
        e.register(GET_SAVE_FORM, |_, _| 0x4b00.into_ret());
        let _ = editor;
        e.call_log = Some(vec![]);
        e.call(0x008a_6ce0, &args![actor, 1u8, 1u8]);
        let log = calls(&mut e);
        // Both reference searches ran, with the two callbacks.
        let searches = called(&log, ENUM_REFERENCES_CLOSE_TO_POINT);
        assert_eq!(searches.len(), 2);
        assert_eq!(searches[0][6], PACKAGE_CALLBACK_A);
        assert_eq!(searches[1][6], PACKAGE_CALLBACK_B);
        // The follower is removed from the actor's extra data list.
        assert_eq!(
            called(&log, 0x0042_2690),
            vec![vec![follower.addr() + 0x44, actor.addr()]]
        );
        // The editor package is flagged and told to the process.
        assert_eq!(
            called(&log, 0x005a_c750),
            vec![vec![0x4a00, actor.addr() + 0x44, 0x800]]
        );
        assert_eq!(
            called(&log, target(PROCESS_VT, 0x59c)),
            vec![vec![proc.addr(), actor.addr(), 0x4a00]]
        );
        assert_eq!(
            called(&log, target(PROCESS_VT, 0x5a8)),
            vec![vec![proc.addr(), 0]]
        );
        // The old package (type 4) makes the save form's type be looked at.
        assert_eq!(called(&log, 0x0040_1170), vec![vec![0x4b00]]);
        // The tail passes the package's initial target count.
        assert_eq!(
            called(&log, target(PROCESS_VT, 0x15c)),
            vec![vec![proc.addr(), 3]]
        );
        assert_eq!(called(&log, HOUR_TO_PROCESS).len(), 1);
        // Order of the main steps.
        let steps = [
            target(PROCESS_VT, 0x20c),
            0x0088_1680,
            target(PROCESS_VT, 0x234),
            0x0088_d640,
            target(PROCESS_VT, 0x524),
            target(PROCESS_VT, 0x4ec),
            target(PROCESS_VT, 0x24),
            0x005a_c750,
            0x0087_faa0,
            target(PROCESS_VT, 0x15c),
        ];
        assert_eq!(order(&log, &steps), steps.to_vec());
        assert_eq!(called(&log, 0x0088_1680), vec![vec![actor.addr(), 0]]);
        assert_eq!(e.get(actor, Actor::bEVPBuffered), 0);
    }

    #[test]
    fn evaluating_with_reset_skips_the_editor_package_when_it_is_the_new_one() {
        let (mut e, actor, _, package) = evaluate_engine();
        e.register(0x0088_14b0, |e, _| e.mem.u32(KNOB).into_ret());
        e.set_global(KNOB, package.addr());
        e.call_log = Some(vec![]);
        e.call(0x008a_6ce0, &args![actor, 1u8, 1u8]);
        let log = calls(&mut e);
        assert!(called(&log, 0x005a_c750).is_empty());
        assert!(called(&log, target(PROCESS_VT, 0x59c)).is_empty());
    }

    #[test]
    fn evaluating_with_reset_finishes_by_the_package_change_kind() {
        // Kind 9 and kind 4 with a package: method +0x418.
        for kind in [9u32, 4] {
            let (mut e, actor, _, _) = evaluate_engine();
            answer(&mut e, ACTOR_VT, 0x214, kind);
            e.call_log = Some(vec![]);
            e.call(0x008a_6ce0, &args![actor, 1u8, 1u8]);
            assert_eq!(
                called(&calls(&mut e), target(ACTOR_VT, 0x418)).len(),
                1,
                "{kind}"
            );
        }
        // Kind 4 without a package: nothing more.
        let (mut e, actor, _, package) = evaluate_engine();
        answer(&mut e, ACTOR_VT, 0x214, 4);
        // Calls 1 to 5 see a package; the check of the kind is the sixth.
        answer_in_turn(
            &mut e,
            PROCESS_VT,
            0x22c,
            &[
                package.addr(),
                package.addr(),
                package.addr(),
                package.addr(),
                package.addr(),
                0,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x008a_6ce0, &args![actor, 1u8, 1u8]);
        assert!(called(&calls(&mut e), target(ACTOR_VT, 0x418)).is_empty());
        // Any other kind: the chase bip is cleared and the animation reset.
        let (mut e, actor, proc, _) = evaluate_engine();
        answer(&mut e, ACTOR_VT, 0x214, 5);
        e.call_log = Some(vec![]);
        e.call(0x008a_6ce0, &args![actor, 1u8, 1u8]);
        let log = calls(&mut e);
        assert!(called(&log, target(ACTOR_VT, 0x418)).is_empty());
        assert_eq!(called(&log, 0x0093_1fb0), vec![vec![actor.addr(), 0]]);
        assert_eq!(
            called(&log, target(PROCESS_VT, 0x4c0)),
            vec![vec![proc.addr(), actor.addr(), 0, 0, 0x7f]]
        );
        assert_eq!(
            called(&log, target(PROCESS_VT, 0x3ec)),
            vec![vec![proc.addr(), u32::MAX, 0]]
        );
        assert_eq!(called(&log, 0x0049_6080), vec![vec![0x500, 0x14, 0]]);
        assert_eq!(called(&log, 0x0049_8910), vec![vec![0x500, 1, 0]]);
    }

    #[test]
    fn evaluating_with_reset_but_no_process_only_clears_the_flags() {
        let (mut e, _, _, _) = evaluate_engine();
        let bare = new_actor(&mut e);
        e.set(bare, Actor::bEVPBuffered, 1);
        e.set(bare, Actor::bResetAI, 1);
        e.call_log = Some(vec![]);
        e.call(0x008a_6ce0, &args![bare, 1u8, 1u8]);
        let log = calls(&mut e);
        assert!(called(&log, HOUR_TO_PROCESS).is_empty());
        assert_eq!(e.get(bare, Actor::bEVPBuffered), 0);
        assert_eq!(e.get(bare, Actor::bResetAI), 0);
    }

    /// Doubles for `SetAnimAction`.
    fn anim_action_engine() -> (Engine, Ptr<Actor>, Ptr) {
        let mut e = engine();
        let (actor, proc) = new_actor_with_process(&mut e);
        e.register(GET_ANIMATION, |_, _| 0x500.into_ret());
        e.register(0x0049_1040, |_, _| 1.into_ret());
        e.register(0x0048_f7f0, |_, a| a[0].into_ret());
        e.register(0x005f_2420, |_, a| a[0].into_ret());
        e.register(0x0043_01b0, |_, _| 0x31.into_ret());
        e.register(0x005f_2440, |_, a| {
            (if a[0] == 0x31 { 0xaa } else { 0 }).into_ret()
        });
        e.register(0x0044_ddc0, |_, a| (a[0] + 1).into_ret());
        e.register(0x0064_50c0, |_, a| (a[0] == 0x1001).into_ret());
        e.register(0x0095_0a60, |_, _| 0x600.into_ret());
        stub(&mut e, &[0x0049_6080]);
        (e, actor, proc)
    }

    #[test]
    fn anim_action_clears_the_group_for_the_group_type_and_tells_the_process() {
        let (mut e, actor, proc) = anim_action_engine();
        e.call_log = Some(vec![]);
        e.call(0x008a_73e0, &args![actor, 3i32, 0u32]);
        let log = calls(&mut e);
        assert_eq!(called(&log, 0x0049_6080), vec![vec![0x500, 2, 0]]);
        assert_eq!(
            called(&log, target(PROCESS_VT, 0x3ec)),
            vec![vec![proc.addr(), 3, 0]]
        );
        // Another group type: not cleared.
        e.register(0x005f_2440, |_, _| 5.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x008a_73e0, &args![actor, 3i32, 0u32]);
        assert!(called(&calls(&mut e), 0x0049_6080).is_empty());
    }

    #[test]
    fn anim_action_blocked_cases_still_tell_the_process() {
        // The current action is 4.
        let (mut e, actor, proc) = anim_action_engine();
        answer(&mut e, PROCESS_VT, 0x3e4, 4);
        e.call_log = Some(vec![]);
        e.call(0x008a_73e0, &args![actor, 3i32, 0u32]);
        let log = calls(&mut e);
        assert!(called(&log, 0x0049_6080).is_empty());
        assert_eq!(called(&log, target(PROCESS_VT, 0x3ec)).len(), 1);
        // The new action is 7.
        let (mut e, actor, _) = anim_action_engine();
        e.call_log = Some(vec![]);
        e.call(0x008a_73e0, &args![actor, 7i32, 0u32]);
        assert!(called(&calls(&mut e), 0x0049_6080).is_empty());
        // The animation lacks group 2.
        let (mut e, actor, _) = anim_action_engine();
        e.register(0x0049_1040, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x008a_73e0, &args![actor, 3i32, 0u32]);
        assert!(called(&calls(&mut e), 0x0049_6080).is_empty());
        // The sequence's table entry is 2.
        let (mut e, actor, _) = anim_action_engine();
        e.set_global(ANIM_GROUP_TABLE + 3 * 0x24, 2u32);
        e.call_log = Some(vec![]);
        e.call(0x008a_73e0, &args![actor, 3i32, 3u32]);
        assert!(called(&calls(&mut e), 0x0049_6080).is_empty());
        // Another value in the entry does not block.
        e.set_global(ANIM_GROUP_TABLE + 3 * 0x24, 1u32);
        e.call_log = Some(vec![]);
        e.call(0x008a_73e0, &args![actor, 3i32, 3u32]);
        assert_eq!(called(&calls(&mut e), 0x0049_6080).len(), 1);
        // No animation, no process.
        let (mut e, _, _) = anim_action_engine();
        e.register(GET_ANIMATION, |_, _| Ret::default());
        let bare = new_actor(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x008a_73e0, &args![bare, 3i32, 0u32]);
        let log = calls(&mut e);
        assert!(called(&log, target(PROCESS_VT, 0x3ec)).is_empty());
        let _ = proc;
    }

    #[test]
    fn anim_action_weapon_actions_need_the_weapon_form_to_allow_them() {
        // Process method +0x148 gives the weapon, whose form is `weapon + 1`;
        // the form 0x1001 is allowed.
        for (weapon, action, cleared) in [
            (0x1000u32, 5i32, true),
            (0x2000, 5, false),
            (0x2000, 2, false),
            (0x2000, 6, false),
            (0x2000, 3, true),
            (0x2000, 4, false),
            (0x2000, 1, true),
            (0, 5, true),
        ] {
            let (mut e, actor, _) = anim_action_engine();
            answer(&mut e, PROCESS_VT, 0x148, weapon);
            e.call_log = Some(vec![]);
            e.call(0x008a_73e0, &args![actor, action, 0u32]);
            let seen = called(&calls(&mut e), 0x0049_6080).len();
            assert_eq!(seen == 1, cleared, "{weapon:x} {action}");
        }
    }

    #[test]
    fn anim_action_for_the_player_also_clears_the_first_person_animation() {
        let (mut e, actor, _) = anim_action_engine();
        e.set_global(PLAYER_CHARACTER, actor.addr());
        e.call_log = Some(vec![]);
        e.call(0x008a_73e0, &args![actor, 3i32, 0u32]);
        let log = calls(&mut e);
        assert_eq!(called(&log, 0x0095_0a60), vec![vec![actor.addr(), 1]]);
        assert_eq!(
            called(&log, 0x0049_6080),
            vec![vec![0x500, 2, 0], vec![0x600, 2, 0]]
        );
    }

    #[test]
    fn anim_action_is_the_process_answer_or_minus_one() {
        let mut e = engine();
        let (actor, _) = new_actor_with_process(&mut e);
        answer(&mut e, PROCESS_VT, 0x3e4, 6);
        assert_eq!(e.call(0x008a_7570, &args![actor]).i32(), 6);
        let bare = new_actor(&mut e);
        assert_eq!(e.call(0x008a_7570, &args![bare]).i32(), -1);
    }

    /// Doubles for the two package starters.
    fn starter_engine() -> (Engine, Ptr<Actor>, Ptr) {
        let mut e = engine();
        let (actor, proc) = new_actor_with_process(&mut e);
        e.set_global(PLAYER_CHARACTER, 0x1357u32);
        e.register(FORM_TYPE, |e, a| (e.mem.i8(a[0] + 0x20) as i32).into_ret());
        e.register(0x0067_0b90, |_, _| 0x4000.into_ret());
        e.register(OPERATOR_NEW, |_, _| 0x4100.into_ret());
        e.register(0x0067_f030, |_, a| a[0].into_ret());
        e.register(GET_PARENT_CELL, |_, _| 0x8000.into_ret());
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        stub(
            &mut e,
            &[
                0x0043_7bf0,
                0x0043_7bd0,
                0x0067_0fc0,
                0x0082_6b40,
                0x0082_6b90,
                0x0067_f3c0,
                0x0067_1d30,
                0x0067_0b30,
                0x0098_4f60,
                0x009e_d0d0,
            ],
        );
        (e, actor, proc)
    }

    #[test]
    fn starting_a_package_at_the_actor_builds_and_installs_it() {
        let (mut e, actor, proc) = starter_engine();
        e.call_log = Some(vec![]);
        e.call(0x008a_75a0, &args![actor]);
        let log = calls(&mut e);
        let steps = [
            0x0067_0b90,
            0x0067_0fc0,
            0x0082_6b40,
            0x0082_6b90,
            OPERATOR_NEW,
            0x0067_f030,
            0x0067_f3c0,
            0x0067_1d30,
            0x0067_0b30,
            0x0098_4f60,
            target(PROCESS_VT, 0x28),
            target(ACTOR_VT, 0x2f4),
        ];
        assert_eq!(order(&log, &steps), steps.to_vec());
        assert_eq!(called(&log, 0x0067_0b90), vec![vec![0x1a]]);
        assert_eq!(called(&log, 0x0067_0fc0), vec![vec![0x4000, 0x1a]]);
        assert_eq!(called(&log, 0x0067_f3c0), vec![vec![0x4100, actor.addr()]]);
        assert_eq!(called(&log, 0x0067_1d30), vec![vec![0x4000, 0x4100]]);
        assert_eq!(called(&log, 0x0067_0b30), vec![vec![0x4100, 1]]);
        assert_eq!(called(&log, 0x0098_4f60), vec![vec![0x4000, 0x15]]);
        assert_eq!(
            called(&log, target(PROCESS_VT, 0x28)),
            vec![vec![proc.addr()]]
        );
        assert_eq!(
            called(&log, target(ACTOR_VT, 0x2f4)),
            vec![vec![actor.addr(), 0x4000, 1, 1]]
        );
    }

    #[test]
    fn starting_a_package_at_the_actor_without_memory_skips_the_location_cleanup() {
        let (mut e, actor, _) = starter_engine();
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x008a_75a0, &args![actor]);
        let log = calls(&mut e);
        assert!(called(&log, 0x0067_f030).is_empty());
        assert!(called(&log, 0x0067_0b30).is_empty());
        assert_eq!(called(&log, 0x0067_1d30), vec![vec![0x4000, 0]]);
    }

    #[test]
    fn starting_a_package_at_the_actor_defers_to_process_method_4c0_when_it_says_7f() {
        let (mut e, actor, proc) = starter_engine();
        answer(&mut e, PROCESS_VT, 0x4d0, 0x7f);
        e.call_log = Some(vec![]);
        e.call(0x008a_75a0, &args![actor]);
        let log = calls(&mut e);
        assert_eq!(
            called(&log, target(PROCESS_VT, 0x4c0)),
            vec![vec![proc.addr(), actor.addr(), 0, 0, 0x7f]]
        );
        assert!(called(&log, 0x0067_0b90).is_empty());
    }

    #[test]
    fn package_starters_stop_at_their_gates() {
        let package: u32 = 0;
        let _ = package;
        for addr in [0x008a_75a0u32, 0x008a_7760] {
            // Either of the two checks holds.
            for gate in [0x0043_7bf0u32, 0x0043_7bd0] {
                let (mut e, actor, _) = starter_engine();
                e.register(gate, |_, _| 1.into_ret());
                e.call_log = Some(vec![]);
                e.call(addr, &args![actor]);
                assert!(called(&calls(&mut e), 0x0067_0b90).is_empty());
            }
            // The current package already has type 0x1a.
            let (mut e, actor, _) = starter_engine();
            let current: Ptr = Ptr::new(e.mem.alloc(0x40));
            e.mem.set_u8(current.addr() + 0x20, 0x1a);
            answer(&mut e, PROCESS_VT, 0x27c, current.addr());
            e.call_log = Some(vec![]);
            e.call(addr, &args![actor]);
            assert!(called(&calls(&mut e), 0x0067_0b90).is_empty());
            // Another type does not stop it.
            e.mem.set_u8(current.addr() + 0x20, 0x19);
            e.call_log = Some(vec![]);
            e.call(addr, &args![actor]);
            assert_eq!(called(&calls(&mut e), 0x0067_0b90).len(), 1);
        }
    }

    #[test]
    fn starting_a_back_up_package_initializes_it() {
        let (mut e, actor, proc) = starter_engine();
        e.set_global(PLAYER_CHARACTER, 0x1357u32);
        e.call_log = Some(vec![]);
        e.call(0x008a_7760, &args![actor]);
        let log = calls(&mut e);
        assert_eq!(called(&log, 0x0067_0b90), vec![vec![0x27]]);
        assert_eq!(called(&log, 0x0067_0fc0), vec![vec![0x4000, 0x27]]);
        assert_eq!(
            called(&log, DYNAMIC_CAST),
            vec![vec![0x4000, 0, TYPE_TES_PACKAGE, TYPE_BACK_UP_PACKAGE, 0]]
        );
        assert_eq!(
            called(&log, 0x009e_d0d0),
            vec![vec![0x4000, actor.addr(), 0x8000, 0x1357]]
        );
        assert_eq!(called(&log, 0x0098_4f60), vec![vec![0x4000, 0x30]]);
        assert_eq!(
            called(&log, target(PROCESS_VT, 0x28)),
            vec![vec![proc.addr()]]
        );
        assert_eq!(
            called(&log, target(ACTOR_VT, 0x2f4)),
            vec![vec![actor.addr(), 0x4000, 1, 1]]
        );
        // A package that is not a back-up package is not initialized.
        e.register(DYNAMIC_CAST, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x008a_7760, &args![actor]);
        let log = calls(&mut e);
        assert!(called(&log, 0x009e_d0d0).is_empty());
        assert_eq!(called(&log, target(ACTOR_VT, 0x2f4)).len(), 1);
    }

    /// Everything `008a5580` needs: an actor with an animation, a process
    /// whose current package is a special-idle package (type 0x1c), a
    /// speaker and the player.
    struct UpdateWorld {
        e: Engine,
        actor: Ptr<Actor>,
        proc: Ptr,
        idle: Ptr,
        speaker: Ptr,
        player: Ptr,
        face: Ptr,
    }

    fn update_world() -> UpdateWorld {
        let mut e = engine();
        e.map(0x011c_f000, 0x1000);
        let (actor, proc) = new_actor_with_process(&mut e);
        e.mem.set_u32(actor.addr() + 0xa4, OBJECT_VT);
        e.mem.set_u32(actor.addr() + 0xc, 0xaa11);
        let idle: Ptr = Ptr::new(e.mem.alloc(0x100));
        e.mem.set_u8(idle.addr() + 0x20, 0x1c);
        let speaker: Ptr = Ptr::new(e.mem.alloc(0x100));
        e.mem.set_u32(speaker.addr(), 0x0d00d);
        e.mem.set_u32(speaker.addr() + 8, 0x0b05e);
        e.mem.set_u32(speaker.addr() + 0xc, 0x0c0de);
        e.mem.set_u32(speaker.addr() + 0x10, 0x0ace);
        e.mem.set_u32(speaker.addr() + 0x18, 0x0e00);
        e.mem.set_u32(speaker.addr() + 0x1c, 0x0f00);
        e.mem.set_u8(speaker.addr() + 0x24, 1);
        let player = new_object(&mut e);
        e.set_global(PLAYER_CHARACTER, player.addr());
        let face = new_object(&mut e);
        answer(&mut e, ACTOR_VT, 0x1e4, 0x6000);
        answer(&mut e, PROCESS_VT, 0x22c, idle.addr());
        e.register_double(target(ACTOR_VT, 0x284), |_, _| Ret {
            st0: 1.5,
            ..Ret::default()
        });
        e.register(FORM_TYPE, |e, a| (e.mem.i8(a[0] + 0x20) as i32).into_ret());
        e.register(FORM_ID, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register(GET_SAVE_FORM, |e, a| e.mem.u32(a[0] + 0x4c).into_ret());
        e.register(0x0054_3c30, |e, a| e.mem.u8(a[0] + 0x24).into_ret());
        e.register(0x0044_1110, |e, a| e.mem.u32(a[0] + 0x1c).into_ret());
        e.register(0x0096_11e0, |e, a| e.mem.u32(a[0] + 0x18).into_ret());
        e.register(0x0068_15c0, |_, a| a[0].into_ret());
        e.register(0x0040_48e0, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(0x0044_ddc0, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(0x0046_0140, |_, a| (a[0] + 0x10).into_ret());
        e.register(0x0055_9450, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(0x00ad_9060, |_, _| 0x9000.into_ret());
        e.register(SOUND_HANDLE_INIT, |e, a| {
            e.mem.set_u32(a[0], u32::MAX);
            Ret::default()
        });
        e.register(GET_FACE_ANIMATION_DATA, |e, _| e.mem.u32(KNOB).into_ret());
        e.set_global(KNOB, face.addr());
        e.register(0x00ad_8930, |e, _| e.mem.u32(KNOB + 4).into_ret());
        e.register(SCRIPT_FLAG, |e, _| e.mem.u32(KNOB + 8).into_ret());
        e.register(INTERFACE_IN_DIALOG, |e, _| e.mem.u32(KNOB + 12).into_ret());
        e.register(0x0049_3bb0, |_, _| Ret::default());
        e.register(0x0045_cd60, |_, _| 1.into_ret());
        e.register(0x0049_85f0, |_, _| Ret::default());
        stub(
            &mut e,
            &[
                0x0093_3150,
                0x00ad_bfd0,
                0x00ad_c050,
                SOUND_HANDLE_DESTRUCT,
                0x00c7_48b0,
                0x009d_db50,
                0x009d_ce20,
                0x009c_9900,
                0x0089_d620,
                0x005f_36f0,
                0x00c5_2960,
            ],
        );
        UpdateWorld {
            e,
            actor,
            proc,
            idle,
            speaker,
            player,
            face,
        }
    }

    #[test]
    fn update_without_animation_does_nothing() {
        let mut w = update_world();
        answer(&mut w.e, ACTOR_VT, 0x1e4, 0);
        w.e.set(w.actor, Actor::bSpeakingDone, 1);
        w.e.call_log = Some(vec![]);
        w.e.call(0x008a_5580, &args![w.actor, 0.5f32, w.speaker, 1u8]);
        let log = calls(&mut w.e);
        assert!(called(&log, target(ACTOR_VT, 0x140)).is_empty());
        assert!(called(&log, SOUND_HANDLE_INIT).is_empty());
        assert_eq!(w.e.get(w.actor, Actor::bTalkingToPlayer), 0);
    }

    #[test]
    fn update_starts_the_speech_of_a_speaker_without_a_form() {
        let mut w = update_world();
        w.e.set(w.actor, Actor::bSpeakingDone, 1);
        w.e.set(w.actor, Actor::bSoundFileDone, 1);
        w.e.set(w.actor, Actor::bVoiceFileDone, 1);
        w.e.call_log = Some(vec![]);
        w.e.call(0x008a_5580, &args![w.actor, 0.5f32, w.speaker, 1u8]);
        let log = calls(&mut w.e);
        assert_eq!(w.e.global::<u32>(SPEAKING_ACTOR), w.actor.addr());
        assert_eq!(w.e.get(w.actor, Actor::bTalkingToPlayer), 1);
        assert_eq!(w.e.get(w.actor, Actor::bUseEmotion), 1);
        let handle = called(&log, SOUND_HANDLE_INIT)[0][0];
        let speak = called(&log, target(ACTOR_VT, 0x284));
        assert_eq!(
            speak,
            vec![vec![
                w.actor.addr(),
                0x0ace,
                handle,
                0x0b05e,
                0x0c0de,
                0x0d00d,
                0x0e00,
                0x0f00,
                w.player.addr(),
                1,
                1,
                0,
                1,
                1
            ]]
        );
        // The speech length goes to the process; the voice flag is cleared.
        assert_eq!(
            called(&log, target(PROCESS_VT, 0x318)),
            vec![vec![w.proc.addr(), 1.5f32.to_bits()]]
        );
        assert_eq!(w.e.get(w.actor, Actor::bVoiceFileDone), 0);
        // The completion callbacks are registered for the actor (its id
        // 0xaa11, the handle word -1).
        assert_eq!(
            called(&log, 0x00ad_c050),
            vec![vec![0x9000, u32::MAX, 0x0093_5f00, 0xaa11]]
        );
        assert_eq!(
            called(&log, 0x00ad_bfd0),
            vec![vec![0x9000, u32::MAX, 0x008b_c590, 0xaa11]]
        );
        assert!(called(&log, 0x0093_3150).is_empty());
        // The special idle package now points at the actor.
        assert_eq!(w.e.mem.u32(w.idle.addr() + 0x94), w.actor.addr());
    }

    #[test]
    fn update_registers_no_callbacks_without_the_notify_flag() {
        let mut w = update_world();
        w.e.set(w.actor, Actor::bSpeakingDone, 1);
        w.e.set(w.actor, Actor::bSoundFileDone, 1);
        w.e.call_log = Some(vec![]);
        w.e.call(0x008a_5580, &args![w.actor, 0.5f32, w.speaker, 0u8]);
        let log = calls(&mut w.e);
        assert_eq!(called(&log, target(ACTOR_VT, 0x284)).len(), 1);
        assert!(called(&log, 0x00ad_c050).is_empty());
        assert!(called(&log, 0x00ad_bfd0).is_empty());
    }

    #[test]
    fn update_queues_the_sound_of_a_speaker_with_a_form() {
        let mut w = update_world();
        let form: Ptr = Ptr::new(w.e.mem.alloc(0x100));
        w.e.mem.set_u32(form.addr() + 0xc, 0xf0f0);
        w.e.mem.set_u32(w.speaker.addr() + 0x4c, form.addr());
        w.e.set(w.actor, Actor::bSpeakingDone, 1);
        w.e.set(w.actor, Actor::bSoundFileDone, 1);
        w.e.call_log = Some(vec![]);
        w.e.call(0x008a_5580, &args![w.actor, 0.5f32, w.speaker, 1u8]);
        let log = calls(&mut w.e);
        let queued = called(&log, 0x0093_3150);
        assert_eq!(queued.len(), 1);
        assert_eq!(queued[0][0], w.actor.addr());
        assert_eq!(queued[0][2..], [0xf0f0, 0, 0x102, 1]);
        assert_eq!(
            called(&log, 0x00ad_bfd0),
            vec![vec![0x9000, 0, 0x0093_5cc0, 0xaa11]]
        );
        // The flags are cleared and no speech starts.
        assert_eq!(w.e.get(w.actor, Actor::bSoundFileDone), 0);
        assert_eq!(w.e.get(w.actor, Actor::bSpeakingDone), 0);
        assert!(called(&log, target(ACTOR_VT, 0x284)).is_empty());
        assert_eq!(w.e.get(w.actor, Actor::bUseEmotion), 1);
    }

    #[test]
    fn update_does_not_speak_unless_the_actor_is_ready() {
        for (speaking, has_speaker) in [(0u8, true), (1, false)] {
            let mut w = update_world();
            w.e.set(w.actor, Actor::bSpeakingDone, speaking);
            let speaker = if has_speaker { w.speaker } else { Ptr::NULL };
            w.e.call_log = Some(vec![]);
            w.e.call(0x008a_5580, &args![w.actor, 0.5f32, speaker, 1u8]);
            let log = calls(&mut w.e);
            assert!(called(&log, SOUND_HANDLE_INIT).is_empty());
            assert_eq!(w.e.get(w.actor, Actor::bTalkingToPlayer), 0);
            assert_eq!(w.e.global::<u32>(SPEAKING_ACTOR), 0);
        }
    }

    /// The actor's mover, the actor's and the player's positions, and the
    /// numbers the angle computation sees: the direction `direction`, the
    /// actor's rotation `rotation`.
    fn mover_world(direction: f32, rotation: f32) -> (UpdateWorld, Ptr, Ptr) {
        let mut w = update_world();
        let mover: Ptr = Ptr::new(w.e.mem.alloc(0x40));
        w.e.set(w.actor, Actor::pActorMover, mover);
        let player_position = floats(&mut w.e, &[10.0, 20.0, 30.0]);
        let own_position = floats(&mut w.e, &[1.0, 2.0, 3.0]);
        answer(&mut w.e, OBJECT_VT, 0x1f4, player_position.addr());
        answer(&mut w.e, ACTOR_VT, 0x1f4, own_position.addr());
        let rotation_block = floats(&mut w.e, &[0.0, 0.0, rotation]);
        w.e.set_global(KNOB + 16, direction);
        w.e.set_global(PI, std::f32::consts::PI);
        w.e.set_global(MINUS_PI, -std::f32::consts::PI);
        w.e.set_global(ONE_DEGREE, std::f64::consts::PI / 180.0);
        w.e.register(0x0043_9ef0, |_, _| Ret::default());
        w.e.register(0x004b_13c0, |e, _| e.mem.f32(KNOB + 16).into_ret());
        w.e.register(0x0043_0830, |e, _| e.mem.u32(KNOB + 20).into_ret());
        w.e.set_global(KNOB + 20, rotation_block.addr());
        stub(&mut w.e, &[0x004e_44f0]);
        w.e.register(0x0040_8840, |_, a| {
            (f32::from_bits(a[0]).abs() as f64).into_ret()
        });
        (w, mover, player_position)
    }

    #[test]
    fn update_turns_the_actor_towards_the_player_for_a_big_angle() {
        let (mut w, mover, player_position) = mover_world(1.0, 0.25);
        w.e.call_log = Some(vec![]);
        w.e.call(0x008a_5580, &args![w.actor, 0.5f32, NO_SPEAKER, 0u8]);
        let log = calls(&mut w.e);
        // The player's position words go to the rotation request.
        let pos = player_position.addr();
        let words = [w.e.mem.u32(pos), w.e.mem.u32(pos + 4), w.e.mem.u32(pos + 8)];
        assert_eq!(
            called(&log, 0x009d_ce20),
            vec![vec![mover.addr(), words[0], words[1], words[2], 0]]
        );
        // The move update is bracketed by the mover's flag.
        let steps = [0x009c_9900, target(ACTOR_VT, 0x348), 0x009c_9900];
        assert_eq!(order(&log, &steps), steps.to_vec());
        assert_eq!(
            called(&log, 0x009c_9900),
            vec![vec![mover.addr(), 1], vec![mover.addr(), 0]]
        );
        assert_eq!(
            called(&log, target(ACTOR_VT, 0x348)),
            vec![vec![w.actor.addr(), 0.5f32.to_bits(), 0]]
        );
        // The vector from the player is built from the two positions.
        assert_eq!(called(&log, 0x0043_9ef0).len(), 1);
        assert_eq!(called(&log, 0x0043_9ef0)[0][0], pos);
    }

    #[test]
    fn update_does_not_turn_for_a_small_angle_or_a_rotating_mover() {
        // 1 degree = 0.01745: 0.01 is below it.
        let (mut w, _, _) = mover_world(0.26, 0.25);
        w.e.call_log = Some(vec![]);
        w.e.call(0x008a_5580, &args![w.actor, 0.5f32, NO_SPEAKER, 0u8]);
        let log = calls(&mut w.e);
        assert!(called(&log, 0x009d_ce20).is_empty());
        assert_eq!(called(&log, target(ACTOR_VT, 0x348)).len(), 1);
        // A mover that is rotating already.
        let (mut w, _, _) = mover_world(1.0, 0.25);
        w.e.register(0x009d_db50, |_, _| 1.into_ret());
        w.e.call_log = Some(vec![]);
        w.e.call(0x008a_5580, &args![w.actor, 0.5f32, NO_SPEAKER, 0u8]);
        let log = calls(&mut w.e);
        assert!(called(&log, 0x009d_ce20).is_empty());
        assert_eq!(called(&log, target(ACTOR_VT, 0x348)).len(), 1);
        // An actor with a package change pending, or a living body check.
        let (mut w, _, _) = mover_world(1.0, 0.25);
        answer(&mut w.e, ACTOR_VT, 0x214, 3);
        w.e.call_log = Some(vec![]);
        w.e.call(0x008a_5580, &args![w.actor, 0.5f32, NO_SPEAKER, 0u8]);
        assert!(called(&calls(&mut w.e), target(ACTOR_VT, 0x348)).is_empty());
        let (mut w, _, _) = mover_world(1.0, 0.25);
        w.e.register(0x0049_3bb0, |_, _| 1.into_ret());
        w.e.call_log = Some(vec![]);
        w.e.call(0x008a_5580, &args![w.actor, 0.5f32, NO_SPEAKER, 0u8]);
        assert!(called(&calls(&mut w.e), target(ACTOR_VT, 0x348)).is_empty());
    }

    #[test]
    fn update_relaxes_the_bones_of_an_idle_process() {
        let mut w = update_world();
        w.e.register(0x0045_cd60, |_, _| Ret::default());
        w.e.register(0x0089_d620, |_, _| 0x7000.into_ret());
        w.e.register(0x005f_36f0, |_, _| 1.into_ret());
        w.e.call_log = Some(vec![]);
        w.e.call(0x008a_5580, &args![w.actor, 0.5f32, NO_SPEAKER, 0u8]);
        let log = calls(&mut w.e);
        assert_eq!(called(&log, 0x00c5_2960), vec![vec![0x7000, 0]]);
        assert_eq!(called(&log, target(ACTOR_VT, 0x140)).len(), 1);
    }

    #[test]
    fn update_sets_the_face_from_the_emotion() {
        // Emotion 1 and 5 pick moods 6 and 5; others do nothing.
        for (emotion, mood) in [(1u32, Some(6u32)), (5, Some(5)), (3, None)] {
            let mut w = update_world();
            w.e.set(w.actor, Actor::ePersuasionEmotion, emotion);
            w.e.mem.set_u32(w.idle.addr() + 0x94, 0x1234);
            w.e.call_log = Some(vec![]);
            w.e.call(0x008a_5580, &args![w.actor, 0.5f32, NO_SPEAKER, 0u8]);
            let log = calls(&mut w.e);
            let mood_calls = called(&log, target(OBJECT_VT, 0x11c));
            match mood {
                Some(m) => assert_eq!(mood_calls, vec![vec![w.face.addr(), m, 0]], "{emotion}"),
                None => assert!(mood_calls.is_empty(), "{emotion}"),
            }
            // Not playing: the idle package forgets its pointer.
            assert_eq!(w.e.mem.u32(w.idle.addr() + 0x94), 0);
        }
    }

    #[test]
    fn update_dialogue_emotion_asks_for_the_actor_value_or_resets() {
        // Emotion 8 with the script flag clear: the face is reset.
        let mut w = update_world();
        w.e.set(w.actor, Actor::ePersuasionEmotion, 8);
        w.e.call_log = Some(vec![]);
        w.e.call(0x008a_5580, &args![w.actor, 0.5f32, NO_SPEAKER, 0u8]);
        let log = calls(&mut w.e);
        assert_eq!(
            called(&log, target(OBJECT_VT, 0xb4)),
            vec![vec![w.face.addr(), 0, 1, 0, 0, 0, 0]]
        );
        // With the flag set, the interface in a dialogue and a free process:
        // the actor value (slot +8 of the sub-object at +0xa4, 00880580, RET 4: with 4), and
        // the face gets it with the 0 pushed first (008a5acf).
        let mut w = update_world();
        w.e.set(w.actor, Actor::ePersuasionEmotion, 8);
        w.e.set_global(KNOB + 8, 1u32);
        w.e.set_global(KNOB + 12, 1u32);
        answer(&mut w.e, OBJECT_VT, 0x8, 0x55);
        w.e.call_log = Some(vec![]);
        w.e.call(0x008a_5580, &args![w.actor, 0.5f32, NO_SPEAKER, 0u8]);
        let log = calls(&mut w.e);
        assert_eq!(
            called(&log, target(OBJECT_VT, 0x8)),
            vec![vec![w.actor.addr() + 0xa4, 4]]
        );
        assert_eq!(
            called(&log, target(OBJECT_VT, 0x11c)),
            vec![vec![w.face.addr(), 0x55, 0]]
        );
        // A busy process, or no dialogue, does nothing.
        for busy in [true, false] {
            let mut w = update_world();
            w.e.set(w.actor, Actor::ePersuasionEmotion, 8);
            w.e.set_global(KNOB + 8, 1u32);
            w.e.set_global(KNOB + 12, if busy { 1u32 } else { 0 });
            if busy {
                answer(&mut w.e, PROCESS_VT, 0x2e8, 1);
            }
            w.e.call_log = Some(vec![]);
            w.e.call(0x008a_5580, &args![w.actor, 0.5f32, NO_SPEAKER, 0u8]);
            let log = calls(&mut w.e);
            assert!(called(&log, target(OBJECT_VT, 0x11c)).is_empty(), "{busy}");
            assert!(called(&log, target(OBJECT_VT, 0xb4)).is_empty(), "{busy}");
        }
    }

    #[test]
    fn update_while_a_sound_plays_only_the_dialogue_emotion_acts() {
        // Playing, emotion 8: the reset runs on the face looked up again.
        let mut w = update_world();
        w.e.set(w.actor, Actor::ePersuasionEmotion, 8);
        w.e.set_global(KNOB + 4, 1u32);
        w.e.mem.set_u32(w.idle.addr() + 0x94, 0x1234);
        w.e.call_log = Some(vec![]);
        w.e.call(0x008a_5580, &args![w.actor, 0.5f32, NO_SPEAKER, 0u8]);
        let log = calls(&mut w.e);
        assert_eq!(called(&log, target(OBJECT_VT, 0xb4)).len(), 1);
        assert_eq!(called(&log, GET_FACE_ANIMATION_DATA).len(), 2);
        // The idle package is left alone while the sound plays.
        assert_eq!(w.e.mem.u32(w.idle.addr() + 0x94), 0x1234);
        // Any other emotion does nothing.
        let mut w = update_world();
        w.e.set(w.actor, Actor::ePersuasionEmotion, 1);
        w.e.set_global(KNOB + 4, 1u32);
        w.e.call_log = Some(vec![]);
        w.e.call(0x008a_5580, &args![w.actor, 0.5f32, NO_SPEAKER, 0u8]);
        let log = calls(&mut w.e);
        assert!(called(&log, target(OBJECT_VT, 0x11c)).is_empty());
        assert!(called(&log, target(OBJECT_VT, 0xb4)).is_empty());
    }

    #[test]
    fn update_restarts_a_finished_special_idle() {
        let mut w = update_world();
        w.e.register(0x0049_85f0, |_, a| (a[0] == 0x6000).into_ret());
        w.e.call_log = Some(vec![]);
        w.e.call(0x008a_5580, &args![w.actor, 0.5f32, NO_SPEAKER, 0u8]);
        let log = calls(&mut w.e);
        assert_eq!(
            called(&log, target(PROCESS_VT, 0x44)),
            vec![vec![w.proc.addr(), w.actor.addr(), 0, 2, 1, 0, 1]]
        );
        // The process says it still has something to do (+0x718).
        answer(&mut w.e, PROCESS_VT, 0x718, 1);
        w.e.call_log = Some(vec![]);
        w.e.call(0x008a_5580, &args![w.actor, 0.5f32, NO_SPEAKER, 0u8]);
        assert!(called(&calls(&mut w.e), target(PROCESS_VT, 0x44)).is_empty());
        // Without a special-idle package nothing is restarted.
        let mut w = update_world();
        w.e.register(0x0049_85f0, |_, _| 1.into_ret());
        w.e.mem.set_u8(w.idle.addr() + 0x20, 0x1b);
        w.e.call_log = Some(vec![]);
        w.e.call(0x008a_5580, &args![w.actor, 0.5f32, NO_SPEAKER, 0u8]);
        assert!(called(&calls(&mut w.e), target(PROCESS_VT, 0x44)).is_empty());
    }

    #[test]
    fn update_tells_the_ragdoll_controller() {
        let mut w = update_world();
        let ragdoll: Ptr = Ptr::new(w.e.mem.alloc(0x40));
        w.e.set(w.actor, Actor::pRagdollController, ragdoll);
        w.e.call_log = Some(vec![]);
        w.e.call(0x008a_5580, &args![w.actor, 0.5f32, NO_SPEAKER, 0u8]);
        assert_eq!(
            called(&calls(&mut w.e), 0x00c7_48b0),
            vec![vec![ragdoll.addr()]]
        );
    }

    // ---------------------------------------------------------------------
    // Second session: `008a7870` up to `008ac6f0`.
    // ---------------------------------------------------------------------

    /// Vtables of the second session's fake actor, process and objects.
    const ACTOR2_VT: u32 = 0x0910_0000;
    const PROCESS2_VT: u32 = 0x0912_0000;
    const OBJECT2_VT: u32 = 0x0914_0000;

    const ACTOR2_SLOTS: [u32; 21] = [
        0x22c, 0x230, 0x1e4, 0x1e8, 0x214, 0x428, 0x2f4, 0x48, 0x3e8, 0x17c, 0x3ec, 0x240, 0x24c,
        0x248, 0x384, 0x3f4, 0x21c, 0x1d0, 0x2a0, 0x2e8, 0x1f4,
    ];
    const PROCESS2_SLOTS: [u32; 22] = [
        0x22c, 0x27c, 0x280, 0x3e8, 0x614, 0x14c, 0x148, 0x440, 0x6b8, 0x168, 0x160, 0x3e4, 0x1b8,
        0x3ec, 0x284, 0x40c, 0x464, 0x478, 0x468, 0x5b4, 0x7a8, 0x44c,
    ];
    const OBJECT2_SLOTS: [u32; 15] = [
        0x0, 0x8, 0xc, 0x28, 0x2c, 0x30, 0x34, 0x38, 0x8c, 0xd8, 0xdc, 0x130, 0x188, 0xf4, 0x1f4,
    ];

    /// An engine for the second session's tests: the first session's pages
    /// and the vtables above.
    fn engine2() -> Engine {
        let mut e = engine();
        for page in [
            0x0101_7000,
            0x011c_f000,
            0x011d_0000,
            0x011f_1000,
            0x011f_2000,
        ] {
            e.map(page, 0x1000);
        }
        make_vtable(&mut e, ACTOR2_VT, &ACTOR2_SLOTS);
        make_vtable(&mut e, PROCESS2_VT, &PROCESS2_SLOTS);
        make_vtable(&mut e, OBJECT2_VT, &OBJECT2_SLOTS);
        // Plain accessors every test of this session may reach.
        e.register(SELF, |_, a| a[0].into_ret());
        e.register(ITEM_COUNT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(NODE_IS_EMPTY, |e, a| {
            (e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0).into_ret()
        });
        e.register(WORD_AT_8, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e
    }

    /// An actor of the second vtable with a process (second vtable too).
    fn actor2(e: &mut Engine) -> (Ptr<Actor>, Ptr) {
        let actor: Ptr<Actor> = e.new_object();
        e.mem.set_u32(actor.addr(), ACTOR2_VT);
        let proc = Ptr::new(e.mem.alloc(0x800));
        e.mem.set_u32(proc.addr(), PROCESS2_VT);
        e.set(actor, Actor::pCurrentProcess, proc);
        (actor, proc)
    }

    /// A zeroed object of `size` bytes with the second object vtable.
    fn object2(e: &mut Engine, size: u32) -> Ptr {
        let object = Ptr::new(e.mem.alloc(size));
        e.mem.set_u32(object.addr(), OBJECT2_VT);
        object
    }

    /// `FORM_TYPE` reads the type byte at `+0x20` of the form, as the first
    /// session's tests do.
    fn form_type_from_byte(e: &mut Engine) {
        e.register(FORM_TYPE, |e, a| (e.mem.i8(a[0] + 0x20) as i32).into_ret());
    }

    #[test]
    fn eating_needs_the_package_type_table_entry_5() {
        let mut e = engine2();
        let (actor, _) = actor2(&mut e);
        form_type_from_byte(&mut e);
        let package = object2(&mut e, 0x40);
        e.mem.set_u8(package.addr() + 0x20, 3);
        e.mem.set_u32(package.addr() + 0x18, 2);
        e.register(0x0096_11e0, |e, a| e.mem.u32(a[0] + 0x18).into_ret());
        let table = Ptr::<()>::new(e.mem.alloc(0x20));
        e.mem.set_u32(table.addr() + 4, 5);
        e.set_global(PACKAGE_TYPE_TABLE + 8, table.addr());
        answer(&mut e, PROCESS2_VT, 0x22c, package.addr());
        answer(&mut e, PROCESS2_VT, 0x280, 1);
        assert!(e.call(0x008a_7870, &args![actor]).bool());
        // Another entry of the table.
        answer(&mut e, PROCESS2_VT, 0x280, 0);
        assert!(!e.call(0x008a_7870, &args![actor]).bool());
        // Another package type.
        answer(&mut e, PROCESS2_VT, 0x280, 1);
        e.mem.set_u8(package.addr() + 0x20, 4);
        assert!(!e.call(0x008a_7870, &args![actor]).bool());
        // No package, no process.
        answer(&mut e, PROCESS2_VT, 0x22c, 0);
        assert!(!e.call(0x008a_7870, &args![actor]).bool());
        let bare: Ptr<Actor> = e.new_object();
        assert!(!e.call(0x008a_7870, &args![bare]).bool());
    }

    #[test]
    fn surfacing_is_package_type_0x1d() {
        let mut e = engine2();
        let (actor, _) = actor2(&mut e);
        form_type_from_byte(&mut e);
        let package = object2(&mut e, 0x40);
        answer(&mut e, PROCESS2_VT, 0x27c, package.addr());
        e.mem.set_u8(package.addr() + 0x20, 0x1d);
        assert!(e.call(0x008a_7a40, &args![actor]).bool());
        e.mem.set_u8(package.addr() + 0x20, 0x1c);
        assert!(!e.call(0x008a_7a40, &args![actor]).bool());
        answer(&mut e, PROCESS2_VT, 0x27c, 0);
        assert!(!e.call(0x008a_7a40, &args![actor]).bool());
        let bare: Ptr<Actor> = e.new_object();
        assert!(!e.call(0x008a_7a40, &args![bare]).bool());
    }

    #[test]
    fn skip_fall_out_behavior_negates_the_package_test_of_the_kind() {
        let tests = [
            (0u32, 0x0067_a850u32),
            (1, 0x0067_a8d0),
            (2, 0x0067_a950),
            (4, 0x0067_aa50),
            (5, 0x0067_aad0),
            (6, 0x0067_ab50),
            (7, 0x0067_abd0),
            (8, 0x0067_ac50),
        ];
        for (kind, test) in tests {
            let mut e = engine2();
            let (actor, _) = actor2(&mut e);
            let package = object2(&mut e, 0x40);
            answer(&mut e, PROCESS2_VT, 0x22c, package.addr());
            e.register(0x0067_a380, |_, _| 1.into_ret());
            for (_, other) in tests {
                e.register(other, |_, _| 0.into_ret());
            }
            e.register(test, |_, _| 1.into_ret());
            // The kind's test holds: do not skip.
            assert!(!e.call(0x008a_78f0, &args![actor, kind]).bool(), "{kind}");
            e.register(test, |_, _| 0.into_ret());
            assert!(e.call(0x008a_78f0, &args![actor, kind]).bool(), "{kind}");
        }
        let mut e = engine2();
        let (actor, _) = actor2(&mut e);
        let package = object2(&mut e, 0x40);
        answer(&mut e, PROCESS2_VT, 0x22c, package.addr());
        e.register(0x0067_a380, |_, _| 1.into_ret());
        // Kind 3 and kinds above 8 are never skipped.
        assert!(!e.call(0x008a_78f0, &args![actor, 3u32]).bool());
        assert!(!e.call(0x008a_78f0, &args![actor, 9u32]).bool());
        // The general test failing, or no package, never skips.
        e.register(0x0067_a380, |_, _| 0.into_ret());
        e.register(0x0067_a850, |_, _| 0.into_ret());
        assert!(!e.call(0x008a_78f0, &args![actor, 0u32]).bool());
        e.register(0x0067_a380, |_, _| 1.into_ret());
        answer(&mut e, PROCESS2_VT, 0x22c, 0);
        assert!(!e.call(0x008a_78f0, &args![actor, 0u32]).bool());
    }

    /// The animation doubles of `fn_008a7a90`: the slot values of the
    /// animation come from its words (`anim + 0x10 + 4 * slot`), the group of
    /// slot `n` from the half words at `anim + 0x40 + 2 * n`.
    fn animation_doubles(e: &mut Engine, anim: Ptr, actor_group: u32) {
        e.register_double(GET_ANIMATION, move |_, _| anim.addr().into_ret());
        e.register(0x0049_1040, |e, a| {
            e.mem.u32(a[0] + 0x10 + 4 * a[1]).into_ret()
        });
        e.register(0x0043_01b0, |e, a| {
            (e.mem.u16(a[0] + 0x40 + 2 * a[1]) as u32).into_ret()
        });
        e.register_double(0x0089_7910, move |_, _| actor_group.into_ret());
        e.register(0x005f_2440, |_, a| (a[0] & 0xff).into_ret());
        e.register(0x0049_6080, |_, _| Ret::default());
        e.register(0x0049_4740, |_, _| Ret::default());
    }

    #[test]
    fn clearing_animation_groups_and_ending_the_action() {
        let mut e = engine2();
        let (actor, _) = actor2(&mut e);
        let anim = object2(&mut e, 0x80);
        animation_doubles(&mut e, anim, 0x33);
        e.set_global(PLAYER_CHARACTER, 0u32);
        // Slot 1 holds the group the process names (method +0x3e8).
        answer(&mut e, PROCESS2_VT, 0x3e8, 0x777);
        e.mem.set_u32(anim.addr() + 0x14, 0x777);
        // Slot 0's group differs from the actor's group; slot 2 plays an
        // ordinary group (type 0x10).
        e.mem.set_u16(anim.addr() + 0x40, 0x11);
        e.mem.set_u16(anim.addr() + 0x44, 0x10);
        e.call_log = Some(vec![]);
        e.call(0x008a_7a90, &args![actor]);
        let log = calls(&mut e);
        let cleared: Vec<u32> = called(&log, 0x0049_6080).iter().map(|c| c[1]).collect();
        assert_eq!(cleared, vec![0, 1, 2, 3, 7]);
        // The slot holding the group makes the action end: method +0x3ec
        // gets (-1, 0).
        assert_eq!(
            called(&log, target(PROCESS2_VT, 0x3ec)),
            vec![vec![actor_proc(&e, actor), u32::MAX, 0]]
        );
        // Slot 0 holds nothing, so the base group is played again.
        assert_eq!(
            called(&log, 0x0049_4740),
            vec![vec![anim.addr(), 0x33, 0, u32::MAX, u32::MAX]]
        );
    }

    /// The process pointer of `actor`.
    fn actor_proc(e: &Engine, actor: Ptr<Actor>) -> u32 {
        e.mem.u32(actor.addr() + 0x68)
    }

    #[test]
    fn group_two_stays_for_types_0xe6_to_0xeb_and_a_busy_actor_does_nothing() {
        let mut e = engine2();
        let (actor, _) = actor2(&mut e);
        let anim = object2(&mut e, 0x80);
        animation_doubles(&mut e, anim, 0x11);
        e.set_global(PLAYER_CHARACTER, 0u32);
        // Slot 0's group equals the actor's group: not cleared; slot 2's
        // type 0xe8 is kept.
        e.mem.set_u16(anim.addr() + 0x40, 0x11);
        e.mem.set_u16(anim.addr() + 0x44, 0xe8);
        e.mem.set_u32(anim.addr() + 0x10, 5);
        e.call_log = Some(vec![]);
        e.call(0x008a_7a90, &args![actor]);
        let log = calls(&mut e);
        let cleared: Vec<u32> = called(&log, 0x0049_6080).iter().map(|c| c[1]).collect();
        assert_eq!(cleared, vec![1, 3, 7]);
        // Nothing named by the process (0), so no action change.
        assert!(called(&log, target(PROCESS2_VT, 0x3ec)).is_empty());
        // Slot 0 holds something, so the base group is not played again.
        assert!(called(&log, 0x0049_4740).is_empty());
        // A nonzero answer of method +0x214 stops everything.
        answer(&mut e, ACTOR2_VT, 0x214, 1);
        e.call_log = Some(vec![]);
        e.call(0x008a_7a90, &args![actor]);
        assert!(called(&calls(&mut e), 0x0049_6080).is_empty());
    }

    #[test]
    fn the_player_clears_its_first_person_animation_too() {
        let mut e = engine2();
        let (actor, _) = actor2(&mut e);
        let anim = object2(&mut e, 0x80);
        let first_person = object2(&mut e, 0x80);
        animation_doubles(&mut e, anim, 0x11);
        e.set_global(PLAYER_CHARACTER, actor.addr());
        e.register_double(0x0095_0a60, move |_, a| {
            assert_eq!(a[1], 1);
            first_person.addr().into_ret()
        });
        e.mem.set_u16(anim.addr() + 0x40, 0x11);
        e.mem.set_u16(first_person.addr() + 0x40, 0x12);
        e.call_log = Some(vec![]);
        e.call(0x008a_7a90, &args![actor]);
        let log = calls(&mut e);
        let on_first_person: Vec<u32> = called(&log, 0x0049_6080)
            .iter()
            .filter(|c| c[0] == first_person.addr())
            .map(|c| c[1])
            .collect();
        // Groups 0 (differs from the actor's), 1, 2, 3 and 7.
        assert_eq!(on_first_person, vec![0, 1, 2, 3, 7]);
        // Its base group is played again as well.
        assert_eq!(called(&log, 0x0049_4740).len(), 2);
    }

    #[test]
    fn trigger_pain_restarts_the_timer_and_shakes_the_player() {
        let mut e = engine2();
        let (actor, proc) = actor2(&mut e);
        e.set_global(PLAYER_CHARACTER, actor.addr());
        e.register(0x0060_8d80, |_, _| 0.into_ret());
        e.register(0x0045_cd60, |_, _| 0.into_ret());
        e.register(0x008c_7aa0, |_, _| 0.into_ret());
        // Settings: each `GameSetting` is a pointer to its float at +4.
        let settings = [
            (0x011d_0694u32, 1.5f32),
            (0x011c_f834, 0.25),
            (0x011d_0508, 2.0),
            (0x011d_0f48, 0.75),
        ];
        for (setting, value) in settings {
            e.map(setting & !0xfff, 0x1000);
            e.mem.set_f32(setting + 4, value);
        }
        e.register(SETTING_VALUE_POINTER, |_, a| (a[0] + 4).into_ret());
        e.set_global(0x0101_7b70, 100.0f64);
        e.register(FTOL, |_, a| {
            let value = f64::from_bits(a[0] as u64 | (a[1] as u64) << 32);
            (value as i32).into_ret()
        });
        e.register(WORD_AT_8, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.set_global(0x011f_2250 + 8, 0u32);
        e.register(0x005d_2860, |_, _| 0x4444.into_ret());
        e.register(0x0052_99a0, |_, _| Ret::default());
        e.register(0x0070_2360, |_, _| 0.into_ret());
        e.set_global(0x011d_ea0c, 0x6666u32);
        e.register(0x0087_7720, |_, a| (a[0] + 1).into_ret());
        e.register(0x00a2_55b0, |_, _| Ret::default());
        e.set_global(0x011f_1708, 0x7777u32);
        e.register(0x0098_39b0, |_, _| 1.into_ret());
        // The actor's own timer field (+0x114) is 0.5, the process's timer is 0.
        e.mem.set_f32(actor.addr() + 0x114, 0.5);
        e.mem.set_f32(proc.addr() + 0x344, 0.0);
        e.call_log = Some(vec![]);
        e.call(0x008a_7d50, &args![actor, 1u8, 1u8]);
        let log = calls(&mut e);
        // Timer = field + delay.
        assert_eq!(e.mem.f32(proc.addr() + 0x344), 2.0);
        assert_eq!(
            called(&log, 0x0052_99a0),
            vec![vec![0x4444, 0.25f32.to_bits(), 0]]
        );
        assert_eq!(
            called(&log, 0x00a2_55b0),
            vec![vec![
                0x6667,
                0.75f32.to_bits(),
                0.75f32.to_bits(),
                200,
                0,
                0,
                0,
                0
            ]]
        );
        assert_eq!(
            called(&log, 0x0098_39b0),
            vec![vec![0x7777, actor.addr(), 0, 2, 2, 0, 0]]
        );
    }

    #[test]
    fn trigger_pain_waits_until_the_delay_has_passed() {
        let mut e = engine2();
        let (actor, proc) = actor2(&mut e);
        e.set_global(PLAYER_CHARACTER, 0u32);
        e.register(0x0060_8d80, |_, _| 0.into_ret());
        e.register(0x0045_cd60, |_, _| 0.into_ret());
        e.register(0x008c_7aa0, |_, _| 0.into_ret());
        e.map(0x011d_0000, 0x1000);
        e.mem.set_f32(0x011d_0698, 1.0);
        e.register(SETTING_VALUE_POINTER, |_, a| (a[0] + 4).into_ret());
        // The actor's field is below the process's timer and within the
        // delay of it: nothing changes.
        e.mem.set_f32(actor.addr() + 0x114, 4.5);
        e.mem.set_f32(proc.addr() + 0x344, 5.0);
        e.call(0x008a_7d50, &args![actor, 0u8, 0u8]);
        assert_eq!(e.mem.f32(proc.addr() + 0x344), 5.0);
        // The field is more than the delay below the timer: restarted.
        e.mem.set_f32(actor.addr() + 0x114, 3.0);
        e.call(0x008a_7d50, &args![actor, 0u8, 0u8]);
        assert_eq!(e.mem.f32(proc.addr() + 0x344), 4.0);
        // The field is at or above the timer: restarted too.
        e.mem.set_f32(actor.addr() + 0x114, 9.0);
        e.call(0x008a_7d50, &args![actor, 0u8, 0u8]);
        assert_eq!(e.mem.f32(proc.addr() + 0x344), 10.0);
    }

    #[test]
    fn trigger_pain_is_queued_or_refused() {
        let mut e = engine2();
        let (actor, proc) = actor2(&mut e);
        e.register(0x0060_8d80, |_, _| 0.into_ret());
        e.register(0x0045_cd60, |_, _| 0.into_ret());
        e.register(0x008c_7aa0, |_, _| 1.into_ret());
        e.register(0x0045_37b0, |_, _| 0x5555.into_ret());
        e.register(0x0087_b7c0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x008a_7d50, &args![actor, 1u8, 0u8]);
        let log = calls(&mut e);
        assert_eq!(
            called(&log, 0x0087_b7c0),
            vec![vec![0x5555, actor.addr(), 1, 0]]
        );
        // Method +0x22c(0) true: nothing at all.
        answer(&mut e, ACTOR2_VT, 0x22c, 1);
        e.call_log = Some(vec![]);
        e.call(0x008a_7d50, &args![actor, 1u8, 0u8]);
        assert!(called(&calls(&mut e), 0x0087_b7c0).is_empty());
        answer(&mut e, ACTOR2_VT, 0x22c, 0);
        // The process's field +0x28 nonzero: nothing.
        e.register(0x0045_cd60, |_, _| 1.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x008a_7d50, &args![actor, 1u8, 0u8]);
        assert!(called(&calls(&mut e), 0x0087_b7c0).is_empty());
        // No process: nothing.
        e.mem.set_u32(actor.addr() + 0x68, 0);
        let _ = proc;
        e.call(0x008a_7d50, &args![actor, 1u8, 0u8]);
    }

    #[test]
    fn pain_timer_accessors() {
        let mut e = engine2();
        let object = Ptr::<()>::new(e.mem.alloc(0x400));
        e.call(0x008a_7f40, &args![object, 3.5f32]);
        assert_eq!(e.mem.f32(object.addr() + 0x344), 3.5);
        assert_eq!(e.call(0x008a_7f60, &args![object]).f32(), 3.5);
    }

    #[test]
    fn location_comes_from_extra_data_then_package_then_base() {
        let mut e = engine2();
        let (actor, _) = actor2(&mut e);
        e.register(GET_EXTRA_LIST, |_, _| 0x1000.into_ret());
        e.register(0x0042_ea20, |_, _| 0x31.into_ret());
        assert_eq!(e.call(0x008a_7f80, &args![actor]).u32(), 0x31);
        // Nothing in the extra data: the second location of the package.
        e.register(0x0042_ea20, |_, _| 0.into_ret());
        e.register(0x0067_33e0, |_, a| (a[0] + 1).into_ret());
        answer(&mut e, PROCESS2_VT, 0x22c, 0x200);
        assert_eq!(e.call(0x008a_7f80, &args![actor]).u32(), 0x201);
        // No package: method +0x188 of the base object.
        answer(&mut e, PROCESS2_VT, 0x22c, 0);
        let base = object2(&mut e, 0x40);
        answer(&mut e, OBJECT2_VT, 0x188, 0x99);
        e.register_double(0x0041_81e0, move |_, _| base.addr().into_ret());
        assert_eq!(e.call(0x008a_7f80, &args![actor]).u32(), 0x99);
        // The package gives nothing either.
        answer(&mut e, PROCESS2_VT, 0x22c, 0x200);
        e.register(0x0067_33e0, |_, _| 0.into_ret());
        assert_eq!(e.call(0x008a_7f80, &args![actor]).u32(), 0x99);
    }

    #[test]
    fn extra_list_and_combat_controller_are_updated() {
        let mut e = engine2();
        let (actor, _) = actor2(&mut e);
        e.register(GET_EXTRA_LIST, |_, _| 0x1000.into_ret());
        e.register(0x0042_ea50, |_, _| Ret::default());
        e.register(0x0097_f3f0, |_, _| Ret::default());
        let controller = object2(&mut e, 0x40);
        answer(&mut e, ACTOR2_VT, 0x428, controller.addr());
        e.call_log = Some(vec![]);
        e.call(0x008a_8010, &args![actor, 7u32]);
        let log = calls(&mut e);
        assert_eq!(called(&log, 0x0042_ea50), vec![vec![0x1000, 7]]);
        assert_eq!(called(&log, 0x0097_f3f0), vec![vec![controller.addr()]]);
        answer(&mut e, ACTOR2_VT, 0x428, 0);
        e.call_log = Some(vec![]);
        e.call(0x008a_8010, &args![actor, 7u32]);
        assert!(called(&calls(&mut e), 0x0097_f3f0).is_empty());
    }

    #[test]
    fn a_new_package_is_made_below_field_two() {
        let mut e = engine2();
        let (actor, _) = actor2(&mut e);
        e.register(0x0045_cd60, |_, _| 1.into_ret());
        e.register(0x0067_0b90, |_, a| {
            assert_eq!(a[0], 0x1b);
            0x2468.into_ret()
        });
        e.register(0x0082_6b90, |_, _| Ret::default());
        e.register(0x0067_77b0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x008a_8060, &args![actor]);
        let log = calls(&mut e);
        assert_eq!(called(&log, 0x0082_6b90), vec![vec![0x2468, 1]]);
        assert_eq!(called(&log, 0x0067_77b0), vec![vec![0x2468, 0]]);
        assert_eq!(
            called(&log, target(ACTOR2_VT, 0x2f4)),
            vec![vec![actor.addr(), 0x2468, 1, 1]]
        );
        // Field 2 or more: nothing is made.
        e.register(0x0045_cd60, |_, _| 2.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x008a_8060, &args![actor]);
        assert!(called(&calls(&mut e), 0x0067_0b90).is_empty());
    }

    #[test]
    fn has_been_eaten_adds_the_extra_when_missing() {
        let mut e = engine2();
        let (actor, _) = actor2(&mut e);
        e.register(GET_EXTRA_LIST, |_, _| 0x1000.into_ret());
        e.register(0x0042_e820, |_, _| Ret::default());
        e.register(0x008a_8150, |_, _| Ret::default());
        let extra = std::rc::Rc::new(std::cell::Cell::new(0u32));
        let shared = extra.clone();
        e.register_double(0x0041_0220, move |_, a| {
            assert_eq!(a[1], 0x5f);
            shared.get().into_ret()
        });
        // The actor must be able to take extra data (method +0x22c(0)).
        answer(&mut e, ACTOR2_VT, 0x22c, 1);
        let added = extra.clone();
        e.register_double(0x0042_e820, move |_, _| {
            added.set(0x3000);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x008a_80c0, &args![actor, 1u8]);
        let log = calls(&mut e);
        assert_eq!(called(&log, 0x0042_e820).len(), 1);
        assert_eq!(
            called(&log, target(ACTOR2_VT, 0x48)),
            vec![vec![actor.addr(), 0x20000]]
        );
        assert_eq!(called(&log, 0x008a_8150), vec![vec![0x3000, 1]]);
        // Already there: not added again.
        e.call_log = Some(vec![]);
        e.call(0x008a_80c0, &args![actor, 0u8]);
        let log = calls(&mut e);
        assert!(called(&log, 0x0042_e820).is_empty());
        assert_eq!(called(&log, 0x008a_8150), vec![vec![0x3000, 0]]);
        // Method +0x22c(0) false: nothing.
        answer(&mut e, ACTOR2_VT, 0x22c, 0);
        e.call_log = Some(vec![]);
        e.call(0x008a_80c0, &args![actor, 1u8]);
        assert!(called(&calls(&mut e), 0x008a_8150).is_empty());
    }

    #[test]
    fn close_to_ground_compares_with_the_node_height() {
        let mut e = engine2();
        let (actor, _) = actor2(&mut e);
        let anim_node = Ptr::<()>::new(e.mem.alloc(0x20));
        let vector = Ptr::<()>::new(e.mem.alloc(0x10));
        e.mem.set_f32(vector.addr() + 8, 10.0);
        e.register(0x0093_06d0, |_, _| 0x700.into_ret());
        e.register(0x0043_fcd0, |_, _| 0x800.into_ret());
        e.register_double(0x004a_ae30, move |_, a| {
            assert_eq!(a[0], 0x800);
            assert_eq!(a[1], BONE_NON_ACCUM);
            anim_node.addr().into_ret()
        });
        e.register_double(0x0045_bb80, move |_, _| vector.addr().into_ret());
        e.register(SELF, |_, a| a[0].into_ret());
        e.register(CHARACTER_GET_POSITION, |e, a| {
            e.mem.set_f32(a[1] + 8, 4.0);
            Ret::default()
        });
        // Height difference 6.0: limit 5.0 is below it (false), 6.0 and 7.0
        // are not.
        assert!(!e.call(0x008a_8300, &args![actor, 5.0f32]).bool());
        assert!(e.call(0x008a_8300, &args![actor, 6.0f32]).bool());
        assert!(e.call(0x008a_8300, &args![actor, 7.0f32]).bool());
        // No node, no model, no controller: true.
        e.register(0x004a_ae30, |_, _| 0.into_ret());
        assert!(e.call(0x008a_8300, &args![actor, 0.0f32]).bool());
        e.register(0x0043_fcd0, |_, _| 0.into_ret());
        assert!(e.call(0x008a_8300, &args![actor, 0.0f32]).bool());
        e.register(0x0093_06d0, |_, _| 0.into_ret());
        assert!(e.call(0x008a_8300, &args![actor, 0.0f32]).bool());
    }

    #[test]
    fn move_forwarder_answers_method_0x3e8() {
        let mut e = engine2();
        let (actor, _) = actor2(&mut e);
        e.register(0x0049_38c0, |_, _| 0.into_ret());
        answer(&mut e, ACTOR2_VT, 0x3e8, 1);
        assert!(e
            .call(0x008a_83c0, &args![actor, 0x11u32, 0x22u32, 1u8, 0x33u32])
            .bool());
        answer(&mut e, ACTOR2_VT, 0x3e8, 0);
        assert!(!e
            .call(0x008a_83c0, &args![actor, 0x11u32, 0x22u32, 1u8, 0x33u32])
            .bool());
    }

    #[test]
    fn move_forwarder_calls_method_0x3e8_after_the_first_word_test() {
        let mut e = engine2();
        let (actor, _) = actor2(&mut e);
        e.register(0x0049_38c0, |_, _| 1.into_ret());
        answer(&mut e, ACTOR2_VT, 0x3e8, 0);
        e.call_log = Some(vec![]);
        e.call(0x008a_83c0, &args![actor, 0x11u32, 0x22u32, 1u8, 0x33u32]);
        let log = calls(&mut e);
        assert_eq!(called(&log, 0x0049_38c0), vec![vec![0x11]]);
        assert_eq!(
            called(&log, target(ACTOR2_VT, 0x3e8)),
            vec![vec![actor.addr(), 0x11, 0x22, 1]]
        );
    }

    /// Everything `fn_008a8420` and `fn_008a89a0` reach: a weapon, an
    /// ammunition form, an inventory item holding `item_count` shots, an item
    /// change in the process holding `change_count`, and doubles for the
    /// callees.
    struct Reload {
        e: Engine,
        actor: Ptr<Actor>,
        proc: Ptr,
        weapon: Ptr,
        ammo: Ptr,
        item: Ptr,
        change: Ptr,
        held: Ptr,
        act: Ptr,
    }

    fn reload_world(item_count: u32, change_count: u32) -> Reload {
        let mut e = engine2();
        let (actor, proc) = actor2(&mut e);
        let weapon = object2(&mut e, 0x200);
        let ammo = object2(&mut e, 0x200);
        let item = Ptr::<()>::new(e.mem.alloc(0x40));
        e.mem.set_u32(item.addr() + 4, item_count);
        let change = Ptr::<()>::new(e.mem.alloc(0x40));
        e.mem.set_u32(change.addr() + 4, change_count);
        let held = Ptr::<()>::new(e.mem.alloc(0x40));
        e.mem.set_u32(held.addr() + 8, weapon.addr());
        let act = object2(&mut e, 0x200);
        e.set_global(PLAYER_CHARACTER, 0u32);
        let (w, a) = (weapon.addr(), ammo.addr());
        e.register_double(0x0052_5980, move |_, args| {
            assert_eq!(args[0], w);
            a.into_ret()
        });
        e.register(0x0044_6390, |_, _| 5.into_ret());
        answer(&mut e, ACTOR2_VT, 0x1e4, act.addr());
        e.register(GET_INVENTORY_CHANGES, |_, _| 0x9000.into_ret());
        let i = item.addr();
        e.register_double(GET_INVENTORY_ITEM, move |_, args| {
            assert_eq!(args[0], 0x9000);
            i.into_ret()
        });
        e.register(ITEM_COUNT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(WORD_AT_8, |e, a| e.mem.u32(a[0] + 8).into_ret());
        answer(&mut e, PROCESS2_VT, 0x14c, change.addr());
        answer(&mut e, PROCESS2_VT, 0x148, held.addr());
        e.register(ITEM_CHANGE_HAS_MOD, |_, _| 0.into_ret());
        e.register(GET_FORM_CLIP_ROUNDS, |_, _| 10.into_ret());
        e.register(0x0051_e2a0, |_, _| 0x40.into_ret());
        e.register(GET_ANIMATION, |_, _| 0.into_ret());
        e.register(ACTOR_GET_ANIM_GROUP, |_, a| a[1].into_ret());
        e.register(ANIM_GROUP_GET_TYPE, |_, a| a[0].into_ret());
        e.register(ANIMATION_PLAY_GROUP, |_, _| Ret::default());
        e.register(ANIMATION_SLOT_VALUE, |_, a| (0x1000 + a[1]).into_ret());
        e.register(ANIMATION_BLEND_OUT, |_, _| Ret::default());
        e.register(0x0097_f7a0, |_, _| Ret::default());
        e.register(0x0047_bcf0, |_, _| 0.into_ret());
        e.register(0x0056_6950, |_, _| 0.into_ret());
        e.register(0x004c_0bf0, |_, _| 0.into_ret());
        e.register(0x006e_cd40, |_, _| Ret::default());
        e.register(0x0049_38c0, |_, _| 0.into_ret());
        e.register(0x0052_4b40, |_, _| 0.into_ret());
        e.register(0x0044_59e0, |_, _| Ret::default());
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(ITEM_CHANGE_CONSTRUCTOR, |_, a| a[0].into_ret());
        Reload {
            e,
            actor,
            proc,
            weapon,
            ammo,
            item,
            change,
            held,
            act,
        }
    }

    #[test]
    fn reload_without_a_weapon_is_done_and_without_ammo_fails() {
        let mut w = reload_world(5, 3);
        assert!(w
            .e
            .call(0x008a_8420, &args![w.actor, 0u32, 1u32, 0u8])
            .bool());
        w.e.register(0x0052_5980, |_, _| 0.into_ret());
        assert!(!w
            .e
            .call(0x008a_8420, &args![w.actor, w.weapon, 1u32, 0u8])
            .bool());
        // A weapon form of type 10, 11 or 13 is its own ammunition.
        for kind in [10, 11, 13] {
            let mut w = reload_world(5, 3);
            w.e.register(0x0052_5980, |_, _| 0.into_ret());
            w.e.set_global(0x011d_f000, kind as u32);
            w.e.register(0x0044_6390, |e, _| e.mem.u32(0x011d_f000).into_ret());
            w.e.register_double(GET_INVENTORY_ITEM, {
                let weapon = w.weapon.addr();
                move |_, a| {
                    assert_eq!(a[1], weapon);
                    0.into_ret()
                }
            });
            // No inventory item: false after the inventory lookup.
            assert!(!w
                .e
                .call(0x008a_8420, &args![w.actor, w.weapon, 1u32, 0u8])
                .bool());
        }
    }

    #[test]
    fn reload_mode_two_plays_the_group_and_sets_the_clip() {
        let mut w = reload_world(5, 3);
        w.e.call_log = Some(vec![]);
        let result = w.e.call(0x008a_8420, &args![w.actor, w.weapon, 2i32, 7u8]);
        assert!(result.bool());
        let log = calls(&mut w.e);
        // The clip is not full (10 rounds, 3 in it, 5 in stock): the reload
        // group (0x40) plays and the action 9 is set with slot 4's value.
        assert_eq!(
            called(&log, ANIMATION_PLAY_GROUP),
            vec![vec![w.act.addr(), 0x40, 1, u32::MAX, u32::MAX]]
        );
        assert_eq!(
            called(&log, target(PROCESS2_VT, 0x3ec)),
            vec![vec![w.proc.addr(), 9, 0x1004]]
        );
        // The combat controller is told (method +0x428 answered 0 here).
        assert!(called(&log, 0x0097_f7a0).is_empty());
        // The clip takes the rounds (the actor does not use ammo).
        assert_eq!(called(&log, 0x006e_cd40), vec![vec![w.change.addr(), 10]]);
        assert_eq!(called(&log, 0x0044_59e0), vec![vec![w.item.addr(), 1]]);
        // The weapon was asked for its rounds with the flag byte.
        assert_eq!(
            called(&log, GET_FORM_CLIP_ROUNDS)[0],
            vec![w.weapon.addr(), 0]
        );
        let _ = w.ammo;
    }

    #[test]
    fn reload_informs_the_combat_controller_and_keeps_stock_for_the_player() {
        let mut w = reload_world(5, 3);
        let controller = object2(&mut w.e, 0x40);
        answer(&mut w.e, ACTOR2_VT, 0x428, controller.addr());
        // The player uses ammo: the clip count is the smaller of the clip
        // size and the stock; with 10 rounds and 5 in stock, 5 stays.
        w.e.set_global(PLAYER_CHARACTER, w.actor.addr());
        w.e.call_log = Some(vec![]);
        assert!(w
            .e
            .call(0x008a_8420, &args![w.actor, w.weapon, 2i32, 0u8])
            .bool());
        let log = calls(&mut w.e);
        assert_eq!(
            called(&log, 0x0097_f7a0),
            vec![vec![controller.addr(), w.weapon.addr(), 1]]
        );
        // `rounds (10) < stock (5)` is false: the clip keeps the stock.
        assert_eq!(called(&log, 0x006e_cd40), vec![vec![w.change.addr(), 5]]);
        // With a stock above the clip size the clip size is stored.
        let mut w = reload_world(15, 3);
        w.e.set_global(PLAYER_CHARACTER, w.actor.addr());
        w.e.call_log = Some(vec![]);
        w.e.call(0x008a_8420, &args![w.actor, w.weapon, 0i32, 0u8]);
        let log = calls(&mut w.e);
        assert_eq!(called(&log, 0x006e_cd40), vec![vec![w.change.addr(), 10]]);
        // Mode 0 does not inform the controller.
        assert!(called(&log, 0x0097_f7a0).is_empty());
    }

    #[test]
    fn reload_mode_one_starts_the_ready_group_or_stores_it() {
        let mut w = reload_world(5, 3);
        w.e.call_log = Some(vec![]);
        w.e.call(0x008a_8420, &args![w.actor, w.weapon, 1i32, 0u8]);
        let log = calls(&mut w.e);
        // `00524b40` false: the group word is stored in the animation.
        assert!(called(&log, ANIMATION_PLAY_GROUP).is_empty());
        assert_eq!(w.e.mem.u16(w.act.addr() + 0x122), 0x40);
        // `00524b40` true: both blends are cleared and the group plays.
        let mut w = reload_world(5, 3);
        w.e.register(0x0052_4b40, |_, _| 1.into_ret());
        w.e.register(0x0049_38c0, |_, _| 1.into_ret());
        w.e.call_log = Some(vec![]);
        w.e.call(0x008a_8420, &args![w.actor, w.weapon, 1i32, 0u8]);
        let log = calls(&mut w.e);
        assert_eq!(
            called(&log, ANIMATION_BLEND_OUT),
            vec![vec![w.act.addr(), 5, 0], vec![w.act.addr(), 6, 0]]
        );
        assert_eq!(called(&log, ANIMATION_PLAY_GROUP).len(), 1);
        assert_eq!(
            called(&log, target(PROCESS2_VT, 0x3ec)),
            vec![vec![w.proc.addr(), 9, 0x1004]]
        );
        // The group id is lowered by 0x17 when 004938c0 holds.
        assert_eq!(called(&log, ACTOR_GET_ANIM_GROUP)[0][1], 0x40 - 0x17);
    }

    #[test]
    fn reload_makes_an_item_change_when_the_process_has_none() {
        let mut w = reload_world(5, 3);
        answer(&mut w.e, PROCESS2_VT, 0x14c, 0);
        w.e.register(OPERATOR_NEW, |e, a| {
            assert_eq!(a[0], 0xc);
            e.mem.alloc(0x40).into_ret()
        });
        w.e.register(ITEM_CHANGE_CONSTRUCTOR, |e, a| {
            // The constructor gets the block, the item's word at +8 and the
            // count; the result is what the process is given.
            e.mem.set_u32(a[0] + 4, a[2]);
            a[0].into_ret()
        });
        w.e.call_log = Some(vec![]);
        assert!(w
            .e
            .call(0x008a_8420, &args![w.actor, w.weapon, 0i32, 0u8])
            .bool());
        let log = calls(&mut w.e);
        let stored = called(&log, target(PROCESS2_VT, 0x168));
        assert_eq!(stored.len(), 1);
        // The process still answers 0 for +0x14c, so nothing is counted.
        assert_eq!(called(&log, ITEM_CHANGE_CONSTRUCTOR)[0][2], 5);
        let _ = w.held;
    }

    #[test]
    fn small_process_accessors() {
        let mut e = engine2();
        let (actor, proc) = actor2(&mut e);
        let object = Ptr::<()>::new(e.mem.alloc(0x200));
        e.call(0x008a_8820, &args![object, 0x1234u16]);
        assert_eq!(e.mem.u16(object.addr() + 0x122), 0x1234);
        e.call_log = Some(vec![]);
        e.call(0x008a_8840, &args![actor]);
        assert_eq!(
            called(&calls(&mut e), target(PROCESS2_VT, 0x614)),
            vec![vec![proc.addr(), 2]]
        );
        let bare: Ptr<Actor> = e.new_object();
        e.call(0x008a_8840, &args![bare]);
        // The flag word at +0x12c.
        assert!(!e.call(0x008a_8e30, &args![object]).bool());
        e.mem.set_u32(object.addr() + 0x12c, 2);
        assert!(e.call(0x008a_8e30, &args![object]).bool());
        e.mem.set_u32(object.addr() + 0x12c, 5);
        assert!(!e.call(0x008a_8e30, &args![object]).bool());
    }

    #[test]
    fn anim_action_ranges() {
        let mut e = engine2();
        let (actor, _) = actor2(&mut e);
        for (action, expected) in [
            (8i32, false),
            (9, true),
            (10, false),
            (14, false),
            (15, true),
            (17, true),
            (18, false),
            (-1, false),
        ] {
            answer(&mut e, PROCESS2_VT, 0x3e4, action as u32);
            assert_eq!(
                e.call(0x008a_8870, &args![actor]).bool(),
                expected,
                "{action}"
            );
        }
    }

    #[test]
    fn ammo_count_and_clip_percent() {
        let mut e = engine2();
        let (actor, _) = actor2(&mut e);
        let change = Ptr::<()>::new(e.mem.alloc(0x40));
        let held = Ptr::<()>::new(e.mem.alloc(0x40));
        e.mem.set_u32(change.addr() + 4, 3);
        e.mem.set_u32(held.addr() + 8, 0x7000);
        e.register(ITEM_COUNT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(WORD_AT_8, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(ITEM_CHANGE_HAS_MOD, |_, _| 1.into_ret());
        e.register(GET_FORM_CLIP_ROUNDS, |_, a| {
            assert_eq!(a[0], 0x7000);
            assert_eq!(a[1], 1);
            12.into_ret()
        });
        // No item change: -1 and 1.0.
        assert_eq!(e.call(0x008a_88b0, &args![actor]).u32(), u32::MAX);
        assert_eq!(e.call(0x008a_88f0, &args![actor]).f32(), 1.0);
        answer(&mut e, PROCESS2_VT, 0x14c, change.addr());
        assert_eq!(e.call(0x008a_88b0, &args![actor]).u32(), 3);
        // An item change but no weapon: 1.0.
        assert_eq!(e.call(0x008a_88f0, &args![actor]).f32(), 1.0);
        answer(&mut e, PROCESS2_VT, 0x148, held.addr());
        assert_eq!(e.call(0x008a_88f0, &args![actor]).f32(), 0.25);
        // A clip size of zero gives 1.0.
        e.register(GET_FORM_CLIP_ROUNDS, |_, _| 0.into_ret());
        assert_eq!(e.call(0x008a_88f0, &args![actor]).f32(), 1.0);
    }

    #[test]
    fn using_ammo_for_the_player_and_others() {
        let mut e = engine2();
        let (actor, _) = actor2(&mut e);
        let weapon = Ptr::<()>::new(e.mem.alloc(0x200));
        e.register(0x0047_bcf0, |_, _| 1.into_ret());
        e.register(0x0056_6950, |_, _| 1.into_ret());
        e.set_global(PLAYER_CHARACTER, actor.addr());
        assert!(e.call(0x008a_8dd0, &args![actor, 0u32]).bool());
        e.set_global(PLAYER_CHARACTER, 0u32);
        // No weapon: no.
        assert!(!e.call(0x008a_8dd0, &args![actor, 0u32]).bool());
        // The weapon's flag 2: yes.
        e.mem.set_u32(weapon.addr() + 0x12c, 2);
        assert!(e.call(0x008a_8dd0, &args![actor, weapon]).bool());
        // Without the flag: depends on 0047bcf0 and the force-next-update test.
        e.mem.set_u32(weapon.addr() + 0x12c, 0);
        assert!(e.call(0x008a_8dd0, &args![actor, weapon]).bool());
        e.register(0x0056_6950, |_, _| 0.into_ret());
        assert!(!e.call(0x008a_8dd0, &args![actor, weapon]).bool());
        e.register(0x0047_bcf0, |_, _| 0.into_ret());
        e.register(0x0056_6950, |_, _| panic!("not asked"));
        assert!(!e.call(0x008a_8dd0, &args![actor, weapon]).bool());
    }

    #[test]
    fn stop_attack_clears_the_attack_groups() {
        let mut e = engine2();
        let (actor, proc) = actor2(&mut e);
        e.set_global(PLAYER_CHARACTER, 0u32);
        e.register(ANIMATION_CLEAR_GROUP, |_, _| Ret::default());
        let anim = object2(&mut e, 0x40);
        answer(&mut e, PROCESS2_VT, 0x1b8, anim.addr());
        // State 3 does nothing.
        answer(&mut e, PROCESS2_VT, 0x3e4, 3);
        e.call_log = Some(vec![]);
        e.call(0x008a_8e50, &args![actor]);
        assert!(called(&calls(&mut e), ANIMATION_CLEAR_GROUP).is_empty());
        // State 2: groups 4 and 2 of the process's animation, then the process
        // is told.
        answer(&mut e, PROCESS2_VT, 0x3e4, 2);
        e.call_log = Some(vec![]);
        e.call(0x008a_8e50, &args![actor]);
        let log = calls(&mut e);
        assert_eq!(
            called(&log, ANIMATION_CLEAR_GROUP),
            vec![vec![anim.addr(), 4, 0], vec![anim.addr(), 2, 0]]
        );
        assert_eq!(
            called(&log, target(PROCESS2_VT, 0x3ec)),
            vec![vec![proc.addr(), u32::MAX, 0]]
        );
        // The player: the first-person and the third-person animations.
        let first = object2(&mut e, 0x40);
        let third = object2(&mut e, 0x40);
        e.set_global(PLAYER_CHARACTER, actor.addr());
        e.register_double(PLAYER_GET_ANIMATION, move |_, a| {
            (if a[1] == 1 {
                first.addr()
            } else {
                third.addr()
            })
            .into_ret()
        });
        answer(&mut e, PROCESS2_VT, 0x3e4, 4);
        e.call_log = Some(vec![]);
        e.call(0x008a_8e50, &args![actor]);
        let order: Vec<(u32, u32)> = called(&calls(&mut e), ANIMATION_CLEAR_GROUP)
            .iter()
            .map(|c| (c[0], c[1]))
            .collect();
        assert_eq!(
            order,
            vec![
                (first.addr(), 4),
                (third.addr(), 4),
                (first.addr(), 2),
                (third.addr(), 2)
            ]
        );
        // No process: nothing.
        let bare: Ptr<Actor> = e.new_object();
        e.call(0x008a_8e50, &args![bare]);
    }

    #[test]
    fn eaters_in_the_extra_list_are_told_to_stop() {
        let mut e = engine2();
        let (actor, _) = actor2(&mut e);
        form_type_from_byte(&mut e);
        e.register(GET_EXTRA_LIST, |_, _| 0x1000.into_ret());
        // The extra data: a record whose word +0xc is the first list node.
        let record = Ptr::<()>::new(e.mem.alloc(0x20));
        e.register_double(0x0042_2700, move |_, _| record.addr().into_ret());
        // Two nodes: (data, next); the data point at actors with processes.
        let node2 = Ptr::<()>::new(e.mem.alloc(8));
        let node1 = Ptr::<()>::new(e.mem.alloc(8));
        let (member1, proc1) = actor2(&mut e);
        let (member2, _) = actor2(&mut e);
        let package = object2(&mut e, 0x40);
        e.mem.set_u8(package.addr() + 0x20, 7);
        e.mem.set_u32(record.addr() + 0xc, node1.addr());
        e.mem.set_u32(node1.addr(), member1.addr());
        e.mem.set_u32(node1.addr() + 4, node2.addr());
        e.mem.set_u32(node2.addr(), member2.addr());
        // Only the first member's process has a type 7 package.
        e.register_double(target(PROCESS2_VT, 0x27c), {
            let first = proc1.addr();
            let package = package.addr();
            move |_, a| (if a[0] == first { package } else { 0 }).into_ret()
        });
        e.register(SELF, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        e.call(0x008a_8f60, &args![actor]);
        let log = calls(&mut e);
        assert_eq!(
            called(&log, target(PROCESS2_VT, 0x284)),
            vec![vec![proc1.addr(), 0]]
        );
        // No extra data: nothing happens.
        e.register(0x0042_2700, |_, _| 0.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x008a_8f60, &args![actor]);
        assert!(called(&calls(&mut e), target(PROCESS2_VT, 0x284)).is_empty());
    }

    /// A float result (`ST0`).
    fn float_ret(value: f32) -> Ret {
        Ret {
            st0: value as f64,
            ..Ret::default()
        }
    }

    /// The world of `fn_008a89a0`: an ammunition item change of `count`
    /// shots, a weapon costing `per_shot` shots per use.
    fn ammo_world(count: u32, per_shot: u32) -> Reload {
        let mut w = reload_world(0, count);
        w.e.mem.set_u32(w.change.addr() + 8, w.ammo.addr());
        w.e.register_double(0x0052_4b60, move |_, _| per_shot.into_ret());
        w.e.register(0x0056_0cf0, |_, _| Ret::default());
        w.e.register(0x0070_9430, |_, _| float_ret(1.0));
        w.e.register(0x008a_16d0, |_, _| 1.into_ret());
        w.e.register(0x0088_c790, |_, _| Ret::default());
        w.e.register(0x0097_f6d0, |_, _| Ret::default());
        w.e.register(0x008d_5cb0, |_, _| Ret::default());
        w.e.register(0x0095_0b00, |_, a| (0x5000 + a[1]).into_ret());
        w.e.register(0x004a_b5b0, |_, _| Ret::default());
        // `fn_008a6840(0)` runs for real: its callees.
        w.e.register(0x0088_46e0, |_, _| 0.into_ret());
        w.e.register(0x008b_bc10, |_, _| 0.into_ret());
        w
    }

    #[test]
    fn using_ammo_without_a_weapon_or_item_change_returns_zero() {
        let mut w = ammo_world(10, 1);
        answer(&mut w.e, PROCESS2_VT, 0x14c, 0);
        assert_eq!(w.e.call(0x008a_89a0, &args![w.actor, 1u32]).u32(), 0);
        answer(&mut w.e, PROCESS2_VT, 0x14c, w.change.addr());
        answer(&mut w.e, PROCESS2_VT, 0x148, 0);
        assert_eq!(w.e.call(0x008a_89a0, &args![w.actor, 1u32]).u32(), 0);
        let bare: Ptr<Actor> = w.e.new_object();
        assert_eq!(w.e.call(0x008a_89a0, &args![bare, 1u32]).u32(), 0);
    }

    #[test]
    fn using_ammo_takes_shots_times_the_cost_and_removes_the_item_for_the_player() {
        let mut w = ammo_world(10, 2);
        w.e.set_global(PLAYER_CHARACTER, w.actor.addr());
        w.e.call_log = Some(vec![]);
        assert_eq!(w.e.call(0x008a_89a0, &args![w.actor, 3u32]).u32(), 4);
        let log = calls(&mut w.e);
        assert_eq!(called(&log, 0x006e_cd40), vec![vec![w.change.addr(), 4]]);
        assert_eq!(
            called(&log, target(ACTOR2_VT, 0x17c)),
            vec![vec![
                w.actor.addr(),
                w.ammo.addr(),
                0,
                6,
                0,
                0,
                0,
                0,
                0,
                1,
                0
            ]]
        );
        // Shots are left: nothing else happens.
        assert!(called(&log, 0x0088_c790).is_empty());
        // Another actor does not use ammo (the weapon's flag word is 0).
        let mut w = ammo_world(10, 2);
        w.e.call_log = Some(vec![]);
        assert_eq!(w.e.call(0x008a_89a0, &args![w.actor, 3u32]).u32(), 4);
        assert!(called(&calls(&mut w.e), target(ACTOR2_VT, 0x17c)).is_empty());
    }

    #[test]
    fn using_ammo_with_minus_one_asks_the_process_and_is_capped_by_the_count() {
        let mut w = ammo_world(5, 1);
        answer(&mut w.e, PROCESS2_VT, 0x440, 0x1_0002);
        // The method's low byte (2) is the number of shots.
        assert_eq!(w.e.call(0x008a_89a0, &args![w.actor, u32::MAX]).u32(), 3);
        // More shots than the count: all of it is used and the ammunition is
        // unequipped.
        let mut w = ammo_world(5, 1);
        w.e.call_log = Some(vec![]);
        assert_eq!(w.e.call(0x008a_89a0, &args![w.actor, 9u32]).u32(), 0);
        let log = calls(&mut w.e);
        assert_eq!(called(&log, 0x006e_cd40), vec![vec![w.change.addr(), 0]]);
        assert_eq!(
            called(&log, 0x0088_c790),
            vec![vec![w.actor.addr(), w.ammo.addr(), 1, 0, 0, 0, 1]]
        );
        // The bark only when 00566950 holds; the controller when it exists.
        assert!(called(&log, 0x008d_5cb0).is_empty());
        assert!(called(&log, 0x0097_f6d0).is_empty());
    }

    #[test]
    fn using_the_last_shot_clears_the_drain_and_drops_a_matching_weapon() {
        let mut w = ammo_world(2, 1);
        let controller = object2(&mut w.e, 0x40);
        answer(&mut w.e, ACTOR2_VT, 0x428, controller.addr());
        answer(&mut w.e, PROCESS2_VT, 0x6b8, 0x4321);
        w.e.register(0x0056_6950, |_, _| 1.into_ret());
        // Weapon form equals the ammunition form: the actor's biped weapon goes.
        w.e.mem.set_u32(w.held.addr() + 8, w.ammo.addr());
        let biped = object2(&mut w.e, 0x40);
        answer(&mut w.e, ACTOR2_VT, 0x1e8, biped.addr());
        w.e.call_log = Some(vec![]);
        assert_eq!(w.e.call(0x008a_89a0, &args![w.actor, 2u32]).u32(), 0);
        let log = calls(&mut w.e);
        assert_eq!(called(&log, 0x0056_0cf0), vec![vec![0x4321, 0]]);
        assert_eq!(called(&log, 0x004a_b5b0), vec![vec![biped.addr()]]);
        assert_eq!(
            called(&log, target(PROCESS2_VT, 0x160)),
            vec![vec![w.proc.addr(), 0, 0, 0]]
        );
        assert_eq!(called(&log, 0x0088_46e0).len(), 1);
        assert_eq!(called(&log, 0x0097_f6d0), vec![vec![controller.addr(), 0]]);
        assert_eq!(called(&log, 0x008d_5cb0), vec![vec![w.actor.addr(), 7]]);
        // The player removes both bipeds.
        let mut w = ammo_world(2, 1);
        w.e.set_global(PLAYER_CHARACTER, w.actor.addr());
        w.e.mem.set_u32(w.held.addr() + 8, w.ammo.addr());
        w.e.call_log = Some(vec![]);
        w.e.call(0x008a_89a0, &args![w.actor, 2u32]);
        let log = calls(&mut w.e);
        assert_eq!(called(&log, 0x004a_b5b0), vec![vec![0x5001], vec![0x5000]]);
    }

    #[test]
    fn a_regenerating_weapon_refills_instead_of_dropping() {
        let mut w = ammo_world(2, 1);
        w.e.register(0x0070_9430, |_, _| float_ret(0.0));
        // The actor's method +0x3ec accepts: the count of the item change is
        // read again and is the result.
        answer(&mut w.e, ACTOR2_VT, 0x3ec, 1);
        w.e.register_double(0x006e_cd40, {
            let change = w.change.addr();
            move |e, a| {
                e.mem.set_u32(change + 4, 7);
                assert_eq!(a[1], 0);
                Ret::default()
            }
        });
        w.e.call_log = Some(vec![]);
        assert_eq!(w.e.call(0x008a_89a0, &args![w.actor, 2u32]).u32(), 7);
        let log = calls(&mut w.e);
        assert_eq!(
            called(&log, target(ACTOR2_VT, 0x3ec)),
            vec![vec![w.actor.addr(), w.weapon.addr(), 1, 0, 0]]
        );
        // The weapon is not dropped, the ammunition not unequipped (the
        // count is not zero any more).
        assert!(called(&log, target(PROCESS2_VT, 0x160)).is_empty());
        assert!(called(&log, 0x0088_c790).is_empty());
        // A positive regeneration rate does not ask the actor.
        let mut w = ammo_world(2, 1);
        w.e.call_log = Some(vec![]);
        assert_eq!(w.e.call(0x008a_89a0, &args![w.actor, 2u32]).u32(), 0);
        assert!(called(&calls(&mut w.e), target(ACTOR2_VT, 0x3ec)).is_empty());
    }

    use std::cell::{Cell, RefCell};
    use std::collections::VecDeque;
    use std::rc::Rc;

    /// The save/load object and its buffer: `825c00` returns the position in
    /// a real buffer, the writers advance it by their size argument and the
    /// readers fill their target from `reads` and advance it likewise.
    struct SaveBuffer {
        owner: u32,
        base: u32,
        written: Rc<Cell<u32>>,
        reads: Rc<RefCell<VecDeque<u32>>>,
    }

    /// `(address, index of the pointer, index of the size)` of the readers.
    const READERS: [(u32, usize, usize); 5] = [
        (0x0085_79e0, 1, 2),
        (0x0048_4d00, 1, 2),
        (0x0048_4d40, 1, 2),
        (0x0085_7aa0, 1, 2),
        (0x0086_4980, 1, 2),
    ];
    /// `(address, index of the size)` of the writers.
    const WRITERS: [(u32, usize); 4] = [
        (0x0085_79b0, 2),
        (0x0048_4ce0, 2),
        (0x0048_4d20, 2),
        (0x0085_7a10, 2),
    ];

    fn save_buffer(e: &mut Engine, version: u8) -> SaveBuffer {
        let owner = e.mem.alloc(0x100);
        e.mem.set_u8(owner + 0x80, version);
        e.set_global(0x011d_e45c, owner);
        let base = e.mem.alloc(0x1000);
        let written = Rc::new(Cell::new(0u32));
        let reads: Rc<RefCell<VecDeque<u32>>> = Rc::default();
        let position = written.clone();
        e.register_double(0x0082_5c00, move |_, _| (base + position.get()).into_ret());
        e.register(0x008d_f040, |e, a| e.mem.u8(a[0] + 0x80).into_ret());
        e.register(0x0086_2110, |e, a| {
            let version = e.mem.u8(a[0] + 0x80);
            (0x1f..0x5a).contains(&version).into_ret()
        });
        for (address, size_index) in WRITERS {
            let position = written.clone();
            e.register_double(address, move |_, a| {
                position.set(position.get() + a[size_index]);
                Ret::default()
            });
        }
        for (address, pointer_index, size_index) in READERS {
            let position = written.clone();
            let queue = reads.clone();
            e.register_double(address, move |e, a| {
                let value = queue.borrow_mut().pop_front().unwrap_or(0);
                match a[size_index] {
                    1 => e.mem.set_u8(a[pointer_index], value as u8),
                    2 => e.mem.set_u16(a[pointer_index], value as u16),
                    _ => e.mem.set_u32(a[pointer_index], value),
                }
                position.set(position.get() + a[size_index]);
                Ret::default()
            });
        }
        // The debug setting byte is off.
        let flag = e.mem.alloc(4);
        e.register_double(0x0040_8d60, move |_, _| flag.into_ret());
        e.register(FORM_ID_OF, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        SaveBuffer {
            owner,
            base,
            written,
            reads,
        }
    }

    /// The address of the debug setting byte `save_buffer` installed.
    fn debug_byte(e: &mut Engine) -> u32 {
        e.call(0x0040_8d60, &args![SAVE_LOG_SETTING]).u32()
    }

    /// A list node `(data, next)`.
    fn node(e: &mut Engine, data: u32, next: u32) -> u32 {
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, data);
        e.mem.set_u32(node + 4, next);
        node
    }

    /// A form with the id `id` at `+0xc`.
    fn form_with_id(e: &mut Engine, id: u32) -> u32 {
        let form = e.mem.alloc(0x20);
        e.mem.set_u32(form + 0xc, id);
        form
    }

    #[test]
    fn save_size_adds_the_parts_of_the_format() {
        let mut e = engine2();
        let (actor, _) = actor2(&mut e);
        let save = save_buffer(&mut e, 0x10);
        e.register(0x0093_1ff0, |_, _| 10.into_ret());
        e.register(0x005a_e380, |_, _| 3.into_ret());
        e.register(0x0093_7ab0, |_, _| 9.into_ret());
        // Version 0x10, no blocks (version < 0x1f is not a block format? the
        // double says blocks need 0x1f..0x5a): 10 + 7 + 2 + 3 * 8.
        assert_eq!(e.call(0x008a_9020, &args![actor, 0u32]).u16(), 43);
        // Flags 0x400 adds one.
        assert_eq!(e.call(0x008a_9020, &args![actor, 0x400u32]).u16(), 44);
        // Blocks add 6.
        e.mem.set_u8(save.owner + 0x80, 0x20);
        assert_eq!(e.call(0x008a_9020, &args![actor, 0u32]).u16(), 49);
        // Flag 0x80000 adds 2 and 8 for every node whose item has a form:
        // the actor's list head at +0xfc holds two items, one with a form.
        let item_with_form = e.mem.alloc(8);
        e.mem.set_u32(item_with_form + 4, 0x1234);
        let item_without = e.mem.alloc(8);
        let second = node(&mut e, item_without, 0);
        e.mem.set_u32(actor.addr() + 0xfc, item_with_form);
        e.mem.set_u32(actor.addr() + 0xfc + 4, second);
        assert_eq!(
            e.call(0x008a_9020, &args![actor, 0x8_0000u32]).u16(),
            49 + 2 + 8
        );
        e.mem.set_u32(actor.addr() + 0xfc, 0);
        e.mem.set_u32(actor.addr() + 0xfc + 4, 0);
        // Version 0x50 (still blocks): 0x32 +4, 0x3c +4, 0x44 + modifiers
        // (flag 0x800000), 0x45 +5.
        e.mem.set_u8(save.owner + 0x80, 0x50);
        assert_eq!(
            e.call(0x008a_9020, &args![actor, 0u32]).u16(),
            49 + 4 + 4 + 5
        );
        assert_eq!(
            e.call(0x008a_9020, &args![actor, 0x80_0000u32]).u16(),
            49 + 4 + 4 + 9 + 5
        );
        // Version 0x7b (no blocks): everything: 43 + 4 + 4 + 5 + 4 + 4 + 10 + 1 + 1.
        e.mem.set_u8(save.owner + 0x80, 0x7b);
        assert_eq!(
            e.call(0x008a_9020, &args![actor, 0u32]).u16(),
            43 + 4 + 4 + 5 + 4 + 4 + 10 + 1 + 1
        );
    }

    #[test]
    fn save_size_logs_when_the_debug_setting_is_on() {
        let mut e = engine2();
        let (actor, _) = actor2(&mut e);
        let save = save_buffer(&mut e, 0x10);
        e.register(0x0093_1ff0, |_, _| 10.into_ret());
        e.register(0x005a_e380, |_, _| 0.into_ret());
        e.register(ERROR, |_, _| Ret::default());
        e.register(0x004f_d3e0, |_, _| 0.into_ret());
        let byte = debug_byte(&mut e);
        e.mem.set_u8(byte, 1);
        e.call_log = Some(vec![]);
        let size = e.call(0x008a_9020, &args![actor, 0u32]).u16();
        assert_eq!(size, 19);
        let log = calls(&mut e);
        // No form being saved: the short message with the size added here.
        assert_eq!(
            called(&log, ERROR),
            vec![vec![0x0101_2c78, 9, 0x4c64, ACTOR_SOURCE_FILE]]
        );
        // With a form: id, name and flags are added.
        let form = e.mem.alloc(0x20);
        e.mem.set_u32(form, 0xabcd);
        e.mem.set_u32(form + 5, 0x77);
        e.register_double(0x004f_d3e0, move |_, _| form.into_ret());
        e.register(LOOKUP_FORM, |_, _| 0x1000.into_ret());
        let namer = object2(&mut e, 0x40);
        e.register_double(LOOKUP_FORM, move |_, _| namer.addr().into_ret());
        answer(&mut e, OBJECT2_VT, 0x130, 0x5151);
        e.call_log = Some(vec![]);
        e.call(0x008a_9020, &args![actor, 0u32]);
        assert_eq!(
            called(&calls(&mut e), ERROR),
            vec![vec![
                0x0101_2cb0,
                9,
                0xabcd,
                0x5151,
                0x77,
                0x4c64,
                ACTOR_SOURCE_FILE
            ]]
        );
        let _ = save;
    }

    /// An actor whose lists and form references are filled for the save and
    /// load tests: the list at `+0xfc` has one item with a form, one without;
    /// the list at `+0xf4` has one entry with a form.
    fn actor_with_lists(e: &mut Engine) -> Ptr<Actor> {
        let (actor, _) = actor2(e);
        let form_a = form_with_id(e, 0xa1);
        let item_a = e.mem.alloc(8);
        e.mem.set_u32(item_a + 4, form_a);
        let item_b = e.mem.alloc(8);
        let second = node(e, item_b, 0);
        e.mem.set_u32(actor.addr() + 0xfc, item_a);
        e.mem.set_u32(actor.addr() + 0xfc + 4, second);
        let entry_form = form_with_id(e, 0xb2);
        let entry = e.mem.alloc(8);
        e.mem.set_u32(entry, entry_form);
        e.mem.set_u32(actor.addr() + 0xf4, entry);
        let reference_a = form_with_id(e, 0xc3);
        let reference_b = form_with_id(e, 0xd4);
        e.mem.set_u32(actor.addr() + 0xc0, reference_a);
        e.mem.set_u32(actor.addr() + 0x148, reference_b);
        actor
    }

    #[test]
    fn save_writes_the_fields_lists_and_patches_the_sizes() {
        let mut e = engine2();
        let actor = actor_with_lists(&mut e);
        let save = save_buffer(&mut e, 0x50);
        e.register(0x0093_2070, |_, _| Ret::default());
        e.register(0x004f_8960, |_, _| 5.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x008a_9380, &args![actor, 0x8_0400u32]);
        let log = calls(&mut e);
        let writes: Vec<(u32, u32)> = called(&log, SAVE_GAME_DATA_OLD)
            .iter()
            .map(|c| (c[1], c[2]))
            .collect();
        let at = |offset: u32| actor.addr() + offset;
        assert_eq!(writes[0], (at(0x114), 4));
        assert_eq!(writes[1], (at(0x124), 1));
        assert_eq!(writes[2], (at(0x125), 1));
        assert_eq!(writes[3], (at(0xbc), 1));
        // The state byte (flag 0x400), then the item of the first list and
        // the data of the entry of the second.
        assert_eq!(writes[4].1, 1);
        assert_eq!(
            e.mem.u32(writes[5].0 + 4),
            e.mem.u32(e.mem.u32(at(0xfc)) + 4)
        );
        assert_eq!(writes.len(), 7);
        // The ids written: the item's form, the entry's form, the two
        // references and the zero id of version 0x45.
        let ids: Vec<u32> = called(&log, SAVE_NUMERIC_ID).iter().map(|c| c[2]).collect();
        assert_eq!(ids, vec![4; 4]);
        // Block header (tag, size) and the patched sizes.
        assert_eq!(e.mem.u16(save.base + 4), 43);
        assert_eq!(e.mem.u16(save.base + 14), 1);
        assert_eq!(e.mem.u16(save.base + 24), 1);
        assert_eq!(save.written.get(), 47);
        assert_eq!(called(&log, 0x0085_79b0)[0][2], 4);
    }

    #[test]
    fn save_calls_the_modifier_list_and_logs_for_version_0x7b() {
        let mut e = engine2();
        let actor = actor_with_lists(&mut e);
        let save = save_buffer(&mut e, 0x7b);
        e.register(0x0093_2070, |_, _| Ret::default());
        e.register(0x0093_78d0, |_, _| Ret::default());
        e.register(ERROR, |_, _| Ret::default());
        e.register(0x004f_d3e0, |_, _| 0.into_ret());
        let byte = debug_byte(&mut e);
        e.mem.set_u8(byte, 1);
        e.call_log = Some(vec![]);
        e.call(0x008a_9380, &args![actor, 0x80_0000u32]);
        let log = calls(&mut e);
        assert_eq!(called(&log, 0x0093_78d0), vec![vec![actor.addr() + 0xd0]]);
        // Version 0x7b: the late fields follow in order.
        let tail: Vec<(u32, u32)> = called(&log, SAVE_GAME_DATA_OLD)
            .iter()
            .rev()
            .take(7)
            .map(|c| (c[1] - actor.addr(), c[2]))
            .collect();
        assert_eq!(
            tail,
            vec![
                (0x118, 1),
                (0x174, 1),
                (0x74, 4),
                (0x158, 4),
                (0x14c, 1),
                (0x126, 1),
                (0xc8, 4)
            ]
        );
        // The log line is the one without a form.
        assert_eq!(called(&log, ERROR).len(), 1);
        assert_eq!(called(&log, ERROR)[0][0], 0x0101_536c);
        assert_eq!(called(&log, ERROR)[0][2], 0x4d09);
        let _ = save;
    }

    #[test]
    fn load_reads_the_fields_in_the_order_they_were_saved() {
        let mut e = engine2();
        let actor = actor_with_lists(&mut e);
        // Free the lists: the load builds them anew.
        e.mem.set_u32(actor.addr() + 0xfc, 0);
        e.mem.set_u32(actor.addr() + 0xfc + 4, 0);
        e.mem.set_u32(actor.addr() + 0xf4, 0);
        e.mem.set_u32(actor.addr() + 0xf4 + 4, 0);
        let save = save_buffer(&mut e, 0x50);
        let values = [
            0x424c_4f4bu32, // block tag
            43,             // block size
            0x3f80_0000,    // +0x114
            1,              // +0x124
            2,              // +0x125
            3,              // +0xbc
            4,              // state byte (flag 0x400)
            1,              // count of the first list
            0x91,           // id of its item's form
            0x92,           // item data
            1,              // count of the second list
            0x93,           // id of the entry's form
            0x94,           // entry data
            0x95,           // +0xc0
            0x0,            // +0x148 (no form)
            9,              // +0xc4
            0,              // numeric id of version 0x45
        ];
        save.reads.borrow_mut().extend(values);
        e.register(0x0093_20e0, |_, _| Ret::default());
        e.register(0x0055_0890, |_, _| Ret::default());
        e.register(0x004f_8960, |_, _| 0.into_ret());
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register_double(0x005a_e3d0, |_, _| Ret::default());
        e.register(ERROR, |_, _| Ret::default());
        e.register(SAVELOAD_ERROR, |_, _| panic!("no layout error expected"));
        e.register(0x004f_d3c0, |_, _| 0.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x008a_9940, &args![actor, 0x8_0400u32, 0u32]);
        let log = calls(&mut e);
        assert_eq!(e.mem.u32(actor.addr() + 0x114), 0x3f80_0000);
        assert_eq!(e.mem.u8(actor.addr() + 0x124), 1);
        assert_eq!(e.mem.u8(actor.addr() + 0x125), 2);
        assert_eq!(e.mem.u8(actor.addr() + 0xbc), 3);
        assert_eq!(e.mem.u32(actor.addr() + 0xc0), 0x95);
        assert_eq!(e.mem.u32(actor.addr() + 0x148), 0);
        assert_eq!(e.mem.u8(actor.addr() + 0xc4), 9);
        // The state read (4) differs from the current one (0), and neither 4 nor
        // the saved state is one of 1, 2, 6: state field only.
        assert_eq!(e.mem.u32(actor.addr() + 0x108), 4);
        // Two blocks were pushed onto the two lists, built from the reads.
        let pushes = called(&log, 0x005a_e3d0);
        assert_eq!(pushes.len(), 2);
        assert_eq!(pushes[0][0], actor.addr() + 0xfc);
        assert_eq!(pushes[1][0], actor.addr() + 0xf4);
        // The handler is told the loading starts and stops around the base.
        assert_eq!(called(&log, 0x0055_0890).len(), 2);
    }

    #[test]
    fn load_reports_a_bad_header_and_size_mismatches() {
        let mut e = engine2();
        let (actor, _) = actor2(&mut e);
        let save = save_buffer(&mut e, 0x20);
        e.register(0x0093_20e0, |_, _| Ret::default());
        e.register(0x0055_0890, |_, _| Ret::default());
        e.register(0x004f_8960, |_, _| 0.into_ret());
        e.register(0x004f_d3c0, |_, _| 0.into_ret());
        e.register(SAVELOAD_ERROR, |_, _| Ret::default());
        // Wrong tag; block size 100 but only a few bytes are read: underrun.
        save.reads.borrow_mut().extend([0x1111_1111u32, 100]);
        e.call_log = Some(vec![]);
        e.call(0x008a_9940, &args![actor, 0u32, 0u32]);
        let log = calls(&mut e);
        let errors = called(&log, SAVELOAD_ERROR);
        assert_eq!(errors.len(), 2);
        assert_eq!(
            errors[0],
            vec![0x0101_56a8, ACTOR_SOURCE_FILE, 0x4d48, 0x20]
        );
        // 100 expected from the position after the size word (2 bytes), 8 +
        // the 3 fields... the exact number is what was read in between.
        assert_eq!(errors[1][0], 0x0101_5440);
        assert_eq!(errors[1][3], 0x4dff);
        assert_eq!(errors[1][4], 0x20);
        // With a form being loaded, the form id, name and flags follow.
        let form = e.mem.alloc(0x20);
        e.mem.set_u32(form, 0xabcd);
        e.mem.set_u32(form + 5, 0x66);
        e.mem.set_u8(form + 9, 3);
        e.register_double(0x004f_d3c0, move |_, _| form.into_ret());
        let namer = object2(&mut e, 0x40);
        e.register_double(LOOKUP_FORM, move |_, _| namer.addr().into_ret());
        answer(&mut e, OBJECT2_VT, 0x130, 0x5151);
        save.written.set(0);
        save.reads.borrow_mut().extend([0x424c_4f4bu32, 1]);
        e.call_log = Some(vec![]);
        e.call(0x008a_9940, &args![actor, 0u32, 0u32]);
        let errors = called(&calls(&mut e), SAVELOAD_ERROR);
        // Size word 1 is below what was read (overrun).
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0][0], 0x0101_5588);
        assert_eq!(&errors[0][3..], &[0x4dff, 0xabcd, 0x5151, 3, 0x66]);
    }

    #[test]
    fn load_sets_up_the_worn_items_and_the_life_state_for_new_flags() {
        let mut e = engine2();
        let (actor, _) = actor2(&mut e);
        let save = save_buffer(&mut e, 0x7b);
        e.register(0x0093_20e0, |_, _| Ret::default());
        e.register(0x0055_0890, |_, _| Ret::default());
        e.register(0x004f_8960, |_, _| 0.into_ret());
        e.register(0x007a_f430, |_, _| 0x5000.into_ret());
        e.register(0x0040_1170, |_, _| 0x2a.into_ret());
        e.register(GET_CURRENT_PACKAGE, |_, _| 0.into_ret());
        e.register(0x0060_47c0, |_, _| Ret::default());
        e.register(0x005f_9e00, |_, _| Ret::default());
        e.register(0x0041_81e0, |_, _| 0x6000.into_ret());
        e.register(0x005f_0f50, |_, _| 0.into_ret());
        e.register(0x008a_1800, |_, _| Ret::default());
        e.register(0x008a_cc80, |_, _| Ret::default());
        e.register(0x0057_7330, |_, _| Ret::default());
        e.register(0x0093_79c0, |_, _| Ret::default());
        e.mem.set_u8(actor.addr() + 0x118, 1);
        e.call_log = Some(vec![]);
        // Flag 0x20 only in the second word: the worn items of an NPC base.
        e.call(0x008a_9940, &args![actor, 0u32, 0x20u32]);
        let log = calls(&mut e);
        assert_eq!(
            called(&log, 0x0060_47c0),
            vec![vec![0x5000, actor.addr(), 1, 1, 0, 1]]
        );
        // Also for a creature base (type 0x2b).
        e.register(0x0040_1170, |_, _| 0x2b.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x008a_9940, &args![actor, 0u32, 0x20u32]);
        assert_eq!(
            called(&calls(&mut e), 0x005f_9e00),
            vec![vec![0x5000, actor.addr(), 1, 1, 1]]
        );
        // Flag 0x400 only in the second word: a base that is not alive resets
        // the life state; one that is has its ragdoll data restored.
        e.call_log = Some(vec![]);
        e.call(0x008a_9940, &args![actor, 0u32, 0x400u32]);
        let log = calls(&mut e);
        assert_eq!(called(&log, 0x008a_1800), vec![vec![actor.addr(), 0]]);
        assert_eq!(called(&log, 0x008a_cc80).len(), 1);
        e.register(0x005f_0f50, |_, _| 1.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x008a_9940, &args![actor, 0u32, 0x400u32]);
        assert_eq!(
            called(&calls(&mut e), 0x0057_7330),
            vec![vec![actor.addr(), 0]]
        );
        let _ = save;
    }

    /// The flag-word helpers of the save buffers (`428110`, `42ce30`,
    /// `004280f0`) and the animation clean-up chain (`496940`, `537bd0`,
    /// `499b70`, `004def90`).
    fn buffer_helper_doubles(e: &mut Engine) {
        e.register(0x0042_8110, |e, a| {
            let word = e.mem.u32(a[0] + 0x17);
            e.mem.set_u32(a[1], word);
            a[1].into_ret()
        });
        e.register(0x0042_ce30, |e, a| {
            let word = e.mem.u32(a[0] + 0x2c);
            e.mem.set_u32(a[1], word);
            a[1].into_ret()
        });
        e.register(0x0042_80f0, |e, a| {
            ((e.mem.u32(a[0]) & a[1]) != 0).into_ret()
        });
    }

    /// A buffer object with the flag word of `428110` set to `flags_a` and the
    /// one of `42ce30` to `flags_b`.
    fn buffer_object(e: &mut Engine, flags_a: u32, flags_b: u32) -> u32 {
        buffer_helper_doubles(e);
        let buffer = object2(e, 0x100);
        e.mem.set_u32(buffer.addr() + 0x17, flags_a);
        e.mem.set_u32(buffer.addr() + 0x2c, flags_b);
        buffer.addr()
    }

    /// The animation of an actor whose node chain ends at `node`.
    fn animation_with_node(e: &mut Engine, node: u32) -> Ptr {
        let anim = object2(e, 0x80);
        e.register(0x0049_6940, |_, a| (a[0] + 0x10).into_ret());
        e.register(0x0053_7bd0, |_, _| 0.into_ret());
        let sink = object2(e, 0x40);
        let inner = object2(e, 0x40);
        e.register_double(0x0053_7bd0, move |_, _| sink.addr().into_ret());
        e.register(0x0049_9b70, |_, _| 3.into_ret());
        answer(e, OBJECT2_VT, 0x8c, inner.addr());
        answer(e, OBJECT2_VT, 0xc, node);
        e.register(0x004d_ef90, |_, _| Ret::default());
        anim
    }

    #[test]
    fn revert_releases_the_animation_node_and_the_list_and_resets_the_actor() {
        let mut e = engine2();
        let actor = actor_with_lists(&mut e);
        let save = save_buffer(&mut e, 0x50);
        e.register(0x0055_f5b0, |_, _| 1.into_ret());
        let anim = animation_with_node(&mut e, 0x4242);
        e.register_double(GET_ANIMATION, move |_, _| anim.addr().into_ret());
        e.set_global(PLAYER_CHARACTER, 0u32);
        let held = object2(&mut e, 0x40);
        e.mem.set_u32(actor.addr() + 0x8c, held.addr());
        e.register(0x0080_fcc0, |_, _| Ret::default());
        e.register(0x0093_2750, |_, _| Ret::default());
        // Removing the head of the list empties it.
        e.register(0x0063_f7b0, |e, a| {
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, 0);
            Ret::default()
        });
        e.register(0x0040_1030, |_, _| Ret::default());
        e.register(0x0093_7b30, |_, _| Ret::default());
        e.register(0x008a_5530, |_, _| Ret::default());
        let face = object2(&mut e, 0x40);
        e.register_double(0x008a_dcb0, move |_, _| face.addr().into_ret());
        e.register(0x008c_5090, |_, _| Ret::default());
        e.mem.set_u8(actor.addr() + 0x118, 1);
        e.mem.set_u8(actor.addr() + 0x15c, 0);
        let first_item = e.mem.u32(actor.addr() + 0xfc);
        e.call_log = Some(vec![]);
        e.call(0x008a_a210, &args![actor, 0x88_0000u32]);
        let log = calls(&mut e);
        assert_eq!(called(&log, 0x004d_ef90), vec![vec![0x4242]]);
        assert_eq!(called(&log, 0x0080_fcc0), vec![vec![held.addr(), 1]]);
        assert_eq!(e.mem.u32(actor.addr() + 0x8c), 0);
        assert_eq!(
            called(&log, 0x0093_2750),
            vec![vec![actor.addr(), 0x88_0000]]
        );
        // The first node held an item, the second also: both are freed, in order
        // (the double empties the whole list on the first removal, so the
        // loop runs once).
        assert_eq!(called(&log, 0x0040_1030), vec![vec![first_item]]);
        assert_eq!(called(&log, 0x0093_7b30), vec![vec![actor.addr() + 0xd0]]);
        assert_eq!(
            called(&log, target(OBJECT2_VT, 0xd8)),
            vec![vec![face.addr(), 0, 1]]
        );
        assert_eq!(e.mem.u8(actor.addr() + 0x118), 0);
        assert_eq!(e.mem.u8(actor.addr() + 0x15c), 1);
        let _ = save;
    }

    #[test]
    fn revert_does_nothing_more_when_the_save_object_is_not_reverting() {
        let mut e = engine2();
        let (actor, _) = actor2(&mut e);
        let _ = save_buffer(&mut e, 0x50);
        e.register(0x0055_f5b0, |_, _| 0.into_ret());
        e.register(0x0093_2750, |_, _| Ret::default());
        e.mem.set_u8(actor.addr() + 0x118, 1);
        e.call_log = Some(vec![]);
        e.call(0x008a_a210, &args![actor, 0u32]);
        let log = calls(&mut e);
        assert_eq!(log.len(), 4);
        assert_eq!(e.mem.u8(actor.addr() + 0x118), 1);
    }

    #[test]
    fn revert_for_the_player_releases_both_animations() {
        let mut e = engine2();
        let (actor, _) = actor2(&mut e);
        let _ = save_buffer(&mut e, 0x50);
        e.register(0x0055_f5b0, |_, _| 1.into_ret());
        let anim = animation_with_node(&mut e, 0x4243);
        e.register_double(0x0095_0a60, move |_, _| anim.addr().into_ret());
        e.set_global(PLAYER_CHARACTER, actor.addr());
        e.register(0x0093_2750, |_, _| Ret::default());
        e.register(0x008a_5530, |_, _| Ret::default());
        e.register(0x008a_dcb0, |_, _| 0.into_ret());
        e.register(0x008c_5090, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x008a_a210, &args![actor, 0u32]);
        let log = calls(&mut e);
        assert_eq!(
            called(&log, 0x0095_0a60),
            vec![vec![actor.addr(), 1], vec![actor.addr(), 0]]
        );
        assert_eq!(called(&log, 0x004d_ef90).len(), 2);
    }

    #[test]
    fn init_load_resolves_forms_and_drops_dead_list_entries() {
        let mut e = engine2();
        let actor = actor_with_lists(&mut e);
        let _ = save_buffer(&mut e, 0x70);
        // Entries of the first list: ids 0x11 (resolves) and 0x12 (does not).
        let item_one = e.mem.alloc(8);
        e.mem.set_u32(item_one + 4, 0x11);
        let item_two = e.mem.alloc(8);
        e.mem.set_u32(item_two + 4, 0x12);
        let second = node(&mut e, item_two, 0);
        e.mem.set_u32(actor.addr() + 0xfc, item_one);
        e.mem.set_u32(actor.addr() + 0xfc + 4, second);
        // The second list: one entry that resolves.
        let entry = e.mem.alloc(8);
        e.mem.set_u32(entry, 0x21);
        e.mem.set_u32(actor.addr() + 0xf4, entry);
        e.mem.set_u32(actor.addr() + 0xf4 + 4, 0);
        e.mem.set_u32(actor.addr() + 0xc0, 0x31);
        e.mem.set_u32(actor.addr() + 0x70, 0x41);
        e.register(0x0093_2490, |_, _| Ret::default());
        e.register(LOOKUP_FORM, |_, a| (a[0] + 0x1000).into_ret());
        // The cast keeps every form except the one for id 0x12.
        e.register(DYNAMIC_CAST, |_, a| {
            (if a[0] == 0x1012 { 0 } else { a[0] }).into_ret()
        });
        e.register_double(0x0090_5330, |e, a| {
            // Unlinks the node after `a[0]`.
            let next = e.mem.u32(a[0] + 4);
            let after = e.mem.u32(next + 4);
            e.mem.set_u32(a[0] + 4, after);
            Ret::default()
        });
        e.register(0x0040_1030, |_, _| Ret::default());
        // No process: the cell and process part is skipped; the actor gets a new
        // process since the process is missing and the kind tests fail.
        e.mem.set_u32(actor.addr() + 0x68, 0);
        e.register(0x0044_0d80, |_, _| 1.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x008a_a4c0, &args![actor, 0x8_0000u32, 0u32]);
        let log = calls(&mut e);
        assert_eq!(e.mem.u32(item_one + 4), 0x1011);
        assert_eq!(e.mem.u32(entry), 0x1021);
        assert_eq!(e.mem.u32(actor.addr() + 0xc0), 0x1031);
        assert_eq!(e.mem.u32(actor.addr() + 0x70), 0x1041);
        // The second item was unlinked after the first and freed.
        assert_eq!(called(&log, 0x0090_5330).len(), 1);
        assert_eq!(called(&log, 0x0090_5330)[0][0], actor.addr() + 0xfc);
        assert_eq!(called(&log, 0x0040_1030), vec![vec![item_two]]);
        assert_eq!(e.mem.u32(actor.addr() + 0xfc + 4), 0);
    }

    #[test]
    fn init_load_removes_an_unresolved_head_entry() {
        let mut e = engine2();
        let (actor, _) = actor2(&mut e);
        let _ = save_buffer(&mut e, 0x10);
        let item = e.mem.alloc(8);
        e.mem.set_u32(item + 4, 0x12);
        e.mem.set_u32(actor.addr() + 0xfc, item);
        e.register(0x0093_2490, |_, _| Ret::default());
        e.register(LOOKUP_FORM, |_, a| a[0].into_ret());
        e.register(DYNAMIC_CAST, |_, _| 0.into_ret());
        e.register(0x0063_f7b0, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        e.register(0x0040_1030, |_, _| Ret::default());
        e.mem.set_u32(actor.addr() + 0x68, 0);
        e.register(0x0044_0d80, |_, _| 1.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x008a_a4c0, &args![actor, 0x8_0000u32, 0u32]);
        let log = calls(&mut e);
        assert_eq!(called(&log, 0x0063_f7b0), vec![vec![actor.addr() + 0xfc]]);
        assert_eq!(called(&log, 0x0040_1030), vec![vec![item]]);
    }

    #[test]
    fn init_load_puts_the_actor_back_into_its_cell() {
        let mut e = engine2();
        let (actor, _) = actor2(&mut e);
        let _ = save_buffer(&mut e, 0x10);
        e.register(0x0093_2490, |_, _| Ret::default());
        e.register(0x0045_cd60, |_, _| 1.into_ret());
        // With a parent cell that passes 00450ff0: method +0x240.
        e.register(GET_PARENT_CELL, |_, _| 0x7100.into_ret());
        e.register(0x0045_0ff0, |_, _| 1.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x008a_a4c0, &args![actor, 0u32, 0u32]);
        let log = calls(&mut e);
        assert_eq!(
            called(&log, target(ACTOR2_VT, 0x240)),
            vec![vec![actor.addr()]]
        );
        // Without a cell, a persistent reference looks its cell up through its
        // position, adds itself to it with the "keep" flag toggled.
        e.register(GET_PARENT_CELL, |_, _| 0.into_ret());
        e.register(0x0056_53d0, |_, _| 1.into_ret());
        e.register(0x0057_5d70, |_, _| 0x7200.into_ret());
        let position = e.mem.alloc(0x10);
        e.mem.set_f32(position, 8192.0);
        e.mem.set_f32(position + 4, -4096.0);
        answer(&mut e, ACTOR2_VT, 0x1f4, position);
        e.register(0x0040_6d90, |_, a| (f32::from_bits(a[0]) as i32).into_ret());
        e.register(0x0046_1c20, |_, _| 0x7300.into_ret());
        e.register(0x0047_c850, |_, _| 1.into_ret());
        e.register(0x0045_34f0, |_, _| Ret::default());
        e.register(0x0054_8230, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x008a_a4c0, &args![actor, 0u32, 0u32]);
        let log = calls(&mut e);
        // Cell grid coordinates: 8192 >> 12 = 2, -4096 >> 12 = -1.
        let handler: u32 = e.global(DATA_HANDLER);
        assert_eq!(
            called(&log, 0x0046_1c20),
            vec![vec![handler, 2, u32::MAX, 0x7200, 0]]
        );
        assert_eq!(
            called(&log, 0x0054_8230),
            vec![vec![0x7300, actor.addr(), 0]]
        );
        let toggles: Vec<u32> = called(&log, 0x0045_34f0).iter().map(|c| c[1]).collect();
        assert_eq!(toggles, vec![0, 1]);
    }

    #[test]
    fn init_load_creates_a_process_of_the_wanted_level() {
        for (level, slot) in [(0u32, 0x240u32), (1, 0x24c), (2, 0x248)] {
            let mut e = engine2();
            let (actor, _) = actor2(&mut e);
            let _ = save_buffer(&mut e, 0x10);
            e.register(0x0093_2490, |_, _| Ret::default());
            e.mem.set_u32(actor.addr() + 0x68, 0);
            e.register(0x0044_0d80, |_, _| 0.into_ret());
            e.register(0x0044_0da0, |_, _| 0.into_ret());
            e.register(OPERATOR_NEW, |e, a| {
                assert_eq!(a[0], 0xb4);
                e.mem.alloc(0xb4).into_ret()
            });
            e.register(0x0090_6dc0, |_, a| a[0].into_ret());
            e.register_double(0x0093_34b0, move |_, _| level.into_ret());
            e.call_log = Some(vec![]);
            e.call(0x008a_a4c0, &args![actor, 0u32, 0u32]);
            let log = calls(&mut e);
            assert_ne!(e.mem.u32(actor.addr() + 0x68), 0);
            assert_eq!(called(&log, target(ACTOR2_VT, slot)).len(), 1, "{level}");
        }
        // A kind the tests accept gets no process.
        let mut e = engine2();
        let (actor, _) = actor2(&mut e);
        let _ = save_buffer(&mut e, 0x10);
        e.register(0x0093_2490, |_, _| Ret::default());
        e.mem.set_u32(actor.addr() + 0x68, 0);
        e.register(0x0044_0d80, |_, _| 0.into_ret());
        e.register(0x0044_0da0, |_, _| 1.into_ret());
        e.call(0x008a_a4c0, &args![actor, 0u32, 0u32]);
        assert_eq!(e.mem.u32(actor.addr() + 0x68), 0);
    }

    #[test]
    fn finish_init_load_attaches_the_effect_light_and_updates_the_actor() {
        let mut e = engine2();
        let (actor, proc) = actor2(&mut e);
        e.set_global(PLAYER_CHARACTER, 0u32);
        e.register(0x0093_2520, |_, _| Ret::default());
        // The holder at +0x88 has its own vtable pointer; method +0x34 answers
        // the held magic item.
        e.mem.set_u32(actor.addr() + 0x88, OBJECT2_VT);
        answer(&mut e, OBJECT2_VT, 0x34, 0x6100);
        e.register(0x0043_fcd0, |_, _| 0x6200.into_ret());
        e.register(0x0040_a300, |_, a| {
            assert_eq!((a[0], a[1]), (0x6100, 0));
            0x6300.into_ret()
        });
        e.register(0x0048_cee0, |_, a| {
            assert_eq!(a[0], 0x6300 + 0x18);
            1.into_ret()
        });
        e.register(0x0081_e440, |_, _| 0x6400.into_ret());
        e.register(0x004e_af60, |_, _| 0.into_ret());
        let anim = animation_with_node(&mut e, 0);
        // The node chain: the object answers method +0xc with the node (an
        // object that takes the clone through method +0xdc).
        let node = object2(&mut e, 0x40);
        answer(&mut e, OBJECT2_VT, 0xc, node.addr());
        e.register_double(GET_ANIMATION, move |_, _| anim.addr().into_ret());
        e.register(0x004f_d380, |_, _| 0x6500.into_ret());
        e.register(0x0052_5420, |_, _| 0.into_ret());
        e.register(OPERATOR_NEW, |e, a| {
            assert_eq!(a[0], 0x1c);
            e.mem.alloc(0x1c).into_ret()
        });
        e.register(FORM_ID_OF, |_, _| 0x6600.into_ret());
        e.register(0x0081_bf80, |_, a| a[0].into_ret());
        e.register(0x006e_cd40, |_, _| Ret::default());
        e.register(0x008c_4640, |_, _| Ret::default());
        // The process's method +0x5b4 gives a positive value: speed method.
        e.register_double(target(PROCESS2_VT, 0x5b4), |_, _| Ret {
            st0: 1.5,
            ..Ret::default()
        });
        // Mode 9: the face animation data is touched.
        answer(&mut e, ACTOR2_VT, 0x214, 9);
        let face = object2(&mut e, 0x40);
        e.register_double(0x008a_dcb0, move |_, _| face.addr().into_ret());
        e.register(0x004f_8960, |_, _| 5.into_ret());
        e.register(0x0049_4710, |_, _| 0.into_ret());
        e.register(0x0056_59f0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x008a_a9a0, &args![actor, 11u32, 12u32]);
        let log = calls(&mut e);
        assert_eq!(
            called(&log, target(PROCESS2_VT, 0x464)),
            vec![vec![proc.addr(), actor.addr()]]
        );
        assert_eq!(called(&log, 0x0093_2520), vec![vec![actor.addr(), 11, 12]]);
        // The node got the clone.
        assert_eq!(
            called(&log, target(OBJECT2_VT, 0xdc)),
            vec![vec![node.addr(), 0x6400, 1]]
        );
        // The light was made with the world's form id and the node and set in
        // the holder.
        assert_eq!(called(&log, 0x006e_cd40).len(), 1);
        assert_eq!(called(&log, 0x006e_cd40)[0][0], actor.addr() + 0x88);
        let light = called(&log, 0x0081_bf80);
        assert_eq!(light.len(), 1);
        assert_eq!(light[0][1], 0x6600);
        assert_eq!(called(&log, 0x008c_4640), vec![vec![actor.addr()]]);
        let bool_f32 = 1.5f32.to_bits();
        assert_eq!(
            called(&log, target(ACTOR2_VT, 0x384)),
            vec![vec![actor.addr(), 1, bool_f32]]
        );
        assert_eq!(
            called(&log, target(OBJECT2_VT, 0xd8)),
            vec![vec![face.addr(), 1, 1]]
        );
        assert_eq!(called(&log, target(ACTOR2_VT, 0x3f4)).len(), 1);
        // The other actor's animation lacks group 3: it is initialised again.
        assert_eq!(called(&log, 0x0056_59f0), vec![vec![actor.addr()]]);
        assert_eq!(
            called(&log, target(PROCESS2_VT, 0x7a8)),
            vec![vec![proc.addr(), actor.addr()]]
        );
    }

    #[test]
    fn finish_init_load_without_a_held_item_or_process_does_little() {
        let mut e = engine2();
        let (actor, _) = actor2(&mut e);
        e.set_global(PLAYER_CHARACTER, actor.addr());
        e.mem.set_u32(actor.addr() + 0x68, 0);
        e.mem.set_u32(actor.addr() + 0x88, OBJECT2_VT);
        e.register(0x0093_2520, |_, _| Ret::default());
        e.register(0x008c_4640, |_, _| Ret::default());
        e.register(0x0043_7bd0, |_, _| 1.into_ret());
        let face = object2(&mut e, 0x40);
        e.register_double(0x008a_dcb0, move |_, _| face.addr().into_ret());
        e.call_log = Some(vec![]);
        e.call(0x008a_a9a0, &args![actor, 0u32, 0u32]);
        let log = calls(&mut e);
        // 00437bd0 holds: the face animation data is touched with (1, 1).
        assert_eq!(
            called(&log, target(OBJECT2_VT, 0xd8)),
            vec![vec![face.addr(), 1, 1]]
        );
        // The player is never re-initialised and the process missing.
        assert!(called(&log, 0x0056_59f0).is_empty());
        assert!(called(&log, 0x0040_a300).is_empty());
    }

    #[test]
    fn keeps_state_across_the_load_depends_on_the_state_and_the_process() {
        let mut e = engine2();
        let (actor, _) = actor2(&mut e);
        e.register(0x0043_fcd0, |_, _| 0.into_ret());
        assert!(!e.call(0x008a_ad40, &args![actor, 0x400u32]).bool());
        e.register(0x0043_fcd0, |_, _| 1.into_ret());
        // Flag 0x400 and state 1, 2 or 6.
        for state in [1u32, 2, 6] {
            e.register_double(0x004f_8960, move |_, _| state.into_ret());
            assert!(e.call(0x008a_ad40, &args![actor, 0x400u32]).bool());
        }
        // The process's method +0x40c: 1, 2, 3, 4, 5 are kept, others not.
        e.register(0x004f_8960, |_, _| 0.into_ret());
        for (kind, expected) in [
            (0u32, false),
            (1, true),
            (2, true),
            (3, true),
            (4, true),
            (5, true),
            (6, false),
        ] {
            answer(&mut e, PROCESS2_VT, 0x40c, kind);
            assert_eq!(
                e.call(0x008a_ad40, &args![actor, 0u32]).bool(),
                expected,
                "{kind}"
            );
        }
        // No process: not kept.
        let bare: Ptr<Actor> = e.new_object();
        assert!(!e.call(0x008a_ad40, &args![bare, 0u32]).bool());
        // State 4 with a list entry that casts and has a low value.
        e.register(0x004f_8960, |_, _| 4.into_ret());
        answer(&mut e, PROCESS2_VT, 0x40c, 0);
        e.mem.set_u32(actor.addr() + 0x94, OBJECT2_VT);
        let item = e.mem.alloc(0x10);
        let list = node(&mut e, item, 0);
        answer(&mut e, OBJECT2_VT, 0x8, list);
        e.register(DYNAMIC_CAST, |_, a| {
            assert_eq!((a[2], a[3]), (CAST_FROM_011A0D1C, CAST_TO_011A146C));
            a[0].into_ret()
        });
        e.register(0x0068_a810, |_, _| 0x1d.into_ret());
        assert!(e.call(0x008a_ad40, &args![actor, 0x400u32]).bool());
        e.register(0x0068_a810, |_, _| 0x1e.into_ret());
        assert!(!e.call(0x008a_ad40, &args![actor, 0x400u32]).bool());
        e.register(DYNAMIC_CAST, |_, _| 0.into_ret());
        assert!(!e.call(0x008a_ad40, &args![actor, 0x400u32]).bool());
    }

    #[test]
    fn buffer_flag_is_forwarded_when_the_actor_has_no_3d_owner() {
        let mut e = engine2();
        let (actor, _) = actor2(&mut e);
        let buffer = buffer_object(&mut e, 0x1000_0000, 0);
        e.register(0x0056_2140, |_, _| Ret::default());
        e.register(0x0056_21f0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x008a_aee0, &args![actor, buffer]);
        let log = calls(&mut e);
        assert_eq!(called(&log, 0x0056_2140), vec![vec![actor.addr(), buffer]]);
        assert_eq!(called(&log, 0x0056_21f0), vec![vec![buffer, 0x1000_0000]]);
        // Method +0x1e4 nonzero: not forwarded.
        answer(&mut e, ACTOR2_VT, 0x1e4, 1);
        e.call_log = Some(vec![]);
        e.call(0x008a_aee0, &args![actor, buffer]);
        assert!(called(&calls(&mut e), 0x0056_21f0).is_empty());
        // Flag missing: not forwarded.
        answer(&mut e, ACTOR2_VT, 0x1e4, 0);
        e.mem.set_u32(buffer + 0x17, 0);
        e.call_log = Some(vec![]);
        e.call(0x008a_aee0, &args![actor, buffer]);
        assert!(called(&calls(&mut e), 0x0056_21f0).is_empty());
    }

    #[test]
    fn save_game_writes_the_format_in_order() {
        let mut e = engine2();
        let actor = actor_with_lists(&mut e);
        let buffer = buffer_object(&mut e, 0x400 | 0x8_0000 | 0x80_0000 | 0x40_0000, 0);
        e.register(0x0093_2880, |_, _| Ret::default());
        e.register_double(0x0096_d490, |_, _| Ret {
            st0: 10.0,
            ..Ret::default()
        });
        e.mem.set_f32(actor.addr() + 0x114, 4.0);
        e.register(0x004f_8960, |_, _| 5.into_ret());
        let written = Rc::new(RefCell::new(Vec::<(u32, u32, u32)>::new()));
        let record = written.clone();
        e.register_double(BUFFER_WRITE, move |e, a| {
            record.borrow_mut().push((a[1], a[2], e.mem.u32(a[1])));
            Ret::default()
        });
        e.register(BUFFER_SAVE_FORM_ID, |_, _| Ret::default());
        e.register(0x0086_5f20, |_, _| 0x4000.into_ret());
        e.register(0x0086_5ff0, |_, _| Ret::default());
        e.register(0x0093_7b50, |_, _| Ret::default());
        let mover = object2(&mut e, 0x40);
        e.mem.set_u32(actor.addr() + 0x190, mover.addr());
        e.call_log = Some(vec![]);
        e.call(0x008a_af40, &args![actor, buffer]);
        let log = calls(&mut e);
        let writes = written.borrow().clone();
        // The first write is the elapsed time 10.0 - 4.0 from the stack.
        assert_eq!(writes[0].1, 4);
        assert_eq!(f32::from_bits(writes[0].2), 6.0);
        // The next 30 are the fields of the format in order.
        let offsets: Vec<(u32, u32)> = writes[1..31]
            .iter()
            .map(|w| (w.0 - actor.addr(), w.1))
            .collect();
        let expected: Vec<(u32, u32)> = FORMAT_FIELDS
            .iter()
            .chain(FORMAT_FIELDS_V8.iter())
            .chain(FORMAT_FIELDS_V9.iter())
            .chain(std::iter::once(&FORMAT_FIELD_V13))
            .copied()
            .collect();
        assert_eq!(offsets, expected);
        // The state byte, then the list entry (the item with a form id).
        assert_eq!(writes[31].1, 1);
        assert_eq!(writes[31].2 & 0xff, 5);
        assert_eq!(writes[32].1, 4);
        assert_eq!(writes[33].1, 4);
        // The form ids: the three references, then the items with a form.
        let ids = called(&log, BUFFER_SAVE_FORM_ID);
        assert_eq!(ids.len(), 5);
        // The counted block: both items (the second without a form).
        assert_eq!(called(&log, 0x0086_5ff0), vec![vec![buffer, 2, 0x4000]]);
        assert_eq!(
            called(&log, 0x0093_7b50),
            vec![
                vec![actor.addr() + 0xd0, buffer],
                vec![actor.addr() + 0xe0, buffer]
            ]
        );
        assert_eq!(
            called(&log, target(OBJECT2_VT, 0x28)),
            vec![vec![mover.addr(), buffer]]
        );
    }

    /// A buffer that reads from a queue: `864980` fills its target, `864a60`
    /// returns the next value, `8648a0` the next id, `8648e0` stores the next
    /// id at its target.
    fn load_buffer(e: &mut Engine, version: u32, flags: u32) -> (u32, Rc<RefCell<VecDeque<u32>>>) {
        let buffer = buffer_object(e, flags, 0);
        answer(e, OBJECT2_VT, 0, version);
        let reads: Rc<RefCell<VecDeque<u32>>> = Rc::default();
        let queue = reads.clone();
        e.register_double(BUFFER_READ, move |e, a| {
            let value = queue.borrow_mut().pop_front().unwrap_or(0);
            match a[2] {
                1 => e.mem.set_u8(a[1], value as u8),
                2 => e.mem.set_u16(a[1], value as u16),
                _ => e.mem.set_u32(a[1], value),
            }
            Ret::default()
        });
        let queue = reads.clone();
        e.register_double(BUFFER_LOAD_FORM_ID, move |e, a| {
            let value = queue.borrow_mut().pop_front().unwrap_or(0);
            e.mem.set_u32(a[1], value);
            Ret::default()
        });
        let queue = reads.clone();
        e.register_double(0x0086_48a0, move |_, _| {
            queue.borrow_mut().pop_front().unwrap_or(0).into_ret()
        });
        let queue = reads.clone();
        e.register_double(0x0086_4a60, move |_, _| {
            queue.borrow_mut().pop_front().unwrap_or(0).into_ret()
        });
        (buffer, reads)
    }

    #[test]
    fn buffer_load_reads_the_fields_and_converts_the_clock() {
        let mut e = engine2();
        let (actor, _) = actor2(&mut e);
        let (buffer, reads) = load_buffer(&mut e, 0xd, 0x400 | 0x8_0000);
        e.register(0x004f_8960, |_, _| 3.into_ret());
        e.register(0x0093_2a00, |_, _| Ret::default());
        e.register(0x0088_4f80, |_, _| Ret::default());
        e.register(0x0096_d490, |_, _| Ret {
            st0: 10.0,
            ..Ret::default()
        });
        e.register(0x008c_1ac0, |_, _| Ret::default());
        e.register(0x008c_1b50, |_, _| Ret::default());
        e.register(LOOKUP_FORM, |_, a| a[0].into_ret());
        e.register(DYNAMIC_CAST, |_, a| (a[0] + 1).into_ret());
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(0x005a_e3d0, |_, _| Ret::default());
        e.register(0x0093_7c60, |_, _| Ret::default());
        e.register(0x0043_fcd0, |_, _| 0.into_ret());
        // Reads in order: clock stamp 2.0, the 23 fields (value i + 1), the
        // version 8 field, the five of version 9, the version 13 field, the
        // id for +0x148, the state byte, the count 1 and its block data.
        let mut values = vec![2.0f32.to_bits()];
        values.extend((1..=23).map(|i| i as u32));
        values.push(0x88); // 0x10c
        values.extend([0x91, 0x92, 0x93, 0x94, 0x95]);
        values.push(0x96); // 0x120
        let words = values.len();
        reads.borrow_mut().extend(values);
        // The form reference ids are interleaved: +0xc0's goes through
        // LoadFormID_ov2 (pops a value), then the id of +0x148.
        reads
            .borrow_mut()
            .extend([0xc0c0, 0x1480, 0x7070, 4, 1, 0xb10c, 0xb10d]);
        assert!(words > 0);
        let mover = object2(&mut e, 0x40);
        e.mem.set_u32(actor.addr() + 0x190, mover.addr());
        e.call_log = Some(vec![]);
        e.call(0x008a_b3a0, &args![actor, buffer]);
        let log = calls(&mut e);
        // The clock stamp became 10.0 - 2.0; the previous state is kept.
        assert_eq!(e.mem.f32(actor.addr() + 0x114), 8.0);
        for (index, (offset, size)) in FORMAT_FIELDS.iter().enumerate() {
            let value = e.mem.u32(actor.addr() + offset);
            let mask = if *size == 1 { 0xff } else { u32::MAX };
            assert_eq!(value & mask, (index as u32 + 1) & mask, "{offset:x}");
        }
        assert!(called(&log, BUFFER_READ)
            .iter()
            .any(|c| c[1] == actor.addr() + 0x10c));
        // Method +0x22c(0) is false and the field is nonzero: reset at the end.
        assert_eq!(e.mem.u32(actor.addr() + 0x10c), 0);
        assert_eq!(e.mem.u32(actor.addr() + 0x120), 0x96);
        // +0xc0 and +0x70 get their ids from LoadFormID_ov2; +0x148 is the
        // cast of the looked up form.
        assert_eq!(e.mem.u32(actor.addr() + 0xc0), 0xc0c0);
        assert_eq!(e.mem.u32(actor.addr() + 0x148), 0x1481);
        assert_eq!(e.mem.u32(actor.addr() + 0x70), 0x7070);
        // The state byte 4 was loaded for flag 0x400.
        assert_eq!(e.mem.u32(actor.addr() + 0x108), 4);
        assert_eq!(
            called(&log, target(OBJECT2_VT, 0x2c)),
            vec![vec![mover.addr(), buffer]]
        );
        assert_eq!(called(&log, 0x008c_1ac0), vec![vec![actor.addr(), 0]]);
        assert_eq!(called(&log, 0x008c_1b50), vec![vec![actor.addr(), 0]]);
        // One block was pushed onto the list at +0xfc.
        let pushes = called(&log, 0x005a_e3d0);
        assert_eq!(pushes.len(), 1);
        assert_eq!(pushes[0][0], actor.addr() + 0xfc);
    }

    #[test]
    fn buffer_load_old_version_skips_the_later_fields_and_the_modifier_lists_are_read() {
        let mut e = engine2();
        let (actor, _) = actor2(&mut e);
        let (buffer, _) = load_buffer(&mut e, 5, 0x80_0000 | 0x40_0000);
        e.register(0x004f_8960, |_, _| 3.into_ret());
        e.register(0x0093_2a00, |_, _| Ret::default());
        e.register(0x0088_4f80, |_, _| Ret::default());
        e.register(0x0096_d490, |_, _| Ret::default());
        e.register(LOOKUP_FORM, |_, _| 0.into_ret());
        e.register(DYNAMIC_CAST, |_, _| 0.into_ret());
        e.register(0x0093_7c60, |_, _| Ret::default());
        e.register(0x0043_fcd0, |_, _| 0.into_ret());
        let mover = object2(&mut e, 0x40);
        e.mem.set_u32(actor.addr() + 0x190, mover.addr());
        e.mem.set_u32(actor.addr() + 0x10c, 9);
        e.call_log = Some(vec![]);
        e.call(0x008a_b3a0, &args![actor, buffer]);
        let log = calls(&mut e);
        let reads: Vec<u32> = called(&log, BUFFER_READ)
            .iter()
            .map(|c| c[1] - actor.addr())
            .collect();
        assert!(!reads.contains(&0x10c));
        assert!(!reads.contains(&0x134));
        assert!(!reads.contains(&0x120));
        assert_eq!(
            called(&log, 0x0093_7c60),
            vec![
                vec![actor.addr() + 0xd0, buffer],
                vec![actor.addr() + 0xe0, buffer]
            ]
        );
        // Method +0x22c(0) false and field +0x10c nonzero: reset to 0.
        assert_eq!(e.mem.u32(actor.addr() + 0x10c), 0);
    }

    #[test]
    fn buffer_load_compares_the_equipment_with_the_inventory() {
        let mut e = engine2();
        let (actor, proc) = actor2(&mut e);
        let (buffer, _) = load_buffer(&mut e, 5, 0);
        e.register(0x004f_8960, |_, _| 3.into_ret());
        e.register(0x0093_2a00, |_, _| Ret::default());
        e.register(0x0088_4f80, |_, _| Ret::default());
        e.register(0x0096_d490, |_, _| Ret::default());
        e.register(LOOKUP_FORM, |_, _| 0.into_ret());
        e.register(DYNAMIC_CAST, |_, _| 0.into_ret());
        let mover = object2(&mut e, 0x40);
        e.mem.set_u32(actor.addr() + 0x190, mover.addr());
        e.register(0x0043_fcd0, |_, _| 1.into_ret());
        let equipment = object2(&mut e, 0x40);
        answer(&mut e, ACTOR2_VT, 0x1e8, equipment.addr());
        e.register(GET_INVENTORY_CHANGES, |_, _| 0x9100.into_ret());
        // Slot items: slot i holds the object `items[i]`, except slot 5 which
        // holds another one; the worn item of slot i is `0x8000 + i` whose form
        // is `items[i]`.
        let items: Vec<u32> = (0..0x13).map(|_| object2(&mut e, 0x40).addr()).collect();
        let other = object2(&mut e, 0x40).addr();
        let slot_items = items.clone();
        e.register_double(0x0088_e0f0, move |_, a| {
            (if a[1] == 5 {
                other
            } else {
                slot_items[a[1] as usize]
            })
            .into_ret()
        });
        e.register(0x0040_1170, |_, _| 7.into_ret());
        e.register(0x004c_8c10, |_, a| (0x8000 + a[1]).into_ret());
        let worn_forms = items.clone();
        e.register_double(WORD_AT_8, move |_, a| {
            worn_forms[(a[0] - 0x8000) as usize].into_ret()
        });
        e.register(0x0044_59e0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x008a_b3a0, &args![actor, buffer]);
        let log = calls(&mut e);
        // The worn items of slots 0 to 4 are released; slot 5 differs: the
        // process is told and the comparison stops.
        assert_eq!(called(&log, 0x0044_59e0).len(), 5);
        assert_eq!(
            called(&log, target(PROCESS2_VT, 0x468)),
            vec![vec![proc.addr(), 1]]
        );
        assert_eq!(called(&log, 0x0088_e0f0).len(), 6);
        // A slot item of form type 0xc or one answering method +0xf4 counts as
        // empty: with all slots empty and no worn items nothing differs.
        e.register(0x0088_e0f0, |_, a| (0x100 + a[1]).into_ret());
        e.register(0x0040_1170, |_, _| 0xc.into_ret());
        e.register(0x004c_8c10, |_, _| 0.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x008a_b3a0, &args![actor, buffer]);
        assert!(called(&calls(&mut e), target(PROCESS2_VT, 0x468)).is_empty());
    }

    #[test]
    fn buffer_load_checks_the_lily_actor_without_equipment_data() {
        let mut e = engine2();
        let (actor, proc) = actor2(&mut e);
        let (buffer, _) = load_buffer(&mut e, 5, 0);
        e.register(0x004f_8960, |_, _| 3.into_ret());
        e.register(0x0093_2a00, |_, _| Ret::default());
        e.register(0x0088_4f80, |_, _| Ret::default());
        e.register(0x0096_d490, |_, _| Ret::default());
        e.register(LOOKUP_FORM, |_, _| 0.into_ret());
        e.register(DYNAMIC_CAST, |_, _| 0.into_ret());
        let mover = object2(&mut e, 0x40);
        e.mem.set_u32(actor.addr() + 0x190, mover.addr());
        e.register(0x0043_fcd0, |_, _| 1.into_ret());
        answer(&mut e, ACTOR2_VT, 0x1e8, 0);
        e.register(GET_INVENTORY_CHANGES, |_, _| 0x9100.into_ret());
        answer(&mut e, ACTOR2_VT, 0x21c, 1);
        e.register(0x0056_6950, |_, _| 1.into_ret());
        e.register(0x0055_d520, |_, _| 0x7777.into_ret());
        e.register(0x00ec_7750, |_, a| {
            assert_eq!((a[0], a[1]), (0x7777, NAME_FRAGMENT));
            1.into_ret()
        });
        e.register(0x004c_8c10, |_, a| {
            assert_eq!(a[1], 5);
            0x8000.into_ret()
        });
        e.register(WORD_AT_8, |_, a| a[0].into_ret());
        // The weapon's form differs from the worn item's: the process is told.
        answer(&mut e, PROCESS2_VT, 0x148, 0x1111);
        e.call_log = Some(vec![]);
        e.call(0x008a_b3a0, &args![actor, buffer]);
        assert_eq!(
            called(&calls(&mut e), target(PROCESS2_VT, 0x468)),
            vec![vec![proc.addr(), 1]]
        );
        // The same form: not told.
        answer(&mut e, PROCESS2_VT, 0x148, 0x8000);
        e.call_log = Some(vec![]);
        e.call(0x008a_b3a0, &args![actor, buffer]);
        assert!(called(&calls(&mut e), target(PROCESS2_VT, 0x468)).is_empty());
        // The name lacks the text: nothing at all.
        e.register(0x00ec_7750, |_, _| 0.into_ret());
        answer(&mut e, PROCESS2_VT, 0x148, 0x1111);
        e.call_log = Some(vec![]);
        e.call(0x008a_b3a0, &args![actor, buffer]);
        assert!(called(&calls(&mut e), target(PROCESS2_VT, 0x468)).is_empty());
    }

    #[test]
    fn buffer_finish_resolves_forms_and_follows_the_package_for_combat() {
        let mut e = engine2();
        let actor = actor_with_lists(&mut e);
        let buffer = buffer_object(&mut e, 0x8_0000, 0);
        e.register(0x0093_2b70, |_, _| Ret::default());
        e.register(LOOKUP_FORM, |_, a| (a[0] + 0x1000).into_ret());
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.mem.set_u32(actor.addr() + 0xc0, 0x31);
        e.mem.set_u32(actor.addr() + 0x70, 0x41);
        let item = e.mem.u32(actor.addr() + 0xfc);
        e.mem.set_u32(item + 4, 0x51);
        let mover = object2(&mut e, 0x40);
        e.mem.set_u32(actor.addr() + 0x190, mover.addr());
        form_type_from_byte(&mut e);
        let package = object2(&mut e, 0x40);
        e.mem.set_u8(package.addr() + 0x20, 0x12);
        answer(&mut e, PROCESS2_VT, 0x22c, package.addr());
        e.register(CLEAR_IN_COMBAT, |_, _| Ret::default());
        e.register(0x0042_ce90, |_, _| 0.into_ret());
        e.set_global(PLAYER_CHARACTER, 0u32);
        e.register(0x004f_8960, |_, _| 5.into_ret());
        e.register(GET_ANIMATION, |_, _| 0.into_ret());
        e.register(0x008b_78c0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x008a_bc40, &args![actor, buffer]);
        let log = calls(&mut e);
        assert_eq!(e.mem.u32(actor.addr() + 0xc0), 0x1031);
        assert_eq!(e.mem.u32(actor.addr() + 0x70), 0x1041);
        assert_eq!(e.mem.u32(item + 4), 0x1051);
        // The package of type 0x12 sets the flag; no ClearInCombat.
        assert_eq!(e.mem.u8(actor.addr() + 0x104), 1);
        assert!(called(&log, CLEAR_IN_COMBAT).is_empty());
        assert_eq!(
            called(&log, target(OBJECT2_VT, 0x30)),
            vec![vec![mover.addr(), buffer]]
        );
        // Without a package of that type the combat state is cleared.
        e.mem.set_u8(package.addr() + 0x20, 0x11);
        e.call_log = Some(vec![]);
        e.call(0x008a_bc40, &args![actor, buffer]);
        assert_eq!(
            called(&calls(&mut e), CLEAR_IN_COMBAT),
            vec![vec![actor.addr(), 0]]
        );
    }

    #[test]
    fn buffer_finish_registers_with_the_player_sets_up_worn_items_and_animation() {
        let mut e = engine2();
        let (actor, proc) = actor2(&mut e);
        let buffer = buffer_object(&mut e, 0, 0);
        e.register(0x0093_2b70, |_, _| Ret::default());
        let mover = object2(&mut e, 0x40);
        e.mem.set_u32(actor.addr() + 0x190, mover.addr());
        e.register(CLEAR_IN_COMBAT, |_, _| Ret::default());
        // The player object has the counter and the list of `fn_008abfa0`.
        let player = Ptr::<()>::new(e.mem.alloc(0xe00));
        e.set_global(PLAYER_CHARACTER, player.addr());
        e.register(0x005a_e3d0, |_, _| Ret::default());
        e.mem.set_u8(actor.addr() + 0x18d, 1);
        // `42ce90` holds and the flag word lacks 0x8000020: worn items are set up
        // for a creature (method +0x21c) or an NPC base.
        e.register(0x0042_ce90, |_, _| 1.into_ret());
        answer(&mut e, ACTOR2_VT, 0x1d0, 0x6a00);
        e.register(0x0041_81e0, |_, _| 0x6000.into_ret());
        e.register(0x005f_9e00, |_, _| Ret::default());
        e.register(0x0060_47c0, |_, _| Ret::default());
        e.register(0x004f_8960, |_, _| 5.into_ret());
        let anim = object2(&mut e, 0x40);
        e.register_double(GET_ANIMATION, move |_, _| anim.addr().into_ret());
        e.register(0x0049_4710, |_, _| 0.into_ret());
        e.register(0x0088_7d00, |_, _| Ret::default());
        e.register(0x008b_78c0, |_, _| Ret::default());
        answer(&mut e, PROCESS2_VT, 0x478, 1);
        e.register(0x00c7_c150, |_, _| Ret::default());
        e.register(0x00ca_0cd0, |_, _| Ret::default());
        e.mem.set_u32(actor.addr() + 0xac, 0x7a00);
        e.mem.set_u32(actor.addr() + 0xb0, 0x7b00);
        e.call_log = Some(vec![]);
        e.call(0x008a_bc40, &args![actor, buffer]);
        let log = calls(&mut e);
        assert_eq!(e.mem.u32(player.addr() + 0xd68), 1);
        assert_eq!(called(&log, 0x005a_e3d0).len(), 1);
        assert_eq!(
            called(&log, 0x0060_47c0),
            vec![vec![0x6000, actor.addr(), 1, 1, 0, 1]]
        );
        assert_eq!(called(&log, 0x0088_7d00), vec![vec![actor.addr()]]);
        assert_eq!(called(&log, 0x008b_78c0), vec![vec![actor.addr(), 0]]);
        assert_eq!(
            called(&log, target(PROCESS2_VT, 0x464)),
            vec![vec![proc.addr(), actor.addr()]]
        );
        assert_eq!(called(&log, 0x00c7_c150), vec![vec![0x7a00, 0]]);
        assert_eq!(called(&log, 0x00ca_0cd0), vec![vec![0x7b00]]);
        // A creature gets 005f9e00 instead.
        answer(&mut e, ACTOR2_VT, 0x21c, 1);
        e.call_log = Some(vec![]);
        e.call(0x008a_bc40, &args![actor, buffer]);
        assert_eq!(
            called(&calls(&mut e), 0x005f_9e00),
            vec![vec![0x6000, actor.addr(), 1, 1, 1]]
        );
        // The flag word having 0x8000020 prevents it.
        let flagged = buffer_object(&mut e, 0x800_0020, 0);
        e.call_log = Some(vec![]);
        e.call(0x008a_bc40, &args![actor, flagged]);
        assert!(called(&calls(&mut e), 0x005f_9e00).is_empty());
    }

    #[test]
    fn list_registration_counts_and_appends_the_argument_address() {
        let mut e = engine2();
        let owner = Ptr::<()>::new(e.mem.alloc(0xe00));
        e.register_double(0x005a_e3d0, {
            move |e, a| {
                // The pointer is the address of a word holding the value.
                assert_eq!(e.mem.u32(a[1]), 0x1234);
                assert_eq!(a[0], owner.addr() + 0x5fc);
                Ret::default()
            }
        });
        e.call(0x008a_bfa0, &args![owner, 0x1234u32]);
        e.call(0x008a_bfa0, &args![owner, 0x1234u32]);
        assert_eq!(e.mem.u32(owner.addr() + 0xd68), 2);
    }

    #[test]
    fn buffer_last_step_enables_the_ragdoll_of_a_dead_actor() {
        let mut e = engine2();
        let (actor, _) = actor2(&mut e);
        let buffer = buffer_object(&mut e, 0, 0);
        e.register(0x0093_2d60, |_, _| Ret::default());
        let mover = object2(&mut e, 0x40);
        e.mem.set_u32(actor.addr() + 0x190, mover.addr());
        e.register(0x0088_b150, |_, _| Ret::default());
        answer(&mut e, ACTOR2_VT, 0x1d0, 0x6a00);
        answer(&mut e, ACTOR2_VT, 0x2e8, 1);
        e.mem.set_u32(actor.addr() + 0xac, 0x7a00);
        e.mem.set_u32(actor.addr() + 0xb0, 0x7b00);
        e.register(0x00c7_c150, |_, _| Ret::default());
        e.register(0x0093_1ed0, |_, a| a[1].into_ret());
        e.register(0x004a_3a20, |_, _| 0x66.into_ret());
        e.register(0x00ca_2ad0, |_, _| Ret::default());
        e.register(GET_EXTRA_LIST, |_, _| 0x1000.into_ret());
        e.register(0x0041_d6d0, |_, _| 0.into_ret());
        e.register(0x00c9_b670, |_, _| Ret::default());
        e.register(0x0043_d410, |_, _| Ret::default());
        e.register(0x00a5_9c60, |_, _| Ret::default());
        e.register(0x008c_4640, |_, _| Ret::default());
        e.register(0x0043_7bd0, |_, _| 0.into_ret());
        e.register(0x008c_1470, |_, _| Ret::default());
        e.register(0x008b_65f0, |_, _| Ret::default());
        e.register(0x008a_1a70, |_, _| Ret::default());
        e.mem.set_u32(actor.addr() + 0x10c, 3);
        e.call_log = Some(vec![]);
        e.call(0x008a_bfe0, &args![actor, buffer]);
        let log = calls(&mut e);
        assert_eq!(called(&log, 0x00c7_c150), vec![vec![0x7a00, 1]]);
        assert_eq!(called(&log, 0x00ca_2ad0), vec![vec![0x7b00, 0x6a00, 0x66]]);
        // No ragdoll data: the knock-down starts with the zero point.
        assert_eq!(
            called(&log, 0x00c9_b670),
            vec![vec![0x6a00, ZERO_POINT, 1, 0, 0]]
        );
        assert_eq!(
            called(&log, target(OBJECT2_VT, 0x34)),
            vec![vec![mover.addr(), buffer]]
        );
        assert_eq!(called(&log, 0x00a5_9c60).len(), 1);
        assert_eq!(called(&log, 0x008a_1a70), vec![vec![actor.addr()]]);
        // With the flag 4 in the buffer the ragdoll is left alone, the rest runs.
        let flagged = buffer_object(&mut e, 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x008a_bfe0, &args![actor, flagged]);
        let log = calls(&mut e);
        assert!(called(&log, 0x00c7_c150).is_empty());
        assert_eq!(called(&log, 0x00a5_9c60).len(), 1);
        // No 3D: no ragdoll and no local update.
        answer(&mut e, ACTOR2_VT, 0x1d0, 0);
        e.call_log = Some(vec![]);
        e.call(0x008a_bfe0, &args![actor, buffer]);
        assert!(called(&calls(&mut e), 0x00a5_9c60).is_empty());
    }

    #[test]
    fn buffer_last_step_touches_the_face_for_mode_nine_and_the_stuff_around() {
        let mut e = engine2();
        let (actor, _) = actor2(&mut e);
        let buffer = buffer_object(&mut e, 0, 0);
        e.register(0x0093_2d60, |_, _| Ret::default());
        let mover = object2(&mut e, 0x40);
        e.mem.set_u32(actor.addr() + 0x190, mover.addr());
        e.register(0x0088_b150, |_, _| Ret::default());
        e.register(0x008c_4640, |_, _| Ret::default());
        e.register(0x008c_1470, |_, _| Ret::default());
        e.register(0x008b_65f0, |_, _| Ret::default());
        let face = object2(&mut e, 0x40);
        e.register_double(0x008a_dcb0, move |_, _| face.addr().into_ret());
        e.register(0x0043_7bd0, |_, _| 0.into_ret());
        // Method +0x22c(1) gets method +0x2a0 called.
        answer(&mut e, ACTOR2_VT, 0x22c, 1);
        e.call_log = Some(vec![]);
        e.call(0x008a_bfe0, &args![actor, buffer]);
        let log = calls(&mut e);
        assert_eq!(called(&log, target(ACTOR2_VT, 0x2a0)).len(), 1);
        // +0x22c(0) is true as well: the face is told.
        assert_eq!(
            called(&log, target(OBJECT2_VT, 0xd8)),
            vec![vec![face.addr(), 1, 1]]
        );
        // Neither mode 9, nor 00437bd0, nor +0x22c: the face is left alone.
        answer(&mut e, ACTOR2_VT, 0x22c, 0);
        e.call_log = Some(vec![]);
        e.call(0x008a_bfe0, &args![actor, buffer]);
        assert!(called(&calls(&mut e), target(OBJECT2_VT, 0xd8)).is_empty());
        answer(&mut e, ACTOR2_VT, 0x214, 9);
        e.call_log = Some(vec![]);
        e.call(0x008a_bfe0, &args![actor, buffer]);
        assert_eq!(called(&calls(&mut e), target(OBJECT2_VT, 0xd8)).len(), 1);
    }

    #[test]
    fn buffer_revert_resets_the_actor() {
        let mut e = engine2();
        let (actor, _) = actor2(&mut e);
        let buffer = buffer_object(&mut e, 0x800_0020, 0x400 | 0x8_0000 | 0x80_0000 | 0x40_0000);
        e.register(0x0057_b520, |_, _| Ret::default());
        e.register(0x0057_1b50, |_, _| Ret::default());
        e.register(0x0045_34f0, |_, _| Ret::default());
        e.register(0x0048_3710, |_, _| Ret::default());
        e.register(0x008b_6820, |_, _| Ret::default());
        e.register(0x0089_d690, |_, _| 1.into_ret());
        e.register(0x00c7_b6a0, |_, _| Ret::default());
        e.register(0x0093_2f60, |_, _| Ret::default());
        e.register(0x008b_bdb0, |_, _| Ret::default());
        e.register(0x008b_be00, |_, _| Ret::default());
        e.register(CLEAR_IN_COMBAT, |_, _| Ret::default());
        e.register(0x0088_4f80, |_, _| Ret::default());
        e.register(0x008c_4400, |_, _| Ret::default());
        e.register(0x0087_fd20, |_, _| Ret::default());
        e.register(0x0093_7d50, |_, _| Ret::default());
        e.register(0x008a_cc80, |_, _| Ret::default());
        e.register(0x0041_81e0, |_, _| 0x6000.into_ret());
        e.register(0x005f_0b00, |_, _| 0.into_ret());
        e.register(0x0062_c3c0, |_, _| 1.into_ret());
        e.register(0x0057_7330, |_, _| Ret::default());
        let mover = object2(&mut e, 0x40);
        e.mem.set_u32(actor.addr() + 0x190, mover.addr());
        let doomed = object2(&mut e, 0x40);
        e.mem.set_u32(actor.addr() + 0x12c, doomed.addr());
        e.mem.set_u32(actor.addr() + 0x130, doomed.addr());
        e.mem.set_u32(actor.addr() + 0xac, 0x7a00);
        answer(&mut e, ACTOR2_VT, 0x1d0, 0x6a00);
        e.mem.set_u32(actor.addr() + 0x108, 3);
        e.mem.set_u32(actor.addr() + 0x10c, 3);
        e.mem.set_u8(actor.addr() + 0xbc, 0);
        e.mem.set_u32(actor.addr() + 0x110, 7);
        e.call_log = Some(vec![]);
        e.call(0x008a_c1e0, &args![actor, buffer]);
        let log = calls(&mut e);
        assert_eq!(called(&log, 0x0057_b520), vec![vec![actor.addr(), 1]]);
        // Flag 0x8000020 in either word: weapon removed, 004534f0, two 00483710.
        assert_eq!(called(&log, 0x0057_1b50).len(), 1);
        assert_eq!(called(&log, 0x0048_3710).len(), 2);
        // Flag 0x20000 only in the second word: limbs cleared.
        assert!(called(&log, 0x008b_6820).is_empty());
        assert_eq!(called(&log, 0x00c7_b6a0), vec![vec![0x7a00, 0]]);
        assert_eq!(
            called(&log, target(OBJECT2_VT, 0)),
            vec![vec![doomed.addr(), 1], vec![doomed.addr(), 1]]
        );
        assert_eq!(e.mem.u32(actor.addr() + 0x12c), 0);
        assert_eq!(e.mem.u32(actor.addr() + 0x130), 0);
        assert_eq!(e.mem.u8(actor.addr() + 0xbc), 1);
        assert_eq!(e.mem.u32(actor.addr() + 0x110), 0xff);
        // The life state follows the health (0 or less: 2) and the last resets.
        assert_eq!(e.mem.u32(actor.addr() + 0x108), 2);
        assert_eq!(e.mem.u32(actor.addr() + 0x10c), 0);
        assert_eq!(called(&log, 0x008a_cc80).len(), 1);
        assert_eq!(called(&log, 0x0087_fd20).len(), 1);
        assert_eq!(
            called(&log, 0x0093_7d50),
            vec![
                vec![actor.addr() + 0xd0, buffer],
                vec![actor.addr() + 0xe0, buffer]
            ]
        );
        assert_eq!(
            called(&log, target(OBJECT2_VT, 0x38)),
            vec![vec![mover.addr(), buffer]]
        );
        // The 3D ragdoll test passes: the ragdoll data is restored.
        assert_eq!(called(&log, 0x0057_7330), vec![vec![actor.addr(), 0]]);
    }

    #[test]
    fn buffer_revert_clears_limbs_and_sets_a_living_state() {
        let mut e = engine2();
        let (actor, _) = actor2(&mut e);
        // 0x20000 in the second word only.
        let buffer = buffer_object(&mut e, 0, 0x2_0000 | 0x400);
        for address in [
            0x0057_b520u32,
            0x008b_6820,
            0x0093_2f60,
            0x008b_bdb0,
            0x008b_be00,
            CLEAR_IN_COMBAT,
            0x0088_4f80,
            0x008c_4400,
            0x008a_cc80,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(0x0041_81e0, |_, _| 0x6000.into_ret());
        e.register(0x005f_0b00, |_, _| 50.into_ret());
        let mover = object2(&mut e, 0x40);
        e.mem.set_u32(actor.addr() + 0x190, mover.addr());
        e.mem.set_u32(actor.addr() + 0x108, 0);
        e.call_log = Some(vec![]);
        e.call(0x008a_c1e0, &args![actor, buffer]);
        let log = calls(&mut e);
        assert_eq!(called(&log, 0x008b_6820), vec![vec![actor.addr()]]);
        assert_eq!(e.mem.u32(actor.addr() + 0x108), 0);
        // A previous state different from 0 is reset first.
        assert!(called(&log, 0x008a_cc80).is_empty());
        e.mem.set_u32(actor.addr() + 0x108, 4);
        e.call_log = Some(vec![]);
        e.call(0x008a_c1e0, &args![actor, buffer]);
        assert_eq!(called(&calls(&mut e), 0x008a_cc80).len(), 1);
    }

    #[test]
    fn package_extra_decides_for_interrupt_packages() {
        let mut e = engine2();
        let (actor, _) = actor2(&mut e);
        let package = object2(&mut e, 0x40);
        let extra = object2(&mut e, 0x40);
        answer(&mut e, PROCESS2_VT, 0x22c, package.addr());
        e.register(0x0067_8610, |_, _| 0.into_ret());
        e.register(0x0088_40d0, |_, a| (a[0] % 2 == 0).into_ret());
        e.register(GET_EXTRA_LIST, |_, _| 0x1000.into_ret());
        let extra_package = extra.addr();
        e.register_double(0x0041_cb10, move |_, _| extra_package.into_ret());
        // Not an interrupt package: tested itself (the address is even).
        assert!(e.call(0x008a_c680, &args![actor]).bool());
        // An interrupt package: the extra package is tested instead.
        e.register(0x0067_8610, |_, _| 1.into_ret());
        assert!(e.call(0x008a_c680, &args![actor]).bool());
        e.register(0x0088_40d0, |_, _| 0.into_ret());
        assert!(!e.call(0x008a_c680, &args![actor]).bool());
        // No package: false.
        answer(&mut e, PROCESS2_VT, 0x22c, 0);
        assert!(!e.call(0x008a_c680, &args![actor]).bool());
    }

    #[test]
    fn combatant_faction_needs_an_unexpelled_faction_with_the_flag() {
        let mut e = engine2();
        let (actor, _) = actor2(&mut e);
        e.register(0x0041_81e0, |_, _| 0x1000.into_ret());
        // The base's list: ranks whose first word is the faction.
        let faction_a = e.mem.alloc(0x40);
        let faction_b = e.mem.alloc(0x40);
        let rank_a = e.mem.alloc(8);
        let rank_b = e.mem.alloc(8);
        e.mem.set_u32(rank_a, faction_a);
        e.mem.set_u32(rank_b, faction_b);
        let second = node(&mut e, rank_b, 0);
        let first = node(&mut e, rank_a, second);
        e.register_double(0x005d_8a70, move |_, _| first.into_ret());
        e.register(GET_EXTRA_LIST, |_, _| 0x2000.into_ret());
        e.register(0x0042_e800, |_, _| 0.into_ret());
        // Flag 4 at +0x34 of the faction.
        e.register(0x0047_d7e0, |e, a| {
            ((e.mem.u32(a[0] + 0x34) & 4) != 0).into_ret()
        });
        assert!(!e.call(0x008a_c6f0, &args![actor]).bool());
        e.mem.set_u32(faction_b + 0x34, 4);
        assert!(e.call(0x008a_c6f0, &args![actor]).bool());
        // The faction changes: expelled factions do not count; the changes'
        // own list (word +0xc of the extra) is walked too.
        let changes = e.mem.alloc(0x20);
        e.register_double(0x0042_e800, move |_, _| changes.into_ret());
        e.register(0x0043_7080, |_, _| 0.into_ret());
        let faction_c = e.mem.alloc(0x40);
        let rank_c = e.mem.alloc(8);
        e.mem.set_u32(rank_c, faction_c);
        e.mem.set_u32(faction_b + 0x34, 0);
        e.mem.set_u32(faction_c + 0x34, 4);
        let change_node = node(&mut e, rank_c, 0);
        e.mem.set_u32(changes + 0xc, change_node);
        assert!(e.call(0x008a_c6f0, &args![actor]).bool());
        // All expelled: false.
        e.register(0x0043_7080, |_, _| 1.into_ret());
        e.mem.set_u32(faction_b + 0x34, 4);
        assert!(!e.call(0x008a_c6f0, &args![actor]).bool());
    }

    /// Makes the fake function behind a vtable slot return the float `value`
    /// in `ST0`.
    fn answer_float(e: &mut Engine, at: u32, slot: u32, value: f32) {
        e.register_double(target(at, slot), move |_, _| value.into_ret());
    }

    /// A table at `at` whose slot `8` is a fake function returning `value`
    /// (the table at `actor + 0xa4`).
    fn actor_value_table(e: &mut Engine, actor: Ptr<Actor>, value: u32) {
        let table = 0x0906_0000;
        make_vtable(e, table, &[8]);
        answer(e, table, 8, value);
        e.mem.set_u32(actor.addr() + 0xa4, table);
    }

    #[test]
    fn process_object_forwarders() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        e.register(0x004f_b070, |_, a| (a[0] + 1).into_ret());
        // No object: nothing is called.
        assert_eq!(e.call(0x008a_c810, &args![actor]).u32(), 0);
        e.call(0x008a_c850, &args![actor, 9u32]);
        let object = new_object(&mut e);
        answer(&mut e, ACTOR_VT, 0x428, object.addr());
        assert_eq!(e.call(0x008a_c810, &args![actor]).u32(), object.addr() + 1);
        e.call(0x008a_c850, &args![actor, 9u32]);
        assert_eq!(e.mem.u32(object.addr() + 0x80), 9);
        e.call(0x008a_c890, &args![object, 4u32]);
        assert_eq!(e.mem.u32(object.addr() + 0x80), 4);
    }

    #[test]
    fn height_from_the_two_points_and_the_process_store() {
        let mut e = engine();
        e.set_global(HEIGHT_FACTOR, 0.75f64);
        let (actor, proc) = new_actor_with_process(&mut e);
        let upper = new_object(&mut e);
        let lower = new_object(&mut e);
        e.mem.set_f32(upper.addr() + 8, 10.0);
        e.mem.set_f32(lower.addr() + 8, 2.0);
        answer(&mut e, ACTOR_VT, 0x390, 0);
        answer(&mut e, ACTOR_VT, 0x1dc, upper.addr());
        answer(&mut e, ACTOR_VT, 0x1d8, lower.addr());
        // The process refuses the store.
        e.register(0x0088_4920, |_, _| 0.into_ret());
        let this: Ptr = Ptr::new(actor.addr() + 0xa8);
        assert_eq!(e.call(0x008a_c8b0, &args![this]).f32(), 6.0);
        // The process takes it: the object at +0x2c gets the float.
        let stored = new_object(&mut e);
        e.mem.set_u32(proc.addr() + 0x2c, stored.addr());
        e.register(0x0088_4920, |_, _| 1.into_ret());
        e.register(0x0088_48c0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x008a_c8b0, &args![this]).f32(), 6.0);
        assert_eq!(e.mem.f32(stored.addr() + 0x2c), 6.0);
        let log = calls(&mut e);
        assert_eq!(called(&log, 0x0088_48c0), vec![vec![stored.addr(), 0x200]]);
    }

    #[test]
    fn height_with_a_controller_is_player_value_times_scale() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        e.set_global(PLAYER_CHARACTER, 0x7777u32);
        answer(&mut e, ACTOR_VT, 0x390, 1);
        e.register(0x0070_c3b0, |_, a| {
            assert_eq!(a[0], 0x7777);
            2.0f64.into_ret()
        });
        e.register(0x0056_7400, |_, _| 1.5f32.into_ret());
        let this: Ptr = Ptr::new(actor.addr() + 0xa8);
        assert_eq!(e.call(0x008a_c8b0, &args![this]).f32(), 3.0);
    }

    #[test]
    fn stores_in_the_process_state_object() {
        let mut e = engine();
        let object = new_object(&mut e);
        let holder: Ptr = Ptr::new(e.mem.alloc(0x40));
        e.register(0x0088_48c0, |_, _| Ret::default());
        // Without the object nothing happens.
        e.call(0x008a_c990, &args![holder, 1.5f32]);
        e.call(0x008a_cb70, &args![holder, 1.5f32]);
        e.mem.set_u32(holder.addr() + 0x2c, object.addr());
        e.call_log = Some(vec![]);
        e.call(0x008a_c990, &args![holder, 1.5f32]);
        e.call(0x008a_cb70, &args![holder, 2.5f32]);
        assert_eq!(e.mem.f32(object.addr() + 0x2c), 1.5);
        assert_eq!(e.mem.f32(object.addr() + 0x10), 2.5);
        let log = calls(&mut e);
        assert_eq!(
            called(&log, 0x0088_48c0),
            vec![vec![object.addr(), 0x200], vec![object.addr(), 8]]
        );
    }

    #[test]
    fn process_mark_needs_a_process_that_passes_the_test() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        e.register(0x0088_4920, |_, _| 1.into_ret());
        e.register(0x0088_4940, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x008a_cba0, &args![actor]);
        assert_eq!(called(&calls(&mut e), 0x0088_4940).len(), 0);
        let (actor, proc) = new_actor_with_process(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x008a_cba0, &args![actor]);
        assert_eq!(
            called(&calls(&mut e), 0x0088_4940),
            vec![vec![proc.addr(), 8]]
        );
        e.register(0x0088_4920, |_, _| 0.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x008a_cba0, &args![actor]);
        assert_eq!(called(&calls(&mut e), 0x0088_4940).len(), 0);
    }

    /// Doubles for `008ac9d0`'s callees; `00645380` records its words.
    fn stub_weapon_value(e: &mut Engine) {
        e.register(0x0044_ddc0, |_, a| (a[0] + 0x100).into_ret());
        e.register(0x004b_cdb0, |_, _| 50.0f32.into_ret());
        e.register(0x0044_59e0, |_, _| Ret::default());
        e.register(0x0088_4920, |_, _| 0.into_ret());
        e.register(0x0064_5380, |_, _| 4.0f32.into_ret());
    }

    #[test]
    fn weapon_value_of_the_process_item() {
        let mut e = engine();
        e.set_global(ITEM_HEALTH_DIVISOR, 100.0f64);
        let (actor, proc) = new_actor_with_process(&mut e);
        let item = new_object(&mut e);
        stub_weapon_value(&mut e);
        e.register(0x0045_cd60, |_, _| 1.into_ret());
        answer(&mut e, PROCESS_VT, 0x148, item.addr());
        e.call_log = Some(vec![]);
        let this: Ptr = Ptr::new(actor.addr() + 0xa8);
        assert_eq!(e.call(0x008a_c9d0, &args![this]).f32(), 4.0);
        let log = calls(&mut e);
        // base = actor + 0xa4, the item word, health 0.5, the item, -1 and
        // floats 0.
        assert_eq!(
            called(&log, 0x0064_5380),
            vec![vec![
                actor.addr() + 0xa4,
                item.addr() + 0x100,
                0.5f32.to_bits(),
                0,
                item.addr(),
                0,
                0,
                0xffff_ffff,
                0,
                0,
                0,
                0,
                0
            ]]
        );
        // The item stays; nothing is released.
        assert_eq!(called(&log, 0x0044_59e0).len(), 0);
        // With a process that passes the test the result is stored.
        let stored = new_object(&mut e);
        e.mem.set_u32(proc.addr() + 0x2c, stored.addr());
        e.register(0x0088_4920, |_, _| 1.into_ret());
        e.register(0x0088_48c0, |_, _| Ret::default());
        e.call(0x008a_c9d0, &args![this]);
        assert_eq!(e.mem.f32(stored.addr() + 0x10), 4.0);
    }

    #[test]
    fn weapon_value_of_the_worn_weapon_releases_the_item() {
        let mut e = engine();
        e.set_global(ITEM_HEALTH_DIVISOR, 100.0f64);
        let (actor, _) = new_actor_with_process(&mut e);
        let item = new_object(&mut e);
        stub_weapon_value(&mut e);
        e.register(0x0045_cd60, |_, _| 2.into_ret());
        e.register(0x004b_f220, |_, a| (a[0] + 0x30).into_ret());
        e.register_double(0x004c_8c10, move |_, _| item.addr().into_ret());
        e.call_log = Some(vec![]);
        let this: Ptr = Ptr::new(actor.addr() + 0xa8);
        assert_eq!(e.call(0x008a_c9d0, &args![this]).f32(), 4.0);
        let log = calls(&mut e);
        assert_eq!(called(&log, 0x0044_59e0), vec![vec![item.addr(), 1]]);
        assert_eq!(
            called(&log, 0x004c_8c10),
            vec![vec![actor.addr() + 0x30, 5, 0]]
        );
        let words = &called(&log, 0x0064_5380)[0];
        assert_eq!(words[1], item.addr() + 0x100);
        assert_eq!(words[4], 0);
        // No worn weapon: health stays 1.0 and the item word 0.
        e.register(0x004c_8c10, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x008a_c9d0, &args![this]);
        let log = calls(&mut e);
        let words = &called(&log, 0x0064_5380)[0];
        assert_eq!(words[1], 0);
        assert_eq!(words[2], 1.0f32.to_bits());
        assert_eq!(called(&log, 0x0044_59e0).len(), 0);
    }

    #[test]
    fn combat_strength() {
        let mut e = engine();
        e.set_global(ZERO_DOUBLE, 0.0f64);
        e.set_global(THIRTY, 30.0f64);
        e.set_global(COMBAT_STRENGTH_EXPONENT, 0.99f32);
        let actor = new_actor(&mut e);
        actor_value_table(&mut e, actor, 20);
        answer_float(&mut e, ACTOR_VT, 0x440, 15.0);
        e.register(0x008b_e600, |_, _| 10.0f32.into_ret());
        e.register(0x0040_ebd0, |_, _| 0.5f32.into_ret());
        e.call_log = Some(vec![]);
        // Negative damage is replaced by the weapon's damage per second.
        let result = e.call(0x008a_cbe0, &args![actor, -1.0f32]).f32();
        assert_eq!(result, 400.0);
        let log = calls(&mut e);
        assert_eq!(
            called(&log, 0x0040_ebd0),
            vec![vec![0.5f32.to_bits(), 0.99f32.to_bits()]]
        );
        // A given damage is used as is.
        e.call_log = Some(vec![]);
        let result = e.call(0x008a_cbe0, &args![actor, 2.0f32]).f32();
        assert_eq!(result, 80.0);
        assert_eq!(called(&calls(&mut e), 0x008b_e600).len(), 0);
    }

    /// Doubles for every callee of `008acc80` outside this file.
    fn stub_refresh(e: &mut Engine) {
        stub(
            e,
            &[
                0x00c9_bdf0,
                0x008b_0b00,
                0x0093_06d0,
                0x008d_6f30,
                0x0045_43c0,
                0x00c7_0c90,
                0x00c7_a8d0,
                0x0049_3bd0,
                0x004e_af60,
                0x0095_0a60,
                0x00c8_0700,
                0x0045_0f90,
            ],
        );
    }

    #[test]
    fn refresh_does_nothing_without_the_object() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        stub_refresh(&mut e);
        e.register(0x0043_fcd0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x008a_cc80, &args![actor]);
        assert_eq!(calls(&mut e).len(), 2);
    }

    #[test]
    fn refresh_updates_the_controller_ragdoll_and_animation() {
        let mut e = engine();
        let (actor, _) = new_actor_with_process(&mut e);
        stub_refresh(&mut e);
        let object = new_object(&mut e);
        let face = new_object(&mut e);
        let controller = new_object(&mut e);
        let cell = new_object(&mut e);
        let ragdoll = new_object(&mut e);
        let animation = new_object(&mut e);
        e.register_double(0x0043_fcd0, move |_, _| object.addr().into_ret());
        e.register_double(0x0093_06d0, move |_, _| controller.addr().into_ret());
        e.register_double(0x008d_6f30, move |_, _| cell.addr().into_ret());
        e.register(0x0045_43c0, |_, a| (a[0] + 1).into_ret());
        e.set(actor, Actor::pRagdollController, ragdoll);
        e.set(actor, Actor::eCriticalStage, 1);
        answer(&mut e, PROCESS_VT, 0x144, face.addr());
        answer(&mut e, ACTOR_VT, 0x1e4, animation.addr());
        answer(&mut e, ACTOR_VT, 0x1d0, 0x5555);
        e.set_global(PLAYER_CHARACTER, 0x7777u32);
        e.call_log = Some(vec![]);
        e.call(0x008a_cc80, &args![actor]);
        let log = calls(&mut e);
        assert_eq!(
            called(&log, 0x00c9_bdf0),
            vec![vec![object.addr(), 1.0f32.to_bits()]]
        );
        assert_eq!(
            called(&log, target(OBJECT_VT, 0xd8)),
            vec![vec![face.addr(), 0, 0]]
        );
        assert_eq!(called(&log, 0x008b_0b00), vec![vec![actor.addr(), 1]]);
        assert_eq!(
            called(&log, 0x00c7_0c90),
            vec![vec![controller.addr(), cell.addr() + 1]]
        );
        assert_eq!(called(&log, 0x00c7_a8d0), vec![vec![ragdoll.addr()]]);
        // Not the player: one scene graph update; stage 1 passes the node.
        assert_eq!(called(&log, 0x0049_3bd0), vec![vec![animation.addr()]]);
        assert_eq!(called(&log, 0x00c8_0700), vec![vec![0x5555]]);
        assert_eq!(called(&log, 0x0045_0f90).len(), 0);
    }

    #[test]
    fn refresh_of_the_player_and_the_other_critical_stages() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        stub_refresh(&mut e);
        let object = new_object(&mut e);
        let animation = new_object(&mut e);
        e.register_double(0x0043_fcd0, move |_, _| object.addr().into_ret());
        answer(&mut e, ACTOR_VT, 0x1e4, animation.addr());
        answer(&mut e, ACTOR_VT, 0x1d0, 0x5555);
        e.set_global(PLAYER_CHARACTER, actor.addr());
        e.register(0x004e_af60, |_, _| 1.into_ret());
        e.register(0x0095_0a60, |_, a| (a[0] + a[1]).into_ret());
        // Stage 2: virtual +0x1c4, the node, then 00450f90(object, 0).
        e.set(actor, Actor::eCriticalStage, 2);
        e.call_log = Some(vec![]);
        e.call(0x008a_cc80, &args![actor]);
        let log = calls(&mut e);
        assert_eq!(
            called(&log, 0x0049_3bd0),
            vec![vec![animation.addr()], vec![actor.addr() + 1]]
        );
        assert_eq!(called(&log, 0x0095_0a60), vec![vec![actor.addr(), 1]]);
        assert_eq!(called(&log, target(ACTOR_VT, 0x1c4)).len(), 1);
        assert_eq!(called(&log, 0x00c8_0700), vec![vec![0x5555]]);
        assert_eq!(called(&log, 0x0045_0f90), vec![vec![object.addr(), 0]]);
        // Stage 4: no node, but the same virtual method and call.
        e.set(actor, Actor::eCriticalStage, 4);
        e.call_log = Some(vec![]);
        e.call(0x008a_cc80, &args![actor]);
        let log = calls(&mut e);
        assert_eq!(called(&log, target(ACTOR_VT, 0x1c4)).len(), 1);
        assert_eq!(called(&log, 0x00c8_0700).len(), 0);
        assert_eq!(called(&log, 0x0045_0f90), vec![vec![object.addr(), 0]]);
        // Another stage: nothing more.
        e.set(actor, Actor::eCriticalStage, 3);
        e.call_log = Some(vec![]);
        e.call(0x008a_cc80, &args![actor]);
        let log = calls(&mut e);
        assert_eq!(called(&log, 0x00c8_0700).len(), 0);
        assert_eq!(called(&log, 0x0045_0f90).len(), 0);
    }

    #[test]
    fn life_state_toggles_go_through_set_life_state() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        e.register(0x008a_1800, |_, _| Ret::default());
        e.register(0x0088_4c60, |_, _| Ret::default());
        e.register(0x004f_8960, |_, _| 0.into_ret());
        for (addr, state, extra) in [(0x008a_ce10u32, 3u32, 0), (0x008a_ce50, 5, 1)] {
            e.call_log = Some(vec![]);
            e.call(addr, &args![actor, 1u32]);
            let log = calls(&mut e);
            assert_eq!(called(&log, 0x008a_1800), vec![vec![actor.addr(), state]]);
            assert_eq!(called(&log, 0x0088_4c60).len(), extra);
            // Clearing: only when the other state matches.
            e.call_log = Some(vec![]);
            e.call(addr, &args![actor, 0u32]);
            assert_eq!(called(&calls(&mut e), 0x008a_1800).len(), 0);
            e.register_double(0x004f_8960, move |_, _| state.into_ret());
            e.call_log = Some(vec![]);
            e.call(addr, &args![actor, 0u32]);
            let log = calls(&mut e);
            assert_eq!(called(&log, 0x008a_1800), vec![vec![actor.addr(), 0]]);
            assert_eq!(called(&log, 0x0088_4c60).len(), 0);
            e.register(0x004f_8960, |_, _| 0.into_ret());
        }
    }

    #[test]
    fn ghost_flag_lives_in_the_state_object_flag_word() {
        let mut e = engine();
        let (actor, proc) = new_actor_with_process(&mut e);
        let state = new_object(&mut e);
        e.register(GET_EXTRA_LIST, |_, a| (a[0] + 0x10).into_ret());
        e.register(0x0041_8a80, |_, a| (a[0] as u8).into_ret());
        // Without the state object the flag reads clear.
        assert!(!e.call(0x008a_ced0, &args![proc, 0x1000_0000u32]).bool());
        e.mem.set_u32(proc.addr() + 0x2c, state.addr());
        e.mem.set_u32(state.addr() + 0x44, 0x1000_0004);
        assert!(e.call(0x008a_ced0, &args![proc, 0x1000_0000u32]).bool());
        assert!(!e.call(0x008a_ced0, &args![proc, 0x2u32]).bool());
        assert_eq!(e.call(0x008a_ce90, &args![actor]).u8(), 1);
        // 008acfa0 and 008acf70 clear and set.
        e.call(0x008a_cf70, &args![proc, 0x1000_0000u32, 0u32]);
        assert_eq!(e.mem.u32(state.addr() + 0x44), 4);
        assert_eq!(e.call(0x008a_ce90, &args![actor]).u8(), 0);
        e.call(0x008a_cfa0, &args![state, 0x30u32, 1u32]);
        assert_eq!(e.mem.u32(state.addr() + 0x44), 0x34);
        // Without a process the extra data list decides.
        let bare = new_actor(&mut e);
        assert_eq!(
            e.call(0x008a_ce90, &args![bare]).u8(),
            (bare.addr() + 0x10) as u8
        );
    }

    #[test]
    fn set_ghost_updates_the_list_the_process_and_calls_the_slot() {
        let mut e = engine();
        let (actor, proc) = new_actor_with_process(&mut e);
        let state = new_object(&mut e);
        e.mem.set_u32(proc.addr() + 0x2c, state.addr());
        e.register(GET_EXTRA_LIST, |_, a| (a[0] + 0x10).into_ret());
        e.register(0x0041_a960, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x008a_cf10, &args![actor, 1u32]);
        let log = calls(&mut e);
        assert_eq!(
            called(&log, 0x0041_a960),
            vec![vec![actor.addr() + 0x10, 1]]
        );
        assert_eq!(e.mem.u32(state.addr() + 0x44), 0x1000_0000);
        assert_eq!(
            called(&log, target(ACTOR_VT, 0x48)),
            vec![vec![actor.addr(), 0x8000_0000]]
        );
        e.call(0x008a_cf10, &args![actor, 0u32]);
        assert_eq!(e.mem.u32(state.addr() + 0x44), 0);
        // No process: only the list and the slot.
        let bare = new_actor(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x008a_cf10, &args![bare, 1u32]);
        assert_eq!(called(&calls(&mut e), 0x0041_a960).len(), 1);
    }

    #[test]
    fn small_stores_and_forwarders() {
        let mut e = engine();
        let object: Ptr = Ptr::new(e.mem.alloc(0xc0));
        e.call(0x008a_d8f0, &args![object, 0x1234_5601u32]);
        assert_eq!(e.mem.u8(object.addr() + 0x9b), 1);
        e.call(0x008a_dc20, &args![object, 7u32, 8u32, 9u32]);
        assert_eq!(
            [
                e.mem.u32(object.addr() + 0x94),
                e.mem.u32(object.addr() + 0x98),
                e.mem.u32(object.addr() + 0x9c)
            ],
            [7, 8, 9]
        );
        // The face animation data comes from the process (with the actor).
        let bare = new_actor(&mut e);
        assert_eq!(e.call(0x008a_dcb0, &args![bare]).u32(), 0);
        let (actor, _) = new_actor_with_process(&mut e);
        e.register_double(target(PROCESS_VT, 0x144), |_, a| a[1].into_ret());
        assert_eq!(e.call(0x008a_dcb0, &args![actor]).u32(), actor.addr());
        // Clearing the weapon needs a process.
        e.call_log = Some(vec![]);
        e.call(0x008a_dc50, &args![bare]);
        assert_eq!(calls(&mut e).len(), 2);
        e.call_log = Some(vec![]);
        e.call(0x008a_dc50, &args![actor]);
        let log = calls(&mut e);
        assert_eq!(
            called(&log, target(PROCESS_VT, 0x160)),
            vec![vec![e.mem.u32(actor.addr() + 0x68), 0, 0, 0]]
        );
        assert_eq!(called(&log, target(PROCESS_VT, 0x168)).len(), 1);
    }

    /// Doubles for the callees of `008acff0`; `00837550` copies the record
    /// it is given to the returned block.
    fn collision_sound_doubles(e: &mut Engine) -> u32 {
        let sink = e.mem.alloc(0x40);
        e.register(0x0062_40d0, |_, _| Ret::default());
        e.register(0x0044_ddc0, |_, a| (a[0] + 1).into_ret());
        e.register(0x0044_6390, |_, a| (a[0] + 2).into_ret());
        e.register(0x0083_7520, |_, _| Ret::default());
        e.register_double(0x0083_7550, move |e, a| {
            for i in 0..10 {
                let word = e.mem.u32(a[0] + 4 * i);
                e.mem.set_u32(sink + 4 * i, word);
            }
            Ret::default()
        });
        e.set_global(COLLISION_SOUND_LOUDNESS, 100.0f32);
        e.set_global(COLLISION_SOUND_HALF, 0.5f32);
        sink
    }

    #[test]
    fn collision_sound_of_the_actor_itself() {
        let mut e = engine();
        let sink = collision_sound_doubles(&mut e);
        let (actor, _) = new_actor_with_process(&mut e);
        let item = new_object(&mut e);
        answer(&mut e, PROCESS_VT, 0x148, item.addr());
        e.call_log = Some(vec![]);
        e.call(
            0x008a_cff0,
            &args![actor, 1.0f32, 2.0f32, 3.0f32, 0u32, 0x10u32, 5u32],
        );
        let log = calls(&mut e);
        // 00837520(actor, 0, 0, actor, id, -1, -1, 1, 0) with the item id.
        assert_eq!(
            called(&log, 0x0083_7520),
            vec![vec![
                actor.addr(),
                0,
                0,
                actor.addr(),
                item.addr() + 3,
                0xffff_ffff,
                0xffff_ffff,
                1,
                0
            ]]
        );
        let record: Vec<u32> = (0..9).map(|i| e.mem.u32(sink + 4 * i)).collect();
        assert_eq!(record[0], 1.0f32.to_bits());
        assert_eq!(record[1], 2.0f32.to_bits());
        assert_eq!(record[2], 3.0f32.to_bits());
        assert_eq!(record[3], 100.0f32.to_bits());
        assert_eq!(record[4], 0.5f32.to_bits());
        // Bytes 0x24 and the kind, then actor + index, 0 and the index.
        assert_eq!(record[5], 0x24 | (5 << 8));
        assert_eq!(record[6], actor.addr() + 0x10);
        assert_eq!(record[7], 0);
        assert_eq!(record[8], 0x10);
        // Kind 6 skips the extra call but still plays the sound.
        e.call_log = Some(vec![]);
        e.call(
            0x008a_cff0,
            &args![actor, 1.0f32, 2.0f32, 3.0f32, 0u32, 0x10u32, 6u32],
        );
        let log = calls(&mut e);
        assert_eq!(called(&log, 0x0083_7520).len(), 0);
        assert_eq!(called(&log, 0x0083_7550).len(), 1);
        // Without an item the id is -1.
        answer(&mut e, PROCESS_VT, 0x148, 0);
        e.call_log = Some(vec![]);
        e.call(
            0x008a_cff0,
            &args![actor, 1.0f32, 2.0f32, 3.0f32, 0u32, 0x10u32, 5u32],
        );
        let log = calls(&mut e);
        assert_eq!(called(&log, 0x0083_7520)[0][4], 0xffff_ffff);
    }

    #[test]
    fn collision_sound_of_a_hit_object() {
        let mut e = engine();
        let _ = collision_sound_doubles(&mut e);
        let actor = new_actor(&mut e);
        let hit = new_object(&mut e);
        e.register_double(target(OBJECT_VT, 0x100), |_, _| 1.into_ret());
        e.call_log = Some(vec![]);
        // A hit that answers its virtual +0x100 gets the sound call alone.
        e.call(
            0x008a_cff0,
            &args![actor, 0u32, 0u32, 0u32, hit, 3u32, 5u32],
        );
        let log = calls(&mut e);
        assert_eq!(
            called(&log, 0x0083_7520),
            vec![vec![
                actor.addr(),
                0,
                0,
                hit.addr(),
                0xffff_ffff,
                0xffff_ffff,
                0xffff_ffff,
                0,
                0
            ]]
        );
        assert_eq!(called(&log, 0x0083_7550).len(), 0);
    }

    #[test]
    fn collision_sound_takes_index_and_kind_from_the_collision_object() {
        let mut e = engine();
        let sink = collision_sound_doubles(&mut e);
        let actor = new_actor(&mut e);
        let hit = new_object(&mut e);
        e.register_double(target(OBJECT_VT, 0x100), |_, _| 0.into_ret());
        e.register_double(target(OBJECT_VT, 0x1d0), |_, _| 0x7001u32.into_ret());
        // Node 0x7001 -> collision object 0x7002 -> index 0x30 -> kind 4.
        e.register(0x004b_5260, |_, a| (a[0] + 1).into_ret());
        e.register(0x006f_a820, |_, _| 0x30.into_ret());
        e.register(0x004a_e6a0, |_, a| (a[0] + 1).into_ret());
        e.register(0x0062_05a0, |_, a| (a[0] + 3).into_ret());
        e.call(
            0x008a_cff0,
            &args![actor, 0u32, 0u32, 0u32, hit, 0u32, 0u32],
        );
        assert_eq!(e.mem.u32(sink + 0x18), actor.addr() + 0x30);
        assert_eq!(e.mem.u32(sink + 0x20), 0x30);
        assert_eq!(e.mem.u8(sink + 0x15), 0x34);
    }

    /// The pointer the list doubles keep nodes in: a node is two words,
    /// the item and the next node.
    fn list_doubles(e: &mut Engine) {
        e.register(GET_CURRENT_PACKAGE, |_, _| Ret::default());
        e.register(0x0096_a2d0, |e, a| {
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, 0);
            Ret::default()
        });
        e.register(0x005a_e3d0, |e, a| {
            let item = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], item);
            Ret::default()
        });
        e.register(0x0068_15c0, |_, a| a[0].into_ret());
        e.register(0x0072_6070, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(0x005f_65d0, |e, a| {
            let wanted = e.mem.u32(a[1]);
            let mut node = a[0];
            while node != 0 {
                if e.mem.u32(node) == wanted {
                    return 1.into_ret();
                }
                node = e.mem.u32(node + 4);
            }
            0.into_ret()
        });
        e.register(0x0090_5820, |e, a| {
            let item = e.mem.u32(a[1]);
            let added = e.mem.alloc(8);
            e.mem.set_u32(added, item);
            let after = e.mem.u32(a[0] + 4);
            e.mem.set_u32(added + 4, after);
            e.mem.set_u32(a[0] + 4, added);
            Ret::default()
        });
        e.register(GET_EXTRA_LIST, |_, a| (a[0] + 0x10).into_ret());
        e.register(0x0042_2700, |_, _| Ret::default());
        e.register(0x004e_4470, |_, _| 0.25f64.into_ret());
        e.register(0x004e_44b0, |_, _| 0.5f64.into_ret());
        stub(
            e,
            &[
                0x0041_6870,
                0x0094_df60,
                0x0044_0da0,
                0x008b_0190,
                0x0056_6950,
                0x008c_7aa0,
                0x0057_5830,
                0x0057_57d0,
                0x0057_5770,
                0x0057_3800,
                0x008b_3ad0,
                0x0088_1680,
                0x0045_11e0,
                0x0087_b650,
                0x0087_b190,
            ],
        );
        e.register(0x0045_37b0, |_, _| 0x5eed.into_ret());
        e.register(0x008b_e1b0, |_, _| 1.5f32.into_ret());
        e.set_global(PLAYER_CHARACTER, 0u32);
    }

    /// An actor with process, 3D and the usual slot answers for the group
    /// move.
    fn group_member(e: &mut Engine) -> Ptr<Actor> {
        let (actor, proc) = new_actor_with_process(e);
        e.mem.set_u32(proc.addr(), PROCESS_VT);
        actor
    }

    #[test]
    fn group_move_places_the_leader_alone() {
        let mut e = engine();
        list_doubles(&mut e);
        let player = e.mem.alloc(0x300);
        e.set_global(PLAYER_CHARACTER, player);
        e.set_global(ONE, 1.0f64);
        e.set_global(FLOAT_MAX, 3.0e38f32);
        let leader = group_member(&mut e);
        let position: Ptr = Ptr::new(e.mem.alloc(0x10));
        for (i, v) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.mem.set_f32(position.addr() + 4 * i as u32, *v);
        }
        e.call_log = Some(vec![]);
        e.call(
            0x008a_d1c0,
            &args![leader, position, 0.75f32, 0u32, 0x99u32, 0u32, 0u32],
        );
        let log = calls(&mut e);
        // The direction is (-cos, -sin, 0) of the angle.
        let direction = called(&log, 0x0041_6870);
        assert_eq!(
            direction[0][1..],
            [
                (-0.5f64 as f32).to_bits(),
                (-0.25f64 as f32).to_bits(),
                0.0f32.to_bits()
            ]
        );
        assert_eq!(called(&log, 0x004e_4470), vec![vec![0.75f32.to_bits()]]);
        // No cell: the way with virtual +0x2a8, the location, the huge range.
        assert_eq!(called(&log, target(ACTOR_VT, 0x2a8)).len(), 1);
        assert_eq!(called(&log, 0x0057_5830).len(), 1);
        assert_eq!(
            called(&log, 0x0057_5770),
            vec![vec![leader.addr(), 3.0e38f32.to_bits()]]
        );
        assert_eq!(
            called(&log, 0x0057_3800),
            vec![vec![leader.addr(), 0, 0x99]]
        );
        // The leader is not stopped.
        assert_eq!(called(&log, 0x008b_3ad0).len(), 0);
        assert_eq!(called(&log, 0x0088_1680).len(), 0);
        assert_eq!(called(&log, 0x0046_ffb0).len(), 1);
    }

    #[test]
    fn group_move_skips_unavailable_and_visible_actors() {
        let mut e = engine();
        list_doubles(&mut e);
        let player = e.mem.alloc(0x300);
        e.set_global(PLAYER_CHARACTER, player);
        e.set_global(ONE, 1.0f64);
        let leader = group_member(&mut e);
        let position: Ptr = Ptr::new(e.mem.alloc(0x10));
        // The leader is busy (virtual +0x22c): nothing is placed.
        e.register_double(target(ACTOR_VT, 0x22c), |_, _| 1.into_ret());
        e.call_log = Some(vec![]);
        e.call(
            0x008a_d1c0,
            &args![leader, position, 0.0f32, 0u32, 0u32, 0u32, 0u32],
        );
        assert_eq!(called(&calls(&mut e), 0x0057_3800).len(), 0);
        // The player's byte +0x204 stops everybody.
        e.register_double(target(ACTOR_VT, 0x22c), |_, _| 0.into_ret());
        e.mem.set_u8(player + 0x204, 1);
        e.call_log = Some(vec![]);
        e.call(
            0x008a_d1c0,
            &args![leader, position, 0.0f32, 0u32, 0u32, 0u32, 0u32],
        );
        assert_eq!(called(&calls(&mut e), 0x0057_3800).len(), 0);
        e.mem.set_u8(player + 0x204, 0);
        // A visible leader with 3D is skipped... no: the leader always goes.
        e.register_double(target(ACTOR_VT, 0x1d0), |_, _| 0x7001u32.into_ret());
        e.register_double(0x008b_0190, |_, _| 1.into_ret());
        e.call_log = Some(vec![]);
        e.call(
            0x008a_d1c0,
            &args![leader, position, 0.0f32, 0u32, 0u32, 0u32, 0u32],
        );
        assert_eq!(called(&calls(&mut e), 0x0057_3800).len(), 1);
        // A waiting actor (forced update and value 1.0) is skipped.
        e.register_double(0x0056_6950, |_, _| 1.into_ret());
        e.register_double(0x008c_1740, |_, _| 1.0f64.into_ret());
        e.call_log = Some(vec![]);
        e.call(
            0x008a_d1c0,
            &args![leader, position, 0.0f32, 0u32, 0u32, 0u32, 0u32],
        );
        let log = calls(&mut e);
        assert_eq!(
            called(&log, 0x008c_1740),
            vec![vec![leader.addr(), WAITING]]
        );
        assert_eq!(called(&log, 0x0057_3800).len(), 0);
        e.register_double(0x008c_1740, |_, _| 0.0f64.into_ret());
        e.call_log = Some(vec![]);
        e.call(
            0x008a_d1c0,
            &args![leader, position, 0.0f32, 0u32, 0u32, 0u32, 0u32],
        );
        assert_eq!(called(&calls(&mut e), 0x0057_3800).len(), 1);
    }

    #[test]
    fn group_move_spaces_the_follower_and_asks_the_pathing() {
        let mut e = engine();
        list_doubles(&mut e);
        let player = e.mem.alloc(0x300);
        e.set_global(PLAYER_CHARACTER, player);
        e.set_global(ONE, 1.0f64);
        e.set_global(TES_SINGLETON, 0x6000u32);
        e.register(SETTING_VALUE_POINTER, |_, a| (a[0] + 4).into_ret());
        e.set_global(SETTING_FOLLOWER_SPACING + 4, 2.0f32);
        let leader = group_member(&mut e);
        let follower = group_member(&mut e);
        // The leader's extra data names the follower in a one-node list.
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, follower.addr());
        let followers = e.mem.alloc(0x10);
        e.mem.set_u32(followers + 0xc, node);
        let leader_extra = leader.addr() + 0x10;
        e.register_double(0x0042_2700, move |_, a| {
            if a[0] == leader_extra {
                followers.into_ret()
            } else {
                Ret::default()
            }
        });
        // Cells are loaded; the pathing finds the spot (7, 8, 9).
        e.register(0x0045_11e0, |_, _| 1.into_ret());
        e.register(0x0045_bb20, |_, a| a[1].into_ret());
        e.register(0x0043_9e90, |_, a| a[1].into_ret());
        stub(
            &mut e,
            &[
                0x006e_7650,
                0x006e_2420,
                0x006e_29f0,
                0x006e_2b50,
                0x006d_1bc0,
            ],
        );
        e.register(0x006d_ce10, |_, a| a[0].into_ret());
        stub(&mut e, &[0x004f_f7e0, 0x006e_2620, 0x006e_7720]);
        e.register(0x006d_0900, |_, _| 1.into_ret());
        e.register(0x006e_7e70, |e, a| {
            for (i, v) in [7.0f32, 8.0, 9.0].iter().enumerate() {
                e.mem.set_f32(a[5] + 4 * i as u32, *v);
            }
            1.into_ret()
        });
        let placed = std::rc::Rc::new(std::cell::RefCell::new(Vec::<(u32, [f32; 3])>::new()));
        let seen = placed.clone();
        e.register_double(0x0057_5830, move |e, a| {
            seen.borrow_mut().push((
                a[0],
                [e.mem.f32(a[1]), e.mem.f32(a[1] + 4), e.mem.f32(a[1] + 8)],
            ));
            Ret::default()
        });
        let position: Ptr = Ptr::new(e.mem.alloc(0x10));
        for (i, v) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.mem.set_f32(position.addr() + 4 * i as u32, *v);
        }
        e.call_log = Some(vec![]);
        e.call(
            0x008a_d1c0,
            &args![leader, position, 0.0f32, 0x5000u32, 0x99u32, 1u32, 0u32],
        );
        let log = calls(&mut e);
        // Leader at the position; follower at the pathing's spot.
        assert_eq!(
            *placed.borrow(),
            vec![
                (leader.addr(), [1.0, 2.0, 3.0]),
                (follower.addr(), [7.0, 8.0, 9.0])
            ]
        );
        // Radius 1.5 each, spacing 2: the follower is asked for 2 + 1.5 + 1.5.
        let scaled = called(&log, 0x0045_bb20);
        assert_eq!(scaled.len(), 1);
        assert_eq!(scaled[0][2], 5.0f32.to_bits());
        let request = called(&log, 0x006e_7e70);
        assert_eq!(request[0][1], 0.0f32.to_bits());
        assert_eq!(request[0][2], 5.0f32.to_bits());
        assert_eq!(request[0][3], 1);
        // The request is flagged (byte +0x9b) before it is used.
        assert_eq!(called(&log, 0x006d_0900).len(), 1);
        // The follower is stopped, the leader not; both get the move.
        assert_eq!(called(&log, 0x008b_3ad0), vec![vec![follower.addr()]]);
        assert_eq!(called(&log, 0x0088_1680), vec![vec![follower.addr(), 0]]);
        assert_eq!(
            called(&log, 0x0057_3800),
            vec![
                vec![leader.addr(), 0x5000, 0x99],
                vec![follower.addr(), 0x5000, 0x99]
            ]
        );
        // Both are in a loaded cell: the angle and the zero range.
        assert_eq!(called(&log, 0x0057_57d0).len(), 2);
        assert_eq!(
            called(&log, 0x0057_5770),
            vec![
                vec![leader.addr(), 0.0f32.to_bits()],
                vec![follower.addr(), 0.0f32.to_bits()]
            ]
        );
    }

    #[test]
    fn group_move_takes_the_actor_off_its_furniture() {
        let mut e = engine();
        list_doubles(&mut e);
        let player = e.mem.alloc(0x300);
        e.set_global(PLAYER_CHARACTER, player);
        e.set_global(ONE, 1.0f64);
        e.set_global(PI_AS_FLOAT, 3.0f64);
        let leader = group_member(&mut e);
        let marker = new_object(&mut e);
        answer(&mut e, PROCESS_VT, 0x4bc, 1);
        answer(&mut e, PROCESS_VT, 0x4c4, 0x42);
        answer(&mut e, PROCESS_VT, 0x4c8, 0x6000);
        answer(&mut e, PROCESS_VT, 0x4d4, marker.addr());
        stub(
            &mut e,
            &[
                0x0056_8020,
                0x0049_74a0,
                0x0093_1d30,
                0x00c5_4550,
                0x0056_8ab0,
                0x0098_ddd0,
                0x0093_1fb0,
                GET_ANIMATION,
            ],
        );
        e.register(0x0050_94b0, |_, a| (a[0] + 1).into_ret());
        e.register(GET_SAVE_FORM, |_, a| (a[0] + 2).into_ret());
        e.register(0x0050_99c0, |_, _| 0.5f32.into_ret());
        answer_float(&mut e, ACTOR_VT, 0x2bc, 1.0);
        let position: Ptr = Ptr::new(e.mem.alloc(0x10));
        e.call_log = Some(vec![]);
        e.call(
            0x008a_d1c0,
            &args![leader, position, 0.0f32, 0u32, 0u32, 0u32, 0u32],
        );
        let log = calls(&mut e);
        assert_eq!(called(&log, 0x0056_8020), vec![vec![0x6000, 0x42, 0]]);
        assert_eq!(
            called(&log, target(PROCESS_VT, 0x4c0)),
            vec![vec![
                e.mem.u32(leader.addr() + 0x68),
                leader.addr(),
                0,
                0x6000,
                0x7f
            ]]
        );
        // The heading of the marker form is negated; the actor is turned by
        // its own heading plus pi.
        assert_eq!(
            called(&log, 0x0050_99c0),
            vec![vec![0x6002, marker.addr() + 1]]
        );
        assert_eq!(
            called(&log, 0x0093_1d30),
            vec![vec![leader.addr(), (-0.5f32).to_bits()]]
        );
        assert_eq!(
            called(&log, target(ACTOR_VT, 0x2c4)),
            vec![vec![leader.addr(), 4.0f32.to_bits()]]
        );
        assert_eq!(
            called(&log, 0x00c5_4550),
            vec![vec![marker.addr(), 0.0f32.to_bits()]]
        );
        assert_eq!(called(&log, 0x0056_8ab0), vec![vec![marker.addr(), 0]]);
        assert_eq!(
            called(&log, 0x0098_ddd0),
            vec![vec![marker.addr(), ZERO_POINT]]
        );
        assert_eq!(
            called(&log, target(PROCESS_VT, 0x540)),
            vec![vec![
                e.mem.u32(leader.addr() + 0x68),
                0,
                0x7f,
                marker.addr()
            ]]
        );
        assert_eq!(called(&log, 0x0093_1fb0), vec![vec![leader.addr(), 0]]);
    }

    #[test]
    fn move_to_a_reference_forwards_its_place() {
        let mut e = engine();
        list_doubles(&mut e);
        let player = e.mem.alloc(0x300);
        e.set_global(PLAYER_CHARACTER, player);
        e.set_global(ONE, 1.0f64);
        e.register(SETTING_VALUE_POINTER, |_, a| (a[0] + 4).into_ret());
        // The actor is busy, so only the setup of 008ad1c0 runs.
        e.register_double(target(ACTOR_VT, 0x22c), |_, _| 1.into_ret());
        let leader = group_member(&mut e);
        let reference = new_object(&mut e);
        let place = e.mem.alloc(0x10);
        e.mem.set_f32(place + 8, 0.75);
        e.register(0x0057_5d70, |_, _| 0x1111.into_ret());
        e.register(GET_PARENT_CELL, |_, _| 0x2222.into_ret());
        e.register_double(0x0043_0830, move |_, _| place.into_ret());
        e.register_double(target(OBJECT_VT, 0x1f4), |_, _| 0x3333.into_ret());
        e.call_log = Some(vec![]);
        // The position block is read before the loop: map one for it.
        let block = e.mem.alloc(0x10);
        e.register_double(target(OBJECT_VT, 0x1f4), move |_, _| block.into_ret());
        e.call(0x008a_d910, &args![leader, reference, 1u32, 0u32]);
        let log = calls(&mut e);
        assert_eq!(called(&log, 0x004e_4470), vec![vec![0.75f32.to_bits()]]);
    }

    #[test]
    fn search_package_setup() {
        let mut e = engine();
        let (actor, proc) = new_actor_with_process(&mut e);
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(0x009f_66b0, |_, a| a[0].into_ret());
        stub(
            &mut e,
            &[
                0x0067_0fc0,
                0x0082_6b40,
                0x0082_6b90,
                0x0067_f030,
                0x0067_f3c0,
                0x0067_1d30,
                0x0067_0b30,
                0x0067_a4a0,
                0x0067_ff70,
                0x0067_2fc0,
                0x0098_4f60,
                0x0068_00b0,
                0x0068_0110,
                0x0040_3550,
            ],
        );
        e.register(0x0067_f030, |_, a| a[0].into_ret());
        e.register(0x0067_ff70, |_, a| a[0].into_ret());
        e.register(0x0067_1d10, |_, a| (a[0] + 0x100).into_ret());
        e.register(SETTING_VALUE_POINTER, |_, a| (a[0] + 4).into_ret());
        e.set_global(SETTING_SEARCH_DISTANCE + 4, 12.75f32);
        e.register(FTOL, |_, a| {
            let value = f64::from_bits(a[0] as u64 | (a[1] as u64) << 32);
            (value as i32).into_ret()
        });
        e.register(0x0043_9090, |_, _| 1.into_ret());
        e.set_global(ZERO_POINT, 0u32);
        let package_store = e.mem.alloc(0x200);
        e.register_double(0x009f_66b0, move |_, _| package_store.into_ret());
        e.register(0x0067_1d10, |_, a| (a[0] + 0x100).into_ret());
        let _ = proc;
        e.call_log = Some(vec![]);
        e.call(0x008a_d970, &args![actor, 0x77u32, 4u32, 5u32, 6u32, 1u32]);
        let log = calls(&mut e);
        // Package type, the location and the target are set up.
        assert_eq!(called(&log, 0x0067_0fc0), vec![vec![package_store, 0x1e]]);
        assert_eq!(called(&log, 0x0067_f3c0).len(), 1);
        assert_eq!(called(&log, 0x0067_0b30).len(), 1);
        assert_eq!(
            called(&log, 0x0068_00b0),
            vec![vec![package_store + 0x100, 0]]
        );
        assert_eq!(
            called(&log, 0x0068_0110),
            vec![vec![package_store + 0x100, 0x77]]
        );
        // The count is the setting truncated.
        assert_eq!(
            called(&log, 0x0040_3550),
            vec![vec![package_store + 0x100, 12]]
        );
        assert_eq!(
            called(&log, target(ACTOR_VT, 0x2f4)),
            vec![vec![actor.addr(), package_store, 1, 1]]
        );
        // The nonzero place is stored in the package.
        assert_eq!(e.mem.u32(package_store + 0x94), 4);
        assert_eq!(e.mem.u32(package_store + 0x98), 5);
        assert_eq!(e.mem.u32(package_store + 0x9c), 6);
    }

    /// Doubles for the small getters `Actor::GetPickUpSoundName` calls.
    fn pick_up_doubles(e: &mut Engine) {
        e.register(0x0072_6070, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(0x0044_ddc0, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(0x0040_1170, |e, a| u32::from(e.mem.u8(a[0] + 4)).into_ret());
        e.register(0x009a_cce0, |e, a| e.mem.u32(a[0] + 0xc8).into_ret());
        e.register(0x0047_9430, |e, a| e.mem.u32(a[0] + 0x20).into_ret());
        e.register(DYNAMIC_CAST, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
    }

    /// A form: its component for `BGSPickupPutdownSounds` at +0x10 (the cast
    /// result), the type byte at +4 and the equip type at +0x20.
    fn pick_up_form(e: &mut Engine, kind: u8, equip: u32) -> Ptr {
        let form = new_object(e);
        e.mem.set_u8(form.addr() + 4, kind);
        e.mem.set_u32(form.addr() + 0x20, equip);
        form
    }

    #[test]
    fn pick_up_sound_names_by_kind_and_direction() {
        let mut e = engine();
        pick_up_doubles(&mut e);
        let actor = new_actor(&mut e);
        let name = |e: &mut Engine, form: Ptr, up: u32, flag: u32| {
            e.call(0x008a_dcf0, &args![actor, form, up, flag]).u32()
        };
        assert_eq!(name(&mut e, Ptr::new(0), 1, 0), 0);
        // Weapons by equip type; anything else generic.
        for (equip, up_name, down_name) in [
            (0, SOUND_GUNS_BIG_UP, SOUND_GUNS_BIG_DOWN),
            (2, SOUND_GUNS_SMALL_UP, SOUND_GUNS_SMALL_DOWN),
            (3, SOUND_MELEE_UP, SOUND_MELEE_DOWN),
            (1, SOUND_GENERIC_UP, SOUND_GENERIC_DOWN),
        ] {
            let weapon = pick_up_form(&mut e, 0x28, equip);
            assert_eq!(name(&mut e, weapon, 1, 0), up_name);
            assert_eq!(name(&mut e, weapon, 0, 0), down_name);
        }
        // Only `up == 1` is the up name of a generic form.
        let other = pick_up_form(&mut e, 0x10, 0);
        assert_eq!(name(&mut e, other, 1, 0), SOUND_GENERIC_UP);
        assert_eq!(name(&mut e, other, 2, 0), SOUND_GENERIC_DOWN);
        // Kind 0x2f with the flag: the sound of the field at +0xc8, or none.
        let sound = new_object(&mut e);
        e.register_double(target(OBJECT_VT, 0x130), |_, a| (a[0] + 5).into_ret());
        let ammo = pick_up_form(&mut e, 0x2f, 0);
        assert_eq!(name(&mut e, ammo, 1, 1), 0);
        e.mem.set_u32(ammo.addr() + 0xc8, sound.addr());
        assert_eq!(name(&mut e, ammo, 1, 1), sound.addr() + 5);
        assert_eq!(name(&mut e, ammo, 1, 0), SOUND_GENERIC_UP);
    }

    #[test]
    fn pick_up_sound_names_from_the_pickup_putdown_component() {
        let mut e = engine();
        pick_up_doubles(&mut e);
        e.register_double(target(OBJECT_VT, 0x130), |_, a| (a[0] + 5).into_ret());
        let actor = new_actor(&mut e);
        let form = pick_up_form(&mut e, 0x28, 0);
        let component = new_object(&mut e);
        let pickup = new_object(&mut e);
        let putdown = new_object(&mut e);
        e.mem.set_u32(form.addr() + 0x10, component.addr());
        // No sound set in the component: the weapon names.
        assert_eq!(
            e.call(0x008a_dcf0, &args![actor, form, 1u32, 0u32]).u32(),
            SOUND_GUNS_BIG_UP
        );
        e.mem.set_u32(component.addr() + 4, pickup.addr());
        e.mem.set_u32(component.addr() + 8, putdown.addr());
        assert_eq!(
            e.call(0x008a_dcf0, &args![actor, form, 1u32, 0u32]).u32(),
            pickup.addr() + 5
        );
        assert_eq!(
            e.call(0x008a_dcf0, &args![actor, form, 0u32, 0u32]).u32(),
            putdown.addr() + 5
        );
        // With the flag the component is ignored.
        assert_eq!(
            e.call(0x008a_dcf0, &args![actor, form, 1u32, 1u32]).u32(),
            SOUND_GUNS_BIG_UP
        );
    }

    #[test]
    fn pick_up_sound_plays_for_the_player_unless_it_already_does() {
        let mut e = engine();
        pick_up_doubles(&mut e);
        let (actor, _) = new_actor_with_process(&mut e);
        e.set_global(PLAYER_CHARACTER, actor.addr());
        let form = pick_up_form(&mut e, 0x10, 0);
        stub(
            &mut e,
            &[
                SOUND_HANDLE_INIT,
                SOUND_HANDLE_ASSIGN,
                SOUND_HANDLE_DESTRUCT,
            ],
        );
        e.register(0x0045_3a70, |_, _| 0xa0d10.into_ret());
        e.register(0x00ad_7550, |_, a| a[1].into_ret());
        e.register(0x00ad_8830, |_, _| Ret::default());
        e.register(0x00ad_8930, |_, _| 0.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x008a_ded0, &args![actor, form, 1u32, 0u32]);
        let log = calls(&mut e);
        let found = called(&log, 0x00ad_7550);
        assert_eq!(found.len(), 1);
        assert_eq!(
            (found[0][0], found[0][2], found[0][3]),
            (0xa0d10, SOUND_GENERIC_UP, 0x121)
        );
        assert_eq!(called(&log, 0x00ad_8830).len(), 1);
        assert_eq!(called(&log, SOUND_HANDLE_DESTRUCT).len(), 2);
        // The handle that was found is assigned to the local one.
        assert_eq!(called(&log, SOUND_HANDLE_ASSIGN)[0][1], found[0][1]);
        // Already playing: not played again.
        e.register(0x00ad_8930, |_, _| 1.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x008a_ded0, &args![actor, form, 1u32, 0u32]);
        assert_eq!(called(&calls(&mut e), 0x00ad_8830).len(), 0);
        // No item: nothing at all.
        e.call_log = Some(vec![]);
        e.call(0x008a_ded0, &args![actor, 0u32, 1u32, 0u32]);
        assert_eq!(calls(&mut e).len(), 1);
    }

    #[test]
    fn pick_up_sound_of_another_actor_goes_through_the_editor_name() {
        let mut e = engine();
        pick_up_doubles(&mut e);
        let (actor, _) = new_actor_with_process(&mut e);
        e.set_global(PLAYER_CHARACTER, 0x7777u32);
        let form = pick_up_form(&mut e, 0x10, 0);
        stub(
            &mut e,
            &[
                SOUND_HANDLE_INIT,
                SOUND_HANDLE_ASSIGN,
                SOUND_HANDLE_DESTRUCT,
            ],
        );
        e.register(0x0093_3270, |_, a| a[1].into_ret());
        e.call_log = Some(vec![]);
        e.call(0x008a_ded0, &args![actor, form, 0u32, 0u32]);
        let log = calls(&mut e);
        let played = called(&log, 0x0093_3270);
        assert_eq!(played.len(), 1);
        assert_eq!(
            (
                played[0][0],
                played[0][2],
                played[0][3],
                played[0][4],
                played[0][5]
            ),
            (actor.addr(), SOUND_GENERIC_DOWN, 0, 0x102, 1)
        );
        assert_eq!(called(&log, 0x00ad_7550).len(), 0);
    }

    /// Doubles and an explosion record for `008ae000`.
    fn impulse_engine() -> (Engine, Ptr<Actor>, Ptr, Ptr) {
        let mut e = engine();
        let (actor, _) = new_actor_with_process(&mut e);
        let target_ref = new_object(&mut e);
        let record: Ptr = Ptr::new(e.mem.alloc(0x100));
        e.mem.set_u32(record.addr() + 4, target_ref.addr());
        e.mem.set_f32(record.addr() + 0x14, 8.0);
        answer(&mut e, ACTOR_VT, 0x1d0, 0);
        e.set_global(ZERO_POINT, 0u32);
        e.set_global(ZERO_DOUBLE, 0.0f64);
        e.set_global(IMPULSE_SCALE_BODY, 2.0f32);
        e.set_global(IMPULSE_SCALE_HIT, 3.0f32);
        e.set_global(IMPULSE_FACTOR, 2.5f64);
        e.set_global(TWO, 2.0f64);
        e.set_global(IMPULSE_FLOOR_DOUBLE, 0.1f64);
        e.set_global(IMPULSE_SLOPE, 0.9f64);
        e.set_global(IMPULSE_FLOOR_FLOAT, 0.1f32);
        e.register(0x0043_fcd0, |_, _| 0x7777.into_ret());
        e.register(0x004a_0bd0, |_, a| a[1].into_ret());
        e.register(0x0068_15c0, |_, a| a[0].into_ret());
        e.register(0x0064_66e0, |_, a| f32::from_bits(a[0]).into_ret());
        e.register(0x0040_4010, |_, a| f32::from_bits(a[0]).into_ret());
        stub(
            &mut e,
            &[
                0x0063_c8a0,
                0x0043_9180,
                0x004a_0c10,
                0x00cb_9650,
                0x00c6_a350,
                0x0043_d410,
                0x00a5_9c60,
                0x004a_3e00,
                0x0062_b660,
            ],
        );
        e.register(0x0043_9ef0, |e, a| {
            for i in 0..3 {
                let value = e.mem.f32(a[0] + 4 * i) - e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, value);
            }
            a[1].into_ret()
        });
        (e, actor, record, target_ref)
    }

    #[test]
    fn impulse_from_a_body_uses_the_strength_alone() {
        let (mut e, actor, record, _) = impulse_engine();
        e.call_log = Some(vec![]);
        e.call(0x008a_e000, &args![actor, record]);
        let log = calls(&mut e);
        // The strength is the one 006466e0 gave (8.0 for the body).
        let impulse = called(&log, 0x0062_b660);
        assert_eq!(impulse.len(), 1);
        assert_eq!(impulse[0][0], 0x7777);
        assert_eq!(impulse[0][2], 8.0f32.to_bits());
        assert_eq!(impulse[0][3], record.addr());
        // The direction is scaled by the body's float.
        assert_eq!(called(&log, 0x0043_9180)[0][1], 2.0f32.to_bits());
        assert_eq!(called(&log, 0x00c6_a350), vec![vec![0x7777, 1, 1, 0, 1]]);
        assert_eq!(called(&log, 0x0043_d410)[0][1..], [0.0f32.to_bits(), 0, 0]);
    }

    #[test]
    fn impulse_is_dropped_for_a_target_that_refuses() {
        let (mut e, actor, record, target_ref) = impulse_engine();
        // Virtual +0x21c true and +0x38c false: nothing happens.
        e.register_double(target(OBJECT_VT, 0x21c), |_, _| 1.into_ret());
        e.register_double(target(OBJECT_VT, 0x38c), |_, _| 0.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x008a_e000, &args![actor, record]);
        assert_eq!(called(&calls(&mut e), 0x0062_b660).len(), 0);
        // +0x38c true: it goes on.
        e.register_double(target(OBJECT_VT, 0x38c), |_, _| 1.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x008a_e000, &args![actor, record]);
        assert_eq!(called(&calls(&mut e), 0x0062_b660).len(), 1);
        // No target at all, or a target without 3D.
        e.mem.set_u32(record.addr() + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x008a_e000, &args![actor, record]);
        assert_eq!(called(&calls(&mut e), 0x0062_b660).len(), 0);
        e.mem.set_u32(record.addr() + 4, target_ref.addr());
        e.register(0x0043_fcd0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x008a_e000, &args![actor, record]);
        assert_eq!(called(&calls(&mut e), 0x0062_b660).len(), 0);
    }

    #[test]
    fn impulse_of_a_hit_falls_off_inside_the_blast_radius() {
        let (mut e, actor, record, _) = impulse_engine();
        // The source is a hit.
        let source = new_object(&mut e);
        e.register_double(target(OBJECT_VT, 0x224), |_, _| 1.into_ret());
        e.mem.set_u32(record.addr() + 8, source.addr());
        // Radii: 4.0 at +0x1a8 (strength 10) and 1.5 at +0x1b8.
        let radii: Ptr = Ptr::new(e.mem.alloc(0x200));
        e.mem.set_f32(radii.addr() + 0x1a8, 4.0);
        e.mem.set_f32(radii.addr() + 0x1b8, 1.5);
        e.mem.set_u32(record.addr() + 0x30, radii.addr());
        // The body is at (1.5, 0.5): squared distance 2.5, between 2.25 and 3.
        let body_point = floats(&mut e, &[1.5, 0.5, 0.0]);
        e.register_double(0x0045_bb80, move |_, _| body_point.addr().into_ret());
        let hit_point = floats(&mut e, &[0.0, 0.0, 0.0]);
        e.register_double(0x004a_0bd0, move |_, _| hit_point.addr().into_ret());
        e.call_log = Some(vec![]);
        e.call(0x008a_e000, &args![actor, record]);
        let log = calls(&mut e);
        // 10 * (((2 * 2.25 - 2.5) / 2.25) * 0.9 + 0.1) = 9.
        let impulse = called(&log, 0x0062_b660);
        let strength = f32::from_bits(impulse[0][2]);
        assert!((strength - 9.0).abs() < 1e-4, "{strength}");
        assert_eq!(called(&log, 0x0064_66e0), vec![vec![8.0f32.to_bits(), 1]]);
        // The hit scale is used.
        assert_eq!(called(&log, 0x0043_9180)[0][1], 3.0f32.to_bits());
        // Far outside twice the radius the strength is a tenth.
        let far = floats(&mut e, &[10.0, 0.0, 0.0]);
        e.register_double(0x0045_bb80, move |_, _| far.addr().into_ret());
        e.call_log = Some(vec![]);
        e.call(0x008a_e000, &args![actor, record]);
        let impulse = called(&calls(&mut e), 0x0062_b660);
        let strength = f32::from_bits(impulse[0][2]);
        assert!((strength - 1.0).abs() < 1e-5, "{strength}");
        // Inside the inner radius nothing changes: strength 10.
        let near = floats(&mut e, &[0.5, 0.5, 0.0]);
        e.register_double(0x0045_bb80, move |_, _| near.addr().into_ret());
        e.call_log = Some(vec![]);
        e.call(0x008a_e000, &args![actor, record]);
        let impulse = called(&calls(&mut e), 0x0062_b660);
        assert_eq!(f32::from_bits(impulse[0][2]), 10.0);
        // A zero hit radius leaves the 10 as well.
        e.mem.set_f32(radii.addr() + 0x1b8, 0.0);
        e.register_double(0x0045_bb80, move |_, _| far.addr().into_ret());
        e.call_log = Some(vec![]);
        e.call(0x008a_e000, &args![actor, record]);
        let impulse = called(&calls(&mut e), 0x0062_b660);
        assert_eq!(f32::from_bits(impulse[0][2]), 10.0);
    }

    #[test]
    fn impulse_of_a_hit_measures_from_the_3d_point_of_the_actor() {
        let (mut e, actor, record, _) = impulse_engine();
        let source = new_object(&mut e);
        e.register_double(target(OBJECT_VT, 0x224), |_, _| 1.into_ret());
        e.mem.set_u32(record.addr() + 8, source.addr());
        let radii: Ptr = Ptr::new(e.mem.alloc(0x200));
        e.mem.set_f32(radii.addr() + 0x1a8, 4.0);
        e.mem.set_f32(radii.addr() + 0x1b8, 1.5);
        e.mem.set_u32(record.addr() + 0x30, radii.addr());
        // The actor's 3D point is (4, 5, 6): 00461130 gives the node and
        // 00524c40 copies its point.
        let node = floats(&mut e, &[4.0, 5.0, 6.0]);
        answer(&mut e, ACTOR_VT, 0x1d0, node.addr());
        answer(&mut e, PROCESS_VT, 0x460, 0x31);
        e.register(0x0046_1130, |_, a| a[0].into_ret());
        e.register(0x0052_4c40, |e, a| {
            for i in 0..3 {
                let value = e.mem.f32(a[0] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, value);
            }
            a[1].into_ret()
        });
        // The body is at (5.5, 5.5, 6): 1.5 and 0.5 away in x and y.
        let body_point = floats(&mut e, &[5.5, 5.5, 6.0]);
        e.register_double(0x0045_bb80, move |_, _| body_point.addr().into_ret());
        let hit_point = floats(&mut e, &[0.0, 0.0, 0.0]);
        e.register_double(0x004a_0bd0, move |_, _| hit_point.addr().into_ret());
        e.call_log = Some(vec![]);
        e.call(0x008a_e000, &args![actor, record]);
        let log = calls(&mut e);
        // The part passed to 00524c40 is the process's virtual +0x460.
        assert_eq!(called(&log, 0x0052_4c40)[0][2], 0x31);
        let impulse = called(&log, 0x0062_b660);
        let strength = f32::from_bits(impulse[0][2]);
        assert!((strength - 9.0).abs() < 1e-4, "{strength}");
    }

    #[test]
    fn impulse_of_a_body_starts_from_the_weapon_point() {
        let (mut e, actor, scratch, _) = weapon_position_engine();
        // Reuse the weapon position doubles for the actor and add those of
        // 008ae000: the 3D node of the actor is its root.
        let target_ref = new_object(&mut e);
        let data: Ptr = Ptr::new(e.mem.alloc(0x100));
        e.mem.set_u32(data.addr() + 4, target_ref.addr());
        e.mem.set_f32(data.addr() + 0x14, 8.0);
        answer(&mut e, PROCESS_VT, 0x460, 0);
        e.set_global(IMPULSE_SCALE_BODY, 2.0f32);
        e.register(0x0043_fcd0, |_, _| 0x7777.into_ret());
        e.register(0x004a_0bd0, |_, a| a[1].into_ret());
        e.register(0x0068_15c0, |_, a| a[0].into_ret());
        e.register(0x0064_66e0, |_, a| f32::from_bits(a[0]).into_ret());
        stub(
            &mut e,
            &[
                0x0063_c8a0,
                0x0043_9180,
                0x004a_0c10,
                0x00cb_9650,
                0x00c6_a350,
                0x0043_d410,
                0x00a5_9c60,
                0x004a_3e00,
                0x0062_b660,
            ],
        );
        // 00524c40 adds the translation of the node and the point given.
        e.register(0x0052_4c40, |e, a| {
            for i in 0..3 {
                let sum = e.mem.f32(a[0] + 4 * i)
                    + if a[2] != 0 {
                        e.mem.f32(a[2] + 4 * i)
                    } else {
                        0.0
                    };
                e.mem.set_f32(a[1] + 4 * i, sum);
            }
            a[1].into_ret()
        });
        let differences = std::rc::Rc::new(std::cell::RefCell::new(Vec::<[f32; 6]>::new()));
        let seen = differences.clone();
        e.register_double(0x0043_9ef0, move |e, a| {
            let mut row = [0.0f32; 6];
            for i in 0..3 {
                row[i] = e.mem.f32(a[0] + 4 * i as u32);
                row[3 + i] = e.mem.f32(a[2] + 4 * i as u32);
            }
            seen.borrow_mut().push(row);
            a[1].into_ret()
        });
        let weapon = e.call(0x008a_5340, &args![actor, scratch, 0u8]).ptr::<()>();
        let weapon = point(&e, weapon);
        answer(&mut e, PROCESS_VT, 0x148, 0x1234);
        e.call(0x008a_e000, &args![actor, data]);
        // A: the root point (10, 20, 30); B: the root point plus the weapon
        // point; the difference is taken A - B.
        let rows = differences.borrow();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0][..3], [10.0, 20.0, 30.0]);
        assert_eq!(
            rows[0][3..],
            [10.0 + weapon[0], 20.0 + weapon[1], 30.0 + weapon[2]]
        );
    }

    #[test]
    fn small_actor_getters_and_setters() {
        let mut e = engine();
        let object: Ptr = Ptr::new(e.mem.alloc(0x400));
        e.mem.set_f32(object.addr() + 0x1a8, 1.5);
        e.mem.set_f32(object.addr() + 0x1b8, 2.5);
        assert_eq!(e.call(0x008a_e480, &args![object]).f32(), 1.5);
        assert_eq!(e.call(0x008a_e4a0, &args![object]).f32(), 2.5);
        e.call(0x008a_e620, &args![object, 0x1234u32]);
        assert_eq!(e.call(0x008a_e600, &args![object]).u32(), 0x1234);
        e.call(0x008a_e640, &args![object, 0x1_01u32]);
        assert_eq!(e.mem.u8(object.addr() + 0x126), 1);
        e.mem.set_u32(object.addr() + 0x3a8, 0x55);
        assert_eq!(e.call(0x008a_fef0, &args![object]).u32(), 0x55);
        e.register(0x0062_1b00, |_, _| 2.5f32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x008a_ff10, &args![object]).f32(), 2.5);
        assert_eq!(
            called(&calls(&mut e), 0x0062_1b00),
            vec![vec![object.addr() + 0x54]]
        );
    }

    #[test]
    fn collector_constructor_and_deleting_destructor() {
        let mut e = engine();
        let object: Ptr = Ptr::new(e.mem.alloc(0x40));
        e.register(0x0062_d350, |_, _| Ret::default());
        e.register(0x00cb_9900, |_, _| Ret::default());
        e.register(0x0062_d470, |_, _| Ret::default());
        assert_eq!(e.call(0x008a_ff30, &args![object]).ptr::<()>(), object);
        assert_eq!(e.mem.u32(object.addr()), 0x0108_4e54);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x008a_ff50, &args![object, 0u32]).ptr::<()>(),
            object
        );
        assert_eq!(called(&calls(&mut e), 0x0062_d470).len(), 0);
        e.call_log = Some(vec![]);
        e.call(0x008a_ff50, &args![object, 3u32]);
        assert_eq!(
            called(&calls(&mut e), 0x0062_d470),
            vec![vec![object.addr()]]
        );
    }

    #[test]
    fn location_on_reference_without_water_is_the_position() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        let position = floats(&mut e, &[1.0, 2.0, 3.0]);
        e.register_double(0x0043_6aa0, move |_, _| position.addr().into_ret());
        e.set_global(NO_WATER, f64::from(f32::MIN));
        e.register(0x005a_2030, |_, _| 0.into_ret());
        assert_eq!(e.call(0x008a_e4c0, &args![actor]).u32(), position.addr());
        // The check passes but there is no water.
        e.register(0x005a_2030, |_, _| 1.into_ret());
        e.register(0x0057_b0a0, |_, _| f32::MIN.into_ret());
        assert_eq!(e.call(0x008a_e4c0, &args![actor]).u32(), position.addr());
    }

    #[test]
    fn location_on_reference_copies_into_the_ring_and_clamps_to_the_water() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        let position = floats(&mut e, &[1.0, 2.0, 3.0]);
        e.register_double(0x0043_6aa0, move |_, _| position.addr().into_ret());
        e.set_global(NO_WATER, f64::from(f32::MIN));
        e.set_global(WATER_MARGIN, 5.0f64);
        e.set_global(HEIGHT_FACTOR, 0.75f64);
        e.register(0x005a_2030, |_, _| 1.into_ret());
        e.register(0x0057_b0a0, |_, _| 10.0f32.into_ret());
        e.register(GET_PARENT_CELL, |_, _| 0x3000.into_ret());
        e.register(0x0045_18e0, |_, _| 1.into_ret());
        e.register(0x0088_53a0, |_, _| 8.0f32.into_ret());
        // z 3 + 6 = 9 is above the water minus 5: z is 5.
        e.set_global(LOCATION_RING_INDEX, 2u32);
        let slot = e.call(0x008a_e4c0, &args![actor]).u32();
        assert_eq!(slot, LOCATION_RING + 3 * 12);
        assert_eq!(point(&e, Ptr::new(slot)), [1.0, 2.0, 5.0]);
        // The ring wraps after the fifth slot.
        e.set_global(LOCATION_RING_INDEX, 4u32);
        let slot = e.call(0x008a_e4c0, &args![actor]).u32();
        assert_eq!(slot, LOCATION_RING);
        // A deep water level leaves z + 6.
        e.register(0x0057_b0a0, |_, _| 40.0f32.into_ret());
        let slot = e.call(0x008a_e4c0, &args![actor]).u32();
        assert_eq!(slot, LOCATION_RING + 12);
        assert_eq!(point(&e, Ptr::new(slot)), [1.0, 2.0, 9.0]);
        // No cell, or a cell that fails 004518e0: the plain copy.
        e.register(0x0045_18e0, |_, _| 0.into_ret());
        let slot = e.call(0x008a_e4c0, &args![actor]).u32();
        assert_eq!(point(&e, Ptr::new(slot)), [1.0, 2.0, 3.0]);
        e.register(GET_PARENT_CELL, |_, _| 0.into_ret());
        let slot = e.call(0x008a_e4c0, &args![actor]).u32();
        assert_eq!(point(&e, Ptr::new(slot)), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn eye_vector_of_an_actor_adds_the_eye_level() {
        let mut e = engine();
        let (actor, _) = new_actor_with_process(&mut e);
        e.set_global(PLAYER_CHARACTER, 0x7777u32);
        let position = floats(&mut e, &[1.0, 2.0, 3.0]);
        e.register_double(0x0043_6aa0, move |_, _| position.addr().into_ret());
        e.register(0x008b_e940, |_, _| 1.5f32.into_ret());
        e.register(0x0068_15c0, |_, a| a[0].into_ret());
        answer_float(&mut e, ACTOR_VT, 0x2bc, 0.5);
        stub(&mut e, &[0x004a_0c90, 0x0043_9f50, 0x004a_0c10]);
        let out: Ptr = Ptr::new(e.mem.alloc(0x10));
        let direction: Ptr = Ptr::new(e.mem.alloc(0x10));
        e.call_log = Some(vec![]);
        e.call(0x008a_ff80, &args![actor, out, direction, 1u32]);
        assert_eq!(point(&e, out), [1.0, 2.0, 4.5]);
        let log = calls(&mut e);
        // The matrix is turned about z by the heading; no x rotation.
        assert_eq!(called(&log, 0x004a_0c90)[0][1], 0.5f32.to_bits());
        assert_eq!(called(&log, 0x0052_4ac0).len(), 0);
        assert_eq!(called(&log, 0x0043_9f50)[0][1..], [1, direction.addr()]);
        assert_eq!(called(&log, 0x004a_0c10), vec![vec![direction.addr()]]);
    }

    #[test]
    fn eye_vector_of_the_player_uses_its_source_and_the_looking_angle() {
        let mut e = engine();
        let (actor, _) = new_actor_with_process(&mut e);
        e.set_global(PLAYER_CHARACTER, actor.addr());
        let source = new_object(&mut e);
        e.set_global(PLAYER_EYE_SOURCE, source.addr());
        let eye = floats(&mut e, &[4.0, 5.0, 6.0]);
        e.register_double(0x0045_bb80, move |_, _| eye.addr().into_ret());
        e.register(0x0068_15c0, |_, a| a[0].into_ret());
        answer_float(&mut e, ACTOR_VT, 0x2bc, 0.5);
        e.register(0x0093_1d70, |_, _| 0.25f32.into_ret());
        e.register(0x0052_4ac0, |_, _| Ret::default());
        // The product is a matrix of nine known words.
        let product = e.mem.alloc(0x30);
        for i in 0..9 {
            e.mem.set_u32(product + 4 * i, 100 + i);
        }
        e.register_double(0x0043_f8d0, move |_, _| product.into_ret());
        e.register(0x0052_4d10, |_, _| 1.into_ret());
        e.register(0x0070_c410, |_, a| a[1].into_ret());
        stub(&mut e, &[0x004a_0c90, 0x004a_0c10, 0x0063_c8a0]);
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::<u32>::new()));
        let words = seen.clone();
        e.register_double(0x0043_9f50, move |e, a| {
            words
                .borrow_mut()
                .extend((0..9).map(|i| e.mem.u32(a[0] + 4 * i)));
            Ret::default()
        });
        let out: Ptr = Ptr::new(e.mem.alloc(0x10));
        let direction: Ptr = Ptr::new(e.mem.alloc(0x10));
        e.call_log = Some(vec![]);
        e.call(0x008a_ff80, &args![actor, out, direction, 1u32]);
        assert_eq!(point(&e, out), [4.0, 5.0, 6.0]);
        let log = calls(&mut e);
        assert_eq!(called(&log, 0x0052_4ac0)[0][1], 0.25f32.to_bits());
        assert_eq!(called(&log, 0x0063_c8a0).len(), 1);
        // The rotation applied is the product (copied word by word).
        assert_eq!(*seen.borrow(), (100..109).collect::<Vec<u32>>());
        // Without the offset flag no offset is added.
        e.call_log = Some(vec![]);
        e.call(0x008a_ff80, &args![actor, out, direction, 0u32]);
        assert_eq!(called(&calls(&mut e), 0x0063_c8a0).len(), 0);
    }

    /// Every function `008ae660` calls outside this file (and the callees of
    /// the functions of this file it uses); the doubles answer 0 unless a
    /// test says otherwise.
    const HIT_CALLEES: &[u32] = &[
        0x0040_1170,
        0x0040_6cc0,
        0x0040_8d60,
        0x0040_ebd0,
        0x0041_3f40,
        0x0041_6870,
        0x0041_81e0,
        0x0043_01b0,
        0x0043_9180,
        0x0043_9ef0,
        0x0043_b4d0,
        0x0043_b4f0,
        0x0043_b540,
        0x0043_d4d0,
        0x0044_59e0,
        0x0044_6390,
        0x0044_ddc0,
        0x0045_19d0,
        0x0045_3a70,
        0x0045_43c0,
        0x0045_7620,
        0x0045_7880,
        0x0045_78c0,
        0x0045_8620,
        0x0045_bb20,
        0x0046_0140,
        0x0046_1130,
        0x0047_6b70,
        0x0047_7ba0,
        0x0049_97b0,
        0x004a_0c10,
        0x004a_1060,
        0x004a_1a10,
        0x004a_3760,
        0x004a_37b0,
        0x004a_3a20,
        0x004a_3a40,
        0x004a_3e00,
        0x004a_3fe0,
        0x004a_40a0,
        0x004a_40c0,
        0x004a_40e0,
        0x004a_4100,
        0x004a_4140,
        0x004a_4180,
        0x004a_41c0,
        0x004a_41e0,
        0x004a_4220,
        0x004a_4240,
        0x004a_e6a0,
        0x004b_4ea0,
        0x004b_5820,
        0x004b_5950,
        0x004b_59f0,
        0x004b_5a80,
        0x004b_6190,
        0x004b_df00,
        0x004b_e380,
        0x0050_8100,
        0x0051_1840,
        0x0051_4410,
        0x0051_7670,
        0x0052_2ba0,
        0x0052_4c40,
        0x0052_5430,
        0x0055_33c0,
        0x0055_9450,
        0x0056_f930,
        0x005a_c750,
        0x005f_2440,
        0x005f_bf20,
        0x0062_05a0,
        0x0062_2a70,
        0x0062_3090,
        0x0062_31a0,
        0x0062_33c0,
        0x0062_4070,
        0x0062_6b30,
        0x0062_6c50,
        0x0062_6d20,
        0x0062_6dd0,
        0x0062_dee0,
        0x0062_e150,
        0x0064_4520,
        0x0064_5c20,
        0x0064_6310,
        0x0066_ef20,
        0x0067_33e0,
        0x0068_90b0,
        0x0068_a810,
        0x0068_a830,
        0x006b_9130,
        0x0070_3350,
        0x0072_6070,
        0x007a_f430,
        0x0080_7770,
        0x0081_f1e0,
        0x0083_7520,
        0x0089_1b90,
        0x0089_d8b0,
        0x008c_71b0,
        0x008d_85e0,
        0x0093_1ed0,
        0x0096_11e0,
        0x0096_3eb0,
        0x009a_9350,
        0x00ad_7480,
        0x00ad_8830,
        0x00ad_8b60,
        0x00c7_fa90,
        0x00cb_9900,
        0x00ec_43fb,
        0x0045_bb80,
        0x008b_e940,
        0x0052_4d10,
        0x0070_c410,
        0x0063_c8a0,
        0x004a_0c90,
        0x0093_1d70,
        0x0052_4ac0,
        0x0043_9f50,
        0x0062_40d0,
        0x0083_7550,
        0x004b_5260,
        0x006f_a820,
        0x0062_d350,
        0x0062_1b00,
        0x0041_a250,
        0x0041_8900,
        0x0048_3710,
    ];

    /// The ray-cast world of `008ae660`: an actor whose swing hits the
    /// object `hit_ref` (a reference, with a physics body when asked).
    struct Swing {
        e: Engine,
        actor: Ptr<Actor>,
        hit_ref: Ptr,
        caster: u32,
    }

    fn swing_world() -> Swing {
        let mut e = engine();
        let (actor, _) = new_actor_with_process(&mut e);
        stub(&mut e, HIT_CALLEES);
        // Settings: the value pointer of a setting is its word at +4.
        e.register(SETTING_VALUE_POINTER, |_, a| (a[0] + 4).into_ret());
        e.register(0x0043_d4d0, |_, a| (a[0] + 4).into_ret());
        e.set_global(SETTING_RANGE_DEFAULT + 4, 3u32);
        e.set_global(SETTING_RANGE_KIND_1 + 4, 100.0f32);
        e.set_global(SETTING_WEIGHT_EXPONENT + 4, 1.0f32);
        e.set_global(SETTING_IMPULSE_FACTOR + 4, 2.0f32);
        e.set_global(SETTING_SPEED_LIMIT + 4, 10.0f32);
        e.set_global(SETTING_BODY_BOOST + 4, 4.0f32);
        e.set_global(HIT_WEIGHT_BONUS, 3.0f64);
        e.set_global(COLOR_CHANNEL_MAX, 255.0f64);
        e.set_global(DECAL_VARIANT_RANGE, 4.0f32);
        e.set_global(HIT_SHAKE_STRENGTH, 0.5f32);
        e.set_global(ONE, 1.0f64);
        e.set_global(ZERO_DOUBLE, 0.0f64);
        e.set_global(PLAYER_CHARACTER, 0x7777u32);
        e.set_global(TES_SINGLETON, 0x6600u32);
        e.set_global(UNARMED_WEAPON, 0x7a00u32);
        // The cell and the physics cell of the actor's cell.
        e.register(GET_PARENT_CELL, |_, _| 0x6000.into_ret());
        e.register(0x0045_43c0, |_, _| 0x6100.into_ret());
        // The hit caster is a block of memory; its collidable (+0x48) and
        // slot (+0x4c) are fixed.
        let caster = e.mem.alloc(0x400);
        e.mem.set_u32(caster + 0x48, 0x7001);
        e.mem.set_u32(caster + 0x4c, 0xffff_ffff);
        e.register_double(0x0062_6dd0, move |_, _| caster.into_ret());
        e.register(0x0068_15c0, |_, a| a[0].into_ret());
        e.register(0x0046_0140, |_, a| a[0].into_ret());
        e.register(0x0080_7770, |_, a| a[0].into_ret());
        e.register(0x0062_6b30, |_, _| 1.into_ret());
        e.register(0x004b_5820, |_, _| 0x7002.into_ret());
        let hit_ref = new_object(&mut e);
        e.register_double(0x0056_f930, move |_, a| {
            if a[0] == 0x7002 {
                hit_ref.addr().into_ret()
            } else {
                0.into_ret()
            }
        });
        e.register(GET_EXTRA_LIST, |_, a| (a[0] + 0x10).into_ret());
        e.register(0x0044_ddc0, |_, a| (a[0] + 0x100).into_ret());
        // The eye position is read from a block; the matrix product too.
        let position = e.mem.alloc(0x10);
        e.register_double(0x0043_6aa0, move |_, _| position.into_ret());
        let product = e.mem.alloc(0x30);
        e.register_double(0x0043_f8d0, move |_, _| product.into_ret());
        answer(&mut e, ACTOR_VT, 0x1d0, 0);
        Swing {
            e,
            actor,
            hit_ref,
            caster,
        }
    }

    /// An impact record whose name object (+0x18) answers `name` to its
    /// virtual method `+0x14`.
    fn impact_record(e: &mut Engine, name: u32) -> Ptr {
        let record: Ptr = Ptr::new(e.mem.alloc(0x100));
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + 0x14, 0x6000_0014);
        e.register_double(0x6000_0014, move |_, _| name.into_ret());
        e.mem.set_u32(record.addr() + 0x18, table);
        record
    }

    #[test]
    fn swing_does_nothing_without_a_cell_a_caster_or_a_ray_hit() {
        let mut w = swing_world();
        let (actor, e) = (w.actor, &mut w.e);
        e.register(0x0045_43c0, |_, _| 0.into_ret());
        assert!(!e.call(0x008a_e660, &args![actor, 0u32]).bool());
        e.register(0x0045_43c0, |_, _| 0x6100.into_ret());
        e.register(0x0062_6dd0, |_, _| 0.into_ret());
        assert!(!e.call(0x008a_e660, &args![actor, 0u32]).bool());
        let caster = w.caster;
        e.register_double(0x0062_6dd0, move |_, _| caster.into_ret());
        // The ray misses: nothing else happens.
        e.register(0x0062_6b30, |_, _| 0.into_ret());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x008a_e660, &args![actor, 0u32]).bool());
        let log = calls(e);
        assert_eq!(called(&log, 0x0062_6d20), vec![vec![caster, 0x6100]]);
        assert_eq!(called(&log, 0x0062_6b30).len(), 1);
        assert_eq!(called(&log, 0x0056_f930).len(), 0);
        // The hit caster is told the actor's group when it differs.
        e.register(0x004a_3a20, |_, _| 2.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x008a_e660, &args![actor, 0u32]);
        assert_eq!(called(&calls(e), 0x0062_6c50), vec![vec![caster, 2]]);
    }

    #[test]
    fn swing_range_comes_from_the_weapon_kind() {
        let mut w = swing_world();
        let (actor, e) = (w.actor, &mut w.e);
        e.set_global(SETTING_RANGE_KIND_0 + 4, 50.0f32);
        e.set_global(SETTING_RANGE_OTHER + 4, 70.0f32);
        e.register(0x0062_6b30, |_, _| 0.into_ret());
        for (kind, range) in [(0u32, 50.0f32), (1, 100.0), (2, 0.0), (9, 70.0)] {
            e.register_double(0x0044_6390, move |_, _| kind.into_ret());
            e.call_log = Some(vec![]);
            e.call(0x008a_e660, &args![actor, 0u32]);
            let ray = called(&calls(e), 0x0062_6b30);
            assert_eq!(ray[0][3], range.to_bits(), "kind {kind}");
        }
        // No form at all: the integer setting.
        e.set_global(UNARMED_WEAPON, 0u32);
        e.call_log = Some(vec![]);
        e.call(0x008a_e660, &args![actor, 0u32]);
        assert_eq!(called(&calls(e), 0x0062_6b30)[0][3], 3.0f32.to_bits());
    }

    /// The hit is an actor (`hit_ref` answers virtual `+0x100`) and the body
    /// of the collidable `0x7004` is of class `0x1e`: no physics body.
    fn swing_hits_an_actor(w: &mut Swing) -> u32 {
        let e = &mut w.e;
        e.register_double(target(OBJECT_VT, 0x100), |_, _| 1.into_ret());
        e.register_double(target(OBJECT_VT, 0x22c), |_, _| 0.into_ret());
        e.register(0x004b_5950, |_, _| 0x7004.into_ret());
        e.register(0x004b_5a80, |_, _| 0x7005.into_ret());
        e.register(0x0043_b4f0, |_, a| a[0].into_ret());
        e.register(0x0043_b4d0, |_, a| {
            (if a[0] == 0x7005 { 0x1eu32 } else { 0 }).into_ret()
        });
        w.hit_ref.addr()
    }

    #[test]
    fn swing_at_an_actor_without_a_body_damages_it_directly() {
        let mut w = swing_world();
        let hit = swing_hits_an_actor(&mut w);
        let (actor, e) = (w.actor, &mut w.e);
        // Fists: the hand to hand formula writes 3.5.
        e.register(0x0064_6310, |e, a| {
            e.mem.set_f32(a[2], 3.5);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x008a_e660, &args![actor, 0u32]).bool());
        let log = calls(e);
        // The target is not a creature type and has no armour.
        assert_eq!(
            called(&log, 0x0083_7520),
            vec![vec![
                actor.addr(),
                3.5f32.to_bits(),
                0,
                hit,
                0,
                0xffff_ffff,
                0xffff_ffff,
                1,
                0
            ]]
        );
        let hand_to_hand = called(&log, 0x0064_6310);
        assert_eq!(hand_to_hand[0][..2], [actor.addr() + 0xa4, 0]);
        // A creature type (virtual +0x21c) takes the creature damage from
        // virtual +0x444, and the other call of the creature target.
        answer(e, ACTOR_VT, 0x21c, 1);
        answer(e, ACTOR_VT, 0x444, 9);
        e.register_double(target(OBJECT_VT, 0x21c), |_, _| 1.into_ret());
        e.call_log = Some(vec![]);
        assert!(e.call(0x008a_e660, &args![actor, 0u32]).bool());
        let log = calls(e);
        assert_eq!(
            called(&log, 0x0083_7520),
            vec![vec![
                actor.addr(),
                9.0f32.to_bits(),
                0,
                hit,
                0,
                0xffff_ffff,
                0xffff_ffff,
                0,
                0
            ]]
        );
        // The target's armour is used when it has some, then released.
        e.register_double(target(OBJECT_VT, 0x21c), |_, _| 0.into_ret());
        e.register(0x0089_d8b0, |_, _| 0x7800.into_ret());
        e.register(0x0051_4410, |_, a| (a[0] & 0x100).into_ret());
        e.call_log = Some(vec![]);
        assert!(e.call(0x008a_e660, &args![actor, 0u32]).bool());
        let log = calls(e);
        assert_eq!(
            called(&log, 0x0083_7520),
            vec![vec![
                actor.addr(),
                9.0f32.to_bits(),
                9.0f32.to_bits(),
                hit,
                0,
                0,
                0xffff_ffff,
                1,
                0
            ]]
        );
        assert_eq!(called(&log, 0x0044_59e0), vec![vec![0x7800, 1]]);
        // A target that is busy (virtual +0x22c) or without a body class
        // 0x1e is left alone.
        e.register_double(target(OBJECT_VT, 0x22c), |_, _| 1.into_ret());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x008a_e660, &args![actor, 0u32]).bool());
        assert_eq!(called(&calls(e), 0x0083_7520).len(), 0);
    }

    #[test]
    fn swing_damage_of_a_held_item_uses_the_power_attack_multiplier() {
        let mut w = swing_world();
        swing_hits_an_actor(&mut w);
        let (actor, e) = (w.actor, &mut w.e);
        let item = new_object(e);
        answer(e, PROCESS_VT, 0x148, item.addr());
        answer(e, ACTOR_VT, 0x1e4, 0x7b00);
        e.register(0x0043_01b0, |_, _| 0x1234.into_ret());
        e.register(0x005f_2440, |_, a| (a[0] + 1).into_ret());
        e.register(0x008d_85e0, |_, _| 0x7c00.into_ret());
        e.register(0x0066_ef20, |_, a| (a[1] + 1).into_ret());
        e.register(0x0064_4520, |_, _| 2.0f32.into_ret());
        e.register(0x004b_df00, |_, _| 21.0f32.into_ret());
        e.call_log = Some(vec![]);
        assert!(e.call(0x008a_e660, &args![actor, 1u32]).bool());
        let log = calls(e);
        // The multiplier: 00644520(value, type) with type = group + 1.
        assert_eq!(
            called(&log, 0x0066_ef20),
            vec![vec![actor.addr() + 0xa4, 0x7c00]]
        );
        assert_eq!(called(&log, 0x0064_4520), vec![vec![0x7c01, 0x1235]]);
        assert_eq!(
            called(&log, 0x004b_df00),
            vec![vec![item.addr(), actor.addr() + 0xa4, 2.0f32.to_bits(), 0]]
        );
        assert_eq!(called(&log, 0x0083_7520)[0][1], 21.0f32.to_bits());
        // Without the power attack flag the multiplier stays 1.
        e.call_log = Some(vec![]);
        e.call(0x008a_e660, &args![actor, 0u32]);
        let log = calls(e);
        assert_eq!(called(&log, 0x0064_4520).len(), 0);
        assert_eq!(called(&log, 0x004b_df00)[0][2], 1.0f32.to_bits());
        // A creature type (virtual +0x21c) or 004997b0 also keeps it at 1.
        e.register(0x0049_97b0, |_, _| 1.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x008a_e660, &args![actor, 1u32]);
        assert_eq!(called(&calls(e), 0x0064_4520).len(), 0);
        // The weapon value of the form is added when allowed.
        e.register(0x0049_97b0, |_, _| 0.into_ret());
        answer(e, ACTOR_VT, 0x360, 1);
        e.register(0x0052_5430, |_, _| 1.into_ret());
        e.register(0x0064_5c20, |_, _| 0x1_0005.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x008a_e660, &args![actor, 0u32]);
        assert_eq!(called(&calls(e), 0x0083_7520)[0][1], 26.0f32.to_bits());
    }

    /// The swing hits a physics body: the body and body ref are set up.
    fn swing_hits_a_body(w: &mut Swing) {
        let e = &mut w.e;
        e.register_double(target(OBJECT_VT, 0x100), |_, _| 1.into_ret());
        e.register(0x004b_59f0, |_, _| 0x7003.into_ret());
        e.register(0x004b_5a80, |_, _| 0x7004.into_ret());
        e.register(0x0043_b4f0, |_, a| a[0].into_ret());
        e.register(0x0043_b4d0, |_, _| 5.into_ret());
        e.register(0x004a_e6a0, |_, _| 0x7600.into_ret());
        e.register(0x0062_05a0, |_, a| a[0].into_ret());
        e.register(0x004a_3a20, |_, _| 2.into_ret());
        e.register(0x0045_8620, |e, a| {
            for (i, v) in [1.0f32, 2.0, 3.0].iter().enumerate() {
                e.mem.set_f32(a[0] + 4 * i as u32, *v);
            }
            Ret::default()
        });
        e.register(0x0043_9ef0, |e, a| {
            for (i, v) in [1.0f32, 0.0, 0.0].iter().enumerate() {
                e.mem.set_f32(a[1] + 4 * i as u32, *v);
            }
            a[1].into_ret()
        });
        e.register(0x0045_bb20, |_, a| a[1].into_ret());
        e.register(0x004b_e380, |_, _| 5.0f32.into_ret());
        e.register(0x0040_ebd0, |_, a| f32::from_bits(a[0]).into_ret());
        e.register(0x004b_4ea0, |_, _| 2.0f32.into_ret());
        e.register(0x0064_6310, |e, a| {
            e.mem.set_f32(a[2], 6.0);
            Ret::default()
        });
        let record = impact_record(e, 0);
        e.register_double(0x0052_2ba0, move |_, _| record.addr().into_ret());
    }

    #[test]
    fn swing_at_a_body_pushes_it_and_plays_the_collision_sound() {
        let mut w = swing_world();
        swing_hits_a_body(&mut w);
        let (actor, hit, e) = (w.actor, w.hit_ref, &mut w.e);
        // The swing weapon has no impact record: the body is still pushed.
        e.register(0x0052_2ba0, |_, _| 0.into_ret());
        e.call_log = Some(vec![]);
        assert!(e.call(0x008a_e660, &args![actor, 0u32]).bool());
        let log = calls(e);
        // The impulse: scaled by weight (5 + 3), the speed ratio 2 / 10 and
        // not boosted (00624070 answers 0).
        let scale = called(&log, 0x0043_9180);
        assert_eq!(scale.len(), 1);
        assert_eq!(scale[0][1], 0.2f32.to_bits());
        let pushes = called(&log, 0x0045_bb20);
        assert_eq!(pushes.len(), 2);
        assert_eq!(pushes[0][2], 2.0f32.to_bits());
        assert_eq!(pushes[1][2], 8.0f32.to_bits());
        // The body gets the impulse (0081f1e0 between the two 005533c0).
        let bodies = called(&log, 0x0081_f1e0);
        assert_eq!(bodies.len(), 1);
        assert_eq!(bodies[0][0], 0x7003);
        assert_eq!(called(&log, 0x0055_33c0), vec![vec![0x6100], vec![0x6100]]);
        // The collision sound of the hit object (it answers virtual +0x100).
        let sound = called(&log, 0x0083_7520);
        assert_eq!(sound.len(), 1);
        assert_eq!(sound[0][3], hit.addr());
        // The action flags are set on the extra data list of the reference.
        assert_eq!(
            called(&log, 0x005a_c750),
            vec![
                vec![actor.addr(), hit.addr() + 0x10, 0x80],
                vec![0x7a00, hit.addr() + 0x10, 0x100]
            ]
        );
        // The reference does not take damage: not an actor-damaging object.
        assert_eq!(called(&log, target(OBJECT_VT, 0x144)).len(), 0);
    }

    #[test]
    fn swing_at_a_body_boosts_a_light_class_and_does_not_slow_a_fast_one() {
        let mut w = swing_world();
        swing_hits_a_body(&mut w);
        let (actor, e) = (w.actor, &mut w.e);
        e.register(0x0062_4070, |_, _| 1.into_ret());
        e.register(0x004b_4ea0, |_, _| 20.0f32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x008a_e660, &args![actor, 0u32]);
        let scale = called(&calls(e), 0x0043_9180);
        // Speed over the limit: no ratio, only the boost.
        assert_eq!(scale.len(), 1);
        assert_eq!(scale[0][1], 4.0f32.to_bits());
    }

    #[test]
    fn swing_at_a_body_uses_the_worn_armour_without_a_weapon() {
        let mut w = swing_world();
        swing_hits_a_body(&mut w);
        let (actor, e) = (w.actor, &mut w.e);
        e.set_global(UNARMED_WEAPON, 0u32);
        e.register(0x0089_1b90, |_, a| {
            (if a[1] == 3 { 0x7d00u32 } else { 0 }).into_ret()
        });
        e.register(0x006b_9130, |_, _| 1.5f32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x008a_e660, &args![actor, 0u32]);
        let log = calls(e);
        // The chest armour weighs 1.5 (+ 3 for the swing).
        let pushes = called(&log, 0x0045_bb20);
        assert_eq!(pushes[1][2], 4.5f32.to_bits());
        assert_eq!(called(&log, 0x0044_59e0), vec![vec![0x7d00, 1]]);
        // No armour at all: weight 3.
        e.register(0x0089_1b90, |_, _| 0.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x008a_e660, &args![actor, 0u32]);
        let pushes = called(&calls(e), 0x0045_bb20);
        assert_eq!(pushes[1][2], 3.0f32.to_bits());
    }

    #[test]
    fn swing_at_a_body_leaves_decals_on_matching_colliders() {
        let mut w = swing_world();
        swing_hits_a_body(&mut w);
        let (actor, e) = (w.actor, &mut w.e);
        // The impact record of the material and its effect 0x7e48.
        let impact = impact_record(e, 0);
        e.register_double(0x0052_2ba0, move |_, _| impact.addr().into_ret());
        e.register(0x0067_33e0, |_, _| 0x7e48.into_ret());
        e.register(0x004a_40c0, |_, _| 2.0f32.into_ret());
        e.register(0x004a_40a0, |_, _| 1.0f32.into_ret());
        e.register(0x0047_6b70, |_, a| f32::from_bits(a[0]).into_ret());
        e.register(0x0062_2a70, |_, _| 0x7f00.into_ret());
        // The collider list: two entries, the first matching the material
        // 0x7600 and the second not.
        let list = e.mem.alloc(0x20);
        let entries = e.mem.alloc(0x40);
        let first = e.mem.alloc(0x20);
        let second = e.mem.alloc(0x20);
        e.mem.set_u32(first, 0x7600);
        e.mem.set_u32(second, 0x7601);
        e.mem.set_u32(entries + 8, first);
        e.mem.set_u32(entries + 0xc, 0xffff_ffff);
        e.mem.set_u32(entries + 0x10 + 8, second);
        e.mem.set_u32(entries + 0x10 + 0xc, 0xffff_ffff);
        e.mem.set_u32(list, entries);
        e.mem.set_u32(list + 4, 2);
        e.register_double(0x0041_3f40, move |_, _| list.into_ret());
        e.register(0x0072_6070, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(0x0062_e150, |e, a| (e.mem.u32(a[0]) + a[1] * 16).into_ret());
        e.register(0x0055_9450, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(0x004a_3a40, |_, a| a[0].into_ret());
        e.register(0x008c_71b0, |_, a| a[0].into_ret());
        // Not the landscape: the collider's own 3D and reference.
        e.register(0x00c7_fa90, |_, _| 0x7002.into_ret());
        e.register(0x004a_1060, |_, _| 1.into_ret());
        e.register(0x0045_19d0, |_, _| 0x8100.into_ret());
        e.register(0x0062_1b00, |_, _| 0.25f32.into_ret());
        e.register(0x004a_4220, |_, _| 0x0033_6699.into_ret());
        e.register(0x004a_4140, |_, _| 1.into_ret());
        e.register(0x004a_4180, |_, _| 2.into_ret());
        e.register(0x004a_4100, |_, _| 3.into_ret());
        e.register(0x004a_41e0, |_, _| 4.into_ret());
        e.register(0x004a_41c0, |_, _| 5.5f32.into_ret());
        e.register(0x004a_40e0, |_, _| 6.5f32.into_ret());
        e.register(0x009a_9350, |_, _| 7.5f32.into_ret());
        e.register(0x004a_4240, |_, _| 0.75f32.into_ret());
        e.register(0x0040_6cc0, |_, _| 2.5f64.into_ret());
        e.register(FTOL, |_, a| {
            (f64::from_bits(a[0] as u64 | (a[1] as u64) << 32) as i32).into_ret()
        });
        e.register(0x0041_6870, |e, a| {
            for i in 1..4 {
                e.mem.set_u32(a[0] + 4 * (i - 1), a[i as usize]);
            }
            a[0].into_ret()
        });
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::<Vec<u32>>::new()));
        let record = seen.clone();
        e.register_double(0x004a_3fe0, move |e, a| {
            record
                .borrow_mut()
                .push((0..0x1f).map(|i| e.mem.u32(a[1] + 4 * i)).collect());
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x008a_e660, &args![actor, 0u32]).bool());
        let log = calls(e);
        // The decal is made with the drawn size, flagged twice, cast into
        // the cell, and the collector asked and freed.
        assert_eq!(called(&log, 0x0062_2a70), vec![vec![1.0f32.to_bits(), 1]]);
        assert_eq!(called(&log, 0x004a_1a10).len(), 2);
        assert_eq!(called(&log, 0x0062_31a0), vec![vec![0x7f00, 0x6100]]);
        assert_eq!(called(&log, 0x0062_3090).len(), 1);
        assert_eq!(called(&log, 0x00cb_9900).len(), 1);
        // Only the matching collider gets a decal (at the cell's spot).
        let infos = seen.borrow();
        assert_eq!(infos.len(), 1);
        assert_eq!(called(&log, 0x004a_3fe0)[0][0], 0x8100);
        let info = &infos[0];
        // The origin (the contact point) and the direction.
        assert_eq!(
            info[..3],
            [1.0f32.to_bits(), 2.0f32.to_bits(), 3.0f32.to_bits()]
        );
        assert_eq!(
            info[3..6],
            [1.0f32.to_bits(), 2.0f32.to_bits(), 3.0f32.to_bits()]
        );
        // The size twice, the spread, the random angle.
        assert_eq!(info[0x38 / 4], 1.0f32.to_bits());
        assert_eq!(info[0x3c / 4], 1.0f32.to_bits());
        assert_eq!(info[0x40 / 4], 0.25f32.to_bits());
        assert_eq!(info[0x44 / 4], 0.75f32.to_bits());
        // The collider's 3D target, the effect, the flags and the variant.
        assert_eq!(info[0x28 / 4], 0x7002);
        assert_eq!(info[0x30 / 4], 0x7e48);
        assert_eq!(info[0x70 / 4].to_le_bytes(), [2, 0, 0, 3]);
        assert_eq!(info[0x74 / 4].to_le_bytes(), [2, 1, 4, 0]);
        assert_eq!(info[0x4c / 4], 5.5f32.to_bits());
        assert_eq!(info[0x54 / 4], 6.5f32.to_bits());
        assert_eq!(info[0x58 / 4], 7.5f32.to_bits());
        // Blue, green, red of 0x336699 over 255.
        let channel = |value: u32| (f64::from(value) / 255.0) as f32;
        assert_eq!(
            info[0x60 / 4..0x6c / 4],
            [
                channel(0x99).to_bits(),
                channel(0x66).to_bits(),
                channel(0x33).to_bits()
            ]
        );
    }

    #[test]
    fn swing_at_a_body_plays_the_impact_sounds_and_damages_the_reference() {
        let mut w = swing_world();
        swing_hits_a_body(&mut w);
        let (actor, hit, e) = (w.actor, w.hit_ref, &mut w.e);
        // The record has two sound names but no effect.
        e.register(0x0068_a810, |_, _| 0x8200.into_ret());
        e.register(0x0068_a830, |_, _| 0x8300.into_ret());
        e.register(0x0051_1840, |_, a| (a[0] + 1).into_ret());
        e.register(0x0045_3a70, |_, _| 0xa0d10.into_ret());
        e.register(0x00ad_7480, |_, a| a[1].into_ret());
        // The reference takes the damage (virtual +0x224) of the fists, 6.
        e.register_double(target(OBJECT_VT, 0x224), |_, _| 1.into_ret());
        e.register(0x0051_7670, |_, _| 3.into_ret());
        answer(e, ACTOR_VT, 0x168, 0x8400);
        e.register(0x0047_7ba0, |_, _| 1.into_ret());
        e.call_log = Some(vec![]);
        assert!(e.call(0x008a_e660, &args![actor, 0u32]).bool());
        let log = calls(e);
        let sounds = called(&log, 0x00ad_7480);
        assert_eq!(sounds.len(), 2);
        assert_eq!(
            (sounds[0][2], sounds[0][3], sounds[0][4]),
            (0x8201, 0x4102, 0x8200)
        );
        assert_eq!(
            (sounds[1][2], sounds[1][3], sounds[1][4]),
            (0x8301, 0x4102, 0x8300)
        );
        // Each is played from the contact point.
        let positions = called(&log, 0x00ad_8b60);
        assert_eq!(positions.len(), 2);
        assert_eq!(
            positions[0][1..],
            [1.0f32.to_bits(), 2.0f32.to_bits(), 3.0f32.to_bits()]
        );
        assert_eq!(called(&log, 0x00ad_8830).len(), 2);
        // The reference learns its attacker (low body class 3 < 4) and takes
        // the damage; the shake is the player's only.
        assert_eq!(
            called(&log, target(OBJECT_VT, 0x164)),
            vec![vec![hit.addr(), 0x8400]]
        );
        assert_eq!(
            called(&log, target(OBJECT_VT, 0x144)),
            vec![vec![hit.addr(), 6.0f32.to_bits(), 0]]
        );
        assert_eq!(called(&log, 0x0096_3eb0).len(), 0);
        // A body class of 4 or more does not.
        e.register(0x0051_7670, |_, _| 4.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x008a_e660, &args![actor, 0u32]);
        assert_eq!(called(&calls(e), target(OBJECT_VT, 0x164)).len(), 0);
        // For the player the shake is called.
        e.set_global(PLAYER_CHARACTER, actor.addr());
        e.call_log = Some(vec![]);
        e.call(0x008a_e660, &args![actor, 0u32]);
        let shake = called(&calls(e), 0x0096_3eb0);
        assert_eq!(shake, vec![vec![actor.addr(), 10, 0.5f32.to_bits(), 0]]);
    }

    #[test]
    fn swing_at_a_body_leaves_a_humanoid_effect_unless_turned_off() {
        let mut w = swing_world();
        swing_hits_a_body(&mut w);
        let (actor, e) = (w.actor, &mut w.e);
        // The name object at +0x18 answers a pointer to "x".
        let name = e.mem.alloc(0x10);
        e.mem.set_u8(name, b'x');
        let impact = impact_record(e, name);
        e.register_double(0x0052_2ba0, move |_, _| impact.addr().into_ret());
        e.register(0x0050_8100, |_, _| 0.5f32.into_ret());
        e.register(0x004b_6190, |_, _| 0.25f32.into_ret());
        e.register(0x004a_3760, |_, a| a[0].into_ret());
        // The hit is an actor with a creature base form.
        e.register(0x0041_81e0, |_, _| 0x8500.into_ret());
        e.register(DYNAMIC_CAST, |_, _| 0x8600.into_ret());
        e.register(0x005f_bf20, |_, _| 1.into_ret());
        e.register(0x0040_8d60, |_, a| a[0].into_ret());
        e.set_global(SETTING_NO_HUMANOID_EFFECT, 0u32);
        e.call_log = Some(vec![]);
        e.call(0x008a_e660, &args![actor, 0u32]);
        let log = calls(e);
        let effect = called(&log, 0x0068_90b0);
        assert_eq!(effect.len(), 1);
        assert_eq!(effect[0][0], 0x6000);
        assert_eq!(effect[0][1], 0.5f32.to_bits());
        assert_eq!(effect[0][2], name);
        assert_eq!(effect[0][9..], [1.0f32.to_bits(), 7, 0]);
        // The setting byte turns it off.
        e.set_global(SETTING_NO_HUMANOID_EFFECT, 1u32);
        e.call_log = Some(vec![]);
        e.call(0x008a_e660, &args![actor, 0u32]);
        assert_eq!(called(&calls(e), 0x0068_90b0).len(), 0);
        // An empty name stays quiet as well.
        e.set_global(SETTING_NO_HUMANOID_EFFECT, 0u32);
        e.mem.set_u8(name, 0);
        e.call_log = Some(vec![]);
        e.call(0x008a_e660, &args![actor, 0u32]);
        assert_eq!(called(&calls(e), 0x0068_90b0).len(), 0);
    }

    #[test]
    fn swing_by_the_player_at_its_activation_target_sets_the_action_flag() {
        let mut w = swing_world();
        let (actor, hit, e) = (w.actor, w.hit_ref, &mut w.e);
        e.set_global(PLAYER_CHARACTER, actor.addr());
        // The hit reference lives in a cell whose form is 0x8700 (+8).
        e.register(GET_SAVE_FORM, |_, _| 0x8708.into_ret());
        e.register_double(0x0070_3350, move |_, _| hit.addr().into_ret());
        e.register(0x0062_dee0, |_, a| u32::from(a[0] == 0x8700).into_ret());
        e.call_log = Some(vec![]);
        assert!(e.call(0x008a_e660, &args![actor, 0u32]).bool());
        let log = calls(e);
        assert_eq!(
            called(&log, 0x005a_c750),
            vec![vec![hit.addr(), hit.addr() + 0x10, 0x1000_0000]]
        );
        // Another reference than the activation target: nothing.
        e.register(0x0070_3350, |_, _| 0x1234.into_ret());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x008a_e660, &args![actor, 0u32]).bool());
        assert_eq!(called(&calls(e), 0x005a_c750).len(), 0);
    }
}
