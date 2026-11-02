//! `fallout shared/pathfinding/pathfind.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The unit solves a `PathingRequest` for the pathing task: `fn_006d0900`
//! looks at the request's type (virtual slot `GetType`, +0x10) and hands it
//! to the solver of that type (0 base request, 3 flee, 4 close point, 5 line
//! of sight, 6 hide, 7 optimal location, 8 covered move). Each solver fills a
//! `PathingSolution` and returns whether it found a path. The rest of the
//! file are small accessors and constructors the solvers use.
//!
//! How the translations read the game code:
//! - The game keeps many objects on its stack (searches, node arrays, scope
//!   guards) and passes their addresses to its functions. Each solver here
//!   takes one zeroed block for its frame ([`with_frame`]) and addresses the
//!   locals by their `EBP`-relative offset exactly as the code does, so a
//!   comment `// ebp-0x20ec` names the local and the test can find it.
//! - The compiler's exception-unwinding frame (`__CxxFrameHandler` state
//!   writes and the `FS:[0]` chain) is not translated; every exit path
//!   destroys the live locals in the order the game does.
//! - The stack cookie check (`__security_check_cookie`) of the functions that
//!   have one is not translated either.
//! - Callee arguments are listed in the uniform form (`this` first, then the
//!   stack words from the top of the stack downwards, i.e. the last `PUSH`
//!   first). Floats the callee leaves in `ST0` come back through `.f32()` or,
//!   where the game compares the unrounded `ST0`, `.f64()`.
//! - Offsets that are not in a declared layout are named in comments with
//!   the Xbox PDB field they correspond to.
//! - Translated: the whole unit (the last session did `006db3e0` to `006dc070`).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiPoint3;

// ---------------------------------------------------------------------------
// Layouts (PC offsets; names from the Xbox PDB where it has the class)
// ---------------------------------------------------------------------------

layout! {
    /// `PathingLocation` (Xbox PDB), 0x28 bytes: vtable, position, navmesh
    /// info, cell, worldspace, triangle and flags.
    pub struct PathingLocation: 0x28 {
        /// `Location` (Xbox PDB): the position.
        0x04 Location: Inline<NiPoint3>,
        /// `pCell` (Xbox PDB): `TESObjectCELL*`.
        0x18 pCell: Ptr,
        /// `pWorldSpace` (Xbox PDB): `TESWorldSpace*`.
        0x1C pWorldSpace: Ptr,
        /// `usTriangle` (Xbox PDB).
        0x24 usTriangle: u16,
        /// `uiFlags` (Xbox PDB): bit 1 (`2`) is set by `fn_006d2c00` and
        /// tested by `006a9520`.
        0x26 uiFlags: u8,
    }

    /// `PathingRequest` (Xbox PDB), 0xB0 bytes; only the fields this unit
    /// touches.
    pub struct PathingRequest: 0xB0 {
        /// `Origin` (Xbox PDB).
        0x0C Origin: Inline<PathingLocation>,
        /// `Destination` (Xbox PDB).
        0x34 Destination: Inline<PathingLocation>,
        /// `fActorRadius` (Xbox PDB).
        0x68 fActorRadius: f32,
        /// `fTargetRadius` (Xbox PDB).
        0x74 fTargetRadius: f32,
        /// `fInitialPathHeading` (Xbox PDB).
        0x90 fInitialPathHeading: f32,
        /// `spAvoidNodeArray` (Xbox PDB): `NiPointer<PathingAvoidNodeArray>`.
        0x94 spAvoidNodeArray: Ptr,
        /// `bFirstTangentUsesHeading` (Xbox PDB).
        0xA0 bFirstTangentUsesHeading: bool,
    }

    /// `PathingRequestCoveredMove` (Xbox PDB), 0x10C bytes: a
    /// `PathingRequest` plus the cover data; only the fields this unit
    /// touches.
    pub struct PathingRequestCoveredMove: 0x10C {
        /// `CoverLocation` (Xbox PDB).
        0xC8 CoverLocation: Inline<NiPoint3>,
        /// `pCombatCoverLocation` (Xbox PDB): `CombatCoverLocation*`.
        0x104 pCombatCoverLocation: Ptr,
        /// `pCombatCoverSearchInfo` (Xbox PDB): `CombatCoverSearchInfo*`.
        0x108 pCombatCoverSearchInfo: Ptr,
    }

    /// `PathingSolution` (Xbox PDB); the PC build has only the node data,
    /// laid out right after the `NiRefObject` base (the Xbox build's debug
    /// data is missing), so the offsets differ from the PDB's.
    pub struct PathingSolution: 0x44 {
        /// `VirtualPathingNodes` (Xbox PDB): a `BSSimpleArray` of 0x30-byte
        /// `VirtualPathingNode`s.
        0x08 VirtualPathingNodes: Inline<crate::types::BSSimpleArray>,
        /// `iFirstLoadedVirtualNodeIndex` (Xbox PDB).
        0x18 iFirstLoadedVirtualNodeIndex: i32,
        /// `iLastLoadedVirtualNodeIndex` (Xbox PDB).
        0x1C iLastLoadedVirtualNodeIndex: i32,
        /// `CurrentPathingNodes` (Xbox PDB): a `BSSimpleArray` of
        /// `PathingNode`s.
        0x20 CurrentPathingNodes: Inline<crate::types::BSSimpleArray>,
        /// `bIncompletePath` (Xbox PDB).
        0x40 bIncompletePath: bool,
    }

    /// `CombatCoverLocation` (Xbox PDB), 0x78 bytes.
    pub struct CombatCoverLocation: 0x78 {
        /// `bDirtyFlag` (Xbox PDB).
        0x00 bDirtyFlag: bool,
        /// `bReserved` (Xbox PDB).
        0x01 bReserved: bool,
        /// `cCoverHeight` (Xbox PDB).
        0x02 cCoverHeight: u8,
        /// `iLocation` (Xbox PDB).
        0x04 iLocation: i32,
        /// `CoverSearchTargetLocation` (Xbox PDB).
        0x08 CoverSearchTargetLocation: Inline<NiPoint3>,
        /// `CoverLocation` (Xbox PDB): a `PathingCoverLocation`.
        0x14 CoverLocation: Inline<PathingCoverLocation>,
        /// `iCoverLocationKey` (Xbox PDB).
        0x68 iCoverLocationKey: i32,
        /// `iCoverLocationLastMovedTimeStamp` (Xbox PDB).
        0x6C iCoverLocationLastMovedTimeStamp: i32,
        /// `ReservationInfo` (Xbox PDB): a `CoverReservationInfo`.
        0x70 ReservationInfo: Inline<CoverReservationInfo>,
    }

    /// `PathingCoverLocation` (Xbox PDB), 0x54 bytes: a `PathingLocation`
    /// followed by three points.
    pub struct PathingCoverLocation: 0x54 {}

    /// `CoverReservationInfo` (Xbox PDB), 6 bytes of flags.
    pub struct CoverReservationInfo: 0x06 {}

    /// `CombatCoverSearchInfo` (Xbox PDB), 0x3C bytes.
    pub struct CombatCoverSearchInfo: 0x3C {
        /// `TargetLocation` (Xbox PDB).
        0x00 TargetLocation: Inline<NiPoint3>,
        /// `Center` (Xbox PDB).
        0x0C Center: Inline<NiPoint3>,
        /// `fRadius` (Xbox PDB).
        0x18 fRadius: f32,
        /// `fTimeStamp` (Xbox PDB): a `CombatTimeStamp` (one float).
        0x1C fTimeStamp: f32,
        /// `iPreviousCoverLocationKey` (Xbox PDB).
        0x30 iPreviousCoverLocationKey: i32,
        /// `ePreviousCoverState` (Xbox PDB).
        0x34 ePreviousCoverState: i32,
        /// `bUsedLastSeenLocation` (Xbox PDB).
        0x38 bUsedLastSeenLocation: bool,
    }
}

// ---------------------------------------------------------------------------
// Frames and callees
// ---------------------------------------------------------------------------

/// The stack frame of a translated function: a zeroed block standing in for
/// the game's stack, addressed by the `EBP`-relative offsets the code uses.
#[derive(Clone, Copy)]
struct Frame(u32);

impl Frame {
    /// The address of the local at `EBP + offset` (the offsets are negative
    /// for locals).
    fn at(self, offset: i32) -> u32 {
        self.0.wrapping_add(offset as u32)
    }
}

/// Runs `body` with a zeroed frame of the size the function reserves
/// (`locals`, from its `SUB ESP` or `__alloca_probe` size) plus slack.
fn with_frame<R>(e: &mut Engine, locals: u32, body: impl FnOnce(&mut Engine, Frame) -> R) -> R {
    e.with_stack(locals + 0x80, |e, block| {
        body(e, Frame(block.addr() + locals + 0x40))
    })
}

/// The scope timer every solver opens (`ECX` = the timer, stack: kind, 1,
/// source file name, line); closed by [`TIMER_SCOPE_DESTROY`].
const TIMER_SCOPE_CONSTRUCT: u32 = 0x0040_4eb0;
const TIMER_SCOPE_DESTROY: u32 = 0x0040_4ee0;
/// The timer kind every solver passes.
const TIMER_SCOPE_KIND: u32 = 0x2d;
/// `D:\_Fallout3\Platforms\Common\Code\Fallout Shared\Pathfinding\Pathfind.cpp`.
const PATHFIND_SOURCE_FILE_NAME: u32 = 0x0106_cb70;

/// Opens the scope timer at `timer` for source line `line`.
fn scope_enter(e: &mut Engine, timer: u32, line: u32) {
    e.call(
        TIMER_SCOPE_CONSTRUCT,
        &args![
            timer,
            TIMER_SCOPE_KIND,
            1u32,
            PATHFIND_SOURCE_FILE_NAME,
            line
        ],
    );
}

/// Closes the scope timer at `timer`.
fn scope_leave(e: &mut Engine, timer: u32) {
    e.call(TIMER_SCOPE_DESTROY, &args![timer]);
}

// General callees (cdecl unless noted; `ECX` is the first word otherwise).

/// `cdecl (size)`: the allocator, returns the block.
const MEMORY_ALLOCATE: u32 = 0x0040_1000;
/// `cdecl (block)`: frees a block from [`MEMORY_ALLOCATE`].
const MEMORY_FREE: u32 = 0x0040_1030;
/// `stdcall (array, element size, count, constructor)`: the compiler's
/// `vector constructor iterator`, runs a constructor over an array.
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x0040_1050;
/// `ECX` = setting object: the address of its `float` value.
const SETTING_FLOAT_ADDRESS: u32 = 0x0040_3e20;
/// `cdecl (a, b)`, `float` in `ST0`: `b`, or `a` when `b < a`.
const FLOAT_MAX: u32 = 0x0040_4010;
/// `cdecl (value)`, `float` in `ST0`: the absolute value.
const FLOAT_ABSOLUTE_VALUE: u32 = 0x0040_8840;
/// `ECX` = point, stack (x, y, z): stores the three floats.
const POINT3_CONSTRUCT: u32 = 0x0041_6870;
/// `ECX` = point, stack (scale): multiplies the point in place.
const POINT3_SCALE_IN_PLACE: u32 = 0x0043_9180;
/// `ECX` = point, stack (result, other): `*result = this + other`, returns
/// `result`.
const POINT3_ADD: u32 = 0x0043_9e90;
/// `ECX` = point, stack (result, other): `*result = this - other`, returns
/// `result`.
const POINT3_SUBTRACT: u32 = 0x0043_9ef0;
/// `ECX` = point, stack (result, scalar): `*result = this * scalar`, returns
/// `result`.
const POINT3_TIMES_SCALAR: u32 = 0x0045_bb20;
/// `ECX` = point: normalizes it in place (zero when shorter than 1e-6).
const POINT3_NORMALIZE: u32 = 0x004a_0c10;
/// `cdecl (result, scalar, vector)`: `*result = *vector * scalar`, returns
/// `result`.
const POINT3_SCALED_COPY: u32 = 0x004a_3760;
/// `ECX` = point: the squared length, in `ST0`.
const POINT3_LENGTH_SQUARED: u32 = 0x004a_7290;
/// `ECX` = point: `NiPoint3::UnitizeGetLength` (Xbox PDB), the length in
/// `ST0`.
const POINT3_UNITIZE_GET_LENGTH: u32 = 0x0045_7910;
/// `ECX` = point, stack (other): adds `other` in place.
const POINT3_ADD_ASSIGN: u32 = 0x0063_c8a0;
/// `ECX` = point: the `NiPoint3` constructor (does nothing), returns `ECX`.
const POINT3_DEFAULT_CONSTRUCTOR: u32 = 0x0068_15c0;
/// `cdecl (angle)`, `float` in `ST0`: one of the two trigonometric functions
/// of an angle in radians; `004e44b0` and `004e44d0` apply the same one.
const ANGLE_FUNCTION_004E44B0: u32 = 0x004e_44b0;
const ANGLE_FUNCTION_004E44D0: u32 = 0x004e_44d0;
/// `cdecl (angle)`, `float` in `ST0`: the other one; `004e4470`, `004e4490`
/// and `005b9e80` apply it.
const ANGLE_FUNCTION_004E4470: u32 = 0x004e_4470;
const ANGLE_FUNCTION_004E4490: u32 = 0x004e_4490;
const ANGLE_FUNCTION_005B9E80: u32 = 0x005b_9e80;
/// `cdecl (min, max)`, `float` in `ST0`: a random number in the range.
const RANDOM_FLOAT: u32 = 0x0047_6b70;
/// No arguments, `float` in `ST0`: a random angle.
const RANDOM_ANGLE: u32 = 0x004a_4240;
/// No arguments: the global `BSRandom` object.
const RANDOM_OBJECT: u32 = 0x0047_6c00;
/// `ECX` = the `BSRandom`: a random unsigned integer.
const RANDOM_UNSIGNED: u32 = 0x0047_6be0;

// Navmesh holders and navmeshes.

/// `ECX` = the holder (a `NiPointer` to a navmesh): constructs it empty.
const NAV_HOLDER_CONSTRUCT: u32 = 0x0042_fb00;
/// `ECX` = the holder: releases the navmesh it holds.
const NAV_HOLDER_RELEASE: u32 = 0x0042_fa40;
/// `ECX` = the holder: the navmesh pointer it holds (same body as
/// [`NI_POINTER_GET_FROM_FIELD`]).
const NAV_HOLDER_GET: u32 = 0x0045_8b50;
/// `ECX` = a `NiPointer` slot: the pointer it holds.
const NI_POINTER_GET_FROM_FIELD: u32 = 0x0055_9450;
/// `ECX` = navmesh, stack (out, triangle): `NavMesh::GetCenter` (Xbox PDB).
const NAVMESH_GET_CENTER: u32 = 0x0055_8220;
/// `ECX` = navmesh: its `NavMeshInfo` (the word at +0x104).
const NAVMESH_GET_INFO: u32 = 0x0068_efb0;
/// `ECX` = navmesh, stack (vertex index): the address of the vertex.
const NAVMESH_GET_VERTEX: u32 = 0x0068_f0a0;
/// `ECX` = navmesh, stack (triangle index): the address of the triangle
/// record.
const NAVMESH_GET_TRIANGLE: u32 = 0x0055_82f0;
/// `ECX` = triangle record, stack (corner): the vertex index (`u16`) of
/// the corner.
const TRIANGLE_GET_VERTEX_INDEX: u32 = 0x0055_82d0;
/// `cdecl (out, a, b, c)`: a random point in the triangle `a b c`, returns
/// `out`.
const TRIANGLE_RANDOM_POINT: u32 = 0x006b_bad0;

// Requests and locations.

/// `ECX` = request: the address of its `Origin` location (+0x0C).
const REQUEST_ORIGIN: u32 = 0x0048_d150;
/// `ECX` = request: the address of its `Destination` location (+0x34).
const REQUEST_DESTINATION: u32 = 0x006a_9540;
/// `ECX` = request: the `float` at +0x68 (`fActorRadius`), in `ST0`.
const REQUEST_ACTOR_RADIUS: u32 = 0x0069_ef80;
/// `ECX` = request: the pointer held by `spAvoidNodeArray` (+0x94).
const REQUEST_AVOID_NODE_ARRAY: u32 = 0x0069_e390;
/// `ECX` = request: the byte at +0xA0 (`bFirstTangentUsesHeading`).
const REQUEST_FIRST_TANGENT_USES_HEADING: u32 = 0x006c_a4c0;
/// `ECX` = request: the `float` at +0x90 (`fInitialPathHeading`), in `ST0`.
const REQUEST_INITIAL_PATH_HEADING: u32 = 0x006c_a4e0;
/// `ECX` = optimal-location request: the `float` at +0xBC
/// (`fAbsoluteMaxDistance`), in `ST0`.
const REQUEST_ABSOLUTE_MAXIMUM_DISTANCE: u32 = 0x0099_e040;
/// `ECX` = optimal-location request: the `float` at +0xB8
/// (`fAbsoluteMinDistance`), in `ST0`.
const REQUEST_ABSOLUTE_MINIMUM_DISTANCE: u32 = 0x0048_ee50;
/// `ECX` = request, stack (value): stores the `float` at +0x74
/// (`fTargetRadius`).
const REQUEST_SET_TARGET_RADIUS: u32 = 0x006d_3b00;
/// `ECX` = request, stack (value): stores the `float` at +0xB0.
const REQUEST_SET_MINIMUM_DISTANCE: u32 = 0x0050_7610;
/// `ECX` = request, stack (value): stores the `float` at +0xB4.
const REQUEST_SET_MAXIMUM_DISTANCE: u32 = 0x006e_5ee0;
/// `ECX` = request, stack (flag): stores the byte at +0xE1 (`bFinished` of
/// the covered-move request).
const REQUEST_SET_FINISHED: u32 = 0x0061_3fb0;
/// `ECX` = request (to construct), stack (source request):
/// `PathingRequest::PathingRequest_ov2` (Xbox PDB).
const REQUEST_COPY_CONSTRUCT: u32 = 0x006e_26a0;
/// `ECX` = request copy: its destructor.
const REQUEST_COPY_DESTRUCT: u32 = 0x006e_2620;
/// `ECX` = `PathingRequestSafeStraightLine`: constructor (Xbox PDB).
const SAFE_STRAIGHT_LINE_CONSTRUCT: u32 = 0x006e_5fa0;
/// `ECX` = `PathingRequestSafeStraightLine`: destructor.
const SAFE_STRAIGHT_LINE_DESTRUCT: u32 = 0x006d_3b20;
/// `cdecl (request, out, flag)`: the point search `fn_006d3780` runs on its
/// request, returns whether it found a point.
const SAFE_STRAIGHT_LINE_FIND: u32 = 0x006d_3b40;
/// `ECX` = covered-move request, stack (source request):
/// `PathingRequestCoveredMove::PathingRequestCoveredMove_ov2` (Xbox PDB).
const COVERED_MOVE_CONSTRUCT_COPY: u32 = 0x006e_4d20;
/// `ECX` = covered-move request: `PathingRequestCoveredMove::
/// ~PathingRequestCoveredMove` (Xbox PDB).
const COVERED_MOVE_DESTRUCT: u32 = 0x006e_4bf0;
/// `ECX` = covered-move request: the address of its `CoverLocation`
/// (this + 0xC8).
const COVERED_MOVE_COVER_POINT: u32 = 0x0063_77e0;
/// `cdecl (request, distance, flag)`: whether a covered move for the
/// request's destination is possible.
const COVERED_MOVE_CHECK: u32 = 0x0099_d480;

/// `ECX` = location: `PathingLocation::GetCell` (Xbox PDB).
const LOCATION_GET_CELL: u32 = 0x006d_d4f0;
/// `ECX` = anything with a word at +0x1C: returns it. Its Xbox PDB name is
/// `PathingLocation::GetWorldspace`; the linker folded the other getters of
/// that word onto it.
const LOCATION_GET_WORLDSPACE: u32 = 0x0044_1110;
/// `ECX` = location, stack (out): copies the position (+4) to `out`,
/// returns `out`.
const LOCATION_COPY_POSITION: u32 = 0x005c_3420;
/// `ECX` = location: `PathingLocation::ResolveToClosestNavmeshAndTriangle`
/// (Xbox PDB), returns whether it found one.
const LOCATION_RESOLVE_CLOSEST: u32 = 0x006d_dc00;
/// `ECX` = location, stack (holder, triangle `u16`): `PathingLocation::
/// GetNavMeshAndTriangle` (Xbox PDB).
const LOCATION_GET_NAVMESH_AND_TRIANGLE: u32 = 0x006d_d640;
/// `ECX` = location, stack (flag): `PathingLocation::ResolveNavMeshInfo`
/// (Xbox PDB), returns whether the location has navmesh info.
const LOCATION_RESOLVE_NAVMESH_INFO: u32 = 0x006d_d6f0;
/// `ECX` = location: whether bit 1 of the flags byte (+0x26) is set.
const LOCATION_HAS_FLAG_2: u32 = 0x006a_9520;
/// `ECX` = location, stack (other): `PathingLocation::operator=` (Xbox PDB).
const LOCATION_ASSIGN: u32 = 0x006d_cce0;
/// `ECX` = location, stack (other): the copy constructor
/// (`PathingLocation::PathingLocation_ov2`, Xbox PDB).
const LOCATION_COPY_CONSTRUCT: u32 = 0x006d_cc40;
/// `ECX` = location, stack (reference): `PathingLocation::
/// PathingLocation_ov3` (Xbox PDB), returns `ECX`.
const LOCATION_CONSTRUCT_AT_REFERENCE: u32 = 0x006d_cd70;
/// `ECX` = location, stack (position, cell, worldspace):
/// `PathingLocation::PathingLocation_ov7` (Xbox PDB).
const LOCATION_CONSTRUCT_FROM_PARTS: u32 = 0x006d_cee0;
/// `ECX` = location, stack (position, navmesh info, triangle `u16`):
/// constructs a location on a navmesh triangle.
const LOCATION_CONSTRUCT_FROM_MESH_POINT: u32 = 0x006d_d010;
/// `ECX` = location, stack (position, navmesh info): constructs a location
/// at a navmesh's point.
const LOCATION_CONSTRUCT_FROM_POINT_AND_INFO: u32 = 0x006d_cf20;
/// `ECX` = location, stack (position, other location): constructs a
/// location at a point, taking the rest from the other location.
const LOCATION_CONSTRUCT_FROM_POINT_AND_LOCATION: u32 = 0x006d_d0d0;
/// `ECX` = location: `PathingLocation::PathingLocation` (Xbox PDB).
const LOCATION_CONSTRUCT: u32 = 0x006d_cbb0;
/// `ECX` = location: its destructor.
const LOCATION_DESTRUCT: u32 = 0x004f_f7e0;
/// `ECX` = cell, stack (position, out height): `TESObjectCELL::
/// GetLandHeight` (Xbox PDB); returns whether the cell has land.
const CELL_GET_LAND_HEIGHT: u32 = 0x0055_47c0;
/// `ECX` = reference: `TESObjectREFR::GetScale` (Xbox PDB), in `ST0`.
const REFERENCE_GET_SCALE: u32 = 0x0056_7400;

// Solutions, nodes and arrays.

/// `ECX` = array, stack (): the element count of a `BSSimpleArray` (+8).
const ARRAY_COUNT: u32 = 0x0044_ddc0;
/// `ECX` = array: whether its element count (+8) is zero.
const ARRAY_IS_EMPTY: u32 = 0x0076_b610;
/// `ECX` = `BSSimpleArray`: reserves a slot, size + 1, returns the old
/// size.
const ARRAY_ADD_SLOT: u32 = 0x0097_8bc0;
/// `ECX` = array, stack (slot, count): constructs `count` 12-byte elements
/// from `slot` on.
const ARRAY_CONSTRUCT_ELEMENTS: u32 = 0x0049_f8f0;
/// `ECX` = array of 0x14-byte search nodes, stack (index): the address of
/// the element.
const NODE_ARRAY_ELEMENT: u32 = 0x006b_3160;
/// `ECX` = array, stack (0x20, 0, 0): constructs the first kind of scrap
/// node array.
const NODE_ARRAY_A_CONSTRUCT: u32 = 0x006a_d1b0;
/// `ECX` = array: destructor of the first kind.
const NODE_ARRAY_A_DESTRUCT: u32 = 0x006a_bc50;
/// `ECX` = array, stack (0x20, 0, 0): constructs the second kind.
const NODE_ARRAY_B_CONSTRUCT: u32 = 0x006a_d250;
/// `ECX` = array: destructor of the second kind.
const NODE_ARRAY_B_DESTRUCT: u32 = 0x006a_bde0;
/// `ECX` = array, stack (0x20, 0, 0): constructs the third kind.
const NODE_ARRAY_C_CONSTRUCT: u32 = 0x006d_b290;
/// `ECX` = array: destructor of the third kind.
const NODE_ARRAY_C_DESTRUCT: u32 = 0x006d_b350;
/// `ECX` = virtual node array: reserves a slot, returns its index.
const VIRTUAL_NODE_ARRAY_ADD_SLOT: u32 = 0x006c_ef10;
/// `ECX` = virtual node array, stack (index): the address of the 0x30-byte
/// element.
const VIRTUAL_NODE_ARRAY_ELEMENT: u32 = 0x006e_8f40;
/// `cdecl (size, block)`: placement `new`, returns the block.
const PLACEMENT_NEW: u32 = 0x006e_6da0;
/// `ECX` = virtual node, stack (location): constructs it from a location.
const VIRTUAL_NODE_CONSTRUCT: u32 = 0x006f_4d00;
/// `ECX` = virtual node, stack (location): `VirtualPathingNode::
/// SetLocation` (Xbox PDB).
const VIRTUAL_NODE_SET_LOCATION: u32 = 0x006f_5060;
/// `ECX` = solution, stack (position, navmesh info): appends a virtual node
/// for the point, returns its index.
const SOLUTION_ADD_VIRTUAL_NODE: u32 = 0x006c_99c0;
/// `ECX` = solution, stack (flag): stores `bIncompletePath` (+0x40).
const SOLUTION_SET_INCOMPLETE: u32 = 0x006c_9fa0;
/// `ECX` = solution: the number of current pathing nodes (the count of the
/// array at +0x20).
const SOLUTION_NODE_COUNT: u32 = 0x008b_6800;
/// `ECX` = solution: the number of virtual nodes (the count of the array
/// at +8).
const SOLUTION_VIRTUAL_NODE_COUNT: u32 = 0x005a_4320;
/// `ECX` = solution, stack (index): the address of the current pathing
/// node `index`.
const SOLUTION_NODE_AT: u32 = 0x006e_7970;
/// `ECX` = solution, stack (index): the address of the virtual node
/// `index`.
const SOLUTION_VIRTUAL_NODE_AT: u32 = 0x006e_78b0;
/// `ECX` = solution: clears it (virtual node indices to -1, incomplete flag
/// to 0, arrays emptied).
const SOLUTION_CLEAR: u32 = 0x006e_77b0;
/// `ECX` = solution (to construct).
const SOLUTION_CONSTRUCT: u32 = 0x006e_7650;
/// `ECX` = solution, stack (other solution): copies the indices, the
/// incomplete flag and the node arrays to the other solution.
const SOLUTION_COPY_TO: u32 = 0x006e_8ab0;
/// `ECX` = solution: its destructor.
const SOLUTION_DESTRUCT: u32 = 0x006e_7720;
/// `ECX` = pathing node: the address of its location (+4).
const NODE_LOCATION: u32 = 0x0071_7e50;
/// `ECX` = pathing node, stack (position): stores the position at +0x2C.
const NODE_SET_POSITION: u32 = 0x006b_0570;
/// `ECX` = search node (copied onto the stack), stack (out): constructs the
/// point (words 2 to 4 of the node) in `out`, returns `out`.
const NODE_TO_POSITION: u32 = 0x006a_6eb0;
/// `ECX` = avoid-node array, stack (position, out): the sum of the pushes
/// the avoid nodes give at the position, in `out`; a weight in `ST0`.
const AVOID_ARRAY_COMPUTE_PUSH: u32 = 0x006d_c9f0;
/// `cdecl (out, heading, location a, location b, radius)`: builds the
/// tangent point of the path's first node.
const BUILD_FIRST_TANGENT: u32 = 0x006d_99a0;
/// `cdecl (worldspace, cell, point, out)`: `Pathing::
/// FindClosestPointOnNavmesh` (engine map), returns whether it found one.
const FIND_CLOSEST_POINT_ON_NAVMESH: u32 = 0x006d_6f80;
/// `cdecl (location, radius, out)`: finds a point for the location (the
/// found point's position is at +0xC of `out`), returns whether it found
/// one.
const FIND_PATH_POINT: u32 = 0x006d_4570;
/// `ECX` = path point: constructor.
const PATH_POINT_CONSTRUCT: u32 = 0x006a_0480;

// Solvers' objects.

/// `ECX` = solver, stack (request, solution): constructs the base solver
/// (0x34 bytes).
const BASE_SOLVER_CONSTRUCT: u32 = 0x006c_8d30;
/// `ECX` = solver: runs it, returns whether it found a path.
const BASE_SOLVER_RUN: u32 = 0x006c_8e10;
/// `ECX` = navmesh list: constructor.
const NAVMESH_LIST_CONSTRUCT: u32 = 0x0042_f800;
/// `ECX` = navmesh list: destructor.
const NAVMESH_LIST_DESTRUCT: u32 = 0x0042_f830;
/// `ECX` = the base solver's member at +0x14: destructor.
const BASE_SOLVER_MEMBER_DESTRUCT: u32 = 0x005e_0540;
/// `cdecl (navmesh list)`: fills the list.
const NAVMESH_LIST_FILL: u32 = 0x006d_91d0;
/// `ECX` = `NavMeshSearchFlee`: constructor.
const FLEE_SEARCH_CONSTRUCT: u32 = 0x006a_9200;
/// `ECX` = flee search: destructor.
const FLEE_SEARCH_DESTRUCT: u32 = 0x006a_92c0;
/// `ECX` = flee search, stack (request, start, path nodes, goal nodes):
/// `NavMeshSearchFlee::BuildNodePath` (Xbox PDB).
const FLEE_SEARCH_BUILD_NODE_PATH: u32 = 0x006a_92e0;
/// `ECX` = path builder: constructor.
const PATH_BUILDER_CONSTRUCT: u32 = 0x006a_d4c0;
/// `ECX` = path builder: destructor.
const PATH_BUILDER_DESTRUCT: u32 = 0x006a_d690;
/// `ECX` = path builder, stack (request, path nodes, goal nodes, start
/// position, end position, solution, 0, 0): builds the solution, returns
/// whether it succeeded.
const PATH_BUILDER_BUILD: u32 = 0x006a_d770;
/// `ECX` = `NavMeshSearchClosePoint`: constructor.
const CLOSE_POINT_SEARCH_CONSTRUCT: u32 = 0x006a_9cb0;
/// `ECX` = close-point search: destructor.
const CLOSE_POINT_SEARCH_DESTRUCT: u32 = 0x006a_9d60;
/// `ECX` = close-point search, stack (request, start, path nodes, goal
/// nodes): builds the node path.
const CLOSE_POINT_SEARCH_BUILD_NODE_PATH: u32 = 0x006a_9d80;
/// `ECX` = `PathSmoother`: constructor.
const PATH_SMOOTHER_CONSTRUCT: u32 = 0x006e_d100;
/// `ECX` = path smoother, stack (request, path nodes, goal nodes, start
/// position, end position, solution, 0, 1): smooths the node path into the
/// solution.
const PATH_SMOOTHER_SMOOTH: u32 = 0x006e_d530;
/// `ECX` = path smoother: `PathSmoother::~PathSmoother` (Xbox PDB).
const PATH_SMOOTHER_DESTRUCT: u32 = 0x006e_d220;
/// `ECX` = `NavMeshSearchLOS`: constructor.
const LOS_SEARCH_CONSTRUCT: u32 = 0x006a_a620;
/// `ECX` = LOS search: destructor.
const LOS_SEARCH_DESTRUCT: u32 = 0x006a_a710;
/// `ECX` = LOS search, stack (request, start, path nodes, goal nodes):
/// `NavMeshSearchLOS::BuildNodePath` (Xbox PDB).
const LOS_SEARCH_BUILD_NODE_PATH: u32 = 0x006a_a780;
/// `ECX` = LOS search, stack (node, path nodes, goal nodes): fills the node
/// arrays from a node the search recorded, returns whether it found a
/// path.
const LOS_SEARCH_PATH_FROM_NODE: u32 = 0x006a_8290;
/// `ECX` = LOS map, stack (holder, triangle `u16` pointer): finds the
/// triangle the map points at, returns whether it found one.
const LOS_MAP_FIND_TRIANGLE: u32 = 0x006e_1770;
/// `ECX` = `NavMeshSearchHide`: constructor.
const HIDE_SEARCH_CONSTRUCT: u32 = 0x006a_c0e0;
/// `ECX` = hide search: destructor.
const HIDE_SEARCH_DESTRUCT: u32 = 0x006a_c1b0;
/// `ECX` = hide search, stack (request, origin start, destination start,
/// path nodes, goal nodes): builds the node path.
const HIDE_SEARCH_BUILD_NODE_PATH: u32 = 0x006a_c340;
/// `ECX` = `NavMeshSearchMaxCost`: constructor.
const MAX_COST_SEARCH_CONSTRUCT: u32 = 0x006a_b410;
/// `ECX` = max-cost search: destructor.
const MAX_COST_SEARCH_DESTRUCT: u32 = 0x006a_b480;
/// `ECX` = max-cost search, stack (request, start, goal location, path
/// nodes, goal nodes): `NavMeshSearchMaxCost::BuildNodePath` (Xbox PDB).
const MAX_COST_SEARCH_BUILD_NODE_PATH: u32 = 0x006a_b4a0;
/// `ECX` = max-cost search: the pointer at +0x2048 (the best node so far).
const MAX_COST_SEARCH_BEST_NODE: u32 = 0x006a_bb90;
/// `ECX` = the search `fn_006d34d0` runs: constructor.
const RANDOM_POINT_SEARCH_CONSTRUCT: u32 = 0x006a_8a80;
/// `ECX` = the same search: destructor.
const RANDOM_POINT_SEARCH_DESTRUCT: u32 = 0x006a_8b40;
/// `ECX` = the same search, stack (request, start, path nodes, goal nodes):
/// builds the node path.
const RANDOM_POINT_SEARCH_BUILD_NODE_PATH: u32 = 0x006a_8b60;
/// `ECX` = time stamp, stack (value): stores the value (`CombatTimeStamp`
/// constructor).
const TIME_STAMP_CONSTRUCT: u32 = 0x0043_5de0;
/// `ECX` = array: constructs the `BSSimpleArray<PathingCoverLocation>`.
const COVER_LOCATION_ARRAY_CONSTRUCT: u32 = 0x006d_ae60;

// Data.

/// The vtable `fn_006d3200` stores in a `PathingCoverLocation`.
const PATHING_COVER_LOCATION_VTABLE: u32 = 0x0106_cbc0;
/// The zero vector the game copies into fresh `NiPoint3`s.
const ZERO_VECTOR: u32 = 0x011f_426c;
/// The three angle settings `Pathing::Init` converts (setting objects).
const SETTING_ANGLE_A: u32 = 0x011f_2ba8;
const SETTING_ANGLE_B: u32 = 0x011f_2af8;
const SETTING_ANGLE_C: u32 = 0x011f_2b04;
/// The three converted angles `Pathing::Init` stores (`float`s).
const CONVERTED_ANGLE_A: u32 = 0x011a_5cf4;
const CONVERTED_ANGLE_B: u32 = 0x011a_5cf8;
const CONVERTED_ANGLE_C: u32 = 0x011a_5cfc;
/// `double` `pi / 180`.
const DEGREES_TO_RADIANS: u32 = 0x0102_3128;
/// `double` scale of a random unsigned number to an angle range.
const RANDOM_ANGLE_SCALE: u32 = 0x0101_e5a0;
/// `double` `pi` (as `float`), subtracted from the scaled random number.
const PI_AS_FLOAT: u32 = 0x0101_ff40;
/// `double` `2.0`.
const TWO: u32 = 0x0101_1590;
/// `double` `0.5`.
const HALF: u32 = 0x0101_1588;
/// `double` `0.9`.
const NINE_TENTHS: u32 = 0x0106_b9e8;
/// `float` `32.0`: the least reference size of the safe-straight-line
/// request.
const MINIMUM_RADIUS: u32 = 0x0101_e340;
/// `float` `512.0`: the distance `fn_006d2c80` asks the covered-move check
/// about.
const COVERED_MOVE_DISTANCE: u32 = 0x0102_31a0;
/// `float` `FLT_MAX`: `fn_006d32e0` stores its negation as the time stamp.
const FLOAT_MAXIMUM: u32 = 0x0106_cb30;
/// Global unit vectors `fn_006d12a0` scales to get a point on a circle.
const CIRCLE_AXIS_A: u32 = 0x011a_9478;
const CIRCLE_AXIS_B: u32 = 0x011a_946c;

// Callees and data of the second batch of translations (`006d3b00` on).

/// `ECX` = safe-straight-line request: the base destructor `fn_006d3b20`
/// runs.
const REQUEST_BASE_DESTRUCT: u32 = 0x006d_ad70;
/// `ECX` = location: whether it has navmesh info.
const LOCATION_HAS_NAVMESH_INFO: u32 = 0x006d_d6a0;
/// `ECX` = location: the word at +0x10 (its navmesh info).
const LOCATION_NAVMESH_INFO_FIELD: u32 = 0x0044_edb0;
/// `ECX` = the point search of `navmeshsearchslpoint.cpp` (constructed by
/// [`HIDE_SEARCH_CONSTRUCT`], destroyed by [`HIDE_SEARCH_DESTRUCT`]), stack
/// (request, start, goal): runs it, returns whether it found a point (the
/// point is at +0x20cc, see `fn_006d3ff0`).
const POINT_SEARCH_RUN: u32 = 0x006a_c220;
/// `ECX` = request (to construct): `PathingRequest::PathingRequest`
/// (Xbox PDB).
const REQUEST_CONSTRUCT: u32 = 0x006e_2420;
/// `ECX` = request, stack (target): `PathingRequest::CopyTo` (Xbox PDB).
const REQUEST_COPY_TO: u32 = 0x006e_2750;
/// `ECX` = close-point request (to construct):
/// `PathingRequestClosePoint::PathingRequestClosePoint` (Xbox PDB).
const CLOSE_POINT_REQUEST_CONSTRUCT: u32 = 0x006e_3f30;
/// `ECX` = solution, stack (last node index as `float`, minimum distance,
/// 0, address of a scratch word, out): looks a point up along the
/// solution's path and stores it in `out`; returns whether it found one.
const SOLUTION_QUERY_006E7E70: u32 = 0x006e_7e70;
/// `ECX` = holder, stack (other holder): copies the `NiPointer`.
const NAV_HOLDER_COPY_CONSTRUCT: u32 = 0x0042_fa20;
/// `ECX` = array of pointers, stack (index): the address of the element.
const POINTER_ARRAY_ELEMENT: u32 = 0x0087_7a30;
/// `ECX` = navmesh: the number of triangles.
const NAVMESH_TRIANGLE_COUNT: u32 = 0x0055_8200;
/// `ECX` = triangle record, stack (edge): the neighbouring triangle across
/// the edge as `u16`, `0xffff` for none.
const TRIANGLE_NEIGHBOUR: u32 = 0x0068_f2c0;
/// `ECX` = navmesh, stack (triangle `u16`, edge, out a, out b): the two
/// end points of the triangle's edge.
const NAVMESH_EDGE_POINTS: u32 = 0x0068_f0c0;
/// `cdecl (out, point, a, b)`: the point of the segment `a b` closest to
/// `point`.
const POINT_SEGMENT_CLOSEST: u32 = 0x006b_9c20;
/// `cdecl (point, a, b, flag)`, `float` in `ST0`: the distance from
/// `point` to the segment `a b`.
const POINT_SEGMENT_DISTANCE: u32 = 0x006b_9b10;
/// `ECX` = point: `x * x + y * y` in `ST0`.
const POINT_PLANAR_LENGTH_SQUARED: u32 = 0x0059_5c80;
/// `ECX` = point: its length, in `ST0`.
const POINT3_LENGTH: u32 = 0x0045_7990;
/// `ECX` = array of 0x20-byte edge candidates: constructor.
const CANDIDATE_ARRAY_CONSTRUCT: u32 = 0x006d_b3b0;
/// `ECX` = candidate array: destructor.
const CANDIDATE_ARRAY_DESTRUCT: u32 = 0x006d_b3e0;
/// `ECX` = candidate array, stack (candidate): appends a copy.
const CANDIDATE_ARRAY_ADD: u32 = 0x006d_b400;
/// `ECX` = candidate array, stack (index): the address of the element.
const CANDIDATE_ARRAY_ELEMENT: u32 = 0x006b_3120;
/// `cdecl (a, b)`: the smaller of two unsigned numbers.
const MINIMUM_UNSIGNED: u32 = 0x0042_f5a0;
/// `ECX` = scratch object, stack (candidate): converts a candidate to a
/// path point, returns it.
const CANDIDATE_TO_PATH_POINT: u32 = 0x0069_df60;
/// `ECX` = path point array, stack (path point): appends a copy.
const PATH_POINT_ARRAY_ADD: u32 = 0x006d_b450;
/// The C runtime's `qsort`: `cdecl (base, count, element size, compare)`.
const QSORT: u32 = 0x00ec_6f20;
/// `ECX` = path point array (`pathbuilder.cpp`): constructor.
const PATH_POINT_ARRAY_CONSTRUCT: u32 = 0x006c_f200;
/// `ECX` = path point array, stack (index): the address of the element.
const PATH_POINT_ARRAY_ELEMENT: u32 = 0x006c_ee10;
/// `ECX` = path point array: destructor.
const PATH_POINT_ARRAY_DESTRUCT: u32 = 0x006c_f230;
/// `ECX` = path point, stack (other path point): assigns it.
const PATH_POINT_ASSIGN: u32 = 0x006c_f900;
/// `ECX` = array of edge references: constructor.
const EDGE_ARRAY_CONSTRUCT: u32 = 0x0069_b2c0;
/// `ECX` = edge reference array: destructor.
const EDGE_ARRAY_DESTRUCT: u32 = 0x0069_b2f0;
/// `ECX` = edge reference array, stack (edge reference): appends a copy.
const EDGE_ARRAY_ADD: u32 = 0x0069_b310;
/// `ECX` = array of triangle records: constructor.
const TRIANGLE_ARRAY_CONSTRUCT: u32 = 0x006d_b4a0;
/// `ECX` = triangle record array: destructor.
const TRIANGLE_ARRAY_DESTRUCT: u32 = 0x006d_b4d0;
/// `ECX` = triangle record array, stack (record): appends a copy.
const TRIANGLE_ARRAY_ADD: u32 = 0x006d_b4f0;
/// `ECX` = edge or triangle array, stack (index): the address of the
/// element.
const ARRAY_ELEMENT_006A1440: u32 = 0x006a_1440;
/// `ECX` = triangle record: constructor (constructs the reference in its
/// first 8 bytes, a `NiPoint3`-style default constructor).
const TRIANGLE_RECORD_CONSTRUCT: u32 = 0x0062_40d0;
/// `ECX` = scratch object, stack (navmesh, triangle `u16`): a triangle
/// reference, returns it.
const TRIANGLE_REFERENCE_CONSTRUCT: u32 = 0x0069_a660;
/// `ECX` = reference, stack (other): copies the navmesh and triangle.
const TRIANGLE_REFERENCE_ASSIGN: u32 = 0x0069_a690;
/// `ECX` = edge reference (to construct), stack (navmesh, triangle `u16`,
/// edge).
const EDGE_REFERENCE_CONSTRUCT: u32 = 0x0069_a6c0;
/// `ECX` = navmesh, stack (edge reference, out reference):
/// `NavMesh::GetMatchingEdge_ov4` (Xbox PDB), returns whether there is a
/// matching edge across the triangle.
const NAVMESH_GET_MATCHING_EDGE: u32 = 0x0068_f670;
/// `ECX` = edge reference, stack (other): assigns it.
const EDGE_REFERENCE_ASSIGN: u32 = 0x006b_c870;
/// `ECX` = teleport path: `TeleportPath::TeleportPath` (Xbox PDB).
const TELEPORT_PATH_CONSTRUCT: u32 = 0x006f_48b0;
/// `ECX` = teleport path: `TeleportPath::~TeleportPath` (Xbox PDB).
const TELEPORT_PATH_DESTRUCT: u32 = 0x006f_4930;
/// `ECX` = teleport path: `TeleportPath::Clear` (Xbox PDB).
const TELEPORT_PATH_CLEAR: u32 = 0x006f_4990;
/// `ECX` = teleport path: `TeleportPath::ComputeLength` (Xbox PDB), in
/// `ST0`.
const TELEPORT_PATH_COMPUTE_LENGTH: u32 = 0x006f_49c0;
/// `ECX` = teleport path, stack (count): reserves room for the path's
/// first array.
const TELEPORT_PATH_RESERVE_STEPS: u32 = 0x006c_ee30;
/// `ECX` = teleport path + 0x10, stack (count): reserves room for the
/// path's second array.
const TELEPORT_PATH_RESERVE_DOORS: u32 = 0x006c_eea0;
/// `ECX` = `TeleportDoorSearch`: `TeleportDoorSearch::TeleportDoorSearch`
/// (Xbox PDB).
const TELEPORT_SEARCH_CONSTRUCT: u32 = 0x006f_33b0;
/// `ECX` = teleport search, stack (from, to, path, argument 4, `float`,
/// argument 6, argument 7): `TeleportDoorSearch::BuildNodePath` (Xbox
/// PDB), returns whether it found a path.
const TELEPORT_SEARCH_BUILD_NODE_PATH: u32 = 0x006f_34e0;
/// `ECX` = teleport search: `TeleportDoorSearch::~TeleportDoorSearch`
/// (Xbox PDB).
const TELEPORT_SEARCH_DESTRUCT: u32 = 0x006f_3460;
/// `ECX` = low-path search (`navmeshinfosearch.cpp`): constructor.
const LOW_PATH_SEARCH_CONSTRUCT: u32 = 0x006b_8340;
/// `ECX` = low-path search, stack (argument 4, from position, from info,
/// to position, to info, info array, reference array, argument 5): runs
/// it, returns whether it found a path.
const LOW_PATH_SEARCH_RUN: u32 = 0x006b_8c50;
/// `ECX` = low-path search: destructor.
const LOW_PATH_SEARCH_DESTRUCT: u32 = 0x006b_8420;
/// `ECX` = array of navmesh info pointers: constructor.
const INFO_ARRAY_CONSTRUCT: u32 = 0x005e_0510;
/// `ECX` = info array: destructor.
const INFO_ARRAY_DESTRUCT: u32 = 0x005e_0540;
/// `ECX` = info array, stack (address of the pointer): appends it
/// (the exe's map names it `BSSimpleArray<..>::AddUninitialized`).
const INFO_ARRAY_ADD: u32 = 0x007c_b2e0;
/// `ECX` = array of reference pointers: constructor.
const REFERENCE_ARRAY_CONSTRUCT: u32 = 0x005e_04c0;
/// `ECX` = reference array: destructor.
const REFERENCE_ARRAY_DESTRUCT: u32 = 0x005e_04f0;
/// `ECX` = navmesh info: a value the low path records for each step.
const INFO_VALUE_A: u32 = 0x0069_0800;
/// `ECX` = navmesh info: the second value the low path records.
const INFO_VALUE_B: u32 = 0x006b_77b0;
/// `ECX` = reference: the object whose position `NODE_LOCATION` returns
/// (`tesobjectrefr.cpp`).
const REFERENCE_GET_POSITION_OWNER: u32 = 0x0056_8e50;
/// `ECX` = path's first array, stack (step): appends a 12-byte step
/// (`flag`, value A, value B).
const PATH_STEP_ADD: u32 = 0x006c_f4c0;
/// `ECX` = path + 0x10, stack (door entry): appends a 0x10-byte entry.
const PATH_DOOR_ADD: u32 = 0x006d_ae00;
/// `cdecl (location, radius, navmesh list)`: fills the list for the
/// location and radius (`fn_006d4120` uses it with a setting radius).
const NEARBY_NAVMESHES_FILL: u32 = 0x006d_8a00;
/// `ECX` = setting object (the radius `fn_006d4120` searches around a
/// location).
const NEARBY_RADIUS_SETTING: u32 = 0x011d_74dc;
/// `float` 0.1: how far from an edge point towards the triangle's center
/// `fn_006d4120` puts its candidate.
const CENTER_FRACTION: u32 = 0x0101_e2bc;
/// `double` -64.0 and 180.0: the open height band `fn_006d4020` sorts
/// into first.
const HEIGHT_BAND_MINIMUM: u32 = 0x0106_b9f0;
const HEIGHT_BAND_MAXIMUM: u32 = 0x0104_ed58;
/// `float` `FLT_MAX`.
const FLOAT_LARGEST: u32 = 0x0101_6970;
/// `ECX` = cover request: `iMaxResults` (+0xC8).
const COVER_MAX_RESULTS: u32 = 0x009a_cce0;
/// `ECX` = results array (of `PathingCoverLocation`), stack (capacity):
/// reserves room.
const COVER_RESULTS_RESERVE: u32 = 0x006d_aeb0;
/// `ECX` = results array, stack (index, location): inserts a copy of the
/// location at the index.
const COVER_RESULTS_INSERT: u32 = 0x006d_af20;
/// `ECX` = array of `float` sort keys, stack (0x20, 0, 0): constructor.
const KEY_ARRAY_CONSTRUCT: u32 = 0x006d_b680;
/// `ECX` = key array, stack (index, address of the key): inserts it.
const KEY_ARRAY_INSERT: u32 = 0x006d_b540;
/// `ECX` = key array: destructor.
const KEY_ARRAY_DESTRUCT: u32 = 0x006d_b740;
/// `ECX` = cover request: the address of `ActorLocation` (+0xE0).
const COVER_ACTOR_LOCATION: u32 = 0x009d_9f40;
/// `ECX` = cover request: its `fActorHeight` (+0x6C), in `ST0`.
const COVER_ACTOR_HEIGHT: u32 = 0x009a_7c40;
/// `ECX` = cover request: `fMaxCoverDistance` (+0xBC), in `ST0`.
const COVER_MAX_DISTANCE: u32 = 0x0099_e040;
/// `ECX` = cover request: `fMaxHeightChange` (+0xC4), in `ST0`.
const COVER_MAX_HEIGHT_CHANGE: u32 = 0x0064_4a50;
/// `ECX` = cover request: the address of `DistanceProjectionVector`
/// (+0xD4).
const COVER_PROJECTION_VECTOR: u32 = 0x0092_3000;
/// `ECX` = cover request: `bSortDistanceFromTarget` (+0xD0).
const COVER_SORT_FROM_TARGET: u32 = 0x0055_7d30;
/// `ECX` = point, stack (other): `NiPoint3::operator==` (exact `float`
/// comparison of the three components).
const POINT3_EQUALS: u32 = 0x0043_90c0;
/// `ECX` = point, stack (other): `NiPoint3::operator!=`.
const POINT3_NOT_EQUALS: u32 = 0x0043_9090;
/// `ECX` = point, stack (other): `NiPoint3::Dot` (Xbox PDB), in `ST0`.
const POINT3_DOT: u32 = 0x004b_6190;
/// `ECX` = point, stack (result, other): the cross product, returns
/// `result`.
const POINT3_CROSS: u32 = 0x004b_3800;
/// `ECX` = `NiPoint2`, stack (x, y): constructor.
const POINT2_CONSTRUCT: u32 = 0x0045_2dc0;
/// `cdecl (height, 0, 1)`, returns a `u16`: the class of a height.
const HEIGHT_CLASS: u32 = 0x006a_d3b0;
/// `ECX` = array of navmesh pointers, stack (8, 0, 0): constructor.
const NAVMESH_LIST_B_CONSTRUCT: u32 = 0x006d_b7a0;
/// `ECX` = that array: destructor.
const NAVMESH_LIST_B_DESTRUCT: u32 = 0x006c_6ce0;
/// `cdecl (location, radius, navmesh list)`: the second function that
/// fills the list (see [`NEARBY_NAVMESHES_FILL`]).
const NEARBY_NAVMESHES_FILL_B: u32 = 0x006d_8a20;
/// `ECX` = holder, stack (other holder): assigns the `NiPointer`.
const NAV_HOLDER_ASSIGN: u32 = 0x0042_f4c0;
/// `ECX` = navmesh: the address of its cover triangle array (+0x78).
const NAVMESH_COVER_ARRAY: u32 = 0x0046_1110;
/// `ECX` = cover triangle array, stack (index): the address of the
/// `u16` triangle.
const COVER_ARRAY_ELEMENT: u32 = 0x0069_dec0;
/// `ECX` = triangle record: whether it carries cover edge flags (word at
/// +0xC and `0x0fbe0000`).
const TRIANGLE_IS_COVER: u32 = 0x0069_0770;
/// `ECX` = navmesh, stack (out, triangle): the triangle's normal, returns
/// the address.
const TRIANGLE_NORMAL: u32 = 0x0069_71e0;
/// `ECX` = triangle record, stack (edge, out class `u16`, out flag byte,
/// out flag byte): the cover information of an edge.
const TRIANGLE_EDGE_INFO: u32 = 0x0069_1040;
/// `ECX` = navmesh, stack (out two vertex pointers, triangle, edge):
/// `NavMesh::GetEdgeVertices` (Xbox PDB).
const NAVMESH_EDGE_VERTICES: u32 = 0x0068_f040;
/// `cdecl (cover position, threat, edge end a, edge end b)`, all planar
/// points: whether the cover position works against the threat.
const COVER_EDGE_TEST: u32 = 0x006b_a010;
/// `ECX` = `PathingCoverLocation`, stack (class, standing class, crouching
/// class, flag, flag, edge): sets its cover data.
const COVER_LOCATION_SET_COVER: u32 = 0x006e_4500;
/// `double` 0.0.
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// `float` 0.5: how far along an edge `fn_006d62e0` puts the cover.
const EDGE_MIDPOINT_FRACTION: u32 = 0x0101_6248;
/// The global `TES` pointer.
const TES_GLOBAL: u32 = 0x011d_ea10;
/// `ECX` = `TES`: `TES::GetNavMeshInfoMap` (Xbox PDB).
const TES_GET_NAVMESH_INFO_MAP: u32 = 0x0045_af00;
/// `ECX` = map, stack (cell): `NavMeshInfoMap::FindNavMeshInfoForCell`
/// (Xbox PDB).
const NAVMESH_INFO_MAP_FIND_FOR_CELL: u32 = 0x006b_77e0;
/// `ECX` = map, stack (two words): the other lookup of the map.
const NAVMESH_INFO_MAP_FIND_BY_PAIR: u32 = 0x006b_7850;
/// `ECX` = location, stack (holder): `PathingLocation::GetNavMesh`
/// (Xbox PDB).
const LOCATION_GET_NAVMESH: u32 = 0x006d_d610;
/// `cdecl (value)`, `float` in `ST0`: wrapper of `004019d0` (the square
/// root).
const SQUARE_ROOT_WRAPPER: u32 = 0x0040_19b0;
/// `ECX` = navmesh, stack (point, radius, out): whether it finds a point
/// near `point` and stores it in `out`.
const NAVMESH_PROBE_A: u32 = 0x0069_7c10;
/// `ECX` = navmesh, stack (point, 0): whether the point is acceptable.
const NAVMESH_PROBE_B: u32 = 0x0069_7670;
/// `ECX` = navmesh, stack (point, out triangle `u16`, out flag byte):
/// `NavMesh::FindClosestTriangleForLocation` (Xbox PDB).
const NAVMESH_FIND_TRIANGLE: u32 = 0x0069_6cf0;
/// `ECX` = navmesh, stack (triangle, previous edge, start, end, out hit):
/// the edge through which the planar segment leaves the triangle, -1 when
/// the end is inside.
const NAVMESH_CROSSED_EDGE: u32 = 0x0069_8060;
/// `ECX` = navmesh, stack (triangle, edge, out holder, out triangle `u16`,
/// out edge): `NavMesh::GetMatchingEdge_ov2` (Xbox PDB), returns whether
/// the edge has a neighbour.
const NAVMESH_GET_MATCHING_EDGE_B: u32 = 0x0068_f3b0;
/// `ECX` = edge reference (to construct), stack (other): the copy
/// constructor (12 bytes), returns `ECX`.
const EDGE_REFERENCE_COPY: u32 = 0x006c_4350;
/// `ECX` = navmesh, stack (triangle, edge): `NavMesh::GetEdgeExtraInfo`
/// (Xbox PDB), returns the address of the info or 0.
const NAVMESH_EDGE_EXTRA_INFO: u32 = 0x0068_f230;
/// `ECX` = location, stack (navmesh info): stores it.
const LOCATION_SET_NAVMESH_INFO: u32 = 0x006d_d3e0;
/// `ECX` = location, stack (triangle): stores it.
const LOCATION_SET_TRIANGLE: u32 = 0x006d_d4d0;
/// `ECX` = location, stack (position): `PathingLocation::SetLocation`
/// (Xbox PDB).
const LOCATION_SET_POSITION: u32 = 0x006d_d460;
/// `cdecl (out, a, b, point, point)`, planar points: returns the address
/// of a planar point computed from the segment `a b`.
const POINT2_SEGMENT_CLOSEST: u32 = 0x006b_a3d0;
/// `ECX` = holder, stack (other holder): whether the first words differ.
const HOLDER_DIFFERS: u32 = 0x0063_1820;
/// `ECX` = base solver, stack (two words): a method of the base solver
/// (`006c9170`) that `fn_006d5340` wraps.
const BASE_SOLVER_METHOD_006C9170: u32 = 0x006c_9170;
/// `ECX` = base solver: the method `fn_006d53b0` wraps.
const BASE_SOLVER_METHOD_006C93D0: u32 = 0x006c_93d0;
/// `ECX` = base solver, stack (one word): the method `fn_006d5420` wraps.
const BASE_SOLVER_METHOD_006C93F0: u32 = 0x006c_93f0;
/// `ECX` = the search of `navmeshsearch.cpp` `fn_006d5490` runs:
/// constructor.
const NAVMESH_SEARCH_CONSTRUCT: u32 = 0x006a_6a40;
/// `ECX` = that search, stack (request copy, start node, goal node, path
/// array, node array, info array): runs it, returns whether it found a
/// path.
const NAVMESH_SEARCH_RUN: u32 = 0x006a_6b70;
/// `ECX` = that search: destructor.
const NAVMESH_SEARCH_DESTRUCT: u32 = 0x006a_6b00;
/// `ECX` = array: default constructor of the first kind of node array
/// (`NODE_ARRAY_A_DESTRUCT` destroys it).
const NODE_ARRAY_A_DEFAULT_CONSTRUCT: u32 = 0x006a_bbb0;
/// `ECX` = path array, stack (0): called before the capacity check.
const PATH_ARRAY_PREPARE: u32 = 0x0084_54f0;
/// `ECX` = path array: its capacity.
const PATH_ARRAY_CAPACITY: u32 = 0x0084_e3a0;
/// `ECX` = path array, stack (capacity): reserves room.
const PATH_ARRAY_RESERVE: u32 = 0x006d_b0d0;
/// `ECX` = path array, stack (index): the address of the path node.
const PATH_ARRAY_ELEMENT: u32 = 0x006d_ad90;
/// `ECX` = path array, stack (first, count, 0): removes entries.
const PATH_ARRAY_REMOVE: u32 = 0x006d_b140;
/// `ECX` = node array, stack (first, count, 0): removes entries.
const NODE_ARRAY_REMOVE: u32 = 0x0049_f3c0;
/// `ECX` = node array, stack (index): the address of the entry (navmesh,
/// triangle `u16` at +4, edge at +8).
const NODE_ARRAY_B_ELEMENT: u32 = 0x006a_7af0;
/// `cdecl (portal end a, portal end b, from, current, out)`: narrows a
/// portal point; returns whether it wrote one to `out`.
const PORTAL_CLIP: u32 = 0x006b_a7e0;
/// `cdecl (from, to, left, right)`: whether the line still passes the
/// portal.
const LINE_REACHES: u32 = 0x0069_6540;
/// `cdecl (...)`: an empty function in this build (its Xbox PDB name is
/// `Error`); `fn_006d5490` calls it on the path while the debug flag is
/// set.
const DEBUG_HOOK: u32 = 0x0040_fbe0;
/// Byte: the debug flag `fn_006d5490` tests and clears.
const PATH_DEBUG_FLAG: u32 = 0x011d_74ec;
/// `float` 20.0 and 40.0: values `fn_006d5490` passes the debug hook.
const DEBUG_VALUE_ODD: u32 = 0x0101_7868;
const DEBUG_VALUE_FINAL: u32 = 0x0104_ef80;
/// `ECX` = the request's `spAvoidNodeArray` slot, stack (array): assigns
/// it.
const AVOID_NODE_ARRAY_ASSIGN: u32 = 0x006d_adb0;

// Callees and data of the third batch of translations (`006d7a40` on).

/// `ECX` = the max-cost search (`navmeshsearchmaxcost.cpp`, constructed by
/// [`MAX_COST_SEARCH_CONSTRUCT`]), stack (out point, from position, start
/// reference, to position, goal reference, limit as `float`): finds the
/// point reachable from the start reference closest to the goal.
const MAX_COST_SEARCH_FIND_POINT: u32 = 0x006a_b670;
/// `ECX` = `PathingLOSGridMap`: the `float` at +0x44 (`fRadius`), in `ST0`.
const LOS_GRID_MAP_RADIUS: u32 = 0x0050_8050;
/// `ECX` = anything with a `float` at +0x48, in `ST0`: the `fSightRadius` of
/// a `PathingLOSGridMap` and the `fRadius` of a `PathingLOSMap` (Xbox PDB).
const FLOAT_AT_OFFSET_0X48: u32 = 0x0063_9aa0;
/// `ECX` = `PathingLOSGridMap`, stack (out): copies the position of its
/// `Center` (the location at +0x1c) to `out`, returns `out`.
const LOS_GRID_MAP_COPY_CENTER: u32 = 0x006a_abd0;
/// `ECX` = `PathingLOSGridMap`, stack (holder of a `PathingLOSMap`): fills
/// the LOS map with the grid map's cells (`pathinglosgridmap.cpp`).
const LOS_GRID_MAP_FILL_LOS_MAP: u32 = 0x006e_07a0;
/// `ECX` = `PathingLOSGridMap`, stack (holder of a `PathingLOSMap`): merges
/// the LOS map into the grid map and stores `fPercentSeen` (+0x18).
const LOS_GRID_MAP_MERGE_LOS_MAP: u32 = 0x006e_0920;
/// `cdecl (size)`: the object allocator of the `NiSystem` unit.
const OBJECT_ALLOCATE: u32 = 0x00aa_13e0;
/// `ECX` = storage of 0x50 bytes, stack (center x, y, z, radius):
/// `PathingLOSMap::PathingLOSMap` (by body), returns `ECX`.
const LOS_MAP_CONSTRUCT: u32 = 0x006e_1370;
/// `ECX` = `NiPointer` slot, stack (object): stores the object and takes a
/// reference on it, returns `ECX`.
const NI_POINTER_CONSTRUCT_FROM: u32 = 0x0063_3c90;
/// `ECX` = `NiPointer` slot: releases the object it holds.
const NI_POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// `ECX` = `PathingLOSMap`: resets every score that is not positive to
/// 0x7f (`pathinglosmap.cpp`).
const LOS_MAP_RESET_SCORES: u32 = 0x006e_2000;
/// `ECX` = `PathingLOSMap`, stack (out): copies `Center` (+0x3c) to `out`,
/// returns `out`.
const LOS_MAP_COPY_CENTER: u32 = 0x006a_b380;
/// `ECX` = `PathingLOSMap`, stack (navmesh form id, out offset): looks the
/// navmesh up in `NavMeshToOffsetMap` (+0x08), stores the offset of its
/// first triangle score, returns whether it is in the map.
const LOS_MAP_FIND_OFFSET: u32 = 0x006e_1650;
/// `ECX` = `PathingLOSMap`: `iMaxScore` (+0x38).
const LOS_MAP_MAX_SCORE: u32 = 0x007a_9280;
/// `ECX` = `PathingLOSMap`: `iMinScore` (+0x39).
const LOS_MAP_MIN_SCORE: u32 = 0x006a_abf0;
/// `ECX` = `PathingLOSMap`, stack (score): stores `iMaxScore`.
const LOS_MAP_SET_MAX_SCORE: u32 = 0x0044_10d0;
/// `ECX` = `PathingLOSMap`, stack (score): stores `iMinScore`.
const LOS_MAP_SET_MIN_SCORE: u32 = 0x0044_10f0;
/// `ECX` = `PathingLOSMap`, stack (count): stores `iTotalTrianglesSeen`
/// (+0x30).
const LOS_MAP_SET_SEEN_COUNT: u32 = 0x005f_4bb0;
/// `ECX` = `PathingLOSMap`, stack (count): stores `iTotalTrianglesUnseen`
/// (+0x34).
const LOS_MAP_SET_UNSEEN_COUNT: u32 = 0x008d_7dc0;
/// `ECX` = `PathingLOSMap`: `iTotalTriangleCount` (+0x2c).
const LOS_MAP_TRIANGLE_COUNT: u32 = 0x0055_b980;
/// `ECX` = object, stack (): the word at +0xC: the form id of a navmesh
/// (`TESForm::iFormID`, Xbox PDB), the capacity of a `BSSimpleArray`.
const WORD_AT_OFFSET_0XC: u32 = 0x0084_e3a0;
/// `cdecl (angle, out sine, out cosine)`: the sine and cosine of an angle
/// (x87 `FSINCOS`).
const SINE_AND_COSINE: u32 = 0x0041_69a0;
/// `ECX` = array of 8-byte elements, stack (index): the address of the
/// element.
const SCORE_ENTRY_ARRAY_ELEMENT: u32 = 0x0069_50d0;
/// `ECX` = array of 8-byte elements, stack (element): appends a copy and
/// returns its index.
const SCORE_ENTRY_ARRAY_ADD: u32 = 0x006d_b840;
/// `ECX` = array of 8-byte elements, stack (0x40, 0, 0): constructor.
const SCORE_ENTRY_ARRAY_CONSTRUCT: u32 = 0x006d_b890;
/// `ECX` = array of 8-byte elements: destructor.
const SCORE_ENTRY_ARRAY_DESTRUCT: u32 = 0x006d_b950;
/// `ECX` = array of 16-byte triangle records, stack (index): the address of
/// the record.
const TRIANGLE_RECORD_ARRAY_ELEMENT: u32 = 0x0055_8dd0;
/// `ECX` = triangle record, stack (edge): whether the neighbour across the
/// edge (the `u16` at +6 + 2 * edge) is not 0xffff.
const TRIANGLE_HAS_NEIGHBOUR: u32 = 0x0068_f200;
/// `ECX` = triangle record, stack (mask): whether the mask and the word at
/// +0xC share a bit.
const TRIANGLE_HAS_FLAG: u32 = 0x0069_1140;
/// `ECX` = navmesh, stack (triangle `u16`, edge, out navmesh holder
/// address, out triangle `u16`, out edge): `NavMesh::GetMatchingEdge_ov3`
/// (Xbox PDB), returns whether there is a matching edge.
const NAVMESH_GET_MATCHING_EDGE_C: u32 = 0x0068_f460;
/// `double` 3.0: the weight of the height difference when `fn_006d7f40`
/// measures the distance to a triangle.
const LOS_HEIGHT_WEIGHT: u32 = 0x0102_1928;
/// `float` -1.0: the heading `fn_006d7ca0` passes for "no heading".
const NO_HEADING: u32 = 0x0101_2054;

/// `ECX` = array (to construct), stack (): the `BSSimpleArray` of cell
/// pointers (`fn_006d8a40`'s scratch list).
const CELL_LIST_CONSTRUCT: u32 = 0x006c_7530;
/// `ECX` = cell list: destructor.
const CELL_LIST_DESTRUCT: u32 = 0x006c_6de0;
/// `ECX` = cell: whether the interior flag (bit 0 of the byte at +0x24) is
/// set.
const CELL_IS_INTERIOR: u32 = 0x0042_5fd0;
/// `ECX` = cell: `TESObjectCELL::GetDataX` (Xbox PDB).
const CELL_GET_DATA_X: u32 = 0x0054_4c30;
/// `ECX` = cell: `TESObjectCELL::GetDataY` (Xbox PDB).
const CELL_GET_DATA_Y: u32 = 0x0054_4c60;
/// `ECX` = cell: `TESObjectCELL::GetWorldSpace` (Xbox PDB), null for an
/// interior cell.
const CELL_GET_WORLDSPACE: u32 = 0x0054_ddd0;
/// `ECX` = worldspace, stack (x, y): `TESWorldSpace::GetCellFromCellCoord`
/// (Xbox PDB).
const WORLDSPACE_GET_CELL_FROM_COORD: u32 = 0x0058_75a0;
/// `float` -`FLT_MAX`.
const FLOAT_MAXIMUM_NEGATIVE: u32 = 0x0101_5f5c;
/// `double` 4096.0: the width of an exterior cell.
const CELL_WIDTH: u32 = 0x0101_7a10;
/// `ECX` = cell: the pointer at +0x64 to the cell's navmesh array.
const CELL_NAVMESH_ARRAY: u32 = 0x0070_ec90;
/// `ECX` = navmesh array: its element count.
const NAVMESH_ARRAY_COUNT: u32 = 0x0062_0b80;
/// `ECX` = navmesh array, stack (out holder, index):
/// `NavMeshArray::GetNavMeshByIndex` (Xbox PDB), stores the element in the
/// holder and returns it.
const NAVMESH_ARRAY_GET: u32 = 0x0046_4f60;
/// `ECX` = list of navmesh holders, stack (holder): appends a copy and
/// returns its index.
const NAV_HOLDER_LIST_ADD: u32 = 0x0042_f850;
/// `ECX` = navmesh info, stack (holder): stores the navmesh of the info in
/// the holder, returns whether it is not null.
const NAVMESH_INFO_GET_NAVMESH: u32 = 0x0069_ad00;
/// `ECX` = holder list, stack (holder, start index, compare function):
/// the index of the first entry the function reports equal, -1 for none.
const HOLDER_LIST_FIND_INDEX: u32 = 0x0071_9b20;
/// The compare function `fn_006d8a40` hands to [`HOLDER_LIST_FIND_INDEX`].
const HOLDER_COMPARE_FUNCTION: u32 = 0x0042_ff10;
/// `ECX` = navmesh: the pointer at +0x24 (the cell of the navmesh).
const NAVMESH_GET_CELL: u32 = 0x0059_bb30;
/// `ECX` = navmesh, stack (out minimum, out maximum): the bounding box of
/// the navmesh, returns 1.
const NAVMESH_GET_BOUNDS: u32 = 0x0069_69e0;
/// `ECX` = the object, stack (): the interior cell pointer at +0x34 of the
/// `TES` object (the exe's map calls the folded body
/// `ActorMover::GetPreferredMoveMode`); also used on another form type.
const WORD_AT_OFFSET_0X34: u32 = 0x005f_36f0;
/// `ECX` = `TES`: the number of exterior cells.
const TES_EXTERIOR_CELL_COUNT: u32 = 0x0045_3980;
/// `ECX` = `TES`, stack (index): the exterior cell at the index.
const TES_EXTERIOR_CELL_AT: u32 = 0x0045_9470;
/// `ECX` = form: the form type byte (+4).
const FORM_TYPE: u32 = 0x0040_1170;
/// `ECX` = array, stack (capacity): `SetReservedSize` of a `BSSimpleArray`.
const ARRAY_SET_RESERVED_SIZE: u32 = 0x0084_b5c0;
/// `ECX` = the object `fn_006d9350` forwards to, stack (word, `float`,
/// array): the function of `tesobjectcell.cpp` it wraps.
const CELL_QUERY_006D9350: u32 = 0x0054_dc00;
/// `ECX` = array of triangle references, stack (): constructor.
const TRIANGLE_REFERENCE_ARRAY_CONSTRUCT: u32 = 0x006d_b9e0;
/// `ECX` = triangle reference array, stack (reference): appends a copy.
const TRIANGLE_REFERENCE_ARRAY_ADD: u32 = 0x006d_ba30;
/// `ECX` = triangle reference array: destructor.
const TRIANGLE_REFERENCE_ARRAY_DESTRUCT: u32 = 0x006d_ba10;
/// `ECX` = array of `u16`, stack (): constructor.
const U16_ARRAY_CONSTRUCT: u32 = 0x0069_17d0;
/// `ECX` = array of `u16`: destructor.
const U16_ARRAY_DESTRUCT: u32 = 0x0069_1800;
/// `ECX` = array of `u16`, stack (index): the address of the element.
const U16_ARRAY_ELEMENT: u32 = 0x005e_eda0;
/// `ECX` = navmesh, stack (minimum, maximum, out array of `u16`): the
/// triangles of the navmesh inside the box.
const NAVMESH_FIND_TRIANGLES_IN_BOX: u32 = 0x0069_6c00;
/// `ECX` = scratch object (vtable 0x0106b1b8), stack (): constructor, the
/// object is never used by `fn_006d9480`.
const SCRATCH_OBJECT_CONSTRUCT: u32 = 0x0069_af30;
/// `ECX` = that scratch object: destructor.
const SCRATCH_OBJECT_DESTRUCT: u32 = 0x0069_af60;
/// `cdecl (end a, end b, point, radius)`: whether the segment `a b` comes
/// within `radius` of the point.
const SEGMENT_WITHIN_RADIUS: u32 = 0x006b_a9e0;
/// `ECX` = location: whether it has navmesh info with a triangle other
/// than 0xffff.
const LOCATION_HAS_TRIANGLE: u32 = 0x006d_d6a0;
/// `ECX` = navmesh, stack (triangle `u16`, position, 180.0, 180.0, out
/// height): `NavMesh::ComputeTriangleZForLocation` (Xbox PDB), returns
/// whether the position is inside the triangle.
const NAVMESH_COMPUTE_TRIANGLE_Z: u32 = 0x0069_7980;
/// `float` 180.0: the two limits `fn_006d9830` passes to
/// [`NAVMESH_COMPUTE_TRIANGLE_Z`].
const TRIANGLE_Z_LIMIT: u32 = 0x0106_b158;

/// `ECX` = array of 0x28-byte locations, stack (index): the address of the
/// element.
const LOCATION_ARRAY_ELEMENT: u32 = 0x0069_1750;
/// `ECX` = array of pointers or `float`s, stack (index): the address of the
/// element (a thunk of [`POINTER_ARRAY_ELEMENT`]).
const POINTER_ARRAY_ELEMENT_THUNK: u32 = 0x006a_7ad0;
// Callees and data of the fourth batch of translations (`006d99a0` on).

/// `ECX` = 3 by 3 matrix (nine `float`s), stack (angle): the rotation about
/// the vertical axis: rows `(sin, cos, 0)`, `(-cos, sin, 0)`, `(0, 0, 1)`.
const MATRIX_FROM_HEADING: u32 = 0x004a_0c90;
/// `ECX` = matrix, stack (out, vector): `*out = matrix * *vector` (each
/// component is the dot product of a matrix row with the vector), returns
/// `out`.
const MATRIX_TIMES_VECTOR: u32 = 0x004b_4500;
/// `ECX` = point, stack (out, other): `NiPoint3::UnitCross` (Xbox PDB), the
/// normalized cross product (zero when the length is tiny), returns `out`.
const POINT3_UNIT_CROSS: u32 = 0x0053_d1a0;
/// `ECX` = point, stack (other): subtracts `other` in place.
const POINT3_SUBTRACT_ASSIGN: u32 = 0x0045_78c0;
/// Global vector `fn_006d99a0` and `fn_006da420` cross the path direction
/// with to get the sideways direction.
const TANGENT_CROSS_AXIS: u32 = 0x011a_9484;
/// `float` 256.0: how far ahead along the heading the tangent functions look.
const TANGENT_PROBE_DISTANCE: u32 = 0x0103_2adc;
/// `float` 1.2: the factor on the side distance `fn_006d99a0` pulls the end
/// point back by.
const TANGENT_PULL_BACK: u32 = 0x0101_8204;
/// `double` -0.7 (the `float` 0.7 widened): the least cosine between the
/// heading and the direction to the target `fn_006d99a0` accepts.
const TANGENT_MINIMUM_DOT: u32 = 0x0106_cbc8;
/// `double` 64.0: the distance to the target below which `fn_006d99a0`
/// refuses a target behind the heading.
const TANGENT_MINIMUM_LENGTH: u32 = 0x0102_40c0;
/// `ECX` = location (to construct), stack (position, cell):
/// `PathingLocation::PathingLocation_ov5` (Xbox PDB).
const LOCATION_CONSTRUCT_FROM_POSITION_AND_CELL: u32 = 0x006d_ce60;
/// `ECX` = location (to construct), stack (position, worldspace):
/// `PathingLocation::PathingLocation_ov6` (Xbox PDB).
const LOCATION_CONSTRUCT_FROM_POSITION_AND_WORLDSPACE: u32 = 0x006d_cea0;
/// `ECX` = cover request (to construct):
/// `PathingRequestCover::PathingRequestCover` (Xbox PDB).
const COVER_REQUEST_CONSTRUCT: u32 = 0x006e_4080;
/// `float` 500.0: the half-width of the square `fn_006da7c0` draws random
/// points in for an interior cell.
const PROFILE_INTERIOR_HALF_WIDTH: u32 = 0x0101_3d84;
/// `float` 2000.0: the search radius `fn_006da7c0` gives
/// `Pathing::FindPointOnNavMesh`.
const PROFILE_POINT_SEARCH_RADIUS: u32 = 0x0101_3970;
/// `ECX` = array of `NiPoint3` (to construct), stack (capacity).
const POINT_ARRAY_CONSTRUCT: u32 = 0x006a_65e0;
/// `ECX` = array of `NiPoint3`, stack (index): the address of the element.
const POINT_ARRAY_ELEMENT: u32 = 0x006a_1440;
/// `ECX` = array of `NiPoint3`: destructor.
const POINT_ARRAY_DESTRUCT: u32 = 0x0049_f1f0;
/// `cdecl (a, b, c, d, e)`, planar points: the polygon test the profiler
/// compares against [`POLYGON_TEST_B`].
const POLYGON_TEST_A: u32 = 0x006b_9e90;
/// `cdecl (a, b, c, d)`, planar points: the second implementation of the
/// polygon test.
const POLYGON_TEST_B: u32 = 0x006b_a010;
/// `cdecl (message)`: prints a debug line (in this build it does nothing).
const DEBUG_PRINT: u32 = 0x005b_5e40;
/// The message `fn_006da7c0` prints when the two polygon tests disagree.
const PROFILE_MISMATCH_MESSAGE: u32 = 0x0106_cbd0;

/// `ECX` = array (to construct), stack (0, 0): the initialization
/// `BSSimpleArray<PathingCoverLocation,1024>`'s constructor runs after
/// setting its vtable.
const COVER_LOCATION_ARRAY_INIT: u32 = 0x006d_c070;
/// `ECX` = cover location array, stack (1): `Clear`-style teardown.
const COVER_LOCATION_ARRAY_CLEAR: u32 = 0x006d_bde0;
/// `ECX` = cover location array, stack (first element, count): destroys
/// the elements.
const COVER_LOCATIONS_DESTROY_RANGE: u32 = 0x006d_bee0;
/// `ECX` = cover location array, stack (capacity, size): moves the array to
/// a buffer of that capacity.
const COVER_LOCATIONS_REALLOCATE: u32 = 0x006d_bfc0;
/// `ECX` = cover location array, stack (first element, count): constructs
/// the elements.
const COVER_LOCATIONS_CONSTRUCT: u32 = 0x006d_be40;
/// `ECX` = cover location array, stack (destination, source, count):
/// moves elements (forwards or backwards as the ranges require).
const COVER_LOCATIONS_COPY: u32 = 0x006d_bf20;
/// `ECX` = cover location array, stack (location): appends a copy, returns
/// its index.
const COVER_LOCATIONS_ADD: u32 = 0x006d_bd90;
/// `ECX` = cover location, stack (other): `PathingCoverLocation`'s
/// assignment.
const COVER_LOCATION_ASSIGN: u32 = 0x006e_4430;
/// `ECX` = array: whether its size (+8) equals its capacity (+0xC).
const ARRAY_IS_FULL: u32 = 0x0043_8b90;
/// `ECX` = array: the capacity a full array grows to (double up to 1024,
/// then plus 1024).
const ARRAY_GROWN_CAPACITY: u32 = 0x009a_3910;
/// `ECX` = array: frees its buffer (virtual slot 8 of the array, with the
/// buffer) and clears the pointer.
const ARRAY_FREE_BUFFER: u32 = 0x006a_8500;
/// `ECX` = array of 0x14-byte entries, stack (first element, count):
/// destroys the elements.
const PATH_ELEMENTS_DESTROY_RANGE: u32 = 0x0072_ba80;
/// `ECX` = array of 0x14-byte entries, stack (destination, source, count):
/// moves elements.
const PATH_ELEMENTS_COPY: u32 = 0x009a_3b20;
/// `ECX` = array of 0x14-byte entries, stack (capacity, size): moves the
/// array to a buffer of that capacity.
const PATH_ELEMENTS_REALLOCATE: u32 = 0x009a_3c50;
/// `ECX` = array: whether it is nearly empty for its capacity (it may
/// shrink): above 1024 slots when the size is at most capacity - 2048, else
/// when the size is at most a quarter of the capacity.
const ARRAY_MAY_SHRINK: u32 = 0x006f_3170;
/// `ECX` = array: the capacity a shrinking array moves to (half, or
/// capacity - 1024 above 2048).
const ARRAY_SHRUNK_CAPACITY: u32 = 0x0086_9600;
/// `ECX` = array, stack (free the buffer): destroys all elements and
/// empties the array.
const ARRAY_CLEAR: u32 = 0x0084_54f0;
/// `ECX` = array of 0x10-byte entries: appends a slot, returns its index.
const DOOR_ARRAY_ADD_SLOT: u32 = 0x006f_31f0;
/// `ECX` = array of 0x10-byte entries, stack (first element, count):
/// constructs the elements.
const DOOR_ELEMENTS_CONSTRUCT: u32 = 0x006d_bcf0;
/// `ECX` = reference counter word (+0x10 of a counted object): takes a
/// reference.
const REFERENCE_ADD: u32 = 0x0040_f6e0;
/// `ECX` = reference counter word (+0x10 of a counted object): releases a
/// reference.
const REFERENCE_RELEASE: u32 = 0x0040_1970;
/// `ECX` = scrap-heap array (to construct): the base constructor.
const SCRAP_ARRAY_BASE_CONSTRUCT: u32 = 0x006d_c100;
/// `ECX` = array, stack (capacity, size): the array initialization that
/// reserves the buffer through virtual slot 4.
const ARRAY_INIT: u32 = 0x006b_3eb0;
/// No arguments: the address of the memory manager object
/// (`0x011f6238`).
const MEMORY_MANAGER_OBJECT: u32 = 0x0040_1020;
/// `ECX` = memory manager object: `MemoryManager::GetThreadScrapHeap`
/// (Xbox PDB).
const GET_THREAD_SCRAP_HEAP: u32 = 0x00aa_42e0;
/// `ECX` = scrap-heap array: the base destructor.
const SCRAP_ARRAY_BASE_DESTRUCT: u32 = 0x006d_b330;
/// `ECX` = candidate array (to construct), stack (0, 0): the base
/// initialization.
const CANDIDATE_ARRAY_INIT: u32 = 0x006d_c1d0;
/// Vtables of the scrap-heap array (`fn_006db290`) and of the candidate
/// array (`fn_006db3b0`).
const SCRAP_ARRAY_VTABLE: u32 = 0x0106_cc00;
const CANDIDATE_ARRAY_VTABLE: u32 = 0x0106_cc28;
/// The vtable `fn_006dae60` stores in a `BSSimpleArray<PathingCoverLocation,
/// 1024>`.
const COVER_LOCATION_ARRAY_VTABLE: u32 = 0x0106_cbec;
/// Vtables stored by the array constructors `fn_006db4a0` (the array whose
/// scalar deleting destructor `006dbb80` is `BSSimpleArray<NavmeshTriFan,
/// 1024>`), `fn_006db7a0`, `fn_006db890` / `fn_006db950` (scrap-heap
/// arrays; `006dbc90` names them `BSScrapArray<CrossedTriangle,1024>`) and
/// `fn_006db9e0` (`006dbcc0`: `BSSimpleArray<NavMeshTriHandle,1024>`).
const TRI_FAN_ARRAY_VTABLE: u32 = 0x0106_cc3c;
const FLOAT_SCRAP_ARRAY_VTABLE: u32 = 0x0106_c6e0;
const CROSSED_TRIANGLE_SCRAP_ARRAY_VTABLE: u32 = 0x0106_cc78;
const TRI_HANDLE_ARRAY_VTABLE: u32 = 0x0106_cca0;
/// `ECX` = array (to construct), stack (0, 0): the base initialization of
/// the array of 0xC-byte entries.
const TRI_FAN_ARRAY_INIT: u32 = 0x0069_ba70;
/// `ECX` = array of 0xC-byte entries, stack (first element, count):
/// constructs entries.
const TRI_FAN_ARRAY_CONSTRUCT: u32 = 0x0069_b9d0;
/// `ECX` = array of 0x20-byte entries: reserves a slot (size + 1), returns
/// the old size.
const EDGE_PROXY_ARRAY_ADD_SLOT: u32 = 0x006b_4060;
/// `ECX` = array of 0x20-byte entries, stack (first element, count):
/// constructs entries.
const EDGE_PROXY_ARRAY_CONSTRUCT: u32 = 0x006d_c130;
/// `ECX` = array of 0x18-byte entries: reserves a slot, returns the old
/// size.
const PATH_POINT_ARRAY_ADD_SLOT: u32 = 0x006d_c260;
/// `ECX` = array of 0x18-byte entries, stack (first element, count):
/// constructs entries.
const PATH_POINT_ENTRY_CONSTRUCT: u32 = 0x006d_05e0;
/// `ECX` = float array, stack (value): appends it (`fn_006db540` at the
/// end of the array).
const FLOAT_ARRAY_APPEND: u32 = 0x006d_c320;
/// `ECX` = float array, stack (destination, source, count): moves `count`
/// entries.
const FLOAT_ARRAY_COPY: u32 = 0x0042_fb60;
/// `ECX` = float array, stack (first element, count): constructs entries.
const FLOAT_ARRAY_CONSTRUCT: u32 = 0x0072_6bf0;
/// `ECX` = float scrap-heap array (to construct): the base constructor
/// (the body is shared with [`NAVMESH_LIST_CONSTRUCT`]).
const FLOAT_SCRAP_ARRAY_BASE_CONSTRUCT: u32 = NAVMESH_LIST_CONSTRUCT;
/// `ECX` = float array, stack (capacity, size): the array initialization.
const FLOAT_ARRAY_INIT: u32 = 0x0042_fcb0;
/// `ECX` = crossed-triangle scrap-heap array (to construct): the base
/// constructor.
const CROSSED_TRIANGLE_SCRAP_ARRAY_BASE_CONSTRUCT: u32 = 0x006d_c370;
/// `ECX` = array, stack (capacity, size): the array initialization of the
/// crossed-triangle scrap-heap array and of the triangle handle array.
const CROSSED_TRIANGLE_ARRAY_INIT: u32 = 0x006d_c440;
/// `ECX` = crossed-triangle scrap-heap array: the base destructor.
const CROSSED_TRIANGLE_SCRAP_ARRAY_BASE_DESTRUCT: u32 = 0x006d_b930;
/// `ECX` = float `BSSimpleArray`: destructor.
const FLOAT_SIMPLE_ARRAY_DESTRUCT: u32 = 0x006d_b720;
/// `ECX` = cover location array: reserves a slot, returns the old size.
const COVER_LOCATION_ARRAY_ADD_SLOT: u32 = 0x006d_c500;

/// `cdecl (block, bytes)`: resizes a block from [`MEMORY_ALLOCATE`] (the
/// `MemoryManager` reallocation), returns the new block.
const REALLOCATE_BLOCK: u32 = 0x0042_f5d0;
// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Copies `count` words from `source` to `destination`.
fn copy_words(e: &mut Engine, destination: u32, source: u32, count: u32) {
    for i in 0..count {
        let word = e.mem.u32(source + 4 * i);
        e.mem.set_u32(destination + 4 * i, word);
    }
}

/// Copies the five-word search node at `source` (a path node: navmesh
/// pointer, triangle in the low half of the second word, and three more
/// words) to `destination`.
fn copy_search_node(e: &mut Engine, destination: u32, source: u32) {
    copy_words(e, destination, source, 5);
}

/// The last element of a node array (`array[count - 1]`).
fn last_node(e: &mut Engine, array: u32) -> u32 {
    let count = e.call(ARRAY_COUNT, &args![array]).u32();
    e.call(NODE_ARRAY_ELEMENT, &args![array, count.wrapping_sub(1)])
        .u32()
}

/// The object an `NiPointer` slot holds (`00559450`).
fn held(e: &mut Engine, slot: u32) -> u32 {
    e.call(NI_POINTER_GET_FROM_FIELD, &args![slot]).u32()
}

/// Adds every navmesh of a cell's navmesh array to `list`, using the
/// holder at `scratch` for each in turn.
fn add_cell_navmeshes(e: &mut Engine, list: Ptr, navmeshes: u32, scratch: u32) {
    let mut index = 0u32;
    while index < e.call(NAVMESH_ARRAY_COUNT, &args![navmeshes]).u32() {
        let holder = e
            .call(NAVMESH_ARRAY_GET, &args![navmeshes, scratch, index])
            .u32();
        e.call(NAV_HOLDER_LIST_ADD, &args![list, holder]);
        e.call(NAV_HOLDER_RELEASE, &args![scratch]);
        index += 1;
    }
}

/// Destroys the locations a function built, the last one first.
fn destroy_locations(e: &mut Engine, locations: &[u32]) {
    for location in locations.iter().rev() {
        e.call(LOCATION_DESTRUCT, &args![*location]);
    }
}

/// The exit of the tangent functions that answer with the zero vector:
/// stores it in `out`, destroys the built locations and returns `out`.
fn finish_with_zero(e: &mut Engine, locations: &[u32], out: Ptr) -> Ptr {
    copy_words(e, out.addr(), ZERO_VECTOR, 3);
    destroy_locations(e, locations);
    out
}

/// After a sideways line of sight from `places[0]` stopped at `places[1]`
/// (the location the line of sight wrote its stopping place to): when the
/// vector `ahead` is longer (squared) than the one from the line's start
/// to the stopping place, the end point becomes the stopping place. The
/// four frame `offsets` are the scratch points the game uses: the position
/// of the line's start, of the stopping place, their difference and the
/// position copied to the end point.
fn pull_end_to_hit(
    e: &mut Engine,
    places: [u32; 2],
    offsets: [i32; 4],
    f: Frame,
    ahead: u32,
    end: u32,
) {
    let line_start = e
        .call(LOCATION_COPY_POSITION, &args![places[0], f.at(offsets[0])])
        .u32();
    let stop = e
        .call(LOCATION_COPY_POSITION, &args![places[1], f.at(offsets[1])])
        .u32();
    let difference = e
        .call(POINT3_SUBTRACT, &args![stop, f.at(offsets[2]), line_start])
        .u32();
    let to_stop = e.call(POINT3_LENGTH_SQUARED, &args![difference]).f64();
    let straight = e.call(POINT3_LENGTH_SQUARED, &args![ahead]).f64();
    if straight > to_stop {
        let position = e
            .call(LOCATION_COPY_POSITION, &args![places[1], f.at(offsets[3])])
            .u32();
        copy_words(e, end, position, 3);
    }
}

// ---------------------------------------------------------------------------
// Functions
// ---------------------------------------------------------------------------

// Translated from 0049f210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<ParentSpaceNode,1024>::Add` (Xbox PDB): reserves a new
/// slot (`00978bc0` bumps the size and returns the old size), constructs the
/// 12-byte element in it (`0049f8f0`), copies `item` over it and returns the
/// index. The buffer is read again after the construction because it can
/// have been reallocated.
pub fn bs_simple_array_parent_space_node_add(
    e: &mut Engine,
    this: Ptr<crate::types::BSSimpleArray>,
    item: Ptr,
) -> i32 {
    let index = e.call(ARRAY_ADD_SLOT, &args![this]).u32();
    let buffer = e.get(this, crate::types::BSSimpleArray::pBuffer);
    let slot = index.wrapping_mul(12).wrapping_add(buffer);
    e.call(ARRAY_CONSTRUCT_ELEMENTS, &args![this, slot, 1u32]);
    let buffer = e.get(this, crate::types::BSSimpleArray::pBuffer);
    let destination = buffer.wrapping_add(index.wrapping_mul(12));
    copy_words(e, destination, item.addr(), 3);
    index as i32
}

// Translated from 006d0870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Pathing::Init` (Xbox PDB): converts three angle settings from degrees to
/// radians, applies the angle function (`005b9e80`) to each and stores the
/// results in three globals. Always returns true.
pub fn pathing_init(e: &mut Engine) -> bool {
    let degrees_to_radians = e.global::<f64>(DEGREES_TO_RADIANS);
    for (setting, destination) in [
        (SETTING_ANGLE_A, CONVERTED_ANGLE_A),
        (SETTING_ANGLE_B, CONVERTED_ANGLE_B),
        (SETTING_ANGLE_C, CONVERTED_ANGLE_C),
    ] {
        let value_address = e.call(SETTING_FLOAT_ADDRESS, &args![setting]).u32();
        let degrees = e.mem.f32(value_address);
        let radians = (degrees as f64 * degrees_to_radians) as f32;
        let result = e.call(ANGLE_FUNCTION_005B9E80, &args![radians]).f32();
        e.set_global(destination, result);
    }
    true
}

// Translated from 006d0900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Solves a `PathingRequest` into a `PathingSolution`: dispatches on the
/// request's `GetType` (virtual slot +0x10) to the solver of that type
/// (0 base, 3 flee, 4 close point, 5 line of sight, 6 hide, 7 optimal
/// location, 8 covered move: named after the search and request classes of
/// the Xbox PDB each solver uses; types 1, 2 and anything above 8 do
/// nothing and fail). Runs inside a scope timer for source line 0x6a.
pub fn fn_006d0900(
    e: &mut Engine,
    request: Ptr<PathingRequest>,
    solution: Ptr<PathingSolution>,
) -> bool {
    e.with_stack(0x10, |e, timer| {
        let timer = timer.addr();
        scope_enter(e, timer, 0x6a);
        let kind = e.vcall(request.addr(), 0x10, &args![]).u32();
        let found = match kind {
            0 => fn_006d0b10(e, request, solution),
            3 => fn_006d0be0(e, request, solution),
            4 => fn_006d1c60(e, request, solution),
            5 => fn_006d1660(e, request, solution),
            6 => fn_006d1f80(e, request, solution),
            7 => fn_006d23d0(e, request, solution),
            8 => fn_006d2c80(e, request, solution),
            _ => false,
        };
        scope_leave(e, timer);
        found
    })
}

// Translated from 006d0b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Solver for a type-0 request: builds the 0x34-byte base solver on the
/// stack for the request and the solution (`006c8d30`), runs it (`006c8e10`)
/// and destroys it (`fn_006d0b80`). Returns what the run returned.
pub fn fn_006d0b10(
    e: &mut Engine,
    request: Ptr<PathingRequest>,
    solution: Ptr<PathingSolution>,
) -> bool {
    with_frame(e, 0x38, |e, f| {
        let solver = f.at(-0x40);
        e.call(BASE_SOLVER_CONSTRUCT, &args![solver, request, solution]);
        let found = e.call(BASE_SOLVER_RUN, &args![solver]).bool();
        fn_006d0b80(e, Ptr::new(solver));
        found
    })
}

// Translated from 006d0b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the base solver: destroys its members at +0x24 and +0x14,
/// in that order.
pub fn fn_006d0b80(e: &mut Engine, this: Ptr) {
    e.call(NAVMESH_LIST_DESTRUCT, &args![this.addr() + 0x24]);
    e.call(BASE_SOLVER_MEMBER_DESTRUCT, &args![this.addr() + 0x14]);
}

// Translated from 006d0be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Solver for a type-3 request (flee): resolves the origin to a navmesh and
/// triangle (`fn_006d12a0` is the fallback whenever that fails: it picks a
/// point to run to without a search), runs `NavMeshSearchFlee::
/// BuildNodePath`, takes the center of the last triangle as the goal (when
/// `006d4570` finds a point for it and the goal is closer to that point than
/// the request's radius, the goal is pushed out to exactly that radius from
/// it) and builds the solution with the path builder.
///
/// When the destination was not resolved (flag 2 of its flags) the
/// destination is resolved and its navmesh fetched first. When the final
/// path has more than two nodes and the request asks for it
/// (`bFirstTangentUsesHeading`), the first tangent of the first node is set
/// from the request's heading.
pub fn fn_006d0be0(
    e: &mut Engine,
    request: Ptr<PathingRequest>,
    solution: Ptr<PathingSolution>,
) -> bool {
    with_frame(e, 0x4310, |e, f| {
        let timer = f.at(-0x20ec);
        let origin_holder = f.at(-0x20a0);
        let origin_triangle = f.at(-0x20f4);
        scope_enter(e, timer, 0xb3);
        let origin = e.call(REQUEST_ORIGIN, &args![request]).u32();
        if !e.call(LOCATION_RESOLVE_CLOSEST, &args![origin]).bool() {
            let found = fn_006d12a0(e, request, solution);
            scope_leave(e, timer);
            return found;
        }
        e.call(NAV_HOLDER_CONSTRUCT, &args![origin_holder]);
        e.mem.set_u16(origin_triangle, 0xffff);
        let origin = e.call(REQUEST_ORIGIN, &args![request]).u32();
        if !e
            .call(
                LOCATION_GET_NAVMESH_AND_TRIANGLE,
                &args![origin, origin_holder, origin_triangle],
            )
            .bool()
        {
            let found = fn_006d12a0(e, request, solution);
            e.call(NAV_HOLDER_RELEASE, &args![origin_holder]);
            scope_leave(e, timer);
            return found;
        }

        // The destination, unless it already carries navmesh info.
        let destination = e.call(REQUEST_DESTINATION, &args![request]).u32();
        if !e.call(LOCATION_HAS_FLAG_2, &args![destination]).bool() {
            let destination = e.call(REQUEST_DESTINATION, &args![request]).u32();
            if !e
                .call(LOCATION_RESOLVE_NAVMESH_INFO, &args![destination, 0u32])
                .bool()
            {
                let found = fn_006d12a0(e, request, solution);
                e.call(NAV_HOLDER_RELEASE, &args![origin_holder]);
                scope_leave(e, timer);
                return found;
            }
            let destination_holder = f.at(-0x4280);
            let destination_triangle = f.at(-0x4284);
            e.call(NAV_HOLDER_CONSTRUCT, &args![destination_holder]);
            e.mem.set_u16(destination_triangle, 0xffff);
            let destination = e.call(REQUEST_DESTINATION, &args![request]).u32();
            if !e
                .call(
                    LOCATION_GET_NAVMESH_AND_TRIANGLE,
                    &args![destination, destination_holder, destination_triangle],
                )
                .bool()
            {
                let found = fn_006d12a0(e, request, solution);
                e.call(NAV_HOLDER_RELEASE, &args![destination_holder]);
                e.call(NAV_HOLDER_RELEASE, &args![origin_holder]);
                scope_leave(e, timer);
                return found;
            }
            e.call(NAV_HOLDER_RELEASE, &args![destination_holder]);
        }

        // The start: the origin's navmesh and triangle.
        let start = f.at(-0x20e8);
        let navmesh = e.call(NAV_HOLDER_GET, &args![origin_holder]).u32();
        e.mem.set_u32(start, navmesh);
        let triangle = e.mem.u16(origin_triangle);
        e.mem.set_u16(start + 4, triangle);
        let start_position = f.at(-0x20ac);
        let origin = e.call(REQUEST_ORIGIN, &args![request]).u32();
        e.call(LOCATION_COPY_POSITION, &args![origin, start_position]);

        let search = f.at(-0x209c);
        e.call(FLEE_SEARCH_CONSTRUCT, &args![search]);
        let path_nodes = f.at(-0x2130);
        e.call(
            NODE_ARRAY_A_CONSTRUCT,
            &args![path_nodes, 0x20u32, 0u32, 0u32],
        );
        let goal_nodes = f.at(-0x20d4);
        e.call(
            NODE_ARRAY_B_CONSTRUCT,
            &args![goal_nodes, 0x20u32, 0u32, 0u32],
        );
        let built = e
            .call(
                FLEE_SEARCH_BUILD_NODE_PATH,
                &args![search, request, start, path_nodes, goal_nodes],
            )
            .bool();
        if !built {
            e.call(NODE_ARRAY_B_DESTRUCT, &args![goal_nodes]);
            e.call(NODE_ARRAY_A_DESTRUCT, &args![path_nodes]);
            e.call(FLEE_SEARCH_DESTRUCT, &args![search]);
            e.call(NAV_HOLDER_RELEASE, &args![origin_holder]);
            scope_leave(e, timer);
            return false;
        }

        // The goal: the center of the last triangle of the path.
        let last = f.at(-0x20c0);
        let element = last_node(e, path_nodes);
        copy_search_node(e, last, element);
        let goal = f.at(-0x427c);
        let last_navmesh = e.mem.u32(last);
        let last_triangle = e.mem.u16(last + 4) as u32;
        e.call(
            NAVMESH_GET_CENTER,
            &args![last_navmesh, goal, last_triangle],
        );
        let info = e.call(NAVMESH_GET_INFO, &args![last_navmesh]).u32();
        let goal_location = f.at(-0x211c);
        e.call(
            LOCATION_CONSTRUCT_FROM_POINT_AND_INFO,
            &args![goal_location, goal, info],
        );
        let found_point = f.at(-0x2148);
        e.call(PATH_POINT_CONSTRUCT, &args![found_point]);
        let radius = e.call(REQUEST_ACTOR_RADIUS, &args![request]).f32();
        if e.call(FIND_PATH_POINT, &args![goal_location, radius, found_point])
            .bool()
        {
            // Pull the goal to the request's radius from the found point.
            let offset = f.at(-0x4294);
            let found_position = found_point + 0xc;
            e.call(POINT3_SUBTRACT, &args![goal, offset, found_position]);
            let length = e.call(POINT3_UNITIZE_GET_LENGTH, &args![offset]).f32();
            let radius = e.call(REQUEST_ACTOR_RADIUS, &args![request]).f32();
            if length < radius {
                let radius = e.call(REQUEST_ACTOR_RADIUS, &args![request]).f32();
                let scaled_result = f.at(-0x430c);
                let scaled = e
                    .call(POINT3_TIMES_SCALAR, &args![offset, scaled_result, radius])
                    .u32();
                let sum_result = f.at(-0x4318);
                let sum = e
                    .call(POINT3_ADD, &args![found_position, sum_result, scaled])
                    .u32();
                copy_words(e, goal, sum, 3);
            }
        }

        let start_navmesh = e.mem.u32(start);
        let info = e.call(NAVMESH_GET_INFO, &args![start_navmesh]).u32();
        e.call(
            SOLUTION_ADD_VIRTUAL_NODE,
            &args![solution, start_position, info],
        );
        let info = e.call(NAVMESH_GET_INFO, &args![last_navmesh]).u32();
        e.call(SOLUTION_ADD_VIRTUAL_NODE, &args![solution, goal, info]);
        e.set(solution, PathingSolution::iFirstLoadedVirtualNodeIndex, 0);
        e.set(solution, PathingSolution::iLastLoadedVirtualNodeIndex, 1);

        let builder = f.at(-0x425c);
        e.call(PATH_BUILDER_CONSTRUCT, &args![builder]);
        let extra_nodes = f.at(-0x4270);
        e.call(
            NODE_ARRAY_C_CONSTRUCT,
            &args![extra_nodes, 0x20u32, 0u32, 0u32],
        );
        let found = e
            .call(
                PATH_BUILDER_BUILD,
                &args![
                    builder,
                    request,
                    path_nodes,
                    goal_nodes,
                    start_position,
                    goal,
                    solution,
                    0u32,
                    0u32
                ],
            )
            .bool();
        if !found {
            e.call(SOLUTION_SET_INCOMPLETE, &args![solution, 1u32]);
        } else if e
            .call(REQUEST_FIRST_TANGENT_USES_HEADING, &args![request])
            .bool()
            && e.call(SOLUTION_NODE_COUNT, &args![solution]).u32() > 2
        {
            let first_node = e.call(SOLUTION_NODE_AT, &args![solution, 0u32]).u32();
            let second_node = e.call(SOLUTION_NODE_AT, &args![solution, 1u32]).u32();
            if first_node != 0 && second_node != 0 {
                let first_location = e.call(NODE_LOCATION, &args![first_node]).u32();
                let first_copy = f.at(-0x42f8);
                e.call(LOCATION_COPY_CONSTRUCT, &args![first_copy, first_location]);
                let second_location = e.call(NODE_LOCATION, &args![second_node]).u32();
                let second_copy = f.at(-0x42c4);
                e.call(
                    LOCATION_COPY_CONSTRUCT,
                    &args![second_copy, second_location],
                );
                // The radius is read for the tangent builder's last word.
                let radius = e.call(REQUEST_ACTOR_RADIUS, &args![request]).f32();
                let heading = e.call(REQUEST_INITIAL_PATH_HEADING, &args![request]).f32();
                let tangent = f.at(-0x42d0);
                e.call(
                    BUILD_FIRST_TANGENT,
                    &args![tangent, heading, first_copy, second_copy, radius],
                );
                e.call(NODE_SET_POSITION, &args![first_node, tangent]);
                e.call(LOCATION_DESTRUCT, &args![second_copy]);
                e.call(LOCATION_DESTRUCT, &args![first_copy]);
            }
        }

        e.call(NODE_ARRAY_C_DESTRUCT, &args![extra_nodes]);
        e.call(PATH_BUILDER_DESTRUCT, &args![builder]);
        e.call(LOCATION_DESTRUCT, &args![goal_location]);
        e.call(NODE_ARRAY_B_DESTRUCT, &args![goal_nodes]);
        e.call(NODE_ARRAY_A_DESTRUCT, &args![path_nodes]);
        e.call(FLEE_SEARCH_DESTRUCT, &args![search]);
        e.call(NAV_HOLDER_RELEASE, &args![origin_holder]);
        scope_leave(e, timer);
        found
    })
}

// Translated from 006d12a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Flee without a search: picks a point `fMinFleeDistance` (the float at
/// +0xB0 of the request) away from the origin and adds the origin and that
/// point as the solution's two virtual nodes. Always returns true.
///
/// When the request has a non-empty avoid-node array the point lies in the
/// direction away from the avoid nodes (`origin - push`, normalized); else
/// it is a random point on the circle around the origin spanned by the two
/// global axes. If the origin has a cell, the point's height is replaced by
/// the land height there. Both points are appended with `fn_006d1520` and the
/// solution's loaded node range is set to 0..1.
pub fn fn_006d12a0(
    e: &mut Engine,
    request: Ptr<PathingRequest>,
    solution: Ptr<PathingSolution>,
) -> bool {
    with_frame(e, 0x94, |e, f| {
        let timer = f.at(-0x3c);
        scope_enter(e, timer, 0x12f);
        let position = f.at(-0x2c);
        let origin = e.call(REQUEST_ORIGIN, &args![request]).u32();
        e.call(LOCATION_COPY_POSITION, &args![origin, position]);
        let origin = e.call(REQUEST_ORIGIN, &args![request]).u32();
        let cell = e.call(LOCATION_GET_CELL, &args![origin]).u32();
        let origin = e.call(REQUEST_ORIGIN, &args![request]).u32();
        let worldspace = e.call(LOCATION_GET_WORLDSPACE, &args![origin]).u32();
        let push = f.at(-0x38);
        e.call(POINT3_DEFAULT_CONSTRUCTOR, &args![push]);
        let target = f.at(-0x1c);
        e.call(POINT3_DEFAULT_CONSTRUCTOR, &args![target]);

        let avoid = e.call(REQUEST_AVOID_NODE_ARRAY, &args![request]).u32();
        let has_avoid_nodes = avoid != 0 && {
            let avoid = e.call(REQUEST_AVOID_NODE_ARRAY, &args![request]).u32();
            !e.call(ARRAY_IS_EMPTY, &args![avoid]).bool()
        };
        if has_avoid_nodes {
            // Away from the avoid nodes: normalized (origin - push) times the
            // minimum distance, from the origin.
            let avoid = e.call(REQUEST_AVOID_NODE_ARRAY, &args![request]).u32();
            e.call(AVOID_ARRAY_COMPUTE_PUSH, &args![avoid, position, push]);
            let away = f.at(-0x54);
            let result = e.call(POINT3_SUBTRACT, &args![position, away, push]).u32();
            copy_words(e, target, result, 3);
            e.call(POINT3_NORMALIZE, &args![target]);
            let distance = fn_006d2c20(e, request.cast());
            e.call(POINT3_SCALE_IN_PLACE, &args![target, distance]);
            e.call(POINT3_ADD_ASSIGN, &args![target, position]);
        } else {
            copy_words(e, push, position, 3);
            let angle = fn_006d1600(e);
            let first_scale = e.call(ANGLE_FUNCTION_004E44B0, &args![angle]).f64();
            let distance = fn_006d2c20(e, request.cast()) as f64;
            let first_scale = (distance * first_scale) as f32;
            let first = f.at(-0x78);
            let first_axis = e
                .call(
                    POINT3_SCALED_COPY,
                    &args![first, first_scale, CIRCLE_AXIS_A],
                )
                .u32();
            let second_scale = e.call(ANGLE_FUNCTION_004E4470, &args![angle]).f64();
            let distance = fn_006d2c20(e, request.cast()) as f64;
            let second_scale = (distance * second_scale) as f32;
            let second = f.at(-0x60);
            let second_axis = e
                .call(
                    POINT3_SCALED_COPY,
                    &args![second, second_scale, CIRCLE_AXIS_B],
                )
                .u32();
            let partial = f.at(-0x6c);
            let partial_result = e.call(POINT3_ADD, &args![push, partial, second_axis]).u32();
            let total = f.at(-0x84);
            let result = e
                .call(POINT3_ADD, &args![partial_result, total, first_axis])
                .u32();
            copy_words(e, target, result, 3);
        }

        if cell != 0 {
            let height = f.at(-0x48);
            if e.call(CELL_GET_LAND_HEIGHT, &args![cell, target, height])
                .bool()
            {
                let height = e.mem.f32(height);
                e.mem.set_f32(target + 8, height);
            }
        }
        fn_006d1520(e, solution, Ptr::new(position), cell, worldspace);
        fn_006d1520(e, solution, Ptr::new(target), cell, worldspace);
        e.set(solution, PathingSolution::iFirstLoadedVirtualNodeIndex, 0);
        e.set(solution, PathingSolution::iLastLoadedVirtualNodeIndex, 1);
        scope_leave(e, timer);
        true
    })
}

// Translated from 006d1520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends a virtual node for a point to the solution: builds a
/// `PathingLocation` from the position, cell and worldspace
/// (`PathingLocation_ov7`), resolves its navmesh info, reserves a slot in
/// the solution's virtual node array (+8), constructs the 0x30-byte node
/// there from the location and returns the slot's index.
pub fn fn_006d1520(
    e: &mut Engine,
    this: Ptr<PathingSolution>,
    position: Ptr,
    cell: u32,
    worldspace: u32,
) -> u32 {
    with_frame(e, 0x44, |e, f| {
        let location = f.at(-0x3c);
        e.call(
            LOCATION_CONSTRUCT_FROM_PARTS,
            &args![location, position, cell, worldspace],
        );
        e.call(LOCATION_RESOLVE_NAVMESH_INFO, &args![location, 0u32]);
        let array = this.addr() + 8;
        let index = e.call(VIRTUAL_NODE_ARRAY_ADD_SLOT, &args![array]).u32();
        let element = e
            .call(VIRTUAL_NODE_ARRAY_ELEMENT, &args![array, index])
            .u32();
        let block = e.call(PLACEMENT_NEW, &args![0x30u32, element]).u32();
        if block != 0 {
            e.call(VIRTUAL_NODE_CONSTRUCT, &args![block, location]);
        }
        e.call(LOCATION_DESTRUCT, &args![location]);
        index
    })
}

// Translated from 006d1600 (decompiled, FalloutNV.exe 1.4.0.525)
/// A random angle: `fn_006d1620` on the global random generator.
pub fn fn_006d1600(e: &mut Engine) -> f32 {
    let random = e.call(RANDOM_OBJECT, &args![]).u32();
    fn_006d1620(e, Ptr::new(random))
}

// Translated from 006d1620 (decompiled, FalloutNV.exe 1.4.0.525)
/// A random angle in `[-pi, pi)`: a random unsigned number times the scale
/// at `0101e5a0`, minus `pi` (`0101ff40`), as a `float`.
pub fn fn_006d1620(e: &mut Engine, this: Ptr) -> f32 {
    let random = e.call(RANDOM_UNSIGNED, &args![this]).u32();
    let scale = e.global::<f64>(RANDOM_ANGLE_SCALE);
    let pi = e.global::<f64>(PI_AS_FLOAT);
    (random as f64 * scale - pi) as f32
}

// Translated from 006d1660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Solver for a type-5 request (line of sight): resolves the origin to a
/// navmesh and triangle (failing if it cannot), runs `NavMeshSearchLOS::
/// BuildNodePath`, and when that finds no path tries the search's three
/// fallback nodes in turn (the first ignored goal, the last valid node and
/// the closest node, each through `006a8290`), falling back to a failed
/// result when none gives a path.
///
/// On success the center of the path's last triangle becomes a copy of the
/// request's destination and the copy is solved again through
/// `fn_006d0900`. When that does not succeed but a fallback node was found,
/// the last virtual node of the solution is replaced by the point on the
/// LOS map's triangle.
pub fn fn_006d1660(
    e: &mut Engine,
    request: Ptr<PathingRequest>,
    solution: Ptr<PathingSolution>,
) -> bool {
    with_frame(e, 0x2244, |e, f| {
        let timer = f.at(-0x20f0);
        let holder = f.at(-0x20b0);
        let triangle = f.at(-0x20f8);
        scope_enter(e, timer, 0x17a);
        let origin = e.call(REQUEST_ORIGIN, &args![request]).u32();
        if !e.call(LOCATION_RESOLVE_CLOSEST, &args![origin]).bool() {
            scope_leave(e, timer);
            return false;
        }
        e.call(NAV_HOLDER_CONSTRUCT, &args![holder]);
        e.mem.set_u16(triangle, 0xffff);
        let origin = e.call(REQUEST_ORIGIN, &args![request]).u32();
        if !e
            .call(
                LOCATION_GET_NAVMESH_AND_TRIANGLE,
                &args![origin, holder, triangle],
            )
            .bool()
        {
            e.call(NAV_HOLDER_RELEASE, &args![holder]);
            scope_leave(e, timer);
            return false;
        }
        let start = f.at(-0x20ec);
        let navmesh = e.call(NAV_HOLDER_GET, &args![holder]).u32();
        e.mem.set_u32(start, navmesh);
        let start_triangle = e.mem.u16(triangle);
        e.mem.set_u16(start + 4, start_triangle);
        let position = f.at(-0x20c0);
        let origin = e.call(REQUEST_ORIGIN, &args![request]).u32();
        e.call(LOCATION_COPY_POSITION, &args![origin, position]);
        let found_first = f.at(-0x20b1);
        let found_other = f.at(-0x20d5);
        e.mem.set_u8(found_first, 0);
        e.mem.set_u8(found_other, 0);
        let search = f.at(-0x20ac);
        e.call(LOS_SEARCH_CONSTRUCT, &args![search]);
        let path_nodes = f.at(-0x210c);
        e.call(
            NODE_ARRAY_A_CONSTRUCT,
            &args![path_nodes, 0x20u32, 0u32, 0u32],
        );
        let goal_nodes = f.at(-0x20d4);
        e.call(
            NODE_ARRAY_B_CONSTRUCT,
            &args![goal_nodes, 0x20u32, 0u32, 0u32],
        );
        let succeeded = f.at(-0x20f1);
        e.mem.set_u8(succeeded, 1);
        let built = e
            .call(
                LOS_SEARCH_BUILD_NODE_PATH,
                &args![search, request, start, path_nodes, goal_nodes],
            )
            .bool();
        if !built {
            // The search's recorded nodes, one after the other.
            let node = fn_006d1be0(e, Ptr::new(search));
            if node != 0 {
                let node = fn_006d1be0(e, Ptr::new(search));
                let found = e
                    .call(
                        LOS_SEARCH_PATH_FROM_NODE,
                        &args![search, node, path_nodes, goal_nodes],
                    )
                    .u8();
                e.mem.set_u8(found_first, found);
            }
            if e.mem.u8(found_first) == 0 {
                let node = fn_006d1c00(e, Ptr::new(search));
                if node != 0 {
                    let node = fn_006d1c00(e, Ptr::new(search));
                    let found = e
                        .call(
                            LOS_SEARCH_PATH_FROM_NODE,
                            &args![search, node, path_nodes, goal_nodes],
                        )
                        .u8();
                    e.mem.set_u8(found_other, found);
                }
            }
            if e.mem.u8(found_first) == 0 && e.mem.u8(found_other) == 0 {
                let node = fn_006d1c20(e, Ptr::new(search));
                if node != 0 {
                    let node = fn_006d1c20(e, Ptr::new(search));
                    let found = e
                        .call(
                            LOS_SEARCH_PATH_FROM_NODE,
                            &args![search, node, path_nodes, goal_nodes],
                        )
                        .u8();
                    e.mem.set_u8(found_other, found);
                }
            }
            if e.mem.u8(found_first) == 0 && e.mem.u8(found_other) == 0 {
                e.mem.set_u8(succeeded, 0);
                e.mem.set_u8(found_other, 1);
            }
        }

        if e.mem.u8(succeeded) != 0 {
            // Solve again from the path's last triangle.
            let last = f.at(-0x2120);
            let element = last_node(e, path_nodes);
            copy_search_node(e, last, element);
            let center = f.at(-0x2208);
            let last_navmesh = e.mem.u32(last);
            let last_triangle = e.mem.u16(last + 4) as u32;
            e.call(
                NAVMESH_GET_CENTER,
                &args![last_navmesh, center, last_triangle],
            );
            e.call(SOLUTION_CLEAR, &args![solution]);
            let copy = f.at(-0x21fc);
            e.call(REQUEST_COPY_CONSTRUCT, &args![copy, request]);
            let info = e.call(NAVMESH_GET_INFO, &args![last_navmesh]).u32();
            let location = f.at(-0x2148);
            e.call(
                LOCATION_CONSTRUCT_FROM_MESH_POINT,
                &args![location, center, info, last_triangle],
            );
            fn_006d1bc0(e, Ptr::new(copy), Ptr::new(location));
            let solved = fn_006d0900(e, Ptr::new(copy), solution);
            e.mem.set_u8(succeeded, solved as u8);
            e.call(LOCATION_DESTRUCT, &args![location]);
            e.call(REQUEST_COPY_DESTRUCT, &args![copy]);
        }

        if e.mem.u8(succeeded) == 0 && (e.mem.u8(found_first) != 0 || e.mem.u8(found_other) != 0) {
            // Move the last virtual node to the LOS map's triangle.
            let map_holder = f.at(-0x2210);
            e.call(NAV_HOLDER_CONSTRUCT, &args![map_holder]);
            let map_triangle = f.at(-0x220c);
            e.mem.set_u16(map_triangle, 0);
            let map = fn_006d1c40(e, Ptr::new(search));
            if e.call(LOS_MAP_FIND_TRIANGLE, &args![map, map_holder, map_triangle])
                .bool()
            {
                // The word at +0x1C of the solution (the last loaded
                // virtual node index; the body is shared with
                // `PathingLocation::GetWorldspace`).
                let last_index = e.call(LOCATION_GET_WORLDSPACE, &args![solution]).u32();
                let use_index = last_index;
                let usable = last_index != u32::MAX
                    && e.call(SOLUTION_VIRTUAL_NODE_COUNT, &args![solution]).u32() != 0;
                if !usable {
                    let start_navmesh = e.mem.u32(start);
                    let info = e.call(NAVMESH_GET_INFO, &args![start_navmesh]).u32();
                    e.call(SOLUTION_ADD_VIRTUAL_NODE, &args![solution, position, info]);
                    e.set(solution, PathingSolution::iLastLoadedVirtualNodeIndex, 0);
                }
                let node = e
                    .call(SOLUTION_VIRTUAL_NODE_AT, &args![solution, use_index])
                    .u32();
                if node != 0 {
                    let map_triangle_index = e.mem.u16(map_triangle) as u32;
                    let map_navmesh = e.call(NI_POINTER_GET_FROM_FIELD, &args![map_holder]).u32();
                    let triangle_center = f.at(-0x2224);
                    e.call(
                        NAVMESH_GET_CENTER,
                        &args![map_navmesh, triangle_center, map_triangle_index],
                    );
                    let map_navmesh = e.call(NI_POINTER_GET_FROM_FIELD, &args![map_holder]).u32();
                    let info = e.call(NAVMESH_GET_INFO, &args![map_navmesh]).u32();
                    let new_location = f.at(-0x224c);
                    e.call(
                        LOCATION_CONSTRUCT_FROM_MESH_POINT,
                        &args![new_location, triangle_center, info, map_triangle_index],
                    );
                    e.call(VIRTUAL_NODE_SET_LOCATION, &args![node, new_location]);
                    e.call(LOCATION_DESTRUCT, &args![new_location]);
                }
            }
            e.call(NAV_HOLDER_RELEASE, &args![map_holder]);
        }

        let result = e.mem.u8(succeeded) != 0;
        e.call(NODE_ARRAY_B_DESTRUCT, &args![goal_nodes]);
        e.call(NODE_ARRAY_A_DESTRUCT, &args![path_nodes]);
        e.call(LOS_SEARCH_DESTRUCT, &args![search]);
        e.call(NAV_HOLDER_RELEASE, &args![holder]);
        scope_leave(e, timer);
        result
    })
}

// Translated from 006d1bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assigns `location` to the request's `Destination` (+0x34).
pub fn fn_006d1bc0(e: &mut Engine, this: Ptr, location: Ptr) {
    e.call(LOCATION_ASSIGN, &args![this.addr() + 0x34, location]);
}

// Translated from 006d1be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at +0x2090 of the LOS search (`NavMeshSearchLOS`). By position
/// it is `pFirstIgnoredGoal` (+0x2094 in the Xbox PDB, which is four bytes
/// larger before this field).
pub fn fn_006d1be0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x2090)
}

// Translated from 006d1c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at +0x2094 of the LOS search (`pLastValidNode`, +0x2098 in the
/// Xbox PDB).
pub fn fn_006d1c00(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x2094)
}

// Translated from 006d1c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at +0x2098 of the LOS search (`pClosestNode`, +0x209c in the
/// Xbox PDB).
pub fn fn_006d1c20(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x2098)
}

// Translated from 006d1c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The pointer held by the `NiPointer` at +0x2070 of the LOS search
/// (`spLOSMap`, +0x2074 in the Xbox PDB).
pub fn fn_006d1c40(e: &mut Engine, this: Ptr) -> u32 {
    e.call(NI_POINTER_GET_FROM_FIELD, &args![this.addr() + 0x2070])
        .u32()
}

// Translated from 006d1c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Solver for a type-4 request (close point): requires the origin to have
/// navmesh info and a navmesh and triangle, runs the close-point search's
/// `BuildNodePath`, adds the origin and the center of the last triangle as
/// the solution's first two virtual nodes and smooths the node path into
/// the solution with a `PathSmoother`. Afterwards the loaded node range is
/// 0..(virtual node count - 1).
pub fn fn_006d1c60(
    e: &mut Engine,
    request: Ptr<PathingRequest>,
    solution: Ptr<PathingSolution>,
) -> bool {
    with_frame(e, 0x221c, |e, f| {
        let timer = f.at(-0x5c);
        scope_enter(e, timer, 0x1f5);
        let origin = e.call(REQUEST_ORIGIN, &args![request]).u32();
        if !e
            .call(LOCATION_RESOLVE_NAVMESH_INFO, &args![origin, 0u32])
            .bool()
        {
            scope_leave(e, timer);
            return false;
        }
        let holder = f.at(-0x10);
        let triangle = f.at(-0x60);
        e.call(NAV_HOLDER_CONSTRUCT, &args![holder]);
        e.mem.set_u16(triangle, 0xffff);
        let origin = e.call(REQUEST_ORIGIN, &args![request]).u32();
        if !e
            .call(
                LOCATION_GET_NAVMESH_AND_TRIANGLE,
                &args![origin, holder, triangle],
            )
            .bool()
        {
            e.call(NAV_HOLDER_RELEASE, &args![holder]);
            scope_leave(e, timer);
            return false;
        }
        let start = f.at(-0x58);
        let navmesh = e.call(NAV_HOLDER_GET, &args![holder]).u32();
        e.mem.set_u32(start, navmesh);
        let start_triangle = e.mem.u16(triangle);
        e.mem.set_u16(start + 4, start_triangle);
        let position = f.at(-0x1c);
        let origin = e.call(REQUEST_ORIGIN, &args![request]).u32();
        e.call(LOCATION_COPY_POSITION, &args![origin, position]);
        let search = f.at(-0x2204);
        e.call(CLOSE_POINT_SEARCH_CONSTRUCT, &args![search]);
        let path_nodes = f.at(-0x74);
        e.call(
            NODE_ARRAY_A_CONSTRUCT,
            &args![path_nodes, 0x20u32, 0u32, 0u32],
        );
        let goal_nodes = f.at(-0x44);
        e.call(
            NODE_ARRAY_B_CONSTRUCT,
            &args![goal_nodes, 0x20u32, 0u32, 0u32],
        );
        let built = e
            .call(
                CLOSE_POINT_SEARCH_BUILD_NODE_PATH,
                &args![search, request, start, path_nodes, goal_nodes],
            )
            .bool();
        if !built {
            e.call(NODE_ARRAY_B_DESTRUCT, &args![goal_nodes]);
            e.call(NODE_ARRAY_A_DESTRUCT, &args![path_nodes]);
            e.call(CLOSE_POINT_SEARCH_DESTRUCT, &args![search]);
            e.call(NAV_HOLDER_RELEASE, &args![holder]);
            scope_leave(e, timer);
            return false;
        }
        let last = f.at(-0x30);
        let element = last_node(e, path_nodes);
        copy_search_node(e, last, element);
        let center = f.at(-0x2224);
        let last_navmesh = e.mem.u32(last);
        let last_triangle = e.mem.u16(last + 4) as u32;
        e.call(
            NAVMESH_GET_CENTER,
            &args![last_navmesh, center, last_triangle],
        );
        let info = e.call(NAVMESH_GET_INFO, &args![navmesh]).u32();
        e.call(SOLUTION_ADD_VIRTUAL_NODE, &args![solution, position, info]);
        let info = e.call(NAVMESH_GET_INFO, &args![last_navmesh]).u32();
        e.call(SOLUTION_ADD_VIRTUAL_NODE, &args![solution, center, info]);
        e.set(solution, PathingSolution::iFirstLoadedVirtualNodeIndex, 0);
        e.set(solution, PathingSolution::iLastLoadedVirtualNodeIndex, 1);
        let smoother = f.at(-0x12c);
        e.call(PATH_SMOOTHER_CONSTRUCT, &args![smoother]);
        let extra_nodes = f.at(-0x2218);
        e.call(
            NODE_ARRAY_C_CONSTRUCT,
            &args![extra_nodes, 0x20u32, 0u32, 0u32],
        );
        e.call(
            PATH_SMOOTHER_SMOOTH,
            &args![
                smoother, request, path_nodes, goal_nodes, position, center, solution, 0u32, 1u32
            ],
        );
        e.set(solution, PathingSolution::iFirstLoadedVirtualNodeIndex, 0);
        let count = e.call(SOLUTION_VIRTUAL_NODE_COUNT, &args![solution]).u32();
        e.set(
            solution,
            PathingSolution::iLastLoadedVirtualNodeIndex,
            count.wrapping_sub(1) as i32,
        );
        e.call(NODE_ARRAY_C_DESTRUCT, &args![extra_nodes]);
        e.call(PATH_SMOOTHER_DESTRUCT, &args![smoother]);
        e.call(NODE_ARRAY_B_DESTRUCT, &args![goal_nodes]);
        e.call(NODE_ARRAY_A_DESTRUCT, &args![path_nodes]);
        e.call(CLOSE_POINT_SEARCH_DESTRUCT, &args![search]);
        e.call(NAV_HOLDER_RELEASE, &args![holder]);
        scope_leave(e, timer);
        true
    })
}

// Translated from 006d1f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Solver for a type-6 request (hide): like the close-point solver but the
/// search takes two starting navmeshes, the origin's and the destination's
/// (the destination must resolve to a navmesh and triangle too), and builds
/// the node path with the hide search's `BuildNodePath`.
pub fn fn_006d1f80(
    e: &mut Engine,
    request: Ptr<PathingRequest>,
    solution: Ptr<PathingSolution>,
) -> bool {
    with_frame(e, 0x2270, |e, f| {
        let timer = f.at(-0x2168);
        scope_enter(e, timer, 0x23f);
        let origin = e.call(REQUEST_ORIGIN, &args![request]).u32();
        if !e.call(LOCATION_RESOLVE_CLOSEST, &args![origin]).bool() {
            scope_leave(e, timer);
            return false;
        }
        let origin_holder = f.at(-0x5c);
        let origin_triangle = f.at(-0x2198);
        e.call(NAV_HOLDER_CONSTRUCT, &args![origin_holder]);
        e.mem.set_u16(origin_triangle, 0xffff);
        let origin = e.call(REQUEST_ORIGIN, &args![request]).u32();
        if !e
            .call(
                LOCATION_GET_NAVMESH_AND_TRIANGLE,
                &args![origin, origin_holder, origin_triangle],
            )
            .bool()
        {
            e.call(NAV_HOLDER_RELEASE, &args![origin_holder]);
            scope_leave(e, timer);
            return false;
        }
        let origin_start = f.at(-0x58);
        let origin_navmesh = e.call(NAV_HOLDER_GET, &args![origin_holder]).u32();
        e.mem.set_u32(origin_start, origin_navmesh);
        let triangle = e.mem.u16(origin_triangle);
        e.mem.set_u16(origin_start + 4, triangle);
        let position = f.at(-0x18);
        let origin = e.call(REQUEST_ORIGIN, &args![request]).u32();
        e.call(LOCATION_COPY_POSITION, &args![origin, position]);

        let destination = e.call(REQUEST_DESTINATION, &args![request]).u32();
        if !e.call(LOCATION_RESOLVE_CLOSEST, &args![destination]).bool() {
            e.call(NAV_HOLDER_RELEASE, &args![origin_holder]);
            scope_leave(e, timer);
            return false;
        }
        let destination_holder = f.at(-0x2194);
        let destination_triangle = f.at(-0x30);
        e.call(NAV_HOLDER_CONSTRUCT, &args![destination_holder]);
        e.mem.set_u16(destination_triangle, 0xffff);
        let destination = e.call(REQUEST_DESTINATION, &args![request]).u32();
        if !e
            .call(
                LOCATION_GET_NAVMESH_AND_TRIANGLE,
                &args![destination, destination_holder, destination_triangle],
            )
            .bool()
        {
            e.call(NAV_HOLDER_RELEASE, &args![destination_holder]);
            e.call(NAV_HOLDER_RELEASE, &args![origin_holder]);
            scope_leave(e, timer);
            return false;
        }
        let destination_start = f.at(-0x2190);
        let navmesh = e.call(NAV_HOLDER_GET, &args![destination_holder]).u32();
        e.mem.set_u32(destination_start, navmesh);
        let triangle = e.mem.u16(destination_triangle);
        e.mem.set_u16(destination_start + 4, triangle);

        let search = f.at(-0x2164);
        e.call(HIDE_SEARCH_CONSTRUCT, &args![search]);
        let path_nodes = f.at(-0x217c);
        e.call(
            NODE_ARRAY_A_CONSTRUCT,
            &args![path_nodes, 0x20u32, 0u32, 0u32],
        );
        let goal_nodes = f.at(-0x44);
        e.call(
            NODE_ARRAY_B_CONSTRUCT,
            &args![goal_nodes, 0x20u32, 0u32, 0u32],
        );
        let built = e
            .call(
                HIDE_SEARCH_BUILD_NODE_PATH,
                &args![
                    search,
                    request,
                    origin_start,
                    destination_start,
                    path_nodes,
                    goal_nodes
                ],
            )
            .bool();
        if !built {
            e.call(NODE_ARRAY_B_DESTRUCT, &args![goal_nodes]);
            e.call(NODE_ARRAY_A_DESTRUCT, &args![path_nodes]);
            e.call(HIDE_SEARCH_DESTRUCT, &args![search]);
            e.call(NAV_HOLDER_RELEASE, &args![destination_holder]);
            e.call(NAV_HOLDER_RELEASE, &args![origin_holder]);
            scope_leave(e, timer);
            return false;
        }
        let last = f.at(-0x2c);
        let element = last_node(e, path_nodes);
        copy_search_node(e, last, element);
        let center = f.at(-0x2274);
        let last_navmesh = e.mem.u32(last);
        let last_triangle = e.mem.u16(last + 4) as u32;
        e.call(
            NAVMESH_GET_CENTER,
            &args![last_navmesh, center, last_triangle],
        );
        let info = e.call(NAVMESH_GET_INFO, &args![origin_navmesh]).u32();
        e.call(SOLUTION_ADD_VIRTUAL_NODE, &args![solution, position, info]);
        let info = e.call(NAVMESH_GET_INFO, &args![last_navmesh]).u32();
        e.call(SOLUTION_ADD_VIRTUAL_NODE, &args![solution, center, info]);
        e.set(solution, PathingSolution::iFirstLoadedVirtualNodeIndex, 0);
        e.set(solution, PathingSolution::iLastLoadedVirtualNodeIndex, 1);
        let smoother = f.at(-0x2254);
        e.call(PATH_SMOOTHER_CONSTRUCT, &args![smoother]);
        let extra_nodes = f.at(-0x2268);
        e.call(
            NODE_ARRAY_C_CONSTRUCT,
            &args![extra_nodes, 0x20u32, 0u32, 0u32],
        );
        e.call(
            PATH_SMOOTHER_SMOOTH,
            &args![
                smoother, request, path_nodes, goal_nodes, position, center, solution, 0u32, 1u32
            ],
        );
        e.set(solution, PathingSolution::iFirstLoadedVirtualNodeIndex, 0);
        let count = e.call(SOLUTION_VIRTUAL_NODE_COUNT, &args![solution]).u32();
        e.set(
            solution,
            PathingSolution::iLastLoadedVirtualNodeIndex,
            count.wrapping_sub(1) as i32,
        );
        e.call(NODE_ARRAY_C_DESTRUCT, &args![extra_nodes]);
        e.call(PATH_SMOOTHER_DESTRUCT, &args![smoother]);
        e.call(NODE_ARRAY_B_DESTRUCT, &args![goal_nodes]);
        e.call(NODE_ARRAY_A_DESTRUCT, &args![path_nodes]);
        e.call(HIDE_SEARCH_DESTRUCT, &args![search]);
        e.call(NAV_HOLDER_RELEASE, &args![destination_holder]);
        e.call(NAV_HOLDER_RELEASE, &args![origin_holder]);
        scope_leave(e, timer);
        true
    })
}

// Translated from 006d23d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Solver for a type-7 request (optimal location): aims at the point on
/// the line from the destination to the origin that lies halfway between
/// the request's two distances (`fn_006d2c20`, `fn_006d2c40`) from the
/// destination, finds the closest navmesh point to it (`006d6f80`), fails
/// when that point is farther than `|halfway - distance|` away, searches
/// with `NavMeshSearchMaxCost`, and builds the solution with the path
/// builder from the origin to the chosen point.
///
/// When the search finds no path it falls back to the search's best node,
/// or, when no node array was filled, either to the base solver
/// (`fn_006d0b10`, when the direct distance exceeds the request's bound
/// from `0099e040`) or to a failed result. The chosen point must lie
/// between the squared distances of the request's two bounds.
pub fn fn_006d23d0(
    e: &mut Engine,
    request: Ptr<PathingRequest>,
    solution: Ptr<PathingSolution>,
) -> bool {
    with_frame(e, 0x4348, |e, f| {
        let timer = f.at(-0x2194);
        scope_enter(e, timer, 0x29c);
        let origin = e.call(REQUEST_ORIGIN, &args![request]).u32();
        if !e.call(LOCATION_RESOLVE_CLOSEST, &args![origin]).bool() {
            scope_leave(e, timer);
            return false;
        }
        let holder = f.at(-0x20);
        let triangle = f.at(-0x21a8);
        e.call(NAV_HOLDER_CONSTRUCT, &args![holder]);
        e.mem.set_u16(triangle, 0xffff);
        let origin = e.call(REQUEST_ORIGIN, &args![request]).u32();
        if !e
            .call(
                LOCATION_GET_NAVMESH_AND_TRIANGLE,
                &args![origin, holder, triangle],
            )
            .bool()
        {
            e.call(NAV_HOLDER_RELEASE, &args![holder]);
            scope_leave(e, timer);
            return false;
        }

        // The point halfway between the two distances, on the line from the
        // destination to the origin.
        let minimum = fn_006d2c20(e, request.cast());
        let maximum = fn_006d2c40(e, request.cast());
        let halfway = ((minimum as f64 + maximum as f64) / e.global::<f64>(TWO)) as f32;
        let origin_position = f.at(-0x2190);
        let origin = e.call(REQUEST_ORIGIN, &args![request]).u32();
        e.call(LOCATION_COPY_POSITION, &args![origin, origin_position]);
        let destination_position = f.at(-0x216c);
        let destination = e.call(REQUEST_DESTINATION, &args![request]).u32();
        e.call(
            LOCATION_COPY_POSITION,
            &args![destination, destination_position],
        );
        let direction = f.at(-0x21b4);
        e.call(
            POINT3_SUBTRACT,
            &args![destination_position, direction, origin_position],
        );
        let length = e.call(POINT3_UNITIZE_GET_LENGTH, &args![direction]).f32();
        let scaled_result = f.at(-0x431c);
        let scaled = e
            .call(
                POINT3_TIMES_SCALAR,
                &args![direction, scaled_result, halfway],
            )
            .u32();
        let aim = f.at(-0x21a4);
        e.call(POINT3_SUBTRACT, &args![destination_position, aim, scaled]);
        let slack = e
            .call(
                FLOAT_ABSOLUTE_VALUE,
                &args![((halfway as f64) - (length as f64)) as f32],
            )
            .f32();
        let slack_slot = f.at(-0x2160);
        e.mem.set_f32(slack_slot, slack);

        let closest = f.at(-0x48);
        e.call(POINT3_DEFAULT_CONSTRUCTOR, &args![closest]);
        let origin = e.call(REQUEST_ORIGIN, &args![request]).u32();
        let cell = e.call(LOCATION_GET_CELL, &args![origin]).u32();
        let origin = e.call(REQUEST_ORIGIN, &args![request]).u32();
        let worldspace = e.call(LOCATION_GET_WORLDSPACE, &args![origin]).u32();
        if !e
            .call(
                FIND_CLOSEST_POINT_ON_NAVMESH,
                &args![worldspace, cell, aim, closest],
            )
            .bool()
        {
            e.call(NAV_HOLDER_RELEASE, &args![holder]);
            scope_leave(e, timer);
            return false;
        }
        let gap = f.at(-0x3c);
        e.call(POINT3_SUBTRACT, &args![closest, gap, aim]);
        let gap_squared = e.call(POINT3_LENGTH_SQUARED, &args![gap]).f64();
        let slack = e.mem.f32(slack_slot) as f64;
        // Fails when the closest point is farther than the slack (an unordered
        // comparison continues, as the x87 test does).
        if slack * slack < gap_squared {
            e.call(NAV_HOLDER_RELEASE, &args![holder]);
            scope_leave(e, timer);
            return false;
        }

        // The goal location and the max-cost search from the origin.
        let goal_location = f.at(-0x215c);
        let origin = e.call(REQUEST_ORIGIN, &args![request]).u32();
        e.call(
            LOCATION_CONSTRUCT_FROM_POINT_AND_LOCATION,
            &args![goal_location, closest, origin],
        );
        let start = f.at(-0x2180);
        let navmesh = e.call(NAV_HOLDER_GET, &args![holder]).u32();
        e.mem.set_u32(start, navmesh);
        let start_triangle = e.mem.u16(triangle);
        e.mem.set_u16(start + 4, start_triangle);
        let center = f.at(-0x18);
        e.call(
            NAVMESH_GET_CENTER,
            &args![navmesh, center, start_triangle as u32],
        );
        let center_copy = f.at(-0x2178);
        copy_words(e, center_copy, center, 3);
        let origin_position_copy = f.at(-0x30);
        let origin = e.call(REQUEST_ORIGIN, &args![request]).u32();
        e.call(LOCATION_COPY_POSITION, &args![origin, origin_position_copy]);
        if !e
            .call(LOCATION_RESOLVE_NAVMESH_INFO, &args![goal_location, 0u32])
            .bool()
        {
            fn_006d2c00(e, Ptr::new(goal_location));
        }
        let search = f.at(-0x2134);
        e.call(MAX_COST_SEARCH_CONSTRUCT, &args![search]);
        let path_nodes = f.at(-0x21d8);
        e.call(
            NODE_ARRAY_A_CONSTRUCT,
            &args![path_nodes, 0x20u32, 0u32, 0u32],
        );
        let goal_nodes = f.at(-0x80);
        e.call(
            NODE_ARRAY_B_CONSTRUCT,
            &args![goal_nodes, 0x20u32, 0u32, 0u32],
        );
        fn_006d2c60(e, Ptr::new(search), slack_times_two(slack));
        let cover_holder = f.at(-0x21c4);
        e.call(NAVMESH_LIST_CONSTRUCT, &args![cover_holder]);
        e.call(NAVMESH_LIST_FILL, &args![cover_holder]);
        let chosen = f.at(-0x430c);
        e.call(POINT3_DEFAULT_CONSTRUCTOR, &args![chosen]);
        let found_path = e
            .call(
                MAX_COST_SEARCH_BUILD_NODE_PATH,
                &args![
                    search,
                    request,
                    start,
                    goal_location,
                    path_nodes,
                    goal_nodes
                ],
            )
            .bool();
        let end_node = f.at(-0x64);
        let mut fallback_result = None;
        if found_path {
            let element = last_node(e, path_nodes);
            copy_search_node(e, end_node, element);
            if !e.call(LOCATION_HAS_FLAG_2, &args![goal_location]).bool() {
                copy_words(e, chosen, closest, 3);
            } else {
                let point_slot = f.at(-0x432c);
                let point = e.call(NODE_TO_POSITION, &args![end_node, point_slot]).u32();
                copy_words(e, chosen, point, 3);
            }
        } else if !e.call(ARRAY_IS_EMPTY, &args![path_nodes]).bool() {
            // No complete path: use the search's best node.
            let best = e.call(MAX_COST_SEARCH_BEST_NODE, &args![search]).u32();
            copy_search_node(e, end_node, best + 8);
            let point_slot = f.at(-0x4338);
            let point = e.call(NODE_TO_POSITION, &args![end_node, point_slot]).u32();
            copy_words(e, chosen, point, 3);
        } else {
            // No nodes at all: hand over to the base solver when the
            // direct distance exceeds the request's bound.
            let bound = e
                .call(REQUEST_ABSOLUTE_MAXIMUM_DISTANCE, &args![request])
                .f64();
            fallback_result = Some(if (length as f64) > bound {
                fn_006d0b10(e, request, solution)
            } else {
                false
            });
        }
        if let Some(result) = fallback_result {
            release_optimal_locals(
                e,
                cover_holder,
                goal_nodes,
                path_nodes,
                search,
                goal_location,
                holder,
            );
            scope_leave(e, timer);
            return result;
        }

        // The chosen point must lie between the request's two bounds
        // (squared); an unordered comparison passes both tests.
        let near_bound = e
            .call(REQUEST_ABSOLUTE_MAXIMUM_DISTANCE, &args![request])
            .f32();
        let far_bound = e
            .call(REQUEST_ABSOLUTE_MINIMUM_DISTANCE, &args![request])
            .f32();
        let chosen_offset = f.at(-0x4348);
        let offset = e
            .call(
                POINT3_SUBTRACT,
                &args![chosen, chosen_offset, destination_position],
            )
            .u32();
        copy_words(e, direction, offset, 3);
        let offset_squared = e.call(POINT3_LENGTH_SQUARED, &args![direction]).f32() as f64;
        let far_squared = far_bound as f64 * far_bound as f64;
        let near_squared = near_bound as f64 * near_bound as f64;
        let too_close = offset_squared < far_squared;
        let too_far = offset_squared > near_squared;
        if too_close || too_far {
            release_optimal_locals(
                e,
                cover_holder,
                goal_nodes,
                path_nodes,
                search,
                goal_location,
                holder,
            );
            scope_leave(e, timer);
            return false;
        }

        let info = e.call(NAVMESH_GET_INFO, &args![navmesh]).u32();
        e.call(
            SOLUTION_ADD_VIRTUAL_NODE,
            &args![solution, origin_position_copy, info],
        );
        let end_navmesh = e.mem.u32(end_node);
        let info = e.call(NAVMESH_GET_INFO, &args![end_navmesh]).u32();
        e.call(SOLUTION_ADD_VIRTUAL_NODE, &args![solution, chosen, info]);
        e.set(solution, PathingSolution::iFirstLoadedVirtualNodeIndex, 0);
        e.set(solution, PathingSolution::iLastLoadedVirtualNodeIndex, 1);
        let builder = f.at(-0x42ec);
        e.call(PATH_BUILDER_CONSTRUCT, &args![builder]);
        let extra_nodes = f.at(-0x4300);
        e.call(
            NODE_ARRAY_C_CONSTRUCT,
            &args![extra_nodes, 0x20u32, 0u32, 0u32],
        );
        let built = e
            .call(
                PATH_BUILDER_BUILD,
                &args![
                    builder,
                    request,
                    path_nodes,
                    goal_nodes,
                    origin_position_copy,
                    chosen,
                    solution,
                    0u32,
                    0u32
                ],
            )
            .bool();
        e.call(NODE_ARRAY_C_DESTRUCT, &args![extra_nodes]);
        e.call(PATH_BUILDER_DESTRUCT, &args![builder]);
        release_optimal_locals(
            e,
            cover_holder,
            goal_nodes,
            path_nodes,
            search,
            goal_location,
            holder,
        );
        scope_leave(e, timer);
        built
    })
}

/// Twice the slack, stored as a `float` (`FADD ST0,ST0`).
fn slack_times_two(slack: f64) -> f32 {
    (slack + slack) as f32
}

/// The common exit of `fn_006d23d0`: destroys the locals it still holds, in
/// the game's order.
#[allow(clippy::too_many_arguments)]
fn release_optimal_locals(
    e: &mut Engine,
    cover_holder: u32,
    goal_nodes: u32,
    path_nodes: u32,
    search: u32,
    goal_location: u32,
    holder: u32,
) {
    e.call(NAVMESH_LIST_DESTRUCT, &args![cover_holder]);
    e.call(NODE_ARRAY_B_DESTRUCT, &args![goal_nodes]);
    e.call(NODE_ARRAY_A_DESTRUCT, &args![path_nodes]);
    e.call(MAX_COST_SEARCH_DESTRUCT, &args![search]);
    e.call(LOCATION_DESTRUCT, &args![goal_location]);
    e.call(NAV_HOLDER_RELEASE, &args![holder]);
}

// Translated from 006d2c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets bit 1 (`2`) of the location's flags (+0x26).
pub fn fn_006d2c00(e: &mut Engine, this: Ptr<PathingLocation>) {
    let flags = e.get(this, PathingLocation::uiFlags);
    e.set(this, PathingLocation::uiFlags, flags | 2);
}

// Translated from 006d2c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `float` at +0xB0: `fMinFleeDistance` of a `PathingRequestFlee` and
/// `fMinDistance` of the close-point family (Xbox PDB), returned in `ST0`.
pub fn fn_006d2c20(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + 0xb0)
}

// Translated from 006d2c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `float` at +0xB4 (`fMaxDistance` of the close-point family, Xbox
/// PDB), returned in `ST0`.
pub fn fn_006d2c40(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + 0xb4)
}

// Translated from 006d2c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the `float` at +0x20A4 of the max-cost search (its cost limit).
pub fn fn_006d2c60(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_f32(this.addr() + 0x20a4, x87_quiet(value));
}

// Translated from 006d2c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Solver for a type-8 request (covered move): first solves the request as
/// a plain request (`fn_006d0b10`), then tries each later node of the
/// solution as the destination of a covered-move copy of the request
/// (`PathingRequestCoveredMove` with a fresh `CombatCoverLocation` and
/// `CombatCoverSearchInfo` shared with the request) until one passes
/// `0099d480` and also solves; that copy's cover location and destination
/// then replace the request's and the new solution is copied over. Without
/// a hit the request's cover location pointer is cleared (and the cover
/// location object deleted). The request is marked finished (+0xE1 = 1)
/// either way.
pub fn fn_006d2c80(
    e: &mut Engine,
    request: Ptr<PathingRequest>,
    solution: Ptr<PathingSolution>,
) -> bool {
    with_frame(e, 0x240, |e, f| {
        let timer = f.at(-0x1c);
        scope_enter(e, timer, 0x322);
        if !fn_006d0b10(e, request, solution) {
            scope_leave(e, timer);
            return false;
        }
        let distance = e.global::<f32>(COVERED_MOVE_DISTANCE);
        let origin = e.call(REQUEST_ORIGIN, &args![request]).u32();
        let previous = f.at(-0x15c);
        e.call(LOCATION_COPY_CONSTRUCT, &args![previous, origin]);

        let cover_block = e.call(MEMORY_ALLOCATE, &args![0x78u32]).u32();
        let cover_location = if cover_block != 0 {
            fn_006d3150(e, Ptr::new(cover_block)).addr()
        } else {
            0
        };
        let search_block = e.call(MEMORY_ALLOCATE, &args![0x3cu32]).u32();
        let search_info = if search_block != 0 {
            fn_006d32e0(e, Ptr::new(search_block)).addr()
        } else {
            0
        };
        let copy = f.at(-0x134);
        e.call(COVERED_MOVE_CONSTRUCT_COPY, &args![copy, request]);
        fn_006d3100(e, Ptr::new(copy), cover_location);
        fn_006d30e0(e, Ptr::new(copy), search_info);
        fn_006d3100(e, request.cast(), cover_location);
        fn_006d30e0(e, request.cast(), search_info);

        let mut index = 1u32;
        while e.call(SOLUTION_NODE_COUNT, &args![solution]).u32() > index {
            let node = e.call(SOLUTION_NODE_AT, &args![solution, index]).u32();
            let node_location = e.call(NODE_LOCATION, &args![node]).u32();
            let next = f.at(-0x188);
            e.call(LOCATION_COPY_CONSTRUCT, &args![next, node_location]);
            fn_006d1bc0(e, Ptr::new(copy), Ptr::new(next));
            if e.call(COVERED_MOVE_CHECK, &args![copy, distance, 1u32])
                .bool()
            {
                let cover_point = e.call(COVERED_MOVE_COVER_POINT, &args![copy]).u32();
                let cover_target = f.at(-0x1fc);
                e.call(
                    LOCATION_CONSTRUCT_FROM_POINT_AND_LOCATION,
                    &args![cover_target, cover_point, next],
                );
                fn_006d1bc0(e, Ptr::new(copy), Ptr::new(cover_target));
                let result = f.at(-0x1d4);
                e.call(SOLUTION_CONSTRUCT, &args![result]);
                let solved = fn_006d0b10(e, Ptr::new(copy), Ptr::new(result));
                if solved {
                    let cover_point = e.call(COVERED_MOVE_COVER_POINT, &args![copy]).u32();
                    fn_006d30b0(e, request.cast(), Ptr::new(cover_point));
                    fn_006d1bc0(e, request.cast(), Ptr::new(cover_target));
                    fn_006d3120(e, Ptr::new(copy));
                    e.call(SOLUTION_COPY_TO, &args![result, solution]);
                    e.call(REQUEST_SET_FINISHED, &args![request, 1u32]);
                    e.call(SOLUTION_DESTRUCT, &args![result]);
                    e.call(LOCATION_DESTRUCT, &args![cover_target]);
                    e.call(LOCATION_DESTRUCT, &args![next]);
                    e.call(COVERED_MOVE_DESTRUCT, &args![copy]);
                    e.call(LOCATION_DESTRUCT, &args![previous]);
                    scope_leave(e, timer);
                    return true;
                }
                e.call(SOLUTION_DESTRUCT, &args![result]);
                e.call(LOCATION_DESTRUCT, &args![cover_target]);
            }
            index += 1;
            e.call(LOCATION_ASSIGN, &args![previous, next]);
            e.call(LOCATION_DESTRUCT, &args![next]);
        }

        fn_006d3120(e, Ptr::new(copy));
        if cover_location != 0 {
            fn_006d3370(e, Ptr::new(cover_location), 1);
        }
        fn_006d3100(e, request.cast(), 0);
        e.call(REQUEST_SET_FINISHED, &args![request, 1u32]);
        e.call(COVERED_MOVE_DESTRUCT, &args![copy]);
        e.call(LOCATION_DESTRUCT, &args![previous]);
        scope_leave(e, timer);
        true
    })
}

// Translated from 006d30b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the point at `point` to the covered-move request's
/// `CoverLocation` (+0xC8).
pub fn fn_006d30b0(e: &mut Engine, this: Ptr, point: Ptr) {
    copy_words(e, this.addr() + 0xc8, point.addr(), 3);
}

// Translated from 006d30e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the covered-move request's `pCombatCoverSearchInfo` (+0x108).
pub fn fn_006d30e0(e: &mut Engine, this: Ptr<PathingRequestCoveredMove>, info: u32) {
    e.set(
        this,
        PathingRequestCoveredMove::pCombatCoverSearchInfo,
        Ptr::new(info),
    );
}

// Translated from 006d3100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the covered-move request's `pCombatCoverLocation` (+0x104).
pub fn fn_006d3100(e: &mut Engine, this: Ptr<PathingRequestCoveredMove>, location: u32) {
    e.set(
        this,
        PathingRequestCoveredMove::pCombatCoverLocation,
        Ptr::new(location),
    );
}

// Translated from 006d3120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the covered-move request's `pCombatCoverLocation` and
/// `pCombatCoverSearchInfo`.
pub fn fn_006d3120(e: &mut Engine, this: Ptr<PathingRequestCoveredMove>) {
    e.set(
        this,
        PathingRequestCoveredMove::pCombatCoverLocation,
        Ptr::NULL,
    );
    e.set(
        this,
        PathingRequestCoveredMove::pCombatCoverSearchInfo,
        Ptr::NULL,
    );
}

// Translated from 006d3150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the 0x78-byte object `fn_006d2c80` stores in
/// `pCombatCoverLocation` (the Xbox PDB's `CombatCoverLocation`): clears its
/// flags and location, copies the global zero vector into
/// `CoverSearchTargetLocation`, constructs the embedded
/// `PathingCoverLocation` (+0x14), clears the key and sets the time stamp to
/// -1 and constructs the `CoverReservationInfo` (+0x70). Returns `this`.
pub fn fn_006d3150(e: &mut Engine, this: Ptr<CombatCoverLocation>) -> Ptr<CombatCoverLocation> {
    e.set(this, CombatCoverLocation::bDirtyFlag, false);
    e.set(this, CombatCoverLocation::bReserved, false);
    e.set(this, CombatCoverLocation::cCoverHeight, 0);
    e.set(this, CombatCoverLocation::iLocation, 0);
    copy_words(e, this.addr() + 8, ZERO_VECTOR, 3);
    fn_006d3200(e, Ptr::new(this.addr() + 0x14));
    e.set(this, CombatCoverLocation::iCoverLocationKey, 0);
    e.set(
        this,
        CombatCoverLocation::iCoverLocationLastMovedTimeStamp,
        -1,
    );
    fn_006d3280(e, Ptr::new(this.addr() + 0x70));
    this
}

// Translated from 006d3200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the 0x54-byte `PathingCoverLocation`: the
/// `PathingLocation` constructor, the derived vtable (`0106cbc0`) and three
/// default-constructed points at +0x28, +0x34 and +0x40. Returns `this`.
pub fn fn_006d3200(e: &mut Engine, this: Ptr<PathingCoverLocation>) -> Ptr<PathingCoverLocation> {
    e.call(LOCATION_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), PATHING_COVER_LOCATION_VTABLE);
    for offset in [0x28u32, 0x34, 0x40] {
        e.call(POINT3_DEFAULT_CONSTRUCTOR, &args![this.addr() + offset]);
    }
    this
}

// Translated from 006d3280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the six-byte `CoverReservationInfo`: clears its six
/// bytes. Returns `this`.
pub fn fn_006d3280(e: &mut Engine, this: Ptr<CoverReservationInfo>) -> Ptr<CoverReservationInfo> {
    for offset in 0..6 {
        e.mem.set_u8(this.addr() + offset, 0);
    }
    this
}

// Translated from 006d32c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of `PathingCoverLocation`: destroys the `PathingLocation`
/// base.
pub fn fn_006d32c0(e: &mut Engine, this: Ptr<PathingCoverLocation>) {
    e.call(LOCATION_DESTRUCT, &args![this]);
}

// Translated from 006d32e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the 0x3C-byte object `fn_006d2c80` stores in
/// `pCombatCoverSearchInfo` (the Xbox PDB's `CombatCoverSearchInfo`): zero
/// vector in `TargetLocation` and `Center`, radius 0, the time stamp set to
/// -`FLT_MAX` (`00435de0`), the cover location array constructed
/// (`006dae60`), the key, state and flag cleared. Returns `this`.
pub fn fn_006d32e0(e: &mut Engine, this: Ptr<CombatCoverSearchInfo>) -> Ptr<CombatCoverSearchInfo> {
    copy_words(e, this.addr(), ZERO_VECTOR, 3);
    copy_words(e, this.addr() + 0xc, ZERO_VECTOR, 3);
    e.set(this, CombatCoverSearchInfo::fRadius, 0.0);
    let maximum = e.global::<f32>(FLOAT_MAXIMUM);
    e.call(TIME_STAMP_CONSTRUCT, &args![this.addr() + 0x1c, -maximum]);
    e.call(COVER_LOCATION_ARRAY_CONSTRUCT, &args![this.addr() + 0x20]);
    e.set(this, CombatCoverSearchInfo::iPreviousCoverLocationKey, 0);
    e.set(this, CombatCoverSearchInfo::ePreviousCoverState, 0);
    e.set(this, CombatCoverSearchInfo::bUsedLastSeenLocation, false);
    this
}

// Translated from 006d3370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of `CombatCoverLocation`: runs
/// `fn_006d33a0` and frees the block when bit 0 of `flags` is set. Returns
/// `this`.
pub fn fn_006d3370(
    e: &mut Engine,
    this: Ptr<CombatCoverLocation>,
    flags: u32,
) -> Ptr<CombatCoverLocation> {
    fn_006d33a0(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![this]);
    }
    this
}

// Translated from 006d33a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of `CombatCoverLocation`: destroys the embedded
/// `PathingCoverLocation` (+0x14).
pub fn fn_006d33a0(e: &mut Engine, this: Ptr<CombatCoverLocation>) {
    fn_006d32c0(e, Ptr::new(this.addr() + 0x14));
}

// Translated from 006d33c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Picks a random point near the request's origin and stores it in `out`.
/// First tries `fn_006d34d0` (a random point on the last triangle of a node
/// path from the origin); if that gives a point there is nothing more to
/// do. Otherwise `out = origin position + (x, y, 0)` with `(x, y)` a point
/// on a circle of random radius between `fn_006d2c20` and `fn_006d2c40` at a
/// random angle, and `out.z` replaced by the origin cell's land height when
/// the cell has one.
pub fn fn_006d33c0(e: &mut Engine, request: Ptr<PathingRequest>, out: Ptr) {
    if fn_006d34d0(e, request, out) {
        return;
    }
    let angle = e.call(RANDOM_ANGLE, &args![]).f32();
    let maximum = fn_006d2c40(e, request.cast());
    let minimum = fn_006d2c20(e, request.cast());
    let radius = e.call(RANDOM_FLOAT, &args![minimum, maximum]).f32();
    let y_scale = e.call(ANGLE_FUNCTION_004E44D0, &args![angle]).f64();
    let y = (y_scale * radius as f64) as f32;
    let x_scale = e.call(ANGLE_FUNCTION_004E4490, &args![angle]).f64();
    let x = (x_scale * radius as f64) as f32;
    with_frame(e, 0x3c, |e, f| {
        let offset = f.at(-0x10);
        e.call(POINT3_CONSTRUCT, &args![offset, x, y, 0.0f32]);
        let origin = e.call(REQUEST_ORIGIN, &args![request]).u32();
        let position = f.at(-0x28);
        let position = e
            .call(LOCATION_COPY_POSITION, &args![origin, position])
            .u32();
        let sum_result = f.at(-0x34);
        let sum = e
            .call(POINT3_ADD, &args![position, sum_result, offset])
            .u32();
        copy_words(e, out.addr(), sum, 3);
        let origin = e.call(REQUEST_ORIGIN, &args![request]).u32();
        let cell = e.call(LOCATION_GET_CELL, &args![origin]).u32();
        if cell == 0 {
            return;
        }
        let height = f.at(-0x18);
        if e.call(CELL_GET_LAND_HEIGHT, &args![cell, out, height])
            .bool()
        {
            let height = e.mem.f32(height);
            e.mem.set_f32(out.addr() + 8, height);
        }
    });
}

// Translated from 006d34d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds a path from the request's origin with the random-point search
/// (`006a8b60`) and stores in `out` a random point (`006bbad0`) on the
/// triangle at the end of the path. Fails when the origin cannot be
/// resolved, has no navmesh and triangle, or the search finds no path.
pub fn fn_006d34d0(e: &mut Engine, request: Ptr<PathingRequest>, out: Ptr) -> bool {
    with_frame(e, 0x2134, |e, f| {
        let origin = e.call(REQUEST_ORIGIN, &args![request]).u32();
        if !e.call(LOCATION_RESOLVE_CLOSEST, &args![origin]).bool() {
            return false;
        }
        let holder = f.at(-0x10);
        let triangle = f.at(-0x74);
        e.call(NAV_HOLDER_CONSTRUCT, &args![holder]);
        e.mem.set_u16(triangle, 0xffff);
        let origin = e.call(REQUEST_ORIGIN, &args![request]).u32();
        if !e
            .call(
                LOCATION_GET_NAVMESH_AND_TRIANGLE,
                &args![origin, holder, triangle],
            )
            .bool()
        {
            e.call(NAV_HOLDER_RELEASE, &args![holder]);
            return false;
        }
        let start = f.at(-0x4c);
        let navmesh = e.call(NAV_HOLDER_GET, &args![holder]).u32();
        e.mem.set_u32(start, navmesh);
        let start_triangle = e.mem.u16(triangle);
        e.mem.set_u16(start + 4, start_triangle);
        let search = f.at(-0x212c);
        e.call(RANDOM_POINT_SEARCH_CONSTRUCT, &args![search]);
        let path_nodes = f.at(-0x8c);
        e.call(
            NODE_ARRAY_A_CONSTRUCT,
            &args![path_nodes, 0x20u32, 0u32, 0u32],
        );
        let goal_nodes = f.at(-0x38);
        e.call(
            NODE_ARRAY_B_CONSTRUCT,
            &args![goal_nodes, 0x20u32, 0u32, 0u32],
        );
        let built = e
            .call(
                RANDOM_POINT_SEARCH_BUILD_NODE_PATH,
                &args![search, request, start, path_nodes, goal_nodes],
            )
            .bool();
        if !built {
            e.call(NODE_ARRAY_B_DESTRUCT, &args![goal_nodes]);
            e.call(NODE_ARRAY_A_DESTRUCT, &args![path_nodes]);
            e.call(RANDOM_POINT_SEARCH_DESTRUCT, &args![search]);
            e.call(NAV_HOLDER_RELEASE, &args![holder]);
            return false;
        }
        let last = f.at(-0x24);
        let element = last_node(e, path_nodes);
        copy_search_node(e, last, element);
        // Three corner points, constructed by the vector constructor
        // iterator, then read from the triangle's vertices.
        let corners = f.at(-0x70);
        e.call(
            VECTOR_CONSTRUCTOR_ITERATOR,
            &args![corners, 0xcu32, 3u32, POINT3_DEFAULT_CONSTRUCTOR],
        );
        let last_navmesh = e.mem.u32(last);
        let last_triangle = e.mem.u16(last + 4) as u32;
        let record = e
            .call(NAVMESH_GET_TRIANGLE, &args![last_navmesh, last_triangle])
            .u32();
        for corner in 0..3u32 {
            let vertex_index = e
                .call(TRIANGLE_GET_VERTEX_INDEX, &args![record, corner])
                .u16() as u32;
            let vertex = e
                .call(NAVMESH_GET_VERTEX, &args![last_navmesh, vertex_index])
                .u32();
            copy_words(e, corners + 12 * corner, vertex, 3);
        }
        let point_result = f.at(-0x213c);
        let point = e
            .call(
                TRIANGLE_RANDOM_POINT,
                &args![point_result, corners, corners + 12, corners + 24],
            )
            .u32();
        copy_words(e, out.addr(), point, 3);
        e.call(NODE_ARRAY_B_DESTRUCT, &args![goal_nodes]);
        e.call(NODE_ARRAY_A_DESTRUCT, &args![path_nodes]);
        e.call(RANDOM_POINT_SEARCH_DESTRUCT, &args![search]);
        e.call(NAV_HOLDER_RELEASE, &args![holder]);
        true
    })
}

// Translated from 006d3780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Pathing::FindSafeStraightLinePathPoint` (Xbox PDB): builds a
/// `PathingRequestSafeStraightLine` from reference `from` to reference
/// `to` and asks `006d3b40` for a safe point, stored in `out`. Each
/// reference's size is the larger of its two bound extents in x and y (the
/// vtable slots +0x1dc and +0x1d8 write the maximum and minimum corners)
/// times its scale, at least 32, halved; these are the origin's
/// (`fActorRadius`) and the destination's (`fTargetRadius`, +0x74) radii.
/// The request's `fMinDistance` is `0.9 * distance + origin radius` and its
/// `fMaxDistance` `distance + origin radius`.
pub fn pathing_find_safe_straight_line_path_point(
    e: &mut Engine,
    from: Ptr,
    to: Ptr,
    distance: f32,
    out: Ptr,
) -> bool {
    with_frame(e, 0x170, |e, f| {
        let request = f.at(-0xd4);
        e.call(SAFE_STRAIGHT_LINE_CONSTRUCT, &args![request]);
        let origin_location = f.at(-0x120);
        let result = e
            .call(
                LOCATION_CONSTRUCT_AT_REFERENCE,
                &args![origin_location, from],
            )
            .u32();
        fn_006d3ac0(e, Ptr::new(request), Ptr::new(result));
        e.call(LOCATION_DESTRUCT, &args![origin_location]);
        let destination_location = f.at(-0x148);
        let result = e
            .call(
                LOCATION_CONSTRUCT_AT_REFERENCE,
                &args![destination_location, to],
            )
            .u32();
        fn_006d1bc0(e, Ptr::new(request), Ptr::new(result));
        e.call(LOCATION_DESTRUCT, &args![destination_location]);

        // The origin's extent, from the first reference's own bounds.
        let maximum_corner = f.at(-0xf0);
        let minimum_corner = f.at(-0xe0);
        e.vcall(from.addr(), 0x1dc, &args![maximum_corner]);
        e.vcall(from.addr(), 0x1d8, &args![minimum_corner]);
        let origin_radius = reference_radius(e, from.addr(), f);
        fn_006d3ae0(e, Ptr::new(request), origin_radius);

        // The destination's extent.
        let bounds_a = f.at(-0x154);
        let corner = e.vcall(to.addr(), 0x1dc, &args![bounds_a]).u32();
        copy_words(e, maximum_corner, corner, 3);
        let bounds_b = f.at(-0x160);
        let corner = e.vcall(to.addr(), 0x1d8, &args![bounds_b]).u32();
        copy_words(e, minimum_corner, corner, 3);
        let destination_radius = reference_radius(e, to.addr(), f);
        e.call(
            REQUEST_SET_TARGET_RADIUS,
            &args![request, destination_radius],
        );

        let minimum =
            (distance as f64 * e.global::<f64>(NINE_TENTHS) + origin_radius as f64) as f32;
        e.call(REQUEST_SET_MINIMUM_DISTANCE, &args![request, minimum]);
        let maximum = (distance as f64 + origin_radius as f64) as f32;
        e.call(REQUEST_SET_MAXIMUM_DISTANCE, &args![request, maximum]);
        let found = e
            .call(SAFE_STRAIGHT_LINE_FIND, &args![request, out, 0u32])
            .bool();
        e.call(SAFE_STRAIGHT_LINE_DESTRUCT, &args![request]);
        found
    })
}

/// Half the reference's footprint: the larger of the x and y differences
/// of the corners at `-0xf0` and `-0xe0` (maximum minus minimum), times the
/// reference's scale (`TESObjectREFR::GetScale`), at least 32, times 0.5.
fn reference_radius(e: &mut Engine, reference: u32, f: Frame) -> f32 {
    let maximum_x = e.mem.f32(f.at(-0xf0)) as f64;
    let minimum_x = e.mem.f32(f.at(-0xe0)) as f64;
    let maximum_y = e.mem.f32(f.at(-0xec)) as f64;
    let minimum_y = e.mem.f32(f.at(-0xdc)) as f64;
    let width = (maximum_x - minimum_x) as f32;
    let depth = (maximum_y - minimum_y) as f32;
    let larger = e.call(FLOAT_MAX, &args![width, depth]).f32();
    let scale = e.call(REFERENCE_GET_SCALE, &args![reference]).f64();
    let size = (scale * larger as f64) as f32;
    let least = e.global::<f32>(MINIMUM_RADIUS);
    let size = e.call(FLOAT_MAX, &args![size, least]).f32();
    (size as f64 * e.global::<f64>(HALF)) as f32
}

// Translated from 006d3ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assigns `location` to the request's `Origin` (+0x0C).
pub fn fn_006d3ac0(e: &mut Engine, this: Ptr, location: Ptr) {
    e.call(LOCATION_ASSIGN, &args![this.addr() + 0xc, location]);
}

// Translated from 006d3ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the request's `fActorRadius` (+0x68).
pub fn fn_006d3ae0(e: &mut Engine, this: Ptr<PathingRequest>, radius: f32) {
    e.set(this, PathingRequest::fActorRadius, x87_quiet(radius));
}

// Translated from 006d3b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the request's `fTargetRadius` (+0x74).
pub fn fn_006d3b00(e: &mut Engine, this: Ptr<PathingRequest>, radius: f32) {
    e.set(this, PathingRequest::fTargetRadius, x87_quiet(radius));
}

// Translated from 006d3b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of a `PathingRequestSafeStraightLine` (Xbox PDB): runs the
/// base destructor `006dad70`.
pub fn fn_006d3b20(e: &mut Engine, this: Ptr<PathingRequest>) {
    e.call(REQUEST_BASE_DESTRUCT, &args![this]);
}

// Translated from 006d3b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds a point for a safe-straight-line request and stores it in `out`.
/// Both the origin and the destination must resolve to a navmesh and
/// triangle. The point search (`006ac220`, whose point `fn_006d3ff0` copies
/// out) goes first; when it finds nothing the request is copied and solved
/// as a type-0 request (`fn_006d0b10`) and the solution is asked for a
/// point (`006e7e70`, with the last node index and the request's minimum
/// distance); when that fails too, a close-point request with the same
/// distances and the destination as its origin gets a random point from
/// `fn_006d34d0`. The third argument callers pass (0) is not read.
pub fn fn_006d3b40(e: &mut Engine, request: Ptr<PathingRequest>, out: Ptr) -> bool {
    with_frame(e, 0x2330, |e, f| {
        let origin = e.call(REQUEST_ORIGIN, &args![request]).u32();
        if !e.call(LOCATION_RESOLVE_CLOSEST, &args![origin]).bool() {
            return false;
        }
        let start_holder = f.at(-0x28);
        let start_triangle = f.at(-0x2160);
        e.call(NAV_HOLDER_CONSTRUCT, &args![start_holder]);
        e.mem.set_u16(start_triangle, 0xffff);
        let origin = e.call(REQUEST_ORIGIN, &args![request]).u32();
        if !e
            .call(
                LOCATION_GET_NAVMESH_AND_TRIANGLE,
                &args![origin, start_holder, start_triangle],
            )
            .bool()
        {
            e.call(NAV_HOLDER_RELEASE, &args![start_holder]);
            return false;
        }
        let start = f.at(-0x24);
        let navmesh = e.call(NAV_HOLDER_GET, &args![start_holder]).u32();
        e.mem.set_u32(start, navmesh);
        let triangle = e.mem.u16(start_triangle);
        e.mem.set_u16(start + 4, triangle);

        let destination = e.call(REQUEST_DESTINATION, &args![request]).u32();
        if !e.call(LOCATION_RESOLVE_CLOSEST, &args![destination]).bool() {
            e.call(NAV_HOLDER_RELEASE, &args![start_holder]);
            return false;
        }
        let goal_holder = f.at(-0x215c);
        let goal_triangle = f.at(-0x10);
        e.call(NAV_HOLDER_CONSTRUCT, &args![goal_holder]);
        e.mem.set_u16(goal_triangle, 0xffff);
        let destination = e.call(REQUEST_DESTINATION, &args![request]).u32();
        if !e
            .call(
                LOCATION_GET_NAVMESH_AND_TRIANGLE,
                &args![destination, goal_holder, goal_triangle],
            )
            .bool()
        {
            e.call(NAV_HOLDER_RELEASE, &args![goal_holder]);
            e.call(NAV_HOLDER_RELEASE, &args![start_holder]);
            return false;
        }
        let goal = f.at(-0x2158);
        let navmesh = e.call(NAV_HOLDER_GET, &args![goal_holder]).u32();
        e.mem.set_u32(goal, navmesh);
        let triangle = e.mem.u16(goal_triangle);
        e.mem.set_u16(goal + 4, triangle);

        let search = f.at(-0x2134);
        e.call(HIDE_SEARCH_CONSTRUCT, &args![search]);
        let list = f.at(-0x2144);
        e.call(NAVMESH_LIST_CONSTRUCT, &args![list]);
        e.call(NAVMESH_LIST_FILL, &args![list]);
        let found = e
            .call(POINT_SEARCH_RUN, &args![search, request, start, goal])
            .bool();
        if found {
            fn_006d3ff0(e, Ptr::new(search), out);
            safe_line_release(e, list, search, goal_holder, start_holder);
            return true;
        }

        let copy = f.at(-0x225c);
        e.call(REQUEST_CONSTRUCT, &args![copy]);
        e.call(REQUEST_COPY_TO, &args![request, copy]);
        let solution = f.at(-0x21ac);
        e.call(SOLUTION_CONSTRUCT, &args![solution]);
        let mut result = false;
        if fn_006d0b10(e, Ptr::new(copy), Ptr::new(solution)) {
            let minimum = fn_006d2c20(e, request.cast());
            let node_count = e.call(SOLUTION_NODE_COUNT, &args![solution]).u32();
            let last_index = node_count.wrapping_sub(1) as f32;
            let scratch = f.at(-0x2260);
            if e.call(
                SOLUTION_QUERY_006E7E70,
                &args![solution, last_index, minimum, 0u32, scratch, out],
            )
            .bool()
            {
                e.call(SOLUTION_DESTRUCT, &args![solution]);
                e.call(REQUEST_COPY_DESTRUCT, &args![copy]);
                safe_line_release(e, list, search, goal_holder, start_holder);
                return true;
            }
            let close = f.at(-0x231c);
            e.call(CLOSE_POINT_REQUEST_CONSTRUCT, &args![close]);
            e.call(REQUEST_COPY_TO, &args![request, close]);
            let destination = e.call(REQUEST_DESTINATION, &args![request]).u32();
            fn_006d3ac0(e, Ptr::new(close), Ptr::new(destination));
            let minimum = fn_006d2c20(e, request.cast());
            e.call(REQUEST_SET_MINIMUM_DISTANCE, &args![close, minimum]);
            let maximum = fn_006d2c40(e, request.cast());
            e.call(REQUEST_SET_MAXIMUM_DISTANCE, &args![close, maximum]);
            result = fn_006d34d0(e, Ptr::new(close), out);
            e.call(REQUEST_BASE_DESTRUCT, &args![close]);
        }
        e.call(SOLUTION_DESTRUCT, &args![solution]);
        e.call(REQUEST_COPY_DESTRUCT, &args![copy]);
        safe_line_release(e, list, search, goal_holder, start_holder);
        result
    })
}

/// The common exit of `fn_006d3b40`: destroys the navmesh list, the point
/// search and the two navmesh holders, in that order.
fn safe_line_release(e: &mut Engine, list: u32, search: u32, goal_holder: u32, start_holder: u32) {
    e.call(NAVMESH_LIST_DESTRUCT, &args![list]);
    e.call(HIDE_SEARCH_DESTRUCT, &args![search]);
    e.call(NAV_HOLDER_RELEASE, &args![goal_holder]);
    e.call(NAV_HOLDER_RELEASE, &args![start_holder]);
}

// Translated from 006d3ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the point the point search found (this + 0x20cc) to `out`.
pub fn fn_006d3ff0(e: &mut Engine, this: Ptr, out: Ptr) {
    copy_words(e, out.addr(), this.addr() + 0x20cc, 3);
}

// Translated from 006d4020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Comparison function `fn_006d4120` gives `qsort` for its 0x20-byte
/// candidates (cdecl, two element pointers). A candidate is in the height
/// band when its float at +0x1c is above -64.0 and below 180.0 (exclusive;
/// NaN is outside). Candidates in the band come first (-1 when only
/// `first` is in it, 1 when only `second` is); otherwise the smaller float
/// at +0x18 comes first.
pub fn fn_006d4020(e: &mut Engine, first: Ptr, second: Ptr) -> i32 {
    let minimum = e.global::<f64>(HEIGHT_BAND_MINIMUM);
    let maximum = e.global::<f64>(HEIGHT_BAND_MAXIMUM);
    let first_height = e.mem.f32(first.addr() + 0x1c) as f64;
    let second_height = e.mem.f32(second.addr() + 0x1c) as f64;
    let first_in_band = first_height > minimum && first_height < maximum;
    let second_in_band = second_height > minimum && second_height < maximum;
    if first_in_band && !second_in_band {
        return -1;
    }
    if !first_in_band && second_in_band {
        return 1;
    }
    let first_distance = e.mem.f32(first.addr() + 0x18);
    let second_distance = e.mem.f32(second.addr() + 0x18);
    if second_distance > first_distance {
        -1
    } else if second_distance < first_distance {
        1
    } else {
        0
    }
}

// Translated from 006d4120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Collects up to `limit` points near a location that has no navmesh info.
/// Returns 1 at once when the location has navmesh info (after resolving
/// it). Otherwise it lists the navmeshes within the search radius setting
/// (`011d74dc`) of the location (`006d8a00`) and, for every triangle edge
/// without a neighbour (`0xffff`), finds the closest point of the edge to
/// the location; when its planar squared distance is below the radius
/// squared a 0x20-byte candidate is recorded: navmesh, triangle (`u16`),
/// edge, the point moved 0.1 of the way to the triangle's center, the
/// distance and the height difference. The candidates are sorted with
/// `fn_006d4020`, the first `limit` are converted to path points and added
/// to `out`. Returns 2 when there is at least one, 0 when none.
pub fn fn_006d4120(e: &mut Engine, location: Ptr, limit: u32, out: Ptr) -> u32 {
    with_frame(e, 0x110, |e, f| {
        e.call(LOCATION_RESOLVE_NAVMESH_INFO, &args![location, 0u32]);
        if e.call(LOCATION_HAS_NAVMESH_INFO, &args![location]).bool() {
            return 1;
        }
        let setting = e
            .call(SETTING_FLOAT_ADDRESS, &args![NEARBY_RADIUS_SETTING])
            .u32();
        let radius = e.mem.f32(setting);
        let radius_squared = (radius as f64 * radius as f64) as f32;
        let list = f.at(-0x34);
        e.call(NAVMESH_LIST_CONSTRUCT, &args![list]);
        e.call(NEARBY_NAVMESHES_FILL, &args![location, radius, list]);
        let candidates = f.at(-0x1c);
        e.call(CANDIDATE_ARRAY_CONSTRUCT, &args![candidates]);
        let holder = f.at(-0x40);
        let mut navmesh_index = 0u32;
        loop {
            let count = e.call(ARRAY_COUNT, &args![list]).u32();
            if navmesh_index >= count {
                break;
            }
            let entry = e
                .call(POINTER_ARRAY_ELEMENT, &args![list, navmesh_index])
                .u32();
            e.call(NAV_HOLDER_COPY_CONSTRUCT, &args![holder, entry]);
            let mut triangle = 0u32;
            loop {
                let navmesh = e.call(NI_POINTER_GET_FROM_FIELD, &args![holder]).u32();
                let triangle_count = e.call(NAVMESH_TRIANGLE_COUNT, &args![navmesh]).u32();
                if triangle >= triangle_count {
                    break;
                }
                let navmesh = e.call(NI_POINTER_GET_FROM_FIELD, &args![holder]).u32();
                let record = e
                    .call(NAVMESH_GET_TRIANGLE, &args![navmesh, triangle & 0xffff])
                    .u32();
                for edge in 0..3u32 {
                    let neighbour = e.call(TRIANGLE_NEIGHBOUR, &args![record, edge]).u16();
                    if neighbour != 0xffff {
                        continue;
                    }
                    let corner = e
                        .call(TRIANGLE_GET_VERTEX_INDEX, &args![record, edge])
                        .u16() as u32;
                    let navmesh = e.call(NI_POINTER_GET_FROM_FIELD, &args![holder]).u32();
                    let start = e.call(NAVMESH_GET_VERTEX, &args![navmesh, corner]).u32();
                    let next_edge = if edge + 1 == 3 { 0 } else { edge + 1 };
                    let corner = e
                        .call(TRIANGLE_GET_VERTEX_INDEX, &args![record, next_edge])
                        .u16() as u32;
                    let navmesh = e.call(NI_POINTER_GET_FROM_FIELD, &args![holder]).u32();
                    let end = e.call(NAVMESH_GET_VERTEX, &args![navmesh, corner]).u32();
                    let position = e
                        .call(LOCATION_COPY_POSITION, &args![location, f.at(-0xa4)])
                        .u32();
                    let closest = f.at(-0x5c);
                    e.call(POINT_SEGMENT_CLOSEST, &args![closest, position, start, end]);
                    let position = e
                        .call(LOCATION_COPY_POSITION, &args![location, f.at(-0xb0)])
                        .u32();
                    let difference = f.at(-0x74);
                    e.call(POINT3_SUBTRACT, &args![position, difference, closest]);
                    let distance = e
                        .call(POINT_PLANAR_LENGTH_SQUARED, &args![difference])
                        .f32();
                    let within_radius = distance < radius_squared;
                    if !within_radius {
                        continue;
                    }
                    let candidate = f.at(-0x94);
                    fn_006d4550(e, Ptr::new(candidate));
                    let navmesh = e.call(NI_POINTER_GET_FROM_FIELD, &args![holder]).u32();
                    e.mem.set_u32(candidate, navmesh);
                    e.mem.set_u16(candidate + 4, triangle as u16);
                    e.mem.set_u32(candidate + 8, edge);
                    let navmesh = e.call(NI_POINTER_GET_FROM_FIELD, &args![holder]).u32();
                    let center = e
                        .call(
                            NAVMESH_GET_CENTER,
                            &args![navmesh, f.at(-0xbc), triangle & 0xffff],
                        )
                        .u32();
                    let from_closest = e
                        .call(POINT3_SUBTRACT, &args![center, f.at(-0xc8), closest])
                        .u32();
                    let fraction = e.global::<f32>(CENTER_FRACTION);
                    let step = e
                        .call(
                            POINT3_TIMES_SCALAR,
                            &args![from_closest, f.at(-0xd4), fraction],
                        )
                        .u32();
                    let moved = e.call(POINT3_ADD, &args![closest, f.at(-0xe0), step]).u32();
                    copy_words(e, candidate + 0xc, moved, 3);
                    e.mem.set_f32(candidate + 0x18, distance);
                    let height = e.mem.f32(difference + 8);
                    e.mem.set_f32(candidate + 0x1c, height);
                    e.call(CANDIDATE_ARRAY_ADD, &args![candidates, candidate]);
                }
                triangle += 1;
            }
            e.call(NAV_HOLDER_RELEASE, &args![holder]);
            navmesh_index += 1;
        }
        if e.call(ARRAY_IS_EMPTY, &args![candidates]).bool() {
            e.call(CANDIDATE_ARRAY_DESTRUCT, &args![candidates]);
            e.call(NAVMESH_LIST_DESTRUCT, &args![list]);
            return 0;
        }
        let count = e.call(ARRAY_COUNT, &args![candidates]).u32();
        let base = e
            .call(CANDIDATE_ARRAY_ELEMENT, &args![candidates, 0u32])
            .u32();
        e.call(QSORT, &args![base, count, 0x20u32, 0x006d_4020u32]);
        let count = e.call(ARRAY_COUNT, &args![candidates]).u32();
        let taken = e.call(MINIMUM_UNSIGNED, &args![limit, count]).u32();
        for index in 0..taken {
            let candidate = e
                .call(CANDIDATE_ARRAY_ELEMENT, &args![candidates, index])
                .u32();
            let point = e
                .call(CANDIDATE_TO_PATH_POINT, &args![f.at(-0xfc), candidate])
                .u32();
            e.call(PATH_POINT_ARRAY_ADD, &args![out, point]);
        }
        e.call(CANDIDATE_ARRAY_DESTRUCT, &args![candidates]);
        e.call(NAVMESH_LIST_DESTRUCT, &args![list]);
        if taken == 0 {
            0
        } else {
            2
        }
    })
}

// Translated from 006d4550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the path point `fn_006d4120` builds its candidates on:
/// runs `006a0480`, returns `this`.
pub fn fn_006d4550(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(PATH_POINT_CONSTRUCT, &args![this]);
    this
}

// Translated from 006d4570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds a point for `location` within `radius` and stores it in `out`
/// (navmesh info, triangle `u16` at +4, edge at +8, position at +0xC).
///
/// A location without navmesh info asks `fn_006d4120` for one point and
/// copies it. Otherwise it floods the triangles within `radius` of the
/// location: starting from the location's own triangle it marks, per
/// triangle, which of the three edges lie closer than `radius` to the
/// location (`mark_near_edges`), and follows the marked edges that have a
/// neighbour to triangles not yet in the visited list. The marked edges
/// without a neighbour go to a list; the one whose closest point is
/// nearest to the location (squared length) is the result. Fails when the
/// location has no triangle or no such edge exists.
pub fn fn_006d4570(e: &mut Engine, location: Ptr, radius: f32, out: Ptr) -> bool {
    with_frame(e, 0x180, |e, f| {
        e.call(LOCATION_RESOLVE_NAVMESH_INFO, &args![location, 0u32]);
        if !e.call(LOCATION_HAS_NAVMESH_INFO, &args![location]).bool() {
            let found = f.at(-0x1c);
            e.call(PATH_POINT_ARRAY_CONSTRUCT, &args![found]);
            fn_006d4120(e, location, 1, Ptr::new(found));
            let result = if e.call(ARRAY_IS_EMPTY, &args![found]).bool() {
                false
            } else {
                let first = e.call(PATH_POINT_ARRAY_ELEMENT, &args![found, 0u32]).u32();
                e.call(PATH_POINT_ASSIGN, &args![out, first]);
                true
            };
            e.call(PATH_POINT_ARRAY_DESTRUCT, &args![found]);
            return result;
        }
        let border_edges = f.at(-0x4c);
        e.call(EDGE_ARRAY_CONSTRUCT, &args![border_edges]);
        let visited = f.at(-0x2c);
        e.call(TRIANGLE_ARRAY_CONSTRUCT, &args![visited]);
        let holder = f.at(-0x50);
        e.call(NAV_HOLDER_CONSTRUCT, &args![holder]);
        let triangle = f.at(-0x58);
        let release = |e: &mut Engine| {
            e.call(NAV_HOLDER_RELEASE, &args![holder]);
            e.call(TRIANGLE_ARRAY_DESTRUCT, &args![visited]);
            e.call(EDGE_ARRAY_DESTRUCT, &args![border_edges]);
        };
        if !e
            .call(
                LOCATION_GET_NAVMESH_AND_TRIANGLE,
                &args![location, holder, triangle],
            )
            .bool()
        {
            release(e);
            return false;
        }
        let start = f.at(-0x3c);
        e.call(TRIANGLE_RECORD_CONSTRUCT, &args![start]);
        let start_triangle = e.mem.u16(triangle) as u32;
        let navmesh = e.call(NAV_HOLDER_GET, &args![holder]).u32();
        let reference = e
            .call(
                TRIANGLE_REFERENCE_CONSTRUCT,
                &args![f.at(-0x128), navmesh, start_triangle],
            )
            .u32();
        e.call(TRIANGLE_REFERENCE_ASSIGN, &args![start, reference]);
        let marks = EdgeMarks {
            location,
            radius,
            first_scratch: -0x68,
            second_scratch: -0x74,
            position_scratch: -0x134,
        };
        mark_near_edges(
            e,
            f,
            &marks,
            MarkSource::Holder(holder),
            start_triangle,
            start,
        );
        e.call(TRIANGLE_ARRAY_ADD, &args![visited, start]);

        let mut index = 0u32;
        loop {
            let count = e.call(ARRAY_COUNT, &args![visited]).u32();
            if index >= count {
                break;
            }
            let current = f.at(-0x84);
            let element = e.call(ARRAY_ELEMENT_006A1440, &args![visited, index]).u32();
            fn_006d4ce0(e, Ptr::new(current), Ptr::new(element));
            for edge in 0..3u32 {
                if e.mem.u8(current + 8 + edge) == 0 {
                    continue;
                }
                let current_navmesh = e.mem.u32(current);
                let current_triangle = e.mem.u16(current + 4) as u32;
                let record = e
                    .call(
                        NAVMESH_GET_TRIANGLE,
                        &args![current_navmesh, current_triangle],
                    )
                    .u32();
                let edge_reference = f.at(-0x94);
                e.call(
                    EDGE_REFERENCE_CONSTRUCT,
                    &args![edge_reference, current_navmesh, current_triangle, edge],
                );
                let neighbour = e.call(TRIANGLE_NEIGHBOUR, &args![record, edge]).u16();
                if neighbour == 0xffff {
                    e.call(EDGE_ARRAY_ADD, &args![border_edges, edge_reference]);
                    continue;
                }
                let other = f.at(-0xa4);
                e.call(TRIANGLE_RECORD_CONSTRUCT, &args![other]);
                let current_navmesh = e.mem.u32(current);
                if !e
                    .call(
                        NAVMESH_GET_MATCHING_EDGE,
                        &args![current_navmesh, edge_reference, other],
                    )
                    .bool()
                {
                    continue;
                }
                let mut seen = false;
                let mut probe = 0u32;
                loop {
                    let count = e.call(ARRAY_COUNT, &args![visited]).u32();
                    if probe >= count {
                        break;
                    }
                    let element = e.call(ARRAY_ELEMENT_006A1440, &args![visited, probe]).u32();
                    if fn_006d4ca0(e, Ptr::new(element), Ptr::new(other)) {
                        seen = true;
                        break;
                    }
                    probe += 1;
                }
                if seen {
                    continue;
                }
                let next = f.at(-0xb8);
                e.call(TRIANGLE_RECORD_CONSTRUCT, &args![next]);
                e.call(TRIANGLE_REFERENCE_ASSIGN, &args![next, other]);
                let other_triangle = e.mem.u16(other + 4) as u32;
                let marks = EdgeMarks {
                    location,
                    radius,
                    first_scratch: -0xc8,
                    second_scratch: -0xd4,
                    position_scratch: -0x140,
                };
                mark_near_edges(
                    e,
                    f,
                    &marks,
                    MarkSource::Reference(other),
                    other_triangle,
                    next,
                );
                e.call(TRIANGLE_ARRAY_ADD, &args![visited, next]);
            }
            index += 1;
        }

        if e.call(ARRAY_IS_EMPTY, &args![border_edges]).bool() {
            release(e);
            return false;
        }
        let best = f.at(-0xf0);
        e.call(TRIANGLE_RECORD_CONSTRUCT, &args![best]);
        let best_point = f.at(-0xe4);
        e.call(POINT3_DEFAULT_CONSTRUCTOR, &args![best_point]);
        let mut best_distance = e.global::<f32>(FLOAT_LARGEST);
        let mut index = 0u32;
        loop {
            let count = e.call(ARRAY_COUNT, &args![border_edges]).u32();
            if index >= count {
                break;
            }
            let end_a = f.at(-0x100);
            e.call(POINT3_DEFAULT_CONSTRUCTOR, &args![end_a]);
            let end_b = f.at(-0x11c);
            e.call(POINT3_DEFAULT_CONSTRUCTOR, &args![end_b]);
            let element = e
                .call(ARRAY_ELEMENT_006A1440, &args![border_edges, index])
                .u32();
            let edge = e.mem.u32(element + 8);
            let element = e
                .call(ARRAY_ELEMENT_006A1440, &args![border_edges, index])
                .u32();
            let edge_triangle = e.mem.u16(element + 4) as u32;
            let element = e
                .call(ARRAY_ELEMENT_006A1440, &args![border_edges, index])
                .u32();
            let edge_navmesh = e.mem.u32(element);
            e.call(
                NAVMESH_EDGE_POINTS,
                &args![edge_navmesh, edge_triangle, edge, end_a, end_b],
            );
            let closest = f.at(-0x110);
            let position = e
                .call(LOCATION_COPY_POSITION, &args![location, f.at(-0x14c)])
                .u32();
            e.call(
                POINT_SEGMENT_CLOSEST,
                &args![closest, position, end_a, end_b],
            );
            let position = e
                .call(LOCATION_COPY_POSITION, &args![location, f.at(-0x158)])
                .u32();
            let difference = e
                .call(POINT3_SUBTRACT, &args![closest, f.at(-0x164), position])
                .u32();
            let distance = e.call(POINT3_LENGTH_SQUARED, &args![difference]).f32();
            if distance < best_distance {
                best_distance = distance;
                copy_words(e, best_point, closest, 3);
                let element = e
                    .call(ARRAY_ELEMENT_006A1440, &args![border_edges, index])
                    .u32();
                e.call(EDGE_REFERENCE_ASSIGN, &args![best, element]);
            }
            index += 1;
        }
        let best_navmesh = e.mem.u32(best);
        let info = e.call(NAVMESH_GET_INFO, &args![best_navmesh]).u32();
        e.mem.set_u32(out.addr(), info);
        let best_triangle = e.mem.u16(best + 4);
        e.mem.set_u16(out.addr() + 4, best_triangle);
        let best_edge = e.mem.u32(best + 8);
        e.mem.set_u32(out.addr() + 8, best_edge);
        copy_words(e, out.addr() + 0xc, best_point, 3);
        release(e);
        true
    })
}

/// Where `mark_near_edges` takes the navmesh from.
enum MarkSource {
    /// A navmesh holder (read through `NiPointer::Get`).
    Holder(u32),
    /// A triangle reference whose first word is the navmesh.
    Reference(u32),
}

/// What `mark_near_edges` needs besides the engine and the frame: the
/// location, the radius and the `EBP`-relative offsets of the scratch
/// locals the two call sites use.
struct EdgeMarks {
    location: Ptr,
    radius: f32,
    first_scratch: i32,
    second_scratch: i32,
    position_scratch: i32,
}

/// The part of `fn_006d4570` that marks the edges of a triangle that lie
/// closer than the radius to the location: for each of the three edges it
/// gets the edge's end points (`0068f0c0`, into two scratch points) and sets
/// the byte `record + 8 + edge` when the radius is greater than the
/// distance from the location to the segment (`006b9b10`, compared in
/// `ST0`). The game has this code twice in the function (for the first
/// triangle and for each new neighbour); only the scratch locals differ.
fn mark_near_edges(
    e: &mut Engine,
    f: Frame,
    marks: &EdgeMarks,
    source: MarkSource,
    triangle: u32,
    record: u32,
) {
    for edge in 0..3u32 {
        let end_a = f.at(marks.first_scratch);
        let end_b = f.at(marks.second_scratch);
        e.call(POINT3_DEFAULT_CONSTRUCTOR, &args![end_a]);
        e.call(POINT3_DEFAULT_CONSTRUCTOR, &args![end_b]);
        let navmesh = match source {
            MarkSource::Holder(holder) => e.call(NI_POINTER_GET_FROM_FIELD, &args![holder]).u32(),
            MarkSource::Reference(reference) => e.mem.u32(reference),
        };
        e.call(
            NAVMESH_EDGE_POINTS,
            &args![navmesh, triangle, edge, end_a, end_b],
        );
        let position = e
            .call(
                LOCATION_COPY_POSITION,
                &args![marks.location, f.at(marks.position_scratch)],
            )
            .u32();
        let distance = e
            .call(POINT_SEGMENT_DISTANCE, &args![position, end_a, end_b, 0u32])
            .f64();
        e.mem
            .set_u8(record + 8 + edge, ((marks.radius as f64) > distance) as u8);
    }
}

// Translated from 006d4ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether two triangle references are equal: the same navmesh word at +0
/// and the same triangle `u16` at +4.
pub fn fn_006d4ca0(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.mem.u32(this.addr()) == e.mem.u32(other.addr())
        && e.mem.u16(this.addr() + 4) == e.mem.u16(other.addr() + 4)
}

// Translated from 006d4ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assigns a triangle record: the reference (`0069a690`), then the `u16`
/// at +8 and the byte at +10 (the three edge marks). Returns `this`.
pub fn fn_006d4ce0(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    e.call(TRIANGLE_REFERENCE_ASSIGN, &args![this, other]);
    let marks = e.mem.u16(other.addr() + 8);
    e.mem.set_u16(this.addr() + 8, marks);
    let mark = e.mem.u8(other.addr() + 10);
    e.mem.set_u8(this.addr() + 10, mark);
    this
}

// Translated from 006d4d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Pathing::BuildTeleportDoorPath` (Xbox PDB): builds a path between two
/// locations through teleport doors into `path` (a `TeleportPath`, cleared
/// first, both arrays given room for 10 entries). On success the path's
/// start (+0x20) and end (+0x2C) are the two locations' positions. The
/// scope timer of line 0x582 covers the call. `argument_4`, `argument_5`,
/// `argument_6` and `argument_7` go to `TeleportDoorSearch::BuildNodePath`
/// unchanged.
#[allow(clippy::too_many_arguments)]
pub fn pathing_build_teleport_door_path(
    e: &mut Engine,
    from: Ptr,
    to: Ptr,
    path: Ptr,
    argument_4: u32,
    argument_5: f32,
    argument_6: u32,
    argument_7: u32,
) -> bool {
    with_frame(e, 0x20f0, |e, f| {
        let timer = f.at(-0x20c8);
        scope_enter(e, timer, 0x582);
        let search = f.at(-0x20c4);
        e.call(TELEPORT_SEARCH_CONSTRUCT, &args![search]);
        e.call(TELEPORT_PATH_CLEAR, &args![path]);
        e.call(TELEPORT_PATH_RESERVE_STEPS, &args![path, 10u32]);
        e.call(
            TELEPORT_PATH_RESERVE_DOORS,
            &args![path.addr() + 0x10, 10u32],
        );
        let found = e
            .call(
                TELEPORT_SEARCH_BUILD_NODE_PATH,
                &args![search, from, to, path, argument_4, argument_5, argument_6, argument_7],
            )
            .bool();
        if found {
            let position = e
                .call(LOCATION_COPY_POSITION, &args![from, f.at(-0x20d4)])
                .u32();
            copy_words(e, path.addr() + 0x20, position, 3);
            let position = e
                .call(LOCATION_COPY_POSITION, &args![to, f.at(-0x20e0)])
                .u32();
            copy_words(e, path.addr() + 0x2c, position, 3);
        }
        e.call(TELEPORT_SEARCH_DESTRUCT, &args![search]);
        scope_leave(e, timer);
        found
    })
}

// Translated from 006d4eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Pathing::ComputeTeleportDoorPathLength` (Xbox PDB): builds the
/// teleport-door path between the locations in a local `TeleportPath`
/// (`pathing_build_teleport_door_path` with 0 for its fourth argument) and returns its length
/// (`TeleportPath::ComputeLength`), or `FLT_MAX` when there is no path.
pub fn pathing_compute_teleport_door_path_length(
    e: &mut Engine,
    from: Ptr,
    to: Ptr,
    distance: f32,
    argument_a: u32,
    argument_b: u32,
) -> f32 {
    with_frame(e, 0x50, |e, f| {
        let path = f.at(-0x44);
        e.call(TELEPORT_PATH_CONSTRUCT, &args![path]);
        let result = if pathing_build_teleport_door_path(
            e,
            from,
            to,
            Ptr::new(path),
            0,
            distance,
            argument_a,
            argument_b,
        ) {
            e.call(TELEPORT_PATH_COMPUTE_LENGTH, &args![path]).f32()
        } else {
            e.global::<f32>(FLOAT_LARGEST)
        };
        e.call(TELEPORT_PATH_DESTRUCT, &args![path]);
        result
    })
}

// Translated from 006d4f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Pathing::BuildLowPath` (Xbox PDB): when both locations have navmesh
/// info, runs the low-path search (`006b8c50`) between them; on success it
/// fills `path`: the first array gets a 12-byte step (flag, two values the
/// info gives) for the first info and then, for every reference the
/// search found, a step for the next info; the second array (+0x10) gets
/// an entry per reference (the reference word and its position); the path's
/// start (+0x20) and end (+0x2C) are the locations' positions. Otherwise
/// (a location without info, or the search failed) it falls back to the
/// teleport-door search with the fourth argument 0, the `float` 0.0 and
/// the two arguments given here.
pub fn pathing_build_low_path(
    e: &mut Engine,
    from: Ptr,
    to: Ptr,
    path: Ptr,
    argument_4: u32,
    argument_5: u32,
) -> bool {
    with_frame(e, 0x41f0, |e, f| {
        if e.call(LOCATION_RESOLVE_NAVMESH_INFO, &args![from, 0u32])
            .bool()
            && e.call(LOCATION_RESOLVE_NAVMESH_INFO, &args![to, 0u32])
                .bool()
        {
            let search = f.at(-0x4164);
            e.call(LOW_PATH_SEARCH_CONSTRUCT, &args![search]);
            let infos = f.at(-0x4184);
            e.call(INFO_ARRAY_CONSTRUCT, &args![infos]);
            let references = f.at(-0x4174);
            e.call(REFERENCE_ARRAY_CONSTRUCT, &args![references]);
            let to_info = e.call(LOCATION_NAVMESH_INFO_FIELD, &args![to]).u32();
            let to_position = e
                .call(LOCATION_COPY_POSITION, &args![to, f.at(-0x41bc)])
                .u32();
            let from_info = e.call(LOCATION_NAVMESH_INFO_FIELD, &args![from]).u32();
            let from_position = e
                .call(LOCATION_COPY_POSITION, &args![from, f.at(-0x41c8)])
                .u32();
            let found = e
                .call(
                    LOW_PATH_SEARCH_RUN,
                    &args![
                        search,
                        argument_4,
                        from_position,
                        from_info,
                        to_position,
                        to_info,
                        infos,
                        references,
                        argument_5
                    ],
                )
                .bool();
            if found {
                // The first step, then one step and one door entry per
                // reference.
                low_path_step(e, path, infos, 0, f.at(-0x4190));
                let mut index = 0u32;
                loop {
                    let count = e.call(ARRAY_COUNT, &args![references]).u32();
                    if index >= count {
                        break;
                    }
                    let slot = e
                        .call(POINTER_ARRAY_ELEMENT, &args![references, index])
                        .u32();
                    if e.mem.u32(slot) != 0 {
                        let entry = f.at(-0x41b0);
                        fn_006d5320(e, Ptr::new(entry));
                        let slot = e
                            .call(POINTER_ARRAY_ELEMENT, &args![references, index])
                            .u32();
                        let reference = e.mem.u32(slot);
                        e.mem.set_u32(entry, reference);
                        let slot = e
                            .call(POINTER_ARRAY_ELEMENT, &args![references, index])
                            .u32();
                        let reference = e.mem.u32(slot);
                        let owner = e
                            .call(REFERENCE_GET_POSITION_OWNER, &args![reference])
                            .u32();
                        let position = e.call(NODE_LOCATION, &args![owner]).u32();
                        copy_words(e, entry + 4, position, 3);
                        e.call(PATH_DOOR_ADD, &args![path.addr() + 0x10, entry]);
                        low_path_step(e, path, infos, index + 1, f.at(-0x41a0));
                    }
                    index += 1;
                }
                let position = e
                    .call(LOCATION_COPY_POSITION, &args![from, f.at(-0x41d4)])
                    .u32();
                copy_words(e, path.addr() + 0x20, position, 3);
                let position = e
                    .call(LOCATION_COPY_POSITION, &args![to, f.at(-0x41e0)])
                    .u32();
                copy_words(e, path.addr() + 0x2c, position, 3);
                e.call(REFERENCE_ARRAY_DESTRUCT, &args![references]);
                e.call(INFO_ARRAY_DESTRUCT, &args![infos]);
                e.call(LOW_PATH_SEARCH_DESTRUCT, &args![search]);
                return true;
            }
            e.call(REFERENCE_ARRAY_DESTRUCT, &args![references]);
            e.call(INFO_ARRAY_DESTRUCT, &args![infos]);
            e.call(LOW_PATH_SEARCH_DESTRUCT, &args![search]);
        }
        let search = f.at(-0x20c4);
        e.call(TELEPORT_SEARCH_CONSTRUCT, &args![search]);
        let found = e
            .call(
                TELEPORT_SEARCH_BUILD_NODE_PATH,
                &args![search, from, to, path, 0u32, 0.0f32, argument_4, argument_5],
            )
            .bool();
        e.call(TELEPORT_SEARCH_DESTRUCT, &args![search]);
        found
    })
}

/// Appends to the path's first array the step for info `index` of the info
/// array: `step` is the 12-byte local (flag byte, then the two values
/// `006b77b0` and `00690800` give for the info; the flag is whether
/// `00690800` is not zero).
fn low_path_step(e: &mut Engine, path: Ptr, infos: u32, index: u32, step: u32) {
    let slot = e.call(POINTER_ARRAY_ELEMENT, &args![infos, index]).u32();
    let info = e.mem.u32(slot);
    let first = e.call(INFO_VALUE_A, &args![info]).u32();
    e.mem.set_u32(step + 4, first);
    let slot = e.call(POINTER_ARRAY_ELEMENT, &args![infos, index]).u32();
    let info = e.mem.u32(slot);
    let second = e.call(INFO_VALUE_B, &args![info]).u32();
    e.mem.set_u32(step + 8, second);
    let slot = e.call(POINTER_ARRAY_ELEMENT, &args![infos, index]).u32();
    let info = e.mem.u32(slot);
    let flag = e.call(INFO_VALUE_A, &args![info]).u32();
    e.mem.set_u8(step, (flag != 0) as u8);
    e.call(PATH_STEP_ADD, &args![path, step]);
}

// Translated from 006d5320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the 0x10-byte door entry of a low path: constructs the
/// point at +4 (`NiPoint3` constructor). Returns `this`.
pub fn fn_006d5320(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(POINT3_DEFAULT_CONSTRUCTOR, &args![this.addr() + 4]);
    this
}

// Translated from 006d5340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds the base solver for a request and solution on the stack (as
/// `fn_006d0b10` does), calls its method `006c9170` with the two extra
/// words and destroys the solver. Returns what the method returned.
pub fn fn_006d5340(
    e: &mut Engine,
    request: Ptr<PathingRequest>,
    solution: Ptr<PathingSolution>,
    argument_3: u32,
    argument_4: u32,
) -> u32 {
    with_frame(e, 0x38, |e, f| {
        let solver = f.at(-0x40);
        e.call(BASE_SOLVER_CONSTRUCT, &args![solver, request, solution]);
        let result = e
            .call(
                BASE_SOLVER_METHOD_006C9170,
                &args![solver, argument_3, argument_4],
            )
            .u32();
        fn_006d0b80(e, Ptr::new(solver));
        result
    })
}

// Translated from 006d53b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds the base solver for a request and solution on the stack, calls
/// its method `006c93d0` and destroys the solver. Returns what the method
/// returned.
pub fn fn_006d53b0(
    e: &mut Engine,
    request: Ptr<PathingRequest>,
    solution: Ptr<PathingSolution>,
) -> u32 {
    with_frame(e, 0x38, |e, f| {
        let solver = f.at(-0x40);
        e.call(BASE_SOLVER_CONSTRUCT, &args![solver, request, solution]);
        let result = e.call(BASE_SOLVER_METHOD_006C93D0, &args![solver]).u32();
        fn_006d0b80(e, Ptr::new(solver));
        result
    })
}

// Translated from 006d5420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds the base solver for a request and solution on the stack, calls
/// its method `006c93f0` with one extra word and destroys the solver.
/// Returns what the method returned.
pub fn fn_006d5420(
    e: &mut Engine,
    request: Ptr<PathingRequest>,
    solution: Ptr<PathingSolution>,
    argument_3: u32,
) -> u32 {
    with_frame(e, 0x38, |e, f| {
        let solver = f.at(-0x40);
        e.call(BASE_SOLVER_CONSTRUCT, &args![solver, request, solution]);
        let result = e
            .call(BASE_SOLVER_METHOD_006C93F0, &args![solver, argument_3])
            .u32();
        fn_006d0b80(e, Ptr::new(solver));
        result
    })
}

/// Stores into `out_length` the straight-line distance between the two
/// locations (the length of `to - from`); the three offsets are the
/// `EBP`-relative scratch locals of the call site (the copy of the `from`
/// position, the difference and the copy of the `to` position).
fn store_straight_distance(
    e: &mut Engine,
    f: Frame,
    from: Ptr,
    to: Ptr,
    out_length: Ptr,
    scratch: [i32; 3],
) {
    let from_position = e
        .call(LOCATION_COPY_POSITION, &args![from, f.at(scratch[0])])
        .u32();
    let to_position = e
        .call(LOCATION_COPY_POSITION, &args![to, f.at(scratch[2])])
        .u32();
    let difference = e
        .call(
            POINT3_SUBTRACT,
            &args![to_position, f.at(scratch[1]), from_position],
        )
        .u32();
    let length = e.call(POINT3_LENGTH, &args![difference]).f32();
    e.mem.set_f32(out_length.addr(), length);
}

// Translated from 006d5490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds a path between two locations on the navmeshes and straightens it.
///
/// The navmeshes the exe lists (`006d91d0`) give an array of their infos.
/// When a location does not resolve to a navmesh and triangle, the
/// function stores the straight-line distance in `out_length` (if given)
/// and fails; the one exception is both locations failing to resolve,
/// which succeeds. The search (`006a6b70`) fills a path (the `out_path`
/// array, or a local one when it is null) and a node array; the first and
/// last path nodes are moved to the two positions. With more than two
/// nodes and an output wanted, the path is smoothed: every node is tried
/// against the following ones with the portals of the node array
/// (`006ba7e0` narrows the left and right portal point, `00696540` tests
/// the line), the nodes that can be skipped are removed, and the remaining
/// inner nodes are moved to the nearer end of their portal. `out_length`
/// then gets the summed length of the path. When the debug flag at
/// `011d74ec` is set, the hook `0040fbe0` (an empty function in this
/// build) is called with the path at the stages the game draws, and the
/// flag is cleared at the end.
pub fn fn_006d5490(
    e: &mut Engine,
    from: Ptr,
    to: Ptr,
    request: Ptr,
    out_length: Ptr,
    out_path: Ptr,
) -> bool {
    with_frame(e, 0x23f0, |e, f| {
        let list = f.at(-0x21f0);
        e.call(NAVMESH_LIST_CONSTRUCT, &args![list]);
        e.call(NAVMESH_LIST_FILL, &args![list]);
        let infos = f.at(-0x21c8);
        e.call(INFO_ARRAY_CONSTRUCT, &args![infos]);
        let mut index = 0u32;
        loop {
            let count = e.call(ARRAY_COUNT, &args![list]).u32();
            if index >= count {
                break;
            }
            let entry = e.call(POINTER_ARRAY_ELEMENT, &args![list, index]).u32();
            let navmesh = e.call(NI_POINTER_GET_FROM_FIELD, &args![entry]).u32();
            let info = e.call(NAVMESH_GET_INFO, &args![navmesh]).u32();
            e.mem.set_u32(f.at(-0x22f0), info);
            e.call(INFO_ARRAY_ADD, &args![infos, f.at(-0x22f0)]);
            index += 1;
        }
        let wants_length = out_length.addr() != 0;
        let release_lists = |e: &mut Engine| {
            e.call(INFO_ARRAY_DESTRUCT, &args![infos]);
            e.call(NAVMESH_LIST_DESTRUCT, &args![list]);
        };

        if !e.call(LOCATION_RESOLVE_CLOSEST, &args![from]).bool() {
            if wants_length {
                store_straight_distance(e, f, from, to, out_length, [-0x2308, -0x2314, -0x22fc]);
            }
            let to_resolves = e.call(LOCATION_RESOLVE_CLOSEST, &args![to]).bool();
            release_lists(e);
            return !to_resolves;
        }
        if !e.call(LOCATION_RESOLVE_CLOSEST, &args![to]).bool() {
            if wants_length {
                store_straight_distance(e, f, from, to, out_length, [-0x2330, -0x233c, -0x2324]);
            }
            release_lists(e);
            return false;
        }

        let from_holder = f.at(-0xd8);
        e.call(NAV_HOLDER_CONSTRUCT, &args![from_holder]);
        let from_triangle = f.at(-0x2208);
        e.mem.set_u16(from_triangle, 0xffff);
        if !e
            .call(
                LOCATION_GET_NAVMESH_AND_TRIANGLE,
                &args![from, from_holder, from_triangle],
            )
            .bool()
        {
            if wants_length {
                store_straight_distance(e, f, from, to, out_length, [-0x2358, -0x2364, -0x234c]);
            }
            e.call(NAV_HOLDER_RELEASE, &args![from_holder]);
            release_lists(e);
            return false;
        }
        let from_node = f.at(-0x21b4);
        let navmesh = e.call(NAV_HOLDER_GET, &args![from_holder]).u32();
        e.mem.set_u32(from_node, navmesh);
        let triangle = e.mem.u16(from_triangle);
        e.mem.set_u16(from_node + 4, triangle);
        e.call(LOCATION_COPY_POSITION, &args![from, f.at(-0x24)]);

        let to_holder = f.at(-0x21e0);
        e.call(NAV_HOLDER_CONSTRUCT, &args![to_holder]);
        let to_triangle = f.at(-0xdc);
        e.mem.set_u16(to_triangle, 0xffff);
        if !e
            .call(
                LOCATION_GET_NAVMESH_AND_TRIANGLE,
                &args![to, to_holder, to_triangle],
            )
            .bool()
        {
            if wants_length {
                store_straight_distance(e, f, from, to, out_length, [-0x2380, -0x238c, -0x2374]);
            }
            e.call(NAV_HOLDER_RELEASE, &args![to_holder]);
            e.call(NAV_HOLDER_RELEASE, &args![from_holder]);
            release_lists(e);
            return false;
        }
        let to_node = f.at(-0x2204);
        let navmesh = e.call(NAV_HOLDER_GET, &args![to_holder]).u32();
        e.mem.set_u32(to_node, navmesh);
        let triangle = e.mem.u16(to_triangle);
        e.mem.set_u16(to_node + 4, triangle);
        e.call(LOCATION_COPY_POSITION, &args![to, f.at(-0x18)]);

        let search = f.at(-0x219c);
        e.call(NAVMESH_SEARCH_CONSTRUCT, &args![search]);
        let local_path = f.at(-0x21dc);
        e.call(NODE_ARRAY_A_DEFAULT_CONSTRUCT, &args![local_path]);
        let path = if out_path.addr() != 0 {
            out_path.addr()
        } else {
            local_path
        };
        e.mem.set_u32(f.at(-0x21b8), path);
        e.call(PATH_ARRAY_PREPARE, &args![path, 0u32]);
        if e.call(PATH_ARRAY_CAPACITY, &args![path]).u32() < 0x20 {
            e.call(PATH_ARRAY_RESERVE, &args![path, 0x20u32]);
        }
        let nodes = f.at(-0xf0);
        e.call(NODE_ARRAY_B_CONSTRUCT, &args![nodes, 0x20u32, 0u32, 0u32]);
        let request_copy = f.at(-0xd4);
        e.call(REQUEST_CONSTRUCT, &args![request_copy]);
        fn_006d61e0(e, Ptr::new(request_copy), request.addr());
        let release_all = |e: &mut Engine| {
            e.call(REQUEST_COPY_DESTRUCT, &args![request_copy]);
            e.call(NODE_ARRAY_B_DESTRUCT, &args![nodes]);
            e.call(NODE_ARRAY_A_DESTRUCT, &args![local_path]);
            e.call(NAVMESH_SEARCH_DESTRUCT, &args![search]);
            e.call(NAV_HOLDER_RELEASE, &args![to_holder]);
            e.call(NAV_HOLDER_RELEASE, &args![from_holder]);
            e.call(INFO_ARRAY_DESTRUCT, &args![infos]);
            e.call(NAVMESH_LIST_DESTRUCT, &args![list]);
        };
        if !e
            .call(
                NAVMESH_SEARCH_RUN,
                &args![search, request_copy, from_node, to_node, path, nodes, infos],
            )
            .bool()
        {
            if wants_length {
                store_straight_distance(e, f, from, to, out_length, [-0x23a8, -0x23b4, -0x239c]);
            }
            release_all(e);
            return false;
        }

        // The ends of the path are the two positions.
        let first = e.call(PATH_ARRAY_ELEMENT, &args![path, 0u32]).u32();
        e.mem.set_u32(f.at(-0x21a0), first);
        let position = e
            .call(LOCATION_COPY_POSITION, &args![from, f.at(-0x23c4)])
            .u32();
        fn_006d61b0(e, Ptr::new(first), Ptr::new(position));
        if e.call(ARRAY_COUNT, &args![path]).u32() > 1 {
            let count = e.call(ARRAY_COUNT, &args![path]).u32();
            let last = e
                .call(PATH_ARRAY_ELEMENT, &args![path, count.wrapping_sub(1)])
                .u32();
            e.mem.set_u32(f.at(-0x2210), last);
            let position = e
                .call(LOCATION_COPY_POSITION, &args![to, f.at(-0x23d0)])
                .u32();
            fn_006d61b0(e, Ptr::new(last), Ptr::new(position));
        }
        let debug = |e: &mut Engine| e.global::<u8>(PATH_DEBUG_FLAG) != 0;
        if debug(e) {
            e.call(DEBUG_HOOK, &args![path, 0.0f32, 0u32]);
        }

        if e.call(ARRAY_COUNT, &args![path]).u32() > 2 && (wants_length || out_path.addr() != 0) {
            smooth_path(e, f, path, nodes);
        }

        if wants_length {
            e.mem.set_f32(out_length.addr(), 0.0);
            if e.call(ARRAY_COUNT, &args![path]).u32() > 1 {
                let previous = f.at(-0x22dc);
                let first = e.call(PATH_ARRAY_ELEMENT, &args![path, 0u32]).u32();
                e.call(NODE_TO_POSITION, &args![first, previous]);
                let mut index = 1u32;
                loop {
                    let count = e.call(ARRAY_COUNT, &args![path]).u32();
                    if index >= count {
                        break;
                    }
                    let current = f.at(-0x22ec);
                    let node = e.call(PATH_ARRAY_ELEMENT, &args![path, index]).u32();
                    e.call(NODE_TO_POSITION, &args![node, current]);
                    let step = e
                        .call(POINT3_SUBTRACT, &args![current, f.at(-0x23dc), previous])
                        .u32();
                    let length = e.call(POINT3_LENGTH, &args![step]).f64();
                    let total = length + e.mem.f32(out_length.addr()) as f64;
                    e.mem.set_f32(out_length.addr(), total as f32);
                    copy_words(e, previous, current, 3);
                    index += 1;
                }
            }
        }
        if debug(e) {
            let color = e.global::<f32>(DEBUG_VALUE_FINAL);
            e.call(DEBUG_HOOK, &args![path, color, 1u32]);
        }
        e.mem.set_u8(PATH_DEBUG_FLAG, 0);
        release_all(e);
        true
    })
}

/// The smoothing part of `fn_006d5490` (the path has more than two nodes).
/// First pass: for each node `i` it takes the portal of the node array
/// entry `i` (`NavMesh` edge end points `left` and `right`, locals at
/// -0x2244 and -0x2220) and walks `j = i + 2, ...`: the portal of entry
/// `j - 1` narrows `left` and `right` (`006ba7e0`, which writes the
/// clipped point to a scratch point when it returns true; otherwise the
/// portal's own end is taken) and `00696540` decides whether the line from
/// node `i` through the narrowed portal still reaches node `j`; the nodes
/// `i + 1 .. j - 2` are removed from the path (`006db140`) and the node
/// array (`0049f3c0`). Second pass: every inner node is moved to the end
/// of its portal (the previous entry's) that is nearer to the previous
/// node.
fn smooth_path(e: &mut Engine, f: Frame, path: u32, nodes: u32) {
    let mut i = 0u32;
    loop {
        let count = e.call(ARRAY_COUNT, &args![path]).u32();
        if i >= count.wrapping_sub(2) {
            break;
        }
        let node = e.call(PATH_ARRAY_ELEMENT, &args![path, i]).u32();
        e.mem.set_u32(f.at(-0x2230), node);
        let node_position = f.at(-0x222c);
        e.call(NODE_TO_POSITION, &args![node, node_position]);
        let portal = e.call(NODE_ARRAY_B_ELEMENT, &args![nodes, i]).u32();
        e.mem.set_u32(f.at(-0x2234), portal);
        let left = f.at(-0x2244);
        let right = f.at(-0x2220);
        e.call(POINT3_DEFAULT_CONSTRUCTOR, &args![left]);
        e.call(POINT3_DEFAULT_CONSTRUCTOR, &args![right]);
        portal_points(e, portal, left, right);
        let mut j = i + 2;
        loop {
            let count = e.call(ARRAY_COUNT, &args![path]).u32();
            if j >= count {
                break;
            }
            let next = e.call(PATH_ARRAY_ELEMENT, &args![path, j]).u32();
            e.mem.set_u32(f.at(-0x2278), next);
            let next_position = f.at(-0x2274);
            e.call(NODE_TO_POSITION, &args![next, next_position]);
            let portal = e.call(NODE_ARRAY_B_ELEMENT, &args![nodes, j - 1]).u32();
            e.mem.set_u32(f.at(-0x227c), portal);
            let end_a = f.at(-0x2268);
            let end_b = f.at(-0x225c);
            e.call(POINT3_DEFAULT_CONSTRUCTOR, &args![end_a]);
            e.call(POINT3_DEFAULT_CONSTRUCTOR, &args![end_b]);
            portal_points(e, portal, end_a, end_b);
            let clipped = f.at(-0x2250);
            e.call(POINT3_DEFAULT_CONSTRUCTOR, &args![clipped]);
            let narrowed = e
                .call(
                    PORTAL_CLIP,
                    &args![end_a, end_b, node_position, left, clipped],
                )
                .bool();
            if narrowed {
                copy_words(e, left, clipped, 3);
            } else {
                copy_words(e, left, end_a, 3);
            }
            let narrowed = e
                .call(
                    PORTAL_CLIP,
                    &args![end_a, end_b, node_position, right, clipped],
                )
                .bool();
            if narrowed {
                copy_words(e, right, clipped, 3);
            } else {
                copy_words(e, right, end_b, 3);
            }
            if e.global::<u8>(PATH_DEBUG_FLAG) != 0 {
                let color = if j % 2 == 0 {
                    0.0
                } else {
                    e.global::<f32>(DEBUG_VALUE_ODD)
                };
                e.call(DEBUG_HOOK, &args![portal, j - 1, color, 3u32]);
            }
            let reaches = e
                .call(
                    LINE_REACHES,
                    &args![node_position, next_position, left, right],
                )
                .bool();
            if !reaches {
                break;
            }
            j += 1;
        }
        if i + 2 < j {
            let removed = j - i - 2;
            e.call(PATH_ARRAY_REMOVE, &args![path, i + 1, removed, 0u32]);
            e.call(NODE_ARRAY_REMOVE, &args![nodes, i, removed, 0u32]);
        }
        i += 1;
    }
    if e.global::<u8>(PATH_DEBUG_FLAG) != 0 {
        let color = e.global::<f32>(DEBUG_VALUE_ODD);
        e.call(DEBUG_HOOK, &args![path, color, 2u32]);
    }
    let mut i = 1u32;
    loop {
        let count = e.call(ARRAY_COUNT, &args![path]).u32();
        if i >= count.wrapping_sub(1) {
            break;
        }
        let node = e.call(PATH_ARRAY_ELEMENT, &args![path, i]).u32();
        e.mem.set_u32(f.at(-0x22d0), node);
        let previous = e.call(PATH_ARRAY_ELEMENT, &args![path, i - 1]).u32();
        e.mem.set_u32(f.at(-0x229c), previous);
        let portal = e.call(NODE_ARRAY_B_ELEMENT, &args![nodes, i - 1]).u32();
        e.mem.set_u32(f.at(-0x228c), portal);
        let previous_position = f.at(-0x22c0);
        e.call(NODE_TO_POSITION, &args![previous, previous_position]);
        let left = f.at(-0x22cc);
        let right = f.at(-0x22b4);
        e.call(POINT3_DEFAULT_CONSTRUCTOR, &args![left]);
        e.call(POINT3_DEFAULT_CONSTRUCTOR, &args![right]);
        portal_points(e, portal, left, right);
        let to_left = f.at(-0x22a8);
        e.call(POINT3_SUBTRACT, &args![previous_position, to_left, left]);
        let to_right = f.at(-0x2298);
        e.call(POINT3_SUBTRACT, &args![previous_position, to_right, right]);
        let left_distance = e.call(POINT3_LENGTH_SQUARED, &args![to_left]).f64();
        let right_distance = e.call(POINT3_LENGTH_SQUARED, &args![to_right]).f64();
        if right_distance > left_distance {
            fn_006d61b0(e, Ptr::new(node), Ptr::new(left));
        } else {
            fn_006d61b0(e, Ptr::new(node), Ptr::new(right));
        }
        i += 1;
    }
}

/// Reads the two end points of the node array entry's edge (`entry` holds
/// the navmesh at +0, the triangle `u16` at +4 and the edge at +8) into the
/// two points.
fn portal_points(e: &mut Engine, entry: u32, first: u32, second: u32) {
    let edge = e.mem.u32(entry + 8);
    let triangle = e.mem.u16(entry + 4) as u32;
    let navmesh = e.mem.u32(entry);
    e.call(
        NAVMESH_EDGE_POINTS,
        &args![navmesh, triangle, edge, first, second],
    );
}

// Translated from 006d61b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a point (three floats) at this + 8.
pub fn fn_006d61b0(e: &mut Engine, this: Ptr, point: Ptr) {
    for i in 0..3 {
        let value = e.mem.f32(point.addr() + 4 * i);
        e.mem.set_f32(this.addr() + 8 + 4 * i, value);
    }
}

// Translated from 006d61e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Passes `argument` to `006dadb0` on the member at this + 0x94 (the
/// `spAvoidNodeArray` of a request).
pub fn fn_006d61e0(e: &mut Engine, this: Ptr, argument: u32) {
    e.call(
        AVOID_NODE_ARRAY_ASSIGN,
        &args![this.addr() + 0x94, argument],
    );
}

// Translated from 006d6200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Pathing::ComputeLowPathDistance` (Xbox PDB): builds the low path
/// between two locations into a local `TeleportPath` (`pathing_build_low_path` with
/// arguments 0 and 1) and stores its length in `out`, or 0.0 when there is
/// no path. Returns whether there was a path.
pub fn pathing_compute_low_path_distance(e: &mut Engine, from: Ptr, to: Ptr, out: Ptr) -> bool {
    with_frame(e, 0x50, |e, f| {
        let path = f.at(-0x44);
        e.call(TELEPORT_PATH_CONSTRUCT, &args![path]);
        let found = pathing_build_low_path(e, from, to, Ptr::new(path), 0, 1);
        let length = if found {
            e.call(TELEPORT_PATH_COMPUTE_LENGTH, &args![path]).f32()
        } else {
            0.0
        };
        e.mem.set_f32(out.addr(), length);
        e.call(TELEPORT_PATH_DESTRUCT, &args![path]);
        found
    })
}

// Translated from 006d62c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `fn_006d5490` with the first three words and two null pointers
/// (no length, no output path). Returns what it returned.
pub fn fn_006d62c0(e: &mut Engine, from: Ptr, to: Ptr, request: Ptr) -> bool {
    fn_006d5490(e, from, to, request, Ptr::new(0), Ptr::new(0))
}

// Translated from 006d62e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds cover locations for a `PathingRequestCover` (Xbox PDB) and inserts
/// them, best first, into the `results` array; returns how many it found.
///
/// The PC layout of the request equals the Xbox one: `Threat` +0xB0,
/// `fMaxCoverDistance` +0xBC, `fActorCrouchHeight` +0xC0,
/// `fMaxHeightChange` +0xC4, `iMaxResults` +0xC8, the flags at +0xCC to
/// +0xD1, `DistanceProjectionVector` +0xD4 and `ActorLocation` +0xE0.
///
/// The actor location (the origin's position when it is the zero vector) is
/// the point the covers are measured from. Heights are turned into classes
/// with `006ad3b0`. For every navmesh within the maximum cover distance of
/// the origin (`006d8a20`), every cover triangle (`navmesh + 0x78` lists
/// them, flagged by `00690770`) whose center is no farther than the
/// maximum distance from the origin and within `fMaxHeightChange` of the
/// actor's height is considered: the center's side of the line from the
/// threat through the actor decides (the opposite side with
/// `bFindBehindTarget`; both sides with `bFindBothSides`); then for each of
/// its two cover edges `00691040` gives the cover class and two flags; a
/// class below 4 is accepted only for class 2 without `bHideCover`, a class
/// of 4 or more when it reaches the class of the standing or crouching
/// actor (`bCanCrouch`) and one of the flags, the crouch class or
/// `bHideCover` allows it. The cover position is the edge midpoint moved
/// along the edge's normal (its edge direction crossed with the triangle
/// normal, flattened and normalized) by the actor radius; `006ba010` must
/// accept it against the threat. The accepted location (a
/// `PathingCoverLocation`) is inserted in the results at the position the
/// sort key finds: the key is the squared distance from the actor location
/// to the triangle center (from the threat vector with
/// `bSortDistanceFromTarget`, plus `2 * dot * dot / key` of the projection
/// vector's dot product when that vector is not zero), ascending or, with
/// `bSortFarthestFirst`, descending. It stops once `iMaxResults` are found.
pub fn fn_006d62e0(e: &mut Engine, request: Ptr, results: Ptr) -> u32 {
    with_frame(e, 0x270, |e, f| {
        let mut count = 0u32;
        let capacity = e.call(COVER_MAX_RESULTS, &args![request]).u32();
        e.call(COVER_RESULTS_RESERVE, &args![results, capacity]);
        let keys = f.at(-0xb4);
        e.call(KEY_ARRAY_CONSTRUCT, &args![keys, 0x20u32, 0u32, 0u32]);

        // The actor location, or the origin's position when it is zero.
        let actor_location = e.call(COVER_ACTOR_LOCATION, &args![request]).u32();
        copy_words(e, f.at(-0x2c), actor_location, 3);
        let origin = e.call(REQUEST_ORIGIN, &args![request]).u32();
        e.call(LOCATION_COPY_POSITION, &args![origin, f.at(-0x20)]);
        let mut has_actor_location = true;
        if e.call(POINT3_EQUALS, &args![f.at(-0x2c), ZERO_VECTOR])
            .bool()
        {
            copy_words(e, f.at(-0x2c), f.at(-0x20), 3);
            has_actor_location = false;
        }

        // The threat's planar position and the planar direction from it to
        // the actor location.
        fn_006d6d60(e, request, Ptr::new(f.at(-0x84)));
        let threat_x = e.mem.f32(f.at(-0x84));
        let threat_y = e.mem.f32(f.at(-0x80));
        e.call(POINT2_CONSTRUCT, &args![f.at(-0xa0), threat_x, threat_y]);
        let actor_y = e.mem.f32(f.at(-0x28));
        let direction_y = (actor_y as f64 - e.mem.f32(f.at(-0x9c)) as f64) as f32;
        let actor_x = e.mem.f32(f.at(-0x2c));
        let direction_x = (actor_x as f64 - e.mem.f32(f.at(-0xa0)) as f64) as f32;
        e.call(
            POINT2_CONSTRUCT,
            &args![f.at(-0x78), direction_x, direction_y],
        );

        let crouch_height = fn_006d6d90(e, request);
        let crouch_class = e
            .call(HEIGHT_CLASS, &args![crouch_height, 0u32, 1u32])
            .u16();
        e.mem.set_u16(f.at(-0x94), crouch_class);
        let actor_height = e.call(COVER_ACTOR_HEIGHT, &args![request]).f32();
        let stand_class = e.call(HEIGHT_CLASS, &args![actor_height, 0u32, 1u32]).u16();
        e.mem.set_u16(f.at(-0xe8), stand_class);

        let distance = e.call(COVER_MAX_DISTANCE, &args![request]).f64();
        let distance_again = e.call(COVER_MAX_DISTANCE, &args![request]).f64();
        let maximum_squared = (distance_again * distance) as f32;
        e.mem.set_f32(f.at(-0x30), maximum_squared);

        let navmeshes = f.at(-0x108);
        e.call(
            NAVMESH_LIST_B_CONSTRUCT,
            &args![navmeshes, 8u32, 0u32, 0u32],
        );
        let reach = e.call(COVER_MAX_DISTANCE, &args![request]).f32();
        let origin = e.call(REQUEST_ORIGIN, &args![request]).u32();
        e.call(NEARBY_NAVMESHES_FILL_B, &args![origin, reach, navmeshes]);
        for offset in [
            -0xf4, -0x48, -0x114, -0x3c, -0xe4, -0x64, -0xd4, -0x70, -0x58, -0xc8,
        ] {
            e.call(POINT3_DEFAULT_CONSTRUCTOR, &args![f.at(offset)]);
        }
        let holder = f.at(-0x68);
        e.call(NAV_HOLDER_CONSTRUCT, &args![holder]);
        let crouch_flag = fn_006d6db0(e, request);
        e.mem.set_u8(f.at(-0xd5), crouch_flag);
        let hide_flag = fn_006d6e30(e, request);
        e.mem.set_u8(f.at(-0x4d), hide_flag);

        let finish = |e: &mut Engine, count: u32| {
            e.call(NAV_HOLDER_RELEASE, &args![holder]);
            e.call(NAVMESH_LIST_B_DESTRUCT, &args![navmeshes]);
            e.call(KEY_ARRAY_DESTRUCT, &args![keys]);
            count
        };
        let mut navmesh_index = 0u32;
        loop {
            let navmesh_count = e.call(ARRAY_COUNT, &args![navmeshes]).u32();
            if navmesh_index >= navmesh_count {
                return finish(e, count);
            }
            let entry = e
                .call(POINTER_ARRAY_ELEMENT, &args![navmeshes, navmesh_index])
                .u32();
            e.call(NAV_HOLDER_ASSIGN, &args![holder, entry]);
            let navmesh = e.call(NI_POINTER_GET_FROM_FIELD, &args![holder]).u32();
            let covers = e.call(NAVMESH_COVER_ARRAY, &args![navmesh]).u32();
            let cover_count = e.call(ARRAY_COUNT, &args![covers]).u16();
            e.mem.set_u16(f.at(-0x98), cover_count);
            let mut cover_index = 0u16;
            while cover_index < cover_count {
                let navmesh = e.call(NI_POINTER_GET_FROM_FIELD, &args![holder]).u32();
                let covers = e.call(NAVMESH_COVER_ARRAY, &args![navmesh]).u32();
                let item = e
                    .call(COVER_ARRAY_ELEMENT, &args![covers, cover_index as u32])
                    .u32();
                let triangle = e.mem.u16(item);
                e.mem.set_u16(f.at(-0x10), triangle);
                let navmesh = e.call(NI_POINTER_GET_FROM_FIELD, &args![holder]).u32();
                let record = e
                    .call(NAVMESH_GET_TRIANGLE, &args![navmesh, triangle as u32])
                    .u32();
                e.mem.set_u32(f.at(-0x90), record);
                cover_index += 1;
                if !e.call(TRIANGLE_IS_COVER, &args![record]).bool() {
                    continue;
                }
                let navmesh = e.call(NI_POINTER_GET_FROM_FIELD, &args![holder]).u32();
                let center = e
                    .call(
                        NAVMESH_GET_CENTER,
                        &args![navmesh, f.at(-0x1c0), triangle as u32],
                    )
                    .u32();
                copy_words(e, f.at(-0xf4), center, 3);
                let to_actor = e
                    .call(
                        POINT3_SUBTRACT,
                        &args![f.at(-0x2c), f.at(-0x1cc), f.at(-0xf4)],
                    )
                    .u32();
                copy_words(e, f.at(-0x48), to_actor, 3);
                let key = e.call(POINT3_LENGTH_SQUARED, &args![f.at(-0x48)]).f32();
                e.mem.set_f32(f.at(-0xb8), key);
                if has_actor_location {
                    e.call(
                        POINT3_SUBTRACT,
                        &args![f.at(-0x20), f.at(-0x134), f.at(-0xf4)],
                    );
                    let from_origin = e.call(POINT3_LENGTH_SQUARED, &args![f.at(-0x134)]).f64();
                    if (maximum_squared as f64) < from_origin {
                        continue;
                    }
                } else if maximum_squared < e.mem.f32(f.at(-0xb8)) {
                    continue;
                }

                // The sort key.
                let projection = e.call(COVER_PROJECTION_VECTOR, &args![request]).u32();
                copy_words(e, f.at(-0x128), projection, 3);
                if e.call(COVER_SORT_FROM_TARGET, &args![request]).bool() {
                    e.call(
                        POINT3_SUBTRACT,
                        &args![f.at(-0x84), f.at(-0x140), f.at(-0xf4)],
                    );
                    let key = e.call(POINT3_LENGTH_SQUARED, &args![f.at(-0x140)]).f32();
                    e.mem.set_f32(f.at(-0xb8), key);
                    if e.call(POINT3_NOT_EQUALS, &args![f.at(-0x128), ZERO_VECTOR])
                        .bool()
                    {
                        let dot = e.call(POINT3_DOT, &args![f.at(-0x140), f.at(-0x128)]).f32();
                        e.mem.set_f32(f.at(-0x144), dot);
                        adjust_cover_key(e, f, dot);
                    }
                } else if e
                    .call(POINT3_NOT_EQUALS, &args![f.at(-0x128), ZERO_VECTOR])
                    .bool()
                {
                    let dot = e.call(POINT3_DOT, &args![f.at(-0x48), f.at(-0x128)]).f32();
                    e.mem.set_f32(f.at(-0x148), dot);
                    adjust_cover_key(e, f, dot);
                }

                // Height and side.
                let height = e.mem.f32(f.at(-0x40));
                let height = e.call(FLOAT_ABSOLUTE_VALUE, &args![height]).f64();
                let allowed = e.call(COVER_MAX_HEIGHT_CHANGE, &args![request]).f64();
                if allowed < height {
                    continue;
                }
                let center_x = e.mem.f32(f.at(-0xf4)) as f64;
                let center_y = e.mem.f32(f.at(-0xf0)) as f64;
                let threat_x = e.mem.f32(f.at(-0xa0)) as f64;
                let threat_y = e.mem.f32(f.at(-0x9c)) as f64;
                let direction_x = e.mem.f32(f.at(-0x78)) as f64;
                let direction_y = e.mem.f32(f.at(-0x74)) as f64;
                let mut side = ((center_x - threat_x) * direction_x
                    + (center_y - threat_y) * direction_y) as f32;
                if fn_006d6df0(e, request) != 0 {
                    side = -side;
                }
                let on_the_near_side = (side as f64) > e.global::<f64>(ZERO_DOUBLE);
                if fn_006d6dd0(e, request) == 0 && !on_the_near_side {
                    continue;
                }

                let navmesh = e.call(NI_POINTER_GET_FROM_FIELD, &args![holder]).u32();
                let normal = e
                    .call(
                        TRIANGLE_NORMAL,
                        &args![navmesh, f.at(-0x1d8), triangle as u32],
                    )
                    .u32();
                copy_words(e, f.at(-0x114), normal, 3);
                for edge in 0..2u16 {
                    e.call(
                        TRIANGLE_EDGE_INFO,
                        &args![record, edge as u32, f.at(-0x88), f.at(-0x8a), f.at(-0xb9)],
                    );
                    let class = e.mem.u16(f.at(-0x88));
                    let hidden_only = e.mem.u8(f.at(-0x4d)) != 0;
                    if class < 4 {
                        if !(class == 2 && !hidden_only) {
                            continue;
                        }
                    } else {
                        let needed = if e.mem.u8(f.at(-0xd5)) != 0 {
                            e.mem.u16(f.at(-0x94))
                        } else {
                            e.mem.u16(f.at(-0xe8))
                        };
                        if class < needed {
                            continue;
                        }
                    }
                    let first_flag = e.mem.u8(f.at(-0x8a));
                    let second_flag = e.mem.u8(f.at(-0xb9));
                    let crouch_class = e.mem.u16(f.at(-0x94));
                    if first_flag == 0 && second_flag == 0 && class > crouch_class && !hidden_only {
                        continue;
                    }

                    // The cover position: the edge's midpoint moved off
                    // the edge by the actor radius.
                    let navmesh = e.call(NI_POINTER_GET_FROM_FIELD, &args![holder]).u32();
                    e.call(
                        NAVMESH_EDGE_VERTICES,
                        &args![navmesh, f.at(-0x154), triangle as u32, edge as u32],
                    );
                    let first_vertex = e.mem.u32(f.at(-0x154));
                    let second_vertex = e.mem.u32(f.at(-0x150));
                    let along = e
                        .call(
                            POINT3_SUBTRACT,
                            &args![second_vertex, f.at(-0x1e4), first_vertex],
                        )
                        .u32();
                    copy_words(e, f.at(-0x3c), along, 3);
                    let half = e.global::<f32>(EDGE_MIDPOINT_FRACTION);
                    let half_way = e
                        .call(POINT3_TIMES_SCALAR, &args![f.at(-0x3c), f.at(-0x1f0), half])
                        .u32();
                    let midpoint = e
                        .call(POINT3_ADD, &args![first_vertex, f.at(-0x1fc), half_way])
                        .u32();
                    copy_words(e, f.at(-0xe4), midpoint, 3);
                    let crossed = e
                        .call(
                            POINT3_CROSS,
                            &args![f.at(-0x3c), f.at(-0x208), f.at(-0x114)],
                        )
                        .u32();
                    copy_words(e, f.at(-0x64), crossed, 3);
                    e.mem.set_f32(f.at(-0x5c), 0.0);
                    e.call(POINT3_NORMALIZE, &args![f.at(-0x64)]);
                    let radius = e.call(REQUEST_ACTOR_RADIUS, &args![request]).f32();
                    let off_edge = e
                        .call(
                            POINT3_TIMES_SCALAR,
                            &args![f.at(-0x64), f.at(-0x214), radius],
                        )
                        .u32();
                    let position = e
                        .call(POINT3_SUBTRACT, &args![f.at(-0xe4), f.at(-0x220), off_edge])
                        .u32();
                    copy_words(e, f.at(-0xd4), position, 3);
                    for (to, from) in [(-0x70, -0xd4), (-0x6c, -0xd0)] {
                        let value = e.mem.f32(f.at(from));
                        e.mem.set_f32(f.at(to), value);
                    }
                    for (to, from, offset) in [
                        (-0x58, first_vertex, 0u32),
                        (-0x54, first_vertex, 4),
                        (-0xc8, second_vertex, 0),
                        (-0xc4, second_vertex, 4),
                    ] {
                        let value = e.mem.f32(from + offset);
                        e.mem.set_f32(f.at(to), value);
                    }
                    if !e
                        .call(
                            COVER_EDGE_TEST,
                            &args![f.at(-0x70), f.at(-0xa0), f.at(-0x58), f.at(-0xc8)],
                        )
                        .bool()
                    {
                        continue;
                    }

                    let cover = f.at(-0x1b4);
                    let navmesh = e.call(NI_POINTER_GET_FROM_FIELD, &args![holder]).u32();
                    let info = e.call(NAVMESH_GET_INFO, &args![navmesh]).u32();
                    fn_006d6e50(e, Ptr::new(cover), f.at(-0xd4), info, triangle);
                    fn_006d6ee0(
                        e,
                        Ptr::new(cover),
                        Ptr::new(second_vertex),
                        Ptr::new(first_vertex),
                        Ptr::new(f.at(-0x64)),
                    );
                    let stand_class = e.mem.u16(f.at(-0xe8));
                    let crouch_class = e.mem.u16(f.at(-0x94));
                    e.call(
                        COVER_LOCATION_SET_COVER,
                        &args![
                            cover,
                            class as u32,
                            stand_class as u32,
                            crouch_class as u32,
                            first_flag as u32,
                            second_flag as u32,
                            edge as u32
                        ],
                    );

                    // Where the key goes among the keys found so far.
                    let mut position = 0u32;
                    while position < count {
                        let farthest_first = fn_006d6e10(e, request) != 0;
                        let slot = e.call(POINTER_ARRAY_ELEMENT, &args![keys, position]).u32();
                        let existing = e.mem.f32(slot);
                        let key = e.mem.f32(f.at(-0xb8));
                        let found = if farthest_first {
                            existing < key
                        } else {
                            existing > key
                        };
                        if found {
                            break;
                        }
                        position += 1;
                    }
                    e.call(KEY_ARRAY_INSERT, &args![keys, position, f.at(-0xb8)]);
                    e.call(COVER_RESULTS_INSERT, &args![results, position, cover]);
                    count += 1;
                    let limit = e.call(COVER_MAX_RESULTS, &args![request]).u32();
                    fn_006d32c0(e, Ptr::new(cover));
                    if count >= limit {
                        return finish(e, count);
                    }
                }
            }
            navmesh_index += 1;
        }
    })
}

/// The key adjustment of `fn_006d62e0` when the request has a projection
/// vector: `key = 2 * dot * dot / key + key`, in the x87's extended
/// precision, stored as a `float`.
fn adjust_cover_key(e: &mut Engine, f: Frame, dot: f32) {
    let key = e.mem.f32(f.at(-0xb8)) as f64;
    let part = dot as f64 * dot as f64 / key;
    e.mem.set_f32(f.at(-0xb8), (part + part + key) as f32);
}

// Translated from 006d6d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the request's `Threat` (+0xB0, three floats) to `out`; returns
/// `out`.
pub fn fn_006d6d60(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    copy_words(e, out.addr(), this.addr() + 0xb0, 3);
    out
}

// Translated from 006d6d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The request's `fActorCrouchHeight` (+0xC0).
pub fn fn_006d6d90(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + 0xc0)
}

// Translated from 006d6db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The request's `bCanCrouch` (+0xCC).
pub fn fn_006d6db0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0xcc)
}

// Translated from 006d6dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The request's `bFindBothSides` (+0xCD).
pub fn fn_006d6dd0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0xcd)
}

// Translated from 006d6df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The request's `bFindBehindTarget` (+0xCE).
pub fn fn_006d6df0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0xce)
}

// Translated from 006d6e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The request's `bSortFarthestFirst` (+0xCF).
pub fn fn_006d6e10(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0xcf)
}

// Translated from 006d6e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The request's `bHideCover` (+0xD1).
pub fn fn_006d6e30(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0xd1)
}

// Translated from 006d6e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a `PathingCoverLocation` (Xbox PDB) at a navmesh point:
/// the `PathingLocation` constructor for a position, navmesh info and
/// triangle (`006dd010`), the derived vtable (`0106cbc0`) and three
/// default-constructed points at +0x28, +0x34 and +0x40. Returns `this`.
pub fn fn_006d6e50(
    e: &mut Engine,
    this: Ptr<PathingCoverLocation>,
    position: u32,
    info: u32,
    triangle: u16,
) -> Ptr<PathingCoverLocation> {
    e.call(
        LOCATION_CONSTRUCT_FROM_MESH_POINT,
        &args![this, position, info, triangle],
    );
    e.mem.set_u32(this.addr(), PATHING_COVER_LOCATION_VTABLE);
    for offset in [0x28u32, 0x34, 0x40] {
        e.call(POINT3_DEFAULT_CONSTRUCTOR, &args![this.addr() + offset]);
    }
    this
}

// Translated from 006d6ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the three points of a `PathingCoverLocation` (the left vertex
/// at +0x28, the right vertex at +0x34 and the edge direction at +0x40)
/// from the three points given.
pub fn fn_006d6ee0(
    e: &mut Engine,
    this: Ptr<PathingCoverLocation>,
    left: Ptr,
    right: Ptr,
    direction: Ptr,
) {
    copy_words(e, this.addr() + 0x28, left.addr(), 3);
    copy_words(e, this.addr() + 0x34, right.addr(), 3);
    copy_words(e, this.addr() + 0x40, direction.addr(), 3);
}

// Translated from 006d6f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Pathing::GetPotentialNavMeshInfoForLocation` (Xbox PDB): the navmesh
/// info of the cell the map knows (`NavMeshInfoMap::FindNavMeshInfoForCell`
/// of the map `TES::GetNavMeshInfoMap` gives for the global `TES`).
pub fn pathing_get_potential_nav_mesh_info_for_location(e: &mut Engine, cell: u32) -> u32 {
    let tes = e.global::<u32>(TES_GLOBAL);
    let map = e.call(TES_GET_NAVMESH_INFO_MAP, &args![tes]).u32();
    e.call(NAVMESH_INFO_MAP_FIND_FOR_CELL, &args![map, cell])
        .u32()
}

// Translated from 006d6f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Pathing::GetPotentialNavMeshInfoForLocation_ov2` (Xbox PDB): the same
/// through the map's lookup by two words (`006b7850`).
pub fn pathing_get_potential_nav_mesh_info_for_location_ov2(
    e: &mut Engine,
    first: u32,
    second: u32,
) -> u32 {
    let tes = e.global::<u32>(TES_GLOBAL);
    let map = e.call(TES_GET_NAVMESH_INFO_MAP, &args![tes]).u32();
    e.call(NAVMESH_INFO_MAP_FIND_BY_PAIR, &args![map, first, second])
        .u32()
}

// Translated from 006d6f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Pathing::FindClosestPointOnNavmesh` (Xbox PDB): the point of the
/// navmesh closest to `point` in a worldspace and cell. A location built
/// from the three that already has navmesh info keeps the point as it is.
/// Otherwise the location is resolved to the closest navmesh and triangle
/// and `out` is that triangle's center. Fails when it cannot resolve one.
/// The scope timer of line 0x906 covers the call.
pub fn pathing_find_closest_point_on_navmesh(
    e: &mut Engine,
    worldspace: u32,
    cell: u32,
    point: Ptr,
    out: Ptr,
) -> bool {
    with_frame(e, 0x60, |e, f| {
        let timer = f.at(-0x10);
        scope_enter(e, timer, 0x906);
        let location = f.at(-0x38);
        e.call(
            LOCATION_CONSTRUCT_FROM_PARTS,
            &args![location, point, cell, worldspace],
        );
        let finish = |e: &mut Engine, result: bool| {
            e.call(LOCATION_DESTRUCT, &args![location]);
            scope_leave(e, timer);
            result
        };
        if e.call(LOCATION_RESOLVE_NAVMESH_INFO, &args![location, 0u32])
            .bool()
            && e.call(LOCATION_HAS_NAVMESH_INFO, &args![location]).bool()
        {
            copy_words(e, out.addr(), point.addr(), 3);
            return finish(e, true);
        }
        if e.call(LOCATION_RESOLVE_CLOSEST, &args![location]).bool() {
            let holder = f.at(-0x40);
            e.call(NAV_HOLDER_CONSTRUCT, &args![holder]);
            let triangle = f.at(-0x3c);
            if e.call(
                LOCATION_GET_NAVMESH_AND_TRIANGLE,
                &args![location, holder, triangle],
            )
            .bool()
            {
                let navmesh = e.call(NI_POINTER_GET_FROM_FIELD, &args![holder]).u32();
                let center = e
                    .call(
                        NAVMESH_GET_CENTER,
                        &args![navmesh, f.at(-0x50), e.mem.u16(triangle) as u32],
                    )
                    .u32();
                copy_words(e, out.addr(), center, 3);
                e.call(NAV_HOLDER_RELEASE, &args![holder]);
                return finish(e, true);
            }
            e.call(NAV_HOLDER_RELEASE, &args![holder]);
        }
        finish(e, false)
    })
}

// Translated from 006d7110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Pathing::FindPointOnNavMesh` (Xbox PDB): a point of the navmesh within
/// `radius` of `point` in a worldspace and cell, stored in `out`.
///
/// When the location built from the three has no navmesh info or no
/// navmesh (`PathingLocation::GetNavMesh`), `out` is `point` plus a random
/// planar offset: `x` in `[-radius, radius]`, `y` in `[-s, s]` with
/// `s = sqrt(radius * radius - x * x)` (the square root wrapper
/// `004019b0`), and always succeeds. Otherwise the navmesh is asked
/// whether it has a point near `point` within `radius` (`00697c10`) and
/// whether that point is acceptable (`00697670`, with 0); when either
/// says no, the closest triangle for the point is used and `out` is its
/// center; if there is none the function fails.
pub fn pathing_find_point_on_nav_mesh(
    e: &mut Engine,
    worldspace: u32,
    cell: u32,
    point: Ptr,
    radius: f32,
    out: Ptr,
) -> bool {
    with_frame(e, 0x80, |e, f| {
        let location = f.at(-0x38);
        e.call(
            LOCATION_CONSTRUCT_FROM_PARTS,
            &args![location, point, cell, worldspace],
        );
        let mut has_navmesh = true;
        if !e
            .call(LOCATION_RESOLVE_NAVMESH_INFO, &args![location, 0u32])
            .bool()
        {
            has_navmesh = false;
        }
        let holder = f.at(-0x3c);
        e.call(NAV_HOLDER_CONSTRUCT, &args![holder]);
        if !e
            .call(LOCATION_GET_NAVMESH, &args![location, holder])
            .bool()
        {
            has_navmesh = false;
        }
        if !has_navmesh {
            let x = e.call(RANDOM_FLOAT, &args![-radius, radius]).f32();
            e.mem.set_f32(f.at(-0x40), x);
            let remaining = (radius as f64 * radius as f64 - x as f64 * x as f64) as f32;
            e.mem.set_f32(f.at(-0x7c), remaining);
            let limit = e.call(SQUARE_ROOT_WRAPPER, &args![remaining]).f32();
            e.mem.set_f32(f.at(-0x44), limit);
            let y = e.call(RANDOM_FLOAT, &args![-limit, limit]).f32();
            e.mem.set_f32(f.at(-0x44), y);
            let offset = e
                .call(POINT3_CONSTRUCT, &args![f.at(-0x58), x, y, 0.0f32])
                .u32();
            let moved = e.call(POINT3_ADD, &args![point, f.at(-0x64), offset]).u32();
            copy_words(e, out.addr(), moved, 3);
            e.call(NAV_HOLDER_RELEASE, &args![holder]);
            e.call(LOCATION_DESTRUCT, &args![location]);
            return true;
        }
        let navmesh = e.call(NI_POINTER_GET_FROM_FIELD, &args![holder]).u32();
        let near = e
            .call(NAVMESH_PROBE_A, &args![navmesh, point, radius, out])
            .bool();
        let mut settled = false;
        if near {
            let navmesh = e.call(NI_POINTER_GET_FROM_FIELD, &args![holder]).u32();
            settled = e.call(NAVMESH_PROBE_B, &args![navmesh, out, 0u32]).bool();
        }
        if !settled {
            let triangle = f.at(-0x48);
            e.mem.set_u16(triangle, 0xffff);
            let flag = f.at(-0x49);
            e.mem.set_u8(flag, 0);
            let navmesh = e.call(NI_POINTER_GET_FROM_FIELD, &args![holder]).u32();
            if !e
                .call(
                    NAVMESH_FIND_TRIANGLE,
                    &args![navmesh, point, triangle, flag],
                )
                .bool()
            {
                e.call(NAV_HOLDER_RELEASE, &args![holder]);
                e.call(LOCATION_DESTRUCT, &args![location]);
                return false;
            }
            let navmesh = e.call(NI_POINTER_GET_FROM_FIELD, &args![holder]).u32();
            let center = e
                .call(
                    NAVMESH_GET_CENTER,
                    &args![navmesh, f.at(-0x74), e.mem.u16(triangle) as u32],
                )
                .u32();
            copy_words(e, out.addr(), center, 3);
        }
        e.call(NAV_HOLDER_RELEASE, &args![holder]);
        e.call(LOCATION_DESTRUCT, &args![location]);
        true
    })
}

// Translated from 006d7350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Pathing::CheckLineOfSight` (Xbox PDB): whether there is a line of sight
/// from the location `from` to the location `to`. A non-null `result` first
/// receives a copy of `from` (it is updated by the walk). When `to` has no
/// navmesh info the answer is true only if `from` has none either.
/// Otherwise `fn_006d74c0` walks from `from` towards the position of `to`,
/// given `to`'s navmesh and triangle when `GetNavMeshAndTriangle` finds
/// them (else no navmesh and triangle 0xffff).
pub fn pathing_check_line_of_sight(
    e: &mut Engine,
    from: Ptr,
    to: Ptr,
    result: Ptr,
    filter: Ptr,
) -> bool {
    with_frame(e, 0x40, |e, f| {
        if result.addr() != 0 {
            e.call(LOCATION_ASSIGN, &args![result, from]);
        }
        if !e
            .call(LOCATION_RESOLVE_NAVMESH_INFO, &args![to, 0u32])
            .bool()
        {
            return !e
                .call(LOCATION_RESOLVE_NAVMESH_INFO, &args![from, 0u32])
                .bool();
        }
        let holder = f.at(-0x14);
        e.call(NAV_HOLDER_CONSTRUCT, &args![holder]);
        let triangle = f.at(-0x10);
        let found = if !e
            .call(
                LOCATION_GET_NAVMESH_AND_TRIANGLE,
                &args![to, holder, triangle],
            )
            .bool()
        {
            let position = e
                .call(LOCATION_COPY_POSITION, &args![to, f.at(-0x24)])
                .u32();
            fn_006d74c0(e, from, Ptr::new(position), 0, 0xffff, result, filter)
        } else {
            let navmesh = e.call(NI_POINTER_GET_FROM_FIELD, &args![holder]).u32();
            let position = e
                .call(LOCATION_COPY_POSITION, &args![to, f.at(-0x34)])
                .u32();
            fn_006d74c0(
                e,
                from,
                Ptr::new(position),
                navmesh,
                e.mem.u16(triangle) as u32,
                result,
                filter,
            )
        };
        e.call(NAV_HOLDER_RELEASE, &args![holder]);
        found
    })
}

// Translated from 006d7490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Pathing::CheckLineOfSight_ov2` (Xbox PDB): `fn_006d74c0` with no
/// target navmesh and triangle 0xffff.
pub fn pathing_check_line_of_sight_ov2(
    e: &mut Engine,
    from: Ptr,
    to: Ptr,
    result: Ptr,
    filter: Ptr,
) -> bool {
    fn_006d74c0(e, from, to, 0, 0xffff, result, filter)
}

// Translated from 006d74c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Pathing::CheckLineOfSight_ov3` (Xbox PDB): walks the navmesh triangles
/// from the location `start` along the segment to the position `end`.
///
/// A non-null `result` first receives a copy of `start`; it ends up at the
/// last place reached. Without navmesh info for `start` the answer is
/// whether `target_navmesh` is null. The walk keeps a 2D hit point and the
/// current triangle: `00698060` gives the edge of the current triangle the
/// segment leaves through (-1 when the end lies inside); `0068f3b0` finds
/// the triangle across it (the navmesh may change; `fn_006d7a20` tells,
/// and then the hit point is recomputed on the new edge with `006ba3d0`);
/// the optional `filter` object (its virtual slot 0, called with copies of
/// the two edge references) may refuse the crossing, as may an edge whose
/// extra info is 1. A refused crossing fails and puts `result` at the
/// current triangle, with the hit point and the triangle center's height.
/// When the end is reached: without a target the answer is true and
/// `result` is the end position in the current triangle; with a target
/// navmesh and triangle the walk must have ended in exactly that one.
pub fn fn_006d74c0(
    e: &mut Engine,
    start: Ptr,
    end: Ptr,
    target_navmesh: u32,
    target_triangle: u32,
    result: Ptr,
    filter: Ptr,
) -> bool {
    with_frame(e, 0x120, |e, f| {
        if result.addr() != 0 {
            e.call(LOCATION_ASSIGN, &args![result, start]);
        }
        if !e
            .call(LOCATION_RESOLVE_NAVMESH_INFO, &args![start, 0u32])
            .bool()
        {
            return target_navmesh == 0;
        }
        let start_holder = f.at(-0x38);
        e.call(NAV_HOLDER_CONSTRUCT, &args![start_holder]);
        let start_triangle = f.at(-0x14);
        if !e
            .call(
                LOCATION_GET_NAVMESH_AND_TRIANGLE,
                &args![start, start_holder, start_triangle],
            )
            .bool()
        {
            e.call(NAV_HOLDER_RELEASE, &args![start_holder]);
            return false;
        }
        let position = e
            .call(LOCATION_COPY_POSITION, &args![start, f.at(-0x94)])
            .u32();
        let start_y = e.mem.f32(position + 4);
        let position = e
            .call(LOCATION_COPY_POSITION, &args![start, f.at(-0xa0)])
            .u32();
        let start_x = e.mem.f32(position);
        let start_2d = f.at(-0x30);
        e.call(POINT2_CONSTRUCT, &args![start_2d, start_x, start_y]);
        let end_y = e.mem.f32(end.addr() + 4);
        let end_x = e.mem.f32(end.addr());
        let end_2d = f.at(-0x28);
        e.call(POINT2_CONSTRUCT, &args![end_2d, end_x, end_y]);
        let hit = f.at(-0x1c);
        e.call(POINT3_DEFAULT_CONSTRUCTOR, &args![hit]);
        let holder = f.at(-0x10);
        e.call(NAV_HOLDER_COPY_CONSTRUCT, &args![holder, start_holder]);
        let current_triangle = f.at(-0x20);
        let triangle = e.mem.u16(start_triangle);
        e.mem.set_u16(current_triangle, triangle);
        let navmesh = e.call(NI_POINTER_GET_FROM_FIELD, &args![holder]).u32();
        let mut edge = e
            .call(
                NAVMESH_CROSSED_EDGE,
                &args![
                    navmesh,
                    triangle as u32,
                    0xffff_ffffu32,
                    start_2d,
                    end_2d,
                    hit
                ],
            )
            .u32();
        let next_holder = f.at(-0x48);
        let next_triangle = f.at(-0x40);
        let next_edge = f.at(-0x44);
        while edge != 0xffff_ffff {
            e.call(NAV_HOLDER_CONSTRUCT, &args![next_holder]);
            let triangle = e.mem.u16(current_triangle) as u32;
            let navmesh = e.call(NI_POINTER_GET_FROM_FIELD, &args![holder]).u32();
            let mut crossed = e
                .call(
                    NAVMESH_GET_MATCHING_EDGE_B,
                    &args![
                        navmesh,
                        triangle,
                        edge,
                        next_holder,
                        next_triangle,
                        next_edge
                    ],
                )
                .bool();
            if crossed && filter.addr() != 0 {
                let triangle = e.mem.u16(current_triangle) as u32;
                let navmesh = e.call(NI_POINTER_GET_FROM_FIELD, &args![holder]).u32();
                let from_edge = f.at(-0x54);
                e.call(
                    EDGE_REFERENCE_CONSTRUCT,
                    &args![from_edge, navmesh, triangle, edge],
                );
                let navmesh = e.call(NI_POINTER_GET_FROM_FIELD, &args![next_holder]).u32();
                let to_edge = f.at(-0x60);
                let next_triangle_value = e.mem.u16(next_triangle) as u32;
                let next_edge_value = e.mem.u32(next_edge);
                e.call(
                    EDGE_REFERENCE_CONSTRUCT,
                    &args![to_edge, navmesh, next_triangle_value, next_edge_value],
                );
                // The filter takes both references by value: copies of
                // the references, the one for the source edge on top.
                let to_copy = f.at(-0x110);
                let from_copy = f.at(-0x11c);
                e.call(EDGE_REFERENCE_COPY, &args![to_copy, to_edge]);
                e.call(EDGE_REFERENCE_COPY, &args![from_copy, from_edge]);
                let mut words = Vec::new();
                for i in 0..3 {
                    words.push(e.mem.u32(from_copy + 4 * i));
                }
                for i in 0..3 {
                    words.push(e.mem.u32(to_copy + 4 * i));
                }
                crossed = e.vcall(filter.addr(), 0, &words).bool();
            }
            if crossed {
                let triangle = e.mem.u16(current_triangle) as u32;
                let navmesh = e.call(NI_POINTER_GET_FROM_FIELD, &args![holder]).u32();
                let info = e
                    .call(NAVMESH_EDGE_EXTRA_INFO, &args![navmesh, triangle, edge])
                    .u32();
                if info != 0 && e.mem.u32(info) == 1 {
                    crossed = false;
                }
            }
            if !crossed {
                if result.addr() != 0 {
                    let navmesh = e.call(NI_POINTER_GET_FROM_FIELD, &args![holder]).u32();
                    let info = e.call(NAVMESH_GET_INFO, &args![navmesh]).u32();
                    e.call(LOCATION_SET_NAVMESH_INFO, &args![result, info]);
                    let triangle = e.mem.u16(current_triangle) as u32;
                    e.call(LOCATION_SET_TRIANGLE, &args![result, triangle]);
                    let triangle = e.mem.u16(current_triangle) as u32;
                    let navmesh = e.call(NI_POINTER_GET_FROM_FIELD, &args![holder]).u32();
                    let center = e
                        .call(NAVMESH_GET_CENTER, &args![navmesh, f.at(-0xc8), triangle])
                        .u32();
                    let center_height = e.mem.f32(center + 8);
                    let hit_y = e.mem.f32(hit + 4);
                    let hit_x = e.mem.f32(hit);
                    let place = e
                        .call(
                            POINT3_CONSTRUCT,
                            &args![f.at(-0xbc), hit_x, hit_y, center_height],
                        )
                        .u32();
                    e.call(LOCATION_SET_POSITION, &args![result, place]);
                }
                e.call(NAV_HOLDER_RELEASE, &args![next_holder]);
                e.call(NAV_HOLDER_RELEASE, &args![holder]);
                e.call(NAV_HOLDER_RELEASE, &args![start_holder]);
                return false;
            }
            if fn_006d7a20(e, Ptr::new(next_holder), Ptr::new(holder)) {
                let triangle = e.mem.u16(next_triangle) as u32;
                let edge_value = e.mem.u32(next_edge);
                let navmesh = e.call(NI_POINTER_GET_FROM_FIELD, &args![next_holder]).u32();
                let vertices = f.at(-0x84);
                e.call(
                    NAVMESH_EDGE_VERTICES,
                    &args![navmesh, vertices, triangle, edge_value],
                );
                let first = e.mem.u32(vertices);
                let first_y = e.mem.f32(first + 4);
                let first_x = e.mem.f32(first);
                e.call(POINT2_CONSTRUCT, &args![f.at(-0x7c), first_x, first_y]);
                let second = e.mem.u32(vertices + 4);
                let second_y = e.mem.f32(second + 4);
                let second_x = e.mem.f32(second);
                e.call(POINT2_CONSTRUCT, &args![f.at(-0x74), second_x, second_y]);
                let closest = e
                    .call(
                        POINT2_SEGMENT_CLOSEST,
                        &args![f.at(-0xb0), f.at(-0x7c), f.at(-0x74), hit, end_2d],
                    )
                    .u32();
                let closest_x = e.mem.u32(closest);
                let closest_y = e.mem.u32(closest + 4);
                e.mem.set_u32(hit, closest_x);
                e.mem.set_u32(hit + 4, closest_y);
            }
            let next_triangle_value = e.mem.u16(next_triangle);
            e.mem.set_u16(current_triangle, next_triangle_value);
            e.call(NAV_HOLDER_ASSIGN, &args![holder, next_holder]);
            let new_hit = f.at(-0x6c);
            e.call(POINT3_DEFAULT_CONSTRUCTOR, &args![new_hit]);
            let triangle = e.mem.u16(next_triangle) as u32;
            let edge_value = e.mem.u32(next_edge);
            let navmesh = e.call(NI_POINTER_GET_FROM_FIELD, &args![next_holder]).u32();
            edge = e
                .call(
                    NAVMESH_CROSSED_EDGE,
                    &args![navmesh, triangle, edge_value, hit, end_2d, new_hit],
                )
                .u32();
            let new_x = e.mem.u32(new_hit);
            let new_y = e.mem.u32(new_hit + 4);
            e.mem.set_u32(hit, new_x);
            e.mem.set_u32(hit + 4, new_y);
            e.call(NAV_HOLDER_RELEASE, &args![next_holder]);
        }

        let reached = if target_navmesh == 0 || (target_triangle & 0xffff) == 0xffff {
            if result.addr() != 0 {
                let navmesh = e.call(NI_POINTER_GET_FROM_FIELD, &args![holder]).u32();
                let info = e.call(NAVMESH_GET_INFO, &args![navmesh]).u32();
                e.call(LOCATION_SET_NAVMESH_INFO, &args![result, info]);
                let triangle = e.mem.u16(current_triangle) as u32;
                e.call(LOCATION_SET_TRIANGLE, &args![result, triangle]);
                e.call(LOCATION_SET_POSITION, &args![result, end]);
            }
            true
        } else if e.call(NI_POINTER_GET_FROM_FIELD, &args![holder]).u32() == target_navmesh
            && e.mem.u16(current_triangle) as u32 == (target_triangle & 0xffff)
        {
            if result.addr() != 0 {
                let info = e.call(NAVMESH_GET_INFO, &args![target_navmesh]).u32();
                let place = f.at(-0xf4);
                let built = e
                    .call(
                        LOCATION_CONSTRUCT_FROM_MESH_POINT,
                        &args![place, end, info, target_triangle & 0xffff],
                    )
                    .u32();
                e.call(LOCATION_ASSIGN, &args![result, built]);
                e.call(LOCATION_DESTRUCT, &args![place]);
            }
            true
        } else {
            false
        };
        e.call(NAV_HOLDER_RELEASE, &args![holder]);
        e.call(NAV_HOLDER_RELEASE, &args![start_holder]);
        reached
    })
}

// Translated from 006d7a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the first words of two `NiPointer` holders differ: `00631820`
/// with `other`.
pub fn fn_006d7a20(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(HOLDER_DIFFERS, &args![this, other]).bool()
}

// Translated from 006d7a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Pathing::FindClosestReachableLocation` (Xbox PDB): resolves the
/// navmesh triangles of `from` and `to` (a null triangle reference for `to`
/// without navmesh info), asks a max-cost search for the point reachable
/// from `from` that comes closest to `to` (`limit` bounds the cost) and
/// stores a copy of `from` moved to that point in `out`.
///
/// Without navmesh info for `from`, or without its triangle, `out` is a copy
/// of `from`. Returns `out`.
pub fn pathing_find_closest_reachable_location(
    e: &mut Engine,
    out: Ptr,
    from: Ptr,
    to: Ptr,
    limit: f32,
) -> Ptr {
    with_frame(e, 0x2150, |e, f| {
        if !e
            .call(LOCATION_RESOLVE_NAVMESH_INFO, &args![from, 0u32])
            .bool()
        {
            e.call(LOCATION_COPY_CONSTRUCT, &args![out, from]);
            return out;
        }
        let from_holder = f.at(-0x2124);
        e.call(NAV_HOLDER_CONSTRUCT, &args![from_holder]);
        let from_triangle = f.at(-0x2120);
        if !e
            .call(
                LOCATION_GET_NAVMESH_AND_TRIANGLE,
                &args![from, from_holder, from_triangle],
            )
            .bool()
        {
            e.call(LOCATION_COPY_CONSTRUCT, &args![out, from]);
            e.call(NAV_HOLDER_RELEASE, &args![from_holder]);
            return out;
        }
        // The start reference: the navmesh pointer and the triangle.
        let start = f.at(-0x20e0);
        let navmesh = held(e, from_holder);
        e.mem.set_u32(start, navmesh);
        let triangle = e.mem.u16(from_triangle);
        e.mem.set_u16(start + 4, triangle);
        e.call(NAV_HOLDER_RELEASE, &args![from_holder]);
        // The goal reference: empty unless `to` resolves.
        let goal = f.at(-0x20f4);
        e.mem.set_u32(goal, 0);
        e.mem.set_u16(goal + 4, 0xffff);
        if e.call(LOCATION_RESOLVE_NAVMESH_INFO, &args![to, 0u32])
            .bool()
        {
            let to_holder = f.at(-0x212c);
            e.call(NAV_HOLDER_CONSTRUCT, &args![to_holder]);
            let to_triangle = f.at(-0x2128);
            if e.call(
                LOCATION_GET_NAVMESH_AND_TRIANGLE,
                &args![to, to_holder, to_triangle],
            )
            .bool()
            {
                let navmesh = held(e, to_holder);
                e.mem.set_u32(goal, navmesh);
                let triangle = e.mem.u16(to_triangle);
                e.mem.set_u16(goal + 4, triangle);
            }
            e.call(NAV_HOLDER_RELEASE, &args![to_holder]);
        }
        let search = f.at(-0x20cc);
        e.call(MAX_COST_SEARCH_CONSTRUCT, &args![search]);
        let to_position = e
            .call(LOCATION_COPY_POSITION, &args![to, f.at(-0x2138)])
            .u32();
        let from_position = e
            .call(LOCATION_COPY_POSITION, &args![from, f.at(-0x2144)])
            .u32();
        let found = f.at(-0x18);
        e.call(
            MAX_COST_SEARCH_FIND_POINT,
            &args![
                search,
                found,
                from_position,
                start,
                to_position,
                goal,
                limit
            ],
        );
        let moved = f.at(-0x211c);
        e.call(LOCATION_COPY_CONSTRUCT, &args![moved, from]);
        e.call(LOCATION_SET_POSITION, &args![moved, found]);
        e.call(LOCATION_COPY_CONSTRUCT, &args![out, moved]);
        e.call(LOCATION_DESTRUCT, &args![moved]);
        e.call(MAX_COST_SEARCH_DESTRUCT, &args![search]);
        out
    })
}

// Translated from 006d7ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Refreshes a `PathingLOSGridMap` (`grid_map_slot` is the address of the
/// `NiPointer` that holds it) for a set of locations: builds a
/// `PathingLOSMap` around the grid map's center and radius, lets the grid
/// map fill it, scores every location of `locations` (a `BSSimpleArray` of
/// 0x28-byte `PathingLocation`s, each with the matching `float` of
/// `headings` when that array is given, else -1.0) through
/// `fn_006d7f40` with the grid map's sight radius, counts the scored
/// triangles that are seen (positive score) and unseen (not 0x7f, not
/// positive) into the LOS map and merges it back into the grid map.
///
/// Returns false, doing nothing, when the grid map has no radius
/// (`fn_006d7f00`).
pub fn fn_006d7ca0(e: &mut Engine, grid_map_slot: Ptr, locations: Ptr, headings: Ptr) -> bool {
    with_frame(e, 0x140, |e, f| {
        let slot = grid_map_slot.addr();
        let grid_map = held(e, slot);
        if !fn_006d7f00(e, Ptr::new(grid_map)) {
            return false;
        }
        let memory = e.call(OBJECT_ALLOCATE, &args![0x50u32]).u32();
        let los_map = if memory != 0 {
            let grid_map = held(e, slot);
            let radius = e.call(LOS_GRID_MAP_RADIUS, &args![grid_map]).f32();
            let grid_map = held(e, slot);
            let center = e
                .call(LOS_GRID_MAP_COPY_CENTER, &args![grid_map, f.at(-0x44)])
                .u32();
            let (x, y, z) = (
                e.mem.u32(center),
                e.mem.u32(center + 4),
                e.mem.u32(center + 8),
            );
            e.call(LOS_MAP_CONSTRUCT, &args![memory, x, y, z, radius])
                .u32()
        } else {
            0
        };
        let holder = f.at(-0x1c);
        e.call(NI_POINTER_CONSTRUCT_FROM, &args![holder, los_map]);
        let grid_map = held(e, slot);
        e.call(LOS_GRID_MAP_FILL_LOS_MAP, &args![grid_map, holder]);
        let count = e.call(ARRAY_COUNT, &args![locations]).i32();
        for index in 0..count {
            let location = e
                .call(LOCATION_ARRAY_ELEMENT, &args![locations, index as u32])
                .u32();
            let heading = if headings.addr() != 0 {
                let element = e
                    .call(POINTER_ARRAY_ELEMENT_THUNK, &args![headings, index as u32])
                    .u32();
                e.mem.f32(element)
            } else {
                e.global::<f32>(NO_HEADING)
            };
            let grid_map = held(e, slot);
            let sight_radius = e.call(FLOAT_AT_OFFSET_0X48, &args![grid_map]).f32();
            fn_006d7f40(
                e,
                Ptr::new(location),
                Ptr::new(holder),
                true,
                sight_radius,
                heading,
            );
        }
        let (mut seen, mut unseen) = (0u32, 0u32);
        let mut triangle = 0u32;
        loop {
            let los_map = held(e, holder);
            if triangle >= e.call(LOS_MAP_TRIANGLE_COUNT, &args![los_map]).u32() {
                break;
            }
            let los_map = held(e, holder);
            let score = fn_006d7ee0(e, Ptr::new(los_map), triangle) as i8;
            if score != 0x7f {
                if score > 0 {
                    seen += 1;
                } else {
                    unseen += 1;
                }
            }
            triangle += 1;
        }
        let los_map = held(e, holder);
        e.call(LOS_MAP_SET_SEEN_COUNT, &args![los_map, seen]);
        let los_map = held(e, holder);
        e.call(LOS_MAP_SET_UNSEEN_COUNT, &args![los_map, unseen]);
        let grid_map = held(e, slot);
        e.call(LOS_GRID_MAP_MERGE_LOS_MAP, &args![grid_map, holder]);
        e.call(NI_POINTER_DESTRUCT, &args![holder]);
        true
    })
}

// Translated from 006d7ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The score of triangle `index` of a `PathingLOSMap`: the byte
/// `pTriangleScores[index]` (+0x18). 0x7f marks an unscored triangle.
pub fn fn_006d7ee0(e: &mut Engine, this: Ptr, index: u32) -> u8 {
    let scores = e.mem.u32(this.addr() + 0x18);
    e.mem.u8(scores.wrapping_add(index))
}

// Translated from 006d7f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the `fRadius` (+0x44) of a `PathingLOSGridMap` is above zero.
pub fn fn_006d7f00(e: &mut Engine, this: Ptr) -> bool {
    let radius = e.mem.f32(this.addr() + 0x44) as f64;
    radius > e.global::<f64>(ZERO_DOUBLE)
}

// Translated from 006d7f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scores the triangles of a `PathingLOSMap` (`los_map_slot` is the address
/// of the `NiPointer` that holds it) as seen from `location`.
///
/// For each navmesh of the LOS map the function visits the triangles whose
/// score is still 0x7f and that do not carry flag 0x20. A triangle is
/// skipped (kept unscored) when the map has a radius and the triangle
/// center is farther than that radius from the map's center (the height
/// difference counted three times), and is scored as unseen when it is
/// farther than `sight_distance` (if positive) from the location, or lies
/// behind the `heading` (if not negative; the dot product of the heading
/// direction `(sin, cos, 0)` with the vector to the triangle center is
/// negative). Otherwise it walks the navmesh triangles along the segment
/// from the triangle center to the location (`00698060`, `0068f460`,
/// crossing into other navmeshes of the map), collecting the visited
/// triangles with their open-edge counts (`fn_006d8910`). When the walk
/// reaches the location's own triangle the visited triangles get those
/// counts as (positive) scores and widen the map's `iMaxScore`; when it
/// is blocked (no matching edge, an edge whose extra info is 1, or a
/// navmesh outside the map) they get the counts negated as scores of
/// unseen triangles and widen `iMinScore`. Triangles that already have a
/// score keep it. `reset` first sets every non-positive score back to
/// 0x7f.
///
/// Returns false when the map has no navmesh or the location has no
/// triangle. The two step counters the game keeps in `ebp-0x20` and
/// `ebp-0x2c` are written but never read, and are left out.
pub fn fn_006d7f40(
    e: &mut Engine,
    location: Ptr,
    los_map_slot: Ptr,
    reset: bool,
    sight_distance: f32,
    heading: f32,
) -> bool {
    with_frame(e, 0x140, |e, f| {
        let slot = los_map_slot.addr();
        let los_map = held(e, slot);
        if fn_006d8990(e, Ptr::new(los_map)) == 0 {
            return false;
        }
        if !e
            .call(LOCATION_RESOLVE_NAVMESH_INFO, &args![location, 0u32])
            .bool()
        {
            return false;
        }
        let start_holder = f.at(-0x1c);
        e.call(NAV_HOLDER_CONSTRUCT, &args![start_holder]);
        let start_triangle = f.at(-0x3c);
        e.mem.set_u16(start_triangle, 0xffff);
        if !e
            .call(
                LOCATION_GET_NAVMESH_AND_TRIANGLE,
                &args![location, start_holder, start_triangle],
            )
            .bool()
        {
            e.call(NAV_HOLDER_RELEASE, &args![start_holder]);
            return false;
        }
        if reset {
            let los_map = held(e, slot);
            e.call(LOS_MAP_RESET_SCORES, &args![los_map]);
        }
        let location_position = f.at(-0x18);
        e.call(LOCATION_COPY_POSITION, &args![location, location_position]);
        let los_map = held(e, slot);
        let center = f.at(-0x38);
        e.call(LOS_MAP_COPY_CENTER, &args![los_map, center]);
        let los_map = held(e, slot);
        let radius = e.call(FLOAT_AT_OFFSET_0X48, &args![los_map]).f32();
        let radius_squared = (radius as f64 * radius as f64) as f32;
        let direction = f.at(-0x48);
        copy_words(e, direction, ZERO_VECTOR, 3);
        if heading >= 0.0 {
            e.call(SINE_AND_COSINE, &args![heading, direction, direction + 4]);
        }

        let mut navmesh_index = 0u32;
        loop {
            let los_map = held(e, slot);
            if navmesh_index >= fn_006d8990(e, Ptr::new(los_map)) {
                break;
            }
            let los_map = held(e, slot);
            let element = fn_006d89b0(e, Ptr::new(los_map), navmesh_index);
            let holder = f.at(-0x54);
            e.call(NAV_HOLDER_COPY_CONSTRUCT, &args![holder, element]);
            let offset = f.at(-0x50);
            e.mem.set_u32(offset, 0);
            let navmesh = held(e, holder);
            let form_id = e.call(WORD_AT_OFFSET_0XC, &args![navmesh]).u32();
            let los_map = held(e, slot);
            if !e
                .call(LOS_MAP_FIND_OFFSET, &args![los_map, form_id, offset])
                .bool()
            {
                e.call(NAV_HOLDER_RELEASE, &args![holder]);
                navmesh_index += 1;
                continue;
            }
            let mut triangle = 0u32;
            'triangles: loop {
                let navmesh = held(e, holder);
                if triangle >= e.call(NAVMESH_TRIANGLE_COUNT, &args![navmesh]).u32() {
                    break;
                }
                // The next triangle that is unscored and not flagged 0x20.
                loop {
                    let index = triangle.wrapping_add(e.mem.u32(offset));
                    let los_map = held(e, slot);
                    let score = fn_006d7ee0(e, Ptr::new(los_map), index) as i8;
                    if score == 0x7f {
                        let navmesh = held(e, holder);
                        let record = e
                            .call(NAVMESH_GET_TRIANGLE, &args![navmesh, triangle & 0xffff])
                            .u32();
                        if !e.call(TRIANGLE_HAS_FLAG, &args![record, 0x20u32]).bool() {
                            break;
                        }
                    }
                    triangle += 1;
                    let navmesh = held(e, holder);
                    if triangle >= e.call(NAVMESH_TRIANGLE_COUNT, &args![navmesh]).u32() {
                        break;
                    }
                }
                let navmesh = held(e, holder);
                if triangle >= e.call(NAVMESH_TRIANGLE_COUNT, &args![navmesh]).u32() {
                    break;
                }
                let triangle_center = f.at(-0x64);
                let navmesh = held(e, holder);
                e.call(
                    NAVMESH_GET_CENTER,
                    &args![navmesh, triangle_center, triangle & 0xffff],
                );
                if radius != 0.0 {
                    let difference = f.at(-0xac);
                    e.call(POINT3_SUBTRACT, &args![triangle_center, difference, center]);
                    let height = e.mem.f32(difference + 8);
                    let weight = e.global::<f64>(LOS_HEIGHT_WEIGHT);
                    e.mem
                        .set_f32(difference + 8, (height as f64 * weight) as f32);
                    let distance_squared = e.call(POINT3_LENGTH_SQUARED, &args![difference]).f64();
                    if (radius_squared as f64) < distance_squared {
                        triangle += 1;
                        continue 'triangles;
                    }
                }
                if sight_distance > 0.0 || heading >= 0.0 {
                    let mut blocked = false;
                    let own = e
                        .call(LOCATION_COPY_POSITION, &args![location, f.at(-0x110)])
                        .u32();
                    let to_location = f.at(-0xb8);
                    e.call(POINT3_SUBTRACT, &args![triangle_center, to_location, own]);
                    if sight_distance > 0.0 {
                        let distance_squared =
                            e.call(POINT3_LENGTH_SQUARED, &args![to_location]).f64();
                        if (sight_distance as f64 * sight_distance as f64) < distance_squared {
                            blocked = true;
                        }
                    }
                    if !blocked && heading >= 0.0 {
                        let dot = e.call(POINT3_DOT, &args![direction, to_location]).f64();
                        if dot < 0.0 {
                            blocked = true;
                        }
                    }
                    if blocked {
                        let index = triangle.wrapping_add(e.mem.u32(offset));
                        let los_map = held(e, slot);
                        if fn_006d7ee0(e, Ptr::new(los_map), index) as i8 == 0x7f {
                            let navmesh = held(e, holder);
                            let edges = fn_006d8910(e, Ptr::new(navmesh), triangle as u16) as i8;
                            let value = (-(edges as i32)) as u32;
                            let index = triangle.wrapping_add(e.mem.u32(offset));
                            let los_map = held(e, slot);
                            fn_006d8970(e, Ptr::new(los_map), index, value as u8);
                        }
                        triangle += 1;
                        continue 'triangles;
                    }
                }

                // Walk from the triangle center to the location.
                let center_2d = f.at(-0x9c);
                let (center_x, center_y) =
                    (e.mem.f32(triangle_center), e.mem.f32(triangle_center + 4));
                e.call(POINT2_CONSTRUCT, &args![center_2d, center_x, center_y]);
                let location_2d = f.at(-0x94);
                let (location_x, location_y) = (
                    e.mem.f32(location_position),
                    e.mem.f32(location_position + 4),
                );
                e.call(
                    POINT2_CONSTRUCT,
                    &args![location_2d, location_x, location_y],
                );
                let hit = f.at(-0x88);
                e.call(POINT3_DEFAULT_CONSTRUCTOR, &args![hit]);
                let mut current_triangle = triangle as u16;
                let mut current_navmesh = held(e, holder);
                let current_offset = f.at(-0xa0);
                let first_offset = e.mem.u32(offset);
                e.mem.set_u32(current_offset, first_offset);
                let mut found = false;
                let entries = f.at(-0x7c);
                e.call(
                    SCORE_ENTRY_ARRAY_CONSTRUCT,
                    &args![entries, 0x40u32, 0u32, 0u32],
                );
                let edges = fn_006d8910(e, Ptr::new(current_navmesh), current_triangle);
                let index = (current_triangle as u32).wrapping_add(e.mem.u32(current_offset));
                let entry = fn_006d89d0(e, Ptr::new(f.at(-0x118)), index, edges);
                e.call(SCORE_ENTRY_ARRAY_ADD, &args![entries, entry]);
                if current_triangle == e.mem.u16(start_triangle)
                    && current_navmesh == held(e, start_holder)
                {
                    found = true;
                } else {
                    let mut edge = e
                        .call(
                            NAVMESH_CROSSED_EDGE,
                            &args![
                                current_navmesh,
                                current_triangle as u32,
                                0xffff_ffffu32,
                                center_2d,
                                location_2d,
                                hit
                            ],
                        )
                        .u32();
                    let next_navmesh_slot = f.at(-0xcc);
                    let next_triangle_slot = f.at(-0xc4);
                    let next_edge_slot = f.at(-0xc8);
                    while edge != 0xffff_ffff {
                        if !e
                            .call(
                                NAVMESH_GET_MATCHING_EDGE_C,
                                &args![
                                    current_navmesh,
                                    current_triangle as u32,
                                    edge,
                                    next_navmesh_slot,
                                    next_triangle_slot,
                                    next_edge_slot
                                ],
                            )
                            .bool()
                        {
                            break;
                        }
                        let info = e
                            .call(
                                NAVMESH_EDGE_EXTRA_INFO,
                                &args![current_navmesh, current_triangle as u32, edge],
                            )
                            .u32();
                        if info != 0 && e.mem.u32(info) == 1 {
                            break;
                        }
                        let next_navmesh = e.mem.u32(next_navmesh_slot);
                        if next_navmesh != current_navmesh {
                            let form_id = e.call(WORD_AT_OFFSET_0XC, &args![next_navmesh]).u32();
                            let los_map = held(e, slot);
                            if !e
                                .call(
                                    LOS_MAP_FIND_OFFSET,
                                    &args![los_map, form_id, current_offset],
                                )
                                .bool()
                            {
                                break;
                            }
                            let vertices = f.at(-0xf0);
                            let next_triangle = e.mem.u16(next_triangle_slot) as u32;
                            let next_edge = e.mem.u32(next_edge_slot);
                            e.call(
                                NAVMESH_EDGE_VERTICES,
                                &args![next_navmesh, vertices, next_triangle, next_edge],
                            );
                            let (first, second) = (e.mem.u32(vertices), e.mem.u32(vertices + 4));
                            let (x, y) = (e.mem.f32(first), e.mem.f32(first + 4));
                            let edge_start = f.at(-0xe8);
                            e.call(POINT2_CONSTRUCT, &args![edge_start, x, y]);
                            let (x, y) = (e.mem.f32(second), e.mem.f32(second + 4));
                            let edge_end = f.at(-0xe0);
                            e.call(POINT2_CONSTRUCT, &args![edge_end, x, y]);
                            let closest = e
                                .call(
                                    POINT2_SEGMENT_CLOSEST,
                                    &args![f.at(-0x120), edge_start, edge_end, hit, location_2d],
                                )
                                .u32();
                            let (x, y) = (e.mem.u32(closest), e.mem.u32(closest + 4));
                            e.mem.set_u32(hit, x);
                            e.mem.set_u32(hit + 4, y);
                        }
                        current_navmesh = next_navmesh;
                        current_triangle = e.mem.u16(next_triangle_slot);
                        let edges = fn_006d8910(e, Ptr::new(current_navmesh), current_triangle);
                        let index =
                            (current_triangle as u32).wrapping_add(e.mem.u32(current_offset));
                        let entry = fn_006d89d0(e, Ptr::new(f.at(-0x128)), index, edges);
                        e.call(SCORE_ENTRY_ARRAY_ADD, &args![entries, entry]);
                        let new_hit = f.at(-0xd4);
                        e.call(POINT3_DEFAULT_CONSTRUCTOR, &args![new_hit]);
                        let entered_edge = e.mem.u32(next_edge_slot);
                        edge = e
                            .call(
                                NAVMESH_CROSSED_EDGE,
                                &args![
                                    current_navmesh,
                                    current_triangle as u32,
                                    entered_edge,
                                    hit,
                                    location_2d,
                                    new_hit
                                ],
                            )
                            .u32();
                        let (x, y) = (e.mem.u32(new_hit), e.mem.u32(new_hit + 4));
                        e.mem.set_u32(hit, x);
                        e.mem.set_u32(hit + 4, y);
                        if current_triangle == e.mem.u16(start_triangle)
                            && current_navmesh == held(e, start_holder)
                        {
                            found = true;
                            break;
                        }
                    }
                }

                if found {
                    let mut k = 0u32;
                    while k < e.call(ARRAY_COUNT, &args![entries]).u32() {
                        let element = e.call(SCORE_ENTRY_ARRAY_ELEMENT, &args![entries, k]).u32();
                        let score = e.mem.u8(element + 4);
                        let element = e.call(SCORE_ENTRY_ARRAY_ELEMENT, &args![entries, k]).u32();
                        let index = e.mem.u32(element);
                        let los_map = held(e, slot);
                        fn_006d8970(e, Ptr::new(los_map), index, score);
                        k += 1;
                    }
                    let mut k = 0u32;
                    while k < e.call(ARRAY_COUNT, &args![entries]).u32() {
                        let los_map = held(e, slot);
                        if e.call(LOS_MAP_MAX_SCORE, &args![los_map]).u8() as i8 == 3 {
                            break;
                        }
                        let element = e.call(SCORE_ENTRY_ARRAY_ELEMENT, &args![entries, k]).u32();
                        let score = e.mem.i8(element + 4) as i32;
                        let los_map = held(e, slot);
                        let maximum = e.call(LOS_MAP_MAX_SCORE, &args![los_map]).u8() as i8 as i32;
                        if score > maximum {
                            let element =
                                e.call(SCORE_ENTRY_ARRAY_ELEMENT, &args![entries, k]).u32();
                            let score = e.mem.u8(element + 4) as u32;
                            let los_map = held(e, slot);
                            e.call(LOS_MAP_SET_MAX_SCORE, &args![los_map, score]);
                        }
                        k += 1;
                    }
                } else {
                    let mut k = 0u32;
                    while k < e.call(ARRAY_COUNT, &args![entries]).u32() {
                        let element = e.call(SCORE_ENTRY_ARRAY_ELEMENT, &args![entries, k]).u32();
                        let index = e.mem.u32(element);
                        let los_map = held(e, slot);
                        if fn_006d7ee0(e, Ptr::new(los_map), index) as i8 == 0x7f {
                            let element =
                                e.call(SCORE_ENTRY_ARRAY_ELEMENT, &args![entries, k]).u32();
                            let value = (-(e.mem.i8(element + 4) as i32)) as u32;
                            let element =
                                e.call(SCORE_ENTRY_ARRAY_ELEMENT, &args![entries, k]).u32();
                            let index = e.mem.u32(element);
                            let los_map = held(e, slot);
                            fn_006d8970(e, Ptr::new(los_map), index, value as u8);
                        }
                        k += 1;
                    }
                    let mut k = 0u32;
                    while k < e.call(ARRAY_COUNT, &args![entries]).u32() {
                        let los_map = held(e, slot);
                        if e.call(LOS_MAP_MIN_SCORE, &args![los_map]).u8() as i8 == -3 {
                            break;
                        }
                        let element = e.call(SCORE_ENTRY_ARRAY_ELEMENT, &args![entries, k]).u32();
                        let score = -(e.mem.i8(element + 4) as i32);
                        let los_map = held(e, slot);
                        let minimum = e.call(LOS_MAP_MIN_SCORE, &args![los_map]).u8() as i8 as i32;
                        if score < minimum {
                            let element =
                                e.call(SCORE_ENTRY_ARRAY_ELEMENT, &args![entries, k]).u32();
                            let value = (-(e.mem.i8(element + 4) as i32)) as u32;
                            let los_map = held(e, slot);
                            e.call(LOS_MAP_SET_MIN_SCORE, &args![los_map, value]);
                        }
                        k += 1;
                    }
                }
                triangle += 1;
                e.call(SCORE_ENTRY_ARRAY_DESTRUCT, &args![entries]);
            }
            e.call(NAV_HOLDER_RELEASE, &args![holder]);
            navmesh_index += 1;
        }
        e.call(NAV_HOLDER_RELEASE, &args![start_holder]);
        true
    })
}

// Translated from 006d8910 (decompiled, FalloutNV.exe 1.4.0.525)
/// How many of the three edges of triangle `triangle` of a navmesh have a
/// neighbouring triangle (`u16` neighbour other than 0xffff).
pub fn fn_006d8910(e: &mut Engine, this: Ptr, triangle: u16) -> u8 {
    let mut count = 0u8;
    for edge in 0..3u32 {
        let record = e
            .call(
                TRIANGLE_RECORD_ARRAY_ELEMENT,
                &args![this.addr() + 0x38, triangle as u32],
            )
            .u32();
        if e.call(TRIANGLE_HAS_NEIGHBOUR, &args![record, edge]).bool() {
            count += 1;
        }
    }
    count
}

// Translated from 006d8970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `score` as the score of triangle `index` of a `PathingLOSMap`
/// (`pTriangleScores[index]`, +0x18).
pub fn fn_006d8970(e: &mut Engine, this: Ptr, index: u32, score: u8) {
    let scores = e.mem.u32(this.addr() + 0x18);
    e.mem.set_u8(scores.wrapping_add(index), score);
}

// Translated from 006d8990 (decompiled, FalloutNV.exe 1.4.0.525)
/// The number of navmeshes of a `PathingLOSMap`: the size of `NavMeshes`
/// (+0x1c).
pub fn fn_006d8990(e: &mut Engine, this: Ptr) -> u32 {
    e.call(ARRAY_COUNT, &args![this.addr() + 0x1c]).u32()
}

// Translated from 006d89b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of navmesh slot `index` of a `PathingLOSMap`'s `NavMeshes`
/// (+0x1c).
pub fn fn_006d89b0(e: &mut Engine, this: Ptr, index: u32) -> Ptr {
    e.call(POINTER_ARRAY_ELEMENT, &args![this.addr() + 0x1c, index])
        .ptr()
}

// Translated from 006d89d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills an 8-byte score entry: the triangle score index and the score
/// byte. Returns `this`.
pub fn fn_006d89d0(e: &mut Engine, this: Ptr, index: u32, score: u8) -> Ptr {
    e.mem.set_u32(this.addr(), index);
    e.mem.set_u8(this.addr() + 4, score);
    this
}

// Translated from 006d8a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_006d8a40` with `single_navmesh` false: collects the navmeshes within
/// `radius` of `location` in `list`.
pub fn fn_006d8a00(e: &mut Engine, location: Ptr, radius: f32, list: Ptr) -> bool {
    fn_006d8a40(e, location, radius, list, false)
}

// Translated from 006d8a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_006d8a40` with `single_navmesh` true: collects only the navmesh of
/// `location` in `list`.
pub fn fn_006d8a20(e: &mut Engine, location: Ptr, radius: f32, list: Ptr) -> bool {
    fn_006d8a40(e, location, radius, list, true)
}

// Translated from 006d8a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Collects navmesh holders into `list` (a list of `NiPointer<NavMesh>`):
/// the navmeshes whose bounds overlap the square of side `2 * radius`
/// around `location`.
///
/// With `single_navmesh` the list gets the location's own navmesh (false
/// without navmesh info). Otherwise it starts from the cells: the
/// location's cell when it is an interior cell, else the exterior cells
/// around the location's cell (3 by 3) that overlap the square. Every
/// cell's navmeshes are added; then, for each navmesh in the list (the list
/// grows while it is walked), the neighbouring navmeshes its info lists
/// (`NavMeshInfo` arrays at +0x24 and +0x34) are added when they are not in
/// the list yet, lie in the same cell (interior) or worldspace (exterior)
/// as the location and have bounds that overlap the square.
///
/// Returns false when the location has no cell or the list ends up empty.
pub fn fn_006d8a40(
    e: &mut Engine,
    location: Ptr,
    radius: f32,
    list: Ptr,
    single_navmesh: bool,
) -> bool {
    with_frame(e, 0x140, |e, f| {
        let position = f.at(-0x24);
        e.call(LOCATION_COPY_POSITION, &args![location, position]);
        let cell = e.call(LOCATION_GET_CELL, &args![location]).u32();
        if cell == 0 {
            return false;
        }
        let (x, y) = (e.mem.f32(position), e.mem.f32(position + 4));
        let search_minimum = f.at(-0x18);
        let lowest = e.global::<f32>(FLOAT_MAXIMUM_NEGATIVE);
        e.call(
            POINT3_CONSTRUCT,
            &args![search_minimum, x - radius, y - radius, lowest],
        );
        let search_maximum = f.at(-0x30);
        let highest = e.global::<f32>(FLOAT_LARGEST);
        e.call(
            POINT3_CONSTRUCT,
            &args![search_maximum, x + radius, y + radius, highest],
        );
        if !single_navmesh {
            let cells = f.at(-0x44);
            e.call(CELL_LIST_CONSTRUCT, &args![cells]);
            if !e.call(CELL_IS_INTERIOR, &args![cell]).bool() {
                let mut cell_x = e.call(CELL_GET_DATA_X, &args![cell]).i32().wrapping_sub(1);
                while cell_x < e.call(CELL_GET_DATA_X, &args![cell]).i32().wrapping_add(2) {
                    let mut cell_y = e.call(CELL_GET_DATA_Y, &args![cell]).i32().wrapping_sub(1);
                    while cell_y < e.call(CELL_GET_DATA_Y, &args![cell]).i32().wrapping_add(2) {
                        let cell_low = f.at(-0x64);
                        let lowest = e.global::<f32>(FLOAT_MAXIMUM_NEGATIVE);
                        e.call(
                            POINT3_CONSTRUCT,
                            &args![
                                cell_low,
                                cell_x.wrapping_shl(12) as f32,
                                cell_y.wrapping_shl(12) as f32,
                                lowest
                            ],
                        );
                        let cell_high = f.at(-0x58);
                        let highest = e.global::<f32>(FLOAT_LARGEST);
                        let width = e.global::<f64>(CELL_WIDTH);
                        let high_x = (cell_x.wrapping_shl(12) as f64 + width) as f32;
                        let high_y = (cell_y.wrapping_shl(12) as f64 + width) as f32;
                        e.call(POINT3_CONSTRUCT, &args![cell_high, high_x, high_y, highest]);
                        if fn_006d93d0(
                            e,
                            Ptr::new(cell_low),
                            Ptr::new(cell_high),
                            Ptr::new(search_minimum),
                            Ptr::new(search_maximum),
                        ) {
                            let worldspace = e.call(CELL_GET_WORLDSPACE, &args![cell]).u32();
                            let neighbour = e
                                .call(
                                    WORLDSPACE_GET_CELL_FROM_COORD,
                                    &args![worldspace, cell_x as u32, cell_y as u32],
                                )
                                .u32();
                            e.mem.set_u32(f.at(-0x68), neighbour);
                            e.call(INFO_ARRAY_ADD, &args![cells, f.at(-0x68)]);
                        }
                        cell_y = cell_y.wrapping_add(1);
                    }
                    cell_x = cell_x.wrapping_add(1);
                }
            } else {
                let own_cell = e.call(LOCATION_GET_CELL, &args![location]).u32();
                e.mem.set_u32(f.at(-0xb4), own_cell);
                e.call(INFO_ARRAY_ADD, &args![cells, f.at(-0xb4)]);
            }
            if e.call(ARRAY_IS_EMPTY, &args![cells]).bool() {
                e.call(CELL_LIST_DESTRUCT, &args![cells]);
                return false;
            }
            let mut cell_index = 0u32;
            while cell_index < e.call(ARRAY_COUNT, &args![cells]).u32() {
                let slot = e
                    .call(POINTER_ARRAY_ELEMENT, &args![cells, cell_index])
                    .u32();
                let listed_cell = e.mem.u32(slot);
                if listed_cell != 0 && e.call(CELL_NAVMESH_ARRAY, &args![listed_cell]).u32() != 0 {
                    let mut navmesh_index = 0u32;
                    loop {
                        let navmeshes = e.call(CELL_NAVMESH_ARRAY, &args![listed_cell]).u32();
                        if navmesh_index >= e.call(NAVMESH_ARRAY_COUNT, &args![navmeshes]).u32() {
                            break;
                        }
                        let out = f.at(-0xbc);
                        let navmeshes = e.call(CELL_NAVMESH_ARRAY, &args![listed_cell]).u32();
                        let holder = e
                            .call(NAVMESH_ARRAY_GET, &args![navmeshes, out, navmesh_index])
                            .u32();
                        e.call(NAV_HOLDER_LIST_ADD, &args![list, holder]);
                        e.call(NAV_HOLDER_RELEASE, &args![out]);
                        navmesh_index += 1;
                    }
                }
                cell_index += 1;
            }
            e.call(CELL_LIST_DESTRUCT, &args![cells]);
        } else {
            if !e
                .call(LOCATION_RESOLVE_NAVMESH_INFO, &args![location, 0u32])
                .bool()
            {
                return false;
            }
            let holder = f.at(-0x78);
            e.call(NAV_HOLDER_CONSTRUCT, &args![holder]);
            if !e
                .call(LOCATION_GET_NAVMESH, &args![location, holder])
                .bool()
            {
                e.call(NAV_HOLDER_RELEASE, &args![holder]);
                return false;
            }
            e.call(NAV_HOLDER_LIST_ADD, &args![list, holder]);
            e.call(NAV_HOLDER_RELEASE, &args![holder]);
        }
        if e.call(ARRAY_IS_EMPTY, &args![list]).bool() {
            return false;
        }
        let mut list_index = 0u32;
        while list_index < e.call(ARRAY_COUNT, &args![list]).u32() {
            let element = e
                .call(POINTER_ARRAY_ELEMENT_THUNK, &args![list, list_index])
                .u32();
            let holder = f.at(-0x88);
            e.call(NAV_HOLDER_COPY_CONSTRUCT, &args![holder, element]);
            let navmesh = held(e, holder);
            let info = e.call(NAVMESH_GET_INFO, &args![navmesh]).u32();
            let first_count = e.call(ARRAY_COUNT, &args![info + 0x24]).u32();
            let navmesh = held(e, holder);
            let info = e.call(NAVMESH_GET_INFO, &args![navmesh]).u32();
            let second_count = e.call(ARRAY_COUNT, &args![info + 0x34]).u32();
            let mut neighbour_index = 0u32;
            while neighbour_index < first_count.wrapping_add(second_count) {
                let candidate = if neighbour_index < first_count {
                    let navmesh = held(e, holder);
                    let info = e.call(NAVMESH_GET_INFO, &args![navmesh]).u32();
                    let slot = e
                        .call(POINTER_ARRAY_ELEMENT, &args![info + 0x24, neighbour_index])
                        .u32();
                    e.mem.u32(slot)
                } else {
                    let navmesh = held(e, holder);
                    let info = e.call(NAVMESH_GET_INFO, &args![navmesh]).u32();
                    let slot = e
                        .call(
                            POINTER_ARRAY_ELEMENT,
                            &args![info + 0x34, neighbour_index - first_count],
                        )
                        .u32();
                    e.mem.u32(slot)
                };
                if candidate != 0 {
                    let found = f.at(-0x94);
                    e.call(NAV_HOLDER_CONSTRUCT, &args![found]);
                    if e.call(NAVMESH_INFO_GET_NAVMESH, &args![candidate, found])
                        .bool()
                    {
                        let listed = e
                            .call(
                                HOLDER_LIST_FIND_INDEX,
                                &args![list, found, 0u32, HOLDER_COMPARE_FUNCTION],
                            )
                            .u32();
                        if listed == 0xffff_ffff {
                            let navmesh = held(e, found);
                            let navmesh_cell = e.call(NAVMESH_GET_CELL, &args![navmesh]).u32();
                            if navmesh_cell != 0 {
                                let same_place = if e.call(CELL_IS_INTERIOR, &args![cell]).bool() {
                                    navmesh_cell == cell
                                } else {
                                    let own = e.call(CELL_GET_WORLDSPACE, &args![cell]).u32();
                                    let other =
                                        e.call(CELL_GET_WORLDSPACE, &args![navmesh_cell]).u32();
                                    own == other
                                };
                                if same_place {
                                    let minimum = f.at(-0xb0);
                                    e.call(POINT3_DEFAULT_CONSTRUCTOR, &args![minimum]);
                                    let maximum = f.at(-0xa0);
                                    e.call(POINT3_DEFAULT_CONSTRUCTOR, &args![maximum]);
                                    let navmesh = held(e, found);
                                    if e.call(NAVMESH_GET_BOUNDS, &args![navmesh, minimum, maximum])
                                        .bool()
                                        && fn_006d93d0(
                                            e,
                                            Ptr::new(minimum),
                                            Ptr::new(maximum),
                                            Ptr::new(search_minimum),
                                            Ptr::new(search_maximum),
                                        )
                                    {
                                        e.call(NAV_HOLDER_LIST_ADD, &args![list, found]);
                                    }
                                }
                            }
                        }
                    }
                    e.call(NAV_HOLDER_RELEASE, &args![found]);
                }
                neighbour_index += 1;
            }
            e.call(NAV_HOLDER_RELEASE, &args![holder]);
            list_index += 1;
        }
        true
    })
}

// Translated from 006d9100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Pathing::GetNavmeshNormalForLocation` (Xbox PDB): stores the normal of
/// the navmesh triangle the location lies on in `out` (three words).
/// Returns false without navmesh info or without a triangle.
pub fn pathing_get_navmesh_normal_for_location(e: &mut Engine, location: Ptr, out: Ptr) -> bool {
    with_frame(e, 0x80, |e, f| {
        if !e
            .call(LOCATION_RESOLVE_NAVMESH_INFO, &args![location, 0u32])
            .bool()
        {
            return false;
        }
        let holder = f.at(-0x10);
        e.call(NAV_HOLDER_CONSTRUCT, &args![holder]);
        let triangle = f.at(-0x14);
        if !e
            .call(
                LOCATION_GET_NAVMESH_AND_TRIANGLE,
                &args![location, holder, triangle],
            )
            .bool()
        {
            e.call(NAV_HOLDER_RELEASE, &args![holder]);
            return false;
        }
        let navmesh = held(e, holder);
        let normal = e
            .call(
                TRIANGLE_NORMAL,
                &args![navmesh, f.at(-0x24), e.mem.u16(triangle) as u32],
            )
            .u32();
        copy_words(e, out.addr(), normal, 3);
        e.call(NAV_HOLDER_RELEASE, &args![holder]);
        true
    })
}

// Translated from 006d91d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds every navmesh of the loaded cells to `list` (a list of
/// `NiPointer<NavMesh>`): those of the `TES` interior cell when there is
/// one (the word at +0x34 of the `TES` object), else those of all exterior
/// cells. Always returns true.
pub fn fn_006d91d0(e: &mut Engine, list: Ptr) -> bool {
    with_frame(e, 0x80, |e, f| {
        let tes = e.global::<u32>(TES_GLOBAL);
        if e.call(WORD_AT_OFFSET_0X34, &args![tes]).u32() != 0 {
            let interior = e.call(WORD_AT_OFFSET_0X34, &args![tes]).u32();
            let navmeshes = e.call(CELL_NAVMESH_ARRAY, &args![interior]).u32();
            if navmeshes != 0 {
                add_cell_navmeshes(e, list, navmeshes, f.at(-0x2c));
            }
        } else {
            let count = e.call(TES_EXTERIOR_CELL_COUNT, &args![tes]).u32();
            let mut index = 0u32;
            while index < count {
                let cell = e.call(TES_EXTERIOR_CELL_AT, &args![tes, index]).u32();
                if cell != 0 {
                    let navmeshes = e.call(CELL_NAVMESH_ARRAY, &args![cell]).u32();
                    if navmeshes != 0 {
                        add_cell_navmeshes(e, list, navmeshes, f.at(-0x30));
                    }
                }
                index += 1;
            }
        }
        true
    })
}

// Translated from 006d9350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards to `0054dc00` on the cell a form designates: `form` itself, or
/// for form type 0x41 the word at +0x34 of the form; `results` is an array
/// that gets room for at least 32 entries first. Returns 0 for a null
/// form, results array or cell.
pub fn fn_006d9350(e: &mut Engine, form: Ptr, argument: u32, distance: f32, results: Ptr) -> u32 {
    if form.addr() == 0 || results.addr() == 0 {
        return 0;
    }
    if e.call(WORD_AT_OFFSET_0XC, &args![results]).u32() < 0x20 {
        e.call(ARRAY_SET_RESERVED_SIZE, &args![results, 0x20u32]);
    }
    let cell = if e.call(FORM_TYPE, &args![form]).u32() == 0x41 {
        e.call(WORD_AT_OFFSET_0X34, &args![form]).u32()
    } else {
        form.addr()
    };
    if cell == 0 {
        return 0;
    }
    e.call(
        CELL_QUERY_006D9350,
        &args![cell, argument, distance, results],
    )
    .u32()
}

// Translated from 006d93d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the box `a .. b` overlaps the box `c .. d` (corner points of
/// three `float`s): `b > c` and `a < d` on every axis. A comparison with a
/// NaN counts as passing.
pub fn fn_006d93d0(e: &mut Engine, a: Ptr, b: Ptr, c: Ptr, d: Ptr) -> bool {
    for axis in 0..3u32 {
        let (a, b, c, d) = (
            e.mem.f32(a.addr() + 4 * axis),
            e.mem.f32(b.addr() + 4 * axis),
            e.mem.f32(c.addr() + 4 * axis),
            e.mem.f32(d.addr() + 4 * axis),
        );
        if b <= c || a >= d {
            return false;
        }
    }
    true
}

// Translated from 006d9480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether no open edge (an edge without neighbouring triangle) of the
/// navmesh triangles in the box of half-width `radius` around `location`
/// comes within `radius` of the location.
///
/// The navmeshes come from `fn_006d8a20`; for each, `00696c00` lists the
/// triangles inside the box `location -/+ (radius, radius, 0)`, which are
/// collected as (navmesh info, triangle) references. Each edge of each
/// referenced triangle without a neighbour is then tested with `006ba9e0`;
/// the first one within the radius makes the answer false.
pub fn fn_006d9480(e: &mut Engine, location: Ptr, radius: f32) -> bool {
    with_frame(e, 0x140, |e, f| {
        let triangles = f.at(-0x1c);
        e.call(TRIANGLE_REFERENCE_ARRAY_CONSTRUCT, &args![triangles]);
        let navmeshes = f.at(-0x3c);
        e.call(NAVMESH_LIST_CONSTRUCT, &args![navmeshes]);
        fn_006d8a20(e, location, radius, Ptr::new(navmeshes));
        let mut navmesh_index = 0u32;
        while navmesh_index < e.call(ARRAY_COUNT, &args![navmeshes]).u32() {
            let found = f.at(-0x50);
            e.call(U16_ARRAY_CONSTRUCT, &args![found]);
            let extent = e
                .call(
                    POINT3_CONSTRUCT,
                    &args![f.at(-0x80), radius, radius, 0.0f32],
                )
                .u32();
            let position = e
                .call(LOCATION_COPY_POSITION, &args![location, f.at(-0x74)])
                .u32();
            let maximum = e
                .call(POINT3_ADD, &args![position, f.at(-0x8c), extent])
                .u32();
            let extent = e
                .call(
                    POINT3_CONSTRUCT,
                    &args![f.at(-0xa4), radius, radius, 0.0f32],
                )
                .u32();
            let position = e
                .call(LOCATION_COPY_POSITION, &args![location, f.at(-0x98)])
                .u32();
            let minimum = e
                .call(POINT3_SUBTRACT, &args![position, f.at(-0xb0), extent])
                .u32();
            let slot = e
                .call(POINTER_ARRAY_ELEMENT, &args![navmeshes, navmesh_index])
                .u32();
            let navmesh = held(e, slot);
            e.call(
                NAVMESH_FIND_TRIANGLES_IN_BOX,
                &args![navmesh, minimum, maximum, found],
            );
            let mut found_index = 0u32;
            while found_index < e.call(ARRAY_COUNT, &args![found]).u32() {
                let element = e.call(U16_ARRAY_ELEMENT, &args![found, found_index]).u32();
                let triangle = e.mem.u16(element) as u32;
                let slot = e
                    .call(POINTER_ARRAY_ELEMENT, &args![navmeshes, navmesh_index])
                    .u32();
                let navmesh = held(e, slot);
                let info = e.call(NAVMESH_GET_INFO, &args![navmesh]).u32();
                let reference = e
                    .call(
                        TRIANGLE_REFERENCE_CONSTRUCT,
                        &args![f.at(-0xb8), info, triangle],
                    )
                    .u32();
                e.call(TRIANGLE_REFERENCE_ARRAY_ADD, &args![triangles, reference]);
                found_index += 1;
            }
            e.call(U16_ARRAY_DESTRUCT, &args![found]);
            navmesh_index += 1;
        }
        let scratch = f.at(-0x2c);
        e.call(SCRATCH_OBJECT_CONSTRUCT, &args![scratch]);
        let mut reference_index = 0u32;
        while reference_index < e.call(ARRAY_COUNT, &args![triangles]).u32() {
            let holder = f.at(-0x5c);
            e.call(NAV_HOLDER_CONSTRUCT, &args![holder]);
            let reference = e
                .call(
                    SCORE_ENTRY_ARRAY_ELEMENT,
                    &args![triangles, reference_index],
                )
                .u32();
            let info = e.mem.u32(reference);
            if e.call(NAVMESH_INFO_GET_NAVMESH, &args![info, holder])
                .bool()
            {
                for edge in 0..3u32 {
                    let reference = e
                        .call(
                            SCORE_ENTRY_ARRAY_ELEMENT,
                            &args![triangles, reference_index],
                        )
                        .u32();
                    let triangle = e.mem.u16(reference + 4) as u32;
                    let navmesh = held(e, holder);
                    let record = e
                        .call(NAVMESH_GET_TRIANGLE, &args![navmesh, triangle])
                        .u32();
                    if e.call(TRIANGLE_HAS_NEIGHBOUR, &args![record, edge]).bool() {
                        continue;
                    }
                    let reference = e
                        .call(
                            SCORE_ENTRY_ARRAY_ELEMENT,
                            &args![triangles, reference_index],
                        )
                        .u32();
                    let triangle = e.mem.u16(reference + 4) as u32;
                    let vertices = f.at(-0x68);
                    let navmesh = held(e, holder);
                    e.call(
                        NAVMESH_EDGE_VERTICES,
                        &args![navmesh, vertices, triangle, edge],
                    );
                    let position = e
                        .call(LOCATION_COPY_POSITION, &args![location, f.at(-0xc4)])
                        .u32();
                    let (first, second) = (e.mem.u32(vertices), e.mem.u32(vertices + 4));
                    if e.call(
                        SEGMENT_WITHIN_RADIUS,
                        &args![first, second, position, radius],
                    )
                    .bool()
                    {
                        e.call(NAV_HOLDER_RELEASE, &args![holder]);
                        e.call(SCRATCH_OBJECT_DESTRUCT, &args![scratch]);
                        e.call(NAVMESH_LIST_DESTRUCT, &args![navmeshes]);
                        e.call(TRIANGLE_REFERENCE_ARRAY_DESTRUCT, &args![triangles]);
                        return false;
                    }
                }
            }
            e.call(NAV_HOLDER_RELEASE, &args![holder]);
            reference_index += 1;
        }
        e.call(SCRATCH_OBJECT_DESTRUCT, &args![scratch]);
        e.call(NAVMESH_LIST_DESTRUCT, &args![navmeshes]);
        e.call(TRIANGLE_REFERENCE_ARRAY_DESTRUCT, &args![triangles]);
        true
    })
}

// Translated from 006d97b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Pathing::GetNavMeshZForLocation` (Xbox PDB): builds a location from
/// `position`, `cell` and `worldspace` and asks `fn_006d9830` for its
/// navmesh height, stored in `out`.
pub fn pathing_get_nav_mesh_z_for_location(
    e: &mut Engine,
    position: Ptr,
    cell: Ptr,
    worldspace: Ptr,
    out: Ptr,
) -> bool {
    with_frame(e, 0x80, |e, f| {
        let location = f.at(-0x34);
        e.call(
            LOCATION_CONSTRUCT_FROM_PARTS,
            &args![location, position, cell, worldspace],
        );
        let found = pathing_get_nav_mesh_z_for_location_ov2(e, Ptr::new(location), out);
        e.call(LOCATION_DESTRUCT, &args![location]);
        found
    })
}

// Translated from 006d9830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Pathing::GetNavMeshZForLocation_ov2` (Xbox PDB): stores in `out` the
/// height of the navmesh triangle under the location (computed by
/// `NavMesh::ComputeTriangleZForLocation` with limits of 180.0), or the
/// location's own height when it has no navmesh info, no triangle, or the
/// position is outside the triangle. Returns whether the navmesh height was
/// found.
pub fn pathing_get_nav_mesh_z_for_location_ov2(e: &mut Engine, location: Ptr, out: Ptr) -> bool {
    with_frame(e, 0xa0, |e, f| {
        let own_height = |e: &mut Engine, scratch: i32| {
            let position = e
                .call(LOCATION_COPY_POSITION, &args![location, f.at(scratch)])
                .u32();
            let height = e.mem.f32(position + 8);
            e.mem.set_f32(out.addr(), height);
        };
        if !e
            .call(LOCATION_RESOLVE_NAVMESH_INFO, &args![location, 0u32])
            .bool()
        {
            own_height(e, -0x20);
            return false;
        }
        if !e.call(LOCATION_HAS_TRIANGLE, &args![location]).bool() {
            own_height(e, -0x2c);
            return false;
        }
        let holder = f.at(-0x14);
        e.call(NAV_HOLDER_CONSTRUCT, &args![holder]);
        let triangle = f.at(-0x10);
        if !e
            .call(
                LOCATION_GET_NAVMESH_AND_TRIANGLE,
                &args![location, holder, triangle],
            )
            .bool()
        {
            own_height(e, -0x38);
            e.call(NAV_HOLDER_RELEASE, &args![holder]);
            return false;
        }
        let limit = e.global::<f32>(TRIANGLE_Z_LIMIT);
        let position = e
            .call(LOCATION_COPY_POSITION, &args![location, f.at(-0x48)])
            .u32();
        let navmesh = held(e, holder);
        let inside = e
            .call(
                NAVMESH_COMPUTE_TRIANGLE_Z,
                &args![
                    navmesh,
                    e.mem.u16(triangle) as u32,
                    position,
                    limit,
                    limit,
                    out
                ],
            )
            .bool();
        if !inside {
            own_height(e, -0x54);
        }
        e.call(NAV_HOLDER_RELEASE, &args![holder]);
        inside
    })
}

// Translated from 006d99a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds the direction a path starts in, from `heading`, the location
/// `from`, the location `to` and the sideways `distance`; stores it in
/// `out` (three `float`s) and returns `out`.
///
/// The heading becomes a unit direction (the heading rotation applied to
/// the global axis). The end point is 256 units from `from` along it, or
/// where `fn_006d7490` (line of sight) stops. The two lines parallel to
/// that line, `distance` to either side (the direction crossed with the
/// global up axis), are checked as well and pull the end point to where
/// they stop when that is closer. The end point is pulled back by
/// `distance * 1.2` along the direction, and has to be in line of sight of
/// `to`, with both sideways lines from `to` passing too. The result is the
/// vector from `from` to the end point, or the zero vector when a check
/// fails, when the end point lies behind the heading by more than the
/// cosine -0.7, or when it lies behind the heading and the target is
/// closer than 64 units.
pub fn fn_006d99a0(
    e: &mut Engine,
    out: Ptr,
    heading: f32,
    from: Ptr,
    to: Ptr,
    distance: f32,
) -> Ptr {
    with_frame(e, 0x3a0, |e, f| {
        // The locations in the order they are built; every exit destroys
        // the built ones in reverse.
        let mut locations: Vec<u32> = Vec::new();
        let matrix = f.at(-0x130);
        e.call(POINT3_DEFAULT_CONSTRUCTOR, &args![matrix]);
        e.call(MATRIX_FROM_HEADING, &args![matrix, heading]);
        let forward = f.at(-0x7c);
        e.call(MATRIX_TIMES_VECTOR, &args![matrix, forward, CIRCLE_AXIS_A]);
        let from_copy = f.at(-0x284);
        e.call(LOCATION_COPY_CONSTRUCT, &args![from_copy, from]);
        locations.push(from_copy);
        let start = f.at(-0x154);
        e.call(LOCATION_COPY_POSITION, &args![from_copy, start]);
        let probe = e.global::<f32>(TANGENT_PROBE_DISTANCE);
        let probe_vector = e
            .call(POINT3_TIMES_SCALAR, &args![forward, f.at(-0x290), probe])
            .u32();
        let end = f.at(-0x148);
        e.call(POINT3_ADD, &args![start, end, probe_vector]);
        let hit = f.at(-0x234);
        e.call(LOCATION_CONSTRUCT, &args![hit]);
        locations.push(hit);
        let zero = Ptr::new(0);
        if !pathing_check_line_of_sight_ov2(
            e,
            Ptr::new(from_copy),
            Ptr::new(end),
            Ptr::new(hit),
            zero,
        ) {
            let position = e
                .call(LOCATION_COPY_POSITION, &args![hit, f.at(-0x29c)])
                .u32();
            copy_words(e, end, position, 3);
        }
        let ahead = f.at(-0x1e4);
        e.call(POINT3_SUBTRACT, &args![end, ahead, start]);
        let cross = e
            .call(
                POINT3_UNIT_CROSS,
                &args![ahead, f.at(-0x2a8), TANGENT_CROSS_AXIS],
            )
            .u32();
        let side = f.at(-0x10c);
        e.call(POINT3_TIMES_SCALAR, &args![cross, side, distance]);
        // The line `distance` to one side.
        let start_side = e
            .call(POINT3_SUBTRACT, &args![start, f.at(-0x2b4), side])
            .u32();
        let first_start = f.at(-0x25c);
        e.call(
            LOCATION_CONSTRUCT_FROM_POINT_AND_LOCATION,
            &args![first_start, start_side, from_copy],
        );
        locations.push(first_start);
        let end_side = e
            .call(POINT3_SUBTRACT, &args![end, f.at(-0x2c0), side])
            .u32();
        let first_end = f.at(-0xf4);
        e.call(
            LOCATION_CONSTRUCT_FROM_POINT_AND_LOCATION,
            &args![first_end, end_side, from_copy],
        );
        locations.push(first_end);
        if !pathing_check_line_of_sight(
            e,
            Ptr::new(first_start),
            Ptr::new(first_end),
            Ptr::new(hit),
            zero,
        ) {
            pull_end_to_hit(
                e,
                [first_start, hit],
                [-0x2d8, -0x2cc, -0x2e4, -0x2f0],
                f,
                ahead,
                end,
            );
        }
        // The line `distance` to the other side.
        let start_side = e.call(POINT3_ADD, &args![start, f.at(-0x2fc), side]).u32();
        let second_start = f.at(-0x1d8);
        e.call(
            LOCATION_CONSTRUCT_FROM_POINT_AND_LOCATION,
            &args![second_start, start_side, from_copy],
        );
        locations.push(second_start);
        let end_side = e.call(POINT3_ADD, &args![end, f.at(-0x308), side]).u32();
        let second_end = f.at(-0x20c);
        e.call(
            LOCATION_CONSTRUCT_FROM_POINT_AND_LOCATION,
            &args![second_end, end_side, from_copy],
        );
        locations.push(second_end);
        if !pathing_check_line_of_sight(
            e,
            Ptr::new(second_start),
            Ptr::new(second_end),
            Ptr::new(hit),
            zero,
        ) {
            pull_end_to_hit(
                e,
                [second_start, hit],
                [-0x320, -0x314, -0x32c, -0x338],
                f,
                ahead,
                end,
            );
        }
        // Pull the end point back by the side distance times 1.2.
        let along = e
            .call(POINT3_TIMES_SCALAR, &args![forward, f.at(-0x344), distance])
            .u32();
        let pull_back = e.global::<f32>(TANGENT_PULL_BACK);
        let pulled = e
            .call(POINT3_TIMES_SCALAR, &args![along, f.at(-0x350), pull_back])
            .u32();
        e.call(POINT3_SUBTRACT_ASSIGN, &args![end, pulled]);
        let to_copy = f.at(-0xa4);
        e.call(LOCATION_COPY_CONSTRUCT, &args![to_copy, to]);
        locations.push(to_copy);
        let target = f.at(-0x13c);
        e.call(LOCATION_COPY_POSITION, &args![to_copy, target]);
        let target_hit = f.at(-0x188);
        e.call(LOCATION_CONSTRUCT, &args![target_hit]);
        locations.push(target_hit);
        if !pathing_check_line_of_sight_ov2(
            e,
            Ptr::new(to_copy),
            Ptr::new(end),
            Ptr::new(target_hit),
            zero,
        ) {
            return finish_with_zero(e, &locations, out);
        }
        // The same two side lines, now around the target.
        let across = f.at(-0x100);
        e.call(POINT3_SUBTRACT, &args![end, across, target]);
        let cross = e
            .call(
                POINT3_UNIT_CROSS,
                &args![across, f.at(-0x35c), TANGENT_CROSS_AXIS],
            )
            .u32();
        // (the game computes this second sideways vector and never uses it)
        e.call(POINT3_TIMES_SCALAR, &args![cross, f.at(-0x160), distance]);
        let target_low = e
            .call(POINT3_SUBTRACT, &args![target, f.at(-0x368), side])
            .u32();
        let low_start = f.at(-0x1b0);
        e.call(
            LOCATION_CONSTRUCT_FROM_POINT_AND_LOCATION,
            &args![low_start, target_low, to_copy],
        );
        locations.push(low_start);
        let end_low = e
            .call(POINT3_SUBTRACT, &args![end, f.at(-0x374), side])
            .u32();
        let low_end = f.at(-0x34);
        e.call(
            LOCATION_CONSTRUCT_FROM_POINT_AND_LOCATION,
            &args![low_end, end_low, to_copy],
        );
        locations.push(low_end);
        if !pathing_check_line_of_sight(
            e,
            Ptr::new(low_start),
            Ptr::new(low_end),
            Ptr::new(target_hit),
            zero,
        ) {
            return finish_with_zero(e, &locations, out);
        }
        let target_high = e.call(POINT3_ADD, &args![target, f.at(-0x380), side]).u32();
        let high_start = f.at(-0x5c);
        e.call(
            LOCATION_CONSTRUCT_FROM_POINT_AND_LOCATION,
            &args![high_start, target_high, to_copy],
        );
        locations.push(high_start);
        let end_high = e.call(POINT3_ADD, &args![end, f.at(-0x38c), side]).u32();
        let high_end = f.at(-0xcc);
        e.call(
            LOCATION_CONSTRUCT_FROM_POINT_AND_LOCATION,
            &args![high_end, end_high, to_copy],
        );
        locations.push(high_end);
        if !pathing_check_line_of_sight(
            e,
            Ptr::new(high_start),
            Ptr::new(high_end),
            Ptr::new(target_hit),
            zero,
        ) {
            return finish_with_zero(e, &locations, out);
        }
        // The target must not lie behind the heading.
        let to_target = f.at(-0x6c);
        e.call(POINT3_SUBTRACT, &args![target, to_target, start]);
        let length = e.call(POINT3_UNITIZE_GET_LENGTH, &args![to_target]).f32();
        let cosine = e.call(POINT3_DOT, &args![to_target, forward]).f32();
        if (cosine as f64) < e.global::<f64>(TANGENT_MINIMUM_DOT) {
            return finish_with_zero(e, &locations, out);
        }
        if cosine < 0.0 && (length as f64) < e.global::<f64>(TANGENT_MINIMUM_LENGTH) {
            return finish_with_zero(e, &locations, out);
        }
        e.call(POINT3_SUBTRACT, &args![end, out, start]);
        destroy_locations(e, &locations);
        out
    })
}

// Translated from 006da420 (decompiled, FalloutNV.exe 1.4.0.525)
/// The first half of `fn_006d99a0`: the unit direction of `heading`, the
/// end point 256 units from `location` along it (or where the line of
/// sight stops), and the same end point pulled in to where the two lines
/// `distance` to either side stop when that is closer. Stores in `out` the
/// vector from the end point to the location's position (the end point
/// subtracted from the start, opposite to `fn_006d99a0`) and returns
/// `out`.
pub fn fn_006da420(e: &mut Engine, out: Ptr, heading: f32, location: Ptr, distance: f32) -> Ptr {
    with_frame(e, 0x230, |e, f| {
        let mut locations: Vec<u32> = Vec::new();
        let matrix = f.at(-0x70);
        e.call(POINT3_DEFAULT_CONSTRUCTOR, &args![matrix]);
        e.call(MATRIX_FROM_HEADING, &args![matrix, heading]);
        let forward = f.at(-0x18);
        e.call(MATRIX_TIMES_VECTOR, &args![matrix, forward, CIRCLE_AXIS_A]);
        let copy = f.at(-0x15c);
        e.call(LOCATION_COPY_CONSTRUCT, &args![copy, location]);
        locations.push(copy);
        let start = f.at(-0x88);
        e.call(LOCATION_COPY_POSITION, &args![copy, start]);
        let probe = e.global::<f32>(TANGENT_PROBE_DISTANCE);
        let probe_vector = e
            .call(POINT3_TIMES_SCALAR, &args![forward, f.at(-0x168), probe])
            .u32();
        let end = f.at(-0x7c);
        e.call(POINT3_ADD, &args![start, end, probe_vector]);
        let hit = f.at(-0x10c);
        e.call(LOCATION_CONSTRUCT, &args![hit]);
        locations.push(hit);
        let zero = Ptr::new(0);
        if !pathing_check_line_of_sight_ov2(e, Ptr::new(copy), Ptr::new(end), Ptr::new(hit), zero) {
            let position = e
                .call(LOCATION_COPY_POSITION, &args![hit, f.at(-0x174)])
                .u32();
            copy_words(e, end, position, 3);
        }
        let ahead = f.at(-0xbc);
        e.call(POINT3_SUBTRACT, &args![end, ahead, start]);
        let cross = e
            .call(
                POINT3_UNIT_CROSS,
                &args![ahead, f.at(-0x180), TANGENT_CROSS_AXIS],
            )
            .u32();
        let side = f.at(-0x4c);
        e.call(POINT3_TIMES_SCALAR, &args![cross, side, distance]);
        let start_side = e
            .call(POINT3_SUBTRACT, &args![start, f.at(-0x18c), side])
            .u32();
        let first_start = f.at(-0x134);
        e.call(
            LOCATION_CONSTRUCT_FROM_POINT_AND_LOCATION,
            &args![first_start, start_side, copy],
        );
        locations.push(first_start);
        let end_side = e
            .call(POINT3_SUBTRACT, &args![end, f.at(-0x198), side])
            .u32();
        let first_end = f.at(-0x40);
        e.call(
            LOCATION_CONSTRUCT_FROM_POINT_AND_LOCATION,
            &args![first_end, end_side, copy],
        );
        locations.push(first_end);
        if !pathing_check_line_of_sight(
            e,
            Ptr::new(first_start),
            Ptr::new(first_end),
            Ptr::new(hit),
            zero,
        ) {
            pull_end_to_hit(
                e,
                [first_start, hit],
                [-0x1b0, -0x1a4, -0x1bc, -0x1c8],
                f,
                ahead,
                end,
            );
        }
        let start_side = e.call(POINT3_ADD, &args![start, f.at(-0x1d4), side]).u32();
        let second_start = f.at(-0xb0);
        e.call(
            LOCATION_CONSTRUCT_FROM_POINT_AND_LOCATION,
            &args![second_start, start_side, copy],
        );
        locations.push(second_start);
        let end_side = e.call(POINT3_ADD, &args![end, f.at(-0x1e0), side]).u32();
        let second_end = f.at(-0xe4);
        e.call(
            LOCATION_CONSTRUCT_FROM_POINT_AND_LOCATION,
            &args![second_end, end_side, copy],
        );
        locations.push(second_end);
        if !pathing_check_line_of_sight(
            e,
            Ptr::new(second_start),
            Ptr::new(second_end),
            Ptr::new(hit),
            zero,
        ) {
            pull_end_to_hit(
                e,
                [second_start, hit],
                [-0x1f8, -0x1ec, -0x204, -0x210],
                f,
                ahead,
                end,
            );
        }
        e.call(POINT3_SUBTRACT, &args![start, out, end]);
        destroy_locations(e, &locations);
        out
    })
}

// Translated from 006da7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Pathing::ProfilePathing` (Xbox PDB): a profiling routine that makes
/// `count` random points in the cell (or in a 1000 by 1000 square for an
/// interior cell, around the origin) and runs one kind of pathing over
/// them. `mode` 0 compares the two implementations of the polygon test
/// over four consecutive points (`006b9e90` and `006ba010`) and prints a
/// message when they differ; mode 1 runs a cover search
/// (`fn_006d62e0`) from every point; mode 2 (whose points come from
/// `Pathing::FindPointOnNavMesh` around the position, retried until it
/// succeeds) solves a base path request from the position to every point.
///
/// Always returns 0.0 (also when the position has no navmesh info).
pub fn pathing_profile_pathing(
    e: &mut Engine,
    cell: Ptr,
    position: Ptr,
    mode: u32,
    count: u32,
) -> f32 {
    with_frame(e, 0x300, |e, f| {
        let holder = f.at(-0x20);
        e.call(NAV_HOLDER_CONSTRUCT, &args![holder]);
        copy_words(e, f.at(-0x2c), ZERO_VECTOR, 3);
        let half_width = e.global::<f32>(PROFILE_INTERIOR_HALF_WIDTH);
        let (min_x, min_y, max_x, max_y);
        if e.call(CELL_IS_INTERIOR, &args![cell]).bool() {
            min_x = -half_width;
            min_y = -half_width;
            max_x = half_width;
            max_y = half_width;
        } else {
            let cell_x = e.call(CELL_GET_DATA_X, &args![cell]).i32();
            min_x = cell_x.wrapping_shl(12) as f32;
            let cell_y = e.call(CELL_GET_DATA_Y, &args![cell]).i32();
            min_y = cell_y.wrapping_shl(12) as f32;
            let width = e.global::<f64>(CELL_WIDTH);
            max_x = (min_x as f64 + width) as f32;
            max_y = (min_y as f64 + width) as f32;
        }
        let location = f.at(-0x70);
        e.call(
            LOCATION_CONSTRUCT_FROM_POSITION_AND_CELL,
            &args![location, position, cell],
        );
        if !e
            .call(LOCATION_RESOLVE_NAVMESH_INFO, &args![location, 0u32])
            .bool()
        {
            e.call(LOCATION_DESTRUCT, &args![location]);
            e.call(NAV_HOLDER_RELEASE, &args![holder]);
            return 0.0;
        }
        e.call(LOCATION_GET_NAVMESH, &args![location, holder]);
        let points = f.at(-0x40);
        e.call(POINT_ARRAY_CONSTRUCT, &args![points, count]);
        if mode == 2 {
            let mut index = 0u32;
            while index < count {
                let out = e.call(POINT_ARRAY_ELEMENT, &args![points, index]).u32();
                let search_radius = e.global::<f32>(PROFILE_POINT_SEARCH_RADIUS);
                let worldspace = e.call(LOCATION_GET_WORLDSPACE, &args![location]).u32();
                if !pathing_find_point_on_nav_mesh(
                    e,
                    worldspace,
                    0,
                    position,
                    search_radius,
                    Ptr::new(out),
                ) {
                    index = index.wrapping_sub(1);
                }
                index = index.wrapping_add(1);
            }
        } else {
            let mut index = 0u32;
            while index < count {
                let x = e.call(RANDOM_FLOAT, &args![min_x, max_x]).f32();
                let element = e.call(POINT_ARRAY_ELEMENT, &args![points, index]).u32();
                e.mem.set_f32(element, x);
                let y = e.call(RANDOM_FLOAT, &args![min_y, max_y]).f32();
                let element = e.call(POINT_ARRAY_ELEMENT, &args![points, index]).u32();
                e.mem.set_f32(element + 4, y);
                let element = e.call(POINT_ARRAY_ELEMENT, &args![points, index]).u32();
                let height = e.mem.u32(position.addr() + 8);
                e.mem.set_u32(element + 8, height);
                index += 1;
            }
        }
        let mut index = 0u32;
        while index < count {
            match mode {
                0 => {
                    let planar = f.at(-0x98);
                    e.call(POINT3_DEFAULT_CONSTRUCTOR, &args![planar]);
                    let corners = [f.at(-0x88), f.at(-0xa4), f.at(-0xac), f.at(-0x90)];
                    for (offset, corner) in corners.iter().enumerate() {
                        let which = (index + offset as u32) % count;
                        let element = e.call(POINT_ARRAY_ELEMENT, &args![points, which]).u32();
                        let y = e.mem.f32(element + 4);
                        let element = e.call(POINT_ARRAY_ELEMENT, &args![points, which]).u32();
                        let x = e.mem.f32(element);
                        e.call(POINT2_CONSTRUCT, &args![*corner, x, y]);
                    }
                    let first = e
                        .call(
                            POLYGON_TEST_A,
                            &args![corners[0], corners[1], corners[2], corners[3], planar],
                        )
                        .bool();
                    let second = e
                        .call(
                            POLYGON_TEST_B,
                            &args![corners[0], corners[1], corners[2], corners[3]],
                        )
                        .bool();
                    if first != second {
                        e.call(DEBUG_PRINT, &args![PROFILE_MISMATCH_MESSAGE]);
                    }
                }
                1 => {
                    let request = f.at(-0x1ac);
                    e.call(COVER_REQUEST_CONSTRUCT, &args![request]);
                    fn_006d3ac0(e, Ptr::new(request), Ptr::new(location));
                    let element = e.call(POINT_ARRAY_ELEMENT, &args![points, index]).u32();
                    fn_006dad40(e, Ptr::new(request), Ptr::new(element));
                    let results = f.at(-0xbc);
                    fn_006dae60(e, Ptr::new(results));
                    fn_006d62e0(e, Ptr::new(request), Ptr::new(results));
                    fn_006dae90(e, Ptr::new(results));
                    fn_006dad70(e, Ptr::new(request));
                }
                2 => {
                    let worldspace = e.call(LOCATION_GET_WORLDSPACE, &args![location]).u32();
                    let element = e.call(POINT_ARRAY_ELEMENT, &args![points, index]).u32();
                    let target = f.at(-0x1d4);
                    e.call(
                        LOCATION_CONSTRUCT_FROM_POSITION_AND_WORLDSPACE,
                        &args![target, element, worldspace],
                    );
                    let request = f.at(-0x2cc);
                    e.call(REQUEST_CONSTRUCT, &args![request]);
                    fn_006d3ac0(e, Ptr::new(request), Ptr::new(location));
                    fn_006d1bc0(e, Ptr::new(request), Ptr::new(target));
                    let solution = f.at(-0x21c);
                    e.call(SOLUTION_CONSTRUCT, &args![solution]);
                    fn_006d0b10(e, Ptr::new(request), Ptr::new(solution));
                    e.call(SOLUTION_DESTRUCT, &args![solution]);
                    e.call(REQUEST_COPY_DESTRUCT, &args![request]);
                    e.call(LOCATION_DESTRUCT, &args![target]);
                }
                _ => {}
            }
            index += 1;
        }
        e.call(POINT_ARRAY_DESTRUCT, &args![points]);
        e.call(LOCATION_DESTRUCT, &args![location]);
        e.call(NAV_HOLDER_RELEASE, &args![holder]);
        0.0
    })
}

// Translated from 006dad40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the three `float`s at `point` into the vector at +0xB0 of the
/// request (the cover request's `fn_006da7c0` use of it sets the point the
/// cover search is about).
pub fn fn_006dad40(e: &mut Engine, this: Ptr, point: Ptr) {
    copy_words(e, this.addr() + 0xb0, point.addr(), 3);
}

// Translated from 006dad70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor that runs the request's base destructor (`006e2620`).
pub fn fn_006dad70(e: &mut Engine, this: Ptr) {
    e.call(REQUEST_COPY_DESTRUCT, &args![this]);
}

// Translated from 006dad90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of element `index` of a path array: `006b3160`.
pub fn fn_006dad90(e: &mut Engine, this: Ptr, index: u32) -> Ptr {
    e.call(NODE_ARRAY_ELEMENT, &args![this, index]).ptr()
}

// Translated from 006dadb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPointer::operator=` for the `PathingAvoidNodeArray` slot: when
/// `array` differs from the pointer held, releases the old object (the
/// reference counter word at +0x10 of the pointee) and takes a reference
/// on the new one. Returns `this`.
pub fn fn_006dadb0(e: &mut Engine, this: Ptr, array: u32) -> Ptr {
    let old = e.mem.u32(this.addr());
    if old != array {
        if old != 0 {
            e.call(REFERENCE_RELEASE, &args![old + 0x10]);
        }
        e.mem.set_u32(this.addr(), array);
        if array != 0 {
            e.call(REFERENCE_ADD, &args![array + 0x10]);
        }
    }
    this
}

// Translated from 006dae00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray::Add` for 0x10-byte entries: reserves a slot
/// (`006f31f0`), constructs the entry (`006dbcf0`), copies the four words
/// of `item` into it and returns its index.
pub fn fn_006dae00(e: &mut Engine, this: Ptr, item: Ptr) -> u32 {
    let index = e.call(DOOR_ARRAY_ADD_SLOT, &args![this]).u32();
    let buffer = e.mem.u32(this.addr() + 4);
    e.call(
        DOOR_ELEMENTS_CONSTRUCT,
        &args![this, (index << 4).wrapping_add(buffer), 1u32],
    );
    let buffer = e.mem.u32(this.addr() + 4);
    copy_words(e, buffer.wrapping_add(index << 4), item.addr(), 4);
    index
}

// Translated from 006dae60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of `BSSimpleArray<PathingCoverLocation,1024>`: stores
/// the vtable and initializes the empty array (`006dc070`). Returns
/// `this`.
pub fn fn_006dae60(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), COVER_LOCATION_ARRAY_VTABLE);
    e.call(COVER_LOCATION_ARRAY_INIT, &args![this, 0u32, 0u32]);
    this
}

// Translated from 006dae90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of `BSSimpleArray<PathingCoverLocation,1024>`: stores the
/// vtable and clears the array with its buffer (`006dbde0` with 1). The
/// Xbox PDB name of this address is
/// `BSSimpleArray<PathingCoverLocation,1024>::Clear`, a name the linker
/// folded onto this body.
pub fn fn_006dae90(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), COVER_LOCATION_ARRAY_VTABLE);
    e.call(COVER_LOCATION_ARRAY_CLEAR, &args![this, 1u32]);
}

// Translated from 006daeb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `SetReservedSize` of `BSSimpleArray<PathingCoverLocation,1024>` (same body
/// as `BSSimpleArray<Actor *,1024>::SetReservedSize`, Xbox PDB): moves the
/// array to a buffer of `capacity` entries (0x54 bytes each), after
/// destroying the entries that no longer fit and cutting the size down.
pub fn fn_006daeb0(e: &mut Engine, this: Ptr, capacity: u32) {
    let this = this.addr();
    if capacity == e.mem.u32(this + 0xc) {
        return;
    }
    let size = e.mem.u32(this + 8);
    if capacity < size {
        let buffer = e.mem.u32(this + 4);
        e.call(
            COVER_LOCATIONS_DESTROY_RANGE,
            &args![
                this,
                capacity.wrapping_mul(0x54).wrapping_add(buffer),
                size - capacity
            ],
        );
        e.mem.set_u32(this + 8, capacity);
    }
    let size = e.mem.u32(this + 8);
    e.call(COVER_LOCATIONS_REALLOCATE, &args![this, capacity, size]);
    e.mem.set_u32(this + 0xc, capacity);
}

// Translated from 006daf20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The insertion method of `BSSimpleArray<PathingCoverLocation,1024>`: puts a copy of
/// `item` at `index`, shifting the later entries up by one. At the end of
/// the array it appends (`006dbd90`). When the array is full it moves to
/// a bigger buffer (the capacity from `009a3910`, allocated through
/// virtual slot 4) while doing so.
pub fn fn_006daf20(e: &mut Engine, this: Ptr, index: u32, item: Ptr) {
    let this = this.addr();
    if index == e.mem.u32(this + 8) {
        e.call(COVER_LOCATIONS_ADD, &args![this, item]);
        return;
    }
    if e.call(ARRAY_IS_FULL, &args![this]).bool() {
        let new_capacity = e.call(ARRAY_GROWN_CAPACITY, &args![this]).u32();
        let new_buffer = e.vcall(this, 4, &args![new_capacity]).u32();
        let old_buffer = e.mem.u32(this + 4);
        e.call(
            COVER_LOCATIONS_COPY,
            &args![this, new_buffer, old_buffer, index],
        );
        let slot = index.wrapping_mul(0x54).wrapping_add(new_buffer);
        e.call(COVER_LOCATIONS_CONSTRUCT, &args![this, slot, 1u32]);
        let size = e.mem.u32(this + 8);
        let old_buffer = e.mem.u32(this + 4);
        e.call(
            COVER_LOCATIONS_COPY,
            &args![
                this,
                index
                    .wrapping_mul(0x54)
                    .wrapping_add(new_buffer)
                    .wrapping_add(0x54),
                index.wrapping_mul(0x54).wrapping_add(old_buffer),
                size - index
            ],
        );
        e.call(ARRAY_FREE_BUFFER, &args![this]);
        e.mem.set_u32(this + 4, new_buffer);
        e.mem.set_u32(this + 0xc, new_capacity);
    } else {
        let size = e.mem.u32(this + 8);
        let buffer = e.mem.u32(this + 4);
        e.call(
            COVER_LOCATIONS_COPY,
            &args![
                this,
                index
                    .wrapping_mul(0x54)
                    .wrapping_add(buffer)
                    .wrapping_add(0x54),
                index.wrapping_mul(0x54).wrapping_add(buffer),
                size - index
            ],
        );
        let buffer = e.mem.u32(this + 4);
        e.call(
            COVER_LOCATIONS_CONSTRUCT,
            &args![this, index.wrapping_mul(0x54).wrapping_add(buffer), 1u32],
        );
    }
    let size = e.mem.u32(this + 8);
    e.mem.set_u32(this + 8, size + 1);
    let buffer = e.mem.u32(this + 4);
    e.call(
        COVER_LOCATION_ASSIGN,
        &args![index.wrapping_mul(0x54).wrapping_add(buffer), item],
    );
}

// Translated from 006db060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<PathingCoverLocation,1024>::_Allocate` (Xbox PDB): the
/// memory for `count` entries of 0x54 bytes.
pub fn fn_006db060(e: &mut Engine, _this: Ptr, count: u32) -> Ptr {
    e.call(MEMORY_ALLOCATE, &args![count.wrapping_mul(0x54)])
        .ptr()
}

// Translated from 006db090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees a block of cover location entries: the deallocation of
/// `BSSimpleArray<PathingCoverLocation,1024>`.
pub fn fn_006db090(e: &mut Engine, _this: Ptr, block: Ptr) {
    e.call(MEMORY_FREE, &args![block]);
}

// Translated from 006db0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<PathingCoverLocation,1024>::_Reallocate` (Xbox PDB): the
/// resize of `block` to `count` entries of 0x54 bytes (`0042f5d0`).
pub fn fn_006db0b0(e: &mut Engine, _this: Ptr, block: Ptr, count: u32) -> u32 {
    e.call(REALLOCATE_BLOCK, &args![block, count.wrapping_mul(0x54)])
        .u32()
}

// Translated from 006db0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SetReservedSize` of the array of 0x14-byte entries (the path array):
/// like `fn_006daeb0`, with `0072ba80` destroying the cut entries and
/// `009a3c50` moving to the new buffer.
pub fn fn_006db0d0(e: &mut Engine, this: Ptr, capacity: u32) {
    let this = this.addr();
    if capacity == e.mem.u32(this + 0xc) {
        return;
    }
    let size = e.mem.u32(this + 8);
    if capacity < size {
        let buffer = e.mem.u32(this + 4);
        e.call(
            PATH_ELEMENTS_DESTROY_RANGE,
            &args![
                this,
                capacity.wrapping_mul(0x14).wrapping_add(buffer),
                size - capacity
            ],
        );
        e.mem.set_u32(this + 8, capacity);
    }
    let size = e.mem.u32(this + 8);
    e.call(PATH_ELEMENTS_REALLOCATE, &args![this, capacity, size]);
    e.mem.set_u32(this + 0xc, capacity);
}

// Translated from 006db140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes `count` entries (0x14 bytes each) from `index` on from the path
/// array. Removing at least as many as there are empties the array
/// (`008454f0`, freeing the buffer when `shrink` is set). With `shrink` and
/// an array that has become nearly empty (`006f3170`) the remaining
/// entries move to a smaller buffer (`00869600` gives its capacity);
/// otherwise they move down in place. The size drops by `count`.
///
/// The shrinking branch moves `size - count` entries from behind the
/// removed range, which is the count the game computes (it is more than
/// remain when `index` is not 0).
pub fn fn_006db140(e: &mut Engine, this: Ptr, index: u32, count: u32, shrink: bool) {
    let this = this.addr();
    let size = e.mem.u32(this + 8);
    if count >= size {
        e.call(ARRAY_CLEAR, &args![this, shrink as u32]);
        return;
    }
    if shrink && e.call(ARRAY_MAY_SHRINK, &args![this]).bool() {
        let new_capacity = e.call(ARRAY_SHRUNK_CAPACITY, &args![this]).u32();
        let new_buffer = e.vcall(this, 4, &args![new_capacity]).u32();
        let old_buffer = e.mem.u32(this + 4);
        e.call(
            PATH_ELEMENTS_COPY,
            &args![this, new_buffer, old_buffer, index],
        );
        let buffer = e.mem.u32(this + 4);
        e.call(
            PATH_ELEMENTS_DESTROY_RANGE,
            &args![this, index.wrapping_mul(0x14).wrapping_add(buffer), count],
        );
        let size = e.mem.u32(this + 8);
        let buffer = e.mem.u32(this + 4);
        e.call(
            PATH_ELEMENTS_COPY,
            &args![
                this,
                index.wrapping_mul(0x14).wrapping_add(new_buffer),
                buffer
                    .wrapping_add(index.wrapping_mul(0x14))
                    .wrapping_add(count.wrapping_mul(0x14)),
                size - count
            ],
        );
        e.call(ARRAY_FREE_BUFFER, &args![this]);
        e.mem.set_u32(this + 4, new_buffer);
        e.mem.set_u32(this + 0xc, new_capacity);
    } else {
        let buffer = e.mem.u32(this + 4);
        e.call(
            PATH_ELEMENTS_DESTROY_RANGE,
            &args![this, index.wrapping_mul(0x14).wrapping_add(buffer), count],
        );
        let size = e.mem.u32(this + 8);
        let buffer = e.mem.u32(this + 4);
        e.call(
            PATH_ELEMENTS_COPY,
            &args![
                this,
                index.wrapping_mul(0x14).wrapping_add(buffer),
                buffer
                    .wrapping_add(index.wrapping_mul(0x14))
                    .wrapping_add(count.wrapping_mul(0x14)),
                size - index - count
            ],
        );
    }
    let size = e.mem.u32(this + 8);
    e.mem.set_u32(this + 8, size - count);
}

// Translated from 006db290 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the scrap-heap array: the base constructor
/// (`006dc100`), the vtable, the allocator (`allocator`, else the thread's
/// scrap heap) at +0x10, and the array initialization with `capacity` and
/// `size`. Returns `this`.
pub fn fn_006db290(e: &mut Engine, this: Ptr, capacity: u32, size: u32, allocator: u32) -> Ptr {
    e.call(SCRAP_ARRAY_BASE_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), SCRAP_ARRAY_VTABLE);
    let allocator = if allocator != 0 {
        allocator
    } else {
        let manager = e.call(MEMORY_MANAGER_OBJECT, &args![]).u32();
        e.call(GET_THREAD_SCRAP_HEAP, &args![manager]).u32()
    };
    e.mem.set_u32(this.addr() + 0x10, allocator);
    e.call(ARRAY_INIT, &args![this, capacity, size]);
    this
}

// Translated from 006db350 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the scrap-heap array: the vtable, the clearing of the
/// array with its buffer (`008454f0` with 1) and the base destructor
/// (`006db330`).
pub fn fn_006db350(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), SCRAP_ARRAY_VTABLE);
    e.call(ARRAY_CLEAR, &args![this, 1u32]);
    e.call(SCRAP_ARRAY_BASE_DESTRUCT, &args![this]);
}

// Translated from 006db3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the candidate array: the vtable and the base
/// initialization (`006dc1d0` with 0, 0). Returns `this`.
pub fn fn_006db3b0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), CANDIDATE_ARRAY_VTABLE);
    e.call(CANDIDATE_ARRAY_INIT, &args![this, 0u32, 0u32]);
    this
}

// Translated from 006db3e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the candidate array (`BSSimpleArray<
/// FastNavMeshEdgeLocationProxy_1024>` by the name of `006dbb10`): the
/// vtable, then the clearing of the array with its buffer (`008454f0` with
/// 1).
pub fn fn_006db3e0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), CANDIDATE_ARRAY_VTABLE);
    e.call(ARRAY_CLEAR, &args![this, 1u32]);
}

// Translated from 006db400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends a copy of `item` to the candidate array (entries of 0x20
/// bytes): reserves a slot (`006b4060`), constructs the entry (`006dc130`),
/// assigns `item` to it (`fn_006dbb40`) and returns the entry's index.
pub fn fn_006db400(e: &mut Engine, this: Ptr, item: Ptr) -> u32 {
    let index = e.call(EDGE_PROXY_ARRAY_ADD_SLOT, &args![this]).u32();
    let buffer = e.mem.u32(this.addr() + 4);
    e.call(
        EDGE_PROXY_ARRAY_CONSTRUCT,
        &args![this, index.wrapping_mul(0x20).wrapping_add(buffer), 1u32],
    );
    let buffer = e.mem.u32(this.addr() + 4);
    fn_006dbb40(
        e,
        Ptr::new(index.wrapping_mul(0x20).wrapping_add(buffer)),
        item,
    );
    index
}

// Translated from 006db450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends a copy of `item` to the array of 0x18-byte path points:
/// reserves a slot (`006dc260`), constructs the entry (`006d05e0`),
/// assigns `item` to it (`006cf900`) and returns the entry's index.
pub fn fn_006db450(e: &mut Engine, this: Ptr, item: Ptr) -> u32 {
    let index = e.call(PATH_POINT_ARRAY_ADD_SLOT, &args![this]).u32();
    let buffer = e.mem.u32(this.addr() + 4);
    e.call(
        PATH_POINT_ENTRY_CONSTRUCT,
        &args![this, index.wrapping_mul(0x18).wrapping_add(buffer), 1u32],
    );
    let buffer = e.mem.u32(this.addr() + 4);
    e.call(
        PATH_POINT_ASSIGN,
        &args![index.wrapping_mul(0x18).wrapping_add(buffer), item],
    );
    index
}

// Translated from 006db4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the array of 0xC-byte entries (`BSSimpleArray<
/// NavmeshTriFan_1024>` by the name of `006dbb80`): the vtable and the base
/// initialization (`0069ba70` with 0, 0). Returns `this`.
pub fn fn_006db4a0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), TRI_FAN_ARRAY_VTABLE);
    e.call(TRI_FAN_ARRAY_INIT, &args![this, 0u32, 0u32]);
    this
}

// Translated from 006db4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the array of 0xC-byte entries: the vtable, then the
/// clearing of the array with its buffer (`008454f0` with 1).
pub fn fn_006db4d0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), TRI_FAN_ARRAY_VTABLE);
    e.call(ARRAY_CLEAR, &args![this, 1u32]);
}

// Translated from 006db4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends a copy of `item` to the array of 0xC-byte entries: reserves a
/// slot (`00978bc0`), constructs the entry (`0069b9d0`), assigns `item` to
/// it (`fn_006dbbb0`) and returns the entry's index.
pub fn fn_006db4f0(e: &mut Engine, this: Ptr, item: Ptr) -> u32 {
    let index = e.call(ARRAY_ADD_SLOT, &args![this]).u32();
    let buffer = e.mem.u32(this.addr() + 4);
    e.call(
        TRI_FAN_ARRAY_CONSTRUCT,
        &args![this, index.wrapping_mul(0xc).wrapping_add(buffer), 1u32],
    );
    let buffer = e.mem.u32(this.addr() + 4);
    fn_006dbbb0(
        e,
        Ptr::new(index.wrapping_mul(0xc).wrapping_add(buffer)),
        item,
    );
    index
}

// Translated from 006db540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Inserts the float at `item` into the float array at `index`. At the end
/// of the array it is appended (`006dc320`). Otherwise, when the array is
/// full (`00438b90`) the entries move to a buffer of the grown capacity
/// (`009a3910`, allocated through virtual slot 4): the ones before `index`,
/// then the ones from `index` on, one place up; the old buffer is freed
/// (`006a8500`). When it is not full the entries from `index` on move up
/// one place in place. The size grows by one and the value is stored.
pub fn fn_006db540(e: &mut Engine, this: Ptr, index: u32, item: Ptr) {
    let this = this.addr();
    if index == e.mem.u32(this + 8) {
        e.call(FLOAT_ARRAY_APPEND, &args![this, item]);
        return;
    }
    if e.call(ARRAY_IS_FULL, &args![this]).bool() {
        let capacity = e.call(ARRAY_GROWN_CAPACITY, &args![this]).u32();
        let new_buffer = e.vcall(this, 4, &args![capacity]).u32();
        let old_buffer = e.mem.u32(this + 4);
        e.call(
            FLOAT_ARRAY_COPY,
            &args![this, new_buffer, old_buffer, index],
        );
        e.call(
            FLOAT_ARRAY_CONSTRUCT,
            &args![this, new_buffer.wrapping_add(index.wrapping_mul(4)), 1u32],
        );
        let size = e.mem.u32(this + 8);
        let old_buffer = e.mem.u32(this + 4);
        e.call(
            FLOAT_ARRAY_COPY,
            &args![
                this,
                new_buffer
                    .wrapping_add(index.wrapping_mul(4))
                    .wrapping_add(4),
                old_buffer.wrapping_add(index.wrapping_mul(4)),
                size.wrapping_sub(index)
            ],
        );
        e.call(ARRAY_FREE_BUFFER, &args![this]);
        e.mem.set_u32(this + 4, new_buffer);
        e.mem.set_u32(this + 0xc, capacity);
    } else {
        let size = e.mem.u32(this + 8);
        let buffer = e.mem.u32(this + 4);
        e.call(
            FLOAT_ARRAY_COPY,
            &args![
                this,
                buffer.wrapping_add(index.wrapping_mul(4)).wrapping_add(4),
                buffer.wrapping_add(index.wrapping_mul(4)),
                size.wrapping_sub(index)
            ],
        );
        let buffer = e.mem.u32(this + 4);
        e.call(
            FLOAT_ARRAY_CONSTRUCT,
            &args![this, buffer.wrapping_add(index.wrapping_mul(4)), 1u32],
        );
    }
    let size = e.mem.u32(this + 8);
    e.mem.set_u32(this + 8, size.wrapping_add(1));
    let buffer = e.mem.u32(this + 4);
    // The game moves the float through the x87 stack; the bits are copied.
    let value = e.mem.u32(item.addr());
    e.mem
        .set_u32(buffer.wrapping_add(index.wrapping_mul(4)), value);
}

// Translated from 006db7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the float scrap-heap array: the base constructor
/// (`0042f800`), the vtable, the allocator (`allocator`, else the thread's
/// scrap heap) at +0x10, and the array initialization (`0042fcb0`) with
/// `capacity` and `size`. Returns `this`.
pub fn fn_006db7a0(e: &mut Engine, this: Ptr, capacity: u32, size: u32, allocator: u32) -> Ptr {
    e.call(FLOAT_SCRAP_ARRAY_BASE_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), FLOAT_SCRAP_ARRAY_VTABLE);
    let allocator = if allocator != 0 {
        allocator
    } else {
        let manager = e.call(MEMORY_MANAGER_OBJECT, &args![]).u32();
        e.call(GET_THREAD_SCRAP_HEAP, &args![manager]).u32()
    };
    e.mem.set_u32(this.addr() + 0x10, allocator);
    e.call(FLOAT_ARRAY_INIT, &args![this, capacity, size]);
    this
}

// Translated from 006db890 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the crossed-triangle scrap-heap array: like
/// `fn_006db7a0` with the base constructor `006dc370`, its own vtable and
/// the initialization `006dc440`. Returns `this`.
pub fn fn_006db890(e: &mut Engine, this: Ptr, capacity: u32, size: u32, allocator: u32) -> Ptr {
    e.call(CROSSED_TRIANGLE_SCRAP_ARRAY_BASE_CONSTRUCT, &args![this]);
    e.mem
        .set_u32(this.addr(), CROSSED_TRIANGLE_SCRAP_ARRAY_VTABLE);
    let allocator = if allocator != 0 {
        allocator
    } else {
        let manager = e.call(MEMORY_MANAGER_OBJECT, &args![]).u32();
        e.call(GET_THREAD_SCRAP_HEAP, &args![manager]).u32()
    };
    e.mem.set_u32(this.addr() + 0x10, allocator);
    e.call(CROSSED_TRIANGLE_ARRAY_INIT, &args![this, capacity, size]);
    this
}

// Translated from 006db950 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the crossed-triangle scrap-heap array: the vtable,
/// the clearing of the array with its buffer (`008454f0` with 1) and the
/// base destructor (`006db930`).
pub fn fn_006db950(e: &mut Engine, this: Ptr) {
    e.mem
        .set_u32(this.addr(), CROSSED_TRIANGLE_SCRAP_ARRAY_VTABLE);
    e.call(ARRAY_CLEAR, &args![this, 1u32]);
    e.call(CROSSED_TRIANGLE_SCRAP_ARRAY_BASE_DESTRUCT, &args![this]);
}

// Translated from 006db9e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the triangle handle array (`BSSimpleArray<
/// NavMeshTriHandle_1024>` by the name of `006dbcc0`): the vtable and the
/// array initialization (`006dc440` with 0, 0). Returns `this`.
pub fn fn_006db9e0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), TRI_HANDLE_ARRAY_VTABLE);
    e.call(CROSSED_TRIANGLE_ARRAY_INIT, &args![this, 0u32, 0u32]);
    this
}

// Translated from 006dba10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the triangle handle array: the vtable, then the
/// clearing of the array with its buffer (`008454f0` with 1).
pub fn fn_006dba10(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), TRI_HANDLE_ARRAY_VTABLE);
    e.call(ARRAY_CLEAR, &args![this, 1u32]);
}

// Translated from 006dba80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<PathingCoverLocation_1024>::_scalar_deleting_destructor_`
/// (Xbox PDB): runs the destructor `fn_006dae90` and, when bit 0 of
/// `flags` is set, frees the object (`00401030`). Returns `this`.
pub fn fn_006dba80(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006dae90(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![this]);
    }
    this
}

// Translated from 006dbab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<PathingNode_P_1024>::_scalar_deleting_destructor_` (Xbox
/// PDB): runs the destructor `006db330` and, when bit 0 of `flags` is set,
/// frees the object. Returns `this`.
pub fn fn_006dbab0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(SCRAP_ARRAY_BASE_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![this]);
    }
    this
}

// Translated from 006dbae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapArray<PathingNode_P_1024>::_scalar_deleting_destructor_` (Xbox
/// PDB): runs the destructor `fn_006db350` and, when bit 0 of `flags` is
/// set, frees the object. Returns `this`.
pub fn fn_006dbae0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006db350(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![this]);
    }
    this
}

// Translated from 006dbb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<FastNavMeshEdgeLocationProxy_1024>::
/// _scalar_deleting_destructor_` (Xbox PDB): runs the destructor
/// `fn_006db3e0` and, when bit 0 of `flags` is set, frees the object.
/// Returns `this`.
pub fn fn_006dbb10(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006db3e0(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![this]);
    }
    this
}

// Translated from 006dbb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assignment of the 0x20-byte candidate entry: the base copy (`006cf900`,
/// a path point) and the words at +0x18 and +0x1C. Returns `this`.
pub fn fn_006dbb40(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    e.call(PATH_POINT_ASSIGN, &args![this, other]);
    // The game moves both words through the x87 stack; the bits are copied.
    let first = e.mem.u32(other.addr() + 0x18);
    e.mem.set_u32(this.addr() + 0x18, first);
    let second = e.mem.u32(other.addr() + 0x1c);
    e.mem.set_u32(this.addr() + 0x1c, second);
    this
}

// Translated from 006dbb80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NavmeshTriFan_1024>::_scalar_deleting_destructor_` (Xbox
/// PDB): runs the destructor `fn_006db4d0` and, when bit 0 of `flags` is
/// set, frees the object. Returns `this`.
pub fn fn_006dbb80(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006db4d0(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![this]);
    }
    this
}

// Translated from 006dbbb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assignment of the 0xC-byte triangle fan entry: the navmesh and triangle
/// copy (`0069a690`, a triangle reference) and the three bytes at +8.
/// Returns `this`.
pub fn fn_006dbbb0(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    e.call(TRIANGLE_REFERENCE_ASSIGN, &args![this, other]);
    for i in 0..3 {
        let byte = e.mem.u8(other.addr() + 8 + i);
        e.mem.set_u8(this.addr() + 8 + i, byte);
    }
    this
}

// Translated from 006dbc00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<float_1024>::_scalar_deleting_destructor_` (Xbox PDB):
/// runs the destructor `006db720` and, when bit 0 of `flags` is set, frees
/// the object. Returns `this`.
pub fn fn_006dbc00(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(FLOAT_SIMPLE_ARRAY_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![this]);
    }
    this
}

// Translated from 006dbc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapArray<float_1024>::_scalar_deleting_destructor_` (Xbox PDB):
/// runs the destructor `006db740` and, when bit 0 of `flags` is set, frees
/// the object. Returns `this`.
pub fn fn_006dbc30(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(KEY_ARRAY_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![this]);
    }
    this
}

// Translated from 006dbc60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<CrossedTriangle_1024>::_scalar_deleting_destructor_`
/// (Xbox PDB): runs the destructor `006db930` and, when bit 0 of `flags` is
/// set, frees the object. Returns `this`.
pub fn fn_006dbc60(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(CROSSED_TRIANGLE_SCRAP_ARRAY_BASE_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![this]);
    }
    this
}

// Translated from 006dbc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapArray<CrossedTriangle_1024>::_scalar_deleting_destructor_` (Xbox
/// PDB): runs the destructor `fn_006db950` and, when bit 0 of `flags` is
/// set, frees the object. Returns `this`.
pub fn fn_006dbc90(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006db950(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![this]);
    }
    this
}

// Translated from 006dbcc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NavMeshTriHandle_1024>::_scalar_deleting_destructor_`
/// (Xbox PDB): runs the destructor `fn_006dba10` and, when bit 0 of `flags`
/// is set, frees the object. Returns `this`.
pub fn fn_006dbcc0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006dba10(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![this]);
    }
    this
}

// Translated from 006dbd90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends a copy of `item` to the cover location array (entries of 0x54
/// bytes): reserves a slot (`006dc500`), constructs the entry
/// (`006dbe40`), assigns `item` to it (`006e4430`) and returns the entry's
/// index.
pub fn fn_006dbd90(e: &mut Engine, this: Ptr, item: Ptr) -> u32 {
    let index = e.call(COVER_LOCATION_ARRAY_ADD_SLOT, &args![this]).u32();
    let buffer = e.mem.u32(this.addr() + 4);
    e.call(
        COVER_LOCATIONS_CONSTRUCT,
        &args![this, index.wrapping_mul(0x54).wrapping_add(buffer), 1u32],
    );
    let buffer = e.mem.u32(this.addr() + 4);
    e.call(
        COVER_LOCATION_ASSIGN,
        &args![index.wrapping_mul(0x54).wrapping_add(buffer), item],
    );
    index
}

// Translated from 006dc070 (decompiled, FalloutNV.exe 1.4.0.525)
/// The initialization of the cover location array: empty buffer, size and
/// capacity; the capacity is at least `size`. A buffer is allocated through
/// virtual slot 4 when the capacity is not 0, and `size` entries are
/// constructed in it (`006dbe40`).
pub fn fn_006dc070(e: &mut Engine, this: Ptr, capacity: u32, size: u32) {
    let this = this.addr();
    e.mem.set_u32(this + 4, 0);
    e.mem.set_u32(this + 8, 0);
    e.mem.set_u32(this + 0xc, 0);
    let capacity = capacity.max(size);
    if capacity != 0 {
        let buffer = e.vcall(this, 4, &args![capacity]).u32();
        e.mem.set_u32(this + 4, buffer);
        e.mem.set_u32(this + 0xc, capacity);
    }
    if size != 0 {
        let buffer = e.mem.u32(this + 4);
        e.call(COVER_LOCATIONS_CONSTRUCT, &args![this, buffer, size]);
        e.mem.set_u32(this + 8, size);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0049f210,
            bs_simple_array_parent_space_node_add(Ptr<crate::types::BSSimpleArray>, Ptr) -> i32
        ),
        entry!(0x006d0870, pathing_init() -> bool),
        entry!(
            0x006d0900,
            fn_006d0900(Ptr<PathingRequest>, Ptr<PathingSolution>) -> bool
        ),
        entry!(
            0x006d0b10,
            fn_006d0b10(Ptr<PathingRequest>, Ptr<PathingSolution>) -> bool
        ),
        entry!(0x006d0b80, fn_006d0b80(Ptr)),
        entry!(
            0x006d0be0,
            fn_006d0be0(Ptr<PathingRequest>, Ptr<PathingSolution>) -> bool
        ),
        entry!(
            0x006d12a0,
            fn_006d12a0(Ptr<PathingRequest>, Ptr<PathingSolution>) -> bool
        ),
        entry!(
            0x006d1520,
            fn_006d1520(Ptr<PathingSolution>, Ptr, u32, u32) -> u32
        ),
        entry!(0x006d1600, fn_006d1600() -> f32),
        entry!(0x006d1620, fn_006d1620(Ptr) -> f32),
        entry!(
            0x006d1660,
            fn_006d1660(Ptr<PathingRequest>, Ptr<PathingSolution>) -> bool
        ),
        entry!(0x006d1bc0, fn_006d1bc0(Ptr, Ptr)),
        entry!(0x006d1be0, fn_006d1be0(Ptr) -> u32),
        entry!(0x006d1c00, fn_006d1c00(Ptr) -> u32),
        entry!(0x006d1c20, fn_006d1c20(Ptr) -> u32),
        entry!(0x006d1c40, fn_006d1c40(Ptr) -> u32),
        entry!(
            0x006d1c60,
            fn_006d1c60(Ptr<PathingRequest>, Ptr<PathingSolution>) -> bool
        ),
        entry!(
            0x006d1f80,
            fn_006d1f80(Ptr<PathingRequest>, Ptr<PathingSolution>) -> bool
        ),
        entry!(
            0x006d23d0,
            fn_006d23d0(Ptr<PathingRequest>, Ptr<PathingSolution>) -> bool
        ),
        entry!(0x006d2c00, fn_006d2c00(Ptr<PathingLocation>)),
        entry!(0x006d2c20, fn_006d2c20(Ptr) -> f32),
        entry!(0x006d2c40, fn_006d2c40(Ptr) -> f32),
        entry!(0x006d2c60, fn_006d2c60(Ptr, f32)),
        entry!(
            0x006d2c80,
            fn_006d2c80(Ptr<PathingRequest>, Ptr<PathingSolution>) -> bool
        ),
        entry!(0x006d30b0, fn_006d30b0(Ptr, Ptr)),
        entry!(0x006d30e0, fn_006d30e0(Ptr<PathingRequestCoveredMove>, u32)),
        entry!(0x006d3100, fn_006d3100(Ptr<PathingRequestCoveredMove>, u32)),
        entry!(0x006d3120, fn_006d3120(Ptr<PathingRequestCoveredMove>)),
        entry!(
            0x006d3150,
            fn_006d3150(Ptr<CombatCoverLocation>) -> Ptr<CombatCoverLocation>
        ),
        entry!(
            0x006d3200,
            fn_006d3200(Ptr<PathingCoverLocation>) -> Ptr<PathingCoverLocation>
        ),
        entry!(
            0x006d3280,
            fn_006d3280(Ptr<CoverReservationInfo>) -> Ptr<CoverReservationInfo>
        ),
        entry!(0x006d32c0, fn_006d32c0(Ptr<PathingCoverLocation>)),
        entry!(
            0x006d32e0,
            fn_006d32e0(Ptr<CombatCoverSearchInfo>) -> Ptr<CombatCoverSearchInfo>
        ),
        entry!(
            0x006d3370,
            fn_006d3370(Ptr<CombatCoverLocation>, u32) -> Ptr<CombatCoverLocation>
        ),
        entry!(0x006d33a0, fn_006d33a0(Ptr<CombatCoverLocation>)),
        entry!(0x006d33c0, fn_006d33c0(Ptr<PathingRequest>, Ptr)),
        entry!(0x006d34d0, fn_006d34d0(Ptr<PathingRequest>, Ptr) -> bool),
        entry!(0x006d3780, pathing_find_safe_straight_line_path_point(Ptr, Ptr, f32, Ptr) -> bool),
        entry!(0x006d3ac0, fn_006d3ac0(Ptr, Ptr)),
        entry!(0x006d3ae0, fn_006d3ae0(Ptr<PathingRequest>, f32)),
        entry!(0x006d3b00, fn_006d3b00(Ptr<PathingRequest>, f32)),
        entry!(0x006d3b20, fn_006d3b20(Ptr<PathingRequest>)),
        entry!(0x006d3b40, fn_006d3b40(Ptr<PathingRequest>, Ptr) -> bool),
        entry!(0x006d3ff0, fn_006d3ff0(Ptr, Ptr)),
        entry!(0x006d4020, fn_006d4020(Ptr, Ptr) -> i32),
        entry!(0x006d4120, fn_006d4120(Ptr, u32, Ptr) -> u32),
        entry!(0x006d4550, fn_006d4550(Ptr) -> Ptr),
        entry!(0x006d4570, fn_006d4570(Ptr, f32, Ptr) -> bool),
        entry!(0x006d4ca0, fn_006d4ca0(Ptr, Ptr) -> bool),
        entry!(0x006d4ce0, fn_006d4ce0(Ptr, Ptr) -> Ptr),
        entry!(
            0x006d4d20,
            pathing_build_teleport_door_path(Ptr, Ptr, Ptr, u32, f32, u32, u32) -> bool
        ),
        entry!(0x006d4eb0, pathing_compute_teleport_door_path_length(Ptr, Ptr, f32, u32, u32) -> f32),
        entry!(0x006d4f70, pathing_build_low_path(Ptr, Ptr, Ptr, u32, u32) -> bool),
        entry!(0x006d5320, fn_006d5320(Ptr) -> Ptr),
        entry!(
            0x006d5340,
            fn_006d5340(Ptr<PathingRequest>, Ptr<PathingSolution>, u32, u32) -> u32
        ),
        entry!(
            0x006d53b0,
            fn_006d53b0(Ptr<PathingRequest>, Ptr<PathingSolution>) -> u32
        ),
        entry!(
            0x006d5420,
            fn_006d5420(Ptr<PathingRequest>, Ptr<PathingSolution>, u32) -> u32
        ),
        entry!(0x006d5490, fn_006d5490(Ptr, Ptr, Ptr, Ptr, Ptr) -> bool),
        entry!(0x006d61b0, fn_006d61b0(Ptr, Ptr)),
        entry!(0x006d61e0, fn_006d61e0(Ptr, u32)),
        entry!(0x006d6200, pathing_compute_low_path_distance(Ptr, Ptr, Ptr) -> bool),
        entry!(0x006d62c0, fn_006d62c0(Ptr, Ptr, Ptr) -> bool),
        entry!(0x006d62e0, fn_006d62e0(Ptr, Ptr) -> u32),
        entry!(0x006d6d60, fn_006d6d60(Ptr, Ptr) -> Ptr),
        entry!(0x006d6d90, fn_006d6d90(Ptr) -> f32),
        entry!(0x006d6db0, fn_006d6db0(Ptr) -> u8),
        entry!(0x006d6dd0, fn_006d6dd0(Ptr) -> u8),
        entry!(0x006d6df0, fn_006d6df0(Ptr) -> u8),
        entry!(0x006d6e10, fn_006d6e10(Ptr) -> u8),
        entry!(0x006d6e30, fn_006d6e30(Ptr) -> u8),
        entry!(
            0x006d6e50,
            fn_006d6e50(Ptr<PathingCoverLocation>, u32, u32, u16) -> Ptr<PathingCoverLocation>
        ),
        entry!(
            0x006d6ee0,
            fn_006d6ee0(Ptr<PathingCoverLocation>, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x006d6f40,
            pathing_get_potential_nav_mesh_info_for_location(u32) -> u32
        ),
        entry!(
            0x006d6f60,
            pathing_get_potential_nav_mesh_info_for_location_ov2(u32, u32) -> u32
        ),
        entry!(
            0x006d6f80,
            pathing_find_closest_point_on_navmesh(u32, u32, Ptr, Ptr) -> bool
        ),
        entry!(
            0x006d7110,
            pathing_find_point_on_nav_mesh(u32, u32, Ptr, f32, Ptr) -> bool
        ),
        entry!(
            0x006d7350,
            pathing_check_line_of_sight(Ptr, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(
            0x006d7490,
            pathing_check_line_of_sight_ov2(Ptr, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(
            0x006d74c0,
            fn_006d74c0(Ptr, Ptr, u32, u32, Ptr, Ptr) -> bool
        ),
        entry!(0x006d7a20, fn_006d7a20(Ptr, Ptr) -> bool),
        entry!(0x006d7a40, pathing_find_closest_reachable_location(Ptr, Ptr, Ptr, f32) -> Ptr),
        entry!(0x006d7ca0, fn_006d7ca0(Ptr, Ptr, Ptr) -> bool),
        entry!(0x006d7ee0, fn_006d7ee0(Ptr, u32) -> u8),
        entry!(0x006d7f00, fn_006d7f00(Ptr) -> bool),
        entry!(0x006d7f40, fn_006d7f40(Ptr, Ptr, bool, f32, f32) -> bool),
        entry!(0x006d8910, fn_006d8910(Ptr, u16) -> u8),
        entry!(0x006d8970, fn_006d8970(Ptr, u32, u8)),
        entry!(0x006d8990, fn_006d8990(Ptr) -> u32),
        entry!(0x006d89b0, fn_006d89b0(Ptr, u32) -> Ptr),
        entry!(0x006d89d0, fn_006d89d0(Ptr, u32, u8) -> Ptr),
        entry!(0x006d8a00, fn_006d8a00(Ptr, f32, Ptr) -> bool),
        entry!(0x006d8a20, fn_006d8a20(Ptr, f32, Ptr) -> bool),
        entry!(0x006d8a40, fn_006d8a40(Ptr, f32, Ptr, bool) -> bool),
        entry!(0x006d9100, pathing_get_navmesh_normal_for_location(Ptr, Ptr) -> bool),
        entry!(0x006d91d0, fn_006d91d0(Ptr) -> bool),
        entry!(0x006d9350, fn_006d9350(Ptr, u32, f32, Ptr) -> u32),
        entry!(0x006d93d0, fn_006d93d0(Ptr, Ptr, Ptr, Ptr) -> bool),
        entry!(0x006d9480, fn_006d9480(Ptr, f32) -> bool),
        entry!(0x006d97b0, pathing_get_nav_mesh_z_for_location(Ptr, Ptr, Ptr, Ptr) -> bool),
        entry!(0x006d9830, pathing_get_nav_mesh_z_for_location_ov2(Ptr, Ptr) -> bool),
        entry!(0x006d99a0, fn_006d99a0(Ptr, f32, Ptr, Ptr, f32) -> Ptr),
        entry!(0x006da420, fn_006da420(Ptr, f32, Ptr, f32) -> Ptr),
        entry!(0x006da7c0, pathing_profile_pathing(Ptr, Ptr, u32, u32) -> f32),
        entry!(0x006dad40, fn_006dad40(Ptr, Ptr)),
        entry!(0x006dad70, fn_006dad70(Ptr)),
        entry!(0x006dad90, fn_006dad90(Ptr, u32) -> Ptr),
        entry!(0x006dadb0, fn_006dadb0(Ptr, u32) -> Ptr),
        entry!(0x006dae00, fn_006dae00(Ptr, Ptr) -> u32),
        entry!(0x006dae60, fn_006dae60(Ptr) -> Ptr),
        entry!(0x006dae90, fn_006dae90(Ptr)),
        entry!(0x006daeb0, fn_006daeb0(Ptr, u32)),
        entry!(0x006daf20, fn_006daf20(Ptr, u32, Ptr)),
        entry!(0x006db060, fn_006db060(Ptr, u32) -> Ptr),
        entry!(0x006db090, fn_006db090(Ptr, Ptr)),
        entry!(0x006db0b0, fn_006db0b0(Ptr, Ptr, u32) -> u32),
        entry!(0x006db0d0, fn_006db0d0(Ptr, u32)),
        entry!(0x006db140, fn_006db140(Ptr, u32, u32, bool)),
        entry!(0x006db290, fn_006db290(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x006db350, fn_006db350(Ptr)),
        entry!(0x006db3b0, fn_006db3b0(Ptr) -> Ptr),
        entry!(0x006db3e0, fn_006db3e0(Ptr)),
        entry!(0x006db400, fn_006db400(Ptr, Ptr) -> u32),
        entry!(0x006db450, fn_006db450(Ptr, Ptr) -> u32),
        entry!(0x006db4a0, fn_006db4a0(Ptr) -> Ptr),
        entry!(0x006db4d0, fn_006db4d0(Ptr)),
        entry!(0x006db4f0, fn_006db4f0(Ptr, Ptr) -> u32),
        entry!(0x006db540, fn_006db540(Ptr, u32, Ptr)),
        entry!(0x006db7a0, fn_006db7a0(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x006db890, fn_006db890(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x006db950, fn_006db950(Ptr)),
        entry!(0x006db9e0, fn_006db9e0(Ptr) -> Ptr),
        entry!(0x006dba10, fn_006dba10(Ptr)),
        entry!(0x006dba80, fn_006dba80(Ptr, u32) -> Ptr),
        entry!(0x006dbab0, fn_006dbab0(Ptr, u32) -> Ptr),
        entry!(0x006dbae0, fn_006dbae0(Ptr, u32) -> Ptr),
        entry!(0x006dbb10, fn_006dbb10(Ptr, u32) -> Ptr),
        entry!(0x006dbb40, fn_006dbb40(Ptr, Ptr) -> Ptr),
        entry!(0x006dbb80, fn_006dbb80(Ptr, u32) -> Ptr),
        entry!(0x006dbbb0, fn_006dbbb0(Ptr, Ptr) -> Ptr),
        entry!(0x006dbc00, fn_006dbc00(Ptr, u32) -> Ptr),
        entry!(0x006dbc30, fn_006dbc30(Ptr, u32) -> Ptr),
        entry!(0x006dbc60, fn_006dbc60(Ptr, u32) -> Ptr),
        entry!(0x006dbc90, fn_006dbc90(Ptr, u32) -> Ptr),
        entry!(0x006dbcc0, fn_006dbcc0(Ptr, u32) -> Ptr),
        entry!(0x006dbd90, fn_006dbd90(Ptr, Ptr) -> u32),
        entry!(0x006dc070, fn_006dc070(Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// Every callee outside this file: each gets a double that returns zero
    /// unless a test installs a better one.
    const CALLEES: &[u32] = &[
        TRI_FAN_ARRAY_INIT,
        TRI_FAN_ARRAY_CONSTRUCT,
        EDGE_PROXY_ARRAY_ADD_SLOT,
        EDGE_PROXY_ARRAY_CONSTRUCT,
        PATH_POINT_ARRAY_ADD_SLOT,
        PATH_POINT_ENTRY_CONSTRUCT,
        FLOAT_ARRAY_APPEND,
        FLOAT_ARRAY_COPY,
        FLOAT_ARRAY_CONSTRUCT,
        FLOAT_ARRAY_INIT,
        CROSSED_TRIANGLE_SCRAP_ARRAY_BASE_CONSTRUCT,
        CROSSED_TRIANGLE_ARRAY_INIT,
        CROSSED_TRIANGLE_SCRAP_ARRAY_BASE_DESTRUCT,
        FLOAT_SIMPLE_ARRAY_DESTRUCT,
        COVER_LOCATION_ARRAY_ADD_SLOT,
        PATH_POINT_ASSIGN,
        TRIANGLE_REFERENCE_ASSIGN,
        KEY_ARRAY_DESTRUCT,
        ARRAY_ADD_SLOT,
        MEMORY_FREE,
        NAVMESH_LIST_CONSTRUCT,
        TIMER_SCOPE_CONSTRUCT,
        TIMER_SCOPE_DESTROY,
        MEMORY_ALLOCATE,
        MEMORY_FREE,
        VECTOR_CONSTRUCTOR_ITERATOR,
        SETTING_FLOAT_ADDRESS,
        FLOAT_MAX,
        FLOAT_ABSOLUTE_VALUE,
        POINT3_CONSTRUCT,
        POINT3_SCALE_IN_PLACE,
        POINT3_ADD,
        POINT3_SUBTRACT,
        POINT3_TIMES_SCALAR,
        POINT3_NORMALIZE,
        POINT3_SCALED_COPY,
        POINT3_LENGTH_SQUARED,
        POINT3_UNITIZE_GET_LENGTH,
        POINT3_ADD_ASSIGN,
        POINT3_DEFAULT_CONSTRUCTOR,
        ANGLE_FUNCTION_004E44B0,
        ANGLE_FUNCTION_004E44D0,
        ANGLE_FUNCTION_004E4470,
        ANGLE_FUNCTION_004E4490,
        ANGLE_FUNCTION_005B9E80,
        RANDOM_FLOAT,
        RANDOM_ANGLE,
        RANDOM_OBJECT,
        RANDOM_UNSIGNED,
        NAV_HOLDER_CONSTRUCT,
        NAV_HOLDER_RELEASE,
        NAV_HOLDER_GET,
        NI_POINTER_GET_FROM_FIELD,
        NAVMESH_GET_CENTER,
        NAVMESH_GET_INFO,
        NAVMESH_GET_VERTEX,
        NAVMESH_GET_TRIANGLE,
        TRIANGLE_GET_VERTEX_INDEX,
        TRIANGLE_RANDOM_POINT,
        REQUEST_ORIGIN,
        REQUEST_DESTINATION,
        REQUEST_ACTOR_RADIUS,
        REQUEST_AVOID_NODE_ARRAY,
        REQUEST_FIRST_TANGENT_USES_HEADING,
        REQUEST_INITIAL_PATH_HEADING,
        REQUEST_ABSOLUTE_MAXIMUM_DISTANCE,
        REQUEST_ABSOLUTE_MINIMUM_DISTANCE,
        REQUEST_SET_TARGET_RADIUS,
        REQUEST_SET_MINIMUM_DISTANCE,
        REQUEST_SET_MAXIMUM_DISTANCE,
        REQUEST_SET_FINISHED,
        REQUEST_COPY_CONSTRUCT,
        REQUEST_COPY_DESTRUCT,
        SAFE_STRAIGHT_LINE_CONSTRUCT,
        SAFE_STRAIGHT_LINE_DESTRUCT,
        SAFE_STRAIGHT_LINE_FIND,
        COVERED_MOVE_CONSTRUCT_COPY,
        COVERED_MOVE_DESTRUCT,
        COVERED_MOVE_COVER_POINT,
        COVERED_MOVE_CHECK,
        LOCATION_GET_CELL,
        LOCATION_GET_WORLDSPACE,
        LOCATION_COPY_POSITION,
        LOCATION_RESOLVE_CLOSEST,
        LOCATION_GET_NAVMESH_AND_TRIANGLE,
        LOCATION_RESOLVE_NAVMESH_INFO,
        LOCATION_HAS_FLAG_2,
        LOCATION_ASSIGN,
        LOCATION_COPY_CONSTRUCT,
        LOCATION_CONSTRUCT_AT_REFERENCE,
        LOCATION_CONSTRUCT_FROM_PARTS,
        LOCATION_CONSTRUCT_FROM_MESH_POINT,
        LOCATION_CONSTRUCT_FROM_POINT_AND_INFO,
        LOCATION_CONSTRUCT_FROM_POINT_AND_LOCATION,
        LOCATION_CONSTRUCT,
        LOCATION_DESTRUCT,
        CELL_GET_LAND_HEIGHT,
        REFERENCE_GET_SCALE,
        ARRAY_COUNT,
        ARRAY_IS_EMPTY,
        ARRAY_ADD_SLOT,
        ARRAY_CONSTRUCT_ELEMENTS,
        NODE_ARRAY_ELEMENT,
        NODE_ARRAY_A_CONSTRUCT,
        NODE_ARRAY_A_DESTRUCT,
        NODE_ARRAY_B_CONSTRUCT,
        NODE_ARRAY_B_DESTRUCT,
        NODE_ARRAY_C_CONSTRUCT,
        NODE_ARRAY_C_DESTRUCT,
        VIRTUAL_NODE_ARRAY_ADD_SLOT,
        VIRTUAL_NODE_ARRAY_ELEMENT,
        PLACEMENT_NEW,
        VIRTUAL_NODE_CONSTRUCT,
        VIRTUAL_NODE_SET_LOCATION,
        SOLUTION_ADD_VIRTUAL_NODE,
        SOLUTION_SET_INCOMPLETE,
        SOLUTION_NODE_COUNT,
        SOLUTION_VIRTUAL_NODE_COUNT,
        SOLUTION_NODE_AT,
        SOLUTION_VIRTUAL_NODE_AT,
        SOLUTION_CLEAR,
        SOLUTION_CONSTRUCT,
        SOLUTION_COPY_TO,
        SOLUTION_DESTRUCT,
        NODE_LOCATION,
        NODE_SET_POSITION,
        NODE_TO_POSITION,
        AVOID_ARRAY_COMPUTE_PUSH,
        BUILD_FIRST_TANGENT,
        FIND_CLOSEST_POINT_ON_NAVMESH,
        FIND_PATH_POINT,
        PATH_POINT_CONSTRUCT,
        BASE_SOLVER_CONSTRUCT,
        BASE_SOLVER_RUN,
        NAVMESH_LIST_CONSTRUCT,
        NAVMESH_LIST_DESTRUCT,
        BASE_SOLVER_MEMBER_DESTRUCT,
        NAVMESH_LIST_FILL,
        FLEE_SEARCH_CONSTRUCT,
        FLEE_SEARCH_DESTRUCT,
        FLEE_SEARCH_BUILD_NODE_PATH,
        PATH_BUILDER_CONSTRUCT,
        PATH_BUILDER_DESTRUCT,
        PATH_BUILDER_BUILD,
        CLOSE_POINT_SEARCH_CONSTRUCT,
        CLOSE_POINT_SEARCH_DESTRUCT,
        CLOSE_POINT_SEARCH_BUILD_NODE_PATH,
        PATH_SMOOTHER_CONSTRUCT,
        PATH_SMOOTHER_SMOOTH,
        PATH_SMOOTHER_DESTRUCT,
        LOS_SEARCH_CONSTRUCT,
        LOS_SEARCH_DESTRUCT,
        LOS_SEARCH_BUILD_NODE_PATH,
        LOS_SEARCH_PATH_FROM_NODE,
        LOS_MAP_FIND_TRIANGLE,
        HIDE_SEARCH_CONSTRUCT,
        HIDE_SEARCH_DESTRUCT,
        HIDE_SEARCH_BUILD_NODE_PATH,
        MAX_COST_SEARCH_CONSTRUCT,
        MAX_COST_SEARCH_DESTRUCT,
        MAX_COST_SEARCH_BUILD_NODE_PATH,
        MAX_COST_SEARCH_BEST_NODE,
        RANDOM_POINT_SEARCH_CONSTRUCT,
        RANDOM_POINT_SEARCH_DESTRUCT,
        RANDOM_POINT_SEARCH_BUILD_NODE_PATH,
        TIME_STAMP_CONSTRUCT,
        COVER_LOCATION_ARRAY_CONSTRUCT,
        REQUEST_BASE_DESTRUCT,
        LOCATION_HAS_NAVMESH_INFO,
        LOCATION_NAVMESH_INFO_FIELD,
        POINT_SEARCH_RUN,
        REQUEST_CONSTRUCT,
        REQUEST_COPY_TO,
        CLOSE_POINT_REQUEST_CONSTRUCT,
        SOLUTION_QUERY_006E7E70,
        NAV_HOLDER_COPY_CONSTRUCT,
        POINTER_ARRAY_ELEMENT,
        NAVMESH_TRIANGLE_COUNT,
        TRIANGLE_NEIGHBOUR,
        NAVMESH_EDGE_POINTS,
        POINT_SEGMENT_CLOSEST,
        POINT_SEGMENT_DISTANCE,
        POINT_PLANAR_LENGTH_SQUARED,
        POINT3_LENGTH,
        CANDIDATE_ARRAY_CONSTRUCT,
        CANDIDATE_ARRAY_DESTRUCT,
        CANDIDATE_ARRAY_ADD,
        CANDIDATE_ARRAY_ELEMENT,
        MINIMUM_UNSIGNED,
        CANDIDATE_TO_PATH_POINT,
        PATH_POINT_ARRAY_ADD,
        QSORT,
        PATH_POINT_ARRAY_CONSTRUCT,
        PATH_POINT_ARRAY_ELEMENT,
        PATH_POINT_ARRAY_DESTRUCT,
        PATH_POINT_ASSIGN,
        EDGE_ARRAY_CONSTRUCT,
        EDGE_ARRAY_DESTRUCT,
        EDGE_ARRAY_ADD,
        TRIANGLE_ARRAY_CONSTRUCT,
        TRIANGLE_ARRAY_DESTRUCT,
        TRIANGLE_ARRAY_ADD,
        ARRAY_ELEMENT_006A1440,
        TRIANGLE_RECORD_CONSTRUCT,
        TRIANGLE_REFERENCE_CONSTRUCT,
        TRIANGLE_REFERENCE_ASSIGN,
        EDGE_REFERENCE_CONSTRUCT,
        NAVMESH_GET_MATCHING_EDGE,
        EDGE_REFERENCE_ASSIGN,
        TELEPORT_PATH_CONSTRUCT,
        TELEPORT_PATH_DESTRUCT,
        TELEPORT_PATH_CLEAR,
        TELEPORT_PATH_COMPUTE_LENGTH,
        TELEPORT_PATH_RESERVE_STEPS,
        TELEPORT_PATH_RESERVE_DOORS,
        TELEPORT_SEARCH_CONSTRUCT,
        TELEPORT_SEARCH_BUILD_NODE_PATH,
        TELEPORT_SEARCH_DESTRUCT,
        LOW_PATH_SEARCH_CONSTRUCT,
        LOW_PATH_SEARCH_RUN,
        LOW_PATH_SEARCH_DESTRUCT,
        INFO_ARRAY_CONSTRUCT,
        INFO_ARRAY_DESTRUCT,
        INFO_ARRAY_ADD,
        REFERENCE_ARRAY_CONSTRUCT,
        REFERENCE_ARRAY_DESTRUCT,
        INFO_VALUE_A,
        INFO_VALUE_B,
        REFERENCE_GET_POSITION_OWNER,
        PATH_STEP_ADD,
        PATH_DOOR_ADD,
        NEARBY_NAVMESHES_FILL,
        COVER_MAX_RESULTS,
        COVER_RESULTS_RESERVE,
        COVER_RESULTS_INSERT,
        KEY_ARRAY_CONSTRUCT,
        KEY_ARRAY_INSERT,
        KEY_ARRAY_DESTRUCT,
        COVER_ACTOR_LOCATION,
        COVER_ACTOR_HEIGHT,
        COVER_MAX_DISTANCE,
        COVER_MAX_HEIGHT_CHANGE,
        COVER_PROJECTION_VECTOR,
        COVER_SORT_FROM_TARGET,
        POINT3_EQUALS,
        POINT3_NOT_EQUALS,
        POINT3_DOT,
        POINT3_CROSS,
        POINT2_CONSTRUCT,
        HEIGHT_CLASS,
        NAVMESH_LIST_B_CONSTRUCT,
        NAVMESH_LIST_B_DESTRUCT,
        NEARBY_NAVMESHES_FILL_B,
        NAV_HOLDER_ASSIGN,
        NAVMESH_COVER_ARRAY,
        COVER_ARRAY_ELEMENT,
        TRIANGLE_IS_COVER,
        TRIANGLE_NORMAL,
        TRIANGLE_EDGE_INFO,
        NAVMESH_EDGE_VERTICES,
        COVER_EDGE_TEST,
        COVER_LOCATION_SET_COVER,
        TES_GET_NAVMESH_INFO_MAP,
        NAVMESH_INFO_MAP_FIND_FOR_CELL,
        NAVMESH_INFO_MAP_FIND_BY_PAIR,
        LOCATION_GET_NAVMESH,
        SQUARE_ROOT_WRAPPER,
        NAVMESH_PROBE_A,
        NAVMESH_PROBE_B,
        NAVMESH_FIND_TRIANGLE,
        NAVMESH_CROSSED_EDGE,
        NAVMESH_GET_MATCHING_EDGE_B,
        EDGE_REFERENCE_COPY,
        NAVMESH_EDGE_EXTRA_INFO,
        LOCATION_SET_NAVMESH_INFO,
        LOCATION_SET_TRIANGLE,
        LOCATION_SET_POSITION,
        POINT2_SEGMENT_CLOSEST,
        HOLDER_DIFFERS,
        BASE_SOLVER_METHOD_006C9170,
        BASE_SOLVER_METHOD_006C93D0,
        BASE_SOLVER_METHOD_006C93F0,
        NAVMESH_SEARCH_CONSTRUCT,
        NAVMESH_SEARCH_RUN,
        NAVMESH_SEARCH_DESTRUCT,
        NODE_ARRAY_A_DEFAULT_CONSTRUCT,
        PATH_ARRAY_PREPARE,
        PATH_ARRAY_CAPACITY,
        PATH_ARRAY_RESERVE,
        PATH_ARRAY_ELEMENT,
        PATH_ARRAY_REMOVE,
        NODE_ARRAY_REMOVE,
        NODE_ARRAY_B_ELEMENT,
        PORTAL_CLIP,
        LINE_REACHES,
        DEBUG_HOOK,
        AVOID_NODE_ARRAY_ASSIGN,
        MAX_COST_SEARCH_FIND_POINT,
        LOS_GRID_MAP_RADIUS,
        FLOAT_AT_OFFSET_0X48,
        LOS_GRID_MAP_COPY_CENTER,
        LOS_GRID_MAP_FILL_LOS_MAP,
        LOS_GRID_MAP_MERGE_LOS_MAP,
        OBJECT_ALLOCATE,
        LOS_MAP_CONSTRUCT,
        NI_POINTER_CONSTRUCT_FROM,
        NI_POINTER_DESTRUCT,
        LOS_MAP_RESET_SCORES,
        LOS_MAP_COPY_CENTER,
        LOS_MAP_FIND_OFFSET,
        LOS_MAP_MAX_SCORE,
        LOS_MAP_MIN_SCORE,
        LOS_MAP_SET_MAX_SCORE,
        LOS_MAP_SET_MIN_SCORE,
        LOS_MAP_SET_SEEN_COUNT,
        LOS_MAP_SET_UNSEEN_COUNT,
        LOS_MAP_TRIANGLE_COUNT,
        WORD_AT_OFFSET_0XC,
        SINE_AND_COSINE,
        SCORE_ENTRY_ARRAY_ELEMENT,
        SCORE_ENTRY_ARRAY_ADD,
        SCORE_ENTRY_ARRAY_CONSTRUCT,
        SCORE_ENTRY_ARRAY_DESTRUCT,
        TRIANGLE_RECORD_ARRAY_ELEMENT,
        TRIANGLE_HAS_NEIGHBOUR,
        TRIANGLE_HAS_FLAG,
        NAVMESH_GET_MATCHING_EDGE_C,
        CELL_LIST_CONSTRUCT,
        CELL_LIST_DESTRUCT,
        CELL_IS_INTERIOR,
        CELL_GET_DATA_X,
        CELL_GET_DATA_Y,
        CELL_GET_WORLDSPACE,
        WORLDSPACE_GET_CELL_FROM_COORD,
        CELL_NAVMESH_ARRAY,
        NAVMESH_ARRAY_COUNT,
        NAVMESH_ARRAY_GET,
        NAV_HOLDER_LIST_ADD,
        NAVMESH_INFO_GET_NAVMESH,
        HOLDER_LIST_FIND_INDEX,
        NAVMESH_GET_CELL,
        NAVMESH_GET_BOUNDS,
        WORD_AT_OFFSET_0X34,
        TES_EXTERIOR_CELL_COUNT,
        TES_EXTERIOR_CELL_AT,
        FORM_TYPE,
        ARRAY_SET_RESERVED_SIZE,
        CELL_QUERY_006D9350,
        TRIANGLE_REFERENCE_ARRAY_CONSTRUCT,
        TRIANGLE_REFERENCE_ARRAY_ADD,
        TRIANGLE_REFERENCE_ARRAY_DESTRUCT,
        U16_ARRAY_CONSTRUCT,
        U16_ARRAY_DESTRUCT,
        U16_ARRAY_ELEMENT,
        NAVMESH_FIND_TRIANGLES_IN_BOX,
        SCRATCH_OBJECT_CONSTRUCT,
        SCRATCH_OBJECT_DESTRUCT,
        SEGMENT_WITHIN_RADIUS,
        LOCATION_HAS_TRIANGLE,
        NAVMESH_COMPUTE_TRIANGLE_Z,
        LOCATION_ARRAY_ELEMENT,
        POINTER_ARRAY_ELEMENT_THUNK,
        MATRIX_FROM_HEADING,
        MATRIX_TIMES_VECTOR,
        POINT3_UNIT_CROSS,
        POINT3_SUBTRACT_ASSIGN,
        LOCATION_CONSTRUCT_FROM_POSITION_AND_CELL,
        LOCATION_CONSTRUCT_FROM_POSITION_AND_WORLDSPACE,
        COVER_REQUEST_CONSTRUCT,
        POINT_ARRAY_CONSTRUCT,
        POINT_ARRAY_ELEMENT,
        POINT_ARRAY_DESTRUCT,
        POLYGON_TEST_A,
        POLYGON_TEST_B,
        DEBUG_PRINT,
        COVER_LOCATION_ARRAY_INIT,
        COVER_LOCATION_ARRAY_CLEAR,
        COVER_LOCATIONS_DESTROY_RANGE,
        COVER_LOCATIONS_REALLOCATE,
        COVER_LOCATIONS_CONSTRUCT,
        COVER_LOCATIONS_COPY,
        COVER_LOCATIONS_ADD,
        COVER_LOCATION_ASSIGN,
        ARRAY_IS_FULL,
        ARRAY_GROWN_CAPACITY,
        ARRAY_FREE_BUFFER,
        PATH_ELEMENTS_DESTROY_RANGE,
        PATH_ELEMENTS_COPY,
        PATH_ELEMENTS_REALLOCATE,
        ARRAY_MAY_SHRINK,
        ARRAY_SHRUNK_CAPACITY,
        ARRAY_CLEAR,
        DOOR_ARRAY_ADD_SLOT,
        DOOR_ELEMENTS_CONSTRUCT,
        REFERENCE_ADD,
        REFERENCE_RELEASE,
        SCRAP_ARRAY_BASE_CONSTRUCT,
        ARRAY_INIT,
        MEMORY_MANAGER_OBJECT,
        GET_THREAD_SCRAP_HEAP,
        SCRAP_ARRAY_BASE_DESTRUCT,
        CANDIDATE_ARRAY_INIT,
        REALLOCATE_BLOCK,
    ];

    /// The pages the translations read constants and globals from.
    const DATA_PAGES: &[u32] = &[
        0x0101_1000,
        0x0101_e000,
        0x0101_f000,
        0x0102_3000,
        0x0106_b000,
        0x0106_c000,
        0x011a_5000,
        0x011a_9000,
        0x011f_2000,
        0x011f_4000,
        0x0101_2000,
        0x0101_6000,
        0x0101_7000,
        0x0104_e000,
        0x011d_7000,
        0x011d_e000,
        0x0101_5000,
        0x0102_1000,
        0x0101_3000,
        0x0101_8000,
        0x0102_4000,
        0x0103_2000,
    ];

    /// Points recorded by a double.
    type Points = Rc<RefCell<Vec<[f32; 3]>>>;
    /// Floats recorded by a double.
    type Limits = Rc<RefCell<Vec<f32>>>;
    /// An engine with a request, a solution and a point recorder.
    type RecordedFixture = (Engine, Ptr<PathingRequest>, Ptr<PathingSolution>, Points);

    const NAVMESH: u32 = 0x00c0_0100;
    const LAST_NAVMESH: u32 = 0x00c0_0200;
    const ROUTE_TYPE_FUNCTION: u32 = 0x00a0_1000;
    const REQUEST_VTABLE: u32 = 0x00a0_0000;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn ret_float(value: f32) -> Ret {
        Ret {
            st0: value as f64,
            ..Ret::default()
        }
    }

    fn float_arg(word: u32) -> f32 {
        f32::from_bits(word)
    }

    /// An engine where every callee outside the file is a no-op that
    /// returns zero, and the data pages exist.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for address in CALLEES {
            e.register(*address, |_, _| Ret::default());
        }
        for page in DATA_PAGES {
            e.map(*page, 0x1000);
        }
        e.set_global(TWO, 2.0f64);
        e.set_global(TANGENT_PROBE_DISTANCE, 256.0f32);
        e.set_global(TANGENT_PULL_BACK, 1.2f32);
        e.set_global(TANGENT_MINIMUM_DOT, -(0.7f32 as f64));
        e.set_global(TANGENT_MINIMUM_LENGTH, 64.0f64);
        e.set_global(PROFILE_INTERIOR_HALF_WIDTH, 500.0f32);
        e.set_global(PROFILE_POINT_SEARCH_RADIUS, 2000.0f32);
        e.set_global(LOS_HEIGHT_WEIGHT, 3.0f64);
        e.set_global(NO_HEADING, -1.0f32);
        e.set_global(FLOAT_MAXIMUM_NEGATIVE, -f32::MAX);
        e.set_global(FLOAT_LARGEST, f32::MAX);
        e.set_global(CELL_WIDTH, 4096.0f64);
        e.set_global(TRIANGLE_Z_LIMIT, 180.0f32);
        e.set_global(HALF, 0.5f64);
        e.set_global(NINE_TENTHS, 0.9f64);
        e.set_global(MINIMUM_RADIUS, 32.0f32);
        e.set_global(COVERED_MOVE_DISTANCE, 512.0f32);
        e.set_global(FLOAT_MAXIMUM, f32::MAX);
        e.set_global(
            RANDOM_ANGLE_SCALE,
            std::f64::consts::PI * 2.0 / 4294967296.0,
        );
        e.set_global(PI_AS_FLOAT, std::f32::consts::PI as f64);
        e.set_global(DEGREES_TO_RADIANS, std::f64::consts::PI / 180.0);
        e
    }

    fn set_point(e: &mut Engine, address: u32, point: [f32; 3]) {
        for (i, value) in point.iter().enumerate() {
            e.mem.set_f32(address + 4 * i as u32, *value);
        }
    }

    fn point(e: &Engine, address: u32) -> [f32; 3] {
        [
            e.mem.f32(address),
            e.mem.f32(address + 4),
            e.mem.f32(address + 8),
        ]
    }

    fn assert_close(actual: [f32; 3], expected: [f32; 3]) {
        for i in 0..3 {
            assert!(
                (actual[i] - expected[i]).abs() < 1e-4,
                "{actual:?} != {expected:?}"
            );
        }
    }

    /// A request big enough for every subclass, and a solution.
    fn objects(e: &mut Engine) -> (Ptr<PathingRequest>, Ptr<PathingSolution>) {
        let request = Ptr::new(e.mem.alloc(0x120));
        let solution = Ptr::new(e.mem.alloc(0x60));
        (request, solution)
    }

    /// Gives the request a vtable whose `GetType` returns `kind`.
    fn set_request_type(e: &mut Engine, request: Ptr<PathingRequest>, kind: u32) {
        let mut slots = vec![0u32; 5];
        slots[4] = ROUTE_TYPE_FUNCTION;
        e.put_vtable(REQUEST_VTABLE, &slots);
        e.register_double(ROUTE_TYPE_FUNCTION, move |_, _| ret(kind));
        e.mem.set_u32(request.addr(), REQUEST_VTABLE);
    }

    fn log_of(e: &mut Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log.take().unwrap()
    }

    fn calls_to(log: &[(u32, Vec<u32>)], address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(callee, _)| *callee == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    fn position_of(log: &[(u32, Vec<u32>)], address: u32) -> Option<usize> {
        log.iter().position(|(callee, _)| *callee == address)
    }

    /// The source lines of the scope timers opened, in order.
    fn timer_lines(log: &[(u32, Vec<u32>)]) -> Vec<u32> {
        calls_to(log, TIMER_SCOPE_CONSTRUCT)
            .iter()
            .map(|args| args[4])
            .collect()
    }

    /// Every scope timer that was opened was closed.
    fn assert_timers_balanced(log: &[(u32, Vec<u32>)]) {
        assert_eq!(
            calls_to(log, TIMER_SCOPE_CONSTRUCT).len(),
            calls_to(log, TIMER_SCOPE_DESTROY).len()
        );
    }

    /// Records the three floats at argument `which` on every call of
    /// `address`; the double returns the index of the call.
    fn record_points(e: &mut Engine, address: u32, which: usize) -> Points {
        let points = Rc::new(RefCell::new(Vec::new()));
        let sink = points.clone();
        e.register_double(address, move |e, a| {
            let mut points = sink.borrow_mut();
            points.push(point(e, a[which]));
            ret(points.len() as u32 - 1)
        });
        points
    }

    /// Arithmetic doubles for the point helpers.
    fn vector_math(e: &mut Engine) {
        e.register(POINT3_SUBTRACT, |e, a| {
            let (x, y) = (point(e, a[0]), point(e, a[2]));
            set_point(e, a[1], [x[0] - y[0], x[1] - y[1], x[2] - y[2]]);
            ret(a[1])
        });
        e.register(POINT3_ADD, |e, a| {
            let (x, y) = (point(e, a[0]), point(e, a[2]));
            set_point(e, a[1], [x[0] + y[0], x[1] + y[1], x[2] + y[2]]);
            ret(a[1])
        });
        e.register(POINT3_TIMES_SCALAR, |e, a| {
            let (x, s) = (point(e, a[0]), float_arg(a[2]));
            set_point(e, a[1], [x[0] * s, x[1] * s, x[2] * s]);
            ret(a[1])
        });
        e.register(POINT3_SCALED_COPY, |e, a| {
            let (x, s) = (point(e, a[2]), float_arg(a[1]));
            set_point(e, a[0], [x[0] * s, x[1] * s, x[2] * s]);
            ret(a[0])
        });
        e.register(POINT3_SCALE_IN_PLACE, |e, a| {
            let (x, s) = (point(e, a[0]), float_arg(a[1]));
            set_point(e, a[0], [x[0] * s, x[1] * s, x[2] * s]);
            ret(a[0])
        });
        e.register(POINT3_ADD_ASSIGN, |e, a| {
            let (x, y) = (point(e, a[0]), point(e, a[1]));
            set_point(e, a[0], [x[0] + y[0], x[1] + y[1], x[2] + y[2]]);
            ret(a[0])
        });
        e.register(POINT3_NORMALIZE, |e, a| {
            let x = point(e, a[0]);
            let length = (x[0] * x[0] + x[1] * x[1] + x[2] * x[2]).sqrt();
            if length > 1e-6 {
                set_point(e, a[0], [x[0] / length, x[1] / length, x[2] / length]);
            }
            Ret::default()
        });
        e.register(POINT3_LENGTH_SQUARED, |e, a| {
            let x = point(e, a[0]);
            ret_float(x[0] * x[0] + x[1] * x[1] + x[2] * x[2])
        });
        e.register(POINT3_UNITIZE_GET_LENGTH, |e, a| {
            let x = point(e, a[0]);
            let length = (x[0] * x[0] + x[1] * x[1] + x[2] * x[2]).sqrt();
            if length > 1e-6 {
                set_point(e, a[0], [x[0] / length, x[1] / length, x[2] / length]);
            }
            ret_float(length)
        });
        e.register(FLOAT_MAX, |_, a| {
            let (x, y) = (float_arg(a[0]), float_arg(a[1]));
            ret_float(if y < x { x } else { y })
        });
        e.register(FLOAT_ABSOLUTE_VALUE, |_, a| {
            ret_float(float_arg(a[0]).abs())
        });
    }

    /// A resolvable origin (1, 2, 3) and destination (11, 2, 3) on a
    /// navmesh, with the search node arrays ending in one node. Returns the
    /// address of that node.
    fn geometry(e: &mut Engine, request: Ptr<PathingRequest>) -> u32 {
        e.register(REQUEST_ORIGIN, |_, a| ret(a[0] + 0xc));
        e.register(REQUEST_DESTINATION, |_, a| ret(a[0] + 0x34));
        e.register(LOCATION_RESOLVE_CLOSEST, |_, _| ret(1));
        e.register(LOCATION_RESOLVE_NAVMESH_INFO, |_, _| ret(1));
        e.register(LOCATION_HAS_FLAG_2, |_, _| ret(1));
        e.register(LOCATION_GET_NAVMESH_AND_TRIANGLE, |e, a| {
            e.mem.set_u16(a[2], 9);
            ret(1)
        });
        e.register(NAV_HOLDER_GET, |_, _| ret(NAVMESH));
        e.register(NAVMESH_GET_INFO, |_, a| ret(a[0] + 0x1000));
        let origin = request.addr() + 0xc;
        e.register_double(LOCATION_COPY_POSITION, move |e, a| {
            let position = if a[0] == origin {
                [1.0, 2.0, 3.0]
            } else {
                [11.0, 2.0, 3.0]
            };
            set_point(e, a[1], position);
            ret(a[1])
        });
        e.register(NAVMESH_GET_CENTER, |e, a| {
            set_point(e, a[1], [10.0, 20.0, 30.0]);
            Ret::default()
        });
        e.register(ARRAY_COUNT, |_, _| ret(2));
        let node = e.mem.alloc(0x14);
        e.mem.set_u32(node, LAST_NAVMESH);
        e.mem.set_u16(node + 4, 7);
        set_point(e, node + 8, [5.0, 6.0, 7.0]);
        e.register_double(NODE_ARRAY_ELEMENT, move |_, a| {
            assert_eq!(a[1], 1, "the last of two nodes");
            ret(node)
        });
        e.register(SOLUTION_VIRTUAL_NODE_COUNT, |_, _| ret(5));
        node
    }

    // -- 0049f210 ----------------------------------------------------------

    #[test]
    fn parent_space_node_add_constructs_then_copies_the_item() {
        let mut e = engine();
        let array: Ptr<crate::types::BSSimpleArray> = e.new_object();
        let first_buffer = e.mem.alloc(0x40);
        let second_buffer = e.mem.alloc(0x40);
        e.set(array, crate::types::BSSimpleArray::pBuffer, first_buffer);
        e.register(ARRAY_ADD_SLOT, |_, _| ret(2));
        // The construction reallocates the buffer: the copy must go to the
        // new one.
        let array_address = array.addr();
        e.register_double(ARRAY_CONSTRUCT_ELEMENTS, move |e, _| {
            e.mem.set_u32(array_address + 4, second_buffer);
            Ret::default()
        });
        let item = e.mem.alloc(12);
        for i in 0..3 {
            e.mem.set_u32(item + 4 * i, 0x100 + i);
        }
        e.call_log = Some(vec![]);
        let index = e
            .call(0x0049_f210, &args![array, Ptr::<()>::new(item)])
            .i32();
        assert_eq!(index, 2);
        let log = log_of(&mut e);
        assert_eq!(
            calls_to(&log, ARRAY_CONSTRUCT_ELEMENTS),
            vec![vec![array.addr(), first_buffer + 24, 1]]
        );
        assert_eq!(e.mem.u32(second_buffer + 24), 0x100);
        assert_eq!(e.mem.u32(second_buffer + 28), 0x101);
        assert_eq!(e.mem.u32(second_buffer + 32), 0x102);
        assert_eq!(e.mem.u32(first_buffer + 24), 0);
    }

    // -- 006d0870 ----------------------------------------------------------

    #[test]
    fn init_converts_the_three_angle_settings() {
        let mut e = engine();
        e.set_global(DEGREES_TO_RADIANS, std::f64::consts::PI / 180.0);
        for (setting, degrees) in [
            (SETTING_ANGLE_A, 90.0f32),
            (SETTING_ANGLE_B, 180.0),
            (SETTING_ANGLE_C, 45.0),
        ] {
            e.set_global(setting, degrees);
        }
        // The settings object is its own value address.
        e.register(SETTING_FLOAT_ADDRESS, |_, a| ret(a[0]));
        // The angle function doubles its argument, so the stores can be
        // told apart from the conversion.
        e.register(ANGLE_FUNCTION_005B9E80, |_, a| {
            ret_float(float_arg(a[0]) * 2.0)
        });
        assert!(e.call(0x006d_0870, &args![]).bool());
        for (destination, degrees) in [
            (CONVERTED_ANGLE_A, 90.0f32),
            (CONVERTED_ANGLE_B, 180.0),
            (CONVERTED_ANGLE_C, 45.0),
        ] {
            let radians = (degrees as f64 * (std::f64::consts::PI / 180.0)) as f32;
            assert_eq!(e.global::<f32>(destination), radians * 2.0);
        }
    }

    // -- 006d0900 ----------------------------------------------------------

    #[test]
    fn dispatch_sends_each_type_to_its_solver() {
        // The line number the solver passes to its scope timer says which
        // solver ran.
        for (kind, line) in [
            (3u32, 0xb3u32),
            (4, 0x1f5),
            (5, 0x17a),
            (6, 0x23f),
            (7, 0x29c),
            (8, 0x322),
        ] {
            let mut e = engine();
            vector_math(&mut e);
            let (request, solution) = objects(&mut e);
            set_request_type(&mut e, request, kind);
            e.call_log = Some(vec![]);
            e.call(0x006d_0900, &args![request, solution]);
            let log = log_of(&mut e);
            assert_eq!(timer_lines(&log)[..2], [0x6a, line], "type {kind}");
            assert_timers_balanced(&log);
        }
    }

    #[test]
    fn dispatch_type_zero_runs_the_base_solver() {
        let mut e = engine();
        let (request, solution) = objects(&mut e);
        set_request_type(&mut e, request, 0);
        e.register(BASE_SOLVER_RUN, |_, _| ret(1));
        e.call_log = Some(vec![]);
        assert!(e.call(0x006d_0900, &args![request, solution]).bool());
        let log = log_of(&mut e);
        assert_eq!(timer_lines(&log), vec![0x6a]);
        assert_eq!(calls_to(&log, BASE_SOLVER_CONSTRUCT).len(), 1);
        assert_timers_balanced(&log);
    }

    #[test]
    fn dispatch_fails_for_the_unused_types() {
        for kind in [1u32, 2, 9, 0x100] {
            let mut e = engine();
            let (request, solution) = objects(&mut e);
            set_request_type(&mut e, request, kind);
            e.register(BASE_SOLVER_RUN, |_, _| ret(1));
            e.call_log = Some(vec![]);
            assert!(!e.call(0x006d_0900, &args![request, solution]).bool());
            let log = log_of(&mut e);
            assert_eq!(timer_lines(&log), vec![0x6a], "type {kind}");
            assert!(calls_to(&log, BASE_SOLVER_CONSTRUCT).is_empty());
        }
    }

    // -- 006d0b10, 006d0b80 -------------------------------------------------

    #[test]
    fn base_solver_runs_and_destroys_the_solver() {
        for found in [true, false] {
            let mut e = engine();
            let (request, solution) = objects(&mut e);
            let run_result = found as u32;
            e.register_double(BASE_SOLVER_RUN, move |_, _| ret(run_result));
            e.call_log = Some(vec![]);
            let result = e.call(0x006d_0b10, &args![request, solution]).bool();
            assert_eq!(result, found);
            let log = log_of(&mut e);
            let construct = &calls_to(&log, BASE_SOLVER_CONSTRUCT)[0];
            let solver = construct[0];
            assert_eq!(construct[1..], [request.addr(), solution.addr()]);
            assert_eq!(calls_to(&log, BASE_SOLVER_RUN), vec![vec![solver]]);
            // Destroyed after the run: the member at +0x24, then +0x14.
            let run = position_of(&log, BASE_SOLVER_RUN).unwrap();
            assert_eq!(log[run + 1], (NAVMESH_LIST_DESTRUCT, vec![solver + 0x24]));
            assert_eq!(
                log[run + 2],
                (BASE_SOLVER_MEMBER_DESTRUCT, vec![solver + 0x14])
            );
        }
    }

    #[test]
    fn base_solver_destructor_destroys_both_members() {
        let mut e = engine();
        let solver = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x006d_0b80, &args![Ptr::<()>::new(solver)]);
        let log = log_of(&mut e);
        assert_eq!(
            log[1..],
            [
                (NAVMESH_LIST_DESTRUCT, vec![solver + 0x24]),
                (BASE_SOLVER_MEMBER_DESTRUCT, vec![solver + 0x14]),
            ]
        );
    }

    // -- 006d0be0 ----------------------------------------------------------

    /// A flee request on a fully resolvable navmesh whose search and path
    /// builder succeed, with the solution's virtual node points recorded.
    fn flee_setup() -> RecordedFixture {
        let mut e = engine();
        let (request, solution) = objects(&mut e);
        geometry(&mut e, request);
        vector_math(&mut e);
        e.register(FLEE_SEARCH_BUILD_NODE_PATH, |_, _| ret(1));
        e.register(PATH_BUILDER_BUILD, |_, _| ret(1));
        let added = record_points(&mut e, SOLUTION_ADD_VIRTUAL_NODE, 1);
        (e, request, solution, added)
    }

    #[test]
    fn flee_builds_a_path_to_the_center_of_the_last_triangle() {
        let (mut e, request, solution, added) = flee_setup();
        e.call_log = Some(vec![]);
        assert!(e.call(0x006d_0be0, &args![request, solution]).bool());
        let log = log_of(&mut e);
        assert_eq!(timer_lines(&log), vec![0xb3]);
        assert_timers_balanced(&log);
        // The start is the origin, the goal the last triangle's center.
        assert_eq!(*added.borrow(), vec![[1.0, 2.0, 3.0], [10.0, 20.0, 30.0]]);
        let adds = calls_to(&log, SOLUTION_ADD_VIRTUAL_NODE);
        assert_eq!(adds[0][2], NAVMESH + 0x1000);
        assert_eq!(adds[1][2], LAST_NAVMESH + 0x1000);
        assert_eq!(calls_to(&log, NAVMESH_GET_CENTER)[0][0], LAST_NAVMESH);
        assert_eq!(calls_to(&log, NAVMESH_GET_CENTER)[0][2], 7);
        assert_eq!(
            e.get(solution, PathingSolution::iFirstLoadedVirtualNodeIndex),
            0
        );
        assert_eq!(
            e.get(solution, PathingSolution::iLastLoadedVirtualNodeIndex),
            1
        );
        let build = &calls_to(&log, PATH_BUILDER_BUILD)[0];
        assert_eq!(build.len(), 9);
        assert_eq!(build[1], request.addr());
        assert_eq!(build[6], solution.addr());
        assert_eq!(build[7..], [0, 0]);
        assert!(calls_to(&log, SOLUTION_SET_INCOMPLETE).is_empty());
        // Everything built was destroyed again.
        assert_eq!(
            calls_to(&log, NODE_ARRAY_A_CONSTRUCT).len(),
            calls_to(&log, NODE_ARRAY_A_DESTRUCT).len()
        );
        assert_eq!(
            calls_to(&log, NAV_HOLDER_CONSTRUCT).len(),
            calls_to(&log, NAV_HOLDER_RELEASE).len()
        );
    }

    #[test]
    fn flee_marks_the_solution_incomplete_when_the_path_builder_fails() {
        let (mut e, request, solution, _) = flee_setup();
        e.register(PATH_BUILDER_BUILD, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert!(!e.call(0x006d_0be0, &args![request, solution]).bool());
        let log = log_of(&mut e);
        assert_eq!(
            calls_to(&log, SOLUTION_SET_INCOMPLETE),
            vec![vec![solution.addr(), 1]]
        );
    }

    #[test]
    fn flee_without_a_search_result_fails_and_cleans_up() {
        let (mut e, request, solution, added) = flee_setup();
        e.register(FLEE_SEARCH_BUILD_NODE_PATH, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert!(!e.call(0x006d_0be0, &args![request, solution]).bool());
        let log = log_of(&mut e);
        assert!(added.borrow().is_empty());
        assert!(calls_to(&log, PATH_BUILDER_BUILD).is_empty());
        assert_eq!(calls_to(&log, FLEE_SEARCH_DESTRUCT).len(), 1);
        assert_timers_balanced(&log);
    }

    #[test]
    fn flee_without_a_resolvable_origin_flees_without_a_search() {
        let (mut e, request, solution, _) = flee_setup();
        e.register(LOCATION_RESOLVE_CLOSEST, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert!(e.call(0x006d_0be0, &args![request, solution]).bool());
        let log = log_of(&mut e);
        assert_eq!(timer_lines(&log), vec![0xb3, 0x12f]);
        assert!(calls_to(&log, FLEE_SEARCH_BUILD_NODE_PATH).is_empty());
        assert_timers_balanced(&log);
    }

    #[test]
    fn flee_resolves_an_unresolved_destination_first() {
        let (mut e, request, solution, _) = flee_setup();
        e.register(LOCATION_HAS_FLAG_2, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert!(e.call(0x006d_0be0, &args![request, solution]).bool());
        let log = log_of(&mut e);
        let resolves = calls_to(&log, LOCATION_RESOLVE_NAVMESH_INFO);
        assert_eq!(resolves[0], vec![request.addr() + 0x34, 0]);
        // The destination's navmesh is fetched too.
        let fetches = calls_to(&log, LOCATION_GET_NAVMESH_AND_TRIANGLE);
        assert_eq!(fetches.len(), 2);
        assert_eq!(fetches[1][0], request.addr() + 0x34);

        // A destination that cannot be resolved flees without a search.
        let (mut e, request, solution, _) = flee_setup();
        e.register(LOCATION_HAS_FLAG_2, |_, _| ret(0));
        e.register(LOCATION_RESOLVE_NAVMESH_INFO, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(0x006d_0be0, &args![request, solution]);
        let log = log_of(&mut e);
        assert_eq!(timer_lines(&log), vec![0xb3, 0x12f]);
    }

    #[test]
    fn flee_pulls_the_goal_to_the_requests_radius_from_the_found_point() {
        let (mut e, request, solution, added) = flee_setup();
        // `006d4570` finds a point; the goal is 1 away from it (less than
        // the radius 2), so the goal becomes found point + unit * 2.
        e.register(FIND_PATH_POINT, |_, _| ret(1));
        e.register(REQUEST_ACTOR_RADIUS, |_, _| ret_float(2.0));
        e.register(POINT3_UNITIZE_GET_LENGTH, |_, _| ret_float(1.0));
        let pulled = e.mem.alloc(12);
        set_point(&mut e, pulled, [7.0, 8.0, 9.0]);
        e.register(POINT3_TIMES_SCALAR, |_, a| ret(a[1]));
        e.register_double(POINT3_ADD, move |_, _| ret(pulled));
        e.call_log = Some(vec![]);
        assert!(e.call(0x006d_0be0, &args![request, solution]).bool());
        let log = log_of(&mut e);
        assert_eq!(calls_to(&log, POINT3_TIMES_SCALAR)[0][2], 2.0f32.to_bits());
        assert_eq!(added.borrow()[1], [7.0, 8.0, 9.0]);
    }

    #[test]
    fn flee_leaves_the_goal_alone_when_it_is_beyond_the_radius() {
        let (mut e, request, solution, added) = flee_setup();
        e.register(FIND_PATH_POINT, |_, _| ret(1));
        e.register(REQUEST_ACTOR_RADIUS, |_, _| ret_float(2.0));
        e.register(POINT3_UNITIZE_GET_LENGTH, |_, _| ret_float(3.0));
        e.call_log = Some(vec![]);
        assert!(e.call(0x006d_0be0, &args![request, solution]).bool());
        let log = log_of(&mut e);
        assert!(calls_to(&log, POINT3_TIMES_SCALAR).is_empty());
        assert_eq!(added.borrow()[1], [10.0, 20.0, 30.0]);
    }

    #[test]
    fn flee_sets_the_first_tangent_from_the_heading() {
        let (mut e, request, solution, _) = flee_setup();
        e.register(REQUEST_FIRST_TANGENT_USES_HEADING, |_, _| ret(1));
        e.register(SOLUTION_NODE_COUNT, |_, _| ret(3));
        e.register(SOLUTION_NODE_AT, |_, a| ret(0x00d0_0000 + 0x100 * a[1]));
        e.register(NODE_LOCATION, |_, a| ret(a[0] + 4));
        e.register(REQUEST_INITIAL_PATH_HEADING, |_, _| ret_float(0.5));
        e.register(REQUEST_ACTOR_RADIUS, |_, _| ret_float(2.0));
        e.call_log = Some(vec![]);
        assert!(e.call(0x006d_0be0, &args![request, solution]).bool());
        let log = log_of(&mut e);
        let tangent = &calls_to(&log, BUILD_FIRST_TANGENT)[0];
        assert_eq!(tangent[1], 0.5f32.to_bits());
        assert_eq!(tangent[4], 2.0f32.to_bits());
        let copies = calls_to(&log, LOCATION_COPY_CONSTRUCT);
        assert_eq!(copies[0][1], 0x00d0_0004);
        assert_eq!(copies[1][1], 0x00d0_0104);
        assert_eq!(copies[0][0], tangent[2]);
        assert_eq!(copies[1][0], tangent[3]);
        assert_eq!(
            calls_to(&log, NODE_SET_POSITION),
            vec![vec![0x00d0_0000, tangent[0]]]
        );
        assert_eq!(calls_to(&log, LOCATION_DESTRUCT).len(), 3);

        // Two path nodes are not enough for a tangent.
        let (mut e, request, solution, _) = flee_setup();
        e.register(REQUEST_FIRST_TANGENT_USES_HEADING, |_, _| ret(1));
        e.register(SOLUTION_NODE_COUNT, |_, _| ret(2));
        e.call_log = Some(vec![]);
        e.call(0x006d_0be0, &args![request, solution]);
        assert!(calls_to(&log_of(&mut e), BUILD_FIRST_TANGENT).is_empty());
    }

    // -- 006d12a0 ----------------------------------------------------------

    /// A request with a minimum flee distance of 4, a resolvable origin
    /// and the circle axes in memory; the locations built for the two
    /// virtual nodes are recorded.
    fn flee_point_setup() -> RecordedFixture {
        let mut e = engine();
        let (request, solution) = objects(&mut e);
        geometry(&mut e, request);
        vector_math(&mut e);
        e.mem.set_f32(request.addr() + 0xb0, 4.0);
        set_point(&mut e, CIRCLE_AXIS_A, [0.0, 1.0, 0.0]);
        set_point(&mut e, CIRCLE_AXIS_B, [1.0, 0.0, 0.0]);
        let built = record_points(&mut e, LOCATION_CONSTRUCT_FROM_PARTS, 1);
        (e, request, solution, built)
    }

    #[test]
    fn flee_point_runs_away_from_the_avoid_nodes() {
        let (mut e, request, solution, built) = flee_point_setup();
        e.register(REQUEST_AVOID_NODE_ARRAY, |_, _| ret(0x00d1_0000));
        e.register(ARRAY_IS_EMPTY, |_, _| ret(0));
        // The nodes push (1, 0, 0): away is the direction (0, 2, 3).
        e.register(AVOID_ARRAY_COMPUTE_PUSH, |e, a| {
            set_point(e, a[2], [1.0, 0.0, 0.0]);
            ret_float(1.0)
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x006d_12a0, &args![request, solution]).bool());
        let log = log_of(&mut e);
        assert_eq!(timer_lines(&log), vec![0x12f]);
        assert_timers_balanced(&log);
        let built = built.borrow();
        assert_eq!(built.len(), 2);
        assert_close(built[0], [1.0, 2.0, 3.0]);
        let length = 13.0f32.sqrt();
        assert_close(
            built[1],
            [1.0, 2.0 + 4.0 * 2.0 / length, 3.0 + 4.0 * 3.0 / length],
        );
        assert_eq!(
            e.get(solution, PathingSolution::iFirstLoadedVirtualNodeIndex),
            0
        );
        assert_eq!(
            e.get(solution, PathingSolution::iLastLoadedVirtualNodeIndex),
            1
        );
    }

    #[test]
    fn flee_point_without_avoid_nodes_takes_a_random_point_on_the_circle() {
        let (mut e, request, solution, built) = flee_point_setup();
        // An empty avoid array counts as none.
        e.register(REQUEST_AVOID_NODE_ARRAY, |_, _| ret(0x00d1_0000));
        e.register(ARRAY_IS_EMPTY, |_, _| ret(1));
        // The random angle is 3/4 of a turn minus pi: pi / 2.
        e.set_global(
            RANDOM_ANGLE_SCALE,
            std::f64::consts::PI * 2.0 / 4294967296.0,
        );
        e.set_global(PI_AS_FLOAT, std::f32::consts::PI as f64);
        e.register(RANDOM_OBJECT, |_, _| ret(0x00d2_0000));
        e.register_double(RANDOM_UNSIGNED, |_, a| {
            assert_eq!(a[0], 0x00d2_0000);
            ret(0xc000_0000)
        });
        e.register(ANGLE_FUNCTION_004E44B0, |_, a| {
            ret_float(float_arg(a[0]).sin())
        });
        e.register(ANGLE_FUNCTION_004E4470, |_, a| {
            ret_float(float_arg(a[0]).cos())
        });
        // The origin has a cell with land at height 44.
        e.register(LOCATION_GET_CELL, |_, _| ret(0x00e0_0000));
        e.register(LOCATION_GET_WORLDSPACE, |_, _| ret(0x00e1_0000));
        e.register(CELL_GET_LAND_HEIGHT, |e, a| {
            e.mem.set_f32(a[2], 44.0);
            ret(1)
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x006d_12a0, &args![request, solution]).bool());
        let log = log_of(&mut e);
        let built = built.borrow();
        // Distance 4 along (0, 1, 0) (the sine of pi / 2 is 1); the cosine
        // of pi / 2 is about 0.
        assert_close(built[1], [1.0, 2.0 + 4.0, 44.0]);
        assert_close(built[0], [1.0, 2.0, 3.0]);
        let parts = calls_to(&log, LOCATION_CONSTRUCT_FROM_PARTS);
        assert_eq!(parts[0][2..], [0x00e0_0000, 0x00e1_0000]);
        assert_eq!(parts[1][2..], [0x00e0_0000, 0x00e1_0000]);
        // The land height is looked up for the new point only.
        assert_eq!(calls_to(&log, CELL_GET_LAND_HEIGHT).len(), 1);
    }

    #[test]
    fn flee_point_keeps_the_height_when_the_cell_has_no_land() {
        let (mut e, request, solution, built) = flee_point_setup();
        e.register(LOCATION_GET_CELL, |_, _| ret(0x00e0_0000));
        e.register(CELL_GET_LAND_HEIGHT, |_, _| ret(0));
        e.call(0x006d_12a0, &args![request, solution]);
        assert_eq!(built.borrow()[1][2], 3.0);
    }

    // -- 006d1520 ----------------------------------------------------------

    #[test]
    fn add_virtual_node_builds_the_node_from_the_resolved_location() {
        let mut e = engine();
        let solution: Ptr<PathingSolution> = e.new_object();
        let position = e.mem.alloc(12);
        e.register(VIRTUAL_NODE_ARRAY_ADD_SLOT, |_, _| ret(3));
        e.register_double(VIRTUAL_NODE_ARRAY_ELEMENT, |_, a| {
            assert_eq!(a[1], 3);
            ret(0x00d3_0000)
        });
        e.register_double(PLACEMENT_NEW, |_, a| {
            assert_eq!(a[..2], [0x30, 0x00d3_0000]);
            ret(a[1])
        });
        e.call_log = Some(vec![]);
        let index = e
            .call(
                0x006d_1520,
                &args![solution, Ptr::<()>::new(position), 0x11u32, 0x22u32],
            )
            .u32();
        assert_eq!(index, 3);
        let log = log_of(&mut e);
        let parts = &calls_to(&log, LOCATION_CONSTRUCT_FROM_PARTS)[0];
        let location = parts[0];
        assert_eq!(parts[1..], [position, 0x11, 0x22]);
        assert_eq!(
            calls_to(&log, LOCATION_RESOLVE_NAVMESH_INFO),
            vec![vec![location, 0]]
        );
        // The array is the solution's, at +8.
        assert_eq!(
            calls_to(&log, VIRTUAL_NODE_ARRAY_ADD_SLOT),
            vec![vec![solution.addr() + 8]]
        );
        assert_eq!(
            calls_to(&log, VIRTUAL_NODE_CONSTRUCT),
            vec![vec![0x00d3_0000, location]]
        );
        assert_eq!(calls_to(&log, LOCATION_DESTRUCT), vec![vec![location]]);

        // No block: no node is constructed.
        e.register(PLACEMENT_NEW, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(
            0x006d_1520,
            &args![solution, Ptr::<()>::new(position), 0u32, 0u32],
        );
        assert!(calls_to(&log_of(&mut e), VIRTUAL_NODE_CONSTRUCT).is_empty());
    }

    // -- 006d1600, 006d1620 --------------------------------------------------

    #[test]
    fn random_angle_maps_a_random_number_to_minus_pi_to_pi() {
        let mut e = engine();
        e.set_global(
            RANDOM_ANGLE_SCALE,
            std::f64::consts::PI * 2.0 / 4294967296.0,
        );
        e.set_global(PI_AS_FLOAT, std::f32::consts::PI as f64);
        e.register(RANDOM_OBJECT, |_, _| ret(0x00d2_0000));
        e.register_double(RANDOM_UNSIGNED, |_, a| {
            assert_eq!(a[0], 0x00d2_0000);
            ret(0x4000_0000)
        });
        let angle = e.call(0x006d_1600, &args![]).f32();
        assert!((angle + std::f32::consts::FRAC_PI_2).abs() < 1e-6);
        e.register(RANDOM_UNSIGNED, |_, _| ret(0));
        let angle = e.call(0x006d_1620, &args![Ptr::<()>::new(1)]).f32();
        assert_eq!(angle, -(std::f32::consts::PI));
        e.register(RANDOM_UNSIGNED, |_, _| ret(u32::MAX));
        let angle = e.call(0x006d_1620, &args![Ptr::<()>::new(1)]).f32();
        assert!(angle > 3.0 && angle <= std::f32::consts::PI + 1e-6);
    }

    // -- 006d1660 ----------------------------------------------------------

    /// A line-of-sight request on a resolvable navmesh whose search finds a
    /// path; the copy of the request the solver builds is typed 0 (base)
    /// and the base solver succeeds.
    fn los_setup() -> (Engine, Ptr<PathingRequest>, Ptr<PathingSolution>) {
        let mut e = engine();
        let (request, solution) = objects(&mut e);
        geometry(&mut e, request);
        e.register(LOS_SEARCH_BUILD_NODE_PATH, |_, _| ret(1));
        e.register(BASE_SOLVER_RUN, |_, _| ret(1));
        e.register(LOCATION_GET_WORLDSPACE, |e, a| ret(e.mem.u32(a[0] + 0x1c)));
        let mut slots = vec![0u32; 5];
        slots[4] = ROUTE_TYPE_FUNCTION;
        e.put_vtable(REQUEST_VTABLE, &slots);
        e.register(ROUTE_TYPE_FUNCTION, |_, _| ret(0));
        e.register(REQUEST_COPY_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], REQUEST_VTABLE);
            Ret::default()
        });
        (e, request, solution)
    }

    #[test]
    fn los_solves_again_from_the_last_triangle_of_the_path() {
        let (mut e, request, solution) = los_setup();
        e.call_log = Some(vec![]);
        assert!(e.call(0x006d_1660, &args![request, solution]).bool());
        let log = log_of(&mut e);
        assert_eq!(timer_lines(&log), vec![0x17a, 0x6a]);
        assert_timers_balanced(&log);
        // The destination of the copy is a location on the last triangle.
        let copy = calls_to(&log, REQUEST_COPY_CONSTRUCT)[0].clone();
        assert_eq!(copy[1], request.addr());
        let location = &calls_to(&log, LOCATION_CONSTRUCT_FROM_MESH_POINT)[0];
        assert_eq!(location[2], LAST_NAVMESH + 0x1000);
        assert_eq!(location[3], 7);
        assert_eq!(
            calls_to(&log, LOCATION_ASSIGN),
            vec![vec![copy[0] + 0x34, location[0]]]
        );
        assert_eq!(calls_to(&log, SOLUTION_CLEAR), vec![vec![solution.addr()]]);
        assert_eq!(calls_to(&log, REQUEST_COPY_DESTRUCT), vec![vec![copy[0]]]);
        assert_eq!(calls_to(&log, LOS_SEARCH_DESTRUCT).len(), 1);
    }

    #[test]
    fn los_fails_when_the_origin_cannot_be_resolved() {
        let (mut e, request, solution) = los_setup();
        e.register(LOCATION_RESOLVE_CLOSEST, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert!(!e.call(0x006d_1660, &args![request, solution]).bool());
        let log = log_of(&mut e);
        assert_eq!(timer_lines(&log), vec![0x17a]);
        assert!(calls_to(&log, NAV_HOLDER_CONSTRUCT).is_empty());

        let (mut e, request, solution) = los_setup();
        e.register(LOCATION_GET_NAVMESH_AND_TRIANGLE, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert!(!e.call(0x006d_1660, &args![request, solution]).bool());
        let log = log_of(&mut e);
        assert!(calls_to(&log, LOS_SEARCH_CONSTRUCT).is_empty());
        assert_eq!(calls_to(&log, NAV_HOLDER_RELEASE).len(), 1);
        assert_timers_balanced(&log);
    }

    #[test]
    fn los_tries_the_searchs_recorded_nodes_in_order() {
        // The first search fails; the recorded nodes at +0x2090, +0x2094 and
        // +0x2098 are tried in turn until one gives a path.
        for (winner, expected_calls) in [(0usize, 1usize), (1, 2), (2, 3)] {
            let (mut e, request, solution) = los_setup();
            e.register(LOS_SEARCH_BUILD_NODE_PATH, |_, _| ret(0));
            e.register(LOS_SEARCH_CONSTRUCT, |e, a| {
                e.mem.set_u32(a[0] + 0x2090, 0x1111);
                e.mem.set_u32(a[0] + 0x2094, 0x2222);
                e.mem.set_u32(a[0] + 0x2098, 0x3333);
                Ret::default()
            });
            let calls = Rc::new(RefCell::new(Vec::new()));
            let sink = calls.clone();
            e.register_double(LOS_SEARCH_PATH_FROM_NODE, move |_, a| {
                sink.borrow_mut().push(a[1]);
                ret((sink.borrow().len() == winner + 1) as u32)
            });
            e.call_log = Some(vec![]);
            assert!(e.call(0x006d_1660, &args![request, solution]).bool());
            assert_eq!(
                calls.borrow()[..],
                [0x1111, 0x2222, 0x3333][..expected_calls]
            );
        }
    }

    #[test]
    fn los_moves_the_last_virtual_node_to_the_map_triangle_when_all_fail() {
        let (mut e, request, solution) = los_setup();
        e.register(LOS_SEARCH_BUILD_NODE_PATH, |_, _| ret(0));
        // No recorded node gives a path; the LOS map points at triangle 6.
        e.register(NI_POINTER_GET_FROM_FIELD, |_, a| ret(a[0] + 0x500));
        e.register(LOS_MAP_FIND_TRIANGLE, |e, a| {
            e.mem.set_u16(a[2], 6);
            ret(1)
        });
        e.register(SOLUTION_VIRTUAL_NODE_AT, |_, _| ret(0x00d4_0000));
        e.set(solution, PathingSolution::iLastLoadedVirtualNodeIndex, -1);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x006d_1660, &args![request, solution]).bool());
        let log = log_of(&mut e);
        // No loaded node: the start is added and the index reset to 0, but
        // the node looked up is the one for the old index (-1).
        assert_eq!(calls_to(&log, SOLUTION_ADD_VIRTUAL_NODE).len(), 1);
        assert_eq!(
            e.get(solution, PathingSolution::iLastLoadedVirtualNodeIndex),
            0
        );
        assert_eq!(
            calls_to(&log, SOLUTION_VIRTUAL_NODE_AT),
            vec![vec![solution.addr(), u32::MAX]]
        );
        let center = &calls_to(&log, NAVMESH_GET_CENTER)[0];
        assert_eq!(center[2], 6);
        let mesh_point = &calls_to(&log, LOCATION_CONSTRUCT_FROM_MESH_POINT)[0];
        assert_eq!(mesh_point[3], 6);
        assert_eq!(
            calls_to(&log, VIRTUAL_NODE_SET_LOCATION),
            vec![vec![0x00d4_0000, mesh_point[0]]]
        );
        assert_timers_balanced(&log);
    }

    #[test]
    fn los_keeps_the_loaded_node_index_when_there_is_one() {
        let (mut e, request, solution) = los_setup();
        e.register(LOS_SEARCH_BUILD_NODE_PATH, |_, _| ret(0));
        e.register(NI_POINTER_GET_FROM_FIELD, |_, a| ret(a[0] + 0x500));
        e.register(LOS_MAP_FIND_TRIANGLE, |_, _| ret(1));
        e.register(SOLUTION_VIRTUAL_NODE_AT, |_, _| ret(0));
        e.set(solution, PathingSolution::iLastLoadedVirtualNodeIndex, 4);
        e.call_log = Some(vec![]);
        e.call(0x006d_1660, &args![request, solution]);
        let log = log_of(&mut e);
        assert!(calls_to(&log, SOLUTION_ADD_VIRTUAL_NODE).is_empty());
        assert_eq!(
            calls_to(&log, SOLUTION_VIRTUAL_NODE_AT),
            vec![vec![solution.addr(), 4]]
        );
        // A missing node means nothing is moved.
        assert!(calls_to(&log, VIRTUAL_NODE_SET_LOCATION).is_empty());
    }

    // -- small accessors ---------------------------------------------------

    #[test]
    fn destination_and_origin_setters_assign_the_locations() {
        let mut e = engine();
        let request = e.mem.alloc(0x120);
        let location = e.mem.alloc(0x28);
        e.call_log = Some(vec![]);
        e.call(
            0x006d_1bc0,
            &args![Ptr::<()>::new(request), Ptr::<()>::new(location)],
        );
        e.call(
            0x006d_3ac0,
            &args![Ptr::<()>::new(request), Ptr::<()>::new(location)],
        );
        let log = log_of(&mut e);
        assert_eq!(
            calls_to(&log, LOCATION_ASSIGN),
            vec![
                vec![request + 0x34, location],
                vec![request + 0xc, location]
            ]
        );
    }

    #[test]
    fn los_search_getters_read_their_fields() {
        let mut e = engine();
        let search = e.mem.alloc(0x2100);
        e.mem.set_u32(search + 0x2090, 0xa1);
        e.mem.set_u32(search + 0x2094, 0xa2);
        e.mem.set_u32(search + 0x2098, 0xa3);
        e.register(NI_POINTER_GET_FROM_FIELD, |_, a| ret(a[0]));
        let search = Ptr::<()>::new(search);
        assert_eq!(e.call(0x006d_1be0, &args![search]).u32(), 0xa1);
        assert_eq!(e.call(0x006d_1c00, &args![search]).u32(), 0xa2);
        assert_eq!(e.call(0x006d_1c20, &args![search]).u32(), 0xa3);
        // The LOS map is the pointer held by the `NiPointer` at +0x2070.
        assert_eq!(
            e.call(0x006d_1c40, &args![search]).u32(),
            search.addr() + 0x2070
        );
    }

    // -- 006d1c60 ----------------------------------------------------------

    /// A close-point request whose search succeeds.
    fn close_point_setup() -> RecordedFixture {
        let mut e = engine();
        let (request, solution) = objects(&mut e);
        geometry(&mut e, request);
        e.register(CLOSE_POINT_SEARCH_BUILD_NODE_PATH, |_, _| ret(1));
        let added = record_points(&mut e, SOLUTION_ADD_VIRTUAL_NODE, 1);
        (e, request, solution, added)
    }

    #[test]
    fn close_point_smooths_the_path_into_the_solution() {
        let (mut e, request, solution, added) = close_point_setup();
        e.call_log = Some(vec![]);
        assert!(e.call(0x006d_1c60, &args![request, solution]).bool());
        let log = log_of(&mut e);
        assert_eq!(timer_lines(&log), vec![0x1f5]);
        assert_timers_balanced(&log);
        assert_eq!(*added.borrow(), vec![[1.0, 2.0, 3.0], [10.0, 20.0, 30.0]]);
        let smooth = &calls_to(&log, PATH_SMOOTHER_SMOOTH)[0];
        assert_eq!(smooth.len(), 9);
        assert_eq!(smooth[1], request.addr());
        assert_eq!(smooth[6], solution.addr());
        assert_eq!(smooth[7..], [0, 1]);
        // The loaded range ends at the last virtual node.
        assert_eq!(
            e.get(solution, PathingSolution::iFirstLoadedVirtualNodeIndex),
            0
        );
        assert_eq!(
            e.get(solution, PathingSolution::iLastLoadedVirtualNodeIndex),
            4
        );
        // Destroyed again: smoother, node arrays, search, holder.
        assert_eq!(
            calls_to(&log, PATH_SMOOTHER_DESTRUCT),
            vec![vec![smooth[0]]]
        );
        assert_eq!(calls_to(&log, CLOSE_POINT_SEARCH_DESTRUCT).len(), 1);
        assert_eq!(calls_to(&log, NAV_HOLDER_RELEASE).len(), 1);
    }

    #[test]
    fn close_point_fails_without_navmesh_info_or_path() {
        let (mut e, request, solution, _) = close_point_setup();
        e.register(LOCATION_RESOLVE_NAVMESH_INFO, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert!(!e.call(0x006d_1c60, &args![request, solution]).bool());
        assert!(calls_to(&log_of(&mut e), NAV_HOLDER_CONSTRUCT).is_empty());

        let (mut e, request, solution, added) = close_point_setup();
        e.register(CLOSE_POINT_SEARCH_BUILD_NODE_PATH, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert!(!e.call(0x006d_1c60, &args![request, solution]).bool());
        let log = log_of(&mut e);
        assert!(added.borrow().is_empty());
        assert_eq!(calls_to(&log, CLOSE_POINT_SEARCH_DESTRUCT).len(), 1);
        assert_timers_balanced(&log);

        let (mut e, request, solution, _) = close_point_setup();
        e.register(LOCATION_GET_NAVMESH_AND_TRIANGLE, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert!(!e.call(0x006d_1c60, &args![request, solution]).bool());
        let log = log_of(&mut e);
        assert_eq!(calls_to(&log, NAV_HOLDER_RELEASE).len(), 1);
        assert!(calls_to(&log, CLOSE_POINT_SEARCH_CONSTRUCT).is_empty());
    }

    // -- 006d1f80 ----------------------------------------------------------

    #[test]
    fn hide_searches_with_both_start_navmeshes() {
        let mut e = engine();
        let (request, solution) = objects(&mut e);
        geometry(&mut e, request);
        e.register(HIDE_SEARCH_BUILD_NODE_PATH, |_, _| ret(1));
        let added = record_points(&mut e, SOLUTION_ADD_VIRTUAL_NODE, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x006d_1f80, &args![request, solution]).bool());
        let log = log_of(&mut e);
        assert_eq!(timer_lines(&log), vec![0x23f]);
        assert_timers_balanced(&log);
        assert_eq!(*added.borrow(), vec![[1.0, 2.0, 3.0], [10.0, 20.0, 30.0]]);
        // Both locations are resolved; the search gets both starts.
        let navs = calls_to(&log, LOCATION_GET_NAVMESH_AND_TRIANGLE);
        assert_eq!(navs[0][0], request.addr() + 0xc);
        assert_eq!(navs[1][0], request.addr() + 0x34);
        let build = &calls_to(&log, HIDE_SEARCH_BUILD_NODE_PATH)[0];
        assert_eq!(build.len(), 6);
        assert_eq!(build[1], request.addr());
        assert_ne!(build[2], build[3]);
        assert_eq!(e.mem.u32(build[2]), NAVMESH);
        let smooth = &calls_to(&log, PATH_SMOOTHER_SMOOTH)[0];
        assert_eq!(smooth[7..], [0, 1]);
        assert_eq!(
            e.get(solution, PathingSolution::iLastLoadedVirtualNodeIndex),
            4
        );
        assert_eq!(
            calls_to(&log, NAV_HOLDER_CONSTRUCT).len(),
            calls_to(&log, NAV_HOLDER_RELEASE).len()
        );
    }

    #[test]
    fn hide_fails_when_a_location_or_the_search_fails() {
        // The origin cannot be resolved.
        let mut e = engine();
        let (request, solution) = objects(&mut e);
        geometry(&mut e, request);
        e.register(HIDE_SEARCH_BUILD_NODE_PATH, |_, _| ret(1));
        e.register(LOCATION_RESOLVE_CLOSEST, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert!(!e.call(0x006d_1f80, &args![request, solution]).bool());
        assert!(calls_to(&log_of(&mut e), NAV_HOLDER_CONSTRUCT).is_empty());

        // The destination cannot be resolved (the origin can).
        let mut e = engine();
        let (request, solution) = objects(&mut e);
        geometry(&mut e, request);
        let destination = request.addr() + 0x34;
        e.register_double(LOCATION_RESOLVE_CLOSEST, move |_, a| {
            ret((a[0] != destination) as u32)
        });
        e.call_log = Some(vec![]);
        assert!(!e.call(0x006d_1f80, &args![request, solution]).bool());
        let log = log_of(&mut e);
        assert_eq!(calls_to(&log, NAV_HOLDER_RELEASE).len(), 1);
        assert_timers_balanced(&log);

        // The destination has no navmesh triangle.
        let mut e = engine();
        let (request, solution) = objects(&mut e);
        geometry(&mut e, request);
        let destination = request.addr() + 0x34;
        e.register_double(LOCATION_GET_NAVMESH_AND_TRIANGLE, move |_, a| {
            ret((a[0] != destination) as u32)
        });
        e.call_log = Some(vec![]);
        assert!(!e.call(0x006d_1f80, &args![request, solution]).bool());
        assert_eq!(calls_to(&log_of(&mut e), NAV_HOLDER_RELEASE).len(), 2);

        // The search finds nothing.
        let mut e = engine();
        let (request, solution) = objects(&mut e);
        geometry(&mut e, request);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x006d_1f80, &args![request, solution]).bool());
        let log = log_of(&mut e);
        assert_eq!(calls_to(&log, HIDE_SEARCH_DESTRUCT).len(), 1);
        assert!(calls_to(&log, SOLUTION_ADD_VIRTUAL_NODE).is_empty());
    }

    // -- 006d23d0 ----------------------------------------------------------

    /// An optimal-location request: distances 4 and 8 (halfway 6) and
    /// absolute bounds 5 and 7, origin (1, 2, 3), destination (11, 2, 3),
    /// a closest navmesh point equal to the aim point (5, 2, 3), a max-cost
    /// search that finds a path ending at (5, 2, 3). The cost limit the
    /// search is given is recorded.
    fn optimal_setup() -> (Engine, Ptr<PathingRequest>, Ptr<PathingSolution>, Limits) {
        let mut e = engine();
        let (request, solution) = objects(&mut e);
        geometry(&mut e, request);
        vector_math(&mut e);
        e.mem.set_f32(request.addr() + 0xb0, 4.0);
        e.mem.set_f32(request.addr() + 0xb4, 8.0);
        e.register(REQUEST_ABSOLUTE_MINIMUM_DISTANCE, |_, _| ret_float(5.0));
        e.register(REQUEST_ABSOLUTE_MAXIMUM_DISTANCE, |_, _| ret_float(7.0));
        e.register(FIND_CLOSEST_POINT_ON_NAVMESH, |e, a| {
            let aim = point(e, a[2]);
            set_point(e, a[3], aim);
            ret(1)
        });
        let limits = Rc::new(RefCell::new(Vec::new()));
        let sink = limits.clone();
        e.register_double(MAX_COST_SEARCH_BUILD_NODE_PATH, move |e, a| {
            sink.borrow_mut().push(e.mem.f32(a[0] + 0x20a4));
            ret(1)
        });
        let end = e.mem.alloc(12);
        set_point(&mut e, end, [5.0, 2.0, 3.0]);
        e.register_double(NODE_TO_POSITION, move |_, _| ret(end));
        e.register(PATH_BUILDER_BUILD, |_, _| ret(1));
        (e, request, solution, limits)
    }

    #[test]
    fn optimal_location_builds_the_path_to_the_chosen_point() {
        let (mut e, request, solution, limits) = optimal_setup();
        let added = record_points(&mut e, SOLUTION_ADD_VIRTUAL_NODE, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x006d_23d0, &args![request, solution]).bool());
        let log = log_of(&mut e);
        assert_eq!(timer_lines(&log), vec![0x29c]);
        assert_timers_balanced(&log);
        // The cost limit is twice the slack |6 - 10| = 4.
        assert_eq!(*limits.borrow(), vec![8.0]);
        assert_eq!(*added.borrow(), vec![[1.0, 2.0, 3.0], [5.0, 2.0, 3.0]]);
        let build = &calls_to(&log, PATH_BUILDER_BUILD)[0];
        assert_eq!(build[1], request.addr());
        assert_eq!(build[6], solution.addr());
        assert_eq!(
            e.get(solution, PathingSolution::iFirstLoadedVirtualNodeIndex),
            0
        );
        assert_eq!(
            e.get(solution, PathingSolution::iLastLoadedVirtualNodeIndex),
            1
        );
        // Everything is destroyed again.
        assert_eq!(calls_to(&log, MAX_COST_SEARCH_DESTRUCT).len(), 1);
        assert_eq!(calls_to(&log, NAVMESH_LIST_DESTRUCT).len(), 1);
        assert_eq!(calls_to(&log, NODE_ARRAY_C_DESTRUCT).len(), 1);
        assert_eq!(calls_to(&log, PATH_BUILDER_DESTRUCT).len(), 1);
        assert_eq!(calls_to(&log, LOCATION_DESTRUCT).len(), 1);
        assert_eq!(calls_to(&log, NAV_HOLDER_RELEASE).len(), 1);
    }

    #[test]
    fn optimal_location_aims_between_the_two_distances() {
        let (mut e, request, solution, _) = optimal_setup();
        e.call_log = Some(vec![]);
        e.call(0x006d_23d0, &args![request, solution]);
        let log = log_of(&mut e);
        // Closest-point query: worldspace, cell, aim point, result.
        let query = &calls_to(&log, FIND_CLOSEST_POINT_ON_NAVMESH)[0];
        assert_eq!(query.len(), 4);
        // The goal location is built from the closest point and the origin.
        let goal = &calls_to(&log, LOCATION_CONSTRUCT_FROM_POINT_AND_LOCATION)[0];
        assert_eq!(goal[1], query[3]);
        assert_eq!(goal[2], request.addr() + 0xc);
    }

    #[test]
    fn optimal_location_fails_when_the_closest_point_is_too_far() {
        let (mut e, request, solution, limits) = optimal_setup();
        // The closest point is 10 from the aim point; the slack is 4.
        e.register(FIND_CLOSEST_POINT_ON_NAVMESH, |e, a| {
            let aim = point(e, a[2]);
            set_point(e, a[3], [aim[0] + 10.0, aim[1], aim[2]]);
            ret(1)
        });
        e.call_log = Some(vec![]);
        assert!(!e.call(0x006d_23d0, &args![request, solution]).bool());
        let log = log_of(&mut e);
        assert!(limits.borrow().is_empty());
        assert_eq!(calls_to(&log, NAV_HOLDER_RELEASE).len(), 1);
        assert_timers_balanced(&log);

        // No closest point at all.
        let (mut e, request, solution, _) = optimal_setup();
        e.register(FIND_CLOSEST_POINT_ON_NAVMESH, |_, _| ret(0));
        assert!(!e.call(0x006d_23d0, &args![request, solution]).bool());
    }

    #[test]
    fn optimal_location_fails_when_the_origin_is_unresolved() {
        let (mut e, request, solution, _) = optimal_setup();
        e.register(LOCATION_RESOLVE_CLOSEST, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert!(!e.call(0x006d_23d0, &args![request, solution]).bool());
        assert!(calls_to(&log_of(&mut e), NAV_HOLDER_CONSTRUCT).is_empty());

        let (mut e, request, solution, _) = optimal_setup();
        e.register(LOCATION_GET_NAVMESH_AND_TRIANGLE, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert!(!e.call(0x006d_23d0, &args![request, solution]).bool());
        assert_eq!(calls_to(&log_of(&mut e), NAV_HOLDER_RELEASE).len(), 1);
    }

    #[test]
    fn optimal_location_without_a_path_uses_the_best_node_or_the_base_solver() {
        // No path, but the node array holds nodes: the best node of the
        // search (copied from +8 of it) gives the point.
        let (mut e, request, solution, _) = optimal_setup();
        e.register(MAX_COST_SEARCH_BUILD_NODE_PATH, |_, _| ret(0));
        e.register(ARRAY_IS_EMPTY, |_, _| ret(0));
        let best = e.mem.alloc(0x20);
        for i in 0..5 {
            e.mem.set_u32(best + 8 + 4 * i, 0xa0 + i);
        }
        e.register_double(MAX_COST_SEARCH_BEST_NODE, move |_, _| ret(best));
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = seen.clone();
        let end = e.mem.alloc(12);
        set_point(&mut e, end, [5.0, 2.0, 3.0]);
        e.register_double(NODE_TO_POSITION, move |e, a| {
            for i in 0..5 {
                sink.borrow_mut().push(e.mem.u32(a[0] + 4 * i));
            }
            ret(end)
        });
        assert!(e.call(0x006d_23d0, &args![request, solution]).bool());
        assert_eq!(*seen.borrow(), vec![0xa0, 0xa1, 0xa2, 0xa3, 0xa4]);

        // No path and no nodes: the base solver runs when the direct
        // distance (10) exceeds the absolute maximum, else it fails.
        for (absolute_maximum, expect_base) in [(7.0f32, true), (10.0, false), (20.0, false)] {
            let (mut e, request, solution, _) = optimal_setup();
            e.register(MAX_COST_SEARCH_BUILD_NODE_PATH, |_, _| ret(0));
            e.register(ARRAY_IS_EMPTY, |_, _| ret(1));
            e.register_double(REQUEST_ABSOLUTE_MAXIMUM_DISTANCE, move |_, _| {
                ret_float(absolute_maximum)
            });
            e.register(BASE_SOLVER_RUN, |_, _| ret(1));
            e.call_log = Some(vec![]);
            let result = e.call(0x006d_23d0, &args![request, solution]).bool();
            let log = log_of(&mut e);
            assert_eq!(result, expect_base, "bound {absolute_maximum}");
            assert_eq!(
                !calls_to(&log, BASE_SOLVER_CONSTRUCT).is_empty(),
                expect_base
            );
            assert!(calls_to(&log, PATH_BUILDER_BUILD).is_empty());
            assert_eq!(calls_to(&log, MAX_COST_SEARCH_DESTRUCT).len(), 1);
            assert_timers_balanced(&log);
        }
    }

    #[test]
    fn optimal_location_requires_the_point_between_the_absolute_bounds() {
        // The chosen point is 6 from the destination: squared 36.
        for (minimum, maximum, expected) in [
            (5.0f32, 7.0f32, true),
            (6.5, 7.0, false),
            (5.0, 5.5, false),
            (6.0, 6.0, true),
        ] {
            let (mut e, request, solution, _) = optimal_setup();
            e.register_double(REQUEST_ABSOLUTE_MINIMUM_DISTANCE, move |_, _| {
                ret_float(minimum)
            });
            e.register_double(REQUEST_ABSOLUTE_MAXIMUM_DISTANCE, move |_, _| {
                ret_float(maximum)
            });
            e.call_log = Some(vec![]);
            let result = e.call(0x006d_23d0, &args![request, solution]).bool();
            let log = log_of(&mut e);
            assert_eq!(result, expected, "bounds {minimum} {maximum}");
            assert_eq!(!calls_to(&log, PATH_BUILDER_BUILD).is_empty(), expected);
            assert_timers_balanced(&log);
            assert_eq!(calls_to(&log, LOCATION_DESTRUCT).len(), 1);
        }
    }

    #[test]
    fn optimal_location_marks_an_unresolved_goal_and_uses_the_node_position() {
        // With navmesh info the goal location has no flag and the chosen
        // point is the closest point; without it the flag is set and the
        // point comes from the last node of the path.
        let (mut e, request, solution, _) = optimal_setup();
        e.register(LOCATION_HAS_FLAG_2, |e, a| {
            ret(((e.mem.u8(a[0] + 0x26) & 2) != 0) as u32)
        });
        e.register(LOCATION_RESOLVE_NAVMESH_INFO, |_, _| ret(1));
        let added = record_points(&mut e, SOLUTION_ADD_VIRTUAL_NODE, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x006d_23d0, &args![request, solution]).bool());
        let log = log_of(&mut e);
        assert!(calls_to(&log, NODE_TO_POSITION).is_empty());
        assert_eq!(added.borrow()[1], [5.0, 2.0, 3.0]);

        let (mut e, request, solution, _) = optimal_setup();
        e.register(LOCATION_HAS_FLAG_2, |e, a| {
            ret(((e.mem.u8(a[0] + 0x26) & 2) != 0) as u32)
        });
        e.register(LOCATION_RESOLVE_NAVMESH_INFO, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert!(e.call(0x006d_23d0, &args![request, solution]).bool());
        assert_eq!(calls_to(&log_of(&mut e), NODE_TO_POSITION).len(), 1);
    }

    // -- 006d2c00 .. 006d2c60 -----------------------------------------------

    #[test]
    fn location_flag_setter_sets_bit_one() {
        let mut e = engine();
        let location: Ptr<PathingLocation> = e.new_object();
        e.set(location, PathingLocation::uiFlags, 0x11);
        e.call(0x006d_2c00, &args![location]);
        assert_eq!(e.get(location, PathingLocation::uiFlags), 0x13);
        e.call(0x006d_2c00, &args![location]);
        assert_eq!(e.get(location, PathingLocation::uiFlags), 0x13);
    }

    #[test]
    fn distance_accessors_read_and_write_their_floats() {
        let mut e = engine();
        let object = e.mem.alloc(0x2100);
        e.mem.set_f32(object + 0xb0, 1.5);
        e.mem.set_f32(object + 0xb4, 2.5);
        let object = Ptr::<()>::new(object);
        assert_eq!(e.call(0x006d_2c20, &args![object]).f32(), 1.5);
        assert_eq!(e.call(0x006d_2c40, &args![object]).f32(), 2.5);
        e.call(0x006d_2c60, &args![object, 7.25f32]);
        assert_eq!(e.mem.f32(object.addr() + 0x20a4), 7.25);
    }

    // -- 006d2c80 ----------------------------------------------------------

    /// A covered-move request whose base solve succeeds, with a solution of
    /// three path nodes; the cover check passes for the candidates in
    /// `passing` (a counter picks them in order) and writes (7, 8, 9) as
    /// the cover point of the request it checks.
    fn covered_move_setup(
        results: Vec<u32>,
    ) -> (Engine, Ptr<PathingRequest>, Ptr<PathingSolution>) {
        let mut e = engine();
        let (request, solution) = objects(&mut e);
        e.register(REQUEST_ORIGIN, |_, a| ret(a[0] + 0xc));
        e.register(MEMORY_ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(MEMORY_FREE, |e, a| {
            e.mem.free(a[0]);
            Ret::default()
        });
        e.register(BASE_SOLVER_RUN, |_, _| ret(1));
        e.register(SOLUTION_NODE_COUNT, |_, _| ret(3));
        e.register(SOLUTION_NODE_AT, |_, a| ret(0x00d0_0000 + 0x100 * a[1]));
        e.register(NODE_LOCATION, |_, a| ret(a[0] + 4));
        e.register(COVERED_MOVE_COVER_POINT, |_, a| ret(a[0] + 0xc8));
        let mut results = results.into_iter();
        e.register_double(COVERED_MOVE_CHECK, move |e, a| {
            set_point(e, a[0] + 0xc8, [7.0, 8.0, 9.0]);
            ret(results.next().unwrap_or(0))
        });
        (e, request, solution)
    }

    #[test]
    fn covered_move_adopts_the_first_destination_that_passes() {
        let (mut e, request, solution) = covered_move_setup(vec![1]);
        e.set_global(COVERED_MOVE_DISTANCE, 512.0f32);
        // The copy's cover pointers are cleared before it is destroyed.
        let cleared = Rc::new(RefCell::new(Vec::new()));
        let sink = cleared.clone();
        e.register_double(COVERED_MOVE_DESTRUCT, move |e, a| {
            sink.borrow_mut()
                .push((e.mem.u32(a[0] + 0x104), e.mem.u32(a[0] + 0x108)));
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x006d_2c80, &args![request, solution]).bool());
        let log = log_of(&mut e);
        assert_eq!(timer_lines(&log), vec![0x322]);
        assert_timers_balanced(&log);
        // The cover point of the copy became the request's.
        assert_eq!(point(&e, request.addr() + 0xc8), [7.0, 8.0, 9.0]);
        // The candidate was the second node (index 1); the check gets the
        // distance and the flag 1.
        let check = &calls_to(&log, COVERED_MOVE_CHECK)[0];
        assert_eq!(check[1..], [512.0f32.to_bits(), 1]);
        assert_eq!(calls_to(&log, SOLUTION_NODE_AT)[0][1], 1);
        // The request keeps the cover objects, the copy does not.
        assert_eq!(*cleared.borrow(), vec![(0, 0)]);
        let cover = e.mem.u32(request.addr() + 0x104);
        let info = e.mem.u32(request.addr() + 0x108);
        assert!(cover != 0 && info != 0);
        let allocations = calls_to(&log, MEMORY_ALLOCATE);
        assert_eq!(allocations, vec![vec![0x78], vec![0x3c]]);
        assert!(calls_to(&log, MEMORY_FREE).is_empty());
        // The new solution is copied over and the request is finished.
        let copied = &calls_to(&log, SOLUTION_COPY_TO)[0];
        assert_eq!(copied[1], solution.addr());
        assert_eq!(
            calls_to(&log, REQUEST_SET_FINISHED),
            vec![vec![request.addr(), 1]]
        );
        // The request's destination is the cover target location.
        let assigned = calls_to(&log, LOCATION_ASSIGN);
        assert_eq!(assigned.last().unwrap()[0], request.addr() + 0x34);
        assert_eq!(calls_to(&log, SOLUTION_DESTRUCT).len(), 1);
        assert_eq!(calls_to(&log, COVERED_MOVE_DESTRUCT).len(), 1);
    }

    #[test]
    fn covered_move_without_a_hit_drops_the_cover_location() {
        let (mut e, request, solution) = covered_move_setup(vec![0, 0]);
        e.call_log = Some(vec![]);
        assert!(e.call(0x006d_2c80, &args![request, solution]).bool());
        let log = log_of(&mut e);
        // Candidates 1 and 2 were checked.
        assert_eq!(calls_to(&log, COVERED_MOVE_CHECK).len(), 2);
        assert!(calls_to(&log, SOLUTION_COPY_TO).is_empty());
        // The cover location object is deleted and the request's pointer
        // cleared; the search info stays.
        let allocations = calls_to(&log, MEMORY_ALLOCATE);
        assert_eq!(allocations.len(), 2);
        assert_eq!(e.mem.u32(request.addr() + 0x104), 0);
        assert_ne!(e.mem.u32(request.addr() + 0x108), 0);
        assert_eq!(calls_to(&log, MEMORY_FREE).len(), 1);
        assert_eq!(calls_to(&log, LOCATION_DESTRUCT).len(), 4);
        assert_eq!(
            calls_to(&log, REQUEST_SET_FINISHED),
            vec![vec![request.addr(), 1]]
        );
        assert_timers_balanced(&log);
        // Per candidate the copy takes it as destination and the running
        // previous location follows it.
        assert_eq!(calls_to(&log, LOCATION_ASSIGN).len(), 4);
    }

    #[test]
    fn covered_move_tries_on_when_the_copy_cannot_be_solved() {
        let (mut e, request, solution) = covered_move_setup(vec![1, 1]);
        // Base solve of the request, the copy's first try fails, the
        // second try succeeds.
        let runs = Rc::new(RefCell::new(0u32));
        let counter = runs.clone();
        e.register_double(BASE_SOLVER_RUN, move |_, _| {
            *counter.borrow_mut() += 1;
            ret((*counter.borrow() != 2) as u32)
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x006d_2c80, &args![request, solution]).bool());
        let log = log_of(&mut e);
        assert_eq!(*runs.borrow(), 3);
        assert_eq!(calls_to(&log, SOLUTION_COPY_TO).len(), 1);
        assert_eq!(calls_to(&log, SOLUTION_DESTRUCT).len(), 2);
        assert_eq!(calls_to(&log, SOLUTION_NODE_AT).len(), 2);
    }

    #[test]
    fn covered_move_fails_when_the_plain_solve_fails() {
        let (mut e, request, solution) = covered_move_setup(vec![1]);
        e.register(BASE_SOLVER_RUN, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert!(!e.call(0x006d_2c80, &args![request, solution]).bool());
        let log = log_of(&mut e);
        assert!(calls_to(&log, MEMORY_ALLOCATE).is_empty());
        assert_timers_balanced(&log);
    }

    // -- covered-move request setters --------------------------------------

    #[test]
    fn covered_move_setters_store_and_clear_the_cover_pointers() {
        let mut e = engine();
        let request = Ptr::<()>::new(e.mem.alloc(0x120));
        let point_source = e.mem.alloc(12);
        set_point(&mut e, point_source, [1.5, 2.5, 3.5]);
        e.call(0x006d_30b0, &args![request, Ptr::<()>::new(point_source)]);
        assert_eq!(point(&e, request.addr() + 0xc8), [1.5, 2.5, 3.5]);
        e.call(0x006d_30e0, &args![request, 0xa1u32]);
        e.call(0x006d_3100, &args![request, 0xa2u32]);
        assert_eq!(e.mem.u32(request.addr() + 0x108), 0xa1);
        assert_eq!(e.mem.u32(request.addr() + 0x104), 0xa2);
        e.call(0x006d_3120, &args![request]);
        assert_eq!(e.mem.u32(request.addr() + 0x108), 0);
        assert_eq!(e.mem.u32(request.addr() + 0x104), 0);
    }

    // -- cover objects -----------------------------------------------------

    /// Fills `size` bytes at `address` with 0xaa.
    fn dirty(e: &mut Engine, address: u32, size: u32) {
        for i in 0..size {
            e.mem.set_u8(address + i, 0xaa);
        }
    }

    #[test]
    fn cover_location_constructor_initializes_every_field() {
        let mut e = engine();
        set_point(&mut e, ZERO_VECTOR, [0.0, 0.0, 0.0]);
        let object = e.mem.alloc(0x78);
        dirty(&mut e, object, 0x78);
        set_point(&mut e, ZERO_VECTOR, [1.0, 2.0, 3.0]);
        e.call_log = Some(vec![]);
        let back = e.call(0x006d_3150, &args![Ptr::<()>::new(object)]).u32();
        assert_eq!(back, object);
        let log = log_of(&mut e);
        assert_eq!(e.mem.u8(object), 0);
        assert_eq!(e.mem.u8(object + 1), 0);
        assert_eq!(e.mem.u8(object + 2), 0);
        assert_eq!(e.mem.u32(object + 4), 0);
        assert_eq!(point(&e, object + 8), [1.0, 2.0, 3.0]);
        assert_eq!(e.mem.u32(object + 0x68), 0);
        assert_eq!(e.mem.u32(object + 0x6c), u32::MAX);
        for i in 0..6 {
            assert_eq!(e.mem.u8(object + 0x70 + i), 0);
        }
        // The embedded `PathingCoverLocation` is constructed at +0x14.
        assert_eq!(
            calls_to(&log, LOCATION_CONSTRUCT),
            vec![vec![object + 0x14]]
        );
        assert_eq!(e.mem.u32(object + 0x14), PATHING_COVER_LOCATION_VTABLE);
    }

    #[test]
    fn pathing_cover_location_constructor_builds_the_base_and_three_points() {
        let mut e = engine();
        let object = e.mem.alloc(0x54);
        e.call_log = Some(vec![]);
        let back = e.call(0x006d_3200, &args![Ptr::<()>::new(object)]).u32();
        assert_eq!(back, object);
        let log = log_of(&mut e);
        assert_eq!(calls_to(&log, LOCATION_CONSTRUCT), vec![vec![object]]);
        assert_eq!(e.mem.u32(object), PATHING_COVER_LOCATION_VTABLE);
        assert_eq!(
            calls_to(&log, POINT3_DEFAULT_CONSTRUCTOR),
            vec![
                vec![object + 0x28],
                vec![object + 0x34],
                vec![object + 0x40]
            ]
        );
    }

    #[test]
    fn reservation_info_constructor_clears_its_six_bytes() {
        let mut e = engine();
        let object = e.mem.alloc(8);
        dirty(&mut e, object, 8);
        let back = e.call(0x006d_3280, &args![Ptr::<()>::new(object)]).u32();
        assert_eq!(back, object);
        for i in 0..6 {
            assert_eq!(e.mem.u8(object + i), 0);
        }
        assert_eq!(e.mem.u8(object + 6), 0xaa);
    }

    #[test]
    fn pathing_cover_location_destructor_destroys_the_location_base() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        e.call(0x006d_32c0, &args![Ptr::<()>::new(0x1234)]);
        let log = log_of(&mut e);
        assert_eq!(calls_to(&log, LOCATION_DESTRUCT), vec![vec![0x1234]]);
    }

    #[test]
    fn cover_search_info_constructor_initializes_every_field() {
        let mut e = engine();
        let object = e.mem.alloc(0x3c);
        dirty(&mut e, object, 0x3c);
        set_point(&mut e, ZERO_VECTOR, [4.0, 5.0, 6.0]);
        e.set_global(FLOAT_MAXIMUM, f32::MAX);
        e.call_log = Some(vec![]);
        let back = e.call(0x006d_32e0, &args![Ptr::<()>::new(object)]).u32();
        assert_eq!(back, object);
        let log = log_of(&mut e);
        assert_eq!(point(&e, object), [4.0, 5.0, 6.0]);
        assert_eq!(point(&e, object + 0xc), [4.0, 5.0, 6.0]);
        assert_eq!(e.mem.f32(object + 0x18), 0.0);
        assert_eq!(
            calls_to(&log, TIME_STAMP_CONSTRUCT),
            vec![vec![object + 0x1c, (-f32::MAX).to_bits()]]
        );
        assert_eq!(
            calls_to(&log, COVER_LOCATION_ARRAY_CONSTRUCT),
            vec![vec![object + 0x20]]
        );
        assert_eq!(e.mem.u32(object + 0x30), 0);
        assert_eq!(e.mem.u32(object + 0x34), 0);
        assert_eq!(e.mem.u8(object + 0x38), 0);
    }

    #[test]
    fn cover_location_deleting_destructor_frees_on_request() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        let back = e
            .call(0x006d_3370, &args![Ptr::<()>::new(0x5000), 1u32])
            .u32();
        assert_eq!(back, 0x5000);
        let log = log_of(&mut e);
        assert_eq!(calls_to(&log, LOCATION_DESTRUCT), vec![vec![0x5014]]);
        assert_eq!(calls_to(&log, MEMORY_FREE), vec![vec![0x5000]]);
        // Bit 0 clear: destroyed but not freed.
        e.call_log = Some(vec![]);
        e.call(0x006d_3370, &args![Ptr::<()>::new(0x5000), 2u32]);
        let log = log_of(&mut e);
        assert_eq!(calls_to(&log, LOCATION_DESTRUCT).len(), 1);
        assert!(calls_to(&log, MEMORY_FREE).is_empty());
    }

    #[test]
    fn cover_location_destructor_destroys_the_embedded_cover_location() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        e.call(0x006d_33a0, &args![Ptr::<()>::new(0x6000)]);
        let log = log_of(&mut e);
        assert_eq!(calls_to(&log, LOCATION_DESTRUCT), vec![vec![0x6014]]);
    }

    // -- 006d34d0 ----------------------------------------------------------

    /// A request on a resolvable navmesh whose random-point search finds a
    /// path ending on triangle 7; the triangle's corners are (0, 0, 0),
    /// (4, 0, 0) and (0, 4, 0) and the random point in it is (1, 1, 0).
    fn random_point_setup() -> (Engine, Ptr<PathingRequest>, Points) {
        let mut e = engine();
        let (request, _) = objects(&mut e);
        geometry(&mut e, request);
        e.register(RANDOM_POINT_SEARCH_BUILD_NODE_PATH, |_, _| ret(1));
        e.register(NAVMESH_GET_TRIANGLE, |_, a| {
            assert_eq!(a[..2], [LAST_NAVMESH, 7]);
            ret(0x00d5_0000)
        });
        e.register_double(TRIANGLE_GET_VERTEX_INDEX, |_, a| {
            assert_eq!(a[0], 0x00d5_0000);
            ret(0x20 + a[1])
        });
        let vertices = e.mem.alloc(36);
        set_point(&mut e, vertices, [0.0, 0.0, 0.0]);
        set_point(&mut e, vertices + 12, [4.0, 0.0, 0.0]);
        set_point(&mut e, vertices + 24, [0.0, 4.0, 0.0]);
        e.register_double(NAVMESH_GET_VERTEX, move |_, a| {
            assert_eq!(a[0], LAST_NAVMESH);
            ret(vertices + 12 * (a[1] - 0x20))
        });
        let corners = Rc::new(RefCell::new(Vec::new()));
        let sink = corners.clone();
        let random = e.mem.alloc(12);
        set_point(&mut e, random, [1.0, 1.0, 0.0]);
        e.register_double(TRIANGLE_RANDOM_POINT, move |e, a| {
            for corner in &a[1..4] {
                sink.borrow_mut().push(point(e, *corner));
            }
            ret(random)
        });
        (e, request, corners)
    }

    #[test]
    fn random_point_is_taken_on_the_last_triangle_of_the_path() {
        let (mut e, request, corners) = random_point_setup();
        let out = e.mem.alloc(12);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x006d_34d0, &args![request, Ptr::<()>::new(out)])
            .bool());
        let log = log_of(&mut e);
        assert_eq!(
            *corners.borrow(),
            vec![[0.0, 0.0, 0.0], [4.0, 0.0, 0.0], [0.0, 4.0, 0.0]]
        );
        assert_eq!(point(&e, out), [1.0, 1.0, 0.0]);
        // The three corners were default-constructed first.
        let iterator = &calls_to(&log, VECTOR_CONSTRUCTOR_ITERATOR)[0];
        assert_eq!(iterator[1..], [12, 3, POINT3_DEFAULT_CONSTRUCTOR]);
        assert_eq!(calls_to(&log, RANDOM_POINT_SEARCH_DESTRUCT).len(), 1);
        assert_eq!(calls_to(&log, NAV_HOLDER_RELEASE).len(), 1);
    }

    #[test]
    fn random_point_fails_without_a_path_or_a_navmesh() {
        let (mut e, request, _) = random_point_setup();
        e.register(RANDOM_POINT_SEARCH_BUILD_NODE_PATH, |_, _| ret(0));
        let out = e.mem.alloc(12);
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x006d_34d0, &args![request, Ptr::<()>::new(out)])
            .bool());
        let log = log_of(&mut e);
        assert_eq!(calls_to(&log, RANDOM_POINT_SEARCH_DESTRUCT).len(), 1);
        assert_eq!(calls_to(&log, NAV_HOLDER_RELEASE).len(), 1);
        assert!(calls_to(&log, TRIANGLE_RANDOM_POINT).is_empty());

        let (mut e, request, _) = random_point_setup();
        e.register(LOCATION_GET_NAVMESH_AND_TRIANGLE, |_, _| ret(0));
        assert!(!e
            .call(0x006d_34d0, &args![request, Ptr::<()>::new(out)])
            .bool());

        let (mut e, request, _) = random_point_setup();
        e.register(LOCATION_RESOLVE_CLOSEST, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x006d_34d0, &args![request, Ptr::<()>::new(out)])
            .bool());
        assert!(calls_to(&log_of(&mut e), NAV_HOLDER_CONSTRUCT).is_empty());
    }

    // -- 006d33c0 ----------------------------------------------------------

    #[test]
    fn random_destination_prefers_a_point_on_the_navmesh() {
        let (mut e, request, _) = random_point_setup();
        let out = e.mem.alloc(12);
        e.call_log = Some(vec![]);
        e.call(0x006d_33c0, &args![request, Ptr::<()>::new(out)]);
        let log = log_of(&mut e);
        assert_eq!(point(&e, out), [1.0, 1.0, 0.0]);
        assert!(calls_to(&log, RANDOM_ANGLE).is_empty());
    }

    #[test]
    fn random_destination_falls_back_to_a_point_on_a_circle() {
        let mut e = engine();
        let (request, _) = objects(&mut e);
        geometry(&mut e, request);
        vector_math(&mut e);
        // The navmesh search is not possible.
        e.register(LOCATION_RESOLVE_CLOSEST, |_, _| ret(0));
        e.mem.set_f32(request.addr() + 0xb0, 3.0);
        e.mem.set_f32(request.addr() + 0xb4, 9.0);
        e.register(RANDOM_ANGLE, |_, _| ret_float(std::f32::consts::FRAC_PI_2));
        // The radius is picked between the two distances.
        e.register_double(RANDOM_FLOAT, |_, a| {
            assert_eq!(a[..2], [3.0f32.to_bits(), 9.0f32.to_bits()]);
            ret_float(6.0)
        });
        e.register(ANGLE_FUNCTION_004E44D0, |_, a| {
            ret_float(float_arg(a[0]).cos())
        });
        e.register(ANGLE_FUNCTION_004E4490, |_, a| {
            ret_float(float_arg(a[0]).sin())
        });
        e.register(POINT3_CONSTRUCT, |e, a| {
            for i in 1..4 {
                e.mem.set_u32(a[0] + 4 * (i - 1), a[i as usize]);
            }
            ret(a[0])
        });
        let out = e.mem.alloc(12);
        // The origin has land at height 50.
        e.register(LOCATION_GET_CELL, |_, _| ret(0x00e0_0000));
        e.register(CELL_GET_LAND_HEIGHT, |e, a| {
            e.mem.set_f32(a[2], 50.0);
            ret(1)
        });
        e.call(0x006d_33c0, &args![request, Ptr::<()>::new(out)]);
        // (x, y) = (sin, cos) of the angle times the radius, from the
        // origin (1, 2); the height is the land height.
        assert_close(point(&e, out), [7.0, 2.0, 50.0]);

        // A cell without land keeps the origin's height.
        e.register(CELL_GET_LAND_HEIGHT, |_, _| ret(0));
        e.call(0x006d_33c0, &args![request, Ptr::<()>::new(out)]);
        assert_close(point(&e, out), [7.0, 2.0, 3.0]);

        // No cell: nothing to look up.
        e.register(LOCATION_GET_CELL, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(0x006d_33c0, &args![request, Ptr::<()>::new(out)]);
        assert!(calls_to(&log_of(&mut e), CELL_GET_LAND_HEIGHT).is_empty());
    }

    // -- 006d3780 ----------------------------------------------------------

    /// A reference whose vtable writes the corners (slot +0x1dc: the
    /// maximum, +0x1d8: the minimum) into its out parameter and returns it.
    fn reference_with_bounds(
        e: &mut Engine,
        vtable: u32,
        maximum: [f32; 3],
        minimum: [f32; 3],
    ) -> u32 {
        let reference = e.mem.alloc(0x10);
        let mut slots = vec![0u32; 0x1dc / 4 + 1];
        slots[0x1dc / 4] = vtable + 0x1000;
        slots[0x1d8 / 4] = vtable + 0x2000;
        e.put_vtable(vtable, &slots);
        e.register_double(vtable + 0x1000, move |e, a| {
            set_point(e, a[1], maximum);
            ret(a[1])
        });
        e.register_double(vtable + 0x2000, move |e, a| {
            set_point(e, a[1], minimum);
            ret(a[1])
        });
        e.mem.set_u32(reference, vtable);
        reference
    }

    #[test]
    fn safe_straight_line_sizes_the_request_from_both_references() {
        let mut e = engine();
        vector_math(&mut e);
        e.set_global(MINIMUM_RADIUS, 32.0f32);
        e.set_global(HALF, 0.5f64);
        e.set_global(NINE_TENTHS, 0.9f64);
        // The first reference is 40 wide at scale 1, the second 40 at
        // scale 2.
        let from = reference_with_bounds(&mut e, 0x00a2_0000, [50.0, 40.0, 0.0], [10.0, 0.0, 0.0]);
        let to =
            reference_with_bounds(&mut e, 0x00a3_0000, [100.0, 100.0, 0.0], [60.0, 100.0, 0.0]);
        e.register_double(REFERENCE_GET_SCALE, move |_, a| {
            ret_float(if a[0] == to { 2.0 } else { 1.0 })
        });
        // The locations built for the references are their own addresses.
        e.register(LOCATION_CONSTRUCT_AT_REFERENCE, |_, a| ret(a[0]));
        let request_address = Rc::new(RefCell::new(0u32));
        let remember = request_address.clone();
        e.register_double(SAFE_STRAIGHT_LINE_CONSTRUCT, move |_, a| {
            *remember.borrow_mut() = a[0];
            ret(a[0])
        });
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = seen.clone();
        e.register_double(SAFE_STRAIGHT_LINE_FIND, move |e, a| {
            // The origin radius is already stored in the request.
            sink.borrow_mut().push(e.mem.f32(a[0] + 0x68));
            ret(1)
        });
        let out = e.mem.alloc(12);
        e.call_log = Some(vec![]);
        let found = e
            .call(
                0x006d_3780,
                &args![
                    Ptr::<()>::new(from),
                    Ptr::<()>::new(to),
                    100.0f32,
                    Ptr::<()>::new(out)
                ],
            )
            .bool();
        assert!(found);
        let log = log_of(&mut e);
        let request = *request_address.borrow();
        assert_eq!(*seen.borrow(), vec![20.0]);
        // Origin and destination are assigned from the locations built at the
        // two references.
        let built = calls_to(&log, LOCATION_CONSTRUCT_AT_REFERENCE);
        assert_eq!(built[0][1], from);
        assert_eq!(built[1][1], to);
        assert_eq!(
            calls_to(&log, LOCATION_ASSIGN),
            vec![
                vec![request + 0xc, built[0][0]],
                vec![request + 0x34, built[1][0]]
            ]
        );
        // fTargetRadius 40, minimum 100 * 0.9 + 20, maximum 100 + 20.
        assert_eq!(
            calls_to(&log, REQUEST_SET_TARGET_RADIUS),
            vec![vec![request, 40.0f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&log, REQUEST_SET_MINIMUM_DISTANCE),
            vec![vec![request, 110.0f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&log, REQUEST_SET_MAXIMUM_DISTANCE),
            vec![vec![request, 120.0f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&log, SAFE_STRAIGHT_LINE_FIND),
            vec![vec![request, out, 0]]
        );
        assert_eq!(
            calls_to(&log, SAFE_STRAIGHT_LINE_DESTRUCT),
            vec![vec![request]]
        );
    }

    #[test]
    fn safe_straight_line_enforces_the_least_radius_and_passes_failure_on() {
        let mut e = engine();
        vector_math(&mut e);
        e.set_global(MINIMUM_RADIUS, 32.0f32);
        e.set_global(HALF, 0.5f64);
        e.set_global(NINE_TENTHS, 0.9f64);
        // Tiny references: the size is raised to 32, the radius is 16.
        let from = reference_with_bounds(&mut e, 0x00a2_0000, [1.0, 1.0, 0.0], [0.0, 0.0, 0.0]);
        let to = reference_with_bounds(&mut e, 0x00a3_0000, [1.0, 1.0, 0.0], [0.0, 0.0, 0.0]);
        e.register(REFERENCE_GET_SCALE, |_, _| ret_float(1.0));
        e.register(LOCATION_CONSTRUCT_AT_REFERENCE, |_, a| ret(a[0]));
        e.call_log = Some(vec![]);
        let found = e
            .call(
                0x006d_3780,
                &args![
                    Ptr::<()>::new(from),
                    Ptr::<()>::new(to),
                    10.0f32,
                    Ptr::<()>::new(0x1000)
                ],
            )
            .bool();
        // The find double returns zero.
        assert!(!found);
        let log = log_of(&mut e);
        assert_eq!(
            calls_to(&log, REQUEST_SET_TARGET_RADIUS)[0][1],
            16.0f32.to_bits()
        );
        assert_eq!(
            calls_to(&log, REQUEST_SET_MINIMUM_DISTANCE)[0][1],
            25.0f32.to_bits()
        );
    }

    // -- 006d3ae0 ----------------------------------------------------------

    #[test]
    fn actor_radius_setter_stores_the_float() {
        let mut e = engine();
        let (request, _) = objects(&mut e);
        e.call(0x006d_3ae0, &args![request, 12.5f32]);
        assert_eq!(e.get(request, PathingRequest::fActorRadius), 12.5);
    }

    // ======================================================================
    // Second batch: 006d3b00 to 006d7a20
    // ======================================================================

    /// A fake array of the game: where it keeps its elements and how many.
    #[derive(Clone, Copy)]
    struct FakeArray {
        storage: u32,
        stride: u32,
        count: u32,
    }

    /// The game's arrays, faked: the accessors every solver shares (element
    /// count, emptiness, element address) answer for each array a test has
    /// registered, over storage the array keeps in the engine's memory.
    #[derive(Clone, Default)]
    struct Arrays(Rc<RefCell<std::collections::HashMap<u32, FakeArray>>>);

    impl Arrays {
        /// Registers an array the test builds itself.
        fn make(&self, e: &mut Engine, array: u32, stride: u32) {
            let storage = e.mem.alloc(stride * 16);
            self.0.borrow_mut().insert(
                array,
                FakeArray {
                    storage,
                    stride,
                    count: 0,
                },
            );
        }

        /// Makes `construct` (ECX = the array) register a fake array of
        /// `stride`-byte elements, with room for 16.
        fn constructor(&self, e: &mut Engine, construct: u32, stride: u32) {
            let arrays = self.clone();
            e.register_double(construct, move |e, a| {
                arrays.make(e, a[0], stride);
                ret(a[0])
            });
        }

        /// `add` (ECX = array, stack: address of the element) appends a copy
        /// of the element.
        fn appender(&self, e: &mut Engine, add: u32) {
            let arrays = self.clone();
            e.register_double(add, move |e, a| {
                arrays.push(e, a[0], a[1]);
                Ret::default()
            });
        }

        /// `insert` (ECX = array, stack: index, address of the element)
        /// inserts a copy of the element at the index.
        fn inserter(&self, e: &mut Engine, insert: u32) {
            let arrays = self.clone();
            e.register_double(insert, move |e, a| {
                let item = arrays.0.borrow()[&a[0]];
                let index = a[1];
                for i in (index..item.count).rev() {
                    for word in 0..item.stride / 4 {
                        let value = e.mem.u32(item.storage + i * item.stride + 4 * word);
                        e.mem
                            .set_u32(item.storage + (i + 1) * item.stride + 4 * word, value);
                    }
                }
                for word in 0..item.stride / 4 {
                    let value = e.mem.u32(a[2] + 4 * word);
                    e.mem
                        .set_u32(item.storage + index * item.stride + 4 * word, value);
                }
                arrays.0.borrow_mut().get_mut(&a[0]).unwrap().count += 1;
                Ret::default()
            });
        }

        /// `remove` (ECX = array, stack: first, count, 0) removes entries.
        fn remover(&self, e: &mut Engine, remove: u32) {
            let arrays = self.clone();
            e.register_double(remove, move |e, a| {
                let item = arrays.0.borrow()[&a[0]];
                let (first, removed) = (a[1], a[2]);
                for i in first..item.count - removed {
                    for word in 0..item.stride / 4 {
                        let value = e
                            .mem
                            .u32(item.storage + (i + removed) * item.stride + 4 * word);
                        e.mem
                            .set_u32(item.storage + i * item.stride + 4 * word, value);
                    }
                }
                arrays.0.borrow_mut().get_mut(&a[0]).unwrap().count -= removed;
                Ret::default()
            });
        }

        /// Appends a copy of the `stride` bytes at `source`; returns the
        /// index.
        fn push(&self, e: &mut Engine, array: u32, source: u32) -> u32 {
            let mut map = self.0.borrow_mut();
            let item = map.get_mut(&array).expect("a registered array");
            let index = item.count;
            for word in 0..item.stride / 4 {
                let value = e.mem.u32(source + 4 * word);
                e.mem
                    .set_u32(item.storage + index * item.stride + 4 * word, value);
            }
            item.count += 1;
            index
        }

        /// Appends one word.
        fn push_word(&self, e: &mut Engine, array: u32, value: u32) -> u32 {
            let scratch = e.mem.alloc(4);
            e.mem.set_u32(scratch, value);
            self.push(e, array, scratch)
        }

        fn count(&self, array: u32) -> u32 {
            self.0.borrow().get(&array).map_or(0, |item| item.count)
        }

        /// The address of the element.
        fn element(&self, array: u32, index: u32) -> u32 {
            let item = self.0.borrow()[&array];
            item.storage + index * item.stride
        }

        /// Registers the element count, the emptiness test and the given
        /// element getters (ECX = array, stack: index) for every array.
        fn accessors(&self, e: &mut Engine, getters: &[u32]) {
            let arrays = self.clone();
            e.register_double(ARRAY_COUNT, move |_, a| ret(arrays.count(a[0])));
            let arrays = self.clone();
            e.register_double(ARRAY_IS_EMPTY, move |_, a| {
                ret((arrays.count(a[0]) == 0) as u32)
            });
            for getter in getters {
                let arrays = self.clone();
                e.register_double(*getter, move |_, a| ret(arrays.element(a[0], a[1])));
            }
        }
    }

    /// A navmesh the second batch's tests use, and the address its triangle
    /// records start at (a record is `TRIANGLE_RECORDS + 0x10 * triangle`).
    const NAV: u32 = 0x00c0_0300;
    const TRIANGLE_RECORDS: u32 = 0x00d6_0000;

    /// Writes the three corners (0, 0, 0), (10, 0, 0) and (0, 10, 0) and
    /// returns their address.
    fn corner_points(e: &mut Engine) -> u32 {
        let corners = e.mem.alloc(36);
        set_point(e, corners, [0.0, 0.0, 0.0]);
        set_point(e, corners + 12, [10.0, 0.0, 0.0]);
        set_point(e, corners + 24, [0.0, 10.0, 0.0]);
        corners
    }

    /// `cdecl (out, point, a, b)`: the closest point of the segment `a b`
    /// to `point`.
    fn segment_closest(e: &mut Engine, a: &[u32]) -> Ret {
        let (point, start, end) = (point(e, a[1]), point(e, a[2]), point(e, a[3]));
        let direction = [end[0] - start[0], end[1] - start[1], end[2] - start[2]];
        let length_squared: f32 = direction.iter().map(|d| d * d).sum();
        let along: f32 = (0..3)
            .map(|i| (point[i] - start[i]) * direction[i])
            .sum::<f32>()
            / length_squared;
        let along = along.clamp(0.0, 1.0);
        let closest = [
            start[0] + along * direction[0],
            start[1] + along * direction[1],
            start[2] + along * direction[2],
        ];
        set_point(e, a[0], closest);
        ret(a[0])
    }

    /// `cdecl (point, a, b, flag)`, `ST0`: the distance from `point` to the
    /// segment `a b`.
    fn segment_distance(e: &mut Engine, a: &[u32]) -> Ret {
        let scratch = e.mem.alloc(12);
        segment_closest(e, &[scratch, a[0], a[1], a[2]]);
        let (point, closest) = (point(e, a[0]), point(e, scratch));
        ret_float(
            (0..3)
                .map(|i| (point[i] - closest[i]).powi(2))
                .sum::<f32>()
                .sqrt(),
        )
    }

    // -- 006d3b00 and 006d3b20 ---------------------------------------------

    #[test]
    fn target_radius_setter_stores_the_float() {
        let mut e = engine();
        let (request, _) = objects(&mut e);
        fn_006d3b00(&mut e, request, 6.5);
        assert_eq!(e.get(request, PathingRequest::fTargetRadius), 6.5);
        assert_eq!(e.mem.f32(request.addr() + 0x74), 6.5);
    }

    #[test]
    fn safe_straight_line_destructor_runs_the_base_destructor() {
        let mut e = engine();
        let (request, _) = objects(&mut e);
        e.call_log = Some(vec![]);
        fn_006d3b20(&mut e, request);
        assert_eq!(
            log_of(&mut e),
            vec![(REQUEST_BASE_DESTRUCT, vec![request.addr()])]
        );
    }

    // -- 006d3b40 and 006d3ff0 ---------------------------------------------

    /// `random_point_setup` (a resolvable request whose random-point search
    /// works) with the doubles of `fn_006d3b40`: the point search finds
    /// nothing, the base solver succeeds and the solution query fails.
    fn safe_line_setup() -> (Engine, Ptr<PathingRequest>) {
        let (mut e, request, _) = random_point_setup();
        e.register(POINT_SEARCH_RUN, |_, _| ret(0));
        e.register(BASE_SOLVER_RUN, |_, _| ret(1));
        e.register(SOLUTION_NODE_COUNT, |_, _| ret(5));
        e.register(SOLUTION_QUERY_006E7E70, |_, _| ret(0));
        e.mem.set_f32(request.addr() + 0xb0, 7.5);
        e.mem.set_f32(request.addr() + 0xb4, 20.0);
        (e, request)
    }

    #[test]
    fn safe_line_point_search_result_is_copied_out() {
        let (mut e, request) = safe_line_setup();
        let found = Rc::new(RefCell::new(Vec::new()));
        let sink = found.clone();
        e.register_double(POINT_SEARCH_RUN, move |e, a| {
            // The point the search found is at search + 0x20cc.
            set_point(e, a[0] + 0x20cc, [1.0, 2.0, 3.0]);
            sink.borrow_mut()
                .push((a.to_vec(), e.mem.u32(a[2]), e.mem.u16(a[2] + 4)));
            ret(1)
        });
        let out = e.mem.alloc(12);
        e.call_log = Some(vec![]);
        assert!(fn_006d3b40(&mut e, request, Ptr::new(out)));
        let log = log_of(&mut e);
        assert_eq!(point(&e, out), [1.0, 2.0, 3.0]);
        // The search got the request and the start node (navmesh and
        // triangle of the origin).
        let found = found.borrow();
        assert_eq!(found[0].0[1], request.addr());
        assert_eq!((found[0].1, found[0].2), (NAVMESH, 9));
        // Nothing else was tried; the list, the search and the two holders
        // were released in that order.
        assert!(calls_to(&log, REQUEST_CONSTRUCT).is_empty());
        let order: Vec<u32> = log
            .iter()
            .map(|(callee, _)| *callee)
            .filter(|callee| {
                [
                    NAVMESH_LIST_DESTRUCT,
                    HIDE_SEARCH_DESTRUCT,
                    NAV_HOLDER_RELEASE,
                ]
                .contains(callee)
            })
            .collect();
        assert_eq!(
            order,
            vec![
                NAVMESH_LIST_DESTRUCT,
                HIDE_SEARCH_DESTRUCT,
                NAV_HOLDER_RELEASE,
                NAV_HOLDER_RELEASE
            ]
        );
    }

    #[test]
    fn safe_line_asks_the_solution_for_a_point() {
        let (mut e, request) = safe_line_setup();
        let query = Rc::new(RefCell::new(Vec::new()));
        let sink = query.clone();
        e.register_double(SOLUTION_QUERY_006E7E70, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            ret(1)
        });
        let out = e.mem.alloc(12);
        e.call_log = Some(vec![]);
        assert!(fn_006d3b40(&mut e, request, Ptr::new(out)));
        let log = log_of(&mut e);
        // The last node index (5 nodes), the minimum distance, 0 and the
        // out point.
        let query = query.borrow();
        assert_eq!(query[0][1..4], [4.0f32.to_bits(), 7.5f32.to_bits(), 0u32]);
        assert_eq!(query[0][5], out);
        let solution = query[0][0];
        // The copy of the request is built, solved and destroyed with the
        // solution; no close-point request is needed.
        let copy = calls_to(&log, REQUEST_CONSTRUCT)[0][0];
        assert_eq!(
            calls_to(&log, REQUEST_COPY_TO),
            vec![vec![request.addr(), copy]]
        );
        assert_eq!(calls_to(&log, SOLUTION_DESTRUCT), vec![vec![solution]]);
        assert_eq!(calls_to(&log, REQUEST_COPY_DESTRUCT), vec![vec![copy]]);
        assert!(calls_to(&log, CLOSE_POINT_REQUEST_CONSTRUCT).is_empty());
    }

    #[test]
    fn safe_line_falls_back_to_a_random_point_near_the_destination() {
        let (mut e, request) = safe_line_setup();
        let out = e.mem.alloc(12);
        e.call_log = Some(vec![]);
        assert!(fn_006d3b40(&mut e, request, Ptr::new(out)));
        let log = log_of(&mut e);
        // The random point (1, 1, 0) of the triangle of the close-point
        // request, which starts at the destination and keeps the
        // distances.
        assert_eq!(point(&e, out), [1.0, 1.0, 0.0]);
        let close = calls_to(&log, CLOSE_POINT_REQUEST_CONSTRUCT)[0][0];
        assert_eq!(
            calls_to(&log, LOCATION_ASSIGN),
            vec![vec![close + 0xc, request.addr() + 0x34]]
        );
        assert_eq!(
            calls_to(&log, REQUEST_SET_MINIMUM_DISTANCE),
            vec![vec![close, 7.5f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&log, REQUEST_SET_MAXIMUM_DISTANCE),
            vec![vec![close, 20.0f32.to_bits()]]
        );
        assert_eq!(calls_to(&log, REQUEST_BASE_DESTRUCT), vec![vec![close]]);
        // Both requests, the solution and the holders are cleaned up.
        assert_eq!(calls_to(&log, SOLUTION_DESTRUCT).len(), 1);
        assert_eq!(calls_to(&log, REQUEST_COPY_DESTRUCT).len(), 1);
        assert_eq!(calls_to(&log, NAVMESH_LIST_DESTRUCT).len(), 2);
    }

    #[test]
    fn safe_line_fails_when_nothing_finds_a_point() {
        // The base solver fails: no solution query, no fallback.
        let (mut e, request) = safe_line_setup();
        e.register(BASE_SOLVER_RUN, |_, _| ret(0));
        let out = e.mem.alloc(12);
        e.call_log = Some(vec![]);
        assert!(!fn_006d3b40(&mut e, request, Ptr::new(out)));
        let log = log_of(&mut e);
        assert!(calls_to(&log, SOLUTION_QUERY_006E7E70).is_empty());
        assert!(calls_to(&log, CLOSE_POINT_REQUEST_CONSTRUCT).is_empty());
        assert_eq!(calls_to(&log, SOLUTION_DESTRUCT).len(), 1);

        // The random point search fails too.
        let (mut e, request) = safe_line_setup();
        e.register(RANDOM_POINT_SEARCH_BUILD_NODE_PATH, |_, _| ret(0));
        assert!(!fn_006d3b40(&mut e, request, Ptr::new(out)));

        // The origin or the destination does not resolve.
        let (mut e, request) = safe_line_setup();
        e.register(LOCATION_RESOLVE_CLOSEST, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert!(!fn_006d3b40(&mut e, request, Ptr::new(out)));
        assert!(calls_to(&log_of(&mut e), NAV_HOLDER_CONSTRUCT).is_empty());
        let (mut e, request) = safe_line_setup();
        let resolved = Rc::new(RefCell::new(0));
        let counter = resolved.clone();
        e.register_double(LOCATION_RESOLVE_CLOSEST, move |_, _| {
            *counter.borrow_mut() += 1;
            ret((*counter.borrow() < 2) as u32)
        });
        e.call_log = Some(vec![]);
        assert!(!fn_006d3b40(&mut e, request, Ptr::new(out)));
        let log = log_of(&mut e);
        assert_eq!(calls_to(&log, NAV_HOLDER_RELEASE).len(), 1);
        assert!(calls_to(&log, POINT_SEARCH_RUN).is_empty());
        // Neither triangle of origin or destination.
        let (mut e, request) = safe_line_setup();
        e.register(LOCATION_GET_NAVMESH_AND_TRIANGLE, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert!(!fn_006d3b40(&mut e, request, Ptr::new(out)));
        assert_eq!(calls_to(&log_of(&mut e), NAV_HOLDER_RELEASE).len(), 1);
    }

    #[test]
    fn found_point_is_copied_from_the_search() {
        let mut e = engine();
        let search = e.mem.alloc(0x2100);
        set_point(&mut e, search + 0x20cc, [4.0, 5.0, 6.0]);
        let out = e.mem.alloc(12);
        fn_006d3ff0(&mut e, Ptr::new(search), Ptr::new(out));
        assert_eq!(point(&e, out), [4.0, 5.0, 6.0]);
    }

    // -- 006d4020 ----------------------------------------------------------

    /// A 0x20-byte candidate with a distance (+0x18) and a height (+0x1c).
    fn candidate(e: &mut Engine, distance: f32, height: f32) -> Ptr {
        let item = Ptr::new(e.mem.alloc(0x20));
        e.mem.set_f32(item.addr() + 0x18, distance);
        e.mem.set_f32(item.addr() + 0x1c, height);
        item
    }

    #[test]
    fn candidate_comparison_puts_the_height_band_first() {
        let mut e = engine();
        e.set_global(HEIGHT_BAND_MINIMUM, -64.0f64);
        e.set_global(HEIGHT_BAND_MAXIMUM, 180.0f64);
        let in_band = candidate(&mut e, 50.0, 10.0);
        let above = candidate(&mut e, 1.0, 180.0);
        let below = candidate(&mut e, 1.0, -64.0);
        let unordered = candidate(&mut e, 1.0, f32::NAN);
        assert_eq!(fn_006d4020(&mut e, in_band, above), -1);
        assert_eq!(fn_006d4020(&mut e, above, in_band), 1);
        // The band is open at both ends and does not contain NaN.
        assert_eq!(fn_006d4020(&mut e, in_band, below), -1);
        assert_eq!(fn_006d4020(&mut e, unordered, in_band), 1);
        // Outside the band the distance decides.
        assert_eq!(fn_006d4020(&mut e, above, below), 0);
        let farther = candidate(&mut e, 9.0, 500.0);
        assert_eq!(fn_006d4020(&mut e, above, farther), -1);
        assert_eq!(fn_006d4020(&mut e, farther, above), 1);
    }

    #[test]
    fn candidate_comparison_orders_by_distance_within_the_band() {
        let mut e = engine();
        e.set_global(HEIGHT_BAND_MINIMUM, -64.0f64);
        e.set_global(HEIGHT_BAND_MAXIMUM, 180.0f64);
        let near = candidate(&mut e, 2.0, 0.0);
        let far = candidate(&mut e, 3.0, 100.0);
        let same = candidate(&mut e, 2.0, 50.0);
        assert_eq!(fn_006d4020(&mut e, near, far), -1);
        assert_eq!(fn_006d4020(&mut e, far, near), 1);
        assert_eq!(fn_006d4020(&mut e, near, same), 0);
    }

    // -- 006d4120 ----------------------------------------------------------

    /// Doubles for the nearby-point searches (`fn_006d4120`, and
    /// `fn_006d4570` for a location without navmesh info): one navmesh with
    /// one triangle (0, 0, 0), (10, 0, 0), (0, 10, 0) whose edge 0 has no
    /// neighbour (edges 1 and 2 have), the location at (5, 3, 0) with
    /// no navmesh info, the radius setting `radius`, a triangle center
    /// (3, 3, 0) and the fraction 0.1. Returns the location.
    fn nearby_setup(e: &mut Engine, arrays: &Arrays, radius: f32) -> u32 {
        vector_math(e);
        let location = e.mem.alloc(0x28);
        let setting = e.mem.alloc(4);
        e.mem.set_f32(setting, radius);
        e.register_double(SETTING_FLOAT_ADDRESS, move |_, a| {
            assert_eq!(a[0], NEARBY_RADIUS_SETTING);
            ret(setting)
        });
        arrays.constructor(e, NAVMESH_LIST_CONSTRUCT, 4);
        arrays.constructor(e, CANDIDATE_ARRAY_CONSTRUCT, 0x20);
        arrays.appender(e, CANDIDATE_ARRAY_ADD);
        arrays.accessors(e, &[POINTER_ARRAY_ELEMENT, CANDIDATE_ARRAY_ELEMENT]);
        let entry = e.mem.alloc(4);
        let list_owner = arrays.clone();
        e.register_double(NEARBY_NAVMESHES_FILL, move |e, a| {
            list_owner.push(e, a[2], entry);
            Ret::default()
        });
        e.register(NI_POINTER_GET_FROM_FIELD, |_, _| ret(NAV));
        e.register(NAVMESH_TRIANGLE_COUNT, |_, _| ret(1));
        e.register(NAVMESH_GET_TRIANGLE, |_, a| {
            ret(TRIANGLE_RECORDS + 0x10 * a[1])
        });
        e.register(TRIANGLE_NEIGHBOUR, |_, a| {
            ret(if a[1] == 0 { 0xffff } else { 7 })
        });
        e.register(TRIANGLE_GET_VERTEX_INDEX, |_, a| ret(a[1]));
        let corners = corner_points(e);
        e.register_double(NAVMESH_GET_VERTEX, move |_, a| ret(corners + 12 * a[1]));
        e.register(LOCATION_COPY_POSITION, |e, a| {
            set_point(e, a[1], [5.0, 3.0, 0.0]);
            ret(a[1])
        });
        e.register(POINT_SEGMENT_CLOSEST, segment_closest);
        e.register(POINT_PLANAR_LENGTH_SQUARED, |e, a| {
            let p = point(e, a[0]);
            ret_float(p[0] * p[0] + p[1] * p[1])
        });
        e.register(NAVMESH_GET_CENTER, |e, a| {
            set_point(e, a[1], [3.0, 3.0, 0.0]);
            ret(a[1])
        });
        e.set_global(CENTER_FRACTION, 0.1f32);
        e.register(MINIMUM_UNSIGNED, |_, a| ret(a[0].min(a[1])));
        e.register(CANDIDATE_TO_PATH_POINT, |_, a| ret(a[1]));
        location
    }

    #[test]
    fn nearby_points_record_a_candidate_per_border_edge_within_the_radius() {
        let mut e = engine();
        let arrays = Arrays::default();
        let location = nearby_setup(&mut e, &arrays, 10.0);
        let out = e.mem.alloc(16);
        arrays.make(&mut e, out, 0x20);
        arrays.appender(&mut e, PATH_POINT_ARRAY_ADD);
        let sorted = Rc::new(RefCell::new(Vec::new()));
        let sink = sorted.clone();
        e.register_double(QSORT, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = fn_006d4120(&mut e, Ptr::new(location), 5, Ptr::new(out));
        let log = log_of(&mut e);
        assert_eq!(result, 2);
        // The radius setting was read and squared for the comparison; the
        // candidate array was sorted with the comparison function.
        let sorted = sorted.borrow();
        assert_eq!(sorted.len(), 1);
        assert_eq!(sorted[0][1..], [1, 0x20, 0x006d_4020]);
        // Candidate: navmesh, triangle, edge, the closest point (5, 0, 0)
        // moved a tenth of the way to the center (3, 3, 0), distance 9
        // and the height difference 0.
        assert_eq!(arrays.count(out), 1);
        let item = arrays.element(out, 0);
        assert_eq!(
            (e.mem.u32(item), e.mem.u16(item + 4), e.mem.u32(item + 8)),
            (NAV, 0, 0)
        );
        assert_close(point(&e, item + 0xc), [4.8, 0.3, 0.0]);
        assert_eq!((e.mem.f32(item + 0x18), e.mem.f32(item + 0x1c)), (9.0, 0.0));
        // The nearby navmeshes were listed for the location and radius.
        assert_eq!(
            calls_to(&log, NEARBY_NAVMESHES_FILL)[0][..2],
            [location, 10.0f32.to_bits()]
        );
        assert_eq!(calls_to(&log, NAV_HOLDER_RELEASE).len(), 1);
        assert_eq!(calls_to(&log, CANDIDATE_ARRAY_DESTRUCT).len(), 1);
        assert_eq!(calls_to(&log, NAVMESH_LIST_DESTRUCT).len(), 1);
    }

    #[test]
    fn nearby_points_respect_the_limit_and_the_radius() {
        // Beyond the radius (3 squared is below 9): no candidate, no sort.
        let mut e = engine();
        let arrays = Arrays::default();
        let location = nearby_setup(&mut e, &arrays, 2.0);
        let out = e.mem.alloc(16);
        arrays.make(&mut e, out, 0x20);
        arrays.appender(&mut e, PATH_POINT_ARRAY_ADD);
        e.call_log = Some(vec![]);
        assert_eq!(fn_006d4120(&mut e, Ptr::new(location), 5, Ptr::new(out)), 0);
        let log = log_of(&mut e);
        assert!(calls_to(&log, QSORT).is_empty());
        assert_eq!(arrays.count(out), 0);
        assert_eq!(calls_to(&log, CANDIDATE_ARRAY_DESTRUCT).len(), 1);

        // A limit of zero takes none of the candidates but reports none.
        let mut e = engine();
        let arrays = Arrays::default();
        let location = nearby_setup(&mut e, &arrays, 10.0);
        let out = e.mem.alloc(16);
        arrays.make(&mut e, out, 0x20);
        arrays.appender(&mut e, PATH_POINT_ARRAY_ADD);
        e.register(QSORT, |_, _| Ret::default());
        assert_eq!(fn_006d4120(&mut e, Ptr::new(location), 0, Ptr::new(out)), 0);
        assert_eq!(arrays.count(out), 0);
    }

    #[test]
    fn nearby_points_stop_at_once_for_a_location_with_navmesh_info() {
        let mut e = engine();
        let arrays = Arrays::default();
        let location = nearby_setup(&mut e, &arrays, 10.0);
        e.register(LOCATION_HAS_NAVMESH_INFO, |_, _| ret(1));
        e.call_log = Some(vec![]);
        assert_eq!(fn_006d4120(&mut e, Ptr::new(location), 5, Ptr::new(0)), 1);
        let log = log_of(&mut e);
        assert_eq!(
            log,
            vec![
                (LOCATION_RESOLVE_NAVMESH_INFO, vec![location, 0]),
                (LOCATION_HAS_NAVMESH_INFO, vec![location])
            ]
        );
    }

    // -- 006d4550 ----------------------------------------------------------

    #[test]
    fn path_point_constructor_returns_this() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        assert_eq!(fn_006d4550(&mut e, Ptr::new(0x1234)), Ptr::new(0x1234));
        assert_eq!(log_of(&mut e), vec![(PATH_POINT_CONSTRUCT, vec![0x1234])]);
    }

    // -- 006d4ca0 and 006d4ce0 ---------------------------------------------

    #[test]
    fn triangle_references_are_equal_by_navmesh_and_triangle() {
        let mut e = engine();
        let a = e.mem.alloc(16);
        let b = e.mem.alloc(16);
        for (item, filler) in [(a, 0x1111u16), (b, 0x2222)] {
            e.mem.set_u32(item, NAV);
            e.mem.set_u16(item + 4, 3);
            // The bytes after the triangle are not compared.
            e.mem.set_u16(item + 6, filler);
        }
        assert!(fn_006d4ca0(&mut e, Ptr::new(a), Ptr::new(b)));
        e.mem.set_u16(b + 4, 4);
        assert!(!fn_006d4ca0(&mut e, Ptr::new(a), Ptr::new(b)));
        e.mem.set_u16(b + 4, 3);
        e.mem.set_u32(b, NAV + 4);
        assert!(!fn_006d4ca0(&mut e, Ptr::new(a), Ptr::new(b)));
    }

    #[test]
    fn triangle_record_assignment_copies_the_reference_and_the_marks() {
        let mut e = engine();
        let this = e.mem.alloc(16);
        let other = e.mem.alloc(16);
        e.mem.set_u16(other + 8, 0x0101);
        e.mem.set_u8(other + 10, 1);
        e.call_log = Some(vec![]);
        assert_eq!(
            fn_006d4ce0(&mut e, Ptr::new(this), Ptr::new(other)),
            Ptr::new(this)
        );
        assert_eq!(
            log_of(&mut e),
            vec![(TRIANGLE_REFERENCE_ASSIGN, vec![this, other])]
        );
        assert_eq!(e.mem.u16(this + 8), 0x0101);
        assert_eq!(e.mem.u8(this + 10), 1);
    }

    // -- 006d4570 ----------------------------------------------------------

    #[test]
    fn point_without_navmesh_info_is_the_nearby_point() {
        let mut e = engine();
        let arrays = Arrays::default();
        let location = nearby_setup(&mut e, &arrays, 10.0);
        arrays.constructor(&mut e, PATH_POINT_ARRAY_CONSTRUCT, 0x20);
        arrays.appender(&mut e, PATH_POINT_ARRAY_ADD);
        arrays.accessors(
            &mut e,
            &[
                POINTER_ARRAY_ELEMENT,
                CANDIDATE_ARRAY_ELEMENT,
                PATH_POINT_ARRAY_ELEMENT,
            ],
        );
        e.register(QSORT, |_, _| Ret::default());
        e.register(PATH_POINT_ASSIGN, |e, a| {
            for word in 0..6 {
                let value = e.mem.u32(a[1] + 4 * word);
                e.mem.set_u32(a[0] + 4 * word, value);
            }
            ret(a[0])
        });
        let out = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert!(fn_006d4570(&mut e, Ptr::new(location), 3.0, Ptr::new(out)));
        let log = log_of(&mut e);
        // Navmesh, triangle, edge and the point of the one candidate.
        assert_eq!(
            (e.mem.u32(out), e.mem.u16(out + 4), e.mem.u32(out + 8)),
            (NAV, 0, 0)
        );
        assert_close(point(&e, out + 0xc), [4.8, 0.3, 0.0]);
        // One point was asked for, and the array that held it is destroyed.
        let found = calls_to(&log, PATH_POINT_ARRAY_CONSTRUCT)[0][0];
        assert_eq!(calls_to(&log, PATH_POINT_ARRAY_DESTRUCT), vec![vec![found]]);
        assert_eq!(calls_to(&log, MINIMUM_UNSIGNED)[0][..], [1, 1]);

        // No candidate: no point.
        let mut e = engine();
        let arrays = Arrays::default();
        let location = nearby_setup(&mut e, &arrays, 2.0);
        arrays.constructor(&mut e, PATH_POINT_ARRAY_CONSTRUCT, 0x20);
        arrays.appender(&mut e, PATH_POINT_ARRAY_ADD);
        arrays.accessors(
            &mut e,
            &[
                POINTER_ARRAY_ELEMENT,
                CANDIDATE_ARRAY_ELEMENT,
                PATH_POINT_ARRAY_ELEMENT,
            ],
        );
        e.call_log = Some(vec![]);
        assert!(!fn_006d4570(&mut e, Ptr::new(location), 3.0, Ptr::new(out)));
        assert!(calls_to(&log_of(&mut e), PATH_POINT_ASSIGN).is_empty());
    }

    /// Doubles for the flood of `fn_006d4570` over a location with
    /// navmesh info: triangle 2 of the navmesh (corners (0, 0, 0),
    /// (10, 0, 0), (0, 10, 0)) holds the location at (5, 3, 0); across its
    /// edge 1 is triangle 4, which has no neighbours. Returns the
    /// location.
    fn flood_setup(e: &mut Engine, arrays: &Arrays) -> u32 {
        vector_math(e);
        let location = e.mem.alloc(0x28);
        e.register(LOCATION_HAS_NAVMESH_INFO, |_, _| ret(1));
        e.register(LOCATION_GET_NAVMESH_AND_TRIANGLE, |e, a| {
            e.mem.set_u16(a[2], 2);
            ret(1)
        });
        e.register(NAV_HOLDER_GET, |_, _| ret(NAV));
        e.register(NI_POINTER_GET_FROM_FIELD, |_, _| ret(NAV));
        e.register(NAVMESH_GET_INFO, |_, a| ret(a[0] + 0x1000));
        e.register(TRIANGLE_REFERENCE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u16(a[0] + 4, a[2] as u16);
            ret(a[0])
        });
        e.register(TRIANGLE_REFERENCE_ASSIGN, |e, a| {
            let (navmesh, triangle) = (e.mem.u32(a[1]), e.mem.u16(a[1] + 4));
            e.mem.set_u32(a[0], navmesh);
            e.mem.set_u16(a[0] + 4, triangle);
            ret(a[0])
        });
        let corners = corner_points(e);
        e.register_double(NAVMESH_EDGE_POINTS, move |e, a| {
            let (first, second) = (
                point(e, corners + 12 * a[2]),
                point(e, corners + 12 * ((a[2] + 1) % 3)),
            );
            set_point(e, a[3], first);
            set_point(e, a[4], second);
            Ret::default()
        });
        e.register(LOCATION_COPY_POSITION, |e, a| {
            set_point(e, a[1], [5.0, 3.0, 0.0]);
            ret(a[1])
        });
        e.register(POINT_SEGMENT_DISTANCE, segment_distance);
        e.register(POINT_SEGMENT_CLOSEST, segment_closest);
        e.register(NAVMESH_GET_TRIANGLE, |_, a| {
            ret(TRIANGLE_RECORDS + 0x10 * a[1])
        });
        e.register(TRIANGLE_NEIGHBOUR, |_, a| {
            ret(if a[0] == TRIANGLE_RECORDS + 0x20 && a[1] == 1 {
                4
            } else {
                0xffff
            })
        });
        e.register(EDGE_REFERENCE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u16(a[0] + 4, a[2] as u16);
            e.mem.set_u32(a[0] + 8, a[3]);
            ret(a[0])
        });
        e.register(EDGE_REFERENCE_ASSIGN, |e, a| {
            for word in 0..3 {
                let value = e.mem.u32(a[1] + 4 * word);
                e.mem.set_u32(a[0] + 4 * word, value);
            }
            ret(a[0])
        });
        e.register(NAVMESH_GET_MATCHING_EDGE, |e, a| {
            // Only the edge 1 of triangle 2 has a neighbour: triangle 4.
            assert_eq!(
                (e.mem.u32(a[1]), e.mem.u16(a[1] + 4), e.mem.u32(a[1] + 8)),
                (NAV, 2, 1)
            );
            e.mem.set_u32(a[2], NAV);
            e.mem.set_u16(a[2] + 4, 4);
            ret(1)
        });
        arrays.constructor(e, TRIANGLE_ARRAY_CONSTRUCT, 0xc);
        arrays.appender(e, TRIANGLE_ARRAY_ADD);
        arrays.constructor(e, EDGE_ARRAY_CONSTRUCT, 0xc);
        arrays.appender(e, EDGE_ARRAY_ADD);
        arrays.accessors(e, &[ARRAY_ELEMENT_006A1440]);
        e.set_global(FLOAT_LARGEST, f32::MAX);
        location
    }

    #[test]
    fn flood_finds_the_border_edge_nearest_to_the_location() {
        let mut e = engine();
        let arrays = Arrays::default();
        let location = flood_setup(&mut e, &arrays);
        let out = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        // The radius 2 reaches only the edge 1 (1.41 away): triangle 2
        // continues into triangle 4, whose edge 1 is a border.
        assert!(fn_006d4570(&mut e, Ptr::new(location), 2.0, Ptr::new(out)));
        let log = log_of(&mut e);
        // Navmesh info, triangle 4, edge 1 and the closest point (6, 4, 0).
        assert_eq!(
            (e.mem.u32(out), e.mem.u16(out + 4), e.mem.u32(out + 8)),
            (NAV + 0x1000, 4, 1)
        );
        assert_close(point(&e, out + 0xc), [6.0, 4.0, 0.0]);
        // The visited triangles carry the edge marks.
        let visited = calls_to(&log, TRIANGLE_ARRAY_CONSTRUCT)[0][0];
        assert_eq!(arrays.count(visited), 2);
        for (index, triangle) in [(0, 2u16), (1, 4)] {
            let record = arrays.element(visited, index);
            assert_eq!((e.mem.u32(record), e.mem.u16(record + 4)), (NAV, triangle));
            assert_eq!(
                [
                    e.mem.u8(record + 8),
                    e.mem.u8(record + 9),
                    e.mem.u8(record + 10)
                ],
                [0, 1, 0]
            );
        }
        let border = calls_to(&log, EDGE_ARRAY_CONSTRUCT)[0][0];
        assert_eq!(arrays.count(border), 1);
        // The three arrays and the holder are released.
        assert_eq!(calls_to(&log, NAV_HOLDER_RELEASE).len(), 1);
        assert_eq!(calls_to(&log, TRIANGLE_ARRAY_DESTRUCT), vec![vec![visited]]);
        assert_eq!(calls_to(&log, EDGE_ARRAY_DESTRUCT), vec![vec![border]]);
    }

    #[test]
    fn flood_fails_without_a_border_edge_or_a_triangle() {
        // A radius that reaches no edge: nothing to follow, nothing found.
        let mut e = engine();
        let arrays = Arrays::default();
        let location = flood_setup(&mut e, &arrays);
        let out = e.mem.alloc(0x20);
        assert!(!fn_006d4570(&mut e, Ptr::new(location), 1.0, Ptr::new(out)));

        // Every edge reached has a neighbour that was visited already.
        let mut e = engine();
        let arrays = Arrays::default();
        let location = flood_setup(&mut e, &arrays);
        e.register(TRIANGLE_NEIGHBOUR, |_, _| ret(4));
        e.register(NAVMESH_GET_MATCHING_EDGE, |e, a| {
            e.mem.set_u32(a[2], NAV);
            e.mem.set_u16(a[2] + 4, 4);
            ret(1)
        });
        e.call_log = Some(vec![]);
        assert!(!fn_006d4570(&mut e, Ptr::new(location), 2.0, Ptr::new(out)));
        let log = log_of(&mut e);
        let visited = calls_to(&log, TRIANGLE_ARRAY_CONSTRUCT)[0][0];
        assert_eq!(arrays.count(visited), 2);
        assert_eq!(calls_to(&log, NAV_HOLDER_RELEASE).len(), 1);

        // The location has no triangle.
        let mut e = engine();
        let arrays = Arrays::default();
        let location = flood_setup(&mut e, &arrays);
        e.register(LOCATION_GET_NAVMESH_AND_TRIANGLE, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert!(!fn_006d4570(&mut e, Ptr::new(location), 2.0, Ptr::new(out)));
        let log = log_of(&mut e);
        assert_eq!(calls_to(&log, NAV_HOLDER_RELEASE).len(), 1);
        assert_eq!(calls_to(&log, TRIANGLE_ARRAY_DESTRUCT).len(), 1);
        assert_eq!(calls_to(&log, EDGE_ARRAY_DESTRUCT).len(), 1);
        assert!(calls_to(&log, TRIANGLE_ARRAY_ADD).is_empty());
    }

    // -- 006d4d20, 006d4eb0 -------------------------------------------------

    #[test]
    fn teleport_door_path_is_built_and_its_ends_set() {
        let mut e = engine();
        let (from, to, path) = (e.mem.alloc(0x28), e.mem.alloc(0x28), e.mem.alloc(0x60));
        let build = Rc::new(RefCell::new(Vec::new()));
        let sink = build.clone();
        e.register_double(TELEPORT_SEARCH_BUILD_NODE_PATH, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            ret(1)
        });
        e.register_double(LOCATION_COPY_POSITION, move |e, a| {
            let position = if a[0] == from {
                [1.0, 2.0, 3.0]
            } else {
                [4.0, 5.0, 6.0]
            };
            set_point(e, a[1], position);
            ret(a[1])
        });
        e.call_log = Some(vec![]);
        assert!(pathing_build_teleport_door_path(
            &mut e,
            Ptr::new(from),
            Ptr::new(to),
            Ptr::new(path),
            7,
            2.5,
            9,
            11
        ));
        let log = log_of(&mut e);
        assert_eq!(timer_lines(&log), vec![0x582]);
        assert_timers_balanced(&log);
        assert_eq!(calls_to(&log, TELEPORT_PATH_CLEAR), vec![vec![path]]);
        assert_eq!(
            calls_to(&log, TELEPORT_PATH_RESERVE_STEPS),
            vec![vec![path, 10]]
        );
        assert_eq!(
            calls_to(&log, TELEPORT_PATH_RESERVE_DOORS),
            vec![vec![path + 0x10, 10]]
        );
        let build = build.borrow();
        assert_eq!(build[0][1..], [from, to, path, 7, 2.5f32.to_bits(), 9, 11]);
        assert_eq!(point(&e, path + 0x20), [1.0, 2.0, 3.0]);
        assert_eq!(point(&e, path + 0x2c), [4.0, 5.0, 6.0]);
        // The search is destroyed before the timer closes.
        assert_eq!(
            calls_to(&log, TELEPORT_SEARCH_DESTRUCT),
            vec![vec![build[0][0]]]
        );
        assert!(
            position_of(&log, TELEPORT_SEARCH_DESTRUCT).unwrap()
                < position_of(&log, TIMER_SCOPE_DESTROY).unwrap()
        );
    }

    #[test]
    fn teleport_door_path_failure_leaves_the_ends_alone() {
        let mut e = engine();
        let (from, to, path) = (e.mem.alloc(0x28), e.mem.alloc(0x28), e.mem.alloc(0x60));
        e.call_log = Some(vec![]);
        assert!(!pathing_build_teleport_door_path(
            &mut e,
            Ptr::new(from),
            Ptr::new(to),
            Ptr::new(path),
            0,
            0.0,
            0,
            0
        ));
        let log = log_of(&mut e);
        assert!(calls_to(&log, LOCATION_COPY_POSITION).is_empty());
        assert_eq!(point(&e, path + 0x20), [0.0; 3]);
        assert_timers_balanced(&log);
        assert_eq!(calls_to(&log, TELEPORT_SEARCH_DESTRUCT).len(), 1);
    }

    #[test]
    fn teleport_door_path_length_is_the_paths_or_the_largest_float() {
        let mut e = engine();
        let (from, to) = (e.mem.alloc(0x28), e.mem.alloc(0x28));
        e.register(LOCATION_COPY_POSITION, |e, a| {
            set_point(e, a[1], [1.0, 2.0, 3.0]);
            ret(a[1])
        });
        e.set_global(FLOAT_LARGEST, f32::MAX);
        let build = Rc::new(RefCell::new(Vec::new()));
        let sink = build.clone();
        e.register_double(TELEPORT_SEARCH_BUILD_NODE_PATH, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            ret(1)
        });
        e.register(TELEPORT_PATH_COMPUTE_LENGTH, |_, _| ret_float(12.5));
        e.call_log = Some(vec![]);
        let length = pathing_compute_teleport_door_path_length(
            &mut e,
            Ptr::new(from),
            Ptr::new(to),
            3.0,
            8,
            9,
        );
        let log = log_of(&mut e);
        assert_eq!(length, 12.5);
        // The search got a zero fourth argument and the distance and the
        // two words.
        assert_eq!(build.borrow()[0][1..2], [from]);
        assert_eq!(build.borrow()[0][4..], [0, 3.0f32.to_bits(), 8, 9]);
        // The local path is constructed first and destroyed last.
        let path = calls_to(&log, TELEPORT_PATH_CONSTRUCT)[0][0];
        assert_eq!(build.borrow()[0][3], path);
        assert_eq!(calls_to(&log, TELEPORT_PATH_DESTRUCT), vec![vec![path]]);
        assert_eq!(
            calls_to(&log, TELEPORT_PATH_COMPUTE_LENGTH),
            vec![vec![path]]
        );

        // No path: the largest float, and the length is not asked for.
        e.register(TELEPORT_SEARCH_BUILD_NODE_PATH, |_, _| ret(0));
        e.call_log = Some(vec![]);
        let length = pathing_compute_teleport_door_path_length(
            &mut e,
            Ptr::new(from),
            Ptr::new(to),
            3.0,
            8,
            9,
        );
        assert_eq!(length, f32::MAX);
        let log = log_of(&mut e);
        assert!(calls_to(&log, TELEPORT_PATH_COMPUTE_LENGTH).is_empty());
        assert_eq!(calls_to(&log, TELEPORT_PATH_DESTRUCT).len(), 1);
    }

    // -- 006d4f70 ----------------------------------------------------------

    /// The steps and the door entries the low-path doubles record.
    type Steps = Rc<RefCell<Vec<(u8, u32, u32)>>>;
    type Doors = Rc<RefCell<Vec<(u32, u32, [f32; 3])>>>;

    /// Doubles for `pathing_build_low_path` where both locations have info
    /// and the low-path search fills the info array with three infos and the
    /// reference array with a reference and a null. Returns the arrays and
    /// what the step and door recorders collect.
    fn low_path_setup(e: &mut Engine, from: u32, to: u32) -> (Steps, Doors) {
        let arrays = Arrays::default();
        arrays.constructor(e, INFO_ARRAY_CONSTRUCT, 4);
        arrays.constructor(e, REFERENCE_ARRAY_CONSTRUCT, 4);
        arrays.accessors(e, &[POINTER_ARRAY_ELEMENT]);
        e.register(LOCATION_RESOLVE_NAVMESH_INFO, |_, _| ret(1));
        e.register(LOCATION_NAVMESH_INFO_FIELD, |_, a| ret(a[0] + 0x10));
        e.register_double(LOCATION_COPY_POSITION, move |e, a| {
            let position = if a[0] == from {
                [1.0, 2.0, 3.0]
            } else {
                assert_eq!(a[0], to);
                [4.0, 5.0, 6.0]
            };
            set_point(e, a[1], position);
            ret(a[1])
        });
        e.register_double(LOW_PATH_SEARCH_RUN, move |e, a| {
            for info in [0x100u32, 0x200, 0x300] {
                arrays.push_word(e, a[6], info);
            }
            for reference in [0x700u32, 0] {
                arrays.push_word(e, a[7], reference);
            }
            ret(1)
        });
        e.register(INFO_VALUE_A, |_, a| ret(a[0] + 1));
        e.register(INFO_VALUE_B, |_, a| ret(a[0] + 2));
        e.register(REFERENCE_GET_POSITION_OWNER, |_, a| ret(a[0] + 0x10));
        let door_position = e.mem.alloc(12);
        set_point(e, door_position, [7.0, 8.0, 9.0]);
        e.register_double(NODE_LOCATION, move |_, a| {
            assert_eq!(a[0], 0x710);
            ret(door_position)
        });
        let steps = Steps::default();
        let sink = steps.clone();
        e.register_double(PATH_STEP_ADD, move |e, a| {
            sink.borrow_mut()
                .push((e.mem.u8(a[1]), e.mem.u32(a[1] + 4), e.mem.u32(a[1] + 8)));
            Ret::default()
        });
        let doors = Doors::default();
        let sink = doors.clone();
        e.register_double(PATH_DOOR_ADD, move |e, a| {
            sink.borrow_mut()
                .push((a[0], e.mem.u32(a[1]), point(e, a[1] + 4)));
            Ret::default()
        });
        (steps, doors)
    }

    #[test]
    fn low_path_follows_the_search_references() {
        let mut e = engine();
        let (from, to, path) = (e.mem.alloc(0x28), e.mem.alloc(0x28), e.mem.alloc(0x60));
        let (steps, doors) = low_path_setup(&mut e, from, to);
        e.call_log = Some(vec![]);
        assert!(pathing_build_low_path(
            &mut e,
            Ptr::new(from),
            Ptr::new(to),
            Ptr::new(path),
            5,
            6
        ));
        let log = log_of(&mut e);
        // The first step is the first info's; the reference gives a door
        // entry (the reference word and the position of its owner) and the
        // step of the next info; the null reference gives nothing.
        assert_eq!(*steps.borrow(), vec![(1, 0x101, 0x102), (1, 0x201, 0x202)]);
        assert_eq!(*doors.borrow(), vec![(path + 0x10, 0x700, [7.0, 8.0, 9.0])]);
        assert_eq!(point(&e, path + 0x20), [1.0, 2.0, 3.0]);
        assert_eq!(point(&e, path + 0x2c), [4.0, 5.0, 6.0]);
        // The search got both infos and positions; everything is torn down
        // and the teleport search is not used.
        let run = &calls_to(&log, LOW_PATH_SEARCH_RUN)[0];
        assert_eq!(
            (run[1], run[3], run[5], run[8]),
            (5, from + 0x10, to + 0x10, 6)
        );
        assert_eq!(calls_to(&log, REFERENCE_ARRAY_DESTRUCT).len(), 1);
        assert_eq!(calls_to(&log, INFO_ARRAY_DESTRUCT).len(), 1);
        assert_eq!(calls_to(&log, LOW_PATH_SEARCH_DESTRUCT).len(), 1);
        assert!(calls_to(&log, TELEPORT_SEARCH_CONSTRUCT).is_empty());
    }

    #[test]
    fn low_path_falls_back_to_the_teleport_door_search() {
        // The search finds nothing: its objects are destroyed and the
        // teleport search runs with 0, 0.0 and the two words.
        let mut e = engine();
        let (from, to, path) = (e.mem.alloc(0x28), e.mem.alloc(0x28), e.mem.alloc(0x60));
        low_path_setup(&mut e, from, to);
        e.register(LOW_PATH_SEARCH_RUN, |_, _| ret(0));
        let build = Rc::new(RefCell::new(Vec::new()));
        let sink = build.clone();
        e.register_double(TELEPORT_SEARCH_BUILD_NODE_PATH, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            ret(1)
        });
        e.call_log = Some(vec![]);
        assert!(pathing_build_low_path(
            &mut e,
            Ptr::new(from),
            Ptr::new(to),
            Ptr::new(path),
            5,
            6
        ));
        let log = log_of(&mut e);
        assert_eq!(build.borrow()[0][1..], [from, to, path, 0, 0, 5, 6]);
        assert_eq!(calls_to(&log, LOW_PATH_SEARCH_DESTRUCT).len(), 1);
        assert!(calls_to(&log, PATH_STEP_ADD).is_empty());

        // The first location without info skips the check of the second.
        let mut e = engine();
        e.register(LOCATION_RESOLVE_NAVMESH_INFO, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert!(!pathing_build_low_path(
            &mut e,
            Ptr::new(from),
            Ptr::new(to),
            Ptr::new(path),
            5,
            6
        ));
        let log = log_of(&mut e);
        assert_eq!(calls_to(&log, LOCATION_RESOLVE_NAVMESH_INFO).len(), 1);
        assert!(calls_to(&log, LOW_PATH_SEARCH_CONSTRUCT).is_empty());
        assert_eq!(calls_to(&log, TELEPORT_SEARCH_DESTRUCT).len(), 1);
    }

    // -- 006d5320 to 006d5420 ----------------------------------------------

    #[test]
    fn door_entry_constructor_constructs_its_point() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        assert_eq!(fn_006d5320(&mut e, Ptr::new(0x2000)), Ptr::new(0x2000));
        assert_eq!(
            log_of(&mut e),
            vec![(POINT3_DEFAULT_CONSTRUCTOR, vec![0x2004])]
        );
    }

    /// Checks a wrapper that builds the base solver, calls one of its
    /// methods and destroys it: returns the construct and method calls.
    fn solver_wrapper(
        method: u32,
        call: impl FnOnce(&mut Engine, Ptr<PathingRequest>, Ptr<PathingSolution>) -> u32,
    ) -> (Vec<u32>, Vec<u32>) {
        let mut e = engine();
        let (request, solution) = objects(&mut e);
        e.register(method, |_, _| ret(77));
        e.call_log = Some(vec![]);
        assert_eq!(call(&mut e, request, solution), 77);
        let log = log_of(&mut e);
        let construct = calls_to(&log, BASE_SOLVER_CONSTRUCT)[0].clone();
        assert_eq!(construct[1..], [request.addr(), solution.addr()]);
        let solver = construct[0];
        // The solver is destroyed: its two members.
        assert_eq!(
            calls_to(&log, NAVMESH_LIST_DESTRUCT),
            vec![vec![solver + 0x24]]
        );
        assert_eq!(
            calls_to(&log, BASE_SOLVER_MEMBER_DESTRUCT),
            vec![vec![solver + 0x14]]
        );
        (construct, calls_to(&log, method)[0].clone())
    }

    #[test]
    fn solver_wrapper_006d5340_passes_two_words_to_its_method() {
        let (construct, method) = solver_wrapper(BASE_SOLVER_METHOD_006C9170, |e, r, s| {
            fn_006d5340(e, r, s, 5, 6)
        });
        assert_eq!(method, vec![construct[0], 5, 6]);
    }

    #[test]
    fn solver_wrapper_006d53b0_calls_its_method_without_words() {
        let (construct, method) = solver_wrapper(BASE_SOLVER_METHOD_006C93D0, fn_006d53b0);
        assert_eq!(method, vec![construct[0]]);
    }

    #[test]
    fn solver_wrapper_006d5420_passes_one_word_to_its_method() {
        let (construct, method) = solver_wrapper(BASE_SOLVER_METHOD_006C93F0, |e, r, s| {
            fn_006d5420(e, r, s, 9)
        });
        assert_eq!(method, vec![construct[0], 9]);
    }

    // -- 006d5490 ----------------------------------------------------------

    /// Doubles for `fn_006d5490`: the navmesh list has two entries, both
    /// locations resolve to triangle 9 of `NAV`, the first is at (0, 0, 0)
    /// and the second at (100, 0, 0). Returns the two locations.
    fn straighten_setup(e: &mut Engine, arrays: &Arrays) -> (u32, u32) {
        vector_math(e);
        let (from, to) = (e.mem.alloc(0x28), e.mem.alloc(0x28));
        arrays.constructor(e, NAVMESH_LIST_CONSTRUCT, 4);
        arrays.constructor(e, INFO_ARRAY_CONSTRUCT, 4);
        arrays.appender(e, INFO_ARRAY_ADD);
        arrays.constructor(e, NODE_ARRAY_A_DEFAULT_CONSTRUCT, 0x14);
        arrays.constructor(e, NODE_ARRAY_B_CONSTRUCT, 0xc);
        arrays.remover(e, PATH_ARRAY_REMOVE);
        arrays.remover(e, NODE_ARRAY_REMOVE);
        arrays.accessors(
            e,
            &[
                POINTER_ARRAY_ELEMENT,
                PATH_ARRAY_ELEMENT,
                NODE_ARRAY_B_ELEMENT,
            ],
        );
        let entry = e.mem.alloc(4);
        let list_owner = arrays.clone();
        e.register_double(NAVMESH_LIST_FILL, move |e, a| {
            list_owner.push(e, a[0], entry);
            list_owner.push(e, a[0], entry);
            Ret::default()
        });
        e.register(NI_POINTER_GET_FROM_FIELD, |_, _| ret(NAV));
        e.register(NAVMESH_GET_INFO, |_, a| ret(a[0] + 0x1000));
        e.register(LOCATION_RESOLVE_CLOSEST, |_, _| ret(1));
        e.register(LOCATION_GET_NAVMESH_AND_TRIANGLE, |e, a| {
            e.mem.set_u16(a[2], 9);
            ret(1)
        });
        e.register(NAV_HOLDER_GET, |_, _| ret(NAV));
        e.register_double(LOCATION_COPY_POSITION, move |e, a| {
            let position = if a[0] == from {
                [0.0, 0.0, 0.0]
            } else {
                [100.0, 0.0, 0.0]
            };
            set_point(e, a[1], position);
            ret(a[1])
        });
        // A path node keeps its position at +8.
        e.register(NODE_TO_POSITION, |e, a| {
            let position = point(e, a[0] + 8);
            set_point(e, a[1], position);
            ret(a[1])
        });
        e.register(PATH_ARRAY_CAPACITY, |_, _| ret(0x40));
        e.register(POINT3_LENGTH, |e, a| {
            let p = point(e, a[0]);
            ret_float((p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt())
        });
        (from, to)
    }

    /// Makes the search fill the path with nodes at the given positions
    /// and the node array with entries (navmesh, triangle 3, edge = index).
    fn search_finds(e: &mut Engine, arrays: &Arrays, positions: Vec<[f32; 3]>, portals: u32) {
        let arrays = arrays.clone();
        e.register_double(NAVMESH_SEARCH_RUN, move |e, a| {
            let node = e.mem.alloc(0x14);
            for position in &positions {
                set_point(e, node + 8, *position);
                arrays.push(e, a[4], node);
            }
            let entry = e.mem.alloc(0xc);
            for edge in 0..portals {
                e.mem.set_u32(entry, NAV);
                e.mem.set_u16(entry + 4, 3);
                e.mem.set_u32(entry + 8, edge);
                arrays.push(e, a[5], entry);
            }
            ret(1)
        });
    }

    /// The positions of a path array's nodes.
    fn path_positions(e: &Engine, arrays: &Arrays, path: u32) -> Vec<[f32; 3]> {
        (0..arrays.count(path))
            .map(|i| point(e, arrays.element(path, i) + 8))
            .collect()
    }

    #[test]
    fn straightening_without_navmesh_positions_succeeds_only_if_neither_resolves() {
        let mut e = engine();
        let arrays = Arrays::default();
        let (from, to) = straighten_setup(&mut e, &arrays);
        e.register(LOCATION_RESOLVE_CLOSEST, |_, _| ret(0));
        let length = e.mem.alloc(4);
        e.mem.set_f32(length, 5.0);
        e.call_log = Some(vec![]);
        // Neither location resolves: the straight line is as good as any;
        // the distance is stored when asked for.
        assert!(fn_006d5490(
            &mut e,
            Ptr::new(from),
            Ptr::new(to),
            Ptr::new(0),
            Ptr::new(length),
            Ptr::new(0)
        ));
        let log = log_of(&mut e);
        assert_eq!(e.mem.f32(length), 100.0);
        assert_eq!(calls_to(&log, LOCATION_RESOLVE_CLOSEST).len(), 2);
        // The navmesh list was filled once and both arrays destroyed.
        assert_eq!(calls_to(&log, NAVMESH_LIST_FILL).len(), 1);
        assert_eq!(calls_to(&log, INFO_ARRAY_DESTRUCT).len(), 1);
        assert_eq!(calls_to(&log, NAVMESH_LIST_DESTRUCT).len(), 1);
        // The infos of both navmeshes were listed.
        let infos = calls_to(&log, INFO_ARRAY_CONSTRUCT)[0][0];
        assert_eq!(arrays.count(infos), 2);
        assert_eq!(e.mem.u32(arrays.element(infos, 1)), NAV + 0x1000);

        // Without a place for the distance it is not computed.
        e.mem.set_f32(length, 5.0);
        e.call_log = Some(vec![]);
        assert!(fn_006d62c0(
            &mut e,
            Ptr::new(from),
            Ptr::new(to),
            Ptr::new(0)
        ));
        assert!(calls_to(&log_of(&mut e), POINT3_LENGTH).is_empty());

        // Only the second resolves: failure.
        e.register_double(LOCATION_RESOLVE_CLOSEST, move |_, a| {
            ret((a[0] == to) as u32)
        });
        assert!(!fn_006d5490(
            &mut e,
            Ptr::new(from),
            Ptr::new(to),
            Ptr::new(0),
            Ptr::new(length),
            Ptr::new(0)
        ));
        assert_eq!(e.mem.f32(length), 100.0);
    }

    #[test]
    fn straightening_fails_with_the_distance_when_a_location_has_no_triangle() {
        // The first resolves but only the second has no navmesh: failure.
        let mut e = engine();
        let arrays = Arrays::default();
        let (from, to) = straighten_setup(&mut e, &arrays);
        e.register_double(LOCATION_RESOLVE_CLOSEST, move |_, a| {
            ret((a[0] == from) as u32)
        });
        let length = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        assert!(!fn_006d5490(
            &mut e,
            Ptr::new(from),
            Ptr::new(to),
            Ptr::new(0),
            Ptr::new(length),
            Ptr::new(0)
        ));
        assert_eq!(e.mem.f32(length), 100.0);
        assert!(calls_to(&log_of(&mut e), NAV_HOLDER_CONSTRUCT).is_empty());

        // The first has no triangle.
        let mut e = engine();
        let arrays = Arrays::default();
        let (from, to) = straighten_setup(&mut e, &arrays);
        e.register_double(LOCATION_GET_NAVMESH_AND_TRIANGLE, move |e, a| {
            e.mem.set_u16(a[2], 9);
            ret((a[0] != from) as u32)
        });
        e.call_log = Some(vec![]);
        assert!(!fn_006d5490(
            &mut e,
            Ptr::new(from),
            Ptr::new(to),
            Ptr::new(0),
            Ptr::new(length),
            Ptr::new(0)
        ));
        let log = log_of(&mut e);
        assert_eq!(e.mem.f32(length), 100.0);
        assert_eq!(calls_to(&log, NAV_HOLDER_RELEASE).len(), 1);
        assert_eq!(calls_to(&log, INFO_ARRAY_DESTRUCT).len(), 1);

        // The second has none.
        let mut e = engine();
        let arrays = Arrays::default();
        let (from, to) = straighten_setup(&mut e, &arrays);
        e.register_double(LOCATION_GET_NAVMESH_AND_TRIANGLE, move |e, a| {
            e.mem.set_u16(a[2], 9);
            ret((a[0] != to) as u32)
        });
        e.mem.set_f32(length, 0.0);
        e.call_log = Some(vec![]);
        assert!(!fn_006d5490(
            &mut e,
            Ptr::new(from),
            Ptr::new(to),
            Ptr::new(0),
            Ptr::new(length),
            Ptr::new(0)
        ));
        let log = log_of(&mut e);
        assert_eq!(e.mem.f32(length), 100.0);
        assert_eq!(calls_to(&log, NAV_HOLDER_RELEASE).len(), 2);

        // The search finds nothing: everything is destroyed again.
        let mut e = engine();
        let arrays = Arrays::default();
        let (from, to) = straighten_setup(&mut e, &arrays);
        e.mem.set_f32(length, 0.0);
        e.call_log = Some(vec![]);
        assert!(!fn_006d5490(
            &mut e,
            Ptr::new(from),
            Ptr::new(to),
            Ptr::new(0),
            Ptr::new(length),
            Ptr::new(0)
        ));
        let log = log_of(&mut e);
        assert_eq!(e.mem.f32(length), 100.0);
        assert_eq!(calls_to(&log, REQUEST_COPY_DESTRUCT).len(), 1);
        assert_eq!(calls_to(&log, NODE_ARRAY_B_DESTRUCT).len(), 1);
        assert_eq!(calls_to(&log, NODE_ARRAY_A_DESTRUCT).len(), 1);
        assert_eq!(calls_to(&log, NAVMESH_SEARCH_DESTRUCT).len(), 1);
        assert_eq!(calls_to(&log, NAV_HOLDER_RELEASE).len(), 2);
    }

    #[test]
    fn straightened_path_ends_at_the_two_positions() {
        let mut e = engine();
        let arrays = Arrays::default();
        let (from, to) = straighten_setup(&mut e, &arrays);
        search_finds(&mut e, &arrays, vec![[1.0, 1.0, 1.0], [2.0, 2.0, 2.0]], 1);
        let path = e.mem.alloc(0x10);
        arrays.make(&mut e, path, 0x14);
        let length = e.mem.alloc(4);
        let (request, _) = objects(&mut e);
        e.call_log = Some(vec![]);
        assert!(fn_006d5490(
            &mut e,
            Ptr::new(from),
            Ptr::new(to),
            request.cast(),
            Ptr::new(length),
            Ptr::new(path)
        ));
        let log = log_of(&mut e);
        // Two nodes: only the ends move and the length is their distance.
        assert_eq!(
            path_positions(&e, &arrays, path),
            vec![[0.0, 0.0, 0.0], [100.0, 0.0, 0.0]]
        );
        assert_eq!(e.mem.f32(length), 100.0);
        assert!(calls_to(&log, PATH_ARRAY_REMOVE).is_empty());
        // The request's avoid nodes go to the copy; the search got it, the
        // start and goal nodes (navmesh, triangle 9) and the arrays.
        let copy = calls_to(&log, REQUEST_CONSTRUCT)[0][0];
        assert_eq!(
            calls_to(&log, AVOID_NODE_ARRAY_ASSIGN),
            vec![vec![copy + 0x94, request.addr()]]
        );
        let run = &calls_to(&log, NAVMESH_SEARCH_RUN)[0];
        assert_eq!(run[1], copy);
        for node in [run[2], run[3]] {
            assert_eq!((e.mem.u32(node), e.mem.u16(node + 4)), (NAV, 9));
        }
        assert_eq!(run[4], path);
        // The path array is prepared and given room.
        assert_eq!(calls_to(&log, PATH_ARRAY_PREPARE), vec![vec![path, 0]]);
        assert!(calls_to(&log, PATH_ARRAY_RESERVE).is_empty());
        // The debug hook is not used.
        assert!(calls_to(&log, DEBUG_HOOK).is_empty());
    }

    #[test]
    fn straightened_path_grows_a_small_path_array() {
        let mut e = engine();
        let arrays = Arrays::default();
        let (from, to) = straighten_setup(&mut e, &arrays);
        search_finds(&mut e, &arrays, vec![[1.0; 3], [2.0; 3]], 1);
        e.register(PATH_ARRAY_CAPACITY, |_, _| ret(0x10));
        e.call_log = Some(vec![]);
        // No output path: a local one is used and no length is wanted.
        assert!(fn_006d5490(
            &mut e,
            Ptr::new(from),
            Ptr::new(to),
            Ptr::new(0),
            Ptr::new(0),
            Ptr::new(0)
        ));
        let log = log_of(&mut e);
        let local = calls_to(&log, NODE_ARRAY_A_DEFAULT_CONSTRUCT)[0][0];
        assert_eq!(calls_to(&log, PATH_ARRAY_RESERVE), vec![vec![local, 0x20]]);
        assert_eq!(calls_to(&log, NODE_ARRAY_A_DESTRUCT), vec![vec![local]]);
    }

    /// A four-node path (the ends are moved to (0, 0, 0) and (100, 0, 0)) with
    /// three portals; the portal of entry `i` has the points
    /// `PORTALS[i]`.
    const PORTALS: [([f32; 3], [f32; 3]); 3] = [
        ([10.0, 5.0, 0.0], [10.0, -8.0, 0.0]),
        ([30.0, 8.0, 0.0], [30.0, -2.0, 0.0]),
        ([60.0, 1.0, 0.0], [60.0, -1.0, 0.0]),
    ];

    fn smoothing_setup(e: &mut Engine, arrays: &Arrays) -> (u32, u32, u32) {
        let (from, to) = straighten_setup(e, arrays);
        search_finds(
            e,
            arrays,
            vec![
                [0.0, 0.0, 0.0],
                [25.0, 10.0, 0.0],
                [50.0, 10.0, 0.0],
                [100.0, 0.0, 0.0],
            ],
            3,
        );
        e.register(NAVMESH_EDGE_POINTS, |e, a| {
            let (left, right) = PORTALS[a[2] as usize];
            set_point(e, a[3], left);
            set_point(e, a[4], right);
            Ret::default()
        });
        e.register(LINE_REACHES, |_, _| ret(0));
        e.register(PORTAL_CLIP, |_, _| ret(0));
        let path = e.mem.alloc(0x10);
        arrays.make(e, path, 0x14);
        (from, to, path)
    }

    #[test]
    fn straightening_removes_the_nodes_a_line_can_skip() {
        let mut e = engine();
        let arrays = Arrays::default();
        let (from, to, path) = smoothing_setup(&mut e, &arrays);
        e.register(LINE_REACHES, |_, _| ret(1));
        e.call_log = Some(vec![]);
        assert!(fn_006d5490(
            &mut e,
            Ptr::new(from),
            Ptr::new(to),
            Ptr::new(0),
            Ptr::new(0),
            Ptr::new(path)
        ));
        let log = log_of(&mut e);
        // Node 0 reaches nodes 2 and 3: nodes 1 and 2 go, with portals 0
        // and 1.
        assert_eq!(calls_to(&log, PATH_ARRAY_REMOVE), vec![vec![path, 1, 2, 0]]);
        let nodes = calls_to(&log, NODE_ARRAY_B_CONSTRUCT)[0][0];
        assert_eq!(
            calls_to(&log, NODE_ARRAY_REMOVE),
            vec![vec![nodes, 0, 2, 0]]
        );
        assert_eq!(
            path_positions(&e, &arrays, path),
            vec![[0.0, 0.0, 0.0], [100.0, 0.0, 0.0]]
        );
        // The line test got node 0, the node it tests, and the portal
        // points: the first portal's both points while the portal of
        // entry j - 1 narrows them.
        let line = &calls_to(&log, LINE_REACHES);
        assert_eq!(line.len(), 2);
    }

    #[test]
    fn straightening_moves_the_inner_nodes_to_the_nearer_portal_end() {
        let mut e = engine();
        let arrays = Arrays::default();
        let (from, to, path) = smoothing_setup(&mut e, &arrays);
        let length = e.mem.alloc(4);
        e.set_global(DEBUG_VALUE_ODD, 20.0f32);
        e.set_global(DEBUG_VALUE_FINAL, 40.0f32);
        e.mem.set_u8(PATH_DEBUG_FLAG, 1);
        e.call_log = Some(vec![]);
        assert!(fn_006d5490(
            &mut e,
            Ptr::new(from),
            Ptr::new(to),
            Ptr::new(0),
            Ptr::new(length),
            Ptr::new(path)
        ));
        let log = log_of(&mut e);
        // No node can be skipped; node 1 goes to the nearer end of portal 0
        // (the left one, seen from node 0), node 2 to that of portal 1 seen
        // from the moved node 1.
        assert!(calls_to(&log, PATH_ARRAY_REMOVE).is_empty());
        assert_eq!(
            path_positions(&e, &arrays, path),
            vec![
                [0.0, 0.0, 0.0],
                [10.0, 5.0, 0.0],
                [30.0, 8.0, 0.0],
                [100.0, 0.0, 0.0]
            ]
        );
        let expected = 125.0f32.sqrt() + 409.0f32.sqrt() + 4964.0f32.sqrt();
        assert!((e.mem.f32(length) - expected).abs() < 1e-3);
        // The debug hook saw the path before smoothing (0.0, 0), the two
        // portals tested (node index 1 and 2, colors 0.0 and 20.0, 3), the
        // smoothed path (20.0, 2) and the end (40.0, 1); the flag is
        // cleared.
        let hooks = calls_to(&log, DEBUG_HOOK);
        assert_eq!(hooks.len(), 5);
        assert_eq!(hooks[0], vec![path, 0.0f32.to_bits(), 0]);
        assert_eq!(hooks[1][1..], [1, 0.0f32.to_bits(), 3]);
        assert_eq!(hooks[2][1..], [2, 20.0f32.to_bits(), 3]);
        assert_eq!(hooks[3], vec![path, 20.0f32.to_bits(), 2]);
        assert_eq!(hooks[4], vec![path, 40.0f32.to_bits(), 1]);
        assert_eq!(e.mem.u8(PATH_DEBUG_FLAG), 0);
    }

    #[test]
    fn straightening_narrows_the_portal_with_the_clipped_point() {
        let mut e = engine();
        let arrays = Arrays::default();
        let (from, to, path) = smoothing_setup(&mut e, &arrays);
        // Clip the left point of every portal to (1, 2, 3); keep the right.
        let clips = Rc::new(RefCell::new(Vec::new()));
        let sink = clips.clone();
        e.register_double(PORTAL_CLIP, move |e, a| {
            let mut clips = sink.borrow_mut();
            clips.push(point(e, a[3]));
            if clips.len() % 2 == 1 {
                set_point(e, a[4], [1.0, 2.0, 3.0]);
                ret(1)
            } else {
                ret(0)
            }
        });
        let lines = Rc::new(RefCell::new(Vec::new()));
        let sink = lines.clone();
        e.register_double(LINE_REACHES, move |e, a| {
            sink.borrow_mut().push((point(e, a[2]), point(e, a[3])));
            ret(0)
        });
        assert!(fn_006d5490(
            &mut e,
            Ptr::new(from),
            Ptr::new(to),
            Ptr::new(0),
            Ptr::new(0),
            Ptr::new(path)
        ));
        // Node 0 tests node 2 and node 1 tests node 3. The clip starts
        // from the portal of entry i (left, then right); the line test
        // gets the narrowed left point and the right end of the portal of
        // entry j - 1.
        assert_eq!(
            *clips.borrow(),
            vec![PORTALS[0].0, PORTALS[0].1, PORTALS[1].0, PORTALS[1].1]
        );
        assert_eq!(
            *lines.borrow(),
            vec![
                ([1.0, 2.0, 3.0], PORTALS[1].1),
                ([1.0, 2.0, 3.0], PORTALS[2].1)
            ]
        );
    }

    // -- 006d61b0, 006d61e0 -------------------------------------------------

    #[test]
    fn path_node_position_setter_stores_a_point_at_8() {
        let mut e = engine();
        let node = e.mem.alloc(0x14);
        let position = e.mem.alloc(12);
        set_point(&mut e, position, [1.0, 2.0, 3.0]);
        fn_006d61b0(&mut e, Ptr::new(node), Ptr::new(position));
        assert_eq!(point(&e, node + 8), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn avoid_node_array_is_assigned_at_0x94() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        fn_006d61e0(&mut e, Ptr::new(0x3000), 0x77);
        assert_eq!(
            log_of(&mut e),
            vec![(AVOID_NODE_ARRAY_ASSIGN, vec![0x3094, 0x77])]
        );
    }

    // -- 006d6200, 006d62c0 -------------------------------------------------

    #[test]
    fn low_path_distance_is_the_length_or_zero() {
        let mut e = engine();
        let (from, to) = (e.mem.alloc(0x28), e.mem.alloc(0x28));
        let out = e.mem.alloc(4);
        e.mem.set_f32(out, 99.0);
        // Neither location has info: the teleport door search decides.
        let build = Rc::new(RefCell::new(Vec::new()));
        let sink = build.clone();
        e.register_double(TELEPORT_SEARCH_BUILD_NODE_PATH, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            ret(1)
        });
        e.register(TELEPORT_PATH_COMPUTE_LENGTH, |_, _| ret_float(42.5));
        e.call_log = Some(vec![]);
        assert!(pathing_compute_low_path_distance(
            &mut e,
            Ptr::new(from),
            Ptr::new(to),
            Ptr::new(out)
        ));
        let log = log_of(&mut e);
        assert_eq!(e.mem.f32(out), 42.5);
        // The low path is built with the arguments 0 and 1, into a local
        // path that is destroyed afterwards.
        let path = calls_to(&log, TELEPORT_PATH_CONSTRUCT)[0][0];
        assert_eq!(build.borrow()[0][1..], [from, to, path, 0, 0, 0, 1]);
        assert_eq!(calls_to(&log, TELEPORT_PATH_DESTRUCT), vec![vec![path]]);

        e.register(TELEPORT_SEARCH_BUILD_NODE_PATH, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert!(!pathing_compute_low_path_distance(
            &mut e,
            Ptr::new(from),
            Ptr::new(to),
            Ptr::new(out)
        ));
        assert_eq!(e.mem.f32(out), 0.0);
        assert!(calls_to(&log_of(&mut e), TELEPORT_PATH_COMPUTE_LENGTH).is_empty());
    }

    // -- 006d62e0 ----------------------------------------------------------

    /// What the cover search sees of a vertex pair and of a location it
    /// builds.
    type EdgeInfos = Rc<RefCell<[(u16, u8, u8); 2]>>;

    /// The state `cover_setup` hands the test.
    struct CoverFixture {
        request: Ptr,
        results: u32,
        arrays: Arrays,
        edge_infos: EdgeInfos,
        /// The arguments of every `006e4500` call.
        covers: Rc<RefCell<Vec<Vec<u32>>>>,
        /// The arguments of every `006ba010` call and the points they
        /// pointed at.
        tests: Rc<RefCell<Vec<[[f32; 2]; 4]>>>,
        /// The cover list of the navmesh.
        triangles: u32,
    }

    /// Doubles for `fn_006d62e0`: a threat at the origin of the world and an
    /// actor at (10, 0, 0) (128 high, 64 crouching, 2 wide), a navmesh with
    /// the cover triangle 3 whose center is (20, 5, 0) (triangle 5: (13, 3,
    /// 0)); edge 0 is cover of class 8 with the first flag, edge 1 has class
    /// 1; the edge vertices are (20, 0, 0) and (20, 10, 0), the normal is
    /// up, and the cover test accepts. The request allows 4 results within
    /// 50 units, 30 up or down; it crouches, searches one side, sorts
    /// nearest first.
    fn cover_setup(e: &mut Engine) -> CoverFixture {
        let arrays = Arrays::default();
        vector_math(e);
        let request = Ptr::new(e.mem.alloc(0x120));
        let r = request.addr();
        e.mem.set_f32(r + 0x68, 2.0);
        e.mem.set_f32(r + 0x6c, 128.0);
        e.mem.set_f32(r + 0xbc, 50.0);
        e.mem.set_f32(r + 0xc0, 64.0);
        e.mem.set_f32(r + 0xc4, 30.0);
        e.mem.set_u32(r + 0xc8, 4);
        e.mem.set_u8(r + 0xcc, 1);
        set_point(e, r + 0xe0, [10.0, 0.0, 0.0]);
        e.register(COVER_MAX_RESULTS, |e, a| ret(e.mem.u32(a[0] + 0xc8)));
        e.register(COVER_ACTOR_LOCATION, |_, a| ret(a[0] + 0xe0));
        e.register(COVER_ACTOR_HEIGHT, |e, a| ret_float(e.mem.f32(a[0] + 0x6c)));
        e.register(COVER_MAX_DISTANCE, |e, a| ret_float(e.mem.f32(a[0] + 0xbc)));
        e.register(COVER_MAX_HEIGHT_CHANGE, |e, a| {
            ret_float(e.mem.f32(a[0] + 0xc4))
        });
        e.register(COVER_PROJECTION_VECTOR, |_, a| ret(a[0] + 0xd4));
        e.register(COVER_SORT_FROM_TARGET, |e, a| {
            ret(e.mem.u8(a[0] + 0xd0) as u32)
        });
        e.register(REQUEST_ACTOR_RADIUS, |e, a| {
            ret_float(e.mem.f32(a[0] + 0x68))
        });
        e.register(REQUEST_ORIGIN, |_, a| ret(a[0] + 0xc));
        e.register(LOCATION_COPY_POSITION, |e, a| {
            set_point(e, a[1], [10.0, 0.0, 0.0]);
            ret(a[1])
        });
        let equal = |e: &Engine, a: &[u32]| point(e, a[0]) == point(e, a[1]);
        e.register_double(POINT3_EQUALS, move |e, a| ret(equal(e, a) as u32));
        e.register_double(POINT3_NOT_EQUALS, move |e, a| ret(!equal(e, a) as u32));
        e.register(POINT3_DOT, |e, a| {
            let (x, y) = (point(e, a[0]), point(e, a[1]));
            ret_float(x[0] * y[0] + x[1] * y[1] + x[2] * y[2])
        });
        e.register(POINT3_CROSS, |e, a| {
            let (x, y) = (point(e, a[0]), point(e, a[2]));
            set_point(
                e,
                a[1],
                [
                    x[1] * y[2] - x[2] * y[1],
                    x[2] * y[0] - x[0] * y[2],
                    x[0] * y[1] - x[1] * y[0],
                ],
            );
            ret(a[1])
        });
        e.register(POINT2_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            ret(a[0])
        });
        e.register(HEIGHT_CLASS, |_, a| ret((float_arg(a[0]) / 10.0) as u32));
        arrays.constructor(e, NAVMESH_LIST_B_CONSTRUCT, 4);
        arrays.constructor(e, KEY_ARRAY_CONSTRUCT, 4);
        arrays.inserter(e, KEY_ARRAY_INSERT);
        arrays.inserter(e, COVER_RESULTS_INSERT);
        arrays.accessors(e, &[POINTER_ARRAY_ELEMENT, COVER_ARRAY_ELEMENT]);
        let entry = e.mem.alloc(4);
        let list_owner = arrays.clone();
        e.register_double(NEARBY_NAVMESHES_FILL_B, move |e, a| {
            list_owner.push(e, a[2], entry);
            Ret::default()
        });
        let triangles = e.mem.alloc(0x10);
        arrays.make(e, triangles, 4);
        arrays.push_word(e, triangles, 3);
        e.register_double(NAVMESH_COVER_ARRAY, move |_, _| ret(triangles));
        e.register(NI_POINTER_GET_FROM_FIELD, |_, _| ret(NAV));
        e.register(NAVMESH_GET_INFO, |_, a| ret(a[0] + 0x1000));
        e.register(NAVMESH_GET_TRIANGLE, |_, a| {
            ret(TRIANGLE_RECORDS + 0x10 * a[1])
        });
        e.register(TRIANGLE_IS_COVER, |_, _| ret(1));
        e.register(NAVMESH_GET_CENTER, |e, a| {
            let center = if a[2] == 3 {
                [20.0, 5.0, 0.0]
            } else {
                [13.0, 3.0, 0.0]
            };
            set_point(e, a[1], center);
            ret(a[1])
        });
        e.register(TRIANGLE_NORMAL, |e, a| {
            set_point(e, a[1], [0.0, 0.0, 1.0]);
            ret(a[1])
        });
        e.set_global(EDGE_MIDPOINT_FRACTION, 0.5f32);
        let edge_infos: EdgeInfos = Rc::new(RefCell::new([(8, 1, 0), (1, 0, 0)]));
        let infos = edge_infos.clone();
        e.register_double(TRIANGLE_EDGE_INFO, move |e, a| {
            let (class, first, second) = infos.borrow()[a[1] as usize];
            e.mem.set_u16(a[2], class);
            e.mem.set_u8(a[3], first);
            e.mem.set_u8(a[4], second);
            Ret::default()
        });
        let vertices = [e.mem.alloc(12), e.mem.alloc(12)];
        set_point(e, vertices[0], [20.0, 0.0, 0.0]);
        set_point(e, vertices[1], [20.0, 10.0, 0.0]);
        e.register_double(NAVMESH_EDGE_VERTICES, move |e, a| {
            e.mem.set_u32(a[1], vertices[0]);
            e.mem.set_u32(a[1] + 4, vertices[1]);
            Ret::default()
        });
        let tests: Rc<RefCell<Vec<[[f32; 2]; 4]>>> = Rc::default();
        let sink = tests.clone();
        e.register_double(COVER_EDGE_TEST, move |e, a| {
            let planar = |i: usize| {
                let p = point(e, a[i]);
                [p[0], p[1]]
            };
            sink.borrow_mut()
                .push([planar(0), planar(1), planar(2), planar(3)]);
            ret(1)
        });
        // A real constructor stores the position and the triangle.
        e.register(LOCATION_CONSTRUCT_FROM_MESH_POINT, |e, a| {
            let position = point(e, a[1]);
            set_point(e, a[0] + 4, position);
            e.mem.set_u32(a[0] + 0x10, a[2]);
            e.mem.set_u16(a[0] + 0x24, a[3] as u16);
            ret(a[0])
        });
        let covers: Rc<RefCell<Vec<Vec<u32>>>> = Rc::default();
        let sink = covers.clone();
        e.register_double(COVER_LOCATION_SET_COVER, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let results = e.mem.alloc(0x10);
        arrays.make(e, results, 0x54);
        CoverFixture {
            request,
            results,
            arrays,
            edge_infos,
            covers,
            tests,
            triangles,
        }
    }

    #[test]
    fn cover_search_builds_the_cover_location_from_the_edge() {
        let mut e = engine();
        let f = cover_setup(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(fn_006d62e0(&mut e, f.request, Ptr::new(f.results)), 1);
        let log = log_of(&mut e);
        // The results were given room for the 4 results asked for.
        assert_eq!(
            calls_to(&log, COVER_RESULTS_RESERVE),
            vec![vec![f.results, 4]]
        );
        // The cover position: the edge's midpoint (20, 5, 0) moved off the
        // edge by the radius 2, against the edge direction crossed with
        // the normal (1, 0, 0); the test got it with the planar threat
        // (the world's origin) and the planar edge ends.
        assert_eq!(
            *f.tests.borrow(),
            vec![[[18.0, 5.0], [0.0, 0.0], [20.0, 0.0], [20.0, 10.0]]]
        );
        let build = &calls_to(&log, LOCATION_CONSTRUCT_FROM_MESH_POINT)[0];
        assert_eq!(point(&e, build[1]), [18.0, 5.0, 0.0]);
        assert_eq!(build[2..], [NAV + 0x1000, 3]);
        assert_eq!(f.arrays.count(f.results), 1);
        let cover = f.arrays.element(f.results, 0);
        assert_eq!(e.mem.u32(cover), PATHING_COVER_LOCATION_VTABLE);
        assert_eq!(e.mem.u16(cover + 0x24), 3);
        assert_eq!(point(&e, cover + 0x28), [20.0, 10.0, 0.0]);
        assert_eq!(point(&e, cover + 0x34), [20.0, 0.0, 0.0]);
        assert_eq!(point(&e, cover + 0x40), [1.0, 0.0, 0.0]);
        // Cover class 8, the classes of the standing (128) and crouching
        // (64) actor, the two flags and the edge.
        let covers = f.covers.borrow();
        assert_eq!(covers.len(), 1);
        assert_eq!(covers[0][1..], [8, 12, 6, 1, 0, 0]);
        // The sort key is the squared distance between the actor and the
        // triangle center.
        let keys = calls_to(&log, KEY_ARRAY_CONSTRUCT)[0][0];
        assert_eq!(f.arrays.count(keys), 1);
        assert_eq!(e.mem.f32(f.arrays.element(keys, 0)), 125.0);
        // The location built on the stack is destroyed after the insertion;
        // the holder, the navmesh list and the keys are released.
        assert_eq!(calls_to(&log, LOCATION_DESTRUCT).len(), 1);
        assert_eq!(calls_to(&log, NAV_HOLDER_RELEASE).len(), 1);
        assert_eq!(calls_to(&log, NAVMESH_LIST_B_DESTRUCT).len(), 1);
        assert_eq!(calls_to(&log, KEY_ARRAY_DESTRUCT), vec![vec![keys]]);
        // The search asked for the navmeshes near the origin within the
        // maximum distance.
        assert_eq!(
            calls_to(&log, NEARBY_NAVMESHES_FILL_B)[0][..2],
            [f.request.addr() + 0xc, 50.0f32.to_bits()]
        );
    }

    /// Adds a second cover triangle (5) to the list.
    fn two_triangles(e: &mut Engine, f: &CoverFixture) {
        f.arrays.push_word(e, f.triangles, 5);
    }

    /// The triangles of the covers in the results, best first.
    fn result_triangles(e: &Engine, f: &CoverFixture) -> Vec<u16> {
        (0..f.arrays.count(f.results))
            .map(|i| e.mem.u16(f.arrays.element(f.results, i) + 0x24))
            .collect()
    }

    #[test]
    fn cover_search_sorts_by_distance_and_stops_at_the_maximum() {
        let mut e = engine();
        let f = cover_setup(&mut e);
        two_triangles(&mut e, &f);
        assert_eq!(fn_006d62e0(&mut e, f.request, Ptr::new(f.results)), 2);
        // Nearest first: triangle 5 (key 18) before triangle 3 (key 125).
        assert_eq!(result_triangles(&e, &f), vec![5, 3]);

        // Farthest first.
        let mut e = engine();
        let f = cover_setup(&mut e);
        two_triangles(&mut e, &f);
        e.mem.set_u8(f.request.addr() + 0xcf, 1);
        assert_eq!(fn_006d62e0(&mut e, f.request, Ptr::new(f.results)), 2);
        assert_eq!(result_triangles(&e, &f), vec![3, 5]);

        // One result at most: the search ends with the first.
        let mut e = engine();
        let f = cover_setup(&mut e);
        two_triangles(&mut e, &f);
        e.mem.set_u32(f.request.addr() + 0xc8, 1);
        e.call_log = Some(vec![]);
        assert_eq!(fn_006d62e0(&mut e, f.request, Ptr::new(f.results)), 1);
        let log = log_of(&mut e);
        assert_eq!(result_triangles(&e, &f), vec![3]);
        assert_eq!(calls_to(&log, NAVMESH_GET_CENTER).len(), 1);
        assert_eq!(calls_to(&log, NAV_HOLDER_RELEASE).len(), 1);
        assert_eq!(calls_to(&log, KEY_ARRAY_DESTRUCT).len(), 1);
    }

    #[test]
    fn cover_search_skips_far_high_and_wrong_sided_triangles() {
        // Too far from the origin (125 is above 5 squared).
        let mut e = engine();
        let f = cover_setup(&mut e);
        e.mem.set_f32(f.request.addr() + 0xbc, 5.0);
        assert_eq!(fn_006d62e0(&mut e, f.request, Ptr::new(f.results)), 0);

        // The actor is 100 above the center: more than the allowed 30.
        let mut e = engine();
        let f = cover_setup(&mut e);
        e.mem.set_f32(f.request.addr() + 0xe0 + 8, 100.0);
        assert_eq!(fn_006d62e0(&mut e, f.request, Ptr::new(f.results)), 0);

        // Behind the target: the side is negated.
        let mut e = engine();
        let f = cover_setup(&mut e);
        e.mem.set_u8(f.request.addr() + 0xce, 1);
        assert_eq!(fn_006d62e0(&mut e, f.request, Ptr::new(f.results)), 0);
        // ... unless both sides count.
        e.mem.set_u8(f.request.addr() + 0xcd, 1);
        assert_eq!(fn_006d62e0(&mut e, f.request, Ptr::new(f.results)), 1);

        // Not a cover triangle.
        let mut e = engine();
        let f = cover_setup(&mut e);
        e.register(TRIANGLE_IS_COVER, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert_eq!(fn_006d62e0(&mut e, f.request, Ptr::new(f.results)), 0);
        assert!(calls_to(&log_of(&mut e), NAVMESH_GET_CENTER).is_empty());
    }

    #[test]
    fn cover_search_accepts_edges_by_class_and_flags() {
        let run = |infos: [(u16, u8, u8); 2], hide: u8, crouch: u8| {
            let mut e = engine();
            let f = cover_setup(&mut e);
            *f.edge_infos.borrow_mut() = infos;
            e.mem.set_u8(f.request.addr() + 0xd1, hide);
            e.mem.set_u8(f.request.addr() + 0xcc, crouch);
            fn_006d62e0(&mut e, f.request, Ptr::new(f.results))
        };
        // Classes below 4: only class 2, and not when hiding.
        assert_eq!(run([(3, 1, 1), (1, 1, 1)], 0, 1), 0);
        assert_eq!(run([(2, 0, 0), (1, 0, 0)], 0, 1), 1);
        assert_eq!(run([(2, 0, 0), (1, 0, 0)], 1, 1), 0);
        // Class 8 reaches the crouching class 6 but not the standing 12.
        assert_eq!(run([(8, 1, 0), (1, 0, 0)], 0, 1), 1);
        assert_eq!(run([(8, 1, 0), (1, 0, 0)], 0, 0), 0);
        assert_eq!(run([(12, 1, 0), (1, 0, 0)], 0, 0), 1);
        // Above the crouching class a flag or hiding is needed.
        assert_eq!(run([(8, 0, 0), (1, 0, 0)], 0, 1), 0);
        assert_eq!(run([(8, 0, 1), (1, 0, 0)], 0, 1), 1);
        assert_eq!(run([(8, 0, 0), (1, 0, 0)], 1, 1), 1);
        // The class of the second edge counts too, and edge 1 is the one
        // reported.
        let mut e = engine();
        let f = cover_setup(&mut e);
        *f.edge_infos.borrow_mut() = [(1, 0, 0), (8, 1, 0)];
        assert_eq!(fn_006d62e0(&mut e, f.request, Ptr::new(f.results)), 1);
        assert_eq!(f.covers.borrow()[0][6], 1);
        // A cover edge the threat test refuses is not used.
        let mut e = engine();
        let f = cover_setup(&mut e);
        e.register(COVER_EDGE_TEST, |_, _| ret(0));
        assert_eq!(fn_006d62e0(&mut e, f.request, Ptr::new(f.results)), 0);
    }

    #[test]
    fn cover_search_uses_the_origin_without_an_actor_location() {
        let mut e = engine();
        let f = cover_setup(&mut e);
        set_point(&mut e, f.request.addr() + 0xe0, [0.0, 0.0, 0.0]);
        e.call_log = Some(vec![]);
        assert_eq!(fn_006d62e0(&mut e, f.request, Ptr::new(f.results)), 1);
        // The origin's position (10, 0, 0) takes its place: the same key.
        let log = log_of(&mut e);
        let keys = calls_to(&log, KEY_ARRAY_CONSTRUCT)[0][0];
        assert_eq!(e.mem.f32(f.arrays.element(keys, 0)), 125.0);
        // A distance beyond the maximum still excludes it.
        let mut e = engine();
        let f = cover_setup(&mut e);
        set_point(&mut e, f.request.addr() + 0xe0, [0.0, 0.0, 0.0]);
        e.mem.set_f32(f.request.addr() + 0xbc, 5.0);
        assert_eq!(fn_006d62e0(&mut e, f.request, Ptr::new(f.results)), 0);
    }

    #[test]
    fn cover_search_corrects_the_key_by_the_projection_vector() {
        // Measured from the actor (the default): -10 along the projection
        // vector (1, 0, 0), key 125 + 2 * 100 / 125.
        let mut e = engine();
        let f = cover_setup(&mut e);
        set_point(&mut e, f.request.addr() + 0xd4, [1.0, 0.0, 0.0]);
        e.call_log = Some(vec![]);
        assert_eq!(fn_006d62e0(&mut e, f.request, Ptr::new(f.results)), 1);
        let keys = calls_to(&log_of(&mut e), KEY_ARRAY_CONSTRUCT)[0][0];
        assert_eq!(
            e.mem.f32(f.arrays.element(keys, 0)),
            (125.0f64 + 2.0 * 100.0 / 125.0) as f32
        );

        // Measured from the threat vector: (0, 0, 0) - (20, 5, 0), key 425
        // and the dot product -20.
        let mut e = engine();
        let f = cover_setup(&mut e);
        set_point(&mut e, f.request.addr() + 0xd4, [1.0, 0.0, 0.0]);
        e.mem.set_u8(f.request.addr() + 0xd0, 1);
        e.call_log = Some(vec![]);
        assert_eq!(fn_006d62e0(&mut e, f.request, Ptr::new(f.results)), 1);
        let keys = calls_to(&log_of(&mut e), KEY_ARRAY_CONSTRUCT)[0][0];
        assert_eq!(
            e.mem.f32(f.arrays.element(keys, 0)),
            (425.0f64 + 2.0 * 400.0 / 425.0) as f32
        );

        // Without a projection vector, measured from the threat: 425.
        let mut e = engine();
        let f = cover_setup(&mut e);
        e.mem.set_u8(f.request.addr() + 0xd0, 1);
        e.call_log = Some(vec![]);
        assert_eq!(fn_006d62e0(&mut e, f.request, Ptr::new(f.results)), 1);
        let keys = calls_to(&log_of(&mut e), KEY_ARRAY_CONSTRUCT)[0][0];
        assert_eq!(e.mem.f32(f.arrays.element(keys, 0)), 425.0);
    }

    // -- 006d6d60 to 006d6e30 -----------------------------------------------

    /// A cover request with the fields the accessors read.
    fn accessor_request(e: &mut Engine) -> Ptr {
        let request = Ptr::new(e.mem.alloc(0x120));
        let r = request.addr();
        set_point(e, r + 0xb0, [1.0, 2.0, 3.0]);
        e.mem.set_f32(r + 0xc0, 64.5);
        for (offset, value) in [(0xcc, 1u8), (0xcd, 2), (0xce, 3), (0xcf, 4), (0xd1, 5)] {
            e.mem.set_u8(r + offset, value);
        }
        request
    }

    #[test]
    fn cover_request_threat_is_copied_out() {
        let mut e = engine();
        let request = accessor_request(&mut e);
        let out = e.mem.alloc(12);
        assert_eq!(fn_006d6d60(&mut e, request, Ptr::new(out)), Ptr::new(out));
        assert_eq!(point(&e, out), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn cover_request_crouch_height_is_read() {
        let mut e = engine();
        let request = accessor_request(&mut e);
        assert_eq!(fn_006d6d90(&mut e, request), 64.5);
    }

    #[test]
    fn cover_request_can_crouch_is_read() {
        let mut e = engine();
        let request = accessor_request(&mut e);
        assert_eq!(fn_006d6db0(&mut e, request), 1);
    }

    #[test]
    fn cover_request_find_both_sides_is_read() {
        let mut e = engine();
        let request = accessor_request(&mut e);
        assert_eq!(fn_006d6dd0(&mut e, request), 2);
    }

    #[test]
    fn cover_request_find_behind_target_is_read() {
        let mut e = engine();
        let request = accessor_request(&mut e);
        assert_eq!(fn_006d6df0(&mut e, request), 3);
    }

    #[test]
    fn cover_request_sort_farthest_first_is_read() {
        let mut e = engine();
        let request = accessor_request(&mut e);
        assert_eq!(fn_006d6e10(&mut e, request), 4);
    }

    #[test]
    fn cover_request_hide_cover_is_read() {
        let mut e = engine();
        let request = accessor_request(&mut e);
        assert_eq!(fn_006d6e30(&mut e, request), 5);
    }

    // -- 006d6e50, 006d6ee0 -------------------------------------------------

    #[test]
    fn cover_location_constructor_sets_the_vtable_and_the_points() {
        let mut e = engine();
        let this = Ptr::<PathingCoverLocation>::new(e.mem.alloc(0x54));
        e.call_log = Some(vec![]);
        assert_eq!(fn_006d6e50(&mut e, this, 0x111, 0x222, 3), this);
        assert_eq!(
            log_of(&mut e),
            vec![
                (
                    LOCATION_CONSTRUCT_FROM_MESH_POINT,
                    vec![this.addr(), 0x111, 0x222, 3]
                ),
                (POINT3_DEFAULT_CONSTRUCTOR, vec![this.addr() + 0x28]),
                (POINT3_DEFAULT_CONSTRUCTOR, vec![this.addr() + 0x34]),
                (POINT3_DEFAULT_CONSTRUCTOR, vec![this.addr() + 0x40]),
            ]
        );
        assert_eq!(e.mem.u32(this.addr()), PATHING_COVER_LOCATION_VTABLE);
    }

    #[test]
    fn cover_location_points_are_stored_in_order() {
        let mut e = engine();
        let this = Ptr::<PathingCoverLocation>::new(e.mem.alloc(0x54));
        let (first, second, third) = (e.mem.alloc(12), e.mem.alloc(12), e.mem.alloc(12));
        set_point(&mut e, first, [1.0, 2.0, 3.0]);
        set_point(&mut e, second, [4.0, 5.0, 6.0]);
        set_point(&mut e, third, [7.0, 8.0, 9.0]);
        fn_006d6ee0(
            &mut e,
            this,
            Ptr::new(first),
            Ptr::new(second),
            Ptr::new(third),
        );
        assert_eq!(point(&e, this.addr() + 0x28), [1.0, 2.0, 3.0]);
        assert_eq!(point(&e, this.addr() + 0x34), [4.0, 5.0, 6.0]);
        assert_eq!(point(&e, this.addr() + 0x40), [7.0, 8.0, 9.0]);
    }

    // -- 006d6f40, 006d6f60 -------------------------------------------------

    /// A `TES` whose map answers the two lookups.
    fn potential_info_setup(e: &mut Engine) {
        e.set_global(TES_GLOBAL, 0x00a4_0000u32);
        e.register_double(TES_GET_NAVMESH_INFO_MAP, |_, a| {
            assert_eq!(a, [0x00a4_0000]);
            ret(0x00a5_0000)
        });
        e.register(NAVMESH_INFO_MAP_FIND_FOR_CELL, |_, a| {
            assert_eq!(a[0], 0x00a5_0000);
            ret(a[1] + 1)
        });
        e.register(NAVMESH_INFO_MAP_FIND_BY_PAIR, |_, a| {
            assert_eq!(a[0], 0x00a5_0000);
            ret(a[1] * 100 + a[2])
        });
    }

    #[test]
    fn potential_navmesh_info_for_a_cell_asks_the_map_of_the_global_tes() {
        let mut e = engine();
        potential_info_setup(&mut e);
        assert_eq!(
            pathing_get_potential_nav_mesh_info_for_location(&mut e, 41),
            42
        );
    }

    #[test]
    fn potential_navmesh_info_for_two_words_asks_the_map_of_the_global_tes() {
        let mut e = engine();
        potential_info_setup(&mut e);
        assert_eq!(
            pathing_get_potential_nav_mesh_info_for_location_ov2(&mut e, 3, 4),
            304
        );
    }

    // -- 006d6f80 ----------------------------------------------------------

    #[test]
    fn closest_point_on_navmesh_keeps_a_point_that_has_navmesh_info() {
        let mut e = engine();
        let (given, out) = (e.mem.alloc(12), e.mem.alloc(12));
        set_point(&mut e, given, [1.0, 2.0, 3.0]);
        e.register(LOCATION_RESOLVE_NAVMESH_INFO, |_, _| ret(1));
        e.register(LOCATION_HAS_NAVMESH_INFO, |_, _| ret(1));
        e.call_log = Some(vec![]);
        assert!(pathing_find_closest_point_on_navmesh(
            &mut e,
            0x10,
            0x20,
            Ptr::new(given),
            Ptr::new(out)
        ));
        let log = log_of(&mut e);
        assert_eq!(point(&e, out), [1.0, 2.0, 3.0]);
        // The location is built from the point, the cell and the
        // worldspace, inside the timer of line 0x906, and destroyed.
        let built = &calls_to(&log, LOCATION_CONSTRUCT_FROM_PARTS)[0];
        assert_eq!(built[1..], [given, 0x20, 0x10]);
        assert_eq!(timer_lines(&log), vec![0x906]);
        assert_timers_balanced(&log);
        assert_eq!(calls_to(&log, LOCATION_DESTRUCT), vec![vec![built[0]]]);
        assert!(calls_to(&log, LOCATION_RESOLVE_CLOSEST).is_empty());
    }

    #[test]
    fn closest_point_on_navmesh_is_the_center_of_the_closest_triangle() {
        let mut e = engine();
        let (given, out) = (e.mem.alloc(12), e.mem.alloc(12));
        e.register(LOCATION_RESOLVE_CLOSEST, |_, _| ret(1));
        e.register(LOCATION_GET_NAVMESH_AND_TRIANGLE, |e, a| {
            e.mem.set_u16(a[2], 7);
            ret(1)
        });
        e.register(NI_POINTER_GET_FROM_FIELD, |_, _| ret(NAV));
        e.register(NAVMESH_GET_CENTER, |e, a| {
            assert_eq!((a[0], a[2]), (NAV, 7));
            set_point(e, a[1], [3.0, 4.0, 5.0]);
            ret(a[1])
        });
        e.call_log = Some(vec![]);
        assert!(pathing_find_closest_point_on_navmesh(
            &mut e,
            0x10,
            0x20,
            Ptr::new(given),
            Ptr::new(out)
        ));
        let log = log_of(&mut e);
        assert_eq!(point(&e, out), [3.0, 4.0, 5.0]);
        assert_eq!(calls_to(&log, NAV_HOLDER_RELEASE).len(), 1);
        assert_eq!(calls_to(&log, LOCATION_DESTRUCT).len(), 1);
        assert_timers_balanced(&log);
    }

    #[test]
    fn closest_point_on_navmesh_fails_without_a_navmesh_or_triangle() {
        let mut e = engine();
        let (given, out) = (e.mem.alloc(12), e.mem.alloc(12));
        e.call_log = Some(vec![]);
        // Nothing resolves.
        assert!(!pathing_find_closest_point_on_navmesh(
            &mut e,
            0x10,
            0x20,
            Ptr::new(given),
            Ptr::new(out)
        ));
        let log = log_of(&mut e);
        assert!(calls_to(&log, NAV_HOLDER_CONSTRUCT).is_empty());
        assert_eq!(calls_to(&log, LOCATION_DESTRUCT).len(), 1);
        assert_timers_balanced(&log);

        // A navmesh but no triangle: the holder is released.
        e.register(LOCATION_RESOLVE_CLOSEST, |_, _| ret(1));
        e.call_log = Some(vec![]);
        assert!(!pathing_find_closest_point_on_navmesh(
            &mut e,
            0x10,
            0x20,
            Ptr::new(given),
            Ptr::new(out)
        ));
        let log = log_of(&mut e);
        assert_eq!(calls_to(&log, NAV_HOLDER_RELEASE).len(), 1);
        assert_eq!(point(&e, out), [0.0; 3]);
    }

    // -- 006d7110 ----------------------------------------------------------

    #[test]
    fn point_on_navmesh_without_a_navmesh_is_a_random_offset() {
        let mut e = engine();
        vector_math(&mut e);
        let (given, out) = (e.mem.alloc(12), e.mem.alloc(12));
        set_point(&mut e, given, [100.0, 200.0, 300.0]);
        e.register(LOCATION_RESOLVE_NAVMESH_INFO, |_, _| ret(1));
        let randoms = Rc::new(RefCell::new(Vec::new()));
        let sink = randoms.clone();
        e.register_double(RANDOM_FLOAT, move |_, a| {
            let mut randoms = sink.borrow_mut();
            randoms.push(a.to_vec());
            ret_float(if randoms.len() == 1 { 6.0 } else { -3.0 })
        });
        e.register(SQUARE_ROOT_WRAPPER, |_, a| {
            ret_float(float_arg(a[0]).sqrt())
        });
        e.register(POINT3_CONSTRUCT, |e, a| {
            for i in 1..4 {
                e.mem.set_u32(a[0] + 4 * (i - 1), a[i as usize]);
            }
            ret(a[0])
        });
        e.call_log = Some(vec![]);
        // The location has info but no navmesh (the GetNavMesh double
        // returns false).
        assert!(pathing_find_point_on_nav_mesh(
            &mut e,
            1,
            2,
            Ptr::new(given),
            10.0,
            Ptr::new(out)
        ));
        let log = log_of(&mut e);
        // x in [-10, 10] was 6, y in [-8, 8] (the root of 100 - 36) was -3.
        assert_eq!(
            *randoms.borrow(),
            vec![
                vec![(-10.0f32).to_bits(), 10.0f32.to_bits()],
                vec![(-8.0f32).to_bits(), 8.0f32.to_bits()]
            ]
        );
        assert_eq!(point(&e, out), [106.0, 197.0, 300.0]);
        let built = &calls_to(&log, LOCATION_CONSTRUCT_FROM_PARTS)[0];
        assert_eq!(built[1..], [given, 2, 1]);
        assert_eq!(calls_to(&log, NAV_HOLDER_RELEASE).len(), 1);
        assert_eq!(calls_to(&log, LOCATION_DESTRUCT).len(), 1);

        // Without navmesh info the same, even if a navmesh would be found.
        e.register(LOCATION_RESOLVE_NAVMESH_INFO, |_, _| ret(0));
        e.register(LOCATION_GET_NAVMESH, |_, _| ret(1));
        assert!(pathing_find_point_on_nav_mesh(
            &mut e,
            1,
            2,
            Ptr::new(given),
            10.0,
            Ptr::new(out)
        ));
        assert_eq!(randoms.borrow().len(), 4);
    }

    /// Doubles for `pathing_find_point_on_nav_mesh` on a navmesh: the first
    /// probe stores (7, 8, 9) and returns `first`, the second returns
    /// `second`; the closest triangle search finds triangle 6 (if
    /// `triangle`), whose center is (3, 4, 5).
    fn navmesh_probe_setup(e: &mut Engine, first: u32, second: u32, triangle: u32) {
        e.register(LOCATION_RESOLVE_NAVMESH_INFO, |_, _| ret(1));
        e.register(LOCATION_GET_NAVMESH, |_, _| ret(1));
        e.register(NI_POINTER_GET_FROM_FIELD, |_, _| ret(NAV));
        e.register_double(NAVMESH_PROBE_A, move |e, a| {
            set_point(e, a[3], [7.0, 8.0, 9.0]);
            ret(first)
        });
        e.register_double(NAVMESH_PROBE_B, move |_, _| ret(second));
        e.register_double(NAVMESH_FIND_TRIANGLE, move |e, a| {
            e.mem.set_u16(a[2], 6);
            ret(triangle)
        });
        e.register(NAVMESH_GET_CENTER, |e, a| {
            assert_eq!((a[0], a[2]), (NAV, 6));
            set_point(e, a[1], [3.0, 4.0, 5.0]);
            ret(a[1])
        });
    }

    #[test]
    fn point_on_navmesh_uses_the_navmesh_probes_or_the_closest_triangle() {
        // Both probes accept: their point.
        let mut e = engine();
        let (given, out) = (e.mem.alloc(12), e.mem.alloc(12));
        navmesh_probe_setup(&mut e, 1, 1, 1);
        e.call_log = Some(vec![]);
        assert!(pathing_find_point_on_nav_mesh(
            &mut e,
            1,
            2,
            Ptr::new(given),
            10.0,
            Ptr::new(out)
        ));
        let log = log_of(&mut e);
        assert_eq!(point(&e, out), [7.0, 8.0, 9.0]);
        assert_eq!(
            calls_to(&log, NAVMESH_PROBE_A),
            vec![vec![NAV, given, 10.0f32.to_bits(), out]]
        );
        assert_eq!(calls_to(&log, NAVMESH_PROBE_B), vec![vec![NAV, out, 0]]);
        assert!(calls_to(&log, NAVMESH_FIND_TRIANGLE).is_empty());
        assert_eq!(calls_to(&log, NAV_HOLDER_RELEASE).len(), 1);

        // The first probe refuses: the second is not asked and the center
        // of the closest triangle is used.
        let mut e = engine();
        navmesh_probe_setup(&mut e, 0, 1, 1);
        e.call_log = Some(vec![]);
        assert!(pathing_find_point_on_nav_mesh(
            &mut e,
            1,
            2,
            Ptr::new(given),
            10.0,
            Ptr::new(out)
        ));
        assert_eq!(point(&e, out), [3.0, 4.0, 5.0]);
        let log = log_of(&mut e);
        assert!(calls_to(&log, NAVMESH_PROBE_B).is_empty());
        assert_eq!(calls_to(&log, NAVMESH_FIND_TRIANGLE)[0][1], given);

        // The second probe refuses: the triangle's center as well.
        let mut e = engine();
        navmesh_probe_setup(&mut e, 1, 0, 1);
        assert!(pathing_find_point_on_nav_mesh(
            &mut e,
            1,
            2,
            Ptr::new(given),
            10.0,
            Ptr::new(out)
        ));
        assert_eq!(point(&e, out), [3.0, 4.0, 5.0]);

        // No triangle: failure, with the holder and location released.
        let mut e = engine();
        navmesh_probe_setup(&mut e, 0, 0, 0);
        e.call_log = Some(vec![]);
        assert!(!pathing_find_point_on_nav_mesh(
            &mut e,
            1,
            2,
            Ptr::new(given),
            10.0,
            Ptr::new(out)
        ));
        let log = log_of(&mut e);
        assert_eq!(calls_to(&log, NAV_HOLDER_RELEASE).len(), 1);
        assert_eq!(calls_to(&log, LOCATION_DESTRUCT).len(), 1);
    }

    // -- 006d7350, 006d7490 -------------------------------------------------

    #[test]
    fn line_of_sight_to_a_location_without_info_needs_a_start_without_info() {
        let mut e = engine();
        let (from, to, result) = (e.mem.alloc(0x28), e.mem.alloc(0x28), e.mem.alloc(0x28));
        e.call_log = Some(vec![]);
        // Neither location has navmesh info: nothing blocks.
        assert!(pathing_check_line_of_sight(
            &mut e,
            Ptr::new(from),
            Ptr::new(to),
            Ptr::new(result),
            Ptr::new(0)
        ));
        let log = log_of(&mut e);
        // The result location receives the start first.
        assert_eq!(calls_to(&log, LOCATION_ASSIGN), vec![vec![result, from]]);
        // The start has info: no line of sight.
        e.register_double(LOCATION_RESOLVE_NAVMESH_INFO, move |_, a| {
            ret((a[0] == from) as u32)
        });
        assert!(!pathing_check_line_of_sight(
            &mut e,
            Ptr::new(from),
            Ptr::new(to),
            Ptr::new(0),
            Ptr::new(0)
        ));
    }

    #[test]
    fn line_of_sight_passes_the_target_navmesh_and_triangle_on() {
        // The start has no info, so the walk answers whether it was given
        // no target navmesh.
        let mut e = engine();
        let (from, to) = (e.mem.alloc(0x28), e.mem.alloc(0x28));
        e.register_double(LOCATION_RESOLVE_NAVMESH_INFO, move |_, a| {
            ret((a[0] == to) as u32)
        });
        e.register(NI_POINTER_GET_FROM_FIELD, |_, _| ret(NAV));
        e.register(LOCATION_GET_NAVMESH_AND_TRIANGLE, |e, a| {
            e.mem.set_u16(a[2], 6);
            ret(1)
        });
        e.call_log = Some(vec![]);
        // The target has a navmesh and a triangle.
        assert!(!pathing_check_line_of_sight(
            &mut e,
            Ptr::new(from),
            Ptr::new(to),
            Ptr::new(0),
            Ptr::new(0)
        ));
        let log = log_of(&mut e);
        assert_eq!(calls_to(&log, NAV_HOLDER_RELEASE).len(), 1);
        assert_eq!(calls_to(&log, LOCATION_COPY_POSITION).len(), 1);
        // Without a triangle for the target there is no target navmesh.
        e.register(LOCATION_GET_NAVMESH_AND_TRIANGLE, |_, _| ret(0));
        assert!(pathing_check_line_of_sight(
            &mut e,
            Ptr::new(from),
            Ptr::new(to),
            Ptr::new(0),
            Ptr::new(0)
        ));
        // The second overload takes the position as it is and no target.
        e.call_log = Some(vec![]);
        assert!(pathing_check_line_of_sight_ov2(
            &mut e,
            Ptr::new(from),
            Ptr::new(0x1234),
            Ptr::new(0),
            Ptr::new(0)
        ));
        assert!(calls_to(&log_of(&mut e), LOCATION_COPY_POSITION).is_empty());
    }

    // -- 006d74c0, 006d7a20 -------------------------------------------------

    /// What `walk_setup` records of every `00698060` call: the arguments
    /// and the planar hit point it was given.
    type Crossings = Rc<RefCell<Vec<(Vec<u32>, [f32; 2], [f32; 2])>>>;

    /// Doubles for the walk of `fn_006d74c0`: the start (1, 2) has info
    /// and triangle 1 of `NAV`; the crossing edges the walk finds are
    /// `script` in order (the first walk step writes the hit (11, 12), the
    /// next (12, 13), ...); crossing an edge leads to triangle 4 across its
    /// edge 1 of the same navmesh; the center of a triangle is (0, 0, 55).
    /// Returns the start, the end (9, 8, 7) and the recorded crossings.
    fn walk_setup(e: &mut Engine, script: Vec<u32>) -> (u32, u32, Crossings) {
        let (start, end) = (e.mem.alloc(0x28), e.mem.alloc(12));
        set_point(e, end, [9.0, 8.0, 7.0]);
        e.register(LOCATION_RESOLVE_NAVMESH_INFO, |_, _| ret(1));
        e.register(LOCATION_GET_NAVMESH_AND_TRIANGLE, |e, a| {
            e.mem.set_u16(a[2], 1);
            ret(1)
        });
        e.register(LOCATION_COPY_POSITION, |e, a| {
            set_point(e, a[1], [1.0, 2.0, 0.0]);
            ret(a[1])
        });
        e.register(POINT2_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            ret(a[0])
        });
        e.register(NI_POINTER_GET_FROM_FIELD, |_, _| ret(NAV));
        let crossings = Crossings::default();
        let sink = crossings.clone();
        let mut script = script.into_iter();
        e.register_double(NAVMESH_CROSSED_EDGE, move |e, a| {
            let mut sink = sink.borrow_mut();
            let hit = [e.mem.f32(a[3]), e.mem.f32(a[3] + 4)];
            let end = [e.mem.f32(a[4]), e.mem.f32(a[4] + 4)];
            sink.push((a.to_vec(), hit, end));
            let step = sink.len() as f32;
            e.mem.set_f32(a[5], 10.0 + step);
            e.mem.set_f32(a[5] + 4, 11.0 + step);
            ret(script.next().expect("a scripted crossing"))
        });
        e.register(NAVMESH_GET_MATCHING_EDGE_B, |e, a| {
            e.mem.set_u16(a[4], 4);
            e.mem.set_u32(a[5], 1);
            ret(1)
        });
        e.register(NAVMESH_GET_INFO, |_, a| ret(a[0] + 0x1000));
        e.register(NAVMESH_GET_CENTER, |e, a| {
            set_point(e, a[1], [0.0, 0.0, 55.0]);
            ret(a[1])
        });
        e.register(EDGE_REFERENCE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u16(a[0] + 4, a[2] as u16);
            e.mem.set_u32(a[0] + 8, a[3]);
            ret(a[0])
        });
        e.register(EDGE_REFERENCE_COPY, |e, a| {
            for word in 0..3 {
                let value = e.mem.u32(a[1] + 4 * word);
                e.mem.set_u32(a[0] + 4 * word, value);
            }
            ret(a[0])
        });
        e.register(POINT3_CONSTRUCT, |e, a| {
            for i in 1..4 {
                e.mem.set_u32(a[0] + 4 * (i - 1), a[i as usize]);
            }
            ret(a[0])
        });
        (start, end, crossings)
    }

    #[test]
    fn walk_follows_the_crossed_edges_to_the_end() {
        let mut e = engine();
        let (start, end, crossings) = walk_setup(&mut e, vec![2, 0xffff_ffff]);
        e.call_log = Some(vec![]);
        assert!(fn_006d74c0(
            &mut e,
            Ptr::new(start),
            Ptr::new(end),
            0,
            0xffff,
            Ptr::new(0),
            Ptr::new(0)
        ));
        let log = log_of(&mut e);
        let crossings = crossings.borrow();
        // First from triangle 1 with no previous edge, between the planar
        // start (1, 2) and end (9, 8); then from triangle 4 (entered by
        // its edge 1) from the hit point the first step found.
        let first = &crossings[0];
        assert_eq!(first.0[..3], [NAV, 1, 0xffff_ffff]);
        assert_eq!((first.1, first.2), ([1.0, 2.0], [9.0, 8.0]));
        let second = &crossings[1];
        assert_eq!(second.0[..3], [NAV, 4, 1]);
        assert_eq!(second.1, [11.0, 12.0]);
        // The second step's hit becomes the hit point.
        assert_eq!(crossings.len(), 2);
        // All three holders are released and nothing else was touched.
        assert_eq!(calls_to(&log, NAV_HOLDER_RELEASE).len(), 3);
        assert!(calls_to(&log, LOCATION_ASSIGN).is_empty());
    }

    fn point_2d(e: &Engine, address: u32) -> [f32; 2] {
        [e.mem.f32(address), e.mem.f32(address + 4)]
    }

    #[test]
    fn walk_reports_where_it_ended() {
        let mut e = engine();
        let (start, end, _) = walk_setup(&mut e, vec![2, 0xffff_ffff]);
        let result = e.mem.alloc(0x28);
        e.call_log = Some(vec![]);
        assert!(fn_006d74c0(
            &mut e,
            Ptr::new(start),
            Ptr::new(end),
            0,
            0xffff,
            Ptr::new(result),
            Ptr::new(0)
        ));
        let log = log_of(&mut e);
        // The result starts as the start location, then gets the info of
        // the navmesh, the triangle 4 and the end position.
        assert_eq!(calls_to(&log, LOCATION_ASSIGN), vec![vec![result, start]]);
        assert_eq!(
            calls_to(&log, LOCATION_SET_NAVMESH_INFO),
            vec![vec![result, NAV + 0x1000]]
        );
        assert_eq!(calls_to(&log, LOCATION_SET_TRIANGLE), vec![vec![result, 4]]);
        assert_eq!(
            calls_to(&log, LOCATION_SET_POSITION),
            vec![vec![result, end]]
        );
    }

    #[test]
    fn walk_must_end_in_the_target_triangle() {
        for (navmesh, triangle, expected) in
            [(NAV, 4u32, true), (NAV, 5, false), (NAV + 4, 4, false)]
        {
            let mut e = engine();
            e.register(LOCATION_CONSTRUCT_FROM_MESH_POINT, |_, a| ret(a[0]));
            let (start, end, _) = walk_setup(&mut e, vec![2, 0xffff_ffff]);
            let result = e.mem.alloc(0x28);
            e.call_log = Some(vec![]);
            assert_eq!(
                fn_006d74c0(
                    &mut e,
                    Ptr::new(start),
                    Ptr::new(end),
                    navmesh,
                    triangle,
                    Ptr::new(result),
                    Ptr::new(0)
                ),
                expected
            );
            let log = log_of(&mut e);
            if expected {
                // The result is the end position on the target triangle: a
                // location built on the stack, assigned and destroyed.
                let built = &calls_to(&log, LOCATION_CONSTRUCT_FROM_MESH_POINT)[0];
                assert_eq!(built[1..], [end, NAV + 0x1000, 4]);
                assert_eq!(calls_to(&log, LOCATION_ASSIGN)[1], vec![result, built[0]]);
                assert_eq!(calls_to(&log, LOCATION_DESTRUCT), vec![vec![built[0]]]);
            } else {
                assert!(calls_to(&log, LOCATION_CONSTRUCT_FROM_MESH_POINT).is_empty());
            }
            assert_eq!(calls_to(&log, NAV_HOLDER_RELEASE).len(), 3);
        }
    }

    #[test]
    fn walk_is_stopped_by_a_refusing_filter() {
        let mut e = engine();
        let (start, end, _) = walk_setup(&mut e, vec![2]);
        let filter = e.mem.alloc(8);
        e.put_vtable(0x00a6_0000, &[0x00a6_1000]);
        e.mem.set_u32(filter, 0x00a6_0000);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = seen.clone();
        e.register_double(0x00a6_1000, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            ret(0)
        });
        let placed = Rc::new(RefCell::new(Vec::new()));
        let sink = placed.clone();
        e.register_double(LOCATION_SET_POSITION, move |e, a| {
            sink.borrow_mut().push(point(e, a[1]));
            Ret::default()
        });
        let result = e.mem.alloc(0x28);
        e.call_log = Some(vec![]);
        assert!(!fn_006d74c0(
            &mut e,
            Ptr::new(start),
            Ptr::new(end),
            0,
            0xffff,
            Ptr::new(result),
            Ptr::new(filter)
        ));
        let log = log_of(&mut e);
        // The filter got the references of the crossing by value: the edge
        // left (navmesh, triangle 1, edge 2) and the edge entered
        // (navmesh, triangle 4, edge 1).
        assert_eq!(*seen.borrow(), vec![vec![filter, NAV, 1, 2, NAV, 4, 1]]);
        // The result is where the walk stopped: triangle 1, at the hit
        // point (11, 12) with the height of the center, 55.
        assert_eq!(
            calls_to(&log, LOCATION_SET_NAVMESH_INFO),
            vec![vec![result, NAV + 0x1000]]
        );
        assert_eq!(calls_to(&log, LOCATION_SET_TRIANGLE), vec![vec![result, 1]]);
        assert_eq!(*placed.borrow(), vec![[11.0, 12.0, 55.0]]);
        // The next holder, the current one and the start's are released.
        assert_eq!(calls_to(&log, NAV_HOLDER_RELEASE).len(), 3);
    }

    #[test]
    fn walk_continues_when_the_filter_accepts_and_stops_at_blocking_edges() {
        let mut e = engine();
        let (start, end, _) = walk_setup(&mut e, vec![2, 0xffff_ffff]);
        let filter = e.mem.alloc(8);
        e.put_vtable(0x00a6_0000, &[0x00a6_1000]);
        e.mem.set_u32(filter, 0x00a6_0000);
        e.register(0x00a6_1000, |_, _| ret(1));
        assert!(fn_006d74c0(
            &mut e,
            Ptr::new(start),
            Ptr::new(end),
            0,
            0xffff,
            Ptr::new(0),
            Ptr::new(filter)
        ));

        // An edge whose extra info is 1 blocks the walk, other infos do not.
        for (value, expected) in [(1u32, false), (2, true)] {
            let mut e = engine();
            let (start, end, _) = walk_setup(&mut e, vec![2, 0xffff_ffff]);
            let info = e.mem.alloc(4);
            e.mem.set_u32(info, value);
            e.register_double(NAVMESH_EDGE_EXTRA_INFO, move |_, _| ret(info));
            assert_eq!(
                fn_006d74c0(
                    &mut e,
                    Ptr::new(start),
                    Ptr::new(end),
                    0,
                    0xffff,
                    Ptr::new(0),
                    Ptr::new(0)
                ),
                expected
            );
        }

        // An edge without a neighbour blocks it as well.
        let mut e = engine();
        let (start, end, _) = walk_setup(&mut e, vec![2]);
        e.register(NAVMESH_GET_MATCHING_EDGE_B, |_, _| ret(0));
        assert!(!fn_006d74c0(
            &mut e,
            Ptr::new(start),
            Ptr::new(end),
            0,
            0xffff,
            Ptr::new(0),
            Ptr::new(0)
        ));
    }

    #[test]
    fn walk_recomputes_the_hit_point_when_the_navmesh_changes() {
        let mut e = engine();
        let (start, end, crossings) = walk_setup(&mut e, vec![2, 0xffff_ffff]);
        e.register(HOLDER_DIFFERS, |_, _| ret(1));
        let vertices = e.mem.alloc(24);
        set_point(&mut e, vertices, [1.0, 2.0, 3.0]);
        set_point(&mut e, vertices + 12, [4.0, 5.0, 6.0]);
        e.register_double(NAVMESH_EDGE_VERTICES, move |e, a| {
            assert_eq!((a[0], a[2], a[3]), (NAV, 4, 1));
            e.mem.set_u32(a[1], vertices);
            e.mem.set_u32(a[1] + 4, vertices + 12);
            Ret::default()
        });
        let closest = e.mem.alloc(8);
        e.mem.set_f32(closest, 21.0);
        e.mem.set_f32(closest + 4, 22.0);
        let ends = Rc::new(RefCell::new(Vec::new()));
        let sink = ends.clone();
        e.register_double(POINT2_SEGMENT_CLOSEST, move |e, a| {
            sink.borrow_mut()
                .push((point_2d(e, a[1]), point_2d(e, a[2]), point_2d(e, a[3])));
            ret(closest)
        });
        assert!(fn_006d74c0(
            &mut e,
            Ptr::new(start),
            Ptr::new(end),
            0,
            0xffff,
            Ptr::new(0),
            Ptr::new(0)
        ));
        // The planar ends of the new edge and the previous hit point.
        assert_eq!(*ends.borrow(), vec![([1.0, 2.0], [4.0, 5.0], [11.0, 12.0])]);
        // The walk goes on from the point found.
        assert_eq!(crossings.borrow()[1].1, [21.0, 22.0]);
    }

    #[test]
    fn walk_without_navmesh_info_or_a_triangle_fails_early() {
        // No info for the start: success only without a target navmesh.
        let mut e = engine();
        let (start, end, _) = walk_setup(&mut e, vec![]);
        e.register(LOCATION_RESOLVE_NAVMESH_INFO, |_, _| ret(0));
        for (navmesh, expected) in [(0, true), (NAV, false)] {
            assert_eq!(
                fn_006d74c0(
                    &mut e,
                    Ptr::new(start),
                    Ptr::new(end),
                    navmesh,
                    0xffff,
                    Ptr::new(0),
                    Ptr::new(0)
                ),
                expected
            );
        }
        // No triangle for the start: failure, the holder is released.
        let mut e = engine();
        let (start, end, _) = walk_setup(&mut e, vec![]);
        e.register(LOCATION_GET_NAVMESH_AND_TRIANGLE, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert!(!fn_006d74c0(
            &mut e,
            Ptr::new(start),
            Ptr::new(end),
            0,
            0xffff,
            Ptr::new(0),
            Ptr::new(0)
        ));
        assert_eq!(calls_to(&log_of(&mut e), NAV_HOLDER_RELEASE).len(), 1);
    }

    #[test]
    fn holder_comparison_is_the_exe_functions_answer() {
        let mut e = engine();
        e.register_double(HOLDER_DIFFERS, |_, a| ret((a[0] != a[1]) as u32));
        assert!(fn_006d7a20(&mut e, Ptr::new(1), Ptr::new(2)));
        assert!(!fn_006d7a20(&mut e, Ptr::new(2), Ptr::new(2)));
    }

    #[test]
    fn straightening_wrapper_asks_for_neither_a_length_nor_a_path() {
        let mut e = engine();
        let arrays = Arrays::default();
        let (from, to) = straighten_setup(&mut e, &arrays);
        // Neither location resolves: success, and no length is computed.
        e.register(LOCATION_RESOLVE_CLOSEST, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert!(fn_006d62c0(
            &mut e,
            Ptr::new(from),
            Ptr::new(to),
            Ptr::new(0)
        ));
        assert!(calls_to(&log_of(&mut e), POINT3_LENGTH).is_empty());
        // The second resolves alone: failure.
        e.register_double(LOCATION_RESOLVE_CLOSEST, move |_, a| {
            ret((a[0] == to) as u32)
        });
        assert!(!fn_006d62c0(
            &mut e,
            Ptr::new(from),
            Ptr::new(to),
            Ptr::new(0)
        ));
    }

    #[test]
    fn line_of_sight_overload_two_walks_without_a_target() {
        let mut e = engine();
        let (from, point_to) = (e.mem.alloc(0x28), e.mem.alloc(12));
        let result = e.mem.alloc(0x28);
        e.call_log = Some(vec![]);
        // The start has no info, so the walk is true exactly for no target
        // navmesh.
        assert!(pathing_check_line_of_sight_ov2(
            &mut e,
            Ptr::new(from),
            Ptr::new(point_to),
            Ptr::new(result),
            Ptr::new(0)
        ));
        assert_eq!(
            log_of(&mut e),
            vec![
                (LOCATION_ASSIGN, vec![result, from]),
                (LOCATION_RESOLVE_NAVMESH_INFO, vec![from, 0])
            ]
        );
    }

    // -- 006d7a40 and the LOS-map functions up to 006d9830 -------------------

    /// `NiPointer` slots read the pointer they hold, as the exe's getter does.
    fn real_holders(e: &mut Engine) {
        e.register(NI_POINTER_GET_FROM_FIELD, |e, a| ret(e.mem.u32(a[0])));
    }

    /// `NiPoint3` construction stores the three `float`s and returns the point.
    fn construct_points(e: &mut Engine) {
        e.register(POINT3_CONSTRUCT, |e, a| {
            set_point(e, a[0], [float_arg(a[1]), float_arg(a[2]), float_arg(a[3])]);
            ret(a[0])
        });
    }

    #[test]
    fn closest_reachable_location_moves_a_copy_of_the_start_to_the_found_point() {
        let mut e = engine();
        let (out, from, to) = (e.mem.alloc(0x28), e.mem.alloc(0x28), e.mem.alloc(0x28));
        e.register(LOCATION_RESOLVE_NAVMESH_INFO, |_, _| ret(1));
        e.register_double(LOCATION_GET_NAVMESH_AND_TRIANGLE, move |e, a| {
            e.mem.set_u16(a[2], if a[0] == from { 7 } else { 9 });
            ret(1)
        });
        let navmeshes = Rc::new(RefCell::new(vec![NAVMESH, LAST_NAVMESH]));
        let queue = navmeshes.clone();
        e.register_double(NI_POINTER_GET_FROM_FIELD, move |_, _| {
            ret(queue.borrow_mut().remove(0))
        });
        e.register(LOCATION_COPY_POSITION, |_, a| ret(a[1]));
        let searches = Rc::new(RefCell::new(Vec::new()));
        let sink = searches.clone();
        e.register_double(MAX_COST_SEARCH_FIND_POINT, move |e, a| {
            let reference = |address: u32| (e.mem.u32(address), e.mem.u16(address + 4));
            sink.borrow_mut()
                .push((reference(a[3]), reference(a[5]), float_arg(a[6])));
            set_point(e, a[1], [4.0, 5.0, 6.0]);
            Ret::default()
        });
        let moved_to = record_points(&mut e, LOCATION_SET_POSITION, 1);
        e.call_log = Some(vec![]);
        assert_eq!(
            pathing_find_closest_reachable_location(
                &mut e,
                Ptr::new(out),
                Ptr::new(from),
                Ptr::new(to),
                123.0
            ),
            Ptr::new(out)
        );
        assert_eq!(
            *searches.borrow(),
            vec![((NAVMESH, 7), (LAST_NAVMESH, 9), 123.0)]
        );
        assert_eq!(*moved_to.borrow(), vec![[4.0, 5.0, 6.0]]);
        let log = log_of(&mut e);
        let copies = calls_to(&log, LOCATION_COPY_CONSTRUCT);
        // A copy of `from`, then `out` copied from that moved copy.
        assert_eq!(copies.len(), 2);
        assert_eq!(copies[0][1], from);
        assert_eq!(copies[1], vec![out, copies[0][0]]);
        assert_eq!(calls_to(&log, LOCATION_DESTRUCT), vec![vec![copies[0][0]]]);
        assert_eq!(calls_to(&log, MAX_COST_SEARCH_DESTRUCT).len(), 1);
        assert_eq!(calls_to(&log, NAV_HOLDER_RELEASE).len(), 2);
    }

    #[test]
    fn closest_reachable_location_without_a_goal_triangle_passes_an_empty_goal() {
        let mut e = engine();
        let (out, from, to) = (e.mem.alloc(0x28), e.mem.alloc(0x28), e.mem.alloc(0x28));
        e.register_double(LOCATION_RESOLVE_NAVMESH_INFO, move |_, a| {
            ret((a[0] == from) as u32)
        });
        e.register(LOCATION_GET_NAVMESH_AND_TRIANGLE, |e, a| {
            e.mem.set_u16(a[2], 3);
            ret(1)
        });
        real_holders(&mut e);
        e.register(LOCATION_COPY_POSITION, |_, a| ret(a[1]));
        let goals = Rc::new(RefCell::new(Vec::new()));
        let sink = goals.clone();
        e.register_double(MAX_COST_SEARCH_FIND_POINT, move |e, a| {
            sink.borrow_mut()
                .push((e.mem.u32(a[5]), e.mem.u16(a[5] + 4)));
            Ret::default()
        });
        pathing_find_closest_reachable_location(
            &mut e,
            Ptr::new(out),
            Ptr::new(from),
            Ptr::new(to),
            1.0,
        );
        assert_eq!(*goals.borrow(), vec![(0, 0xffff)]);
    }

    #[test]
    fn closest_reachable_location_copies_the_start_when_it_cannot_be_resolved() {
        for triangle_found in [false, true] {
            let mut e = engine();
            let (out, from, to) = (e.mem.alloc(0x28), e.mem.alloc(0x28), e.mem.alloc(0x28));
            e.register_double(LOCATION_RESOLVE_NAVMESH_INFO, move |_, _| {
                ret(triangle_found as u32)
            });
            e.call_log = Some(vec![]);
            assert_eq!(
                pathing_find_closest_reachable_location(
                    &mut e,
                    Ptr::new(out),
                    Ptr::new(from),
                    Ptr::new(to),
                    1.0
                ),
                Ptr::new(out)
            );
            let log = log_of(&mut e);
            assert_eq!(
                calls_to(&log, LOCATION_COPY_CONSTRUCT),
                vec![vec![out, from]]
            );
            assert!(calls_to(&log, MAX_COST_SEARCH_CONSTRUCT).is_empty());
            // With info but no triangle the holder is released.
            assert_eq!(
                calls_to(&log, NAV_HOLDER_RELEASE).len(),
                triangle_found as usize
            );
        }
    }

    /// A `PathingLOSMap` stand-in: its score buffer (+0x18) holds `scores`.
    fn los_map_with_scores(e: &mut Engine, scores: &[u8]) -> (u32, u32) {
        let map = e.mem.alloc(0x60);
        let buffer = e.mem.alloc(scores.len() as u32);
        e.mem.write(buffer, scores);
        e.mem.set_u32(map + 0x18, buffer);
        (map, buffer)
    }

    #[test]
    fn los_map_accessors_read_and_write_the_score_buffer_and_arrays() {
        let mut e = engine();
        let (map, buffer) = los_map_with_scores(&mut e, &[1, 0x7f, 0xfe]);
        assert_eq!(fn_006d7ee0(&mut e, Ptr::new(map), 2), 0xfe);
        fn_006d8970(&mut e, Ptr::new(map), 1, 9);
        assert_eq!(e.mem.u8(buffer + 1), 9);
        // The navmesh array sits at +0x1c.
        e.call_log = Some(vec![]);
        fn_006d8990(&mut e, Ptr::new(map));
        fn_006d89b0(&mut e, Ptr::new(map), 3);
        assert_eq!(
            log_of(&mut e),
            vec![
                (ARRAY_COUNT, vec![map + 0x1c]),
                (POINTER_ARRAY_ELEMENT, vec![map + 0x1c, 3])
            ]
        );
        let entry = e.mem.alloc(8);
        assert_eq!(
            fn_006d89d0(&mut e, Ptr::new(entry), 0x1234, 0xff),
            Ptr::new(entry)
        );
        assert_eq!((e.mem.u32(entry), e.mem.u8(entry + 4)), (0x1234, 0xff));
    }

    #[test]
    fn grid_map_radius_must_be_above_zero() {
        let mut e = engine();
        let grid_map = e.mem.alloc(0x54);
        for (radius, expected) in [(5.0, true), (0.0, false), (-1.0, false), (f32::NAN, false)] {
            e.mem.set_f32(grid_map + 0x44, radius);
            assert_eq!(
                fn_006d7f00(&mut e, Ptr::new(grid_map)),
                expected,
                "{radius}"
            );
        }
    }

    #[test]
    fn triangle_edge_count_counts_the_edges_with_a_neighbour() {
        let mut e = engine();
        e.register_double(TRIANGLE_RECORD_ARRAY_ELEMENT, |_, a| {
            assert_eq!(a[1], 5, "the triangle index");
            ret(0x7000)
        });
        // Edges 0 and 2 have neighbours.
        e.register(TRIANGLE_HAS_NEIGHBOUR, |_, a| ret((a[1] != 1) as u32));
        e.call_log = Some(vec![]);
        assert_eq!(fn_006d8910(&mut e, Ptr::new(0x1000), 5), 2);
        let log = log_of(&mut e);
        // The records come from the array at navmesh + 0x38.
        assert_eq!(calls_to(&log, TRIANGLE_RECORD_ARRAY_ELEMENT)[0][0], 0x1038);
        assert_eq!(calls_to(&log, TRIANGLE_HAS_NEIGHBOUR).len(), 3);
    }

    // -- 006d7f40 ----------------------------------------------------------

    const LOS_NAVMESH: u32 = 0x00c0_0300;

    /// A LOS map of one navmesh with two triangles, scores all 0x7f; the
    /// location lies in triangle 1. Every callee outside the file answers
    /// like a consistent little world: set the fields to change it.
    struct LosScene {
        slot: u32,
        map: u32,
        scores: u32,
        location: u32,
        centers: Rc<RefCell<Vec<[f32; 3]>>>,
        radius: Rc<std::cell::Cell<f32>>,
        matching: Rc<std::cell::Cell<bool>>,
        crossings: Rc<RefCell<Vec<u32>>>,
    }

    fn los_scene(e: &mut Engine) -> LosScene {
        let (map, scores) = los_map_with_scores(e, &[0x7f; 8]);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, map);
        let location = e.mem.alloc(0x28);
        let centers = Rc::new(RefCell::new(vec![[1.0, 0.0, 0.0], [2.0, 0.0, 0.0]]));
        let radius = Rc::new(std::cell::Cell::new(0.0f32));
        let matching = Rc::new(std::cell::Cell::new(true));
        let crossings = Rc::new(RefCell::new(Vec::new()));
        real_holders(e);
        vector_math(e);
        construct_points(e);
        e.register_double(POINT3_DOT, |e, a| {
            let (x, y) = (point(e, a[0]), point(e, a[1]));
            ret_float(x[0] * y[0] + x[1] * y[1] + x[2] * y[2])
        });
        e.register(SINE_AND_COSINE, |e, a| {
            e.mem.set_f32(a[1], float_arg(a[0]).sin());
            e.mem.set_f32(a[2], float_arg(a[0]).cos());
            Ret::default()
        });
        // Locations, navmesh and the list of navmeshes.
        e.register(LOCATION_RESOLVE_NAVMESH_INFO, |_, _| ret(1));
        e.register(LOCATION_GET_NAVMESH_AND_TRIANGLE, |e, a| {
            e.mem.set_u32(a[1], LOS_NAVMESH);
            e.mem.set_u16(a[2], 1);
            ret(1)
        });
        e.register(LOCATION_COPY_POSITION, |e, a| {
            set_point(e, a[1], [0.0, 0.0, 0.0]);
            ret(a[1])
        });
        e.register(NAV_HOLDER_COPY_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], LOS_NAVMESH);
            Ret::default()
        });
        e.register(POINTER_ARRAY_ELEMENT, |_, a| ret(a[0]));
        e.register(WORD_AT_OFFSET_0XC, |_, a| ret(a[0]));
        e.register(LOS_MAP_FIND_OFFSET, |e, a| {
            e.mem.set_u32(a[2], 0);
            ret(1)
        });
        e.register(NAVMESH_TRIANGLE_COUNT, |_, _| ret(2));
        e.register(NAVMESH_GET_TRIANGLE, |_, _| ret(0xdead00));
        e.register(TRIANGLE_HAS_FLAG, |_, _| ret(0));
        let sink = centers.clone();
        e.register_double(NAVMESH_GET_CENTER, move |e, a| {
            set_point(e, a[1], sink.borrow()[a[2] as usize]);
            Ret::default()
        });
        e.register(LOS_MAP_COPY_CENTER, |e, a| {
            set_point(e, a[1], [0.0, 0.0, 0.0]);
            ret(a[1])
        });
        let sink = radius.clone();
        e.register_double(FLOAT_AT_OFFSET_0X48, move |_, _| ret_float(sink.get()));
        // The score entries: a growing list of (index, score).
        let entries = Rc::new(RefCell::new(Vec::<(u32, u8)>::new()));
        let buffer = e.mem.alloc(8 * 16);
        let sink = entries.clone();
        e.register_double(SCORE_ENTRY_ARRAY_CONSTRUCT, move |_, _| {
            sink.borrow_mut().clear();
            Ret::default()
        });
        let sink = entries.clone();
        e.register_double(SCORE_ENTRY_ARRAY_ADD, move |e, a| {
            sink.borrow_mut()
                .push((e.mem.u32(a[1]), e.mem.u8(a[1] + 4)));
            ret(sink.borrow().len() as u32 - 1)
        });
        let sink = entries.clone();
        e.register_double(SCORE_ENTRY_ARRAY_ELEMENT, move |e, a| {
            let (index, score) = sink.borrow()[a[1] as usize];
            e.mem.set_u32(buffer + 8 * a[1], index);
            e.mem.set_u8(buffer + 8 * a[1] + 4, score);
            ret(buffer + 8 * a[1])
        });
        // Every array other than the navmesh list is the entry list.
        let sink = entries.clone();
        e.register_double(ARRAY_COUNT, move |_, a| {
            ret(if a[0] == map + 0x1c {
                1
            } else {
                sink.borrow().len() as u32
            })
        });
        e.register(TRIANGLE_RECORD_ARRAY_ELEMENT, |_, a| ret(a[0]));
        e.register(TRIANGLE_HAS_NEIGHBOUR, |_, a| ret((a[1] < 2) as u32));
        let sink = crossings.clone();
        e.register_double(NAVMESH_CROSSED_EDGE, move |_, _| {
            let mut pending = sink.borrow_mut();
            ret(if pending.is_empty() {
                0xffff_ffff
            } else {
                pending.remove(0)
            })
        });
        let sink = matching.clone();
        e.register_double(NAVMESH_GET_MATCHING_EDGE_C, move |e, a| {
            e.mem.set_u32(a[3], LOS_NAVMESH);
            e.mem.set_u16(a[4], 1);
            e.mem.set_u32(a[5], 0);
            ret(sink.get() as u32)
        });
        e.register(NAVMESH_EDGE_EXTRA_INFO, |_, _| ret(0));
        e.register_double(LOS_MAP_MAX_SCORE, move |e, _| {
            ret(e.mem.u8(map + 0x38) as u32)
        });
        e.register_double(LOS_MAP_MIN_SCORE, move |e, _| {
            ret(e.mem.u8(map + 0x39) as u32)
        });
        e.register_double(LOS_MAP_SET_MAX_SCORE, move |e, a| {
            e.mem.set_u8(map + 0x38, a[1] as u8);
            Ret::default()
        });
        e.register_double(LOS_MAP_SET_MIN_SCORE, move |e, a| {
            e.mem.set_u8(map + 0x39, a[1] as u8);
            Ret::default()
        });
        LosScene {
            slot,
            map,
            scores,
            location,
            centers,
            radius,
            matching,
            crossings,
        }
    }

    fn score_bytes(e: &Engine, scene: &LosScene) -> Vec<u8> {
        e.mem.bytes(scene.scores, 4)
    }

    fn score_los(e: &mut Engine, scene: &LosScene, sight: f32, heading: f32) -> bool {
        fn_006d7f40(
            e,
            Ptr::new(scene.location),
            Ptr::new(scene.slot),
            false,
            sight,
            heading,
        )
    }

    #[test]
    fn scoring_walks_to_the_location_and_scores_the_visited_triangles() {
        let mut e = engine();
        let scene = los_scene(&mut e);
        // Triangle 0 crosses its edge 0 into triangle 1, where the location is.
        scene.crossings.borrow_mut().extend([0, 0xffff_ffff]);
        assert!(score_los(&mut e, &scene, 0.0, -1.0));
        // Both visited triangles get their open-edge count, 2.
        assert_eq!(score_bytes(&e, &scene), vec![2, 2, 0x7f, 0x7f]);
        assert_eq!(e.mem.u8(scene.map + 0x38), 2, "iMaxScore widened");
        assert_eq!(e.mem.u8(scene.map + 0x39), 0);
    }

    #[test]
    fn scoring_marks_the_triangles_of_a_blocked_walk_unseen() {
        let mut e = engine();
        let scene = los_scene(&mut e);
        scene.crossings.borrow_mut().push(0);
        scene.matching.set(false);
        assert!(score_los(&mut e, &scene, 0.0, -1.0));
        // Triangle 0 is blocked (-2); triangle 1 holds the location (2).
        assert_eq!(score_bytes(&e, &scene), vec![0xfe, 2, 0x7f, 0x7f]);
        assert_eq!(e.mem.u8(scene.map + 0x39), 0xfe, "iMinScore widened");
        assert_eq!(e.mem.u8(scene.map + 0x38), 2);
    }

    #[test]
    fn scoring_skips_triangles_beyond_the_map_radius() {
        let mut e = engine();
        let scene = los_scene(&mut e);
        scene.radius.set(10.0);
        *scene.centers.borrow_mut() = vec![[100.0, 0.0, 0.0], [5.0, 0.0, 0.0]];
        assert!(score_los(&mut e, &scene, 0.0, -1.0));
        assert_eq!(score_bytes(&e, &scene), vec![0x7f, 2, 0x7f, 0x7f]);
        // The height difference counts three times.
        let mut e = engine();
        let scene = los_scene(&mut e);
        scene.radius.set(10.0);
        *scene.centers.borrow_mut() = vec![[0.0, 0.0, 4.0], [0.0, 0.0, 1.0]];
        assert!(score_los(&mut e, &scene, 0.0, -1.0));
        assert_eq!(score_bytes(&e, &scene), vec![0x7f, 2, 0x7f, 0x7f]);
    }

    #[test]
    fn scoring_marks_triangles_beyond_the_sight_distance_unseen() {
        let mut e = engine();
        let scene = los_scene(&mut e);
        *scene.centers.borrow_mut() = vec![[10.0, 0.0, 0.0], [1.0, 0.0, 0.0]];
        assert!(score_los(&mut e, &scene, 3.0, -1.0));
        // Triangle 0 is too far: minus its open-edge count, no walk.
        assert_eq!(score_bytes(&e, &scene), vec![0xfe, 2, 0x7f, 0x7f]);
    }

    #[test]
    fn scoring_marks_triangles_behind_the_heading_unseen() {
        let mut e = engine();
        let scene = los_scene(&mut e);
        // Heading 0: the direction is (sin, cos, 0) = (0, 1, 0).
        *scene.centers.borrow_mut() = vec![[0.0, -10.0, 0.0], [0.0, 5.0, 0.0]];
        assert!(score_los(&mut e, &scene, 0.0, 0.0));
        assert_eq!(score_bytes(&e, &scene), vec![0xfe, 2, 0x7f, 0x7f]);
    }

    #[test]
    fn scoring_resets_the_map_first_when_asked() {
        let mut e = engine();
        let scene = los_scene(&mut e);
        e.call_log = Some(vec![]);
        fn_006d7f40(
            &mut e,
            Ptr::new(scene.location),
            Ptr::new(scene.slot),
            true,
            0.0,
            -1.0,
        );
        let log = log_of(&mut e);
        assert_eq!(calls_to(&log, LOS_MAP_RESET_SCORES), vec![vec![scene.map]]);
    }

    #[test]
    fn scoring_fails_without_navmeshes_info_or_a_triangle() {
        let mut e = engine();
        let scene = los_scene(&mut e);
        e.register(ARRAY_COUNT, |_, _| ret(0));
        assert!(!score_los(&mut e, &scene, 0.0, -1.0));
        let mut e = engine();
        let scene = los_scene(&mut e);
        e.register(LOCATION_RESOLVE_NAVMESH_INFO, |_, _| ret(0));
        assert!(!score_los(&mut e, &scene, 0.0, -1.0));
        let mut e = engine();
        let scene = los_scene(&mut e);
        e.register(LOCATION_GET_NAVMESH_AND_TRIANGLE, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert!(!score_los(&mut e, &scene, 0.0, -1.0));
        assert_eq!(calls_to(&log_of(&mut e), NAV_HOLDER_RELEASE).len(), 1);
    }

    #[test]
    fn grid_map_refresh_scores_every_location_and_counts_the_seen_triangles() {
        let mut e = engine();
        let slot = e.mem.alloc(4);
        let grid_map = e.mem.alloc(0x54);
        e.mem.set_u32(slot, grid_map);
        e.mem.set_f32(grid_map + 0x44, 40.0);
        let (los_map, _) = los_map_with_scores(&mut e, &[5, 0x7f, 0xfe, 0, 9]);
        e.mem.set_u32(los_map + 0x2c, 5);
        // A freshly built LOS map is held by the local holder.
        real_holders(&mut e);
        e.register(OBJECT_ALLOCATE, |_, a| {
            assert_eq!(a[0], 0x50);
            ret(0x00d0_0000)
        });
        e.register(LOS_GRID_MAP_RADIUS, |_, _| ret_float(40.0));
        e.register(LOS_GRID_MAP_COPY_CENTER, |e, a| {
            set_point(e, a[1], [1.0, 2.0, 3.0]);
            ret(a[1])
        });
        e.register_double(LOS_MAP_CONSTRUCT, move |_, a| {
            assert_eq!(a[0], 0x00d0_0000);
            assert_eq!(
                (
                    float_arg(a[1]),
                    float_arg(a[2]),
                    float_arg(a[3]),
                    float_arg(a[4])
                ),
                (1.0, 2.0, 3.0, 40.0)
            );
            ret(los_map)
        });
        e.register(NI_POINTER_CONSTRUCT_FROM, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(ARRAY_COUNT, |_, _| ret(0));
        e.register(LOS_MAP_TRIANGLE_COUNT, |e, a| ret(e.mem.u32(a[0] + 0x2c)));
        let counts = Rc::new(RefCell::new(Vec::new()));
        let sink = counts.clone();
        e.register_double(LOS_MAP_SET_SEEN_COUNT, move |_, a| {
            sink.borrow_mut().push(("seen", a[1]));
            Ret::default()
        });
        let sink = counts.clone();
        e.register_double(LOS_MAP_SET_UNSEEN_COUNT, move |_, a| {
            sink.borrow_mut().push(("unseen", a[1]));
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(fn_006d7ca0(
            &mut e,
            Ptr::new(slot),
            Ptr::new(0x1000),
            Ptr::new(0)
        ));
        // 5 is seen; 0xfe and 0 are not; 0x7f is unscored; 9 is seen.
        assert_eq!(*counts.borrow(), vec![("seen", 2), ("unseen", 2)]);
        let log = log_of(&mut e);
        assert_eq!(calls_to(&log, LOS_GRID_MAP_FILL_LOS_MAP).len(), 1);
        assert_eq!(calls_to(&log, LOS_GRID_MAP_MERGE_LOS_MAP).len(), 1);
        assert_eq!(calls_to(&log, NI_POINTER_DESTRUCT).len(), 1);
    }

    #[test]
    fn grid_map_refresh_passes_each_location_with_its_heading() {
        let mut e = engine();
        let slot = e.mem.alloc(4);
        let grid_map = e.mem.alloc(0x54);
        e.mem.set_u32(slot, grid_map);
        e.mem.set_f32(grid_map + 0x44, 40.0);
        let (los_map, _) = los_map_with_scores(&mut e, &[]);
        real_holders(&mut e);
        e.register(OBJECT_ALLOCATE, |_, _| ret(0x00d0_0000));
        e.register(LOS_GRID_MAP_COPY_CENTER, |_, a| ret(a[1]));
        e.register_double(LOS_MAP_CONSTRUCT, move |_, _| ret(los_map));
        e.register(NI_POINTER_CONSTRUCT_FROM, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(LOCATION_ARRAY_ELEMENT, |_, a| ret(0x2000 + 0x28 * a[1]));
        let headings = e.mem.alloc(8);
        e.mem.set_f32(headings, 0.5);
        e.mem.set_f32(headings + 4, 1.5);
        e.register_double(POINTER_ARRAY_ELEMENT_THUNK, move |_, a| {
            ret(headings + 4 * a[1])
        });
        // The scorer fails at once for lack of navmesh info; the locations
        // it was given show in the info checks.
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = seen.clone();
        e.register_double(LOCATION_RESOLVE_NAVMESH_INFO, move |_, a| {
            sink.borrow_mut().push(a[0]);
            ret(0)
        });
        e.register(ARRAY_COUNT, |_, a| ret(if a[0] == 0x1000 { 2 } else { 1 }));
        e.mem.set_u32(los_map + 0x2c, 0);
        e.register(LOS_MAP_TRIANGLE_COUNT, |_, _| ret(0));
        e.call_log = Some(vec![]);
        // `fn_006d7f40` stops at `fn_006d8990` (the navmesh count is 1, so it
        // goes on to the info check) once per location.
        assert!(fn_006d7ca0(
            &mut e,
            Ptr::new(slot),
            Ptr::new(0x1000),
            Ptr::new(0x3000)
        ));
        assert_eq!(*seen.borrow(), vec![0x2000, 0x2028]);
        let log = log_of(&mut e);
        // The headings array is read for both locations.
        assert_eq!(
            calls_to(&log, POINTER_ARRAY_ELEMENT_THUNK),
            vec![vec![0x3000, 0], vec![0x3000, 1]]
        );
    }

    #[test]
    fn grid_map_refresh_does_nothing_without_a_radius() {
        let mut e = engine();
        let slot = e.mem.alloc(4);
        let grid_map = e.mem.alloc(0x54);
        e.mem.set_u32(slot, grid_map);
        real_holders(&mut e);
        e.call_log = Some(vec![]);
        assert!(!fn_006d7ca0(
            &mut e,
            Ptr::new(slot),
            Ptr::new(0x1000),
            Ptr::new(0)
        ));
        assert!(calls_to(&log_of(&mut e), OBJECT_ALLOCATE).is_empty());
    }

    // -- 006d8a40 .. 006d9830 ----------------------------------------------

    #[test]
    fn box_overlap_needs_every_axis_to_overlap() {
        let mut e = engine();
        let corners = e.mem.alloc(0x30);
        let overlap = |e: &mut Engine, a: [f32; 3], b: [f32; 3], c: [f32; 3], d: [f32; 3]| {
            for (i, p) in [a, b, c, d].iter().enumerate() {
                set_point(e, corners + 12 * i as u32, *p);
            }
            fn_006d93d0(
                e,
                Ptr::new(corners),
                Ptr::new(corners + 12),
                Ptr::new(corners + 24),
                Ptr::new(corners + 36),
            )
        };
        let (low, high) = ([0.0; 3], [10.0; 3]);
        assert!(overlap(&mut e, low, high, [5.0; 3], [15.0; 3]));
        // Touching boxes do not overlap: b.x == c.x, then a.y == d.y.
        assert!(!overlap(&mut e, low, high, [10.0, 5.0, 5.0], [15.0; 3]));
        assert!(!overlap(
            &mut e,
            [0.0, 15.0, 0.0],
            [10.0, 25.0, 10.0],
            [5.0; 3],
            [15.0; 3]
        ));
        // Separated on the last axis only.
        assert!(!overlap(&mut e, low, high, [5.0, 5.0, 11.0], [15.0; 3]));
        assert!(!overlap(
            &mut e,
            [0.0, 0.0, 20.0],
            [10.0, 10.0, 30.0],
            [5.0; 3],
            [15.0; 3]
        ));
        // A NaN comparison passes.
        assert!(overlap(&mut e, low, high, [f32::NAN, 5.0, 5.0], [15.0; 3]));
    }

    #[test]
    fn navmesh_z_for_location_reads_the_triangle_height() {
        let mut e = engine();
        let (location, out) = (e.mem.alloc(0x28), e.mem.alloc(4));
        real_holders(&mut e);
        e.register(LOCATION_RESOLVE_NAVMESH_INFO, |_, _| ret(1));
        e.register(LOCATION_HAS_TRIANGLE, |_, _| ret(1));
        e.register(LOCATION_GET_NAVMESH_AND_TRIANGLE, |e, a| {
            e.mem.set_u32(a[1], NAVMESH);
            e.mem.set_u16(a[2], 6);
            ret(1)
        });
        e.register(LOCATION_COPY_POSITION, |e, a| {
            set_point(e, a[1], [1.0, 2.0, 9.5]);
            ret(a[1])
        });
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = seen.clone();
        e.register_double(NAVMESH_COMPUTE_TRIANGLE_Z, move |e, a| {
            sink.borrow_mut()
                .push((a[0], a[1], float_arg(a[3]), float_arg(a[4])));
            e.mem.set_f32(a[5], 3.25);
            ret(1)
        });
        e.call_log = Some(vec![]);
        assert!(pathing_get_nav_mesh_z_for_location_ov2(
            &mut e,
            Ptr::new(location),
            Ptr::new(out)
        ));
        assert_eq!(e.mem.f32(out), 3.25);
        assert_eq!(*seen.borrow(), vec![(NAVMESH, 6, 180.0, 180.0)]);
        assert_eq!(calls_to(&log_of(&mut e), NAV_HOLDER_RELEASE).len(), 1);
    }

    #[test]
    fn navmesh_z_for_location_falls_back_to_the_locations_own_height() {
        // (no info, no triangle, no triangle for the holder, outside the triangle)
        for stage in 0..4 {
            let mut e = engine();
            let (location, out) = (e.mem.alloc(0x28), e.mem.alloc(4));
            real_holders(&mut e);
            e.register(LOCATION_COPY_POSITION, |e, a| {
                set_point(e, a[1], [1.0, 2.0, 9.5]);
                ret(a[1])
            });
            e.register_double(LOCATION_RESOLVE_NAVMESH_INFO, move |_, _| {
                ret((stage > 0) as u32)
            });
            e.register_double(LOCATION_HAS_TRIANGLE, move |_, _| ret((stage > 1) as u32));
            e.register_double(LOCATION_GET_NAVMESH_AND_TRIANGLE, move |_, _| {
                ret((stage > 2) as u32)
            });
            e.register(NAVMESH_COMPUTE_TRIANGLE_Z, |_, _| ret(0));
            e.call_log = Some(vec![]);
            assert!(!pathing_get_nav_mesh_z_for_location_ov2(
                &mut e,
                Ptr::new(location),
                Ptr::new(out)
            ));
            assert_eq!(e.mem.f32(out), 9.5, "stage {stage}");
            // The holder exists from stage 2 on and is released every time.
            assert_eq!(
                calls_to(&log_of(&mut e), NAV_HOLDER_RELEASE).len(),
                (stage > 1) as usize,
                "stage {stage}"
            );
        }
    }

    #[test]
    fn navmesh_z_for_a_position_builds_the_location_first() {
        let mut e = engine();
        let (position, out) = (e.mem.alloc(12), e.mem.alloc(4));
        e.register(LOCATION_RESOLVE_NAVMESH_INFO, |_, _| ret(0));
        e.register(LOCATION_COPY_POSITION, |e, a| {
            set_point(e, a[1], [0.0, 0.0, 7.0]);
            ret(a[1])
        });
        e.call_log = Some(vec![]);
        assert!(!pathing_get_nav_mesh_z_for_location(
            &mut e,
            Ptr::new(position),
            Ptr::new(0x111),
            Ptr::new(0x222),
            Ptr::new(out)
        ));
        let log = log_of(&mut e);
        let built = calls_to(&log, LOCATION_CONSTRUCT_FROM_PARTS);
        assert_eq!(built.len(), 1);
        assert_eq!(built[0][1..], [position, 0x111, 0x222]);
        assert_eq!(calls_to(&log, LOCATION_DESTRUCT), vec![vec![built[0][0]]]);
        assert_eq!(e.mem.f32(out), 7.0);
    }

    #[test]
    fn navmesh_normal_copies_the_triangle_normal() {
        let mut e = engine();
        let (location, out, normal) = (e.mem.alloc(0x28), e.mem.alloc(12), e.mem.alloc(12));
        set_point(&mut e, normal, [0.0, 0.0, 1.0]);
        real_holders(&mut e);
        e.register(LOCATION_RESOLVE_NAVMESH_INFO, |_, _| ret(1));
        e.register(LOCATION_GET_NAVMESH_AND_TRIANGLE, |e, a| {
            e.mem.set_u32(a[1], NAVMESH);
            e.mem.set_u16(a[2], 4);
            ret(1)
        });
        e.register_double(TRIANGLE_NORMAL, move |_, a| {
            assert_eq!((a[0], a[2]), (NAVMESH, 4));
            ret(normal)
        });
        assert!(pathing_get_navmesh_normal_for_location(
            &mut e,
            Ptr::new(location),
            Ptr::new(out)
        ));
        assert_eq!(point(&e, out), [0.0, 0.0, 1.0]);
    }

    #[test]
    fn navmesh_normal_fails_without_info_or_triangle() {
        let mut e = engine();
        let (location, out) = (e.mem.alloc(0x28), e.mem.alloc(12));
        assert!(!pathing_get_navmesh_normal_for_location(
            &mut e,
            Ptr::new(location),
            Ptr::new(out)
        ));
        e.register(LOCATION_RESOLVE_NAVMESH_INFO, |_, _| ret(1));
        e.call_log = Some(vec![]);
        assert!(!pathing_get_navmesh_normal_for_location(
            &mut e,
            Ptr::new(location),
            Ptr::new(out)
        ));
        assert_eq!(calls_to(&log_of(&mut e), NAV_HOLDER_RELEASE).len(), 1);
    }

    #[test]
    fn all_navmeshes_come_from_the_interior_cell_or_the_exterior_cells() {
        let mut e = engine();
        e.set_global(TES_GLOBAL, 0x1234u32);
        let list = 0x2000u32;
        let added = Rc::new(RefCell::new(Vec::new()));
        let sink = added.clone();
        e.register_double(NAV_HOLDER_LIST_ADD, move |_, a| {
            sink.borrow_mut().push(a[1]);
            ret(0)
        });
        e.register(CELL_NAVMESH_ARRAY, |_, a| {
            ret(if a[0] == 0x7001 { 0 } else { 0x9000 + a[0] })
        });
        e.register(NAVMESH_ARRAY_COUNT, |_, _| ret(2));
        e.register(NAVMESH_ARRAY_GET, |_, a| ret(a[1]));
        e.call_log = Some(vec![]);
        // Interior cell at +0x34 of the TES object.
        e.register_double(WORD_AT_OFFSET_0X34, |_, a| {
            assert_eq!(a[0], 0x1234);
            ret(0x7000)
        });
        assert!(fn_006d91d0(&mut e, Ptr::new(list)));
        let log = log_of(&mut e);
        assert_eq!(added.borrow().len(), 2);
        assert_eq!(calls_to(&log, NAV_HOLDER_RELEASE).len(), 2);
        assert_eq!(calls_to(&log, TES_EXTERIOR_CELL_COUNT).len(), 0);
        // Without an interior cell: the exterior cells, skipping the null
        // cell and the cell without navmesh array.
        added.borrow_mut().clear();
        e.register(WORD_AT_OFFSET_0X34, |_, _| ret(0));
        e.register(TES_EXTERIOR_CELL_COUNT, |_, _| ret(3));
        e.register(TES_EXTERIOR_CELL_AT, |_, a| {
            ret([0, 0x7000, 0x7001][a[1] as usize])
        });
        assert!(fn_006d91d0(&mut e, Ptr::new(list)));
        assert_eq!(added.borrow().len(), 2, "only cell 0x7000 has navmeshes");
    }

    #[test]
    fn form_query_forwards_to_the_cell_a_form_designates() {
        let mut e = engine();
        let results = e.mem.alloc(0x10);
        e.mem.set_u32(results + 0xc, 4);
        e.register(WORD_AT_OFFSET_0XC, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register_double(WORD_AT_OFFSET_0X34, |_, a| ret(a[0] + 0x1000));
        let forwarded = Rc::new(RefCell::new(Vec::new()));
        let sink = forwarded.clone();
        e.register_double(CELL_QUERY_006D9350, move |_, a| {
            sink.borrow_mut().push((a[0], a[1], float_arg(a[2]), a[3]));
            ret(77)
        });
        e.register_double(FORM_TYPE, |_, a| {
            ret(if a[0] == 0x500 { 0x41 } else { 0x39 })
        });
        e.call_log = Some(vec![]);
        // A form of type 0x41 forwards its +0x34 word and room is reserved.
        assert_eq!(
            fn_006d9350(&mut e, Ptr::new(0x500), 9, 2.5, Ptr::new(results)),
            77
        );
        // Any other form is forwarded itself; the array is big enough now.
        e.mem.set_u32(results + 0xc, 0x20);
        assert_eq!(
            fn_006d9350(&mut e, Ptr::new(0x600), 8, 1.5, Ptr::new(results)),
            77
        );
        assert_eq!(
            *forwarded.borrow(),
            vec![(0x1500, 9, 2.5, results), (0x600, 8, 1.5, results)]
        );
        let log = log_of(&mut e);
        assert_eq!(
            calls_to(&log, ARRAY_SET_RESERVED_SIZE),
            vec![vec![results, 0x20]]
        );
        // Nothing to ask without a form, a results array or a cell.
        assert_eq!(
            fn_006d9350(&mut e, Ptr::new(0), 1, 1.0, Ptr::new(results)),
            0
        );
        assert_eq!(fn_006d9350(&mut e, Ptr::new(0x600), 1, 1.0, Ptr::new(0)), 0);
        e.register(WORD_AT_OFFSET_0X34, |_, _| ret(0));
        assert_eq!(
            fn_006d9350(&mut e, Ptr::new(0x500), 1, 1.0, Ptr::new(results)),
            0
        );
    }

    /// The doubles the navmesh collection of `fn_006d8a40` needs; returns
    /// the holders added to the list.
    fn nearby_scene(e: &mut Engine, list: u32) -> Rc<RefCell<Vec<u32>>> {
        real_holders(e);
        construct_points(e);
        e.register(LOCATION_GET_CELL, |_, _| ret(0x7000));
        e.register(LOCATION_COPY_POSITION, |e, a| {
            set_point(e, a[1], [5.0 * 4096.0 + 100.0, 7.0 * 4096.0 + 100.0, 0.0]);
            ret(a[1])
        });
        let added = Rc::new(RefCell::new(Vec::new()));
        let sink = added.clone();
        e.register_double(NAV_HOLDER_LIST_ADD, move |_, a| {
            sink.borrow_mut().push(a[1]);
            ret(0)
        });
        let sink = added.clone();
        e.register_double(ARRAY_COUNT, move |_, a| {
            ret(if a[0] == list {
                sink.borrow().len() as u32
            } else {
                0
            })
        });
        added
    }

    #[test]
    fn nearby_navmeshes_need_a_cell() {
        let mut e = engine();
        let list = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert!(!fn_006d8a00(&mut e, Ptr::new(0x1000), 50.0, Ptr::new(list)));
        assert!(calls_to(&log_of(&mut e), CELL_LIST_CONSTRUCT).is_empty());
    }

    #[test]
    fn nearby_navmeshes_of_a_single_navmesh_request_are_the_locations_own() {
        let mut e = engine();
        let list = e.mem.alloc(0x10);
        let added = nearby_scene(&mut e, list);
        e.register(LOCATION_RESOLVE_NAVMESH_INFO, |_, _| ret(1));
        e.register(LOCATION_GET_NAVMESH, |e, a| {
            e.mem.set_u32(a[1], NAVMESH);
            ret(1)
        });
        e.call_log = Some(vec![]);
        assert!(fn_006d8a20(&mut e, Ptr::new(0x1000), 50.0, Ptr::new(list)));
        assert_eq!(added.borrow().len(), 1);
        let log = log_of(&mut e);
        assert!(calls_to(&log, CELL_LIST_CONSTRUCT).is_empty());
        // Failure paths: no info, no navmesh (the holder is released).
        e.register(LOCATION_RESOLVE_NAVMESH_INFO, |_, _| ret(0));
        assert!(!fn_006d8a20(&mut e, Ptr::new(0x1000), 50.0, Ptr::new(list)));
        e.register(LOCATION_RESOLVE_NAVMESH_INFO, |_, _| ret(1));
        e.register(LOCATION_GET_NAVMESH, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert!(!fn_006d8a20(&mut e, Ptr::new(0x1000), 50.0, Ptr::new(list)));
        assert_eq!(calls_to(&log_of(&mut e), NAV_HOLDER_RELEASE).len(), 1);
    }

    #[test]
    fn nearby_exterior_cells_are_those_overlapping_the_search_square() {
        let mut e = engine();
        let list = e.mem.alloc(0x10);
        nearby_scene(&mut e, list);
        e.register(CELL_IS_INTERIOR, |_, _| ret(0));
        e.register(CELL_GET_DATA_X, |_, _| ret(5));
        e.register(CELL_GET_DATA_Y, |_, _| ret(7));
        let looked_up = Rc::new(RefCell::new(Vec::new()));
        let sink = looked_up.clone();
        e.register_double(WORLDSPACE_GET_CELL_FROM_COORD, move |_, a| {
            sink.borrow_mut().push((a[1], a[2]));
            ret(0x8000)
        });
        let cells = Rc::new(RefCell::new(Vec::new()));
        let sink = cells.clone();
        e.register_double(INFO_ARRAY_ADD, move |e, a| {
            sink.borrow_mut().push(e.mem.u32(a[1]));
            ret(0)
        });
        // Nothing was collected, so the list stays empty.
        e.register(ARRAY_IS_EMPTY, |_, _| ret(1));
        e.call_log = Some(vec![]);
        assert!(!fn_006d8a00(&mut e, Ptr::new(0x1000), 50.0, Ptr::new(list)));
        // Only the cell (5, 7) of the 3 by 3 block touches the square.
        assert_eq!(*looked_up.borrow(), vec![(5, 7)]);
        assert_eq!(*cells.borrow(), vec![0x8000]);
        assert_eq!(calls_to(&log_of(&mut e), CELL_LIST_DESTRUCT).len(), 1);
    }

    #[test]
    fn nearby_navmeshes_include_overlapping_neighbours_of_the_same_cell() {
        let mut e = engine();
        let list = e.mem.alloc(0x10);
        let added = nearby_scene(&mut e, list);
        // An interior cell holding one navmesh whose info lists one neighbour.
        e.register(CELL_IS_INTERIOR, |_, _| ret(1));
        let cell_slot = e.mem.alloc(4);
        e.mem.set_u32(cell_slot, 0x7000);
        e.register_double(POINTER_ARRAY_ELEMENT, move |e, a| {
            if a[0] == 0x6024 {
                let candidate = e.mem.alloc(4);
                e.mem.set_u32(candidate, 0x7700);
                ret(candidate)
            } else {
                ret(cell_slot)
            }
        });
        e.register(INFO_ARRAY_ADD, |_, _| ret(0));
        e.register(ARRAY_IS_EMPTY, |_, _| ret(0));
        let cells = Rc::new(std::cell::Cell::new(0u32));
        let sink = cells.clone();
        e.register_double(CELL_LIST_CONSTRUCT, move |_, a| {
            sink.set(a[0]);
            Ret::default()
        });
        let sink = cells.clone();
        let added_for_count = added.clone();
        e.register_double(ARRAY_COUNT, move |_, a| {
            ret(if a[0] == sink.get() {
                1
            } else if a[0] == list {
                added_for_count.borrow().len() as u32
            } else if a[0] == 0x6024 {
                1
            } else {
                0
            })
        });
        e.register(CELL_NAVMESH_ARRAY, |_, _| ret(0x9000));
        e.register(NAVMESH_ARRAY_COUNT, |_, _| ret(1));
        e.register(NAVMESH_ARRAY_GET, |_, _| ret(0xab00));
        e.register(NAVMESH_GET_INFO, |_, _| ret(0x6000));
        e.register(NAVMESH_INFO_GET_NAVMESH, |e, a| {
            e.mem.set_u32(a[1], 0xcd00);
            ret(1)
        });
        let sink = added.clone();
        e.register_double(HOLDER_LIST_FIND_INDEX, move |_, a| {
            ret(if sink.borrow().contains(&a[1]) {
                0
            } else {
                0xffff_ffff
            })
        });
        // The neighbour lies in the location's cell, with bounds that overlap
        // the square around the location.
        e.register(NAVMESH_GET_CELL, |_, _| ret(0x7000));
        e.register(NAVMESH_GET_BOUNDS, |e, a| {
            set_point(e, a[1], [5.0 * 4096.0, 7.0 * 4096.0, -10.0]);
            set_point(e, a[2], [5.0 * 4096.0 + 200.0, 7.0 * 4096.0 + 200.0, 10.0]);
            ret(1)
        });
        assert!(fn_006d8a00(&mut e, Ptr::new(0x1000), 50.0, Ptr::new(list)));
        // The cell's navmesh, then the neighbour.
        assert_eq!(added.borrow().len(), 2);
        // The same neighbour in another cell is left out.
        added.borrow_mut().clear();
        e.register(NAVMESH_GET_CELL, |_, _| ret(0x7100));
        assert!(fn_006d8a00(&mut e, Ptr::new(0x1000), 50.0, Ptr::new(list)));
        assert_eq!(added.borrow().len(), 1);
        // One already in the list, or with a null cell, is left out.
        added.borrow_mut().clear();
        e.register(NAVMESH_GET_CELL, |_, _| ret(0x7000));
        e.register(HOLDER_LIST_FIND_INDEX, |_, _| ret(0));
        assert!(fn_006d8a00(&mut e, Ptr::new(0x1000), 50.0, Ptr::new(list)));
        assert_eq!(added.borrow().len(), 1);
        added.borrow_mut().clear();
        let sink = added.clone();
        e.register_double(HOLDER_LIST_FIND_INDEX, move |_, a| {
            ret(if sink.borrow().contains(&a[1]) {
                0
            } else {
                0xffff_ffff
            })
        });
        e.register(NAVMESH_GET_CELL, |_, _| ret(0));
        assert!(fn_006d8a00(&mut e, Ptr::new(0x1000), 50.0, Ptr::new(list)));
        assert_eq!(added.borrow().len(), 1);
    }

    /// `fn_006d9480` with one navmesh holding one triangle at (10, 20, 5).
    fn open_edge_scene(e: &mut Engine, neighbours: [bool; 3]) {
        real_holders(e);
        vector_math(e);
        construct_points(e);
        e.register(LOCATION_GET_CELL, |_, _| ret(0x7000));
        e.register(LOCATION_RESOLVE_NAVMESH_INFO, |_, _| ret(1));
        e.register(LOCATION_GET_NAVMESH, |e, a| {
            e.mem.set_u32(a[1], NAVMESH);
            ret(1)
        });
        e.register(LOCATION_COPY_POSITION, |e, a| {
            set_point(e, a[1], [10.0, 20.0, 5.0]);
            ret(a[1])
        });
        e.register(NAV_HOLDER_LIST_ADD, |_, _| ret(0));
        let arrays = Rc::new(RefCell::new([0u32; 3]));
        let sink = arrays.clone();
        e.register_double(NAVMESH_LIST_CONSTRUCT, move |_, a| {
            sink.borrow_mut()[0] = a[0];
            Ret::default()
        });
        let sink = arrays.clone();
        e.register_double(TRIANGLE_REFERENCE_ARRAY_CONSTRUCT, move |_, a| {
            sink.borrow_mut()[1] = a[0];
            Ret::default()
        });
        let sink = arrays.clone();
        e.register_double(U16_ARRAY_CONSTRUCT, move |_, a| {
            sink.borrow_mut()[2] = a[0];
            Ret::default()
        });
        let sink = arrays.clone();
        e.register_double(ARRAY_COUNT, move |_, a| {
            // The navmesh list, the triangle references and the found
            // triangles each hold one entry; any navmesh info has none.
            ret(sink.borrow().contains(&a[0]) as u32)
        });
        e.register(NAVMESH_GET_INFO, |_, _| ret(0x6000));
        let navmesh_slot = e.mem.alloc(4);
        e.mem.set_u32(navmesh_slot, NAVMESH);
        e.register_double(POINTER_ARRAY_ELEMENT, move |_, _| ret(navmesh_slot));
        let triangle_slot = e.mem.alloc(2);
        e.mem.set_u16(triangle_slot, 5);
        e.register_double(U16_ARRAY_ELEMENT, move |_, _| ret(triangle_slot));
        e.register(TRIANGLE_REFERENCE_CONSTRUCT, |_, a| ret(a[0]));
        let reference = e.mem.alloc(8);
        e.mem.set_u32(reference, 0x6000);
        e.mem.set_u16(reference + 4, 5);
        e.register_double(SCORE_ENTRY_ARRAY_ELEMENT, move |_, _| ret(reference));
        e.register(NAVMESH_INFO_GET_NAVMESH, |e, a| {
            e.mem.set_u32(a[1], NAVMESH);
            ret(1)
        });
        e.register(NAVMESH_GET_TRIANGLE, |_, _| ret(0xdead00));
        e.register_double(TRIANGLE_HAS_NEIGHBOUR, move |_, a| {
            ret(neighbours[a[1] as usize] as u32)
        });
        let vertices = e.mem.alloc(24);
        e.register_double(NAVMESH_EDGE_VERTICES, move |e, a| {
            e.mem.set_u32(a[1], vertices);
            e.mem.set_u32(a[1] + 4, vertices + 12);
            Ret::default()
        });
    }

    #[test]
    fn open_edges_are_tested_against_the_radius_around_the_location() {
        let mut e = engine();
        open_edge_scene(&mut e, [true, false, true]);
        let boxes = Rc::new(RefCell::new(Vec::new()));
        let sink = boxes.clone();
        e.register_double(NAVMESH_FIND_TRIANGLES_IN_BOX, move |e, a| {
            sink.borrow_mut()
                .push((a[0], point(e, a[1]), point(e, a[2])));
            Ret::default()
        });
        let tested = Rc::new(RefCell::new(Vec::new()));
        let sink = tested.clone();
        e.register_double(SEGMENT_WITHIN_RADIUS, move |e, a| {
            sink.borrow_mut().push((point(e, a[2]), float_arg(a[3])));
            ret(0)
        });
        e.call_log = Some(vec![]);
        assert!(fn_006d9480(&mut e, Ptr::new(0x1000), 3.0));
        // The box: the location -/+ (radius, radius, 0).
        assert_eq!(
            *boxes.borrow(),
            vec![(NAVMESH, [7.0, 17.0, 5.0], [13.0, 23.0, 5.0])]
        );
        // Only the edge without a neighbour (edge 1) is tested.
        assert_eq!(*tested.borrow(), vec![([10.0, 20.0, 5.0], 3.0)]);
        let log = log_of(&mut e);
        assert_eq!(calls_to(&log, SCRATCH_OBJECT_CONSTRUCT).len(), 1);
        assert_eq!(calls_to(&log, SCRATCH_OBJECT_DESTRUCT).len(), 1);
        assert_eq!(calls_to(&log, NAVMESH_LIST_DESTRUCT).len(), 1);
        assert_eq!(calls_to(&log, TRIANGLE_REFERENCE_ARRAY_DESTRUCT).len(), 1);
        assert_eq!(calls_to(&log, U16_ARRAY_DESTRUCT).len(), 1);
    }

    #[test]
    fn an_open_edge_within_the_radius_fails_and_cleans_up() {
        let mut e = engine();
        open_edge_scene(&mut e, [true, true, false]);
        e.register(SEGMENT_WITHIN_RADIUS, |_, _| ret(1));
        e.call_log = Some(vec![]);
        assert!(!fn_006d9480(&mut e, Ptr::new(0x1000), 3.0));
        let log = log_of(&mut e);
        assert_eq!(calls_to(&log, SEGMENT_WITHIN_RADIUS).len(), 1);
        assert_eq!(calls_to(&log, SCRATCH_OBJECT_DESTRUCT).len(), 1);
        assert_eq!(calls_to(&log, NAVMESH_LIST_DESTRUCT).len(), 1);
        assert_eq!(calls_to(&log, TRIANGLE_REFERENCE_ARRAY_DESTRUCT).len(), 1);
        // The holder of the failing reference is released.
        assert!(!calls_to(&log, NAV_HOLDER_RELEASE).is_empty());
    }

    #[test]
    fn fully_connected_triangles_have_no_open_edges() {
        let mut e = engine();
        open_edge_scene(&mut e, [true, true, true]);
        e.register(SEGMENT_WITHIN_RADIUS, |_, _| panic!("no edge to test"));
        assert!(fn_006d9480(&mut e, Ptr::new(0x1000), 3.0));
    }

    // -- 006d99a0 .. 006db3b0 ----------------------------------------------

    /// The world the tangent functions look at: the heading points along
    /// +y, the sideways direction along +x, the start is the origin and the
    /// target `to` is at (0, 300, 0). `failing` lists the line-of-sight
    /// checks (1-based, in call order) that fail.
    struct TangentScene {
        built: Rc<RefCell<Vec<u32>>>,
        destroyed: Rc<RefCell<Vec<u32>>>,
        cosine: Rc<std::cell::Cell<f32>>,
        length: Rc<std::cell::Cell<f32>>,
    }

    fn tangent_scene(e: &mut Engine, failing: &[u32]) -> TangentScene {
        let failing = failing.to_vec();
        vector_math(e);
        e.register(POINT3_DEFAULT_CONSTRUCTOR, |_, a| ret(a[0]));
        e.register(MATRIX_FROM_HEADING, |_, _| Ret::default());
        e.register(MATRIX_TIMES_VECTOR, |e, a| {
            set_point(e, a[1], [0.0, 1.0, 0.0]);
            ret(a[1])
        });
        e.register(POINT3_UNIT_CROSS, |e, a| {
            set_point(e, a[1], [1.0, 0.0, 0.0]);
            ret(a[1])
        });
        let built = Rc::new(RefCell::new(Vec::new()));
        let copies = Rc::new(RefCell::new(Vec::new()));
        let (sink, copy_sink) = (built.clone(), copies.clone());
        e.register_double(LOCATION_COPY_CONSTRUCT, move |_, a| {
            sink.borrow_mut().push(a[0]);
            copy_sink.borrow_mut().push(a[0]);
            ret(a[0])
        });
        let sink = built.clone();
        e.register_double(LOCATION_CONSTRUCT, move |_, a| {
            sink.borrow_mut().push(a[0]);
            ret(a[0])
        });
        let sink = built.clone();
        e.register_double(LOCATION_CONSTRUCT_FROM_POINT_AND_LOCATION, move |_, a| {
            sink.borrow_mut().push(a[0]);
            ret(a[0])
        });
        let destroyed = Rc::new(RefCell::new(Vec::new()));
        let sink = destroyed.clone();
        e.register_double(LOCATION_DESTRUCT, move |_, a| {
            sink.borrow_mut().push(a[0]);
            Ret::default()
        });
        // Only the copy made of `to` (the second copy) has a position.
        e.register_double(LOCATION_COPY_POSITION, move |e, a| {
            let is_target = copies.borrow().get(1) == Some(&a[0]);
            set_point(
                e,
                a[1],
                if is_target {
                    [0.0, 300.0, 0.0]
                } else {
                    [0.0; 3]
                },
            );
            ret(a[1])
        });
        // Every line-of-sight check starts by assigning the result.
        let checks = Rc::new(std::cell::Cell::new(0u32));
        let resolves = Rc::new(std::cell::Cell::new(0u32));
        let (check_count, resolve_count) = (checks.clone(), resolves.clone());
        e.register_double(LOCATION_ASSIGN, move |_, _| {
            check_count.set(check_count.get() + 1);
            resolve_count.set(0);
            Ret::default()
        });
        e.register_double(LOCATION_RESOLVE_NAVMESH_INFO, move |_, _| {
            resolves.set(resolves.get() + 1);
            let check = checks.get();
            // Checks 1 and 4 walk to a point (`fn_006d7490`): they fail when
            // the start has info but no triangle. The others compare two
            // locations: they fail when only the start has info.
            let walks_to_a_point = check == 1 || check == 4;
            ret((failing.contains(&check) && (walks_to_a_point || resolves.get() == 2)) as u32)
        });
        e.register(LOCATION_GET_NAVMESH_AND_TRIANGLE, |_, _| ret(0));
        let cosine = Rc::new(std::cell::Cell::new(1.0f32));
        let length = Rc::new(std::cell::Cell::new(300.0f32));
        let sink = cosine.clone();
        e.register_double(POINT3_DOT, move |_, _| ret_float(sink.get()));
        let sink = length.clone();
        e.register_double(POINT3_UNITIZE_GET_LENGTH, move |_, _| ret_float(sink.get()));
        e.register(POINT3_SUBTRACT_ASSIGN, |e, a| {
            let (x, y) = (point(e, a[0]), point(e, a[1]));
            set_point(e, a[0], [x[0] - y[0], x[1] - y[1], x[2] - y[2]]);
            ret(a[0])
        });
        TangentScene {
            built,
            destroyed,
            cosine,
            length,
        }
    }

    fn run_first_tangent(e: &mut Engine, distance: f32) -> [f32; 3] {
        let out = e.mem.alloc(12);
        let (from, to) = (e.mem.alloc(0x28), e.mem.alloc(0x28));
        set_point(e, out, [9.0, 9.0, 9.0]);
        assert_eq!(
            fn_006d99a0(
                e,
                Ptr::new(out),
                0.5,
                Ptr::new(from),
                Ptr::new(to),
                distance
            ),
            Ptr::new(out)
        );
        point(e, out)
    }

    #[test]
    fn first_tangent_points_from_the_start_to_the_pulled_back_end() {
        let mut e = engine();
        let scene = tangent_scene(&mut e, &[]);
        e.call_log = Some(vec![]);
        // The end is 256 ahead, pulled back by 1.2 * 10.
        assert_close(run_first_tangent(&mut e, 10.0), [0.0, 244.0, 0.0]);
        let log = log_of(&mut e);
        // The heading rotation and the sideways vectors use the globals.
        assert_eq!(calls_to(&log, MATRIX_FROM_HEADING)[0][1], 0.5f32.to_bits());
        assert_eq!(calls_to(&log, MATRIX_TIMES_VECTOR)[0][2], CIRCLE_AXIS_A);
        assert!(calls_to(&log, POINT3_UNIT_CROSS)
            .iter()
            .all(|call| call[2] == TANGENT_CROSS_AXIS));
        // Twelve locations are built and destroyed in reverse.
        let built = scene.built.borrow().clone();
        assert_eq!(built.len(), 12);
        let mut reversed = built.clone();
        reversed.reverse();
        assert_eq!(*scene.destroyed.borrow(), reversed);
    }

    #[test]
    fn first_tangent_pulls_the_end_to_where_a_side_line_stops() {
        let mut e = engine();
        // The first side line stops at the origin, nearer than the end.
        tangent_scene(&mut e, &[2]);
        assert_close(run_first_tangent(&mut e, 10.0), [0.0, -12.0, 0.0]);
        let mut e = engine();
        tangent_scene(&mut e, &[3]);
        assert_close(run_first_tangent(&mut e, 10.0), [0.0, -12.0, 0.0]);
        // The straight line stopping at the origin does the same.
        let mut e = engine();
        tangent_scene(&mut e, &[1]);
        assert_close(run_first_tangent(&mut e, 10.0), [0.0, -12.0, 0.0]);
    }

    #[test]
    fn first_tangent_is_zero_when_the_target_or_its_side_lines_are_blocked() {
        for (failing, built) in [(&[4u32][..], 8usize), (&[5], 10), (&[6], 12)] {
            let mut e = engine();
            let scene = tangent_scene(&mut e, failing);
            assert_eq!(
                run_first_tangent(&mut e, 10.0),
                [0.0, 0.0, 0.0],
                "{failing:?}"
            );
            assert_eq!(scene.built.borrow().len(), built, "{failing:?}");
            assert_eq!(scene.destroyed.borrow().len(), built, "{failing:?}");
        }
    }

    #[test]
    fn first_tangent_rejects_a_target_behind_the_heading() {
        // (cosine, distance to the target, zero?)
        for (cosine, length, zero) in [
            (-0.8, 500.0, true),
            (-0.5, 100.0, false),
            (-0.5, 10.0, true),
            (0.5, 10.0, false),
            (0.0, 10.0, false),
        ] {
            let mut e = engine();
            let scene = tangent_scene(&mut e, &[]);
            scene.cosine.set(cosine);
            scene.length.set(length);
            let expected = if zero { [0.0; 3] } else { [0.0, 244.0, 0.0] };
            assert_close(run_first_tangent(&mut e, 10.0), expected);
        }
    }

    #[test]
    fn second_tangent_is_the_start_minus_the_pulled_in_end() {
        let mut e = engine();
        let scene = tangent_scene(&mut e, &[]);
        let (out, location) = (e.mem.alloc(12), e.mem.alloc(0x28));
        assert_eq!(
            fn_006da420(&mut e, Ptr::new(out), 0.5, Ptr::new(location), 10.0),
            Ptr::new(out)
        );
        // The end point is 256 ahead of the start: start - end.
        assert_close(point(&e, out), [0.0, -256.0, 0.0]);
        // Six locations: the copy, the line-of-sight result and four
        // sideways ones.
        assert_eq!(scene.built.borrow().len(), 6);
        let mut reversed = scene.built.borrow().clone();
        reversed.reverse();
        assert_eq!(*scene.destroyed.borrow(), reversed);
        // A failing check pulls the end back to where it stopped (the
        // origin, the start).
        for failing in [&[1u32][..], &[2], &[3]] {
            let mut e = engine();
            tangent_scene(&mut e, failing);
            let (out, location) = (e.mem.alloc(12), e.mem.alloc(0x28));
            fn_006da420(&mut e, Ptr::new(out), 0.5, Ptr::new(location), 10.0);
            assert_close(point(&e, out), [0.0, 0.0, 0.0]);
        }
    }

    // -- the profiler --------------------------------------------------------

    #[test]
    fn profiling_without_navmesh_info_does_nothing_but_clean_up() {
        let mut e = engine();
        let (cell, position) = (e.mem.alloc(0x100), e.mem.alloc(12));
        e.register(CELL_GET_DATA_X, |_, _| ret(1));
        e.register(CELL_GET_DATA_Y, |_, _| ret(2));
        e.call_log = Some(vec![]);
        assert_eq!(
            pathing_profile_pathing(&mut e, Ptr::new(cell), Ptr::new(position), 0, 4),
            0.0
        );
        let log = log_of(&mut e);
        assert_eq!(calls_to(&log, LOCATION_DESTRUCT).len(), 1);
        assert_eq!(calls_to(&log, NAV_HOLDER_RELEASE).len(), 1);
        assert!(calls_to(&log, POINT_ARRAY_CONSTRUCT).is_empty());
    }

    /// A profiler world: an exterior cell (1, 2), random points at the
    /// middle of the cell square, an array of `count` points.
    fn profile_scene(e: &mut Engine, count: u32) -> (u32, u32, u32) {
        let (cell, position) = (e.mem.alloc(0x100), e.mem.alloc(12));
        set_point(e, position, [1.0, 2.0, 7.0]);
        let points = e.mem.alloc(12 * count.max(1));
        e.register(CELL_GET_DATA_X, |_, _| ret(1));
        e.register(CELL_GET_DATA_Y, |_, _| ret(2));
        e.register(LOCATION_RESOLVE_NAVMESH_INFO, |_, _| ret(1));
        e.register_double(POINT_ARRAY_ELEMENT, move |_, a| ret(points + 12 * a[1]));
        e.register(RANDOM_FLOAT, |_, a| {
            ret_float((float_arg(a[0]) + float_arg(a[1])) / 2.0)
        });
        e.register(POINT2_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            ret(a[0])
        });
        vector_math(e);
        construct_points(e);
        // The cover search reads the actor location of its request.
        let actor_location = e.mem.alloc(12);
        e.register_double(COVER_ACTOR_LOCATION, move |_, _| ret(actor_location));
        (cell, position, points)
    }

    #[test]
    fn profiling_mode_zero_compares_the_polygon_tests_over_four_points() {
        let mut e = engine();
        let (cell, position, points) = profile_scene(&mut e, 4);
        let tests = Rc::new(RefCell::new(Vec::new()));
        let sink = tests.clone();
        e.register_double(POLYGON_TEST_A, move |e, a| {
            let corners: Vec<[f32; 2]> = (0..4)
                .map(|i| [e.mem.f32(a[i]), e.mem.f32(a[i] + 4)])
                .collect();
            sink.borrow_mut().push(corners);
            ret(1)
        });
        // The second test disagrees on the third polygon only.
        let calls = Rc::new(std::cell::Cell::new(0));
        let counter = calls.clone();
        e.register_double(POLYGON_TEST_B, move |_, _| {
            counter.set(counter.get() + 1);
            ret((counter.get() != 3) as u32)
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            pathing_profile_pathing(&mut e, Ptr::new(cell), Ptr::new(position), 0, 4),
            0.0
        );
        let log = log_of(&mut e);
        // Random points in the cell (1, 2) square [4096, 8192] x [8192, 12288]
        // at height 7.
        assert_eq!(calls_to(&log, RANDOM_FLOAT).len(), 8);
        assert_eq!(
            calls_to(&log, RANDOM_FLOAT)[0][..2],
            [4096.0f32.to_bits(), 8192.0f32.to_bits()]
        );
        assert_eq!(
            calls_to(&log, RANDOM_FLOAT)[1][..2],
            [8192.0f32.to_bits(), 12288.0f32.to_bits()]
        );
        assert_eq!(point(&e, points + 12), [6144.0, 10240.0, 7.0]);
        // Four polygons of four consecutive (cyclic) points each.
        assert_eq!(tests.borrow().len(), 4);
        assert_eq!(tests.borrow()[0], vec![[6144.0, 10240.0]; 4]);
        assert_eq!(
            calls_to(&log, DEBUG_PRINT),
            vec![vec![PROFILE_MISMATCH_MESSAGE]]
        );
        assert_eq!(calls_to(&log, POINT_ARRAY_DESTRUCT).len(), 1);
        assert_eq!(calls_to(&log, LOCATION_DESTRUCT).len(), 1);
    }

    #[test]
    fn profiling_an_interior_cell_draws_points_around_the_origin() {
        let mut e = engine();
        let (cell, position, _) = profile_scene(&mut e, 1);
        e.register(CELL_IS_INTERIOR, |_, _| ret(1));
        e.set_global(PROFILE_INTERIOR_HALF_WIDTH, 500.0f32);
        e.call_log = Some(vec![]);
        pathing_profile_pathing(&mut e, Ptr::new(cell), Ptr::new(position), 7, 1);
        let log = log_of(&mut e);
        let draws = calls_to(&log, RANDOM_FLOAT);
        assert_eq!(draws[0][..2], [(-500.0f32).to_bits(), 500.0f32.to_bits()]);
        assert_eq!(draws[1][..2], [(-500.0f32).to_bits(), 500.0f32.to_bits()]);
        // An unknown mode only draws the points.
        assert!(calls_to(&log, POLYGON_TEST_A).is_empty());
    }

    #[test]
    fn profiling_mode_one_runs_a_cover_search_from_every_point() {
        let mut e = engine();
        let (cell, position, points) = profile_scene(&mut e, 2);
        e.call_log = Some(vec![]);
        pathing_profile_pathing(&mut e, Ptr::new(cell), Ptr::new(position), 1, 2);
        let log = log_of(&mut e);
        assert_eq!(calls_to(&log, COVER_REQUEST_CONSTRUCT).len(), 2);
        // The request's point (+0xB0) is the profiler's point.
        let request = calls_to(&log, COVER_REQUEST_CONSTRUCT)[1][0];
        assert_eq!(point(&e, request + 0xb0), point(&e, points + 12));
        assert_eq!(calls_to(&log, REQUEST_COPY_DESTRUCT).len(), 2);
        assert_eq!(calls_to(&log, COVER_LOCATION_ARRAY_CLEAR).len(), 2);
    }

    #[test]
    fn profiling_mode_two_solves_a_request_to_every_point_found_on_the_navmesh() {
        let mut e = engine();
        let (cell, position, _) = profile_scene(&mut e, 2);
        e.register(RANDOM_FLOAT, |_, _| ret_float(0.0));
        let tries = Rc::new(std::cell::Cell::new(0));
        let counter = tries.clone();
        e.register_double(LOCATION_GET_NAVMESH, move |_, _| {
            counter.set(counter.get() + 1);
            ret(0)
        });
        e.call_log = Some(vec![]);
        pathing_profile_pathing(&mut e, Ptr::new(cell), Ptr::new(position), 2, 2);
        let log = log_of(&mut e);
        // GetNavMesh is asked once by the profiler and once per point (it
        // finds none, so each point is the random planar offset).
        assert_eq!(tries.get(), 3);
        assert_eq!(calls_to(&log, REQUEST_CONSTRUCT).len(), 2);
        assert_eq!(calls_to(&log, SOLUTION_CONSTRUCT).len(), 2);
        assert_eq!(calls_to(&log, SOLUTION_DESTRUCT).len(), 2);
        assert_eq!(calls_to(&log, BASE_SOLVER_RUN).len(), 2);
    }

    // -- small members --------------------------------------------------------

    #[test]
    fn request_point_setter_and_destructor_wrapper() {
        let mut e = engine();
        let (request, point_at) = (e.mem.alloc(0xc0), e.mem.alloc(12));
        set_point(&mut e, point_at, [1.0, 2.0, 3.0]);
        fn_006dad40(&mut e, Ptr::new(request), Ptr::new(point_at));
        assert_eq!(point(&e, request + 0xb0), [1.0, 2.0, 3.0]);
        e.call_log = Some(vec![]);
        fn_006dad70(&mut e, Ptr::new(request));
        assert_eq!(log_of(&mut e), vec![(REQUEST_COPY_DESTRUCT, vec![request])]);
    }

    #[test]
    fn path_array_element_forwards_the_index() {
        let mut e = engine();
        e.register(NODE_ARRAY_ELEMENT, |_, a| ret(a[0] + 0x14 * a[1]));
        assert_eq!(fn_006dad90(&mut e, Ptr::new(0x1000), 3), Ptr::new(0x103c));
    }

    #[test]
    fn avoid_array_slot_assignment_moves_the_references() {
        let mut e = engine();
        let slot = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        // Assigning the pointer it holds changes nothing.
        assert_eq!(fn_006dadb0(&mut e, Ptr::new(slot), 0), Ptr::new(slot));
        assert!(log_of(&mut e).is_empty());
        // From nothing to an object: a reference is taken.
        e.call_log = Some(vec![]);
        fn_006dadb0(&mut e, Ptr::new(slot), 0x5000);
        assert_eq!(e.mem.u32(slot), 0x5000);
        assert_eq!(log_of(&mut e), vec![(REFERENCE_ADD, vec![0x5010])]);
        // To another object: the old one is released first.
        e.call_log = Some(vec![]);
        fn_006dadb0(&mut e, Ptr::new(slot), 0x6000);
        assert_eq!(
            log_of(&mut e),
            vec![
                (REFERENCE_RELEASE, vec![0x5010]),
                (REFERENCE_ADD, vec![0x6010])
            ]
        );
        // To nothing: only a release.
        e.call_log = Some(vec![]);
        fn_006dadb0(&mut e, Ptr::new(slot), 0);
        assert_eq!(e.mem.u32(slot), 0);
        assert_eq!(log_of(&mut e), vec![(REFERENCE_RELEASE, vec![0x6010])]);
    }

    #[test]
    fn door_array_add_constructs_a_slot_and_copies_the_item() {
        let mut e = engine();
        let (array, item, buffer) = (e.mem.alloc(0x10), e.mem.alloc(16), e.mem.alloc(64));
        e.mem.set_u32(array + 4, buffer);
        e.mem
            .write(item, &[1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0]);
        e.register(DOOR_ARRAY_ADD_SLOT, |_, _| ret(2));
        e.call_log = Some(vec![]);
        assert_eq!(fn_006dae00(&mut e, Ptr::new(array), Ptr::new(item)), 2);
        assert_eq!(
            log_of(&mut e)[1],
            (DOOR_ELEMENTS_CONSTRUCT, vec![array, buffer + 32, 1])
        );
        assert_eq!(
            [
                e.mem.u32(buffer + 32),
                e.mem.u32(buffer + 36),
                e.mem.u32(buffer + 40),
                e.mem.u32(buffer + 44)
            ],
            [1, 2, 3, 4]
        );
    }

    #[test]
    fn cover_array_constructor_and_destructor_store_the_vtable() {
        let mut e = engine();
        let array = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(fn_006dae60(&mut e, Ptr::new(array)), Ptr::new(array));
        assert_eq!(e.mem.u32(array), COVER_LOCATION_ARRAY_VTABLE);
        fn_006dae90(&mut e, Ptr::new(array));
        assert_eq!(
            log_of(&mut e),
            vec![
                (COVER_LOCATION_ARRAY_INIT, vec![array, 0, 0]),
                (COVER_LOCATION_ARRAY_CLEAR, vec![array, 1])
            ]
        );
    }

    #[test]
    fn candidate_and_scrap_arrays_construct_and_destroy() {
        let mut e = engine();
        let array = e.mem.alloc(0x14);
        e.call_log = Some(vec![]);
        assert_eq!(fn_006db3b0(&mut e, Ptr::new(array)), Ptr::new(array));
        assert_eq!(e.mem.u32(array), CANDIDATE_ARRAY_VTABLE);
        assert_eq!(
            log_of(&mut e),
            vec![(CANDIDATE_ARRAY_INIT, vec![array, 0, 0])]
        );
        // The scrap-heap array takes the allocator it is given...
        e.call_log = Some(vec![]);
        fn_006db290(&mut e, Ptr::new(array), 0x20, 3, 0x7700);
        assert_eq!(e.mem.u32(array), SCRAP_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(array + 0x10), 0x7700);
        assert_eq!(
            log_of(&mut e),
            vec![
                (SCRAP_ARRAY_BASE_CONSTRUCT, vec![array]),
                (ARRAY_INIT, vec![array, 0x20, 3])
            ]
        );
        // ... or the thread's scrap heap.
        e.register(MEMORY_MANAGER_OBJECT, |_, _| ret(0x11f_6238));
        e.register_double(GET_THREAD_SCRAP_HEAP, |_, a| {
            assert_eq!(a[0], 0x11f_6238);
            ret(0x8800)
        });
        fn_006db290(&mut e, Ptr::new(array), 0x20, 0, 0);
        assert_eq!(e.mem.u32(array + 0x10), 0x8800);
        // The destructor clears with the buffer and runs the base destructor.
        e.call_log = Some(vec![]);
        fn_006db350(&mut e, Ptr::new(array));
        assert_eq!(
            log_of(&mut e),
            vec![
                (ARRAY_CLEAR, vec![array, 1]),
                (SCRAP_ARRAY_BASE_DESTRUCT, vec![array])
            ]
        );
    }

    #[test]
    fn cover_array_allocation_wrappers_scale_by_the_entry_size() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        fn_006db060(&mut e, Ptr::new(0x1000), 3);
        fn_006db090(&mut e, Ptr::new(0x1000), Ptr::new(0x2000));
        fn_006db0b0(&mut e, Ptr::new(0x1000), Ptr::new(0x2000), 5);
        assert_eq!(
            log_of(&mut e),
            vec![
                (MEMORY_ALLOCATE, vec![3 * 0x54]),
                (MEMORY_FREE, vec![0x2000]),
                (REALLOCATE_BLOCK, vec![0x2000, 5 * 0x54])
            ]
        );
    }

    /// An array object with `size`, `capacity` and a buffer.
    fn array_with(e: &mut Engine, size: u32, capacity: u32, buffer: u32) -> u32 {
        let array = e.mem.alloc(0x14);
        e.mem.set_u32(array + 4, buffer);
        e.mem.set_u32(array + 8, size);
        e.mem.set_u32(array + 0xc, capacity);
        array
    }

    #[test]
    fn reserving_cover_locations_cuts_the_size_and_reallocates() {
        let mut e = engine();
        let array = array_with(&mut e, 5, 8, 0x4000);
        e.call_log = Some(vec![]);
        // The same capacity: nothing happens.
        fn_006daeb0(&mut e, Ptr::new(array), 8);
        assert!(log_of(&mut e).is_empty());
        // A smaller capacity destroys the entries that no longer fit.
        e.call_log = Some(vec![]);
        fn_006daeb0(&mut e, Ptr::new(array), 3);
        assert_eq!(
            log_of(&mut e),
            vec![
                (
                    COVER_LOCATIONS_DESTROY_RANGE,
                    vec![array, 0x4000 + 3 * 0x54, 2]
                ),
                (COVER_LOCATIONS_REALLOCATE, vec![array, 3, 3])
            ]
        );
        assert_eq!((e.mem.u32(array + 8), e.mem.u32(array + 0xc)), (3, 3));
        // A larger capacity only reallocates.
        e.call_log = Some(vec![]);
        fn_006daeb0(&mut e, Ptr::new(array), 10);
        assert_eq!(
            log_of(&mut e),
            vec![(COVER_LOCATIONS_REALLOCATE, vec![array, 10, 3])]
        );
        assert_eq!(e.mem.u32(array + 0xc), 10);
    }

    #[test]
    fn reserving_path_entries_uses_the_0x14_byte_helpers() {
        let mut e = engine();
        let array = array_with(&mut e, 5, 8, 0x4000);
        e.call_log = Some(vec![]);
        fn_006db0d0(&mut e, Ptr::new(array), 2);
        assert_eq!(
            log_of(&mut e),
            vec![
                (
                    PATH_ELEMENTS_DESTROY_RANGE,
                    vec![array, 0x4000 + 2 * 0x14, 3]
                ),
                (PATH_ELEMENTS_REALLOCATE, vec![array, 2, 2])
            ]
        );
    }

    #[test]
    fn inserting_a_cover_location_shifts_the_later_ones() {
        let mut e = engine();
        let array = array_with(&mut e, 4, 8, 0x4000);
        // Inserting at the end appends.
        e.call_log = Some(vec![]);
        fn_006daf20(&mut e, Ptr::new(array), 4, Ptr::new(0x9000));
        assert_eq!(
            log_of(&mut e),
            vec![(COVER_LOCATIONS_ADD, vec![array, 0x9000])]
        );
        // In the middle with room: the tail moves up, the slot is built and
        // assigned.
        e.register(ARRAY_IS_FULL, |_, _| ret(0));
        e.call_log = Some(vec![]);
        fn_006daf20(&mut e, Ptr::new(array), 1, Ptr::new(0x9000));
        assert_eq!(
            log_of(&mut e),
            vec![
                (ARRAY_IS_FULL, vec![array]),
                (
                    COVER_LOCATIONS_COPY,
                    vec![array, 0x4000 + 2 * 0x54, 0x4000 + 0x54, 3]
                ),
                (COVER_LOCATIONS_CONSTRUCT, vec![array, 0x4000 + 0x54, 1]),
                (COVER_LOCATION_ASSIGN, vec![0x4000 + 0x54, 0x9000]),
            ]
        );
        assert_eq!(e.mem.u32(array + 8), 5);
    }

    #[test]
    fn inserting_into_a_full_cover_array_moves_to_a_bigger_buffer() {
        let mut e = engine();
        let array = array_with(&mut e, 4, 4, 0x4000);
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 4, 0x7777);
        e.mem.set_u32(array, vtable);
        e.register(ARRAY_IS_FULL, |_, _| ret(1));
        e.register(ARRAY_GROWN_CAPACITY, |_, _| ret(8));
        e.register_double(0x7777, |_, a| {
            assert_eq!(a[1], 8, "the allocation of the new capacity");
            ret(0x5000)
        });
        e.call_log = Some(vec![]);
        fn_006daf20(&mut e, Ptr::new(array), 1, Ptr::new(0x9000));
        let log = log_of(&mut e);
        // The head and the shifted tail go to the new buffer, around the new slot.
        assert_eq!(
            calls_to(&log, COVER_LOCATIONS_COPY),
            vec![
                vec![array, 0x5000, 0x4000, 1],
                vec![array, 0x5000 + 2 * 0x54, 0x4000 + 0x54, 3]
            ]
        );
        assert_eq!(
            calls_to(&log, COVER_LOCATIONS_CONSTRUCT),
            vec![vec![array, 0x5000 + 0x54, 1]]
        );
        assert_eq!(calls_to(&log, ARRAY_FREE_BUFFER).len(), 1);
        assert_eq!(
            (
                e.mem.u32(array + 4),
                e.mem.u32(array + 8),
                e.mem.u32(array + 0xc)
            ),
            (0x5000, 5, 8)
        );
        assert_eq!(
            calls_to(&log, COVER_LOCATION_ASSIGN),
            vec![vec![0x5000 + 0x54, 0x9000]]
        );
    }

    #[test]
    fn removing_path_entries_covers_the_whole_array_in_place_and_shrinking() {
        let mut e = engine();
        // Removing at least all of them empties the array.
        let array = array_with(&mut e, 3, 8, 0x4000);
        e.call_log = Some(vec![]);
        fn_006db140(&mut e, Ptr::new(array), 0, 3, true);
        assert_eq!(log_of(&mut e), vec![(ARRAY_CLEAR, vec![array, 1])]);
        // In place: destroy the range, move the rest down.
        let array = array_with(&mut e, 6, 8, 0x4000);
        e.call_log = Some(vec![]);
        fn_006db140(&mut e, Ptr::new(array), 1, 2, false);
        assert_eq!(
            log_of(&mut e),
            vec![
                (PATH_ELEMENTS_DESTROY_RANGE, vec![array, 0x4000 + 0x14, 2]),
                (
                    PATH_ELEMENTS_COPY,
                    vec![array, 0x4000 + 0x14, 0x4000 + 3 * 0x14, 3]
                )
            ]
        );
        assert_eq!(e.mem.u32(array + 8), 4);
        // Shrinking: into a smaller buffer obtained through the vtable.
        let array = array_with(&mut e, 6, 64, 0x4000);
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 4, 0x7777);
        e.mem.set_u32(array, vtable);
        e.register(ARRAY_MAY_SHRINK, |_, _| ret(1));
        e.register(ARRAY_SHRUNK_CAPACITY, |_, _| ret(32));
        e.register_double(0x7777, |_, a| {
            assert_eq!(a[1], 32);
            ret(0x5000)
        });
        e.call_log = Some(vec![]);
        fn_006db140(&mut e, Ptr::new(array), 1, 2, true);
        let log = log_of(&mut e);
        assert_eq!(
            calls_to(&log, PATH_ELEMENTS_COPY),
            vec![
                vec![array, 0x5000, 0x4000, 1],
                // the game counts size - count (4) entries, not size - index - count
                vec![array, 0x5000 + 0x14, 0x4000 + 3 * 0x14, 4]
            ]
        );
        assert_eq!(calls_to(&log, ARRAY_FREE_BUFFER).len(), 1);
        assert_eq!(
            (
                e.mem.u32(array + 4),
                e.mem.u32(array + 8),
                e.mem.u32(array + 0xc)
            ),
            (0x5000, 4, 32)
        );
        // Shrinking is only done when the array may shrink.
        let array = array_with(&mut e, 6, 64, 0x4000);
        e.register(ARRAY_MAY_SHRINK, |_, _| ret(0));
        e.call_log = Some(vec![]);
        fn_006db140(&mut e, Ptr::new(array), 0, 1, true);
        assert!(calls_to(&log_of(&mut e), ARRAY_SHRUNK_CAPACITY).is_empty());
    }

    /// An array object whose vtable slot 1 (+4, the allocation) is
    /// `function`.
    fn array_with_allocator(e: &mut Engine, vtable: u32, function: u32) -> u32 {
        e.put_vtable(vtable, &[0, function]);
        let array = e.mem.alloc(0x14);
        e.mem.set_u32(array, vtable);
        array
    }

    #[test]
    fn small_array_constructors_and_destructors_store_their_vtable() {
        let mut e = engine();
        let array = e.mem.alloc(0x14);
        let this = Ptr::new(array);
        e.call_log = Some(vec![]);
        fn_006db3e0(&mut e, this);
        assert_eq!(e.mem.u32(array), CANDIDATE_ARRAY_VTABLE);
        assert_eq!(log_of(&mut e), vec![(ARRAY_CLEAR, vec![array, 1])]);
        e.call_log = Some(vec![]);
        assert_eq!(fn_006db4a0(&mut e, this), this);
        assert_eq!(e.mem.u32(array), TRI_FAN_ARRAY_VTABLE);
        assert_eq!(
            log_of(&mut e),
            vec![(TRI_FAN_ARRAY_INIT, vec![array, 0, 0])]
        );
        e.mem.set_u32(array, 0);
        e.call_log = Some(vec![]);
        fn_006db4d0(&mut e, this);
        assert_eq!(e.mem.u32(array), TRI_FAN_ARRAY_VTABLE);
        assert_eq!(log_of(&mut e), vec![(ARRAY_CLEAR, vec![array, 1])]);
        e.call_log = Some(vec![]);
        assert_eq!(fn_006db9e0(&mut e, this), this);
        assert_eq!(e.mem.u32(array), TRI_HANDLE_ARRAY_VTABLE);
        assert_eq!(
            log_of(&mut e),
            vec![(CROSSED_TRIANGLE_ARRAY_INIT, vec![array, 0, 0])]
        );
        e.mem.set_u32(array, 0);
        e.call_log = Some(vec![]);
        fn_006dba10(&mut e, this);
        assert_eq!(e.mem.u32(array), TRI_HANDLE_ARRAY_VTABLE);
        assert_eq!(log_of(&mut e), vec![(ARRAY_CLEAR, vec![array, 1])]);
    }

    #[test]
    fn scrap_array_constructors_choose_the_allocator() {
        let mut e = engine();
        let array = e.mem.alloc(0x14);
        let this = Ptr::new(array);
        // The float scrap array takes the allocator it is given...
        e.call_log = Some(vec![]);
        assert_eq!(fn_006db7a0(&mut e, this, 0x20, 3, 0x7700), this);
        assert_eq!(e.mem.u32(array), FLOAT_SCRAP_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(array + 0x10), 0x7700);
        assert_eq!(
            log_of(&mut e),
            vec![
                (FLOAT_SCRAP_ARRAY_BASE_CONSTRUCT, vec![array]),
                (FLOAT_ARRAY_INIT, vec![array, 0x20, 3])
            ]
        );
        // ... or the thread's scrap heap.
        e.register(MEMORY_MANAGER_OBJECT, |_, _| ret(0x11f_6238));
        e.register_double(GET_THREAD_SCRAP_HEAP, |_, a| {
            assert_eq!(a[0], 0x11f_6238);
            ret(0x8800)
        });
        fn_006db7a0(&mut e, this, 8, 0, 0);
        assert_eq!(e.mem.u32(array + 0x10), 0x8800);
        // The crossed-triangle scrap array does the same with its own callees.
        e.call_log = Some(vec![]);
        assert_eq!(fn_006db890(&mut e, this, 0x10, 2, 0x7710), this);
        assert_eq!(e.mem.u32(array), CROSSED_TRIANGLE_SCRAP_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(array + 0x10), 0x7710);
        assert_eq!(
            log_of(&mut e),
            vec![
                (CROSSED_TRIANGLE_SCRAP_ARRAY_BASE_CONSTRUCT, vec![array]),
                (CROSSED_TRIANGLE_ARRAY_INIT, vec![array, 0x10, 2])
            ]
        );
        fn_006db890(&mut e, this, 0x10, 0, 0);
        assert_eq!(e.mem.u32(array + 0x10), 0x8800);
        // Its destructor clears the array, then runs the base destructor.
        e.mem.set_u32(array, 0);
        e.call_log = Some(vec![]);
        fn_006db950(&mut e, this);
        assert_eq!(e.mem.u32(array), CROSSED_TRIANGLE_SCRAP_ARRAY_VTABLE);
        assert_eq!(
            log_of(&mut e),
            vec![
                (ARRAY_CLEAR, vec![array, 1]),
                (CROSSED_TRIANGLE_SCRAP_ARRAY_BASE_DESTRUCT, vec![array])
            ]
        );
    }

    #[test]
    fn scalar_deleting_destructors_free_only_when_bit_zero_is_set() {
        type Destructor = fn(&mut Engine, Ptr, u32) -> Ptr;
        type Calls = Vec<(u32, Vec<u32>)>;
        let cases: Vec<(Destructor, Calls)> = vec![
            (
                fn_006dba80,
                vec![(COVER_LOCATION_ARRAY_CLEAR, vec![0x3000, 1])],
            ),
            (fn_006dbab0, vec![(SCRAP_ARRAY_BASE_DESTRUCT, vec![0x3000])]),
            (
                fn_006dbae0,
                vec![
                    (ARRAY_CLEAR, vec![0x3000, 1]),
                    (SCRAP_ARRAY_BASE_DESTRUCT, vec![0x3000]),
                ],
            ),
            (fn_006dbb10, vec![(ARRAY_CLEAR, vec![0x3000, 1])]),
            (fn_006dbb80, vec![(ARRAY_CLEAR, vec![0x3000, 1])]),
            (
                fn_006dbc00,
                vec![(FLOAT_SIMPLE_ARRAY_DESTRUCT, vec![0x3000])],
            ),
            (fn_006dbc30, vec![(KEY_ARRAY_DESTRUCT, vec![0x3000])]),
            (
                fn_006dbc60,
                vec![(CROSSED_TRIANGLE_SCRAP_ARRAY_BASE_DESTRUCT, vec![0x3000])],
            ),
            (
                fn_006dbc90,
                vec![
                    (ARRAY_CLEAR, vec![0x3000, 1]),
                    (CROSSED_TRIANGLE_SCRAP_ARRAY_BASE_DESTRUCT, vec![0x3000]),
                ],
            ),
            (fn_006dbcc0, vec![(ARRAY_CLEAR, vec![0x3000, 1])]),
        ];
        for (destructor, inner) in cases {
            let mut e = engine();
            e.map(0x3000, 0x100);
            // Without the flag only the destructor runs...
            e.call_log = Some(vec![]);
            assert_eq!(destructor(&mut e, Ptr::new(0x3000), 0), Ptr::new(0x3000));
            assert_eq!(log_of(&mut e), inner);
            // ... with it, the object is freed afterwards (other bits are ignored).
            e.call_log = Some(vec![]);
            assert_eq!(destructor(&mut e, Ptr::new(0x3000), 3), Ptr::new(0x3000));
            let mut with_free = inner;
            with_free.push((MEMORY_FREE, vec![0x3000]));
            assert_eq!(log_of(&mut e), with_free);
            e.call_log = Some(vec![]);
            destructor(&mut e, Ptr::new(0x3000), 2);
            assert!(calls_to(&log_of(&mut e), MEMORY_FREE).is_empty());
        }
    }

    #[test]
    fn appending_to_the_candidate_array_assigns_the_entry() {
        let mut e = engine();
        let buffer = e.mem.alloc(0x100);
        let array = array_with(&mut e, 2, 8, buffer);
        let item = e.mem.alloc(0x20);
        e.mem.set_u32(item + 0x18, 0x3f80_0000);
        e.mem.set_u32(item + 0x1c, 0xc000_0000);
        e.register(EDGE_PROXY_ARRAY_ADD_SLOT, |_, _| ret(2));
        e.call_log = Some(vec![]);
        assert_eq!(fn_006db400(&mut e, Ptr::new(array), Ptr::new(item)), 2);
        let entry = buffer + 2 * 0x20;
        assert_eq!(
            log_of(&mut e),
            vec![
                (EDGE_PROXY_ARRAY_ADD_SLOT, vec![array]),
                (EDGE_PROXY_ARRAY_CONSTRUCT, vec![array, entry, 1]),
                (PATH_POINT_ASSIGN, vec![entry, item])
            ]
        );
        assert_eq!(e.mem.u32(entry + 0x18), 0x3f80_0000);
        assert_eq!(e.mem.u32(entry + 0x1c), 0xc000_0000);
    }

    #[test]
    fn candidate_entry_assignment_copies_the_base_and_two_words() {
        let mut e = engine();
        let this = e.mem.alloc(0x20);
        let other = e.mem.alloc(0x20);
        e.mem.set_u32(other + 0x14, 0x1111);
        e.mem.set_u32(other + 0x18, 7);
        e.mem.set_u32(other + 0x1c, 9);
        e.call_log = Some(vec![]);
        assert_eq!(
            fn_006dbb40(&mut e, Ptr::new(this), Ptr::new(other)),
            Ptr::new(this)
        );
        assert_eq!(log_of(&mut e), vec![(PATH_POINT_ASSIGN, vec![this, other])]);
        // The base copy is the callee's; only +0x18 and +0x1C are copied here.
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert_eq!((e.mem.u32(this + 0x18), e.mem.u32(this + 0x1c)), (7, 9));
    }

    #[test]
    fn appending_to_the_path_point_array_assigns_the_entry() {
        let mut e = engine();
        let array = array_with(&mut e, 3, 8, 0x5000);
        e.register(PATH_POINT_ARRAY_ADD_SLOT, |_, _| ret(3));
        e.call_log = Some(vec![]);
        assert_eq!(fn_006db450(&mut e, Ptr::new(array), Ptr::new(0x9000)), 3);
        let entry = 0x5000 + 3 * 0x18;
        assert_eq!(
            log_of(&mut e),
            vec![
                (PATH_POINT_ARRAY_ADD_SLOT, vec![array]),
                (PATH_POINT_ENTRY_CONSTRUCT, vec![array, entry, 1]),
                (PATH_POINT_ASSIGN, vec![entry, 0x9000])
            ]
        );
    }

    #[test]
    fn appending_to_the_triangle_fan_array_assigns_the_entry() {
        let mut e = engine();
        let buffer = e.mem.alloc(0x100);
        let array = array_with(&mut e, 1, 8, buffer);
        let item = e.mem.alloc(0x10);
        for i in 0..4 {
            e.mem.set_u8(item + 8 + i, 1 + i as u8);
        }
        e.register(ARRAY_ADD_SLOT, |_, _| ret(1));
        e.call_log = Some(vec![]);
        assert_eq!(fn_006db4f0(&mut e, Ptr::new(array), Ptr::new(item)), 1);
        let entry = buffer + 0xc;
        assert_eq!(
            log_of(&mut e),
            vec![
                (ARRAY_ADD_SLOT, vec![array]),
                (TRI_FAN_ARRAY_CONSTRUCT, vec![array, entry, 1]),
                (TRIANGLE_REFERENCE_ASSIGN, vec![entry, item])
            ]
        );
        // Three bytes from +8, not the fourth.
        assert_eq!(
            (
                e.mem.u8(entry + 8),
                e.mem.u8(entry + 9),
                e.mem.u8(entry + 10),
                e.mem.u8(entry + 11)
            ),
            (1, 2, 3, 0)
        );
    }

    #[test]
    fn appending_to_the_cover_location_array_assigns_the_entry() {
        let mut e = engine();
        let array = array_with(&mut e, 4, 8, 0x6000);
        e.register(COVER_LOCATION_ARRAY_ADD_SLOT, |_, _| ret(4));
        e.call_log = Some(vec![]);
        assert_eq!(fn_006dbd90(&mut e, Ptr::new(array), Ptr::new(0x9100)), 4);
        let entry = 0x6000 + 4 * 0x54;
        assert_eq!(
            log_of(&mut e),
            vec![
                (COVER_LOCATION_ARRAY_ADD_SLOT, vec![array]),
                (COVER_LOCATIONS_CONSTRUCT, vec![array, entry, 1]),
                (COVER_LOCATION_ASSIGN, vec![entry, 0x9100])
            ]
        );
    }

    #[test]
    fn inserting_a_float_at_the_end_appends() {
        let mut e = engine();
        let buffer = e.mem.alloc(0x40);
        let array = array_with(&mut e, 3, 8, buffer);
        e.call_log = Some(vec![]);
        fn_006db540(&mut e, Ptr::new(array), 3, Ptr::new(0x9200));
        assert_eq!(
            log_of(&mut e),
            vec![(FLOAT_ARRAY_APPEND, vec![array, 0x9200])]
        );
        assert_eq!(e.mem.u32(array + 8), 3);
    }

    #[test]
    fn inserting_a_float_in_place_moves_the_tail_up() {
        let mut e = engine();
        let buffer = e.mem.alloc(0x40);
        let array = array_with(&mut e, 3, 8, buffer);
        let value = e.mem.alloc(8);
        e.mem.set_u32(value, 0x4048_0000);
        e.register(ARRAY_IS_FULL, |_, _| ret(0));
        e.call_log = Some(vec![]);
        fn_006db540(&mut e, Ptr::new(array), 1, Ptr::new(value));
        assert_eq!(
            log_of(&mut e),
            vec![
                (ARRAY_IS_FULL, vec![array]),
                (FLOAT_ARRAY_COPY, vec![array, buffer + 8, buffer + 4, 2]),
                (FLOAT_ARRAY_CONSTRUCT, vec![array, buffer + 4, 1])
            ]
        );
        assert_eq!(e.mem.u32(array + 8), 4);
        assert_eq!(e.mem.u32(array + 4), buffer);
        assert_eq!(e.mem.u32(buffer + 4), 0x4048_0000);
    }

    #[test]
    fn inserting_a_float_into_a_full_array_moves_to_a_bigger_buffer() {
        let mut e = engine();
        let old_buffer = e.mem.alloc(0x40);
        let new_buffer = e.mem.alloc(0x80);
        let array = array_with_allocator(&mut e, 0x00a7_0000, 0x00a7_1000);
        e.mem.set_u32(array + 4, old_buffer);
        e.mem.set_u32(array + 8, 3);
        e.mem.set_u32(array + 0xc, 3);
        let value = e.mem.alloc(8);
        e.mem.set_u32(value, 0x4048_0000);
        e.register(ARRAY_IS_FULL, |_, _| ret(1));
        e.register(ARRAY_GROWN_CAPACITY, |_, _| ret(6));
        e.register_double(0x00a7_1000, move |_, a| {
            assert_eq!(a[1], 6);
            ret(new_buffer)
        });
        e.call_log = Some(vec![]);
        fn_006db540(&mut e, Ptr::new(array), 1, Ptr::new(value));
        let log = log_of(&mut e);
        assert_eq!(
            calls_to(&log, FLOAT_ARRAY_COPY),
            vec![
                vec![array, new_buffer, old_buffer, 1],
                vec![array, new_buffer + 8, old_buffer + 4, 2]
            ]
        );
        assert_eq!(
            calls_to(&log, FLOAT_ARRAY_CONSTRUCT),
            vec![vec![array, new_buffer + 4, 1]]
        );
        assert_eq!(calls_to(&log, ARRAY_FREE_BUFFER), vec![vec![array]]);
        assert_eq!(
            (
                e.mem.u32(array + 4),
                e.mem.u32(array + 8),
                e.mem.u32(array + 0xc)
            ),
            (new_buffer, 4, 6)
        );
        assert_eq!(e.mem.u32(new_buffer + 4), 0x4048_0000);
    }

    #[test]
    fn triangle_fan_entry_assignment_copies_exactly_three_bytes() {
        let mut e = engine();
        let this = e.mem.alloc(0x10);
        let other = e.mem.alloc(0x10);
        for i in 0..4 {
            e.mem.set_u8(other + 8 + i, 0x10 + i as u8);
        }
        assert_eq!(
            fn_006dbbb0(&mut e, Ptr::new(this), Ptr::new(other)),
            Ptr::new(this)
        );
        assert_eq!(e.mem.u8(this + 10), 0x12);
        assert_eq!(e.mem.u8(this + 11), 0);
    }

    #[test]
    fn cover_location_array_initialization_allocates_and_constructs() {
        let mut e = engine();
        let buffer = e.mem.alloc(0x100);
        let array = array_with_allocator(&mut e, 0x00a7_2000, 0x00a7_3000);
        e.register_double(0x00a7_3000, move |_, a| {
            assert_eq!(a[1], 5);
            ret(buffer)
        });
        // The capacity is raised to the size.
        e.call_log = Some(vec![]);
        fn_006dc070(&mut e, Ptr::new(array), 2, 5);
        assert_eq!(
            log_of(&mut e),
            vec![
                (0x00a7_3000, vec![array, 5]),
                (COVER_LOCATIONS_CONSTRUCT, vec![array, buffer, 5])
            ]
        );
        assert_eq!(
            (
                e.mem.u32(array + 4),
                e.mem.u32(array + 8),
                e.mem.u32(array + 0xc)
            ),
            (buffer, 5, 5)
        );
        // Nothing requested: nothing allocated, nothing constructed.
        e.call_log = Some(vec![]);
        fn_006dc070(&mut e, Ptr::new(array), 0, 0);
        assert!(log_of(&mut e).is_empty());
        assert_eq!(
            (
                e.mem.u32(array + 4),
                e.mem.u32(array + 8),
                e.mem.u32(array + 0xc)
            ),
            (0, 0, 0)
        );
        // Room without entries: only the allocation.
        e.call_log = Some(vec![]);
        fn_006dc070(&mut e, Ptr::new(array), 5, 0);
        assert_eq!(log_of(&mut e).len(), 1);
        assert_eq!(e.mem.u32(array + 0xc), 5);
    }

    #[test]
    fn funcs_cover_every_translated_function() {
        assert_eq!(funcs().len(), 146);
    }
}
