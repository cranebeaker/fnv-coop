//! `fallout/ai/combat/combatprocedureengagetarget.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 009d3b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `01091514` (RTTI `.?AVCombatProcedureEngageTarget@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `009ca6c0`. One word, plain `RET`, returns nothing.
pub fn fn_009d3b30(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0109_1514);
    e.call(0x009c_a6c0, &args![this]);
}

layout! {
    /// `CombatProcedureEngageTarget` (Xbox PDB), 0x64 bytes; the offsets
    /// are the ones the PC code uses.
    pub struct CombatProcedureEngageTarget: 0x64 {
        /// `pCombatController` (Xbox PDB, `CombatProcedure`): `CombatController*`.
        0x04 pCombatController: Ptr,
        /// `eState` (Xbox PDB): 0 initializing, 1 holding, 2 crouching, 3
        /// moving to the optimal location (close), 4 (far), 5 approaching
        /// the target, 6 dodging forward/backward, 7 dodging left/right, 8
        /// moving to the target (the debug texts).
        0x10 eState: u32,
        /// `fMinOptimalRange` (Xbox PDB).
        0x14 fMinOptimalRange: f32,
        /// `fMaxOptimalRange` (Xbox PDB).
        0x18 fMaxOptimalRange: f32,
        /// `fAbsoluteMaxRange` (Xbox PDB).
        0x1C fAbsoluteMaxRange: f32,
        /// `bLeftRightDodgeHadLOS` (Xbox PDB).
        0x30 bLeftRightDodgeHadLOS: bool,
        /// `bLeftRightDodgeCheckLOS` (Xbox PDB).
        0x31 bLeftRightDodgeCheckLOS: bool,
        /// `bLeftRightDodgeLostLOS` (Xbox PDB).
        0x32 bLeftRightDodgeLostLOS: bool,
        /// `cCheckedOptimalPath` (Xbox PDB): 1 checked, 0 not, -1 movement
        /// check failed, -2 line of sight check failed.
        0x33 cCheckedOptimalPath: i8,
    }
}

/// The embedded members (Xbox PDB names): the `CombatTimer`s, the dodge
/// start (`NiPoint3`), the two `BGSWorldLocation`s and the time stamp.
const HOLDING_TIMER: u32 = 0x20;
const DODGE_LOS_TIMER: u32 = 0x28;
const DODGE_START_LOCATION: u32 = 0x34;
const DESTINATION: u32 = 0x40;
const TARGET_OPTIMAL_WORLD_LOCATION: u32 = 0x50;
const PATHING_FAILED_TIMESTAMP: u32 = 0x60;
/// `CombatProcedureEngageTarget`'s vtable (`01091514`).
const VTABLE: u32 = 0x0109_1514;

/// `CombatProcedure` methods (Xbox PDB names), its constructor `009ca680`,
/// `009caac0` (fail with a reason), `operator delete` (`00401030`).
const BASE_CONSTRUCT: u32 = 0x009c_a680;
const GET_ATTACKER: u32 = 0x0099_6de0;
const GET_TARGET: u32 = 0x0099_6e00;
const SET_MOVE_MODE: u32 = 0x0099_6e20;
const CLEAR_MOVE_MODE: u32 = 0x0099_6e40;
const SET_LOOK_AT_TARGET: u32 = 0x0099_6e60;
const CLEAR_LOOK_AT_TARGET: u32 = 0x0099_6ea0;
const BASE_SAVE_GAME: u32 = 0x0099_6ed0;
const BASE_LOAD_GAME: u32 = 0x0099_6ef0;
const FAIL_PROCEDURE: u32 = 0x009c_aac0;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// Controller getters: `00586150` the combat state, `007031c0` the weapon
/// record, `004030b0` the target, `0097ae90` the actor, `00998a30` the byte
/// `+0xd4`, `009a0240` the byte `+0xd5` (the engine map's name
/// `BSFaceGenNiNode::GetFixedNormals` belongs to a folded method).
const GET_COMBAT_STATE: u32 = 0x0058_6150;
const CONTROLLER_WEAPON: u32 = 0x0070_31c0;
const CONTROLLER_TARGET: u32 = 0x0040_30b0;
const CONTROLLER_ACTOR: u32 = 0x0097_ae90;
const CONTROLLER_BYTE_D4: u32 = 0x0099_8a30;
const CONTROLLER_BYTE_D5: u32 = 0x009a_0240;
/// Combat state methods (Xbox PDB names where the map has them):
/// `GetLastSeenTargetFiringLocation`, `CheckMovement` (three words),
/// `FindBestDodgeLocation` (five words), `AddUnreachableLocation`;
/// `0097fb00` (copies the time stamp `+0x98` out), `0097fb20` (the byte
/// `+0x71`), `009a0260` (four words, `AL`), `0099e840` (six words, `AL`).
const GET_LAST_SEEN_TARGET_FIRING_LOCATION: u32 = 0x0099_abc0;
const CHECK_MOVEMENT: u32 = 0x009a_02a0;
const FIND_BEST_DODGE_LOCATION: u32 = 0x0099_e190;
const ADD_UNREACHABLE_LOCATION: u32 = 0x009a_0ec0;
const STATE_TIMESTAMP_COPY: u32 = 0x0097_fb00;
const STATE_BYTE_71: u32 = 0x0097_fb20;
const STATE_CHECK_LOCATION: u32 = 0x009a_0260;
const STATE_FIND_OPTIMAL_LOCATION: u32 = 0x0099_e840;
/// `CombatUtilities` (Xbox PDB, `__cdecl`).
const COMPUTE_OPTIMAL_WEAPON_RANGES: u32 = 0x009a_9180;
const CALCULATE_LOS: u32 = 0x009a_7940;
/// `Pathing::FindClosestPointOnNavmesh` (Xbox PDB, `__cdecl`).
const FIND_CLOSEST_POINT_ON_NAVMESH: u32 = 0x006d_6f80;
/// Actor methods (Xbox PDB names) and unnamed helpers: `008b3b90` (a path
/// test), `008d6f30` (its cell), `004997b0` (`AL`: move flag 0x400 without
/// 0x800).
const STOP_MOVING_ACTOR: u32 = 0x008b_3ab0;
const FORCE_STOP_MOVING: u32 = 0x008b_3ad0;
const IS_PATH_VALID: u32 = 0x008b_3960;
const IS_PATHING_COMPLETE: u32 = 0x008b_3bb0;
const IS_WAITING_ON_PATH: u32 = 0x008b_3bf0;
const GET_CURRENT_PATHFINDING_GOAL: u32 = 0x008b_3840;
const SET_PATHFINDING_GOAL: u32 = 0x008b_3630;
const BUILD_REQUEST: u32 = 0x008b_3880;
const BUILD_REQUEST_OV2: u32 = 0x008b_38d0;
const ACTOR_PATH_TEST: u32 = 0x008b_3b90;
const ACTOR_CELL_8D6F30: u32 = 0x008d_6f30;
const ACTOR_TEST_4997B0: u32 = 0x0049_97b0;
const GET_WORLD_SPACE: u32 = 0x0057_5d70;
const GET_WORLD_LOCATION: u32 = 0x0057_5650;
const GET_SAVED_ACQUIRE_OBJECT: u32 = 0x008d_8520;
/// Actor vtable slot `+0x1f4` (the position) and the acquire object's slot
/// `+0x408` (one word, `AL`: whether a move animation exists).
const SLOT_POSITION: u32 = 0x1f4;
const SLOT_HAS_MOVE_ANIM: u32 = 0x408;
/// `BGSWorldLocation` methods (Xbox PDB names): `GetDistanceSquared`,
/// `GetDistance`, `GetCellOrWorld` (two words), `SaveGame` / `LoadGame`;
/// `0097ede0` (its constructor; the map's name `CheckSpace` belongs to a
/// folded method), `0097ee20` (a dodge location's constructor), `0097b940`
/// (its point), `0084e3a0` (its word `+0xc`), `006815c0` (its point,
/// returning `this`), `0098ddd0` (sets its point, one word), `0041fd00`
/// (sets its word, one word).
const WORLD_LOCATION_DISTANCE_SQUARED: u32 = 0x0052_7a40;
const WORLD_LOCATION_DISTANCE: u32 = 0x0052_7a80;
const WORLD_LOCATION_CELL_OR_WORLD: u32 = 0x0052_7990;
const WORLD_LOCATION_SAVE: u32 = 0x0052_7b70;
const WORLD_LOCATION_LOAD: u32 = 0x0052_7ba0;
const WORLD_LOCATION_CONSTRUCT: u32 = 0x0097_ede0;
const DODGE_LOCATION_CONSTRUCT: u32 = 0x0097_ee20;
const DODGE_LOCATION_POINT: u32 = 0x0097_b940;
const WORLD_LOCATION_WORD_C: u32 = 0x0084_e3a0;
const POINT_CONSTRUCT: u32 = 0x0068_15c0;
const WORLD_LOCATION_SET_POINT: u32 = 0x0098_ddd0;
const WORLD_LOCATION_SET_WORD: u32 = 0x0041_fd00;
/// Pathing requests: `PathingRequest::PathingRequest` (Xbox PDB) and its
/// destructor `006e2620`; `PathingRequestOptimalLocation::PathingRequestOptimalLocation`
/// (Xbox PDB) and its destructor `006dad70`; the request setters
/// (`__thiscall`, one word): `00507610`, `006e5ee0`, `006e4ef0`, `006e4320`
/// (floats); `0048d150` (its `+0xc`), `005c3420` (a point into the out
/// point), `006e2980` (goal point and radius, two words).
const PATHING_REQUEST_CONSTRUCT: u32 = 0x006e_2420;
const PATHING_REQUEST_DESTRUCT: u32 = 0x006e_2620;
const OPTIMAL_REQUEST_CONSTRUCT: u32 = 0x006e_5d10;
const OPTIMAL_REQUEST_DESTRUCT: u32 = 0x006d_ad70;
const REQUEST_SET_MIN_RANGE: u32 = 0x0050_7610;
const REQUEST_SET_MAX_RANGE: u32 = 0x006e_5ee0;
const REQUEST_SET_HALF_MIN_RANGE: u32 = 0x006e_4ef0;
const REQUEST_SET_ABSOLUTE_RANGE: u32 = 0x006e_4320;
const REQUEST_LOCATION: u32 = 0x0048_d150;
const LOCATION_POINT: u32 = 0x005c_3420;
const REQUEST_SET_GOAL: u32 = 0x006e_2980;
/// Timers: `00906f60` (constructor), `0097fa50` (running), `008d7f40`
/// (start), `008d7f80` (ran out), `00910b10` (clear), `00435e00`
/// (elapsed), `006b9130` (duration), `00435dd0` (now), `00435de0` (sets a
/// time stamp), `0097fa90` (`ST0`), and the `SaveGame` / `LoadGame` of
/// `CombatTimer` and `CombatTimeStamp` (Xbox PDB).
const TIMER_CONSTRUCT: u32 = 0x0090_6f60;
const TIMER_RUNNING: u32 = 0x0097_fa50;
const TIMER_START: u32 = 0x008d_7f40;
const TIMER_EXPIRED: u32 = 0x008d_7f80;
const TIMER_CLEAR: u32 = 0x0091_0b10;
const TIMER_ELAPSED: u32 = 0x0043_5e00;
const TIMER_DURATION: u32 = 0x006b_9130;
const TIME_NOW: u32 = 0x0043_5dd0;
const TIMESTAMP_SET: u32 = 0x0043_5de0;
const TIMESTAMP_VALUE_FA90: u32 = 0x0097_fa90;
const TIMER_SAVE: u32 = 0x009a_5eb0;
const TIMER_LOAD: u32 = 0x009a_5f20;
const TIMESTAMP_SAVE: u32 = 0x009a_5f90;
const TIMESTAMP_LOAD: u32 = 0x009a_5ff0;
/// `RandomFloat` (engine map name of `00476b70`, two floats) and
/// `004dff00` (`__cdecl`, one float, `AL`: a random chance).
const RANDOM_FLOAT: u32 = 0x0047_6b70;
const RANDOM_CHANCE: u32 = 0x004d_ff00;
/// Point helpers: `00439ef0` (difference into the out point), `004a7290`
/// (squared length).
const POINT_DIFFERENCE: u32 = 0x0043_9ef0;
const POINT_LENGTH_SQUARED: u32 = 0x004a_7290;
/// The byte global at `011a5b70`: whether the optimal location goes
/// through a `PathingRequestOptimalLocation`.
const USE_OPTIMAL_LOCATION_REQUEST: u32 = 0x011a_5b70;
/// The default (zero) `NiPoint3` at `011f426c`.
const DEFAULT_POINT: u32 = 0x011f_426c;
/// `float` constants of the exe: 0.01, 0.1, 0.25, 0.5, -1, 2, 5, 16, 24,
/// 64, 128, 192, 256, 384, 10000, the largest `float` (`010914d0`).
const HUNDREDTH_FLOAT: u32 = 0x0101_3ea4;
const TENTH_FLOAT: u32 = 0x0101_e2bc;
const QUARTER_FLOAT: u32 = 0x0101_622c;
const HALF_FLOAT: u32 = 0x0101_6248;
const MINUS_ONE_FLOAT: u32 = 0x0101_2054;
const TWO_FLOAT: u32 = 0x0101_62c0;
const FIVE_FLOAT: u32 = 0x0101_712c;
const SIXTEEN_FLOAT: u32 = 0x0101_e57c;
const TWENTY_FOUR_FLOAT: u32 = 0x0104_e0e8;
const SIXTY_FOUR_FLOAT: u32 = 0x0101_e6ec;
const HUNDRED_TWENTY_EIGHT_FLOAT: u32 = 0x0101_e704;
const HUNDRED_NINETY_TWO_FLOAT: u32 = 0x0105_36b8;
const TWO_HUNDRED_FIFTY_SIX_FLOAT: u32 = 0x0103_2adc;
const THREE_HUNDRED_EIGHTY_FOUR_FLOAT: u32 = 0x0105_0b98;
const TEN_THOUSAND_FLOAT: u32 = 0x0102_2958;
const LARGEST_FLOAT: u32 = 0x0109_14d0;
/// `double` constants of the exe: 0.5, 1, 5, 256 (`010231d8`), 65536
/// (`0108e058`), 262144 (`01091138`).
const HALF_DOUBLE: u32 = 0x0101_1588;
const ONE_DOUBLE: u32 = 0x0101_2070;
const FIVE_DOUBLE: u32 = 0x0102_0998;
const TWO_HUNDRED_FIFTY_SIX_DOUBLE: u32 = 0x0102_31d8;
const NEAR_DISTANCE_SQUARED_DOUBLE: u32 = 0x0108_e058;
const APPROACH_DISTANCE_SQUARED_DOUBLE: u32 = 0x0109_1138;
/// `BGSSaveGameBuffer` / `BGSLoadGameBuffer` byte writers and readers.
const SAVE_BYTES: u32 = 0x0086_5e50;
const LOAD_BYTES: u32 = 0x0086_4980;
/// The debug-text object's print slot (`__cdecl` through the vtable).
const SLOT_PRINT: u32 = 0x14;
/// Strings.
const TEXT_INITIALIZING: u32 = 0x0109_0b68;
const TEXT_PATH_BUILDING_FAILED_CLOSE: u32 = 0x0109_1558;
const TEXT_PATHING_FAILED_CLOSE: u32 = 0x0109_1590;
const TEXT_PATH_BUILDING_FAILED_FAR: u32 = 0x0109_15c8;
const TEXT_PATHING_FAILED_FAR: u32 = 0x0109_1600;
const TEXT_PATH_BUILDING_FAILED_APPROACH: u32 = 0x0109_1634;
const TEXT_PATHING_FAILED_APPROACH: u32 = 0x0109_165c;
const TEXT_PATH_BUILDING_FAILED_TARGET: u32 = 0x0109_1684;
const TEXT_PATHING_FAILED_TARGET: u32 = 0x0109_16ac;
const TEXT_PATH_BUILDING_FAILED_DODGE: u32 = 0x0109_16d4;
const TEXT_PATHING_FAILED_DODGE: u32 = 0x0109_16f4;
const TEXT_CANNOT_SETUP_TARGET_PATH: u32 = 0x0109_1714;
const TEXT_CANNOT_SETUP_DODGE_PATH: u32 = 0x0109_173c;
const TEXT_OPTIMAL_BUILD_FAILED: u32 = 0x0109_1760;
const TEXT_OPTIMAL_LOS_FAILED: u32 = 0x0109_1794;
const TEXT_OPTIMAL_MOVEMENT_FAILED: u32 = 0x0109_17cc;
const TEXT_LOST_LOS_TIMER: u32 = 0x0109_1808;
const TEXT_DODGING_LEFT_RIGHT: u32 = 0x0109_1834;
const TEXT_DODGING_FORWARD_BACKWARD: u32 = 0x0109_1848;
const TEXT_MOVING_TO_TARGET: u32 = 0x0109_1864;
const TEXT_APPROACHING_TARGET: u32 = 0x0109_1878;
const TEXT_MOVING_OPTIMAL_FAR: u32 = 0x0109_188c;
const TEXT_TARGET_MOVEMENT: u32 = 0x0109_18a0;
const TEXT_RANGE: u32 = 0x0109_18c4;
const TEXT_MOVING_OPTIMAL_CLOSE: u32 = 0x0109_18d8;
const TEXT_CROUCHING_TIMER: u32 = 0x0109_18f0;
const TEXT_CROUCHING: u32 = 0x0109_1910;
const TEXT_HOLDING_TIMER: u32 = 0x0109_191c;
const TEXT_HOLDING: u32 = 0x0109_1938;

fn controller_of(e: &Engine, this: Ptr<CombatProcedureEngageTarget>) -> u32 {
    e.get(this, CombatProcedureEngageTarget::pCombatController)
        .addr()
}

fn copy_words(e: &mut Engine, from: u32, to: u32, words: u32) {
    for i in 0..words {
        let word = e.mem.u32(from + i * 4);
        e.mem.set_u32(to + i * 4, word);
    }
}

fn quiet_global(e: &Engine, addr: u32) -> u32 {
    x87_quiet_bits(e.global::<u32>(addr))
}

/// `RandomFloat` between two `float` constants of the exe, stored as a
/// `float`.
fn random_between(e: &mut Engine, low: u32, high: u32) -> u32 {
    let high = quiet_global(e, high);
    let low = quiet_global(e, low);
    x87_store32(e.call(RANDOM_FLOAT, &args![low, high]).f64())
}

/// A pathing request to `goal` (`Actor::BuildRequest` with the attacker's
/// cell and worldspace, the radius `radius`, no avoid nodes) at `request`
/// in the frame.
fn build_request(e: &mut Engine, attacker: u32, request: u32, goal: u32, radius: u32) {
    e.call(PATHING_REQUEST_CONSTRUCT, &args![request]);
    let worldspace = e.call(GET_WORLD_SPACE, &args![attacker]).u32();
    let cell = e.call(ACTOR_CELL_8D6F30, &args![attacker]).u32();
    e.call(
        BUILD_REQUEST,
        &args![attacker, request, goal, cell, worldspace, radius, 0u32],
    );
}

/// A pathing request to a world location (`Actor::BuildRequest_ov2` with its
/// point and its word `+0xc`) at `request` in the frame.
fn build_request_to_location(
    e: &mut Engine,
    attacker: u32,
    request: u32,
    location: u32,
    radius: u32,
) {
    e.call(PATHING_REQUEST_CONSTRUCT, &args![request]);
    let space = e.call(WORLD_LOCATION_WORD_C, &args![location]).u32();
    let point = e.call(POINT_CONSTRUCT, &args![location]).u32();
    e.call(
        BUILD_REQUEST_OV2,
        &args![attacker, request, point, space, radius, 0u32],
    );
}

/// The end of a hold: crouch (state 2, move mode 2) when `009d4f50` and a
/// half chance say so, otherwise hold (state 1).
fn hold_or_crouch(e: &mut Engine, this: Ptr<CombatProcedureEngageTarget>) {
    let crouch = fn_009d4f50(e, this) && {
        let half = quiet_global(e, HALF_FLOAT);
        e.call(RANDOM_CHANCE, &args![half]).bool()
    };
    if crouch {
        e.call(SET_MOVE_MODE, &args![this, 2u32]);
        e.set(this, CombatProcedureEngageTarget::eState, 2);
    } else {
        e.set(this, CombatProcedureEngageTarget::eState, 1);
    }
}

// Translated from 009d3a00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009d3a00 - the compiler's FS:[0] exception frame (unwinding of the base class if a member constructor throws) is not translated
/// `CombatProcedureEngageTarget::CombatProcedureEngageTarget` (Xbox PDB):
/// the base class, the vtable, state 0, zero ranges, the two timers, the
/// dodge flags (`cCheckedOptimalPath` 1), the dodge start (the default
/// point), the two world locations and the failed-pathing time stamp at
/// `-FLT_MAX`. Returns `this`.
pub fn construct(
    e: &mut Engine,
    this: Ptr<CombatProcedureEngageTarget>,
) -> Ptr<CombatProcedureEngageTarget> {
    let t = this.addr();
    e.call(BASE_CONSTRUCT, &args![t]);
    e.mem.set_u32(t, VTABLE);
    e.set(this, CombatProcedureEngageTarget::eState, 0);
    for offset in [0x14u32, 0x18, 0x1c] {
        e.mem.set_u32(t + offset, 0.0f32.to_bits());
    }
    e.call(TIMER_CONSTRUCT, &args![t + HOLDING_TIMER]);
    e.call(TIMER_CONSTRUCT, &args![t + DODGE_LOS_TIMER]);
    e.set(
        this,
        CombatProcedureEngageTarget::bLeftRightDodgeHadLOS,
        false,
    );
    e.set(
        this,
        CombatProcedureEngageTarget::bLeftRightDodgeCheckLOS,
        false,
    );
    e.set(
        this,
        CombatProcedureEngageTarget::bLeftRightDodgeLostLOS,
        false,
    );
    e.set(this, CombatProcedureEngageTarget::cCheckedOptimalPath, 1);
    copy_words(e, DEFAULT_POINT, t + DODGE_START_LOCATION, 3);
    e.call(WORLD_LOCATION_CONSTRUCT, &args![t + DESTINATION]);
    e.call(
        WORLD_LOCATION_CONSTRUCT,
        &args![t + TARGET_OPTIMAL_WORLD_LOCATION],
    );
    let never = x87_store32(-x87_load32(e.global::<u32>(LARGEST_FLOAT)));
    e.call(TIMESTAMP_SET, &args![t + PATHING_FAILED_TIMESTAMP, never]);
    this
}

// Translated from 009d3b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor (the vtable's slot 0): the destructor
/// body `009d3b30`, then `operator delete` when bit 0 of `flags` is set.
/// Returns `this`.
pub fn scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<CombatProcedureEngageTarget>,
    flags: u32,
) -> Ptr<CombatProcedureEngageTarget> {
    fn_009d3b30(e, this.cast());
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 009d3b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureEngageTarget::Shutdown` (Xbox PDB): clears the move
/// mode, stops the attacker and clears the look-at target.
pub fn shutdown(e: &mut Engine, this: Ptr<CombatProcedureEngageTarget>) {
    e.call(CLEAR_MOVE_MODE, &args![this]);
    let attacker = e.call(GET_ATTACKER, &args![this]).u32();
    e.call(STOP_MOVING_ACTOR, &args![attacker]);
    e.call(CLEAR_LOOK_AT_TARGET, &args![this]);
}

// Translated from 009d3b80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009d3b80 - the compiler's FS:[0] exception frame (unwinding of the pathing requests) is not translated
/// `CombatProcedureEngageTarget::Update` (Xbox PDB): with a target, looks
/// at its last seen firing location, goes back to holding when the
/// controller's byte `+0xd5` is clear, and refreshes the optimal weapon
/// ranges (noting a change). With the squared distance to the target it
/// runs the state (switch table `009d4ee8`): 0 waits 0.1 to 0.5 seconds;
/// holding and crouching approach a close target (within 512, after half
/// a second of the combat state's time stamp), handle the left/right
/// dodge's line of sight, move to the optimal location when outside the
/// optimal range (or dodge, or move to the target beyond the absolute
/// range), and when the holding timer runs out may dodge sideways or
/// crouch; the dodges (6, 7) follow their paths and stop when the line of
/// sight is lost; the moves (3, 4, 5, 8) follow their paths, stop once
/// close enough or seen, re-check the optimal path and re-plan when the
/// target moved more than 256 or the ranges changed.
///
/// Locals follow the disassembly's frame (offsets below `EBP`).
pub fn update(e: &mut Engine, this: Ptr<CombatProcedureEngageTarget>) {
    const FRAME: u32 = 0x570;
    let t = this.addr();
    e.with_stack(FRAME, |e, base| {
        let ebp = base.addr() + FRAME;
        let attacker = e.call(GET_ATTACKER, &args![t]).u32();
        let target = e.call(GET_TARGET, &args![t]).u32();
        let controller = controller_of(e, this);
        let combat_state = e.call(GET_COMBAT_STATE, &args![controller]).u32();
        if target == 0 {
            return;
        }
        let last_seen = ebp - 0x58;
        e.call(POINT_CONSTRUCT, &args![last_seen]);
        if e.call(
            GET_LAST_SEEN_TARGET_FIRING_LOCATION,
            &args![combat_state, last_seen],
        )
        .bool()
        {
            e.call(SET_LOOK_AT_TARGET, &args![t, last_seen, 1u32]);
        }
        let controller = controller_of(e, this);
        if !e.call(CONTROLLER_BYTE_D5, &args![controller]).bool() {
            e.set(this, CombatProcedureEngageTarget::eState, 1);
            return;
        }
        let old_min = x87_quiet_bits(e.mem.u32(t + 0x14));
        let old_max = x87_quiet_bits(e.mem.u32(t + 0x18));
        let controller = controller_of(e, this);
        let weapon = e.call(CONTROLLER_WEAPON, &args![controller]).u32();
        let controller = controller_of(e, this);
        e.call(
            COMPUTE_OPTIMAL_WEAPON_RANGES,
            &args![controller, weapon, t + 0x14, t + 0x18, t + 0x1c],
        );
        // FUCOMPP; TEST AH,0x44: a change (or a NaN) in either range.
        let ranges_changed = x87_load32(old_min) != x87_load32(e.mem.u32(t + 0x14))
            || x87_load32(old_max) != x87_load32(e.mem.u32(t + 0x18));
        let target_location = ebp - 0x2c;
        let attacker_location = ebp - 0x44;
        e.call(GET_WORLD_LOCATION, &args![target, target_location]);
        e.call(GET_WORLD_LOCATION, &args![attacker, attacker_location]);
        let squared = x87_store32(
            e.call(
                WORLD_LOCATION_DISTANCE_SQUARED,
                &args![attacker_location, target_location],
            )
            .f64(),
        );
        let squared_of = |e: &Engine, offset: u32| {
            let range = e.mem.u32(t + offset);
            x87_mul(x87_load32(range), x87_m32(range))
        };
        let beyond_max = x87_load32(squared) > squared_of(e, 0x18);
        let within_min = x87_load32(squared) < squared_of(e, 0x14);
        let beyond_absolute = x87_load32(squared) > squared_of(e, 0x1c);
        match e.get(this, CombatProcedureEngageTarget::eState) {
            0 => {
                let wait = random_between(e, TENTH_FLOAT, HALF_FLOAT);
                e.call(TIMER_START, &args![t + HOLDING_TIMER, wait]);
                e.set(this, CombatProcedureEngageTarget::eState, 1);
            }
            1 | 2 => {
                // Approaching a close target.
                let stamp = e
                    .call(STATE_TIMESTAMP_COPY, &args![combat_state, ebp - 0x520])
                    .u32();
                let since = e.call(TIMER_ELAPSED, &args![stamp]).f64();
                if since > e.global::<f64>(HALF_DOUBLE)
                    && x87_load32(squared) < e.global::<f64>(APPROACH_DISTANCE_SQUARED_DOUBLE)
                    && e.call(
                        CHECK_MOVEMENT,
                        &args![combat_state, target_location, 1u32, 0u32],
                    )
                    .bool()
                {
                    let request = ebp - 0x10c;
                    let minus_one = quiet_global(e, MINUS_ONE_FLOAT);
                    build_request_to_location(e, attacker, request, target_location, minus_one);
                    if e.call(SET_PATHFINDING_GOAL, &args![attacker, request])
                        .bool()
                    {
                        copy_words(e, target_location, t + DESTINATION, 4);
                        e.call(SET_MOVE_MODE, &args![t, 1u32]);
                        e.set(this, CombatProcedureEngageTarget::eState, 5);
                        e.call(PATHING_REQUEST_DESTRUCT, &args![request]);
                        return;
                    }
                    e.call(PATHING_REQUEST_DESTRUCT, &args![request]);
                }
                // The left/right dodge's line of sight.
                if e.get(this, CombatProcedureEngageTarget::bLeftRightDodgeCheckLOS) {
                    let elapsed = e.call(TIMER_ELAPSED, &args![t + HOLDING_TIMER]).f64();
                    if elapsed < e.global::<f64>(HALF_DOUBLE) {
                        if !e.call(STATE_BYTE_71, &args![combat_state]).bool() {
                            e.set(
                                this,
                                CombatProcedureEngageTarget::bLeftRightDodgeLostLOS,
                                true,
                            );
                            e.set(
                                this,
                                CombatProcedureEngageTarget::bLeftRightDodgeCheckLOS,
                                false,
                            );
                        }
                    } else {
                        e.set(
                            this,
                            CombatProcedureEngageTarget::bLeftRightDodgeCheckLOS,
                            false,
                        );
                    }
                }
                if e.get(this, CombatProcedureEngageTarget::bLeftRightDodgeLostLOS)
                    && (e.call(TIMER_EXPIRED, &args![t + HOLDING_TIMER]).bool()
                        || e.call(TIMER_ELAPSED, &args![t + HOLDING_TIMER]).f64()
                            > e.global::<f64>(ONE_DOUBLE))
                {
                    e.set(
                        this,
                        CombatProcedureEngageTarget::bLeftRightDodgeCheckLOS,
                        false,
                    );
                    e.set(
                        this,
                        CombatProcedureEngageTarget::bLeftRightDodgeLostLOS,
                        false,
                    );
                    // The byte as `AL` left it.
                    let seen = e.call(STATE_BYTE_71, &args![combat_state]).u8();
                    e.mem.set_u8(t + 0x30, seen);
                    e.call(TIMER_CLEAR, &args![t + DODGE_LOS_TIMER]);
                    let request = ebp - 0x1bc;
                    build_request(e, attacker, request, t + DODGE_START_LOCATION, 0);
                    if e.call(SET_PATHFINDING_GOAL, &args![attacker, request])
                        .bool()
                    {
                        e.call(SET_MOVE_MODE, &args![t, 1u32]);
                        e.set(this, CombatProcedureEngageTarget::eState, 7);
                        e.call(PATHING_REQUEST_DESTRUCT, &args![request]);
                        return;
                    }
                    e.call(PATHING_REQUEST_DESTRUCT, &args![request]);
                }
                // Outside the optimal range.
                if beyond_max || within_min {
                    let mut limit = quiet_global(e, TWO_FLOAT);
                    if beyond_max && beyond_absolute {
                        limit = quiet_global(e, TENTH_FLOAT);
                    }
                    // FLD limit; FCOMPP; TEST AH,5; JP: once the limit is
                    // less than the time held.
                    let due = e.call(TIMER_EXPIRED, &args![t + HOLDING_TIMER]).bool()
                        || x87_load32(limit)
                            < e.call(TIMER_ELAPSED, &args![t + HOLDING_TIMER]).f64();
                    if due {
                        if create_optimal_location_path(e, this, beyond_absolute) {
                            let state = if beyond_max { 3 } else { 4 };
                            e.set(this, CombatProcedureEngageTarget::eState, state);
                            return;
                        }
                        if !beyond_absolute {
                            let location = ebp - 0x1e4;
                            e.call(DODGE_LOCATION_CONSTRUCT, &args![location]);
                            let distance = random_between(
                                e,
                                HUNDRED_TWENTY_EIGHT_FLOAT,
                                TWO_HUNDRED_FIFTY_SIX_FLOAT,
                            );
                            let side = quiet_global(e, TWENTY_FOUR_FLOAT);
                            let kind = if beyond_max { 4u32 } else { 8 };
                            if e.call(
                                FIND_BEST_DODGE_LOCATION,
                                &args![combat_state, location, kind, 0u32, distance, side],
                            )
                            .bool()
                            {
                                let request = ebp - 0x2a4;
                                if !dodge_path(
                                    e,
                                    this,
                                    attacker,
                                    request,
                                    location,
                                    ebp - 0x1f0,
                                    distance,
                                ) {
                                    return;
                                }
                                e.call(SET_MOVE_MODE, &args![t, 1u32]);
                                e.set(this, CombatProcedureEngageTarget::eState, 6);
                                e.call(PATHING_REQUEST_DESTRUCT, &args![request]);
                                return;
                            }
                        } else if e
                            .call(
                                CHECK_MOVEMENT,
                                &args![combat_state, target_location, 1u32, 0u32],
                            )
                            .bool()
                        {
                            let request = ebp - 0x354;
                            let radius = quiet_global(e, HUNDRED_TWENTY_EIGHT_FLOAT);
                            build_request_to_location(
                                e,
                                attacker,
                                request,
                                target_location,
                                radius,
                            );
                            if !e
                                .call(SET_PATHFINDING_GOAL, &args![attacker, request])
                                .bool()
                            {
                                e.call(FAIL_PROCEDURE, &args![t, TEXT_CANNOT_SETUP_TARGET_PATH]);
                                e.call(PATHING_REQUEST_DESTRUCT, &args![request]);
                                return;
                            }
                            copy_words(e, target_location, t + DESTINATION, 4);
                            e.call(SET_MOVE_MODE, &args![t, 3u32]);
                            e.set(this, CombatProcedureEngageTarget::eState, 8);
                            e.call(PATHING_REQUEST_DESTRUCT, &args![request]);
                            return;
                        } else {
                            let cell = ebp - 0x368;
                            let world = ebp - 0x364;
                            let point = ebp - 0x360;
                            e.mem.set_u32(cell, 0);
                            e.mem.set_u32(world, 0);
                            e.call(
                                WORLD_LOCATION_CELL_OR_WORLD,
                                &args![target_location, cell, world],
                            );
                            e.call(POINT_CONSTRUCT, &args![point]);
                            let from = e.call(POINT_CONSTRUCT, &args![target_location]).u32();
                            let world_word = e.mem.u32(world);
                            let cell_word = e.mem.u32(cell);
                            let found = e
                                .call(
                                    FIND_CLOSEST_POINT_ON_NAVMESH,
                                    &args![world_word, cell_word, from, point],
                                )
                                .bool()
                                && {
                                    let space = e
                                        .call(WORLD_LOCATION_WORD_C, &args![target_location])
                                        .u32();
                                    e.call(
                                        STATE_CHECK_LOCATION,
                                        &args![combat_state, point, space, 1u32, 0u32],
                                    )
                                    .bool()
                                };
                            if found {
                                let request = ebp - 0x41c;
                                e.call(PATHING_REQUEST_CONSTRUCT, &args![request]);
                                let radius = quiet_global(e, HUNDRED_TWENTY_EIGHT_FLOAT);
                                let cell_word = e.mem.u32(cell);
                                let world_word = e.mem.u32(world);
                                e.call(
                                    BUILD_REQUEST,
                                    &args![
                                        attacker, request, point, cell_word, world_word, radius,
                                        0u32
                                    ],
                                );
                                if !e
                                    .call(SET_PATHFINDING_GOAL, &args![attacker, request])
                                    .bool()
                                {
                                    e.call(
                                        FAIL_PROCEDURE,
                                        &args![t, TEXT_CANNOT_SETUP_TARGET_PATH],
                                    );
                                    e.call(PATHING_REQUEST_DESTRUCT, &args![request]);
                                    return;
                                }
                                e.call(WORLD_LOCATION_SET_POINT, &args![t + DESTINATION, point]);
                                let space =
                                    e.call(WORLD_LOCATION_WORD_C, &args![target_location]).u32();
                                e.call(WORLD_LOCATION_SET_WORD, &args![t + DESTINATION, space]);
                                e.call(SET_MOVE_MODE, &args![t, 3u32]);
                                e.set(this, CombatProcedureEngageTarget::eState, 8);
                                e.call(PATHING_REQUEST_DESTRUCT, &args![request]);
                                return;
                            }
                        }
                    }
                }
                // The holding timer ran out.
                if !e.call(TIMER_EXPIRED, &args![t + HOLDING_TIMER]).bool() {
                    return;
                }
                let mut sideways = true;
                let object = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![attacker]).u32();
                if !e.vcall(object, SLOT_HAS_MOVE_ANIM, &args![5u32]).bool() || {
                    let object = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![attacker]).u32();
                    !e.vcall(object, SLOT_HAS_MOVE_ANIM, &args![6u32]).bool()
                } {
                    sideways = false;
                }
                if sideways && {
                    let quarter = quiet_global(e, QUARTER_FLOAT);
                    !e.call(RANDOM_CHANCE, &args![quarter]).bool()
                } {
                    // The byte as `AL` left it.
                    let seen = e.call(STATE_BYTE_71, &args![combat_state]).u8();
                    e.mem.set_u8(t + 0x30, seen);
                    let distance = if seen != 0 {
                        random_between(
                            e,
                            HUNDRED_TWENTY_EIGHT_FLOAT,
                            THREE_HUNDRED_EIGHTY_FOUR_FLOAT,
                        )
                    } else {
                        random_between(e, SIXTY_FOUR_FLOAT, HUNDRED_NINETY_TWO_FLOAT)
                    };
                    e.call(TIMER_CLEAR, &args![t + DODGE_LOS_TIMER]);
                    let location = ebp - 0x440;
                    e.call(DODGE_LOCATION_CONSTRUCT, &args![location]);
                    let side = quiet_global(e, TWENTY_FOUR_FLOAT);
                    if e.call(
                        FIND_BEST_DODGE_LOCATION,
                        &args![combat_state, location, 3u32, 0u32, distance, side],
                    )
                    .bool()
                    {
                        let request = ebp - 0x504;
                        if !dodge_path(e, this, attacker, request, location, ebp - 0x450, distance)
                        {
                            return;
                        }
                        e.set(
                            this,
                            CombatProcedureEngageTarget::bLeftRightDodgeCheckLOS,
                            true,
                        );
                        let position = e.vcall(attacker, SLOT_POSITION, &args![]).u32();
                        copy_words(e, position, t + DODGE_START_LOCATION, 3);
                        e.call(SET_MOVE_MODE, &args![t, 1u32]);
                        e.set(this, CombatProcedureEngageTarget::eState, 7);
                        e.call(PATHING_REQUEST_DESTRUCT, &args![request]);
                        return;
                    }
                }
                if fn_009d4f50(e, this) {
                    if e.call(ACTOR_TEST_4997B0, &args![attacker]).bool() {
                        let half = quiet_global(e, HALF_FLOAT);
                        if e.call(RANDOM_CHANCE, &args![half]).bool() {
                            e.set(this, CombatProcedureEngageTarget::eState, 2);
                        } else {
                            e.call(CLEAR_MOVE_MODE, &args![t]);
                            e.set(this, CombatProcedureEngageTarget::eState, 1);
                        }
                    } else {
                        e.call(SET_MOVE_MODE, &args![t, 2u32]);
                        e.set(this, CombatProcedureEngageTarget::eState, 2);
                    }
                } else {
                    e.set(this, CombatProcedureEngageTarget::eState, 1);
                }
                let wait = random_between(e, TWO_FLOAT, FIVE_FLOAT);
                e.call(TIMER_START, &args![t + HOLDING_TIMER, wait]);
            }
            6 | 7 => {
                e.set(
                    this,
                    CombatProcedureEngageTarget::bLeftRightDodgeLostLOS,
                    false,
                );
                if e.call(ACTOR_PATH_TEST, &args![attacker]).bool() {
                    if !e.call(IS_PATH_VALID, &args![attacker]).bool() {
                        fn_009d4f30(e, combat_state);
                    }
                    let reason = if e.call(IS_PATH_VALID, &args![attacker]).bool() {
                        TEXT_PATHING_FAILED_DODGE
                    } else {
                        TEXT_PATH_BUILDING_FAILED_DODGE
                    };
                    e.call(FAIL_PROCEDURE, &args![t, reason]);
                    return;
                }
                if e.get(this, CombatProcedureEngageTarget::eState) == 7 {
                    if !e.call(TIMER_RUNNING, &args![t + DODGE_LOS_TIMER]).bool() {
                        if e.get(this, CombatProcedureEngageTarget::bLeftRightDodgeHadLOS)
                            && !e.call(STATE_BYTE_71, &args![combat_state]).bool()
                        {
                            let half = quiet_global(e, HALF_FLOAT);
                            e.call(TIMER_START, &args![t + DODGE_LOS_TIMER, half]);
                        }
                    } else if e.call(TIMER_EXPIRED, &args![t + DODGE_LOS_TIMER]).bool() {
                        e.call(STOP_MOVING_ACTOR, &args![attacker]);
                        e.set(
                            this,
                            CombatProcedureEngageTarget::bLeftRightDodgeHadLOS,
                            false,
                        );
                        e.call(TIMER_CLEAR, &args![t + DODGE_LOS_TIMER]);
                    }
                }
                if e.call(IS_PATHING_COMPLETE, &args![attacker]).bool() {
                    hold_or_crouch(e, this);
                    let wait = random_between(e, TWO_FLOAT, FIVE_FLOAT);
                    e.call(TIMER_START, &args![t + HOLDING_TIMER, wait]);
                }
            }
            3 | 4 | 5 | 8 => {
                e.set(
                    this,
                    CombatProcedureEngageTarget::bLeftRightDodgeLostLOS,
                    false,
                );
                let mut stop = false;
                let mut force = false;
                if e.call(ACTOR_PATH_TEST, &args![attacker]).bool() {
                    if !e.call(IS_PATH_VALID, &args![attacker]).bool() {
                        let state = e.get(this, CombatProcedureEngageTarget::eState);
                        if state == 4 || state == 3 {
                            stamp_pathing_failed(e, this, ebp - 0x524);
                            stop = true;
                        } else {
                            e.call(
                                ADD_UNREACHABLE_LOCATION,
                                &args![combat_state, t + DESTINATION],
                            );
                            if e.get(this, CombatProcedureEngageTarget::eState) == 8 {
                                stop = true;
                            }
                        }
                    }
                    if !stop {
                        // The compiler's switch on `state - 3` (table
                        // `009d4f0c`).
                        let reasons = match e.get(this, CombatProcedureEngageTarget::eState) {
                            3 => Some((TEXT_PATHING_FAILED_FAR, TEXT_PATH_BUILDING_FAILED_FAR)),
                            4 => Some((TEXT_PATHING_FAILED_CLOSE, TEXT_PATH_BUILDING_FAILED_CLOSE)),
                            5 => Some((
                                TEXT_PATHING_FAILED_APPROACH,
                                TEXT_PATH_BUILDING_FAILED_APPROACH,
                            )),
                            8 => {
                                Some((TEXT_PATHING_FAILED_TARGET, TEXT_PATH_BUILDING_FAILED_TARGET))
                            }
                            _ => None,
                        };
                        if let Some((valid, invalid)) = reasons {
                            let reason = if e.call(IS_PATH_VALID, &args![attacker]).bool() {
                                valid
                            } else {
                                invalid
                            };
                            e.call(FAIL_PROCEDURE, &args![t, reason]);
                        }
                        return;
                    }
                }
                if !stop {
                    match e.get(this, CombatProcedureEngageTarget::eState) {
                        3 => {
                            if x87_load32(squared) < squared_of(e, 0x14) {
                                stop = true;
                            }
                        }
                        5 => {
                            if e.call(STATE_BYTE_71, &args![combat_state]).bool() {
                                stop = true;
                            }
                        }
                        8 => {
                            if e.call(STATE_BYTE_71, &args![combat_state]).bool()
                                && !beyond_absolute
                            {
                                stop = true;
                            }
                        }
                        _ => {}
                    }
                }
                let state = e.get(this, CombatProcedureEngageTarget::eState);
                if state == 3 || state == 4 {
                    if !stop
                        && e.get(this, CombatProcedureEngageTarget::cCheckedOptimalPath) == 0
                        && !e.call(IS_WAITING_ON_PATH, &args![attacker]).bool()
                    {
                        e.set(this, CombatProcedureEngageTarget::cCheckedOptimalPath, 1);
                        let goal = ebp - 0x514;
                        e.call(POINT_CONSTRUCT, &args![goal]);
                        if e.call(GET_CURRENT_PATHFINDING_GOAL, &args![attacker, goal])
                            .bool()
                        {
                            let space = e
                                .call(WORLD_LOCATION_WORD_C, &args![attacker_location])
                                .u32();
                            if !e
                                .call(
                                    STATE_CHECK_LOCATION,
                                    &args![combat_state, goal, space, 0u32, 0u32],
                                )
                                .bool()
                            {
                                e.set(this, CombatProcedureEngageTarget::cCheckedOptimalPath, -1);
                            } else {
                                let position = e.vcall(target, SLOT_POSITION, &args![]).u32();
                                if !e
                                    .call(
                                        CALCULATE_LOS,
                                        &args![attacker, target, goal, position, 0u32, u32::MAX],
                                    )
                                    .bool()
                                {
                                    e.set(
                                        this,
                                        CombatProcedureEngageTarget::cCheckedOptimalPath,
                                        -2,
                                    );
                                }
                            }
                            if e.get(this, CombatProcedureEngageTarget::cCheckedOptimalPath) < 0 {
                                stop = true;
                                force = true;
                                stamp_pathing_failed(e, this, ebp - 0x528);
                            }
                        }
                    }
                    if !stop {
                        let moved = x87_store32(
                            e.call(
                                WORLD_LOCATION_DISTANCE_SQUARED,
                                &args![target_location, t + TARGET_OPTIMAL_WORLD_LOCATION],
                            )
                            .f64(),
                        );
                        if (ranges_changed
                            || x87_load32(moved) > e.global::<f64>(NEAR_DISTANCE_SQUARED_DOUBLE))
                            && create_optimal_location_path(e, this, beyond_absolute)
                        {
                            return;
                        }
                    }
                }
                if stop {
                    if force {
                        e.call(FORCE_STOP_MOVING, &args![attacker]);
                    } else {
                        e.call(STOP_MOVING_ACTOR, &args![attacker]);
                    }
                }
                if e.call(IS_PATHING_COMPLETE, &args![attacker]).bool() || stop {
                    hold_or_crouch(e, this);
                    let mut wait = random_between(e, TWO_FLOAT, FIVE_FLOAT);
                    if ranges_changed {
                        wait = quiet_global(e, HUNDREDTH_FLOAT);
                    }
                    e.call(TIMER_START, &args![t + HOLDING_TIMER, wait]);
                }
            }
            _ => {}
        }
    });
}

/// Stamps the failed-pathing time (`+0x60`) with the current time, through
/// the local time stamp at `stamp`.
fn stamp_pathing_failed(e: &mut Engine, this: Ptr<CombatProcedureEngageTarget>, stamp: u32) {
    let now = e.call(TIME_NOW, &args![]).f64();
    e.call(TIMESTAMP_SET, &args![stamp, x87_store32(now)]);
    let value = e.mem.u32(stamp);
    e.mem.set_u32(this.addr() + PATHING_FAILED_TIMESTAMP, value);
}

/// The path to a dodge location: a request to its point within 16, its goal
/// point (copied into `goal`) set with the radius `distance + 256`, and the attacker's goal
/// (failing "Cannot setup path to dodge location"). Returns whether the
/// goal was set; the request is destroyed only on failure.
fn dodge_path(
    e: &mut Engine,
    this: Ptr<CombatProcedureEngageTarget>,
    attacker: u32,
    request: u32,
    location: u32,
    goal: u32,
    distance: u32,
) -> bool {
    e.call(PATHING_REQUEST_CONSTRUCT, &args![request]);
    let sixteen = quiet_global(e, SIXTEEN_FLOAT);
    let worldspace = e.call(GET_WORLD_SPACE, &args![attacker]).u32();
    let cell = e.call(ACTOR_CELL_8D6F30, &args![attacker]).u32();
    let point = e.call(DODGE_LOCATION_POINT, &args![location]).u32();
    e.call(
        BUILD_REQUEST,
        &args![attacker, request, point, cell, worldspace, sixteen, 0u32],
    );
    let request_location = e.call(REQUEST_LOCATION, &args![request]).u32();
    e.call(LOCATION_POINT, &args![request_location, goal]);
    let radius = x87_store32(x87_add(
        x87_load32(distance),
        e.global::<f64>(TWO_HUNDRED_FIFTY_SIX_DOUBLE),
    ));
    e.call(REQUEST_SET_GOAL, &args![request, goal, radius]);
    if !e
        .call(SET_PATHFINDING_GOAL, &args![attacker, request])
        .bool()
    {
        e.call(FAIL_PROCEDURE, &args![this, TEXT_CANNOT_SETUP_DODGE_PATH]);
        e.call(PATHING_REQUEST_DESTRUCT, &args![request]);
        return false;
    }
    true
}

// Translated from 009d4f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the byte at `+0x188` to 0xff (`__thiscall`; on the combat state).
pub fn fn_009d4f30(e: &mut Engine, this: u32) {
    e.mem.set_u8(this + 0x188, 0xff);
}

// Translated from 009d4f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the procedure may crouch: the controller's byte `+0xd4` is set
/// and its target is at least 256 (65536 squared) from its actor.
pub fn fn_009d4f50(e: &mut Engine, this: Ptr<CombatProcedureEngageTarget>) -> bool {
    let controller = controller_of(e, this);
    if !e.call(CONTROLLER_BYTE_D4, &args![controller]).bool() {
        return false;
    }
    e.with_stack(0x24, |e, frame| {
        let target_position = frame.addr();
        let actor_position = target_position + 0xc;
        let difference = actor_position + 0xc;
        let controller = controller_of(e, this);
        let target = e.call(CONTROLLER_TARGET, &args![controller]).u32();
        let position = e.vcall(target, SLOT_POSITION, &args![]).u32();
        copy_words(e, position, target_position, 3);
        let controller = controller_of(e, this);
        let actor = e.call(CONTROLLER_ACTOR, &args![controller]).u32();
        let position = e.vcall(actor, SLOT_POSITION, &args![]).u32();
        copy_words(e, position, actor_position, 3);
        e.call(
            POINT_DIFFERENCE,
            &args![target_position, difference, actor_position],
        );
        let squared = e.call(POINT_LENGTH_SQUARED, &args![difference]).f64();
        // FCOMP 65536; TEST AH,5; JP: true unless less.
        !(squared < e.global::<f64>(NEAR_DISTANCE_SQUARED_DOUBLE))
    })
}

// Translated from 009d5000 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009d5000 - the compiler's FS:[0] exception frame (unwinding of the pathing requests) is not translated
/// `CombatProcedureEngageTarget::CreateOptimalLocationPath` (Xbox PDB; one
/// word: whether the target is beyond the absolute range): nothing within
/// 5 seconds of a failed attempt. With the global `011a5b70` set it paths
/// with a `PathingRequestOptimalLocation` around the target (the ranges,
/// half the minimum, the absolute range); otherwise it asks the combat
/// state for an optimal location (`0099e840`) and its movement check, and
/// paths there, keeping it as the destination. On success it keeps the
/// target's location and moves (mode 3 when far, else 1).
pub fn create_optimal_location_path(
    e: &mut Engine,
    this: Ptr<CombatProcedureEngageTarget>,
    far: bool,
) -> bool {
    const FRAME: u32 = 0x1f0;
    let t = this.addr();
    let since = e
        .call(TIMER_ELAPSED, &args![t + PATHING_FAILED_TIMESTAMP])
        .f64();
    if since < e.global::<f64>(FIVE_DOUBLE) {
        return false;
    }
    e.set(this, CombatProcedureEngageTarget::cCheckedOptimalPath, 1);
    let move_mode = if far { 3u32 } else { 1 };
    e.with_stack(FRAME, |e, base| {
        let ebp = base.addr() + FRAME;
        if e.mem.u8(USE_OPTIMAL_LOCATION_REQUEST) != 0 {
            let attacker = e.call(GET_ATTACKER, &args![t]).u32();
            let target = e.call(GET_TARGET, &args![t]).u32();
            let request = ebp - 0xd4;
            e.call(OPTIMAL_REQUEST_CONSTRUCT, &args![request]);
            let sixteen = quiet_global(e, SIXTEEN_FLOAT);
            let worldspace = e.call(GET_WORLD_SPACE, &args![attacker]).u32();
            let cell = e.call(ACTOR_CELL_8D6F30, &args![attacker]).u32();
            let position = e.vcall(target, SLOT_POSITION, &args![]).u32();
            e.call(
                BUILD_REQUEST,
                &args![attacker, request, position, cell, worldspace, sixteen, 0u32],
            );
            let minimum = x87_quiet_bits(e.mem.u32(t + 0x14));
            e.call(REQUEST_SET_MIN_RANGE, &args![request, minimum]);
            let maximum = x87_quiet_bits(e.mem.u32(t + 0x18));
            e.call(REQUEST_SET_MAX_RANGE, &args![request, maximum]);
            let half = x87_store32(x87_mul(
                x87_load32(e.mem.u32(t + 0x14)),
                e.global::<f64>(HALF_DOUBLE),
            ));
            e.call(REQUEST_SET_HALF_MIN_RANGE, &args![request, half]);
            let absolute = x87_quiet_bits(e.mem.u32(t + 0x1c));
            e.call(REQUEST_SET_ABSOLUTE_RANGE, &args![request, absolute]);
            if !e
                .call(SET_PATHFINDING_GOAL, &args![attacker, request])
                .bool()
            {
                e.call(OPTIMAL_REQUEST_DESTRUCT, &args![request]);
                return false;
            }
            let location = e
                .call(GET_WORLD_LOCATION, &args![target, ebp - 0x1c8])
                .u32();
            copy_words(e, location, t + TARGET_OPTIMAL_WORLD_LOCATION, 4);
            e.call(SET_MOVE_MODE, &args![t, move_mode]);
            e.set(this, CombatProcedureEngageTarget::cCheckedOptimalPath, 0);
            e.call(OPTIMAL_REQUEST_DESTRUCT, &args![request]);
            return true;
        }
        let attacker = e.call(GET_ATTACKER, &args![t]).u32();
        let target = e.call(GET_TARGET, &args![t]).u32();
        let location = ebp - 0xfc;
        e.call(DODGE_LOCATION_CONSTRUCT, &args![location]);
        let controller = controller_of(e, this);
        let combat_state = e.call(GET_COMBAT_STATE, &args![controller]).u32();
        let absolute = x87_quiet_bits(e.mem.u32(t + 0x1c));
        let half = x87_store32(x87_mul(
            x87_load32(e.mem.u32(t + 0x14)),
            e.global::<f64>(HALF_DOUBLE),
        ));
        let maximum = x87_quiet_bits(e.mem.u32(t + 0x18));
        let minimum = x87_quiet_bits(e.mem.u32(t + 0x14));
        if !e
            .call(
                STATE_FIND_OPTIMAL_LOCATION,
                &args![
                    combat_state,
                    minimum,
                    maximum,
                    half,
                    absolute,
                    location,
                    1u32
                ],
            )
            .bool()
        {
            return false;
        }
        let point = e.call(POINT_CONSTRUCT, &args![location]).u32();
        if !e
            .call(CHECK_MOVEMENT, &args![combat_state, point, 1u32, 0u32])
            .bool()
        {
            return false;
        }
        let request = ebp - 0x1b4;
        e.call(PATHING_REQUEST_CONSTRUCT, &args![request]);
        let sixteen = quiet_global(e, SIXTEEN_FLOAT);
        let worldspace = e.call(GET_WORLD_SPACE, &args![attacker]).u32();
        let cell = e.call(ACTOR_CELL_8D6F30, &args![attacker]).u32();
        let goal = e.call(DODGE_LOCATION_POINT, &args![location]).u32();
        e.call(
            BUILD_REQUEST,
            &args![attacker, request, goal, cell, worldspace, sixteen, 0u32],
        );
        if !e
            .call(SET_PATHFINDING_GOAL, &args![attacker, request])
            .bool()
        {
            e.call(PATHING_REQUEST_DESTRUCT, &args![request]);
            return false;
        }
        let point = e.call(POINT_CONSTRUCT, &args![location]).u32();
        copy_words(e, point, t + DESTINATION, 4);
        let target_location = e
            .call(GET_WORLD_LOCATION, &args![target, ebp - 0x1dc])
            .u32();
        copy_words(e, target_location, t + TARGET_OPTIMAL_WORLD_LOCATION, 4);
        e.call(SET_MOVE_MODE, &args![t, move_mode]);
        e.call(PATHING_REQUEST_DESTRUCT, &args![request]);
        true
    })
}

// Translated from 009d5410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureEngageTarget::DisplayDebugText` (Xbox PDB): the state
/// (switch table `009d5854`) with its timer, the optimal range and the
/// target's movement since the plan, the lost-LOS timer of a left/right
/// dodge, and within 5 seconds of a failed optimal path its reason.
pub fn display_debug_text(
    e: &mut Engine,
    this: Ptr<CombatProcedureEngageTarget>,
    text: u32,
    first: u32,
    second: u32,
) {
    let t = this.addr();
    let target = e.call(GET_TARGET, &args![t]).u32();
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
    let print_range = |e: &mut Engine, location: u32| {
        let maximum = x87_load32(e.mem.u32(t + 0x18));
        let minimum = x87_load32(e.mem.u32(t + 0x14));
        e.vcall(
            text,
            SLOT_PRINT,
            &args![first, second, TEXT_RANGE, minimum, maximum],
        );
        if target != 0 {
            e.call(GET_WORLD_LOCATION, &args![target, location]);
            let limit = e.global::<f64>(TWO_HUNDRED_FIFTY_SIX_DOUBLE);
            let moved = e
                .call(
                    WORLD_LOCATION_DISTANCE,
                    &args![location, t + TARGET_OPTIMAL_WORLD_LOCATION],
                )
                .f64();
            e.vcall(
                text,
                SLOT_PRINT,
                &args![first, second, TEXT_TARGET_MOVEMENT, moved, limit],
            );
        }
    };
    e.with_stack(0x20, |e, locations| {
        match e.get(this, CombatProcedureEngageTarget::eState) {
            0 => print(e, TEXT_INITIALIZING),
            1 => {
                print(e, TEXT_HOLDING);
                print_timer(e, TEXT_HOLDING_TIMER, HOLDING_TIMER);
            }
            2 => {
                print(e, TEXT_CROUCHING);
                print_timer(e, TEXT_CROUCHING_TIMER, HOLDING_TIMER);
            }
            3 => {
                print(e, TEXT_MOVING_OPTIMAL_CLOSE);
                print_range(e, locations.addr() + 0x10);
            }
            4 => {
                print(e, TEXT_MOVING_OPTIMAL_FAR);
                print_range(e, locations.addr());
            }
            5 => print(e, TEXT_APPROACHING_TARGET),
            6 => print(e, TEXT_DODGING_FORWARD_BACKWARD),
            7 => {
                print(e, TEXT_DODGING_LEFT_RIGHT);
                if e.get(this, CombatProcedureEngageTarget::bLeftRightDodgeHadLOS)
                    && e.call(TIMER_RUNNING, &args![t + DODGE_LOS_TIMER]).bool()
                {
                    print_timer(e, TEXT_LOST_LOS_TIMER, DODGE_LOS_TIMER);
                }
            }
            8 => print(e, TEXT_MOVING_TO_TARGET),
            _ => {}
        }
    });
    let since = e
        .call(TIMER_ELAPSED, &args![t + PATHING_FAILED_TIMESTAMP])
        .f64();
    if since < e.global::<f64>(FIVE_DOUBLE) {
        let format = match e.get(this, CombatProcedureEngageTarget::cCheckedOptimalPath) {
            -1 => TEXT_OPTIMAL_MOVEMENT_FAILED,
            -2 => TEXT_OPTIMAL_LOS_FAILED,
            _ => TEXT_OPTIMAL_BUILD_FAILED,
        };
        let five = e.global::<f64>(FIVE_DOUBLE);
        let value = e
            .call(TIMESTAMP_VALUE_FA90, &args![t + PATHING_FAILED_TIMESTAMP])
            .f64();
        e.vcall(text, SLOT_PRINT, &args![first, second, format, value, five]);
    }
}

// Translated from 009d5880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureEngageTarget::SaveGame` (Xbox PDB): the base class, the
/// state and ranges, the two timers, the dodge flags and start, the two
/// world locations, the optimal-path check and the time stamp.
pub fn save_game(e: &mut Engine, this: Ptr<CombatProcedureEngageTarget>, buffer: u32) {
    let t = this.addr();
    e.call(BASE_SAVE_GAME, &args![t, buffer]);
    for offset in [0x10u32, 0x14, 0x18, 0x1c] {
        e.call(SAVE_BYTES, &args![buffer, t + offset, 4u32, 0u32]);
    }
    e.call(TIMER_SAVE, &args![t + HOLDING_TIMER, buffer]);
    e.call(TIMER_SAVE, &args![t + DODGE_LOS_TIMER, buffer]);
    for offset in [0x30u32, 0x31, 0x32] {
        e.call(SAVE_BYTES, &args![buffer, t + offset, 1u32, 0u32]);
    }
    e.call(
        SAVE_BYTES,
        &args![buffer, t + DODGE_START_LOCATION, 0xcu32, 0u32],
    );
    e.call(WORLD_LOCATION_SAVE, &args![t + DESTINATION, buffer]);
    e.call(
        WORLD_LOCATION_SAVE,
        &args![t + TARGET_OPTIMAL_WORLD_LOCATION, buffer],
    );
    e.call(SAVE_BYTES, &args![buffer, t + 0x33, 1u32, 0u32]);
    e.call(TIMESTAMP_SAVE, &args![t + PATHING_FAILED_TIMESTAMP, buffer]);
}

// Translated from 009d5990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureEngageTarget::LoadGame` (Xbox PDB): the mirror of
/// `SaveGame`; saves before version 14 have no absolute range (it becomes
/// 10000), and the optimal-path check and the time stamp only come with
/// version 10 or later (the buffer's version is its virtual function at
/// slot 0).
pub fn load_game(e: &mut Engine, this: Ptr<CombatProcedureEngageTarget>, buffer: u32) {
    let t = this.addr();
    e.call(BASE_LOAD_GAME, &args![t, buffer]);
    for offset in [0x10u32, 0x14, 0x18] {
        e.call(LOAD_BYTES, &args![buffer, t + offset, 4u32]);
    }
    if e.vcall(buffer, 0, &args![]).u8() >= 0xe {
        e.call(LOAD_BYTES, &args![buffer, t + 0x1c, 4u32]);
    } else {
        let absolute = quiet_global(e, TEN_THOUSAND_FLOAT);
        e.mem.set_u32(t + 0x1c, absolute);
    }
    e.call(TIMER_LOAD, &args![t + HOLDING_TIMER, buffer]);
    e.call(TIMER_LOAD, &args![t + DODGE_LOS_TIMER, buffer]);
    for offset in [0x30u32, 0x31, 0x32] {
        e.call(LOAD_BYTES, &args![buffer, t + offset, 1u32]);
    }
    e.call(LOAD_BYTES, &args![buffer, t + DODGE_START_LOCATION, 0xcu32]);
    e.call(WORLD_LOCATION_LOAD, &args![t + DESTINATION, buffer]);
    e.call(
        WORLD_LOCATION_LOAD,
        &args![t + TARGET_OPTIMAL_WORLD_LOCATION, buffer],
    );
    if e.vcall(buffer, 0, &args![]).u8() >= 0xa {
        e.call(LOAD_BYTES, &args![buffer, t + 0x33, 1u32]);
        e.call(TIMESTAMP_LOAD, &args![t + PATHING_FAILED_TIMESTAMP, buffer]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009d_3b30, fn_009d3b30(Ptr)),
        entry!(
            0x009d_3a00,
            construct(Ptr<CombatProcedureEngageTarget>) -> Ptr<CombatProcedureEngageTarget>
        ),
        entry!(
            0x009d_3b00,
            scalar_deleting_destructor(
                Ptr<CombatProcedureEngageTarget>,
                u32,
            ) -> Ptr<CombatProcedureEngageTarget>
        ),
        entry!(0x009d_3b50, shutdown(Ptr<CombatProcedureEngageTarget>)),
        entry!(0x009d_3b80, update(Ptr<CombatProcedureEngageTarget>)),
        entry!(0x009d_4f30, fn_009d4f30(u32)),
        entry!(
            0x009d_4f50,
            fn_009d4f50(Ptr<CombatProcedureEngageTarget>) -> bool
        ),
        entry!(
            0x009d_5000,
            create_optimal_location_path(Ptr<CombatProcedureEngageTarget>, bool) -> bool
        ),
        entry!(
            0x009d_5410,
            display_debug_text(Ptr<CombatProcedureEngageTarget>, u32, u32, u32)
        ),
        entry!(
            0x009d_5880,
            save_game(Ptr<CombatProcedureEngageTarget>, u32)
        ),
        entry!(
            0x009d_5990,
            load_game(Ptr<CombatProcedureEngageTarget>, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    #[test]
    fn fn_009d3b30_stores_the_vtable_then_forwards_this() {
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
        e.call(0x009d_3b30, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x009c_a6c0)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0109_1514);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0109_1514,
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
        CLEAR_MOVE_MODE,
        SET_LOOK_AT_TARGET,
        CLEAR_LOOK_AT_TARGET,
        BASE_SAVE_GAME,
        BASE_LOAD_GAME,
        FAIL_PROCEDURE,
        OPERATOR_DELETE,
        GET_COMBAT_STATE,
        CONTROLLER_WEAPON,
        CONTROLLER_TARGET,
        CONTROLLER_ACTOR,
        CONTROLLER_BYTE_D4,
        CONTROLLER_BYTE_D5,
        GET_LAST_SEEN_TARGET_FIRING_LOCATION,
        CHECK_MOVEMENT,
        FIND_BEST_DODGE_LOCATION,
        ADD_UNREACHABLE_LOCATION,
        STATE_TIMESTAMP_COPY,
        STATE_BYTE_71,
        STATE_CHECK_LOCATION,
        STATE_FIND_OPTIMAL_LOCATION,
        COMPUTE_OPTIMAL_WEAPON_RANGES,
        CALCULATE_LOS,
        FIND_CLOSEST_POINT_ON_NAVMESH,
        STOP_MOVING_ACTOR,
        FORCE_STOP_MOVING,
        IS_PATH_VALID,
        IS_PATHING_COMPLETE,
        IS_WAITING_ON_PATH,
        GET_CURRENT_PATHFINDING_GOAL,
        SET_PATHFINDING_GOAL,
        BUILD_REQUEST,
        BUILD_REQUEST_OV2,
        ACTOR_PATH_TEST,
        ACTOR_CELL_8D6F30,
        ACTOR_TEST_4997B0,
        GET_WORLD_SPACE,
        GET_WORLD_LOCATION,
        GET_SAVED_ACQUIRE_OBJECT,
        WORLD_LOCATION_DISTANCE_SQUARED,
        WORLD_LOCATION_DISTANCE,
        WORLD_LOCATION_CELL_OR_WORLD,
        WORLD_LOCATION_SAVE,
        WORLD_LOCATION_LOAD,
        WORLD_LOCATION_CONSTRUCT,
        DODGE_LOCATION_CONSTRUCT,
        DODGE_LOCATION_POINT,
        WORLD_LOCATION_WORD_C,
        POINT_CONSTRUCT,
        WORLD_LOCATION_SET_POINT,
        WORLD_LOCATION_SET_WORD,
        PATHING_REQUEST_CONSTRUCT,
        PATHING_REQUEST_DESTRUCT,
        OPTIMAL_REQUEST_CONSTRUCT,
        OPTIMAL_REQUEST_DESTRUCT,
        REQUEST_SET_MIN_RANGE,
        REQUEST_SET_MAX_RANGE,
        REQUEST_SET_HALF_MIN_RANGE,
        REQUEST_SET_ABSOLUTE_RANGE,
        REQUEST_LOCATION,
        LOCATION_POINT,
        REQUEST_SET_GOAL,
        TIMER_CONSTRUCT,
        TIMER_RUNNING,
        TIMER_START,
        TIMER_EXPIRED,
        TIMER_CLEAR,
        TIMER_ELAPSED,
        TIMER_DURATION,
        TIME_NOW,
        TIMESTAMP_SET,
        TIMESTAMP_VALUE_FA90,
        TIMER_SAVE,
        TIMER_LOAD,
        TIMESTAMP_SAVE,
        TIMESTAMP_LOAD,
        RANDOM_FLOAT,
        RANDOM_CHANCE,
        POINT_DIFFERENCE,
        POINT_LENGTH_SQUARED,
        SAVE_BYTES,
        LOAD_BYTES,
        // The base destructor `fn_009d3b30` calls.
        0x009c_a6c0,
    ];

    struct World {
        rig: Rig,
        this: Ptr<CombatProcedureEngageTarget>,
        t: u32,
        controller: u32,
        attacker: u32,
        target: u32,
        state: u32,
        position: u32,
        stamp: u32,
        acquire: u32,
    }

    /// A procedure with ranges 100 / 200 / 1000; the attacker, the target
    /// and the combat state; the world locations written as the reference
    /// and three counting words; the elapsed time of a timer or stamp read
    /// from its `+4` (`set_elapsed`); `POINT_CONSTRUCT` returning `this`;
    /// `TIMESTAMP_SET` storing its value.
    fn world() -> World {
        let mut rig = Rig::new();
        let t = rig.e.mem.alloc(0x80);
        let this = Ptr::<CombatProcedureEngageTarget>::new(t);
        let controller = rig.object(0x100);
        rig.e.set(
            this,
            CombatProcedureEngageTarget::pCombatController,
            Ptr::new(controller),
        );
        rig.e.mem.set_f32(t + 0x14, 100.0);
        rig.e.mem.set_f32(t + 0x18, 200.0);
        rig.e.mem.set_f32(t + 0x1c, 1000.0);
        let attacker = rig.actor();
        let target = rig.actor();
        let state = rig.e.mem.alloc(0x200);
        let position = rig.e.mem.alloc(0xc);
        rig.e.mem.set_f32(position, 7.0);
        rig.e.mem.set_f32(position + 4, 8.0);
        rig.e.mem.set_f32(position + 8, 9.0);
        let stamp = rig.e.mem.alloc(8);
        let acquire = rig.object(8);
        rig.answer(GET_SAVED_ACQUIRE_OBJECT, acquire);
        rig.answer(GET_ATTACKER, attacker);
        rig.answer(GET_TARGET, target);
        rig.answer(GET_COMBAT_STATE, state);
        rig.answer(CONTROLLER_BYTE_D5, 1);
        rig.slot(SLOT_POSITION, position);
        rig.e
            .register_double(STATE_TIMESTAMP_COPY, move |_, _| ret(stamp));
        rig.e
            .register_double(TIMER_ELAPSED, |e, a| float(e.mem.f32(a[0] + 4) as f64));
        rig.e.register_double(GET_WORLD_LOCATION, |e, a| {
            for i in 0..4 {
                e.mem.set_u32(a[1] + i * 4, a[0] + i);
            }
            ret(a[1])
        });
        rig.e.register_double(POINT_CONSTRUCT, |_, a| ret(a[0]));
        rig.e.register_double(TIMESTAMP_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        World {
            rig,
            this,
            t,
            controller,
            attacker,
            target,
            state,
            position,
            stamp,
            acquire,
        }
    }

    impl World {
        fn state(&self) -> u32 {
            self.rig
                .e
                .get(self.this, CombatProcedureEngageTarget::eState)
        }

        fn set_state(&mut self, state: u32) {
            self.rig
                .e
                .set(self.this, CombatProcedureEngageTarget::eState, state);
        }

        /// The elapsed time the timer (or stamp) at `addr` reports.
        fn set_elapsed(&mut self, addr: u32, seconds: f32) {
            self.rig.e.mem.set_f32(addr + 4, seconds);
        }

        /// The squared distance from the attacker to the target.
        fn set_distance(&mut self, distance: f64) {
            self.rig
                .answer_float(WORLD_LOCATION_DISTANCE_SQUARED, distance * distance);
        }

        fn update(&mut self) -> Log {
            let t = self.t;
            self.rig.run(0x009d_3b80, &args![t]).1
        }
    }

    #[test]
    fn construct_sets_up_the_procedure() {
        let mut w = world();
        let block = w.rig.e.mem.alloc(0x80);
        w.rig.e.mem.write(block, &[0xcc; 0x64]);
        w.rig.e.mem.set_f32(DEFAULT_POINT, 1.0);
        w.rig.e.mem.set_f32(DEFAULT_POINT + 4, 2.0);
        w.rig.e.mem.set_f32(DEFAULT_POINT + 8, 3.0);
        let (result, log) = w.rig.run(0x009d_3a00, &args![block]);
        assert_eq!(result.u32(), block);
        assert_eq!(calls_to(&log, BASE_CONSTRUCT), vec![vec![block]]);
        assert_eq!(w.rig.e.mem.u32(block), VTABLE);
        assert_eq!(w.rig.e.mem.u32(block + 0x10), 0);
        for offset in [0x14, 0x18, 0x1c] {
            assert_eq!(w.rig.e.mem.u32(block + offset), 0);
        }
        assert_eq!(
            calls_to(&log, TIMER_CONSTRUCT),
            vec![vec![block + 0x20], vec![block + 0x28]]
        );
        let flags: Vec<_> = (0x30..0x34).map(|o| w.rig.e.mem.u8(block + o)).collect();
        assert_eq!(flags, vec![0, 0, 0, 1]);
        assert_eq!(w.rig.e.mem.f32(block + 0x34), 1.0);
        assert_eq!(w.rig.e.mem.f32(block + 0x3c), 3.0);
        assert_eq!(
            calls_to(&log, WORLD_LOCATION_CONSTRUCT),
            vec![vec![block + 0x40], vec![block + 0x50]]
        );
        assert_eq!(w.rig.e.mem.f32(block + 0x60), -f32::MAX);
    }

    #[test]
    fn the_deleting_destructor_frees_on_bit_0() {
        let mut w = world();
        let (result, log) = w.rig.run(0x009d_3b00, &args![w.t, 1u32]);
        assert_eq!(result.u32(), w.t);
        assert_eq!(w.rig.e.mem.u32(w.t), VTABLE);
        assert_eq!(calls_to(&log, 0x009c_a6c0), vec![vec![w.t]]);
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![w.t]]);
        let (_, log) = w.rig.run(0x009d_3b00, &args![w.t, 0u32]);
        assert!(calls_to(&log, OPERATOR_DELETE).is_empty());
    }

    #[test]
    fn shutdown_stops_the_attacker() {
        let mut w = world();
        let (_, log) = w.rig.run(0x009d_3b50, &args![w.t]);
        assert_eq!(calls_to(&log, CLEAR_MOVE_MODE), vec![vec![w.t]]);
        assert_eq!(calls_to(&log, STOP_MOVING_ACTOR), vec![vec![w.attacker]]);
        assert_eq!(calls_to(&log, CLEAR_LOOK_AT_TARGET), vec![vec![w.t]]);
    }

    #[test]
    fn update_starts_by_looking_and_waiting() {
        // No target: nothing past the combat state.
        let mut w = world();
        w.rig.answer(GET_TARGET, 0);
        let log = w.update();
        assert_eq!(calls_to(&log, GET_COMBAT_STATE), vec![vec![w.controller]]);
        assert!(calls_to(&log, GET_LAST_SEEN_TARGET_FIRING_LOCATION).is_empty());

        // The controller's byte `+0xd5` clear: back to holding.
        let mut w = world();
        w.set_state(4);
        w.rig.answer(CONTROLLER_BYTE_D5, 0);
        w.rig.answer(GET_LAST_SEEN_TARGET_FIRING_LOCATION, 1);
        let log = w.update();
        let seen = calls_to(&log, GET_LAST_SEEN_TARGET_FIRING_LOCATION);
        assert_eq!(seen[0][0], w.state);
        assert_eq!(
            calls_to(&log, SET_LOOK_AT_TARGET),
            vec![vec![w.t, seen[0][1], 1]]
        );
        assert_eq!(w.state(), 1);
        assert!(calls_to(&log, COMPUTE_OPTIMAL_WEAPON_RANGES).is_empty());

        // Initializing: the ranges, then 0.1 to 0.5 seconds of holding.
        let mut w = world();
        w.rig.answer(CONTROLLER_WEAPON, 0x6600);
        w.rig.answer_float(RANDOM_FLOAT, 0.25);
        let t = w.t;
        let log = w.update();
        assert_eq!(
            calls_to(&log, COMPUTE_OPTIMAL_WEAPON_RANGES),
            vec![vec![w.controller, 0x6600, t + 0x14, t + 0x18, t + 0x1c]]
        );
        let locations = calls_to(&log, GET_WORLD_LOCATION);
        assert_eq!(locations[0][0], w.target);
        assert_eq!(locations[1][0], w.attacker);
        assert_eq!(
            calls_to(&log, WORLD_LOCATION_DISTANCE_SQUARED),
            vec![vec![locations[1][1], locations[0][1]]]
        );
        assert_eq!(
            calls_to(&log, RANDOM_FLOAT),
            vec![vec![bits(0.1), bits(0.5)]]
        );
        assert_eq!(
            calls_to(&log, TIMER_START),
            vec![vec![t + 0x20, bits(0.25)]]
        );
        assert_eq!(w.state(), 1);
    }

    #[test]
    fn update_holding_approaches_a_close_target() {
        let mut w = world();
        let t = w.t;
        w.set_state(1);
        w.set_distance(150.0);
        let stamp = w.stamp;
        w.set_elapsed(stamp, 1.0);
        w.rig.answer(CHECK_MOVEMENT, 1);
        w.rig.answer(WORLD_LOCATION_WORD_C, 0x4400);
        w.rig.answer(SET_PATHFINDING_GOAL, 1);
        let log = w.update();
        let target_location = calls_to(&log, GET_WORLD_LOCATION)[0][1];
        assert_eq!(calls_to(&log, STATE_TIMESTAMP_COPY)[0][0], w.state);
        assert_eq!(
            calls_to(&log, CHECK_MOVEMENT),
            vec![vec![w.state, target_location, 1, 0]]
        );
        let request = calls_to(&log, PATHING_REQUEST_CONSTRUCT)[0][0];
        assert_eq!(
            calls_to(&log, BUILD_REQUEST_OV2),
            vec![vec![
                w.attacker,
                request,
                target_location,
                0x4400,
                bits(-1.0),
                0
            ]]
        );
        assert_eq!(w.rig.e.mem.u32(t + 0x40), w.target);
        assert_eq!(w.rig.e.mem.u32(t + 0x4c), w.target + 3);
        assert_eq!(calls_to(&log, SET_MOVE_MODE), vec![vec![t, 1]]);
        assert_eq!(
            calls_to(&log, PATHING_REQUEST_DESTRUCT),
            vec![vec![request]]
        );
        assert_eq!(w.state(), 5);
    }

    #[test]
    fn update_holding_tracks_the_left_right_dodge_sight() {
        let mut w = world();
        let t = w.t;
        w.set_state(1);
        w.set_distance(150.0);
        w.rig.e.set(
            w.this,
            CombatProcedureEngageTarget::bLeftRightDodgeCheckLOS,
            true,
        );
        w.set_elapsed(t + 0x20, 0.25);
        w.update();
        assert!(w
            .rig
            .e
            .get(w.this, CombatProcedureEngageTarget::bLeftRightDodgeLostLOS));
        assert!(!w
            .rig
            .e
            .get(w.this, CombatProcedureEngageTarget::bLeftRightDodgeCheckLOS));

        // A second later: back to the dodge start.
        w.set_elapsed(t + 0x20, 1.5);
        w.rig.answer(STATE_BYTE_71, 1);
        w.rig.answer(GET_WORLD_SPACE, 0x3200);
        w.rig.answer(ACTOR_CELL_8D6F30, 0x3300);
        w.rig.answer(SET_PATHFINDING_GOAL, 1);
        let log = w.update();
        assert!(!w
            .rig
            .e
            .get(w.this, CombatProcedureEngageTarget::bLeftRightDodgeLostLOS));
        assert_eq!(w.rig.e.mem.u8(t + 0x30), 1);
        assert_eq!(calls_to(&log, TIMER_CLEAR), vec![vec![t + 0x28]]);
        let request = calls_to(&log, PATHING_REQUEST_CONSTRUCT)[0][0];
        assert_eq!(
            calls_to(&log, BUILD_REQUEST),
            vec![vec![w.attacker, request, t + 0x34, 0x3300, 0x3200, 0, 0]]
        );
        assert_eq!(w.state(), 7);
    }

    #[test]
    fn update_holding_moves_to_the_optimal_location() {
        // Beyond the maximum range: the optimal location (state 3).
        let mut w = world();
        let t = w.t;
        w.set_state(1);
        w.set_distance(300.0);
        w.rig.answer(TIMER_EXPIRED, 1);
        w.set_elapsed(t + 0x60, 10.0);
        w.rig.answer(STATE_FIND_OPTIMAL_LOCATION, 1);
        w.rig.answer(CHECK_MOVEMENT, 1);
        w.rig.answer(SET_PATHFINDING_GOAL, 1);
        let log = w.update();
        let location = calls_to(&log, DODGE_LOCATION_CONSTRUCT)[0][0];
        assert_eq!(
            calls_to(&log, STATE_FIND_OPTIMAL_LOCATION),
            vec![vec![
                w.state,
                bits(100.0),
                bits(200.0),
                bits(50.0),
                bits(1000.0),
                location,
                1
            ]]
        );
        assert_eq!(calls_to(&log, SET_MOVE_MODE), vec![vec![t, 1]]);
        assert_eq!(w.state(), 3);

        // Too close, with no optimal location: a dodge (kind 8).
        let mut w = world();
        let t = w.t;
        w.set_state(2);
        w.set_distance(50.0);
        w.rig.answer(TIMER_EXPIRED, 1);
        w.rig.answer_float(RANDOM_FLOAT, 150.0);
        w.rig.answer(FIND_BEST_DODGE_LOCATION, 1);
        w.rig.answer(DODGE_LOCATION_POINT, 0x4500);
        w.rig.answer(REQUEST_LOCATION, 0x4600);
        w.rig.answer(SET_PATHFINDING_GOAL, 1);
        let log = w.update();
        assert_eq!(
            calls_to(&log, RANDOM_FLOAT),
            vec![vec![bits(128.0), bits(256.0)]]
        );
        let location = calls_to(&log, DODGE_LOCATION_CONSTRUCT)[0][0];
        assert_eq!(
            calls_to(&log, FIND_BEST_DODGE_LOCATION),
            vec![vec![w.state, location, 8, 0, bits(150.0), bits(24.0)]]
        );
        let request = calls_to(&log, PATHING_REQUEST_CONSTRUCT)[0][0];
        assert_eq!(calls_to(&log, BUILD_REQUEST)[0][2], 0x4500);
        let goal = calls_to(&log, LOCATION_POINT)[0].clone();
        assert_eq!(goal[0], 0x4600);
        assert_eq!(
            calls_to(&log, REQUEST_SET_GOAL),
            vec![vec![request, goal[1], bits(406.0)]]
        );
        assert_eq!(calls_to(&log, SET_MOVE_MODE), vec![vec![t, 1]]);
        assert_eq!(w.state(), 6);

        // The dodge path cannot be set up.
        let mut w = world();
        w.set_state(2);
        w.set_distance(50.0);
        w.rig.answer(TIMER_EXPIRED, 1);
        w.rig.answer(FIND_BEST_DODGE_LOCATION, 1);
        let log = w.update();
        assert_eq!(
            calls_to(&log, FAIL_PROCEDURE),
            vec![vec![w.t, TEXT_CANNOT_SETUP_DODGE_PATH]]
        );
        assert_eq!(w.state(), 2);
    }

    #[test]
    fn update_holding_moves_to_a_far_target() {
        // Beyond the absolute range with a clear movement.
        let mut w = world();
        let t = w.t;
        w.set_state(1);
        w.set_distance(2000.0);
        w.rig.answer(TIMER_EXPIRED, 1);
        w.rig.answer(CHECK_MOVEMENT, 1);
        w.rig.answer(WORLD_LOCATION_WORD_C, 0x4400);
        w.rig.answer(SET_PATHFINDING_GOAL, 1);
        let log = w.update();
        let target_location = calls_to(&log, GET_WORLD_LOCATION)[0][1];
        let request = calls_to(&log, PATHING_REQUEST_CONSTRUCT)[0][0];
        assert_eq!(
            calls_to(&log, BUILD_REQUEST_OV2),
            vec![vec![
                w.attacker,
                request,
                target_location,
                0x4400,
                bits(128.0),
                0
            ]]
        );
        assert_eq!(w.rig.e.mem.u32(t + 0x40), w.target);
        assert_eq!(calls_to(&log, SET_MOVE_MODE), vec![vec![t, 3]]);
        assert_eq!(w.state(), 8);
        assert!(calls_to(&log, STATE_FIND_OPTIMAL_LOCATION).is_empty());

        // Through the closest navmesh point otherwise.
        let mut w = world();
        let t = w.t;
        w.set_state(1);
        w.set_distance(2000.0);
        w.rig.answer(TIMER_EXPIRED, 1);
        w.rig
            .e
            .register_double(WORLD_LOCATION_CELL_OR_WORLD, |e, a| {
                e.mem.set_u32(a[1], 0x3300);
                e.mem.set_u32(a[2], 0x3200);
                Ret::default()
            });
        w.rig.answer(FIND_CLOSEST_POINT_ON_NAVMESH, 1);
        w.rig.answer(STATE_CHECK_LOCATION, 1);
        w.rig.answer(WORLD_LOCATION_WORD_C, 0x4400);
        w.rig.answer(SET_PATHFINDING_GOAL, 1);
        let log = w.update();
        let target_location = calls_to(&log, GET_WORLD_LOCATION)[0][1];
        let navmesh = calls_to(&log, FIND_CLOSEST_POINT_ON_NAVMESH);
        assert_eq!(navmesh[0][..3], [0x3200, 0x3300, target_location]);
        let point = navmesh[0][3];
        assert_eq!(
            calls_to(&log, STATE_CHECK_LOCATION),
            vec![vec![w.state, point, 0x4400, 1, 0]]
        );
        let request = calls_to(&log, PATHING_REQUEST_CONSTRUCT)[0][0];
        assert_eq!(
            calls_to(&log, BUILD_REQUEST),
            vec![vec![
                w.attacker,
                request,
                point,
                0x3300,
                0x3200,
                bits(128.0),
                0
            ]]
        );
        assert_eq!(
            calls_to(&log, WORLD_LOCATION_SET_POINT),
            vec![vec![t + 0x40, point]]
        );
        assert_eq!(
            calls_to(&log, WORLD_LOCATION_SET_WORD),
            vec![vec![t + 0x40, 0x4400]]
        );
        assert_eq!(w.state(), 8);
    }

    #[test]
    fn update_holding_dodges_sideways_or_crouches_when_the_timer_runs_out() {
        // Both move animations and three in four: a left/right dodge.
        let mut w = world();
        let t = w.t;
        w.set_state(1);
        w.set_distance(150.0);
        w.rig.answer(TIMER_EXPIRED, 1);
        w.rig.slot(SLOT_HAS_MOVE_ANIM, 1);
        w.rig.answer(STATE_BYTE_71, 1);
        w.rig.answer_float(RANDOM_FLOAT, 200.0);
        w.rig.answer(FIND_BEST_DODGE_LOCATION, 1);
        w.rig.answer(SET_PATHFINDING_GOAL, 1);
        let log = w.update();
        assert_eq!(
            slot_calls(&log, SLOT_HAS_MOVE_ANIM),
            vec![vec![w.acquire, 5], vec![w.acquire, 6]]
        );
        assert_eq!(calls_to(&log, RANDOM_CHANCE), vec![vec![bits(0.25)]]);
        assert_eq!(
            calls_to(&log, RANDOM_FLOAT),
            vec![vec![bits(128.0), bits(384.0)]]
        );
        assert_eq!(calls_to(&log, FIND_BEST_DODGE_LOCATION)[0][2], 3);
        assert!(w
            .rig
            .e
            .get(w.this, CombatProcedureEngageTarget::bLeftRightDodgeCheckLOS));
        assert_eq!(w.rig.e.mem.f32(t + 0x34), 7.0);
        assert_eq!(w.rig.e.mem.f32(t + 0x3c), 9.0);
        assert_eq!(w.state(), 7);

        // Not seen: a shorter dodge.
        let mut w = world();
        w.set_state(1);
        w.set_distance(150.0);
        w.rig.answer(TIMER_EXPIRED, 1);
        w.rig.slot(SLOT_HAS_MOVE_ANIM, 1);
        let log = w.update();
        assert_eq!(
            calls_to(&log, RANDOM_FLOAT)[0],
            vec![bits(64.0), bits(192.0)]
        );

        // No move animations and no crouching: holds for 2 to 5 seconds.
        let mut w = world();
        let t = w.t;
        w.set_state(2);
        w.set_distance(150.0);
        w.rig.answer(TIMER_EXPIRED, 1);
        w.rig.answer_float(RANDOM_FLOAT, 3.0);
        let log = w.update();
        assert!(calls_to(&log, RANDOM_CHANCE).is_empty());
        assert_eq!(
            calls_to(&log, RANDOM_FLOAT),
            vec![vec![bits(2.0), bits(5.0)]]
        );
        assert_eq!(calls_to(&log, TIMER_START), vec![vec![t + 0x20, bits(3.0)]]);
        assert_eq!(w.state(), 1);

        // Crouching allowed without the move flag: crouches.
        let mut w = world();
        let t = w.t;
        w.set_state(1);
        w.set_distance(150.0);
        w.rig.answer(TIMER_EXPIRED, 1);
        w.rig.answer(CONTROLLER_BYTE_D4, 1);
        w.rig.answer(CONTROLLER_TARGET, w.target);
        w.rig.answer(CONTROLLER_ACTOR, w.attacker);
        w.rig.answer_float(POINT_LENGTH_SQUARED, 70000.0);
        let log = w.update();
        assert_eq!(calls_to(&log, SET_MOVE_MODE), vec![vec![t, 2]]);
        assert_eq!(w.state(), 2);

        // With the move flag, a half chance that fails: clears the mode.
        let mut w = world();
        w.set_state(1);
        w.set_distance(150.0);
        w.rig.answer(TIMER_EXPIRED, 1);
        w.rig.answer(CONTROLLER_BYTE_D4, 1);
        w.rig.answer(CONTROLLER_TARGET, w.target);
        w.rig.answer(CONTROLLER_ACTOR, w.attacker);
        w.rig.answer_float(POINT_LENGTH_SQUARED, 70000.0);
        w.rig.answer(ACTOR_TEST_4997B0, 1);
        let log = w.update();
        assert_eq!(calls_to(&log, RANDOM_CHANCE), vec![vec![bits(0.5)]]);
        assert_eq!(calls_to(&log, CLEAR_MOVE_MODE), vec![vec![w.t]]);
        assert_eq!(w.state(), 1);
    }

    #[test]
    fn update_dodging_follows_the_path() {
        // A broken path: the combat state's byte and the reason.
        let mut w = world();
        w.set_state(6);
        w.rig.answer(ACTOR_PATH_TEST, 1);
        let log = w.update();
        assert_eq!(w.rig.e.mem.u8(w.state + 0x188), 0xff);
        assert_eq!(
            calls_to(&log, FAIL_PROCEDURE),
            vec![vec![w.t, TEXT_PATH_BUILDING_FAILED_DODGE]]
        );
        let mut w = world();
        w.set_state(6);
        w.rig.answer(ACTOR_PATH_TEST, 1);
        w.rig.answer(IS_PATH_VALID, 1);
        let log = w.update();
        assert_eq!(w.rig.e.mem.u8(w.state + 0x188), 0);
        assert_eq!(
            calls_to(&log, FAIL_PROCEDURE),
            vec![vec![w.t, TEXT_PATHING_FAILED_DODGE]]
        );

        // Left/right, sight lost: half a second, then a stop.
        let mut w = world();
        let t = w.t;
        w.set_state(7);
        w.rig.e.mem.set_u8(t + 0x30, 1);
        w.rig.e.mem.set_u8(t + 0x32, 1);
        let log = w.update();
        assert_eq!(w.rig.e.mem.u8(t + 0x32), 0);
        assert_eq!(calls_to(&log, TIMER_START), vec![vec![t + 0x28, bits(0.5)]]);
        w.rig.answer(TIMER_RUNNING, 1);
        w.rig.answer(TIMER_EXPIRED, 1);
        let log = w.update();
        assert_eq!(calls_to(&log, STOP_MOVING_ACTOR), vec![vec![w.attacker]]);
        assert_eq!(w.rig.e.mem.u8(t + 0x30), 0);
        assert_eq!(calls_to(&log, TIMER_CLEAR), vec![vec![t + 0x28]]);

        // There: holds again.
        let mut w = world();
        let t = w.t;
        w.set_state(6);
        w.rig.answer(IS_PATHING_COMPLETE, 1);
        w.rig.answer_float(RANDOM_FLOAT, 4.0);
        let log = w.update();
        assert_eq!(calls_to(&log, TIMER_START), vec![vec![t + 0x20, bits(4.0)]]);
        assert_eq!(w.state(), 1);
    }

    #[test]
    fn update_moving_handles_a_broken_path() {
        // To the optimal location: the failed stamp and a stop.
        let mut w = world();
        let t = w.t;
        w.set_state(4);
        w.rig.answer(ACTOR_PATH_TEST, 1);
        w.rig.answer_float(TIME_NOW, 12.0);
        let log = w.update();
        assert_eq!(w.rig.e.mem.f32(t + 0x60), 12.0);
        assert_eq!(calls_to(&log, STOP_MOVING_ACTOR), vec![vec![w.attacker]]);
        assert!(calls_to(&log, FAIL_PROCEDURE).is_empty());
        assert_eq!(w.state(), 1);

        // Approaching: the destination is unreachable, then the reason.
        let mut w = world();
        let t = w.t;
        w.set_state(5);
        w.rig.answer(ACTOR_PATH_TEST, 1);
        let log = w.update();
        assert_eq!(
            calls_to(&log, ADD_UNREACHABLE_LOCATION),
            vec![vec![w.state, t + 0x40]]
        );
        assert_eq!(
            calls_to(&log, FAIL_PROCEDURE),
            vec![vec![t, TEXT_PATH_BUILDING_FAILED_APPROACH]]
        );

        // A valid path that still fails, per state.
        for (state, reason) in [
            (3, TEXT_PATHING_FAILED_FAR),
            (4, TEXT_PATHING_FAILED_CLOSE),
            (5, TEXT_PATHING_FAILED_APPROACH),
            (8, TEXT_PATHING_FAILED_TARGET),
        ] {
            let mut w = world();
            w.set_state(state);
            w.rig.answer(ACTOR_PATH_TEST, 1);
            w.rig.answer(IS_PATH_VALID, 1);
            let log = w.update();
            assert_eq!(calls_to(&log, FAIL_PROCEDURE), vec![vec![w.t, reason]]);
        }

        // Moving to the target: unreachable, it stops without failing.
        let mut w = world();
        w.set_state(8);
        w.rig.answer(ACTOR_PATH_TEST, 1);
        let log = w.update();
        assert_eq!(calls_to(&log, ADD_UNREACHABLE_LOCATION).len(), 1);
        assert!(calls_to(&log, FAIL_PROCEDURE).is_empty());
        assert_eq!(calls_to(&log, STOP_MOVING_ACTOR), vec![vec![w.attacker]]);
        assert_eq!(w.state(), 1);
    }

    #[test]
    fn update_moving_stops_when_close_or_seen() {
        let mut w = world();
        w.set_state(3);
        w.set_distance(50.0);
        w.rig.e.mem.set_u8(w.t + 0x33, 1);
        let log = w.update();
        assert_eq!(calls_to(&log, STOP_MOVING_ACTOR), vec![vec![w.attacker]]);
        assert_eq!(w.state(), 1);

        let mut w = world();
        w.set_state(5);
        w.rig.answer(STATE_BYTE_71, 1);
        let log = w.update();
        assert_eq!(calls_to(&log, STOP_MOVING_ACTOR).len(), 1);

        // Seen but still beyond the absolute range: keeps moving.
        let mut w = world();
        w.set_state(8);
        w.set_distance(2000.0);
        w.rig.answer(STATE_BYTE_71, 1);
        let log = w.update();
        assert!(calls_to(&log, STOP_MOVING_ACTOR).is_empty());
        assert_eq!(w.state(), 8);

        // Arrived after the ranges changed: a hundredth of a second.
        let mut w = world();
        let t = w.t;
        w.set_state(5);
        w.rig
            .e
            .register_double(COMPUTE_OPTIMAL_WEAPON_RANGES, |e, a| {
                e.mem.set_f32(a[2], 120.0);
                Ret::default()
            });
        w.rig.answer(IS_PATHING_COMPLETE, 1);
        let log = w.update();
        assert_eq!(
            calls_to(&log, TIMER_START),
            vec![vec![t + 0x20, bits(0.01)]]
        );
    }

    #[test]
    fn update_moving_checks_the_optimal_path() {
        // The goal fails the location check: -1 and a forced stop.
        let mut w = world();
        let t = w.t;
        w.set_state(4);
        w.set_distance(150.0);
        w.rig.e.mem.set_u8(t + 0x33, 0);
        w.rig.answer(GET_CURRENT_PATHFINDING_GOAL, 1);
        w.rig.answer(WORLD_LOCATION_WORD_C, 0x4400);
        w.rig.answer_float(TIME_NOW, 20.0);
        let log = w.update();
        let goal = calls_to(&log, GET_CURRENT_PATHFINDING_GOAL)[0][1];
        assert_eq!(
            calls_to(&log, STATE_CHECK_LOCATION),
            vec![vec![w.state, goal, 0x4400, 0, 0]]
        );
        assert_eq!(w.rig.e.mem.u8(t + 0x33), 0xff);
        assert_eq!(calls_to(&log, FORCE_STOP_MOVING), vec![vec![w.attacker]]);
        assert_eq!(w.rig.e.mem.f32(t + 0x60), 20.0);
        assert_eq!(w.state(), 1);

        // No line of sight from the goal: -2.
        let mut w = world();
        let t = w.t;
        w.set_state(3);
        w.set_distance(150.0);
        w.rig.e.mem.set_u8(t + 0x33, 0);
        w.rig.answer(GET_CURRENT_PATHFINDING_GOAL, 1);
        w.rig.answer(STATE_CHECK_LOCATION, 1);
        let log = w.update();
        let goal = calls_to(&log, GET_CURRENT_PATHFINDING_GOAL)[0][1];
        assert_eq!(
            calls_to(&log, CALCULATE_LOS),
            vec![vec![w.attacker, w.target, goal, w.position, 0, u32::MAX]]
        );
        assert_eq!(w.rig.e.mem.u8(t + 0x33), 0xfe);

        // Both fine and the target moved more than 256: plans again.
        let mut w = world();
        let t = w.t;
        w.set_state(3);
        w.rig.e.mem.set_u8(t + 0x33, 0);
        w.rig.answer(GET_CURRENT_PATHFINDING_GOAL, 1);
        w.rig.answer(STATE_CHECK_LOCATION, 1);
        w.rig.answer(CALCULATE_LOS, 1);
        w.rig
            .queue_floats(WORLD_LOCATION_DISTANCE_SQUARED, &[22500.0, 70000.0]);
        w.set_elapsed(t + 0x60, 10.0);
        w.rig.answer(STATE_FIND_OPTIMAL_LOCATION, 1);
        w.rig.answer(CHECK_MOVEMENT, 1);
        w.rig.answer(SET_PATHFINDING_GOAL, 1);
        let log = w.update();
        let distances = calls_to(&log, WORLD_LOCATION_DISTANCE_SQUARED);
        let target_location = calls_to(&log, GET_WORLD_LOCATION)[0][1];
        assert_eq!(distances[1], vec![target_location, t + 0x50]);
        assert_eq!(calls_to(&log, STATE_FIND_OPTIMAL_LOCATION).len(), 1);
        assert!(calls_to(&log, STOP_MOVING_ACTOR).is_empty());
        assert_eq!(w.state(), 3);
    }

    #[test]
    fn fn_009d4f30_marks_the_byte() {
        let mut w = world();
        let block = w.rig.e.mem.alloc(0x200);
        w.rig.run(0x009d_4f30, &args![block]);
        assert_eq!(w.rig.e.mem.u8(block + 0x188), 0xff);
        assert_eq!(w.rig.e.mem.u8(block + 0x187), 0);
    }

    #[test]
    fn fn_009d4f50_wants_the_target_256_away() {
        let mut w = world();
        let (result, log) = w.rig.run(0x009d_4f50, &args![w.t]);
        assert!(!result.bool());
        assert!(calls_to(&log, CONTROLLER_TARGET).is_empty());

        w.rig.answer(CONTROLLER_BYTE_D4, 1);
        w.rig.answer(CONTROLLER_TARGET, w.target);
        w.rig.answer(CONTROLLER_ACTOR, w.attacker);
        w.rig.answer_float(POINT_LENGTH_SQUARED, 65536.0);
        let (result, log) = w.rig.run(0x009d_4f50, &args![w.t]);
        assert!(result.bool());
        assert_eq!(calls_to(&log, CONTROLLER_BYTE_D4), vec![vec![w.controller]]);
        assert_eq!(
            slot_calls(&log, SLOT_POSITION),
            vec![vec![w.target], vec![w.attacker]]
        );
        let difference = calls_to(&log, POINT_DIFFERENCE);
        assert_eq!(difference[0][1], difference[0][0] + 0x18);
        assert_eq!(difference[0][2], difference[0][0] + 0xc);
        assert_eq!(
            calls_to(&log, POINT_LENGTH_SQUARED),
            vec![vec![difference[0][1]]]
        );
        w.rig.answer_float(POINT_LENGTH_SQUARED, 65535.0);
        assert!(!w.rig.run(0x009d_4f50, &args![w.t]).0.bool());
    }

    #[test]
    fn create_optimal_location_path_with_either_request() {
        // Within 5 seconds of a failure: nothing.
        let mut w = world();
        let t = w.t;
        w.rig.e.mem.set_u8(t + 0x33, 0);
        w.set_elapsed(t + 0x60, 4.0);
        let (result, log) = w.rig.run(0x009d_5000, &args![t, true]);
        assert!(!result.bool());
        assert_eq!(w.rig.e.mem.u8(t + 0x33), 0);
        assert!(calls_to(&log, GET_ATTACKER).is_empty());

        // The optimal-location request.
        let mut w = world();
        let t = w.t;
        w.set_elapsed(t + 0x60, 5.0);
        w.rig.e.mem.set_u8(USE_OPTIMAL_LOCATION_REQUEST, 1);
        w.rig.answer(GET_WORLD_SPACE, 0x3200);
        w.rig.answer(ACTOR_CELL_8D6F30, 0x3300);
        w.rig.answer(SET_PATHFINDING_GOAL, 1);
        let (result, log) = w.rig.run(0x009d_5000, &args![t, true]);
        assert!(result.bool());
        let request = calls_to(&log, OPTIMAL_REQUEST_CONSTRUCT)[0][0];
        assert_eq!(
            calls_to(&log, BUILD_REQUEST),
            vec![vec![
                w.attacker,
                request,
                w.position,
                0x3300,
                0x3200,
                bits(16.0),
                0
            ]]
        );
        assert_eq!(
            calls_to(&log, REQUEST_SET_MIN_RANGE),
            vec![vec![request, bits(100.0)]]
        );
        assert_eq!(
            calls_to(&log, REQUEST_SET_MAX_RANGE),
            vec![vec![request, bits(200.0)]]
        );
        assert_eq!(
            calls_to(&log, REQUEST_SET_HALF_MIN_RANGE),
            vec![vec![request, bits(50.0)]]
        );
        assert_eq!(
            calls_to(&log, REQUEST_SET_ABSOLUTE_RANGE),
            vec![vec![request, bits(1000.0)]]
        );
        assert_eq!(w.rig.e.mem.u32(t + 0x50), w.target);
        assert_eq!(w.rig.e.mem.u32(t + 0x5c), w.target + 3);
        assert_eq!(calls_to(&log, SET_MOVE_MODE), vec![vec![t, 3]]);
        assert_eq!(w.rig.e.mem.u8(t + 0x33), 0);
        assert_eq!(
            calls_to(&log, OPTIMAL_REQUEST_DESTRUCT),
            vec![vec![request]]
        );

        // Its goal refused.
        w.rig.answer(SET_PATHFINDING_GOAL, 0);
        let (result, log) = w.rig.run(0x009d_5000, &args![t, false]);
        assert!(!result.bool());
        assert_eq!(w.rig.e.mem.u8(t + 0x33), 1);
        assert_eq!(calls_to(&log, OPTIMAL_REQUEST_DESTRUCT).len(), 1);
        assert!(calls_to(&log, SET_MOVE_MODE).is_empty());

        // The combat state's optimal location.
        let mut w = world();
        let t = w.t;
        w.set_elapsed(t + 0x60, 6.0);
        w.rig
            .e
            .register_double(STATE_FIND_OPTIMAL_LOCATION, |e, a| {
                for i in 0..4 {
                    e.mem.set_u32(a[5] + i * 4, 0x900 + i);
                }
                ret(1)
            });
        w.rig.answer(CHECK_MOVEMENT, 1);
        w.rig.answer(DODGE_LOCATION_POINT, 0x4500);
        w.rig.answer(SET_PATHFINDING_GOAL, 1);
        let (result, log) = w.rig.run(0x009d_5000, &args![t, false]);
        assert!(result.bool());
        let location = calls_to(&log, DODGE_LOCATION_CONSTRUCT)[0][0];
        assert_eq!(
            calls_to(&log, CHECK_MOVEMENT),
            vec![vec![w.state, location, 1, 0]]
        );
        let request = calls_to(&log, PATHING_REQUEST_CONSTRUCT)[0][0];
        assert_eq!(
            calls_to(&log, BUILD_REQUEST)[0][..3],
            [w.attacker, request, 0x4500]
        );
        assert_eq!(w.rig.e.mem.u32(t + 0x40), 0x900);
        assert_eq!(w.rig.e.mem.u32(t + 0x4c), 0x903);
        assert_eq!(w.rig.e.mem.u32(t + 0x50), w.target);
        assert_eq!(calls_to(&log, SET_MOVE_MODE), vec![vec![t, 1]]);
        assert_eq!(
            calls_to(&log, PATHING_REQUEST_DESTRUCT),
            vec![vec![request]]
        );
        assert_eq!(w.rig.e.mem.u8(t + 0x33), 1);

        // No optimal location.
        let mut w = world();
        let t = w.t;
        w.set_elapsed(t + 0x60, 6.0);
        let (result, log) = w.rig.run(0x009d_5000, &args![t, false]);
        assert!(!result.bool());
        assert!(calls_to(&log, CHECK_MOVEMENT).is_empty());
    }

    #[test]
    fn display_debug_text_prints_the_state() {
        let mut w = world();
        let t = w.t;
        let text = w.rig.object(8);
        w.set_elapsed(t + 0x60, 9.0);
        let (_, log) = w.rig.run(0x009d_5410, &args![t, text, 5u32, 6u32]);
        assert_eq!(
            slot_calls(&log, SLOT_PRINT),
            vec![vec![text, 5, 6, TEXT_INITIALIZING]]
        );

        // Moving to the optimal location: the range and the target's move.
        w.set_state(3);
        w.rig.answer_float(WORLD_LOCATION_DISTANCE, 40.0);
        let (_, log) = w.rig.run(0x009d_5410, &args![t, text, 5u32, 6u32]);
        let mut range = vec![text, 5, 6, TEXT_RANGE];
        range.extend(args![100.0f64, 200.0f64]);
        let mut moved = vec![text, 5, 6, TEXT_TARGET_MOVEMENT];
        moved.extend(args![40.0f64, 256.0f64]);
        assert_eq!(
            slot_calls(&log, SLOT_PRINT),
            vec![vec![text, 5, 6, TEXT_MOVING_OPTIMAL_CLOSE], range, moved]
        );
        let location = calls_to(&log, GET_WORLD_LOCATION)[0][1];
        assert_eq!(
            calls_to(&log, WORLD_LOCATION_DISTANCE),
            vec![vec![location, t + 0x50]]
        );

        // Holding: its timer.
        w.set_state(1);
        w.set_elapsed(t + 0x20, 1.5);
        w.rig.answer_float(TIMER_DURATION, 3.0);
        let (_, log) = w.rig.run(0x009d_5410, &args![t, text, 5u32, 6u32]);
        let mut timer = vec![text, 5, 6, TEXT_HOLDING_TIMER];
        timer.extend(args![1.5f64, 3.0f64]);
        assert_eq!(
            slot_calls(&log, SLOT_PRINT),
            vec![vec![text, 5, 6, TEXT_HOLDING], timer]
        );

        // Dodging left/right having seen the target: the lost-LOS timer;
        // a failed optimal path 2 seconds ago: its reason.
        w.set_state(7);
        w.rig.e.mem.set_u8(t + 0x30, 1);
        w.rig.answer(TIMER_RUNNING, 1);
        w.set_elapsed(t + 0x28, 0.25);
        w.set_elapsed(t + 0x60, 2.0);
        w.rig.e.mem.set_u8(t + 0x33, 0xfe);
        w.rig.answer_float(TIMESTAMP_VALUE_FA90, 2.0);
        let (_, log) = w.rig.run(0x009d_5410, &args![t, text, 5u32, 6u32]);
        let mut lost = vec![text, 5, 6, TEXT_LOST_LOS_TIMER];
        lost.extend(args![0.25f64, 3.0f64]);
        let mut failed = vec![text, 5, 6, TEXT_OPTIMAL_LOS_FAILED];
        failed.extend(args![2.0f64, 5.0f64]);
        assert_eq!(
            slot_calls(&log, SLOT_PRINT),
            vec![vec![text, 5, 6, TEXT_DODGING_LEFT_RIGHT], lost, failed]
        );
        assert_eq!(calls_to(&log, TIMESTAMP_VALUE_FA90), vec![vec![t + 0x60]]);
        w.set_state(8);
        w.rig.e.mem.set_u8(t + 0x33, 0xff);
        let (_, log) = w.rig.run(0x009d_5410, &args![t, text, 5u32, 6u32]);
        assert_eq!(
            slot_calls(&log, SLOT_PRINT)[1][3],
            TEXT_OPTIMAL_MOVEMENT_FAILED
        );
        w.rig.e.mem.set_u8(t + 0x33, 0);
        let (_, log) = w.rig.run(0x009d_5410, &args![t, text, 5u32, 6u32]);
        assert_eq!(
            slot_calls(&log, SLOT_PRINT)[1][3],
            TEXT_OPTIMAL_BUILD_FAILED
        );
    }

    #[test]
    fn save_game_writes_every_member() {
        let mut w = world();
        let t = w.t;
        let (_, log) = w.rig.run(0x009d_5880, &args![t, 0x40u32]);
        assert_eq!(calls_to(&log, BASE_SAVE_GAME), vec![vec![t, 0x40]]);
        let saved: Vec<_> = calls_to(&log, SAVE_BYTES)
            .iter()
            .map(|c| (c[1] - t, c[2]))
            .collect();
        assert_eq!(
            saved,
            vec![
                (0x10, 4),
                (0x14, 4),
                (0x18, 4),
                (0x1c, 4),
                (0x30, 1),
                (0x31, 1),
                (0x32, 1),
                (0x34, 0xc),
                (0x33, 1)
            ]
        );
        assert_eq!(
            calls_to(&log, TIMER_SAVE),
            vec![vec![t + 0x20, 0x40], vec![t + 0x28, 0x40]]
        );
        assert_eq!(
            calls_to(&log, WORLD_LOCATION_SAVE),
            vec![vec![t + 0x40, 0x40], vec![t + 0x50, 0x40]]
        );
        assert_eq!(calls_to(&log, TIMESTAMP_SAVE), vec![vec![t + 0x60, 0x40]]);
    }

    #[test]
    fn load_game_follows_the_save_version() {
        let mut w = world();
        let t = w.t;
        let buffer = w.rig.object(8);
        w.rig.slot(0, 0xe);
        let (_, log) = w.rig.run(0x009d_5990, &args![t, buffer]);
        assert_eq!(calls_to(&log, BASE_LOAD_GAME), vec![vec![t, buffer]]);
        let loaded: Vec<_> = calls_to(&log, LOAD_BYTES)
            .iter()
            .map(|c| (c[1] - t, c[2]))
            .collect();
        assert_eq!(
            loaded,
            vec![
                (0x10, 4),
                (0x14, 4),
                (0x18, 4),
                (0x1c, 4),
                (0x30, 1),
                (0x31, 1),
                (0x32, 1),
                (0x34, 0xc),
                (0x33, 1)
            ]
        );
        assert_eq!(
            calls_to(&log, WORLD_LOCATION_LOAD),
            vec![vec![t + 0x40, buffer], vec![t + 0x50, buffer]]
        );
        assert_eq!(calls_to(&log, TIMESTAMP_LOAD), vec![vec![t + 0x60, buffer]]);

        // Before version 14: an absolute range of 10000.
        w.rig.slot(0, 0xd);
        let (_, log) = w.rig.run(0x009d_5990, &args![t, buffer]);
        assert_eq!(calls_to(&log, LOAD_BYTES).len(), 8);
        assert_eq!(w.rig.e.mem.f32(t + 0x1c), 10000.0);
        assert_eq!(calls_to(&log, TIMER_LOAD).len(), 2);

        // Before version 10: no optimal-path check or stamp.
        w.rig.slot(0, 9);
        let (_, log) = w.rig.run(0x009d_5990, &args![t, buffer]);
        assert_eq!(calls_to(&log, LOAD_BYTES).len(), 7);
        assert!(calls_to(&log, TIMESTAMP_LOAD).is_empty());
    }
}
