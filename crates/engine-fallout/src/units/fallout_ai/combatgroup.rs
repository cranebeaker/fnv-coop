//! `fallout/ai/combat/combatgroup.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The unit holds `CombatGroup` (a group of combat actors with the targets
//! they fight), the two element classes of its arrays (`CombatTarget`,
//! `CombatMember`) and the small `CombatGroupCluster` / `CombatSearchLocation`
//! records. The unit is translated over several sessions, in address order:
//! this part covers `0069cfe0`, `00985410` to `00986c60`, `009871c0` to
//! `0098a4d0` and `0098a580` to `0098fd10`; the last function, `00990500`
//! (`CombatGroup::ComputeMemberHealthPercentage`), completes the unit.
//!
//! Conventions of this file:
//! - Layouts first (below); embedded structs that other units own
//!   (`BGSWorldLocation`, 0x10 bytes; `CombatTimer`, 8 bytes;
//!   `CombatTimeStamp`, a `float`) are addressed through offset constants.
//! - Tiny accessors the game calls (array size, element `i`, a form's ID)
//!   are called by address like the game does; they are folded template
//!   instances, so their constants are named by what the body does.
//! - `CombatGroup`'s `BSSimpleArray<CombatTarget>` is at +0x08 (elements of
//!   0x68 bytes), its `BSSimpleArray<CombatMember>` at +0x18 (0x14 bytes).
//! - The compiler's exception-unwinding frames in the constructor,
//!   destructor and `009862e0` are not translated.
//! - x87 note: comparisons of `float`s follow the disassembly's flag tests
//!   (strict `<` / `>`, and a NaN fails them); sums stay in `f32`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSSimpleArray;
use crate::units::crt::glue::alloca16_esp;

/// Global holding the `CombatManager*` singleton.
const COMBAT_MANAGER: u32 = 0x011f_1958;
/// `BSSimpleArray<T>::GetSize`-like accessor (`this->iSize` at +8).
const ARRAY_SIZE: u32 = 0x0044_ddc0;
/// The group's target count (`TargetArray.iSize`), taking the group.
const TARGET_COUNT: u32 = 0x005a_4320;
/// The group's member count (`MemberArray.iSize`), taking the group.
const MEMBER_COUNT: u32 = 0x0099_0890;
/// Element `i` of a `BSSimpleArray<CombatTarget>` (address of the element).
const TARGET_AT: u32 = 0x0096_a2b0;
/// Element `i` of a `BSSimpleArray<CombatMember>` (address of the element).
const MEMBER_AT: u32 = 0x006d_ad90;
/// The actor of the group's target `i`, taking the group.
const TARGET_ACTOR_AT: u32 = 0x0096_8630;
/// The actor of the group's member `i`, taking the group.
const MEMBER_ACTOR_AT: u32 = 0x0090_3600;
/// Element `i` of a `BSSimpleArray<CombatGroupCluster *>` (address of the
/// slot).
const CLUSTER_AT: u32 = 0x006a_7ad0;
/// `SetAt(i, value)` of a `BSSimpleArray<T *>`.
const ARRAY_SET_AT: u32 = 0x0047_a110;
/// `BSSimpleArray<T *>::SetSize(size, 1)`.
const POINTER_ARRAY_SET_SIZE: u32 = 0x006f_2290;
/// `BSSimpleArray<CombatSearchDoor>::SetSize(size, 1)`.
const SEARCH_DOOR_ARRAY_SET_SIZE: u32 = 0x0069_cea0;
/// Element `i` of a `BSSimpleArray<CombatSearchDoor>`.
const SEARCH_DOOR_AT: u32 = 0x006a_1440;
/// `TESForm::iFormID` getter (`this + 0xc`); the same body reads
/// `BGSWorldLocation::pSpace`.
const FORM_ID: u32 = 0x0084_e3a0;
/// `CombatTimeStamp` value (`float` at `this`), in ST0.
const TIME_STAMP_VALUE: u32 = 0x006a_7f50;
/// Game time now (`float` global at `011f1bf0`), in ST0.
const GAME_TIME_NOW: u32 = 0x0043_5dd0;
/// `CombatTimeStamp` setter (`this = value`).
const TIME_STAMP_SET: u32 = 0x0043_5de0;
/// `CombatTimeStamp` zero constructor.
const TIME_STAMP_ZERO: u32 = 0x0043_00a0;
/// Game time now minus the stamp at `this`, in ST0.
const TIME_STAMP_AGE: u32 = 0x0043_5e00;
/// `CombatTimer` constructor (zeroes both floats).
const TIMER_CONSTRUCT: u32 = 0x0090_6f60;
/// `CombatTimer` has expired (game time minus start reaches the target
/// time), in AL.
const TIMER_EXPIRED: u32 = 0x008d_7f80;
/// `CombatTimer::Start(delay)`: start time now, target time `delay`.
const TIMER_START: u32 = 0x008d_7f40;
/// `CombatTimer` is running (target time not zero), in AL.
const TIMER_RUNNING: u32 = 0x0097_fa50;
/// `BGSWorldLocation` constructor (copies a default point, space null).
const WORLD_LOCATION_CONSTRUCT: u32 = 0x0097_ede0;
/// `BGSWorldLocation::GetDistanceSquared` (this, other), in ST0.
const DISTANCE_SQUARED: u32 = 0x0052_7a40;
/// `BGSWorldLocation::SaveGame` (this, buffer).
const WORLD_LOCATION_SAVE: u32 = 0x0052_7b70;
/// `BGSWorldLocation::LoadGame` (this, buffer).
const WORLD_LOCATION_LOAD: u32 = 0x0052_7ba0;
/// `CombatTimeStamp::SaveGame` (this, buffer).
const TIME_STAMP_SAVE: u32 = 0x009a_5f90;
/// `CombatTimeStamp::LoadGame` (this, buffer).
const TIME_STAMP_LOAD: u32 = 0x009a_5ff0;
/// `BGSSaveGameBuffer::SaveFormID_ov2` (buffer, form, 0).
const SAVE_FORM_ID: u32 = 0x0086_5df0;
/// `BGSSaveGameBuffer` raw write (buffer, data, size, 0).
const SAVE_BYTES: u32 = 0x0086_5e50;
/// `BGSLoadGameBuffer::LoadFormID_ov2` (buffer, address of the field).
const LOAD_FORM_ID: u32 = 0x0086_48e0;
/// `BGSLoadGameBuffer` raw read (buffer, data, size).
const LOAD_BYTES: u32 = 0x0086_4980;
/// `TESObjectREFR::GetWorldLocation(this, out)`.
const GET_WORLD_LOCATION: u32 = 0x0057_5650;
/// `Setting` value pointer (`this + 4`, or a static zero when `this` is
/// null): the setting's `float`.
const SETTING_FLOAT_VALUE: u32 = 0x0040_3e20;
/// The same for a `bool` setting.
const SETTING_BOOL_VALUE: u32 = 0x0040_8d60;
/// `Actor::IsFleeing(this, 0)`.
const ACTOR_IS_FLEEING: u32 = 0x008a_6650;
/// The actor test `008b06d0(this, target, 0, &out, 0)` (the engine map
/// calls it `Actor::GetShouldAttackActor`).
const ACTOR_GET_SHOULD_ATTACK: u32 = 0x008b_06d0;
/// Whether `this` accepts `other` as a combat partner (`this`, `other`),
/// in AL.
const ACTOR_COMBAT_COMPATIBLE: u32 = 0x008b_0670;
/// `CombatManager` calls: (manager; group, actor).
const MANAGER_ADD_TARGET: u32 = 0x0099_26c0;
const MANAGER_REMOVE_TARGET: u32 = 0x0099_26e0;
const MANAGER_ADD_MEMBER: u32 = 0x0099_2700;
const MANAGER_REMOVE_MEMBER: u32 = 0x008d_0600;
/// `BSSimpleArray<CombatTarget>::Add(element)` (returns the index).
const TARGET_ARRAY_ADD: u32 = 0x0098_eec0;
/// `BSSimpleArray<CombatTarget>::RemoveAt(index, 1)`.
const TARGET_ARRAY_REMOVE_AT: u32 = 0x0098_ef10;
/// `BSSimpleArray<CombatMember>::Add(element)`.
const MEMBER_ARRAY_ADD: u32 = 0x0098_f180;
/// `BSSimpleArray<CombatMember>::RemoveAt(index, 1)`.
const MEMBER_ARRAY_REMOVE_AT: u32 = 0x0098_f1f0;
/// The game's `TintScenegraph(node, color, 0)` / `UnTintScenegraph(node)`.
const TINT_SCENEGRAPH: u32 = 0x004b_6360;
const UNTINT_SCENEGRAPH: u32 = 0x004b_6630;
/// `printf`-style game log (`format, ...`).
const LOG_MESSAGE: u32 = 0x005b_5e40;
/// Setting holding the "tint combat members" flag (`bool`).
const TINT_SETTING: u32 = 0x011f_1878;
/// Setting holding the "only targets in the actor's cell" flag (`bool`).
const SAME_CELL_SETTING: u32 = 0x011f_1840;
/// `FLT_MAX` (`-FLT_MAX` is its negation).
const MAX_FLOAT: u32 = 0x0108_d540;
/// `-1.0f`.
const MINUS_ONE: u32 = 0x0101_2054;
/// `0.0` as a `double`.
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// Integer `Setting` value pointer (`this` = the setting): the setting's
/// `int`.
const SETTING_INT_VALUE: u32 = 0x0043_d4d0;
/// `NiPointer::operator->`-like getter (`[this]`).
const NI_POINTER_GET: u32 = 0x0055_9450;
/// `VirtualActorPathHandler::GetDistanceTraveled` (Xbox PDB), in ST0.
const PATH_HANDLER_DISTANCE_TRAVELLED: u32 = 0x004a_7bd0;
/// The actor test `00437bd0(actor)`, in AL (the map files it under
/// `extradataobjects.cpp`).
const ACTOR_TEST_00437BD0: u32 = 0x0043_7bd0;
/// `&this->TargetArray` of another group (`this + 8`).
const OTHER_TARGET_ARRAY: u32 = 0x0041_3f40;
/// `&this->MemberArray` of another group (`this + 0x18`).
const OTHER_MEMBER_ARRAY: u32 = 0x0050_0940;
/// Cluster `index` (a signed byte) of the group, or null (`0098de60`).
const CLUSTER_BY_INDEX: u32 = 0x0098_de60;
/// `BSSimpleArray<T *>::Add(&value)` (returns the index).
const POINTER_ARRAY_ADD: u32 = 0x007c_b2e0;
/// `COMBAT: Error merging groups.  Could not add member %08X %s`, and the
/// same for a target.
const MERGE_MEMBER_ERROR: u32 = 0x0108_d6a0;
const MERGE_TARGET_ERROR: u32 = 0x0108_d664;
/// `operator new(size)`.
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// `BSSimpleArray<PathingAvoidNode, 1024>` constructor (`this`).
const AVOID_ARRAY_CONSTRUCT: u32 = 0x006e_3850;
/// `PathingAvoidNode` constructor (`this`; point, two floats), 0x24 bytes.
const AVOID_NODE_CONSTRUCT: u32 = 0x0069_15d0;
/// `BSSimpleArray<PathingAvoidNode_1024>::Add(node)` (Xbox PDB).
const AVOID_ARRAY_ADD: u32 = 0x0090_5450;
/// `NiPoint3` arithmetic, all `thiscall`: `operator[](index)` returning
/// the address of the component; `+` and `-` (`this`; result, other);
/// `this * (1 / divisor)` into a result (`this`; result, divisor);
/// the same in place (`this`; divisor); `this * factor` into a result;
/// `+=`; the dot product and the squared length (ST0); `UnitizeGetLength`
/// (ST0); the conversion of a `BGSWorldLocation` that returns `this`.
const POINT_COMPONENT: u32 = 0x004a_51b0;
const POINT_ADD: u32 = 0x0043_9e90;
const POINT_SUBTRACT: u32 = 0x0043_9ef0;
const POINT_SCALE_DOWN: u32 = 0x0053_d280;
const POINT_DIVIDE_IN_PLACE: u32 = 0x0049_41c0;
const POINT_SCALE: u32 = 0x0045_bb20;
const POINT_ACCUMULATE: u32 = 0x0063_c8a0;
const POINT_DOT: u32 = 0x004b_6190;
const POINT_LENGTH_SQUARED: u32 = 0x004a_7290;
const POINT_UNITIZE_GET_LENGTH: u32 = 0x0045_7910;
const POINT_NO_OP: u32 = 0x0068_15c0;
/// The `cdecl` wrapper around `004019b0` that `fn_00989490` and the
/// intersection test use on a `float` (a square root).
const SQUARE_ROOT: u32 = 0x0045_79e0;
/// The compiler's vector constructor iterator (`ptr, size, count,
/// constructor`).
const VECTOR_CONSTRUCTOR: u32 = 0x0040_1050;
/// `2.0f`.
const TWO_FLOAT: u32 = 0x0101_62c0;
/// `1024.0f`.
const SPHERE_MINIMUM_RADIUS: u32 = 0x0102_36e0;
/// `4.0` as a `double`.
const FOUR_DOUBLE: u32 = 0x0101_db80;
/// The zero `NiPoint3` (three words) the accumulators start from.
const ZERO_POINT: u32 = 0x011f_426c;
/// `0.33` and `0.67` as `double`s.
const FRACTION_LOW: u32 = 0x0102_6998;
const FRACTION_HIGH: u32 = 0x0105_1680;
/// 8 corners x 3 axes of bytes: non-zero means the corner takes the
/// maximum on that axis.
const CORNER_TABLE: u32 = 0x0108_d57c;

/// `SearchLocations` (`BSSimpleArray<CombatSearchLocation, 1024>`, +0x110) accessors
/// the game calls with the group: its size, and element `i` (address).
const SEARCH_LOCATION_COUNT: u32 = 0x0098_8750;
const SEARCH_LOCATION_AT: u32 = 0x0098_8770;
/// Element `i` of a `BSSimpleArray<CombatSearchLocation>` (`array`, `i`).
const SEARCH_LOCATION_ELEMENT: u32 = 0x0098_ebc0;
/// The same through a different folded instance, used when a new entry is
/// filled in (`array`, `i`).
const SEARCH_LOCATION_SLOT: u32 = 0x006b_f670;
/// `BSSimpleArray<CombatSearchLocation>` append of one uninitialised
/// element: returns its index (`array`).
const SEARCH_LOCATION_ADD: u32 = 0x006b_fd40;
/// `BSSimpleArray<CombatSearchLocation>::RemoveAt` (`array`, `index`,
/// `count`).
const SEARCH_LOCATION_REMOVE: u32 = 0x0098_f430;
/// The group's member `i` (address of its `CombatMember`), taking the group.
const MEMBER_OF_GROUP: u32 = 0x0098_8790;
/// Element `i` of the `BSSimpleArray<CombatSearchDoor>` at +0x120 (`array`,
/// `i`), 12-byte elements.
const SEARCH_DOOR_ELEMENT: u32 = 0x006a_7af0;
/// Appends one uninitialised `CombatSearchDoor` and returns its index
/// (`array`).
const SEARCH_DOOR_ADD: u32 = 0x0097_8bc0;
/// `BSSimpleArray<T *>::RemoveAt` (`array`, `index`, `count`) for the
/// `ClusterArray`.
const CLUSTER_REMOVE: u32 = 0x009a_4320;
/// `BSSimpleArray<T *>::RemoveAt` (`array`, `index`, `count`, 0) that the
/// search-door code uses to drop duplicate doors.
const POINTER_ARRAY_REMOVE_RANGE: u32 = 0x0098_ebe0;
/// `BSSimpleArray<T *>` (16 bytes) constructor and destructor.
const POINTER_ARRAY_CONSTRUCT: u32 = 0x005e_04c0;
const POINTER_ARRAY_DESTRUCT: u32 = 0x005e_04f0;
/// Sorts a `BSSimpleArray` of 4-byte elements with a comparison function
/// (`array`, `compare`).
const ARRAY_SORT: u32 = 0x0072_9970;
/// The comparison function `006bfb30` (unsigned order of two words).
const COMPARE_WORDS: u32 = 0x006b_fb30;
/// Collects the teleport doors within a radius of a point into an array
/// (`space`, `point`, `radius`, `array`; cdecl).
const COLLECT_TELEPORT_DOORS: u32 = 0x006d_9350;
/// `BGSWorldLocation::GetDistance` (this, other), in ST0.
const WORLD_LOCATION_DISTANCE: u32 = 0x0052_7a80;
/// A scaled distance between two `BGSWorldLocation`s (this, other, scale
/// `NiPoint3`), in ST0.
const WORLD_LOCATION_SCALED_DISTANCE: u32 = 0x0052_7ac0;
/// `TESObjectREFR::GetSpace`.
const REFERENCE_SPACE: u32 = 0x0057_5ca0;
/// `MobileObject::GetCurrentProcessType` (Xbox PDB).
const PROCESS_TYPE: u32 = 0x0093_1850;
/// The `thiscall` getter `00586150` (`[this + 0x9c]`): the combat state
/// object of a process.
const PROCESS_COMBAT_STATE: u32 = 0x0058_6150;
/// `CombatState::CheckMovement` (Xbox PDB): (state, location, 1, 0), in AL.
const COMBAT_STATE_CHECK_MOVEMENT: u32 = 0x009a_02a0;
/// `CombatController::IsFleeing` (Xbox PDB), taking the process.
const CONTROLLER_IS_FLEEING: u32 = 0x0098_1990;
/// The process test `00981420(process)`, in AL.
const PROCESS_TEST_00981420: u32 = 0x0098_1420;
/// The process test `004013e0(process)`: bit 3 of the word at +8, in AL.
const PROCESS_FLAG_BIT_3: u32 = 0x0040_13e0;
/// `Actor::GetCurrentPathfindingGoal(actor, point)`, in AL.
const ACTOR_PATHFINDING_GOAL: u32 = 0x008b_3840;
/// `BSSimpleArray<ParentSpaceNode>::Add(array, point)`.
const PATH_POINT_ARRAY_ADD: u32 = 0x0049_f210;
/// `Actor::GetWeaponDamagePerSecond` (Xbox PDB), in ST0.
const ACTOR_WEAPON_DAMAGE_PER_SECOND: u32 = 0x008b_e600;
/// `Actor::CalculateCombatStrength(actor, damage per second)` (Xbox PDB),
/// in ST0.
const ACTOR_COMBAT_STRENGTH: u32 = 0x008a_cbe0;
/// The combat-state values `00979260(state)` (damage per second) and
/// `00453700(state)` (combat strength), in ST0.
const STATE_DAMAGE_PER_SECOND: u32 = 0x0097_9260;
const STATE_COMBAT_STRENGTH: u32 = 0x0045_3700;
/// `Actor::IsFleeing(actor, 0)` (Xbox PDB), in AL.
const ACTOR_IS_FLEEING_ARG: u32 = 0x008a_6650;
/// Global holding the pointer of the player's actor (compared with the members' actors).
const PLAYER: u32 = 0x011d_ea3c;
/// `NiPoint3` default constructor (returns `this`).
const POINT_CONSTRUCT: u32 = 0x0041_6870;
/// `BGSWorldLocation` constructor from a point (`this`; point, space).
const WORLD_LOCATION_FROM_POINT: u32 = 0x0043_a3c0;
/// `BGSWorldLocation::IsLocationLoaded`, `GetCellOrWorld`.
const WORLD_LOCATION_IS_LOADED: u32 = 0x0052_7a00;
const WORLD_LOCATION_GET_CELL_OR_WORLD: u32 = 0x0052_7990;
/// The point test `004390c0(this, other)`: whether the points are equal.
const POINTS_EQUAL: u32 = 0x0043_90c0;
/// `Pathing::FindClosestPointOnNavmesh` (Xbox PDB; cdecl: two words from
/// `GetCellOrWorld`, the point, the result).
const FIND_CLOSEST_POINT_ON_NAVMESH: u32 = 0x006d_6f80;
/// `PathingLocation` (0x28 bytes): constructor from a point and the two
/// words (`PathingLocation_ov7`), from an actor (`PathingLocation_ov3`),
/// `ResolveNavMeshInfo(0)`, the destructor, and the call `006f4ed0(point)`.
const PATHING_LOCATION_FROM_POINT: u32 = 0x006d_cee0;
const PATHING_LOCATION_FROM_ACTOR: u32 = 0x006d_cd70;
const PATHING_LOCATION_RESOLVE: u32 = 0x006d_d6f0;
const PATHING_LOCATION_DESTRUCT: u32 = 0x004f_f7e0;
const PATHING_LOCATION_SET_POINT: u32 = 0x006f_4ed0;
/// `PathingLOSGridMap` constructor (`this`), and its method
/// `006e0320(this, location, radius)`, `006e05b0(this)`.
const LOS_GRID_MAP_CONSTRUCT: u32 = 0x006e_01d0;
const LOS_GRID_MAP_UPDATE: u32 = 0x006e_0320;
const LOS_GRID_MAP_STEP: u32 = 0x006e_05b0;
/// `NiPointer` assignment (`address of the pointer`, `new value`).
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// The search constructors/destructors of the two arrays the search code
/// builds from the members: `(array, count, 0, 0)` and the destructors,
/// the `Add` of each, and the call `006d7ca0(&pointer, locations, angles)`.
const LOCATION_ARRAY_CONSTRUCT: u32 = 0x0098_f5f0;
const ANGLE_ARRAY_CONSTRUCT: u32 = 0x006d_b680;
const LOCATION_ARRAY_DESTRUCT: u32 = 0x006f_2bb0;
const ANGLE_ARRAY_DESTRUCT: u32 = 0x006d_b740;
const LOCATION_ARRAY_ADD: u32 = 0x006c_f1b0;
const ANGLE_ARRAY_ADD: u32 = 0x006d_c320;
const LOS_GRID_MAP_APPLY: u32 = 0x006d_7ca0;
/// `ClampAngle` (cdecl, `float`), in ST0.
const CLAMP_ANGLE: u32 = 0x004b_1480;
/// A random integer between its two arguments (cdecl; the code calls it with
/// `(0, member count)` to pick a member; it goes through `00476c00`).
const RANDOM_INDEX: u32 = 0x0094_4460;
/// The `CombatDialogueManager*` global and `StartDialogue_ov2` (Xbox PDB;
/// `this`, speaker, target, 4, flag, 0, 0 or the variants the callers use).
const DIALOGUE_MANAGER: u32 = 0x011f_1708;
const START_DIALOGUE: u32 = 0x0098_39b0;
/// Operator `delete` (cdecl, one word).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// Operator `new` of the game (cdecl, size).
const OPERATOR_NEW_SMALL: u32 = 0x0040_1000;
/// `memset(ptr, value, size)` (cdecl).
const MEMSET: u32 = 0x0040_3d30;
/// `min(a, b)` of two signed words (cdecl).
const SIGNED_MIN: u32 = 0x004a_8f20;
/// `minf` and `maxf` of two `float`s (cdecl): the smaller and the larger.
const FLOAT_MIN: u32 = 0x0040_ebd0;
/// The larger of two `float`s (cdecl).
const FLOAT_MAX: u32 = 0x0040_4010;
/// `00910b10(timer)`: sets the timer's target time (+4) to 0 (the map names it
/// `TESWeightForm::InitializeDataComponent`, a folded body); `0097f220(timer)`
/// sets the target time to -1.0.
const TIMER_RESET: u32 = 0x0091_0b10;
const TIMER_EXPIRE_NOW: u32 = 0x0097_f220;
/// `CombatTimer::SaveGame` (this, buffer).
const TIMER_SAVE: u32 = 0x009a_5eb0;
/// `BGSSaveGameBuffer::SaveVariableSizedValue` (buffer, value).
const SAVE_SIZED_VALUE: u32 = 0x0086_5f60;
/// `BGSSaveGameBuffer::SaveFormID` (buffer, form, 0).
const SAVE_FORM_ID_PLAIN: u32 = 0x0086_5db0;
/// `PathingLOSGridMap::SaveGame` (grid, buffer).
const LOS_GRID_MAP_SAVE: u32 = 0x006e_0c40;
/// The group strategy's index (`this` = the strategy), `00726070`.
const STRATEGY_INDEX: u32 = 0x0072_6070;
/// The strategy chooser `009900f0(group)` (cdecl, one word).
const CHOOSE_STRATEGY: u32 = 0x0099_00f0;
/// The teleport reference of a door reference (`00568e50(door)`):
/// null when it has none.
const DOOR_TELEPORT_REFERENCE: u32 = 0x0056_8e50;
/// `DoorTeleportData::GetTeleportWorldLocation` (this, out) and
/// `CombatUtilities::CanActorExistInSpace` (cdecl: actor, location).
const TELEPORT_WORLD_LOCATION: u32 = 0x0043_a390;
const CAN_ACTOR_EXIST_IN_SPACE: u32 = 0x009a_96f0;
/// `ExtraDataList`-style getter `004181e0(actor)` the door search uses.
const ACTOR_EXTRA_OBJECT: u32 = 0x0041_81e0;
/// `Actor::IsWaitingOnPath`, and `008b3b90(actor)`.
const ACTOR_IS_WAITING_ON_PATH: u32 = 0x008b_3bf0;
const ACTOR_TEST_008B3B90: u32 = 0x008b_3b90;
/// `30.0` as a `double`.
const THIRTY_DOUBLE: u32 = 0x0101_db88;
/// `70.0` as a `double`.
const SEVENTY_DOUBLE: u32 = 0x0107_3568;
/// `5.0` as a `double`.
const FIVE_DOUBLE: u32 = 0x0102_0998;
/// `2.25` as a `double`.
const TWO_AND_QUARTER_DOUBLE: u32 = 0x0108_d6e8;
/// `128.0` as a `double`, and `4096.0` as a `double`.
const SEARCH_AREA_MARGIN: u32 = 0x0102_e430;
const SEARCH_AREA_MOVE_LIMIT: u32 = 0x0101_7a10;
/// `0.99` as a `double`.
const PATH_PROGRESS_LIMIT: u32 = 0x0106_d508;
/// `0.5f`, and `0.25f`.
const HALF_FLOAT: u32 = 0x0101_6248;
const QUARTER_FLOAT: u32 = 0x0101_622c;
/// `1000000.0f` (the radius the group search asks `fn_00989970` for), and
/// the delay `0x0101712c` the strategy chooser restarts its timer with.
const ONE_MILLION_FLOAT: u32 = 0x0108_cb00;
const STRATEGY_DELAY: u32 = 0x0101_712c;
/// The `bool` global that turns cluster building off, the `float` radius
/// of a cluster and the `float` delay between two builds.
const CLUSTER_DISABLE: u32 = 0x011a_4d34;
const CLUSTER_RADIUS: u32 = 0x011a_4d2c;
const CLUSTER_DELAY: u32 = 0x011a_4d30;
/// The table of 3 x 2 setting pointers `0098b070` reads (radius settings).
const SEARCH_RADIUS_TABLE: u32 = 0x011a_4d38;
/// The group test `0097ef30(group)`: `iSearchCount` is not zero.
const SEARCH_RUNNING: u32 = 0x0097_ef30;
/// The type byte (`[this + 4]`) of a form (`00401170`).
const FORM_TYPE_BYTE: u32 = 0x0040_1170;
/// Frees the elements of a `BSSimpleArray` (`array`, 1).
const ARRAY_CLEAR: u32 = 0x0084_54f0;
/// `RandomFloat(low, high)` (cdecl), in ST0.
const RANDOM_FLOAT: u32 = 0x0047_6b70;
/// `007058c0(state)`: the record the combat state keeps for its target;
/// its word at +4 is that target.
const STATE_TARGET_RECORD: u32 = 0x0070_58c0;
/// `BSSimpleArray<T *>` search (`array`, `&value`, 0, `compare`) and its
/// comparison function.
const CLUSTER_SEARCH: u32 = 0x0071_9b20;
const CLUSTER_COMPARE: u32 = 0x009a_3830;

// Constants of `LoadGame`, `InitLoadGame` and the `BSSimpleArray<T, 1024>`
// instances of this unit (the third part of the unit, from `0098e3d0`).
/// `BGSLoadGameBuffer::LoadVariableSizedValue` (Xbox PDB; `buffer`): a count.
const LOAD_SIZED_VALUE: u32 = 0x0086_4a60;
/// `BGSLoadGameBuffer::LoadFormID` (`buffer`, no stack word): the form ID
/// read; `LoadGame` looks it up with `FORM_LOOKUP` and down-casts it.
const LOAD_FORM_ID_VALUE: u32 = 0x0086_48a0;
/// `CombatTimer::LoadGame` (Xbox PDB; `timer`, `buffer`).
const TIMER_LOAD: u32 = 0x009a_5f20;
/// `PathingLOSGridMap::LoadGame` (Xbox PDB; `grid`, `buffer`).
const LOS_GRID_MAP_LOAD: u32 = 0x006e_0d20;
/// Form lookup `004839c0(form ID)` (cdecl) that the loaders feed to
/// `RT_DYNAMIC_CAST`.
const FORM_LOOKUP: u32 = 0x0048_39c0;
/// `__RTDynamicCast(object, 0, source type, target type, 0)`.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The type descriptors the loaders cast between: the source (the same one
/// `00985930` uses), the target `00985930` and `InitLoadGame` cast to, and
/// the target the door loader casts to.
const CAST_SOURCE_TYPE: u32 = 0x0118_3028;
const CAST_TARGET_TYPE: u32 = 0x0118_46d4;
const CAST_DOOR_TARGET_TYPE: u32 = 0x0118_41cc;
/// `BSSimpleArray<CombatSearchDoor>::SetSize(count, 1)` as `LoadGame` calls it.
const SEARCH_DOOR_RESIZE: u32 = 0x006a_8890;
/// `CombatManager::TargetLocalActor` (Xbox PDB; manager, actor, group).
const MANAGER_TARGET_LOCAL_ACTOR: u32 = 0x0099_2a50;
/// The word at `+0x80` of a process (`004fb070`); `fn_0098eb30` compares it
/// with the group.
const PROCESS_WORD_80: u32 = 0x004f_b070;
/// Table of group strategies, indexed by the byte a save stores.
const STRATEGY_TABLE: u32 = 0x011f_18b4;
/// Size of the `PathingLOSGridMap` object `LoadGame` allocates.
const LOS_GRID_MAP_SIZE: u32 = 0x54;
/// `BSSimpleArray` helpers shared by the instances of this unit:
/// `(array; pointer, count)` destroys elements, `(array; to, from, count)`
/// copies elements, `(available, count)` (cdecl) gives the number of tail
/// elements `RemoveAt` moves, `(from, bytes)` (cdecl) is the reallocation
/// primitive.
const ARRAY_DESTRUCT_ELEMENTS: u32 = 0x0072_ba80;
const ARRAY_COPY_ELEMENTS: u32 = 0x0042_fb60;
const ARRAY_MOVE_COUNT: u32 = 0x0042_f5a0;
const ARRAY_REALLOC_BYTES: u32 = 0x0042_f5d0;
/// The pointer-array shrink test (`this`), the new reserve it wants
/// (`this`), and the release of the old buffer (`this`).
const POINTER_ARRAY_SHOULD_SHRINK: u32 = 0x006f_3170;
const POINTER_ARRAY_SHRUNK_RESERVE: u32 = 0x0086_9600;
const ARRAY_RELEASE_BUFFER: u32 = 0x006a_8500;
/// Growth of an array that is full: whether it is (`this`) and the new
/// reserve (`this`).
const ARRAY_IS_FULL: u32 = 0x0043_8b90;
const ARRAY_GROWN_RESERVE: u32 = 0x009a_3910;
/// `BSSimpleArray<CombatMember>` slot reservation (`this`): the new index.
const MEMBER_ARRAY_RESERVE_SLOT: u32 = 0x0078_81d0;
/// Element `i` of a `BSSimpleArray<CombatTarget>` (`array`, `i`), the
/// instance `fn_0098f580` copies through.
const TARGET_ARRAY_ELEMENT: u32 = 0x0096_a330;
/// `MemoryManager` singleton getter and `GetThreadScrapHeap` (`manager`).
const MEMORY_MANAGER: u32 = 0x0040_1020;
const THREAD_SCRAP_HEAP: u32 = 0x00aa_42e0;
/// Base constructor of the scrap-heap array (`this`) and its initialiser
/// (`this`; two words).
const SCRAP_ARRAY_BASE_CONSTRUCT: u32 = 0x006c_f180;
const SCRAP_ARRAY_INIT: u32 = 0x006d_0140;
/// Initialisers of the `CombatSearchDoor` and `CombatGroupCluster *` arrays
/// (`this`; two words).
const SEARCH_DOOR_ARRAY_INIT: u32 = 0x006b_3f40;
const CLUSTER_POINTER_ARRAY_INIT: u32 = 0x006b_3eb0;
/// Vtables of the array instances of this unit.
const TARGET_ARRAY_VTABLE: u32 = 0x0108_d6f4;
const MEMBER_ARRAY_VTABLE: u32 = 0x0108_d708;
const SEARCH_LOCATION_ARRAY_VTABLE: u32 = 0x0108_d71c;
const SEARCH_DOOR_ARRAY_VTABLE: u32 = 0x0108_d730;
const CLUSTER_POINTER_ARRAY_VTABLE: u32 = 0x0108_d744;
const GROUP_MEMBER_DATA_ARRAY_VTABLE: u32 = 0x0108_d758;
const SCRAP_ARRAY_VTABLE: u32 = 0x0106_d864;

/// The element size and the callees that differ between the
/// `BSSimpleArray<T, 1024>` instances with a `SetSize` / `RemoveAt` in this
/// unit: `grow(array; new reserve, count)`, `construct(array; first, count)`
/// and `move_tail(array; to, from, count)`.
struct ArrayInstance {
    element: u32,
    grow: u32,
    construct: u32,
    move_tail: u32,
}

/// `BSSimpleArray<CombatTarget, 1024>`.
const TARGET_ARRAY_INSTANCE: ArrayInstance = ArrayInstance {
    element: 0x68,
    grow: 0x0098_fa00,
    construct: 0x0098_f840,
    move_tail: 0x0098_f960,
};
/// `BSSimpleArray<CombatMember, 1024>`.
const MEMBER_ARRAY_INSTANCE: ArrayInstance = ArrayInstance {
    element: 0x14,
    grow: 0x009a_3c50,
    construct: 0x0098_fb40,
    move_tail: 0x009a_3b20,
};
/// `BSSimpleArray<CombatSearchLocation, 1024>`.
const SEARCH_LOCATION_ARRAY_INSTANCE: ArrayInstance = ArrayInstance {
    element: 0x1c,
    grow: 0x006b_ff60,
    construct: 0x0098_fc70,
    move_tail: 0x006c_0010,
};

layout! {
    /// `CombatGroupData` (Xbox PDB), 0x38 bytes: only the fields used so far.
    pub struct CombatGroupData: 0x38 {
        /// `pCombatGroup` (Xbox PDB): `CombatGroup*`.
        0x00 pCombatGroup: Ptr<CombatGroup>,
        /// `fMemberHealthPercentage` (Xbox PDB): cached, 0 means not computed.
        0x14 fMemberHealthPercentage: f32,
    }
}

layout! {
    /// `CombatTarget` (Xbox PDB), 0x68 bytes on both builds. The four
    /// `BGSWorldLocation`s (0x10 bytes each) are at +0x08, +0x18, +0x28 and
    /// +0x38; see the `*_LOCATION` constants.
    pub struct CombatTarget: 0x68 {
        /// `pActor` (Xbox PDB): `Actor*`.
        0x00 pActor: Ptr,
        /// `iDetectionLevel` (Xbox PDB).
        0x04 iDetectionLevel: i32,
        /// `sLastSearchNoticed` (Xbox PDB).
        0x48 sLastSearchNoticed: u16,
        /// `sAttackerCount` (Xbox PDB).
        0x4a sAttackerCount: u16,
        /// `fLastSeenTimeStamp` (Xbox PDB): a `CombatTimeStamp`.
        0x4c fLastSeenTimeStamp: f32,
        /// `fLastDetectedTimeStamp` (Xbox PDB).
        0x50 fLastDetectedTimeStamp: f32,
        /// `fDetectionLevelUpdateTimeStamp` (Xbox PDB).
        0x54 fDetectionLevelUpdateTimeStamp: f32,
        /// `fLastNoticedTimeStamp` (Xbox PDB).
        0x58 fLastNoticedTimeStamp: f32,
        /// `fDetectionEventTimeStamp` (Xbox PDB).
        0x5c fDetectionEventTimeStamp: f32,
        /// `fLastAttackedMemberTimeStamp` (Xbox PDB).
        0x60 fLastAttackedMemberTimeStamp: f32,
        /// `cMemberLOSCount` (Xbox PDB).
        0x64 cMemberLOSCount: u8,
        /// `cMember360LOSCount` (Xbox PDB).
        0x65 cMember360LOSCount: u8,
    }

    /// `CombatMember` (Xbox PDB), 0x14 bytes.
    pub struct CombatMember: 0x14 {
        /// `pActor` (Xbox PDB): `Actor*`.
        0x00 pActor: Ptr,
        /// `iGroupStrategyAssignment` (Xbox PDB).
        0x04 iGroupStrategyAssignment: u32,
        /// `fDamagePerSecond` (Xbox PDB).
        0x08 fDamagePerSecond: f32,
        /// `fCombatStrength` (Xbox PDB).
        0x0c fCombatStrength: f32,
        /// `pCluster` (Xbox PDB): `CombatGroupCluster*`.
        0x10 pCluster: Ptr,
    }

    /// `CombatSearchLocation` (Xbox PDB), 0x1c bytes: a `BGSWorldLocation`
    /// at +0 followed by these fields.
    pub struct CombatSearchLocation: 0x1c {
        /// `fTimeStamp` (Xbox PDB): a `CombatTimeStamp`.
        0x10 fTimeStamp: f32,
        /// `iTargetID` (Xbox PDB).
        0x14 iTargetID: u32,
        /// `fScore` (Xbox PDB).
        0x18 fScore: f32,
    }

    /// `CombatGroup` (Xbox PDB), 0x15c bytes, every scalar field the
    /// constructor sets. Embedded `CombatTimer`s, `BGSWorldLocation`s and
    /// `BSSimpleArray`s are addressed with the offset constants of
    /// `impl CombatGroup`.
    pub struct CombatGroup: 0x15c {
        /// `iGroupNum` (Xbox PDB).
        0x00 iGroupNum: u32,
        /// `iGroupID` (Xbox PDB).
        0x04 iGroupID: u32,
        /// `pGroupStrategy` (Xbox PDB): `CombatGroupStrategy*`.
        0x28 pGroupStrategy: Ptr,
        /// `bStrategyForced` (Xbox PDB).
        0x2c bStrategyForced: bool,
        /// `iLastGroupStrategyChosenIndex` (Xbox PDB).
        0x40 iLastGroupStrategyChosenIndex: u32,
        /// `fLastGroupStrategyChosenTimeStamp` (Xbox PDB).
        0x44 fLastGroupStrategyChosenTimeStamp: f32,
        /// `fMemberCombatStrength` (Xbox PDB).
        0xc0 fMemberCombatStrength: f32,
        /// `fAverageMemberDamagePerSecond` (Xbox PDB).
        0xc4 fAverageMemberDamagePerSecond: f32,
        /// `fTargetCombatStrength` (Xbox PDB).
        0xc8 fTargetCombatStrength: f32,
        /// `fAverageTargetCombatStrength` (Xbox PDB).
        0xcc fAverageTargetCombatStrength: f32,
        /// `iSearchCount` (Xbox PDB).
        0xd0 iSearchCount: u32,
        /// `spPathingLOSGridMap` (Xbox PDB): an `NiPointer`.
        0xd4 spPathingLOSGridMap: Ptr,
        /// `fSearchStartedTimeStamp` (Xbox PDB).
        0xe8 fSearchStartedTimeStamp: f32,
        /// `pSearchingMember` (Xbox PDB): `Actor*`.
        0xec pSearchingMember: Ptr,
        /// `fSearchRadius` (Xbox PDB).
        0x10c fSearchRadius: f32,
        /// `spSearchDebugGeometry` (Xbox PDB): `NiPointer<NiNode>`.
        0x130 spSearchDebugGeometry: Ptr,
        /// `bUpdateSearchDebugGeometry` (Xbox PDB).
        0x134 bUpdateSearchDebugGeometry: bool,
        /// `iInitializedMemberCount` (Xbox PDB).
        0x148 iInitializedMemberCount: u32,
        /// `iFleeingMemberCount` (Xbox PDB).
        0x14c iFleeingMemberCount: u32,
        /// `iNonFleeingMemberCount` (Xbox PDB).
        0x150 iNonFleeingMemberCount: u32,
        /// `cCombatMusicState` (Xbox PDB); -1 until the music timer decides.
        0x154 cCombatMusicState: i8,
        /// `spDebugGeometry` (Xbox PDB): `NiPointer<NiNode>`.
        0x158 spDebugGeometry: Ptr,
    }
}

impl CombatTarget {
    /// `LastNoticedLocation` (a `BGSWorldLocation`).
    pub const LAST_NOTICED_LOCATION: u32 = 0x08;
    /// `LastDetectedLocation`.
    pub const LAST_DETECTED_LOCATION: u32 = 0x18;
    /// `LastSeenLocation`.
    pub const LAST_SEEN_LOCATION: u32 = 0x28;
    /// `LastAttackedMemberLocation`.
    pub const LAST_ATTACKED_MEMBER_LOCATION: u32 = 0x38;
}

impl CombatGroup {
    /// `TargetArray` (`BSSimpleArray<CombatTarget, 1024>`).
    pub const TARGET_ARRAY: u32 = 0x08;
    /// `MemberArray` (`BSSimpleArray<CombatMember, 1024>`).
    pub const MEMBER_ARRAY: u32 = 0x18;
    /// `ChooseStrategyTimer`, `UpdateStrategyTimer` (`CombatTimer`s).
    pub const CHOOSE_STRATEGY_TIMER: u32 = 0x30;
    pub const UPDATE_STRATEGY_TIMER: u32 = 0x38;
    /// `AvoidThreatDialogueTimer`, `CombatStrengthTimer`.
    pub const AVOID_THREAT_DIALOGUE_TIMER: u32 = 0x48;
    pub const COMBAT_STRENGTH_TIMER: u32 = 0x50;
    /// `DetectionDialogueTimers`: ten `CombatTimer`s.
    pub const DETECTION_DIALOGUE_TIMERS: u32 = 0x58;
    /// `TargetUpdateTimer`, `ClusterUpdateTimer`, `CombatMusicUpdateTimer`.
    pub const TARGET_UPDATE_TIMER: u32 = 0xa8;
    pub const CLUSTER_UPDATE_TIMER: u32 = 0xb0;
    pub const COMBAT_MUSIC_UPDATE_TIMER: u32 = 0xb8;
    /// `SearchUpdateTimer`, `SearchAreaUpdateTimer`.
    pub const SEARCH_UPDATE_TIMER: u32 = 0xd8;
    pub const SEARCH_AREA_UPDATE_TIMER: u32 = 0xe0;
    /// `SearchCenter` (`BGSWorldLocation`).
    pub const SEARCH_CENTER: u32 = 0xf0;
    /// `SearchFocalPoint` (`NiPoint3`).
    pub const SEARCH_FOCAL_POINT: u32 = 0x100;
    /// `SearchLocations` (`BSSimpleArray<CombatSearchLocation, 1024>`).
    pub const SEARCH_LOCATIONS: u32 = 0x110;
    /// `SearchTeleportDoors` (`BSSimpleArray<CombatSearchDoor, 1024>`).
    pub const SEARCH_TELEPORT_DOORS: u32 = 0x120;
    /// `ClusterArray` (`BSSimpleArray<CombatGroupCluster *, 1024>`).
    pub const CLUSTER_ARRAY: u32 = 0x138;
}

/// The singleton `CombatManager*` (read each time, as the game does).
fn combat_manager(e: &Engine) -> u32 {
    e.global::<u32>(COMBAT_MANAGER)
}

fn time_stamp(e: &mut Engine, stamp: Ptr) -> f32 {
    e.call(TIME_STAMP_VALUE, &args![stamp]).f32()
}

/// Copies one `BGSWorldLocation` (four words, as the game does).
fn copy_location(e: &mut Engine, to: Ptr, from: Ptr) {
    for word in 0..4 {
        let value = e.mem.u32(from.addr() + 4 * word);
        e.mem.set_u32(to.addr() + 4 * word, value);
    }
}

/// Reads the byte a `bool` setting accessor points at.
fn bool_setting(e: &mut Engine, setting: u32) -> bool {
    let value = e.call(SETTING_BOOL_VALUE, &args![setting]).u32();
    e.mem.u8(value) != 0
}

/// Reads the `float` a `float` setting accessor points at.
fn float_setting(e: &mut Engine, setting: u32) -> f32 {
    let value = e.call(SETTING_FLOAT_VALUE, &args![setting]).u32();
    e.mem.f32(value)
}

/// Age of a `CombatTimeStamp` (`00435e00`): game time now minus the stamp.
fn time_stamp_age(e: &mut Engine, stamp: Ptr) -> f32 {
    e.call(TIME_STAMP_AGE, &args![stamp]).f32()
}

/// Reads the `int` an integer setting accessor points at.
fn int_setting(e: &mut Engine, setting: u32) -> u32 {
    let value = e.call(SETTING_INT_VALUE, &args![setting]).u32();
    e.mem.u32(value)
}

/// Copies one `NiPoint3` (three words) from `from` to `to`.
fn copy_point(e: &mut Engine, to: Ptr, from: u32) {
    for word in 0..3 {
        let value = e.mem.u32(from + 4 * word);
        e.mem.set_u32(to.addr() + 4 * word, value);
    }
}

// Translated from 0069cfe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies a `BSSimpleArray<CombatSearchDoor, 1024>` (12-byte elements)
/// into `other`: sizes `other` to this array's size, then copies element by
/// element. The engine map names it
/// `BSSimpleArray<CombatSearchDoor_1024>::SetSize` (Xbox PDB), but the body
/// is the copy.
pub fn fn_0069cfe0(e: &mut Engine, this: Ptr<BSSimpleArray>, other: Ptr<BSSimpleArray>) {
    let size = e.get(this, BSSimpleArray::iSize);
    e.call(SEARCH_DOOR_ARRAY_SET_SIZE, &args![other, size, 1u32]);
    let mut index = 0;
    while index < e.get(this, BSSimpleArray::iSize) {
        let from = e.get(this, BSSimpleArray::pBuffer) + index * 0xc;
        let to = e.call(SEARCH_DOOR_AT, &args![other, index]).u32();
        for word in 0..3 {
            let value = e.mem.u32(from + 4 * word);
            e.mem.set_u32(to + 4 * word, value);
        }
        index += 1;
    }
}

// Translated from 00985410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatTarget` constructor (the map has no name; identified by the
/// layout): the actor, detection level 0, four default locations, the
/// search-noticed count, attacker count 0, the time stamps (-FLT_MAX for
/// last seen / last detected / last attacked member, the current game time
/// for last noticed, 0 for the other two), both line-of-sight counts 0, and
/// `LastNoticedLocation` set to the actor's current world location.
pub fn fn_00985410(
    e: &mut Engine,
    this: Ptr<CombatTarget>,
    actor: Ptr,
    search_noticed: u16,
) -> Ptr<CombatTarget> {
    e.set(this, CombatTarget::pActor, actor);
    e.set(this, CombatTarget::iDetectionLevel, 0);
    for location in [
        CombatTarget::LAST_NOTICED_LOCATION,
        CombatTarget::LAST_DETECTED_LOCATION,
        CombatTarget::LAST_SEEN_LOCATION,
        CombatTarget::LAST_ATTACKED_MEMBER_LOCATION,
    ] {
        e.call(WORLD_LOCATION_CONSTRUCT, &args![this.byte_add(location)]);
    }
    e.set(this, CombatTarget::sLastSearchNoticed, search_noticed);
    e.set(this, CombatTarget::sAttackerCount, 0);
    let lowest = -e.global::<f32>(MAX_FLOAT);
    e.call(
        TIME_STAMP_SET,
        &args![this.byte_add(CombatTarget::fLastSeenTimeStamp.off), lowest],
    );
    e.call(
        TIME_STAMP_SET,
        &args![
            this.byte_add(CombatTarget::fLastDetectedTimeStamp.off),
            lowest
        ],
    );
    e.call(
        TIME_STAMP_ZERO,
        &args![this.byte_add(CombatTarget::fDetectionLevelUpdateTimeStamp.off)],
    );
    let now = e.call(GAME_TIME_NOW, &args![]).f32();
    e.call(
        TIME_STAMP_SET,
        &args![this.byte_add(CombatTarget::fLastNoticedTimeStamp.off), now],
    );
    e.call(
        TIME_STAMP_ZERO,
        &args![this.byte_add(CombatTarget::fDetectionEventTimeStamp.off)],
    );
    e.call(
        TIME_STAMP_SET,
        &args![
            this.byte_add(CombatTarget::fLastAttackedMemberTimeStamp.off),
            lowest
        ],
    );
    e.set(this, CombatTarget::cMemberLOSCount, 0);
    e.set(this, CombatTarget::cMember360LOSCount, 0);
    e.with_stack(0x10, |e, scratch| {
        let world = e
            .call(GET_WORLD_LOCATION, &args![actor, scratch])
            .ptr::<()>();
        copy_location(e, this.byte_add(CombatTarget::LAST_NOTICED_LOCATION), world);
    });
    this
}

// Translated from 00985520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Merges `other` into this `CombatTarget`: the higher detection level; the
/// newer of each pair of time stamps (last detected with its location, last
/// seen with its location, detection level update, last noticed, last
/// attacked member with its location); and the sums of the attacker count
/// and of the two line-of-sight counts.
pub fn fn_00985520(e: &mut Engine, this: Ptr<CombatTarget>, other: Ptr<CombatTarget>) {
    if e.get(this, CombatTarget::iDetectionLevel) < e.get(other, CombatTarget::iDetectionLevel) {
        let level = e.get(other, CombatTarget::iDetectionLevel);
        e.set(this, CombatTarget::iDetectionLevel, level);
    }
    // (time stamp offset, location taken along with it)
    let pairs = [
        (
            CombatTarget::fLastDetectedTimeStamp.off,
            Some(CombatTarget::LAST_DETECTED_LOCATION),
        ),
        (
            CombatTarget::fLastSeenTimeStamp.off,
            Some(CombatTarget::LAST_SEEN_LOCATION),
        ),
        (CombatTarget::fDetectionLevelUpdateTimeStamp.off, None),
        (CombatTarget::fLastNoticedTimeStamp.off, None),
        (
            CombatTarget::fLastAttackedMemberTimeStamp.off,
            Some(CombatTarget::LAST_ATTACKED_MEMBER_LOCATION),
        ),
    ];
    for (stamp, location) in pairs {
        let mine = time_stamp(e, this.byte_add(stamp));
        let theirs = time_stamp(e, other.byte_add(stamp));
        if theirs > mine {
            let value = e.mem.u32(other.addr() + stamp);
            e.mem.set_u32(this.addr() + stamp, value);
            if let Some(location) = location {
                copy_location(e, this.byte_add(location), other.byte_add(location));
            }
        }
    }
    let attackers = e
        .get(this, CombatTarget::sAttackerCount)
        .wrapping_add(e.get(other, CombatTarget::sAttackerCount));
    e.set(this, CombatTarget::sAttackerCount, attackers);
    let los = e
        .get(this, CombatTarget::cMemberLOSCount)
        .wrapping_add(e.get(other, CombatTarget::cMemberLOSCount));
    e.set(this, CombatTarget::cMemberLOSCount, los);
    let los_360 = e
        .get(this, CombatTarget::cMember360LOSCount)
        .wrapping_add(e.get(other, CombatTarget::cMember360LOSCount));
    e.set(this, CombatTarget::cMember360LOSCount, los_360);
}

// Translated from 009856e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatTarget::SaveGame` (Xbox PDB): writes the actor as a form ID, the
/// detection level (4 bytes), the search-noticed and attacker counts (2
/// bytes each), the two line-of-sight counts, four locations and six time
/// stamps. The second stack word is never read.
pub fn combat_target_save_game(
    e: &mut Engine,
    this: Ptr<CombatTarget>,
    buffer: Ptr,
    _unused_2: u32,
) {
    let actor = e.get(this, CombatTarget::pActor);
    e.call(SAVE_FORM_ID, &args![buffer, actor, 0u32]);
    for (offset, size) in [(0x04u32, 4u32), (0x48, 2), (0x4a, 2), (0x64, 1), (0x65, 1)] {
        e.call(
            SAVE_BYTES,
            &args![buffer, this.byte_add(offset), size, 0u32],
        );
    }
    for location in [0x08u32, 0x18, 0x28, 0x38] {
        e.call(WORLD_LOCATION_SAVE, &args![this.byte_add(location), buffer]);
    }
    for stamp in [0x50u32, 0x54, 0x58, 0x4c, 0x5c, 0x60] {
        e.call(TIME_STAMP_SAVE, &args![this.byte_add(stamp), buffer]);
    }
}

// Translated from 00985800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatTarget::LoadGame` (Xbox PDB): reads what `SaveGame` wrote. The
/// fourth location (`LastAttackedMemberLocation`) is only in saves of
/// version 14 or later, the last time stamp (`LastAttackedMemberTimeStamp`)
/// in version 13 or later (the version is the result of the buffer's
/// virtual method at slot 0). The second stack word is never read.
pub fn combat_target_load_game(
    e: &mut Engine,
    this: Ptr<CombatTarget>,
    buffer: Ptr,
    _unused_2: u32,
) {
    e.call(LOAD_FORM_ID, &args![buffer, this]);
    for (offset, size) in [(0x04u32, 4u32), (0x48, 2), (0x4a, 2), (0x64, 1), (0x65, 1)] {
        e.call(LOAD_BYTES, &args![buffer, this.byte_add(offset), size]);
    }
    for location in [0x08u32, 0x18, 0x28] {
        e.call(WORLD_LOCATION_LOAD, &args![this.byte_add(location), buffer]);
    }
    if e.vcall(buffer.addr(), 0, &args![]).u8() >= 0x0e {
        e.call(WORLD_LOCATION_LOAD, &args![this.byte_add(0x38), buffer]);
    }
    for stamp in [0x50u32, 0x54, 0x58, 0x4c, 0x5c] {
        e.call(TIME_STAMP_LOAD, &args![this.byte_add(stamp), buffer]);
    }
    if e.vcall(buffer.addr(), 0, &args![]).u8() >= 0x0d {
        e.call(TIME_STAMP_LOAD, &args![this.byte_add(0x60), buffer]);
    }
}

// Translated from 00985930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Replaces the value in the word at `this` by an `Actor*`: zero stays
/// zero; otherwise the word is looked up with `004839c0` and the result is
/// down-cast with `__RTDynamicCast` (`TESForm` type descriptor `01183028`
/// to the one at `011846d4`, no reference). Both stack words are never read
/// (the shape of a "resolve references" callback).
pub fn fn_00985930(e: &mut Engine, this: Ptr, _unused_1: u32, _unused_2: u32) {
    let value = e.mem.u32(this.addr());
    let resolved = if value == 0 {
        0
    } else {
        let form = e.call(0x0048_39c0, &args![value]).u32();
        e.call(
            0x00ec_43fb,
            &args![form, 0u32, 0x0118_3028u32, 0x0118_46d4u32, 0u32],
        )
        .u32()
    };
    e.mem.set_u32(this.addr(), resolved);
}

// Translated from 00985980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the damage per second, combat strength and cluster of another
/// `CombatMember` into this one (not the actor or the strategy assignment).
pub fn fn_00985980(e: &mut Engine, this: Ptr<CombatMember>, other: Ptr<CombatMember>) {
    let damage = e.get(other, CombatMember::fDamagePerSecond);
    e.set(this, CombatMember::fDamagePerSecond, damage);
    let strength = e.get(other, CombatMember::fCombatStrength);
    e.set(this, CombatMember::fCombatStrength, strength);
    let cluster = e.get(other, CombatMember::pCluster);
    e.set(this, CombatMember::pCluster, cluster);
}

// Translated from 009859c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatMember::SaveGame` (Xbox PDB): the actor as a form ID, the three
/// 4-byte fields after it, and the index of the member's cluster in the
/// group's cluster array (`0xff` when it has none).
pub fn combat_member_save_game(
    e: &mut Engine,
    this: Ptr<CombatMember>,
    buffer: Ptr,
    group: Ptr<CombatGroup>,
) {
    let actor = e.get(this, CombatMember::pActor);
    e.call(SAVE_FORM_ID, &args![buffer, actor, 0u32]);
    for offset in [0x04u32, 0x08, 0x0c] {
        e.call(
            SAVE_BYTES,
            &args![buffer, this.byte_add(offset), 4u32, 0u32],
        );
    }
    let mut cluster_index = 0xffu8;
    let cluster = e.get(this, CombatMember::pCluster);
    if !cluster.is_null() {
        cluster_index = e.call(0x0098_de30, &args![group, cluster]).u8();
    }
    e.with_stack(4, |e, byte| {
        e.mem.set_u8(byte.addr(), cluster_index);
        e.call(SAVE_BYTES, &args![buffer, byte, 1u32, 0u32]);
    });
}

// Translated from 00985a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatMember::LoadGame` (Xbox PDB): reads what `SaveGame` wrote and
/// turns the cluster index back into the group's cluster (`0098de60`; null
/// for `0xff`).
pub fn combat_member_load_game(
    e: &mut Engine,
    this: Ptr<CombatMember>,
    buffer: Ptr,
    group: Ptr<CombatGroup>,
) {
    e.call(LOAD_FORM_ID, &args![buffer, this]);
    for offset in [0x04u32, 0x08, 0x0c] {
        e.call(LOAD_BYTES, &args![buffer, this.byte_add(offset), 4u32]);
    }
    let cluster_index = e.with_stack(4, |e, byte| {
        e.mem.set_u8(byte.addr(), 0xff);
        e.call(LOAD_BYTES, &args![buffer, byte, 1u32]);
        e.mem.i8(byte.addr())
    });
    let cluster = if cluster_index == -1 {
        Ptr::NULL
    } else {
        e.call(0x0098_de60, &args![group, cluster_index as u8])
            .ptr::<()>()
    };
    e.set(this, CombatMember::pCluster, cluster);
}

// Translated from 00985ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroupCluster` save: its `Center` location as 0x10 raw bytes, then
/// the member count (4 bytes). The second stack word is never read.
pub fn fn_00985ae0(e: &mut Engine, this: Ptr, buffer: Ptr, _unused_2: u32) {
    e.call(SAVE_BYTES, &args![buffer, this, 0x10u32, 0u32]);
    e.call(SAVE_BYTES, &args![buffer, this.byte_add(0x10), 4u32, 0u32]);
}

// Translated from 00985b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroupCluster` load: reads what `fn_00985ae0` wrote. The second
/// stack word is never read.
pub fn fn_00985b10(e: &mut Engine, this: Ptr, buffer: Ptr, _unused_2: u32) {
    e.call(LOAD_BYTES, &args![buffer, this, 0x10u32]);
    e.call(LOAD_BYTES, &args![buffer, this.byte_add(0x10), 4u32]);
}

// Translated from 00985b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatSearchLocation` score seen from `location`: the time left of the
/// entry (`fn_00985bb0`) minus the squared distance to `location` (capped
/// by `0040ebd0` at the `float` at `0108d5a0`) divided by the `double` at
/// `0108d598`, plus the `double` at `01020758` (10) when the entry has a
/// target ID.
pub fn fn_00985b40(e: &mut Engine, this: Ptr<CombatSearchLocation>, location: Ptr) -> f32 {
    let remaining = fn_00985bb0(e, this);
    let distance = e.call(DISTANCE_SQUARED, &args![location, this]).f32();
    let cap: f32 = e.global(0x0108_d5a0);
    let distance = e.call(0x0040_ebd0, &args![distance, cap]).f32();
    let divisor: f64 = e.global(0x0108_d598);
    let mut score = (remaining as f64 - distance as f64 / divisor) as f32;
    if e.get(this, CombatSearchLocation::iTargetID) != 0 {
        let bonus: f64 = e.global(0x0102_0758);
        score = (score as f64 + bonus) as f32;
    }
    score
}

// Translated from 00985bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatSearchLocation`: the time left of the entry, `fScore` minus the
/// age of its time stamp (`fScore` holds a duration here).
pub fn fn_00985bb0(e: &mut Engine, this: Ptr<CombatSearchLocation>) -> f32 {
    let age = e
        .call(
            TIME_STAMP_AGE,
            &args![this.byte_add(CombatSearchLocation::fTimeStamp.off)],
        )
        .f32();
    (e.get(this, CombatSearchLocation::fScore) as f64 - age as f64) as f32
}

// Translated from 00985be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatSearchLocation` debug colour, written to `result` (an `NiColorA`)
/// and returned, by the time left (`fn_00985bb0`): (1, 0, 1) above the
/// `double` at `0101e2c0` (50), (0, 0, 1) above the one at `0101db88` (30),
/// otherwise (1, 1, 0); the alpha is the `float` at `01016248`.
pub fn fn_00985be0(e: &mut Engine, this: Ptr<CombatSearchLocation>, result: Ptr) -> Ptr {
    let alpha: f32 = e.global(0x0101_6248);
    e.with_stack(0x20, |e, scratch| {
        // The game first builds an all-zero colour that is overwritten.
        e.call(0x0041_4430, &args![scratch, 0.0f32, 0.0f32, 0.0f32, 0.0f32]);
        let remaining = fn_00985bb0(e, this) as f64;
        let upper: f64 = e.global(0x0101_e2c0);
        let lower: f64 = e.global(0x0101_db88);
        let chosen = scratch.byte_add(0x10);
        let (red, green, blue) = if remaining > upper {
            (1.0f32, 0.0f32, 1.0f32)
        } else if remaining > lower {
            (0.0, 0.0, 1.0)
        } else {
            (1.0, 1.0, 0.0)
        };
        e.call(0x0041_4430, &args![chosen, red, green, blue, alpha]);
        for word in 0..4 {
            let value = e.mem.u32(chosen.addr() + 4 * word);
            e.mem.set_u32(result.addr() + 4 * word, value);
        }
    });
    result
}

// Translated from 00985d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup` constructor (the map has no name): group number 0, group
/// ID `group_id`, empty target and member arrays, no strategy, all timers
/// zeroed, strength figures -1, no search state, no clusters, music state
/// -1 (undecided). The compiler's unwinding frame is not translated.
pub fn fn_00985d10(e: &mut Engine, this: Ptr<CombatGroup>, group_id: u32) -> Ptr<CombatGroup> {
    e.set(this, CombatGroup::iGroupNum, 0);
    e.set(this, CombatGroup::iGroupID, group_id);
    e.call(
        0x0098_ed30,
        &args![this.byte_add(CombatGroup::TARGET_ARRAY)],
    );
    e.call(
        0x0098_eff0,
        &args![this.byte_add(CombatGroup::MEMBER_ARRAY)],
    );
    e.set(this, CombatGroup::pGroupStrategy, Ptr::NULL);
    e.set(this, CombatGroup::bStrategyForced, false);
    for timer in [
        CombatGroup::CHOOSE_STRATEGY_TIMER,
        CombatGroup::UPDATE_STRATEGY_TIMER,
    ] {
        e.call(TIMER_CONSTRUCT, &args![this.byte_add(timer)]);
    }
    e.set(this, CombatGroup::iLastGroupStrategyChosenIndex, 0);
    e.call(
        TIME_STAMP_ZERO,
        &args![this.byte_add(CombatGroup::fLastGroupStrategyChosenTimeStamp.off)],
    );
    for timer in [
        CombatGroup::AVOID_THREAT_DIALOGUE_TIMER,
        CombatGroup::COMBAT_STRENGTH_TIMER,
    ] {
        e.call(TIMER_CONSTRUCT, &args![this.byte_add(timer)]);
    }
    // `_vector_constructor_iterator_(base, size, count, constructor)`.
    e.call(
        0x0040_1050,
        &args![
            this.byte_add(CombatGroup::DETECTION_DIALOGUE_TIMERS),
            8u32,
            10u32,
            TIMER_CONSTRUCT
        ],
    );
    for timer in [
        CombatGroup::TARGET_UPDATE_TIMER,
        CombatGroup::CLUSTER_UPDATE_TIMER,
        CombatGroup::COMBAT_MUSIC_UPDATE_TIMER,
    ] {
        e.call(TIMER_CONSTRUCT, &args![this.byte_add(timer)]);
    }
    let unset: f32 = e.global(MINUS_ONE);
    e.set(this, CombatGroup::fMemberCombatStrength, unset);
    e.set(this, CombatGroup::fAverageMemberDamagePerSecond, unset);
    e.set(this, CombatGroup::fTargetCombatStrength, unset);
    e.set(this, CombatGroup::fAverageTargetCombatStrength, unset);
    e.set(this, CombatGroup::iSearchCount, 0);
    e.call(
        0x0063_3c90,
        &args![this.byte_add(CombatGroup::spPathingLOSGridMap.off), 0u32],
    );
    for timer in [
        CombatGroup::SEARCH_UPDATE_TIMER,
        CombatGroup::SEARCH_AREA_UPDATE_TIMER,
    ] {
        e.call(TIMER_CONSTRUCT, &args![this.byte_add(timer)]);
    }
    let lowest = -e.global::<f32>(MAX_FLOAT);
    e.call(
        TIME_STAMP_SET,
        &args![
            this.byte_add(CombatGroup::fSearchStartedTimeStamp.off),
            lowest
        ],
    );
    e.set(this, CombatGroup::pSearchingMember, Ptr::NULL);
    e.call(
        WORLD_LOCATION_CONSTRUCT,
        &args![this.byte_add(CombatGroup::SEARCH_CENTER)],
    );
    for word in 0..3 {
        let value = e.global::<u32>(0x011f_426c + 4 * word);
        e.mem.set_u32(
            this.addr() + CombatGroup::SEARCH_FOCAL_POINT + 4 * word,
            value,
        );
    }
    e.set(this, CombatGroup::fSearchRadius, 0.0);
    e.call(
        0x0098_f2a0,
        &args![this.byte_add(CombatGroup::SEARCH_LOCATIONS)],
    );
    e.call(
        0x0098_f4c0,
        &args![this.byte_add(CombatGroup::SEARCH_TELEPORT_DOORS)],
    );
    e.call(
        0x0063_3c90,
        &args![this.byte_add(CombatGroup::spSearchDebugGeometry.off), 0u32],
    );
    e.set(this, CombatGroup::bUpdateSearchDebugGeometry, false);
    e.call(
        0x0098_f510,
        &args![this.byte_add(CombatGroup::CLUSTER_ARRAY)],
    );
    e.set(this, CombatGroup::iInitializedMemberCount, 0);
    e.set(this, CombatGroup::iFleeingMemberCount, 0);
    e.set(this, CombatGroup::iNonFleeingMemberCount, 0);
    e.set(this, CombatGroup::cCombatMusicState, -1);
    e.call(
        0x0063_3c90,
        &args![this.byte_add(CombatGroup::spDebugGeometry.off), 0u32],
    );
    this
}

// Translated from 00985f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup` destructor (the map has no name): takes every target and
/// every member out of the combat manager, destroys the clusters (`00401030`
/// per cluster), calls `009878b0(this, 1)` (a clean-up the map misnames
/// `PathingDebugGeometry::MakeSmoothingData`), then destroys the members in
/// reverse order: the debug geometry pointer, the cluster array, the search
/// debug geometry pointer, the teleport door array, the search location
/// array, the line-of-sight grid pointer, the member array and the target
/// array. Exception unwinding states are not translated.
pub fn fn_00985f90(e: &mut Engine, this: Ptr<CombatGroup>) {
    let targets = e.call(TARGET_COUNT, &args![this]).u32();
    for index in 0..targets {
        let actor = e.call(TARGET_ACTOR_AT, &args![this, index]).u32();
        let manager = combat_manager(e);
        e.call(MANAGER_REMOVE_TARGET, &args![manager, this, actor]);
    }
    let members = e.call(MEMBER_COUNT, &args![this]).u32();
    for index in 0..members {
        let actor = e.call(MEMBER_ACTOR_AT, &args![this, index]).u32();
        let manager = combat_manager(e);
        e.call(MANAGER_REMOVE_MEMBER, &args![manager, this, actor]);
    }
    let clusters = this.byte_add(CombatGroup::CLUSTER_ARRAY);
    let mut index = 0;
    while index < e.call(ARRAY_SIZE, &args![clusters]).u32() {
        let slot = e.call(CLUSTER_AT, &args![clusters, index]).u32();
        let cluster = e.mem.u32(slot);
        e.call(0x0040_1030, &args![cluster]);
        index += 1;
    }
    e.call(0x0098_78b0, &args![this, 1u32]);
    e.call(
        0x0045_cec0,
        &args![this.byte_add(CombatGroup::spDebugGeometry.off)],
    );
    e.call(0x0098_f540, &args![clusters]);
    e.call(
        0x0045_cec0,
        &args![this.byte_add(CombatGroup::spSearchDebugGeometry.off)],
    );
    e.call(
        0x0098_f4f0,
        &args![this.byte_add(CombatGroup::SEARCH_TELEPORT_DOORS)],
    );
    e.call(
        0x0098_f2d0,
        &args![this.byte_add(CombatGroup::SEARCH_LOCATIONS)],
    );
    e.call(
        0x0045_cec0,
        &args![this.byte_add(CombatGroup::spPathingLOSGridMap.off)],
    );
    e.call(
        0x0098_f020,
        &args![this.byte_add(CombatGroup::MEMBER_ARRAY)],
    );
    e.call(
        0x0098_ed60,
        &args![this.byte_add(CombatGroup::TARGET_ARRAY)],
    );
}

// Translated from 00986150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup::Update` (Xbox PDB): refreshes the counts (`UpdateCounts`);
/// when the target update timer (`+0xa8`) has expired, runs `0098cf70` and
/// `0098d030` and restarts it (delay: the `float` at `010162c0`); runs
/// `0098d1a0` and `0098a580`; when the combat strength timer (`+0x50`) has
/// expired, runs `UpdateCombatStrength` and restarts it with the setting at
/// `011cfe84`; with more than one member runs `0098d2c0`; and decides the
/// combat music state (`+0x154`, -1 until decided): while the music timer
/// (`+0xb8`) is not running and `0097ef30` says no search was started, it is
/// armed with the setting at `011ceb44`; once it expires the state becomes 1
/// when member strength over target strength lies strictly between the
/// settings at `011cee6c` and `011cf944` (and the target strength is
/// positive), else 0.
pub fn combat_group_update(e: &mut Engine, this: Ptr<CombatGroup>) {
    e.call(0x0098_ce80, &args![this]);
    let target_timer = this.byte_add(CombatGroup::TARGET_UPDATE_TIMER);
    if e.call(TIMER_EXPIRED, &args![target_timer]).bool() {
        e.call(0x0098_cf70, &args![this]);
        e.call(0x0098_d030, &args![this]);
        let delay: f32 = e.global(0x0101_62c0);
        e.call(TIMER_START, &args![target_timer, delay]);
    }
    e.call(0x0098_d1a0, &args![this]);
    e.call(0x0098_a580, &args![this]);
    let strength_timer = this.byte_add(CombatGroup::COMBAT_STRENGTH_TIMER);
    if e.call(TIMER_EXPIRED, &args![strength_timer]).bool() {
        e.call(0x0098_c870, &args![this]);
        let delay = float_setting(e, 0x011c_fe84);
        e.call(TIMER_START, &args![strength_timer, delay]);
    }
    if e.call(MEMBER_COUNT, &args![this]).u32() > 1 {
        e.call(0x0098_d2c0, &args![this]);
    }
    if e.get(this, CombatGroup::cCombatMusicState) != -1 {
        return;
    }
    let music_timer = this.byte_add(CombatGroup::COMBAT_MUSIC_UPDATE_TIMER);
    if !e.call(TIMER_RUNNING, &args![music_timer]).bool() {
        if !e.call(0x0097_ef30, &args![this]).bool() {
            let delay = float_setting(e, 0x011c_eb44);
            e.call(TIMER_START, &args![music_timer, delay]);
        }
    } else if e.call(TIMER_EXPIRED, &args![music_timer]).bool() {
        e.set(this, CombatGroup::cCombatMusicState, 0);
        let target_strength = e.get(this, CombatGroup::fTargetCombatStrength);
        if 0.0 < target_strength as f64 {
            let ratio = e.get(this, CombatGroup::fMemberCombatStrength) / target_strength;
            let low = float_setting(e, 0x011c_ee6c);
            if low < ratio {
                let high = float_setting(e, 0x011c_f944);
                if ratio < high {
                    e.set(this, CombatGroup::cCombatMusicState, 1);
                }
            }
        }
    }
}

// Translated from 009862e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the group's strategy (the map has no name). A null strategy only
/// clears `bStrategyForced`. Otherwise it builds a temporary
/// (`00990320(temp, group)`), runs `0098a3c0(group)`, calls the strategy's
/// virtual method at slot 4 with the group and the temporary, stores the
/// strategy's `00726070` result in `iLastGroupStrategyChosenIndex` and the
/// game time in `fLastGroupStrategyChosenTimeStamp`, sets the flag, and
/// destroys the temporary (`fn_009863b0`). Exception unwinding is not
/// translated.
pub fn fn_009862e0(e: &mut Engine, this: Ptr<CombatGroup>, strategy: Ptr) {
    e.set(this, CombatGroup::pGroupStrategy, strategy);
    if e.get(this, CombatGroup::pGroupStrategy).is_null() {
        e.set(this, CombatGroup::bStrategyForced, false);
        return;
    }
    e.with_stack(0x40, |e, temporary| {
        e.call(0x0099_0320, &args![temporary, this]);
        e.call(0x0098_a3c0, &args![this]);
        let strategy = e.get(this, CombatGroup::pGroupStrategy);
        e.vcall(strategy.addr(), 4, &args![this, temporary]);
        let index = e.call(0x0072_6070, &args![strategy]).u32();
        e.set(this, CombatGroup::iLastGroupStrategyChosenIndex, index);
        let now = e.call(GAME_TIME_NOW, &args![]).f32();
        let stored = e.with_stack(4, |e, local| {
            e.call(TIME_STAMP_SET, &args![local, now]);
            e.mem.f32(local.addr())
        });
        e.set(this, CombatGroup::fLastGroupStrategyChosenTimeStamp, stored);
        e.set(this, CombatGroup::bStrategyForced, true);
        fn_009863b0(e, temporary);
    });
}

// Translated from 009863b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the temporary `fn_009862e0` builds (the decompiler's
/// `~_ATL_WIN_MODULE70` label is wrong): destroys the two arrays at +0x28
/// and +0x18 with `0098f560`. Exception unwinding is not translated.
pub fn fn_009863b0(e: &mut Engine, this: Ptr) {
    e.call(0x0098_f560, &args![this.byte_add(0x28)]);
    e.call(0x0098_f560, &args![this.byte_add(0x18)]);
}

// Translated from 00986410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup::AddTarget` (Xbox PDB): false for null; true when the actor
/// already is a target; false when it cannot be added (`CanAddTarget`);
/// false (with a log message) when it is a member; otherwise appends a new
/// `CombatTarget` (built with the group's search count), tells the combat
/// manager and returns true.
pub fn combat_group_add_target(e: &mut Engine, this: Ptr<CombatGroup>, actor: Ptr) -> bool {
    if actor.is_null() {
        return false;
    }
    if combat_group_is_target(e, this, actor) {
        return true;
    }
    if !combat_group_can_add_target(e, this, actor) {
        return false;
    }
    if combat_group_is_member(e, this, actor) {
        log_actor_message(e, actor, 0x0108_d5a8);
        return false;
    }
    let search_count = e.get(this, CombatGroup::iSearchCount);
    e.with_stack(CombatTarget::SIZE, |e, target| {
        fn_00985410(e, target.cast(), actor, search_count as u16);
        e.call(
            TARGET_ARRAY_ADD,
            &args![this.byte_add(CombatGroup::TARGET_ARRAY), target],
        );
    });
    let manager = combat_manager(e);
    e.call(MANAGER_ADD_TARGET, &args![manager, this, actor]);
    true
}

/// The "COMBAT: Trying to add actor %08X %s to Combat Group ..." log line
/// (`format` is the string's address): the actor's form ID and its name
/// (the virtual method at slot 0x130 when `00474cb0` says the actor has
/// one, else `0055d520`).
fn log_actor_message(e: &mut Engine, actor: Ptr, format: u32) {
    let name = if e.call(0x0047_4cb0, &args![actor]).u32() != 0 {
        e.vcall(actor.addr(), 0x130, &args![]).u32()
    } else {
        e.call(0x0055_d520, &args![actor]).u32()
    };
    let form_id = e.call(FORM_ID, &args![actor]).u32();
    e.call(LOG_MESSAGE, &args![format, form_id, name]);
}

// Translated from 00986500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup::RemoveTarget` (Xbox PDB): removes the first target whose
/// actor is `actor` (`fn_00986560`).
pub fn combat_group_remove_target(e: &mut Engine, this: Ptr<CombatGroup>, actor: Ptr) {
    let targets = this.byte_add(CombatGroup::TARGET_ARRAY);
    let mut index = 0;
    while index < e.call(ARRAY_SIZE, &args![targets]).u32() {
        let target = e
            .call(TARGET_AT, &args![targets, index])
            .ptr::<CombatTarget>();
        if e.get(target, CombatTarget::pActor) == actor {
            fn_00986560(e, this, index);
            return;
        }
        index += 1;
    }
}

// Translated from 00986560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes target `index`: tells the combat manager the actor is no longer
/// a target of the group, then removes the array entry.
pub fn fn_00986560(e: &mut Engine, this: Ptr<CombatGroup>, index: u32) {
    let targets = this.byte_add(CombatGroup::TARGET_ARRAY);
    let target = e
        .call(TARGET_AT, &args![targets, index])
        .ptr::<CombatTarget>();
    let actor = e.get(target, CombatTarget::pActor);
    let manager = combat_manager(e);
    e.call(MANAGER_REMOVE_TARGET, &args![manager, this, actor]);
    e.call(TARGET_ARRAY_REMOVE_AT, &args![targets, index, 1u32]);
}

// Translated from 009865b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup::IsTarget` (Xbox PDB): whether the actor is one of the
/// group's targets.
pub fn combat_group_is_target(e: &mut Engine, this: Ptr<CombatGroup>, actor: Ptr) -> bool {
    !fn_009865d0(e, this, actor).is_null()
}

// Translated from 009865d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `CombatTarget` of an actor (by form ID, `fn_00986610`), or null.
pub fn fn_009865d0(e: &mut Engine, this: Ptr<CombatGroup>, actor: Ptr) -> Ptr<CombatTarget> {
    let form_id = e.call(FORM_ID, &args![actor]).u32();
    fn_00986610(e, this, form_id)
}

// Translated from 009865f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `CombatMember` of an actor (by form ID, `fn_00986670`), or null.
pub fn fn_009865f0(e: &mut Engine, this: Ptr<CombatGroup>, actor: Ptr) -> Ptr<CombatMember> {
    let form_id = e.call(FORM_ID, &args![actor]).u32();
    fn_00986670(e, this, form_id)
}

// Translated from 00986610 (decompiled, FalloutNV.exe 1.4.0.525)
/// The first `CombatTarget` whose actor has form ID `form_id`, or null.
pub fn fn_00986610(e: &mut Engine, this: Ptr<CombatGroup>, form_id: u32) -> Ptr<CombatTarget> {
    let targets = this.byte_add(CombatGroup::TARGET_ARRAY);
    let mut index = 0;
    while index < e.call(ARRAY_SIZE, &args![targets]).u32() {
        let target = e
            .call(TARGET_AT, &args![targets, index])
            .ptr::<CombatTarget>();
        let actor = e.get(target, CombatTarget::pActor);
        if e.call(FORM_ID, &args![actor]).u32() == form_id {
            return target;
        }
        index += 1;
    }
    Ptr::NULL
}

// Translated from 00986670 (decompiled, FalloutNV.exe 1.4.0.525)
/// The first `CombatMember` whose actor has form ID `form_id`, or null.
pub fn fn_00986670(e: &mut Engine, this: Ptr<CombatGroup>, form_id: u32) -> Ptr<CombatMember> {
    let members = this.byte_add(CombatGroup::MEMBER_ARRAY);
    let mut index = 0;
    while index < e.call(ARRAY_SIZE, &args![members]).u32() {
        let member = e
            .call(MEMBER_AT, &args![members, index])
            .ptr::<CombatMember>();
        let actor = e.get(member, CombatMember::pActor);
        if e.call(FORM_ID, &args![actor]).u32() == form_id {
            return member;
        }
        index += 1;
    }
    Ptr::NULL
}

// Translated from 009866d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup::CanAddTarget` (Xbox PDB): true when every member actor
/// (null ones are skipped) accepts `target` (`008b0670(member, target)`).
pub fn combat_group_can_add_target(e: &mut Engine, this: Ptr<CombatGroup>, target: Ptr) -> bool {
    let members = e.call(MEMBER_COUNT, &args![this]).u32();
    let mut index = 0;
    while index < members {
        let member = e.call(MEMBER_ACTOR_AT, &args![this, index]).u32();
        if member != 0
            && !e
                .call(ACTOR_COMBAT_COMPATIBLE, &args![member, target])
                .bool()
        {
            return false;
        }
        index += 1;
    }
    true
}

// Translated from 00986740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `0098f580` with `other`'s target array as `this` and this group's
/// target array as the argument (an assignment between the two
/// `BSSimpleArray<CombatTarget>`s; the callee is not in this session's
/// range).
pub fn fn_00986740(e: &mut Engine, this: Ptr<CombatGroup>, other: Ptr<CombatGroup>) {
    e.call(
        0x0098_f580,
        &args![
            other.byte_add(CombatGroup::TARGET_ARRAY),
            this.byte_add(CombatGroup::TARGET_ARRAY)
        ],
    );
}

// Translated from 00986760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills the pointer array `other` with the address of each of this group's
/// `CombatTarget`s, in order (sized first with `SetSize(count, 1)`).
pub fn fn_00986760(e: &mut Engine, this: Ptr<CombatGroup>, other: Ptr) {
    let targets = this.byte_add(CombatGroup::TARGET_ARRAY);
    let count = e.call(ARRAY_SIZE, &args![targets]).u32();
    e.call(POINTER_ARRAY_SET_SIZE, &args![other, count, 1u32]);
    let mut index = 0;
    while index < e.call(ARRAY_SIZE, &args![targets]).u32() {
        let target = e.call(TARGET_AT, &args![targets, index]).u32();
        e.call(ARRAY_SET_AT, &args![other, index, target]);
        index += 1;
    }
}

// Translated from 009867d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup::AddMember` (Xbox PDB): false for null; true when the actor
/// already is a member; false when it cannot be added (`CanAddMember`);
/// false (with a log message) when it is a target. Otherwise appends a new
/// `CombatMember`, tells the combat manager, and, when the tint setting
/// (`011f1878`) is on and the actor has a 3D root (virtual method at slot
/// 0x1d0), tints it with the colour of the group number (`005d8550`);
/// finally restarts the cluster update timer (`+0xb0`, `0097f220`) and
/// returns true.
pub fn combat_group_add_member(e: &mut Engine, this: Ptr<CombatGroup>, actor: Ptr) -> bool {
    if actor.is_null() {
        return false;
    }
    if combat_group_is_member(e, this, actor) {
        return true;
    }
    if !combat_group_can_add_member(e, this, actor) {
        return false;
    }
    if combat_group_is_target(e, this, actor) {
        log_actor_message(e, actor, 0x0108_d600);
        return false;
    }
    e.with_stack(CombatMember::SIZE, |e, member| {
        fn_00986950(e, member.cast(), actor);
        e.call(
            MEMBER_ARRAY_ADD,
            &args![this.byte_add(CombatGroup::MEMBER_ARRAY), member],
        );
    });
    let manager = combat_manager(e);
    e.call(MANAGER_ADD_MEMBER, &args![manager, this, actor]);
    if bool_setting(e, TINT_SETTING) && e.vcall(actor.addr(), 0x1d0, &args![]).u32() != 0 {
        let group_number = e.get(this, CombatGroup::iGroupNum);
        let colour = e.call(0x005d_8550, &args![group_number]).u32();
        let (red, green, blue) = (
            e.mem.f32(colour),
            e.mem.f32(colour + 4),
            e.mem.f32(colour + 8),
        );
        e.with_stack(0xc, |e, rgb| {
            e.call(0x0041_6870, &args![rgb, red, green, blue]);
            let node = e.vcall(actor.addr(), 0x1d0, &args![]).u32();
            e.call(TINT_SCENEGRAPH, &args![node, rgb, 0u32]);
        });
    }
    e.call(
        0x0097_f220,
        &args![this.byte_add(CombatGroup::CLUSTER_UPDATE_TIMER)],
    );
    true
}

// Translated from 00986950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatMember` constructor (the map has no name): the actor, group
/// strategy assignment 0, damage per second and combat strength -1.0, no
/// cluster.
pub fn fn_00986950(e: &mut Engine, this: Ptr<CombatMember>, actor: Ptr) -> Ptr<CombatMember> {
    e.set(this, CombatMember::pActor, actor);
    e.set(this, CombatMember::iGroupStrategyAssignment, 0);
    let unset: f32 = e.global(MINUS_ONE);
    e.set(this, CombatMember::fDamagePerSecond, unset);
    e.set(this, CombatMember::fCombatStrength, unset);
    e.set(this, CombatMember::pCluster, Ptr::NULL);
    this
}

// Translated from 009869a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes `actor` from the group's members (the map has no name): tells the
/// combat manager, removes the entry, clears `pSearchingMember` when it was
/// that actor, and (tint setting on and a 3D root) removes the tint with
/// `UnTintScenegraph`. Does nothing when the actor is not a member.
pub fn fn_009869a0(e: &mut Engine, this: Ptr<CombatGroup>, actor: Ptr) {
    let members = this.byte_add(CombatGroup::MEMBER_ARRAY);
    let mut index = 0;
    while index < e.call(ARRAY_SIZE, &args![members]).u32() {
        let member = e
            .call(MEMBER_AT, &args![members, index])
            .ptr::<CombatMember>();
        if e.get(member, CombatMember::pActor) == actor {
            let manager = combat_manager(e);
            e.call(MANAGER_REMOVE_MEMBER, &args![manager, this, actor]);
            e.call(MEMBER_ARRAY_REMOVE_AT, &args![members, index, 1u32]);
            if e.get(this, CombatGroup::pSearchingMember) == actor {
                e.set(this, CombatGroup::pSearchingMember, Ptr::NULL);
            }
            if bool_setting(e, TINT_SETTING) && e.vcall(actor.addr(), 0x1d0, &args![]).u32() != 0 {
                let node = e.vcall(actor.addr(), 0x1d0, &args![]).u32();
                e.call(UNTINT_SCENEGRAPH, &args![node]);
            }
            return;
        }
        index += 1;
    }
}

// Translated from 00986a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup::IsMember` (Xbox PDB): whether the actor is one of the
/// group's members.
pub fn combat_group_is_member(e: &mut Engine, this: Ptr<CombatGroup>, actor: Ptr) -> bool {
    !fn_009865f0(e, this, actor).is_null()
}

// Translated from 00986aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup::CanAddMember` (Xbox PDB): true when `member` accepts every
/// target actor of the group (`008b0670(member, target actor)`).
pub fn combat_group_can_add_member(e: &mut Engine, this: Ptr<CombatGroup>, member: Ptr) -> bool {
    let targets = e.call(TARGET_COUNT, &args![this]).u32();
    let mut index = 0;
    while index < targets {
        let target = e.call(TARGET_ACTOR_AT, &args![this, index]).u32();
        if !e
            .call(ACTOR_COMBAT_COMPATIBLE, &args![member, target])
            .bool()
        {
            return false;
        }
        index += 1;
    }
    true
}

// Translated from 00986b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills the pointer array `other` with the address of each of this group's
/// `CombatMember`s, in order (sized first with `SetSize(count, 1)`).
pub fn fn_00986b00(e: &mut Engine, this: Ptr<CombatGroup>, other: Ptr) {
    let members = this.byte_add(CombatGroup::MEMBER_ARRAY);
    let count = e.call(ARRAY_SIZE, &args![members]).u32();
    e.call(POINTER_ARRAY_SET_SIZE, &args![other, count, 1u32]);
    let mut index = 0;
    while index < e.call(ARRAY_SIZE, &args![members]).u32() {
        let member = e.call(MEMBER_AT, &args![members, index]).u32();
        e.call(ARRAY_SET_AT, &args![other, index, member]);
        index += 1;
    }
}

// Translated from 00986b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls virtual method 0x3fc with argument 0 on every member actor.
pub fn fn_00986b70(e: &mut Engine, this: Ptr<CombatGroup>) {
    let members = e.call(MEMBER_COUNT, &args![this]).u32();
    for index in 0..members {
        let actor = e.call(MEMBER_ACTOR_AT, &args![this, index]).u32();
        e.vcall(actor, 0x3fc, &args![0u32]);
    }
}

// Translated from 00986bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes every member actor out of the combat manager's lists
/// (`008d0600(manager; group, actor)`), then calls `008454f0(member array,
/// 0)`.
pub fn fn_00986bd0(e: &mut Engine, this: Ptr<CombatGroup>) {
    let members = e.call(MEMBER_COUNT, &args![this]).u32();
    for index in 0..members {
        let actor = e.call(MEMBER_ACTOR_AT, &args![this, index]).u32();
        let manager = combat_manager(e);
        e.call(MANAGER_REMOVE_MEMBER, &args![manager, this, actor]);
    }
    e.call(
        0x0084_54f0,
        &args![this.byte_add(CombatGroup::MEMBER_ARRAY), 0u32],
    );
}

// Translated from 00986c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the group has no members (`0076b610` on the member array).
pub fn fn_00986c40(e: &mut Engine, this: Ptr<CombatGroup>) -> bool {
    e.call(
        0x0076_b610,
        &args![this.byte_add(CombatGroup::MEMBER_ARRAY)],
    )
    .bool()
}

// Translated from 00986c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Chooses the target `actor` (a member of this group) should fight (the map
/// has no name; returns the `Actor*`, or null).
///
/// Null when the group has no targets. An actor without a process
/// (virtual method at slot 0x428) is removed from the group
/// (`fn_009869a0`) and null is returned. Otherwise every target is examined
/// in order: skipped when it lies outside the process' view angle (when
/// that is positive; `009a6d70` with the position from the target's
/// virtual method at slot 0x1f4), when the cell setting (`011f1840`) is on
/// and it is in another cell, or when `008b06d0` says the actor should not
/// attack it (only asked when `00566950` is true for the actor). The
/// detection call `009887b0` gives a level (forced to 1 when negative for
/// the process' current target when `0097ee90` allows), two flags and a
/// time stamp. With a level above zero the score is 0, or, with more than
/// one target, the sum of the settings `011a4d78` .. `011a4d50` (second
/// flag, first flag, being the current target and a recency bonus, same
/// weapon style, distance capped and scaled, fewer attackers than the
/// current-target flag, minus penalties for unreachable locations,
/// locations the actor cannot exist in, a target that is unfit and one that
/// is fleeing); the best score wins. With a level of zero or less the
/// target is a fallback ranked by its last noticed time stamp (30 seconds
/// in the past, at least the previous best); the result is the best scored
/// target, else the best fallback, else the last target examined.
pub fn fn_00986c60(e: &mut Engine, this: Ptr<CombatGroup>, actor: Ptr) -> Ptr {
    if e.call(
        0x0076_b610,
        &args![this.byte_add(CombatGroup::TARGET_ARRAY)],
    )
    .bool()
    {
        return Ptr::NULL;
    }
    // The game's locals, in one block: +0x00 the fallback time stamp,
    // +0x04 the stamp from the detection call, +0x08 its level, +0x0c / +0x0d
    // its two flags, +0x10 the actor's location, +0x20 the target's
    // location, +0x30 a stamp temporary, +0x34 the age buffer, +0x38 the
    // `008b06d0` output.
    e.with_stack(0x40, |e, frame| choose_target(e, this, actor, frame))
}

fn choose_target(e: &mut Engine, this: Ptr<CombatGroup>, actor: Ptr, frame: Ptr) -> Ptr {
    let fallback_stamp = frame;
    let detected_stamp = frame.byte_add(0x04);
    let detected_level = frame.byte_add(0x08);
    let first_flag = frame.byte_add(0x0c);
    let second_flag = frame.byte_add(0x0d);
    let actor_location = frame.byte_add(0x10);
    let target_location = frame.byte_add(0x20);
    let stamp_temporary = frame.byte_add(0x30);
    let age_buffer = frame.byte_add(0x34);
    let attack_output = frame.byte_add(0x38);

    let mut best_score = -e.global::<f32>(MAX_FLOAT);
    let lowest = -e.global::<f32>(MAX_FLOAT);
    e.call(TIME_STAMP_SET, &args![fallback_stamp, lowest]);
    let mut best_actor = Ptr::NULL;
    let mut best_by_stamp = Ptr::NULL;
    let mut last_examined = Ptr::NULL;
    e.call(GET_WORLD_LOCATION, &args![actor, actor_location]);
    let process = e.vcall(actor.addr(), 0x428, &args![]).u32();
    if process == 0 {
        fn_009869a0(e, this, actor);
        return Ptr::NULL;
    }
    let current_target = e.call(0x0040_30b0, &args![process]).u32();
    let process_part = e.call(0x0058_6150, &args![process]).u32();
    let view = e.call(0x0098_0070, &args![process]).u32();
    let view_angle = e.call(0x0063_9aa0, &args![view]).f32();
    let cell = e.call(0x0056_7d90, &args![actor]).u32();
    let uses_melee = e.call(0x009a_9630, &args![actor]).u8();
    let count = e.call(TARGET_COUNT, &args![this]).u32();

    let mut index = 0;
    while index < count {
        let entry = e
            .call(0x0098_71c0, &args![this, index])
            .ptr::<CombatTarget>();
        index += 1;
        let target = e.get(entry, CombatTarget::pActor);

        if view_angle as f64 > e.global::<f64>(ZERO_DOUBLE) {
            let radians = (view_angle as f64 * e.global::<f64>(0x0102_3128)) as f32;
            let full_circle: f32 = e.global(0x0101_ff50);
            let position = e.vcall(target.addr(), 0x1f4, &args![]).u32();
            if !e
                .call(0x009a_6d70, &args![actor, position, radians, full_circle])
                .bool()
            {
                continue;
            }
        }
        if bool_setting(e, SAME_CELL_SETTING) {
            let target_cell = e.call(0x0056_7d90, &args![target]).u32();
            if cell != target_cell {
                continue;
            }
        }
        if e.call(0x0056_6950, &args![actor]).bool() {
            e.mem.set_u32(attack_output.addr(), 0);
            let wanted = e
                .call(
                    ACTOR_GET_SHOULD_ATTACK,
                    &args![actor, target, 0u32, attack_output, 0u32],
                )
                .bool();
            if !wanted {
                continue;
            }
        }

        e.mem.set_u8(second_flag.addr(), 0);
        e.mem.set_i32(detected_level.addr(), 0);
        e.mem.set_u8(first_flag.addr(), 0);
        e.call(TIME_STAMP_ZERO, &args![detected_stamp]);
        e.call(
            0x0098_87b0,
            &args![
                this,
                actor,
                target,
                detected_level,
                detected_stamp,
                first_flag,
                second_flag
            ],
        );
        if e.mem.i32(detected_level.addr()) < 0
            && target.addr() == current_target
            && e.call(0x0097_ee90, &args![process_part]).bool()
        {
            e.mem.set_i32(detected_level.addr(), 1);
        }

        if e.mem.i32(detected_level.addr()) > 0 {
            let mut score = 0.0f32;
            if count > 1 {
                let is_current = target.addr() == current_target;
                if e.mem.u8(second_flag.addr()) != 0 {
                    score += e.global::<f32>(0x011a_4d78);
                }
                if e.mem.u8(first_flag.addr()) != 0 {
                    score += e.global::<f32>(0x011a_4d74);
                }
                if is_current {
                    score += e.global::<f32>(0x011a_4d70);
                    if e.mem.u8(second_flag.addr()) == 0 {
                        let part = e.call(0x0058_6150, &args![process]).u32();
                        let stamp = e.call(0x0097_fb00, &args![part, age_buffer]).u32();
                        let age = e.call(TIME_STAMP_AGE, &args![stamp]).f32();
                        let window: f32 = e.global(0x011a_4d6c);
                        if age < window {
                            score += e.global::<f32>(0x011a_4d78);
                        }
                    }
                }
                let target_uses_melee = e.call(0x009a_9630, &args![target]).u8();
                if uses_melee == target_uses_melee {
                    score += e.global::<f32>(0x011a_4d68);
                }
                e.call(GET_WORLD_LOCATION, &args![target, target_location]);
                let target_space = e.call(FORM_ID, &args![target_location]).u32();
                let actor_space = e.call(FORM_ID, &args![actor_location]).u32();
                let other_space = target_space != actor_space;
                let distance = if other_space {
                    if is_current {
                        0.0
                    } else {
                        let far: f32 = e.global(0x011a_4d64);
                        far * far
                    }
                } else {
                    e.call(DISTANCE_SQUARED, &args![actor_location, target_location])
                        .f32()
                };
                let cap: f32 = e.global(0x0108_d660);
                let distance = e.call(0x0040_ebd0, &args![cap, distance]).f32();
                let divisor: f64 = e.global(0x0108_d658);
                let weight: f32 = e.global(0x011a_4d60);
                score = ((1.0 - distance as f64 / divisor) * weight as f64 + score as f64) as f32;
                let current_attackers = is_current as u16;
                if e.get(entry, CombatTarget::sAttackerCount) > current_attackers {
                    score += e.global::<f32>(0x011f_18b0);
                }
                if e.call(0x0098_71e0, &args![process_part]).bool() || other_space {
                    let reach: f32 = e.global(0x0103_2adc);
                    if e.call(0x009a_0f40, &args![process_part, target_location, reach])
                        .bool()
                    {
                        score -= e.global::<f32>(0x011a_4d5c);
                    }
                }
                if other_space && !e.call(0x009a_96f0, &args![actor, target_location]).bool() {
                    score -= e.global::<f32>(0x011a_4d58);
                }
                if e.vcall(target.addr(), 0x22c, &args![0u32]).bool()
                    || e.vcall(target.addr(), 0x230, &args![]).bool()
                {
                    score -= e.global::<f32>(0x011a_4d54);
                }
                if e.call(ACTOR_IS_FLEEING, &args![target, 0u32]).bool() {
                    score -= e.global::<f32>(0x011a_4d50);
                }
            }
            if best_score < score {
                best_score = score;
                best_actor = target;
            }
        } else {
            let noticed = time_stamp(e, entry.byte_add(CombatTarget::fLastNoticedTimeStamp.off));
            let past = (noticed as f64 - e.global::<f64>(0x0101_db88)) as f32;
            let current = time_stamp(e, detected_stamp);
            let newer = e.call(0x0040_4010, &args![past, current]).f32();
            e.call(TIME_STAMP_SET, &args![stamp_temporary, newer]);
            let newer = e.mem.u32(stamp_temporary.addr());
            e.mem.set_u32(detected_stamp.addr(), newer);
            let candidate = time_stamp(e, detected_stamp);
            if time_stamp(e, fallback_stamp) < candidate {
                best_by_stamp = target;
                let value = e.mem.u32(detected_stamp.addr());
                e.mem.set_u32(fallback_stamp.addr(), value);
            }
        }
        last_examined = target;
    }
    if best_actor.is_null() {
        best_actor = best_by_stamp;
    }
    if best_actor.is_null() {
        best_actor = last_examined;
    }
    best_actor
}

// Translated from 009871c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Target `index` of the group: `TARGET_AT` on `TargetArray` (the address
/// of the element, returned as `0096a2b0` returns it). The map has no name.
pub fn fn_009871c0(e: &mut Engine, this: Ptr<CombatGroup>, index: u32) -> Ptr<CombatTarget> {
    e.call(
        TARGET_AT,
        &args![this.byte_add(CombatGroup::TARGET_ARRAY), index],
    )
    .ptr()
}

// Translated from 009871e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatTarget`: whether `fDetectionEventTimeStamp` (read as the `float`
/// itself) equals the `double` zero.
pub fn fn_009871e0(e: &mut Engine, this: Ptr<CombatTarget>) -> bool {
    let stamp = e.get(this, CombatTarget::fDetectionEventTimeStamp);
    let zero: f64 = e.global(ZERO_DOUBLE);
    stamp as f64 == zero
}

// Translated from 00987220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: whether `target` should be dropped from consideration
/// (the map has no name; the body is a chain of time and distance tests).
/// In order, it answers true when:
/// - every member is fleeing (`fn_0098a2e0`) and either the target was last
///   detected longer ago than the setting `011ce290`, or longer ago than the
///   setting `011ce750` and no member lies within the setting `011ce38c` of
///   the target's actor (`fn_00989880`);
/// - the target was last noticed longer ago than the setting `011ce744`
///   and has no attackers, or longer ago than `011cf368` and either the
///   group's one search has a path handler that has travelled no distance,
///   or no member lies within the setting `011cf3fc` of the actor;
/// - a search is running (`iSearchCount`) and it exceeds the target's
///   search-noticed count plus the integer setting `011cef50`; or the
///   search started less than the setting `011cf0a4` ago, the last
///   attacked-member stamp is older than it, the last detected stamp is
///   still `-FLT_MAX` and the last noticed stamp is older than `011cf3b4`;
/// - the actor has the property `00437bd0` tests.
///
/// Otherwise the answer is the actor's virtual method 0x22c (argument 0)
/// being true while its virtual method 0x2e8 is false.
pub fn fn_00987220(e: &mut Engine, this: Ptr<CombatGroup>, target: Ptr<CombatTarget>) -> bool {
    let actor = e.get(target, CombatTarget::pActor);
    let last_detected = target.byte_add(CombatTarget::fLastDetectedTimeStamp.off);
    let last_noticed = target.byte_add(CombatTarget::fLastNoticedTimeStamp.off);
    if fn_0098a2e0(e, this) {
        let age = time_stamp_age(e, last_detected);
        if float_setting(e, 0x011c_e290) < age {
            return true;
        }
        let age = time_stamp_age(e, last_detected);
        if float_setting(e, 0x011c_e750) < age {
            let radius = float_setting(e, 0x011c_e38c);
            if fn_00989880(e, this, actor, radius).is_null() {
                return true;
            }
        }
    }
    let age = time_stamp_age(e, last_noticed);
    if float_setting(e, 0x011c_e744) < age {
        if e.get(target, CombatTarget::sAttackerCount) == 0 {
            return true;
        }
        let age = time_stamp_age(e, last_noticed);
        if float_setting(e, 0x011c_f368) < age {
            let mut stationary = false;
            let handler_slot = this.byte_add(CombatGroup::spPathingLOSGridMap.off);
            if e.get(this, CombatGroup::iSearchCount) == 1
                && e.call(NI_POINTER_GET, &args![handler_slot]).u32() != 0
            {
                let handler = e.call(NI_POINTER_GET, &args![handler_slot]).u32();
                let travelled = e
                    .call(PATH_HANDLER_DISTANCE_TRAVELLED, &args![handler])
                    .f32();
                let zero: f64 = e.global(ZERO_DOUBLE);
                if travelled as f64 == zero {
                    stationary = true;
                }
            }
            if stationary {
                return true;
            }
            let radius = float_setting(e, 0x011c_f3fc);
            if fn_00989880(e, this, actor, radius).is_null() {
                return true;
            }
        }
    }
    let search_count = e.get(this, CombatGroup::iSearchCount);
    if search_count != 0 {
        let noticed = e.get(target, CombatTarget::sLastSearchNoticed) as u32;
        let limit = noticed.wrapping_add(int_setting(e, 0x011c_ef50));
        if search_count > limit {
            return true;
        }
        let search_started = this.byte_add(CombatGroup::fSearchStartedTimeStamp.off);
        let age = time_stamp_age(e, search_started);
        if float_setting(e, 0x011c_f0a4) > age {
            let attacked = target.byte_add(CombatTarget::fLastAttackedMemberTimeStamp.off);
            let age = time_stamp_age(e, attacked);
            if float_setting(e, 0x011c_f0a4) <= age {
                let detected = time_stamp(e, last_detected);
                if detected == -e.global::<f32>(MAX_FLOAT) {
                    let age = time_stamp_age(e, last_noticed);
                    if float_setting(e, 0x011c_f3b4) < age {
                        return true;
                    }
                }
            }
        }
    }
    if e.call(ACTOR_TEST_00437BD0, &args![actor]).bool() {
        return true;
    }
    if !e.vcall(actor.addr(), 0x22c, &args![0u32]).bool() {
        return false;
    }
    !e.vcall(actor.addr(), 0x2e8, &args![]).bool()
}

// Translated from 009874b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup::IncrementAttackerCount` (Xbox PDB): one more attacker on
/// the actor's target entry, if it has one.
pub fn combat_group_increment_attacker_count(e: &mut Engine, this: Ptr<CombatGroup>, actor: Ptr) {
    let target = fn_009865d0(e, this, actor);
    if !target.is_null() {
        let count = e.get(target, CombatTarget::sAttackerCount);
        e.set(target, CombatTarget::sAttackerCount, count.wrapping_add(1));
    }
}

// Translated from 009874f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// One attacker fewer on the actor's target entry, if it has one (the
/// counterpart of `CombatGroup::IncrementAttackerCount`; no check against
/// zero).
pub fn fn_009874f0(e: &mut Engine, this: Ptr<CombatGroup>, actor: Ptr) {
    let target = fn_009865d0(e, this, actor);
    if !target.is_null() {
        let count = e.get(target, CombatTarget::sAttackerCount);
        e.set(target, CombatTarget::sAttackerCount, count.wrapping_sub(1));
    }
}

// Translated from 00987530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup::MergeGroup` (Xbox PDB): moves everything of `other` into
/// this group. Returns false, changing nothing, when this group cannot add
/// one of `other`'s targets or `other` cannot add one of this group's
/// targets (`CanAddTarget`, `fn_009866d0`). Otherwise `other`'s clusters
/// are appended to this group's cluster array (`other`'s cluster count read
/// as a signed byte, each cluster fetched by `0098de60`) and `other`'s
/// cluster array is cleared; each member of `other` is added with
/// `AddMember` and told its new group by virtual method 0x3fc, with its
/// entry copied over (`fn_00985980`), or logged when it cannot be added;
/// each target is added with `AddTarget` and its entry merged
/// (`fn_00985520`), or logged. The music state of `other` is taken when it
/// is not -1. Returns true.
pub fn combat_group_merge_group(
    e: &mut Engine,
    this: Ptr<CombatGroup>,
    other: Ptr<CombatGroup>,
) -> bool {
    let other_targets = e.call(OTHER_TARGET_ARRAY, &args![other]).u32();
    let mut index = 0;
    while index < e.call(ARRAY_SIZE, &args![other_targets]).u32() {
        let target = e
            .call(TARGET_AT, &args![other_targets, index])
            .ptr::<CombatTarget>();
        let actor = e.get(target, CombatTarget::pActor);
        if !combat_group_can_add_target(e, this, actor) {
            return false;
        }
        index += 1;
    }
    let my_targets = this.byte_add(CombatGroup::TARGET_ARRAY);
    let mut index = 0;
    while index < e.call(ARRAY_SIZE, &args![my_targets]).u32() {
        let target = e
            .call(TARGET_AT, &args![my_targets, index])
            .ptr::<CombatTarget>();
        let actor = e.get(target, CombatTarget::pActor);
        if !combat_group_can_add_target(e, other, actor) {
            return false;
        }
        index += 1;
    }
    let mut index = 0u32;
    while index < fn_009877c0(e, other) as u8 as i8 as i32 as u32 {
        let cluster = e.call(CLUSTER_BY_INDEX, &args![other, index & 0xff]).u32();
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), cluster);
            e.call(
                POINTER_ARRAY_ADD,
                &args![this.byte_add(CombatGroup::CLUSTER_ARRAY), slot],
            );
        });
        index += 1;
    }
    fn_009877e0(e, other);
    let other_members = e.call(OTHER_MEMBER_ARRAY, &args![other]).u32();
    let mut index = 0;
    while index < e.call(ARRAY_SIZE, &args![other_members]).u32() {
        let member = e
            .call(MEMBER_AT, &args![other_members, index])
            .ptr::<CombatMember>();
        let actor = e.get(member, CombatMember::pActor);
        if combat_group_add_member(e, this, actor) {
            e.vcall(actor.addr(), 0x3fc, &args![this]);
            let mine = fn_009865f0(e, this, actor);
            if !mine.is_null() {
                fn_00985980(e, mine, member);
            }
        } else {
            let name = e.vcall(actor.addr(), 0x130, &args![]).u32();
            let form_id = e.call(FORM_ID, &args![actor]).u32();
            e.call(LOG_MESSAGE, &args![MERGE_MEMBER_ERROR, form_id, name]);
        }
        index += 1;
    }
    let mut index = 0;
    while index < e.call(ARRAY_SIZE, &args![other_targets]).u32() {
        let target = e
            .call(TARGET_AT, &args![other_targets, index])
            .ptr::<CombatTarget>();
        let actor = e.get(target, CombatTarget::pActor);
        if combat_group_add_target(e, this, actor) {
            let mine = fn_009865d0(e, this, actor);
            if !mine.is_null() {
                fn_00985520(e, mine, target);
            }
        } else {
            let name = e.vcall(actor.addr(), 0x130, &args![]).u32();
            let form_id = e.call(FORM_ID, &args![actor]).u32();
            e.call(LOG_MESSAGE, &args![MERGE_TARGET_ERROR, form_id, name]);
        }
        index += 1;
    }
    let state = fn_00987800(e, other);
    if state != -1 {
        e.set(this, CombatGroup::cCombatMusicState, state);
    }
    true
}

// Translated from 009877c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: the size of `ClusterArray` (`0044ddc0`).
pub fn fn_009877c0(e: &mut Engine, this: Ptr<CombatGroup>) -> u32 {
    e.call(
        ARRAY_SIZE,
        &args![this.byte_add(CombatGroup::CLUSTER_ARRAY)],
    )
    .u32()
}

// Translated from 009877e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: calls `008454f0(ClusterArray, 1)`, the array's
/// clear-and-release.
pub fn fn_009877e0(e: &mut Engine, this: Ptr<CombatGroup>) {
    e.call(
        0x0084_54f0,
        &args![this.byte_add(CombatGroup::CLUSTER_ARRAY), 1u32],
    );
}

// Translated from 00987800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: `cCombatMusicState`.
pub fn fn_00987800(e: &mut Engine, this: Ptr<CombatGroup>) -> i8 {
    e.get(this, CombatGroup::cCombatMusicState)
}

// Translated from 00989010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSWorldLocation::SetLocation` (Xbox PDB): sets the three coordinates
/// from the point at `point` and the space (`pSpace`, at +0x0c) to `space`.
pub fn bgs_world_location_set_location(e: &mut Engine, this: Ptr, point: Ptr, space: u32) {
    for word in 0..3 {
        let value = e.mem.u32(point.addr() + 4 * word);
        e.mem.set_u32(this.addr() + 4 * word, value);
    }
    e.mem.set_u32(this.addr() + 0x0c, space);
}

// Translated from 00989040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: the detection level of `target` as the detection call
/// `009887b0` gives it with no actor of its own (argument 0); the level
/// starts at -100 and the call's other outputs (a time stamp, two flags)
/// are dropped.
pub fn fn_00989040(e: &mut Engine, this: Ptr<CombatGroup>, target: Ptr) -> i32 {
    // The game's locals: +0 the level, +4 the stamp, +8 / +9 the flags.
    e.with_stack(0x10, |e, frame| {
        let level = frame;
        let stamp = frame.byte_add(4);
        let first_flag = frame.byte_add(8);
        let second_flag = frame.byte_add(9);
        e.mem.set_i32(level.addr(), -100);
        e.call(TIME_STAMP_ZERO, &args![stamp]);
        e.mem.set_u8(first_flag.addr(), 0);
        e.mem.set_u8(second_flag.addr(), 0);
        e.call(
            0x0098_87b0,
            &args![this, 0u32, target, level, stamp, first_flag, second_flag],
        );
        e.mem.i32(level.addr())
    })
}

// Translated from 00989090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: writes the actor's `fLastDetectedTimeStamp` into the
/// time stamp at `result`, or `-FLT_MAX` (`CombatTimeStamp` setter) when the
/// actor is not a target. Returns `result`.
pub fn fn_00989090(e: &mut Engine, this: Ptr<CombatGroup>, result: Ptr, actor: Ptr) -> Ptr {
    let target = fn_009865d0(e, this, actor);
    if target.is_null() {
        let lowest = -e.global::<f32>(MAX_FLOAT);
        e.call(TIME_STAMP_SET, &args![result, lowest]);
    } else {
        let value = e
            .mem
            .u32(target.addr() + CombatTarget::fLastDetectedTimeStamp.off);
        e.mem.set_u32(result.addr(), value);
    }
    result
}

// Translated from 009890e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: copies the actor's `LastDetectedLocation` to `out` when
/// the actor is a target and the location's space (`0084e3a0`, the word at
/// +0x0c) is not null.
pub fn fn_009890e0(e: &mut Engine, this: Ptr<CombatGroup>, actor: Ptr, out: Ptr) -> bool {
    let target = fn_009865d0(e, this, actor);
    if target.is_null() {
        return false;
    }
    let location = target.byte_add(CombatTarget::LAST_DETECTED_LOCATION);
    if e.call(FORM_ID, &args![location]).u32() == 0 {
        return false;
    }
    copy_location(e, out, location);
    true
}

// Translated from 00989140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: for target number `index` (`fn_009871c0`), when `time` is
/// later than its `fLastSeenTimeStamp` age (`00435e00`), copies its
/// `LastSeenLocation` to `out` and returns true.
pub fn fn_00989140(
    e: &mut Engine,
    this: Ptr<CombatGroup>,
    index: u32,
    out: Ptr,
    time: f32,
) -> bool {
    let target = fn_009871c0(e, this, index);
    if target.is_null() {
        return false;
    }
    let age = time_stamp_age(e, target.byte_add(CombatTarget::fLastSeenTimeStamp.off));
    let later = time > age;
    if !later {
        return false;
    }
    copy_location(e, out, target.byte_add(CombatTarget::LAST_SEEN_LOCATION));
    true
}

// Translated from 009891a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: the actor's target entry's `cMemberLOSCount`, or 0.
pub fn fn_009891a0(e: &mut Engine, this: Ptr<CombatGroup>, actor: Ptr) -> u8 {
    let target = fn_009865d0(e, this, actor);
    if target.is_null() {
        0
    } else {
        e.get(target, CombatTarget::cMemberLOSCount)
    }
}

// Translated from 009891d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: with a group strategy set (`pGroupStrategy`), looks up
/// `first` with `0097ae90` and `second` with `009611e0`; when the second
/// answers non-null and is the strategy assignment of the member
/// `first` (`fn_0098a310`), clears that assignment (`fn_0098a360`).
pub fn fn_009891d0(e: &mut Engine, this: Ptr<CombatGroup>, first: Ptr, second: Ptr) {
    if e.get(this, CombatGroup::pGroupStrategy).is_null() {
        return;
    }
    let actor = e.call(0x0097_ae90, &args![first]).u32();
    let wanted = e.call(0x0096_11e0, &args![second]).u32();
    if wanted != 0 && fn_0098a310(e, this, Ptr::new(actor)) == wanted {
        fn_0098a360(e, this, Ptr::new(actor), 0);
    }
}

// Translated from 00989230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: records that the target `actor` just attacked a member:
/// its `fLastAttackedMemberTimeStamp` becomes the game time now and its
/// `LastAttackedMemberLocation` the actor's world location. Nothing for a
/// null actor or a non-target. The first stack word (the caller's attacker,
/// `0097f5bc`) is never read (`RET 0x8`).
pub fn fn_00989230(e: &mut Engine, this: Ptr<CombatGroup>, _unused_1: u32, actor: Ptr) {
    if actor.is_null() {
        return;
    }
    let target = fn_009865d0(e, this, actor);
    if target.is_null() {
        return;
    }
    // The game's locals: +0 a time stamp, +0x10 a world location.
    e.with_stack(0x20, |e, frame| {
        let now = e.call(GAME_TIME_NOW, &args![]).f32();
        e.call(TIME_STAMP_SET, &args![frame, now]);
        let stamp = e.mem.u32(frame.addr());
        e.mem.set_u32(
            target.addr() + CombatTarget::fLastAttackedMemberTimeStamp.off,
            stamp,
        );
        let location = e
            .call(GET_WORLD_LOCATION, &args![actor, frame.byte_add(0x10)])
            .ptr();
        copy_location(
            e,
            target.byte_add(CombatTarget::LAST_ATTACKED_MEMBER_LOCATION),
            location,
        );
    });
}

/// The array of avoid nodes the three functions below fill: the caller's, or
/// a new 0x18-byte one (`operator new` `00aa13e0`, built by `006e3850`).
fn avoid_array_or_new(e: &mut Engine, array: Ptr) -> Ptr {
    if !array.is_null() {
        return array;
    }
    let block = e.call(OPERATOR_NEW, &args![0x18u32]).u32();
    if block == 0 {
        Ptr::NULL
    } else {
        e.call(AVOID_ARRAY_CONSTRUCT, &args![block]).ptr()
    }
}

/// Builds a `PathingAvoidNode` (0x24 bytes, `006915d0`) on the stack and
/// adds it to `array` (`00905450`).
fn add_avoid_node(e: &mut Engine, array: Ptr, point: Ptr, first: f32, second: f32) {
    e.with_stack(0x24, |e, node| {
        e.call(AVOID_NODE_CONSTRUCT, &args![node, point, first, second]);
        e.call(AVOID_ARRAY_ADD, &args![array, node]);
    });
}

// Translated from 009892a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: adds an avoid node for every target (except `exclude`)
/// lying inside a sphere around the middle of `actor`'s position
/// (virtual method 0x1f4) and `point` (the half of their sum), whose
/// squared radius is `(radius^2 + |point - middle|^2)` times the setting
/// `011cf380` squared. Each node is placed at the target's position with
/// the settings `011ce648` and `011cfcf8` as its two values. `array` is
/// the avoid node array to fill, a new one when null; it is returned.
/// (The compiler's exception frame is not translated.)
pub fn fn_009892a0(
    e: &mut Engine,
    this: Ptr<CombatGroup>,
    actor: Ptr,
    point: Ptr,
    radius: f32,
    array: Ptr,
    exclude: Ptr,
) -> Ptr {
    // The game's locals: +0x00 the actor position plus `point` (then the
    // middle at +0x0c), +0x18 the offset from the middle to `point`, +0x24
    // the target position copy, +0x30 the offset from the middle to it.
    e.with_stack(0x40, |e, frame| {
        let sum = frame;
        let middle = frame.byte_add(0x0c);
        let offset = frame.byte_add(0x18);
        let position = frame.byte_add(0x24);
        let difference = frame.byte_add(0x30);
        let actor_position = e.vcall(actor.addr(), 0x1f4, &args![]).u32();
        let two: f32 = e.global(TWO_FLOAT);
        let added = e.call(POINT_ADD, &args![actor_position, sum, point]).u32();
        e.call(POINT_SCALE_DOWN, &args![added, middle, two]);
        e.call(POINT_SUBTRACT, &args![point, offset, middle]);
        let length = e.call(POINT_LENGTH_SQUARED, &args![offset]).f32();
        let mut limit = (radius as f64 * radius as f64 + length as f64) as f32;
        let setting_a = float_setting(e, 0x011c_f380);
        let setting_b = float_setting(e, 0x011c_f380);
        limit = (setting_a as f64 * setting_b as f64 * limit as f64) as f32;
        let mut array = array;
        let count = e.call(TARGET_COUNT, &args![this]).u32();
        for index in 0..count {
            let target_actor = e.call(TARGET_ACTOR_AT, &args![this, index]).u32();
            if target_actor == exclude.addr() {
                continue;
            }
            let found = e.vcall(target_actor, 0x1f4, &args![]).u32();
            copy_point(e, position, found);
            e.call(POINT_SUBTRACT, &args![middle, difference, position]);
            let distance = e.call(POINT_LENGTH_SQUARED, &args![difference]).f32();
            let inside = limit > distance;
            if !inside {
                continue;
            }
            array = avoid_array_or_new(e, array);
            let second = float_setting(e, 0x011c_fcf8);
            let first = float_setting(e, 0x011c_e648);
            add_avoid_node(e, array, position, first, second);
        }
        array
    })
}

// Translated from 00989490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: adds one avoid node for the group's bounding sphere:
/// centre and squared extent from `fn_00989eb0` (the targets' box), the
/// radius being the square root (`004579e0`) of the squared extent when that
/// exceeds the square of 1024.0 (else 1024.0), and the second value the
/// setting `011cfcf8` times the `double` 4.0. The three words before the
/// array are not read (`_unused_1` .. `_unused_3`). `array` is filled, a new
/// one when null; it is returned.
pub fn fn_00989490(
    e: &mut Engine,
    this: Ptr<CombatGroup>,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    array: Ptr,
) -> Ptr {
    e.with_stack(0x10, |e, centre| {
        e.call(POINT_NO_OP, &args![centre]);
        let mut radius: f32 = e.global(SPHERE_MINIMUM_RADIUS);
        let squared = fn_00989eb0(e, this, centre);
        if squared as f64 > radius as f64 * radius as f64 {
            radius = e.call(SQUARE_ROOT, &args![squared]).f32();
        }
        let array = avoid_array_or_new(e, array);
        let setting = float_setting(e, 0x011c_fcf8);
        let factor: f64 = e.global(FOUR_DOUBLE);
        let second = (setting as f64 * factor) as f32;
        add_avoid_node(e, array, centre, radius, second);
        array
    })
}

// Translated from 009895a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: adds an avoid node for every target whose world location
/// lies within the setting `011cea50` of `actor`'s (squared distance
/// compared with the setting squared), at that location, with the settings
/// `011ce900` and `011ceb2c` as its values. `array` is filled, a new one
/// when null; it is returned. (The exception frame is not translated.)
pub fn fn_009895a0(e: &mut Engine, this: Ptr<CombatGroup>, actor: Ptr, array: Ptr) -> Ptr {
    let mut array = array;
    let count = e.call(TARGET_COUNT, &args![this]).u32();
    let a = float_setting(e, 0x011c_ea50);
    let b = float_setting(e, 0x011c_ea50);
    let limit = (a as f64 * b as f64) as f32;
    // The game's locals: +0 the actor's location, +0x10 a target's.
    e.with_stack(0x20, |e, frame| {
        let actor_location = frame;
        let target_location = frame.byte_add(0x10);
        e.call(GET_WORLD_LOCATION, &args![actor, actor_location]);
        for index in 0..count {
            let target_actor = e.call(TARGET_ACTOR_AT, &args![this, index]).u32();
            e.call(GET_WORLD_LOCATION, &args![target_actor, target_location]);
            let distance = e
                .call(DISTANCE_SQUARED, &args![actor_location, target_location])
                .f32();
            let inside = limit > distance;
            if !inside {
                continue;
            }
            array = avoid_array_or_new(e, array);
            let second = float_setting(e, 0x011c_eb2c);
            let first = float_setting(e, 0x011c_e900);
            let point = e.call(POINT_NO_OP, &args![target_location]).u32();
            add_avoid_node(e, array, Ptr::new(point), first, second);
        }
    });
    array
}

// Translated from 00989700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup::FindTargetNearLocation` (Xbox PDB): the actor of the first
/// target (in array order) whose world location is closer to `location`
/// than `radius`, or null. With `only_recent` set, targets must first pass
/// `fn_009897a0`.
pub fn combat_group_find_target_near_location(
    e: &mut Engine,
    this: Ptr<CombatGroup>,
    location: Ptr,
    radius: f32,
    only_recent: bool,
) -> Ptr {
    let limit = (radius as f64 * radius as f64) as f32;
    let count = e.call(TARGET_COUNT, &args![this]).u32();
    for index in 0..count {
        let target = fn_009871c0(e, this, index);
        if only_recent && !fn_009897a0(e, target) {
            continue;
        }
        let actor = e.get(target, CombatTarget::pActor);
        let distance = e.with_stack(0x10, |e, found| {
            e.call(GET_WORLD_LOCATION, &args![actor, found]);
            e.call(DISTANCE_SQUARED, &args![location, found]).f32()
        });
        if limit > distance {
            return actor;
        }
    }
    Ptr::NULL
}

// Translated from 009897a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatTarget`: whether the setting `011ce9b4` is later than the age of
/// `fLastDetectedTimeStamp`.
pub fn fn_009897a0(e: &mut Engine, this: Ptr<CombatTarget>) -> bool {
    let age = time_stamp_age(e, this.byte_add(CombatTarget::fLastDetectedTimeStamp.off));
    float_setting(e, 0x011c_e9b4) > age
}

// Translated from 009897f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: the first member actor, other than `exclude`, whose world
/// location is closer to `location` than `radius`; null when none.
pub fn fn_009897f0(
    e: &mut Engine,
    this: Ptr<CombatGroup>,
    location: Ptr,
    radius: f32,
    exclude: Ptr,
) -> Ptr {
    let limit = (radius as f64 * radius as f64) as f32;
    let count = e.call(MEMBER_COUNT, &args![this]).u32();
    for index in 0..count {
        let actor = e.call(MEMBER_ACTOR_AT, &args![this, index]).u32();
        if actor == exclude.addr() {
            continue;
        }
        let distance = e.with_stack(0x10, |e, found| {
            e.call(GET_WORLD_LOCATION, &args![actor, found]);
            e.call(DISTANCE_SQUARED, &args![location, found]).f32()
        });
        if limit > distance {
            return Ptr::new(actor);
        }
    }
    Ptr::NULL
}

// Translated from 00989880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: `fn_009897f0` around the world location of `actor`
/// (with `actor` itself excluded): the first other member within `radius`.
pub fn fn_00989880(e: &mut Engine, this: Ptr<CombatGroup>, actor: Ptr, radius: f32) -> Ptr {
    e.with_stack(0x10, |e, location| {
        let location = e.call(GET_WORLD_LOCATION, &args![actor, location]).ptr();
        fn_009897f0(e, this, location, radius, actor)
    })
}

// Translated from 009898b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: the member actor, other than `exclude`, whose world
/// location is nearest to `location` and nearer than `max_distance` (no
/// limit when it is `FLT_MAX`); null when none.
pub fn fn_009898b0(
    e: &mut Engine,
    this: Ptr<CombatGroup>,
    location: Ptr,
    exclude: Ptr,
    max_distance: f32,
) -> Ptr {
    let maximum: f32 = e.global(MAX_FLOAT);
    let mut best = if max_distance == maximum {
        maximum
    } else {
        (max_distance as f64 * max_distance as f64) as f32
    };
    let mut nearest = Ptr::NULL;
    let count = e.call(MEMBER_COUNT, &args![this]).u32();
    for index in 0..count {
        let actor = e.call(MEMBER_ACTOR_AT, &args![this, index]).u32();
        if actor == exclude.addr() {
            continue;
        }
        let distance = e.with_stack(0x10, |e, found| {
            e.call(GET_WORLD_LOCATION, &args![actor, found]);
            e.call(DISTANCE_SQUARED, &args![location, found]).f32()
        });
        if best > distance {
            best = distance;
            nearest = Ptr::new(actor);
        }
    }
    nearest
}

// Translated from 00989970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: `fn_009898b0` from the world location of `actor`, with
/// `actor` excluded: the member nearest to it within `radius`.
pub fn fn_00989970(e: &mut Engine, this: Ptr<CombatGroup>, actor: Ptr, radius: f32) -> Ptr {
    e.with_stack(0x10, |e, location| {
        let location = e.call(GET_WORLD_LOCATION, &args![actor, location]).ptr();
        fn_009898b0(e, this, location, actor, radius)
    })
}

/// `CheckMemberTargetIntersection`'s fixed frame under `EBP` (`SUB ESP,0x104`).
const INTERSECTION_FIXED_LOCALS: u32 = 0x104;

// Translated from 009899a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup::CheckMemberTargetIntersection` (Xbox PDB): whether the
/// members and the targets lie on opposite sides of the group.
/// Nothing is checked (false) when either side is empty or both have one
/// actor. Otherwise the centres of the members (`M`) and of the targets
/// (`T`) and of both together (`A`) are taken from the actors' positions
/// (virtual method 0x1f4); `d = T - M` is the direction, and the point
/// `P = M + unit(d) * f(dot(A - M, d))` (`f` = `004579e0`) is found. The
/// members and targets that lie behind `P` along `M - P` (negative dot
/// product) are counted; the answer is true when those fractions are both
/// above the `double` at `01026998` (0.33) or both below the one at
/// `01051680` (0.67).
///
/// The frame is the exe's: `EBP` is the top of one `with_stack` block, the
/// fixed locals sit under it (`SUB ESP,0x104`) and the two position tables
/// (12 bytes an actor) come from `_alloca16` (`00ec5ec0`, the member table
/// first from `ESP = EBP-0x104`, then the target table under it), placed with
/// [`alloca16_esp`], so their addresses and 16-byte alignment are the exe's.
/// The block holds the fixed locals, both tables and both alignment pads. The
/// `/GS` cookie is not written.
pub fn combat_group_check_member_target_intersection(
    e: &mut Engine,
    this: Ptr<CombatGroup>,
) -> bool {
    let member_count = e.call(MEMBER_COUNT, &args![this]).u32();
    let target_count = e.call(TARGET_COUNT, &args![this]).u32();
    if member_count == 0 || target_count == 0 || (member_count == 1 && target_count == 1) {
        return false;
    }
    let block_size = INTERSECTION_FIXED_LOCALS
        .wrapping_add(12u32.wrapping_mul(member_count.wrapping_add(target_count)))
        .wrapping_add(0x20);
    e.with_stack(block_size, |e, block| {
        let ebp = block.addr().wrapping_add(block_size);
        let member_positions = alloca16_esp(ebp - INTERSECTION_FIXED_LOCALS, 12 * member_count);
        let target_positions = alloca16_esp(member_positions, 12 * target_count);
        // The game's NiPoint3 locals under EBP: the sum (then centre) of the
        // members, of the targets and of both, the direction, the offset from M to
        // A, the scaled direction, the point P, M - P, and the temporaries of the
        // member and the target loops (00989adc..00989d2b).
        let member_sum: Ptr = Ptr::new(ebp - 0x4c);
        let target_sum: Ptr = Ptr::new(ebp - 0x74);
        let total: Ptr = Ptr::new(ebp - 0x80);
        let direction: Ptr = Ptr::new(ebp - 0x24);
        let to_all: Ptr = Ptr::new(ebp - 0x5c);
        let scaled: Ptr = Ptr::new(ebp - 0xc0);
        let behind_point: Ptr = Ptr::new(ebp - 0x30);
        let reference: Ptr = Ptr::new(ebp - 0x14);
        let member_temporary: Ptr = Ptr::new(ebp - 0xa0);
        let target_temporary: Ptr = Ptr::new(ebp - 0xb0);
        for sum in [member_sum, target_sum] {
            for word in 0..3 {
                let value = e.mem.u32(ZERO_POINT + 4 * word);
                e.mem.set_u32(sum.addr() + 4 * word, value);
            }
        }
        for index in 0..member_count {
            let actor = e.call(MEMBER_ACTOR_AT, &args![this, index]).u32();
            let found = e.vcall(actor, 0x1f4, &args![]).u32();
            let slot = Ptr::new(member_positions + 12 * index);
            copy_point(e, slot, found);
            e.call(POINT_ACCUMULATE, &args![member_sum, slot]);
        }
        for index in 0..target_count {
            let actor = e.call(TARGET_ACTOR_AT, &args![this, index]).u32();
            let found = e.vcall(actor, 0x1f4, &args![]).u32();
            let slot = Ptr::new(target_positions + 12 * index);
            copy_point(e, slot, found);
            e.call(POINT_ACCUMULATE, &args![target_sum, slot]);
        }
        e.call(POINT_ADD, &args![member_sum, total, target_sum]);
        e.call(
            POINT_DIVIDE_IN_PLACE,
            &args![member_sum, member_count as f32],
        );
        e.call(
            POINT_DIVIDE_IN_PLACE,
            &args![target_sum, target_count as f32],
        );
        e.call(
            POINT_DIVIDE_IN_PLACE,
            &args![total, member_count.wrapping_add(target_count) as f32],
        );
        e.call(POINT_SUBTRACT, &args![target_sum, direction, member_sum]);
        e.call(POINT_SUBTRACT, &args![total, to_all, member_sum]);
        let along = e.call(POINT_DOT, &args![to_all, direction]).f32();
        let distance = e.call(SQUARE_ROOT, &args![along]).f32();
        e.call(POINT_UNITIZE_GET_LENGTH, &args![direction]);
        let scaled = e
            .call(POINT_SCALE, &args![direction, scaled, distance])
            .u32();
        e.call(POINT_ADD, &args![member_sum, behind_point, scaled]);
        e.call(POINT_SUBTRACT, &args![member_sum, reference, behind_point]);
        let mut members_behind = 0u32;
        for index in 0..member_count {
            e.call(
                POINT_SUBTRACT,
                &args![
                    member_positions + 12 * index,
                    member_temporary,
                    behind_point
                ],
            );
            let dot = e.call(POINT_DOT, &args![member_temporary, reference]).f32();
            if (dot as f64) < e.global::<f64>(ZERO_DOUBLE) {
                members_behind += 1;
            }
        }
        let mut targets_behind = 0u32;
        for index in 0..target_count {
            e.call(
                POINT_SUBTRACT,
                &args![
                    target_positions + 12 * index,
                    target_temporary,
                    behind_point
                ],
            );
            let dot = e.call(POINT_DOT, &args![target_temporary, reference]).f32();
            if (dot as f64) < e.global::<f64>(ZERO_DOUBLE) {
                targets_behind += 1;
            }
        }
        let member_fraction = (members_behind as f64 / member_count as f64) as f32 as f64;
        let target_fraction = (targets_behind as f64 / target_count as f64) as f32 as f64;
        let low: f64 = e.global(FRACTION_LOW);
        let high: f64 = e.global(FRACTION_HIGH);
        (member_fraction > low && target_fraction > low)
            || (member_fraction < high && target_fraction < high)
    })
}

/// Builds the box of the actors of `count_at` / `actor_at` (the group's
/// members or targets) in `corners` (eight `NiPoint3`s): unless `keep` is
/// set, corner `c`'s component `a` starts at `-FLT_MAX` when the table at
/// `0108d57c` (3 bytes per corner) holds a non-zero byte and `FLT_MAX`
/// otherwise; then every actor position (virtual method 0x1f4) raises (table
/// non-zero) or lowers (zero) the components.
fn expand_corners(
    e: &mut Engine,
    this: Ptr<CombatGroup>,
    corners: Ptr,
    keep: bool,
    count_at: u32,
    actor_at: u32,
) {
    if !keep {
        for corner in 0..8u32 {
            for axis in 0..3u32 {
                let flag = e.mem.u8(CORNER_TABLE + corner * 3 + axis);
                let maximum: f32 = e.global(MAX_FLOAT);
                let start = if flag != 0 { -maximum } else { maximum };
                let slot = e
                    .call(POINT_COMPONENT, &args![corners.addr() + corner * 12, axis])
                    .u32();
                e.mem.set_f32(slot, start);
            }
        }
    }
    let count = e.call(count_at, &args![this]).u32();
    let position = e.mem.alloc(12);
    for index in 0..count {
        let actor = e.call(actor_at, &args![this, index]).u32();
        let found = e.vcall(actor, 0x1f4, &args![]).u32();
        copy_point(e, Ptr::new(position), found);
        for corner in 0..8u32 {
            for axis in 0..3u32 {
                let own = e.call(POINT_COMPONENT, &args![position, axis]).u32();
                let value = e.mem.f32(own);
                let flag = e.mem.u8(CORNER_TABLE + corner * 3 + axis);
                let slot = e
                    .call(POINT_COMPONENT, &args![corners.addr() + corner * 12, axis])
                    .u32();
                let current = e.mem.f32(slot);
                let replace = if flag != 0 {
                    current < value
                } else {
                    current > value
                };
                if replace {
                    let slot = e
                        .call(POINT_COMPONENT, &args![corners.addr() + corner * 12, axis])
                        .u32();
                    e.mem.set_f32(slot, value);
                }
            }
        }
    }
    e.mem.free(position);
}

/// The centre and the squared diagonal of the box `fill` builds: eight
/// corner `NiPoint3`s are constructed (`00401050` over `006815c0`), filled,
/// and the sum of the first (all maxima) and last (all minima) corners
/// halved is written to `out`; the result is the squared length of their
/// difference (`004a7290`).
fn box_centre(e: &mut Engine, out: Ptr, fill: impl FnOnce(&mut Engine, Ptr)) -> f32 {
    // The game's locals: eight corners (0x60 bytes), then the sum, the
    // halved sum and the difference.
    e.with_stack(0x90, |e, frame| {
        let last_corner = frame.byte_add(0x54);
        let sum = frame.byte_add(0x60);
        let halved = frame.byte_add(0x6c);
        let difference = frame.byte_add(0x78);
        e.call(
            VECTOR_CONSTRUCTOR,
            &args![frame, 0x0cu32, 8u32, POINT_NO_OP],
        );
        fill(e, frame);
        let two: f32 = e.global(TWO_FLOAT);
        let added = e.call(POINT_ADD, &args![frame, sum, last_corner]).u32();
        let centre = e.call(POINT_SCALE_DOWN, &args![added, halved, two]).u32();
        copy_point(e, out, centre);
        e.call(POINT_SUBTRACT, &args![frame, difference, last_corner]);
        e.call(POINT_LENGTH_SQUARED, &args![difference]).f32()
    })
}

// Translated from 00989e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: the centre (written to `out`) of the box around the
/// members' positions, and its squared diagonal (`fn_00989f40`).
pub fn fn_00989e20(e: &mut Engine, this: Ptr<CombatGroup>, out: Ptr) -> f32 {
    box_centre(e, out, |e, corners| fn_00989f40(e, this, corners, false))
}

// Translated from 00989eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: the centre (written to `out`) of the box around the
/// targets' positions, and its squared diagonal (`fn_0098a110`).
pub fn fn_00989eb0(e: &mut Engine, this: Ptr<CombatGroup>, out: Ptr) -> f32 {
    box_centre(e, out, |e, corners| fn_0098a110(e, this, corners, false))
}

// Translated from 00989f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: extends the eight `corners` (`NiPoint3`s) to contain the
/// members' positions (see `expand_corners`); `keep` leaves the corners'
/// current values as the starting point.
pub fn fn_00989f40(e: &mut Engine, this: Ptr<CombatGroup>, corners: Ptr, keep: bool) {
    expand_corners(e, this, corners, keep, MEMBER_COUNT, MEMBER_ACTOR_AT);
}

// Translated from 0098a110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: the same for the targets' positions.
pub fn fn_0098a110(e: &mut Engine, this: Ptr<CombatGroup>, corners: Ptr, keep: bool) {
    expand_corners(e, this, corners, keep, TARGET_COUNT, TARGET_ACTOR_AT);
}

// Translated from 0098a2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: whether the group has fleeing members and no
/// non-fleeing one.
pub fn fn_0098a2e0(e: &mut Engine, this: Ptr<CombatGroup>) -> bool {
    e.get(this, CombatGroup::iNonFleeingMemberCount) == 0
        && e.get(this, CombatGroup::iFleeingMemberCount) != 0
}

// Translated from 0098a310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: the actor's member entry's `iGroupStrategyAssignment`, or 0.
pub fn fn_0098a310(e: &mut Engine, this: Ptr<CombatGroup>, actor: Ptr) -> u32 {
    let member = fn_009865f0(e, this, actor);
    if member.is_null() {
        0
    } else {
        e.get(member, CombatMember::iGroupStrategyAssignment)
    }
}

// Translated from 0098a340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: `iGroupStrategyAssignment` of member number `index`.
pub fn fn_0098a340(e: &mut Engine, this: Ptr<CombatGroup>, index: u32) -> u32 {
    let member = e
        .call(
            MEMBER_AT,
            &args![this.byte_add(CombatGroup::MEMBER_ARRAY), index],
        )
        .ptr::<CombatMember>();
    e.get(member, CombatMember::iGroupStrategyAssignment)
}

// Translated from 0098a360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: sets the actor's member entry's
/// `iGroupStrategyAssignment`, when it has an entry.
pub fn fn_0098a360(e: &mut Engine, this: Ptr<CombatGroup>, actor: Ptr, assignment: u32) {
    let member = fn_009865f0(e, this, actor);
    if !member.is_null() {
        e.set(member, CombatMember::iGroupStrategyAssignment, assignment);
    }
}

// Translated from 0098a390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup::SetMemberGroupStrategyAssignment` (Xbox PDB): sets
/// `iGroupStrategyAssignment` of member number `index`.
pub fn combat_group_set_member_group_strategy_assignment(
    e: &mut Engine,
    this: Ptr<CombatGroup>,
    index: u32,
    assignment: u32,
) {
    let member = e
        .call(
            MEMBER_AT,
            &args![this.byte_add(CombatGroup::MEMBER_ARRAY), index],
        )
        .ptr::<CombatMember>();
    e.set(member, CombatMember::iGroupStrategyAssignment, assignment);
}

// Translated from 0098a3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: clears every member's `iGroupStrategyAssignment`.
pub fn fn_0098a3c0(e: &mut Engine, this: Ptr<CombatGroup>) {
    let count = e.call(MEMBER_COUNT, &args![this]).u32();
    for index in 0..count {
        let member = e
            .call(
                MEMBER_AT,
                &args![this.byte_add(CombatGroup::MEMBER_ARRAY), index],
            )
            .ptr::<CombatMember>();
        e.set(member, CombatMember::iGroupStrategyAssignment, 0);
    }
}

// Translated from 0098a410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup::FindMemberWithStrategyAssignment` (Xbox PDB): the actor of
/// the first member with `iGroupStrategyAssignment` equal to `assignment`
/// (`fn_0098a340`), or null.
pub fn combat_group_find_member_with_strategy_assignment(
    e: &mut Engine,
    this: Ptr<CombatGroup>,
    assignment: u32,
) -> Ptr {
    let count = e.call(MEMBER_COUNT, &args![this]).u32();
    for index in 0..count {
        if fn_0098a340(e, this, index) == assignment {
            return e.call(MEMBER_ACTOR_AT, &args![this, index]).ptr();
        }
    }
    Ptr::NULL
}

// Translated from 0098a470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup::CheckGroupStrategyChosenTimeStamp` (Xbox PDB): true when
/// no strategy has been chosen yet (`fLastGroupStrategyChosenTimeStamp` is
/// zero) or the last chosen index differs from `index`; otherwise whether
/// `limit` is below the age of that time stamp.
pub fn combat_group_check_group_strategy_chosen_time_stamp(
    e: &mut Engine,
    this: Ptr<CombatGroup>,
    index: u32,
    limit: f32,
) -> bool {
    let stamp = this.byte_add(CombatGroup::fLastGroupStrategyChosenTimeStamp.off);
    let value = time_stamp(e, stamp);
    let zero: f64 = e.global(ZERO_DOUBLE);
    if value as f64 == zero {
        return true;
    }
    if e.get(this, CombatGroup::iLastGroupStrategyChosenIndex) != index {
        return true;
    }
    let age = time_stamp_age(e, stamp);
    limit < age
}

// Translated from 0098a4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: the members' average of `a / b`, where `a` is the `float`
/// returned by virtual method 0xc and `b` the integer returned by virtual
/// method 0 of the object embedded in each member actor at +0xa4 (both
/// called with argument 0x10; the object looks like an actor value owner).
/// With no members the quotient `0 / 0` is returned.
pub fn fn_0098a4d0(e: &mut Engine, this: Ptr<CombatGroup>) -> f32 {
    let mut sum = 0.0f32;
    let count = e.call(MEMBER_COUNT, &args![this]).u32();
    for index in 0..count {
        let actor = e.call(MEMBER_ACTOR_AT, &args![this, index]).u32();
        let owner = actor + 0xa4;
        let value = e.vcall(owner, 0x0c, &args![0x10u32]).f32();
        let divisor = e.vcall(owner, 0x00, &args![0x10u32]).i32();
        sum = (value as f64 / divisor as f64 + sum as f64) as f32;
    }
    (sum as f64 / count as f64) as f32
}

// Translated from 00990500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup::ComputeMemberHealthPercentage` (Xbox PDB; the body is a
/// `CombatGroupData` method): the cached `fMemberHealthPercentage`; when it
/// is 0 it is first computed by `fn_0098a4d0` on `pCombatGroup` and stored.
pub fn fn_00990500(e: &mut Engine, this: Ptr<CombatGroupData>) -> f32 {
    let zero: f64 = e.global(ZERO_DOUBLE);
    if e.get(this, CombatGroupData::fMemberHealthPercentage) as f64 == zero {
        let group = e.get(this, CombatGroupData::pCombatGroup);
        let value = fn_0098a4d0(e, group);
        e.set(this, CombatGroupData::fMemberHealthPercentage, value);
    }
    e.get(this, CombatGroupData::fMemberHealthPercentage)
}

/// Reads a `NiPointer` (`[slot]`) as the game's getter `00559450` does.
fn ni_pointer_get(e: &mut Engine, slot: Ptr) -> u32 {
    e.call(NI_POINTER_GET, &args![slot]).u32()
}
// Translated from 0098a580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: the per-update step of the search (the engine map has no
/// name). The C++ exception frame is not translated.
///
/// First, when the group has initialised members: it counts the targets and
/// notes whether every one is "ready" (no targets, or at least as many
/// fleeing members as initialised ones). With a search running (`iSearchCount`
/// not zero) it resets the search (`fn_0098bae0`) when ready, or when some
/// target's last-detected time stamp is younger than the setting `011ce9b4`;
/// with no search running and not ready it starts one (`fn_0098add0`) when
/// every target's time stamp is at least as old as the setting `011ce528`
/// and some target has attackers.
///
/// Second, with a search running: drops the searching member when it is
/// done, and, when the search timer (+0xd8) has expired, advances the
/// line-of-sight grid (`+0xd4`), refreshes the search area (timer +0xe0),
/// rebuilds the grid's search location when the area is loaded and the grid
/// is missing or the area moved, feeds the members' locations and headings
/// to the grid, restarts the timer from the setting `011ce43c`, and has a
/// random member that is not the player start dialogue toward the target it
/// is fighting.
pub fn fn_0098a580(e: &mut Engine, this: Ptr<CombatGroup>) {
    if e.get(this, CombatGroup::iInitializedMemberCount) != 0 {
        let count = e.call(TARGET_COUNT, &args![this]).u32();
        let ready = count == 0
            || e.get(this, CombatGroup::iFleeingMemberCount)
                >= e.get(this, CombatGroup::iInitializedMemberCount);
        if e.call(SEARCH_RUNNING, &args![this]).bool() {
            if ready {
                fn_0098bae0(e, this);
            } else {
                for index in 0..count {
                    let target = fn_009871c0(e, this, index);
                    let age = e
                        .call(
                            TIME_STAMP_AGE,
                            &args![target.byte_add(CombatTarget::fLastDetectedTimeStamp.off)],
                        )
                        .f64();
                    let limit = float_setting(e, 0x011c_e9b4);
                    if limit as f64 > age {
                        fn_0098bae0(e, this);
                        break;
                    }
                }
            }
        } else if !ready {
            let mut old_enough = 0u32;
            let mut attacked = 0u32;
            for index in 0..count {
                let target = fn_009871c0(e, this, index);
                let age = e
                    .call(
                        TIME_STAMP_AGE,
                        &args![target.byte_add(CombatTarget::fLastDetectedTimeStamp.off)],
                    )
                    .f64();
                let limit = float_setting(e, 0x011c_e528);
                if limit as f64 <= age {
                    old_enough += 1;
                }
                if e.get(target, CombatTarget::sAttackerCount) != 0 {
                    attacked += 1;
                }
            }
            if attacked != 0 && old_enough == count {
                fn_0098add0(e, this);
            }
        }
    }
    if !e.call(SEARCH_RUNNING, &args![this]).bool() {
        return;
    }
    let member = e.get(this, CombatGroup::pSearchingMember);
    if !member.is_null()
        && (e.call(ACTOR_TEST_008B3B90, &args![member]).bool()
            || !e.call(ACTOR_IS_WAITING_ON_PATH, &args![member]).bool())
    {
        e.set(this, CombatGroup::pSearchingMember, Ptr::NULL);
    }
    let search_timer = this.byte_add(CombatGroup::SEARCH_UPDATE_TIMER);
    if !e.call(TIMER_EXPIRED, &args![search_timer]).bool() {
        return;
    }
    let grid_slot = this.byte_add(CombatGroup::spPathingLOSGridMap.off);
    if ni_pointer_get(e, grid_slot) != 0 {
        let grid = ni_pointer_get(e, grid_slot);
        let travelled = e.call(PATH_HANDLER_DISTANCE_TRAVELLED, &args![grid]).f64();
        let limit: f64 = e.global(PATH_PROGRESS_LIMIT);
        if travelled > limit {
            let grid = ni_pointer_get(e, grid_slot);
            e.call(LOS_GRID_MAP_STEP, &args![grid]);
            let searches = e.get(this, CombatGroup::iSearchCount);
            e.set(this, CombatGroup::iSearchCount, searches.wrapping_add(1));
            e.call(
                TIMER_EXPIRE_NOW,
                &args![this.byte_add(CombatGroup::SEARCH_AREA_UPDATE_TIMER)],
            );
        }
    }
    let mut area_moved = false;
    let area_timer = this.byte_add(CombatGroup::SEARCH_AREA_UPDATE_TIMER);
    if e.call(TIMER_EXPIRED, &args![area_timer]).bool() {
        area_moved = combat_group_update_search_area(e, this);
        let delay = float_setting(e, 0x011c_f870);
        e.call(TIMER_START, &args![area_timer, delay]);
    }
    let centre = this.byte_add(CombatGroup::SEARCH_CENTER);
    if e.call(WORLD_LOCATION_IS_LOADED, &args![centre]).bool() {
        if ni_pointer_get(e, grid_slot) == 0 || area_moved {
            rebuild_search_grid(e, this);
        }
    } else if ni_pointer_get(e, grid_slot) != 0 {
        e.call(NI_POINTER_ASSIGN, &args![grid_slot, 0u32]);
    }
    if ni_pointer_get(e, grid_slot) != 0 {
        feed_members_to_grid(e, this);
    }
    let delay = float_setting(e, 0x011c_e43c);
    e.call(TIMER_START, &args![search_timer, delay]);
    let members = e.call(MEMBER_COUNT, &args![this]).u32();
    let chosen_index = e.call(RANDOM_INDEX, &args![0u32, members]).u32();
    let chosen = e.call(MEMBER_ACTOR_AT, &args![this, chosen_index]).u32();
    let target = e.vcall(chosen, 0x42c, &args![]).u32();
    if chosen == e.global::<u32>(PLAYER) || target == 0 {
        return;
    }
    let entry = fn_009865d0(e, this, Ptr::new(target));
    let manager = e.global::<u32>(DIALOGUE_MANAGER);
    let mut seen = false;
    if !entry.is_null() {
        let stamp = e
            .call(
                TIME_STAMP_VALUE,
                &args![entry.byte_add(CombatTarget::fLastDetectedTimeStamp.off)],
            )
            .f32();
        seen = stamp != -e.global::<f32>(MAX_FLOAT);
    }
    e.call(
        START_DIALOGUE,
        &args![manager, chosen, target, 4u32, seen as u32, 0u32, 0u32],
    );
}

/// The block of `fn_0098a580` that (re)creates the line-of-sight grid: gets
/// the search center's cell or world; finds the closest navmesh point to the
/// focal point; resolves a `PathingLocation` there; makes the grid when the
/// group has none; feeds it the location and radius; sets its field +0x48
/// from the setting `011cf42c`; and flags the debug geometry for update.
fn rebuild_search_grid(e: &mut Engine, this: Ptr<CombatGroup>) {
    let centre = this.byte_add(CombatGroup::SEARCH_CENTER);
    let grid_slot = this.byte_add(CombatGroup::spPathingLOSGridMap.off);
    e.with_stack(0x20, |e, frame| {
        // frame + 0: first word, +4: second word, +8: the closest point
        // (a `NiPoint3`), +0x14: the `PathingLocation` (0x28 bytes)
        e.mem.set_u32(frame.addr(), 0);
        e.mem.set_u32(frame.addr() + 4, 0);
        if !e
            .call(
                WORLD_LOCATION_GET_CELL_OR_WORLD,
                &args![centre, frame.byte_add(4), frame],
            )
            .bool()
        {
            return;
        }
        let point = frame.byte_add(8);
        e.call(POINT_NO_OP, &args![point]);
        let first = e.mem.u32(frame.addr());
        let second = e.mem.u32(frame.addr() + 4);
        let focal = this.byte_add(CombatGroup::SEARCH_FOCAL_POINT);
        if !e
            .call(
                FIND_CLOSEST_POINT_ON_NAVMESH,
                &args![first, second, focal, point],
            )
            .bool()
        {
            return;
        }
        e.with_stack(0x28, |e, location| {
            e.call(
                PATHING_LOCATION_FROM_POINT,
                &args![location, point, second, first],
            );
            if e.call(PATHING_LOCATION_RESOLVE, &args![location, 0u32])
                .bool()
            {
                let centre_point = e.call(POINT_NO_OP, &args![centre]).u32();
                e.call(PATHING_LOCATION_SET_POINT, &args![location, centre_point]);
                if ni_pointer_get(e, grid_slot) == 0 {
                    let memory = e.call(OPERATOR_NEW, &args![0x54u32]).u32();
                    let grid = if memory != 0 {
                        e.call(LOS_GRID_MAP_CONSTRUCT, &args![memory]).u32()
                    } else {
                        0
                    };
                    e.call(NI_POINTER_ASSIGN, &args![grid_slot, grid]);
                }
                let radius = e.get(this, CombatGroup::fSearchRadius);
                let grid = ni_pointer_get(e, grid_slot);
                e.call(LOS_GRID_MAP_UPDATE, &args![grid, location, radius]);
                let setting = float_setting(e, 0x011c_f42c);
                let grid = Ptr::new(ni_pointer_get(e, grid_slot));
                fn_0098adb0(e, grid, setting);
                e.set(this, CombatGroup::bUpdateSearchDebugGeometry, true);
            }
            e.call(PATHING_LOCATION_DESTRUCT, &args![location]);
        });
    });
}

/// The block of `fn_0098a580` that gives the grid the members' locations
/// and headings: for every member whose space is the search center's space
/// and whose `PathingLocation` has navmesh information (`fn_0098ad70`), the
/// location is added to one array and the clamped heading (virtual method
/// 0x2bc of the actor, argument 0) to another; the grid takes both
/// (`006d7ca0`), and the debug geometry is flagged when the grid's distance
/// travelled changed.
fn feed_members_to_grid(e: &mut Engine, this: Ptr<CombatGroup>) {
    let grid_slot = this.byte_add(CombatGroup::spPathingLOSGridMap.off);
    let members = e.call(MEMBER_COUNT, &args![this]).u32();
    e.with_stack(0x14, |e, locations| {
        e.call(
            LOCATION_ARRAY_CONSTRUCT,
            &args![locations, members, 0u32, 0u32],
        );
        e.with_stack(0x14, |e, angles| {
            e.call(ANGLE_ARRAY_CONSTRUCT, &args![angles, members, 0u32, 0u32]);
            for index in 0..members {
                let actor = e.call(MEMBER_ACTOR_AT, &args![this, index]).u32();
                let space = e.call(REFERENCE_SPACE, &args![actor]).u32();
                let centre_space = e
                    .call(FORM_ID, &args![this.byte_add(CombatGroup::SEARCH_CENTER)])
                    .u32();
                if space != centre_space {
                    continue;
                }
                e.with_stack(0x28, |e, location| {
                    e.call(PATHING_LOCATION_FROM_ACTOR, &args![location, actor]);
                    if fn_0098ad70(e, location) {
                        e.call(LOCATION_ARRAY_ADD, &args![locations, location]);
                        let heading = e.vcall(actor, 0x2bc, &args![0u32]).f32();
                        let clamped = e.call(CLAMP_ANGLE, &args![heading]).f32();
                        e.with_stack(4, |e, value| {
                            e.mem.set_f32(value.addr(), clamped);
                            e.call(ANGLE_ARRAY_ADD, &args![angles, value]);
                        });
                    }
                    e.call(PATHING_LOCATION_DESTRUCT, &args![location]);
                });
            }
            if e.call(ARRAY_SIZE, &args![locations]).u32() != 0 {
                let grid = ni_pointer_get(e, grid_slot);
                let before = e.call(PATH_HANDLER_DISTANCE_TRAVELLED, &args![grid]).f32();
                e.call(LOS_GRID_MAP_APPLY, &args![grid_slot, locations, angles]);
                let grid = ni_pointer_get(e, grid_slot);
                let after = e.call(PATH_HANDLER_DISTANCE_TRAVELLED, &args![grid]).f64();
                if before as f64 != after {
                    e.set(this, CombatGroup::bUpdateSearchDebugGeometry, true);
                }
            }
            e.call(ANGLE_ARRAY_DESTRUCT, &args![angles]);
        });
        e.call(LOCATION_ARRAY_DESTRUCT, &args![locations]);
    });
}

// Translated from 0098ad70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingLocation`: whether either of the words at +0x10 and +0x14 is not
/// zero (the navmesh information a resolved location carries).
pub fn fn_0098ad70(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0x10) != 0 || e.mem.u32(this.addr() + 0x14) != 0
}

// Translated from 0098adb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingLOSGridMap`: stores the `float` `value` at +0x48.
pub fn fn_0098adb0(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_f32(this.addr() + 0x48, x87_quiet(value));
}

// Translated from 0098add0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: starts the search. Builds the search locations and the
/// search area and teleport doors (`fn_0098c0d0`, `fn_0098b100`,
/// `BuildSearchTeleportDoorArray`), sets `iSearchCount` to 1, starts the
/// search timer (+0xd8) with 0.5 and the area timer (+0xe0) with the setting
/// `011cf870`, stamps `fSearchStartedTimeStamp` with the game time, clears
/// every target's `sLastSearchNoticed`, and starts two of the detection
/// dialogue timers (+0x58 and +0x60) with random delays between the settings
/// `011cf24c` and `011ce35c`. The scan over the targets only tracks the two
/// newest time stamps in locals it never uses.
pub fn fn_0098add0(e: &mut Engine, this: Ptr<CombatGroup>) {
    let targets = e.call(TARGET_COUNT, &args![this]).u32();
    fn_0098c0d0(e, this);
    fn_0098b100(e, this);
    combat_group_build_search_teleport_door_array(e, this);
    e.set(this, CombatGroup::iSearchCount, 1);
    let half: f32 = e.global(HALF_FLOAT);
    e.call(
        TIMER_START,
        &args![this.byte_add(CombatGroup::SEARCH_UPDATE_TIMER), half],
    );
    let area_delay = float_setting(e, 0x011c_f870);
    e.call(
        TIMER_START,
        &args![
            this.byte_add(CombatGroup::SEARCH_AREA_UPDATE_TIMER),
            area_delay
        ],
    );
    e.with_stack(0x20, |e, locals| {
        // The game's locals: the two newest stamps and the actors they
        // belong to (never read again), and a scratch stamp.
        let newest = locals;
        let newest_actor = locals.byte_add(4);
        let second_newest = locals.byte_add(8);
        let second_actor = locals.byte_add(0xc);
        let scratch = locals.byte_add(0x10);
        let now = e.call(GAME_TIME_NOW, &args![]).f32();
        e.call(TIME_STAMP_SET, &args![scratch, now]);
        let started = e.mem.f32(scratch.addr());
        e.set(this, CombatGroup::fSearchStartedTimeStamp, started);
        let lowest = -e.global::<f32>(MAX_FLOAT);
        e.call(TIME_STAMP_SET, &args![newest, lowest]);
        e.call(TIME_STAMP_SET, &args![second_newest, lowest]);
        e.mem.set_u32(newest_actor.addr(), 0);
        e.mem.set_u32(second_actor.addr(), 0);
        for index in 0..targets {
            let target = fn_009871c0(e, this, index);
            e.set(target, CombatTarget::sLastSearchNoticed, 0);
            let detected_at = target.byte_add(CombatTarget::fLastDetectedTimeStamp.off);
            let noticed_at = target.byte_add(CombatTarget::fLastNoticedTimeStamp.off);
            let detected = time_stamp(e, detected_at) as f64;
            if (time_stamp(e, newest) as f64) < detected {
                let bits = e.mem.u32(detected_at.addr());
                e.mem.set_u32(newest.addr(), bits);
                let actor = e.mem.u32(target.addr());
                e.mem.set_u32(newest_actor.addr(), actor);
            } else {
                let noticed = time_stamp(e, noticed_at) as f64;
                if (time_stamp(e, second_newest) as f64) < noticed {
                    let bits = e.mem.u32(noticed_at.addr());
                    e.mem.set_u32(second_newest.addr(), bits);
                    let actor = e.mem.u32(target.addr());
                    e.mem.set_u32(second_actor.addr(), actor);
                }
            }
        }
    });
    for timer in [
        CombatGroup::DETECTION_DIALOGUE_TIMERS,
        CombatGroup::DETECTION_DIALOGUE_TIMERS + 8,
    ] {
        let high = float_setting(e, 0x011c_e35c);
        let low = float_setting(e, 0x011c_f24c);
        let delay = e.call(RANDOM_FLOAT, &args![low, high]).f32();
        e.call(TIMER_START, &args![this.byte_add(timer), delay]);
    }
}

// Translated from 0098afb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup::UpdateSearchArea` (Xbox PDB): rebuilds the search
/// locations (`fn_0098c0d0`) and recomputes the search center and radius
/// (`fn_0098b100`). Returns, and rebuilds the teleport doors
/// (`BuildSearchTeleportDoorArray`) for, "the area moved": the radius
/// changed by more than 128 either way, or the center moved by more than
/// 64 (squared distance above 4096).
pub fn combat_group_update_search_area(e: &mut Engine, this: Ptr<CombatGroup>) -> bool {
    fn_0098c0d0(e, this);
    e.with_stack(0x10, |e, old_centre| {
        copy_location(e, old_centre, this.byte_add(CombatGroup::SEARCH_CENTER));
        let old_radius = e.get(this, CombatGroup::fSearchRadius);
        fn_0098b100(e, this);
        let new_radius = e.get(this, CombatGroup::fSearchRadius) as f64;
        let margin: f64 = e.global(SEARCH_AREA_MARGIN);
        let old = old_radius as f64;
        let mut moved = false;
        if old > new_radius + margin || old < new_radius - margin {
            moved = true;
        } else {
            let squared = e
                .call(
                    DISTANCE_SQUARED,
                    &args![old_centre, this.byte_add(CombatGroup::SEARCH_CENTER)],
                )
                .f64();
            let limit: f64 = e.global(SEARCH_AREA_MOVE_LIMIT);
            if squared > limit {
                moved = true;
            }
        }
        if moved {
            combat_group_build_search_teleport_door_array(e, this);
        }
        moved
    })
}

// Translated from 0098b070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: looks up the two radius settings for a search around
/// `location`. The row of the table at `011a4d38` (12 bytes per row, two
/// setting pointers in use per column) is 0 when the location has a space
/// whose type byte (`00401170`) is 0x39 and 1 otherwise; the column is 1
/// when `iSearchCount` is above 1 and 0 otherwise. The first setting goes to
/// `out_first`, the second to `out_second` (both `float`s).
pub fn fn_0098b070(
    e: &mut Engine,
    this: Ptr<CombatGroup>,
    location: Ptr,
    out_first: Ptr,
    out_second: Ptr,
) {
    let mut row = 1u32;
    if e.call(FORM_ID, &args![location]).u32() != 0 {
        let space = e.call(FORM_ID, &args![location]).u32();
        if e.call(FORM_TYPE_BYTE, &args![space]).u32() == 0x39 {
            row = 0;
        }
    }
    let column = (e.get(this, CombatGroup::iSearchCount) >= 2) as u32;
    let base = SEARCH_RADIUS_TABLE + row * 12 + column * 4;
    let setting = e.mem.u32(base);
    let value = e.call(SETTING_FLOAT_VALUE, &args![setting]).u32();
    let first = e.mem.f32(value);
    e.mem.set_f32(out_first.addr(), first);
    let setting = e.mem.u32(base + 4);
    let value = e.call(SETTING_FLOAT_VALUE, &args![setting]).u32();
    let second = e.mem.f32(value);
    e.mem.set_f32(out_second.addr(), second);
}

// Translated from 0098b100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: recomputes the search center (`SearchCenter`, +0xf0), the
/// focal point (+0x100) and the search radius (+0x10c).
///
/// With search locations: the center is the average of the locations within
/// the larger radius setting of the best one (the best is the entry with the
/// highest `fn_00985bb0`; the average weighs the locations by it) and takes
/// its space; the radius is derived from the farthest location in the same
/// space (see `finish_radius`). With a single location the center and focal
/// point are that location's. With no locations and an empty center space
/// (`pSpace` null), the same is done with the members' world locations: the
/// center is the average of those within the radius of the member with the
/// lowest process type (below 4), and a single member gives its location.
pub fn fn_0098b100(e: &mut Engine, this: Ptr<CombatGroup>) {
    e.with_stack(8, |e, radii| {
        e.mem.set_f32(radii.addr(), 0.0);
        e.mem.set_f32(radii.addr() + 4, 0.0);
        let locations = e.call(SEARCH_LOCATION_COUNT, &args![this]).u32();
        if locations != 0 {
            centre_from_locations(e, this, locations, radii);
        } else if e
            .call(FORM_ID, &args![this.byte_add(CombatGroup::SEARCH_CENTER)])
            .u32()
            == 0
        {
            centre_from_members(e, this, radii);
        }
    });
}

/// `fn_0098b100` with search locations (`locations` of them); `radii` holds
/// the two settings of `fn_0098b070` (smaller at +0, larger at +4).
fn centre_from_locations(e: &mut Engine, this: Ptr<CombatGroup>, locations: u32, radii: Ptr) {
    let centre_field = this.byte_add(CombatGroup::SEARCH_CENTER);
    let focal_field = this.byte_add(CombatGroup::SEARCH_FOCAL_POINT);
    let mut best_score = -e.global::<f32>(MAX_FLOAT);
    if locations <= 1 {
        let location = e
            .call(SEARCH_LOCATION_AT, &args![this, 0u32])
            .ptr::<CombatSearchLocation>();
        let _ = fn_00985bb0(e, location);
        copy_location(e, centre_field, location.cast());
        let point = e.call(POINT_NO_OP, &args![location]).u32();
        copy_point(e, focal_field, point);
        fn_0098b070(e, this, location.cast(), radii, radii.byte_add(4));
        let smaller = e.mem.f32(radii.addr());
        e.set(this, CombatGroup::fSearchRadius, smaller);
        return;
    }
    let mut best_index = 0u32;
    for index in 0..locations {
        let location = e
            .call(SEARCH_LOCATION_AT, &args![this, index])
            .ptr::<CombatSearchLocation>();
        let score = fn_00985bb0(e, location);
        if best_score < score {
            best_score = score;
            best_index = index;
        }
    }
    let best = e
        .call(SEARCH_LOCATION_AT, &args![this, best_index])
        .ptr::<CombatSearchLocation>();
    e.with_stack(0x10, |e, centre| {
        copy_point(e, centre, ZERO_POINT);
        let mut total = 0.0f32;
        fn_0098b070(e, this, best.cast(), radii, radii.byte_add(4));
        let larger = e.mem.f32(radii.addr() + 4);
        let larger_squared = larger as f64 * larger as f64;
        for index in 0..locations {
            let location = e
                .call(SEARCH_LOCATION_AT, &args![this, index])
                .ptr::<CombatSearchLocation>();
            let squared = e.call(DISTANCE_SQUARED, &args![best, location]).f64();
            if larger_squared < squared {
                continue;
            }
            let score = fn_00985bb0(e, location);
            e.with_stack(0xc, |e, scaled| {
                let point = e.call(POINT_NO_OP, &args![location]).u32();
                let result = e.call(POINT_SCALE, &args![point, scaled, score]).u32();
                e.call(POINT_ACCUMULATE, &args![centre, result]);
            });
            total = (total as f64 + score as f64) as f32;
        }
        e.call(POINT_DIVIDE_IN_PLACE, &args![centre, total]);
        let space = e.call(FORM_ID, &args![best]).u32();
        bgs_world_location_set_location(e, centre_field, centre, space);
    });
    let point = e.call(POINT_NO_OP, &args![best]).u32();
    copy_point(e, focal_field, point);
    let mut farthest = 0.0f32;
    let larger = e.mem.f32(radii.addr() + 4);
    let larger_squared = larger as f64 * larger as f64;
    for index in 0..locations {
        let location = e
            .call(SEARCH_LOCATION_AT, &args![this, index])
            .ptr::<CombatSearchLocation>();
        if e.call(FORM_ID, &args![location]).u32() != e.call(FORM_ID, &args![centre_field]).u32() {
            continue;
        }
        let squared = e
            .call(DISTANCE_SQUARED, &args![centre_field, location])
            .f32();
        if farthest < squared {
            farthest = squared;
            if farthest as f64 > larger_squared {
                break;
            }
        }
    }
    finish_radius(e, this, farthest, radii);
}

/// `fn_0098b100` without search locations: from the members.
fn centre_from_members(e: &mut Engine, this: Ptr<CombatGroup>, radii: Ptr) {
    let centre_field = this.byte_add(CombatGroup::SEARCH_CENTER);
    let focal_field = this.byte_add(CombatGroup::SEARCH_FOCAL_POINT);
    let members = e.call(MEMBER_COUNT, &args![this]).u32();
    if members <= 1 {
        let actor = e.call(MEMBER_ACTOR_AT, &args![this, 0u32]).u32();
        e.with_stack(0x10, |e, location| {
            let world = e
                .call(GET_WORLD_LOCATION, &args![actor, location])
                .ptr::<()>();
            copy_location(e, centre_field, world);
        });
        let point = e.call(POINT_NO_OP, &args![centre_field]).u32();
        copy_point(e, focal_field, point);
        fn_0098b070(e, this, centre_field, radii, radii.byte_add(4));
        let smaller = e.mem.f32(radii.addr());
        e.set(this, CombatGroup::fSearchRadius, smaller);
        return;
    }
    let mut best_type = 4i32;
    let mut best_actor = 0u32;
    for index in 0..members {
        let actor = e.call(MEMBER_ACTOR_AT, &args![this, index]).u32();
        let process_type = e.call(PROCESS_TYPE, &args![actor]).i32();
        if process_type < best_type {
            best_type = process_type;
            best_actor = actor;
        }
    }
    e.with_stack(0x20, |e, frame| {
        let best_location = frame;
        let location = frame.byte_add(0x10);
        e.call(GET_WORLD_LOCATION, &args![best_actor, best_location]);
        e.with_stack(0x10, |e, centre| {
            copy_point(e, centre, ZERO_POINT);
            let mut inside = 0u32;
            fn_0098b070(e, this, best_location, radii, radii.byte_add(4));
            let larger = e.mem.f32(radii.addr() + 4);
            let larger_squared = larger as f64 * larger as f64;
            for index in 0..members {
                let actor = e.call(MEMBER_ACTOR_AT, &args![this, index]).u32();
                e.call(GET_WORLD_LOCATION, &args![actor, location]);
                let squared = e
                    .call(DISTANCE_SQUARED, &args![best_location, location])
                    .f64();
                if larger_squared < squared {
                    continue;
                }
                let point = e.call(POINT_NO_OP, &args![location]).u32();
                e.call(POINT_ACCUMULATE, &args![centre, point]);
                inside += 1;
            }
            e.call(POINT_DIVIDE_IN_PLACE, &args![centre, inside as f32]);
            let space = e.call(FORM_ID, &args![best_location]).u32();
            bgs_world_location_set_location(e, centre_field, centre, space);
        });
        let point = e.call(POINT_NO_OP, &args![best_location]).u32();
        copy_point(e, focal_field, point);
        let mut farthest = 0.0f32;
        let larger = e.mem.f32(radii.addr() + 4);
        let larger_squared = larger as f64 * larger as f64;
        for index in 0..members {
            let actor = e.call(MEMBER_ACTOR_AT, &args![this, index]).u32();
            e.call(GET_WORLD_LOCATION, &args![actor, location]);
            if e.call(FORM_ID, &args![location]).u32()
                != e.call(FORM_ID, &args![centre_field]).u32()
            {
                continue;
            }
            let squared = e
                .call(DISTANCE_SQUARED, &args![centre_field, location])
                .f32();
            if farthest < squared {
                farthest = squared;
                if farthest as f64 > larger_squared {
                    break;
                }
            }
        }
        finish_radius(e, this, farthest, radii);
    });
}

/// The end of `fn_0098b100` with several locations or members: the radius
/// is the larger setting when the farthest squared distance exceeds its
/// square; otherwise the square root of that distance plus the setting
/// `011ce7b0`, limited to the larger setting (`0040ebd0`) and then to at
/// least the smaller one (`00404010`).
fn finish_radius(e: &mut Engine, this: Ptr<CombatGroup>, farthest: f32, radii: Ptr) {
    let smaller = e.mem.f32(radii.addr());
    let larger = e.mem.f32(radii.addr() + 4);
    if farthest as f64 > larger as f64 * larger as f64 {
        e.set(this, CombatGroup::fSearchRadius, larger);
        return;
    }
    let root = e.call(SQUARE_ROOT, &args![farthest]).f64();
    let padding = float_setting(e, 0x011c_e7b0);
    let radius = (padding as f64 + root) as f32;
    e.set(this, CombatGroup::fSearchRadius, radius);
    let radius = e.get(this, CombatGroup::fSearchRadius);
    let limited = e.call(FLOAT_MIN, &args![radius, larger]).f32();
    e.set(this, CombatGroup::fSearchRadius, limited);
    let radius = e.get(this, CombatGroup::fSearchRadius);
    let raised = e.call(FLOAT_MAX, &args![radius, smaller]).f32();
    e.set(this, CombatGroup::fSearchRadius, raised);
}

// Translated from 0098b8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup::BuildSearchTeleportDoorArray` (Xbox PDB): rebuilds
/// `SearchTeleportDoors`. Collects the teleport doors around the search
/// center with the radius `fSearchRadius` and the smaller radius setting
/// around every member whose distance (the unsquared one, compared with the
/// larger setting squared as the game does) is not below it, sorts the
/// collected doors, removes duplicates, and adds the ones that are not yet
/// in the group's door array (`fn_0098c600`).
pub fn combat_group_build_search_teleport_door_array(e: &mut Engine, this: Ptr<CombatGroup>) {
    e.with_stack(0x30, |e, frame| {
        let array = frame;
        let first = frame.byte_add(0x10);
        let second = frame.byte_add(0x14);
        let location = frame.byte_add(0x20);
        e.call(POINTER_ARRAY_CONSTRUCT, &args![array]);
        e.mem.set_f32(first.addr(), 0.0);
        e.mem.set_f32(second.addr(), 0.0);
        let centre = this.byte_add(CombatGroup::SEARCH_CENTER);
        fn_0098b070(e, this, centre, first, second);
        let radius = e.get(this, CombatGroup::fSearchRadius);
        let point = e.call(POINT_NO_OP, &args![centre]).u32();
        let space = e.call(FORM_ID, &args![centre]).u32();
        e.call(COLLECT_TELEPORT_DOORS, &args![space, point, radius, array]);
        let members = e.call(MEMBER_COUNT, &args![this]).u32();
        let larger = e.mem.f32(second.addr());
        let larger_squared = larger as f64 * larger as f64;
        for index in 0..members {
            let actor = e.call(MEMBER_ACTOR_AT, &args![this, index]).u32();
            e.call(GET_WORLD_LOCATION, &args![actor, location]);
            let distance = e
                .call(WORLD_LOCATION_DISTANCE, &args![centre, location])
                .f64();
            if larger_squared > distance {
                continue;
            }
            let smaller = e.mem.f32(first.addr());
            let point = e.call(POINT_NO_OP, &args![location]).u32();
            let space = e.call(FORM_ID, &args![location]).u32();
            e.call(COLLECT_TELEPORT_DOORS, &args![space, point, smaller, array]);
        }
        e.call(ARRAY_SORT, &args![array, COMPARE_WORDS]);
        let mut index = 0u32;
        while index < e.call(ARRAY_SIZE, &args![array]).u32() {
            let slot = e.call(CLUSTER_AT, &args![array, index]).u32();
            let door = e.mem.u32(slot);
            let mut duplicates = 0u32;
            let mut next = index + 1;
            while next < e.call(ARRAY_SIZE, &args![array]).u32() {
                let slot = e.call(CLUSTER_AT, &args![array, next]).u32();
                if e.mem.u32(slot) != door {
                    break;
                }
                duplicates += 1;
                next += 1;
            }
            if duplicates != 0 {
                e.call(
                    POINTER_ARRAY_REMOVE_RANGE,
                    &args![array, index + 1, duplicates, 0u32],
                );
            }
            index += 1;
        }
        let mut index = 0u32;
        while index < e.call(ARRAY_SIZE, &args![array]).u32() {
            let slot = e.call(CLUSTER_AT, &args![array, index]).u32();
            let door = e.mem.u32(slot);
            fn_0098c600(e, this, door);
            index += 1;
        }
        e.call(POINTER_ARRAY_DESTRUCT, &args![array]);
    });
}

// Translated from 0098bae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: ends the search. Releases the line-of-sight grid and the
/// search debug geometry, clears the debug-update flag, resets the two
/// search timers, sets `fSearchStartedTimeStamp` to the lowest time stamp,
/// zeroes `iSearchCount` and the radius, clears the center and the focal
/// point, and empties the search location and teleport door arrays.
pub fn fn_0098bae0(e: &mut Engine, this: Ptr<CombatGroup>) {
    e.call(
        NI_POINTER_ASSIGN,
        &args![this.byte_add(CombatGroup::spPathingLOSGridMap.off), 0u32],
    );
    e.call(
        NI_POINTER_ASSIGN,
        &args![this.byte_add(CombatGroup::spSearchDebugGeometry.off), 0u32],
    );
    e.set(this, CombatGroup::bUpdateSearchDebugGeometry, false);
    e.call(
        TIMER_RESET,
        &args![this.byte_add(CombatGroup::SEARCH_UPDATE_TIMER)],
    );
    e.call(
        TIMER_RESET,
        &args![this.byte_add(CombatGroup::SEARCH_AREA_UPDATE_TIMER)],
    );
    let lowest = -e.global::<f32>(MAX_FLOAT);
    e.with_stack(4, |e, stamp| {
        e.call(TIME_STAMP_SET, &args![stamp, lowest]);
        let value = e.mem.f32(stamp.addr());
        e.set(this, CombatGroup::fSearchStartedTimeStamp, value);
    });
    e.set(this, CombatGroup::iSearchCount, 0);
    e.set(this, CombatGroup::fSearchRadius, 0.0);
    fn_0098bbc0(e, this.byte_add(CombatGroup::SEARCH_CENTER));
    copy_point(
        e,
        this.byte_add(CombatGroup::SEARCH_FOCAL_POINT),
        ZERO_POINT,
    );
    e.call(
        ARRAY_CLEAR,
        &args![this.byte_add(CombatGroup::SEARCH_LOCATIONS), 1u32],
    );
    e.call(
        ARRAY_CLEAR,
        &args![this.byte_add(CombatGroup::SEARCH_TELEPORT_DOORS), 1u32],
    );
}

// Translated from 0098bbc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSWorldLocation` constructor from the default point: the three
/// coordinates of the global at `011f426c`, and a null space.
pub fn fn_0098bbc0(e: &mut Engine, this: Ptr) {
    copy_point(e, this, ZERO_POINT);
    e.mem.set_u32(this.addr() + 0x0c, 0);
}

// Translated from 0098bc00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup::FindSearchLocation` (Xbox PDB): the search location with
/// the best score for `actor`, or null. An entry qualifies when its
/// remaining time (`fn_00985bb0`) plus 5 (when `reference` is given, shares
/// the entry's space and has the same point as the entry) reaches
/// `minimum`, its `fn_00985b40` score for the actor's location does not
/// lose to the best so far, and, when the actor has a process, the combat
/// state's movement check (`CheckMovement(location, 1, 0)`) passes. Ties go
/// to the later entry.
pub fn combat_group_find_search_location(
    e: &mut Engine,
    this: Ptr<CombatGroup>,
    actor: Ptr,
    minimum: u8,
    reference: Ptr,
) -> Ptr {
    let process = e.vcall(actor.addr(), 0x428, &args![]).u32();
    e.with_stack(0x10, |e, location| {
        e.call(GET_WORLD_LOCATION, &args![actor, location]);
        let mut best_index = u32::MAX;
        let mut best = 0.0f32;
        let count = e.call(SEARCH_LOCATION_COUNT, &args![this]).u32();
        for index in 0..count {
            let entry = e
                .call(SEARCH_LOCATION_AT, &args![this, index])
                .ptr::<CombatSearchLocation>();
            let score = fn_00985bb0(e, entry);
            let mut bonus = 0.0f32;
            if !reference.is_null()
                && e.call(FORM_ID, &args![entry]).u32() == e.call(FORM_ID, &args![reference]).u32()
            {
                let reference_point = e.call(POINT_NO_OP, &args![reference]).u32();
                let entry_point = e.call(POINT_NO_OP, &args![entry]).u32();
                if e.call(POINTS_EQUAL, &args![entry_point, reference_point])
                    .bool()
                {
                    let five: f64 = e.global(FIVE_DOUBLE);
                    bonus = (bonus as f64 + five) as f32;
                }
            }
            if minimum as f64 > score as f64 + bonus as f64 {
                continue;
            }
            // The game adds `bonus` to this and stores it in a local it never reads.
            let _ = fn_00985b40(e, entry, location);
            if best > score {
                continue;
            }
            if process != 0 {
                let state = e.call(PROCESS_COMBAT_STATE, &args![process]).u32();
                if !e
                    .call(
                        COMBAT_STATE_CHECK_MOVEMENT,
                        &args![state, entry, 1u32, 0u32],
                    )
                    .bool()
                {
                    continue;
                }
            }
            best_index = index;
            best = score;
        }
        if best_index == u32::MAX {
            Ptr::NULL
        } else {
            e.call(SEARCH_LOCATION_AT, &args![this, best_index]).ptr()
        }
    })
}

// Translated from 0098bd70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: ages the search state near `position` (a
/// `BGSWorldLocation`). The squared setting `011cf684` is the range; the
/// scale applied to the vertical axis is (1, 1, 0.25). For every search
/// location whose scaled distance to `position` is within the range:
/// when its remaining time exceeds 30 the excess is taken off its `fScore`;
/// and when it belongs to a target (`iTargetID`) that the group has, each of
/// the target's three time stamps (last detected +0x50, last noticed +0x58,
/// last attacked member +0x60) whose `fn_0098c060` excess is above 30 and
/// whose location (+0x18, +0x08, +0x38) is within the range as well is
/// moved back by that excess over 30.
pub fn fn_0098bd70(e: &mut Engine, this: Ptr<CombatGroup>, position: Ptr) {
    let first = float_setting(e, 0x011c_f684);
    let second = float_setting(e, 0x011c_f684);
    let range = (first as f64 * second as f64) as f32;
    e.with_stack(0x20, |e, frame| {
        let scale = frame;
        let outs = frame.byte_add(0x10);
        let (excess_detected, excess_noticed, excess_attacked) =
            (outs, outs.byte_add(4), outs.byte_add(8));
        let quarter: f32 = e.global(QUARTER_FLOAT);
        e.call(POINT_CONSTRUCT, &args![scale, 1.0f32, 1.0f32, quarter]);
        let count = e.call(SEARCH_LOCATION_COUNT, &args![this]).u32();
        let thirty: f64 = e.global(THIRTY_DOUBLE);
        for index in 0..count {
            let entry = e
                .call(SEARCH_LOCATION_AT, &args![this, index])
                .ptr::<CombatSearchLocation>();
            let distance = e
                .call(
                    WORLD_LOCATION_SCALED_DISTANCE,
                    &args![entry, position, scale],
                )
                .f64();
            let within = range as f64 > distance;
            if !within {
                continue;
            }
            let remaining = fn_00985bb0(e, entry);
            if remaining as f64 > thirty {
                let score = e.get(entry, CombatSearchLocation::fScore);
                let lowered = (score as f64 - (remaining as f64 - thirty)) as f32;
                e.set(entry, CombatSearchLocation::fScore, lowered);
            }
            let target_id = e.get(entry, CombatSearchLocation::iTargetID);
            if target_id == 0 {
                continue;
            }
            let target = fn_00986610(e, this, target_id);
            if target.is_null() {
                continue;
            }
            fn_0098c060(
                e,
                this,
                target,
                excess_detected,
                excess_noticed,
                excess_attacked,
            );
            // (excess slot, location of the target, time stamp field)
            for (excess, location, stamp) in [
                (
                    excess_detected,
                    CombatTarget::LAST_DETECTED_LOCATION,
                    CombatTarget::fLastDetectedTimeStamp.off,
                ),
                (
                    excess_noticed,
                    CombatTarget::LAST_NOTICED_LOCATION,
                    CombatTarget::fLastNoticedTimeStamp.off,
                ),
                (
                    excess_attacked,
                    CombatTarget::LAST_ATTACKED_MEMBER_LOCATION,
                    CombatTarget::fLastAttackedMemberTimeStamp.off,
                ),
            ] {
                let excess_value = e.mem.f32(excess.addr());
                let exceeds = excess_value as f64 > thirty;
                if !exceeds {
                    continue;
                }
                let target_location = target.byte_add(location);
                let distance = e
                    .call(
                        WORLD_LOCATION_SCALED_DISTANCE,
                        &args![target_location, position, scale],
                    )
                    .f64();
                let within = range as f64 > distance;
                if !within {
                    continue;
                }
                let stamp_at = target.byte_add(stamp);
                let stamp_value = e.call(TIME_STAMP_VALUE, &args![stamp_at]).f64();
                let moved = (stamp_value - (excess_value as f64 - thirty)) as f32;
                e.with_stack(4, |e, scratch| {
                    e.call(TIME_STAMP_SET, &args![scratch, moved]);
                    let bits = e.mem.u32(scratch.addr());
                    e.mem.set_u32(stamp_at.addr(), bits);
                });
            }
        }
    });
}

// Translated from 0098bfb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: adds a search location: a copy of `location`, stamped
/// with the game time, with `iTargetID` `target_id` and `fScore` `score`.
pub fn fn_0098bfb0(
    e: &mut Engine,
    this: Ptr<CombatGroup>,
    location: Ptr,
    score: f32,
    target_id: u32,
) {
    let array = this.byte_add(CombatGroup::SEARCH_LOCATIONS);
    let index = e.call(SEARCH_LOCATION_ADD, &args![array]).u32();
    let entry = e
        .call(SEARCH_LOCATION_SLOT, &args![array, index])
        .ptr::<()>();
    copy_location(e, entry, location);
    let now = e.call(GAME_TIME_NOW, &args![]).f32();
    e.with_stack(4, |e, stamp| {
        e.call(TIME_STAMP_SET, &args![stamp, now]);
        let bits = e.mem.u32(stamp.addr());
        let entry = e.call(SEARCH_LOCATION_SLOT, &args![array, index]).u32();
        e.mem
            .set_u32(entry + CombatSearchLocation::fTimeStamp.off, bits);
    });
    let entry = e.call(SEARCH_LOCATION_SLOT, &args![array, index]).u32();
    e.mem
        .set_u32(entry + CombatSearchLocation::iTargetID.off, target_id);
    let entry = e.call(SEARCH_LOCATION_SLOT, &args![array, index]).u32();
    e.mem
        .set_f32(entry + CombatSearchLocation::fScore.off, score);
}

// Translated from 0098c060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: how far each of a target's time stamps is below its
/// expiry: `out_detected` = (setting `011ce528` + 70 - age of the last
/// detected stamp +0x50), `out_noticed` = 30 - age of the last noticed
/// stamp +0x58, `out_attacked` = 70 - age of the last attacked member stamp
/// +0x60. (The outputs are `float`s.)
pub fn fn_0098c060(
    e: &mut Engine,
    _this: Ptr<CombatGroup>,
    target: Ptr<CombatTarget>,
    out_detected: Ptr,
    out_noticed: Ptr,
    out_attacked: Ptr,
) {
    let seventy: f64 = e.global(SEVENTY_DOUBLE);
    let thirty: f64 = e.global(THIRTY_DOUBLE);
    let age = e
        .call(
            TIME_STAMP_AGE,
            &args![target.byte_add(CombatTarget::fLastDetectedTimeStamp.off)],
        )
        .f64();
    let remaining = seventy - age;
    let setting = float_setting(e, 0x011c_e528);
    e.mem
        .set_f32(out_detected.addr(), (setting as f64 + remaining) as f32);
    let age = e
        .call(
            TIME_STAMP_AGE,
            &args![target.byte_add(CombatTarget::fLastNoticedTimeStamp.off)],
        )
        .f64();
    e.mem.set_f32(out_noticed.addr(), (thirty - age) as f32);
    let age = e
        .call(
            TIME_STAMP_AGE,
            &args![target.byte_add(CombatTarget::fLastAttackedMemberTimeStamp.off)],
        )
        .f64();
    e.mem.set_f32(out_attacked.addr(), (seventy - age) as f32);
}

// Translated from 0098c0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: rebuilds the search locations from the targets. First
/// removes every search location whose remaining time (`fn_00985bb0`) is
/// below zero or that belongs to a target (`iTargetID` not zero). Then, for
/// every target, takes the three excesses of `fn_0098c060` (last detected
/// a, last noticed b, last attacked member c) and, when one of them is above
/// zero: if b is below a or c, adds a search location at the last detected
/// location with score a (or at the last attacked member location with score
/// c when c is above a); otherwise, when b is above zero, adds one at the
/// last noticed location with score b, or, when the larger of a and c is
/// above zero and the last detected / attacked location (whichever has the
/// larger value) is in the same space, at the average of that location and
/// the noticed one weighted by the two scores.
pub fn fn_0098c0d0(e: &mut Engine, this: Ptr<CombatGroup>) {
    let array = this.byte_add(CombatGroup::SEARCH_LOCATIONS);
    let mut index = 0u32;
    while index < e.call(ARRAY_SIZE, &args![array]).u32() {
        let entry = e
            .call(SEARCH_LOCATION_ELEMENT, &args![array, index])
            .ptr::<CombatSearchLocation>();
        let remaining = fn_00985bb0(e, entry);
        if remaining < 0.0 || e.get(entry, CombatSearchLocation::iTargetID) != 0 {
            e.call(SEARCH_LOCATION_REMOVE, &args![array, index, 1u32]);
            index = index.wrapping_sub(1);
        }
        index = index.wrapping_add(1);
    }
    let targets = e.call(TARGET_COUNT, &args![this]).u32();
    for target_index in 0..targets {
        let target = fn_009871c0(e, this, target_index);
        e.with_stack(0x80, |e, frame| {
            let (detected, noticed, attacked) = (frame, frame.byte_add(4), frame.byte_add(8));
            fn_0098c060(e, this, target, detected, noticed, attacked);
            let a = e.mem.f32(detected.addr());
            let b = e.mem.f32(noticed.addr());
            let c = e.mem.f32(attacked.addr());
            if !(a > 0.0 || b > 0.0 || c > 0.0) {
                return;
            }
            let actor = e.get(target, CombatTarget::pActor);
            if b < a || b < c {
                let id = e.call(FORM_ID, &args![actor]).u32();
                if c <= a {
                    let location = target.byte_add(CombatTarget::LAST_DETECTED_LOCATION);
                    fn_0098bfb0(e, this, location, a, id);
                } else {
                    let location = target.byte_add(CombatTarget::LAST_ATTACKED_MEMBER_LOCATION);
                    fn_0098bfb0(e, this, location, c, id);
                }
                return;
            }
            let positive = b > 0.0;
            if !positive {
                return;
            }
            let combined = frame.byte_add(0x10);
            e.call(WORLD_LOCATION_CONSTRUCT, &args![combined]);
            let larger = e.call(FLOAT_MAX, &args![a, c]).f32();
            if larger > 0.0 {
                let source = if c < a {
                    CombatTarget::LAST_DETECTED_LOCATION
                } else {
                    CombatTarget::LAST_ATTACKED_MEMBER_LOCATION
                };
                copy_location(e, combined, target.byte_add(source));
            }
            let noticed_at = target.byte_add(CombatTarget::LAST_NOTICED_LOCATION);
            if larger > 0.0
                && e.call(FORM_ID, &args![combined]).u32()
                    == e.call(FORM_ID, &args![noticed_at]).u32()
            {
                let first_scaled = frame.byte_add(0x30);
                let second_scaled = frame.byte_add(0x40);
                let result = frame.byte_add(0x50);
                let point = e.call(POINT_NO_OP, &args![combined]).u32();
                e.call(POINT_SCALE, &args![point, first_scaled, larger]);
                let point = e.call(POINT_NO_OP, &args![noticed_at]).u32();
                let scaled = e.call(POINT_SCALE, &args![point, second_scaled, b]).u32();
                e.call(POINT_ACCUMULATE, &args![first_scaled, scaled]);
                let total = (larger as f64 + b as f64) as f32;
                e.call(POINT_DIVIDE_IN_PLACE, &args![first_scaled, total]);
                let space = e.call(FORM_ID, &args![noticed_at]).u32();
                e.call(
                    WORLD_LOCATION_FROM_POINT,
                    &args![result, first_scaled, space],
                );
                let id = e.call(FORM_ID, &args![actor]).u32();
                fn_0098bfb0(e, this, result, b, id);
            } else {
                let id = e.call(FORM_ID, &args![actor]).u32();
                fn_0098bfb0(e, this, noticed_at, b, id);
            }
        });
    }
}

// Translated from 0098c3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup::FindSearchDoor` (Xbox PDB): the reference of the search
/// door nearest to `actor` within `radius`, or null. Nothing is found for an
/// actor whose virtual method 0x21c is true and whose extra object's method
/// 0x28 (at +0x30) is true. A door qualifies when both its references are
/// set, its attempt count (+8) does not exceed the setting `011cfc98`, it is
/// neither reserved (+0xa) nor already investigated (+9), it has a teleport
/// reference, (with `check_space`) the actor can exist in the teleport's
/// space, and (when the actor has a process) the combat state's movement
/// check on the door's location passes.
pub fn combat_group_find_search_door(
    e: &mut Engine,
    this: Ptr<CombatGroup>,
    actor: Ptr,
    radius: f32,
    check_space: u8,
) -> Ptr {
    if e.vcall(actor.addr(), 0x21c, &args![]).bool() {
        let extra = e.call(ACTOR_EXTRA_OBJECT, &args![actor]).u32();
        if e.vcall(extra + 0x30, 0x28, &args![]).bool() {
            return Ptr::NULL;
        }
    }
    let process = e.vcall(actor.addr(), 0x428, &args![]).u32();
    let doors = this.byte_add(CombatGroup::SEARCH_TELEPORT_DOORS);
    e.with_stack(0x20, |e, frame| {
        let actor_location = frame;
        let door_location = frame.byte_add(0x10);
        e.call(GET_WORLD_LOCATION, &args![actor, actor_location]);
        let mut found = 0u32;
        let mut best = (radius as f64 * radius as f64) as f32;
        let mut index = 0u32;
        while index < e.call(ARRAY_SIZE, &args![doors]).u32() {
            let door = e.call(SEARCH_DOOR_ELEMENT, &args![doors, index]).u32();
            index += 1;
            let reference = e.mem.u32(door);
            if reference == 0 || e.mem.u32(door + 4) == 0 {
                continue;
            }
            let attempts = e.mem.u8(door + 8) as u32;
            let limit = int_setting(e, 0x011c_fc98);
            if attempts > limit || e.mem.u8(door + 0xa) != 0 || e.mem.u8(door + 9) != 0 {
                continue;
            }
            let teleport = e.call(DOOR_TELEPORT_REFERENCE, &args![reference]).u32();
            if teleport == 0 {
                continue;
            }
            if check_space != 0 {
                let can_exist = e.with_stack(0x10, |e, space_location| {
                    e.call(TELEPORT_WORLD_LOCATION, &args![teleport, space_location]);
                    e.call(CAN_ACTOR_EXIST_IN_SPACE, &args![actor, space_location])
                        .bool()
                });
                if !can_exist {
                    continue;
                }
            }
            e.call(GET_WORLD_LOCATION, &args![reference, door_location]);
            if process != 0 {
                let state = e.call(PROCESS_COMBAT_STATE, &args![process]).u32();
                if !e
                    .call(
                        COMBAT_STATE_CHECK_MOVEMENT,
                        &args![state, door_location, 1u32, 0u32],
                    )
                    .bool()
                {
                    continue;
                }
            }
            let squared = e
                .call(DISTANCE_SQUARED, &args![actor_location, door_location])
                .f32();
            if best > squared {
                found = reference;
                best = squared;
            }
        }
        Ptr::new(found)
    })
}

// Translated from 0098c590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup::GetCombatSearchDoor` (Xbox PDB): the `SearchTeleportDoors`
/// entry that has `reference` as its first or second reference, or null.
pub fn combat_group_get_combat_search_door(
    e: &mut Engine,
    this: Ptr<CombatGroup>,
    reference: u32,
) -> Ptr {
    let doors = this.byte_add(CombatGroup::SEARCH_TELEPORT_DOORS);
    let mut index = 0u32;
    while index < e.call(ARRAY_SIZE, &args![doors]).u32() {
        let door = e.call(SEARCH_DOOR_ELEMENT, &args![doors, index]).u32();
        if e.mem.u32(door) == reference || e.mem.u32(door + 4) == reference {
            return Ptr::new(door);
        }
        index += 1;
    }
    Ptr::NULL
}

// Translated from 0098c600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: adds `reference` to `SearchTeleportDoors` when it is not
/// there yet and has a teleport reference (`00568e50`) whose pointer
/// getter (`00559450`) gives a second reference: the new entry holds the
/// two references and its bytes +8, +0xa and +9 start at zero.
pub fn fn_0098c600(e: &mut Engine, this: Ptr<CombatGroup>, reference: u32) {
    if !combat_group_get_combat_search_door(e, this, reference).is_null() {
        return;
    }
    let teleport = e.call(DOOR_TELEPORT_REFERENCE, &args![reference]).u32();
    if teleport == 0 || e.call(NI_POINTER_GET, &args![teleport]).u32() == 0 {
        return;
    }
    let doors = this.byte_add(CombatGroup::SEARCH_TELEPORT_DOORS);
    let index = e.call(SEARCH_DOOR_ADD, &args![doors]).u32();
    let door = e.call(SEARCH_DOOR_AT, &args![doors, index]).u32();
    e.mem.set_u32(door, reference);
    let linked = e.call(NI_POINTER_GET, &args![teleport]).u32();
    let door = e.call(SEARCH_DOOR_AT, &args![doors, index]).u32();
    e.mem.set_u32(door + 4, linked);
    let door = e.call(SEARCH_DOOR_AT, &args![doors, index]).u32();
    e.mem.set_u8(door + 8, 0);
    let door = e.call(SEARCH_DOOR_AT, &args![doors, index]).u32();
    e.mem.set_u8(door + 0xa, 0);
    let door = e.call(SEARCH_DOOR_AT, &args![doors, index]).u32();
    e.mem.set_u8(door + 9, 0);
}

// Translated from 0098c6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: sets byte +9 of the search door for `reference` (when
/// there is one).
pub fn fn_0098c6e0(e: &mut Engine, this: Ptr<CombatGroup>, reference: u32) {
    let door = combat_group_get_combat_search_door(e, this, reference);
    if !door.is_null() {
        e.mem.set_u8(door.addr() + 9, 1);
    }
}

// Translated from 0098c710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup::FailedToInvestigateSearchDoor` (Xbox PDB): clears byte +0xa
/// of the search door for `reference` (when there is one) and counts one
/// more attempt in byte +8.
pub fn combat_group_failed_to_investigate_search_door(
    e: &mut Engine,
    this: Ptr<CombatGroup>,
    reference: u32,
) {
    let door = combat_group_get_combat_search_door(e, this, reference);
    if !door.is_null() {
        e.mem.set_u8(door.addr() + 0xa, 0);
        let attempts = e.mem.u8(door.addr() + 8).wrapping_add(1);
        e.mem.set_u8(door.addr() + 8, attempts);
    }
}

// Translated from 0098c750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: sets byte +0xa of the search door for `reference` (when
/// there is one); `UnreserveSearchDoor` clears it.
pub fn fn_0098c750(e: &mut Engine, this: Ptr<CombatGroup>, reference: u32) {
    let door = combat_group_get_combat_search_door(e, this, reference);
    if !door.is_null() {
        e.mem.set_u8(door.addr() + 0xa, 1);
    }
}

// Translated from 0098c780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup::UnreserveSearchDoor` (Xbox PDB): clears byte +0xa of the
/// search door for `reference` (when there is one).
pub fn combat_group_unreserve_search_door(e: &mut Engine, this: Ptr<CombatGroup>, reference: u32) {
    let door = combat_group_get_combat_search_door(e, this, reference);
    if !door.is_null() {
        e.mem.set_u8(door.addr() + 0xa, 0);
    }
}

// Translated from 0098c7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup::CalculateSearchIgnoreLocations` (Xbox PDB): with more than
/// one member, adds to `array` the pathfinding goal
/// (`Actor::GetCurrentPathfindingGoal`) of every member other than `skip`
/// that has a process (and passes the process test `00981420`); returns
/// whether any was added.
pub fn combat_group_calculate_search_ignore_locations(
    e: &mut Engine,
    this: Ptr<CombatGroup>,
    skip: Ptr,
    array: Ptr,
) -> bool {
    let members = e.call(MEMBER_COUNT, &args![this]).u32();
    if members <= 1 {
        return false;
    }
    let mut added = false;
    for index in 0..members {
        let actor = e.call(MEMBER_ACTOR_AT, &args![this, index]).u32();
        if actor == skip.addr() {
            continue;
        }
        let process = e.vcall(actor, 0x428, &args![]).u32();
        if process == 0 || !e.call(PROCESS_TEST_00981420, &args![process]).bool() {
            continue;
        }
        e.with_stack(0xc, |e, point| {
            e.call(POINT_NO_OP, &args![point]);
            if e.call(ACTOR_PATHFINDING_GOAL, &args![actor, point]).bool() {
                e.call(PATH_POINT_ARRAY_ADD, &args![array, point]);
                added = true;
            }
        });
    }
    added
}

// Translated from 0098c870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup::UpdateCombatStrength` (Xbox PDB): recomputes
/// `fMemberCombatStrength` (sum over the usable members of their combat
/// strength), `fAverageMemberDamagePerSecond`, `fTargetCombatStrength` (sum
/// over the usable targets) and `fAverageTargetCombatStrength`.
///
/// A member's `fDamagePerSecond` and `fCombatStrength` are refreshed first:
/// from the player's weapon and `Actor::CalculateCombatStrength` for the
/// player, otherwise from the combat state of its process (a member with no
/// process keeps its old values). A member counts when it is not fleeing
/// (`CombatController::IsFleeing`) and both values are not below zero. A
/// target counts when its strength (the player's `CalculateCombatStrength`,
/// the state's value, or `CalculateCombatStrength(-1)` for an actor with no
/// process; -1 for a fleeing one) is not below zero.
pub fn combat_group_update_combat_strength(e: &mut Engine, this: Ptr<CombatGroup>) {
    e.set(this, CombatGroup::fMemberCombatStrength, 0.0);
    e.set(this, CombatGroup::fAverageMemberDamagePerSecond, 0.0);
    let members = e.call(MEMBER_COUNT, &args![this]).u32();
    let mut counted = 0u32;
    let player = e.global::<u32>(PLAYER);
    for index in 0..members {
        let mut fleeing = false;
        let member = e
            .call(MEMBER_OF_GROUP, &args![this, index])
            .ptr::<CombatMember>();
        let actor = e.get(member, CombatMember::pActor);
        if actor.addr() == player {
            let damage = e.call(ACTOR_WEAPON_DAMAGE_PER_SECOND, &args![player]).f32();
            e.set(member, CombatMember::fDamagePerSecond, damage);
            let damage = e.get(member, CombatMember::fDamagePerSecond);
            let strength = e.call(ACTOR_COMBAT_STRENGTH, &args![player, damage]).f32();
            e.set(member, CombatMember::fCombatStrength, strength);
        } else {
            let process = e.vcall(actor.addr(), 0x428, &args![]).u32();
            if process != 0 {
                fleeing = e.call(CONTROLLER_IS_FLEEING, &args![process]).bool();
                let state = e.call(PROCESS_COMBAT_STATE, &args![process]).u32();
                let damage = e.call(STATE_DAMAGE_PER_SECOND, &args![state]).f32();
                e.set(member, CombatMember::fDamagePerSecond, damage);
                let state = e.call(PROCESS_COMBAT_STATE, &args![process]).u32();
                let strength = e.call(STATE_COMBAT_STRENGTH, &args![state]).f32();
                e.set(member, CombatMember::fCombatStrength, strength);
            }
        }
        let strength = e.get(member, CombatMember::fCombatStrength);
        let damage = e.get(member, CombatMember::fDamagePerSecond);
        if !fleeing && strength >= 0.0 && damage >= 0.0 {
            let total = e.get(this, CombatGroup::fMemberCombatStrength);
            e.set(
                this,
                CombatGroup::fMemberCombatStrength,
                (total as f64 + strength as f64) as f32,
            );
            let total = e.get(this, CombatGroup::fAverageMemberDamagePerSecond);
            e.set(
                this,
                CombatGroup::fAverageMemberDamagePerSecond,
                (total as f64 + damage as f64) as f32,
            );
            counted += 1;
        }
    }
    if counted != 0 {
        let total = e.get(this, CombatGroup::fAverageMemberDamagePerSecond);
        e.set(
            this,
            CombatGroup::fAverageMemberDamagePerSecond,
            (total as f64 / counted as f64) as f32,
        );
    }
    e.set(this, CombatGroup::fTargetCombatStrength, 0.0);
    e.set(this, CombatGroup::fAverageTargetCombatStrength, 0.0);
    let targets = e.call(TARGET_COUNT, &args![this]).u32();
    let mut counted = 0u32;
    for index in 0..targets {
        let minus_one: f32 = e.global(MINUS_ONE);
        let mut strength = minus_one;
        let actor = e.call(TARGET_ACTOR_AT, &args![this, index]).u32();
        if actor == player {
            strength = e
                .call(ACTOR_COMBAT_STRENGTH, &args![player, minus_one])
                .f32();
        } else if !e.call(ACTOR_IS_FLEEING_ARG, &args![actor, 0u32]).bool() {
            let process = e.vcall(actor, 0x428, &args![]).u32();
            if process != 0 {
                let state = e.call(PROCESS_COMBAT_STATE, &args![process]).u32();
                strength = e.call(STATE_COMBAT_STRENGTH, &args![state]).f32();
            } else {
                strength = e
                    .call(ACTOR_COMBAT_STRENGTH, &args![actor, minus_one])
                    .f32();
            }
        }
        if strength >= 0.0 {
            let total = e.get(this, CombatGroup::fTargetCombatStrength);
            e.set(
                this,
                CombatGroup::fTargetCombatStrength,
                (total as f64 + strength as f64) as f32,
            );
            counted += 1;
        }
    }
    if counted != 0 {
        let total = e.get(this, CombatGroup::fTargetCombatStrength);
        e.set(
            this,
            CombatGroup::fAverageTargetCombatStrength,
            (total as f64 / counted as f64) as f32,
        );
    }
}

// Translated from 0098cb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: the combat strength of the group's members as seen with
/// `actor` in mind. With a null `actor` it is `fMemberCombatStrength`.
/// Otherwise the sum over the members of: for `actor` itself (when it has a
/// process) the state's strength if above zero; for every other member that
/// is not fleeing, its stored `fCombatStrength` if above zero.
pub fn fn_0098cb30(e: &mut Engine, this: Ptr<CombatGroup>, actor: Ptr) -> f32 {
    if actor.is_null() {
        return e.get(this, CombatGroup::fMemberCombatStrength);
    }
    let mut sum = 0.0f32;
    let members = e.call(MEMBER_COUNT, &args![this]).u32();
    for index in 0..members {
        let member = e
            .call(MEMBER_OF_GROUP, &args![this, index])
            .ptr::<CombatMember>();
        let member_actor = e.get(member, CombatMember::pActor);
        let process = e.vcall(member_actor.addr(), 0x428, &args![]).u32();
        if actor == member_actor && process != 0 {
            let state = e.call(PROCESS_COMBAT_STATE, &args![process]).u32();
            let strength = e.call(STATE_COMBAT_STRENGTH, &args![state]).f32();
            if strength > 0.0 {
                sum = (sum as f64 + strength as f64) as f32;
            }
        } else if process == 0 || !e.call(CONTROLLER_IS_FLEEING, &args![process]).bool() {
            let strength = e.get(member, CombatMember::fCombatStrength);
            if strength > 0.0 {
                sum = (sum as f64 + strength as f64) as f32;
            }
        }
    }
    sum
}

// Translated from 0098cc20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: the members' average damage per second as seen with
/// `actor` in mind. With a null `actor` it is
/// `fAverageMemberDamagePerSecond`. Otherwise the average over the counted
/// members of: for `actor` itself (when it has a process) the state's
/// damage per second if above zero; for every other member that is not
/// fleeing and whose `fCombatStrength` is not below zero, its stored
/// `fDamagePerSecond`.
pub fn fn_0098cc20(e: &mut Engine, this: Ptr<CombatGroup>, actor: Ptr) -> f32 {
    if actor.is_null() {
        return e.get(this, CombatGroup::fAverageMemberDamagePerSecond);
    }
    let mut sum = 0.0f32;
    let members = e.call(MEMBER_COUNT, &args![this]).u32();
    let mut counted = 0u32;
    for index in 0..members {
        let member = e
            .call(MEMBER_OF_GROUP, &args![this, index])
            .ptr::<CombatMember>();
        let member_actor = e.get(member, CombatMember::pActor);
        let process = e.vcall(member_actor.addr(), 0x428, &args![]).u32();
        if actor == member_actor && process != 0 {
            let state = e.call(PROCESS_COMBAT_STATE, &args![process]).u32();
            let damage = e.call(STATE_DAMAGE_PER_SECOND, &args![state]).f32();
            if damage > 0.0 {
                sum = (sum as f64 + damage as f64) as f32;
                counted += 1;
            }
        } else {
            let usable = process == 0 || !e.call(CONTROLLER_IS_FLEEING, &args![process]).bool();
            if usable && e.get(member, CombatMember::fCombatStrength) >= 0.0 {
                let damage = e.get(member, CombatMember::fDamagePerSecond);
                sum = (sum as f64 + damage as f64) as f32;
                counted += 1;
            }
        }
    }
    if counted != 0 {
        sum = (sum as f64 / counted as f64) as f32;
    }
    sum
}

// Translated from 0098cd50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: the member (other than `actor`) nearest to `actor` within
/// `radius`, or null. The squared distance of `preferred` is divided by
/// 2.25 first. A nearest member whose process has a target of its own
/// (`007058c0`) that is `actor` is discarded.
pub fn fn_0098cd50(
    e: &mut Engine,
    this: Ptr<CombatGroup>,
    actor: Ptr,
    preferred: Ptr,
    radius: f32,
) -> Ptr {
    let mut best = (radius as f64 * radius as f64) as f32;
    let mut found = 0u32;
    let position_ptr = e.vcall(actor.addr(), 0x1f4, &args![]).u32();
    e.with_stack(0x20, |e, frame| {
        let position = frame;
        let difference = frame.byte_add(0x10);
        copy_point(e, position, position_ptr);
        let members = e.call(MEMBER_COUNT, &args![this]).u32();
        for index in 0..members {
            let other = e.call(MEMBER_ACTOR_AT, &args![this, index]).u32();
            if other == actor.addr() {
                continue;
            }
            let other_position = e.vcall(other, 0x1f4, &args![]).u32();
            e.call(POINT_SUBTRACT, &args![other_position, difference, position]);
            let mut squared = e.call(POINT_LENGTH_SQUARED, &args![difference]).f32();
            if other == preferred.addr() {
                let divisor: f64 = e.global(TWO_AND_QUARTER_DOUBLE);
                squared = (squared as f64 / divisor) as f32;
            }
            if best > squared {
                best = squared;
                found = other;
            }
        }
    });
    if found != 0 {
        let process = e.vcall(found, 0x428, &args![]).u32();
        if process != 0 {
            let state = e.call(PROCESS_COMBAT_STATE, &args![process]).u32();
            let goal = e.call(STATE_TARGET_RECORD, &args![state]).u32();
            if goal != 0 && e.mem.u32(goal + 4) == actor.addr() {
                found = 0;
            }
        }
    }
    Ptr::new(found)
}

// Translated from 0098ce80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup::UpdateCounts` (Xbox PDB): recounts the members that have a
/// process: `iInitializedMemberCount` (bit 3 of the process word at +8, test
/// `004013e0`), and `iFleeingMemberCount` / `iNonFleeingMemberCount` by
/// `CombatController::IsFleeing`.
pub fn combat_group_update_counts(e: &mut Engine, this: Ptr<CombatGroup>) {
    e.set(this, CombatGroup::iInitializedMemberCount, 0);
    e.set(this, CombatGroup::iFleeingMemberCount, 0);
    e.set(this, CombatGroup::iNonFleeingMemberCount, 0);
    let members = e.call(MEMBER_COUNT, &args![this]).u32();
    for index in 0..members {
        let actor = e.call(MEMBER_ACTOR_AT, &args![this, index]).u32();
        let process = e.vcall(actor, 0x428, &args![]).u32();
        if process == 0 {
            continue;
        }
        if e.call(PROCESS_FLAG_BIT_3, &args![process]).bool() {
            let count = e.get(this, CombatGroup::iInitializedMemberCount);
            e.set(this, CombatGroup::iInitializedMemberCount, count + 1);
        }
        if e.call(CONTROLLER_IS_FLEEING, &args![process]).bool() {
            let count = e.get(this, CombatGroup::iFleeingMemberCount);
            e.set(this, CombatGroup::iFleeingMemberCount, count + 1);
        } else {
            let count = e.get(this, CombatGroup::iNonFleeingMemberCount);
            e.set(this, CombatGroup::iNonFleeingMemberCount, count + 1);
        }
    }
}

// Translated from 0098cf70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: recounts `sAttackerCount` of every target: the number of
/// members other than the player whose virtual method 0x42c (their current
/// target) is that target's actor.
pub fn fn_0098cf70(e: &mut Engine, this: Ptr<CombatGroup>) {
    let targets = e.call(TARGET_COUNT, &args![this]).u32();
    let members = e.call(MEMBER_COUNT, &args![this]).u32();
    let player = e.global::<u32>(PLAYER);
    for index in 0..targets {
        let target = fn_009871c0(e, this, index);
        e.set(target, CombatTarget::sAttackerCount, 0);
        for member_index in 0..members {
            let actor = e.call(MEMBER_ACTOR_AT, &args![this, member_index]).u32();
            if actor == player {
                continue;
            }
            let aimed_at = e.vcall(actor, 0x42c, &args![]).u32();
            if aimed_at == e.get(target, CombatTarget::pActor).addr() {
                let count = e.get(target, CombatTarget::sAttackerCount);
                e.set(target, CombatTarget::sAttackerCount, count.wrapping_add(1));
            }
        }
    }
}

// Translated from 0098d030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: drops the targets `fn_00987220` rejects. When the group
/// has just one target and it is dropped, the actor `fn_00989970` finds for
/// it within 1,000,000 units (or, failing that, for the player) says a line
/// of dialogue about it unless the target is dead (virtual method 0x22c with
/// 0): dialogue type 8 once the target has been detected (its last detected
/// time stamp is not the lowest value) and type 5 otherwise.
pub fn fn_0098d030(e: &mut Engine, this: Ptr<CombatGroup>) {
    let mut targets = e.call(TARGET_COUNT, &args![this]).u32();
    if targets == 0 {
        return;
    }
    e.with_stack(4, |e, stamp| {
        let lowest = -e.global::<f32>(MAX_FLOAT);
        e.call(TIME_STAMP_SET, &args![stamp, lowest]);
    });
    let mut index = 0u32;
    while index < targets {
        let target = fn_009871c0(e, this, index);
        if fn_00987220(e, this, target) {
            if targets == 1 {
                let radius: f32 = e.global(ONE_MILLION_FLOAT);
                let actor = e.get(target, CombatTarget::pActor);
                let mut speaker = fn_00989970(e, this, actor, radius);
                if speaker.is_null() {
                    let player = Ptr::new(e.global::<u32>(PLAYER));
                    speaker = fn_00989970(e, this, player, radius);
                }
                if !speaker.is_null() && !e.vcall(actor.addr(), 0x22c, &args![0u32]).bool() {
                    let stamp = e
                        .call(
                            TIME_STAMP_VALUE,
                            &args![target.byte_add(CombatTarget::fLastDetectedTimeStamp.off)],
                        )
                        .f32();
                    let kind = if stamp == -e.global::<f32>(MAX_FLOAT) {
                        5u32
                    } else {
                        8u32
                    };
                    let manager = e.global::<u32>(DIALOGUE_MANAGER);
                    e.call(
                        START_DIALOGUE,
                        &args![manager, speaker, actor, 4u32, kind, 1u32, 0u32],
                    );
                }
            }
            fn_00986560(e, this, index);
            index = index.wrapping_sub(1);
            targets = targets.wrapping_sub(1);
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 0098d1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: keeps the group strategy up to date. With fewer than two
/// members there is no strategy (`pGroupStrategy` and `bStrategyForced`
/// cleared) and the choose timer (+0x30) restarts with 0. Otherwise, when
/// the strategy is not forced and the choose timer has expired, the strategy
/// chooser (`009900f0`) picks one, the timer restarts with the delay at
/// `0101712c`, and a chosen strategy records its index
/// (`iLastGroupStrategyChosenIndex`) and the time
/// (`fLastGroupStrategyChosenTimeStamp`); then, when there is a strategy and
/// the update timer (+0x38) has expired, the strategy's virtual method 8 is
/// run on the group (it drops the strategy when it returns false) and the
/// update timer restarts with 1.
pub fn fn_0098d1a0(e: &mut Engine, this: Ptr<CombatGroup>) {
    let choose_timer = this.byte_add(CombatGroup::CHOOSE_STRATEGY_TIMER);
    let update_timer = this.byte_add(CombatGroup::UPDATE_STRATEGY_TIMER);
    if e.call(MEMBER_COUNT, &args![this]).u32() <= 1 {
        e.set(this, CombatGroup::pGroupStrategy, Ptr::NULL);
        e.set(this, CombatGroup::bStrategyForced, false);
        e.call(TIMER_START, &args![choose_timer, 0.0f32]);
        return;
    }
    if !e.get(this, CombatGroup::bStrategyForced)
        && e.call(TIMER_EXPIRED, &args![choose_timer]).bool()
    {
        let strategy = e.call(CHOOSE_STRATEGY, &args![this]).ptr::<()>();
        e.set(this, CombatGroup::pGroupStrategy, strategy);
        let delay: f32 = e.global(STRATEGY_DELAY);
        e.call(TIMER_START, &args![choose_timer, delay]);
        if !e.get(this, CombatGroup::pGroupStrategy).is_null() {
            let strategy = e.get(this, CombatGroup::pGroupStrategy);
            let index = e.call(STRATEGY_INDEX, &args![strategy]).u32();
            e.set(this, CombatGroup::iLastGroupStrategyChosenIndex, index);
            e.with_stack(4, |e, stamp| {
                let now = e.call(GAME_TIME_NOW, &args![]).f32();
                e.call(TIME_STAMP_SET, &args![stamp, now]);
                let value = e.mem.f32(stamp.addr());
                e.set(this, CombatGroup::fLastGroupStrategyChosenTimeStamp, value);
            });
        }
    }
    if !e.get(this, CombatGroup::pGroupStrategy).is_null()
        && e.call(TIMER_EXPIRED, &args![update_timer]).bool()
    {
        let strategy = e.get(this, CombatGroup::pGroupStrategy);
        if !e.vcall(strategy.addr(), 8, &args![this]).bool() {
            e.set(this, CombatGroup::pGroupStrategy, Ptr::NULL);
            e.set(this, CombatGroup::bStrategyForced, false);
        }
        e.call(TIMER_START, &args![update_timer, 1.0f32]);
    }
}

// Translated from 0098d2c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: rebuilds the clusters of nearby members
/// (`CombatGroupCluster`, `ClusterArray` at +0x138), unless the global
/// `011a4d34` switches that off. The C++ exception frame is not translated.
///
/// When the cluster update timer (+0xb0) has expired, restarts it with the
/// delay `011a4d30` and works on at most 63 members (`min(count, 0x3f)`):
/// each gets a record of its member entry, world location and nearest
/// neighbour. Members whose cluster is farther than twice the radius
/// `011a4d2c` leave it; clusters left empty are deleted; every member with no
/// cluster looks up its nearest neighbour (squared distances are cached in a
/// 64 x 64 matrix of `float`s, 0 meaning "not computed") and mutual nearest
/// pairs without a cluster form a new one centred between them; clusters
/// closer than the radius are merged (the later one is deleted and its
/// members move to the earlier one); finally a member with no cluster takes
/// the first cluster found by following its chain of nearest neighbours
/// when that one is closer than the radius, and so do the members on the
/// chain that have none. The cluster centres are recomputed from their
/// members before (`fn_0098dc40`) and after.
pub fn fn_0098d2c0(e: &mut Engine, this: Ptr<CombatGroup>) {
    if e.mem.u8(CLUSTER_DISABLE) != 0 {
        return;
    }
    fn_0098dc40(e, this);
    let timer = this.byte_add(CombatGroup::CLUSTER_UPDATE_TIMER);
    if !e.call(TIMER_EXPIRED, &args![timer]).bool() {
        return;
    }
    let delay: f32 = e.global(CLUSTER_DELAY);
    e.call(TIMER_START, &args![timer, delay]);
    let radius: f32 = e.global(CLUSTER_RADIUS);
    let radius_squared = radius as f64 * radius as f64;
    let four: f64 = e.global(FOUR_DOUBLE);
    let clusters = this.byte_add(CombatGroup::CLUSTER_ARRAY);
    let members = e.call(MEMBER_COUNT, &args![this]).i32();
    let members = e.call(SIGNED_MIN, &args![members, 0x3fu32]).u32();
    // The game's frame: 64 records of 0x18 bytes (+0 the `CombatMember`,
    // +4 its world location, +0x14 the nearest record as a signed byte),
    // then the 64 x 64 `float` matrix, then three scratch values.
    e.with_stack(0x4700, |e, frame| {
        let matrix = frame.byte_add(0x600);
        let scratch_location = frame.byte_add(0x4600);
        let new_cluster = frame.byte_add(0x4610);
        let scratch_point = frame.byte_add(0x4620);
        let record = |index: u32| frame.byte_add(index.wrapping_mul(0x18));

        let cluster_of = |e: &Engine, index: u32| {
            let member = e.mem.u32(record(index).addr());
            e.mem.u32(member + CombatMember::pCluster.off)
        };
        let set_cluster = |e: &mut Engine, index: u32, cluster: u32| {
            let member = e.mem.u32(record(index).addr());
            e.mem.set_u32(member + CombatMember::pCluster.off, cluster);
        };
        let nearest_of = |e: &Engine, index: i32| {
            e.mem.i8(record(index as u32).addr().wrapping_add(0x14)) as i32
        };
        let cell = |row: u32, column: u32| matrix.addr() + row * 0x100 + column * 4;
        e.call(
            VECTOR_CONSTRUCTOR,
            &args![frame, 0x18u32, 0x40u32, 0x0098_dc20u32],
        );
        e.call(MEMSET, &args![frame, 0u32, 0x600u32]);
        e.call(MEMSET, &args![matrix, 0u32, 0x4000u32]);
        for index in 0..members {
            let member = e.call(MEMBER_OF_GROUP, &args![this, index]).u32();
            e.mem.set_u32(record(index).addr(), member);
            let actor = e.mem.u32(member);
            let world = e
                .call(GET_WORLD_LOCATION, &args![actor, scratch_location])
                .ptr::<()>();
            copy_location(e, record(index).byte_add(4), world);
            e.mem.set_u8(record(index).addr() + 0x14, 0xff);
            let cluster = cluster_of(e, index);
            if cluster != 0 {
                let squared = e
                    .call(DISTANCE_SQUARED, &args![cluster, record(index).byte_add(4)])
                    .f64();
                if radius_squared * four < squared {
                    let count = e.mem.u32(cluster + 0x10);
                    e.mem.set_u32(cluster + 0x10, count.wrapping_sub(1));
                    set_cluster(e, index, 0);
                }
            }
        }
        // Delete the clusters nobody is left in.
        let mut index = 0u32;
        while index < e.call(ARRAY_SIZE, &args![clusters]).u32() {
            let slot = e.call(CLUSTER_AT, &args![clusters, index]).u32();
            let cluster = e.mem.u32(slot);
            if e.mem.u32(cluster + 0x10) == 0 {
                e.call(CLUSTER_REMOVE, &args![clusters, index, 1u32]);
                e.call(OPERATOR_DELETE, &args![cluster]);
                index = index.wrapping_sub(1);
            }
            index = index.wrapping_add(1);
        }
        // Each unclustered member finds its nearest neighbour.
        for row in 0..members {
            if cluster_of(e, row) != 0 {
                continue;
            }
            let mut best = e.global::<f32>(MAX_FLOAT);
            let mut best_column = 0u8;
            for column in 0..members {
                if row == column {
                    continue;
                }
                if e.mem.f32(cell(row, column)) == 0.0 {
                    let squared = e
                        .call(
                            DISTANCE_SQUARED,
                            &args![record(row).byte_add(4), record(column).byte_add(4)],
                        )
                        .f32();
                    e.mem.set_f32(cell(row, column), squared);
                    e.mem.set_f32(cell(column, row), squared);
                }
                let squared = e.mem.f32(cell(row, column));
                if best > squared {
                    best_column = column as u8;
                    best = squared;
                }
            }
            e.mem.set_u8(record(row).addr() + 0x14, best_column);
        }
        // Mutual nearest pairs with no cluster form a new one.
        for first in 0..members {
            if cluster_of(e, first) != 0 {
                continue;
            }
            let second = nearest_of(e, first as i32);
            if nearest_of(e, second) != first as i8 as i32 {
                continue;
            }
            let memory = e.call(OPERATOR_NEW_SMALL, &args![0x14u32]).u32();
            let cluster = if memory != 0 {
                fn_0098dc00(e, Ptr::new(memory)).addr()
            } else {
                0
            };
            e.mem.set_u32(new_cluster.addr(), cluster);
            e.call(POINTER_ARRAY_ADD, &args![clusters, new_cluster]);
            let cluster = e.mem.u32(new_cluster.addr());
            set_cluster(e, first, cluster);
            set_cluster(e, second as u32, cluster);
            let first_point = e
                .call(POINT_NO_OP, &args![record(second as u32).byte_add(4)])
                .u32();
            let second_point = e.call(POINT_NO_OP, &args![record(first).byte_add(4)]).u32();
            e.call(POINT_ADD, &args![second_point, scratch_point, first_point]);
            let two: f32 = e.global(TWO_FLOAT);
            e.call(POINT_DIVIDE_IN_PLACE, &args![scratch_point, two]);
            let space = e.call(FORM_ID, &args![record(first).byte_add(4)]).u32();
            bgs_world_location_set_location(e, Ptr::new(cluster), scratch_point, space);
        }
        // Merge the clusters that are closer than the radius.
        let mut first_index = 0u32;
        while first_index < e.call(ARRAY_SIZE, &args![clusters]).u32() {
            let slot = e.call(CLUSTER_AT, &args![clusters, first_index]).u32();
            let kept = e.mem.u32(slot);
            let mut other_index = first_index + 1;
            while other_index < e.call(ARRAY_SIZE, &args![clusters]).u32() {
                let slot = e.call(CLUSTER_AT, &args![clusters, other_index]).u32();
                let other = e.mem.u32(slot);
                let squared = e.call(DISTANCE_SQUARED, &args![kept, other]).f64();
                if radius_squared > squared {
                    for member_index in 0..members {
                        if cluster_of(e, member_index) == other {
                            set_cluster(e, member_index, kept);
                        }
                    }
                    e.call(OPERATOR_DELETE, &args![other]);
                    e.call(CLUSTER_REMOVE, &args![clusters, other_index, 1u32]);
                    other_index = other_index.wrapping_sub(1);
                }
                other_index = other_index.wrapping_add(1);
            }
            first_index += 1;
        }
        // The rest join the first cluster along their chain, if close enough.
        for index in 0..members {
            if cluster_of(e, index) != 0 {
                continue;
            }
            let mut found = 0u32;
            let mut chain = nearest_of(e, index as i32);
            while found == 0 {
                found = cluster_of(e, chain as u32);
                chain = nearest_of(e, chain);
            }
            let squared = e
                .call(DISTANCE_SQUARED, &args![record(index).byte_add(4), found])
                .f64();
            if radius_squared > squared {
                set_cluster(e, index, found);
                chain = nearest_of(e, index as i32);
                while cluster_of(e, chain as u32) == 0 {
                    set_cluster(e, chain as u32, found);
                    chain = nearest_of(e, chain);
                }
            }
        }
    });
    fn_0098dc40(e, this);
}

// Translated from 0098dc00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroupCluster` constructor (0x14 bytes): a default
/// `BGSWorldLocation` at +0 and a member count (+0x10) of zero. Returns
/// `this`.
pub fn fn_0098dc00(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(WORLD_LOCATION_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr() + 0x10, 0);
    this
}

// Translated from 0098dc20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the 0x18-byte record `fn_0098d2c0` keeps per member: a
/// default `BGSWorldLocation` at +4. Returns `this`.
pub fn fn_0098dc20(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(WORLD_LOCATION_CONSTRUCT, &args![this.byte_add(4)]);
    this
}

// Translated from 0098dc40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: recomputes the centre of every cluster from its members.
/// Zeroes the centres and counts, adds each member's position (virtual
/// method 0x1f4 of its actor) to its cluster's centre and counts it, then
/// divides each centre by its count.
pub fn fn_0098dc40(e: &mut Engine, this: Ptr<CombatGroup>) {
    let clusters = this.byte_add(CombatGroup::CLUSTER_ARRAY);
    let mut index = 0u32;
    while index < e.call(ARRAY_SIZE, &args![clusters]).u32() {
        let slot = e.call(CLUSTER_AT, &args![clusters, index]).u32();
        let cluster = e.mem.u32(slot);
        fn_0098ddd0(e, Ptr::new(cluster), Ptr::new(ZERO_POINT));
        e.mem.set_u32(cluster + 0x10, 0);
        index += 1;
    }
    let members = e.call(MEMBER_COUNT, &args![this]).u32();
    for member_index in 0..members {
        let member = e
            .call(MEMBER_OF_GROUP, &args![this, member_index])
            .ptr::<CombatMember>();
        let cluster = e.get(member, CombatMember::pCluster);
        if cluster.is_null() {
            continue;
        }
        e.with_stack(0xc, |e, sum| {
            let centre = e.call(POINT_NO_OP, &args![cluster]).u32();
            copy_point(e, sum, centre);
            let actor = e.get(member, CombatMember::pActor);
            let position = e.vcall(actor.addr(), 0x1f4, &args![]).u32();
            e.call(POINT_ACCUMULATE, &args![sum, position]);
            fn_0098ddd0(e, cluster, sum);
        });
        let count = e.mem.u32(cluster.addr() + 0x10);
        e.mem.set_u32(cluster.addr() + 0x10, count.wrapping_add(1));
    }
    let mut index = 0u32;
    while index < e.call(ARRAY_SIZE, &args![clusters]).u32() {
        let slot = e.call(CLUSTER_AT, &args![clusters, index]).u32();
        let cluster = e.mem.u32(slot);
        e.with_stack(0xc, |e, centre| {
            let point = e.call(POINT_NO_OP, &args![cluster]).u32();
            copy_point(e, centre, point);
            let count = e.mem.u32(cluster + 0x10);
            e.call(POINT_DIVIDE_IN_PLACE, &args![centre, count as f32]);
            fn_0098ddd0(e, Ptr::new(cluster), centre);
        });
        index += 1;
    }
}

// Translated from 0098ddd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroupCluster`: copies the three coordinates of `point` to the
/// cluster's centre (the space is left alone).
pub fn fn_0098ddd0(e: &mut Engine, this: Ptr, point: Ptr) {
    copy_point(e, this, point.addr());
}

// Translated from 0098de00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: the cluster of the member entry for `actor`, or null when
/// the actor has no entry.
pub fn fn_0098de00(e: &mut Engine, this: Ptr<CombatGroup>, actor: Ptr) -> u32 {
    let member = fn_009865f0(e, this, actor);
    if member.is_null() {
        0
    } else {
        e.get(member, CombatMember::pCluster).addr()
    }
}

// Translated from 0098de30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: searches the `ClusterArray` (`00719b20`, with the
/// comparison `009a3830` and a start index of 0) for `cluster`, whose
/// address it passes; returns that search's result.
pub fn fn_0098de30(e: &mut Engine, this: Ptr<CombatGroup>, cluster: u32) -> u32 {
    let array = this.byte_add(CombatGroup::CLUSTER_ARRAY);
    e.with_stack(4, |e, value| {
        e.mem.set_u32(value.addr(), cluster);
        e.call(CLUSTER_SEARCH, &args![array, value, 0u32, CLUSTER_COMPARE])
            .u32()
    })
}

// Translated from 0098de60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup`: the cluster with the given index (a signed byte), or null
/// for a negative index.
pub fn fn_0098de60(e: &mut Engine, this: Ptr<CombatGroup>, index: i8) -> u32 {
    if index < 0 {
        return 0;
    }
    let slot = e
        .call(
            CLUSTER_AT,
            &args![this.byte_add(CombatGroup::CLUSTER_ARRAY), index as i32],
        )
        .u32();
    e.mem.u32(slot)
}

// Translated from 0098de90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup::SaveGame` (Xbox PDB): writes the group to the save buffer,
/// in this order: the clusters (count, then each through `fn_00985ae0`), the
/// targets (count, each `CombatTarget::SaveGame`), the members (count, each
/// `CombatMember::SaveGame`); the strategy's index as one byte (0xff when
/// there is none) and `bStrategyForced`; the choose and update timers;
/// `iLastGroupStrategyChosenIndex` and its time stamp; the avoid-threat
/// dialogue timer, the cluster timer and the target timer; the ten
/// detection dialogue timers; `iSearchCount` and, when it is not zero, the
/// search state (searching member as a form ID, the focal point, the two
/// search timers, `fSearchStartedTimeStamp`, `SearchCenter`, the radius, the
/// search locations, the teleport doors, and the line-of-sight grid behind a
/// flag byte); then the combat strength timer, the four strength values, the
/// music timer and `cCombatMusicState`. (Counts are written through
/// `SaveVariableSizedValue`; arrays at an impossible address count as empty,
/// as the folded accessors check.)
pub fn combat_group_save_game(e: &mut Engine, this: Ptr<CombatGroup>, buffer: Ptr) {
    let clusters = this.byte_add(CombatGroup::CLUSTER_ARRAY);
    let count = array_count(e, clusters);
    e.call(SAVE_SIZED_VALUE, &args![buffer, count]);
    for index in 0..count {
        let slot = e.call(CLUSTER_AT, &args![clusters, index]).u32();
        let cluster = e.mem.u32(slot);
        fn_00985ae0(e, Ptr::new(cluster), buffer, this.addr());
    }
    let targets = this.byte_add(CombatGroup::TARGET_ARRAY);
    let count = array_count(e, targets);
    e.call(SAVE_SIZED_VALUE, &args![buffer, count]);
    for index in 0..count {
        let target = e
            .call(TARGET_AT, &args![targets, index])
            .ptr::<CombatTarget>();
        combat_target_save_game(e, target, buffer, this.addr());
    }
    let members = this.byte_add(CombatGroup::MEMBER_ARRAY);
    let count = array_count(e, members);
    e.call(SAVE_SIZED_VALUE, &args![buffer, count]);
    for index in 0..count {
        let member = e
            .call(MEMBER_AT, &args![members, index])
            .ptr::<CombatMember>();
        combat_member_save_game(e, member, buffer, this);
    }
    let strategy = e.get(this, CombatGroup::pGroupStrategy);
    let index = if strategy.is_null() {
        u32::MAX
    } else {
        e.call(STRATEGY_INDEX, &args![strategy]).u32()
    };
    e.with_stack(4, |e, byte| {
        e.mem.set_u8(byte.addr(), index as u8);
        e.call(SAVE_BYTES, &args![buffer, byte, 1u32, 0u32]);
    });
    save_field(e, buffer, this, CombatGroup::bStrategyForced.off, 1);
    save_timer(e, buffer, this, CombatGroup::CHOOSE_STRATEGY_TIMER);
    save_timer(e, buffer, this, CombatGroup::UPDATE_STRATEGY_TIMER);
    save_field(
        e,
        buffer,
        this,
        CombatGroup::iLastGroupStrategyChosenIndex.off,
        4,
    );
    e.call(
        TIME_STAMP_SAVE,
        &args![
            this.byte_add(CombatGroup::fLastGroupStrategyChosenTimeStamp.off),
            buffer
        ],
    );
    save_timer(e, buffer, this, CombatGroup::AVOID_THREAT_DIALOGUE_TIMER);
    save_timer(e, buffer, this, CombatGroup::CLUSTER_UPDATE_TIMER);
    save_timer(e, buffer, this, CombatGroup::TARGET_UPDATE_TIMER);
    for timer in 0..10u32 {
        save_timer(
            e,
            buffer,
            this,
            CombatGroup::DETECTION_DIALOGUE_TIMERS + timer * 8,
        );
    }
    save_field(e, buffer, this, CombatGroup::iSearchCount.off, 4);
    if e.get(this, CombatGroup::iSearchCount) != 0 {
        let member = e.get(this, CombatGroup::pSearchingMember);
        e.call(SAVE_FORM_ID, &args![buffer, member, 0u32]);
        save_field(e, buffer, this, CombatGroup::SEARCH_FOCAL_POINT, 0xc);
        save_timer(e, buffer, this, CombatGroup::SEARCH_UPDATE_TIMER);
        save_timer(e, buffer, this, CombatGroup::SEARCH_AREA_UPDATE_TIMER);
        e.call(
            TIME_STAMP_SAVE,
            &args![
                this.byte_add(CombatGroup::fSearchStartedTimeStamp.off),
                buffer
            ],
        );
        e.call(
            WORLD_LOCATION_SAVE,
            &args![this.byte_add(CombatGroup::SEARCH_CENTER), buffer],
        );
        save_field(e, buffer, this, CombatGroup::fSearchRadius.off, 4);
        let locations = this.byte_add(CombatGroup::SEARCH_LOCATIONS);
        let count = array_count(e, locations);
        e.call(SAVE_SIZED_VALUE, &args![buffer, count]);
        for index in 0..count {
            let entry = e
                .call(SEARCH_LOCATION_ELEMENT, &args![locations, index])
                .ptr::<CombatSearchLocation>();
            e.call(WORLD_LOCATION_SAVE, &args![entry, buffer]);
            save_field(e, buffer, entry, CombatSearchLocation::fScore.off, 4);
            let target_id = e.get(entry, CombatSearchLocation::iTargetID);
            e.call(SAVE_FORM_ID_PLAIN, &args![buffer, target_id, 0u32]);
            e.call(
                TIME_STAMP_SAVE,
                &args![entry.byte_add(CombatSearchLocation::fTimeStamp.off), buffer],
            );
        }
        let doors = this.byte_add(CombatGroup::SEARCH_TELEPORT_DOORS);
        let count = array_count(e, doors);
        e.call(SAVE_SIZED_VALUE, &args![buffer, count]);
        for index in 0..count {
            let door = e
                .call(SEARCH_DOOR_ELEMENT, &args![doors, index])
                .ptr::<()>();
            let reference = e.mem.u32(door.addr());
            e.call(SAVE_FORM_ID, &args![buffer, reference, 0u32]);
            let linked = e.mem.u32(door.addr() + 4);
            e.call(SAVE_FORM_ID, &args![buffer, linked, 0u32]);
            for offset in [8u32, 9, 0xa] {
                e.call(
                    SAVE_BYTES,
                    &args![buffer, door.byte_add(offset), 1u32, 0u32],
                );
            }
        }
        let grid_slot = this.byte_add(CombatGroup::spPathingLOSGridMap.off);
        let has_grid = ni_pointer_get(e, grid_slot) != 0;
        e.with_stack(4, |e, flag| {
            e.mem.set_u8(flag.addr(), has_grid as u8);
            e.call(SAVE_BYTES, &args![buffer, flag, 1u32, 0u32]);
        });
        if has_grid {
            let grid = ni_pointer_get(e, grid_slot);
            e.call(LOS_GRID_MAP_SAVE, &args![grid, buffer]);
        }
    }
    save_timer(e, buffer, this, CombatGroup::COMBAT_STRENGTH_TIMER);
    save_field(e, buffer, this, CombatGroup::fMemberCombatStrength.off, 4);
    save_field(
        e,
        buffer,
        this,
        CombatGroup::fAverageMemberDamagePerSecond.off,
        4,
    );
    save_field(e, buffer, this, CombatGroup::fTargetCombatStrength.off, 4);
    save_field(
        e,
        buffer,
        this,
        CombatGroup::fAverageTargetCombatStrength.off,
        4,
    );
    save_timer(e, buffer, this, CombatGroup::COMBAT_MUSIC_UPDATE_TIMER);
    save_field(e, buffer, this, CombatGroup::cCombatMusicState.off, 1);
}

/// The size of an embedded array as the folded `SaveGame` accessors read it
/// (they test the address of the array for null first, which never fails).
fn array_count<T>(e: &mut Engine, array: Ptr<T>) -> u32 {
    if array.addr() == 0 {
        0
    } else {
        e.call(ARRAY_SIZE, &args![array]).u32()
    }
}

/// `LoadFormID` of the loaders: reads a form ID from the buffer, looks the
/// form up and casts it with `__RTDynamicCast` to `target_type`. Returns the
/// cast result.
fn load_form_cast(e: &mut Engine, buffer: Ptr, target_type: u32) -> u32 {
    let form_id = e.call(LOAD_FORM_ID_VALUE, &args![buffer]).u32();
    let form = e.call(FORM_LOOKUP, &args![form_id]).u32();
    e.call(
        RT_DYNAMIC_CAST,
        &args![form, 0u32, CAST_SOURCE_TYPE, target_type, 0u32],
    )
    .u32()
}

/// Loads `size` bytes from the buffer into the object at `base + offset`.
fn load_field<T>(e: &mut Engine, buffer: Ptr, base: Ptr<T>, offset: u32, size: u32) {
    e.call(LOAD_BYTES, &args![buffer, base.byte_add(offset), size]);
}

/// `CombatTimer::LoadGame` of the timer at `offset` in the group.
fn load_timer(e: &mut Engine, buffer: Ptr, this: Ptr<CombatGroup>, offset: u32) {
    e.call(TIMER_LOAD, &args![this.byte_add(offset), buffer]);
}

// Translated from 0098e3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup::LoadGame` (Xbox PDB): reads what [`combat_group_save_game`]
/// wrote, in the same order. The clusters (count, a new 0x14-byte cluster
/// each, read by `fn_00985b10`), the targets (resized, each
/// `CombatTarget::LoadGame`), the members (each `CombatMember::LoadGame`);
/// the strategy byte (the strategy table entry when it is not 0xff, else the
/// strategy stays); `bStrategyForced`; the choose and update timers;
/// `iLastGroupStrategyChosenIndex` and its time stamp; the avoid-threat
/// dialogue, cluster-update and target-update timers; the ten detection
/// dialogue timers; `iSearchCount`. When that is not zero the search state
/// follows: the searching member (form ID), the focal point, the two search
/// timers, `fSearchStartedTimeStamp` (only when the buffer's version, the
/// result of its virtual method at slot 0, is 5 or more), the centre, the
/// radius, the search locations, the teleport doors (two form IDs cast to the
/// door type and three bytes each) and, behind a flag byte, a new
/// `PathingLOSGridMap` (0x54 bytes) that is loaded; `bUpdateSearchDebugGeometry`
/// is then set. Last come the combat strength timer, the four strength
/// values, the music timer and `cCombatMusicState`. The C++ exception frame
/// is not translated.
pub fn combat_group_load_game(e: &mut Engine, this: Ptr<CombatGroup>, buffer: Ptr) {
    let clusters = this.byte_add(CombatGroup::CLUSTER_ARRAY);
    let count = e.call(LOAD_SIZED_VALUE, &args![buffer]).u32();
    e.call(POINTER_ARRAY_SET_SIZE, &args![clusters, count, 1u32]);
    for index in 0..count {
        let slot = e.call(CLUSTER_AT, &args![clusters, index]).u32();
        let memory = e.call(OPERATOR_NEW_SMALL, &args![0x14u32]).u32();
        let cluster = if memory == 0 {
            Ptr::NULL
        } else {
            fn_0098dc00(e, Ptr::new(memory))
        };
        e.mem.set_u32(slot, cluster.addr());
        fn_00985b10(e, cluster, buffer, this.addr());
    }
    let targets = this.byte_add(CombatGroup::TARGET_ARRAY);
    let count = e.call(LOAD_SIZED_VALUE, &args![buffer]).u32();
    fn_0098ed80(e, targets.cast(), count, 1);
    for index in 0..count {
        let target = e
            .call(TARGET_AT, &args![targets, index])
            .ptr::<CombatTarget>();
        combat_target_load_game(e, target, buffer, this.addr());
    }
    let members = this.byte_add(CombatGroup::MEMBER_ARRAY);
    let count = e.call(LOAD_SIZED_VALUE, &args![buffer]).u32();
    fn_0098f040(e, members.cast(), count, 1);
    for index in 0..count {
        let member = e
            .call(MEMBER_AT, &args![members, index])
            .ptr::<CombatMember>();
        combat_member_load_game(e, member, buffer, this);
    }
    let strategy_byte = e.with_stack(4, |e, byte| {
        e.mem.set_u8(byte.addr(), 0xff);
        e.call(LOAD_BYTES, &args![buffer, byte, 1u32]);
        e.mem.u8(byte.addr()) as i8
    });
    if strategy_byte != -1 {
        let strategy = fn_0098e980(e, i32::from(strategy_byte));
        e.set(this, CombatGroup::pGroupStrategy, Ptr::new(strategy));
    }
    load_field(e, buffer, this, CombatGroup::bStrategyForced.off, 1);
    load_timer(e, buffer, this, CombatGroup::CHOOSE_STRATEGY_TIMER);
    load_timer(e, buffer, this, CombatGroup::UPDATE_STRATEGY_TIMER);
    load_field(
        e,
        buffer,
        this,
        CombatGroup::iLastGroupStrategyChosenIndex.off,
        4,
    );
    e.call(
        TIME_STAMP_LOAD,
        &args![
            this.byte_add(CombatGroup::fLastGroupStrategyChosenTimeStamp.off),
            buffer
        ],
    );
    load_timer(e, buffer, this, CombatGroup::AVOID_THREAT_DIALOGUE_TIMER);
    load_timer(e, buffer, this, CombatGroup::CLUSTER_UPDATE_TIMER);
    load_timer(e, buffer, this, CombatGroup::TARGET_UPDATE_TIMER);
    for timer in 0..10u32 {
        load_timer(
            e,
            buffer,
            this,
            CombatGroup::DETECTION_DIALOGUE_TIMERS + timer * 8,
        );
    }
    load_field(e, buffer, this, CombatGroup::iSearchCount.off, 4);
    if e.get(this, CombatGroup::iSearchCount) != 0 {
        e.call(
            LOAD_FORM_ID,
            &args![buffer, this.byte_add(CombatGroup::pSearchingMember.off)],
        );
        load_field(e, buffer, this, CombatGroup::SEARCH_FOCAL_POINT, 0xc);
        load_timer(e, buffer, this, CombatGroup::SEARCH_UPDATE_TIMER);
        load_timer(e, buffer, this, CombatGroup::SEARCH_AREA_UPDATE_TIMER);
        if e.vcall(buffer.addr(), 0, &args![]).u8() >= 5 {
            e.call(
                TIME_STAMP_LOAD,
                &args![
                    this.byte_add(CombatGroup::fSearchStartedTimeStamp.off),
                    buffer
                ],
            );
        }
        e.call(
            WORLD_LOCATION_LOAD,
            &args![this.byte_add(CombatGroup::SEARCH_CENTER), buffer],
        );
        load_field(e, buffer, this, CombatGroup::fSearchRadius.off, 4);
        let locations = this.byte_add(CombatGroup::SEARCH_LOCATIONS);
        let count = e.call(LOAD_SIZED_VALUE, &args![buffer]).u32();
        fn_0098f2f0(e, locations.cast(), count, 1);
        for index in 0..count {
            let entry = fn_0098ebc0(e, locations, index);
            let entry = Ptr::<CombatSearchLocation>::new(entry);
            e.call(WORLD_LOCATION_LOAD, &args![entry, buffer]);
            load_field(e, buffer, entry, CombatSearchLocation::fScore.off, 4);
            e.call(
                LOAD_FORM_ID,
                &args![buffer, entry.byte_add(CombatSearchLocation::iTargetID.off)],
            );
            e.call(
                TIME_STAMP_LOAD,
                &args![entry.byte_add(CombatSearchLocation::fTimeStamp.off), buffer],
            );
        }
        let doors = this.byte_add(CombatGroup::SEARCH_TELEPORT_DOORS);
        let count = e.call(LOAD_SIZED_VALUE, &args![buffer]).u32();
        e.call(SEARCH_DOOR_RESIZE, &args![doors, count, 1u32]);
        for index in 0..count {
            let door = e
                .call(SEARCH_DOOR_ELEMENT, &args![doors, index])
                .ptr::<()>();
            let reference = load_form_cast(e, buffer, CAST_DOOR_TARGET_TYPE);
            e.mem.set_u32(door.addr(), reference);
            let linked = load_form_cast(e, buffer, CAST_DOOR_TARGET_TYPE);
            e.mem.set_u32(door.addr() + 4, linked);
            for offset in [8u32, 9, 0xa] {
                e.call(LOAD_BYTES, &args![buffer, door.byte_add(offset), 1u32]);
            }
        }
        let has_grid = e.with_stack(4, |e, flag| {
            e.mem.set_u8(flag.addr(), 0);
            e.call(LOAD_BYTES, &args![buffer, flag, 1u32]);
            e.mem.u8(flag.addr()) != 0
        });
        if has_grid {
            let memory = e.call(OPERATOR_NEW, &args![LOS_GRID_MAP_SIZE]).u32();
            let grid = if memory == 0 {
                0
            } else {
                e.call(LOS_GRID_MAP_CONSTRUCT, &args![memory]).u32()
            };
            let grid_slot = this.byte_add(CombatGroup::spPathingLOSGridMap.off);
            e.call(NI_POINTER_ASSIGN, &args![grid_slot, grid]);
            let loaded = ni_pointer_get(e, grid_slot);
            e.call(LOS_GRID_MAP_LOAD, &args![loaded, buffer]);
        }
        e.set(this, CombatGroup::bUpdateSearchDebugGeometry, true);
    }
    load_timer(e, buffer, this, CombatGroup::COMBAT_STRENGTH_TIMER);
    load_field(e, buffer, this, CombatGroup::fMemberCombatStrength.off, 4);
    load_field(
        e,
        buffer,
        this,
        CombatGroup::fAverageMemberDamagePerSecond.off,
        4,
    );
    load_field(e, buffer, this, CombatGroup::fTargetCombatStrength.off, 4);
    load_field(
        e,
        buffer,
        this,
        CombatGroup::fAverageTargetCombatStrength.off,
        4,
    );
    load_timer(e, buffer, this, CombatGroup::COMBAT_MUSIC_UPDATE_TIMER);
    load_field(e, buffer, this, CombatGroup::cCombatMusicState.off, 1);
}

// Translated from 0098e980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Entry `index` of the table of group strategies at `011f18b4` (cdecl).
pub fn fn_0098e980(e: &mut Engine, index: i32) -> u32 {
    e.global::<u32>(STRATEGY_TABLE.wrapping_add((index as u32).wrapping_mul(4)))
}

// Translated from 0098e990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup::InitLoadGame` (Xbox PDB): after a load, turns the saved
/// references of the group into pointers. Every target's and member's actor
/// word is resolved with `fn_00985930` (`forwarded` is passed through to it
/// and ignored there); an entry whose actor is gone is removed (the index
/// steps back); a surviving target is announced to the combat manager
/// (`TargetLocalActor`, actor and group). When a search is running the
/// searching member is looked up and cast the same way. At the end the
/// counts are refreshed. The C++ exception frame is not translated.
pub fn combat_group_init_load_game(e: &mut Engine, this: Ptr<CombatGroup>, forwarded: u32) {
    let targets = this.byte_add(CombatGroup::TARGET_ARRAY);
    let mut count = e.call(ARRAY_SIZE, &args![targets]).u32();
    let mut index = 0u32;
    while index < count {
        let target = e.call(TARGET_AT, &args![targets, index]).u32();
        fn_00985930(e, Ptr::new(target), forwarded, this.addr());
        let actor = e.mem.u32(target);
        if actor != 0 {
            let manager = combat_manager(e);
            e.call(
                MANAGER_TARGET_LOCAL_ACTOR,
                &args![manager, actor, this.addr()],
            );
        } else {
            fn_0098ef10(e, targets.cast(), index, 1);
            index = index.wrapping_sub(1);
            count = count.wrapping_sub(1);
        }
        index = index.wrapping_add(1);
    }
    let members = this.byte_add(CombatGroup::MEMBER_ARRAY);
    let mut count = e.call(ARRAY_SIZE, &args![members]).u32();
    let mut index = 0u32;
    while index < count {
        let member = e.call(MEMBER_AT, &args![members, index]).u32();
        fn_00985930(e, Ptr::new(member), forwarded, this.addr());
        if e.mem.u32(member) == 0 {
            fn_0098f1f0(e, members.cast(), index, 1);
            index = index.wrapping_sub(1);
            count = count.wrapping_sub(1);
        }
        index = index.wrapping_add(1);
    }
    if e.call(SEARCH_RUNNING, &args![this]).bool() {
        let searching = e.get(this, CombatGroup::pSearchingMember);
        let resolved = if searching.is_null() {
            0
        } else {
            let form = e.call(FORM_LOOKUP, &args![searching]).u32();
            e.call(
                RT_DYNAMIC_CAST,
                &args![form, 0u32, CAST_SOURCE_TYPE, CAST_TARGET_TYPE, 0u32],
            )
            .u32()
        };
        e.set(this, CombatGroup::pSearchingMember, Ptr::new(resolved));
    }
    combat_group_update_counts(e, this);
}

// Translated from 0098eb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroup` (the map has no name): first `fn_0098cf70`, then every
/// member other than the player whose actor has no process (virtual method
/// at slot 0x428), or whose process' word at `+0x80` is not this group, is
/// removed with `fn_009869a0` (the index steps back). The second stack word
/// is never read.
pub fn fn_0098eb30(e: &mut Engine, this: Ptr<CombatGroup>, _unused_1: u32) {
    fn_0098cf70(e, this);
    let mut index = 0u32;
    while index < e.call(MEMBER_COUNT, &args![this]).u32() {
        let actor = e.call(MEMBER_ACTOR_AT, &args![this, index]).u32();
        if actor != e.global::<u32>(PLAYER) {
            let process = e.vcall(actor, 0x428, &args![]).u32();
            if process == 0 || e.call(PROCESS_WORD_80, &args![process]).u32() != this.addr() {
                fn_009869a0(e, this, Ptr::new(actor));
                index = index.wrapping_sub(1);
            }
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 0098ebc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Element `index` of a `BSSimpleArray<CombatSearchLocation>`: forwards to
/// the instance at `006bf670` and returns its result.
pub fn fn_0098ebc0(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    e.call(SEARCH_LOCATION_SLOT, &args![this, index]).u32()
}

// Translated from 0098ebe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<T *, 1024>` removal of `count` pointers from `index` (the
/// map has no name). When `count` reaches the size the array is cleared
/// (`008454f0`, with `shrink`) and nothing else happens. Otherwise the
/// removed slots are destroyed and the tail is copied down; with `shrink`
/// set and `006f3170` agreeing, the array first moves to a buffer of the
/// size `00869600` asks for (the tail copy there keeps `size - count` as its
/// count, as the code does). The size drops by `count`.
pub fn fn_0098ebe0(e: &mut Engine, this: Ptr<BSSimpleArray>, index: u32, count: u32, shrink: u8) {
    let size = e.get(this, BSSimpleArray::iSize);
    if count >= size {
        e.call(ARRAY_CLEAR, &args![this, u32::from(shrink)]);
        return;
    }
    let slot = |buffer: u32| buffer.wrapping_add(index.wrapping_mul(4));
    if shrink != 0 && e.call(POINTER_ARRAY_SHOULD_SHRINK, &args![this]).bool() {
        let new_reserved = e.call(POINTER_ARRAY_SHRUNK_RESERVE, &args![this]).u32();
        let new_buffer = e.vcall(this.addr(), 4, &args![new_reserved]).u32();
        let old_buffer = e.get(this, BSSimpleArray::pBuffer);
        e.call(
            ARRAY_COPY_ELEMENTS,
            &args![this, new_buffer, old_buffer, index],
        );
        let old_buffer = e.get(this, BSSimpleArray::pBuffer);
        e.call(
            ARRAY_DESTRUCT_ELEMENTS,
            &args![this, slot(old_buffer), count],
        );
        let size = e.get(this, BSSimpleArray::iSize);
        let old_buffer = e.get(this, BSSimpleArray::pBuffer);
        e.call(
            ARRAY_COPY_ELEMENTS,
            &args![
                this,
                slot(new_buffer),
                slot(old_buffer).wrapping_add(count.wrapping_mul(4)),
                size.wrapping_sub(count)
            ],
        );
        e.call(ARRAY_RELEASE_BUFFER, &args![this]);
        e.set(this, BSSimpleArray::pBuffer, new_buffer);
        e.set(this, BSSimpleArray::iReservedSize, new_reserved);
    } else {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        e.call(ARRAY_DESTRUCT_ELEMENTS, &args![this, slot(buffer), count]);
        let size = e.get(this, BSSimpleArray::iSize);
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        e.call(
            ARRAY_COPY_ELEMENTS,
            &args![
                this,
                slot(buffer),
                slot(buffer).wrapping_add(count.wrapping_mul(4)),
                size.wrapping_sub(index).wrapping_sub(count)
            ],
        );
    }
    let size = e.get(this, BSSimpleArray::iSize);
    e.set(this, BSSimpleArray::iSize, size.wrapping_sub(count));
}

/// `BSSimpleArray<T, 1024>::SetSize(new_size, shrink)` for one instance.
/// Zero clears the array (`008454f0`). Growing past the reserve allocates
/// (virtual method at `+4`) when there is no buffer yet, else calls the
/// instance's `grow`; the new elements are constructed. Shrinking destroys
/// the dropped elements and, with `shrink` set and the new size at most a
/// quarter of the reserve, reallocates to exactly the new size. Within the
/// reserve and above the size, the new elements are constructed.
fn array_set_size(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    new_size: u32,
    shrink: u8,
    kind: &ArrayInstance,
) {
    if new_size == 0 {
        e.call(ARRAY_CLEAR, &args![this, u32::from(shrink)]);
        return;
    }
    let reserved = e.get(this, BSSimpleArray::iReservedSize);
    let size = e.get(this, BSSimpleArray::iSize);
    if new_size > reserved {
        if reserved == 0 {
            let buffer = e.vcall(this.addr(), 4, &args![new_size]).u32();
            e.set(this, BSSimpleArray::pBuffer, buffer);
        } else {
            e.call(kind.grow, &args![this, new_size, size]);
        }
        e.set(this, BSSimpleArray::iReservedSize, new_size);
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        e.call(
            kind.construct,
            &args![
                this,
                size.wrapping_mul(kind.element).wrapping_add(buffer),
                new_size.wrapping_sub(size)
            ],
        );
        e.set(this, BSSimpleArray::iSize, new_size);
    } else if new_size < size {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        e.call(
            ARRAY_DESTRUCT_ELEMENTS,
            &args![
                this,
                new_size.wrapping_mul(kind.element).wrapping_add(buffer),
                size - new_size
            ],
        );
        e.set(this, BSSimpleArray::iSize, new_size);
        if shrink != 0 && new_size <= e.get(this, BSSimpleArray::iReservedSize) >> 2 {
            e.call(kind.grow, &args![this, new_size, new_size]);
            e.set(this, BSSimpleArray::iReservedSize, new_size);
        }
    } else {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        e.call(
            kind.construct,
            &args![
                this,
                size.wrapping_mul(kind.element).wrapping_add(buffer),
                new_size - size
            ],
        );
        e.set(this, BSSimpleArray::iSize, new_size);
    }
}

/// `BSSimpleArray<T, 1024>::RemoveAt(index, count)` for one instance: the
/// number `n` of tail elements to move is `0042f5a0(size - index - count,
/// count)`; the removed elements are destroyed and the last `n` elements of
/// the array are moved into the start of the hole.
fn array_remove_at(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    index: u32,
    count: u32,
    kind: &ArrayInstance,
) {
    let size = e.get(this, BSSimpleArray::iSize);
    let tail = e
        .call(
            ARRAY_MOVE_COUNT,
            &args![size.wrapping_sub(index).wrapping_sub(count), count],
        )
        .u32();
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    let hole = index.wrapping_mul(kind.element).wrapping_add(buffer);
    e.call(ARRAY_DESTRUCT_ELEMENTS, &args![this, hole, count]);
    let size = e.get(this, BSSimpleArray::iSize);
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    let from = size
        .wrapping_mul(kind.element)
        .wrapping_add(buffer)
        .wrapping_sub(tail.wrapping_mul(kind.element));
    let hole = index.wrapping_mul(kind.element).wrapping_add(buffer);
    e.call(kind.move_tail, &args![this, hole, from, tail]);
    e.set(this, BSSimpleArray::iSize, size.wrapping_sub(count));
}

/// The array initialisers' common body: empty buffer, size and reserve, then
/// a buffer for `max(reserve, count)` elements (virtual method at `+4`) and
/// `count` constructed elements.
fn array_init(e: &mut Engine, this: Ptr<BSSimpleArray>, reserve: u32, count: u32, construct: u32) {
    e.set(this, BSSimpleArray::pBuffer, 0);
    e.set(this, BSSimpleArray::iSize, 0);
    e.set(this, BSSimpleArray::iReservedSize, 0);
    let reserve = reserve.max(count);
    if reserve != 0 {
        let buffer = e.vcall(this.addr(), 4, &args![reserve]).u32();
        e.set(this, BSSimpleArray::pBuffer, buffer);
        e.set(this, BSSimpleArray::iReservedSize, reserve);
    }
    if count != 0 {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        e.call(construct, &args![this, buffer, count]);
        e.set(this, BSSimpleArray::iSize, count);
    }
}

/// `BSSimpleArray<T, 1024>::Add`'s slot growth: when the array is full
/// (`00438b90`) it gets a first buffer of 4 (virtual method at `+4`) or the
/// reserve `009a3910` asks for (`grow`, with the size). Counts one more
/// element and returns its index.
fn array_reserve_slot(e: &mut Engine, this: Ptr<BSSimpleArray>, grow: u32) -> u32 {
    if e.call(ARRAY_IS_FULL, &args![this]).bool() {
        if e.get(this, BSSimpleArray::iReservedSize) == 0 {
            let buffer = e.vcall(this.addr(), 4, &args![4u32]).u32();
            e.set(this, BSSimpleArray::pBuffer, buffer);
            e.set(this, BSSimpleArray::iReservedSize, 4);
        } else {
            let reserve = e.call(ARRAY_GROWN_RESERVE, &args![this]).u32();
            let size = e.get(this, BSSimpleArray::iSize);
            e.call(grow, &args![this, reserve, size]);
            e.set(this, BSSimpleArray::iReservedSize, reserve);
        }
    }
    let size = e.get(this, BSSimpleArray::iSize).wrapping_add(1);
    e.set(this, BSSimpleArray::iSize, size);
    size.wrapping_sub(1)
}

// Translated from 0098ed30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<CombatTarget, 1024>` constructor: vtable `0108d6f4`, empty
/// (`fn_0098fab0(0, 0)`). Returns `this`.
pub fn fn_0098ed30(e: &mut Engine, this: Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray> {
    e.mem.set_u32(this.addr(), TARGET_ARRAY_VTABLE);
    fn_0098fab0(e, this, 0, 0);
    this
}

// Translated from 0098ed60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<CombatTarget, 1024>` destructor: vtable `0108d6f4`, then
/// the array is cleared (`008454f0(1)`).
pub fn fn_0098ed60(e: &mut Engine, this: Ptr<BSSimpleArray>) {
    e.mem.set_u32(this.addr(), TARGET_ARRAY_VTABLE);
    e.call(ARRAY_CLEAR, &args![this, 1u32]);
}

// Translated from 0098ed80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<CombatTarget, 1024>::SetSize(new_size, shrink)` (the map
/// has no name): see `array_set_size`, elements of 0x68 bytes.
pub fn fn_0098ed80(e: &mut Engine, this: Ptr<BSSimpleArray>, new_size: u32, shrink: u8) {
    array_set_size(e, this, new_size, shrink, &TARGET_ARRAY_INSTANCE);
}

// Translated from 0098eec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<CombatTarget, 1024>::Add(element)`: reserves a slot
/// (`fn_0098f7b0`), constructs it (`0098f840`) and copies the 0x68-byte
/// element in. Returns the index.
pub fn fn_0098eec0(e: &mut Engine, this: Ptr<BSSimpleArray>, element: Ptr<CombatTarget>) -> u32 {
    let index = fn_0098f7b0(e, this);
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    let slot = index.wrapping_mul(0x68).wrapping_add(buffer);
    e.call(TARGET_ARRAY_INSTANCE.construct, &args![this, slot, 1u32]);
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    let slot = buffer.wrapping_add(index.wrapping_mul(0x68));
    for word in 0..0x1a {
        let value = e.mem.u32(element.addr() + 4 * word);
        e.mem.set_u32(slot + 4 * word, value);
    }
    index
}

// Translated from 0098ef10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<CombatTarget, 1024>::RemoveAt(index, count)`: see
/// `array_remove_at`, elements of 0x68 bytes.
pub fn fn_0098ef10(e: &mut Engine, this: Ptr<BSSimpleArray>, index: u32, count: u32) {
    array_remove_at(e, this, index, count, &TARGET_ARRAY_INSTANCE);
}

// Translated from 0098efa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<CombatTarget, 1024>::_Allocate` (Xbox PDB): the game's
/// allocator (`00401000`) for `count` elements of 0x68 bytes. `this` is not
/// read.
pub fn bs_simple_array_combat_target_allocate(e: &mut Engine, _this: Ptr, count: u32) -> u32 {
    e.call(OPERATOR_NEW_SMALL, &args![count.wrapping_mul(0x68)])
        .u32()
}

// Translated from 0098efd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<CombatTarget, 1024>::_Reallocate` (Xbox PDB): calls the
/// reallocation primitive `0042f5d0(buffer, count * 0x68)` and answers the
/// new buffer (the exe ends on that call and leaves its `EAX`). `this` is not
/// read.
pub fn bs_simple_array_combat_target_reallocate(
    e: &mut Engine,
    _this: Ptr,
    buffer: u32,
    count: u32,
) -> u32 {
    e.call(
        ARRAY_REALLOC_BYTES,
        &args![buffer, count.wrapping_mul(0x68)],
    )
    .u32()
}

// Translated from 0098eff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<CombatMember, 1024>` constructor: vtable `0108d708`, empty
/// (`fn_0098fbe0(0, 0)`). Returns `this`.
pub fn fn_0098eff0(e: &mut Engine, this: Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray> {
    e.mem.set_u32(this.addr(), MEMBER_ARRAY_VTABLE);
    fn_0098fbe0(e, this, 0, 0);
    this
}

// Translated from 0098f020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<CombatMember, 1024>` destructor: vtable `0108d708`, then
/// the array is cleared (`008454f0(1)`).
pub fn fn_0098f020(e: &mut Engine, this: Ptr<BSSimpleArray>) {
    e.mem.set_u32(this.addr(), MEMBER_ARRAY_VTABLE);
    e.call(ARRAY_CLEAR, &args![this, 1u32]);
}

// Translated from 0098f040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<CombatMember, 1024>::SetSize(new_size, shrink)`: see
/// `array_set_size`, elements of 0x14 bytes.
pub fn fn_0098f040(e: &mut Engine, this: Ptr<BSSimpleArray>, new_size: u32, shrink: u8) {
    array_set_size(e, this, new_size, shrink, &MEMBER_ARRAY_INSTANCE);
}

// Translated from 0098f180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<CombatMember, 1024>::Add(element)`: reserves a slot
/// (`007881d0`), constructs it (`0098fb40`) and copies the 0x14-byte element
/// in. Returns the index.
pub fn fn_0098f180(e: &mut Engine, this: Ptr<BSSimpleArray>, element: Ptr<CombatMember>) -> u32 {
    let index = e.call(MEMBER_ARRAY_RESERVE_SLOT, &args![this]).u32();
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    let slot = index.wrapping_mul(0x14).wrapping_add(buffer);
    e.call(MEMBER_ARRAY_INSTANCE.construct, &args![this, slot, 1u32]);
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    let slot = buffer.wrapping_add(index.wrapping_mul(0x14));
    for word in 0..5 {
        let value = e.mem.u32(element.addr() + 4 * word);
        e.mem.set_u32(slot + 4 * word, value);
    }
    index
}

// Translated from 0098f1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<CombatMember, 1024>::RemoveAt(index, count)`: see
/// `array_remove_at`, elements of 0x14 bytes.
pub fn fn_0098f1f0(e: &mut Engine, this: Ptr<BSSimpleArray>, index: u32, count: u32) {
    array_remove_at(e, this, index, count, &MEMBER_ARRAY_INSTANCE);
}

// Translated from 0098f280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<CombatMember, 1024>::_Reallocate`: calls the reallocation
/// primitive `0042f5d0(buffer, count * 0x14)` and answers the new buffer (the
/// exe ends on that call and leaves its `EAX`). `this` is not read.
pub fn fn_0098f280(e: &mut Engine, _this: Ptr, buffer: u32, count: u32) -> u32 {
    e.call(
        ARRAY_REALLOC_BYTES,
        &args![buffer, count.wrapping_mul(0x14)],
    )
    .u32()
}

// Translated from 0098f2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<CombatSearchLocation, 1024>` constructor: vtable
/// `0108d71c`, empty (`fn_0098fd10(0, 0)`). Returns `this`.
pub fn fn_0098f2a0(e: &mut Engine, this: Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray> {
    e.mem.set_u32(this.addr(), SEARCH_LOCATION_ARRAY_VTABLE);
    fn_0098fd10(e, this, 0, 0);
    this
}

// Translated from 0098f2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<CombatSearchLocation, 1024>` destructor: vtable
/// `0108d71c`, then the array is cleared (`008454f0(1)`).
pub fn fn_0098f2d0(e: &mut Engine, this: Ptr<BSSimpleArray>) {
    e.mem.set_u32(this.addr(), SEARCH_LOCATION_ARRAY_VTABLE);
    e.call(ARRAY_CLEAR, &args![this, 1u32]);
}

// Translated from 0098f2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<CombatSearchLocation, 1024>::SetSize(new_size, shrink)`:
/// see `array_set_size`, elements of 0x1c bytes.
pub fn fn_0098f2f0(e: &mut Engine, this: Ptr<BSSimpleArray>, new_size: u32, shrink: u8) {
    array_set_size(e, this, new_size, shrink, &SEARCH_LOCATION_ARRAY_INSTANCE);
}

// Translated from 0098f430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<CombatSearchLocation, 1024>::RemoveAt(index, count)`: see
/// `array_remove_at`, elements of 0x1c bytes.
pub fn fn_0098f430(e: &mut Engine, this: Ptr<BSSimpleArray>, index: u32, count: u32) {
    array_remove_at(e, this, index, count, &SEARCH_LOCATION_ARRAY_INSTANCE);
}

// Translated from 0098f4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<CombatSearchDoor, 1024>` constructor: vtable `0108d730`,
/// then the initialiser `006b3f40(0, 0)`. Returns `this`.
pub fn fn_0098f4c0(e: &mut Engine, this: Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray> {
    e.mem.set_u32(this.addr(), SEARCH_DOOR_ARRAY_VTABLE);
    e.call(SEARCH_DOOR_ARRAY_INIT, &args![this, 0u32, 0u32]);
    this
}

// Translated from 0098f4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<CombatSearchDoor, 1024>` destructor: vtable `0108d730`,
/// then the array is cleared (`008454f0(1)`).
pub fn fn_0098f4f0(e: &mut Engine, this: Ptr<BSSimpleArray>) {
    e.mem.set_u32(this.addr(), SEARCH_DOOR_ARRAY_VTABLE);
    e.call(ARRAY_CLEAR, &args![this, 1u32]);
}

// Translated from 0098f510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<CombatGroupCluster *, 1024>` constructor: vtable
/// `0108d744`, then the initialiser `006b3eb0(0, 0)`. Returns `this`.
pub fn fn_0098f510(e: &mut Engine, this: Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray> {
    e.mem.set_u32(this.addr(), CLUSTER_POINTER_ARRAY_VTABLE);
    e.call(CLUSTER_POINTER_ARRAY_INIT, &args![this, 0u32, 0u32]);
    this
}

// Translated from 0098f540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<CombatGroupCluster *, 1024>` destructor: vtable
/// `0108d744`, then the array is cleared (`008454f0(1)`).
pub fn fn_0098f540(e: &mut Engine, this: Ptr<BSSimpleArray>) {
    e.mem.set_u32(this.addr(), CLUSTER_POINTER_ARRAY_VTABLE);
    e.call(ARRAY_CLEAR, &args![this, 1u32]);
}

// Translated from 0098f560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<GroupMemberData, 1024>` destructor: vtable `0108d758`,
/// then the array is cleared (`008454f0(1)`).
pub fn fn_0098f560(e: &mut Engine, this: Ptr<BSSimpleArray>) {
    e.mem.set_u32(this.addr(), GROUP_MEMBER_DATA_ARRAY_VTABLE);
    e.call(ARRAY_CLEAR, &args![this, 1u32]);
}

// Translated from 0098f580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies a `BSSimpleArray<CombatTarget, 1024>` into `other`: sizes `other`
/// to this array's size (`fn_0098ed80`, shrink on), then copies the 0x68
/// bytes of each element into `other`'s element `i` (`0096a330`).
pub fn fn_0098f580(e: &mut Engine, this: Ptr<BSSimpleArray>, other: Ptr<BSSimpleArray>) {
    let size = e.get(this, BSSimpleArray::iSize);
    fn_0098ed80(e, other, size, 1);
    let mut index = 0u32;
    while index < e.get(this, BSSimpleArray::iSize) {
        let from = e.get(this, BSSimpleArray::pBuffer) + index * 0x68;
        let to = e.call(TARGET_ARRAY_ELEMENT, &args![other, index]).u32();
        for word in 0..0x1a {
            let value = e.mem.u32(from + 4 * word);
            e.mem.set_u32(to + 4 * word, value);
        }
        index += 1;
    }
}

// Translated from 0098f5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a scrap-heap array (vtable `0106d864`; the engine map has
/// no name): the base constructor `006cf180`, the vtable, the heap word at
/// `+0x10` (`heap`, or the thread's scrap heap when `heap` is zero) and the
/// initialiser `006d0140(first, second)`. Returns `this`. The C++ exception
/// frame is not translated.
pub fn fn_0098f5f0(e: &mut Engine, this: Ptr, first: u32, second: u32, heap: u32) -> Ptr {
    e.call(SCRAP_ARRAY_BASE_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), SCRAP_ARRAY_VTABLE);
    let heap = if heap == 0 {
        let manager = e.call(MEMORY_MANAGER, &args![]).u32();
        e.call(THREAD_SCRAP_HEAP, &args![manager]).u32()
    } else {
        heap
    };
    e.mem.set_u32(this.addr() + 0x10, heap);
    e.call(SCRAP_ARRAY_INIT, &args![this, first, second]);
    this
}

/// Scalar deleting destructor of one of this unit's array instances:
/// destructs, then frees `this` when bit 0 of `flags` is set. Returns
/// `this`.
fn scalar_deleting(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    flags: u32,
    destruct: fn(&mut Engine, Ptr<BSSimpleArray>),
) -> Ptr<BSSimpleArray> {
    destruct(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0098f690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<CombatTarget, 1024>::_scalar_deleting_destructor_` (Xbox
/// PDB): `fn_0098ed60`, then `operator delete` when bit 0 of `flags` is set.
pub fn bs_simple_array_combat_target_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    flags: u32,
) -> Ptr<BSSimpleArray> {
    scalar_deleting(e, this, flags, fn_0098ed60)
}

// Translated from 0098f6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<CombatMember, 1024>::_scalar_deleting_destructor_` (Xbox
/// PDB): `fn_0098f020`, then `operator delete` when bit 0 of `flags` is set.
pub fn bs_simple_array_combat_member_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    flags: u32,
) -> Ptr<BSSimpleArray> {
    scalar_deleting(e, this, flags, fn_0098f020)
}

// Translated from 0098f6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<CombatSearchLocation, 1024>::_scalar_deleting_destructor_`
/// (Xbox PDB): `fn_0098f2d0`, then `operator delete` when bit 0 of `flags`
/// is set.
pub fn bs_simple_array_combat_search_location_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    flags: u32,
) -> Ptr<BSSimpleArray> {
    scalar_deleting(e, this, flags, fn_0098f2d0)
}

// Translated from 0098f720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<CombatSearchDoor, 1024>::_scalar_deleting_destructor_`
/// (Xbox PDB): `fn_0098f4f0`, then `operator delete` when bit 0 of `flags`
/// is set.
pub fn bs_simple_array_combat_search_door_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    flags: u32,
) -> Ptr<BSSimpleArray> {
    scalar_deleting(e, this, flags, fn_0098f4f0)
}

// Translated from 0098f750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<CombatGroupCluster *, 1024>::_scalar_deleting_destructor_`
/// (Xbox PDB): `fn_0098f540`, then `operator delete` when bit 0 of `flags`
/// is set.
pub fn bs_simple_array_combat_group_cluster_pointer_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    flags: u32,
) -> Ptr<BSSimpleArray> {
    scalar_deleting(e, this, flags, fn_0098f540)
}

// Translated from 0098f780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<GroupMemberData, 1024>::_scalar_deleting_destructor_`
/// (Xbox PDB): `fn_0098f560`, then `operator delete` when bit 0 of `flags`
/// is set.
pub fn bs_simple_array_group_member_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    flags: u32,
) -> Ptr<BSSimpleArray> {
    scalar_deleting(e, this, flags, fn_0098f560)
}

// Translated from 0098f7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<CombatTarget, 1024>` slot reservation (the map has no
/// name): see `array_reserve_slot`, growing with `0098fa00`. Returns the new
/// element's index.
pub fn fn_0098f7b0(e: &mut Engine, this: Ptr<BSSimpleArray>) -> u32 {
    array_reserve_slot(e, this, TARGET_ARRAY_INSTANCE.grow)
}

// Translated from 0098fab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initialiser of a `BSSimpleArray<CombatTarget, 1024>` (called by its
/// constructor with `(0, 0)`): see `array_init`, constructing with
/// `0098f840`.
pub fn fn_0098fab0(e: &mut Engine, this: Ptr<BSSimpleArray>, reserve: u32, count: u32) {
    array_init(e, this, reserve, count, TARGET_ARRAY_INSTANCE.construct);
}

// Translated from 0098fbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initialiser of a `BSSimpleArray<CombatMember, 1024>`: see `array_init`,
/// constructing with `0098fb40`.
pub fn fn_0098fbe0(e: &mut Engine, this: Ptr<BSSimpleArray>, reserve: u32, count: u32) {
    array_init(e, this, reserve, count, MEMBER_ARRAY_INSTANCE.construct);
}

// Translated from 0098fd10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initialiser of a `BSSimpleArray<CombatSearchLocation, 1024>`: see
/// `array_init`, constructing with `0098fc70`.
pub fn fn_0098fd10(e: &mut Engine, this: Ptr<BSSimpleArray>, reserve: u32, count: u32) {
    array_init(
        e,
        this,
        reserve,
        count,
        SEARCH_LOCATION_ARRAY_INSTANCE.construct,
    );
}
/// Writes `size` bytes of the object at `base + offset` to the save buffer.
fn save_field<T>(e: &mut Engine, buffer: Ptr, base: Ptr<T>, offset: u32, size: u32) {
    e.call(
        SAVE_BYTES,
        &args![buffer, base.byte_add(offset), size, 0u32],
    );
}

/// `CombatTimer::SaveGame` of the timer at `offset` in the group.
fn save_timer(e: &mut Engine, buffer: Ptr, this: Ptr<CombatGroup>, offset: u32) {
    e.call(TIMER_SAVE, &args![this.byte_add(offset), buffer]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0069cfe0,
            fn_0069cfe0(Ptr<BSSimpleArray>, Ptr<BSSimpleArray>)
        ),
        entry!(
            0x00985410,
            fn_00985410(Ptr<CombatTarget>, Ptr, u16) -> Ptr<CombatTarget>
        ),
        entry!(
            0x00985520,
            fn_00985520(Ptr<CombatTarget>, Ptr<CombatTarget>)
        ),
        entry!(
            0x009856e0,
            combat_target_save_game(Ptr<CombatTarget>, Ptr, u32)
        ),
        entry!(
            0x00985800,
            combat_target_load_game(Ptr<CombatTarget>, Ptr, u32)
        ),
        entry!(0x00985930, fn_00985930(Ptr, u32, u32)),
        entry!(
            0x00985980,
            fn_00985980(Ptr<CombatMember>, Ptr<CombatMember>)
        ),
        entry!(
            0x009859c0,
            combat_member_save_game(Ptr<CombatMember>, Ptr, Ptr<CombatGroup>)
        ),
        entry!(
            0x00985a50,
            combat_member_load_game(Ptr<CombatMember>, Ptr, Ptr<CombatGroup>)
        ),
        entry!(0x00985ae0, fn_00985ae0(Ptr, Ptr, u32)),
        entry!(0x00985b10, fn_00985b10(Ptr, Ptr, u32)),
        entry!(
            0x00985b40,
            fn_00985b40(Ptr<CombatSearchLocation>, Ptr) -> f32
        ),
        entry!(0x00985bb0, fn_00985bb0(Ptr<CombatSearchLocation>) -> f32),
        entry!(
            0x00985be0,
            fn_00985be0(Ptr<CombatSearchLocation>, Ptr) -> Ptr
        ),
        entry!(
            0x00985d10,
            fn_00985d10(Ptr<CombatGroup>, u32) -> Ptr<CombatGroup>
        ),
        entry!(0x00985f90, fn_00985f90(Ptr<CombatGroup>)),
        entry!(0x00986150, combat_group_update(Ptr<CombatGroup>)),
        entry!(0x009862e0, fn_009862e0(Ptr<CombatGroup>, Ptr)),
        entry!(0x009863b0, fn_009863b0(Ptr)),
        entry!(
            0x00986410,
            combat_group_add_target(Ptr<CombatGroup>, Ptr) -> bool
        ),
        entry!(
            0x00986500,
            combat_group_remove_target(Ptr<CombatGroup>, Ptr)
        ),
        entry!(0x00986560, fn_00986560(Ptr<CombatGroup>, u32)),
        entry!(
            0x009865b0,
            combat_group_is_target(Ptr<CombatGroup>, Ptr) -> bool
        ),
        entry!(
            0x009865d0,
            fn_009865d0(Ptr<CombatGroup>, Ptr) -> Ptr<CombatTarget>
        ),
        entry!(
            0x009865f0,
            fn_009865f0(Ptr<CombatGroup>, Ptr) -> Ptr<CombatMember>
        ),
        entry!(
            0x00986610,
            fn_00986610(Ptr<CombatGroup>, u32) -> Ptr<CombatTarget>
        ),
        entry!(
            0x00986670,
            fn_00986670(Ptr<CombatGroup>, u32) -> Ptr<CombatMember>
        ),
        entry!(
            0x009866d0,
            combat_group_can_add_target(Ptr<CombatGroup>, Ptr) -> bool
        ),
        entry!(0x00986740, fn_00986740(Ptr<CombatGroup>, Ptr<CombatGroup>)),
        entry!(0x00986760, fn_00986760(Ptr<CombatGroup>, Ptr)),
        entry!(
            0x009867d0,
            combat_group_add_member(Ptr<CombatGroup>, Ptr) -> bool
        ),
        entry!(
            0x00986950,
            fn_00986950(Ptr<CombatMember>, Ptr) -> Ptr<CombatMember>
        ),
        entry!(0x009869a0, fn_009869a0(Ptr<CombatGroup>, Ptr)),
        entry!(
            0x00986a80,
            combat_group_is_member(Ptr<CombatGroup>, Ptr) -> bool
        ),
        entry!(
            0x00986aa0,
            combat_group_can_add_member(Ptr<CombatGroup>, Ptr) -> bool
        ),
        entry!(0x00986b00, fn_00986b00(Ptr<CombatGroup>, Ptr)),
        entry!(0x00986b70, fn_00986b70(Ptr<CombatGroup>)),
        entry!(0x00986bd0, fn_00986bd0(Ptr<CombatGroup>)),
        entry!(0x00986c40, fn_00986c40(Ptr<CombatGroup>) -> bool),
        entry!(0x00986c60, fn_00986c60(Ptr<CombatGroup>, Ptr) -> Ptr),
        entry!(
            0x009871c0,
            fn_009871c0(Ptr<CombatGroup>, u32) -> Ptr<CombatTarget>
        ),
        entry!(0x009871e0, fn_009871e0(Ptr<CombatTarget>) -> bool),
        entry!(
            0x00987220,
            fn_00987220(Ptr<CombatGroup>, Ptr<CombatTarget>) -> bool
        ),
        entry!(
            0x009874b0,
            combat_group_increment_attacker_count(Ptr<CombatGroup>, Ptr)
        ),
        entry!(0x009874f0, fn_009874f0(Ptr<CombatGroup>, Ptr)),
        entry!(
            0x00987530,
            combat_group_merge_group(Ptr<CombatGroup>, Ptr<CombatGroup>) -> bool
        ),
        entry!(0x009877c0, fn_009877c0(Ptr<CombatGroup>) -> u32),
        entry!(0x009877e0, fn_009877e0(Ptr<CombatGroup>)),
        entry!(0x00987800, fn_00987800(Ptr<CombatGroup>) -> i8),
        entry!(0x00989010, bgs_world_location_set_location(Ptr, Ptr, u32)),
        entry!(0x00989040, fn_00989040(Ptr<CombatGroup>, Ptr) -> i32),
        entry!(0x00989090, fn_00989090(Ptr<CombatGroup>, Ptr, Ptr) -> Ptr),
        entry!(0x009890e0, fn_009890e0(Ptr<CombatGroup>, Ptr, Ptr) -> bool),
        entry!(
            0x00989140,
            fn_00989140(Ptr<CombatGroup>, u32, Ptr, f32) -> bool
        ),
        entry!(0x009891a0, fn_009891a0(Ptr<CombatGroup>, Ptr) -> u8),
        entry!(0x009891d0, fn_009891d0(Ptr<CombatGroup>, Ptr, Ptr)),
        entry!(0x00989230, fn_00989230(Ptr<CombatGroup>, u32, Ptr)),
        entry!(
            0x009892a0,
            fn_009892a0(Ptr<CombatGroup>, Ptr, Ptr, f32, Ptr, Ptr) -> Ptr
        ),
        entry!(
            0x00989490,
            fn_00989490(Ptr<CombatGroup>, u32, u32, u32, Ptr) -> Ptr
        ),
        entry!(0x009895a0, fn_009895a0(Ptr<CombatGroup>, Ptr, Ptr) -> Ptr),
        entry!(
            0x00989700,
            combat_group_find_target_near_location(Ptr<CombatGroup>, Ptr, f32, bool) -> Ptr
        ),
        entry!(0x009897a0, fn_009897a0(Ptr<CombatTarget>) -> bool),
        entry!(
            0x009897f0,
            fn_009897f0(Ptr<CombatGroup>, Ptr, f32, Ptr) -> Ptr
        ),
        entry!(0x00989880, fn_00989880(Ptr<CombatGroup>, Ptr, f32) -> Ptr),
        entry!(
            0x009898b0,
            fn_009898b0(Ptr<CombatGroup>, Ptr, Ptr, f32) -> Ptr
        ),
        entry!(0x00989970, fn_00989970(Ptr<CombatGroup>, Ptr, f32) -> Ptr),
        entry!(
            0x009899a0,
            combat_group_check_member_target_intersection(Ptr<CombatGroup>) -> bool
        ),
        entry!(0x00989e20, fn_00989e20(Ptr<CombatGroup>, Ptr) -> f32),
        entry!(0x00989eb0, fn_00989eb0(Ptr<CombatGroup>, Ptr) -> f32),
        entry!(0x00989f40, fn_00989f40(Ptr<CombatGroup>, Ptr, bool)),
        entry!(0x0098a110, fn_0098a110(Ptr<CombatGroup>, Ptr, bool)),
        entry!(0x0098a2e0, fn_0098a2e0(Ptr<CombatGroup>) -> bool),
        entry!(0x0098a310, fn_0098a310(Ptr<CombatGroup>, Ptr) -> u32),
        entry!(0x0098a340, fn_0098a340(Ptr<CombatGroup>, u32) -> u32),
        entry!(0x0098a360, fn_0098a360(Ptr<CombatGroup>, Ptr, u32)),
        entry!(
            0x0098a390,
            combat_group_set_member_group_strategy_assignment(Ptr<CombatGroup>, u32, u32)
        ),
        entry!(0x0098a3c0, fn_0098a3c0(Ptr<CombatGroup>)),
        entry!(
            0x0098a410,
            combat_group_find_member_with_strategy_assignment(Ptr<CombatGroup>, u32) -> Ptr
        ),
        entry!(
            0x0098a470,
            combat_group_check_group_strategy_chosen_time_stamp(Ptr<CombatGroup>, u32, f32) -> bool
        ),
        entry!(0x0098a4d0, fn_0098a4d0(Ptr<CombatGroup>) -> f32),
        entry!(0x0098a580, fn_0098a580(Ptr<CombatGroup>)),
        entry!(0x0098ad70, fn_0098ad70(Ptr) -> bool),
        entry!(0x0098adb0, fn_0098adb0(Ptr, f32)),
        entry!(0x0098add0, fn_0098add0(Ptr<CombatGroup>)),
        entry!(
            0x0098afb0,
            combat_group_update_search_area(Ptr<CombatGroup>) -> bool
        ),
        entry!(0x0098b070, fn_0098b070(Ptr<CombatGroup>, Ptr, Ptr, Ptr)),
        entry!(0x0098b100, fn_0098b100(Ptr<CombatGroup>)),
        entry!(
            0x0098b8c0,
            combat_group_build_search_teleport_door_array(Ptr<CombatGroup>)
        ),
        entry!(0x0098bae0, fn_0098bae0(Ptr<CombatGroup>)),
        entry!(0x0098bbc0, fn_0098bbc0(Ptr)),
        entry!(
            0x0098bc00,
            combat_group_find_search_location(Ptr<CombatGroup>, Ptr, u8, Ptr) -> Ptr
        ),
        entry!(0x0098bd70, fn_0098bd70(Ptr<CombatGroup>, Ptr)),
        entry!(0x0098bfb0, fn_0098bfb0(Ptr<CombatGroup>, Ptr, f32, u32)),
        entry!(
            0x0098c060,
            fn_0098c060(Ptr<CombatGroup>, Ptr<CombatTarget>, Ptr, Ptr, Ptr)
        ),
        entry!(0x0098c0d0, fn_0098c0d0(Ptr<CombatGroup>)),
        entry!(
            0x0098c3d0,
            combat_group_find_search_door(Ptr<CombatGroup>, Ptr, f32, u8) -> Ptr
        ),
        entry!(
            0x0098c590,
            combat_group_get_combat_search_door(Ptr<CombatGroup>, u32) -> Ptr
        ),
        entry!(0x0098c600, fn_0098c600(Ptr<CombatGroup>, u32)),
        entry!(0x0098c6e0, fn_0098c6e0(Ptr<CombatGroup>, u32)),
        entry!(
            0x0098c710,
            combat_group_failed_to_investigate_search_door(Ptr<CombatGroup>, u32)
        ),
        entry!(0x0098c750, fn_0098c750(Ptr<CombatGroup>, u32)),
        entry!(
            0x0098c780,
            combat_group_unreserve_search_door(Ptr<CombatGroup>, u32)
        ),
        entry!(
            0x0098c7b0,
            combat_group_calculate_search_ignore_locations(Ptr<CombatGroup>, Ptr, Ptr) -> bool
        ),
        entry!(
            0x0098c870,
            combat_group_update_combat_strength(Ptr<CombatGroup>)
        ),
        entry!(0x0098cb30, fn_0098cb30(Ptr<CombatGroup>, Ptr) -> f32),
        entry!(0x0098cc20, fn_0098cc20(Ptr<CombatGroup>, Ptr) -> f32),
        entry!(
            0x0098cd50,
            fn_0098cd50(Ptr<CombatGroup>, Ptr, Ptr, f32) -> Ptr
        ),
        entry!(0x0098ce80, combat_group_update_counts(Ptr<CombatGroup>)),
        entry!(0x0098cf70, fn_0098cf70(Ptr<CombatGroup>)),
        entry!(0x0098d030, fn_0098d030(Ptr<CombatGroup>)),
        entry!(0x0098d1a0, fn_0098d1a0(Ptr<CombatGroup>)),
        entry!(0x0098d2c0, fn_0098d2c0(Ptr<CombatGroup>)),
        entry!(0x0098dc00, fn_0098dc00(Ptr) -> Ptr),
        entry!(0x0098dc20, fn_0098dc20(Ptr) -> Ptr),
        entry!(0x0098dc40, fn_0098dc40(Ptr<CombatGroup>)),
        entry!(0x0098ddd0, fn_0098ddd0(Ptr, Ptr)),
        entry!(0x0098de00, fn_0098de00(Ptr<CombatGroup>, Ptr) -> u32),
        entry!(0x0098de30, fn_0098de30(Ptr<CombatGroup>, u32) -> u32),
        entry!(0x0098de60, fn_0098de60(Ptr<CombatGroup>, i8) -> u32),
        entry!(0x0098de90, combat_group_save_game(Ptr<CombatGroup>, Ptr)),
        entry!(0x0098e3d0, combat_group_load_game(Ptr<CombatGroup>, Ptr)),
        entry!(0x0098e980, fn_0098e980(i32) -> u32),
        entry!(
            0x0098e990,
            combat_group_init_load_game(Ptr<CombatGroup>, u32)
        ),
        entry!(0x0098eb30, fn_0098eb30(Ptr<CombatGroup>, u32)),
        entry!(0x0098ebc0, fn_0098ebc0(Ptr, u32) -> u32),
        entry!(0x0098ebe0, fn_0098ebe0(Ptr<BSSimpleArray>, u32, u32, u8)),
        entry!(
            0x0098ed30,
            fn_0098ed30(Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>
        ),
        entry!(0x0098ed60, fn_0098ed60(Ptr<BSSimpleArray>)),
        entry!(0x0098ed80, fn_0098ed80(Ptr<BSSimpleArray>, u32, u8)),
        entry!(
            0x0098eec0,
            fn_0098eec0(Ptr<BSSimpleArray>, Ptr<CombatTarget>) -> u32
        ),
        entry!(0x0098ef10, fn_0098ef10(Ptr<BSSimpleArray>, u32, u32)),
        entry!(
            0x0098efa0,
            bs_simple_array_combat_target_allocate(Ptr, u32) -> u32
        ),
        entry!(
            0x0098efd0,
            bs_simple_array_combat_target_reallocate(Ptr, u32, u32) -> u32
        ),
        entry!(
            0x0098eff0,
            fn_0098eff0(Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>
        ),
        entry!(0x0098f020, fn_0098f020(Ptr<BSSimpleArray>)),
        entry!(0x0098f040, fn_0098f040(Ptr<BSSimpleArray>, u32, u8)),
        entry!(
            0x0098f180,
            fn_0098f180(Ptr<BSSimpleArray>, Ptr<CombatMember>) -> u32
        ),
        entry!(0x0098f1f0, fn_0098f1f0(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x0098f280, fn_0098f280(Ptr, u32, u32) -> u32),
        entry!(
            0x0098f2a0,
            fn_0098f2a0(Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>
        ),
        entry!(0x0098f2d0, fn_0098f2d0(Ptr<BSSimpleArray>)),
        entry!(0x0098f2f0, fn_0098f2f0(Ptr<BSSimpleArray>, u32, u8)),
        entry!(0x0098f430, fn_0098f430(Ptr<BSSimpleArray>, u32, u32)),
        entry!(
            0x0098f4c0,
            fn_0098f4c0(Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>
        ),
        entry!(0x0098f4f0, fn_0098f4f0(Ptr<BSSimpleArray>)),
        entry!(
            0x0098f510,
            fn_0098f510(Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>
        ),
        entry!(0x0098f540, fn_0098f540(Ptr<BSSimpleArray>)),
        entry!(0x0098f560, fn_0098f560(Ptr<BSSimpleArray>)),
        entry!(
            0x0098f580,
            fn_0098f580(Ptr<BSSimpleArray>, Ptr<BSSimpleArray>)
        ),
        entry!(0x0098f5f0, fn_0098f5f0(Ptr, u32, u32, u32) -> Ptr),
        entry!(
            0x0098f690,
            bs_simple_array_combat_target_scalar_deleting_destructor(
                Ptr<BSSimpleArray>,
                u32,
            )
                -> Ptr<BSSimpleArray>
        ),
        entry!(
            0x0098f6c0,
            bs_simple_array_combat_member_scalar_deleting_destructor(
                Ptr<BSSimpleArray>,
                u32,
            )
                -> Ptr<BSSimpleArray>
        ),
        entry!(
            0x0098f6f0,
            bs_simple_array_combat_search_location_scalar_deleting_destructor(
                Ptr<BSSimpleArray>,
                u32,
            )
                -> Ptr<BSSimpleArray>
        ),
        entry!(
            0x0098f720,
            bs_simple_array_combat_search_door_scalar_deleting_destructor(
                Ptr<BSSimpleArray>,
                u32,
            )
                -> Ptr<BSSimpleArray>
        ),
        entry!(
            0x0098f750,
            bs_simple_array_combat_group_cluster_pointer_scalar_deleting_destructor(
                Ptr<BSSimpleArray>,
                u32,
            )
                -> Ptr<
                BSSimpleArray,
            >
        ),
        entry!(
            0x0098f780,
            bs_simple_array_group_member_data_scalar_deleting_destructor(
                Ptr<BSSimpleArray>,
                u32,
            )
                -> Ptr<BSSimpleArray>
        ),
        entry!(0x0098f7b0, fn_0098f7b0(Ptr<BSSimpleArray>) -> u32),
        entry!(0x0098fab0, fn_0098fab0(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x0098fbe0, fn_0098fbe0(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x0098fd10, fn_0098fd10(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x00990500, fn_00990500(Ptr<CombatGroupData>) -> f32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The game time `00435dd0` returns in these tests.
    const NOW: f32 = 100.0;
    /// Where the fake actors' vtable lives.
    const ACTOR_VTABLE: u32 = 0x7100_0000;
    /// Address of the fake virtual method at `slot` of the actors' vtable.
    const fn fake(slot: u32) -> u32 {
        0x7000_0000 + slot
    }

    /// Engine with the data the code reads mapped, the array, form and time
    /// stamp accessors doubled over memory, and an actor vtable. The tests
    /// double everything else they reach.
    ///
    /// Fake actor layout: +0x0c form ID, +0x30 3D root, +0x34 / +0x35 the
    /// results of virtual methods 0x22c / 0x230, +0x38 process, +0x40 the
    /// position slot 0x1f4 returns.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_2000u32,
            0x0101_6000,
            0x0101_d000,
            0x0101_e000,
            0x0101_f000,
            0x0102_0000,
            0x0102_3000,
            0x0102_6000,
            0x0105_1000,
            0x0103_2000,
            0x0108_d000,
            0x011a_4000,
            0x011c_e000,
            0x011c_f000,
            0x011f_1000,
            0x011f_4000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(0x011f_1bf0, NOW);
        e.set_global(MAX_FLOAT, f32::MAX);
        e.set_global(MINUS_ONE, -1.0f32);
        e.set_global(ZERO_DOUBLE, 0.0f64);

        e.register(ARRAY_SIZE, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(TARGET_COUNT, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        e.register(MEMBER_COUNT, |e, a| e.mem.u32(a[0] + 0x20).into_ret());
        e.register(TARGET_AT, |e, a| {
            (e.mem.u32(a[0] + 4) + a[1] * 0x68).into_ret()
        });
        e.register(MEMBER_AT, |e, a| {
            (e.mem.u32(a[0] + 4) + a[1] * 0x14).into_ret()
        });
        e.register(TARGET_ACTOR_AT, |e, a| {
            e.mem.u32(e.mem.u32(a[0] + 0x0c) + a[1] * 0x68).into_ret()
        });
        e.register(MEMBER_ACTOR_AT, |e, a| {
            e.mem.u32(e.mem.u32(a[0] + 0x1c) + a[1] * 0x14).into_ret()
        });
        e.register(CLUSTER_AT, |e, a| {
            (e.mem.u32(a[0] + 4) + a[1] * 4).into_ret()
        });
        e.register(FORM_ID, |e, a| e.mem.u32(a[0] + 0x0c).into_ret());
        e.register(TIME_STAMP_VALUE, |e, a| e.mem.f32(a[0]).into_ret());
        e.register(GAME_TIME_NOW, |e, _| {
            e.global::<f32>(0x011f_1bf0).into_ret()
        });
        e.register(TIME_STAMP_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        e.register(TIME_STAMP_ZERO, |e, a| {
            e.mem.set_f32(a[0], 0.0);
            a[0].into_ret()
        });
        e.register(TIME_STAMP_AGE, |e, a| (NOW - e.mem.f32(a[0])).into_ret());
        // `0040ebd0` is the smaller of its two floats (ties: the second),
        // `00404010` the larger (ties: the second).
        e.register(0x0040_ebd0, |_, a| {
            let (x, y) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
            (if y <= x { y } else { x }).into_ret()
        });
        e.register(0x0040_4010, |_, a| {
            let (x, y) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
            (if y < x { x } else { y }).into_ret()
        });
        // A setting accessor answers with the address it was given, so a
        // test writes the setting's value there.
        e.register(SETTING_BOOL_VALUE, |_, a| a[0].into_ret());
        e.register(SETTING_FLOAT_VALUE, |_, a| a[0].into_ret());
        e.register(LOG_MESSAGE, |_, _| Ret::default());

        let mut slots = vec![0u32; 0x120];
        for slot in [0x130u32, 0x1d0, 0x1f4, 0x22c, 0x230, 0x2e8, 0x3fc, 0x428] {
            slots[(slot / 4) as usize] = fake(slot);
        }
        e.put_vtable(ACTOR_VTABLE, &slots);
        e.register(fake(0x130), |_, _| 0xbeefu32.into_ret());
        e.register(fake(0x1d0), |e, a| e.mem.u32(a[0] + 0x30).into_ret());
        e.register(fake(0x1f4), |_, a| (a[0] + 0x40).into_ret());
        e.register(fake(0x22c), |e, a| e.mem.u8(a[0] + 0x34).into_ret());
        e.register(fake(0x230), |e, a| e.mem.u8(a[0] + 0x35).into_ret());
        e.register(fake(0x3fc), |_, _| Ret::default());
        e.register(fake(0x428), |e, a| e.mem.u32(a[0] + 0x38).into_ret());
        e
    }

    fn actor(e: &mut Engine, form_id: u32) -> Ptr {
        let actor = e.mem.alloc(0x80);
        e.mem.set_u32(actor, ACTOR_VTABLE);
        e.mem.set_u32(actor + 0x0c, form_id);
        Ptr::new(actor)
    }

    /// A group whose arrays hold the given target and member actors. The
    /// fake accessors read the arrays at +0x08 (buffer at +0x0c, size at
    /// +0x10) and +0x18 (buffer +0x1c, size +0x20).
    fn group_with(e: &mut Engine, targets: &[Ptr], members: &[Ptr]) -> Ptr<CombatGroup> {
        let group = e.new_object::<CombatGroup>();
        let target_buffer = e.mem.alloc(0x68 * (targets.len() as u32 + 4));
        for (i, target) in targets.iter().enumerate() {
            e.mem
                .set_u32(target_buffer + 0x68 * i as u32, target.addr());
        }
        e.mem.set_u32(group.addr() + 0x0c, target_buffer);
        e.mem.set_u32(group.addr() + 0x10, targets.len() as u32);
        let member_buffer = e.mem.alloc(0x14 * (members.len() as u32 + 4));
        for (i, member) in members.iter().enumerate() {
            e.mem
                .set_u32(member_buffer + 0x14 * i as u32, member.addr());
        }
        e.mem.set_u32(group.addr() + 0x1c, member_buffer);
        e.mem.set_u32(group.addr() + 0x20, members.len() as u32);
        group
    }

    fn target_entry(e: &Engine, group: Ptr<CombatGroup>, index: u32) -> Ptr<CombatTarget> {
        Ptr::new(e.mem.u32(group.addr() + 0x0c) + 0x68 * index)
    }

    /// Doubles answering 0 for the given addresses, and starts the call log.
    fn record(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    fn addresses(e: &Engine) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(a, _)| *a)
            .collect()
    }

    #[test]
    fn search_door_array_copy_sizes_the_other_array_then_copies() {
        let mut e = engine();
        let source = e.new_object::<BSSimpleArray>();
        let buffer = e.mem.alloc(24);
        for word in 0..6 {
            e.mem.set_u32(buffer + 4 * word, 0x100 + word);
        }
        e.set(source, BSSimpleArray::pBuffer, buffer);
        e.set(source, BSSimpleArray::iSize, 2);
        let other = e.new_object::<BSSimpleArray>();
        let destination = e.mem.alloc(24);
        e.set(other, BSSimpleArray::pBuffer, destination);
        e.register(SEARCH_DOOR_AT, |e, a| {
            (e.mem.u32(a[0] + 4) + a[1] * 12).into_ret()
        });
        record(&mut e, &[SEARCH_DOOR_ARRAY_SET_SIZE]);
        fn_0069cfe0(&mut e, source, other);
        assert_eq!(
            calls(&e, SEARCH_DOOR_ARRAY_SET_SIZE),
            vec![vec![other.addr(), 2, 1]]
        );
        for word in 0..6 {
            assert_eq!(e.mem.u32(destination + 4 * word), 0x100 + word);
        }
    }

    #[test]
    fn combat_target_constructor_sets_defaults_and_the_noticed_location() {
        let mut e = engine();
        e.register(WORLD_LOCATION_CONSTRUCT, |e, a| {
            for word in 0..3 {
                e.mem.set_u32(a[0] + 4 * word, 0x5555);
            }
            a[0].into_ret()
        });
        e.register(GET_WORLD_LOCATION, |e, a| {
            for word in 0..4 {
                e.mem.set_u32(a[1] + 4 * word, 0x700 + word);
            }
            a[1].into_ret()
        });
        let target = e.new_object::<CombatTarget>();
        let who = actor(&mut e, 7);
        let result = fn_00985410(&mut e, target, who, 9);
        assert_eq!(result, target);
        assert_eq!(e.get(target, CombatTarget::pActor), who);
        assert_eq!(e.get(target, CombatTarget::iDetectionLevel), 0);
        assert_eq!(e.get(target, CombatTarget::sLastSearchNoticed), 9);
        assert_eq!(e.get(target, CombatTarget::sAttackerCount), 0);
        assert_eq!(e.get(target, CombatTarget::fLastSeenTimeStamp), -f32::MAX);
        assert_eq!(
            e.get(target, CombatTarget::fLastDetectedTimeStamp),
            -f32::MAX
        );
        assert_eq!(
            e.get(target, CombatTarget::fDetectionLevelUpdateTimeStamp),
            0.0
        );
        assert_eq!(e.get(target, CombatTarget::fLastNoticedTimeStamp), NOW);
        assert_eq!(e.get(target, CombatTarget::fDetectionEventTimeStamp), 0.0);
        assert_eq!(
            e.get(target, CombatTarget::fLastAttackedMemberTimeStamp),
            -f32::MAX
        );
        // The other locations keep the constructor's default; the noticed
        // one is the actor's.
        assert_eq!(e.mem.u32(target.addr() + 0x18), 0x5555);
        for word in 0..4 {
            assert_eq!(e.mem.u32(target.addr() + 8 + 4 * word), 0x700 + word);
        }
    }

    #[test]
    fn combat_target_merge_takes_newer_stamps_with_their_locations_and_sums_counts() {
        let mut e = engine();
        let mine = e.new_object::<CombatTarget>();
        let theirs = e.new_object::<CombatTarget>();
        e.set(mine, CombatTarget::iDetectionLevel, 2);
        e.set(theirs, CombatTarget::iDetectionLevel, 5);
        // Last detected: theirs is newer (copied with its location).
        e.set(mine, CombatTarget::fLastDetectedTimeStamp, 1.0);
        e.set(theirs, CombatTarget::fLastDetectedTimeStamp, 2.0);
        e.mem.set_u32(theirs.addr() + 0x18, 0xd1);
        // Last seen: mine is newer (kept).
        e.set(mine, CombatTarget::fLastSeenTimeStamp, 9.0);
        e.set(theirs, CombatTarget::fLastSeenTimeStamp, 3.0);
        e.mem.set_u32(theirs.addr() + 0x28, 0xd2);
        // Detection level update: equal (kept).
        e.set(mine, CombatTarget::fDetectionLevelUpdateTimeStamp, 4.0);
        e.set(theirs, CombatTarget::fDetectionLevelUpdateTimeStamp, 4.0);
        // Last noticed: theirs newer (no location).
        e.set(mine, CombatTarget::fLastNoticedTimeStamp, 1.0);
        e.set(theirs, CombatTarget::fLastNoticedTimeStamp, 6.0);
        // Last attacked member: theirs newer (with location).
        e.set(mine, CombatTarget::fLastAttackedMemberTimeStamp, 1.0);
        e.set(theirs, CombatTarget::fLastAttackedMemberTimeStamp, 8.0);
        e.mem.set_u32(theirs.addr() + 0x38, 0xd3);
        e.set(mine, CombatTarget::sAttackerCount, 1);
        e.set(theirs, CombatTarget::sAttackerCount, 2);
        e.set(mine, CombatTarget::cMemberLOSCount, 250);
        e.set(theirs, CombatTarget::cMemberLOSCount, 10);
        e.set(mine, CombatTarget::cMember360LOSCount, 1);
        e.set(theirs, CombatTarget::cMember360LOSCount, 1);
        fn_00985520(&mut e, mine, theirs);
        assert_eq!(e.get(mine, CombatTarget::iDetectionLevel), 5);
        assert_eq!(e.get(mine, CombatTarget::fLastDetectedTimeStamp), 2.0);
        assert_eq!(e.mem.u32(mine.addr() + 0x18), 0xd1);
        assert_eq!(e.get(mine, CombatTarget::fLastSeenTimeStamp), 9.0);
        assert_eq!(e.mem.u32(mine.addr() + 0x28), 0);
        assert_eq!(e.get(mine, CombatTarget::fLastNoticedTimeStamp), 6.0);
        assert_eq!(e.get(mine, CombatTarget::fLastAttackedMemberTimeStamp), 8.0);
        assert_eq!(e.mem.u32(mine.addr() + 0x38), 0xd3);
        assert_eq!(e.get(mine, CombatTarget::sAttackerCount), 3);
        assert_eq!(e.get(mine, CombatTarget::cMemberLOSCount), 4); // wraps
        assert_eq!(e.get(mine, CombatTarget::cMember360LOSCount), 2);
        // A lower detection level on the other side is not taken.
        e.set(theirs, CombatTarget::iDetectionLevel, 1);
        fn_00985520(&mut e, mine, theirs);
        assert_eq!(e.get(mine, CombatTarget::iDetectionLevel), 5);
    }

    #[test]
    fn combat_target_save_writes_fields_in_order() {
        let mut e = engine();
        record(
            &mut e,
            &[
                SAVE_FORM_ID,
                SAVE_BYTES,
                WORLD_LOCATION_SAVE,
                TIME_STAMP_SAVE,
            ],
        );
        let target = e.new_object::<CombatTarget>();
        let who = actor(&mut e, 7);
        e.set(target, CombatTarget::pActor, who);
        let buffer = Ptr::new(0x6000_0000);
        combat_target_save_game(&mut e, target, buffer, 0);
        let t = target.addr();
        let b = buffer.addr();
        assert_eq!(calls(&e, SAVE_FORM_ID), vec![vec![b, who.addr(), 0]]);
        assert_eq!(
            calls(&e, SAVE_BYTES),
            vec![
                vec![b, t + 4, 4, 0],
                vec![b, t + 0x48, 2, 0],
                vec![b, t + 0x4a, 2, 0],
                vec![b, t + 0x64, 1, 0],
                vec![b, t + 0x65, 1, 0],
            ]
        );
        assert_eq!(
            calls(&e, WORLD_LOCATION_SAVE),
            vec![
                vec![t + 8, b],
                vec![t + 0x18, b],
                vec![t + 0x28, b],
                vec![t + 0x38, b]
            ]
        );
        let stamps: Vec<u32> = calls(&e, TIME_STAMP_SAVE)
            .iter()
            .map(|c| c[0] - t)
            .collect();
        assert_eq!(stamps, vec![0x50, 0x54, 0x58, 0x4c, 0x5c, 0x60]);
    }

    /// A save buffer whose version (virtual method at slot 0) is `version`.
    fn buffer_with_version(e: &mut Engine, version: u32) -> Ptr {
        let vtable = 0x7200_0000;
        e.put_vtable(vtable, &[0x7200_0100]);
        e.register(0x7200_0100, |e, a| e.mem.u32(a[0] + 4).into_ret());
        let buffer = e.mem.alloc(8);
        e.mem.set_u32(buffer, vtable);
        e.mem.set_u32(buffer + 4, version);
        Ptr::new(buffer)
    }

    #[test]
    fn combat_target_load_reads_newer_fields_only_in_newer_saves() {
        for (version, locations, stamps) in [(12u32, 3usize, 5usize), (13, 3, 6), (14, 4, 6)] {
            let mut e = engine();
            record(
                &mut e,
                &[
                    LOAD_FORM_ID,
                    LOAD_BYTES,
                    WORLD_LOCATION_LOAD,
                    TIME_STAMP_LOAD,
                ],
            );
            let buffer = buffer_with_version(&mut e, version);
            let target = e.new_object::<CombatTarget>();
            combat_target_load_game(&mut e, target, buffer, 0);
            let t = target.addr();
            assert_eq!(calls(&e, LOAD_FORM_ID), vec![vec![buffer.addr(), t]]);
            assert_eq!(calls(&e, LOAD_BYTES).len(), 5);
            assert_eq!(calls(&e, LOAD_BYTES)[1], vec![buffer.addr(), t + 0x48, 2]);
            assert_eq!(calls(&e, WORLD_LOCATION_LOAD).len(), locations, "{version}");
            assert_eq!(calls(&e, TIME_STAMP_LOAD).len(), stamps, "{version}");
            if version >= 14 {
                assert_eq!(
                    calls(&e, WORLD_LOCATION_LOAD)[3],
                    vec![t + 0x38, buffer.addr()]
                );
            }
            if version >= 13 {
                assert_eq!(calls(&e, TIME_STAMP_LOAD)[5][0], t + 0x60);
            }
        }
    }

    #[test]
    fn fn_00985930_resolves_a_form_into_an_actor_or_keeps_null() {
        let mut e = engine();
        record(&mut e, &[]);
        e.register(0x0048_39c0, |_, a| (a[0] + 1).into_ret());
        e.register(0x00ec_43fb, |_, a| (a[0] + 2).into_ret());
        let slot = e.mem.alloc(4);
        fn_00985930(&mut e, Ptr::new(slot), 0, 0);
        assert_eq!(e.mem.u32(slot), 0);
        assert!(calls(&e, 0x0048_39c0).is_empty());
        e.mem.set_u32(slot, 0x100);
        fn_00985930(&mut e, Ptr::new(slot), 0, 0);
        assert_eq!(e.mem.u32(slot), 0x103);
        assert_eq!(calls(&e, 0x0048_39c0), vec![vec![0x100]]);
        assert_eq!(
            calls(&e, 0x00ec_43fb),
            vec![vec![0x101, 0, 0x0118_3028, 0x0118_46d4, 0]]
        );
    }

    #[test]
    fn fn_00985980_copies_strengths_and_cluster() {
        let mut e = engine();
        let from = e.new_object::<CombatMember>();
        let to = e.new_object::<CombatMember>();
        e.set(from, CombatMember::pActor, Ptr::new(0x1111));
        e.set(from, CombatMember::fDamagePerSecond, 2.5);
        e.set(from, CombatMember::fCombatStrength, 7.0);
        e.set(from, CombatMember::pCluster, Ptr::new(0x2222));
        e.set(to, CombatMember::pActor, Ptr::new(0x3333));
        fn_00985980(&mut e, to, from);
        assert_eq!(e.get(to, CombatMember::fDamagePerSecond), 2.5);
        assert_eq!(e.get(to, CombatMember::fCombatStrength), 7.0);
        assert_eq!(e.get(to, CombatMember::pCluster), Ptr::new(0x2222));
        assert_eq!(e.get(to, CombatMember::pActor), Ptr::new(0x3333));
    }

    #[test]
    fn combat_member_save_writes_the_cluster_index_or_ff() {
        let mut e = engine();
        record(&mut e, &[SAVE_FORM_ID]);
        // The raw writer remembers the last single byte it was asked to
        // write (in a spare global).
        e.register(SAVE_BYTES, |e, a| {
            if a[2] == 1 {
                let byte = e.mem.u8(a[1]);
                e.set_global(0x011f_1000, byte);
            }
            Ret::default()
        });
        e.register(0x0098_de30, |_, a| {
            assert_eq!(a[1], 0xc1c1);
            3u32.into_ret()
        });
        let member = e.new_object::<CombatMember>();
        let who = actor(&mut e, 1);
        e.set(member, CombatMember::pActor, who);
        let group = e.new_object::<CombatGroup>();
        let buffer = Ptr::new(0x6000_0000);
        e.call_log = Some(vec![]);
        combat_member_save_game(&mut e, member, buffer, group);
        assert_eq!(e.global::<u8>(0x011f_1000), 0xff);
        assert_eq!(
            calls(&e, SAVE_FORM_ID),
            vec![vec![buffer.addr(), who.addr(), 0]]
        );
        e.set(member, CombatMember::pCluster, Ptr::new(0xc1c1));
        combat_member_save_game(&mut e, member, buffer, group);
        assert_eq!(e.global::<u8>(0x011f_1000), 3);
        let m = member.addr();
        let words: Vec<Vec<u32>> = calls(&e, SAVE_BYTES)
            .into_iter()
            .filter(|c| c[2] == 4)
            .take(3)
            .collect();
        assert_eq!(
            words,
            vec![
                vec![buffer.addr(), m + 4, 4, 0],
                vec![buffer.addr(), m + 8, 4, 0],
                vec![buffer.addr(), m + 0xc, 4, 0],
            ]
        );
    }

    #[test]
    fn combat_member_load_turns_the_index_back_into_a_cluster() {
        let mut e = engine();
        record(&mut e, &[LOAD_FORM_ID]);
        e.register(LOAD_BYTES, |e, a| {
            if a[2] == 1 {
                let value = e.global::<u8>(0x011f_1000);
                e.mem.set_u8(a[1], value);
            }
            Ret::default()
        });
        e.register(0x0098_de60, |_, a| (0xc000 + (a[1] & 0xff)).into_ret());
        let member = e.new_object::<CombatMember>();
        let group = e.new_object::<CombatGroup>();
        e.set(member, CombatMember::pCluster, Ptr::new(0x1234));
        e.set_global(0x011f_1000, 0xffu8);
        combat_member_load_game(&mut e, member, Ptr::new(0x6000_0000), group);
        assert!(e.get(member, CombatMember::pCluster).is_null());
        e.set_global(0x011f_1000, 2u8);
        combat_member_load_game(&mut e, member, Ptr::new(0x6000_0000), group);
        assert_eq!(e.get(member, CombatMember::pCluster), Ptr::new(0xc002));
        assert_eq!(calls(&e, LOAD_FORM_ID)[0], vec![0x6000_0000, member.addr()]);
    }

    #[test]
    fn cluster_save_and_load_use_sixteen_plus_four_bytes() {
        let mut e = engine();
        record(&mut e, &[SAVE_BYTES, LOAD_BYTES]);
        let cluster = Ptr::new(0x5000_0000);
        let buffer = Ptr::new(0x6000_0000);
        fn_00985ae0(&mut e, cluster, buffer, 0);
        fn_00985b10(&mut e, cluster, buffer, 0);
        assert_eq!(
            calls(&e, SAVE_BYTES),
            vec![
                vec![0x6000_0000, 0x5000_0000, 0x10, 0],
                vec![0x6000_0000, 0x5000_0010, 4, 0]
            ]
        );
        assert_eq!(
            calls(&e, LOAD_BYTES),
            vec![
                vec![0x6000_0000, 0x5000_0000, 0x10],
                vec![0x6000_0000, 0x5000_0010, 4]
            ]
        );
    }

    #[test]
    fn search_location_time_left_score_and_colour() {
        let mut e = engine();
        e.set_global(0x0108_d5a0, 16777216.0f32);
        e.set_global(0x0108_d598, 360000.0f64);
        e.set_global(0x0102_0758, 10.0f64);
        e.set_global(0x0101_6248, 0.5f32);
        e.set_global(0x0101_e2c0, 50.0f64);
        e.set_global(0x0101_db88, 30.0f64);
        e.register(DISTANCE_SQUARED, |e, a| {
            (e.mem.f32(a[0]) - e.mem.f32(a[1])).powi(2).into_ret()
        });
        e.register(0x0041_4430, |e, a| {
            for word in 0..4 {
                e.mem.set_u32(a[0] + 4 * word, a[1 + word as usize]);
            }
            a[0].into_ret()
        });
        let entry = e.new_object::<CombatSearchLocation>();
        // Stamped 20 seconds ago with 60 seconds of duration: 40 left.
        e.set(entry, CombatSearchLocation::fTimeStamp, NOW - 20.0);
        e.set(entry, CombatSearchLocation::fScore, 60.0);
        assert_eq!(fn_00985bb0(&mut e, entry), 40.0);

        let from = e.mem.alloc(16);
        e.mem.set_f32(from, 600.0); // the entry's point is at 0: 360000 away
        assert_eq!(fn_00985b40(&mut e, entry, Ptr::new(from)), 39.0);
        e.set(entry, CombatSearchLocation::iTargetID, 5);
        assert_eq!(fn_00985b40(&mut e, entry, Ptr::new(from)), 49.0);

        let colour = e.mem.alloc(16);
        for (score, want) in [
            (80.0f32, [1.0f32, 0.0, 1.0, 0.5]),
            (60.0, [0.0, 0.0, 1.0, 0.5]),
            (45.0, [1.0, 1.0, 0.0, 0.5]),
        ] {
            // 20 seconds old: 60, 40 and 25 seconds left.
            e.set(entry, CombatSearchLocation::fScore, score);
            let out = fn_00985be0(&mut e, entry, Ptr::new(colour));
            assert_eq!(out.addr(), colour);
            let got: Vec<f32> = (0..4).map(|i| e.mem.f32(colour + 4 * i)).collect();
            assert_eq!(got, want, "{score}");
        }
    }

    /// The combat manager singleton of these tests.
    const MANAGER: u32 = 0x7300_0000;

    fn with_manager(e: &mut Engine) {
        e.set_global(COMBAT_MANAGER, MANAGER);
    }

    #[test]
    fn group_constructor_sets_every_field_and_constructs_the_parts() {
        let mut e = engine();
        let timers = [0x0090_6f60u32];
        record(
            &mut e,
            &[
                0x0098_ed30,
                0x0098_eff0,
                timers[0],
                0x0040_1050,
                0x0063_3c90,
                WORLD_LOCATION_CONSTRUCT,
                0x0098_f2a0,
                0x0098_f4c0,
                0x0098_f510,
            ],
        );
        for word in 0..3 {
            e.set_global(0x011f_426c + 4 * word, 0x4000_0000 + word);
        }
        let group = e.new_object::<CombatGroup>();
        e.mem.write(group.addr(), &[0xaa; 0x15c]);
        let result = fn_00985d10(&mut e, group, 12);
        assert_eq!(result, group);
        assert_eq!(e.get(group, CombatGroup::iGroupNum), 0);
        assert_eq!(e.get(group, CombatGroup::iGroupID), 12);
        assert!(e.get(group, CombatGroup::pGroupStrategy).is_null());
        assert!(!e.get(group, CombatGroup::bStrategyForced));
        assert_eq!(e.get(group, CombatGroup::iLastGroupStrategyChosenIndex), 0);
        assert_eq!(
            e.get(group, CombatGroup::fLastGroupStrategyChosenTimeStamp),
            0.0
        );
        assert_eq!(e.get(group, CombatGroup::fMemberCombatStrength), -1.0);
        assert_eq!(
            e.get(group, CombatGroup::fAverageMemberDamagePerSecond),
            -1.0
        );
        assert_eq!(e.get(group, CombatGroup::fTargetCombatStrength), -1.0);
        assert_eq!(
            e.get(group, CombatGroup::fAverageTargetCombatStrength),
            -1.0
        );
        assert_eq!(e.get(group, CombatGroup::iSearchCount), 0);
        assert_eq!(
            e.get(group, CombatGroup::fSearchStartedTimeStamp),
            -f32::MAX
        );
        assert!(e.get(group, CombatGroup::pSearchingMember).is_null());
        for word in 0..3 {
            assert_eq!(
                e.mem.u32(group.addr() + 0x100 + 4 * word),
                0x4000_0000 + word
            );
        }
        assert_eq!(e.get(group, CombatGroup::fSearchRadius), 0.0);
        assert!(!e.get(group, CombatGroup::bUpdateSearchDebugGeometry));
        assert_eq!(e.get(group, CombatGroup::iInitializedMemberCount), 0);
        assert_eq!(e.get(group, CombatGroup::iFleeingMemberCount), 0);
        assert_eq!(e.get(group, CombatGroup::iNonFleeingMemberCount), 0);
        assert_eq!(e.get(group, CombatGroup::cCombatMusicState), -1);

        let g = group.addr();
        let timer_offsets: Vec<u32> = calls(&e, 0x0090_6f60).iter().map(|c| c[0] - g).collect();
        assert_eq!(
            timer_offsets,
            vec![0x30, 0x38, 0x48, 0x50, 0xa8, 0xb0, 0xb8, 0xd8, 0xe0]
        );
        assert_eq!(
            calls(&e, 0x0040_1050),
            vec![vec![g + 0x58, 8, 10, 0x0090_6f60]]
        );
        assert_eq!(calls(&e, 0x0098_ed30), vec![vec![g + 0x08]]);
        assert_eq!(calls(&e, 0x0098_eff0), vec![vec![g + 0x18]]);
        assert_eq!(calls(&e, 0x0098_f2a0), vec![vec![g + 0x110]]);
        assert_eq!(calls(&e, 0x0098_f4c0), vec![vec![g + 0x120]]);
        assert_eq!(calls(&e, 0x0098_f510), vec![vec![g + 0x138]]);
        assert_eq!(
            calls(&e, 0x0063_3c90),
            vec![vec![g + 0xd4, 0], vec![g + 0x130, 0], vec![g + 0x158, 0]]
        );
        assert_eq!(calls(&e, WORLD_LOCATION_CONSTRUCT), vec![vec![g + 0xf0]]);
    }

    #[test]
    fn group_destructor_leaves_the_manager_then_destroys_parts_in_reverse() {
        let mut e = engine();
        with_manager(&mut e);
        let t0 = actor(&mut e, 1);
        let t1 = actor(&mut e, 2);
        let m0 = actor(&mut e, 3);
        let group = group_with(&mut e, &[t0, t1], &[m0]);
        // Two clusters in the cluster array (buffer +0x13c, size +0x140).
        let clusters = e.mem.alloc(8);
        e.mem.set_u32(clusters, 0xc100);
        e.mem.set_u32(clusters + 4, 0xc200);
        e.mem.set_u32(group.addr() + 0x13c, clusters);
        e.mem.set_u32(group.addr() + 0x140, 2);
        let parts = [
            MANAGER_REMOVE_TARGET,
            MANAGER_REMOVE_MEMBER,
            0x0040_1030,
            0x0098_78b0,
            0x0045_cec0,
            0x0098_f540,
            0x0098_f4f0,
            0x0098_f2d0,
            0x0098_f020,
            0x0098_ed60,
        ];
        record(&mut e, &parts);
        fn_00985f90(&mut e, group);
        let g = group.addr();
        assert_eq!(
            calls(&e, MANAGER_REMOVE_TARGET),
            vec![vec![MANAGER, g, t0.addr()], vec![MANAGER, g, t1.addr()]]
        );
        assert_eq!(
            calls(&e, MANAGER_REMOVE_MEMBER),
            vec![vec![MANAGER, g, m0.addr()]]
        );
        assert_eq!(calls(&e, 0x0040_1030), vec![vec![0xc100], vec![0xc200]]);
        let order: Vec<(u32, Vec<u32>)> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| {
                [
                    0x0098_78b0,
                    0x0045_cec0,
                    0x0098_f540,
                    0x0098_f4f0,
                    0x0098_f2d0,
                    0x0098_f020,
                    0x0098_ed60,
                ]
                .contains(a)
            })
            .cloned()
            .collect();
        assert_eq!(
            order,
            vec![
                (0x0098_78b0, vec![g, 1]),
                (0x0045_cec0, vec![g + 0x158]),
                (0x0098_f540, vec![g + 0x138]),
                (0x0045_cec0, vec![g + 0x130]),
                (0x0098_f4f0, vec![g + 0x120]),
                (0x0098_f2d0, vec![g + 0x110]),
                (0x0045_cec0, vec![g + 0xd4]),
                (0x0098_f020, vec![g + 0x18]),
                (0x0098_ed60, vec![g + 0x08]),
            ]
        );
    }

    /// Timers for the update tests: a timer whose first float is 1.0 has
    /// expired, one whose second float is not zero is running; starting one
    /// clears the first and stores the delay in the second.
    fn update_engine() -> Engine {
        let mut e = engine();
        e.register(TIMER_EXPIRED, |e, a| (e.mem.f32(a[0]) == 1.0).into_ret());
        e.register(TIMER_RUNNING, |e, a| {
            (e.mem.f32(a[0] + 4) != 0.0).into_ret()
        });
        e.register(TIMER_START, |e, a| {
            e.mem.set_f32(a[0], 0.0);
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        // `0097ef30`: whether a search was started (`iSearchCount`).
        e.register(0x0097_ef30, |e, a| (e.mem.u32(a[0] + 0xd0) != 0).into_ret());
        record(
            &mut e,
            &[
                0x0098_ce80,
                0x0098_cf70,
                0x0098_d030,
                0x0098_d1a0,
                0x0098_a580,
                0x0098_c870,
                0x0098_d2c0,
            ],
        );
        e.set_global(0x0101_62c0, 2.0f32);
        e.set_global(0x011c_fe84, 5.0f32);
        e.set_global(0x011c_eb44, 9.0f32);
        e.set_global(0x011c_ee6c, 0.5f32);
        e.set_global(0x011c_f944, 0.8f32);
        e
    }

    /// A group whose music state is still undecided (-1).
    fn music_group(e: &mut Engine) -> Ptr<CombatGroup> {
        let group = group_with(e, &[], &[]);
        e.set(group, CombatGroup::cCombatMusicState, -1);
        group
    }

    fn expire(e: &mut Engine, group: Ptr<CombatGroup>, timer: u32) {
        e.mem.set_f32(group.addr() + timer, 1.0);
    }

    #[test]
    fn update_with_nothing_due_only_refreshes() {
        let mut e = update_engine();
        let group = group_with(&mut e, &[], &[]);
        e.set(group, CombatGroup::cCombatMusicState, 0);
        combat_group_update(&mut e, group);
        assert_eq!(
            addresses(&e)
                .into_iter()
                .filter(|a| *a != TIMER_EXPIRED && *a != MEMBER_COUNT)
                .collect::<Vec<_>>(),
            vec![0x0098_ce80, 0x0098_d1a0, 0x0098_a580]
        );
    }

    #[test]
    fn update_restarts_the_expired_timers_and_runs_the_extras() {
        let mut e = update_engine();
        let m0 = actor(&mut e, 1);
        let m1 = actor(&mut e, 2);
        let group = group_with(&mut e, &[], &[m0, m1]);
        e.set(group, CombatGroup::cCombatMusicState, 0);
        expire(&mut e, group, CombatGroup::TARGET_UPDATE_TIMER);
        expire(&mut e, group, CombatGroup::COMBAT_STRENGTH_TIMER);
        combat_group_update(&mut e, group);
        let g = group.addr();
        assert_eq!(
            calls(&e, TIMER_START),
            vec![
                vec![g + 0xa8, 2.0f32.to_bits()],
                vec![g + 0x50, 5.0f32.to_bits()]
            ]
        );
        assert_eq!(calls(&e, 0x0098_cf70).len(), 1);
        assert_eq!(calls(&e, 0x0098_d030).len(), 1);
        assert_eq!(calls(&e, 0x0098_c870).len(), 1);
        assert_eq!(calls(&e, 0x0098_d2c0).len(), 1, "two members");
    }

    #[test]
    fn update_decides_the_music_state() {
        // Not running: armed with the setting, unless a search started.
        let mut e = update_engine();
        let group = music_group(&mut e);
        combat_group_update(&mut e, group);
        assert_eq!(
            calls(&e, TIMER_START),
            vec![vec![group.addr() + 0xb8, 9.0f32.to_bits()]]
        );
        assert_eq!(e.get(group, CombatGroup::cCombatMusicState), -1);
        let mut e = update_engine();
        let group = music_group(&mut e);
        e.set(group, CombatGroup::iSearchCount, 1);
        combat_group_update(&mut e, group);
        assert!(calls(&e, TIMER_START).is_empty());

        // Running and expired: the strength ratio picks 1 or 0.
        for (member, target, state) in [
            (6.0f32, 10.0f32, 1i8),
            (9.0, 10.0, 0),
            (4.0, 10.0, 0),
            (5.0, 10.0, 0),
            (5.0, 0.0, 0),
        ] {
            let mut e = update_engine();
            let group = music_group(&mut e);
            e.mem.set_f32(group.addr() + 0xb8, 1.0);
            e.mem.set_f32(group.addr() + 0xbc, 3.0);
            e.set(group, CombatGroup::fMemberCombatStrength, member);
            e.set(group, CombatGroup::fTargetCombatStrength, target);
            combat_group_update(&mut e, group);
            assert_eq!(
                e.get(group, CombatGroup::cCombatMusicState),
                state,
                "{member}/{target}"
            );
        }
        // Running but not expired: still undecided.
        let mut e = update_engine();
        let group = music_group(&mut e);
        e.mem.set_f32(group.addr() + 0xbc, 3.0);
        combat_group_update(&mut e, group);
        assert_eq!(e.get(group, CombatGroup::cCombatMusicState), -1);
    }

    #[test]
    fn set_strategy_clears_the_flag_for_null_and_runs_the_strategy_otherwise() {
        let mut e = engine();
        record(
            &mut e,
            &[0x0099_0320, 0x0098_a3c0, 0x0098_f560, 0x0072_6070],
        );
        let group = e.new_object::<CombatGroup>();
        e.set(group, CombatGroup::bStrategyForced, true);
        fn_009862e0(&mut e, group, Ptr::NULL);
        assert!(!e.get(group, CombatGroup::bStrategyForced));
        assert!(calls(&e, 0x0099_0320).is_empty());

        e.register(0x0072_6070, |_, _| 7u32.into_ret());
        let strategy = e.mem.alloc(8);
        e.put_vtable(0x7400_0000, &[0, 0x7400_0100]);
        e.mem.set_u32(strategy, 0x7400_0000);
        e.register(0x7400_0100, |_, _| Ret::default());
        fn_009862e0(&mut e, group, Ptr::new(strategy));
        let g = group.addr();
        let temporary = calls(&e, 0x0099_0320)[0][0];
        assert_eq!(calls(&e, 0x0099_0320), vec![vec![temporary, g]]);
        assert_eq!(calls(&e, 0x0098_a3c0), vec![vec![g]]);
        assert_eq!(calls(&e, 0x7400_0100), vec![vec![strategy, g, temporary]]);
        assert_eq!(calls(&e, 0x0072_6070), vec![vec![strategy]]);
        assert_eq!(e.get(group, CombatGroup::iLastGroupStrategyChosenIndex), 7);
        assert_eq!(
            e.get(group, CombatGroup::fLastGroupStrategyChosenTimeStamp),
            NOW
        );
        assert!(e.get(group, CombatGroup::bStrategyForced));
        assert_eq!(
            e.get(group, CombatGroup::pGroupStrategy),
            Ptr::new(strategy)
        );
        assert_eq!(
            calls(&e, 0x0098_f560),
            vec![vec![temporary + 0x28], vec![temporary + 0x18]]
        );
    }

    #[test]
    fn strategy_temporary_destructor_destroys_two_arrays() {
        let mut e = engine();
        record(&mut e, &[0x0098_f560]);
        fn_009863b0(&mut e, Ptr::new(0x5000_0000));
        assert_eq!(
            calls(&e, 0x0098_f560),
            vec![vec![0x5000_0028], vec![0x5000_0018]]
        );
    }

    /// Doubles that make the array adders and removers real: element
    /// `size` is copied in, resp. elements after `index` are shifted down.
    fn array_engine() -> Engine {
        let mut e = engine();
        with_manager(&mut e);
        e.register(TARGET_ARRAY_ADD, |e, a| {
            let (buffer, size) = (e.mem.u32(a[0] + 4), e.mem.u32(a[0] + 8));
            let bytes = e.mem.bytes(a[1], 0x68);
            e.mem.write(buffer + size * 0x68, &bytes);
            e.mem.set_u32(a[0] + 8, size + 1);
            size.into_ret()
        });
        e.register(MEMBER_ARRAY_ADD, |e, a| {
            let (buffer, size) = (e.mem.u32(a[0] + 4), e.mem.u32(a[0] + 8));
            let bytes = e.mem.bytes(a[1], 0x14);
            e.mem.write(buffer + size * 0x14, &bytes);
            e.mem.set_u32(a[0] + 8, size + 1);
            size.into_ret()
        });
        e.register(TARGET_ARRAY_REMOVE_AT, |e, a| {
            let (buffer, size) = (e.mem.u32(a[0] + 4), e.mem.u32(a[0] + 8));
            let tail = e
                .mem
                .bytes(buffer + (a[1] + 1) * 0x68, (size - a[1] - 1) * 0x68);
            e.mem.write(buffer + a[1] * 0x68, &tail);
            e.mem.set_u32(a[0] + 8, size - 1);
            Ret::default()
        });
        e.register(MEMBER_ARRAY_REMOVE_AT, |e, a| {
            let (buffer, size) = (e.mem.u32(a[0] + 4), e.mem.u32(a[0] + 8));
            let tail = e
                .mem
                .bytes(buffer + (a[1] + 1) * 0x14, (size - a[1] - 1) * 0x14);
            e.mem.write(buffer + a[1] * 0x14, &tail);
            e.mem.set_u32(a[0] + 8, size - 1);
            Ret::default()
        });
        // An actor is incompatible with everything when its byte at +0x3c
        // is set.
        e.register(ACTOR_COMBAT_COMPATIBLE, |e, a| {
            (e.mem.u8(a[0] + 0x3c) == 0).into_ret()
        });
        e.register(WORLD_LOCATION_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(GET_WORLD_LOCATION, |_, a| a[1].into_ret());
        e.register(0x0047_4cb0, |e, a| e.mem.u8(a[0] + 0x3d).into_ret());
        e.register(0x0055_d520, |_, _| 0xcafeu32.into_ret());
        e.register(MANAGER_ADD_TARGET, |_, _| Ret::default());
        e.register(MANAGER_REMOVE_TARGET, |_, _| Ret::default());
        e.register(MANAGER_ADD_MEMBER, |_, _| Ret::default());
        e.register(MANAGER_REMOVE_MEMBER, |_, _| Ret::default());
        e.register(0x0097_f220, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    fn array_len(e: &Engine, group: Ptr<CombatGroup>, array: u32) -> u32 {
        e.mem.u32(group.addr() + array + 8)
    }

    #[test]
    fn add_target_handles_each_refusal_and_appends_otherwise() {
        let mut e = array_engine();
        let existing = actor(&mut e, 1);
        let member = actor(&mut e, 2);
        let newcomer = actor(&mut e, 3);
        let group = group_with(&mut e, &[existing], &[member]);
        e.set(group, CombatGroup::iSearchCount, 4);
        assert!(!combat_group_add_target(&mut e, group, Ptr::NULL));
        assert!(combat_group_add_target(&mut e, group, existing));
        assert_eq!(array_len(&e, group, 8), 1);

        // A member of the group: refused with a log line.
        assert!(!combat_group_add_target(&mut e, group, member));
        assert_eq!(calls(&e, LOG_MESSAGE), vec![vec![0x0108_d5a8, 2, 0xcafe]]);
        e.mem.set_u8(member.addr() + 0x3d, 1);
        assert!(!combat_group_add_target(&mut e, group, member));
        assert_eq!(calls(&e, LOG_MESSAGE)[1], vec![0x0108_d5a8, 2, 0xbeef]);

        // A member that cannot fight the newcomer: refused.
        e.mem.set_u8(member.addr() + 0x3c, 1);
        assert!(!combat_group_add_target(&mut e, group, newcomer));
        assert_eq!(array_len(&e, group, 8), 1);
        e.mem.set_u8(member.addr() + 0x3c, 0);

        assert!(combat_group_add_target(&mut e, group, newcomer));
        assert_eq!(array_len(&e, group, 8), 2);
        let entry = target_entry(&e, group, 1);
        assert_eq!(e.get(entry, CombatTarget::pActor), newcomer);
        assert_eq!(e.get(entry, CombatTarget::sLastSearchNoticed), 4);
        assert_eq!(e.get(entry, CombatTarget::fLastNoticedTimeStamp), NOW);
        assert_eq!(
            calls(&e, MANAGER_ADD_TARGET),
            vec![vec![MANAGER, group.addr(), newcomer.addr()]]
        );
    }

    #[test]
    fn remove_target_removes_the_matching_entry() {
        let mut e = array_engine();
        let (a, b, c) = (actor(&mut e, 1), actor(&mut e, 2), actor(&mut e, 3));
        let group = group_with(&mut e, &[a, b, c], &[]);
        let stranger = actor(&mut e, 9);
        combat_group_remove_target(&mut e, group, stranger);
        assert_eq!(array_len(&e, group, 8), 3);
        combat_group_remove_target(&mut e, group, b);
        assert_eq!(array_len(&e, group, 8), 2);
        assert_eq!(e.get(target_entry(&e, group, 0), CombatTarget::pActor), a);
        assert_eq!(e.get(target_entry(&e, group, 1), CombatTarget::pActor), c);
        assert_eq!(
            calls(&e, MANAGER_REMOVE_TARGET),
            vec![vec![MANAGER, group.addr(), b.addr()]]
        );
        assert_eq!(
            calls(&e, TARGET_ARRAY_REMOVE_AT),
            vec![vec![group.addr() + 8, 1, 1]]
        );
    }

    #[test]
    fn find_helpers_match_by_form_id() {
        let mut e = array_engine();
        let (t, m) = (actor(&mut e, 10), actor(&mut e, 20));
        let group = group_with(&mut e, &[t], &[m]);
        let same_id_as_target = actor(&mut e, 10);
        let same_id_as_member = actor(&mut e, 20);
        let stranger = actor(&mut e, 30);
        let entry = target_entry(&e, group, 0);
        let member_entry: Ptr = Ptr::new(e.mem.u32(group.addr() + 0x1c));
        assert!(combat_group_is_target(&mut e, group, t));
        assert!(combat_group_is_target(&mut e, group, same_id_as_target));
        assert!(!combat_group_is_target(&mut e, group, m));
        assert!(!combat_group_is_target(&mut e, group, stranger));
        assert!(combat_group_is_member(&mut e, group, same_id_as_member));
        assert!(!combat_group_is_member(&mut e, group, t));
        assert_eq!(fn_009865d0(&mut e, group, t), entry);
        assert!(fn_009865d0(&mut e, group, stranger).is_null());
        assert_eq!(fn_009865f0(&mut e, group, m), member_entry.cast());
        assert!(fn_009865f0(&mut e, group, stranger).is_null());
        assert_eq!(fn_00986610(&mut e, group, 10), entry);
        assert!(fn_00986610(&mut e, group, 20).is_null());
        assert_eq!(fn_00986670(&mut e, group, 20), member_entry.cast());
        assert!(fn_00986670(&mut e, group, 10).is_null());
    }

    #[test]
    fn remove_target_at_tells_the_manager_then_removes() {
        let mut e = array_engine();
        let (a, b) = (actor(&mut e, 1), actor(&mut e, 2));
        let group = group_with(&mut e, &[a, b], &[]);
        fn_00986560(&mut e, group, 0);
        assert_eq!(array_len(&e, group, 8), 1);
        assert_eq!(e.get(target_entry(&e, group, 0), CombatTarget::pActor), b);
        let order = addresses(&e);
        let manager_at = order
            .iter()
            .position(|a| *a == MANAGER_REMOVE_TARGET)
            .unwrap();
        let remove_at = order
            .iter()
            .position(|a| *a == TARGET_ARRAY_REMOVE_AT)
            .unwrap();
        assert!(manager_at < remove_at);
        assert_eq!(
            calls(&e, MANAGER_REMOVE_TARGET),
            vec![vec![MANAGER, group.addr(), a.addr()]]
        );
    }

    #[test]
    fn can_add_target_asks_every_non_null_member() {
        let mut e = array_engine();
        let (m0, m1) = (actor(&mut e, 1), actor(&mut e, 2));
        let newcomer = actor(&mut e, 3);
        let group = group_with(&mut e, &[], &[m0, Ptr::NULL, m1]);
        assert!(combat_group_can_add_target(&mut e, group, newcomer));
        assert_eq!(
            calls(&e, ACTOR_COMBAT_COMPATIBLE),
            vec![
                vec![m0.addr(), newcomer.addr()],
                vec![m1.addr(), newcomer.addr()]
            ]
        );
        e.mem.set_u8(m1.addr() + 0x3c, 1);
        assert!(!combat_group_can_add_target(&mut e, group, newcomer));
        let empty = group_with(&mut e, &[], &[]);
        assert!(combat_group_can_add_target(&mut e, empty, newcomer));
    }

    #[test]
    fn can_add_member_asks_the_candidate_about_every_target() {
        let mut e = array_engine();
        let (t0, t1) = (actor(&mut e, 1), actor(&mut e, 2));
        let candidate = actor(&mut e, 3);
        let group = group_with(&mut e, &[t0, t1], &[]);
        assert!(combat_group_can_add_member(&mut e, group, candidate));
        assert_eq!(
            calls(&e, ACTOR_COMBAT_COMPATIBLE),
            vec![
                vec![candidate.addr(), t0.addr()],
                vec![candidate.addr(), t1.addr()]
            ]
        );
        e.mem.set_u8(candidate.addr() + 0x3c, 1);
        assert!(!combat_group_can_add_member(&mut e, group, candidate));
        let empty = group_with(&mut e, &[], &[]);
        assert!(combat_group_can_add_member(&mut e, empty, candidate));
    }

    #[test]
    fn target_array_assignment_and_pointer_lists() {
        let mut e = array_engine();
        let (a, b) = (actor(&mut e, 1), actor(&mut e, 2));
        let group = group_with(&mut e, &[a, b], &[b]);
        let other = group_with(&mut e, &[], &[]);
        record(&mut e, &[0x0098_f580, POINTER_ARRAY_SET_SIZE, ARRAY_SET_AT]);
        fn_00986740(&mut e, group, other);
        assert_eq!(
            calls(&e, 0x0098_f580),
            vec![vec![other.addr() + 8, group.addr() + 8]]
        );
        let list = 0x5000_0000;
        fn_00986760(&mut e, group, Ptr::new(list));
        assert_eq!(calls(&e, POINTER_ARRAY_SET_SIZE), vec![vec![list, 2, 1]]);
        let first = target_entry(&e, group, 0).addr();
        assert_eq!(
            calls(&e, ARRAY_SET_AT),
            vec![vec![list, 0, first], vec![list, 1, first + 0x68]]
        );
        fn_00986b00(&mut e, group, Ptr::new(list));
        assert_eq!(calls(&e, POINTER_ARRAY_SET_SIZE)[1], vec![list, 1, 1]);
        let member = e.mem.u32(group.addr() + 0x1c);
        assert_eq!(calls(&e, ARRAY_SET_AT)[2], vec![list, 0, member]);
    }

    #[test]
    fn member_constructor_defaults() {
        let mut e = engine();
        let member = e.new_object::<CombatMember>();
        e.mem.write(member.addr(), &[0xaa; 0x14]);
        let who = actor(&mut e, 1);
        assert_eq!(fn_00986950(&mut e, member, who), member);
        assert_eq!(e.get(member, CombatMember::pActor), who);
        assert_eq!(e.get(member, CombatMember::iGroupStrategyAssignment), 0);
        assert_eq!(e.get(member, CombatMember::fDamagePerSecond), -1.0);
        assert_eq!(e.get(member, CombatMember::fCombatStrength), -1.0);
        assert!(e.get(member, CombatMember::pCluster).is_null());
    }

    #[test]
    fn add_member_refusals_and_tinting() {
        let mut e = array_engine();
        let target = actor(&mut e, 1);
        let existing = actor(&mut e, 2);
        let newcomer = actor(&mut e, 3);
        let group = group_with(&mut e, &[target], &[existing]);
        e.set(group, CombatGroup::iGroupNum, 6);
        assert!(!combat_group_add_member(&mut e, group, Ptr::NULL));
        assert!(combat_group_add_member(&mut e, group, existing));
        // A target of the group: refused with a log line.
        assert!(!combat_group_add_member(&mut e, group, target));
        assert_eq!(calls(&e, LOG_MESSAGE), vec![vec![0x0108_d600, 1, 0xcafe]]);
        // Incompatible with a target: refused.
        e.mem.set_u8(newcomer.addr() + 0x3c, 1);
        assert!(!combat_group_add_member(&mut e, group, newcomer));
        e.mem.set_u8(newcomer.addr() + 0x3c, 0);

        // Appended; the tint setting is off.
        record(&mut e, &[TINT_SCENEGRAPH]);
        assert!(combat_group_add_member(&mut e, group, newcomer));
        assert_eq!(array_len(&e, group, 0x18), 2);
        let entry = Ptr::new(e.mem.u32(group.addr() + 0x1c) + 0x14);
        assert_eq!(e.get(entry, CombatMember::pActor), newcomer);
        assert_eq!(e.get(entry, CombatMember::fCombatStrength), -1.0);
        assert!(calls(&e, TINT_SCENEGRAPH).is_empty());
        assert_eq!(
            calls(&e, MANAGER_ADD_MEMBER),
            vec![vec![MANAGER, group.addr(), newcomer.addr()]]
        );
        assert_eq!(calls(&e, 0x0097_f220), vec![vec![group.addr() + 0xb0]]);

        // With the tint setting on and a 3D root: tinted with the group's
        // colour.
        e.mem.set_u8(TINT_SETTING, 1);
        let fourth = actor(&mut e, 4);
        e.mem.set_u32(fourth.addr() + 0x30, 0x9999);
        let colour = e.mem.alloc(16);
        for (i, v) in [0.25f32, 0.5, 0.75, 1.0].iter().enumerate() {
            e.mem.set_f32(colour + 4 * i as u32, *v);
        }
        e.set_global(0x011f_1b00, colour);
        e.register(0x005d_8550, |e, a| {
            assert_eq!(a[0], 6);
            e.global::<u32>(0x011f_1b00).into_ret()
        });
        e.register(0x0041_6870, |e, a| {
            for i in 0..3 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            a[0].into_ret()
        });
        e.register_double(TINT_SCENEGRAPH, |e, a| {
            assert_eq!(a[0], 0x9999);
            assert_eq!(a[2], 0);
            let rgb: Vec<f32> = (0..3).map(|i| e.mem.f32(a[1] + 4 * i)).collect();
            assert_eq!(rgb, vec![0.25, 0.5, 0.75]);
            e.set_global(0x011f_1b04, 1u32);
            Ret::default()
        });
        assert!(combat_group_add_member(&mut e, group, fourth));
        assert_eq!(e.global::<u32>(0x011f_1b04), 1);
        // No 3D root: not tinted.
        let fifth = actor(&mut e, 5);
        e.set_global(0x011f_1b04, 0u32);
        assert!(combat_group_add_member(&mut e, group, fifth));
        assert_eq!(e.global::<u32>(0x011f_1b04), 0);
    }

    #[test]
    fn remove_member_clears_search_state_and_the_tint() {
        let mut e = array_engine();
        let (a, b) = (actor(&mut e, 1), actor(&mut e, 2));
        let group = group_with(&mut e, &[], &[a, b]);
        e.set(group, CombatGroup::pSearchingMember, a);
        e.mem.set_u8(TINT_SETTING, 1);
        e.mem.set_u32(a.addr() + 0x30, 0x7777);
        record(&mut e, &[UNTINT_SCENEGRAPH]);
        // Not a member: nothing happens.
        let stranger = actor(&mut e, 9);
        fn_009869a0(&mut e, group, stranger);
        assert!(calls(&e, MANAGER_REMOVE_MEMBER).is_empty());
        fn_009869a0(&mut e, group, a);
        assert_eq!(array_len(&e, group, 0x18), 1);
        assert_eq!(Ptr::new(e.mem.u32(e.mem.u32(group.addr() + 0x1c))), b);
        assert!(e.get(group, CombatGroup::pSearchingMember).is_null());
        assert_eq!(
            calls(&e, MANAGER_REMOVE_MEMBER),
            vec![vec![MANAGER, group.addr(), a.addr()]]
        );
        assert_eq!(calls(&e, UNTINT_SCENEGRAPH), vec![vec![0x7777]]);
        // The other member without a 3D root, search member untouched.
        e.set(group, CombatGroup::pSearchingMember, Ptr::new(0x1234));
        fn_009869a0(&mut e, group, b);
        assert_eq!(calls(&e, UNTINT_SCENEGRAPH).len(), 1);
        assert_eq!(
            e.get(group, CombatGroup::pSearchingMember),
            Ptr::new(0x1234)
        );
    }

    #[test]
    fn member_loops_call_the_actors_and_the_manager() {
        let mut e = array_engine();
        let (a, b) = (actor(&mut e, 1), actor(&mut e, 2));
        let group = group_with(&mut e, &[], &[a, b]);
        record(&mut e, &[0x0084_54f0]);
        fn_00986b70(&mut e, group);
        assert_eq!(
            calls(&e, fake(0x3fc)),
            vec![vec![a.addr(), 0], vec![b.addr(), 0]]
        );
        fn_00986bd0(&mut e, group);
        assert_eq!(
            calls(&e, MANAGER_REMOVE_MEMBER),
            vec![
                vec![MANAGER, group.addr(), a.addr()],
                vec![MANAGER, group.addr(), b.addr()]
            ]
        );
        assert_eq!(calls(&e, 0x0084_54f0), vec![vec![group.addr() + 0x18, 0]]);
    }

    #[test]
    fn has_no_members_asks_the_member_array() {
        let mut e = engine();
        e.register(0x0076_b610, |e, a| (e.mem.u32(a[0] + 8) == 0).into_ret());
        let a = actor(&mut e, 1);
        let with = group_with(&mut e, &[], &[a]);
        let without = group_with(&mut e, &[], &[]);
        assert!(!fn_00986c40(&mut e, with));
        assert!(fn_00986c40(&mut e, without));
    }

    /// The scene of the target-choice tests. Fake fields (see `engine`):
    /// actors: +0x14 x position, +0x10 space, +0x18 cell, +0x1c melee,
    /// +0x1d force-check flag, +0x1e should-attack, +0x1f fleeing, +0x24 /
    /// +0x28 / +0x2c / +0x2d the level, stamp and two flags the detection
    /// call answers with, +0x40 byte: inside the firing arc. Process: +0
    /// current target, +0x100 byte answering `0097ee90`, +0x200 view angle.
    struct Scene {
        e: Engine,
        group: Ptr<CombatGroup>,
        actor: Ptr,
        targets: Vec<Ptr>,
        process: u32,
    }

    fn scene(target_count: usize) -> Scene {
        let mut e = engine();
        with_manager(&mut e);
        let me = actor(&mut e, 100);
        let targets: Vec<Ptr> = (0..target_count)
            .map(|i| {
                let target = actor(&mut e, 200 + i as u32);
                e.mem.set_u8(target.addr() + 0x1e, 1); // should attack
                e.mem.set_u8(target.addr() + 0x40, 1); // inside the arc
                e.mem.set_i32(target.addr() + 0x24, 1); // detected
                target
            })
            .collect();
        let group = group_with(&mut e, &targets, &[me]);
        let process = e.mem.alloc(0x300);
        e.mem.set_u32(me.addr() + 0x38, process);

        e.register(0x0076_b610, |e, a| (e.mem.u32(a[0] + 8) == 0).into_ret());
        e.register(GET_WORLD_LOCATION, |e, a| {
            let (x, space) = (e.mem.u32(a[0] + 0x14), e.mem.u32(a[0] + 0x10));
            e.mem.set_u32(a[1], x);
            e.mem.set_u32(a[1] + 0x0c, space);
            a[1].into_ret()
        });
        e.register(0x0040_30b0, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(0x0058_6150, |_, a| (a[0] + 0x100).into_ret());
        e.register(0x0098_0070, |_, a| (a[0] + 0x200).into_ret());
        e.register(0x0063_9aa0, |e, a| e.mem.f32(a[0]).into_ret());
        e.register(0x0056_7d90, |e, a| e.mem.u32(a[0] + 0x18).into_ret());
        e.register(0x009a_9630, |e, a| e.mem.u8(a[0] + 0x1c).into_ret());
        e.register(0x0098_71c0, |e, a| {
            (e.mem.u32(a[0] + 0x0c) + a[1] * 0x68).into_ret()
        });
        e.register(0x009a_6d70, |e, a| (e.mem.u8(a[1]) != 0).into_ret());
        e.register(0x0056_6950, |e, a| e.mem.u8(a[0] + 0x1d).into_ret());
        e.register(ACTOR_GET_SHOULD_ATTACK, |e, a| {
            e.mem.u8(a[1] + 0x1e).into_ret()
        });
        e.register(0x0098_87b0, |e, a| {
            let target = a[2];
            let level = e.mem.i32(target + 0x24);
            e.mem.set_i32(a[3], level);
            let stamp = e.mem.u32(target + 0x28);
            e.mem.set_u32(a[4], stamp);
            let (first, second) = (e.mem.u8(target + 0x2c), e.mem.u8(target + 0x2d));
            e.mem.set_u8(a[5], first);
            e.mem.set_u8(a[6], second);
            Ret::default()
        });
        e.register(0x0097_ee90, |e, a| e.mem.u8(a[0]).into_ret());
        e.register(0x0098_71e0, |_, _| Ret::default());
        e.register(0x009a_0f40, |_, _| Ret::default());
        e.register(0x009a_96f0, |_, _| 1u32.into_ret());
        e.register(DISTANCE_SQUARED, |e, a| {
            (e.mem.f32(a[1]) - e.mem.f32(a[0])).powi(2).into_ret()
        });
        e.register(0x0097_fb00, |e, a| {
            e.mem.set_f32(a[1], NOW - 1.0);
            a[1].into_ret()
        });
        e.register(ACTOR_IS_FLEEING, |e, a| e.mem.u8(a[0] + 0x1f).into_ret());
        e.register(MANAGER_REMOVE_MEMBER, |_, _| Ret::default());
        e.register(MEMBER_ARRAY_REMOVE_AT, |e, a| {
            e.mem.set_u32(a[0] + 8, 0);
            Ret::default()
        });
        for (address, value) in [
            (0x011a_4d78u32, 100.0f32),
            (0x011a_4d74, 10.0),
            (0x011a_4d70, 1000.0),
            (0x011a_4d6c, 5.0),
            (0x011a_4d68, 1.0),
            (0x011a_4d64, 10.0),
            (0x011a_4d60, 50.0),
            (0x011a_4d5c, 7.0),
            (0x011a_4d58, 5.0),
            (0x011a_4d54, 3.0),
            (0x011a_4d50, 2.0),
            (0x011f_18b0, 0.5),
            (0x0108_d660, 4194304.0),
            (0x0101_ff50, std::f32::consts::TAU),
            (0x0103_2adc, 256.0),
        ] {
            e.set_global(address, value);
        }
        e.set_global(0x0108_d658, 4194304.0f64);
        e.set_global(0x0102_3128, std::f64::consts::PI / 180.0);
        e.set_global(0x0101_db88, 30.0f64);
        e.call_log = Some(vec![]);
        Scene {
            e,
            group,
            actor: me,
            targets,
            process,
        }
    }

    fn choose(s: &mut Scene) -> Ptr {
        fn_00986c60(&mut s.e, s.group, s.actor)
    }

    #[test]
    fn choose_target_without_targets_or_process() {
        let mut s = scene(0);
        assert!(choose(&mut s).is_null());
        assert!(calls(&s.e, fake(0x428)).is_empty());

        // An actor without a process leaves the group.
        let mut s = scene(2);
        s.e.mem.set_u32(s.actor.addr() + 0x38, 0);
        assert!(choose(&mut s).is_null());
        assert_eq!(
            calls(&s.e, MANAGER_REMOVE_MEMBER),
            vec![vec![MANAGER, s.group.addr(), s.actor.addr()]]
        );
        assert_eq!(
            calls(&s.e, MEMBER_ARRAY_REMOVE_AT),
            vec![vec![s.group.addr() + 0x18, 0, 1]]
        );
    }

    #[test]
    fn choose_target_scores_flags_and_distance() {
        // A single target scores 0 and wins over the starting -FLT_MAX.
        let mut s = scene(1);
        assert_eq!(choose(&mut s), s.targets[0]);

        // Two targets, level above zero: the second has the first flag
        // (+10). Same space, same position: 50 for distance, 1 for the same
        // weapon style.
        let mut s = scene(2);
        s.e.mem.set_u8(s.targets[1].addr() + 0x2c, 1);
        assert_eq!(choose(&mut s), s.targets[1]);
        // The second flag (+100) of the first outweighs it.
        s.e.mem.set_u8(s.targets[0].addr() + 0x2d, 1);
        assert_eq!(choose(&mut s), s.targets[0]);

        // Distance: 1024 units away scores 50 * (1 - 1048576 / 4194304).
        let mut s = scene(2);
        s.e.mem
            .set_u32(s.targets[0].addr() + 0x14, 1024.0f32.to_bits());
        assert_eq!(choose(&mut s), s.targets[1]);
        // Both far, the nearer wins.
        s.e.mem
            .set_u32(s.targets[1].addr() + 0x14, 2048.0f32.to_bits());
        assert_eq!(choose(&mut s), s.targets[0]);

        // Same weapon style as the actor adds 1 to the other target only.
        let mut s = scene(2);
        s.e.mem.set_u8(s.targets[0].addr() + 0x1c, 1);
        assert_eq!(choose(&mut s), s.targets[1]);
    }

    #[test]
    fn choose_target_current_target_bonus_and_penalties() {
        // The process' current target gets 1000, plus 100 while its
        // recency (one second) is inside the window of 5.
        let mut s = scene(2);
        s.e.mem.set_u32(s.process, s.targets[1].addr());
        assert_eq!(choose(&mut s), s.targets[1]);
        // A fleeing current target loses 2: still ahead.
        s.e.mem.set_u8(s.targets[1].addr() + 0x1f, 1);
        assert_eq!(choose(&mut s), s.targets[1]);
        // An unfit one (virtual method 0x22c) loses 3, a target in another
        // space that the actor cannot exist in loses the same again; with a
        // current-target bonus of 1000 it still wins, without it it loses
        // to the other target.
        let mut s = scene(2);
        s.e.mem.set_u8(s.targets[0].addr() + 0x34, 1);
        assert_eq!(choose(&mut s), s.targets[1]);
        let mut s = scene(2);
        s.e.mem.set_u8(s.targets[0].addr() + 0x35, 1);
        assert_eq!(choose(&mut s), s.targets[1]);
        let mut s = scene(2);
        s.e.mem.set_u8(s.targets[0].addr() + 0x1f, 1);
        assert_eq!(choose(&mut s), s.targets[1]);
        // More attackers than the current-target flag: +0.5.
        let mut s = scene(2);
        let entry = target_entry(&s.e, s.group, 0);
        s.e.set(entry, CombatTarget::sAttackerCount, 1);
        assert_eq!(choose(&mut s), s.targets[0]);
    }

    #[test]
    fn choose_target_other_space_uses_the_far_distance() {
        // The actor is in space 1, the first target in space 2 (squared
        // far distance 100, term 50 * (1 - 100 / 4194304)); the second in
        // the actor's space but 1024 away (37.5): the first wins. The
        // actor cannot exist there (-5) and the location is unreachable
        // (-7): the second wins.
        let mut s = scene(2);
        s.e.mem.set_u32(s.actor.addr() + 0x10, 1);
        s.e.mem.set_u32(s.targets[0].addr() + 0x10, 2);
        s.e.mem.set_u32(s.targets[1].addr() + 0x10, 1);
        s.e.mem
            .set_u32(s.targets[1].addr() + 0x14, 1024.0f32.to_bits());
        assert_eq!(choose(&mut s), s.targets[0]);
        s.e.register(0x009a_96f0, |_, _| Ret::default());
        s.e.set_global(0x011a_4d58, 20.0f32);
        assert_eq!(choose(&mut s), s.targets[1]);
        s.e.register(0x009a_96f0, |_, _| 1u32.into_ret());
        s.e.register(0x009a_0f40, |_, _| 1u32.into_ret());
        s.e.set_global(0x011a_4d5c, 20.0f32);
        assert_eq!(choose(&mut s), s.targets[1]);
    }

    #[test]
    fn choose_target_skips_and_fallbacks() {
        // Outside the firing arc (view angle above zero): skipped.
        let mut s = scene(2);
        s.e.mem.set_f32(s.process + 0x200, 90.0);
        s.e.mem.set_u8(s.targets[0].addr() + 0x40, 0);
        s.e.mem.set_u8(s.targets[1].addr() + 0x2c, 1);
        assert_eq!(choose(&mut s), s.targets[1]);
        let arc = calls(&s.e, 0x009a_6d70);
        assert_eq!(arc.len(), 2);
        assert_eq!(arc[0][0], s.actor.addr());
        assert_eq!(arc[0][1], s.targets[0].addr() + 0x40);
        assert_eq!(f32::from_bits(arc[0][2]), (90.0f64.to_radians()) as f32);
        assert_eq!(f32::from_bits(arc[0][3]), std::f32::consts::TAU);

        // Another cell with the same-cell setting on: skipped; every
        // target skipped gives null.
        let mut s = scene(2);
        s.e.mem.set_u8(SAME_CELL_SETTING, 1);
        s.e.mem.set_u32(s.actor.addr() + 0x18, 5);
        s.e.mem.set_u32(s.targets[0].addr() + 0x18, 6);
        s.e.mem.set_u32(s.targets[1].addr() + 0x18, 5);
        assert_eq!(choose(&mut s), s.targets[1]);
        s.e.mem.set_u32(s.targets[1].addr() + 0x18, 6);
        assert!(choose(&mut s).is_null());

        // The force-check flag makes the attack test decide.
        let mut s = scene(2);
        s.e.mem.set_u8(s.actor.addr() + 0x1d, 1);
        s.e.mem.set_u8(s.targets[0].addr() + 0x1e, 0);
        s.e.mem.set_u8(s.targets[0].addr() + 0x2c, 1);
        assert_eq!(choose(&mut s), s.targets[1]);
        assert_eq!(calls(&s.e, ACTOR_GET_SHOULD_ATTACK).len(), 2);

        // Level zero or less: ranked by the last noticed stamp, 30 seconds
        // in the past (the newest stamp wins).
        let mut s = scene(2);
        for (i, noticed) in [50.0f32, 70.0].into_iter().enumerate() {
            s.e.mem.set_i32(s.targets[i].addr() + 0x24, 0);
            let entry = target_entry(&s.e, s.group, i as u32);
            s.e.set(entry, CombatTarget::fLastNoticedTimeStamp, noticed);
        }
        assert_eq!(choose(&mut s), s.targets[1]);
        let entry = target_entry(&s.e, s.group, 1);
        s.e.set(entry, CombatTarget::fLastNoticedTimeStamp, 40.0);
        assert_eq!(choose(&mut s), s.targets[0]);
        // Stamps long before the detection call's own (0) are lifted to it
        // by the maximum: the first target stays the best.
        for i in 0..2 {
            let entry = target_entry(&s.e, s.group, i);
            s.e.set(entry, CombatTarget::fLastNoticedTimeStamp, -1000.0);
        }
        assert_eq!(choose(&mut s), s.targets[0]);
    }

    #[test]
    fn choose_target_negative_level_for_the_current_target_may_become_one() {
        let mut s = scene(2);
        s.e.mem.set_i32(s.targets[0].addr() + 0x24, -1);
        s.e.mem.set_u32(s.process, s.targets[0].addr());
        // The process says it may not be forced: level stays negative and
        // the target is only a fallback; the other one is scored.
        assert_eq!(choose(&mut s), s.targets[1]);
        // Forced to 1: it scores with the current-target bonus.
        s.e.mem.set_u8(s.process + 0x100, 1);
        assert_eq!(choose(&mut s), s.targets[0]);
        assert_eq!(calls(&s.e, 0x0097_ee90).len(), 2);
    }

    // ---- tests of 009871c0 .. 0098a4d0 ----

    fn place(e: &mut Engine, actor: Ptr, x: f32, y: f32, z: f32) {
        for (i, v) in [x, y, z].into_iter().enumerate() {
            e.mem.set_f32(actor.addr() + 0x40 + 4 * i as u32, v);
        }
        e.mem.set_u32(actor.addr() + 0x4c, 0x77);
    }

    fn point(e: &mut Engine, x: f32, y: f32, z: f32) -> Ptr {
        let p = e.mem.alloc(16);
        for (i, v) in [x, y, z].into_iter().enumerate() {
            e.mem.set_f32(p + 4 * i as u32, v);
        }
        Ptr::new(p)
    }

    /// An `array_engine` whose world locations are the actors' position
    /// slot (+0x40, four words), with squared distances over three words,
    /// real `NiPoint3` arithmetic, and the constants and tables of the
    /// geometry functions.
    fn geometry_engine() -> Engine {
        let mut e = array_engine();
        e.register(GET_WORLD_LOCATION, |e, a| {
            for word in 0..4 {
                let value = e.mem.u32(a[0] + 0x40 + 4 * word);
                e.mem.set_u32(a[1] + 4 * word, value);
            }
            a[1].into_ret()
        });
        e.register(DISTANCE_SQUARED, |e, a| {
            let mut sum = 0.0f32;
            for word in 0..3 {
                let d = e.mem.f32(a[0] + 4 * word) - e.mem.f32(a[1] + 4 * word);
                sum += d * d;
            }
            sum.into_ret()
        });
        e.register(POINT_COMPONENT, |_, a| (a[0] + 4 * a[1]).into_ret());
        e.register(POINT_ADD, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) + e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        e.register(POINT_SUBTRACT, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) - e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        e.register(POINT_SCALE_DOWN, |e, a| {
            let inverse = 1.0 / f32::from_bits(a[2]);
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) * inverse;
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        e.register(POINT_DIVIDE_IN_PLACE, |e, a| {
            let inverse = 1.0 / f32::from_bits(a[1]);
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) * inverse;
                e.mem.set_f32(a[0] + 4 * i, v);
            }
            a[0].into_ret()
        });
        e.register(POINT_SCALE, |e, a| {
            let factor = f32::from_bits(a[2]);
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) * factor;
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        e.register(POINT_ACCUMULATE, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) + e.mem.f32(a[1] + 4 * i);
                e.mem.set_f32(a[0] + 4 * i, v);
            }
            a[0].into_ret()
        });
        e.register(POINT_DOT, |e, a| {
            let mut sum = 0.0f32;
            for i in 0..3 {
                sum += e.mem.f32(a[0] + 4 * i) * e.mem.f32(a[1] + 4 * i);
            }
            sum.into_ret()
        });
        e.register(POINT_LENGTH_SQUARED, |e, a| {
            let mut sum = 0.0f32;
            for i in 0..3 {
                sum += e.mem.f32(a[0] + 4 * i) * e.mem.f32(a[0] + 4 * i);
            }
            sum.into_ret()
        });
        e.register(POINT_UNITIZE_GET_LENGTH, |e, a| {
            let mut sum = 0.0f32;
            for i in 0..3 {
                sum += e.mem.f32(a[0] + 4 * i) * e.mem.f32(a[0] + 4 * i);
            }
            let length = sum.sqrt();
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) / length;
                e.mem.set_f32(a[0] + 4 * i, v);
            }
            length.into_ret()
        });
        e.register(POINT_NO_OP, |_, a| a[0].into_ret());
        e.register(SQUARE_ROOT, |_, a| f32::from_bits(a[0]).sqrt().into_ret());
        e.register(VECTOR_CONSTRUCTOR, |_, _| Ret::default());
        e.register(SETTING_INT_VALUE, |_, a| a[0].into_ret());
        e.set_global(TWO_FLOAT, 2.0f32);
        e.set_global(SPHERE_MINIMUM_RADIUS, 1024.0f32);
        e.set_global(FOUR_DOUBLE, 4.0f64);
        e.set_global(FRACTION_LOW, 0.33f32 as f64);
        e.set_global(FRACTION_HIGH, 0.67f32 as f64);
        e.mem.write(
            CORNER_TABLE,
            &[
                1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0,
            ],
        );
        e
    }

    type Nodes = std::rc::Rc<std::cell::RefCell<Vec<([f32; 3], f32, f32)>>>;

    /// Doubles for the avoid node classes; the returned list collects
    /// (point, first value, second value) of every node built.
    fn avoid_nodes(e: &mut Engine) -> Nodes {
        let nodes: Nodes = Default::default();
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(AVOID_ARRAY_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0xa11a);
            a[0].into_ret()
        });
        let sink = nodes.clone();
        e.register_double(AVOID_NODE_CONSTRUCT, move |e, a| {
            sink.borrow_mut().push((
                [e.mem.f32(a[1]), e.mem.f32(a[1] + 4), e.mem.f32(a[1] + 8)],
                f32::from_bits(a[2]),
                f32::from_bits(a[3]),
            ));
            a[0].into_ret()
        });
        e.register(AVOID_ARRAY_ADD, |_, _| 0u32.into_ret());
        nodes
    }

    #[test]
    fn target_by_index_is_the_array_element() {
        let mut e = engine();
        let (a, b) = (actor(&mut e, 1), actor(&mut e, 2));
        let group = group_with(&mut e, &[a, b], &[]);
        assert_eq!(fn_009871c0(&mut e, group, 1), target_entry(&e, group, 1));
    }

    #[test]
    fn detection_event_stamp_is_zero_test() {
        let mut e = engine();
        let target = e.new_object::<CombatTarget>();
        assert!(fn_009871e0(&mut e, target));
        e.set(target, CombatTarget::fDetectionEventTimeStamp, 0.5);
        assert!(!fn_009871e0(&mut e, target));
    }

    /// Group with one target and the stamps all fresh (age 0), the settings
    /// the chain of `fn_00987220` compares with at 1000.
    fn drop_scene(e: &mut Engine) -> (Ptr<CombatGroup>, Ptr<CombatTarget>, Ptr) {
        let who = actor(e, 5);
        let group = group_with(e, &[who], &[]);
        let target = target_entry(e, group, 0);
        fresh_stamps(e, group, target);
        for setting in [
            0x011c_e290u32,
            0x011c_e750,
            0x011c_e744,
            0x011c_f368,
            0x011c_f3b4,
            0x011c_f0a4,
        ] {
            e.set_global(setting, 1000.0f32);
        }
        e.register(ACTOR_TEST_00437BD0, |e, a| e.mem.u8(a[0] + 0x39).into_ret());
        e.register(fake(0x2e8), |e, a| e.mem.u8(a[0] + 0x36).into_ret());
        (group, target, who)
    }

    /// All the time stamps `fn_00987220` reads are the game time now.
    fn fresh_stamps(e: &mut Engine, group: Ptr<CombatGroup>, target: Ptr<CombatTarget>) {
        for off in [0x4cu32, 0x50, 0x54, 0x58, 0x5c, 0x60] {
            e.mem.set_f32(target.addr() + off, NOW);
        }
        e.mem.set_f32(group.addr() + 0xe8, NOW);
    }

    #[test]
    fn drop_chain_ends_with_the_actor_virtuals() {
        let mut e = geometry_engine();
        let (group, target, who) = drop_scene(&mut e);
        for (checks, second, expected) in [(0, 0, false), (1, 0, true), (1, 1, false)] {
            e.mem.set_u8(who.addr() + 0x34, checks);
            e.mem.set_u8(who.addr() + 0x36, second);
            assert_eq!(fn_00987220(&mut e, group, target), expected);
        }
        // The actor property alone answers true.
        e.mem.set_u8(who.addr() + 0x34, 0);
        e.mem.set_u8(who.addr() + 0x39, 1);
        assert!(fn_00987220(&mut e, group, target));
    }

    #[test]
    fn drop_chain_fleeing_group_and_old_detection() {
        let mut e = geometry_engine();
        let (group, target, who) = drop_scene(&mut e);
        e.set(group, CombatGroup::iFleeingMemberCount, 2);
        assert!(!fn_00987220(&mut e, group, target));
        // Detected 10 seconds ago, limit 5: dropped at once.
        e.mem.set_f32(target.addr() + 0x50, NOW - 10.0);
        e.set_global(0x011c_e290, 5.0f32);
        assert!(fn_00987220(&mut e, group, target));
        // Limit 50 not reached; the second limit (1) is, and no member is
        // near (no members): dropped.
        e.set_global(0x011c_e290, 50.0f32);
        e.set_global(0x011c_e750, 1.0f32);
        e.set_global(0x011c_e38c, 10.0f32);
        assert!(fn_00987220(&mut e, group, target));
        // A member within the radius keeps the target.
        let near = actor(&mut e, 6);
        place(&mut e, near, 1.0, 0.0, 0.0);
        place(&mut e, who, 0.0, 0.0, 0.0);
        let group = group_with(&mut e, &[who], &[near]);
        let target = target_entry(&e, group, 0);
        fresh_stamps(&mut e, group, target);
        e.mem.set_f32(target.addr() + 0x50, NOW - 10.0);
        e.set(group, CombatGroup::iFleeingMemberCount, 2);
        assert!(!fn_00987220(&mut e, group, target));
    }

    #[test]
    fn drop_chain_old_notice() {
        let mut e = geometry_engine();
        let (group, target, who) = drop_scene(&mut e);
        e.mem.set_f32(target.addr() + 0x58, NOW - 10.0);
        e.set_global(0x011c_e744, 5.0f32);
        // No attackers: dropped.
        assert!(fn_00987220(&mut e, group, target));
        e.set(target, CombatTarget::sAttackerCount, 1);
        e.set_global(0x011c_f368, 50.0f32);
        assert!(!fn_00987220(&mut e, group, target));
        // The second limit reached: dropped when no member is near.
        e.set_global(0x011c_f368, 5.0f32);
        e.set_global(0x011c_f3fc, 10.0f32);
        assert!(fn_00987220(&mut e, group, target));
        // A member near the target's actor keeps it, unless the search's
        // path handler has not moved.
        let near = actor(&mut e, 6);
        place(&mut e, near, 1.0, 0.0, 0.0);
        place(&mut e, who, 0.0, 0.0, 0.0);
        let group = group_with(&mut e, &[who], &[near]);
        let target = target_entry(&e, group, 0);
        fresh_stamps(&mut e, group, target);
        e.mem.set_f32(target.addr() + 0x58, NOW - 10.0);
        e.set(target, CombatTarget::sAttackerCount, 1);
        assert!(!fn_00987220(&mut e, group, target));
        e.set(group, CombatGroup::iSearchCount, 1);
        e.mem.set_u32(group.addr() + 0xd4, 0x1234);
        e.set_global(0x011c_ef50, 10u32);
        e.set_global(0x011c_f0a4, -1.0f32);
        e.register(NI_POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(PATH_HANDLER_DISTANCE_TRAVELLED, |_, _| 0.0f32.into_ret());
        assert!(fn_00987220(&mut e, group, target));
        e.register(PATH_HANDLER_DISTANCE_TRAVELLED, |_, _| 2.0f32.into_ret());
        assert!(!fn_00987220(&mut e, group, target));
    }

    #[test]
    fn drop_chain_search_tests() {
        let mut e = geometry_engine();
        let (group, target, _) = drop_scene(&mut e);
        e.set(group, CombatGroup::iSearchCount, 5);
        e.set(target, CombatTarget::sLastSearchNoticed, 1);
        e.set_global(0x011c_ef50, 2u32);
        // 5 searches against 1 + 2: dropped.
        assert!(fn_00987220(&mut e, group, target));
        e.set_global(0x011c_ef50, 4u32);
        // Search started 1 second ago (< 3), last attacked-member stamp 10
        // seconds ago (>= 3), last detected stamp -FLT_MAX, noticed 10
        // seconds ago against 5: dropped.
        e.set_global(0x011c_f0a4, 3.0f32);
        e.mem.set_f32(group.addr() + 0xe8, NOW - 1.0);
        e.mem.set_f32(target.addr() + 0x60, NOW - 10.0);
        e.mem.set_f32(target.addr() + 0x50, -f32::MAX);
        e.mem.set_f32(target.addr() + 0x58, NOW - 10.0);
        e.set_global(0x011c_f3b4, 5.0f32);
        assert!(fn_00987220(&mut e, group, target));
        // Each condition missing keeps the target.
        e.set_global(0x011c_f3b4, 50.0f32);
        assert!(!fn_00987220(&mut e, group, target));
        e.set_global(0x011c_f3b4, 5.0f32);
        e.mem.set_f32(target.addr() + 0x50, NOW);
        assert!(!fn_00987220(&mut e, group, target));
        e.mem.set_f32(target.addr() + 0x50, -f32::MAX);
        e.mem.set_f32(target.addr() + 0x60, NOW - 1.0);
        assert!(!fn_00987220(&mut e, group, target));
        e.mem.set_f32(target.addr() + 0x60, NOW - 10.0);
        e.mem.set_f32(group.addr() + 0xe8, NOW - 5.0);
        assert!(!fn_00987220(&mut e, group, target));
    }

    #[test]
    fn attacker_count_goes_up_and_down_for_known_targets() {
        let mut e = engine();
        let (a, b) = (actor(&mut e, 1), actor(&mut e, 2));
        let group = group_with(&mut e, &[a], &[]);
        let entry = target_entry(&e, group, 0);
        combat_group_increment_attacker_count(&mut e, group, a);
        combat_group_increment_attacker_count(&mut e, group, a);
        fn_009874f0(&mut e, group, a);
        assert_eq!(e.get(entry, CombatTarget::sAttackerCount), 1);
        combat_group_increment_attacker_count(&mut e, group, b);
        fn_009874f0(&mut e, group, b);
        assert_eq!(e.get(entry, CombatTarget::sAttackerCount), 1);
        fn_009874f0(&mut e, group, a);
        fn_009874f0(&mut e, group, a);
        assert_eq!(e.get(entry, CombatTarget::sAttackerCount), 0xffff);
    }

    #[test]
    fn merge_group_moves_clusters_members_targets_and_music() {
        let mut e = array_engine();
        let (t1, m1) = (actor(&mut e, 1), actor(&mut e, 2));
        let (t2, m2) = (actor(&mut e, 3), actor(&mut e, 4));
        let this = group_with(&mut e, &[t1], &[m1]);
        let other = group_with(&mut e, &[t2], &[m2]);
        e.set(this, CombatGroup::cCombatMusicState, -1);
        e.set(other, CombatGroup::cCombatMusicState, 3);
        let member: Ptr<CombatMember> = Ptr::new(e.mem.u32(other.addr() + 0x1c));
        e.set(member, CombatMember::fDamagePerSecond, 5.0);
        e.set(member, CombatMember::fCombatStrength, 7.0);
        e.register(OTHER_TARGET_ARRAY, |_, a| (a[0] + 8).into_ret());
        e.register(OTHER_MEMBER_ARRAY, |_, a| (a[0] + 0x18).into_ret());
        // Two clusters in `other` (the size of its cluster array is at
        // +0x140).
        e.mem.set_u32(other.addr() + 0x140, 2);
        e.register(CLUSTER_BY_INDEX, |_, a| (0xc100 + a[1]).into_ret());
        let added = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let sink = added.clone();
        e.register_double(POINTER_ARRAY_ADD, move |e, a| {
            sink.borrow_mut().push((a[0], e.mem.u32(a[1])));
            0u32.into_ret()
        });
        e.register(0x0084_54f0, |_, _| Ret::default());
        assert!(combat_group_merge_group(&mut e, this, other));
        assert_eq!(
            *added.borrow(),
            vec![(this.addr() + 0x138, 0xc100), (this.addr() + 0x138, 0xc101)]
        );
        assert_eq!(calls(&e, 0x0084_54f0), vec![vec![other.addr() + 0x138, 1]]);
        // The member came over, was told its group and got the other's
        // numbers; so did the target.
        assert_eq!(array_len(&e, this, 0x18), 2);
        let mine = fn_009865f0(&mut e, this, m2);
        assert_eq!(e.get(mine, CombatMember::fDamagePerSecond), 5.0);
        assert_eq!(e.get(mine, CombatMember::fCombatStrength), 7.0);
        assert_eq!(calls(&e, fake(0x3fc)), vec![vec![m2.addr(), this.addr()]]);
        assert_eq!(array_len(&e, this, 8), 2);
        assert!(!fn_009865d0(&mut e, this, t2).is_null());
        assert_eq!(e.get(this, CombatGroup::cCombatMusicState), 3);
        // A music state of -1 is not taken.
        e.set(other, CombatGroup::cCombatMusicState, -1);
        assert!(combat_group_merge_group(&mut e, this, other));
        assert_eq!(e.get(this, CombatGroup::cCombatMusicState), 3);
    }

    #[test]
    fn merge_group_refuses_incompatible_actors_and_logs_failed_adds() {
        let mut e = array_engine();
        let (t1, m1) = (actor(&mut e, 1), actor(&mut e, 2));
        let (t2, m2) = (actor(&mut e, 3), actor(&mut e, 4));
        let this = group_with(&mut e, &[t1], &[m1]);
        let other = group_with(&mut e, &[t2], &[m2]);
        e.register(OTHER_TARGET_ARRAY, |_, a| (a[0] + 8).into_ret());
        e.register(OTHER_MEMBER_ARRAY, |_, a| (a[0] + 0x18).into_ret());
        e.register(0x0084_54f0, |_, _| Ret::default());
        record(&mut e, &[POINTER_ARRAY_ADD]);
        // `this` has a member that cannot fight other's target.
        e.mem.set_u8(m1.addr() + 0x3c, 1);
        assert!(!combat_group_merge_group(&mut e, this, other));
        // `other` has a member that cannot fight this group's target.
        e.mem.set_u8(m1.addr() + 0x3c, 0);
        e.mem.set_u8(m2.addr() + 0x3c, 1);
        assert!(!combat_group_merge_group(&mut e, this, other));
        assert!(calls(&e, 0x0084_54f0).is_empty());
        assert_eq!(array_len(&e, this, 0x18), 1);
        // A member of `other` that is a target of `this` cannot be added,
        // and a target of `other` that is a member of `this` neither.
        e.mem.set_u8(m2.addr() + 0x3c, 0);
        let other = group_with(&mut e, &[m1], &[t1]);
        assert!(combat_group_merge_group(&mut e, this, other));
        let logs = calls(&e, LOG_MESSAGE);
        assert_eq!(logs.last().unwrap(), &vec![MERGE_TARGET_ERROR, 2, 0xbeef]);
        assert!(logs.contains(&vec![MERGE_MEMBER_ERROR, 1, 0xbeef]));
        assert_eq!(array_len(&e, this, 0x18), 1);
        assert_eq!(array_len(&e, this, 8), 1);
    }

    #[test]
    fn cluster_and_music_accessors() {
        let mut e = engine();
        let group = e.new_object::<CombatGroup>();
        e.mem.set_u32(group.addr() + 0x140, 4);
        assert_eq!(fn_009877c0(&mut e, group), 4);
        e.set(group, CombatGroup::cCombatMusicState, -1);
        assert_eq!(fn_00987800(&mut e, group), -1);
        record(&mut e, &[0x0084_54f0]);
        fn_009877e0(&mut e, group);
        assert_eq!(calls(&e, 0x0084_54f0), vec![vec![group.addr() + 0x138, 1]]);
    }

    #[test]
    fn set_location_copies_the_point_and_sets_the_space() {
        let mut e = engine();
        let location = e.mem.alloc(16);
        let source = point(&mut e, 1.0, 2.0, 3.0);
        bgs_world_location_set_location(&mut e, Ptr::new(location), source, 0x44);
        assert_eq!(e.mem.f32(location), 1.0);
        assert_eq!(e.mem.f32(location + 4), 2.0);
        assert_eq!(e.mem.f32(location + 8), 3.0);
        assert_eq!(e.mem.u32(location + 12), 0x44);
    }

    #[test]
    fn detection_level_call_has_no_actor_and_starts_at_minus_one_hundred() {
        let mut e = engine();
        e.register_double(0x0098_87b0, |e, a| {
            assert_eq!(&a[..3], &[0x5000_0000, 0, 0x77]);
            assert_eq!(e.mem.i32(a[3]), -100);
            assert_eq!(e.mem.u8(a[5]), 0);
            assert_eq!(e.mem.u8(a[6]), 0);
            e.mem.set_i32(a[3], 42);
            Ret::default()
        });
        let level = fn_00989040(&mut e, Ptr::new(0x5000_0000), Ptr::new(0x77));
        assert_eq!(level, 42);
    }

    #[test]
    fn last_detected_stamp_or_minus_max() {
        let mut e = engine();
        let who = actor(&mut e, 1);
        let group = group_with(&mut e, &[who], &[]);
        let target = target_entry(&e, group, 0);
        e.set(target, CombatTarget::fLastDetectedTimeStamp, 12.5);
        let result = e.mem.alloc(4);
        let stranger = actor(&mut e, 2);
        assert_eq!(
            fn_00989090(&mut e, group, Ptr::new(result), stranger),
            Ptr::new(result)
        );
        assert_eq!(e.mem.f32(result), -f32::MAX);
        fn_00989090(&mut e, group, Ptr::new(result), who);
        assert_eq!(e.mem.f32(result), 12.5);
    }

    #[test]
    fn last_detected_and_seen_locations() {
        let mut e = engine();
        let who = actor(&mut e, 1);
        let stranger = actor(&mut e, 9);
        let group = group_with(&mut e, &[who], &[]);
        let target = target_entry(&e, group, 0);
        let out = e.mem.alloc(16);
        // No space in the detected location: nothing is copied.
        assert!(!fn_009890e0(&mut e, group, who, Ptr::new(out)));
        assert!(!fn_009890e0(&mut e, group, stranger, Ptr::new(out)));
        for word in 0..4 {
            e.mem.set_u32(target.addr() + 0x18 + 4 * word, 0x10 + word);
        }
        assert!(fn_009890e0(&mut e, group, who, Ptr::new(out)));
        assert_eq!(e.mem.u32(out + 12), 0x13);
        assert_eq!(e.mem.u32(out), 0x10);
        // Seen location: copied when the time given is later than the age.
        for word in 0..4 {
            e.mem.set_u32(target.addr() + 0x28 + 4 * word, 0x20 + word);
        }
        e.set(target, CombatTarget::fLastSeenTimeStamp, NOW - 10.0);
        let seen = e.mem.alloc(16);
        assert!(!fn_00989140(&mut e, group, 0, Ptr::new(seen), 10.0));
        assert!(fn_00989140(&mut e, group, 0, Ptr::new(seen), 10.5));
        assert_eq!(e.mem.u32(seen), 0x20);
        assert_eq!(e.mem.u32(seen + 12), 0x23);
    }

    #[test]
    fn member_los_count_defaults_to_zero() {
        let mut e = engine();
        let who = actor(&mut e, 1);
        let stranger = actor(&mut e, 2);
        let group = group_with(&mut e, &[who], &[]);
        let target = target_entry(&e, group, 0);
        e.set(target, CombatTarget::cMemberLOSCount, 3);
        assert_eq!(fn_009891a0(&mut e, group, who), 3);
        assert_eq!(fn_009891a0(&mut e, group, stranger), 0);
    }

    #[test]
    fn strategy_assignment_is_cleared_only_when_it_matches() {
        let mut e = engine();
        let who = actor(&mut e, 1);
        let group = group_with(&mut e, &[], &[who]);
        let member: Ptr<CombatMember> = Ptr::new(e.mem.u32(group.addr() + 0x1c));
        e.register(0x0097_ae90, |_, a| a[0].into_ret());
        e.register(0x0096_11e0, |_, a| (a[0] & 0xff).into_ret());
        e.set(member, CombatMember::iGroupStrategyAssignment, 7);
        // No group strategy: nothing happens.
        fn_009891d0(&mut e, group, who, Ptr::new(7));
        assert_eq!(e.get(member, CombatMember::iGroupStrategyAssignment), 7);
        e.set(group, CombatGroup::pGroupStrategy, Ptr::new(0x999));
        fn_009891d0(&mut e, group, who, Ptr::new(8));
        assert_eq!(e.get(member, CombatMember::iGroupStrategyAssignment), 7);
        fn_009891d0(&mut e, group, who, Ptr::new(0));
        assert_eq!(e.get(member, CombatMember::iGroupStrategyAssignment), 7);
        fn_009891d0(&mut e, group, who, Ptr::new(7));
        assert_eq!(e.get(member, CombatMember::iGroupStrategyAssignment), 0);
    }

    #[test]
    fn attacked_member_stamp_and_location() {
        let mut e = geometry_engine();
        let who = actor(&mut e, 1);
        let stranger = actor(&mut e, 2);
        place(&mut e, who, 1.0, 2.0, 3.0);
        let group = group_with(&mut e, &[who], &[]);
        let target = target_entry(&e, group, 0);
        fn_00989230(&mut e, group, 0, Ptr::NULL);
        assert_eq!(
            e.get(target, CombatTarget::fLastAttackedMemberTimeStamp),
            0.0
        );
        fn_00989230(&mut e, group, 0, stranger);
        assert_eq!(
            e.get(target, CombatTarget::fLastAttackedMemberTimeStamp),
            0.0
        );
        fn_00989230(&mut e, group, 0, who);
        assert_eq!(
            e.get(target, CombatTarget::fLastAttackedMemberTimeStamp),
            NOW
        );
        assert_eq!(e.mem.f32(target.addr() + 0x38), 1.0);
        assert_eq!(e.mem.f32(target.addr() + 0x40), 3.0);
        assert_eq!(e.mem.u32(target.addr() + 0x44), 0x77);
    }

    #[test]
    fn avoid_nodes_around_the_middle_of_two_points() {
        let mut e = geometry_engine();
        let nodes = avoid_nodes(&mut e);
        let me = actor(&mut e, 1);
        place(&mut e, me, 0.0, 0.0, 0.0);
        let (near, far, skipped) = (actor(&mut e, 2), actor(&mut e, 3), actor(&mut e, 4));
        place(&mut e, near, 5.0, 0.0, 1.0);
        place(&mut e, far, 5.0, 0.0, 10.0);
        place(&mut e, skipped, 5.0, 0.0, 0.5);
        let group = group_with(&mut e, &[near, far, skipped], &[]);
        e.set_global(0x011c_f380, 1.0f32);
        e.set_global(0x011c_e648, 1.5f32);
        e.set_global(0x011c_fcf8, 2.5f32);
        let target = point(&mut e, 10.0, 0.0, 0.0);
        // Middle (5, 0, 0): squared limit (2 * 2 + 25) * 1 = 29.
        let array = fn_009892a0(&mut e, group, me, target, 2.0, Ptr::NULL, skipped);
        assert_eq!(e.mem.u32(array.addr()), 0xa11a);
        assert_eq!(*nodes.borrow(), vec![([5.0, 0.0, 1.0], 1.5, 2.5)]);
        assert_eq!(calls(&e, AVOID_ARRAY_ADD).len(), 1);
        // A given array is reused.
        let again = fn_009892a0(&mut e, group, me, target, 2.0, array, Ptr::NULL);
        assert_eq!(again, array);
        assert_eq!(nodes.borrow().len(), 3);
        assert_eq!(calls(&e, OPERATOR_NEW).len(), 1);
    }

    #[test]
    fn avoid_node_for_the_targets_bounding_sphere() {
        let mut e = geometry_engine();
        let nodes = avoid_nodes(&mut e);
        e.set_global(0x011c_fcf8, 2.0f32);
        let (a, b) = (actor(&mut e, 1), actor(&mut e, 2));
        place(&mut e, a, 0.0, 0.0, 0.0);
        place(&mut e, b, 10.0, 0.0, 0.0);
        let group = group_with(&mut e, &[a, b], &[]);
        // Small box: the radius stays 1024, the second value is 2 * 4.
        let array = fn_00989490(&mut e, group, 0, 0, 0, Ptr::NULL);
        assert_eq!(e.mem.u32(array.addr()), 0xa11a);
        assert_eq!(*nodes.borrow(), vec![([5.0, 0.0, 0.0], 1024.0, 8.0)]);
        // A huge one: the radius is the square root of the squared extent.
        place(&mut e, b, 3000.0, 0.0, 0.0);
        let again = fn_00989490(&mut e, group, 0, 0, 0, array);
        assert_eq!(again, array);
        assert_eq!(nodes.borrow()[1], ([1500.0, 0.0, 0.0], 3000.0, 8.0));
    }

    #[test]
    fn avoid_nodes_for_targets_near_an_actor() {
        let mut e = geometry_engine();
        let nodes = avoid_nodes(&mut e);
        e.set_global(0x011c_ea50, 10.0f32);
        e.set_global(0x011c_e900, 1.5f32);
        e.set_global(0x011c_eb2c, 2.5f32);
        let me = actor(&mut e, 1);
        place(&mut e, me, 0.0, 0.0, 0.0);
        let (near, far) = (actor(&mut e, 2), actor(&mut e, 3));
        place(&mut e, near, 5.0, 0.0, 0.0);
        place(&mut e, far, 20.0, 0.0, 0.0);
        let group = group_with(&mut e, &[far, near], &[]);
        let array = fn_009895a0(&mut e, group, me, Ptr::NULL);
        assert_eq!(e.mem.u32(array.addr()), 0xa11a);
        assert_eq!(*nodes.borrow(), vec![([5.0, 0.0, 0.0], 1.5, 2.5)]);
        // Nothing near: null stays null.
        let empty = group_with(&mut e, &[far], &[]);
        assert!(fn_009895a0(&mut e, empty, me, Ptr::NULL).is_null());
    }

    /// A group whose members sit at the given x positions (y = z = 0).
    fn members_at(e: &mut Engine, xs: &[f32]) -> (Ptr<CombatGroup>, Vec<Ptr>) {
        let actors: Vec<Ptr> = xs
            .iter()
            .enumerate()
            .map(|(i, &x)| {
                let a = actor(e, 10 + i as u32);
                place(e, a, x, 0.0, 0.0);
                a
            })
            .collect();
        let group = group_with(e, &[], &actors);
        (group, actors)
    }

    #[test]
    fn member_search_by_radius_and_nearest() {
        let mut e = geometry_engine();
        let (group, members) = members_at(&mut e, &[5.0, 3.0, 20.0]);
        let origin = point(&mut e, 0.0, 0.0, 0.0);
        // First within 10, excluding the first, then excluding nothing.
        assert_eq!(
            fn_009897f0(&mut e, group, origin, 10.0, members[0]),
            members[1]
        );
        assert_eq!(
            fn_009897f0(&mut e, group, origin, 10.0, Ptr::NULL),
            members[0]
        );
        assert!(fn_009897f0(&mut e, group, origin, 2.0, Ptr::NULL).is_null());
        // Nearest.
        assert_eq!(
            fn_009898b0(&mut e, group, origin, Ptr::NULL, f32::MAX),
            members[1]
        );
        assert_eq!(
            fn_009898b0(&mut e, group, origin, members[1], 100.0),
            members[0]
        );
        assert!(fn_009898b0(&mut e, group, origin, Ptr::NULL, 2.0).is_null());
        // From an actor's own location, the actor excluded.
        let me = actor(&mut e, 99);
        place(&mut e, me, 4.0, 0.0, 0.0);
        assert_eq!(fn_00989880(&mut e, group, me, 2.0), members[0]);
        assert_eq!(fn_00989970(&mut e, group, me, 100.0), members[0]);
        assert!(fn_00989880(&mut e, group, me, 0.5).is_null());
        // The actor itself is not found.
        assert_eq!(fn_00989880(&mut e, group, members[0], 3.0), members[1]);
    }

    #[test]
    fn target_near_location_with_and_without_the_recency_test() {
        let mut e = geometry_engine();
        let (far, stale, fresh) = (actor(&mut e, 1), actor(&mut e, 2), actor(&mut e, 3));
        place(&mut e, far, 20.0, 0.0, 0.0);
        place(&mut e, stale, 5.0, 0.0, 0.0);
        place(&mut e, fresh, 6.0, 0.0, 0.0);
        let group = group_with(&mut e, &[far, stale, fresh], &[]);
        let origin = point(&mut e, 0.0, 0.0, 0.0);
        e.set_global(0x011c_e9b4, 50.0f32);
        for (i, stamp) in [NOW, NOW - 60.0, NOW - 1.0].into_iter().enumerate() {
            let entry = target_entry(&e, group, i as u32);
            e.set(entry, CombatTarget::fLastDetectedTimeStamp, stamp);
        }
        let first = target_entry(&e, group, 0);
        assert!(fn_009897a0(&mut e, first));
        let second = target_entry(&e, group, 1);
        assert!(!fn_009897a0(&mut e, second));
        assert_eq!(
            combat_group_find_target_near_location(&mut e, group, origin, 10.0, false),
            stale
        );
        assert_eq!(
            combat_group_find_target_near_location(&mut e, group, origin, 10.0, true),
            fresh
        );
        assert!(
            combat_group_find_target_near_location(&mut e, group, origin, 1.0, false).is_null()
        );
    }

    /// A group with members and targets at the given x positions.
    fn sides(e: &mut Engine, members: &[f32], targets: &[f32]) -> Ptr<CombatGroup> {
        let make = |e: &mut Engine, xs: &[f32], base: u32| -> Vec<Ptr> {
            xs.iter()
                .enumerate()
                .map(|(i, &x)| {
                    let a = actor(e, base + i as u32);
                    place(e, a, x, 0.0, 0.0);
                    a
                })
                .collect()
        };
        let members = make(e, members, 10);
        let targets = make(e, targets, 50);
        group_with(e, &targets, &members)
    }

    #[test]
    fn member_target_intersection() {
        let mut e = geometry_engine();
        // Nothing to compare.
        let group = sides(&mut e, &[], &[1.0]);
        assert!(!combat_group_check_member_target_intersection(
            &mut e, group
        ));
        let group = sides(&mut e, &[1.0], &[]);
        assert!(!combat_group_check_member_target_intersection(
            &mut e, group
        ));
        let group = sides(&mut e, &[1.0], &[5.0]);
        assert!(!combat_group_check_member_target_intersection(
            &mut e, group
        ));
        // Members 0 and 2 against targets 10 and 12: only the targets are
        // behind the point P (fractions 0 and 1), which is neither "both
        // above 0.33" nor "both below 0.67".
        let group = sides(&mut e, &[0.0, 2.0], &[10.0, 12.0]);
        assert!(!combat_group_check_member_target_intersection(
            &mut e, group
        ));
        // Members 0 and 100 against targets 20 and 30: P lies at about
        // 32.3 on the other side, so one member and both targets are
        // behind it (0.5 and 1.0).
        let group = sides(&mut e, &[0.0, 100.0], &[20.0, 30.0]);
        assert!(combat_group_check_member_target_intersection(&mut e, group));
    }

    #[test]
    fn member_target_intersection_places_its_alloca16_tables_as_the_exe_does() {
        let mut e = geometry_engine();
        let group = sides(&mut e, &[0.0, 2.0], &[10.0, 12.0]);
        // Two members and two targets: the block holds 0x104 bytes of fixed
        // locals, 2 * 24 bytes of tables and both pads.
        let size = INTERSECTION_FIXED_LOCALS + 2 * 24 + 0x20;
        // The frame is the next local on the thread's stack; its bytes stay there after the
        // call returns.
        let block = e.next_stack_local(size);
        let ebp = block + size;
        combat_group_check_member_target_intersection(&mut e, group);
        // `_alloca16` from ESP = EBP-0x104, then from the member table.
        let members = (ebp - 0x104 - 24) & !0xf;
        let targets = (members - 24) & !0xf;
        assert!(targets >= block);
        let xs = |e: &Engine, table: u32| [e.mem.f32(table), e.mem.f32(table + 12)];
        assert_eq!(xs(&e, members), [0.0, 2.0]);
        assert_eq!(xs(&e, targets), [10.0, 12.0]);
        // The centres in the fixed locals under EBP.
        assert_eq!(e.mem.f32(ebp - 0x4c), 1.0);
        assert_eq!(e.mem.f32(ebp - 0x74), 11.0);
        assert_eq!(e.mem.f32(ebp - 0x80), 6.0);
    }

    #[test]
    fn bounding_boxes_of_members_and_targets() {
        let mut e = geometry_engine();
        let (m1, m2) = (actor(&mut e, 1), actor(&mut e, 2));
        place(&mut e, m1, 1.0, 5.0, 3.0);
        place(&mut e, m2, 4.0, 2.0, 9.0);
        let (t1, t2) = (actor(&mut e, 3), actor(&mut e, 4));
        place(&mut e, t1, -2.0, 0.0, 0.0);
        place(&mut e, t2, 2.0, 4.0, 6.0);
        let group = group_with(&mut e, &[t1, t2], &[m1, m2]);
        let out = e.mem.alloc(12);
        // Members: box (1, 2, 3) .. (4, 5, 9).
        let squared = fn_00989e20(&mut e, group, Ptr::new(out));
        assert_eq!(
            [e.mem.f32(out), e.mem.f32(out + 4), e.mem.f32(out + 8)],
            [2.5, 3.5, 6.0]
        );
        assert_eq!(squared, 54.0);
        // Targets: box (-2, 0, 0) .. (2, 4, 6).
        let squared = fn_00989eb0(&mut e, group, Ptr::new(out));
        assert_eq!(
            [e.mem.f32(out), e.mem.f32(out + 4), e.mem.f32(out + 8)],
            [0.0, 2.0, 3.0]
        );
        assert_eq!(squared, 16.0 + 16.0 + 36.0);
        // The corners: corner 0 holds the maxima, corner 7 the minima, and
        // `keep` extends what is there.
        let corners = e.mem.alloc(96);
        fn_00989f40(&mut e, group, Ptr::new(corners), false);
        assert_eq!(e.mem.f32(corners), 4.0);
        assert_eq!(e.mem.f32(corners + 4), 5.0);
        assert_eq!(e.mem.f32(corners + 7 * 12 + 8), 3.0);
        // Corner 1 (table 1, 1, 0) holds max x, max y and min z.
        assert_eq!(e.mem.f32(corners + 12 + 8), 3.0);
        fn_0098a110(&mut e, group, Ptr::new(corners), true);
        assert_eq!(e.mem.f32(corners), 4.0);
        assert_eq!(e.mem.f32(corners + 7 * 12), -2.0);
        assert_eq!(e.mem.f32(corners + 7 * 12 + 8), 0.0);
    }

    #[test]
    fn fleeing_state_and_strategy_assignments() {
        let mut e = engine();
        let (a, b) = (actor(&mut e, 1), actor(&mut e, 2));
        let stranger = actor(&mut e, 3);
        let group = group_with(&mut e, &[], &[a, b]);
        assert!(!fn_0098a2e0(&mut e, group));
        e.set(group, CombatGroup::iFleeingMemberCount, 1);
        assert!(fn_0098a2e0(&mut e, group));
        e.set(group, CombatGroup::iNonFleeingMemberCount, 1);
        assert!(!fn_0098a2e0(&mut e, group));

        combat_group_set_member_group_strategy_assignment(&mut e, group, 1, 9);
        assert_eq!(fn_0098a340(&mut e, group, 1), 9);
        assert_eq!(fn_0098a340(&mut e, group, 0), 0);
        assert_eq!(fn_0098a310(&mut e, group, b), 9);
        assert_eq!(fn_0098a310(&mut e, group, stranger), 0);
        fn_0098a360(&mut e, group, a, 4);
        fn_0098a360(&mut e, group, stranger, 5);
        assert_eq!(fn_0098a340(&mut e, group, 0), 4);
        assert_eq!(
            combat_group_find_member_with_strategy_assignment(&mut e, group, 9),
            b
        );
        assert!(combat_group_find_member_with_strategy_assignment(&mut e, group, 6).is_null());
        fn_0098a3c0(&mut e, group);
        assert_eq!(fn_0098a340(&mut e, group, 0), 0);
        assert_eq!(fn_0098a340(&mut e, group, 1), 0);
    }

    #[test]
    fn strategy_chosen_stamp_check() {
        let mut e = engine();
        let group = e.new_object::<CombatGroup>();
        // No stamp yet: true.
        assert!(combat_group_check_group_strategy_chosen_time_stamp(
            &mut e, group, 2, 1.0
        ));
        e.set(
            group,
            CombatGroup::fLastGroupStrategyChosenTimeStamp,
            NOW - 10.0,
        );
        e.set(group, CombatGroup::iLastGroupStrategyChosenIndex, 2);
        // Another strategy: true.
        assert!(combat_group_check_group_strategy_chosen_time_stamp(
            &mut e, group, 3, 100.0
        ));
        // The same strategy: true only when the limit is below the age.
        assert!(!combat_group_check_group_strategy_chosen_time_stamp(
            &mut e, group, 2, 10.0
        ));
        assert!(combat_group_check_group_strategy_chosen_time_stamp(
            &mut e, group, 2, 9.0
        ));
    }

    #[test]
    fn average_of_value_over_divisor_of_members() {
        let mut e = engine();
        // The object with the two virtual methods is at +0xa4 of each
        // member actor.
        let mut members = vec![];
        for _ in 0..2 {
            let big = e.mem.alloc(0x100);
            e.mem.set_u32(big, ACTOR_VTABLE);
            e.mem.set_u32(big + 0xa4, 0x7200_0000);
            members.push(Ptr::new(big));
        }
        e.put_vtable(0x7200_0000, &[fake(0x500), 0, 0, fake(0x50c)]);
        e.register(fake(0x500), |e, a| {
            assert_eq!(a[1], 0x10);
            e.mem.u32(a[0] - 0xa4 + 0x50).into_ret()
        });
        e.register(fake(0x50c), |e, a| {
            assert_eq!(a[1], 0x10);
            e.mem.f32(a[0] - 0xa4 + 0x54).into_ret()
        });
        // Member 0: 30 / 10 = 3; member 1: 5 / 2 = 2.5; average 2.75.
        e.mem.set_u32(members[0].addr() + 0x50, 10);
        e.mem.set_f32(members[0].addr() + 0x54, 30.0);
        e.mem.set_u32(members[1].addr() + 0x50, 2);
        e.mem.set_f32(members[1].addr() + 0x54, 5.0);
        let group = group_with(&mut e, &[], &members);
        assert_eq!(fn_0098a4d0(&mut e, group), 2.75);
        let empty = group_with(&mut e, &[], &[]);
        assert!(fn_0098a4d0(&mut e, empty).is_nan());
    }

    #[test]
    fn member_health_percentage_is_cached() {
        let mut e = Engine::new();
        e.map(ZERO_DOUBLE, 8);
        e.mem.set_f64(ZERO_DOUBLE, 0.0);
        let group = e.new_object::<CombatGroup>();
        let data = e.new_object::<CombatGroupData>();
        e.set(data, CombatGroupData::pCombatGroup, group);
        e.set(data, CombatGroupData::fMemberHealthPercentage, 0.5);
        // Cached: the group is not touched.
        assert_eq!(fn_00990500(&mut e, data), 0.5);
        // Zero: computed from the (empty) group, 0/0, and stored.
        e.set(data, CombatGroupData::fMemberHealthPercentage, 0.0);
        e.register(MEMBER_COUNT, |_, _| 0u32.into_ret());
        assert!(fn_00990500(&mut e, data).is_nan());
        assert!(e
            .get(data, CombatGroupData::fMemberHealthPercentage)
            .is_nan());
    }

    // ---- search, strength, cluster and save tests ----

    /// Where the fake `GetWorldLocation` reads a fake actor's location:
    /// four words at +0x60.
    const ACTOR_LOCATION: u32 = 0x60;

    /// `engine()` plus the pages and fakes the search code needs: the
    /// arrays of the group (`SearchLocations` at +0x110, 0x1c-byte elements;
    /// `SearchTeleportDoors` at +0x120, 12-byte elements; buffer at +4 and
    /// size at +8 of each), `NiPointer` getter, points, distances.
    fn search_engine() -> Engine {
        let mut e = engine();
        add_search_fakes(&mut e);
        e
    }

    /// The pages and fakes of `search_engine`, for an engine built another way.
    fn add_search_fakes(e: &mut Engine) {
        for page in [
            0x011d_e000u32,
            0x0102_e000,
            0x0101_7000,
            0x0106_d000,
            0x0107_3000,
            0x0108_c000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(PLAYER, 0x7300_0000u32);
        e.register(SEARCH_LOCATION_COUNT, |e, a| {
            e.mem.u32(a[0] + 0x118).into_ret()
        });
        e.register(SEARCH_LOCATION_AT, |e, a| {
            (e.mem.u32(a[0] + 0x114) + a[1] * 0x1c).into_ret()
        });
        e.register(SEARCH_LOCATION_ELEMENT, |e, a| {
            (e.mem.u32(a[0] + 4) + a[1] * 0x1c).into_ret()
        });
        e.register(SEARCH_LOCATION_SLOT, |e, a| {
            (e.mem.u32(a[0] + 4) + a[1] * 0x1c).into_ret()
        });
        e.register(SEARCH_LOCATION_ADD, |e, a| {
            let size = e.mem.u32(a[0] + 8);
            e.mem.set_u32(a[0] + 8, size + 1);
            size.into_ret()
        });
        e.register(SEARCH_LOCATION_REMOVE, |e, a| {
            let (buffer, size) = (e.mem.u32(a[0] + 4), e.mem.u32(a[0] + 8));
            for entry in a[1]..size - a[2] {
                for word in 0..7 {
                    let value = e.mem.u32(buffer + (entry + a[2]) * 0x1c + 4 * word);
                    e.mem.set_u32(buffer + entry * 0x1c + 4 * word, value);
                }
            }
            e.mem.set_u32(a[0] + 8, size - a[2]);
            Ret::default()
        });
        e.register(MEMBER_OF_GROUP, |e, a| {
            (e.mem.u32(a[0] + 0x1c) + a[1] * 0x14).into_ret()
        });
        e.register(SEARCH_DOOR_ELEMENT, |e, a| {
            (e.mem.u32(a[0] + 4) + a[1] * 12).into_ret()
        });
        e.register(SEARCH_DOOR_AT, |e, a| {
            (e.mem.u32(a[0] + 4) + a[1] * 12).into_ret()
        });
        e.register(SEARCH_DOOR_ADD, |e, a| {
            let size = e.mem.u32(a[0] + 8);
            e.mem.set_u32(a[0] + 8, size + 1);
            size.into_ret()
        });
        e.register(NI_POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(POINT_NO_OP, |_, a| a[0].into_ret());
        e.register(FORM_TYPE_BYTE, |e, a| e.mem.u8(a[0] + 4).into_ret());
        e.register(DISTANCE_SQUARED, |e, a| {
            let d = |i: u32| e.mem.f32(a[0] + 4 * i) - e.mem.f32(a[1] + 4 * i);
            ((d(0) * d(0) + d(1) * d(1) + d(2) * d(2)) as f64).into_ret()
        });
        e.register(SQUARE_ROOT, |_, a| f32::from_bits(a[0]).sqrt().into_ret());
        e.register(POINT_SCALE, |e, a| {
            let factor = f32::from_bits(a[2]);
            for i in 0..3 {
                let value = e.mem.f32(a[0] + 4 * i) * factor;
                e.mem.set_f32(a[1] + 4 * i, value);
            }
            a[1].into_ret()
        });
        e.register(POINT_ACCUMULATE, |e, a| {
            for i in 0..3 {
                let value = e.mem.f32(a[0] + 4 * i) + e.mem.f32(a[1] + 4 * i);
                e.mem.set_f32(a[0] + 4 * i, value);
            }
            a[0].into_ret()
        });
        e.register(POINT_ADD, |e, a| {
            for i in 0..3 {
                let value = e.mem.f32(a[0] + 4 * i) + e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, value);
            }
            a[1].into_ret()
        });
        e.register(POINT_SUBTRACT, |e, a| {
            for i in 0..3 {
                let value = e.mem.f32(a[0] + 4 * i) - e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, value);
            }
            a[1].into_ret()
        });
        e.register(POINT_LENGTH_SQUARED, |e, a| {
            let c = |i: u32| e.mem.f32(a[0] + 4 * i);
            (c(0) * c(0) + c(1) * c(1) + c(2) * c(2)).into_ret()
        });
        e.register(POINT_DIVIDE_IN_PLACE, |e, a| {
            let factor = 1.0 / f32::from_bits(a[1]);
            for i in 0..3 {
                let value = e.mem.f32(a[0] + 4 * i) * factor;
                e.mem.set_f32(a[0] + 4 * i, value);
            }
            a[0].into_ret()
        });
        // The world location of a fake actor: four words at +0x60.
        e.register(GET_WORLD_LOCATION, |e, a| {
            for word in 0..4 {
                let value = e.mem.u32(a[0] + ACTOR_LOCATION + 4 * word);
                e.mem.set_u32(a[1] + 4 * word, value);
            }
            a[1].into_ret()
        });
        e.register(WORLD_LOCATION_CONSTRUCT, |e, a| {
            for word in 0..4 {
                e.mem.set_u32(a[0] + 4 * word, 0);
            }
            a[0].into_ret()
        });
    }

    /// Allocates the buffers behind the group's search arrays.
    fn give_search_buffers(e: &mut Engine, group: Ptr<CombatGroup>) {
        let locations = e.mem.alloc(0x1c * 16);
        e.mem.set_u32(group.addr() + 0x114, locations);
        let doors = e.mem.alloc(12 * 16);
        e.mem.set_u32(group.addr() + 0x124, doors);
    }

    /// Puts `locations` into the group's `SearchLocations`: (point, space,
    /// `fScore`, `iTargetID`); their time stamp is the current game time,
    /// so the remaining time is the score.
    fn put_locations(
        e: &mut Engine,
        group: Ptr<CombatGroup>,
        locations: &[([f32; 3], u32, f32, u32)],
    ) {
        give_search_buffers(e, group);
        let buffer = e.mem.u32(group.addr() + 0x114);
        for (i, (point, space, score, id)) in locations.iter().enumerate() {
            let at = buffer + 0x1c * i as u32;
            for (axis, value) in point.iter().enumerate() {
                e.mem.set_f32(at + 4 * axis as u32, *value);
            }
            e.mem.set_u32(at + 0x0c, *space);
            e.mem.set_f32(at + 0x10, NOW);
            e.mem.set_u32(at + 0x14, *id);
            e.mem.set_f32(at + 0x18, *score);
        }
        e.mem.set_u32(group.addr() + 0x118, locations.len() as u32);
    }

    /// Puts `doors` into the group's `SearchTeleportDoors`: (reference,
    /// linked reference, attempts, investigated, reserved).
    fn put_doors(e: &mut Engine, group: Ptr<CombatGroup>, doors: &[(u32, u32, u8, u8, u8)]) {
        if e.mem.u32(group.addr() + 0x124) == 0 {
            give_search_buffers(e, group);
        }
        let buffer = e.mem.u32(group.addr() + 0x124);
        for (i, (reference, linked, attempts, investigated, reserved)) in doors.iter().enumerate() {
            let at = buffer + 12 * i as u32;
            e.mem.set_u32(at, *reference);
            e.mem.set_u32(at + 4, *linked);
            e.mem.set_u8(at + 8, *attempts);
            e.mem.set_u8(at + 9, *investigated);
            e.mem.set_u8(at + 10, *reserved);
        }
        e.mem.set_u32(group.addr() + 0x128, doors.len() as u32);
    }

    fn door_bytes(e: &Engine, group: Ptr<CombatGroup>, index: u32) -> (u32, u32, u8, u8, u8) {
        let at = e.mem.u32(group.addr() + 0x124) + 12 * index;
        (
            e.mem.u32(at),
            e.mem.u32(at + 4),
            e.mem.u8(at + 8),
            e.mem.u8(at + 9),
            e.mem.u8(at + 10),
        )
    }

    fn point_at(e: &Engine, addr: u32) -> [f32; 3] {
        [e.mem.f32(addr), e.mem.f32(addr + 4), e.mem.f32(addr + 8)]
    }

    /// Sets the radius settings `fn_0098b070` reads for a location with no
    /// space (row 1) and a search count below 2 (column 0): `smaller` and
    /// `larger`.
    fn put_radius_settings(e: &mut Engine, smaller: f32, larger: f32) {
        e.set_global(0x011f_1100u32, smaller);
        e.set_global(0x011f_1104u32, larger);
        // Row 1 starts 12 bytes into the table; column 0 reads words 0 and 1.
        e.set_global(SEARCH_RADIUS_TABLE + 12, 0x011f_1100u32);
        e.set_global(SEARCH_RADIUS_TABLE + 16, 0x011f_1104u32);
        e.set_global(SEARCH_RADIUS_TABLE + 20, 0x011f_1104u32);
    }

    #[test]
    fn pathing_location_information_and_grid_value() {
        let mut e = engine();
        let location = e.mem.alloc(0x30);
        assert!(!fn_0098ad70(&mut e, Ptr::new(location)));
        e.mem.set_u32(location + 0x14, 1);
        assert!(e.call(0x0098_ad70, &args![location]).bool());
        e.mem.set_u32(location + 0x14, 0);
        e.mem.set_u32(location + 0x10, 7);
        assert!(fn_0098ad70(&mut e, Ptr::new(location)));
        let grid = e.mem.alloc(0x60);
        e.call(0x0098_adb0, &args![grid, 2.5f32]);
        assert_eq!(e.mem.f32(grid + 0x48), 2.5);
    }

    #[test]
    fn default_world_location_takes_the_default_point_and_no_space() {
        let mut e = engine();
        for (i, value) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.mem.set_f32(ZERO_POINT + 4 * i as u32, *value);
        }
        let location = e.mem.alloc(0x10);
        for word in 0..4 {
            e.mem.set_u32(location + 4 * word, 0xffff_ffff);
        }
        fn_0098bbc0(&mut e, Ptr::new(location));
        assert_eq!(point_at(&e, location), [1.0, 2.0, 3.0]);
        assert_eq!(e.mem.u32(location + 0x0c), 0);
    }

    #[test]
    fn radius_settings_depend_on_the_space_and_the_search_count() {
        let mut e = search_engine();
        let group = e.new_object::<CombatGroup>();
        // Table: row 0 (the 0x39 space) then row 1; two settings per
        // column, columns overlapping by one word.
        let values = [
            (0x011f_1100u32, 1.0f32),
            (0x011f_1104, 2.0),
            (0x011f_1108, 3.0),
            (0x011f_110c, 4.0),
            (0x011f_1110, 5.0),
            (0x011f_1114, 6.0),
        ];
        for (i, (address, value)) in values.iter().enumerate() {
            e.set_global(*address, *value);
            e.set_global(SEARCH_RADIUS_TABLE + 4 * i as u32, *address);
        }
        let outputs = e.mem.alloc(8);
        let space = e.mem.alloc(0x10);
        let location = e.mem.alloc(0x10);
        let read = |e: &Engine| (e.mem.f32(outputs), e.mem.f32(outputs + 4));
        // No space: row 1, column 0.
        fn_0098b070(
            &mut e,
            group,
            Ptr::new(location),
            Ptr::new(outputs),
            Ptr::new(outputs + 4),
        );
        assert_eq!(read(&e), (4.0, 5.0));
        // Search count 2: column 1.
        e.set(group, CombatGroup::iSearchCount, 2);
        e.call(0x0098_b070, &args![group, location, outputs, outputs + 4]);
        assert_eq!(read(&e), (5.0, 6.0));
        // A space whose type byte is 0x39: row 0.
        e.mem.set_u32(location + 0x0c, space);
        e.mem.set_u8(space + 4, 0x39);
        fn_0098b070(
            &mut e,
            group,
            Ptr::new(location),
            Ptr::new(outputs),
            Ptr::new(outputs + 4),
        );
        assert_eq!(read(&e), (2.0, 3.0));
        // Another type: row 1 again.
        e.mem.set_u8(space + 4, 0x10);
        e.set(group, CombatGroup::iSearchCount, 1);
        fn_0098b070(
            &mut e,
            group,
            Ptr::new(location),
            Ptr::new(outputs),
            Ptr::new(outputs + 4),
        );
        assert_eq!(read(&e), (4.0, 5.0));
    }

    #[test]
    fn search_door_lookup_and_state_bytes() {
        let mut e = search_engine();
        let group = e.new_object::<CombatGroup>();
        put_doors(&mut e, group, &[(10, 20, 0, 0, 0), (30, 40, 255, 0, 1)]);
        let first = combat_group_get_combat_search_door(&mut e, group, 20);
        assert_eq!(first.addr(), e.mem.u32(group.addr() + 0x124));
        let second = e.call(0x0098_c590, &args![group, 30u32]).ptr::<()>();
        assert_eq!(second.addr(), first.addr() + 12);
        assert!(combat_group_get_combat_search_door(&mut e, group, 99).is_null());
        // Bytes: +9 set by 0098c6e0, +0xa by 0098c750 and cleared by
        // UnreserveSearchDoor, +8 counted by FailedToInvestigate.
        fn_0098c6e0(&mut e, group, 10);
        assert_eq!(door_bytes(&e, group, 0), (10, 20, 0, 1, 0));
        fn_0098c750(&mut e, group, 20);
        assert_eq!(door_bytes(&e, group, 0).4, 1);
        combat_group_unreserve_search_door(&mut e, group, 10);
        assert_eq!(door_bytes(&e, group, 0).4, 0);
        combat_group_failed_to_investigate_search_door(&mut e, group, 40);
        assert_eq!(door_bytes(&e, group, 1), (30, 40, 0, 0, 0));
        combat_group_failed_to_investigate_search_door(&mut e, group, 30);
        assert_eq!(door_bytes(&e, group, 1).2, 1);
        // An unknown reference changes nothing.
        fn_0098c6e0(&mut e, group, 99);
        combat_group_unreserve_search_door(&mut e, group, 99);
        assert_eq!(door_bytes(&e, group, 0), (10, 20, 0, 1, 0));
    }

    #[test]
    fn adding_a_search_door_needs_a_teleport_reference_with_a_link() {
        let mut e = search_engine();
        let group = e.new_object::<CombatGroup>();
        put_doors(&mut e, group, &[(10, 20, 1, 1, 1)]);
        // Door 50 has a teleport reference whose pointer is 0x777; door 60's
        // reference is null; door 70's pointer is null.
        let linked = e.mem.alloc(8);
        e.mem.set_u32(linked, 0x777);
        let unlinked = e.mem.alloc(8);
        e.register_double(DOOR_TELEPORT_REFERENCE, move |_, a| match a[0] {
            50 => linked.into_ret(),
            70 => unlinked.into_ret(),
            _ => Ret::default(),
        });
        fn_0098c600(&mut e, group, 10);
        assert_eq!(e.mem.u32(group.addr() + 0x128), 1);
        fn_0098c600(&mut e, group, 60);
        fn_0098c600(&mut e, group, 70);
        assert_eq!(e.mem.u32(group.addr() + 0x128), 1);
        e.mem.set_u8(e.mem.u32(group.addr() + 0x124) + 12 + 8, 9);
        e.call(0x0098_c600, &args![group, 50u32]);
        assert_eq!(e.mem.u32(group.addr() + 0x128), 2);
        assert_eq!(door_bytes(&e, group, 1), (50, 0x777, 0, 0, 0));
    }

    #[test]
    fn search_ignore_locations_collects_other_members_goals() {
        let mut e = search_engine();
        let skipped = actor(&mut e, 1);
        let a = actor(&mut e, 2);
        let b = actor(&mut e, 3);
        let c = actor(&mut e, 4);
        // Processes: a and b have one (the 0x38 word), c has none.
        e.mem.set_u32(skipped.addr() + 0x38, 0x5000);
        e.mem.set_u32(a.addr() + 0x38, 0x5001);
        e.mem.set_u32(b.addr() + 0x38, 0x5002);
        // The process test passes for 0x5001 only; the goal exists for a.
        e.register(PROCESS_TEST_00981420, |_, a| (a[0] == 0x5001).into_ret());
        e.register(ACTOR_PATHFINDING_GOAL, |_, a| (a[0] != 0).into_ret());
        record(&mut e, &[PATH_POINT_ARRAY_ADD]);
        let array = e.mem.alloc(0x10);
        let group = group_with(&mut e, &[], &[skipped, a, b, c]);
        assert!(combat_group_calculate_search_ignore_locations(
            &mut e,
            group,
            skipped,
            Ptr::new(array)
        ));
        let adds = calls(&e, PATH_POINT_ARRAY_ADD);
        assert_eq!(adds.len(), 1);
        assert_eq!(adds[0][0], array);
        // One member: nothing to do.
        let single = group_with(&mut e, &[], &[a]);
        assert!(!combat_group_calculate_search_ignore_locations(
            &mut e,
            single,
            skipped,
            Ptr::new(array)
        ));
        assert_eq!(calls(&e, PATH_POINT_ARRAY_ADD).len(), 1);
    }

    #[test]
    fn adding_a_search_location_copies_the_location_and_sets_the_fields() {
        let mut e = search_engine();
        let group = e.new_object::<CombatGroup>();
        give_search_buffers(&mut e, group);
        let location = e.mem.alloc(0x10);
        for (i, value) in [4.0f32, 5.0, 6.0].iter().enumerate() {
            e.mem.set_f32(location + 4 * i as u32, *value);
        }
        e.mem.set_u32(location + 0x0c, 0x4242);
        fn_0098bfb0(&mut e, group, Ptr::new(location), 12.5, 0x1234);
        assert_eq!(e.mem.u32(group.addr() + 0x118), 1);
        let entry = e.mem.u32(group.addr() + 0x114);
        assert_eq!(point_at(&e, entry), [4.0, 5.0, 6.0]);
        assert_eq!(e.mem.u32(entry + 0x0c), 0x4242);
        assert_eq!(e.mem.f32(entry + 0x10), NOW);
        assert_eq!(e.mem.u32(entry + 0x14), 0x1234);
        assert_eq!(e.mem.f32(entry + 0x18), 12.5);
    }

    #[test]
    fn target_time_stamp_excesses() {
        let mut e = search_engine();
        let group = e.new_object::<CombatGroup>();
        let target = e.new_object::<CombatTarget>();
        // Ages 10, 20 and 30 (game time 100).
        e.set(target, CombatTarget::fLastDetectedTimeStamp, 90.0f32);
        e.set(target, CombatTarget::fLastNoticedTimeStamp, 80.0f32);
        e.set(target, CombatTarget::fLastAttackedMemberTimeStamp, 70.0f32);
        e.set_global(0x011c_e528u32, 5.0f32);
        e.mem.set_f64(0x0107_3568, 70.0);
        e.mem.set_f64(0x0101_db88, 30.0);
        let outputs = e.mem.alloc(12);
        fn_0098c060(
            &mut e,
            group,
            target,
            Ptr::new(outputs),
            Ptr::new(outputs + 4),
            Ptr::new(outputs + 8),
        );
        // 5 + (70 - 10), 30 - 20, 70 - 30.
        assert_eq!(point_at(&e, outputs), [65.0, 10.0, 40.0]);
    }

    /// Registers all the virtual methods of the fake actors the search and
    /// strength tests use: besides `engine()`'s, 0x21c (byte at +0x3c), 0x2bc
    /// (`float` at +0x48), 0x42c (word at +0x4c).
    fn fuller_actor_vtable(e: &mut Engine) {
        let mut slots = vec![0u32; 0x120];
        for slot in [
            0x130u32, 0x1d0, 0x1f4, 0x21c, 0x22c, 0x230, 0x2bc, 0x2e8, 0x3fc, 0x428, 0x42c,
        ] {
            slots[(slot / 4) as usize] = fake(slot);
        }
        e.put_vtable(ACTOR_VTABLE, &slots);
        e.register(fake(0x21c), |e, a| e.mem.u8(a[0] + 0x3c).into_ret());
        e.register(fake(0x2bc), |e, a| e.mem.f32(a[0] + 0x48).into_ret());
        e.register(fake(0x42c), |e, a| e.mem.u32(a[0] + 0x4c).into_ret());
    }

    /// A block that stands for a process: +0 damage per second, +4 combat
    /// strength (both `float`), +8 flags word (bit 3 initialised), +0xc
    /// fleeing byte.
    fn process_block(e: &mut Engine, dps: f32, strength: f32, flags: u32, fleeing: bool) -> u32 {
        let block = e.mem.alloc(0x10);
        e.mem.set_f32(block, dps);
        e.mem.set_f32(block + 4, strength);
        e.mem.set_u32(block + 8, flags);
        e.mem.set_u8(block + 0x0c, fleeing as u8);
        block
    }

    /// Doubles for the combat-state accessors over `process_block`.
    fn add_process_fakes(e: &mut Engine) {
        e.register(PROCESS_COMBAT_STATE, |_, a| a[0].into_ret());
        e.register(STATE_DAMAGE_PER_SECOND, |e, a| e.mem.f32(a[0]).into_ret());
        e.register(STATE_COMBAT_STRENGTH, |e, a| e.mem.f32(a[0] + 4).into_ret());
        e.register(CONTROLLER_IS_FLEEING, |e, a| {
            e.mem.u8(a[0] + 0x0c).into_ret()
        });
        e.register(PROCESS_FLAG_BIT_3, |e, a| {
            (e.mem.u32(a[0] + 8) & 8 != 0).into_ret()
        });
        // `CalculateCombatStrength(actor, damage)`: twice the damage, or 7
        // for the -1 the target code passes.
        e.register(ACTOR_COMBAT_STRENGTH, |_, a| {
            let damage = f32::from_bits(a[1]);
            (if damage < 0.0 { 7.0 } else { damage * 2.0 }).into_ret()
        });
        e.register(ACTOR_WEAPON_DAMAGE_PER_SECOND, |_, _| 10.0f32.into_ret());
        e.register(ACTOR_IS_FLEEING_ARG, |e, a| {
            e.mem.u8(a[0] + 0x3e).into_ret()
        });
    }

    fn member_entry(e: &Engine, group: Ptr<CombatGroup>, index: u32) -> Ptr<CombatMember> {
        Ptr::new(e.mem.u32(group.addr() + 0x1c) + 0x14 * index)
    }

    /// Sets the word at +0x38 of a fake actor (its process), +0x44 (its
    /// process type) and its location (+0x60: x, y, z, space).
    fn place_actor(e: &mut Engine, actor: Ptr, process: u32, x: f32, space: u32) {
        e.mem.set_u32(actor.addr() + 0x38, process);
        e.mem.set_f32(actor.addr() + ACTOR_LOCATION, x);
        e.mem.set_u32(actor.addr() + ACTOR_LOCATION + 0x0c, space);
        // The position that virtual method 0x1f4 returns is +0x40.
        e.mem.set_f32(actor.addr() + 0x40, x);
    }

    #[test]
    fn search_center_from_locations() {
        let mut e = search_engine();
        let space = e.mem.alloc(8);
        let group = e.new_object::<CombatGroup>();
        put_radius_settings(&mut e, 20.0, 100.0);
        e.set_global(0x011c_e7b0u32, 20.0f32);
        let first = space;
        put_locations(
            &mut e,
            group,
            &[
                ([0.0, 0.0, 0.0], first, 40.0, 0),
                ([10.0, 0.0, 0.0], first, 20.0, 0),
            ],
        );
        fn_0098b100(&mut e, group);
        // The center is the average weighted by the remaining times.
        let factor = 1.0f32 / 60.0;
        let centre = point_at(&e, group.addr() + 0xf0);
        assert!((centre[0] - 200.0 * factor).abs() < 1e-5);
        assert_eq!(e.mem.u32(group.addr() + 0xfc), first);
        assert_eq!(point_at(&e, group.addr() + 0x100), [0.0, 0.0, 0.0]);
        // Radius: 20 + the root of the largest squared distance (6.67^2),
        // inside [20, 100].
        let radius = e.get(group, CombatGroup::fSearchRadius);
        let farthest = (10.0f32 - 200.0 * factor).powi(2);
        assert!((radius - (20.0 + farthest.sqrt())).abs() < 1e-3);
        // A far location beyond the larger radius caps the radius at it.
        put_radius_settings(&mut e, 1.0, 5.0);
        fn_0098b100(&mut e, group);
        assert_eq!(point_at(&e, group.addr() + 0xf0), [0.0, 0.0, 0.0]);
        assert_eq!(e.get(group, CombatGroup::fSearchRadius), 5.0);
        // One location: it is the center, the radius the smaller setting.
        let single = e.new_object::<CombatGroup>();
        put_locations(&mut e, single, &[([3.0, 4.0, 5.0], first, 9.0, 0)]);
        e.call(0x0098_b100, &args![single]);
        assert_eq!(point_at(&e, single.addr() + 0xf0), [3.0, 4.0, 5.0]);
        assert_eq!(e.mem.u32(single.addr() + 0xfc), first);
        assert_eq!(point_at(&e, single.addr() + 0x100), [3.0, 4.0, 5.0]);
        assert_eq!(e.get(single, CombatGroup::fSearchRadius), 1.0);
    }

    #[test]
    fn search_center_from_members() {
        let mut e = search_engine();
        e.register(PROCESS_TYPE, |e, a| e.mem.i32(a[0] + 0x44).into_ret());
        let space = e.mem.alloc(8);
        put_radius_settings(&mut e, 20.0, 100.0);
        let (a, b, c) = (actor(&mut e, 1), actor(&mut e, 2), actor(&mut e, 3));
        for (who, x, kind) in [(a, 10.0f32, 4i32), (b, 0.0, 2), (c, 300.0, 3)] {
            place_actor(&mut e, who, 0, x, space);
            e.mem.set_i32(who.addr() + 0x44, kind);
        }
        let group = group_with(&mut e, &[], &[a, b, c]);
        fn_0098b100(&mut e, group);
        // Best member b (lowest type); a and b within 100 of it.
        assert_eq!(point_at(&e, group.addr() + 0xf0), [5.0, 0.0, 0.0]);
        assert_eq!(e.mem.u32(group.addr() + 0xfc), space);
        assert_eq!(point_at(&e, group.addr() + 0x100), [0.0, 0.0, 0.0]);
        // c lies beyond the larger radius: the radius is the larger setting.
        assert_eq!(e.get(group, CombatGroup::fSearchRadius), 100.0);
        // A single member: its location, radius the smaller setting.
        let single = group_with(&mut e, &[], &[a]);
        fn_0098b100(&mut e, single);
        assert_eq!(point_at(&e, single.addr() + 0xf0), [10.0, 0.0, 0.0]);
        assert_eq!(e.mem.u32(single.addr() + 0xfc), space);
        assert_eq!(e.get(single, CombatGroup::fSearchRadius), 20.0);
        // With the center's space already set nothing is recomputed.
        e.mem.set_u32(single.addr() + 0xfc, 0x1234);
        e.set(single, CombatGroup::fSearchRadius, 3.0f32);
        fn_0098b100(&mut e, single);
        assert_eq!(e.get(single, CombatGroup::fSearchRadius), 3.0);
    }

    #[test]
    fn search_area_moves_when_the_radius_or_the_center_changed_enough() {
        let mut e = search_engine();
        let space = e.mem.alloc(8);
        put_radius_settings(&mut e, 20.0, 100.0);
        e.set_global(SEARCH_AREA_MARGIN, 128.0f64);
        e.set_global(SEARCH_AREA_MOVE_LIMIT, 4096.0f64);
        record(
            &mut e,
            &[
                POINTER_ARRAY_CONSTRUCT,
                POINTER_ARRAY_DESTRUCT,
                COLLECT_TELEPORT_DOORS,
                ARRAY_SORT,
            ],
        );
        let group = e.new_object::<CombatGroup>();
        put_locations(&mut e, group, &[([0.0, 0.0, 0.0], space, 9.0, 0)]);
        // The new area is center (0, 0, 0), radius 20.
        e.mem.set_f32(group.addr() + 0xf0, 0.0);
        e.set(group, CombatGroup::fSearchRadius, 20.0f32);
        assert!(!combat_group_update_search_area(&mut e, group));
        assert!(calls(&e, POINTER_ARRAY_CONSTRUCT).is_empty());
        // Radius far above the new one.
        e.set(group, CombatGroup::fSearchRadius, 149.0f32);
        assert!(e.call(0x0098_afb0, &args![group]).bool());
        assert_eq!(calls(&e, POINTER_ARRAY_CONSTRUCT).len(), 1);
        // The radius is back to 20; a center 70 away (4900 > 4096) moves.
        e.mem.set_f32(group.addr() + 0xf0, 70.0);
        assert!(combat_group_update_search_area(&mut e, group));
        // 60 away (3600) does not; a radius 100 below does not either.
        e.mem.set_f32(group.addr() + 0xf0, 60.0);
        e.set(group, CombatGroup::fSearchRadius, 0.0f32);
        assert!(!combat_group_update_search_area(&mut e, group));
        assert_eq!(calls(&e, POINTER_ARRAY_CONSTRUCT).len(), 2);
    }

    #[test]
    fn teleport_door_array_collects_sorts_dedupes_and_adds_new_doors() {
        let mut e = search_engine();
        let space = e.mem.alloc(8);
        put_radius_settings(&mut e, 10.0, 3.0);
        // The array the game keeps on its stack: buffer at +4, size at +8.
        e.register(POINTER_ARRAY_CONSTRUCT, |e, a| {
            let buffer = e.mem.alloc(0x100);
            e.mem.set_u32(a[0] + 4, buffer);
            e.mem.set_u32(a[0] + 8, 0);
            a[0].into_ret()
        });
        e.register(POINTER_ARRAY_DESTRUCT, |_, _| Ret::default());
        // The first collection (the center) finds 7, 3, 7; the next one (a
        // member beyond the larger radius) finds 3, 9.
        let mut batches = vec![vec![3u32, 9], vec![7, 3, 7]];
        e.register_double(COLLECT_TELEPORT_DOORS, move |e, a| {
            let array = a[3];
            let doors = batches.pop().unwrap_or_default();
            for door in doors {
                let size = e.mem.u32(array + 8);
                let buffer = e.mem.u32(array + 4);
                e.mem.set_u32(buffer + 4 * size, door);
                e.mem.set_u32(array + 8, size + 1);
            }
            Ret::default()
        });
        e.register(ARRAY_SORT, |e, a| {
            let (buffer, size) = (e.mem.u32(a[0] + 4), e.mem.u32(a[0] + 8));
            let mut values: Vec<u32> = (0..size).map(|i| e.mem.u32(buffer + 4 * i)).collect();
            values.sort_unstable();
            for (i, value) in values.iter().enumerate() {
                e.mem.set_u32(buffer + 4 * i as u32, *value);
            }
            Ret::default()
        });
        e.register(POINTER_ARRAY_REMOVE_RANGE, |e, a| {
            let (buffer, size) = (e.mem.u32(a[0] + 4), e.mem.u32(a[0] + 8));
            for i in a[1]..size - a[2] {
                let value = e.mem.u32(buffer + 4 * (i + a[2]));
                e.mem.set_u32(buffer + 4 * i, value);
            }
            e.mem.set_u32(a[0] + 8, size - a[2]);
            Ret::default()
        });
        // GetDistance: the first coordinate difference.
        e.register(WORLD_LOCATION_DISTANCE, |e, a| {
            ((e.mem.f32(a[1]) - e.mem.f32(a[0])).abs() as f64).into_ret()
        });
        e.register(DOOR_TELEPORT_REFERENCE, |_, _| Ret::default());
        let (near, far) = (actor(&mut e, 1), actor(&mut e, 2));
        // Larger setting 3 -> its square 9: the near member (distance 4)
        // is not below 9?  4 < 9, so it is skipped; the far one (20) counts.
        place_actor(&mut e, near, 0, 4.0, space);
        place_actor(&mut e, far, 0, 20.0, space);
        let group = group_with(&mut e, &[], &[near, far]);
        e.mem.set_u32(group.addr() + 0xfc, space);
        e.set(group, CombatGroup::fSearchRadius, 50.0f32);
        e.call_log = Some(vec![]);
        combat_group_build_search_teleport_door_array(&mut e, group);
        let collections = calls(&e, COLLECT_TELEPORT_DOORS);
        assert_eq!(collections.len(), 2);
        // (space, point, radius, array): the center with the group's radius,
        // then the far member with the smaller setting.
        assert_eq!(collections[0][0], space);
        assert_eq!(f32::from_bits(collections[0][2]), 50.0);
        assert_eq!(f32::from_bits(collections[1][2]), 10.0);
        // Sorted 3, 3, 7, 7, 9 -> 3, 7, 9; each added through 0098c600,
        // which asks for its teleport reference.
        let asked: Vec<u32> = calls(&e, DOOR_TELEPORT_REFERENCE)
            .iter()
            .map(|c| c[0])
            .collect();
        assert_eq!(asked, vec![3, 7, 9]);
        assert_eq!(calls(&e, POINTER_ARRAY_REMOVE_RANGE).len(), 2);
    }

    #[test]
    fn ending_the_search_resets_everything() {
        let mut e = search_engine();
        record(&mut e, &[NI_POINTER_ASSIGN, TIMER_RESET, ARRAY_CLEAR]);
        let group = e.new_object::<CombatGroup>();
        e.set(group, CombatGroup::bUpdateSearchDebugGeometry, true);
        e.set(group, CombatGroup::iSearchCount, 4);
        e.set(group, CombatGroup::fSearchRadius, 9.0f32);
        e.set(group, CombatGroup::fSearchStartedTimeStamp, 55.0f32);
        for (i, value) in [7.0f32, 8.0, 9.0].iter().enumerate() {
            e.mem.set_f32(group.addr() + 0xf0 + 4 * i as u32, *value);
            e.mem.set_f32(group.addr() + 0x100 + 4 * i as u32, *value);
        }
        e.mem.set_u32(group.addr() + 0xfc, 0x99);
        for (i, value) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.mem.set_f32(ZERO_POINT + 4 * i as u32, *value);
        }
        e.call_log = Some(vec![]);
        fn_0098bae0(&mut e, group);
        assert!(!e.get(group, CombatGroup::bUpdateSearchDebugGeometry));
        assert_eq!(e.get(group, CombatGroup::iSearchCount), 0);
        assert_eq!(e.get(group, CombatGroup::fSearchRadius), 0.0);
        assert_eq!(
            e.get(group, CombatGroup::fSearchStartedTimeStamp),
            -f32::MAX
        );
        assert_eq!(point_at(&e, group.addr() + 0xf0), [1.0, 2.0, 3.0]);
        assert_eq!(e.mem.u32(group.addr() + 0xfc), 0);
        assert_eq!(point_at(&e, group.addr() + 0x100), [1.0, 2.0, 3.0]);
        assert_eq!(
            calls(&e, NI_POINTER_ASSIGN),
            vec![vec![group.addr() + 0xd4, 0], vec![group.addr() + 0x130, 0]]
        );
        assert_eq!(
            calls(&e, TIMER_RESET),
            vec![vec![group.addr() + 0xd8], vec![group.addr() + 0xe0]]
        );
        assert_eq!(
            calls(&e, ARRAY_CLEAR),
            vec![vec![group.addr() + 0x110, 1], vec![group.addr() + 0x120, 1]]
        );
        e.call(0x0098_bae0, &args![group]);
    }

    #[test]
    fn best_search_location_for_an_actor() {
        let mut e = search_engine();
        let space = e.mem.alloc(8);
        e.set_global(0x0108_d5a0u32, 1000.0f32);
        e.set_global(0x0108_d598u32, 1.0f64);
        e.set_global(0x0102_0758u32, 10.0f64);
        e.set_global(FIVE_DOUBLE, 5.0f64);
        e.register(POINTS_EQUAL, |e, a| {
            (0..3)
                .all(|i| e.mem.f32(a[0] + 4 * i) == e.mem.f32(a[1] + 4 * i))
                .into_ret()
        });
        let who = actor(&mut e, 1);
        place_actor(&mut e, who, 0, 0.0, space);
        let group = e.new_object::<CombatGroup>();
        // Scores 8, 9, 12, 12, 6; the second shares the reference's space
        // and point, which adds 5 to its qualification.
        let scores = [8.0f32, 9.0, 12.0, 12.0, 6.0];
        let list: Vec<([f32; 3], u32, f32, u32)> = scores
            .iter()
            .enumerate()
            .map(|(i, s)| ([i as f32, 0.0, 0.0], space, *s, 0))
            .collect();
        put_locations(&mut e, group, &list);
        let reference = e.mem.alloc(0x10);
        e.mem.set_f32(reference, 1.0);
        e.mem.set_u32(reference + 0x0c, space);
        let entries: Vec<u32> = (0..5)
            .map(|i| e.mem.u32(group.addr() + 0x114) + 0x1c * i)
            .collect();
        // Threshold 10: entries 1 (9 + 5), 2 and 3 qualify; ties go to the
        // later one.
        let found = combat_group_find_search_location(&mut e, group, who, 10, Ptr::new(reference));
        assert_eq!(found.addr(), entries[3]);
        // Without the reference only entries 2 and 3 qualify.
        let found = e
            .call(0x0098_bc00, &args![group, who, 10u32, 0u32])
            .ptr::<()>();
        assert_eq!(found.addr(), entries[3]);
        // With a process whose combat state refuses the last one.
        let process = 0x5555u32;
        e.mem.set_u32(who.addr() + 0x38, process);
        e.register(PROCESS_COMBAT_STATE, |_, a| (a[0] + 1).into_ret());
        let refused = entries[3];
        e.register_double(COMBAT_STATE_CHECK_MOVEMENT, move |_, a| {
            (a[1] != refused).into_ret()
        });
        let found = combat_group_find_search_location(&mut e, group, who, 10, Ptr::NULL);
        assert_eq!(found.addr(), entries[2]);
        // A higher threshold than any score: nothing.
        let found = combat_group_find_search_location(&mut e, group, who, 200, Ptr::NULL);
        assert!(found.is_null());
    }

    #[test]
    fn ageing_search_locations_and_target_time_stamps_near_a_position() {
        let mut e = search_engine();
        e.set_global(0x011c_f684u32, 10.0f32);
        e.set_global(0x011c_e528u32, 0.0f32);
        e.mem.set_f64(0x0107_3568, 70.0);
        e.mem.set_f64(0x0101_db88, 30.0);
        e.set_global(QUARTER_FLOAT, 0.25f32);
        record(&mut e, &[POINT_CONSTRUCT]);
        // The scaled distance of a location: its first coordinate.
        e.register(WORLD_LOCATION_SCALED_DISTANCE, |e, a| {
            (e.mem.f32(a[0]) as f64).into_ret()
        });
        let (t1, t2) = (actor(&mut e, 0x77), actor(&mut e, 0x78));
        let group = group_with(&mut e, &[t1, t2], &[]);
        put_locations(
            &mut e,
            group,
            &[
                ([50.0, 0.0, 0.0], 0, 40.0, 0),
                ([500.0, 0.0, 0.0], 0, 99.0, 0),
                ([10.0, 0.0, 0.0], 0, 20.0, 0x77),
                ([1.0, 0.0, 0.0], 0, 20.0, 0x78),
            ],
        );
        // t1: detected stamp 90 (age 10 -> excess 60), noticed 100 (30),
        // attacked 50 (20); its detected location is close.
        let first = target_entry(&e, group, 0);
        e.set(first, CombatTarget::fLastDetectedTimeStamp, 90.0f32);
        e.set(first, CombatTarget::fLastNoticedTimeStamp, 100.0f32);
        e.set(first, CombatTarget::fLastAttackedMemberTimeStamp, 50.0f32);
        e.mem.set_f32(first.addr() + 0x18, 20.0);
        // t2: detected excess 60 but far away; noticed stamp 110 (excess 40)
        // and close.
        let second = target_entry(&e, group, 1);
        e.set(second, CombatTarget::fLastDetectedTimeStamp, 90.0f32);
        e.set(second, CombatTarget::fLastNoticedTimeStamp, 110.0f32);
        e.set(second, CombatTarget::fLastAttackedMemberTimeStamp, 50.0f32);
        e.mem.set_f32(second.addr() + 0x18, 1000.0);
        e.mem.set_f32(second.addr() + 0x08, 5.0);
        let position = e.mem.alloc(0x10);
        fn_0098bd70(&mut e, group, Ptr::new(position));
        let buffer = e.mem.u32(group.addr() + 0x114);
        // 40 remaining -> score lowered by 10; the far one and the others
        // (not above 30) stay.
        assert_eq!(e.mem.f32(buffer + 0x18), 30.0);
        assert_eq!(e.mem.f32(buffer + 0x1c + 0x18), 99.0);
        assert_eq!(e.mem.f32(buffer + 2 * 0x1c + 0x18), 20.0);
        // t1's detected stamp moved back by the excess over 30.
        assert_eq!(e.get(first, CombatTarget::fLastDetectedTimeStamp), 60.0);
        assert_eq!(e.get(first, CombatTarget::fLastNoticedTimeStamp), 100.0);
        assert_eq!(
            e.get(first, CombatTarget::fLastAttackedMemberTimeStamp),
            50.0
        );
        // t2: only the noticed one (110 - (40 - 30)).
        assert_eq!(e.get(second, CombatTarget::fLastDetectedTimeStamp), 90.0);
        assert_eq!(e.get(second, CombatTarget::fLastNoticedTimeStamp), 100.0);
        // The scale given to the distance is (1, 1, 0.25).
        assert_eq!(calls(&e, POINT_CONSTRUCT).len(), 1);
        assert_eq!(f32::from_bits(calls(&e, POINT_CONSTRUCT)[0][3]), 0.25);
    }

    #[test]
    fn search_locations_are_rebuilt_from_the_targets_strongly() {
        let mut e = search_engine();
        e.mem.set_f64(0x0107_3568, 70.0);
        e.mem.set_f64(0x0101_db88, 30.0);
        e.set_global(0x011c_e528u32, 0.0f32);
        // BGSWorldLocation(point, space): the point and the space.
        e.register(WORLD_LOCATION_FROM_POINT, |e, a| {
            for i in 0..3 {
                let value = e.mem.f32(a[1] + 4 * i);
                e.mem.set_f32(a[0] + 4 * i, value);
            }
            e.mem.set_u32(a[0] + 0x0c, a[2]);
            a[0].into_ret()
        });
        let space_one = e.mem.alloc(8);
        let space_two = e.mem.alloc(8);
        let actors: Vec<Ptr> = (0..5).map(|i| actor(&mut e, 0x11 + i)).collect();
        let group = group_with(&mut e, &actors, &[]);
        // Old entries: kept (score 5), negative score, with a target ID,
        // kept (score 9).
        put_locations(
            &mut e,
            group,
            &[
                ([1.0, 1.0, 1.0], 0, 5.0, 0),
                ([2.0, 2.0, 2.0], 0, -1.0, 0),
                ([3.0, 3.0, 3.0], 0, 9.0, 0x99),
                ([4.0, 4.0, 4.0], 0, 9.0, 0),
            ],
        );
        // (detected, noticed, attacked) stamps with the game time 100, so
        // the excesses are (70 - age, 30 - age, 70 - age):
        // t0: (50, 20, 70): b < a, c > a -> attacked location, score 70
        // t1: (60, 20, 60): b < a, c <= a -> detected location, score 60
        // t2: (30, 30, 30): b is not below -> noticed + attacked, same space
        // t3: (30, 30, 30), another space -> noticed location, score 30
        // t4: all negative: nothing
        let stamps = [
            (80.0f32, 90.0, 100.0),
            (90.0, 90.0, 90.0),
            (60.0, 100.0, 60.0),
            (60.0, 100.0, 60.0),
            (0.0, 0.0, 0.0),
        ];
        for (i, (detected, noticed, attacked)) in stamps.iter().enumerate() {
            let target = target_entry(&e, group, i as u32);
            e.set(target, CombatTarget::fLastDetectedTimeStamp, *detected);
            e.set(target, CombatTarget::fLastNoticedTimeStamp, *noticed);
            e.set(
                target,
                CombatTarget::fLastAttackedMemberTimeStamp,
                *attacked,
            );
            // Locations: detected +0x18, noticed +0x08, attacked +0x38
            // (x = 10 + 100 * kind + i).
            for (kind, offset) in [(1u32, 0x18u32), (0, 0x08), (2, 0x38)] {
                let at = target.addr() + offset;
                e.mem.set_f32(at, (10 + 100 * kind) as f32 + i as f32);
                e.mem.set_u32(
                    at + 0x0c,
                    if i == 3 && kind == 2 {
                        space_two
                    } else {
                        space_one
                    },
                );
            }
        }
        e.call_log = Some(vec![]);
        fn_0098c0d0(&mut e, group);
        let buffer = e.mem.u32(group.addr() + 0x114);
        assert_eq!(e.mem.u32(group.addr() + 0x118), 2 + 4);
        let entry = |i: u32| buffer + 0x1c * i;
        // The two kept ones first.
        assert_eq!(e.mem.f32(entry(0) + 0x18), 5.0);
        assert_eq!(e.mem.f32(entry(1) + 0x18), 9.0);
        assert_eq!(e.mem.u32(entry(1) + 0x14), 0);
        // t0 -> attacked location (x = 210), score 70, the actor's form ID.
        assert_eq!(e.mem.f32(entry(2)), 210.0);
        assert_eq!(e.mem.f32(entry(2) + 0x18), 70.0);
        assert_eq!(e.mem.u32(entry(2) + 0x14), 0x11);
        // t1 -> detected location (x = 111), score 60.
        assert_eq!(e.mem.f32(entry(3)), 111.0);
        assert_eq!(e.mem.f32(entry(3) + 0x18), 60.0);
        assert_eq!(e.mem.u32(entry(3) + 0x14), 0x12);
        // t2 -> the average of the noticed (12) and attacked (212) locations
        // weighted by 30 and 30, in the noticed location's space, score 30.
        let average = (212.0f32 * 30.0 + 12.0 * 30.0) * (1.0f32 / 60.0);
        assert!((e.mem.f32(entry(4)) - average).abs() < 1e-3);
        assert_eq!(e.mem.u32(entry(4) + 0x0c), space_one);
        assert_eq!(e.mem.f32(entry(4) + 0x18), 30.0);
        assert_eq!(e.mem.u32(entry(4) + 0x14), 0x13);
        // t3 (the attacked location is in another space) -> noticed.
        assert_eq!(e.mem.f32(entry(5)), 13.0);
        assert_eq!(e.mem.f32(entry(5) + 0x18), 30.0);
        assert_eq!(e.mem.u32(entry(5) + 0x14), 0x14);
    }

    #[test]
    fn nearest_search_door_of_an_actor() {
        let mut e = search_engine();
        fuller_actor_vtable(&mut e);
        e.set_global(0x011c_fc98u32, 5u32);
        let who = actor(&mut e, 1);
        place_actor(&mut e, who, 0, 0.0, 0);
        // Door references are fake actors with a location.
        let mut references = vec![];
        for (i, x) in [50.0f32, 10.0, 20.0, 30.0, 40.0, 5.0, 60.0]
            .iter()
            .enumerate()
        {
            let door = actor(&mut e, 100 + i as u32);
            place_actor(&mut e, door, 0, *x, 0);
            references.push(door.addr());
        }
        let group = e.new_object::<CombatGroup>();
        // 0: far-ish; 1: nearest; 2: no link; 3: reserved; 4: investigated;
        // 5: too many attempts; 6: no teleport reference.
        put_doors(
            &mut e,
            group,
            &[
                (references[0], 1, 0, 0, 0),
                (references[1], 1, 0, 0, 0),
                (references[2], 0, 0, 0, 0),
                (references[3], 1, 0, 0, 1),
                (references[4], 1, 0, 1, 0),
                (references[5], 1, 9, 0, 0),
                (references[6], 1, 0, 0, 0),
            ],
        );
        let no_teleport = references[6];
        e.register_double(DOOR_TELEPORT_REFERENCE, move |_, a| {
            (if a[0] == no_teleport { 0 } else { a[0] + 1 }).into_ret()
        });
        // The teleport location is marked with the teleport reference it
        // came from; the actor cannot exist in the nearest door's space.
        e.register(TELEPORT_WORLD_LOCATION, |e, a| {
            e.mem.set_u32(a[1], a[0]);
            a[1].into_ret()
        });
        let blocked = references[1] + 1;
        e.register_double(CAN_ACTOR_EXIST_IN_SPACE, move |e, a| {
            (e.mem.u32(a[1]) != blocked).into_ret()
        });
        let found = combat_group_find_search_door(&mut e, group, who, 100.0, 0);
        assert_eq!(found.addr(), references[1]);
        // Within a radius of 8 nothing qualifies (the nearest is 10 away).
        let found = e
            .call(0x0098_c3d0, &args![group, who, 8.0f32, 0u32])
            .ptr::<()>();
        assert!(found.is_null());
        // With the space check the nearest is out: the next one is chosen.
        let found = combat_group_find_search_door(&mut e, group, who, 100.0, 1);
        assert_eq!(found.addr(), references[0]);
        // An actor whose method 0x21c is true and whose extra object's
        // method 0x28 is true finds nothing.
        e.mem.set_u8(who.addr() + 0x3c, 1);
        let extra = e.mem.alloc(0x40);
        let mut slots = vec![0u32; 16];
        slots[10] = fake(0x28);
        e.put_vtable(0x7210_0000, &slots);
        e.mem.set_u32(extra + 0x30, 0x7210_0000);
        e.register(fake(0x28), |_, _| true.into_ret());
        e.register_double(ACTOR_EXTRA_OBJECT, move |_, _| extra.into_ret());
        let found = combat_group_find_search_door(&mut e, group, who, 100.0, 0);
        assert!(found.is_null());
        e.register(fake(0x28), |_, _| false.into_ret());
        let found = combat_group_find_search_door(&mut e, group, who, 100.0, 0);
        assert_eq!(found.addr(), references[1]);
    }

    #[test]
    fn combat_strength_of_members_and_targets() {
        let mut e = search_engine();
        add_process_fakes(&mut e);
        let player = actor(&mut e, 1);
        e.set_global(PLAYER, player.addr());
        let (m1, m2, m3) = (actor(&mut e, 2), actor(&mut e, 3), actor(&mut e, 4));
        // m1: process, not fleeing, dps 3 strength 5; m2: fleeing; m3: no
        // process (keeps its stored values 4 / 2).
        let p1 = process_block(&mut e, 3.0, 5.0, 0, false);
        let p2 = process_block(&mut e, 100.0, 100.0, 0, true);
        e.mem.set_u32(m1.addr() + 0x38, p1);
        e.mem.set_u32(m2.addr() + 0x38, p2);
        let (t1, t2, t3) = (actor(&mut e, 5), actor(&mut e, 6), actor(&mut e, 7));
        // t1: process with strength 6; t2: fleeing; t3: no process.
        let p3 = process_block(&mut e, 0.0, 6.0, 0, false);
        e.mem.set_u32(t1.addr() + 0x38, p3);
        e.mem.set_u8(t2.addr() + 0x3e, 1);
        let group = group_with(&mut e, &[player, t1, t2, t3], &[player, m1, m2, m3]);
        let last = member_entry(&e, group, 3);
        e.set(last, CombatMember::fCombatStrength, 4.0f32);
        e.set(last, CombatMember::fDamagePerSecond, 2.0f32);
        combat_group_update_combat_strength(&mut e, group);
        // Members: the player (dps 10 -> strength 20), m1 (5), m3 (4).
        assert_eq!(e.get(group, CombatGroup::fMemberCombatStrength), 29.0);
        assert_eq!(
            e.get(group, CombatGroup::fAverageMemberDamagePerSecond),
            5.0
        );
        let first = member_entry(&e, group, 0);
        assert_eq!(e.get(first, CombatMember::fDamagePerSecond), 10.0);
        assert_eq!(e.get(first, CombatMember::fCombatStrength), 20.0);
        let second = member_entry(&e, group, 1);
        assert_eq!(e.get(second, CombatMember::fDamagePerSecond), 3.0);
        assert_eq!(e.get(second, CombatMember::fCombatStrength), 5.0);
        // Targets: the player (7), t1 (6), t3 (7); t2 fleeing is out.
        assert_eq!(e.get(group, CombatGroup::fTargetCombatStrength), 20.0);
        assert_eq!(
            e.get(group, CombatGroup::fAverageTargetCombatStrength),
            20.0f32 / 3.0
        );
        // No members and no targets: the sums and averages are zero.
        let empty = group_with(&mut e, &[], &[]);
        e.call(0x0098_c870, &args![empty]);
        assert_eq!(e.get(empty, CombatGroup::fMemberCombatStrength), 0.0);
        assert_eq!(e.get(empty, CombatGroup::fAverageTargetCombatStrength), 0.0);
    }

    #[test]
    fn member_strength_and_damage_as_seen_from_an_actor() {
        let mut e = search_engine();
        add_process_fakes(&mut e);
        let (a, b, c, d) = (
            actor(&mut e, 1),
            actor(&mut e, 2),
            actor(&mut e, 3),
            actor(&mut e, 4),
        );
        let process_a = process_block(&mut e, 0.0, 0.0, 0, false);
        let process_b = process_block(&mut e, 0.0, 0.0, 0, true);
        let process_d = process_block(&mut e, 9.0, 7.0, 0, false);
        e.mem.set_u32(a.addr() + 0x38, process_a);
        e.mem.set_u32(b.addr() + 0x38, process_b);
        e.mem.set_u32(d.addr() + 0x38, process_d);
        let group = group_with(&mut e, &[], &[a, b, c, d]);
        // Stored values: (strength, damage per second).
        for (i, (strength, damage)) in [(10.0f32, 1.0f32), (20.0, 2.0), (-5.0, 3.0), (40.0, 4.0)]
            .iter()
            .enumerate()
        {
            let member = member_entry(&e, group, i as u32);
            e.set(member, CombatMember::fCombatStrength, *strength);
            e.set(member, CombatMember::fDamagePerSecond, *damage);
        }
        e.set(group, CombatGroup::fMemberCombatStrength, 123.0f32);
        e.set(group, CombatGroup::fAverageMemberDamagePerSecond, 4.5f32);
        // A null actor: the stored sums.
        assert_eq!(fn_0098cb30(&mut e, group, Ptr::NULL), 123.0);
        assert_eq!(fn_0098cc20(&mut e, group, Ptr::NULL), 4.5);
        // Seen from d: d's state (7) + a's stored 10; b is fleeing, c has a
        // negative strength.
        assert_eq!(fn_0098cb30(&mut e, group, d), 17.0);
        // Damage: d's state 9, a's stored 1 -> average 5.
        assert_eq!(e.call(0x0098_cc20, &args![group, d]).f32(), 5.0);
        // An actor outside the group: a (10) and d (40) count.
        let outsider = actor(&mut e, 9);
        assert_eq!(e.call(0x0098_cb30, &args![group, outsider]).f32(), 50.0);
        assert_eq!(fn_0098cc20(&mut e, group, outsider), (1.0 + 4.0) / 2.0);
    }

    #[test]
    fn nearest_other_member_within_a_radius() {
        let mut e = search_engine();
        add_process_fakes(&mut e);
        let (who, near, preferred, farther) = (
            actor(&mut e, 1),
            actor(&mut e, 2),
            actor(&mut e, 3),
            actor(&mut e, 4),
        );
        for (a, x) in [(who, 0.0f32), (near, 3.0), (preferred, 4.0), (farther, 4.5)] {
            place_actor(&mut e, a, 0, x, 0);
        }
        let group = group_with(&mut e, &[], &[who, near, preferred, farther]);
        // Squared distances 9, 16 / 2.25 = 7.1, 20.25 inside 5 (25).
        e.set_global(TWO_AND_QUARTER_DOUBLE, 2.25f64);
        assert_eq!(fn_0098cd50(&mut e, group, who, preferred, 5.0), preferred);
        // Without a preferred member the nearest wins.
        assert_eq!(
            e.call(0x0098_cd50, &args![group, who, 0u32, 5.0f32])
                .ptr::<()>(),
            near
        );
        // Radius 2: nobody.
        assert!(fn_0098cd50(&mut e, group, who, preferred, 2.0).is_null());
        // The nearest one is discarded when its combat state's record
        // points back at the actor.
        let record_block = e.mem.alloc(8);
        e.mem.set_u32(record_block + 4, who.addr());
        e.register_double(STATE_TARGET_RECORD, move |_, _| record_block.into_ret());
        let process = process_block(&mut e, 0.0, 0.0, 0, false);
        e.mem.set_u32(preferred.addr() + 0x38, process);
        assert!(fn_0098cd50(&mut e, group, who, preferred, 5.0).is_null());
        e.mem.set_u32(record_block + 4, 0x1234);
        assert_eq!(fn_0098cd50(&mut e, group, who, preferred, 5.0), preferred);
    }

    #[test]
    fn member_counts_follow_the_processes() {
        let mut e = search_engine();
        add_process_fakes(&mut e);
        let actors: Vec<Ptr> = (0..4).map(|i| actor(&mut e, i + 1)).collect();
        // No process; initialised, not fleeing; initialised and fleeing;
        // not initialised, fleeing.
        let p1 = process_block(&mut e, 0.0, 0.0, 8, false);
        let p2 = process_block(&mut e, 0.0, 0.0, 8, true);
        let p3 = process_block(&mut e, 0.0, 0.0, 0, true);
        for (a, p) in actors[1..].iter().zip([p1, p2, p3]) {
            e.mem.set_u32(a.addr() + 0x38, p);
        }
        let group = group_with(&mut e, &[], &actors);
        e.set(group, CombatGroup::iInitializedMemberCount, 9);
        combat_group_update_counts(&mut e, group);
        assert_eq!(e.get(group, CombatGroup::iInitializedMemberCount), 2);
        assert_eq!(e.get(group, CombatGroup::iFleeingMemberCount), 2);
        assert_eq!(e.get(group, CombatGroup::iNonFleeingMemberCount), 1);
        e.call(0x0098_ce80, &args![group]);
    }

    #[test]
    fn attacker_counts_count_members_aimed_at_each_target() {
        let mut e = search_engine();
        fuller_actor_vtable(&mut e);
        let player = actor(&mut e, 1);
        e.set_global(PLAYER, player.addr());
        let (m1, m2, m3) = (actor(&mut e, 2), actor(&mut e, 3), actor(&mut e, 4));
        let (t1, t2) = (actor(&mut e, 5), actor(&mut e, 6));
        // Aimed at: the player at t1 (not counted), m1 and m2 at t1, m3 at t2.
        for (a, who) in [(player, t1), (m1, t1), (m2, t1), (m3, t2)] {
            e.mem.set_u32(a.addr() + 0x4c, who.addr());
        }
        let group = group_with(&mut e, &[t1, t2], &[player, m1, m2, m3]);
        let first = target_entry(&e, group, 0);
        e.set(first, CombatTarget::sAttackerCount, 9);
        fn_0098cf70(&mut e, group);
        assert_eq!(e.get(first, CombatTarget::sAttackerCount), 2);
        let second = target_entry(&e, group, 1);
        assert_eq!(e.get(second, CombatTarget::sAttackerCount), 1);
        e.call(0x0098_cf70, &args![group]);
    }

    /// `array_engine()` with the search fakes, for the tests that remove
    /// targets or add members.
    fn array_search_engine() -> Engine {
        let mut e = array_engine();
        add_search_fakes(&mut e);
        e.register(ACTOR_TEST_00437BD0, |_, _| Ret::default());
        e
    }

    #[test]
    fn invalid_targets_are_dropped_and_a_lone_one_is_commented_on() {
        let mut e = array_search_engine();
        e.set_global(0x011c_e744u32, 10.0f32);
        e.set_global(0x011f_1708u32, 0x4444u32);
        e.set_global(ONE_MILLION_FLOAT, 1.0e6f32);
        record(&mut e, &[START_DIALOGUE]);
        let (stale, fresh, speaker) = (actor(&mut e, 1), actor(&mut e, 2), actor(&mut e, 3));
        place_actor(&mut e, speaker, 0, 5.0, 0);
        let group = group_with(&mut e, &[stale, fresh], &[speaker]);
        // The stale target was last noticed at time 0 (age 100 > 10) and has
        // no attackers; the fresh one was noticed now.
        e.set(
            target_entry(&e, group, 0),
            CombatTarget::fLastNoticedTimeStamp,
            0.0f32,
        );
        e.set(
            target_entry(&e, group, 1),
            CombatTarget::fLastNoticedTimeStamp,
            NOW,
        );
        fn_0098d030(&mut e, group);
        assert_eq!(array_len(&e, group, 8), 1);
        assert_eq!(
            e.get(target_entry(&e, group, 0), CombatTarget::pActor),
            fresh
        );
        // With two targets nobody speaks.
        assert!(calls(&e, START_DIALOGUE).is_empty());
        assert_eq!(
            calls(&e, MANAGER_REMOVE_TARGET),
            vec![vec![MANAGER, group.addr(), stale.addr()]]
        );
        // A lone stale target that was never detected: type 5 from the
        // nearest member; once detected, type 8.
        let lone = group_with(&mut e, &[stale], &[speaker]);
        e.set(
            target_entry(&e, lone, 0),
            CombatTarget::fLastNoticedTimeStamp,
            0.0f32,
        );
        e.set(
            target_entry(&e, lone, 0),
            CombatTarget::fLastDetectedTimeStamp,
            -f32::MAX,
        );
        e.call(0x0098_d030, &args![lone]);
        assert_eq!(array_len(&e, lone, 8), 0);
        assert_eq!(
            calls(&e, START_DIALOGUE),
            vec![vec![0x4444, speaker.addr(), stale.addr(), 4, 5, 1, 0]]
        );
        let lone = group_with(&mut e, &[stale], &[speaker]);
        e.set(
            target_entry(&e, lone, 0),
            CombatTarget::fLastNoticedTimeStamp,
            0.0f32,
        );
        e.set(
            target_entry(&e, lone, 0),
            CombatTarget::fLastDetectedTimeStamp,
            10.0f32,
        );
        fn_0098d030(&mut e, lone);
        assert_eq!(calls(&e, START_DIALOGUE).len(), 2);
        assert_eq!(calls(&e, START_DIALOGUE)[1][4], 8);
        // No member near the target (and none for the player): silence.
        let player = actor(&mut e, 4);
        e.set_global(PLAYER, player.addr());
        let alone = group_with(&mut e, &[stale], &[]);
        e.set(
            target_entry(&e, alone, 0),
            CombatTarget::fLastNoticedTimeStamp,
            0.0f32,
        );
        fn_0098d030(&mut e, alone);
        assert_eq!(calls(&e, START_DIALOGUE).len(), 2);
        // No targets: nothing happens.
        let none = group_with(&mut e, &[], &[]);
        fn_0098d030(&mut e, none);
    }

    /// Strategy test doubles: the chooser, the index, the timers (expired
    /// when the first byte of the timer is set) and the strategy's virtual
    /// method 8 (its result is the byte at +8 of the strategy).
    fn strategy_engine() -> Engine {
        let mut e = search_engine();
        e.register(TIMER_EXPIRED, |e, a| e.mem.u8(a[0]).into_ret());
        record(&mut e, &[TIMER_START]);
        e.register(STRATEGY_INDEX, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.set_global(STRATEGY_DELAY, 5.0f32);
        e.put_vtable(0x7220_0000, &[0, 0, fake(8)]);
        e.register(fake(8), |e, a| e.mem.u8(a[0] + 8).into_ret());
        e
    }

    #[test]
    fn strategy_is_chosen_and_run() {
        let mut e = strategy_engine();
        let strategy = e.mem.alloc(0x10);
        e.mem.set_u32(strategy, 0x7220_0000);
        e.mem.set_u32(strategy + 4, 3);
        e.mem.set_u8(strategy + 8, 1);
        e.register_double(CHOOSE_STRATEGY, move |_, _| strategy.into_ret());
        let (a, b) = (actor(&mut e, 1), actor(&mut e, 2));
        let group = group_with(&mut e, &[], &[a, b]);
        // Choose timer expired, update timer expired.
        e.mem.set_u8(group.addr() + 0x30, 1);
        e.mem.set_u8(group.addr() + 0x38, 1);
        e.call_log = Some(vec![]);
        fn_0098d1a0(&mut e, group);
        assert_eq!(e.get(group, CombatGroup::pGroupStrategy).addr(), strategy);
        assert_eq!(e.get(group, CombatGroup::iLastGroupStrategyChosenIndex), 3);
        assert_eq!(
            e.get(group, CombatGroup::fLastGroupStrategyChosenTimeStamp),
            NOW
        );
        let starts = calls(&e, TIMER_START);
        assert_eq!(starts.len(), 2);
        assert_eq!(starts[0][0], group.addr() + 0x30);
        assert_eq!(f32::from_bits(starts[0][1]), 5.0);
        assert_eq!(starts[1][0], group.addr() + 0x38);
        assert_eq!(f32::from_bits(starts[1][1]), 1.0);
        // The strategy's method says stop: it is dropped.
        e.mem.set_u8(strategy + 8, 0);
        e.mem.set_u8(group.addr() + 0x30, 0);
        e.call(0x0098_d1a0, &args![group]);
        assert!(e.get(group, CombatGroup::pGroupStrategy).is_null());
        assert!(!e.get(group, CombatGroup::bStrategyForced));
        // A forced strategy is not re-chosen, even with the timer expired.
        e.set(group, CombatGroup::pGroupStrategy, Ptr::new(strategy));
        e.set(group, CombatGroup::bStrategyForced, true);
        e.mem.set_u8(group.addr() + 0x30, 1);
        e.mem.set_u8(strategy + 8, 1);
        e.register_double(CHOOSE_STRATEGY, |_, _| panic!("must not choose"));
        e.mem.set_u8(group.addr() + 0x38, 0);
        fn_0098d1a0(&mut e, group);
        assert!(e.get(group, CombatGroup::bStrategyForced));
        // Fewer than two members: no strategy, the timer restarts with 0.
        let single = group_with(&mut e, &[], &[a]);
        e.set(single, CombatGroup::pGroupStrategy, Ptr::new(strategy));
        e.set(single, CombatGroup::bStrategyForced, true);
        e.call_log = Some(vec![]);
        fn_0098d1a0(&mut e, single);
        assert!(e.get(single, CombatGroup::pGroupStrategy).is_null());
        assert!(!e.get(single, CombatGroup::bStrategyForced));
        assert_eq!(
            calls(&e, TIMER_START),
            vec![vec![single.addr() + 0x30, 0.0f32.to_bits()]]
        );
    }

    /// The doubles `fn_0098d2c0` and `fn_0098dc40` need besides the search
    /// fakes: the cluster array (buffer +0x13c, size +0x140), memory
    /// functions and the expired cluster timer.
    fn cluster_engine(radius: f32) -> Engine {
        let mut e = search_engine();
        e.map(0x011a_4000, 0x1000);
        e.set_global(CLUSTER_RADIUS, radius);
        e.set_global(CLUSTER_DELAY, 2.0f32);
        e.set_global(FOUR_DOUBLE, 4.0f64);
        e.set_global(TWO_FLOAT, 2.0f32);
        e.mem.set_u8(CLUSTER_DISABLE, 0);
        e.register(TIMER_EXPIRED, |e, a| e.mem.u8(a[0]).into_ret());
        record(&mut e, &[TIMER_START, VECTOR_CONSTRUCTOR, OPERATOR_DELETE]);
        e.register(MEMSET, |e, a| {
            e.mem.write(a[0], &vec![a[1] as u8; a[2] as usize]);
            Ret::default()
        });
        e.register(OPERATOR_NEW_SMALL, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(SIGNED_MIN, |_, a| (a[0] as i32).min(a[1] as i32).into_ret());
        e.register(POINTER_ARRAY_ADD, |e, a| {
            let (buffer, size) = (e.mem.u32(a[0] + 4), e.mem.u32(a[0] + 8));
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(buffer + 4 * size, value);
            e.mem.set_u32(a[0] + 8, size + 1);
            size.into_ret()
        });
        e.register(CLUSTER_REMOVE, |e, a| {
            let (buffer, size) = (e.mem.u32(a[0] + 4), e.mem.u32(a[0] + 8));
            for i in a[1]..size - a[2] {
                let value = e.mem.u32(buffer + 4 * (i + a[2]));
                e.mem.set_u32(buffer + 4 * i, value);
            }
            e.mem.set_u32(a[0] + 8, size - a[2]);
            Ret::default()
        });
        e
    }

    /// A group whose members are fake actors at the given x positions
    /// (all in `space`), with room in its cluster array.
    fn cluster_group(e: &mut Engine, positions: &[f32], space: u32) -> Ptr<CombatGroup> {
        let mut members = vec![];
        for (i, x) in positions.iter().enumerate() {
            let who = actor(e, 10 + i as u32);
            place_actor(e, who, 0, *x, space);
            members.push(who);
        }
        let group = group_with(e, &[], &members);
        let buffer = e.mem.alloc(0x100);
        e.mem.set_u32(group.addr() + 0x13c, buffer);
        // The cluster timer (+0xb0) has expired.
        e.mem.set_u8(group.addr() + 0xb0, 1);
        group
    }

    fn cluster_count(e: &Engine, group: Ptr<CombatGroup>) -> u32 {
        e.mem.u32(group.addr() + 0x140)
    }

    fn cluster_at(e: &Engine, group: Ptr<CombatGroup>, index: u32) -> u32 {
        e.mem.u32(e.mem.u32(group.addr() + 0x13c) + 4 * index)
    }

    #[test]
    fn nearby_members_form_a_cluster() {
        let mut e = cluster_engine(5.0);
        let space = e.mem.alloc(8);
        // 0 and 1 are mutual nearest neighbours; 2 is far away.
        let group = cluster_group(&mut e, &[0.0, 1.0, 100.0], space);
        fn_0098d2c0(&mut e, group);
        assert_eq!(cluster_count(&e, group), 1);
        let cluster = cluster_at(&e, group, 0);
        let (m0, m1, m2) = (
            member_entry(&e, group, 0),
            member_entry(&e, group, 1),
            member_entry(&e, group, 2),
        );
        assert_eq!(e.get(m0, CombatMember::pCluster).addr(), cluster);
        assert_eq!(e.get(m1, CombatMember::pCluster).addr(), cluster);
        assert!(e.get(m2, CombatMember::pCluster).is_null());
        // The centre is recomputed from the members: x = 0.5, 2 members.
        assert_eq!(point_at(&e, cluster), [0.5, 0.0, 0.0]);
        assert_eq!(e.mem.u32(cluster + 0x10), 2);
        assert_eq!(e.mem.u32(cluster + 0x0c), space);
        let starts = calls(&e, TIMER_START);
        assert_eq!(starts[0][0], group.addr() + 0xb0);
        assert_eq!(f32::from_bits(starts[0][1]), 2.0);
        // Nothing happens before the timer expires (the centres are still
        // recomputed) or when the global switch is on.
        e.mem.set_u8(group.addr() + 0xb0, 0);
        e.mem.set_f32(cluster, 99.0);
        e.call(0x0098_d2c0, &args![group]);
        assert_eq!(point_at(&e, cluster), [0.5, 0.0, 0.0]);
        assert_eq!(calls(&e, TIMER_START).len(), 1);
        e.mem.set_u8(group.addr() + 0xb0, 1);
        e.mem.set_u8(CLUSTER_DISABLE, 1);
        e.mem.set_f32(cluster, 99.0);
        fn_0098d2c0(&mut e, group);
        assert_eq!(e.mem.f32(cluster), 99.0);
    }

    #[test]
    fn a_close_member_joins_the_cluster_and_far_clusters_are_dropped() {
        let mut e = cluster_engine(5.0);
        let space = e.mem.alloc(8);
        // 0 and 1 form a cluster; 2 is within the radius of its centre; 3
        // is far from everything.
        let group = cluster_group(&mut e, &[0.0, 1.0, 3.0, 100.0], space);
        // Members 0 and 3 start in a stale cluster: its centre is their
        // average (50), more than twice the radius from both.
        let stale = e.mem.alloc(0x14);
        e.mem.set_u32(stale + 0x10, 2);
        e.mem.set_u32(group.addr() + 0x140, 1);
        e.mem.set_u32(e.mem.u32(group.addr() + 0x13c), stale);
        e.set(
            member_entry(&e, group, 0),
            CombatMember::pCluster,
            Ptr::new(stale),
        );
        e.set(
            member_entry(&e, group, 3),
            CombatMember::pCluster,
            Ptr::new(stale),
        );
        fn_0098d2c0(&mut e, group);
        // The stale cluster is deleted; one new cluster holds members 0-2.
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![stale]]);
        assert_eq!(cluster_count(&e, group), 1);
        let cluster = cluster_at(&e, group, 0);
        assert_ne!(cluster, stale);
        for i in 0..3 {
            let member = member_entry(&e, group, i);
            assert_eq!(e.get(member, CombatMember::pCluster).addr(), cluster);
        }
        assert!(e
            .get(member_entry(&e, group, 3), CombatMember::pCluster)
            .is_null());
        assert_eq!(e.mem.u32(cluster + 0x10), 3);
        let centre = point_at(&e, cluster);
        assert!((centre[0] - 4.0 / 3.0).abs() < 1e-5);
    }

    #[test]
    fn clusters_closer_than_the_radius_are_merged() {
        let mut e = cluster_engine(25.0);
        let space = e.mem.alloc(8);
        // Pairs (0, 1) at 0.5 and (2, 3) at 20.5: 20 apart, inside 25.
        let group = cluster_group(&mut e, &[0.0, 1.0, 20.0, 21.0], space);
        fn_0098d2c0(&mut e, group);
        assert_eq!(cluster_count(&e, group), 1);
        let cluster = cluster_at(&e, group, 0);
        for i in 0..4 {
            let member = member_entry(&e, group, i);
            assert_eq!(e.get(member, CombatMember::pCluster).addr(), cluster);
        }
        assert_eq!(calls(&e, OPERATOR_DELETE).len(), 1);
        assert_eq!(e.mem.u32(cluster + 0x10), 4);
        assert_eq!(point_at(&e, cluster), [10.5, 0.0, 0.0]);
        // With a radius of 5 the pairs stay apart.
        let mut e = cluster_engine(5.0);
        let space = e.mem.alloc(8);
        let group = cluster_group(&mut e, &[0.0, 1.0, 20.0, 21.0], space);
        fn_0098d2c0(&mut e, group);
        assert_eq!(cluster_count(&e, group), 2);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
    }

    #[test]
    fn cluster_helpers() {
        let mut e = cluster_engine(5.0);
        let space = e.mem.alloc(8);
        // Construct: the location constructor, then a zero count.
        let block = e.mem.alloc(0x20);
        e.mem.set_u32(block + 0x10, 7);
        assert_eq!(fn_0098dc00(&mut e, Ptr::new(block)).addr(), block);
        assert_eq!(e.mem.u32(block + 0x10), 0);
        assert_eq!(e.call(0x0098_dc20, &args![block]).u32(), block);
        // The point copy leaves the space alone.
        let cluster = e.mem.alloc(0x14);
        e.mem.set_u32(cluster + 0x0c, 0x55);
        let point = e.mem.alloc(0xc);
        for (i, value) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.mem.set_f32(point + 4 * i as u32, *value);
        }
        fn_0098ddd0(&mut e, Ptr::new(cluster), Ptr::new(point));
        e.call(0x0098_ddd0, &args![cluster, point]);
        assert_eq!(point_at(&e, cluster), [1.0, 2.0, 3.0]);
        assert_eq!(e.mem.u32(cluster + 0x0c), 0x55);
        // The cluster of an actor and by index.
        let group = cluster_group(&mut e, &[0.0, 1.0], space);
        let stranger = actor(&mut e, 99);
        let other = e.mem.alloc(0x14);
        e.mem.set_u32(group.addr() + 0x140, 2);
        e.mem.set_u32(e.mem.u32(group.addr() + 0x13c), cluster);
        e.mem.set_u32(e.mem.u32(group.addr() + 0x13c) + 4, other);
        let first_member = member_entry(&e, group, 0);
        e.set(first_member, CombatMember::pCluster, Ptr::new(other));
        let first_actor = e.get(first_member, CombatMember::pActor);
        assert_eq!(fn_0098de00(&mut e, group, first_actor), other);
        assert_eq!(e.call(0x0098_de00, &args![group, stranger]).u32(), 0);
        let second_actor = e.get(member_entry(&e, group, 1), CombatMember::pActor);
        assert_eq!(fn_0098de00(&mut e, group, second_actor), 0);
        assert_eq!(fn_0098de60(&mut e, group, 1), other);
        assert_eq!(fn_0098de60(&mut e, group, 0), cluster);
        assert_eq!(e.call(0x0098_de60, &args![group, 0xffu32]).u32(), 0);
        // The search passes the array, the address of a word holding the
        // cluster, 0 and the comparison.
        let mut seen = vec![];
        e.register_double(CLUSTER_SEARCH, move |e, a| {
            seen.push((a.to_vec(), e.mem.u32(a[1])));
            (seen.len() as u32 + 40).into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(fn_0098de30(&mut e, group, other), 41);
        let logged = calls(&e, CLUSTER_SEARCH);
        assert_eq!(logged.len(), 1);
        assert_eq!(logged[0][0], group.addr() + 0x138);
        assert_eq!(&logged[0][2..], &[0, CLUSTER_COMPARE]);
    }

    #[test]
    fn centres_of_clusters_are_recomputed_from_their_members() {
        let mut e = cluster_engine(5.0);
        let space = e.mem.alloc(8);
        let group = cluster_group(&mut e, &[2.0, 4.0, 9.0], space);
        let (one, two) = (e.mem.alloc(0x14), e.mem.alloc(0x14));
        // Stale centres and counts that must be replaced.
        for cluster in [one, two] {
            e.mem.set_f32(cluster, 77.0);
            e.mem.set_u32(cluster + 0x10, 5);
        }
        e.mem.set_u32(group.addr() + 0x140, 2);
        e.mem.set_u32(e.mem.u32(group.addr() + 0x13c), one);
        e.mem.set_u32(e.mem.u32(group.addr() + 0x13c) + 4, two);
        for (i, cluster) in [(0u32, one), (1, one), (2, two)] {
            e.set(
                member_entry(&e, group, i),
                CombatMember::pCluster,
                Ptr::new(cluster),
            );
        }
        fn_0098dc40(&mut e, group);
        assert_eq!(e.mem.u32(one + 0x10), 2);
        assert_eq!(e.mem.f32(one), 3.0);
        assert_eq!(e.mem.u32(two + 0x10), 1);
        assert_eq!(e.mem.f32(two), 9.0);
        // A cluster with no members ends up at the origin divided by zero:
        // its count is zero.
        let empty = e.mem.alloc(0x14);
        e.mem.set_u32(group.addr() + 0x140, 3);
        e.mem.set_u32(e.mem.u32(group.addr() + 0x13c) + 8, empty);
        e.call(0x0098_dc40, &args![group]);
        assert_eq!(e.mem.u32(empty + 0x10), 0);
        assert!(e.mem.f32(empty).is_nan());
    }

    #[test]
    fn save_game_writes_the_group_in_order() {
        let mut e = search_engine();
        let saved = std::rc::Rc::new(std::cell::RefCell::new(Vec::<(u32, Vec<u8>)>::new()));
        let log = saved.clone();
        record(
            &mut e,
            &[
                SAVE_SIZED_VALUE,
                TIMER_SAVE,
                TIME_STAMP_SAVE,
                WORLD_LOCATION_SAVE,
                SAVE_FORM_ID,
                SAVE_FORM_ID_PLAIN,
                LOS_GRID_MAP_SAVE,
                CLUSTER_SEARCH,
            ],
        );
        e.register_double(SAVE_BYTES, move |e, a| {
            log.borrow_mut().push((a[2], e.mem.bytes(a[1], a[2])));
            Ret::default()
        });
        e.register(STRATEGY_INDEX, |e, a| e.mem.u32(a[0] + 4).into_ret());
        let strategy = e.mem.alloc(0x10);
        e.mem.set_u32(strategy + 4, 3);
        let (t1, t2, m1) = (actor(&mut e, 1), actor(&mut e, 2), actor(&mut e, 3));
        let group = group_with(&mut e, &[t1, t2], &[m1]);
        let g = group.addr();
        // One cluster, two search locations, one door, a grid and a strategy.
        let cluster = e.mem.alloc(0x14);
        let cluster_buffer = e.mem.alloc(0x10);
        e.mem.set_u32(cluster_buffer, cluster);
        e.mem.set_u32(g + 0x13c, cluster_buffer);
        e.mem.set_u32(g + 0x140, 1);
        put_locations(
            &mut e,
            group,
            &[
                ([1.0, 2.0, 3.0], 0, 4.0, 0x55),
                ([5.0, 6.0, 7.0], 0, 8.0, 0),
            ],
        );
        put_doors(&mut e, group, &[(0x61, 0x62, 1, 0, 1)]);
        let grid = e.mem.alloc(0x10);
        e.mem.set_u32(g + 0xd4, grid);
        e.set(group, CombatGroup::pGroupStrategy, Ptr::new(strategy));
        let searching = actor(&mut e, 4);
        e.set(group, CombatGroup::pSearchingMember, searching);
        e.set(group, CombatGroup::iSearchCount, 2);
        e.set(group, CombatGroup::cCombatMusicState, 5);
        e.call_log = Some(vec![]);
        combat_group_save_game(&mut e, group, Ptr::new(0x9000));
        let inside = |p: u32| p >= g && p < g + 0x15c;
        let summary: Vec<String> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter_map(|(address, a)| match *address {
                SAVE_SIZED_VALUE => Some(format!("count {}", a[1])),
                SAVE_BYTES if inside(a[1]) => Some(format!("bytes +{:x} x{}", a[1] - g, a[2])),
                TIMER_SAVE if inside(a[0]) => Some(format!("timer +{:x}", a[0] - g)),
                TIME_STAMP_SAVE if inside(a[0]) => Some(format!("stamp +{:x}", a[0] - g)),
                WORLD_LOCATION_SAVE if inside(a[0]) => Some(format!("location +{:x}", a[0] - g)),
                _ => None,
            })
            .collect();
        let mut expected: Vec<String> = [
            "count 1",
            "count 2",
            "count 1",
            "bytes +2c x1",
            "timer +30",
            "timer +38",
            "bytes +40 x4",
            "stamp +44",
            "timer +48",
            "timer +b0",
            "timer +a8",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        for timer in 0..10u32 {
            expected.push(format!("timer +{:x}", 0x58 + 8 * timer));
        }
        expected.extend(
            [
                "bytes +d0 x4",
                "bytes +100 x12",
                "timer +d8",
                "timer +e0",
                "stamp +e8",
                "location +f0",
                "bytes +10c x4",
                "count 2",
                "count 1",
                "timer +50",
                "bytes +c0 x4",
                "bytes +c4 x4",
                "bytes +c8 x4",
                "bytes +cc x4",
                "timer +b8",
                "bytes +154 x1",
            ]
            .iter()
            .map(|s| s.to_string()),
        );
        assert_eq!(summary, expected);
        // The strategy index byte (3) is among the one-byte writes, the last is the
        // music state; the grid is saved with the buffer.
        let singles: Vec<Vec<u8>> = saved
            .borrow()
            .iter()
            .filter(|(n, _)| *n == 1)
            .map(|(_, b)| b.clone())
            .collect();
        assert!(singles.contains(&vec![3u8]));
        assert_eq!(singles.last().unwrap(), &vec![5u8]);
        assert_eq!(calls(&e, LOS_GRID_MAP_SAVE), vec![vec![grid, 0x9000]]);
        // The form IDs written: the two targets, the member, the searching
        // member (a fourth actor) and the door's two references.
        let written: Vec<u32> = calls(&e, SAVE_FORM_ID).iter().map(|c| c[1]).collect();
        assert_eq!(
            written,
            vec![
                t1.addr(),
                t2.addr(),
                m1.addr(),
                searching.addr(),
                0x61,
                0x62
            ]
        );
        // The search location's target ID goes through the plain form ID writer.
        assert_eq!(calls(&e, SAVE_FORM_ID_PLAIN)[0], vec![0x9000, 0x55, 0]);

        // Without a search nothing of the search state is written; with no
        // strategy the index byte is 0xff, with no clusters the count is 0.
        saved.borrow_mut().clear();
        let quiet = group_with(&mut e, &[], &[]);
        e.call_log = Some(vec![]);
        e.call(0x0098_de90, &args![quiet, 0x9000u32]);
        assert_eq!(saved.borrow().first().unwrap(), &(1, vec![0xffu8]));
        let g = quiet.addr();
        let after: Vec<String> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter_map(|(address, a)| match *address {
                SAVE_SIZED_VALUE => Some(format!("count {}", a[1])),
                TIMER_SAVE => Some(format!("timer +{:x}", a[0] - g)),
                _ => None,
            })
            .collect();
        assert_eq!(after[..3], ["count 0", "count 0", "count 0"]);
        assert!(!after.iter().any(|s| s == "timer +d8"));
        assert!(after.iter().any(|s| s == "timer +50"));
    }

    /// The doubles `fn_0098add0` needs: the settings, the timers, a random
    /// delay of 3.5, the radius settings and the world locations.
    fn search_start_engine() -> Engine {
        let mut e = search_engine();
        e.set_global(HALF_FLOAT, 0.5f32);
        e.set_global(0x011c_f870u32, 4.0f32);
        e.set_global(0x011c_e35cu32, 9.0f32);
        e.set_global(0x011c_f24cu32, 2.0f32);
        e.set_global(0x011c_e528u32, 0.0f32);
        e.mem.set_f64(0x0107_3568, 70.0);
        e.mem.set_f64(0x0101_db88, 30.0);
        e.register(RANDOM_FLOAT, |_, _| 3.5f32.into_ret());
        e.register(WORLD_LOCATION_DISTANCE, |_, _| 0.0f64.into_ret());
        record(
            &mut e,
            &[
                TIMER_START,
                POINTER_ARRAY_CONSTRUCT,
                POINTER_ARRAY_DESTRUCT,
                COLLECT_TELEPORT_DOORS,
                ARRAY_SORT,
            ],
        );
        e
    }

    /// A group of one member and two targets for the search start tests.
    fn search_start_group(e: &mut Engine) -> Ptr<CombatGroup> {
        let space = e.mem.alloc(8);
        put_radius_settings(e, 10.0, 30.0);
        let member = actor(e, 1);
        place_actor(e, member, 0, 5.0, space);
        let (t0, t1) = (actor(e, 2), actor(e, 3));
        let group = group_with(e, &[t0, t1], &[member]);
        // t0: detected at 60 (age 40), noticed at 70; t1: detected 80,
        // noticed 50; the attacked-member stamps are old.
        let (first, second) = (target_entry(e, group, 0), target_entry(e, group, 1));
        for (target, detected, noticed) in [(first, 60.0f32, 70.0f32), (second, 80.0, 50.0)] {
            e.set(target, CombatTarget::fLastDetectedTimeStamp, detected);
            e.set(target, CombatTarget::fLastNoticedTimeStamp, noticed);
            e.set(target, CombatTarget::fLastAttackedMemberTimeStamp, 0.0f32);
            e.set(target, CombatTarget::sLastSearchNoticed, 7);
            let x = if target == first { 10.0f32 } else { 20.0 };
            e.mem.set_f32(target.addr() + 0x18, x);
        }
        give_search_buffers(e, group);
        group
    }

    #[test]
    fn starting_a_search_sets_up_the_state() {
        let mut e = search_start_engine();
        let group = search_start_group(&mut e);
        e.call_log = Some(vec![]);
        fn_0098add0(&mut e, group);
        assert_eq!(e.get(group, CombatGroup::iSearchCount), 1);
        assert_eq!(e.get(group, CombatGroup::fSearchStartedTimeStamp), NOW);
        for i in 0..2 {
            let target = target_entry(&e, group, i);
            assert_eq!(e.get(target, CombatTarget::sLastSearchNoticed), 0);
        }
        // The two search locations (the targets' detected locations, 10 and
        // 20, scores 30 and 50) are averaged with those scores as weights;
        // the radius ends at the smaller setting.
        assert_eq!(e.mem.u32(group.addr() + 0x118), 2);
        let centre = point_at(&e, group.addr() + 0xf0);
        assert!((centre[0] - (10.0 * 30.0 + 20.0 * 50.0) / 80.0).abs() < 1e-3);
        assert_eq!(e.get(group, CombatGroup::fSearchRadius), 10.0);
        let starts: Vec<(u32, f32)> = calls(&e, TIMER_START)
            .iter()
            .map(|c| (c[0] - group.addr(), f32::from_bits(c[1])))
            .collect();
        assert_eq!(
            starts,
            vec![(0xd8, 0.5), (0xe0, 4.0), (0x58, 3.5), (0x60, 3.5)]
        );
        // The random delays are asked between the settings 011cf24c and
        // 011ce35c.
        let randoms = calls(&e, RANDOM_FLOAT);
        assert_eq!(randoms.len(), 2);
        assert_eq!(f32::from_bits(randoms[0][0]), 2.0);
        assert_eq!(f32::from_bits(randoms[0][1]), 9.0);
        e.call(0x0098_add0, &args![group]);
    }

    /// Everything `fn_0098a580` calls besides the search start.
    fn search_step_engine() -> Engine {
        let mut e = search_engine();
        add_process_fakes(&mut e);
        fuller_actor_vtable(&mut e);
        e.set_global(DIALOGUE_MANAGER, 0x4444u32);
        e.register(SEARCH_RUNNING, |e, a| {
            (e.mem.u32(a[0] + 0xd0) != 0).into_ret()
        });
        e.register(TIMER_EXPIRED, |e, a| e.mem.u8(a[0]).into_ret());
        e.register(ACTOR_TEST_008B3B90, |e, a| e.mem.u8(a[0] + 0x3d).into_ret());
        e.register(ACTOR_IS_WAITING_ON_PATH, |e, a| {
            e.mem.u8(a[0] + 0x3e).into_ret()
        });
        e.register(NI_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(LOS_GRID_MAP_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(WORLD_LOCATION_GET_CELL_OR_WORLD, |e, a| {
            e.mem.set_u32(a[1], 0xbb);
            e.mem.set_u32(a[2], 0xaa);
            true.into_ret()
        });
        e.register(FIND_CLOSEST_POINT_ON_NAVMESH, |_, _| true.into_ret());
        e.register(PATHING_LOCATION_RESOLVE, |_, _| true.into_ret());
        e.register(REFERENCE_SPACE, |e, a| {
            e.mem.u32(a[0] + ACTOR_LOCATION + 0x0c).into_ret()
        });
        // A location has navmesh information when the actor's byte +0x3f
        // is set.
        e.register(PATHING_LOCATION_FROM_ACTOR, |e, a| {
            let info = e.mem.u8(a[1] + 0x3f) as u32;
            e.mem.set_u32(a[0] + 0x10, info);
            a[0].into_ret()
        });
        e.register(LOCATION_ARRAY_ADD, |e, a| {
            let size = e.mem.u32(a[0] + 8);
            e.mem.set_u32(a[0] + 8, size + 1);
            Ret::default()
        });
        e.register(CLAMP_ANGLE, |_, a| (f32::from_bits(a[0]) + 1.0).into_ret());
        e.register(RANDOM_INDEX, |_, a| (a[1] - 1).into_ret());
        e.register(PATH_HANDLER_DISTANCE_TRAVELLED, |_, _| 0.0f32.into_ret());
        record(
            &mut e,
            &[
                TIMER_START,
                TIMER_EXPIRE_NOW,
                LOS_GRID_MAP_STEP,
                LOS_GRID_MAP_UPDATE,
                LOS_GRID_MAP_APPLY,
                PATHING_LOCATION_FROM_POINT,
                PATHING_LOCATION_SET_POINT,
                PATHING_LOCATION_DESTRUCT,
                LOCATION_ARRAY_CONSTRUCT,
                LOCATION_ARRAY_DESTRUCT,
                ANGLE_ARRAY_CONSTRUCT,
                ANGLE_ARRAY_DESTRUCT,
                ANGLE_ARRAY_ADD,
                TIMER_RESET,
                ARRAY_CLEAR,
                START_DIALOGUE,
            ],
        );
        e
    }

    #[test]
    fn search_update_resets_or_starts_the_search() {
        let mut e = search_step_engine();
        e.set_global(0x011c_e9b4u32, 10.0f32);
        e.set_global(0x011c_e528u32, 5.0f32);
        let (t0, t1) = (actor(&mut e, 1), actor(&mut e, 2));
        let idle = actor(&mut e, 3);
        // A search is running and there are no targets: reset.
        let group = group_with(&mut e, &[], &[idle]);
        e.set(group, CombatGroup::iInitializedMemberCount, 1);
        e.set(group, CombatGroup::iSearchCount, 2);
        e.call_log = Some(vec![]);
        fn_0098a580(&mut e, group);
        assert_eq!(e.get(group, CombatGroup::iSearchCount), 0);
        assert_eq!(calls(&e, NI_POINTER_ASSIGN).len(), 2);
        // Running, not ready (targets, nobody fleeing): a target detected 5
        // ago (below 10) resets.
        let group = group_with(&mut e, &[t0, t1], &[idle]);
        e.set(group, CombatGroup::iInitializedMemberCount, 2);
        e.set(group, CombatGroup::iSearchCount, 2);
        e.set(
            target_entry(&e, group, 0),
            CombatTarget::fLastDetectedTimeStamp,
            50.0f32,
        );
        e.set(
            target_entry(&e, group, 1),
            CombatTarget::fLastDetectedTimeStamp,
            95.0f32,
        );
        e.call(0x0098_a580, &args![group]);
        assert_eq!(e.get(group, CombatGroup::iSearchCount), 0);
        // Targets detected 50 and 40 ago do not reset it.
        let group = group_with(&mut e, &[t0, t1], &[idle]);
        e.set(group, CombatGroup::iInitializedMemberCount, 2);
        e.set(group, CombatGroup::iSearchCount, 2);
        e.set(
            target_entry(&e, group, 0),
            CombatTarget::fLastDetectedTimeStamp,
            50.0f32,
        );
        e.set(
            target_entry(&e, group, 1),
            CombatTarget::fLastDetectedTimeStamp,
            60.0f32,
        );
        // The searching member is done (its byte +0x3d is set): dropped.
        e.mem.set_u8(idle.addr() + 0x3d, 1);
        e.set(group, CombatGroup::pSearchingMember, idle);
        fn_0098a580(&mut e, group);
        assert_eq!(e.get(group, CombatGroup::iSearchCount), 2);
        assert!(e.get(group, CombatGroup::pSearchingMember).is_null());
        // A searching member that is waiting on a path stays; with neither
        // flag it is dropped as well.
        e.mem.set_u8(idle.addr() + 0x3d, 0);
        e.mem.set_u8(idle.addr() + 0x3e, 1);
        e.set(group, CombatGroup::pSearchingMember, idle);
        fn_0098a580(&mut e, group);
        assert_eq!(e.get(group, CombatGroup::pSearchingMember), idle);
        e.mem.set_u8(idle.addr() + 0x3e, 0);
        fn_0098a580(&mut e, group);
        assert!(e.get(group, CombatGroup::pSearchingMember).is_null());
    }

    #[test]
    fn search_update_starts_a_search_when_every_target_is_old_enough() {
        let mut e = search_start_engine();
        e.register(SEARCH_RUNNING, |e, a| {
            (e.mem.u32(a[0] + 0xd0) != 0).into_ret()
        });
        e.register(TIMER_EXPIRED, |e, a| e.mem.u8(a[0]).into_ret());
        e.set_global(0x011c_e528u32, 5.0f32);
        let group = search_start_group(&mut e);
        e.set(group, CombatGroup::iInitializedMemberCount, 2);
        // Ages 40 and 20 are at least 5; nobody has attackers yet.
        fn_0098a580(&mut e, group);
        assert_eq!(e.get(group, CombatGroup::iSearchCount), 0);
        // One target has an attacker: the search starts.
        e.set(target_entry(&e, group, 1), CombatTarget::sAttackerCount, 1);
        fn_0098a580(&mut e, group);
        assert_eq!(e.get(group, CombatGroup::iSearchCount), 1);
        // A target detected too recently (age 2 < 5) prevents it.
        let group = search_start_group(&mut e);
        e.set(group, CombatGroup::iInitializedMemberCount, 2);
        e.set(target_entry(&e, group, 1), CombatTarget::sAttackerCount, 1);
        e.set(
            target_entry(&e, group, 0),
            CombatTarget::fLastDetectedTimeStamp,
            98.0f32,
        );
        e.call(0x0098_a580, &args![group]);
        assert_eq!(e.get(group, CombatGroup::iSearchCount), 0);
        // Everyone fleeing: "ready", but no search is running, so nothing
        // is started.
        let ready = search_start_group(&mut e);
        e.set(ready, CombatGroup::iInitializedMemberCount, 2);
        e.set(ready, CombatGroup::iFleeingMemberCount, 2);
        fn_0098a580(&mut e, ready);
        assert_eq!(e.get(ready, CombatGroup::iSearchCount), 0);
    }

    #[test]
    fn search_update_builds_the_grid_and_starts_dialogue() {
        let mut e = search_step_engine();
        e.set_global(0x011c_f42cu32, 6.5f32);
        e.set_global(0x011c_e43cu32, 1.5f32);
        e.set_global(PATH_PROGRESS_LIMIT, 0.99f64);
        let space = e.mem.alloc(8);
        let (m0, m1, victim) = (actor(&mut e, 1), actor(&mut e, 2), actor(&mut e, 3));
        place_actor(&mut e, m0, 0, 1.0, space);
        place_actor(&mut e, m1, 0, 2.0, space);
        e.mem.set_u8(m0.addr() + 0x3f, 1);
        e.mem.set_f32(m0.addr() + 0x48, 1.25);
        e.mem.set_u32(m1.addr() + 0x4c, victim.addr());
        let group = group_with(&mut e, &[victim], &[m0, m1]);
        let g = group.addr();
        e.set(group, CombatGroup::iSearchCount, 1);
        e.set(group, CombatGroup::fSearchRadius, 33.0f32);
        e.mem.set_u32(g + 0xfc, space);
        // The search timer has expired; the area timer has not; the area is
        // loaded; there is no grid yet.
        e.mem.set_u8(g + 0xd8, 1);
        e.register(WORLD_LOCATION_IS_LOADED, |_, _| true.into_ret());
        e.set(
            target_entry(&e, group, 0),
            CombatTarget::fLastDetectedTimeStamp,
            90.0f32,
        );
        e.call_log = Some(vec![]);
        fn_0098a580(&mut e, group);
        // The grid exists now, was given the radius, and has the setting.
        let grid = e.mem.u32(g + 0xd4);
        assert_ne!(grid, 0);
        assert_eq!(e.mem.f32(grid + 0x48), 6.5);
        assert!(e.get(group, CombatGroup::bUpdateSearchDebugGeometry));
        let updates = calls(&e, LOS_GRID_MAP_UPDATE);
        assert_eq!(updates.len(), 1);
        assert_eq!(updates[0][0], grid);
        assert_eq!(f32::from_bits(updates[0][2]), 33.0);
        // m0's location (with information) and clamped heading go to the grid.
        let applied = calls(&e, LOS_GRID_MAP_APPLY);
        assert_eq!(applied.len(), 1);
        assert_eq!(applied[0][0], g + 0xd4);
        assert_eq!(calls(&e, LOCATION_ARRAY_ADD).len(), 1);
        // The search timer restarts with the setting.
        let starts = calls(&e, TIMER_START);
        assert_eq!(starts.last().unwrap()[0], g + 0xd8);
        assert_eq!(f32::from_bits(starts.last().unwrap()[1]), 1.5);
        // Dialogue: the member picked by the random index (the last, m1)
        // speaks to its target; the target was detected, so flag 1.
        assert_eq!(
            calls(&e, START_DIALOGUE),
            vec![vec![0x4444, m1.addr(), victim.addr(), 4, 1, 0, 0]]
        );
        // The target was never detected: flag 0.
        e.set(
            target_entry(&e, group, 0),
            CombatTarget::fLastDetectedTimeStamp,
            -f32::MAX,
        );
        e.mem.set_u8(g + 0xd8, 1);
        fn_0098a580(&mut e, group);
        assert_eq!(calls(&e, START_DIALOGUE)[1][4], 0);
        // The player never speaks.
        e.set_global(PLAYER, m1.addr());
        e.mem.set_u8(g + 0xd8, 1);
        fn_0098a580(&mut e, group);
        assert_eq!(calls(&e, START_DIALOGUE).len(), 2);
    }

    #[test]
    fn search_update_advances_or_drops_an_existing_grid() {
        let mut e = search_step_engine();
        e.set_global(PATH_PROGRESS_LIMIT, 0.99f64);
        e.set_global(0x011c_f870u32, 4.0f32);
        let space = e.mem.alloc(8);
        let m0 = actor(&mut e, 1);
        place_actor(&mut e, m0, 0, 1.0, space);
        let group = group_with(&mut e, &[], &[m0]);
        let g = group.addr();
        let grid = e.mem.alloc(0x60);
        e.mem.set_u32(g + 0xd4, grid);
        e.set(group, CombatGroup::iSearchCount, 1);
        e.mem.set_u32(g + 0xfc, space);
        e.mem.set_u8(g + 0xd8, 1);
        e.mem.set_u8(g + 0xe0, 1);
        // The grid has travelled 1.5 (above 0.99): it steps once more, the
        // search count goes up and the area timer expires at once. The area
        // timer being expired, the area is refreshed.
        e.register(PATH_HANDLER_DISTANCE_TRAVELLED, |_, _| 1.5f32.into_ret());
        e.register(WORLD_LOCATION_IS_LOADED, |_, _| true.into_ret());
        record(
            &mut e,
            &[
                POINTER_ARRAY_CONSTRUCT,
                POINTER_ARRAY_DESTRUCT,
                COLLECT_TELEPORT_DOORS,
                ARRAY_SORT,
                WORLD_LOCATION_DISTANCE,
            ],
        );
        put_radius_settings(&mut e, 10.0, 30.0);
        give_search_buffers(&mut e, group);
        fn_0098a580(&mut e, group);
        assert_eq!(calls(&e, LOS_GRID_MAP_STEP), vec![vec![grid]]);
        assert_eq!(e.get(group, CombatGroup::iSearchCount), 2);
        assert_eq!(calls(&e, TIMER_EXPIRE_NOW), vec![vec![g + 0xe0]]);
        // The area is not loaded: the grid is dropped.
        e.register(WORLD_LOCATION_IS_LOADED, |_, _| false.into_ret());
        e.mem.set_u8(g + 0xd8, 1);
        e.mem.set_u8(g + 0xe0, 0);
        e.register(PATH_HANDLER_DISTANCE_TRAVELLED, |_, _| 0.5f32.into_ret());
        e.call_log = Some(vec![]);
        fn_0098a580(&mut e, group);
        assert_eq!(e.mem.u32(g + 0xd4), 0);
        assert_eq!(calls(&e, NI_POINTER_ASSIGN), vec![vec![g + 0xd4, 0]]);
        assert!(calls(&e, LOS_GRID_MAP_STEP).is_empty());
    }

    // ---- Part 3: LoadGame, InitLoadGame and the BSSimpleArray instances ----

    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::rc::Rc;

    /// Vtable of the fake arrays: slot `+4` allocates `count * 0x70` bytes.
    const ARRAY_VTABLE: u32 = 0x7300_0000;
    /// The fake allocator.
    const ALLOCATOR: u32 = 0x7300_0100;

    /// `engine()` plus the array helpers doubled over memory: the allocator
    /// behind the fake vtable, the minimum, element destruction (nothing),
    /// and the three tail movers (a byte copy of `count` elements).
    fn bsarray_engine() -> Engine {
        let mut e = engine();
        e.put_vtable(ARRAY_VTABLE, &[0, ALLOCATOR]);
        e.register(ALLOCATOR, |e, a| e.mem.alloc(a[1].max(1) * 0x70).into_ret());
        e.register(ARRAY_MOVE_COUNT, |_, a| a[0].min(a[1]).into_ret());
        e.register(ARRAY_DESTRUCT_ELEMENTS, |_, _| Ret::default());
        fn move_elements(e: &mut Engine, a: &[u32], size: u32) -> Ret {
            for i in 0..a[3] * size {
                let byte = e.mem.u8(a[2] + i);
                e.mem.set_u8(a[1] + i, byte);
            }
            Ret::default()
        }
        e.register(TARGET_ARRAY_INSTANCE.move_tail, |e, a| {
            move_elements(e, a, 0x68)
        });
        e.register(MEMBER_ARRAY_INSTANCE.move_tail, |e, a| {
            move_elements(e, a, 0x14)
        });
        e.register(SEARCH_LOCATION_ARRAY_INSTANCE.move_tail, |e, a| {
            move_elements(e, a, 0x1c)
        });
        e
    }

    /// An array object with the fake vtable and the given buffer, size and
    /// reserve.
    fn fake_array(e: &mut Engine, buffer: u32, size: u32, reserved: u32) -> Ptr<BSSimpleArray> {
        let array = e.new_object::<BSSimpleArray>();
        e.mem.set_u32(array.addr(), ARRAY_VTABLE);
        e.set(array, BSSimpleArray::pBuffer, buffer);
        e.set(array, BSSimpleArray::iSize, size);
        e.set(array, BSSimpleArray::iReservedSize, reserved);
        array
    }

    #[test]
    fn fn_0098e980_reads_the_strategy_table() {
        let mut e = engine();
        e.set_global(STRATEGY_TABLE + 8, 0xabcdu32);
        assert_eq!(fn_0098e980(&mut e, 2), 0xabcd);
        assert_eq!(fn_0098e980(&mut e, 0), 0);
    }

    #[test]
    fn init_load_game_resolves_actors_and_drops_the_missing_ones() {
        let mut e = bsarray_engine();
        // The form lookup keeps the word; the cast turns the "gone" actor
        // (address kept in a test global) into null and keeps the rest.
        e.register(FORM_LOOKUP, |_, a| a[0].into_ret());
        e.register(RT_DYNAMIC_CAST, |e, a| {
            (if a[0] == e.global::<u32>(0x0101_2100) {
                0
            } else {
                a[0]
            })
            .into_ret()
        });
        record(
            &mut e,
            &[
                MANAGER_TARGET_LOCAL_ACTOR,
                MANAGER_REMOVE_MEMBER,
                UNTINT_SCENEGRAPH,
            ],
        );
        e.set_global(COMBAT_MANAGER, 0x4242u32);
        e.register(SEARCH_RUNNING, |_, _| true.into_ret());
        let a = actor(&mut e, 1);
        let b = actor(&mut e, 2);
        let gone = actor(&mut e, 3);
        let searching = actor(&mut e, 4);
        e.set_global(0x0101_2100, gone.addr());
        let group = group_with(&mut e, &[a, gone, b], &[gone, a, gone]);
        let g = group.addr();
        e.set(group, CombatGroup::pSearchingMember, searching);
        combat_group_init_load_game(&mut e, group, 0x77);
        // Targets [a, b] remain, each announced to the combat manager once.
        assert_eq!(e.mem.u32(g + 0x10), 2);
        let targets = e.mem.u32(g + 0x0c);
        assert_eq!(e.mem.u32(targets), a.addr());
        assert_eq!(e.mem.u32(targets + 0x68), b.addr());
        assert_eq!(
            calls(&e, MANAGER_TARGET_LOCAL_ACTOR),
            vec![vec![0x4242, a.addr(), g], vec![0x4242, b.addr(), g]]
        );
        // Members: only `a` remains (the removal re-checks the moved entry).
        assert_eq!(e.mem.u32(g + 0x20), 1);
        assert_eq!(e.mem.u32(e.mem.u32(g + 0x1c)), a.addr());
        // The running search keeps its (cast) searching member.
        assert_eq!(e.get(group, CombatGroup::pSearchingMember), searching);
        // No search: the searching member word is left alone.
        e.register(SEARCH_RUNNING, |_, _| false.into_ret());
        e.set(group, CombatGroup::pSearchingMember, gone);
        combat_group_init_load_game(&mut e, group, 0x77);
        assert_eq!(e.get(group, CombatGroup::pSearchingMember), gone);
        // A running search with no searching member keeps null.
        e.register(SEARCH_RUNNING, |_, _| true.into_ret());
        e.set(group, CombatGroup::pSearchingMember, Ptr::NULL);
        combat_group_init_load_game(&mut e, group, 0x77);
        assert!(e.get(group, CombatGroup::pSearchingMember).is_null());
    }

    #[test]
    fn fn_0098eb30_removes_members_without_this_group() {
        let mut e = bsarray_engine();
        e.map(0x011d_e000, 0x1000);
        record(&mut e, &[MANAGER_REMOVE_MEMBER]);
        e.register(PROCESS_WORD_80, |e, a| e.mem.u32(a[0] + 0x80).into_ret());
        let player = actor(&mut e, 1);
        let ours = actor(&mut e, 2);
        let without_process = actor(&mut e, 3);
        let other_group = actor(&mut e, 4);
        let group = group_with(&mut e, &[], &[player, ours, without_process, other_group]);
        e.set_global(PLAYER, player.addr());
        for (who, owner) in [(ours, group.addr()), (other_group, 0x9999)] {
            let process = e.mem.alloc(0x90);
            e.mem.set_u32(process + 0x80, owner);
            e.mem.set_u32(who.addr() + 0x38, process);
        }
        // `fn_0098cf70` (real) has no targets to recount; the player has no
        // process but is skipped.
        fn_0098eb30(&mut e, group, 0);
        assert_eq!(e.mem.u32(group.addr() + 0x20), 2);
        let buffer = e.mem.u32(group.addr() + 0x1c);
        assert_eq!(e.mem.u32(buffer), player.addr());
        assert_eq!(e.mem.u32(buffer + 0x14), ours.addr());
        let removed: Vec<u32> = calls(&e, MANAGER_REMOVE_MEMBER)
            .iter()
            .map(|c| c[2])
            .collect();
        assert_eq!(removed, vec![without_process.addr(), other_group.addr()]);
    }

    #[test]
    fn fn_0098ebc0_forwards_to_the_location_slot_accessor() {
        let mut e = engine();
        e.register(SEARCH_LOCATION_SLOT, |_, a| {
            (a[0] + 0x100 + a[1]).into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(fn_0098ebc0(&mut e, Ptr::new(0x40), 3), 0x143);
        assert_eq!(calls(&e, SEARCH_LOCATION_SLOT), vec![vec![0x40, 3]]);
    }

    #[test]
    fn fn_0098ebe0_clears_removes_in_place_or_moves_to_a_smaller_buffer() {
        let mut e = bsarray_engine();
        record(
            &mut e,
            &[
                ARRAY_CLEAR,
                POINTER_ARRAY_SHOULD_SHRINK,
                POINTER_ARRAY_SHRUNK_RESERVE,
                ARRAY_COPY_ELEMENTS,
                ARRAY_RELEASE_BUFFER,
            ],
        );
        // The count reaches the size: cleared, size untouched.
        let buffer = e.mem.alloc(32);
        let array = fake_array(&mut e, buffer, 3, 4);
        fn_0098ebe0(&mut e, array, 1, 3, 1);
        assert_eq!(calls(&e, ARRAY_CLEAR), vec![vec![array.addr(), 1]]);
        assert_eq!(e.get(array, BSSimpleArray::iSize), 3);
        // In place (no shrink): destroy `count` slots, copy the tail down.
        e.call_log = Some(vec![]);
        fn_0098ebe0(&mut e, array, 1, 1, 0);
        assert_eq!(
            calls(&e, ARRAY_DESTRUCT_ELEMENTS),
            vec![vec![array.addr(), buffer + 4, 1]]
        );
        assert_eq!(
            calls(&e, ARRAY_COPY_ELEMENTS),
            vec![vec![array.addr(), buffer + 4, buffer + 8, 1]]
        );
        assert_eq!(e.get(array, BSSimpleArray::iSize), 2);
        assert!(calls(&e, POINTER_ARRAY_SHOULD_SHRINK).is_empty());
        // Shrink requested but refused by `006f3170`: still in place.
        e.call_log = Some(vec![]);
        fn_0098ebe0(&mut e, array, 0, 1, 1);
        assert_eq!(calls(&e, POINTER_ARRAY_SHOULD_SHRINK).len(), 1);
        assert!(calls(&e, ARRAY_RELEASE_BUFFER).is_empty());
        assert_eq!(e.get(array, BSSimpleArray::iSize), 1);
        // Shrink accepted: a new buffer of the size `00869600` asks for.
        e.register(POINTER_ARRAY_SHOULD_SHRINK, |_, _| true.into_ret());
        e.register(POINTER_ARRAY_SHRUNK_RESERVE, |_, _| 5u32.into_ret());
        let array = fake_array(&mut e, buffer, 4, 8);
        e.call_log = Some(vec![]);
        fn_0098ebe0(&mut e, array, 1, 1, 1);
        let new_buffer = e.get(array, BSSimpleArray::pBuffer);
        assert_ne!(new_buffer, buffer);
        assert_eq!(e.get(array, BSSimpleArray::iReservedSize), 5);
        assert_eq!(e.get(array, BSSimpleArray::iSize), 3);
        assert_eq!(
            calls(&e, ARRAY_COPY_ELEMENTS),
            vec![
                vec![array.addr(), new_buffer, buffer, 1],
                vec![array.addr(), new_buffer + 4, buffer + 8, 3],
            ]
        );
        assert_eq!(calls(&e, ARRAY_RELEASE_BUFFER), vec![vec![array.addr()]]);
    }

    /// Runs the `SetSize` of one instance through its cases.
    fn check_set_size(
        set_size: fn(&mut Engine, Ptr<BSSimpleArray>, u32, u8),
        kind: &ArrayInstance,
    ) {
        let mut e = bsarray_engine();
        record(&mut e, &[ARRAY_CLEAR, kind.grow, kind.construct]);
        let array = fake_array(&mut e, 0, 0, 0);
        let a = array.addr();
        // Zero clears.
        set_size(&mut e, array, 0, 1);
        assert_eq!(calls(&e, ARRAY_CLEAR), vec![vec![a, 1]]);
        // No buffer yet: allocate, construct all.
        e.call_log = Some(vec![]);
        set_size(&mut e, array, 3, 1);
        let buffer = e.get(array, BSSimpleArray::pBuffer);
        assert_ne!(buffer, 0);
        assert_eq!(calls(&e, ALLOCATOR), vec![vec![a, 3]]);
        assert_eq!(calls(&e, kind.construct), vec![vec![a, buffer, 3]]);
        assert_eq!(e.get(array, BSSimpleArray::iReservedSize), 3);
        assert_eq!(e.get(array, BSSimpleArray::iSize), 3);
        // Past the reserve: the instance's grow, then the new tail.
        e.call_log = Some(vec![]);
        set_size(&mut e, array, 5, 1);
        assert_eq!(calls(&e, kind.grow), vec![vec![a, 5, 3]]);
        let buffer = e.get(array, BSSimpleArray::pBuffer);
        assert_eq!(
            calls(&e, kind.construct),
            vec![vec![a, buffer + 3 * kind.element, 2]]
        );
        assert_eq!(e.get(array, BSSimpleArray::iReservedSize), 5);
        // Within the reserve: construct only the extra element.
        e.set(array, BSSimpleArray::iSize, 3);
        e.call_log = Some(vec![]);
        set_size(&mut e, array, 4, 1);
        assert!(calls(&e, kind.grow).is_empty());
        assert_eq!(
            calls(&e, kind.construct),
            vec![vec![a, buffer + 3 * kind.element, 1]]
        );
        assert_eq!(e.get(array, BSSimpleArray::iSize), 4);
        // Shrinking destroys the dropped elements; the reserve only
        // follows when the new size is at most a quarter of it.
        e.call_log = Some(vec![]);
        set_size(&mut e, array, 2, 1);
        assert_eq!(
            calls(&e, ARRAY_DESTRUCT_ELEMENTS),
            vec![vec![a, buffer + 2 * kind.element, 2]]
        );
        assert!(calls(&e, kind.grow).is_empty());
        assert_eq!(e.get(array, BSSimpleArray::iReservedSize), 5);
        e.call_log = Some(vec![]);
        set_size(&mut e, array, 1, 0);
        assert!(calls(&e, kind.grow).is_empty());
        e.set(array, BSSimpleArray::iSize, 2);
        set_size(&mut e, array, 1, 1);
        assert_eq!(calls(&e, kind.grow), vec![vec![a, 1, 1]]);
        assert_eq!(e.get(array, BSSimpleArray::iReservedSize), 1);
        assert_eq!(e.get(array, BSSimpleArray::iSize), 1);
    }

    #[test]
    fn fn_0098ed80_sets_the_size_of_a_target_array() {
        check_set_size(fn_0098ed80, &TARGET_ARRAY_INSTANCE);
    }

    #[test]
    fn fn_0098f040_sets_the_size_of_a_member_array() {
        check_set_size(fn_0098f040, &MEMBER_ARRAY_INSTANCE);
    }

    #[test]
    fn fn_0098f2f0_sets_the_size_of_a_search_location_array() {
        check_set_size(fn_0098f2f0, &SEARCH_LOCATION_ARRAY_INSTANCE);
    }

    /// Runs the `RemoveAt` of one instance: five elements whose bytes are
    /// their index, remove one in the middle, then the last two.
    fn check_remove_at(
        remove_at: fn(&mut Engine, Ptr<BSSimpleArray>, u32, u32),
        kind: &ArrayInstance,
    ) {
        let mut e = bsarray_engine();
        let buffer = e.mem.alloc(kind.element * 5);
        for index in 0..5 {
            for byte in 0..kind.element {
                e.mem
                    .set_u8(buffer + index * kind.element + byte, index as u8 + 1);
            }
        }
        let array = fake_array(&mut e, buffer, 5, 8);
        record(&mut e, &[]);
        remove_at(&mut e, array, 1, 1);
        // Tail count min(5 - 1 - 1, 1) = 1: the last element fills the hole.
        assert_eq!(
            calls(&e, ARRAY_DESTRUCT_ELEMENTS),
            vec![vec![array.addr(), buffer + kind.element, 1]]
        );
        assert_eq!(calls(&e, ARRAY_MOVE_COUNT), vec![vec![3, 1]]);
        assert_eq!(e.get(array, BSSimpleArray::iSize), 4);
        assert_eq!(e.mem.u8(buffer + kind.element), 5);
        assert_eq!(e.mem.u8(buffer), 1);
        // Removing the last two: nothing left to move.
        e.call_log = Some(vec![]);
        remove_at(&mut e, array, 2, 2);
        assert_eq!(
            calls(&e, kind.move_tail),
            vec![vec![
                array.addr(),
                buffer + 2 * kind.element,
                buffer + 4 * kind.element,
                0
            ]]
        );
        assert_eq!(e.get(array, BSSimpleArray::iSize), 2);
    }

    #[test]
    fn fn_0098ef10_removes_target_elements() {
        check_remove_at(fn_0098ef10, &TARGET_ARRAY_INSTANCE);
    }

    #[test]
    fn fn_0098f1f0_removes_member_elements() {
        check_remove_at(fn_0098f1f0, &MEMBER_ARRAY_INSTANCE);
    }

    #[test]
    fn fn_0098f430_removes_search_location_elements() {
        check_remove_at(fn_0098f430, &SEARCH_LOCATION_ARRAY_INSTANCE);
    }

    #[test]
    fn fn_0098f7b0_reserves_the_next_slot_and_grows_a_full_array() {
        let mut e = bsarray_engine();
        record(
            &mut e,
            &[
                ARRAY_IS_FULL,
                ARRAY_GROWN_RESERVE,
                TARGET_ARRAY_INSTANCE.grow,
            ],
        );
        // Room left: only the size moves.
        let array = fake_array(&mut e, 0x1000, 2, 4);
        assert_eq!(fn_0098f7b0(&mut e, array), 2);
        assert_eq!(e.get(array, BSSimpleArray::iSize), 3);
        assert!(calls(&e, ALLOCATOR).is_empty());
        // Full with no buffer: a first buffer of four.
        e.register(ARRAY_IS_FULL, |_, _| true.into_ret());
        let empty = fake_array(&mut e, 0, 0, 0);
        assert_eq!(fn_0098f7b0(&mut e, empty), 0);
        assert_eq!(calls(&e, ALLOCATOR), vec![vec![empty.addr(), 4]]);
        assert_eq!(e.get(empty, BSSimpleArray::iReservedSize), 4);
        assert_ne!(e.get(empty, BSSimpleArray::pBuffer), 0);
        assert_eq!(e.get(empty, BSSimpleArray::iSize), 1);
        // Full with a buffer: `009a3910` picks the new reserve.
        e.register(ARRAY_GROWN_RESERVE, |_, _| 8u32.into_ret());
        let full = fake_array(&mut e, 0x1000, 4, 4);
        assert_eq!(fn_0098f7b0(&mut e, full), 4);
        assert_eq!(
            calls(&e, TARGET_ARRAY_INSTANCE.grow),
            vec![vec![full.addr(), 8, 4]]
        );
        assert_eq!(e.get(full, BSSimpleArray::iReservedSize), 8);
        assert_eq!(e.get(full, BSSimpleArray::iSize), 5);
    }

    #[test]
    fn fn_0098eec0_adds_a_target_element() {
        let mut e = bsarray_engine();
        record(
            &mut e,
            &[
                ARRAY_IS_FULL,
                ARRAY_GROWN_RESERVE,
                TARGET_ARRAY_INSTANCE.construct,
            ],
        );
        let buffer = e.mem.alloc(0x68 * 4);
        let array = fake_array(&mut e, buffer, 1, 4);
        let element = e.new_object::<CombatTarget>();
        for word in 0..0x1a {
            e.mem.set_u32(element.addr() + 4 * word, 0x100 + word);
        }
        assert_eq!(fn_0098eec0(&mut e, array, element), 1);
        assert_eq!(e.get(array, BSSimpleArray::iSize), 2);
        assert_eq!(
            calls(&e, TARGET_ARRAY_INSTANCE.construct),
            vec![vec![array.addr(), buffer + 0x68, 1]]
        );
        for word in 0..0x1a {
            assert_eq!(e.mem.u32(buffer + 0x68 + 4 * word), 0x100 + word);
        }
        // The word before it is untouched.
        assert_eq!(e.mem.u32(buffer + 0x68 - 4), 0);
    }

    #[test]
    fn fn_0098f180_adds_a_member_element() {
        let mut e = bsarray_engine();
        record(&mut e, &[MEMBER_ARRAY_INSTANCE.construct]);
        e.register(MEMBER_ARRAY_RESERVE_SLOT, |e, a| {
            let size = e.mem.u32(a[0] + 8);
            e.mem.set_u32(a[0] + 8, size + 1);
            size.into_ret()
        });
        let buffer = e.mem.alloc(0x14 * 4);
        let array = fake_array(&mut e, buffer, 2, 4);
        let element = e.new_object::<CombatMember>();
        for word in 0..5 {
            e.mem.set_u32(element.addr() + 4 * word, 0x200 + word);
        }
        assert_eq!(fn_0098f180(&mut e, array, element), 2);
        assert_eq!(e.get(array, BSSimpleArray::iSize), 3);
        assert_eq!(
            calls(&e, MEMBER_ARRAY_INSTANCE.construct),
            vec![vec![array.addr(), buffer + 0x28, 1]]
        );
        for word in 0..5 {
            assert_eq!(e.mem.u32(buffer + 0x28 + 4 * word), 0x200 + word);
        }
    }

    #[test]
    fn target_array_allocate_and_reallocate_scale_by_the_element_size() {
        let mut e = engine();
        e.register(OPERATOR_NEW_SMALL, |_, a| (a[0] + 1).into_ret());
        e.register(ARRAY_REALLOC_BYTES, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(
            bs_simple_array_combat_target_allocate(&mut e, Ptr::NULL, 3),
            0x68 * 3 + 1
        );
        bs_simple_array_combat_target_reallocate(&mut e, Ptr::NULL, 0x5000, 2);
        fn_0098f280(&mut e, Ptr::NULL, 0x6000, 2);
        assert_eq!(
            calls(&e, ARRAY_REALLOC_BYTES),
            vec![vec![0x5000, 0xd0], vec![0x6000, 0x28]]
        );
    }

    /// A constructor: sets the vtable and, with the initialiser doubled,
    /// calls it.
    fn check_constructor(
        constructor: fn(&mut Engine, Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>,
        vtable: u32,
    ) {
        let mut e = engine();
        record(
            &mut e,
            &[SEARCH_DOOR_ARRAY_INIT, CLUSTER_POINTER_ARRAY_INIT],
        );
        let array = e.new_object::<BSSimpleArray>();
        for word in 1..4 {
            e.mem.set_u32(array.addr() + 4 * word, 0xdead);
        }
        assert_eq!(constructor(&mut e, array), array);
        assert_eq!(e.mem.u32(array.addr()), vtable);
        // The arrays with their own `SetSize` initialise themselves empty.
        if vtable == SEARCH_DOOR_ARRAY_VTABLE || vtable == CLUSTER_POINTER_ARRAY_VTABLE {
            let init = if vtable == SEARCH_DOOR_ARRAY_VTABLE {
                SEARCH_DOOR_ARRAY_INIT
            } else {
                CLUSTER_POINTER_ARRAY_INIT
            };
            assert_eq!(calls(&e, init), vec![vec![array.addr(), 0, 0]]);
        } else {
            for word in 1..4 {
                assert_eq!(e.mem.u32(array.addr() + 4 * word), 0);
            }
        }
    }

    #[test]
    fn fn_0098ed30_constructs_a_target_array() {
        check_constructor(fn_0098ed30, TARGET_ARRAY_VTABLE);
    }

    #[test]
    fn fn_0098eff0_constructs_a_member_array() {
        check_constructor(fn_0098eff0, MEMBER_ARRAY_VTABLE);
    }

    #[test]
    fn fn_0098f2a0_constructs_a_search_location_array() {
        check_constructor(fn_0098f2a0, SEARCH_LOCATION_ARRAY_VTABLE);
    }

    #[test]
    fn fn_0098f4c0_constructs_a_search_door_array() {
        check_constructor(fn_0098f4c0, SEARCH_DOOR_ARRAY_VTABLE);
    }

    #[test]
    fn fn_0098f510_constructs_a_cluster_pointer_array() {
        check_constructor(fn_0098f510, CLUSTER_POINTER_ARRAY_VTABLE);
    }

    /// A destructor: sets the vtable, then clears the array with flag 1.
    fn check_destructor(destructor: fn(&mut Engine, Ptr<BSSimpleArray>), vtable: u32) {
        let mut e = engine();
        record(&mut e, &[ARRAY_CLEAR]);
        let array = e.new_object::<BSSimpleArray>();
        destructor(&mut e, array);
        assert_eq!(e.mem.u32(array.addr()), vtable);
        assert_eq!(calls(&e, ARRAY_CLEAR), vec![vec![array.addr(), 1]]);
    }

    #[test]
    fn fn_0098ed60_destroys_a_target_array() {
        check_destructor(fn_0098ed60, TARGET_ARRAY_VTABLE);
    }

    #[test]
    fn fn_0098f020_destroys_a_member_array() {
        check_destructor(fn_0098f020, MEMBER_ARRAY_VTABLE);
    }

    #[test]
    fn fn_0098f2d0_destroys_a_search_location_array() {
        check_destructor(fn_0098f2d0, SEARCH_LOCATION_ARRAY_VTABLE);
    }

    #[test]
    fn fn_0098f4f0_destroys_a_search_door_array() {
        check_destructor(fn_0098f4f0, SEARCH_DOOR_ARRAY_VTABLE);
    }

    #[test]
    fn fn_0098f540_destroys_a_cluster_pointer_array() {
        check_destructor(fn_0098f540, CLUSTER_POINTER_ARRAY_VTABLE);
    }

    #[test]
    fn fn_0098f560_destroys_a_group_member_data_array() {
        check_destructor(fn_0098f560, GROUP_MEMBER_DATA_ARRAY_VTABLE);
    }

    /// A scalar deleting destructor: destructs (vtable set), then frees only
    /// with bit 0 of the flags; returns `this`.
    fn check_scalar_deleting(
        function: fn(&mut Engine, Ptr<BSSimpleArray>, u32) -> Ptr<BSSimpleArray>,
        vtable: u32,
    ) {
        let mut e = engine();
        record(&mut e, &[ARRAY_CLEAR, OPERATOR_DELETE]);
        let array = e.new_object::<BSSimpleArray>();
        assert_eq!(function(&mut e, array, 0), array);
        assert_eq!(e.mem.u32(array.addr()), vtable);
        assert_eq!(calls(&e, ARRAY_CLEAR), vec![vec![array.addr(), 1]]);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        assert_eq!(function(&mut e, array, 3), array);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![array.addr()]]);
    }

    #[test]
    fn target_array_scalar_deleting_destructor() {
        check_scalar_deleting(
            bs_simple_array_combat_target_scalar_deleting_destructor,
            TARGET_ARRAY_VTABLE,
        );
    }

    #[test]
    fn member_array_scalar_deleting_destructor() {
        check_scalar_deleting(
            bs_simple_array_combat_member_scalar_deleting_destructor,
            MEMBER_ARRAY_VTABLE,
        );
    }

    #[test]
    fn search_location_array_scalar_deleting_destructor() {
        check_scalar_deleting(
            bs_simple_array_combat_search_location_scalar_deleting_destructor,
            SEARCH_LOCATION_ARRAY_VTABLE,
        );
    }

    #[test]
    fn search_door_array_scalar_deleting_destructor() {
        check_scalar_deleting(
            bs_simple_array_combat_search_door_scalar_deleting_destructor,
            SEARCH_DOOR_ARRAY_VTABLE,
        );
    }

    #[test]
    fn cluster_pointer_array_scalar_deleting_destructor() {
        check_scalar_deleting(
            bs_simple_array_combat_group_cluster_pointer_scalar_deleting_destructor,
            CLUSTER_POINTER_ARRAY_VTABLE,
        );
    }

    #[test]
    fn group_member_data_array_scalar_deleting_destructor() {
        check_scalar_deleting(
            bs_simple_array_group_member_data_scalar_deleting_destructor,
            GROUP_MEMBER_DATA_ARRAY_VTABLE,
        );
    }

    /// An initialiser: empties the array, then reserves `max(reserve,
    /// count)` and constructs `count` elements.
    fn check_init(init: fn(&mut Engine, Ptr<BSSimpleArray>, u32, u32), construct: u32) {
        let mut e = bsarray_engine();
        record(&mut e, &[construct]);
        let array = fake_array(&mut e, 0xdead, 7, 9);
        let a = array.addr();
        init(&mut e, array, 0, 0);
        assert_eq!(e.get(array, BSSimpleArray::pBuffer), 0);
        assert_eq!(e.get(array, BSSimpleArray::iSize), 0);
        assert_eq!(e.get(array, BSSimpleArray::iReservedSize), 0);
        assert!(calls(&e, ALLOCATOR).is_empty());
        // A reserve without elements.
        init(&mut e, array, 2, 0);
        assert_eq!(calls(&e, ALLOCATOR), vec![vec![a, 2]]);
        assert_eq!(e.get(array, BSSimpleArray::iReservedSize), 2);
        assert_eq!(e.get(array, BSSimpleArray::iSize), 0);
        assert!(calls(&e, construct).is_empty());
        // Elements beyond the reserve: the reserve is raised to the count.
        e.call_log = Some(vec![]);
        init(&mut e, array, 1, 3);
        let buffer = e.get(array, BSSimpleArray::pBuffer);
        assert_eq!(calls(&e, ALLOCATOR), vec![vec![a, 3]]);
        assert_eq!(calls(&e, construct), vec![vec![a, buffer, 3]]);
        assert_eq!(e.get(array, BSSimpleArray::iReservedSize), 3);
        assert_eq!(e.get(array, BSSimpleArray::iSize), 3);
    }

    #[test]
    fn fn_0098fab0_initialises_a_target_array() {
        check_init(fn_0098fab0, TARGET_ARRAY_INSTANCE.construct);
    }

    #[test]
    fn fn_0098fbe0_initialises_a_member_array() {
        check_init(fn_0098fbe0, MEMBER_ARRAY_INSTANCE.construct);
    }

    #[test]
    fn fn_0098fd10_initialises_a_search_location_array() {
        check_init(fn_0098fd10, SEARCH_LOCATION_ARRAY_INSTANCE.construct);
    }

    #[test]
    fn fn_0098f580_copies_the_target_elements_into_another_array() {
        let mut e = bsarray_engine();
        record(&mut e, &[TARGET_ARRAY_INSTANCE.construct]);
        e.register(TARGET_ARRAY_ELEMENT, |e, a| {
            (e.mem.u32(a[0] + 4) + a[1] * 0x68).into_ret()
        });
        let buffer = e.mem.alloc(0x68 * 2);
        for word in 0..0x1a {
            e.mem.set_u32(buffer + 4 * word, 0x10 + word);
            e.mem.set_u32(buffer + 0x68 + 4 * word, 0x80 + word);
        }
        let source = fake_array(&mut e, buffer, 2, 2);
        let other = fake_array(&mut e, 0, 0, 0);
        fn_0098f580(&mut e, source, other);
        assert_eq!(e.get(other, BSSimpleArray::iSize), 2);
        let copy = e.get(other, BSSimpleArray::pBuffer);
        for word in 0..0x1a {
            assert_eq!(e.mem.u32(copy + 4 * word), 0x10 + word);
            assert_eq!(e.mem.u32(copy + 0x68 + 4 * word), 0x80 + word);
        }
        assert_eq!(
            calls(&e, TARGET_ARRAY_ELEMENT),
            vec![vec![other.addr(), 0], vec![other.addr(), 1]]
        );
    }

    #[test]
    fn fn_0098f5f0_constructs_a_scrap_array_with_the_given_or_the_scrap_heap() {
        let mut e = engine();
        record(&mut e, &[SCRAP_ARRAY_BASE_CONSTRUCT, SCRAP_ARRAY_INIT]);
        e.register(MEMORY_MANAGER, |_, _| 0x1111u32.into_ret());
        e.register(THREAD_SCRAP_HEAP, |_, a| (a[0] + 0x2222).into_ret());
        let object = e.mem.alloc(0x20);
        let this = Ptr::new(object);
        assert_eq!(fn_0098f5f0(&mut e, this, 7, 8, 0x5555), this);
        assert_eq!(e.mem.u32(object), SCRAP_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(object + 0x10), 0x5555);
        assert_eq!(calls(&e, SCRAP_ARRAY_BASE_CONSTRUCT), vec![vec![object]]);
        assert_eq!(calls(&e, SCRAP_ARRAY_INIT), vec![vec![object, 7, 8]]);
        assert!(calls(&e, MEMORY_MANAGER).is_empty());
        // Heap zero: the thread's scrap heap of the memory manager.
        fn_0098f5f0(&mut e, this, 1, 2, 0);
        assert_eq!(e.mem.u32(object + 0x10), 0x1111 + 0x2222);
        assert_eq!(calls(&e, THREAD_SCRAP_HEAP), vec![vec![0x1111]]);
    }

    /// Doubles every callee `combat_group_load_game` reaches and runs it.
    /// `sizes` are the counts `LoadVariableSizedValue` returns in turn;
    /// `bytes` the values written by the size-1 reads outside the group (a
    /// `None` leaves the byte); `strategy` the strategy byte the buffer
    /// holds; `search_count` the `iSearchCount` read.
    fn load_run(
        version: u32,
        sizes: &[u32],
        bytes: &[Option<u8>],
        strategy: u8,
        search_count: u32,
    ) -> (Engine, Ptr<CombatGroup>, Ptr) {
        let mut e = bsarray_engine();
        let group = e.new_object::<CombatGroup>();
        let g = group.addr();
        for offset in [0x08u32, 0x18, 0x110] {
            e.mem.set_u32(g + offset, ARRAY_VTABLE);
        }
        e.set_global(STRATEGY_TABLE + 4, 0x5151u32);
        e.set_global(STRATEGY_TABLE + 8, 0x5252u32);
        let buffer = buffer_with_version(&mut e, version);
        record(
            &mut e,
            &[
                ARRAY_CLEAR,
                TARGET_ARRAY_INSTANCE.construct,
                MEMBER_ARRAY_INSTANCE.construct,
                SEARCH_LOCATION_ARRAY_INSTANCE.construct,
                WORLD_LOCATION_CONSTRUCT,
                LOAD_FORM_ID,
                WORLD_LOCATION_LOAD,
                TIME_STAMP_LOAD,
                TIMER_LOAD,
                LOS_GRID_MAP_LOAD,
            ],
        );
        e.register(POINTER_ARRAY_SET_SIZE, |e, a| {
            let slots = e.mem.alloc(a[1] * 4 + 8);
            e.mem.set_u32(a[0] + 4, slots);
            Ret::default()
        });
        e.register(SEARCH_DOOR_RESIZE, |e, a| {
            let doors = e.mem.alloc(a[1] * 12 + 12);
            e.mem.set_u32(a[0] + 4, doors);
            Ret::default()
        });
        e.register(SEARCH_DOOR_ELEMENT, |e, a| {
            (e.mem.u32(a[0] + 4) + a[1] * 12).into_ret()
        });
        e.register(SEARCH_LOCATION_SLOT, |e, a| {
            (e.mem.u32(a[0] + 4) + a[1] * 0x1c).into_ret()
        });
        e.register(OPERATOR_NEW_SMALL, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(LOS_GRID_MAP_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(NI_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(NI_POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(FORM_LOOKUP, |_, a| (a[0] + 0x1000).into_ret());
        e.register(RT_DYNAMIC_CAST, |_, a| (a[0] + 0x10000).into_ret());
        let next_form = Rc::new(RefCell::new(0x100u32));
        e.register_double(LOAD_FORM_ID_VALUE, move |_, _| {
            let mut id = next_form.borrow_mut();
            *id += 1;
            (*id).into_ret()
        });
        let queue = Rc::new(RefCell::new(VecDeque::from(sizes.to_vec())));
        e.register_double(LOAD_SIZED_VALUE, move |_, _| {
            queue.borrow_mut().pop_front().unwrap().into_ret()
        });
        let outside = Rc::new(RefCell::new(VecDeque::from(bytes.to_vec())));
        e.register_double(LOAD_BYTES, move |e, a| {
            let (destination, size) = (a[1], a[2]);
            if destination == g + 0xd0 {
                e.mem.set_u32(destination, search_count);
            } else if size == 1 && e.mem.u8(destination) == 0xff {
                // The strategy byte (the code presets it to 0xff).
                e.mem.set_u8(destination, strategy);
            } else if size == 1 && !(g..g + 0x15c).contains(&destination) {
                if let Some(Some(value)) = outside.borrow_mut().pop_front() {
                    e.mem.set_u8(destination, value);
                }
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        combat_group_load_game(&mut e, group, buffer);
        (e, group, buffer)
    }

    #[test]
    fn load_game_without_a_search_reads_clusters_timers_and_strengths() {
        let (e, group, buffer) = load_run(14, &[2, 0, 0], &[], 1, 0);
        let g = group.addr();
        let b = buffer.addr();
        assert_eq!(
            calls(&e, POINTER_ARRAY_SET_SIZE),
            vec![vec![g + 0x138, 2, 1]]
        );
        assert_eq!(calls(&e, OPERATOR_NEW_SMALL), vec![vec![0x14], vec![0x14]]);
        let slots = e.mem.u32(g + 0x138 + 4);
        let first = e.mem.u32(slots);
        let second = e.mem.u32(slots + 4);
        assert!(first != 0 && second != 0 && first != second);
        // Each cluster is read by `fn_00985b10`: 0x10 bytes, then a count.
        let cluster_reads: Vec<Vec<u32>> = calls(&e, LOAD_BYTES)
            .into_iter()
            .filter(|c| c[1] == first || c[1] == first + 0x10)
            .collect();
        assert_eq!(
            cluster_reads,
            vec![vec![b, first, 0x10], vec![b, first + 0x10, 4]]
        );
        // The strategy byte (1) selects the table entry.
        assert_eq!(e.get(group, CombatGroup::pGroupStrategy), Ptr::new(0x5151));
        let timers: Vec<u32> = calls(&e, TIMER_LOAD)
            .iter()
            .map(|c| {
                assert_eq!(c[1], b);
                c[0] - g
            })
            .collect();
        let mut expected = vec![0x30u32, 0x38, 0x48, 0xb0, 0xa8];
        expected.extend((0..10).map(|i| 0x58 + 8 * i));
        expected.extend([0x50, 0xb8]);
        assert_eq!(timers, expected);
        let stamps: Vec<u32> = calls(&e, TIME_STAMP_LOAD)
            .iter()
            .map(|c| c[0] - g)
            .collect();
        assert_eq!(stamps, vec![0x44]);
        let group_reads: Vec<(u32, u32)> = calls(&e, LOAD_BYTES)
            .iter()
            .filter(|c| (g..g + 0x15c).contains(&c[1]))
            .map(|c| (c[1] - g, c[2]))
            .collect();
        assert_eq!(
            group_reads,
            vec![
                (0x2c, 1),
                (0x40, 4),
                (0xd0, 4),
                (0xc0, 4),
                (0xc4, 4),
                (0xc8, 4),
                (0xcc, 4),
                (0x154, 1)
            ]
        );
        // No search: none of its state is read and the flag stays off.
        assert!(calls(&e, LOAD_FORM_ID).is_empty());
        assert!(!e.get(group, CombatGroup::bUpdateSearchDebugGeometry));
        // Resized arrays are empty.
        assert_eq!(
            calls(&e, ARRAY_CLEAR),
            vec![vec![g + 8, 1], vec![g + 0x18, 1]]
        );
    }

    #[test]
    fn load_game_keeps_the_strategy_for_0xff_and_reads_targets_and_members() {
        let (e, group, buffer) = load_run(14, &[0, 1, 1], &[], 0xff, 0);
        let g = group.addr();
        let b = buffer.addr();
        assert_eq!(e.get(group, CombatGroup::pGroupStrategy), Ptr::NULL);
        // One target and one member were sized and read (the first word of
        // each is read through `LoadFormID`).
        assert_eq!(e.mem.u32(g + 0x10), 1);
        assert_eq!(e.mem.u32(g + 0x20), 1);
        let target = e.mem.u32(g + 0x0c);
        let member = e.mem.u32(g + 0x1c);
        let form_reads: Vec<Vec<u32>> = calls(&e, LOAD_FORM_ID);
        assert_eq!(form_reads, vec![vec![b, target], vec![b, member]]);
        // The target's two one-byte counts are read from inside it.
        let target_bytes: Vec<Vec<u32>> = calls(&e, LOAD_BYTES)
            .into_iter()
            .filter(|c| c[2] == 1 && (target..target + 0x68).contains(&c[1]))
            .collect();
        assert_eq!(
            target_bytes,
            vec![vec![b, target + 0x64, 1], vec![b, target + 0x65, 1]]
        );
    }

    #[test]
    fn load_game_with_a_search_reads_locations_doors_and_the_grid() {
        for version in [4u32, 5] {
            let (e, group, buffer) = load_run(
                version,
                &[0, 0, 0, 1, 1],
                &[None, None, None, Some(1)],
                0xff,
                1,
            );
            let g = group.addr();
            let b = buffer.addr();
            let entry = e.mem.u32(g + 0x110 + 4);
            let form_ids: Vec<Vec<u32>> = calls(&e, LOAD_FORM_ID);
            assert_eq!(form_ids, vec![vec![b, g + 0xec], vec![b, entry + 0x14]]);
            let stamps: Vec<u32> = calls(&e, TIME_STAMP_LOAD).iter().map(|c| c[0]).collect();
            let mut expected = vec![g + 0x44];
            if version >= 5 {
                expected.push(g + 0xe8);
            }
            expected.push(entry + 0x10);
            assert_eq!(stamps, expected, "version {version}");
            let timers: Vec<u32> = calls(&e, TIMER_LOAD).iter().map(|c| c[0] - g).collect();
            assert!(timers.contains(&0xd8) && timers.contains(&0xe0));
            assert_eq!(
                calls(&e, WORLD_LOCATION_LOAD),
                vec![vec![g + 0xf0, b], vec![entry, b]]
            );
            let reads: Vec<(u32, u32)> = calls(&e, LOAD_BYTES)
                .iter()
                .filter(|c| (g..g + 0x15c).contains(&c[1]))
                .map(|c| (c[1] - g, c[2]))
                .collect();
            assert!(reads.contains(&(0x100, 0xc)) && reads.contains(&(0x10c, 4)));
            assert_eq!(
                calls(&e, LOAD_BYTES)
                    .iter()
                    .filter(|c| c[1] == entry + 0x18)
                    .count(),
                1
            );
            // The door: two cast form IDs and three bytes.
            let door = e.mem.u32(g + 0x120 + 4);
            assert_eq!(e.mem.u32(door), 0x101 + 0x1000 + 0x10000);
            assert_eq!(e.mem.u32(door + 4), 0x102 + 0x1000 + 0x10000);
            for offset in [8u32, 9, 0xa] {
                assert!(calls(&e, LOAD_BYTES).contains(&vec![b, door + offset, 1]));
            }
            assert_eq!(calls(&e, SEARCH_DOOR_RESIZE), vec![vec![g + 0x120, 1, 1]]);
            // The grid flag was set: a 0x54-byte grid is built and loaded.
            let grid = e.mem.u32(g + 0xd4);
            assert_ne!(grid, 0);
            assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![0x54]]);
            assert_eq!(calls(&e, LOS_GRID_MAP_LOAD), vec![vec![grid, b]]);
            assert!(e.get(group, CombatGroup::bUpdateSearchDebugGeometry));
        }
    }

    #[test]
    fn load_game_with_a_search_and_no_grid_flag_builds_no_grid() {
        let (e, group, _) = load_run(5, &[0, 0, 0, 0, 0], &[Some(0)], 0xff, 1);
        assert_eq!(e.mem.u32(group.addr() + 0xd4), 0);
        assert!(calls(&e, OPERATOR_NEW).is_empty());
        assert!(calls(&e, LOS_GRID_MAP_LOAD).is_empty());
        assert!(e.get(group, CombatGroup::bUpdateSearchDebugGeometry));
    }

    #[test]
    fn target_and_member_reallocations_answer_the_new_buffer() {
        let mut e = engine();
        e.register(ARRAY_REALLOC_BYTES, |_, a| (a[0] + a[1]).into_ret());
        assert_eq!(
            bs_simple_array_combat_target_reallocate(&mut e, Ptr::NULL, 0x5000, 2),
            0x50d0
        );
        assert_eq!(fn_0098f280(&mut e, Ptr::NULL, 0x6000, 2), 0x6028);
    }
}
