//! `fallout/ai/combat/combatprocedurebeincover.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 009d2510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers 7 (`ECX` not read, no stack words). The map names it
/// `CombatProcedureBeInCover::GetType` (through that class's vtable), but
/// the body is shared by identical-code folding: several vtables point at
/// it, and `TESAttributes`' save and load (`00480100`, `00480130`) call it
/// for the size of the attribute data.
pub fn fn_009d2510(_e: &mut Engine, _this: Ptr) -> u32 {
    7
}

/// The function `fn_009d3370` calls on the member (`006e4820`).
const CALLEE_006E4820: u32 = 0x006e_4820;

// Translated from 009d3370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `006e4820` with `ECX` = `this + 0x14` and passes on its `bool` result (`AL`).
pub fn fn_009d3370(e: &mut Engine, this: Ptr) -> bool {
    e.call(CALLEE_006E4820, &args![this.byte_add(0x14)]).bool()
}

layout! {
    /// `CombatProcedureBeInCover` (Xbox PDB), 0x6c bytes; the offsets are
    /// the ones the PC code uses.
    pub struct CombatProcedureBeInCover: 0x6c {
        /// `pCombatController` (Xbox PDB, `CombatProcedure`): `CombatController*`.
        0x04 pCombatController: Ptr,
        /// `eState` (Xbox PDB): 0 initializing, 1 waiting behind cover, 2
        /// moving out, 3 waiting out of cover, 4 firing out of cover, 5
        /// moving in, 6 moving in and rotating, 7 holding ground (the debug
        /// texts).
        0x10 eState: u32,
        /// `pCoverLocation` (Xbox PDB): `CombatCoverLocation*`.
        0x14 pCoverLocation: Ptr,
    }
}

/// The embedded members (Xbox PDB names): `CurrentCoverPosition`,
/// `CurrentFirePosition`, `LookAtPosition` (`NiPoint3`) and the
/// `CombatTimer`s.
const COVER_POSITION: u32 = 0x18;
const FIRE_POSITION: u32 = 0x24;
const LOOK_AT_POSITION: u32 = 0x30;
const WAIT_BEHIND_COVER_TIMER: u32 = 0x3c;
const WAITING_OUT_OF_COVER_TIMER: u32 = 0x44;
const FIRING_OUT_OF_COVER_TIMER: u32 = 0x4c;
const REEVALUATE_COVER_TIMER: u32 = 0x54;
const HOLDING_GROUND_TIMER: u32 = 0x5c;
const TARGET_BLOCKED_TIMER: u32 = 0x64;
/// `CombatProcedureBeInCover`'s vtable (`010910f4`).
const VTABLE: u32 = 0x0109_10f4;

/// `CombatProcedure` methods (Xbox PDB names), its constructor `009ca680`,
/// `009caac0` (fail with a reason).
const BASE_CONSTRUCT: u32 = 0x009c_a680;
const GET_ATTACKER: u32 = 0x0099_6de0;
const GET_TARGET: u32 = 0x0099_6e00;
const SET_MOVE_MODE: u32 = 0x0099_6e20;
const SET_LOOK_AT_TARGET: u32 = 0x0099_6e60;
const CLEAR_LOOK_AT_TARGET: u32 = 0x0099_6ea0;
const BASE_SAVE_GAME: u32 = 0x0099_6ed0;
const BASE_LOAD_GAME: u32 = 0x0099_6ef0;
const FAIL_PROCEDURE: u32 = 0x009c_aac0;
/// Controller getters: `00586150` the combat state, `004fb070` the combat
/// group, `00980070` the combat style, `00998a30` the byte `+0xd4`
/// (crouching, as the move mode it picks shows).
const GET_COMBAT_STATE: u32 = 0x0058_6150;
const CONTROLLER_GROUP: u32 = 0x004f_b070;
const CONTROLLER_STYLE: u32 = 0x0098_0070;
const CONTROLLER_BYTE_D4: u32 = 0x0099_8a30;
/// Combat state methods (Xbox PDB names where the map has them):
/// `IsCoverLocationValid`, `CheckCoverSafe`, `ClearCoverLocation`;
/// `0097fb20` (the byte `+0x71`), `008b3c90` (the byte `+0x72`), `0097fb40`
/// (copies the time stamp `+0x9c` out), `0097fb60` / `0097b050` (the cover
/// locations `+0xe8` / `+0xe4`).
const IS_COVER_LOCATION_VALID: u32 = 0x009a_0600;
const CHECK_COVER_SAFE: u32 = 0x0099_ccd0;
const CLEAR_COVER_LOCATION: u32 = 0x009a_03d0;
const STATE_BYTE_71: u32 = 0x0097_fb20;
const STATE_BYTE_72: u32 = 0x008b_3c90;
const STATE_TIMESTAMP_COPY: u32 = 0x0097_fb40;
const STATE_COVER_LOCATION: u32 = 0x0097_fb60;
const STATE_LAST_COVER_LOCATION: u32 = 0x0097_b050;
/// `009891a0` (`__thiscall` on the combat group, one word, `AL`).
const GROUP_TEST_91A0: u32 = 0x0098_91a0;
/// `005407b0` (`__thiscall`, one byte): stores the byte at `+0` (here of
/// the cover location).
const STORE_BYTE_0: u32 = 0x0054_07b0;
/// `00982be0` (`__thiscall` on the cover location's `+0x14`, `AL`) and
/// `006e4820` (the same, its byte `+0x4c` is 2).
const COVER_TEST_2BE0: u32 = 0x0098_2be0;
const COVER_TEST_4820: u32 = 0x006e_4820;
/// `CombatUtilities::CalculateCoverPositions` (Xbox PDB, `__cdecl`).
const CALCULATE_COVER_POSITIONS: u32 = 0x009a_9370;
/// `RandomFloat` (engine map name of `00476b70`, `__cdecl`, two floats).
const RANDOM_FLOAT: u32 = 0x0047_6b70;
/// Actor methods (Xbox PDB names) and unnamed helpers: `008b3b90` (a path
/// test), `008d6f30` (its cell), `008a8870` (`AL`).
const STOP_MOVING_ACTOR: u32 = 0x008b_3ab0;
const IS_PATH_VALID: u32 = 0x008b_3960;
const IS_PATHING_COMPLETE: u32 = 0x008b_3bb0;
const SET_PATHFINDING_GOAL: u32 = 0x008b_3630;
const BUILD_REQUEST: u32 = 0x008b_3880;
const GET_SNEAK_HEIGHT: u32 = 0x0088_54d0;
const GET_HEIGHT: u32 = 0x0088_53a0;
const GET_CURRENT_AMMO_CLIP_PERCENT: u32 = 0x008a_88f0;
const QUEUE_RELOAD: u32 = 0x008a_8840;
const ACTOR_TEST_8A8870: u32 = 0x008a_8870;
const ACTOR_PATH_TEST: u32 = 0x008b_3b90;
const ACTOR_CELL_8D6F30: u32 = 0x008d_6f30;
const GET_WORLD_SPACE: u32 = 0x0057_5d70;
/// Actor vtable slots: `+0x1f4` the position, `+0x194` (one word: an out
/// point, which it returns).
const SLOT_POSITION: u32 = 0x1f4;
const SLOT_LOOK_POINT: u32 = 0x194;
/// `PathingRequest::PathingRequest` (Xbox PDB) and its destructor
/// `006e2620`.
const PATHING_REQUEST_CONSTRUCT: u32 = 0x006e_2420;
const PATHING_REQUEST_DESTRUCT: u32 = 0x006e_2620;
/// Point and timer helpers: `00439ef0` (difference into the out point),
/// `004a7290` (squared length), `006815c0` (a constructor returning
/// `this`), `00906f60` (the timer's constructor), `0097fa50` (running),
/// `008d7f40` (start), `008d7f80` (ran out), `00910b10` (clear),
/// `00435e00` (elapsed), `006b9130` (duration), `006a7f50` (a time stamp's
/// time), and the `CombatTimer::SaveGame` / `LoadGame` (Xbox PDB).
const POINT_DIFFERENCE: u32 = 0x0043_9ef0;
const POINT_LENGTH_SQUARED: u32 = 0x004a_7290;
const POINT_CONSTRUCT: u32 = 0x0068_15c0;
const TIMER_CONSTRUCT: u32 = 0x0090_6f60;
const TIMER_RUNNING: u32 = 0x0097_fa50;
const TIMER_START: u32 = 0x008d_7f40;
const TIMER_EXPIRED: u32 = 0x008d_7f80;
const TIMER_CLEAR: u32 = 0x0091_0b10;
const TIMER_ELAPSED: u32 = 0x0043_5e00;
const TIMER_DURATION: u32 = 0x006b_9130;
const TIMESTAMP_TIME: u32 = 0x006a_7f50;
const TIMER_SAVE: u32 = 0x009a_5eb0;
const TIMER_LOAD: u32 = 0x009a_5f20;
/// `00403e20` (`__thiscall` on a setting): the address of its value;
/// `fCoverWaitReloadClipPercent`, `fCoverFiringReloadClipPercent` (named by
/// their static initializers).
const SETTING_VALUE: u32 = 0x0040_3e20;
const SETTING_COVER_WAIT_RELOAD_CLIP_PERCENT: u32 = 0x011c_e320;
const SETTING_COVER_FIRING_RELOAD_CLIP_PERCENT: u32 = 0x011c_fa34;
/// Constants of the exe: the `float` 2; the `double`s 0.5, 0.75, 2, 4,
/// 1024 (`0101ecb8`), 262144 (`01091138`) and 589824 (`0108cb08`).
const TWO_FLOAT: u32 = 0x0101_62c0;
const HALF_DOUBLE: u32 = 0x0101_1588;
const THREE_QUARTERS_DOUBLE: u32 = 0x0101_de30;
const TWO_DOUBLE: u32 = 0x0101_1590;
const FOUR_DOUBLE: u32 = 0x0101_db80;
const MOVE_IN_DISTANCE_SQUARED_DOUBLE: u32 = 0x0101_ecb8;
const HOLD_DISTANCE_SQUARED_DOUBLE: u32 = 0x0109_1138;
const REEVALUATE_DISTANCE_SQUARED_DOUBLE: u32 = 0x0108_cb08;
/// `BGSSaveGameBuffer` / `BGSLoadGameBuffer` byte writers and readers.
const SAVE_BYTES: u32 = 0x0086_5e50;
const LOAD_BYTES: u32 = 0x0086_4980;
/// The debug-text object's print slot (`__cdecl` through the vtable).
const SLOT_PRINT: u32 = 0x14;
/// Strings.
const TEXT_INITIALIZING: u32 = 0x0109_0b68;
const TEXT_FIRING_TIMER: u32 = 0x0109_1084;
const TEXT_TARGET_HAS_LOS: u32 = 0x0109_1140;
const TEXT_PATH_BUILDING_FAILED_IN_AND_ROTATE: u32 = 0x0109_1164;
const TEXT_PATHING_FAILED_IN_AND_ROTATING: u32 = 0x0109_1190;
const TEXT_PATH_BUILDING_FAILED_IN: u32 = 0x0109_11bc;
const TEXT_PATHING_FAILED_IN: u32 = 0x0109_11dc;
const TEXT_CANNOT_SETUP_BACK_TO_COVER: u32 = 0x0109_11fc;
const TEXT_WAITED_BUT_BLOCKED: u32 = 0x0109_1230;
const TEXT_WAITED_GROUP_HAS_LOS: u32 = 0x0109_1254;
const TEXT_WAITED_WITHOUT_FIRING: u32 = 0x0109_128c;
const TEXT_PATH_BUILDING_FAILED_OUT: u32 = 0x0109_12b0;
const TEXT_PATHING_FAILED_OUT: u32 = 0x0109_12d4;
const TEXT_CANNOT_SETUP_FIRE_POSITION: u32 = 0x0109_12f4;
const TEXT_CANNOT_SETUP_COVER_POSITION: u32 = 0x0109_1318;
const TEXT_COVER_LOCATION_GONE: u32 = 0x0109_133c;
const TEXT_FIRE_LOCATION: u32 = 0x0109_135c;
const TEXT_COVER_LOCATION: u32 = 0x0109_1380;
const TEXT_REEVALUATE_TIMER: u32 = 0x0109_13a4;
const TEXT_HOLDING_GROUND_TIMER: u32 = 0x0109_13c8;
const TEXT_HOLDING_GROUND: u32 = 0x0109_13ec;
const TEXT_MOVING_IN_AND_ROTATE: u32 = 0x0109_13fc;
const TEXT_MOVING_IN: u32 = 0x0109_1414;
const TEXT_FIRING_OUT_OF_COVER: u32 = 0x0109_1420;
const TEXT_TARGET_BLOCKED_TIMER: u32 = 0x0109_1434;
const TEXT_WAITING_TIMER: u32 = 0x0109_1458;
const TEXT_WAITING_OUT_OF_COVER: u32 = 0x0109_1474;
const TEXT_MOVING_OUT: u32 = 0x0109_148c;
const TEXT_WAIT_TIMER: u32 = 0x0109_1498;
const TEXT_WAITING_BEHIND_COVER: u32 = 0x0109_14b0;

fn controller_of(e: &Engine, this: Ptr<CombatProcedureBeInCover>) -> u32 {
    e.get(this, CombatProcedureBeInCover::pCombatController)
        .addr()
}

fn cover_of(e: &Engine, this: Ptr<CombatProcedureBeInCover>) -> u32 {
    e.get(this, CombatProcedureBeInCover::pCoverLocation).addr()
}

fn copy_point(e: &mut Engine, from: u32, to: u32) {
    for i in 0..3 {
        let word = e.mem.u32(from + i * 4);
        e.mem.set_u32(to + i * 4, word);
    }
}

/// A random time of the style (`009d32e0`, `009d3310` or `009d3340` on the
/// controller's style) stored as a `float`.
fn style_time(
    e: &mut Engine,
    this: Ptr<CombatProcedureBeInCover>,
    getter: fn(&mut Engine, u32) -> f64,
) -> u32 {
    let controller = controller_of(e, this);
    let style = e.call(CONTROLLER_STYLE, &args![controller]).u32();
    x87_store32(getter(e, style))
}

/// Builds a pathing request to `goal` and sets it as the attacker's goal;
/// on failure the procedure fails with `reason`. The request lives in the
/// frame at `request`. Returns whether the goal was set (the request is
/// destroyed either way).
fn path_to(e: &mut Engine, this: u32, attacker: u32, request: u32, goal: u32, reason: u32) -> bool {
    e.call(PATHING_REQUEST_CONSTRUCT, &args![request]);
    let worldspace = e.call(GET_WORLD_SPACE, &args![attacker]).u32();
    let cell = e.call(ACTOR_CELL_8D6F30, &args![attacker]).u32();
    e.call(
        BUILD_REQUEST,
        &args![
            attacker,
            request,
            goal,
            cell,
            worldspace,
            0.0f32.to_bits(),
            0u32
        ],
    );
    if !e
        .call(SET_PATHFINDING_GOAL, &args![attacker, request])
        .bool()
    {
        e.call(FAIL_PROCEDURE, &args![this, reason]);
        e.call(PATHING_REQUEST_DESTRUCT, &args![request]);
        return false;
    }
    true
}

/// The failure of a path the actor was following: the reason depends on
/// whether the path is still valid.
fn fail_path(e: &mut Engine, this: u32, attacker: u32, valid_reason: u32, invalid_reason: u32) {
    let reason = if e.call(IS_PATH_VALID, &args![attacker]).bool() {
        valid_reason
    } else {
        invalid_reason
    };
    e.call(FAIL_PROCEDURE, &args![this, reason]);
}

// Translated from 009d2440 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009d2440 - the compiler's FS:[0] exception frame (unwinding of the base class if a member constructor throws) is not translated
/// `CombatProcedureBeInCover::CombatProcedureBeInCover` (Xbox PDB; one
/// word): the base class, the vtable, state 0, the cover location, the
/// three points and the six timers. Returns `this`.
pub fn construct(
    e: &mut Engine,
    this: Ptr<CombatProcedureBeInCover>,
    cover_location: u32,
) -> Ptr<CombatProcedureBeInCover> {
    let t = this.addr();
    e.call(BASE_CONSTRUCT, &args![t]);
    e.mem.set_u32(t, VTABLE);
    e.set(this, CombatProcedureBeInCover::eState, 0);
    e.mem.set_u32(t + 0x14, cover_location);
    for point in [COVER_POSITION, FIRE_POSITION, LOOK_AT_POSITION] {
        e.call(POINT_CONSTRUCT, &args![t + point]);
    }
    for timer in [
        WAIT_BEHIND_COVER_TIMER,
        WAITING_OUT_OF_COVER_TIMER,
        FIRING_OUT_OF_COVER_TIMER,
        REEVALUATE_COVER_TIMER,
        HOLDING_GROUND_TIMER,
        TARGET_BLOCKED_TIMER,
    ] {
        e.call(TIMER_CONSTRUCT, &args![t + timer]);
    }
    this
}

// Translated from 009d25b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009d25b0 - the compiler's FS:[0] exception frame (unwinding of the pathing requests) is not translated
/// `CombatProcedureBeInCover::Update` (Xbox PDB): fails when the cover
/// location no longer exists. Every second (the reevaluate timer) it
/// checks the cover: with the target seen (`0097fb20`) an unsafe cover
/// makes it hold its ground (state 7, 2 seconds); otherwise a target more
/// than 768 from the cover that the group test `009891a0` accepts marks
/// the cover location (`005407b0` with 1). Then the state (switch table
/// `009d32b4`): 0 computes the cover and fire positions (the look-at
/// height raised by 0.75 of the actor's height) and paths to the cover
/// (state 6); 1 waits behind cover, reloading below
/// `fCoverWaitReloadClipPercent` or moving out to the fire position (state
/// 2) when the wait timer runs out; 2 and 5 and 6 follow their paths; 3
/// waits out of cover until the target is seen again (firing, state 4),
/// the timer runs out, a group member sees the target, or a blocked shot
/// lasts 2 seconds; 4 fires until the clip falls below
/// `fCoverFiringReloadClipPercent`, `008a8870` holds or the firing timer
/// runs out, then moves back in (state 5); 7 moves back in after a
/// reevaluation, and gives up when the holding timer runs out or, after a
/// quarter of it, with the target within 512.
///
/// Locals follow the disassembly's frame (offsets below `EBP`).
pub fn update(e: &mut Engine, this: Ptr<CombatProcedureBeInCover>) {
    const FRAME: u32 = 0x3a0;
    let t = this.addr();
    let controller = controller_of(e, this);
    let combat_state = e.call(GET_COMBAT_STATE, &args![controller]).u32();
    let cover = cover_of(e, this);
    if !e
        .call(IS_COVER_LOCATION_VALID, &args![combat_state, cover])
        .bool()
    {
        e.call(FAIL_PROCEDURE, &args![t, TEXT_COVER_LOCATION_GONE]);
        return;
    }
    e.with_stack(FRAME, |e, base| {
        let ebp = base.addr() + FRAME;
        let attacker = e.call(GET_ATTACKER, &args![t]).u32();
        let target = e.call(GET_TARGET, &args![t]).u32();
        let controller = controller_of(e, this);
        let combat_state = e.call(GET_COMBAT_STATE, &args![controller]).u32();
        let controller = controller_of(e, this);
        let group = e.call(CONTROLLER_GROUP, &args![controller]).u32();
        let mut reevaluated = false;
        let mut holding = false;
        let cover = cover_of(e, this);
        let crouch_cover = fn_009d3370(e, Ptr::new(cover));
        let controller = controller_of(e, this);
        let crouching = e.call(CONTROLLER_BYTE_D4, &args![controller]).bool();
        let move_mode = if crouch_cover && crouching { 2u32 } else { 1 };
        let one = 1.0f32.to_bits();
        if !e
            .call(TIMER_RUNNING, &args![t + REEVALUATE_COVER_TIMER])
            .bool()
        {
            e.call(TIMER_START, &args![t + REEVALUATE_COVER_TIMER, one]);
        } else if e
            .call(TIMER_EXPIRED, &args![t + REEVALUATE_COVER_TIMER])
            .bool()
        {
            e.call(TIMER_START, &args![t + REEVALUATE_COVER_TIMER, one]);
            reevaluated = true;
            if e.get(this, CombatProcedureBeInCover::eState) != 0 {
                if e.call(STATE_BYTE_71, &args![combat_state]).bool() {
                    let cover = cover_of(e, this);
                    let safe = e
                        .call(
                            CHECK_COVER_SAFE,
                            &args![combat_state, cover + 0x14, t + COVER_POSITION, target, 1u32],
                        )
                        .bool();
                    if !safe && e.get(this, CombatProcedureBeInCover::eState) != 7 {
                        e.call(STOP_MOVING_ACTOR, &args![attacker]);
                        e.call(CLEAR_LOOK_AT_TARGET, &args![t]);
                        e.set(this, CombatProcedureBeInCover::eState, 7);
                        let two = x87_quiet_bits(e.global::<u32>(TWO_FLOAT));
                        e.call(TIMER_START, &args![t + HOLDING_GROUND_TIMER, two]);
                        holding = true;
                    }
                } else {
                    let target_position = ebp - 0x40;
                    let cover_position = ebp - 0x34;
                    let difference = ebp - 0x4c;
                    let position = e.vcall(target, SLOT_POSITION, &args![]).u32();
                    copy_point(e, position, target_position);
                    let cover = cover_of(e, this);
                    copy_point(e, cover + 8, cover_position);
                    e.call(
                        POINT_DIFFERENCE,
                        &args![target_position, difference, cover_position],
                    );
                    let squared = e.call(POINT_LENGTH_SQUARED, &args![difference]).f64();
                    if squared > e.global::<f64>(REEVALUATE_DISTANCE_SQUARED_DOUBLE)
                        && e.call(GROUP_TEST_91A0, &args![group, target]).bool()
                    {
                        let cover = cover_of(e, this);
                        e.call(STORE_BYTE_0, &args![cover, 1u32]);
                    }
                }
            }
        }
        // The compiler's switch on the state (table `009d32b4`).
        match e.get(this, CombatProcedureBeInCover::eState) {
            0 => {
                let cover = cover_of(e, this);
                e.mem.set_u8(cover + 0x74, 0);
                let controller = controller_of(e, this);
                e.call(
                    CALCULATE_COVER_POSITIONS,
                    &args![controller, cover, t + COVER_POSITION, t + FIRE_POSITION],
                );
                copy_point(e, t + FIRE_POSITION, t + LOOK_AT_POSITION);
                let controller = controller_of(e, this);
                let height = if e.call(CONTROLLER_BYTE_D4, &args![controller]).bool() {
                    e.call(GET_SNEAK_HEIGHT, &args![attacker, 0u32]).f64()
                } else {
                    e.call(GET_HEIGHT, &args![attacker]).f64()
                };
                let height = x87_store32(height);
                let raised = x87_add(
                    x87_mul(x87_load32(height), e.global::<f64>(THREE_QUARTERS_DOUBLE)),
                    x87_m32(e.mem.u32(t + 0x38)),
                );
                e.mem.set_u32(t + 0x38, x87_store32(raised));
                let request = ebp - 0x104;
                if !path_to(
                    e,
                    t,
                    attacker,
                    request,
                    t + COVER_POSITION,
                    TEXT_CANNOT_SETUP_COVER_POSITION,
                ) {
                    return;
                }
                e.set(this, CombatProcedureBeInCover::eState, 6);
                let cover = cover_of(e, this);
                if fn_009d3390(e, cover) {
                    e.call(SET_LOOK_AT_TARGET, &args![t, t + LOOK_AT_POSITION, 3u32]);
                }
                e.call(SET_MOVE_MODE, &args![t, 3u32]);
                e.call(PATHING_REQUEST_DESTRUCT, &args![request]);
            }
            1 => {
                if e.call(ACTOR_TEST_8A8870, &args![attacker]).bool() {
                    return;
                }
                let stamp = e
                    .call(STATE_TIMESTAMP_COPY, &args![combat_state, ebp - 0x34c])
                    .u32();
                let since = e.call(TIMER_ELAPSED, &args![stamp]).f64();
                let reload = since > e.global::<f64>(HALF_DOUBLE) && {
                    let clip = e
                        .call(GET_CURRENT_AMMO_CLIP_PERCENT, &args![attacker])
                        .f64();
                    let value = e
                        .call(
                            SETTING_VALUE,
                            &args![SETTING_COVER_WAIT_RELOAD_CLIP_PERCENT],
                        )
                        .u32();
                    x87_load32(e.mem.u32(value)) > clip
                };
                if reload {
                    e.call(QUEUE_RELOAD, &args![attacker]);
                    return;
                }
                if !e
                    .call(TIMER_EXPIRED, &args![t + WAIT_BEHIND_COVER_TIMER])
                    .bool()
                {
                    return;
                }
                let request = ebp - 0x1b4;
                if !path_to(
                    e,
                    t,
                    attacker,
                    request,
                    t + FIRE_POSITION,
                    TEXT_CANNOT_SETUP_FIRE_POSITION,
                ) {
                    return;
                }
                let cover = cover_of(e, this);
                if fn_009d3390(e, cover) {
                    e.call(CLEAR_LOOK_AT_TARGET, &args![t]);
                }
                let point = e.vcall(target, SLOT_LOOK_POINT, &args![ebp - 0x358]).u32();
                e.call(SET_LOOK_AT_TARGET, &args![t, point, 1u32]);
                e.set(this, CombatProcedureBeInCover::eState, 2);
                e.call(SET_MOVE_MODE, &args![t, move_mode]);
                e.call(PATHING_REQUEST_DESTRUCT, &args![request]);
            }
            2 => {
                if e.call(ACTOR_PATH_TEST, &args![attacker]).bool() {
                    fail_path(
                        e,
                        t,
                        attacker,
                        TEXT_PATHING_FAILED_OUT,
                        TEXT_PATH_BUILDING_FAILED_OUT,
                    );
                } else if e.call(IS_PATHING_COMPLETE, &args![attacker]).bool() {
                    e.set(this, CombatProcedureBeInCover::eState, 3);
                    e.call(CLEAR_LOOK_AT_TARGET, &args![t]);
                    let wait = style_time(e, this, fn_009d3310);
                    e.call(TIMER_START, &args![t + WAITING_OUT_OF_COVER_TIMER, wait]);
                }
            }
            3 => {
                let stamp = ebp - 0x1b8;
                e.call(STATE_TIMESTAMP_COPY, &args![combat_state, stamp]);
                let last_seen = e.call(TIMESTAMP_TIME, &args![stamp]).f64();
                let waited_since = e
                    .call(TIMESTAMP_TIME, &args![t + WAIT_BEHIND_COVER_TIMER])
                    .f64();
                // FCOMP last_seen; TEST AH,5; JP: fires when the wait
                // started before the target was last seen.
                if waited_since < last_seen {
                    e.set(this, CombatProcedureBeInCover::eState, 4);
                    let fire = style_time(e, this, fn_009d3340);
                    e.call(TIMER_START, &args![t + FIRING_OUT_OF_COVER_TIMER, fire]);
                    e.call(TIMER_CLEAR, &args![t + TARGET_BLOCKED_TIMER]);
                } else if e
                    .call(TIMER_EXPIRED, &args![t + WAITING_OUT_OF_COVER_TIMER])
                    .bool()
                {
                    e.call(CLEAR_COVER_LOCATION, &args![combat_state, 0u32]);
                    e.call(FAIL_PROCEDURE, &args![t, TEXT_WAITED_WITHOUT_FIRING]);
                    return;
                } else if e
                    .call(TIMER_ELAPSED, &args![t + WAITING_OUT_OF_COVER_TIMER])
                    .f64()
                    > e.global::<f64>(TWO_DOUBLE)
                    && !e.call(STATE_BYTE_71, &args![combat_state]).bool()
                    && e.call(GROUP_TEST_91A0, &args![group, target]).bool()
                {
                    e.call(CLEAR_COVER_LOCATION, &args![combat_state, 0u32]);
                    e.call(FAIL_PROCEDURE, &args![t, TEXT_WAITED_GROUP_HAS_LOS]);
                    return;
                }
                if e.call(STATE_BYTE_72, &args![combat_state]).bool() {
                    if !e
                        .call(TIMER_RUNNING, &args![t + TARGET_BLOCKED_TIMER])
                        .bool()
                    {
                        let two = x87_quiet_bits(e.global::<u32>(TWO_FLOAT));
                        e.call(TIMER_START, &args![t + TARGET_BLOCKED_TIMER, two]);
                    } else if e
                        .call(TIMER_EXPIRED, &args![t + TARGET_BLOCKED_TIMER])
                        .bool()
                    {
                        e.call(CLEAR_COVER_LOCATION, &args![combat_state, 1u32]);
                        e.call(FAIL_PROCEDURE, &args![t, TEXT_WAITED_BUT_BLOCKED]);
                    }
                }
            }
            4 => {
                let clip = x87_store32(
                    e.call(GET_CURRENT_AMMO_CLIP_PERCENT, &args![attacker])
                        .f64(),
                );
                let value = e
                    .call(
                        SETTING_VALUE,
                        &args![SETTING_COVER_FIRING_RELOAD_CLIP_PERCENT],
                    )
                    .u32();
                // FLD clip; FLD setting; FCOMPP; TEST AH,0x41; JZ: back in
                // when the setting is greater.
                let back = x87_load32(e.mem.u32(value)) > x87_load32(clip)
                    || e.call(ACTOR_TEST_8A8870, &args![attacker]).bool()
                    || e.call(TIMER_EXPIRED, &args![t + FIRING_OUT_OF_COVER_TIMER])
                        .bool();
                if !back {
                    return;
                }
                let request = ebp - 0x26c;
                if !path_to(
                    e,
                    t,
                    attacker,
                    request,
                    t + COVER_POSITION,
                    TEXT_CANNOT_SETUP_BACK_TO_COVER,
                ) {
                    return;
                }
                e.call(SET_MOVE_MODE, &args![t, move_mode]);
                e.set(this, CombatProcedureBeInCover::eState, 5);
                e.call(PATHING_REQUEST_DESTRUCT, &args![request]);
            }
            5 => {
                if e.call(ACTOR_PATH_TEST, &args![attacker]).bool() {
                    fail_path(
                        e,
                        t,
                        attacker,
                        TEXT_PATHING_FAILED_IN,
                        TEXT_PATH_BUILDING_FAILED_IN,
                    );
                    return;
                }
                if e.call(IS_PATHING_COMPLETE, &args![attacker]).bool() {
                    e.set(this, CombatProcedureBeInCover::eState, 1);
                    e.call(CLEAR_LOOK_AT_TARGET, &args![t]);
                    e.call(SET_MOVE_MODE, &args![t, 2u32]);
                    let wait = style_time(e, this, fn_009d32e0);
                    e.call(TIMER_START, &args![t + WAIT_BEHIND_COVER_TIMER, wait]);
                    return;
                }
                let position = e.vcall(attacker, SLOT_POSITION, &args![]).u32();
                let difference = e
                    .call(
                        POINT_DIFFERENCE,
                        &args![position, ebp - 0x364, t + COVER_POSITION],
                    )
                    .u32();
                let squared = x87_store32(e.call(POINT_LENGTH_SQUARED, &args![difference]).f64());
                // FCOMP 1024; TEST AH,5; JP: only when less.
                if x87_load32(squared) < e.global::<f64>(MOVE_IN_DISTANCE_SQUARED_DOUBLE) {
                    e.set(this, CombatProcedureBeInCover::eState, 6);
                    let cover = cover_of(e, this);
                    if fn_009d3390(e, cover) {
                        e.call(SET_LOOK_AT_TARGET, &args![t, t + LOOK_AT_POSITION, 3u32]);
                    }
                    e.call(SET_MOVE_MODE, &args![t, 1u32]);
                }
            }
            6 => {
                if e.call(ACTOR_PATH_TEST, &args![attacker]).bool() {
                    fail_path(
                        e,
                        t,
                        attacker,
                        TEXT_PATHING_FAILED_IN_AND_ROTATING,
                        TEXT_PATH_BUILDING_FAILED_IN_AND_ROTATE,
                    );
                } else if e.call(IS_PATHING_COMPLETE, &args![attacker]).bool() {
                    e.set(this, CombatProcedureBeInCover::eState, 1);
                    e.call(SET_MOVE_MODE, &args![t, 2u32]);
                    let wait = style_time(e, this, fn_009d32e0);
                    e.call(TIMER_START, &args![t + WAIT_BEHIND_COVER_TIMER, wait]);
                }
            }
            7 => {
                if reevaluated && !holding {
                    let request = ebp - 0x324;
                    if !path_to(
                        e,
                        t,
                        attacker,
                        request,
                        t + COVER_POSITION,
                        TEXT_CANNOT_SETUP_BACK_TO_COVER,
                    ) {
                        return;
                    }
                    e.call(SET_MOVE_MODE, &args![t, move_mode]);
                    e.set(this, CombatProcedureBeInCover::eState, 5);
                    e.call(PATHING_REQUEST_DESTRUCT, &args![request]);
                    return;
                }
                if e.call(TIMER_EXPIRED, &args![t + HOLDING_GROUND_TIMER])
                    .bool()
                {
                    e.call(CLEAR_COVER_LOCATION, &args![combat_state, 0u32]);
                    e.call(FAIL_PROCEDURE, &args![t, TEXT_TARGET_HAS_LOS]);
                    return;
                }
                let elapsed = e
                    .call(TIMER_ELAPSED, &args![t + HOLDING_GROUND_TIMER])
                    .f64();
                let duration = e
                    .call(TIMER_DURATION, &args![t + HOLDING_GROUND_TIMER])
                    .f64();
                // FDIV 4; FCOMP elapsed; TEST AH,5; JP: only once a quarter
                // has passed.
                if x87_div(duration, e.global::<f64>(FOUR_DOUBLE)) < elapsed {
                    let target_position = ebp - 0x33c;
                    let attacker_position = ebp - 0x330;
                    let difference = ebp - 0x348;
                    let position = e.vcall(target, SLOT_POSITION, &args![]).u32();
                    copy_point(e, position, target_position);
                    let position = e.vcall(attacker, SLOT_POSITION, &args![]).u32();
                    copy_point(e, position, attacker_position);
                    e.call(
                        POINT_DIFFERENCE,
                        &args![attacker_position, difference, target_position],
                    );
                    let squared = e.call(POINT_LENGTH_SQUARED, &args![difference]).f64();
                    if squared < e.global::<f64>(HOLD_DISTANCE_SQUARED_DOUBLE) {
                        e.call(CLEAR_COVER_LOCATION, &args![combat_state, 0u32]);
                        e.call(FAIL_PROCEDURE, &args![t, TEXT_TARGET_HAS_LOS]);
                    }
                }
            }
            _ => {}
        }
    });
}

/// `RandomFloat` between the two floats at `low` and `low + 4` of the
/// style.
fn style_random(e: &mut Engine, style: u32, low: u32) -> f64 {
    let high = x87_quiet_bits(e.mem.u32(style + low + 4));
    let low = x87_quiet_bits(e.mem.u32(style + low));
    e.call(RANDOM_FLOAT, &args![low, high]).f64()
}

// Translated from 009d32e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A random time between the style's floats `+0x20` and `+0x24`
/// (`__thiscall` on the combat style, `ST0`): the wait behind cover.
pub fn fn_009d32e0(e: &mut Engine, this: u32) -> f64 {
    style_random(e, this, 0x20)
}

// Translated from 009d3310 (decompiled, FalloutNV.exe 1.4.0.525)
/// A random time between the style's floats `+0x28` and `+0x2c`: the wait
/// out of cover.
pub fn fn_009d3310(e: &mut Engine, this: u32) -> f64 {
    style_random(e, this, 0x28)
}

// Translated from 009d3340 (decompiled, FalloutNV.exe 1.4.0.525)
/// A random time between the style's floats `+0x30` and `+0x34`: the
/// firing out of cover.
pub fn fn_009d3340(e: &mut Engine, this: u32) -> f64 {
    style_random(e, this, 0x30)
}

// Translated from 009d3390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the cover location (`__thiscall`) has its `+0x14` test
/// `00982be0` or `006e4820`, or its byte `+0x74` set (then the procedure
/// looks at the look-at position while moving in).
pub fn fn_009d3390(e: &mut Engine, this: u32) -> bool {
    e.call(COVER_TEST_2BE0, &args![this + 0x14]).bool()
        || e.call(COVER_TEST_4820, &args![this + 0x14]).bool()
        || e.mem.u8(this + 0x74) != 0
}

// Translated from 009d33e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureBeInCover::FiredProjectile` (Xbox PDB; one word, not
/// read): behind cover the wait starts again; while moving out the actor
/// stops and fires (state 4) for a random time.
pub fn fired_projectile(e: &mut Engine, this: Ptr<CombatProcedureBeInCover>, _unused_1: u32) {
    let t = this.addr();
    match e.get(this, CombatProcedureBeInCover::eState) {
        1 => {
            let wait = style_time(e, this, fn_009d32e0);
            e.call(TIMER_START, &args![t + WAIT_BEHIND_COVER_TIMER, wait]);
        }
        2 => {
            let attacker = e.call(GET_ATTACKER, &args![t]).u32();
            e.call(STOP_MOVING_ACTOR, &args![attacker]);
            e.call(CLEAR_LOOK_AT_TARGET, &args![t]);
            e.set(this, CombatProcedureBeInCover::eState, 4);
            let fire = style_time(e, this, fn_009d3340);
            e.call(TIMER_START, &args![t + FIRING_OUT_OF_COVER_TIMER, fire]);
        }
        _ => {}
    }
}

// Translated from 009d3470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureBeInCover::DisplayDebugText` (Xbox PDB): the state
/// (switch table `009d37d4`) with its timer, the reevaluate timer while it
/// runs, and the cover and fire locations.
pub fn display_debug_text(
    e: &mut Engine,
    this: Ptr<CombatProcedureBeInCover>,
    text: u32,
    first: u32,
    second: u32,
) {
    let t = this.addr();
    let print = |e: &mut Engine, line: u32| {
        e.vcall(text, SLOT_PRINT, &args![first, second, line]);
    };
    let print_timer = |e: &mut Engine, format: u32, timer: u32| {
        let duration = e.call(TIMER_DURATION, &args![t + timer]).f64();
        let elapsed = e.call(TIMER_ELAPSED, &args![t + timer]).f64();
        e.vcall(
            text,
            SLOT_PRINT,
            &args![first, second, format, elapsed, duration],
        );
    };
    match e.get(this, CombatProcedureBeInCover::eState) {
        0 => print(e, TEXT_INITIALIZING),
        1 => {
            print(e, TEXT_WAITING_BEHIND_COVER);
            print_timer(e, TEXT_WAIT_TIMER, WAIT_BEHIND_COVER_TIMER);
        }
        2 => print(e, TEXT_MOVING_OUT),
        3 => {
            print(e, TEXT_WAITING_OUT_OF_COVER);
            print_timer(e, TEXT_WAITING_TIMER, WAITING_OUT_OF_COVER_TIMER);
            if e.call(TIMER_RUNNING, &args![t + TARGET_BLOCKED_TIMER])
                .bool()
            {
                print_timer(e, TEXT_TARGET_BLOCKED_TIMER, TARGET_BLOCKED_TIMER);
            }
        }
        4 => {
            print(e, TEXT_FIRING_OUT_OF_COVER);
            print_timer(e, TEXT_FIRING_TIMER, FIRING_OUT_OF_COVER_TIMER);
        }
        5 => print(e, TEXT_MOVING_IN),
        6 => print(e, TEXT_MOVING_IN_AND_ROTATE),
        7 => {
            print(e, TEXT_HOLDING_GROUND);
            print_timer(e, TEXT_HOLDING_GROUND_TIMER, HOLDING_GROUND_TIMER);
        }
        _ => {}
    }
    if e.call(TIMER_RUNNING, &args![t + REEVALUATE_COVER_TIMER])
        .bool()
    {
        print_timer(e, TEXT_REEVALUATE_TIMER, REEVALUATE_COVER_TIMER);
    }
    for (format, point) in [
        (TEXT_COVER_LOCATION, COVER_POSITION),
        (TEXT_FIRE_LOCATION, FIRE_POSITION),
    ] {
        let z = x87_load32(e.mem.u32(t + point + 8));
        let y = x87_load32(e.mem.u32(t + point + 4));
        let x = x87_load32(e.mem.u32(t + point));
        e.vcall(text, SLOT_PRINT, &args![first, second, format, x, y, z]);
    }
}

/// The timers `SaveGame` and `LoadGame` write and read, in order.
const SAVED_TIMERS: [u32; 6] = [
    WAIT_BEHIND_COVER_TIMER,
    WAITING_OUT_OF_COVER_TIMER,
    FIRING_OUT_OF_COVER_TIMER,
    REEVALUATE_COVER_TIMER,
    HOLDING_GROUND_TIMER,
    TARGET_BLOCKED_TIMER,
];

// Translated from 009d3800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureBeInCover::SaveGame` (Xbox PDB): the base class, the
/// state, whether the cover location is the combat state's current one,
/// the three points and the six timers.
pub fn save_game(e: &mut Engine, this: Ptr<CombatProcedureBeInCover>, buffer: u32) {
    let t = this.addr();
    e.call(BASE_SAVE_GAME, &args![t, buffer]);
    e.with_stack(4, |e, current| {
        let controller = controller_of(e, this);
        let combat_state = e.call(GET_COMBAT_STATE, &args![controller]).u32();
        let location = e.call(STATE_COVER_LOCATION, &args![combat_state]).u32();
        let is_current = location == e.mem.u32(t + 0x14);
        e.mem.set_u8(current.addr(), u8::from(is_current));
        e.call(SAVE_BYTES, &args![buffer, t + 0x10, 4u32, 0u32]);
        e.call(SAVE_BYTES, &args![buffer, current, 1u32, 0u32]);
    });
    for point in [COVER_POSITION, FIRE_POSITION, LOOK_AT_POSITION] {
        e.call(SAVE_BYTES, &args![buffer, t + point, 0xcu32, 0u32]);
    }
    for timer in SAVED_TIMERS {
        e.call(TIMER_SAVE, &args![t + timer, buffer]);
    }
}

// Translated from 009d38f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureBeInCover::LoadGame` (Xbox PDB): the mirror of
/// `SaveGame`; the cover location becomes the combat state's current one
/// or its last one.
pub fn load_game(e: &mut Engine, this: Ptr<CombatProcedureBeInCover>, buffer: u32) {
    let t = this.addr();
    e.call(BASE_LOAD_GAME, &args![t, buffer]);
    e.with_stack(4, |e, current| {
        e.mem.set_u8(current.addr(), 0);
        e.call(LOAD_BYTES, &args![buffer, t + 0x10, 4u32]);
        e.call(LOAD_BYTES, &args![buffer, current, 1u32]);
        let controller = controller_of(e, this);
        let combat_state = e.call(GET_COMBAT_STATE, &args![controller]).u32();
        let location = if e.mem.u8(current.addr()) != 0 {
            e.call(STATE_COVER_LOCATION, &args![combat_state]).u32()
        } else {
            e.call(STATE_LAST_COVER_LOCATION, &args![combat_state])
                .u32()
        };
        e.mem.set_u32(t + 0x14, location);
    });
    for point in [COVER_POSITION, FIRE_POSITION, LOOK_AT_POSITION] {
        e.call(LOAD_BYTES, &args![buffer, t + point, 0xcu32]);
    }
    for timer in SAVED_TIMERS {
        e.call(TIMER_LOAD, &args![t + timer, buffer]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009d2510, fn_009d2510(Ptr) -> u32),
        entry!(0x009d_3370, fn_009d3370(Ptr) -> bool),
        entry!(
            0x009d_2440,
            construct(Ptr<CombatProcedureBeInCover>, u32) -> Ptr<CombatProcedureBeInCover>
        ),
        entry!(0x009d_25b0, update(Ptr<CombatProcedureBeInCover>)),
        entry!(0x009d_32e0, fn_009d32e0(u32) -> f64),
        entry!(0x009d_3310, fn_009d3310(u32) -> f64),
        entry!(0x009d_3340, fn_009d3340(u32) -> f64),
        entry!(0x009d_3390, fn_009d3390(u32) -> bool),
        entry!(
            0x009d_33e0,
            fired_projectile(Ptr<CombatProcedureBeInCover>, u32)
        ),
        entry!(
            0x009d_3470,
            display_debug_text(Ptr<CombatProcedureBeInCover>, u32, u32, u32)
        ),
        entry!(0x009d_3800, save_game(Ptr<CombatProcedureBeInCover>, u32)),
        entry!(0x009d_38f0, load_game(Ptr<CombatProcedureBeInCover>, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_seven() {
        let mut e = Engine::new();
        let back = e.call(0x009d_2510, &args![Ptr::<()>::new(0x1234)]).u32();
        assert_eq!(back, 7);
    }

    #[test]
    fn l68d_009d3370_passes_this_plus_14_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_006E4820, |_, a| Ret {
            eax: a[0] & 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(!e.call(0x009d_3370, &args![0x4100u32]).bool());
        assert!(e.call(0x009d_3370, &args![0x4100u32 + 1]).bool());
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_006E4820).collect();
        assert_eq!(inner.len(), 2);
        assert_eq!(inner[0].1, vec![0x4100 + 0x14]);
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

    impl Rig {
        fn queue_floats(&self, addr: u32, values: &[f64]) {
            self.script
                .borrow_mut()
                .queues
                .insert(addr, values.iter().map(|&v| float(v)).collect());
        }
    }

    const CALLEES: &[u32] = &[
        BASE_CONSTRUCT,
        GET_ATTACKER,
        GET_TARGET,
        SET_MOVE_MODE,
        SET_LOOK_AT_TARGET,
        CLEAR_LOOK_AT_TARGET,
        BASE_SAVE_GAME,
        BASE_LOAD_GAME,
        FAIL_PROCEDURE,
        GET_COMBAT_STATE,
        CONTROLLER_GROUP,
        CONTROLLER_STYLE,
        CONTROLLER_BYTE_D4,
        IS_COVER_LOCATION_VALID,
        CHECK_COVER_SAFE,
        CLEAR_COVER_LOCATION,
        STATE_BYTE_71,
        STATE_BYTE_72,
        STATE_TIMESTAMP_COPY,
        STATE_COVER_LOCATION,
        STATE_LAST_COVER_LOCATION,
        GROUP_TEST_91A0,
        STORE_BYTE_0,
        COVER_TEST_2BE0,
        COVER_TEST_4820,
        CALCULATE_COVER_POSITIONS,
        RANDOM_FLOAT,
        STOP_MOVING_ACTOR,
        IS_PATH_VALID,
        IS_PATHING_COMPLETE,
        SET_PATHFINDING_GOAL,
        BUILD_REQUEST,
        GET_SNEAK_HEIGHT,
        GET_HEIGHT,
        GET_CURRENT_AMMO_CLIP_PERCENT,
        QUEUE_RELOAD,
        ACTOR_TEST_8A8870,
        ACTOR_PATH_TEST,
        ACTOR_CELL_8D6F30,
        GET_WORLD_SPACE,
        PATHING_REQUEST_CONSTRUCT,
        PATHING_REQUEST_DESTRUCT,
        POINT_DIFFERENCE,
        POINT_LENGTH_SQUARED,
        POINT_CONSTRUCT,
        TIMER_CONSTRUCT,
        TIMER_RUNNING,
        TIMER_START,
        TIMER_EXPIRED,
        TIMER_CLEAR,
        TIMER_ELAPSED,
        TIMER_DURATION,
        TIMESTAMP_TIME,
        TIMER_SAVE,
        TIMER_LOAD,
        SAVE_BYTES,
        LOAD_BYTES,
    ];

    struct World {
        rig: Rig,
        this: Ptr<CombatProcedureBeInCover>,
        t: u32,
        controller: u32,
        attacker: u32,
        target: u32,
        cover: u32,
        style: u32,
    }

    fn world() -> World {
        let mut rig = Rig::new();
        let t = rig.e.mem.alloc(0x6c);
        let this = Ptr::<CombatProcedureBeInCover>::new(t);
        let controller = rig.object(0x100);
        rig.e.set(
            this,
            CombatProcedureBeInCover::pCombatController,
            Ptr::new(controller),
        );
        let attacker = rig.actor();
        let target = rig.actor();
        let cover = rig.e.mem.alloc(0x80);
        rig.e.mem.set_u32(t + 0x14, cover);
        let style = rig.e.mem.alloc(0x40);
        rig.e.mem.set_f32(style + 0x20, 1.0);
        rig.e.mem.set_f32(style + 0x24, 2.0);
        rig.e.mem.set_f32(style + 0x30, 3.0);
        rig.e.mem.set_f32(style + 0x34, 4.0);
        let position = rig.e.mem.alloc(0xc);
        rig.answer(GET_ATTACKER, attacker);
        rig.answer(GET_TARGET, target);
        rig.answer(GET_COMBAT_STATE, 0x5100);
        rig.answer(CONTROLLER_GROUP, 0x5200);
        rig.answer(CONTROLLER_STYLE, style);
        rig.answer(IS_COVER_LOCATION_VALID, 1);
        rig.answer(TIMER_RUNNING, 1);
        rig.answer(SET_PATHFINDING_GOAL, 1);
        rig.slot(SLOT_POSITION, position);
        World {
            rig,
            this,
            t,
            controller,
            attacker,
            target,
            cover,
            style,
        }
    }

    fn state(w: &World) -> u32 {
        w.rig.e.get(w.this, CombatProcedureBeInCover::eState)
    }

    #[test]
    fn construct_sets_up_the_procedure() {
        let mut w = world();
        let block = w.rig.e.mem.alloc(0x6c);
        w.rig.e.mem.write(block, &[0xcc; 0x6c]);
        let (result, log) = w.rig.run(0x009d_2440, &args![block, 0x7700u32]);
        assert_eq!(result.u32(), block);
        assert_eq!(calls_to(&log, BASE_CONSTRUCT), vec![vec![block]]);
        assert_eq!(w.rig.e.mem.u32(block), VTABLE);
        assert_eq!(w.rig.e.mem.u32(block + 0x10), 0);
        assert_eq!(w.rig.e.mem.u32(block + 0x14), 0x7700);
        assert_eq!(
            calls_to(&log, POINT_CONSTRUCT),
            vec![vec![block + 0x18], vec![block + 0x24], vec![block + 0x30]]
        );
        let timers: Vec<_> = calls_to(&log, TIMER_CONSTRUCT)
            .iter()
            .map(|c| c[0] - block)
            .collect();
        assert_eq!(timers, vec![0x3c, 0x44, 0x4c, 0x54, 0x5c, 0x64]);
    }

    #[test]
    fn update_fails_without_the_cover_and_paths_to_it() {
        let mut w = world();
        w.rig.answer(IS_COVER_LOCATION_VALID, 0);
        let (_, log) = w.rig.run(0x009d_25b0, &args![w.t]);
        assert_eq!(
            calls_to(&log, FAIL_PROCEDURE),
            vec![vec![w.t, TEXT_COVER_LOCATION_GONE]]
        );
        assert_eq!(
            calls_to(&log, IS_COVER_LOCATION_VALID),
            vec![vec![0x5100, w.cover]]
        );

        // State 0: the cover positions, the look-at height and the path.
        let mut w = world();
        let t = w.t;
        w.rig.answer(TIMER_RUNNING, 0);
        w.rig.answer_float(GET_HEIGHT, 100.0);
        w.rig.e.mem.set_f32(t + 0x2c, 5.0);
        w.rig.answer(GET_WORLD_SPACE, 0x3200);
        w.rig.answer(ACTOR_CELL_8D6F30, 0x3300);
        w.rig.e.mem.set_u8(w.cover + 0x74, 1);
        let (_, log) = w.rig.run(0x009d_25b0, &args![t]);
        assert_eq!(calls_to(&log, TIMER_START), vec![vec![t + 0x54, bits(1.0)]]);
        assert_eq!(
            calls_to(&log, CALCULATE_COVER_POSITIONS),
            vec![vec![w.controller, w.cover, t + 0x18, t + 0x24]]
        );
        // The look-at point is the fire point raised by 0.75 * 100.
        assert_eq!(w.rig.e.mem.f32(t + 0x38), 80.0);
        let request = calls_to(&log, PATHING_REQUEST_CONSTRUCT)[0][0];
        assert_eq!(
            calls_to(&log, BUILD_REQUEST),
            vec![vec![w.attacker, request, t + 0x18, 0x3300, 0x3200, 0, 0]]
        );
        assert_eq!(state(&w), 6);
        assert_eq!(w.rig.e.mem.u8(w.cover + 0x74), 0);
        assert_eq!(calls_to(&log, SET_MOVE_MODE), vec![vec![t, 3]]);
        assert_eq!(
            calls_to(&log, PATHING_REQUEST_DESTRUCT),
            vec![vec![request]]
        );
    }

    #[test]
    fn update_reevaluates_the_cover() {
        // An unsafe cover while the target is seen: holds its ground.
        let mut w = world();
        let t = w.t;
        w.rig.e.set(w.this, CombatProcedureBeInCover::eState, 1);
        w.rig.answer(TIMER_EXPIRED, 1);
        w.rig.answer(STATE_BYTE_71, 1);
        w.rig.answer(ACTOR_TEST_8A8870, 1);
        let (_, log) = w.rig.run(0x009d_25b0, &args![t]);
        assert_eq!(
            calls_to(&log, CHECK_COVER_SAFE),
            vec![vec![0x5100, w.cover + 0x14, t + 0x18, w.target, 1]]
        );
        assert_eq!(state(&w), 7);
        assert_eq!(calls_to(&log, STOP_MOVING_ACTOR), vec![vec![w.attacker]]);
        assert_eq!(
            calls_to(&log, TIMER_START),
            vec![vec![t + 0x54, bits(1.0)], vec![t + 0x5c, bits(2.0)]]
        );

        // Not seen and far from the cover: the cover location is marked.
        let mut w = world();
        w.rig.e.set(w.this, CombatProcedureBeInCover::eState, 1);
        w.rig.answer(TIMER_EXPIRED, 1);
        w.rig.answer(ACTOR_TEST_8A8870, 1);
        w.rig.answer_float(POINT_LENGTH_SQUARED, 600_000.0);
        w.rig.answer(GROUP_TEST_91A0, 1);
        let (_, log) = w.rig.run(0x009d_25b0, &args![w.t]);
        assert_eq!(
            calls_to(&log, GROUP_TEST_91A0),
            vec![vec![0x5200, w.target]]
        );
        assert_eq!(calls_to(&log, STORE_BYTE_0), vec![vec![w.cover, 1]]);
    }

    #[test]
    fn update_moves_out_fires_and_moves_back() {
        // Behind cover with the wait over: out to the fire position.
        let mut w = world();
        let t = w.t;
        w.rig.e.set(w.this, CombatProcedureBeInCover::eState, 1);
        w.rig.answer(TIMER_EXPIRED, 0);
        w.rig.queue(TIMER_EXPIRED, &[0, 1]);
        w.rig.answer(CONTROLLER_BYTE_D4, 1);
        w.rig.answer(COVER_TEST_4820, 1);
        w.rig.e.mem.set_u8(w.cover + 0x4c, 2);
        w.rig.slot(SLOT_LOOK_POINT, 0x4400);
        let (_, log) = w.rig.run(0x009d_25b0, &args![t]);
        assert_eq!(state(&w), 2);
        let request = calls_to(&log, PATHING_REQUEST_CONSTRUCT)[0][0];
        assert_eq!(calls_to(&log, BUILD_REQUEST)[0][2], t + 0x24);
        assert_eq!(calls_to(&log, SET_LOOK_AT_TARGET), vec![vec![t, 0x4400, 1]]);
        assert_eq!(calls_to(&log, SET_MOVE_MODE), vec![vec![t, 2]]);
        assert_eq!(
            calls_to(&log, PATHING_REQUEST_DESTRUCT),
            vec![vec![request]]
        );

        // Low on ammunition behind cover: reloads.
        let mut w = world();
        w.rig.e.set(w.this, CombatProcedureBeInCover::eState, 1);
        w.rig.answer_float(TIMER_ELAPSED, 1.0);
        w.rig.answer_float(GET_CURRENT_AMMO_CLIP_PERCENT, 0.1);
        w.rig.setting(SETTING_COVER_WAIT_RELOAD_CLIP_PERCENT, 0.25);
        let (_, log) = w.rig.run(0x009d_25b0, &args![w.t]);
        assert_eq!(calls_to(&log, QUEUE_RELOAD), vec![vec![w.attacker]]);

        // Out of cover, the target seen after the wait began: firing.
        let mut w = world();
        let t = w.t;
        w.rig.e.set(w.this, CombatProcedureBeInCover::eState, 3);
        w.rig.queue_floats(TIMESTAMP_TIME, &[5.0, 4.0]);
        w.rig.answer_float(RANDOM_FLOAT, 2.5);
        let (_, log) = w.rig.run(0x009d_25b0, &args![t]);
        assert_eq!(state(&w), 4);
        assert_eq!(
            calls_to(&log, RANDOM_FLOAT),
            vec![vec![bits(3.0), bits(4.0)]]
        );
        assert!(calls_to(&log, TIMER_START).contains(&vec![t + 0x4c, bits(2.5)]));
        assert_eq!(calls_to(&log, TIMER_CLEAR), vec![vec![t + 0x64]]);

        // Firing with the clip low: back in.
        let mut w = world();
        w.rig.e.set(w.this, CombatProcedureBeInCover::eState, 4);
        w.rig.answer_float(GET_CURRENT_AMMO_CLIP_PERCENT, 0.1);
        w.rig
            .setting(SETTING_COVER_FIRING_RELOAD_CLIP_PERCENT, 0.25);
        let (_, log) = w.rig.run(0x009d_25b0, &args![w.t]);
        assert_eq!(state(&w), 5);
        assert_eq!(calls_to(&log, BUILD_REQUEST)[0][2], w.t + 0x18);

        // Moving in close to the cover: rotating in (state 6).
        let mut w = world();
        w.rig.e.set(w.this, CombatProcedureBeInCover::eState, 5);
        w.rig.answer_float(POINT_LENGTH_SQUARED, 100.0);
        let (_, log) = w.rig.run(0x009d_25b0, &args![w.t]);
        assert_eq!(state(&w), 6);
        assert_eq!(calls_to(&log, SET_MOVE_MODE), vec![vec![w.t, 1]]);

        // Rotating in done: waiting behind cover.
        let mut w = world();
        w.rig.e.set(w.this, CombatProcedureBeInCover::eState, 6);
        w.rig.answer(IS_PATHING_COMPLETE, 1);
        w.rig.answer_float(RANDOM_FLOAT, 1.5);
        let (_, log) = w.rig.run(0x009d_25b0, &args![w.t]);
        assert_eq!(state(&w), 1);
        assert_eq!(
            calls_to(&log, RANDOM_FLOAT),
            vec![vec![bits(1.0), bits(2.0)]]
        );
        assert!(calls_to(&log, TIMER_START).contains(&vec![w.t + 0x3c, bits(1.5)]));
    }

    #[test]
    fn update_handles_failed_paths_and_holding() {
        let mut w = world();
        w.rig.e.set(w.this, CombatProcedureBeInCover::eState, 2);
        w.rig.answer(ACTOR_PATH_TEST, 1);
        w.rig.answer(IS_PATH_VALID, 1);
        let (_, log) = w.rig.run(0x009d_25b0, &args![w.t]);
        assert_eq!(
            calls_to(&log, FAIL_PROCEDURE),
            vec![vec![w.t, TEXT_PATHING_FAILED_OUT]]
        );

        // Holding ground: a quarter into the timer with the target close.
        let mut w = world();
        w.rig.e.set(w.this, CombatProcedureBeInCover::eState, 7);
        w.rig.answer_float(TIMER_ELAPSED, 1.0);
        w.rig.answer_float(TIMER_DURATION, 2.0);
        w.rig.answer_float(POINT_LENGTH_SQUARED, 1000.0);
        let (_, log) = w.rig.run(0x009d_25b0, &args![w.t]);
        assert_eq!(calls_to(&log, CLEAR_COVER_LOCATION), vec![vec![0x5100, 0]]);
        assert_eq!(
            calls_to(&log, FAIL_PROCEDURE),
            vec![vec![w.t, TEXT_TARGET_HAS_LOS]]
        );

        // Holding after a reevaluation: back to the cover.
        let mut w = world();
        w.rig.e.set(w.this, CombatProcedureBeInCover::eState, 7);
        w.rig.queue(TIMER_EXPIRED, &[1]);
        w.rig.answer(STATE_BYTE_71, 1);
        w.rig.answer(CHECK_COVER_SAFE, 1);
        let (_, log) = w.rig.run(0x009d_25b0, &args![w.t]);
        assert_eq!(state(&w), 5);
        assert_eq!(calls_to(&log, BUILD_REQUEST)[0][2], w.t + 0x18);
    }

    #[test]
    fn fn_009d32e0_009d3310_and_009d3340_draw_style_times() {
        let mut w = world();
        let style = w.style;
        w.rig.e.mem.set_f32(style + 0x28, 7.0);
        w.rig.e.mem.set_f32(style + 0x2c, 8.0);
        w.rig.answer_float(RANDOM_FLOAT, 1.25);
        for (addr, low, high) in [
            (0x009d_32e0u32, 1.0f32, 2.0f32),
            (0x009d_3310, 7.0, 8.0),
            (0x009d_3340, 3.0, 4.0),
        ] {
            let (result, log) = w.rig.run(addr, &args![style]);
            assert_eq!(result.f64(), 1.25);
            assert_eq!(
                calls_to(&log, RANDOM_FLOAT),
                vec![vec![bits(low), bits(high)]]
            );
        }
    }

    #[test]
    fn fn_009d3390_tests_the_cover_location() {
        let mut w = world();
        let cover = w.cover;
        let (result, log) = w.rig.run(0x009d_3390, &args![cover]);
        assert!(!result.bool());
        assert_eq!(calls_to(&log, COVER_TEST_2BE0), vec![vec![cover + 0x14]]);
        assert_eq!(calls_to(&log, COVER_TEST_4820), vec![vec![cover + 0x14]]);
        w.rig.e.mem.set_u8(cover + 0x74, 1);
        assert!(w.rig.run(0x009d_3390, &args![cover]).0.bool());
        w.rig.answer(COVER_TEST_2BE0, 1);
        let (result, log) = w.rig.run(0x009d_3390, &args![cover]);
        assert!(result.bool());
        assert!(calls_to(&log, COVER_TEST_4820).is_empty());
    }

    #[test]
    fn fired_projectile_restarts_or_fires() {
        let mut w = world();
        w.rig.e.set(w.this, CombatProcedureBeInCover::eState, 1);
        w.rig.answer_float(RANDOM_FLOAT, 1.5);
        let (_, log) = w.rig.run(0x009d_33e0, &args![w.t, 0u32]);
        assert_eq!(
            calls_to(&log, TIMER_START),
            vec![vec![w.t + 0x3c, bits(1.5)]]
        );
        w.rig.e.set(w.this, CombatProcedureBeInCover::eState, 2);
        let (_, log) = w.rig.run(0x009d_33e0, &args![w.t, 0u32]);
        assert_eq!(state(&w), 4);
        assert_eq!(calls_to(&log, STOP_MOVING_ACTOR), vec![vec![w.attacker]]);
        assert_eq!(
            calls_to(&log, TIMER_START),
            vec![vec![w.t + 0x4c, bits(1.5)]]
        );
        let (_, log) = w.rig.run(0x009d_33e0, &args![w.t, 0u32]);
        assert!(calls_to(&log, TIMER_START).is_empty());
    }

    #[test]
    fn display_debug_text_prints_the_state_and_the_locations() {
        let mut w = world();
        let text = w.rig.object(8);
        w.rig.e.set(w.this, CombatProcedureBeInCover::eState, 3);
        w.rig.answer_float(TIMER_DURATION, 2.0);
        w.rig.answer_float(TIMER_ELAPSED, 0.5);
        w.rig.e.mem.set_f32(w.t + 0x18, 1.0);
        w.rig.e.mem.set_f32(w.t + 0x2c, 9.0);
        let (_, log) = w.rig.run(0x009d_3470, &args![w.t, text, 5u32, 6u32]);
        let printed = slot_calls(&log, SLOT_PRINT);
        let formats: Vec<_> = printed.iter().map(|c| c[3]).collect();
        assert_eq!(
            formats,
            vec![
                TEXT_WAITING_OUT_OF_COVER,
                TEXT_WAITING_TIMER,
                TEXT_TARGET_BLOCKED_TIMER,
                TEXT_REEVALUATE_TIMER,
                TEXT_COVER_LOCATION,
                TEXT_FIRE_LOCATION
            ]
        );
        let mut cover = vec![text, 5, 6, TEXT_COVER_LOCATION];
        cover.extend(args![1.0f64, 0.0f64, 0.0f64]);
        assert_eq!(printed[4], cover);
        let mut fire = vec![text, 5, 6, TEXT_FIRE_LOCATION];
        fire.extend(args![0.0f64, 0.0f64, 9.0f64]);
        assert_eq!(printed[5], fire);
    }

    #[test]
    fn save_and_load_game_keep_the_cover_location() {
        let mut w = world();
        let t = w.t;
        w.rig.answer(STATE_COVER_LOCATION, w.cover);
        let saved = Rc::new(RefCell::new(Vec::new()));
        let record = Rc::clone(&saved);
        w.rig.e.register_double(SAVE_BYTES, move |e, a| {
            record.borrow_mut().push((a[1], a[2], e.mem.u8(a[1])));
            Ret::default()
        });
        let (_, log) = w.rig.run(0x009d_3800, &args![t, 0x40u32]);
        let saved = saved.borrow();
        assert_eq!(saved[0], (t + 0x10, 4, 0));
        assert_eq!((saved[1].1, saved[1].2), (1, 1));
        assert_eq!(
            saved[2..].iter().map(|s| s.0).collect::<Vec<_>>(),
            vec![t + 0x18, t + 0x24, t + 0x30]
        );
        assert_eq!(calls_to(&log, TIMER_SAVE).len(), 6);

        w.rig.answer(STATE_LAST_COVER_LOCATION, 0x7800);
        let (_, log) = w.rig.run(0x009d_38f0, &args![t, 0x40u32]);
        assert_eq!(w.rig.e.mem.u32(t + 0x14), 0x7800);
        assert_eq!(calls_to(&log, LOAD_BYTES).len(), 5);
        assert_eq!(calls_to(&log, TIMER_LOAD).len(), 6);
    }
}
