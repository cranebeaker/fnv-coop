//! `fallout/ai/combat/combatproceduremove.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `CombatProcedureMove` is the combat procedure that walks an actor to a
//! destination (and, in the cover states, to a covered position and then
//! hides there). Its vtable is at `01091bb4` (read from the exe: slot 0 is
//! the deleting destructor `009d6b70`, 1 `Update`, 3 `Shutdown`, 4
//! `ClearReferencePointers`, 5 `DisplayDebugText`, 11 `StartedPlanning`, 12
//! `FinishedPlanning`, 13 `GetType`, 14 `SaveGame`, 15 `LoadGame`, 16
//! `InitLoadGame`).
//!
//! Conventions of this file:
//! - The layout is the Xbox PDB's (`pdbdump type`), and every offset below
//!   was checked against the PC code. Embedded structs
//!   (`BGSWorldLocation`, 0x10 bytes; `CombatTimer`, 8 bytes; a
//!   `NiPoint3`) are addressed through the offset constants.
//! - The compiler's exception-unwinding frames (`FS:[0]`) are not
//!   translated: the functions that have one carry a `Translation
//!   incomplete` marker. Their normal-path destructor calls are translated.
//! - `ECX` receivers of `__thiscall` callees are the first argument word.
//!   Stack arguments follow in the order the callee reads them (the
//!   decompiler hangs pushed words on the wrong call; every argument list
//!   here comes from the disassembly).
//! - x87: float comparisons follow the flag tests of the disassembly (a
//!   strict `<` or `>`, and NaN fails them); `float` stores round once,
//!   sums of two floats are formed in `f64`.
//! - The game's locals that are passed by address live in one block that
//!   stands for the game's stack frame (`MoveFrame`), so offsets below
//!   `EBP` match the disassembly.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `CombatProcedureMove` (Xbox PDB), 0x7c bytes on the PC too (its
    /// constructor `009d69f0` is allocated with `0x7c` by `00996f10`).
    /// The embedded structs are at `DESTINATION` and the other offset
    /// constants below.
    pub struct CombatProcedureMove: 0x7c {
        /// `pCombatController` (Xbox PDB, `CombatProcedure`): `CombatController*`.
        0x04 pCombatController: Ptr,
        /// `eState` (Xbox PDB): 0 initializing, 1 moving, 2 moving using
        /// cover, 3 hiding behind cover (the names are the debug texts of
        /// `DisplayDebugText`).
        0x10 eState: u32,
        /// `fTargetRadius` (Xbox PDB).
        0x34 fTargetRadius: f32,
        /// `pTargetRef` (Xbox PDB): `TESObjectREFR*`.
        0x38 pTargetRef: Ptr,
        /// `iLocation` (Xbox PDB).
        0x3C iLocation: i32,
        /// `bReservedSecondaryLocation` (Xbox PDB).
        0x54 bReservedSecondaryLocation: bool,
        /// `spQueuedPathingRequest` (Xbox PDB): `NiPointer<PathingRequest>`.
        0x58 spQueuedPathingRequest: Ptr,
        /// `bHasQueuedPathingRequest` (Xbox PDB).
        0x5C bHasQueuedPathingRequest: bool,
        /// `iCurrentNode` (Xbox PDB).
        0x60 iCurrentNode: i32,
    }
}

/// `Destination` (Xbox PDB, `BGSWorldLocation`).
const DESTINATION: u32 = 0x14;
/// `CurrentDestination` (Xbox PDB, `BGSWorldLocation`).
const CURRENT_DESTINATION: u32 = 0x24;
/// `HidingTimer` (Xbox PDB, `CombatTimer`).
const HIDING_TIMER: u32 = 0x40;
/// `LookAtPosition` (Xbox PDB, `NiPoint3`); its `z` is at `+0x50`.
const LOOK_AT_POSITION: u32 = 0x48;
/// The `z` of `LookAtPosition`.
const LOOK_AT_Z: u32 = 0x50;
/// `CoveredMoveSearchTimer` (Xbox PDB, `CombatTimer`).
const COVERED_MOVE_SEARCH_TIMER: u32 = 0x64;
/// `CheckCoverTimer` (Xbox PDB, `CombatTimer`).
const CHECK_COVER_TIMER: u32 = 0x6C;
/// `CheckDestinationTimer` (Xbox PDB, `CombatTimer`).
const CHECK_DESTINATION_TIMER: u32 = 0x74;

/// The vtable of `CombatProcedureMove` that the destructor stores.
const VTABLE: u32 = 0x0109_1bb4;

// ---- Callees outside this file (by exe address) ----

/// `operator delete`.
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `NiPointer` destructor (`0045cec0`), as `highprocess_p3` names it.
const SMART_POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// The base class destructor (`009ca6c0`, `CombatProcedure`'s part).
const BASE_DESTRUCT: u32 = 0x009c_a6c0;
/// `CombatProcedure::GetAttacker` (Xbox PDB name of `00996de0`).
const GET_ATTACKER: u32 = 0x0099_6de0;
/// `CombatProcedure::GetTarget` (Xbox PDB name of `00996e00`).
const GET_TARGET: u32 = 0x0099_6e00;
/// `CombatProcedure::SetMoveMode` (Xbox PDB name of `00996e20`): move mode.
const SET_MOVE_MODE: u32 = 0x0099_6e20;
/// `CombatProcedure::ClearMoveMode` (Xbox PDB name of `00996e40`).
const CLEAR_MOVE_MODE: u32 = 0x0099_6e40;
/// `CombatProcedure::SetLookAtTarget` (Xbox PDB name of `00996e60`):
/// position, a byte.
const SET_LOOK_AT_TARGET: u32 = 0x0099_6e60;
/// `CombatProcedure::ClearLookAtTarget` (Xbox PDB name of `00996ea0`).
const CLEAR_LOOK_AT_TARGET: u32 = 0x0099_6ea0;
/// `CombatProcedure::SaveGame` (Xbox PDB name of `00996ed0`): buffer.
const BASE_SAVE_GAME: u32 = 0x0099_6ed0;
/// `CombatProcedure::LoadGame` (Xbox PDB name of `00996ef0`): buffer.
const BASE_LOAD_GAME: u32 = 0x0099_6ef0;
/// The controller's `CombatState*` (`00586150`, a getter of the word at
/// `+0x9c`; the engine map files it under `tesworldspace.cpp` because of
/// code folding). Its Xbox methods (`UnreserveCoverLocation`, ...) are
/// called on this object.
const GET_COMBAT_STATE: u32 = 0x0058_6150;
/// `00559450`: the pointer a `NiPointer` holds (the word at its address).
const SMART_POINTER_GET: u32 = 0x0055_9450;
/// `0066b0d0` (`__thiscall`): assigns the pointer to the `NiPointer`.
const SMART_POINTER_SET: u32 = 0x0066_b0d0;
/// `0097ae90`: the controller's `+0xbc` (the attacking actor).
const CONTROLLER_ACTOR: u32 = 0x0097_ae90;
/// `MobileObject::GetCurrentProcessType` (Xbox PDB name of `00931850`);
/// -1 when the object has no process.
const GET_PROCESS_TYPE: u32 = 0x0093_1850;
/// `TESObjectREFR::GetSpace` (Xbox PDB name of `00575ca0`).
const GET_SPACE: u32 = 0x0057_5ca0;
/// `TESObjectREFR::GetWorldLocation` (Xbox PDB name of `00575650`):
/// (reference; out location).
const GET_WORLD_LOCATION: u32 = 0x0057_5650;
/// `BGSWorldLocation::GetDistanceSquared` (Xbox PDB name of `00527a40`):
/// (location; other), in `ST0`.
const DISTANCE_SQUARED: u32 = 0x0052_7a40;
/// `BGSWorldLocation::SaveGame` / `LoadGame` (Xbox PDB names).
const WORLD_LOCATION_SAVE: u32 = 0x0052_7b70;
const WORLD_LOCATION_LOAD: u32 = 0x0052_7ba0;
/// `CombatTimer::SaveGame` / `LoadGame` (Xbox PDB names): (timer; buffer).
const TIMER_SAVE: u32 = 0x009a_5eb0;
const TIMER_LOAD: u32 = 0x009a_5f20;
/// `BGSSaveGameBuffer` raw write (`00865e50`): (buffer; data, size, 0).
const SAVE_BYTES: u32 = 0x0086_5e50;
/// `BGSLoadGameBuffer` raw read (`00864980`): (buffer; data, size).
const LOAD_BYTES: u32 = 0x0086_4980;
/// `BGSSaveGameBuffer::SaveFormID_ov2` (Xbox PDB name): (buffer; form, 0).
const SAVE_FORM_ID: u32 = 0x0086_5df0;
/// `BGSLoadGameBuffer::LoadFormID_ov2` (Xbox PDB name): (buffer; address
/// of the id).
const LOAD_FORM_ID: u32 = 0x0086_48e0;
/// `004534f0` (`__thiscall`, one word): called by `InitLoadGame` first
/// (the slot also serves `Disarmed` and `FiredProjectile`).
const BASE_INIT_LOAD_GAME: u32 = 0x0045_34f0;
/// `004839c0` (cdecl, one word): looks a form up by its id (through the
/// form table at `011c54c0`), 0 when missing.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (`00ec43fb`, cdecl).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The two type descriptors `InitLoadGame` casts between.
const CAST_SOURCE_TYPE: u32 = 0x0118_3028;
const CAST_TARGET_TYPE: u32 = 0x0118_41cc;
/// `Actor::GetCurrentPathingRequest` (Xbox PDB name of `008b3820`).
const GET_CURRENT_PATHING_REQUEST: u32 = 0x008b_3820;
/// `Actor::StopMoving` (Xbox PDB name of `008b3ab0`).
const STOP_MOVING: u32 = 0x008b_3ab0;
/// `00666f00`: a getter the shutdown tests on the procedure.
const SHUTDOWN_TEST: u32 = 0x0066_6f00;
/// `CombatState::UnreserveCoverLocation` / `ReserveCoverLocation` (Xbox
/// PDB names of `009d6c80` / `009d7c20`).
const UNRESERVE_COVER_LOCATION: u32 = 0x009d_6c80;
const RESERVE_COVER_LOCATION: u32 = 0x009d_7c20;
/// `00997520` (`__thiscall` on the member at `CombatState+0x128`): copies
/// the 0x38-byte record at the argument into it (resets it when null).
const STATE_COPY_RECORD: u32 = 0x0099_7520;
/// `009a0560` (`__thiscall` on the `CombatState`): (location), AL; 1 for a
/// null argument.
const STATE_CHECK_LOCATION: u32 = 0x009a_0560;
/// `CombatState::ClearCoverLocation` / `ClearSecondaryCoverLocation` /
/// `AddUnreachableCoverLocation` / `AddUnreachableLocation` (Xbox PDB
/// names of `009a03d0`, `009a0470`, `009a0d00`, `009a0ec0`).
const CLEAR_COVER_LOCATION: u32 = 0x009a_03d0;
const CLEAR_SECONDARY_COVER_LOCATION: u32 = 0x009a_0470;
const ADD_UNREACHABLE_COVER_LOCATION: u32 = 0x009a_0d00;
const ADD_UNREACHABLE_LOCATION: u32 = 0x009a_0ec0;
/// `009a03a0`: stores a word at `CombatState+0xe8` (after clearing the
/// secondary cover location).
const COMBAT_STATE_SET_COVER: u32 = 0x009a_03a0;
/// `0097fb60`: the word at `CombatState+0xe8` (the cover location).
const COMBAT_STATE_COVER: u32 = 0x0097_fb60;
/// `0097b050`: the word at `CombatState+0xe4`.
const COMBAT_STATE_LAST_COVER: u32 = 0x0097_b050;
/// `0097ee90` (AL): compares the setting at `011ce9b4` with the age of the
/// timestamp at `CombatState+0x94` (true when the setting is greater).
const COMBAT_STATE_COVER_CHECK_DUE: u32 = 0x0097_ee90;
/// `0097f2f0` (AL): the byte at `CombatState+0x70`.
const COMBAT_STATE_FLAG: u32 = 0x0097_f2f0;
/// `009d6730`: stores `CombatState+0xa0` into its argument and returns it.
const COMBAT_STATE_TIME_COPY: u32 = 0x009d_6730;
/// `009a0680` (no stack argument).
const COMBAT_STATE_REFRESH: u32 = 0x009a_0680;
/// `CombatState::CheckCoverSafe` (Xbox PDB name of `0099ccd0`):
/// (state; cover+0x14, location, target, byte).
const CHECK_COVER_SAFE: u32 = 0x0099_ccd0;
/// `0099aa10` (`__thiscall` on the `CombatState`): (location, kind); for
/// kind 0xf it also calls `004fb070` / `0098bd70` on the location.
const ARRIVAL_STEP: u32 = 0x0099_aa10;
/// `0099e060` (`__thiscall` on the `CombatState`): a cover search (path
/// object, point, kind, range, avoid array), AL; on success it leaves the
/// found cover location in the word at `+0xe8`.
const FIND_COVER_LOCATION: u32 = 0x0099_e060;
/// `CombatState::BuildCoveredMoveRequest` (Xbox PDB name of `0099d310`).
const BUILD_COVERED_MOVE_REQUEST: u32 = 0x0099_d310;
/// `CombatState::GetLocationTargetDistance` (Xbox PDB name of
/// `009a0b00`), in `ST0`.
const GET_LOCATION_TARGET_DISTANCE: u32 = 0x009a_0b00;
/// `CombatPlan::ContainsAction` (Xbox PDB name of `00981940`): (the
/// controller; action).
const CONTAINS_ACTION: u32 = 0x0098_1940;
/// `009cc490` (`__thiscall` on the `CombatState`): ORs the mask into the
/// word at `+4`.
const STATE_OR_FLAGS: u32 = 0x009c_c490;
/// `009d4f30` (`__fastcall` on the `CombatState`): sets the byte at `+0x188`
/// to 0xff.
const STATE_SET_BYTE_188: u32 = 0x009d_4f30;
/// `009caac0` (`__thiscall`): (procedure; message): fails the procedure
/// with a message.
const FAIL_PROCEDURE: u32 = 0x009c_aac0;
/// `009caae0` (`__thiscall`): completes the procedure.
const COMPLETE_PROCEDURE: u32 = 0x009c_aae0;
/// `CombatUtilities::CalculateCoverPositions` (Xbox PDB name of
/// `009a9370`, cdecl): (controller, cover, out point, look-at position).
const CALCULATE_COVER_POSITIONS: u32 = 0x009a_9370;
/// `CombatUtilities::GetTargetFiringZOffset` (Xbox PDB name of
/// `009a8460`, cdecl): (actor, 2, 0), in `ST0`.
const GET_TARGET_FIRING_Z_OFFSET: u32 = 0x009a_8460;
/// `CombatUtilities::ComputeFaceTargetLocation` (Xbox PDB name of
/// `009a68b0`, cdecl): (out, actor, reference, location, -1.0).
const COMPUTE_FACE_TARGET_LOCATION: u32 = 0x009a_68b0;
/// `CombatUtilities::CalculateLOS` (Xbox PDB name of `009a7940`, cdecl):
/// (actor, reference, pathing location, location, 0, -1), AL.
const CALCULATE_LOS: u32 = 0x009a_7940;
/// `BGSWorldLocation::SetLocation` (Xbox PDB name of `00989010`):
/// (location; point, space).
const SET_LOCATION: u32 = 0x0098_9010;
/// `0098ddd0` (`__thiscall` on a location): (pathing location).
const SET_LOCATION_FROM_PATHING: u32 = 0x0098_ddd0;
/// `00989490` and `009892a0` (`__thiscall` on `004fb070(controller)`):
/// build the avoid-node array's entries for a cover search.
const BUILD_AVOID_NODES: u32 = 0x0098_9490;
const BUILD_AVOID_NODES_WITH_TARGET: u32 = 0x0098_92a0;
/// `004fb070`: the object the avoid-node builders are called on.
const AVOID_NODE_SOURCE: u32 = 0x004f_b070;
/// `00825c00` (`__thiscall` on the `CombatManager` global) and `009a57a0`
/// (`__thiscall` on its result): build the avoid-node array entry from
/// (position pointer, actor, -1.0).
const GET_THREAT_MAP: u32 = 0x0082_5c00;
const THREAT_MAP_NODE: u32 = 0x009a_57a0;
/// The `CombatManager*` global.
const COMBAT_MANAGER: u32 = 0x011f_1958;
/// `006dadb0` (`__thiscall`): stores a new array pointer into a
/// `NiPointer`.
const ARRAY_POINTER_SET: u32 = 0x006d_adb0;
/// `006b37b0` (one word) / `006b37e0`: constructor and destructor of the
/// avoid-node array smart pointer.
const ARRAY_POINTER_CONSTRUCT: u32 = 0x006b_37b0;
const ARRAY_POINTER_DESTRUCT: u32 = 0x006b_37e0;
/// `0098ddd0`'s companions: the new-array allocation (`00aa13e0`, cdecl)
/// and its constructor (`006e3850`).
const ALLOCATE: u32 = 0x00aa_13e0;
const ARRAY_CONSTRUCT: u32 = 0x006e_3850;
/// `BSSimpleArray<PathingAvoidNode_1024>::Add` (Xbox PDB name of
/// `00905450`): (array; node).
const AVOID_ARRAY_ADD: u32 = 0x0090_5450;
/// `006915d0` (`__thiscall` on a 20-byte node): (position, radius b, radius a).
const AVOID_NODE_CONSTRUCT: u32 = 0x0069_15d0;
/// `PathingRequest::PathingRequest` / destructor (`006e2420` / `006e2620`)
/// and `PathingRequestCoveredMove` constructor / destructor (`006e4a50` /
/// `006e4bf0`) (Xbox PDB names).
const PATHING_REQUEST_CONSTRUCT: u32 = 0x006e_2420;
const PATHING_REQUEST_DESTRUCT: u32 = 0x006e_2620;
const COVERED_MOVE_REQUEST_CONSTRUCT: u32 = 0x006e_4a50;
const COVERED_MOVE_REQUEST_DESTRUCT: u32 = 0x006e_4bf0;
/// `006e2980`: sets the goal (point, radius) on a request.
const REQUEST_SET_GOAL: u32 = 0x006e_2980;
/// `008e7c50` (`__thiscall`): (byte) on the chosen request.
const REQUEST_SET_FLAG: u32 = 0x008e_7c50;
/// `00705b10`: (request; kind).
const COVERED_REQUEST_SET_KIND: u32 = 0x0070_5b10;
/// `Actor::BuildRequest_ov2` (Xbox PDB name of `008b38d0`): (actor;
/// request, location, space, radius, avoid array).
const BUILD_REQUEST: u32 = 0x008b_38d0;
/// `Actor::SetPathfindingGoal` (Xbox PDB name of `008b3630`).
const SET_PATHFINDING_GOAL: u32 = 0x008b_3630;
/// `Actor::IsWaitingOnPath` / `IsPathValid` / `IsPathingComplete` (Xbox
/// PDB names of `008b3bf0`, `008b3960`, `008b3bb0`).
const IS_WAITING_ON_PATH: u32 = 0x008b_3bf0;
const IS_PATH_VALID: u32 = 0x008b_3960;
const IS_PATHING_COMPLETE: u32 = 0x008b_3bb0;
/// `008b3b90` (AL, on the actor): `Update` treats true as "the path failed"
/// (the name is not confirmed).
const PATH_FAILED_TEST: u32 = 0x008b_3b90;
/// `008b3980`: the actor's current path node index.
const CURRENT_PATH_NODE: u32 = 0x008b_3980;
/// `008b39a0` (`__thiscall` on the actor): (node, out point), AL.
const PATH_NODE_POINT: u32 = 0x008b_39a0;
/// `CombatTimer` calls (from `highprocess.cpp`): start with a delay
/// (`008d7f40`), expired (`008d7f80`, AL).
const TIMER_START: u32 = 0x008d_7f40;
const TIMER_EXPIRED: u32 = 0x008d_7f80;
/// `CombatTimer` has been started (`0097fa50`, AL).
const TIMER_RUNNING: u32 = 0x0097_fa50;
/// `006b9130` and `00435e00`: the two floats the debug text prints for a
/// timer (both `__thiscall` on the timer, in `ST0`).
const TIMER_VALUE_A: u32 = 0x006b_9130;
const TIMER_VALUE_B: u32 = 0x0043_5e00;
/// `006a7f50`: the float at the start of the object (`__thiscall`, `ST0`).
const FLOAT_AT: u32 = 0x006a_7f50;
/// `00435dd0`: the current game time (`ST0`).
const GAME_TIME_NOW: u32 = 0x0043_5dd0;
/// `00435de0` (`__thiscall`, one float): stores the value at `this`.
const TIME_STAMP_SET: u32 = 0x0043_5de0;
/// A setting's value pointer (`00403e20`, `this` = the setting).
const SETTING_VALUE: u32 = 0x0040_3e20;
/// The `Setting` objects the update reads (their values are read through
/// `00403e20`; the names of the settings are not known).
/// The delay that starts `CheckDestinationTimer` for kind 0xf.
const SETTING_011CE9C0: u32 = 0x011c_e9c0;
/// The squared limits the destination follows the target by: in the cover
/// state (`011cf7b8`), otherwise (`011ce494`), and when either actor has a
/// process (`011ce654`).
const SETTING_011CF7B8: u32 = 0x011c_f7b8;
const SETTING_011CE494: u32 = 0x011c_e494;
const SETTING_011CE654: u32 = 0x011c_e654;
/// The squared distance inside which kind 0xf tests the line of sight.
const SETTING_011CF2A0: u32 = 0x011c_f2a0;
/// The delay that restarts `CheckDestinationTimer`.
const SETTING_011CFAD4: u32 = 0x011c_fad4;
/// The two radii of the target avoid node.
const SETTING_011CF120: u32 = 0x011c_f120;
const SETTING_011CF4E0: u32 = 0x011c_f4e0;
/// Vtable calls on a reference: slot `0x100` (AL), `0x130` (name), `0x1f4`
/// (position pointer); on a procedure's target: slot `0x10` (type).
const SLOT_TYPE: u32 = 0x10;
const SLOT_HAS_PROCESS: u32 = 0x100;
const SLOT_NAME: u32 = 0x130;
const SLOT_POSITION: u32 = 0x1f4;
/// `00474cb0` (`__thiscall`) and `0055d520`: a reference's name by two
/// routes; `0084e3a0`: its id (also `BGSWorldLocation::pSpace`).
const HAS_NAME: u32 = 0x0047_4cb0;
const FALLBACK_NAME: u32 = 0x0055_d520;
const FORM_ID: u32 = 0x0084_e3a0;
/// `00440da0` / `00440d80` (AL): the reference is disabled / deleted.
const REF_DISABLED: u32 = 0x0044_0da0;
const REF_DELETED: u32 = 0x0044_0d80;
/// `006815c0`: returns its `this` (a constructor-like identity).
const IDENTITY: u32 = 0x0068_15c0;
/// `006377e0`: the address `+0xc8` of the object; `004f8960`: the word at
/// `+0x108`; `0068efb0` / `006d3100`: get / set the word at `+0x104`;
/// `009d7c60`: the byte at `+0xe1` (all on a queued pathing request).
const REQUEST_POINT: u32 = 0x0063_77e0;
const REQUEST_WORD_108: u32 = 0x004f_8960;
const REQUEST_GET_104: u32 = 0x0068_efb0;
const REQUEST_SET_104: u32 = 0x006d_3100;
const REQUEST_BYTE_E1: u32 = 0x009d_7c60;
/// `00696540` (cdecl): line test (a, b, c, d), AL; `005c3420`: builds the
/// first point (this = cover+0x14; out).
const LINE_TEST: u32 = 0x0069_6540;
const COVER_POINT: u32 = 0x005c_3420;
/// `PathingLocation` (Xbox PDB names): `006dce10` (`PathingLocation_ov4`,
/// (location, actor)), `006dd220` (`PathingLocation_ov8`, (location)),
/// `006ded70` (`IsValidLocation`, (radius), AL), destructor `004ff7e0`.
const PATHING_LOCATION_NEW_FROM_LOCATION: u32 = 0x006d_ce10;
const PATHING_LOCATION_NEW_FROM_POINT: u32 = 0x006d_d220;
const PATHING_LOCATION_IS_VALID: u32 = 0x006d_ed70;
const PATHING_LOCATION_DESTRUCT: u32 = 0x004f_f7e0;
/// `Pathing::CheckLineOfSight_ov2` (Xbox PDB name of `006d7490`, cdecl):
/// (pathing location, location, 0, 0), AL.
const CHECK_LINE_OF_SIGHT: u32 = 0x006d_7490;
/// `NiPoint3` helpers: difference (`00439ef0`: this = a; out, b), unitize
/// (`00457910`: this, returns the length in `ST0`), scale (`00439180`:
/// this; factor), sum (`00439e90`: this = a; out, b), squared length
/// (`004a7290`, `ST0`), distance test (`00439090`: this; other, AL).
const POINT_DIFFERENCE: u32 = 0x0043_9ef0;
const POINT_UNITIZE: u32 = 0x0045_7910;
const POINT_SCALE: u32 = 0x0043_9180;
const POINT_SUM: u32 = 0x0043_9e90;
const POINT_LENGTH_SQUARED: u32 = 0x004a_7290;
const POINTS_NEAR: u32 = 0x0043_9090;
/// The path search objects (`006d3200` constructor, `006d32c0`
/// destructor).
const SEARCH_CONSTRUCT: u32 = 0x006d_3200;
const SEARCH_DESTRUCT: u32 = 0x006d_32c0;
/// `007058c0` (`__thiscall` on the combat state): a pointer to a byte
/// record; `00461580` (`__thiscall`): (flags), AL.
const COVER_RECORD: u32 = 0x0070_58c0;
const COVER_RECORD_TEST: u32 = 0x0046_1580;
/// `Actor::GetRadius` (Xbox PDB name of `008be1b0`), in `ST0`.
const GET_RADIUS: u32 = 0x008b_e1b0;

// ---- Globals the update reads ----

/// Default goal point words (three floats) of `CreatePath`.
const DEFAULT_GOAL: u32 = 0x011f_426c;
/// Byte global: covered-move requests are only built when it is set, and
/// the cover search in `Update` only runs when it is clear.
const COVER_MODE_BYTE: u32 = 0x011a_5bfc;
/// Eight-byte table indexed by the move kind: byte 0 at `+0x11a5652`
/// (covered moves allowed), byte 1 at `+0x11a5653` (avoid nodes wanted).
const KIND_TABLE: u32 = 0x011a_5652;
const KIND_TABLE_AVOID: u32 = 0x011a_5653;
/// Float / double constants of the exe (read at run time).
const HALF: u32 = 0x0101_6248;
const TWO: u32 = 0x0101_62c0;
const QUARTER: u32 = 0x0101_622c;
const MINUS_ONE: u32 = 0x0101_2054;
const ONE_DOUBLE: u32 = 0x0101_2070;
const FAR_RADIUS: u32 = 0x0103_a1c4;
const SEARCH_RANGE_FIRST: u32 = 0x0102_31a0;
const SEARCH_RANGE: u32 = 0x0103_2adc;
const MAX_SEARCH_LENGTH: u32 = 0x0109_1bf8;
const MAX_COVER_DISTANCE_SQUARED: u32 = 0x0109_1c00;

/// The size of the block that stands for `Update`'s stack frame and the
/// offset of `EBP` in it. The deepest local is at `EBP - 0x1ac`.
const UPDATE_FRAME: u32 = 0x1c0;
/// The same for `CreatePath_ov2` (deepest local `EBP - 0x278`).
const CREATE_PATH_FRAME: u32 = 0x290;

/// Copies the four words of a `BGSWorldLocation`.
fn copy_location(e: &mut Engine, from: u32, to: u32) {
    for i in 0..4 {
        let word = e.mem.u32(from + i * 4);
        e.mem.set_u32(to + i * 4, word);
    }
}

/// `__thiscall` getter of a setting's `float`: `00403e20(setting)`
/// returns the address of the value.
fn setting_float(e: &mut Engine, setting: u32) -> f32 {
    let value = e.call(SETTING_VALUE, &args![setting]).u32();
    e.mem.f32(value)
}

/// Starts the timer at `timer` with `delay` (`008d7f40`).
fn start_timer(e: &mut Engine, timer: u32, delay: f32) {
    e.call(TIMER_START, &args![timer, delay]);
}

/// The byte of `KIND_TABLE` (or `KIND_TABLE_AVOID`) for a move kind; the
/// code indexes with the signed `iLocation`.
fn kind_flag(e: &Engine, table: u32, kind: i32) -> u8 {
    e.mem.u8(table.wrapping_add((kind as u32).wrapping_mul(8)))
}

// Translated from 009d6b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureMove::GetType` (Xbox PDB): the procedure type, 6.
pub fn get_type(_e: &mut Engine, _this: Ptr<CombatProcedureMove>) -> u32 {
    6
}

// Translated from 009d6b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureMove::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor body (`009d6ba0`), then `operator delete` when bit 0 of
/// `flags` is set. Returns `this`.
pub fn scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<CombatProcedureMove>,
    flags: u32,
) -> Ptr<CombatProcedureMove> {
    fn_009d6ba0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this.addr()]);
    }
    this
}

// Translated from 009d6ba0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009d6ba0 - the compiler's FS:[0] exception frame (unwinding if the member destructor throws) is not translated
/// The destructor body of `CombatProcedureMove` (the deleting destructor
/// `009d6b70` calls it; Ghidra labels the folded code as a library
/// `time_put` destructor, which it is not): stores the class's vtable,
/// destroys the queued pathing request's `NiPointer` and runs the base
/// destructor (`009ca6c0`).
pub fn fn_009d6ba0(e: &mut Engine, this: Ptr<CombatProcedureMove>) {
    e.mem.set_u32(this.addr(), VTABLE);
    e.call(
        SMART_POINTER_DESTRUCT,
        &args![this.addr() + CombatProcedureMove::spQueuedPathingRequest.off],
    );
    e.call(BASE_DESTRUCT, &args![this.addr()]);
}

// Translated from 009d6c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureMove::Shutdown` (Xbox PDB): while moving (states 1 and
/// 2) the actor stops (unless `00666f00` says otherwise); while hiding
/// (state 3) the look-at target is cleared; then the move mode is cleared
/// and a reserved secondary cover location released.
pub fn shutdown(e: &mut Engine, this: Ptr<CombatProcedureMove>) {
    let controller = e.get(this, CombatProcedureMove::pCombatController);
    let actor = e.call(CONTROLLER_ACTOR, &args![controller.addr()]).u32();
    let state = e.get(this, CombatProcedureMove::eState);
    if state == 1 || state == 2 {
        if !e.call(SHUTDOWN_TEST, &args![this.addr()]).bool() {
            e.call(STOP_MOVING, &args![actor]);
        }
    } else if state == 3 {
        e.call(CLEAR_LOOK_AT_TARGET, &args![this.addr()]);
    }
    e.call(CLEAR_MOVE_MODE, &args![this.addr()]);
    if e.get(this, CombatProcedureMove::bReservedSecondaryLocation) {
        let combat_state = e.call(GET_COMBAT_STATE, &args![controller.addr()]).u32();
        e.call(UNRESERVE_COVER_LOCATION, &args![combat_state]);
    }
}

// Translated from 009d7bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit `bit` (taken modulo 32, as `SHL` does) of the word at
/// `+0x90` of the `CombatState` is set.
pub fn fn_009d7bd0(e: &mut Engine, this: u32, bit: u32) -> bool {
    let mask = 1u32 << (bit & 31);
    e.mem.u32(this + 0x90) & mask != 0
}

// Translated from 009d7c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `00997520` on the member at `+0x128` of the `CombatState`.
pub fn fn_009d7c00(e: &mut Engine, this: u32, value: u32) {
    e.call(STATE_COPY_RECORD, &args![this + 0x128, value]);
}

// Translated from 009d7c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `009a0560` on the `CombatState` with the word at `+0xe8`; returns
/// its result unchanged (the callers test `AL`).
pub fn fn_009d7c40(e: &mut Engine, this: u32) -> u32 {
    let cover = e.mem.u32(this + 0xe8);
    e.call(STATE_CHECK_LOCATION, &args![this, cover]).u32()
}

// Translated from 009d7c80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009d7c80 - the compiler's FS:[0] exception frame (unwinding of the avoid-node smart pointer) is not translated
/// `CombatProcedureMove::CreatePath` (Xbox PDB): builds the avoid-node
/// array (`CreateAvoidNodeArray`) in a local smart pointer, creates the
/// path with it (`CreatePath_ov2`), destroys the pointer and returns the
/// result.
pub fn create_path(e: &mut Engine, this: Ptr<CombatProcedureMove>) -> bool {
    e.with_stack(8, |e, avoid| {
        e.call(ARRAY_POINTER_CONSTRUCT, &args![avoid.addr(), 0u32]);
        create_avoid_node_array(e, this, avoid.addr());
        let result = create_path_ov2(e, this, avoid.addr());
        e.call(ARRAY_POINTER_DESTRUCT, &args![avoid.addr()]);
        result
    })
}

// Translated from 009d7d00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009d7d00 - the compiler's FS:[0] exception frame (unwinding of the request objects and the temporary location) is not translated
/// `CombatProcedureMove::CreatePath_ov2` (Xbox PDB): sets up the pathing
/// request for the procedure (a plain `PathingRequest`, or the covered-move
/// one when the cover flags say so), gives it the goal and radius and hands
/// it to the actor (`Actor::SetPathfindingGoal`). `avoid` is the address of
/// the avoid-node array's smart pointer. Returns whether the actor accepted
/// the goal.
///
/// Locals follow the disassembly's frame (offsets below `EBP`): the actor
/// `-0x1c`, the goal point `-0x18..-0x0c`, the combat state `-0x140`, the
/// goal radius `-0x13c`, the move mode `-0x138`, the target radius
/// `-0x144`, the covered-move request `-0x134`, the plain request `-0x204`.
pub fn create_path_ov2(e: &mut Engine, this: Ptr<CombatProcedureMove>, avoid: u32) -> bool {
    e.with_stack(CREATE_PATH_FRAME, |e, base| {
        let ebp = base.addr() + CREATE_PATH_FRAME;
        let this_addr = this.addr();
        let goal = ebp - 0x18;
        let attacker = e.call(GET_ATTACKER, &args![this_addr]).u32();
        let _target = e.call(GET_TARGET, &args![this_addr]).u32();
        let controller = e.get(this, CombatProcedureMove::pCombatController);
        let combat_state = e.call(GET_COMBAT_STATE, &args![controller.addr()]).u32();
        let mut move_mode = 3u32;
        for i in 0..3 {
            let word = e.mem.u32(DEFAULT_GOAL + i * 4);
            e.mem.set_u32(goal + i * 4, word);
        }
        let mut goal_radius = 0.0f32;
        let kind = e.get(this, CombatProcedureMove::iLocation);
        if kind == 0 {
            if e.call(CONTAINS_ACTION, &args![controller.addr(), 0x18u32])
                .bool()
            {
                move_mode = 4;
            }
        } else if kind == 3 {
            let position = e.vcall(attacker, SLOT_POSITION, &args![]).u32();
            for i in 0..3 {
                let word = e.mem.u32(position + i * 4);
                e.mem.set_u32(goal + i * 4, word);
            }
            goal_radius = e.global::<f32>(FAR_RADIUS);
        }
        let record = e.call(COVER_RECORD, &args![combat_state]).u32();
        if record != 0
            && e.mem.u8(record) == 1
            && !e
                .call(COVER_RECORD_TEST, &args![combat_state, 0x100u32])
                .bool()
            && e.get(this, CombatProcedureMove::iLocation) != 8
        {
            let record_space = e.call(FORM_ID, &args![record + 8]).u32();
            let attacker_space = e.call(GET_SPACE, &args![attacker]).u32();
            if record_space == attacker_space {
                let point = e.call(IDENTITY, &args![record + 8]).u32();
                for i in 0..3 {
                    let word = e.mem.u32(point + i * 4);
                    e.mem.set_u32(goal + i * 4, word);
                }
                goal_radius = e
                    .call(GET_LOCATION_TARGET_DISTANCE, &args![combat_state])
                    .f32();
                let position = e.vcall(attacker, SLOT_POSITION, &args![]).u32();
                let difference = ebp - 0x210;
                e.call(POINT_DIFFERENCE, &args![goal, difference, position]);
                let length_squared = e.call(POINT_LENGTH_SQUARED, &args![difference]).f64();
                // FCOMPP of (radius * radius) against the squared length.
                if (goal_radius as f64) * (goal_radius as f64) < length_squared {
                    goal_radius = 0.0;
                }
            }
        }
        let mut target_radius = 0.0f32;
        if e.get(this, CombatProcedureMove::eState) == 1 {
            target_radius = e.get(this, CombatProcedureMove::fTargetRadius);
        }
        let target_ref = e.get(this, CombatProcedureMove::pTargetRef);
        if e.get(this, CombatProcedureMove::iLocation) == 1
            && e.get(this, CombatProcedureMove::eState) == 1
            && !target_ref.is_null()
        {
            let face = ebp - 0x21c;
            let pathing = ebp - 0x244;
            let minus_one = e.global::<f32>(MINUS_ONE);
            let location = e.call(IDENTITY, &args![this_addr + DESTINATION]).u32();
            e.call(
                COMPUTE_FACE_TARGET_LOCATION,
                &args![face, attacker, target_ref.addr(), location, minus_one],
            );
            e.call(
                PATHING_LOCATION_NEW_FROM_LOCATION,
                &args![pathing, face, attacker],
            );
            let radius = e.call(GET_RADIUS, &args![attacker]).f32();
            if e.call(PATHING_LOCATION_IS_VALID, &args![pathing, radius])
                .bool()
            {
                let location = e.call(IDENTITY, &args![this_addr + DESTINATION]).u32();
                let line_of_sight = e
                    .call(
                        CALCULATE_LOS,
                        &args![
                            attacker,
                            target_ref.addr(),
                            face,
                            location,
                            0u32,
                            0xffff_ffffu32
                        ],
                    )
                    .bool();
                if line_of_sight {
                    e.call(
                        SET_LOCATION_FROM_PATHING,
                        &args![this_addr + CURRENT_DESTINATION, face],
                    );
                    let array = e.call(SMART_POINTER_GET, &args![avoid]).u32();
                    if array == 0 {
                        let block = e.call(ALLOCATE, &args![0x18u32]).u32();
                        let created = if block != 0 {
                            e.call(ARRAY_CONSTRUCT, &args![block]).u32()
                        } else {
                            0
                        };
                        e.call(ARRAY_POINTER_SET, &args![avoid, created]);
                    }
                    let radius_a = setting_float(e, SETTING_011CF120);
                    let radius_b = setting_float(e, SETTING_011CF4E0);
                    let position = e.vcall(target_ref.addr(), SLOT_POSITION, &args![]).u32();
                    let node_slot = ebp - 0x270;
                    let node = e
                        .call(
                            AVOID_NODE_CONSTRUCT,
                            &args![node_slot, position, radius_b, radius_a],
                        )
                        .u32();
                    let array = e.call(SMART_POINTER_GET, &args![avoid]).u32();
                    e.call(AVOID_ARRAY_ADD, &args![array, node]);
                }
            }
            e.call(PATHING_LOCATION_DESTRUCT, &args![pathing]);
        }
        let plain_request = ebp - 0x204;
        let covered_request = ebp - 0x134;
        e.call(PATHING_REQUEST_CONSTRUCT, &args![plain_request]);
        e.call(COVERED_MOVE_REQUEST_CONSTRUCT, &args![covered_request]);
        let kind = e.get(this, CombatProcedureMove::iLocation);
        let chosen = if e.mem.u8(COVER_MODE_BYTE) != 0
            && kind_flag(e, KIND_TABLE, kind) != 0
            && fn_009d7bd0(e, combat_state, 1)
            && e.call(GET_PROCESS_TYPE, &args![attacker]).u32() == 0
        {
            e.call(
                BUILD_COVERED_MOVE_REQUEST,
                &args![combat_state, covered_request],
            );
            e.call(
                COVERED_REQUEST_SET_KIND,
                &args![covered_request, kind as u32],
            );
            covered_request
        } else {
            plain_request
        };
        let negated = target_radius < 0.0;
        if negated {
            target_radius = -target_radius;
        }
        let array = e.call(SMART_POINTER_GET, &args![avoid]).u32();
        let space = e
            .call(FORM_ID, &args![this_addr + CURRENT_DESTINATION])
            .u32();
        let location = e
            .call(IDENTITY, &args![this_addr + CURRENT_DESTINATION])
            .u32();
        e.call(
            BUILD_REQUEST,
            &args![attacker, chosen, location, space, target_radius, array],
        );
        e.call(REQUEST_SET_GOAL, &args![chosen, goal, goal_radius]);
        if negated {
            e.call(REQUEST_SET_FLAG, &args![chosen, 0u32]);
        }
        let accepted = e
            .call(SET_PATHFINDING_GOAL, &args![attacker, chosen])
            .bool();
        if !accepted {
            e.call(FAIL_PROCEDURE, &args![this_addr, CANNOT_SETUP_PATH_MESSAGE]);
        } else {
            e.set(this, CombatProcedureMove::iCurrentNode, -1);
            let request = e.call(GET_CURRENT_PATHING_REQUEST, &args![attacker]).u32();
            e.call(
                SMART_POINTER_SET,
                &args![
                    this_addr + CombatProcedureMove::spQueuedPathingRequest.off,
                    request
                ],
            );
            e.call(SET_MOVE_MODE, &args![this_addr, move_mode]);
        }
        e.call(COVERED_MOVE_REQUEST_DESTRUCT, &args![covered_request]);
        e.call(PATHING_REQUEST_DESTRUCT, &args![plain_request]);
        accepted
    })
}

/// `"Cannot setup path to destination"`.
const CANNOT_SETUP_PATH_MESSAGE: u32 = 0x0109_1c80;

// Translated from 009d82e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureMove::CreateAvoidNodeArray` (Xbox PDB): fills the
/// smart pointer at `out` with the avoid-node array for the procedure: a
/// node from the threat map around the attacker, stamps the combat state's
/// timestamp at `+0xc0` with the game time, then (by move kind) adds the
/// avoid nodes for kind 4, or reads the target for kinds 0 and 1, and adds
/// the target-aware avoid nodes when the kind table asks for them.
pub fn create_avoid_node_array(e: &mut Engine, this: Ptr<CombatProcedureMove>, out: u32) {
    let this_addr = this.addr();
    let attacker = e.call(GET_ATTACKER, &args![this_addr]).u32();
    let controller = e.get(this, CombatProcedureMove::pCombatController);
    let combat_state = e.call(GET_COMBAT_STATE, &args![controller.addr()]).u32();
    let minus_one = e.global::<f32>(MINUS_ONE);
    let position = e.vcall(attacker, SLOT_POSITION, &args![]).u32();
    let manager = e.mem.u32(COMBAT_MANAGER);
    let threat_map = e.call(GET_THREAT_MAP, &args![manager]).u32();
    let node = e
        .call(
            THREAT_MAP_NODE,
            &args![threat_map, position, attacker, minus_one],
        )
        .u32();
    e.call(ARRAY_POINTER_SET, &args![out, node]);
    let now = e.call(GAME_TIME_NOW, &args![]).f32();
    fn_009d8410(e, combat_state, now);
    let mut target = 0u32;
    let kind = e.get(this, CombatProcedureMove::iLocation);
    if kind >= 0 {
        if kind > 1 {
            if kind == 4 {
                let array = e.call(SMART_POINTER_GET, &args![out]).u32();
                let location = e.call(IDENTITY, &args![this_addr + DESTINATION]).u32();
                let source = e.call(AVOID_NODE_SOURCE, &args![controller.addr()]).u32();
                let result = e
                    .call(
                        BUILD_AVOID_NODES,
                        &args![source, attacker, location, 0.0f32, array],
                    )
                    .u32();
                e.call(ARRAY_POINTER_SET, &args![out, result]);
            }
        } else {
            target = e.call(GET_TARGET, &args![this_addr]).u32();
        }
    }
    let kind = e.get(this, CombatProcedureMove::iLocation);
    if kind_flag(e, KIND_TABLE_AVOID, kind) != 0 {
        let array = e.call(SMART_POINTER_GET, &args![out]).u32();
        let location = e.call(IDENTITY, &args![this_addr + DESTINATION]).u32();
        let source = e.call(AVOID_NODE_SOURCE, &args![controller.addr()]).u32();
        let result = e
            .call(
                BUILD_AVOID_NODES_WITH_TARGET,
                &args![source, attacker, location, 0.0f32, array, target],
            )
            .u32();
        e.call(ARRAY_POINTER_SET, &args![out, result]);
    }
}

// Translated from 009d8410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a timestamp of `value` (`00435de0` on a local) at `+0xc0` of the
/// `CombatState`.
pub fn fn_009d8410(e: &mut Engine, this: u32, value: f32) {
    e.with_stack(4, |e, stamp| {
        e.call(TIME_STAMP_SET, &args![stamp.addr(), value]);
        let word = e.mem.u32(stamp.addr());
        e.mem.set_u32(this + 0xc0, word);
    });
}

// Translated from 009d8440 (decompiled, FalloutNV.exe 1.4.0.525)
/// The delay (a `float` in `ST0`) after which hiding behind cover ends,
/// by move kind: 2.0 for kinds 6 and 7, 0.5 for kind 8, else 1.0.
pub fn fn_009d8440(e: &mut Engine, this: Ptr<CombatProcedureMove>) -> f32 {
    let kind = e.get(this, CombatProcedureMove::iLocation);
    if kind >= 6 {
        if kind <= 7 {
            return e.global::<f32>(TWO);
        }
        if kind == 8 {
            return e.global::<f32>(HALF);
        }
    }
    1.0
}

// Translated from 009d8480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureMove::ClearReferencePointers` (Xbox PDB): when the
/// cleared reference is the target, forgets it and fails the procedure
/// with a message (`"Targeted reference pointer was cleared"`).
pub fn clear_reference_pointers(e: &mut Engine, this: Ptr<CombatProcedureMove>, reference: Ptr) {
    if e.get(this, CombatProcedureMove::pTargetRef) == reference {
        e.set(this, CombatProcedureMove::pTargetRef, Ptr::NULL);
        e.call(FAIL_PROCEDURE, &args![this.addr(), TARGET_CLEARED_MESSAGE]);
    }
}

/// `"Targeted reference pointer was cleared"`.
const TARGET_CLEARED_MESSAGE: u32 = 0x0109_08e4;

// Translated from 009d84b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureMove::StartedPlanning` (Xbox PDB): releases the reserved
/// secondary cover location, if any.
pub fn started_planning(e: &mut Engine, this: Ptr<CombatProcedureMove>) {
    if e.get(this, CombatProcedureMove::bReservedSecondaryLocation) {
        let controller = e.get(this, CombatProcedureMove::pCombatController);
        let combat_state = e.call(GET_COMBAT_STATE, &args![controller.addr()]).u32();
        e.call(UNRESERVE_COVER_LOCATION, &args![combat_state]);
        e.set(this, CombatProcedureMove::bReservedSecondaryLocation, false);
    }
}

// Translated from 009d84e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureMove::FinishedPlanning` (Xbox PDB): in the cover states
/// (2 and 3) reserves the secondary cover location.
pub fn finished_planning(e: &mut Engine, this: Ptr<CombatProcedureMove>) {
    let state = e.get(this, CombatProcedureMove::eState);
    if state == 2 || state == 3 {
        let controller = e.get(this, CombatProcedureMove::pCombatController);
        let combat_state = e.call(GET_COMBAT_STATE, &args![controller.addr()]).u32();
        e.call(RESERVE_COVER_LOCATION, &args![combat_state]);
        e.set(this, CombatProcedureMove::bReservedSecondaryLocation, true);
    }
}

/// Slot `0x14` of the printer object `DisplayDebugText` receives: a
/// `cdecl` print function taking the object as its first stack word, two
/// words that `DisplayDebugText` passes through, a format string and the
/// values.
const SLOT_PRINT: u32 = 0x14;

/// One call of the printer: `printer->Print(x, y, format, ...)`.
fn print_line(e: &mut Engine, printer: u32, x: u32, y: u32, format: u32, values: &[u32]) {
    let mut words = args![x, y, format];
    words.extend_from_slice(values);
    e.vcall(printer, SLOT_PRINT, &words);
}

/// The two values the debug text prints for a timer: `00435e00` (printed
/// first) and `006b9130`, each a `float` in `ST0` widened to a `double`
/// (the stack order of the disassembly: `006b9130` is stored first, so the
/// second value of the format is its result).
fn timer_values(e: &mut Engine, timer: u32) -> Vec<u32> {
    let second = e.call(TIMER_VALUE_A, &args![timer]).f64();
    let first = e.call(TIMER_VALUE_B, &args![timer]).f64();
    args![first, second].into()
}

// Translated from 009d8520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureMove::DisplayDebugText` (Xbox PDB): prints the state's
/// name and its timers, the destination (with the name of its space when it
/// is not the attacker's), the target and the destination timer through the
/// printer object (`printer`), passing `x` and `y` through.
pub fn display_debug_text(
    e: &mut Engine,
    this: Ptr<CombatProcedureMove>,
    printer: u32,
    x: u32,
    y: u32,
) {
    let this_addr = this.addr();
    match e.get(this, CombatProcedureMove::eState) {
        0 => print_line(e, printer, x, y, TEXT_INITIALIZING, &[]),
        1 => print_line(e, printer, x, y, TEXT_MOVING, &[]),
        2 => {
            print_line(e, printer, x, y, TEXT_MOVING_USING_COVER, &[]);
            let values = timer_values(e, this_addr + CHECK_COVER_TIMER);
            print_line(e, printer, x, y, TEXT_CHECK_COVER_TIMER, &values);
        }
        3 => {
            print_line(e, printer, x, y, TEXT_HIDING_BEHIND_COVER, &[]);
            let values = timer_values(e, this_addr + HIDING_TIMER);
            print_line(e, printer, x, y, TEXT_HIDING_TIMER, &values);
            let values = timer_values(e, this_addr + CHECK_COVER_TIMER);
            print_line(e, printer, x, y, TEXT_CHECK_COVER_TIMER, &values);
        }
        _ => {}
    }
    let destination = e.call(IDENTITY, &args![this_addr + DESTINATION]).u32();
    let point = [
        e.mem.f32(destination),
        e.mem.f32(destination + 4),
        e.mem.f32(destination + 8),
    ];
    let space = e.call(FORM_ID, &args![this_addr + DESTINATION]).u32();
    let mut named = false;
    if space != 0 {
        let controller = e.get(this, CombatProcedureMove::pCombatController);
        let actor = e.call(CONTROLLER_ACTOR, &args![controller.addr()]).u32();
        let attacker_space = e.call(GET_SPACE, &args![actor]).u32();
        if space != attacker_space {
            let name = e.vcall(space, SLOT_NAME, &args![]).u32();
            let values = args![name, point[0] as f64, point[1] as f64, point[2] as f64];
            print_line(e, printer, x, y, TEXT_DESTINATION_WITH_SPACE, &values);
            named = true;
        }
    }
    if !named {
        let values = args![point[0] as f64, point[1] as f64, point[2] as f64];
        print_line(e, printer, x, y, TEXT_DESTINATION, &values);
    }
    let target = e.get(this, CombatProcedureMove::pTargetRef);
    if !target.is_null() {
        let position = e.vcall(target.addr(), SLOT_POSITION, &args![]).u32();
        let point = [
            e.mem.f32(position),
            e.mem.f32(position + 4),
            e.mem.f32(position + 8),
        ];
        let name = if e.call(HAS_NAME, &args![target.addr()]).u32() != 0 {
            e.vcall(target.addr(), SLOT_NAME, &args![]).u32()
        } else {
            e.call(FALLBACK_NAME, &args![target.addr()]).u32()
        };
        let id = e.call(FORM_ID, &args![target.addr()]).u32();
        let values = args![name, id, point[0] as f64, point[1] as f64, point[2] as f64];
        print_line(e, printer, x, y, TEXT_TARGET, &values);
    }
    if e.call(TIMER_RUNNING, &args![this_addr + CHECK_DESTINATION_TIMER])
        .bool()
    {
        let values = timer_values(e, this_addr + CHECK_DESTINATION_TIMER);
        print_line(e, printer, x, y, TEXT_CHECK_DESTINATION_TIMER, &values);
    }
}

/// The debug texts (strings in the exe): `"INITIALIZING"`, `"MOVING"`,
/// `"MOVING_USING_COVER"`, `"Check Cover Timer: %.02f/%.02f"`,
/// `"HIDING_BEHIND_COVER"`, `"Hiding Timer: %.02f/%.02f"`,
/// `"Destination: %s (%.02f,%.02f,%.02f)"`,
/// `"Destination: (%.02f,%.02f,%.02f)"`,
/// `"Target: '%s' (%08X) - Position: (%.02f,%.02f,%.02f)"`,
/// `"Check Destination Timer: %.02f/%.02f"`.
const TEXT_INITIALIZING: u32 = 0x0109_0b68;
const TEXT_MOVING: u32 = 0x0109_1d5c;
const TEXT_MOVING_USING_COVER: u32 = 0x0109_1d48;
const TEXT_CHECK_COVER_TIMER: u32 = 0x0109_1d28;
const TEXT_HIDING_BEHIND_COVER: u32 = 0x0109_1d14;
const TEXT_HIDING_TIMER: u32 = 0x0109_1b30;
const TEXT_DESTINATION_WITH_SPACE: u32 = 0x0109_1cf0;
const TEXT_DESTINATION: u32 = 0x0109_1ccc;
const TEXT_TARGET: u32 = 0x0109_090c;
const TEXT_CHECK_DESTINATION_TIMER: u32 = 0x0109_1ca4;

// Translated from 009d8880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureMove::SaveGame` (Xbox PDB): the base class, then the
/// state, both locations, the target radius, the target's form id, the
/// location kind, the current node, the four timers (covered-move search,
/// hiding, check cover, check destination order), the look-at position,
/// the reserved flag, and whether a queued pathing request exists.
pub fn save_game(e: &mut Engine, this: Ptr<CombatProcedureMove>, buffer: u32) {
    let t = this.addr();
    e.call(BASE_SAVE_GAME, &args![t, buffer]);
    e.call(SAVE_BYTES, &args![buffer, t + 0x10, 4u32, 0u32]);
    e.call(WORLD_LOCATION_SAVE, &args![t + DESTINATION, buffer]);
    e.call(WORLD_LOCATION_SAVE, &args![t + CURRENT_DESTINATION, buffer]);
    e.call(SAVE_BYTES, &args![buffer, t + 0x34, 4u32, 0u32]);
    let target = e.get(this, CombatProcedureMove::pTargetRef);
    e.call(SAVE_FORM_ID, &args![buffer, target.addr(), 0u32]);
    e.call(SAVE_BYTES, &args![buffer, t + 0x3c, 4u32, 0u32]);
    e.call(SAVE_BYTES, &args![buffer, t + 0x60, 4u32, 0u32]);
    e.call(TIMER_SAVE, &args![t + COVERED_MOVE_SEARCH_TIMER, buffer]);
    e.call(TIMER_SAVE, &args![t + HIDING_TIMER, buffer]);
    e.call(TIMER_SAVE, &args![t + CHECK_COVER_TIMER, buffer]);
    e.call(TIMER_SAVE, &args![t + CHECK_DESTINATION_TIMER, buffer]);
    e.call(
        SAVE_BYTES,
        &args![buffer, t + LOOK_AT_POSITION, 0xcu32, 0u32],
    );
    e.call(SAVE_BYTES, &args![buffer, t + 0x54, 1u32, 0u32]);
    let queued = e
        .call(
            SMART_POINTER_GET,
            &args![t + CombatProcedureMove::spQueuedPathingRequest.off],
        )
        .u32();
    e.set(
        this,
        CombatProcedureMove::bHasQueuedPathingRequest,
        queued != 0,
    );
    e.call(SAVE_BYTES, &args![buffer, t + 0x5c, 1u32, 0u32]);
}

// Translated from 009d89a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureMove::LoadGame` (Xbox PDB): the mirror of `SaveGame`;
/// the check-destination timer is only in saves of version 8 or later and
/// the queued-request flag in version 12 or later (the buffer's version is
/// its virtual function at slot 0).
pub fn load_game(e: &mut Engine, this: Ptr<CombatProcedureMove>, buffer: u32) {
    let t = this.addr();
    e.call(BASE_LOAD_GAME, &args![t, buffer]);
    e.call(LOAD_BYTES, &args![buffer, t + 0x10, 4u32]);
    e.call(WORLD_LOCATION_LOAD, &args![t + DESTINATION, buffer]);
    e.call(WORLD_LOCATION_LOAD, &args![t + CURRENT_DESTINATION, buffer]);
    e.call(LOAD_BYTES, &args![buffer, t + 0x34, 4u32]);
    e.call(LOAD_FORM_ID, &args![buffer, t + 0x38]);
    e.call(LOAD_BYTES, &args![buffer, t + 0x3c, 4u32]);
    e.call(LOAD_BYTES, &args![buffer, t + 0x60, 4u32]);
    e.call(TIMER_LOAD, &args![t + COVERED_MOVE_SEARCH_TIMER, buffer]);
    e.call(TIMER_LOAD, &args![t + HIDING_TIMER, buffer]);
    e.call(TIMER_LOAD, &args![t + CHECK_COVER_TIMER, buffer]);
    let version = e.vcall(buffer, 0, &args![]).u8();
    if version >= 8 {
        e.call(TIMER_LOAD, &args![t + CHECK_DESTINATION_TIMER, buffer]);
    }
    e.call(LOAD_BYTES, &args![buffer, t + LOOK_AT_POSITION, 0xcu32]);
    e.call(LOAD_BYTES, &args![buffer, t + 0x54, 1u32]);
    let version = e.vcall(buffer, 0, &args![]).u8();
    if version >= 0xc {
        e.call(LOAD_BYTES, &args![buffer, t + 0x5c, 1u32]);
    }
}

// Translated from 009d8ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureMove::InitLoadGame` (Xbox PDB): after the base class's
/// step, turns the loaded target form id into the reference (a lookup by
/// id and a `__RTDynamicCast`; 0 when there is none) and re-queues the
/// actor's current pathing request when a queued request was saved.
pub fn init_load_game(e: &mut Engine, this: Ptr<CombatProcedureMove>, argument: u32) {
    let t = this.addr();
    let attacker = e.call(GET_ATTACKER, &args![t]).u32();
    e.call(BASE_INIT_LOAD_GAME, &args![t, argument]);
    let form_id = e.get(this, CombatProcedureMove::pTargetRef);
    let reference = if form_id.is_null() {
        Ptr::NULL
    } else {
        let form = e.call(LOOKUP_FORM, &args![form_id.addr()]).u32();
        e.call(
            RT_DYNAMIC_CAST,
            &args![form, 0u32, CAST_SOURCE_TYPE, CAST_TARGET_TYPE, 0u32],
        )
        .ptr::<()>()
    };
    e.set(this, CombatProcedureMove::pTargetRef, reference);
    if e.get(this, CombatProcedureMove::bHasQueuedPathingRequest) {
        let request = e.call(GET_CURRENT_PATHING_REQUEST, &args![attacker]).u32();
        e.call(
            SMART_POINTER_SET,
            &args![t + CombatProcedureMove::spQueuedPathingRequest.off, request],
        );
    }
}

/// The locals of `CombatProcedureMove::Update` that its helper steps share.
struct MoveFrame {
    this: Ptr<CombatProcedureMove>,
    /// What the game's `EBP` holds: the locals are below it.
    ebp: u32,
    /// `CombatProcedure::GetAttacker` (the actor that moves).
    attacker: u32,
    /// `MobileObject::GetCurrentProcessType` of the attacker (-1: none).
    process_type: u32,
    /// `CombatProcedure::GetTarget`.
    target: u32,
    /// The controller's `CombatState`.
    combat_state: u32,
}

impl MoveFrame {
    fn at(&self) -> u32 {
        self.this.addr()
    }

    /// The address of the local `below` bytes under `EBP`.
    fn local(&self, below: u32) -> u32 {
        self.ebp - below
    }
}

/// `CombatProcedureMove::Update` state 0 (initializing): the first path.
fn update_initializing(e: &mut Engine, f: &MoveFrame) {
    let t = f.at();
    copy_location(e, t + DESTINATION, t + CURRENT_DESTINATION);
    e.set(f.this, CombatProcedureMove::eState, 1);
    if create_path(e, f.this) && e.get(f.this, CombatProcedureMove::iLocation) == 0xf {
        let delay = setting_float(e, SETTING_011CE9C0);
        start_timer(e, t + CHECK_DESTINATION_TIMER, delay);
    }
}

/// The first block of states 1 and 2: a queued pathing request that the
/// actor is no longer waiting on is turned into the current destination,
/// and in any case the queue is emptied. Returns `true` when `Update` has
/// to stop here (the path was re-created).
fn update_queued_request(e: &mut Engine, f: &MoveFrame) -> bool {
    let t = f.at();
    let queued_address = t + CombatProcedureMove::spQueuedPathingRequest.off;
    if e.call(SMART_POINTER_GET, &args![queued_address]).u32() == 0
        || e.call(IS_WAITING_ON_PATH, &args![f.attacker]).bool()
    {
        return false;
    }
    let request = e.call(SMART_POINTER_GET, &args![queued_address]).u32();
    if e.vcall(request, SLOT_TYPE, &args![]).u32() == 8
        && e.call(REQUEST_BYTE_E1, &args![request]).bool()
    {
        let word = e.call(REQUEST_WORD_108, &args![request]).u32();
        fn_009d7c00(e, f.combat_state, word);
        let stored = e.call(REQUEST_GET_104, &args![request]).u32();
        e.call(COMBAT_STATE_SET_COVER, &args![f.combat_state, stored]);
        e.call(REQUEST_SET_104, &args![request, 0u32]);
        if stored == 0 {
            copy_location(e, t + DESTINATION, t + CURRENT_DESTINATION);
        } else {
            let space = e.call(GET_SPACE, &args![f.attacker]).u32();
            let point = e.call(REQUEST_POINT, &args![request]).u32();
            e.call(SET_LOCATION, &args![t + CURRENT_DESTINATION, point, space]);
            if fn_009d7c40(e, f.combat_state) as u8 == 0 {
                e.call(SMART_POINTER_SET, &args![queued_address, 0u32]);
                copy_location(e, t + DESTINATION, t + CURRENT_DESTINATION);
                e.set(f.this, CombatProcedureMove::eState, 1);
                create_path(e, f.this);
                return true;
            }
            e.set(f.this, CombatProcedureMove::eState, 2);
            e.set(
                f.this,
                CombatProcedureMove::bReservedSecondaryLocation,
                true,
            );
            let spot = f.local(0x38);
            e.call(IDENTITY, &args![spot]);
            let cover = e.call(COMBAT_STATE_COVER, &args![f.combat_state]).u32();
            let controller = e.get(f.this, CombatProcedureMove::pCombatController);
            e.call(
                CALCULATE_COVER_POSITIONS,
                &args![controller.addr(), cover, spot, t + LOOK_AT_POSITION],
            );
            add_firing_z_offset(e, f);
            let delay = e.global::<f32>(HALF);
            start_timer(e, t + CHECK_COVER_TIMER, delay);
        }
    }
    e.call(SMART_POINTER_SET, &args![queued_address, 0u32]);
    false
}

/// Adds the target firing z offset (`GetTargetFiringZOffset(actor, 2, 0)`,
/// a `float` in `ST0`) to the look-at position's `z`.
fn add_firing_z_offset(e: &mut Engine, f: &MoveFrame) {
    let t = f.at();
    let offset = e
        .call(GET_TARGET_FIRING_Z_OFFSET, &args![f.attacker, 2u32, 0u32])
        .f64();
    let z = e.mem.f32(t + LOOK_AT_Z) as f64;
    e.mem.set_f32(t + LOOK_AT_Z, (offset + z) as f32);
}

/// States 1 and 2 when the actor has a path (`008b3b90`) that is no longer
/// valid or has failed: records the unreachable locations and fails the
/// procedure with the matching message.
fn update_path_failed(e: &mut Engine, f: &MoveFrame) {
    let t = f.at();
    if !e.call(IS_PATH_VALID, &args![f.attacker]).bool() {
        if e.get(f.this, CombatProcedureMove::eState) == 2 {
            let cover = e.call(COMBAT_STATE_COVER, &args![f.combat_state]).u32();
            if cover != 0 {
                let id = e.mem.u32(cover + 0x68);
                e.call(ADD_UNREACHABLE_COVER_LOCATION, &args![f.combat_state, id]);
                e.call(CLEAR_SECONDARY_COVER_LOCATION, &args![f.combat_state, 0u32]);
            }
        } else {
            let kind = e.get(f.this, CombatProcedureMove::iLocation);
            // The game's switch: kinds 0, 1, 7 and 8 OR the bit of the kind into
            // the state's flag word, kind 3 sets the byte at +0x188.
            match kind {
                0 | 1 | 7 | 8 => {
                    let mask = 1u32 << (kind as u32 & 31);
                    e.call(STATE_OR_FLAGS, &args![f.combat_state, mask]);
                }
                3 => {
                    e.call(STATE_SET_BYTE_188, &args![f.combat_state]);
                }
                _ => {}
            }
            let last = e
                .call(COMBAT_STATE_LAST_COVER, &args![f.combat_state])
                .u32();
            if last != 0 && e.mem.i32(last + 4) == kind {
                let id = e.mem.u32(last + 0x68);
                e.call(ADD_UNREACHABLE_COVER_LOCATION, &args![f.combat_state, id]);
                e.call(CLEAR_COVER_LOCATION, &args![f.combat_state, 0u32]);
            }
        }
        e.call(
            ADD_UNREACHABLE_LOCATION,
            &args![f.combat_state, t + CURRENT_DESTINATION],
        );
        e.call(COMBAT_STATE_REFRESH, &args![f.combat_state]);
        if e.get(f.this, CombatProcedureMove::iLocation) == 1
            && e.get(f.this, CombatProcedureMove::eState) != 2
        {
            let first = e.call(IDENTITY, &args![t + DESTINATION]).u32();
            let second = e.call(IDENTITY, &args![t + CURRENT_DESTINATION]).u32();
            if e.call(POINTS_NEAR, &args![second, first]).bool() {
                e.call(
                    ADD_UNREACHABLE_LOCATION,
                    &args![f.combat_state, t + DESTINATION],
                );
            }
        }
    }
    let message = if e.call(IS_PATH_VALID, &args![f.attacker]).bool() {
        MESSAGE_PATHING_FAILED
    } else {
        MESSAGE_PATH_BUILDING_FAILED
    };
    e.call(FAIL_PROCEDURE, &args![t, message]);
}

/// `"Pathing failed while moving to destination"`.
const MESSAGE_PATHING_FAILED: u32 = 0x0109_1c54;
/// `"Path Building failed to destination"`.
const MESSAGE_PATH_BUILDING_FAILED: u32 = 0x0109_1c30;
/// `"Target ref is now disabled or deleted"`.
const MESSAGE_TARGET_GONE: u32 = 0x0109_1c08;

/// States 1 and 2 while the actor is on its way: decides whether the
/// destination has to be moved to the target's new place (returns `true`
/// then): the
/// destination follows the target when the target moved further than a
/// setting, and in the cover state the cover is re-checked when the cover
/// timer has expired.
fn update_follow_target(e: &mut Engine, f: &MoveFrame) -> bool {
    let t = f.at();
    let mut retarget = false;
    let target_ref = e.get(f.this, CombatProcedureMove::pTargetRef);
    if !target_ref.is_null() {
        let mut target_process = 0u32;
        if e.vcall(target_ref.addr(), SLOT_HAS_PROCESS, &args![])
            .bool()
        {
            target_process = e.call(GET_PROCESS_TYPE, &args![target_ref.addr()]).u32();
        }
        let place = f.local(0x50);
        e.call(GET_WORLD_LOCATION, &args![target_ref.addr(), place]);
        let limit = if f.process_type == 0 && target_process == 0 {
            if e.get(f.this, CombatProcedureMove::eState) == 2 {
                squared_setting(e, SETTING_011CF7B8) as f64
            } else {
                squared_setting(e, SETTING_011CE494) as f64
            }
        } else {
            squared_setting(e, SETTING_011CE654) as f64
        };
        let distance = e
            .call(DISTANCE_SQUARED, &args![place, t + DESTINATION])
            .f64();
        // FCOMPP of the limit against the distance: moves when limit < distance.
        if limit < distance {
            copy_location(e, place, t + DESTINATION);
            retarget = true;
        }
    }
    if e.get(f.this, CombatProcedureMove::eState) == 2
        && e.call(TIMER_EXPIRED, &args![t + CHECK_COVER_TIMER]).bool()
    {
        if !retarget
            && f.target != 0
            && e.call(COMBAT_STATE_COVER_CHECK_DUE, &args![f.combat_state])
                .bool()
        {
            let cover = e.call(COMBAT_STATE_COVER, &args![f.combat_state]).u32();
            if cover == 0 {
                retarget = true;
            } else {
                let target_place = f.local(0x70);
                let attacker_place = f.local(0x80);
                e.call(GET_WORLD_LOCATION, &args![f.target, target_place]);
                e.call(GET_WORLD_LOCATION, &args![f.attacker, attacker_place]);
                let distance = e
                    .call(DISTANCE_SQUARED, &args![attacker_place, target_place])
                    .f64();
                if distance < e.global::<f64>(MAX_COVER_DISTANCE_SQUARED) {
                    let position = e.vcall(f.target, SLOT_POSITION, &args![]).u32();
                    let point = e
                        .call(COVER_POINT, &args![cover + 0x14, f.local(0x174)])
                        .u32();
                    if !e
                        .call(
                            LINE_TEST,
                            &args![point, position, cover + 0x3c, cover + 0x48],
                        )
                        .bool()
                    {
                        retarget = true;
                    } else {
                        let location = e.call(IDENTITY, &args![t + CURRENT_DESTINATION]).u32();
                        if !e
                            .call(
                                CHECK_COVER_SAFE,
                                &args![f.combat_state, cover + 0x14, location, f.target, 0u32],
                            )
                            .bool()
                        {
                            retarget = true;
                        }
                    }
                }
            }
        }
        let delay = e.global::<f32>(HALF);
        start_timer(e, t + CHECK_COVER_TIMER, delay);
    }
    retarget
}

/// A setting's `float` multiplied by itself (the code reads the setting
/// twice and multiplies the two reads in extended precision, then stores
/// a `float`).
fn squared_setting(e: &mut Engine, setting: u32) -> f32 {
    let first = setting_float(e, setting);
    let second = setting_float(e, setting);
    (first as f64 * second as f64) as f32
}

/// The cover search of state 1: when the actor has moved on to another path
/// node (or the search timer has expired) it walks the path ahead in steps
/// of up to 1536 units, looking for a cover location; found cover switches
/// the procedure to state 2. Returns `true` when `Update` has to stop (the
/// path to the cover could not be created).
fn update_cover_search(e: &mut Engine, f: &MoveFrame) -> bool {
    let t = f.at();
    let mut range = e.global::<f32>(SEARCH_RANGE);
    let node = e.call(CURRENT_PATH_NODE, &args![f.attacker]).u32() as i32;
    if e.get(f.this, CombatProcedureMove::iCurrentNode) == -1 {
        range = e.global::<f32>(SEARCH_RANGE_FIRST);
    }
    if node == e.get(f.this, CombatProcedureMove::iCurrentNode)
        && !e
            .call(TIMER_EXPIRED, &args![t + COVERED_MOVE_SEARCH_TIMER])
            .bool()
    {
        return false;
    }
    e.set(f.this, CombatProcedureMove::iCurrentNode, node);
    let delay = e.global::<f32>(TWO);
    start_timer(e, t + COVERED_MOVE_SEARCH_TIMER, delay);
    let avoid = f.local(0x90);
    let current = f.local(0xb0);
    let next = f.local(0xa4);
    let step = f.local(0xbc);
    let sum = f.local(0x180);
    let search = f.local(0x114);
    let cover_point = f.local(0x124);
    e.call(ARRAY_POINTER_CONSTRUCT, &args![avoid, 0u32]);
    let mut avoid_created = false;
    let position = e.vcall(f.attacker, SLOT_POSITION, &args![]).u32();
    for i in 0..3 {
        let word = e.mem.u32(position + i * 4);
        e.mem.set_u32(current + i * 4, word);
    }
    e.call(IDENTITY, &args![next]);
    let mut total = 0.0f32;
    let mut done = false;
    let mut node = node;
    loop {
        if done
            || !e
                .call(PATH_NODE_POINT, &args![f.attacker, (node + 1) as u32, next])
                .bool()
        {
            break;
        }
        node += 1;
        e.call(POINT_DIFFERENCE, &args![next, step, current]);
        let length = e.call(POINT_UNITIZE, &args![step]).f32();
        let maximum = e.global::<f64>(MAX_SEARCH_LENGTH);
        // FCOMP of (total + length) against 1536.0: more than the maximum
        // when the flags show neither "below" nor "equal" (nor NaN).
        if total as f64 + length as f64 > maximum {
            let remaining = (maximum - total as f64) as f32;
            e.call(POINT_SCALE, &args![step, remaining]);
            let result = e.call(POINT_SUM, &args![current, sum, step]).u32();
            for i in 0..3 {
                let word = e.mem.u32(result + i * 4);
                e.mem.set_u32(next + i * 4, word);
            }
            done = true;
        }
        total = (total as f64 + length as f64) as f32;
        for i in 0..3 {
            let word = e.mem.u32(next + i * 4);
            e.mem.set_u32(current + i * 4, word);
        }
        if !avoid_created {
            create_avoid_node_array(e, f.this, avoid);
        }
        avoid_created = true;
        e.call(SEARCH_CONSTRUCT, &args![search]);
        let array = e.call(SMART_POINTER_GET, &args![avoid]).u32();
        let kind = e.get(f.this, CombatProcedureMove::iLocation);
        let found = e
            .call(
                FIND_COVER_LOCATION,
                &args![f.combat_state, search, next, kind as u32, range, array],
            )
            .bool();
        if found {
            e.set(
                f.this,
                CombatProcedureMove::bReservedSecondaryLocation,
                true,
            );
            e.call(RESERVE_COVER_LOCATION, &args![f.combat_state]);
            e.call(IDENTITY, &args![cover_point]);
            let cover = e.call(COMBAT_STATE_COVER, &args![f.combat_state]).u32();
            let controller = e.get(f.this, CombatProcedureMove::pCombatController);
            e.call(
                CALCULATE_COVER_POSITIONS,
                &args![controller.addr(), cover, cover_point, t + LOOK_AT_POSITION],
            );
            add_firing_z_offset(e, f);
            e.set(f.this, CombatProcedureMove::eState, 2);
            let space = e.call(GET_SPACE, &args![f.attacker]).u32();
            e.call(
                SET_LOCATION,
                &args![t + CURRENT_DESTINATION, cover_point, space],
            );
            if !create_path(e, f.this) {
                e.call(SEARCH_DESTRUCT, &args![search]);
                e.call(ARRAY_POINTER_DESTRUCT, &args![avoid]);
                return true;
            }
            let delay = e.global::<f32>(HALF);
            start_timer(e, t + CHECK_COVER_TIMER, delay);
            e.call(SEARCH_DESTRUCT, &args![search]);
            break;
        }
        e.call(SEARCH_DESTRUCT, &args![search]);
    }
    e.call(ARRAY_POINTER_DESTRUCT, &args![avoid]);
    false
}

/// The end of states 1 and 2: the line-of-sight test of kind 0xf, stopping,
/// and the arrival handling (to state 3 from the cover state, else the
/// reached report and completion).
fn update_arrival(e: &mut Engine, f: &MoveFrame) {
    let t = f.at();
    let mut in_sight = false;
    if e.get(f.this, CombatProcedureMove::iLocation) == 0xf
        && e.call(TIMER_EXPIRED, &args![t + CHECK_DESTINATION_TIMER])
            .bool()
    {
        let place = f.local(0x134);
        e.call(GET_WORLD_LOCATION, &args![f.attacker, place]);
        let distance = e
            .call(DISTANCE_SQUARED, &args![place, t + DESTINATION])
            .f64();
        let first = setting_float(e, SETTING_011CF2A0);
        let second = setting_float(e, SETTING_011CF2A0);
        // FCOMP of the product against the distance: inside when the flags
        // show neither "below" nor "equal" (nor NaN).
        if (first as f64 * second as f64) > distance {
            let pathing = f.local(0x15c);
            e.call(
                PATHING_LOCATION_NEW_FROM_POINT,
                &args![pathing, t + DESTINATION],
            );
            let location = e.call(IDENTITY, &args![place]).u32();
            if e.call(CHECK_LINE_OF_SIGHT, &args![pathing, location, 0u32, 0u32])
                .bool()
            {
                in_sight = true;
            }
            e.call(PATHING_LOCATION_DESTRUCT, &args![pathing]);
        }
        let delay = setting_float(e, SETTING_011CFAD4);
        start_timer(e, t + CHECK_DESTINATION_TIMER, delay);
    }
    if in_sight {
        e.call(STOP_MOVING, &args![f.attacker]);
    }
    if in_sight || e.call(IS_PATHING_COMPLETE, &args![f.attacker]).bool() {
        if e.get(f.this, CombatProcedureMove::eState) == 2 {
            e.set(f.this, CombatProcedureMove::eState, 3);
            let delay = fn_009d8440(e, f.this);
            start_timer(e, t + HIDING_TIMER, delay);
            let delay = e.global::<f32>(QUARTER);
            start_timer(e, t + CHECK_COVER_TIMER, delay);
            e.call(SET_LOOK_AT_TARGET, &args![t, t + LOOK_AT_POSITION, 3u32]);
            e.call(SET_MOVE_MODE, &args![t, 2u32]);
        } else {
            let kind = e.get(f.this, CombatProcedureMove::iLocation);
            e.call(
                ARRIVAL_STEP,
                &args![f.combat_state, t + DESTINATION, kind as u32],
            );
            e.call(COMPLETE_PROCEDURE, &args![t]);
        }
    }
}

/// States 1 and 2.
fn update_moving(e: &mut Engine, f: &MoveFrame) {
    let t = f.at();
    if update_queued_request(e, f) {
        return;
    }
    if e.call(PATH_FAILED_TEST, &args![f.attacker]).bool() {
        update_path_failed(e, f);
        return;
    }
    let target_ref = e.get(f.this, CombatProcedureMove::pTargetRef);
    if !target_ref.is_null()
        && (e.call(REF_DISABLED, &args![target_ref.addr()]).bool()
            || e.call(REF_DELETED, &args![target_ref.addr()]).bool())
    {
        e.call(FAIL_PROCEDURE, &args![t, MESSAGE_TARGET_GONE]);
        return;
    }
    if update_follow_target(e, f) {
        if e.get(f.this, CombatProcedureMove::eState) == 2 {
            e.call(UNRESERVE_COVER_LOCATION, &args![f.combat_state]);
            e.call(CLEAR_SECONDARY_COVER_LOCATION, &args![f.combat_state, 0u32]);
            e.set(
                f.this,
                CombatProcedureMove::bReservedSecondaryLocation,
                false,
            );
        }
        copy_location(e, t + DESTINATION, t + CURRENT_DESTINATION);
        e.set(f.this, CombatProcedureMove::eState, 1);
        create_path(e, f.this);
        return;
    }
    let kind = e.get(f.this, CombatProcedureMove::iLocation);
    if e.mem.u8(COVER_MODE_BYTE) == 0
        && e.get(f.this, CombatProcedureMove::eState) == 1
        && !e.call(IS_WAITING_ON_PATH, &args![f.attacker]).bool()
        && f.process_type == 0
        && kind_flag(e, KIND_TABLE, kind) != 0
        && fn_009d7bd0(e, f.combat_state, 1)
        && update_cover_search(e, f)
    {
        return;
    }
    update_arrival(e, f);
}

/// State 3 (hiding behind the cover): leaves the state when the hiding
/// timer expired, the cover is gone, the target changed its mind
/// (the combat state's times) or the cover is no longer safe.
fn update_hiding(e: &mut Engine, f: &MoveFrame) {
    let t = f.at();
    let mut leave = false;
    let cover = e.call(COMBAT_STATE_COVER, &args![f.combat_state]).u32();
    if e.call(TIMER_EXPIRED, &args![t + HIDING_TIMER]).bool() || cover == 0 {
        leave = true;
    } else if f.target != 0 {
        let started = e.call(FLOAT_AT, &args![t + HIDING_TIMER]).f64();
        let limit = (started + e.global::<f64>(ONE_DOUBLE)) as f32;
        let now = e.call(GAME_TIME_NOW, &args![]).f64();
        // FCOMPP of the limit against now: acts when limit < now.
        if (limit as f64) < now {
            if e.call(COMBAT_STATE_FLAG, &args![f.combat_state]).bool() {
                leave = true;
            } else {
                let copy = f.local(0x184);
                let stamp = e
                    .call(COMBAT_STATE_TIME_COPY, &args![f.combat_state, copy])
                    .u32();
                let other = e.call(FLOAT_AT, &args![stamp]).f64();
                if (limit as f64) < other {
                    leave = true;
                }
            }
        }
        if !leave
            && e.call(COMBAT_STATE_COVER_CHECK_DUE, &args![f.combat_state])
                .bool()
            && e.call(TIMER_EXPIRED, &args![t + CHECK_COVER_TIMER]).bool()
        {
            let location = e.call(IDENTITY, &args![t + CURRENT_DESTINATION]).u32();
            if !e
                .call(
                    CHECK_COVER_SAFE,
                    &args![f.combat_state, cover + 0x14, location, f.target, 1u32],
                )
                .bool()
            {
                leave = true;
            }
            let delay = e.global::<f32>(QUARTER);
            start_timer(e, t + CHECK_COVER_TIMER, delay);
        }
    }
    if leave {
        e.set(f.this, CombatProcedureMove::eState, 0);
        e.call(UNRESERVE_COVER_LOCATION, &args![f.combat_state]);
        e.set(
            f.this,
            CombatProcedureMove::bReservedSecondaryLocation,
            false,
        );
        e.call(CLEAR_SECONDARY_COVER_LOCATION, &args![f.combat_state, 0u32]);
        e.call(CLEAR_LOOK_AT_TARGET, &args![t]);
    }
}

// Translated from 009d6ca0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009d6ca0 - the compiler's FS:[0] exception frame (unwinding of the avoid-node smart pointer, the search object and the pathing location) is not translated
/// `CombatProcedureMove::Update` (Xbox PDB): one step of the procedure by
/// its state (`eState`): 0 creates the first path; 1 and 2 move to the
/// destination (re-pathing when the target moved, searching for cover on
/// the way in state 1, failing when the path breaks, and on arrival turning
/// to hiding in state 3 or completing); 3 hides until the timers or the
/// cover say to stop.
///
/// The frame offsets used by the helper steps are those of the
/// disassembly (below `EBP`).
pub fn update(e: &mut Engine, this: Ptr<CombatProcedureMove>) {
    let attacker = e.call(GET_ATTACKER, &args![this.addr()]).u32();
    let process_type = e.call(GET_PROCESS_TYPE, &args![attacker]).u32();
    let target = e.call(GET_TARGET, &args![this.addr()]).u32();
    let controller = e.get(this, CombatProcedureMove::pCombatController);
    let combat_state = e.call(GET_COMBAT_STATE, &args![controller.addr()]).u32();
    let state = e.get(this, CombatProcedureMove::eState);
    e.with_stack(UPDATE_FRAME, |e, base| {
        let frame = MoveFrame {
            this,
            ebp: base.addr() + UPDATE_FRAME,
            attacker,
            process_type,
            target,
            combat_state,
        };
        match state {
            0 => update_initializing(e, &frame),
            1 | 2 => update_moving(e, &frame),
            3 => update_hiding(e, &frame),
            _ => {}
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009d6b60, get_type(Ptr<CombatProcedureMove>) -> u32),
        entry!(
            0x009d6b70,
            scalar_deleting_destructor(Ptr<CombatProcedureMove>, u32) -> Ptr<CombatProcedureMove>
        ),
        entry!(0x009d6ba0, fn_009d6ba0(Ptr<CombatProcedureMove>)),
        entry!(0x009d6c00, shutdown(Ptr<CombatProcedureMove>)),
        entry!(0x009d6ca0, update(Ptr<CombatProcedureMove>)),
        entry!(0x009d7bd0, fn_009d7bd0(u32, u32) -> bool),
        entry!(0x009d7c00, fn_009d7c00(u32, u32)),
        entry!(0x009d7c40, fn_009d7c40(u32) -> u32),
        entry!(0x009d7c80, create_path(Ptr<CombatProcedureMove>) -> bool),
        entry!(
            0x009d7d00,
            create_path_ov2(Ptr<CombatProcedureMove>, u32) -> bool
        ),
        entry!(
            0x009d82e0,
            create_avoid_node_array(Ptr<CombatProcedureMove>, u32)
        ),
        entry!(0x009d8410, fn_009d8410(u32, f32)),
        entry!(0x009d8440, fn_009d8440(Ptr<CombatProcedureMove>) -> f32),
        entry!(
            0x009d8480,
            clear_reference_pointers(Ptr<CombatProcedureMove>, Ptr)
        ),
        entry!(0x009d84b0, started_planning(Ptr<CombatProcedureMove>)),
        entry!(0x009d84e0, finished_planning(Ptr<CombatProcedureMove>)),
        entry!(
            0x009d8520,
            display_debug_text(Ptr<CombatProcedureMove>, u32, u32, u32)
        ),
        entry!(0x009d8880, save_game(Ptr<CombatProcedureMove>, u32)),
        entry!(0x009d89a0, load_game(Ptr<CombatProcedureMove>, u32)),
        entry!(0x009d8ad0, init_load_game(Ptr<CombatProcedureMove>, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::{HashMap, VecDeque};
    use std::rc::Rc;

    type Log = Vec<(u32, Vec<u32>)>;

    /// What the doubles answer: a constant per address, or a queue that is
    /// used up first.
    #[derive(Default)]
    struct Script {
        returns: HashMap<u32, Ret>,
        queues: HashMap<u32, VecDeque<Ret>>,
        /// The `float` a setting object reads as (default 0.0).
        settings: HashMap<u32, f32>,
        /// What `TESObjectREFR::GetWorldLocation` writes.
        place: [f32; 3],
    }

    type Shared = Rc<RefCell<Script>>;

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

    /// The slot doubles of the shared fake vtable live at `FAKE + slot`.
    const FAKE: u32 = 0x7000_0000;
    const FAKE_SLOTS: [u32; 6] = [
        0x00,
        SLOT_TYPE,
        SLOT_PRINT,
        SLOT_HAS_PROCESS,
        SLOT_NAME,
        SLOT_POSITION,
    ];

    /// Every function outside the file that the translations call by
    /// address (the others are direct calls).
    const CALLEES: &[u32] = &[
        OPERATOR_DELETE,
        SMART_POINTER_DESTRUCT,
        BASE_DESTRUCT,
        GET_ATTACKER,
        GET_TARGET,
        SET_MOVE_MODE,
        CLEAR_MOVE_MODE,
        SET_LOOK_AT_TARGET,
        CLEAR_LOOK_AT_TARGET,
        BASE_SAVE_GAME,
        BASE_LOAD_GAME,
        GET_COMBAT_STATE,
        SMART_POINTER_GET,
        SMART_POINTER_SET,
        CONTROLLER_ACTOR,
        GET_PROCESS_TYPE,
        GET_SPACE,
        DISTANCE_SQUARED,
        WORLD_LOCATION_SAVE,
        WORLD_LOCATION_LOAD,
        TIMER_SAVE,
        TIMER_LOAD,
        SAVE_BYTES,
        LOAD_BYTES,
        SAVE_FORM_ID,
        LOAD_FORM_ID,
        BASE_INIT_LOAD_GAME,
        LOOKUP_FORM,
        RT_DYNAMIC_CAST,
        GET_CURRENT_PATHING_REQUEST,
        STOP_MOVING,
        SHUTDOWN_TEST,
        UNRESERVE_COVER_LOCATION,
        RESERVE_COVER_LOCATION,
        STATE_COPY_RECORD,
        STATE_CHECK_LOCATION,
        CLEAR_COVER_LOCATION,
        CLEAR_SECONDARY_COVER_LOCATION,
        ADD_UNREACHABLE_COVER_LOCATION,
        ADD_UNREACHABLE_LOCATION,
        COMBAT_STATE_SET_COVER,
        COMBAT_STATE_COVER,
        COMBAT_STATE_LAST_COVER,
        COMBAT_STATE_COVER_CHECK_DUE,
        COMBAT_STATE_FLAG,
        COMBAT_STATE_TIME_COPY,
        COMBAT_STATE_REFRESH,
        CHECK_COVER_SAFE,
        ARRIVAL_STEP,
        FIND_COVER_LOCATION,
        BUILD_COVERED_MOVE_REQUEST,
        GET_LOCATION_TARGET_DISTANCE,
        CONTAINS_ACTION,
        STATE_OR_FLAGS,
        STATE_SET_BYTE_188,
        FAIL_PROCEDURE,
        COMPLETE_PROCEDURE,
        CALCULATE_COVER_POSITIONS,
        GET_TARGET_FIRING_Z_OFFSET,
        COMPUTE_FACE_TARGET_LOCATION,
        CALCULATE_LOS,
        SET_LOCATION,
        SET_LOCATION_FROM_PATHING,
        BUILD_AVOID_NODES,
        BUILD_AVOID_NODES_WITH_TARGET,
        AVOID_NODE_SOURCE,
        GET_THREAT_MAP,
        THREAT_MAP_NODE,
        ARRAY_POINTER_SET,
        ARRAY_POINTER_CONSTRUCT,
        ARRAY_POINTER_DESTRUCT,
        ALLOCATE,
        ARRAY_CONSTRUCT,
        AVOID_ARRAY_ADD,
        AVOID_NODE_CONSTRUCT,
        PATHING_REQUEST_CONSTRUCT,
        PATHING_REQUEST_DESTRUCT,
        COVERED_MOVE_REQUEST_CONSTRUCT,
        COVERED_MOVE_REQUEST_DESTRUCT,
        REQUEST_SET_GOAL,
        REQUEST_SET_FLAG,
        COVERED_REQUEST_SET_KIND,
        BUILD_REQUEST,
        SET_PATHFINDING_GOAL,
        IS_WAITING_ON_PATH,
        IS_PATH_VALID,
        IS_PATHING_COMPLETE,
        PATH_FAILED_TEST,
        CURRENT_PATH_NODE,
        PATH_NODE_POINT,
        TIMER_START,
        TIMER_EXPIRED,
        TIMER_RUNNING,
        TIMER_VALUE_A,
        TIMER_VALUE_B,
        FLOAT_AT,
        GAME_TIME_NOW,
        TIME_STAMP_SET,
        HAS_NAME,
        FALLBACK_NAME,
        FORM_ID,
        REF_DISABLED,
        REF_DELETED,
        REQUEST_POINT,
        REQUEST_WORD_108,
        REQUEST_GET_104,
        REQUEST_SET_104,
        REQUEST_BYTE_E1,
        LINE_TEST,
        COVER_POINT,
        PATHING_LOCATION_NEW_FROM_LOCATION,
        PATHING_LOCATION_NEW_FROM_POINT,
        PATHING_LOCATION_IS_VALID,
        PATHING_LOCATION_DESTRUCT,
        CHECK_LINE_OF_SIGHT,
        POINT_DIFFERENCE,
        POINT_UNITIZE,
        POINT_SCALE,
        POINT_SUM,
        POINT_LENGTH_SQUARED,
        POINTS_NEAR,
        SEARCH_CONSTRUCT,
        SEARCH_DESTRUCT,
        COVER_RECORD,
        COVER_RECORD_TEST,
        GET_RADIUS,
    ];

    /// The vtable call of slot `0x1f4` appears in the log as a call to the
    /// fake slot address.
    const POSITION_CALL: u32 = FAKE + SLOT_POSITION;
    const TYPE_CALL: u32 = FAKE + SLOT_TYPE;
    const NAME_CALL: u32 = FAKE + SLOT_NAME;
    const PRINT_CALL: u32 = FAKE + SLOT_PRINT;
    const PROCESS_CALL: u32 = FAKE + SLOT_HAS_PROCESS;
    const VERSION_CALL: u32 = FAKE;

    /// The engine with the exe constants the code reads, doubles for every
    /// callee, and one procedure.
    struct Rig {
        e: Engine,
        script: Shared,
        this: Ptr<CombatProcedureMove>,
        controller: u32,
        state: u32,
        actor: u32,
        vtable: u32,
        position: u32,
    }

    impl Rig {
        fn new() -> Rig {
            let mut e = Engine::new();
            for page in [
                0x0101_2000u32,
                0x0101_6000,
                0x0102_3000,
                0x0103_2000,
                0x0103_a000,
                0x0109_1000,
                0x011a_5000,
                0x011f_1000,
                0x011f_4000,
            ] {
                e.map(page, 0x1000);
            }
            e.mem.set_f32(MINUS_ONE, -1.0);
            e.mem.set_f64(ONE_DOUBLE, 1.0);
            e.mem.set_f32(HALF, 0.5);
            e.mem.set_f32(TWO, 2.0);
            e.mem.set_f32(QUARTER, 0.25);
            e.mem.set_f32(SEARCH_RANGE, 256.0);
            e.mem.set_f32(SEARCH_RANGE_FIRST, 512.0);
            e.mem.set_f32(FAR_RADIUS, 640.0);
            e.mem.set_f64(MAX_SEARCH_LENGTH, 1536.0);
            e.mem.set_f64(MAX_COVER_DISTANCE_SQUARED, 16777216.0);
            for (i, v) in [1.0f32, 2.0, 3.0].into_iter().enumerate() {
                e.mem.set_f32(DEFAULT_GOAL + 4 * i as u32, v);
            }
            let script: Shared = Rc::new(RefCell::new(Script::default()));
            script.borrow_mut().place = [10.0, 20.0, 30.0];
            install(&mut e, &script, CALLEES);
            let fakes: Vec<u32> = FAKE_SLOTS.iter().map(|s| FAKE + s).collect();
            install(&mut e, &script, &fakes);
            e.register(IDENTITY, |_, a| ret(a[0]));
            let settings = script.clone();
            e.register_double(SETTING_VALUE, move |e, a| {
                let value = settings
                    .borrow()
                    .settings
                    .get(&a[0])
                    .copied()
                    .unwrap_or(0.0);
                let slot = e.mem.alloc(4);
                e.mem.set_f32(slot, value);
                ret(slot)
            });
            let places = script.clone();
            e.register_double(GET_WORLD_LOCATION, move |e, a| {
                let place = places.borrow().place;
                for (i, v) in place.into_iter().enumerate() {
                    e.mem.set_f32(a[1] + 4 * i as u32, v);
                }
                e.mem.set_u32(a[1] + 12, 0x1234);
                Ret::default()
            });
            let vtable = e.mem.alloc(0x200);
            for slot in FAKE_SLOTS {
                e.mem.set_u32(vtable + slot, FAKE + slot);
            }
            let position = e.mem.alloc(12);
            let actor = e.mem.alloc(0x20);
            e.mem.set_u32(actor, vtable);
            let controller = e.mem.alloc(0x100);
            let state = e.mem.alloc(0x200);
            e.mem.set_u32(COMBAT_MANAGER, 0x5555);
            let this = e.new_object::<CombatProcedureMove>();
            e.set(
                this,
                CombatProcedureMove::pCombatController,
                Ptr::new(controller),
            );
            {
                let mut s = script.borrow_mut();
                s.returns.insert(GET_COMBAT_STATE, ret(state));
                s.returns.insert(GET_ATTACKER, ret(actor));
                s.returns.insert(CONTROLLER_ACTOR, ret(actor));
                s.returns.insert(POSITION_CALL, ret(position));
            }
            Rig {
                e,
                script,
                this,
                controller,
                state,
                actor,
                vtable,
                position,
            }
        }

        /// A fake game object that shares the vtable of the doubles.
        fn object(&mut self) -> u32 {
            let object = self.e.mem.alloc(0x120);
            self.e.mem.set_u32(object, self.vtable);
            object
        }

        fn answer(&self, addr: u32, value: u32) {
            self.script.borrow_mut().returns.insert(addr, ret(value));
        }

        fn answer_float(&self, addr: u32, value: f64) {
            self.script.borrow_mut().returns.insert(addr, float(value));
        }

        fn queue(&self, addr: u32, values: &[u32]) {
            self.script
                .borrow_mut()
                .queues
                .insert(addr, values.iter().map(|v| ret(*v)).collect());
        }

        fn queue_float(&self, addr: u32, values: &[f64]) {
            self.script
                .borrow_mut()
                .queues
                .insert(addr, values.iter().map(|v| float(*v)).collect());
        }

        fn setting(&self, setting: u32, value: f32) {
            self.script.borrow_mut().settings.insert(setting, value);
        }

        fn set_state(&mut self, state: u32) {
            self.e.set(self.this, CombatProcedureMove::eState, state);
        }

        fn set_kind(&mut self, kind: i32) {
            self.e.set(self.this, CombatProcedureMove::iLocation, kind);
        }

        /// Sets the table bytes of a kind (`covered`, `avoid`).
        fn kind_table(&mut self, kind: u32, covered: u8, avoid: u8) {
            self.e.mem.set_u8(KIND_TABLE + kind * 8, covered);
            self.e.mem.set_u8(KIND_TABLE_AVOID + kind * 8, avoid);
        }

        /// Calls `addr` with `args` through the exe address and returns the
        /// result and the calls the translation made (without the
        /// top-level call).
        fn run(&mut self, addr: u32, args: &[u32]) -> (Ret, Log) {
            self.e.call_log = Some(vec![]);
            let result = self.e.call(addr, args);
            let mut calls = self.e.call_log.take().unwrap();
            calls.remove(0);
            (result, calls)
        }

        fn t(&self) -> u32 {
            self.this.addr()
        }

        fn update(&mut self) -> Log {
            let t = self.t();
            self.run(0x009d_6ca0, &args![t]).1
        }
    }

    /// Doubles over `addrs` that answer from the script.
    fn install(e: &mut Engine, script: &Shared, addrs: &[u32]) {
        for &addr in addrs {
            let script = script.clone();
            e.register_double(addr, move |_, _| {
                let mut s = script.borrow_mut();
                if let Some(next) = s.queues.get_mut(&addr).and_then(VecDeque::pop_front) {
                    return next;
                }
                s.returns.get(&addr).copied().unwrap_or_default()
            });
        }
    }

    fn addresses(calls: &Log) -> Vec<u32> {
        calls.iter().map(|c| c.0).collect()
    }

    fn calls_to(calls: &Log, addr: u32) -> Vec<Vec<u32>> {
        calls
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn first_call(calls: &Log, addr: u32) -> Vec<u32> {
        calls_to(calls, addr)
            .into_iter()
            .next()
            .unwrap_or_else(|| panic!("no call to {addr:08x}"))
    }

    fn called(calls: &Log, addr: u32) -> bool {
        calls.iter().any(|c| c.0 == addr)
    }

    fn bits(value: f32) -> u32 {
        value.to_bits()
    }

    // ---- the small functions ----

    #[test]
    fn get_type_is_six() {
        let mut rig = Rig::new();
        let t = rig.t();
        assert_eq!(rig.run(0x009d_6b60, &args![t]).0.u32(), 6);
    }

    #[test]
    fn deleting_destructor_runs_the_body_and_deletes_on_bit_zero() {
        for flags in [0u32, 1, 2, 3] {
            let mut rig = Rig::new();
            let t = rig.t();
            let (result, calls) = rig.run(0x009d_6b70, &args![t, flags]);
            assert_eq!(result.u32(), t);
            let mut expected = vec![
                (SMART_POINTER_DESTRUCT, vec![t + 0x58]),
                (BASE_DESTRUCT, vec![t]),
            ];
            if flags & 1 != 0 {
                expected.push((OPERATOR_DELETE, vec![t]));
            }
            assert_eq!(calls, expected);
            assert_eq!(rig.e.mem.u32(t), VTABLE);
        }
    }

    #[test]
    fn destructor_body_stores_the_vtable_and_destroys_the_queued_request() {
        let mut rig = Rig::new();
        let t = rig.t();
        let (_, calls) = rig.run(0x009d_6ba0, &args![t]);
        assert_eq!(
            calls,
            vec![
                (SMART_POINTER_DESTRUCT, vec![t + 0x58]),
                (BASE_DESTRUCT, vec![t]),
            ]
        );
        assert_eq!(rig.e.mem.u32(t), VTABLE);
    }

    #[test]
    fn shutdown_stops_the_actor_while_moving() {
        for state in [1u32, 2] {
            let mut rig = Rig::new();
            let t = rig.t();
            rig.set_state(state);
            let (_, calls) = rig.run(0x009d_6c00, &args![t]);
            assert_eq!(
                calls,
                vec![
                    (CONTROLLER_ACTOR, vec![rig.controller]),
                    (SHUTDOWN_TEST, vec![t]),
                    (STOP_MOVING, vec![rig.actor]),
                    (CLEAR_MOVE_MODE, vec![t]),
                ]
            );
            // When the test says yes, the actor is left alone.
            let mut rig = Rig::new();
            rig.set_state(state);
            rig.answer(SHUTDOWN_TEST, 1);
            let (_, calls) = rig.run(0x009d_6c00, &args![t]);
            assert!(!called(&calls, STOP_MOVING));
        }
    }

    #[test]
    fn shutdown_clears_the_look_at_target_while_hiding_and_releases_the_cover() {
        let mut rig = Rig::new();
        let t = rig.t();
        rig.set_state(3);
        rig.e.set(
            rig.this,
            CombatProcedureMove::bReservedSecondaryLocation,
            true,
        );
        let (_, calls) = rig.run(0x009d_6c00, &args![t]);
        assert_eq!(
            calls,
            vec![
                (CONTROLLER_ACTOR, vec![rig.controller]),
                (CLEAR_LOOK_AT_TARGET, vec![t]),
                (CLEAR_MOVE_MODE, vec![t]),
                (GET_COMBAT_STATE, vec![rig.controller]),
                (UNRESERVE_COVER_LOCATION, vec![rig.state]),
            ]
        );
        // State 0: neither.
        let mut rig = Rig::new();
        let (_, calls) = rig.run(0x009d_6c00, &args![t]);
        assert_eq!(addresses(&calls), vec![CONTROLLER_ACTOR, CLEAR_MOVE_MODE]);
    }

    #[test]
    fn bit_test_uses_the_shift_count_modulo_32() {
        let mut rig = Rig::new();
        let state = rig.state;
        rig.e.mem.set_u32(state + 0x90, 0b10);
        assert!(rig.run(0x009d_7bd0, &args![state, 1u32]).0.bool());
        assert!(!rig.run(0x009d_7bd0, &args![state, 0u32]).0.bool());
        // 33 shifts like 1.
        assert!(rig.run(0x009d_7bd0, &args![state, 33u32]).0.bool());
    }

    #[test]
    fn state_helpers_pass_their_members_on() {
        let mut rig = Rig::new();
        let state = rig.state;
        let (_, calls) = rig.run(0x009d_7c00, &args![state, 0x4321u32]);
        assert_eq!(
            calls,
            vec![(STATE_COPY_RECORD, vec![state + 0x128, 0x4321])]
        );
        rig.e.mem.set_u32(state + 0xe8, 0x77);
        rig.answer(STATE_CHECK_LOCATION, 0x1234_5601);
        let (result, calls) = rig.run(0x009d_7c40, &args![state]);
        assert_eq!(calls, vec![(STATE_CHECK_LOCATION, vec![state, 0x77])]);
        assert_eq!(result.u32(), 0x1234_5601);
    }

    #[test]
    fn timestamp_setter_stores_the_stamp_at_c0() {
        let mut rig = Rig::new();
        let state = rig.state;
        rig.e.register(TIME_STAMP_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        rig.run(0x009d_8410, &args![state, 1.5f32]);
        assert_eq!(rig.e.mem.f32(state + 0xc0), 1.5);
    }

    #[test]
    fn hiding_delay_depends_on_the_kind() {
        let mut rig = Rig::new();
        let t = rig.t();
        for (kind, expected) in [
            (0, 1.0),
            (5, 1.0),
            (6, 2.0),
            (7, 2.0),
            (8, 0.5),
            (9, 1.0),
            (-1, 1.0),
        ] {
            rig.set_kind(kind);
            assert_eq!(rig.run(0x009d_8440, &args![t]).0.f32(), expected, "{kind}");
        }
    }

    #[test]
    fn clearing_the_target_reference_fails_the_procedure() {
        let mut rig = Rig::new();
        let t = rig.t();
        rig.e
            .set(rig.this, CombatProcedureMove::pTargetRef, Ptr::new(0x900));
        let (_, calls) = rig.run(0x009d_8480, &args![t, 0x800u32]);
        assert!(calls.is_empty());
        assert_eq!(
            rig.e.get(rig.this, CombatProcedureMove::pTargetRef).addr(),
            0x900
        );
        let (_, calls) = rig.run(0x009d_8480, &args![t, 0x900u32]);
        assert_eq!(
            calls,
            vec![(FAIL_PROCEDURE, vec![t, TARGET_CLEARED_MESSAGE])]
        );
        assert_eq!(
            rig.e.get(rig.this, CombatProcedureMove::pTargetRef).addr(),
            0
        );
    }

    #[test]
    fn planning_releases_and_reserves_the_cover_location() {
        let mut rig = Rig::new();
        let t = rig.t();
        // Nothing reserved: nothing to release.
        assert!(rig.run(0x009d_84b0, &args![t]).1.is_empty());
        rig.e.set(
            rig.this,
            CombatProcedureMove::bReservedSecondaryLocation,
            true,
        );
        let (_, calls) = rig.run(0x009d_84b0, &args![t]);
        assert_eq!(
            calls,
            vec![
                (GET_COMBAT_STATE, vec![rig.controller]),
                (UNRESERVE_COVER_LOCATION, vec![rig.state]),
            ]
        );
        assert!(!rig
            .e
            .get(rig.this, CombatProcedureMove::bReservedSecondaryLocation));
        // Finished planning reserves in states 2 and 3 only.
        for (state, reserves) in [(0u32, false), (1, false), (2, true), (3, true)] {
            rig.set_state(state);
            rig.e.set(
                rig.this,
                CombatProcedureMove::bReservedSecondaryLocation,
                false,
            );
            let (_, calls) = rig.run(0x009d_84e0, &args![t]);
            assert_eq!(called(&calls, RESERVE_COVER_LOCATION), reserves);
            assert_eq!(
                rig.e
                    .get(rig.this, CombatProcedureMove::bReservedSecondaryLocation),
                reserves
            );
        }
    }

    // ---- CreateAvoidNodeArray / CreatePath ----

    #[test]
    fn avoid_node_array_for_kind_four_adds_the_destination_nodes() {
        let mut rig = Rig::new();
        let t = rig.t();
        let out = rig.e.mem.alloc(4);
        rig.e.register(TIME_STAMP_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        rig.set_kind(4);
        rig.answer(GET_THREAT_MAP, 0x61);
        rig.answer(THREAT_MAP_NODE, 0x62);
        rig.answer_float(GAME_TIME_NOW, 7.5);
        rig.answer(SMART_POINTER_GET, 0x63);
        rig.answer(AVOID_NODE_SOURCE, 0x64);
        rig.answer(BUILD_AVOID_NODES, 0x65);
        let (_, calls) = rig.run(0x009d_82e0, &args![t, out]);
        assert_eq!(
            addresses(&calls),
            vec![
                GET_ATTACKER,
                GET_COMBAT_STATE,
                POSITION_CALL,
                GET_THREAT_MAP,
                THREAT_MAP_NODE,
                ARRAY_POINTER_SET,
                GAME_TIME_NOW,
                TIME_STAMP_SET,
                SMART_POINTER_GET,
                IDENTITY,
                AVOID_NODE_SOURCE,
                BUILD_AVOID_NODES,
                ARRAY_POINTER_SET,
            ]
        );
        assert_eq!(first_call(&calls, GET_THREAT_MAP), vec![0x5555]);
        assert_eq!(
            first_call(&calls, THREAT_MAP_NODE),
            vec![0x61, rig.position, rig.actor, bits(-1.0)]
        );
        assert_eq!(
            calls_to(&calls, ARRAY_POINTER_SET),
            vec![vec![out, 0x62], vec![out, 0x65]]
        );
        assert_eq!(
            first_call(&calls, BUILD_AVOID_NODES),
            vec![0x64, rig.actor, t + DESTINATION, bits(0.0), 0x63]
        );
        assert_eq!(rig.e.mem.f32(rig.state + 0xc0), 7.5);
    }

    #[test]
    fn avoid_node_array_for_a_target_kind_reads_the_target_and_uses_the_table() {
        let mut rig = Rig::new();
        let t = rig.t();
        let out = rig.e.mem.alloc(4);
        rig.set_kind(1);
        rig.kind_table(1, 0, 1);
        rig.answer(GET_TARGET, 0x91);
        rig.answer(SMART_POINTER_GET, 0x63);
        rig.answer(AVOID_NODE_SOURCE, 0x64);
        rig.answer(BUILD_AVOID_NODES_WITH_TARGET, 0x66);
        let (_, calls) = rig.run(0x009d_82e0, &args![t, out]);
        assert!(called(&calls, GET_TARGET));
        assert!(!called(&calls, BUILD_AVOID_NODES));
        assert_eq!(
            first_call(&calls, BUILD_AVOID_NODES_WITH_TARGET),
            vec![0x64, rig.actor, t + DESTINATION, bits(0.0), 0x63, 0x91]
        );
        assert_eq!(
            calls_to(&calls, ARRAY_POINTER_SET).last().unwrap(),
            &vec![out, 0x66]
        );
        // Kind 2 without a table entry adds nothing beyond the threat node.
        let mut rig = Rig::new();
        rig.set_kind(2);
        let (_, calls) = rig.run(0x009d_82e0, &args![t, out]);
        assert!(!called(&calls, GET_TARGET));
        assert_eq!(calls_to(&calls, ARRAY_POINTER_SET).len(), 1);
        // A negative kind skips everything after the stamp.
        let mut rig = Rig::new();
        rig.set_kind(-1);
        let (_, calls) = rig.run(0x009d_82e0, &args![t, out]);
        assert!(!called(&calls, GET_TARGET));
    }

    #[test]
    fn create_path_wraps_the_request_in_an_avoid_node_smart_pointer() {
        let mut rig = Rig::new();
        let t = rig.t();
        rig.set_kind(2);
        rig.answer(SET_PATHFINDING_GOAL, 1);
        let (result, calls) = rig.run(0x009d_7c80, &args![t]);
        assert!(result.bool());
        assert_eq!(calls[0].0, ARRAY_POINTER_CONSTRUCT);
        let avoid = calls[0].1[0];
        assert_eq!(calls[0].1[1], 0);
        assert_eq!(
            calls.last().unwrap(),
            &(ARRAY_POINTER_DESTRUCT, vec![avoid])
        );
        assert_eq!(first_call(&calls, GET_THREAT_MAP), vec![0x5555]);
        assert_eq!(first_call(&calls, SET_PATHFINDING_GOAL)[0], rig.actor);
        // A rejected goal gives false.
        rig.answer(SET_PATHFINDING_GOAL, 0);
        assert!(!rig.run(0x009d_7c80, &args![t]).0.bool());
    }

    // ---- CreatePath_ov2 ----

    /// Records the goal point words each `006e2980` call receives.
    fn record_goals(rig: &mut Rig) -> Rc<RefCell<Vec<[u32; 3]>>> {
        let goals = Rc::new(RefCell::new(Vec::new()));
        let seen = goals.clone();
        rig.e.register_double(REQUEST_SET_GOAL, move |e, a| {
            seen.borrow_mut()
                .push([e.mem.u32(a[1]), e.mem.u32(a[1] + 4), e.mem.u32(a[1] + 8)]);
            Ret::default()
        });
        goals
    }

    #[test]
    fn path_request_is_built_and_given_to_the_actor() {
        let mut rig = Rig::new();
        let t = rig.t();
        let avoid = rig.e.mem.alloc(4);
        let goals = record_goals(&mut rig);
        rig.set_kind(2);
        rig.answer(SET_PATHFINDING_GOAL, 1);
        rig.answer(GET_CURRENT_PATHING_REQUEST, 0x71);
        rig.answer(SMART_POINTER_GET, 0x72);
        rig.answer(FORM_ID, 0x73);
        let (result, calls) = rig.run(0x009d_7d00, &args![t, avoid]);
        assert!(result.bool());
        assert_eq!(
            addresses(&calls),
            vec![
                GET_ATTACKER,
                GET_TARGET,
                GET_COMBAT_STATE,
                COVER_RECORD,
                PATHING_REQUEST_CONSTRUCT,
                COVERED_MOVE_REQUEST_CONSTRUCT,
                SMART_POINTER_GET,
                FORM_ID,
                IDENTITY,
                BUILD_REQUEST,
                REQUEST_SET_GOAL,
                SET_PATHFINDING_GOAL,
                GET_CURRENT_PATHING_REQUEST,
                SMART_POINTER_SET,
                SET_MOVE_MODE,
                COVERED_MOVE_REQUEST_DESTRUCT,
                PATHING_REQUEST_DESTRUCT,
            ]
        );
        let plain = first_call(&calls, PATHING_REQUEST_CONSTRUCT)[0];
        let covered = first_call(&calls, COVERED_MOVE_REQUEST_CONSTRUCT)[0];
        assert_ne!(plain, covered);
        assert_eq!(
            first_call(&calls, BUILD_REQUEST),
            vec![
                rig.actor,
                plain,
                t + CURRENT_DESTINATION,
                0x73,
                bits(0.0),
                0x72
            ]
        );
        assert_eq!(
            first_call(&calls, SET_PATHFINDING_GOAL),
            vec![rig.actor, plain]
        );
        assert_eq!(first_call(&calls, REQUEST_SET_GOAL)[0], plain);
        assert_eq!(first_call(&calls, REQUEST_SET_GOAL)[2], bits(0.0));
        assert_eq!(goals.borrow()[0], [bits(1.0), bits(2.0), bits(3.0)]);
        assert_eq!(first_call(&calls, SMART_POINTER_SET), vec![t + 0x58, 0x71]);
        assert_eq!(first_call(&calls, SET_MOVE_MODE), vec![t, 3]);
        assert_eq!(
            first_call(&calls, COVERED_MOVE_REQUEST_DESTRUCT),
            vec![covered]
        );
        assert_eq!(first_call(&calls, PATHING_REQUEST_DESTRUCT), vec![plain]);
        assert_eq!(rig.e.get(rig.this, CombatProcedureMove::iCurrentNode), -1);
    }

    #[test]
    fn a_rejected_goal_fails_the_procedure() {
        let mut rig = Rig::new();
        let t = rig.t();
        let avoid = rig.e.mem.alloc(4);
        rig.set_kind(2);
        rig.e.set(rig.this, CombatProcedureMove::iCurrentNode, 5);
        let (result, calls) = rig.run(0x009d_7d00, &args![t, avoid]);
        assert!(!result.bool());
        assert_eq!(
            first_call(&calls, FAIL_PROCEDURE),
            vec![t, CANNOT_SETUP_PATH_MESSAGE]
        );
        assert!(!called(&calls, GET_CURRENT_PATHING_REQUEST));
        assert!(!called(&calls, SET_MOVE_MODE));
        assert!(called(&calls, COVERED_MOVE_REQUEST_DESTRUCT));
        assert!(called(&calls, PATHING_REQUEST_DESTRUCT));
        assert_eq!(rig.e.get(rig.this, CombatProcedureMove::iCurrentNode), 5);
    }

    #[test]
    fn kind_zero_uses_move_mode_four_when_the_plan_contains_the_action() {
        let mut rig = Rig::new();
        let t = rig.t();
        let avoid = rig.e.mem.alloc(4);
        rig.set_kind(0);
        rig.answer(SET_PATHFINDING_GOAL, 1);
        let (_, calls) = rig.run(0x009d_7d00, &args![t, avoid]);
        assert_eq!(
            first_call(&calls, CONTAINS_ACTION),
            vec![rig.controller, 0x18]
        );
        assert_eq!(first_call(&calls, SET_MOVE_MODE), vec![t, 3]);
        rig.answer(CONTAINS_ACTION, 1);
        let (_, calls) = rig.run(0x009d_7d00, &args![t, avoid]);
        assert_eq!(first_call(&calls, SET_MOVE_MODE), vec![t, 4]);
    }

    #[test]
    fn kind_three_aims_at_the_attackers_position_with_the_far_radius() {
        let mut rig = Rig::new();
        let t = rig.t();
        let avoid = rig.e.mem.alloc(4);
        let goals = record_goals(&mut rig);
        let position = rig.position;
        for (i, v) in [4.0f32, 5.0, 6.0].into_iter().enumerate() {
            rig.e.mem.set_f32(position + 4 * i as u32, v);
        }
        rig.set_kind(3);
        rig.answer(SET_PATHFINDING_GOAL, 1);
        let (_, calls) = rig.run(0x009d_7d00, &args![t, avoid]);
        assert_eq!(goals.borrow()[0], [bits(4.0), bits(5.0), bits(6.0)]);
        assert_eq!(first_call(&calls, REQUEST_SET_GOAL)[2], bits(640.0));
    }

    #[test]
    fn a_negative_target_radius_is_made_positive_and_flagged() {
        let mut rig = Rig::new();
        let t = rig.t();
        let avoid = rig.e.mem.alloc(4);
        rig.set_kind(2);
        rig.set_state(1);
        rig.e
            .set(rig.this, CombatProcedureMove::fTargetRadius, -2.0);
        rig.answer(SET_PATHFINDING_GOAL, 1);
        let (_, calls) = rig.run(0x009d_7d00, &args![t, avoid]);
        let plain = first_call(&calls, PATHING_REQUEST_CONSTRUCT)[0];
        assert_eq!(first_call(&calls, BUILD_REQUEST)[4], bits(2.0));
        assert_eq!(first_call(&calls, REQUEST_SET_FLAG), vec![plain, 0]);
        // A positive radius is kept and sets no flag.
        rig.e.set(rig.this, CombatProcedureMove::fTargetRadius, 2.0);
        let (_, calls) = rig.run(0x009d_7d00, &args![t, avoid]);
        assert_eq!(first_call(&calls, BUILD_REQUEST)[4], bits(2.0));
        assert!(!called(&calls, REQUEST_SET_FLAG));
        // Outside state 1 the radius is not used.
        rig.set_state(0);
        let (_, calls) = rig.run(0x009d_7d00, &args![t, avoid]);
        assert_eq!(first_call(&calls, BUILD_REQUEST)[4], bits(0.0));
    }

    #[test]
    fn the_covered_move_request_is_used_when_the_cover_flags_allow_it() {
        let mut rig = Rig::new();
        let t = rig.t();
        let avoid = rig.e.mem.alloc(4);
        rig.set_kind(2);
        rig.answer(SET_PATHFINDING_GOAL, 1);
        rig.e.mem.set_u8(COVER_MODE_BYTE, 1);
        rig.kind_table(2, 1, 0);
        let state = rig.state;
        rig.e.mem.set_u32(state + 0x90, 0b10);
        let (_, calls) = rig.run(0x009d_7d00, &args![t, avoid]);
        let covered = first_call(&calls, COVERED_MOVE_REQUEST_CONSTRUCT)[0];
        assert_eq!(
            first_call(&calls, BUILD_COVERED_MOVE_REQUEST),
            vec![state, covered]
        );
        assert_eq!(
            first_call(&calls, COVERED_REQUEST_SET_KIND),
            vec![covered, 2]
        );
        assert_eq!(first_call(&calls, BUILD_REQUEST)[1], covered);
        // An attacker with a process keeps the plain request.
        rig.answer(GET_PROCESS_TYPE, 1);
        let (_, calls) = rig.run(0x009d_7d00, &args![t, avoid]);
        let plain = first_call(&calls, PATHING_REQUEST_CONSTRUCT)[0];
        assert!(!called(&calls, BUILD_COVERED_MOVE_REQUEST));
        assert_eq!(first_call(&calls, BUILD_REQUEST)[1], plain);
    }

    #[test]
    fn a_location_record_in_the_actors_space_sets_goal_and_radius() {
        let mut rig = Rig::new();
        let t = rig.t();
        let avoid = rig.e.mem.alloc(4);
        let goals = record_goals(&mut rig);
        rig.set_kind(2);
        rig.answer(SET_PATHFINDING_GOAL, 1);
        let record = rig.e.mem.alloc(0x20);
        rig.e.mem.set_u8(record, 1);
        for (i, v) in [7.0f32, 8.0, 9.0].into_iter().enumerate() {
            rig.e.mem.set_f32(record + 8 + 4 * i as u32, v);
        }
        rig.answer(COVER_RECORD, record);
        // The id of the record's space and of the attacker's space agree.
        rig.answer(FORM_ID, 0x73);
        rig.answer(GET_SPACE, 0x73);
        rig.answer_float(GET_LOCATION_TARGET_DISTANCE, 5.0);
        rig.answer_float(POINT_LENGTH_SQUARED, 100.0);
        let (_, calls) = rig.run(0x009d_7d00, &args![t, avoid]);
        assert_eq!(
            first_call(&calls, COVER_RECORD_TEST),
            vec![rig.state, 0x100]
        );
        // 5 * 5 < 100: the radius is dropped.
        assert_eq!(goals.borrow()[0], [bits(7.0), bits(8.0), bits(9.0)]);
        assert_eq!(first_call(&calls, REQUEST_SET_GOAL)[2], bits(0.0));
        // 5 * 5 >= 10: the radius stays.
        rig.answer_float(POINT_LENGTH_SQUARED, 10.0);
        let (_, calls) = rig.run(0x009d_7d00, &args![t, avoid]);
        assert_eq!(first_call(&calls, REQUEST_SET_GOAL)[2], bits(5.0));
        // A different space leaves the defaults.
        rig.answer(GET_SPACE, 0x74);
        let (_, calls) = rig.run(0x009d_7d00, &args![t, avoid]);
        assert!(!called(&calls, GET_LOCATION_TARGET_DISTANCE));
        assert_eq!(
            goals.borrow().last().unwrap(),
            &[bits(1.0), bits(2.0), bits(3.0)]
        );
        // Kind 8 and a record that fails its test are skipped too.
        rig.answer(GET_SPACE, 0x73);
        rig.set_kind(8);
        let (_, calls) = rig.run(0x009d_7d00, &args![t, avoid]);
        assert!(!called(&calls, GET_LOCATION_TARGET_DISTANCE));
        rig.set_kind(2);
        rig.answer(COVER_RECORD_TEST, 1);
        let (_, calls) = rig.run(0x009d_7d00, &args![t, avoid]);
        assert!(!called(&calls, GET_LOCATION_TARGET_DISTANCE));
    }

    #[test]
    fn a_target_in_view_adds_an_avoid_node_around_it() {
        let mut rig = Rig::new();
        let t = rig.t();
        let avoid = rig.e.mem.alloc(4);
        let target = rig.object();
        rig.e
            .set(rig.this, CombatProcedureMove::pTargetRef, Ptr::new(target));
        rig.set_kind(1);
        rig.set_state(1);
        rig.answer(SET_PATHFINDING_GOAL, 1);
        rig.answer_float(GET_RADIUS, 3.5);
        rig.answer(PATHING_LOCATION_IS_VALID, 1);
        rig.answer(CALCULATE_LOS, 1);
        rig.answer(ALLOCATE, 0x80);
        rig.answer(ARRAY_CONSTRUCT, 0x81);
        rig.answer(AVOID_NODE_CONSTRUCT, 0x82);
        rig.queue(SMART_POINTER_GET, &[0, 0x72, 0x72]);
        rig.setting(SETTING_011CF120, 1.5);
        rig.setting(SETTING_011CF4E0, 2.5);
        let (_, calls) = rig.run(0x009d_7d00, &args![t, avoid]);
        let face = first_call(&calls, COMPUTE_FACE_TARGET_LOCATION)[0];
        let pathing = first_call(&calls, PATHING_LOCATION_NEW_FROM_LOCATION)[0];
        assert_eq!(
            first_call(&calls, COMPUTE_FACE_TARGET_LOCATION),
            vec![face, rig.actor, target, t + DESTINATION, bits(-1.0)]
        );
        assert_eq!(
            first_call(&calls, PATHING_LOCATION_NEW_FROM_LOCATION),
            vec![pathing, face, rig.actor]
        );
        assert_eq!(
            first_call(&calls, PATHING_LOCATION_IS_VALID),
            vec![pathing, bits(3.5)]
        );
        assert_eq!(
            first_call(&calls, CALCULATE_LOS),
            vec![rig.actor, target, face, t + DESTINATION, 0, 0xffff_ffff]
        );
        assert_eq!(
            first_call(&calls, SET_LOCATION_FROM_PATHING),
            vec![t + CURRENT_DESTINATION, face]
        );
        // The array did not exist: it is allocated (0x18 bytes), constructed
        // and stored; then the node is added to it.
        assert_eq!(first_call(&calls, ALLOCATE), vec![0x18]);
        assert_eq!(first_call(&calls, ARRAY_CONSTRUCT), vec![0x80]);
        assert_eq!(first_call(&calls, ARRAY_POINTER_SET), vec![avoid, 0x81]);
        let node = first_call(&calls, AVOID_NODE_CONSTRUCT);
        assert_eq!(node[1..], [rig.position, bits(2.5), bits(1.5)]);
        assert_eq!(first_call(&calls, AVOID_ARRAY_ADD), vec![0x72, 0x82]);
        assert_eq!(first_call(&calls, PATHING_LOCATION_DESTRUCT), vec![pathing]);
        // The request is built with the array the pointer holds now.
        assert_eq!(first_call(&calls, BUILD_REQUEST)[5], 0x72);
    }

    #[test]
    fn an_invalid_pathing_location_adds_no_node_but_is_still_destroyed() {
        let mut rig = Rig::new();
        let t = rig.t();
        let avoid = rig.e.mem.alloc(4);
        let target = rig.object();
        rig.e
            .set(rig.this, CombatProcedureMove::pTargetRef, Ptr::new(target));
        rig.set_kind(1);
        rig.set_state(1);
        rig.answer(SET_PATHFINDING_GOAL, 1);
        let (_, calls) = rig.run(0x009d_7d00, &args![t, avoid]);
        assert!(!called(&calls, CALCULATE_LOS));
        assert!(called(&calls, PATHING_LOCATION_DESTRUCT));
        // Valid, but not in sight.
        rig.answer(PATHING_LOCATION_IS_VALID, 1);
        let (_, calls) = rig.run(0x009d_7d00, &args![t, avoid]);
        assert!(called(&calls, CALCULATE_LOS));
        assert!(!called(&calls, SET_LOCATION_FROM_PATHING));
        assert!(called(&calls, PATHING_LOCATION_DESTRUCT));
    }

    // ---- DisplayDebugText ----

    /// A printer object whose print slot records its words.
    fn printer(rig: &mut Rig) -> (u32, Rc<RefCell<Log>>) {
        let printer = rig.object();
        let lines = Rc::new(RefCell::new(Vec::new()));
        let seen = lines.clone();
        rig.e.register_double(PRINT_CALL, move |_, a| {
            seen.borrow_mut().push((a[3], a.to_vec()));
            Ret::default()
        });
        (printer, lines)
    }

    fn doubles(values: &[f64]) -> Vec<u32> {
        let mut words = Vec::new();
        for v in values {
            words.extend(args![*v]);
        }
        words
    }

    #[test]
    fn debug_text_for_the_initial_state_prints_the_destination() {
        let mut rig = Rig::new();
        let t = rig.t();
        let (printer, lines) = printer(&mut rig);
        for (i, v) in [1.5f32, 2.5, 3.5].into_iter().enumerate() {
            rig.e.mem.set_f32(t + DESTINATION + 4 * i as u32, v);
        }
        rig.run(0x009d_8520, &args![t, printer, 0x11u32, 0x22u32]);
        let lines = lines.borrow();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0].1, vec![printer, 0x11, 0x22, TEXT_INITIALIZING]);
        let mut expected = vec![printer, 0x11, 0x22, TEXT_DESTINATION];
        expected.extend(doubles(&[1.5, 2.5, 3.5]));
        assert_eq!(lines[1].1, expected);
    }

    #[test]
    fn debug_text_for_the_cover_states_prints_the_timers() {
        let mut rig = Rig::new();
        let t = rig.t();
        let (printer, lines) = printer(&mut rig);
        rig.answer_float(TIMER_VALUE_A, 4.0);
        rig.answer_float(TIMER_VALUE_B, 3.0);
        rig.set_state(1);
        rig.run(0x009d_8520, &args![t, printer, 1u32, 2u32]);
        assert_eq!(lines.borrow()[0].0, TEXT_MOVING);
        lines.borrow_mut().clear();
        rig.set_state(2);
        rig.run(0x009d_8520, &args![t, printer, 1u32, 2u32]);
        {
            let lines = lines.borrow();
            let formats: Vec<u32> = lines.iter().map(|l| l.0).collect();
            assert_eq!(
                formats,
                vec![
                    TEXT_MOVING_USING_COVER,
                    TEXT_CHECK_COVER_TIMER,
                    TEXT_DESTINATION
                ]
            );
            // The format's first value is the result of 00435e00.
            let mut expected = vec![printer, 1, 2, TEXT_CHECK_COVER_TIMER];
            expected.extend(doubles(&[3.0, 4.0]));
            assert_eq!(lines[1].1, expected);
        }
        lines.borrow_mut().clear();
        rig.set_state(3);
        let (_, calls) = rig.run(0x009d_8520, &args![t, printer, 1u32, 2u32]);
        let formats: Vec<u32> = lines.borrow().iter().map(|l| l.0).collect();
        assert_eq!(
            formats,
            vec![
                TEXT_HIDING_BEHIND_COVER,
                TEXT_HIDING_TIMER,
                TEXT_CHECK_COVER_TIMER,
                TEXT_DESTINATION
            ]
        );
        assert_eq!(
            calls_to(&calls, TIMER_VALUE_A),
            vec![vec![t + HIDING_TIMER], vec![t + CHECK_COVER_TIMER]]
        );
        // Another state prints no state line.
        lines.borrow_mut().clear();
        rig.set_state(4);
        rig.run(0x009d_8520, &args![t, printer, 1u32, 2u32]);
        assert_eq!(lines.borrow().len(), 1);
    }

    #[test]
    fn debug_text_names_the_space_of_a_destination_outside_the_attackers() {
        let mut rig = Rig::new();
        let t = rig.t();
        let (printer, lines) = printer(&mut rig);
        let space = rig.object();
        rig.answer(FORM_ID, space);
        rig.answer(GET_SPACE, space + 4);
        rig.answer(NAME_CALL, 0x5000);
        for (i, v) in [1.0f32, 2.0, 3.0].into_iter().enumerate() {
            rig.e.mem.set_f32(t + DESTINATION + 4 * i as u32, v);
        }
        rig.run(0x009d_8520, &args![t, printer, 1u32, 2u32]);
        let mut expected = vec![printer, 1, 2, TEXT_DESTINATION_WITH_SPACE, 0x5000];
        expected.extend(doubles(&[1.0, 2.0, 3.0]));
        assert_eq!(lines.borrow()[1].1, expected);
        // The attacker's own space prints the plain destination.
        lines.borrow_mut().clear();
        rig.answer(GET_SPACE, space);
        rig.run(0x009d_8520, &args![t, printer, 1u32, 2u32]);
        assert_eq!(lines.borrow()[1].0, TEXT_DESTINATION);
    }

    #[test]
    fn debug_text_describes_the_target_and_the_destination_timer() {
        let mut rig = Rig::new();
        let t = rig.t();
        let (printer, lines) = printer(&mut rig);
        let target = rig.object();
        rig.e
            .set(rig.this, CombatProcedureMove::pTargetRef, Ptr::new(target));
        for (i, v) in [5.0f32, 6.0, 7.0].into_iter().enumerate() {
            rig.e.mem.set_f32(rig.position + 4 * i as u32, v);
        }
        rig.answer(HAS_NAME, 1);
        rig.answer(NAME_CALL, 0x5000);
        rig.answer(FALLBACK_NAME, 0x6000);
        // The destination has no space (0); the target answers its form id.
        rig.queue(FORM_ID, &[0, 0xabcd]);
        rig.answer(TIMER_RUNNING, 1);
        rig.answer_float(TIMER_VALUE_A, 9.0);
        rig.answer_float(TIMER_VALUE_B, 8.0);
        rig.run(0x009d_8520, &args![t, printer, 1u32, 2u32]);
        let lines = lines.borrow();
        let formats: Vec<u32> = lines.iter().map(|l| l.0).collect();
        assert_eq!(
            formats,
            vec![
                TEXT_INITIALIZING,
                TEXT_DESTINATION,
                TEXT_TARGET,
                TEXT_CHECK_DESTINATION_TIMER
            ]
        );
        let mut expected = vec![printer, 1, 2, TEXT_TARGET, 0x5000, 0xabcd];
        expected.extend(doubles(&[5.0, 6.0, 7.0]));
        assert_eq!(lines[2].1, expected);
        let mut expected = vec![printer, 1, 2, TEXT_CHECK_DESTINATION_TIMER];
        expected.extend(doubles(&[8.0, 9.0]));
        assert_eq!(lines[3].1, expected);
    }

    #[test]
    fn debug_text_uses_the_fallback_name_of_a_target_without_one() {
        let mut rig = Rig::new();
        let t = rig.t();
        let (printer, lines) = printer(&mut rig);
        let target = rig.object();
        rig.e
            .set(rig.this, CombatProcedureMove::pTargetRef, Ptr::new(target));
        rig.answer(FALLBACK_NAME, 0x6000);
        rig.run(0x009d_8520, &args![t, printer, 1u32, 2u32]);
        assert_eq!(lines.borrow()[2].1[4], 0x6000);
    }

    // ---- SaveGame / LoadGame / InitLoadGame ----

    #[test]
    fn save_game_writes_the_fields_in_order() {
        let mut rig = Rig::new();
        let t = rig.t();
        let buffer = rig.object();
        rig.e
            .set(rig.this, CombatProcedureMove::pTargetRef, Ptr::new(0x900));
        rig.answer(SMART_POINTER_GET, 0x72);
        let (_, calls) = rig.run(0x009d_8880, &args![t, buffer]);
        let expected: Log = vec![
            (BASE_SAVE_GAME, vec![t, buffer]),
            (SAVE_BYTES, vec![buffer, t + 0x10, 4, 0]),
            (WORLD_LOCATION_SAVE, vec![t + 0x14, buffer]),
            (WORLD_LOCATION_SAVE, vec![t + 0x24, buffer]),
            (SAVE_BYTES, vec![buffer, t + 0x34, 4, 0]),
            (SAVE_FORM_ID, vec![buffer, 0x900, 0]),
            (SAVE_BYTES, vec![buffer, t + 0x3c, 4, 0]),
            (SAVE_BYTES, vec![buffer, t + 0x60, 4, 0]),
            (TIMER_SAVE, vec![t + 0x64, buffer]),
            (TIMER_SAVE, vec![t + 0x40, buffer]),
            (TIMER_SAVE, vec![t + 0x6c, buffer]),
            (TIMER_SAVE, vec![t + 0x74, buffer]),
            (SAVE_BYTES, vec![buffer, t + 0x48, 0xc, 0]),
            (SAVE_BYTES, vec![buffer, t + 0x54, 1, 0]),
            (SMART_POINTER_GET, vec![t + 0x58]),
            (SAVE_BYTES, vec![buffer, t + 0x5c, 1, 0]),
        ];
        assert_eq!(calls, expected);
        assert!(rig
            .e
            .get(rig.this, CombatProcedureMove::bHasQueuedPathingRequest));
        rig.answer(SMART_POINTER_GET, 0);
        rig.run(0x009d_8880, &args![t, buffer]);
        assert!(!rig
            .e
            .get(rig.this, CombatProcedureMove::bHasQueuedPathingRequest));
    }

    #[test]
    fn load_game_reads_the_fields_and_honours_the_save_version() {
        let mut rig = Rig::new();
        let t = rig.t();
        let buffer = rig.object();
        let head: Log = vec![
            (BASE_LOAD_GAME, vec![t, buffer]),
            (LOAD_BYTES, vec![buffer, t + 0x10, 4]),
            (WORLD_LOCATION_LOAD, vec![t + 0x14, buffer]),
            (WORLD_LOCATION_LOAD, vec![t + 0x24, buffer]),
            (LOAD_BYTES, vec![buffer, t + 0x34, 4]),
            (LOAD_FORM_ID, vec![buffer, t + 0x38]),
            (LOAD_BYTES, vec![buffer, t + 0x3c, 4]),
            (LOAD_BYTES, vec![buffer, t + 0x60, 4]),
            (TIMER_LOAD, vec![t + 0x64, buffer]),
            (TIMER_LOAD, vec![t + 0x40, buffer]),
            (TIMER_LOAD, vec![t + 0x6c, buffer]),
            (VERSION_CALL, vec![buffer]),
        ];
        let timer: Log = vec![(TIMER_LOAD, vec![t + 0x74, buffer])];
        let middle: Log = vec![
            (LOAD_BYTES, vec![buffer, t + 0x48, 0xc]),
            (LOAD_BYTES, vec![buffer, t + 0x54, 1]),
            (VERSION_CALL, vec![buffer]),
        ];
        let queued: Log = vec![(LOAD_BYTES, vec![buffer, t + 0x5c, 1])];
        for (version, with_timer, with_queued) in [
            (7u32, false, false),
            (8, true, false),
            (11, true, false),
            (12, true, true),
        ] {
            rig.answer(VERSION_CALL, version);
            let (_, calls) = rig.run(0x009d_89a0, &args![t, buffer]);
            let mut expected = head.clone();
            if with_timer {
                expected.extend(timer.clone());
            }
            expected.extend(middle.clone());
            if with_queued {
                expected.extend(queued.clone());
            }
            assert_eq!(calls, expected, "version {version}");
        }
    }

    #[test]
    fn init_load_game_turns_the_form_id_into_the_reference() {
        let mut rig = Rig::new();
        let t = rig.t();
        rig.e
            .set(rig.this, CombatProcedureMove::pTargetRef, Ptr::new(0x1234));
        rig.answer(LOOKUP_FORM, 0x99);
        rig.answer(RT_DYNAMIC_CAST, 0xabc);
        let (_, calls) = rig.run(0x009d_8ad0, &args![t, 0x55u32]);
        assert_eq!(
            calls,
            vec![
                (GET_ATTACKER, vec![t]),
                (BASE_INIT_LOAD_GAME, vec![t, 0x55]),
                (LOOKUP_FORM, vec![0x1234]),
                (
                    RT_DYNAMIC_CAST,
                    vec![0x99, 0, CAST_SOURCE_TYPE, CAST_TARGET_TYPE, 0]
                ),
            ]
        );
        assert_eq!(
            rig.e.get(rig.this, CombatProcedureMove::pTargetRef).addr(),
            0xabc
        );
        // No target: no lookup, and the queued request is re-queued when saved.
        rig.e
            .set(rig.this, CombatProcedureMove::pTargetRef, Ptr::NULL);
        rig.e.set(
            rig.this,
            CombatProcedureMove::bHasQueuedPathingRequest,
            true,
        );
        rig.answer(GET_CURRENT_PATHING_REQUEST, 0x71);
        let (_, calls) = rig.run(0x009d_8ad0, &args![t, 0x55u32]);
        assert_eq!(
            calls,
            vec![
                (GET_ATTACKER, vec![t]),
                (BASE_INIT_LOAD_GAME, vec![t, 0x55]),
                (GET_CURRENT_PATHING_REQUEST, vec![rig.actor]),
                (SMART_POINTER_SET, vec![t + 0x58, 0x71]),
            ]
        );
        assert_eq!(
            rig.e.get(rig.this, CombatProcedureMove::pTargetRef).addr(),
            0
        );
    }

    // ---- Update ----

    #[test]
    fn update_in_state_zero_starts_the_first_path() {
        let mut rig = Rig::new();
        let t = rig.t();
        rig.set_kind(2);
        rig.e.mem.set_u32(t + DESTINATION, 7);
        rig.e.mem.set_u32(t + DESTINATION + 12, 9);
        let calls = rig.update();
        assert_eq!(rig.e.mem.u32(t + CURRENT_DESTINATION), 7);
        assert_eq!(rig.e.mem.u32(t + CURRENT_DESTINATION + 12), 9);
        assert_eq!(rig.e.get(rig.this, CombatProcedureMove::eState), 1);
        assert_eq!(
            addresses(&calls)[..4],
            [GET_ATTACKER, GET_PROCESS_TYPE, GET_TARGET, GET_COMBAT_STATE]
        );
        assert!(called(&calls, ARRAY_POINTER_CONSTRUCT));
        // The goal was rejected: no timer.
        assert!(!called(&calls, TIMER_START));
    }

    #[test]
    fn update_in_state_zero_starts_the_destination_timer_for_kind_0xf() {
        let mut rig = Rig::new();
        let t = rig.t();
        rig.set_kind(0xf);
        rig.answer(SET_PATHFINDING_GOAL, 1);
        rig.setting(SETTING_011CE9C0, 3.0);
        let calls = rig.update();
        assert_eq!(
            calls_to(&calls, TIMER_START),
            vec![vec![t + CHECK_DESTINATION_TIMER, bits(3.0)]]
        );
        // Other kinds do not.
        rig.set_state(0);
        rig.set_kind(2);
        let calls = rig.update();
        assert!(!called(&calls, TIMER_START));
    }

    #[test]
    fn update_ignores_states_above_three() {
        let mut rig = Rig::new();
        rig.set_state(4);
        let calls = rig.update();
        assert_eq!(
            addresses(&calls),
            vec![GET_ATTACKER, GET_PROCESS_TYPE, GET_TARGET, GET_COMBAT_STATE]
        );
    }

    #[test]
    fn update_reports_a_failed_path_and_marks_the_destination_unreachable() {
        let mut rig = Rig::new();
        let t = rig.t();
        let state = rig.state;
        rig.set_state(1);
        rig.set_kind(0);
        rig.answer(PATH_FAILED_TEST, 1);
        let calls = rig.update();
        assert_eq!(first_call(&calls, STATE_OR_FLAGS), vec![state, 1]);
        assert_eq!(
            calls_to(&calls, ADD_UNREACHABLE_LOCATION),
            vec![vec![state, t + CURRENT_DESTINATION]]
        );
        assert!(called(&calls, COMBAT_STATE_REFRESH));
        assert_eq!(
            calls.last().unwrap(),
            &(FAIL_PROCEDURE, vec![t, MESSAGE_PATH_BUILDING_FAILED])
        );
        // A valid path that failed while moving gets the other message and
        // no bookkeeping.
        rig.answer(IS_PATH_VALID, 1);
        let calls = rig.update();
        assert!(!called(&calls, ADD_UNREACHABLE_LOCATION));
        assert_eq!(
            calls.last().unwrap(),
            &(FAIL_PROCEDURE, vec![t, MESSAGE_PATHING_FAILED])
        );
    }

    #[test]
    fn update_failed_path_bookkeeping_depends_on_the_kind() {
        let mut rig = Rig::new();
        let t = rig.t();
        let state = rig.state;
        rig.set_state(1);
        rig.answer(PATH_FAILED_TEST, 1);
        // Kind 3 sets the byte; a last cover location of the same kind is
        // marked unreachable and cleared.
        rig.set_kind(3);
        let last = rig.e.mem.alloc(0x80);
        rig.e.mem.set_i32(last + 4, 3);
        rig.e.mem.set_u32(last + 0x68, 0x4242);
        rig.answer(COMBAT_STATE_LAST_COVER, last);
        let calls = rig.update();
        assert_eq!(first_call(&calls, STATE_SET_BYTE_188), vec![state]);
        assert!(!called(&calls, STATE_OR_FLAGS));
        assert_eq!(
            first_call(&calls, ADD_UNREACHABLE_COVER_LOCATION),
            vec![state, 0x4242]
        );
        assert_eq!(first_call(&calls, CLEAR_COVER_LOCATION), vec![state, 0]);
        // Kind 8 ORs bit 8.
        rig.set_kind(8);
        rig.e.mem.set_i32(last + 4, 5);
        let calls = rig.update();
        assert_eq!(first_call(&calls, STATE_OR_FLAGS), vec![state, 0x100]);
        assert!(!called(&calls, CLEAR_COVER_LOCATION));
        // Kind 1 outside state 2 also marks the start unreachable when the
        // two destinations are near each other.
        rig.set_kind(1);
        rig.answer(POINTS_NEAR, 1);
        let calls = rig.update();
        assert_eq!(
            calls_to(&calls, ADD_UNREACHABLE_LOCATION),
            vec![
                vec![state, t + CURRENT_DESTINATION],
                vec![state, t + DESTINATION]
            ]
        );
        assert_eq!(
            first_call(&calls, POINTS_NEAR),
            vec![t + CURRENT_DESTINATION, t + DESTINATION]
        );
        // In state 2 the cover location is marked instead.
        rig.set_state(2);
        let cover = rig.e.mem.alloc(0x80);
        rig.e.mem.set_u32(cover + 0x68, 0x777);
        rig.answer(COMBAT_STATE_COVER, cover);
        let calls = rig.update();
        assert_eq!(
            first_call(&calls, ADD_UNREACHABLE_COVER_LOCATION),
            vec![state, 0x777]
        );
        assert_eq!(
            first_call(&calls, CLEAR_SECONDARY_COVER_LOCATION),
            vec![state, 0]
        );
        assert!(!called(&calls, POINTS_NEAR));
    }

    #[test]
    fn update_fails_when_the_target_reference_is_gone() {
        let mut rig = Rig::new();
        let t = rig.t();
        let target = rig.object();
        rig.e
            .set(rig.this, CombatProcedureMove::pTargetRef, Ptr::new(target));
        rig.set_state(1);
        rig.answer(REF_DISABLED, 1);
        let calls = rig.update();
        assert_eq!(
            calls.last().unwrap(),
            &(FAIL_PROCEDURE, vec![t, MESSAGE_TARGET_GONE])
        );
        assert!(!called(&calls, REF_DELETED));
        rig.answer(REF_DISABLED, 0);
        rig.answer(REF_DELETED, 1);
        let calls = rig.update();
        assert_eq!(
            calls.last().unwrap(),
            &(FAIL_PROCEDURE, vec![t, MESSAGE_TARGET_GONE])
        );
    }

    #[test]
    fn update_follows_a_target_that_moved_too_far() {
        let mut rig = Rig::new();
        let t = rig.t();
        let target = rig.object();
        rig.e
            .set(rig.this, CombatProcedureMove::pTargetRef, Ptr::new(target));
        rig.set_state(1);
        rig.set_kind(2);
        rig.setting(SETTING_011CE494, 9.0);
        rig.answer_float(DISTANCE_SQUARED, 100.0);
        let calls = rig.update();
        // 9 * 9 < 100: the destination moves to the target's place.
        assert_eq!(rig.e.mem.f32(t + DESTINATION), 10.0);
        assert_eq!(rig.e.mem.f32(t + DESTINATION + 8), 30.0);
        assert_eq!(rig.e.mem.u32(t + DESTINATION + 12), 0x1234);
        assert_eq!(rig.e.mem.f32(t + CURRENT_DESTINATION + 4), 20.0);
        assert!(called(&calls, ARRAY_POINTER_CONSTRUCT));
        assert!(!called(&calls, IS_PATHING_COMPLETE));
        // Settings are read twice each (the code reads and multiplies).
        assert_eq!(calls_to(&calls, SETTING_VALUE).len(), 2);
        // A target that stayed within the limit changes nothing.
        let mut rig = Rig::new();
        let t = rig.t();
        let target = rig.object();
        rig.e
            .set(rig.this, CombatProcedureMove::pTargetRef, Ptr::new(target));
        rig.set_state(1);
        rig.set_kind(2);
        rig.setting(SETTING_011CE494, 9.0);
        rig.answer_float(DISTANCE_SQUARED, 81.0);
        let calls = rig.update();
        assert_eq!(rig.e.mem.f32(t + DESTINATION), 0.0);
        assert!(!called(&calls, ARRAY_POINTER_CONSTRUCT));
        assert!(called(&calls, IS_PATHING_COMPLETE));
    }

    #[test]
    fn update_picks_the_follow_limit_by_state_and_process() {
        // State 2 uses `011cf7b8`; an attacker with a process `011ce654`.
        let mut rig = Rig::new();
        let target = rig.object();
        rig.e
            .set(rig.this, CombatProcedureMove::pTargetRef, Ptr::new(target));
        rig.set_state(2);
        rig.set_kind(2);
        let calls = rig.update();
        let settings: Vec<u32> = calls_to(&calls, SETTING_VALUE)
            .iter()
            .map(|c| c[0])
            .collect();
        assert_eq!(settings[..2], [SETTING_011CF7B8, SETTING_011CF7B8]);
        let mut rig = Rig::new();
        let target = rig.object();
        rig.e
            .set(rig.this, CombatProcedureMove::pTargetRef, Ptr::new(target));
        rig.set_state(1);
        rig.set_kind(2);
        rig.answer(GET_PROCESS_TYPE, 3);
        let calls = rig.update();
        let settings: Vec<u32> = calls_to(&calls, SETTING_VALUE)
            .iter()
            .map(|c| c[0])
            .collect();
        assert_eq!(settings[..2], [SETTING_011CE654, SETTING_011CE654]);
        // A target with a process also uses `011ce654`.
        let mut rig = Rig::new();
        let target = rig.object();
        rig.e
            .set(rig.this, CombatProcedureMove::pTargetRef, Ptr::new(target));
        rig.set_state(1);
        rig.set_kind(2);
        rig.answer(PROCESS_CALL, 1);
        rig.queue(GET_PROCESS_TYPE, &[0, 4]);
        let calls = rig.update();
        let settings: Vec<u32> = calls_to(&calls, SETTING_VALUE)
            .iter()
            .map(|c| c[0])
            .collect();
        assert_eq!(settings[..2], [SETTING_011CE654, SETTING_011CE654]);
        assert_eq!(first_call(&calls, GET_PROCESS_TYPE).len(), 1);
    }

    #[test]
    fn update_in_state_one_completes_on_arrival() {
        let mut rig = Rig::new();
        let t = rig.t();
        let state = rig.state;
        rig.set_state(1);
        rig.set_kind(5);
        rig.answer(IS_PATHING_COMPLETE, 1);
        let calls = rig.update();
        assert_eq!(
            calls.iter().rev().take(2).rev().cloned().collect::<Log>(),
            vec![
                (ARRIVAL_STEP, vec![state, t + DESTINATION, 5]),
                (COMPLETE_PROCEDURE, vec![t]),
            ]
        );
        assert!(!called(&calls, STOP_MOVING));
    }

    #[test]
    fn update_in_state_two_turns_to_hiding_on_arrival() {
        let mut rig = Rig::new();
        let t = rig.t();
        rig.set_state(2);
        rig.set_kind(6);
        rig.answer(IS_PATHING_COMPLETE, 1);
        let calls = rig.update();
        assert_eq!(rig.e.get(rig.this, CombatProcedureMove::eState), 3);
        assert_eq!(
            calls_to(&calls, TIMER_START),
            vec![
                vec![t + HIDING_TIMER, bits(2.0)],
                vec![t + CHECK_COVER_TIMER, bits(0.25)]
            ]
        );
        assert_eq!(
            first_call(&calls, SET_LOOK_AT_TARGET),
            vec![t, t + LOOK_AT_POSITION, 3]
        );
        assert_eq!(first_call(&calls, SET_MOVE_MODE), vec![t, 2]);
        assert!(!called(&calls, COMPLETE_PROCEDURE));
    }

    #[test]
    fn update_kind_0xf_stops_when_the_destination_is_in_sight() {
        let mut rig = Rig::new();
        let t = rig.t();
        rig.set_state(1);
        rig.set_kind(0xf);
        rig.answer(TIMER_EXPIRED, 1);
        rig.answer_float(DISTANCE_SQUARED, 100.0);
        rig.setting(SETTING_011CF2A0, 20.0);
        rig.setting(SETTING_011CFAD4, 1.25);
        rig.answer(CHECK_LINE_OF_SIGHT, 1);
        let calls = rig.update();
        let pathing = first_call(&calls, PATHING_LOCATION_NEW_FROM_POINT)[0];
        assert_eq!(
            first_call(&calls, PATHING_LOCATION_NEW_FROM_POINT),
            vec![pathing, t + DESTINATION]
        );
        assert_eq!(first_call(&calls, CHECK_LINE_OF_SIGHT).len(), 4);
        assert_eq!(first_call(&calls, CHECK_LINE_OF_SIGHT)[0], pathing);
        assert_eq!(first_call(&calls, PATHING_LOCATION_DESTRUCT), vec![pathing]);
        assert_eq!(
            calls_to(&calls, TIMER_START).last().unwrap(),
            &vec![t + CHECK_DESTINATION_TIMER, bits(1.25)]
        );
        assert_eq!(first_call(&calls, STOP_MOVING), vec![rig.actor]);
        assert!(called(&calls, COMPLETE_PROCEDURE));
        // 20 * 20 = 400 is not above a distance of 400: out of range.
        rig.answer_float(DISTANCE_SQUARED, 400.0);
        let calls = rig.update();
        assert!(!called(&calls, PATHING_LOCATION_NEW_FROM_POINT));
        assert!(!called(&calls, STOP_MOVING));
        assert!(!called(&calls, COMPLETE_PROCEDURE));
        assert_eq!(calls_to(&calls, TIMER_START).len(), 1);
    }

    #[test]
    fn update_consumes_a_queued_request_that_is_not_in_use() {
        let mut rig = Rig::new();
        let t = rig.t();
        let state = rig.state;
        let request = rig.object();
        rig.set_state(1);
        rig.set_kind(5);
        rig.answer(SMART_POINTER_GET, request);
        rig.answer(TYPE_CALL, 8);
        rig.answer(REQUEST_BYTE_E1, 1);
        rig.answer(REQUEST_WORD_108, 0x31);
        rig.answer(REQUEST_GET_104, 0);
        rig.e.mem.set_u32(t + DESTINATION + 4, 5);
        let calls = rig.update();
        assert_eq!(
            first_call(&calls, STATE_COPY_RECORD),
            vec![state + 0x128, 0x31]
        );
        assert_eq!(first_call(&calls, COMBAT_STATE_SET_COVER), vec![state, 0]);
        assert_eq!(first_call(&calls, REQUEST_SET_104), vec![request, 0]);
        assert_eq!(rig.e.mem.u32(t + CURRENT_DESTINATION + 4), 5);
        // The queue is emptied at the end of the block.
        assert_eq!(calls_to(&calls, SMART_POINTER_SET), vec![vec![t + 0x58, 0]]);
        // The request is kept while the actor still waits on its path.
        rig.answer(IS_WAITING_ON_PATH, 1);
        let calls = rig.update();
        assert!(!called(&calls, TYPE_CALL));
        assert!(!called(&calls, SMART_POINTER_SET));
        // A request of another type is dropped without effect.
        rig.answer(IS_WAITING_ON_PATH, 0);
        rig.answer(TYPE_CALL, 7);
        let calls = rig.update();
        assert!(!called(&calls, STATE_COPY_RECORD));
        assert_eq!(calls_to(&calls, SMART_POINTER_SET).len(), 1);
    }

    #[test]
    fn update_re_creates_the_path_when_the_cover_check_fails() {
        let mut rig = Rig::new();
        let t = rig.t();
        let state = rig.state;
        let request = rig.object();
        rig.set_state(1);
        rig.set_kind(5);
        rig.answer(SMART_POINTER_GET, request);
        rig.answer(TYPE_CALL, 8);
        rig.answer(REQUEST_BYTE_E1, 1);
        rig.answer(REQUEST_GET_104, 0x40);
        rig.answer(REQUEST_POINT, 0x66);
        rig.answer(GET_SPACE, 0x31);
        let calls = rig.update();
        assert_eq!(
            first_call(&calls, SET_LOCATION),
            vec![t + CURRENT_DESTINATION, 0x66, 0x31]
        );
        assert_eq!(
            first_call(&calls, STATE_CHECK_LOCATION),
            vec![state, rig.e.mem.u32(state + 0xe8)]
        );
        assert_eq!(calls_to(&calls, SMART_POINTER_SET), vec![vec![t + 0x58, 0]]);
        assert!(called(&calls, ARRAY_POINTER_CONSTRUCT));
        assert_eq!(rig.e.get(rig.this, CombatProcedureMove::eState), 1);
    }

    #[test]
    fn update_switches_to_the_cover_state_for_a_queued_covered_move() {
        let mut rig = Rig::new();
        let t = rig.t();
        let state = rig.state;
        let request = rig.object();
        rig.set_state(1);
        rig.set_kind(5);
        rig.answer(SMART_POINTER_GET, request);
        rig.answer(TYPE_CALL, 8);
        rig.answer(REQUEST_BYTE_E1, 1);
        rig.answer(REQUEST_GET_104, 0x40);
        rig.answer(REQUEST_POINT, 0x66);
        rig.answer(GET_SPACE, 0x31);
        rig.answer(STATE_CHECK_LOCATION, 1);
        rig.answer(COMBAT_STATE_COVER, 0x44);
        rig.answer_float(GET_TARGET_FIRING_Z_OFFSET, 1.5);
        rig.e.mem.set_f32(t + LOOK_AT_Z, 2.0);
        let calls = rig.update();
        assert_eq!(rig.e.get(rig.this, CombatProcedureMove::eState), 2);
        assert!(rig
            .e
            .get(rig.this, CombatProcedureMove::bReservedSecondaryLocation));
        let spot = first_call(&calls, IDENTITY)[0];
        assert_eq!(
            first_call(&calls, CALCULATE_COVER_POSITIONS),
            vec![rig.controller, 0x44, spot, t + LOOK_AT_POSITION]
        );
        assert_eq!(
            first_call(&calls, GET_TARGET_FIRING_Z_OFFSET),
            vec![rig.actor, 2, 0]
        );
        assert_eq!(rig.e.mem.f32(t + LOOK_AT_Z), 3.5);
        assert_eq!(
            calls_to(&calls, TIMER_START).first().unwrap(),
            &vec![t + CHECK_COVER_TIMER, bits(0.5)]
        );
        assert_eq!(calls_to(&calls, SMART_POINTER_SET), vec![vec![t + 0x58, 0]]);
        assert!(!called(&calls, ARRAY_POINTER_CONSTRUCT));
        let _ = state;
    }

    /// The state for the cover search of state 1: kind 2 with the table and
    /// state bit set, so that the search runs.
    fn cover_search_rig() -> Rig {
        let mut rig = Rig::new();
        rig.set_state(1);
        rig.set_kind(2);
        rig.kind_table(2, 1, 0);
        let state = rig.state;
        rig.e.mem.set_u32(state + 0x90, 0b10);
        rig.answer(CURRENT_PATH_NODE, 5);
        rig.answer(SET_PATHFINDING_GOAL, 1);
        rig.answer_float(POINT_UNITIZE, 100.0);
        // The first node of a new path.
        rig.e.set(rig.this, CombatProcedureMove::iCurrentNode, -1);
        rig
    }

    #[test]
    fn update_finds_cover_while_walking_the_path_ahead() {
        let mut rig = cover_search_rig();
        let t = rig.t();
        let state = rig.state;
        rig.queue(PATH_NODE_POINT, &[1, 1, 0]);
        rig.queue(FIND_COVER_LOCATION, &[0, 1]);
        // The first read (the queued request) finds nothing.
        rig.queue(SMART_POINTER_GET, &[0]);
        rig.answer(SMART_POINTER_GET, 0x72);
        rig.answer(COMBAT_STATE_COVER, 0x44);
        rig.answer(GET_SPACE, 0x31);
        rig.answer_float(GET_TARGET_FIRING_Z_OFFSET, 1.5);
        let calls = rig.update();
        // The accepted path to the cover resets the node index (`CreatePath`).
        assert_eq!(rig.e.get(rig.this, CombatProcedureMove::iCurrentNode), -1);
        assert_eq!(rig.e.get(rig.this, CombatProcedureMove::eState), 2);
        assert!(rig
            .e
            .get(rig.this, CombatProcedureMove::bReservedSecondaryLocation));
        assert_eq!(rig.e.mem.f32(t + LOOK_AT_Z), 1.5);
        let timers = calls_to(&calls, TIMER_START);
        assert_eq!(
            timers,
            vec![
                vec![t + COVERED_MOVE_SEARCH_TIMER, bits(2.0)],
                vec![t + CHECK_COVER_TIMER, bits(0.5)]
            ]
        );
        // Two nodes were fetched (6 then 7) and the avoid array built once.
        let nodes = calls_to(&calls, PATH_NODE_POINT);
        assert_eq!(nodes.len(), 2);
        assert_eq!(nodes[0][..2], [rig.actor, 6]);
        assert_eq!(nodes[1][..2], [rig.actor, 7]);
        // One for the search, one for the path to the cover.
        assert_eq!(calls_to(&calls, THREAT_MAP_NODE).len(), 2);
        let searches = calls_to(&calls, FIND_COVER_LOCATION);
        assert_eq!(searches.len(), 2);
        assert_eq!(searches[0][0], state);
        assert_eq!(searches[0][3..], [2, bits(512.0), 0x72]);
        assert_eq!(
            calls_to(&calls, SEARCH_CONSTRUCT).len(),
            calls_to(&calls, SEARCH_DESTRUCT).len()
        );
        assert_eq!(calls_to(&calls, RESERVE_COVER_LOCATION), vec![vec![state]]);
        let point = first_call(&calls, CALCULATE_COVER_POSITIONS)[2];
        assert_eq!(
            first_call(&calls, SET_LOCATION),
            vec![t + CURRENT_DESTINATION, point, 0x31]
        );
        // The smart pointer made for the search is destroyed again.
        let avoid = first_call(&calls, ARRAY_POINTER_CONSTRUCT);
        assert_eq!(
            calls_to(&calls, ARRAY_POINTER_DESTRUCT).last().unwrap()[0],
            avoid[0]
        );
        assert!(!called(&calls, IS_PATHING_COMPLETE) || called(&calls, IS_PATHING_COMPLETE));
    }

    #[test]
    fn update_cover_search_uses_the_bigger_range_for_the_first_node() {
        let mut rig = cover_search_rig();
        rig.e.set(rig.this, CombatProcedureMove::iCurrentNode, 5);
        let calls = rig.update();
        // Same node and the timer has not expired: no search at all.
        assert!(!called(&calls, ARRAY_POINTER_CONSTRUCT));
        assert!(!called(&calls, TIMER_START));
        // With the timer expired it searches with the normal range.
        rig.answer(TIMER_EXPIRED, 1);
        rig.queue(PATH_NODE_POINT, &[1, 0]);
        let calls = rig.update();
        assert_eq!(first_call(&calls, FIND_COVER_LOCATION)[4], bits(256.0));
        // The first node of a path (-1) uses the bigger range.
        let mut rig = cover_search_rig();
        rig.e.set(rig.this, CombatProcedureMove::iCurrentNode, -1);
        rig.queue(PATH_NODE_POINT, &[1, 0]);
        let calls = rig.update();
        assert_eq!(first_call(&calls, FIND_COVER_LOCATION)[4], bits(512.0));
        assert_eq!(rig.e.get(rig.this, CombatProcedureMove::iCurrentNode), 5);
    }

    #[test]
    fn update_cover_search_clips_the_last_step_to_the_maximum_length() {
        let mut rig = cover_search_rig();
        rig.queue(PATH_NODE_POINT, &[1, 1, 1]);
        rig.answer_float(POINT_UNITIZE, 2000.0);
        let sum = rig.e.mem.alloc(12);
        for (i, v) in [9.0f32, 8.0, 7.0].into_iter().enumerate() {
            rig.e.mem.set_f32(sum + 4 * i as u32, v);
        }
        rig.answer(POINT_SUM, sum);
        let calls = rig.update();
        // The first step already exceeds 1536: it is scaled to 1536 and the
        // search ends after one pass.
        assert_eq!(calls_to(&calls, PATH_NODE_POINT).len(), 1);
        assert_eq!(first_call(&calls, POINT_SCALE)[1], bits(1536.0));
        let search = calls_to(&calls, FIND_COVER_LOCATION);
        assert_eq!(search.len(), 1);
        let next = first_call(&calls, PATH_NODE_POINT)[2];
        assert_eq!(search[0][2], next);
        assert_eq!(first_call(&calls, POINT_DIFFERENCE)[0], next);
    }

    #[test]
    fn update_stops_when_the_path_to_the_found_cover_cannot_be_made() {
        let mut rig = cover_search_rig();
        rig.queue(PATH_NODE_POINT, &[1]);
        rig.answer(FIND_COVER_LOCATION, 1);
        rig.answer(SET_PATHFINDING_GOAL, 0);
        let calls = rig.update();
        assert_eq!(rig.e.get(rig.this, CombatProcedureMove::eState), 2);
        assert!(called(&calls, SEARCH_DESTRUCT));
        assert!(called(&calls, ARRAY_POINTER_DESTRUCT));
        // Update ends there: the arrival handling does not run.
        assert!(!called(&calls, IS_PATHING_COMPLETE));
        assert_eq!(calls_to(&calls, TIMER_START).len(), 1);
    }

    #[test]
    fn update_cover_search_does_not_run_when_the_cover_flags_forbid_it() {
        for case in 0..4 {
            let mut rig = cover_search_rig();
            match case {
                0 => rig.e.mem.set_u8(COVER_MODE_BYTE, 1),
                1 => rig.answer(IS_WAITING_ON_PATH, 1),
                2 => rig.answer(GET_PROCESS_TYPE, 1),
                _ => {
                    let state = rig.state;
                    rig.e.mem.set_u32(state + 0x90, 0);
                }
            }
            let calls = rig.update();
            assert!(!called(&calls, CURRENT_PATH_NODE), "case {case}");
        }
        // State 2 does not search either.
        let mut rig = cover_search_rig();
        rig.set_state(2);
        let calls = rig.update();
        assert!(!called(&calls, CURRENT_PATH_NODE));
    }

    #[test]
    fn update_retargets_a_cover_that_is_no_longer_safe() {
        let mut rig = Rig::new();
        let t = rig.t();
        let state = rig.state;
        let target = rig.object();
        let cover = rig.e.mem.alloc(0x80);
        rig.set_state(2);
        rig.set_kind(2);
        rig.answer(GET_TARGET, target);
        rig.answer(TIMER_EXPIRED, 1);
        rig.answer(COMBAT_STATE_COVER_CHECK_DUE, 1);
        rig.answer(COMBAT_STATE_COVER, cover);
        rig.answer_float(DISTANCE_SQUARED, 100.0);
        rig.answer(LINE_TEST, 1);
        rig.answer(COVER_POINT, 0x66);
        rig.answer(CHECK_COVER_SAFE, 1);
        let calls = rig.update();
        // Safe: the cover is kept and the timer restarted.
        assert_eq!(
            first_call(&calls, CHECK_COVER_SAFE),
            vec![state, cover + 0x14, t + CURRENT_DESTINATION, target, 0]
        );
        assert_eq!(
            first_call(&calls, LINE_TEST),
            vec![0x66, rig.position, cover + 0x3c, cover + 0x48]
        );
        assert_eq!(first_call(&calls, COVER_POINT)[0], cover + 0x14);
        assert!(!called(&calls, UNRESERVE_COVER_LOCATION));
        assert_eq!(
            first_call(&calls, TIMER_START),
            vec![t + CHECK_COVER_TIMER, bits(0.5)]
        );
        // Not safe: the cover is released and the path re-created.
        rig.answer(CHECK_COVER_SAFE, 0);
        rig.e.set(
            rig.this,
            CombatProcedureMove::bReservedSecondaryLocation,
            true,
        );
        rig.set_state(2);
        let calls = rig.update();
        assert_eq!(first_call(&calls, UNRESERVE_COVER_LOCATION), vec![state]);
        assert_eq!(
            first_call(&calls, CLEAR_SECONDARY_COVER_LOCATION),
            vec![state, 0]
        );
        assert!(!rig
            .e
            .get(rig.this, CombatProcedureMove::bReservedSecondaryLocation));
        assert_eq!(rig.e.get(rig.this, CombatProcedureMove::eState), 1);
        assert!(called(&calls, ARRAY_POINTER_CONSTRUCT));
        // No line of sight to the target from the cover: also retargets.
        rig.answer(CHECK_COVER_SAFE, 1);
        rig.answer(LINE_TEST, 0);
        rig.set_state(2);
        let calls = rig.update();
        assert!(!called(&calls, CHECK_COVER_SAFE));
        assert!(called(&calls, UNRESERVE_COVER_LOCATION));
        // A far attacker skips the cover tests.
        rig.e.mem.set_f64(MAX_COVER_DISTANCE_SQUARED, 50.0);
        rig.set_state(2);
        let calls = rig.update();
        assert!(!called(&calls, LINE_TEST));
        assert!(!called(&calls, UNRESERVE_COVER_LOCATION));
    }

    #[test]
    fn update_leaves_the_hiding_state_when_the_hiding_timer_ends() {
        let mut rig = Rig::new();
        let t = rig.t();
        let state = rig.state;
        rig.set_state(3);
        rig.e.set(
            rig.this,
            CombatProcedureMove::bReservedSecondaryLocation,
            true,
        );
        rig.answer(COMBAT_STATE_COVER, 0x44);
        rig.answer(TIMER_EXPIRED, 1);
        let calls = rig.update();
        assert_eq!(rig.e.get(rig.this, CombatProcedureMove::eState), 0);
        assert!(!rig
            .e
            .get(rig.this, CombatProcedureMove::bReservedSecondaryLocation));
        assert_eq!(
            calls[4..].to_vec(),
            vec![
                (COMBAT_STATE_COVER, vec![state]),
                (TIMER_EXPIRED, vec![t + HIDING_TIMER]),
                (UNRESERVE_COVER_LOCATION, vec![state]),
                (CLEAR_SECONDARY_COVER_LOCATION, vec![state, 0]),
                (CLEAR_LOOK_AT_TARGET, vec![t]),
            ]
        );
        // Without a cover the state also ends.
        let mut rig = Rig::new();
        rig.set_state(3);
        rig.update();
        assert_eq!(rig.e.get(rig.this, CombatProcedureMove::eState), 0);
        // With a cover and no target nothing happens.
        let mut rig = Rig::new();
        rig.set_state(3);
        rig.answer(COMBAT_STATE_COVER, 0x44);
        rig.update();
        assert_eq!(rig.e.get(rig.this, CombatProcedureMove::eState), 3);
    }

    #[test]
    fn update_hiding_compares_the_start_plus_one_with_the_game_time() {
        let mut rig = Rig::new();
        let t = rig.t();
        let state = rig.state;
        let target = rig.object();
        rig.set_state(3);
        rig.answer(GET_TARGET, target);
        rig.answer(COMBAT_STATE_COVER, 0x44);
        rig.answer_float(FLOAT_AT, 5.0);
        // 5 + 1 is not before the game time 6.
        rig.answer_float(GAME_TIME_NOW, 6.0);
        rig.update();
        assert_eq!(rig.e.get(rig.this, CombatProcedureMove::eState), 3);
        // Before 7, and the state's flag is set: leaves.
        rig.answer_float(GAME_TIME_NOW, 7.0);
        rig.answer(COMBAT_STATE_FLAG, 1);
        let calls = rig.update();
        assert_eq!(rig.e.get(rig.this, CombatProcedureMove::eState), 0);
        assert!(!called(&calls, COMBAT_STATE_TIME_COPY));
        // Flag clear: the copied time is compared too (5 + 1 < 5.5 is false).
        rig.set_state(3);
        rig.answer(COMBAT_STATE_FLAG, 0);
        rig.answer(COMBAT_STATE_TIME_COPY, 0x3000);
        let calls = rig.update();
        assert_eq!(rig.e.get(rig.this, CombatProcedureMove::eState), 3);
        assert_eq!(first_call(&calls, COMBAT_STATE_TIME_COPY)[0], state);
        assert_eq!(calls_to(&calls, FLOAT_AT).last().unwrap(), &vec![0x3000]);
        // 6 < 6.5: leaves.
        rig.queue_float(FLOAT_AT, &[5.0, 6.5]);
        let calls = rig.update();
        assert_eq!(rig.e.get(rig.this, CombatProcedureMove::eState), 0);
        assert!(called(&calls, CLEAR_LOOK_AT_TARGET));
        let _ = t;
    }

    #[test]
    fn update_hiding_re_checks_the_cover_when_its_timer_expires() {
        let mut rig = Rig::new();
        let t = rig.t();
        let state = rig.state;
        let target = rig.object();
        let cover = rig.e.mem.alloc(0x80);
        rig.set_state(3);
        rig.answer(GET_TARGET, target);
        rig.answer(COMBAT_STATE_COVER, cover);
        rig.answer_float(FLOAT_AT, 5.0);
        rig.answer_float(GAME_TIME_NOW, 0.0);
        rig.answer(COMBAT_STATE_COVER_CHECK_DUE, 1);
        // The hiding timer has not expired, the cover timer has.
        rig.queue(TIMER_EXPIRED, &[0, 1]);
        rig.answer(CHECK_COVER_SAFE, 1);
        let calls = rig.update();
        assert_eq!(
            first_call(&calls, CHECK_COVER_SAFE),
            vec![state, cover + 0x14, t + CURRENT_DESTINATION, target, 1]
        );
        assert_eq!(rig.e.get(rig.this, CombatProcedureMove::eState), 3);
        assert_eq!(
            first_call(&calls, TIMER_START),
            vec![t + CHECK_COVER_TIMER, bits(0.25)]
        );
        // An unsafe cover ends the state.
        rig.queue(TIMER_EXPIRED, &[0, 1]);
        rig.answer(CHECK_COVER_SAFE, 0);
        rig.update();
        assert_eq!(rig.e.get(rig.this, CombatProcedureMove::eState), 0);
    }
}
