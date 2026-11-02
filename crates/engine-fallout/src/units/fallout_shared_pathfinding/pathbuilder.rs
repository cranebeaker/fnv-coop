//! `fallout shared/pathfinding/pathbuilder.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The unit holds `PathBuilder` (Xbox PDB, 0x34 bytes): it turns a
//! `PathingRequest` into the nodes of a `PathingSolution`. First a coarse
//! path of `VirtualPathingNode`s (one per navmesh info, door or location,
//! `fn_006c9920` and its siblings, `PathBuilder::BuildNavMeshInfoPath`,
//! `PathBuilder::BuildTeleportDoorPath`), then the detailed path of
//! `PathingNode`s between the loaded virtual nodes (`fn_006ca0e0`,
//! `fn_006ca850`, `fn_006cb950`, `fn_006cc5e0`). The rest of the file are
//! the helpers those use (ray-cast requests, line-of-sight filters, array
//! template instances).
//!
//! How the translations read the game code:
//! - The game keeps many objects on its stack (searches, arrays, locations,
//!   points) and passes their addresses. Each function takes one zeroed
//!   block for its frame ([`with_frame`]) and addresses the locals by the
//!   `EBP`-relative offset the code uses (`f.at(-0x20ac)`).
//! - The compiler's exception-unwinding frames (`FS:[0]`) and the security
//!   cookie the prologues mix into them are not translated; every function
//!   that has one carries a `Translation incomplete` marker. Every exit path
//!   destroys the live locals in the order the game does.
//! - Callees outside this file are called by address, with the argument
//!   words read from the disassembly (`this` first, then the stack words in
//!   push order reversed). Functions of this file call each other directly.
//! - x87 comparisons: `TEST AH,0x41` after `FCOMP` continues only when
//!   `ST0` is greater (`a > b`), `TEST AH,0x5` + `JP` only when it is less
//!   (`a < b`); both are false for NaN, as in Rust. Values the game keeps
//!   in `ST0` or stores as `double` are compared as `f64`; `float` loads and
//!   stores of copied values go through `x87_quiet_bits`.
//! - Layouts: [`PathBuilder`], [`RayCastPathRequest`] and
//!   [`CheckLineOfSightFilterWater`] are this unit's classes. The solution,
//!   request and node layouts are the ones in `pathfind.rs` and
//!   `pathingsolution.rs`.
//!
//! All 89 functions of the unit, `006c94c0` to `006d0310`, are translated
//! (the array template instances at the end: constructors, destructors,
//! allocators and scalar deleting destructors, each named by the RTTI of
//! the vtable it stores). Earlier functions still reach the later ones by
//! address (`EDGE_ARRAY_CONSTRUCT`, ...), which runs the same translation.

#[allow(unused_imports)]
use crate::prelude::*;

use super::pathfind::{PathingRequest, PathingSolution};
use crate::types::BSSimpleArray;

// ---------------------------------------------------------------------------
// Layouts
// ---------------------------------------------------------------------------

layout! {
    /// `PathBuilder` (Xbox PDB), 0x34 bytes; the PC code uses the same
    /// offsets.
    pub struct PathBuilder: 0x34 {
        /// `pRequest` (Xbox PDB): `PathingRequest*`.
        0x00 pRequest: u32,
        /// `pSolution` (Xbox PDB): `PathingSolution*`.
        0x04 pSolution: u32,
        /// `pScrapHeap` (Xbox PDB): `ScrapHeap*`, given to every scrap array.
        0x08 pScrapHeap: u32,
        /// `bPathUpdate` (Xbox PDB).
        0x10 bPathUpdate: bool,
        /// `DetailedPathInfos` (Xbox PDB): `BSSimpleArray<NavMeshInfo const *>`.
        0x14 DetailedPathInfos: Inline<BSSimpleArray>,
        /// `DetailedPathMeshes` (Xbox PDB): `BSSimpleArray<NavMeshPtr>`.
        0x24 DetailedPathMeshes: Inline<BSSimpleArray>,
    }

    /// `RayCastPathRequest` (Xbox PDB), 0x68 bytes.
    pub struct RayCastPathRequest: 0x68 {
        /// `eType` (Xbox PDB): `RAYCAST_REQUEST_TYPE`.
        0x00 eType: u32,
        /// `pInitialPathRequest` (Xbox PDB): `PathingRequest*`.
        0x04 pInitialPathRequest: u32,
        /// `StartLocation` (Xbox PDB): a `PathingLocation`.
        0x08 StartLocation: Inline<super::pathfind::PathingLocation>,
        /// `GoalLocation` (Xbox PDB): a `PathingLocation`.
        0x30 GoalLocation: Inline<super::pathfind::PathingLocation>,
        /// `Edges` (Xbox PDB): `BSSimpleArray<NavMeshEdgeLocation>`.
        0x58 Edges: Inline<BSSimpleArray>,
    }

    /// `CheckLineOfSightFilterWater` (Xbox PDB; the RTTI of vtable
    /// `0106c93c` is `.?AUCheckLineOfSightFilterWater@@`), 8 bytes.
    pub struct CheckLineOfSightFilterWater: 0x08 {
        /// `bIsLargeCreature` (Xbox PDB).
        0x04 bIsLargeCreature: bool,
    }
}

/// `PathingSolution::VirtualPathingNodes` (Xbox PDB name, PC offset), the
/// array the node-adding functions append to.
const VIRTUAL_NODES: u32 = 0x08;
/// `PathingSolution` `DoorIDs` (Xbox PDB name), a `BSSimpleArray` of form
/// ids at +0x30 on PC (see `pathingsolution.rs`).
const DOOR_IDS: u32 = 0x30;
/// Size of a `VirtualPathingNode` (Xbox PDB).
const VIRTUAL_NODE_SIZE: u32 = 0x30;
/// Size of a `PathingNode` (Xbox PDB).
const PATHING_NODE_SIZE: u32 = 0x3c;
/// Size of a `PathingLocation` (Xbox PDB).
const PATHING_LOCATION_SIZE: u32 = 0x28;
/// Size of a `NavMeshEdgeLocation` (Xbox PDB).
const EDGE_LOCATION_SIZE: u32 = 0x18;
/// Size of a `ParentSpaceNode` (Xbox PDB).
const PARENT_SPACE_NODE_SIZE: u32 = 0x0c;
/// Size of a `TeleportLink` (Xbox PDB).
const TELEPORT_LINK_SIZE: u32 = 0x10;
/// Offset of `CheckLineOfSightFilterWater::bIsLargeCreature`.
const FILTER_LARGE_CREATURE: u32 = 0x04;
/// The word `fn_006cd880` sets in the navmesh search it is given.
const SEARCH_WORD_2084: u32 = 0x2084;

// ---------------------------------------------------------------------------
// Constants and globals (read from the exe at run time)
// ---------------------------------------------------------------------------

/// Vtable of `CheckLineOfSightFilterWater` (RTTI), slot 0 `006cd5c0`.
const CHECK_LINE_OF_SIGHT_FILTER_WATER_VTABLE: u32 = 0x0106_c93c;
/// Vtable of `Pathing::CheckLineOfSightFilter` (RTTI
/// `.?AUCheckLineOfSightFilter@Pathing@@`), slot 0 the pure-call stub.
const CHECK_LINE_OF_SIGHT_FILTER_VTABLE: u32 = 0x0106_c944;
/// Vtable of `BSSimpleArray<PathingLocation,1024>` (RTTI
/// `.?AV?$BSSimpleArray@VPathingLocation@@$0EAA@@@`).
const LOCATION_ARRAY_VTABLE: u32 = 0x0106_b91c;
/// The point the game passes with every node it appends (`0069f920`);
/// `00568fa0` also returns it when a door has no linked position.
const POINT_011F426C: u32 = 0x011f_426c;
/// Up axis (`NiPoint3`) for the cross product in `fn_006cd1f0`.
const AXIS_011A9484: u32 = 0x011a_9484;
/// The `TES` pointer (`ECX` of `0045cbc0`).
const TES_POINTER: u32 = 0x011d_ea10;
/// Settings (`Setting` objects; `00408d60` returns their bool value's
/// address, `00403e20` their float value's).
const SETTING_RAY_CAST_011D7404: u32 = 0x011d_7404;
const SETTING_DIRECT_LINE_011D73D8: u32 = 0x011d_73d8;
const SETTING_POV_SEARCH_011D7444: u32 = 0x011d_7444;
const SETTING_SMOOTHER_011D7434: u32 = 0x011d_7434;
const SETTING_RADIUS_011D6EF0: u32 = 0x011d_6ef0;
/// `double`: the squared distance between the first (or last) two current
/// nodes must exceed it before a tangent is built (`fn_006ca0e0`).
const TANGENT_DISTANCE_01017A10: u32 = 0x0101_7a10;
/// `double`: lowest and highest height difference accepted for a ray-cast
/// edge point (`fn_006cac90`, `fn_006cb6a0`).
const HEIGHT_LOW_0106B9F0: u32 = 0x0106_b9f0;
const HEIGHT_HIGH_0104ED58: u32 = 0x0104_ed58;
/// `float`: the weight of the triangle centre when a point is pulled into a
/// triangle.
const CENTER_WEIGHT_0101E2BC: u32 = 0x0101_e2bc;
/// `float`: the weight of the two edge vertices in `fn_006cde50` and
/// `fn_006ce2d0`.
const EDGE_WEIGHT_0104E2B8: u32 = 0x0104_e2b8;
/// `float`: the starting "best distance" of the triangle searches.
const START_DISTANCE_01016970: u32 = 0x0101_6970;
/// `double`: factor on the two end gaps in `fn_006cb950`.
const GAP_FACTOR_01011590: u32 = 0x0101_1590;
/// `double`: span below which a straight path is tried.
const SHORT_SPAN_01028338: u32 = 0x0102_8338;
/// `double`: actor radius above which large-creature rules apply.
const LARGE_RADIUS_0106C948: u32 = 0x0106_c948;
/// `double`: added to the ground height before comparing a point with it.
const GROUND_OFFSET_0104F2F0: u32 = 0x0104_f2f0;
/// `float`: the reach used for a door node instead of the target radius.
const DOOR_REACH_0101E33C: u32 = 0x0101_e33c;
/// Comparison callbacks passed to the array searches.
const BOOL_EQUALS_006CF760: u32 = 0x006c_f760;
const POINTER_EQUALS_009A3830: u32 = 0x009a_3830;
const HOLDER_EQUALS_0042FF10: u32 = 0x0042_ff10;

// ---------------------------------------------------------------------------
// Callees (outside this file)
// ---------------------------------------------------------------------------

/// Request accessors, thiscall on a `PathingRequest` (bodies read in
/// Ghidra): `Origin` (+0x0C), `Destination` (+0x34), `fActorRadius`
/// (+0x68, `ST0`), `fTargetRadius` (+0x74, `ST0`), the float at +0x8C
/// (`ST0`), the bytes at +0x98, +0x99, +0x9B, +0xA2, +0xA4, the word at
/// +0xA8 and the `NiPointer` at +0x94 (`spAvoidNodeArray`).
const REQUEST_ORIGIN: u32 = 0x0048_d150;
const REQUEST_DESTINATION: u32 = 0x006a_9540;
const REQUEST_ACTOR_RADIUS: u32 = 0x0069_ef80;
const REQUEST_TARGET_RADIUS: u32 = 0x009b_0bd0;
const REQUEST_FLOAT_8C: u32 = 0x0050_7e10;
const REQUEST_BYTE_98: u32 = 0x0073_cb80;
const REQUEST_BYTE_99: u32 = 0x0050_5c00;
const REQUEST_BYTE_9B: u32 = 0x006a_9560;
const REQUEST_BYTE_A2: u32 = 0x006a_7980;
const REQUEST_BYTE_A4: u32 = 0x006a_79a0;
const REQUEST_WORD_A8: u32 = 0x008c_dd90;
const REQUEST_AVOID_NODE_ARRAY: u32 = 0x0069_e390;

/// Solution accessors, thiscall: virtual node `i` (`006e8f40` on +0x08),
/// the number of virtual nodes, current node `i` (on +0x20), the number of
/// current nodes, the words at +0x18 and +0x1C (first and last loaded
/// virtual node), the last current node (null when there is none) and the
/// byte at +0x40 (`bIncompletePath`).
const SOLUTION_VIRTUAL_NODE: u32 = 0x006e_78b0;
const SOLUTION_VIRTUAL_NODE_COUNT: u32 = 0x005a_4320;
const SOLUTION_CURRENT_NODE: u32 = 0x006e_7970;
const SOLUTION_CURRENT_NODE_COUNT: u32 = 0x008b_6800;
const SOLUTION_FIRST_LOADED: u32 = 0x0096_11e0;
/// Returns the word at +0x1C: the map's `PathingLocation::GetWorldspace`;
/// on a solution it is the last loaded virtual node index.
const WORD_AT_1C: u32 = 0x0044_1110;
const SOLUTION_LAST_CURRENT_NODE: u32 = 0x006b_05a0;
const SOLUTION_INCOMPLETE: u32 = 0x006b_05d0;
/// Thiscall on the solution: stores -1 in +0x18 and +0x1C after clearing
/// the current nodes.
const SOLUTION_RESET: u32 = 0x006c_93a0;
/// Thiscall (location): appends a virtual node built by `006f4d00` (the
/// same shape as `fn_006c9920`).
const SOLUTION_ADD_LOCATION_NODE: u32 = 0x006c_90d0;
/// Thiscall (location, point): appends a current node.
const SOLUTION_ADD_CURRENT_NODE: u32 = 0x0069_f920;
/// Thiscall: removes the last current node.
const SOLUTION_REMOVE_LAST_CURRENT_NODE: u32 = 0x006c_9350;

/// Node accessors: the node's `Location` (+4), `pDoorRef` of a virtual node
/// (+0x2C), and the node position setter (copies a point to +0x2C).
const NODE_LOCATION: u32 = 0x0071_7e50;
const VIRTUAL_NODE_DOOR: u32 = 0x0055_b980;
const NODE_SET_POSITION: u32 = 0x006b_0570;
/// `VirtualPathingNode` constructors (`virtualpathingnode.cpp`), thiscall
/// on the placement address.
const VIRTUAL_NODE_CONSTRUCT_INFO: u32 = 0x006f_4e40;
const VIRTUAL_NODE_CONSTRUCT_POINT_INFO: u32 = 0x006f_4f90;
const VIRTUAL_NODE_CONSTRUCT_INFO_POINT_DOOR: u32 = 0x006f_4f00;
const VIRTUAL_NODE_CONSTRUCT_LOCATION_DOOR: u32 = 0x006f_4da0;
/// `BSSimpleArray<VirtualPathingNode>`: address of element `i`.
const VIRTUAL_NODE_SLOT: u32 = 0x006e_8f40;
/// Placement `operator new`, cdecl (size, where): returns `where`.
const PLACEMENT_NEW: u32 = 0x006e_6da0;

/// `PathingLocation` functions (`pathinglocation.cpp`).
const LOCATION_CONSTRUCT: u32 = 0x006d_cbb0;
const LOCATION_COPY_CONSTRUCT: u32 = 0x006d_cc40;
const LOCATION_ASSIGN: u32 = 0x006d_cce0;
const LOCATION_DESTRUCT: u32 = 0x004f_f7e0;
const LOCATION_CONSTRUCT_FROM_MESH_POINT: u32 = 0x006d_d010;
const LOCATION_CONSTRUCT_FROM_PARTS: u32 = 0x006d_cee0;
const LOCATION_CONSTRUCT_FROM_POINT_AND_LOCATION: u32 = 0x006d_d0d0;
const LOCATION_CONSTRUCT_FROM_POINT_AND_INFO: u32 = 0x006d_cf20;
const LOCATION_COPY_POSITION: u32 = 0x005c_3420;
const LOCATION_GET_CELL: u32 = 0x006d_d4f0;
const LOCATION_GET_BEST_NAVMESH_INFO: u32 = 0x006d_d540;
const LOCATION_GET_NAVMESH: u32 = 0x006d_d610;
const LOCATION_GET_NAVMESH_AND_TRIANGLE: u32 = 0x006d_d640;
const LOCATION_GET_NAVMESH_INFOS: u32 = 0x006d_d5c0;
const LOCATION_HAS_NAVMESH_INFO: u32 = 0x006d_d6a0;
const LOCATION_RESOLVE_NAVMESH_INFO: u32 = 0x006d_d6f0;
const LOCATION_RESOLVE: u32 = 0x006d_e250;
const LOCATION_SET_POSITION: u32 = 0x006d_d460;

/// `NiPoint3` helpers, thiscall: (out, other) for add and subtract, (out,
/// scalar) for the scaled copy, (other) for `+=`; the squared lengths and
/// lengths leave a `float` in `ST0`.
const POINT3_ADD: u32 = 0x0043_9e90;
const POINT3_SUBTRACT: u32 = 0x0043_9ef0;
const POINT3_TIMES_SCALAR: u32 = 0x0045_bb20;
const POINT3_ADD_ASSIGN: u32 = 0x0063_c8a0;
const POINT3_UNIT_CROSS: u32 = 0x0053_d1a0;
const POINT3_DEFAULT_CONSTRUCT: u32 = 0x0068_15c0;
const POINT3_LENGTH_SQUARED: u32 = 0x004a_7290;
const POINT3_LENGTH: u32 = 0x0045_7990;
const POINT2_LENGTH_SQUARED: u32 = 0x0059_5c80;
const POINT2_LENGTH: u32 = 0x0058_9850;
/// cdecl (float): the absolute value (`00408860` on its argument).
const FLOAT_ABSOLUTE_VALUE: u32 = 0x0040_8840;

/// Array helpers. `ARRAY_SIZE` returns the word at +8; `ARRAY_IS_EMPTY`
/// whether it is zero. The element accessors return the address of
/// element `i` for 4-, 12-, 16-, 0x14- and 0x28-byte elements; the "last"
/// ones of the last element.
const ARRAY_SIZE: u32 = 0x0044_ddc0;
const ARRAY_IS_EMPTY: u32 = 0x0076_b610;
const WORD_ELEMENT: u32 = 0x0087_7a30;
const LAST_WORD_ELEMENT: u32 = 0x006f_2270;
const SPACE_ELEMENT: u32 = 0x006a_1440;
const LINK_ELEMENT: u32 = 0x0055_8dd0;
const FIRST_LINK_ELEMENT: u32 = 0x006a_1420;
const TRIANGLE_HANDLE_ELEMENT: u32 = 0x006b_3160;
const LAST_TRIANGLE_HANDLE: u32 = 0x006b_31a0;
const LOCATION_ELEMENT: u32 = 0x0069_1750;
/// Thiscall (value pointer, start, compare): index of the first match from
/// `start`, or -1.
const ARRAY_FIND: u32 = 0x0071_9b20;
const BOOL_ARRAY_FIND: u32 = 0x006c_f700;
/// Thiscall (word pointer): appends a word.
const ARRAY_PUSH_WORD: u32 = 0x007c_b2e0;
/// Thiscall (flag): clears an array (`DetailedPathInfos`, the segment list).
const ARRAY_CLEAR_008454F0: u32 = 0x0084_54f0;

/// Scrap arrays and arrays on the stack (the class names are the Xbox PDB
/// template instances the queue lists for this unit, matched by the
/// element sizes the code uses).
const INFO_SCRAP_ARRAY_CONSTRUCT: u32 = 0x006c_f2a0;
const INFO_SCRAP_ARRAY_DESTRUCT: u32 = 0x006c_f340;
const DOOR_SCRAP_ARRAY_CONSTRUCT: u32 = 0x006c_f3a0;
const DOOR_SCRAP_ARRAY_DESTRUCT: u32 = 0x006c_f440;
const BOOL_SCRAP_ARRAY_CONSTRUCT: u32 = 0x006c_f570;
const BOOL_SCRAP_ARRAY_ADD: u32 = 0x006c_f520;
const BOOL_SCRAP_ARRAY_DESTRUCT: u32 = 0x006c_f610;
const SEGMENT_SCRAP_ARRAY_CONSTRUCT: u32 = 0x006c_f780;
const SEGMENT_SCRAP_ARRAY_DESTRUCT: u32 = 0x006c_f840;
const SEGMENT_ARRAY_ADD: u32 = 0x006c_f4c0;
const EDGE_ARRAY_CONSTRUCT: u32 = 0x006c_f200;
const EDGE_ARRAY_DESTRUCT: u32 = 0x006c_f230;
const EDGE_ARRAY_COPY_TO: u32 = 0x006c_f8a0;
const LOCATION_ARRAY_DESTRUCT: u32 = 0x006a_bfc0;
const LOCATION_ARRAY_INIT: u32 = 0x006d_0140;
const LOCATION_ARRAY_ADD_SLOT: u32 = 0x0069_25b0;
const LOCATION_ARRAY_CONSTRUCT_ELEMENTS: u32 = 0x006d_00a0;
const LOCATION_ARRAY_COPY_TO: u32 = 0x006c_f940;
const LOCATION_ARRAY_CLEAR: u32 = 0x006a_c010;
const TRIANGLE_ARRAY_A_CONSTRUCT: u32 = 0x006c_f9a0;
const TRIANGLE_ARRAY_A_DESTRUCT: u32 = 0x006a_bdc0;
const TRIANGLE_ARRAY_C_CONSTRUCT: u32 = 0x006c_f9d0;
const TRIANGLE_ARRAY_C_DESTRUCT: u32 = 0x006a_bc30;
const PORTAL_ARRAY_CONSTRUCT: u32 = 0x006c_fb20;
const PORTAL_ARRAY_DESTRUCT: u32 = 0x006c_fbc0;
const INFO_HANDLE_ARRAY_CONSTRUCT: u32 = 0x006c_fa00;
const INFO_HANDLE_ARRAY_ADD: u32 = 0x006d_ba30;
const INFO_HANDLE_ARRAY_DESTRUCT: u32 = 0x006c_fac0;
const INFO_LIST_CONSTRUCT: u32 = 0x005e_0510;
const INFO_LIST_DESTRUCT: u32 = 0x005e_0540;
const NAVMESH_LIST_CONSTRUCT: u32 = 0x0042_f800;
const NAVMESH_LIST_DESTRUCT: u32 = 0x0042_f830;
/// cdecl (list): fills the list with navmeshes (`pathfind.cpp`).
const NEARBY_NAVMESHES_FILL: u32 = 0x006d_91d0;
/// Array template helpers used by the template instances of this file.
const ARRAY_IS_FULL: u32 = 0x0043_8b90;
const ARRAY_GROWN_CAPACITY: u32 = 0x009a_3910;
const ARRAY_SHOULD_SHRINK: u32 = 0x006f_3170;
const ARRAY_SHRUNK_CAPACITY: u32 = 0x0086_9600;
const ARRAY_FREE_BUFFER: u32 = 0x006a_8500;
const VIRTUAL_NODE_ARRAY_REALLOCATE: u32 = 0x006c_fe70;
const VIRTUAL_NODE_RANGE_DESTRUCT: u32 = 0x006c_fe30;
const PATHING_NODE_RANGE_DESTRUCT: u32 = 0x006c_ff20;
const PATHING_NODE_RANGE_MOVE: u32 = 0x006a_2270;
const SPACE_RANGE_DESTRUCT: u32 = 0x0072_ba80;
const SPACE_ARRAY_REALLOCATE: u32 = 0x006d_c590;
const LINK_ARRAY_REALLOCATE: u32 = 0x009f_6540;

/// `NiPointer`-style holders of a navmesh (`NavMeshPtr`).
const NAV_HOLDER_CONSTRUCT: u32 = 0x0042_fb00;
const NAV_HOLDER_RELEASE: u32 = 0x0042_fa40;
const NI_POINTER_GET: u32 = 0x0055_9450;
const NAVMESH_HOLDER_ARRAY_ADD: u32 = 0x0042_f850;
const NAVMESH_HOLDER_ARRAY_CLEAR: u32 = 0x0042_f9c0;

/// Navmesh functions (`navmesh.cpp`, `navmesh_util.cpp`, `tesobjectcell.cpp`).
const NAVMESH_INFO_LOAD_HOLDER: u32 = 0x0069_ad00;
const NAVMESH_GET_INFO: u32 = 0x0068_efb0;
const NAVMESH_GET_CENTER: u32 = 0x0055_8220;
const NAVMESH_GET_TRIANGLE: u32 = 0x0055_82f0;
const NAVMESH_GET_EDGE_VERTICES: u32 = 0x0068_f040;
const NAVMESH_EDGE_POINTS: u32 = 0x0068_f0c0;
const NAVMESH_FIND_EDGE: u32 = 0x0069_7010;
const NAVMESH_FIND_PORTAL_TRIANGLES: u32 = 0x0069_7590;
const NAVMESH_CELL: u32 = 0x0059_bb30;
const TRIANGLE_TEST_006A5EC0: u32 = 0x006a_5ec0;
const TRIANGLE_HAS_FLAG_0X10: u32 = 0x0069_3b90;
const TRIANGLE_FLAGS_SET: u32 = 0x0069_1140;
const TRIANGLE_HANDLE_CENTER: u32 = 0x006a_6eb0;
const FAST_TRIANGLE_HANDLE_CONSTRUCT: u32 = 0x0069_a660;
const EDGE_HANDLE_COPY: u32 = 0x006c_4350;
/// cdecl helpers of `pathfind.cpp` and others.
const FIND_EDGES: u32 = 0x006d_4120;
const CHECK_LINE_OF_SIGHT: u32 = 0x006d_7350;
const BUILD_FIRST_TANGENT: u32 = 0x006d_99a0;
const BUILD_LAST_TANGENT: u32 = 0x006d_a420;
const CLOSEST_ON_EDGE_006B9C20: u32 = 0x006b_9c20;
const EDGE_PULL_TEST_006BA100: u32 = 0x006b_a100;
const CLAMP_INTEGER: u32 = 0x0064_7ba0;
/// `TES` ground height, thiscall (point, cell): `ST0`.
const TES_GROUND_HEIGHT: u32 = 0x0045_cbc0;

/// References: the linked door position (`TESObjectREFR::GetLinkedDoorTeleportPosition`),
/// the object `00568e50` returns for a door, and a form's id (+0x0C).
const REFERENCE_LINKED_DOOR_POSITION: u32 = 0x0056_8fa0;
const REFERENCE_GET_POSITION_OWNER: u32 = 0x0056_8e50;
const FORM_GET_ID: u32 = 0x0084_e3a0;
/// thiscall on a cell: true when `00450fd0` returns 6.
const CELL_TEST_00450FF0: u32 = 0x0045_0ff0;

/// Settings: the address of a bool or float setting's value.
const SETTING_BOOL_ADDRESS: u32 = 0x0040_8d60;
const SETTING_FLOAT_ADDRESS: u32 = 0x0040_3e20;

/// Searches.
const INFO_SEARCH_CONSTRUCT: u32 = 0x006b_8340;
const INFO_SEARCH_RUN: u32 = 0x006b_8c50;
const INFO_SEARCH_DESTRUCT: u32 = 0x006b_8420;
const TELEPORT_DOOR_SEARCH_CONSTRUCT: u32 = 0x006f_33b0;
const TELEPORT_DOOR_SEARCH_BUILD_NODE_PATH: u32 = 0x006f_34e0;
const TELEPORT_DOOR_SEARCH_DESTRUCT: u32 = 0x006f_3460;
const TELEPORT_PATH_CONSTRUCT: u32 = 0x006f_48b0;
const TELEPORT_PATH_DESTRUCT: u32 = 0x006f_4930;
const RAY_CAST_SEARCH_CONSTRUCT: u32 = 0x006e_61a0;
const RAY_CAST_SEARCH_BUILD_PATH: u32 = 0x006e_6320;
const RAY_CAST_SEARCH_DESTRUCT: u32 = 0x006e_62b0;
const NAVMESH_SEARCH_CONSTRUCT: u32 = 0x006a_6a40;
const NAVMESH_SEARCH_RUN: u32 = 0x006a_6b70;
const NAVMESH_SEARCH_DESTRUCT: u32 = 0x006a_6b00;
const POV_SEARCH_CONSTRUCT: u32 = 0x0069_e110;
const POV_SEARCH_RUN: u32 = 0x0069_f010;
const POV_SEARCH_DESTRUCT: u32 = 0x0069_e200;
const PATH_SMOOTHER_CONSTRUCT: u32 = 0x006e_d100;
const PATH_SMOOTHER_RUN: u32 = 0x006e_d530;
const PATH_SMOOTHER_DESTRUCT: u32 = 0x006e_d220;
const POV_SMOOTHER_CONSTRUCT: u32 = 0x006a_d4c0;
const POV_SMOOTHER_RUN: u32 = 0x006a_d770;
const POV_SMOOTHER_DESTRUCT: u32 = 0x006a_d690;

// Vtables of the array template instances this file constructs and
// destroys (each named by its RTTI type descriptor, read through the
// complete object locator before the vtable).
/// `.?AV?$BSSimpleArray@UNavMeshEdgeLocation@@$0EAA@@@`.
const EDGE_ARRAY_VTABLE: u32 = 0x0106_c954;
/// `.?AV?$BSScrapArray@PBVNavMeshInfo@@$0EAA@@@`.
const INFO_SCRAP_ARRAY_VTABLE: u32 = 0x0106_c968;
/// `.?AV?$BSScrapArray@PAVTESObjectREFR@@$0EAA@@@`.
const REFERENCE_SCRAP_ARRAY_VTABLE: u32 = 0x0106_c97c;
/// `.?AV?$BSScrapArray@_N$0EAA@@@`.
const BOOL_SCRAP_ARRAY_VTABLE: u32 = 0x0106_c990;
/// `.?AV?$BSScrapArray@UVirtualNodeSegment@PathBuilder@@$0EAA@@@`.
const SEGMENT_SCRAP_ARRAY_VTABLE: u32 = 0x0106_c9a4;
/// `.?AV?$BSSimpleArray@UNavMeshSearchLink@@$0EAA@@@`.
const SEARCH_LINK_ARRAY_VTABLE: u32 = 0x0106_b8c8;
/// `.?AV?$BSSimpleArray@UNavMeshSearchNode@@$0EAA@@@`.
const SEARCH_NODE_ARRAY_VTABLE: u32 = 0x0106_b8a0;
/// `.?AV?$BSScrapArray@UFastNavMeshTriHandle@@$0EAA@@@`.
const TRIANGLE_HANDLE_SCRAP_ARRAY_VTABLE: u32 = 0x0106_c9cc;
/// `.?AV?$BSScrapArray@H$0EAA@@@`.
const INTEGER_SCRAP_ARRAY_VTABLE: u32 = 0x0106_c9f4;
/// Offset of a scrap array's `ScrapHeap*`, after the `BSSimpleArray` part.
const SCRAP_ARRAY_HEAP: u32 = 0x10;
/// The alignment word the scrap arrays pass to `ScrapHeap::Allocate` (read
/// from the exe; 4 in 1.4.0.525).
const SCRAP_ALIGNMENT_010A2720: u32 = 0x010a_2720;

/// The memory manager wrappers, cdecl: allocate (bytes), free (block),
/// reallocate (block, bytes); `00401020` returns the memory manager
/// (`011f6238`), the `ECX` of `MemoryManager::GetThreadScrapHeap`.
const MEMORY_ALLOCATE: u32 = 0x0040_1000;
const MEMORY_FREE: u32 = 0x0040_1030;
const MEMORY_REALLOCATE: u32 = 0x0042_f5d0;
const MEMORY_MANAGER_GET: u32 = 0x0040_1020;
const GET_THREAD_SCRAP_HEAP: u32 = 0x00aa_42e0;
/// `ScrapHeap::Allocate` (bytes, alignment) and `ScrapHeap::Deallocate`
/// (block), thiscall on the heap.
const SCRAP_HEAP_ALLOCATE: u32 = 0x00aa_54a0;
const SCRAP_HEAP_DEALLOCATE: u32 = 0x00aa_5610;
/// cdecl (destination, source, bytes): `memcpy`.
const MEMORY_COPY: u32 = 0x0040_1460;
/// `BSSimpleArray<bool>`: makes room for one element and returns its index
/// (`006d0450`); placement-constructs `count` elements at an address
/// (`006e1280`).
const BOOL_ARRAY_ADD_SLOT: u32 = 0x006d_0450;
const BOOL_ARRAY_CONSTRUCT_ELEMENTS: u32 = 0x006e_1280;
/// `BSSimpleArray<NavMeshEdgeLocation>`: placement-constructs `count`
/// elements at a buffer (thiscall, buffer, count).
const EDGE_ARRAY_CONSTRUCT_ELEMENTS: u32 = 0x006d_05e0;
/// Base `BSSimpleArray` constructors the scrap array constructors run
/// first, and the destructors their destructors end with.
const INFO_ARRAY_BASE_CONSTRUCT: u32 = 0x005e_0510;
const REFERENCE_ARRAY_BASE_CONSTRUCT: u32 = 0x005e_04c0;
const REFERENCE_ARRAY_BASE_DESTRUCT: u32 = 0x005e_04f0;
const BOOL_ARRAY_BASE_CONSTRUCT: u32 = 0x006d_0420;
const SEGMENT_ARRAY_BASE_CONSTRUCT: u32 = 0x006d_03a0;
const SEGMENT_ARRAY_BASE_DESTRUCT: u32 = 0x006c_f820;
const TRIANGLE_HANDLE_ARRAY_BASE_CONSTRUCT: u32 = 0x006d_04e0;
const TRIANGLE_HANDLE_ARRAY_BASE_DESTRUCT: u32 = 0x006c_faa0;
const INTEGER_ARRAY_BASE_CONSTRUCT: u32 = 0x0069_4de0;
const INTEGER_ARRAY_BASE_DESTRUCT: u32 = 0x0069_4e10;
/// The array initialisers (thiscall: capacity, size): clear the three
/// words, allocate `max(capacity, size)` through virtual slot 4, construct
/// `size` elements (bodies read in Ghidra).
const ARRAY_INIT_006B3EB0: u32 = 0x006b_3eb0;
const ARRAY_INIT_006B3F40: u32 = 0x006b_3f40;
const ARRAY_INIT_0069B940: u32 = 0x0069_b940;
const ARRAY_INIT_009A3D00: u32 = 0x009a_3d00;
const ARRAY_INIT_006DC440: u32 = 0x006d_c440;
/// The tasklet manager (`BSTaskletManager::QInstance`) and its virtual
/// slot 0xC (thread id), the import the game asks the thread id from
/// (`0040fc90`), the path manager (`PathManager::QInstance`) and its
/// queued ray casts.
const TASKLET_MANAGER_INSTANCE: u32 = 0x00b0_0a00;
const TASKLET_MANAGER_SLOT_0C: u32 = 0x0c;
const CURRENT_THREAD_QUERY: u32 = 0x0040_fc90;
const PATH_MANAGER_INSTANCE: u32 = 0x0047_d0b0;
const PATH_MANAGER_GET_RAY_CAST_PATH: u32 = 0x006e_bc20;
const PATH_MANAGER_GET_RAY_CAST_PATH_TO_NAV_MESH: u32 = 0x006e_bbf0;

// ---------------------------------------------------------------------------
// Frames and small helpers
// ---------------------------------------------------------------------------

/// The stack frame of a translated function: a zeroed block standing in for
/// the game's stack, addressed by the `EBP`-relative offsets the code uses.
#[derive(Clone, Copy)]
struct Frame(u32);

impl Frame {
    /// The address of the local at `EBP + offset`.
    fn at(self, offset: i32) -> u32 {
        self.0.wrapping_add(offset as u32)
    }
}

/// Runs `body` with a zeroed frame of `locals` bytes (the `SUB ESP` or
/// `__alloca_probe` size) plus slack.
fn with_frame<R>(e: &mut Engine, locals: u32, body: impl FnOnce(&mut Engine, Frame) -> R) -> R {
    e.with_stack(locals + 0x80, |e, block| {
        body(e, Frame(block.addr() + locals + 0x40))
    })
}

fn request(e: &Engine, this: Ptr<PathBuilder>) -> u32 {
    e.get(this, PathBuilder::pRequest)
}

fn solution(e: &Engine, this: Ptr<PathBuilder>) -> u32 {
    e.get(this, PathBuilder::pSolution)
}

fn scrap_heap(e: &Engine, this: Ptr<PathBuilder>) -> u32 {
    e.get(this, PathBuilder::pScrapHeap)
}

fn call_u32(e: &mut Engine, addr: u32, words: &[u32]) -> u32 {
    e.call(addr, words).u32()
}

fn call_bool(e: &mut Engine, addr: u32, words: &[u32]) -> bool {
    e.call(addr, words).bool()
}

fn virtual_node(e: &mut Engine, solution: u32, index: u32) -> u32 {
    call_u32(e, SOLUTION_VIRTUAL_NODE, &[solution, index])
}

fn node_location(e: &mut Engine, node: u32) -> u32 {
    call_u32(e, NODE_LOCATION, &[node])
}

/// `005c3420`: copies the location's point into `out` and returns `out`.
fn location_point(e: &mut Engine, location: u32, out: u32) -> u32 {
    call_u32(e, LOCATION_COPY_POSITION, &[location, out])
}

fn array_size(e: &mut Engine, array: u32) -> u32 {
    call_u32(e, ARRAY_SIZE, &[array])
}

/// The word stored in element `index` of a 4-byte array.
fn word_element(e: &mut Engine, array: u32, index: u32) -> u32 {
    let slot = call_u32(e, WORD_ELEMENT, &[array, index]);
    e.mem.u32(slot)
}

fn copy_point(e: &mut Engine, destination: u32, source: u32) {
    for i in 0..3 {
        let word = e.mem.u32(source + 4 * i);
        e.mem.set_u32(destination + 4 * i, word);
    }
}

/// A `float` constant as `FLD`/`FSTP` pushes it.
fn float_constant_bits(e: &Engine, addr: u32) -> u32 {
    x87_quiet_bits(e.global::<u32>(addr))
}

/// `00408d60` on a bool setting, then the byte at the returned address.
fn setting_bool(e: &mut Engine, setting: u32) -> bool {
    let value = call_u32(e, SETTING_BOOL_ADDRESS, &[setting]);
    e.mem.u8(value) != 0
}

/// Whether the tasklet manager reports the current thread (the vcall the
/// ray-cast builders make before queuing their work on the path manager).
fn on_tasklet_thread(e: &mut Engine) -> bool {
    let manager = call_u32(e, TASKLET_MANAGER_INSTANCE, &[]);
    let thread = call_u32(e, CURRENT_THREAD_QUERY, &[]);
    e.vcall(manager, TASKLET_MANAGER_SLOT_0C, &[thread]).bool()
}

/// The `fTargetRadius` or door reach comparison shared by `fn_006cc5e0`:
/// `reach * reach > |offset.xy|^2` and `|offset.z| <` the high height limit.
fn within_reach(e: &mut Engine, reach: f32, offset: u32) -> bool {
    let planar = e.call(POINT2_LENGTH_SQUARED, &[offset]).f64();
    let inside = f64::from(reach) * f64::from(reach) > planar;
    if !inside {
        return false;
    }
    let height = x87_quiet_bits(e.mem.u32(offset + 8));
    let absolute = e.call(FLOAT_ABSOLUTE_VALUE, &[height]).f64();
    absolute < e.global::<f64>(HEIGHT_HIGH_0104ED58)
}

/// The reach `fn_006cc5e0` uses for the node `node`: the door reach for a
/// door node (`fn_006ca0c0`), the request's target radius otherwise.
fn node_reach(e: &mut Engine, request: u32, node: u32) -> f32 {
    if fn_006ca0c0(e, node) {
        f32::from_bits(float_constant_bits(e, DOOR_REACH_0101E33C))
    } else {
        e.call(REQUEST_TARGET_RADIUS, &[request]).f32()
    }
}

/// Appends a `VirtualPathingNode` to the solution's `VirtualPathingNodes`
/// and constructs it in place with `construct` and `words` (the body shared
/// by `fn_006c9920`, `fn_006c99c0`, `fn_006c9a70` and
/// `PathingSolution::AddVirtualPathingNode`). Returns the new index.
fn add_virtual_node(e: &mut Engine, solution: u32, construct: u32, words: &[u32]) -> u32 {
    let nodes = solution + VIRTUAL_NODES;
    let index = fn_006cef10(e, nodes);
    let slot = call_u32(e, VIRTUAL_NODE_SLOT, &[nodes, index]);
    let place = call_u32(e, PLACEMENT_NEW, &[VIRTUAL_NODE_SIZE, slot]);
    if place != 0 {
        let mut arguments = vec![place];
        arguments.extend_from_slice(words);
        e.call(construct, &arguments);
    }
    index
}

/// Writes a `PathBuilder::VirtualNodeSegment` (Xbox PDB: first and last
/// virtual node, `bHasNavMeshes`) at `segment` and appends it.
fn add_segment(
    e: &mut Engine,
    segments: u32,
    segment: u32,
    first: i32,
    last: i32,
    navmeshes: bool,
) {
    e.mem.set_i32(segment, first);
    e.mem.set_i32(segment + 4, last);
    e.mem.set_u8(segment + 8, u8::from(navmeshes));
    e.call(SEGMENT_ARRAY_ADD, &[segments, segment]);
}

// ---------------------------------------------------------------------------
// Translations
// ---------------------------------------------------------------------------

/// The door pair of `PathBuilder::BuildNavMeshInfoPath`: a virtual node at
/// the door's linked position for navmesh info `index`, then one at the
/// point the door's owner gives for info `index + 1`.
fn add_door_nodes(
    e: &mut Engine,
    solution: u32,
    infos: u32,
    door: u32,
    index: u32,
    position: u32,
    point: u32,
) {
    let linked = call_u32(e, REFERENCE_LINKED_DOOR_POSITION, &[door]);
    copy_point(e, position, linked);
    let info = word_element(e, infos, index);
    fn_006c9a70(e, solution, info, position, door);
    let owner = call_u32(e, REFERENCE_GET_POSITION_OWNER, &[door]);
    let owner_point = node_location(e, owner);
    copy_point(e, point, owner_point);
    let next = word_element(e, infos, index + 1);
    fn_006c99c0(e, solution, point, next);
}

// Translated from 006c94c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006c94c0 - the C++ exception-unwinding frame (FS:[0]) that destroys the search and the two scrap arrays when a callee throws is not translated
/// `PathBuilder::BuildNavMeshInfoPath` (Xbox PDB): runs the navmesh info
/// search (`006b8c50`) from the request's origin (`start_info`) to its
/// destination (`goal_info`) into two scrap arrays (navmesh infos and the
/// doors between them), then appends the virtual nodes: the origin, for
/// each door the door and its far side, every info in between, and the
/// destination. Returns what the search returned.
pub fn path_builder_build_nav_mesh_info_path(
    e: &mut Engine,
    this: Ptr<PathBuilder>,
    start_info: u32,
    goal_info: u32,
) -> bool {
    with_frame(e, 0x2140, |e, f| {
        let search = f.at(-0x20ac);
        let infos = f.at(-0x20e0);
        let doors = f.at(-0x20c8);
        e.call(INFO_SEARCH_CONSTRUCT, &[search]);
        let heap = scrap_heap(e, this);
        e.call(INFO_SCRAP_ARRAY_CONSTRUCT, &[infos, 0x14, 0, heap]);
        let heap = scrap_heap(e, this);
        e.call(DOOR_SCRAP_ARRAY_CONSTRUCT, &[doors, 0x14, 0, heap]);
        let req = request(e, this);
        let mut request_5c = 0;
        if !call_bool(e, REQUEST_BYTE_98, &[req]) {
            request_5c = fn_006c9900(e, req);
        }
        let flag = u32::from(call_bool(e, REQUEST_BYTE_A4, &[req]));
        let destination = call_u32(e, REQUEST_DESTINATION, &[req]);
        let goal_point = location_point(e, destination, f.at(-0x2128));
        let origin = call_u32(e, REQUEST_ORIGIN, &[req]);
        let start_point = location_point(e, origin, f.at(-0x2134));
        let found = call_bool(
            e,
            INFO_SEARCH_RUN,
            &[
                search,
                request_5c,
                start_point,
                start_info,
                goal_point,
                goal_info,
                infos,
                doors,
                flag,
            ],
        );
        let sol = solution(e, this);
        if array_size(e, infos) <= 1 {
            let origin = call_u32(e, REQUEST_ORIGIN, &[req]);
            e.call(SOLUTION_ADD_LOCATION_NODE, &[sol, origin]);
            let destination = call_u32(e, REQUEST_DESTINATION, &[req]);
            e.call(SOLUTION_ADD_LOCATION_NODE, &[sol, destination]);
        } else {
            let origin = call_u32(e, REQUEST_ORIGIN, &[req]);
            e.call(SOLUTION_ADD_LOCATION_NODE, &[sol, origin]);
            if word_element(e, doors, 0) != 0 {
                let door = word_element(e, doors, 0);
                add_door_nodes(e, sol, infos, door, 0, f.at(-0x20f0), f.at(-0x20fc));
            }
            let mut index = 1u32;
            while index < array_size(e, infos).wrapping_sub(1) {
                if word_element(e, doors, index) == 0 {
                    let info = word_element(e, infos, index);
                    fn_006c9920(e, sol, info);
                } else {
                    let door = word_element(e, doors, index);
                    if door != 0 {
                        add_door_nodes(e, sol, infos, door, index, f.at(-0x2110), f.at(-0x211c));
                    }
                }
                index += 1;
            }
            let destination = call_u32(e, REQUEST_DESTINATION, &[req]);
            e.call(SOLUTION_ADD_LOCATION_NODE, &[sol, destination]);
        }
        e.call(DOOR_SCRAP_ARRAY_DESTRUCT, &[doors]);
        e.call(INFO_SCRAP_ARRAY_DESTRUCT, &[infos]);
        e.call(INFO_SEARCH_DESTRUCT, &[search]);
        found
    })
}

// Translated from 006c9900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns `this + 0x5C` (the address of the request's field at +0x5C,
/// which `PathBuilder::BuildNavMeshInfoPath` hands to its search).
pub fn fn_006c9900(_e: &mut Engine, this: u32) -> u32 {
    this.wrapping_add(0x5c)
}

// Translated from 006c9920 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006c9920 - the C++ exception-unwinding frame (FS:[0]) around the placement construction is not translated
/// Thiscall on a `PathingSolution`: appends a virtual node built by
/// `006f4e40` from `info` (a navmesh info); returns its index.
pub fn fn_006c9920(e: &mut Engine, this: u32, info: u32) -> u32 {
    add_virtual_node(e, this, VIRTUAL_NODE_CONSTRUCT_INFO, &[info])
}

// Translated from 006c99c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006c99c0 - the C++ exception-unwinding frame (FS:[0]) around the placement construction is not translated
/// Thiscall on a `PathingSolution`: appends a virtual node built by
/// `006f4f90` from `point` and `info`; returns its index.
pub fn fn_006c99c0(e: &mut Engine, this: u32, point: u32, info: u32) -> u32 {
    add_virtual_node(e, this, VIRTUAL_NODE_CONSTRUCT_POINT_INFO, &[point, info])
}

// Translated from 006c9a70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006c9a70 - the C++ exception-unwinding frame (FS:[0]) around the placement construction is not translated
/// Thiscall on a `PathingSolution`: appends a virtual node built by
/// `006f4f00` from `info`, `point` and `door`; returns its index.
pub fn fn_006c9a70(e: &mut Engine, this: u32, info: u32, point: u32, door: u32) -> u32 {
    add_virtual_node(
        e,
        this,
        VIRTUAL_NODE_CONSTRUCT_INFO_POINT_DOOR,
        &[info, point, door],
    )
}

/// The exits of `PathBuilder::BuildTeleportDoorPath`: the path, then the
/// search, are destroyed.
fn finish_teleport_path(e: &mut Engine, search: u32, path: u32, result: bool) -> bool {
    e.call(TELEPORT_PATH_DESTRUCT, &[path]);
    e.call(TELEPORT_DOOR_SEARCH_DESTRUCT, &[search]);
    result
}

// Translated from 006c9b20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006c9b20 - the C++ exception-unwinding frame (FS:[0]) that destroys the teleport path and the door search when a callee throws is not translated
/// `PathBuilder::BuildTeleportDoorPath` (Xbox PDB): runs the teleport door
/// search (`TeleportDoorSearch::BuildNodePath`) from the request's origin
/// to its destination into a `TeleportPath` (its `Spaces` and `Doors`
/// arrays reserved to 10 first), then builds the navmesh info path of
/// every space with `PathBuilder::BuildNavMeshInfoPathInSpace`: origin to
/// the first door, door to door, last door to the destination. A space
/// that fails marks the solution incomplete (`fn_006c9fa0`) and returns
/// false; otherwise the door search's result is returned.
pub fn path_builder_build_teleport_door_path(e: &mut Engine, this: Ptr<PathBuilder>) -> bool {
    with_frame(e, 0x2170, |e, f| {
        let search = f.at(-0x20cc);
        let path = f.at(-0x2104);
        let doors = f.at(-0x20f4);
        e.call(TELEPORT_DOOR_SEARCH_CONSTRUCT, &[search]);
        e.call(TELEPORT_PATH_CONSTRUCT, &[path]);
        fn_006cee30(e, path, 10);
        fn_006ceea0(e, doors, 10);
        let req = request(e, this);
        let mut request_5c = 0;
        if !call_bool(e, REQUEST_BYTE_98, &[req]) {
            request_5c = fn_006c9900(e, req);
        }
        let destination = call_u32(e, REQUEST_DESTINATION, &[req]);
        let origin = call_u32(e, REQUEST_ORIGIN, &[req]);
        let found = call_bool(
            e,
            TELEPORT_DOOR_SEARCH_BUILD_NODE_PATH,
            &[
                search,
                origin,
                destination,
                path,
                0,
                0.0f32.to_bits(),
                request_5c,
                0,
            ],
        );
        let sol = solution(e, this);
        if array_size(e, path) <= 1 {
            if found {
                let space = call_u32(e, SPACE_ELEMENT, &[path, 0]);
                let destination = call_u32(e, REQUEST_DESTINATION, &[req]);
                let goal_point = location_point(e, destination, f.at(-0x2148));
                let origin = call_u32(e, REQUEST_ORIGIN, &[req]);
                let start_point = location_point(e, origin, f.at(-0x2154));
                if !path_builder_build_nav_mesh_info_path_in_space(
                    e,
                    this,
                    start_point,
                    goal_point,
                    space,
                    0,
                ) {
                    return finish_teleport_path(e, search, path, false);
                }
            }
            return finish_teleport_path(e, search, path, found);
        }
        let origin = call_u32(e, REQUEST_ORIGIN, &[req]);
        location_point(e, origin, f.at(-0x2120));
        let link = call_u32(e, FIRST_LINK_ELEMENT, &[doors]);
        let door = e.mem.u32(link);
        let position = call_u32(e, REFERENCE_LINKED_DOOR_POSITION, &[door]);
        copy_point(e, f.at(-0x2114), position);
        let link = call_u32(e, FIRST_LINK_ELEMENT, &[doors]);
        let door = e.mem.u32(link);
        let space = fn_006ced70(e, path);
        if !path_builder_build_nav_mesh_info_path_in_space(
            e,
            this,
            f.at(-0x2120),
            f.at(-0x2114),
            space,
            door,
        ) {
            return finish_teleport_path(e, search, path, false);
        }
        let mut index = 0u32;
        while index < array_size(e, doors).wrapping_sub(1) {
            let link = call_u32(e, LINK_ELEMENT, &[doors, index]);
            copy_point(e, f.at(-0x213c), link + 4);
            let next = call_u32(e, LINK_ELEMENT, &[doors, index + 1]);
            let next_door = e.mem.u32(next);
            let position = call_u32(e, REFERENCE_LINKED_DOOR_POSITION, &[next_door]);
            copy_point(e, f.at(-0x2130), position);
            let next = call_u32(e, LINK_ELEMENT, &[doors, index + 1]);
            let next_door = e.mem.u32(next);
            let space = call_u32(e, SPACE_ELEMENT, &[path, index + 1]);
            if !path_builder_build_nav_mesh_info_path_in_space(
                e,
                this,
                f.at(-0x213c),
                f.at(-0x2130),
                space,
                next_door,
            ) {
                fn_006c9fa0(e, sol, true);
                return finish_teleport_path(e, search, path, false);
            }
            index += 1;
        }
        let last = fn_006cedb0(e, doors);
        copy_point(e, f.at(-0x2120), last + 4);
        let destination = call_u32(e, REQUEST_DESTINATION, &[req]);
        let goal_point = location_point(e, destination, f.at(-0x2164));
        copy_point(e, f.at(-0x2114), goal_point);
        let space = fn_006ced90(e, path);
        if !path_builder_build_nav_mesh_info_path_in_space(
            e,
            this,
            f.at(-0x2120),
            f.at(-0x2114),
            space,
            0,
        ) {
            fn_006c9fa0(e, sol, true);
            return finish_teleport_path(e, search, path, false);
        }
        finish_teleport_path(e, search, path, found)
    })
}

// Translated from 006c9fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall on a `PathingSolution`: stores `value` in `bIncompletePath`
/// (+0x40).
pub fn fn_006c9fa0(e: &mut Engine, this: u32, value: bool) {
    e.set(
        Ptr::<PathingSolution>::new(this),
        PathingSolution::bIncompletePath,
        value,
    );
}

/// Whether virtual node `index`'s location has a cell for which
/// `00450ff0` holds (the run test of `fn_006c9fc0`).
fn node_cell_passes(e: &mut Engine, solution: u32, index: u32) -> bool {
    let node = virtual_node(e, solution, index);
    let location = node_location(e, node);
    let cell = call_u32(e, LOCATION_GET_CELL, &[location]);
    cell != 0 && call_bool(e, CELL_TEST_00450FF0, &[cell])
}

// Translated from 006c9fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds the run of virtual nodes from `index` whose cells pass
/// `00450ff0`: false when node `index` fails; otherwise `*first = index`
/// and `*last` the last node of the run, which also ends after the first
/// door node (`fn_006ca0c0`).
pub fn fn_006c9fc0(
    e: &mut Engine,
    this: Ptr<PathBuilder>,
    index: i32,
    first: u32,
    last: u32,
) -> bool {
    let sol = solution(e, this);
    if !node_cell_passes(e, sol, index as u32) {
        return false;
    }
    e.mem.set_i32(first, index);
    e.mem.set_i32(last, e.mem.i32(first).wrapping_add(1));
    loop {
        let count = call_u32(e, SOLUTION_VIRTUAL_NODE_COUNT, &[sol]) as i32;
        if e.mem.i32(last) >= count {
            break;
        }
        let current = e.mem.u32(last);
        if !node_cell_passes(e, sol, current) {
            break;
        }
        let current = e.mem.u32(last);
        let node = virtual_node(e, sol, current);
        if fn_006ca0c0(e, node) {
            e.mem.set_i32(last, e.mem.i32(last).wrapping_add(1));
            break;
        }
        e.mem.set_i32(last, e.mem.i32(last).wrapping_add(1));
    }
    e.mem.set_i32(last, e.mem.i32(last).wrapping_sub(1));
    true
}

// Translated from 006ca0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 2 (`4`) of the node's `iFlags` (Xbox PDB, +0) is set (the
/// door flag `fn_006cc5a0` sets).
pub fn fn_006ca0c0(e: &mut Engine, this: u32) -> bool {
    e.mem.u32(this) & 4 == 4
}

/// Copies the locations of the current nodes `near` and `far` into the
/// stack locations `near_copy` and `far_copy` (`fn_006ca0e0`).
fn copy_node_locations(e: &mut Engine, near: u32, far: u32, near_copy: u32, far_copy: u32) {
    let location = node_location(e, near);
    e.call(LOCATION_COPY_CONSTRUCT, &[near_copy, location]);
    let location = node_location(e, far);
    e.call(LOCATION_COPY_CONSTRUCT, &[far_copy, location]);
}

// Translated from 006ca0e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006ca0e0 - the C++ exception-unwinding frame (FS:[0]) that destroys the location copies when a callee throws is not translated
/// Builds the detailed path of the loaded run of virtual nodes from
/// `index`: resets the solution (`006c93a0`), finds the run
/// (`fn_006c9fc0`; none: returns true), stores it as the first and last
/// loaded virtual node, builds the nodes (`fn_006ca850`). With the
/// request's first tangent flag (`fn_006ca4c0`) on a run that starts at 0
/// the first current node gets the start tangent (`006d99a0`); when the
/// build succeeded, the request's byte +0x99 is set and the run ends at the
/// last virtual node, the last current node gets the end tangent
/// (`006da420`). `segment_word` is passed on to `fn_006ca850`, which does
/// not read it. Returns whether the nodes were built (false also when
/// nothing was and there are no current nodes).
pub fn fn_006ca0e0(e: &mut Engine, this: Ptr<PathBuilder>, segment_word: u32, index: i32) -> bool {
    with_frame(e, 0x160, |e, f| {
        let sol = solution(e, this);
        e.call(SOLUTION_RESET, &[sol]);
        e.mem.set_i32(f.at(-0x10), -1);
        e.mem.set_i32(f.at(-0x14), -1);
        if !fn_006c9fc0(e, this, index, f.at(-0x10), f.at(-0x14)) {
            return true;
        }
        let solution_ptr = Ptr::<PathingSolution>::new(sol);
        let first = e.mem.i32(f.at(-0x10));
        e.set(
            solution_ptr,
            PathingSolution::iFirstLoadedVirtualNodeIndex,
            first,
        );
        let last = e.mem.i32(f.at(-0x14));
        e.set(
            solution_ptr,
            PathingSolution::iLastLoadedVirtualNodeIndex,
            last,
        );
        let built = fn_006ca850(e, this, segment_word);
        if !built && call_u32(e, SOLUTION_CURRENT_NODE_COUNT, &[sol]) == 0 {
            return false;
        }
        let req = request(e, this);
        if fn_006ca4c0(e, req)
            && call_u32(e, SOLUTION_FIRST_LOADED, &[sol]) == 0
            && call_u32(e, SOLUTION_CURRENT_NODE_COUNT, &[sol]) >= 2
        {
            let near = call_u32(e, SOLUTION_CURRENT_NODE, &[sol, 0]);
            let far = call_u32(e, SOLUTION_CURRENT_NODE, &[sol, 1]);
            if near != 0 && far != 0 {
                let (near_copy, far_copy) = (f.at(-0x70), f.at(-0x48));
                copy_node_locations(e, near, far, near_copy, far_copy);
                let near_point = location_point(e, near_copy, f.at(-0x120));
                let far_point = location_point(e, far_copy, f.at(-0x114));
                let offset = call_u32(e, POINT3_SUBTRACT, &[far_point, f.at(-0x12c), near_point]);
                let distance = e.call(POINT3_LENGTH_SQUARED, &[offset]).f64();
                if distance > e.global::<f64>(TANGENT_DISTANCE_01017A10) {
                    let radius = e.call(REQUEST_ACTOR_RADIUS, &[req]).f32();
                    let heading = fn_006ca4e0(e, req);
                    e.call(
                        BUILD_FIRST_TANGENT,
                        &[
                            f.at(-0x7c),
                            heading.to_bits(),
                            near_copy,
                            far_copy,
                            radius.to_bits(),
                        ],
                    );
                    e.call(NODE_SET_POSITION, &[near, f.at(-0x7c)]);
                }
                e.call(LOCATION_DESTRUCT, &[far_copy]);
                e.call(LOCATION_DESTRUCT, &[near_copy]);
            }
        }
        if built && call_bool(e, REQUEST_BYTE_99, &[req]) {
            let last_loaded = call_u32(e, WORD_AT_1C, &[sol]);
            let count = call_u32(e, SOLUTION_VIRTUAL_NODE_COUNT, &[sol]);
            if last_loaded == count.wrapping_sub(1) {
                let count = call_u32(e, SOLUTION_CURRENT_NODE_COUNT, &[sol]);
                let near = call_u32(e, SOLUTION_CURRENT_NODE, &[sol, count.wrapping_sub(1)]);
                let count = call_u32(e, SOLUTION_CURRENT_NODE_COUNT, &[sol]);
                let far = call_u32(e, SOLUTION_CURRENT_NODE, &[sol, count.wrapping_sub(2)]);
                if near != 0 && far != 0 {
                    let (near_copy, far_copy) = (f.at(-0xac), f.at(-0xd4));
                    copy_node_locations(e, near, far, near_copy, far_copy);
                    let far_point = location_point(e, far_copy, f.at(-0x144));
                    let near_point = location_point(e, near_copy, f.at(-0x138));
                    let offset =
                        call_u32(e, POINT3_SUBTRACT, &[near_point, f.at(-0x150), far_point]);
                    let distance = e.call(POINT3_LENGTH_SQUARED, &[offset]).f64();
                    if distance > e.global::<f64>(TANGENT_DISTANCE_01017A10) {
                        let end_copy = f.at(-0x108);
                        let location = node_location(e, near);
                        e.call(LOCATION_COPY_CONSTRUCT, &[end_copy, location]);
                        let radius = e.call(REQUEST_ACTOR_RADIUS, &[req]).f32();
                        let value = e.call(REQUEST_FLOAT_8C, &[req]).f32();
                        e.call(
                            BUILD_LAST_TANGENT,
                            &[f.at(-0xe0), value.to_bits(), end_copy, radius.to_bits()],
                        );
                        e.call(NODE_SET_POSITION, &[near, f.at(-0xe0)]);
                        e.call(LOCATION_DESTRUCT, &[end_copy]);
                    }
                    e.call(LOCATION_DESTRUCT, &[far_copy]);
                    e.call(LOCATION_DESTRUCT, &[near_copy]);
                }
            }
        }
        built
    })
}

// Translated from 006ca4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The request's `bFirstTangentUsesHeading` (Xbox PDB, +0xA0).
pub fn fn_006ca4c0(e: &mut Engine, this: u32) -> bool {
    e.get(
        Ptr::<PathingRequest>::new(this),
        PathingRequest::bFirstTangentUsesHeading,
    )
}

// Translated from 006ca4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The request's `fInitialPathHeading` (Xbox PDB, +0x90), as `FLD` loads
/// it.
pub fn fn_006ca4e0(e: &mut Engine, this: u32) -> f32 {
    let bits = e.mem.u32(this + 0x90);
    f32::from_bits(x87_quiet_bits(bits))
}

// Translated from 006ca500 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006ca500 - the C++ exception-unwinding frame (FS:[0]) that destroys the flag array when a callee throws is not translated
/// Splits the loaded virtual nodes (first to last loaded index) into
/// `PathBuilder::VirtualNodeSegment`s appended to `segments` (cleared
/// first): runs of nodes without bit 0 (`fn_006ca830`) are segments without
/// navmeshes; a run with it is cut wherever two neighbouring nodes have
/// different navmesh infos that do not list each other (the arrays at
/// +0x24 and +0x34 of the info, `00719b20`). False when a node of such a
/// run has no navmesh info.
pub fn fn_006ca500(e: &mut Engine, this: Ptr<PathBuilder>, segments: u32) -> bool {
    with_frame(e, 0x98, |e, f| {
        e.call(ARRAY_CLEAR_008454F0, &[segments, 0]);
        let sol = solution(e, this);
        let solution_ptr = Ptr::<PathingSolution>::new(sol);
        let first = e.get(solution_ptr, PathingSolution::iFirstLoadedVirtualNodeIndex);
        let last = e.get(solution_ptr, PathingSolution::iLastLoadedVirtualNodeIndex);
        let flags = f.at(-0x28);
        let heap = scrap_heap(e, this);
        let count = last.wrapping_sub(first).wrapping_add(1) as u32;
        e.call(BOOL_SCRAP_ARRAY_CONSTRUCT, &[flags, count, 0, heap]);
        let mut index = first;
        while index <= last {
            let node = virtual_node(e, sol, index as u32);
            let value = fn_006ca830(e, node);
            e.mem.set_u8(f.at(-0x85), u8::from(value));
            e.call(BOOL_SCRAP_ARRAY_ADD, &[flags, f.at(-0x85)]);
            index = index.wrapping_add(1);
        }
        let mut start = first;
        while start <= last {
            e.mem.set_u8(f.at(-0x86), 1);
            let found = e
                .call(
                    BOOL_ARRAY_FIND,
                    &[
                        flags,
                        f.at(-0x86),
                        start.wrapping_sub(first) as u32,
                        BOOL_EQUALS_006CF760,
                    ],
                )
                .i32();
            if found == -1 {
                add_segment(e, segments, f.at(-0x84), start, last, false);
                start = last.wrapping_add(1);
                continue;
            }
            e.mem.set_u8(f.at(-0x87), 0);
            let mut end = e
                .call(
                    BOOL_ARRAY_FIND,
                    &[flags, f.at(-0x87), found as u32, BOOL_EQUALS_006CF760],
                )
                .i32()
                .wrapping_sub(1);
            if end == -2 {
                end = (array_size(e, flags) as i32).wrapping_sub(1);
            }
            let found = found.wrapping_add(first);
            let end = end.wrapping_add(first);
            if found > start {
                add_segment(
                    e,
                    segments,
                    f.at(-0x5c),
                    start,
                    found.wrapping_sub(1),
                    false,
                );
            }
            let mut current = found;
            let mut node_index = found;
            while node_index.wrapping_add(1) <= end {
                let node = virtual_node(e, sol, node_index as u32);
                let next = virtual_node(e, sol, node_index.wrapping_add(1) as u32);
                let location = node_location(e, node);
                e.call(LOCATION_RESOLVE, &[location]);
                let location = node_location(e, next);
                e.call(LOCATION_RESOLVE, &[location]);
                let location = node_location(e, node);
                let info = call_u32(e, LOCATION_GET_BEST_NAVMESH_INFO, &[location]);
                let location = node_location(e, next);
                let next_info = call_u32(e, LOCATION_GET_BEST_NAVMESH_INFO, &[location]);
                e.mem.set_u32(f.at(-0x6c), next_info);
                if info == 0 || next_info == 0 {
                    e.call(BOOL_SCRAP_ARRAY_DESTRUCT, &[flags]);
                    return false;
                }
                if info != next_info
                    && e.call(
                        ARRAY_FIND,
                        &[info + 0x24, f.at(-0x6c), 0, POINTER_EQUALS_009A3830],
                    )
                    .i32()
                        == -1
                    && e.call(
                        ARRAY_FIND,
                        &[info + 0x34, f.at(-0x6c), 0, POINTER_EQUALS_009A3830],
                    )
                    .i32()
                        == -1
                {
                    add_segment(e, segments, f.at(-0x78), current, node_index, true);
                    current = node_index.wrapping_add(1);
                }
                node_index = node_index.wrapping_add(1);
            }
            add_segment(e, segments, f.at(-0x48), current, end, true);
            start = end.wrapping_add(1);
        }
        e.call(BOOL_SCRAP_ARRAY_DESTRUCT, &[flags]);
        true
    })
}

// Translated from 006ca830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 0 of the node's `iFlags` (Xbox PDB, +0) is set.
pub fn fn_006ca830(e: &mut Engine, this: u32) -> bool {
    e.mem.u32(this) & 1 != 0
}

// Translated from 006ca850 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006ca850 - the C++ exception-unwinding frame (FS:[0]) that destroys the segment array when a callee throws is not translated
/// Builds the current nodes of the loaded virtual nodes segment by segment
/// (`fn_006ca500`): a segment with navmeshes through `fn_006cb950`, one
/// without (or one whose build failed) through `fn_006cec40`. Unless the
/// request's byte +0xA1 is set (`fn_006caa20`), two navmesh segments in a
/// row mark the solution incomplete and fail, and so does a failed navmesh
/// segment. The code also compares the segment count with 0x100063 and
/// then only clears a local nothing reads; the comparison's array size call
/// is kept. The word argument is not read.
pub fn fn_006ca850(e: &mut Engine, this: Ptr<PathBuilder>, _unused_1: u32) -> bool {
    with_frame(e, 0x48, |e, f| {
        let segments = f.at(-0x24);
        let heap = scrap_heap(e, this);
        e.call(SEGMENT_SCRAP_ARRAY_CONSTRUCT, &[segments, 0, 0, heap]);
        if !fn_006ca500(e, this, segments) {
            e.call(SEGMENT_SCRAP_ARRAY_DESTRUCT, &[segments]);
            return false;
        }
        let req = request(e, this);
        let lenient = fn_006caa20(e, req);
        array_size(e, segments);
        let sol = solution(e, this);
        let mut index = 0u32;
        while index < array_size(e, segments) {
            let mut built = false;
            let segment = call_u32(e, SPACE_ELEMENT, &[segments, index]);
            if e.mem.u8(segment + 8) != 0 {
                if index > 0 {
                    let previous = call_u32(e, SPACE_ELEMENT, &[segments, index - 1]);
                    if e.mem.u8(previous + 8) != 0 && !lenient {
                        fn_006c9fa0(e, sol, true);
                        e.call(SEGMENT_SCRAP_ARRAY_DESTRUCT, &[segments]);
                        return false;
                    }
                }
                let (first, last) = (e.mem.u32(segment), e.mem.u32(segment + 4));
                built = fn_006cb950(e, this, first, last);
                if !built && !lenient {
                    e.call(SEGMENT_SCRAP_ARRAY_DESTRUCT, &[segments]);
                    return false;
                }
            }
            if !built {
                let (first, last) = (e.mem.u32(segment), e.mem.u32(segment + 4));
                if !fn_006cec40(e, this, first, last) {
                    e.call(SEGMENT_SCRAP_ARRAY_DESTRUCT, &[segments]);
                    return false;
                }
            }
            index += 1;
        }
        e.call(SEGMENT_SCRAP_ARRAY_DESTRUCT, &[segments]);
        true
    })
}

// Translated from 006caa20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The request's byte at +0xA1.
pub fn fn_006caa20(e: &mut Engine, this: u32) -> bool {
    e.mem.u8(this + 0xa1) != 0
}

/// The common end of `fn_006caa40`: the node's location itself when its
/// navmesh info resolves and is valid, otherwise what the edge search
/// (`006d4120`) gives (1: the location, 2: the path to a valid triangle).
fn add_resolved_location(
    e: &mut Engine,
    this: Ptr<PathBuilder>,
    location: u32,
    out: u32,
    edges: u32,
    flag: bool,
) -> i32 {
    if !call_bool(e, LOCATION_RESOLVE_NAVMESH_INFO, &[location, 0]) {
        return 2;
    }
    if call_bool(e, LOCATION_HAS_NAVMESH_INFO, &[location]) {
        fn_006cf1b0(e, out, location);
        return 1;
    }
    match e.call(FIND_EDGES, &[location, 10, edges]).i32() {
        1 => {
            fn_006cf1b0(e, out, location);
            1
        }
        2 => {
            path_builder_build_path_to_valid_nav_mesh_triangle(e, this, location, out, flag, edges)
        }
        _ => 2,
    }
}

// Translated from 006caa40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006caa40 - the C++ exception-unwinding frame (FS:[0]) that destroys the local location when a callee throws is not translated
/// Finds the start location of the segment that begins at virtual node
/// `index` and appends it to `out`: when the path is not an update and the
/// node is not the first, a location on the edge between the previous node
/// and this one (`fn_006cde50`; for the first loaded node only when the
/// previous one has bit 1, otherwise when this one lacks it); otherwise the
/// node's own location (`add_resolved_location`), with `edges` for the
/// edge search. Returns 1 on success, 2 on failure, or what
/// `PathBuilder::BuildPathToValidNavMeshTriangle` returns.
pub fn fn_006caa40(
    e: &mut Engine,
    this: Ptr<PathBuilder>,
    index: i32,
    out: u32,
    edges: u32,
) -> i32 {
    with_frame(e, 0x80, |e, f| {
        let sol = solution(e, this);
        let node = virtual_node(e, sol, index as u32);
        if index != 0 && !e.get(this, PathBuilder::bPathUpdate) {
            let previous = virtual_node(e, sol, index.wrapping_sub(1) as u32);
            let first_loaded = call_u32(e, SOLUTION_FIRST_LOADED, &[sol]) as i32;
            let local = if index > first_loaded {
                if fn_006cac70(e, previous) {
                    Some(f.at(-0x3c))
                } else {
                    None
                }
            } else if !fn_006cac70(e, node) {
                Some(f.at(-0x64))
            } else {
                None
            };
            if let Some(location) = local {
                e.call(LOCATION_CONSTRUCT, &[location]);
                if fn_006cde50(e, this, previous, node, location) {
                    fn_006cf1b0(e, out, location);
                    e.call(LOCATION_DESTRUCT, &[location]);
                    return 1;
                }
                e.call(LOCATION_DESTRUCT, &[location]);
            }
        }
        if !fn_006cac70(e, node) {
            return 2;
        }
        let location = node_location(e, node);
        add_resolved_location(e, this, location, out, edges, true)
    })
}

// Translated from 006cac70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 1 (`2`) of the node's `iFlags` (Xbox PDB, +0) is set.
pub fn fn_006cac70(e: &mut Engine, this: u32) -> bool {
    e.mem.u32(this) & 2 == 2
}

// Translated from 006cac90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006cac90 - the C++ exception-unwinding frame (FS:[0]) that destroys the local location when a callee throws is not translated
/// Finds the end location of the segment that ends at virtual node `index`
/// and appends it to `out`: unless the node is the last virtual node, a
/// location on the edge to the next node (`fn_006ce2d0`, also stored in
/// the node; before the last loaded node only when the next one has bit 1,
/// otherwise when this one is not a door node). Otherwise the node's own
/// location; when the edge search (`006d4120`) returns 2, the first edge
/// point within the target radius (planar) and the height limits becomes
/// the location, and without one the path to a valid triangle is built.
/// Returns 1 on success, 2 on failure.
pub fn fn_006cac90(
    e: &mut Engine,
    this: Ptr<PathBuilder>,
    index: i32,
    out: u32,
    edges: u32,
) -> i32 {
    with_frame(e, 0xd0, |e, f| {
        let sol = solution(e, this);
        let node = virtual_node(e, sol, index as u32);
        let count = call_u32(e, SOLUTION_VIRTUAL_NODE_COUNT, &[sol]);
        if index != count.wrapping_sub(1) as i32 {
            let next = virtual_node(e, sol, index.wrapping_add(1) as u32);
            let last_loaded = call_u32(e, WORD_AT_1C, &[sol]) as i32;
            let local = if index < last_loaded {
                if fn_006cac70(e, next) {
                    Some(f.at(-0x3c))
                } else {
                    None
                }
            } else if !fn_006ca0c0(e, node) {
                Some(f.at(-0x64))
            } else {
                None
            };
            if let Some(location) = local {
                e.call(LOCATION_CONSTRUCT, &[location]);
                if fn_006ce2d0(e, this, node, next, location) {
                    let node_loc = node_location(e, node);
                    e.call(LOCATION_ASSIGN, &[node_loc, location]);
                    fn_006cf1b0(e, out, location);
                    e.call(LOCATION_DESTRUCT, &[location]);
                    return 1;
                }
                e.call(LOCATION_DESTRUCT, &[location]);
            }
        }
        if !fn_006cac70(e, node) {
            return 2;
        }
        let location = node_location(e, node);
        if !call_bool(e, LOCATION_RESOLVE_NAVMESH_INFO, &[location, 0]) {
            return 2;
        }
        if call_bool(e, LOCATION_HAS_NAVMESH_INFO, &[location]) {
            fn_006cf1b0(e, out, location);
            return 1;
        }
        match e.call(FIND_EDGES, &[location, 10, edges]).i32() {
            1 => {
                fn_006cf1b0(e, out, location);
                1
            }
            2 => {
                let req = request(e, this);
                let mut edge_index = 0u32;
                while edge_index < array_size(e, edges) {
                    let edge = fn_006cee10(e, edges, edge_index);
                    let point = location_point(e, location, f.at(-0xb4));
                    call_u32(e, POINT3_SUBTRACT, &[point, f.at(-0x78), edge + 0xc]);
                    let planar = e.call(POINT2_LENGTH_SQUARED, &[f.at(-0x78)]).f64();
                    let first = e.call(REQUEST_TARGET_RADIUS, &[req]).f64();
                    let second = e.call(REQUEST_TARGET_RADIUS, &[req]).f64();
                    let height = f64::from(e.mem.f32(f.at(-0x70)));
                    if second * first > planar
                        && height > e.global::<f64>(HEIGHT_LOW_0106B9F0)
                        && height < e.global::<f64>(HEIGHT_HIGH_0104ED58)
                    {
                        let edge = fn_006cee10(e, edges, edge_index);
                        let triangle = u32::from(e.mem.u16(edge + 4));
                        let edge = fn_006cee10(e, edges, edge_index);
                        let info = e.mem.u32(edge);
                        let edge = fn_006cee10(e, edges, edge_index);
                        let local = f.at(-0xa0);
                        e.call(
                            LOCATION_CONSTRUCT_FROM_MESH_POINT,
                            &[local, edge + 0xc, info, triangle],
                        );
                        fn_006cf1b0(e, out, local);
                        e.call(LOCATION_DESTRUCT, &[local]);
                        return 1;
                    }
                    edge_index += 1;
                }
                path_builder_build_path_to_valid_nav_mesh_triangle(
                    e, this, location, out, false, edges,
                )
            }
            _ => 2,
        }
    })
}

// Translated from 006cb050 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006cb050 - the C++ exception-unwinding frame (FS:[0]) that destroys the ray-cast request, path and search when a callee throws is not translated
/// `PathBuilder::BuildPathToValidNavMeshTriangle` (Xbox PDB): with the
/// ray-cast setting (`011d7404`) on, runs the edge search (`006d4120`):
/// 0 fails (2), 1 appends `location` (1), 2 ray-casts from the edges to
/// the location, queued on the path manager when on the tasklet thread,
/// otherwise with a local `RayCastPathRequest` (type `!flag`) run until the
/// search stops returning 0; 1 copies the path's nodes into `out`. With the
/// setting off, the closest point on a navmesh triangle (`fn_006cdb40`) is
/// appended after `location`. Returns 1 or 2 (or the path manager's
/// result).
pub fn path_builder_build_path_to_valid_nav_mesh_triangle(
    e: &mut Engine,
    this: Ptr<PathBuilder>,
    location: u32,
    out: u32,
    flag: bool,
    edges: u32,
) -> i32 {
    with_frame(e, 0x180, |e, f| {
        if setting_bool(e, SETTING_RAY_CAST_011D7404) {
            match e.call(FIND_EDGES, &[location, 10, edges]).i32() {
                0 => 2,
                1 => {
                    fn_006cf1b0(e, out, location);
                    1
                }
                2 => {
                    if on_tasklet_thread(e) {
                        let manager = call_u32(e, PATH_MANAGER_INSTANCE, &[]);
                        let req = request(e, this);
                        return e
                            .call(
                                PATH_MANAGER_GET_RAY_CAST_PATH_TO_NAV_MESH,
                                &[manager, req, edges, location, out, u32::from(flag)],
                            )
                            .i32();
                    }
                    let ray_request = f.at(-0x7c);
                    ray_cast_path_request_ray_cast_path_request(e, ray_request);
                    e.call(LOCATION_ASSIGN, &[ray_request + 0x30, location]);
                    let ray_request_ptr = Ptr::<RayCastPathRequest>::new(ray_request);
                    e.set(ray_request_ptr, RayCastPathRequest::eType, u32::from(!flag));
                    let req = request(e, this);
                    e.set(
                        ray_request_ptr,
                        RayCastPathRequest::pInitialPathRequest,
                        req,
                    );
                    e.call(EDGE_ARRAY_COPY_TO, &[edges, ray_request + 0x58]);
                    let path = f.at(-0x12c);
                    fn_006cb480(e, path);
                    let search = f.at(-0x114);
                    e.call(RAY_CAST_SEARCH_CONSTRUCT, &[search]);
                    let mut result = e
                        .call(RAY_CAST_SEARCH_BUILD_PATH, &[search, ray_request, path])
                        .i32();
                    while result == 0 {
                        result = e
                            .call(RAY_CAST_SEARCH_BUILD_PATH, &[search, ray_request, path])
                            .i32();
                    }
                    let built = if result == 1 {
                        e.call(LOCATION_ARRAY_COPY_TO, &[path + 8, out]);
                        1
                    } else {
                        2
                    };
                    e.call(RAY_CAST_SEARCH_DESTRUCT, &[search]);
                    fn_006cb4a0(e, path);
                    fn_006cb410(e, ray_request);
                    built
                }
                _ => 2,
            }
        } else {
            let closest = f.at(-0x154);
            e.call(LOCATION_CONSTRUCT, &[closest]);
            let found = e.with_stack(PATHING_LOCATION_SIZE, |e, argument| {
                e.call(LOCATION_COPY_CONSTRUCT, &[argument.addr(), location]);
                fn_006cdb40(e, this, argument.addr(), closest, flag)
            });
            let result = if found {
                fn_006cf1b0(e, out, location);
                fn_006cf1b0(e, out, closest);
                1
            } else {
                2
            };
            e.call(LOCATION_DESTRUCT, &[closest]);
            result
        }
    })
}

// Translated from 006cb3a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006cb3a0 - the C++ exception-unwinding frame (FS:[0]) that destroys the constructed locations when a member constructor throws is not translated
/// `RayCastPathRequest::RayCastPathRequest` (Xbox PDB): constructs
/// `StartLocation`, `GoalLocation` and `Edges`; returns `this`.
pub fn ray_cast_path_request_ray_cast_path_request(e: &mut Engine, this: u32) -> u32 {
    e.call(LOCATION_CONSTRUCT, &[this + 0x08]);
    e.call(LOCATION_CONSTRUCT, &[this + 0x30]);
    e.call(EDGE_ARRAY_CONSTRUCT, &[this + 0x58]);
    this
}

// Translated from 006cb410 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006cb410 - the C++ exception-unwinding frame (FS:[0]) that keeps destroying members when a destructor throws is not translated
/// The `RayCastPathRequest` destructor: `Edges`, `GoalLocation`,
/// `StartLocation`, in that order.
pub fn fn_006cb410(e: &mut Engine, this: u32) {
    e.call(EDGE_ARRAY_DESTRUCT, &[this + 0x58]);
    e.call(LOCATION_DESTRUCT, &[this + 0x30]);
    e.call(LOCATION_DESTRUCT, &[this + 0x08]);
}

// Translated from 006cb480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs the `BSSimpleArray<PathingLocation>` at +8 (`fn_006cf180`;
/// the Xbox PDB's `RayCastPath::Nodes`); returns `this`.
pub fn fn_006cb480(e: &mut Engine, this: u32) -> u32 {
    fn_006cf180(e, this + 8);
    this
}

// Translated from 006cb4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the location array at +8 (`006abfc0`).
pub fn fn_006cb4a0(e: &mut Engine, this: u32) {
    e.call(LOCATION_ARRAY_DESTRUCT, &[this + 8]);
}

// Translated from 006cb4c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006cb4c0 - the C++ exception-unwinding frame (FS:[0]) that destroys the ray-cast request, path and search when a callee throws is not translated
/// `PathBuilder::BuildStraightPathUsingRayCasts` (Xbox PDB): with the
/// ray-cast setting (`011d7404`) on, ray-casts from `start` to `goal`
/// (queued on the path manager when on the tasklet thread, otherwise one
/// run of a type-2 `RayCastPathRequest`); on result 1 the path's nodes are
/// copied to `out`. Returns 1 on success, 2 otherwise (also with the
/// setting off).
pub fn path_builder_build_straight_path_using_ray_casts(
    e: &mut Engine,
    this: Ptr<PathBuilder>,
    start: u32,
    goal: u32,
    out: u32,
) -> i32 {
    with_frame(e, 0x138, |e, f| {
        if !setting_bool(e, SETTING_RAY_CAST_011D7404) {
            return 2;
        }
        if on_tasklet_thread(e) {
            let manager = call_u32(e, PATH_MANAGER_INSTANCE, &[]);
            let req = request(e, this);
            return e
                .call(
                    PATH_MANAGER_GET_RAY_CAST_PATH,
                    &[manager, req, start, goal, out],
                )
                .i32();
        }
        let ray_request = f.at(-0x74);
        ray_cast_path_request_ray_cast_path_request(e, ray_request);
        e.call(LOCATION_ASSIGN, &[ray_request + 0x08, start]);
        e.call(LOCATION_ASSIGN, &[ray_request + 0x30, goal]);
        let ray_request_ptr = Ptr::<RayCastPathRequest>::new(ray_request);
        e.set(ray_request_ptr, RayCastPathRequest::eType, 2);
        let req = request(e, this);
        e.set(
            ray_request_ptr,
            RayCastPathRequest::pInitialPathRequest,
            req,
        );
        let path = f.at(-0x124);
        fn_006cb480(e, path);
        let search = f.at(-0x10c);
        e.call(RAY_CAST_SEARCH_CONSTRUCT, &[search]);
        let result = e
            .call(RAY_CAST_SEARCH_BUILD_PATH, &[search, ray_request, path])
            .i32();
        let built = if result == 1 {
            e.call(LOCATION_ARRAY_COPY_TO, &[path + 8, out]);
            1
        } else {
            2
        };
        e.call(RAY_CAST_SEARCH_DESTRUCT, &[search]);
        fn_006cb4a0(e, path);
        fn_006cb410(e, ray_request);
        built
    })
}

// Translated from 006cb6a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006cb6a0 - the C++ exception-unwinding frame (FS:[0]) that releases the navmesh holder when a callee throws is not translated
/// Pulls the end of a segment onto the first edge of `edges`: the point of
/// the edge closest to virtual node `index` (`006b9c20`), accepted when it
/// is within the target radius (planar) and the height limits, is moved
/// towards its triangle's centre and becomes the only location of `out`.
pub fn fn_006cb6a0(
    e: &mut Engine,
    this: Ptr<PathBuilder>,
    index: u32,
    edges: u32,
    out: u32,
) -> bool {
    with_frame(e, 0xe0, |e, f| {
        let first = fn_006cedf0(e, edges);
        let edge = f.at(-0x40);
        fn_006cb900(e, edge, first);
        e.call(POINT3_DEFAULT_CONSTRUCT, &[f.at(-0x18)]);
        e.call(POINT3_DEFAULT_CONSTRUCT, &[f.at(-0x28)]);
        let holder = f.at(-0x1c);
        e.call(NAV_HOLDER_CONSTRUCT, &[holder]);
        let info = e.mem.u32(edge);
        if call_bool(e, NAVMESH_INFO_LOAD_HOLDER, &[info, holder]) {
            let navmesh = call_u32(e, NI_POINTER_GET, &[holder]);
            let triangle = u32::from(e.mem.u16(edge + 4));
            let edge_index = e.mem.u32(edge + 8);
            e.call(
                NAVMESH_EDGE_POINTS,
                &[navmesh, triangle, edge_index, f.at(-0x18), f.at(-0x28)],
            );
            let sol = solution(e, this);
            let node = virtual_node(e, sol, index);
            let location = node_location(e, node);
            location_point(e, location, f.at(-0x58));
            e.call(
                CLOSEST_ON_EDGE_006B9C20,
                &[f.at(-0x4c), f.at(-0x58), f.at(-0x18), f.at(-0x28)],
            );
            let offset = call_u32(e, POINT3_SUBTRACT, &[f.at(-0x4c), f.at(-0x98), f.at(-0x58)]);
            let planar = e.call(POINT2_LENGTH_SQUARED, &[offset]).f64();
            let req = request(e, this);
            let first_radius = e.call(REQUEST_TARGET_RADIUS, &[req]).f64();
            let second_radius = e.call(REQUEST_TARGET_RADIUS, &[req]).f64();
            if second_radius * first_radius > planar {
                let offset = call_u32(e, POINT3_SUBTRACT, &[f.at(-0x4c), f.at(-0xa4), f.at(-0x58)]);
                let height = f64::from(e.mem.f32(offset + 8));
                if height > e.global::<f64>(HEIGHT_LOW_0106B9F0) {
                    let offset =
                        call_u32(e, POINT3_SUBTRACT, &[f.at(-0x4c), f.at(-0xb0), f.at(-0x58)]);
                    let height = f64::from(e.mem.f32(offset + 8));
                    if height < e.global::<f64>(HEIGHT_HIGH_0104ED58) {
                        let triangle = u32::from(e.mem.u16(edge + 4));
                        let navmesh = call_u32(e, NI_POINTER_GET, &[holder]);
                        e.call(NAVMESH_GET_CENTER, &[navmesh, f.at(-0x64), triangle]);
                        let weight = float_constant_bits(e, CENTER_WEIGHT_0101E2BC);
                        let toward =
                            call_u32(e, POINT3_SUBTRACT, &[f.at(-0x64), f.at(-0xbc), f.at(-0x4c)]);
                        let scaled =
                            call_u32(e, POINT3_TIMES_SCALAR, &[toward, f.at(-0xc8), weight]);
                        e.call(POINT3_ADD_ASSIGN, &[f.at(-0x4c), scaled]);
                        e.call(LOCATION_ARRAY_CLEAR, &[out, 0]);
                        let triangle = u32::from(e.mem.u16(edge + 4));
                        let info = e.mem.u32(edge);
                        let local = f.at(-0x8c);
                        e.call(
                            LOCATION_CONSTRUCT_FROM_MESH_POINT,
                            &[local, f.at(-0x4c), info, triangle],
                        );
                        fn_006cf1b0(e, out, local);
                        e.call(LOCATION_DESTRUCT, &[local]);
                        e.call(NAV_HOLDER_RELEASE, &[holder]);
                        return true;
                    }
                }
            }
        }
        e.call(NAV_HOLDER_RELEASE, &[holder]);
        false
    })
}

// Translated from 006cb900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshEdgeLocation` copy constructor: the edge handle (`006c4350`),
/// then `Location` (+0x0C); returns `this`.
pub fn fn_006cb900(e: &mut Engine, this: u32, source: u32) -> u32 {
    e.call(EDGE_HANDLE_COPY, &[this, source]);
    e.call(POINT3_DEFAULT_CONSTRUCT, &[this + 0xc]);
    copy_point(e, this + 0xc, source + 0xc);
    this
}

/// The door step after a segment's nodes (`fn_006cb950`): a door virtual
/// node `last` passes its door to the last current node
/// (`PathingNode::SetTeleportDoor`); otherwise when the next virtual node
/// is in the same worldspace (non-null), its location is appended and the
/// last current node gets the door flag (`fn_006cc5a0`).
fn link_segment_end(e: &mut Engine, solution: u32, last: u32) {
    let node = virtual_node(e, solution, last);
    if fn_006ca0c0(e, node) {
        let door = call_u32(e, VIRTUAL_NODE_DOOR, &[node]);
        let current = call_u32(e, SOLUTION_LAST_CURRENT_NODE, &[solution]);
        pathing_node_set_teleport_door(e, current, door);
        return;
    }
    let location = node_location(e, node);
    if call_u32(e, WORD_AT_1C, &[location]) == 0 {
        return;
    }
    if call_u32(e, SOLUTION_VIRTUAL_NODE_COUNT, &[solution]) <= last.wrapping_add(1) {
        return;
    }
    let next = virtual_node(e, solution, last.wrapping_add(1));
    let next_location = node_location(e, next);
    let next_space = call_u32(e, WORD_AT_1C, &[next_location]);
    let location = node_location(e, node);
    if next_space != call_u32(e, WORD_AT_1C, &[location]) {
        return;
    }
    let next = virtual_node(e, solution, last.wrapping_add(1));
    let next_location = node_location(e, next);
    e.call(
        SOLUTION_ADD_CURRENT_NODE,
        &[solution, next_location, POINT_011F426C],
    );
    let current = call_u32(e, SOLUTION_LAST_CURRENT_NODE, &[solution]);
    fn_006cc5a0(e, current, true);
}

/// Appends every location of `locations`, in order, as a current node.
fn add_location_nodes(e: &mut Engine, solution: u32, locations: u32) {
    let mut index = 0u32;
    while index < array_size(e, locations) {
        let location = call_u32(e, LOCATION_ELEMENT, &[locations, index]);
        e.call(
            SOLUTION_ADD_CURRENT_NODE,
            &[solution, location, POINT_011F426C],
        );
        index += 1;
    }
}

/// The four arrays of `fn_006cb950`, destroyed in the game's order.
#[derive(Clone, Copy)]
struct SegmentArrays {
    start_edges: u32,
    start_locations: u32,
    end_edges: u32,
    end_locations: u32,
}

impl SegmentArrays {
    fn destroy(self, e: &mut Engine) {
        e.call(LOCATION_ARRAY_DESTRUCT, &[self.end_locations]);
        e.call(EDGE_ARRAY_DESTRUCT, &[self.end_edges]);
        e.call(LOCATION_ARRAY_DESTRUCT, &[self.start_locations]);
        e.call(EDGE_ARRAY_DESTRUCT, &[self.start_edges]);
    }
}

/// The planar distance from edge 0 of `edges` to virtual node `index`
/// (`00589850`), or 0 when `edges` is empty.
fn edge_gap(
    e: &mut Engine,
    solution: u32,
    edges: u32,
    index: u32,
    f: Frame,
    slots: [i32; 3],
) -> f32 {
    if call_bool(e, ARRAY_IS_EMPTY, &[edges]) {
        return 0.0;
    }
    let edge = fn_006cee10(e, edges, 0);
    copy_point(e, f.at(slots[0]), edge + 0xc);
    let node = virtual_node(e, solution, index);
    let location = node_location(e, node);
    let point = location_point(e, location, f.at(slots[1]));
    let offset = call_u32(e, POINT3_SUBTRACT, &[f.at(slots[0]), f.at(slots[2]), point]);
    e.call(POINT2_LENGTH, &[offset]).f32()
}

// Translated from 006cb950 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006cb950 - the C++ exception-unwinding frame (FS:[0]) that destroys the four location and edge arrays when a callee throws is not translated
/// Builds the current nodes of a navmesh segment from virtual node `first`
/// to `last`: the start locations (`fn_006caa40`) and end locations
/// (`fn_006cac90`); with a short span (shorter than the end gaps times a
/// factor, or a failed end that is closer than the short-span limit) a
/// straight ray-cast path is tried first. Otherwise the start locations,
/// the navmesh path (`fn_006cc5e0`) and the end locations (reversed) are
/// appended, with a pulled end point (`fn_006cb6a0`) when the end failed;
/// a navmesh path that fails with 2 retries the straight path when the
/// span is short and the radius setting (`011d6ef0`) is above the actor
/// radius and the line of sight is blocked. Returns whether the solution
/// is complete.
pub fn fn_006cb950(e: &mut Engine, this: Ptr<PathBuilder>, first: u32, last: u32) -> bool {
    with_frame(e, 0x160, |e, f| {
        let arrays = SegmentArrays {
            start_edges: f.at(-0x44),
            start_locations: f.at(-0x54),
            end_edges: f.at(-0x64),
            end_locations: f.at(-0x2c),
        };
        e.call(EDGE_ARRAY_CONSTRUCT, &[arrays.start_edges]);
        fn_006cf180(e, arrays.start_locations);
        let start_result = fn_006caa40(
            e,
            this,
            first as i32,
            arrays.start_locations,
            arrays.start_edges,
        );
        e.call(EDGE_ARRAY_CONSTRUCT, &[arrays.end_edges]);
        fn_006cf180(e, arrays.end_locations);
        let end_result = fn_006cac90(e, this, last as i32, arrays.end_locations, arrays.end_edges);
        if start_result == 0 || end_result == 0 {
            arrays.destroy(e);
            return false;
        }
        let sol = solution(e, this);
        let req = request(e, this);
        let start_gap = edge_gap(e, sol, arrays.start_edges, first, f, [-0x74, -0xf8, -0x104]);
        let end_gap = edge_gap(e, sol, arrays.end_edges, last, f, [-0x80, -0x110, -0x11c]);
        let node = virtual_node(e, sol, last);
        let location = node_location(e, node);
        let end_point = location_point(e, location, f.at(-0x134));
        let node = virtual_node(e, sol, first);
        let location = node_location(e, node);
        let start_point = location_point(e, location, f.at(-0x128));
        let offset = call_u32(e, POINT3_SUBTRACT, &[start_point, f.at(-0x140), end_point]);
        let span = f64::from(e.call(POINT2_LENGTH, &[offset]).f32());
        let mut straight = false;
        let limit =
            (f64::from(start_gap) + f64::from(end_gap)) * e.global::<f64>(GAP_FACTOR_01011590);
        if span < limit {
            straight = true;
        }
        if (start_result == 2 || end_result == 2) && span < e.global::<f64>(SHORT_SPAN_01028338) {
            straight = true;
        }
        if straight {
            let path = f.at(-0x94);
            fn_006cf180(e, path);
            let node = virtual_node(e, sol, last);
            let goal = node_location(e, node);
            let node = virtual_node(e, sol, first);
            let start = node_location(e, node);
            match path_builder_build_straight_path_using_ray_casts(e, this, start, goal, path) {
                0 => {
                    e.call(LOCATION_ARRAY_DESTRUCT, &[path]);
                    arrays.destroy(e);
                    return false;
                }
                1 => {
                    add_location_nodes(e, sol, path);
                    link_segment_end(e, sol, last);
                    e.call(LOCATION_ARRAY_DESTRUCT, &[path]);
                    arrays.destroy(e);
                    return true;
                }
                _ => {
                    e.call(LOCATION_ARRAY_DESTRUCT, &[path]);
                }
            }
        }
        if start_result == 2 {
            arrays.destroy(e);
            return false;
        }
        if end_result == 2 {
            if call_bool(e, ARRAY_IS_EMPTY, &[arrays.end_edges]) {
                arrays.destroy(e);
                return false;
            }
            if !fn_006cb6a0(e, this, last, arrays.end_edges, arrays.end_locations) {
                fn_006c9fa0(e, sol, true);
            }
            let edge = fn_006cedf0(e, arrays.end_edges);
            let triangle = u32::from(e.mem.u16(edge + 4));
            let edge = fn_006cedf0(e, arrays.end_edges);
            let info = e.mem.u32(edge);
            let edge = fn_006cedf0(e, arrays.end_edges);
            let local = f.at(-0xc4);
            e.call(
                LOCATION_CONSTRUCT_FROM_MESH_POINT,
                &[local, edge + 0xc, info, triangle],
            );
            fn_006cf1b0(e, arrays.end_locations, local);
            e.call(LOCATION_DESTRUCT, &[local]);
        }
        let mut index = 0u32;
        while index < array_size(e, arrays.start_locations).wrapping_sub(1) {
            let location = call_u32(e, LOCATION_ELEMENT, &[arrays.start_locations, index]);
            e.call(SOLUTION_ADD_CURRENT_NODE, &[sol, location, POINT_011F426C]);
            index += 1;
        }
        let end_location = fn_006cedd0(e, arrays.end_locations);
        let start_location = fn_006cedd0(e, arrays.start_locations);
        match fn_006cc5e0(e, this, start_location, end_location, first, last) {
            0 => {
                if !call_bool(e, ARRAY_IS_EMPTY, &[arrays.end_locations]) {
                    let mut index = (array_size(e, arrays.end_locations) as i32).wrapping_sub(1);
                    while index >= 0 {
                        let location =
                            call_u32(e, LOCATION_ELEMENT, &[arrays.end_locations, index as u32]);
                        e.call(SOLUTION_ADD_CURRENT_NODE, &[sol, location, POINT_011F426C]);
                        index -= 1;
                    }
                }
                link_segment_end(e, sol, last);
                let complete = !call_bool(e, SOLUTION_INCOMPLETE, &[sol]);
                arrays.destroy(e);
                complete
            }
            1 => {
                fn_006c9fa0(e, sol, true);
                arrays.destroy(e);
                false
            }
            2 => {
                if span < e.global::<f64>(SHORT_SPAN_01028338) {
                    let radius = e.call(REQUEST_ACTOR_RADIUS, &[req]).f64();
                    let setting = call_u32(e, SETTING_FLOAT_ADDRESS, &[SETTING_RADIUS_011D6EF0]);
                    if f64::from(e.mem.f32(setting)) > radius {
                        let node = virtual_node(e, sol, last);
                        let goal = node_location(e, node);
                        let node = virtual_node(e, sol, first);
                        let start = node_location(e, node);
                        if call_bool(e, CHECK_LINE_OF_SIGHT, &[start, goal, 0, 0]) {
                            arrays.destroy(e);
                            return false;
                        }
                        let path = f.at(-0xdc);
                        fn_006cf180(e, path);
                        let node = virtual_node(e, sol, last);
                        let goal = node_location(e, node);
                        let node = virtual_node(e, sol, first);
                        let start = node_location(e, node);
                        if path_builder_build_straight_path_using_ray_casts(
                            e, this, start, goal, path,
                        ) != 1
                        {
                            e.call(LOCATION_ARRAY_DESTRUCT, &[path]);
                            arrays.destroy(e);
                            return false;
                        }
                        fn_006c9fa0(e, sol, false);
                        add_location_nodes(e, sol, path);
                        let complete = !call_bool(e, SOLUTION_INCOMPLETE, &[sol]);
                        e.call(LOCATION_ARRAY_DESTRUCT, &[path]);
                        arrays.destroy(e);
                        return complete;
                    }
                }
                arrays.destroy(e);
                false
            }
            _ => {
                arrays.destroy(e);
                false
            }
        }
    })
}

// Translated from 006cc560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingNode::SetTeleportDoor` (Xbox PDB): stores the door in
/// `pDoorRef` (+0x38), moves `Location` to the door's linked position and
/// sets flag bits 0 and 1.
pub fn pathing_node_set_teleport_door(e: &mut Engine, this: u32, door: u32) {
    e.mem.set_u32(this + 0x38, door);
    let door = e.mem.u32(this + 0x38);
    let position = call_u32(e, REFERENCE_LINKED_DOOR_POSITION, &[door]);
    e.call(LOCATION_SET_POSITION, &[this + 4, position]);
    let flags = e.mem.u32(this);
    e.mem.set_u32(this, flags | 3);
}

// Translated from 006cc5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`value`) or clears bit 2 (`4`) of the node's `iFlags`.
pub fn fn_006cc5a0(e: &mut Engine, this: u32, value: bool) {
    let flags = e.mem.u32(this);
    let flags = if value {
        flags | 4
    } else {
        flags & 0xffff_fffb
    };
    e.mem.set_u32(this, flags);
}

/// The per-attempt locals of `fn_006cc5e0`, destroyed in the game's order.
#[derive(Clone, Copy)]
struct AttemptLocals {
    path_triangles: u32,
    extra_triangles: u32,
    start_copy: u32,
    goal_copy: u32,
}

impl AttemptLocals {
    fn destroy(self, e: &mut Engine) {
        e.call(LOCATION_DESTRUCT, &[self.goal_copy]);
        e.call(LOCATION_DESTRUCT, &[self.start_copy]);
        e.call(TRIANGLE_ARRAY_A_DESTRUCT, &[self.extra_triangles]);
        e.call(TRIANGLE_ARRAY_C_DESTRUCT, &[self.path_triangles]);
    }
}

/// Removes current nodes until there are `count` again (`006c9350`).
fn trim_current_nodes(e: &mut Engine, solution: u32, count: u32) {
    while call_u32(e, SOLUTION_CURRENT_NODE_COUNT, &[solution]) != count {
        e.call(SOLUTION_REMOVE_LAST_CURRENT_NODE, &[solution]);
    }
}

/// Collects the navmeshes of the virtual nodes into `DetailedPathMeshes`
/// and their infos into `DetailedPathInfos` (the start of `fn_006cc5e0`).
fn collect_path_meshes(e: &mut Engine, this: Ptr<PathBuilder>, solution: u32, f: Frame) {
    let infos = this.addr() + 0x14;
    let meshes = this.addr() + 0x24;
    let mut index = 0u32;
    while index < call_u32(e, SOLUTION_VIRTUAL_NODE_COUNT, &[solution]) {
        let holder = f.at(-0x2c);
        e.call(NAV_HOLDER_CONSTRUCT, &[holder]);
        let node = virtual_node(e, solution, index);
        let location = node_location(e, node);
        if call_bool(e, LOCATION_GET_NAVMESH, &[location, holder]) {
            e.call(NAVMESH_HOLDER_ARRAY_ADD, &[meshes, holder]);
            let navmesh = call_u32(e, NI_POINTER_GET, &[holder]);
            let info = call_u32(e, NAVMESH_GET_INFO, &[navmesh]);
            e.mem.set_u32(f.at(-0x140), info);
            e.call(ARRAY_PUSH_WORD, &[infos, f.at(-0x140)]);
        }
        e.call(NAV_HOLDER_RELEASE, &[holder]);
        index += 1;
    }
}

/// The second attempt of `fn_006cc5e0` first adds the nearby navmeshes
/// (`006d91d0`) that are not in `DetailedPathMeshes` yet.
fn add_nearby_meshes(e: &mut Engine, this: Ptr<PathBuilder>, f: Frame) {
    let infos = this.addr() + 0x14;
    let meshes = this.addr() + 0x24;
    let list = f.at(-0xb4);
    e.call(NAVMESH_LIST_CONSTRUCT, &[list]);
    e.call(NEARBY_NAVMESHES_FILL, &[list]);
    let mut index = 0u32;
    while index < array_size(e, list) {
        let item = call_u32(e, WORD_ELEMENT, &[list, index]);
        if e.call(ARRAY_FIND, &[meshes, item, 0, HOLDER_EQUALS_0042FF10])
            .i32()
            == -1
        {
            let item = call_u32(e, WORD_ELEMENT, &[list, index]);
            e.call(NAVMESH_HOLDER_ARRAY_ADD, &[meshes, item]);
            let item = call_u32(e, WORD_ELEMENT, &[list, index]);
            let navmesh = call_u32(e, NI_POINTER_GET, &[item]);
            let info = call_u32(e, NAVMESH_GET_INFO, &[navmesh]);
            e.mem.set_u32(f.at(-0x144), info);
            e.call(ARRAY_PUSH_WORD, &[infos, f.at(-0x144)]);
        }
        index += 1;
    }
    e.call(NAVMESH_LIST_DESTRUCT, &[list]);
}

/// Where `reaches_goal_node` takes its point from: a location, or a node
/// whose location it fetches after the goal's.
#[derive(Clone, Copy)]
enum PointSource {
    Location(u32),
    Node(u32),
}

/// Whether the point of `source` is within reach of the virtual node
/// `goal_node` (`fn_006cc5e0`'s goal test).
fn reaches_goal_node(
    e: &mut Engine,
    request: u32,
    goal_node: u32,
    source: PointSource,
    goal_out: u32,
    point_out: u32,
    offset: u32,
) -> bool {
    let location = node_location(e, goal_node);
    let goal_point = location_point(e, location, goal_out);
    let point_location = match source {
        PointSource::Location(location) => location,
        PointSource::Node(node) => node_location(e, node),
    };
    let point = location_point(e, point_location, point_out);
    call_u32(e, POINT3_SUBTRACT, &[point, offset, goal_point]);
    let reach = node_reach(e, request, goal_node);
    within_reach(e, reach, offset)
}

// Translated from 006cc5e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006cc5e0 - the C++ exception-unwinding frame (FS:[0]) that destroys the triangle arrays and location copies when a callee throws is not translated
/// Builds the navmesh part of a segment between the locations `start` and
/// `goal` (virtual nodes `first` to `last`). With the direct-line setting
/// (`011d73d8`) on, no avoid nodes and the request bytes +0x99 and +0xA2
/// clear, a free straight line (`fn_006cd1f0`) ends it (0). Otherwise the
/// navmeshes of the path are collected and up to the request's word +0xA8
/// (clamped to 1..10) attempts are made: the navmesh search
/// (`fn_006cd670`; on failure the end of the triangles found becomes the
/// goal, when it reaches the goal node or later attempts allow it), then
/// the smoothing (`fn_006cd8a0`). Returns 1 when the solution was marked
/// incomplete, 0 when not, 2 on failure.
pub fn fn_006cc5e0(
    e: &mut Engine,
    this: Ptr<PathBuilder>,
    start: u32,
    goal: u32,
    first: u32,
    last: u32,
) -> i32 {
    with_frame(e, 0x1b0, |e, f| {
        let req = request(e, this);
        let sol = solution(e, this);
        if setting_bool(e, SETTING_DIRECT_LINE_011D73D8)
            && call_u32(e, REQUEST_AVOID_NODE_ARRAY, &[req]) == 0
            && !call_bool(e, REQUEST_BYTE_99, &[req])
            && !call_bool(e, REQUEST_BYTE_A2, &[req])
            && fn_006cd1f0(e, this, start, goal)
        {
            return 0;
        }
        let search_triangles = f.at(-0x20);
        e.call(TRIANGLE_ARRAY_A_CONSTRUCT, &[search_triangles, 0x20, 0]);
        let mut incomplete = false;
        let tries = f.at(-0x24);
        let value = call_u32(e, REQUEST_WORD_A8, &[req]);
        e.mem.set_u32(tries, value);
        e.call(CLAMP_INTEGER, &[tries, 1, 10]);
        e.call(ARRAY_CLEAR_008454F0, &[this.addr() + 0x14, 0]);
        e.call(NAVMESH_HOLDER_ARRAY_CLEAR, &[this.addr() + 0x24, 0]);
        collect_path_meshes(e, this, sol, f);
        let locals = AttemptLocals {
            path_triangles: f.at(-0xa0),
            extra_triangles: f.at(-0x68),
            start_copy: f.at(-0x58),
            goal_copy: f.at(-0x90),
        };
        let mut attempt = 0i32;
        while attempt < e.mem.i32(tries) {
            if attempt == 1 {
                add_nearby_meshes(e, this, f);
            }
            e.call(
                TRIANGLE_ARRAY_C_CONSTRUCT,
                &[locals.path_triangles, 0x20, 0],
            );
            e.call(
                TRIANGLE_ARRAY_A_CONSTRUCT,
                &[locals.extra_triangles, 0x20, 0],
            );
            e.call(LOCATION_COPY_CONSTRUCT, &[locals.start_copy, start]);
            e.call(LOCATION_COPY_CONSTRUCT, &[locals.goal_copy, goal]);
            if !fn_006cd670(
                e,
                this,
                start,
                goal,
                search_triangles,
                locals.path_triangles,
                locals.extra_triangles,
                first,
                last,
            ) {
                if call_bool(e, ARRAY_IS_EMPTY, &[locals.path_triangles]) && attempt >= 1 {
                    locals.destroy(e);
                    e.call(TRIANGLE_ARRAY_A_DESTRUCT, &[search_triangles]);
                    return 2;
                }
                if call_bool(e, ARRAY_IS_EMPTY, &[locals.path_triangles]) {
                    locals.destroy(e);
                    attempt += 1;
                    continue;
                }
                let handle = call_u32(e, LAST_TRIANGLE_HANDLE, &[locals.path_triangles]);
                let triangle = u32::from(e.mem.u16(handle + 4));
                let navmesh = e.mem.u32(handle);
                let info = call_u32(e, NAVMESH_GET_INFO, &[navmesh]);
                let center = call_u32(e, TRIANGLE_HANDLE_CENTER, &[handle, f.at(-0x154)]);
                let reached_end = f.at(-0xe4);
                e.call(
                    LOCATION_CONSTRUCT_FROM_MESH_POINT,
                    &[reached_end, center, info, triangle],
                );
                if call_u32(e, WORD_AT_1C, &[sol]) == last {
                    let goal_node = virtual_node(e, sol, last);
                    if reaches_goal_node(
                        e,
                        req,
                        goal_node,
                        PointSource::Location(reached_end),
                        f.at(-0x16c),
                        f.at(-0x160),
                        f.at(-0xf8),
                    ) {
                        if !call_bool(e, REQUEST_BYTE_9B, &[req]) {
                            e.call(LOCATION_DESTRUCT, &[reached_end]);
                            locals.destroy(e);
                            e.call(TRIANGLE_ARRAY_A_DESTRUCT, &[search_triangles]);
                            return 2;
                        }
                        if !fn_006ca0c0(e, goal_node) {
                            incomplete = true;
                            fn_006c9fa0(e, sol, true);
                        }
                    } else if attempt == 0 {
                        e.call(LOCATION_DESTRUCT, &[reached_end]);
                        locals.destroy(e);
                        attempt += 1;
                        continue;
                    } else {
                        incomplete = true;
                        fn_006c9fa0(e, sol, true);
                    }
                } else {
                    incomplete = true;
                    fn_006c9fa0(e, sol, true);
                }
                if !call_bool(e, REQUEST_BYTE_9B, &[req]) {
                    e.call(LOCATION_DESTRUCT, &[reached_end]);
                    locals.destroy(e);
                    e.call(TRIANGLE_ARRAY_A_DESTRUCT, &[search_triangles]);
                    return 2;
                }
                e.call(LOCATION_ASSIGN, &[locals.goal_copy, reached_end]);
                e.call(LOCATION_DESTRUCT, &[reached_end]);
            }
            let node_count = call_u32(e, SOLUTION_CURRENT_NODE_COUNT, &[sol]);
            if fn_006cd8a0(
                e,
                this,
                locals.start_copy,
                locals.goal_copy,
                search_triangles,
                locals.path_triangles,
                locals.extra_triangles,
            ) {
                locals.destroy(e);
                break;
            }
            let tries_now = e.mem.i32(tries);
            if attempt == tries_now.wrapping_sub(1) && fn_006cd1d0(e, req) {
                incomplete = true;
                fn_006c9fa0(e, sol, true);
                let mut index = 0u32;
                while index < array_size(e, locals.path_triangles) {
                    let handle =
                        call_u32(e, TRIANGLE_HANDLE_ELEMENT, &[locals.path_triangles, index]);
                    let navmesh = e.mem.u32(handle);
                    let info = call_u32(e, NAVMESH_GET_INFO, &[navmesh]);
                    let handle =
                        call_u32(e, TRIANGLE_HANDLE_ELEMENT, &[locals.path_triangles, index]);
                    let center = call_u32(e, TRIANGLE_HANDLE_CENTER, &[handle, f.at(-0x180)]);
                    let local = f.at(-0x124);
                    e.call(
                        LOCATION_CONSTRUCT_FROM_POINT_AND_INFO,
                        &[local, center, info],
                    );
                    e.call(SOLUTION_ADD_CURRENT_NODE, &[sol, local, POINT_011F426C]);
                    e.call(LOCATION_DESTRUCT, &[local]);
                    index += 1;
                }
                locals.destroy(e);
                break;
            }
            let current = call_u32(e, SOLUTION_LAST_CURRENT_NODE, &[sol]);
            if current != 0 && call_u32(e, WORD_AT_1C, &[sol]) == last {
                let goal_node = virtual_node(e, sol, last);
                if reaches_goal_node(
                    e,
                    req,
                    goal_node,
                    PointSource::Node(current),
                    f.at(-0x198),
                    f.at(-0x18c),
                    f.at(-0x13c),
                ) {
                    locals.destroy(e);
                    break;
                }
            }
            if !call_bool(e, ARRAY_IS_EMPTY, &[search_triangles])
                && attempt != e.mem.i32(tries).wrapping_sub(1)
            {
                if attempt < e.mem.i32(tries).wrapping_sub(1) {
                    trim_current_nodes(e, sol, node_count);
                }
                locals.destroy(e);
                attempt += 1;
                continue;
            }
            if current != 0 {
                // The game also sets its incomplete flag here; both exits
                // below return a constant, so nothing reads it.
                fn_006c9fa0(e, sol, true);
                if call_bool(e, REQUEST_BYTE_9B, &[req]) {
                    locals.destroy(e);
                    e.call(TRIANGLE_ARRAY_A_DESTRUCT, &[search_triangles]);
                    return 1;
                }
                trim_current_nodes(e, sol, node_count);
            }
            locals.destroy(e);
            e.call(TRIANGLE_ARRAY_A_DESTRUCT, &[search_triangles]);
            return 2;
        }
        e.call(TRIANGLE_ARRAY_A_DESTRUCT, &[search_triangles]);
        i32::from(incomplete)
    })
}

// Translated from 006cd1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The request's byte at +0xA3.
pub fn fn_006cd1d0(e: &mut Engine, this: u32) -> bool {
    e.mem.u8(this + 0xa3) != 0
}

/// Whether the point of `location` is below the ground height (`TES`,
/// `0045cbc0`) of its cell plus the ground offset.
fn below_ground(e: &mut Engine, location: u32, point_out: u32, height_out: u32) -> bool {
    let cell = call_u32(e, LOCATION_GET_CELL, &[location]);
    let point = location_point(e, location, point_out);
    let tes = e.global::<u32>(TES_POINTER);
    let ground = e.call(TES_GROUND_HEIGHT, &[tes, point, cell]).f64()
        + e.global::<f64>(GROUND_OFFSET_0104F2F0);
    let point = location_point(e, location, height_out);
    f64::from(e.mem.f32(point + 8)) < ground
}

// Translated from 006cd1f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006cd1f0 - the C++ exception-unwinding frame (FS:[0]) that destroys the four side locations when a callee throws is not translated
/// The direct line of `fn_006cc5e0`: false when the line of sight from
/// `start` to `goal` (water filter, `bIsLargeCreature` = radius setting
/// `011d6ef0` below the actor radius) is blocked. When either end is below
/// the ground height, the start is appended as a current node; otherwise
/// both lines offset sideways by the actor radius (unit cross product with
/// the up axis) must be clear too.
pub fn fn_006cd1f0(e: &mut Engine, this: Ptr<PathBuilder>, start: u32, goal: u32) -> bool {
    with_frame(e, 0x1a8, |e, f| {
        let filter = f.at(-0x14);
        fn_006cd5a0(e, filter);
        let req = request(e, this);
        let radius = e.call(REQUEST_ACTOR_RADIUS, &[req]).f64();
        let setting = call_u32(e, SETTING_FLOAT_ADDRESS, &[SETTING_RADIUS_011D6EF0]);
        let large = f64::from(e.mem.f32(setting)) < radius;
        e.mem
            .set_u8(filter + FILTER_LARGE_CREATURE, u8::from(large));
        if !call_bool(e, CHECK_LINE_OF_SIGHT, &[start, goal, 0, filter]) {
            return false;
        }
        let sol = solution(e, this);
        if below_ground(e, start, f.at(-0xd8), f.at(-0xe4))
            || below_ground(e, goal, f.at(-0xf0), f.at(-0xfc))
        {
            e.call(SOLUTION_ADD_CURRENT_NODE, &[sol, start, POINT_011F426C]);
            return true;
        }
        let start_point = location_point(e, start, f.at(-0x114));
        let goal_point = location_point(e, goal, f.at(-0x108));
        call_u32(e, POINT3_SUBTRACT, &[goal_point, f.at(-0x7c), start_point]);
        let radius = e.call(REQUEST_ACTOR_RADIUS, &[req]).f32();
        let cross = call_u32(
            e,
            POINT3_UNIT_CROSS,
            &[f.at(-0x7c), f.at(-0x120), AXIS_011A9484],
        );
        call_u32(
            e,
            POINT3_TIMES_SCALAR,
            &[cross, f.at(-0x48), radius.to_bits()],
        );
        let side = f.at(-0x48);
        let sides = [
            (
                POINT3_SUBTRACT,
                f.at(-0xcc),
                f.at(-0x3c),
                [-0x12c, -0x138, -0x144, -0x150],
            ),
            (
                POINT3_ADD,
                f.at(-0x70),
                f.at(-0xa4),
                [-0x160, -0x16c, -0x178, -0x184],
            ),
        ];
        let mut built: Vec<u32> = Vec::new();
        for (operation, start_side, goal_side, slots) in sides {
            let point = location_point(e, start, f.at(slots[0]));
            let shifted = call_u32(e, operation, &[point, f.at(slots[1]), side]);
            e.call(
                LOCATION_CONSTRUCT_FROM_POINT_AND_LOCATION,
                &[start_side, shifted, start],
            );
            built.push(start_side);
            let point = location_point(e, goal, f.at(slots[2]));
            let shifted = call_u32(e, operation, &[point, f.at(slots[3]), side]);
            e.call(
                LOCATION_CONSTRUCT_FROM_POINT_AND_LOCATION,
                &[goal_side, shifted, goal],
            );
            built.push(goal_side);
            if !call_bool(e, CHECK_LINE_OF_SIGHT, &[start_side, goal_side, 0, 0]) {
                for location in built.iter().rev() {
                    e.call(LOCATION_DESTRUCT, &[*location]);
                }
                return false;
            }
        }
        e.call(SOLUTION_ADD_CURRENT_NODE, &[sol, start, POINT_011F426C]);
        for location in built.iter().rev() {
            e.call(LOCATION_DESTRUCT, &[*location]);
        }
        true
    })
}

// Translated from 006cd5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `CheckLineOfSightFilterWater` constructor (vtable `0106c93c`,
/// RTTI): the base constructor (`fn_006cd650`), then its own vtable;
/// returns `this`. `bIsLargeCreature` is left as it is.
pub fn fn_006cd5a0(e: &mut Engine, this: u32) -> u32 {
    fn_006cd650(e, this);
    e.mem.set_u32(this, CHECK_LINE_OF_SIGHT_FILTER_WATER_VTABLE);
    this
}

// Translated from 006cd5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CheckLineOfSightFilterWater::CanCrossEdge` (Xbox PDB): false when the
/// triangle `triangle` of `navmesh` passes `006a5ec0`, has flag 0x10 while
/// `bIsLargeCreature` is set, or has flag 0x80000000 or 0x1000 set. The
/// first three and the last argument words are not read.
#[allow(clippy::too_many_arguments)]
pub fn check_line_of_sight_filter_water_can_cross_edge(
    e: &mut Engine,
    this: Ptr<CheckLineOfSightFilterWater>,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    navmesh: u32,
    triangle: u16,
    _unused_6: u32,
) -> bool {
    let shape = call_u32(e, NAVMESH_GET_TRIANGLE, &[navmesh, u32::from(triangle)]);
    if call_bool(e, TRIANGLE_TEST_006A5EC0, &[shape]) {
        return false;
    }
    if call_bool(e, TRIANGLE_HAS_FLAG_0X10, &[shape])
        && e.get(this, CheckLineOfSightFilterWater::bIsLargeCreature)
    {
        return false;
    }
    if call_bool(e, TRIANGLE_FLAGS_SET, &[shape, 0x8000_0000]) {
        return false;
    }
    if call_bool(e, TRIANGLE_FLAGS_SET, &[shape, 0x1000]) {
        return false;
    }
    true
}

// Translated from 006cd650 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `Pathing::CheckLineOfSightFilter` constructor (vtable `0106c944`,
/// RTTI): stores the vtable; returns `this`.
pub fn fn_006cd650(e: &mut Engine, this: u32) -> u32 {
    e.mem.set_u32(this, CHECK_LINE_OF_SIGHT_FILTER_VTABLE);
    this
}

// Translated from 006cd670 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006cd670 - the C++ exception-unwinding frame (FS:[0]) that destroys the navmesh search and releases the holders when a callee throws is not translated
/// Runs the navmesh search (`006a6b70`) from the navmesh triangle of
/// `start` to that of `goal` (`PathingLocation::GetNavMeshAndTriangle`),
/// with `search_triangles` stored in the search (`fn_006cd880`) and
/// `path_triangles`, `extra_triangles` and `DetailedPathInfos` passed on.
/// False when either location has no navmesh. The last two words (the
/// caller's virtual node indices) are not read.
#[allow(clippy::too_many_arguments)]
pub fn fn_006cd670(
    e: &mut Engine,
    this: Ptr<PathBuilder>,
    start: u32,
    goal: u32,
    search_triangles: u32,
    path_triangles: u32,
    extra_triangles: u32,
    _unused_6: u32,
    _unused_7: u32,
) -> bool {
    with_frame(e, 0x20f8, |e, f| {
        let start_holder = f.at(-0x20f0);
        e.call(NAV_HOLDER_CONSTRUCT, &[start_holder]);
        if !call_bool(
            e,
            LOCATION_GET_NAVMESH_AND_TRIANGLE,
            &[start, start_holder, f.at(-0x10)],
        ) {
            e.call(NAV_HOLDER_RELEASE, &[start_holder]);
            return false;
        }
        let navmesh = call_u32(e, NI_POINTER_GET, &[start_holder]);
        e.mem.set_u32(f.at(-0x20d0), navmesh);
        let triangle = e.mem.u16(f.at(-0x10));
        e.mem.set_u16(f.at(-0x20cc), triangle);
        let goal_holder = f.at(-0x20ec);
        e.call(NAV_HOLDER_CONSTRUCT, &[goal_holder]);
        if !call_bool(
            e,
            LOCATION_GET_NAVMESH_AND_TRIANGLE,
            &[goal, goal_holder, f.at(-0x20d4)],
        ) {
            e.call(NAV_HOLDER_RELEASE, &[goal_holder]);
            e.call(NAV_HOLDER_RELEASE, &[start_holder]);
            return false;
        }
        let navmesh = call_u32(e, NI_POINTER_GET, &[goal_holder]);
        e.mem.set_u32(f.at(-0x20e8), navmesh);
        let triangle = e.mem.u16(f.at(-0x20d4));
        e.mem.set_u16(f.at(-0x20e4), triangle);
        let search = f.at(-0x20bc);
        e.call(NAVMESH_SEARCH_CONSTRUCT, &[search]);
        fn_006cd880(e, search, search_triangles);
        let req = request(e, this);
        let found = call_bool(
            e,
            NAVMESH_SEARCH_RUN,
            &[
                search,
                req,
                f.at(-0x20d0),
                f.at(-0x20e8),
                path_triangles,
                extra_triangles,
                this.addr() + 0x14,
            ],
        );
        e.call(NAVMESH_SEARCH_DESTRUCT, &[search]);
        e.call(NAV_HOLDER_RELEASE, &[goal_holder]);
        e.call(NAV_HOLDER_RELEASE, &[start_holder]);
        found
    })
}

// Translated from 006cd880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the word at +0x2084 of a navmesh search.
pub fn fn_006cd880(e: &mut Engine, this: u32, value: u32) {
    e.mem.set_u32(this + SEARCH_WORD_2084, value);
}

// Translated from 006cd8a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006cd8a0 - the C++ exception-unwinding frame (FS:[0]) that destroys the search or smoother and the info array when a callee throws is not translated
/// Turns the triangles of the navmesh search into current nodes: with the
/// POV-search setting (`011d7444`) on and an actor radius above the large
/// radius, a POV search (`0069f010`) over the triangles' navmesh info
/// handles; else with the smoother setting (`011d7434`) on, the path
/// smoother (`006ed530`); else the POV smoother (`006ad770`). Returns what
/// it returned.
pub fn fn_006cd8a0(
    e: &mut Engine,
    this: Ptr<PathBuilder>,
    start: u32,
    goal: u32,
    search_triangles: u32,
    path_triangles: u32,
    extra_triangles: u32,
) -> bool {
    with_frame(e, 0x4330, |e, f| {
        let req = request(e, this);
        if setting_bool(e, SETTING_POV_SEARCH_011D7444)
            && e.call(REQUEST_ACTOR_RADIUS, &[req]).f64() > e.global::<f64>(LARGE_RADIUS_0106C948)
        {
            let handles = f.at(-0x2118);
            let heap = scrap_heap(e, this);
            let count = array_size(e, path_triangles);
            e.call(INFO_HANDLE_ARRAY_CONSTRUCT, &[handles, count, 0, heap]);
            let mut index = 0u32;
            while index < array_size(e, path_triangles) {
                let handle = call_u32(e, TRIANGLE_HANDLE_ELEMENT, &[path_triangles, index]);
                let triangle = u32::from(e.mem.u16(handle + 4));
                let handle = call_u32(e, TRIANGLE_HANDLE_ELEMENT, &[path_triangles, index]);
                let navmesh = e.mem.u32(handle);
                let made = call_u32(
                    e,
                    FAST_TRIANGLE_HANDLE_CONSTRUCT,
                    &[f.at(-0x42ec), navmesh, triangle],
                );
                e.call(INFO_HANDLE_ARRAY_ADD, &[handles, made]);
                index += 1;
            }
            let search = f.at(-0x2104);
            e.call(POV_SEARCH_CONSTRUCT, &[search]);
            let sol = solution(e, this);
            let found = call_bool(e, POV_SEARCH_RUN, &[search, start, goal, handles, req, sol]);
            e.call(POV_SEARCH_DESTRUCT, &[search]);
            e.call(INFO_HANDLE_ARRAY_DESTRUCT, &[handles]);
            return found;
        }
        if setting_bool(e, SETTING_SMOOTHER_011D7434) {
            let smoother = f.at(-0x21d4);
            e.call(PATH_SMOOTHER_CONSTRUCT, &[smoother]);
            let goal_point = location_point(e, goal, f.at(-0x42fc));
            let start_point = location_point(e, start, f.at(-0x4308));
            let sol = solution(e, this);
            let smoothed = call_bool(
                e,
                PATH_SMOOTHER_RUN,
                &[
                    smoother,
                    req,
                    path_triangles,
                    extra_triangles,
                    start_point,
                    goal_point,
                    sol,
                    search_triangles,
                    1,
                ],
            );
            e.call(PATH_SMOOTHER_DESTRUCT, &[smoother]);
            return smoothed;
        }
        let smoother = f.at(-0x42e4);
        e.call(POV_SMOOTHER_CONSTRUCT, &[smoother]);
        let goal_point = location_point(e, goal, f.at(-0x4318));
        let start_point = location_point(e, start, f.at(-0x4324));
        let sol = solution(e, this);
        let smoothed = call_bool(
            e,
            POV_SMOOTHER_RUN,
            &[
                smoother,
                req,
                path_triangles,
                extra_triangles,
                start_point,
                goal_point,
                sol,
                search_triangles,
                this.addr() + 0x14,
            ],
        );
        e.call(POV_SMOOTHER_DESTRUCT, &[smoother]);
        smoothed
    })
}

// Translated from 006cdb40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006cdb40 - the C++ exception-unwinding frame (FS:[0]) that destroys the by-value location and the info list when a callee throws is not translated
/// Finds the closest point on a navmesh triangle for `location` (a
/// `PathingLocation` passed by value; `location` is the address of the
/// copy, which this function destroys): for every navmesh info of the
/// location, the edge the navmesh finds (`00697010`, target radius,
/// `flag`), its triangle centre, the edge point (`006ba100`, pulled
/// towards the centre), the one closest to the location wins. Stores it in
/// `out` and returns true, or false when none was found.
pub fn fn_006cdb40(
    e: &mut Engine,
    this: Ptr<PathBuilder>,
    location: u32,
    out: u32,
    flag: bool,
) -> bool {
    with_frame(e, 0xf8, |e, f| {
        let best_distance = f.at(-0x18);
        let start = float_constant_bits(e, START_DISTANCE_01016970);
        e.mem.set_u32(best_distance, start);
        let mut best = -1i32;
        let mut best_triangle = 0xffffu16;
        copy_point(e, f.at(-0x38), POINT_011F426C);
        let list = f.at(-0x2c);
        e.call(INFO_LIST_CONSTRUCT, &[list]);
        e.call(LOCATION_GET_NAVMESH_INFOS, &[location, list]);
        let count = array_size(e, list);
        let req = request(e, this);
        let mut index = 0u32;
        while index < count {
            let holder = f.at(-0x40);
            e.call(NAV_HOLDER_CONSTRUCT, &[holder]);
            let info = word_element(e, list, index);
            if call_bool(e, NAVMESH_INFO_LOAD_HOLDER, &[info, holder]) {
                let radius = e.call(REQUEST_TARGET_RADIUS, &[req]).f32();
                let point = location_point(e, location, f.at(-0x78));
                let navmesh = call_u32(e, NI_POINTER_GET, &[holder]);
                if call_bool(
                    e,
                    NAVMESH_FIND_EDGE,
                    &[
                        navmesh,
                        point,
                        radius.to_bits(),
                        f.at(-0x44),
                        f.at(-0x48),
                        u32::from(flag),
                    ],
                ) {
                    let triangle = u32::from(e.mem.u16(f.at(-0x44)));
                    let edge = e.mem.u32(f.at(-0x48));
                    let navmesh = call_u32(e, NI_POINTER_GET, &[holder]);
                    e.call(
                        NAVMESH_GET_EDGE_VERTICES,
                        &[navmesh, f.at(-0x5c), triangle, edge],
                    );
                    let navmesh = call_u32(e, NI_POINTER_GET, &[holder]);
                    e.call(NAVMESH_GET_CENTER, &[navmesh, f.at(-0x54), triangle]);
                    copy_point(e, f.at(-0x68), f.at(-0x54));
                    let navmesh = call_u32(e, NI_POINTER_GET, &[holder]);
                    let center = call_u32(e, NAVMESH_GET_CENTER, &[navmesh, f.at(-0x84), triangle]);
                    let point = location_point(e, location, f.at(-0x90));
                    let (first_vertex, second_vertex) =
                        (e.mem.u32(f.at(-0x5c)), e.mem.u32(f.at(-0x58)));
                    if call_bool(
                        e,
                        EDGE_PULL_TEST_006BA100,
                        &[point, center, first_vertex, second_vertex, f.at(-0x68)],
                    ) {
                        let weight = float_constant_bits(e, CENTER_WEIGHT_0101E2BC);
                        let toward =
                            call_u32(e, POINT3_SUBTRACT, &[f.at(-0x54), f.at(-0x9c), f.at(-0x68)]);
                        let scaled =
                            call_u32(e, POINT3_TIMES_SCALAR, &[toward, f.at(-0xa8), weight]);
                        e.call(POINT3_ADD_ASSIGN, &[f.at(-0x68), scaled]);
                    }
                    let point = location_point(e, location, f.at(-0xb4));
                    let offset = call_u32(e, POINT3_SUBTRACT, &[f.at(-0x68), f.at(-0xc0), point]);
                    let distance = e.call(POINT3_LENGTH_SQUARED, &[offset]).f32();
                    e.mem.set_f32(f.at(-0x6c), distance);
                    if e.mem.f32(best_distance) > e.mem.f32(f.at(-0x6c)) {
                        let bits = x87_quiet_bits(e.mem.u32(f.at(-0x6c)));
                        e.mem.set_u32(best_distance, bits);
                        best = index as i32;
                        best_triangle = e.mem.u16(f.at(-0x44));
                        copy_point(e, f.at(-0x38), f.at(-0x68));
                    }
                }
            }
            e.call(NAV_HOLDER_RELEASE, &[holder]);
            index += 1;
        }
        if best == -1 {
            e.call(INFO_LIST_DESTRUCT, &[list]);
            e.call(LOCATION_DESTRUCT, &[location]);
            return false;
        }
        let info = word_element(e, list, best as u32);
        let local = f.at(-0xec);
        let made = call_u32(
            e,
            LOCATION_CONSTRUCT_FROM_MESH_POINT,
            &[local, f.at(-0x38), info, u32::from(best_triangle)],
        );
        e.call(LOCATION_ASSIGN, &[out, made]);
        e.call(LOCATION_DESTRUCT, &[local]);
        e.call(INFO_LIST_DESTRUCT, &[list]);
        e.call(LOCATION_DESTRUCT, &[location]);
        true
    })
}

/// The uniform entry of `fn_006cdb40`: the location comes as ten words (the
/// `PathingLocation` passed by value), copied to a stack block whose
/// address the typed function takes.
fn fn_006cdb40_entry(e: &mut Engine, words: &[u32]) -> Ret {
    let this = Ptr::<PathBuilder>::new(words[0]);
    let out = words[11];
    let flag = words[12] as u8 != 0;
    let found = e.with_stack(PATHING_LOCATION_SIZE, |e, copy| {
        for (i, word) in words[1..11].iter().enumerate() {
            e.mem.set_u32(copy.addr() + 4 * i as u32, *word);
        }
        fn_006cdb40(e, this, copy.addr(), out, flag)
    });
    RetVal::into_ret(found)
}

/// The edge point of `fn_006cde50` and `fn_006ce2d0`: both edge vertices
/// times the edge weight plus the triangle centre times the centre weight,
/// written to `result`. Returns `result`.
#[allow(clippy::too_many_arguments)]
fn edge_point(
    e: &mut Engine,
    navmesh_holder: u32,
    triangle: u32,
    edge: u32,
    f: Frame,
    vertices: [i32; 2],
    center: i32,
    temporaries: [i32; 3],
    result: u32,
) -> u32 {
    e.call(POINT3_DEFAULT_CONSTRUCT, &[f.at(vertices[0])]);
    e.call(POINT3_DEFAULT_CONSTRUCT, &[f.at(vertices[1])]);
    let navmesh = call_u32(e, NI_POINTER_GET, &[navmesh_holder]);
    e.call(
        NAVMESH_EDGE_POINTS,
        &[
            navmesh,
            triangle,
            edge,
            f.at(vertices[0]),
            f.at(vertices[1]),
        ],
    );
    let navmesh = call_u32(e, NI_POINTER_GET, &[navmesh_holder]);
    e.call(NAVMESH_GET_CENTER, &[navmesh, f.at(center), triangle]);
    let center_weight = float_constant_bits(e, CENTER_WEIGHT_0101E2BC);
    let scaled_center = call_u32(
        e,
        POINT3_TIMES_SCALAR,
        &[f.at(center), f.at(temporaries[0]), center_weight],
    );
    let edge_weight = float_constant_bits(e, EDGE_WEIGHT_0104E2B8);
    let sum = call_u32(
        e,
        POINT3_ADD,
        &[f.at(vertices[0]), f.at(temporaries[1]), f.at(vertices[1])],
    );
    let scaled_sum = call_u32(
        e,
        POINT3_TIMES_SCALAR,
        &[sum, f.at(temporaries[2]), edge_weight],
    );
    call_u32(e, POINT3_ADD, &[scaled_sum, result, scaled_center])
}

/// Finds the edge of the navmesh in `holder` within twice the distance from
/// the point of the node `reference` to the info's point (`info + 0x10`),
/// as `fn_006cde50` and `fn_006ce2d0` start (the node's location is fetched
/// for each use, as the code does).
#[allow(clippy::too_many_arguments)]
fn find_near_edge(
    e: &mut Engine,
    holder: u32,
    info: u32,
    reference: u32,
    f: Frame,
    slots: [i32; 4],
    flag: u32,
) -> bool {
    let location = node_location(e, reference);
    let point = location_point(e, location, f.at(slots[0]));
    let offset = call_u32(e, POINT3_SUBTRACT, &[point, f.at(slots[1]), info + 0x10]);
    let distance = e.call(POINT3_LENGTH, &[offset]).f32();
    e.mem.set_f32(f.at(-0x1c), distance);
    e.mem.set_u16(f.at(-0x14), 0xffff);
    e.mem.set_i32(f.at(-0x18), -1);
    let doubled = e.mem.f32(f.at(-0x1c)) + e.mem.f32(f.at(-0x1c));
    e.mem.set_f32(f.at(slots[3]), doubled);
    let doubled_bits = x87_quiet_bits(e.mem.u32(f.at(slots[3])));
    let location = node_location(e, reference);
    let point = location_point(e, location, f.at(slots[2]));
    let navmesh = call_u32(e, NI_POINTER_GET, &[holder]);
    call_bool(
        e,
        NAVMESH_FIND_EDGE,
        &[navmesh, point, doubled_bits, f.at(-0x14), f.at(-0x18), flag],
    )
}

/// The portal-triangle search of `fn_006cde50` and `fn_006ce2d0`: among the
/// portal triangles of the navmesh in `holder` towards `info` (skipping
/// flag-0x10 triangles for large actors), the one whose centre is closer
/// to `reference` (`REQUEST_ORIGIN` or `REQUEST_DESTINATION`) than the
/// starting distance. As in the game, the starting distance is never
/// lowered (the code stores it into the candidate instead), so the last
/// such triangle wins. Returns the chosen element word, or 0xFFFF.
#[allow(clippy::too_many_arguments)]
fn closest_portal_triangle(
    e: &mut Engine,
    request: u32,
    holder: u32,
    portals: u32,
    reference: u32,
    f: Frame,
    slots: [i32; 4],
) -> u32 {
    let mut best = 0xffffu32;
    let start = float_constant_bits(e, START_DISTANCE_01016970);
    e.mem.set_u32(f.at(slots[3]), start);
    let mut index = 0u32;
    while index < array_size(e, portals) {
        let slot = call_u32(e, WORD_ELEMENT, &[portals, index]);
        let triangle = u32::from(e.mem.u16(slot));
        let navmesh = call_u32(e, NI_POINTER_GET, &[holder]);
        let shape = call_u32(e, NAVMESH_GET_TRIANGLE, &[navmesh, triangle]);
        if e.call(REQUEST_ACTOR_RADIUS, &[request]).f64() > e.global::<f64>(LARGE_RADIUS_0106C948)
            && call_bool(e, TRIANGLE_HAS_FLAG_0X10, &[shape])
        {
            index += 1;
            continue;
        }
        let slot = call_u32(e, WORD_ELEMENT, &[portals, index]);
        let triangle = u32::from(e.mem.u16(slot));
        let navmesh = call_u32(e, NI_POINTER_GET, &[holder]);
        e.call(NAVMESH_GET_CENTER, &[navmesh, f.at(slots[0]), triangle]);
        let location = call_u32(e, reference, &[request]);
        let point = location_point(e, location, f.at(slots[1]));
        let offset = call_u32(e, POINT3_SUBTRACT, &[f.at(slots[0]), f.at(slots[2]), point]);
        let distance = e.call(POINT3_LENGTH, &[offset]).f32();
        if e.mem.f32(f.at(slots[3])) > distance {
            best = word_element(e, portals, index);
        }
        index += 1;
    }
    best
}

// Translated from 006cde50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006cde50 - the C++ exception-unwinding frame (FS:[0]) that destroys the portal array and releases the navmesh holder when a callee throws is not translated
/// Finds the location where the path enters the navmesh of virtual node
/// `node` coming from `previous` and stores it in `out`: for a node with
/// bit 1, the edge point (`edge_point`) of the edge near `previous`
/// (`00697010` with flag 1); otherwise the centre of the portal triangle
/// towards `previous`'s navmesh info that `closest_portal_triangle` picks
/// (measured from the request's origin). False when the node's navmesh
/// does not load or nothing is found.
pub fn fn_006cde50(
    e: &mut Engine,
    this: Ptr<PathBuilder>,
    previous: u32,
    node: u32,
    out: u32,
) -> bool {
    with_frame(e, 0x160, |e, f| {
        let location = node_location(e, node);
        e.call(LOCATION_RESOLVE, &[location]);
        let holder = f.at(-0x10);
        e.call(NAV_HOLDER_CONSTRUCT, &[holder]);
        let location = node_location(e, node);
        let info = call_u32(e, LOCATION_GET_BEST_NAVMESH_INFO, &[location]);
        if !call_bool(e, NAVMESH_INFO_LOAD_HOLDER, &[info, holder]) {
            e.call(NAV_HOLDER_RELEASE, &[holder]);
            return false;
        }
        if fn_006cac70(e, node) {
            let location = node_location(e, node);
            let info = call_u32(e, LOCATION_GET_BEST_NAVMESH_INFO, &[location]);
            let found = find_near_edge(
                e,
                holder,
                info,
                previous,
                f,
                [-0x84, -0x90, -0x9c, -0x150],
                1,
            );
            if !found {
                e.call(NAV_HOLDER_RELEASE, &[holder]);
                return false;
            }
            let triangle = u32::from(e.mem.u16(f.at(-0x14)));
            let edge = e.mem.u32(f.at(-0x18));
            e.call(POINT3_DEFAULT_CONSTRUCT, &[f.at(-0x40)]);
            e.call(POINT3_DEFAULT_CONSTRUCT, &[f.at(-0x34)]);
            let navmesh = call_u32(e, NI_POINTER_GET, &[holder]);
            e.call(
                NAVMESH_EDGE_POINTS,
                &[navmesh, triangle, edge, f.at(-0x40), f.at(-0x34)],
            );
            let navmesh = call_u32(e, NI_POINTER_GET, &[holder]);
            e.call(NAVMESH_GET_CENTER, &[navmesh, f.at(-0x28), triangle]);
            let navmesh = call_u32(e, NI_POINTER_GET, &[holder]);
            let info = call_u32(e, NAVMESH_GET_INFO, &[navmesh]);
            let center_weight = float_constant_bits(e, CENTER_WEIGHT_0101E2BC);
            let scaled_center = call_u32(
                e,
                POINT3_TIMES_SCALAR,
                &[f.at(-0x28), f.at(-0xe8), center_weight],
            );
            let edge_weight = float_constant_bits(e, EDGE_WEIGHT_0104E2B8);
            let sum = call_u32(e, POINT3_ADD, &[f.at(-0x40), f.at(-0xd0), f.at(-0x34)]);
            let scaled_sum = call_u32(e, POINT3_TIMES_SCALAR, &[sum, f.at(-0xdc), edge_weight]);
            let point = call_u32(e, POINT3_ADD, &[scaled_sum, f.at(-0xf4), scaled_center]);
            let local = f.at(-0xc4);
            let made = call_u32(
                e,
                LOCATION_CONSTRUCT_FROM_MESH_POINT,
                &[local, point, info, triangle],
            );
            e.call(LOCATION_ASSIGN, &[out, made]);
            e.call(LOCATION_DESTRUCT, &[local]);
            e.call(NAV_HOLDER_RELEASE, &[holder]);
            return true;
        }
        let portals = f.at(-0x58);
        let heap = scrap_heap(e, this);
        e.call(PORTAL_ARRAY_CONSTRUCT, &[portals, 0, 0, heap]);
        let previous_location = node_location(e, previous);
        let previous_info = call_u32(e, LOCATION_GET_BEST_NAVMESH_INFO, &[previous_location]);
        let navmesh = call_u32(e, NI_POINTER_GET, &[holder]);
        e.call(
            NAVMESH_FIND_PORTAL_TRIANGLES,
            &[navmesh, previous_info, portals],
        );
        let req = request(e, this);
        let best = closest_portal_triangle(
            e,
            req,
            holder,
            portals,
            REQUEST_ORIGIN,
            f,
            [-0x74, -0x104, -0x110, -0x44],
        );
        if best == 0xffff {
            e.call(PORTAL_ARRAY_DESTRUCT, &[portals]);
            e.call(NAV_HOLDER_RELEASE, &[holder]);
            return false;
        }
        let triangle = best & 0xffff;
        let location = node_location(e, node);
        let info = call_u32(e, LOCATION_GET_BEST_NAVMESH_INFO, &[location]);
        let navmesh = call_u32(e, NI_POINTER_GET, &[holder]);
        let center = call_u32(e, NAVMESH_GET_CENTER, &[navmesh, f.at(-0x144), triangle]);
        let local = f.at(-0x138);
        let made = call_u32(
            e,
            LOCATION_CONSTRUCT_FROM_MESH_POINT,
            &[local, center, info, triangle],
        );
        e.call(LOCATION_ASSIGN, &[out, made]);
        e.call(LOCATION_DESTRUCT, &[local]);
        e.call(PORTAL_ARRAY_DESTRUCT, &[portals]);
        e.call(NAV_HOLDER_RELEASE, &[holder]);
        true
    })
}

/// Lifts the height of `point` to the ground height (`0045cbc0` on the
/// navmesh's cell, `0059bb30`) when it is below ground plus the offset.
fn lift_to_ground(e: &mut Engine, holder: u32, point: u32, height_slot: u32) {
    let navmesh = call_u32(e, NI_POINTER_GET, &[holder]);
    let cell = call_u32(e, NAVMESH_CELL, &[navmesh]);
    let tes = e.global::<u32>(TES_POINTER);
    let ground = e.call(TES_GROUND_HEIGHT, &[tes, point, cell]).f32();
    e.mem.set_f32(height_slot, ground);
    let limit = f64::from(e.mem.f32(height_slot)) + e.global::<f64>(GROUND_OFFSET_0104F2F0);
    if f64::from(e.mem.f32(point + 8)) < limit {
        let bits = x87_quiet_bits(e.mem.u32(height_slot));
        e.mem.set_u32(point + 8, bits);
    }
}

// Translated from 006ce2d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006ce2d0 - the C++ exception-unwinding frame (FS:[0]) that destroys the portal array and releases the navmesh holder when a callee throws is not translated
/// Finds the location where the path leaves the navmesh of virtual node
/// `node` towards `next` and stores it in `out`: for a node with bit 1,
/// the edge point of the edge near `next` (`00697010` with flag 0);
/// otherwise the centre of the portal triangle towards `next`'s navmesh
/// info picked by `closest_portal_triangle` (measured from the request's
/// destination). Either point is lifted to the ground height when below
/// it. False when the node's navmesh does not load or nothing is found.
pub fn fn_006ce2d0(e: &mut Engine, this: Ptr<PathBuilder>, node: u32, next: u32, out: u32) -> bool {
    with_frame(e, 0x168, |e, f| {
        let location = node_location(e, node);
        e.call(LOCATION_RESOLVE, &[location]);
        let holder = f.at(-0x10);
        e.call(NAV_HOLDER_CONSTRUCT, &[holder]);
        let location = node_location(e, node);
        let info = call_u32(e, LOCATION_GET_BEST_NAVMESH_INFO, &[location]);
        if !call_bool(e, NAVMESH_INFO_LOAD_HOLDER, &[info, holder]) {
            e.call(NAV_HOLDER_RELEASE, &[holder]);
            return false;
        }
        if fn_006cac70(e, node) {
            let location = node_location(e, node);
            let info = call_u32(e, LOCATION_GET_BEST_NAVMESH_INFO, &[location]);
            let found = find_near_edge(e, holder, info, next, f, [-0xa4, -0xb0, -0xbc, -0x158], 0);
            if !found {
                e.call(NAV_HOLDER_RELEASE, &[holder]);
                return false;
            }
            let triangle = u32::from(e.mem.u16(f.at(-0x14)));
            let edge = e.mem.u32(f.at(-0x18));
            let point = edge_point(
                e,
                holder,
                triangle,
                edge,
                f,
                [-0x4c, -0x34],
                -0x28,
                [-0xe0, -0xc8, -0xd4],
                f.at(-0x40),
            );
            lift_to_ground(e, holder, point, f.at(-0x50));
            let triangle = u32::from(e.mem.u16(f.at(-0x14)));
            let navmesh = call_u32(e, NI_POINTER_GET, &[holder]);
            let info = call_u32(e, NAVMESH_GET_INFO, &[navmesh]);
            let local = f.at(-0x108);
            let made = call_u32(
                e,
                LOCATION_CONSTRUCT_FROM_MESH_POINT,
                &[local, f.at(-0x40), info, triangle],
            );
            e.call(LOCATION_ASSIGN, &[out, made]);
            e.call(LOCATION_DESTRUCT, &[local]);
            e.call(NAV_HOLDER_RELEASE, &[holder]);
            return true;
        }
        let portals = f.at(-0x68);
        let heap = scrap_heap(e, this);
        e.call(PORTAL_ARRAY_CONSTRUCT, &[portals, 0, 0, heap]);
        let next_location = node_location(e, next);
        let next_info = call_u32(e, LOCATION_GET_BEST_NAVMESH_INFO, &[next_location]);
        let navmesh = call_u32(e, NI_POINTER_GET, &[holder]);
        e.call(
            NAVMESH_FIND_PORTAL_TRIANGLES,
            &[navmesh, next_info, portals],
        );
        let req = request(e, this);
        let best = closest_portal_triangle(
            e,
            req,
            holder,
            portals,
            REQUEST_DESTINATION,
            f,
            [-0x84, -0x118, -0x124, -0x54],
        );
        if best == 0xffff {
            e.call(PORTAL_ARRAY_DESTRUCT, &[portals]);
            e.call(NAV_HOLDER_RELEASE, &[holder]);
            return false;
        }
        let triangle = best & 0xffff;
        let navmesh = call_u32(e, NI_POINTER_GET, &[holder]);
        e.call(NAVMESH_GET_CENTER, &[navmesh, f.at(-0x90), triangle]);
        lift_to_ground(e, holder, f.at(-0x90), f.at(-0x94));
        let location = node_location(e, node);
        let info = call_u32(e, LOCATION_GET_BEST_NAVMESH_INFO, &[location]);
        let local = f.at(-0x14c);
        let made = call_u32(
            e,
            LOCATION_CONSTRUCT_FROM_MESH_POINT,
            &[local, f.at(-0x90), info, triangle],
        );
        e.call(LOCATION_ASSIGN, &[out, made]);
        e.call(LOCATION_DESTRUCT, &[local]);
        e.call(PORTAL_ARRAY_DESTRUCT, &[portals]);
        e.call(NAV_HOLDER_RELEASE, &[holder]);
        true
    })
}

// Translated from 006ce7e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006ce7e0 - the C++ exception-unwinding frame (FS:[0]) around the placement construction is not translated
/// `PathingSolution::AddVirtualPathingNode` (Xbox PDB): appends a virtual
/// node for `location` and `door` (`VirtualPathingNode::VirtualPathingNode`)
/// and, with a door, the door's form id to `DoorIDs` (+0x30). Returns the
/// node's index.
pub fn pathing_solution_add_virtual_pathing_node(
    e: &mut Engine,
    this: u32,
    location: u32,
    door: u32,
) -> u32 {
    let index = add_virtual_node(
        e,
        this,
        VIRTUAL_NODE_CONSTRUCT_LOCATION_DOOR,
        &[location, door],
    );
    if door != 0 {
        let id = call_u32(e, FORM_GET_ID, &[door]);
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), id);
            e.call(ARRAY_PUSH_WORD, &[this + DOOR_IDS, slot.addr()]);
        });
    }
    index
}

// Translated from 006ce8b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006ce8b0 - the C++ exception-unwinding frame (FS:[0]) that destroys the search, the scrap arrays and the two locations when a callee throws is not translated
/// `PathBuilder::BuildNavMeshInfoPathInSpace` (Xbox PDB): locations for
/// `start_point` and `goal_point` in the space `space` (a
/// `ParentSpaceNode`: its interior cell +8 and worldspace +4), resolved;
/// the navmesh info search between them. Appends the start, every info in
/// between and the goal (as a door node when `door` is set); when the
/// search failed but found infos and the request's byte +0x9B is set, the
/// solution is marked incomplete and the path ends at the last info's
/// point instead. Returns whether the search succeeded (false when a
/// location does not resolve).
pub fn path_builder_build_nav_mesh_info_path_in_space(
    e: &mut Engine,
    this: Ptr<PathBuilder>,
    start_point: u32,
    goal_point: u32,
    space: u32,
    door: u32,
) -> bool {
    with_frame(e, 0x2160, |e, f| {
        let start = f.at(-0x2130);
        let goal = f.at(-0x20f4);
        let (worldspace, cell) = (e.mem.u32(space + 4), e.mem.u32(space + 8));
        e.call(
            LOCATION_CONSTRUCT_FROM_PARTS,
            &[start, start_point, cell, worldspace],
        );
        let (worldspace, cell) = (e.mem.u32(space + 4), e.mem.u32(space + 8));
        e.call(
            LOCATION_CONSTRUCT_FROM_PARTS,
            &[goal, goal_point, cell, worldspace],
        );
        if !call_bool(e, LOCATION_RESOLVE, &[start]) || !call_bool(e, LOCATION_RESOLVE, &[goal]) {
            e.call(LOCATION_DESTRUCT, &[goal]);
            e.call(LOCATION_DESTRUCT, &[start]);
            return false;
        }
        let search = f.at(-0x20cc);
        let infos = f.at(-0x2108);
        let doors = f.at(-0x28);
        e.call(INFO_SEARCH_CONSTRUCT, &[search]);
        let heap = scrap_heap(e, this);
        e.call(INFO_SCRAP_ARRAY_CONSTRUCT, &[infos, 0x14, 0, heap]);
        let heap = scrap_heap(e, this);
        e.call(DOOR_SCRAP_ARRAY_CONSTRUCT, &[doors, 0x14, 0, heap]);
        let goal_info = call_u32(e, LOCATION_GET_BEST_NAVMESH_INFO, &[goal]);
        let goal_at = location_point(e, goal, f.at(-0x2144));
        let start_info = call_u32(e, LOCATION_GET_BEST_NAVMESH_INFO, &[start]);
        let start_at = location_point(e, start, f.at(-0x2150));
        let found = call_bool(
            e,
            INFO_SEARCH_RUN,
            &[
                search, 0, start_at, start_info, goal_at, goal_info, infos, doors, 0,
            ],
        );
        let finish = |e: &mut Engine, result: bool| {
            e.call(DOOR_SCRAP_ARRAY_DESTRUCT, &[doors]);
            e.call(INFO_SCRAP_ARRAY_DESTRUCT, &[infos]);
            e.call(INFO_SEARCH_DESTRUCT, &[search]);
            e.call(LOCATION_DESTRUCT, &[goal]);
            e.call(LOCATION_DESTRUCT, &[start]);
            result
        };
        let sol = solution(e, this);
        if !found {
            if call_bool(e, ARRAY_IS_EMPTY, &[infos]) {
                return finish(e, false);
            }
            let req = request(e, this);
            if !call_bool(e, REQUEST_BYTE_9B, &[req]) {
                return finish(e, false);
            }
            fn_006c9fa0(e, sol, true);
        }
        e.call(SOLUTION_ADD_LOCATION_NODE, &[sol, start]);
        let mut index = 1u32;
        while index < array_size(e, infos).wrapping_sub(1) {
            let info = word_element(e, infos, index);
            fn_006c9920(e, sol, info);
            index += 1;
        }
        if found {
            if door != 0 {
                pathing_solution_add_virtual_pathing_node(e, sol, goal, door);
            } else {
                e.call(SOLUTION_ADD_LOCATION_NODE, &[sol, goal]);
            }
        } else {
            let last = call_u32(e, LAST_WORD_ELEMENT, &[infos]);
            let info = e.mem.u32(last);
            let last = call_u32(e, LAST_WORD_ELEMENT, &[infos]);
            let point = e.mem.u32(last) + 0x10;
            fn_006c99c0(e, sol, point, info);
        }
        finish(e, found)
    })
}

// Translated from 006cec40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006cec40 - the C++ exception-unwinding frame (FS:[0]) that destroys the location copy when a callee throws is not translated
/// Appends a current node for every virtual node from `first` to `last`
/// (a segment without navmeshes): the node's location, moved to its best
/// navmesh info's point (+0x10) unless the node has bit 1. A door node at
/// `last` passes its door to the last current node. Returns true.
pub fn fn_006cec40(e: &mut Engine, this: Ptr<PathBuilder>, first: u32, last: u32) -> bool {
    with_frame(e, 0x40, |e, f| {
        let sol = solution(e, this);
        let copy = f.at(-0x38);
        let mut index = first;
        while index <= last {
            let node = virtual_node(e, sol, index);
            let location = node_location(e, node);
            e.call(LOCATION_COPY_CONSTRUCT, &[copy, location]);
            let node = virtual_node(e, sol, index);
            if !fn_006cac70(e, node) {
                let info = call_u32(e, LOCATION_GET_BEST_NAVMESH_INFO, &[copy]);
                e.call(LOCATION_SET_POSITION, &[copy, info + 0x10]);
            }
            e.call(SOLUTION_ADD_CURRENT_NODE, &[sol, copy, POINT_011F426C]);
            e.call(LOCATION_DESTRUCT, &[copy]);
            index = index.wrapping_add(1);
        }
        let node = virtual_node(e, sol, last);
        if fn_006ca0c0(e, node) {
            let node = virtual_node(e, sol, last);
            let door = call_u32(e, VIRTUAL_NODE_DOOR, &[node]);
            let current = call_u32(e, SOLUTION_LAST_CURRENT_NODE, &[sol]);
            pathing_node_set_teleport_door(e, current, door);
        }
        true
    })
}

// Translated from 006ced60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns true; the four argument words are not read (thiscall, `RET
/// 0x10`).
pub fn fn_006ced60(
    _e: &mut Engine,
    _this: u32,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
) -> bool {
    true
}

// Translated from 006ced70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The first element of a `BSSimpleArray<ParentSpaceNode>` (`006a1440`).
pub fn fn_006ced70(e: &mut Engine, this: u32) -> u32 {
    call_u32(e, SPACE_ELEMENT, &[this, 0])
}

// Translated from 006ced90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The last element of a `BSSimpleArray<ParentSpaceNode>` (`006a1440`).
pub fn fn_006ced90(e: &mut Engine, this: u32) -> u32 {
    let last = e
        .get(Ptr::<BSSimpleArray>::new(this), BSSimpleArray::iSize)
        .wrapping_sub(1);
    call_u32(e, SPACE_ELEMENT, &[this, last])
}

// Translated from 006cedb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The last element of a `BSSimpleArray<TeleportLink>` (`00558dd0`).
pub fn fn_006cedb0(e: &mut Engine, this: u32) -> u32 {
    let last = e
        .get(Ptr::<BSSimpleArray>::new(this), BSSimpleArray::iSize)
        .wrapping_sub(1);
    call_u32(e, LINK_ELEMENT, &[this, last])
}

// Translated from 006cedd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The last element of a `BSSimpleArray<PathingLocation>` (`00691750`).
pub fn fn_006cedd0(e: &mut Engine, this: u32) -> u32 {
    let last = e
        .get(Ptr::<BSSimpleArray>::new(this), BSSimpleArray::iSize)
        .wrapping_sub(1);
    call_u32(e, LOCATION_ELEMENT, &[this, last])
}

// Translated from 006cedf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The first element of a `BSSimpleArray<NavMeshEdgeLocation>`
/// (`fn_006cee10`).
pub fn fn_006cedf0(e: &mut Engine, this: u32) -> u32 {
    fn_006cee10(e, this, 0)
}

// Translated from 006cee10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Element `index` of a `BSSimpleArray<NavMeshEdgeLocation>` (0x18 bytes
/// each, from `pBuffer`).
pub fn fn_006cee10(e: &mut Engine, this: u32, index: u32) -> u32 {
    let buffer = e.get(Ptr::<BSSimpleArray>::new(this), BSSimpleArray::pBuffer);
    index.wrapping_mul(EDGE_LOCATION_SIZE).wrapping_add(buffer)
}

/// The capacity setter shared by `fn_006cee30` and `fn_006ceea0`: nothing
/// when `count` is the capacity; elements past `count` are destroyed
/// (`0072ba80`) and the size cut; then `reallocate` (count, size) and the
/// new capacity.
fn set_capacity(e: &mut Engine, this: u32, count: u32, stride: u32, reallocate: u32) {
    let array = Ptr::<BSSimpleArray>::new(this);
    if count == e.get(array, BSSimpleArray::iReservedSize) {
        return;
    }
    let size = e.get(array, BSSimpleArray::iSize);
    if count < size {
        let buffer = e.get(array, BSSimpleArray::pBuffer);
        e.call(
            SPACE_RANGE_DESTRUCT,
            &[
                this,
                count.wrapping_mul(stride).wrapping_add(buffer),
                size.wrapping_sub(count),
            ],
        );
        e.set(array, BSSimpleArray::iSize, count);
    }
    let size = e.get(array, BSSimpleArray::iSize);
    e.call(reallocate, &[this, count, size]);
    e.set(array, BSSimpleArray::iReservedSize, count);
}

// Translated from 006cee30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the capacity of a `BSSimpleArray<ParentSpaceNode>` (12-byte
/// elements) to `count` (`006dc590` reallocates).
pub fn fn_006cee30(e: &mut Engine, this: u32, count: u32) {
    set_capacity(
        e,
        this,
        count,
        PARENT_SPACE_NODE_SIZE,
        SPACE_ARRAY_REALLOCATE,
    );
}

// Translated from 006ceea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the capacity of a `BSSimpleArray<TeleportLink>` (16-byte elements)
/// to `count` (`009f6540` reallocates).
pub fn fn_006ceea0(e: &mut Engine, this: u32, count: u32) {
    set_capacity(e, this, count, TELEPORT_LINK_SIZE, LINK_ARRAY_REALLOCATE);
}

// Translated from 006cef10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<VirtualPathingNode>`: makes room for one more element
/// (a full array allocates 4 through virtual slot 4 when it has no
/// capacity, otherwise grows to `009a3910`'s capacity with `006cfe70`) and
/// returns the new element's index (the size before).
pub fn fn_006cef10(e: &mut Engine, this: u32) -> u32 {
    let array = Ptr::<BSSimpleArray>::new(this);
    if call_bool(e, ARRAY_IS_FULL, &[this]) {
        if e.get(array, BSSimpleArray::iReservedSize) == 0 {
            let capacity = 4u32;
            let buffer = e.vcall(this, 4, &[capacity]).u32();
            e.set(array, BSSimpleArray::pBuffer, buffer);
            e.set(array, BSSimpleArray::iReservedSize, capacity);
        } else {
            let capacity = call_u32(e, ARRAY_GROWN_CAPACITY, &[this]);
            let size = e.get(array, BSSimpleArray::iSize);
            e.call(VIRTUAL_NODE_ARRAY_REALLOCATE, &[this, capacity, size]);
            e.set(array, BSSimpleArray::iReservedSize, capacity);
        }
    }
    let size = e.get(array, BSSimpleArray::iSize).wrapping_add(1);
    e.set(array, BSSimpleArray::iSize, size);
    size.wrapping_sub(1)
}

/// The clear shared by `fn_006cefa0` and `fn_006cf120`: destroys the
/// elements, frees the buffer when `free` (`006a8500`), size 0.
fn clear_array(e: &mut Engine, this: u32, free: bool, destruct: u32) {
    let array = Ptr::<BSSimpleArray>::new(this);
    if e.get(array, BSSimpleArray::pBuffer) == 0 {
        return;
    }
    let buffer = e.get(array, BSSimpleArray::pBuffer);
    let size = e.get(array, BSSimpleArray::iSize);
    e.call(destruct, &[this, buffer, size]);
    if free {
        e.call(ARRAY_FREE_BUFFER, &[this]);
        e.set(array, BSSimpleArray::pBuffer, 0);
        e.set(array, BSSimpleArray::iReservedSize, 0);
    }
    e.set(array, BSSimpleArray::iSize, 0);
}

// Translated from 006cefa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears a `BSSimpleArray<VirtualPathingNode>` (`006cfe30` destroys the
/// elements), freeing the buffer when `free`.
pub fn fn_006cefa0(e: &mut Engine, this: u32, free: bool) {
    clear_array(e, this, free, VIRTUAL_NODE_RANGE_DESTRUCT);
}

// Translated from 006cf000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes element `index` of a `BSSimpleArray<PathingNode>` (0x3C-byte
/// elements): with `shrink` and when the array should shrink
/// (`006f3170`), into a new buffer of `00869600`'s capacity (virtual slot
/// 4): the elements before `index`, then `size - 1` elements from after it
/// (the count the code passes), the old buffer freed. Otherwise in place:
/// the element destroyed and the rest moved down. The size drops by one.
pub fn fn_006cf000(e: &mut Engine, this: u32, index: u32, shrink: bool) {
    let array = Ptr::<BSSimpleArray>::new(this);
    let offset = index.wrapping_mul(PATHING_NODE_SIZE);
    if shrink && call_bool(e, ARRAY_SHOULD_SHRINK, &[this]) {
        let capacity = call_u32(e, ARRAY_SHRUNK_CAPACITY, &[this]);
        let buffer = e.vcall(this, 4, &[capacity]).u32();
        let old = e.get(array, BSSimpleArray::pBuffer);
        e.call(PATHING_NODE_RANGE_MOVE, &[this, buffer, old, index]);
        let old = e.get(array, BSSimpleArray::pBuffer);
        e.call(
            PATHING_NODE_RANGE_DESTRUCT,
            &[this, offset.wrapping_add(old), 1],
        );
        let size = e.get(array, BSSimpleArray::iSize);
        let old = e.get(array, BSSimpleArray::pBuffer);
        e.call(
            PATHING_NODE_RANGE_MOVE,
            &[
                this,
                offset.wrapping_add(buffer),
                old.wrapping_add(offset).wrapping_add(PATHING_NODE_SIZE),
                size.wrapping_sub(1),
            ],
        );
        e.call(ARRAY_FREE_BUFFER, &[this]);
        e.set(array, BSSimpleArray::pBuffer, buffer);
        e.set(array, BSSimpleArray::iReservedSize, capacity);
    } else {
        let buffer = e.get(array, BSSimpleArray::pBuffer);
        e.call(
            PATHING_NODE_RANGE_DESTRUCT,
            &[this, offset.wrapping_add(buffer), 1],
        );
        let size = e.get(array, BSSimpleArray::iSize);
        let buffer = e.get(array, BSSimpleArray::pBuffer);
        e.call(
            PATHING_NODE_RANGE_MOVE,
            &[
                this,
                offset.wrapping_add(buffer),
                buffer.wrapping_add(offset).wrapping_add(PATHING_NODE_SIZE),
                size.wrapping_sub(index).wrapping_sub(1),
            ],
        );
    }
    let size = e.get(array, BSSimpleArray::iSize).wrapping_sub(1);
    e.set(array, BSSimpleArray::iSize, size);
}

// Translated from 006cf120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears a `BSSimpleArray<PathingNode>` (`006cff20` destroys the
/// elements), freeing the buffer when `free`.
pub fn fn_006cf120(e: &mut Engine, this: u32, free: bool) {
    clear_array(e, this, free, PATHING_NODE_RANGE_DESTRUCT);
}

// Translated from 006cf180 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSSimpleArray<PathingLocation,1024>` constructor (vtable
/// `0106b91c`, RTTI): stores the vtable and initialises the array
/// (`006d0140` with 0, 0); returns `this`.
pub fn fn_006cf180(e: &mut Engine, this: u32) -> u32 {
    e.mem.set_u32(this, LOCATION_ARRAY_VTABLE);
    e.call(LOCATION_ARRAY_INIT, &[this, 0, 0]);
    this
}

// Translated from 006cf1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<PathingLocation>`: appends a copy of `location` (a slot
/// from `006925b0`, constructed by `006d00a0`, then assigned); returns its
/// index.
pub fn fn_006cf1b0(e: &mut Engine, this: u32, location: u32) -> u32 {
    let index = call_u32(e, LOCATION_ARRAY_ADD_SLOT, &[this]);
    let array = Ptr::<BSSimpleArray>::new(this);
    let buffer = e.get(array, BSSimpleArray::pBuffer);
    let slot = index
        .wrapping_mul(PATHING_LOCATION_SIZE)
        .wrapping_add(buffer);
    e.call(LOCATION_ARRAY_CONSTRUCT_ELEMENTS, &[this, slot, 1]);
    let buffer = e.get(array, BSSimpleArray::pBuffer);
    let slot = index
        .wrapping_mul(PATHING_LOCATION_SIZE)
        .wrapping_add(buffer);
    e.call(LOCATION_ASSIGN, &[slot, location]);
    index
}

// Translated from 006cf200 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSSimpleArray<NavMeshEdgeLocation,1024>` constructor (vtable
/// `0106c954`, RTTI): stores the vtable and initialises the array
/// (`fn_006d0310` with 0, 0); returns `this`.
pub fn fn_006cf200(e: &mut Engine, this: u32) -> u32 {
    e.mem.set_u32(this, EDGE_ARRAY_VTABLE);
    fn_006d0310(e, this, 0, 0);
    this
}

// Translated from 006cf230 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSSimpleArray<NavMeshEdgeLocation,1024>` destructor: stores the
/// vtable and clears the array, freeing the buffer (`008454f0` with 1).
pub fn fn_006cf230(e: &mut Engine, this: u32) {
    e.mem.set_u32(this, EDGE_ARRAY_VTABLE);
    e.call(ARRAY_CLEAR_008454F0, &[this, 1]);
}

// Translated from 006cf250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NavMeshEdgeLocation,1024>::_Allocate` (Xbox PDB): a block
/// of `count` 0x18-byte elements from the memory manager (`00401000`).
pub fn bs_simple_array_nav_mesh_edge_location_allocate(
    e: &mut Engine,
    _this: u32,
    count: u32,
) -> u32 {
    call_u32(
        e,
        MEMORY_ALLOCATE,
        &[count.wrapping_mul(EDGE_LOCATION_SIZE)],
    )
}

// Translated from 006cf280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NavMeshEdgeLocation,1024>::_Reallocate` (Xbox PDB):
/// resizes `block` to `count` elements through the memory manager
/// (`0042f5d0`) and returns what it returns (the new block, left in `EAX`).
pub fn bs_simple_array_nav_mesh_edge_location_reallocate(
    e: &mut Engine,
    _this: u32,
    block: u32,
    count: u32,
) -> u32 {
    call_u32(
        e,
        MEMORY_REALLOCATE,
        &[block, count.wrapping_mul(EDGE_LOCATION_SIZE)],
    )
}

/// The functions a scrap array constructor of this file runs: its base
/// `BSSimpleArray` constructor, its vtable and its array initialiser.
#[derive(Clone, Copy)]
struct ScrapArrayKind {
    base: u32,
    vtable: u32,
    init: u32,
}

/// The constructor shape shared by the scrap arrays of this file: the base
/// constructor, the scrap array's vtable, the scrap heap (`heap`, or the
/// thread's scrap heap from `MemoryManager::GetThreadScrapHeap` when it is
/// 0) at +0x10, then the array initialiser with `capacity` and `size`.
/// Returns `this`.
fn construct_scrap_array(
    e: &mut Engine,
    this: u32,
    kind: ScrapArrayKind,
    capacity: u32,
    size: u32,
    heap: u32,
) -> u32 {
    e.call(kind.base, &[this]);
    e.mem.set_u32(this, kind.vtable);
    let heap = if heap == 0 {
        let manager = call_u32(e, MEMORY_MANAGER_GET, &[]);
        call_u32(e, GET_THREAD_SCRAP_HEAP, &[manager])
    } else {
        heap
    };
    e.mem.set_u32(this + SCRAP_ARRAY_HEAP, heap);
    e.call(kind.init, &[this, capacity, size]);
    this
}

/// The destructor shape shared by the scrap arrays of this file: the scrap
/// array's vtable, the clear that frees the buffer (`008454f0` with 1),
/// then the base `BSSimpleArray` destructor.
fn destruct_scrap_array(e: &mut Engine, this: u32, vtable: u32, base: u32) {
    e.mem.set_u32(this, vtable);
    e.call(ARRAY_CLEAR_008454F0, &[this, 1]);
    e.call(base, &[this]);
}

// Translated from 006cf2a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006cf2a0 - the C++ exception-unwinding frame (FS:[0]) that destroys the base array when a callee throws is not translated
/// The `BSScrapArray<NavMeshInfo const *,1024>` constructor (vtable
/// `0106c968`, RTTI): base `005e0510`, the scrap heap (`heap` or the
/// thread's), then `006b3eb0` (capacity, size). Returns `this`.
pub fn fn_006cf2a0(e: &mut Engine, this: u32, capacity: u32, size: u32, heap: u32) -> u32 {
    let kind = ScrapArrayKind {
        base: INFO_ARRAY_BASE_CONSTRUCT,
        vtable: INFO_SCRAP_ARRAY_VTABLE,
        init: ARRAY_INIT_006B3EB0,
    };
    construct_scrap_array(e, this, kind, capacity, size, heap)
}

// Translated from 006cf3a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006cf3a0 - the C++ exception-unwinding frame (FS:[0]) that destroys the base array when a callee throws is not translated
/// The `BSScrapArray<TESObjectREFR *,1024>` constructor (vtable
/// `0106c97c`, RTTI): base `005e04c0`, the scrap heap (`heap` or the
/// thread's), then `006b3eb0` (capacity, size). Returns `this`.
pub fn fn_006cf3a0(e: &mut Engine, this: u32, capacity: u32, size: u32, heap: u32) -> u32 {
    let kind = ScrapArrayKind {
        base: REFERENCE_ARRAY_BASE_CONSTRUCT,
        vtable: REFERENCE_SCRAP_ARRAY_VTABLE,
        init: ARRAY_INIT_006B3EB0,
    };
    construct_scrap_array(e, this, kind, capacity, size, heap)
}

// Translated from 006cf440 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006cf440 - the C++ exception-unwinding frame (FS:[0]) that still runs the base destructor when the clear throws is not translated
/// The `BSScrapArray<TESObjectREFR *,1024>` destructor: vtable `0106c97c`,
/// clear with free (`008454f0`), then the base destructor `005e04f0`.
pub fn fn_006cf440(e: &mut Engine, this: u32) {
    destruct_scrap_array(
        e,
        this,
        REFERENCE_SCRAP_ARRAY_VTABLE,
        REFERENCE_ARRAY_BASE_DESTRUCT,
    );
}

// Translated from 006cf520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<bool>`: appends the byte `*value` (a slot from
/// `006d0450`, constructed by `006e1280`, then written); returns its index.
pub fn fn_006cf520(e: &mut Engine, this: u32, value: u32) -> u32 {
    let index = call_u32(e, BOOL_ARRAY_ADD_SLOT, &[this]);
    let array = Ptr::<BSSimpleArray>::new(this);
    let buffer = e.get(array, BSSimpleArray::pBuffer);
    e.call(
        BOOL_ARRAY_CONSTRUCT_ELEMENTS,
        &[this, buffer.wrapping_add(index), 1],
    );
    let buffer = e.get(array, BSSimpleArray::pBuffer);
    let byte = e.mem.u8(value);
    e.mem.set_u8(buffer.wrapping_add(index), byte);
    index
}

// Translated from 006cf570 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006cf570 - the C++ exception-unwinding frame (FS:[0]) that destroys the base array when a callee throws is not translated
/// The `BSScrapArray<bool,1024>` constructor (vtable `0106c990`, RTTI):
/// base `006d0420`, the scrap heap (`heap` or the thread's), then
/// `0069b940` (capacity, size). Returns `this`.
pub fn fn_006cf570(e: &mut Engine, this: u32, capacity: u32, size: u32, heap: u32) -> u32 {
    let kind = ScrapArrayKind {
        base: BOOL_ARRAY_BASE_CONSTRUCT,
        vtable: BOOL_SCRAP_ARRAY_VTABLE,
        init: ARRAY_INIT_0069B940,
    };
    construct_scrap_array(e, this, kind, capacity, size, heap)
}

// Translated from 006cf670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapArray<bool,1024>::_Allocate` (Xbox PDB): `count` bytes from the
/// array's scrap heap (`ScrapHeap::Allocate` with the alignment word at
/// `010a2720`).
pub fn bs_scrap_array_bool_allocate(e: &mut Engine, this: u32, count: u32) -> u32 {
    let alignment = e.global::<u32>(SCRAP_ALIGNMENT_010A2720);
    let heap = e.mem.u32(this + SCRAP_ARRAY_HEAP);
    call_u32(e, SCRAP_HEAP_ALLOCATE, &[heap, count, alignment])
}

// Translated from 006cf6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapArray<bool,1024>::_Reallocate` (Xbox PDB): a new block of
/// `count` bytes from the scrap heap, the array's size (`0044ddc0`) in
/// bytes copied from `block` (`memcpy`), `block` given back to the heap;
/// returns the new block.
pub fn bs_scrap_array_bool_reallocate(e: &mut Engine, this: u32, block: u32, count: u32) -> u32 {
    let alignment = e.global::<u32>(SCRAP_ALIGNMENT_010A2720);
    let heap = e.mem.u32(this + SCRAP_ARRAY_HEAP);
    let fresh = call_u32(e, SCRAP_HEAP_ALLOCATE, &[heap, count, alignment]);
    let size = array_size(e, this);
    e.call(MEMORY_COPY, &[fresh, block, size]);
    let heap = e.mem.u32(this + SCRAP_ARRAY_HEAP);
    e.call(SCRAP_HEAP_DEALLOCATE, &[heap, block]);
    fresh
}

// Translated from 006cf780 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006cf780 - the C++ exception-unwinding frame (FS:[0]) that destroys the base array when a callee throws is not translated
/// The `BSScrapArray<PathBuilder::VirtualNodeSegment,1024>` constructor
/// (vtable `0106c9a4`, RTTI): base `006d03a0`, the scrap heap (`heap` or
/// the thread's), then `006b3f40` (capacity, size). Returns `this`.
pub fn fn_006cf780(e: &mut Engine, this: u32, capacity: u32, size: u32, heap: u32) -> u32 {
    let kind = ScrapArrayKind {
        base: SEGMENT_ARRAY_BASE_CONSTRUCT,
        vtable: SEGMENT_SCRAP_ARRAY_VTABLE,
        init: ARRAY_INIT_006B3F40,
    };
    construct_scrap_array(e, this, kind, capacity, size, heap)
}

// Translated from 006cf840 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006cf840 - the C++ exception-unwinding frame (FS:[0]) that still runs the base destructor when the clear throws is not translated
/// The `BSScrapArray<PathBuilder::VirtualNodeSegment,1024>` destructor:
/// vtable `0106c9a4`, clear with free (`008454f0`), then the base
/// destructor `006cf820`.
pub fn fn_006cf840(e: &mut Engine, this: u32) {
    destruct_scrap_array(
        e,
        this,
        SEGMENT_SCRAP_ARRAY_VTABLE,
        SEGMENT_ARRAY_BASE_DESTRUCT,
    );
}

// Translated from 006cf9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSSimpleArray<NavMeshSearchLink,1024>` constructor (vtable
/// `0106b8c8`, RTTI): stores the vtable and runs `006b3f40` (capacity,
/// size); returns `this`.
pub fn fn_006cf9a0(e: &mut Engine, this: u32, capacity: u32, size: u32) -> u32 {
    e.mem.set_u32(this, SEARCH_LINK_ARRAY_VTABLE);
    e.call(ARRAY_INIT_006B3F40, &[this, capacity, size]);
    this
}

// Translated from 006cf9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSSimpleArray<NavMeshSearchNode,1024>` constructor (vtable
/// `0106b8a0`, RTTI): stores the vtable and runs `009a3d00` (capacity,
/// size); returns `this`.
pub fn fn_006cf9d0(e: &mut Engine, this: u32, capacity: u32, size: u32) -> u32 {
    e.mem.set_u32(this, SEARCH_NODE_ARRAY_VTABLE);
    e.call(ARRAY_INIT_009A3D00, &[this, capacity, size]);
    this
}

// Translated from 006cfa00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006cfa00 - the C++ exception-unwinding frame (FS:[0]) that destroys the base array when a callee throws is not translated
/// The `BSScrapArray<FastNavMeshTriHandle,1024>` constructor (vtable
/// `0106c9cc`, RTTI): base `006d04e0`, the scrap heap (`heap` or the
/// thread's), then `006dc440` (capacity, size). Returns `this`.
pub fn fn_006cfa00(e: &mut Engine, this: u32, capacity: u32, size: u32, heap: u32) -> u32 {
    let kind = ScrapArrayKind {
        base: TRIANGLE_HANDLE_ARRAY_BASE_CONSTRUCT,
        vtable: TRIANGLE_HANDLE_SCRAP_ARRAY_VTABLE,
        init: ARRAY_INIT_006DC440,
    };
    construct_scrap_array(e, this, kind, capacity, size, heap)
}

// Translated from 006cfac0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006cfac0 - the C++ exception-unwinding frame (FS:[0]) that still runs the base destructor when the clear throws is not translated
/// The `BSScrapArray<FastNavMeshTriHandle,1024>` destructor: vtable
/// `0106c9cc`, clear with free (`008454f0`), then the base destructor
/// `006cfaa0`.
pub fn fn_006cfac0(e: &mut Engine, this: u32) {
    destruct_scrap_array(
        e,
        this,
        TRIANGLE_HANDLE_SCRAP_ARRAY_VTABLE,
        TRIANGLE_HANDLE_ARRAY_BASE_DESTRUCT,
    );
}

// Translated from 006cfb20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006cfb20 - the C++ exception-unwinding frame (FS:[0]) that destroys the base array when a callee throws is not translated
/// The `BSScrapArray<int,1024>` constructor (vtable `0106c9f4`, RTTI):
/// base `00694de0`, the scrap heap (`heap` or the thread's), then
/// `006b3eb0` (capacity, size). Returns `this`.
pub fn fn_006cfb20(e: &mut Engine, this: u32, capacity: u32, size: u32, heap: u32) -> u32 {
    let kind = ScrapArrayKind {
        base: INTEGER_ARRAY_BASE_CONSTRUCT,
        vtable: INTEGER_SCRAP_ARRAY_VTABLE,
        init: ARRAY_INIT_006B3EB0,
    };
    construct_scrap_array(e, this, kind, capacity, size, heap)
}

// Translated from 006cfbc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006cfbc0 - the C++ exception-unwinding frame (FS:[0]) that still runs the base destructor when the clear throws is not translated
/// The `BSScrapArray<int,1024>` destructor: vtable `0106c9f4`, clear with
/// free (`008454f0`), then the base destructor `00694e10`.
pub fn fn_006cfbc0(e: &mut Engine, this: u32) {
    destruct_scrap_array(
        e,
        this,
        INTEGER_SCRAP_ARRAY_VTABLE,
        INTEGER_ARRAY_BASE_DESTRUCT,
    );
}

/// The tail of the scalar deleting destructors: when bit 0 of `flags` is
/// set the object goes back to the memory manager (`00401030`). Returns
/// `this`.
fn free_when_flagged(e: &mut Engine, this: u32, flags: u32) -> u32 {
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &[this]);
    }
    this
}

// Translated from 006cfc80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NavMeshEdgeLocation,1024>::_scalar_deleting_destructor_`
/// (Xbox PDB): `fn_006cf230`, then frees `this` when bit 0 of `flags` is
/// set. Returns `this`.
pub fn bs_simple_array_nav_mesh_edge_location_scalar_deleting_destructor(
    e: &mut Engine,
    this: u32,
    flags: u32,
) -> u32 {
    fn_006cf230(e, this);
    free_when_flagged(e, this, flags)
}

// Translated from 006cfcb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapArray<NavMeshInfo const *,1024>::_scalar_deleting_destructor_`
/// (Xbox PDB): the destructor `006cf340` (vtable `0106c968`), then frees
/// `this` when bit 0 of `flags` is set. Returns `this`.
pub fn bs_scrap_array_nav_mesh_info_const_p_scalar_deleting_destructor(
    e: &mut Engine,
    this: u32,
    flags: u32,
) -> u32 {
    e.call(INFO_SCRAP_ARRAY_DESTRUCT, &[this]);
    free_when_flagged(e, this, flags)
}

// Translated from 006cfce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapArray<TESObjectREFR *,1024>::_scalar_deleting_destructor_`
/// (Xbox PDB): `fn_006cf440`, then frees `this` when bit 0 of `flags` is
/// set. Returns `this`.
pub fn bs_scrap_array_tes_object_refr_p_scalar_deleting_destructor(
    e: &mut Engine,
    this: u32,
    flags: u32,
) -> u32 {
    fn_006cf440(e, this);
    free_when_flagged(e, this, flags)
}

// Translated from 006cfd10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<PathBuilder::VirtualNodeSegment,1024>::_scalar_deleting_destructor_`
/// (Xbox PDB): the destructor `006cf820` (vtable `0106c9b8`), then frees
/// `this` when bit 0 of `flags` is set. Returns `this`.
pub fn bs_simple_array_path_builder_virtual_node_segment_scalar_deleting_destructor(
    e: &mut Engine,
    this: u32,
    flags: u32,
) -> u32 {
    e.call(SEGMENT_ARRAY_BASE_DESTRUCT, &[this]);
    free_when_flagged(e, this, flags)
}

// Translated from 006cfd40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapArray<bool,1024>::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor `006cf610` (vtable `0106c990`), then frees `this` when bit 0
/// of `flags` is set. Returns `this`.
pub fn bs_scrap_array_bool_scalar_deleting_destructor(
    e: &mut Engine,
    this: u32,
    flags: u32,
) -> u32 {
    e.call(BOOL_SCRAP_ARRAY_DESTRUCT, &[this]);
    free_when_flagged(e, this, flags)
}

// Translated from 006cfd70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapArray<PathBuilder::VirtualNodeSegment,1024>::_scalar_deleting_destructor_`
/// (Xbox PDB): `fn_006cf840`, then frees `this` when bit 0 of `flags` is
/// set. Returns `this`.
pub fn bs_scrap_array_path_builder_virtual_node_segment_scalar_deleting_destructor(
    e: &mut Engine,
    this: u32,
    flags: u32,
) -> u32 {
    fn_006cf840(e, this);
    free_when_flagged(e, this, flags)
}

// Translated from 006cfda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<FastNavMeshTriHandle,1024>::_scalar_deleting_destructor_`
/// (Xbox PDB): the destructor `006cfaa0` (vtable `0106c9e0`), then frees
/// `this` when bit 0 of `flags` is set. Returns `this`.
pub fn bs_simple_array_fast_nav_mesh_tri_handle_scalar_deleting_destructor(
    e: &mut Engine,
    this: u32,
    flags: u32,
) -> u32 {
    e.call(TRIANGLE_HANDLE_ARRAY_BASE_DESTRUCT, &[this]);
    free_when_flagged(e, this, flags)
}

// Translated from 006cfdd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapArray<FastNavMeshTriHandle,1024>::_scalar_deleting_destructor_`
/// (Xbox PDB): `fn_006cfac0`, then frees `this` when bit 0 of `flags` is
/// set. Returns `this`.
pub fn bs_scrap_array_fast_nav_mesh_tri_handle_scalar_deleting_destructor(
    e: &mut Engine,
    this: u32,
    flags: u32,
) -> u32 {
    fn_006cfac0(e, this);
    free_when_flagged(e, this, flags)
}

// Translated from 006cfe00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapArray<int,1024>::_scalar_deleting_destructor_` (Xbox PDB):
/// `fn_006cfbc0`, then frees `this` when bit 0 of `flags` is set. Returns
/// `this`.
pub fn bs_scrap_array_int_scalar_deleting_destructor(e: &mut Engine, this: u32, flags: u32) -> u32 {
    fn_006cfbc0(e, this);
    free_when_flagged(e, this, flags)
}

// Translated from 006d0310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NavMeshEdgeLocation>` initialiser: clears buffer, size
/// and capacity; allocates `max(capacity, size)` elements through virtual
/// slot 4 when that is not 0; constructs `size` elements (`006d05e0`) when
/// `size` is not 0.
pub fn fn_006d0310(e: &mut Engine, this: u32, capacity: u32, size: u32) {
    let array = Ptr::<BSSimpleArray>::new(this);
    e.set(array, BSSimpleArray::pBuffer, 0);
    e.set(array, BSSimpleArray::iSize, 0);
    e.set(array, BSSimpleArray::iReservedSize, 0);
    let capacity = capacity.max(size);
    if capacity != 0 {
        let buffer = e.vcall(this, 4, &[capacity]).u32();
        e.set(array, BSSimpleArray::pBuffer, buffer);
        e.set(array, BSSimpleArray::iReservedSize, capacity);
    }
    if size != 0 {
        let buffer = e.get(array, BSSimpleArray::pBuffer);
        e.call(EDGE_ARRAY_CONSTRUCT_ELEMENTS, &[this, buffer, size]);
        e.set(array, BSSimpleArray::iSize, size);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x006c94c0,
            path_builder_build_nav_mesh_info_path(Ptr<PathBuilder>, u32, u32) -> bool
        ),
        entry!(0x006c9900, fn_006c9900(u32) -> u32),
        entry!(0x006c9920, fn_006c9920(u32, u32) -> u32),
        entry!(0x006c99c0, fn_006c99c0(u32, u32, u32) -> u32),
        entry!(0x006c9a70, fn_006c9a70(u32, u32, u32, u32) -> u32),
        entry!(
            0x006c9b20,
            path_builder_build_teleport_door_path(Ptr<PathBuilder>) -> bool
        ),
        entry!(0x006c9fa0, fn_006c9fa0(u32, bool)),
        entry!(
            0x006c9fc0,
            fn_006c9fc0(Ptr<PathBuilder>, i32, u32, u32) -> bool
        ),
        entry!(0x006ca0c0, fn_006ca0c0(u32) -> bool),
        entry!(0x006ca0e0, fn_006ca0e0(Ptr<PathBuilder>, u32, i32) -> bool),
        entry!(0x006ca4c0, fn_006ca4c0(u32) -> bool),
        entry!(0x006ca4e0, fn_006ca4e0(u32) -> f32),
        entry!(0x006ca500, fn_006ca500(Ptr<PathBuilder>, u32) -> bool),
        entry!(0x006ca830, fn_006ca830(u32) -> bool),
        entry!(0x006ca850, fn_006ca850(Ptr<PathBuilder>, u32) -> bool),
        entry!(0x006caa20, fn_006caa20(u32) -> bool),
        entry!(
            0x006caa40,
            fn_006caa40(Ptr<PathBuilder>, i32, u32, u32) -> i32
        ),
        entry!(0x006cac70, fn_006cac70(u32) -> bool),
        entry!(
            0x006cac90,
            fn_006cac90(Ptr<PathBuilder>, i32, u32, u32) -> i32
        ),
        entry!(
            0x006cb050,
            path_builder_build_path_to_valid_nav_mesh_triangle(
                Ptr<PathBuilder>,
                u32,
                u32,
                bool,
                u32,
            ) -> i32
        ),
        entry!(0x006cb3a0, ray_cast_path_request_ray_cast_path_request(u32) -> u32),
        entry!(0x006cb410, fn_006cb410(u32)),
        entry!(0x006cb480, fn_006cb480(u32) -> u32),
        entry!(0x006cb4a0, fn_006cb4a0(u32)),
        entry!(
            0x006cb4c0,
            path_builder_build_straight_path_using_ray_casts(
                Ptr<PathBuilder>,
                u32,
                u32,
                u32,
            ) -> i32
        ),
        entry!(
            0x006cb6a0,
            fn_006cb6a0(Ptr<PathBuilder>, u32, u32, u32) -> bool
        ),
        entry!(0x006cb900, fn_006cb900(u32, u32) -> u32),
        entry!(0x006cb950, fn_006cb950(Ptr<PathBuilder>, u32, u32) -> bool),
        entry!(0x006cc560, pathing_node_set_teleport_door(u32, u32)),
        entry!(0x006cc5a0, fn_006cc5a0(u32, bool)),
        entry!(
            0x006cc5e0,
            fn_006cc5e0(Ptr<PathBuilder>, u32, u32, u32, u32) -> i32
        ),
        entry!(0x006cd1d0, fn_006cd1d0(u32) -> bool),
        entry!(0x006cd1f0, fn_006cd1f0(Ptr<PathBuilder>, u32, u32) -> bool),
        entry!(0x006cd5a0, fn_006cd5a0(u32) -> u32),
        entry!(
            0x006cd5c0,
            check_line_of_sight_filter_water_can_cross_edge(
                Ptr<CheckLineOfSightFilterWater>,
                u32,
                u32,
                u32,
                u32,
                u16,
                u32,
            ) -> bool
        ),
        entry!(0x006cd650, fn_006cd650(u32) -> u32),
        entry!(
            0x006cd670,
            fn_006cd670(Ptr<PathBuilder>, u32, u32, u32, u32, u32, u32, u32) -> bool
        ),
        entry!(0x006cd880, fn_006cd880(u32, u32)),
        entry!(
            0x006cd8a0,
            fn_006cd8a0(Ptr<PathBuilder>, u32, u32, u32, u32, u32) -> bool
        ),
        (0x006cdb40, fn_006cdb40_entry as AbiFn),
        entry!(
            0x006cde50,
            fn_006cde50(Ptr<PathBuilder>, u32, u32, u32) -> bool
        ),
        entry!(
            0x006ce2d0,
            fn_006ce2d0(Ptr<PathBuilder>, u32, u32, u32) -> bool
        ),
        entry!(0x006ce7e0, pathing_solution_add_virtual_pathing_node(u32, u32, u32) -> u32),
        entry!(
            0x006ce8b0,
            path_builder_build_nav_mesh_info_path_in_space(
                Ptr<PathBuilder>,
                u32,
                u32,
                u32,
                u32,
            ) -> bool
        ),
        entry!(0x006cec40, fn_006cec40(Ptr<PathBuilder>, u32, u32) -> bool),
        entry!(0x006ced60, fn_006ced60(u32, u32, u32, u32, u32) -> bool),
        entry!(0x006ced70, fn_006ced70(u32) -> u32),
        entry!(0x006ced90, fn_006ced90(u32) -> u32),
        entry!(0x006cedb0, fn_006cedb0(u32) -> u32),
        entry!(0x006cedd0, fn_006cedd0(u32) -> u32),
        entry!(0x006cedf0, fn_006cedf0(u32) -> u32),
        entry!(0x006cee10, fn_006cee10(u32, u32) -> u32),
        entry!(0x006cee30, fn_006cee30(u32, u32)),
        entry!(0x006ceea0, fn_006ceea0(u32, u32)),
        entry!(0x006cef10, fn_006cef10(u32) -> u32),
        entry!(0x006cefa0, fn_006cefa0(u32, bool)),
        entry!(0x006cf000, fn_006cf000(u32, u32, bool)),
        entry!(0x006cf120, fn_006cf120(u32, bool)),
        entry!(0x006cf180, fn_006cf180(u32) -> u32),
        entry!(0x006cf1b0, fn_006cf1b0(u32, u32) -> u32),
        entry!(0x006cf200, fn_006cf200(u32) -> u32),
        entry!(0x006cf230, fn_006cf230(u32)),
        entry!(
            0x006cf250,
            bs_simple_array_nav_mesh_edge_location_allocate(u32, u32) -> u32
        ),
        entry!(
            0x006cf280,
            bs_simple_array_nav_mesh_edge_location_reallocate(u32, u32, u32) -> u32
        ),
        entry!(0x006cf2a0, fn_006cf2a0(u32, u32, u32, u32) -> u32),
        entry!(0x006cf3a0, fn_006cf3a0(u32, u32, u32, u32) -> u32),
        entry!(0x006cf440, fn_006cf440(u32)),
        entry!(0x006cf520, fn_006cf520(u32, u32) -> u32),
        entry!(0x006cf570, fn_006cf570(u32, u32, u32, u32) -> u32),
        entry!(0x006cf670, bs_scrap_array_bool_allocate(u32, u32) -> u32),
        entry!(0x006cf6a0, bs_scrap_array_bool_reallocate(u32, u32, u32) -> u32),
        entry!(0x006cf780, fn_006cf780(u32, u32, u32, u32) -> u32),
        entry!(0x006cf840, fn_006cf840(u32)),
        entry!(0x006cf9a0, fn_006cf9a0(u32, u32, u32) -> u32),
        entry!(0x006cf9d0, fn_006cf9d0(u32, u32, u32) -> u32),
        entry!(0x006cfa00, fn_006cfa00(u32, u32, u32, u32) -> u32),
        entry!(0x006cfac0, fn_006cfac0(u32)),
        entry!(0x006cfb20, fn_006cfb20(u32, u32, u32, u32) -> u32),
        entry!(0x006cfbc0, fn_006cfbc0(u32)),
        entry!(
            0x006cfc80,
            bs_simple_array_nav_mesh_edge_location_scalar_deleting_destructor(u32, u32) -> u32
        ),
        entry!(
            0x006cfcb0,
            bs_scrap_array_nav_mesh_info_const_p_scalar_deleting_destructor(u32, u32) -> u32
        ),
        entry!(
            0x006cfce0,
            bs_scrap_array_tes_object_refr_p_scalar_deleting_destructor(u32, u32) -> u32
        ),
        entry!(
            0x006cfd10,
            bs_simple_array_path_builder_virtual_node_segment_scalar_deleting_destructor(
                u32,
                u32,
            ) -> u32
        ),
        entry!(
            0x006cfd40,
            bs_scrap_array_bool_scalar_deleting_destructor(u32, u32) -> u32
        ),
        entry!(
            0x006cfd70,
            bs_scrap_array_path_builder_virtual_node_segment_scalar_deleting_destructor(
                u32,
                u32,
            ) -> u32
        ),
        entry!(
            0x006cfda0,
            bs_simple_array_fast_nav_mesh_tri_handle_scalar_deleting_destructor(u32, u32) -> u32
        ),
        entry!(
            0x006cfdd0,
            bs_scrap_array_fast_nav_mesh_tri_handle_scalar_deleting_destructor(u32, u32) -> u32
        ),
        entry!(
            0x006cfe00,
            bs_scrap_array_int_scalar_deleting_destructor(u32, u32) -> u32
        ),
        entry!(0x006d0310, fn_006d0310(u32, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// Every callee outside this file: each gets a double that returns zero
    /// unless the fixture or a test installs a better one.
    const CALLEES: &[u32] = &[
        REQUEST_ORIGIN,
        REQUEST_DESTINATION,
        REQUEST_ACTOR_RADIUS,
        REQUEST_TARGET_RADIUS,
        REQUEST_FLOAT_8C,
        REQUEST_BYTE_98,
        REQUEST_BYTE_99,
        REQUEST_BYTE_9B,
        REQUEST_BYTE_A2,
        REQUEST_BYTE_A4,
        REQUEST_WORD_A8,
        REQUEST_AVOID_NODE_ARRAY,
        SOLUTION_VIRTUAL_NODE,
        SOLUTION_VIRTUAL_NODE_COUNT,
        SOLUTION_CURRENT_NODE,
        SOLUTION_CURRENT_NODE_COUNT,
        SOLUTION_FIRST_LOADED,
        WORD_AT_1C,
        SOLUTION_LAST_CURRENT_NODE,
        SOLUTION_INCOMPLETE,
        SOLUTION_RESET,
        SOLUTION_ADD_LOCATION_NODE,
        SOLUTION_ADD_CURRENT_NODE,
        SOLUTION_REMOVE_LAST_CURRENT_NODE,
        NODE_LOCATION,
        VIRTUAL_NODE_DOOR,
        NODE_SET_POSITION,
        VIRTUAL_NODE_CONSTRUCT_INFO,
        VIRTUAL_NODE_CONSTRUCT_POINT_INFO,
        VIRTUAL_NODE_CONSTRUCT_INFO_POINT_DOOR,
        VIRTUAL_NODE_CONSTRUCT_LOCATION_DOOR,
        VIRTUAL_NODE_SLOT,
        PLACEMENT_NEW,
        LOCATION_CONSTRUCT,
        LOCATION_COPY_CONSTRUCT,
        LOCATION_ASSIGN,
        LOCATION_DESTRUCT,
        LOCATION_CONSTRUCT_FROM_MESH_POINT,
        LOCATION_CONSTRUCT_FROM_PARTS,
        LOCATION_CONSTRUCT_FROM_POINT_AND_LOCATION,
        LOCATION_CONSTRUCT_FROM_POINT_AND_INFO,
        LOCATION_COPY_POSITION,
        LOCATION_GET_CELL,
        LOCATION_GET_BEST_NAVMESH_INFO,
        LOCATION_GET_NAVMESH,
        LOCATION_GET_NAVMESH_AND_TRIANGLE,
        LOCATION_GET_NAVMESH_INFOS,
        LOCATION_HAS_NAVMESH_INFO,
        LOCATION_RESOLVE_NAVMESH_INFO,
        LOCATION_RESOLVE,
        LOCATION_SET_POSITION,
        POINT3_ADD,
        POINT3_SUBTRACT,
        POINT3_TIMES_SCALAR,
        POINT3_ADD_ASSIGN,
        POINT3_UNIT_CROSS,
        POINT3_DEFAULT_CONSTRUCT,
        POINT3_LENGTH_SQUARED,
        POINT3_LENGTH,
        POINT2_LENGTH_SQUARED,
        POINT2_LENGTH,
        FLOAT_ABSOLUTE_VALUE,
        ARRAY_SIZE,
        ARRAY_IS_EMPTY,
        WORD_ELEMENT,
        LAST_WORD_ELEMENT,
        SPACE_ELEMENT,
        LINK_ELEMENT,
        FIRST_LINK_ELEMENT,
        TRIANGLE_HANDLE_ELEMENT,
        LAST_TRIANGLE_HANDLE,
        LOCATION_ELEMENT,
        ARRAY_FIND,
        BOOL_ARRAY_FIND,
        ARRAY_PUSH_WORD,
        ARRAY_CLEAR_008454F0,
        INFO_SCRAP_ARRAY_CONSTRUCT,
        INFO_SCRAP_ARRAY_DESTRUCT,
        DOOR_SCRAP_ARRAY_CONSTRUCT,
        DOOR_SCRAP_ARRAY_DESTRUCT,
        BOOL_SCRAP_ARRAY_CONSTRUCT,
        BOOL_SCRAP_ARRAY_ADD,
        BOOL_SCRAP_ARRAY_DESTRUCT,
        SEGMENT_SCRAP_ARRAY_CONSTRUCT,
        SEGMENT_SCRAP_ARRAY_DESTRUCT,
        SEGMENT_ARRAY_ADD,
        EDGE_ARRAY_CONSTRUCT,
        EDGE_ARRAY_DESTRUCT,
        EDGE_ARRAY_COPY_TO,
        LOCATION_ARRAY_DESTRUCT,
        LOCATION_ARRAY_INIT,
        LOCATION_ARRAY_ADD_SLOT,
        LOCATION_ARRAY_CONSTRUCT_ELEMENTS,
        LOCATION_ARRAY_COPY_TO,
        LOCATION_ARRAY_CLEAR,
        TRIANGLE_ARRAY_A_CONSTRUCT,
        TRIANGLE_ARRAY_A_DESTRUCT,
        TRIANGLE_ARRAY_C_CONSTRUCT,
        TRIANGLE_ARRAY_C_DESTRUCT,
        PORTAL_ARRAY_CONSTRUCT,
        PORTAL_ARRAY_DESTRUCT,
        INFO_HANDLE_ARRAY_CONSTRUCT,
        INFO_HANDLE_ARRAY_ADD,
        INFO_HANDLE_ARRAY_DESTRUCT,
        INFO_LIST_CONSTRUCT,
        INFO_LIST_DESTRUCT,
        NAVMESH_LIST_CONSTRUCT,
        NAVMESH_LIST_DESTRUCT,
        NEARBY_NAVMESHES_FILL,
        ARRAY_IS_FULL,
        ARRAY_GROWN_CAPACITY,
        ARRAY_SHOULD_SHRINK,
        ARRAY_SHRUNK_CAPACITY,
        ARRAY_FREE_BUFFER,
        VIRTUAL_NODE_ARRAY_REALLOCATE,
        VIRTUAL_NODE_RANGE_DESTRUCT,
        PATHING_NODE_RANGE_DESTRUCT,
        PATHING_NODE_RANGE_MOVE,
        SPACE_RANGE_DESTRUCT,
        SPACE_ARRAY_REALLOCATE,
        LINK_ARRAY_REALLOCATE,
        NAV_HOLDER_CONSTRUCT,
        NAV_HOLDER_RELEASE,
        NI_POINTER_GET,
        NAVMESH_HOLDER_ARRAY_ADD,
        NAVMESH_HOLDER_ARRAY_CLEAR,
        NAVMESH_INFO_LOAD_HOLDER,
        NAVMESH_GET_INFO,
        NAVMESH_GET_CENTER,
        NAVMESH_GET_TRIANGLE,
        NAVMESH_GET_EDGE_VERTICES,
        NAVMESH_EDGE_POINTS,
        NAVMESH_FIND_EDGE,
        NAVMESH_FIND_PORTAL_TRIANGLES,
        NAVMESH_CELL,
        TRIANGLE_TEST_006A5EC0,
        TRIANGLE_HAS_FLAG_0X10,
        TRIANGLE_FLAGS_SET,
        TRIANGLE_HANDLE_CENTER,
        FAST_TRIANGLE_HANDLE_CONSTRUCT,
        EDGE_HANDLE_COPY,
        FIND_EDGES,
        CHECK_LINE_OF_SIGHT,
        BUILD_FIRST_TANGENT,
        BUILD_LAST_TANGENT,
        CLOSEST_ON_EDGE_006B9C20,
        EDGE_PULL_TEST_006BA100,
        CLAMP_INTEGER,
        TES_GROUND_HEIGHT,
        REFERENCE_LINKED_DOOR_POSITION,
        REFERENCE_GET_POSITION_OWNER,
        FORM_GET_ID,
        CELL_TEST_00450FF0,
        SETTING_BOOL_ADDRESS,
        SETTING_FLOAT_ADDRESS,
        INFO_SEARCH_CONSTRUCT,
        INFO_SEARCH_RUN,
        INFO_SEARCH_DESTRUCT,
        TELEPORT_DOOR_SEARCH_CONSTRUCT,
        TELEPORT_DOOR_SEARCH_BUILD_NODE_PATH,
        TELEPORT_DOOR_SEARCH_DESTRUCT,
        TELEPORT_PATH_CONSTRUCT,
        TELEPORT_PATH_DESTRUCT,
        RAY_CAST_SEARCH_CONSTRUCT,
        RAY_CAST_SEARCH_BUILD_PATH,
        RAY_CAST_SEARCH_DESTRUCT,
        NAVMESH_SEARCH_CONSTRUCT,
        NAVMESH_SEARCH_RUN,
        NAVMESH_SEARCH_DESTRUCT,
        POV_SEARCH_CONSTRUCT,
        POV_SEARCH_RUN,
        POV_SEARCH_DESTRUCT,
        PATH_SMOOTHER_CONSTRUCT,
        PATH_SMOOTHER_RUN,
        PATH_SMOOTHER_DESTRUCT,
        POV_SMOOTHER_CONSTRUCT,
        POV_SMOOTHER_RUN,
        POV_SMOOTHER_DESTRUCT,
        TASKLET_MANAGER_INSTANCE,
        CURRENT_THREAD_QUERY,
        PATH_MANAGER_INSTANCE,
        PATH_MANAGER_GET_RAY_CAST_PATH,
        PATH_MANAGER_GET_RAY_CAST_PATH_TO_NAV_MESH,
        MEMORY_ALLOCATE,
        MEMORY_FREE,
        MEMORY_REALLOCATE,
        MEMORY_MANAGER_GET,
        GET_THREAD_SCRAP_HEAP,
        SCRAP_HEAP_ALLOCATE,
        SCRAP_HEAP_DEALLOCATE,
        MEMORY_COPY,
        BOOL_ARRAY_ADD_SLOT,
        BOOL_ARRAY_CONSTRUCT_ELEMENTS,
        EDGE_ARRAY_CONSTRUCT_ELEMENTS,
        INFO_ARRAY_BASE_CONSTRUCT,
        REFERENCE_ARRAY_BASE_CONSTRUCT,
        REFERENCE_ARRAY_BASE_DESTRUCT,
        BOOL_ARRAY_BASE_CONSTRUCT,
        SEGMENT_ARRAY_BASE_CONSTRUCT,
        SEGMENT_ARRAY_BASE_DESTRUCT,
        TRIANGLE_HANDLE_ARRAY_BASE_CONSTRUCT,
        TRIANGLE_HANDLE_ARRAY_BASE_DESTRUCT,
        INTEGER_ARRAY_BASE_CONSTRUCT,
        INTEGER_ARRAY_BASE_DESTRUCT,
        ARRAY_INIT_006B3EB0,
        ARRAY_INIT_006B3F40,
        ARRAY_INIT_0069B940,
        ARRAY_INIT_009A3D00,
        ARRAY_INIT_006DC440,
    ];

    /// Globals the code reads (mapped zeroed by the fixture), with sizes.
    const GLOBALS: &[(u32, u32)] = &[
        (POINT_011F426C, 12),
        (AXIS_011A9484, 12),
        (TES_POINTER, 4),
        (SETTING_RAY_CAST_011D7404, 8),
        (SETTING_DIRECT_LINE_011D73D8, 8),
        (SETTING_POV_SEARCH_011D7444, 8),
        (SETTING_SMOOTHER_011D7434, 8),
        (SETTING_RADIUS_011D6EF0, 8),
        (TANGENT_DISTANCE_01017A10, 8),
        (HEIGHT_LOW_0106B9F0, 8),
        (HEIGHT_HIGH_0104ED58, 8),
        (CENTER_WEIGHT_0101E2BC, 4),
        (EDGE_WEIGHT_0104E2B8, 4),
        (START_DISTANCE_01016970, 4),
        (GAP_FACTOR_01011590, 8),
        (SHORT_SPAN_01028338, 8),
        (LARGE_RADIUS_0106C948, 8),
        (GROUND_OFFSET_0104F2F0, 8),
        (DOOR_REACH_0101E33C, 4),
        (SCRAP_ALIGNMENT_010A2720, 4),
    ];

    /// The tasklet manager's thread check (vtable slot 0xC) in the tests.
    const THREAD_CHECK: u32 = 0x7ff0_0010;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn ret_f(st0: f64) -> Ret {
        Ret {
            st0,
            ..Ret::default()
        }
    }

    fn read3(e: &Engine, a: u32) -> [f32; 3] {
        [e.mem.f32(a), e.mem.f32(a + 4), e.mem.f32(a + 8)]
    }

    fn write3(e: &mut Engine, a: u32, v: [f32; 3]) {
        e.mem.set_f32(a, v[0]);
        e.mem.set_f32(a + 4, v[1]);
        e.mem.set_f32(a + 8, v[2]);
    }

    struct World {
        e: Engine,
        this: Ptr<PathBuilder>,
        request: u32,
        solution: u32,
    }

    /// Registers the point arithmetic doubles (real `float` math).
    fn point_math(e: &mut Engine) {
        e.register(POINT3_SUBTRACT, |e, a| {
            let (p, q) = (read3(e, a[0]), read3(e, a[2]));
            write3(e, a[1], [p[0] - q[0], p[1] - q[1], p[2] - q[2]]);
            ret(a[1])
        });
        e.register(POINT3_ADD, |e, a| {
            let (p, q) = (read3(e, a[0]), read3(e, a[2]));
            write3(e, a[1], [p[0] + q[0], p[1] + q[1], p[2] + q[2]]);
            ret(a[1])
        });
        e.register(POINT3_TIMES_SCALAR, |e, a| {
            let (p, s) = (read3(e, a[0]), f32::from_bits(a[2]));
            write3(e, a[1], [p[0] * s, p[1] * s, p[2] * s]);
            ret(a[1])
        });
        e.register(POINT3_ADD_ASSIGN, |e, a| {
            let (p, q) = (read3(e, a[0]), read3(e, a[1]));
            write3(e, a[0], [p[0] + q[0], p[1] + q[1], p[2] + q[2]]);
            ret(a[0])
        });
        e.register(POINT2_LENGTH_SQUARED, |e, a| {
            let p = read3(e, a[0]);
            ret_f(f64::from(p[0] * p[0] + p[1] * p[1]))
        });
        e.register(POINT2_LENGTH, |e, a| {
            let p = read3(e, a[0]);
            ret_f(f64::from((p[0] * p[0] + p[1] * p[1]).sqrt()))
        });
        e.register(POINT3_LENGTH_SQUARED, |e, a| {
            let p = read3(e, a[0]);
            ret_f(f64::from(p[0] * p[0] + p[1] * p[1] + p[2] * p[2]))
        });
        e.register(POINT3_LENGTH, |e, a| {
            let p = read3(e, a[0]);
            ret_f(f64::from((p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt()))
        });
        e.register(FLOAT_ABSOLUTE_VALUE, |_, a| {
            ret_f(f64::from(f32::from_bits(a[0]).abs()))
        });
    }

    /// Registers the accessors that read the memory the tests build.
    fn accessors(e: &mut Engine) {
        e.register(ARRAY_SIZE, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(ARRAY_IS_EMPTY, |e, a| {
            ret(u32::from(e.mem.u32(a[0] + 8) == 0))
        });
        e.register(WORD_ELEMENT, |e, a| ret(e.mem.u32(a[0] + 4) + 4 * a[1]));
        e.register(SPACE_ELEMENT, |e, a| ret(e.mem.u32(a[0] + 4) + 12 * a[1]));
        e.register(LINK_ELEMENT, |e, a| ret(e.mem.u32(a[0] + 4) + 16 * a[1]));
        e.register(FIRST_LINK_ELEMENT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(LOCATION_ELEMENT, |e, a| {
            ret(e.mem.u32(a[0] + 4) + 0x28 * a[1])
        });
        e.register(TRIANGLE_HANDLE_ELEMENT, |e, a| {
            ret(e.mem.u32(a[0] + 4) + 0x14 * a[1])
        });
        e.register(LAST_TRIANGLE_HANDLE, |e, a| {
            ret(e.mem.u32(a[0] + 4) + 0x14 * (e.mem.u32(a[0] + 8) - 1))
        });
        e.register(LAST_WORD_ELEMENT, |e, a| {
            ret(e.mem.u32(a[0] + 4) + 4 * (e.mem.u32(a[0] + 8) - 1))
        });
        e.register(NODE_LOCATION, |_, a| ret(a[0] + 4));
        e.register(SOLUTION_VIRTUAL_NODE, |e, a| {
            ret(e.mem.u32(a[0] + 0xc) + 0x30 * a[1])
        });
        e.register(SOLUTION_VIRTUAL_NODE_COUNT, |e, a| {
            ret(e.mem.u32(a[0] + 0x10))
        });
        e.register(SOLUTION_FIRST_LOADED, |e, a| ret(e.mem.u32(a[0] + 0x18)));
        e.register(WORD_AT_1C, |e, a| ret(e.mem.u32(a[0] + 0x1c)));
        e.register(REQUEST_ORIGIN, |_, a| ret(a[0] + 0xc));
        e.register(REQUEST_DESTINATION, |_, a| ret(a[0] + 0x34));
        e.register(LOCATION_COPY_POSITION, |e, a| {
            let p = read3(e, a[0] + 4);
            write3(e, a[1], p);
            ret(a[1])
        });
        e.register(NI_POINTER_GET, |e, a| ret(e.mem.u32(a[0])));
        e.register(SETTING_BOOL_ADDRESS, |_, a| ret(a[0]));
        e.register(SETTING_FLOAT_ADDRESS, |_, a| ret(a[0]));
    }

    /// A path builder with a request and a solution, every callee doubled,
    /// the accessors reading the memory the tests build, and real point
    /// arithmetic.
    fn world() -> World {
        let mut e = Engine::new();
        for &addr in CALLEES {
            e.register(addr, |_, _| Ret::default());
        }
        for &(addr, size) in GLOBALS {
            e.map(addr, size);
        }
        accessors(&mut e);
        point_math(&mut e);
        // The tasklet manager: an object whose vtable slot 0xC is the
        // thread check (false unless a test says otherwise).
        let manager = e.mem.alloc(8);
        let vtable = e.mem.alloc(16);
        e.mem.set_u32(manager, vtable);
        e.mem.set_u32(vtable + 0xc, THREAD_CHECK);
        e.register(THREAD_CHECK, |_, _| Ret::default());
        e.register_double(TASKLET_MANAGER_INSTANCE, move |_, _| ret(manager));
        let this = e.new_object::<PathBuilder>();
        let request = e.mem.alloc(0xb0);
        let solution = e.mem.alloc(0x44);
        e.set(this, PathBuilder::pRequest, request);
        e.set(this, PathBuilder::pSolution, solution);
        e.set(this, PathBuilder::pScrapHeap, 0x5ca7);
        e.call_log = Some(Vec::new());
        World {
            e,
            this,
            request,
            solution,
        }
    }

    fn returns(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| ret(value));
    }

    fn returns_float(e: &mut Engine, addr: u32, value: f64) {
        e.register_double(addr, move |_, _| ret_f(value));
    }

    fn sequence(e: &mut Engine, addr: u32, values: &[u32]) {
        let values = values.to_vec();
        let mut next = 0usize;
        e.register_double(addr, move |_, _| {
            let value = values[next.min(values.len() - 1)];
            next += 1;
            ret(value)
        });
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// The addresses of `addrs` in the order the log has them.
    fn order(e: &Engine, addrs: &[u32]) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| addrs.contains(a))
            .map(|(a, _)| *a)
            .collect()
    }

    type Points = Rc<RefCell<Vec<[f32; 3]>>>;

    /// Records the point (three floats) at argument word `word` of every
    /// call to `addr`, returning `result`.
    fn record_point(e: &mut Engine, addr: u32, word: usize, result: u32) -> Points {
        let seen: Points = Rc::new(RefCell::new(Vec::new()));
        let sink = seen.clone();
        e.register_double(addr, move |e, a| {
            sink.borrow_mut().push(read3(e, a[word]));
            ret(result)
        });
        seen
    }

    /// A `BSSimpleArray` at `array` holding `count` zeroed elements of
    /// `stride` bytes; returns the buffer.
    fn fill_array(e: &mut Engine, array: u32, stride: u32, count: u32) -> u32 {
        let buffer = e.mem.alloc((stride * count).max(4));
        e.mem.set_u32(array + 4, buffer);
        e.mem.set_u32(array + 8, count);
        e.mem.set_u32(array + 0xc, count);
        buffer
    }

    /// Makes the constructor at `addr` fill its array with the given words.
    fn constructs_words(e: &mut Engine, addr: u32, words: &[u32]) {
        let words = words.to_vec();
        e.register_double(addr, move |e, a| {
            let buffer = fill_array(e, a[0], 4, words.len() as u32);
            for (i, w) in words.iter().enumerate() {
                e.mem.set_u32(buffer + 4 * i as u32, *w);
            }
            ret(a[0])
        });
    }

    /// Gives the solution `flags.len()` virtual nodes with these flags;
    /// returns their buffer.
    fn virtual_nodes(w: &mut World, flags: &[u32]) -> u32 {
        let nodes = w.solution + VIRTUAL_NODES;
        let buffer = fill_array(&mut w.e, nodes, VIRTUAL_NODE_SIZE, flags.len() as u32);
        for (i, f) in flags.iter().enumerate() {
            w.e.mem.set_u32(buffer + VIRTUAL_NODE_SIZE * i as u32, *f);
        }
        buffer
    }

    fn node(buffer: u32, index: u32) -> u32 {
        buffer + VIRTUAL_NODE_SIZE * index
    }

    // -- 006c94c0 ----------------------------------------------------------

    #[test]
    fn build_nav_mesh_info_path_single_info_adds_origin_and_destination() {
        let mut w = world();
        let e = &mut w.e;
        constructs_words(e, INFO_SCRAP_ARRAY_CONSTRUCT, &[0x111]);
        returns(e, REQUEST_BYTE_A4, 1);
        write3(e, w.request + 0xc + 4, [1.0, 2.0, 3.0]);
        write3(e, w.request + 0x34 + 4, [4.0, 5.0, 6.0]);
        type Seen = Rc<RefCell<Vec<([f32; 3], [f32; 3])>>>;
        let points: Seen = Rc::new(RefCell::new(Vec::new()));
        let sink = points.clone();
        e.register_double(INFO_SEARCH_RUN, move |e, a| {
            sink.borrow_mut().push((read3(e, a[2]), read3(e, a[4])));
            ret(1)
        });
        assert!(path_builder_build_nav_mesh_info_path(e, w.this, 0xa1, 0xa2));
        let run = &calls(e, INFO_SEARCH_RUN)[0];
        assert_eq!(run[1], w.request + 0x5c);
        assert_eq!((run[3], run[5], run[8]), (0xa1, 0xa2, 1));
        assert_eq!(points.borrow()[0], ([1.0, 2.0, 3.0], [4.0, 5.0, 6.0]));
        assert_eq!(
            calls(e, SOLUTION_ADD_LOCATION_NODE),
            vec![
                vec![w.solution, w.request + 0xc],
                vec![w.solution, w.request + 0x34]
            ]
        );
        assert_eq!(
            calls(e, INFO_SCRAP_ARRAY_CONSTRUCT)[0][1..],
            [0x14, 0, 0x5ca7]
        );
        assert_eq!(
            order(
                e,
                &[
                    DOOR_SCRAP_ARRAY_DESTRUCT,
                    INFO_SCRAP_ARRAY_DESTRUCT,
                    INFO_SEARCH_DESTRUCT
                ]
            ),
            vec![
                DOOR_SCRAP_ARRAY_DESTRUCT,
                INFO_SCRAP_ARRAY_DESTRUCT,
                INFO_SEARCH_DESTRUCT
            ]
        );
    }

    #[test]
    fn build_nav_mesh_info_path_adds_door_pairs_and_infos() {
        let mut w = world();
        let e = &mut w.e;
        returns(e, REQUEST_BYTE_98, 1);
        constructs_words(e, INFO_SCRAP_ARRAY_CONSTRUCT, &[0x10, 0x11, 0x12]);
        constructs_words(e, DOOR_SCRAP_ARRAY_CONSTRUCT, &[0xd0, 0, 0]);
        let linked = e.mem.alloc(12);
        write3(e, linked, [7.0, 8.0, 9.0]);
        returns(e, REFERENCE_LINKED_DOOR_POSITION, linked);
        let owner = e.mem.alloc(16);
        write3(e, owner + 4, [1.5, 2.5, 3.5]);
        returns(e, REFERENCE_GET_POSITION_OWNER, owner);
        e.register(VIRTUAL_NODE_SLOT, |_, a| ret(0x700 + a[1]));
        e.register(PLACEMENT_NEW, |_, a| ret(a[1]));
        let door_points = record_point(e, VIRTUAL_NODE_CONSTRUCT_INFO_POINT_DOOR, 2, 0);
        let far_points = record_point(e, VIRTUAL_NODE_CONSTRUCT_POINT_INFO, 1, 0);
        assert!(!path_builder_build_nav_mesh_info_path(e, w.this, 1, 2));
        assert_eq!(calls(e, INFO_SEARCH_RUN)[0][1], 0);
        let door = &calls(e, VIRTUAL_NODE_CONSTRUCT_INFO_POINT_DOOR)[0];
        assert_eq!((door[0], door[1], door[3]), (0x700, 0x10, 0xd0));
        assert_eq!(door_points.borrow()[0], [7.0, 8.0, 9.0]);
        let far = &calls(e, VIRTUAL_NODE_CONSTRUCT_POINT_INFO)[0];
        assert_eq!((far[0], far[2]), (0x701, 0x11));
        assert_eq!(far_points.borrow()[0], [1.5, 2.5, 3.5]);
        assert_eq!(
            calls(e, VIRTUAL_NODE_CONSTRUCT_INFO),
            vec![vec![0x702, 0x11]]
        );
        assert_eq!(calls(e, SOLUTION_ADD_LOCATION_NODE).len(), 2);
        assert_eq!(e.mem.u32(w.solution + VIRTUAL_NODES + 8), 3);
    }

    // -- 006c9900 .. 006c9a70 ---------------------------------------------

    #[test]
    fn fn_006c9900_is_the_field_at_0x5c() {
        let mut e = Engine::new();
        assert_eq!(fn_006c9900(&mut e, 0x1000), 0x105c);
    }

    #[test]
    fn virtual_node_adders_construct_in_place() {
        let mut w = world();
        let e = &mut w.e;
        e.register(VIRTUAL_NODE_SLOT, |_, a| ret(0x900 + 0x30 * a[1]));
        sequence(e, PLACEMENT_NEW, &[0x900, 0, 0x960]);
        assert_eq!(fn_006c9920(e, w.solution, 0x11), 0);
        assert_eq!(fn_006c99c0(e, w.solution, 0x22, 0x33), 1);
        assert_eq!(fn_006c9a70(e, w.solution, 0x44, 0x55, 0x66), 2);
        assert_eq!(calls(e, PLACEMENT_NEW)[1], vec![0x30, 0x930]);
        assert_eq!(
            calls(e, VIRTUAL_NODE_CONSTRUCT_INFO),
            vec![vec![0x900, 0x11]]
        );
        assert!(calls(e, VIRTUAL_NODE_CONSTRUCT_POINT_INFO).is_empty());
        assert_eq!(
            calls(e, VIRTUAL_NODE_CONSTRUCT_INFO_POINT_DOOR),
            vec![vec![0x960, 0x44, 0x55, 0x66]]
        );
    }

    #[test]
    fn fn_006c99c0_passes_point_and_info() {
        let mut w = world();
        let e = &mut w.e;
        returns(e, PLACEMENT_NEW, 0x880);
        assert_eq!(fn_006c99c0(e, w.solution, 0x22, 0x33), 0);
        assert_eq!(
            calls(e, VIRTUAL_NODE_CONSTRUCT_POINT_INFO),
            vec![vec![0x880, 0x22, 0x33]]
        );
    }

    #[test]
    fn fn_006c9a70_passes_info_point_and_door() {
        let mut w = world();
        let e = &mut w.e;
        returns(e, PLACEMENT_NEW, 0x880);
        assert_eq!(fn_006c9a70(e, w.solution, 1, 2, 3), 0);
        assert_eq!(
            calls(e, VIRTUAL_NODE_CONSTRUCT_INFO_POINT_DOOR),
            vec![vec![0x880, 1, 2, 3]]
        );
    }

    // -- 006c9b20 ----------------------------------------------------------

    #[test]
    fn build_teleport_door_path_without_spaces_returns_the_search_result() {
        let mut w = world();
        let e = &mut w.e;
        assert!(!path_builder_build_teleport_door_path(e, w.this));
        let search = &calls(e, TELEPORT_DOOR_SEARCH_BUILD_NODE_PATH)[0];
        assert_eq!(search[1], w.request + 0xc);
        assert_eq!(search[2], w.request + 0x34);
        assert_eq!(search[4..], [0, 0, w.request + 0x5c, 0]);
        assert_eq!(calls(e, SPACE_ARRAY_REALLOCATE)[0][1..], [10, 0]);
        assert_eq!(calls(e, LINK_ARRAY_REALLOCATE)[0][1..], [10, 0]);
        assert!(calls(e, LOCATION_CONSTRUCT_FROM_PARTS).is_empty());
        assert_eq!(
            order(e, &[TELEPORT_PATH_DESTRUCT, TELEPORT_DOOR_SEARCH_DESTRUCT]),
            vec![TELEPORT_PATH_DESTRUCT, TELEPORT_DOOR_SEARCH_DESTRUCT]
        );
    }

    #[test]
    fn build_teleport_door_path_builds_each_space() {
        let mut w = world();
        let e = &mut w.e;
        returns(e, TELEPORT_DOOR_SEARCH_BUILD_NODE_PATH, 1);
        e.register(TELEPORT_PATH_CONSTRUCT, |e, a| {
            let spaces = fill_array(e, a[0], 12, 2);
            e.mem.set_u32(spaces + 4, 0xa4);
            e.mem.set_u32(spaces + 8, 0xa8);
            let links = fill_array(e, a[0] + 0x10, 16, 1);
            e.mem.set_u32(links, 0xd00);
            write3(e, links + 4, [3.0, 3.0, 3.0]);
            ret(a[0])
        });
        let linked = e.mem.alloc(12);
        write3(e, linked, [2.0, 2.0, 2.0]);
        returns(e, REFERENCE_LINKED_DOOR_POSITION, linked);
        write3(e, w.request + 0xc + 4, [1.0, 1.0, 1.0]);
        write3(e, w.request + 0x34 + 4, [4.0, 4.0, 4.0]);
        returns(e, LOCATION_RESOLVE, 1);
        returns(e, INFO_SEARCH_RUN, 1);
        returns(e, PLACEMENT_NEW, 0x880);
        constructs_words(e, INFO_SCRAP_ARRAY_CONSTRUCT, &[0x55]);
        let points = record_point(e, LOCATION_CONSTRUCT_FROM_PARTS, 1, 0);
        assert!(path_builder_build_teleport_door_path(e, w.this));
        let parts = calls(e, LOCATION_CONSTRUCT_FROM_PARTS);
        assert_eq!(parts.len(), 4);
        assert_eq!(parts[0][2..], [0xa8, 0xa4]);
        assert_eq!(
            *points.borrow(),
            vec![
                [1.0, 1.0, 1.0],
                [2.0, 2.0, 2.0],
                [3.0, 3.0, 3.0],
                [4.0, 4.0, 4.0]
            ]
        );
        // The first space ends at the door (a door node), the last at the
        // destination (a plain node).
        assert_eq!(calls(e, VIRTUAL_NODE_CONSTRUCT_LOCATION_DOOR)[0][2], 0xd00);
        assert_eq!(calls(e, SOLUTION_ADD_LOCATION_NODE).len(), 3);
    }

    // -- 006c9fa0 .. 006ca0c0 ---------------------------------------------

    #[test]
    fn fn_006c9fa0_sets_incomplete_path() {
        let mut w = world();
        fn_006c9fa0(&mut w.e, w.solution, true);
        assert_eq!(w.e.mem.u8(w.solution + 0x40), 1);
        fn_006c9fa0(&mut w.e, w.solution, false);
        assert_eq!(w.e.mem.u8(w.solution + 0x40), 0);
    }

    #[test]
    fn fn_006c9fc0_finds_the_run_up_to_a_door() {
        let mut w = world();
        let buffer = virtual_nodes(&mut w, &[0, 0, 4, 0]);
        let e = &mut w.e;
        returns(e, LOCATION_GET_CELL, 0x900);
        returns(e, CELL_TEST_00450FF0, 1);
        let (first, last) = (e.mem.alloc(4), e.mem.alloc(4));
        assert!(fn_006c9fc0(e, w.this, 0, first, last));
        assert_eq!((e.mem.i32(first), e.mem.i32(last)), (0, 2));
        assert_eq!(calls(e, LOCATION_GET_CELL)[0], vec![node(buffer, 0) + 4]);
        returns(e, LOCATION_GET_CELL, 0);
        assert!(!fn_006c9fc0(e, w.this, 1, first, last));
    }

    #[test]
    fn node_flag_tests_read_bits_0_to_2() {
        let mut e = Engine::new();
        let n = e.mem.alloc(4);
        e.mem.set_u32(n, 7);
        assert!(fn_006ca830(&mut e, n) && fn_006cac70(&mut e, n) && fn_006ca0c0(&mut e, n));
        e.mem.set_u32(n, 0);
        assert!(!fn_006ca830(&mut e, n) && !fn_006cac70(&mut e, n) && !fn_006ca0c0(&mut e, n));
    }

    #[test]
    fn fn_006ca0c0_tests_bit_2() {
        let mut e = Engine::new();
        let n = e.mem.alloc(4);
        e.mem.set_u32(n, 4);
        assert!(fn_006ca0c0(&mut e, n));
        e.mem.set_u32(n, 3);
        assert!(!fn_006ca0c0(&mut e, n));
    }

    #[test]
    fn fn_006ca830_tests_bit_0() {
        let mut e = Engine::new();
        let n = e.mem.alloc(4);
        e.mem.set_u32(n, 1);
        assert!(fn_006ca830(&mut e, n));
        e.mem.set_u32(n, 6);
        assert!(!fn_006ca830(&mut e, n));
    }

    // -- 006ca0e0 ----------------------------------------------------------

    #[test]
    fn fn_006ca0e0_without_a_run_returns_true() {
        let mut w = world();
        virtual_nodes(&mut w, &[0]);
        assert!(fn_006ca0e0(&mut w.e, w.this, 0, 0));
        assert_eq!(calls(&w.e, SOLUTION_RESET), vec![vec![w.solution]]);
        assert!(calls(&w.e, SEGMENT_SCRAP_ARRAY_CONSTRUCT).is_empty());
    }

    #[test]
    fn fn_006ca0e0_builds_the_first_tangent() {
        let mut w = world();
        virtual_nodes(&mut w, &[0]);
        let e = &mut w.e;
        returns(e, LOCATION_GET_CELL, 0x900);
        returns(e, CELL_TEST_00450FF0, 1);
        returns(e, BOOL_ARRAY_FIND, u32::MAX);
        e.mem.set_u8(w.request + 0xa0, 1);
        e.mem.set_f32(w.request + 0x90, 0.5);
        returns(e, SOLUTION_CURRENT_NODE_COUNT, 2);
        e.register(SOLUTION_CURRENT_NODE, |_, a| ret(0x4000 + a[1]));
        returns_float(e, REQUEST_ACTOR_RADIUS, 2.0);
        e.set_global::<f64>(TANGENT_DISTANCE_01017A10, 1.0);
        // The copies get the points (0, 0, 0) and (3, 0, 0), then further
        // multiples of 3.
        let mut copies = 0u32;
        e.register_double(LOCATION_COPY_CONSTRUCT, move |e, a| {
            write3(e, a[0] + 4, [3.0 * copies as f32, 0.0, 0.0]);
            copies += 1;
            ret(a[0])
        });
        assert!(fn_006ca0e0(e, w.this, 0x77, 0));
        assert_eq!(e.mem.i32(w.solution + 0x18), 0);
        assert_eq!(e.mem.i32(w.solution + 0x1c), 0);
        let tangent = &calls(e, BUILD_FIRST_TANGENT)[0];
        assert_eq!(tangent[1], 0.5f32.to_bits());
        assert_eq!(tangent[4], 2.0f32.to_bits());
        let set = &calls(e, NODE_SET_POSITION)[0];
        assert_eq!((set[0], set[1]), (0x4000, tangent[0]));
        assert_eq!(calls(e, LOCATION_DESTRUCT).len(), 2);
        // Closer than the threshold: no tangent.
        e.set_global::<f64>(TANGENT_DISTANCE_01017A10, 100.0);
        e.call_log = Some(Vec::new());
        assert!(fn_006ca0e0(e, w.this, 0x77, 0));
        assert!(calls(e, BUILD_FIRST_TANGENT).is_empty());
    }

    // -- 006ca4c0, 006ca4e0, 006caa20, 006cd1d0 ---------------------------

    #[test]
    fn request_accessors_read_their_fields() {
        let mut w = world();
        let (e, r) = (&mut w.e, w.request);
        e.mem.set_u8(r + 0xa0, 1);
        e.mem.set_u8(r + 0xa1, 1);
        e.mem.set_u32(r + 0x90, 0x7f80_0001);
        assert!(fn_006ca4c0(e, r) && fn_006caa20(e, r) && !fn_006cd1d0(e, r));
        e.mem.set_u8(r + 0xa3, 1);
        assert!(fn_006cd1d0(e, r));
        // FLD quiets a signalling NaN.
        assert_eq!(fn_006ca4e0(e, r).to_bits(), 0x7fc0_0001);
    }

    #[test]
    fn fn_006ca4e0_reads_the_initial_heading() {
        let mut e = Engine::new();
        let r = e.mem.alloc(0xb0);
        e.mem.set_f32(r + 0x90, 1.25);
        assert_eq!(fn_006ca4e0(&mut e, r), 1.25);
    }

    #[test]
    fn fn_006ca4c0_reads_byte_a0() {
        let mut e = Engine::new();
        let r = e.mem.alloc(0xb0);
        assert!(!fn_006ca4c0(&mut e, r));
        e.mem.set_u8(r + 0xa0, 1);
        assert!(fn_006ca4c0(&mut e, r));
    }

    #[test]
    fn fn_006cd1d0_reads_byte_a3() {
        let mut e = Engine::new();
        let r = e.mem.alloc(0xb0);
        assert!(!fn_006cd1d0(&mut e, r));
        e.mem.set_u8(r + 0xa3, 1);
        assert!(fn_006cd1d0(&mut e, r));
    }

    // -- 006ca500 ----------------------------------------------------------

    type Segments = Rc<RefCell<Vec<(i32, i32, u8)>>>;

    fn record_segments(e: &mut Engine) -> Segments {
        let seen: Segments = Rc::new(RefCell::new(Vec::new()));
        let sink = seen.clone();
        e.register_double(SEGMENT_ARRAY_ADD, move |e, a| {
            sink.borrow_mut()
                .push((e.mem.i32(a[1]), e.mem.i32(a[1] + 4), e.mem.u8(a[1] + 8)));
            ret(0)
        });
        seen
    }

    #[test]
    fn fn_006ca500_cuts_navmesh_runs_at_unlinked_infos() {
        let mut w = world();
        let buffer = virtual_nodes(&mut w, &[0, 1, 1]);
        let e = &mut w.e;
        e.mem.set_i32(w.solution + 0x18, 0);
        e.mem.set_i32(w.solution + 0x1c, 2);
        e.register(BOOL_SCRAP_ARRAY_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 8, a[1]);
            ret(a[0])
        });
        sequence(e, BOOL_ARRAY_FIND, &[1, u32::MAX]);
        e.register(LOCATION_GET_BEST_NAVMESH_INFO, |_, a| ret(0x5000 + a[0]));
        returns(e, ARRAY_FIND, u32::MAX);
        let segments = record_segments(e);
        assert!(fn_006ca500(e, w.this, 0x6000));
        assert_eq!(*segments.borrow(), vec![(0, 0, 0), (1, 1, 1), (2, 2, 1)]);
        assert_eq!(calls(e, BOOL_SCRAP_ARRAY_CONSTRUCT)[0][1..], [3, 0, 0x5ca7]);
        assert_eq!(calls(e, ARRAY_CLEAR_008454F0)[0], vec![0x6000, 0]);
        let info = 0x5000 + node(buffer, 1) + 4;
        assert_eq!(calls(e, ARRAY_FIND)[0][0], info + 0x24);
        assert_eq!(calls(e, ARRAY_FIND)[1][0], info + 0x34);
        assert_eq!(calls(e, BOOL_SCRAP_ARRAY_DESTRUCT).len(), 1);
    }

    #[test]
    fn fn_006ca500_fails_without_navmesh_info() {
        let mut w = world();
        virtual_nodes(&mut w, &[1, 1]);
        let e = &mut w.e;
        e.mem.set_i32(w.solution + 0x1c, 1);
        e.register(BOOL_SCRAP_ARRAY_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 8, a[1]);
            ret(a[0])
        });
        sequence(e, BOOL_ARRAY_FIND, &[0, u32::MAX]);
        let segments = record_segments(e);
        assert!(!fn_006ca500(e, w.this, 0x6000));
        assert!(segments.borrow().is_empty());
        assert_eq!(calls(e, BOOL_SCRAP_ARRAY_DESTRUCT).len(), 1);
    }

    // -- 006ca850 ----------------------------------------------------------

    fn segments_array(e: &mut Engine, list: &'static [(u32, u32, u8)]) {
        e.register_double(SEGMENT_SCRAP_ARRAY_CONSTRUCT, move |e, a| {
            let buffer = fill_array(e, a[0], 12, list.len() as u32);
            for (i, (first, last, navmeshes)) in list.iter().enumerate() {
                let s = buffer + 12 * i as u32;
                e.mem.set_u32(s, *first);
                e.mem.set_u32(s + 4, *last);
                e.mem.set_u8(s + 8, *navmeshes);
            }
            ret(a[0])
        });
    }

    #[test]
    fn fn_006ca850_builds_plain_segments_and_fails_a_navmesh_one() {
        let mut w = world();
        virtual_nodes(&mut w, &[0, 0]);
        let e = &mut w.e;
        returns(e, BOOL_ARRAY_FIND, u32::MAX);
        segments_array(e, &[(0, 0, 0), (1, 1, 0)]);
        assert!(fn_006ca850(e, w.this, 0));
        assert_eq!(calls(e, SOLUTION_ADD_CURRENT_NODE).len(), 2);
        assert_eq!(
            calls(e, SEGMENT_SCRAP_ARRAY_CONSTRUCT)[0][1..],
            [0, 0, 0x5ca7]
        );
        assert_eq!(calls(e, SEGMENT_SCRAP_ARRAY_DESTRUCT).len(), 1);

        // A navmesh segment whose build fails (both ends fail, the straight
        // path is off): false, unless the request allows it (+0xA1), then
        // it is built as a plain segment.
        let mut w = world();
        virtual_nodes(&mut w, &[0]);
        let e = &mut w.e;
        returns(e, BOOL_ARRAY_FIND, u32::MAX);
        e.set_global::<f64>(SHORT_SPAN_01028338, 1.0);
        segments_array(e, &[(0, 0, 1)]);
        assert!(!fn_006ca850(e, w.this, 0));
        assert!(calls(e, SOLUTION_ADD_CURRENT_NODE).is_empty());
        e.mem.set_u8(w.request + 0xa1, 1);
        assert!(fn_006ca850(e, w.this, 0));
        assert_eq!(calls(e, SOLUTION_ADD_CURRENT_NODE).len(), 1);
    }

    #[test]
    fn fn_006caa20_reads_byte_a1() {
        let mut e = Engine::new();
        let r = e.mem.alloc(0xb0);
        assert!(!fn_006caa20(&mut e, r));
        e.mem.set_u8(r + 0xa1, 2);
        assert!(fn_006caa20(&mut e, r));
    }

    // -- 006caa40 / 006cac90 ----------------------------------------------

    #[test]
    fn fn_006caa40_falls_back_to_the_node_location() {
        let mut w = world();
        let buffer = virtual_nodes(&mut w, &[2, 0]);
        let e = &mut w.e;
        let out = e.mem.alloc(0x10);
        // Node 1 after a bit-1 node: the edge location fails (no navmesh),
        // and node 1 lacks bit 1: 2.
        assert_eq!(fn_006caa40(e, w.this, 1, out, 0x3000), 2);
        assert_eq!(calls(e, LOCATION_CONSTRUCT).len(), 1);
        assert_eq!(calls(e, LOCATION_DESTRUCT).len(), 1);
        // Node 0 with bit 1 and a valid navmesh info: appended.
        returns(e, LOCATION_RESOLVE_NAVMESH_INFO, 1);
        returns(e, LOCATION_HAS_NAVMESH_INFO, 1);
        fill_array(e, out, 0x28, 0);
        assert_eq!(fn_006caa40(e, w.this, 0, out, 0x3000), 1);
        let location = node(buffer, 0) + 4;
        assert_eq!(calls(e, LOCATION_ASSIGN).last().unwrap()[1], location);
        // No valid info, the edge search says 2, the ray-cast setting is
        // off and no triangle is found: 2.
        returns(e, LOCATION_HAS_NAVMESH_INFO, 0);
        returns(e, FIND_EDGES, 2);
        assert_eq!(fn_006caa40(e, w.this, 0, out, 0x3000), 2);
        assert_eq!(
            calls(e, FIND_EDGES).last().unwrap(),
            &vec![location, 10, 0x3000]
        );
    }

    #[test]
    fn fn_006cac90_takes_an_edge_point_within_reach() {
        let mut w = world();
        let buffer = virtual_nodes(&mut w, &[2]);
        let e = &mut w.e;
        write3(e, node(buffer, 0) + 8, [8.0, 0.0, 0.0]);
        returns(e, LOCATION_RESOLVE_NAVMESH_INFO, 1);
        returns(e, FIND_EDGES, 2);
        returns_float(e, REQUEST_TARGET_RADIUS, 5.0);
        e.set_global::<f64>(HEIGHT_LOW_0106B9F0, -1.0);
        e.set_global::<f64>(HEIGHT_HIGH_0104ED58, 1.0);
        let edges = e.mem.alloc(0x10);
        let edge = fill_array(e, edges, EDGE_LOCATION_SIZE, 1);
        e.mem.set_u32(edge, 0x1e0);
        e.mem.set_u16(edge + 4, 7);
        write3(e, edge + 0xc, [10.0, 0.0, 0.0]);
        let out = e.mem.alloc(0x10);
        fill_array(e, out, 0x28, 0);
        assert_eq!(fn_006cac90(e, w.this, 0, out, edges), 1);
        let made = &calls(e, LOCATION_CONSTRUCT_FROM_MESH_POINT)[0];
        assert_eq!(made[1..], [edge + 0xc, 0x1e0, 7]);
        // Out of reach: the path to a valid triangle (setting off) fails.
        returns_float(e, REQUEST_TARGET_RADIUS, 1.0);
        assert_eq!(fn_006cac90(e, w.this, 0, out, edges), 2);
        assert_eq!(calls(e, LOCATION_CONSTRUCT_FROM_MESH_POINT).len(), 1);
    }

    #[test]
    fn fn_006cac70_tests_bit_1() {
        let mut e = Engine::new();
        let n = e.mem.alloc(4);
        e.mem.set_u32(n, 2);
        assert!(fn_006cac70(&mut e, n));
        e.mem.set_u32(n, 5);
        assert!(!fn_006cac70(&mut e, n));
    }

    // -- 006cb050 ----------------------------------------------------------

    #[test]
    fn build_path_to_valid_nav_mesh_triangle_ray_casts() {
        let mut w = world();
        let e = &mut w.e;
        e.mem.set_u8(SETTING_RAY_CAST_011D7404, 1);
        let out = e.mem.alloc(0x10);
        fill_array(e, out, 0x28, 0);
        returns(e, FIND_EDGES, 1);
        assert_eq!(
            path_builder_build_path_to_valid_nav_mesh_triangle(e, w.this, 0x123, out, true, 0x77),
            1
        );
        assert_eq!(calls(e, LOCATION_ASSIGN)[0][1], 0x123);
        returns(e, FIND_EDGES, 0);
        assert_eq!(
            path_builder_build_path_to_valid_nav_mesh_triangle(e, w.this, 0x123, out, true, 0x77),
            2
        );
        // 2: a local ray-cast request, run until it stops returning 0.
        returns(e, FIND_EDGES, 2);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = seen.clone();
        e.register_double(RAY_CAST_SEARCH_BUILD_PATH, move |e, a| {
            sink.borrow_mut()
                .push((e.mem.u32(a[1]), e.mem.u32(a[1] + 4)));
            let count = sink.borrow().len();
            ret(u32::from(count >= 3))
        });
        assert_eq!(
            path_builder_build_path_to_valid_nav_mesh_triangle(e, w.this, 0x123, out, true, 0x77),
            1
        );
        assert_eq!(*seen.borrow(), vec![(0, w.request); 3]);
        assert_eq!(calls(e, LOCATION_ARRAY_COPY_TO)[0][1], out);
        assert_eq!(calls(e, EDGE_ARRAY_COPY_TO)[0][0], 0x77);
        assert_eq!(calls(e, LOCATION_ASSIGN).last().unwrap()[1], 0x123);
        assert_eq!(
            order(
                e,
                &[
                    RAY_CAST_SEARCH_DESTRUCT,
                    LOCATION_ARRAY_DESTRUCT,
                    EDGE_ARRAY_DESTRUCT
                ]
            ),
            vec![
                RAY_CAST_SEARCH_DESTRUCT,
                LOCATION_ARRAY_DESTRUCT,
                EDGE_ARRAY_DESTRUCT
            ]
        );
        // On the tasklet thread the path manager does it.
        e.register(THREAD_CHECK, |_, _| ret(1));
        returns(e, PATH_MANAGER_INSTANCE, 0xbeef);
        returns(e, PATH_MANAGER_GET_RAY_CAST_PATH_TO_NAV_MESH, 7);
        assert_eq!(
            path_builder_build_path_to_valid_nav_mesh_triangle(e, w.this, 0x123, out, false, 0x77),
            7
        );
        assert_eq!(
            calls(e, PATH_MANAGER_GET_RAY_CAST_PATH_TO_NAV_MESH)[0],
            vec![0xbeef, w.request, 0x77, 0x123, out, 0]
        );
    }

    #[test]
    fn build_path_to_valid_nav_mesh_triangle_without_ray_casts() {
        let mut w = world();
        let e = &mut w.e;
        let location = e.mem.alloc(0x28);
        let out = e.mem.alloc(0x10);
        assert_eq!(
            path_builder_build_path_to_valid_nav_mesh_triangle(e, w.this, location, out, true, 0),
            2
        );
        // The by-value copy is made from the location and destroyed by
        // fn_006cdb40; the closest location is destroyed last.
        let copy = &calls(e, LOCATION_COPY_CONSTRUCT)[0];
        assert_eq!(copy[1], location);
        let destroyed = calls(e, LOCATION_DESTRUCT);
        assert_eq!(destroyed.len(), 2);
        assert_eq!(destroyed[0][0], copy[0]);
    }

    // -- 006cb3a0 .. 006cb4c0 ---------------------------------------------

    #[test]
    fn ray_cast_path_request_constructs_its_members() {
        let mut w = world();
        let e = &mut w.e;
        assert_eq!(
            ray_cast_path_request_ray_cast_path_request(e, 0x1000),
            0x1000
        );
        assert_eq!(
            calls(e, LOCATION_CONSTRUCT),
            vec![vec![0x1008], vec![0x1030]]
        );
        assert_eq!(calls(e, EDGE_ARRAY_CONSTRUCT), vec![vec![0x1058]]);
    }

    #[test]
    fn fn_006cb410_destroys_in_reverse_order() {
        let mut w = world();
        fn_006cb410(&mut w.e, 0x2000);
        assert_eq!(calls(&w.e, EDGE_ARRAY_DESTRUCT), vec![vec![0x2058]]);
        assert_eq!(
            calls(&w.e, LOCATION_DESTRUCT),
            vec![vec![0x2030], vec![0x2008]]
        );
        assert_eq!(
            order(&w.e, &[EDGE_ARRAY_DESTRUCT, LOCATION_DESTRUCT]),
            vec![EDGE_ARRAY_DESTRUCT, LOCATION_DESTRUCT, LOCATION_DESTRUCT]
        );
    }

    #[test]
    fn fn_006cb480_constructs_the_node_array() {
        let mut w = world();
        let e = &mut w.e;
        let path = e.mem.alloc(0x18);
        assert_eq!(fn_006cb480(e, path), path);
        assert_eq!(e.mem.u32(path + 8), LOCATION_ARRAY_VTABLE);
        assert_eq!(calls(e, LOCATION_ARRAY_INIT), vec![vec![path + 8, 0, 0]]);
    }

    #[test]
    fn fn_006cb4a0_destroys_the_node_array() {
        let mut w = world();
        fn_006cb4a0(&mut w.e, 0x3000);
        assert_eq!(calls(&w.e, LOCATION_ARRAY_DESTRUCT), vec![vec![0x3008]]);
    }

    #[test]
    fn build_straight_path_using_ray_casts() {
        let mut w = world();
        let e = &mut w.e;
        assert_eq!(
            path_builder_build_straight_path_using_ray_casts(e, w.this, 0x10, 0x20, 0x30),
            2
        );
        assert!(calls(e, RAY_CAST_SEARCH_BUILD_PATH).is_empty());
        e.mem.set_u8(SETTING_RAY_CAST_011D7404, 1);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = seen.clone();
        e.register_double(RAY_CAST_SEARCH_BUILD_PATH, move |e, a| {
            sink.borrow_mut()
                .push((e.mem.u32(a[1]), e.mem.u32(a[1] + 4)));
            ret(1)
        });
        assert_eq!(
            path_builder_build_straight_path_using_ray_casts(e, w.this, 0x10, 0x20, 0x30),
            1
        );
        assert_eq!(*seen.borrow(), vec![(2, w.request)]);
        let assigns = calls(e, LOCATION_ASSIGN);
        assert_eq!((assigns[0][1], assigns[1][1]), (0x10, 0x20));
        assert_eq!(assigns[1][0], assigns[0][0] + 0x28);
        assert_eq!(calls(e, LOCATION_ARRAY_COPY_TO)[0][1], 0x30);
        e.register(THREAD_CHECK, |_, _| ret(1));
        returns(e, PATH_MANAGER_GET_RAY_CAST_PATH, 1);
        assert_eq!(
            path_builder_build_straight_path_using_ray_casts(e, w.this, 0x10, 0x20, 0x30),
            1
        );
        assert_eq!(
            calls(e, PATH_MANAGER_GET_RAY_CAST_PATH)[0][1..],
            [w.request, 0x10, 0x20, 0x30]
        );
    }

    // -- 006cb6a0 / 006cb900 ----------------------------------------------

    #[test]
    fn fn_006cb6a0_pulls_the_edge_point_towards_the_centre() {
        let mut w = world();
        virtual_nodes(&mut w, &[0]);
        let e = &mut w.e;
        let edges = e.mem.alloc(0x10);
        let edge = fill_array(e, edges, EDGE_LOCATION_SIZE, 1);
        e.mem.set_u32(edge, 0x1e0);
        e.mem.set_u16(edge + 4, 9);
        e.mem.set_u32(edge + 8, 2);
        e.register(EDGE_HANDLE_COPY, |e, a| {
            for i in 0..3 {
                let word = e.mem.u32(a[1] + 4 * i);
                e.mem.set_u32(a[0] + 4 * i, word);
            }
            ret(a[0])
        });
        returns(e, NAVMESH_INFO_LOAD_HOLDER, 1);
        e.register(CLOSEST_ON_EDGE_006B9C20, |e, a| {
            write3(e, a[0], [2.0, 0.0, 0.0]);
            ret(a[0])
        });
        e.register(NAVMESH_GET_CENTER, |e, a| {
            write3(e, a[1], [4.0, 0.0, 0.0]);
            ret(a[1])
        });
        e.set_global::<f32>(CENTER_WEIGHT_0101E2BC, 0.5);
        returns_float(e, REQUEST_TARGET_RADIUS, 3.0);
        e.set_global::<f64>(HEIGHT_LOW_0106B9F0, -1.0);
        e.set_global::<f64>(HEIGHT_HIGH_0104ED58, 1.0);
        let points = record_point(e, LOCATION_CONSTRUCT_FROM_MESH_POINT, 1, 0);
        let out = e.mem.alloc(0x10);
        fill_array(e, out, 0x28, 0);
        assert!(fn_006cb6a0(e, w.this, 0, edges, out));
        assert_eq!(points.borrow()[0], [3.0, 0.0, 0.0]);
        assert_eq!(
            calls(e, LOCATION_CONSTRUCT_FROM_MESH_POINT)[0][2..],
            [0x1e0, 9]
        );
        assert_eq!(calls(e, LOCATION_ARRAY_CLEAR)[0], vec![out, 0]);
        assert_eq!(calls(e, NAVMESH_EDGE_POINTS)[0][1..3], [9, 2]);
        // Out of reach: false.
        returns_float(e, REQUEST_TARGET_RADIUS, 1.0);
        assert!(!fn_006cb6a0(e, w.this, 0, edges, out));
        assert_eq!(calls(e, NAV_HOLDER_RELEASE).len(), 2);
    }

    #[test]
    fn fn_006cb900_copies_an_edge_location() {
        let mut w = world();
        let e = &mut w.e;
        let (from, to) = (e.mem.alloc(0x18), e.mem.alloc(0x18));
        write3(e, from + 0xc, [1.0, 2.0, 3.0]);
        assert_eq!(fn_006cb900(e, to, from), to);
        assert_eq!(read3(e, to + 0xc), [1.0, 2.0, 3.0]);
        assert_eq!(calls(e, EDGE_HANDLE_COPY), vec![vec![to, from]]);
        assert_eq!(calls(e, POINT3_DEFAULT_CONSTRUCT), vec![vec![to + 0xc]]);
    }

    // -- 006cb950 ----------------------------------------------------------

    #[test]
    fn fn_006cb950_failing_ends_and_a_straight_path() {
        let mut w = world();
        virtual_nodes(&mut w, &[0]);
        let e = &mut w.e;
        e.set_global::<f64>(SHORT_SPAN_01028338, 1.0);
        assert!(!fn_006cb950(e, w.this, 0, 0));
        assert_eq!(calls(e, EDGE_ARRAY_DESTRUCT).len(), 2);
        assert_eq!(calls(e, LOCATION_ARRAY_DESTRUCT).len(), 3);
        // With ray casts the straight path is taken: its two locations
        // become current nodes.
        e.mem.set_u8(SETTING_RAY_CAST_011D7404, 1);
        returns(e, RAY_CAST_SEARCH_BUILD_PATH, 1);
        let locations = e.mem.alloc(0x50);
        e.register_double(LOCATION_ARRAY_COPY_TO, move |e, a| {
            e.mem.set_u32(a[1] + 4, locations);
            e.mem.set_u32(a[1] + 8, 2);
            ret(0)
        });
        e.call_log = Some(Vec::new());
        assert!(fn_006cb950(e, w.this, 0, 0));
        let added = calls(e, SOLUTION_ADD_CURRENT_NODE);
        assert_eq!(added.len(), 2);
        assert_eq!(added[1], vec![w.solution, locations + 0x28, POINT_011F426C]);
    }

    #[test]
    fn fn_006cb950_runs_the_navmesh_path() {
        let mut w = world();
        virtual_nodes(&mut w, &[2, 2]);
        let e = &mut w.e;
        // Both ends are their node locations; the navmesh path (a direct
        // line) succeeds.
        returns(e, LOCATION_RESOLVE_NAVMESH_INFO, 1);
        returns(e, LOCATION_HAS_NAVMESH_INFO, 1);
        e.register(LOCATION_ARRAY_ADD_SLOT, |e, a| {
            let size = e.mem.u32(a[0] + 8);
            if e.mem.u32(a[0] + 4) == 0 {
                let buffer = e.mem.alloc(0x28 * 4);
                e.mem.set_u32(a[0] + 4, buffer);
            }
            e.mem.set_u32(a[0] + 8, size + 1);
            ret(size)
        });
        e.mem.set_u8(SETTING_DIRECT_LINE_011D73D8, 1);
        returns(e, CHECK_LINE_OF_SIGHT, 1);
        returns_float(e, TES_GROUND_HEIGHT, 1.0e9);
        assert!(fn_006cb950(e, w.this, 0, 1));
        // The direct line adds the start; the end location follows.
        assert_eq!(calls(e, SOLUTION_ADD_CURRENT_NODE).len(), 2);
        assert_eq!(calls(e, SOLUTION_INCOMPLETE).len(), 1);
    }

    // -- 006cc560 / 006cc5a0 ----------------------------------------------

    #[test]
    fn pathing_node_set_teleport_door_moves_to_the_door() {
        let mut w = world();
        let e = &mut w.e;
        let n = e.mem.alloc(0x3c);
        e.mem.set_u32(n, 0x10);
        returns(e, REFERENCE_LINKED_DOOR_POSITION, 0x4444);
        pathing_node_set_teleport_door(e, n, 0xd00);
        assert_eq!(e.mem.u32(n + 0x38), 0xd00);
        assert_eq!(e.mem.u32(n), 0x13);
        assert_eq!(calls(e, REFERENCE_LINKED_DOOR_POSITION), vec![vec![0xd00]]);
        assert_eq!(calls(e, LOCATION_SET_POSITION), vec![vec![n + 4, 0x4444]]);
    }

    #[test]
    fn fn_006cc5a0_sets_and_clears_bit_2() {
        let mut e = Engine::new();
        let n = e.mem.alloc(4);
        e.mem.set_u32(n, 0x101);
        fn_006cc5a0(&mut e, n, true);
        assert_eq!(e.mem.u32(n), 0x105);
        fn_006cc5a0(&mut e, n, false);
        assert_eq!(e.mem.u32(n), 0x101);
    }

    // -- 006cc5e0 ----------------------------------------------------------

    #[test]
    fn fn_006cc5e0_direct_line_and_attempts() {
        let mut w = world();
        virtual_nodes(&mut w, &[0]);
        let e = &mut w.e;
        let (start, goal) = (e.mem.alloc(0x28), e.mem.alloc(0x28));
        // Direct line: free and below ground, 0.
        e.mem.set_u8(SETTING_DIRECT_LINE_011D73D8, 1);
        returns(e, CHECK_LINE_OF_SIGHT, 1);
        returns_float(e, TES_GROUND_HEIGHT, 1.0e9);
        assert_eq!(fn_006cc5e0(e, w.this, start, goal, 0, 0), 0);
        assert!(calls(e, TRIANGLE_ARRAY_A_CONSTRUCT).is_empty());
        // One attempt whose search fails with no triangles: 0, nothing
        // marked.
        e.mem.set_u8(SETTING_DIRECT_LINE_011D73D8, 0);
        returns(e, REQUEST_WORD_A8, 1);
        e.call_log = Some(Vec::new());
        assert_eq!(fn_006cc5e0(e, w.this, start, goal, 0, 0), 0);
        assert_eq!(calls(e, CLAMP_INTEGER)[0][1..], [1, 10]);
        assert_eq!(
            calls(e, ARRAY_CLEAR_008454F0)[0],
            vec![w.this.addr() + 0x14, 0]
        );
        assert_eq!(
            calls(e, NAVMESH_HOLDER_ARRAY_CLEAR)[0],
            vec![w.this.addr() + 0x24, 0]
        );
        assert_eq!(calls(e, LOCATION_COPY_CONSTRUCT)[0][1], start);
        assert_eq!(calls(e, LOCATION_COPY_CONSTRUCT)[1][1], goal);
        assert!(calls(e, POV_SMOOTHER_RUN).is_empty());
        assert_eq!(calls(e, TRIANGLE_ARRAY_A_DESTRUCT).len(), 2);
        // The search and the smoother succeed: 0.
        returns(e, LOCATION_GET_NAVMESH_AND_TRIANGLE, 1);
        returns(e, NAVMESH_SEARCH_RUN, 1);
        returns(e, POV_SMOOTHER_RUN, 1);
        e.call_log = Some(Vec::new());
        assert_eq!(fn_006cc5e0(e, w.this, start, goal, 0, 0), 0);
        assert_eq!(calls(e, POV_SMOOTHER_RUN).len(), 1);
        // The smoother fails on the last attempt with byte +0xA3: the
        // triangles become nodes and the path is incomplete, 1.
        returns(e, POV_SMOOTHER_RUN, 0);
        e.mem.set_u8(w.request + 0xa3, 1);
        e.register(TRIANGLE_ARRAY_C_CONSTRUCT, |e, a| {
            fill_array(e, a[0], 0x14, 2);
            ret(a[0])
        });
        e.call_log = Some(Vec::new());
        assert_eq!(fn_006cc5e0(e, w.this, start, goal, 0, 0), 1);
        assert_eq!(e.mem.u8(w.solution + 0x40), 1);
        assert_eq!(calls(e, LOCATION_CONSTRUCT_FROM_POINT_AND_INFO).len(), 2);
        assert_eq!(calls(e, SOLUTION_ADD_CURRENT_NODE).len(), 2);
    }

    // -- 006cd1f0 ----------------------------------------------------------

    #[test]
    fn fn_006cd1f0_checks_the_line_and_both_sides() {
        let mut w = world();
        let e = &mut w.e;
        returns_float(e, REQUEST_ACTOR_RADIUS, 2.0);
        e.mem.set_f32(SETTING_RADIUS_011D6EF0, 1.0);
        e.register(POINT3_UNIT_CROSS, |_, a| ret(a[1]));
        let filters = Rc::new(RefCell::new(Vec::new()));
        let sink = filters.clone();
        e.register_double(CHECK_LINE_OF_SIGHT, move |e, a| {
            if a[3] != 0 {
                sink.borrow_mut()
                    .push((e.mem.u32(a[3]), e.mem.u8(a[3] + 4)));
            }
            ret(0)
        });
        assert!(!fn_006cd1f0(e, w.this, 0x100, 0x200));
        assert_eq!(
            *filters.borrow(),
            vec![(CHECK_LINE_OF_SIGHT_FILTER_WATER_VTABLE, 1)]
        );
        // Clear line, above ground, both sides clear: the start is added.
        returns(e, CHECK_LINE_OF_SIGHT, 1);
        returns_float(e, TES_GROUND_HEIGHT, -1.0e9);
        let (start, goal) = (e.mem.alloc(0x28), e.mem.alloc(0x28));
        e.call_log = Some(Vec::new());
        assert!(fn_006cd1f0(e, w.this, start, goal));
        assert_eq!(
            calls(e, SOLUTION_ADD_CURRENT_NODE),
            vec![vec![w.solution, start, POINT_011F426C]]
        );
        let sides = calls(e, LOCATION_CONSTRUCT_FROM_POINT_AND_LOCATION);
        assert_eq!(sides.len(), 4);
        assert_eq!(
            (sides[0][2], sides[1][2], sides[2][2], sides[3][2]),
            (start, goal, start, goal)
        );
        assert_eq!(calls(e, LOCATION_DESTRUCT).len(), 4);
        assert_eq!(calls(e, POINT3_UNIT_CROSS)[0][2], AXIS_011A9484);
        // The second side blocked: false, all four destroyed.
        sequence(e, CHECK_LINE_OF_SIGHT, &[1, 1, 0]);
        e.call_log = Some(Vec::new());
        assert!(!fn_006cd1f0(e, w.this, start, goal));
        assert!(calls(e, SOLUTION_ADD_CURRENT_NODE).is_empty());
        assert_eq!(calls(e, LOCATION_DESTRUCT).len(), 4);
    }

    // -- 006cd5a0 .. 006cd880 ---------------------------------------------

    #[test]
    fn fn_006cd5a0_stores_the_water_filter_vtable() {
        let mut e = Engine::new();
        let f = e.mem.alloc(8);
        assert_eq!(fn_006cd5a0(&mut e, f), f);
        assert_eq!(e.mem.u32(f), CHECK_LINE_OF_SIGHT_FILTER_WATER_VTABLE);
    }

    #[test]
    fn fn_006cd650_stores_the_base_vtable() {
        let mut e = Engine::new();
        let f = e.mem.alloc(4);
        assert_eq!(fn_006cd650(&mut e, f), f);
        assert_eq!(e.mem.u32(f), CHECK_LINE_OF_SIGHT_FILTER_VTABLE);
    }

    #[test]
    fn can_cross_edge_rejects_flagged_triangles() {
        let mut w = world();
        let e = &mut w.e;
        let filter = e.new_object::<CheckLineOfSightFilterWater>();
        returns(e, NAVMESH_GET_TRIANGLE, 0x7700);
        let cross = |e: &mut Engine| {
            check_line_of_sight_filter_water_can_cross_edge(e, filter, 1, 2, 3, 0x6600, 5, 6)
        };
        assert!(cross(e));
        assert_eq!(calls(e, NAVMESH_GET_TRIANGLE)[0], vec![0x6600, 5]);
        assert_eq!(calls(e, TRIANGLE_FLAGS_SET)[0], vec![0x7700, 0x8000_0000]);
        assert_eq!(calls(e, TRIANGLE_FLAGS_SET)[1], vec![0x7700, 0x1000]);
        returns(e, TRIANGLE_HAS_FLAG_0X10, 1);
        assert!(cross(e));
        e.set(filter, CheckLineOfSightFilterWater::bIsLargeCreature, true);
        assert!(!cross(e));
        returns(e, TRIANGLE_HAS_FLAG_0X10, 0);
        sequence(e, TRIANGLE_FLAGS_SET, &[0, 1]);
        assert!(!cross(e));
        returns(e, TRIANGLE_TEST_006A5EC0, 1);
        assert!(!cross(e));
    }

    #[test]
    fn fn_006cd670_runs_the_navmesh_search() {
        let mut w = world();
        let e = &mut w.e;
        assert!(!fn_006cd670(e, w.this, 0x100, 0x200, 0xa, 0xc, 0xd, 0, 0));
        assert_eq!(calls(e, NAV_HOLDER_RELEASE).len(), 1);
        e.register(LOCATION_GET_NAVMESH_AND_TRIANGLE, |e, a| {
            e.mem.set_u32(a[1], 0x9000 + a[0]);
            e.mem.set_u16(a[2], (a[0] / 0x100) as u16);
            ret(1)
        });
        type Seen = Rc<RefCell<Vec<(u32, u16, u32, u16, u32)>>>;
        let handles: Seen = Rc::new(RefCell::new(Vec::new()));
        let sink = handles.clone();
        e.register_double(NAVMESH_SEARCH_RUN, move |e, a| {
            sink.borrow_mut().push((
                e.mem.u32(a[2]),
                e.mem.u16(a[2] + 4),
                e.mem.u32(a[3]),
                e.mem.u16(a[3] + 4),
                e.mem.u32(a[0] + SEARCH_WORD_2084),
            ));
            ret(1)
        });
        e.call_log = Some(Vec::new());
        assert!(fn_006cd670(e, w.this, 0x100, 0x200, 0xa, 0xc, 0xd, 0, 0));
        assert_eq!(*handles.borrow(), vec![(0x9100, 1, 0x9200, 2, 0xa)]);
        let run = &calls(e, NAVMESH_SEARCH_RUN)[0];
        assert_eq!(
            (run[1], run[4], run[5], run[6]),
            (w.request, 0xc, 0xd, w.this.addr() + 0x14)
        );
        assert_eq!(
            order(e, &[NAVMESH_SEARCH_DESTRUCT, NAV_HOLDER_RELEASE]),
            vec![
                NAVMESH_SEARCH_DESTRUCT,
                NAV_HOLDER_RELEASE,
                NAV_HOLDER_RELEASE
            ]
        );
    }

    #[test]
    fn fn_006cd880_stores_the_search_word() {
        let mut e = Engine::new();
        let s = e.mem.alloc(0x2090);
        fn_006cd880(&mut e, s, 0x1234);
        assert_eq!(e.mem.u32(s + 0x2084), 0x1234);
    }

    // -- 006cd8a0 ----------------------------------------------------------

    #[test]
    fn fn_006cd8a0_picks_search_or_smoother() {
        let mut w = world();
        let e = &mut w.e;
        let (start, goal) = (e.mem.alloc(0x28), e.mem.alloc(0x28));
        let triangles = e.mem.alloc(0x10);
        let handle = fill_array(e, triangles, 0x14, 1);
        e.mem.set_u32(handle, 0x6600);
        e.mem.set_u16(handle + 4, 3);
        // Neither setting: the POV smoother.
        returns(e, POV_SMOOTHER_RUN, 1);
        assert!(fn_006cd8a0(e, w.this, start, goal, 0xa, triangles, 0xd));
        let run = &calls(e, POV_SMOOTHER_RUN)[0];
        assert_eq!((run[1], run[2], run[3]), (w.request, triangles, 0xd));
        assert_eq!(run[6..], [w.solution, 0xa, w.this.addr() + 0x14]);
        // The smoother setting: the path smoother.
        e.mem.set_u8(SETTING_SMOOTHER_011D7434, 1);
        assert!(!fn_006cd8a0(e, w.this, start, goal, 0xa, triangles, 0xd));
        assert_eq!(calls(e, PATH_SMOOTHER_RUN)[0][6..], [w.solution, 0xa, 1]);
        // The POV search for a large actor.
        e.mem.set_u8(SETTING_POV_SEARCH_011D7444, 1);
        returns_float(e, REQUEST_ACTOR_RADIUS, 2.0);
        e.set_global::<f64>(LARGE_RADIUS_0106C948, 1.0);
        returns(e, FAST_TRIANGLE_HANDLE_CONSTRUCT, 0x4321);
        returns(e, POV_SEARCH_RUN, 1);
        assert!(fn_006cd8a0(e, w.this, start, goal, 0xa, triangles, 0xd));
        assert_eq!(
            calls(e, FAST_TRIANGLE_HANDLE_CONSTRUCT)[0][1..],
            [0x6600, 3]
        );
        assert_eq!(
            calls(e, INFO_HANDLE_ARRAY_CONSTRUCT)[0][1..],
            [1, 0, 0x5ca7]
        );
        assert_eq!(calls(e, INFO_HANDLE_ARRAY_ADD)[0][1], 0x4321);
        let search = &calls(e, POV_SEARCH_RUN)[0];
        assert_eq!(
            (search[1], search[2], search[4], search[5]),
            (start, goal, w.request, w.solution)
        );
    }

    // -- 006cdb40 ----------------------------------------------------------

    #[test]
    fn fn_006cdb40_keeps_the_closest_triangle_point() {
        let mut w = world();
        let e = &mut w.e;
        let location = e.mem.alloc(0x28);
        e.set_global::<f32>(START_DISTANCE_01016970, 1000.0);
        e.register(LOCATION_GET_NAVMESH_INFOS, |e, a| {
            let buffer = fill_array(e, a[1], 4, 2);
            e.mem.set_u32(buffer, 0x11);
            e.mem.set_u32(buffer + 4, 0x22);
            ret(0)
        });
        returns(e, NAVMESH_INFO_LOAD_HOLDER, 1);
        e.register(NAVMESH_FIND_EDGE, |e, a| {
            e.mem.set_u16(a[3], 6);
            ret(1)
        });
        // The first info's centre is at distance 2, the second's at 1.
        let mut centre = 0u32;
        e.register_double(NAVMESH_GET_CENTER, move |e, a| {
            let x = if centre < 2 { 2.0 } else { 1.0 };
            centre += 1;
            write3(e, a[1], [x, 0.0, 0.0]);
            ret(a[1])
        });
        let out = e.mem.alloc(0x28);
        let points = record_point(e, LOCATION_CONSTRUCT_FROM_MESH_POINT, 1, 0x5555);
        assert!(fn_006cdb40(e, w.this, location, out, true));
        assert_eq!(points.borrow()[0], [1.0, 0.0, 0.0]);
        assert_eq!(
            calls(e, LOCATION_CONSTRUCT_FROM_MESH_POINT)[0][2..],
            [0x22, 6]
        );
        assert_eq!(calls(e, LOCATION_ASSIGN), vec![vec![out, 0x5555]]);
        assert_eq!(calls(e, NAVMESH_FIND_EDGE)[0][5], 1);
        assert_eq!(calls(e, LOCATION_DESTRUCT).last().unwrap(), &vec![location]);
        // No navmesh info: false, the copy is still destroyed.
        e.register(LOCATION_GET_NAVMESH_INFOS, |_, _| Ret::default());
        e.call_log = Some(Vec::new());
        let other = e.mem.alloc(0x28);
        assert!(!fn_006cdb40(e, w.this, other, out, false));
        assert_eq!(calls(e, LOCATION_DESTRUCT), vec![vec![other]]);
    }

    #[test]
    fn fn_006cdb40_entry_takes_the_location_by_value() {
        let mut w = world();
        let e = &mut w.e;
        let words: Vec<u32> = (0..10).map(|i| 0x100 + i).collect();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = seen.clone();
        e.register_double(LOCATION_GET_NAVMESH_INFOS, move |e, a| {
            sink.borrow_mut()
                .extend((0..10).map(|i| e.mem.u32(a[0] + 4 * i)));
            ret(0)
        });
        let mut words_in = vec![w.this.addr()];
        words_in.extend(&words);
        words_in.extend([0x7000, 1]);
        assert!(!e.call(0x006c_db40, &words_in).bool());
        assert_eq!(*seen.borrow(), words);
    }

    // -- 006cde50 / 006ce2d0 ----------------------------------------------

    #[test]
    fn fn_006cde50_edge_and_portal_locations() {
        let mut w = world();
        let buffer = virtual_nodes(&mut w, &[0, 2]);
        let (previous, current) = (node(buffer, 0), node(buffer, 1));
        let e = &mut w.e;
        let out = e.mem.alloc(0x28);
        assert!(!fn_006cde50(e, w.this, previous, current, out));
        // Bit 1: the edge point near the previous node.
        returns(e, NAVMESH_INFO_LOAD_HOLDER, 1);
        let info = e.mem.alloc(0x20);
        write3(e, info + 0x10, [3.0, 4.0, 0.0]);
        returns(e, LOCATION_GET_BEST_NAVMESH_INFO, info);
        e.register(NAVMESH_FIND_EDGE, |e, a| {
            e.mem.set_u16(a[3], 4);
            e.mem.set_u32(a[4], 1);
            ret(1)
        });
        e.register(NAVMESH_EDGE_POINTS, |e, a| {
            write3(e, a[3], [2.0, 0.0, 0.0]);
            write3(e, a[4], [4.0, 0.0, 0.0]);
            ret(0)
        });
        e.register(NAVMESH_GET_CENTER, |e, a| {
            write3(e, a[1], [0.0, 8.0, 0.0]);
            ret(a[1])
        });
        e.set_global::<f32>(CENTER_WEIGHT_0101E2BC, 0.25);
        e.set_global::<f32>(EDGE_WEIGHT_0104E2B8, 0.5);
        returns(e, NAVMESH_GET_INFO, 0x1f0);
        let points = record_point(e, LOCATION_CONSTRUCT_FROM_MESH_POINT, 1, 0x5555);
        assert!(fn_006cde50(e, w.this, previous, current, out));
        assert_eq!(points.borrow()[0], [3.0, 2.0, 0.0]);
        assert_eq!(
            calls(e, LOCATION_CONSTRUCT_FROM_MESH_POINT)[0][2..],
            [0x1f0, 4]
        );
        let find = &calls(e, NAVMESH_FIND_EDGE)[0];
        assert_eq!(find[2], 10.0f32.to_bits());
        assert_eq!(find[5], 1);
        assert_eq!(calls(e, LOCATION_ASSIGN), vec![vec![out, 0x5555]]);
        // No bit 1: the last portal triangle closer than the start distance
        // (the distance is never lowered, as in the game).
        let mut w = world();
        let buffer = virtual_nodes(&mut w, &[0, 0]);
        let (previous, current) = (node(buffer, 0), node(buffer, 1));
        let e = &mut w.e;
        returns(e, NAVMESH_INFO_LOAD_HOLDER, 1);
        e.set_global::<f32>(START_DISTANCE_01016970, 5.0);
        e.register(PORTAL_ARRAY_CONSTRUCT, |e, a| {
            let buffer = fill_array(e, a[0], 4, 3);
            e.mem.set_u32(buffer, 1);
            e.mem.set_u32(buffer + 4, 2);
            e.mem.set_u32(buffer + 8, 3);
            ret(a[0])
        });
        e.register(NAVMESH_GET_CENTER, |e, a| {
            write3(e, a[1], [a[2] as f32 * 2.0, 0.0, 0.0]);
            ret(a[1])
        });
        let out = e.mem.alloc(0x28);
        assert!(fn_006cde50(e, w.this, previous, current, out));
        assert_eq!(calls(e, LOCATION_CONSTRUCT_FROM_MESH_POINT)[0][3], 2);
        assert_eq!(calls(e, PORTAL_ARRAY_CONSTRUCT)[0][1..], [0, 0, 0x5ca7]);
        assert_eq!(calls(e, PORTAL_ARRAY_DESTRUCT).len(), 1);
    }

    #[test]
    fn fn_006ce2d0_lifts_points_to_the_ground() {
        let mut w = world();
        let buffer = virtual_nodes(&mut w, &[2, 0]);
        let (current, next) = (node(buffer, 0), node(buffer, 1));
        let e = &mut w.e;
        returns(e, NAVMESH_INFO_LOAD_HOLDER, 1);
        let info = e.mem.alloc(0x20);
        returns(e, LOCATION_GET_BEST_NAVMESH_INFO, info);
        returns(e, NAVMESH_FIND_EDGE, 1);
        e.register(NAVMESH_GET_CENTER, |e, a| {
            write3(e, a[1], [0.0, 0.0, 2.0]);
            ret(a[1])
        });
        e.set_global::<f32>(CENTER_WEIGHT_0101E2BC, 1.0);
        e.set_global::<f64>(GROUND_OFFSET_0104F2F0, 1.0);
        returns_float(e, TES_GROUND_HEIGHT, 5.0);
        let points = record_point(e, LOCATION_CONSTRUCT_FROM_MESH_POINT, 1, 0x5555);
        let out = e.mem.alloc(0x28);
        assert!(fn_006ce2d0(e, w.this, current, next, out));
        assert_eq!(points.borrow()[0], [0.0, 0.0, 5.0]);
        assert_eq!(calls(e, NAVMESH_FIND_EDGE)[0][5], 0);
        // Above ground plus the offset: kept.
        returns_float(e, TES_GROUND_HEIGHT, 0.5);
        assert!(fn_006ce2d0(e, w.this, current, next, out));
        assert_eq!(points.borrow()[1], [0.0, 0.0, 2.0]);
        // Without bit 1 and no portal triangle: false.
        e.mem.set_u32(current, 0);
        assert!(!fn_006ce2d0(e, w.this, current, next, out));
        assert_eq!(calls(e, PORTAL_ARRAY_DESTRUCT).len(), 1);
    }

    // -- 006ce7e0 / 006ce8b0 / 006cec40 -----------------------------------

    #[test]
    fn add_virtual_pathing_node_records_the_door_id() {
        let mut w = world();
        let e = &mut w.e;
        returns(e, PLACEMENT_NEW, 0x880);
        returns(e, FORM_GET_ID, 0x0001_2345);
        let ids = Rc::new(RefCell::new(Vec::new()));
        let sink = ids.clone();
        e.register_double(ARRAY_PUSH_WORD, move |e, a| {
            sink.borrow_mut().push((a[0], e.mem.u32(a[1])));
            ret(0)
        });
        assert_eq!(
            pathing_solution_add_virtual_pathing_node(e, w.solution, 0x100, 0xd00),
            0
        );
        assert_eq!(
            calls(e, VIRTUAL_NODE_CONSTRUCT_LOCATION_DOOR),
            vec![vec![0x880, 0x100, 0xd00]]
        );
        assert_eq!(*ids.borrow(), vec![(w.solution + 0x30, 0x0001_2345)]);
        assert_eq!(
            pathing_solution_add_virtual_pathing_node(e, w.solution, 0x100, 0),
            1
        );
        assert_eq!(ids.borrow().len(), 1);
    }

    #[test]
    fn build_nav_mesh_info_path_in_space() {
        let mut w = world();
        let e = &mut w.e;
        let space = e.mem.alloc(12);
        e.mem.set_u32(space + 4, 0xa4);
        e.mem.set_u32(space + 8, 0xa8);
        assert!(!path_builder_build_nav_mesh_info_path_in_space(
            e, w.this, 0x10, 0x20, space, 0
        ));
        assert_eq!(
            calls(e, LOCATION_CONSTRUCT_FROM_PARTS)[0][1..],
            [0x10, 0xa8, 0xa4]
        );
        assert_eq!(calls(e, LOCATION_DESTRUCT).len(), 2);
        // Found, with a door: the goal is a door node.
        returns(e, LOCATION_RESOLVE, 1);
        returns(e, INFO_SEARCH_RUN, 1);
        constructs_words(e, INFO_SCRAP_ARRAY_CONSTRUCT, &[0x1, 0x2, 0x3]);
        e.call_log = Some(Vec::new());
        assert!(path_builder_build_nav_mesh_info_path_in_space(
            e, w.this, 0x10, 0x20, space, 0xd00
        ));
        let run = &calls(e, INFO_SEARCH_RUN)[0];
        assert_eq!((run[1], run[8]), (0, 0));
        let start = calls(e, LOCATION_CONSTRUCT_FROM_PARTS)[0][0];
        assert_eq!(
            calls(e, SOLUTION_ADD_LOCATION_NODE),
            vec![vec![w.solution, start]]
        );
        assert!(calls(e, VIRTUAL_NODE_CONSTRUCT_INFO).is_empty());
        assert_eq!(calls(e, VIRTUAL_NODE_SLOT).len(), 2);
        assert_eq!(calls(e, FORM_GET_ID), vec![vec![0xd00]]);
        // Not found but with infos and byte +0x9B: incomplete, ends at the
        // last info's point.
        returns(e, INFO_SEARCH_RUN, 0);
        returns(e, REQUEST_BYTE_9B, 1);
        returns(e, PLACEMENT_NEW, 0x880);
        e.call_log = Some(Vec::new());
        assert!(!path_builder_build_nav_mesh_info_path_in_space(
            e, w.this, 0x10, 0x20, space, 0
        ));
        assert_eq!(e.mem.u8(w.solution + 0x40), 1);
        assert_eq!(
            calls(e, VIRTUAL_NODE_CONSTRUCT_INFO),
            vec![vec![0x880, 0x2]]
        );
        assert_eq!(
            calls(e, VIRTUAL_NODE_CONSTRUCT_POINT_INFO),
            vec![vec![0x880, 0x13, 0x3]]
        );
    }

    #[test]
    fn fn_006cec40_adds_plain_nodes_and_the_door() {
        let mut w = world();
        let buffer = virtual_nodes(&mut w, &[2, 4]);
        let e = &mut w.e;
        returns(e, LOCATION_GET_BEST_NAVMESH_INFO, 0x1000);
        returns(e, VIRTUAL_NODE_DOOR, 0xd00);
        let current = e.mem.alloc(0x3c);
        returns(e, SOLUTION_LAST_CURRENT_NODE, current);
        assert!(fn_006cec40(e, w.this, 0, 1));
        let copies = calls(e, LOCATION_COPY_CONSTRUCT);
        assert_eq!(copies[0][1], node(buffer, 0) + 4);
        assert_eq!(
            calls(e, LOCATION_SET_POSITION),
            vec![vec![copies[1][0], 0x1010], vec![current + 4, 0]]
        );
        assert_eq!(calls(e, SOLUTION_ADD_CURRENT_NODE).len(), 2);
        assert_eq!(e.mem.u32(current + 0x38), 0xd00);
    }

    // -- 006ced60 .. 006cf1b0 ---------------------------------------------

    #[test]
    fn fn_006ced60_returns_true() {
        let mut e = Engine::new();
        assert!(fn_006ced60(&mut e, 0, 1, 2, 3, 4));
    }

    #[test]
    fn array_end_accessors() {
        let mut w = world();
        let e = &mut w.e;
        let array = e.mem.alloc(0x10);
        let buffer = fill_array(e, array, 0x28, 3);
        assert_eq!(fn_006ced70(e, array), buffer);
        assert_eq!(fn_006ced90(e, array), buffer + 24);
        assert_eq!(fn_006cedb0(e, array), buffer + 32);
        assert_eq!(fn_006cedd0(e, array), buffer + 0x50);
        assert_eq!(fn_006cedf0(e, array), buffer);
        assert_eq!(fn_006cee10(e, array, 2), buffer + 0x30);
    }

    #[test]
    fn fn_006cee10_indexes_edge_locations() {
        let mut e = Engine::new();
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array + 4, 0x1000);
        assert_eq!(fn_006cee10(&mut e, array, 3), 0x1048);
    }

    #[test]
    fn fn_006cee30_cuts_and_reallocates() {
        let mut w = world();
        let e = &mut w.e;
        let array = e.mem.alloc(0x10);
        fill_array(e, array, 12, 5);
        fn_006cee30(e, array, 5);
        assert!(calls(e, SPACE_ARRAY_REALLOCATE).is_empty());
        fn_006cee30(e, array, 2);
        let buffer = e.mem.u32(array + 4);
        assert_eq!(
            calls(e, SPACE_RANGE_DESTRUCT),
            vec![vec![array, buffer + 24, 3]]
        );
        assert_eq!(calls(e, SPACE_ARRAY_REALLOCATE), vec![vec![array, 2, 2]]);
        assert_eq!((e.mem.u32(array + 8), e.mem.u32(array + 0xc)), (2, 2));
    }

    #[test]
    fn fn_006ceea0_cuts_links_and_reallocates() {
        let mut w = world();
        let e = &mut w.e;
        let array = e.mem.alloc(0x10);
        let buffer = fill_array(e, array, 16, 4);
        fn_006ceea0(e, array, 1);
        assert_eq!(
            calls(e, SPACE_RANGE_DESTRUCT),
            vec![vec![array, buffer + 16, 3]]
        );
        assert_eq!(calls(e, LINK_ARRAY_REALLOCATE), vec![vec![array, 1, 1]]);
        fn_006ceea0(e, array, 8);
        assert_eq!(calls(e, LINK_ARRAY_REALLOCATE)[1], vec![array, 8, 1]);
        assert_eq!(e.mem.u32(array + 0xc), 8);
    }

    #[test]
    fn fn_006cef10_grows_when_full() {
        let mut w = world();
        let e = &mut w.e;
        let array = e.mem.alloc(0x10);
        returns(e, ARRAY_IS_FULL, 1);
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(array, vtable);
        e.mem.set_u32(vtable + 4, 0x7ff0_0020);
        returns(e, 0x7ff0_0020, 0x8800);
        assert_eq!(fn_006cef10(e, array), 0);
        assert_eq!(calls(e, 0x7ff0_0020), vec![vec![array, 4]]);
        assert_eq!(
            (
                e.mem.u32(array + 4),
                e.mem.u32(array + 8),
                e.mem.u32(array + 0xc)
            ),
            (0x8800, 1, 4)
        );
        returns(e, ARRAY_GROWN_CAPACITY, 8);
        assert_eq!(fn_006cef10(e, array), 1);
        assert_eq!(
            calls(e, VIRTUAL_NODE_ARRAY_REALLOCATE),
            vec![vec![array, 8, 1]]
        );
        assert_eq!(e.mem.u32(array + 0xc), 8);
        returns(e, ARRAY_IS_FULL, 0);
        assert_eq!(fn_006cef10(e, array), 2);
    }

    #[test]
    fn fn_006cefa0_clears_virtual_nodes() {
        let mut w = world();
        let e = &mut w.e;
        let array = e.mem.alloc(0x10);
        fn_006cefa0(e, array, true);
        assert!(calls(e, VIRTUAL_NODE_RANGE_DESTRUCT).is_empty());
        let buffer = fill_array(e, array, 0x30, 3);
        fn_006cefa0(e, array, false);
        assert_eq!(
            calls(e, VIRTUAL_NODE_RANGE_DESTRUCT),
            vec![vec![array, buffer, 3]]
        );
        assert_eq!((e.mem.u32(array + 4), e.mem.u32(array + 8)), (buffer, 0));
        assert!(calls(e, ARRAY_FREE_BUFFER).is_empty());
    }

    #[test]
    fn fn_006cf120_clears_pathing_nodes() {
        let mut w = world();
        let e = &mut w.e;
        let array = e.mem.alloc(0x10);
        let buffer = fill_array(e, array, 0x3c, 2);
        fn_006cf120(e, array, true);
        assert_eq!(
            calls(e, PATHING_NODE_RANGE_DESTRUCT),
            vec![vec![array, buffer, 2]]
        );
        assert_eq!(calls(e, ARRAY_FREE_BUFFER), vec![vec![array]]);
        assert_eq!(
            (
                e.mem.u32(array + 4),
                e.mem.u32(array + 8),
                e.mem.u32(array + 0xc)
            ),
            (0, 0, 0)
        );
    }

    #[test]
    fn fn_006cf000_removes_in_place_or_shrinks() {
        let mut w = world();
        let e = &mut w.e;
        let array = e.mem.alloc(0x10);
        let buffer = fill_array(e, array, 0x3c, 4);
        fn_006cf000(e, array, 1, true);
        assert_eq!(
            calls(e, PATHING_NODE_RANGE_DESTRUCT),
            vec![vec![array, buffer + 0x3c, 1]]
        );
        assert_eq!(
            calls(e, PATHING_NODE_RANGE_MOVE),
            vec![vec![array, buffer + 0x3c, buffer + 0x78, 2]]
        );
        assert_eq!(e.mem.u32(array + 8), 3);
        // Shrinking: a new buffer of the shrunk capacity.
        returns(e, ARRAY_SHOULD_SHRINK, 1);
        returns(e, ARRAY_SHRUNK_CAPACITY, 2);
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(array, vtable);
        e.mem.set_u32(vtable + 4, 0x7ff0_0030);
        returns(e, 0x7ff0_0030, 0x9900);
        e.call_log = Some(Vec::new());
        fn_006cf000(e, array, 1, true);
        assert_eq!(
            calls(e, PATHING_NODE_RANGE_MOVE),
            vec![
                vec![array, 0x9900, buffer, 1],
                vec![array, 0x9900 + 0x3c, buffer + 0x78, 2]
            ]
        );
        assert_eq!(calls(e, ARRAY_FREE_BUFFER), vec![vec![array]]);
        assert_eq!(
            (
                e.mem.u32(array + 4),
                e.mem.u32(array + 8),
                e.mem.u32(array + 0xc)
            ),
            (0x9900, 2, 2)
        );
    }

    #[test]
    fn fn_006cf180_constructs_a_location_array() {
        let mut w = world();
        let e = &mut w.e;
        let array = e.mem.alloc(0x10);
        assert_eq!(fn_006cf180(e, array), array);
        assert_eq!(e.mem.u32(array), LOCATION_ARRAY_VTABLE);
        assert_eq!(calls(e, LOCATION_ARRAY_INIT), vec![vec![array, 0, 0]]);
    }

    #[test]
    fn fn_006cf1b0_appends_a_location() {
        let mut w = world();
        let e = &mut w.e;
        let array = e.mem.alloc(0x10);
        let buffer = fill_array(e, array, 0x28, 3);
        returns(e, LOCATION_ARRAY_ADD_SLOT, 2);
        assert_eq!(fn_006cf1b0(e, array, 0x500), 2);
        assert_eq!(
            calls(e, LOCATION_ARRAY_CONSTRUCT_ELEMENTS),
            vec![vec![array, buffer + 0x50, 1]]
        );
        assert_eq!(calls(e, LOCATION_ASSIGN), vec![vec![buffer + 0x50, 0x500]]);
    }

    /// Gives `object` a vtable whose slot 4 is the double `slot4`, which
    /// returns `buffer`.
    fn vtable_slot4(e: &mut Engine, object: u32, slot4: u32, buffer: u32) {
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(vtable + 4, slot4);
        returns(e, slot4, buffer);
    }

    #[test]
    fn fn_006cf200_constructs_an_edge_array() {
        let mut w = world();
        let e = &mut w.e;
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array + 4, 0x77);
        assert_eq!(fn_006cf200(e, array), array);
        assert_eq!(e.mem.u32(array), EDGE_ARRAY_VTABLE);
        assert_eq!(
            (
                e.mem.u32(array + 4),
                e.mem.u32(array + 8),
                e.mem.u32(array + 0xc)
            ),
            (0, 0, 0)
        );
        assert!(calls(e, EDGE_ARRAY_CONSTRUCT_ELEMENTS).is_empty());
    }

    #[test]
    fn fn_006cf230_destroys_an_edge_array() {
        let mut w = world();
        let e = &mut w.e;
        let array = e.mem.alloc(0x10);
        fn_006cf230(e, array);
        assert_eq!(e.mem.u32(array), EDGE_ARRAY_VTABLE);
        assert_eq!(calls(e, ARRAY_CLEAR_008454F0), vec![vec![array, 1]]);
    }

    #[test]
    fn edge_array_allocate_takes_0x18_bytes_per_element() {
        let mut w = world();
        let e = &mut w.e;
        returns(e, MEMORY_ALLOCATE, 0x4400);
        assert_eq!(
            bs_simple_array_nav_mesh_edge_location_allocate(e, 0x100, 3),
            0x4400
        );
        assert_eq!(calls(e, MEMORY_ALLOCATE), vec![vec![0x48]]);
    }

    #[test]
    fn edge_array_reallocate_returns_the_new_block() {
        let mut w = world();
        let e = &mut w.e;
        returns(e, MEMORY_REALLOCATE, 0x4800);
        assert_eq!(
            bs_simple_array_nav_mesh_edge_location_reallocate(e, 0x100, 0x4400, 5),
            0x4800
        );
        assert_eq!(calls(e, MEMORY_REALLOCATE), vec![vec![0x4400, 0x78]]);
    }

    /// Checks a scrap array constructor: the base constructor first (its
    /// vtable overwritten), the given heap kept, the thread's scrap heap
    /// taken for 0, and the initialiser's arguments.
    fn check_scrap_constructor(
        construct: fn(&mut Engine, u32, u32, u32, u32) -> u32,
        base: u32,
        vtable: u32,
        init: u32,
    ) {
        let mut w = world();
        let e = &mut w.e;
        e.register(base, |e, a| {
            e.mem.set_u32(a[0], 0xdead);
            ret(a[0])
        });
        let array = e.mem.alloc(0x14);
        assert_eq!(construct(e, array, 0x14, 2, 0x5ca7), array);
        assert_eq!(e.mem.u32(array), vtable);
        assert_eq!(e.mem.u32(array + 0x10), 0x5ca7);
        assert_eq!(calls(e, base), vec![vec![array]]);
        assert_eq!(calls(e, init), vec![vec![array, 0x14, 2]]);
        assert!(calls(e, GET_THREAD_SCRAP_HEAP).is_empty());
        returns(e, MEMORY_MANAGER_GET, 0x011f_6238);
        returns(e, GET_THREAD_SCRAP_HEAP, 0x6000);
        construct(e, array, 0, 0, 0);
        assert_eq!(e.mem.u32(array + 0x10), 0x6000);
        assert_eq!(calls(e, GET_THREAD_SCRAP_HEAP), vec![vec![0x011f_6238]]);
        assert_eq!(calls(e, init)[1], vec![array, 0, 0]);
    }

    /// Checks a scrap array destructor: vtable, clear with free, base.
    fn check_scrap_destructor(destruct: fn(&mut Engine, u32), vtable: u32, base: u32) {
        let mut w = world();
        let e = &mut w.e;
        let array = e.mem.alloc(0x14);
        destruct(e, array);
        assert_eq!(e.mem.u32(array), vtable);
        assert_eq!(calls(e, ARRAY_CLEAR_008454F0), vec![vec![array, 1]]);
        assert_eq!(calls(e, base), vec![vec![array]]);
    }

    #[test]
    fn fn_006cf2a0_constructs_an_info_scrap_array() {
        check_scrap_constructor(
            fn_006cf2a0,
            INFO_ARRAY_BASE_CONSTRUCT,
            INFO_SCRAP_ARRAY_VTABLE,
            ARRAY_INIT_006B3EB0,
        );
    }

    #[test]
    fn fn_006cf3a0_constructs_a_reference_scrap_array() {
        check_scrap_constructor(
            fn_006cf3a0,
            REFERENCE_ARRAY_BASE_CONSTRUCT,
            REFERENCE_SCRAP_ARRAY_VTABLE,
            ARRAY_INIT_006B3EB0,
        );
    }

    #[test]
    fn fn_006cf440_destroys_a_reference_scrap_array() {
        check_scrap_destructor(
            fn_006cf440,
            REFERENCE_SCRAP_ARRAY_VTABLE,
            REFERENCE_ARRAY_BASE_DESTRUCT,
        );
    }

    #[test]
    fn fn_006cf520_appends_a_byte() {
        let mut w = world();
        let e = &mut w.e;
        let array = e.mem.alloc(0x14);
        let buffer = fill_array(e, array, 1, 8);
        returns(e, BOOL_ARRAY_ADD_SLOT, 5);
        let value = e.mem.alloc(4);
        e.mem.set_u8(value, 1);
        assert_eq!(fn_006cf520(e, array, value), 5);
        assert_eq!(
            calls(e, BOOL_ARRAY_CONSTRUCT_ELEMENTS),
            vec![vec![array, buffer + 5, 1]]
        );
        assert_eq!(e.mem.u8(buffer + 5), 1);
        assert_eq!(e.mem.u8(buffer + 4), 0);
    }

    #[test]
    fn fn_006cf570_constructs_a_bool_scrap_array() {
        check_scrap_constructor(
            fn_006cf570,
            BOOL_ARRAY_BASE_CONSTRUCT,
            BOOL_SCRAP_ARRAY_VTABLE,
            ARRAY_INIT_0069B940,
        );
    }

    #[test]
    fn bool_scrap_array_allocate_uses_the_scrap_heap() {
        let mut w = world();
        let e = &mut w.e;
        e.set_global::<u32>(SCRAP_ALIGNMENT_010A2720, 4);
        let array = e.mem.alloc(0x14);
        e.mem.set_u32(array + 0x10, 0x6000);
        returns(e, SCRAP_HEAP_ALLOCATE, 0x6100);
        assert_eq!(bs_scrap_array_bool_allocate(e, array, 9), 0x6100);
        assert_eq!(calls(e, SCRAP_HEAP_ALLOCATE), vec![vec![0x6000, 9, 4]]);
    }

    #[test]
    fn bool_scrap_array_reallocate_copies_and_frees() {
        let mut w = world();
        let e = &mut w.e;
        e.set_global::<u32>(SCRAP_ALIGNMENT_010A2720, 4);
        let array = e.mem.alloc(0x14);
        e.mem.set_u32(array + 8, 3);
        e.mem.set_u32(array + 0x10, 0x6000);
        returns(e, SCRAP_HEAP_ALLOCATE, 0x6200);
        assert_eq!(bs_scrap_array_bool_reallocate(e, array, 0x6100, 8), 0x6200);
        assert_eq!(calls(e, SCRAP_HEAP_ALLOCATE), vec![vec![0x6000, 8, 4]]);
        assert_eq!(calls(e, MEMORY_COPY), vec![vec![0x6200, 0x6100, 3]]);
        assert_eq!(calls(e, SCRAP_HEAP_DEALLOCATE), vec![vec![0x6000, 0x6100]]);
    }

    #[test]
    fn fn_006cf780_constructs_a_segment_scrap_array() {
        check_scrap_constructor(
            fn_006cf780,
            SEGMENT_ARRAY_BASE_CONSTRUCT,
            SEGMENT_SCRAP_ARRAY_VTABLE,
            ARRAY_INIT_006B3F40,
        );
    }

    #[test]
    fn fn_006cf840_destroys_a_segment_scrap_array() {
        check_scrap_destructor(
            fn_006cf840,
            SEGMENT_SCRAP_ARRAY_VTABLE,
            SEGMENT_ARRAY_BASE_DESTRUCT,
        );
    }

    #[test]
    fn fn_006cf9a0_constructs_a_search_link_array() {
        let mut w = world();
        let e = &mut w.e;
        let array = e.mem.alloc(0x10);
        assert_eq!(fn_006cf9a0(e, array, 4, 1), array);
        assert_eq!(e.mem.u32(array), SEARCH_LINK_ARRAY_VTABLE);
        assert_eq!(calls(e, ARRAY_INIT_006B3F40), vec![vec![array, 4, 1]]);
    }

    #[test]
    fn fn_006cf9d0_constructs_a_search_node_array() {
        let mut w = world();
        let e = &mut w.e;
        let array = e.mem.alloc(0x10);
        assert_eq!(fn_006cf9d0(e, array, 6, 0), array);
        assert_eq!(e.mem.u32(array), SEARCH_NODE_ARRAY_VTABLE);
        assert_eq!(calls(e, ARRAY_INIT_009A3D00), vec![vec![array, 6, 0]]);
    }

    #[test]
    fn fn_006cfa00_constructs_a_triangle_handle_scrap_array() {
        check_scrap_constructor(
            fn_006cfa00,
            TRIANGLE_HANDLE_ARRAY_BASE_CONSTRUCT,
            TRIANGLE_HANDLE_SCRAP_ARRAY_VTABLE,
            ARRAY_INIT_006DC440,
        );
    }

    #[test]
    fn fn_006cfac0_destroys_a_triangle_handle_scrap_array() {
        check_scrap_destructor(
            fn_006cfac0,
            TRIANGLE_HANDLE_SCRAP_ARRAY_VTABLE,
            TRIANGLE_HANDLE_ARRAY_BASE_DESTRUCT,
        );
    }

    #[test]
    fn fn_006cfb20_constructs_an_integer_scrap_array() {
        check_scrap_constructor(
            fn_006cfb20,
            INTEGER_ARRAY_BASE_CONSTRUCT,
            INTEGER_SCRAP_ARRAY_VTABLE,
            ARRAY_INIT_006B3EB0,
        );
    }

    #[test]
    fn fn_006cfbc0_destroys_an_integer_scrap_array() {
        check_scrap_destructor(
            fn_006cfbc0,
            INTEGER_SCRAP_ARRAY_VTABLE,
            INTEGER_ARRAY_BASE_DESTRUCT,
        );
    }

    /// Checks a scalar deleting destructor: the destructor call `marker`
    /// (an address its destructor calls with `this` first) always, the
    /// object freed only with bit 0 of the flags.
    fn check_scalar_deleting(deleting: fn(&mut Engine, u32, u32) -> u32, marker: u32) {
        let mut w = world();
        let e = &mut w.e;
        let array = e.mem.alloc(0x14);
        assert_eq!(deleting(e, array, 0), array);
        assert_eq!(calls(e, marker).len(), 1);
        assert_eq!(calls(e, marker)[0][0], array);
        assert!(calls(e, MEMORY_FREE).is_empty());
        assert_eq!(deleting(e, array, 3), array);
        assert_eq!(calls(e, marker).len(), 2);
        assert_eq!(calls(e, MEMORY_FREE), vec![vec![array]]);
    }

    #[test]
    fn edge_array_scalar_deleting_destructor() {
        check_scalar_deleting(
            bs_simple_array_nav_mesh_edge_location_scalar_deleting_destructor,
            ARRAY_CLEAR_008454F0,
        );
    }

    #[test]
    fn info_scrap_array_scalar_deleting_destructor() {
        check_scalar_deleting(
            bs_scrap_array_nav_mesh_info_const_p_scalar_deleting_destructor,
            INFO_SCRAP_ARRAY_DESTRUCT,
        );
    }

    #[test]
    fn reference_scrap_array_scalar_deleting_destructor() {
        check_scalar_deleting(
            bs_scrap_array_tes_object_refr_p_scalar_deleting_destructor,
            REFERENCE_ARRAY_BASE_DESTRUCT,
        );
    }

    #[test]
    fn segment_array_scalar_deleting_destructor() {
        check_scalar_deleting(
            bs_simple_array_path_builder_virtual_node_segment_scalar_deleting_destructor,
            SEGMENT_ARRAY_BASE_DESTRUCT,
        );
    }

    #[test]
    fn bool_scrap_array_scalar_deleting_destructor() {
        check_scalar_deleting(
            bs_scrap_array_bool_scalar_deleting_destructor,
            BOOL_SCRAP_ARRAY_DESTRUCT,
        );
    }

    #[test]
    fn segment_scrap_array_scalar_deleting_destructor() {
        check_scalar_deleting(
            bs_scrap_array_path_builder_virtual_node_segment_scalar_deleting_destructor,
            SEGMENT_ARRAY_BASE_DESTRUCT,
        );
    }

    #[test]
    fn triangle_handle_array_scalar_deleting_destructor() {
        check_scalar_deleting(
            bs_simple_array_fast_nav_mesh_tri_handle_scalar_deleting_destructor,
            TRIANGLE_HANDLE_ARRAY_BASE_DESTRUCT,
        );
    }

    #[test]
    fn triangle_handle_scrap_array_scalar_deleting_destructor() {
        check_scalar_deleting(
            bs_scrap_array_fast_nav_mesh_tri_handle_scalar_deleting_destructor,
            TRIANGLE_HANDLE_ARRAY_BASE_DESTRUCT,
        );
    }

    #[test]
    fn integer_scrap_array_scalar_deleting_destructor() {
        check_scalar_deleting(
            bs_scrap_array_int_scalar_deleting_destructor,
            INTEGER_ARRAY_BASE_DESTRUCT,
        );
    }

    #[test]
    fn fn_006d0310_allocates_and_constructs() {
        let mut w = world();
        let e = &mut w.e;
        let array = e.mem.alloc(0x10);
        vtable_slot4(e, array, 0x7ff0_0040, 0x9a00);
        // Size above capacity: capacity becomes the size.
        fn_006d0310(e, array, 2, 5);
        assert_eq!(calls(e, 0x7ff0_0040), vec![vec![array, 5]]);
        assert_eq!(
            calls(e, EDGE_ARRAY_CONSTRUCT_ELEMENTS),
            vec![vec![array, 0x9a00, 5]]
        );
        assert_eq!(
            (
                e.mem.u32(array + 4),
                e.mem.u32(array + 8),
                e.mem.u32(array + 0xc)
            ),
            (0x9a00, 5, 5)
        );
        // Capacity only: allocated, nothing constructed.
        fn_006d0310(e, array, 8, 0);
        assert_eq!(calls(e, 0x7ff0_0040)[1], vec![array, 8]);
        assert_eq!(calls(e, EDGE_ARRAY_CONSTRUCT_ELEMENTS).len(), 1);
        assert_eq!(
            (
                e.mem.u32(array + 4),
                e.mem.u32(array + 8),
                e.mem.u32(array + 0xc)
            ),
            (0x9a00, 0, 8)
        );
    }
}
