//! `fallout/ai/movement/actormover.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `ActorMover` walks one actor along a path. It owns the current path
//! request (`spCurrentRequest`, +0x1C), the solved path (`spCurrentPath`,
//! +0x20) and an `ActorPathHandler` object (+0x24) that follows the path
//! and produces the movement. Its vtable is at `0109223c` (read from the
//! exe: slot 0 the scalar deleting destructor `009daec0`, 1
//! `SetMoveModePreference`, 2 `ClearMoveModePreference`, 3 `ForceMoveMode`,
//! 4 `ClearForcedMoveMode`, 5 `Update`, 6 `UpdateTurnTimer` (`004534f0`),
//! 7 `GetPreferredMoveMode`, 8 `GetMoveMode`, 9 `GetMoveDelta`, 10
//! `SaveGame`, 11 `LoadGame`, 12 `InitLoadGame`, 13 `FinishLoadGame`, 14
//! `Revert`). The PC object is 0x88 bytes (the Xbox debug build adds
//! the path debug data after that); the derived class constructor
//! `009e9d00` continues at +0x88.
//!
//! Conventions of this file:
//! - The layout is the Xbox PDB's (`pdbdump type ActorMover`); every
//!   offset below was checked against the PC code. The enum names of
//!   `ActorMover::ACTOR_MOVER_STATE`, `PATHING_REQUEST_TYPE` and
//!   `PATHING_MESSAGE_TYPE` are the PDB's.
//! - The compiler's exception-unwinding frames (`FS:[0]`) are not
//!   translated: the functions that have one carry a `Translation
//!   incomplete` marker. Their normal-path destructor calls are translated.
//! - Receivers (`ECX`) of `__thiscall` callees are the first argument word.
//!   Stack arguments follow in the order the callee reads them (the
//!   decompiler hangs pushed words on the wrong call; every argument list
//!   here comes from the disassembly). An argument the code copies through
//!   the x87 stack (`FLD`/`FSTP`) is passed through [`fpu_copy`], which
//!   quiets a signalling NaN; one it copies through an integer register is
//!   passed unchanged.
//! - x87: float comparisons follow the flag tests of the disassembly (a
//!   strict `<` or `>`, and a NaN fails them); `float` stores round once,
//!   everything else is formed in `f64`.
//! - The `ActorPathHandler` (+0x24) virtual slots used here, by byte
//!   offset: 0x00 deleting destructor (argument 1), 0x04 and 0x0C start and
//!   update the movement (they return the result code that
//!   `fn_009dc5e0` handles), 0x10 the update without a path, 0x18 distance
//!   travelled, 0x1C percent travelled, 0x20 location of the next node,
//!   0x24 location of a node, 0x28 distance remaining, 0x2C approximate
//!   location, 0x30 current node index, 0x34 is a detailed handler, 0x38
//!   a path-change notice, 0x3C and 0x40 and 0x44 blocked/avoidance
//!   queries, 0x48 move mode, 0x4C move delta, 0x58 previous-path
//!   hand-over, 0x5C deleted-reference notice, 0x70 clear movement. The
//!   Xbox PDB lists names at these offsets; where one is quoted below the
//!   PC argument list was read from the code, not taken from the PDB.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiPoint3;
use crate::units::fallout_shared_pathfinding::pathfind::PathingLocation;

layout! {
    /// `ActorMover` (Xbox PDB), 0x88 bytes on the PC.
    pub struct ActorMover: 0x88 {
        /// `LookAtPoint` (Xbox PDB): the point the actor faces.
        0x04 LookAtPoint: Inline<NiPoint3>,
        /// `CachedMoveDelta` (Xbox PDB): the movement of the last update.
        0x10 CachedMoveDelta: Inline<NiPoint3>,
        /// `spCurrentRequest` (Xbox PDB): `NiPointer<PathingRequest>`.
        0x1C spCurrentRequest: Ptr,
        /// `spCurrentPath` (Xbox PDB): `NiPointer<PathingSolution>`.
        0x20 spCurrentPath: Ptr,
        /// `pCurrentPathHandler` (Xbox PDB): `ActorPathHandler*`.
        0x24 pCurrentPathHandler: Ptr,
        /// `pActor` (Xbox PDB): `Actor*`.
        0x28 pActor: Ptr,
        /// `DetectionDoorID` (Xbox PDB).
        0x2C DetectionDoorID: u32,
        /// `spMessageQueue` (Xbox PDB): `NiPointer<ActorPathingMessageQueue>`.
        0x30 spMessageQueue: Ptr,
        /// `uiPreferredMoveMode` (Xbox PDB): the constructor stores `0x101`.
        0x34 uiPreferredMoveMode: u32,
        /// `uiForcedMoveMode` (Xbox PDB).
        0x38 uiForcedMoveMode: u32,
        /// `iCachedLastValidMoveMode` (Xbox PDB).
        0x3C iCachedLastValidMoveMode: u32,
        /// `iPreviousNode` (Xbox PDB).
        0x40 iPreviousNode: u16,
        /// `iNextNode` (Xbox PDB).
        0x42 iNextNode: u16,
        /// `FailedPathDestination` (Xbox PDB).
        0x44 FailedPathDestination: Inline<PathingLocation>,
        /// `eState` (Xbox PDB, `ACTOR_MOVER_STATE`; stored as a dword).
        0x6C eState: u32,
        /// `bPathingFailed` (Xbox PDB).
        0x70 bPathingFailed: bool,
        /// `bForceMoveMode` (Xbox PDB).
        0x71 bForceMoveMode: bool,
        /// `bStopMovementNextUpdate` (Xbox PDB).
        0x72 bStopMovementNextUpdate: bool,
        /// `bFaceTargetPoint` (Xbox PDB).
        0x73 bFaceTargetPoint: bool,
        /// `bWaitingForPath` (Xbox PDB).
        0x74 bWaitingForPath: bool,
        /// `bActorChangedProcessLevel` (Xbox PDB).
        0x75 bActorChangedProcessLevel: bool,
        /// `bInDialog` (Xbox PDB).
        0x76 bInDialog: bool,
        /// `bHaveValidDeltaThisFrame` (Xbox PDB).
        0x77 bHaveValidDeltaThisFrame: bool,
        /// `bStopCurrentMoveAnim` (Xbox PDB).
        0x78 bStopCurrentMoveAnim: bool,
        /// `iLastFailureTimeStamp` (Xbox PDB).
        0x7C iLastFailureTimeStamp: u32,
        /// `iFirstFailureTimeStamp` (Xbox PDB).
        0x80 iFirstFailureTimeStamp: u32,
        /// `iContinuedFailureCount` (Xbox PDB).
        0x84 iContinuedFailureCount: i32,
    }
}

/// `ACTOR_MOVER_STATE_INIT` (Xbox PDB).
const STATE_INIT: u32 = 1;
/// `ACTOR_MOVER_STATE_MOVING` (Xbox PDB).
const STATE_MOVING: u32 = 3;
/// `ACTOR_MOVER_STATE_ROTATING` (Xbox PDB).
const STATE_ROTATING: u32 = 4;
/// `ACTOR_MOVER_STATE_FINISHED` (Xbox PDB).
const STATE_FINISHED: u32 = 6;
/// `ACTOR_MOVER_STATE_FAILED` (Xbox PDB).
const STATE_FAILED: u32 = 7;
/// `ACTOR_MOVER_STATE_FAILED_PROCESS_UPDATE` (Xbox PDB).
const STATE_FAILED_PROCESS_UPDATE: u32 = 8;

/// The vtable the constructor installs.
const VTABLE: u32 = 0x0109_223c;
/// A global `NiPoint3` the code copies (as three plain words) to clear a
/// point; it reads 0, 0, 0 in the executable's data.
const UNSET_POINT: u32 = 0x011f_426c;
/// `50.0` (`double`).
const FIFTY: u32 = 0x0101_e2c0;
/// `0.0` (`double`).
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// The player character pointer (`ECX` of `PlayerCharacter::IsSleepingorResting`).
const PLAYER_CHARACTER: u32 = 0x011d_ea3c;
/// A global object whose method `0042ce10` tests bit 1 of its dword +0x244.
const GLOBAL_FLAGS_OBJECT: u32 = 0x011d_df38;
/// A global object whose method `0084d030` returns the float at +0x0C.
const GLOBAL_TIMER_OBJECT: u32 = 0x011f_6394;

// Callees outside this file (all called by address).

/// `operator new` (`00401000`, one argument).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `operator delete` (`00401030`, one argument).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The game heap allocator (`00aa13e0`, cdecl, one argument).
const ALLOCATE: u32 = 0x00aa_13e0;
/// An empty function the code calls on `this` at the end of many methods.
const DEBUG_STUB: u32 = 0x0048_3710;
/// `_ftol2_sse` (`00ec62c0`): truncates the float in ST0 (passed as an
/// `f64` argument in the uniform form).
const FTOL: u32 = 0x00ec_62c0;
/// Decrements the count at +4 of the object in `ECX` and calls its vtable
/// slot 1 when it reaches zero.
const NI_REF_RELEASE: u32 = 0x0040_1970;
/// Increments the count at +4 of the object in `ECX`.
const NI_REF_ACQUIRE: u32 = 0x0040_f6e0;
/// Returns the pointer held by the `NiPointer` whose address is `ECX`.
const GET_NI_POINTER: u32 = 0x0055_9450;
/// `NiPointer::operator=(T*)`: stores the argument in the `NiPointer` at
/// `ECX`, releasing the old object and acquiring the new one.
const NI_POINTER_SET: u32 = 0x0066_b0d0;
/// The same assignment for the message queue pointer (the count lives at
/// +0x18 of the queue object).
const QUEUE_POINTER_SET: u32 = 0x0090_57d0;
/// `NiPointer(T*)` constructor (acquires).
const NI_POINTER_CONSTRUCT: u32 = 0x0063_3c90;
/// The same constructor for the message queue pointer.
const QUEUE_POINTER_CONSTRUCT: u32 = 0x006e_bf60;
/// The message queue pointer's destructor.
const QUEUE_POINTER_DESTRUCT: u32 = 0x006e_bf90;
/// `NiPointer` destructor (releases the held object).
const NI_POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// Copy constructor of the message queue pointer (argument: address of the
/// source `NiPointer`).
const QUEUE_POINTER_COPY: u32 = 0x006e_bf30;
/// Copy constructor of the request pointer.
const REQUEST_POINTER_COPY: u32 = 0x0055_9a40;
/// Constructor of the 0x1C-byte object the message queue pointer holds.
const QUEUE_CONSTRUCT: u32 = 0x008d_f900;

/// `PathingLocation::PathingLocation()` (`006dcbb0`).
const LOCATION_CONSTRUCT: u32 = 0x006d_cbb0;
/// Destructor body of a `PathingLocation` (sets its vtable).
const LOCATION_DESTRUCT: u32 = 0x004f_f7e0;
/// `PathingLocation` from a position pointer, a cell and a world space.
const LOCATION_FROM_POSITION: u32 = 0x006d_cee0;
/// `PathingLocation::operator=`.
const LOCATION_ASSIGN: u32 = 0x006d_cce0;
/// `PathingLocation::GetCell`.
const LOCATION_GET_CELL: u32 = 0x006d_d4f0;
/// Returns the dword at +0x1C of `ECX` (the map names it
/// `PathingLocation::GetWorldspace`; it is also used on a path).
const DWORD_AT_1C: u32 = 0x0044_1110;
/// `PathingLocation::ClearTriangleData`.
const LOCATION_CLEAR_TRIANGLE: u32 = 0x006d_dbe0;
/// Tests bit 1 (value 2) of the flag byte at +0x26 of a location.
const LOCATION_FLAG_2: u32 = 0x006a_9520;
/// The cell's flag test (bit 0 of the byte at +0x24).
const CELL_FLAG_0: u32 = 0x0042_5fd0;

/// Address of the origin location of the request (`ECX + 0x0C`).
const REQUEST_ORIGIN: u32 = 0x0048_d150;
/// Address of the destination location of the request (`ECX + 0x34`).
const REQUEST_DESTINATION: u32 = 0x006a_9540;
/// The request byte at +0x9B (`bAllowIncompletePath`, Xbox PDB).
const REQUEST_ALLOW_INCOMPLETE: u32 = 0x006a_9560;
/// The request byte at +0x99 (`bFaceTargetAtGoal`, Xbox PDB).
const REQUEST_FACE_TARGET_AT_GOAL: u32 = 0x0050_5c00;
/// The request float at +0x8C (`fGoalAngle`, Xbox PDB).
const REQUEST_GOAL_ANGLE: u32 = 0x0050_7e10;
/// The request float at +0x70 (`fGoalZDelta`, Xbox PDB).
const REQUEST_GOAL_Z_DELTA: u32 = 0x0051_b860;
/// The request float at +0x74 (`fTargetRadius`, Xbox PDB).
const REQUEST_TARGET_RADIUS: u32 = 0x009b_0bd0;
/// `PathingRequest::PathingRequest()`.
const REQUEST_CONSTRUCT: u32 = 0x006e_2420;
/// `PathingRequest` destructor.
const REQUEST_DESTRUCT: u32 = 0x006e_2620;
/// Sets the actor of a request (`006e2b20`, argument: the actor).
const REQUEST_SET_ACTOR: u32 = 0x006e_2b20;
/// Fills a request from an actor (`006e29f0`).
const REQUEST_INIT_FROM_ACTOR: u32 = 0x006e_29f0;
/// Sets the request's destination from a location (`006d1bc0`).
const REQUEST_SET_DESTINATION: u32 = 0x006d_1bc0;
/// Sets the request's target radius (`006d3b00`, a float).
const REQUEST_SET_TARGET_RADIUS: u32 = 0x006d_3b00;
/// Sets the request's avoid node array (`006d61e0`).
const REQUEST_SET_AVOID_NODES: u32 = 0x006d_61e0;
/// Sets the request byte at +0x98 (`bCantOpenDoors`, Xbox PDB).
const REQUEST_SET_CANT_OPEN_DOORS: u32 = 0x006e_2b50;
/// Sets the request byte at +0x9D (`bCanSwim`, Xbox PDB).
const REQUEST_SET_CAN_SWIM: u32 = 0x006e_2b70;
/// Sets the request byte at +0x9E (`bCanFly`, Xbox PDB).
const REQUEST_SET_CAN_FLY: u32 = 0x006e_2b90;
/// Returns the default target radius (`PathingRequest::fDefaultRadius`
/// loaded into ST0).
const REQUEST_DEFAULT_RADIUS: u32 = 0x006e_25d0;
/// A request method taking a float (`008e7c20`, argument the float).
const REQUEST_SET_FLOAT: u32 = 0x008e_7c20;
/// Copy-constructor of a standard request (`PathingRequest(const
/// PathingRequest&)`).
const REQUEST_COPY_CONSTRUCT: u32 = 0x006e_26a0;
/// Checks a request against a path (`006d53b0`, cdecl: request, path).
const REQUEST_CHECK_PATH: u32 = 0x006d_53b0;
/// Checks a request against a path from a node (`006d5420`, cdecl:
/// request, path, node index).
const REQUEST_CHECK_PATH_FROM: u32 = 0x006d_5420;

/// The path's node array count (`005a4320`: `ECX + 8`'s element count).
const PATH_NODE_COUNT: u32 = 0x005a_4320;
/// The path's second array count (`008b6800`: `ECX + 0x20`'s count).
const PATH_COUNT_20: u32 = 0x008b_6800;
/// Tests whether the array at `ECX` is empty (`[ECX + 8] == 0`).
const ARRAY_IS_EMPTY: u32 = 0x0076_b610;
/// The byte at +0x40 of a path.
const PATH_FLAG_40: u32 = 0x006b_05d0;
/// A path's element of its second array, or 0 when that array is empty.
const PATH_SECOND_ARRAY_ELEMENT: u32 = 0x006b_05a0;
/// `(dword at ECX) & 3 == 3`.
const NODE_FLAGS_ARE_3: u32 = 0x006e_2330;
/// Tests the node in `ECX` against an actor (`006e22f0`).
const NODE_TEST_ACTOR: u32 = 0x006e_22f0;
/// Clears the path's second array and resets its indexes to -1.
const PATH_RESET_LOADED: u32 = 0x006c_93a0;
/// Returns the path's node at an index (`006e78b0`, argument the index).
const PATH_NODE_AT: u32 = 0x006e_78b0;
/// Returns `ECX + 4` (the location inside a node).
const NODE_LOCATION: u32 = 0x0071_7e50;
/// Returns the dword at +0x18 of the path (the first loaded node index).
const PATH_FIRST_LOADED: u32 = 0x0096_11e0;
/// Returns the node at the end of the path (`ECX + 8`'s last element).
const PATH_LAST_NODE: u32 = 0x009d_fd50;

/// `PathManager::QInstance`.
const PATH_MANAGER_INSTANCE: u32 = 0x0047_d0b0;
/// `PathManager::BuildPath` (arguments: request pointer, message queue
/// pointer, a flag).
const BUILD_PATH: u32 = 0x006e_b9d0;

/// `Actor` (`MobileObject`) `+0x68` process pointer.
const GET_PROCESS: u32 = 0x008d_8520;
/// The parent cell of the reference (`+0x40`).
const GET_PARENT_CELL: u32 = 0x008d_6f30;
/// `TESObjectREFR::GetWorldSpace`.
const GET_WORLD_SPACE: u32 = 0x0057_5d70;
/// `TESObjectREFR::GetRelevantWaterHeight` (float in ST0).
const GET_WATER_HEIGHT: u32 = 0x0057_b0a0;
/// Copies the position at `ECX + 4` to the argument and returns it.
const GET_POSITION: u32 = 0x005c_3420;
/// Returns `ECX + 0x24`.
const GET_ADDRESS_24: u32 = 0x0043_0830;
/// `MobileObject::GetCurrentProcessType`.
const GET_PROCESS_TYPE: u32 = 0x0093_1850;
/// `MobileObject::GetDesiredProcessLevel`.
const GET_DESIRED_PROCESS_LEVEL: u32 = 0x0093_34b0;
/// The process's dword at +0x28.
const PROCESS_LEVEL: u32 = 0x0045_cd60;
/// The parent cell's test of the value 6 (`00450fd0 == 6`).
const CELL_IS_SIX: u32 = 0x0045_0ff0;
/// `Actor::GetOutofFurnitureQuick`.
const ACTOR_LEAVE_FURNITURE: u32 = 0x0088_d640;
/// Puts the actor into a movement state (`00881860`, argument a flag).
const ACTOR_STOP_MOVEMENT: u32 = 0x0088_1860;
/// `Actor::CanMove`.
const ACTOR_CAN_MOVE: u32 = 0x0088_43a0;
/// A second actor movement test (`00884750`).
const ACTOR_MOVEMENT_TEST: u32 = 0x0088_4750;
/// `Actor::GetCurrentSpeed` (ST0, no stack argument).
const ACTOR_GET_SPEED: u32 = 0x008a_0b10;
/// `Actor::GetAnimAction`.
const ACTOR_GET_ANIM_ACTION: u32 = 0x008a_7570;
/// `Actor::IsVisible` (tests a flag mask in the dword at +0x11C).
const ACTOR_TEST_FLAGS: u32 = 0x008b_0190;
/// The actor's slot 0x218 result.
const ACTOR_SLOT_218: u32 = 0x008e_ea60;
/// Tests bit 4 of the dword at +8 of the actor.
const ACTOR_FORM_FLAG_10: u32 = 0x0050_d4a0;
/// `PlayerCharacter::IsSleepingorResting`.
const PLAYER_IS_SLEEPING: u32 = 0x0094_df60;
/// A test of the global object (`0042ce10`).
const GLOBAL_FLAGS_TEST: u32 = 0x0042_ce10;
/// Returns the float at +0x0C of the global timer object.
const GLOBAL_TIMER_FLOAT: u32 = 0x0084_d030;
/// Returns the base object of an actor (`004181e0`).
const ACTOR_BASE: u32 = 0x0041_81e0;
/// A flag of the actor's base (`005f0b50`).
const BASE_CAN_SWIM: u32 = 0x005f_0b50;
/// A flag of the actor's base (`005f0c40`).
const BASE_CAN_FLY: u32 = 0x005f_0c40;
/// Animation: derives the movement delta (`00494390`).
const ANIMATION_GET_DELTA: u32 = 0x0049_4390;
/// Animation: element of a table at +0xE0 (`00491040`).
const ANIMATION_ELEMENT: u32 = 0x0049_1040;
/// Animation: `ShouldBeMoving`.
const ANIMATION_SHOULD_BE_MOVING: u32 = 0x0049_5be0;
/// The dword at +0x44 of the table entry (`008041a0`).
const ENTRY_VALUE: u32 = 0x0080_41a0;
/// The dword at +0x0C of a process (`0084e3a0`).
const PROCESS_DOOR_ID: u32 = 0x0084_e3a0;
/// The byte at +0x73 (`bFaceTargetPoint`).
const FACE_TARGET_POINT: u32 = 0x008b_3d10;

/// `Point3 - Point3` (`00439ef0`: receiver, result pointer, right operand;
/// returns the result pointer).
const POINT_SUBTRACT: u32 = 0x0043_9ef0;
/// `x*x + y*y` of a point as a float in ST0 (`00595c80`).
const POINT_LENGTH_SQUARED_XY: u32 = 0x0059_5c80;
/// The length of a point (`00457990`, float in ST0).
const POINT_LENGTH: u32 = 0x0045_7990;
/// `fabs`-like helper (`00408840`, one float on the stack, float in ST0).
const FLOAT_ABS: u32 = 0x0040_8840;
/// A float helper (`004e4450`, one float on the stack, float in ST0).
const FLOAT_ROUND: u32 = 0x004e_4450;
/// `GetZAngleFromVector` (cdecl, pointer to the vector, float in ST0).
const Z_ANGLE_FROM_VECTOR: u32 = 0x004b_13c0;
/// `NiPoint3::NiPoint3(x, y, z)` (`00416870`).
const POINT_CONSTRUCT: u32 = 0x0041_6870;

/// Constructor of the detailed handler (`009dff40`: actor, mover).
const DETAILED_HANDLER_CONSTRUCT_SIMPLE: u32 = 0x009d_ff40;
/// Constructor of the detailed handler used for a path (`009e0190`: actor,
/// mover, path field address, request).
const DETAILED_HANDLER_CONSTRUCT: u32 = 0x009e_0190;
/// Constructor of the virtual handler used for a path (`009ea770`: actor,
/// mover, path field address, request).
const VIRTUAL_HANDLER_CONSTRUCT: u32 = 0x009e_a770;
/// Hands an actor/request/handler to the detailed handler logic (`009e71b0`,
/// cdecl: actor, request, handler).
const HANDLER_ATTACH: u32 = 0x009e_71b0;
/// Reports the end of a path to a global object (`0087b650`, arguments:
/// actor, position pointer).
const REPORT_PATH_END: u32 = 0x0087_b650;
/// The global object of `REPORT_PATH_END` (`004537b0`).
const REPORT_TARGET_OBJECT: u32 = 0x0045_37b0;
/// A global test (`008c7aa0`, no arguments).
const GLOBAL_TEST: u32 = 0x008c_7aa0;
/// `ActorMover::SetPathingFailed`.
const SET_PATHING_FAILED: u32 = 0x009d_e490;

// Callees of the last functions (`009de3e0` and after). Names that the
// Xbox PDB gives are marked; the others describe what the code shows.

/// The `TES` pointer (`011dea10`), the receiver of `0045af00` and
/// `0045cbc0`.
const TES_POINTER: u32 = 0x011d_ea10;
/// `TES::GetNavMeshInfoMap` (`0045af00`, receiver only).
const NAV_MESH_INFO_MAP: u32 = 0x0045_af00;
/// Packs two 16-bit grid coordinates into one word (`00587410`, cdecl: x, y).
const PACK_GRID_COORDS: u32 = 0x0058_7410;
/// Looks a pair up in the nav mesh info map (`006b7850`: map, first,
/// second).
const NAV_MESH_INFO_FIND: u32 = 0x006b_7850;
/// `0045cbc0` on the `TES` pointer: the water height at a position in a
/// cell (position, cell); the float comes back in ST0.
const WATER_HEIGHT_AT: u32 = 0x0045_cbc0;
/// `008ad1c0` (receiver: the actor; position, float, two words taken from
/// the destination location, 0, 1).
const ACTOR_PLACE: u32 = 0x008a_d1c0;
/// The dword at +0x18 of `ECX` (`009611e0`).
const DWORD_AT_18: u32 = 0x0096_11e0;
/// A location method called on the failed destination when the failure is
/// cleared (`006d2c00`; name unconfirmed).
const LOCATION_RESET: u32 = 0x006d_2c00;
/// The time in milliseconds (`00825c00`, receiver `011f6394`).
const TIMER_NOW: u32 = 0x0082_5c00;
/// Time-out of a failure when the actor passes `IsVisible(7)` (`0109222c`,
/// a float).
const FAILURE_TIMEOUT_VISIBLE: u32 = 0x0109_222c;
/// Time-out of a failure otherwise (`01092230`, a float).
const FAILURE_TIMEOUT_HIDDEN: u32 = 0x0109_2230;
/// Time-out when the request's ends lie on different sides of the water
/// (`01092234`, a float).
const FAILURE_TIMEOUT_WATER: u32 = 0x0109_2234;
/// `0.001` (`double`, `01016978`): milliseconds to seconds.
const MILLISECONDS_TO_SECONDS: u32 = 0x0101_6978;
/// The flag mask the code passes to `Actor::IsVisible` (`008b0190`).
const IS_VISIBLE_MASK: u32 = 7;

/// `Actor::IsFollowing_ov2` (Xbox PDB name, `008bc7f0`: actor, other).
const IS_FOLLOWING: u32 = 0x008b_c7f0;
/// `008985d0` (receiver: the actor).
const ACTOR_FOLLOW_FALLBACK: u32 = 0x0089_85d0;
/// A point's length in ST0 (`00589850`).
const DIFFERENCE_LENGTH: u32 = 0x0058_9850;
/// `PathingLocation::PathingLocation_ov3` (Xbox PDB, `006dcd70`): builds a
/// location at an actor.
const LOCATION_FROM_ACTOR: u32 = 0x006d_cd70;
/// `PathingLocation::ResolveNavMeshInfo` (Xbox PDB, `006dd6f0`).
const LOCATION_RESOLVE_NAV_MESH: u32 = 0x006d_d6f0;
/// A test of a location (`006dd6a0`).
const LOCATION_IS_RESOLVED: u32 = 0x006d_d6a0;
/// A test of a location (`006dd540`).
const LOCATION_NAV_MESH_TEST: u32 = 0x006d_d540;
/// A request method taking a location (`006d3ac0`).
const REQUEST_SET_LOCATION: u32 = 0x006d_3ac0;
/// A request method taking a float (`006d3ae0`).
const REQUEST_SET_SECOND_FLOAT: u32 = 0x006d_3ae0;
/// A float of the actor in ST0 (`008be280`).
const ACTOR_FLOAT: u32 = 0x008b_e280;
/// A request method taking a byte (`008ad8f0`).
const REQUEST_SET_BYTE_A: u32 = 0x008a_d8f0;
/// A request method taking a byte (`008df1c0`).
const REQUEST_SET_BYTE_B: u32 = 0x008d_f1c0;
/// `NiPoint3::NiPoint3()` (`006815c0`).
const POINT_CONSTRUCT_DEFAULT: u32 = 0x0068_15c0;
/// `PathingSolution::PathingSolution()` (`006e7650`).
const PATH_CONSTRUCT: u32 = 0x006e_7650;
/// `PathingSolution` destructor (`006e7720`).
const PATH_DESTRUCT: u32 = 0x006e_7720;
/// Solves a request into a path object (`006d0900`, cdecl: request, path).
const SOLVE_REQUEST: u32 = 0x006d_0900;
/// The path's total length (`006e78d0`, ST0).
const PATH_LENGTH: u32 = 0x006e_78d0;
/// A point on the path (`006e8790`: path, result pointer, two floats);
/// returns the result pointer.
const PATH_POINT_AT: u32 = 0x006e_8790;
/// Moves a reference to another cell or world space (`00573800`, cdecl:
/// reference, cell, world space).
const MOVE_REF_TO_NEW_SPACE: u32 = 0x0057_3800;
/// Slot 0x2A8 of the actor takes a position pointer (the actor is placed
/// there; the Xbox PDB name at the offset does not match the PC slot).
const ACTOR_SLOT_PLACE: u32 = 0x2a8;
/// `TESObjectREFR::SetLocationOnReference` (`00575830`, argument the
/// position).
const SET_LOCATION_ON_REFERENCE: u32 = 0x0057_5830;
/// `TESWorldSpace::GetCellFromWorldCoord` (`00587550`, argument the
/// position).
const GET_CELL_FROM_WORLD_COORD: u32 = 0x0058_7550;
/// The second report to the global of [`REPORT_TARGET_OBJECT`] (`0087b190`,
/// argument: the actor).
const REPORT_MOVE_DONE: u32 = 0x0087_b190;
/// The target radius of the replacement request (`01017b78`, a float).
const FAILURE_TARGET_RADIUS: u32 = 0x0101_7b78;
/// `0.6` as a double (`010290b8`): scales the actor's float for the
/// replacement request.
const ACTOR_FLOAT_SCALE: u32 = 0x0102_90b8;
/// `128.0` (`0101e704`, a float): how far short of the path's end the actor
/// is placed.
const PATH_MARGIN_SHORT: u32 = 0x0101_e704;
/// `256.0` (`01032adc`, a float): the same for a large actor float.
const PATH_MARGIN_LONG: u32 = 0x0103_2adc;
/// `0.5` (`double`, `01011588`).
const HALF: u32 = 0x0101_1588;
/// `100.0` (`double`, `01017a40`).
const HUNDRED: u32 = 0x0101_7a40;
/// Element `index` of an array of 0x30-byte elements (`006e8f40`:
/// array, index).
const ARRAY_ELEMENT_BY_INDEX: u32 = 0x006e_8f40;
/// A test of a path node (`006ca830`).
const NODE_TEST_FLAG: u32 = 0x006c_a830;

/// Write of `size` bytes from a pointer to the save buffer (`00865e50`:
/// buffer, pointer, size, 0).
const SAVE_BYTES: u32 = 0x0086_5e50;
/// `BGSSaveGameBuffer::SaveFormID` (Xbox PDB, `00865db0`: buffer, form id,
/// 0).
const SAVE_FORM_ID: u32 = 0x0086_5db0;
/// Read of `size` bytes into a pointer from the load buffer (`00864980`:
/// buffer, pointer, size).
const LOAD_BYTES: u32 = 0x0086_4980;
/// `BGSLoadGameBuffer::LoadFormID_ov2` (Xbox PDB name, `008648e0`: buffer,
/// pointer for the form id).
const LOAD_FORM_ID: u32 = 0x0086_48e0;
/// `PathingSolution::SaveGame` (Xbox PDB, `006e8b10`).
const PATH_SAVE_GAME: u32 = 0x006e_8b10;
/// `PathingSolution::LoadGame` (Xbox PDB name, `006e8d50`).
const PATH_LOAD_GAME: u32 = 0x006e_8d50;
/// `PathingRequest::CreatePathingRequest` (Xbox PDB name, `006e2f80`, cdecl,
/// one word: the request type).
const CREATE_PATHING_REQUEST: u32 = 0x006e_2f80;
/// A test of the part of the process at +4 (`008c6b00`, one argument 0).
const PROCESS_PART_TEST: u32 = 0x008c_6b00;
/// Debug print (`005b5e40`, cdecl: format and arguments).
const PRINT_LINE: u32 = 0x005b_5e40;
/// The format string `PATHFINDING: Actor %s (0x%08x) is using a stale path
/// (the navmesh changed), forcing a failure`.
const STALE_PATH_FORMAT: u32 = 0x0109_2278;
/// `00984230` on a request: the address of an `NiPointer` inside it.
const REQUEST_HOLDER: u32 = 0x0098_4230;
/// `004fb070` (receiver: the object the actor's slot 0x428 returns).
const SLOT_OBJECT_HOLDER: u32 = 0x004f_b070;
/// `009d9f20` (receiver: the result of `004fb070`).
const SLOT_OBJECT_VALUE: u32 = 0x009d_9f20;
/// `009d9ec0` (receiver: the request, argument: the address of an
/// `NiPointer`).
const REQUEST_ATTACH: u32 = 0x009d_9ec0;

/// `ActorPathingMessage::ActorPathingMessage` (8 bytes: type, `NiPointer`).
const MESSAGE_CONSTRUCT: u32 = 0x006e_9bd0;
/// `ActorPathingMessage::~ActorPathingMessage`.
const MESSAGE_DESTRUCT: u32 = 0x0080_0190;

/// `PATHING_REQUEST_TYPE` (Xbox PDB) values with a request class of their
/// own, each with the object size and the copy-constructor the game uses
/// to build it from the queued request.
const REQUEST_CLASSES: [(u32, u32, u32); 7] = [
    (0, 0xb0, REQUEST_COPY_CONSTRUCT),
    (3, 0xb4, 0x006e_5400),
    (4, 0xdc, 0x006e_5640),
    (5, 0xd8, 0x006e_5940),
    (6, 0xbc, 0x006e_6010),
    (7, 0xc0, 0x006e_5db0),
    (8, 0x10c, 0x006e_4d20),
];

/// `FLD`/`FSTP` of a `float` quiets a signalling NaN (sets the top mantissa
/// bit) and keeps the sign and the other bits.
fn fpu_copy(value: f32) -> f32 {
    let bits = value.to_bits();
    if bits & 0x7f80_0000 == 0x7f80_0000 && bits & 0x007f_ffff != 0 {
        f32::from_bits(bits | 0x0040_0000)
    } else {
        value
    }
}

/// Copies 12 bytes as three plain dwords (the game's `MOV` sequences keep
/// every bit).
fn copy_point(e: &mut Engine, from: u32, to: u32) {
    for offset in [0, 4, 8] {
        let word = e.mem.u32(from + offset);
        e.mem.set_u32(to + offset, word);
    }
}

/// The pointer held by the `NiPointer` at `address`.
fn ni_pointer_value(e: &mut Engine, address: u32) -> u32 {
    e.call(GET_NI_POINTER, &args![address]).u32()
}

/// `spCurrentRequest.get()`.
fn current_request(e: &mut Engine, this: Ptr<ActorMover>) -> u32 {
    ni_pointer_value(e, this.addr() + ActorMover::spCurrentRequest.off)
}

/// `spCurrentPath.get()`.
fn current_path(e: &mut Engine, this: Ptr<ActorMover>) -> u32 {
    ni_pointer_value(e, this.addr() + ActorMover::spCurrentPath.off)
}

/// Stores `value` in the `NiPointer` at `address` through `0066b0d0`.
fn set_ni_pointer(e: &mut Engine, address: u32, value: u32) {
    e.call(NI_POINTER_SET, &args![address, value]);
}

/// Deletes an `ActorPathHandler` through its deleting destructor (slot 0,
/// argument 1) when there is one.
fn delete_handler(e: &mut Engine, handler: u32) {
    if handler != 0 {
        e.vcall(handler, 0x00, &args![1u32]);
    }
}

/// Replaces the message queue with a new 0x1C-byte object (allocated by
/// `00aa13e0`, built by `008df900`) through `009057d0`.
fn reset_message_queue(e: &mut Engine, this: Ptr<ActorMover>) {
    let block = e.call(ALLOCATE, &args![0x1cu32]).u32();
    let queue = if block != 0 {
        e.call(QUEUE_CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    e.call(
        QUEUE_POINTER_SET,
        &args![this.addr() + ActorMover::spMessageQueue.off, queue],
    );
}

// Translated from 005f36f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorMover::GetPreferredMoveMode` (Xbox PDB; vtable slot 7): the dword
/// at +0x34. The linker folded other classes' identical getters into this
/// address (the guide notes one that returns `TES::pInteriorCell`).
pub fn fn_005f36f0(e: &mut Engine, this: Ptr<ActorMover>) -> u32 {
    e.get(this, ActorMover::uiPreferredMoveMode)
}

// Translated from 006e5cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPointer<PathingDebugGeometryData>::operator=(const NiPointer&)` (Xbox
/// PDB name; the same code serves every `NiPointer` instance): when the two
/// pointers differ, releases the old object, copies the pointer and
/// acquires the new object. Returns `this`.
pub fn ni_pointer_operator_assign(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let old = e.mem.u32(this.addr());
    let new = e.mem.u32(source.addr());
    if old != new {
        if old != 0 {
            e.call(NI_REF_RELEASE, &args![old]);
        }
        e.mem.set_u32(this.addr(), new);
        let stored = e.mem.u32(this.addr());
        if stored != 0 {
            e.call(NI_REF_ACQUIRE, &args![stored]);
        }
    }
    this
}

// Translated from 009dad00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009dad00 - C++ exception unwinding (FS:[0] frame) not translated
/// `ActorMover::ActorMover(Actor*)` (Xbox PDB): installs the vtable, clears
/// both points to the global unset point, builds the `NiPointer` members
/// and the failed-destination location, stores the actor and resets every
/// state field (`uiPreferredMoveMode` becomes `0x101`).
pub fn actor_mover_actor_mover(
    e: &mut Engine,
    this: Ptr<ActorMover>,
    actor: Ptr,
) -> Ptr<ActorMover> {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE);
    copy_point(e, UNSET_POINT, base + ActorMover::LookAtPoint.off);
    copy_point(e, UNSET_POINT, base + ActorMover::CachedMoveDelta.off);
    e.call(NI_POINTER_CONSTRUCT, &args![base + 0x1c, 0u32]);
    e.call(NI_POINTER_CONSTRUCT, &args![base + 0x20, 0u32]);
    e.call(QUEUE_POINTER_CONSTRUCT, &args![base + 0x30, 0u32]);
    e.call(LOCATION_CONSTRUCT, &args![base + 0x44]);
    e.set(this, ActorMover::eState, 0);
    e.set(this, ActorMover::pActor, actor);
    set_ni_pointer(e, base + 0x1c, 0);
    set_ni_pointer(e, base + 0x20, 0);
    e.call(QUEUE_POINTER_SET, &args![base + 0x30, 0u32]);
    e.set(this, ActorMover::pCurrentPathHandler, Ptr::NULL);
    e.set(this, ActorMover::DetectionDoorID, 0);
    e.set(this, ActorMover::uiPreferredMoveMode, 0x101);
    e.call(SET_PATHING_FAILED, &args![this, false]);
    e.set(this, ActorMover::iPreviousNode, 0xffff);
    e.set(this, ActorMover::iNextNode, 0xffff);
    e.set(this, ActorMover::bActorChangedProcessLevel, false);
    e.set(this, ActorMover::bHaveValidDeltaThisFrame, false);
    e.set(this, ActorMover::bStopMovementNextUpdate, false);
    e.set(this, ActorMover::bWaitingForPath, false);
    e.set(this, ActorMover::bInDialog, false);
    e.set(this, ActorMover::bFaceTargetPoint, false);
    e.set(this, ActorMover::bForceMoveMode, false);
    e.set(this, ActorMover::bStopCurrentMoveAnim, false);
    e.set(this, ActorMover::uiForcedMoveMode, 0);
    e.set(this, ActorMover::iCachedLastValidMoveMode, 0);
    e.set(this, ActorMover::iLastFailureTimeStamp, 0);
    e.set(this, ActorMover::iFirstFailureTimeStamp, 0);
    e.set(this, ActorMover::iContinuedFailureCount, 0);
    this
}

// Translated from 009daec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorMover::_scalar_deleting_destructor_` (Xbox PDB, vtable slot 0):
/// runs the destructor body and, when bit 0 of `flags` is set, frees the
/// object. Returns `this`.
pub fn actor_mover_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ActorMover>,
    flags: u32,
) -> Ptr<ActorMover> {
    fn_009daef0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 009daef0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009daef0 - C++ exception unwinding (FS:[0] frame) not translated
/// The destructor body of `ActorMover` (called by the scalar deleting
/// destructor `009daec0`): restores the vtable, clears the current data
/// and destroys the members.
pub fn fn_009daef0(e: &mut Engine, this: Ptr<ActorMover>) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE);
    actor_mover_clear_current_data(e, this);
    e.call(LOCATION_DESTRUCT, &args![base + 0x44]);
    e.call(QUEUE_POINTER_DESTRUCT, &args![base + 0x30]);
    e.call(NI_POINTER_DESTRUCT, &args![base + 0x20]);
    e.call(NI_POINTER_DESTRUCT, &args![base + 0x1c]);
}

// Translated from 009daf80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009daf80 - C++ exception unwinding (FS:[0] frame) not translated
/// `ActorMover::ClearCurrentData` (Xbox PDB): sets the state to finished,
/// drops the request and the path, deletes the path handler, replaces the
/// message queue when a path was being awaited (clearing that flag) and
/// calls the virtual `ClearForcedMoveMode`.
pub fn actor_mover_clear_current_data(e: &mut Engine, this: Ptr<ActorMover>) {
    let base = this.addr();
    e.set(this, ActorMover::eState, STATE_FINISHED);
    set_ni_pointer(e, base + 0x1c, 0);
    set_ni_pointer(e, base + 0x20, 0);
    let handler = e.get(this, ActorMover::pCurrentPathHandler).addr();
    if handler != 0 {
        delete_handler(e, handler);
        e.set(this, ActorMover::pCurrentPathHandler, Ptr::NULL);
    }
    if e.get(this, ActorMover::bWaitingForPath) {
        reset_message_queue(e, this);
        e.set(this, ActorMover::bWaitingForPath, false);
    }
    e.vcall(base, 0x10, &[]);
    e.call(DEBUG_STUB, &args![this]);
}

// Translated from 009db090 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009db090 - C++ exception unwinding (FS:[0] frame) not translated
/// Starts pathing for a request (called by `009dba30`): checks the request,
/// resets the mover's data and, when the origin and destination are close
/// on the same ground above the water, finishes at once (rotating to the
/// goal angle when the request asks for it). Otherwise it builds the
/// request object for the request's type, stores it, forwards it to a
/// path handler already present, creates the message queue if needed and
/// hands the request to `PathManager::BuildPath`; on success it waits for
/// the path (`bWaitingForPath`), on failure it fails the mover.
pub fn fn_009db090(e: &mut Engine, this: Ptr<ActorMover>, request: Ptr) -> bool {
    // The exe's frame (`EBP` frame, exception record, `SUB ESP,0x100`, `ESI` and the cookie):
    // 0x114 bytes, with 0xc bytes of outgoing arguments below it for `BuildPath`.
    e.with_stack(0xc + 0x114, |e, raw| {
        set_request_in_frame(e, this, request, raw.addr())
    })
}

/// [`fn_009db090`] in the exe's frame: `raw` is the outgoing argument area, the frame proper
/// starts at `raw + 0xc`.
fn set_request_in_frame(e: &mut Engine, this: Ptr<ActorMover>, request: Ptr, raw: u32) -> bool {
    let frame = raw + 0xc;
    let base = this.addr();
    e.vcall(request.addr(), 0x0c, &[]);
    e.set(this, ActorMover::eState, STATE_INIT);
    actor_mover_clear_data_for_new_pathing_request(e, this, false);

    let shortcut = pathing_shortcut(e, this, request.addr(), frame);
    if let Some(result) = shortcut {
        return result;
    }

    let kind = e.vcall(request.addr(), 0x10, &[]).u32();
    if let Some(&(_, size, construct)) = REQUEST_CLASSES.iter().find(|class| class.0 == kind) {
        let block = e.call(ALLOCATE, &args![size]).u32();
        let object = if block != 0 {
            e.call(construct, &args![block, request]).u32()
        } else {
            0
        };
        set_ni_pointer(e, base + 0x1c, object);
    }

    let held = current_request(e, this);
    let origin = e.call(REQUEST_ORIGIN, &args![held]).u32();
    e.call(LOCATION_CLEAR_TRIANGLE, &args![origin]);
    let held = current_request(e, this);
    let destination = e.call(REQUEST_DESTINATION, &args![held]).u32();
    e.call(LOCATION_CLEAR_TRIANGLE, &args![destination]);
    let actor = e.get(this, ActorMover::pActor);
    let held = current_request(e, this);
    e.call(REQUEST_SET_ACTOR, &args![held, actor]);
    e.call(DEBUG_STUB, &args![this]);
    let retry_count = e
        .get(this, ActorMover::iContinuedFailureCount)
        .wrapping_add(3) as u32;
    let held = current_request(e, this);
    fn_009db930(e, held, retry_count);

    let handler = e.get(this, ActorMover::pCurrentPathHandler).addr();
    if !e.get(this, ActorMover::bPathingFailed) && handler != 0 {
        let held = current_request(e, this);
        e.vcall(handler, 0x38, &args![held]);
    }
    let handler = e.get(this, ActorMover::pCurrentPathHandler).addr();
    if handler != 0 && e.vcall(handler, 0x34, &[]).bool() {
        let held = current_request(e, this);
        e.call(HANDLER_ATTACH, &args![actor, held, handler]);
    } else {
        let cell = e.call(GET_PARENT_CELL, &args![actor]).u32();
        if cell != 0 {
            let cell = e.call(GET_PARENT_CELL, &args![actor]).u32();
            if e.call(CELL_IS_SIX, &args![cell]).bool() {
                let held = current_request(e, this);
                e.call(HANDLER_ATTACH, &args![actor, held, 0u32]);
            }
        }
    }

    let player = e.global::<u32>(PLAYER_CHARACTER);
    let sleeping = e.call(PLAYER_IS_SLEEPING, &args![player]).bool();
    if ni_pointer_value(e, base + 0x30) == 0 {
        reset_message_queue(e, this);
    }
    // The two `NiPointer` arguments of `BuildPath` are copies the callee destroys, built in
    // their own argument slots: the flag at `frame - 4`, the queue copy at `frame - 8`
    // (009db836), the request copy at `frame - 0xc` (009db869).
    e.mem.set_u32(raw + 8, u32::from(!sleeping));
    e.call(QUEUE_POINTER_COPY, &args![raw + 4, base + 0x30]);
    e.call(REQUEST_POINTER_COPY, &args![raw, base + 0x1c]);
    let manager = e.call(PATH_MANAGER_INSTANCE, &[]).u32();
    let queue_copy = e.mem.u32(raw + 4);
    let request_copy = e.mem.u32(raw);
    let built = e
        .call(
            BUILD_PATH,
            &args![manager, request_copy, queue_copy, !sleeping],
        )
        .bool();
    if built {
        e.set(this, ActorMover::bWaitingForPath, true);
        true
    } else {
        actor_mover_clear_current_data(e, this);
        e.set(this, ActorMover::eState, STATE_FAILED);
        e.call(SET_PATHING_FAILED, &args![this, true]);
        false
    }
}

/// The first part of `009db090`: when the origin and the destination
/// locations are in the same area (cell, or world space outdoors), both
/// stand at least 50 above the actor's relevant water height, their
/// horizontal distance is within the target radius and their height
/// difference within the goal Z delta, the mover finishes at once. Returns
/// the function's result in that case.
/// `frame` is `009db090`'s frame: the positions land at +0xd8 (origin, `EBP-0x3c`), +0xcc
/// (destination, `EBP-0x48`), +0xb4 (goal, `EBP-0x60`) and +0xc0 (start, `EBP-0x54`), the
/// difference at +0xe8 (`EBP-0x2c`).
fn pathing_shortcut(
    e: &mut Engine,
    this: Ptr<ActorMover>,
    request: u32,
    frame: u32,
) -> Option<bool> {
    let destination = e.call(REQUEST_DESTINATION, &args![request]).u32();
    if e.call(LOCATION_FLAG_2, &args![destination]).bool() {
        return None;
    }
    let origin = e.call(REQUEST_ORIGIN, &args![request]).u32();
    let destination = e.call(REQUEST_DESTINATION, &args![request]).u32();
    let origin_cell = e.call(LOCATION_GET_CELL, &args![origin]).u32();
    let outdoors = origin_cell == 0 || !e.call(CELL_FLAG_0, &args![origin_cell]).bool();
    let same_area = if outdoors {
        let origin_space = e.call(DWORD_AT_1C, &args![origin]).u32();
        let destination_space = e.call(DWORD_AT_1C, &args![destination]).u32();
        origin_space == destination_space
    } else {
        origin_cell == e.call(LOCATION_GET_CELL, &args![destination]).u32()
    };
    let actor = e.get(this, ActorMover::pActor);
    let fifty: f64 = e.global(FIFTY);

    let origin_position = e.call(GET_POSITION, &args![origin, frame + 0xd8]).u32();
    let water = e.call(GET_WATER_HEIGHT, &args![actor]).f64() + fifty;
    let origin_below = (e.mem.f32(origin_position + 8) as f64) < water;
    let destination_position = e
        .call(GET_POSITION, &args![destination, frame + 0xcc])
        .u32();
    let water = e.call(GET_WATER_HEIGHT, &args![actor]).f64() + fifty;
    let destination_below = (e.mem.f32(destination_position + 8) as f64) < water;
    if !same_area || origin_below || destination_below {
        return None;
    }

    let goal = e
        .call(GET_POSITION, &args![destination, frame + 0xb4])
        .u32();
    let start = e.call(GET_POSITION, &args![origin, frame + 0xc0]).u32();
    e.call(POINT_SUBTRACT, &args![start, frame + 0xe8, goal]);
    let horizontal = e.call(POINT_LENGTH_SQUARED_XY, &args![frame + 0xe8]).f64();
    let radius = e.call(REQUEST_TARGET_RADIUS, &args![request]).f64();
    let radius_again = e.call(REQUEST_TARGET_RADIUS, &args![request]).f64();
    if radius_again * radius < horizontal {
        return None;
    }
    let height = e.mem.f32(frame + 0xe8 + 8);
    let height = e.call(FLOAT_ABS, &args![fpu_copy(height)]).f64();
    if e.call(REQUEST_GOAL_Z_DELTA, &args![request]).f64() < height {
        return None;
    }

    actor_mover_finish_moving(e, this, true);
    let mut result = true;
    if e.call(REQUEST_FACE_TARGET_AT_GOAL, &args![request]).bool() {
        let angle = e.call(REQUEST_GOAL_ANGLE, &args![request]).f32();
        result = actor_mover_request_rotate_actor_ov2(e, this, angle, true);
    }
    Some(result)
}

// Translated from 009db930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the retry count in a request (`PathingRequest::iRetryCount`,
/// Xbox PDB, +0xA8).
pub fn fn_009db930(e: &mut Engine, request: u32, retry_count: u32) {
    e.mem.set_u32(request + 0xa8, retry_count);
}

// Translated from 009db950 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009db950 - C++ exception unwinding (FS:[0] frame) not translated
/// `ActorMover::ClearDataForNewPathingRequest` (Xbox PDB): clears
/// `bStopMovementNextUpdate`, calls the virtual `ClearForcedMoveMode`
/// and drops the request. When a path was being awaited it gets a new
/// message queue and, if `clear_waiting` is set, forgets the wait. The path
/// is then always dropped (`009dba05`: `0066b0d0(+0x20, 0)`) and the
/// pathing-failed state reset (`009dba0f`: `SetPathingFailed(0)`): the
/// not-waiting jump at `009db9a2` and the flag test at `009db9f4` both land
/// on `009db9fd`, ahead of those two calls.
pub fn actor_mover_clear_data_for_new_pathing_request(
    e: &mut Engine,
    this: Ptr<ActorMover>,
    clear_waiting: bool,
) {
    let base = this.addr();
    e.set(this, ActorMover::bStopMovementNextUpdate, false);
    e.vcall(base, 0x10, &[]);
    set_ni_pointer(e, base + 0x1c, 0);
    if e.get(this, ActorMover::bWaitingForPath) {
        reset_message_queue(e, this);
        if clear_waiting {
            e.set(this, ActorMover::bWaitingForPath, false);
        }
    }
    set_ni_pointer(e, base + 0x20, 0);
    e.call(SET_PATHING_FAILED, &args![this, false]);
    e.call(DEBUG_STUB, &args![this]);
}

// Translated from 009dba30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards the request to `009db090`.
pub fn fn_009dba30(e: &mut Engine, this: Ptr<ActorMover>, request: Ptr) -> bool {
    fn_009db090(e, this, request)
}

// Translated from 009dba50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009dba50 - C++ exception unwinding (FS:[0] frame) not translated
/// `ActorMover::SetGoal` (Xbox PDB): builds a `PathingRequest` on the
/// stack for the goal position, cell and world space with the given target
/// radius and avoid node array (`009dbc90`), and starts pathing with it.
pub fn actor_mover_set_goal(
    e: &mut Engine,
    this: Ptr<ActorMover>,
    position: Ptr,
    cell: Ptr,
    world_space: Ptr,
    target_radius: f32,
    avoid_nodes: Ptr,
) -> bool {
    // The exe's frame (`EBP` frame, exception record, `SUB ESP,0xb8`, the cookie): 0xc8 bytes,
    // the request at +0xc (`EBP-0xbc`).
    e.with_stack(0xc8, |e, frame| {
        let request = frame.addr() + 0xc;
        e.call(REQUEST_CONSTRUCT, &args![request]);
        let actor = e.get(this, ActorMover::pActor);
        fn_009dbc90(
            e,
            actor,
            Ptr::new(request),
            position,
            cell,
            world_space,
            fpu_copy(target_radius),
            avoid_nodes,
        );
        let result = fn_009dba30(e, this, Ptr::new(request));
        e.call(REQUEST_DESTRUCT, &args![request]);
        result
    })
}

// Translated from 009dbb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets a goal at a reference: its position (actor-style virtual slot
/// 0x1F4), parent cell and world space, with the given radius and avoid
/// node array (`SetGoal`).
pub fn fn_009dbb10(
    e: &mut Engine,
    this: Ptr<ActorMover>,
    target: Ptr,
    target_radius: f32,
    avoid_nodes: Ptr,
) -> bool {
    let world_space = e.call(GET_WORLD_SPACE, &args![target]).u32();
    let cell = e.call(GET_PARENT_CELL, &args![target]).u32();
    let position = e.vcall(target.addr(), 0x1f4, &[]).u32();
    actor_mover_set_goal(
        e,
        this,
        Ptr::new(position),
        Ptr::new(cell),
        Ptr::new(world_space),
        target_radius,
        avoid_nodes,
    )
}

// Translated from 009dbb60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009dbb60 - C++ exception unwinding (FS:[0] frame) not translated
/// Like `SetGoal` with a second float: builds the request (`009dbc90` with
/// `final_radius`), gives it `request_float` through `008e7c20` and starts
/// pathing.
#[allow(clippy::too_many_arguments)] // The exe ABI has seven stack arguments.
pub fn fn_009dbb60(
    e: &mut Engine,
    this: Ptr<ActorMover>,
    position: Ptr,
    cell: Ptr,
    world_space: Ptr,
    request_float: f32,
    target_radius: f32,
    avoid_nodes: Ptr,
) -> bool {
    // The exe's frame (`EBP` frame, exception record, `SUB ESP,0xb8`, the cookie): 0xc8 bytes,
    // the request at +0xc (`EBP-0xbc`).
    e.with_stack(0xc8, |e, frame| {
        let request = frame.addr() + 0xc;
        e.call(REQUEST_CONSTRUCT, &args![request]);
        let actor = e.get(this, ActorMover::pActor);
        fn_009dbc90(
            e,
            actor,
            Ptr::new(request),
            position,
            cell,
            world_space,
            fpu_copy(target_radius),
            avoid_nodes,
        );
        e.call(REQUEST_SET_FLOAT, &args![request, fpu_copy(request_float)]);
        let result = fn_009dba30(e, this, Ptr::new(request));
        e.call(REQUEST_DESTRUCT, &args![request]);
        result
    })
}

// Translated from 009dbc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets a goal at a reference with the float at +8 of `00430830`'s result
/// (the reference's `+0x24` block) as the request float.
pub fn fn_009dbc30(
    e: &mut Engine,
    this: Ptr<ActorMover>,
    target: Ptr,
    target_radius: f32,
    avoid_nodes: Ptr,
) -> bool {
    let block = e.call(GET_ADDRESS_24, &args![target]).u32();
    let request_float = e.mem.f32(block + 8);
    let world_space = e.call(GET_WORLD_SPACE, &args![target]).u32();
    let cell = e.call(GET_PARENT_CELL, &args![target]).u32();
    let position = e.vcall(target.addr(), 0x1f4, &[]).u32();
    fn_009dbb60(
        e,
        this,
        Ptr::new(position),
        Ptr::new(cell),
        Ptr::new(world_space),
        fpu_copy(request_float),
        fpu_copy(target_radius),
        avoid_nodes,
    )
}

// Translated from 009dbc90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009dbc90 - C++ exception unwinding (FS:[0] frame) not translated
/// Fills a `PathingRequest` for an actor (cdecl): destination from the
/// position/cell/world space, the target radius (the default radius when
/// `target_radius` is not above zero), the avoid node array, and the
/// actor's door, swim and fly abilities.
#[allow(clippy::too_many_arguments)] // The exe ABI has seven stack arguments.
pub fn fn_009dbc90(
    e: &mut Engine,
    actor: Ptr,
    request: Ptr,
    position: Ptr,
    cell: Ptr,
    world_space: Ptr,
    target_radius: f32,
    avoid_nodes: Ptr,
) {
    // The exe's frame (`EBP` frame, exception record, `SUB ESP,0x2c`, the cookie): 0x3c bytes,
    // the location at +8 (`EBP-0x34`).
    e.with_stack(0x3c, |e, frame| {
        let location = frame.addr() + 8;
        e.call(
            LOCATION_FROM_POSITION,
            &args![location, position, cell, world_space],
        );
        e.call(REQUEST_INIT_FROM_ACTOR, &args![request, actor]);
        e.call(REQUEST_SET_DESTINATION, &args![request, location]);
        let zero: f64 = e.global(ZERO_DOUBLE);
        if target_radius as f64 <= zero {
            let default_radius = e.call(REQUEST_DEFAULT_RADIUS, &[]).f32();
            e.call(REQUEST_SET_TARGET_RADIUS, &args![request, default_radius]);
        } else {
            e.call(
                REQUEST_SET_TARGET_RADIUS,
                &args![request, fpu_copy(target_radius)],
            );
        }
        e.call(REQUEST_SET_AVOID_NODES, &args![request, avoid_nodes]);
        if e.vcall(actor.addr(), 0x21c, &[]).bool() {
            let base = e.call(ACTOR_BASE, &args![actor]).u32();
            let target = base + 0x30;
            let cant_open_doors = e.vcall(target, 0x28, &[]).bool();
            e.call(
                REQUEST_SET_CANT_OPEN_DOORS,
                &args![request, cant_open_doors],
            );
        }
        let base = e.call(ACTOR_BASE, &args![actor]).u32();
        let can_swim = e.call(BASE_CAN_SWIM, &args![base]).bool();
        e.call(REQUEST_SET_CAN_SWIM, &args![request, can_swim]);
        let base = e.call(ACTOR_BASE, &args![actor]).u32();
        let can_fly = e.call(BASE_CAN_FLY, &args![base]).bool();
        e.call(REQUEST_SET_CAN_FLY, &args![request, can_fly]);
        e.call(LOCATION_DESTRUCT, &args![location]);
    });
}

// Translated from 009dbdc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009dbdc0 - C++ exception unwinding (FS:[0] frame) not translated
/// Takes over a freshly built path: checks the request against the path
/// (`006d53b0`, or `006d5420` from node 0 when the path's second array is
/// empty), fails the mover when the check says so, then creates the path
/// handler (a detailed one, built over the previous handler, when the
/// actor's process type is 0 and the path has nodes in its second array,
/// or when it is not 0 and the array is empty; otherwise a virtual one),
/// starts it with `(0.0, delta_time)` and handles its result code.
pub fn fn_009dbdc0(e: &mut Engine, this: Ptr<ActorMover>, delta_time: f32) {
    let base = this.addr();
    e.call(DEBUG_STUB, &args![this]);
    let actor = e.get(this, ActorMover::pActor);
    let path = current_path(e, this);
    let detailed;
    if e.call(PATH_COUNT_20, &args![path]).u32() != 0 {
        if e.call(GET_PROCESS_TYPE, &args![actor]).u32() != 0 {
            detailed = false;
            let held = current_request(e, this);
            e.call(REQUEST_SET_ACTOR, &args![held, actor]);
            let path = current_path(e, this);
            let request = current_request(e, this);
            let verdict = e.call(REQUEST_CHECK_PATH, &args![request, path]).i32();
            if verdict == -1 || verdict == 0 {
                e.set(this, ActorMover::eState, STATE_FAILED);
                e.call(SET_PATHING_FAILED, &args![this, true]);
                return;
            }
        } else {
            detailed = true;
        }
    } else if e.call(GET_PROCESS_TYPE, &args![actor]).u32() == 0 {
        detailed = true;
        let held = current_request(e, this);
        e.call(REQUEST_SET_ACTOR, &args![held, actor]);
        let path = current_path(e, this);
        let request = current_request(e, this);
        let verdict = e
            .call(REQUEST_CHECK_PATH_FROM, &args![request, path, 0u32])
            .i32();
        if verdict == -1 || verdict == 0 {
            e.call(ACTOR_STOP_MOVEMENT, &args![actor, true]);
            e.set(this, ActorMover::eState, STATE_FAILED);
            e.call(SET_PATHING_FAILED, &args![this, true]);
            return;
        }
    } else {
        detailed = false;
    }

    if detailed {
        let path = current_path(e, this);
        let first = e.call(PATH_FIRST_LOADED, &args![path]).u32();
        e.set(this, ActorMover::iPreviousNode, first as u16);
        let path = current_path(e, this);
        let second = e.call(DWORD_AT_1C, &args![path]).u32();
        e.set(this, ActorMover::iNextNode, second as u16);
        let previous = e.get(this, ActorMover::pCurrentPathHandler).addr();
        let block = e.call(OPERATOR_NEW, &args![0xe4u32]).u32();
        let handler = if block != 0 {
            let request = current_request(e, this);
            e.call(
                DETAILED_HANDLER_CONSTRUCT,
                &args![block, actor, this, base + 0x20, request],
            )
            .u32()
        } else {
            0
        };
        e.set(this, ActorMover::pCurrentPathHandler, Ptr::new(handler));
        e.vcall(handler, 0x58, &args![previous]);
        delete_handler(e, previous);
    } else {
        e.set(this, ActorMover::iPreviousNode, 0);
        let path = current_path(e, this);
        let count = e.call(PATH_NODE_COUNT, &args![path]).u32();
        e.set(this, ActorMover::iNextNode, count.wrapping_sub(1) as u16);
        let previous = e.get(this, ActorMover::pCurrentPathHandler).addr();
        delete_handler(e, previous);
        let block = e.call(OPERATOR_NEW, &args![0x20u32]).u32();
        let handler = if block != 0 {
            let request = current_request(e, this);
            e.call(
                VIRTUAL_HANDLER_CONSTRUCT,
                &args![block, actor, this, base + 0x20, request],
            )
            .u32()
        } else {
            0
        };
        e.set(this, ActorMover::pCurrentPathHandler, Ptr::new(handler));
    }
    let handler = e.get(this, ActorMover::pCurrentPathHandler).addr();
    let result = e
        .vcall(handler, 0x04, &args![0.0f32, fpu_copy(delta_time)])
        .u32();
    fn_009dc5e0(e, this, result);
}

// Translated from 009dc120 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009dc120 - C++ exception unwinding (FS:[0] frame) not translated
/// `ActorMover::Update` (Xbox PDB, vtable slot 5): carries out a requested
/// stop, then -- unless the actor's process state forbids moving -- runs the
/// path handler (`fn_009dc590`), collects the path built by the path manager
/// (`spMessageQueue`) or lets the handler update without a path; finally
/// computes the move delta (`fn_009ddc00`).
pub fn actor_mover_update(e: &mut Engine, this: Ptr<ActorMover>, delta_time: f32) {
    if e.get(this, ActorMover::bStopMovementNextUpdate) {
        e.set(this, ActorMover::bStopCurrentMoveAnim, true);
        actor_mover_finish_moving(e, this, true);
        e.set(this, ActorMover::bStopMovementNextUpdate, false);
    }
    let actor = e.get(this, ActorMover::pActor).addr();
    if e.vcall(actor, 0x22c, &args![0u32]).bool() {
        let handler = e.get(this, ActorMover::pCurrentPathHandler).addr();
        if handler != 0 {
            e.vcall(handler, 0x70, &[]);
        }
    } else {
        update_movement(e, this, actor, delta_time);
    }
    fn_009ddc00(e, this);
    e.call(DEBUG_STUB, &args![this]);
}

/// The middle of `Update`: everything between the stop request and the move
/// delta. A `return` stands for the code's jump to the move delta.
fn update_movement(e: &mut Engine, this: Ptr<ActorMover>, actor: u32, delta_time: f32) {
    let base = this.addr();
    let process = e.call(GET_PROCESS, &args![actor]).u32();
    if process == 0 {
        return;
    }
    if e.vcall(process, 0x610, &[]).u32() == 4 {
        return;
    }
    let level = e.call(PROCESS_LEVEL, &args![process]).u32();
    if level != e.call(GET_DESIRED_PROCESS_LEVEL, &args![actor]).u32() {
        return;
    }
    if e.vcall(process, 0x4bc, &[]).u32() != 0 {
        return;
    }
    if e.vcall(actor, 0x230, &[]).bool() {
        return;
    }
    if e.vcall(process, 0x40c, &[]).u32() != 0 {
        return;
    }
    if e.vcall(process, 0x6fc, &[]).bool() {
        return;
    }
    if e.get(this, ActorMover::bActorChangedProcessLevel) {
        e.set(this, ActorMover::bActorChangedProcessLevel, false);
        fn_009dd2d0(e, this);
    }
    if !e.get(this, ActorMover::bPathingFailed) {
        if !e.get(this, ActorMover::bWaitingForPath) {
            if current_path(e, this) != 0 {
                let path = current_path(e, this);
                if fn_009dc530(e, path) {
                    e.set(this, ActorMover::eState, STATE_MOVING);
                    fn_009dc590(e, this, delta_time);
                    return;
                }
            }
        } else {
            // The exe's frame (`EBP` frame, exception record, `SUB ESP,0x20`, `ESI`, the cookie):
            // 0x34 bytes, the path message at +0x1c (`EBP-0x18`).
            let received = e.with_stack(0x34, |e, frame| {
                let message = frame.addr() + 0x1c;
                e.call(MESSAGE_CONSTRUCT, &args![message]);
                let queue = ni_pointer_value(e, base + 0x30);
                let received = e.vcall(queue, 0x10, &args![message]).bool();
                if received {
                    match e.mem.u32(message) {
                        0 => {
                            e.set(this, ActorMover::eState, STATE_MOVING);
                            e.set(this, ActorMover::bWaitingForPath, false);
                            ni_pointer_operator_assign(
                                e,
                                Ptr::new(base + 0x20),
                                Ptr::new(message + 4),
                            );
                            fn_009dbdc0(e, this, delta_time);
                        }
                        1 => {
                            e.set(this, ActorMover::bWaitingForPath, false);
                            ni_pointer_operator_assign(
                                e,
                                Ptr::new(base + 0x20),
                                Ptr::new(message + 4),
                            );
                            path_message_error(e, this, delta_time);
                        }
                        _ => {}
                    }
                }
                e.call(MESSAGE_DESTRUCT, &args![message]);
                received
            });
            if received {
                return;
            }
        }
    }
    let path_valid = current_path(e, this) != 0 && {
        let path = current_path(e, this);
        fn_009dc530(e, path)
    };
    if path_valid && !e.get(this, ActorMover::bPathingFailed) {
        return;
    }
    let handler = e.get(this, ActorMover::pCurrentPathHandler).addr();
    if handler != 0 {
        let result = e.vcall(handler, 0x10, &args![fpu_copy(delta_time)]).u32();
        if result == 5 {
            e.set(this, ActorMover::eState, STATE_FINISHED);
        }
    }
}

/// `Update`'s handling of an error message from the path manager: moves on
/// with the (incomplete) path when it is valid and incomplete paths are
/// allowed, otherwise fails the mover, remembers where the path ended and
/// clears the current data.
fn path_message_error(e: &mut Engine, this: Ptr<ActorMover>, delta_time: f32) {
    let base = this.addr();
    if current_path(e, this) != 0 {
        let path = current_path(e, this);
        if fn_009dc530(e, path) {
            let request = current_request(e, this);
            if e.call(REQUEST_ALLOW_INCOMPLETE, &args![request]).bool() {
                e.set(this, ActorMover::eState, STATE_MOVING);
                fn_009dbdc0(e, this, delta_time);
                return;
            }
        }
    }
    e.set(this, ActorMover::eState, STATE_FAILED);
    e.call(SET_PATHING_FAILED, &args![this, true]);
    if current_path(e, this) != 0 {
        let path = current_path(e, this);
        let index = e.call(DWORD_AT_1C, &args![path]).u32();
        if index != 0xffff_ffff {
            let path = current_path(e, this);
            if e.call(PATH_NODE_COUNT, &args![path]).u32() != 0 {
                let path = current_path(e, this);
                let node = e.call(PATH_NODE_AT, &args![path, index]).u32();
                if node != 0 {
                    let location = e.call(NODE_LOCATION, &args![node]).u32();
                    e.call(LOCATION_ASSIGN, &args![base + 0x44, location]);
                    e.call(LOCATION_CLEAR_TRIANGLE, &args![base + 0x44]);
                }
            }
        }
    }
    actor_mover_clear_current_data(e, this);
}

// Translated from 009dc530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tells whether a path (`PathingSolution`) is usable: its first array must
/// not be empty and, when both loaded indexes are set, its second array
/// must not be empty either. `006b05d0` is also called, its result unused.
pub fn fn_009dc530(e: &mut Engine, path: u32) -> bool {
    if e.call(ARRAY_IS_EMPTY, &args![path + 8]).bool() {
        return false;
    }
    if e.mem.i32(path + 0x18) != -1
        && e.mem.i32(path + 0x1c) != -1
        && e.call(ARRAY_IS_EMPTY, &args![path + 0x20]).bool()
    {
        return false;
    }
    e.call(PATH_FLAG_40, &args![path]);
    true
}

// Translated from 009dc590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs the path handler's update (slot 0x0C) unless the actor is in
/// dialog, and handles its result code.
pub fn fn_009dc590(e: &mut Engine, this: Ptr<ActorMover>, delta_time: f32) {
    if e.get(this, ActorMover::bInDialog) {
        return;
    }
    let handler = e.get(this, ActorMover::pCurrentPathHandler).addr();
    let result = e.vcall(handler, 0x0c, &args![fpu_copy(delta_time)]).u32();
    fn_009dc5e0(e, this, result);
}

// Translated from 009dc5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Handles a path handler's result code. 1 to 4 fail the mover; 5 and 6
/// advance along the path (`fn_009dc980`) and fail it when the path is
/// exhausted or too far from the actor.
pub fn fn_009dc5e0(e: &mut Engine, this: Ptr<ActorMover>, result: u32) {
    match result {
        1..=4 => {
            e.set(this, ActorMover::eState, STATE_FAILED);
            actor_mover_do_stop_moving(e, this);
            e.call(SET_PATHING_FAILED, &args![this, true]);
        }
        5 | 6 => {
            let status = fn_009dc980(e, this);
            let path = current_path(e, this);
            if !e.call(PATH_FLAG_40, &args![path]).bool() {
                if result == 6 && e.get(this, ActorMover::eState) != STATE_FAILED {
                    e.set(this, ActorMover::eState, STATE_MOVING);
                }
                actor_mover_do_stop_moving(e, this);
                return;
            }
            if status == 1 {
                return;
            }
            let path = current_path(e, this);
            let failed = if e.call(PATH_COUNT_20, &args![path]).u32() == 0 {
                true
            } else {
                path_end_is_too_far(e, this)
            };
            if failed {
                e.set(this, ActorMover::eState, STATE_FAILED);
                e.call(SET_PATHING_FAILED, &args![this, true]);
                actor_mover_clear_current_data(e, this);
                actor_mover_do_stop_moving(e, this);
            }
        }
        _ => {}
    }
}

/// Whether the squared horizontal distance from the actor to the path's
/// last loaded node location exceeds the request's squared target radius.
fn path_end_is_too_far(e: &mut Engine, this: Ptr<ActorMover>) -> bool {
    let actor = e.get(this, ActorMover::pActor).addr();
    let position = e.vcall(actor, 0x1f4, &[]).u32();
    // In `009dc5e0`'s frame (`SUB ESP,0x34` under `EBP`): the node position at +0x18
    // (`EBP-0x1c`) and the difference at +0x24 (`EBP-0x10`).
    let horizontal = e.with_stack(0x34, |e, frame| {
        let scratch = frame.addr() + 0x18;
        let path = current_path(e, this);
        let index = e.call(DWORD_AT_1C, &args![path]).u32();
        let path = current_path(e, this);
        let node = e.call(PATH_NODE_AT, &args![path, index]).u32();
        let location = e.call(NODE_LOCATION, &args![node]).u32();
        let node_position = e.call(GET_POSITION, &args![location, scratch]).u32();
        e.call(
            POINT_SUBTRACT,
            &args![node_position, scratch + 12, position],
        );
        e.call(POINT_LENGTH_SQUARED_XY, &args![scratch + 12]).f64()
    });
    let request = current_request(e, this);
    let radius = e.call(REQUEST_TARGET_RADIUS, &args![request]).f64();
    let request = current_request(e, this);
    let radius_again = e.call(REQUEST_TARGET_RADIUS, &args![request]).f64();
    radius_again * radius < horizontal
}

// Translated from 009dc780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Keeps `bActorChangedProcessLevel` in step with whether the path handler
/// is a detailed one: it is cleared when the flag was set and the handler
/// is not detailed, and set when the handler is detailed.
pub fn fn_009dc780(e: &mut Engine, this: Ptr<ActorMover>) {
    let handler = e.get(this, ActorMover::pCurrentPathHandler).addr();
    if handler == 0 {
        return;
    }
    if e.get(this, ActorMover::bActorChangedProcessLevel) && !e.vcall(handler, 0x34, &[]).bool() {
        e.set(this, ActorMover::bActorChangedProcessLevel, false);
        return;
    }
    if e.vcall(handler, 0x34, &[]).bool() {
        e.set(this, ActorMover::bActorChangedProcessLevel, true);
    }
}

// Translated from 009dc7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reacts to the actor's process level: nothing at levels 4 and 9; at any
/// other non-zero level the actor leaves its furniture; at level 0 it keeps
/// `bActorChangedProcessLevel` in step with the handler (clearing it once
/// the path has progressed past its first node) or, when there is no
/// unfinished path, stops the actor's movement.
pub fn fn_009dc7f0(e: &mut Engine, this: Ptr<ActorMover>) {
    let actor = e.get(this, ActorMover::pActor).addr();
    let process = e.call(GET_PROCESS, &args![actor]).u32();
    if e.vcall(process, 0x4bc, &[]).u32() == 4 {
        return;
    }
    let process = e.call(GET_PROCESS, &args![actor]).u32();
    if e.vcall(process, 0x4bc, &[]).u32() == 9 {
        return;
    }
    let process = e.call(GET_PROCESS, &args![actor]).u32();
    if e.vcall(process, 0x4bc, &[]).u32() != 0 {
        e.call(ACTOR_LEAVE_FURNITURE, &args![actor]);
        return;
    }
    let handler = e.get(this, ActorMover::pCurrentPathHandler).addr();
    if handler == 0 || actor_mover_is_pathing_complete(e, this) {
        let flags_object = e.global::<u32>(GLOBAL_FLAGS_OBJECT);
        if !e.call(GLOBAL_FLAGS_TEST, &args![flags_object]).bool() {
            e.call(ACTOR_STOP_MOVEMENT, &args![actor, false]);
        }
        return;
    }
    if e.get(this, ActorMover::bActorChangedProcessLevel) {
        let handler = e.get(this, ActorMover::pCurrentPathHandler).addr();
        if e.vcall(handler, 0x34, &[]).bool() {
            if ni_pointer_value(e, this.addr() + 0x20) == 0 {
                return;
            }
            let previous = e.get(this, ActorMover::iPreviousNode);
            let next = e.get(this, ActorMover::iNextNode);
            if previous == next {
                let previous = e.get(this, ActorMover::iPreviousNode) as u32;
                let path = current_path(e, this);
                let count = e.call(PATH_NODE_COUNT, &args![path]).u32();
                if previous < count.wrapping_sub(1) {
                    return;
                }
            }
            e.set(this, ActorMover::bActorChangedProcessLevel, false);
            return;
        }
    }
    let handler = e.get(this, ActorMover::pCurrentPathHandler).addr();
    if !e.vcall(handler, 0x34, &[]).bool() {
        e.set(this, ActorMover::bActorChangedProcessLevel, true);
    }
}

// Translated from 009dc950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tells the path handler (slot 0x5C) that a reference was deleted.
pub fn fn_009dc950(e: &mut Engine, this: Ptr<ActorMover>, reference: Ptr) {
    let handler = e.get(this, ActorMover::pCurrentPathHandler).addr();
    if handler != 0 {
        e.vcall(handler, 0x5c, &args![reference]);
    }
}

// Translated from 009dc980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Advances to the next node of the path. Returns 2 when the mover failed
/// (no next node, or the node cannot be used by the actor and the actor's
/// process level is 0), 1 when the path's second array was reset, 0
/// otherwise.
pub fn fn_009dc980(e: &mut Engine, this: Ptr<ActorMover>) -> u32 {
    let next = e.get(this, ActorMover::iNextNode);
    if next == 0 {
        e.set(this, ActorMover::eState, STATE_FAILED);
        e.call(SET_PATHING_FAILED, &args![this, true]);
        actor_mover_stop_moving(e, this);
        return 2;
    }
    e.set(this, ActorMover::iPreviousNode, next);
    let path = current_path(e, this);
    if e.call(PATH_COUNT_20, &args![path]).u32() != 0 {
        let path = current_path(e, this);
        let element = e.call(PATH_SECOND_ARRAY_ELEMENT, &args![path]).u32();
        if e.call(NODE_FLAGS_ARE_3, &args![element]).bool() {
            let actor = e.get(this, ActorMover::pActor);
            if !e.call(NODE_TEST_ACTOR, &args![element, actor]).bool() {
                let process = e.call(GET_PROCESS, &args![actor]).u32();
                if e.vcall(process, 0x610, &[]).u32() == 0 {
                    e.set(this, ActorMover::eState, STATE_FAILED);
                    e.call(SET_PATHING_FAILED, &args![this, true]);
                    actor_mover_do_stop_moving(e, this);
                    return 2;
                }
            }
            let path = current_path(e, this);
            e.call(PATH_RESET_LOADED, &args![path]);
            return 1;
        }
    }
    let previous = e.get(this, ActorMover::iPreviousNode) as u32;
    let handler = e.get(this, ActorMover::pCurrentPathHandler).addr();
    let handler_path = ni_pointer_value(e, handler + 0xc);
    let count = e.call(PATH_NODE_COUNT, &args![handler_path]).u32();
    if previous >= count.wrapping_sub(1) {
        actor_mover_do_stop_moving(e, this);
    }
    0
}

// Translated from 009dcab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The path handler's slot 0x30 (current node index), 0 without a handler.
pub fn fn_009dcab0(e: &mut Engine, this: Ptr<ActorMover>) -> u32 {
    let handler = e.get(this, ActorMover::pCurrentPathHandler).addr();
    if handler != 0 {
        e.vcall(handler, 0x30, &[]).u32()
    } else {
        0
    }
}

// Translated from 009dcae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorMover::IsActorInLoadedArea` (Xbox PDB): the actor has a parent cell
/// that passes `00450ff0` and the player is not sleeping or resting.
pub fn actor_mover_is_actor_in_loaded_area(e: &mut Engine, this: Ptr<ActorMover>) -> bool {
    let actor = e.get(this, ActorMover::pActor);
    if e.call(GET_PARENT_CELL, &args![actor]).u32() == 0 {
        return false;
    }
    let cell = e.call(GET_PARENT_CELL, &args![actor]).u32();
    if !e.call(CELL_IS_SIX, &args![cell]).bool() {
        return false;
    }
    let player = e.global::<u32>(PLAYER_CHARACTER);
    !e.call(PLAYER_IS_SLEEPING, &args![player]).bool()
}

// Translated from 009dcb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The path handler's slot 0x3C when there is a handler and a usable path,
/// otherwise true.
pub fn fn_009dcb30(e: &mut Engine, this: Ptr<ActorMover>) -> bool {
    handler_query_with_default(e, this, 0x3c, true)
}

// Translated from 009dcb90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The path handler's slot 0x40 when there is a handler and a usable path,
/// otherwise false.
pub fn fn_009dcb90(e: &mut Engine, this: Ptr<ActorMover>) -> bool {
    handler_query_with_default(e, this, 0x40, false)
}

/// Shared body of `009dcb30` and `009dcb90`.
fn handler_query_with_default(
    e: &mut Engine,
    this: Ptr<ActorMover>,
    slot: u32,
    default: bool,
) -> bool {
    if e.get(this, ActorMover::pCurrentPathHandler).addr() != 0 && current_path(e, this) != 0 {
        let path = current_path(e, this);
        if fn_009dc530(e, path) {
            let handler = e.get(this, ActorMover::pCurrentPathHandler).addr();
            return e.vcall(handler, slot, &[]).bool();
        }
    }
    default
}

// Translated from 009dcbf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorMover::GetCurrentPercent` (Xbox PDB): the path handler's slot 0x1C,
/// 0.0 without a handler.
pub fn actor_mover_get_current_percent(e: &mut Engine, this: Ptr<ActorMover>) -> f32 {
    handler_float(e, this, 0x1c)
}

// Translated from 009dcc20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorMover::GetDistanceTraveled` (Xbox PDB): the path handler's slot
/// 0x18, 0.0 without a handler.
pub fn actor_mover_get_distance_traveled(e: &mut Engine, this: Ptr<ActorMover>) -> f32 {
    handler_float(e, this, 0x18)
}

// Translated from 009dcc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorMover::GetDistanceRemaining` (Xbox PDB): the path handler's slot
/// 0x28, 0.0 without a handler.
pub fn actor_mover_get_distance_remaining(e: &mut Engine, this: Ptr<ActorMover>) -> f32 {
    handler_float(e, this, 0x28)
}

/// A float the path handler returns in ST0 from slot `slot`, 0.0 (`FLDZ`)
/// without a handler.
fn handler_float(e: &mut Engine, this: Ptr<ActorMover>, slot: u32) -> f32 {
    let handler = e.get(this, ActorMover::pCurrentPathHandler).addr();
    if handler != 0 {
        e.vcall(handler, slot, &[]).f32()
    } else {
        0.0
    }
}

// Translated from 009dcc80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the approximate location of the actor into `out`: the path
/// handler's slot 0x2C when there is a handler, otherwise a copy of the
/// actor's position (slot 0x1F4). Returns `out`.
pub fn fn_009dcc80(e: &mut Engine, this: Ptr<ActorMover>, out: Ptr, value: f32) -> Ptr {
    let handler = e.get(this, ActorMover::pCurrentPathHandler).addr();
    if handler != 0 {
        e.vcall(handler, 0x2c, &args![out, fpu_copy(value)]);
        return out;
    }
    let actor = e.get(this, ActorMover::pActor).addr();
    let position = e.vcall(actor, 0x1f4, &[]).u32();
    copy_point(e, position, out.addr());
    out
}

// Translated from 009dccf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The path handler's slot 0x44 when there is a usable path, otherwise
/// false.
pub fn fn_009dccf0(e: &mut Engine, this: Ptr<ActorMover>) -> bool {
    if current_path(e, this) != 0 {
        let path = current_path(e, this);
        if fn_009dc530(e, path) {
            let handler = e.get(this, ActorMover::pCurrentPathHandler).addr();
            return e.vcall(handler, 0x44, &[]).bool();
        }
    }
    false
}

// Translated from 009dcd40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorMover::IsPathingComplete` (Xbox PDB): false while a path is being
/// awaited; true once the mover is finished or failed; otherwise true when
/// there is no usable path and the mover is not rotating.
pub fn actor_mover_is_pathing_complete(e: &mut Engine, this: Ptr<ActorMover>) -> bool {
    if e.get(this, ActorMover::bWaitingForPath) {
        return false;
    }
    let state = e.get(this, ActorMover::eState);
    if state == STATE_FINISHED || state == STATE_FAILED {
        return true;
    }
    if current_path(e, this) != 0 {
        let path = current_path(e, this);
        if fn_009dc530(e, path) {
            return false;
        }
    }
    e.get(this, ActorMover::eState) != STATE_ROTATING
}

// Translated from 009dcdb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorMover::IsPathing` (Xbox PDB): false when the move mode is forced;
/// true while rotating or awaiting a path; otherwise true when there is a
/// usable path and pathing has not failed.
pub fn actor_mover_is_pathing(e: &mut Engine, this: Ptr<ActorMover>) -> bool {
    if e.get(this, ActorMover::bForceMoveMode) {
        return false;
    }
    if e.get(this, ActorMover::eState) == STATE_ROTATING {
        return true;
    }
    if e.get(this, ActorMover::bWaitingForPath) {
        return true;
    }
    if current_path(e, this) != 0 {
        let path = current_path(e, this);
        if fn_009dc530(e, path) {
            return !e.get(this, ActorMover::bPathingFailed);
        }
    }
    false
}

// Translated from 009dce20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorMover::RequestRotateActor` (Xbox PDB): rotates the actor to face
/// the point `(x, y, z)`: the Z angle of the point minus the actor's
/// position (`004b13c0`) is handed to the angle overload.
pub fn actor_mover_request_rotate_actor(
    e: &mut Engine,
    this: Ptr<ActorMover>,
    x: f32,
    y: f32,
    z: f32,
    flag: bool,
) -> bool {
    let actor = e.get(this, ActorMover::pActor).addr();
    let position = e.vcall(actor, 0x1f4, &[]).u32();
    // The exe's frame (`SUB ESP,0x14` under `EBP`) with the saved `EBP`, the return address and
    // the arguments: the point arrives by value in its argument slots (`EBP+8`, +0x1c) and
    // `00439ef0` takes that address as the receiver; the difference lands at +8 (`EBP-0xc`).
    let angle = e.with_stack(0x2c, |e, frame| {
        let point = frame.addr() + 0x1c;
        let difference = frame.addr() + 8;
        e.mem.set_f32(point, x);
        e.mem.set_f32(point + 4, y);
        e.mem.set_f32(point + 8, z);
        e.mem.set_u8(point + 0xc, u8::from(flag));
        e.call(POINT_SUBTRACT, &args![point, difference, position]);
        e.call(Z_ANGLE_FROM_VECTOR, &args![difference]).f32()
    });
    actor_mover_request_rotate_actor_ov2(e, this, angle, flag)
}

// Translated from 009dce80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009dce80 - C++ exception unwinding (FS:[0] frame) not translated
/// `ActorMover::RequestRotateActor` (Xbox PDB, the overload taking an angle
/// and a flag): drops any pending path, then either finishes at once (the
/// actor cannot move, or slot 0x214 says so) or, when the actor is outside
/// the loaded area, hands the angle to the actor's slot 0x2C4, or starts a
/// rotation through a detailed path handler (slot 0x08). Returns true when
/// the mover finished or started rotating.
pub fn actor_mover_request_rotate_actor_ov2(
    e: &mut Engine,
    this: Ptr<ActorMover>,
    angle: f32,
    flag: bool,
) -> bool {
    let mut result = false;
    actor_mover_clear_data_for_new_pathing_request(e, this, true);
    let actor = e.get(this, ActorMover::pActor);
    if e.vcall(actor.addr(), 0x1e4, &[]).u32() != 0 {
        let animation = e.vcall(actor.addr(), 0x1e4, &[]).u32();
        if e.call(ANIMATION_SHOULD_BE_MOVING, &args![animation]).bool() {
            e.set(this, ActorMover::bStopCurrentMoveAnim, true);
        }
    }
    let cannot_move = !e.call(ACTOR_CAN_MOVE, &args![actor]).bool()
        || e.vcall(actor.addr(), 0x214, &[]).u32() != 0;
    if cannot_move {
        e.set(this, ActorMover::eState, STATE_FINISHED);
        result = true;
    } else if !actor_mover_is_actor_in_loaded_area(e, this) {
        e.set(this, ActorMover::eState, STATE_FINISHED);
        e.vcall(actor.addr(), 0x2c4, &args![fpu_copy(angle)]);
        result = true;
    }
    if !result {
        let handler = e.get(this, ActorMover::pCurrentPathHandler).addr();
        if handler != 0 && !e.vcall(handler, 0x34, &[]).bool() {
            let handler = e.get(this, ActorMover::pCurrentPathHandler).addr();
            delete_handler(e, handler);
            e.set(this, ActorMover::pCurrentPathHandler, Ptr::NULL);
        }
        if e.get(this, ActorMover::pCurrentPathHandler).addr() == 0 {
            let block = e.call(OPERATOR_NEW, &args![0xe4u32]).u32();
            let handler = if block != 0 {
                e.call(
                    DETAILED_HANDLER_CONSTRUCT_SIMPLE,
                    &args![block, actor, this],
                )
                .u32()
            } else {
                0
            };
            e.set(this, ActorMover::pCurrentPathHandler, Ptr::new(handler));
        }
        let handler = e.get(this, ActorMover::pCurrentPathHandler).addr();
        if e.vcall(handler, 0x08, &args![fpu_copy(angle), flag]).bool() {
            e.set(this, ActorMover::eState, STATE_ROTATING);
            result = true;
        } else {
            e.set(this, ActorMover::eState, STATE_FINISHED);
            result = false;
        }
    }
    e.call(DEBUG_STUB, &args![this]);
    result
}

// Translated from 009dd0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorMover::StopMoving` (Xbox PDB): `DoStopMoving`.
pub fn actor_mover_stop_moving(e: &mut Engine, this: Ptr<ActorMover>) {
    actor_mover_do_stop_moving(e, this);
    e.call(DEBUG_STUB, &args![this]);
}

// Translated from 009dd0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorMover::ForceStopMoving` (Xbox PDB): stops, and when a stop was
/// pending finishes at once, caches the last valid move mode, clears the
/// cached move delta and marks it valid for this frame.
pub fn actor_mover_force_stop_moving(e: &mut Engine, this: Ptr<ActorMover>) {
    actor_mover_do_stop_moving(e, this);
    if e.get(this, ActorMover::bStopMovementNextUpdate) {
        e.set(this, ActorMover::bStopMovementNextUpdate, false);
        actor_mover_finish_moving(e, this, true);
        let mode = fn_009dd130(e, this);
        e.set(this, ActorMover::iCachedLastValidMoveMode, mode);
        copy_point(
            e,
            UNSET_POINT,
            this.addr() + ActorMover::CachedMoveDelta.off,
        );
        e.set(this, ActorMover::bHaveValidDeltaThisFrame, true);
    }
    e.call(DEBUG_STUB, &args![this]);
}

// Translated from 009dd130 (decompiled, FalloutNV.exe 1.4.0.525)
/// The preferred move mode's high part (`uiPreferredMoveMode & 0xFF00`).
pub fn fn_009dd130(e: &mut Engine, this: Ptr<ActorMover>) -> u32 {
    e.get(this, ActorMover::uiPreferredMoveMode) & 0xff00
}

// Translated from 009dd150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorMover::DoStopMoving` (Xbox PDB): unless already finished, marks
/// the mover finished when the handler is not a detailed one and requests
/// the stop on the next update; a failed path requests it too; then resets
/// the pathing-failed state.
pub fn actor_mover_do_stop_moving(e: &mut Engine, this: Ptr<ActorMover>) {
    if e.get(this, ActorMover::eState) != STATE_FINISHED {
        let handler = e.get(this, ActorMover::pCurrentPathHandler).addr();
        if handler != 0 && !e.vcall(handler, 0x34, &[]).bool() {
            e.set(this, ActorMover::eState, STATE_FINISHED);
        }
        e.set(this, ActorMover::bStopMovementNextUpdate, true);
    }
    if e.get(this, ActorMover::bPathingFailed) {
        e.set(this, ActorMover::bStopMovementNextUpdate, true);
    }
    e.call(SET_PATHING_FAILED, &args![this, false]);
}

// Translated from 009dd1c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009dd1c0 - C++ exception unwinding (FS:[0] frame) not translated
/// `ActorMover::FinishMoving` (Xbox PDB): the state becomes finished unless
/// it is 7 or 8 (failed); the request and path are dropped; with
/// `clear_movement` the handler (slot 0x70) clears its movement; a path being
/// awaited is abandoned with a new message queue; the pathing-failed state
/// is reset.
pub fn actor_mover_finish_moving(e: &mut Engine, this: Ptr<ActorMover>, clear_movement: bool) {
    let base = this.addr();
    let state = e.get(this, ActorMover::eState) as i32;
    if !(7..=8).contains(&state) {
        e.set(this, ActorMover::eState, STATE_FINISHED);
    }
    set_ni_pointer(e, base + 0x1c, 0);
    set_ni_pointer(e, base + 0x20, 0);
    let handler = e.get(this, ActorMover::pCurrentPathHandler).addr();
    if handler != 0 && clear_movement {
        e.vcall(handler, 0x70, &[]);
    }
    if e.get(this, ActorMover::bWaitingForPath) {
        reset_message_queue(e, this);
        e.set(this, ActorMover::bWaitingForPath, false);
    }
    e.call(SET_PATHING_FAILED, &args![this, false]);
    e.call(DEBUG_STUB, &args![this]);
}

// Translated from 009dd2d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009dd2d0 - C++ exception unwinding (FS:[0] frame) not translated
/// Re-creates the path handler after the actor's process level changed.
/// Without a request and a path the handler is dropped and the mover
/// finishes. For an actor whose process type is not 0 the node reached is
/// worked out from the handler's percent travelled, the request is checked
/// against the path and a virtual handler is built and started at that
/// node. For process type 0 the next node is taken from the handler, the
/// request is re-checked from that node, and a detailed handler is built
/// and started at the fraction of the first segment already travelled.
pub fn fn_009dd2d0(e: &mut Engine, this: Ptr<ActorMover>) {
    let base = this.addr();
    let actor = e.get(this, ActorMover::pActor);
    let has_data = ni_pointer_value(e, base + 0x20) != 0 && ni_pointer_value(e, base + 0x1c) != 0;
    if !has_data {
        let old = e.get(this, ActorMover::pCurrentPathHandler).addr();
        delete_handler(e, old);
        e.set(this, ActorMover::pCurrentPathHandler, Ptr::NULL);
        e.set(this, ActorMover::eState, STATE_FINISHED);
        e.call(SET_PATHING_FAILED, &args![this, false]);
        return;
    }
    if e.call(GET_PROCESS_TYPE, &args![actor]).u32() != 0 {
        rebuild_virtual_handler(e, this);
    } else {
        rebuild_detailed_handler(e, this);
    }
}

/// `009dd2d0` for an actor whose process type is not 0.
fn rebuild_virtual_handler(e: &mut Engine, this: Ptr<ActorMover>) {
    let base = this.addr();
    let actor = e.get(this, ActorMover::pActor);
    let handler = e.get(this, ActorMover::pCurrentPathHandler).addr();
    if !e.vcall(handler, 0x34, &[]).bool() {
        return;
    }
    let next = e.get(this, ActorMover::iNextNode) as u32;
    let previous = e.get(this, ActorMover::iPreviousNode) as u32;
    let mut target = if next == previous {
        next + 1
    } else {
        let handler = e.get(this, ActorMover::pCurrentPathHandler).addr();
        let percent = e.vcall(handler, 0x1c, &[]).f32();
        let span = next.wrapping_sub(previous) as i32;
        let steps = ((span as f64) * (percent as f64)) as f32;
        let rounded = e.call(FLOAT_ROUND, &args![fpu_copy(steps)]).f64();
        let whole = e.call(FTOL, &args![rounded]).i32();
        previous.wrapping_add(whole as u32)
    };
    let path = current_path(e, this);
    let last = e.call(PATH_NODE_COUNT, &args![path]).u32().wrapping_sub(1);
    if target > last {
        let path = current_path(e, this);
        target = e.call(PATH_NODE_COUNT, &args![path]).u32().wrapping_sub(1);
    }
    let request = current_request(e, this);
    e.call(REQUEST_SET_ACTOR, &args![request, actor]);
    let path = current_path(e, this);
    let request = current_request(e, this);
    let verdict = e.call(REQUEST_CHECK_PATH, &args![request, path]).i32();
    if verdict >= -1 {
        if verdict <= 0 {
            e.set(this, ActorMover::eState, STATE_FAILED);
            e.call(SET_PATHING_FAILED, &args![this, true]);
            return;
        }
        if verdict == 1 {
            e.set(this, ActorMover::iPreviousNode, target as u16);
            let path = current_path(e, this);
            let count = e.call(PATH_NODE_COUNT, &args![path]).u32();
            e.set(this, ActorMover::iNextNode, count.wrapping_sub(1) as u16);
        }
    }
    let old = e.get(this, ActorMover::pCurrentPathHandler).addr();
    delete_handler(e, old);
    let block = e.call(OPERATOR_NEW, &args![0x20u32]).u32();
    let new = if block != 0 {
        let request = current_request(e, this);
        e.call(
            VIRTUAL_HANDLER_CONSTRUCT,
            &args![block, actor, this, base + 0x20, request],
        )
        .u32()
    } else {
        0
    };
    e.set(this, ActorMover::pCurrentPathHandler, Ptr::new(new));
    let start = e.get(this, ActorMover::iPreviousNode) as f32;
    e.vcall(new, 0x04, &args![start, 0.0f32]);
}

/// `009dd2d0` for an actor whose process type is 0.
fn rebuild_detailed_handler(e: &mut Engine, this: Ptr<ActorMover>) {
    let base = this.addr();
    let actor = e.get(this, ActorMover::pActor);
    let handler = e.get(this, ActorMover::pCurrentPathHandler).addr();
    let next_index = if e.vcall(handler, 0x34, &[]).bool() {
        e.get(this, ActorMover::iPreviousNode) as u32 + 1
    } else {
        let handler = e.get(this, ActorMover::pCurrentPathHandler).addr();
        if e.vcall(handler, 0x30, &[]).u32() != 0 {
            let handler = e.get(this, ActorMover::pCurrentPathHandler).addr();
            e.vcall(handler, 0x30, &[]).u32().wrapping_sub(1)
        } else {
            0
        }
    };
    let path = current_path(e, this);
    let last = e.call(PATH_NODE_COUNT, &args![path]).u32().wrapping_sub(1);
    if next_index >= last {
        // Past the last node: report where the path ends, then finish. In `009dd2d0`'s frame
        // (`EBP` frame, exception record, `SUB ESP,0xac`, the cookie: 0xbc bytes) the position
        // lands at +0x5c (`EBP-0x60`) when the test holds, else at +0x50 (`EBP-0x6c`).
        e.with_stack(0xbc, |e, frame| {
            let test = e.call(GLOBAL_TEST, &[]).bool();
            let scratch = frame.addr() + if test { 0x5c } else { 0x50 };
            let path = current_path(e, this);
            let node = fn_009ddb30(e, path);
            let location = e.call(NODE_LOCATION, &args![node]).u32();
            let position = e.call(GET_POSITION, &args![location, scratch]).u32();
            if test {
                let target = e.call(REPORT_TARGET_OBJECT, &[]).u32();
                e.call(REPORT_PATH_END, &args![target, actor, position]);
            } else {
                e.vcall(actor.addr(), 0x2a8, &args![position]);
            }
        });
        actor_mover_finish_moving(e, this, true);
        return;
    }
    let request = current_request(e, this);
    e.call(REQUEST_SET_ACTOR, &args![request, actor]);
    let path = current_path(e, this);
    let request = current_request(e, this);
    let verdict = e
        .call(REQUEST_CHECK_PATH_FROM, &args![request, path, next_index])
        .i32();
    if verdict == -1 || verdict == 0 {
        let path = current_path(e, this);
        let usable = e.call(PATH_FLAG_40, &args![path]).bool()
            && {
                let request = current_request(e, this);
                e.call(REQUEST_ALLOW_INCOMPLETE, &args![request]).bool()
            }
            && {
                let path = current_path(e, this);
                e.call(PATH_COUNT_20, &args![path]).u32() != 0
            };
        if usable {
            set_nodes_from_path(e, this);
        } else {
            e.call(ACTOR_STOP_MOVEMENT, &args![actor, true]);
            e.set(this, ActorMover::eState, STATE_FAILED_PROCESS_UPDATE);
            e.call(SET_PATHING_FAILED, &args![this, true]);
            return;
        }
    } else if verdict == 2 {
        set_nodes_from_path(e, this);
    }
    if e.get(this, ActorMover::iPreviousNode) == e.get(this, ActorMover::iNextNode) {
        actor_mover_finish_moving(e, this, true);
        return;
    }
    let mut fraction = 0.0f32;
    let handler = e.get(this, ActorMover::pCurrentPathHandler).addr();
    if !e.vcall(handler, 0x34, &[]).bool() {
        fraction = first_segment_fraction(e, this);
    }
    let old = e.get(this, ActorMover::pCurrentPathHandler).addr();
    delete_handler(e, old);
    let block = e.call(OPERATOR_NEW, &args![0xe4u32]).u32();
    let new = if block != 0 {
        let request = current_request(e, this);
        e.call(
            DETAILED_HANDLER_CONSTRUCT,
            &args![block, actor, this, base + 0x20, request],
        )
        .u32()
    } else {
        0
    };
    e.set(this, ActorMover::pCurrentPathHandler, Ptr::new(new));
    let result = e.vcall(new, 0x04, &args![fraction, 0.0f32]).u32();
    if result == 5 || result == 6 {
        fn_009dc5e0(e, this, result);
        actor_mover_finish_moving(e, this, true);
    }
}

/// Sets `iPreviousNode` and `iNextNode` from the path's first and last
/// loaded node indexes (low words).
fn set_nodes_from_path(e: &mut Engine, this: Ptr<ActorMover>) {
    let path = current_path(e, this);
    let first = e.call(PATH_FIRST_LOADED, &args![path]).u32();
    e.set(this, ActorMover::iPreviousNode, first as u16);
    let path = current_path(e, this);
    let last = e.call(DWORD_AT_1C, &args![path]).u32();
    e.set(this, ActorMover::iNextNode, last as u16);
}

/// The handler's distance travelled divided by the length of the segment
/// between the path's first and last loaded nodes (as a `float`).
/// In `009dd2d0`'s frame (0xbc bytes): the start at +0x80 (`EBP-0x3c`), the end at +0x90
/// (`EBP-0x2c`) and their difference at +0x44 (`EBP-0x78`).
fn first_segment_fraction(e: &mut Engine, this: Ptr<ActorMover>) -> f32 {
    let length = e.with_stack(0xbc, |e, frame| {
        let frame = frame.addr();
        let path = current_path(e, this);
        let first = e.call(PATH_FIRST_LOADED, &args![path]).u32();
        let path = current_path(e, this);
        let node = e.call(PATH_NODE_AT, &args![path, first]).u32();
        let location = e.call(NODE_LOCATION, &args![node]).u32();
        let start = e.call(GET_POSITION, &args![location, frame + 0x80]).u32();
        let path = current_path(e, this);
        let last = e.call(DWORD_AT_1C, &args![path]).u32();
        let path = current_path(e, this);
        let node = e.call(PATH_NODE_AT, &args![path, last]).u32();
        let location = e.call(NODE_LOCATION, &args![node]).u32();
        let end = e.call(GET_POSITION, &args![location, frame + 0x90]).u32();
        let difference = e
            .call(POINT_SUBTRACT, &args![end, frame + 0x44, start])
            .u32();
        e.call(POINT_LENGTH, &args![difference]).f32()
    });
    let handler = e.get(this, ActorMover::pCurrentPathHandler).addr();
    let travelled = e.vcall(handler, 0x18, &[]).f64();
    (travelled / length as f64) as f32
}

// Translated from 009ddb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The last node of a path: `009dfd50` on the array at +8.
pub fn fn_009ddb30(e: &mut Engine, path: u32) -> u32 {
    e.call(PATH_LAST_NODE, &args![path + 8]).u32()
}

// Translated from 009ddb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorMover::IsRotating` (Xbox PDB): the state is rotating.
pub fn actor_mover_is_rotating(e: &mut Engine, this: Ptr<ActorMover>) -> bool {
    e.get(this, ActorMover::eState) == STATE_ROTATING
}

// Translated from 009ddb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorMover::GetMoveMode` (Xbox PDB, vtable slot 8): the preferred mode's
/// high part when pathing failed, otherwise the cached last valid mode.
pub fn actor_mover_get_move_mode(e: &mut Engine, this: Ptr<ActorMover>) -> u32 {
    if e.get(this, ActorMover::bPathingFailed) {
        fn_009dd130(e, this)
    } else {
        e.get(this, ActorMover::iCachedLastValidMoveMode)
    }
}

// Translated from 009ddbb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorMover::GetMoveDelta` (Xbox PDB, vtable slot 9): when this frame's
/// delta is valid, copies the cached move delta to `out`, consumes the flag
/// and returns true.
pub fn actor_mover_get_move_delta(e: &mut Engine, this: Ptr<ActorMover>, out: Ptr) -> bool {
    if e.get(this, ActorMover::bHaveValidDeltaThisFrame) {
        e.set(this, ActorMover::bHaveValidDeltaThisFrame, false);
        copy_point(e, this.addr() + ActorMover::CachedMoveDelta.off, out.addr());
        true
    } else {
        false
    }
}

// Translated from 009ddc00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Computes this frame's move mode flags and move delta. With a forced move
/// mode the delta comes from the actor's animation (slot 0x1E4) or, without
/// one, a forward step of the actor's current speed times the global frame
/// time; otherwise from the path handler (slots 0x48 and 0x4C). A pending
/// stop of the move animation zeroes the delta. Bits 0x200 and 0x400 of the
/// mode are masked by actor tests, and bit 0x400 is only kept while the
/// animation allows it. The results go to `iCachedLastValidMoveMode`,
/// `CachedMoveDelta` and `bHaveValidDeltaThisFrame`.
pub fn fn_009ddc00(e: &mut Engine, this: Ptr<ActorMover>) {
    let base = this.addr();
    let mut flags = 0u32;
    let mut moving = false;
    // The exe's frame (`SUB ESP,0x40` under `EBP`): the delta at +0x2c (`EBP-0x14`), the built
    // step point at +0x14 (`EBP-0x2c`).
    e.with_stack(0x40, |e, frame| {
        let frame = frame.addr();
        let delta = frame + 0x2c;
        copy_point(e, UNSET_POINT, delta);
        let actor = e.get(this, ActorMover::pActor);
        if e.get(this, ActorMover::bForceMoveMode) {
            flags = e.get(this, ActorMover::uiForcedMoveMode);
            let animation = e.vcall(actor.addr(), 0x1e4, &[]).u32();
            if animation != 0 {
                if e.call(ACTOR_FORM_FLAG_10, &args![actor]).bool() {
                    e.call(
                        ANIMATION_GET_DELTA,
                        &args![animation, delta, actor, 1u32, 0u32],
                    );
                } else {
                    e.call(
                        ANIMATION_GET_DELTA,
                        &args![animation, delta, actor, 0u32, 1u32],
                    );
                }
            } else if !e.call(ACTOR_TEST_FLAGS, &args![actor, 7u32]).bool() {
                let speed = e.call(ACTOR_GET_SPEED, &args![actor]).f64();
                let frame_time = e
                    .call(GLOBAL_TIMER_FLOAT, &args![GLOBAL_TIMER_OBJECT])
                    .f64();
                let step = (frame_time * speed) as f32;
                let built = frame + 0x14;
                let step_point = e
                    .call(POINT_CONSTRUCT, &args![built, 0.0f32, step, 0.0f32])
                    .u32();
                copy_point(e, step_point, delta);
            }
            moving = true;
        } else if e.get(this, ActorMover::pCurrentPathHandler).addr() != 0 {
            let handler = e.get(this, ActorMover::pCurrentPathHandler).addr();
            flags = e.vcall(handler, 0x48, &[]).u32();
            let handler = e.get(this, ActorMover::pCurrentPathHandler).addr();
            if e.vcall(handler, 0x4c, &args![delta]).bool() {
                moving = true;
            }
            if e.get(this, ActorMover::bStopCurrentMoveAnim) {
                flags &= !0xf;
                copy_point(e, UNSET_POINT, delta);
                moving = true;
                let action = e.call(ACTOR_GET_ANIM_ACTION, &args![actor]).i32();
                if (12..=14).contains(&action) {
                    e.set(this, ActorMover::bStopCurrentMoveAnim, false);
                    moving = false;
                } else {
                    let animation = e.vcall(actor.addr(), 0x1e4, &[]).u32();
                    if animation == 0
                        || !e.call(ANIMATION_SHOULD_BE_MOVING, &args![animation]).bool()
                    {
                        e.set(this, ActorMover::bStopCurrentMoveAnim, false);
                    }
                }
            }
        }

        if flags & 0x200 != 0 && !e.call(ACTOR_MOVEMENT_TEST, &args![actor]).bool() {
            flags &= !0x200;
            flags |= 0x100;
        }
        if flags & 0x400 != 0 && !e.call(ACTOR_SLOT_218, &args![actor]).bool() {
            flags &= !0x400;
        }
        let cached = e.get(this, ActorMover::iCachedLastValidMoveMode);
        if flags & 0x400 != cached & 0x400 {
            let animation = e.vcall(actor.addr(), 0x1e4, &[]).u32();
            let mut clear = animation == 0;
            if !clear {
                for index in [0u32, 1, 4] {
                    let element = e.call(ANIMATION_ELEMENT, &args![animation, index]).u32();
                    if element != 0 {
                        let element = e.call(ANIMATION_ELEMENT, &args![animation, index]).u32();
                        if e.call(ENTRY_VALUE, &args![element]).u32() != 1 {
                            clear = true;
                            break;
                        }
                    }
                }
                if !clear && e.call(ACTOR_GET_ANIM_ACTION, &args![actor]).i32() != -1 {
                    clear = true;
                }
            }
            if clear {
                flags &= !0x400;
                flags |= cached & 0x400;
            }
        }
        e.set(this, ActorMover::iCachedLastValidMoveMode, flags);
        copy_point(e, delta, base + ActorMover::CachedMoveDelta.off);
        e.set(this, ActorMover::bHaveValidDeltaThisFrame, moving);
    });
    e.call(DEBUG_STUB, &args![this]);
}

// Translated from 009ddf60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009ddf60 - C++ exception unwinding (FS:[0] frame) not translated
/// `ActorMover::SetMoveModePreference` (Xbox PDB, vtable slot 1): the high
/// part (`0xFF00`) of `mode`, when set, creates a detailed path handler if
/// there is none and replaces the preferred mode's high part; the low part
/// (`0x000F`), when set, replaces its low part.
pub fn actor_mover_set_move_mode_preference(e: &mut Engine, this: Ptr<ActorMover>, mode: u32) {
    if mode & 0xff00 != 0 {
        if e.get(this, ActorMover::pCurrentPathHandler).addr() == 0 {
            let block = e.call(OPERATOR_NEW, &args![0xe4u32]).u32();
            let actor = e.get(this, ActorMover::pActor);
            let handler = if block != 0 {
                e.call(
                    DETAILED_HANDLER_CONSTRUCT_SIMPLE,
                    &args![block, actor, this],
                )
                .u32()
            } else {
                0
            };
            e.set(this, ActorMover::pCurrentPathHandler, Ptr::new(handler));
        }
        let preferred = e.get(this, ActorMover::uiPreferredMoveMode);
        e.set(
            this,
            ActorMover::uiPreferredMoveMode,
            (preferred & 0xf) | (mode & 0xff00),
        );
    }
    if mode & 0xf != 0 {
        let preferred = e.get(this, ActorMover::uiPreferredMoveMode);
        e.set(
            this,
            ActorMover::uiPreferredMoveMode,
            (mode & 0xf) | (preferred & 0xff00),
        );
    }
}

// Translated from 009de040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorMover::ClearMoveModePreference` (Xbox PDB, vtable slot 2): clears
/// the bits of `mode` from the preferred move mode.
pub fn actor_mover_clear_move_mode_preference(e: &mut Engine, this: Ptr<ActorMover>, mode: u32) {
    let preferred = e.get(this, ActorMover::uiPreferredMoveMode);
    e.set(this, ActorMover::uiPreferredMoveMode, !mode & preferred);
}

// Translated from 009de060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorMover::ForceMoveMode` (Xbox PDB, vtable slot 3): forces the mode.
pub fn actor_mover_force_move_mode(e: &mut Engine, this: Ptr<ActorMover>, mode: u32) {
    e.set(this, ActorMover::bForceMoveMode, true);
    e.set(this, ActorMover::uiForcedMoveMode, mode);
}

// Translated from 009de080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorMover::ClearForcedMoveMode` (Xbox PDB, vtable slot 4).
pub fn actor_mover_clear_forced_move_mode(e: &mut Engine, this: Ptr<ActorMover>) {
    e.set(this, ActorMover::bForceMoveMode, false);
    e.set(this, ActorMover::uiForcedMoveMode, 0);
}

// Translated from 009de0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the location of the next node into `out`: the path handler's slot
/// 0x20 when there is a path and a handler, otherwise a copy of the actor's
/// position (slot 0x1F4). Returns `out`.
pub fn fn_009de0a0(e: &mut Engine, this: Ptr<ActorMover>, out: Ptr) -> Ptr {
    if current_path(e, this) == 0 || e.get(this, ActorMover::pCurrentPathHandler).addr() == 0 {
        let actor = e.get(this, ActorMover::pActor).addr();
        let position = e.vcall(actor, 0x1f4, &[]).u32();
        copy_point(e, position, out.addr());
        return out;
    }
    let handler = e.get(this, ActorMover::pCurrentPathHandler).addr();
    e.vcall(handler, 0x20, &args![out]);
    out
}

// Translated from 009de110 (decompiled, FalloutNV.exe 1.4.0.525)
/// The path handler's slot 0x24 (location of a node) when there is a path
/// and a handler, otherwise false.
pub fn fn_009de110(e: &mut Engine, this: Ptr<ActorMover>, first: u32, second: u32) -> bool {
    if current_path(e, this) != 0 && e.get(this, ActorMover::pCurrentPathHandler).addr() != 0 {
        let handler = e.get(this, ActorMover::pCurrentPathHandler).addr();
        e.vcall(handler, 0x24, &args![first, second]).bool()
    } else {
        false
    }
}

// Translated from 009de160 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009de160 - C++ exception unwinding (FS:[0] frame) not translated
/// `ActorMover::SetLookAtTarget` (Xbox PDB): stores the point to face and
/// sets `bFaceTargetPoint`; when the actor's process type is 0 and there is
/// no path handler, creates a detailed one.
pub fn actor_mover_set_look_at_target(
    e: &mut Engine,
    this: Ptr<ActorMover>,
    x: f32,
    y: f32,
    z: f32,
) {
    let point = this.addr() + ActorMover::LookAtPoint.off;
    e.mem.set_f32(point, x);
    e.mem.set_f32(point + 4, y);
    e.mem.set_f32(point + 8, z);
    e.set(this, ActorMover::bFaceTargetPoint, true);
    let actor = e.get(this, ActorMover::pActor);
    if e.call(GET_PROCESS_TYPE, &args![actor]).u32() != 0 {
        return;
    }
    if e.get(this, ActorMover::pCurrentPathHandler).addr() == 0 {
        let block = e.call(OPERATOR_NEW, &args![0xe4u32]).u32();
        let handler = if block != 0 {
            e.call(
                DETAILED_HANDLER_CONSTRUCT_SIMPLE,
                &args![block, actor, this],
            )
            .u32()
        } else {
            0
        };
        e.set(this, ActorMover::pCurrentPathHandler, Ptr::new(handler));
    }
    e.call(DEBUG_STUB, &args![this]);
}

// Translated from 009de230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorMover::ClearLookAtTarget` (Xbox PDB): clears the point and
/// `bFaceTargetPoint`.
pub fn actor_mover_clear_look_at_target(e: &mut Engine, this: Ptr<ActorMover>) {
    copy_point(e, UNSET_POINT, this.addr() + ActorMover::LookAtPoint.off);
    e.set(this, ActorMover::bFaceTargetPoint, false);
    e.call(DEBUG_STUB, &args![this]);
}

// Translated from 009de270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the look-at point to `out` and returns true when
/// `bFaceTargetPoint` is set (`008b3d10`), otherwise returns false.
pub fn fn_009de270(e: &mut Engine, this: Ptr<ActorMover>, out: Ptr) -> bool {
    if e.call(FACE_TARGET_POINT, &args![this]).bool() {
        copy_point(e, this.addr() + ActorMover::LookAtPoint.off, out.addr());
        true
    } else {
        false
    }
}

// Translated from 009de2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// When there is a usable path, copies the location of its last node to
/// `out` and returns true, otherwise returns false.
pub fn fn_009de2b0(e: &mut Engine, this: Ptr<ActorMover>, out: Ptr) -> bool {
    if current_path(e, this) == 0 {
        return false;
    }
    let path = current_path(e, this);
    if !fn_009dc530(e, path) {
        return false;
    }
    let path = current_path(e, this);
    let node = fn_009ddb30(e, path);
    if node == 0 {
        return false;
    }
    // The exe's frame (`SUB ESP,0x14` under `EBP`): the position at +4 (`EBP-0x10`).
    e.with_stack(0x14, |e, frame| {
        let scratch = frame.addr() + 4;
        let location = e.call(NODE_LOCATION, &args![node]).u32();
        let position = e.call(GET_POSITION, &args![location, scratch]).u32();
        copy_point(e, position, out.addr());
    });
    true
}

// Translated from 009de330 (decompiled, FalloutNV.exe 1.4.0.525)
/// When there is a usable path, assigns the location of its last node to
/// the `PathingLocation` at `target` and returns true, otherwise false.
pub fn fn_009de330(e: &mut Engine, this: Ptr<ActorMover>, target: Ptr) -> bool {
    if current_path(e, this) == 0 {
        return false;
    }
    let path = current_path(e, this);
    if !fn_009dc530(e, path) {
        return false;
    }
    let path = current_path(e, this);
    let node = fn_009ddb30(e, path);
    if node == 0 {
        return false;
    }
    let location = e.call(NODE_LOCATION, &args![node]).u32();
    e.call(LOCATION_ASSIGN, &args![target, location]);
    true
}

// Translated from 009de3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the door id of a process (`0084e3a0`, 0 for a null process) in
/// `DetectionDoorID`.
pub fn fn_009de3a0(e: &mut Engine, this: Ptr<ActorMover>, process: Ptr) {
    let id = if process.is_null() {
        0
    } else {
        e.call(PROCESS_DOOR_ID, &args![process]).u32()
    };
    e.set(this, ActorMover::DetectionDoorID, id);
}

// Translated from 009de3e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorMover::IsPathValid` (Xbox PDB): there is a path and
/// `fn_009dc530` accepts it.
pub fn actor_mover_is_path_valid(e: &mut Engine, this: Ptr<ActorMover>) -> bool {
    if current_path(e, this) == 0 {
        return false;
    }
    let path = current_path(e, this);
    fn_009dc530(e, path)
}

// Translated from 009de420 (decompiled, FalloutNV.exe 1.4.0.525)
/// There is a path and its byte at +0x40 is set (`006b05d0`).
pub fn fn_009de420(e: &mut Engine, this: Ptr<ActorMover>) -> bool {
    if current_path(e, this) == 0 {
        return false;
    }
    let path = current_path(e, this);
    e.call(PATH_FLAG_40, &args![path]).bool()
}

// Translated from 009de460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorMover::GetFailedPathDestination` (Xbox PDB): the address of the
/// failed destination location, or 0 when its flag (bit 1 of the byte at
/// +0x26, tested by `006a9520`) is set.
pub fn actor_mover_get_failed_path_destination(e: &mut Engine, this: Ptr<ActorMover>) -> u32 {
    let location = this.addr() + ActorMover::FailedPathDestination.off;
    if e.call(LOCATION_FLAG_2, &args![location]).bool() {
        0
    } else {
        location
    }
}

/// Is a position of the request below the water height `TES` reports for
/// it? `destination` selects the request's destination (`006a9540`)
/// instead of its origin (`0048d150`); the position is the one
/// `005c3420` copies out of that location, the height is asked of `TES`
/// for the given `cell`. The comparison is `position.z < height` (an x87
/// compare against a double; a NaN fails it).
fn request_end_below_water(
    e: &mut Engine,
    this: Ptr<ActorMover>,
    destination: bool,
    cell: u32,
    first: u32,
    second: u32,
) -> bool {
    let end = if destination {
        REQUEST_DESTINATION
    } else {
        REQUEST_ORIGIN
    };
    let request = current_request(e, this);
    let location = e.call(end, &args![request]).u32();
    let position = e.call(GET_POSITION, &args![location, first]).u32();
    let tes = e.global::<u32>(TES_POINTER);
    let height = e.call(WATER_HEIGHT_AT, &args![tes, position, cell]).f64();
    let request = current_request(e, this);
    let location = e.call(end, &args![request]).u32();
    let position = e.call(GET_POSITION, &args![location, second]).u32();
    let z = e.mem.f32(position + 8);
    (z as f64) < height
}

/// For the held request: both its origin and its destination have a cell
/// and exactly one of the two lies below the water height at its position. `slots` are the
/// caller's four position locals: the origin's two, then the destination's two.
fn request_ends_differ_in_water(e: &mut Engine, this: Ptr<ActorMover>, slots: [u32; 4]) -> bool {
    let request = current_request(e, this);
    let origin = e.call(REQUEST_ORIGIN, &args![request]).u32();
    let origin_cell = e.call(LOCATION_GET_CELL, &args![origin]).u32();
    let request = current_request(e, this);
    let destination = e.call(REQUEST_DESTINATION, &args![request]).u32();
    let destination_cell = e.call(LOCATION_GET_CELL, &args![destination]).u32();
    if origin_cell == 0 || destination_cell == 0 {
        return false;
    }
    let origin_below = request_end_below_water(e, this, false, origin_cell, slots[0], slots[1]);
    let destination_below =
        request_end_below_water(e, this, true, destination_cell, slots[2], slots[3]);
    origin_below != destination_below
}

/// The game timer's time (`00825c00` on `011f6394`).
fn timer_now(e: &mut Engine) -> u32 {
    e.call(TIMER_NOW, &args![GLOBAL_TIMER_OBJECT]).u32()
}

/// `(now - stamp)` milliseconds as a `float` number of seconds: the 32-bit
/// difference is loaded as an unsigned 64-bit integer, multiplied by
/// `0.001` and rounded once to `float`.
fn seconds_since(e: &mut Engine, stamp: u32) -> f32 {
    let now = timer_now(e);
    let factor: f64 = e.global(MILLISECONDS_TO_SECONDS);
    (now.wrapping_sub(stamp) as f64 * factor) as f32
}

// Translated from 009de490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorMover::SetPathingFailed` (Xbox PDB): stores the flag. Clearing it
/// resets the failed destination (`006d2c00`) and ends. Setting it, a
/// process-type-3 actor standing in the corner grid cell (0, 0) of an
/// exterior with no nav mesh info for it is placed at the request's
/// destination (`008ad1c0`); then the failure time-out (3.0 seconds when
/// the actor is visible, 1.5 otherwise, 1.0 when the request's ends lie on
/// different sides of the water) is compared with the seconds since the
/// last and the first failure: a repeated failure inside the time-out only
/// counts, a fresh one resets the count, and a third failure after the
/// time-out of the first runs `fn_009de8b0` first.
pub fn actor_mover_set_pathing_failed(e: &mut Engine, this: Ptr<ActorMover>, failed: bool) {
    e.set(this, ActorMover::bPathingFailed, failed);
    if !failed {
        e.call(
            LOCATION_RESET,
            &args![this.addr() + ActorMover::FailedPathDestination.off],
        );
        return;
    }
    // The exe's frame (`SUB ESP,0x8c` under `EBP`): 0x8c bytes; the placement position at
    // +0x60 (`EBP-0x2c`), the water test's origin positions at +0x54 and +0x48 (`EBP-0x38`,
    // `-0x44`) and destination positions at +0x3c and +0x30 (`EBP-0x50`, `-0x5c`).
    e.with_stack(0x8c, |e, frame| {
        let frame = frame.addr();
        let actor = e.get(this, ActorMover::pActor).addr();
        if e.call(GET_PROCESS_TYPE, &args![actor]).i32() == 3 {
            let cell = e.call(GET_PARENT_CELL, &args![actor]).u32();
            let skip = cell != 0 && {
                let cell = e.call(GET_PARENT_CELL, &args![actor]).u32();
                e.call(CELL_FLAG_0, &args![cell]).bool()
            };
            if !skip {
                let position = e.vcall(actor, 0x1f4, &[]).u32();
                let x = e.mem.f32(position) as f64;
                let grid_x = e.call(FTOL, &args![x]).i32() >> 12;
                let position = e.vcall(actor, 0x1f4, &[]).u32();
                let y = e.mem.f32(position + 4) as f64;
                let grid_y = e.call(FTOL, &args![y]).i32() >> 12;
                if grid_x == 0 && grid_y == 0 {
                    let packed = e
                        .call(
                            PACK_GRID_COORDS,
                            &args![grid_x as u16 as u32, grid_y as u16 as u32],
                        )
                        .u32();
                    let world_space = e.call(GET_WORLD_SPACE, &args![actor]).u32();
                    let tes = e.global::<u32>(TES_POINTER);
                    let map = e.call(NAV_MESH_INFO_MAP, &args![tes]).u32();
                    if e.call(NAV_MESH_INFO_FIND, &args![map, world_space, packed])
                        .u32()
                        == 0
                    {
                        let request = current_request(e, this);
                        let destination = e.call(REQUEST_DESTINATION, &args![request]).u32();
                        let space = e.call(DWORD_AT_1C, &args![destination]).u32();
                        let request = current_request(e, this);
                        let destination = e.call(REQUEST_DESTINATION, &args![request]).u32();
                        let word_18 = e.call(DWORD_AT_18, &args![destination]).u32();
                        let scratch = frame + 0x60;
                        let request = current_request(e, this);
                        let destination = e.call(REQUEST_DESTINATION, &args![request]).u32();
                        let position = e.call(GET_POSITION, &args![destination, scratch]).u32();
                        e.call(
                            ACTOR_PLACE,
                            &args![actor, position, 0.0f32, word_18, space, 0u32, 1u32],
                        );
                    }
                }
            }
        }
        let visible = e
            .call(ACTOR_TEST_FLAGS, &args![actor, IS_VISIBLE_MASK])
            .bool();
        let mut timeout = fpu_copy(e.global::<f32>(if visible {
            FAILURE_TIMEOUT_VISIBLE
        } else {
            FAILURE_TIMEOUT_HIDDEN
        }));
        if current_request(e, this) != 0
            && request_ends_differ_in_water(
                e,
                this,
                [frame + 0x54, frame + 0x48, frame + 0x3c, frame + 0x30],
            )
        {
            timeout = fpu_copy(e.global::<f32>(FAILURE_TIMEOUT_WATER));
        }
        let last = e.get(this, ActorMover::iLastFailureTimeStamp);
        let since_last = seconds_since(e, last);
        let first = e.get(this, ActorMover::iFirstFailureTimeStamp);
        let since_first = seconds_since(e, first);
        if timeout <= since_last {
            e.set(this, ActorMover::iContinuedFailureCount, 0);
            let now = timer_now(e);
            e.set(this, ActorMover::iLastFailureTimeStamp, now);
            let now = timer_now(e);
            e.set(this, ActorMover::iFirstFailureTimeStamp, now);
        } else if timeout <= since_first && e.get(this, ActorMover::iContinuedFailureCount) >= 2 {
            fn_009de8b0(e, this);
            e.set(this, ActorMover::iContinuedFailureCount, 0);
            let now = timer_now(e);
            e.set(this, ActorMover::iLastFailureTimeStamp, now);
            let now = timer_now(e);
            e.set(this, ActorMover::iFirstFailureTimeStamp, now);
        } else {
            let count = e.get(this, ActorMover::iContinuedFailureCount);
            e.set(
                this,
                ActorMover::iContinuedFailureCount,
                count.wrapping_add(1),
            );
            let now = timer_now(e);
            e.set(this, ActorMover::iLastFailureTimeStamp, now);
        }
    });
    e.call(DEBUG_STUB, &args![this]);
}

// Translated from 009de8b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009de8b0 - C++ exception unwinding (FS:[0] frame) not translated
/// Called by `SetPathingFailed` on a third failure (name unconfirmed).
/// When the path handler can warp to the next node (slot 0x74) and a path
/// exists, it only does that. Otherwise it decides whether the actor is
/// stuck: a follower farther from the player than twice the request's
/// target radius, or an actor of process type 0 whose request ends lie on
/// different sides of the water or whose own location is not resolved. If
/// it is, it solves a new request (to the actor's own location, radius
/// 10, ...) and places the actor at the point 128 or 256 units short of
/// the path's end (or at the first point of an unsolved path), through
/// `fn_009df010`, or through the actor's slot 0x2A8 after
/// `MoveRefToNewSpace` for a follower in another world space.
pub fn fn_009de8b0(e: &mut Engine, this: Ptr<ActorMover>) {
    // The exe's frame (`EBP` frame, exception record, `SUB ESP,0x1f8`, `ESI`, the cookie): 0x210
    // bytes. The follow distance's difference at +0xa4 (`EBP-0x16c`); the water test's positions
    // at +0x98, +0x8c, +0x80, +0x74 (`EBP-0x178` ... `-0x19c`); the actor's location at +0x1d0
    // (`EBP-0x40`); the request at +0x10c (`EBP-0x104`) and its destination location at +0x4c
    // (`EBP-0x1c4`); the target at +0x1c0 (`EBP-0x50`); the path at +0xbc (`EBP-0x154`); the
    // path points at +0x40 (`EBP-0x1d0`) and +0x34 (`EBP-0x1dc`).
    e.with_stack(0x210, |e, frame| unstick_in_frame(e, this, frame.addr()));
}

/// [`fn_009de8b0`] in the exe's frame (`frame` is its lowest address).
fn unstick_in_frame(e: &mut Engine, this: Ptr<ActorMover>, frame: u32) {
    let base = this.addr();
    let actor = e.get(this, ActorMover::pActor).addr();
    let handler = e.get(this, ActorMover::pCurrentPathHandler).addr();
    if handler != 0
        && ni_pointer_value(e, handler + 0x0c) != 0
        && e.vcall(handler, 0x74, &[]).bool()
    {
        e.call(DEBUG_STUB, &args![this]);
        return;
    }
    let mut stuck = false;
    if current_request(e, this) == 0 {
        return;
    }
    let player = e.global::<u32>(PLAYER_CHARACTER);
    if e.call(IS_FOLLOWING, &args![actor, player]).bool() {
        let handler = e.get(this, ActorMover::pCurrentPathHandler).addr();
        if handler != 0
            && ni_pointer_value(e, base + 0x20) != 0
            && !e.vcall(handler, 0x34, &[]).bool()
        {
            let path = current_path(e, this);
            if !e.call(PATH_FLAG_40, &args![path]).bool() {
                e.call(ACTOR_FOLLOW_FALLBACK, &args![actor]);
                return;
            }
        }
        let player_position = e.vcall(player, 0x1f4, &[]).u32();
        let actor_position = e.vcall(actor, 0x1f4, &[]).u32();
        let difference = frame + 0xa4;
        let result = e
            .call(
                POINT_SUBTRACT,
                &args![actor_position, difference, player_position],
            )
            .u32();
        let distance = e.call(DIFFERENCE_LENGTH, &args![result]).f64();
        let request = current_request(e, this);
        let radius = e.call(REQUEST_TARGET_RADIUS, &args![request]).f64();
        if radius + radius < distance {
            stuck = true;
        }
    }
    if e.call(GET_PROCESS_TYPE, &args![actor]).i32() == 0 {
        if request_ends_differ_in_water(
            e,
            this,
            [frame + 0x98, frame + 0x8c, frame + 0x80, frame + 0x74],
        ) {
            stuck = true;
        }
        let location = frame + 0x1d0;
        e.call(LOCATION_FROM_ACTOR, &args![location, actor]);
        e.call(LOCATION_RESOLVE_NAV_MESH, &args![location, 0u32]);
        if !e.call(LOCATION_IS_RESOLVED, &args![location]).bool() {
            stuck = true;
        }
        e.call(LOCATION_DESTRUCT, &args![location]);
    }
    if !stuck {
        return;
    }

    let request = frame + 0x10c;
    e.call(REQUEST_CONSTRUCT, &args![request]);
    let held = current_request(e, this);
    let destination = e.call(REQUEST_DESTINATION, &args![held]).u32();
    e.call(REQUEST_SET_LOCATION, &args![request, destination]);
    let location = frame + 0x4c;
    e.call(LOCATION_FROM_ACTOR, &args![location, actor]);
    e.call(REQUEST_SET_DESTINATION, &args![request, location]);
    e.call(LOCATION_DESTRUCT, &args![location]);
    let radius = fpu_copy(e.global::<f32>(FAILURE_TARGET_RADIUS));
    e.call(REQUEST_SET_TARGET_RADIUS, &args![request, radius]);
    let actor_float = e.call(ACTOR_FLOAT, &args![actor]).f64();
    let scale: f64 = e.global(ACTOR_FLOAT_SCALE);
    let scaled = fpu_copy((actor_float * scale) as f32);
    e.call(REQUEST_SET_SECOND_FLOAT, &args![request, scaled]);
    e.call(REQUEST_SET_ACTOR, &args![request, actor]);
    e.call(REQUEST_SET_BYTE_A, &args![request, 1u32]);
    fn_009deff0(e, Ptr::new(request), 1);
    e.call(REQUEST_SET_CANT_OPEN_DOORS, &args![request, 0u32]);
    let actor_base = e.call(ACTOR_BASE, &args![actor]).u32();
    let can_swim = e.call(BASE_CAN_SWIM, &args![actor_base]).bool();
    e.call(REQUEST_SET_CAN_SWIM, &args![request, can_swim]);
    let actor_base = e.call(ACTOR_BASE, &args![actor]).u32();
    let can_fly = e.call(BASE_CAN_FLY, &args![actor_base]).bool();
    e.call(REQUEST_SET_CAN_FLY, &args![request, can_fly]);
    e.call(REQUEST_SET_BYTE_B, &args![request, 1u32]);

    let target = frame + 0x1c0;
    e.call(POINT_CONSTRUCT_DEFAULT, &args![target]);
    // The parent cell of the player is read and kept in a local the code
    // never uses again.
    e.call(GET_PARENT_CELL, &args![player]);
    let path = frame + 0xbc;
    e.call(PATH_CONSTRUCT, &args![path]);
    let solved = e.call(SOLVE_REQUEST, &args![request, path]).bool();
    let mut found = true;
    if solved {
        let count = e.call(PATH_COUNT_20, &args![path]).u32();
        if count < 2 {
            found = false;
        } else {
            let mut margin = fpu_copy(e.global::<f32>(PATH_MARGIN_SHORT));
            let actor_float = e.call(ACTOR_FLOAT, &args![actor]).f64();
            let half: f64 = e.global(HALF);
            let limit: f64 = e.global(HUNDRED);
            if actor_float * half > limit {
                margin = fpu_copy(e.global::<f32>(PATH_MARGIN_LONG));
            }
            let length = e.call(PATH_LENGTH, &args![path]).f64() as f32;
            let mut distance = (length as f64 - margin as f64) as f32;
            let zero: f64 = e.global(ZERO_DOUBLE);
            if (distance as f64) < zero {
                distance = (length as f64 * half) as f32;
            }
            let out = frame + 0x40;
            let point = e
                .call(PATH_POINT_AT, &args![path, out, 0.0f32, fpu_copy(distance)])
                .u32();
            copy_point(e, point, target);
        }
    } else if e.call(PATH_COUNT_20, &args![path]).u32() == 0 {
        found = false;
    } else {
        let out = frame + 0x34;
        let element = e.call(PATH_SECOND_ARRAY_ELEMENT, &args![path]).u32();
        let node_location = e.call(NODE_LOCATION, &args![element]).u32();
        let point = e.call(GET_POSITION, &args![node_location, out]).u32();
        copy_point(e, point, target);
    }
    if found {
        e.call(DEBUG_STUB, &args![this]);
        if e.call(IS_FOLLOWING, &args![actor, player]).bool() {
            let own_space = e.call(GET_WORLD_SPACE, &args![actor]).u32();
            let player_space = e.call(GET_WORLD_SPACE, &args![player]).u32();
            if own_space != player_space {
                let space = e.call(GET_WORLD_SPACE, &args![player]).u32();
                let cell = e.call(GET_PARENT_CELL, &args![player]).u32();
                e.call(MOVE_REF_TO_NEW_SPACE, &args![actor, cell, space]);
                e.vcall(actor, ACTOR_SLOT_PLACE, &args![target]);
            } else {
                actor_mover_move_actor_to_location(e, this, Ptr::new(target));
            }
        } else {
            actor_mover_move_actor_to_location(e, this, Ptr::new(target));
        }
    }
    e.call(PATH_DESTRUCT, &args![path]);
    e.call(REQUEST_DESTRUCT, &args![request]);
}

// Translated from 009deff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingRequest` method: stores the byte at +0xA3
/// (`bIgnoreSmoothingIfFails`, Xbox PDB). The receiver is a request.
pub fn fn_009deff0(e: &mut Engine, this: Ptr, flag: u32) {
    e.mem.set_u8(this.addr() + 0xa3, flag as u8);
}

// Translated from 009df010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorMover::MoveActorToLocation` (Xbox PDB): places the actor at the
/// position. With the global test (`008c7aa0`) set, it reports the move
/// to the global of `004537b0` (`0087b650`, `0087b190`) instead of using
/// the actor's slot 0x2A8. Then it sets the location on the reference; in
/// an actor without a world space that is all (returns true). Otherwise
/// the actor moves to a new space when the cell at the position is not
/// its cell. Returns true when the actor's desired process level is not 0
/// or the player is sleeping or resting.
pub fn actor_mover_move_actor_to_location(
    e: &mut Engine,
    this: Ptr<ActorMover>,
    position: Ptr,
) -> bool {
    let actor = e.get(this, ActorMover::pActor).addr();
    if e.call(GLOBAL_TEST, &[]).bool() {
        let target = e.call(REPORT_TARGET_OBJECT, &[]).u32();
        e.call(REPORT_PATH_END, &args![target, actor, position]);
        let target = e.call(REPORT_TARGET_OBJECT, &[]).u32();
        e.call(REPORT_MOVE_DONE, &args![target, actor]);
    } else {
        e.vcall(actor, ACTOR_SLOT_PLACE, &args![position]);
    }
    e.call(SET_LOCATION_ON_REFERENCE, &args![actor, position]);
    let world_space = e.call(GET_WORLD_SPACE, &args![actor]).u32();
    if world_space == 0 {
        return true;
    }
    let cell = e.call(GET_PARENT_CELL, &args![actor]).u32();
    let cell_at = e
        .call(GET_CELL_FROM_WORLD_COORD, &args![world_space, position])
        .u32();
    if cell != cell_at {
        e.call(MOVE_REF_TO_NEW_SPACE, &args![actor, 0u32, world_space]);
    }
    if e.call(GET_DESIRED_PROCESS_LEVEL, &args![actor]).u32() != 0 {
        return true;
    }
    let player = e.global::<u32>(PLAYER_CHARACTER);
    e.call(PLAYER_IS_SLEEPING, &args![player]).bool()
}

/// Writes `size` bytes at `address` to the save buffer.
fn save_bytes(e: &mut Engine, buffer: u32, address: u32, size: u32) {
    e.call(SAVE_BYTES, &args![buffer, address, size, 0u32]);
}

// Translated from 009df100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorMover::SaveGame` (Xbox PDB; vtable slot 10): writes the node
/// indexes, flags, move modes, points, state, the time since the last
/// failure and the failure count, saves the failed destination through its
/// vtable (slot 0), writes the detection door's form id, then a flag byte
/// (bit 0 request, bit 1 path, 4 plain handler or 8 detailed handler) and
/// the request (type byte, then its slot 0x18), the path (`006e8b10`) and
/// the handler (slot 0x78).
pub fn actor_mover_save_game(e: &mut Engine, this: Ptr<ActorMover>, buffer: Ptr) {
    let base = this.addr();
    let save = buffer.addr();
    for (offset, size) in [
        (0x40, 2),
        (0x42, 2),
        (0x70, 1),
        (0x34, 4),
        (0x71, 1),
        (0x38, 4),
        (0x72, 1),
        (0x73, 1),
        (0x04, 12),
        (0x10, 12),
        (0x3c, 4),
        (0x74, 1),
        (0x75, 1),
        (0x77, 1),
        (0x76, 1),
        (0x78, 1),
        (0x6c, 4),
    ] {
        save_bytes(e, save, base + offset, size);
    }
    // The exe's frame (`SUB ESP,0x18` under `EBP`): the time since the last failure at +0x10
    // (`EBP-8`), the flag byte at +0x17 (`EBP-1`) and the request kind at +0xf (`EBP-9`).
    e.with_stack(0x18, |e, frame| {
        let frame = frame.addr();
        let local = frame + 0x10;
        let last = e.get(this, ActorMover::iLastFailureTimeStamp);
        let now = timer_now(e);
        e.mem.set_u32(local, now.wrapping_sub(last));
        save_bytes(e, save, local, 4);
        save_bytes(e, save, base + 0x84, 4);
        e.vcall(base + 0x44, 0x00, &args![buffer]);
        let door = e.get(this, ActorMover::DetectionDoorID);
        e.call(SAVE_FORM_ID, &args![buffer, door, 0u32]);
        let mut flags = 0u8;
        if current_request(e, this) != 0 {
            flags |= 1;
        }
        if current_path(e, this) != 0 {
            flags |= 2;
        }
        let handler = e.get(this, ActorMover::pCurrentPathHandler).addr();
        if handler != 0 {
            flags |= if e.vcall(handler, 0x34, &[]).bool() {
                8
            } else {
                4
            };
        }
        e.mem.set_u8(frame + 0x17, flags);
        save_bytes(e, save, frame + 0x17, 1);
        if current_request(e, this) != 0 {
            let request = current_request(e, this);
            let kind = e.vcall(request, 0x10, &[]).u8();
            e.mem.set_u8(frame + 0xf, kind);
            save_bytes(e, save, frame + 0xf, 1);
            let request = current_request(e, this);
            e.vcall(request, 0x18, &args![buffer]);
        }
        if current_path(e, this) != 0 {
            let path = current_path(e, this);
            e.call(PATH_SAVE_GAME, &args![path, buffer]);
        }
        let handler = e.get(this, ActorMover::pCurrentPathHandler).addr();
        if handler != 0 {
            e.vcall(handler, 0x78, &args![buffer]);
        }
    });
}

/// Reads `size` bytes from the load buffer into `address`.
fn load_bytes(e: &mut Engine, buffer: u32, address: u32, size: u32) {
    e.call(LOAD_BYTES, &args![buffer, address, size]);
}

// Translated from 009df3d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009df3d0 - C++ exception unwinding (FS:[0] frame) not translated
/// `ActorMover::LoadGame` (Xbox PDB; vtable slot 11): reads what
/// `SaveGame` wrote. The save-version byte (the buffer's vtable slot 0)
/// gates the failure timing (0x0E and later) and the failed destination
/// (7 and later). It builds the request (`CreatePathingRequest`, then its
/// slot 0x1C; the actor is set when the process part test says so), the
/// path (loaded, then each node tested: a node whose test passes and
/// whose location has no nav mesh makes the mover fail with a printed
/// message) and the handler (virtual one for bit 4, detailed one for bit
/// 8, then its slot 0x7C). A stale path clears the current data, sets
/// the state to 7 and calls `SetPathingFailed(true)`.
pub fn actor_mover_load_game(e: &mut Engine, this: Ptr<ActorMover>, buffer: Ptr) {
    let base = this.addr();
    let load = buffer.addr();
    for (offset, size) in [
        (0x40, 2),
        (0x42, 2),
        (0x70, 1),
        (0x34, 4),
        (0x71, 1),
        (0x38, 4),
        (0x72, 1),
        (0x73, 1),
        (0x04, 12),
        (0x10, 12),
        (0x3c, 4),
        (0x74, 1),
        (0x75, 1),
        (0x77, 1),
        (0x76, 1),
        (0x78, 1),
        (0x6c, 4),
    ] {
        load_bytes(e, load, base + offset, size);
    }
    // The exe's frame (`EBP` frame, exception record, `SUB ESP,0x3c`, the cookie): 0x4c bytes,
    // the stored time at +0x38 (`EBP-0x14`), the flag byte at +0x3e (`EBP-0xe`) and the request
    // kind at +0x37 (`EBP-0x15`).
    e.with_stack(0x4c, |e, frame| {
        let frame = frame.addr();
        let local = frame + 0x38;
        if e.vcall(load, 0x00, &[]).u8() >= 0x0e {
            e.mem.set_u32(local, 0);
            load_bytes(e, load, local, 4);
            let now = timer_now(e);
            let stored = e.mem.u32(local);
            if stored < now {
                let now = timer_now(e);
                e.set(
                    this,
                    ActorMover::iLastFailureTimeStamp,
                    now.wrapping_sub(stored),
                );
            }
            load_bytes(e, load, base + 0x84, 4);
        }
        if e.vcall(load, 0x00, &[]).u8() >= 7 {
            e.vcall(base + 0x44, 0x04, &args![buffer]);
        }
        e.call(LOAD_FORM_ID, &args![load, base + 0x2c]);
        e.mem.set_u8(frame + 0x3e, 0);
        load_bytes(e, load, frame + 0x3e, 1);
        let flags = e.mem.u8(frame + 0x3e);
        let actor = e.get(this, ActorMover::pActor).addr();
        if flags & 1 != 0 {
            e.mem.set_u8(frame + 0x37, 0);
            load_bytes(e, load, frame + 0x37, 1);
            let kind = e.mem.u8(frame + 0x37) as u32;
            let request = e.call(CREATE_PATHING_REQUEST, &args![kind]).u32();
            set_ni_pointer(e, base + 0x1c, request);
            let request = current_request(e, this);
            e.vcall(request, 0x1c, &args![buffer]);
            if actor != 0 {
                let process = e.call(GET_PROCESS, &args![actor]).u32();
                if process != 0 {
                    let process = e.call(GET_PROCESS, &args![actor]).u32();
                    if !e.call(PROCESS_PART_TEST, &args![process + 4, 0u32]).bool() {
                        let request = current_request(e, this);
                        e.call(REQUEST_SET_ACTOR, &args![request, actor]);
                    }
                }
            }
        }
        let mut stale = false;
        if flags & 2 != 0 {
            let block = e.call(ALLOCATE, &args![0x44u32]).u32();
            let path = if block != 0 {
                e.call(PATH_CONSTRUCT, &args![block]).u32()
            } else {
                0
            };
            set_ni_pointer(e, base + 0x20, path);
            let path = current_path(e, this);
            e.call(PATH_LOAD_GAME, &args![path, buffer]);
            let mut index = 0u32;
            loop {
                let path = current_path(e, this);
                let count = e.call(PATH_NODE_COUNT, &args![path]).u32();
                if index >= count {
                    break;
                }
                let path = current_path(e, this);
                let node = e.call(PATH_NODE_AT, &args![path, index]).u32();
                if e.call(NODE_TEST_FLAG, &args![node]).bool() {
                    let path = current_path(e, this);
                    let node = e.call(PATH_NODE_AT, &args![path, index]).u32();
                    let location = e.call(NODE_LOCATION, &args![node]).u32();
                    if e.call(LOCATION_NAV_MESH_TEST, &args![location]).u32() == 0 {
                        let id = e.call(PROCESS_DOOR_ID, &args![actor]).u32();
                        let name = e.vcall(actor, 0x130, &[]).u32();
                        e.call(PRINT_LINE, &args![STALE_PATH_FORMAT, name, id]);
                        stale = true;
                        break;
                    }
                }
                index += 1;
            }
        }
        if flags & 4 != 0 {
            let block = e.call(OPERATOR_NEW, &args![0x20u32]).u32();
            let handler = if block != 0 {
                let request = current_request(e, this);
                e.call(
                    VIRTUAL_HANDLER_CONSTRUCT,
                    &args![block, actor, this, base + 0x20, request],
                )
                .u32()
            } else {
                0
            };
            e.set(this, ActorMover::pCurrentPathHandler, Ptr::new(handler));
        } else if flags & 8 != 0 {
            let block = e.call(OPERATOR_NEW, &args![0xe4u32]).u32();
            let handler = if block != 0 {
                let request = current_request(e, this);
                e.call(
                    DETAILED_HANDLER_CONSTRUCT,
                    &args![block, actor, this, base + 0x20, request],
                )
                .u32()
            } else {
                0
            };
            e.set(this, ActorMover::pCurrentPathHandler, Ptr::new(handler));
        }
        let handler = e.get(this, ActorMover::pCurrentPathHandler).addr();
        if handler != 0 {
            e.vcall(handler, 0x7c, &args![buffer]);
        }
        if stale {
            actor_mover_clear_current_data(e, this);
            e.set(this, ActorMover::eState, STATE_FAILED);
            actor_mover_set_pathing_failed(e, this, true);
        }
    });
}

// Translated from 009df8c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009df8c0 - C++ exception unwinding (FS:[0] frame) not translated
/// `ActorMover::InitLoadGame` (Xbox PDB; vtable slot 12): passes the load
/// buffer to the handler (slot 0x80). With a request: when its type is 5
/// and the actor's slot 0x428 object resolves (`004fb070`, `009d9f20`),
/// the request is attached to that object through a temporary `NiPointer`
/// (`009d9ec0`); then the request's slot 0x20 runs and the request is
/// given the actor.
pub fn actor_mover_init_load_game(e: &mut Engine, this: Ptr<ActorMover>, buffer: Ptr) {
    let actor = e.get(this, ActorMover::pActor).addr();
    let handler = e.get(this, ActorMover::pCurrentPathHandler).addr();
    if handler != 0 {
        e.vcall(handler, 0x80, &args![buffer]);
    }
    if current_request(e, this) == 0 {
        return;
    }
    let request = current_request(e, this);
    if e.vcall(request, 0x10, &[]).u32() == 5 {
        let held = current_request(e, this);
        let object = e.vcall(actor, 0x428, &[]).u32();
        if object != 0 && e.call(SLOT_OBJECT_HOLDER, &args![object]).u32() != 0 {
            let holder = e.call(SLOT_OBJECT_HOLDER, &args![object]).u32();
            let value = e.call(SLOT_OBJECT_VALUE, &args![holder]).u32();
            // The exe's frame (`EBP` frame, exception record, `SUB ESP,0x18`, the cookie): 0x28
            // bytes, the temporary `NiPointer` at +0x10 (`EBP-0x18`).
            e.with_stack(0x28, |e, frame| {
                let temporary = frame.addr() + 0x10;
                e.call(NI_POINTER_CONSTRUCT, &args![temporary, value]);
                e.call(REQUEST_ATTACH, &args![held, temporary]);
                e.call(NI_POINTER_DESTRUCT, &args![temporary]);
            });
        }
    }
    let request = current_request(e, this);
    e.vcall(request, 0x20, &args![buffer]);
    let request = current_request(e, this);
    e.call(REQUEST_SET_ACTOR, &args![request, actor]);
}

// Translated from 009dfa00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009dfa00 - C++ exception unwinding (FS:[0] frame) not translated
/// `ActorMover::FinishLoadGame` (Xbox PDB; vtable slot 13; the buffer
/// word is not read): when the mover was waiting for a path and has a
/// request, it re-asks `PathManager::BuildPath` (creating the message
/// queue if needed); a request of type 5 whose holder (`00984230`) is
/// empty, or a refused `BuildPath`, fails the mover and ends the wait.
pub fn actor_mover_finish_load_game(e: &mut Engine, this: Ptr<ActorMover>, _unused_1: u32) {
    let base = this.addr();
    if !e.get(this, ActorMover::bWaitingForPath) || current_request(e, this) == 0 {
        return;
    }
    let mut rebuild = false;
    let request = current_request(e, this);
    if e.vcall(request, 0x10, &[]).u32() == 5 {
        let held = current_request(e, this);
        let holder = e.call(REQUEST_HOLDER, &args![held]).u32();
        if ni_pointer_value(e, holder) == 0 {
            rebuild = true;
        }
    }
    if ni_pointer_value(e, base + 0x30) == 0 {
        reset_message_queue(e, this);
    }
    if !rebuild {
        // The exe's frame (`EBP` frame, exception record, `SUB ESP,0x38`, the cookie): 0x48 bytes;
        // the `NiPointer` copies `BuildPath` destroys are built in their own argument slots
        // below it: the flag 1 at `frame - 4`, the queue copy at `frame - 8` (009dfafc), the
        // request copy at `frame - 0xc` (009dfb1e).
        let built = e.with_stack(0xc + 0x48, |e, raw| {
            let raw = raw.addr();
            e.mem.set_u32(raw + 8, 1);
            e.call(QUEUE_POINTER_COPY, &args![raw + 4, base + 0x30]);
            e.call(REQUEST_POINTER_COPY, &args![raw, base + 0x1c]);
            let manager = e.call(PATH_MANAGER_INSTANCE, &[]).u32();
            let queue_copy = e.mem.u32(raw + 4);
            let request_copy = e.mem.u32(raw);
            e.call(BUILD_PATH, &args![manager, request_copy, queue_copy, true])
                .bool()
        });
        if built {
            return;
        }
    }
    actor_mover_set_pathing_failed(e, this, true);
    e.set(this, ActorMover::bWaitingForPath, false);
}

// Translated from 009dfb80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009dfb80 - C++ exception unwinding (FS:[0] frame) not translated
/// `ActorMover::Revert` (Xbox PDB; vtable slot 14; the argument is not
/// read): the constructor's reset of the state, after releasing what the
/// mover holds: a message queue replaced when it was waiting for a path,
/// the request and the path dropped, the handler deleted, then
/// `SetPathingFailed(false)`.
pub fn actor_mover_revert(e: &mut Engine, this: Ptr<ActorMover>, _unused_1: u32) {
    let base = this.addr();
    if e.get(this, ActorMover::bWaitingForPath) {
        reset_message_queue(e, this);
    }
    e.set(this, ActorMover::eState, 0);
    set_ni_pointer(e, base + 0x1c, 0);
    set_ni_pointer(e, base + 0x20, 0);
    let handler = e.get(this, ActorMover::pCurrentPathHandler).addr();
    delete_handler(e, handler);
    e.set(this, ActorMover::pCurrentPathHandler, Ptr::NULL);
    e.set(this, ActorMover::uiPreferredMoveMode, 0x101);
    actor_mover_set_pathing_failed(e, this, false);
    e.set(this, ActorMover::iPreviousNode, 0xffff);
    e.set(this, ActorMover::iNextNode, 0xffff);
    e.set(this, ActorMover::bActorChangedProcessLevel, false);
    e.set(this, ActorMover::bStopMovementNextUpdate, false);
    e.set(this, ActorMover::bWaitingForPath, false);
    e.set(this, ActorMover::bInDialog, false);
    copy_point(e, UNSET_POINT, base + ActorMover::LookAtPoint.off);
    copy_point(e, UNSET_POINT, base + ActorMover::CachedMoveDelta.off);
    e.set(this, ActorMover::bFaceTargetPoint, false);
    e.set(this, ActorMover::bForceMoveMode, false);
    e.set(this, ActorMover::bStopCurrentMoveAnim, false);
    e.set(this, ActorMover::bHaveValidDeltaThisFrame, false);
    e.set(this, ActorMover::uiForcedMoveMode, 0);
    e.set(this, ActorMover::iCachedLastValidMoveMode, 0);
    e.set(this, ActorMover::DetectionDoorID, 0);
    e.set(this, ActorMover::iContinuedFailureCount, 0);
    e.set(this, ActorMover::iLastFailureTimeStamp, 0);
    e.set(this, ActorMover::iFirstFailureTimeStamp, 0);
}

// Translated from 009dfd50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The last element of an array of 0x30-byte elements whose count is at
/// +8 (the element at `count - 1`, through `006e8f40`); the receiver is
/// the array.
pub fn fn_009dfd50(e: &mut Engine, array: u32) -> u32 {
    let count = e.mem.u32(array + 8);
    e.call(ARRAY_ELEMENT_BY_INDEX, &args![array, count.wrapping_sub(1)])
        .u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005f36f0, fn_005f36f0(Ptr<ActorMover>) -> u32),
        entry!(0x006e5cc0, ni_pointer_operator_assign(Ptr, Ptr) -> Ptr),
        entry!(
            0x009dad00,
            actor_mover_actor_mover(Ptr<ActorMover>, Ptr) -> Ptr<ActorMover>
        ),
        entry!(
            0x009daec0,
            actor_mover_scalar_deleting_destructor(Ptr<ActorMover>, u32) -> Ptr<ActorMover>
        ),
        entry!(0x009daef0, fn_009daef0(Ptr<ActorMover>)),
        entry!(0x009daf80, actor_mover_clear_current_data(Ptr<ActorMover>)),
        entry!(0x009db090, fn_009db090(Ptr<ActorMover>, Ptr) -> bool),
        entry!(0x009db930, fn_009db930(u32, u32)),
        entry!(
            0x009db950,
            actor_mover_clear_data_for_new_pathing_request(Ptr<ActorMover>, bool)
        ),
        entry!(0x009dba30, fn_009dba30(Ptr<ActorMover>, Ptr) -> bool),
        entry!(
            0x009dba50,
            actor_mover_set_goal(Ptr<ActorMover>, Ptr, Ptr, Ptr, f32, Ptr) -> bool
        ),
        entry!(
            0x009dbb10,
            fn_009dbb10(Ptr<ActorMover>, Ptr, f32, Ptr) -> bool
        ),
        entry!(
            0x009dbb60,
            fn_009dbb60(Ptr<ActorMover>, Ptr, Ptr, Ptr, f32, f32, Ptr) -> bool
        ),
        entry!(
            0x009dbc30,
            fn_009dbc30(Ptr<ActorMover>, Ptr, f32, Ptr) -> bool
        ),
        entry!(0x009dbc90, fn_009dbc90(Ptr, Ptr, Ptr, Ptr, Ptr, f32, Ptr)),
        entry!(0x009dbdc0, fn_009dbdc0(Ptr<ActorMover>, f32)),
        entry!(0x009dc120, actor_mover_update(Ptr<ActorMover>, f32)),
        entry!(0x009dc530, fn_009dc530(u32) -> bool),
        entry!(0x009dc590, fn_009dc590(Ptr<ActorMover>, f32)),
        entry!(0x009dc5e0, fn_009dc5e0(Ptr<ActorMover>, u32)),
        entry!(0x009dc780, fn_009dc780(Ptr<ActorMover>)),
        entry!(0x009dc7f0, fn_009dc7f0(Ptr<ActorMover>)),
        entry!(0x009dc950, fn_009dc950(Ptr<ActorMover>, Ptr)),
        entry!(0x009dc980, fn_009dc980(Ptr<ActorMover>) -> u32),
        entry!(0x009dcab0, fn_009dcab0(Ptr<ActorMover>) -> u32),
        entry!(
            0x009dcae0,
            actor_mover_is_actor_in_loaded_area(Ptr<ActorMover>) -> bool
        ),
        entry!(0x009dcb30, fn_009dcb30(Ptr<ActorMover>) -> bool),
        entry!(0x009dcb90, fn_009dcb90(Ptr<ActorMover>) -> bool),
        entry!(
            0x009dcbf0,
            actor_mover_get_current_percent(Ptr<ActorMover>) -> f32
        ),
        entry!(
            0x009dcc20,
            actor_mover_get_distance_traveled(Ptr<ActorMover>) -> f32
        ),
        entry!(
            0x009dcc50,
            actor_mover_get_distance_remaining(Ptr<ActorMover>) -> f32
        ),
        entry!(0x009dcc80, fn_009dcc80(Ptr<ActorMover>, Ptr, f32) -> Ptr),
        entry!(0x009dccf0, fn_009dccf0(Ptr<ActorMover>) -> bool),
        entry!(
            0x009dcd40,
            actor_mover_is_pathing_complete(Ptr<ActorMover>) -> bool
        ),
        entry!(0x009dcdb0, actor_mover_is_pathing(Ptr<ActorMover>) -> bool),
        entry!(
            0x009dce20,
            actor_mover_request_rotate_actor(Ptr<ActorMover>, f32, f32, f32, bool) -> bool
        ),
        entry!(
            0x009dce80,
            actor_mover_request_rotate_actor_ov2(Ptr<ActorMover>, f32, bool) -> bool
        ),
        entry!(0x009dd0a0, actor_mover_stop_moving(Ptr<ActorMover>)),
        entry!(0x009dd0c0, actor_mover_force_stop_moving(Ptr<ActorMover>)),
        entry!(0x009dd130, fn_009dd130(Ptr<ActorMover>) -> u32),
        entry!(0x009dd150, actor_mover_do_stop_moving(Ptr<ActorMover>)),
        entry!(0x009dd1c0, actor_mover_finish_moving(Ptr<ActorMover>, bool)),
        entry!(0x009dd2d0, fn_009dd2d0(Ptr<ActorMover>)),
        entry!(0x009ddb30, fn_009ddb30(u32) -> u32),
        entry!(0x009ddb50, actor_mover_is_rotating(Ptr<ActorMover>) -> bool),
        entry!(
            0x009ddb70,
            actor_mover_get_move_mode(Ptr<ActorMover>) -> u32
        ),
        entry!(
            0x009ddbb0,
            actor_mover_get_move_delta(Ptr<ActorMover>, Ptr) -> bool
        ),
        entry!(0x009ddc00, fn_009ddc00(Ptr<ActorMover>)),
        entry!(
            0x009ddf60,
            actor_mover_set_move_mode_preference(Ptr<ActorMover>, u32)
        ),
        entry!(
            0x009de040,
            actor_mover_clear_move_mode_preference(Ptr<ActorMover>, u32)
        ),
        entry!(
            0x009de060,
            actor_mover_force_move_mode(Ptr<ActorMover>, u32)
        ),
        entry!(
            0x009de080,
            actor_mover_clear_forced_move_mode(Ptr<ActorMover>)
        ),
        entry!(0x009de0a0, fn_009de0a0(Ptr<ActorMover>, Ptr) -> Ptr),
        entry!(0x009de110, fn_009de110(Ptr<ActorMover>, u32, u32) -> bool),
        entry!(
            0x009de160,
            actor_mover_set_look_at_target(Ptr<ActorMover>, f32, f32, f32)
        ),
        entry!(
            0x009de230,
            actor_mover_clear_look_at_target(Ptr<ActorMover>)
        ),
        entry!(0x009de270, fn_009de270(Ptr<ActorMover>, Ptr) -> bool),
        entry!(0x009de2b0, fn_009de2b0(Ptr<ActorMover>, Ptr) -> bool),
        entry!(0x009de330, fn_009de330(Ptr<ActorMover>, Ptr) -> bool),
        entry!(0x009de3a0, fn_009de3a0(Ptr<ActorMover>, Ptr)),
        entry!(
            0x009de3e0,
            actor_mover_is_path_valid(Ptr<ActorMover>) -> bool
        ),
        entry!(0x009de420, fn_009de420(Ptr<ActorMover>) -> bool),
        entry!(
            0x009de460,
            actor_mover_get_failed_path_destination(Ptr<ActorMover>) -> u32
        ),
        entry!(
            0x009de490,
            actor_mover_set_pathing_failed(Ptr<ActorMover>, bool)
        ),
        entry!(0x009de8b0, fn_009de8b0(Ptr<ActorMover>)),
        entry!(0x009deff0, fn_009deff0(Ptr, u32)),
        entry!(
            0x009df010,
            actor_mover_move_actor_to_location(Ptr<ActorMover>, Ptr) -> bool
        ),
        entry!(0x009df100, actor_mover_save_game(Ptr<ActorMover>, Ptr)),
        entry!(0x009df3d0, actor_mover_load_game(Ptr<ActorMover>, Ptr)),
        entry!(0x009df8c0, actor_mover_init_load_game(Ptr<ActorMover>, Ptr)),
        entry!(
            0x009dfa00,
            actor_mover_finish_load_game(Ptr<ActorMover>, u32)
        ),
        entry!(0x009dfb80, actor_mover_revert(Ptr<ActorMover>, u32)),
        entry!(0x009dfd50, fn_009dfd50(u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    type Call = (u32, Vec<u32>);

    const ACTOR_TABLE: u32 = 0x0300_0000;
    const ACTOR_FAKE: u32 = 0x0500_0000;
    const HANDLER_TABLE: u32 = 0x0301_0000;
    const HANDLER_FAKE: u32 = 0x0501_0000;
    const PROCESS_TABLE: u32 = 0x0302_0000;
    const PROCESS_FAKE: u32 = 0x0502_0000;
    const REQUEST_TABLE: u32 = 0x0303_0000;
    const REQUEST_FAKE: u32 = 0x0503_0000;
    const MOVER_FAKE: u32 = 0x0504_0000;
    const QUEUE_TABLE: u32 = 0x0305_0000;
    const QUEUE_FAKE: u32 = 0x0505_0000;
    /// A page of scratch memory the tests put data in.
    const SCRATCH: u32 = 0x0600_0000;
    /// The cell holding the pointer the actor's process getter returns.
    const PROCESS_CELL: u32 = SCRATCH + 0xf00;
    /// The position the actor's slot 0x1F4 returns.
    const ACTOR_POSITION: u32 = SCRATCH + 0x200;

    /// Every callee outside this file: each gets a double that returns zero
    /// until a test replaces it.
    const CALLEES: &[u32] = &[
        OPERATOR_NEW,
        OPERATOR_DELETE,
        ALLOCATE,
        DEBUG_STUB,
        FTOL,
        NI_REF_RELEASE,
        NI_REF_ACQUIRE,
        GET_NI_POINTER,
        NI_POINTER_SET,
        QUEUE_POINTER_SET,
        NI_POINTER_CONSTRUCT,
        QUEUE_POINTER_CONSTRUCT,
        QUEUE_POINTER_DESTRUCT,
        NI_POINTER_DESTRUCT,
        QUEUE_POINTER_COPY,
        REQUEST_POINTER_COPY,
        QUEUE_CONSTRUCT,
        LOCATION_CONSTRUCT,
        LOCATION_DESTRUCT,
        LOCATION_FROM_POSITION,
        LOCATION_ASSIGN,
        LOCATION_GET_CELL,
        DWORD_AT_1C,
        LOCATION_CLEAR_TRIANGLE,
        LOCATION_FLAG_2,
        CELL_FLAG_0,
        REQUEST_ORIGIN,
        REQUEST_DESTINATION,
        REQUEST_ALLOW_INCOMPLETE,
        REQUEST_FACE_TARGET_AT_GOAL,
        REQUEST_GOAL_ANGLE,
        REQUEST_GOAL_Z_DELTA,
        REQUEST_TARGET_RADIUS,
        REQUEST_CONSTRUCT,
        REQUEST_DESTRUCT,
        REQUEST_SET_ACTOR,
        REQUEST_INIT_FROM_ACTOR,
        REQUEST_SET_DESTINATION,
        REQUEST_SET_TARGET_RADIUS,
        REQUEST_SET_AVOID_NODES,
        REQUEST_SET_CANT_OPEN_DOORS,
        REQUEST_SET_CAN_SWIM,
        REQUEST_SET_CAN_FLY,
        REQUEST_DEFAULT_RADIUS,
        REQUEST_SET_FLOAT,
        REQUEST_COPY_CONSTRUCT,
        REQUEST_CHECK_PATH,
        REQUEST_CHECK_PATH_FROM,
        PATH_NODE_COUNT,
        PATH_COUNT_20,
        ARRAY_IS_EMPTY,
        PATH_FLAG_40,
        PATH_SECOND_ARRAY_ELEMENT,
        NODE_FLAGS_ARE_3,
        NODE_TEST_ACTOR,
        PATH_RESET_LOADED,
        PATH_NODE_AT,
        NODE_LOCATION,
        PATH_FIRST_LOADED,
        PATH_LAST_NODE,
        PATH_MANAGER_INSTANCE,
        BUILD_PATH,
        GET_PROCESS,
        GET_PARENT_CELL,
        GET_WORLD_SPACE,
        GET_WATER_HEIGHT,
        GET_POSITION,
        GET_ADDRESS_24,
        GET_PROCESS_TYPE,
        GET_DESIRED_PROCESS_LEVEL,
        PROCESS_LEVEL,
        CELL_IS_SIX,
        ACTOR_LEAVE_FURNITURE,
        ACTOR_STOP_MOVEMENT,
        ACTOR_CAN_MOVE,
        ACTOR_MOVEMENT_TEST,
        ACTOR_GET_SPEED,
        ACTOR_GET_ANIM_ACTION,
        ACTOR_TEST_FLAGS,
        ACTOR_SLOT_218,
        ACTOR_FORM_FLAG_10,
        PLAYER_IS_SLEEPING,
        GLOBAL_FLAGS_TEST,
        GLOBAL_TIMER_FLOAT,
        ACTOR_BASE,
        BASE_CAN_SWIM,
        BASE_CAN_FLY,
        ANIMATION_GET_DELTA,
        ANIMATION_ELEMENT,
        ANIMATION_SHOULD_BE_MOVING,
        ENTRY_VALUE,
        PROCESS_DOOR_ID,
        FACE_TARGET_POINT,
        POINT_SUBTRACT,
        POINT_LENGTH_SQUARED_XY,
        POINT_LENGTH,
        FLOAT_ABS,
        FLOAT_ROUND,
        Z_ANGLE_FROM_VECTOR,
        POINT_CONSTRUCT,
        DETAILED_HANDLER_CONSTRUCT_SIMPLE,
        DETAILED_HANDLER_CONSTRUCT,
        VIRTUAL_HANDLER_CONSTRUCT,
        HANDLER_ATTACH,
        REPORT_PATH_END,
        REPORT_TARGET_OBJECT,
        GLOBAL_TEST,
        SET_PATHING_FAILED,
        NAV_MESH_INFO_MAP,
        PACK_GRID_COORDS,
        NAV_MESH_INFO_FIND,
        WATER_HEIGHT_AT,
        ACTOR_PLACE,
        DWORD_AT_18,
        LOCATION_RESET,
        TIMER_NOW,
        IS_FOLLOWING,
        ACTOR_FOLLOW_FALLBACK,
        DIFFERENCE_LENGTH,
        LOCATION_FROM_ACTOR,
        LOCATION_RESOLVE_NAV_MESH,
        LOCATION_IS_RESOLVED,
        LOCATION_NAV_MESH_TEST,
        REQUEST_SET_LOCATION,
        REQUEST_SET_SECOND_FLOAT,
        ACTOR_FLOAT,
        REQUEST_SET_BYTE_A,
        REQUEST_SET_BYTE_B,
        POINT_CONSTRUCT_DEFAULT,
        PATH_CONSTRUCT,
        PATH_DESTRUCT,
        SOLVE_REQUEST,
        PATH_LENGTH,
        PATH_POINT_AT,
        MOVE_REF_TO_NEW_SPACE,
        SET_LOCATION_ON_REFERENCE,
        GET_CELL_FROM_WORLD_COORD,
        REPORT_MOVE_DONE,
        SAVE_BYTES,
        SAVE_FORM_ID,
        LOAD_BYTES,
        LOAD_FORM_ID,
        PATH_SAVE_GAME,
        PATH_LOAD_GAME,
        CREATE_PATHING_REQUEST,
        PROCESS_PART_TEST,
        PRINT_LINE,
        REQUEST_HOLDER,
        SLOT_OBJECT_HOLDER,
        SLOT_OBJECT_VALUE,
        REQUEST_ATTACH,
        ARRAY_ELEMENT_BY_INDEX,
        NODE_TEST_FLAG,
        MESSAGE_CONSTRUCT,
        MESSAGE_DESTRUCT,
        0x006e_5400,
        0x006e_5640,
        0x006e_5940,
        0x006e_6010,
        0x006e_5db0,
        0x006e_4d20,
    ];

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn ret_float(st0: f64) -> Ret {
        Ret {
            st0,
            ..Ret::default()
        }
    }

    /// An object whose vtable (at `table`) points every slot to `fake +
    /// slot offset`; a test registers a double at the offsets it uses.
    fn object(e: &mut Engine, table: u32, fake: u32) -> u32 {
        e.map(table, 0x1000);
        for offset in (0..0x800).step_by(4) {
            e.mem.set_u32(table + offset, fake + offset);
        }
        let object = e.mem.alloc(0x200);
        e.mem.set_u32(object, table);
        object
    }

    /// An engine where every callee is a double returning zero, the data the
    /// code reads is mapped, and the plain accessors read the memory.
    fn world() -> Engine {
        let mut e = Engine::new();
        for &callee in CALLEES {
            e.register(callee, |_, _| Ret::default());
        }
        for page in [
            0x0101_2000,
            0x0101_e000,
            0x011d_d000,
            0x011d_e000,
            0x011f_4000,
            0x0101_1000,
            0x0101_6000,
            0x0101_7000,
            0x0102_9000,
            0x0103_2000,
            0x0109_2000,
            SCRATCH,
            0x0700_0000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(FIFTY, 50.0f64);
        e.set_global(ZERO_DOUBLE, 0.0f64);
        e.register(GET_NI_POINTER, |e, a| ret(e.mem.u32(a[0])));
        let store: AbiFn = |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        };
        e.register(NI_POINTER_SET, store);
        e.register(QUEUE_POINTER_SET, store);
        let allocate: AbiFn = |e, a| ret(e.mem.alloc(a[0]));
        e.register(ALLOCATE, allocate);
        e.register(OPERATOR_NEW, allocate);
        e.register(DWORD_AT_1C, |e, a| ret(e.mem.u32(a[0] + 0x1c)));
        e.register(
            ARRAY_IS_EMPTY,
            |e, a| ret((e.mem.u32(a[0] + 8) == 0) as u32),
        );
        e
    }

    struct Rig {
        e: Engine,
        mover: Ptr<ActorMover>,
        actor: u32,
    }

    /// A mover with an actor object; the mover's vtable (the real address,
    /// which the destructor and the constructor install) points to doubles.
    fn rig() -> Rig {
        let mut e = world();
        let actor = object(&mut e, ACTOR_TABLE, ACTOR_FAKE);
        let slots: Vec<u32> = (0..15).map(|slot| MOVER_FAKE + 4 * slot).collect();
        e.put_vtable(VTABLE, &slots);
        e.register(MOVER_FAKE + 0x10, |_, _| Ret::default());
        let mover: Ptr<ActorMover> = e.new_object();
        e.mem.set_u32(mover.addr(), VTABLE);
        e.set(mover, ActorMover::pActor, Ptr::new(actor));
        Rig { e, mover, actor }
    }

    fn attach_handler(r: &mut Rig) -> u32 {
        let handler = object(&mut r.e, HANDLER_TABLE, HANDLER_FAKE);
        r.e.set(r.mover, ActorMover::pCurrentPathHandler, Ptr::new(handler));
        handler
    }

    /// A path that `fn_009dc530` accepts: a non-empty first array and no
    /// loaded node indexes.
    fn attach_path(r: &mut Rig) -> u32 {
        let path = r.e.mem.alloc(0x48);
        r.e.mem.set_u32(path + 0x10, 1);
        r.e.mem.set_i32(path + 0x18, -1);
        r.e.mem.set_i32(path + 0x1c, -1);
        r.e.mem.set_u32(r.mover.addr() + 0x20, path);
        path
    }

    fn attach_request(r: &mut Rig) -> u32 {
        let request = object(&mut r.e, REQUEST_TABLE, REQUEST_FAKE);
        r.e.mem.set_u32(r.mover.addr() + 0x1c, request);
        request
    }

    /// A process object (virtual slots at 0x610, 0x4bc, 0x40c, 0x6fc) that
    /// the actor's process getter returns.
    fn attach_process(r: &mut Rig) -> u32 {
        let process = object(&mut r.e, PROCESS_TABLE, PROCESS_FAKE);
        r.e.register(GET_PROCESS, |e, _| ret(e.mem.u32(PROCESS_CELL)));
        r.e.mem.set_u32(PROCESS_CELL, process);
        process
    }

    fn calls(e: &mut Engine) -> Vec<Call> {
        e.call_log.take().unwrap()
    }

    fn count(log: &[Call], address: u32) -> usize {
        log.iter().filter(|call| call.0 == address).count()
    }

    fn first(log: &[Call], address: u32) -> &Call {
        log.iter().find(|call| call.0 == address).unwrap()
    }

    /// The addresses of `wanted` that appear in the log, in order.
    fn order(log: &[Call], wanted: &[u32]) -> Vec<u32> {
        log.iter()
            .map(|call| call.0)
            .filter(|address| wanted.contains(address))
            .collect()
    }

    fn put_point(e: &mut Engine, address: u32, x: f32, y: f32, z: f32) {
        e.mem.set_f32(address, x);
        e.mem.set_f32(address + 4, y);
        e.mem.set_f32(address + 8, z);
    }

    fn point(e: &Engine, address: u32) -> [f32; 3] {
        [
            e.mem.f32(address),
            e.mem.f32(address + 4),
            e.mem.f32(address + 8),
        ]
    }

    #[test]
    fn preferred_move_mode_is_the_dword_at_0x34() {
        let mut r = rig();
        r.e.set(r.mover, ActorMover::uiPreferredMoveMode, 0x0201);
        assert_eq!(r.e.call(0x005f_36f0, &args![r.mover]).u32(), 0x0201);
    }

    #[test]
    fn ni_pointer_assign_releases_the_old_and_acquires_the_new() {
        let mut e = world();
        let this = e.mem.alloc(4);
        let source = e.mem.alloc(4);
        e.mem.set_u32(this, 0x1000);
        e.mem.set_u32(source, 0x2000);
        e.call_log = Some(vec![]);
        let back = e.call(0x006e_5cc0, &args![this, source]).u32();
        assert_eq!(back, this);
        assert_eq!(e.mem.u32(this), 0x2000);
        assert_eq!(
            calls(&mut e),
            vec![
                (0x006e_5cc0, vec![this, source]),
                (NI_REF_RELEASE, vec![0x1000]),
                (NI_REF_ACQUIRE, vec![0x2000]),
            ]
        );
    }

    #[test]
    fn ni_pointer_assign_skips_equal_null_old_and_null_new() {
        let mut e = world();
        let this = e.mem.alloc(4);
        let source = e.mem.alloc(4);
        // Equal pointers: nothing happens.
        e.mem.set_u32(this, 0x1000);
        e.mem.set_u32(source, 0x1000);
        e.call_log = Some(vec![]);
        e.call(0x006e_5cc0, &args![this, source]);
        assert_eq!(calls(&mut e).len(), 1);
        // Null old pointer: only the acquire.
        e.mem.set_u32(this, 0);
        e.call_log = Some(vec![]);
        e.call(0x006e_5cc0, &args![this, source]);
        assert_eq!(
            order(&calls(&mut e), &[NI_REF_RELEASE, NI_REF_ACQUIRE]),
            vec![NI_REF_ACQUIRE]
        );
        // Null new pointer: only the release.
        e.mem.set_u32(source, 0);
        e.call_log = Some(vec![]);
        e.call(0x006e_5cc0, &args![this, source]);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(
            order(&calls(&mut e), &[NI_REF_RELEASE, NI_REF_ACQUIRE]),
            vec![NI_REF_RELEASE]
        );
    }

    #[test]
    fn constructor_initialises_every_field() {
        let mut e = world();
        // A signalling NaN, 1.0 and -0.0: copied as plain words.
        e.set_global(UNSET_POINT, 0x7fa0_0001u32);
        e.set_global(UNSET_POINT + 4, 0x3f80_0000u32);
        e.set_global(UNSET_POINT + 8, 0x8000_0000u32);
        let mover: Ptr<ActorMover> = e.new_object();
        for offset in (0..0x88).step_by(4) {
            e.mem.set_u32(mover.addr() + offset, 0xdead_beef);
        }
        e.call_log = Some(vec![]);
        let back = e
            .call(0x009d_ad00, &args![mover, Ptr::<()>::new(0x1234)])
            .ptr::<ActorMover>();
        assert_eq!(back, mover);
        let log = calls(&mut e);
        assert_eq!(e.mem.u32(mover.addr()), VTABLE);
        for base in [4, 0x10] {
            assert_eq!(e.mem.u32(mover.addr() + base), 0x7fa0_0001);
            assert_eq!(e.mem.u32(mover.addr() + base + 4), 0x3f80_0000);
            assert_eq!(e.mem.u32(mover.addr() + base + 8), 0x8000_0000);
        }
        assert_eq!(e.get(mover, ActorMover::pActor).addr(), 0x1234);
        assert_eq!(e.get(mover, ActorMover::eState), 0);
        assert_eq!(e.get(mover, ActorMover::uiPreferredMoveMode), 0x101);
        assert_eq!(e.get(mover, ActorMover::pCurrentPathHandler), Ptr::NULL);
        assert_eq!(e.get(mover, ActorMover::DetectionDoorID), 0);
        assert_eq!(e.get(mover, ActorMover::iPreviousNode), 0xffff);
        assert_eq!(e.get(mover, ActorMover::iNextNode), 0xffff);
        for flag in [
            ActorMover::bActorChangedProcessLevel,
            ActorMover::bHaveValidDeltaThisFrame,
            ActorMover::bStopMovementNextUpdate,
            ActorMover::bWaitingForPath,
            ActorMover::bInDialog,
            ActorMover::bFaceTargetPoint,
            ActorMover::bForceMoveMode,
            ActorMover::bStopCurrentMoveAnim,
        ] {
            assert!(!e.get(mover, flag));
        }
        assert_eq!(e.get(mover, ActorMover::uiForcedMoveMode), 0);
        assert_eq!(e.get(mover, ActorMover::iCachedLastValidMoveMode), 0);
        assert_eq!(e.get(mover, ActorMover::iLastFailureTimeStamp), 0);
        assert_eq!(e.get(mover, ActorMover::iFirstFailureTimeStamp), 0);
        assert_eq!(e.get(mover, ActorMover::iContinuedFailureCount), 0);
        assert_eq!(
            order(
                &log,
                &[
                    NI_POINTER_CONSTRUCT,
                    QUEUE_POINTER_CONSTRUCT,
                    LOCATION_CONSTRUCT,
                    NI_POINTER_SET,
                    QUEUE_POINTER_SET,
                    SET_PATHING_FAILED
                ]
            ),
            vec![
                NI_POINTER_CONSTRUCT,
                NI_POINTER_CONSTRUCT,
                QUEUE_POINTER_CONSTRUCT,
                LOCATION_CONSTRUCT,
                NI_POINTER_SET,
                NI_POINTER_SET,
                QUEUE_POINTER_SET,
                SET_PATHING_FAILED
            ]
        );
        assert_eq!(first(&log, SET_PATHING_FAILED).1, vec![mover.addr(), 0]);
        assert_eq!(first(&log, LOCATION_CONSTRUCT).1, vec![mover.addr() + 0x44]);
    }

    #[test]
    fn scalar_deleting_destructor_frees_only_with_bit_zero() {
        for (flags, freed) in [(1u32, true), (0, false), (3, true), (2, false)] {
            let mut r = rig();
            r.e.call_log = Some(vec![]);
            let back =
                r.e.call(0x009d_aec0, &args![r.mover, flags])
                    .ptr::<ActorMover>();
            assert_eq!(back, r.mover);
            let log = calls(&mut r.e);
            assert_eq!(
                log.iter()
                    .any(|call| call.0 == OPERATOR_DELETE && call.1 == vec![r.mover.addr()]),
                freed
            );
            assert_eq!(count(&log, LOCATION_DESTRUCT), 1);
        }
    }

    #[test]
    fn destructor_clears_the_data_then_destroys_the_members_in_order() {
        let mut r = rig();
        r.e.mem.set_u32(r.mover.addr(), 0);
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_aef0, &args![r.mover]);
        let log = calls(&mut r.e);
        assert_eq!(r.e.mem.u32(r.mover.addr()), VTABLE);
        assert_eq!(r.e.get(r.mover, ActorMover::eState), STATE_FINISHED);
        let base = r.mover.addr();
        let members: Vec<Call> = log
            .iter()
            .filter(|call| {
                [
                    LOCATION_DESTRUCT,
                    QUEUE_POINTER_DESTRUCT,
                    NI_POINTER_DESTRUCT,
                ]
                .contains(&call.0)
            })
            .cloned()
            .collect();
        assert_eq!(
            members,
            vec![
                (LOCATION_DESTRUCT, vec![base + 0x44]),
                (QUEUE_POINTER_DESTRUCT, vec![base + 0x30]),
                (NI_POINTER_DESTRUCT, vec![base + 0x20]),
                (NI_POINTER_DESTRUCT, vec![base + 0x1c]),
            ]
        );
    }

    #[test]
    fn clear_current_data_deletes_the_handler_and_replaces_the_queue() {
        let mut r = rig();
        let handler = attach_handler(&mut r);
        r.e.register(HANDLER_FAKE, |_, _| Ret::default());
        r.e.register(QUEUE_CONSTRUCT, |_, a| ret(a[0] + 1));
        r.e.set(r.mover, ActorMover::bWaitingForPath, true);
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_af80, &args![r.mover]);
        let log = calls(&mut r.e);
        assert_eq!(r.e.get(r.mover, ActorMover::eState), STATE_FINISHED);
        assert_eq!(r.e.get(r.mover, ActorMover::pCurrentPathHandler), Ptr::NULL);
        assert!(!r.e.get(r.mover, ActorMover::bWaitingForPath));
        assert_eq!(
            order(
                &log,
                &[
                    NI_POINTER_SET,
                    HANDLER_FAKE,
                    ALLOCATE,
                    QUEUE_CONSTRUCT,
                    QUEUE_POINTER_SET,
                    MOVER_FAKE + 0x10,
                    DEBUG_STUB
                ]
            ),
            vec![
                NI_POINTER_SET,
                NI_POINTER_SET,
                HANDLER_FAKE,
                ALLOCATE,
                QUEUE_CONSTRUCT,
                QUEUE_POINTER_SET,
                MOVER_FAKE + 0x10,
                DEBUG_STUB
            ]
        );
        assert_eq!(first(&log, HANDLER_FAKE).1, vec![handler, 1]);
        assert_eq!(first(&log, ALLOCATE).1, vec![0x1c]);
        let queue = first(&log, QUEUE_POINTER_SET);
        assert_eq!(queue.1[0], r.mover.addr() + 0x30);
        assert_eq!(r.e.mem.u32(r.mover.addr() + 0x30), queue.1[1]);
    }

    #[test]
    fn clear_current_data_without_handler_or_wait_only_resets() {
        let mut r = rig();
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_af80, &args![r.mover]);
        let log = calls(&mut r.e);
        assert_eq!(count(&log, ALLOCATE), 0);
        assert_eq!(count(&log, QUEUE_POINTER_SET), 0);
        assert_eq!(r.e.get(r.mover, ActorMover::eState), STATE_FINISHED);
    }

    /// Doubles for the part of `009db090` that finishes at once when the
    /// origin and the destination are close: a request object whose
    /// locations hold the positions, with the water far below.
    fn shortcut_world(e: &mut Engine) -> u32 {
        let request = object(e, REQUEST_TABLE, REQUEST_FAKE);
        e.register(REQUEST_FAKE + 0x0c, |_, _| Ret::default());
        e.register(REQUEST_FAKE + 0x10, |_, _| ret(0));
        e.register(REQUEST_ORIGIN, |_, a| ret(a[0] + 0x0c));
        e.register(REQUEST_DESTINATION, |_, a| ret(a[0] + 0x34));
        e.register(GET_POSITION, |e, a| {
            for offset in [0, 4, 8] {
                let word = e.mem.u32(a[0] + 4 + offset);
                e.mem.set_u32(a[1] + offset, word);
            }
            ret(a[1])
        });
        e.register(GET_WATER_HEIGHT, |_, _| ret_float(-100.0));
        e.register(POINT_SUBTRACT, |e, a| {
            for offset in [0, 4, 8] {
                let difference = e.mem.f32(a[0] + offset) - e.mem.f32(a[2] + offset);
                e.mem.set_f32(a[1] + offset, difference);
            }
            ret(a[1])
        });
        e.register(POINT_LENGTH_SQUARED_XY, |e, a| {
            let x = e.mem.f32(a[0]);
            let y = e.mem.f32(a[0] + 4);
            ret_float((x * x + y * y) as f64)
        });
        e.register(REQUEST_TARGET_RADIUS, |_, _| ret_float(1.0));
        e.register(REQUEST_GOAL_Z_DELTA, |_, _| ret_float(1.0));
        e.register(FLOAT_ABS, |_, a| {
            ret_float(f32::from_bits(a[0]).abs() as f64)
        });
        request
    }

    #[test]
    fn start_pathing_finishes_at_once_when_the_goal_is_here() {
        let mut r = rig();
        let request = shortcut_world(&mut r.e);
        put_point(&mut r.e, request + 0x10, 1.0, 2.0, 100.0);
        put_point(&mut r.e, request + 0x38, 1.5, 2.0, 100.0);
        r.e.call_log = Some(vec![]);
        let result =
            r.e.call(0x009d_b090, &args![r.mover, Ptr::<()>::new(request)])
                .bool();
        assert!(result);
        let log = calls(&mut r.e);
        assert_eq!(r.e.get(r.mover, ActorMover::eState), STATE_FINISHED);
        assert_eq!(count(&log, BUILD_PATH), 0);
        assert_eq!(count(&log, ALLOCATE), 0);
        assert_eq!(count(&log, REQUEST_GOAL_ANGLE), 0);
        // The request is checked first (virtual slot 0x0c).
        assert_eq!(log[1], (REQUEST_FAKE + 0x0c, vec![request]));
    }

    #[test]
    fn start_pathing_rotates_to_the_goal_angle_when_asked() {
        let mut r = rig();
        let request = shortcut_world(&mut r.e);
        r.e.register(REQUEST_FACE_TARGET_AT_GOAL, |_, _| ret(1));
        r.e.register(REQUEST_GOAL_ANGLE, |_, _| ret_float(1.25));
        // The rotation: the actor cannot move, so it finishes at once.
        r.e.register(ACTOR_CAN_MOVE, |_, _| ret(0));
        r.e.register(ACTOR_FAKE + 0x1e4, |_, _| ret(0));
        r.e.call_log = Some(vec![]);
        let result =
            r.e.call(0x009d_b090, &args![r.mover, Ptr::<()>::new(request)])
                .bool();
        assert!(result);
        let log = calls(&mut r.e);
        assert_eq!(count(&log, REQUEST_GOAL_ANGLE), 1);
        assert_eq!(count(&log, ACTOR_CAN_MOVE), 1);
        assert_eq!(r.e.get(r.mover, ActorMover::eState), STATE_FINISHED);
    }

    #[test]
    fn start_pathing_far_goal_goes_to_the_path_manager() {
        for built in [true, false] {
            let mut r = rig();
            let request = shortcut_world(&mut r.e);
            // The goal is 5 units away with a target radius of 1.
            put_point(&mut r.e, request + 0x38, 3.0, 4.0, 100.0);
            r.e.register(REQUEST_FAKE + 0x10, |_, _| ret(0));
            r.e.register(REQUEST_COPY_CONSTRUCT, |_, a| ret(a[0]));
            r.e.register(PATH_MANAGER_INSTANCE, |_, _| ret(0x77));
            r.e.register(QUEUE_CONSTRUCT, |_, a| ret(a[0]));
            let copy: AbiFn = |e, a| {
                let value = e.mem.u32(a[1]);
                e.mem.set_u32(a[0], value);
                ret(a[0])
            };
            r.e.register(QUEUE_POINTER_COPY, copy);
            r.e.register(REQUEST_POINTER_COPY, copy);
            if built {
                r.e.register(BUILD_PATH, |_, _| ret(1));
            }
            r.e.set(r.mover, ActorMover::iContinuedFailureCount, 4);
            r.e.call_log = Some(vec![]);
            let result =
                r.e.call(0x009d_b090, &args![r.mover, Ptr::<()>::new(request)])
                    .bool();
            let log = calls(&mut r.e);
            assert_eq!(result, built);
            let held = first(&log, REQUEST_SET_ACTOR).1[0];
            assert_ne!(held, 0);
            assert_eq!(r.e.mem.u32(held + 0xa8), 7);
            assert_eq!(first(&log, ALLOCATE).1, vec![0xb0]);
            let queue = r.e.mem.u32(r.mover.addr() + 0x30);
            assert_ne!(queue, 0);
            assert_eq!(first(&log, BUILD_PATH).1, vec![0x77, held, queue, 1]);
            assert_eq!(first(&log, REQUEST_SET_ACTOR).1, vec![held, r.actor]);
            assert_eq!(count(&log, LOCATION_CLEAR_TRIANGLE), 2);
            if built {
                assert!(r.e.get(r.mover, ActorMover::bWaitingForPath));
                assert_eq!(r.e.get(r.mover, ActorMover::eState), STATE_INIT);
            } else {
                assert!(!r.e.get(r.mover, ActorMover::bWaitingForPath));
                assert_eq!(r.e.get(r.mover, ActorMover::eState), STATE_FAILED);
                assert_eq!(log.last().unwrap().0, SET_PATHING_FAILED);
                assert_eq!(log.last().unwrap().1, vec![r.mover.addr(), 1]);
            }
        }
    }

    #[test]
    fn start_pathing_builds_the_request_class_of_the_request_type() {
        for (kind, size, constructor) in [
            (3u32, 0xb4u32, 0x006e_5400u32),
            (4, 0xdc, 0x006e_5640),
            (5, 0xd8, 0x006e_5940),
            (6, 0xbc, 0x006e_6010),
            (7, 0xc0, 0x006e_5db0),
            (8, 0x10c, 0x006e_4d20),
        ] {
            let mut r = rig();
            let request = shortcut_world(&mut r.e);
            put_point(&mut r.e, request + 0x38, 30.0, 40.0, 100.0);
            r.e.register(REQUEST_FAKE + 0x10, |_, _| ret(0));
            r.e.register(constructor, |_, a| ret(a[0]));
            r.e.register(PATH_MANAGER_INSTANCE, |_, _| ret(0x77));
            r.e.register(QUEUE_CONSTRUCT, |_, a| ret(a[0]));
            r.e.register(BUILD_PATH, |_, _| ret(1));
            // The type is read through the request's virtual slot 0x10.
            let kind_slot: AbiFn = match kind {
                3 => |_, _| ret(3),
                4 => |_, _| ret(4),
                5 => |_, _| ret(5),
                6 => |_, _| ret(6),
                7 => |_, _| ret(7),
                _ => |_, _| ret(8),
            };
            r.e.register(REQUEST_FAKE + 0x10, kind_slot);
            r.e.call_log = Some(vec![]);
            r.e.call(0x009d_b090, &args![r.mover, Ptr::<()>::new(request)]);
            let log = calls(&mut r.e);
            assert_eq!(first(&log, ALLOCATE).1, vec![size]);
            let object = r.e.mem.u32(r.mover.addr() + 0x1c);
            assert_eq!(first(&log, constructor).1, vec![object, request]);
        }
    }

    #[test]
    fn start_pathing_hands_the_request_to_an_existing_handler() {
        for detailed in [true, false] {
            let mut r = rig();
            let request = shortcut_world(&mut r.e);
            put_point(&mut r.e, request + 0x38, 30.0, 40.0, 100.0);
            r.e.register(REQUEST_FAKE + 0x10, |_, _| ret(0));
            r.e.register(REQUEST_COPY_CONSTRUCT, |_, a| ret(a[0]));
            r.e.register(PATH_MANAGER_INSTANCE, |_, _| ret(0x77));
            r.e.register(QUEUE_CONSTRUCT, |_, a| ret(a[0]));
            r.e.register(BUILD_PATH, |_, _| ret(1));
            let handler = attach_handler(&mut r);
            r.e.register(HANDLER_FAKE + 0x38, |_, _| Ret::default());
            let slot: AbiFn = if detailed {
                |_, _| ret(1)
            } else {
                |_, _| ret(0)
            };
            r.e.register(HANDLER_FAKE + 0x34, slot);
            r.e.register(GET_PARENT_CELL, |_, _| ret(0));
            r.e.call_log = Some(vec![]);
            r.e.call(0x009d_b090, &args![r.mover, Ptr::<()>::new(request)]);
            let log = calls(&mut r.e);
            let held = r.e.mem.u32(r.mover.addr() + 0x1c);
            assert_eq!(first(&log, HANDLER_FAKE + 0x38).1, vec![handler, held]);
            assert_eq!(count(&log, HANDLER_ATTACH), detailed as usize);
            if detailed {
                assert_eq!(first(&log, HANDLER_ATTACH).1, vec![r.actor, held, handler]);
            }
        }
    }

    #[test]
    fn start_pathing_without_handler_attaches_for_a_cell_that_passes() {
        let mut r = rig();
        let request = shortcut_world(&mut r.e);
        put_point(&mut r.e, request + 0x38, 30.0, 40.0, 100.0);
        r.e.register(REQUEST_FAKE + 0x10, |_, _| ret(0));
        r.e.register(REQUEST_COPY_CONSTRUCT, |_, a| ret(a[0]));
        r.e.register(PATH_MANAGER_INSTANCE, |_, _| ret(0x77));
        r.e.register(QUEUE_CONSTRUCT, |_, a| ret(a[0]));
        r.e.register(BUILD_PATH, |_, _| ret(1));
        r.e.register(GET_PARENT_CELL, |_, _| ret(0x4444));
        r.e.register(CELL_IS_SIX, |_, _| ret(1));
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_b090, &args![r.mover, Ptr::<()>::new(request)]);
        let log = calls(&mut r.e);
        let held = r.e.mem.u32(r.mover.addr() + 0x1c);
        assert_eq!(first(&log, HANDLER_ATTACH).1, vec![r.actor, held, 0]);
        assert_eq!(first(&log, CELL_IS_SIX).1, vec![0x4444]);
    }

    #[test]
    fn start_pathing_passes_the_sleep_state_to_the_path_manager() {
        let mut r = rig();
        let request = shortcut_world(&mut r.e);
        put_point(&mut r.e, request + 0x38, 30.0, 40.0, 100.0);
        r.e.register(REQUEST_COPY_CONSTRUCT, |_, a| ret(a[0]));
        r.e.register(PATH_MANAGER_INSTANCE, |_, _| ret(0x77));
        r.e.register(QUEUE_CONSTRUCT, |_, a| ret(a[0]));
        r.e.register(BUILD_PATH, |_, _| ret(1));
        r.e.register(PLAYER_IS_SLEEPING, |_, _| ret(1));
        r.e.set_global(PLAYER_CHARACTER, 0x5555u32);
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_b090, &args![r.mover, Ptr::<()>::new(request)]);
        let log = calls(&mut r.e);
        assert_eq!(first(&log, PLAYER_IS_SLEEPING).1, vec![0x5555]);
        assert_eq!(first(&log, BUILD_PATH).1[3], 0);
    }

    #[test]
    fn retry_count_is_stored_at_0xa8() {
        let mut e = world();
        let request = e.mem.alloc(0xb0);
        e.call(0x009d_b930, &args![request, 9u32]);
        assert_eq!(e.mem.u32(request + 0xa8), 9);
    }

    #[test]
    fn clear_data_for_new_request_waits_and_clears_only_on_demand() {
        // 009db950: only the new queue (009db9a2) and the end of the wait (009db9f4) are
        // conditional; the path drop (009dba05) and SetPathingFailed(0) (009dba0f) always run.
        for (waiting, clear_waiting) in [(false, false), (false, true), (true, false), (true, true)]
        {
            let mut r = rig();
            attach_path(&mut r);
            attach_request(&mut r);
            r.e.register(QUEUE_CONSTRUCT, |_, a| ret(a[0]));
            r.e.set(r.mover, ActorMover::bStopMovementNextUpdate, true);
            r.e.set(r.mover, ActorMover::bWaitingForPath, waiting);
            r.e.call_log = Some(vec![]);
            r.e.call(0x009d_b950, &args![r.mover, clear_waiting]);
            let log = calls(&mut r.e);
            assert!(!r.e.get(r.mover, ActorMover::bStopMovementNextUpdate));
            assert_eq!(r.e.mem.u32(r.mover.addr() + 0x1c), 0);
            assert_eq!(count(&log, MOVER_FAKE + 0x10), 1);
            assert_eq!(count(&log, QUEUE_POINTER_SET), waiting as usize);
            assert_eq!(count(&log, SET_PATHING_FAILED), 1);
            assert_eq!(
                r.e.get(r.mover, ActorMover::bWaitingForPath),
                waiting && !clear_waiting
            );
            assert_eq!(r.e.mem.u32(r.mover.addr() + 0x20), 0);
        }
    }

    const BASE_BLOCK: u32 = 0x0700_0000;
    const BASE_TABLE: u32 = 0x0306_0000;
    const BASE_FAKE: u32 = 0x0506_0000;

    fn p(address: u32) -> Ptr {
        Ptr::new(address)
    }

    /// The doubles `SetGoal` and its relatives need on top of the
    /// shortcut world: the request built on the stack gets a vtable and
    /// the actor's base object answers the door question.
    fn goal_world(r: &mut Rig) {
        shortcut_world(&mut r.e);
        r.e.register(REQUEST_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], REQUEST_TABLE);
            ret(a[0])
        });
        r.e.register(LOCATION_FROM_POSITION, |_, a| ret(a[0]));
        r.e.register(REQUEST_DEFAULT_RADIUS, |_, _| ret_float(2.5));
        r.e.register(ACTOR_FAKE + 0x21c, |_, _| ret(0));
        r.e.register(ACTOR_FAKE + 0x1f4, |_, _| ret(ACTOR_POSITION));
        r.e.register(ACTOR_BASE, |_, _| ret(BASE_BLOCK));
        r.e.mem.set_u32(BASE_BLOCK + 0x30, BASE_TABLE);
        r.e.map(BASE_TABLE, 0x1000);
        r.e.mem.set_u32(BASE_TABLE + 0x28, BASE_FAKE + 0x28);
        r.e.register(BASE_FAKE + 0x28, |_, _| ret(1));
        r.e.register(BASE_CAN_SWIM, |_, _| ret(1));
        r.e.register(BASE_CAN_FLY, |_, _| ret(0));
    }

    #[test]
    fn set_goal_builds_a_request_and_starts_pathing() {
        let mut r = rig();
        goal_world(&mut r);
        r.e.call_log = Some(vec![]);
        let started =
            r.e.call(
                0x009d_ba50,
                &args![r.mover, p(0x111), p(0x222), p(0x333), 2.0f32, p(0x444)],
            )
            .bool();
        assert!(started);
        let log = calls(&mut r.e);
        let request = first(&log, REQUEST_CONSTRUCT).1[0];
        let location = first(&log, LOCATION_FROM_POSITION).1[0];
        assert_eq!(
            first(&log, LOCATION_FROM_POSITION).1,
            vec![location, 0x111, 0x222, 0x333]
        );
        assert_eq!(
            first(&log, REQUEST_INIT_FROM_ACTOR).1,
            vec![request, r.actor]
        );
        assert_eq!(
            first(&log, REQUEST_SET_DESTINATION).1,
            vec![request, location]
        );
        assert_eq!(
            first(&log, REQUEST_SET_TARGET_RADIUS).1,
            vec![request, 2.0f32.to_bits()]
        );
        assert_eq!(first(&log, REQUEST_SET_AVOID_NODES).1, vec![request, 0x444]);
        assert_eq!(first(&log, LOCATION_DESTRUCT).1, vec![location]);
        assert_eq!(first(&log, REQUEST_DESTRUCT).1, vec![request]);
        assert_eq!(
            order(
                &log,
                &[REQUEST_CONSTRUCT, REQUEST_INIT_FROM_ACTOR, REQUEST_DESTRUCT]
            ),
            vec![REQUEST_CONSTRUCT, REQUEST_INIT_FROM_ACTOR, REQUEST_DESTRUCT]
        );
    }

    #[test]
    fn set_goal_with_a_forwarding_wrapper_returns_the_pathing_result() {
        let mut r = rig();
        let request = shortcut_world(&mut r.e);
        r.e.call_log = Some(vec![]);
        assert!(r.e.call(0x009d_ba30, &args![r.mover, p(request)]).bool());
        assert_eq!(
            first(&calls(&mut r.e), 0x009d_ba30).1,
            vec![r.mover.addr(), request]
        );
    }

    #[test]
    fn goal_at_a_reference_uses_its_position_cell_and_world_space() {
        let mut r = rig();
        goal_world(&mut r);
        r.e.register(GET_WORLD_SPACE, |_, _| ret(0xaa));
        r.e.register(GET_PARENT_CELL, |_, _| ret(0xbb));
        r.e.call_log = Some(vec![]);
        let started =
            r.e.call(0x009d_bb10, &args![r.mover, p(r.actor), 3.0f32, p(0x444)])
                .bool();
        assert!(started);
        let log = calls(&mut r.e);
        let location = first(&log, LOCATION_FROM_POSITION).1[0];
        assert_eq!(
            first(&log, LOCATION_FROM_POSITION).1,
            vec![location, ACTOR_POSITION, 0xbb, 0xaa]
        );
        assert_eq!(
            order(
                &log,
                &[GET_WORLD_SPACE, GET_PARENT_CELL, ACTOR_FAKE + 0x1f4]
            ),
            vec![GET_WORLD_SPACE, GET_PARENT_CELL, ACTOR_FAKE + 0x1f4]
        );
        let request = first(&log, REQUEST_CONSTRUCT).1[0];
        assert_eq!(
            first(&log, REQUEST_SET_TARGET_RADIUS).1,
            vec![request, 3.0f32.to_bits()]
        );
    }

    #[test]
    fn goal_with_a_request_float_passes_it_to_the_request() {
        let mut r = rig();
        goal_world(&mut r);
        r.e.call_log = Some(vec![]);
        let started =
            r.e.call(
                0x009d_bb60,
                &args![
                    r.mover,
                    p(0x111),
                    p(0x222),
                    p(0x333),
                    1.5f32,
                    2.5f32,
                    p(0x444)
                ],
            )
            .bool();
        assert!(started);
        let log = calls(&mut r.e);
        let request = first(&log, REQUEST_CONSTRUCT).1[0];
        assert_eq!(
            first(&log, REQUEST_SET_TARGET_RADIUS).1,
            vec![request, 2.5f32.to_bits()]
        );
        assert_eq!(
            first(&log, REQUEST_SET_FLOAT).1,
            vec![request, 1.5f32.to_bits()]
        );
        assert_eq!(
            order(
                &log,
                &[REQUEST_SET_AVOID_NODES, REQUEST_SET_FLOAT, REQUEST_DESTRUCT]
            ),
            vec![REQUEST_SET_AVOID_NODES, REQUEST_SET_FLOAT, REQUEST_DESTRUCT]
        );
    }

    #[test]
    fn goal_at_a_reference_reads_the_float_at_offset_0x2c() {
        let mut r = rig();
        goal_world(&mut r);
        r.e.register(GET_ADDRESS_24, |_, _| ret(SCRATCH + 0x300));
        r.e.mem.set_f32(SCRATCH + 0x308, 7.5);
        r.e.call_log = Some(vec![]);
        let started =
            r.e.call(0x009d_bc30, &args![r.mover, p(r.actor), 3.0f32, p(0x444)])
                .bool();
        assert!(started);
        let log = calls(&mut r.e);
        let request = first(&log, REQUEST_CONSTRUCT).1[0];
        assert_eq!(
            first(&log, REQUEST_SET_FLOAT).1,
            vec![request, 7.5f32.to_bits()]
        );
        assert_eq!(
            first(&log, REQUEST_SET_TARGET_RADIUS).1,
            vec![request, 3.0f32.to_bits()]
        );
    }

    #[test]
    fn fill_request_uses_the_default_radius_unless_the_radius_is_positive() {
        for (given, expected) in [
            (3.5f32.to_bits(), 3.5f32.to_bits()),
            (0.0f32.to_bits(), 2.5f32.to_bits()),
            ((-1.0f32).to_bits(), 2.5f32.to_bits()),
            (0x7fc0_0000, 0x7fc0_0000),
            // A signalling NaN is quieted by the FLD/FSTP copy.
            (0x7fa0_0001, 0x7fe0_0001),
        ] {
            let mut r = rig();
            goal_world(&mut r);
            r.e.call_log = Some(vec![]);
            r.e.call(
                0x009d_bc90,
                &args![
                    r.actor,
                    0x5000u32,
                    0x111u32,
                    0x222u32,
                    0x333u32,
                    f32::from_bits(given),
                    0x444u32
                ],
            );
            let log = calls(&mut r.e);
            assert_eq!(
                first(&log, REQUEST_SET_TARGET_RADIUS).1,
                vec![0x5000, expected]
            );
        }
    }

    #[test]
    fn fill_request_sets_the_actor_abilities() {
        for opens_doors in [false, true] {
            let mut r = rig();
            goal_world(&mut r);
            let slot: AbiFn = if opens_doors {
                |_, _| ret(1)
            } else {
                |_, _| ret(0)
            };
            r.e.register(ACTOR_FAKE + 0x21c, slot);
            r.e.call_log = Some(vec![]);
            r.e.call(
                0x009d_bc90,
                &args![r.actor, 0x5000u32, 0x111u32, 0x222u32, 0x333u32, 1.0f32, 0x444u32],
            );
            let log = calls(&mut r.e);
            assert_eq!(
                count(&log, REQUEST_SET_CANT_OPEN_DOORS),
                opens_doors as usize
            );
            if opens_doors {
                assert_eq!(first(&log, REQUEST_SET_CANT_OPEN_DOORS).1, vec![0x5000, 1]);
            }
            assert_eq!(first(&log, REQUEST_SET_CAN_SWIM).1, vec![0x5000, 1]);
            assert_eq!(first(&log, REQUEST_SET_CAN_FLY).1, vec![0x5000, 0]);
            assert_eq!(first(&log, BASE_CAN_SWIM).1, vec![BASE_BLOCK]);
            let location = first(&log, LOCATION_FROM_POSITION).1[0];
            assert_eq!(first(&log, LOCATION_DESTRUCT).1, vec![location]);
        }
    }

    /// The request's tail for the tests that take over a path: a mover with
    /// a request, a path and a handler.
    fn takeover_rig() -> (Rig, u32) {
        let mut r = rig();
        attach_request(&mut r);
        attach_path(&mut r);
        let previous = attach_handler(&mut r);
        r.e.register(HANDLER_FAKE, |_, _| Ret::default());
        r.e.register(HANDLER_FAKE + 0x04, |_, _| ret(0));
        r.e.register(HANDLER_FAKE + 0x58, |_, _| Ret::default());
        r.e.register(OPERATOR_NEW, |e, a| {
            let block = e.mem.alloc(a[0]);
            e.mem.set_u32(block, HANDLER_TABLE);
            ret(block)
        });
        for constructor in [
            DETAILED_HANDLER_CONSTRUCT,
            VIRTUAL_HANDLER_CONSTRUCT,
            DETAILED_HANDLER_CONSTRUCT_SIMPLE,
        ] {
            r.e.register(constructor, |_, a| ret(a[0]));
        }
        (r, previous)
    }

    #[test]
    fn take_over_a_path_with_a_virtual_handler() {
        let (mut r, previous) = takeover_rig();
        r.e.register(GET_PROCESS_TYPE, |_, _| ret(1));
        r.e.register(PATH_NODE_COUNT, |_, _| ret(4));
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_bdc0, &args![r.mover, 0.25f32]);
        let log = calls(&mut r.e);
        let handler = r.e.get(r.mover, ActorMover::pCurrentPathHandler).addr();
        assert_ne!(handler, previous);
        assert_eq!(r.e.get(r.mover, ActorMover::iPreviousNode), 0);
        assert_eq!(r.e.get(r.mover, ActorMover::iNextNode), 3);
        let request = r.e.mem.u32(r.mover.addr() + 0x1c);
        assert_eq!(
            first(&log, VIRTUAL_HANDLER_CONSTRUCT).1,
            vec![
                handler,
                r.actor,
                r.mover.addr(),
                r.mover.addr() + 0x20,
                request
            ]
        );
        assert_eq!(first(&log, OPERATOR_NEW).1, vec![0x20]);
        // The previous handler is deleted before the new one is built, and
        // the new one is started with (0.0, delta time).
        assert_eq!(first(&log, HANDLER_FAKE).1, vec![previous, 1]);
        assert_eq!(
            order(&log, &[HANDLER_FAKE, OPERATOR_NEW, HANDLER_FAKE + 0x04]),
            vec![HANDLER_FAKE, OPERATOR_NEW, HANDLER_FAKE + 0x04]
        );
        assert_eq!(
            first(&log, HANDLER_FAKE + 0x04).1,
            vec![handler, 0.0f32.to_bits(), 0.25f32.to_bits()]
        );
        // No check against the path for a count of zero and a non-zero type.
        assert_eq!(count(&log, REQUEST_CHECK_PATH), 0);
        assert_eq!(count(&log, REQUEST_CHECK_PATH_FROM), 0);
    }

    #[test]
    fn take_over_a_path_with_a_detailed_handler() {
        let (mut r, previous) = takeover_rig();
        let path = r.e.mem.u32(r.mover.addr() + 0x20);
        r.e.mem.set_u32(path + 0x1c, 9);
        r.e.register(PATH_COUNT_20, |_, _| ret(2));
        r.e.register(GET_PROCESS_TYPE, |_, _| ret(0));
        r.e.register(PATH_FIRST_LOADED, |_, _| ret(0x0001_0007));
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_bdc0, &args![r.mover, 0.25f32]);
        let log = calls(&mut r.e);
        let handler = r.e.get(r.mover, ActorMover::pCurrentPathHandler).addr();
        assert_eq!(r.e.get(r.mover, ActorMover::iPreviousNode), 7);
        assert_eq!(r.e.get(r.mover, ActorMover::iNextNode), 9);
        let request = r.e.mem.u32(r.mover.addr() + 0x1c);
        assert_eq!(
            first(&log, DETAILED_HANDLER_CONSTRUCT).1,
            vec![
                handler,
                r.actor,
                r.mover.addr(),
                r.mover.addr() + 0x20,
                request
            ]
        );
        assert_eq!(first(&log, OPERATOR_NEW).1, vec![0xe4]);
        // The new handler is told about the previous one, which is deleted
        // afterwards.
        assert_eq!(
            order(
                &log,
                &[
                    OPERATOR_NEW,
                    HANDLER_FAKE + 0x58,
                    HANDLER_FAKE,
                    HANDLER_FAKE + 0x04
                ]
            ),
            vec![
                OPERATOR_NEW,
                HANDLER_FAKE + 0x58,
                HANDLER_FAKE,
                HANDLER_FAKE + 0x04
            ]
        );
        assert_eq!(first(&log, HANDLER_FAKE + 0x58).1, vec![handler, previous]);
        assert_eq!(first(&log, HANDLER_FAKE).1, vec![previous, 1]);
        assert_eq!(count(&log, REQUEST_CHECK_PATH), 0);
    }

    #[test]
    fn take_over_a_path_fails_when_the_request_check_says_no() {
        for verdict in [0xffff_ffffu32, 0] {
            let (mut r, previous) = takeover_rig();
            r.e.register(PATH_COUNT_20, |_, _| ret(2));
            r.e.register(GET_PROCESS_TYPE, |_, _| ret(1));
            let check: AbiFn = if verdict == 0 {
                |_, _| ret(0)
            } else {
                |_, _| ret(0xffff_ffff)
            };
            r.e.register(REQUEST_CHECK_PATH, check);
            r.e.call_log = Some(vec![]);
            r.e.call(0x009d_bdc0, &args![r.mover, 0.25f32]);
            let log = calls(&mut r.e);
            assert_eq!(r.e.get(r.mover, ActorMover::eState), STATE_FAILED);
            assert_eq!(
                log.last().unwrap(),
                &(SET_PATHING_FAILED, vec![r.mover.addr(), 1])
            );
            assert_eq!(count(&log, OPERATOR_NEW), 0);
            assert_eq!(
                r.e.get(r.mover, ActorMover::pCurrentPathHandler).addr(),
                previous
            );
        }
    }

    #[test]
    fn take_over_an_empty_path_stops_the_actor_when_the_check_from_the_start_fails() {
        let (mut r, _) = takeover_rig();
        r.e.register(GET_PROCESS_TYPE, |_, _| ret(0));
        r.e.register(REQUEST_CHECK_PATH_FROM, |_, _| ret(0));
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_bdc0, &args![r.mover, 0.25f32]);
        let log = calls(&mut r.e);
        assert_eq!(first(&log, REQUEST_CHECK_PATH_FROM).1[2], 0);
        assert_eq!(first(&log, ACTOR_STOP_MOVEMENT).1, vec![r.actor, 1]);
        assert_eq!(r.e.get(r.mover, ActorMover::eState), STATE_FAILED);
        assert_eq!(count(&log, OPERATOR_NEW), 0);
    }

    #[test]
    fn take_over_an_empty_path_with_a_process_type_builds_a_virtual_handler() {
        let (mut r, _) = takeover_rig();
        r.e.register(GET_PROCESS_TYPE, |_, _| ret(2));
        r.e.register(PATH_NODE_COUNT, |_, _| ret(1));
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_bdc0, &args![r.mover, 0.25f32]);
        let log = calls(&mut r.e);
        assert_eq!(count(&log, VIRTUAL_HANDLER_CONSTRUCT), 1);
        assert_eq!(r.e.get(r.mover, ActorMover::iNextNode), 0);
    }

    #[test]
    fn result_codes_one_to_four_fail_the_mover() {
        for code in 1..=4u32 {
            let mut r = rig();
            attach_handler(&mut r);
            r.e.register(HANDLER_FAKE + 0x34, |_, _| ret(0));
            r.e.call_log = Some(vec![]);
            r.e.call(0x009d_c5e0, &args![r.mover, code]);
            let log = calls(&mut r.e);
            assert_eq!(
                order(&log, &[SET_PATHING_FAILED]).len(),
                2,
                "do-stop resets, then the failure is recorded"
            );
            assert_eq!(
                log.last().unwrap(),
                &(SET_PATHING_FAILED, vec![r.mover.addr(), 1])
            );
            assert_eq!(r.e.get(r.mover, ActorMover::eState), STATE_FINISHED);
            assert!(r.e.get(r.mover, ActorMover::bStopMovementNextUpdate));
        }
    }

    #[test]
    fn other_result_codes_do_nothing() {
        for code in [0u32, 7, 100] {
            let mut r = rig();
            r.e.call_log = Some(vec![]);
            r.e.call(0x009d_c5e0, &args![r.mover, code]);
            assert_eq!(calls(&mut r.e).len(), 1);
        }
    }

    #[test]
    fn result_five_without_a_next_node_fails_and_stops() {
        let mut r = rig();
        attach_path(&mut r);
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_c5e0, &args![r.mover, 5u32]);
        let log = calls(&mut r.e);
        // `fn_009dc980` fails (no next node): state 7, then stops moving.
        assert_eq!(first(&log, SET_PATHING_FAILED).1, vec![r.mover.addr(), 1]);
        // Back in the caller the path flag (+0x40) is clear: do-stop again.
        assert_eq!(count(&log, PATH_FLAG_40), 1);
        assert!(r.e.get(r.mover, ActorMover::bStopMovementNextUpdate));
    }

    #[test]
    fn result_six_without_the_path_flag_keeps_moving_state() {
        let mut r = rig();
        attach_path(&mut r);
        attach_handler(&mut r);
        r.e.register(HANDLER_FAKE + 0x34, |_, _| ret(1));
        r.e.register(PATH_NODE_COUNT, |_, _| ret(5));
        r.e.set(r.mover, ActorMover::iNextNode, 2);
        r.e.set(r.mover, ActorMover::eState, STATE_MOVING);
        r.e.call(0x009d_c5e0, &args![r.mover, 6u32]);
        assert_eq!(r.e.get(r.mover, ActorMover::eState), STATE_MOVING);
        assert_eq!(r.e.get(r.mover, ActorMover::iPreviousNode), 2);
        // A failed mover stays failed.
        r.e.set(r.mover, ActorMover::eState, STATE_FAILED);
        r.e.call(0x009d_c5e0, &args![r.mover, 6u32]);
        assert_eq!(r.e.get(r.mover, ActorMover::eState), STATE_FAILED);
    }

    #[test]
    fn result_five_with_an_empty_second_array_fails_the_mover() {
        let mut r = rig();
        attach_path(&mut r);
        attach_request(&mut r);
        attach_handler(&mut r);
        r.e.register(HANDLER_FAKE, |_, _| Ret::default());
        r.e.register(HANDLER_FAKE + 0x34, |_, _| ret(1));
        r.e.register(PATH_NODE_COUNT, |_, _| ret(5));
        r.e.register(PATH_FLAG_40, |_, _| ret(1));
        r.e.set(r.mover, ActorMover::iNextNode, 2);
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_c5e0, &args![r.mover, 5u32]);
        let log = calls(&mut r.e);
        assert_eq!(first(&log, SET_PATHING_FAILED).1, vec![r.mover.addr(), 1]);
        // `ClearCurrentData` ran: state finished, path and request dropped.
        assert_eq!(r.e.get(r.mover, ActorMover::eState), STATE_FINISHED);
        assert_eq!(r.e.mem.u32(r.mover.addr() + 0x20), 0);
    }

    /// A path with two nodes in its second array whose last loaded node is
    /// `distance` units east of the actor, and a target radius of `radius`.
    fn far_path_rig(distance: f32, radius: f64) -> Rig {
        let mut r = rig();
        let request = shortcut_world(&mut r.e);
        r.e.mem.set_u32(r.mover.addr() + 0x1c, request);
        attach_path(&mut r);
        attach_handler(&mut r);
        r.e.register(HANDLER_FAKE, |_, _| Ret::default());
        r.e.register(HANDLER_FAKE + 0x34, |_, _| ret(1));
        r.e.register(PATH_NODE_COUNT, |_, _| ret(5));
        r.e.register(PATH_FLAG_40, |_, _| ret(1));
        r.e.register(PATH_COUNT_20, |_, _| ret(2));
        r.e.register(PATH_NODE_AT, |_, _| ret(SCRATCH + 0x500));
        r.e.register(NODE_LOCATION, |_, a| ret(a[0] + 4));
        r.e.register(ACTOR_FAKE + 0x1f4, |_, _| ret(ACTOR_POSITION));
        put_point(&mut r.e, ACTOR_POSITION, 0.0, 0.0, 0.0);
        // `NODE_LOCATION` returns node + 4, `GET_POSITION` reads +4 more.
        put_point(&mut r.e, SCRATCH + 0x508, distance, 0.0, 0.0);
        let radius_slot: AbiFn = if radius == 4.0 {
            |_, _| ret_float(4.0)
        } else {
            |_, _| ret_float(11.0)
        };
        r.e.register(REQUEST_TARGET_RADIUS, radius_slot);
        r.e.set(r.mover, ActorMover::iNextNode, 2);
        r
    }

    #[test]
    fn result_five_fails_when_the_path_end_is_farther_than_the_radius() {
        let mut r = far_path_rig(10.0, 4.0);
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_c5e0, &args![r.mover, 5u32]);
        let log = calls(&mut r.e);
        assert_eq!(first(&log, SET_PATHING_FAILED).1, vec![r.mover.addr(), 1]);
        assert_eq!(r.e.get(r.mover, ActorMover::eState), STATE_FINISHED);
        assert_eq!(count(&log, REQUEST_TARGET_RADIUS), 2);
    }

    #[test]
    fn result_five_keeps_going_when_the_path_end_is_within_the_radius() {
        let mut r = far_path_rig(10.0, 11.0);
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_c5e0, &args![r.mover, 5u32]);
        let log = calls(&mut r.e);
        assert_eq!(count(&log, SET_PATHING_FAILED), 0);
        assert_eq!(count(&log, MOVER_FAKE + 0x10), 0);
    }

    #[test]
    fn path_check_needs_the_arrays_to_have_content() {
        let mut e = world();
        let path = e.mem.alloc(0x48);
        e.mem.set_i32(path + 0x18, -1);
        e.mem.set_i32(path + 0x1c, -1);
        // First array empty.
        assert!(!e.call(0x009d_c530, &args![path]).bool());
        e.mem.set_u32(path + 0x10, 1);
        // Both indexes -1: the second array is not looked at.
        e.call_log = Some(vec![]);
        assert!(e.call(0x009d_c530, &args![path]).bool());
        let log = calls(&mut e);
        assert_eq!(count(&log, ARRAY_IS_EMPTY), 1);
        assert_eq!(count(&log, PATH_FLAG_40), 1);
        // Indexes set and an empty second array.
        e.mem.set_i32(path + 0x18, 0);
        e.mem.set_i32(path + 0x1c, 0);
        assert!(!e.call(0x009d_c530, &args![path]).bool());
        e.mem.set_u32(path + 0x28, 1);
        assert!(e.call(0x009d_c530, &args![path]).bool());
        // One index -1 is enough to skip the second array.
        e.mem.set_u32(path + 0x28, 0);
        e.mem.set_i32(path + 0x1c, -1);
        assert!(e.call(0x009d_c530, &args![path]).bool());
    }

    #[test]
    fn handler_update_result_is_handled_unless_in_dialog() {
        let mut r = rig();
        let handler = attach_handler(&mut r);
        r.e.register(HANDLER_FAKE + 0x0c, |_, _| ret(1));
        r.e.register(HANDLER_FAKE + 0x34, |_, _| ret(0));
        r.e.set(r.mover, ActorMover::bInDialog, true);
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_c590, &args![r.mover, 0.5f32]);
        assert_eq!(calls(&mut r.e).len(), 1);
        r.e.set(r.mover, ActorMover::bInDialog, false);
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_c590, &args![r.mover, 0.5f32]);
        let log = calls(&mut r.e);
        assert_eq!(
            first(&log, HANDLER_FAKE + 0x0c).1,
            vec![handler, 0.5f32.to_bits()]
        );
        assert_eq!(count(&log, SET_PATHING_FAILED), 2);
        assert!(r.e.get(r.mover, ActorMover::bStopMovementNextUpdate));
        assert_eq!(r.e.get(r.mover, ActorMover::eState), STATE_FINISHED);
    }

    #[test]
    fn process_level_flag_follows_the_handler_kind() {
        for (flag, detailed, expected) in [
            (false, false, false),
            (false, true, true),
            (true, false, false),
            (true, true, true),
        ] {
            let mut r = rig();
            attach_handler(&mut r);
            let slot: AbiFn = if detailed {
                |_, _| ret(1)
            } else {
                |_, _| ret(0)
            };
            r.e.register(HANDLER_FAKE + 0x34, slot);
            r.e.set(r.mover, ActorMover::bActorChangedProcessLevel, flag);
            r.e.call(0x009d_c780, &args![r.mover]);
            assert_eq!(
                r.e.get(r.mover, ActorMover::bActorChangedProcessLevel),
                expected
            );
        }
        // Without a handler nothing happens.
        let mut r = rig();
        r.e.set(r.mover, ActorMover::bActorChangedProcessLevel, true);
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_c780, &args![r.mover]);
        assert_eq!(calls(&mut r.e).len(), 1);
        assert!(r.e.get(r.mover, ActorMover::bActorChangedProcessLevel));
    }

    #[test]
    fn process_level_four_and_nine_do_nothing_and_other_levels_leave_furniture() {
        for (level, leaves) in [(4u32, false), (9, false), (3, true)] {
            let mut r = rig();
            attach_process(&mut r);
            let slot: AbiFn = match level {
                4 => |_, _| ret(4),
                9 => |_, _| ret(9),
                _ => |_, _| ret(3),
            };
            r.e.register(PROCESS_FAKE + 0x4bc, slot);
            r.e.call_log = Some(vec![]);
            r.e.call(0x009d_c7f0, &args![r.mover]);
            let log = calls(&mut r.e);
            assert_eq!(count(&log, ACTOR_LEAVE_FURNITURE), leaves as usize);
            assert_eq!(count(&log, ACTOR_STOP_MOVEMENT), 0);
        }
    }

    #[test]
    fn process_level_zero_without_a_path_stops_the_actor_unless_the_global_says_no() {
        for blocked in [false, true] {
            let mut r = rig();
            attach_process(&mut r);
            r.e.register(PROCESS_FAKE + 0x4bc, |_, _| ret(0));
            r.e.set_global(GLOBAL_FLAGS_OBJECT, 0x6666u32);
            let test: AbiFn = if blocked {
                |_, _| ret(1)
            } else {
                |_, _| ret(0)
            };
            r.e.register(GLOBAL_FLAGS_TEST, test);
            r.e.call_log = Some(vec![]);
            r.e.call(0x009d_c7f0, &args![r.mover]);
            let log = calls(&mut r.e);
            assert_eq!(first(&log, GLOBAL_FLAGS_TEST).1, vec![0x6666]);
            assert_eq!(count(&log, ACTOR_STOP_MOVEMENT), (!blocked) as usize);
            if !blocked {
                assert_eq!(first(&log, ACTOR_STOP_MOVEMENT).1, vec![r.actor, 0]);
            }
        }
    }

    #[test]
    fn process_level_zero_with_a_path_syncs_the_changed_level_flag() {
        // (flag, detailed handler, previous node, next node, path nodes) ->
        // flag afterwards.
        for (flag, detailed, previous, next, nodes, expected) in [
            // Detailed, same node, not yet at the end: the flag stays.
            (true, true, 1u16, 1u16, 5u32, true),
            // Detailed, same node at the end: cleared.
            (true, true, 4, 4, 5, false),
            // Detailed, different nodes: cleared.
            (true, true, 1, 2, 5, false),
            // Not detailed: the flag is set.
            (false, false, 1, 1, 5, true),
            // Flag set, not detailed: set again after the check.
            (true, false, 1, 1, 5, true),
            // Detailed with the flag clear: left alone.
            (false, true, 1, 1, 5, false),
        ] {
            let mut r = rig();
            attach_process(&mut r);
            attach_path(&mut r);
            attach_handler(&mut r);
            r.e.register(PROCESS_FAKE + 0x4bc, |_, _| ret(0));
            let slot: AbiFn = if detailed {
                |_, _| ret(1)
            } else {
                |_, _| ret(0)
            };
            r.e.register(HANDLER_FAKE + 0x34, slot);
            r.e.register(PATH_NODE_COUNT, |_, _| ret(5));
            r.e.set(r.mover, ActorMover::eState, STATE_MOVING);
            r.e.set(r.mover, ActorMover::bActorChangedProcessLevel, flag);
            r.e.set(r.mover, ActorMover::iPreviousNode, previous);
            r.e.set(r.mover, ActorMover::iNextNode, next);
            let _ = nodes;
            r.e.call(0x009d_c7f0, &args![r.mover]);
            assert_eq!(
                r.e.get(r.mover, ActorMover::bActorChangedProcessLevel),
                expected,
                "flag {flag} detailed {detailed} previous {previous} next {next}"
            );
        }
    }

    #[test]
    fn deleted_reference_is_forwarded_to_the_handler() {
        let mut r = rig();
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_c950, &args![r.mover, p(0x777)]);
        assert_eq!(calls(&mut r.e).len(), 1);
        let handler = attach_handler(&mut r);
        r.e.register(HANDLER_FAKE + 0x5c, |_, _| Ret::default());
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_c950, &args![r.mover, p(0x777)]);
        assert_eq!(
            first(&calls(&mut r.e), HANDLER_FAKE + 0x5c).1,
            vec![handler, 0x777]
        );
    }

    #[test]
    fn advance_without_a_next_node_fails_the_mover() {
        let mut r = rig();
        r.e.call_log = Some(vec![]);
        let code = r.e.call(0x009d_c980, &args![r.mover]).u32();
        let log = calls(&mut r.e);
        assert_eq!(code, 2);
        assert_eq!(r.e.get(r.mover, ActorMover::eState), STATE_FAILED);
        assert_eq!(first(&log, SET_PATHING_FAILED).1, vec![r.mover.addr(), 1]);
        assert!(r.e.get(r.mover, ActorMover::bStopMovementNextUpdate));
    }

    #[test]
    fn advance_moves_the_previous_node_and_stops_at_the_end() {
        for (next, stops) in [(3u16, false), (4, true), (9, true)] {
            let mut r = rig();
            attach_path(&mut r);
            attach_handler(&mut r);
            r.e.register(HANDLER_FAKE + 0x34, |_, _| ret(0));
            r.e.register(PATH_NODE_COUNT, |_, _| ret(5));
            r.e.set(r.mover, ActorMover::iNextNode, next);
            r.e.set(r.mover, ActorMover::eState, STATE_MOVING);
            r.e.call_log = Some(vec![]);
            let code = r.e.call(0x009d_c980, &args![r.mover]).u32();
            let log = calls(&mut r.e);
            assert_eq!(code, 0);
            assert_eq!(r.e.get(r.mover, ActorMover::iPreviousNode), next);
            assert_eq!(count(&log, SET_PATHING_FAILED), stops as usize);
        }
    }

    #[test]
    fn advance_through_the_second_array_resets_it() {
        for (usable, level, expected, resets) in [
            (true, 0u32, 1u32, true),
            (false, 5, 1, true),
            (false, 0, 2, false),
        ] {
            let mut r = rig();
            attach_path(&mut r);
            attach_process(&mut r);
            r.e.register(PATH_COUNT_20, |_, _| ret(1));
            r.e.register(PATH_SECOND_ARRAY_ELEMENT, |_, _| ret(0x1234));
            r.e.register(NODE_FLAGS_ARE_3, |_, _| ret(1));
            let test: AbiFn = if usable { |_, _| ret(1) } else { |_, _| ret(0) };
            r.e.register(NODE_TEST_ACTOR, test);
            let slot: AbiFn = if level == 0 {
                |_, _| ret(0)
            } else {
                |_, _| ret(5)
            };
            r.e.register(PROCESS_FAKE + 0x610, slot);
            attach_handler(&mut r);
            r.e.register(HANDLER_FAKE + 0x34, |_, _| ret(0));
            r.e.set(r.mover, ActorMover::iNextNode, 2);
            r.e.call_log = Some(vec![]);
            let code = r.e.call(0x009d_c980, &args![r.mover]).u32();
            let log = calls(&mut r.e);
            assert_eq!(code, expected);
            assert_eq!(count(&log, PATH_RESET_LOADED), resets as usize);
            assert_eq!(first(&log, NODE_TEST_ACTOR).1, vec![0x1234, r.actor]);
        }
    }

    #[test]
    fn current_node_index_comes_from_the_handler() {
        let mut r = rig();
        assert_eq!(r.e.call(0x009d_cab0, &args![r.mover]).u32(), 0);
        attach_handler(&mut r);
        r.e.register(HANDLER_FAKE + 0x30, |_, _| ret(6));
        assert_eq!(r.e.call(0x009d_cab0, &args![r.mover]).u32(), 6);
    }

    #[test]
    fn actor_in_loaded_area_needs_a_cell_and_a_waking_player() {
        for (cell, passes, sleeping, expected) in [
            (0u32, true, false, false),
            (0x40, false, false, false),
            (0x40, true, true, false),
            (0x40, true, false, true),
        ] {
            let mut r = rig();
            let cell_slot: AbiFn = if cell == 0 {
                |_, _| ret(0)
            } else {
                |_, _| ret(0x40)
            };
            r.e.register(GET_PARENT_CELL, cell_slot);
            let passes_slot: AbiFn = if passes { |_, _| ret(1) } else { |_, _| ret(0) };
            r.e.register(CELL_IS_SIX, passes_slot);
            let sleeping_slot: AbiFn = if sleeping {
                |_, _| ret(1)
            } else {
                |_, _| ret(0)
            };
            r.e.register(PLAYER_IS_SLEEPING, sleeping_slot);
            r.e.set_global(PLAYER_CHARACTER, 0x5555u32);
            let result = r.e.call(0x009d_cae0, &args![r.mover]).bool();
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn handler_queries_fall_back_without_a_usable_path() {
        for (address, slot, default) in
            [(0x009d_cb30u32, 0x3cu32, true), (0x009d_cb90, 0x40, false)]
        {
            // No handler.
            let mut r = rig();
            attach_path(&mut r);
            assert_eq!(r.e.call(address, &args![r.mover]).bool(), default);
            // A handler but no path.
            let mut r = rig();
            attach_handler(&mut r);
            assert_eq!(r.e.call(address, &args![r.mover]).bool(), default);
            // A handler and a path whose first array is empty.
            let mut r = rig();
            attach_handler(&mut r);
            let path = attach_path(&mut r);
            r.e.mem.set_u32(path + 0x10, 0);
            assert_eq!(r.e.call(address, &args![r.mover]).bool(), default);
            // A usable path: the handler decides.
            for answer in [true, false] {
                let mut r = rig();
                attach_handler(&mut r);
                attach_path(&mut r);
                let reply: AbiFn = if answer { |_, _| ret(1) } else { |_, _| ret(0) };
                r.e.register(HANDLER_FAKE + slot, reply);
                assert_eq!(r.e.call(address, &args![r.mover]).bool(), answer);
            }
        }
    }

    #[test]
    fn handler_floats_default_to_zero() {
        for (address, slot) in [
            (0x009d_cbf0u32, 0x1cu32),
            (0x009d_cc20, 0x18),
            (0x009d_cc50, 0x28),
        ] {
            let mut r = rig();
            assert_eq!(r.e.call(address, &args![r.mover]).f32(), 0.0);
            attach_handler(&mut r);
            r.e.register(HANDLER_FAKE + slot, |_, _| ret_float(0.75));
            assert_eq!(r.e.call(address, &args![r.mover]).f32(), 0.75);
        }
    }

    #[test]
    fn approximate_location_comes_from_the_handler_or_the_actor() {
        let mut r = rig();
        goal_world(&mut r);
        // Plain words, including a signalling NaN, are copied bit for bit.
        r.e.mem.set_u32(ACTOR_POSITION, 0x7fa0_0001);
        r.e.mem.set_u32(ACTOR_POSITION + 4, 0x3f80_0000);
        r.e.mem.set_u32(ACTOR_POSITION + 8, 0x8000_0000);
        let out = SCRATCH + 0x100;
        let back = r.e.call(0x009d_cc80, &args![r.mover, p(out), 1.5f32]).u32();
        assert_eq!(back, out);
        assert_eq!(r.e.mem.u32(out), 0x7fa0_0001);
        assert_eq!(r.e.mem.u32(out + 4), 0x3f80_0000);
        assert_eq!(r.e.mem.u32(out + 8), 0x8000_0000);
        // With a handler the call goes to its slot 0x2c; the float is
        // passed through the x87 stack and so quieted.
        let handler = attach_handler(&mut r);
        r.e.register(HANDLER_FAKE + 0x2c, |_, _| Ret::default());
        r.e.call_log = Some(vec![]);
        let back =
            r.e.call(
                0x009d_cc80,
                &args![r.mover, p(out), f32::from_bits(0x7fa0_0001)],
            )
            .u32();
        assert_eq!(back, out);
        assert_eq!(
            first(&calls(&mut r.e), HANDLER_FAKE + 0x2c).1,
            vec![handler, out, 0x7fe0_0001]
        );
    }

    #[test]
    fn handler_blocked_query_needs_a_usable_path() {
        let mut r = rig();
        attach_handler(&mut r);
        assert!(!r.e.call(0x009d_ccf0, &args![r.mover]).bool());
        attach_path(&mut r);
        r.e.register(HANDLER_FAKE + 0x44, |_, _| ret(1));
        assert!(r.e.call(0x009d_ccf0, &args![r.mover]).bool());
    }

    #[test]
    fn pathing_complete_follows_the_state() {
        // (waiting, state, usable path, expected)
        for (waiting, state, usable, expected) in [
            (true, STATE_FINISHED, false, false),
            (false, STATE_FINISHED, true, true),
            (false, STATE_FAILED, true, true),
            (false, STATE_MOVING, true, false),
            (false, STATE_MOVING, false, true),
            (false, STATE_ROTATING, false, false),
            (false, STATE_ROTATING, true, false),
        ] {
            let mut r = rig();
            if usable {
                attach_path(&mut r);
            }
            r.e.set(r.mover, ActorMover::bWaitingForPath, waiting);
            r.e.set(r.mover, ActorMover::eState, state);
            assert_eq!(
                r.e.call(0x009d_cd40, &args![r.mover]).bool(),
                expected,
                "waiting {waiting} state {state} usable {usable}"
            );
        }
    }

    #[test]
    fn pathing_follows_the_state_and_the_path() {
        // (forced, state, waiting, usable path, failed, expected)
        for (forced, state, waiting, usable, failed, expected) in [
            (true, STATE_ROTATING, true, true, false, false),
            (false, STATE_ROTATING, false, false, false, true),
            (false, STATE_MOVING, true, false, false, true),
            (false, STATE_MOVING, false, true, false, true),
            (false, STATE_MOVING, false, true, true, false),
            (false, STATE_MOVING, false, false, false, false),
        ] {
            let mut r = rig();
            if usable {
                attach_path(&mut r);
            }
            r.e.set(r.mover, ActorMover::bForceMoveMode, forced);
            r.e.set(r.mover, ActorMover::eState, state);
            r.e.set(r.mover, ActorMover::bWaitingForPath, waiting);
            r.e.set(r.mover, ActorMover::bPathingFailed, failed);
            assert_eq!(
                r.e.call(0x009d_cdb0, &args![r.mover]).bool(),
                expected,
                "forced {forced} state {state} waiting {waiting} usable {usable} failed {failed}"
            );
        }
    }

    #[test]
    fn rotate_to_a_point_uses_the_angle_from_the_difference() {
        let mut r = rig();
        goal_world(&mut r);
        put_point(&mut r.e, ACTOR_POSITION, 1.0, 2.0, 3.0);
        // The difference vector is copied out for the test to look at.
        r.e.register(Z_ANGLE_FROM_VECTOR, |e, a| {
            for offset in [0, 4, 8] {
                let word = e.mem.u32(a[0] + offset);
                e.mem.set_u32(SCRATCH + 0x700 + offset, word);
            }
            ret_float(0.5)
        });
        r.e.register(ACTOR_CAN_MOVE, |_, _| ret(0));
        r.e.register(ACTOR_FAKE + 0x1e4, |_, _| ret(0));
        r.e.call_log = Some(vec![]);
        let result =
            r.e.call(
                0x009d_ce20,
                &args![r.mover, 11.0f32, 22.0f32, 33.0f32, true],
            )
            .bool();
        assert!(result);
        let log = calls(&mut r.e);
        assert_eq!(point(&r.e, SCRATCH + 0x700), [10.0, 20.0, 30.0]);
        assert_eq!(r.e.get(r.mover, ActorMover::eState), STATE_FINISHED);
        assert_eq!(count(&log, Z_ANGLE_FROM_VECTOR), 1);
    }

    /// Doubles for `RequestRotateActor` (the angle overload).
    fn rotate_world(r: &mut Rig) {
        r.e.register(ACTOR_FAKE + 0x1e4, |_, _| ret(0));
        r.e.register(ACTOR_CAN_MOVE, |_, _| ret(1));
        r.e.register(ACTOR_FAKE + 0x214, |_, _| ret(0));
        r.e.register(ACTOR_FAKE + 0x2c4, |_, _| Ret::default());
        r.e.register(GET_PARENT_CELL, |_, _| ret(0x40));
        r.e.register(CELL_IS_SIX, |_, _| ret(1));
    }

    #[test]
    fn rotate_finishes_at_once_when_the_actor_cannot_move() {
        for blocked_by_slot in [false, true] {
            let mut r = rig();
            rotate_world(&mut r);
            if blocked_by_slot {
                r.e.register(ACTOR_FAKE + 0x214, |_, _| ret(1));
            } else {
                r.e.register(ACTOR_CAN_MOVE, |_, _| ret(0));
            }
            r.e.set(r.mover, ActorMover::bWaitingForPath, false);
            let result = r.e.call(0x009d_ce80, &args![r.mover, 1.0f32, false]).bool();
            assert!(result);
            assert_eq!(r.e.get(r.mover, ActorMover::eState), STATE_FINISHED);
            assert_eq!(r.e.get(r.mover, ActorMover::pCurrentPathHandler), Ptr::NULL);
        }
    }

    #[test]
    fn rotate_outside_the_loaded_area_turns_the_actor_directly() {
        let mut r = rig();
        rotate_world(&mut r);
        r.e.register(GET_PARENT_CELL, |_, _| ret(0));
        r.e.call_log = Some(vec![]);
        let result =
            r.e.call(
                0x009d_ce80,
                &args![r.mover, f32::from_bits(0x7fa0_0001), true],
            )
            .bool();
        assert!(result);
        let log = calls(&mut r.e);
        assert_eq!(
            first(&log, ACTOR_FAKE + 0x2c4).1,
            vec![r.actor, 0x7fe0_0001]
        );
        assert_eq!(r.e.get(r.mover, ActorMover::eState), STATE_FINISHED);
    }

    #[test]
    fn rotate_with_a_handler_starts_the_rotation() {
        for started in [true, false] {
            let (mut r, previous) = takeover_rig();
            rotate_world(&mut r);
            r.e.register(PLAYER_IS_SLEEPING, |_, _| ret(0));
            // The existing handler is not a detailed one: it is replaced.
            r.e.register(HANDLER_FAKE + 0x34, |_, _| ret(0));
            let rotation: AbiFn = if started {
                |_, _| ret(1)
            } else {
                |_, _| ret(0)
            };
            r.e.register(HANDLER_FAKE + 0x08, rotation);
            r.e.register(ANIMATION_SHOULD_BE_MOVING, |_, _| ret(1));
            r.e.register(ACTOR_FAKE + 0x1e4, |_, _| ret(0x0600_0900));
            r.e.call_log = Some(vec![]);
            let result = r.e.call(0x009d_ce80, &args![r.mover, 1.5f32, true]).bool();
            let log = calls(&mut r.e);
            assert_eq!(result, started);
            let handler = r.e.get(r.mover, ActorMover::pCurrentPathHandler).addr();
            assert_ne!(handler, previous);
            assert_eq!(first(&log, HANDLER_FAKE).1, vec![previous, 1]);
            assert_eq!(
                first(&log, DETAILED_HANDLER_CONSTRUCT_SIMPLE).1,
                vec![handler, r.actor, r.mover.addr()]
            );
            assert_eq!(
                first(&log, HANDLER_FAKE + 0x08).1,
                vec![handler, 1.5f32.to_bits(), 1]
            );
            assert!(r.e.get(r.mover, ActorMover::bStopCurrentMoveAnim));
            let expected = if started {
                STATE_ROTATING
            } else {
                STATE_FINISHED
            };
            assert_eq!(r.e.get(r.mover, ActorMover::eState), expected);
        }
    }

    #[test]
    fn rotate_keeps_a_detailed_handler() {
        let (mut r, previous) = takeover_rig();
        rotate_world(&mut r);
        r.e.register(HANDLER_FAKE + 0x34, |_, _| ret(1));
        r.e.register(HANDLER_FAKE + 0x08, |_, _| ret(1));
        r.e.call_log = Some(vec![]);
        assert!(r.e.call(0x009d_ce80, &args![r.mover, 1.5f32, false]).bool());
        let log = calls(&mut r.e);
        assert_eq!(count(&log, HANDLER_FAKE), 0);
        assert_eq!(
            r.e.get(r.mover, ActorMover::pCurrentPathHandler).addr(),
            previous
        );
        assert_eq!(
            first(&log, HANDLER_FAKE + 0x08).1,
            vec![previous, 1.5f32.to_bits(), 0]
        );
    }

    #[test]
    fn stop_moving_calls_do_stop_and_the_stub() {
        let mut r = rig();
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_d0a0, &args![r.mover]);
        let log = calls(&mut r.e);
        assert!(r.e.get(r.mover, ActorMover::bStopMovementNextUpdate));
        assert_eq!(
            order(&log, &[SET_PATHING_FAILED, DEBUG_STUB]),
            vec![SET_PATHING_FAILED, DEBUG_STUB]
        );
    }

    #[test]
    fn force_stop_finishes_a_pending_stop() {
        let mut r = rig();
        r.e.set(r.mover, ActorMover::uiPreferredMoveMode, 0x1234);
        r.e.set(r.mover, ActorMover::bHaveValidDeltaThisFrame, false);
        r.e.set_global(UNSET_POINT, 0x7fa0_0001u32);
        put_point(&mut r.e, r.mover.addr() + 0x10, 5.0, 6.0, 7.0);
        r.e.call(0x009d_d0c0, &args![r.mover]);
        // `DoStopMoving` requested the stop and the function carried it out.
        assert!(!r.e.get(r.mover, ActorMover::bStopMovementNextUpdate));
        assert_eq!(
            r.e.get(r.mover, ActorMover::iCachedLastValidMoveMode),
            0x1200
        );
        assert!(r.e.get(r.mover, ActorMover::bHaveValidDeltaThisFrame));
        assert_eq!(r.e.mem.u32(r.mover.addr() + 0x10), 0x7fa0_0001);
        assert_eq!(r.e.get(r.mover, ActorMover::eState), STATE_FINISHED);
    }

    #[test]
    fn force_stop_with_the_state_already_finished_does_not_finish_again() {
        let mut r = rig();
        r.e.set(r.mover, ActorMover::eState, STATE_FINISHED);
        r.e.set(r.mover, ActorMover::iCachedLastValidMoveMode, 77);
        r.e.call(0x009d_d0c0, &args![r.mover]);
        assert_eq!(r.e.get(r.mover, ActorMover::iCachedLastValidMoveMode), 77);
        assert!(!r.e.get(r.mover, ActorMover::bHaveValidDeltaThisFrame));
    }

    #[test]
    fn preferred_mode_high_part() {
        let mut r = rig();
        r.e.set(r.mover, ActorMover::uiPreferredMoveMode, 0xabcd_ef12);
        assert_eq!(r.e.call(0x009d_d130, &args![r.mover]).u32(), 0xef00);
    }

    #[test]
    fn do_stop_moving_marks_the_stop_and_resets_the_failure() {
        // (state, handler kind: none/plain/detailed, pathing failed) ->
        // (state afterwards, stop requested)
        for (state, handler, failed, state_after, stop) in [
            (STATE_MOVING, 0u8, false, STATE_MOVING, true),
            (STATE_MOVING, 1, false, STATE_FINISHED, true),
            (STATE_MOVING, 2, false, STATE_MOVING, true),
            (STATE_FINISHED, 1, false, STATE_FINISHED, false),
            (STATE_FINISHED, 0, true, STATE_FINISHED, true),
        ] {
            let mut r = rig();
            if handler != 0 {
                attach_handler(&mut r);
                let detailed: AbiFn = if handler == 2 {
                    |_, _| ret(1)
                } else {
                    |_, _| ret(0)
                };
                r.e.register(HANDLER_FAKE + 0x34, detailed);
            }
            r.e.set(r.mover, ActorMover::eState, state);
            r.e.set(r.mover, ActorMover::bPathingFailed, failed);
            r.e.call_log = Some(vec![]);
            r.e.call(0x009d_d150, &args![r.mover]);
            let log = calls(&mut r.e);
            assert_eq!(r.e.get(r.mover, ActorMover::eState), state_after);
            assert_eq!(r.e.get(r.mover, ActorMover::bStopMovementNextUpdate), stop);
            assert_eq!(first(&log, SET_PATHING_FAILED).1, vec![r.mover.addr(), 0]);
        }
    }

    #[test]
    fn finish_moving_sets_finished_unless_failed() {
        for (before, after) in [
            (0u32, STATE_FINISHED),
            (2, STATE_FINISHED),
            (6, STATE_FINISHED),
            (7, 7),
            (8, 8),
            (9, STATE_FINISHED),
        ] {
            let mut r = rig();
            attach_request(&mut r);
            attach_path(&mut r);
            r.e.set(r.mover, ActorMover::eState, before);
            r.e.call_log = Some(vec![]);
            r.e.call(0x009d_d1c0, &args![r.mover, false]);
            let log = calls(&mut r.e);
            assert_eq!(r.e.get(r.mover, ActorMover::eState), after);
            assert_eq!(r.e.mem.u32(r.mover.addr() + 0x1c), 0);
            assert_eq!(r.e.mem.u32(r.mover.addr() + 0x20), 0);
            assert_eq!(first(&log, SET_PATHING_FAILED).1, vec![r.mover.addr(), 0]);
            assert_eq!(log.last().unwrap().0, DEBUG_STUB);
        }
    }

    #[test]
    fn finish_moving_clears_the_handler_movement_only_on_request() {
        for clear in [true, false] {
            let mut r = rig();
            let handler = attach_handler(&mut r);
            r.e.register(HANDLER_FAKE + 0x70, |_, _| Ret::default());
            r.e.call_log = Some(vec![]);
            r.e.call(0x009d_d1c0, &args![r.mover, clear]);
            let log = calls(&mut r.e);
            assert_eq!(count(&log, HANDLER_FAKE + 0x70), clear as usize);
            if clear {
                assert_eq!(first(&log, HANDLER_FAKE + 0x70).1, vec![handler]);
            }
        }
    }

    #[test]
    fn finish_moving_abandons_a_pending_path_request() {
        let mut r = rig();
        r.e.register(QUEUE_CONSTRUCT, |_, a| ret(a[0]));
        r.e.set(r.mover, ActorMover::bWaitingForPath, true);
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_d1c0, &args![r.mover, true]);
        let log = calls(&mut r.e);
        assert!(!r.e.get(r.mover, ActorMover::bWaitingForPath));
        assert_eq!(first(&log, ALLOCATE).1, vec![0x1c]);
        assert_eq!(first(&log, QUEUE_POINTER_SET).1[0], r.mover.addr() + 0x30);
        assert_eq!(
            order(&log, &[QUEUE_POINTER_SET, SET_PATHING_FAILED]),
            vec![QUEUE_POINTER_SET, SET_PATHING_FAILED]
        );
    }

    /// A mover whose actor's process lets `Update` run.
    fn update_rig() -> Rig {
        let mut r = rig();
        attach_process(&mut r);
        r.e.register(ACTOR_FAKE + 0x22c, |_, _| ret(0));
        r.e.register(ACTOR_FAKE + 0x230, |_, _| ret(0));
        r.e.register(PROCESS_FAKE + 0x610, |_, _| ret(0));
        r.e.register(PROCESS_FAKE + 0x4bc, |_, _| ret(0));
        r.e.register(PROCESS_FAKE + 0x40c, |_, _| ret(0));
        r.e.register(PROCESS_FAKE + 0x6fc, |_, _| ret(0));
        r.e.register(PROCESS_LEVEL, |_, _| ret(5));
        r.e.register(GET_DESIRED_PROCESS_LEVEL, |_, _| ret(5));
        r
    }

    /// The handler slots the move delta (`fn_009ddc00`) reads.
    fn quiet_handler(r: &mut Rig) -> u32 {
        let handler = attach_handler(r);
        r.e.register(HANDLER_FAKE, |_, _| Ret::default());
        r.e.register(HANDLER_FAKE + 0x48, |_, _| ret(0));
        r.e.register(HANDLER_FAKE + 0x4c, |_, _| ret(0));
        handler
    }

    #[test]
    fn update_carries_out_a_requested_stop() {
        let mut r = update_rig();
        r.e.register(ACTOR_FAKE + 0x22c, |_, _| ret(1));
        r.e.set(r.mover, ActorMover::bStopMovementNextUpdate, true);
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_c120, &args![r.mover, 0.1f32]);
        let log = calls(&mut r.e);
        assert!(!r.e.get(r.mover, ActorMover::bStopMovementNextUpdate));
        assert!(r.e.get(r.mover, ActorMover::bStopCurrentMoveAnim));
        assert_eq!(r.e.get(r.mover, ActorMover::eState), STATE_FINISHED);
        assert_eq!(first(&log, ACTOR_FAKE + 0x22c).1, vec![r.actor, 0]);
        assert_eq!(count(&log, GET_PROCESS), 0);
        assert_eq!(log.last().unwrap().0, DEBUG_STUB);
    }

    #[test]
    fn update_while_the_actor_is_busy_clears_the_handler_movement() {
        let mut r = update_rig();
        r.e.register(ACTOR_FAKE + 0x22c, |_, _| ret(1));
        quiet_handler(&mut r);
        r.e.register(HANDLER_FAKE + 0x70, |_, _| Ret::default());
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_c120, &args![r.mover, 0.1f32]);
        assert_eq!(count(&calls(&mut r.e), HANDLER_FAKE + 0x70), 1);
    }

    #[test]
    fn update_does_nothing_until_the_process_allows_it() {
        let gates: [fn(&mut Rig); 7] = [
            |r| r.e.mem.set_u32(PROCESS_CELL, 0),
            |r| r.e.register(PROCESS_FAKE + 0x610, |_, _| ret(4)),
            |r| r.e.register(GET_DESIRED_PROCESS_LEVEL, |_, _| ret(6)),
            |r| r.e.register(PROCESS_FAKE + 0x4bc, |_, _| ret(1)),
            |r| r.e.register(ACTOR_FAKE + 0x230, |_, _| ret(1)),
            |r| r.e.register(PROCESS_FAKE + 0x40c, |_, _| ret(1)),
            |r| r.e.register(PROCESS_FAKE + 0x6fc, |_, _| ret(1)),
        ];
        for gate in gates {
            let mut r = update_rig();
            attach_path(&mut r);
            quiet_handler(&mut r);
            r.e.register(HANDLER_FAKE + 0x0c, |_, _| ret(0));
            r.e.register(HANDLER_FAKE + 0x10, |_, _| ret(0));
            gate(&mut r);
            r.e.call_log = Some(vec![]);
            r.e.call(0x009d_c120, &args![r.mover, 0.1f32]);
            let log = calls(&mut r.e);
            assert_eq!(count(&log, HANDLER_FAKE + 0x0c), 0);
            assert_eq!(count(&log, HANDLER_FAKE + 0x10), 0);
            assert_eq!(r.e.get(r.mover, ActorMover::eState), 0);
            assert_eq!(log.last().unwrap().0, DEBUG_STUB);
        }
    }

    #[test]
    fn update_follows_a_usable_path() {
        let mut r = update_rig();
        attach_path(&mut r);
        let handler = quiet_handler(&mut r);
        r.e.register(HANDLER_FAKE + 0x0c, |_, _| ret(0));
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_c120, &args![r.mover, 0.25f32]);
        let log = calls(&mut r.e);
        assert_eq!(r.e.get(r.mover, ActorMover::eState), STATE_MOVING);
        assert_eq!(
            first(&log, HANDLER_FAKE + 0x0c).1,
            vec![handler, 0.25f32.to_bits()]
        );
    }

    #[test]
    fn update_after_a_process_level_change_rebuilds_without_a_path() {
        let mut r = update_rig();
        let previous = quiet_handler(&mut r);
        r.e.set(r.mover, ActorMover::bActorChangedProcessLevel, true);
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_c120, &args![r.mover, 0.25f32]);
        let log = calls(&mut r.e);
        assert!(!r.e.get(r.mover, ActorMover::bActorChangedProcessLevel));
        assert_eq!(first(&log, HANDLER_FAKE).1, vec![previous, 1]);
        assert_eq!(r.e.get(r.mover, ActorMover::pCurrentPathHandler), Ptr::NULL);
        assert_eq!(r.e.get(r.mover, ActorMover::eState), STATE_FINISHED);
    }

    #[test]
    fn update_takes_the_path_the_path_manager_built() {
        let mut r = update_rig();
        attach_request(&mut r);
        let queue = object(&mut r.e, QUEUE_TABLE, QUEUE_FAKE);
        r.e.mem.set_u32(r.mover.addr() + 0x30, queue);
        r.e.set(r.mover, ActorMover::bWaitingForPath, true);
        r.e.register(QUEUE_FAKE + 0x10, |e, a| {
            e.mem.set_u32(a[1], 0);
            e.mem.set_u32(a[1] + 4, 0x9999);
            ret(1)
        });
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_c120, &args![r.mover, 0.25f32]);
        let log = calls(&mut r.e);
        // The path is stored, then `fn_009dbdc0` runs; the request check
        // (a double that answers 0) fails the mover and stops the actor.
        assert_eq!(r.e.mem.u32(r.mover.addr() + 0x20), 0x9999);
        assert_eq!(first(&log, NI_REF_ACQUIRE).1, vec![0x9999]);
        assert!(!r.e.get(r.mover, ActorMover::bWaitingForPath));
        assert_eq!(first(&log, ACTOR_STOP_MOVEMENT).1, vec![r.actor, 1]);
        assert_eq!(r.e.get(r.mover, ActorMover::eState), STATE_FAILED);
        assert_eq!(count(&log, MESSAGE_DESTRUCT), 1);
        assert_eq!(first(&log, QUEUE_FAKE + 0x10).1[0], queue);
    }

    #[test]
    fn update_with_a_path_error_fails_and_remembers_the_end() {
        let mut r = update_rig();
        attach_request(&mut r);
        let queue = object(&mut r.e, QUEUE_TABLE, QUEUE_FAKE);
        r.e.mem.set_u32(r.mover.addr() + 0x30, queue);
        r.e.set(r.mover, ActorMover::bWaitingForPath, true);
        // The solution the message carries: a usable path with a last node.
        let path = SCRATCH + 0x400;
        r.e.mem.set_u32(path + 0x10, 1);
        r.e.mem.set_i32(path + 0x18, -1);
        r.e.mem.set_u32(path + 0x1c, 5);
        r.e.register(QUEUE_FAKE + 0x10, |e, a| {
            e.mem.set_u32(a[1], 1);
            e.mem.set_u32(a[1] + 4, SCRATCH + 0x400);
            ret(1)
        });
        r.e.register(PATH_NODE_COUNT, |_, _| ret(3));
        r.e.register(PATH_NODE_AT, |_, _| ret(0x4321));
        r.e.register(NODE_LOCATION, |_, a| ret(a[0] + 4));
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_c120, &args![r.mover, 0.25f32]);
        let log = calls(&mut r.e);
        assert_eq!(first(&log, PATH_NODE_AT).1, vec![path, 5]);
        assert_eq!(
            first(&log, LOCATION_ASSIGN).1,
            vec![r.mover.addr() + 0x44, 0x4325]
        );
        assert_eq!(
            first(&log, LOCATION_CLEAR_TRIANGLE).1,
            vec![r.mover.addr() + 0x44]
        );
        assert_eq!(first(&log, SET_PATHING_FAILED).1, vec![r.mover.addr(), 1]);
        // `ClearCurrentData` ran last: the state is finished.
        assert_eq!(r.e.get(r.mover, ActorMover::eState), STATE_FINISHED);
        assert!(!r.e.get(r.mover, ActorMover::bWaitingForPath));
    }

    #[test]
    fn update_without_a_path_lets_the_handler_update() {
        for (answer, state) in [(5u32, STATE_FINISHED), (3, 0)] {
            let mut r = update_rig();
            let handler = quiet_handler(&mut r);
            let reply: AbiFn = if answer == 5 {
                |_, _| ret(5)
            } else {
                |_, _| ret(3)
            };
            r.e.register(HANDLER_FAKE + 0x10, reply);
            r.e.call_log = Some(vec![]);
            r.e.call(0x009d_c120, &args![r.mover, 0.5f32]);
            let log = calls(&mut r.e);
            assert_eq!(
                first(&log, HANDLER_FAKE + 0x10).1,
                vec![handler, 0.5f32.to_bits()]
            );
            assert_eq!(r.e.get(r.mover, ActorMover::eState), state);
        }
    }

    #[test]
    fn update_with_pathing_failed_still_updates_the_handler() {
        let mut r = update_rig();
        attach_path(&mut r);
        quiet_handler(&mut r);
        r.e.register(HANDLER_FAKE + 0x10, |_, _| ret(1));
        r.e.set(r.mover, ActorMover::bPathingFailed, true);
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_c120, &args![r.mover, 0.5f32]);
        assert_eq!(count(&calls(&mut r.e), HANDLER_FAKE + 0x10), 1);
    }

    /// A node-less mover that has a request, a path, a detailed handler and a
    /// process type of 1 (the virtual rebuild).
    fn rebuild_rig() -> (Rig, u32) {
        let (mut r, previous) = takeover_rig();
        r.e.register(HANDLER_FAKE + 0x34, |_, _| ret(1));
        r.e.register(HANDLER_FAKE + 0x1c, |_, _| ret_float(0.5));
        r.e.register(GET_PROCESS_TYPE, |_, _| ret(1));
        r.e.register(FLOAT_ROUND, |_, a| ret_float(f32::from_bits(a[0]) as f64));
        r.e.register(FTOL, |_, a| {
            let value = f64::from_bits(a[0] as u64 | (a[1] as u64) << 32);
            ret(value as i32 as u32)
        });
        r.e.register(PATH_NODE_COUNT, |_, _| ret(10));
        r.e.register(REQUEST_CHECK_PATH, |_, _| ret(1));
        r.e.set(r.mover, ActorMover::iPreviousNode, 3);
        r.e.set(r.mover, ActorMover::iNextNode, 7);
        (r, previous)
    }

    #[test]
    fn rebuild_without_a_request_and_path_finishes() {
        let mut r = rig();
        let previous = attach_handler(&mut r);
        r.e.register(HANDLER_FAKE, |_, _| Ret::default());
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_d2d0, &args![r.mover]);
        let log = calls(&mut r.e);
        assert_eq!(first(&log, HANDLER_FAKE).1, vec![previous, 1]);
        assert_eq!(r.e.get(r.mover, ActorMover::pCurrentPathHandler), Ptr::NULL);
        assert_eq!(r.e.get(r.mover, ActorMover::eState), STATE_FINISHED);
        assert_eq!(first(&log, SET_PATHING_FAILED).1, vec![r.mover.addr(), 0]);
        // Only the path present: still nothing to work with.
        let mut r = rig();
        attach_path(&mut r);
        r.e.call(0x009d_d2d0, &args![r.mover]);
        assert_eq!(r.e.get(r.mover, ActorMover::eState), STATE_FINISHED);
    }

    #[test]
    fn rebuild_for_a_process_type_picks_the_node_from_the_percent() {
        let (mut r, previous) = rebuild_rig();
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_d2d0, &args![r.mover]);
        let log = calls(&mut r.e);
        // 4 nodes apart at 50 percent: node 3 + 2.
        assert_eq!(first(&log, FLOAT_ROUND).1, vec![2.0f32.to_bits()]);
        assert_eq!(r.e.get(r.mover, ActorMover::iPreviousNode), 5);
        assert_eq!(r.e.get(r.mover, ActorMover::iNextNode), 9);
        let handler = r.e.get(r.mover, ActorMover::pCurrentPathHandler).addr();
        assert_ne!(handler, previous);
        assert_eq!(first(&log, HANDLER_FAKE).1, vec![previous, 1]);
        let request = r.e.mem.u32(r.mover.addr() + 0x1c);
        assert_eq!(
            first(&log, VIRTUAL_HANDLER_CONSTRUCT).1,
            vec![
                handler,
                r.actor,
                r.mover.addr(),
                r.mover.addr() + 0x20,
                request
            ]
        );
        assert_eq!(
            first(&log, HANDLER_FAKE + 0x04).1,
            vec![handler, 5.0f32.to_bits(), 0.0f32.to_bits()]
        );
        assert_eq!(first(&log, REQUEST_SET_ACTOR).1, vec![request, r.actor]);
    }

    #[test]
    fn rebuild_from_the_same_node_uses_the_next_one_and_clamps() {
        let (mut r, _) = rebuild_rig();
        r.e.set(r.mover, ActorMover::iPreviousNode, 7);
        r.e.call(0x009d_d2d0, &args![r.mover]);
        assert_eq!(r.e.get(r.mover, ActorMover::iPreviousNode), 8);
        // A target beyond the last node is clamped to it.
        let (mut r, _) = rebuild_rig();
        r.e.register(PATH_NODE_COUNT, |_, _| ret(5));
        r.e.set(r.mover, ActorMover::iPreviousNode, 7);
        r.e.call(0x009d_d2d0, &args![r.mover]);
        assert_eq!(r.e.get(r.mover, ActorMover::iPreviousNode), 4);
        assert_eq!(r.e.get(r.mover, ActorMover::iNextNode), 4);
    }

    #[test]
    fn rebuild_fails_when_the_request_check_fails() {
        let (mut r, previous) = rebuild_rig();
        r.e.register(REQUEST_CHECK_PATH, |_, _| ret(0xffff_ffff));
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_d2d0, &args![r.mover]);
        let log = calls(&mut r.e);
        assert_eq!(r.e.get(r.mover, ActorMover::eState), STATE_FAILED);
        assert_eq!(
            log.last().unwrap(),
            &(SET_PATHING_FAILED, vec![r.mover.addr(), 1])
        );
        assert_eq!(
            r.e.get(r.mover, ActorMover::pCurrentPathHandler).addr(),
            previous
        );
        // A check result above 1 keeps the nodes and still rebuilds.
        let (mut r, previous) = rebuild_rig();
        r.e.register(REQUEST_CHECK_PATH, |_, _| ret(2));
        r.e.call(0x009d_d2d0, &args![r.mover]);
        assert_ne!(
            r.e.get(r.mover, ActorMover::pCurrentPathHandler).addr(),
            previous
        );
        assert_eq!(r.e.get(r.mover, ActorMover::iNextNode), 7);
    }

    #[test]
    fn rebuild_leaves_a_plain_handler_alone() {
        let (mut r, previous) = rebuild_rig();
        r.e.register(HANDLER_FAKE + 0x34, |_, _| ret(0));
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_d2d0, &args![r.mover]);
        let log = calls(&mut r.e);
        assert_eq!(count(&log, OPERATOR_NEW), 0);
        assert_eq!(
            r.e.get(r.mover, ActorMover::pCurrentPathHandler).addr(),
            previous
        );
    }

    /// The detailed rebuild (process type 0) over the takeover rig.
    fn detailed_rebuild_rig() -> (Rig, u32) {
        let (mut r, previous) = takeover_rig();
        r.e.register(GET_PROCESS_TYPE, |_, _| ret(0));
        r.e.register(HANDLER_FAKE + 0x34, |_, _| ret(1));
        r.e.register(PATH_NODE_COUNT, |_, _| ret(10));
        r.e.register(REQUEST_CHECK_PATH_FROM, |_, _| ret(2));
        r.e.register(PATH_FIRST_LOADED, |_, _| ret(4));
        let path = r.e.mem.u32(r.mover.addr() + 0x20);
        r.e.mem.set_u32(path + 0x1c, 9);
        r.e.set(r.mover, ActorMover::iPreviousNode, 3);
        (r, previous)
    }

    #[test]
    fn detailed_rebuild_starts_a_new_handler_at_the_loaded_nodes() {
        let (mut r, previous) = detailed_rebuild_rig();
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_d2d0, &args![r.mover]);
        let log = calls(&mut r.e);
        let request = r.e.mem.u32(r.mover.addr() + 0x1c);
        let path = r.e.mem.u32(r.mover.addr() + 0x20);
        // The next index is the previous node + 1 for a detailed handler.
        assert_eq!(
            first(&log, REQUEST_CHECK_PATH_FROM).1,
            vec![request, path, 4]
        );
        assert_eq!(r.e.get(r.mover, ActorMover::iPreviousNode), 4);
        assert_eq!(r.e.get(r.mover, ActorMover::iNextNode), 9);
        let handler = r.e.get(r.mover, ActorMover::pCurrentPathHandler).addr();
        assert_ne!(handler, previous);
        assert_eq!(
            first(&log, DETAILED_HANDLER_CONSTRUCT).1,
            vec![
                handler,
                r.actor,
                r.mover.addr(),
                r.mover.addr() + 0x20,
                request
            ]
        );
        // A detailed handler starts at fraction 0.
        assert_eq!(
            first(&log, HANDLER_FAKE + 0x04).1,
            vec![handler, 0.0f32.to_bits(), 0.0f32.to_bits()]
        );
        assert_eq!(count(&log, POINT_LENGTH), 0);
    }

    #[test]
    fn detailed_rebuild_without_a_detailed_handler_computes_the_fraction() {
        let (mut r, _) = detailed_rebuild_rig();
        r.e.register(HANDLER_FAKE + 0x34, |_, _| ret(0));
        r.e.register(HANDLER_FAKE + 0x18, |_, _| ret_float(2.5));
        r.e.register(HANDLER_FAKE + 0x30, |_, _| ret(3));
        r.e.register(PATH_NODE_AT, |_, a| ret(SCRATCH + 0x600 + a[1] * 0x10));
        r.e.register(NODE_LOCATION, |_, a| ret(a[0] + 4));
        r.e.register(GET_POSITION, |e, a| {
            for offset in [0, 4, 8] {
                let word = e.mem.u32(a[0] + 4 + offset);
                e.mem.set_u32(a[1] + offset, word);
            }
            ret(a[1])
        });
        r.e.register(POINT_SUBTRACT, |e, a| {
            for offset in [0, 4, 8] {
                let difference = e.mem.f32(a[0] + offset) - e.mem.f32(a[2] + offset);
                e.mem.set_f32(a[1] + offset, difference);
            }
            ret(a[1])
        });
        r.e.register(POINT_LENGTH, |e, a| {
            let x = e.mem.f32(a[0]);
            let y = e.mem.f32(a[0] + 4);
            ret_float((x * x + y * y).sqrt() as f64)
        });
        // Node 4 (first loaded) and node 9 (last loaded): 3-4-5 apart.
        put_point(&mut r.e, SCRATCH + 0x640 + 8, 0.0, 0.0, 0.0);
        put_point(&mut r.e, SCRATCH + 0x690 + 8, 3.0, 4.0, 0.0);
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_d2d0, &args![r.mover]);
        let log = calls(&mut r.e);
        let handler = r.e.get(r.mover, ActorMover::pCurrentPathHandler).addr();
        // Distance travelled 2.5 over a segment of length 5.
        assert_eq!(
            first(&log, HANDLER_FAKE + 0x04).1,
            vec![handler, 0.5f32.to_bits(), 0.0f32.to_bits()]
        );
    }

    #[test]
    fn detailed_rebuild_at_the_same_node_just_finishes() {
        let (mut r, _) = detailed_rebuild_rig();
        r.e.register(PATH_FIRST_LOADED, |_, _| ret(9));
        r.e.register(HANDLER_FAKE + 0x70, |_, _| Ret::default());
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_d2d0, &args![r.mover]);
        let log = calls(&mut r.e);
        assert_eq!(count(&log, OPERATOR_NEW), 0);
        assert_eq!(r.e.get(r.mover, ActorMover::eState), STATE_FINISHED);
        assert_eq!(count(&log, HANDLER_FAKE + 0x70), 1);
    }

    #[test]
    fn detailed_rebuild_handles_the_check_results() {
        // -1 with a path that cannot be used: the actor stops and the
        // mover fails with the process-update state.
        let (mut r, _) = detailed_rebuild_rig();
        r.e.register(REQUEST_CHECK_PATH_FROM, |_, _| ret(0xffff_ffff));
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_d2d0, &args![r.mover]);
        let log = calls(&mut r.e);
        assert_eq!(first(&log, ACTOR_STOP_MOVEMENT).1, vec![r.actor, 1]);
        assert_eq!(
            r.e.get(r.mover, ActorMover::eState),
            STATE_FAILED_PROCESS_UPDATE
        );
        assert_eq!(
            log.last().unwrap(),
            &(SET_PATHING_FAILED, vec![r.mover.addr(), 1])
        );
        // -1 with a usable path (flag, incomplete allowed, second array):
        // the nodes are taken from the path.
        let (mut r, _) = detailed_rebuild_rig();
        r.e.register(REQUEST_CHECK_PATH_FROM, |_, _| ret(0));
        r.e.register(PATH_FLAG_40, |_, _| ret(1));
        r.e.register(REQUEST_ALLOW_INCOMPLETE, |_, _| ret(1));
        r.e.register(PATH_COUNT_20, |_, _| ret(1));
        r.e.call(0x009d_d2d0, &args![r.mover]);
        assert_eq!(r.e.get(r.mover, ActorMover::iPreviousNode), 4);
        assert_eq!(r.e.get(r.mover, ActorMover::iNextNode), 9);
        // 1: the nodes are left alone (3 and 7 here).
        let (mut r, _) = detailed_rebuild_rig();
        r.e.register(REQUEST_CHECK_PATH_FROM, |_, _| ret(1));
        r.e.set(r.mover, ActorMover::iNextNode, 7);
        r.e.call(0x009d_d2d0, &args![r.mover]);
        assert_eq!(r.e.get(r.mover, ActorMover::iPreviousNode), 3);
        assert_eq!(r.e.get(r.mover, ActorMover::iNextNode), 7);
    }

    #[test]
    fn detailed_rebuild_acts_on_the_result_code_of_the_new_handler() {
        let (mut r, _) = detailed_rebuild_rig();
        r.e.register(HANDLER_FAKE + 0x04, |_, _| ret(7));
        r.e.register(HANDLER_FAKE + 0x70, |_, _| Ret::default());
        r.e.call(0x009d_d2d0, &args![r.mover]);
        // 7 is not 5 or 6: nothing else happens.
        assert_ne!(r.e.get(r.mover, ActorMover::eState), STATE_FINISHED);
    }

    #[test]
    fn detailed_rebuild_past_the_last_node_reports_where_the_path_ends() {
        for global_test in [true, false] {
            let (mut r, _) = detailed_rebuild_rig();
            r.e.register(PATH_NODE_COUNT, |_, _| ret(3));
            r.e.set(r.mover, ActorMover::iPreviousNode, 2);
            r.e.register(PATH_LAST_NODE, |_, _| ret(0x1111));
            r.e.register(NODE_LOCATION, |_, a| ret(a[0] + 4));
            r.e.register(GET_POSITION, |_, a| ret(a[1]));
            r.e.register(REPORT_TARGET_OBJECT, |_, _| ret(0x888));
            r.e.register(ACTOR_FAKE + 0x2a8, |_, _| Ret::default());
            r.e.register(HANDLER_FAKE + 0x70, |_, _| Ret::default());
            let test: AbiFn = if global_test {
                |_, _| ret(1)
            } else {
                |_, _| ret(0)
            };
            r.e.register(GLOBAL_TEST, test);
            let path = r.e.mem.u32(r.mover.addr() + 0x20);
            r.e.call_log = Some(vec![]);
            r.e.call(0x009d_d2d0, &args![r.mover]);
            let log = calls(&mut r.e);
            let position = first(&log, GET_POSITION).1[1];
            assert_eq!(first(&log, PATH_LAST_NODE).1, vec![path + 8]);
            assert_eq!(first(&log, NODE_LOCATION).1, vec![0x1111]);
            if global_test {
                assert_eq!(
                    first(&log, REPORT_PATH_END).1,
                    vec![0x888, r.actor, position]
                );
                assert_eq!(count(&log, ACTOR_FAKE + 0x2a8), 0);
            } else {
                assert_eq!(first(&log, ACTOR_FAKE + 0x2a8).1, vec![r.actor, position]);
                assert_eq!(count(&log, REPORT_PATH_END), 0);
            }
            assert_eq!(r.e.get(r.mover, ActorMover::eState), STATE_FINISHED);
        }
    }

    #[test]
    fn rebuild_of_a_handler_without_a_detailed_handler_uses_the_node_index() {
        let (mut r, _) = detailed_rebuild_rig();
        r.e.register(HANDLER_FAKE + 0x34, |_, _| ret(0));
        r.e.register(HANDLER_FAKE + 0x30, |_, _| ret(6));
        r.e.register(HANDLER_FAKE + 0x18, |_, _| ret_float(0.0));
        r.e.register(PATH_NODE_AT, |_, _| ret(SCRATCH + 0x600));
        r.e.register(NODE_LOCATION, |_, a| ret(a[0] + 4));
        r.e.register(GET_POSITION, |_, a| ret(a[1]));
        r.e.register(POINT_SUBTRACT, |_, a| ret(a[1]));
        r.e.register(POINT_LENGTH, |_, _| ret_float(1.0));
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_d2d0, &args![r.mover]);
        let log = calls(&mut r.e);
        let request = r.e.mem.u32(r.mover.addr() + 0x1c);
        let path = r.e.mem.u32(r.mover.addr() + 0x20);
        // The handler's current node (6) minus one is the next index.
        assert_eq!(
            first(&log, REQUEST_CHECK_PATH_FROM).1,
            vec![request, path, 5]
        );
    }

    #[test]
    fn last_node_of_the_path_array() {
        let mut e = world();
        e.register(PATH_LAST_NODE, |_, a| ret(a[0] + 1));
        assert_eq!(e.call(0x009d_db30, &args![0x1000u32]).u32(), 0x1009);
    }

    #[test]
    fn is_rotating_and_move_mode_and_move_delta() {
        let mut r = rig();
        r.e.set(r.mover, ActorMover::eState, STATE_ROTATING);
        assert!(r.e.call(0x009d_db50, &args![r.mover]).bool());
        r.e.set(r.mover, ActorMover::eState, STATE_MOVING);
        assert!(!r.e.call(0x009d_db50, &args![r.mover]).bool());

        r.e.set(r.mover, ActorMover::uiPreferredMoveMode, 0x0301);
        r.e.set(r.mover, ActorMover::iCachedLastValidMoveMode, 0x0044);
        assert_eq!(r.e.call(0x009d_db70, &args![r.mover]).u32(), 0x0044);
        r.e.set(r.mover, ActorMover::bPathingFailed, true);
        assert_eq!(r.e.call(0x009d_db70, &args![r.mover]).u32(), 0x0300);

        let out = SCRATCH + 0x100;
        put_point(&mut r.e, r.mover.addr() + 0x10, 1.0, 2.0, 3.0);
        assert!(!r.e.call(0x009d_dbb0, &args![r.mover, p(out)]).bool());
        r.e.set(r.mover, ActorMover::bHaveValidDeltaThisFrame, true);
        assert!(r.e.call(0x009d_dbb0, &args![r.mover, p(out)]).bool());
        assert_eq!(point(&r.e, out), [1.0, 2.0, 3.0]);
        assert!(!r.e.get(r.mover, ActorMover::bHaveValidDeltaThisFrame));
        assert!(!r.e.call(0x009d_dbb0, &args![r.mover, p(out)]).bool());
    }

    /// The delta in forced mode comes from the animation when there is one.
    #[test]
    fn move_delta_in_forced_mode_comes_from_the_animation() {
        for (form_flag, expected) in [(true, [1u32, 0]), (false, [0, 1])] {
            let mut r = rig();
            r.e.set(r.mover, ActorMover::bForceMoveMode, true);
            r.e.set(r.mover, ActorMover::uiForcedMoveMode, 0x0300);
            r.e.register(ACTOR_FAKE + 0x1e4, |_, _| ret(0x0600_0a00));
            let slot: AbiFn = if form_flag {
                |_, _| ret(1)
            } else {
                |_, _| ret(0)
            };
            r.e.register(ACTOR_FORM_FLAG_10, slot);
            r.e.register(ACTOR_MOVEMENT_TEST, |_, _| ret(1));
            r.e.register(ANIMATION_GET_DELTA, |e, a| {
                e.mem.set_f32(a[1], 1.0);
                e.mem.set_f32(a[1] + 4, 2.0);
                e.mem.set_f32(a[1] + 8, 3.0);
                ret(0)
            });
            r.e.call_log = Some(vec![]);
            r.e.call(0x009d_dc00, &args![r.mover]);
            let log = calls(&mut r.e);
            let call = first(&log, ANIMATION_GET_DELTA);
            assert_eq!(call.1[0], 0x0600_0a00);
            assert_eq!(
                call.1[2..].to_vec(),
                vec![r.actor, expected[0], expected[1]]
            );
            assert_eq!(point(&r.e, r.mover.addr() + 0x10), [1.0, 2.0, 3.0]);
            assert_eq!(
                r.e.get(r.mover, ActorMover::iCachedLastValidMoveMode),
                0x0300
            );
            assert!(r.e.get(r.mover, ActorMover::bHaveValidDeltaThisFrame));
            assert_eq!(log.last().unwrap().0, DEBUG_STUB);
        }
    }

    #[test]
    fn move_delta_in_forced_mode_without_animation_steps_forward() {
        let mut r = rig();
        r.e.set(r.mover, ActorMover::bForceMoveMode, true);
        r.e.register(ACTOR_FAKE + 0x1e4, |_, _| ret(0));
        r.e.register(ACTOR_TEST_FLAGS, |_, _| ret(0));
        r.e.register(ACTOR_GET_SPEED, |_, _| ret_float(2.0));
        r.e.register(GLOBAL_TIMER_FLOAT, |_, _| ret_float(0.5));
        r.e.register(POINT_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            e.mem.set_u32(a[0] + 8, a[3]);
            ret(a[0])
        });
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_dc00, &args![r.mover]);
        let log = calls(&mut r.e);
        assert_eq!(first(&log, ACTOR_TEST_FLAGS).1, vec![r.actor, 7]);
        assert_eq!(first(&log, GLOBAL_TIMER_FLOAT).1, vec![GLOBAL_TIMER_OBJECT]);
        assert_eq!(point(&r.e, r.mover.addr() + 0x10), [0.0, 1.0, 0.0]);
        assert!(r.e.get(r.mover, ActorMover::bHaveValidDeltaThisFrame));
        // With the test saying yes the delta stays at the unset point.
        r.e.register(ACTOR_TEST_FLAGS, |_, _| ret(1));
        put_point(&mut r.e, r.mover.addr() + 0x10, 9.0, 9.0, 9.0);
        r.e.call(0x009d_dc00, &args![r.mover]);
        assert_eq!(point(&r.e, r.mover.addr() + 0x10), [0.0, 0.0, 0.0]);
        assert!(r.e.get(r.mover, ActorMover::bHaveValidDeltaThisFrame));
    }

    #[test]
    fn move_delta_from_the_handler_masks_the_mode_bits() {
        for (movement_ok, slot_ok, animation, want) in [
            // 0x200 and 0x400 both fail their tests: 0x601 -> 0x101.
            (false, false, false, 0x101u32),
            // The 0x400 test passes but the cache differs and there is no
            // animation: 0x400 is dropped again.
            (true, true, false, 0x201),
            // The animation allows 0x400 (action -1, no table entries).
            (true, true, true, 0x601),
        ] {
            let mut r = rig();
            let handler = quiet_handler(&mut r);
            let _ = handler;
            r.e.register(HANDLER_FAKE + 0x48, |_, _| ret(0x0601));
            r.e.register(HANDLER_FAKE + 0x4c, |e, a| {
                e.mem.set_f32(a[1], 4.0);
                e.mem.set_f32(a[1] + 4, 5.0);
                e.mem.set_f32(a[1] + 8, 6.0);
                ret(1)
            });
            let movement: AbiFn = if movement_ok {
                |_, _| ret(1)
            } else {
                |_, _| ret(0)
            };
            r.e.register(ACTOR_MOVEMENT_TEST, movement);
            let slot: AbiFn = if slot_ok {
                |_, _| ret(1)
            } else {
                |_, _| ret(0)
            };
            r.e.register(ACTOR_SLOT_218, slot);
            let anim: AbiFn = if animation {
                |_, _| ret(0x0600_0a00)
            } else {
                |_, _| ret(0)
            };
            r.e.register(ACTOR_FAKE + 0x1e4, anim);
            r.e.register(ACTOR_GET_ANIM_ACTION, |_, _| ret(0xffff_ffff));
            r.e.call(0x009d_dc00, &args![r.mover]);
            let flags = r.e.get(r.mover, ActorMover::iCachedLastValidMoveMode);
            assert_eq!(flags, want);
            assert_eq!(point(&r.e, r.mover.addr() + 0x10), [4.0, 5.0, 6.0]);
            assert!(r.e.get(r.mover, ActorMover::bHaveValidDeltaThisFrame));
        }
    }

    #[test]
    fn move_delta_stops_the_move_animation_when_asked() {
        // The animation action is in 12..=14: the stop is done.
        let mut r = rig();
        quiet_handler(&mut r);
        r.e.register(HANDLER_FAKE + 0x48, |_, _| ret(0x15));
        r.e.register(HANDLER_FAKE + 0x4c, |_, _| ret(1));
        r.e.set(r.mover, ActorMover::bStopCurrentMoveAnim, true);
        r.e.register(ACTOR_GET_ANIM_ACTION, |_, _| ret(13));
        r.e.call(0x009d_dc00, &args![r.mover]);
        assert!(!r.e.get(r.mover, ActorMover::bStopCurrentMoveAnim));
        assert!(!r.e.get(r.mover, ActorMover::bHaveValidDeltaThisFrame));
        assert_eq!(r.e.get(r.mover, ActorMover::iCachedLastValidMoveMode), 0x10);
        assert_eq!(point(&r.e, r.mover.addr() + 0x10), [0.0, 0.0, 0.0]);

        // Another action while the animation still moves: kept going.
        for (animation, keeps) in [(0x0600_0a00u32, true), (0, false)] {
            let mut r = rig();
            quiet_handler(&mut r);
            r.e.set(r.mover, ActorMover::bStopCurrentMoveAnim, true);
            r.e.register(ACTOR_GET_ANIM_ACTION, |_, _| ret(5));
            let anim: AbiFn = if animation != 0 {
                |_, _| ret(0x0600_0a00)
            } else {
                |_, _| ret(0)
            };
            r.e.register(ACTOR_FAKE + 0x1e4, anim);
            r.e.register(ANIMATION_SHOULD_BE_MOVING, |_, _| ret(1));
            r.e.call(0x009d_dc00, &args![r.mover]);
            assert_eq!(r.e.get(r.mover, ActorMover::bStopCurrentMoveAnim), keeps);
            assert!(r.e.get(r.mover, ActorMover::bHaveValidDeltaThisFrame));
        }
        // The animation does not move any more: the stop is done.
        let mut r = rig();
        quiet_handler(&mut r);
        r.e.set(r.mover, ActorMover::bStopCurrentMoveAnim, true);
        r.e.register(ACTOR_GET_ANIM_ACTION, |_, _| ret(5));
        r.e.register(ACTOR_FAKE + 0x1e4, |_, _| ret(0x0600_0a00));
        r.e.register(ANIMATION_SHOULD_BE_MOVING, |_, _| ret(0));
        r.e.call(0x009d_dc00, &args![r.mover]);
        assert!(!r.e.get(r.mover, ActorMover::bStopCurrentMoveAnim));
    }

    #[test]
    fn mode_bit_0x400_is_cleared_when_an_animation_table_entry_disagrees() {
        let mut r = rig();
        quiet_handler(&mut r);
        r.e.register(HANDLER_FAKE + 0x48, |_, _| ret(0x0400));
        r.e.register(ACTOR_SLOT_218, |_, _| ret(1));
        r.e.register(ACTOR_FAKE + 0x1e4, |_, _| ret(0x0600_0a00));
        // Entry 0 exists and its value is not 1.
        r.e.register(ANIMATION_ELEMENT, |_, a| {
            ret(if a[1] == 0 { 0x77 } else { 0 })
        });
        r.e.register(ENTRY_VALUE, |_, _| ret(2));
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_dc00, &args![r.mover]);
        let log = calls(&mut r.e);
        assert_eq!(r.e.get(r.mover, ActorMover::iCachedLastValidMoveMode), 0);
        assert_eq!(first(&log, ENTRY_VALUE).1, vec![0x77]);
        // All entries 1 or missing, and an action other than -1: cleared.
        let mut r = rig();
        quiet_handler(&mut r);
        r.e.register(HANDLER_FAKE + 0x48, |_, _| ret(0x0400));
        r.e.register(ACTOR_SLOT_218, |_, _| ret(1));
        r.e.register(ACTOR_FAKE + 0x1e4, |_, _| ret(0x0600_0a00));
        r.e.register(ANIMATION_ELEMENT, |_, a| {
            ret(if a[1] == 1 { 0x77 } else { 0 })
        });
        r.e.register(ENTRY_VALUE, |_, _| ret(1));
        r.e.register(ACTOR_GET_ANIM_ACTION, |_, _| ret(3));
        r.e.call(0x009d_dc00, &args![r.mover]);
        assert_eq!(r.e.get(r.mover, ActorMover::iCachedLastValidMoveMode), 0);
    }

    #[test]
    fn move_mode_preference_sets_the_two_parts() {
        // (mode, handler present, preferred before, preferred after)
        for (mode, has_handler, before, after) in [
            (0x0102u32, false, 0x0101u32, 0x0102u32),
            (0x0200, false, 0x0101, 0x0201),
            (0x000f, true, 0x0101, 0x010f),
            (0x0000, false, 0x0101, 0x0101),
            (0x0300, true, 0x0f0f, 0x030f),
        ] {
            let mut r = rig();
            let handler = if has_handler {
                attach_handler(&mut r)
            } else {
                0
            };
            r.e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
            r.e.register(DETAILED_HANDLER_CONSTRUCT_SIMPLE, |_, a| ret(a[0]));
            r.e.set(r.mover, ActorMover::uiPreferredMoveMode, before);
            r.e.call_log = Some(vec![]);
            r.e.call(0x009d_df60, &args![r.mover, mode]);
            let log = calls(&mut r.e);
            assert_eq!(r.e.get(r.mover, ActorMover::uiPreferredMoveMode), after);
            let created = !has_handler && mode & 0xff00 != 0;
            assert_eq!(
                count(&log, DETAILED_HANDLER_CONSTRUCT_SIMPLE),
                created as usize
            );
            if created {
                let new = r.e.get(r.mover, ActorMover::pCurrentPathHandler).addr();
                assert_eq!(first(&log, OPERATOR_NEW).1, vec![0xe4]);
                assert_eq!(
                    first(&log, DETAILED_HANDLER_CONSTRUCT_SIMPLE).1,
                    vec![new, r.actor, r.mover.addr()]
                );
            } else if has_handler {
                assert_eq!(
                    r.e.get(r.mover, ActorMover::pCurrentPathHandler).addr(),
                    handler
                );
            }
        }
    }

    #[test]
    fn clear_and_force_move_mode() {
        let mut r = rig();
        r.e.set(r.mover, ActorMover::uiPreferredMoveMode, 0x01ff);
        r.e.call(0x009d_e040, &args![r.mover, 0x000fu32]);
        assert_eq!(r.e.get(r.mover, ActorMover::uiPreferredMoveMode), 0x01f0);
        r.e.call(0x009d_e060, &args![r.mover, 0x0200u32]);
        assert!(r.e.get(r.mover, ActorMover::bForceMoveMode));
        assert_eq!(r.e.get(r.mover, ActorMover::uiForcedMoveMode), 0x0200);
        r.e.call(0x009d_e080, &args![r.mover]);
        assert!(!r.e.get(r.mover, ActorMover::bForceMoveMode));
        assert_eq!(r.e.get(r.mover, ActorMover::uiForcedMoveMode), 0);
    }

    #[test]
    fn next_node_location_comes_from_the_handler_or_the_actor() {
        let mut r = goal_rig();
        put_point(&mut r.e, ACTOR_POSITION, 1.0, 2.0, 3.0);
        let out = SCRATCH + 0x100;
        let back = r.e.call(0x009d_e0a0, &args![r.mover, p(out)]).u32();
        assert_eq!(back, out);
        assert_eq!(point(&r.e, out), [1.0, 2.0, 3.0]);
        // A path but no handler still uses the actor.
        attach_path(&mut r);
        put_point(&mut r.e, out, 0.0, 0.0, 0.0);
        r.e.call(0x009d_e0a0, &args![r.mover, p(out)]);
        assert_eq!(point(&r.e, out), [1.0, 2.0, 3.0]);
        // Path and handler: the handler writes it.
        let handler = attach_handler(&mut r);
        r.e.register(HANDLER_FAKE + 0x20, |_, _| Ret::default());
        r.e.call_log = Some(vec![]);
        let back = r.e.call(0x009d_e0a0, &args![r.mover, p(out)]).u32();
        assert_eq!(back, out);
        assert_eq!(
            first(&calls(&mut r.e), HANDLER_FAKE + 0x20).1,
            vec![handler, out]
        );
    }

    /// A rig whose actor answers the position slot.
    fn goal_rig() -> Rig {
        let mut r = rig();
        r.e.register(ACTOR_FAKE + 0x1f4, |_, _| ret(ACTOR_POSITION));
        r
    }

    #[test]
    fn node_location_query_needs_a_path_and_a_handler() {
        let mut r = rig();
        assert!(!r.e.call(0x009d_e110, &args![r.mover, 1u32, 2u32]).bool());
        attach_path(&mut r);
        assert!(!r.e.call(0x009d_e110, &args![r.mover, 1u32, 2u32]).bool());
        let handler = attach_handler(&mut r);
        r.e.register(HANDLER_FAKE + 0x24, |_, _| ret(1));
        r.e.call_log = Some(vec![]);
        assert!(r.e.call(0x009d_e110, &args![r.mover, 1u32, 2u32]).bool());
        assert_eq!(
            first(&calls(&mut r.e), HANDLER_FAKE + 0x24).1,
            vec![handler, 1, 2]
        );
    }

    #[test]
    fn look_at_target_is_stored_and_a_handler_is_made_for_process_type_zero() {
        for (process_type, creates) in [(0u32, true), (1, false)] {
            let mut r = rig();
            r.e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
            r.e.register(DETAILED_HANDLER_CONSTRUCT_SIMPLE, |_, a| ret(a[0]));
            let kind: AbiFn = if process_type == 0 {
                |_, _| ret(0)
            } else {
                |_, _| ret(1)
            };
            r.e.register(GET_PROCESS_TYPE, kind);
            r.e.call_log = Some(vec![]);
            r.e.call(
                0x009d_e160,
                &args![r.mover, f32::from_bits(0x7fa0_0001), 2.0f32, -3.0f32],
            );
            let log = calls(&mut r.e);
            assert_eq!(r.e.mem.u32(r.mover.addr() + 4), 0x7fa0_0001);
            assert_eq!(r.e.mem.f32(r.mover.addr() + 8), 2.0);
            assert_eq!(r.e.mem.f32(r.mover.addr() + 12), -3.0);
            assert!(r.e.get(r.mover, ActorMover::bFaceTargetPoint));
            assert_eq!(count(&log, OPERATOR_NEW), creates as usize);
            assert_eq!(count(&log, DEBUG_STUB), creates as usize);
            assert_eq!(
                r.e.get(r.mover, ActorMover::pCurrentPathHandler).addr() != 0,
                creates
            );
        }
        // An existing handler is kept.
        let mut r = rig();
        let handler = attach_handler(&mut r);
        r.e.register(GET_PROCESS_TYPE, |_, _| ret(0));
        r.e.call(0x009d_e160, &args![r.mover, 1.0f32, 2.0f32, 3.0f32]);
        assert_eq!(
            r.e.get(r.mover, ActorMover::pCurrentPathHandler).addr(),
            handler
        );
    }

    #[test]
    fn clear_look_at_target_resets_the_point_and_the_flag() {
        let mut r = rig();
        r.e.set_global(UNSET_POINT + 8, 0x8000_0000u32);
        put_point(&mut r.e, r.mover.addr() + 4, 1.0, 2.0, 3.0);
        r.e.set(r.mover, ActorMover::bFaceTargetPoint, true);
        r.e.call(0x009d_e230, &args![r.mover]);
        assert!(!r.e.get(r.mover, ActorMover::bFaceTargetPoint));
        assert_eq!(r.e.mem.u32(r.mover.addr() + 4), 0);
        assert_eq!(r.e.mem.u32(r.mover.addr() + 12), 0x8000_0000);
    }

    #[test]
    fn look_at_target_query_copies_the_point_when_set() {
        let mut r = rig();
        r.e.register(FACE_TARGET_POINT, |e, a| ret(e.mem.u8(a[0] + 0x73) as u32));
        let out = SCRATCH + 0x100;
        put_point(&mut r.e, r.mover.addr() + 4, 1.0, 2.0, 3.0);
        put_point(&mut r.e, out, 9.0, 9.0, 9.0);
        assert!(!r.e.call(0x009d_e270, &args![r.mover, p(out)]).bool());
        assert_eq!(point(&r.e, out), [9.0, 9.0, 9.0]);
        r.e.set(r.mover, ActorMover::bFaceTargetPoint, true);
        assert!(r.e.call(0x009d_e270, &args![r.mover, p(out)]).bool());
        assert_eq!(point(&r.e, out), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn path_end_location_queries() {
        let mut r = rig();
        let out = SCRATCH + 0x100;
        // No path.
        assert!(!r.e.call(0x009d_e2b0, &args![r.mover, p(out)]).bool());
        assert!(!r.e.call(0x009d_e330, &args![r.mover, p(out)]).bool());
        // A usable path whose last node is missing.
        let path = attach_path(&mut r);
        r.e.register(PATH_LAST_NODE, |_, _| ret(0));
        assert!(!r.e.call(0x009d_e2b0, &args![r.mover, p(out)]).bool());
        assert!(!r.e.call(0x009d_e330, &args![r.mover, p(out)]).bool());
        // With the node: positions come out.
        r.e.register(PATH_LAST_NODE, |_, _| ret(SCRATCH + 0x800));
        r.e.register(NODE_LOCATION, |_, a| ret(a[0] + 4));
        r.e.register(GET_POSITION, |e, a| {
            for offset in [0, 4, 8] {
                let word = e.mem.u32(a[0] + 4 + offset);
                e.mem.set_u32(a[1] + offset, word);
            }
            ret(a[1])
        });
        put_point(&mut r.e, SCRATCH + 0x808, 7.0, 8.0, 9.0);
        r.e.call_log = Some(vec![]);
        assert!(r.e.call(0x009d_e2b0, &args![r.mover, p(out)]).bool());
        assert_eq!(point(&r.e, out), [7.0, 8.0, 9.0]);
        assert_eq!(first(&calls(&mut r.e), PATH_LAST_NODE).1, vec![path + 8]);
        r.e.call_log = Some(vec![]);
        assert!(r.e.call(0x009d_e330, &args![r.mover, p(out)]).bool());
        assert_eq!(
            first(&calls(&mut r.e), LOCATION_ASSIGN).1,
            vec![out, SCRATCH + 0x804]
        );
        // An unusable path.
        r.e.mem.set_u32(path + 0x10, 0);
        assert!(!r.e.call(0x009d_e2b0, &args![r.mover, p(out)]).bool());
        assert!(!r.e.call(0x009d_e330, &args![r.mover, p(out)]).bool());
    }

    #[test]
    fn door_id_comes_from_the_process() {
        let mut r = rig();
        r.e.register(PROCESS_DOOR_ID, |_, a| ret(a[0] + 1));
        r.e.set(r.mover, ActorMover::DetectionDoorID, 77);
        r.e.call(0x009d_e3a0, &args![r.mover, p(0)]);
        assert_eq!(r.e.get(r.mover, ActorMover::DetectionDoorID), 0);
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_e3a0, &args![r.mover, p(0x500)]);
        assert_eq!(r.e.get(r.mover, ActorMover::DetectionDoorID), 0x501);
        assert_eq!(first(&calls(&mut r.e), PROCESS_DOOR_ID).1, vec![0x500]);
    }
    // The functions from `009de3e0` on.

    const PLAYER_TABLE: u32 = 0x0306_0000;
    const PLAYER_FAKE: u32 = 0x0506_0000;
    const BUFFER_TABLE: u32 = 0x0307_0000;
    const BUFFER_FAKE: u32 = 0x0507_0000;
    const LOCATION_TABLE: u32 = 0x0308_0000;
    const LOCATION_FAKE: u32 = 0x0508_0000;
    /// Two request locations; the position doubles read from +0x100.
    const END_A: u32 = SCRATCH + 0x400;
    const END_B: u32 = SCRATCH + 0x500;

    fn clock(e: &mut Engine, start: u32) -> Rc<Cell<u32>> {
        let now = Rc::new(Cell::new(start));
        let reading = now.clone();
        e.register_double(TIMER_NOW, move |_, _| ret(reading.get()));
        now
    }

    /// The request's two ends at heights `z_a` and `z_b`, the water at
    /// `height`, `cell` the cell both ends report.
    fn water_request(r: &mut Rig, z_a: f32, z_b: f32, height: f64, cell: u32) {
        attach_request(r);
        r.e.register(REQUEST_ORIGIN, |_, _| ret(END_A));
        r.e.register(REQUEST_DESTINATION, |_, _| ret(END_B));
        put_point(&mut r.e, END_A + 0x100, 1.0, 2.0, z_a);
        put_point(&mut r.e, END_B + 0x100, 3.0, 4.0, z_b);
        r.e.register(GET_POSITION, |e, a| {
            copy_point(e, a[0] + 0x100, a[1]);
            ret(a[1])
        });
        r.e.register_double(LOCATION_GET_CELL, move |_, _| ret(cell));
        r.e.register_double(WATER_HEIGHT_AT, move |_, _| ret_float(height));
    }

    /// One failure at `now`; returns the count and the two time stamps.
    fn failure_step(now: u32, last: u32, first: u32, count: i32, hidden: f32) -> (i32, u32, u32) {
        let mut r = rig();
        r.e.set_global(FAILURE_TIMEOUT_HIDDEN, hidden);
        r.e.set_global(MILLISECONDS_TO_SECONDS, 0.001f64);
        clock(&mut r.e, now);
        r.e.set(r.mover, ActorMover::iLastFailureTimeStamp, last);
        r.e.set(r.mover, ActorMover::iFirstFailureTimeStamp, first);
        r.e.set(r.mover, ActorMover::iContinuedFailureCount, count);
        actor_mover_set_pathing_failed(&mut r.e, r.mover, true);
        assert!(r.e.get(r.mover, ActorMover::bPathingFailed));
        (
            r.e.get(r.mover, ActorMover::iContinuedFailureCount),
            r.e.get(r.mover, ActorMover::iLastFailureTimeStamp),
            r.e.get(r.mover, ActorMover::iFirstFailureTimeStamp),
        )
    }

    #[test]
    fn path_validity_queries() {
        let mut r = rig();
        assert!(!r.e.call(0x009d_e3e0, &args![r.mover]).bool());
        assert!(!r.e.call(0x009d_e420, &args![r.mover]).bool());
        let path = attach_path(&mut r);
        assert!(r.e.call(0x009d_e3e0, &args![r.mover]).bool());
        r.e.register(PATH_FLAG_40, |e, a| ret(e.mem.u8(a[0] + 0x40) as u32));
        assert!(!r.e.call(0x009d_e420, &args![r.mover]).bool());
        r.e.mem.set_u8(path + 0x40, 1);
        assert!(r.e.call(0x009d_e420, &args![r.mover]).bool());
        // A path the validity test refuses.
        r.e.mem.set_u32(path + 0x10, 0);
        assert!(!r.e.call(0x009d_e3e0, &args![r.mover]).bool());
    }

    #[test]
    fn failed_destination_is_hidden_by_its_flag() {
        let mut r = rig();
        r.e.register(LOCATION_FLAG_2, |_, _| ret(0));
        r.e.call_log = Some(vec![]);
        let location = r.mover.addr() + 0x44;
        assert_eq!(r.e.call(0x009d_e460, &args![r.mover]).u32(), location);
        assert_eq!(first(&calls(&mut r.e), LOCATION_FLAG_2).1, vec![location]);
        r.e.register(LOCATION_FLAG_2, |_, _| ret(1));
        assert_eq!(r.e.call(0x009d_e460, &args![r.mover]).u32(), 0);
    }

    #[test]
    fn clearing_the_failure_only_resets_the_location() {
        let mut r = rig();
        r.e.set(r.mover, ActorMover::bPathingFailed, true);
        r.e.call_log = Some(vec![]);
        actor_mover_set_pathing_failed(&mut r.e, r.mover, false);
        let log = calls(&mut r.e);
        assert!(!r.e.get(r.mover, ActorMover::bPathingFailed));
        assert_eq!(log, vec![(LOCATION_RESET, vec![r.mover.addr() + 0x44])]);
    }

    #[test]
    fn failure_counting_follows_the_time_out() {
        // Hidden time-out 1.5 s. A failure 2.0 s after the last restarts.
        assert_eq!(failure_step(3000, 1000, 1000, 1, 1.5), (0, 3000, 3000));
        // 1.0 s after both: counted, the first stamp stays.
        assert_eq!(failure_step(2000, 1000, 1000, 1, 1.5), (2, 2000, 1000));
        // Recent, but the first is older than the time-out and two were
        // counted: the recovery runs (no request: it returns at once) and
        // everything restarts.
        assert_eq!(failure_step(3000, 2900, 1000, 2, 1.5), (0, 3000, 3000));
        // The same with one counted failure only: counted.
        assert_eq!(failure_step(3000, 2900, 1000, 1, 1.5), (2, 3000, 1000));
        // A time-out that is NaN fails every comparison: counted.
        assert_eq!(failure_step(3000, 1000, 1000, 0, f32::NAN), (1, 3000, 1000));
        // The 32-bit difference is unsigned and wraps: 0x200 ms.
        assert_eq!(
            failure_step(0x100, 0xffff_ff00, 0xffff_ff00, 0, 0.6),
            (1, 0x100, 0xffff_ff00)
        );
        assert_eq!(
            failure_step(0x100, 0xffff_ff00, 0xffff_ff00, 0, 0.25),
            (0, 0x100, 0x100)
        );
    }

    #[test]
    fn failure_time_out_depends_on_visibility_and_water() {
        // Visible: 3.0 s, so 2.0 s after the last only counts.
        let mut r = rig();
        r.e.set_global(FAILURE_TIMEOUT_VISIBLE, 3.0f32);
        r.e.set_global(FAILURE_TIMEOUT_HIDDEN, 1.5f32);
        r.e.set_global(MILLISECONDS_TO_SECONDS, 0.001f64);
        r.e.register(ACTOR_TEST_FLAGS, |_, _| ret(1));
        clock(&mut r.e, 3000);
        r.e.set(r.mover, ActorMover::iLastFailureTimeStamp, 1000);
        r.e.set(r.mover, ActorMover::iFirstFailureTimeStamp, 1000);
        r.e.call_log = Some(vec![]);
        actor_mover_set_pathing_failed(&mut r.e, r.mover, true);
        assert_eq!(
            first(&calls(&mut r.e), ACTOR_TEST_FLAGS).1,
            vec![r.actor, 7]
        );
        assert_eq!(r.e.get(r.mover, ActorMover::iContinuedFailureCount), 1);
        // A request whose ends lie on different sides of the water: 1.0 s,
        // so 1.2 s after the last restarts; on the same side it only counts.
        for (z_b, restarts) in [(5.0f32, true), (0.5, false), (f32::NAN, true)] {
            let mut r = rig();
            r.e.set_global(FAILURE_TIMEOUT_HIDDEN, 1.5f32);
            r.e.set_global(FAILURE_TIMEOUT_WATER, 1.0f32);
            r.e.set_global(MILLISECONDS_TO_SECONDS, 0.001f64);
            water_request(&mut r, 0.0, z_b, 2.0, 0x11);
            clock(&mut r.e, 2200);
            r.e.set(r.mover, ActorMover::iLastFailureTimeStamp, 1000);
            r.e.set(r.mover, ActorMover::iFirstFailureTimeStamp, 1000);
            r.e.call_log = Some(vec![]);
            actor_mover_set_pathing_failed(&mut r.e, r.mover, true);
            let log = calls(&mut r.e);
            let expected_count = if restarts { 0 } else { 1 };
            assert_eq!(
                r.e.get(r.mover, ActorMover::iContinuedFailureCount),
                expected_count,
                "z {z_b}"
            );
            // The height is asked of the TES for the position and the cell.
            assert_eq!(count(&log, WATER_HEIGHT_AT), 2);
            assert_eq!(first(&log, WATER_HEIGHT_AT).1[2], 0x11);
        }
        // Without a cell on one end the water is not looked at.
        let mut r = rig();
        r.e.set_global(FAILURE_TIMEOUT_HIDDEN, 1.5f32);
        water_request(&mut r, 0.0, 5.0, 2.0, 0);
        r.e.call_log = Some(vec![]);
        actor_mover_set_pathing_failed(&mut r.e, r.mover, true);
        assert_eq!(count(&calls(&mut r.e), WATER_HEIGHT_AT), 0);
    }

    #[test]
    fn a_stuck_actor_on_the_grid_origin_is_placed_at_the_destination() {
        // (process type, cell is flagged, x, y, info map answer, places)
        for (kind, flagged, x, y, found, places) in [
            (3u32, false, 0.0f32, 0.0f32, 0u32, true),
            (3, false, 0.0, 0.0, 9, false),
            (3, true, 0.0, 0.0, 0, false),
            (3, false, 4096.0, 0.0, 0, false),
            (3, false, 0.0, 4096.0, 0, false),
            (2, false, 0.0, 0.0, 0, false),
        ] {
            let mut r = rig();
            attach_request(&mut r);
            r.e.register(FTOL, |_, a| {
                let value = f64::from_bits(a[0] as u64 | (a[1] as u64) << 32);
                ret(value as i32 as u32)
            });
            r.e.register_double(GET_PROCESS_TYPE, move |_, _| ret(kind));
            r.e.register(GET_PARENT_CELL, |_, _| ret(0x40));
            r.e.register_double(CELL_FLAG_0, move |_, _| ret(flagged as u32));
            r.e.register(ACTOR_FAKE + 0x1f4, |_, _| ret(ACTOR_POSITION));
            put_point(&mut r.e, ACTOR_POSITION, x, y, 7.0);
            r.e.register(PACK_GRID_COORDS, |_, _| ret(0x77));
            r.e.register(GET_WORLD_SPACE, |_, _| ret(0x66));
            r.e.register(NAV_MESH_INFO_MAP, |_, _| ret(0x88));
            r.e.register_double(NAV_MESH_INFO_FIND, move |_, _| ret(found));
            r.e.register(REQUEST_DESTINATION, |_, _| ret(END_B));
            r.e.mem.set_u32(END_B + 0x1c, 0x1234);
            r.e.register(DWORD_AT_18, |_, _| ret(0x18));
            r.e.register(GET_POSITION, |_, a| ret(a[1]));
            r.e.set_global(TES_POINTER, 0x7777u32);
            r.e.set_global(FAILURE_TIMEOUT_HIDDEN, 1.5f32);
            r.e.call_log = Some(vec![]);
            actor_mover_set_pathing_failed(&mut r.e, r.mover, true);
            let log = calls(&mut r.e);
            assert_eq!(
                count(&log, ACTOR_PLACE),
                places as usize,
                "{kind} {flagged} {x} {y}"
            );
            if places {
                let place = first(&log, ACTOR_PLACE);
                assert_eq!(place.1[0], r.actor);
                assert_eq!(place.1[2..], [0, 0x18, 0x1234, 0, 1]);
                assert_eq!(first(&log, PACK_GRID_COORDS).1, vec![0, 0]);
                assert_eq!(first(&log, NAV_MESH_INFO_FIND).1, vec![0x88, 0x66, 0x77]);
                assert_eq!(first(&log, NAV_MESH_INFO_MAP).1, vec![0x7777]);
            }
        }
    }

    #[test]
    fn move_actor_to_location_reports_or_places_the_actor() {
        let position = SCRATCH + 0x300;
        let player = SCRATCH + 0x700;
        // The global test passes: the two reports, not the actor's slot.
        // The actor has no world space: done.
        let mut r = rig();
        r.e.register(GLOBAL_TEST, |_, _| ret(1));
        r.e.register(REPORT_TARGET_OBJECT, |_, _| ret(0x888));
        r.e.call_log = Some(vec![]);
        assert!(r.e.call(0x009d_f010, &args![r.mover, p(position)]).bool());
        let log = calls(&mut r.e);
        assert_eq!(
            first(&log, REPORT_PATH_END).1,
            vec![0x888, r.actor, position]
        );
        assert_eq!(first(&log, REPORT_MOVE_DONE).1, vec![0x888, r.actor]);
        assert_eq!(
            first(&log, SET_LOCATION_ON_REFERENCE).1,
            vec![r.actor, position]
        );
        assert_eq!(count(&log, GET_PARENT_CELL), 0);
        // Not the global test: the actor's slot 0x2A8.
        let mut r = rig();
        r.e.register(ACTOR_FAKE + 0x2a8, |_, _| Ret::default());
        r.e.call_log = Some(vec![]);
        assert!(r.e.call(0x009d_f010, &args![r.mover, p(position)]).bool());
        let log = calls(&mut r.e);
        assert_eq!(first(&log, ACTOR_FAKE + 0x2a8).1, vec![r.actor, position]);
        assert_eq!(count(&log, REPORT_PATH_END), 0);
        // With a world space: moved only when the cell differs; the result
        // is true for a nonzero desired level or a sleeping player.
        for (cell, cell_at, level, sleeping, moves, expected) in [
            (0x10u32, 0x10u32, 0u32, 0u32, false, false),
            (0x10, 0x20, 0, 0, true, false),
            (0x10, 0x10, 2, 0, false, true),
            (0x10, 0x10, 0, 1, false, true),
        ] {
            let mut r = rig();
            r.e.register(ACTOR_FAKE + 0x2a8, |_, _| Ret::default());
            r.e.register(GET_WORLD_SPACE, |_, _| ret(0x66));
            r.e.register_double(GET_PARENT_CELL, move |_, _| ret(cell));
            r.e.register_double(GET_CELL_FROM_WORLD_COORD, move |_, _| ret(cell_at));
            r.e.register_double(GET_DESIRED_PROCESS_LEVEL, move |_, _| ret(level));
            r.e.register_double(PLAYER_IS_SLEEPING, move |_, _| ret(sleeping));
            r.e.set_global(PLAYER_CHARACTER, player);
            r.e.call_log = Some(vec![]);
            let result = r.e.call(0x009d_f010, &args![r.mover, p(position)]).bool();
            let log = calls(&mut r.e);
            assert_eq!(result, expected);
            assert_eq!(count(&log, MOVE_REF_TO_NEW_SPACE), moves as usize);
            assert_eq!(
                first(&log, GET_CELL_FROM_WORLD_COORD).1,
                vec![0x66, position]
            );
            if moves {
                assert_eq!(first(&log, MOVE_REF_TO_NEW_SPACE).1, vec![r.actor, 0, 0x66]);
            }
            if sleeping == 1 {
                assert_eq!(first(&log, PLAYER_IS_SLEEPING).1, vec![player]);
            }
        }
    }

    #[test]
    fn ignore_smoothing_setter_stores_a_byte() {
        let mut e = world();
        let request = e.mem.alloc(0xb0);
        e.call(0x009d_eff0, &args![request, 1u32]);
        assert_eq!(e.mem.u8(request + 0xa3), 1);
        e.call(0x009d_eff0, &args![request, 0x100u32]);
        assert_eq!(e.mem.u8(request + 0xa3), 0);
    }

    #[test]
    fn array_last_element_asks_for_count_minus_one() {
        let mut e = world();
        let array = e.mem.alloc(16);
        e.mem.set_u32(array + 8, 4);
        e.register(ARRAY_ELEMENT_BY_INDEX, |_, a| ret(0x9000 + a[1]));
        e.call_log = Some(vec![]);
        assert_eq!(fn_009dfd50(&mut e, array), 0x9003);
        assert_eq!(
            first(&calls(&mut e), ARRAY_ELEMENT_BY_INDEX).1,
            vec![array, 3]
        );
    }

    #[test]
    fn revert_resets_everything_the_constructor_resets() {
        let mut r = rig();
        let handler = attach_handler(&mut r);
        r.e.register(HANDLER_FAKE, |_, _| Ret::default());
        r.e.set_global(UNSET_POINT + 8, 0x8000_0000u32);
        for offset in (4..0x88).step_by(4) {
            r.e.mem.set_u32(r.mover.addr() + offset, 0xdead_beef);
        }
        r.e.mem.set_u32(r.mover.addr() + 0x1c, SCRATCH);
        r.e.mem.set_u32(r.mover.addr() + 0x20, SCRATCH);
        r.e.mem.set_u32(r.mover.addr() + 0x24, handler);
        r.e.set(r.mover, ActorMover::bWaitingForPath, true);
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_fb80, &args![r.mover, 0u32]);
        let log = calls(&mut r.e);
        let m = r.mover;
        assert_eq!(first(&log, HANDLER_FAKE).1, vec![handler, 1]);
        assert_eq!(count(&log, QUEUE_POINTER_SET), 1);
        assert_eq!(first(&log, LOCATION_RESET).1, vec![m.addr() + 0x44]);
        assert_eq!(r.e.mem.u32(m.addr() + 0x1c), 0);
        assert_eq!(r.e.mem.u32(m.addr() + 0x20), 0);
        assert_eq!(r.e.mem.u32(m.addr() + 0x24), 0);
        assert_eq!(r.e.get(m, ActorMover::eState), 0);
        assert_eq!(r.e.get(m, ActorMover::uiPreferredMoveMode), 0x101);
        assert_eq!(r.e.get(m, ActorMover::iPreviousNode), 0xffff);
        assert_eq!(r.e.get(m, ActorMover::iNextNode), 0xffff);
        assert!(!r.e.get(m, ActorMover::bPathingFailed));
        for flag in [
            ActorMover::bActorChangedProcessLevel,
            ActorMover::bStopMovementNextUpdate,
            ActorMover::bWaitingForPath,
            ActorMover::bInDialog,
            ActorMover::bFaceTargetPoint,
            ActorMover::bForceMoveMode,
            ActorMover::bStopCurrentMoveAnim,
            ActorMover::bHaveValidDeltaThisFrame,
        ] {
            assert!(!r.e.get(m, flag));
        }
        assert_eq!(r.e.mem.u32(m.addr() + 0x0c), 0x8000_0000);
        assert_eq!(r.e.mem.u32(m.addr() + 0x18), 0x8000_0000);
        for field in [0x38, 0x3c, 0x2c, 0x84, 0x7c, 0x80] {
            assert_eq!(r.e.mem.u32(m.addr() + field), 0, "{field:x}");
        }
        // Not waiting for a path: the queue stays, a missing handler is fine.
        let mut r = rig();
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_fb80, &args![r.mover, 0u32]);
        assert_eq!(count(&calls(&mut r.e), QUEUE_POINTER_SET), 0);
    }

    /// A mover whose request ends at `END_B`, whose positions come from
    /// +0x100 of a location and whose actor's slot 0x2A8 records the
    /// position it is given.
    fn stuck_rig(process_type: u32) -> (Rig, Rc<RefCell<Vec<[f32; 3]>>>) {
        let mut r = rig();
        attach_request(&mut r);
        r.e.register_double(GET_PROCESS_TYPE, move |_, _| ret(process_type));
        r.e.register(REQUEST_DESTINATION, |_, _| ret(END_B));
        r.e.register(GET_POSITION, |e, a| {
            copy_point(e, a[0] + 0x100, a[1]);
            ret(a[1])
        });
        let placed = Rc::new(RefCell::new(Vec::new()));
        let record = placed.clone();
        r.e.register_double(ACTOR_FAKE + 0x2a8, move |e, a| {
            record
                .borrow_mut()
                .push([e.mem.f32(a[1]), e.mem.f32(a[1] + 4), e.mem.f32(a[1] + 8)]);
            Ret::default()
        });
        r.e.set_global(FAILURE_TARGET_RADIUS, 10.0f32);
        r.e.set_global(ACTOR_FLOAT_SCALE, 0.6f64);
        r.e.set_global(PATH_MARGIN_SHORT, 128.0f32);
        r.e.set_global(PATH_MARGIN_LONG, 256.0f32);
        r.e.set_global(HALF, 0.5f64);
        r.e.set_global(HUNDRED, 100.0f64);
        r.e.register(ACTOR_FLOAT, |_, _| ret_float(30.0));
        (r, placed)
    }

    fn built(log: &[Call]) -> bool {
        count(log, REQUEST_CONSTRUCT) == 1
    }

    #[test]
    fn recovery_does_nothing_for_a_warping_handler_or_without_a_request() {
        // The handler holds a path and warps to its next node.
        let mut r = rig();
        let handler = attach_handler(&mut r);
        r.e.mem.set_u32(handler + 0x0c, 1);
        attach_request(&mut r);
        r.e.register(HANDLER_FAKE + 0x74, |_, _| ret(1));
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_e8b0, &args![r.mover]);
        let log = calls(&mut r.e);
        assert_eq!(first(&log, DEBUG_STUB).1, vec![r.mover.addr()]);
        assert!(!built(&log));
        // The handler cannot warp, and there is no request.
        let mut r = rig();
        let handler = attach_handler(&mut r);
        r.e.mem.set_u32(handler + 0x0c, 1);
        r.e.register(HANDLER_FAKE + 0x74, |_, _| ret(0));
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_e8b0, &args![r.mover]);
        let log = calls(&mut r.e);
        assert_eq!(count(&log, DEBUG_STUB), 0);
        assert_eq!(count(&log, GET_PROCESS_TYPE), 0);
    }

    #[test]
    fn recovery_needs_a_stuck_actor() {
        // Process type 1, not following: nothing to do.
        let (mut r, _) = stuck_rig(1);
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_e8b0, &args![r.mover]);
        assert!(!built(&calls(&mut r.e)));
        // Process type 0 with a resolved location and the ends on one side
        // of the water: nothing either.
        let (mut r, _) = stuck_rig(0);
        r.e.register(LOCATION_IS_RESOLVED, |_, _| ret(1));
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_e8b0, &args![r.mover]);
        let log = calls(&mut r.e);
        assert!(!built(&log));
        assert_eq!(count(&log, LOCATION_DESTRUCT), 1);
        assert_eq!(first(&log, LOCATION_FROM_ACTOR).1[1..], [r.actor]);
        assert_eq!(first(&log, LOCATION_RESOLVE_NAV_MESH).1[1..], [0]);
        // An unresolved location makes it stuck.
        let (mut r, _) = stuck_rig(0);
        r.e.register(LOCATION_IS_RESOLVED, |_, _| ret(0));
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_e8b0, &args![r.mover]);
        assert!(built(&calls(&mut r.e)));
        // The ends on different sides of the water make it stuck.
        let (mut r, _) = stuck_rig(0);
        water_request(&mut r, 0.0, 5.0, 2.0, 0x11);
        r.e.register(LOCATION_IS_RESOLVED, |_, _| ret(1));
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_e8b0, &args![r.mover]);
        assert!(built(&calls(&mut r.e)));
    }

    #[test]
    fn recovery_builds_a_request_and_places_the_actor_at_the_first_node() {
        let (mut r, placed) = stuck_rig(0);
        let request_bytes = Rc::new(Cell::new(0xffu32));
        let request_byte = request_bytes.clone();
        r.e.register_double(REQUEST_DESTRUCT, move |e, a| {
            request_byte.set(e.mem.u8(a[0] + 0xa3) as u32);
            Ret::default()
        });
        r.e.register(ACTOR_BASE, |_, _| ret(0x5000));
        r.e.register(BASE_CAN_SWIM, |_, _| ret(1));
        r.e.register(PATH_COUNT_20, |_, _| ret(3));
        r.e.register(PATH_SECOND_ARRAY_ELEMENT, |_, _| ret(END_A - 4));
        r.e.register(NODE_LOCATION, |_, a| ret(a[0] + 4));
        put_point(&mut r.e, END_A + 0x100, 11.0, 12.0, 13.0);
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_e8b0, &args![r.mover]);
        let log = calls(&mut r.e);
        let request = first(&log, REQUEST_CONSTRUCT).1[0];
        let path = first(&log, PATH_CONSTRUCT).1[0];
        assert_eq!(
            order(
                &log,
                &[
                    REQUEST_CONSTRUCT,
                    REQUEST_SET_LOCATION,
                    REQUEST_SET_DESTINATION,
                    REQUEST_SET_TARGET_RADIUS,
                    REQUEST_SET_SECOND_FLOAT,
                    REQUEST_SET_ACTOR,
                    REQUEST_SET_BYTE_A,
                    REQUEST_SET_CANT_OPEN_DOORS,
                    REQUEST_SET_CAN_SWIM,
                    REQUEST_SET_CAN_FLY,
                    REQUEST_SET_BYTE_B,
                    POINT_CONSTRUCT_DEFAULT,
                    PATH_CONSTRUCT,
                    SOLVE_REQUEST,
                    DEBUG_STUB,
                    PATH_DESTRUCT,
                    REQUEST_DESTRUCT,
                ]
            ),
            vec![
                REQUEST_CONSTRUCT,
                REQUEST_SET_LOCATION,
                REQUEST_SET_DESTINATION,
                REQUEST_SET_TARGET_RADIUS,
                REQUEST_SET_SECOND_FLOAT,
                REQUEST_SET_ACTOR,
                REQUEST_SET_BYTE_A,
                REQUEST_SET_CANT_OPEN_DOORS,
                REQUEST_SET_CAN_SWIM,
                REQUEST_SET_CAN_FLY,
                REQUEST_SET_BYTE_B,
                POINT_CONSTRUCT_DEFAULT,
                PATH_CONSTRUCT,
                SOLVE_REQUEST,
                DEBUG_STUB,
                PATH_DESTRUCT,
                REQUEST_DESTRUCT,
            ]
        );
        assert_eq!(first(&log, REQUEST_SET_LOCATION).1, vec![request, END_B]);
        assert_eq!(
            first(&log, REQUEST_SET_TARGET_RADIUS).1,
            vec![request, 10.0f32.to_bits()]
        );
        assert_eq!(
            first(&log, REQUEST_SET_SECOND_FLOAT).1,
            vec![request, ((30.0f64 * 0.6f64) as f32).to_bits()]
        );
        assert_eq!(first(&log, REQUEST_SET_ACTOR).1, vec![request, r.actor]);
        assert_eq!(first(&log, REQUEST_SET_BYTE_A).1, vec![request, 1]);
        assert_eq!(first(&log, REQUEST_SET_BYTE_B).1, vec![request, 1]);
        assert_eq!(first(&log, REQUEST_SET_CANT_OPEN_DOORS).1, vec![request, 0]);
        assert_eq!(first(&log, REQUEST_SET_CAN_SWIM).1, vec![request, 1]);
        assert_eq!(first(&log, REQUEST_SET_CAN_FLY).1, vec![request, 0]);
        assert_eq!(first(&log, BASE_CAN_SWIM).1, vec![0x5000]);
        assert_eq!(first(&log, SOLVE_REQUEST).1, vec![request, path]);
        assert_eq!(request_bytes.get(), 1);
        assert_eq!(*placed.borrow(), vec![[11.0, 12.0, 13.0]]);
        assert_eq!(count(&log, MOVE_REF_TO_NEW_SPACE), 0);
        // An unsolved, empty path places nothing.
        let (mut r, placed) = stuck_rig(0);
        r.e.register(PATH_COUNT_20, |_, _| ret(0));
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_e8b0, &args![r.mover]);
        let log = calls(&mut r.e);
        assert_eq!(count(&log, DEBUG_STUB), 0);
        assert_eq!(count(&log, PATH_DESTRUCT), 1);
        assert_eq!(count(&log, REQUEST_DESTRUCT), 1);
        assert!(placed.borrow().is_empty());
    }

    #[test]
    fn recovery_places_the_actor_short_of_a_solved_paths_end() {
        // (actor float, path length, point asked for)
        for (actor_float, length, asked) in [
            (30.0f64, 1000.0f32, 872.0f32),
            (300.0, 1000.0, 744.0),
            (30.0, 100.0, 50.0),
            (202.0, 128.0, 64.0),
        ] {
            let (mut r, placed) = stuck_rig(0);
            r.e.register_double(ACTOR_FLOAT, move |_, _| ret_float(actor_float));
            r.e.register(SOLVE_REQUEST, |_, _| ret(1));
            r.e.register(PATH_COUNT_20, |_, _| ret(2));
            r.e.register_double(PATH_LENGTH, move |_, _| ret_float(length as f64));
            r.e.register(PATH_POINT_AT, |e, a| {
                put_point(e, a[1], 4.0, 5.0, 6.0);
                ret(a[1])
            });
            r.e.call_log = Some(vec![]);
            r.e.call(0x009d_e8b0, &args![r.mover]);
            let log = calls(&mut r.e);
            let at = first(&log, PATH_POINT_AT);
            assert_eq!(at.1[0], first(&log, PATH_CONSTRUCT).1[0]);
            assert_eq!(at.1[2..], [0, asked.to_bits()], "{actor_float} {length}");
            assert_eq!(*placed.borrow(), vec![[4.0, 5.0, 6.0]]);
        }
        // A solved path of one node only: nothing is placed.
        let (mut r, placed) = stuck_rig(0);
        r.e.register(SOLVE_REQUEST, |_, _| ret(1));
        r.e.register(PATH_COUNT_20, |_, _| ret(1));
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_e8b0, &args![r.mover]);
        let log = calls(&mut r.e);
        assert_eq!(count(&log, PATH_POINT_AT), 0);
        assert_eq!(count(&log, PATH_DESTRUCT), 1);
        assert!(placed.borrow().is_empty());
    }

    #[test]
    fn a_follower_is_stuck_when_far_from_the_player() {
        // (twice the radius, distance, stuck)
        for (radius, distance, stuck) in [
            (30.0f64, 100.0f64, true),
            (60.0, 100.0, false),
            (50.0, 100.0, false),
        ] {
            let (mut r, _) = stuck_rig(1);
            let player = object(&mut r.e, PLAYER_TABLE, PLAYER_FAKE);
            r.e.set_global(PLAYER_CHARACTER, player);
            r.e.register(IS_FOLLOWING, |_, _| ret(1));
            r.e.register(PLAYER_FAKE + 0x1f4, |_, _| ret(SCRATCH + 0x900));
            r.e.register(ACTOR_FAKE + 0x1f4, |_, _| ret(ACTOR_POSITION));
            r.e.register(POINT_SUBTRACT, |_, a| ret(a[1]));
            r.e.register_double(DIFFERENCE_LENGTH, move |_, _| ret_float(distance));
            r.e.register_double(REQUEST_TARGET_RADIUS, move |_, _| ret_float(radius));
            r.e.register(GET_WORLD_SPACE, |_, a| ret(a[0] & 0xff));
            r.e.call_log = Some(vec![]);
            r.e.call(0x009d_e8b0, &args![r.mover]);
            let log = calls(&mut r.e);
            assert_eq!(built(&log), stuck, "{radius}");
            assert_eq!(
                first(&log, POINT_SUBTRACT).1,
                vec![
                    ACTOR_POSITION,
                    first(&log, POINT_SUBTRACT).1[1],
                    SCRATCH + 0x900
                ]
            );
            assert_eq!(first(&log, IS_FOLLOWING).1, vec![r.actor, player]);
        }
    }

    #[test]
    fn a_follower_in_another_world_space_is_moved_to_the_players_space() {
        for same_space in [false, true] {
            let (mut r, placed) = stuck_rig(1);
            let player = object(&mut r.e, PLAYER_TABLE, PLAYER_FAKE);
            r.e.set_global(PLAYER_CHARACTER, player);
            r.e.register(IS_FOLLOWING, |_, _| ret(1));
            r.e.register(PLAYER_FAKE + 0x1f4, |_, _| ret(SCRATCH + 0x900));
            r.e.register(ACTOR_FAKE + 0x1f4, |_, _| ret(ACTOR_POSITION));
            r.e.register(POINT_SUBTRACT, |_, a| ret(a[1]));
            r.e.register_double(DIFFERENCE_LENGTH, |_, _| ret_float(1000.0));
            r.e.register_double(REQUEST_TARGET_RADIUS, |_, _| ret_float(1.0));
            let actor = r.actor;
            r.e.register_double(GET_WORLD_SPACE, move |_, a| {
                ret(if a[0] == actor || same_space {
                    0x66
                } else {
                    0x77
                })
            });
            r.e.register(GET_PARENT_CELL, |_, _| ret(0x44));
            r.e.register(GET_CELL_FROM_WORLD_COORD, |_, _| ret(0x44));
            r.e.register(SOLVE_REQUEST, |_, _| ret(1));
            r.e.register(PATH_COUNT_20, |_, _| ret(2));
            r.e.register(PATH_LENGTH, |_, _| ret_float(1000.0));
            r.e.register(PATH_POINT_AT, |e, a| {
                put_point(e, a[1], 4.0, 5.0, 6.0);
                ret(a[1])
            });
            r.e.call_log = Some(vec![]);
            r.e.call(0x009d_e8b0, &args![r.mover]);
            let log = calls(&mut r.e);
            assert_eq!(*placed.borrow(), vec![[4.0, 5.0, 6.0]]);
            if same_space {
                assert_eq!(count(&log, MOVE_REF_TO_NEW_SPACE), 0);
                assert_eq!(count(&log, SET_LOCATION_ON_REFERENCE), 1);
            } else {
                assert_eq!(
                    first(&log, MOVE_REF_TO_NEW_SPACE).1,
                    vec![r.actor, 0x44, 0x77]
                );
                assert_eq!(count(&log, SET_LOCATION_ON_REFERENCE), 0);
            }
        }
    }

    #[test]
    fn a_follower_with_a_plain_handler_and_no_flag_falls_back() {
        for flag in [false, true] {
            let (mut r, _) = stuck_rig(1);
            let player = object(&mut r.e, PLAYER_TABLE, PLAYER_FAKE);
            r.e.set_global(PLAYER_CHARACTER, player);
            r.e.register(IS_FOLLOWING, |_, _| ret(1));
            attach_handler(&mut r);
            attach_path(&mut r);
            r.e.register(HANDLER_FAKE + 0x34, |_, _| ret(0));
            r.e.register_double(PATH_FLAG_40, move |_, _| ret(flag as u32));
            r.e.register(PLAYER_FAKE + 0x1f4, |_, _| ret(SCRATCH + 0x900));
            r.e.register(ACTOR_FAKE + 0x1f4, |_, _| ret(ACTOR_POSITION));
            r.e.register(POINT_SUBTRACT, |_, a| ret(a[1]));
            r.e.register(DIFFERENCE_LENGTH, |_, _| ret_float(0.0));
            r.e.register(REQUEST_TARGET_RADIUS, |_, _| ret_float(1.0));
            r.e.call_log = Some(vec![]);
            r.e.call(0x009d_e8b0, &args![r.mover]);
            let log = calls(&mut r.e);
            assert_eq!(count(&log, ACTOR_FOLLOW_FALLBACK), (!flag) as usize);
            if !flag {
                assert_eq!(first(&log, ACTOR_FOLLOW_FALLBACK).1, vec![r.actor]);
                assert_eq!(count(&log, POINT_SUBTRACT), 0);
            } else {
                assert_eq!(count(&log, POINT_SUBTRACT), 1);
            }
        }
    }

    #[test]
    fn save_game_writes_the_fields_in_order() {
        let sequence = [
            (0x40, 2),
            (0x42, 2),
            (0x70, 1),
            (0x34, 4),
            (0x71, 1),
            (0x38, 4),
            (0x72, 1),
            (0x73, 1),
            (0x04, 12),
            (0x10, 12),
            (0x3c, 4),
            (0x74, 1),
            (0x75, 1),
            (0x77, 1),
            (0x76, 1),
            (0x78, 1),
            (0x6c, 4),
        ];
        let buffer = SCRATCH + 0xa00;
        // (request, path, handler: none, plain, detailed)
        for (with_request, with_path, handler_kind) in [
            (true, true, 2u32),
            (false, false, 1),
            (false, true, 0),
            (true, false, 0),
        ] {
            let mut r = rig();
            let handler = if handler_kind != 0 {
                attach_handler(&mut r)
            } else {
                0
            };
            let request = if with_request {
                attach_request(&mut r)
            } else {
                0
            };
            let path = if with_path { attach_path(&mut r) } else { 0 };
            let _ = (handler, path);
            r.e.register_double(HANDLER_FAKE + 0x34, move |_, _| {
                ret((handler_kind == 2) as u32)
            });
            r.e.register(HANDLER_FAKE + 0x78, |_, _| Ret::default());
            r.e.register(REQUEST_FAKE + 0x10, |_, _| ret(5));
            r.e.register(REQUEST_FAKE + 0x18, |_, _| Ret::default());
            r.e.map(LOCATION_TABLE, 0x1000);
            r.e.mem.set_u32(LOCATION_TABLE, LOCATION_FAKE);
            r.e.mem.set_u32(r.mover.addr() + 0x44, LOCATION_TABLE);
            r.e.register(LOCATION_FAKE, |_, _| Ret::default());
            r.e.set(r.mover, ActorMover::DetectionDoorID, 0xd00d);
            r.e.set(r.mover, ActorMover::iLastFailureTimeStamp, 4000);
            clock(&mut r.e, 5000);
            r.e.mem.set_u16(r.mover.addr() + 0x40, 0x1122);
            type Writes = Rc<RefCell<Vec<(u32, Vec<u8>)>>>;
            let writes: Writes = Rc::default();
            let record = writes.clone();
            r.e.register_double(SAVE_BYTES, move |e, a| {
                let bytes = (0..a[2]).map(|i| e.mem.u8(a[1] + i)).collect();
                record.borrow_mut().push((a[1], bytes));
                Ret::default()
            });
            r.e.call_log = Some(vec![]);
            r.e.call(0x009d_f100, &args![r.mover, p(buffer)]);
            let log = calls(&mut r.e);
            let writes = writes.borrow();
            let base = r.mover.addr();
            for (index, (offset, size)) in sequence.iter().enumerate() {
                assert_eq!(writes[index].0, base + offset, "{offset:x}");
                assert_eq!(writes[index].1.len(), *size);
            }
            assert_eq!(writes[0].1, vec![0x22, 0x11]);
            // The time since the last failure, then the failure count.
            assert_eq!(writes[17].1, 1000u32.to_le_bytes().to_vec());
            assert_eq!(writes[18].0, base + 0x84);
            assert_eq!(first(&log, LOCATION_FAKE).1, vec![base + 0x44, buffer]);
            assert_eq!(first(&log, SAVE_FORM_ID).1, vec![buffer, 0xd00d, 0]);
            let flags = with_request as u8
                | (with_path as u8) << 1
                | match handler_kind {
                    0 => 0,
                    1 => 4,
                    _ => 8,
                };
            assert_eq!(writes[19].1, vec![flags]);
            let mut expected_writes = 20;
            if with_request {
                assert_eq!(writes[20].1, vec![5]);
                expected_writes += 1;
                assert_eq!(first(&log, REQUEST_FAKE + 0x18).1, vec![request, buffer]);
            } else {
                assert_eq!(count(&log, REQUEST_FAKE + 0x18), 0);
            }
            assert_eq!(writes.len(), expected_writes);
            assert_eq!(count(&log, PATH_SAVE_GAME), with_path as usize);
            if with_path {
                assert_eq!(first(&log, PATH_SAVE_GAME).1, vec![path, buffer]);
            }
            assert_eq!(
                count(&log, HANDLER_FAKE + 0x78),
                (handler_kind != 0) as usize
            );
        }
    }

    /// What a load run read: the (destination, size) of every read, and the
    /// buffer's calls. Reads into the mover leave it alone; every other
    /// read takes the next value of `values` (little endian).
    struct LoadRun {
        reads: Rc<RefCell<Vec<(u32, u32)>>>,
        buffer: u32,
    }

    fn load_rig(version: u8, values: &[u32]) -> (Rig, LoadRun) {
        let mut r = rig();
        let buffer = object(&mut r.e, BUFFER_TABLE, BUFFER_FAKE);
        r.e.register_double(BUFFER_FAKE, move |_, _| ret(version as u32));
        let reads = Rc::new(RefCell::new(Vec::new()));
        let record = reads.clone();
        let queue = Rc::new(RefCell::new(std::collections::VecDeque::from(
            values.to_vec(),
        )));
        let low = r.mover.addr();
        r.e.register_double(LOAD_BYTES, move |e, a| {
            record.borrow_mut().push((a[1], a[2]));
            if a[1] < low || a[1] >= low + 0x88 {
                let value = queue.borrow_mut().pop_front().unwrap();
                for i in 0..a[2] {
                    e.mem.set_u8(a[1] + i, (value >> (8 * i)) as u8);
                }
            }
            Ret::default()
        });
        r.e.map(LOCATION_TABLE, 0x1000);
        r.e.mem.set_u32(LOCATION_TABLE + 4, LOCATION_FAKE + 4);
        r.e.mem.set_u32(r.mover.addr() + 0x44, LOCATION_TABLE);
        r.e.register(LOCATION_FAKE + 4, |_, _| Ret::default());
        r.e.set_global(UNSET_POINT, 0u32);
        (r, LoadRun { reads, buffer })
    }

    const SAVED_FIELDS: [(u32, u32); 17] = [
        (0x40, 2),
        (0x42, 2),
        (0x70, 1),
        (0x34, 4),
        (0x71, 1),
        (0x38, 4),
        (0x72, 1),
        (0x73, 1),
        (0x04, 12),
        (0x10, 12),
        (0x3c, 4),
        (0x74, 1),
        (0x75, 1),
        (0x77, 1),
        (0x76, 1),
        (0x78, 1),
        (0x6c, 4),
    ];

    #[test]
    fn load_game_reads_the_fields_by_save_version() {
        // (version, timestamp, now, reads timing, reads location,
        // stored last-failure stamp)
        for (version, stored, now, timing, location, stamp) in [
            (0x0eu8, 500u32, 2000u32, true, true, 1500u32),
            (0x0e, 3000, 2000, true, true, 0),
            (0x0d, 0, 2000, false, true, 0),
            (0x06, 0, 2000, false, false, 0),
        ] {
            let values: Vec<u32> = if timing { vec![stored, 0] } else { vec![0] };
            let (mut r, run) = load_rig(version, &values);
            clock(&mut r.e, now);
            r.e.call_log = Some(vec![]);
            r.e.call(0x009d_f3d0, &args![r.mover, p(run.buffer)]);
            let log = calls(&mut r.e);
            let base = r.mover.addr();
            let reads = run.reads.borrow();
            for (index, (offset, size)) in SAVED_FIELDS.iter().enumerate() {
                assert_eq!(reads[index], (base + offset, *size));
            }
            if timing {
                assert_eq!(reads[17].1, 4);
                assert_eq!(reads[18], (base + 0x84, 4));
                assert_eq!(reads[19].1, 1);
            } else {
                assert_eq!(reads[17].1, 1);
            }
            assert_eq!(
                r.e.get(r.mover, ActorMover::iLastFailureTimeStamp),
                stamp,
                "version {version:x}"
            );
            assert_eq!(count(&log, LOCATION_FAKE + 4), location as usize);
            assert_eq!(first(&log, LOAD_FORM_ID).1, vec![run.buffer, base + 0x2c]);
            // No flag bit: nothing is built.
            assert_eq!(count(&log, CREATE_PATHING_REQUEST), 0);
            assert_eq!(r.e.mem.u32(base + 0x24), 0);
        }
    }

    #[test]
    fn load_game_builds_the_request() {
        for (part_test, sets_actor) in [(0u32, true), (1, false)] {
            let (mut r, run) = load_rig(0x0e, &[0, 1, 3]);
            clock(&mut r.e, 10);
            let request = object(&mut r.e, REQUEST_TABLE, REQUEST_FAKE);
            r.e.register_double(CREATE_PATHING_REQUEST, move |_, _| ret(request));
            r.e.register(REQUEST_FAKE + 0x1c, |_, _| Ret::default());
            let process = attach_process(&mut r);
            r.e.register_double(PROCESS_PART_TEST, move |_, _| ret(part_test));
            r.e.call_log = Some(vec![]);
            r.e.call(0x009d_f3d0, &args![r.mover, p(run.buffer)]);
            let log = calls(&mut r.e);
            assert_eq!(first(&log, CREATE_PATHING_REQUEST).1, vec![3]);
            assert_eq!(r.e.mem.u32(r.mover.addr() + 0x1c), request);
            assert_eq!(
                first(&log, REQUEST_FAKE + 0x1c).1,
                vec![request, run.buffer]
            );
            assert_eq!(first(&log, PROCESS_PART_TEST).1, vec![process + 4, 0]);
            assert_eq!(count(&log, REQUEST_SET_ACTOR), sets_actor as usize);
            if sets_actor {
                assert_eq!(first(&log, REQUEST_SET_ACTOR).1, vec![request, r.actor]);
            }
        }
        // No process: the actor is not given to the request.
        let (mut r, run) = load_rig(0x0e, &[0, 1, 3]);
        let request = object(&mut r.e, REQUEST_TABLE, REQUEST_FAKE);
        r.e.register_double(CREATE_PATHING_REQUEST, move |_, _| ret(request));
        r.e.register(REQUEST_FAKE + 0x1c, |_, _| Ret::default());
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_f3d0, &args![r.mover, p(run.buffer)]);
        assert_eq!(count(&calls(&mut r.e), PROCESS_PART_TEST), 0);
    }

    #[test]
    fn load_game_builds_the_path_and_fails_on_a_stale_node() {
        // The second node's location has no nav mesh: the failure message,
        // then the clear and the failed state.
        for (nav_answers, stale) in [([1u32, 1], false), ([1, 0], true)] {
            let (mut r, run) = load_rig(0x0e, &[0, 2]);
            clock(&mut r.e, 10);
            r.e.register(PATH_CONSTRUCT, |_, a| ret(a[0]));
            r.e.register(PATH_NODE_COUNT, |_, _| ret(2));
            r.e.register(PATH_NODE_AT, |_, a| ret(0x900 + a[1]));
            r.e.register(NODE_TEST_FLAG, |_, _| ret(1));
            r.e.register(NODE_LOCATION, |_, a| ret(a[0] + 4));
            let answers = Rc::new(RefCell::new(nav_answers.to_vec()));
            let next = answers.clone();
            r.e.register_double(LOCATION_NAV_MESH_TEST, move |_, _| {
                ret(next.borrow_mut().remove(0))
            });
            r.e.register(ACTOR_FAKE + 0x130, |_, _| ret(0xaaa));
            r.e.register(PROCESS_DOOR_ID, |_, _| ret(0xbbb));
            r.e.call_log = Some(vec![]);
            r.e.call(0x009d_f3d0, &args![r.mover, p(run.buffer)]);
            let log = calls(&mut r.e);
            let path = first(&log, PATH_LOAD_GAME).1[0];
            let held = r.e.mem.u32(r.mover.addr() + 0x20);
            assert_ne!(path, 0);
            assert_eq!(first(&log, ALLOCATE).1, vec![0x44]);
            assert_eq!(first(&log, PATH_LOAD_GAME).1, vec![path, run.buffer]);
            assert_eq!(count(&log, PATH_NODE_AT), 4);
            assert_eq!(count(&log, PRINT_LINE), stale as usize);
            assert_eq!(count(&log, DEBUG_STUB), 2 * stale as usize);
            if stale {
                assert_eq!(
                    first(&log, PRINT_LINE).1,
                    vec![STALE_PATH_FORMAT, 0xaaa, 0xbbb]
                );
                assert_eq!(r.e.get(r.mover, ActorMover::eState), STATE_FAILED);
                assert!(r.e.get(r.mover, ActorMover::bPathingFailed));
                // ClearCurrentData dropped the path.
                assert_eq!(r.e.mem.u32(r.mover.addr() + 0x20), 0);
            } else {
                assert!(!r.e.get(r.mover, ActorMover::bPathingFailed));
                assert_eq!(held, path);
            }
        }
        // A node whose test fails is skipped without a location test.
        let (mut r, run) = load_rig(0x0e, &[0, 2]);
        r.e.register(PATH_CONSTRUCT, |_, a| ret(a[0]));
        r.e.register(PATH_NODE_COUNT, |_, _| ret(3));
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_f3d0, &args![r.mover, p(run.buffer)]);
        let log = calls(&mut r.e);
        assert_eq!(count(&log, NODE_TEST_FLAG), 3);
        assert_eq!(count(&log, LOCATION_NAV_MESH_TEST), 0);
    }

    #[test]
    fn load_game_creates_the_handler_the_flags_ask_for() {
        // (flags, constructor, size)
        for (flags, constructor, size) in [
            (4u32, VIRTUAL_HANDLER_CONSTRUCT, 0x20u32),
            (8, DETAILED_HANDLER_CONSTRUCT, 0xe4),
            (12, VIRTUAL_HANDLER_CONSTRUCT, 0x20),
        ] {
            let (mut r, run) = load_rig(0x0e, &[0, flags]);
            clock(&mut r.e, 10);
            let handler = object(&mut r.e, HANDLER_TABLE, HANDLER_FAKE);
            r.e.register_double(constructor, move |_, _| ret(handler));
            r.e.register(HANDLER_FAKE + 0x7c, |_, _| Ret::default());
            r.e.call_log = Some(vec![]);
            r.e.call(0x009d_f3d0, &args![r.mover, p(run.buffer)]);
            let log = calls(&mut r.e);
            assert_eq!(r.e.mem.u32(r.mover.addr() + 0x24), handler);
            assert_eq!(first(&log, OPERATOR_NEW).1, vec![size]);
            let made = first(&log, constructor);
            assert_eq!(
                made.1[1..],
                [r.actor, r.mover.addr(), r.mover.addr() + 0x20, 0]
            );
            assert_eq!(
                first(&log, HANDLER_FAKE + 0x7c).1,
                vec![handler, run.buffer]
            );
        }
    }

    #[test]
    fn init_load_game_attaches_the_request_to_the_target_object() {
        let buffer = SCRATCH + 0xa00;
        // No request: only the handler hears about it.
        let mut r = rig();
        let handler = attach_handler(&mut r);
        r.e.register(HANDLER_FAKE + 0x80, |_, _| Ret::default());
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_f8c0, &args![r.mover, p(buffer)]);
        let log = calls(&mut r.e);
        assert_eq!(first(&log, HANDLER_FAKE + 0x80).1, vec![handler, buffer]);
        assert_eq!(log.len(), 3);
        // A request of another type: initialised and given the actor.
        for (kind, attaches) in [(5u32, true), (4, false)] {
            let mut r = rig();
            let request = attach_request(&mut r);
            r.e.register_double(REQUEST_FAKE + 0x10, move |_, _| ret(kind));
            r.e.register(REQUEST_FAKE + 0x20, |_, _| Ret::default());
            r.e.register(ACTOR_FAKE + 0x428, |_, _| ret(0x6000));
            r.e.register(SLOT_OBJECT_HOLDER, |_, _| ret(0x6100));
            r.e.register(SLOT_OBJECT_VALUE, |_, _| ret(0x6200));
            r.e.call_log = Some(vec![]);
            r.e.call(0x009d_f8c0, &args![r.mover, p(buffer)]);
            let log = calls(&mut r.e);
            assert_eq!(count(&log, REQUEST_ATTACH), attaches as usize);
            if attaches {
                assert_eq!(first(&log, SLOT_OBJECT_HOLDER).1, vec![0x6000]);
                assert_eq!(first(&log, SLOT_OBJECT_VALUE).1, vec![0x6100]);
                let temporary = first(&log, NI_POINTER_CONSTRUCT).1[0];
                assert_eq!(first(&log, NI_POINTER_CONSTRUCT).1[1], 0x6200);
                assert_eq!(first(&log, REQUEST_ATTACH).1, vec![request, temporary]);
                assert_eq!(first(&log, NI_POINTER_DESTRUCT).1, vec![temporary]);
            }
            assert_eq!(first(&log, REQUEST_FAKE + 0x20).1, vec![request, buffer]);
            assert_eq!(first(&log, REQUEST_SET_ACTOR).1, vec![request, r.actor]);
        }
        // Type 5 but the slot object is missing or has no holder.
        for holder in [0u32, 0x6100] {
            let mut r = rig();
            attach_request(&mut r);
            r.e.register(REQUEST_FAKE + 0x10, |_, _| ret(5));
            r.e.register(REQUEST_FAKE + 0x20, |_, _| Ret::default());
            r.e.register_double(ACTOR_FAKE + 0x428, move |_, _| ret(holder));
            r.e.register(SLOT_OBJECT_HOLDER, |_, _| ret(0));
            r.e.call_log = Some(vec![]);
            r.e.call(0x009d_f8c0, &args![r.mover, p(buffer)]);
            assert_eq!(count(&calls(&mut r.e), REQUEST_ATTACH), 0);
        }
    }

    #[test]
    fn finish_load_game_asks_the_path_manager_again() {
        // Not waiting for a path, or no request: nothing happens.
        let mut r = rig();
        attach_request(&mut r);
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_fa00, &args![r.mover, 0u32]);
        assert_eq!(calls(&mut r.e).len(), 1);
        let mut r = rig();
        r.e.set(r.mover, ActorMover::bWaitingForPath, true);
        r.e.call_log = Some(vec![]);
        r.e.call(0x009d_fa00, &args![r.mover, 0u32]);
        let log = calls(&mut r.e);
        assert_eq!(count(&log, BUILD_PATH), 0);
        assert!(!r.e.get(r.mover, ActorMover::bPathingFailed));
        // (request type, holder set, queue set, BuildPath answer): the
        // mover keeps waiting only when BuildPath accepts.
        for (kind, holder, queue, accepts, builds, new_queue) in [
            (0u32, 0u32, 0x700u32, 1u32, true, false),
            (0, 0, 0x700, 0, true, false),
            (5, 0x66, 0x700, 1, true, false),
            (5, 0, 0x700, 1, false, false),
            (0, 0, 0, 1, true, true),
        ] {
            let mut r = rig();
            let request = attach_request(&mut r);
            r.e.register_double(REQUEST_FAKE + 0x10, move |_, _| ret(kind));
            let holder_cell = SCRATCH + 0x600;
            r.e.mem.set_u32(holder_cell, holder);
            r.e.register(REQUEST_HOLDER, |_, _| ret(SCRATCH + 0x600));
            r.e.mem.set_u32(r.mover.addr() + 0x30, queue);
            r.e.set(r.mover, ActorMover::bWaitingForPath, true);
            r.e.register(QUEUE_POINTER_COPY, |e, a| {
                let value = e.mem.u32(a[1]);
                e.mem.set_u32(a[0], value);
                ret(a[0])
            });
            r.e.register(REQUEST_POINTER_COPY, |e, a| {
                let value = e.mem.u32(a[1]);
                e.mem.set_u32(a[0], value);
                ret(a[0])
            });
            r.e.register(PATH_MANAGER_INSTANCE, |_, _| ret(0x4444));
            r.e.register(QUEUE_CONSTRUCT, |_, a| ret(a[0]));
            r.e.register_double(BUILD_PATH, move |_, _| ret(accepts));
            r.e.call_log = Some(vec![]);
            r.e.call(0x009d_fa00, &args![r.mover, 0u32]);
            let log = calls(&mut r.e);
            assert_eq!(count(&log, BUILD_PATH), builds as usize, "{kind} {holder}");
            if builds {
                let queue_now = r.e.mem.u32(r.mover.addr() + 0x30);
                assert_eq!(
                    first(&log, BUILD_PATH).1,
                    vec![0x4444, request, queue_now, 1]
                );
            }
            assert_eq!(count(&log, ALLOCATE), new_queue as usize);
            let keeps_waiting = builds && accepts == 1;
            assert_eq!(r.e.get(r.mover, ActorMover::bWaitingForPath), keeps_waiting);
            assert_eq!(r.e.get(r.mover, ActorMover::bPathingFailed), !keeps_waiting);
        }
    }

    #[test]
    fn every_listed_function_is_registered_once() {
        let table = funcs();
        let mut seen = std::collections::HashSet::new();
        for (address, _) in &table {
            assert!(seen.insert(*address), "{address:08x} registered twice");
        }
        assert_eq!(table.len(), 73);
        let e = Engine::new();
        for (address, _) in &table {
            assert!(e.is_translated(*address), "{address:08x}");
        }
    }

    // ---- the exe's stack frames ----

    /// The local the next `with_stack(size)` on this thread takes (the frame of the function the
    /// test calls next, taken outside any other local of it), with the call log switched on.
    fn next_frame(e: &mut Engine, size: u32) -> u32 {
        let frame = e.next_stack_local(size);
        e.call_log = Some(vec![]);
        frame
    }

    /// The second word of each logged call to `address`.
    fn outputs(log: &[Call], address: u32, word: usize) -> Vec<u32> {
        log.iter()
            .filter(|call| call.0 == address)
            .map(|call| call.1[word])
            .collect()
    }

    #[test]
    fn start_pathing_keeps_the_shortcut_positions_at_the_exe_frame_offsets() {
        let mut r = rig();
        let request = shortcut_world(&mut r.e);
        put_point(&mut r.e, request + 0x10, 1.0, 2.0, 100.0);
        put_point(&mut r.e, request + 0x38, 1.5, 2.0, 100.0);
        let raw = next_frame(&mut r.e, 0xc + 0x114);
        r.e.call(0x009d_b090, &args![r.mover, Ptr::<()>::new(request)]);
        let log = calls(&mut r.e);
        let frame = raw + 0xc;
        // Origin, destination, goal, start; the difference at +0xe8.
        assert_eq!(
            outputs(&log, GET_POSITION, 1),
            vec![frame + 0xd8, frame + 0xcc, frame + 0xb4, frame + 0xc0]
        );
        assert_eq!(first(&log, POINT_SUBTRACT).1[1], frame + 0xe8);
        assert_eq!(first(&log, POINT_LENGTH_SQUARED_XY).1, vec![frame + 0xe8]);
    }

    #[test]
    fn start_pathing_builds_the_build_path_copies_in_their_argument_slots() {
        let mut r = rig();
        let request = shortcut_world(&mut r.e);
        put_point(&mut r.e, request + 0x38, 3.0, 4.0, 100.0);
        r.e.register(REQUEST_COPY_CONSTRUCT, |_, a| ret(a[0]));
        r.e.register(PATH_MANAGER_INSTANCE, |_, _| ret(0x77));
        r.e.register(QUEUE_CONSTRUCT, |_, a| ret(a[0]));
        let copy: AbiFn = |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            ret(a[0])
        };
        r.e.register(QUEUE_POINTER_COPY, copy);
        r.e.register(REQUEST_POINTER_COPY, copy);
        r.e.register(BUILD_PATH, |_, _| ret(1));
        let raw = next_frame(&mut r.e, 0xc + 0x114);
        r.e.call(0x009d_b090, &args![r.mover, Ptr::<()>::new(request)]);
        let log = calls(&mut r.e);
        // The queue copy at frame - 8, the request copy at frame - 0xc, the flag at frame - 4.
        assert_eq!(
            first(&log, QUEUE_POINTER_COPY).1,
            vec![raw + 4, r.mover.addr() + 0x30]
        );
        assert_eq!(
            first(&log, REQUEST_POINTER_COPY).1,
            vec![raw, r.mover.addr() + 0x1c]
        );
        assert_eq!(r.e.mem.u32(raw + 8), 1);
    }

    #[test]
    fn set_goal_keeps_the_request_and_the_location_at_the_exe_frame_offsets() {
        for wrapper in [0x009d_ba50u32, 0x009d_bb60] {
            let mut r = rig();
            goal_world(&mut r);
            let frame = next_frame(&mut r.e, 0xc8);
            // `009dbc90`'s 0x3c-byte frame, taken while the request's frame is held.
            let fill = r.e.with_stack(0xc8, |e, _| e.next_stack_local(0x3c));
            if wrapper == 0x009d_ba50 {
                r.e.call(
                    wrapper,
                    &args![r.mover, p(0x111), p(0x222), p(0x333), 2.0f32, p(0x444)],
                );
            } else {
                r.e.call(
                    wrapper,
                    &args![
                        r.mover,
                        p(0x111),
                        p(0x222),
                        p(0x333),
                        1.0f32,
                        2.0f32,
                        p(0x444)
                    ],
                );
            }
            let log = calls(&mut r.e);
            assert_eq!(first(&log, REQUEST_CONSTRUCT).1, vec![frame + 0xc]);
            assert_eq!(first(&log, LOCATION_FROM_POSITION).1[0], fill + 8);
            assert_eq!(first(&log, REQUEST_DESTRUCT).1, vec![frame + 0xc]);
        }
    }

    #[test]
    fn rotate_to_a_point_uses_the_argument_slots_and_frame_offset_8() {
        let mut r = rig();
        r.e.register(ACTOR_FAKE + 0x1f4, |_, _| ret(ACTOR_POSITION));
        put_point(&mut r.e, ACTOR_POSITION, 1.0, 2.0, 3.0);
        r.e.register(Z_ANGLE_FROM_VECTOR, |_, _| ret_float(0.5));
        r.e.register(ACTOR_CAN_MOVE, |_, _| ret(0));
        r.e.register(ACTOR_FAKE + 0x1e4, |_, _| ret(0));
        let frame = next_frame(&mut r.e, 0x2c);
        r.e.call(
            0x009d_ce20,
            &args![r.mover, 11.0f32, 22.0f32, 33.0f32, true],
        );
        let log = calls(&mut r.e);
        // The point in its argument slots (+0x1c), the difference at +8.
        assert_eq!(
            first(&log, POINT_SUBTRACT).1,
            vec![frame + 0x1c, frame + 8, ACTOR_POSITION]
        );
        assert_eq!(point(&r.e, frame + 0x1c), [11.0, 22.0, 33.0]);
        assert_eq!(first(&log, Z_ANGLE_FROM_VECTOR).1, vec![frame + 8]);
    }

    #[test]
    fn the_path_end_queries_use_the_exe_frame_offsets() {
        let mut r = rig();
        let path = attach_path(&mut r);
        r.e.register(PATH_LAST_NODE, |_, _| ret(SCRATCH + 0x800));
        r.e.register(NODE_LOCATION, |_, a| ret(a[0] + 4));
        r.e.register(GET_POSITION, |_, a| ret(a[1]));
        let out = SCRATCH + 0x100;
        let frame = next_frame(&mut r.e, 0x14);
        assert!(r.e.call(0x009d_e2b0, &args![r.mover, p(out)]).bool());
        assert_eq!(first(&calls(&mut r.e), GET_POSITION).1[1], frame + 4);
        // The distance to the path end (`009dc5e0`): node position +0x18, difference +0x24.
        r.e.register(ACTOR_FAKE + 0x1f4, |_, _| ret(ACTOR_POSITION));
        r.e.register(PATH_NODE_AT, |_, _| ret(SCRATCH + 0x800));
        r.e.register(POINT_SUBTRACT, |_, a| ret(a[1]));
        r.e.register(POINT_LENGTH_SQUARED_XY, |_, _| ret_float(0.0));
        attach_request(&mut r);
        r.e.register(REQUEST_TARGET_RADIUS, |_, _| ret_float(1.0));
        let frame = next_frame(&mut r.e, 0x34);
        path_end_is_too_far(&mut r.e, r.mover);
        let log = calls(&mut r.e);
        assert_eq!(first(&log, GET_POSITION).1[1], frame + 0x18);
        assert_eq!(
            first(&log, POINT_SUBTRACT).1,
            vec![frame + 0x18, frame + 0x24, ACTOR_POSITION]
        );
        let _ = path;
    }

    #[test]
    fn the_water_test_writes_into_the_callers_slots() {
        let mut r = rig();
        water_request(&mut r, 0.0, 5.0, 2.0, 0x11);
        let slots = [
            SCRATCH + 0x300,
            SCRATCH + 0x310,
            SCRATCH + 0x320,
            SCRATCH + 0x330,
        ];
        r.e.call_log = Some(vec![]);
        assert!(request_ends_differ_in_water(&mut r.e, r.mover, slots));
        assert_eq!(outputs(&calls(&mut r.e), GET_POSITION, 1), slots.to_vec());
    }

    #[test]
    fn set_pathing_failed_uses_the_exe_frame_offsets() {
        let mut r = rig();
        r.e.set_global(FAILURE_TIMEOUT_HIDDEN, 1.5f32);
        r.e.set_global(FAILURE_TIMEOUT_WATER, 1.0f32);
        r.e.set_global(MILLISECONDS_TO_SECONDS, 0.001f64);
        water_request(&mut r, 0.0, 5.0, 2.0, 0x11);
        clock(&mut r.e, 2200);
        let frame = next_frame(&mut r.e, 0x8c);
        actor_mover_set_pathing_failed(&mut r.e, r.mover, true);
        // Origin +0x54 / +0x48, destination +0x3c / +0x30.
        assert_eq!(
            outputs(&calls(&mut r.e), GET_POSITION, 1),
            vec![frame + 0x54, frame + 0x48, frame + 0x3c, frame + 0x30]
        );
    }

    #[test]
    fn recovery_keeps_its_locals_at_the_exe_frame_offsets() {
        let (mut r, _) = stuck_rig(0);
        r.e.register(LOCATION_IS_RESOLVED, |_, _| ret(0));
        r.e.register(PATH_COUNT_20, |_, _| ret(3));
        r.e.register(PATH_SECOND_ARRAY_ELEMENT, |_, _| ret(END_A - 4));
        r.e.register(NODE_LOCATION, |_, a| ret(a[0] + 4));
        let frame = next_frame(&mut r.e, 0x210);
        r.e.call(0x009d_e8b0, &args![r.mover]);
        let log = calls(&mut r.e);
        let actor_locations = outputs(&log, LOCATION_FROM_ACTOR, 0);
        assert_eq!(actor_locations, vec![frame + 0x1d0, frame + 0x4c]);
        assert_eq!(first(&log, REQUEST_CONSTRUCT).1, vec![frame + 0x10c]);
        assert_eq!(first(&log, POINT_CONSTRUCT_DEFAULT).1, vec![frame + 0x1c0]);
        assert_eq!(first(&log, PATH_CONSTRUCT).1, vec![frame + 0xbc]);
        // The unsolved path's second node position at +0x34.
        assert_eq!(first(&log, GET_POSITION).1[1], frame + 0x34);
    }

    #[test]
    fn recovery_takes_a_solved_paths_point_into_frame_offset_0x40() {
        let (mut r, _) = stuck_rig(0);
        r.e.register(LOCATION_IS_RESOLVED, |_, _| ret(0));
        r.e.register(SOLVE_REQUEST, |_, _| ret(1));
        r.e.register(PATH_COUNT_20, |_, _| ret(2));
        r.e.register(PATH_LENGTH, |_, _| ret_float(1000.0));
        r.e.register(PATH_POINT_AT, |_, a| ret(a[1]));
        let frame = next_frame(&mut r.e, 0x210);
        r.e.call(0x009d_e8b0, &args![r.mover]);
        assert_eq!(first(&calls(&mut r.e), PATH_POINT_AT).1[1], frame + 0x40);
    }

    #[test]
    fn save_game_writes_each_value_from_its_own_frame_local() {
        let mut r = rig();
        attach_request(&mut r);
        r.e.register(REQUEST_FAKE + 0x10, |_, _| ret(5));
        r.e.register(REQUEST_FAKE + 0x18, |_, _| Ret::default());
        r.e.map(LOCATION_TABLE, 0x1000);
        r.e.mem.set_u32(LOCATION_TABLE, LOCATION_FAKE);
        r.e.mem.set_u32(r.mover.addr() + 0x44, LOCATION_TABLE);
        r.e.register(LOCATION_FAKE, |_, _| Ret::default());
        clock(&mut r.e, 5000);
        let frame = next_frame(&mut r.e, 0x18);
        r.e.call(0x009d_f100, &args![r.mover, p(SCRATCH + 0xa00)]);
        let log = calls(&mut r.e);
        let written: Vec<u32> = outputs(&log, SAVE_BYTES, 1);
        // The time at +0x10, the flags at +0x17, the request kind at +0xf.
        assert_eq!(written[written.len() - 2..], [frame + 0x17, frame + 0xf]);
        assert!(written.contains(&(frame + 0x10)));
    }

    #[test]
    fn load_game_reads_each_value_into_its_own_frame_local() {
        let (mut r, run) = load_rig(0x0e, &[500, 0]);
        clock(&mut r.e, 2000);
        let frame = next_frame(&mut r.e, 0x4c);
        r.e.call(0x009d_f3d0, &args![r.mover, p(run.buffer)]);
        let reads = run.reads.borrow();
        assert_eq!(reads[17].0, frame + 0x38);
        assert_eq!(reads[19].0, frame + 0x3e);
    }

    #[test]
    fn init_load_game_keeps_the_temporary_at_frame_offset_0x10() {
        let mut r = rig();
        attach_request(&mut r);
        r.e.register(REQUEST_FAKE + 0x10, |_, _| ret(5));
        r.e.register(REQUEST_FAKE + 0x20, |_, _| Ret::default());
        r.e.register(ACTOR_FAKE + 0x428, |_, _| ret(0x6000));
        r.e.register(SLOT_OBJECT_HOLDER, |_, _| ret(0x6100));
        r.e.register(SLOT_OBJECT_VALUE, |_, _| ret(0x6200));
        let frame = next_frame(&mut r.e, 0x28);
        r.e.call(0x009d_f8c0, &args![r.mover, p(SCRATCH + 0xa00)]);
        let log = calls(&mut r.e);
        assert_eq!(
            first(&log, NI_POINTER_CONSTRUCT).1,
            vec![frame + 0x10, 0x6200]
        );
    }

    #[test]
    fn finish_load_game_builds_the_copies_in_their_argument_slots() {
        let mut r = rig();
        attach_request(&mut r);
        r.e.set(r.mover, ActorMover::bWaitingForPath, true);
        r.e.register(REQUEST_FAKE + 0x10, |_, _| ret(0));
        r.e.mem.set_u32(r.mover.addr() + 0x30, 0x700);
        r.e.register(PATH_MANAGER_INSTANCE, |_, _| ret(0x77));
        let copy: AbiFn = |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            ret(a[0])
        };
        r.e.register(QUEUE_POINTER_COPY, copy);
        r.e.register(REQUEST_POINTER_COPY, copy);
        r.e.register(BUILD_PATH, |_, _| ret(1));
        let raw = next_frame(&mut r.e, 0xc + 0x48);
        r.e.call(0x009d_fa00, &args![r.mover, 0u32]);
        let log = calls(&mut r.e);
        assert_eq!(
            first(&log, QUEUE_POINTER_COPY).1,
            vec![raw + 4, r.mover.addr() + 0x30]
        );
        assert_eq!(
            first(&log, REQUEST_POINTER_COPY).1,
            vec![raw, r.mover.addr() + 0x1c]
        );
        assert_eq!(r.e.mem.u32(raw + 8), 1);
    }

    #[test]
    fn the_move_delta_and_the_detailed_rebuild_use_the_exe_frame_offsets() {
        // The forced-mode delta at +0x2c of 0x40 bytes.
        let mut r = rig();
        r.e.set(r.mover, ActorMover::bForceMoveMode, true);
        r.e.register(ACTOR_FAKE + 0x1e4, |_, _| ret(0x0600_0a00));
        r.e.register(ACTOR_MOVEMENT_TEST, |_, _| ret(1));
        let frame = next_frame(&mut r.e, 0x40);
        r.e.call(0x009d_dc00, &args![r.mover]);
        assert_eq!(
            first(&calls(&mut r.e), ANIMATION_GET_DELTA).1[1],
            frame + 0x2c
        );
        // The path end past the last node: +0x5c with the test, +0x50 without.
        for (global_test, offset) in [(true, 0x5cu32), (false, 0x50)] {
            let (mut r, _) = detailed_rebuild_rig();
            r.e.register(PATH_NODE_COUNT, |_, _| ret(3));
            r.e.set(r.mover, ActorMover::iPreviousNode, 2);
            r.e.register(PATH_LAST_NODE, |_, _| ret(0x1111));
            r.e.register(NODE_LOCATION, |_, a| ret(a[0] + 4));
            r.e.register(GET_POSITION, |_, a| ret(a[1]));
            r.e.register(REPORT_TARGET_OBJECT, |_, _| ret(0x888));
            r.e.register(ACTOR_FAKE + 0x2a8, |_, _| Ret::default());
            r.e.register(HANDLER_FAKE + 0x70, |_, _| Ret::default());
            let test: AbiFn = if global_test {
                |_, _| ret(1)
            } else {
                |_, _| ret(0)
            };
            r.e.register(GLOBAL_TEST, test);
            let frame = next_frame(&mut r.e, 0xbc);
            r.e.call(0x009d_d2d0, &args![r.mover]);
            assert_eq!(first(&calls(&mut r.e), GET_POSITION).1[1], frame + offset);
        }
    }

    #[test]
    fn the_first_segment_fraction_uses_the_exe_frame_offsets() {
        let mut r = rig();
        attach_path(&mut r);
        attach_handler(&mut r);
        r.e.register(HANDLER_FAKE + 0x18, |_, _| ret_float(1.0));
        r.e.register(PATH_NODE_AT, |_, _| ret(SCRATCH + 0x800));
        r.e.register(NODE_LOCATION, |_, a| ret(a[0] + 4));
        r.e.register(GET_POSITION, |_, a| ret(a[1]));
        r.e.register(POINT_SUBTRACT, |_, a| ret(a[1]));
        r.e.register(POINT_LENGTH, |_, _| ret_float(2.0));
        let frame = next_frame(&mut r.e, 0xbc);
        first_segment_fraction(&mut r.e, r.mover);
        let log = calls(&mut r.e);
        assert_eq!(
            outputs(&log, GET_POSITION, 1),
            vec![frame + 0x80, frame + 0x90]
        );
        assert_eq!(
            first(&log, POINT_SUBTRACT).1,
            vec![frame + 0x90, frame + 0x44, frame + 0x80]
        );
    }

    #[test]
    fn the_path_message_sits_at_frame_offset_0x1c() {
        let mut r = rig();
        attach_process(&mut r);
        for slot in [0x610u32, 0x4bc, 0x40c, 0x6fc] {
            r.e.register(PROCESS_FAKE + slot, |_, _| ret(0));
        }
        r.e.register(ACTOR_FAKE + 0x230, |_, _| ret(0));
        r.e.set(r.mover, ActorMover::bWaitingForPath, true);
        let queue = object(&mut r.e, QUEUE_TABLE, QUEUE_FAKE);
        r.e.mem.set_u32(r.mover.addr() + 0x30, queue);
        r.e.register(QUEUE_FAKE + 0x10, |_, _| ret(0));
        let frame = next_frame(&mut r.e, 0x34);
        let actor = r.actor;
        update_movement(&mut r.e, r.mover, actor, 0.1);
        let log = calls(&mut r.e);
        assert_eq!(first(&log, MESSAGE_CONSTRUCT).1, vec![frame + 0x1c]);
        assert_eq!(first(&log, QUEUE_FAKE + 0x10).1, vec![queue, frame + 0x1c]);
    }
}
