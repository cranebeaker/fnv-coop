//! `fallout/ai/movement/detailedactorpathhandler.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The detailed path handler moves an actor along a `PathingSolution`
//! node by node (tether point, avoidance of other actors, doors, rotation
//! in place). Its vtable is `010923ec`; the slots were read from the exe
//! and match the order of the Xbox PDB's `ActorPathHandler` slots:
//! 0x00 destructor `009e0160`, 0x04 `Init` `009e0470`, 0x08
//! `InitRotationOnly` `009e0930`, 0x0C `Update` `009e0a00`, 0x10
//! `NoPathUpdate` `009e16e0`, 0x14 `UpdateWhilePlayerWaits` `009e1d50`,
//! 0x18 `GetDistanceTraveled` `009e2830`, 0x1C `GetPercentTraveled`
//! `009e28a0`, 0x20 `GetNextNodeLocation` `009e3a70`, 0x24
//! `GetNodeLocation` `009e3b60`, 0x28 `GetDistanceRemaining` `009e2860`,
//! 0x2C `ComputeApproximateLocation` `009e2900`, 0x30
//! `GetCurrentNodeIndex` `0070ec90`, 0x34 `IsDetailedPathHandler`
//! `008d0360`, 0x38 `SetWaitForUpdatedPath` `009e6ba0`, 0x3C
//! `IsWaitingForAvoidance` `009e2a80`, 0x40 `IsMovementBlocked` `009e3be0`,
//! 0x44 `ComputePathPassesThroughPlayersArea` `009e3110`, 0x48 `005f5f80`,
//! 0x4C `GetMoveDelta` `009e32f0`, 0x50 `GetNextTetherPoint` `009e3990`,
//! 0x54 `ForceMoveMode` `009e3360`, 0x58 `HandlePreviousPathTransition`
//! `009e3380`, 0x5C `HandleDeletedReference` `009e8690`, 0x60
//! `PrintCurrentStatus` `009e8b50`, 0x70 `ClearMovement` `009e39c0`, 0x74
//! `WarpToNextNode` `009e8800`, 0x78 `SaveGame` `009e8f80`, 0x7C `LoadGame`
//! `009e9390`, 0x80 `InitLoadGame` `009e9760`.
//!
//! Actor virtual slots used here (PC offsets; read on `Character`'s vtable
//! `01086a6c`): 0x1F4 `Actor::GetLocationOnReference` (`008ae4c0`, returns
//! a pointer to the location), 0x2BC `Actor::GetHeading` (`008bd7b0`,
//! float in ST0, one word), 0x2C4 sets the heading (`Character`: `008a6a00`,
//! which calls `MobileObject::SetHeading`). Other slots are described where
//! they are used, by what the code does with them.
//!
//! x87: the code computes in extended precision and stores `float`s; the
//! translations compute in `f64` and round at each `float` store. A float
//! that the code only moves through the FPU (`FLD`/`FSTP`) is passed through
//! [`x87_quiet`], which quiets a signalling NaN as the FPU load does.
//!
//! Every function whose frame registers a C++ exception handler (`FS:[0]`)
//! or calls the `/GS` cookie check (`00ec408c`) is marked incomplete: the
//! unwinding is not translated.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{BSSimpleArray, BSSimpleList, NiPoint3, NiTArray};
use crate::units::fallout_ai::actormover::ActorMover;

layout! {
    /// `DetailedActorPathHandler` (Xbox PDB), 0xE4 bytes on the PC (the
    /// operator new in `ActorMover` asks for 0xE4); the Xbox build's debug
    /// fields after +0xE2 (`spAvoidanceDebugGeometry`, ...) are not there.
    /// Offsets checked against the constructors.
    pub struct DetailedActorPathHandler: 0xE4 {
        /// `pActor` (Xbox PDB, `ActorPathHandler`): `Actor*`.
        0x04 pActor: Ptr,
        /// `pActorMover` (Xbox PDB, `ActorPathHandler`): `ActorMover*`.
        0x08 pActorMover: Ptr,
        /// `spPath` (Xbox PDB): `NiPointer<PathingSolution>`.
        0x0C spPath: Ptr,
        /// `spRequest` (Xbox PDB): `NiPointer<PathingRequest>`.
        0x10 spRequest: Ptr,
        /// `fTetherMinMove` (Xbox PDB).
        0x14 fTetherMinMove: f32,
        /// `fTetherDistFromActor` (Xbox PDB).
        0x18 fTetherDistFromActor: f32,
        /// `MoveDelta` (Xbox PDB).
        0x1C MoveDelta: Inline<NiPoint3>,
        /// `BlockedTargetPoint` (Xbox PDB).
        0x28 BlockedTargetPoint: Inline<NiPoint3>,
        /// `TetherTargetPoint` (Xbox PDB).
        0x34 TetherTargetPoint: Inline<NiPoint3>,
        /// `LastLocation` (Xbox PDB).
        0x40 LastLocation: Inline<NiPoint3>,
        /// `LastTetherPoint` (Xbox PDB).
        0x4C LastTetherPoint: Inline<NiPoint3>,
        /// `ActorAvoidList` (Xbox PDB): `BSSimpleList<Actor *>`.
        0x58 ActorAvoidList: Inline<BSSimpleList>,
        /// `MoveMode` (Xbox PDB).
        0x60 MoveMode: u32,
        /// `iCurrentNodeIndex` (Xbox PDB).
        0x64 iCurrentNodeIndex: u32,
        /// `fRotateGoal` (Xbox PDB).
        0x68 fRotateGoal: f32,
        /// `fPathLength` (Xbox PDB).
        0x6C fPathLength: f32,
        /// `fCurrentSplineT` (Xbox PDB).
        0x70 fCurrentSplineT: f32,
        /// `fActorAvoidanceTimer` (Xbox PDB).
        0x74 fActorAvoidanceTimer: f32,
        /// `fAvoidanceDetectionValue` (Xbox PDB).
        0x78 fAvoidanceDetectionValue: f32,
        /// `iUnblockDirection` (Xbox PDB).
        0x7C iUnblockDirection: i32,
        /// `fAlternateFacingAngle` (Xbox PDB).
        0x80 fAlternateFacingAngle: f32,
        /// `fBlockedStartAngle` (Xbox PDB).
        0x84 fBlockedStartAngle: f32,
        /// `fBlockTime` (Xbox PDB).
        0x88 fBlockTime: f32,
        /// `fTimeInAnim` (Xbox PDB).
        0x8C fTimeInAnim: f32,
        /// `fRunWalkTime` (Xbox PDB).
        0x90 fRunWalkTime: f32,
        /// `fMinRotateTime` (Xbox PDB).
        0x94 fMinRotateTime: f32,
        /// `fWaitTime` (Xbox PDB).
        0x98 fWaitTime: f32,
        /// `fLastFrameMinMoveDelta` (Xbox PDB).
        0x9C fLastFrameMinMoveDelta: f32,
        /// `LastFrameMinMoveDelta` (Xbox PDB).
        0xA0 LastFrameMinMoveDelta: Inline<NiPoint3>,
        /// `fGoalRadiusSqr` (Xbox PDB).
        0xAC fGoalRadiusSqr: f32,
        /// `fFallTime` (Xbox PDB).
        0xB0 fFallTime: f32,
        /// `fDistanceTraveled` (Xbox PDB).
        0xB4 fDistanceTraveled: f32,
        /// `fIgnoreLookAtPointTimer` (Xbox PDB).
        0xB8 fIgnoreLookAtPointTimer: f32,
        /// `fLastRotationApplied` (Xbox PDB).
        0xBC fLastRotationApplied: f32,
        /// `bIgnoreInitialRotation` (Xbox PDB).
        0xC0 bIgnoreInitialRotation: bool,
        /// `bIgnoreInitialRotationKeepActorHeading` (Xbox PDB).
        0xC1 bIgnoreInitialRotationKeepActorHeading: bool,
        /// `fIgnoreInitialRotationTetherParam` (Xbox PDB).
        0xC4 fIgnoreInitialRotationTetherParam: f32,
        /// `eState` (Xbox PDB, `PATH_HANDLER_STATE`).
        0xC8 eState: u32,
        /// `eBlendRunning` (Xbox PDB, `PATH_RUNNING_BLEND`).
        0xCC eBlendRunning: u32,
        /// `eAvoidState` (Xbox PDB, `PATH_AVOID_STATE`).
        0xD0 eAvoidState: u32,
        /// `eDoorState` (Xbox PDB, `PATH_HANDLER_DOOR_STATE`).
        0xD4 eDoorState: u32,
        /// `pCurrentDoor` (Xbox PDB): `TESObjectREFR*`.
        0xD8 pCurrentDoor: Ptr,
        /// `bRotateMode` (Xbox PDB).
        0xDC bRotateMode: bool,
        /// `bRotateCCW` (Xbox PDB).
        0xDD bRotateCCW: bool,
        /// `bMovementBlocked` (Xbox PDB).
        0xDE bMovementBlocked: bool,
        /// `bEndUpdatePath` (Xbox PDB).
        0xDF bEndUpdatePath: bool,
        /// `bWaitingForUpdate` (Xbox PDB).
        0xE0 bWaitingForUpdate: bool,
        /// `bIgnoreLookAtPoint` (Xbox PDB).
        0xE1 bIgnoreLookAtPoint: bool,
        /// `bDeltaComputedThisFrame` (Xbox PDB).
        0xE2 bDeltaComputedThisFrame: bool,
    }
}

/// Short name for the handler in this file.
type H = DetailedActorPathHandler;

/// The handler's vtable.
pub(crate) const VTABLE: u32 = 0x0109_23ec;

// Data the code reads from the exe.
/// A global `NiPoint3` the code copies word by word as the "no point" value
/// (three words at `011f426c`; its name is not confirmed).
pub(crate) const NO_POINT: u32 = 0x011f_426c;
/// `float` `FLT_MAX` (`01016970`).
const FLOAT_MAX: u32 = 0x0101_6970;
/// `float` `-1.0` (`01012054`).
const MINUS_ONE_FLOAT: u32 = 0x0101_2054;
/// `float` 10.0 (`01017b78`), the constructors' `fTetherMinMove`.
const TETHER_MIN_MOVE: u32 = 0x0101_7b78;
/// `float` 20.0 (`01017868`), the constructors' `fTetherDistFromActor`.
const TETHER_DIST_FROM_ACTOR: u32 = 0x0101_7868;
/// `double` 0.0 (`01012060`).
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// `double` 1.0 (`01012070`).
const ONE_DOUBLE: u32 = 0x0101_2070;
/// `double` -1.0 (`0101a6b0`).
const MINUS_ONE_DOUBLE: u32 = 0x0101_a6b0;
/// `double` 0.95 (`0107d1b8`).
const NINETY_FIVE_HUNDREDTHS: u32 = 0x0107_d1b8;
/// `float` 0.1 (`0101e2bc`), the smallest goal radius `Init` uses.
const ONE_TENTH_FLOAT: u32 = 0x0101_e2bc;
/// `float` (`0108b8bc`): the goal radius `Init` uses when the path's byte
/// at +0x40 is set.
const GOAL_RADIUS_PATH_FLAG: u32 = 0x0108_b8bc;
/// `float` (`01092470`): the goal radius `Init` uses when the path's
/// destination flags have both low bits set.
const GOAL_RADIUS_BOTH_FLAGS: u32 = 0x0109_2470;
/// `double` 0.4 (`010290c0`): fall time after which `Update` stops.
const FALL_TIME_STOP: u32 = 0x0102_90c0;
/// `double` 0.65 (`01074b98`): fall time after which `Update` returns 4.
const FALL_TIME_LANDED: u32 = 0x0107_4b98;
/// `double` 0.25 (`010290b0`).
const QUARTER: u32 = 0x0102_90b0;
/// `double` 100.0 (`01017a40`).
const HUNDRED: u32 = 0x0101_7a40;
/// `double` pi / 180 (`01023128`).
const DEGREES_TO_RADIANS: u32 = 0x0102_3128;
/// `float` (`010923dc`, about one degree in radians): the turn below which
/// `NoPathUpdate` stops turning in place.
const TURN_THRESHOLD: u32 = 0x0109_23dc;
/// `double` (`01092478`, about 88 degrees in radians).
const STEEP_PITCH: u32 = 0x0109_2478;

// Settings (`Setting` objects; their names are not confirmed here). The
// code reads their value through `SETTING_VALUE` / `SETTING_BOOL`.
/// A float setting `Update` copies into `fActorAvoidanceTimer`.
const SETTING_AVOIDANCE_TIMER: u32 = 0x011f_2b2c;
/// A bool setting that turns actor avoidance on in `Update`.
const SETTING_AVOIDANCE_ENABLED: u32 = 0x011f_2bb8;
/// Turn-rate factors in `NoPathUpdate`: slot 0x21C false and the actor's
/// byte +0x104 set / clear, then slot 0x21C true and the byte set / clear.
const SETTING_TURN_RATE_A: u32 = 0x011f_2ac4;
const SETTING_TURN_RATE_B: u32 = 0x011f_2ad8;
const SETTING_TURN_RATE_C: u32 = 0x011f_2b10;
const SETTING_TURN_RATE_D: u32 = 0x011f_2c10;
/// The minimum time `NoPathUpdate` turns once it starts turning.
const SETTING_MIN_ROTATE_TIME: u32 = 0x011c_d294;

// Callees outside this unit (bodies read in Ghidra; names from the engine
// map where it has one).
/// `NiPointer<T>` get (`00559450`): returns the pointer held at `this`.
const NIPOINTER_GET: u32 = 0x0055_9450;
/// `NiPointer<T>::operator=` (`0066b0d0`): releases the old object, holds
/// the new one.
const NIPOINTER_ASSIGN: u32 = 0x0066_b0d0;
/// `NiPointer<T>` from a raw pointer (`00633c90`).
const NIPOINTER_FROM_RAW: u32 = 0x0063_3c90;
/// `NiPointer<T>` destructor (`0045cec0`).
const NIPOINTER_RELEASE: u32 = 0x0045_cec0;
/// The empty default constructor of a point or matrix (`006815c0`,
/// returns `this`).
const DEFAULT_CONSTRUCT: u32 = 0x0068_15c0;
/// `NiPoint3::NiPoint3(x, y, z)` (`00416870`).
const POINT_CONSTRUCT: u32 = 0x0041_6870;
/// `BSSimpleList` constructor (`0096a2d0`: zeroes item and next).
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// `BSSimpleList` clear (`00470470`).
const LIST_CLEAR: u32 = 0x0047_0470;
/// `BSSimpleList` destructor (`0046ffb0`, calls the clear).
const LIST_DESTRUCT: u32 = 0x0046_ffb0;
/// `ActorPathHandler` constructor (`009dfd70`: actor, mover).
const BASE_CONSTRUCT: u32 = 0x009d_fd70;
/// `ActorPathHandler` constructor (`009dfe40`: actor, mover, path pointer
/// address, request).
const BASE_CONSTRUCT_WITH_PATH: u32 = 0x009d_fe40;
/// `ActorPathHandler` destructor body (`009dfed0`).
const BASE_DESTRUCT: u32 = 0x009d_fed0;
/// Operator delete (`00401030`, cdecl).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// Operator new (`00aa13e0`, cdecl: size).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// `*this == value` for a pointer (`00822510`).
const POINTER_EQUALS: u32 = 0x0082_2510;
/// `PathingSolution`: the dword at +8 (`008b6800`), the node count.
const PATH_NODE_COUNT: u32 = 0x008b_6800;
/// `PathingSolution`: the byte at +0x40 (`006b05d0`).
const PATH_BYTE_40: u32 = 0x006b_05d0;
/// `PathingSolution`: a pointer the code reads flags from (`006b05a0`).
const PATH_END_NODE: u32 = 0x006b_05a0;
/// Length of a path (`006e78d0`, float in ST0).
const PATH_LENGTH: u32 = 0x006e_78d0;
/// `(*this & 3) == 3` (`006e2330`).
const FLAGS_BOTH_LOW: u32 = 0x006e_2330;
/// `(*this & 4) == 4` (`006ca0c0`).
const FLAGS_BIT_2: u32 = 0x006c_a0c0;
/// The float at +0x74 (`009b0bd0`; `PathingRequest::fTargetRadius`).
const FLOAT_AT_74: u32 = 0x009b_0bd0;
/// `max(a, b)` of two floats (`00404010`, cdecl, float in ST0).
const FLOAT_MAX_OF: u32 = 0x0040_4010;
/// The byte at +0x104 (`00493bb0`).
const BYTE_AT_104: u32 = 0x0049_3bb0;
/// `ActorMover`: the byte at +0x73 (`008b3d10`, `bFaceTargetPoint`).
const MOVER_FACE_TARGET_POINT: u32 = 0x008b_3d10;
/// `ActorMover`: copies `LookAtPoint` into the out point when
/// `bFaceTargetPoint` is set (`009de270`; returns that byte as a bool).
const MOVER_GET_LOOK_AT_POINT: u32 = 0x009d_e270;
/// `ActorMover`: `uiPreferredMoveMode & 0xff00` (`009dd130`).
const MOVER_MOVE_MODE_BASE: u32 = 0x009d_d130;
/// `ActorMover`: the byte at +0x70 (`0097f2f0`, `bPathingFailed`).
const MOVER_BYTE_70: u32 = 0x0097_f2f0;
/// `ActorMover`: the byte at +0x71 (`0097fb20`, `bForceMoveMode`).
const MOVER_BYTE_71: u32 = 0x0097_fb20;
/// `ActorMover`: hands it a pathing request (`009db090`, one word).
const MOVER_REQUEST_PATH: u32 = 0x009d_b090;
/// An empty function (`00483710`) the code calls on the mover.
const EMPTY_FUNCTION: u32 = 0x0048_3710;
/// `point -= other` (`004578c0`).
const POINT_SUBTRACT_ASSIGN: u32 = 0x0045_78c0;
/// `point - other` into a result (`00439ef0`: receiver, result, right).
const POINT_SUBTRACT: u32 = 0x0043_9ef0;
/// `point *= factor` (`00439180`).
const POINT_SCALE: u32 = 0x0043_9180;
/// Length of a point (`00457990`, float in ST0).
const POINT_LENGTH: u32 = 0x0045_7990;
/// Length of a point's x and y (`00589850`, float in ST0).
const POINT_LENGTH_XY: u32 = 0x0058_9850;
/// `x*x + y*y` of a point (`00595c80`, float in ST0).
const POINT_LENGTH_SQUARED_XY: u32 = 0x0059_5c80;
/// Normalizes a point (`004a0c10`).
const POINT_NORMALIZE: u32 = 0x004a_0c10;
/// `NiPoint3::Dot` (`004b6190`).
const POINT_DOT: u32 = 0x004b_6190;
/// Matrix times point into a result (`004b4500`: matrix, result, point).
const MATRIX_TIMES_POINT: u32 = 0x004b_4500;
/// A float helper taking one float (`00408840`, cdecl, float in ST0); it
/// calls `00408860`. The code uses it as an absolute value.
const FLOAT_ABS: u32 = 0x0040_8840;
/// `GetZAngleFromVector` (`004b13c0`, cdecl).
const Z_ANGLE_FROM_VECTOR: u32 = 0x004b_13c0;
/// `ClampAngle` (`004b1480`, cdecl).
const CLAMP_ANGLE: u32 = 0x004b_1480;
/// Signed shortest turn from one angle to another (`004b15e0`, cdecl: from,
/// to, out direction as 1.0 / -1.0 / 0.0; the size in ST0).
const ANGLE_DIFFERENCE: u32 = 0x004b_15e0;
/// Sine and cosine from tables (`00639730`, cdecl: angle, out, out).
const SIN_COS: u32 = 0x0063_9730;
/// `Actor::CanMove` (`008843a0`).
const ACTOR_CAN_MOVE: u32 = 0x0088_43a0;
/// `MobileObject::GetCharController` (`009306d0`).
const ACTOR_GET_CHAR_CONTROLLER: u32 = 0x0093_06d0;
/// `PlayerCharacter::IsSleepingorResting` (`0094df60`).
const PLAYER_IS_SLEEPING: u32 = 0x0094_df60;
/// The player (`PlayerCharacter*` at `011dea3c`).
const PLAYER: u32 = 0x011d_ea3c;
/// Character controller state (`005c0880`).
const CONTROLLER_STATE: u32 = 0x005c_0880;
/// Character controller: the float at +0x548 (`0088b0f0`).
const CONTROLLER_FALL_TIME: u32 = 0x0088_b0f0;
/// A setting's value address (`00403e20`, `00408d60`: `this + 4`).
const SETTING_VALUE: u32 = 0x0040_3e20;
const SETTING_BOOL: u32 = 0x0040_8d60;
/// `PathingRequest::PathingRequest` (`006e2420`).
const REQUEST_CONSTRUCT: u32 = 0x006e_2420;
/// `PathingRequest::CopyTo` (`006e2750`).
const REQUEST_COPY_TO: u32 = 0x006e_2750;
/// `PathingLocation` assignment (`006d1bc0`).
const LOCATION_ASSIGN: u32 = 0x006d_1bc0;
/// `PathingSolution`: a pointer to its last node (`009ddb30`).
const PATH_LAST_NODE: u32 = 0x009d_db30;
/// `this + 4` (`00717e50`).
const PLUS_4: u32 = 0x0071_7e50;
/// `this + 0x34` (`006a9540`; `PathingRequest::Destination`).
const PLUS_34: u32 = 0x006a_9540;
/// `(byte +0x26 & 2) != 0` (`006a9520`; a `PathingLocation` flag).
const LOCATION_FLAG_2: u32 = 0x006a_9520;
/// Copies the three words at +4 into a result (`005c3420`).
const COPY_POINT_AT_4: u32 = 0x005c_3420;
/// `TESActorBase::IsImmobile` (`005f0c80`) and the actor's base form
/// (`004181e0`).
const ACTOR_BASE_IS_IMMOBILE: u32 = 0x005f_0c80;
const ACTOR_BASE_FORM: u32 = 0x0041_81e0;
/// The actor's byte at +0x1B0 (`0087eeb0`).
const ACTOR_BYTE_1B0: u32 = 0x0087_eeb0;
/// `MobileObject::IsInDialoguewithPlayer` (`00933840`).
const ACTOR_IN_DIALOGUE_WITH_PLAYER: u32 = 0x0093_3840;
/// `CombatUtilities::GetAttackerFiringLocation` (`009a8050`, cdecl:
/// result, actor).
const FIRING_LOCATION: u32 = 0x009a_8050;
/// `CombatUtilities::GetAngleToProjectedPoint` (`009a88e0`, cdecl: result,
/// actor, point by value).
const ANGLE_TO_PROJECTED_POINT: u32 = 0x009a_88e0;
/// `Actor::GetRadius` (`008be1b0`, float in ST0).
const ACTOR_GET_RADIUS: u32 = 0x008b_e1b0;

/// `float` pi (`0102b3c8`) and -pi (`0106b648`).
const PI_FLOAT: u32 = 0x0102_b3c8;
const MINUS_PI_FLOAT: u32 = 0x0106_b648;
/// `float` -pi/2 (`01016b78`) and pi/2 (`0101ff38`).
const ANGLE_LEFT: u32 = 0x0101_6b78;
const ANGLE_RIGHT: u32 = 0x0101_ff38;
/// `float` 2pi (`0101ff50`) and `double` 2pi (`0101ff48`).
const TWO_PI_FLOAT: u32 = 0x0101_ff50;
const TWO_PI_DOUBLE: u32 = 0x0101_ff48;
/// `double` pi/2 (`01064f50`).
const HALF_PI: u32 = 0x0106_4f50;
/// `double` 0.5 (`01011588`).
const HALF: u32 = 0x0101_1588;
/// `double` 2.0 (`01011590`).
const TWO_DOUBLE: u32 = 0x0101_1590;
/// `double` 3.0 (`01021928`).
const THREE: u32 = 0x0102_1928;
/// `double` 4.0 (`0101db80`).
const FOUR: u32 = 0x0101_db80;
/// `double` 5.0 (`01020998`): also the depth under the water surface
/// `009e3560` keeps the tether point at.
const FIVE: u32 = 0x0102_0998;
const UNDER_WATER: u32 = FIVE;
/// `double` 10.0 (`01020758`).
const TEN: u32 = 0x0102_0758;
/// `double` 45.0 (`010357e8`).
const FORTY_FIVE: u32 = 0x0103_57e8;
/// `double` 0.125 (`010924a8`).
const EIGHTH: u32 = 0x0109_24a8;
/// `double` 0.2 (`01018a90`).
const FIFTH: u32 = 0x0101_8a90;
/// `double` 0.33 (`01026998`).
const THIRD: u32 = 0x0102_6998;
/// `double` 0.1 (`0101ffa0`): the spline step of the tether.
const SPLINE_STEP: u32 = 0x0101_ffa0;
/// `double` 180/pi (`0102f248`).
const RADIANS_TO_DEGREES: u32 = 0x0102_f248;
/// `double` (`01092490`, about 15 degrees in radians).
const FIFTEEN_DEGREES: u32 = 0x0109_2490;
/// `double` 135.0 (`01092488`) and 45.0 (`010357e8`): turn sizes in degrees
/// in `009e2aa0`.
const LARGE_TURN: u32 = 0x0109_2488;
const SMALL_TURN: u32 = FORTY_FIVE;
/// `double` 1.5 (`01016ff0`): block time before an actor counts as blocked.
const BLOCK_TIME: u32 = 0x0101_6ff0;
/// `double` 128.0 (`0102e430`).
const RADIUS_LIMIT: u32 = 0x0102_e430;
/// `double` 180.0 (`0104ed58`).
const HEIGHT_LIMIT: u32 = 0x0104_ed58;
/// `double` 1e6 (`0106b798`).
const AVOID_RANGE_SQUARED: u32 = 0x0106_b798;
/// `double` -FLT_MAX (`010241b0`): "no water" from
/// `TESObjectREFR::GetRelevantWaterHeight`.
const NO_WATER_HEIGHT: u32 = 0x0102_41b0;
/// `double` (`010924a0`): the looking rate in radians per second.
const LOOKING_RATE: u32 = 0x0109_24a0;
/// Turn thresholds of `009e4800` (`double`: 2.7, 2.0, 1.6) and the move
/// speed factors it picks (`float`: 0.05, 0.1, 0.2, 0.5).
const TURN_SLOWEST: u32 = 0x0109_2498;
const TURN_SLOW: u32 = 0x0106_efa0;
const SCALE_SLOWEST: u32 = 0x0101_81bc;
const SCALE_SLOW: u32 = 0x0101_df44;
const SCALE_NEAR_END: u32 = 0x0101_6248;
/// `float` (`010313d4`, 0.025): the smallest turn slow-down factor.
const MIN_TURN_FACTOR: u32 = 0x0103_13d4;
/// `float` turn thresholds of `InitRotationMode` (`010923e4`, `010923e0`).
const TURN_THRESHOLD_EXACT: u32 = 0x0109_23e4;
const TURN_THRESHOLD_RUNNING: u32 = 0x0109_23e0;
/// `float` 1.5 (`01016088`) and (`01028358`): avoid node radius factors.
const AVOID_RADIUS_WALK: u32 = 0x0101_6088;
const AVOID_RADIUS_RUN: u32 = 0x0102_8358;
/// `float` (`010162c0`): the weight of an avoid node ahead of a moving actor.
const AVOID_NODE_WEIGHT: u32 = 0x0101_62c0;
/// `float` look-ahead factors of `009e5690` (`011a5cf4` walking,
/// `011a5cfc` running).
const AVOID_FACTOR_WALK: u32 = 0x011a_5cf4;
const AVOID_FACTOR_RUN: u32 = 0x011a_5cfc;
/// A global point `009e71b0` hands `009e5690` as the move delta
/// (`011a9478`).
const START_DELTA: u32 = 0x011a_9478;
/// The table `009e55e0` indexes (`011a3ff0`).
const ACQUIRE_TABLE: u32 = 0x011a_3ff0;
/// The avoidance candidates (see `fn_009e54a0`).
const CANDIDATES: u32 = 0x011f_2bf8;
const CANDIDATES_MAX_SIZE: u32 = 0x011f_2a9c;
const CANDIDATES_SIZE: u32 = 0x011f_2aa0;
const CANDIDATES_TIMESTAMP: u32 = 0x011f_2aa4;
/// The object whose float at +0xC `009e5690` uses as the frame time and
/// whose dword at +0x14 is the candidates' stamp (`011f6394`).
const FRAME_OBJECT: u32 = 0x011f_6394;
/// The object whose list at +0x80 holds the actors (`011e0e80`).
const ACTOR_LIST_OWNER: u32 = 0x011e_0e80;
/// `TES` (`TES*` at `011dea10`) and the calendar (`011de7b8`).
const TES: u32 = 0x011d_ea10;
const CALENDAR: u32 = 0x011d_e7b8;
/// `double` 3600.0 (`01012640`).
const SECONDS_PER_HOUR: u32 = 0x0101_2640;

// More settings (names not confirmed).
const SETTING_AVOID_CONTROLLER_LIMIT: u32 = 0x011f_2bd0;
const SETTING_AVOID_LOOK_AHEAD: u32 = 0x011f_2b98;
const SETTING_AVOID_NEAR: u32 = 0x011f_2b8c;
const SETTING_AVOID_THRESHOLD: u32 = 0x011f_2b1c;
const SETTING_AVOID_WAIT_TIME: u32 = 0x011f_2b74;
const SETTING_AVOID_MIN_TIMER: u32 = 0x011f_2bc4;
const SETTING_AVOID_REPATH_TIMER: u32 = 0x011f_2b3c;
const SETTING_AVOID_NODE_WEIGHT: u32 = 0x011f_2b5c;
const SETTING_PATH_LEAD: u32 = 0x011f_2b68;

// More callees outside this unit.
const ACTOR_BYTE_14D: u32 = 0x005a_2030;
const ACTOR_CAN_PASS: u32 = 0x008b_c7d0;
const ACTOR_COMPUTE_MOVE_DELTA: u32 = 0x008b_7100;
const ACTOR_FLAG_008B3C50: u32 = 0x008b_3c50;
/// `Actor::GetAnimGroupDuration` (`00897b50`).
const ACTOR_GET_ANIM_GROUP_DURATION: u32 = 0x0089_7b50;
/// `Actor::GetCurrentSpeed` (`008a0b10`).
const ACTOR_GET_CURRENT_SPEED: u32 = 0x008a_0b10;
/// Fills a `PathingLocation` with where the actor is going (`008b3860`).
const ACTOR_GET_DESTINATION: u32 = 0x008b_3860;
/// `Actor::GetForwardLength` (`008be420`).
const ACTOR_GET_FORWARD_LENGTH: u32 = 0x008b_e420;
/// `Actor::GetHeight` (`008853a0`).
const ACTOR_GET_HEIGHT: u32 = 0x0088_53a0;
/// `MobileObject::GetLooking` (`00931d70`) and `SetLooking` (`00931d90`).
const ACTOR_GET_LOOKING: u32 = 0x0093_1d70;
const ACTOR_SET_LOOKING: u32 = 0x0093_1d90;
/// `Actor::GetRunSpeed` (`00884eb0`) and `GetWalkSpeed` (`00884dc0`).
const ACTOR_GET_RUN_SPEED: u32 = 0x0088_4eb0;
const ACTOR_GET_WALK_SPEED: u32 = 0x0088_4dc0;
/// `(008846e0 & 0xf) != 0` (`004938e0`).
const ACTOR_IS_MOVING: u32 = 0x0049_38e0;
/// A move flags word of an actor (`008846e0`, its process's slot 0x20).
const ACTOR_MOVE_FLAGS: u32 = 0x0088_46e0;
/// A pointer to an actor's move delta (`008b3d50`).
const ACTOR_MOVE_DELTA: u32 = 0x008b_3d50;
/// Moves the actor to a point (`008ad1c0`: point, heading, cell,
/// worldspace, two flags).
const ACTOR_MOVE_TO_PATH_END: u32 = 0x008a_d1c0;
/// The dword at +0x68 (`008d8520`): the actor's process.
const ACTOR_PROCESS: u32 = 0x008d_8520;
/// The actor's state is 3 (`00437bd0`) or 5 (`00437bf0`), from `004f8960`.
const ACTOR_STATE_IS_3: u32 = 0x0043_7bd0;
const ACTOR_STATE_IS_5: u32 = 0x0043_7bf0;
const ACTOR_SLOT_ANIMATION: u32 = 0x1e4;
const ACTOR_SLOT_PROCESS: u32 = 0x1d0;
const ACTOR_SLOT_214: u32 = 0x214;
const ACTOR_SLOT_2C0: u32 = 0x2c0;
const ACTOR_SLOT_42C: u32 = 0x42c;
/// `BSSimpleArray<Actor *>`-style list: constructor (`008c1c00`),
/// destructor (`008c1cb0`), add (`007cb2e0`).
const ACTOR_LIST_CONSTRUCT: u32 = 0x008c_1c00;
const ACTOR_LIST_DESTRUCT: u32 = 0x008c_1cb0;
const ACTOR_LIST_ADD: u32 = 0x007c_b2e0;
/// `CombatUtilities::GetAngleToProjectedPoint` (`009a8af0`, cdecl:
/// result, two points by value).
const ANGLE_TO_PROJECTED_POINT_BETWEEN: u32 = 0x009a_8af0;
/// `Animation::GroupLoaded` (`00494710`) and `Animation::ShouldBeMoving`
/// (`00495be0`).
const ANIMATION_GROUP_LOADED: u32 = 0x0049_4710;
const ANIMATION_SHOULD_BE_MOVING: u32 = 0x0049_5be0;
/// `NiTArray` helpers: clear (`005e03d0`), set size (`0096ad30`), add
/// (`0096a610`), element address (`00877a30`, 4-byte elements).
const ARRAY_CLEAR: u32 = 0x005e_03d0;
const ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const ARRAY_ADD: u32 = 0x0096_a610;
const ARRAY_ELEMENT: u32 = 0x0087_7a30;
/// Element address of an array of 8-byte entries (`006950d0`).
const ARRAY_ELEMENT_8: u32 = 0x0069_50d0;
/// Index search with a compare function (`00719b20`) and the compare
/// function `009e2660` passes.
const ARRAY_FIND: u32 = 0x0071_9b20;
const ARRAY_COMPARE_009A3830: u32 = 0x009a_3830;
/// `size == 0` (`0076b610`) and the size at +8 (`0044ddc0`).
const ARRAY_IS_EMPTY: u32 = 0x0076_b610;
const ARRAY_SIZE: u32 = 0x0044_ddc0;
const DWORD_AT_8: u32 = ARRAY_SIZE;
/// The `PathingAvoidNode` array: constructor (`006a7f60`), destructor
/// (`006a7f90`), new array constructor (`006e3850`), add (`00905450`).
const AVOID_NODE_ARRAY_CONSTRUCT: u32 = 0x006a_7f60;
const AVOID_NODE_ARRAY_DESTRUCT: u32 = 0x006a_7f90;
const AVOID_NODE_ARRAY_NEW: u32 = 0x006e_3850;
const AVOID_NODE_ARRAY_ADD: u32 = 0x0090_5450;
/// A static avoid node: radius (+0x18, `004a7bd0`) and its setter
/// (`00819a50`), the float at +0x1C (`00495460`) and its setter
/// (`00804cb0`).
const AVOID_NODE_RADIUS: u32 = 0x004a_7bd0;
const AVOID_NODE_SET_RADIUS: u32 = 0x0081_9a50;
const AVOID_NODE_FLOAT_1C: u32 = 0x0049_5460;
const AVOID_NODE_SET_FLOAT_1C: u32 = 0x0080_4cb0;
/// `BSWrap` (`004e44f0`, cdecl: value pointer, low, high).
const BS_WRAP: u32 = 0x004e_44f0;
/// `Calendar::GetTimeScale` (`00867950`).
const CALENDAR_GET_TIME_SCALE: u32 = 0x0086_7950;
/// `(byte +0x24 & 1) != 0` of a cell (`00425fd0`).
const CELL_IS_INTERIOR: u32 = 0x0042_5fd0;
/// `Pathing::CheckLineOfSight` (`006d7490`, cdecl).
const CHECK_LINE_OF_SIGHT: u32 = 0x006d_7490;
/// Character controller: flags test (`00621270`), havok position
/// (`00621a40`), the float at +0x5FC (`008a3b50`) and a support object
/// (`00c6fd60`).
const CONTROLLER_FLAGS_TEST: u32 = 0x0062_1270;
const CONTROLLER_HK_POSITION: u32 = 0x0062_1a40;
const CONTROLLER_FLOAT_5FC: u32 = 0x008a_3b50;
const CONTROLLER_SUPPORT: u32 = 0x00c6_fd60;
/// Copies 16 bytes (`004a3c90`) and converts a havok vector to a point
/// (`00458620`, cdecl).
const COPY_16_BYTES: u32 = 0x004a_3c90;
const HK_VECTOR_TO_POINT: u32 = 0x0045_8620;
/// `DoorInPathFilter`'s vtable (`01092484`) and its base constructor
/// (`006cd650`).
const DOOR_IN_PATH_FILTER_VTABLE: u32 = 0x0109_2484;
const FILTER_BASE_CONSTRUCT: u32 = 0x006c_d650;
/// A door's virtual slot 0xF0.
const DOOR_SLOT_F0: u32 = 0xf0;
/// `_fabs` on a `double` (`00ec6cde`, cdecl).
const DOUBLE_ABS: u32 = 0x00ec_6cde;
/// The dword at +0x18 (`009611e0`).
const DWORD_AT_18: u32 = 0x0096_11e0;
/// `TESObjectREFR::FindReferenceFor3D` (`0056f930`, cdecl).
const FIND_REFERENCE_FOR_3D: u32 = 0x0056_f930;
/// The float at +0x68 (`0069ef80`; `PathingRequest::fActorRadius`).
const FLOAT_AT_68: u32 = 0x0069_ef80;
/// Clamps the float at a pointer (`005330e0`, cdecl: pointer, low, high).
const FLOAT_CLAMP: u32 = 0x0053_30e0;
/// A float remainder (`004b1500`, cdecl: value, divisor).
const FLOAT_MODULO: u32 = 0x004b_1500;
/// The dword at +0xC of a form (`0084e3a0`).
const FORM_DWORD_C: u32 = 0x0084_e3a0;
/// The float at +0xC (`0084d030`).
const FRAME_FLOAT_C: u32 = 0x0084_d030;
/// `_ftol2_sse` (`00ec62c0`, the value as a leading `f64`).
const FTOL: u32 = 0x00ec_62c0;
/// `BGSOpenCloseForm::GetOpenState` (`0047b250`, cdecl).
const GET_OPEN_STATE: u32 = 0x0047_b250;
/// A reference linked from a path node's +0x38 (`00568e50`).
const LINKED_REFERENCE: u32 = 0x0056_8e50;
/// `BSSimpleList` helpers: contains (`005f65d0`), count (`005ae380`),
/// empty (`008256d0`), next node (`00726070`), push front (`005ae3d0`),
/// pop front (`0063f7b0`).
const LIST_CONTAINS: u32 = 0x005f_65d0;
const LIST_COUNT: u32 = 0x005a_e380;
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
const LIST_NEXT: u32 = 0x0072_6070;
const LIST_PUSH_FRONT: u32 = 0x005a_e3d0;
const LIST_POP_FRONT: u32 = 0x0063_f7b0;
/// `PathingLocation`: copy constructor (`006dcc40`), default constructor
/// (`006dcbb0`), from a reference (`006dcd70`), from a point and a reference
/// (`006dce10`), from a point, cell and worldspace (`006dcee0`), destructor
/// (`004ff7e0`), `GetCell` (`006dd4f0`), `GetWorldspace` (`00441110`),
/// `GetNavMeshAndTriangle` (`006dd640`), `ResolveNavMeshInfo` (`006dd6f0`),
/// within a distance of another (`006deeb0`).
const LOCATION_COPY: u32 = 0x006d_cc40;
const LOCATION_DEFAULT: u32 = 0x006d_cbb0;
const LOCATION_FROM_REFERENCE: u32 = 0x006d_cd70;
const LOCATION_FROM_POINT: u32 = 0x006d_ce10;
const LOCATION_FROM_POINT_IN_CELL: u32 = 0x006d_cee0;
const LOCATION_DESTRUCT: u32 = 0x004f_f7e0;
const LOCATION_GET_CELL: u32 = 0x006d_d4f0;
const LOCATION_GET_WORLDSPACE: u32 = 0x0044_1110;
const LOCATION_GET_NAVMESH_AND_TRIANGLE: u32 = 0x006d_d640;
const LOCATION_RESOLVE_NAVMESH: u32 = 0x006d_d6f0;
const LOCATION_WITHIN: u32 = 0x006d_eeb0;
/// Looks up a form by ID (`004839c0`, cdecl).
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `NiMatrix3::MakeXRotation` (`00524ac0`).
const MATRIX_MAKE_X_ROTATION: u32 = 0x0052_4ac0;
/// The mover's virtual slot 0x1C (a move mode word).
const MOVER_SLOT_1C: u32 = 0x1c;
/// `NavMesh`: add an avoid node (`00691510`), closed door info
/// (`NavMesh::GetClosedDoorInfoForTriangle`, `00699af0`), a triangle
/// (`005582f0`), static avoid nodes (`NavMesh::GetStaticAvoidNodesForTri`,
/// `00691660`); a triangle's flag test (`00691140`).
const NAVMESH_ADD_AVOID_NODE: u32 = 0x0069_1510;
const NAVMESH_CLOSED_DOOR_INFO: u32 = 0x0069_9af0;
const NAVMESH_GET_TRIANGLE: u32 = 0x0055_82f0;
const NAVMESH_STATIC_AVOID_NODES: u32 = 0x0069_1660;
const TRIANGLE_HAS_FLAG: u32 = 0x0069_1140;
/// `Pathing::GetNavMeshZForLocation` (`006d9830`, cdecl).
const NAVMESH_Z_FOR_LOCATION: u32 = 0x006d_9830;
/// `NiPointer` constructor from null (`0042fb00`), copy constructor
/// (`00559a40`) and destructor (`0042fa40`).
const NIPOINTER_CONSTRUCT_NULL: u32 = 0x0042_fb00;
const NIPOINTER_COPY: u32 = 0x0055_9a40;
const NIPOINTER_DESTRUCT: u32 = 0x0042_fa40;
/// Virtual slots of a process (0x278) and of the object of `009e55e0`.
const PROCESS_SLOT_278: u32 = 0x278;
const OBJECT_SLOT_27C: u32 = 0x27c;
const OBJECT_SLOT_280: u32 = 0x280;
const OBJECT_SLOT_128: u32 = 0x128;
/// `PathingSolution`: advance along it (`006e7e70`), the distance at a
/// spline parameter (`006e84b0`), whether it is usable (`009dc530`), the
/// location after a distance (`006e8790`), node `i` (`006e7970`), the node
/// count again (`005a4320`), the spline parameter at a distance
/// (`006e8680`), the point at a spline parameter (`006e7a10`).
const PATH_ADVANCE: u32 = 0x006e_7e70;
const PATH_DISTANCE_AT: u32 = 0x006e_84b0;
const PATH_IS_USABLE: u32 = 0x009d_c530;
const PATH_LOCATION_AFTER: u32 = 0x006e_8790;
const PATH_NODE: u32 = 0x006e_7970;
const PATH_NODE_COUNT_2: u32 = 0x005a_4320;
const PATH_PARAMETER_AT: u32 = 0x006e_8680;
const PATH_POINT_AT: u32 = 0x006e_7a10;
/// `this + 0x2C` (`005d8a70`), `this + 0x80` (`0096e290`), `this + 0xC`
/// (`0048d150`, `PathingRequest::Origin`).
const PLUS_2C: u32 = 0x005d_8a70;
const PLUS_80: u32 = 0x0096_e290;
const PLUS_C: u32 = 0x0048_d150;
/// Point helpers: add into a result (`00439e90`), divide into a result
/// (`0053d280`), squared length (`004a7290`), `NiPoint3::UnitizeGetLength`
/// (`00457910`), point times matrix (`004b3ae0`, cdecl: result, point,
/// matrix).
const POINT_ADD: u32 = 0x0043_9e90;
const POINT_DIVIDE: u32 = 0x0053_d280;
const POINT_LENGTH_SQUARED: u32 = 0x004a_7290;
const POINT_UNITIZE: u32 = 0x0045_7910;
const POINT_TIMES_MATRIX: u32 = 0x004b_3ae0;
/// Process flags 0x4000 (`007ff8e0`) and 0x100000 (`00552470`).
const PROCESS_FLAG_4000: u32 = 0x007f_f8e0;
const PROCESS_FLAG_100000: u32 = 0x0055_2470;
/// `TESObjectREFR::Activate` (`00573170`), `SetTargeted` (`00564db0`),
/// `GetWorldSpace` (`00575d70`), `GetRelevantWaterHeight` (`0057b0a0`), the
/// parent cell (+0x40, `008d6f30`), the position (+0x30, `00436aa0`).
const REFERENCE_ACTIVATE: u32 = 0x0057_3170;
const REFERENCE_SET_TARGETED: u32 = 0x0056_4db0;
const REFERENCE_GET_WORLDSPACE: u32 = 0x0057_5d70;
const REFERENCE_WATER_HEIGHT: u32 = 0x0057_b0a0;
const REFERENCE_PARENT_CELL: u32 = 0x008d_6f30;
const REFERENCE_POSITION: u32 = 0x0043_6aa0;
/// A reference's virtual slot 0x100.
const REFERENCE_SLOT_100: u32 = 0x100;
/// `PathingRequest`: its avoid node array (`0069e390`) and setter
/// (`006d61e0`), bytes +0x98 (`0073cb80`), +0x99 (`00505c00`), +0xAC
/// (`00632ce0`), the float +0x8C (`00507e10`), set byte +0x9C (`008e7c50`),
/// set the initial heading (`006e2bb0`), set the origin (`006d3ac0`).
const REQUEST_AVOID_NODES: u32 = 0x0069_e390;
const REQUEST_SET_AVOID_NODES: u32 = 0x006d_61e0;
const REQUEST_BYTE_98: u32 = 0x0073_cb80;
const REQUEST_BYTE_99: u32 = 0x0050_5c00;
const REQUEST_BYTE_AC: u32 = 0x0063_2ce0;
const REQUEST_FLOAT_8C: u32 = 0x0050_7e10;
const REQUEST_SET_BYTE_9C: u32 = 0x008e_7c50;
const REQUEST_SET_INITIAL_HEADING: u32 = 0x006e_2bb0;
const REQUEST_SET_ORIGIN: u32 = 0x006d_3ac0;
/// `NavMeshMath::SegmentCircleIntersection` (`006ba9e0`, cdecl).
const SEGMENT_CIRCLE_INTERSECTION: u32 = 0x006b_a9e0;
/// The signed byte at +0x20 (`0041ca90`).
const SIGNED_BYTE_AT_20: u32 = 0x0041_ca90;
/// The dword at +0x14 (`00825c00`).
const STAMP_OF: u32 = 0x0082_5c00;
/// A support object of a character controller's support (`00810660`).
const SUPPORT_OBJECT: u32 = 0x0081_0660;
/// `TES::GetWorldSpace` (`004fd3e0`) and the interior cell (`005f36f0`).
const TES_GET_WORLDSPACE: u32 = 0x004f_d3e0;
const TES_INTERIOR_CELL: u32 = 0x005f_36f0;
/// A trigonometric float function (`004f6e40`; Ghidra's library match says
/// `_logf`, its body calls a `double` function Ghidra cannot tell from
/// `cos`); `009e3560` uses it on the vertical component of a unit vector.
const TRIG_004F6E40: u32 = 0x004f_6e40;
/// `TESWorldSpace::GetCellFromWorldCoord` (`00587550`).
const WORLDSPACE_CELL_AT: u32 = 0x0058_7550;

// More own virtual slots.
const SLOT_GET_DISTANCE_TRAVELED: u32 = 0x18;
const SLOT_IS_DETAILED: u32 = 0x34;
const SLOT_IS_WAITING_FOR_AVOIDANCE: u32 = 0x3c;
const SLOT_GET_MOVE_MODE: u32 = 0x48;

// More functions of this unit translated by later sessions.
/// The avoidance hit array (`009e9950` constructor, `009e9980` destructor,
/// `009e99a0` add).
const AVOID_ARRAY_CONSTRUCT: u32 = 0x009e_9950;
const AVOID_ARRAY_DESTRUCT: u32 = 0x009e_9980;
const AVOID_ARRAY_ADD: u32 = 0x009e_99a0;

// Functions of this unit that the earlier functions call by address (now
// translated below; the calls stay by address so that tests can double
// them).
/// `DetailedActorPathHandler::CheckGoalReached` (`009e8000`).
const CHECK_GOAL_REACHED: u32 = 0x009e_8000;
/// Resets `MoveMode`, `MoveDelta` and the last frame's delta (`009e8790`).
const RESET_MOVE_DELTA: u32 = 0x009e_8790;

// Callees of `CheckGoalReached`, `WarpToNextNode`, `PrintCurrentStatus`,
// `SaveGame`, `LoadGame`, `InitLoadGame` and the avoidance array.
/// The dword at +0x1C (`00441110`, the body of
/// `PathingLocation::GetWorldspace`); `CheckGoalReached` reads it from a
/// path and uses it as a node index.
const DWORD_AT_1C: u32 = LOCATION_GET_WORLDSPACE;
/// A node of a path by index (`006e78b0`: the path's array at +8,
/// `006e8f40`).
const PATH_NODE_BY_INDEX: u32 = 0x006e_78b0;
/// A check in `pathfind.cpp` (`006d5340`, cdecl: request, path, node index,
/// location; it has an exception frame of its own). `CheckGoalReached`
/// treats 1 as the goal and 2 and 3 as "keep updating".
const PATHFIND_006D5340: u32 = 0x006d_5340;
/// The float at +0x70 of a `PathingRequest` (`0051b860`, ST0).
const REQUEST_FLOAT_70: u32 = 0x0051_b860;
/// `min(a, b)` of two floats (`0040ebd0`, cdecl, ST0).
const FLOAT_MIN: u32 = 0x0040_ebd0;
/// The reference's name source: `00474cb0` (0 while the guard byte
/// `011c40c8` is set, else `0044a670` of what the actor's slot 0x130
/// returns when that is not null), then the slot itself, or
/// `TESObjectREFR`'s name (`0055d520`).
const HAS_NAME: u32 = 0x0047_4cb0;
const ACTOR_SLOT_NAME: u32 = 0x130;
const REFERENCE_NAME: u32 = 0x0055_d520;
/// The game's log line (`005b5e40`, cdecl, printf arguments).
const LOG_MESSAGE: u32 = 0x005b_5e40;
/// "PATHFINDING: Actor %s failed pathing due to goal Z difference %f,
/// being too large" (`010924b0`).
const GOAL_Z_MESSAGE: u32 = 0x0109_24b0;
/// `double` 25600.0 (`01092508`): the squared distance to the last node
/// within which `CheckGoalReached` keeps updating the path's end.
const END_UPDATE_RANGE_SQUARED: u32 = 0x0109_2508;
/// `float` 25.0 (`011a5d00`).
const GOAL_MIN_DISTANCE: u32 = 0x011a_5d00;
/// `float` 64.0 (`0101e6ec`) and 128.0 (`0101e704`): the distances
/// `WarpToNextNode` uses.
const WARP_MIN_DISTANCE: u32 = 0x0101_e6ec;
const WARP_TETHER_DISTANCE: u32 = 0x0101_e704;
/// `double` 0.6000000238418579 (`010290b8`).
const WARP_END_FACTOR: u32 = 0x0102_90b8;
/// A float of the actor (`008be350`, ST0; read through its process at
/// +0x68 or a slot of the object at +0xA8).
const ACTOR_MEASURE_008BE350: u32 = 0x008b_e350;
/// `ActorMover::MoveActorToLocation` (`009df010`).
const MOVER_MOVE_ACTOR_TO_LOCATION: u32 = 0x009d_f010;
/// The `NiTArray` compaction (`00600bc0`): moves the non-null elements
/// down, shrinks the buffer to them.
const ARRAY_COMPACT: u32 = 0x0060_0bc0;
/// Removes an item from a `BSSimpleList` (`00905330`, item address).
const LIST_REMOVE: u32 = 0x0090_5330;
/// Stores the item of a list node (`00726c60`, item address).
const LIST_SET_ITEM: u32 = 0x0072_6c60;
/// `this` itself (`006815c0`, the same body as the point's default
/// constructor): a list node's item address.
const LIST_NODE_ITEM: u32 = DEFAULT_CONSTRUCT;
/// String copy, append and format with a size (`00406d30`, `00406d50`,
/// `00406d00`; cdecl, buffer, size, then the source or the format).
const STRING_COPY: u32 = 0x0040_6d30;
const STRING_APPEND: u32 = 0x0040_6d50;
const STRING_FORMAT: u32 = 0x0040_6d00;
/// The status printer's slot 0x14 (cdecl virtual: printer, two words passed
/// through, format, arguments).
const PRINTER_SLOT_PRINT: u32 = 0x14;
/// `BGSSaveGameBuffer`: save bytes (`00865e50`: address, size, 0), a form
/// ID (`SaveFormID_ov2`, `00865df0`), `StartVariableSizedValue`
/// (`00865f20`), `SaveVariableSizedValue_ov2` (`00865ff0`).
const SAVE_BYTES: u32 = 0x0086_5e50;
const SAVE_FORM_ID: u32 = 0x0086_5df0;
const SAVE_START_VARIABLE_SIZED: u32 = 0x0086_5f20;
const SAVE_VARIABLE_SIZED: u32 = 0x0086_5ff0;
/// `BGSLoadGameBuffer`: load bytes (`00864980`: address, size),
/// `LoadFormID_ov2` (`008648e0`), `LoadVariableSizedValue` (`00864a60`);
/// its slot 0 gives the save's version byte.
const LOAD_BYTES: u32 = 0x0086_4980;
const LOAD_FORM_ID: u32 = 0x0086_48e0;
const LOAD_VARIABLE_SIZED: u32 = 0x0086_4a60;
const LOAD_SLOT_VERSION: u32 = 0x00;
/// `__RTDynamicCast` (`00ec43fb`) and the type descriptors it is given:
/// `TESForm` (`01183028`), `TESObjectREFR` (`011841cc`), `Actor`
/// (`011846d4`), read from the exe.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_OBJECT_REFR: u32 = 0x0118_41cc;
const TYPE_ACTOR: u32 = 0x0118_46d4;
/// The `BSSimpleArray<ActorAvoidanceData,1024>` vtable (`01092640`, its
/// RTTI type descriptor read from the exe), the base destructor
/// (`008454f0`), the slot that allocates (+4), the grow step `Add` uses
/// (`00845460`, returns the new element's index) and the element
/// constructor over a range (`009e9a50`).
const AVOID_ARRAY_VTABLE: u32 = 0x0109_2640;
const SIMPLE_ARRAY_DESTRUCT: u32 = 0x0084_54f0;
const SIMPLE_ARRAY_SLOT_ALLOCATE: u32 = 0x04;
const SIMPLE_ARRAY_GROW: u32 = 0x0084_5460;
const AVOID_DATA_CONSTRUCT_RANGE: u32 = 0x009e_9a50;

// Actor virtual slots.
const ACTOR_SLOT_LOCATION: u32 = 0x1f4;
const ACTOR_SLOT_GET_HEADING: u32 = 0x2bc;
const ACTOR_SLOT_SET_HEADING: u32 = 0x2c4;
/// A float the actor returns (`Character`: `008d3290`, a setting's value);
/// `NoPathUpdate` scales it by pi/180 as degrees per second.
const ACTOR_SLOT_TURN_SPEED: u32 = 0x354;
/// A flag (`Character`: `0047c850`, returns 0).
const ACTOR_SLOT_21C: u32 = 0x21c;

// Own virtual slots.
const SLOT_UPDATE: u32 = 0x0c;
const SLOT_UPDATE_WHILE_PLAYER_WAITS: u32 = 0x14;

/// `PATH_HANDLER_STATE` values as the code uses them (names unconfirmed):
/// 1 initializing, 2 moving, 3 the tether point reached the end, 4 passed
/// the end, 5 done, 6 rotating only, 7 rotation finished.
const STATE_INIT: u32 = 1;
const STATE_MOVING: u32 = 2;
const STATE_AT_END: u32 = 3;
const STATE_PAST_END: u32 = 4;
const STATE_DONE: u32 = 5;
const STATE_ROTATING: u32 = 6;
const STATE_ROTATED: u32 = 7;

/// `float` from memory loaded and stored again through the FPU.
fn fld(e: &Engine, addr: u32) -> f32 {
    x87_quiet(e.mem.f32(addr))
}

/// Three words copied with `MOV`s.
fn copy_point(e: &mut Engine, dst: u32, src: u32) {
    for i in 0..3 {
        let w = e.mem.u32(src + 4 * i);
        e.mem.set_u32(dst + 4 * i, w);
    }
}

fn path_of(e: &mut Engine, this: Ptr<H>) -> u32 {
    e.call(NIPOINTER_GET, &args![this.addr() + H::spPath.off])
        .u32()
}

fn request_of(e: &mut Engine, this: Ptr<H>) -> u32 {
    e.call(NIPOINTER_GET, &args![this.addr() + H::spRequest.off])
        .u32()
}

fn actor_location(e: &mut Engine, this: Ptr<H>) -> u32 {
    let actor = e.get(this, H::pActor).addr();
    e.vcall(actor, ACTOR_SLOT_LOCATION, &[]).u32()
}

fn actor_heading(e: &mut Engine, this: Ptr<H>) -> f64 {
    let actor = e.get(this, H::pActor).addr();
    e.vcall(actor, ACTOR_SLOT_GET_HEADING, &args![0u32]).f64()
}

fn set_actor_heading(e: &mut Engine, this: Ptr<H>, heading: f32) {
    let actor = e.get(this, H::pActor).addr();
    e.vcall(actor, ACTOR_SLOT_SET_HEADING, &args![heading]);
}

fn mover_of(e: &Engine, this: Ptr<H>) -> u32 {
    e.get(this, H::pActorMover).addr()
}

fn reset_move_delta(e: &mut Engine, this: Ptr<H>) {
    e.call(RESET_MOVE_DELTA, &args![this]);
}

/// A float setting's value, loaded through the FPU.
fn setting_float(e: &mut Engine, setting: u32) -> f32 {
    let value = e.call(SETTING_VALUE, &args![setting]).u32();
    fld(e, value)
}

fn double(e: &Engine, addr: u32) -> f64 {
    e.global::<f64>(addr)
}

/// `MoveDelta` and the last frame's delta set to the "no point" value and
/// `fLastFrameMinMoveDelta` to 0 (the same stores in several functions).
fn clear_frame_delta(e: &mut Engine, this: Ptr<H>) {
    copy_point(e, this.addr() + H::MoveDelta.off, NO_POINT);
    e.set(this, H::fLastFrameMinMoveDelta, 0.0);
    copy_point(e, this.addr() + H::LastFrameMinMoveDelta.off, NO_POINT);
}

// Translated from 00401290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns `true` and ignores its one stack word. The engine map files it
/// here as `DetailedActorPathHandler::IsDetailedPathHandler`, but the
/// handler's slot 0x34 is `008d0360`; this body is a shared `return true`
/// (`RET 4`) that many vtables point at.
pub fn fn_00401290(_e: &mut Engine, _this: u32, _unused_1: u32) -> bool {
    true
}

// Translated from 0070ec90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetailedActorPathHandler::GetCurrentNodeIndex` (Xbox PDB; slot 0x30 of
/// the vtable): `iCurrentNodeIndex`.
pub fn detailed_actor_path_handler_get_current_node_index(e: &mut Engine, this: Ptr<H>) -> u32 {
    e.get(this, H::iCurrentNodeIndex)
}

/// The constructors' common stores after the base constructor.
fn construct_fields(e: &mut Engine, this: Ptr<H>) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE);
    copy_point(e, base + H::MoveDelta.off, NO_POINT);
    e.call(DEFAULT_CONSTRUCT, &args![base + H::BlockedTargetPoint.off]);
    copy_point(e, base + H::TetherTargetPoint.off, NO_POINT);
    let max = fld(e, FLOAT_MAX);
    e.call(
        POINT_CONSTRUCT,
        &args![base + H::LastLocation.off, max, max, max],
    );
    copy_point(e, base + H::LastTetherPoint.off, NO_POINT);
    e.call(LIST_CONSTRUCT, &args![base + H::ActorAvoidList.off]);
    e.call(
        DEFAULT_CONSTRUCT,
        &args![base + H::LastFrameMinMoveDelta.off],
    );
    e.set(this, H::MoveMode, 0);
    e.set(this, H::fTimeInAnim, 1.0);
    let minus_one = fld(e, MINUS_ONE_FLOAT);
    e.set(this, H::fMinRotateTime, minus_one);
    e.set(this, H::fAvoidanceDetectionValue, 0.0);
    e.set(this, H::fIgnoreLookAtPointTimer, 0.0);
    e.set(this, H::bEndUpdatePath, false);
    e.set(this, H::bWaitingForUpdate, false);
    e.set(this, H::bDeltaComputedThisFrame, false);
    e.set(this, H::bIgnoreLookAtPoint, false);
}

/// The constructors' stores after `eState`.
fn construct_rest(e: &mut Engine, this: Ptr<H>) {
    e.set(this, H::eBlendRunning, 0);
    e.set(this, H::fRunWalkTime, 0.0);
    e.set(this, H::eAvoidState, 0);
    e.set(this, H::iCurrentNodeIndex, 0);
    e.set(this, H::fGoalRadiusSqr, 0.0);
    e.set(this, H::fDistanceTraveled, 0.0);
    e.set(this, H::fPathLength, 0.0);
    e.set(this, H::fCurrentSplineT, 0.0);
    let min_move = fld(e, TETHER_MIN_MOVE);
    e.set(this, H::fTetherMinMove, min_move);
    let dist = fld(e, TETHER_DIST_FROM_ACTOR);
    e.set(this, H::fTetherDistFromActor, dist);
    e.set(this, H::pCurrentDoor, Ptr::NULL);
    e.set(this, H::bIgnoreInitialRotation, false);
    e.set(this, H::bIgnoreInitialRotationKeepActorHeading, false);
    e.set(this, H::fIgnoreInitialRotationTetherParam, 0.0);
    e.set(this, H::fLastRotationApplied, 0.0);
}

// Translated from 009dff40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009dff40 - C++ exception frame (FS:[0]) and its unwinding not translated
/// `DetailedActorPathHandler::DetailedActorPathHandler` (Xbox PDB):
/// constructs the handler for `actor` and `mover` without a path; state 5.
pub fn detailed_actor_path_handler_detailed_actor_path_handler(
    e: &mut Engine,
    this: Ptr<H>,
    actor: Ptr,
    mover: Ptr,
) -> Ptr<H> {
    e.call(BASE_CONSTRUCT, &args![this, actor, mover]);
    construct_fields(e, this);
    e.set(this, H::eState, STATE_DONE);
    construct_rest(e, this);
    this
}

// Translated from 009e0160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetailedActorPathHandler::scalar deleting destructor` (Xbox PDB): runs
/// the destructor body and frees the object when bit 0 of `flags` is set.
pub fn detailed_actor_path_handler_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<H>,
    flags: u32,
) -> Ptr<H> {
    fn_009e03e0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 009e0190 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009e0190 - C++ exception frame (FS:[0]) and its unwinding not translated
/// The constructor used for a path: `actor`, `mover`, the address of the
/// path pointer to hold and the request. The state is 5 (done) when
/// `*path_pointer` is null and no request is given, else 0.
pub fn fn_009e0190(
    e: &mut Engine,
    this: Ptr<H>,
    actor: Ptr,
    mover: Ptr,
    path_pointer: Ptr,
    request: Ptr,
) -> Ptr<H> {
    e.call(
        BASE_CONSTRUCT_WITH_PATH,
        &args![this, actor, mover, path_pointer, request],
    );
    construct_fields(e, this);
    let no_path = e.call(POINTER_EQUALS, &args![path_pointer, 0u32]).bool();
    let state = if no_path && request.is_null() {
        STATE_DONE
    } else {
        0
    };
    e.set(this, H::eState, state);
    construct_rest(e, this);
    this
}

// Translated from 009e03e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009e03e0 - C++ exception frame (FS:[0]) and its unwinding not translated
/// The destructor body: releases the path and the request, clears and
/// destroys the avoid list, then runs the base destructor.
pub fn fn_009e03e0(e: &mut Engine, this: Ptr<H>) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE);
    e.call(NIPOINTER_ASSIGN, &args![base + H::spPath.off, 0u32]);
    e.call(NIPOINTER_ASSIGN, &args![base + H::spRequest.off, 0u32]);
    e.call(LIST_CLEAR, &args![base + H::ActorAvoidList.off]);
    e.call(LIST_DESTRUCT, &args![base + H::ActorAvoidList.off]);
    e.call(BASE_DESTRUCT, &args![this]);
}

// Translated from 009e0470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetailedActorPathHandler::Init` (Xbox PDB): resets the per-path state,
/// takes the goal radius from the path and request, sets up the tether and
/// the initial rotation, and runs a first `Update` of `update_seconds` when
/// that is positive (its result is returned; 0 otherwise). A path with
/// fewer than two nodes finishes at once (state 5, returns 5).
/// `start_distance` goes to `009e77d0`; when it is 0.0 the initial heading is
/// handed to `InitRotationMode`, otherwise set on the actor directly.
pub fn detailed_actor_path_handler_init(
    e: &mut Engine,
    this: Ptr<H>,
    start_distance: f32,
    update_seconds: f32,
) -> u32 {
    e.set(this, H::eState, STATE_INIT);
    if e.get(this, H::eBlendRunning) == 0 {
        e.set(this, H::eBlendRunning, 3);
    }
    let avoid = e.get(this, H::eAvoidState);
    if avoid != 2 && avoid != 4 {
        e.set(this, H::eAvoidState, 1);
        e.set(this, H::fActorAvoidanceTimer, 0.0);
        e.set(this, H::fWaitTime, 0.0);
    }
    copy_point(e, this.addr() + H::MoveDelta.off, NO_POINT);
    e.set(this, H::bRotateMode, false);
    e.set(this, H::bDeltaComputedThisFrame, false);
    e.set(this, H::bMovementBlocked, false);
    e.set(this, H::fBlockTime, 0.0);
    e.set(this, H::iUnblockDirection, 0);
    e.set(this, H::fAlternateFacingAngle, 0.0);
    e.set(this, H::fBlockedStartAngle, 0.0);
    e.set(this, H::fLastFrameMinMoveDelta, 0.0);
    copy_point(e, this.addr() + H::LastFrameMinMoveDelta.off, NO_POINT);
    let location = actor_location(e, this);
    copy_point(e, this.addr() + H::LastLocation.off, location);
    e.set(this, H::fFallTime, 0.0);
    let minus_one = fld(e, MINUS_ONE_FLOAT);
    e.set(this, H::fMinRotateTime, minus_one);

    let path = path_of(e, this);
    if e.call(PATH_NODE_COUNT, &args![path]).u32() < 2 {
        reset_move_delta(e, this);
        e.set(this, H::bWaitingForUpdate, false);
        let location = actor_location(e, this);
        copy_point(e, this.addr() + H::TetherTargetPoint.off, location);
        copy_point(
            e,
            this.addr() + H::LastTetherPoint.off,
            this.addr() + H::TetherTargetPoint.off,
        );
        e.set(this, H::eState, STATE_DONE);
        return 5;
    }

    let path = path_of(e, this);
    let radius = if e.call(PATH_BYTE_40, &args![path]).bool() {
        fld(e, GOAL_RADIUS_PATH_FLAG)
    } else {
        let minimum = fld(e, ONE_TENTH_FLOAT);
        let request = request_of(e, this);
        let first = e.call(FLOAT_AT_74, &args![request]).f64();
        let request = request_of(e, this);
        let second = e.call(FLOAT_AT_74, &args![request]).f64();
        let product = x87_quiet((second * first) as f32);
        let mut radius = e.call(FLOAT_MAX_OF, &args![product, minimum]).f64() as f32;
        let path = path_of(e, this);
        let source = e.call(PATH_END_NODE, &args![path]).u32();
        if e.call(FLAGS_BOTH_LOW, &args![source]).bool() {
            radius = fld(e, GOAL_RADIUS_BOTH_FLAGS);
        }
        radius
    };
    e.set(this, H::fGoalRadiusSqr, radius);
    let path = path_of(e, this);
    let source = e.call(PATH_END_NODE, &args![path]).u32();
    if e.call(FLAGS_BIT_2, &args![source]).bool() {
        e.set(this, H::bEndUpdatePath, true);
    }
    fn_009e27e0(e, this);
    let waiting = e.get(this, H::bWaitingForUpdate);
    fn_009e77d0(e, this, x87_quiet(start_distance));
    e.set(this, H::bIgnoreInitialRotation, false);
    e.set(this, H::bIgnoreInitialRotationKeepActorHeading, false);
    e.set(this, H::fIgnoreInitialRotationTetherParam, 0.0);
    let actor = e.get(this, H::pActor);
    if !e.call(BYTE_AT_104, &args![actor]).bool() {
        let path = path_of(e, this);
        let length = e.call(PATH_LENGTH, &args![path]).f64();
        if e.get(this, H::fTetherDistFromActor) as f64 > length {
            let mover = mover_of(e, this);
            if !e.call(MOVER_FACE_TARGET_POINT, &args![mover]).bool() {
                e.set(this, H::bIgnoreInitialRotation, true);
                e.set(this, H::bIgnoreInitialRotationKeepActorHeading, true);
                let path = path_of(e, this);
                let count = e.call(PATH_NODE_COUNT, &args![path]).u32();
                e.set(this, H::fIgnoreInitialRotationTetherParam, count as f32);
            }
        }
    }

    if e.call(CHECK_GOAL_REACHED, &args![this]).u32() == 0 {
        let heading = e.with_stack(12, |e, target| {
            e.call(DEFAULT_CONSTRUCT, &args![target]);
            let have_look_at = !e.get(this, H::bIgnoreLookAtPoint) && {
                let mover = mover_of(e, this);
                e.call(MOVER_GET_LOOK_AT_POINT, &args![mover, target])
                    .bool()
            };
            if !have_look_at {
                copy_point(e, target.addr(), this.addr() + H::TetherTargetPoint.off);
            }
            let location = actor_location(e, this);
            e.call(POINT_SUBTRACT_ASSIGN, &args![target, location]);
            let length = e.call(POINT_LENGTH, &args![target]).f64();
            let limit = length * double(e, NINETY_FIVE_HUNDREDTHS);
            let z = fld(e, target.addr() + 8);
            let z_abs = e.call(FLOAT_ABS, &args![z]).f64();
            if z_abs >= limit {
                actor_heading(e, this) as f32
            } else {
                e.call(Z_ANGLE_FROM_VECTOR, &args![target]).f64() as f32
            }
        });
        if !waiting {
            if start_distance as f64 == double(e, ZERO_DOUBLE) {
                detailed_actor_path_handler_init_rotation_mode(
                    e,
                    this,
                    x87_quiet(heading),
                    false,
                    waiting,
                );
            } else {
                set_actor_heading(e, this, x87_quiet(heading));
            }
        }
    }
    e.set(this, H::eState, STATE_MOVING);
    if update_seconds as f64 > double(e, ZERO_DOUBLE) {
        e.vcall(this.addr(), SLOT_UPDATE, &args![x87_quiet(update_seconds)])
            .u32()
    } else {
        0
    }
}

// Translated from 009e0930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetailedActorPathHandler::InitRotationOnly` (Xbox PDB): drops the path
/// and request and starts turning in place to `heading`
/// (`InitRotationMode(heading, flag, false)`); state 6 when that starts a
/// rotation, else 5.
pub fn detailed_actor_path_handler_init_rotation_only(
    e: &mut Engine,
    this: Ptr<H>,
    heading: f32,
    flag: bool,
) -> bool {
    e.set(this, H::bWaitingForUpdate, false);
    e.set(this, H::eState, STATE_DONE);
    e.set(this, H::eBlendRunning, 3);
    e.set(this, H::eAvoidState, 1);
    e.set(this, H::fAvoidanceDetectionValue, 0.0);
    e.set(this, H::fDistanceTraveled, 0.0);
    e.set(this, H::fPathLength, 0.0);
    e.call(NIPOINTER_ASSIGN, &args![this.addr() + H::spPath.off, 0u32]);
    e.call(
        NIPOINTER_ASSIGN,
        &args![this.addr() + H::spRequest.off, 0u32],
    );
    e.set(this, H::bIgnoreInitialRotation, false);
    e.set(this, H::bIgnoreInitialRotationKeepActorHeading, false);
    e.set(this, H::fIgnoreInitialRotationTetherParam, 0.0);
    if detailed_actor_path_handler_init_rotation_mode(e, this, x87_quiet(heading), flag, false) {
        e.set(this, H::eState, STATE_ROTATING);
        true
    } else {
        e.set(this, H::eState, STATE_DONE);
        false
    }
}

// Translated from 009e0a00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009e0a00 - C++ exception frame (FS:[0]) and its unwinding not translated
/// `DetailedActorPathHandler::Update` (Xbox PDB): one movement step of
/// `seconds`. Returns 5 when done, 4 after a long fall, 3 when a door blocks
/// the way, 1 when blocked, 0 while moving (or the goal check's result).
pub fn detailed_actor_path_handler_update(e: &mut Engine, this: Ptr<H>, seconds: f32) -> u32 {
    let state = e.get(this, H::eState);
    if state == STATE_DONE || state == STATE_ROTATING {
        return 5;
    }
    let actor = e.get(this, H::pActor);
    if !e.call(ACTOR_CAN_MOVE, &args![actor]).bool() {
        if path_of(e, this) != 0 {
            let goal = e.call(CHECK_GOAL_REACHED, &args![this]).u32();
            if goal == 5 {
                reset_move_delta(e, this);
                e.set(this, H::eState, STATE_DONE);
                return goal;
            }
        }
        return 0;
    }
    e.set(this, H::bDeltaComputedThisFrame, true);
    let controller = e.call(ACTOR_GET_CHAR_CONTROLLER, &args![actor]).u32();
    if seconds as f64 == double(e, ZERO_DOUBLE) || controller == 0 {
        reset_move_delta(e, this);
        return 0;
    }
    let player: u32 = e.global(PLAYER);
    if e.call(PLAYER_IS_SLEEPING, &args![player]).bool() {
        return e
            .vcall(
                this.addr(),
                SLOT_UPDATE_WHILE_PLAYER_WAITS,
                &args![x87_quiet(seconds)],
            )
            .u32();
    }
    if e.call(CONTROLLER_STATE, &args![controller]).u32() == 2 {
        let fall = e.call(CONTROLLER_FALL_TIME, &args![controller]).f64() as f32;
        e.set(this, H::fFallTime, fall);
        if e.get(this, H::fFallTime) as f64 > double(e, FALL_TIME_STOP) {
            reset_move_delta(e, this);
            return 0;
        }
    } else {
        if e.get(this, H::fFallTime) as f64 > double(e, FALL_TIME_LANDED) {
            reset_move_delta(e, this);
            return 4;
        }
        e.set(this, H::fFallTime, 0.0);
    }

    if !e.get(this, H::bRotateMode) && !e.get(this, H::bWaitingForUpdate) {
        let goal = e.call(CHECK_GOAL_REACHED, &args![this]).u32();
        if goal != 0 && (goal != 5 || !detailed_actor_path_handler_check_for_end_rotate(e, this)) {
            reset_move_delta(e, this);
            e.set(this, H::eState, STATE_DONE);
            return goal;
        }
    }
    if !e.get(this, H::bWaitingForUpdate) {
        fn_009e3d50(e, this, x87_quiet(seconds).to_bits());
    }
    if e.get(this, H::bIgnoreInitialRotation)
        && e.get(this, H::fIgnoreInitialRotationTetherParam) <= e.get(this, H::fCurrentSplineT)
    {
        e.set(this, H::bIgnoreInitialRotation, false);
        e.set(this, H::bIgnoreInitialRotationKeepActorHeading, false);
        e.set(this, H::fIgnoreInitialRotationTetherParam, 0.0);
    }

    if e.get(this, H::bRotateMode) {
        let heading = detailed_actor_path_handler_update_rotation(e, this, x87_quiet(seconds));
        set_actor_heading(e, this, x87_quiet(heading));
        let looking = e.with_stack(8, |e, out| {
            let heading_out = out;
            let looking_out = out.byte_add(4);
            detailed_actor_path_handler_compute_heading_and_looking(
                e,
                this,
                heading_out,
                looking_out,
            );
            fld(e, looking_out.addr())
        });
        detailed_actor_path_handler_update_looking(e, this, x87_quiet(seconds), looking);
        clear_frame_delta(e, this);
        let mover = mover_of(e, this);
        e.call(EMPTY_FUNCTION, &args![mover]);
        return 0;
    }
    if e.get(this, H::eState) == STATE_PAST_END {
        reset_move_delta(e, this);
        return 0;
    }
    if e.get(this, H::eAvoidState) == 2 {
        let wait = (e.get(this, H::fWaitTime) as f64 - seconds as f64) as f32;
        e.set(this, H::fWaitTime, wait);
        if e.get(this, H::fWaitTime) as f64 > double(e, ZERO_DOUBLE) {
            let avoid = fn_009e5ae0(e, this, x87_quiet(seconds));
            if avoid == 0 {
                let timer = (e.get(this, H::fActorAvoidanceTimer) as f64 - seconds as f64) as f32;
                e.set(this, H::fActorAvoidanceTimer, timer);
                if e.get(this, H::fActorAvoidanceTimer) as f64 <= double(e, ZERO_DOUBLE) {
                    e.set(this, H::fWaitTime, 0.0);
                    e.set(this, H::eAvoidState, 1);
                }
            } else if avoid == 3 {
                reset_move_delta(e, this);
                return 1;
            } else {
                let timer = setting_float(e, SETTING_AVOIDANCE_TIMER);
                e.set(this, H::fActorAvoidanceTimer, timer);
            }
        } else {
            e.set(this, H::fWaitTime, 0.0);
            e.set(this, H::eAvoidState, 1);
        }
        reset_move_delta(e, this);
        return 0;
    }
    match detailed_actor_path_handler_handle_door_opening(e, this) {
        1 => {
            reset_move_delta(e, this);
            return 0;
        }
        2 => {
            reset_move_delta(e, this);
            return 3;
        }
        _ => {}
    }
    fn_009e2aa0(e, this, x87_quiet(seconds));
    if !fn_009e4cf0(e, this, x87_quiet(seconds)) {
        if e.get(this, H::eState) == STATE_PAST_END {
            reset_move_delta(e, this);
            return 0;
        }
        e.set(this, H::eAvoidState, 5);
        return 1;
    }
    fn_009e3520(e, this, x87_quiet(seconds));
    let enabled = e
        .call(SETTING_BOOL, &args![SETTING_AVOIDANCE_ENABLED])
        .u32();
    if e.mem.u8(enabled) != 0 {
        if !e.get(this, H::bMovementBlocked) {
            let avoid = fn_009e5ae0(e, this, x87_quiet(seconds));
            if avoid != 0 {
                e.set(this, H::fAvoidanceDetectionValue, 0.0);
                match avoid {
                    1 => {
                        e.set(this, H::eAvoidState, 2);
                        reset_move_delta(e, this);
                        return 0;
                    }
                    2 => {
                        if let Some(result) = update_request_new_path(e, this) {
                            return result;
                        }
                    }
                    3 => {
                        reset_move_delta(e, this);
                        return 1;
                    }
                    _ => {}
                }
            }
        }
        if e.get(this, H::eAvoidState) == 4 {
            let timer = (e.get(this, H::fActorAvoidanceTimer) as f64 - seconds as f64) as f32;
            e.set(this, H::fActorAvoidanceTimer, timer);
            if (e.get(this, H::fActorAvoidanceTimer) as f64) < double(e, ZERO_DOUBLE) {
                e.set(this, H::fActorAvoidanceTimer, 0.0);
                e.set(this, H::eAvoidState, 1);
            }
        }
    }

    e.with_stack(4, |e, scale_out| {
        let heading = fn_009e4800(e, this, x87_quiet(seconds), scale_out);
        set_actor_heading(e, this, x87_quiet(heading));
        let mover = mover_of(e, this);
        e.call(EMPTY_FUNCTION, &args![mover]);
        let scale = e.mem.f32(scale_out.addr());
        if scale as f64 > double(e, ZERO_DOUBLE) {
            e.call(
                POINT_SCALE,
                &args![this.addr() + H::MoveDelta.off, x87_quiet(scale)],
            );
        }
    });
    fn_009e3560(e, this, x87_quiet(seconds).to_bits());
    let length = e
        .call(POINT_LENGTH_XY, &args![this.addr() + H::MoveDelta.off])
        .f64() as f32;
    e.set(this, H::fLastFrameMinMoveDelta, length);
    e.with_stack(0x24 + 12, |e, block| {
        let matrix = block;
        let rotated = block.byte_add(0x24);
        e.call(DEFAULT_CONSTRUCT, &args![matrix]);
        let heading_now = actor_heading(e, this) as f32;
        fn_009e1640(e, matrix, heading_now);
        let result = e
            .call(
                MATRIX_TIMES_POINT,
                &args![matrix, rotated, this.addr() + H::MoveDelta.off],
            )
            .u32();
        copy_point(e, this.addr() + H::LastFrameMinMoveDelta.off, result);
    });

    if e.get(this, H::eState) == STATE_AT_END {
        e.with_stack(24, |e, block| {
            let along = block;
            let to_actor = block.byte_add(12);
            e.call(
                POINT_SUBTRACT,
                &args![
                    this.addr() + H::TetherTargetPoint.off,
                    along,
                    this.addr() + H::LastTetherPoint.off
                ],
            );
            e.mem.set_f32(along.addr() + 8, 0.0);
            let location = actor_location(e, this);
            e.call(
                POINT_SUBTRACT,
                &args![this.addr() + H::TetherTargetPoint.off, to_actor, location],
            );
            e.mem.set_f32(to_actor.addr() + 8, 0.0);
            let dot = e.call(POINT_DOT, &args![along, to_actor]).f64() as f32;
            if dot as f64 <= double(e, ZERO_DOUBLE) {
                e.set(this, H::eState, STATE_PAST_END);
            }
        });
        return 0;
    }
    if path_of(e, this) != 0 {
        let next = e.get(this, H::iCurrentNodeIndex).wrapping_add(1);
        let path = path_of(e, this);
        let count = e.call(PATH_NODE_COUNT, &args![path]).u32();
        if next >= count {
            e.with_stack(24, |e, block| {
                let goal = block;
                let difference = block.byte_add(12);
                let path = path_of(e, this);
                let source = e.call(PATH_END_NODE, &args![path]).u32();
                let point = e.call(PLUS_4, &args![source]).u32();
                let goal = e.call(COPY_POINT_AT_4, &args![point, goal]).u32();
                let location = actor_location(e, this);
                let difference = e
                    .call(POINT_SUBTRACT, &args![location, difference, goal])
                    .u32();
                let distance = e.call(POINT_LENGTH_XY, &args![difference]).f64() as f32;
                if distance < e.get(this, H::fLastFrameMinMoveDelta) {
                    let delta = this.addr() + H::MoveDelta.off;
                    let z = fld(e, delta + 8);
                    e.call(POINT_NORMALIZE, &args![delta]);
                    let x = (e.mem.f32(delta) as f64 * distance as f64) as f32;
                    e.mem.set_f32(delta, x);
                    let y = (e.mem.f32(delta + 4) as f64 * distance as f64) as f32;
                    e.mem.set_f32(delta + 4, y);
                    e.mem.set_f32(delta + 8, z);
                    let length = e.call(POINT_LENGTH_XY, &args![delta]).f64() as f32;
                    e.set(this, H::fLastFrameMinMoveDelta, length);
                    copy_point(e, this.addr() + H::LastFrameMinMoveDelta.off, delta);
                }
            });
        }
    }
    0
}

/// `Update`'s avoidance result 2: asks the mover for a new path to the same
/// destination with a copy of the request. `Some(result)` when `Update`
/// returns there, `None` when it goes on.
fn update_request_new_path(e: &mut Engine, this: Ptr<H>) -> Option<u32> {
    let block = e.call(OPERATOR_NEW, &args![0xb0u32]).u32();
    let request = if block != 0 {
        e.call(REQUEST_CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    e.with_stack(4, |e, holder| {
        e.call(NIPOINTER_FROM_RAW, &args![holder, request]);
        let new_request = e.call(NIPOINTER_GET, &args![holder]).u32();
        let current = request_of(e, this);
        e.call(REQUEST_COPY_TO, &args![current, new_request]);
        if path_of(e, this) == 0 {
            e.set(this, H::eAvoidState, 2);
            reset_move_delta(e, this);
            e.call(NIPOINTER_RELEASE, &args![holder]);
            return Some(0);
        }
        let path = path_of(e, this);
        let use_path_end = if !e.call(PATH_BYTE_40, &args![path]).bool() {
            true
        } else {
            let current = request_of(e, this);
            let destination = e.call(PLUS_34, &args![current]).u32();
            e.call(LOCATION_FLAG_2, &args![destination]).bool()
        };
        let location = if use_path_end {
            let path = path_of(e, this);
            let last = e.call(PATH_LAST_NODE, &args![path]).u32();
            e.call(PLUS_4, &args![last]).u32()
        } else {
            let current = request_of(e, this);
            e.call(PLUS_34, &args![current]).u32()
        };
        let new_request = e.call(NIPOINTER_GET, &args![holder]).u32();
        e.call(LOCATION_ASSIGN, &args![new_request, location]);
        let new_request = e.call(NIPOINTER_GET, &args![holder]).u32();
        fn_009e16c0(e, Ptr::new(new_request));
        e.set(this, H::eAvoidState, 3);
        let new_request = e.call(NIPOINTER_GET, &args![holder]).u32();
        let mover = mover_of(e, this);
        e.call(MOVER_REQUEST_PATH, &args![mover, new_request]);
        let mover_state = e.get(Ptr::<ActorMover>::new(mover), ActorMover::eState);
        if mover_state == 6 {
            reset_move_delta(e, this);
            e.set(this, H::bWaitingForUpdate, false);
            e.set(this, H::eState, STATE_DONE);
        }
        if e.get(this, H::bWaitingForUpdate) {
            e.call(NIPOINTER_ASSIGN, &args![this.addr() + H::spPath.off, 0u32]);
            if e.get(this, H::eState) == STATE_PAST_END {
                reset_move_delta(e, this);
                e.call(NIPOINTER_RELEASE, &args![holder]);
                return Some(0);
            }
        } else {
            e.set(this, H::eAvoidState, 1);
        }
        e.call(NIPOINTER_RELEASE, &args![holder]);
        None
    })
}

// Translated from 009e1640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills the 3x3 matrix at `matrix` with the rotation by `angle` about z:
/// `[c s 0; -s c 0; 0 0 1]`, sine and cosine from the game's tables
/// (`00639730`).
pub fn fn_009e1640(e: &mut Engine, matrix: Ptr, angle: f32) {
    let m = matrix.addr();
    e.with_stack(8, |e, out| {
        let sine = out;
        let cosine = out.byte_add(4);
        e.call(SIN_COS, &args![x87_quiet(angle), sine, cosine]);
        let s = fld(e, sine.addr());
        let c = fld(e, cosine.addr());
        e.mem.set_f32(m, c);
        e.mem.set_f32(m + 4, s);
        e.mem.set_f32(m + 8, 0.0);
        e.mem.set_f32(m + 0xc, -s);
        e.mem.set_f32(m + 0x10, c);
        e.mem.set_f32(m + 0x14, 0.0);
        e.mem.set_f32(m + 0x18, 0.0);
        e.mem.set_f32(m + 0x1c, 0.0);
        e.mem.set_f32(m + 0x20, 1.0);
    });
}

// Translated from 009e16c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the byte at +0xA0 of a `PathingRequest`
/// (`bFirstTangentUsesHeading`).
pub fn fn_009e16c0(e: &mut Engine, request: Ptr) {
    e.mem.set_u8(request.addr() + 0xa0, 0);
}

// Translated from 009e16e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetailedActorPathHandler::NoPathUpdate` (Xbox PDB): the update while
/// there is no path to follow. Depending on `eState` it runs `Update`
/// (3), finishes (0, 4, 5, 7) or turns in place (6); in the other states it
/// turns the actor towards the mover's look-at point by at most the turn
/// rate times `seconds`, setting the mover's turn bits in `MoveMode`
/// (0x20 left, 0x10 right). Returns 5 when finished, 0 otherwise.
pub fn detailed_actor_path_handler_no_path_update(
    e: &mut Engine,
    this: Ptr<H>,
    seconds: f32,
) -> u32 {
    let mut result: i32 = -1;
    let actor = e.get(this, H::pActor);
    let base = e.call(ACTOR_BASE_FORM, &args![actor]).u32();
    let immobile = e.call(ACTOR_BASE_IS_IMMOBILE, &args![base]).bool();
    let mover = mover_of(e, this);
    if !e.call(ACTOR_BYTE_1B0, &args![actor]).bool()
        && !e.call(MOVER_BYTE_70, &args![mover]).bool()
        && !e.call(MOVER_BYTE_71, &args![mover]).bool()
    {
        // The compiler's jump table at 009e1d30.
        match e.get(this, H::eState) {
            STATE_AT_END => {
                e.vcall(this.addr(), SLOT_UPDATE, &args![x87_quiet(seconds)]);
                result = 0;
            }
            STATE_ROTATED => {
                reset_move_delta(e, this);
                e.set(this, H::eState, STATE_DONE);
                result = 5;
            }
            STATE_ROTATING => {
                let heading =
                    detailed_actor_path_handler_update_rotation(e, this, x87_quiet(seconds));
                set_actor_heading(e, this, heading);
                clear_frame_delta(e, this);
                e.set(this, H::bDeltaComputedThisFrame, true);
                e.call(EMPTY_FUNCTION, &args![mover]);
                if !e.get(this, H::bRotateMode) {
                    e.set(this, H::eState, STATE_ROTATED);
                }
                result = 0;
            }
            state @ (0 | STATE_PAST_END | STATE_DONE) => {
                if state == STATE_DONE {
                    e.call(NIPOINTER_ASSIGN, &args![this.addr() + H::spPath.off, 0u32]);
                    e.call(
                        NIPOINTER_ASSIGN,
                        &args![this.addr() + H::spRequest.off, 0u32],
                    );
                }
                if !e.call(MOVER_FACE_TARGET_POINT, &args![mover]).bool() {
                    let mode = e.call(MOVER_MOVE_MODE_BASE, &args![mover]).u32();
                    e.set(this, H::MoveMode, mode);
                    result = 5;
                }
            }
            _ => {}
        }
    }

    let mut looking = 0.0f32;
    if result == -1 {
        if e.get(this, H::bIgnoreLookAtPoint) {
            let timer = (e.get(this, H::fIgnoreLookAtPointTimer) as f64 - seconds as f64) as f32;
            e.set(this, H::fIgnoreLookAtPointTimer, timer);
            if (timer as f64) < double(e, ZERO_DOUBLE) {
                e.set(this, H::bIgnoreLookAtPoint, false);
                result = 0;
            }
        }
        if result == -1 {
            looking = e.with_stack(0x60, |e, block| {
                no_path_turn(e, this, seconds, immobile, block)
            });
        }
        result = 0;
    }
    if !immobile {
        detailed_actor_path_handler_update_looking(e, this, x87_quiet(seconds), x87_quiet(looking));
    }
    result as u32
}

/// `NoPathUpdate`'s turn towards the look-at point; `block` holds the
/// game's stack locals (target +0x00, firing location +0x0C, projected
/// angles +0x18, the vector to the target +0x24, a difference +0x30, turn
/// direction +0x3C). Returns the pitch passed on to `UpdateLooking` (0.0
/// when it does not get that far).
fn no_path_turn(e: &mut Engine, this: Ptr<H>, seconds: f32, immobile: bool, block: Ptr) -> f32 {
    let target = block;
    let firing = block.byte_add(0x0c);
    let projected = block.byte_add(0x18);
    let to_target = block.byte_add(0x24);
    let difference = block.byte_add(0x30);
    let direction = block.byte_add(0x3c);
    let actor = e.get(this, H::pActor);
    let mover = mover_of(e, this);
    e.call(DEFAULT_CONSTRUCT, &args![target]);
    if !e
        .call(MOVER_GET_LOOK_AT_POINT, &args![mover, target])
        .bool()
    {
        return 0.0;
    }
    if e.call(ACTOR_IN_DIALOGUE_WITH_PLAYER, &args![actor]).bool() {
        return 0.0;
    }
    let heading_now = actor_heading(e, this) as f32;
    let mut heading = e.call(CLAMP_ANGLE, &args![heading_now]).f64() as f32;
    e.call(FIRING_LOCATION, &args![firing, actor]);
    let offset = e
        .call(POINT_SUBTRACT, &args![firing, difference, target])
        .u32();
    let distance_squared = e.call(POINT_LENGTH_SQUARED_XY, &args![offset]).f64();
    let radius_a = e.call(ACTOR_GET_RADIUS, &args![actor]).f64();
    let radius_b = e.call(ACTOR_GET_RADIUS, &args![actor]).f64();
    if radius_b * radius_a * double(e, QUARTER) > distance_squared {
        // Inside a quarter of the radius squared: nothing to turn to.
        return 0.0;
    }
    let tx = e.mem.u32(target.addr());
    let ty = e.mem.u32(target.addr() + 4);
    let tz = e.mem.u32(target.addr() + 8);
    e.call(
        ANGLE_TO_PROJECTED_POINT,
        &args![projected, actor, tx, ty, tz],
    );
    let yaw_raw = fld(e, projected.addr() + 8);
    let yaw = e.call(CLAMP_ANGLE, &args![yaw_raw]).f64() as f32;
    let pitch = fld(e, projected.addr());
    let difference_abs = if e.call(ACTOR_GET_RADIUS, &args![actor]).f64() >= double(e, HUNDRED) {
        let location = actor_location(e, this);
        e.call(POINT_SUBTRACT, &args![target, to_target, location]);
        e.mem.set_f32(to_target.addr() + 8, 0.0);
        let angle = e.call(Z_ANGLE_FROM_VECTOR, &args![to_target]).f64() as f32;
        let turn = e
            .call(
                ANGLE_DIFFERENCE,
                &args![x87_quiet(heading), angle, direction],
            )
            .f64() as f32;
        e.call(FLOAT_ABS, &args![turn]).f64() as f32
    } else {
        let turn = e
            .call(
                ANGLE_DIFFERENCE,
                &args![x87_quiet(heading), x87_quiet(yaw), direction],
            )
            .f64() as f32;
        e.call(FLOAT_ABS, &args![turn]).f64() as f32
    };
    let rate = e.vcall(actor.addr(), ACTOR_SLOT_TURN_SPEED, &[]).f64();
    let mut step = (rate * double(e, DEGREES_TO_RADIANS) * seconds as f64) as f32;
    let flagged = e.vcall(actor.addr(), ACTOR_SLOT_21C, &[]).bool();
    let byte = e.call(BYTE_AT_104, &args![actor]).bool();
    let factor = match (flagged, byte) {
        (false, true) => setting_float(e, SETTING_TURN_RATE_A),
        (false, false) => setting_float(e, SETTING_TURN_RATE_B),
        (true, true) => setting_float(e, SETTING_TURN_RATE_C),
        (true, false) => setting_float(e, SETTING_TURN_RATE_D),
    };
    step = (step as f64 * factor as f64) as f32;
    let mut turn = x87_quiet(difference_abs);
    if step < turn {
        turn = x87_quiet(step);
    }
    let dir = e.mem.f32(direction.addr());
    let mut mode = e.get(this, H::MoveMode);
    if !immobile && e.get(this, H::fMinRotateTime) as f64 > double(e, ZERO_DOUBLE) {
        let left = (e.get(this, H::fMinRotateTime) as f64 - seconds as f64) as f32;
        e.set(this, H::fMinRotateTime, left);
        heading = (turn as f64 * dir as f64 + heading as f64) as f32;
    } else if e.global::<f32>(TURN_THRESHOLD) < difference_abs {
        if !immobile {
            let minimum = e.call(SETTING_VALUE, &args![SETTING_MIN_ROTATE_TIME]).u32();
            let left = (e.mem.f32(minimum) as f64 - seconds as f64) as f32;
            e.set(this, H::fMinRotateTime, left);
        }
        if dir as f64 == double(e, ONE_DOUBLE) {
            mode = e.call(MOVER_MOVE_MODE_BASE, &args![mover]).u32() | 0x20;
            heading = (heading as f64 + turn as f64) as f32;
        } else if dir as f64 == double(e, MINUS_ONE_DOUBLE) {
            mode = e.call(MOVER_MOVE_MODE_BASE, &args![mover]).u32() | 0x10;
            heading = (heading as f64 - turn as f64) as f32;
        }
    } else {
        mode = e.call(MOVER_MOVE_MODE_BASE, &args![mover]).u32();
        heading = (turn as f64 * dir as f64 + heading as f64) as f32;
    }
    if !e.call(MOVER_BYTE_71, &args![mover]).bool() {
        e.set(this, H::MoveMode, mode);
    }
    let set_heading = !immobile || {
        let pitch_abs = e.call(FLOAT_ABS, &args![pitch]).f64();
        pitch_abs <= double(e, STEEP_PITCH)
    };
    if set_heading {
        set_actor_heading(e, this, x87_quiet(heading));
    }
    if immobile {
        detailed_actor_path_handler_update_looking(e, this, x87_quiet(seconds), pitch);
    }
    e.call(EMPTY_FUNCTION, &args![mover]);
    pitch
}

// Translated from 009e1d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetailedActorPathHandler::UpdateWhilePlayerWaits` (Xbox PDB): while the
/// player waits or sleeps, moves the actor straight to the end of the path
/// (`008ad1c0` with the destination, a heading, its cell and worldspace) and
/// finishes: returns 5 when the mover's next node is the path's last, 6
/// otherwise. A request whose byte +0x9F is set instead calls the actor's
/// slot 0x2C0 with the player and returns 0. The word it takes (the update's
/// seconds) is not read.
pub fn detailed_actor_path_handler_update_while_player_waits(
    e: &mut Engine,
    this: Ptr<H>,
    _unused_1: u32,
) -> u32 {
    fn_009e5480(e, this);
    fn_009e68c0(e, this);
    e.set(this, H::bRotateMode, false);
    e.set(this, H::eState, STATE_MOVING);
    e.set(this, H::eBlendRunning, 3);
    e.set(this, H::fFallTime, 0.0);
    let request = request_of(e, this);
    if request != 0 {
        let request = request_of(e, this);
        if fn_009e20a0(e, Ptr::new(request)) {
            let player: u32 = e.global(PLAYER);
            let actor = e.get(this, H::pActor).addr();
            e.vcall(actor, ACTOR_SLOT_2C0, &args![player]);
            return 0;
        }
    }
    let traveled = e.vcall(this.addr(), SLOT_GET_DISTANCE_TRAVELED, &[]).f64();
    let _remaining = (e.get(this, H::fPathLength) as f64 - traveled) as f32;
    e.with_stack(0x30, |e, block| {
        let destination = block;
        let first = block.byte_add(0x0c);
        let second = block.byte_add(0x18);
        let along = block.byte_add(0x24);
        let path = path_of(e, this);
        let source = e.call(PATH_END_NODE, &args![path]).u32();
        let location = e.call(PLUS_4, &args![source]).u32();
        e.call(COPY_POINT_AT_4, &args![location, destination]);
        let angle_source = e.call(PLUS_2C, &args![source]).u32();
        let mut heading = e.call(Z_ANGLE_FROM_VECTOR, &args![angle_source]).f64() as f32;
        let cell = e.call(LOCATION_GET_CELL, &args![location]).u32();
        let worldspace = e.call(LOCATION_GET_WORLDSPACE, &args![location]).u32();
        let time_scale = e.call(CALENDAR_GET_TIME_SCALE, &args![CALENDAR]).f64();
        let hours = (double(e, SECONDS_PER_HOUR) / time_scale) as f32;
        let actor = e.get(this, H::pActor);
        let speed = e.call(ACTOR_GET_CURRENT_SPEED, &args![actor]).f64();
        let _distance = (speed * hours as f64) as f32;
        let mover = mover_of(e, this);
        let next = e.get(Ptr::<ActorMover>::new(mover), ActorMover::iNextNode) as u32 + 1;
        let path = path_of(e, this);
        let count = e.call(PATH_NODE_COUNT_2, &args![path]).u32();
        let at_end = next == count;
        let kind = e.vcall(actor.addr(), ACTOR_SLOT_214, &[]).u32();
        let skip = kind == 4 && e.vcall(actor.addr(), ACTOR_SLOT_214, &[]).u32() == 9;
        if !skip {
            let request = request_of(e, this);
            let use_request = request != 0 && {
                let request = request_of(e, this);
                e.call(REQUEST_BYTE_99, &args![request]).bool() && at_end
            };
            if use_request {
                let request = request_of(e, this);
                heading = e.call(REQUEST_FLOAT_8C, &args![request]).f64() as f32;
            } else if e.call(MOVER_FACE_TARGET_POINT, &args![mover]).bool() {
                let target = first;
                e.call(DEFAULT_CONSTRUCT, &args![target]);
                e.call(MOVER_GET_LOOK_AT_POINT, &args![mover, target]);
                let tx = e.mem.u32(target.addr());
                let ty = e.mem.u32(target.addr() + 4);
                let tz = e.mem.u32(target.addr() + 8);
                e.call(ANGLE_TO_PROJECTED_POINT, &args![second, actor, tx, ty, tz]);
                let yaw = fld(e, second.addr() + 8);
                heading = e.call(CLAMP_ANGLE, &args![yaw]).f64() as f32;
            } else {
                let path = path_of(e, this);
                let count = e.call(PATH_NODE_COUNT, &args![path]).u32();
                if count <= 1 {
                    heading = actor_heading(e, this) as f32;
                } else {
                    let path = path_of(e, this);
                    let node = e.call(PATH_NODE, &args![path, count.wrapping_sub(2)]).u32();
                    let point = e.call(PLUS_4, &args![node]).u32();
                    e.call(COPY_POINT_AT_4, &args![point, first]);
                    let path = path_of(e, this);
                    let node = e.call(PATH_NODE, &args![path, count.wrapping_sub(1)]).u32();
                    let point = e.call(PLUS_4, &args![node]).u32();
                    e.call(COPY_POINT_AT_4, &args![point, second]);
                    e.call(POINT_SUBTRACT, &args![second, along, first]);
                    heading = e.call(Z_ANGLE_FROM_VECTOR, &args![along]).f64() as f32;
                }
            }
        }
        e.call(
            ACTOR_MOVE_TO_PATH_END,
            &args![
                actor,
                destination,
                x87_quiet(heading),
                cell,
                worldspace,
                1u32,
                0u32
            ],
        );
        reset_move_delta(e, this);
        e.set(this, H::eState, STATE_DONE);
        if at_end {
            5
        } else {
            6
        }
    })
}

// Translated from 009e20a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at +0x9F of a `PathingRequest`.
pub fn fn_009e20a0(e: &mut Engine, request: Ptr) -> bool {
    e.mem.u8(request.addr() + 0x9f) != 0
}

// Translated from 009e20c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetailedActorPathHandler::HandleDoorOpening` (Xbox PDB): the door state
/// machine (`eDoorState`). In state 0 it looks up the mover's detection door
/// (`ActorMover::DetectionDoorID`); a closed door (open state 3) that is in
/// the path and that the request does not ignore (byte +0x98) is activated
/// by the actor: state 1 and the door targeted, or state 2 when the
/// activation fails. In state 1 it waits for the door to open (open state
/// 1: back to 0; 2: returns 1). Returns the state (2 = blocked); 0 when
/// there is nothing to do.
pub fn detailed_actor_path_handler_handle_door_opening(e: &mut Engine, this: Ptr<H>) -> u32 {
    match e.get(this, H::eDoorState) {
        0 => {
            let mover = Ptr::<ActorMover>::new(mover_of(e, this));
            let door_id = e.get(mover, ActorMover::DetectionDoorID);
            let door = if door_id != 0 {
                e.call(LOOKUP_FORM, &args![door_id]).u32()
            } else {
                0
            };
            if door != 0 && e.vcall(door, DOOR_SLOT_F0, &[]).bool() {
                if e.call(GET_OPEN_STATE, &args![door]).u32() != 3 {
                    return 0;
                }
                let request = request_of(e, this);
                if !e.call(REQUEST_BYTE_98, &args![request]).bool() {
                    let id = e.call(FORM_DWORD_C, &args![door]).u32();
                    if detailed_actor_path_handler_is_door_in_path(e, this, id) {
                        let actor = e.get(this, H::pActor);
                        if e.call(REFERENCE_ACTIVATE, &args![door, actor, 0u32, 0u32, 1u32])
                            .bool()
                        {
                            e.set(this, H::pCurrentDoor, Ptr::new(door));
                            e.call(REFERENCE_SET_TARGETED, &args![door, 1u32]);
                            e.set(this, H::eDoorState, 1);
                        } else {
                            e.set(this, H::eDoorState, 2);
                        }
                    }
                }
                return e.get(this, H::eDoorState);
            }
            e.set(this, H::eDoorState, 0);
            0
        }
        1 => {
            let door = e.get(this, H::pCurrentDoor);
            if door.is_null() {
                e.set(this, H::eDoorState, 2);
                return 2;
            }
            match e.call(GET_OPEN_STATE, &args![door]).u32() {
                1 => {
                    e.set(this, H::pCurrentDoor, Ptr::NULL);
                    e.set(this, H::eDoorState, 0);
                    0
                }
                2 => 1,
                _ => 0,
            }
        }
        2 => 2,
        _ => {
            e.set(this, H::eDoorState, 0);
            0
        }
    }
}

/// Writes the triangle flag test of `IsDoorInPath` into `filter + 0x0A`:
/// whether the location's triangle has flag 0x1000.
fn door_triangle_flag(
    e: &mut Engine,
    location: Ptr,
    navmesh_holder: Ptr,
    triangle: Ptr,
    filter: Ptr,
) {
    if e.call(
        LOCATION_GET_NAVMESH_AND_TRIANGLE,
        &args![location, navmesh_holder, triangle],
    )
    .bool()
    {
        let index = e.mem.u16(triangle.addr()) as u32;
        let navmesh = e.call(NIPOINTER_GET, &args![navmesh_holder]).u32();
        let found = e.call(NAVMESH_GET_TRIANGLE, &args![navmesh, index]).u32();
        if found != 0 {
            let flag = e.call(TRIANGLE_HAS_FLAG, &args![found, 0x1000u32]).u8();
            e.mem.set_u8(filter.addr() + 0x0a, flag);
        }
    }
}

/// Any of the three flags of a `DoorInPathFilter` (+8, +9, +0xA).
fn door_filter_hit(e: &Engine, filter: Ptr) -> bool {
    e.mem.u8(filter.addr() + 8) != 0
        || e.mem.u8(filter.addr() + 9) != 0
        || e.mem.u8(filter.addr() + 0x0a) != 0
}

// Translated from 009e22d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009e22d0 - C++ exception frame (FS:[0]) and its unwinding not translated
/// `DetailedActorPathHandler::IsDoorInPath` (Xbox PDB): whether the door
/// with form ID `door_id` lies on the way. Without a path it checks the
/// line from the actor to the tether point; with one it asks the path
/// (`009e2660`) and then checks each half-node segment of the path with a
/// `DoorInPathFilter` and `Pathing::CheckLineOfSight`.
pub fn detailed_actor_path_handler_is_door_in_path(
    e: &mut Engine,
    this: Ptr<H>,
    door_id: u32,
) -> bool {
    let actor = e.get(this, H::pActor);
    if path_of(e, this) == 0 {
        return e.with_stack(0x40, |e, block| {
            let filter = block;
            let location = block.byte_add(0x0c);
            let navmesh_holder = block.byte_add(0x34);
            let triangle = block.byte_add(0x38);
            fn_009e2690(e, filter, door_id);
            e.call(LOCATION_FROM_REFERENCE, &args![location, actor]);
            e.call(LOCATION_RESOLVE_NAVMESH, &args![location, 0u32]);
            e.call(NIPOINTER_CONSTRUCT_NULL, &args![navmesh_holder]);
            door_triangle_flag(e, location, navmesh_holder, triangle, filter);
            e.call(
                CHECK_LINE_OF_SIGHT,
                &args![
                    location,
                    this.addr() + H::TetherTargetPoint.off,
                    0u32,
                    filter
                ],
            );
            let hit = door_filter_hit(e, filter);
            e.call(NIPOINTER_DESTRUCT, &args![navmesh_holder]);
            e.call(LOCATION_DESTRUCT, &args![location]);
            hit
        });
    }
    let path = path_of(e, this);
    if fn_009e2660(e, Ptr::new(path), door_id) {
        return true;
    }
    let mut t = 0.0f32;
    loop {
        let path = path_of(e, this);
        let count = e.call(PATH_NODE_COUNT, &args![path]).u32();
        if (t as f64) >= count as f64 - double(e, ONE_DOUBLE) || t.is_nan() {
            return false;
        }
        let next = (t as f64 + double(e, HALF)) as f32;
        let hit = e.with_stack(0x58, |e, block| {
            let start = block;
            let end = block.byte_add(0x0c);
            let filter = block.byte_add(0x18);
            let location = block.byte_add(0x24);
            let navmesh_holder = block.byte_add(0x4c);
            let triangle = block.byte_add(0x50);
            e.call(DEFAULT_CONSTRUCT, &args![start]);
            e.call(DEFAULT_CONSTRUCT, &args![end]);
            let path = path_of(e, this);
            e.call(PATH_POINT_AT, &args![path, x87_quiet(t), start]);
            let path = path_of(e, this);
            e.call(PATH_POINT_AT, &args![path, x87_quiet(next), end]);
            fn_009e2690(e, filter, door_id);
            e.call(LOCATION_FROM_POINT, &args![location, start, actor]);
            e.call(LOCATION_RESOLVE_NAVMESH, &args![location, 0u32]);
            e.call(NIPOINTER_CONSTRUCT_NULL, &args![navmesh_holder]);
            door_triangle_flag(e, location, navmesh_holder, triangle, filter);
            e.call(CHECK_LINE_OF_SIGHT, &args![location, end, 0u32, filter]);
            let hit = door_filter_hit(e, filter);
            e.call(NIPOINTER_DESTRUCT, &args![navmesh_holder]);
            e.call(LOCATION_DESTRUCT, &args![location]);
            hit
        });
        if hit {
            return true;
        }
        t = (t as f64 + double(e, HALF)) as f32;
    }
}

// Translated from 009e2660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the path's array at +0x30 holds `door_id`: searches it from
/// index 0 (`00719b20` with the compare function `009a3830` and the address
/// of the argument).
pub fn fn_009e2660(e: &mut Engine, path: Ptr, door_id: u32) -> bool {
    e.with_stack(4, |e, key| {
        e.mem.set_u32(key.addr(), door_id);
        let index = e
            .call(
                ARRAY_FIND,
                &args![path.addr() + 0x30, key, 0u32, ARRAY_COMPARE_009A3830],
            )
            .u32();
        index != u32::MAX
    })
}

// Translated from 009e2690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DoorInPathFilter` constructor: base constructor (`006cd650`), its
/// vtable `01092484`, the door's form ID at +4, flags +8 and +9 clear and
/// +0xA set.
pub fn fn_009e2690(e: &mut Engine, filter: Ptr, door_id: u32) -> Ptr {
    e.call(FILTER_BASE_CONSTRUCT, &args![filter]);
    let f = filter.addr();
    e.mem.set_u32(f, DOOR_IN_PATH_FILTER_VTABLE);
    e.mem.set_u32(f + 4, door_id);
    e.mem.set_u8(f + 8, 0);
    e.mem.set_u8(f + 9, 0);
    e.mem.set_u8(f + 0x0a, 1);
    filter
}

/// Whether the triangle of a navmesh is a closed door's (flag 0x1000) of
/// the filter's door (or any door when the filter has none).
fn edge_end_is_door(e: &mut Engine, filter: Ptr, navmesh: u32, triangle: u16) -> bool {
    let found = e
        .call(NAVMESH_GET_TRIANGLE, &args![navmesh, triangle as u32])
        .u32();
    if !e.call(TRIANGLE_HAS_FLAG, &args![found, 0x1000u32]).bool() {
        return false;
    }
    let info = e
        .call(NAVMESH_CLOSED_DOOR_INFO, &args![navmesh, triangle as u32])
        .u32();
    if info == 0 {
        return false;
    }
    let door = e.mem.u32(filter.addr() + 4);
    e.mem.u32(info) == door || door == 0
}

// Translated from 009e26d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DoorInPathFilter::CanCrossEdge` (Xbox PDB): looks at both ends of an
/// edge (navmesh and triangle each); clears flag +0xA unless both ends are
/// the door, sets +9 when only the first is and +8 when only the second is.
/// Always lets the edge be crossed.
#[allow(clippy::too_many_arguments)] // The exe ABI has six stack arguments and `this`.
pub fn door_in_path_filter_can_cross_edge(
    e: &mut Engine,
    this: Ptr,
    navmesh_a: u32,
    triangle_a: u16,
    _unused_3: u32,
    navmesh_b: u32,
    triangle_b: u16,
    _unused_6: u32,
) -> bool {
    let a = edge_end_is_door(e, this, navmesh_a, triangle_a);
    let b = edge_end_is_door(e, this, navmesh_b, triangle_b);
    let f = this.addr();
    if !a {
        e.mem.set_u8(f + 0x0a, 0);
    }
    if !b {
        e.mem.set_u8(f + 0x0a, 0);
    }
    if a && !b {
        e.mem.set_u8(f + 9, 1);
    }
    if !a && b {
        e.mem.set_u8(f + 8, 1);
    }
    true
}

// Translated from 009e27e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the tether distances from the request: `fTetherDistFromActor` is
/// the request's float at +0x68 (`PathingRequest::fActorRadius`) and
/// `fTetherMinMove` half of it (after storing the constructors' 10 and 20
/// first). Returns `false`.
pub fn fn_009e27e0(e: &mut Engine, this: Ptr<H>) -> bool {
    let min_move = fld(e, TETHER_MIN_MOVE);
    e.set(this, H::fTetherMinMove, min_move);
    let dist = fld(e, TETHER_DIST_FROM_ACTOR);
    e.set(this, H::fTetherDistFromActor, dist);
    let request = request_of(e, this);
    let radius = e.call(FLOAT_AT_68, &args![request]).f64() as f32;
    e.set(this, H::fTetherDistFromActor, radius);
    let half = (e.get(this, H::fTetherDistFromActor) as f64 * double(e, HALF)) as f32;
    e.set(this, H::fTetherMinMove, half);
    false
}

// Translated from 009e2830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetailedActorPathHandler::GetDistanceTraveled` (Xbox PDB):
/// `fDistanceTraveled`, 0 without a path.
pub fn detailed_actor_path_handler_get_distance_traveled(e: &mut Engine, this: Ptr<H>) -> f32 {
    if path_of(e, this) == 0 {
        0.0
    } else {
        x87_quiet(e.get(this, H::fDistanceTraveled))
    }
}

// Translated from 009e2860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetailedActorPathHandler::GetDistanceRemaining` (Xbox PDB): the path
/// length minus the distance traveled (slot 0x18), at least 0.
pub fn detailed_actor_path_handler_get_distance_remaining(e: &mut Engine, this: Ptr<H>) -> f32 {
    let traveled = e.vcall(this.addr(), SLOT_GET_DISTANCE_TRAVELED, &[]).f64();
    let remaining = (e.get(this, H::fPathLength) as f64 - traveled) as f32;
    let at_least = e
        .call(FLOAT_MAX_OF, &args![x87_quiet(remaining), 0.0f32])
        .f64() as f32;
    x87_quiet(at_least)
}

// Translated from 009e28a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetailedActorPathHandler::GetPercentTraveled` (Xbox PDB): distance
/// traveled over path length, clamped to 0..1 (`005330e0`); 1.0 when the
/// length is not positive.
pub fn detailed_actor_path_handler_get_percent_traveled(e: &mut Engine, this: Ptr<H>) -> f32 {
    if e.get(this, H::fPathLength) as f64 <= double(e, ZERO_DOUBLE) {
        return 1.0;
    }
    let traveled = e.vcall(this.addr(), SLOT_GET_DISTANCE_TRAVELED, &[]).f64();
    let fraction = (traveled / e.get(this, H::fPathLength) as f64) as f32;
    e.with_stack(4, |e, value| {
        e.mem.set_f32(value.addr(), fraction);
        e.call(FLOAT_CLAMP, &args![value, 0.0f32, 1.0f32]);
        fld(e, value.addr())
    })
}

// Translated from 009e2900 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009e2900 - C++ exception frame (FS:[0]) and its unwinding not translated
/// `DetailedActorPathHandler::ComputeApproximateLocation` (Xbox PDB): where
/// the actor will be after `seconds` along the path (`006e8790` from the
/// current spline parameter, the walk or run speed times `seconds`; run
/// when bit 0x200 of the mover's slot 0x1C is set), written to `result`.
/// Without a path, an empty one, or with the mover's byte +0x70 set, the
/// actor's location.
pub fn detailed_actor_path_handler_compute_approximate_location(
    e: &mut Engine,
    this: Ptr<H>,
    result: Ptr,
    seconds: f32,
) -> Ptr {
    let mover = mover_of(e, this);
    if e.call(MOVER_BYTE_70, &args![mover]).bool() {
        let location = actor_location(e, this);
        copy_point(e, result.addr(), location);
        return result;
    }
    e.with_stack(4, |e, holder| {
        e.call(NIPOINTER_COPY, &args![holder, this.addr() + H::spPath.off]);
        let path = e.call(NIPOINTER_GET, &args![holder]).u32();
        let has_nodes = path != 0 && {
            let path = e.call(NIPOINTER_GET, &args![holder]).u32();
            e.call(PATH_NODE_COUNT, &args![path]).u32() != 0
        };
        if has_nodes {
            let mode = e.vcall(mover, MOVER_SLOT_1C, &[]).u32();
            let actor = e.get(this, H::pActor);
            let speed = if mode & 0x200 != 0 {
                e.call(ACTOR_GET_RUN_SPEED, &args![actor]).f64() as f32
            } else {
                e.call(ACTOR_GET_WALK_SPEED, &args![actor]).f64() as f32
            };
            let distance = (speed as f64 * seconds as f64) as f32;
            let spline = fld(e, this.addr() + H::fCurrentSplineT.off);
            let path = e.call(NIPOINTER_GET, &args![holder]).u32();
            e.call(
                PATH_LOCATION_AFTER,
                &args![path, result, spline, x87_quiet(distance)],
            );
        } else {
            let location = actor_location(e, this);
            copy_point(e, result.addr(), location);
        }
        e.call(NIPOINTER_RELEASE, &args![holder]);
    });
    result
}

// Translated from 009e2a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetailedActorPathHandler::IsWaitingForAvoidance` (Xbox PDB):
/// `eAvoidState == 2`.
pub fn detailed_actor_path_handler_is_waiting_for_avoidance(e: &mut Engine, this: Ptr<H>) -> bool {
    e.get(this, H::eAvoidState) == 2
}

/// `GroupLoaded` on the actor's animation data (slot 0x1E4) for `group`.
fn actor_group_loaded(e: &mut Engine, this: Ptr<H>, group: u32) -> bool {
    let actor = e.get(this, H::pActor).addr();
    let animation = e.vcall(actor, ACTOR_SLOT_ANIMATION, &[]).u32();
    e.call(ANIMATION_GROUP_LOADED, &args![animation, group])
        .bool()
}

/// `|fabs(pitch) - pi/2|` as `009e2aa0` computes it (`00408840`, then
/// `_fabs` on the double).
fn pitch_off_vertical(e: &mut Engine, pitch: f32) -> f64 {
    let pitch_abs = e.call(FLOAT_ABS, &args![pitch]).f64();
    let off = pitch_abs - double(e, HALF_PI);
    e.call(DOUBLE_ABS, &args![off]).f64()
}

// Translated from 009e2aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Chooses the direction bits of `MoveMode` (low six bits) for this frame
/// and the walk/run blend (`eBlendRunning`, `fRunWalkTime`). With the
/// mover facing a target point (or `bIgnoreInitialRotation`) the bits come
/// from the turn between the actor's projected angle and the target: none
/// (the mover's own bits) under one degree, 2 above 135 degrees when the
/// animation group 4 is loaded, 1 under 45 degrees, otherwise 4 or 8 by the
/// turn's side (1 and `bIgnoreLookAtPoint` for a second when the actor's
/// slot 0x21C is set and group 5 or 6 is missing). Otherwise the bits are
/// the mover's (slot 0x1C). Always returns `true`.
pub fn fn_009e2aa0(e: &mut Engine, this: Ptr<H>, seconds: f32) -> bool {
    let actor = e.get(this, H::pActor);
    if e.call(ACTOR_STATE_IS_5, &args![actor]).bool()
        || e.call(ACTOR_STATE_IS_3, &args![actor]).bool()
    {
        e.set(this, H::MoveMode, 0);
        return true;
    }
    let old_mode = e.get(this, H::MoveMode);
    if e.get(this, H::bIgnoreLookAtPoint) {
        let timer = (e.get(this, H::fIgnoreLookAtPointTimer) as f64 - seconds as f64) as f32;
        e.set(this, H::fIgnoreLookAtPointTimer, timer);
        if (timer as f64) < double(e, ZERO_DOUBLE) {
            e.set(this, H::bIgnoreLookAtPoint, false);
            e.set(this, H::fTimeInAnim, 1.0);
        }
    }
    let mover = mover_of(e, this);
    let mut base = e.call(MOVER_MOVE_MODE_BASE, &args![mover]).u32();
    let mut direction = e.get(this, H::MoveMode) & 0x3f;
    if e.call(ACTOR_BYTE_14D, &args![actor]).bool()
        && e.vcall(actor.addr(), ACTOR_SLOT_21C, &[]).bool()
    {
        e.set(this, H::MoveMode, base | 1);
        return true;
    }

    let facing = !e.get(this, H::bIgnoreLookAtPoint)
        && (e.call(MOVER_FACE_TARGET_POINT, &args![mover]).bool()
            || e.get(this, H::bIgnoreInitialRotation));
    if facing {
        e.with_stack(0x18, |e, block| {
            let point = block;
            let projected = block.byte_add(0x0c);
            e.call(DEFAULT_CONSTRUCT, &args![point]);
            let target_angle = if e.call(MOVER_FACE_TARGET_POINT, &args![mover]).bool() {
                e.call(MOVER_GET_LOOK_AT_POINT, &args![mover, point]);
                let location = actor_location(e, this);
                e.call(POINT_SUBTRACT_ASSIGN, &args![point, location]);
                e.call(Z_ANGLE_FROM_VECTOR, &args![point]).f64() as f32
            } else if e.get(this, H::bIgnoreInitialRotationKeepActorHeading) {
                copy_point(e, point.addr(), NO_POINT);
                actor_heading(e, this) as f32
            } else if path_of(e, this) != 0 {
                let parameter = fld(e, this.addr() + H::fIgnoreInitialRotationTetherParam.off);
                let path = path_of(e, this);
                e.call(PATH_POINT_AT, &args![path, parameter, point]);
                let location = actor_location(e, this);
                e.call(POINT_SUBTRACT_ASSIGN, &args![point, location]);
                e.call(Z_ANGLE_FROM_VECTOR, &args![point]).f64() as f32
            } else {
                copy_point(e, point.addr(), NO_POINT);
                actor_heading(e, this) as f32
            };
            let tether = this.addr() + H::TetherTargetPoint.off;
            let (tx, ty, tz) = (
                e.mem.u32(tether),
                e.mem.u32(tether + 4),
                e.mem.u32(tether + 8),
            );
            let location = actor_location(e, this);
            let (lx, ly, lz) = (
                e.mem.u32(location),
                e.mem.u32(location + 4),
                e.mem.u32(location + 8),
            );
            e.call(
                ANGLE_TO_PROJECTED_POINT_BETWEEN,
                &args![projected, lx, ly, lz, tx, ty, tz],
            );
            let pitch = fld(e, projected.addr());
            let yaw = fld(e, projected.addr() + 8);
            let near_vertical = e.call(ACTOR_BYTE_14D, &args![actor]).bool() && {
                let off = pitch_off_vertical(e, pitch);
                off < double(e, FIFTEEN_DEGREES)
            };
            let angle = if near_vertical {
                x87_quiet(target_angle)
            } else {
                yaw
            };
            let side = block.byte_add(0x14);
            e.mem.set_f32(side.addr(), 1.0);
            let angle = e.call(CLAMP_ANGLE, &args![angle]).f64() as f32;
            let turn = e
                .call(
                    ANGLE_DIFFERENCE,
                    &args![x87_quiet(angle), x87_quiet(target_angle), side],
                )
                .f64() as f32;
            if e.get(this, H::fTimeInAnim) as f64 > double(e, QUARTER) {
                let turn_abs = e.call(FLOAT_ABS, &args![x87_quiet(turn)]).f64();
                if turn_abs > double(e, DEGREES_TO_RADIANS) {
                    let degrees = (turn as f64 * double(e, RADIANS_TO_DEGREES)) as f32;
                    let mut chosen = None;
                    let degrees_abs = e.call(FLOAT_ABS, &args![x87_quiet(degrees)]).f64();
                    if degrees_abs > double(e, LARGE_TURN) && actor_group_loaded(e, this, 4) {
                        let off = pitch_off_vertical(e, pitch);
                        let backwards = if off >= double(e, FIFTEEN_DEGREES) {
                            true
                        } else {
                            !e.call(ACTOR_BYTE_14D, &args![actor]).bool()
                        };
                        if backwards {
                            chosen = Some(2);
                        }
                    }
                    direction = match chosen {
                        Some(bits) => bits,
                        None => {
                            let degrees_abs = e.call(FLOAT_ABS, &args![x87_quiet(degrees)]).f64();
                            if degrees_abs > double(e, SMALL_TURN) {
                                let side_value = e.mem.f32(side.addr());
                                let (group, bits) = if side_value as f64 > double(e, ZERO_DOUBLE) {
                                    (5, 4)
                                } else {
                                    (6, 8)
                                };
                                if e.vcall(actor.addr(), ACTOR_SLOT_21C, &[]).bool()
                                    && !actor_group_loaded(e, this, group)
                                {
                                    e.set(this, H::bIgnoreLookAtPoint, true);
                                    e.set(this, H::fIgnoreLookAtPointTimer, 1.0);
                                    1
                                } else {
                                    bits
                                }
                            } else {
                                1
                            }
                        }
                    };
                } else {
                    direction = e.vcall(mover, MOVER_SLOT_1C, &[]).u32() & 0x3f;
                }
            } else {
                let time = (e.get(this, H::fTimeInAnim) as f64 + seconds as f64) as f32;
                e.set(this, H::fTimeInAnim, time);
            }
        });
    } else {
        direction = e.vcall(mover, MOVER_SLOT_1C, &[]).u32() & 0x3f;
    }

    let blend = match e.get(this, H::eBlendRunning) {
        1 => true,
        2 => {
            e.set(this, H::eBlendRunning, 3);
            false
        }
        3 => {
            let start = base & 0x200 != 0
                && (old_mode & 0x200 == 0 || (old_mode & 0x30 != 0 && direction & 0x30 == 0));
            if start {
                e.set(this, H::eBlendRunning, 1);
                let duration = e
                    .call(ACTOR_GET_ANIM_GROUP_DURATION, &args![actor, 3u32])
                    .f64();
                e.set(this, H::fRunWalkTime, (duration * double(e, HALF)) as f32);
            }
            start
        }
        _ => false,
    };
    if blend {
        base = (base & !0x200) | 0x100;
        let time = (e.get(this, H::fRunWalkTime) as f64 - seconds as f64) as f32;
        e.set(this, H::fRunWalkTime, time);
        if (time as f64) < double(e, ZERO_DOUBLE) {
            e.set(this, H::eBlendRunning, 2);
            e.set(this, H::fRunWalkTime, 0.0);
        }
    }
    e.set(this, H::MoveMode, direction | base);
    true
}

// Translated from 009e3110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetailedActorPathHandler::ComputePathPassesThroughPlayersArea` (Xbox
/// PDB): whether a node after the current one lies in the player's cell
/// (or, when the player has no cell and `TES` has no interior cell, the
/// player's or `TES`'s worldspace), directly or through the reference of a
/// node whose flags have both low bits set (`+0x38`, `00568e50`).
pub fn detailed_actor_path_handler_compute_path_passes_through_players_area(
    e: &mut Engine,
    this: Ptr<H>,
) -> bool {
    let path = path_of(e, this);
    if path == 0 {
        return false;
    }
    let path = path_of(e, this);
    if !e.call(PATH_IS_USABLE, &args![path]).bool() {
        return false;
    }
    let path = path_of(e, this);
    let count = e.call(PATH_NODE_COUNT, &args![path]).u32();
    let mut index = e.get(this, H::iCurrentNodeIndex).wrapping_add(1);
    let mut by_cell = true;
    let player: u32 = e.global(PLAYER);
    let mut cell = e.call(REFERENCE_PARENT_CELL, &args![player]).u32();
    let mut worldspace = e.call(REFERENCE_GET_WORLDSPACE, &args![player]).u32();
    if cell == 0 {
        let tes: u32 = e.global(TES);
        cell = e.call(TES_INTERIOR_CELL, &args![tes]).u32();
        if cell == 0 {
            by_cell = false;
        }
    }
    if !by_cell && worldspace == 0 {
        let tes: u32 = e.global(TES);
        worldspace = e.call(TES_GET_WORLDSPACE, &args![tes]).u32();
        if worldspace == 0 {
            return false;
        }
    }
    while index < count {
        let path = path_of(e, this);
        let node = e.call(PATH_NODE, &args![path, index]).u32();
        index = index.wrapping_add(1);
        if node == 0 {
            continue;
        }
        let location = e.call(PLUS_4, &args![node]).u32();
        if by_cell {
            if cell == e.call(LOCATION_GET_CELL, &args![location]).u32() {
                return true;
            }
        } else if worldspace == e.call(LOCATION_GET_WORLDSPACE, &args![location]).u32() {
            return true;
        }
        if !e.call(FLAGS_BOTH_LOW, &args![node]).bool() {
            continue;
        }
        let linked = fn_009e32d0(e, Ptr::new(node));
        if linked == 0 {
            continue;
        }
        let holder = e.call(LINKED_REFERENCE, &args![linked]).u32();
        if holder == 0 {
            continue;
        }
        let reference = e.call(NIPOINTER_GET, &args![holder]).u32();
        if reference == 0 {
            continue;
        }
        if by_cell {
            if cell == e.call(REFERENCE_PARENT_CELL, &args![reference]).u32() {
                return true;
            }
        } else if worldspace == e.call(REFERENCE_GET_WORLDSPACE, &args![reference]).u32() {
            return true;
        }
    }
    false
}

// Translated from 009e32d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The dword at +0x38 (a shared getter body; this unit's path nodes keep a
/// pointer there).
pub fn fn_009e32d0(e: &mut Engine, object: Ptr) -> u32 {
    e.mem.u32(object.addr() + 0x38)
}

// Translated from 009e32f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetailedActorPathHandler::GetMoveDelta` (Xbox PDB): when a delta was
/// computed this frame, hands out `MoveDelta`, clears the flag and returns
/// `true`; otherwise the "no point" value and `false`.
pub fn detailed_actor_path_handler_get_move_delta(
    e: &mut Engine,
    this: Ptr<H>,
    result: Ptr,
) -> bool {
    if e.get(this, H::bDeltaComputedThisFrame) {
        e.set(this, H::bDeltaComputedThisFrame, false);
        copy_point(e, result.addr(), this.addr() + H::MoveDelta.off);
        true
    } else {
        copy_point(e, result.addr(), NO_POINT);
        false
    }
}

// Translated from 009e3360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetailedActorPathHandler::ForceMoveMode` (Xbox PDB): sets `MoveMode`.
pub fn detailed_actor_path_handler_force_move_mode(e: &mut Engine, this: Ptr<H>, mode: u32) {
    e.set(this, H::MoveMode, mode);
}

/// Copies a float field from another handler through the FPU.
fn copy_float_field(e: &mut Engine, this: Ptr<H>, other: Ptr<H>, field: Field<H, f32>) {
    let value = fld(e, other.addr() + field.off);
    e.set(this, field, value);
}

// Translated from 009e3380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetailedActorPathHandler::HandlePreviousPathTransition` (Xbox PDB):
/// when the previous handler is a detailed one (slot 0x34), takes over its
/// move mode, run blend, avoidance state (waiting stays waiting; state 3
/// becomes 4 and moves its avoid list over), look-at timer and, when it was
/// waiting for a path update, its tether points.
pub fn detailed_actor_path_handler_handle_previous_path_transition(
    e: &mut Engine,
    this: Ptr<H>,
    previous: Ptr<H>,
) {
    if previous.is_null() || !e.vcall(previous.addr(), SLOT_IS_DETAILED, &[]).bool() {
        return;
    }
    let mode = e.get(previous, H::MoveMode);
    e.set(this, H::MoveMode, mode);
    let blend = e.get(previous, H::eBlendRunning);
    e.set(this, H::eBlendRunning, blend);
    copy_float_field(e, this, previous, H::fRunWalkTime);
    copy_float_field(e, this, previous, H::fTimeInAnim);
    copy_float_field(e, this, previous, H::fAvoidanceDetectionValue);
    match e.get(previous, H::eAvoidState) {
        2 => {
            e.set(this, H::eAvoidState, 2);
            copy_float_field(e, this, previous, H::fActorAvoidanceTimer);
            copy_float_field(e, this, previous, H::fWaitTime);
        }
        3 => {
            e.set(this, H::eAvoidState, 4);
            copy_float_field(e, this, previous, H::fActorAvoidanceTimer);
            e.set(this, H::fWaitTime, 0.0);
            let from = previous.addr() + H::ActorAvoidList.off;
            let to = this.addr() + H::ActorAvoidList.off;
            loop {
                let list = e.call(DEFAULT_CONSTRUCT, &args![from]).u32();
                if e.mem.u32(list) == 0 {
                    break;
                }
                let item = e.call(DEFAULT_CONSTRUCT, &args![from]).u32();
                e.call(LIST_PUSH_FRONT, &args![to, item]);
                e.call(LIST_POP_FRONT, &args![from]);
            }
        }
        _ => {}
    }
    let ignore = e.get(previous, H::bIgnoreLookAtPoint);
    e.set(this, H::bIgnoreLookAtPoint, ignore);
    copy_float_field(e, this, previous, H::fIgnoreLookAtPointTimer);
    if e.get(previous, H::bWaitingForUpdate) {
        e.set(this, H::bWaitingForUpdate, true);
        copy_point(
            e,
            this.addr() + H::LastTetherPoint.off,
            previous.addr() + H::LastTetherPoint.off,
        );
        copy_point(
            e,
            this.addr() + H::TetherTargetPoint.off,
            previous.addr() + H::TetherTargetPoint.off,
        );
    }
}

// Translated from 009e3520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unless `008b7100` handles `MoveDelta` for the actor, sets the delta's y
/// to the actor's current speed times `seconds`.
pub fn fn_009e3520(e: &mut Engine, this: Ptr<H>, seconds: f32) {
    let actor = e.get(this, H::pActor);
    if !e
        .call(
            ACTOR_COMPUTE_MOVE_DELTA,
            &args![actor, this.addr() + H::MoveDelta.off],
        )
        .bool()
    {
        let speed = e.call(ACTOR_GET_CURRENT_SPEED, &args![actor]).f64();
        let y = (speed * seconds as f64) as f32;
        e.mem.set_f32(this.addr() + H::MoveDelta.off + 4, y);
    }
}

// Translated from 009e3560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Turns `MoveDelta` (forward along y) into the direction of the tether
/// point (or the blocked target point while blocked). On land: rotated about
/// z by the angle to the point minus the heading and the move direction's
/// offset (`MoveMode` slot 0x48: 2 behind, 4 and 8 to the sides). Swimming
/// (actor byte +0x14D): pitched about x towards the point, the point kept
/// 5 units under the water surface. The word it takes is not read.
pub fn fn_009e3560(e: &mut Engine, this: Ptr<H>, _unused_1: u32) {
    let actor = e.get(this, H::pActor);
    if e.call(ACTOR_BYTE_14D, &args![actor]).bool() {
        e.with_stack(0x54, |e, block| {
            let point = block;
            let position = block.byte_add(0x0c);
            let matrix = block.byte_add(0x18);
            let rotated = block.byte_add(0x3c);
            let tether = this.addr() + H::TetherTargetPoint.off;
            copy_point(e, point.addr(), tether);
            let water = e.call(REFERENCE_WATER_HEIGHT, &args![actor]).f64() as f32;
            if water as f64 != double(e, NO_WATER_HEIGHT) && water < e.mem.f32(tether + 8) {
                let height = e.call(REFERENCE_WATER_HEIGHT, &args![actor]).f64();
                e.mem
                    .set_f32(tether + 8, (height - double(e, UNDER_WATER)) as f32);
            }
            let controller = e.call(ACTOR_GET_CHAR_CONTROLLER, &args![actor]).u32();
            let actor_position = e.call(REFERENCE_POSITION, &args![actor]).u32();
            copy_point(e, position.addr(), actor_position);
            if controller != 0 {
                let half_height = e.call(ACTOR_GET_HEIGHT, &args![actor]).f64() * double(e, HALF);
                let factor = fn_009e3970(e, Ptr::new(controller)) as f64;
                let z = (factor * half_height + e.mem.f32(position.addr() + 8) as f64) as f32;
                e.mem.set_f32(position.addr() + 8, z);
                e.call(POINT_SUBTRACT_ASSIGN, &args![point, position]);
            } else {
                let location = actor_location(e, this);
                e.call(POINT_SUBTRACT_ASSIGN, &args![point, location]);
            }
            let height = e.call(ACTOR_GET_HEIGHT, &args![actor]).f64();
            let position_z = e.mem.f32(position.addr() + 8);
            let tether_z = e.mem.f32(tether + 8);
            if tether_z as f64 >= position_z as f64 - height && position_z > tether_z {
                e.mem.set_f32(point.addr() + 8, 0.0);
            }
            e.call(DEFAULT_CONSTRUCT, &args![matrix]);
            e.call(POINT_NORMALIZE, &args![point]);
            let minus_one = fld(e, MINUS_ONE_FLOAT);
            e.call(FLOAT_CLAMP, &args![point.addr() + 8, minus_one, 1.0f32]);
            let z = fld(e, point.addr() + 8);
            let pitch = if z as f64 >= double(e, ZERO_DOUBLE) {
                let angle = e.call(TRIG_004F6E40, &args![z]).f64() as f32;
                (double(e, HALF_PI) - angle as f64) as f32
            } else {
                let angle = e.call(TRIG_004F6E40, &args![-z]).f64() as f32;
                (angle as f64 - double(e, HALF_PI)) as f32
            };
            let delta_y = e.mem.f32(this.addr() + H::MoveDelta.off + 4);
            let pitch =
                if (delta_y as f64) < double(e, ZERO_DOUBLE) && e.get(this, H::MoveMode) & 2 != 0 {
                    -pitch
                } else {
                    pitch
                };
            e.call(MATRIX_MAKE_X_ROTATION, &args![matrix, pitch]);
            let result = e
                .call(
                    POINT_TIMES_MATRIX,
                    &args![rotated, this.addr() + H::MoveDelta.off, matrix],
                )
                .u32();
            copy_point(e, this.addr() + H::MoveDelta.off, result);
        });
        return;
    }
    e.with_stack(0x3c, |e, block| {
        let point = block;
        let matrix = block.byte_add(0x0c);
        let rotated = block.byte_add(0x30);
        let source = if e.get(this, H::bMovementBlocked) {
            this.addr() + H::BlockedTargetPoint.off
        } else {
            this.addr() + H::TetherTargetPoint.off
        };
        copy_point(e, point.addr(), source);
        let location = actor_location(e, this);
        e.call(POINT_SUBTRACT_ASSIGN, &args![point, location]);
        e.mem.set_f32(point.addr() + 8, 0.0);
        let angle = e.call(Z_ANGLE_FROM_VECTOR, &args![point]).f64() as f32;
        let mode = e.vcall(this.addr(), SLOT_GET_MOVE_MODE, &[]).u32() & 0xf;
        // The compiler's jump table at 009e3940.
        let offset = match mode {
            2 => fld(e, PI_FLOAT),
            4 => fld(e, ANGLE_LEFT),
            8 => fld(e, ANGLE_RIGHT),
            _ => 0.0,
        };
        let heading = actor_heading(e, this);
        let facing = (heading + offset as f64) as f32;
        let turn = (angle as f64 - facing as f64) as f32;
        e.call(DEFAULT_CONSTRUCT, &args![matrix]);
        fn_009e1640(e, matrix, x87_quiet(turn));
        let result = e
            .call(
                MATRIX_TIMES_POINT,
                &args![matrix, rotated, this.addr() + H::MoveDelta.off],
            )
            .u32();
        copy_point(e, this.addr() + H::MoveDelta.off, result);
    });
}

// Translated from 009e3970 (decompiled, FalloutNV.exe 1.4.0.525)
/// The float at +0x560 of a character controller.
pub fn fn_009e3970(e: &mut Engine, controller: Ptr) -> f32 {
    fld(e, controller.addr() + 0x560)
}

// Translated from 009e3990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetailedActorPathHandler::GetNextTetherPoint` (Xbox PDB): copies
/// `TetherTargetPoint` to `result`; returns `true`.
pub fn detailed_actor_path_handler_get_next_tether_point(
    e: &mut Engine,
    this: Ptr<H>,
    result: Ptr,
) -> bool {
    copy_point(e, result.addr(), this.addr() + H::TetherTargetPoint.off);
    true
}

// Translated from 009e39c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetailedActorPathHandler::ClearMovement` (Xbox PDB): clears the delta,
/// resets the move mode to the mover's, drops path and request, the door
/// and the avoid list; state 5.
pub fn detailed_actor_path_handler_clear_movement(e: &mut Engine, this: Ptr<H>) {
    copy_point(e, this.addr() + H::MoveDelta.off, NO_POINT);
    let mover = mover_of(e, this);
    let mode = e.call(MOVER_MOVE_MODE_BASE, &args![mover]).u32();
    e.set(this, H::MoveMode, mode);
    e.set(this, H::fDistanceTraveled, 0.0);
    e.call(NIPOINTER_ASSIGN, &args![this.addr() + H::spPath.off, 0u32]);
    e.call(
        NIPOINTER_ASSIGN,
        &args![this.addr() + H::spRequest.off, 0u32],
    );
    e.set(this, H::bWaitingForUpdate, false);
    e.set(this, H::eDoorState, 0);
    e.set(this, H::pCurrentDoor, Ptr::NULL);
    e.set(this, H::eAvoidState, 1);
    e.call(LIST_CLEAR, &args![this.addr() + H::ActorAvoidList.off]);
    e.set(this, H::eState, STATE_DONE);
}

// Translated from 009e3a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetailedActorPathHandler::GetNextNodeLocation` (Xbox PDB): the
/// location of the node after the current one, of the path's end node when
/// the current one is the last, or the actor's location without a path or
/// nodes; written to `result`, which is returned.
pub fn detailed_actor_path_handler_get_next_node_location(
    e: &mut Engine,
    this: Ptr<H>,
    result: Ptr,
) -> Ptr {
    let has_nodes = path_of(e, this) != 0 && {
        let path = path_of(e, this);
        e.call(PATH_NODE_COUNT, &args![path]).u32() != 0
    };
    if !has_nodes {
        let location = actor_location(e, this);
        copy_point(e, result.addr(), location);
        return result;
    }
    let next = e.get(this, H::iCurrentNodeIndex).wrapping_add(1);
    let path = path_of(e, this);
    let count = e.call(PATH_NODE_COUNT, &args![path]).u32();
    let node = if next < count {
        let next = e.get(this, H::iCurrentNodeIndex).wrapping_add(1);
        let path = path_of(e, this);
        e.call(PATH_NODE, &args![path, next]).u32()
    } else {
        let path = path_of(e, this);
        e.call(PATH_END_NODE, &args![path]).u32()
    };
    let location = e.call(PLUS_4, &args![node]).u32();
    e.call(COPY_POINT_AT_4, &args![location, result]);
    result
}

// Translated from 009e3b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetailedActorPathHandler::GetNodeLocation` (Xbox PDB): the location of
/// node `index` into `result`; `false` without a path or for an index past
/// the end.
pub fn detailed_actor_path_handler_get_node_location(
    e: &mut Engine,
    this: Ptr<H>,
    index: u32,
    result: Ptr,
) -> bool {
    if path_of(e, this) == 0 {
        return false;
    }
    let path = path_of(e, this);
    if index >= e.call(PATH_NODE_COUNT, &args![path]).u32() {
        return false;
    }
    e.with_stack(12, |e, local| {
        let path = path_of(e, this);
        let node = e.call(PATH_NODE, &args![path, index]).u32();
        let location = e.call(PLUS_4, &args![node]).u32();
        let copied = e.call(COPY_POINT_AT_4, &args![location, local]).u32();
        copy_point(e, result.addr(), copied);
    });
    true
}

// Translated from 009e3be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetailedActorPathHandler::IsMovementBlocked` (Xbox PDB):
/// `bMovementBlocked`.
pub fn detailed_actor_path_handler_is_movement_blocked(e: &mut Engine, this: Ptr<H>) -> bool {
    e.get(this, H::bMovementBlocked)
}

// Translated from 009e3c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetailedActorPathHandler::UpdateTetherPosition` (Xbox PDB): unless the
/// current node is the last, records the distance traveled at the current
/// spline parameter (`006e84b0`), keeps the old tether point, and moves the
/// tether `distance` along the path (`006e7e70`; backwards for a negative
/// distance), updating the spline parameter, the tether point and the node
/// index (the parameter truncated).
pub fn detailed_actor_path_handler_update_tether_position(
    e: &mut Engine,
    this: Ptr<H>,
    distance: f32,
) {
    let path = path_of(e, this);
    if e.call(PATH_NODE_COUNT, &args![path]).u32() == 0 {
        return;
    }
    let next = e.get(this, H::iCurrentNodeIndex).wrapping_add(1);
    let path = path_of(e, this);
    if next >= e.call(PATH_NODE_COUNT, &args![path]).u32() {
        return;
    }
    let spline = fld(e, this.addr() + H::fCurrentSplineT.off);
    let path = path_of(e, this);
    let traveled = e.call(PATH_DISTANCE_AT, &args![path, spline]).f64() as f32;
    e.set(this, H::fDistanceTraveled, traveled);
    copy_point(
        e,
        this.addr() + H::LastTetherPoint.off,
        this.addr() + H::TetherTargetPoint.off,
    );
    fn_009e5480(e, this);
    e.with_stack(16, |e, block| {
        let point = block;
        let parameter = block.byte_add(12);
        e.call(DEFAULT_CONSTRUCT, &args![point]);
        let spline = fld(e, this.addr() + H::fCurrentSplineT.off);
        let (amount, forward) = if distance as f64 >= double(e, ZERO_DOUBLE) {
            (x87_quiet(distance), 1u32)
        } else {
            (-x87_quiet(distance), 0u32)
        };
        let path = path_of(e, this);
        e.call(
            PATH_ADVANCE,
            &args![path, spline, amount, forward, parameter, point],
        );
        let new_spline = fld(e, parameter.addr());
        e.set(this, H::fCurrentSplineT, new_spline);
        copy_point(e, this.addr() + H::TetherTargetPoint.off, point.addr());
    });
    let spline = e.get(this, H::fCurrentSplineT);
    let index = e.call(FTOL, &args![spline as f64]).u32();
    e.set(this, H::iCurrentNodeIndex, index);
    let mover = mover_of(e, this);
    e.call(EMPTY_FUNCTION, &args![mover]);
}

/// `009e3d50`'s measures for the current tether point: (dot of the
/// heading-rotated delta with the flat vector from the actor to the tether
/// point, dot of the path's direction just ahead with it, its squared flat
/// length). `block` holds the locals: rotated delta +0x00, vector +0x0C,
/// point ahead +0x18, temporary +0x24.
fn tether_measures(e: &mut Engine, this: Ptr<H>, block: Ptr, first: bool) -> (f32, f32, f32) {
    let direction = block;
    let to_tether = block.byte_add(0x0c);
    let ahead = block.byte_add(0x18);
    let temporary = block.byte_add(0x24);
    let tether = this.addr() + H::TetherTargetPoint.off;
    if first {
        let location = actor_location(e, this);
        e.call(POINT_SUBTRACT, &args![tether, to_tether, location]);
        e.call(DEFAULT_CONSTRUCT, &args![ahead]);
    }
    let parameter = (e.get(this, H::fCurrentSplineT) as f64 + double(e, SPLINE_STEP)) as f32;
    let path = path_of(e, this);
    e.call(PATH_POINT_AT, &args![path, parameter, ahead]);
    e.call(POINT_SUBTRACT_ASSIGN, &args![ahead, tether]);
    if !first {
        let location = actor_location(e, this);
        let result = e
            .call(POINT_SUBTRACT, &args![tether, temporary, location])
            .u32();
        copy_point(e, to_tether.addr(), result);
    }
    e.mem.set_f32(to_tether.addr() + 8, 0.0);
    let along = e.call(POINT_DOT, &args![direction, to_tether]).f64() as f32;
    let ahead_dot = e.call(POINT_DOT, &args![ahead, to_tether]).f64() as f32;
    let distance = e.call(POINT_LENGTH_SQUARED_XY, &args![to_tether]).f64() as f32;
    (along, ahead_dot, distance)
}

/// The tether point set to the path's end node location.
fn tether_to_path_end(e: &mut Engine, this: Ptr<H>) {
    e.with_stack(12, |e, local| {
        let path = path_of(e, this);
        let node = e.call(PATH_END_NODE, &args![path]).u32();
        let location = e.call(PLUS_4, &args![node]).u32();
        let copied = e.call(COPY_POINT_AT_4, &args![location, local]).u32();
        copy_point(e, this.addr() + H::TetherTargetPoint.off, copied);
    });
}

/// The node count minus one as the x87 loads it (`FILD` of the unsigned
/// count less one).
fn last_node_parameter(e: &mut Engine, this: Ptr<H>) -> f64 {
    let path = path_of(e, this);
    let count = e.call(PATH_NODE_COUNT, &args![path]).u32();
    count.wrapping_sub(1) as f64
}

// Translated from 009e3d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves the tether point ahead along the path in steps of 0.1 of the
/// spline parameter while the actor has reached or passed it (the move
/// direction points away from it, the path ahead turns back, or it is
/// closer than twice the delta's length or `fTetherMinMove`); at the last
/// node the tether point is the path's end. Nothing happens while the move
/// direction bits disagree with `MoveDelta`. The word it takes is not read.
pub fn fn_009e3d50(e: &mut Engine, this: Ptr<H>, _unused_1: u32) {
    let path = path_of(e, this);
    if e.call(PATH_NODE_COUNT, &args![path]).u32() == 0 {
        return;
    }
    let path = path_of(e, this);
    if e.call(PATH_NODE_COUNT, &args![path]).u32() != 0 {
        let next = e.get(this, H::iCurrentNodeIndex).wrapping_add(1);
        let path = path_of(e, this);
        if next >= e.call(PATH_NODE_COUNT, &args![path]).u32() {
            tether_to_path_end(e, this);
            return;
        }
    }
    let mode = e.get(this, H::MoveMode);
    let zero = double(e, ZERO_DOUBLE);
    let delta = this.addr() + H::MoveDelta.off;
    if mode & 1 != 0 && (e.mem.f32(delta + 4) as f64) < zero {
        return;
    }
    if mode & 2 != 0 && e.mem.f32(delta + 4) as f64 > zero {
        return;
    }
    if mode & 4 != 0 && e.mem.f32(delta) as f64 > zero {
        return;
    }
    if mode & 8 != 0 && (e.mem.f32(delta) as f64) < zero {
        return;
    }
    e.with_stack(0x54, |e, block| {
        let direction = block;
        let matrix = block.byte_add(0x30);
        e.call(DEFAULT_CONSTRUCT, &args![matrix]);
        let heading = actor_heading(e, this) as f32;
        fn_009e1640(e, matrix, heading);
        e.call(MATRIX_TIMES_POINT, &args![matrix, direction, delta]);
        let (mut along, mut ahead, mut distance) = tether_measures(e, this, block, true);
        let length = e.call(POINT_LENGTH_SQUARED, &args![delta]).f64();
        let limit = (length * double(e, FOUR)) as f32;
        let min_move = e.get(this, H::fTetherMinMove);
        let min_move_squared = (min_move as f64 * min_move as f64) as f32;
        let limit = e
            .call(
                FLOAT_MAX_OF,
                &args![x87_quiet(limit), x87_quiet(min_move_squared)],
            )
            .f64() as f32;
        let advance = (along as f64) < zero || (ahead as f64) < zero || limit > distance;
        if !advance {
            return;
        }
        let spline = fld(e, this.addr() + H::fCurrentSplineT.off);
        let path = path_of(e, this);
        let traveled = e.call(PATH_DISTANCE_AT, &args![path, spline]).f64() as f32;
        e.set(this, H::fDistanceTraveled, traveled);
        copy_point(
            e,
            this.addr() + H::LastTetherPoint.off,
            this.addr() + H::TetherTargetPoint.off,
        );
        let last = last_node_parameter(e, this);
        if e.get(this, H::fCurrentSplineT) as f64 >= last {
            let last = last_node_parameter(e, this);
            e.set(this, H::fCurrentSplineT, last as f32);
            let spline = e.get(this, H::fCurrentSplineT);
            let index = e.call(FTOL, &args![spline as f64]).u32();
            e.set(this, H::iCurrentNodeIndex, index);
            tether_to_path_end(e, this);
            return;
        }
        let mut previous = x87_quiet(along);
        loop {
            let spline = (e.get(this, H::fCurrentSplineT) as f64 + double(e, SPLINE_STEP)) as f32;
            e.set(this, H::fCurrentSplineT, spline);
            let path = path_of(e, this);
            e.call(
                PATH_POINT_AT,
                &args![
                    path,
                    x87_quiet(spline),
                    this.addr() + H::TetherTargetPoint.off
                ],
            );
            (along, ahead, distance) = tether_measures(e, this, block, false);
            let last = last_node_parameter(e, this);
            if e.get(this, H::fCurrentSplineT) as f64 >= last {
                let last = last_node_parameter(e, this);
                e.set(this, H::fCurrentSplineT, last as f32);
                break;
            }
            let stop = if (along as f64) < zero || (ahead as f64) < zero {
                previous > along
            } else {
                limit <= distance
            };
            if stop {
                break;
            }
            previous = x87_quiet(along);
        }
        let spline = e.get(this, H::fCurrentSplineT);
        let index = e.call(FTOL, &args![spline as f64]).u32();
        e.set(this, H::iCurrentNodeIndex, index);
        let mover = mover_of(e, this);
        e.call(EMPTY_FUNCTION, &args![mover]);
    });
}

// Translated from 009e4450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetailedActorPathHandler::ComputeHeadingAndLooking` (Xbox PDB): the
/// heading the actor should face and the looking (pitch) angle, written to
/// `heading_out` and `looking_out`. Blocked: `fAlternateFacingAngle`.
/// Ignoring the initial rotation: the actor's heading, or towards the path
/// point at `fIgnoreInitialRotationTetherParam`. Facing the mover's target
/// point: towards it (the projected angle under 100 units of radius; the
/// actor's heading when it is within a quarter of the radius squared), with
/// its pitch unless swimming. Otherwise towards the path point 0.1 ahead
/// (or the tether point), keeping the heading when swimming nearly
/// vertically.
pub fn detailed_actor_path_handler_compute_heading_and_looking(
    e: &mut Engine,
    this: Ptr<H>,
    heading_out: Ptr,
    looking_out: Ptr,
) {
    let actor = e.get(this, H::pActor);
    let (heading, looking) = if e.get(this, H::bMovementBlocked) {
        (fld(e, this.addr() + H::fAlternateFacingAngle.off), 0.0)
    } else if e.get(this, H::bIgnoreInitialRotation) {
        let heading = if e.get(this, H::bIgnoreInitialRotationKeepActorHeading) {
            actor_heading(e, this) as f32
        } else if path_of(e, this) != 0 {
            e.with_stack(0x18, |e, block| {
                let point = block;
                let vector = block.byte_add(0x0c);
                e.call(DEFAULT_CONSTRUCT, &args![point]);
                let parameter = fld(e, this.addr() + H::fIgnoreInitialRotationTetherParam.off);
                let path = path_of(e, this);
                e.call(PATH_POINT_AT, &args![path, parameter, point]);
                let location = actor_location(e, this);
                e.call(POINT_SUBTRACT, &args![point, vector, location]);
                e.mem.set_f32(vector.addr() + 8, 0.0);
                e.call(Z_ANGLE_FROM_VECTOR, &args![vector]).f64() as f32
            })
        } else {
            actor_heading(e, this) as f32
        };
        (heading, 0.0)
    } else if !e.get(this, H::bIgnoreLookAtPoint) && {
        let mover = mover_of(e, this);
        e.call(MOVER_FACE_TARGET_POINT, &args![mover]).bool()
    } {
        e.with_stack(0x48, |e, block| {
            let target = block;
            let firing = block.byte_add(0x0c);
            let difference = block.byte_add(0x18);
            let projected = block.byte_add(0x24);
            let vector = block.byte_add(0x30);
            let mut looking = 0.0f32;
            e.call(DEFAULT_CONSTRUCT, &args![target]);
            let mover = mover_of(e, this);
            e.call(MOVER_GET_LOOK_AT_POINT, &args![mover, target]);
            e.call(FIRING_LOCATION, &args![firing, actor]);
            let offset = e
                .call(POINT_SUBTRACT, &args![firing, difference, target])
                .u32();
            let distance_squared = e.call(POINT_LENGTH_SQUARED_XY, &args![offset]).f64();
            let radius_a = e.call(ACTOR_GET_RADIUS, &args![actor]).f64();
            let radius_b = e.call(ACTOR_GET_RADIUS, &args![actor]).f64();
            if radius_b * radius_a * double(e, QUARTER) > distance_squared {
                return (actor_heading(e, this) as f32, looking);
            }
            let tx = e.mem.u32(target.addr());
            let ty = e.mem.u32(target.addr() + 4);
            let tz = e.mem.u32(target.addr() + 8);
            e.call(
                ANGLE_TO_PROJECTED_POINT,
                &args![projected, actor, tx, ty, tz],
            );
            if !e.call(ACTOR_BYTE_14D, &args![actor]).bool() {
                looking = fld(e, projected.addr());
            }
            let heading = if e.call(ACTOR_GET_RADIUS, &args![actor]).f64() >= double(e, HUNDRED) {
                let location = actor_location(e, this);
                e.call(POINT_SUBTRACT, &args![target, vector, location]);
                e.mem.set_f32(vector.addr() + 8, 0.0);
                e.call(Z_ANGLE_FROM_VECTOR, &args![vector]).f64() as f32
            } else {
                fld(e, projected.addr() + 8)
            };
            (heading, looking)
        })
    } else {
        let heading = e.with_stack(0x18, |e, block| {
            let point = block;
            let vector = block.byte_add(0x0c);
            e.call(DEFAULT_CONSTRUCT, &args![point]);
            let has_nodes = path_of(e, this) != 0 && {
                let path = path_of(e, this);
                e.call(PATH_NODE_COUNT, &args![path]).u32() != 0
            };
            if has_nodes {
                let parameter =
                    (e.get(this, H::fCurrentSplineT) as f64 + double(e, SPLINE_STEP)) as f32;
                let path = path_of(e, this);
                e.call(PATH_POINT_AT, &args![path, parameter, point]);
            } else {
                copy_point(e, point.addr(), this.addr() + H::TetherTargetPoint.off);
            }
            let location = actor_location(e, this);
            e.call(POINT_SUBTRACT, &args![point, vector, location]);
            let keep = e.call(ACTOR_BYTE_14D, &args![actor]).bool() && {
                let length = e.call(POINT_LENGTH, &args![vector]).f64();
                let limit = length * double(e, NINETY_FIVE_HUNDREDTHS);
                let z = fld(e, vector.addr() + 8);
                e.call(FLOAT_ABS, &args![z]).f64() >= limit
            };
            if keep {
                actor_heading(e, this) as f32
            } else {
                e.call(Z_ANGLE_FROM_VECTOR, &args![vector]).f64() as f32
            }
        });
        // The code clears the looking angle when swimming; it is still 0 here.
        e.call(ACTOR_BYTE_14D, &args![actor]);
        (heading, 0.0)
    };
    e.mem.set_f32(heading_out.addr(), x87_quiet(heading));
    e.mem.set_f32(looking_out.addr(), x87_quiet(looking));
}

/// `BSWrap(value, -pi, pi)` on a float the game keeps on its stack.
fn wrap_pi(e: &mut Engine, value: Ptr) {
    let minus_pi = fld(e, MINUS_PI_FLOAT);
    let pi = fld(e, PI_FLOAT);
    e.call(BS_WRAP, &args![value, minus_pi, pi]);
}

// Translated from 009e4800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Turns towards the heading from `ComputeHeadingAndLooking` by at most
/// three times the actor's turn rate (slot 0x354, degrees per second) times
/// `seconds`, slowed while the tether point is near and the rate is above
/// 45 (factor 0.025..1). When the turn left is larger than that step,
/// `*scale_out` gets a move-speed factor (0.05 above 2.7 radians, 0.1 above
/// 2.0, 0.2 above 1.6, 0.5 near the end of the path). Records the turn in
/// `fLastRotationApplied`, updates the looking angle and returns the new
/// heading.
pub fn fn_009e4800(e: &mut Engine, this: Ptr<H>, seconds: f32, scale_out: Ptr) -> f32 {
    let actor = e.get(this, H::pActor);
    e.with_stack(0x80, |e, block| {
        let heading = block;
        let looking = block.byte_add(4);
        let current = block.byte_add(8);
        let turn = block.byte_add(0x0c);
        let factor = block.byte_add(0x10);
        let wrapped = block.byte_add(0x14);
        let spline_out = block.byte_add(0x18);
        let tether = block.byte_add(0x1c);
        let to_tether = block.byte_add(0x28);
        let vector = block.byte_add(0x34);
        let goal = block.byte_add(0x40);
        let goal_offset = block.byte_add(0x4c);
        detailed_actor_path_handler_compute_heading_and_looking(e, this, heading, looking);
        let now = actor_heading(e, this) as f32;
        e.mem.set_f32(current.addr(), now);
        wrap_pi(e, current);
        wrap_pi(e, heading);
        let difference =
            (e.mem.f32(heading.addr()) as f64 - e.mem.f32(current.addr()) as f64) as f32;
        e.mem.set_f32(turn.addr(), difference);
        wrap_pi(e, turn);
        let turn_value = fld(e, turn.addr());
        if e.call(FLOAT_ABS, &args![turn_value]).f64() >= double(e, DEGREES_TO_RADIANS) {
            e.mem.set_f32(factor.addr(), 1.0);
            if path_of(e, this) != 0
                && e.vcall(actor.addr(), ACTOR_SLOT_TURN_SPEED, &[]).f64() > double(e, FORTY_FIVE)
            {
                let location = actor_location(e, this);
                let offset = e
                    .call(
                        POINT_SUBTRACT,
                        &args![this.addr() + H::TetherTargetPoint.off, to_tether, location],
                    )
                    .u32();
                let distance = e.call(POINT_LENGTH, &args![offset]).f64() as f32;
                let speed = if e.get(this, H::MoveMode) & 0x200 != 0 {
                    e.call(ACTOR_GET_RUN_SPEED, &args![actor]).f64() as f32
                } else {
                    e.call(ACTOR_GET_WALK_SPEED, &args![actor]).f64() as f32
                };
                let speed = x87_quiet(speed);
                copy_point(e, tether.addr(), this.addr() + H::TetherTargetPoint.off);
                if speed > distance {
                    let rest = (speed as f64 - distance as f64) as f32;
                    let spline = fld(e, this.addr() + H::fCurrentSplineT.off);
                    let path = path_of(e, this);
                    e.call(
                        PATH_ADVANCE,
                        &args![path, spline, x87_quiet(rest), 1u32, spline_out, tether],
                    );
                }
                let now = actor_heading(e, this) as f32;
                let location = actor_location(e, this);
                e.call(POINT_SUBTRACT, &args![tether, vector, location]);
                let towards = e.call(Z_ANGLE_FROM_VECTOR, &args![vector]).f64() as f32;
                e.mem
                    .set_f32(wrapped.addr(), (towards as f64 - now as f64) as f32);
                wrap_pi(e, wrapped);
                let wrapped_value = fld(e, wrapped.addr());
                let off = e.call(FLOAT_ABS, &args![wrapped_value]).f64();
                e.mem
                    .set_f32(factor.addr(), (off / double(e, HALF_PI)) as f32);
                let low = fld(e, MIN_TURN_FACTOR);
                e.call(FLOAT_CLAMP, &args![factor, low, 1.0f32]);
            }
            let rate = e.vcall(actor.addr(), ACTOR_SLOT_TURN_SPEED, &[]).f64();
            let step = (rate
                * double(e, DEGREES_TO_RADIANS)
                * seconds as f64
                * double(e, THREE)
                * e.mem.f32(factor.addr()) as f64) as f32;
            let turn_value = fld(e, turn.addr());
            if (step as f64) < e.call(FLOAT_ABS, &args![turn_value]).f64() {
                let abs = |e: &mut Engine| {
                    let turn_value = fld(e, turn.addr());
                    e.call(FLOAT_ABS, &args![turn_value]).f64()
                };
                if abs(e) > double(e, TURN_SLOWEST) {
                    let scale = fld(e, SCALE_SLOWEST);
                    e.mem.set_f32(scale_out.addr(), scale);
                } else if abs(e) > double(e, TWO_DOUBLE) {
                    let scale = fld(e, ONE_TENTH_FLOAT);
                    e.mem.set_f32(scale_out.addr(), scale);
                } else if abs(e) > double(e, TURN_SLOW) {
                    let scale = fld(e, SCALE_SLOW);
                    e.mem.set_f32(scale_out.addr(), scale);
                } else if path_of(e, this) != 0 && request_of(e, this) != 0 {
                    let path = path_of(e, this);
                    let node = e.call(PATH_END_NODE, &args![path]).u32();
                    let point = e.call(PLUS_4, &args![node]).u32();
                    let end = e.call(COPY_POINT_AT_4, &args![point, goal]).u32();
                    let location = actor_location(e, this);
                    let offset = e
                        .call(POINT_SUBTRACT, &args![location, goal_offset, end])
                        .u32();
                    let distance_squared = e.call(POINT_LENGTH_SQUARED_XY, &args![offset]).f64();
                    let radius_a = e.call(ACTOR_GET_RADIUS, &args![actor]).f64();
                    let radius_b = e.call(ACTOR_GET_RADIUS, &args![actor]).f64();
                    if radius_b * radius_a > distance_squared {
                        let scale = fld(e, SCALE_NEAR_END);
                        e.mem.set_f32(scale_out.addr(), scale);
                    }
                }
            }
            let step = x87_quiet(step);
            e.call(FLOAT_CLAMP, &args![turn, -step, step]);
            let new_heading =
                (e.mem.f32(current.addr()) as f64 + e.mem.f32(turn.addr()) as f64) as f32;
            e.mem.set_f32(heading.addr(), new_heading);
        }
        let turn_value = fld(e, turn.addr());
        let applied = e.call(FLOAT_ABS, &args![turn_value]).f64() as f32;
        e.set(this, H::fLastRotationApplied, applied);
        let looking_value = fld(e, looking.addr());
        detailed_actor_path_handler_update_looking(e, this, x87_quiet(seconds), looking_value);
        fld(e, heading.addr())
    })
}

// Translated from 009e4c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetailedActorPathHandler::UpdateLooking` (Xbox PDB): moves the actor's
/// looking angle (`MobileObject::GetLooking`/`SetLooking`) towards
/// `looking` by at most `seconds` times 1.309 (75 degrees per second).
pub fn detailed_actor_path_handler_update_looking(
    e: &mut Engine,
    this: Ptr<H>,
    seconds: f32,
    looking: f32,
) {
    let actor = e.get(this, H::pActor);
    let current = e.call(ACTOR_GET_LOOKING, &args![actor]).f64() as f32;
    let max_step = (seconds as f64 * double(e, LOOKING_RATE)) as f32;
    let step = e.with_stack(4, |e, direction| {
        e.mem.set_f32(direction.addr(), 0.0);
        let turn = e
            .call(
                ANGLE_DIFFERENCE,
                &args![x87_quiet(current), x87_quiet(looking), direction],
            )
            .f64() as f32;
        let size = e.call(FLOAT_ABS, &args![turn]).f64() as f32;
        let mut step = x87_quiet(size);
        if max_step < step {
            step = x87_quiet(max_step);
        }
        (step, e.mem.f32(direction.addr()))
    });
    let new_looking = (step.0 as f64 * step.1 as f64 + current as f64) as f32;
    e.call(ACTOR_SET_LOOKING, &args![actor, x87_quiet(new_looking)]);
}

/// Whether the character controller's support reference (`00c6fd60`,
/// `00810660`, `TESObjectREFR::FindReferenceFor3D`) is set and answers its
/// slot 0x100 (`009e4cf0`'s obstacle test).
fn controller_touches_reference(e: &mut Engine, controller: u32) -> bool {
    let support = e.call(CONTROLLER_SUPPORT, &args![controller]).u32();
    let object = if support != 0 {
        e.call(SUPPORT_OBJECT, &args![support]).u32()
    } else {
        0
    };
    let reference = e.call(FIND_REFERENCE_FOR_3D, &args![object]).u32();
    reference != 0 && e.vcall(reference, REFERENCE_SLOT_100, &[]).bool()
}

/// `009e4cf0`'s blocked handling: marks the static avoid nodes near the
/// tether point (radius doubled under 128, else the float at +0x1C times
/// 10), or adds one there when there was none (`00691510`).
fn mark_avoid_nodes(e: &mut Engine, this: Ptr<H>) {
    let actor = e.get(this, H::pActor);
    let worldspace = e.call(REFERENCE_GET_WORLDSPACE, &args![actor]).u32();
    let cell = e.call(REFERENCE_PARENT_CELL, &args![actor]).u32();
    let tether = this.addr() + H::TetherTargetPoint.off;
    e.with_stack(0x50, |e, block| {
        let location = block;
        let navmesh_holder = block.byte_add(0x28);
        let triangle = block.byte_add(0x2c);
        let nodes = block.byte_add(0x30);
        let difference = block.byte_add(0x40);
        e.call(
            LOCATION_FROM_POINT_IN_CELL,
            &args![location, tether, cell, worldspace],
        );
        if e.call(LOCATION_RESOLVE_NAVMESH, &args![location, 0u32])
            .bool()
        {
            e.call(NIPOINTER_CONSTRUCT_NULL, &args![navmesh_holder]);
            if e.call(
                LOCATION_GET_NAVMESH_AND_TRIANGLE,
                &args![location, navmesh_holder, triangle],
            )
            .bool()
            {
                e.call(AVOID_NODE_ARRAY_CONSTRUCT, &args![nodes]);
                let index = e.mem.u16(triangle.addr()) as u32;
                let navmesh = e.call(NIPOINTER_GET, &args![navmesh_holder]).u32();
                e.call(NAVMESH_STATIC_AVOID_NODES, &args![navmesh, index, nodes]);
                let mut none_near = true;
                if !e.call(ARRAY_IS_EMPTY, &args![nodes]).bool() {
                    let mut i = 0u32;
                    while i < e.call(ARRAY_SIZE, &args![nodes]).u32() {
                        let slot = e.call(ARRAY_ELEMENT, &args![nodes, i]).u32();
                        let node = e.mem.u32(slot);
                        let point = e.call(DEFAULT_CONSTRUCT, &args![node]).u32();
                        let offset = e
                            .call(POINT_SUBTRACT, &args![point, difference, tether])
                            .u32();
                        let distance_squared = e.call(POINT_LENGTH_SQUARED, &args![offset]).f64();
                        let radius_a = e.call(AVOID_NODE_RADIUS, &args![node]).f64();
                        let radius_b = e.call(AVOID_NODE_RADIUS, &args![node]).f64();
                        if radius_b * radius_a > distance_squared {
                            let radius = e.call(AVOID_NODE_RADIUS, &args![node]).f64();
                            if radius < double(e, RADIUS_LIMIT) {
                                let radius = e.call(AVOID_NODE_RADIUS, &args![node]).f64();
                                let doubled = (radius + radius) as f32;
                                e.call(AVOID_NODE_SET_RADIUS, &args![node, x87_quiet(doubled)]);
                            } else {
                                let value = e.call(AVOID_NODE_FLOAT_1C, &args![node]).f64();
                                let scaled = (value * double(e, TEN)) as f32;
                                e.call(AVOID_NODE_SET_FLOAT_1C, &args![node, x87_quiet(scaled)]);
                            }
                            none_near = false;
                        }
                        i += 1;
                    }
                }
                if none_near {
                    let request = request_of(e, this);
                    let radius = e.call(FLOAT_AT_68, &args![request]).f64() as f32;
                    let navmesh = e.call(NIPOINTER_GET, &args![navmesh_holder]).u32();
                    e.call(
                        NAVMESH_ADD_AVOID_NODE,
                        &args![navmesh, tether, index, radius, 1.0f32],
                    );
                }
                e.call(AVOID_NODE_ARRAY_DESTRUCT, &args![nodes]);
            }
            e.call(NIPOINTER_DESTRUCT, &args![navmesh_holder]);
        }
        e.call(LOCATION_DESTRUCT, &args![location]);
    });
}

// Translated from 009e4cf0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009e4cf0 - C++ exception frame (FS:[0]) and its unwinding not translated
/// The blocked check of a moving actor: compares how far the actor moved
/// since the last update with the expected move (squared, an eighth of
/// it); when it moved too little `fBlockTime` grows (a fifth as fast when
/// the character controller stands on a reference answering slot 0x100) and
/// a tether point that reached the end becomes "passed" (returns `false`).
/// After 1.5 seconds with a moving animation the actor is blocked
/// (`bMovementBlocked`): while not avoiding (avoid state 0, 1 or 5) the
/// static avoid nodes at the tether point are enlarged or one is added.
/// Returns `false` while blocked, `true` otherwise (also while avoiding).
pub fn fn_009e4cf0(e: &mut Engine, this: Ptr<H>, seconds: f32) -> bool {
    let avoid = e.get(this, H::eAvoidState);
    if avoid == 3 || avoid == 4 {
        return true;
    }
    let actor = e.get(this, H::pActor);
    let (stuck, obstacle) = e.with_stack(0x30, |e, block| {
        let location = block;
        let tether_from_last = block.byte_add(0x0c);
        let tether_from_here = block.byte_add(0x18);
        let moved = block.byte_add(0x24);
        let actor_point = actor_location(e, this);
        copy_point(e, location.addr(), actor_point);
        let tether = this.addr() + H::TetherTargetPoint.off;
        let last = this.addr() + H::LastLocation.off;
        e.call(POINT_SUBTRACT, &args![tether, tether_from_last, last]);
        e.call(POINT_SUBTRACT, &args![tether, tether_from_here, location]);
        e.call(POINT_SUBTRACT, &args![location, moved, last]);
        let mut stuck = false;
        let mut obstacle = false;
        let short = if !e.call(ACTOR_BYTE_14D, &args![actor]).bool() {
            e.call(POINT_LENGTH_SQUARED_XY, &args![tether_from_last]);
            e.call(POINT_LENGTH_XY, &args![tether_from_last]);
            e.call(POINT_LENGTH_SQUARED_XY, &args![tether_from_here]);
            let moved_squared = e.call(POINT_DOT, &args![moved, moved]).f64();
            let expected = e.get(this, H::fLastFrameMinMoveDelta) as f64;
            expected * expected * double(e, EIGHTH) > moved_squared
        } else {
            e.call(POINT_LENGTH_SQUARED, &args![tether_from_last]);
            e.call(POINT_LENGTH, &args![tether_from_last]);
            e.call(POINT_LENGTH_SQUARED, &args![tether_from_here]);
            let moved_squared = e.call(POINT_DOT, &args![moved, moved]).f64();
            let last_delta = this.addr() + H::LastFrameMinMoveDelta.off;
            let expected = e.call(POINT_DOT, &args![last_delta, last_delta]).f64();
            expected * double(e, EIGHTH) > moved_squared
        };
        if short {
            let controller = e.call(ACTOR_GET_CHAR_CONTROLLER, &args![actor]).u32();
            if controller != 0 && controller_touches_reference(e, controller) {
                obstacle = true;
            }
            stuck = true;
        }
        copy_point(e, last, location.addr());
        (stuck, obstacle)
    });
    if stuck {
        let time = if obstacle {
            (seconds as f64 * double(e, FIFTH) + e.get(this, H::fBlockTime) as f64) as f32
        } else {
            (e.get(this, H::fBlockTime) as f64 + seconds as f64) as f32
        };
        e.set(this, H::fBlockTime, time);
        if e.get(this, H::eState) == STATE_AT_END {
            e.set(this, H::eState, STATE_PAST_END);
            return false;
        }
    } else if !e.get(this, H::bMovementBlocked) {
        e.set(this, H::fBlockTime, 0.0);
        return true;
    }
    if e.get(this, H::fBlockTime) as f64 > double(e, BLOCK_TIME) {
        let animation = e.vcall(actor.addr(), ACTOR_SLOT_ANIMATION, &[]).u32();
        if animation != 0 {
            let animation = e.vcall(actor.addr(), ACTOR_SLOT_ANIMATION, &[]).u32();
            if e.call(ANIMATION_SHOULD_BE_MOVING, &args![animation]).bool() {
                e.set(this, H::bMovementBlocked, true);
            }
        }
    }
    let blocked = e.get(this, H::bMovementBlocked);
    if blocked {
        let avoid = e.get(this, H::eAvoidState);
        if avoid == 0 || avoid == 1 || avoid == 5 {
            mark_avoid_nodes(e, this);
        }
        let mover = mover_of(e, this);
        e.call(EMPTY_FUNCTION, &args![mover]);
    }
    !blocked
}

// Translated from 009e5480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears `bMovementBlocked` and `fBlockTime`.
pub fn fn_009e5480(e: &mut Engine, this: Ptr<H>) {
    e.set(this, H::bMovementBlocked, false);
    e.set(this, H::fBlockTime, 0.0);
}

// Translated from 009e54a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Rebuilds the shared list of actors to avoid (the Xbox PDB's statics
/// `AvoidanceCandidates`, `...MaxSize`, `...Size`, `...Timestamp`; their
/// PC addresses `011f2bf8`, `011f2a9c`, `011f2aa0`, `011f2aa4` matched by
/// use) once per change of the stamp at +0x14 of `011f6394`: every actor of
/// the list at +0x80 of `011e0e80` whose process (slot 0x1D0) has flag
/// 0x4000, unless its process has flag 0x100000 and its character
/// controller's float +0x5FC is above the setting `011f2bd0`. Returns
/// `false` when the stamp has not changed.
pub fn fn_009e54a0(e: &mut Engine) -> bool {
    let stamp = e.call(STAMP_OF, &args![FRAME_OBJECT]).u32();
    if e.global::<u32>(CANDIDATES_TIMESTAMP) == stamp {
        return false;
    }
    let mut node = e.call(PLUS_80, &args![ACTOR_LIST_OWNER]).u32();
    e.set_global(CANDIDATES_TIMESTAMP, stamp);
    e.call(ARRAY_CLEAR, &args![CANDIDATES]);
    let count = e.call(LIST_COUNT, &args![node]).u32();
    e.set_global(CANDIDATES_MAX_SIZE, count);
    e.call(ARRAY_SET_SIZE, &args![CANDIDATES, count]);
    e.set_global(CANDIDATES_SIZE, 0u32);
    while node != 0 {
        let item = e.call(DEFAULT_CONSTRUCT, &args![node]).u32();
        let actor = e.mem.u32(item);
        if actor != 0 {
            let process = e.vcall(actor, ACTOR_SLOT_PROCESS, &[]).u32();
            if process != 0 && e.call(PROCESS_FLAG_4000, &args![process]).bool() {
                let skip = e.call(ACTOR_GET_CHAR_CONTROLLER, &args![actor]).u32() != 0
                    && e.call(PROCESS_FLAG_100000, &args![process]).bool()
                    && {
                        let controller = e.call(ACTOR_GET_CHAR_CONTROLLER, &args![actor]).u32();
                        let value = e.call(CONTROLLER_FLOAT_5FC, &args![controller]).f64();
                        let limit = setting_float(e, SETTING_AVOID_CONTROLLER_LIMIT);
                        (limit as f64) < value
                    };
                if !skip {
                    let size: u32 = e.global(CANDIDATES_SIZE);
                    e.set_global(CANDIDATES_SIZE, size.wrapping_add(1));
                    e.with_stack(4, |e, slot| {
                        e.mem.set_u32(slot.addr(), actor);
                        e.call(ARRAY_ADD, &args![CANDIDATES, slot]);
                    });
                }
            }
        }
        node = e.call(LIST_NEXT, &args![node]).u32();
    }
    true
}

// Translated from 009e55e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A reference the actor's process points at (cdecl): with the process
/// (`008d8520`, the actor's dword +0x68), when its slot 0x27C object's dword
/// +0x18 and its slot 0x280 index the table `011a3ff0` to the value 6, the
/// reference of its slot 0x128 if that answers slot 0x100; null otherwise.
pub fn fn_009e55e0(e: &mut Engine, actor: Ptr) -> u32 {
    let object = e.call(ACTOR_PROCESS, &args![actor]).u32();
    if object == 0 {
        return 0;
    }
    let first = e.vcall(object, OBJECT_SLOT_27C, &[]).u32();
    if first == 0 {
        return 0;
    }
    let row = e.call(DWORD_AT_18, &args![first]).u32();
    let column = e.vcall(object, OBJECT_SLOT_280, &[]).u32();
    let table = e.mem.u32(ACQUIRE_TABLE.wrapping_add(row.wrapping_mul(4)));
    if e.mem.u32(table.wrapping_add(column.wrapping_mul(4))) != 6 {
        return 0;
    }
    let reference = e.vcall(object, OBJECT_SLOT_128, &[]).u32();
    if reference != 0 && e.vcall(reference, REFERENCE_SLOT_100, &[]).bool() {
        reference
    } else {
        0
    }
}

/// The moving direction of an actor: its move delta (`008b3d50`) rotated
/// by its heading, into `result`.
fn actor_moving_direction(e: &mut Engine, actor: u32, matrix: Ptr, result: Ptr) -> u32 {
    e.call(DEFAULT_CONSTRUCT, &args![matrix]);
    let heading = e.vcall(actor, ACTOR_SLOT_GET_HEADING, &args![0u32]).f64() as f32;
    fn_009e1640(e, matrix, heading);
    let delta = e.call(ACTOR_MOVE_DELTA, &args![actor]).u32();
    e.call(MATRIX_TIMES_POINT, &args![matrix, result, delta])
        .u32()
}

// Translated from 009e5690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Collects the actors in the way of `actor` (cdecl): its move delta
/// rotated by its heading gives the direction and speed; every avoidance
/// candidate nearer than 1000 units, at least a third of the actor's radius
/// (from the request, +0x68) big, and not behind it (beyond the setting
/// `011f2b8c`) is tested with `009e6900`, and so is the player; each hit is
/// added to `result` (`009e99a0`) with its value. `handler` is passed on.
pub fn fn_009e5690(
    e: &mut Engine,
    actor: Ptr,
    request: Ptr,
    move_mode: u32,
    delta: Ptr,
    handler: Ptr<H>,
    result: Ptr,
) {
    e.with_stack(0x100, |e, block| {
        let matrix = block;
        let rotated = block.byte_add(0x24);
        let direction = block.byte_add(0x30);
        let position = block.byte_add(0x3c);
        let value = block.byte_add(0x48);
        let offset = block.byte_add(0x4c);
        let relative = block.byte_add(0x58);
        let other_matrix = block.byte_add(0x64);
        let other_direction = block.byte_add(0x88);
        let to_other = block.byte_add(0x94);
        let to_other_unit = block.byte_add(0xa0);
        let pair = block.byte_add(0xac);
        e.call(DEFAULT_CONSTRUCT, &args![matrix]);
        let heading = e
            .vcall(actor.addr(), ACTOR_SLOT_GET_HEADING, &args![0u32])
            .f64() as f32;
        fn_009e1640(e, matrix, heading);
        e.call(MATRIX_TIMES_POINT, &args![matrix, rotated, delta]);
        copy_point(e, direction.addr(), rotated.addr());
        let tick = e.call(FRAME_FLOAT_C, &args![FRAME_OBJECT]).f64() as f32;
        let look_ahead = setting_float(e, SETTING_AVOID_LOOK_AHEAD);
        let length = e.call(POINT_UNITIZE, &args![direction]).f64() as f32;
        let speed = (length as f64 / tick as f64) as f32;
        let reach = (look_ahead as f64 * speed as f64) as f32;
        let location = e.vcall(actor.addr(), ACTOR_SLOT_LOCATION, &[]).u32();
        copy_point(e, position.addr(), location);
        let radius = e.call(FLOAT_AT_68, &args![request]).f64() as f32;
        e.mem.set_f32(value.addr(), 0.0);
        let mut factor = fld(e, AVOID_FACTOR_WALK);
        fn_009e55e0(e, actor);
        if move_mode & 0x200 != 0 {
            factor = fld(e, AVOID_FACTOR_RUN);
        }
        fn_009e54a0(e);
        let mut i = 0u32;
        while i < e.global::<u32>(CANDIDATES_SIZE) {
            let slot = e.call(ARRAY_ELEMENT, &args![CANDIDATES, i]).u32();
            let other = e.mem.u32(slot);
            i += 1;
            if other == actor.addr() {
                continue;
            }
            let other_location = e.vcall(other, ACTOR_SLOT_LOCATION, &[]).u32();
            copy_point(e, offset.addr(), other_location);
            e.call(POINT_SUBTRACT_ASSIGN, &args![offset, position]);
            let distance_squared = e.call(POINT_LENGTH_SQUARED_XY, &args![offset]).f64() as f32;
            if distance_squared as f64 > double(e, AVOID_RANGE_SQUARED) {
                continue;
            }
            let other_radius = e.call(ACTOR_GET_RADIUS, &args![other]).f64() as f32;
            let third = radius as f64 * double(e, THIRD);
            if (other_radius as f64) < third || other_radius.is_nan() || third.is_nan() {
                continue;
            }
            copy_point(e, relative.addr(), rotated.addr());
            if e.call(ACTOR_IS_MOVING, &args![other]).bool() {
                actor_moving_direction(e, other, other_matrix, other_direction);
                e.call(POINT_SUBTRACT_ASSIGN, &args![relative, other_direction]);
            }
            let relative_length = e.call(POINT_UNITIZE, &args![relative]).f64() as f32;
            let near_a = e.call(SETTING_VALUE, &args![SETTING_AVOID_NEAR]).u32();
            let near_b = e.call(SETTING_VALUE, &args![SETTING_AVOID_NEAR]).u32();
            let near_squared = e.mem.f32(near_a) as f64 * e.mem.f32(near_b) as f64;
            if distance_squared as f64 > near_squared {
                let other_location = e.vcall(other, ACTOR_SLOT_LOCATION, &[]).u32();
                copy_point(e, to_other.addr(), other_location);
                e.call(POINT_SUBTRACT_ASSIGN, &args![to_other, position]);
                copy_point(e, to_other_unit.addr(), to_other.addr());
                e.call(POINT_NORMALIZE, &args![to_other_unit]);
                let ahead = e.call(POINT_DOT, &args![to_other_unit, direction]).f64();
                if ahead < double(e, ZERO_DOUBLE) {
                    continue;
                }
            }
            let other_speed = (relative_length as f64 / tick as f64) as f32;
            let other_reach = (look_ahead as f64 * other_speed as f64) as f32;
            let mut distance = (radius as f64 + other_reach as f64) as f32;
            let minimum = e.call(SETTING_VALUE, &args![SETTING_AVOID_NEAR]).u32();
            if e.mem.f32(minimum) > distance {
                let minimum = e.call(SETTING_VALUE, &args![SETTING_AVOID_NEAR]).u32();
                distance = fld(e, minimum);
            }
            if fn_009e6900(
                e,
                actor,
                handler,
                Ptr::new(other),
                position,
                relative,
                x87_quiet(distance),
                x87_quiet(factor),
                value,
                false,
            ) {
                let hit = fld(e, value.addr());
                let entry = fn_009e5ab0(e, pair, other, hit);
                e.call(AVOID_ARRAY_ADD, &args![result, entry]);
            }
        }
        let player: u32 = e.global(PLAYER);
        if fn_009e6900(
            e,
            actor,
            handler,
            Ptr::new(player),
            position,
            rotated,
            x87_quiet(reach),
            x87_quiet(factor),
            value,
            false,
        ) {
            let hit = fld(e, value.addr());
            let player: u32 = e.global(PLAYER);
            let entry = fn_009e5ab0(e, pair, player, hit);
            e.call(AVOID_ARRAY_ADD, &args![result, entry]);
        }
    });
}

// Translated from 009e5ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An (actor, value) pair at `this`; returns `this`.
pub fn fn_009e5ab0(e: &mut Engine, this: Ptr, actor: u32, value: f32) -> Ptr {
    e.mem.set_u32(this.addr(), actor);
    e.mem.set_f32(this.addr() + 4, x87_quiet(value));
    this
}

/// `009e5ae0`'s check whether `other` stands at the end of the path:
/// `Some(3)` when it does and the request's byte +0xAC is set (the caller
/// returns 3); clears the request's byte +0x9C when it does otherwise.
fn other_at_path_end(e: &mut Engine, this: Ptr<H>, other: u32, block: Ptr) -> Option<u32> {
    if path_of(e, this) == 0 {
        return None;
    }
    let end_location = block;
    let other_location = block.byte_add(0x28);
    let path = path_of(e, this);
    let node = e.call(PATH_END_NODE, &args![path]).u32();
    let location = e.call(PLUS_4, &args![node]).u32();
    e.call(LOCATION_COPY, &args![end_location, location]);
    let radius = e.call(ACTOR_GET_RADIUS, &args![other]).f64() as f32;
    let built = e
        .call(LOCATION_FROM_REFERENCE, &args![other_location, other])
        .u32();
    let near = e
        .call(LOCATION_WITHIN, &args![end_location, built, radius])
        .bool();
    e.call(LOCATION_DESTRUCT, &args![other_location]);
    if near {
        let request = request_of(e, this);
        if e.call(REQUEST_BYTE_AC, &args![request]).bool() {
            e.call(LOCATION_DESTRUCT, &args![end_location]);
            return Some(3);
        }
        let request = request_of(e, this);
        e.call(REQUEST_SET_BYTE_9C, &args![request, 0u32]);
    }
    e.call(LOCATION_DESTRUCT, &args![end_location]);
    None
}

/// `009e5ae0`'s avoid nodes for the actors of the avoid list, added to the
/// new `PathingAvoidNode` array `nodes`: one at the actor's position (from
/// its character controller unless that has flag 0x8000000) and, when it
/// moves faster than its radius per frame, one ahead of it.
fn add_avoid_nodes(e: &mut Engine, this: Ptr<H>, nodes: u32, block: Ptr) {
    let multiplier = if e.get(this, H::MoveMode) & 0x200 != 0 {
        fld(e, AVOID_RADIUS_RUN)
    } else {
        fld(e, AVOID_RADIUS_WALK)
    };
    let multiplier = x87_quiet(multiplier);
    let mut node = this.addr() + H::ActorAvoidList.off;
    let request = request_of(e, this);
    let own_radius = e.call(FLOAT_AT_68, &args![request]).f64() as f32;
    let difference = block;
    let actor_block = block.byte_add(0x10);
    while node != 0 {
        let item = e.call(DEFAULT_CONSTRUCT, &args![node]).u32();
        let other = e.mem.u32(item);
        node = e.call(LIST_NEXT, &args![node]).u32();
        let mut near_factor = 1.0f32;
        let actor_location_now = actor_location(e, this);
        let other_location = e.vcall(other, ACTOR_SLOT_LOCATION, &[]).u32();
        let offset = e
            .call(
                POINT_SUBTRACT,
                &args![other_location, difference, actor_location_now],
            )
            .u32();
        let distance = e.call(POINT_LENGTH, &args![offset]).f64() as f32;
        let reach = e.call(ACTOR_GET_RADIUS, &args![other]).f64() + own_radius as f64;
        if (distance as f64) < reach {
            near_factor = fld(e, AVOID_RADIUS_WALK);
        }
        let (radius, node_type) = avoid_node_radius(e, other, multiplier, Some(near_factor));
        avoid_nodes_for_actor(e, other, radius, node_type, nodes, actor_block, |e| {
            setting_float(e, SETTING_AVOID_NODE_WEIGHT)
        });
    }
}

// Translated from 009e5ae0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009e5ae0 - C++ exception frame (FS:[0]) and /GS cookie check (00ec408c) not translated
/// The avoidance check of a moving actor against the actors in its way
/// (`009e5690`). Each one adds its value times `seconds` to
/// `fAvoidanceDetectionValue` (twice that for one standing still); once the
/// value reaches the setting `011f2b1c` the actor either waits (returns 1,
/// when one of them is the actor it acts on or one it cannot pass) or asks
/// for a new path around avoid nodes made from them (returns 2). Returns 3
/// when an actor stands at the end of the path and the request's byte +0xAC
/// is set, 0 otherwise (the detection value decays when nothing is in the
/// way). While waiting (avoid state 2) any actor in the way keeps it
/// waiting (1). Without a process with flag 0x4000, or moving too slowly,
/// nothing happens (0).
pub fn fn_009e5ae0(e: &mut Engine, this: Ptr<H>, seconds: f32) -> u32 {
    let actor = e.get(this, H::pActor);
    let process = e.vcall(actor.addr(), ACTOR_SLOT_PROCESS, &[]).u32();
    if process == 0 || !e.call(PROCESS_FLAG_4000, &args![process]).bool() {
        return 0;
    }
    if e.call(PROCESS_FLAG_100000, &args![process]).bool() {
        let controller = e.call(ACTOR_GET_CHAR_CONTROLLER, &args![actor]).u32();
        let value = e.call(CONTROLLER_FLOAT_5FC, &args![controller]).f64();
        let limit = setting_float(e, SETTING_AVOID_CONTROLLER_LIMIT);
        if (limit as f64) < value {
            return 0;
        }
    }
    if e.get(this, H::eAvoidState) != 2 {
        let speed = e
            .call(
                POINT_LENGTH_SQUARED_XY,
                &args![this.addr() + H::MoveDelta.off],
            )
            .f64();
        if speed < double(e, QUARTER) {
            return 0;
        }
    }
    let avoid = e.get(this, H::eAvoidState);
    if avoid != 4 && avoid != 3 {
        e.call(LIST_CLEAR, &args![this.addr() + H::ActorAvoidList.off]);
    }
    let target = fn_009e55e0(e, actor);
    // The avoid array is a 0x10-byte BSSimpleArray (009e9b20 clears +4, +8 and +0xc); the exe
    // keeps it at EBP-0x3c, apart from the vector at EBP-0x54. It sits after the other locals
    // here, not under the delta at +0xc.
    e.with_stack(0x310, |e, block| {
        let found = block.byte_add(0x300);
        let delta = block.byte_add(0x0c);
        let up = block.byte_add(0x18);
        let in_way = block.byte_add(0x24);
        let entry = block.byte_add(0x34);
        let location = block.byte_add(0x40);
        let scratch = block.byte_add(0x68);
        let nodes_block = block.byte_add(0x100);
        e.call(AVOID_ARRAY_CONSTRUCT, &args![found]);
        copy_point(e, delta.addr(), this.addr() + H::MoveDelta.off);
        if e.get(this, H::eAvoidState) == 2 {
            let point = e
                .call(POINT_CONSTRUCT, &args![up, 0.0f32, 1.0f32, 0.0f32])
                .u32();
            copy_point(e, delta.addr(), point);
        }
        let request = request_of(e, this);
        let mode = e.get(this, H::MoveMode);
        fn_009e5690(e, actor, Ptr::new(request), mode, delta, this, found);
        e.call(ACTOR_LIST_CONSTRUCT, &args![in_way]);
        let cleanup = |e: &mut Engine| {
            e.call(ACTOR_LIST_DESTRUCT, &args![in_way]);
            e.call(AVOID_ARRAY_DESTRUCT, &args![found]);
        };
        let mut must_wait = false;
        let mut player_in_way = false;
        let mut i = 0u32;
        while i < e.call(ARRAY_SIZE, &args![found]).u32() {
            let item = e.call(ARRAY_ELEMENT_8, &args![found, i]).u32();
            let other = e.mem.u32(item);
            let item = e.call(ARRAY_ELEMENT_8, &args![found, i]).u32();
            let weight = fld(e, item + 4);
            i += 1;
            let player: u32 = e.global(PLAYER);
            if other == player {
                if player != target {
                    if let Some(result) = other_at_path_end(e, this, player, scratch) {
                        cleanup(e);
                        return result;
                    }
                    let value = (weight as f64 * seconds as f64
                        + e.get(this, H::fAvoidanceDetectionValue) as f64)
                        as f32;
                    e.set(this, H::fAvoidanceDetectionValue, value);
                    e.mem.set_u32(entry.addr(), player);
                    e.call(ACTOR_LIST_ADD, &args![in_way, entry]);
                    player_in_way = true;
                }
                continue;
            }
            let mut moving = e.call(ACTOR_IS_MOVING, &args![other]).bool()
                && !e.call(ACTOR_FLAG_008B3C50, &args![other]).bool();
            e.call(LOCATION_DEFAULT, &args![location]);
            if e.call(ACTOR_GET_DESTINATION, &args![other, location])
                .bool()
            {
                let request = request_of(e, this);
                let radius = e.call(FLOAT_AT_68, &args![request]).f64() as f32;
                let own = scratch.byte_add(0x50);
                let built = e.call(LOCATION_FROM_REFERENCE, &args![own, actor]).u32();
                let near = e
                    .call(LOCATION_WITHIN, &args![location, built, radius])
                    .bool();
                e.call(LOCATION_DESTRUCT, &args![own]);
                if near {
                    moving = false;
                }
            }
            e.mem.set_u32(entry.addr(), other);
            if moving {
                if e.get(this, H::eAvoidState) == 2 {
                    e.call(LOCATION_DESTRUCT, &args![location]);
                    cleanup(e);
                    return 1;
                }
                let value = (weight as f64 * seconds as f64
                    + e.get(this, H::fAvoidanceDetectionValue) as f64)
                    as f32;
                e.set(this, H::fAvoidanceDetectionValue, value);
                if e.get(this, H::MoveMode) & 0x200 != 0 {
                    if e.call(ACTOR_MOVE_FLAGS, &args![other]).u32() & 0x200 != 0 {
                        must_wait = true;
                    } else {
                        e.call(ACTOR_LIST_ADD, &args![in_way, entry]);
                    }
                } else if !e.call(ACTOR_CAN_PASS, &args![actor, other]).bool() {
                    must_wait = true;
                    let threshold = e.call(SETTING_VALUE, &args![SETTING_AVOID_THRESHOLD]).u32();
                    let threshold = e.mem.f32(threshold);
                    if threshold <= e.get(this, H::fAvoidanceDetectionValue) {
                        e.call(LOCATION_DESTRUCT, &args![location]);
                        break;
                    }
                }
            } else {
                if e.get(this, H::eAvoidState) == 2 {
                    e.call(LOCATION_DESTRUCT, &args![location]);
                    continue;
                }
                let value = (weight as f64 * double(e, TWO_DOUBLE) * seconds as f64
                    + e.get(this, H::fAvoidanceDetectionValue) as f64)
                    as f32;
                e.set(this, H::fAvoidanceDetectionValue, value);
                if target != 0 && other == target {
                    must_wait = true;
                }
                if let Some(result) = other_at_path_end(e, this, other, scratch) {
                    e.call(LOCATION_DESTRUCT, &args![location]);
                    cleanup(e);
                    return result;
                }
                e.call(ACTOR_LIST_ADD, &args![in_way, entry]);
            }
            e.call(LOCATION_DESTRUCT, &args![location]);
        }

        if e.get(this, H::eAvoidState) == 2 {
            cleanup(e);
            return 0;
        }
        let value = e.get(this, H::fAvoidanceDetectionValue);
        if value as f64 == double(e, ZERO_DOUBLE) {
            cleanup(e);
            return 0;
        }
        let threshold = e.call(SETTING_VALUE, &args![SETTING_AVOID_THRESHOLD]).u32();
        let threshold = e.mem.f32(threshold);
        let reached = threshold <= e.get(this, H::fAvoidanceDetectionValue);
        if reached {
            if must_wait {
                let wait = setting_float(e, SETTING_AVOID_WAIT_TIME);
                e.set(this, H::fWaitTime, wait);
                let timer = setting_float(e, SETTING_AVOIDANCE_TIMER);
                e.set(this, H::fActorAvoidanceTimer, timer);
                let mover = mover_of(e, this);
                e.call(EMPTY_FUNCTION, &args![mover]);
                cleanup(e);
                return 1;
            }
            if !e.call(ARRAY_IS_EMPTY, &args![in_way]).bool() {
                if e.get(this, H::eAvoidState) == 4
                    || player_in_way
                    || e.get(this, H::MoveMode) & 0x200 != 0
                {
                    let minimum = setting_float(e, SETTING_AVOID_MIN_TIMER);
                    let timer = fld(e, this.addr() + H::fActorAvoidanceTimer.off);
                    let longer = e.call(FLOAT_MAX_OF, &args![timer, minimum]).f64() as f32;
                    e.set(this, H::fActorAvoidanceTimer, longer);
                } else {
                    let timer = setting_float(e, SETTING_AVOID_REPATH_TIMER);
                    e.set(this, H::fActorAvoidanceTimer, timer);
                }
                let memory = e.call(OPERATOR_NEW, &args![0x18u32]).u32();
                let nodes = if memory != 0 {
                    e.call(AVOID_NODE_ARRAY_NEW, &args![memory]).u32()
                } else {
                    0
                };
                let count = e.call(ARRAY_SIZE, &args![in_way]).u32();
                for j in 0..count {
                    let slot = e.call(ARRAY_ELEMENT, &args![in_way, j]).u32();
                    e.call(
                        LIST_PUSH_FRONT,
                        &args![this.addr() + H::ActorAvoidList.off, slot],
                    );
                }
                add_avoid_nodes(e, this, nodes, nodes_block);
                let own = scratch;
                let built = e.call(LOCATION_FROM_REFERENCE, &args![own, actor]).u32();
                let request = request_of(e, this);
                e.call(REQUEST_SET_ORIGIN, &args![request, built]);
                e.call(LOCATION_DESTRUCT, &args![own]);
                let request = request_of(e, this);
                e.call(REQUEST_SET_AVOID_NODES, &args![request, nodes]);
                let mover = mover_of(e, this);
                e.call(EMPTY_FUNCTION, &args![mover]);
                cleanup(e);
                return 2;
            }
        }
        if !must_wait && e.call(ARRAY_IS_EMPTY, &args![in_way]).bool() {
            let value = (e.get(this, H::fAvoidanceDetectionValue) as f64 - seconds as f64) as f32;
            e.set(this, H::fAvoidanceDetectionValue, value);
            if (value as f64) < double(e, ZERO_DOUBLE) {
                e.set(this, H::fAvoidanceDetectionValue, 0.0);
            }
        }
        cleanup(e);
        0
    })
}

// Translated from 009e6870 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `PathingAvoidNode` (0x24 bytes) at `this`: the point, an empty point
/// at +0xC, the radius at +0x18, the weight at +0x1C and the type at
/// +0x20. Returns `this`.
pub fn fn_009e6870(
    e: &mut Engine,
    this: Ptr,
    point: Ptr,
    radius: f32,
    weight: f32,
    node_type: u32,
) -> Ptr {
    copy_point(e, this.addr(), point.addr());
    e.call(DEFAULT_CONSTRUCT, &args![this.addr() + 0x0c]);
    e.mem.set_f32(this.addr() + 0x18, x87_quiet(radius));
    e.mem.set_f32(this.addr() + 0x1c, x87_quiet(weight));
    e.mem.set_u32(this.addr() + 0x20, node_type);
    this
}

// Translated from 009e68c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Ends avoidance: avoid state 1, the avoid list cleared, the detection
/// value and the avoidance timer 0.
pub fn fn_009e68c0(e: &mut Engine, this: Ptr<H>) {
    e.set(this, H::eAvoidState, 1);
    e.call(LIST_CLEAR, &args![this.addr() + H::ActorAvoidList.off]);
    e.set(this, H::fAvoidanceDetectionValue, 0.0);
    e.set(this, H::fActorAvoidanceTimer, 0.0);
}

// Translated from 009e6900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `other` is in the way of `actor` moving from `position` along
/// the unit vector `direction` (cdecl): within the actor's forward length
/// plus `other`'s radius plus `distance` (flat), less than 180 units above
/// or below, not the actor's own slot 0x42C when its byte +0x104 is set,
/// not its process's slot 0x278 target unless that one's byte +0x20 is 1,
/// and ahead of it (the dot with `direction` at least `threshold`, or
/// closer than the setting `011f2b8c`). An actor already in the handler's
/// avoid list (while avoiding, states 3 and 4) is never in the way. With
/// `quick` set it returns at once; otherwise `*value_out` gets the dot
/// raised to the 2nd, 4th or 8th power (by the remaining distance in
/// quarters).
#[allow(clippy::too_many_arguments)] // The exe ABI has nine stack arguments.
pub fn fn_009e6900(
    e: &mut Engine,
    actor: Ptr,
    handler: Ptr<H>,
    other: Ptr,
    position: Ptr,
    direction: Ptr,
    distance: f32,
    threshold: f32,
    value_out: Ptr,
    quick: bool,
) -> bool {
    e.with_stack(0x24, |e, block| {
        let offset = block;
        let difference = block.byte_add(0x0c);
        let key = block.byte_add(0x18);
        let ratio = block.byte_add(0x1c);
        e.call(DEFAULT_CONSTRUCT, &args![offset]);
        e.mem.set_f32(value_out.addr(), 0.0);
        if !handler.is_null() {
            let avoid = e.get(handler, H::eAvoidState);
            if avoid == 4 || avoid == 3 {
                e.mem.set_u32(key.addr(), other.addr());
                if e.call(
                    LIST_CONTAINS,
                    &args![handler.addr() + H::ActorAvoidList.off, key],
                )
                .bool()
                {
                    return false;
                }
            }
        }
        let other_location = e.vcall(other.addr(), ACTOR_SLOT_LOCATION, &[]).u32();
        let result = e
            .call(POINT_SUBTRACT, &args![other_location, difference, position])
            .u32();
        copy_point(e, offset.addr(), result);
        let forward = e.call(ACTOR_GET_FORWARD_LENGTH, &args![actor]).f64() as f32;
        let radius = e.call(ACTOR_GET_RADIUS, &args![other]).f64();
        let reach = (radius + distance as f64 + forward as f64) as f32;
        let reach_squared = (reach as f64 * reach as f64) as f32;
        let distance_squared = e.call(POINT_LENGTH_SQUARED_XY, &args![offset]).f64();
        if reach_squared as f64 <= distance_squared
            || reach_squared.is_nan()
            || distance_squared.is_nan()
        {
            return false;
        }
        let z = fld(e, offset.addr() + 8);
        let height = e.call(FLOAT_ABS, &args![z]).f64();
        if height >= double(e, HEIGHT_LIMIT) || height.is_nan() {
            return false;
        }
        if e.call(BYTE_AT_104, &args![actor]).bool()
            && e.vcall(actor.addr(), ACTOR_SLOT_42C, &[]).u32() == other.addr()
        {
            return false;
        }
        let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
        let engaged = if process != 0 {
            let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
            e.vcall(process, PROCESS_SLOT_278, &[]).u32()
        } else {
            0
        };
        if engaged != 0 && e.call(DWORD_AT_8, &args![engaged]).u32() == other.addr() {
            let first = e.call(NIPOINTER_GET, &args![engaged]).u32();
            if e.call(SIGNED_BYTE_AT_20, &args![first]).u32() != 1 {
                return false;
            }
        }
        e.mem.set_f32(offset.addr() + 8, 0.0);
        let length = e.call(POINT_UNITIZE, &args![offset]).f64() as f32;
        let near = e.call(SETTING_VALUE, &args![SETTING_AVOID_NEAR]).u32();
        let close = e.mem.f32(near) > length;
        let remaining = (length as f64 - forward as f64) as f32;
        let remaining = e
            .call(FLOAT_MAX_OF, &args![x87_quiet(remaining), 0.0f32])
            .f64() as f32;
        let dot = e.call(POINT_DOT, &args![offset, direction]).f64() as f32;
        let hit = threshold <= dot || close;
        if !hit {
            return false;
        }
        if quick {
            return true;
        }
        e.mem
            .set_f32(ratio.addr(), (remaining as f64 / reach as f64) as f32);
        e.call(FLOAT_CLAMP, &args![ratio, 0.0f32, 1.0f32]);
        let scaled = e.mem.f32(ratio.addr()) as f64 * double(e, FOUR);
        let quarters = e.call(FTOL, &args![scaled]).u32();
        let mut value = x87_quiet(dot);
        // The compiler's jump table at 009e6b90 (on quarters - 1).
        let squarings = match quarters.wrapping_sub(1) {
            0 => 1,
            1 => 2,
            2 | 3 => 3,
            _ => 0,
        };
        for _ in 0..squarings {
            value = (value as f64 * value as f64) as f32;
        }
        e.mem.set_f32(value_out.addr(), x87_quiet(value));
        true
    })
}

/// `request`'s origin set from a `PathingLocation` built by `build` in a
/// stack block (`006d3ac0`), then the location destroyed.
fn set_request_origin(e: &mut Engine, request: Ptr, build: impl FnOnce(&mut Engine, Ptr) -> u32) {
    e.with_stack(0x28, |e, location| {
        let built = build(e, location);
        e.call(REQUEST_SET_ORIGIN, &args![request, built]);
        e.call(LOCATION_DESTRUCT, &args![location]);
    });
}

// Translated from 009e6ba0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009e6ba0 - C++ exception frame (FS:[0]) and its unwinding not translated
/// `DetailedActorPathHandler::SetWaitForUpdatedPath` (Xbox PDB): prepares
/// `request` for a new path while this one keeps the actor moving. Without
/// a path (or nodes, or while the player sleeps) the state becomes 0.
/// Already waiting: the request starts at the tether point. Otherwise, in
/// state 2 only: blocked, avoiding or rotating actors stop at their
/// location (state 4, the request starts there); else the tether point is
/// moved to a lead distance (setting `011f2b68`, twice, plus itself when
/// running, at most half the request's length) ahead, and unless it left
/// the loaded world (state 4) the request starts there (state 3) with the
/// path's heading as its first tangent, or at the actor when an avoided
/// actor crosses the way.
pub fn detailed_actor_path_handler_set_wait_for_updated_path(
    e: &mut Engine,
    this: Ptr<H>,
    request: Ptr,
) {
    let has_nodes = path_of(e, this) != 0 && {
        let path = path_of(e, this);
        e.call(PATH_NODE_COUNT, &args![path]).u32() != 0
    };
    let player: u32 = e.global(PLAYER);
    if !has_nodes || e.call(PLAYER_IS_SLEEPING, &args![player]).bool() {
        e.set(this, H::eState, 0);
        return;
    }
    e.set(this, H::fDistanceTraveled, 0.0);
    let actor = e.get(this, H::pActor);
    let tether = this.addr() + H::TetherTargetPoint.off;
    if e.get(this, H::bWaitingForUpdate) {
        set_request_origin(e, request, |e, location| {
            e.call(LOCATION_FROM_POINT, &args![location, tether, actor])
                .u32()
        });
        return;
    }
    if e.get(this, H::eState) != STATE_MOVING {
        return;
    }
    e.set(this, H::bWaitingForUpdate, true);
    e.with_stack(0x80, |e, block| {
        let here = block;
        let destination = block.byte_add(0x0c);
        let origin = block.byte_add(0x18);
        let span = block.byte_add(0x24);
        let to_tether = block.byte_add(0x30);
        let start = block.byte_add(0x3c);
        let ahead = block.byte_add(0x48);
        let location = actor_location(e, this);
        copy_point(e, here.addr(), location);
        let stop = e.get(this, H::bMovementBlocked)
            || e.vcall(this.addr(), SLOT_IS_WAITING_FOR_AVOIDANCE, &[])
                .bool()
            || e.get(this, H::bRotateMode);
        if stop {
            copy_point(e, tether, here.addr());
            copy_point(e, this.addr() + H::LastTetherPoint.off, here.addr());
            e.set(this, H::eState, STATE_PAST_END);
            set_request_origin(e, request, |e, location| {
                e.call(LOCATION_FROM_REFERENCE, &args![location, actor])
                    .u32()
            });
            return;
        }
        let end = e.call(PLUS_34, &args![request]).u32();
        let end_point = e.call(COPY_POINT_AT_4, &args![end, destination]).u32();
        let begin = e.call(PLUS_C, &args![request]).u32();
        let begin_point = e.call(COPY_POINT_AT_4, &args![begin, origin]).u32();
        let length = e
            .call(POINT_SUBTRACT, &args![begin_point, span, end_point])
            .u32();
        let request_length = e.call(POINT_LENGTH_XY, &args![length]).f64() as f32;
        let mut lead = setting_float(e, SETTING_PATH_LEAD);
        if e.get(this, H::MoveMode) & 0x200 != 0 {
            let more = e.call(SETTING_VALUE, &args![SETTING_PATH_LEAD]).u32();
            lead = (lead as f64 + e.mem.f32(more) as f64) as f32;
        }
        lead = (lead as f64 + lead as f64) as f32;
        if lead > request_length {
            lead = (request_length as f64 * double(e, HALF)) as f32;
        }
        let offset = e
            .call(POINT_SUBTRACT, &args![here, to_tether, tether])
            .u32();
        let tether_distance = e.call(POINT_LENGTH_XY, &args![offset]).f64() as f32;
        if lead > tether_distance {
            let further = (lead as f64 - tether_distance as f64) as f32;
            detailed_actor_path_handler_update_tether_position(e, this, x87_quiet(further));
        } else {
            let back = (tether_distance as f64 - lead as f64) as f32;
            detailed_actor_path_handler_update_tether_position(e, this, -x87_quiet(back));
        }
        let mut outside = true;
        let cell = e.call(REFERENCE_PARENT_CELL, &args![actor]).u32();
        let interior = cell != 0 && {
            let cell = e.call(REFERENCE_PARENT_CELL, &args![actor]).u32();
            e.call(CELL_IS_INTERIOR, &args![cell]).bool()
        };
        if interior {
            outside = false;
        } else if e.call(REFERENCE_GET_WORLDSPACE, &args![actor]).u32() != 0 {
            let worldspace = e.call(REFERENCE_GET_WORLDSPACE, &args![actor]).u32();
            if e.call(WORLDSPACE_CELL_AT, &args![worldspace, tether]).u32() != 0 {
                outside = false;
            }
        }
        if outside {
            copy_point(e, tether, here.addr());
            copy_point(e, this.addr() + H::LastTetherPoint.off, here.addr());
            e.set(this, H::eState, STATE_PAST_END);
            return;
        }
        copy_point(e, this.addr() + H::LastTetherPoint.off, here.addr());
        e.set(this, H::eState, STATE_AT_END);
        copy_point(e, start.addr(), tether);
        let mut use_heading = true;
        e.call(DEFAULT_CONSTRUCT, &args![ahead]);
        let parameter = (e.get(this, H::fCurrentSplineT) as f64 + double(e, SPLINE_STEP)) as f32;
        let path = path_of(e, this);
        e.call(PATH_POINT_AT, &args![path, parameter, ahead]);
        e.call(POINT_SUBTRACT_ASSIGN, &args![ahead, tether]);
        let heading = e.call(Z_ANGLE_FROM_VECTOR, &args![ahead]).f64() as f32;
        let list = this.addr() + H::ActorAvoidList.off;
        if !e.call(LIST_IS_EMPTY, &args![list]).bool() {
            let mut node = list;
            while node != 0 {
                let item = e.call(DEFAULT_CONSTRUCT, &args![node]).u32();
                let other = e.mem.u32(item);
                node = e.call(LIST_NEXT, &args![node]).u32();
                let radius = e.call(ACTOR_GET_RADIUS, &args![other]).f64() as f32;
                let other_location = e.vcall(other, ACTOR_SLOT_LOCATION, &[]).u32();
                let location = actor_location(e, this);
                if e.call(
                    SEGMENT_CIRCLE_INTERSECTION,
                    &args![location, tether, other_location, radius],
                )
                .bool()
                {
                    let location = actor_location(e, this);
                    copy_point(e, start.addr(), location);
                    use_heading = false;
                    break;
                }
            }
        }
        set_request_origin(e, request, |e, location| {
            e.call(LOCATION_FROM_POINT, &args![location, start, actor])
                .u32()
        });
        let current = request_of(e, this);
        if use_heading {
            e.call(
                REQUEST_SET_INITIAL_HEADING,
                &args![current, x87_quiet(heading)],
            );
        } else {
            fn_009e16c0(e, Ptr::new(current));
        }
    });
}

/// The avoid nodes of one actor added to the `PathingAvoidNode` array
/// `nodes` (`009e5ae0` and `009e71b0`): one ahead of it when it moves more
/// than its radius per frame (weight `010162c0`), and one at its position
/// (taken from its character controller unless that has flag 0x8000000)
/// with the weight `position_weight` reads.
#[allow(clippy::too_many_arguments)] // The two callers pass the same seven values.
fn avoid_nodes_for_actor(
    e: &mut Engine,
    other: u32,
    radius: f32,
    node_type: u32,
    nodes: u32,
    block: Ptr,
    position_weight: impl FnOnce(&mut Engine) -> f32,
) {
    let matrix = block;
    let moving = block.byte_add(0x24);
    let per_frame = block.byte_add(0x30);
    let ahead = block.byte_add(0x3c);
    let avoid_node = block.byte_add(0x48);
    let position = block.byte_add(0x6c);
    let havok = block.byte_add(0x78);
    let converted = block.byte_add(0x88);
    actor_moving_direction(e, other, matrix, moving);
    let tick = e.call(FRAME_FLOAT_C, &args![FRAME_OBJECT]).f64() as f32;
    e.call(POINT_DIVIDE, &args![moving, per_frame, tick]);
    let speed_squared = e.call(POINT_LENGTH_SQUARED, &args![per_frame]).f64();
    if e.call(ACTOR_GET_RADIUS, &args![other]).f64() < speed_squared {
        let other_location = e.vcall(other, ACTOR_SLOT_LOCATION, &[]).u32();
        let point = e
            .call(POINT_ADD, &args![other_location, ahead, per_frame])
            .u32();
        let weight = fld(e, AVOID_NODE_WEIGHT);
        let built = fn_009e6870(
            e,
            avoid_node,
            Ptr::new(point),
            x87_quiet(radius),
            weight,
            node_type,
        );
        e.call(AVOID_NODE_ARRAY_ADD, &args![nodes, built]);
    }
    let other_location = e.vcall(other, ACTOR_SLOT_LOCATION, &[]).u32();
    copy_point(e, position.addr(), other_location);
    let controller = e.call(ACTOR_GET_CHAR_CONTROLLER, &args![other]).u32();
    if controller != 0
        && e.call(
            CONTROLLER_FLAGS_TEST,
            &args![controller + 0x410, 0x0800_0000u32],
        )
        .u32()
            == 0
    {
        let source = e.call(CONTROLLER_HK_POSITION, &args![controller]).u32();
        e.call(COPY_16_BYTES, &args![havok, source]);
        e.call(DEFAULT_CONSTRUCT, &args![converted]);
        e.call(HK_VECTOR_TO_POINT, &args![converted, havok]);
        let x = fld(e, converted.addr());
        e.mem.set_f32(position.addr(), x);
        let y = fld(e, converted.addr() + 4);
        e.mem.set_f32(position.addr() + 4, y);
    }
    let weight = position_weight(e);
    let built = fn_009e6870(
        e,
        avoid_node,
        position,
        x87_quiet(radius),
        weight,
        node_type,
    );
    e.call(AVOID_NODE_ARRAY_ADD, &args![nodes, built]);
}

/// Radius and type of an actor's avoid node: half the radius (type 0) for
/// actors whose slot 0x214 is 4 or 9, otherwise the radius times `factor`
/// (type 2); both times `multiplier`.
fn avoid_node_radius(
    e: &mut Engine,
    other: u32,
    multiplier: f32,
    factor: Option<f32>,
) -> (f32, u32) {
    let kind = e.vcall(other, ACTOR_SLOT_214, &[]).u32();
    let small = kind == 4 || e.vcall(other, ACTOR_SLOT_214, &[]).u32() == 9;
    let radius = e.call(ACTOR_GET_RADIUS, &args![other]).f64();
    if small {
        ((radius * double(e, HALF) * multiplier as f64) as f32, 0)
    } else {
        let scaled = radius * multiplier as f64;
        let scaled = match factor {
            Some(factor) => scaled * factor as f64,
            None => scaled,
        };
        (scaled as f32, 2)
    }
}

// Translated from 009e71b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009e71b0 - C++ exception frame (FS:[0]) and /GS cookie check (00ec408c) not translated
/// Adds avoid nodes for the actors standing in the way of `actor` at the
/// start of a path (cdecl: actor, request, handler): collects them with
/// `009e5690` (move mode 0x101, the global point `011a9478` as the delta),
/// gives the request an avoid node array when it has none, and adds each
/// one's nodes (radius times 1.5); with a handler they also go into its
/// avoid list and its avoid state becomes 3. Nothing happens while the
/// handler is already in avoid state 3.
pub fn fn_009e71b0(e: &mut Engine, actor: Ptr, request: Ptr, handler: Ptr<H>) {
    if !handler.is_null() && e.get(handler, H::eAvoidState) == 3 {
        return;
    }
    let multiplier = fld(e, AVOID_RADIUS_WALK);
    // The avoid array is a 0x10-byte BSSimpleArray (EBP-0x2c in the exe); it sits after the
    // other locals here, not under the slot at +0xc.
    e.with_stack(0xd0, |e, block| {
        let found = block.byte_add(0xc0);
        let slot = block.byte_add(0x0c);
        let nodes_block = block.byte_add(0x10);
        e.call(AVOID_ARRAY_CONSTRUCT, &args![found]);
        fn_009e5690(
            e,
            actor,
            request,
            0x101,
            Ptr::new(START_DELTA),
            handler,
            found,
        );
        if !e.call(ARRAY_IS_EMPTY, &args![found]).bool() {
            e.call(FLOAT_AT_68, &args![request]);
            let mut nodes = e.call(REQUEST_AVOID_NODES, &args![request]).u32();
            if nodes == 0 {
                let memory = e.call(OPERATOR_NEW, &args![0x18u32]).u32();
                nodes = if memory != 0 {
                    e.call(AVOID_NODE_ARRAY_NEW, &args![memory]).u32()
                } else {
                    0
                };
                e.call(REQUEST_SET_AVOID_NODES, &args![request, nodes]);
            }
            let mut i = 0u32;
            while i < e.call(ARRAY_SIZE, &args![found]).u32() {
                let item = e.call(ARRAY_ELEMENT_8, &args![found, i]).u32();
                let other = e.mem.u32(item);
                e.mem.set_u32(slot.addr(), other);
                if !handler.is_null() {
                    e.call(
                        LIST_PUSH_FRONT,
                        &args![handler.addr() + H::ActorAvoidList.off, slot],
                    );
                }
                let (radius, node_type) = avoid_node_radius(e, other, multiplier, None);
                avoid_nodes_for_actor(e, other, radius, node_type, nodes, nodes_block, |e| {
                    fld(e, AVOID_NODE_WEIGHT)
                });
                if !handler.is_null() {
                    e.set(handler, H::eAvoidState, 3);
                }
                i += 1;
            }
        }
        e.call(AVOID_ARRAY_DESTRUCT, &args![found]);
    });
}

// Translated from 009e7550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetailedActorPathHandler::CheckForEndRotate` (Xbox PDB): when the
/// request asks for a final heading (byte +0x99), the mover's next node is
/// the path's last and the actor's slot 0x214 is neither 4 nor 9, starts
/// turning to the request's heading (+0x8C) and returns `bRotateMode`;
/// `false` otherwise.
pub fn detailed_actor_path_handler_check_for_end_rotate(e: &mut Engine, this: Ptr<H>) -> bool {
    let request = request_of(e, this);
    if !e.call(REQUEST_BYTE_99, &args![request]).bool() {
        return false;
    }
    let mover = Ptr::<ActorMover>::new(mover_of(e, this));
    let next = e.get(mover, ActorMover::iNextNode) as u32 + 1;
    let path = path_of(e, this);
    if next != e.call(PATH_NODE_COUNT_2, &args![path]).u32() {
        return false;
    }
    let actor = e.get(this, H::pActor).addr();
    if e.vcall(actor, ACTOR_SLOT_214, &[]).u32() == 4
        || e.vcall(actor, ACTOR_SLOT_214, &[]).u32() == 9
    {
        return false;
    }
    let request = request_of(e, this);
    let heading = e.call(REQUEST_FLOAT_8C, &args![request]).f64() as f32;
    detailed_actor_path_handler_init_rotation_mode(e, this, heading, false, false);
    e.get(this, H::bRotateMode)
}

// Translated from 009e7610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetailedActorPathHandler::InitRotationMode` (Xbox PDB): starts turning
/// in place to `heading` (brought into 0..2pi with `004b1500`) when the
/// turn is larger than a minimum (`010923dc`; `010923e4` with `exact`;
/// `010923e0` with `running_only` while the mover runs): sets
/// `bRotateMode`, the side (`bRotateCCW`), the goal (wrapped to -pi..pi),
/// and the minimum rotate time from the setting `011cd294` unless the
/// actor's byte +0x1B0 is set. Returns whether it started; never while
/// `bIgnoreInitialRotation` is set.
pub fn detailed_actor_path_handler_init_rotation_mode(
    e: &mut Engine,
    this: Ptr<H>,
    heading: f32,
    exact: bool,
    running_only: bool,
) -> bool {
    if e.get(this, H::bIgnoreInitialRotation) {
        return false;
    }
    let mut heading = heading;
    if (heading as f64) < double(e, ZERO_DOUBLE) {
        let full = fld(e, TWO_PI_FLOAT);
        let raised = (heading as f64 + double(e, TWO_PI_DOUBLE)) as f32;
        heading = e.call(FLOAT_MODULO, &args![x87_quiet(raised), full]).f64() as f32;
    } else if heading as f64 >= double(e, TWO_PI_DOUBLE) {
        let full = fld(e, TWO_PI_FLOAT);
        heading = e.call(FLOAT_MODULO, &args![x87_quiet(heading), full]).f64() as f32;
    }
    let (turn_abs, side) = e.with_stack(4, |e, direction| {
        let now = actor_heading(e, this) as f32;
        let turn = e
            .call(ANGLE_DIFFERENCE, &args![now, x87_quiet(heading), direction])
            .f64() as f32;
        let turn_abs = e.call(FLOAT_ABS, &args![x87_quiet(turn)]).f64() as f32;
        (turn_abs, e.mem.f32(direction.addr()))
    });
    let mut minimum = fld(e, TURN_THRESHOLD);
    if exact {
        minimum = fld(e, TURN_THRESHOLD_EXACT);
    } else if running_only {
        let mover = mover_of(e, this);
        if e.call(MOVER_MOVE_MODE_BASE, &args![mover]).u32() & 0x200 != 0 {
            minimum = fld(e, TURN_THRESHOLD_RUNNING);
        }
    }
    if minimum >= turn_abs || minimum.is_nan() || turn_abs.is_nan() {
        return false;
    }
    e.set(this, H::bRotateCCW, side as f64 > double(e, ZERO_DOUBLE));
    e.set(this, H::bRotateMode, true);
    e.set(this, H::fRotateGoal, x87_quiet(heading));
    let goal = this.addr() + H::fRotateGoal.off;
    wrap_pi(e, Ptr::new(goal));
    let actor = e.get(this, H::pActor);
    if !e.call(ACTOR_BYTE_1B0, &args![actor]).bool() {
        let time = setting_float(e, SETTING_MIN_ROTATE_TIME);
        e.set(this, H::fMinRotateTime, time);
    }
    let mover = mover_of(e, this);
    e.call(EMPTY_FUNCTION, &args![mover]);
    true
}

// Translated from 009e77d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009e77d0 - C++ exception frame (FS:[0]) and its unwinding not translated
/// Places the tether at `start_distance` along the path: stores the path
/// length and the spline parameter there (`006e8680`, which also sets the
/// node index). Not waiting for an update: the tether point is that path
/// point, and with a positive distance the actor is moved onto it (its
/// navmesh height, `008ad1c0`); then `UpdateTetherPosition(0)`. Waiting for
/// an update (the flag is cleared): an empty path puts the tether at the
/// path's end; otherwise of four path points ahead (spacing 20 units over
/// the length) the nearest to the actor is taken, preferring those on the
/// other side of the actor along the old tether direction.
pub fn fn_009e77d0(e: &mut Engine, this: Ptr<H>, start_distance: f32) {
    let path = path_of(e, this);
    let length = e.call(PATH_LENGTH, &args![path]).f64() as f32;
    e.set(this, H::fPathLength, length);
    let length = fld(e, this.addr() + H::fPathLength.off);
    let path = path_of(e, this);
    let index = this.addr() + H::iCurrentNodeIndex.off;
    let spline = e
        .call(
            PATH_PARAMETER_AT,
            &args![path, x87_quiet(start_distance), length, index],
        )
        .f64() as f32;
    e.set(this, H::fCurrentSplineT, spline);
    let tether = this.addr() + H::TetherTargetPoint.off;
    let last_tether = this.addr() + H::LastTetherPoint.off;
    let actor = e.get(this, H::pActor);
    if !e.get(this, H::bWaitingForUpdate) {
        let spline = fld(e, this.addr() + H::fCurrentSplineT.off);
        let path = path_of(e, this);
        e.call(PATH_POINT_AT, &args![path, spline, tether]);
        copy_point(e, last_tether, tether);
        if start_distance as f64 > double(e, ZERO_DOUBLE) {
            e.with_stack(0x34, |e, block| {
                let location = block;
                let point = block.byte_add(0x28);
                e.call(LOCATION_FROM_POINT, &args![location, tether, actor]);
                copy_point(e, point.addr(), tether);
                e.call(NAVMESH_Z_FOR_LOCATION, &args![location, point.addr() + 8]);
                let heading = actor_heading(e, this) as f32;
                let worldspace = e.call(REFERENCE_GET_WORLDSPACE, &args![actor]).u32();
                let cell = e.call(REFERENCE_PARENT_CELL, &args![actor]).u32();
                e.call(
                    ACTOR_MOVE_TO_PATH_END,
                    &args![
                        actor,
                        point,
                        x87_quiet(heading),
                        cell,
                        worldspace,
                        1u32,
                        1u32
                    ],
                );
                e.call(LOCATION_DESTRUCT, &args![location]);
            });
        }
        detailed_actor_path_handler_update_tether_position(e, this, 0.0);
        return;
    }
    e.set(this, H::bWaitingForUpdate, false);
    if e.get(this, H::fPathLength) as f64 == double(e, ZERO_DOUBLE) {
        let path = path_of(e, this);
        let count = e.call(PATH_NODE_COUNT, &args![path]).u32();
        e.set(this, H::iCurrentNodeIndex, count.wrapping_sub(1));
        let last = e.get(this, H::iCurrentNodeIndex);
        e.set(this, H::fCurrentSplineT, last as f32);
        tether_to_path_end(e, this);
        let location = actor_location(e, this);
        copy_point(e, last_tether, location);
        e.set(this, H::bWaitingForUpdate, false);
        return;
    }
    e.with_stack(0x60, |e, block| {
        let along = block;
        let here = block.byte_add(0x0c);
        let to_tether = block.byte_add(0x18);
        let candidate = block.byte_add(0x24);
        let point = block.byte_add(0x30);
        let difference = block.byte_add(0x3c);
        let node = block.byte_add(0x48);
        e.call(POINT_SUBTRACT, &args![tether, along, last_tether]);
        e.mem.set_f32(along.addr() + 8, 0.0);
        let location = actor_location(e, this);
        copy_point(e, here.addr(), location);
        e.call(POINT_SUBTRACT, &args![tether, to_tether, here]);
        let side = e.call(POINT_DOT, &args![to_tether, along]).f64() as f32;
        let behind = (side as f64) < double(e, ZERO_DOUBLE);
        let length = e.get(this, H::fPathLength);
        let step = (double(e, FIVE) / length as f64) as f32;
        let spacing = (step as f64 * double(e, FOUR)) as f32;
        let mut best_distance = fld(e, FLOAT_MAX);
        let mut best_parameter = 0.0f32;
        let mut best_index = 0u32;
        e.call(DEFAULT_CONSTRUCT, &args![candidate]);
        e.call(DEFAULT_CONSTRUCT, &args![point]);
        let mut found = false;
        for k in 0..4i32 {
            let distance =
                ((start_distance as f64 + step as f64) + k as f64 * spacing as f64) as f32;
            let length = fld(e, this.addr() + H::fPathLength.off);
            let path = path_of(e, this);
            let parameter = e
                .call(
                    PATH_PARAMETER_AT,
                    &args![path, x87_quiet(distance), length, node],
                )
                .f64() as f32;
            let path = path_of(e, this);
            e.call(PATH_POINT_AT, &args![path, x87_quiet(parameter), point]);
            let result = e
                .call(POINT_SUBTRACT, &args![point, difference, here])
                .u32();
            copy_point(e, candidate.addr(), result);
            e.mem.set_f32(candidate.addr() + 8, 0.0);
            let dot = e.call(POINT_DOT, &args![candidate, along]).f64() as f32;
            let distance_squared = e.call(POINT_LENGTH_SQUARED_XY, &args![candidate]).f64() as f32;
            let crossing = if behind {
                dot as f64 > double(e, ZERO_DOUBLE)
            } else {
                (dot as f64) < double(e, ZERO_DOUBLE)
            };
            let node_index = e.mem.u32(node.addr());
            if crossing && !found {
                found = true;
                best_index = node_index;
                best_parameter = x87_quiet(parameter);
                best_distance = x87_quiet(distance_squared);
            } else if best_distance > distance_squared {
                best_index = node_index;
                best_parameter = x87_quiet(parameter);
                best_distance = x87_quiet(distance_squared);
            }
        }
        let path = path_of(e, this);
        e.call(
            PATH_POINT_AT,
            &args![path, x87_quiet(best_parameter), tether],
        );
        e.set(this, H::iCurrentNodeIndex, best_index);
        e.set(this, H::fCurrentSplineT, x87_quiet(best_parameter));
        copy_point(e, last_tether, here.addr());
    });
}

// Translated from 009e7d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetailedActorPathHandler::UpdateRotation` (Xbox PDB): one step of the
/// turn in place towards `fRotateGoal`, at the actor's turn rate (slot
/// 0x354, degrees per second, times the setting chosen like
/// `NoPathUpdate`'s) times `seconds`, with the mover's turn bits in
/// `MoveMode` (0x20 counter-clockwise, 0x10 clockwise). Within one degree,
/// or when the step reaches the goal, the heading is the goal, and the
/// rotation ends once `fMinRotateTime` has run out. Returns the new
/// heading; an actor that cannot move keeps its heading.
pub fn detailed_actor_path_handler_update_rotation(
    e: &mut Engine,
    this: Ptr<H>,
    seconds: f32,
) -> f32 {
    let actor = e.get(this, H::pActor);
    let mover = mover_of(e, this);
    e.with_stack(8, |e, block| {
        let heading = block;
        let turn = block.byte_add(4);
        let now = actor_heading(e, this) as f32;
        e.mem.set_f32(heading.addr(), now);
        if !e.call(ACTOR_CAN_MOVE, &args![actor]).bool() {
            let mode = e.call(MOVER_MOVE_MODE_BASE, &args![mover]).u32();
            e.set(this, H::MoveMode, mode);
            return fld(e, heading.addr());
        }
        wrap_pi(e, heading);
        let difference =
            (e.get(this, H::fRotateGoal) as f64 - e.mem.f32(heading.addr()) as f64) as f32;
        e.mem.set_f32(turn.addr(), difference);
        wrap_pi(e, turn);
        let finish = |e: &mut Engine| {
            let goal = fld(e, this.addr() + H::fRotateGoal.off);
            e.mem.set_f32(heading.addr(), goal);
            if e.get(this, H::fMinRotateTime) as f64 <= double(e, ZERO_DOUBLE) {
                e.set(this, H::bRotateMode, false);
                e.call(EMPTY_FUNCTION, &args![mover]);
            }
        };
        let turn_value = fld(e, turn.addr());
        if e.call(FLOAT_ABS, &args![turn_value]).f64() > double(e, DEGREES_TO_RADIANS) {
            let rate = e.vcall(actor.addr(), ACTOR_SLOT_TURN_SPEED, &[]).f64();
            let step = (rate * double(e, DEGREES_TO_RADIANS) * seconds as f64) as f32;
            let flagged = e.vcall(actor.addr(), ACTOR_SLOT_21C, &[]).bool();
            let byte = e.call(BYTE_AT_104, &args![actor]).bool();
            let factor = match (flagged, byte) {
                (false, true) => setting_float(e, SETTING_TURN_RATE_A),
                (false, false) => setting_float(e, SETTING_TURN_RATE_B),
                (true, true) => setting_float(e, SETTING_TURN_RATE_C),
                (true, false) => setting_float(e, SETTING_TURN_RATE_D),
            };
            let step = (step as f64 * factor as f64) as f32;
            let turn_value = fld(e, turn.addr());
            let turn_abs = e.call(FLOAT_ABS, &args![turn_value]).f64();
            if step as f64 > turn_abs {
                finish(e);
            } else if e.get(this, H::bRotateCCW) {
                let mode = e.call(MOVER_MOVE_MODE_BASE, &args![mover]).u32() | 0x20;
                e.set(this, H::MoveMode, mode);
                let next = (e.mem.f32(heading.addr()) as f64 + step as f64) as f32;
                e.mem.set_f32(heading.addr(), next);
            } else {
                let mode = e.call(MOVER_MOVE_MODE_BASE, &args![mover]).u32() | 0x10;
                e.set(this, H::MoveMode, mode);
                let next = (e.mem.f32(heading.addr()) as f64 - step as f64) as f32;
                e.mem.set_f32(heading.addr(), next);
            }
        } else {
            finish(e);
        }
        let left = (e.get(this, H::fMinRotateTime) as f64 - seconds as f64) as f32;
        e.set(this, H::fMinRotateTime, left);
        fld(e, heading.addr())
    })
}

// Translated from 009e8000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetailedActorPathHandler::CheckGoalReached` (Xbox PDB): 5 when the
/// actor has reached the goal, 2 when the path failed on the goal's height,
/// otherwise 0.
///
/// While `bEndUpdatePath` is set the path's end is still being updated:
/// near the end (the current node two before the last and within 160 units
/// of the next one, or past it) the check `006d5340` decides (1: the goal;
/// 2 or 3: `bEndUpdatePath` from the end node's flag 4 and the mover's
/// `iNextNode` from the path's dword at +0x1C; else `bEndUpdatePath` ends).
///
/// Otherwise the goal is the end node's point (or, when the path's byte at
/// +0x40 is clear and its dword at +0x1C names its last node, that node's
/// point); it is reached within `fGoalRadiusSqr` (horizontal, squared).
/// With the request's byte at +0x9C set and both the radius and the
/// distance above 25, a tether point that has stopped moving towards the
/// goal while there is no avoidance and no block does not count. Then the
/// height difference decides: against the request's float at +0x70, or,
/// with the actor's byte at +0x14D set, against the smaller of that and
/// the actor's height. An actor two nodes or less from the end with too
/// large a difference fails (and the reason is logged).
pub fn detailed_actor_path_handler_check_goal_reached(e: &mut Engine, this: Ptr<H>) -> u32 {
    if e.get(this, H::bEndUpdatePath) {
        return check_goal_while_updating_end(e, this);
    }
    e.with_stack(0x54, |e, block| {
        let goal = block;
        let difference = block.byte_add(0x0c);
        let copy = block.byte_add(0x18);
        let result = block.byte_add(0x24);
        let along = block.byte_add(0x30);
        let to_actor = block.byte_add(0x3c);
        e.call(DEFAULT_CONSTRUCT, &args![goal]);
        e.call(DEFAULT_CONSTRUCT, &args![difference]);
        let path = path_of(e, this);
        let mut node = None;
        if !e.call(PATH_BYTE_40, &args![path]).bool() {
            let path = path_of(e, this);
            let index = e.call(DWORD_AT_1C, &args![path]).u32();
            let path = path_of(e, this);
            let count = e.call(PATH_NODE_COUNT_2, &args![path]).u32();
            if index == count.wrapping_sub(1) {
                let path = path_of(e, this);
                let index = e.call(DWORD_AT_1C, &args![path]).u32();
                let path = path_of(e, this);
                node = Some(e.call(PATH_NODE_BY_INDEX, &args![path, index]).u32());
            }
        }
        let node = match node {
            Some(node) => node,
            None => {
                let path = path_of(e, this);
                e.call(PATH_END_NODE, &args![path]).u32()
            }
        };
        let point = e.call(PLUS_4, &args![node]).u32();
        let copied = e.call(COPY_POINT_AT_4, &args![point, copy]).u32();
        copy_point(e, goal.addr(), copied);
        let location = actor_location(e, this);
        let offset = e.call(POINT_SUBTRACT, &args![goal, result, location]).u32();
        copy_point(e, difference.addr(), offset);
        let distance = e.call(POINT_LENGTH_SQUARED_XY, &args![difference]).f64() as f32;
        let radius = e.get(this, H::fGoalRadiusSqr);
        let within = distance <= radius;
        if !within {
            return 0;
        }
        let mut reached = true;
        let request = request_of(e, this);
        if fn_009e8670(e, Ptr::new(request)) != 0 {
            let minimum = e.mem.f32(GOAL_MIN_DISTANCE);
            if minimum < e.get(this, H::fGoalRadiusSqr) && minimum < distance {
                let tether = this.addr() + H::TetherTargetPoint.off;
                e.call(
                    POINT_SUBTRACT,
                    &args![tether, along, this.addr() + H::LastTetherPoint.off],
                );
                e.mem.set_f32(along.addr() + 8, 0.0);
                let location = actor_location(e, this);
                e.call(POINT_SUBTRACT, &args![tether, to_actor, location]);
                e.mem.set_f32(to_actor.addr() + 8, 0.0);
                let dot = e.call(POINT_DOT, &args![along, to_actor]).f64() as f32;
                if dot as f64 >= double(e, ZERO_DOUBLE)
                    && e.get(this, H::fAvoidanceDetectionValue) as f64 == double(e, ZERO_DOUBLE)
                    && e.get(this, H::fBlockTime) as f64 == double(e, ZERO_DOUBLE)
                {
                    reached = false;
                }
            }
        }
        if !reached {
            return 0;
        }
        let actor = e.get(this, H::pActor).addr();
        let height_difference = |e: &mut Engine| {
            let z = x87_quiet(e.mem.f32(difference.addr() + 8));
            e.call(FLOAT_ABS, &args![z.to_bits()]).f64()
        };
        if !e.call(ACTOR_BYTE_14D, &args![actor]).bool() {
            let z_abs = height_difference(e);
            let request = request_of(e, this);
            let allowed = e.call(REQUEST_FLOAT_70, &args![request]).f64();
            if allowed >= z_abs {
                return 5;
            }
            let index = e.get(this, H::iCurrentNodeIndex);
            let path = path_of(e, this);
            let count = e.call(PATH_NODE_COUNT, &args![path]).u32();
            if index.wrapping_add(1) < count || e.call(ACTOR_BYTE_14D, &args![actor]).bool() {
                return 0;
            }
            let name = if e.call(HAS_NAME, &args![actor]).u32() != 0 {
                e.vcall(actor, ACTOR_SLOT_NAME, &[]).u32()
            } else {
                e.call(REFERENCE_NAME, &args![actor]).u32()
            };
            let z_abs = height_difference(e);
            e.call(LOG_MESSAGE, &args![GOAL_Z_MESSAGE, name, z_abs]);
            2
        } else {
            let height = e.call(ACTOR_GET_HEIGHT, &args![actor]).f64() as f32;
            let request = request_of(e, this);
            let allowed = e.call(REQUEST_FLOAT_70, &args![request]).f64() as f32;
            let limit = e
                .call(FLOAT_MIN, &args![allowed.to_bits(), height.to_bits()])
                .f64() as f32;
            let z_abs = height_difference(e);
            if limit as f64 >= z_abs {
                5
            } else {
                0
            }
        }
    })
}

/// `CheckGoalReached` while `bEndUpdatePath` is set.
fn check_goal_while_updating_end(e: &mut Engine, this: Ptr<H>) -> u32 {
    let index = e.get(this, H::iCurrentNodeIndex);
    let path = path_of(e, this);
    if index.wrapping_add(2) < e.call(PATH_NODE_COUNT, &args![path]).u32() {
        return 0;
    }
    e.with_stack(0x24, |e, block| {
        let offset = block;
        let copy = block.byte_add(0x0c);
        let result = block.byte_add(0x18);
        e.call(DEFAULT_CONSTRUCT, &args![offset]);
        let index = e.get(this, H::iCurrentNodeIndex);
        let path = path_of(e, this);
        let count = e.call(PATH_NODE_COUNT, &args![path]).u32();
        if index.wrapping_add(2) == count {
            let location = actor_location(e, this);
            let index = e.get(this, H::iCurrentNodeIndex);
            let path = path_of(e, this);
            let node = e.call(PATH_NODE, &args![path, index]).u32();
            let point = e.call(PLUS_4, &args![node]).u32();
            let copied = e.call(COPY_POINT_AT_4, &args![point, copy]).u32();
            let difference = e
                .call(POINT_SUBTRACT, &args![copied, result, location])
                .u32();
            copy_point(e, offset.addr(), difference);
            let distance = e.call(POINT_LENGTH_SQUARED_XY, &args![offset]).f64();
            let near = distance <= double(e, END_UPDATE_RANGE_SQUARED);
            if !near {
                return 0;
            }
        }
        let path = path_of(e, this);
        e.call(PATH_NODE_COUNT, &args![path]);
        let mover = Ptr::<ActorMover>::new(mover_of(e, this));
        let next = e.get(mover, ActorMover::iNextNode) as u32;
        // One less unless it is 0.
        let node_index = next.saturating_sub(1);
        let location = actor_location(e, this);
        let path = path_of(e, this);
        let request = request_of(e, this);
        let check = e
            .call(
                PATHFIND_006D5340,
                &args![request, path, node_index, location],
            )
            .i32();
        match check {
            1 => 5,
            2 | 3 => {
                let path = path_of(e, this);
                let end = e.call(PATH_END_NODE, &args![path]).u32();
                let flag = e.call(FLAGS_BIT_2, &args![end]).u8();
                e.mem.set_u8(this.addr() + H::bEndUpdatePath.off, flag);
                let path = path_of(e, this);
                let next = e.call(DWORD_AT_1C, &args![path]).u32();
                e.set(mover, ActorMover::iNextNode, next as u16);
                0
            }
            _ => {
                e.set(this, H::bEndUpdatePath, false);
                0
            }
        }
    })
}

// Translated from 009e8670 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at +0x9C of a `PathingRequest` (`CheckGoalReached` asks it).
pub fn fn_009e8670(e: &mut Engine, request: Ptr) -> u8 {
    e.mem.u8(request.addr() + 0x9c)
}

// Translated from 009e8690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetailedActorPathHandler::HandleDeletedReference` (Xbox PDB): forgets
/// the door when it is the reference; for a reference whose slot 0x100
/// answers true, takes it out of the avoidance candidates (each match is
/// removed with `009e98d0`, the array compacted and the candidate count
/// lowered; the index still advances) and out of the avoid list, ending
/// avoidance when the list is then empty.
pub fn detailed_actor_path_handler_handle_deleted_reference(
    e: &mut Engine,
    this: Ptr<H>,
    reference: Ptr,
) {
    if reference.addr() == e.get(this, H::pCurrentDoor).addr() {
        e.set(this, H::eDoorState, 0);
        e.set(this, H::pCurrentDoor, Ptr::NULL);
    }
    if !e.vcall(reference.addr(), REFERENCE_SLOT_100, &[]).bool() {
        return;
    }
    let mut index = 0u32;
    while index < e.global::<u32>(CANDIDATES_SIZE) {
        let slot = e.call(ARRAY_ELEMENT, &args![CANDIDATES, index]).u32();
        if e.mem.u32(slot) == reference.addr() {
            fn_009e98d0(e, Ptr::new(CANDIDATES), index);
            e.call(ARRAY_COMPACT, &args![CANDIDATES]);
            let size = e.global::<u32>(CANDIDATES_SIZE);
            e.set_global(CANDIDATES_SIZE, size.wrapping_sub(1));
        }
        index += 1;
    }
    let list = this.addr() + H::ActorAvoidList.off;
    let listed = e.with_stack(4, |e, item| {
        e.mem.set_u32(item.addr(), reference.addr());
        e.call(LIST_CONTAINS, &args![list, item]).bool()
    });
    if !listed {
        return;
    }
    e.with_stack(4, |e, item| {
        e.mem.set_u32(item.addr(), reference.addr());
        e.call(LIST_REMOVE, &args![list, item]);
    });
    if e.call(LIST_IS_EMPTY, &args![list]).bool() {
        fn_009e68c0(e, this);
    }
}

// Translated from 009e8790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets the movement of this frame: `MoveMode` from the mover
/// (`009dd130`), `MoveDelta` and `LastFrameMinMoveDelta` to the "no point"
/// value, `fLastFrameMinMoveDelta` 0.
pub fn fn_009e8790(e: &mut Engine, this: Ptr<H>) {
    let mover = mover_of(e, this);
    let mode = e.call(MOVER_MOVE_MODE_BASE, &args![mover]).u32();
    e.set(this, H::MoveMode, mode);
    clear_frame_delta(e, this);
}

// Translated from 009e8800 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009e8800 - C++ exception frame (FS:[0]) and its unwinding not translated
/// `DetailedActorPathHandler::WarpToNextNode` (Xbox PDB): moves the actor
/// straight to the tether point when that is safe. Not with a request whose
/// float at +0x68 is above 100 or a path without nodes. A tether point less
/// than 128 units away (horizontally) is pushed further first; then the
/// tether point must be at least 64 away, and the path's end at least the
/// actor's `008be350` float times 0.6 from it when the path's byte at +0x40
/// is set (else 0). The line from the actor to the tether point is checked
/// with a `DoorInPathFilter` for no door (`Pathing::CheckLineOfSight`);
/// without a hit the mover moves the actor there
/// (`ActorMover::MoveActorToLocation`) and the result is `true`.
pub fn detailed_actor_path_handler_warp_to_next_node(e: &mut Engine, this: Ptr<H>) -> bool {
    if request_of(e, this) != 0 {
        let request = request_of(e, this);
        let value = e.call(FLOAT_AT_68, &args![request]).f64();
        if value > double(e, HUNDRED) {
            return false;
        }
    }
    // The code also stores the squares of both distances; nothing reads them.
    let min_distance = fld(e, WARP_MIN_DISTANCE);
    let tether_distance = fld(e, WARP_TETHER_DISTANCE);
    let tether = this.addr() + H::TetherTargetPoint.off;
    e.with_stack(0x6c, |e, block| {
        let first = block;
        let second = block.byte_add(0x0c);
        let copy = block.byte_add(0x18);
        let third = block.byte_add(0x24);
        let filter = block.byte_add(0x30);
        let location = block.byte_add(0x3c);
        let navmesh_holder = block.byte_add(0x64);
        let triangle = block.byte_add(0x68);
        let actor_at = actor_location(e, this);
        let offset = e
            .call(POINT_SUBTRACT, &args![tether, first, actor_at])
            .u32();
        let distance = e.call(POINT_LENGTH_XY, &args![offset]).f64() as f32;
        let mut end_limit = 0.0f32;
        let path = path_of(e, this);
        if e.call(PATH_BYTE_40, &args![path]).bool() {
            let actor = e.get(this, H::pActor).addr();
            let value = e.call(ACTOR_MEASURE_008BE350, &args![actor]).f64();
            end_limit = (value * double(e, WARP_END_FACTOR)) as f32;
        }
        let path = path_of(e, this);
        if e.call(PATH_NODE_COUNT, &args![path]).u32() == 0 {
            return false;
        }
        if tether_distance > distance {
            let further = (tether_distance as f64 - distance as f64) as f32;
            detailed_actor_path_handler_update_tether_position(e, this, x87_quiet(further));
        }
        let actor_at = actor_location(e, this);
        let offset = e
            .call(POINT_SUBTRACT, &args![tether, second, actor_at])
            .u32();
        let distance = e.call(POINT_LENGTH_XY, &args![offset]).f64() as f32;
        let path = path_of(e, this);
        let end = e.call(PATH_END_NODE, &args![path]).u32();
        let point = e.call(PLUS_4, &args![end]).u32();
        let copied = e.call(COPY_POINT_AT_4, &args![point, copy]).u32();
        let offset = e.call(POINT_SUBTRACT, &args![copied, third, tether]).u32();
        let end_distance = e.call(POINT_LENGTH_XY, &args![offset]).f64() as f32;
        if min_distance > distance || end_limit > end_distance {
            return false;
        }
        let actor = e.get(this, H::pActor);
        fn_009e2690(e, filter, 0);
        e.call(LOCATION_FROM_REFERENCE, &args![location, actor]);
        e.call(LOCATION_RESOLVE_NAVMESH, &args![location, 0u32]);
        e.call(NIPOINTER_CONSTRUCT_NULL, &args![navmesh_holder]);
        door_triangle_flag(e, location, navmesh_holder, triangle, filter);
        e.call(CHECK_LINE_OF_SIGHT, &args![location, tether, 0u32, filter]);
        let moved = if door_filter_hit(e, filter) {
            false
        } else {
            let mover = mover_of(e, this);
            e.call(MOVER_MOVE_ACTOR_TO_LOCATION, &args![mover, tether]);
            true
        };
        e.call(NIPOINTER_DESTRUCT, &args![navmesh_holder]);
        e.call(LOCATION_DESTRUCT, &args![location]);
        moved
    })
}

// Translated from 009e8b50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009e8b50 - /GS cookie check (00ec408c) not translated
/// `DetailedActorPathHandler::PrintCurrentStatus` (Xbox PDB): prints three
/// lines through the printer's slot 0x14 (with the two words passed
/// through): the actor's position, "Pathing State: " with the state's name
/// (state 2 reads "Rotating" in rotate mode and "Static Avoidance" when
/// blocked), and "Actor Avoid State: " with the avoid state's name
/// (states 1 and 4 add " :detect %f" with `fAvoidanceDetectionValue` when
/// it is above 0). The texts are the exe's strings, by address.
pub fn detailed_actor_path_handler_print_current_status(
    e: &mut Engine,
    this: Ptr<H>,
    printer: Ptr,
    first: u32,
    second: u32,
) {
    const POSITION_FORMAT: u32 = 0x0109_2604;
    const STATE_TITLE: u32 = 0x0109_25f4;
    const STATE_INITIALIZING: u32 = 0x0109_25e4;
    const STATE_ROTATING_TEXT: u32 = 0x0109_25d8;
    const STATE_STATIC_AVOIDANCE: u32 = 0x0109_25c4;
    const STATE_MOVING_TEXT: u32 = 0x0109_25bc;
    const STATE_MOVING_WHILE_WAITING: u32 = 0x0109_25a4;
    const STATE_WAITING_FOR_PATH: u32 = 0x0109_2588;
    const STATE_GOAL_REACHED: u32 = 0x0109_2578;
    const STATE_ROTATING_WITHOUT_PATH: u32 = 0x0109_2560;
    const INVALID: u32 = 0x0104_8858;
    const AVOID_TITLE: u32 = 0x0109_254c;
    const AVOID_OK: u32 = 0x0103_934c;
    const AVOID_DETECT_FORMAT: u32 = 0x0109_2540;
    const AVOID_WAITING: u32 = 0x0107_1db4;
    const AVOID_REPATHING: u32 = 0x0109_2528;
    const AVOID_REPATH_IGNORE: u32 = 0x0109_2518;
    const AVOID_FAILED: u32 = 0x0109_2510;
    const SIZE: u32 = 0x104;

    let location = actor_location(e, this);
    let [x, y, z] = [0, 4, 8].map(|i| x87_quiet(e.mem.f32(location + i)) as f64);
    e.vcall(
        printer.addr(),
        PRINTER_SLOT_PRINT,
        &args![first, second, POSITION_FORMAT, x, y, z],
    );
    e.with_stack(0x210, |e, block| {
        let text = block;
        let detect = block.byte_add(0x108);
        e.call(STRING_COPY, &args![text, SIZE, STATE_TITLE]);
        let state = match e.get(this, H::eState) {
            1 => STATE_INITIALIZING,
            2 if e.get(this, H::bRotateMode) => STATE_ROTATING_TEXT,
            2 if e.get(this, H::bMovementBlocked) => STATE_STATIC_AVOIDANCE,
            2 => STATE_MOVING_TEXT,
            3 => STATE_MOVING_WHILE_WAITING,
            4 => STATE_WAITING_FOR_PATH,
            5 => STATE_GOAL_REACHED,
            6 | 7 => STATE_ROTATING_WITHOUT_PATH,
            _ => INVALID,
        };
        e.call(STRING_APPEND, &args![text, SIZE, state]);
        e.vcall(
            printer.addr(),
            PRINTER_SLOT_PRINT,
            &args![first, second, text],
        );
        e.call(STRING_COPY, &args![text, SIZE, AVOID_TITLE]);
        let avoid_state = e.get(this, H::eAvoidState);
        let name = match avoid_state {
            1 => AVOID_OK,
            2 => AVOID_WAITING,
            3 => AVOID_REPATHING,
            4 => AVOID_REPATH_IGNORE,
            5 => AVOID_FAILED,
            _ => INVALID,
        };
        e.call(STRING_APPEND, &args![text, SIZE, name]);
        if avoid_state == 1 || avoid_state == 4 {
            let detection = e.get(this, H::fAvoidanceDetectionValue);
            if detection as f64 > double(e, ZERO_DOUBLE) {
                let value = x87_quiet(detection) as f64;
                e.call(
                    STRING_FORMAT,
                    &args![detect, SIZE, AVOID_DETECT_FORMAT, value],
                );
                e.call(STRING_APPEND, &args![text, SIZE, detect]);
            }
        }
        e.vcall(
            printer.addr(),
            PRINTER_SLOT_PRINT,
            &args![first, second, text],
        );
    });
}

/// The fields `SaveGame` writes, in its order (address offset, size).
const SAVED_FIELDS: [(u32, u32); 43] = [
    (0x1c, 0xc),
    (0x28, 0xc),
    (0x34, 0xc),
    (0x40, 0xc),
    (0x4c, 0xc),
    (0x60, 4),
    (0x64, 4),
    (0x68, 4),
    (0x6c, 4),
    (0x70, 4),
    (0x74, 4),
    (0x78, 4),
    (0x7c, 4),
    (0x80, 4),
    (0x84, 4),
    (0x88, 4),
    (0x8c, 4),
    (0x90, 4),
    (0x94, 4),
    (0x98, 4),
    (0x9c, 4),
    (0xac, 4),
    (0xb0, 4),
    (0xb4, 4),
    (0xb8, 4),
    (0x14, 4),
    (0x18, 4),
    (0xc8, 4),
    (0xcc, 4),
    (0xd0, 4),
    (0xd4, 4),
    (0xdc, 1),
    (0xdd, 1),
    (0xde, 1),
    (0xdf, 1),
    (0xe0, 1),
    (0xe2, 1),
    (0xe1, 1),
    (0xc0, 1),
    (0xc1, 1),
    (0xc4, 4),
    (0xa0, 0xc),
    (0xbc, 4),
];

// Translated from 009e8f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetailedActorPathHandler::SaveGame` (Xbox PDB): writes the fields of
/// [`SAVED_FIELDS`] (`00865e50`), the door's form ID, then the avoid list's
/// non-null actors as form IDs inside a variable-sized value holding their
/// count.
pub fn detailed_actor_path_handler_save_game(e: &mut Engine, this: Ptr<H>, buffer: Ptr) {
    for (offset, size) in SAVED_FIELDS {
        e.call(SAVE_BYTES, &args![buffer, this.addr() + offset, size, 0u32]);
    }
    let door = e.get(this, H::pCurrentDoor);
    e.call(SAVE_FORM_ID, &args![buffer, door, 0u32]);
    let mut count = 0u32;
    let start = e.call(SAVE_START_VARIABLE_SIZED, &args![buffer]).u32();
    let mut node = this.addr() + H::ActorAvoidList.off;
    while node != 0 {
        let item = e.call(LIST_NODE_ITEM, &args![node]).u32();
        let actor = e.mem.u32(item);
        if actor != 0 {
            e.call(SAVE_FORM_ID, &args![buffer, actor, 0u32]);
            count += 1;
        }
        node = e.call(LIST_NEXT, &args![node]).u32();
    }
    e.call(SAVE_VARIABLE_SIZED, &args![buffer, count, start]);
}

// Translated from 009e9390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetailedActorPathHandler::LoadGame` (Xbox PDB): reads the fields
/// `SaveGame` writes (`00864980`), the tether distances only from save
/// version 4, the initial-rotation fields from 12 and
/// `LastFrameMinMoveDelta` and `fLastRotationApplied` from 18 (the version
/// byte from the buffer's slot 0); then the door's form ID and the avoid
/// list's form IDs, each pushed to the front of the list.
pub fn detailed_actor_path_handler_load_game(e: &mut Engine, this: Ptr<H>, buffer: Ptr) {
    let load = |e: &mut Engine, fields: &[(u32, u32)]| {
        for (offset, size) in fields {
            e.call(LOAD_BYTES, &args![buffer, this.addr() + offset, *size]);
        }
    };
    let version = |e: &mut Engine| e.vcall(buffer.addr(), LOAD_SLOT_VERSION, &[]).u8();
    load(e, &SAVED_FIELDS[..25]);
    if version(e) >= 4 {
        load(e, &SAVED_FIELDS[25..27]);
    }
    load(e, &SAVED_FIELDS[27..38]);
    if version(e) >= 12 {
        load(e, &SAVED_FIELDS[38..41]);
    }
    if version(e) >= 18 {
        load(e, &SAVED_FIELDS[41..]);
    }
    e.call(
        LOAD_FORM_ID,
        &args![buffer, this.addr() + H::pCurrentDoor.off],
    );
    let count = e.call(LOAD_VARIABLE_SIZED, &args![buffer]).u32();
    let list = this.addr() + H::ActorAvoidList.off;
    for _ in 0..count {
        e.with_stack(4, |e, id| {
            e.mem.set_u32(id.addr(), 0);
            e.call(LOAD_FORM_ID, &args![buffer, id]);
            e.call(LIST_PUSH_FRONT, &args![list, id]);
        });
    }
}

/// A loaded form ID turned into the form (`004839c0`) and cast with
/// `__RTDynamicCast` from `TESForm` to `target`; 0 stays 0.
fn resolve_form(e: &mut Engine, id: u32, target: u32) -> u32 {
    if id == 0 {
        return 0;
    }
    let form = e.call(LOOKUP_FORM, &args![id]).u32();
    e.call(
        RT_DYNAMIC_CAST,
        &args![form, 0u32, TYPE_TES_FORM, target, 0u32],
    )
    .u32()
}

// Translated from 009e9760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetailedActorPathHandler::InitLoadGame` (Xbox PDB): turns the loaded
/// form IDs into pointers: the door as a `TESObjectREFR`, each avoid list
/// item as an `Actor` (an item that is 0 or not an actor is removed: the
/// first node by popping the front, a later one through the previous
/// node), then marks the door targeted (`TESObjectREFR::SetTargeted`). The
/// stack word is not read.
pub fn detailed_actor_path_handler_init_load_game(e: &mut Engine, this: Ptr<H>, _unused_1: u32) {
    let door = e.get(this, H::pCurrentDoor).addr();
    let door = resolve_form(e, door, TYPE_TES_OBJECT_REFR);
    e.set(this, H::pCurrentDoor, Ptr::new(door));
    let mut node = this.addr() + H::ActorAvoidList.off;
    let mut previous = 0u32;
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let item = e.call(LIST_NODE_ITEM, &args![node]).u32();
        let id = e.mem.u32(item);
        let remove = if id == 0 {
            true
        } else {
            let actor = resolve_form(e, id, TYPE_ACTOR);
            if actor == 0 {
                true
            } else {
                e.with_stack(4, |e, value| {
                    e.mem.set_u32(value.addr(), actor);
                    e.call(LIST_SET_ITEM, &args![node, value]);
                });
                false
            }
        };
        if !remove {
            previous = node;
            node = e.call(LIST_NEXT, &args![node]).u32();
        } else if previous != 0 {
            let item = e.call(LIST_NODE_ITEM, &args![node]).u32();
            e.call(LIST_REMOVE, &args![previous, item]);
            node = e.call(LIST_NEXT, &args![previous]).u32();
        } else {
            e.call(LIST_POP_FRONT, &args![node]);
        }
    }
    let door = e.get(this, H::pCurrentDoor);
    if !door.is_null() {
        e.call(REFERENCE_SET_TARGETED, &args![door, 1u32]);
    }
}

// Translated from 009e98d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes element `index` out of an `NiTArray` of pointers and returns it:
/// the slot is cleared, `m_usESize` drops when it held one and `m_usSize`
/// when it was the last; an index at or past `m_usSize` returns 0.
pub fn fn_009e98d0(e: &mut Engine, array: Ptr<NiTArray>, index: u32) -> u32 {
    if index >= e.get(array, NiTArray::m_usSize) as u32 {
        return 0;
    }
    let slot = e
        .get(array, NiTArray::m_pBase)
        .wrapping_add(index.wrapping_mul(4));
    let element = e.mem.u32(slot);
    e.mem.set_u32(slot, 0);
    if element != 0 {
        let used = e.get(array, NiTArray::m_usESize);
        e.set(array, NiTArray::m_usESize, used.wrapping_sub(1));
    }
    let size = e.get(array, NiTArray::m_usSize);
    if index == size.wrapping_sub(1) as u32 {
        e.set(array, NiTArray::m_usSize, size.wrapping_sub(1));
    }
    element
}

// Translated from 009e9950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<DetailedActorPathHandler::ActorAvoidanceData,1024>`
/// constructor: its vtable (`01092640`) and an empty buffer (`009e9b20`
/// with 0, 0).
pub fn fn_009e9950(e: &mut Engine, array: Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray> {
    e.mem.set_u32(array.addr(), AVOID_ARRAY_VTABLE);
    fn_009e9b20(e, array, 0, 0);
    array
}

// Translated from 009e9980 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same array's destructor: its vtable again, then the base's
/// (`008454f0` with 1).
pub fn fn_009e9980(e: &mut Engine, array: Ptr<BSSimpleArray>) {
    e.mem.set_u32(array.addr(), AVOID_ARRAY_VTABLE);
    e.call(SIMPLE_ARRAY_DESTRUCT, &args![array, 1u32]);
}

// Translated from 009e99a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same array's `Add`: grows it by one (`00845460`, the new index),
/// constructs that element (`009e9a50`) and copies `data` into it
/// (`009e99f0`); returns the index.
pub fn fn_009e99a0(e: &mut Engine, array: Ptr<BSSimpleArray>, data: Ptr) -> u32 {
    let index = e.call(SIMPLE_ARRAY_GROW, &args![array]).u32();
    let element = |e: &Engine| {
        e.get(array, BSSimpleArray::pBuffer)
            .wrapping_add(index.wrapping_mul(8))
    };
    let at = element(e);
    e.call(AVOID_DATA_CONSTRUCT_RANGE, &args![array, at, 1u32]);
    let at = element(e);
    fn_009e99f0(e, Ptr::new(at), data);
    index
}

// Translated from 009e99f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetailedActorPathHandler::ActorAvoidanceData` assignment: `pActor`
/// (+0) and `fDetection` (+4, through the FPU); returns `this`.
pub fn fn_009e99f0(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    let actor = e.mem.u32(other.addr());
    e.mem.set_u32(this.addr(), actor);
    let detection = x87_quiet_bits(e.mem.u32(other.addr() + 4));
    e.mem.set_u32(this.addr() + 4, detection);
    this
}

// Translated from 009e9a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<DetailedActorPathHandler::ActorAvoidanceData,1024>::`
/// `scalar deleting destructor'` (Xbox PDB): the destructor (`009e9980`),
/// then `operator delete` with bit 0 of `flags`; returns the array.
pub fn bs_simple_array_actor_avoidance_data_scalar_deleting_destructor(
    e: &mut Engine,
    array: Ptr<BSSimpleArray>,
    flags: u32,
) -> Ptr<BSSimpleArray> {
    fn_009e9980(e, array);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![array]);
    }
    array
}

// Translated from 009e9b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same array's buffer set-up: empty, then a buffer of
/// `max(reserve, size)` elements from its slot +4 when that is not 0, and
/// `size` elements constructed (`009e9a50`) when `size` is not 0.
pub fn fn_009e9b20(e: &mut Engine, array: Ptr<BSSimpleArray>, reserve: u32, size: u32) {
    e.set(array, BSSimpleArray::pBuffer, 0);
    e.set(array, BSSimpleArray::iSize, 0);
    e.set(array, BSSimpleArray::iReservedSize, 0);
    let capacity = reserve.max(size);
    if capacity != 0 {
        let buffer = e
            .vcall(array.addr(), SIMPLE_ARRAY_SLOT_ALLOCATE, &args![capacity])
            .u32();
        e.set(array, BSSimpleArray::pBuffer, buffer);
        e.set(array, BSSimpleArray::iReservedSize, capacity);
    }
    if size != 0 {
        let buffer = e.get(array, BSSimpleArray::pBuffer);
        e.call(AVOID_DATA_CONSTRUCT_RANGE, &args![array, buffer, size]);
        e.set(array, BSSimpleArray::iSize, size);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00401290, fn_00401290(u32, u32) -> bool),
        entry!(
            0x0070ec90,
            detailed_actor_path_handler_get_current_node_index(Ptr<H>) -> u32
        ),
        entry!(
            0x009dff40,
            detailed_actor_path_handler_detailed_actor_path_handler(Ptr<H>, Ptr, Ptr) -> Ptr<H>
        ),
        entry!(
            0x009e0160,
            detailed_actor_path_handler_scalar_deleting_destructor(Ptr<H>, u32) -> Ptr<H>
        ),
        entry!(
            0x009e0190,
            fn_009e0190(Ptr<H>, Ptr, Ptr, Ptr, Ptr) -> Ptr<H>
        ),
        entry!(0x009e03e0, fn_009e03e0(Ptr<H>)),
        entry!(
            0x009e0470,
            detailed_actor_path_handler_init(Ptr<H>, f32, f32) -> u32
        ),
        entry!(
            0x009e0930,
            detailed_actor_path_handler_init_rotation_only(Ptr<H>, f32, bool) -> bool
        ),
        entry!(
            0x009e0a00,
            detailed_actor_path_handler_update(Ptr<H>, f32) -> u32
        ),
        entry!(0x009e1640, fn_009e1640(Ptr, f32)),
        entry!(0x009e16c0, fn_009e16c0(Ptr)),
        entry!(
            0x009e16e0,
            detailed_actor_path_handler_no_path_update(Ptr<H>, f32) -> u32
        ),
        entry!(
            0x009e1d50,
            detailed_actor_path_handler_update_while_player_waits(Ptr<H>, u32) -> u32
        ),
        entry!(0x009e20a0, fn_009e20a0(Ptr) -> bool),
        entry!(
            0x009e20c0,
            detailed_actor_path_handler_handle_door_opening(Ptr<H>) -> u32
        ),
        entry!(
            0x009e22d0,
            detailed_actor_path_handler_is_door_in_path(Ptr<H>, u32) -> bool
        ),
        entry!(0x009e2660, fn_009e2660(Ptr, u32) -> bool),
        entry!(0x009e2690, fn_009e2690(Ptr, u32) -> Ptr),
        entry!(0x009e26d0, door_in_path_filter_can_cross_edge(Ptr, u32, u16, u32, u32, u16, u32) -> bool),
        entry!(0x009e27e0, fn_009e27e0(Ptr<H>) -> bool),
        entry!(
            0x009e2830,
            detailed_actor_path_handler_get_distance_traveled(Ptr<H>) -> f32
        ),
        entry!(
            0x009e2860,
            detailed_actor_path_handler_get_distance_remaining(Ptr<H>) -> f32
        ),
        entry!(
            0x009e28a0,
            detailed_actor_path_handler_get_percent_traveled(Ptr<H>) -> f32
        ),
        entry!(
            0x009e2900,
            detailed_actor_path_handler_compute_approximate_location(Ptr<H>, Ptr, f32) -> Ptr
        ),
        entry!(
            0x009e2a80,
            detailed_actor_path_handler_is_waiting_for_avoidance(Ptr<H>) -> bool
        ),
        entry!(0x009e2aa0, fn_009e2aa0(Ptr<H>, f32) -> bool),
        entry!(
            0x009e3110,
            detailed_actor_path_handler_compute_path_passes_through_players_area(Ptr<H>) -> bool
        ),
        entry!(0x009e32d0, fn_009e32d0(Ptr) -> u32),
        entry!(
            0x009e32f0,
            detailed_actor_path_handler_get_move_delta(Ptr<H>, Ptr) -> bool
        ),
        entry!(
            0x009e3360,
            detailed_actor_path_handler_force_move_mode(Ptr<H>, u32)
        ),
        entry!(
            0x009e3380,
            detailed_actor_path_handler_handle_previous_path_transition(Ptr<H>, Ptr<H>)
        ),
        entry!(0x009e3520, fn_009e3520(Ptr<H>, f32)),
        entry!(0x009e3560, fn_009e3560(Ptr<H>, u32)),
        entry!(0x009e3970, fn_009e3970(Ptr) -> f32),
        entry!(
            0x009e3990,
            detailed_actor_path_handler_get_next_tether_point(Ptr<H>, Ptr) -> bool
        ),
        entry!(
            0x009e39c0,
            detailed_actor_path_handler_clear_movement(Ptr<H>)
        ),
        entry!(
            0x009e3a70,
            detailed_actor_path_handler_get_next_node_location(Ptr<H>, Ptr) -> Ptr
        ),
        entry!(
            0x009e3b60,
            detailed_actor_path_handler_get_node_location(Ptr<H>, u32, Ptr) -> bool
        ),
        entry!(
            0x009e3be0,
            detailed_actor_path_handler_is_movement_blocked(Ptr<H>) -> bool
        ),
        entry!(
            0x009e3c00,
            detailed_actor_path_handler_update_tether_position(Ptr<H>, f32)
        ),
        entry!(0x009e3d50, fn_009e3d50(Ptr<H>, u32)),
        entry!(
            0x009e4450,
            detailed_actor_path_handler_compute_heading_and_looking(Ptr<H>, Ptr, Ptr)
        ),
        entry!(0x009e4800, fn_009e4800(Ptr<H>, f32, Ptr) -> f32),
        entry!(
            0x009e4c60,
            detailed_actor_path_handler_update_looking(Ptr<H>, f32, f32)
        ),
        entry!(0x009e4cf0, fn_009e4cf0(Ptr<H>, f32) -> bool),
        entry!(0x009e5480, fn_009e5480(Ptr<H>)),
        entry!(0x009e54a0, fn_009e54a0() -> bool),
        entry!(0x009e55e0, fn_009e55e0(Ptr) -> u32),
        entry!(0x009e5690, fn_009e5690(Ptr, Ptr, u32, Ptr, Ptr<H>, Ptr)),
        entry!(0x009e5ab0, fn_009e5ab0(Ptr, u32, f32) -> Ptr),
        entry!(0x009e5ae0, fn_009e5ae0(Ptr<H>, f32) -> u32),
        entry!(0x009e6870, fn_009e6870(Ptr, Ptr, f32, f32, u32) -> Ptr),
        entry!(0x009e68c0, fn_009e68c0(Ptr<H>)),
        entry!(
            0x009e6900,
            fn_009e6900(Ptr, Ptr<H>, Ptr, Ptr, Ptr, f32, f32, Ptr, bool) -> bool
        ),
        entry!(
            0x009e6ba0,
            detailed_actor_path_handler_set_wait_for_updated_path(Ptr<H>, Ptr)
        ),
        entry!(0x009e71b0, fn_009e71b0(Ptr, Ptr, Ptr<H>)),
        entry!(
            0x009e7550,
            detailed_actor_path_handler_check_for_end_rotate(Ptr<H>) -> bool
        ),
        entry!(
            0x009e7610,
            detailed_actor_path_handler_init_rotation_mode(Ptr<H>, f32, bool, bool) -> bool
        ),
        entry!(0x009e77d0, fn_009e77d0(Ptr<H>, f32)),
        entry!(
            0x009e7d70,
            detailed_actor_path_handler_update_rotation(Ptr<H>, f32) -> f32
        ),
        entry!(
            0x009e8000,
            detailed_actor_path_handler_check_goal_reached(Ptr<H>) -> u32
        ),
        entry!(0x009e8670, fn_009e8670(Ptr) -> u8),
        entry!(
            0x009e8690,
            detailed_actor_path_handler_handle_deleted_reference(Ptr<H>, Ptr)
        ),
        entry!(0x009e8790, fn_009e8790(Ptr<H>)),
        entry!(
            0x009e8800,
            detailed_actor_path_handler_warp_to_next_node(Ptr<H>) -> bool
        ),
        entry!(
            0x009e8b50,
            detailed_actor_path_handler_print_current_status(Ptr<H>, Ptr, u32, u32)
        ),
        entry!(
            0x009e8f80,
            detailed_actor_path_handler_save_game(Ptr<H>, Ptr)
        ),
        entry!(
            0x009e9390,
            detailed_actor_path_handler_load_game(Ptr<H>, Ptr)
        ),
        entry!(
            0x009e9760,
            detailed_actor_path_handler_init_load_game(Ptr<H>, u32)
        ),
        entry!(0x009e98d0, fn_009e98d0(Ptr<NiTArray>, u32) -> u32),
        entry!(
            0x009e9950,
            fn_009e9950(Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>
        ),
        entry!(0x009e9980, fn_009e9980(Ptr<BSSimpleArray>)),
        entry!(0x009e99a0, fn_009e99a0(Ptr<BSSimpleArray>, Ptr) -> u32),
        entry!(0x009e99f0, fn_009e99f0(Ptr, Ptr) -> Ptr),
        entry!(
            0x009e9a20,
            bs_simple_array_actor_avoidance_data_scalar_deleting_destructor(
                Ptr<BSSimpleArray>,
                u32,
            )
                -> Ptr<BSSimpleArray>
        ),
        entry!(0x009e9b20, fn_009e9b20(Ptr<BSSimpleArray>, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Fake addresses for the doubles behind the test vtables.
    const ACTOR_VTABLE: u32 = 0x7100_0000;
    const MOVER_VTABLE: u32 = 0x7100_1000;
    const OBJECT_VTABLE: u32 = 0x7100_2000;
    const FAKE: u32 = 0x7200_0000;

    // Test actor fields (the doubles read them; not the game's layout).
    const A_LOCATION: u32 = 0x30;
    const A_HEADING: u32 = 0x80;
    const A_RADIUS: u32 = 0x84;
    const A_TURN_SPEED: u32 = 0x88;
    const A_CAN_MOVE: u32 = 0x90;
    const A_LOOKING: u32 = 0x94;
    const A_SPEED: u32 = 0x98;
    const A_CONTROLLER: u32 = 0x9c;
    const A_SLOT_214: u32 = 0xa0;
    const A_PROCESS: u32 = 0xa4;
    const A_FLAG_21C: u32 = 0xa8;

    // Test path fields.
    const P_LENGTH: u32 = 0x44;
    const P_END_NODE: u32 = 0x48;
    const P_NODES: u32 = 0x100;

    fn ret_u32(v: u32) -> Ret {
        Ret {
            eax: v,
            ..Ret::default()
        }
    }

    fn ret_f(v: f64) -> Ret {
        Ret {
            st0: v,
            ..Ret::default()
        }
    }

    fn f(a: &[u32], i: usize) -> f32 {
        f32::from_bits(a[i])
    }

    fn point(e: &Engine, p: u32) -> [f32; 3] {
        [e.mem.f32(p), e.mem.f32(p + 4), e.mem.f32(p + 8)]
    }

    fn set_point(e: &mut Engine, p: u32, v: [f32; 3]) {
        for (i, c) in v.iter().enumerate() {
            e.mem.set_f32(p + 4 * i as u32, *c);
        }
    }

    /// Every external callee and later-session function this unit calls.
    const CALLEES: &[u32] = &[
        NIPOINTER_GET,
        NIPOINTER_ASSIGN,
        NIPOINTER_FROM_RAW,
        NIPOINTER_RELEASE,
        DEFAULT_CONSTRUCT,
        POINT_CONSTRUCT,
        LIST_CONSTRUCT,
        LIST_CLEAR,
        LIST_DESTRUCT,
        BASE_CONSTRUCT,
        BASE_CONSTRUCT_WITH_PATH,
        BASE_DESTRUCT,
        OPERATOR_DELETE,
        OPERATOR_NEW,
        POINTER_EQUALS,
        PATH_NODE_COUNT,
        PATH_BYTE_40,
        PATH_END_NODE,
        PATH_LENGTH,
        FLAGS_BOTH_LOW,
        FLAGS_BIT_2,
        FLOAT_AT_74,
        FLOAT_MAX_OF,
        BYTE_AT_104,
        MOVER_FACE_TARGET_POINT,
        MOVER_GET_LOOK_AT_POINT,
        MOVER_MOVE_MODE_BASE,
        MOVER_BYTE_70,
        MOVER_BYTE_71,
        MOVER_REQUEST_PATH,
        EMPTY_FUNCTION,
        POINT_SUBTRACT_ASSIGN,
        POINT_SUBTRACT,
        POINT_SCALE,
        POINT_LENGTH,
        POINT_LENGTH_XY,
        POINT_LENGTH_SQUARED_XY,
        POINT_NORMALIZE,
        POINT_DOT,
        MATRIX_TIMES_POINT,
        FLOAT_ABS,
        Z_ANGLE_FROM_VECTOR,
        CLAMP_ANGLE,
        ANGLE_DIFFERENCE,
        SIN_COS,
        ACTOR_CAN_MOVE,
        ACTOR_GET_CHAR_CONTROLLER,
        PLAYER_IS_SLEEPING,
        CONTROLLER_STATE,
        CONTROLLER_FALL_TIME,
        SETTING_VALUE,
        SETTING_BOOL,
        REQUEST_CONSTRUCT,
        REQUEST_COPY_TO,
        LOCATION_ASSIGN,
        PATH_LAST_NODE,
        PLUS_4,
        PLUS_34,
        LOCATION_FLAG_2,
        COPY_POINT_AT_4,
        ACTOR_BASE_IS_IMMOBILE,
        ACTOR_BASE_FORM,
        ACTOR_BYTE_1B0,
        ACTOR_IN_DIALOGUE_WITH_PLAYER,
        FIRING_LOCATION,
        ANGLE_TO_PROJECTED_POINT,
        ACTOR_GET_RADIUS,
        CHECK_GOAL_REACHED,
        RESET_MOVE_DELTA,
        ACTOR_BYTE_14D,
        ACTOR_CAN_PASS,
        ACTOR_COMPUTE_MOVE_DELTA,
        ACTOR_FLAG_008B3C50,
        ACTOR_GET_ANIM_GROUP_DURATION,
        ACTOR_GET_CURRENT_SPEED,
        ACTOR_GET_DESTINATION,
        ACTOR_GET_FORWARD_LENGTH,
        ACTOR_GET_HEIGHT,
        ACTOR_GET_LOOKING,
        ACTOR_SET_LOOKING,
        ACTOR_GET_RUN_SPEED,
        ACTOR_GET_WALK_SPEED,
        ACTOR_IS_MOVING,
        ACTOR_MOVE_FLAGS,
        ACTOR_MOVE_DELTA,
        ACTOR_MOVE_TO_PATH_END,
        ACTOR_PROCESS,
        ACTOR_STATE_IS_3,
        ACTOR_STATE_IS_5,
        ACTOR_LIST_CONSTRUCT,
        ACTOR_LIST_DESTRUCT,
        ACTOR_LIST_ADD,
        ANGLE_TO_PROJECTED_POINT_BETWEEN,
        ANIMATION_GROUP_LOADED,
        ANIMATION_SHOULD_BE_MOVING,
        ARRAY_CLEAR,
        ARRAY_SET_SIZE,
        ARRAY_ADD,
        ARRAY_ELEMENT,
        ARRAY_ELEMENT_8,
        ARRAY_FIND,
        ARRAY_IS_EMPTY,
        ARRAY_SIZE,
        AVOID_NODE_ARRAY_CONSTRUCT,
        AVOID_NODE_ARRAY_DESTRUCT,
        AVOID_NODE_ARRAY_NEW,
        AVOID_NODE_ARRAY_ADD,
        AVOID_NODE_RADIUS,
        AVOID_NODE_SET_RADIUS,
        AVOID_NODE_FLOAT_1C,
        AVOID_NODE_SET_FLOAT_1C,
        BS_WRAP,
        CALENDAR_GET_TIME_SCALE,
        CELL_IS_INTERIOR,
        CHECK_LINE_OF_SIGHT,
        CONTROLLER_FLAGS_TEST,
        CONTROLLER_HK_POSITION,
        CONTROLLER_FLOAT_5FC,
        CONTROLLER_SUPPORT,
        COPY_16_BYTES,
        HK_VECTOR_TO_POINT,
        FILTER_BASE_CONSTRUCT,
        DOUBLE_ABS,
        DWORD_AT_18,
        FIND_REFERENCE_FOR_3D,
        FLOAT_AT_68,
        FLOAT_CLAMP,
        FLOAT_MODULO,
        FORM_DWORD_C,
        FRAME_FLOAT_C,
        FTOL,
        GET_OPEN_STATE,
        LINKED_REFERENCE,
        LIST_CONTAINS,
        LIST_COUNT,
        LIST_IS_EMPTY,
        LIST_NEXT,
        LIST_PUSH_FRONT,
        LIST_POP_FRONT,
        LOCATION_COPY,
        LOCATION_DEFAULT,
        LOCATION_FROM_REFERENCE,
        LOCATION_FROM_POINT,
        LOCATION_FROM_POINT_IN_CELL,
        LOCATION_DESTRUCT,
        LOCATION_GET_CELL,
        LOCATION_GET_WORLDSPACE,
        LOCATION_GET_NAVMESH_AND_TRIANGLE,
        LOCATION_RESOLVE_NAVMESH,
        LOCATION_WITHIN,
        LOOKUP_FORM,
        MATRIX_MAKE_X_ROTATION,
        NAVMESH_ADD_AVOID_NODE,
        NAVMESH_CLOSED_DOOR_INFO,
        NAVMESH_GET_TRIANGLE,
        NAVMESH_STATIC_AVOID_NODES,
        TRIANGLE_HAS_FLAG,
        NAVMESH_Z_FOR_LOCATION,
        NIPOINTER_CONSTRUCT_NULL,
        NIPOINTER_COPY,
        NIPOINTER_DESTRUCT,
        PATH_ADVANCE,
        PATH_DISTANCE_AT,
        PATH_IS_USABLE,
        PATH_LOCATION_AFTER,
        PATH_NODE,
        PATH_NODE_COUNT_2,
        PATH_PARAMETER_AT,
        PATH_POINT_AT,
        PLUS_2C,
        PLUS_80,
        PLUS_C,
        POINT_ADD,
        POINT_DIVIDE,
        POINT_LENGTH_SQUARED,
        POINT_UNITIZE,
        POINT_TIMES_MATRIX,
        PROCESS_FLAG_4000,
        PROCESS_FLAG_100000,
        REFERENCE_ACTIVATE,
        REFERENCE_SET_TARGETED,
        REFERENCE_GET_WORLDSPACE,
        REFERENCE_WATER_HEIGHT,
        REFERENCE_PARENT_CELL,
        REFERENCE_POSITION,
        REQUEST_AVOID_NODES,
        REQUEST_SET_AVOID_NODES,
        REQUEST_BYTE_98,
        REQUEST_BYTE_99,
        REQUEST_BYTE_AC,
        REQUEST_FLOAT_8C,
        REQUEST_SET_BYTE_9C,
        REQUEST_SET_INITIAL_HEADING,
        REQUEST_SET_ORIGIN,
        SEGMENT_CIRCLE_INTERSECTION,
        SIGNED_BYTE_AT_20,
        STAMP_OF,
        SUPPORT_OBJECT,
        TES_GET_WORLDSPACE,
        TES_INTERIOR_CELL,
        TRIG_004F6E40,
        WORLDSPACE_CELL_AT,
        AVOID_ARRAY_CONSTRUCT,
        AVOID_ARRAY_DESTRUCT,
        AVOID_ARRAY_ADD,
        LOCATION_GET_CELL,
        PATH_NODE_BY_INDEX,
        PATHFIND_006D5340,
        REQUEST_FLOAT_70,
        FLOAT_MIN,
        HAS_NAME,
        REFERENCE_NAME,
        LOG_MESSAGE,
        ACTOR_MEASURE_008BE350,
        MOVER_MOVE_ACTOR_TO_LOCATION,
        ARRAY_COMPACT,
        LIST_REMOVE,
        LIST_SET_ITEM,
        STRING_COPY,
        STRING_APPEND,
        STRING_FORMAT,
        SAVE_BYTES,
        SAVE_FORM_ID,
        SAVE_START_VARIABLE_SIZED,
        SAVE_VARIABLE_SIZED,
        LOAD_BYTES,
        LOAD_FORM_ID,
        LOAD_VARIABLE_SIZED,
        RT_DYNAMIC_CAST,
        SIMPLE_ARRAY_DESTRUCT,
        SIMPLE_ARRAY_GROW,
        AVOID_DATA_CONSTRUCT_RANGE,
    ];

    /// The data the code reads from the exe, with the exe's values.
    // The values are the exe's float constants as written, not std approximations.
    #[allow(clippy::approx_constant)]
    fn set_data(e: &mut Engine) {
        for page in [
            0x0101_1000,
            0x0101_2000,
            0x0101_6000,
            0x0101_7000,
            0x0101_8000,
            0x0101_a000,
            0x0101_d000,
            0x0101_e000,
            0x0101_f000,
            0x0102_0000,
            0x0102_1000,
            0x0102_3000,
            0x0102_4000,
            0x0102_6000,
            0x0102_8000,
            0x0102_9000,
            0x0102_b000,
            0x0102_e000,
            0x0102_f000,
            0x0103_1000,
            0x0103_5000,
            0x0104_e000,
            0x0106_4000,
            0x0106_b000,
            0x0106_e000,
            0x0107_4000,
            0x0107_d000,
            0x0108_b000,
            0x0109_2000,
            0x011a_3000,
            0x011a_5000,
            0x011a_9000,
            0x011c_d000,
            0x011d_e000,
            0x011e_0000,
            0x011f_2000,
            0x011f_4000,
            0x011f_6000,
        ] {
            e.map(page, 0x1000);
        }
        let floats: &[(u32, f32)] = &[
            (FLOAT_MAX, f32::MAX),
            (MINUS_ONE_FLOAT, -1.0),
            (TETHER_MIN_MOVE, 10.0),
            (TETHER_DIST_FROM_ACTOR, 20.0),
            (ONE_TENTH_FLOAT, 0.1),
            (GOAL_RADIUS_PATH_FLAG, 900.0),
            (GOAL_RADIUS_BOTH_FLAGS, 6400.0),
            (TURN_THRESHOLD, 0.017_453_292),
            (PI_FLOAT, std::f32::consts::PI),
            (MINUS_PI_FLOAT, -std::f32::consts::PI),
            (ANGLE_LEFT, -std::f32::consts::FRAC_PI_2),
            (ANGLE_RIGHT, std::f32::consts::FRAC_PI_2),
            (TWO_PI_FLOAT, std::f32::consts::TAU),
            (SCALE_SLOWEST, 0.05),
            (SCALE_SLOW, 0.2),
            (SCALE_NEAR_END, 0.5),
            (MIN_TURN_FACTOR, 0.025),
            (TURN_THRESHOLD_EXACT, 0.017_453_292),
            (TURN_THRESHOLD_RUNNING, 0.523_598_8),
            (AVOID_RADIUS_WALK, 1.5),
            (AVOID_RADIUS_RUN, 1.75),
            (AVOID_NODE_WEIGHT, 2.0),
            (AVOID_FACTOR_WALK, 0.707_106_8),
            (AVOID_FACTOR_RUN, 0.866),
            (GOAL_MIN_DISTANCE, 25.0),
            (WARP_MIN_DISTANCE, 64.0),
            (WARP_TETHER_DISTANCE, 128.0),
        ];
        for (address, value) in floats {
            e.set_global(*address, *value);
        }
        let doubles: &[(u32, f64)] = &[
            (ZERO_DOUBLE, 0.0),
            (ONE_DOUBLE, 1.0),
            (MINUS_ONE_DOUBLE, -1.0),
            (NINETY_FIVE_HUNDREDTHS, 0.949_999_988_079_071),
            (FALL_TIME_STOP, 0.4),
            (FALL_TIME_LANDED, 0.65),
            (QUARTER, 0.25),
            (HUNDRED, 100.0),
            (DEGREES_TO_RADIANS, 0.017_453_292_384_743_69),
            (STEEP_PITCH, 1.535_889_744_758_606),
            (TWO_PI_DOUBLE, std::f64::consts::TAU),
            (HALF_PI, std::f64::consts::FRAC_PI_2),
            (HALF, 0.5),
            (TWO_DOUBLE, 2.0),
            (THREE, 3.0),
            (FOUR, 4.0),
            (FIVE, 5.0),
            (TEN, 10.0),
            (FORTY_FIVE, 45.0),
            (EIGHTH, 0.125),
            (FIFTH, 0.2),
            (THIRD, 0.33),
            (SPLINE_STEP, 0.1),
            (RADIANS_TO_DEGREES, 57.295_780_181_884_77),
            (FIFTEEN_DEGREES, 0.261_799_395_084_381_1),
            (LARGE_TURN, 135.0),
            (BLOCK_TIME, 1.5),
            (RADIUS_LIMIT, 128.0),
            (HEIGHT_LIMIT, 180.0),
            (AVOID_RANGE_SQUARED, 1.0e6),
            (NO_WATER_HEIGHT, -3.402_823_466_385_288_6e38),
            (LOOKING_RATE, 1.308_996_9),
            (TURN_SLOWEST, 2.700_000_047_683_716),
            (TURN_SLOW, 1.600_000_023_841_858),
            (SECONDS_PER_HOUR, 3600.0),
            (END_UPDATE_RANGE_SQUARED, 25600.0),
            (WARP_END_FACTOR, f64::from(0.6f32)),
        ];
        for (address, value) in doubles {
            e.set_global(*address, *value);
        }
        // Settings hold their value at +4 (`SETTING_VALUE` returns that).
        let settings: &[(u32, f32)] = &[
            (SETTING_AVOIDANCE_TIMER, 2.0),
            (SETTING_TURN_RATE_A, 1.0),
            (SETTING_TURN_RATE_B, 1.0),
            (SETTING_TURN_RATE_C, 1.0),
            (SETTING_TURN_RATE_D, 1.0),
            (SETTING_MIN_ROTATE_TIME, 0.5),
            (SETTING_AVOID_CONTROLLER_LIMIT, 1.0),
            (SETTING_AVOID_LOOK_AHEAD, 1.0),
            (SETTING_AVOID_NEAR, 50.0),
            (SETTING_AVOID_THRESHOLD, 1.0),
            (SETTING_AVOID_WAIT_TIME, 3.0),
            (SETTING_AVOID_MIN_TIMER, 4.0),
            (SETTING_AVOID_REPATH_TIMER, 5.0),
            (SETTING_AVOID_NODE_WEIGHT, 6.0),
            (SETTING_PATH_LEAD, 30.0),
        ];
        for (address, value) in settings {
            e.set_global(*address + 4, *value);
        }
        e.set_global(SETTING_AVOIDANCE_ENABLED + 4, 1u8);
        set_point(e, NO_POINT, [0.0, 0.0, 0.0]);
    }

    /// Doubles that behave like the game's small helpers.
    fn helpers(e: &mut Engine) {
        for address in CALLEES {
            e.register(*address, |_, _| Ret::default());
        }
        e.register(NIPOINTER_GET, |e, a| ret_u32(e.mem.u32(a[0])));
        e.register(NIPOINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret_u32(a[0])
        });
        e.register(NIPOINTER_FROM_RAW, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret_u32(a[0])
        });
        e.register(NIPOINTER_COPY, |e, a| {
            let v = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], v);
            ret_u32(a[0])
        });
        e.register(SETTING_VALUE, |_, a| ret_u32(a[0] + 4));
        e.register(SETTING_BOOL, |_, a| ret_u32(a[0] + 4));
        e.register(DEFAULT_CONSTRUCT, |_, a| ret_u32(a[0]));
        e.register(POINT_CONSTRUCT, |e, a| {
            set_point(e, a[0], [f(a, 1), f(a, 2), f(a, 3)]);
            ret_u32(a[0])
        });
        e.register(POINT_SUBTRACT, |e, a| {
            let l = point(e, a[0]);
            let r = point(e, a[2]);
            set_point(e, a[1], [l[0] - r[0], l[1] - r[1], l[2] - r[2]]);
            ret_u32(a[1])
        });
        e.register(POINT_ADD, |e, a| {
            let l = point(e, a[0]);
            let r = point(e, a[2]);
            set_point(e, a[1], [l[0] + r[0], l[1] + r[1], l[2] + r[2]]);
            ret_u32(a[1])
        });
        e.register(POINT_SUBTRACT_ASSIGN, |e, a| {
            let l = point(e, a[0]);
            let r = point(e, a[1]);
            set_point(e, a[0], [l[0] - r[0], l[1] - r[1], l[2] - r[2]]);
            ret_u32(a[0])
        });
        e.register(POINT_SCALE, |e, a| {
            let l = point(e, a[0]);
            let s = f(a, 1);
            set_point(e, a[0], [l[0] * s, l[1] * s, l[2] * s]);
            ret_u32(a[0])
        });
        e.register(POINT_DIVIDE, |e, a| {
            let l = point(e, a[0]);
            let s = f(a, 2);
            set_point(e, a[1], [l[0] / s, l[1] / s, l[2] / s]);
            ret_u32(a[1])
        });
        e.register(POINT_LENGTH, |e, a| {
            let p = point(e, a[0]);
            ret_f(((p[0] * p[0] + p[1] * p[1] + p[2] * p[2]) as f64).sqrt())
        });
        e.register(POINT_LENGTH_SQUARED, |e, a| {
            let p = point(e, a[0]);
            ret_f((p[0] * p[0] + p[1] * p[1] + p[2] * p[2]) as f64)
        });
        e.register(POINT_LENGTH_XY, |e, a| {
            let p = point(e, a[0]);
            ret_f(((p[0] * p[0] + p[1] * p[1]) as f64).sqrt())
        });
        e.register(POINT_LENGTH_SQUARED_XY, |e, a| {
            let p = point(e, a[0]);
            ret_f((p[0] * p[0] + p[1] * p[1]) as f64)
        });
        e.register(POINT_DOT, |e, a| {
            let l = point(e, a[0]);
            let r = point(e, a[1]);
            ret_f((l[0] * r[0] + l[1] * r[1] + l[2] * r[2]) as f64)
        });
        e.register(POINT_NORMALIZE, |e, a| {
            let p = point(e, a[0]);
            let l = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
            if l > 1e-6 {
                set_point(e, a[0], [p[0] / l, p[1] / l, p[2] / l]);
            }
            Ret::default()
        });
        e.register(POINT_UNITIZE, |e, a| {
            let p = point(e, a[0]);
            let l = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
            if l > 1e-6 {
                set_point(e, a[0], [p[0] / l, p[1] / l, p[2] / l]);
            }
            ret_f(l as f64)
        });
        e.register(MATRIX_TIMES_POINT, |e, a| {
            let m: Vec<f32> = (0..9).map(|i| e.mem.f32(a[0] + 4 * i)).collect();
            let p = point(e, a[2]);
            let r = [
                m[0] * p[0] + m[1] * p[1] + m[2] * p[2],
                m[3] * p[0] + m[4] * p[1] + m[5] * p[2],
                m[6] * p[0] + m[7] * p[1] + m[8] * p[2],
            ];
            set_point(e, a[1], r);
            ret_u32(a[1])
        });
        e.register(COPY_POINT_AT_4, |e, a| {
            let p = point(e, a[0] + 4);
            set_point(e, a[1], p);
            ret_u32(a[1])
        });
        e.register(PLUS_4, |_, a| ret_u32(a[0] + 4));
        e.register(PLUS_34, |_, a| ret_u32(a[0] + 0x34));
        e.register(PLUS_C, |_, a| ret_u32(a[0] + 0xc));
        e.register(FLOAT_ABS, |_, a| ret_f(f(a, 0).abs() as f64));
        e.register(FLOAT_MAX_OF, |_, a| ret_f(f(a, 0).max(f(a, 1)) as f64));
        e.register(FLOAT_CLAMP, |e, a| {
            let v = e.mem.f32(a[0]).clamp(f(a, 1), f(a, 2));
            e.mem.set_f32(a[0], v);
            Ret::default()
        });
        e.register(CLAMP_ANGLE, |_, a| ret_f(f(a, 0) as f64));
        e.register(Z_ANGLE_FROM_VECTOR, |e, a| {
            let p = point(e, a[0]);
            ret_f((p[0] as f64).atan2(p[1] as f64))
        });
        e.register(ANGLE_DIFFERENCE, |e, a| {
            let d = f(a, 1) - f(a, 0);
            e.mem.set_f32(a[2], if d == 0.0 { 0.0 } else { d.signum() });
            ret_f(d as f64)
        });
        e.register(SIN_COS, |e, a| {
            let angle = f(a, 0);
            e.mem.set_f32(a[1], angle.sin());
            e.mem.set_f32(a[2], angle.cos());
            Ret::default()
        });
        e.register(FTOL, |_, a| ret_u32(f64::take(a, &mut 0) as i32 as u32));
        e.register(PATH_NODE_COUNT, |e, a| ret_u32(e.mem.u32(a[0] + 8)));
        e.register(PATH_NODE_COUNT_2, |e, a| ret_u32(e.mem.u32(a[0] + 8)));
        e.register(PATH_BYTE_40, |e, a| ret_u32(e.mem.u8(a[0] + 0x40) as u32));
        e.register(PATH_LENGTH, |e, a| ret_f(e.mem.f32(a[0] + P_LENGTH) as f64));
        e.register(PATH_END_NODE, |e, a| ret_u32(e.mem.u32(a[0] + P_END_NODE)));
        e.register(PATH_NODE, |e, a| {
            ret_u32(e.mem.u32(a[0] + P_NODES + 4 * a[1]))
        });
        e.register(FLOAT_AT_68, |e, a| ret_f(e.mem.f32(a[0] + 0x68) as f64));
        e.register(FLOAT_AT_74, |e, a| ret_f(e.mem.f32(a[0] + 0x74) as f64));
        e.register(MOVER_MOVE_MODE_BASE, |e, a| {
            ret_u32(e.mem.u32(a[0] + 0x34) & 0xff00)
        });
        e.register(MOVER_FACE_TARGET_POINT, |e, a| {
            ret_u32(e.mem.u8(a[0] + 0x73) as u32)
        });
        e.register(MOVER_BYTE_70, |e, a| ret_u32(e.mem.u8(a[0] + 0x70) as u32));
        e.register(MOVER_BYTE_71, |e, a| ret_u32(e.mem.u8(a[0] + 0x71) as u32));
        e.register(MOVER_GET_LOOK_AT_POINT, |e, a| {
            if e.mem.u8(a[0] + 0x73) == 0 {
                return ret_u32(0);
            }
            let p = point(e, a[0] + 4);
            set_point(e, a[1], p);
            ret_u32(1)
        });
        e.register(ACTOR_GET_RADIUS, |e, a| {
            ret_f(e.mem.f32(a[0] + A_RADIUS) as f64)
        });
        e.register(ACTOR_CAN_MOVE, |e, a| {
            ret_u32(e.mem.u8(a[0] + A_CAN_MOVE) as u32)
        });
        e.register(ACTOR_GET_CHAR_CONTROLLER, |e, a| {
            ret_u32(e.mem.u32(a[0] + A_CONTROLLER))
        });
        e.register(ACTOR_GET_LOOKING, |e, a| {
            ret_f(e.mem.f32(a[0] + A_LOOKING) as f64)
        });
        e.register(ACTOR_SET_LOOKING, |e, a| {
            e.mem.set_f32(a[0] + A_LOOKING, f(a, 1));
            Ret::default()
        });
        e.register(ACTOR_GET_CURRENT_SPEED, |e, a| {
            ret_f(e.mem.f32(a[0] + A_SPEED) as f64)
        });
        e.register(ACTOR_GET_RUN_SPEED, |e, a| {
            ret_f(e.mem.f32(a[0] + A_SPEED) as f64 * 2.0)
        });
        e.register(ACTOR_GET_WALK_SPEED, |e, a| {
            ret_f(e.mem.f32(a[0] + A_SPEED) as f64)
        });
        e.register(ACTOR_PROCESS, |e, a| ret_u32(e.mem.u32(a[0] + A_PROCESS)));
        e.register(LIST_NEXT, |e, a| ret_u32(e.mem.u32(a[0] + 4)));
        e.register(ARRAY_SIZE, |e, a| ret_u32(e.mem.u32(a[0] + 8)));
        e.register(ARRAY_IS_EMPTY, |e, a| {
            ret_u32((e.mem.u32(a[0] + 8) == 0) as u32)
        });
        e.register(ARRAY_ELEMENT, |e, a| {
            ret_u32(e.mem.u32(a[0] + 4) + 4 * a[1])
        });
        e.register(ARRAY_ELEMENT_8, |e, a| {
            ret_u32(e.mem.u32(a[0] + 4) + 8 * a[1])
        });
        e.register(LIST_IS_EMPTY, |e, a| {
            ret_u32((e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0) as u32)
        });
    }

    /// Doubles for the actor's virtual slots (fields at the test offsets).
    fn actor_slots(e: &mut Engine) {
        let slots = [
            (ACTOR_SLOT_LOCATION, FAKE),
            (ACTOR_SLOT_GET_HEADING, FAKE + 1),
            (ACTOR_SLOT_SET_HEADING, FAKE + 2),
            (ACTOR_SLOT_TURN_SPEED, FAKE + 3),
            (ACTOR_SLOT_21C, FAKE + 4),
            (ACTOR_SLOT_214, FAKE + 5),
            (ACTOR_SLOT_PROCESS, FAKE + 6),
            (ACTOR_SLOT_ANIMATION, FAKE + 7),
            (ACTOR_SLOT_2C0, FAKE + 8),
            (ACTOR_SLOT_42C, FAKE + 7),
            (REFERENCE_SLOT_100, FAKE + 9),
        ];
        e.map(ACTOR_VTABLE, 0x1000);
        for (slot, target) in slots {
            e.mem.set_u32(ACTOR_VTABLE + slot, target);
        }
        e.register(FAKE, |_, a| ret_u32(a[0] + A_LOCATION));
        e.register(FAKE + 1, |e, a| ret_f(e.mem.f32(a[0] + A_HEADING) as f64));
        e.register(FAKE + 2, |e, a| {
            e.mem.set_f32(a[0] + A_HEADING, f(a, 1));
            Ret::default()
        });
        e.register(
            FAKE + 3,
            |e, a| ret_f(e.mem.f32(a[0] + A_TURN_SPEED) as f64),
        );
        e.register(FAKE + 4, |e, a| ret_u32(e.mem.u8(a[0] + A_FLAG_21C) as u32));
        e.register(FAKE + 5, |e, a| ret_u32(e.mem.u32(a[0] + A_SLOT_214)));
        e.register(FAKE + 6, |e, a| ret_u32(e.mem.u32(a[0] + A_PROCESS)));
        e.register(FAKE + 7, |_, _| Ret::default());
        e.register(FAKE + 8, |_, _| Ret::default());
        e.register(FAKE + 9, |_, _| ret_u32(1));
        e.map(MOVER_VTABLE, 0x1000);
        e.mem.set_u32(MOVER_VTABLE + MOVER_SLOT_1C, FAKE + 10);
        e.register(FAKE + 10, |e, a| ret_u32(e.mem.u32(a[0] + 0x34)));
        e.map(OBJECT_VTABLE, 0x1000);
        for slot in [
            PROCESS_SLOT_278,
            OBJECT_SLOT_27C,
            OBJECT_SLOT_280,
            OBJECT_SLOT_128,
        ] {
            e.mem.set_u32(OBJECT_VTABLE + slot, FAKE + 7);
        }
    }

    /// The handler's vtable with the exe's slots (the functions this unit
    /// registers, and empty ones elsewhere).
    fn handler_vtable(e: &mut Engine) {
        let slots: [u32; 33] = [
            0x009e_0160,
            0x009e_0470,
            0x009e_0930,
            0x009e_0a00,
            0x009e_16e0,
            0x009e_1d50,
            0x009e_2830,
            0x009e_28a0,
            0x009e_3a70,
            0x009e_3b60,
            0x009e_2860,
            0x009e_2900,
            0x0070_ec90,
            0x008d_0360,
            0x009e_6ba0,
            0x009e_2a80,
            0x009e_3be0,
            0x009e_3110,
            0x005f_5f80,
            0x009e_32f0,
            0x009e_3990,
            0x009e_3360,
            0x009e_3380,
            0x009e_8690,
            0x009e_8b50,
            0x0048_3710,
            0x008d_05e0,
            0x0048_3710,
            0x009e_39c0,
            0x009e_8800,
            0x009e_8f80,
            0x009e_9390,
            0x009e_9760,
        ];
        e.put_vtable(VTABLE, &slots);
        e.register(0x008d_0360, |_, _| ret_u32(1));
        e.register(0x005f_5f80, |e, a| ret_u32(e.mem.u32(a[0] + 0x60)));
    }

    /// A handler with an actor, a mover, a path and a request, all set up
    /// for the doubles.
    struct World {
        e: Engine,
        this: Ptr<H>,
        actor: u32,
        mover: u32,
        path: u32,
        request: u32,
    }

    fn world() -> World {
        let mut e = Engine::new();
        set_data(&mut e);
        helpers(&mut e);
        actor_slots(&mut e);
        handler_vtable(&mut e);
        e.map(PLAYER, 4);
        let player = e.mem.alloc(0x200);
        e.mem.set_u32(player, ACTOR_VTABLE);
        e.set_global(PLAYER, player);
        let actor = e.mem.alloc(0x200);
        e.mem.set_u32(actor, ACTOR_VTABLE);
        e.mem.set_u8(actor + A_CAN_MOVE, 1);
        e.mem.set_f32(actor + A_RADIUS, 30.0);
        e.mem.set_f32(actor + A_TURN_SPEED, 90.0);
        e.mem.set_f32(actor + A_SPEED, 100.0);
        let mover = e.mem.alloc(0x88);
        e.mem.set_u32(mover, MOVER_VTABLE);
        let path = e.mem.alloc(0x200);
        let request = e.mem.alloc(0xb0);
        let this: Ptr<H> = e.new_object();
        e.mem.set_u32(this.addr(), VTABLE);
        e.set(this, H::pActor, Ptr::new(actor));
        e.set(this, H::pActorMover, Ptr::new(mover));
        e.set(this, H::spPath, Ptr::new(path));
        e.set(this, H::spRequest, Ptr::new(request));
        World {
            e,
            this,
            actor,
            mover,
            path,
            request,
        }
    }

    /// A path node at `location` (the node's `PathingLocation` at +4, its
    /// point at +8).
    fn node(e: &mut Engine, location: [f32; 3]) -> u32 {
        let node = e.mem.alloc(0x40);
        set_point(e, node + 8, location);
        node
    }

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn shared_return_true_ignores_its_word() {
        let mut e = Engine::new();
        assert!(e.call(0x0040_1290, &args![0u32, 7u32]).bool());
    }

    #[test]
    fn current_node_index_is_the_field() {
        let mut w = world();
        w.e.set(w.this, H::iCurrentNodeIndex, 7);
        assert_eq!(w.e.call(0x0070_ec90, &args![w.this]).u32(), 7);
    }

    #[test]
    fn constructor_without_path_sets_the_defaults() {
        let mut w = world();
        let this: Ptr<H> = w.e.new_object();
        w.e.call_log = Some(vec![]);
        let back =
            w.e.call(0x009d_ff40, &args![this, w.actor, w.mover])
                .ptr::<H>();
        assert_eq!(back, this);
        assert_eq!(
            calls(&w.e, BASE_CONSTRUCT),
            vec![vec![this.addr(), w.actor, w.mover]]
        );
        assert_eq!(w.e.mem.u32(this.addr()), VTABLE);
        assert_eq!(w.e.get(this, H::eState), STATE_DONE);
        assert_eq!(w.e.get(this, H::fTetherMinMove), 10.0);
        assert_eq!(w.e.get(this, H::fTetherDistFromActor), 20.0);
        assert_eq!(w.e.get(this, H::fTimeInAnim), 1.0);
        assert_eq!(w.e.get(this, H::fMinRotateTime), -1.0);
        let max = calls(&w.e, POINT_CONSTRUCT);
        assert_eq!(max[0][0], this.addr() + 0x40);
        assert_eq!(f32::from_bits(max[0][1]), f32::MAX);
    }

    #[test]
    fn scalar_deleting_destructor_frees_only_with_bit_0() {
        let mut w = world();
        w.e.call_log = Some(vec![]);
        w.e.call(0x009e_0160, &args![w.this, 0u32]);
        assert!(calls(&w.e, OPERATOR_DELETE).is_empty());
        assert_eq!(calls(&w.e, BASE_DESTRUCT).len(), 1);
        w.e.call(0x009e_0160, &args![w.this, 1u32]);
        assert_eq!(calls(&w.e, OPERATOR_DELETE), vec![vec![w.this.addr()]]);
    }

    #[test]
    fn constructor_with_path_is_done_only_without_path_and_request() {
        let mut w = world();
        w.e.register(POINTER_EQUALS, |e, a| {
            ret_u32((e.mem.u32(a[0]) == a[1]) as u32)
        });
        let holder = w.e.mem.alloc(4);
        let this: Ptr<H> = w.e.new_object();
        w.e.call(0x009e_0190, &args![this, w.actor, w.mover, holder, 0u32]);
        assert_eq!(w.e.get(this, H::eState), STATE_DONE);
        w.e.call(
            0x009e_0190,
            &args![this, w.actor, w.mover, holder, w.request],
        );
        assert_eq!(w.e.get(this, H::eState), 0);
        w.e.mem.set_u32(holder, w.path);
        w.e.call_log = Some(vec![]);
        w.e.call(0x009e_0190, &args![this, w.actor, w.mover, holder, 0u32]);
        assert_eq!(w.e.get(this, H::eState), 0);
        assert_eq!(
            calls(&w.e, BASE_CONSTRUCT_WITH_PATH),
            vec![vec![this.addr(), w.actor, w.mover, holder, 0]]
        );
    }

    #[test]
    fn destructor_body_releases_path_request_and_list() {
        let mut w = world();
        w.e.call_log = Some(vec![]);
        w.e.call(0x009e_03e0, &args![w.this]);
        assert_eq!(w.e.get(w.this, H::spPath), Ptr::NULL);
        assert_eq!(w.e.get(w.this, H::spRequest), Ptr::NULL);
        assert_eq!(calls(&w.e, LIST_CLEAR), vec![vec![w.this.addr() + 0x58]]);
        assert_eq!(calls(&w.e, LIST_DESTRUCT), vec![vec![w.this.addr() + 0x58]]);
        assert_eq!(calls(&w.e, BASE_DESTRUCT), vec![vec![w.this.addr()]]);
    }

    #[test]
    fn init_finishes_paths_shorter_than_two_nodes() {
        let mut w = world();
        set_point(&mut w.e, w.actor + A_LOCATION, [1.0, 2.0, 3.0]);
        w.e.mem.set_u32(w.path + 8, 1);
        let result = w.e.call(0x009e_0470, &args![w.this, 0.0f32, 1.0f32]).u32();
        assert_eq!(result, 5);
        assert_eq!(w.e.get(w.this, H::eState), STATE_DONE);
        assert_eq!(point(&w.e, w.this.addr() + 0x34), [1.0, 2.0, 3.0]);
        assert_eq!(point(&w.e, w.this.addr() + 0x4c), [1.0, 2.0, 3.0]);
        assert_eq!(point(&w.e, w.this.addr() + 0x40), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn init_sets_goal_radius_tether_and_ignored_rotation() {
        let mut w = world();
        w.e.mem.set_u32(w.path + 8, 3);
        w.e.mem.set_f32(w.request + 0x74, 3.0);
        w.e.mem.set_f32(w.request + 0x68, 40.0);
        w.e.register(CHECK_GOAL_REACHED, |_, _| ret_u32(1));
        let result = w.e.call(0x009e_0470, &args![w.this, 0.0f32, 0.0f32]).u32();
        assert_eq!(result, 0);
        assert_eq!(w.e.get(w.this, H::eState), STATE_MOVING);
        assert_eq!(w.e.get(w.this, H::eBlendRunning), 3);
        assert_eq!(w.e.get(w.this, H::eAvoidState), 1);
        assert_eq!(w.e.get(w.this, H::fGoalRadiusSqr), 9.0);
        assert_eq!(w.e.get(w.this, H::fTetherDistFromActor), 40.0);
        assert_eq!(w.e.get(w.this, H::fTetherMinMove), 20.0);
        assert!(w.e.get(w.this, H::bIgnoreInitialRotation));
        assert_eq!(w.e.get(w.this, H::fIgnoreInitialRotationTetherParam), 3.0);
        // A path flag gives the fixed radius.
        w.e.mem.set_u8(w.path + 0x40, 1);
        w.e.call(0x009e_0470, &args![w.this, 0.0f32, 0.0f32]);
        assert_eq!(w.e.get(w.this, H::fGoalRadiusSqr), 900.0);
    }

    #[test]
    fn init_rotation_only_starts_a_turn() {
        let mut w = world();
        assert!(w.e.call(0x009e_0930, &args![w.this, 1.0f32, false]).bool());
        assert_eq!(w.e.get(w.this, H::eState), STATE_ROTATING);
        assert!(w.e.get(w.this, H::bRotateMode));
        assert_eq!(w.e.get(w.this, H::spPath), Ptr::NULL);
        // No turn needed: done.
        w.e.set(w.this, H::bRotateMode, false);
        assert!(!w.e.call(0x009e_0930, &args![w.this, 0.0f32, false]).bool());
        assert_eq!(w.e.get(w.this, H::eState), STATE_DONE);
    }

    #[test]
    fn update_returns_early_by_state_and_controller() {
        let mut w = world();
        w.e.set(w.this, H::eState, STATE_DONE);
        assert_eq!(w.e.call(0x009e_0a00, &args![w.this, 0.1f32]).u32(), 5);
        // An actor that cannot move finishes when the goal check says 5.
        w.e.set(w.this, H::eState, STATE_MOVING);
        w.e.mem.set_u8(w.actor + A_CAN_MOVE, 0);
        w.e.register(CHECK_GOAL_REACHED, |_, _| ret_u32(5));
        w.e.call_log = Some(vec![]);
        assert_eq!(w.e.call(0x009e_0a00, &args![w.this, 0.1f32]).u32(), 5);
        assert_eq!(calls(&w.e, RESET_MOVE_DELTA).len(), 1);
        assert_eq!(w.e.get(w.this, H::eState), STATE_DONE);
        // No character controller: stops.
        w.e.set(w.this, H::eState, STATE_MOVING);
        w.e.mem.set_u8(w.actor + A_CAN_MOVE, 1);
        assert_eq!(w.e.call(0x009e_0a00, &args![w.this, 0.1f32]).u32(), 0);
        assert!(w.e.get(w.this, H::bDeltaComputedThisFrame));
    }

    #[test]
    fn update_handles_falling() {
        let mut w = world();
        w.e.set(w.this, H::eState, STATE_MOVING);
        w.e.mem.set_u32(w.actor + A_CONTROLLER, 0x1234);
        w.e.register(CONTROLLER_STATE, |_, _| ret_u32(2));
        w.e.register(CONTROLLER_FALL_TIME, |_, _| ret_f(0.5));
        assert_eq!(w.e.call(0x009e_0a00, &args![w.this, 0.1f32]).u32(), 0);
        assert_eq!(w.e.get(w.this, H::fFallTime), 0.5);
        // Landed after a long fall.
        w.e.register(CONTROLLER_STATE, |_, _| ret_u32(0));
        w.e.set(w.this, H::fFallTime, 0.7);
        assert_eq!(w.e.call(0x009e_0a00, &args![w.this, 0.1f32]).u32(), 4);
    }

    #[test]
    fn update_in_rotate_mode_turns_and_clears_the_delta() {
        let mut w = world();
        w.e.set(w.this, H::eState, STATE_MOVING);
        w.e.mem.set_u32(w.actor + A_CONTROLLER, 0x1234);
        w.e.set(w.this, H::bRotateMode, true);
        w.e.set(w.this, H::fRotateGoal, 1.0);
        set_point(&mut w.e, w.this.addr() + 0x1c, [5.0, 5.0, 5.0]);
        assert_eq!(w.e.call(0x009e_0a00, &args![w.this, 0.1f32]).u32(), 0);
        // 90 degrees per second for 0.1 seconds, clockwise (bRotateCCW clear).
        let heading = w.e.mem.f32(w.actor + A_HEADING);
        assert!((heading + 0.157_079_63).abs() < 1e-5, "{heading}");
        assert_eq!(point(&w.e, w.this.addr() + 0x1c), [0.0, 0.0, 0.0]);
        assert_eq!(w.e.get(w.this, H::MoveMode) & 0x30, 0x10);
    }

    #[test]
    fn z_rotation_matrix() {
        let mut e = Engine::new();
        e.register(SIN_COS, |e, a| {
            let angle = f(a, 0);
            e.mem.set_f32(a[1], angle.sin());
            e.mem.set_f32(a[2], angle.cos());
            Ret::default()
        });
        let m = e.mem.alloc(0x24);
        e.call(0x009e_1640, &args![m, std::f32::consts::FRAC_PI_2]);
        let values: Vec<f32> = (0..9).map(|i| e.mem.f32(m + 4 * i)).collect();
        assert!(values[0].abs() < 1e-6);
        assert_eq!(values[1], 1.0);
        assert_eq!(values[3], -1.0);
        assert_eq!(values[8], 1.0);
    }

    #[test]
    fn clears_the_requests_first_tangent_flag() {
        let mut e = Engine::new();
        let request = e.mem.alloc(0xb0);
        e.mem.set_u8(request + 0xa0, 1);
        e.call(0x009e_16c0, &args![request]);
        assert_eq!(e.mem.u8(request + 0xa0), 0);
    }

    #[test]
    fn no_path_update_by_state() {
        let mut w = world();
        w.e.set(w.this, H::eState, STATE_ROTATED);
        assert_eq!(w.e.call(0x009e_16e0, &args![w.this, 0.1f32]).u32(), 5);
        assert_eq!(w.e.get(w.this, H::eState), STATE_DONE);
        // Done and not facing a target: the mover's mode, finished.
        w.e.mem.set_u32(w.mover + 0x34, 0x0201);
        assert_eq!(w.e.call(0x009e_16e0, &args![w.this, 0.1f32]).u32(), 5);
        assert_eq!(w.e.get(w.this, H::MoveMode), 0x0200);
        assert_eq!(w.e.get(w.this, H::spPath), Ptr::NULL);
        // Moving without a look-at point: nothing to turn to.
        w.e.set(w.this, H::eState, STATE_MOVING);
        w.e.mem.set_f32(w.actor + A_LOOKING, 0.5);
        assert_eq!(w.e.call(0x009e_16e0, &args![w.this, 0.1f32]).u32(), 0);
        // UpdateLooking towards 0.0.
        assert!(w.e.mem.f32(w.actor + A_LOOKING) < 0.5);
    }

    #[test]
    fn no_path_update_turns_towards_the_look_at_point() {
        let mut w = world();
        w.e.set(w.this, H::eState, STATE_MOVING);
        w.e.mem.set_u8(w.mover + 0x73, 1);
        set_point(&mut w.e, w.mover + 4, [100.0, 0.0, 0.0]);
        w.e.register(FIRING_LOCATION, |e, a| {
            set_point(e, a[0], [0.0, 0.0, 0.0]);
            ret_u32(a[0])
        });
        w.e.register(ANGLE_TO_PROJECTED_POINT, |e, a| {
            set_point(e, a[0], [0.0, 0.0, std::f32::consts::FRAC_PI_2]);
            ret_u32(a[0])
        });
        w.e.mem.set_u32(w.mover + 0x34, 0x0100);
        assert_eq!(w.e.call(0x009e_16e0, &args![w.this, 0.1f32]).u32(), 0);
        // Turned left by 9 degrees, the left-turn bit set, minimum rotate
        // time started.
        let heading = w.e.mem.f32(w.actor + A_HEADING);
        assert!((heading - 0.157_079_63).abs() < 1e-5, "{heading}");
        assert_eq!(w.e.get(w.this, H::MoveMode), 0x0120);
        assert_eq!(w.e.get(w.this, H::fMinRotateTime), 0.4);
    }

    #[test]
    fn update_while_player_waits_moves_to_the_end() {
        let mut w = world();
        w.e.register(PLUS_2C, |_, a| ret_u32(a[0] + 0x2c));
        let end = node(&mut w.e, [7.0, 8.0, 9.0]);
        w.e.mem.set_u32(w.path + P_END_NODE, end);
        w.e.mem.set_u32(w.path + 8, 4);
        for (i, at) in [(2u32, [0.0f32, 0.0, 0.0]), (3, [0.0, 10.0, 0.0])] {
            let n = node(&mut w.e, at);
            w.e.mem.set_u32(w.path + P_NODES + 4 * i, n);
        }
        w.e.mem.set_u16(w.mover + 0x42, 3);
        w.e.call_log = Some(vec![]);
        let result = w.e.call(0x009e_1d50, &args![w.this, 0u32]).u32();
        assert_eq!(result, 5);
        assert_eq!(w.e.get(w.this, H::eState), STATE_DONE);
        let moved = calls(&w.e, ACTOR_MOVE_TO_PATH_END);
        assert_eq!(moved.len(), 1);
        assert_eq!(point(&w.e, moved[0][1]), [7.0, 8.0, 9.0]);
        assert_eq!(&moved[0][5..], &[1, 0]);
        // Not at the last node: 6.
        w.e.mem.set_u16(w.mover + 0x42, 1);
        assert_eq!(w.e.call(0x009e_1d50, &args![w.this, 0u32]).u32(), 6);
        // A request flagged at +0x9F hands the player to slot 0x2C0.
        w.e.mem.set_u8(w.request + 0x9f, 1);
        w.e.call_log = Some(vec![]);
        assert_eq!(w.e.call(0x009e_1d50, &args![w.this, 0u32]).u32(), 0);
        let player: u32 = w.e.global(PLAYER);
        assert_eq!(calls(&w.e, FAKE + 8), vec![vec![w.actor, player]]);
    }

    #[test]
    fn request_byte_9f() {
        let mut e = Engine::new();
        let request = e.mem.alloc(0xb0);
        assert!(!e.call(0x009e_20a0, &args![request]).bool());
        e.mem.set_u8(request + 0x9f, 1);
        assert!(e.call(0x009e_20a0, &args![request]).bool());
    }

    #[test]
    fn door_opening_states() {
        let mut w = world();
        assert_eq!(w.e.call(0x009e_20c0, &args![w.this]).u32(), 0);
        w.e.set(w.this, H::eDoorState, 2);
        assert_eq!(w.e.call(0x009e_20c0, &args![w.this]).u32(), 2);
        // Waiting for an opened door.
        let door = w.e.mem.alloc(0x100);
        w.e.mem.set_u32(door, OBJECT_VTABLE);
        w.e.set(w.this, H::eDoorState, 1);
        w.e.set(w.this, H::pCurrentDoor, Ptr::new(door));
        w.e.register(GET_OPEN_STATE, |_, _| ret_u32(2));
        assert_eq!(w.e.call(0x009e_20c0, &args![w.this]).u32(), 1);
        w.e.register(GET_OPEN_STATE, |_, _| ret_u32(1));
        assert_eq!(w.e.call(0x009e_20c0, &args![w.this]).u32(), 0);
        assert_eq!(w.e.get(w.this, H::pCurrentDoor), Ptr::NULL);
        // A closed door in the path is opened by the actor.
        w.e.mem.set_u32(w.mover + 0x2c, 0x1234);
        w.e.mem.set_u32(OBJECT_VTABLE + DOOR_SLOT_F0, FAKE + 9);
        w.e.register_double(LOOKUP_FORM, move |_, _| ret_u32(door));
        w.e.register(GET_OPEN_STATE, |_, _| ret_u32(3));
        w.e.register(REFERENCE_ACTIVATE, |_, _| ret_u32(1));
        w.e.call_log = Some(vec![]);
        assert_eq!(w.e.call(0x009e_20c0, &args![w.this]).u32(), 1);
        assert_eq!(w.e.get(w.this, H::pCurrentDoor).addr(), door);
        assert_eq!(
            calls(&w.e, REFERENCE_ACTIVATE),
            vec![vec![door, w.actor, 0, 0, 1]]
        );
        assert_eq!(calls(&w.e, REFERENCE_SET_TARGETED), vec![vec![door, 1]]);
    }

    #[test]
    fn door_in_path_checks() {
        let mut w = world();
        // With a path: the path's door list answers first.
        w.e.register(ARRAY_FIND, |_, _| ret_u32(0));
        assert!(w.e.call(0x009e_22d0, &args![w.this, 5u32]).bool());
        // Not listed and no segments: not in the path.
        w.e.register(ARRAY_FIND, |_, _| ret_u32(u32::MAX));
        assert!(!w.e.call(0x009e_22d0, &args![w.this, 5u32]).bool());
        // Without a path the filter's third flag (set by its constructor)
        // stays set: in the path.
        w.e.set(w.this, H::spPath, Ptr::NULL);
        w.e.call_log = Some(vec![]);
        assert!(w.e.call(0x009e_22d0, &args![w.this, 5u32]).bool());
        let sight = calls(&w.e, CHECK_LINE_OF_SIGHT);
        assert_eq!(sight[0][1], w.this.addr() + 0x34);
    }

    #[test]
    fn path_door_list_search() {
        let mut e = Engine::new();
        let path = e.mem.alloc(0x40);
        e.register(ARRAY_FIND, |e, a| {
            ret_u32(if e.mem.u32(a[1]) == 9 { 1 } else { u32::MAX })
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x009e_2660, &args![path, 9u32]).bool());
        assert_eq!(calls(&e, ARRAY_FIND)[0][0], path + 0x30);
        assert_eq!(calls(&e, ARRAY_FIND)[0][3], ARRAY_COMPARE_009A3830);
        assert!(!e.call(0x009e_2660, &args![path, 8u32]).bool());
    }

    #[test]
    fn door_filter_constructor() {
        let mut e = Engine::new();
        e.register(FILTER_BASE_CONSTRUCT, |_, _| Ret::default());
        let filter = e.mem.alloc(0x0c);
        assert_eq!(e.call(0x009e_2690, &args![filter, 77u32]).u32(), filter);
        assert_eq!(e.mem.u32(filter), DOOR_IN_PATH_FILTER_VTABLE);
        assert_eq!(e.mem.u32(filter + 4), 77);
        assert_eq!(e.mem.bytes(filter + 8, 3), vec![0, 0, 1]);
    }

    #[test]
    fn door_filter_marks_edges_into_the_door() {
        let mut e = Engine::new();
        e.register(NAVMESH_GET_TRIANGLE, |_, a| ret_u32(a[0] + a[1]));
        e.register(TRIANGLE_HAS_FLAG, |_, _| ret_u32(1));
        e.register(NAVMESH_CLOSED_DOOR_INFO, |e, a| {
            ret_u32(if a[1] == 1 {
                e.mem.u32(0x6000_0000 + 0x10)
            } else {
                0
            })
        });
        e.map(0x6000_0000, 0x100);
        let info = 0x6000_0000;
        e.mem.set_u32(info, 77);
        e.mem.set_u32(info + 0x10, info);
        let filter = e.mem.alloc(0x0c);
        e.mem.set_u32(filter + 4, 77);
        e.mem.set_u8(filter + 0x0a, 1);
        assert!(e
            .call(
                0x009e_26d0,
                &args![filter, 0x100u32, 1u16, 0u32, 0x200u32, 2u16, 0u32]
            )
            .bool());
        assert_eq!(e.mem.bytes(filter + 8, 3), vec![0, 1, 0]);
    }

    #[test]
    fn tether_distances_from_the_request() {
        let mut w = world();
        w.e.mem.set_f32(w.request + 0x68, 50.0);
        assert!(!w.e.call(0x009e_27e0, &args![w.this]).bool());
        assert_eq!(w.e.get(w.this, H::fTetherDistFromActor), 50.0);
        assert_eq!(w.e.get(w.this, H::fTetherMinMove), 25.0);
    }

    #[test]
    fn distance_traveled_remaining_and_percent() {
        let mut w = world();
        w.e.set(w.this, H::fDistanceTraveled, 30.0);
        w.e.set(w.this, H::fPathLength, 120.0);
        assert_eq!(w.e.call(0x009e_2830, &args![w.this]).f32(), 30.0);
        assert_eq!(w.e.call(0x009e_2860, &args![w.this]).f32(), 90.0);
        assert_eq!(w.e.call(0x009e_28a0, &args![w.this]).f32(), 0.25);
        w.e.set(w.this, H::fDistanceTraveled, 150.0);
        assert_eq!(w.e.call(0x009e_2860, &args![w.this]).f32(), 0.0);
        assert_eq!(w.e.call(0x009e_28a0, &args![w.this]).f32(), 1.0);
        w.e.set(w.this, H::fPathLength, 0.0);
        assert_eq!(w.e.call(0x009e_28a0, &args![w.this]).f32(), 1.0);
        w.e.set(w.this, H::spPath, Ptr::NULL);
        assert_eq!(w.e.call(0x009e_2830, &args![w.this]).f32(), 0.0);
    }

    #[test]
    fn approximate_location_along_the_path() {
        let mut w = world();
        let result = w.e.mem.alloc(12);
        set_point(&mut w.e, w.actor + A_LOCATION, [1.0, 2.0, 3.0]);
        // No nodes: the actor's location.
        w.e.call(0x009e_2900, &args![w.this, result, 0.5f32]);
        assert_eq!(point(&w.e, result), [1.0, 2.0, 3.0]);
        // Running (mover slot 0x1C bit 0x200): run speed times the time.
        w.e.mem.set_u32(w.path + 8, 3);
        w.e.mem.set_u32(w.mover + 0x34, 0x200);
        w.e.set(w.this, H::fCurrentSplineT, 1.5);
        w.e.call_log = Some(vec![]);
        let back = w.e.call(0x009e_2900, &args![w.this, result, 0.5f32]).u32();
        assert_eq!(back, result);
        let after = calls(&w.e, PATH_LOCATION_AFTER);
        assert_eq!(after[0][..2], [w.path, result]);
        assert_eq!(f32::from_bits(after[0][2]), 1.5);
        assert_eq!(f32::from_bits(after[0][3]), 100.0);
    }

    #[test]
    fn waiting_for_avoidance_is_avoid_state_2() {
        let mut w = world();
        assert!(!w.e.call(0x009e_2a80, &args![w.this]).bool());
        w.e.set(w.this, H::eAvoidState, 2);
        assert!(w.e.call(0x009e_2a80, &args![w.this]).bool());
    }

    #[test]
    fn move_direction_bits_and_run_blend() {
        let mut w = world();
        // Actor state 5: no movement.
        w.e.register(ACTOR_STATE_IS_5, |_, _| ret_u32(1));
        w.e.set(w.this, H::MoveMode, 0x123);
        assert!(w.e.call(0x009e_2aa0, &args![w.this, 0.1f32]).bool());
        assert_eq!(w.e.get(w.this, H::MoveMode), 0);
        // Not facing a target: the mover's direction bits and base.
        w.e.register(ACTOR_STATE_IS_5, |_, _| ret_u32(0));
        w.e.mem.set_u32(w.mover + 0x34, 0x0101);
        w.e.set(w.this, H::eBlendRunning, 2);
        w.e.call(0x009e_2aa0, &args![w.this, 0.1f32]);
        assert_eq!(w.e.get(w.this, H::MoveMode), 0x0101);
        assert_eq!(w.e.get(w.this, H::eBlendRunning), 3);
        // Starting to run blends from walking.
        w.e.mem.set_u32(w.mover + 0x34, 0x0201);
        w.e.register(ACTOR_GET_ANIM_GROUP_DURATION, |_, _| ret_f(2.0));
        w.e.call(0x009e_2aa0, &args![w.this, 0.1f32]);
        assert_eq!(w.e.get(w.this, H::eBlendRunning), 1);
        assert_eq!(w.e.get(w.this, H::MoveMode), 0x0101);
        assert_eq!(w.e.get(w.this, H::fRunWalkTime), 0.9);
    }

    #[test]
    fn path_through_the_players_cell() {
        let mut w = world();
        w.e.mem.set_u32(w.path + 8, 3);
        let cell = 0x4444;
        w.e.register(PATH_IS_USABLE, |_, _| ret_u32(1));
        w.e.register(REFERENCE_PARENT_CELL, |_, _| ret_u32(0x4444));
        let far = node(&mut w.e, [0.0, 0.0, 0.0]);
        w.e.mem.set_u32(w.path + P_NODES + 4, far);
        let near = node(&mut w.e, [0.0, 0.0, 0.0]);
        w.e.mem.set_u32(w.path + P_NODES + 8, near);
        w.e.register_double(LOCATION_GET_CELL, move |_, a| {
            ret_u32(if a[0] == near + 4 { cell } else { 0 })
        });
        assert!(w.e.call(0x009e_3110, &args![w.this]).bool());
        // Past that node: no.
        w.e.set(w.this, H::iCurrentNodeIndex, 2);
        assert!(!w.e.call(0x009e_3110, &args![w.this]).bool());
    }

    #[test]
    fn dword_at_38() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object + 0x38, 5);
        assert_eq!(e.call(0x009e_32d0, &args![object]).u32(), 5);
    }

    #[test]
    fn move_delta_is_handed_out_once() {
        let mut w = world();
        let result = w.e.mem.alloc(12);
        set_point(&mut w.e, w.this.addr() + 0x1c, [1.0, 2.0, 3.0]);
        w.e.set(w.this, H::bDeltaComputedThisFrame, true);
        assert!(w.e.call(0x009e_32f0, &args![w.this, result]).bool());
        assert_eq!(point(&w.e, result), [1.0, 2.0, 3.0]);
        assert!(!w.e.call(0x009e_32f0, &args![w.this, result]).bool());
        assert_eq!(point(&w.e, result), [0.0, 0.0, 0.0]);
    }

    #[test]
    fn force_move_mode_sets_the_mode() {
        let mut w = world();
        w.e.call(0x009e_3360, &args![w.this, 0x77u32]);
        assert_eq!(w.e.get(w.this, H::MoveMode), 0x77);
    }

    #[test]
    fn previous_path_state_is_taken_over() {
        let mut w = world();
        let previous: Ptr<H> = w.e.new_object();
        w.e.mem.set_u32(previous.addr(), VTABLE);
        w.e.set(previous, H::MoveMode, 0x201);
        w.e.set(previous, H::eBlendRunning, 1);
        w.e.set(previous, H::fRunWalkTime, 0.5);
        w.e.set(previous, H::eAvoidState, 2);
        w.e.set(previous, H::fActorAvoidanceTimer, 1.5);
        w.e.set(previous, H::fWaitTime, 2.5);
        w.e.set(previous, H::bWaitingForUpdate, true);
        set_point(&mut w.e, previous.addr() + 0x34, [1.0, 2.0, 3.0]);
        w.e.call(0x009e_3380, &args![w.this, previous]);
        assert_eq!(w.e.get(w.this, H::MoveMode), 0x201);
        assert_eq!(w.e.get(w.this, H::fRunWalkTime), 0.5);
        assert_eq!(w.e.get(w.this, H::eAvoidState), 2);
        assert_eq!(w.e.get(w.this, H::fWaitTime), 2.5);
        assert!(w.e.get(w.this, H::bWaitingForUpdate));
        assert_eq!(point(&w.e, w.this.addr() + 0x34), [1.0, 2.0, 3.0]);
        // Avoid state 3 becomes 4 and the list moves over.
        w.e.set(previous, H::eAvoidState, 3);
        w.e.mem.set_u32(previous.addr() + 0x58, 0x99);
        w.e.register(LIST_POP_FRONT, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        w.e.call_log = Some(vec![]);
        w.e.call(0x009e_3380, &args![w.this, previous]);
        assert_eq!(w.e.get(w.this, H::eAvoidState), 4);
        assert_eq!(w.e.get(w.this, H::fWaitTime), 0.0);
        assert_eq!(
            calls(&w.e, LIST_PUSH_FRONT),
            vec![vec![w.this.addr() + 0x58, previous.addr() + 0x58]]
        );
    }

    #[test]
    fn move_delta_forward_speed() {
        let mut w = world();
        w.e.call(0x009e_3520, &args![w.this, 0.5f32]);
        assert_eq!(w.e.mem.f32(w.this.addr() + 0x20), 50.0);
        w.e.mem.set_f32(w.this.addr() + 0x20, 1.0);
        w.e.register(ACTOR_COMPUTE_MOVE_DELTA, |_, _| ret_u32(1));
        w.e.call(0x009e_3520, &args![w.this, 0.5f32]);
        assert_eq!(w.e.mem.f32(w.this.addr() + 0x20), 1.0);
    }

    #[test]
    fn move_delta_turned_towards_the_tether() {
        let mut w = world();
        set_point(&mut w.e, w.this.addr() + 0x34, [0.0, 10.0, 0.0]);
        set_point(&mut w.e, w.this.addr() + 0x1c, [0.0, 1.0, 0.0]);
        // Moving to the right (mode 8): rotated by -pi/2.
        w.e.set(w.this, H::MoveMode, 8);
        w.e.call(0x009e_3560, &args![w.this, 0u32]);
        let delta = point(&w.e, w.this.addr() + 0x1c);
        assert!(
            (delta[0] + 1.0).abs() < 1e-6 && delta[1].abs() < 1e-6,
            "{delta:?}"
        );
        // Swimming: pitched about x and transformed by the matrix helper.
        w.e.register(ACTOR_BYTE_14D, |_, _| ret_u32(1));
        w.e.register(POINT_TIMES_MATRIX, |e, a| {
            set_point(e, a[0], [9.0, 9.0, 9.0]);
            ret_u32(a[0])
        });
        w.e.register(REFERENCE_POSITION, |_, a| ret_u32(a[0] + A_LOCATION));
        w.e.call_log = Some(vec![]);
        w.e.call(0x009e_3560, &args![w.this, 0u32]);
        assert_eq!(point(&w.e, w.this.addr() + 0x1c), [9.0, 9.0, 9.0]);
        assert_eq!(calls(&w.e, MATRIX_MAKE_X_ROTATION).len(), 1);
    }

    #[test]
    fn controller_float_560() {
        let mut e = Engine::new();
        let controller = e.mem.alloc(0x600);
        e.mem.set_f32(controller + 0x560, 0.75);
        assert_eq!(e.call(0x009e_3970, &args![controller]).f32(), 0.75);
    }

    #[test]
    fn next_tether_point_is_the_tether_target() {
        let mut w = world();
        let result = w.e.mem.alloc(12);
        set_point(&mut w.e, w.this.addr() + 0x34, [4.0, 5.0, 6.0]);
        assert!(w.e.call(0x009e_3990, &args![w.this, result]).bool());
        assert_eq!(point(&w.e, result), [4.0, 5.0, 6.0]);
    }

    #[test]
    fn clear_movement_resets_the_handler() {
        let mut w = world();
        w.e.mem.set_u32(w.mover + 0x34, 0x0203);
        w.e.set(w.this, H::eDoorState, 2);
        w.e.set(w.this, H::bWaitingForUpdate, true);
        w.e.call(0x009e_39c0, &args![w.this]);
        assert_eq!(w.e.get(w.this, H::MoveMode), 0x0200);
        assert_eq!(w.e.get(w.this, H::eDoorState), 0);
        assert_eq!(w.e.get(w.this, H::eAvoidState), 1);
        assert_eq!(w.e.get(w.this, H::eState), STATE_DONE);
        assert!(!w.e.get(w.this, H::bWaitingForUpdate));
        assert_eq!(w.e.get(w.this, H::spPath), Ptr::NULL);
    }

    #[test]
    fn next_node_location() {
        let mut w = world();
        let result = w.e.mem.alloc(12);
        set_point(&mut w.e, w.actor + A_LOCATION, [1.0, 1.0, 1.0]);
        w.e.call(0x009e_3a70, &args![w.this, result]);
        assert_eq!(point(&w.e, result), [1.0, 1.0, 1.0]);
        w.e.mem.set_u32(w.path + 8, 2);
        let second = node(&mut w.e, [2.0, 2.0, 2.0]);
        w.e.mem.set_u32(w.path + P_NODES + 4, second);
        assert_eq!(w.e.call(0x009e_3a70, &args![w.this, result]).u32(), result);
        assert_eq!(point(&w.e, result), [2.0, 2.0, 2.0]);
        let end = node(&mut w.e, [3.0, 3.0, 3.0]);
        w.e.mem.set_u32(w.path + P_END_NODE, end);
        w.e.set(w.this, H::iCurrentNodeIndex, 1);
        w.e.call(0x009e_3a70, &args![w.this, result]);
        assert_eq!(point(&w.e, result), [3.0, 3.0, 3.0]);
    }

    #[test]
    fn node_location_by_index() {
        let mut w = world();
        let result = w.e.mem.alloc(12);
        w.e.mem.set_u32(w.path + 8, 1);
        let first = node(&mut w.e, [5.0, 6.0, 7.0]);
        w.e.mem.set_u32(w.path + P_NODES, first);
        assert!(w.e.call(0x009e_3b60, &args![w.this, 0u32, result]).bool());
        assert_eq!(point(&w.e, result), [5.0, 6.0, 7.0]);
        assert!(!w.e.call(0x009e_3b60, &args![w.this, 1u32, result]).bool());
    }

    #[test]
    fn movement_blocked_flag() {
        let mut w = world();
        assert!(!w.e.call(0x009e_3be0, &args![w.this]).bool());
        w.e.set(w.this, H::bMovementBlocked, true);
        assert!(w.e.call(0x009e_3be0, &args![w.this]).bool());
    }

    #[test]
    fn tether_moves_along_the_path() {
        let mut w = world();
        w.e.mem.set_u32(w.path + 8, 3);
        set_point(&mut w.e, w.this.addr() + 0x34, [1.0, 0.0, 0.0]);
        w.e.register(PATH_DISTANCE_AT, |_, _| ret_f(12.0));
        w.e.register(PATH_ADVANCE, |e, a| {
            assert_eq!(a[3], 0, "backwards");
            assert_eq!(f(a, 2), 5.0);
            e.mem.set_f32(a[4], 1.5);
            set_point(e, a[5], [2.0, 0.0, 0.0]);
            Ret::default()
        });
        w.e.call(0x009e_3c00, &args![w.this, -5.0f32]);
        assert_eq!(w.e.get(w.this, H::fDistanceTraveled), 12.0);
        assert_eq!(w.e.get(w.this, H::fCurrentSplineT), 1.5);
        assert_eq!(w.e.get(w.this, H::iCurrentNodeIndex), 1);
        assert_eq!(point(&w.e, w.this.addr() + 0x34), [2.0, 0.0, 0.0]);
        assert_eq!(point(&w.e, w.this.addr() + 0x4c), [1.0, 0.0, 0.0]);
    }

    #[test]
    fn tether_advances_or_sits_at_the_path_end() {
        let mut w = world();
        w.e.mem.set_u32(w.path + 8, 2);
        let end = node(&mut w.e, [9.0, 9.0, 0.0]);
        w.e.mem.set_u32(w.path + P_END_NODE, end);
        w.e.set(w.this, H::iCurrentNodeIndex, 1);
        w.e.call(0x009e_3d50, &args![w.this, 0u32]);
        assert_eq!(point(&w.e, w.this.addr() + 0x34), [9.0, 9.0, 0.0]);
        // Before the end with the tether point reached: steps of 0.1 until
        // the last node.
        w.e.set(w.this, H::iCurrentNodeIndex, 0);
        w.e.set(w.this, H::fTetherMinMove, 10.0);
        set_point(&mut w.e, w.this.addr() + 0x34, [0.0, 0.0, 0.0]);
        w.e.mem.set_u32(w.path + 8, 3);
        w.e.call(0x009e_3d50, &args![w.this, 0u32]);
        assert_eq!(w.e.get(w.this, H::fCurrentSplineT), 2.0);
        assert_eq!(w.e.get(w.this, H::iCurrentNodeIndex), 2);
        // A move bit that disagrees with the delta: nothing.
        w.e.set(w.this, H::fCurrentSplineT, 0.0);
        w.e.set(w.this, H::iCurrentNodeIndex, 0);
        w.e.set(w.this, H::MoveMode, 1);
        set_point(&mut w.e, w.this.addr() + 0x1c, [0.0, -1.0, 0.0]);
        w.e.call(0x009e_3d50, &args![w.this, 0u32]);
        assert_eq!(w.e.get(w.this, H::fCurrentSplineT), 0.0);
    }

    #[test]
    fn heading_and_looking_sources() {
        let mut w = world();
        let out = w.e.mem.alloc(8);
        w.e.set(w.this, H::bMovementBlocked, true);
        w.e.set(w.this, H::fAlternateFacingAngle, 0.5);
        w.e.call(0x009e_4450, &args![w.this, out, out + 4]);
        assert_eq!(w.e.mem.f32(out), 0.5);
        // Towards the tether point.
        w.e.set(w.this, H::bMovementBlocked, false);
        set_point(&mut w.e, w.this.addr() + 0x34, [10.0, 0.0, 0.0]);
        w.e.call(0x009e_4450, &args![w.this, out, out + 4]);
        assert_eq!(w.e.mem.f32(out), std::f32::consts::FRAC_PI_2);
        // Facing a target: the projected angles.
        w.e.mem.set_u8(w.mover + 0x73, 1);
        set_point(&mut w.e, w.mover + 4, [100.0, 0.0, 0.0]);
        w.e.register(ANGLE_TO_PROJECTED_POINT, |e, a| {
            set_point(e, a[0], [0.25, 0.0, 1.25]);
            ret_u32(a[0])
        });
        w.e.register(FIRING_LOCATION, |e, a| {
            set_point(e, a[0], [0.0, 0.0, 0.0]);
            ret_u32(a[0])
        });
        w.e.call(0x009e_4450, &args![w.this, out, out + 4]);
        assert_eq!(w.e.mem.f32(out), 1.25);
        assert_eq!(w.e.mem.f32(out + 4), 0.25);
    }

    #[test]
    fn heading_step_is_limited_by_the_turn_rate() {
        let mut w = world();
        let scale = w.e.mem.alloc(4);
        let end = node(&mut w.e, [1000.0, 1000.0, 0.0]);
        w.e.mem.set_u32(w.path + P_END_NODE, end);
        // The target heading pi/2 away; 0.1 s at 90 deg/s times 3.
        set_point(&mut w.e, w.this.addr() + 0x34, [10.0, 0.0, 0.0]);
        let heading = w.e.call(0x009e_4800, &args![w.this, 0.1f32, scale]).f32();
        assert!((heading - 0.471_238_9).abs() < 1e-5, "{heading}");
        assert_eq!(w.e.mem.f32(scale), 0.0);
        let applied = w.e.get(w.this, H::fLastRotationApplied);
        assert!((applied - 0.471_238_9).abs() < 1e-5);
        // A turn of almost pi slows the move down.
        set_point(&mut w.e, w.this.addr() + 0x34, [0.1, -10.0, 0.0]);
        w.e.call(0x009e_4800, &args![w.this, 0.1f32, scale]);
        assert_eq!(w.e.mem.f32(scale), 0.05);
        // Already facing it: the heading itself.
        set_point(&mut w.e, w.this.addr() + 0x34, [0.0, 10.0, 0.0]);
        assert_eq!(
            w.e.call(0x009e_4800, &args![w.this, 0.1f32, scale]).f32(),
            0.0
        );
    }

    #[test]
    fn looking_moves_at_75_degrees_per_second() {
        let mut w = world();
        w.e.call(0x009e_4c60, &args![w.this, 0.1f32, 1.0f32]);
        let looking = w.e.mem.f32(w.actor + A_LOOKING);
        assert!((looking - 0.130_899_69).abs() < 1e-6, "{looking}");
        w.e.call(0x009e_4c60, &args![w.this, 10.0f32, 1.0f32]);
        assert!((w.e.mem.f32(w.actor + A_LOOKING) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn blocked_check() {
        let mut w = world();
        w.e.set(w.this, H::eAvoidState, 3);
        assert!(w.e.call(0x009e_4cf0, &args![w.this, 0.1f32]).bool());
        // Did not move although it should have: the block time grows and a
        // tether at the end becomes passed.
        w.e.set(w.this, H::eAvoidState, 1);
        w.e.set(w.this, H::fLastFrameMinMoveDelta, 10.0);
        w.e.set(w.this, H::eState, STATE_AT_END);
        assert!(!w.e.call(0x009e_4cf0, &args![w.this, 0.1f32]).bool());
        assert_eq!(w.e.get(w.this, H::fBlockTime), 0.1);
        assert_eq!(w.e.get(w.this, H::eState), STATE_PAST_END);
        // Moved enough: unblocked.
        w.e.set(w.this, H::eState, STATE_MOVING);
        set_point(&mut w.e, w.actor + A_LOCATION, [10.0, 0.0, 0.0]);
        assert!(w.e.call(0x009e_4cf0, &args![w.this, 0.1f32]).bool());
        assert_eq!(w.e.get(w.this, H::fBlockTime), 0.0);
        // Long enough with a moving animation: blocked.
        w.e.set(w.this, H::fBlockTime, 1.6);
        w.e.register(FAKE + 7, |_, _| ret_u32(0x5555));
        w.e.register(ANIMATION_SHOULD_BE_MOVING, |_, _| ret_u32(1));
        w.e.call_log = Some(vec![]);
        assert!(!w.e.call(0x009e_4cf0, &args![w.this, 0.1f32]).bool());
        assert!(w.e.get(w.this, H::bMovementBlocked));
        assert_eq!(calls(&w.e, LOCATION_FROM_POINT_IN_CELL).len(), 1);
    }

    #[test]
    fn unblock_clears_flag_and_time() {
        let mut w = world();
        w.e.set(w.this, H::bMovementBlocked, true);
        w.e.set(w.this, H::fBlockTime, 3.0);
        w.e.call(0x009e_5480, &args![w.this]);
        assert!(!w.e.get(w.this, H::bMovementBlocked));
        assert_eq!(w.e.get(w.this, H::fBlockTime), 0.0);
    }

    #[test]
    fn avoidance_candidates_rebuild_once_per_stamp() {
        let mut w = world();
        w.e.register(STAMP_OF, |_, _| ret_u32(7));
        w.e.set_global(CANDIDATES_TIMESTAMP, 7u32);
        assert!(!w.e.call(0x009e_54a0, &[]).bool());
        // One actor with a moving process.
        let item = w.e.mem.alloc(8);
        w.e.mem.set_u32(item, w.actor);
        w.e.register_double(PLUS_80, move |_, _| ret_u32(item));
        w.e.mem.set_u32(w.actor + A_PROCESS, 0x3333);
        w.e.register(PROCESS_FLAG_4000, |_, _| ret_u32(1));
        w.e.set_global(CANDIDATES_TIMESTAMP, 6u32);
        w.e.call_log = Some(vec![]);
        assert!(w.e.call(0x009e_54a0, &[]).bool());
        assert_eq!(w.e.global::<u32>(CANDIDATES_TIMESTAMP), 7);
        assert_eq!(w.e.global::<u32>(CANDIDATES_SIZE), 1);
        assert_eq!(calls(&w.e, ARRAY_ADD).len(), 1);
    }

    #[test]
    fn acted_on_reference() {
        let mut w = world();
        assert_eq!(w.e.call(0x009e_55e0, &args![w.actor]).u32(), 0);
        let process = w.e.mem.alloc(0x10);
        w.e.mem.set_u32(process, OBJECT_VTABLE);
        w.e.mem.set_u32(w.actor + A_PROCESS, process);
        let first = w.e.mem.alloc(0x20);
        let table = w.e.mem.alloc(0x10);
        w.e.mem.set_u32(table + 8, 6);
        w.e.mem.set_u32(ACQUIRE_TABLE + 4, table);
        w.e.mem.set_u32(first + 0x18, 1);
        let reference = w.e.mem.alloc(0x10);
        w.e.mem.set_u32(reference, ACTOR_VTABLE);
        w.e.mem.set_u32(OBJECT_VTABLE + OBJECT_SLOT_27C, FAKE + 11);
        w.e.mem.set_u32(OBJECT_VTABLE + OBJECT_SLOT_280, FAKE + 12);
        w.e.mem.set_u32(OBJECT_VTABLE + OBJECT_SLOT_128, FAKE + 13);
        w.e.register_double(FAKE + 11, move |_, _| ret_u32(first));
        w.e.register(FAKE + 12, |_, _| ret_u32(2));
        w.e.register_double(FAKE + 13, move |_, _| ret_u32(reference));
        w.e.register(DWORD_AT_18, |e, a| ret_u32(e.mem.u32(a[0] + 0x18)));
        assert_eq!(w.e.call(0x009e_55e0, &args![w.actor]).u32(), reference);
    }

    #[test]
    fn actors_in_the_way_include_the_player_ahead() {
        let mut w = world();
        w.e.register(FRAME_FLOAT_C, |_, _| ret_f(0.1));
        let delta = w.e.mem.alloc(12);
        set_point(&mut w.e, delta, [0.0, 1.0, 0.0]);
        let result = w.e.mem.alloc(0x10);
        w.e.mem.set_f32(w.request + 0x68, 30.0);
        let player: u32 = w.e.global(PLAYER);
        w.e.mem.set_f32(player + A_RADIUS, 60.0);
        set_point(&mut w.e, player + A_LOCATION, [0.0, 20.0, 0.0]);
        w.e.call_log = Some(vec![]);
        w.e.call(
            0x009e_5690,
            &args![w.actor, w.request, 0u32, delta, w.this, result],
        );
        let added = calls(&w.e, AVOID_ARRAY_ADD);
        assert_eq!(added.len(), 1);
        assert_eq!(added[0][0], result);
        // Behind the actor, beyond the near distance: nobody.
        set_point(&mut w.e, player + A_LOCATION, [0.0, -60.0, 0.0]);
        w.e.call_log = Some(vec![]);
        w.e.call(
            0x009e_5690,
            &args![w.actor, w.request, 0u32, delta, w.this, result],
        );
        assert!(calls(&w.e, AVOID_ARRAY_ADD).is_empty());
    }

    #[test]
    fn actor_value_pair() {
        let mut e = Engine::new();
        let pair = e.mem.alloc(8);
        assert_eq!(e.call(0x009e_5ab0, &args![pair, 9u32, 0.5f32]).u32(), pair);
        assert_eq!(e.mem.u32(pair), 9);
        assert_eq!(e.mem.f32(pair + 4), 0.5);
    }

    #[test]
    fn avoidance_check_needs_a_moving_process() {
        let mut w = world();
        assert_eq!(w.e.call(0x009e_5ae0, &args![w.this, 0.1f32]).u32(), 0);
        // A process with flag 0x4000 but hardly moving: nothing.
        let process = w.e.mem.alloc(0x10);
        w.e.mem.set_u32(process, OBJECT_VTABLE);
        w.e.mem.set_u32(OBJECT_VTABLE + PROCESS_SLOT_278, FAKE + 7);
        w.e.mem.set_u32(w.actor + A_PROCESS, process);
        w.e.register(PROCESS_FLAG_4000, |_, _| ret_u32(1));
        set_point(&mut w.e, w.this.addr() + 0x1c, [0.1, 0.1, 0.0]);
        w.e.call_log = Some(vec![]);
        assert_eq!(w.e.call(0x009e_5ae0, &args![w.this, 0.1f32]).u32(), 0);
        assert!(calls(&w.e, AVOID_ARRAY_CONSTRUCT).is_empty());
        // Moving, nothing in the way: the detection value decays.
        set_point(&mut w.e, w.this.addr() + 0x1c, [0.0, 5.0, 0.0]);
        w.e.set(w.this, H::fAvoidanceDetectionValue, 0.5);
        w.e.register(ARRAY_IS_EMPTY, |_, _| ret_u32(1));
        assert_eq!(w.e.call(0x009e_5ae0, &args![w.this, 0.1f32]).u32(), 0);
        assert_eq!(w.e.get(w.this, H::fAvoidanceDetectionValue), 0.4);
        assert_eq!(calls(&w.e, AVOID_ARRAY_DESTRUCT).len(), 1);
    }

    #[test]
    fn avoid_node_constructor() {
        let mut e = Engine::new();
        e.register(DEFAULT_CONSTRUCT, |_, a| ret_u32(a[0]));
        let node = e.mem.alloc(0x24);
        let at = e.mem.alloc(12);
        set_point(&mut e, at, [1.0, 2.0, 3.0]);
        let back = e
            .call(0x009e_6870, &args![node, at, 4.0f32, 5.0f32, 2u32])
            .u32();
        assert_eq!(back, node);
        assert_eq!(point(&e, node), [1.0, 2.0, 3.0]);
        assert_eq!(e.mem.f32(node + 0x18), 4.0);
        assert_eq!(e.mem.f32(node + 0x1c), 5.0);
        assert_eq!(e.mem.u32(node + 0x20), 2);
    }

    #[test]
    fn end_avoidance() {
        let mut w = world();
        w.e.set(w.this, H::eAvoidState, 4);
        w.e.set(w.this, H::fActorAvoidanceTimer, 2.0);
        w.e.call(0x009e_68c0, &args![w.this]);
        assert_eq!(w.e.get(w.this, H::eAvoidState), 1);
        assert_eq!(w.e.get(w.this, H::fActorAvoidanceTimer), 0.0);
    }

    #[test]
    fn actor_in_the_way_test() {
        let mut w = world();
        let other = w.e.mem.alloc(0x200);
        w.e.mem.set_u32(other, ACTOR_VTABLE);
        w.e.mem.set_f32(other + A_RADIUS, 10.0);
        set_point(&mut w.e, other + A_LOCATION, [0.0, 20.0, 0.0]);
        let position = w.e.mem.alloc(12);
        let direction = w.e.mem.alloc(12);
        set_point(&mut w.e, direction, [0.0, 1.0, 0.0]);
        let value = w.e.mem.alloc(4);
        let hit = |w: &mut World, quick: bool| {
            w.e.call(
                0x009e_6900,
                &args![w.actor, w.this, other, position, direction, 100.0f32, 0.5f32, value, quick],
            )
            .bool()
        };
        assert!(hit(&mut w, true));
        assert_eq!(w.e.mem.f32(value), 0.0);
        // Ahead within the reach of 110.
        set_point(&mut w.e, other + A_LOCATION, [0.0, 60.0, 0.0]);
        assert!(hit(&mut w, false));
        // 60 of 110 left: two quarters, 0.8 squared twice.
        set_point(&mut w.e, direction, [0.0, 0.8, 0.0]);
        assert!(hit(&mut w, false));
        let expected = ((0.8f32 * 0.8) * (0.8f32 * 0.8)) as f64;
        assert!((w.e.mem.f32(value) as f64 - expected).abs() < 1e-6);
        // Behind, beyond the near distance: no.
        set_point(&mut w.e, other + A_LOCATION, [0.0, -100.0, 0.0]);
        assert!(!hit(&mut w, false));
        // Out of reach: no.
        set_point(&mut w.e, other + A_LOCATION, [0.0, 111.0, 0.0]);
        assert!(!hit(&mut w, false));
    }

    #[test]
    fn wait_for_updated_path() {
        let mut w = world();
        let request = w.e.mem.alloc(0xb0);
        w.e.call(0x009e_6ba0, &args![w.this, request]);
        assert_eq!(w.e.get(w.this, H::eState), 0);
        // Already waiting: the request starts at the tether point.
        w.e.mem.set_u32(w.path + 8, 3);
        w.e.set(w.this, H::bWaitingForUpdate, true);
        w.e.call_log = Some(vec![]);
        w.e.call(0x009e_6ba0, &args![w.this, request]);
        assert_eq!(calls(&w.e, REQUEST_SET_ORIGIN)[0][0], request);
        assert_eq!(calls(&w.e, LOCATION_FROM_POINT)[0][1], w.this.addr() + 0x34);
        // Moving but blocked: stops at its location.
        w.e.set(w.this, H::bWaitingForUpdate, false);
        w.e.set(w.this, H::eState, STATE_MOVING);
        w.e.set(w.this, H::bMovementBlocked, true);
        set_point(&mut w.e, w.actor + A_LOCATION, [3.0, 4.0, 0.0]);
        w.e.call(0x009e_6ba0, &args![w.this, request]);
        assert_eq!(w.e.get(w.this, H::eState), STATE_PAST_END);
        assert!(w.e.get(w.this, H::bWaitingForUpdate));
        assert_eq!(point(&w.e, w.this.addr() + 0x34), [3.0, 4.0, 0.0]);
    }

    #[test]
    fn avoid_nodes_at_the_start_of_a_path() {
        let mut w = world();
        w.e.set(w.this, H::eAvoidState, 3);
        w.e.call_log = Some(vec![]);
        w.e.call(0x009e_71b0, &args![w.actor, w.request, w.this]);
        assert!(calls(&w.e, AVOID_ARRAY_CONSTRUCT).is_empty());
        // Nobody in the way: only the array's life.
        w.e.set(w.this, H::eAvoidState, 1);
        w.e.register(ARRAY_IS_EMPTY, |_, _| ret_u32(1));
        w.e.register(FRAME_FLOAT_C, |_, _| ret_f(0.1));
        w.e.call_log = Some(vec![]);
        w.e.call(0x009e_71b0, &args![w.actor, w.request, w.this]);
        assert_eq!(calls(&w.e, AVOID_ARRAY_CONSTRUCT).len(), 1);
        assert_eq!(calls(&w.e, AVOID_ARRAY_DESTRUCT).len(), 1);
        assert!(calls(&w.e, REQUEST_AVOID_NODES).is_empty());
        assert_eq!(w.e.get(w.this, H::eAvoidState), 1);
    }

    #[test]
    fn end_rotate_at_the_last_node() {
        let mut w = world();
        assert!(!w.e.call(0x009e_7550, &args![w.this]).bool());
        w.e.register(REQUEST_BYTE_99, |_, _| ret_u32(1));
        w.e.register(REQUEST_FLOAT_8C, |_, _| ret_f(1.0));
        w.e.mem.set_u32(w.path + 8, 3);
        w.e.mem.set_u16(w.mover + 0x42, 2);
        assert!(w.e.call(0x009e_7550, &args![w.this]).bool());
        assert_eq!(w.e.get(w.this, H::fRotateGoal), 1.0);
        // Not the last node.
        w.e.set(w.this, H::bRotateMode, false);
        w.e.mem.set_u16(w.mover + 0x42, 0);
        assert!(!w.e.call(0x009e_7550, &args![w.this]).bool());
    }

    #[test]
    fn rotation_mode_starts_above_the_threshold() {
        let mut w = world();
        assert!(!w
            .e
            .call(0x009e_7610, &args![w.this, 0.01f32, false, false])
            .bool());
        assert!(w
            .e
            .call(0x009e_7610, &args![w.this, 1.0f32, false, false])
            .bool());
        assert!(w.e.get(w.this, H::bRotateMode));
        assert!(w.e.get(w.this, H::bRotateCCW));
        assert_eq!(w.e.get(w.this, H::fRotateGoal), 1.0);
        assert_eq!(w.e.get(w.this, H::fMinRotateTime), 0.5);
        // A negative heading is brought into 0..2pi first.
        w.e.register(FLOAT_MODULO, |_, a| ret_f((f(a, 0) % f(a, 1)) as f64));
        w.e.call(0x009e_7610, &args![w.this, -1.0f32, false, false]);
        let goal = w.e.get(w.this, H::fRotateGoal);
        assert!(
            (goal - (std::f32::consts::TAU - 1.0)).abs() < 1e-5,
            "{goal}"
        );
        // Ignoring the initial rotation: never.
        w.e.set(w.this, H::bIgnoreInitialRotation, true);
        assert!(!w
            .e
            .call(0x009e_7610, &args![w.this, 1.0f32, false, false])
            .bool());
    }

    #[test]
    fn tether_placed_at_the_start_distance() {
        let mut w = world();
        w.e.mem.set_f32(w.path + P_LENGTH, 100.0);
        w.e.register(PATH_PARAMETER_AT, |e, a| {
            e.mem.set_u32(a[3], 1);
            ret_f(1.25)
        });
        w.e.register(PATH_POINT_AT, |e, a| {
            set_point(e, a[2], [f(a, 1), 0.0, 0.0]);
            Ret::default()
        });
        w.e.call_log = Some(vec![]);
        w.e.call(0x009e_77d0, &args![w.this, 5.0f32]);
        assert_eq!(w.e.get(w.this, H::fPathLength), 100.0);
        assert_eq!(w.e.get(w.this, H::fCurrentSplineT), 1.25);
        assert_eq!(w.e.get(w.this, H::iCurrentNodeIndex), 1);
        assert_eq!(point(&w.e, w.this.addr() + 0x34), [1.25, 0.0, 0.0]);
        assert_eq!(calls(&w.e, ACTOR_MOVE_TO_PATH_END).len(), 1);
        // Waiting with an empty path: at the end.
        w.e.mem.set_f32(w.path + P_LENGTH, 0.0);
        w.e.mem.set_u32(w.path + 8, 4);
        let end = node(&mut w.e, [8.0, 8.0, 8.0]);
        w.e.mem.set_u32(w.path + P_END_NODE, end);
        w.e.set(w.this, H::bWaitingForUpdate, true);
        w.e.call(0x009e_77d0, &args![w.this, 5.0f32]);
        assert!(!w.e.get(w.this, H::bWaitingForUpdate));
        assert_eq!(w.e.get(w.this, H::iCurrentNodeIndex), 3);
        assert_eq!(w.e.get(w.this, H::fCurrentSplineT), 3.0);
        assert_eq!(point(&w.e, w.this.addr() + 0x34), [8.0, 8.0, 8.0]);
    }

    #[test]
    fn rotation_step_and_finish() {
        let mut w = world();
        w.e.mem.set_u32(w.mover + 0x34, 0x0100);
        w.e.set(w.this, H::fRotateGoal, 1.0);
        w.e.set(w.this, H::bRotateCCW, true);
        w.e.set(w.this, H::bRotateMode, true);
        let heading = w.e.call(0x009e_7d70, &args![w.this, 0.1f32]).f32();
        assert!((heading - 0.157_079_63).abs() < 1e-5, "{heading}");
        assert_eq!(w.e.get(w.this, H::MoveMode), 0x0120);
        // Close enough: the goal, and the rotation ends.
        w.e.mem.set_f32(w.actor + A_HEADING, 0.95);
        assert_eq!(w.e.call(0x009e_7d70, &args![w.this, 0.1f32]).f32(), 1.0);
        assert!(!w.e.get(w.this, H::bRotateMode));
        // An actor that cannot move keeps its heading.
        w.e.mem.set_u8(w.actor + A_CAN_MOVE, 0);
        assert_eq!(w.e.call(0x009e_7d70, &args![w.this, 0.1f32]).f32(), 0.95);
        assert_eq!(w.e.get(w.this, H::MoveMode), 0x0100);
    }

    /// The doubles `CheckGoalReached` needs beyond the shared ones.
    fn goal_doubles(e: &mut Engine) {
        e.register(
            REQUEST_FLOAT_70,
            |e, a| ret_f(e.mem.f32(a[0] + 0x70) as f64),
        );
        e.register(FLOAT_MIN, |_, a| ret_f(f(a, 0).min(f(a, 1)) as f64));
        e.register(PATH_NODE_BY_INDEX, |e, a| {
            ret_u32(e.mem.u32(a[0] + P_NODES + 4 * a[1]))
        });
    }

    #[test]
    fn goal_reached_within_radius_and_height() {
        let mut w = world();
        goal_doubles(&mut w.e);
        w.e.mem.set_u8(w.path + 0x40, 1);
        let end = node(&mut w.e, [100.0, 0.0, 10.0]);
        w.e.mem.set_u32(w.path + P_END_NODE, end);
        set_point(&mut w.e, w.actor + A_LOCATION, [95.0, 0.0, 0.0]);
        w.e.set(w.this, H::fGoalRadiusSqr, 100.0);
        w.e.mem.set_f32(w.request + 0x70, 20.0);
        assert_eq!(
            detailed_actor_path_handler_check_goal_reached(&mut w.e, w.this),
            5
        );
        // Too far.
        w.e.set(w.this, H::fGoalRadiusSqr, 10.0);
        assert_eq!(
            detailed_actor_path_handler_check_goal_reached(&mut w.e, w.this),
            0
        );
        // Too high, but not at the end yet.
        w.e.set(w.this, H::fGoalRadiusSqr, 100.0);
        w.e.mem.set_f32(w.request + 0x70, 5.0);
        w.e.mem.set_u32(w.path + 8, 3);
        assert_eq!(
            detailed_actor_path_handler_check_goal_reached(&mut w.e, w.this),
            0
        );
        // Too high at the end: failed, and logged.
        w.e.set(w.this, H::iCurrentNodeIndex, 2);
        w.e.call_log = Some(vec![]);
        assert_eq!(
            detailed_actor_path_handler_check_goal_reached(&mut w.e, w.this),
            2
        );
        let logged = calls(&w.e, LOG_MESSAGE);
        assert_eq!(logged.len(), 1);
        assert_eq!(logged[0][..2], [GOAL_Z_MESSAGE, 0]);
        assert_eq!(f64::take(&logged[0], &mut 2), 10.0);
        assert_eq!(calls(&w.e, REFERENCE_NAME), vec![vec![w.actor]]);
    }

    #[test]
    fn goal_height_against_actor_height_and_last_node_goal() {
        let mut w = world();
        goal_doubles(&mut w.e);
        w.e.register(ACTOR_BYTE_14D, |_, _| ret_u32(1));
        w.e.register(
            ACTOR_GET_HEIGHT,
            |e, a| ret_f(e.mem.f32(a[0] + 0xb0) as f64),
        );
        // The path's byte at +0x40 is clear and its dword at +0x1C (0 from
        // the double) names the last node: that node is the goal.
        w.e.mem.set_u32(w.path + 8, 1);
        let last = node(&mut w.e, [0.0, 0.0, 10.0]);
        w.e.mem.set_u32(w.path + P_NODES, last);
        w.e.set(w.this, H::fGoalRadiusSqr, 100.0);
        w.e.mem.set_f32(w.request + 0x70, 20.0);
        w.e.mem.set_f32(w.actor + 0xb0, 8.0);
        w.e.call_log = Some(vec![]);
        assert_eq!(
            detailed_actor_path_handler_check_goal_reached(&mut w.e, w.this),
            0
        );
        assert_eq!(calls(&w.e, PATH_NODE_BY_INDEX), vec![vec![w.path, 0]]);
        assert!(calls(&w.e, PATH_END_NODE).is_empty());
        assert_eq!(
            calls(&w.e, FLOAT_MIN)[0],
            vec![20.0f32.to_bits(), 8.0f32.to_bits()]
        );
        w.e.mem.set_f32(w.actor + 0xb0, 12.0);
        assert_eq!(
            detailed_actor_path_handler_check_goal_reached(&mut w.e, w.this),
            5
        );
    }

    #[test]
    fn goal_not_reached_while_tether_moves_away() {
        let mut w = world();
        goal_doubles(&mut w.e);
        w.e.mem.set_u8(w.path + 0x40, 1);
        let end = node(&mut w.e, [100.0, 0.0, 0.0]);
        w.e.mem.set_u32(w.path + P_END_NODE, end);
        set_point(&mut w.e, w.actor + A_LOCATION, [120.0, 0.0, 0.0]);
        w.e.set(w.this, H::fGoalRadiusSqr, 1000.0);
        w.e.mem.set_f32(w.request + 0x70, 20.0);
        w.e.mem.set_u8(w.request + 0x9c, 1);
        set_point(&mut w.e, w.this.addr() + 0x34, [150.0, 0.0, 0.0]);
        set_point(&mut w.e, w.this.addr() + 0x4c, [140.0, 0.0, 0.0]);
        assert_eq!(
            detailed_actor_path_handler_check_goal_reached(&mut w.e, w.this),
            0
        );
        // Blocked: reached after all.
        w.e.set(w.this, H::fBlockTime, 1.0);
        assert_eq!(
            detailed_actor_path_handler_check_goal_reached(&mut w.e, w.this),
            5
        );
        // Tether point moving back towards the actor: reached.
        w.e.set(w.this, H::fBlockTime, 0.0);
        set_point(&mut w.e, w.this.addr() + 0x4c, [160.0, 0.0, 0.0]);
        assert_eq!(
            detailed_actor_path_handler_check_goal_reached(&mut w.e, w.this),
            5
        );
    }

    #[test]
    fn goal_while_updating_the_path_end() {
        let mut w = world();
        goal_doubles(&mut w.e);
        let results = std::rc::Rc::new(std::cell::RefCell::new(vec![0i32, 2, 1]));
        let pending = results.clone();
        w.e.register_double(PATHFIND_006D5340, move |_, _| {
            ret_u32(pending.borrow_mut().pop().unwrap() as u32)
        });
        w.e.register(LOCATION_GET_WORLDSPACE, |_, _| ret_u32(7));
        w.e.register(FLAGS_BIT_2, |_, _| ret_u32(1));
        w.e.set(w.this, H::bEndUpdatePath, true);
        w.e.mem.set_u32(w.path + 8, 3);
        // Two nodes or more before the end: nothing yet.
        assert_eq!(
            detailed_actor_path_handler_check_goal_reached(&mut w.e, w.this),
            0
        );
        // The node two before the end, too far from the actor.
        w.e.mem.set_u32(w.path + 8, 2);
        let first = node(&mut w.e, [0.0, 0.0, 0.0]);
        w.e.mem.set_u32(w.path + P_NODES, first);
        set_point(&mut w.e, w.actor + A_LOCATION, [200.0, 0.0, 0.0]);
        w.e.call_log = Some(vec![]);
        assert_eq!(
            detailed_actor_path_handler_check_goal_reached(&mut w.e, w.this),
            0
        );
        assert!(calls(&w.e, PATHFIND_006D5340).is_empty());
        // Close enough: the check answers 1 (the goal).
        set_point(&mut w.e, w.actor + A_LOCATION, [100.0, 0.0, 0.0]);
        w.e.mem.set_u16(w.mover + 0x42, 3);
        assert_eq!(
            detailed_actor_path_handler_check_goal_reached(&mut w.e, w.this),
            5
        );
        assert_eq!(
            calls(&w.e, PATHFIND_006D5340),
            vec![vec![w.request, w.path, 2, w.actor + A_LOCATION]]
        );
        // 2: keep updating, with the end node's flag and the next node.
        assert_eq!(
            detailed_actor_path_handler_check_goal_reached(&mut w.e, w.this),
            0
        );
        assert!(w.e.get(w.this, H::bEndUpdatePath));
        assert_eq!(w.e.mem.u16(w.mover + 0x42), 7);
        // 0: the end is final.
        assert_eq!(
            detailed_actor_path_handler_check_goal_reached(&mut w.e, w.this),
            0
        );
        assert!(!w.e.get(w.this, H::bEndUpdatePath));
        assert!(results.borrow().is_empty());
    }

    #[test]
    fn request_byte_9c() {
        let mut e = Engine::new();
        let request = e.mem.alloc(0xb0);
        e.mem.set_u8(request + 0x9c, 3);
        assert_eq!(e.call(0x009e_8670, &args![request]).u8(), 3);
    }

    #[test]
    fn deleted_reference_leaves_door_candidates_and_avoid_list() {
        let mut w = world();
        let reference = w.e.mem.alloc(0x10);
        w.e.mem.set_u32(reference, ACTOR_VTABLE);
        let other = w.e.mem.alloc(0x10);
        w.e.set(w.this, H::pCurrentDoor, Ptr::new(reference));
        w.e.set(w.this, H::eDoorState, 2);
        w.e.set(w.this, H::eAvoidState, 3);
        let buffer = w.e.mem.alloc(12);
        for (i, v) in [reference, other, reference].into_iter().enumerate() {
            w.e.mem.set_u32(buffer + 4 * i as u32, v);
        }
        let array: Ptr<NiTArray> = Ptr::new(CANDIDATES);
        w.e.set(array, NiTArray::m_pBase, buffer);
        w.e.set(array, NiTArray::m_usSize, 3);
        w.e.set(array, NiTArray::m_usESize, 3);
        w.e.set_global(CANDIDATES_SIZE, 3u32);
        w.e.register(LIST_CONTAINS, |_, _| ret_u32(1));
        w.e.call_log = Some(vec![]);
        w.e.call(0x009e_8690, &args![w.this, reference]);
        assert_eq!(w.e.get(w.this, H::pCurrentDoor), Ptr::NULL);
        assert_eq!(w.e.get(w.this, H::eDoorState), 0);
        // The first match is removed; the index moves on past the shorter
        // count, so the last one stays.
        assert_eq!(w.e.mem.u32(buffer), 0);
        assert_eq!(w.e.mem.u32(buffer + 8), reference);
        assert_eq!(w.e.get(array, NiTArray::m_usESize), 2);
        assert_eq!(w.e.global::<u32>(CANDIDATES_SIZE), 2);
        assert_eq!(calls(&w.e, ARRAY_COMPACT), vec![vec![CANDIDATES]]);
        let list = w.this.addr() + 0x58;
        assert_eq!(calls(&w.e, LIST_REMOVE).len(), 1);
        assert_eq!(calls(&w.e, LIST_REMOVE)[0][0], list);
        assert_eq!(w.e.get(w.this, H::eAvoidState), 1);
        assert_eq!(calls(&w.e, LIST_CLEAR), vec![vec![list]]);
        // A reference whose slot 0x100 answers false only loses the door.
        let vtable = w.e.mem.alloc(0x200);
        w.e.mem.set_u32(vtable + REFERENCE_SLOT_100, FAKE + 7);
        w.e.mem.set_u32(other, vtable);
        w.e.call_log = Some(vec![]);
        w.e.call(0x009e_8690, &args![w.this, other]);
        assert!(calls(&w.e, ARRAY_ELEMENT).is_empty());
        assert!(calls(&w.e, LIST_CONTAINS).is_empty());
    }

    #[test]
    fn reset_move_delta_takes_the_movers_mode() {
        let mut w = world();
        w.e.mem.set_u32(w.mover + 0x34, 0x0300);
        set_point(&mut w.e, NO_POINT, [1.0, 2.0, 3.0]);
        w.e.set(w.this, H::fLastFrameMinMoveDelta, 4.0);
        // The shared doubles stand in for this address; call it directly.
        fn_009e8790(&mut w.e, w.this);
        assert_eq!(w.e.get(w.this, H::MoveMode), 0x0300);
        assert_eq!(point(&w.e, w.this.addr() + 0x1c), [1.0, 2.0, 3.0]);
        assert_eq!(point(&w.e, w.this.addr() + 0xa0), [1.0, 2.0, 3.0]);
        assert_eq!(w.e.get(w.this, H::fLastFrameMinMoveDelta), 0.0);
    }

    #[test]
    fn warp_to_next_node_checks_distances_and_doors() {
        let mut w = world();
        let end = node(&mut w.e, [0.0, 0.0, 0.0]);
        w.e.mem.set_u32(w.path + P_END_NODE, end);
        set_point(&mut w.e, w.this.addr() + 0x34, [200.0, 0.0, 0.0]);
        // A request whose float at +0x68 is above 100.
        w.e.mem.set_f32(w.request + 0x68, 150.0);
        assert!(!detailed_actor_path_handler_warp_to_next_node(
            &mut w.e, w.this
        ));
        // No nodes.
        w.e.mem.set_f32(w.request + 0x68, 50.0);
        assert!(!detailed_actor_path_handler_warp_to_next_node(
            &mut w.e, w.this
        ));
        // The door filter's third flag stays set without a navmesh triangle.
        w.e.mem.set_u32(w.path + 8, 1);
        w.e.call_log = Some(vec![]);
        assert!(!detailed_actor_path_handler_warp_to_next_node(
            &mut w.e, w.this
        ));
        assert_eq!(calls(&w.e, CHECK_LINE_OF_SIGHT).len(), 1);
        assert!(calls(&w.e, MOVER_MOVE_ACTOR_TO_LOCATION).is_empty());
        assert_eq!(calls(&w.e, LOCATION_DESTRUCT).len(), 1);
        // A triangle without the door flag: warped.
        w.e.register(LOCATION_GET_NAVMESH_AND_TRIANGLE, |_, _| ret_u32(1));
        w.e.register(NAVMESH_GET_TRIANGLE, |_, _| ret_u32(0x40));
        w.e.call_log = Some(vec![]);
        assert!(detailed_actor_path_handler_warp_to_next_node(
            &mut w.e, w.this
        ));
        let tether = w.this.addr() + 0x34;
        assert_eq!(
            calls(&w.e, MOVER_MOVE_ACTOR_TO_LOCATION),
            vec![vec![w.mover, tether]]
        );
        assert_eq!(calls(&w.e, CHECK_LINE_OF_SIGHT)[0][1..3], [tether, 0]);
        // The path's end closer to the tether point than the actor's float
        // times 0.6.
        w.e.mem.set_u8(w.path + 0x40, 1);
        w.e.register(ACTOR_MEASURE_008BE350, |_, _| ret_f(400.0));
        assert!(!detailed_actor_path_handler_warp_to_next_node(
            &mut w.e, w.this
        ));
        // A tether point nearer than 128 is pushed on by the difference.
        w.e.mem.set_u8(w.path + 0x40, 0);
        w.e.mem.set_u32(w.path + 8, 2);
        set_point(&mut w.e, tether, [100.0, 0.0, 0.0]);
        w.e.call_log = Some(vec![]);
        detailed_actor_path_handler_warp_to_next_node(&mut w.e, w.this);
        let advanced = calls(&w.e, PATH_ADVANCE);
        assert_eq!(advanced.len(), 1);
        assert_eq!(f(&advanced[0], 2), 28.0);
    }

    #[test]
    fn print_current_status_names_the_states() {
        let mut w = world();
        let printer = w.e.mem.alloc(4);
        let vtable = w.e.mem.alloc(0x20);
        w.e.mem.set_u32(printer, vtable);
        w.e.mem.set_u32(vtable + PRINTER_SLOT_PRINT, FAKE + 20);
        w.e.register(FAKE + 20, |_, _| Ret::default());
        set_point(&mut w.e, w.actor + A_LOCATION, [1.0, 2.0, 3.0]);
        w.e.set(w.this, H::eState, STATE_MOVING);
        w.e.set(w.this, H::bMovementBlocked, true);
        w.e.set(w.this, H::eAvoidState, 4);
        w.e.set(w.this, H::fAvoidanceDetectionValue, 0.5);
        w.e.call_log = Some(vec![]);
        w.e.call(0x009e_8b50, &args![w.this, printer, 11u32, 12u32]);
        let printed = calls(&w.e, FAKE + 20);
        assert_eq!(printed.len(), 3);
        assert_eq!(printed[0][..4], [printer, 11, 12, 0x0109_2604]);
        assert_eq!(f64::take(&printed[0], &mut 4), 1.0);
        assert_eq!(f64::take(&printed[0], &mut 8), 3.0);
        let appended: Vec<u32> = calls(&w.e, STRING_APPEND).iter().map(|a| a[2]).collect();
        assert_eq!(appended[0], 0x0109_25c4);
        assert_eq!(appended[1], 0x0109_2518);
        let formatted = calls(&w.e, STRING_FORMAT);
        assert_eq!(formatted.len(), 1);
        assert_eq!(formatted[0][2], 0x0109_2540);
        assert_eq!(f64::take(&formatted[0], &mut 3), 0.5);
        assert_eq!(appended[2], formatted[0][0]);
        assert_eq!(printed[1][3], printed[2][3]);
        // Unknown states and no detection value.
        w.e.set(w.this, H::eState, 9);
        w.e.set(w.this, H::eAvoidState, 1);
        w.e.set(w.this, H::fAvoidanceDetectionValue, 0.0);
        w.e.call_log = Some(vec![]);
        w.e.call(0x009e_8b50, &args![w.this, printer, 11u32, 12u32]);
        let appended: Vec<u32> = calls(&w.e, STRING_APPEND).iter().map(|a| a[2]).collect();
        assert_eq!(appended, vec![0x0104_8858, 0x0103_934c]);
        assert!(calls(&w.e, STRING_FORMAT).is_empty());
    }

    /// A three-node avoid list at the handler's +0x58: `first`, a node
    /// without an item, `third`. Returns the second and third nodes.
    fn avoid_list(e: &mut Engine, this: Ptr<H>, first: u32, third: u32) -> (u32, u32) {
        let second = e.mem.alloc(8);
        let last = e.mem.alloc(8);
        let head = this.addr() + 0x58;
        e.mem.set_u32(head, first);
        e.mem.set_u32(head + 4, second);
        e.mem.set_u32(second + 4, last);
        e.mem.set_u32(last, third);
        (second, last)
    }

    #[test]
    fn save_game_writes_fields_door_and_avoid_list() {
        let mut w = world();
        let buffer = w.e.mem.alloc(4);
        w.e.set(w.this, H::pCurrentDoor, Ptr::new(0x5000));
        avoid_list(&mut w.e, w.this, 0x6000, 0x7000);
        w.e.register(SAVE_START_VARIABLE_SIZED, |_, _| ret_u32(77));
        w.e.call_log = Some(vec![]);
        w.e.call(0x009e_8f80, &args![w.this, buffer]);
        let saved = calls(&w.e, SAVE_BYTES);
        assert_eq!(saved.len(), 43);
        let base = w.this.addr();
        assert_eq!(saved[0], vec![buffer, base + 0x1c, 0xc, 0]);
        assert_eq!(saved[36], vec![buffer, base + 0xe2, 1, 0]);
        assert_eq!(saved[42], vec![buffer, base + 0xbc, 4, 0]);
        assert_eq!(
            calls(&w.e, SAVE_FORM_ID),
            vec![
                vec![buffer, 0x5000, 0],
                vec![buffer, 0x6000, 0],
                vec![buffer, 0x7000, 0]
            ]
        );
        assert_eq!(calls(&w.e, SAVE_VARIABLE_SIZED), vec![vec![buffer, 2, 77]]);
    }

    #[test]
    fn load_game_reads_by_version() {
        let mut w = world();
        let buffer = w.e.mem.alloc(8);
        let vtable = w.e.mem.alloc(4);
        w.e.mem.set_u32(buffer, vtable);
        w.e.mem.set_u32(vtable, FAKE + 21);
        w.e.register(FAKE + 21, |e, a| ret_u32(e.mem.u32(a[0] + 4)));
        w.e.register(LOAD_VARIABLE_SIZED, |_, _| ret_u32(2));
        w.e.register(LOAD_FORM_ID, |e, a| {
            let next = e.mem.u32(a[1]) + 0x100;
            e.mem.set_u32(a[1], next);
            Ret::default()
        });
        let pushed = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let record = pushed.clone();
        w.e.register_double(LIST_PUSH_FRONT, move |e, a| {
            record.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        for (version, count) in [(3u32, 36usize), (4, 38), (12, 41), (18, 43)] {
            w.e.mem.set_u32(buffer + 4, version);
            w.e.call_log = Some(vec![]);
            w.e.call(0x009e_9390, &args![w.this, buffer]);
            assert_eq!(calls(&w.e, LOAD_BYTES).len(), count, "version {version}");
        }
        let loaded = calls(&w.e, LOAD_BYTES);
        let base = w.this.addr();
        assert_eq!(loaded[25], vec![buffer, base + 0x14, 4]);
        assert_eq!(loaded[38], vec![buffer, base + 0xc0, 1]);
        assert_eq!(loaded[42], vec![buffer, base + 0xbc, 4]);
        assert_eq!(calls(&w.e, LOAD_FORM_ID)[0], vec![buffer, base + 0xd8]);
        let list = base + 0x58;
        assert_eq!(pushed.borrow()[6..], [(list, 0x100), (list, 0x100)]);
    }

    #[test]
    fn init_load_game_resolves_door_and_actors() {
        let mut w = world();
        w.e.register(LOOKUP_FORM, |_, a| ret_u32(a[0] + 0x1000));
        // Only form 7 is an actor; every door id is a reference.
        w.e.register(RT_DYNAMIC_CAST, |_, a| {
            let ok = a[3] == TYPE_TES_OBJECT_REFR || a[0] == 0x1007;
            assert_eq!([a[1], a[2], a[4]], [0, TYPE_TES_FORM, 0]);
            ret_u32(if ok { a[0] } else { 0 })
        });
        w.e.register(LIST_POP_FRONT, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            let (item, after) = if next == 0 {
                (0, 0)
            } else {
                (e.mem.u32(next), e.mem.u32(next + 4))
            };
            e.mem.set_u32(a[0], item);
            e.mem.set_u32(a[0] + 4, after);
            Ret::default()
        });
        w.e.register(LIST_SET_ITEM, |e, a| {
            let v = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], v);
            Ret::default()
        });
        w.e.register(LIST_REMOVE, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            let after = e.mem.u32(next + 4);
            e.mem.set_u32(a[0] + 4, after);
            Ret::default()
        });
        w.e.set(w.this, H::pCurrentDoor, Ptr::new(5));
        let head = w.this.addr() + 0x58;
        let (second, last) = avoid_list(&mut w.e, w.this, 0, 9);
        w.e.mem.set_u32(second, 7);
        w.e.call_log = Some(vec![]);
        w.e.call(0x009e_9760, &args![w.this, 0u32]);
        let door = 0x1005;
        assert_eq!(w.e.get(w.this, H::pCurrentDoor), Ptr::new(door));
        assert_eq!(calls(&w.e, LIST_POP_FRONT), vec![vec![head]]);
        assert_eq!(w.e.mem.u32(head), 0x1007);
        assert_eq!(calls(&w.e, LIST_REMOVE), vec![vec![head, last]]);
        assert_eq!(w.e.mem.u32(head + 4), 0);
        assert_eq!(calls(&w.e, REFERENCE_SET_TARGETED), vec![vec![door, 1]]);
        // No door: nothing targeted.
        w.e.set(w.this, H::pCurrentDoor, Ptr::NULL);
        w.e.call_log = Some(vec![]);
        w.e.call(0x009e_9760, &args![w.this, 0u32]);
        assert!(calls(&w.e, REFERENCE_SET_TARGETED).is_empty());
        assert!(calls(&w.e, LOOKUP_FORM).iter().all(|a| a[0] != 0));
    }

    #[test]
    fn array_element_taken_out() {
        let mut e = Engine::new();
        let array: Ptr<NiTArray> = e.new_object();
        let buffer = e.mem.alloc(12);
        e.mem.set_u32(buffer, 0x10);
        e.mem.set_u32(buffer + 8, 0x30);
        e.set(array, NiTArray::m_pBase, buffer);
        e.set(array, NiTArray::m_usSize, 3);
        e.set(array, NiTArray::m_usESize, 2);
        assert_eq!(e.call(0x009e_98d0, &args![array, 3u32]).u32(), 0);
        // An empty slot: the counts stay.
        assert_eq!(e.call(0x009e_98d0, &args![array, 1u32]).u32(), 0);
        assert_eq!(e.get(array, NiTArray::m_usESize), 2);
        assert_eq!(e.get(array, NiTArray::m_usSize), 3);
        // The last one: the size drops too.
        assert_eq!(e.call(0x009e_98d0, &args![array, 2u32]).u32(), 0x30);
        assert_eq!(e.mem.u32(buffer + 8), 0);
        assert_eq!(e.get(array, NiTArray::m_usESize), 1);
        assert_eq!(e.get(array, NiTArray::m_usSize), 2);
        assert_eq!(e.call(0x009e_98d0, &args![array, 0u32]).u32(), 0x10);
        assert_eq!(e.get(array, NiTArray::m_usSize), 2);
    }

    #[test]
    fn avoidance_array_construct_destruct_and_delete() {
        let mut w = world();
        let array: Ptr<BSSimpleArray> = w.e.new_object();
        w.e.set(array, BSSimpleArray::pBuffer, 9);
        w.e.set(array, BSSimpleArray::iSize, 9);
        w.e.call_log = Some(vec![]);
        // The shared doubles stand in for 009e9950, 009e9980 and 009e99a0.
        assert_eq!(fn_009e9950(&mut w.e, array), array);
        assert_eq!(w.e.mem.u32(array.addr()), AVOID_ARRAY_VTABLE);
        assert_eq!(w.e.get(array, BSSimpleArray::pBuffer), 0);
        assert_eq!(w.e.get(array, BSSimpleArray::iSize), 0);
        assert!(calls(&w.e, AVOID_DATA_CONSTRUCT_RANGE).is_empty());
        w.e.mem.set_u32(array.addr(), 0);
        w.e.call(0x009e_9a20, &args![array, 0u32]);
        assert_eq!(w.e.mem.u32(array.addr()), AVOID_ARRAY_VTABLE);
        assert_eq!(
            calls(&w.e, SIMPLE_ARRAY_DESTRUCT),
            vec![vec![array.addr(), 1]]
        );
        assert!(calls(&w.e, OPERATOR_DELETE).is_empty());
        let back = w.e.call(0x009e_9a20, &args![array, 1u32]).u32();
        assert_eq!(back, array.addr());
        assert_eq!(calls(&w.e, OPERATOR_DELETE), vec![vec![array.addr()]]);
    }

    #[test]
    fn avoidance_array_buffer_set_up() {
        let mut w = world();
        let array: Ptr<BSSimpleArray> = w.e.new_object();
        let vtable = w.e.mem.alloc(8);
        w.e.mem.set_u32(array.addr(), vtable);
        w.e.mem
            .set_u32(vtable + SIMPLE_ARRAY_SLOT_ALLOCATE, FAKE + 22);
        w.e.register(FAKE + 22, |_, a| ret_u32(0x8000 + a[1]));
        w.e.call_log = Some(vec![]);
        w.e.call(0x009e_9b20, &args![array, 2u32, 5u32]);
        assert_eq!(calls(&w.e, FAKE + 22), vec![vec![array.addr(), 5]]);
        assert_eq!(w.e.get(array, BSSimpleArray::pBuffer), 0x8005);
        assert_eq!(w.e.get(array, BSSimpleArray::iReservedSize), 5);
        assert_eq!(w.e.get(array, BSSimpleArray::iSize), 5);
        assert_eq!(
            calls(&w.e, AVOID_DATA_CONSTRUCT_RANGE),
            vec![vec![array.addr(), 0x8005, 5]]
        );
        // Reserve only: a buffer, nothing constructed.
        w.e.call_log = Some(vec![]);
        w.e.call(0x009e_9b20, &args![array, 6u32, 0u32]);
        assert_eq!(w.e.get(array, BSSimpleArray::pBuffer), 0x8006);
        assert_eq!(w.e.get(array, BSSimpleArray::iReservedSize), 6);
        assert_eq!(w.e.get(array, BSSimpleArray::iSize), 0);
        assert!(calls(&w.e, AVOID_DATA_CONSTRUCT_RANGE).is_empty());
    }

    #[test]
    fn avoidance_array_add_copies_the_entry() {
        let mut w = world();
        let array: Ptr<BSSimpleArray> = w.e.new_object();
        let buffer = w.e.mem.alloc(0x20);
        w.e.set(array, BSSimpleArray::pBuffer, buffer);
        w.e.register(SIMPLE_ARRAY_GROW, |_, _| ret_u32(1));
        let data = w.e.mem.alloc(8);
        w.e.mem.set_u32(data, 0x1234);
        w.e.mem.set_u32(data + 4, 0x7f80_0001);
        w.e.call_log = Some(vec![]);
        assert_eq!(fn_009e99a0(&mut w.e, array, Ptr::new(data)), 1);
        assert_eq!(
            calls(&w.e, AVOID_DATA_CONSTRUCT_RANGE),
            vec![vec![array.addr(), buffer + 8, 1]]
        );
        assert_eq!(w.e.mem.u32(buffer + 8), 0x1234);
        // `fDetection` goes through the FPU: the signalling NaN is quieted.
        assert_eq!(w.e.mem.u32(buffer + 12), 0x7fc0_0001);
        let copy = w.e.mem.alloc(8);
        let back = w.e.call(0x009e_99f0, &args![copy, data]).u32();
        assert_eq!(back, copy);
        assert_eq!(w.e.mem.u32(copy + 4), 0x7fc0_0001);
    }
}
