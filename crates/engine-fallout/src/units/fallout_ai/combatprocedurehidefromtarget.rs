//! `fallout/ai/combat/combatprocedurehidefromtarget.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 009d6120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `01091a64` (RTTI `.?AVCombatProcedureHideFromTarget@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `009ca6c0`. One word, plain `RET`, returns nothing.
pub fn fn_009d6120(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0109_1a64);
    e.call(0x009c_a6c0, &args![this]);
}

layout! {
    /// `CombatProcedureHideFromTarget` (Xbox PDB), 0x40 bytes; the offsets
    /// are the ones the PC code uses.
    pub struct CombatProcedureHideFromTarget: 0x40 {
        /// `pCombatController` (Xbox PDB, `CombatProcedure`): `CombatController*`.
        0x04 pCombatController: Ptr,
        /// `eState` (Xbox PDB): 0 initializing, 1 moving to cover, 2 hiding
        /// (the debug texts).
        0x10 eState: u32,
        /// `fNumSecondsToHide` (Xbox PDB).
        0x14 fNumSecondsToHide: f32,
        /// `pCoverLocation` (Xbox PDB): `CombatCoverLocation*`.
        0x38 pCoverLocation: Ptr,
        /// `bIgnoreLOS` (Xbox PDB).
        0x3C bIgnoreLOS: bool,
        /// `bClearCoverWhenFinished` (Xbox PDB).
        0x3D bClearCoverWhenFinished: bool,
    }
}

/// The embedded members (Xbox PDB names): `HidingTimer` (`CombatTimer`),
/// `HidePosition` and `LookAtPosition` (`NiPoint3`).
const HIDING_TIMER: u32 = 0x18;
const HIDE_POSITION: u32 = 0x20;
const LOOK_AT_POSITION: u32 = 0x2c;
/// `CombatProcedureHideFromTarget`'s vtable (`01091a64`, RTTI
/// `.?AVCombatProcedureHideFromTarget@@`, as `fn_009d6120` stores it).
const VTABLE: u32 = 0x0109_1a64;

/// `CombatProcedure` methods (Xbox PDB names), its constructor `009ca680`,
/// `009caac0` (fail with a reason) and `009caae0` (complete).
const BASE_CONSTRUCT: u32 = 0x009c_a680;
const SET_MOVE_MODE: u32 = 0x0099_6e20;
const CLEAR_MOVE_MODE: u32 = 0x0099_6e40;
const SET_LOOK_AT_TARGET: u32 = 0x0099_6e60;
const CLEAR_LOOK_AT_TARGET: u32 = 0x0099_6ea0;
const BASE_SAVE_GAME: u32 = 0x0099_6ed0;
const BASE_LOAD_GAME: u32 = 0x0099_6ef0;
const FAIL_PROCEDURE: u32 = 0x009c_aac0;
const COMPLETE_PROCEDURE: u32 = 0x009c_aae0;
/// `operator delete` (`00401030`, `__cdecl`).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// Controller getters: `0097ae90` the actor, `004030b0` the target
/// (`+0xc0`), `00586150` the combat state (`+0x9c`).
const CONTROLLER_ACTOR: u32 = 0x0097_ae90;
const CONTROLLER_TARGET: u32 = 0x0040_30b0;
const GET_COMBAT_STATE: u32 = 0x0058_6150;
/// Combat state methods: `CombatState::IsCoverLocationValid`,
/// `CombatState::ClearCoverLocation` (Xbox PDB), `0097fb20` (the byte
/// `+0x71`), `007058c0` (the cover record `+0xc4`), `0097fb60` / `0097b050`
/// (the cover locations `+0xe8` / `+0xe4`), `0097ce70` (stores the byte
/// `+0xd8`; the engine map's name `BSFaceGenNiNode::SetApplyRotToParent`
/// belongs to a folded method).
const IS_COVER_LOCATION_VALID: u32 = 0x009a_0600;
const CLEAR_COVER_LOCATION: u32 = 0x009a_03d0;
const STATE_BYTE_71: u32 = 0x0097_fb20;
const STATE_COVER_RECORD: u32 = 0x0070_58c0;
const STATE_COVER_LOCATION: u32 = 0x0097_fb60;
const STATE_LAST_COVER_LOCATION: u32 = 0x0097_b050;
const STATE_SET_BYTE_D8: u32 = 0x0097_ce70;
/// `CombatUtilities` (Xbox PDB, `__cdecl`).
const CALCULATE_COVER_POSITIONS: u32 = 0x009a_9370;
const GET_TARGET_FIRING_Z_OFFSET: u32 = 0x009a_8460;
const GET_TARGET_FIRING_LOCATION: u32 = 0x009a_8600;
/// Actor methods (Xbox PDB names) and unnamed helpers: `008b3b90` (a path
/// test), `008d6f30` (its cell).
const STOP_MOVING_ACTOR: u32 = 0x008b_3ab0;
const IS_PATH_VALID: u32 = 0x008b_3960;
const IS_PATHING_COMPLETE: u32 = 0x008b_3bb0;
const SET_PATHFINDING_GOAL: u32 = 0x008b_3630;
const BUILD_REQUEST: u32 = 0x008b_3880;
const ACTOR_PATH_TEST: u32 = 0x008b_3b90;
const ACTOR_CELL_8D6F30: u32 = 0x008d_6f30;
const GET_WORLD_SPACE: u32 = 0x0057_5d70;
const GET_WORLD_LOCATION: u32 = 0x0057_5650;
/// `BGSWorldLocation::GetDistanceSquared` (Xbox PDB).
const WORLD_LOCATION_DISTANCE_SQUARED: u32 = 0x0052_7a40;
/// Actor vtable slots: `+0x1f4` the position, `+0x2bc` the heading.
const SLOT_POSITION: u32 = 0x1f4;
const SLOT_HEADING: u32 = 0x2bc;
/// `PathingRequest::PathingRequest` (Xbox PDB), its destructor `006e2620`
/// and `006e2bb0` (sets the heading).
const PATHING_REQUEST_CONSTRUCT: u32 = 0x006e_2420;
const PATHING_REQUEST_DESTRUCT: u32 = 0x006e_2620;
const REQUEST_SET_HEADING: u32 = 0x006e_2bb0;
/// Point and timer helpers: `00439ef0` (difference into the out point),
/// `004a7290` (squared length), `006815c0` (a constructor returning
/// `this`), `00906f60` (the timer's constructor), `008d7f40` (start),
/// `008d7f80` (ran out), `00435e00` (elapsed), `006b9130` (duration),
/// `006a7f50` (a time stamp's time), `00435dd0` (now), and the
/// `CombatTimer::SaveGame` / `LoadGame` (Xbox PDB).
const POINT_DIFFERENCE: u32 = 0x0043_9ef0;
const POINT_LENGTH_SQUARED: u32 = 0x004a_7290;
const POINT_CONSTRUCT: u32 = 0x0068_15c0;
const TIMER_CONSTRUCT: u32 = 0x0090_6f60;
const TIMER_START: u32 = 0x008d_7f40;
const TIMER_EXPIRED: u32 = 0x008d_7f80;
const TIMER_ELAPSED: u32 = 0x0043_5e00;
const TIMER_DURATION: u32 = 0x006b_9130;
const TIMESTAMP_TIME: u32 = 0x006a_7f50;
const TIME_NOW: u32 = 0x0043_5dd0;
const TIMER_SAVE: u32 = 0x009a_5eb0;
const TIMER_LOAD: u32 = 0x009a_5f20;
/// `00403e20` (`__thiscall` on a setting): the address of its value;
/// `fCombatHideFailedTargetDistance` (named by its static initializer).
const SETTING_VALUE: u32 = 0x0040_3e20;
const SETTING_COMBAT_HIDE_FAILED_TARGET_DISTANCE: u32 = 0x011c_fde4;
/// `double` constants of the exe: 1, 4096 and 625 (`01091b28`).
const ONE_DOUBLE: u32 = 0x0101_2070;
const FOUR_THOUSAND_NINETY_SIX_DOUBLE: u32 = 0x0101_7a10;
const HIDE_DISTANCE_SQUARED_DOUBLE: u32 = 0x0109_1b28;
/// `BGSSaveGameBuffer` / `BGSLoadGameBuffer` byte writers and readers.
const SAVE_BYTES: u32 = 0x0086_5e50;
const LOAD_BYTES: u32 = 0x0086_4980;
/// The debug-text object's print slot (`__cdecl` through the vtable).
const SLOT_PRINT: u32 = 0x14;
/// Strings.
const TEXT_INITIALIZING: u32 = 0x0109_0b68;
const TEXT_PATH_BUILDING_FAILED: u32 = 0x0109_1aa8;
const TEXT_PATHING_FAILED: u32 = 0x0109_1ad0;
const TEXT_CANNOT_SETUP_PATH: u32 = 0x0109_1b00;
const TEXT_HIDING_TIMER: u32 = 0x0109_1b30;
const TEXT_HIDING: u32 = 0x0109_1b4c;
const TEXT_MOVING_TO_COVER: u32 = 0x0109_1b54;

fn controller_of(e: &Engine, this: Ptr<CombatProcedureHideFromTarget>) -> u32 {
    e.get(this, CombatProcedureHideFromTarget::pCombatController)
        .addr()
}

fn copy_point(e: &mut Engine, from: u32, to: u32) {
    for i in 0..3 {
        let word = e.mem.u32(from + i * 4);
        e.mem.set_u32(to + i * 4, word);
    }
}

// Translated from 009d2570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureHideFromTarget::Shutdown` (Xbox PDB; `CombatProcedureBeInCover`'s
/// vtable points at the same body): stops the controller's actor and
/// clears the move mode and the look-at target.
pub fn shutdown(e: &mut Engine, this: Ptr<CombatProcedureHideFromTarget>) {
    let controller = controller_of(e, this);
    let actor = e.call(CONTROLLER_ACTOR, &args![controller]).u32();
    e.call(STOP_MOVING_ACTOR, &args![actor]);
    e.call(CLEAR_MOVE_MODE, &args![this]);
    e.call(CLEAR_LOOK_AT_TARGET, &args![this]);
}

// Translated from 009d6030 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009d6030 - the compiler's FS:[0] exception frame (unwinding of the base class if a member constructor throws) is not translated
/// `CombatProcedureHideFromTarget::CombatProcedureHideFromTarget` (Xbox
/// PDB; four words): the base class, the vtable, state 0, the seconds to
/// hide, the timer and the two points, the cover location and the two
/// flags. Returns `this`.
pub fn construct(
    e: &mut Engine,
    this: Ptr<CombatProcedureHideFromTarget>,
    cover_location: u32,
    seconds: f32,
    ignore_los: bool,
    clear_cover_when_finished: bool,
) -> Ptr<CombatProcedureHideFromTarget> {
    let t = this.addr();
    e.call(BASE_CONSTRUCT, &args![t]);
    e.mem.set_u32(t, VTABLE);
    e.set(this, CombatProcedureHideFromTarget::eState, 0);
    e.mem.set_u32(t + 0x14, x87_quiet_bits(seconds.to_bits()));
    e.call(TIMER_CONSTRUCT, &args![t + HIDING_TIMER]);
    e.call(POINT_CONSTRUCT, &args![t + HIDE_POSITION]);
    e.call(POINT_CONSTRUCT, &args![t + LOOK_AT_POSITION]);
    e.mem.set_u32(t + 0x38, cover_location);
    e.set(this, CombatProcedureHideFromTarget::bIgnoreLOS, ignore_los);
    e.set(
        this,
        CombatProcedureHideFromTarget::bClearCoverWhenFinished,
        clear_cover_when_finished,
    );
    this
}

// Translated from 009d60e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The procedure type of `CombatProcedureHideFromTarget` (its vtable's
/// `GetType` slot): 9.
pub fn get_type(_e: &mut Engine, _this: Ptr<CombatProcedureHideFromTarget>) -> u32 {
    9
}

// Translated from 009d60f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor (the vtable's slot 0): the destructor
/// body `009d6120`, then `operator delete` when bit 0 of `flags` is set.
/// Returns `this`.
pub fn scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<CombatProcedureHideFromTarget>,
    flags: u32,
) -> Ptr<CombatProcedureHideFromTarget> {
    fn_009d6120(e, this.cast());
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 009d6140 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009d6140 - the compiler's FS:[0] exception frame (unwinding of the pathing request) is not translated
/// `CombatProcedureHideFromTarget::Update` (Xbox PDB). State 0: with a
/// valid cover location the hide and look-at positions are the cover
/// positions (the look-at height raised by the target's firing offset);
/// otherwise the actor stays and looks at the target's firing location.
/// Further than 25 (625 squared) from the hide position (with cover) it
/// paths there (state 1, failing "Cannot setup path to hiding location");
/// otherwise it hides at once. State 1: a failed path fails; a finished one
/// starts the hiding timer (state 2). State 2: when the timer ran out it
/// completes (clearing the cover when asked); a second into hiding, unless
/// the actor is within 64 of the combat state's cover record, it gives up
/// the cover and completes when the target sees the cover (byte `+0x71`)
/// and LOS is not ignored, when the target came within
/// `fCombatHideFailedTargetDistance`, or when the cover location's time
/// (`009d6730`) is later.
///
/// Locals follow the disassembly's frame (offsets below `EBP`).
pub fn update(e: &mut Engine, this: Ptr<CombatProcedureHideFromTarget>) {
    const FRAME: u32 = 0x140;
    let t = this.addr();
    e.with_stack(FRAME, |e, base| {
        let ebp = base.addr() + FRAME;
        let controller = controller_of(e, this);
        let actor = e.call(CONTROLLER_ACTOR, &args![controller]).u32();
        let controller = controller_of(e, this);
        let target = e.call(CONTROLLER_TARGET, &args![controller]).u32();
        let controller = controller_of(e, this);
        let combat_state = e.call(GET_COMBAT_STATE, &args![controller]).u32();
        match e.get(this, CombatProcedureHideFromTarget::eState) {
            0 => {
                let cover = e.mem.u32(t + 0x38);
                let valid = e
                    .call(IS_COVER_LOCATION_VALID, &args![combat_state, cover])
                    .bool();
                if valid {
                    let cover = e.mem.u32(t + 0x38);
                    e.mem.set_u8(cover + 0x74, 1);
                    let controller = controller_of(e, this);
                    e.call(
                        CALCULATE_COVER_POSITIONS,
                        &args![controller, cover, t + HIDE_POSITION, t + LOOK_AT_POSITION],
                    );
                    let who = if target != 0 { target } else { actor };
                    let offset = e
                        .call(GET_TARGET_FIRING_Z_OFFSET, &args![who, 2u32, 0u32])
                        .f64();
                    let height = x87_add(offset, x87_m32(e.mem.u32(t + 0x34)));
                    e.mem.set_u32(t + 0x34, x87_store32(height));
                } else {
                    let position = e.vcall(actor, SLOT_POSITION, &args![]).u32();
                    copy_point(e, position, t + HIDE_POSITION);
                    if target != 0 {
                        let location = e
                            .call(
                                GET_TARGET_FIRING_LOCATION,
                                &args![ebp - 0x114, target, 2u32],
                            )
                            .u32();
                        copy_point(e, location, t + LOOK_AT_POSITION);
                    }
                }
                let difference = ebp - 0x28;
                let position = e.vcall(actor, SLOT_POSITION, &args![]).u32();
                e.call(
                    POINT_DIFFERENCE,
                    &args![position, difference, t + HIDE_POSITION],
                );
                let far = valid && {
                    let squared = e.call(POINT_LENGTH_SQUARED, &args![difference]).f64();
                    squared > e.global::<f64>(HIDE_DISTANCE_SQUARED_DOUBLE)
                };
                if far {
                    e.call(SET_MOVE_MODE, &args![t, 3u32]);
                    let request = ebp - 0xdc;
                    e.call(PATHING_REQUEST_CONSTRUCT, &args![request]);
                    let worldspace = e.call(GET_WORLD_SPACE, &args![actor]).u32();
                    let cell = e.call(ACTOR_CELL_8D6F30, &args![actor]).u32();
                    e.call(
                        BUILD_REQUEST,
                        &args![
                            actor,
                            request,
                            t + HIDE_POSITION,
                            cell,
                            worldspace,
                            0.0f32.to_bits(),
                            0u32
                        ],
                    );
                    let heading = e.vcall(actor, SLOT_HEADING, &args![0u32]).f64();
                    e.call(REQUEST_SET_HEADING, &args![request, x87_store32(heading)]);
                    if !e.call(SET_PATHFINDING_GOAL, &args![actor, request]).bool() {
                        e.call(FAIL_PROCEDURE, &args![t, TEXT_CANNOT_SETUP_PATH]);
                        e.call(PATHING_REQUEST_DESTRUCT, &args![request]);
                        return;
                    }
                    e.set(this, CombatProcedureHideFromTarget::eState, 1);
                    e.call(PATHING_REQUEST_DESTRUCT, &args![request]);
                } else {
                    let seconds = x87_quiet_bits(e.mem.u32(t + 0x14));
                    e.call(TIMER_START, &args![t + HIDING_TIMER, seconds]);
                    if target != 0 {
                        e.call(SET_LOOK_AT_TARGET, &args![t, t + LOOK_AT_POSITION, 3u32]);
                    }
                    e.call(SET_MOVE_MODE, &args![t, 2u32]);
                    e.set(this, CombatProcedureHideFromTarget::eState, 2);
                }
            }
            1 => {
                e.call(SET_MOVE_MODE, &args![t, 3u32]);
                if e.call(ACTOR_PATH_TEST, &args![actor]).bool() {
                    let reason = if e.call(IS_PATH_VALID, &args![actor]).bool() {
                        TEXT_PATHING_FAILED
                    } else {
                        TEXT_PATH_BUILDING_FAILED
                    };
                    e.call(FAIL_PROCEDURE, &args![t, reason]);
                } else if e.call(IS_PATHING_COMPLETE, &args![actor]).bool() {
                    let seconds = x87_quiet_bits(e.mem.u32(t + 0x14));
                    e.call(TIMER_START, &args![t + HIDING_TIMER, seconds]);
                    e.call(SET_LOOK_AT_TARGET, &args![t, t + LOOK_AT_POSITION, 3u32]);
                    e.call(SET_MOVE_MODE, &args![t, 2u32]);
                    e.set(this, CombatProcedureHideFromTarget::eState, 2);
                }
            }
            2 => {
                if e.call(TIMER_EXPIRED, &args![t + HIDING_TIMER]).bool() {
                    if e.get(this, CombatProcedureHideFromTarget::bClearCoverWhenFinished) {
                        e.call(CLEAR_COVER_LOCATION, &args![combat_state, 0u32]);
                    }
                    e.call(COMPLETE_PROCEDURE, &args![t]);
                    return;
                }
                let started = e.call(TIMESTAMP_TIME, &args![t + HIDING_TIMER]).f64();
                let settled = x87_store32(x87_add(started, e.global::<f64>(ONE_DOUBLE)));
                let now = e.call(TIME_NOW, &args![]).f64();
                // FLD settled; FCOMPP; TEST AH,5; JP: only when it is earlier.
                let mut check = x87_load32(settled) < now;
                if check {
                    let record = e.call(STATE_COVER_RECORD, &args![combat_state]).u32();
                    if record != 0 && e.mem.u8(record) == 1 {
                        let location = e.call(GET_WORLD_LOCATION, &args![actor, ebp - 0x124]).u32();
                        let squared = e
                            .call(
                                WORLD_LOCATION_DISTANCE_SQUARED,
                                &args![record + 8, location],
                            )
                            .f64();
                        if squared < e.global::<f64>(FOUR_THOUSAND_NINETY_SIX_DOUBLE) {
                            check = false;
                        }
                    }
                }
                if !check {
                    return;
                }
                let give_up = |e: &mut Engine| {
                    let controller = controller_of(e, this);
                    let state = e.call(GET_COMBAT_STATE, &args![controller]).u32();
                    e.call(CLEAR_COVER_LOCATION, &args![state, 1u32]);
                    e.call(STATE_SET_BYTE_D8, &args![combat_state, 0u32]);
                    e.call(COMPLETE_PROCEDURE, &args![t]);
                };
                if e.call(STATE_BYTE_71, &args![combat_state]).bool() {
                    if !e.get(this, CombatProcedureHideFromTarget::bIgnoreLOS) {
                        give_up(e);
                        return;
                    }
                    if target != 0 {
                        let actor_location = ebp - 0x108;
                        let target_location = ebp - 0xf8;
                        e.call(GET_WORLD_LOCATION, &args![actor, actor_location]);
                        e.call(GET_WORLD_LOCATION, &args![target, target_location]);
                        let squared = e
                            .call(
                                WORLD_LOCATION_DISTANCE_SQUARED,
                                &args![actor_location, target_location],
                            )
                            .f64();
                        let first = e
                            .call(
                                SETTING_VALUE,
                                &args![SETTING_COMBAT_HIDE_FAILED_TARGET_DISTANCE],
                            )
                            .u32();
                        let second = e
                            .call(
                                SETTING_VALUE,
                                &args![SETTING_COMBAT_HIDE_FAILED_TARGET_DISTANCE],
                            )
                            .u32();
                        let limit =
                            x87_mul(x87_load32(e.mem.u32(first)), x87_m32(e.mem.u32(second)));
                        if limit > squared {
                            give_up(e);
                            return;
                        }
                    }
                }
                let stamp = fn_009d6730(e, combat_state, ebp - 0x128);
                let cover_time = e.call(TIMESTAMP_TIME, &args![stamp]).f64();
                if x87_load32(settled) < cover_time {
                    give_up(e);
                }
            }
            _ => {}
        }
    });
}

// Translated from 009d6730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the word at `+0xa0` (a time stamp of the combat state) to `out`
/// and returns `out` (one word).
pub fn fn_009d6730(e: &mut Engine, this: u32, out: u32) -> u32 {
    let word = e.mem.u32(this + 0xa0);
    e.mem.set_u32(out, word);
    out
}

// Translated from 009d6750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureHideFromTarget::DisplayDebugText` (Xbox PDB): the state,
/// and while hiding the hiding timer.
pub fn display_debug_text(
    e: &mut Engine,
    this: Ptr<CombatProcedureHideFromTarget>,
    text: u32,
    first: u32,
    second: u32,
) {
    let t = this.addr();
    match e.get(this, CombatProcedureHideFromTarget::eState) {
        0 => {
            e.vcall(text, SLOT_PRINT, &args![first, second, TEXT_INITIALIZING]);
        }
        1 => {
            e.vcall(
                text,
                SLOT_PRINT,
                &args![first, second, TEXT_MOVING_TO_COVER],
            );
        }
        2 => {
            e.vcall(text, SLOT_PRINT, &args![first, second, TEXT_HIDING]);
            let duration = e.call(TIMER_DURATION, &args![t + HIDING_TIMER]).f64();
            let elapsed = e.call(TIMER_ELAPSED, &args![t + HIDING_TIMER]).f64();
            e.vcall(
                text,
                SLOT_PRINT,
                &args![first, second, TEXT_HIDING_TIMER, elapsed, duration],
            );
        }
        _ => {}
    }
}

// Translated from 009d6820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureHideFromTarget::SaveGame` (Xbox PDB): the base class,
/// the state, the seconds, the timer, the two points, the two flags and
/// which of the combat state's cover locations the cover location is (2
/// the current one, 1 another, 0 none).
pub fn save_game(e: &mut Engine, this: Ptr<CombatProcedureHideFromTarget>, buffer: u32) {
    let t = this.addr();
    e.call(BASE_SAVE_GAME, &args![t, buffer]);
    e.call(SAVE_BYTES, &args![buffer, t + 0x10, 4u32, 0u32]);
    e.call(SAVE_BYTES, &args![buffer, t + 0x14, 4u32, 0u32]);
    e.call(TIMER_SAVE, &args![t + HIDING_TIMER, buffer]);
    e.call(SAVE_BYTES, &args![buffer, t + HIDE_POSITION, 0xcu32, 0u32]);
    e.call(
        SAVE_BYTES,
        &args![buffer, t + LOOK_AT_POSITION, 0xcu32, 0u32],
    );
    e.call(SAVE_BYTES, &args![buffer, t + 0x3c, 1u32, 0u32]);
    e.call(SAVE_BYTES, &args![buffer, t + 0x3d, 1u32, 0u32]);
    e.with_stack(4, |e, which| {
        let mut kind = 0u8;
        let cover = e.mem.u32(t + 0x38);
        if cover != 0 {
            let controller = controller_of(e, this);
            let combat_state = e.call(GET_COMBAT_STATE, &args![controller]).u32();
            let current = e.call(STATE_COVER_LOCATION, &args![combat_state]).u32();
            kind = u8::from(e.mem.u32(t + 0x38) == current) + 1;
        }
        e.mem.set_u8(which.addr(), kind);
        e.call(SAVE_BYTES, &args![buffer, which, 1u32, 0u32]);
    });
}

// Translated from 009d6900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureHideFromTarget::LoadGame` (Xbox PDB): the mirror of
/// `SaveGame`; the cover location becomes the combat state's current one
/// (2) or its last one (1).
pub fn load_game(e: &mut Engine, this: Ptr<CombatProcedureHideFromTarget>, buffer: u32) {
    let t = this.addr();
    e.call(BASE_LOAD_GAME, &args![t, buffer]);
    e.call(LOAD_BYTES, &args![buffer, t + 0x10, 4u32]);
    e.call(LOAD_BYTES, &args![buffer, t + 0x14, 4u32]);
    e.call(TIMER_LOAD, &args![t + HIDING_TIMER, buffer]);
    e.call(LOAD_BYTES, &args![buffer, t + HIDE_POSITION, 0xcu32]);
    e.call(LOAD_BYTES, &args![buffer, t + LOOK_AT_POSITION, 0xcu32]);
    e.call(LOAD_BYTES, &args![buffer, t + 0x3c, 1u32]);
    e.call(LOAD_BYTES, &args![buffer, t + 0x3d, 1u32]);
    e.with_stack(4, |e, which| {
        e.mem.set_u8(which.addr(), 0);
        e.call(LOAD_BYTES, &args![buffer, which, 1u32]);
        let kind = e.mem.u8(which.addr());
        if kind == 2 {
            let controller = controller_of(e, this);
            let combat_state = e.call(GET_COMBAT_STATE, &args![controller]).u32();
            let cover = e.call(STATE_COVER_LOCATION, &args![combat_state]).u32();
            e.mem.set_u32(t + 0x38, cover);
        } else if kind == 1 {
            let controller = controller_of(e, this);
            let combat_state = e.call(GET_COMBAT_STATE, &args![controller]).u32();
            let cover = e
                .call(STATE_LAST_COVER_LOCATION, &args![combat_state])
                .u32();
            e.mem.set_u32(t + 0x38, cover);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009d_6120, fn_009d6120(Ptr)),
        entry!(0x009d_2570, shutdown(Ptr<CombatProcedureHideFromTarget>)),
        entry!(
            0x009d_6030,
            construct(
                Ptr<CombatProcedureHideFromTarget>,
                u32,
                f32,
                bool,
                bool,
            ) -> Ptr<CombatProcedureHideFromTarget>
        ),
        entry!(
            0x009d_60e0,
            get_type(Ptr<CombatProcedureHideFromTarget>) -> u32
        ),
        entry!(
            0x009d_60f0,
            scalar_deleting_destructor(
                Ptr<CombatProcedureHideFromTarget>,
                u32,
            ) -> Ptr<CombatProcedureHideFromTarget>
        ),
        entry!(0x009d_6140, update(Ptr<CombatProcedureHideFromTarget>)),
        entry!(0x009d_6730, fn_009d6730(u32, u32) -> u32),
        entry!(
            0x009d_6750,
            display_debug_text(Ptr<CombatProcedureHideFromTarget>, u32, u32, u32)
        ),
        entry!(
            0x009d_6820,
            save_game(Ptr<CombatProcedureHideFromTarget>, u32)
        ),
        entry!(
            0x009d_6900,
            load_game(Ptr<CombatProcedureHideFromTarget>, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    #[test]
    fn fn_009d6120_stores_the_vtable_then_forwards_this() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcd; 8]);
        // The double copies the vtable word it finds when it runs to `+4`.
        e.register(0x009c_a6c0, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x009d_6120, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x009c_a6c0)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0109_1a64);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0109_1a64,
            "the vtable was stored before the call"
        );
    }

    use super::*;
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

    const CALLEES: &[u32] = &[
        BASE_CONSTRUCT,
        SET_MOVE_MODE,
        CLEAR_MOVE_MODE,
        SET_LOOK_AT_TARGET,
        CLEAR_LOOK_AT_TARGET,
        BASE_SAVE_GAME,
        BASE_LOAD_GAME,
        FAIL_PROCEDURE,
        COMPLETE_PROCEDURE,
        OPERATOR_DELETE,
        CONTROLLER_ACTOR,
        CONTROLLER_TARGET,
        GET_COMBAT_STATE,
        IS_COVER_LOCATION_VALID,
        CLEAR_COVER_LOCATION,
        STATE_BYTE_71,
        STATE_COVER_RECORD,
        STATE_COVER_LOCATION,
        STATE_LAST_COVER_LOCATION,
        STATE_SET_BYTE_D8,
        CALCULATE_COVER_POSITIONS,
        GET_TARGET_FIRING_Z_OFFSET,
        GET_TARGET_FIRING_LOCATION,
        STOP_MOVING_ACTOR,
        IS_PATH_VALID,
        IS_PATHING_COMPLETE,
        SET_PATHFINDING_GOAL,
        BUILD_REQUEST,
        ACTOR_PATH_TEST,
        ACTOR_CELL_8D6F30,
        GET_WORLD_SPACE,
        GET_WORLD_LOCATION,
        WORLD_LOCATION_DISTANCE_SQUARED,
        PATHING_REQUEST_CONSTRUCT,
        PATHING_REQUEST_DESTRUCT,
        REQUEST_SET_HEADING,
        POINT_DIFFERENCE,
        POINT_LENGTH_SQUARED,
        POINT_CONSTRUCT,
        TIMER_CONSTRUCT,
        TIMER_START,
        TIMER_EXPIRED,
        TIMER_ELAPSED,
        TIMER_DURATION,
        TIMESTAMP_TIME,
        TIME_NOW,
        TIMER_SAVE,
        TIMER_LOAD,
        SAVE_BYTES,
        LOAD_BYTES,
        // The base destructor `fn_009d6120` calls.
        0x009c_a6c0,
    ];

    struct World {
        rig: Rig,
        this: Ptr<CombatProcedureHideFromTarget>,
        t: u32,
        controller: u32,
        actor: u32,
        target: u32,
        position: u32,
        cover: u32,
    }

    fn world() -> World {
        let mut rig = Rig::new();
        let t = rig.e.mem.alloc(0x40);
        let this = Ptr::<CombatProcedureHideFromTarget>::new(t);
        let controller = rig.object(0x100);
        rig.e.set(
            this,
            CombatProcedureHideFromTarget::pCombatController,
            Ptr::new(controller),
        );
        let actor = rig.actor();
        let target = rig.actor();
        let position = rig.e.mem.alloc(0xc);
        rig.e.mem.set_f32(position, 5.0);
        let cover = rig.e.mem.alloc(0x80);
        rig.e.mem.set_u32(t + 0x38, cover);
        rig.e.mem.set_f32(t + 0x14, 6.0);
        rig.answer(CONTROLLER_ACTOR, actor);
        rig.answer(CONTROLLER_TARGET, target);
        rig.answer(GET_COMBAT_STATE, 0x5100);
        rig.slot(SLOT_POSITION, position);
        World {
            rig,
            this,
            t,
            controller,
            actor,
            target,
            position,
            cover,
        }
    }

    #[test]
    fn shutdown_stops_the_actor() {
        let mut w = world();
        let (_, log) = w.rig.run(0x009d_2570, &args![w.t]);
        assert_eq!(calls_to(&log, CONTROLLER_ACTOR), vec![vec![w.controller]]);
        assert_eq!(calls_to(&log, STOP_MOVING_ACTOR), vec![vec![w.actor]]);
        assert_eq!(calls_to(&log, CLEAR_MOVE_MODE), vec![vec![w.t]]);
        assert_eq!(calls_to(&log, CLEAR_LOOK_AT_TARGET), vec![vec![w.t]]);
    }

    #[test]
    fn construct_sets_up_the_procedure() {
        let mut w = world();
        let block = w.rig.e.mem.alloc(0x40);
        w.rig.e.mem.write(block, &[0xcc; 0x40]);
        let (result, log) = w.rig.run(
            0x009d_6030,
            &args![block, 0x7700u32, bits(4.5), true, false],
        );
        assert_eq!(result.u32(), block);
        assert_eq!(calls_to(&log, BASE_CONSTRUCT), vec![vec![block]]);
        assert_eq!(w.rig.e.mem.u32(block), VTABLE);
        assert_eq!(w.rig.e.mem.u32(block + 0x10), 0);
        assert_eq!(w.rig.e.mem.f32(block + 0x14), 4.5);
        assert_eq!(calls_to(&log, TIMER_CONSTRUCT), vec![vec![block + 0x18]]);
        assert_eq!(
            calls_to(&log, POINT_CONSTRUCT),
            vec![vec![block + 0x20], vec![block + 0x2c]]
        );
        assert_eq!(w.rig.e.mem.u32(block + 0x38), 0x7700);
        assert_eq!(w.rig.e.mem.u8(block + 0x3c), 1);
        assert_eq!(w.rig.e.mem.u8(block + 0x3d), 0);
    }

    #[test]
    fn get_type_and_the_deleting_destructor() {
        let mut w = world();
        assert_eq!(w.rig.run(0x009d_60e0, &args![w.t]).0.u32(), 9);
        let (result, log) = w.rig.run(0x009d_60f0, &args![w.t, 1u32]);
        assert_eq!(result.u32(), w.t);
        assert_eq!(w.rig.e.mem.u32(w.t), VTABLE);
        assert_eq!(calls_to(&log, 0x009c_a6c0), vec![vec![w.t]]);
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![w.t]]);
        let (_, log) = w.rig.run(0x009d_60f0, &args![w.t, 0u32]);
        assert!(calls_to(&log, OPERATOR_DELETE).is_empty());
    }

    #[test]
    fn update_moves_to_cover_or_hides() {
        // A valid cover location far away: a path to the hide position.
        let mut w = world();
        let t = w.t;
        w.rig.answer(IS_COVER_LOCATION_VALID, 1);
        w.rig.answer_float(GET_TARGET_FIRING_Z_OFFSET, 1.5);
        w.rig.e.mem.set_f32(t + 0x34, 2.0);
        w.rig.answer_float(POINT_LENGTH_SQUARED, 700.0);
        w.rig.answer(GET_WORLD_SPACE, 0x3200);
        w.rig.answer(ACTOR_CELL_8D6F30, 0x3300);
        w.rig.answer(SET_PATHFINDING_GOAL, 1);
        let (_, log) = w.rig.run(0x009d_6140, &args![t]);
        assert_eq!(
            calls_to(&log, IS_COVER_LOCATION_VALID),
            vec![vec![0x5100, w.cover]]
        );
        assert_eq!(w.rig.e.mem.u8(w.cover + 0x74), 1);
        assert_eq!(
            calls_to(&log, CALCULATE_COVER_POSITIONS),
            vec![vec![w.controller, w.cover, t + 0x20, t + 0x2c]]
        );
        assert_eq!(
            calls_to(&log, GET_TARGET_FIRING_Z_OFFSET),
            vec![vec![w.target, 2, 0]]
        );
        assert_eq!(w.rig.e.mem.f32(t + 0x34), 3.5);
        let request = calls_to(&log, PATHING_REQUEST_CONSTRUCT)[0][0];
        assert_eq!(
            calls_to(&log, BUILD_REQUEST),
            vec![vec![w.actor, request, t + 0x20, 0x3300, 0x3200, 0, 0]]
        );
        assert_eq!(
            w.rig.e.get(w.this, CombatProcedureHideFromTarget::eState),
            1
        );
        assert_eq!(calls_to(&log, SET_MOVE_MODE), vec![vec![t, 3]]);

        // No valid cover: hides where it is, looking at the target.
        let mut w = world();
        let t = w.t;
        w.rig.answer(GET_TARGET_FIRING_LOCATION, w.position);
        let (_, log) = w.rig.run(0x009d_6140, &args![t]);
        assert_eq!(w.rig.e.mem.f32(t + 0x20), 5.0);
        assert_eq!(w.rig.e.mem.f32(t + 0x2c), 5.0);
        assert_eq!(calls_to(&log, TIMER_START), vec![vec![t + 0x18, bits(6.0)]]);
        assert_eq!(
            calls_to(&log, SET_LOOK_AT_TARGET),
            vec![vec![t, t + 0x2c, 3]]
        );
        assert_eq!(
            w.rig.e.get(w.this, CombatProcedureHideFromTarget::eState),
            2
        );
        assert!(calls_to(&log, POINT_LENGTH_SQUARED).is_empty());
    }

    #[test]
    fn update_follows_the_path_and_the_hiding() {
        let mut w = world();
        w.rig
            .e
            .set(w.this, CombatProcedureHideFromTarget::eState, 1);
        w.rig.answer(ACTOR_PATH_TEST, 1);
        let (_, log) = w.rig.run(0x009d_6140, &args![w.t]);
        assert_eq!(
            calls_to(&log, FAIL_PROCEDURE),
            vec![vec![w.t, TEXT_PATH_BUILDING_FAILED]]
        );
        let mut w = world();
        w.rig
            .e
            .set(w.this, CombatProcedureHideFromTarget::eState, 1);
        w.rig.answer(IS_PATHING_COMPLETE, 1);
        w.rig.run(0x009d_6140, &args![w.t]);
        assert_eq!(
            w.rig.e.get(w.this, CombatProcedureHideFromTarget::eState),
            2
        );

        // Hiding: the timer ran out, clearing the cover when asked.
        let mut w = world();
        w.rig
            .e
            .set(w.this, CombatProcedureHideFromTarget::eState, 2);
        w.rig.e.set(
            w.this,
            CombatProcedureHideFromTarget::bClearCoverWhenFinished,
            true,
        );
        w.rig.answer(TIMER_EXPIRED, 1);
        let (_, log) = w.rig.run(0x009d_6140, &args![w.t]);
        assert_eq!(calls_to(&log, CLEAR_COVER_LOCATION), vec![vec![0x5100, 0]]);
        assert_eq!(calls_to(&log, COMPLETE_PROCEDURE), vec![vec![w.t]]);

        // A second in, seen by the target without ignoring LOS: gives up.
        let mut w = world();
        w.rig
            .e
            .set(w.this, CombatProcedureHideFromTarget::eState, 2);
        w.rig.answer_float(TIMESTAMP_TIME, 10.0);
        w.rig.answer_float(TIME_NOW, 12.0);
        w.rig.answer(STATE_BYTE_71, 1);
        let (_, log) = w.rig.run(0x009d_6140, &args![w.t]);
        assert_eq!(calls_to(&log, CLEAR_COVER_LOCATION), vec![vec![0x5100, 1]]);
        assert_eq!(calls_to(&log, STATE_SET_BYTE_D8), vec![vec![0x5100, 0]]);
        assert_eq!(calls_to(&log, COMPLETE_PROCEDURE), vec![vec![w.t]]);

        // Ignoring LOS with the target close: gives up too.
        let mut w = world();
        w.rig
            .e
            .set(w.this, CombatProcedureHideFromTarget::eState, 2);
        w.rig
            .e
            .set(w.this, CombatProcedureHideFromTarget::bIgnoreLOS, true);
        w.rig.answer_float(TIMESTAMP_TIME, 10.0);
        w.rig.answer_float(TIME_NOW, 12.0);
        w.rig.answer(STATE_BYTE_71, 1);
        w.rig
            .setting(SETTING_COMBAT_HIDE_FAILED_TARGET_DISTANCE, 10.0);
        w.rig.answer_float(WORLD_LOCATION_DISTANCE_SQUARED, 50.0);
        let (_, log) = w.rig.run(0x009d_6140, &args![w.t]);
        let distance = calls_to(&log, WORLD_LOCATION_DISTANCE_SQUARED);
        let locations = calls_to(&log, GET_WORLD_LOCATION);
        assert_eq!(locations[0][0], w.actor);
        assert_eq!(locations[1][0], w.target);
        assert_eq!(distance, vec![vec![locations[0][1], locations[1][1]]]);
        assert_eq!(calls_to(&log, COMPLETE_PROCEDURE), vec![vec![w.t]]);

        // Within 64 of the cover record: keeps hiding.
        let mut w = world();
        w.rig
            .e
            .set(w.this, CombatProcedureHideFromTarget::eState, 2);
        w.rig.answer_float(TIMESTAMP_TIME, 10.0);
        w.rig.answer_float(TIME_NOW, 12.0);
        let record = w.rig.e.mem.alloc(0x20);
        w.rig.e.mem.set_u8(record, 1);
        w.rig.answer(STATE_COVER_RECORD, record);
        w.rig.answer_float(WORLD_LOCATION_DISTANCE_SQUARED, 100.0);
        let (_, log) = w.rig.run(0x009d_6140, &args![w.t]);
        assert_eq!(
            calls_to(&log, WORLD_LOCATION_DISTANCE_SQUARED)[0][0],
            record + 8
        );
        assert!(calls_to(&log, STATE_BYTE_71).is_empty());
        assert!(calls_to(&log, COMPLETE_PROCEDURE).is_empty());
    }

    #[test]
    fn fn_009d6730_copies_the_time_stamp() {
        let mut w = world();
        let state = w.rig.e.mem.alloc(0x100);
        let out = w.rig.e.mem.alloc(4);
        w.rig.e.mem.set_u32(state + 0xa0, 0x4321);
        assert_eq!(w.rig.run(0x009d_6730, &args![state, out]).0.u32(), out);
        assert_eq!(w.rig.e.mem.u32(out), 0x4321);
    }

    #[test]
    fn display_debug_text_prints_the_state() {
        let mut w = world();
        let text = w.rig.object(8);
        let (_, log) = w.rig.run(0x009d_6750, &args![w.t, text, 5u32, 6u32]);
        assert_eq!(
            slot_calls(&log, SLOT_PRINT),
            vec![vec![text, 5, 6, TEXT_INITIALIZING]]
        );
        w.rig
            .e
            .set(w.this, CombatProcedureHideFromTarget::eState, 2);
        w.rig.answer_float(TIMER_DURATION, 6.0);
        w.rig.answer_float(TIMER_ELAPSED, 1.5);
        let (_, log) = w.rig.run(0x009d_6750, &args![w.t, text, 5u32, 6u32]);
        let mut timer = vec![text, 5, 6, TEXT_HIDING_TIMER];
        timer.extend(args![1.5f64, 6.0f64]);
        assert_eq!(
            slot_calls(&log, SLOT_PRINT),
            vec![vec![text, 5, 6, TEXT_HIDING], timer]
        );
        w.rig
            .e
            .set(w.this, CombatProcedureHideFromTarget::eState, 1);
        let (_, log) = w.rig.run(0x009d_6750, &args![w.t, text, 5u32, 6u32]);
        assert_eq!(
            slot_calls(&log, SLOT_PRINT),
            vec![vec![text, 5, 6, TEXT_MOVING_TO_COVER]]
        );
    }

    #[test]
    fn save_and_load_game_keep_the_cover_location() {
        let mut w = world();
        let t = w.t;
        w.rig.answer(STATE_COVER_LOCATION, w.cover);
        let saved = Rc::new(RefCell::new(Vec::new()));
        let record = Rc::clone(&saved);
        w.rig.e.register_double(SAVE_BYTES, move |e, a| {
            record.borrow_mut().push((a[1], e.mem.u8(a[1])));
            Ret::default()
        });
        let (_, log) = w.rig.run(0x009d_6820, &args![t, 0x40u32]);
        let sizes: Vec<_> = calls_to(&log, SAVE_BYTES).iter().map(|c| c[2]).collect();
        assert_eq!(sizes, vec![4, 4, 0xc, 0xc, 1, 1, 1]);
        assert_eq!(saved.borrow().last().unwrap().1, 2);
        assert_eq!(calls_to(&log, TIMER_SAVE), vec![vec![t + 0x18, 0x40]]);

        // Loading the kind 1 takes the last cover location.
        w.rig.answer(STATE_LAST_COVER_LOCATION, 0x7800);
        w.rig.e.register(LOAD_BYTES, |e, a| {
            if a[2] == 1 {
                e.mem.set_u8(a[1], 1);
            }
            Ret::default()
        });
        let (_, log) = w.rig.run(0x009d_6900, &args![t, 0x40u32]);
        assert_eq!(calls_to(&log, LOAD_BYTES).len(), 7);
        assert_eq!(w.rig.e.mem.u32(t + 0x38), 0x7800);
        assert_eq!(calls_to(&log, TIMER_LOAD), vec![vec![t + 0x18, 0x40]]);
    }
}
