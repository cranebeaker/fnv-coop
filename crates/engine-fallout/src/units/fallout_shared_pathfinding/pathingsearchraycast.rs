//! `fallout shared/pathfinding/pathingsearchraycast.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `PathingSearchRayCast` (Xbox PDB) finds a straight path for a
//! `RayCastPathRequest` with Havok ray casts: from the goal to the first
//! reachable navmesh edge (`RCRT_START` 0, `RCRT_GOAL` 1, the direction set
//! by the type), or, for an actor without a navmesh (`RCRT_NOMESH` 2),
//! between start and goal directly (in steps of the actor radius, or as one
//! cast under water). The result goes into a `RayCastPath`
//! (`RCP_NONE` 0, `RCP_SUCCESS` 1, `RCP_FAILED` 2).
//!
//! How the translations read the game code:
//! - Locals the game keeps on its stack and passes by address are taken
//!   from one block per function, addressed by their `EBP`-relative offset
//!   (`local(0x30)` is `EBP-0x30`).
//! - Floats copied with `FLD`/`FSTP` keep their bit pattern except that a
//!   signalling NaN is quieted (`x87_quiet_bits`); floats passed on the
//!   stack are passed as their bits.
//! - The compiler's exception-unwinding frames (`FS:[0]`) and the stack
//!   cookie check (`00ec408c`) are not translated; the functions that have
//!   one are marked incomplete.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{BSSimpleArray, NiPoint3};
use crate::units::fallout_shared_pathfinding::pathfind::PathingLocation;

layout! {
    /// `PathingSearchRayCast` (Xbox PDB), 0x94 bytes; the PC offsets match.
    pub struct PathingSearchRayCast: 0x94 {
        /// `StartLocation` (Xbox PDB).
        0x04 StartLocation: Inline<PathingLocation>,
        /// `GoalLocation` (Xbox PDB).
        0x2C GoalLocation: Inline<PathingLocation>,
        /// `pRequest` (Xbox PDB): the request's `pInitialPathRequest`.
        0x54 pRequest: Ptr,
        /// `eType` (Xbox PDB): `RAYCAST_REQUEST_TYPE`.
        0x58 eType: u32,
        /// `BoundsMin` (Xbox PDB).
        0x5C BoundsMin: Inline<NiPoint3>,
        /// `BoundsMax` (Xbox PDB).
        0x68 BoundsMax: Inline<NiPoint3>,
        /// `fActorRadius` (Xbox PDB).
        0x74 fActorRadius: f32,
        /// `pCache` (Xbox PDB).
        0x78 pCache: Ptr,
        /// `pHavokWorld` (Xbox PDB): `bhkWorld*`.
        0x7C pHavokWorld: Ptr,
        /// `pPickData` (Xbox PDB): `bhkPickData*` (0xB0 bytes, 16-aligned).
        0x80 pPickData: Ptr,
        /// `pCollector` (Xbox PDB): `hkpClosestRayHitCollector*` (0x70
        /// bytes, 16-aligned).
        0x84 pCollector: Ptr,
        /// `iRayCastCount` (Xbox PDB).
        0x88 iRayCastCount: i32,
        /// `fMaxDrop` (Xbox PDB).
        0x8C fMaxDrop: f32,
        /// `fMaxStep` (Xbox PDB).
        0x90 fMaxStep: f32,
    }

    /// `RayCastPathRequest` (Xbox PDB), 0x68 bytes.
    pub struct RayCastPathRequest: 0x68 {
        /// `eType` (Xbox PDB): `RAYCAST_REQUEST_TYPE`.
        0x00 eType: u32,
        /// `pInitialPathRequest` (Xbox PDB): `PathingRequest*`.
        0x04 pInitialPathRequest: Ptr,
        /// `StartLocation` (Xbox PDB).
        0x08 StartLocation: Inline<PathingLocation>,
        /// `GoalLocation` (Xbox PDB).
        0x30 GoalLocation: Inline<PathingLocation>,
        /// `Edges` (Xbox PDB): `BSSimpleArray<NavMeshEdgeLocation,1024>`
        /// (0x18-byte elements, the point at +0x0C).
        0x58 Edges: Inline<BSSimpleArray>,
    }

    /// `RayCastPath` (Xbox PDB), 0x18 bytes.
    pub struct RayCastPath: 0x18 {
        /// `eType` (Xbox PDB): `RAYCAST_REQUEST_TYPE`.
        0x00 eType: u32,
        /// `eResult` (Xbox PDB): `RAYCASTPATH_RESULT`.
        0x04 eResult: u32,
        /// `Nodes` (Xbox PDB): `BSSimpleArray<PathingLocation,1024>`.
        0x08 Nodes: Inline<BSSimpleArray>,
    }
}

/// `RCRT_START` (Xbox PDB `RAYCAST_REQUEST_TYPE`).
const RCRT_START: u32 = 0;
/// `RCRT_GOAL`.
const RCRT_GOAL: u32 = 1;
/// `RCRT_NOMESH`.
const RCRT_NOMESH: u32 = 2;
/// `RCP_NONE` (Xbox PDB `RAYCASTPATH_RESULT`).
const RCP_NONE: u32 = 0;
/// `RCP_SUCCESS`.
const RCP_SUCCESS: u32 = 1;
/// `RCP_FAILED`.
const RCP_FAILED: u32 = 2;

/// `PathingSearchRayCast`'s vtable (slot 0 is `006e6280`).
const VTABLE: u32 = 0x0106_d47c;
/// `PathingLocation::PathingLocation` (`006dcbb0`, thiscall).
const LOCATION_CONSTRUCT: u32 = 0x006d_cbb0;
/// `004ff7e0` (thiscall): the `PathingLocation` destructor.
const LOCATION_DESTROY: u32 = 0x004f_f7e0;
/// `PathingLocation::operator=` (`006dcce0`, thiscall `(this, other)`,
/// `RET 4`).
const LOCATION_ASSIGN: u32 = 0x006d_cce0;
/// `006815c0` (thiscall, no stack words): the `NiPoint3` constructor.
const POINT_CONSTRUCT: u32 = 0x0068_15c0;
/// `00403e20` (thiscall): the address of a `Setting`'s value (`this + 4`).
const SETTING_VALUE: u32 = 0x0040_3e20;
/// The `Setting` whose value becomes `fMaxDrop`.
const MAX_DROP_SETTING: u32 = 0x011c_ff94;
/// `32.0` (`float`): `fMaxStep`, and the smallest actor radius.
const MAX_STEP: u32 = 0x0101_e340;
/// `operator delete` (`00401030`, cdecl `(block)`).
const OPERATOR_DELETE: u32 = 0x0040_1030;

/// `0069ef80` (thiscall): the request's `fActorRadius` (+0x68), in `ST0`.
const REQUEST_ACTOR_RADIUS: u32 = 0x0069_ef80;
/// `00404010` (cdecl `(float a, float b)`): the larger, `b` unless `a > b`,
/// in `ST0`.
const FLOAT_MAX: u32 = 0x0040_4010;
/// `0040ebd0` (cdecl `(float a, float b)`): the smaller, `b` unless `b > a`,
/// in `ST0`.
const FLOAT_MIN: u32 = 0x0040_ebd0;
/// `00408840` (cdecl `(float)`): the absolute value, in `ST0`.
const FLOAT_ABS: u32 = 0x0040_8840;
/// `005c3420` (thiscall `(location, result)`, `RET 4`): copies the
/// location's point (+4) into `result` and returns it.
const LOCATION_POSITION: u32 = 0x005c_3420;
/// `00439ef0` (thiscall `(a, result, b)`, `RET 8`): `*result = a - b`.
const POINT_SUBTRACT: u32 = 0x0043_9ef0;
/// `00439e90` (thiscall `(a, result, b)`, `RET 8`): `*result = a + b`.
const POINT_ADD: u32 = 0x0043_9e90;
/// `0045bb20` (thiscall `(a, result, float s)`, `RET 8`): `*result = a * s`.
const POINT_SCALE: u32 = 0x0045_bb20;
/// `0063c8a0` (thiscall `(a, b)`, `RET 4`): `*a += *b`.
const POINT_ADD_ASSIGN: u32 = 0x0063_c8a0;
/// `00416870` (thiscall `(point, x, y, z)`, `RET 0xC`): sets the point.
const POINT_SET: u32 = 0x0041_6870;
/// `NiPoint3::UnitizeGetLength` (`00457910`, thiscall): the length in
/// `ST0`.
const UNITIZE: u32 = 0x0045_7910;
/// `0.5` (`float`).
const HALF_FLOAT: u32 = 0x0101_6248;
/// `0.5` (`double`).
const HALF_DOUBLE: u32 = 0x0101_1588;
/// `0.1` (`float`): how far an edge point is moved toward its triangle's
/// center.
const TENTH: u32 = 0x0101_e2bc;
/// `64.0` (`double`): the largest height difference the last cast of
/// [`fn_006e6e40`] accepts.
const MAX_HEIGHT_DIFFERENCE: u32 = 0x0102_40c0;
/// `_ftol2_sse` (`00ec62c0`): the value in `ST0` (an `f64` argument)
/// truncated to an integer.
const FTOL: u32 = 0x00ec_62c0;

/// `PathingLocation::GetCell` (`006dd4f0`, thiscall).
const GET_CELL: u32 = 0x006d_d4f0;
/// `TESObjectCELL::GetWaterHeight` (`005471e0`, thiscall): in `ST0`.
const WATER_HEIGHT: u32 = 0x0054_71e0;
/// `0048d150` (thiscall): `this + 0x0C`, the request's `Origin`.
const REQUEST_ORIGIN: u32 = 0x0048_d150;
/// `006cf1b0` (thiscall `(array, location)`, `RET 4`): appends a copy of
/// the location to a `BSSimpleArray<PathingLocation>`.
const NODES_ADD: u32 = 0x006c_f1b0;
/// `006cedd0` (thiscall): the address of the last 0x28-byte element.
const NODES_LAST: u32 = 0x006c_edd0;
/// `PathingLocation::ResolveNavMeshInfo` (`006dd6f0`, thiscall
/// `(location, filter)`, `RET 4`).
const RESOLVE_NAV_MESH_INFO: u32 = 0x006d_d6f0;
/// `0076b610` (thiscall): whether the array is empty (`iSize == 0`).
const IS_EMPTY: u32 = 0x0076_b610;
/// `006cedf0` (thiscall): the address of the first 0x18-byte element.
const FIRST_EDGE: u32 = 0x006c_edf0;
/// `006cee10` (thiscall `(array, index)`, `RET 4`): the address of a
/// 0x18-byte element.
const EDGE_AT: u32 = 0x006c_ee10;
/// `008454f0` (thiscall `(array, word)`, `RET 4`).
const ARRAY_008454F0: u32 = 0x0084_54f0;
/// `006cb900` (thiscall `(this, other)`, `RET 4`): the
/// `NavMeshEdgeLocation` copy constructor.
const EDGE_COPY: u32 = 0x006c_b900;
/// `0042fb00` (thiscall): the `NavMeshPtr` constructor.
const NAV_MESH_PTR_CONSTRUCT: u32 = 0x0042_fb00;
/// `0042fa40` (thiscall): the `NavMeshPtr` destructor.
const NAV_MESH_PTR_DESTROY: u32 = 0x0042_fa40;
/// `0069ad00` (thiscall `(navmesh info, NavMeshPtr*)`, `RET 4`, byte
/// result): gets the info's navmesh.
const GET_NAV_MESH: u32 = 0x0069_ad00;
/// `00559450` (thiscall): the `NavMesh*` a `NavMeshPtr` holds.
const NAV_MESH_PTR_GET: u32 = 0x0055_9450;
/// `NavMesh::GetCenter` (`00558220`, thiscall `(mesh, result, triangle)`,
/// `RET 8`).
const TRIANGLE_CENTER: u32 = 0x0055_8220;
/// `006dd010` (thiscall `(location, point, navmesh info, triangle)`,
/// `RET 0xC`): a `PathingLocation` constructor.
const LOCATION_FROM_POINT: u32 = 0x006d_d010;
/// `006e6f90` (thiscall `(search, from, to, result)`, `RET 0xC`, byte
/// result): one ray cast step (another unit).
const RAY_CAST_STEP: u32 = 0x006e_6f90;

/// `004543c0` (thiscall): the cell's Havok world.
const CELL_HAVOK_WORLD: u32 = 0x0045_43c0;
/// `008c71b0` (thiscall `(this, word)`, `RET 4`): stores one word.
const FILTER_INFO_CONSTRUCT: u32 = 0x008c_71b0;
/// `004a39f0` (thiscall `(this, word)`, `RET 4`), run with 0x26.
const FILTER_INFO_004A39F0: u32 = 0x004a_39f0;
/// `0059ce80` (thiscall `(this, word)`, `RET 4`), run with 3.
const FILTER_INFO_0059CE80: u32 = 0x0059_ce80;
/// `00553320` (cdecl `(alignment, size)`): an aligned allocation.
const ALIGNED_ALLOCATE: u32 = 0x0055_3320;
/// `00553380` (cdecl `(block)`): frees an aligned allocation.
const ALIGNED_FREE: u32 = 0x0055_3380;
/// `006e6da0(size, address)` (cdecl): placement allocation.
const PLACEMENT: u32 = 0x006e_6da0;
/// `004a3c20` (thiscall): the 0xB0-byte `bhkPickData` constructor.
const PICK_DATA_CONSTRUCT: u32 = 0x004a_3c20;
/// `004a3f70` (thiscall `(pick data, word)`, `RET 4`).
const PICK_DATA_004A3F70: u32 = 0x004a_3f70;
/// `00500a20` (thiscall `(pick data, cache)`, `RET 4`).
const PICK_DATA_00500A20: u32 = 0x0050_0a20;
/// `0062a220` (thiscall): the 0x70-byte collector constructor.
const COLLECTOR_CONSTRUCT: u32 = 0x0062_a220;
/// `0059ceb0` (thiscall `(pick data, collector)`, `RET 4`).
const PICK_DATA_0059CEB0: u32 = 0x0059_ceb0;
/// `00437c10` (thiscall): constructs the two points of an AABB.
const AABB_CONSTRUCT: u32 = 0x0043_7c10;
/// `004a3e00` (cdecl `(result, point)`): a `NiPoint3` into a 16-byte
/// Havok vector.
const TO_HAVOK_VECTOR: u32 = 0x004a_3e00;
/// `005533c0` (thiscall), run on the world before and after the cache is
/// built.
const WORLD_005533C0: u32 = 0x0055_33c0;
/// `00541c80` (thiscall): a pointer from the `bhkWorld` (0 for a null one).
const WORLD_00541C80: u32 = 0x0054_1c80;
/// `006286d0` (thiscall): the word at +0x58 (folded; named
/// `LowProcess::GetNumberOfItemsActivated` in the engine map).
const WORD_AT_58: u32 = 0x0062_86d0;
/// `006e7620` (cdecl `(size, word)`): allocates the cache.
const CACHE_CREATE: u32 = 0x006e_7620;
/// `005e0b60` (cdecl `(cache)`): frees the cache.
const CACHE_FREE: u32 = 0x005e_0b60;
/// `0062a2b0` (thiscall), run on the pick data + 0x30.
const PICK_RESULT_0062A2B0: u32 = 0x0062_a2b0;
/// `0062a290` (thiscall), run on the collector.
const COLLECTOR_0062A290: u32 = 0x0062_a290;
/// `004a3da0` (thiscall `(pick data, point)`, `RET 4`): the cast's start.
const PICK_DATA_SET_FROM: u32 = 0x004a_3da0;
/// `004a3eb0` (thiscall `(pick data, point)`, `RET 4`): the cast's end.
const PICK_DATA_SET_TO: u32 = 0x004a_3eb0;
/// `006f3170` (thiscall, byte result): whether the array should shrink
/// (`iSize` at most a quarter of a capacity up to 1024, else at most the
/// capacity - 2048).
const SHOULD_SHRINK: u32 = 0x006f_3170;
/// `00869600` (thiscall): the shrunk capacity.
const SHRUNK_CAPACITY: u32 = 0x0086_9600;
/// `006d0730` (thiscall `(array, dest, src, count)`, `RET 0xC`): moves
/// `count` 0x18-byte elements.
const EDGES_MOVE: u32 = 0x006d_0730;
/// `0072ba80` (thiscall `(array, element, count)`, `RET 8`): destroys
/// elements.
const EDGES_DESTROY: u32 = 0x0072_ba80;
/// `006a8500` (thiscall): frees the array's buffer.
const FREE_BUFFER: u32 = 0x006a_8500;

/// `0044ddc0` (thiscall): the array's `iSize`.
const ARRAY_SIZE: u32 = 0x0044_ddc0;
/// Size of a `NavMeshEdgeLocation` (an `Edges` element).
const EDGE_SIZE: u32 = 0x18;
/// Offset of a `NavMeshEdgeLocation`'s `Location` point.
const EDGE_LOCATION: u32 = 0x0c;

/// The local at `EBP - off` of a frame block of `size` bytes.
fn local(frame: Ptr, size: u32, off: u32) -> u32 {
    frame.addr() + size - off
}

fn copy_point(e: &mut Engine, to: u32, from: u32) {
    for k in 0..3 {
        let word = e.mem.u32(from + 4 * k);
        e.mem.set_u32(to + 4 * k, word);
    }
}

/// An `FLD dword` / `FSTP dword` copy.
fn copy_float(e: &mut Engine, to: u32, from: u32) {
    let bits = x87_quiet_bits(e.mem.u32(from));
    e.mem.set_u32(to, bits);
}

/// A `float` pushed with `FLD dword` / `FSTP dword [ESP]`.
fn float_arg(e: &Engine, at: u32) -> u32 {
    x87_quiet_bits(e.mem.u32(at))
}

fn position(e: &mut Engine, location: u32, result: u32) -> u32 {
    e.call(LOCATION_POSITION, &args![location, result]).u32()
}

// Translated from 006e61a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingSearchRayCast::PathingSearchRayCast` (Xbox PDB): stores the
/// vtable, constructs both locations and bounds, clears the Havok pointers
/// and the cast count (`pCache` is left as it is), takes `fMaxDrop` from the
/// `Setting` at `011cff94` and `fMaxStep` = 32.0. Returns `this`.
// Translation incomplete: 006e61a0 - FS:[0] exception frame (unwinding of the locations) is not translated.
pub fn pathing_search_ray_cast_pathing_search_ray_cast(
    e: &mut Engine,
    this: Ptr<PathingSearchRayCast>,
) -> Ptr<PathingSearchRayCast> {
    e.mem.set_u32(this.addr(), VTABLE);
    e.call(
        LOCATION_CONSTRUCT,
        &args![this.at(PathingSearchRayCast::StartLocation)],
    );
    e.call(
        LOCATION_CONSTRUCT,
        &args![this.at(PathingSearchRayCast::GoalLocation)],
    );
    e.call(
        POINT_CONSTRUCT,
        &args![this.at(PathingSearchRayCast::BoundsMin)],
    );
    e.call(
        POINT_CONSTRUCT,
        &args![this.at(PathingSearchRayCast::BoundsMax)],
    );
    e.set(this, PathingSearchRayCast::pHavokWorld, Ptr::NULL);
    e.set(this, PathingSearchRayCast::pCollector, Ptr::NULL);
    e.set(this, PathingSearchRayCast::pPickData, Ptr::NULL);
    e.set(this, PathingSearchRayCast::iRayCastCount, 0);
    let value = e.call(SETTING_VALUE, &args![MAX_DROP_SETTING]).u32();
    copy_float(e, this.addr() + PathingSearchRayCast::fMaxDrop.off, value);
    copy_float(
        e,
        this.addr() + PathingSearchRayCast::fMaxStep.off,
        MAX_STEP,
    );
    this
}

// Translated from 006e6280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingSearchRayCast::`scalar deleting destructor'` (Xbox PDB): the
/// destructor ([`fn_006e62b0`]), then `operator delete` when bit 0 of
/// `flags` is set. Returns `this`. `RET 4`.
pub fn pathing_search_ray_cast_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<PathingSearchRayCast>,
    flags: u32,
) -> Ptr<PathingSearchRayCast> {
    fn_006e62b0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 006e62b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; by its body `PathingSearchRayCast`'s
/// destructor: stores the vtable and destroys the goal, then the start
/// location.
// Translation incomplete: 006e62b0 - FS:[0] exception frame (unwinding of the locations) is not translated.
pub fn fn_006e62b0(e: &mut Engine, this: Ptr<PathingSearchRayCast>) {
    e.mem.set_u32(this.addr(), VTABLE);
    e.call(
        LOCATION_DESTROY,
        &args![this.at(PathingSearchRayCast::GoalLocation)],
    );
    e.call(
        LOCATION_DESTROY,
        &args![this.at(PathingSearchRayCast::StartLocation)],
    );
}

/// Records the outcome of a no-mesh cast: on success the request's start
/// and goal become the path's nodes.
fn record_no_mesh_result(
    e: &mut Engine,
    request: Ptr<RayCastPathRequest>,
    path: Ptr<RayCastPath>,
    found: bool,
) {
    if found {
        let nodes = path.at(RayCastPath::Nodes);
        e.call(
            NODES_ADD,
            &args![nodes, request.at(RayCastPathRequest::StartLocation)],
        );
        e.call(
            NODES_ADD,
            &args![nodes, request.at(RayCastPathRequest::GoalLocation)],
        );
        e.set(path, RayCastPath::eResult, RCP_SUCCESS);
    } else {
        e.set(path, RayCastPath::eResult, RCP_FAILED);
    }
}

/// `BuildPath`'s `RCRT_NOMESH` case (`006e643a` to `006e684a`).
fn build_no_mesh_path(
    e: &mut Engine,
    this: Ptr<PathingSearchRayCast>,
    request: Ptr<RayCastPathRequest>,
    path: Ptr<RayCastPath>,
    frame: Ptr,
) -> u32 {
    let l = |off: u32| local(frame, BUILD_PATH_FRAME, off);
    let start = this.at(PathingSearchRayCast::StartLocation).addr();
    let goal = this.at(PathingSearchRayCast::GoalLocation).addr();
    e.call(
        LOCATION_ASSIGN,
        &args![goal, request.at(RayCastPathRequest::GoalLocation)],
    );
    e.call(
        LOCATION_ASSIGN,
        &args![start, request.at(RayCastPathRequest::StartLocation)],
    );

    // The larger horizontal extent of goal - start.
    let from = position(e, start, l(0xa8));
    let to = position(e, goal, l(0x9c));
    e.call(POINT_SUBTRACT, &args![to, l(0x30), from]);
    let dy = float_arg(e, l(0x30) + 4);
    let abs_y = e.call(FLOAT_ABS, &args![dy]).f32();
    let dx = float_arg(e, l(0x30));
    let abs_x = e.call(FLOAT_ABS, &args![dx]).f32();
    let extent = e
        .call(FLOAT_MAX, &args![abs_x.to_bits(), abs_y.to_bits()])
        .f32();
    e.mem.set_f32(l(0x24), extent);

    // The lower and the higher of the two heights.
    let to = position(e, goal, l(0xb4));
    let goal_z = float_arg(e, to + 8);
    let from = position(e, start, l(0xc0));
    let start_z = float_arg(e, from + 8);
    let low = e.call(FLOAT_MIN, &args![start_z, goal_z]).f32();
    e.mem.set_f32(l(0x1c), low);
    let to = position(e, goal, l(0xcc));
    let goal_z = float_arg(e, to + 8);
    let from = position(e, start, l(0xd8));
    let start_z = float_arg(e, from + 8);
    let high = e.call(FLOAT_MAX, &args![start_z, goal_z]).f32();
    e.mem.set_f32(l(0x20), high);

    // The midpoint, and the bounds: a square of the extent around it.
    let half = float_arg(e, HALF_FLOAT);
    let from = position(e, start, l(0xf0));
    let to = position(e, goal, l(0xe4));
    let sum = e.call(POINT_ADD, &args![to, l(0xfc), from]).u32();
    e.call(POINT_SCALE, &args![sum, l(0x18), half]);
    let half_double: f64 = e.global(HALF_DOUBLE);
    let extent = e.mem.f32(l(0x24));
    let half_extent = (extent as f64 * half_double) as f32;
    let offset = e
        .call(
            POINT_SET,
            &args![l(0x108), half_extent.to_bits(), half_extent.to_bits(), 0u32],
        )
        .u32();
    let min = e
        .call(POINT_SUBTRACT, &args![l(0x18), l(0x114), offset])
        .u32();
    copy_point(e, this.at(PathingSearchRayCast::BoundsMin).addr(), min);
    let offset = e
        .call(
            POINT_SET,
            &args![l(0x120), half_extent.to_bits(), half_extent.to_bits(), 0u32],
        )
        .u32();
    let max = e.call(POINT_ADD, &args![l(0x18), l(0x12c), offset]).u32();
    copy_point(e, this.at(PathingSearchRayCast::BoundsMax).addr(), max);
    copy_float(e, this.addr() + 0x64, l(0x1c));
    copy_float(e, this.addr() + 0x70, l(0x20));

    if !pathing_search_ray_cast_init_ray_casts(e, this) {
        e.set(path, RayCastPath::eResult, RCP_FAILED);
        return RCP_FAILED;
    }

    // Both ends under water: one cast between them.
    let mut under_water = false;
    if e.call(GET_CELL, &args![start]).u32() != 0 && e.call(GET_CELL, &args![goal]).u32() != 0 {
        let from = position(e, start, l(0x138));
        let cell = e.call(GET_CELL, &args![start]).u32();
        let water = e.call(WATER_HEIGHT, &args![cell]).f64();
        if (e.mem.f32(from + 8) as f64) < water {
            let to = position(e, goal, l(0x144));
            let cell = e.call(GET_CELL, &args![goal]).u32();
            let water = e.call(WATER_HEIGHT, &args![cell]).f64();
            under_water = (e.mem.f32(to + 8) as f64) < water;
        }
    }
    let found = if under_water {
        let to = position(e, goal, l(0x150));
        let from = position(e, start, l(0x15c));
        fn_006e7190(e, this, Ptr::new(from), Ptr::new(to))
    } else {
        let to = position(
            e,
            request.at(RayCastPathRequest::GoalLocation).addr(),
            l(0x168),
        );
        let from = position(
            e,
            request.at(RayCastPathRequest::StartLocation).addr(),
            l(0x174),
        );
        fn_006e6e40(e, this, Ptr::new(from), Ptr::new(to))
    };
    record_no_mesh_result(e, request, path, found);
    fn_006e6db0(e, this);
    e.get(path, RayCastPath::eResult)
}

/// `BuildPath`'s success case for an edge (`006e6928` to `006e6a69`):
/// the point of the first edge, moved a tenth of the way toward its
/// triangle's center, becomes the second node after the goal.
fn add_edge_nodes(
    e: &mut Engine,
    this: Ptr<PathingSearchRayCast>,
    edges: u32,
    path: Ptr<RayCastPath>,
    frame: Ptr,
) {
    let l = |off: u32| local(frame, BUILD_PATH_FRAME, off);
    let first = e.call(FIRST_EDGE, &args![edges]).u32();
    e.call(EDGE_COPY, &args![l(0x50), first]);
    e.call(NAV_MESH_PTR_CONSTRUCT, &args![l(0x38)]);
    let info = e.mem.u32(l(0x50));
    if !e.call(GET_NAV_MESH, &args![info, l(0x38)]).bool() {
        e.call(ARRAY_008454F0, &args![edges, 1u32]);
        e.set(path, RayCastPath::eResult, RCP_FAILED);
        e.call(NAV_MESH_PTR_DESTROY, &args![l(0x38)]);
        return;
    }
    copy_point(e, l(0x68), l(0x50) + EDGE_LOCATION);
    let triangle = e.mem.u16(l(0x4c)) as u32;
    let mesh = e.call(NAV_MESH_PTR_GET, &args![l(0x38)]).u32();
    e.call(TRIANGLE_CENTER, &args![mesh, l(0x5c), triangle]);
    let tenth = float_arg(e, TENTH);
    let toward = e
        .call(POINT_SUBTRACT, &args![l(0x5c), l(0x198), l(0x68)])
        .u32();
    let step = e.call(POINT_SCALE, &args![toward, l(0x1a4), tenth]).u32();
    e.call(POINT_ADD_ASSIGN, &args![l(0x68), step]);
    let triangle = e.mem.u16(l(0x4c)) as u32;
    let info = e.mem.u32(l(0x50));
    e.call(
        LOCATION_FROM_POINT,
        &args![l(0x90), l(0x68), info, triangle],
    );
    let nodes = path.at(RayCastPath::Nodes);
    e.call(
        NODES_ADD,
        &args![nodes, this.at(PathingSearchRayCast::GoalLocation)],
    );
    e.call(NODES_ADD, &args![nodes, l(0x90)]);
    let last = e.call(NODES_LAST, &args![nodes]).u32();
    e.call(RESOLVE_NAV_MESH_INFO, &args![last, 0u32]);
    e.set(path, RayCastPath::eResult, RCP_SUCCESS);
    e.call(LOCATION_DESTROY, &args![l(0x90)]);
    e.call(NAV_MESH_PTR_DESTROY, &args![l(0x38)]);
}

/// Bytes of `BuildPath`'s frame below `EBP`.
const BUILD_PATH_FRAME: u32 = 0x1b0;

// Translated from 006e6320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingSearchRayCast::BuildPath` (Xbox PDB): takes the request's type
/// and initial pathing request, sets the path's type, the actor radius
/// (the request's, at least 32.0) and clears the cast count.
///
/// `RCRT_NOMESH`: copies start and goal, sets the bounds to a square of the
/// larger horizontal extent around the midpoint between the lower and the
/// higher end; without ray casts (`InitRayCasts`) the result is
/// `RCP_FAILED` (2). When both ends are in cells and below their water
/// height one cast ([`fn_006e7190`]) decides, else the stepped casts
/// ([`fn_006e6e40`]) between the request's start and goal; on success both
/// become the path's nodes (`RCP_SUCCESS`), else `RCP_FAILED`.
///
/// Other types (`RCRT_GOAL` and `RCRT_START` also copy the goal and set the
/// bounds from the edges, [`fn_006e7230`]): without ray casts the result is
/// 2 (the path's result untouched). While fewer than 10 casts were made and
/// edges remain, casts between the goal and the first edge (goal to edge
/// for `RCRT_START`, edge to goal otherwise); on a hit the goal and a point
/// near the edge become the nodes (`RCP_SUCCESS`, `RCP_FAILED` without the
/// edge's navmesh, which also clears the edges); on a miss the edge is
/// removed ([`fn_006e7500`]) and the result is `RCP_NONE`, or
/// `RCP_FAILED` once no edge is left. The ray cast data is released
/// ([`fn_006e6db0`]) and the path's result returned. `RET 8`.
// Translation incomplete: 006e6320 - FS:[0] exception frame (unwinding of the local NavMeshPtr and PathingLocation) is not translated.
pub fn pathing_search_ray_cast_build_path(
    e: &mut Engine,
    this: Ptr<PathingSearchRayCast>,
    request: Ptr<RayCastPathRequest>,
    path: Ptr<RayCastPath>,
) -> u32 {
    let kind = e.get(request, RayCastPathRequest::eType);
    e.set(this, PathingSearchRayCast::eType, kind);
    let initial = e.get(request, RayCastPathRequest::pInitialPathRequest);
    e.set(this, PathingSearchRayCast::pRequest, initial);
    let kind = e.get(this, PathingSearchRayCast::eType);
    e.set(path, RayCastPath::eType, kind);
    let max_step = float_arg(e, MAX_STEP);
    let initial = e.get(this, PathingSearchRayCast::pRequest);
    let radius = e.call(REQUEST_ACTOR_RADIUS, &args![initial]).f32();
    let radius = e.call(FLOAT_MAX, &args![radius.to_bits(), max_step]).f32();
    e.set(this, PathingSearchRayCast::fActorRadius, radius);
    e.set(this, PathingSearchRayCast::iRayCastCount, 0);

    e.with_stack(BUILD_PATH_FRAME, |e, frame| {
        let l = |off: u32| local(frame, BUILD_PATH_FRAME, off);
        let goal = this.at(PathingSearchRayCast::GoalLocation);
        let edges = request.at(RayCastPathRequest::Edges).addr();
        match e.get(request, RayCastPathRequest::eType) {
            RCRT_START | RCRT_GOAL => {
                e.call(
                    LOCATION_ASSIGN,
                    &args![goal, request.at(RayCastPathRequest::GoalLocation)],
                );
                fn_006e7230(e, this, Ptr::new(edges));
            }
            RCRT_NOMESH => return build_no_mesh_path(e, this, request, path, frame),
            _ => {}
        }
        if !pathing_search_ray_cast_init_ray_casts(e, this) {
            return RCP_FAILED;
        }
        e.set(this, PathingSearchRayCast::iRayCastCount, 0);
        while e.get(this, PathingSearchRayCast::iRayCastCount) < 10
            && !e.call(IS_EMPTY, &args![edges]).bool()
        {
            let found = if e.get(this, PathingSearchRayCast::eType) == RCRT_START {
                let edge = e.call(FIRST_EDGE, &args![edges]).u32() + EDGE_LOCATION;
                let to = position(e, goal.addr(), l(0x180));
                fn_006e6e40(e, this, Ptr::new(to), Ptr::new(edge))
            } else {
                let to = position(e, goal.addr(), l(0x18c));
                let edge = e.call(FIRST_EDGE, &args![edges]).u32() + EDGE_LOCATION;
                fn_006e6e40(e, this, Ptr::new(edge), Ptr::new(to))
            };
            if found {
                add_edge_nodes(e, this, edges, path, frame);
                break;
            }
            fn_006e7500(e, Ptr::new(edges), 0, true);
            if e.call(IS_EMPTY, &args![edges]).bool() {
                e.set(path, RayCastPath::eResult, RCP_FAILED);
                break;
            }
            e.set(path, RayCastPath::eResult, RCP_NONE);
        }
        fn_006e6db0(e, this);
        e.get(path, RayCastPath::eResult)
    })
}

// Translated from 006e6ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: builds the ray cast cache over the bounds:
/// an AABB of `BoundsMin`/`BoundsMax` as Havok vectors, then on the Havok
/// world of the request's origin cell (`004543c0`): `005533c0`, the world
/// object's (`00541c80`, then `006286d0`) virtual +0x60 sized cache from
/// `006e7620(size, 0)` into `pCache`, virtual +0x68 `(aabb, pCache)`, and
/// `005533c0` again.
// Translation incomplete: 006e6ae0 - stack cookie check (00ec408c) and the 16-byte stack alignment are not translated.
pub fn fn_006e6ae0(e: &mut Engine, this: Ptr<PathingSearchRayCast>) {
    e.with_stack(0x20, |e, aabb| {
        e.call(AABB_CONSTRUCT, &args![aabb]);
        e.call(
            TO_HAVOK_VECTOR,
            &args![aabb, this.at(PathingSearchRayCast::BoundsMin)],
        );
        e.call(
            TO_HAVOK_VECTOR,
            &args![aabb.addr() + 0x10, this.at(PathingSearchRayCast::BoundsMax)],
        );
        let request = e.get(this, PathingSearchRayCast::pRequest);
        let origin = e.call(REQUEST_ORIGIN, &args![request]).u32();
        let cell = e.call(GET_CELL, &args![origin]).u32();
        let world = e.call(CELL_HAVOK_WORLD, &args![cell]).u32();
        e.call(WORLD_005533C0, &args![world]);
        let inner = e.call(WORLD_00541C80, &args![world]).u32();
        let object = e.call(WORD_AT_58, &args![inner]).u32();
        let size = e.vcall(object, 0x60, &args![]).u32();
        let cache = e.call(CACHE_CREATE, &args![size, 0u32]).u32();
        e.set(this, PathingSearchRayCast::pCache, Ptr::new(cache));
        let inner = e.call(WORLD_00541C80, &args![world]).u32();
        let object = e.call(WORD_AT_58, &args![inner]).u32();
        let cache = e.get(this, PathingSearchRayCast::pCache);
        e.vcall(object, 0x68, &args![aabb, cache]);
        e.call(WORLD_005533C0, &args![world]);
    });
}

// Translated from 006e6bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: frees the cache (`005e0b60`) and clears
/// `pCache`.
pub fn fn_006e6bd0(e: &mut Engine, this: Ptr<PathingSearchRayCast>) {
    let cache = e.get(this, PathingSearchRayCast::pCache);
    e.call(CACHE_FREE, &args![cache]);
    e.set(this, PathingSearchRayCast::pCache, Ptr::NULL);
}

// Translated from 006e6c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingSearchRayCast::InitRayCasts` (Xbox PDB): fails without a cell
/// at the request's origin or without its Havok world (kept in
/// `pHavokWorld`). Otherwise builds a filter word (`008c71b0(0)`,
/// `004a39f0(0x26)`, `0059ce80(3)`), the cache ([`fn_006e6ae0`]), the pick
/// data (0xB0 bytes, 16-aligned; `004a3f70(filter)`,
/// `00500a20(pCache)`) and the collector (0x70 bytes, 16-aligned), hands
/// the collector to the pick data (`0059ceb0`) and succeeds.
// Translation incomplete: 006e6c00 - FS:[0] exception frame (unwinding of the placement allocations) is not translated.
pub fn pathing_search_ray_cast_init_ray_casts(
    e: &mut Engine,
    this: Ptr<PathingSearchRayCast>,
) -> bool {
    let request = e.get(this, PathingSearchRayCast::pRequest);
    let origin = e.call(REQUEST_ORIGIN, &args![request]).u32();
    let cell = e.call(GET_CELL, &args![origin]).u32();
    if cell == 0 {
        return false;
    }
    let world = e.call(CELL_HAVOK_WORLD, &args![cell]).u32();
    e.set(this, PathingSearchRayCast::pHavokWorld, Ptr::new(world));
    if e.get(this, PathingSearchRayCast::pHavokWorld).is_null() {
        return false;
    }
    e.with_stack(4, |e, filter| {
        e.call(FILTER_INFO_CONSTRUCT, &args![filter, 0u32]);
        e.call(FILTER_INFO_004A39F0, &args![filter, 0x26u32]);
        e.call(FILTER_INFO_0059CE80, &args![filter, 3u32]);
        fn_006e6ae0(e, this);

        let block = e.call(ALIGNED_ALLOCATE, &args![0x10u32, 0xb0u32]).u32();
        let place = e.call(PLACEMENT, &args![0xb0u32, block]).u32();
        let pick = if place != 0 {
            e.call(PICK_DATA_CONSTRUCT, &args![place]).u32()
        } else {
            0
        };
        e.set(this, PathingSearchRayCast::pPickData, Ptr::new(pick));
        let pick = e.get(this, PathingSearchRayCast::pPickData);
        let word = e.mem.u32(filter.addr());
        e.call(PICK_DATA_004A3F70, &args![pick, word]);
        let pick = e.get(this, PathingSearchRayCast::pPickData);
        let cache = e.get(this, PathingSearchRayCast::pCache);
        e.call(PICK_DATA_00500A20, &args![pick, cache]);

        let block = e.call(ALIGNED_ALLOCATE, &args![0x10u32, 0x70u32]).u32();
        let place = e.call(PLACEMENT, &args![0x70u32, block]).u32();
        let collector = if place != 0 {
            e.call(COLLECTOR_CONSTRUCT, &args![place]).u32()
        } else {
            0
        };
        e.set(this, PathingSearchRayCast::pCollector, Ptr::new(collector));
        let pick = e.get(this, PathingSearchRayCast::pPickData);
        let collector = e.get(this, PathingSearchRayCast::pCollector);
        e.call(PICK_DATA_0059CEB0, &args![pick, collector]);
        true
    })
}

// Translated from 006e6db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: releases the ray cast data: the cache
/// ([`fn_006e6bd0`]), the collector (virtual +4 with 0, then the aligned
/// free) and the pick data (aligned free), clearing each pointer.
pub fn fn_006e6db0(e: &mut Engine, this: Ptr<PathingSearchRayCast>) {
    fn_006e6bd0(e, this);
    let collector = e.get(this, PathingSearchRayCast::pCollector);
    if !collector.is_null() {
        e.vcall(collector.addr(), 4, &args![0u32]);
        let collector = e.get(this, PathingSearchRayCast::pCollector);
        e.call(ALIGNED_FREE, &args![collector]);
        e.set(this, PathingSearchRayCast::pCollector, Ptr::NULL);
    }
    let pick = e.get(this, PathingSearchRayCast::pPickData);
    if !pick.is_null() {
        e.call(ALIGNED_FREE, &args![pick]);
        e.set(this, PathingSearchRayCast::pPickData, Ptr::NULL);
    }
}

/// Bytes of [`fn_006e6e40`]'s frame below `EBP`.
const STEP_FRAME: u32 = 0x6c;

// Translated from 006e6e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: casts from `start` toward `end` in steps of
/// the actor radius: the direction is `end - start` unitized, the step
/// count its length / `fActorRadius` truncated (`_ftol2`). Each step casts
/// (`006e6f90`) from the current point to the point one radius further at
/// the current height and continues from the hit point; a failed step
/// fails. The last cast goes to `end` at the current height; it succeeds
/// when its hit point is less than 64.0 (`010240c0`) above or below `end`
/// (NaN fails). `RET 8`.
pub fn fn_006e6e40(e: &mut Engine, this: Ptr<PathingSearchRayCast>, start: Ptr, end: Ptr) -> bool {
    e.with_stack(STEP_FRAME, |e, frame| {
        let l = |off: u32| local(frame, STEP_FRAME, off);
        let radius_at = this.addr() + PathingSearchRayCast::fActorRadius.off;
        e.call(POINT_SUBTRACT, &args![end, l(0x38), start]);
        let length = e.call(UNITIZE, &args![l(0x38)]).f32();
        e.mem.set_f32(l(0x2c), length);
        let steps_value = e.mem.f32(l(0x2c)) as f64 / e.mem.f32(radius_at) as f64;
        let steps = e.call(FTOL, &args![steps_value]).i32();
        copy_point(e, l(0x10), start.addr());
        let mut i = 0i32;
        while i < steps {
            let radius = float_arg(e, radius_at);
            let scaled = e.call(POINT_SCALE, &args![l(0x38), l(0x60), radius]).u32();
            e.call(POINT_ADD, &args![l(0x10), l(0x54), scaled]);
            copy_float(e, l(0x4c), l(0x8));
            e.call(POINT_CONSTRUCT, &args![l(0x48)]);
            if !e
                .call(RAY_CAST_STEP, &args![this, l(0x10), l(0x54), l(0x48)])
                .bool()
            {
                return false;
            }
            copy_point(e, l(0x10), l(0x48));
            i = i.wrapping_add(1);
        }
        e.call(POINT_CONSTRUCT, &args![l(0x1c)]);
        copy_point(e, l(0x28), end.addr());
        copy_float(e, l(0x20), l(0x8));
        if !e
            .call(RAY_CAST_STEP, &args![this, l(0x10), l(0x28), l(0x1c)])
            .bool()
        {
            return false;
        }
        let difference = (e.mem.f32(l(0x14)) as f64 - e.mem.f32(end.addr() + 8) as f64) as f32;
        let height = e.call(FLOAT_ABS, &args![difference.to_bits()]).f64();
        let limit: f64 = e.global(MAX_HEIGHT_DIFFERENCE);
        height < limit
    })
}

// Translated from 006e7190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: one ray cast from `start` to `end`: resets
/// the pick data's result (+0x30, `0062a2b0`) and the collector
/// (`0062a290`), counts the cast, sets both points and casts through the
/// Havok world's virtual +0xC8 with the pick data; succeeds when that
/// reports no hit. `RET 8`.
pub fn fn_006e7190(e: &mut Engine, this: Ptr<PathingSearchRayCast>, start: Ptr, end: Ptr) -> bool {
    let pick = e.get(this, PathingSearchRayCast::pPickData);
    e.call(PICK_RESULT_0062A2B0, &args![pick.addr() + 0x30]);
    let collector = e.get(this, PathingSearchRayCast::pCollector);
    e.call(COLLECTOR_0062A290, &args![collector]);
    let count = e.get(this, PathingSearchRayCast::iRayCastCount);
    e.set(
        this,
        PathingSearchRayCast::iRayCastCount,
        count.wrapping_add(1),
    );
    let pick = e.get(this, PathingSearchRayCast::pPickData);
    e.call(PICK_DATA_SET_FROM, &args![pick, start]);
    let pick = e.get(this, PathingSearchRayCast::pPickData);
    e.call(PICK_DATA_SET_TO, &args![pick, end]);
    let pick = e.get(this, PathingSearchRayCast::pPickData);
    let world = e.get(this, PathingSearchRayCast::pHavokWorld);
    !e.vcall(world.addr(), 0xc8, &args![pick]).bool()
}

/// `BoundsMin` or `BoundsMax` component `at` combined with `value` through
/// `combine` (`00404010` or `0040ebd0`): `combine(value, current)`.
fn combine_bound(e: &mut Engine, combine: u32, at: u32, value_at: impl FnOnce(&mut Engine) -> u32) {
    let current = float_arg(e, at);
    let value_at = value_at(e);
    let value = float_arg(e, value_at);
    let result = e.call(combine, &args![value, current]).f32();
    e.mem.set_f32(at, result);
}

// Translated from 006e7230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: sets the bounds around the edges' points and
/// the goal: starts from `FLT_MAX` (`01016970`) / `-FLT_MAX` (`01015f5c`),
/// then each component of each edge's point (+0x0C) and of the goal's
/// position goes through `0040ebd0` into `BoundsMin` and `00404010` into
/// `BoundsMax`. `RET 4`.
pub fn fn_006e7230(e: &mut Engine, this: Ptr<PathingSearchRayCast>, edges: Ptr<BSSimpleArray>) {
    const LARGEST: u32 = 0x0101_6970;
    const LOWEST: u32 = 0x0101_5f5c;
    e.with_stack(0x68, |e, frame| {
        let l = |off: u32| local(frame, 0x68, off);
        let min_at = this.at(PathingSearchRayCast::BoundsMin).addr();
        let max_at = this.at(PathingSearchRayCast::BoundsMax).addr();
        let largest = float_arg(e, LARGEST);
        let point = e
            .call(POINT_SET, &args![l(0x10), largest, largest, largest])
            .u32();
        copy_point(e, min_at, point);
        let lowest = float_arg(e, LOWEST);
        let point = e
            .call(POINT_SET, &args![l(0x1c), lowest, lowest, lowest])
            .u32();
        copy_point(e, max_at, point);

        let mut i = 0u32;
        while i < e.call(ARRAY_SIZE, &args![edges]).u32() {
            for (combine, at) in [
                (FLOAT_MIN, min_at),
                (FLOAT_MIN, min_at + 4),
                (FLOAT_MIN, min_at + 8),
                (FLOAT_MAX, max_at),
                (FLOAT_MAX, max_at + 4),
                (FLOAT_MAX, max_at + 8),
            ] {
                let component = (at - min_at) % 0xc;
                combine_bound(e, combine, at, |e| {
                    e.call(EDGE_AT, &args![edges, i]).u32() + EDGE_LOCATION + component
                });
            }
            i = i.wrapping_add(1);
        }

        let goal = this.at(PathingSearchRayCast::GoalLocation).addr();
        for (combine, at, local_off) in [
            (FLOAT_MIN, min_at, 0x28),
            (FLOAT_MIN, min_at + 4, 0x34),
            (FLOAT_MIN, min_at + 8, 0x40),
            (FLOAT_MAX, max_at, 0x4c),
            (FLOAT_MAX, max_at + 4, 0x58),
            (FLOAT_MAX, max_at + 8, 0x64),
        ] {
            let component = (at - min_at) % 0xc;
            combine_bound(e, combine, at, |e| {
                position(e, goal, l(local_off)) + component
            });
        }
    });
}

// Translated from 006e7500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: removes element `index` from an array of
/// 0x18-byte elements (the `Edges`). With `shrink` and when the array
/// should shrink (`006f3170`), the elements move into a new buffer of the
/// shrunk capacity (`00869600`, allocated through vtable slot +4): the ones
/// before `index`, then, after destroying the removed one, `iSize - 1`
/// elements from after it (as the game passes it), and the old buffer is
/// freed. Otherwise the removed element is destroyed and the rest moved
/// down. `iSize` decreases by one. `RET 8`.
pub fn fn_006e7500(e: &mut Engine, this: Ptr<BSSimpleArray>, index: u32, shrink: bool) {
    let offset = index.wrapping_mul(EDGE_SIZE);
    if shrink && e.call(SHOULD_SHRINK, &args![this]).bool() {
        let capacity = e.call(SHRUNK_CAPACITY, &args![this]).u32();
        let buffer = e.vcall(this.addr(), 4, &args![capacity]).u32();
        let old = e.get(this, BSSimpleArray::pBuffer);
        e.call(EDGES_MOVE, &args![this, buffer, old, index]);
        let old = e.get(this, BSSimpleArray::pBuffer);
        e.call(EDGES_DESTROY, &args![this, old.wrapping_add(offset), 1u32]);
        let size = e.get(this, BSSimpleArray::iSize);
        let old = e.get(this, BSSimpleArray::pBuffer);
        e.call(
            EDGES_MOVE,
            &args![
                this,
                buffer.wrapping_add(offset),
                old.wrapping_add(offset).wrapping_add(EDGE_SIZE),
                size.wrapping_sub(1)
            ],
        );
        e.call(FREE_BUFFER, &args![this]);
        e.set(this, BSSimpleArray::pBuffer, buffer);
        e.set(this, BSSimpleArray::iReservedSize, capacity);
    } else {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        e.call(
            EDGES_DESTROY,
            &args![this, buffer.wrapping_add(offset), 1u32],
        );
        let size = e.get(this, BSSimpleArray::iSize);
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        e.call(
            EDGES_MOVE,
            &args![
                this,
                buffer.wrapping_add(offset),
                buffer.wrapping_add(offset).wrapping_add(EDGE_SIZE),
                size.wrapping_sub(index).wrapping_sub(1)
            ],
        );
    }
    let size = e.get(this, BSSimpleArray::iSize);
    e.set(this, BSSimpleArray::iSize, size.wrapping_sub(1));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x006e61a0,
            pathing_search_ray_cast_pathing_search_ray_cast(
                Ptr<PathingSearchRayCast>,
            )
                -> Ptr<PathingSearchRayCast>
        ),
        entry!(
            0x006e6280,
            pathing_search_ray_cast_scalar_deleting_destructor(
                Ptr<PathingSearchRayCast>,
                u32,
            )
                -> Ptr<PathingSearchRayCast>
        ),
        entry!(0x006e62b0, fn_006e62b0(Ptr<PathingSearchRayCast>)),
        entry!(
            0x006e6320,
            pathing_search_ray_cast_build_path(
                Ptr<PathingSearchRayCast>,
                Ptr<RayCastPathRequest>,
                Ptr<RayCastPath>,
            ) -> u32
        ),
        entry!(0x006e6ae0, fn_006e6ae0(Ptr<PathingSearchRayCast>)),
        entry!(0x006e6bd0, fn_006e6bd0(Ptr<PathingSearchRayCast>)),
        entry!(
            0x006e6c00,
            pathing_search_ray_cast_init_ray_casts(Ptr<PathingSearchRayCast>) -> bool
        ),
        entry!(0x006e6db0, fn_006e6db0(Ptr<PathingSearchRayCast>)),
        entry!(
            0x006e6e40,
            fn_006e6e40(Ptr<PathingSearchRayCast>, Ptr, Ptr) -> bool
        ),
        entry!(
            0x006e7190,
            fn_006e7190(Ptr<PathingSearchRayCast>, Ptr, Ptr) -> bool
        ),
        entry!(
            0x006e7230,
            fn_006e7230(Ptr<PathingSearchRayCast>, Ptr<BSSimpleArray>)
        ),
        entry!(0x006e7500, fn_006e7500(Ptr<BSSimpleArray>, u32, bool)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type CallLog = Vec<(u32, Vec<u32>)>;

    fn calls(log: &CallLog, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn yes(_: &mut Engine, _: &[u32]) -> Ret {
        1u32.into_ret()
    }

    fn first_word(_: &mut Engine, a: &[u32]) -> Ret {
        a[0].into_ret()
    }

    fn point(e: &Engine, at: u32) -> [f32; 3] {
        [e.mem.f32(at), e.mem.f32(at + 4), e.mem.f32(at + 8)]
    }

    fn set_point(e: &mut Engine, at: u32, p: [f32; 3]) {
        for (k, v) in p.iter().enumerate() {
            e.mem.set_f32(at + 4 * k as u32, *v);
        }
    }

    fn float(a: u32) -> f32 {
        f32::from_bits(a)
    }

    /// The Havok world's vtable: +0x60 returns 0x40, +0x68 does nothing,
    /// +0xC8 returns the byte at world + 8 (the "hit" flag).
    const WORLD_VTABLE: u32 = 0x0f10_0000;
    /// The collector's vtable (+4 does nothing).
    const COLLECTOR_VTABLE: u32 = 0x0f20_0000;
    /// The edge array's vtable (+4 allocates).
    const EDGES_VTABLE: u32 = 0x0f30_0000;

    /// Doubles for every callee outside this file, mostly with the
    /// behaviour their bodies have (point arithmetic, min/max/abs, location
    /// copies), plus the constants the code reads.
    fn ray_engine() -> Engine {
        let mut e = Engine::new();
        for addr in [
            LOCATION_CONSTRUCT,
            LOCATION_DESTROY,
            POINT_CONSTRUCT,
            OPERATOR_DELETE,
            NODES_ADD,
            RESOLVE_NAV_MESH_INFO,
            ARRAY_008454F0,
            NAV_MESH_PTR_CONSTRUCT,
            NAV_MESH_PTR_DESTROY,
            LOCATION_FROM_POINT,
            FILTER_INFO_004A39F0,
            FILTER_INFO_0059CE80,
            PICK_DATA_004A3F70,
            PICK_DATA_00500A20,
            PICK_DATA_0059CEB0,
            AABB_CONSTRUCT,
            TO_HAVOK_VECTOR,
            WORLD_005533C0,
            CACHE_FREE,
            ALIGNED_FREE,
            PICK_RESULT_0062A2B0,
            COLLECTOR_0062A290,
            PICK_DATA_SET_FROM,
            PICK_DATA_SET_TO,
            EDGES_MOVE,
            EDGES_DESTROY,
            FREE_BUFFER,
            0x0f00_0068,
            0x0f00_0004,
        ] {
            e.register(addr, nothing);
        }
        e.register(SETTING_VALUE, |_, a| (a[0] + 4).into_ret());
        e.register(REQUEST_ACTOR_RADIUS, |e, a| {
            e.mem.f32(a[0] + 0x68).into_ret()
        });
        e.register(FLOAT_MAX, |_, a| {
            let (x, y) = (float(a[0]), float(a[1]));
            (if x <= y { y } else { x }).into_ret()
        });
        e.register(FLOAT_MIN, |_, a| {
            let (x, y) = (float(a[0]), float(a[1]));
            (if y <= x { y } else { x }).into_ret()
        });
        e.register(FLOAT_ABS, |_, a| float(a[0]).abs().into_ret());
        e.register(LOCATION_ASSIGN, |e, a| {
            for k in 0..10 {
                let w = e.mem.u32(a[1] + 4 * k);
                e.mem.set_u32(a[0] + 4 * k, w);
            }
            a[0].into_ret()
        });
        e.register(LOCATION_POSITION, |e, a| {
            copy_point(e, a[1], a[0] + 4);
            a[1].into_ret()
        });
        e.register(POINT_SUBTRACT, |e, a| {
            let (p, q) = (point(e, a[0]), point(e, a[2]));
            set_point(e, a[1], [p[0] - q[0], p[1] - q[1], p[2] - q[2]]);
            a[1].into_ret()
        });
        e.register(POINT_ADD, |e, a| {
            let (p, q) = (point(e, a[0]), point(e, a[2]));
            set_point(e, a[1], [p[0] + q[0], p[1] + q[1], p[2] + q[2]]);
            a[1].into_ret()
        });
        e.register(POINT_SCALE, |e, a| {
            let (p, s) = (point(e, a[0]), float(a[2]));
            set_point(e, a[1], [p[0] * s, p[1] * s, p[2] * s]);
            a[1].into_ret()
        });
        e.register(POINT_ADD_ASSIGN, |e, a| {
            let (p, q) = (point(e, a[0]), point(e, a[1]));
            set_point(e, a[0], [p[0] + q[0], p[1] + q[1], p[2] + q[2]]);
            a[0].into_ret()
        });
        e.register(POINT_SET, |e, a| {
            set_point(e, a[0], [float(a[1]), float(a[2]), float(a[3])]);
            a[0].into_ret()
        });
        e.register(UNITIZE, |e, a| {
            let p = point(e, a[0]);
            let length = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
            set_point(e, a[0], [p[0] / length, p[1] / length, p[2] / length]);
            length.into_ret()
        });
        e.register(FTOL, |_, a| (f64::take(a, &mut 0) as i32).into_ret());
        // A step reaches its target: the result is the target.
        e.register(RAY_CAST_STEP, |e, a| {
            copy_point(e, a[3], a[2]);
            true.into_ret()
        });
        e.register(REQUEST_ORIGIN, |_, a| (a[0] + 0xc).into_ret());
        e.register(GET_CELL, |e, a| e.mem.u32(a[0] + 0x18).into_ret());
        e.register(WATER_HEIGHT, |e, a| e.mem.f32(a[0] + 4).into_ret());
        e.register(CELL_HAVOK_WORLD, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(FILTER_INFO_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        e.register(WORLD_00541C80, first_word);
        e.register(WORD_AT_58, first_word);
        e.register(0x0f00_0060, |_, _| 0x40u32.into_ret());
        e.register(0x0f00_00c8, |e, a| (e.mem.u8(a[0] + 8) as u32).into_ret());
        e.register(CACHE_CREATE, |_, _| 0xcac0u32.into_ret());
        e.register(ALIGNED_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(PLACEMENT, |_, a| a[1].into_ret());
        e.register(PICK_DATA_CONSTRUCT, first_word);
        e.register(COLLECTOR_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], COLLECTOR_VTABLE);
            a[0].into_ret()
        });
        e.register(FIRST_EDGE, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(EDGE_AT, |e, a| {
            (e.mem.u32(a[0] + 4) + a[1] * 0x18).into_ret()
        });
        e.register(ARRAY_SIZE, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(IS_EMPTY, |e, a| (e.mem.u32(a[0] + 8) == 0).into_ret());
        e.register(SHOULD_SHRINK, |_, _| false.into_ret());
        e.register(EDGE_COPY, |e, a| {
            for k in 0..6 {
                let w = e.mem.u32(a[1] + 4 * k);
                e.mem.set_u32(a[0] + 4 * k, w);
            }
            a[0].into_ret()
        });
        e.register(GET_NAV_MESH, yes);
        e.register(NAV_MESH_PTR_GET, |_, _| 0x4e00u32.into_ret());
        // The triangle's center is 10 further along x than the first edge
        // point, 5 higher.
        e.register(TRIANGLE_CENTER, |e, a| {
            set_point(e, a[1], [110.0, 0.0, 5.0]);
            a[1].into_ret()
        });
        e.register(NODES_LAST, |_, _| 0x7770u32.into_ret());
        let mut slots = vec![0u32; 0x33];
        slots[0x60 / 4] = 0x0f00_0060;
        slots[0x68 / 4] = 0x0f00_0068;
        slots[0xc8 / 4] = 0x0f00_00c8;
        e.put_vtable(WORLD_VTABLE, &slots);
        e.put_vtable(COLLECTOR_VTABLE, &[0, 0x0f00_0004]);
        for page in [
            0x0101_1000,
            0x0101_5000,
            0x0101_6000,
            0x0101_e000,
            0x0102_4000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(MAX_STEP, 32.0f32);
        e.set_global(HALF_FLOAT, 0.5f32);
        e.set_global(HALF_DOUBLE, 0.5f64);
        e.set_global(TENTH, 0.1f32);
        e.set_global(MAX_HEIGHT_DIFFERENCE, 64.0f64);
        e.set_global(0x0101_6970, f32::MAX);
        e.set_global(0x0101_5f5c, -f32::MAX);
        e
    }

    /// A cell with a Havok world (hit flag `hit`) and water at `water`.
    fn cell(e: &mut Engine, water: f32, hit: bool) -> u32 {
        let world = e.mem.alloc(0x10);
        e.mem.set_u32(world, WORLD_VTABLE);
        e.mem.set_u8(world + 8, hit as u8);
        let cell = e.mem.alloc(8);
        e.mem.set_u32(cell, world);
        e.mem.set_f32(cell + 4, water);
        cell
    }

    type Search = (
        Ptr<PathingSearchRayCast>,
        Ptr<RayCastPathRequest>,
        Ptr<RayCastPath>,
    );

    /// A search and a request of `kind` from `start` to `goal`, both
    /// locations in `cell`; the initial pathing request's radius is 20.
    fn search(e: &mut Engine, kind: u32, start: [f32; 3], goal: [f32; 3], cell: u32) -> Search {
        let this: Ptr<PathingSearchRayCast> = e.new_object();
        let request: Ptr<RayCastPathRequest> = e.new_object();
        let path: Ptr<RayCastPath> = e.new_object();
        let initial = e.mem.alloc(0xb0);
        e.mem.set_f32(initial + 0x68, 20.0);
        // The initial request's origin (+0x0C) is in the cell too.
        e.mem.set_u32(initial + 0xc + 0x18, cell);
        e.set(request, RayCastPathRequest::eType, kind);
        e.set(
            request,
            RayCastPathRequest::pInitialPathRequest,
            Ptr::new(initial),
        );
        set_point(e, request.addr() + 0x08 + 4, start);
        e.mem.set_u32(request.addr() + 0x08 + 0x18, cell);
        set_point(e, request.addr() + 0x30 + 4, goal);
        e.mem.set_u32(request.addr() + 0x30 + 0x18, cell);
        e.set(path, RayCastPath::eResult, 0x55);
        (this, request, path)
    }

    #[test]
    fn constructor_sets_defaults_and_quiets_the_setting() {
        let mut e = ray_engine();
        e.map(0x011c_f000, 0x1000);
        e.mem.set_u32(MAX_DROP_SETTING + 4, 0x7f80_0001);
        let this: Ptr<PathingSearchRayCast> = e.new_object();
        for off in [0x78u32, 0x7c, 0x80, 0x84, 0x88] {
            e.mem.set_u32(this.addr() + off, 0x99);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006e61a0, &args![this]).u32(), this.addr());
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this.addr()), VTABLE);
        assert_eq!(
            calls(&log, LOCATION_CONSTRUCT),
            vec![vec![this.addr() + 4], vec![this.addr() + 0x2c]]
        );
        assert_eq!(
            calls(&log, POINT_CONSTRUCT),
            vec![vec![this.addr() + 0x5c], vec![this.addr() + 0x68]]
        );
        assert_eq!(e.mem.u32(this.addr() + 0x78), 0x99);
        for off in [0x7cu32, 0x80, 0x84, 0x88] {
            assert_eq!(e.mem.u32(this.addr() + off), 0);
        }
        assert_eq!(e.mem.u32(this.addr() + 0x8c), 0x7fc0_0001);
        assert_eq!(e.get(this, PathingSearchRayCast::fMaxStep), 32.0);
    }

    #[test]
    fn destructors_destroy_goal_then_start_and_free_with_bit_0() {
        let mut e = ray_engine();
        let this: Ptr<PathingSearchRayCast> = e.new_object();
        for (flags, frees) in [(0u32, 0usize), (1, 1)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x006e6280, &args![this, flags]).u32(), this.addr());
            let log = e.call_log.take().unwrap();
            assert_eq!(
                calls(&log, LOCATION_DESTROY),
                vec![vec![this.addr() + 0x2c], vec![this.addr() + 4]]
            );
            assert_eq!(calls(&log, OPERATOR_DELETE).len(), frees);
            assert_eq!(e.mem.u32(this.addr()), VTABLE);
        }
    }

    #[test]
    fn no_mesh_under_water_casts_once_and_sets_the_bounds() {
        let mut e = ray_engine();
        let water = cell(&mut e, 100.0, false);
        let (this, request, path) = search(
            &mut e,
            RCRT_NOMESH,
            [0.0, 0.0, 10.0],
            [30.0, 40.0, 20.0],
            water,
        );
        e.call_log = Some(vec![]);
        let result = e.call(0x006e6320, &args![this, request, path]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(result, RCP_SUCCESS);
        assert_eq!(e.get(path, RayCastPath::eResult), RCP_SUCCESS);
        assert_eq!(e.get(path, RayCastPath::eType), RCRT_NOMESH);
        assert_eq!(e.get(this, PathingSearchRayCast::eType), RCRT_NOMESH);
        assert_eq!(e.get(this, PathingSearchRayCast::fActorRadius), 32.0);
        // Extent 40 around the midpoint (15, 20), heights 10..20.
        assert_eq!(point(&e, this.addr() + 0x5c), [-5.0, 0.0, 10.0]);
        assert_eq!(point(&e, this.addr() + 0x68), [35.0, 40.0, 20.0]);
        assert_eq!(e.get(this, PathingSearchRayCast::iRayCastCount), 1);
        assert_eq!(calls(&log, PICK_DATA_SET_FROM).len(), 1);
        let nodes = path.addr() + 8;
        assert_eq!(
            calls(&log, NODES_ADD),
            vec![
                vec![nodes, request.addr() + 8],
                vec![nodes, request.addr() + 0x30]
            ]
        );
        assert!(calls(&log, RAY_CAST_STEP).is_empty());
        // The ray cast data is released.
        assert_eq!(calls(&log, CACHE_FREE), vec![vec![0xcac0]]);
        assert_eq!(calls(&log, ALIGNED_FREE).len(), 2);
        assert!(e.get(this, PathingSearchRayCast::pPickData).is_null());
        assert!(e.get(this, PathingSearchRayCast::pCollector).is_null());

        // A hit under water fails.
        let wet = cell(&mut e, 100.0, true);
        let (this, request, path) = search(
            &mut e,
            RCRT_NOMESH,
            [0.0, 0.0, 10.0],
            [30.0, 40.0, 20.0],
            wet,
        );
        let result = e.call(0x006e6320, &args![this, request, path]).u32();
        assert_eq!(result, RCP_FAILED);
        assert_eq!(e.get(path, RayCastPath::eResult), RCP_FAILED);
    }

    #[test]
    fn no_mesh_above_water_steps_by_the_radius() {
        let mut e = ray_engine();
        // The goal (z 20) is above the water at 15: stepped casts.
        let dry = cell(&mut e, 15.0, false);
        let (this, request, path) = search(
            &mut e,
            RCRT_NOMESH,
            [0.0, 0.0, 10.0],
            [30.0, 40.0, 20.0],
            dry,
        );
        e.call_log = Some(vec![]);
        let result = e.call(0x006e6320, &args![this, request, path]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(result, RCP_SUCCESS);
        // Length ~51 / radius 32: one step and the last cast.
        let steps = calls(&log, RAY_CAST_STEP);
        assert_eq!(steps.len(), 2);
        assert!(calls(&log, PICK_DATA_SET_FROM).is_empty());
        assert_eq!(e.get(path, RayCastPath::eResult), RCP_SUCCESS);
    }

    #[test]
    fn no_mesh_without_ray_casts_fails_without_releasing() {
        let mut e = ray_engine();
        let (this, request, path) = search(&mut e, RCRT_NOMESH, [0.0; 3], [1.0, 0.0, 0.0], 0);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x006e6320, &args![this, request, path]).u32(),
            RCP_FAILED
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(path, RayCastPath::eResult), RCP_FAILED);
        assert!(calls(&log, CACHE_FREE).is_empty());
    }

    /// Two edges at (100, 0, 0) and (200, 0, 0), with navmesh infos 0x110
    /// and 0x120 and triangle 7.
    fn add_edges(e: &mut Engine, request: Ptr<RayCastPathRequest>) -> u32 {
        let edges = request.addr() + 0x58;
        let buffer = e.mem.alloc(0x30);
        for (i, x) in [100.0f32, 200.0].iter().enumerate() {
            let edge = buffer + 0x18 * i as u32;
            e.mem.set_u32(edge, 0x110 + 0x10 * i as u32);
            e.mem.set_u16(edge + 4, 7);
            set_point(e, edge + 0xc, [*x, 0.0, 0.0]);
        }
        e.mem.set_u32(edges, EDGES_VTABLE);
        e.mem.set_u32(edges + 4, buffer);
        e.mem.set_u32(edges + 8, 2);
        e.mem.set_u32(edges + 0xc, 2);
        edges
    }

    #[test]
    fn goal_request_removes_missed_edges_and_adds_a_point_near_the_hit_one() {
        let mut e = ray_engine();
        let dry = cell(&mut e, -100.0, false);
        let (this, request, path) = search(&mut e, RCRT_GOAL, [0.0; 3], [90.0, 0.0, 0.0], dry);
        let edges = add_edges(&mut e, request);
        let buffer = e.mem.u32(edges + 4);
        let record = e.mem.alloc(0x18);
        let mut casts = 0;
        e.register_double(RAY_CAST_STEP, move |e, a| {
            casts += 1;
            if casts == 1 {
                copy_point(e, record, a[1]);
                copy_point(e, record + 0xc, a[2]);
            }
            copy_point(e, a[3], a[2]);
            (casts > 1).into_ret()
        });
        e.call_log = Some(vec![]);
        let result = e.call(0x006e6320, &args![this, request, path]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(result, RCP_SUCCESS);
        // The goal was copied and the bounds cover edges and goal.
        assert_eq!(
            calls(&log, LOCATION_ASSIGN)[0],
            vec![this.addr() + 0x2c, request.addr() + 0x30]
        );
        assert_eq!(point(&e, this.addr() + 0x5c), [90.0, 0.0, 0.0]);
        assert_eq!(point(&e, this.addr() + 0x68), [200.0, 0.0, 0.0]);
        // The first cast (edge to goal) missed: element 0 was removed.
        let steps = calls(&log, RAY_CAST_STEP);
        assert_eq!(steps.len(), 2);
        assert_eq!(point(&e, record), [100.0, 0.0, 0.0]);
        assert_eq!(point(&e, record + 0xc), [90.0, 0.0, 0.0]);
        assert_eq!(calls(&log, EDGES_DESTROY), vec![vec![edges, buffer, 1]]);
        assert_eq!(e.mem.u32(edges + 8), 1);
        // The point near the hit edge: 10% toward the triangle's center.
        let location = calls(&log, LOCATION_FROM_POINT);
        assert_eq!(location.len(), 1);
        assert_eq!(point(&e, location[0][1]), [101.0, 0.0, 0.5]);
        assert_eq!(&location[0][2..], &[0x110, 7]);
        let nodes = path.addr() + 8;
        assert_eq!(
            calls(&log, NODES_ADD),
            vec![vec![nodes, this.addr() + 0x2c], vec![nodes, location[0][0]]]
        );
        assert_eq!(calls(&log, RESOLVE_NAV_MESH_INFO), vec![vec![0x7770, 0]]);
        assert_eq!(calls(&log, LOCATION_DESTROY), vec![vec![location[0][0]]]);
        assert_eq!(calls(&log, NAV_MESH_PTR_DESTROY).len(), 1);
        assert_eq!(calls(&log, CACHE_FREE).len(), 1);
    }

    #[test]
    fn start_request_casts_from_the_goal_and_fails_without_a_navmesh() {
        let mut e = ray_engine();
        let dry = cell(&mut e, -100.0, false);
        let (this, request, path) = search(&mut e, RCRT_START, [0.0; 3], [90.0, 0.0, 0.0], dry);
        let edges = add_edges(&mut e, request);
        e.register(GET_NAV_MESH, |_, _| false.into_ret());
        let record = e.mem.alloc(0x18);
        e.register_double(RAY_CAST_STEP, move |e, a| {
            copy_point(e, record, a[1]);
            copy_point(e, record + 0xc, a[2]);
            copy_point(e, a[3], a[2]);
            true.into_ret()
        });
        e.call_log = Some(vec![]);
        let result = e.call(0x006e6320, &args![this, request, path]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(result, RCP_FAILED);
        // Goal (90) toward the edge (100): the last cast targets x = 100.
        let steps = calls(&log, RAY_CAST_STEP);
        assert_eq!(steps.len(), 1);
        assert_eq!(point(&e, record), [90.0, 0.0, 0.0]);
        assert_eq!(point(&e, record + 0xc), [100.0, 0.0, 0.0]);
        assert_eq!(calls(&log, ARRAY_008454F0), vec![vec![edges, 1]]);
        assert!(calls(&log, NODES_ADD).is_empty());
        assert_eq!(calls(&log, NAV_MESH_PTR_DESTROY).len(), 1);
    }

    #[test]
    fn edge_search_stops_after_ten_casts_or_without_edges() {
        let mut e = ray_engine();
        let dry = cell(&mut e, -100.0, false);
        // Every cast misses and counts as ten.
        e.register(RAY_CAST_STEP, |e, a| {
            let count = e.mem.u32(a[0] + 0x88);
            e.mem.set_u32(a[0] + 0x88, count + 10);
            false.into_ret()
        });
        let (this, request, path) = search(&mut e, RCRT_GOAL, [0.0; 3], [90.0, 0.0, 0.0], dry);
        let edges = add_edges(&mut e, request);
        assert_eq!(
            e.call(0x006e6320, &args![this, request, path]).u32(),
            RCP_NONE
        );
        assert_eq!(e.mem.u32(edges + 8), 1);

        // Casts that do not count: both edges are removed, then it fails.
        e.register(RAY_CAST_STEP, |_, _| false.into_ret());
        let (this, request, path) = search(&mut e, RCRT_GOAL, [0.0; 3], [90.0, 0.0, 0.0], dry);
        let edges = add_edges(&mut e, request);
        assert_eq!(
            e.call(0x006e6320, &args![this, request, path]).u32(),
            RCP_FAILED
        );
        assert_eq!(e.mem.u32(edges + 8), 0);

        // Without ray casts the path's result is left alone.
        let (this, request, path) = search(&mut e, RCRT_GOAL, [0.0; 3], [90.0, 0.0, 0.0], 0);
        add_edges(&mut e, request);
        assert_eq!(
            e.call(0x006e6320, &args![this, request, path]).u32(),
            RCP_FAILED
        );
        assert_eq!(e.get(path, RayCastPath::eResult), 0x55);
    }

    #[test]
    fn init_ray_casts_needs_a_cell_and_a_world() {
        let mut e = ray_engine();
        let this: Ptr<PathingSearchRayCast> = e.new_object();
        let initial = e.mem.alloc(0xb0);
        e.set(this, PathingSearchRayCast::pRequest, Ptr::new(initial));
        assert!(!e.call(0x006e6c00, &args![this]).bool());
        // A cell without a world.
        let bare = e.mem.alloc(8);
        e.mem.set_u32(initial + 0xc + 0x18, bare);
        assert!(!e.call(0x006e6c00, &args![this]).bool());

        let water = cell(&mut e, 0.0, false);
        e.mem.set_u32(initial + 0xc + 0x18, water);
        e.call_log = Some(vec![]);
        assert!(e.call(0x006e6c00, &args![this]).bool());
        let log = e.call_log.take().unwrap();
        let world = e.mem.u32(water);
        assert_eq!(e.get(this, PathingSearchRayCast::pHavokWorld).addr(), world);
        let pick = e.get(this, PathingSearchRayCast::pPickData).addr();
        let collector = e.get(this, PathingSearchRayCast::pCollector).addr();
        assert_ne!(pick, 0);
        assert_eq!(e.mem.u32(collector), COLLECTOR_VTABLE);
        assert_eq!(
            calls(&log, ALIGNED_ALLOCATE),
            vec![vec![0x10, 0xb0], vec![0x10, 0x70]]
        );
        let filter = calls(&log, FILTER_INFO_CONSTRUCT)[0][0];
        assert_eq!(calls(&log, FILTER_INFO_004A39F0), vec![vec![filter, 0x26]]);
        assert_eq!(calls(&log, FILTER_INFO_0059CE80), vec![vec![filter, 3]]);
        assert_eq!(calls(&log, PICK_DATA_004A3F70), vec![vec![pick, 0]]);
        assert_eq!(calls(&log, PICK_DATA_00500A20), vec![vec![pick, 0xcac0]]);
        assert_eq!(calls(&log, PICK_DATA_0059CEB0), vec![vec![pick, collector]]);

        // Placement returning 0 leaves the pointers null.
        e.register(PLACEMENT, |_, _| 0u32.into_ret());
        assert!(e.call(0x006e6c00, &args![this]).bool());
        assert!(e.get(this, PathingSearchRayCast::pPickData).is_null());
        assert!(e.get(this, PathingSearchRayCast::pCollector).is_null());
    }

    #[test]
    fn cache_is_built_over_the_bounds_aabb() {
        let mut e = ray_engine();
        let water = cell(&mut e, 0.0, false);
        let this: Ptr<PathingSearchRayCast> = e.new_object();
        let initial = e.mem.alloc(0xb0);
        e.mem.set_u32(initial + 0xc + 0x18, water);
        e.set(this, PathingSearchRayCast::pRequest, Ptr::new(initial));
        e.call_log = Some(vec![]);
        e.call(0x006e6ae0, &args![this]);
        let log = e.call_log.take().unwrap();
        let world = e.mem.u32(water);
        let aabb = calls(&log, AABB_CONSTRUCT)[0][0];
        assert_eq!(
            calls(&log, TO_HAVOK_VECTOR),
            vec![
                vec![aabb, this.addr() + 0x5c],
                vec![aabb + 0x10, this.addr() + 0x68]
            ]
        );
        assert_eq!(calls(&log, WORLD_005533C0), vec![vec![world], vec![world]]);
        assert_eq!(calls(&log, CACHE_CREATE), vec![vec![0x40, 0]]);
        assert_eq!(calls(&log, 0x0f00_0068), vec![vec![world, aabb, 0xcac0]]);
        assert_eq!(e.get(this, PathingSearchRayCast::pCache).addr(), 0xcac0);
    }

    #[test]
    fn release_frees_cache_collector_and_pick_data() {
        let mut e = ray_engine();
        let this: Ptr<PathingSearchRayCast> = e.new_object();
        e.set(this, PathingSearchRayCast::pCache, Ptr::new(0xcac0));
        e.call_log = Some(vec![]);
        e.call(0x006e6bd0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, CACHE_FREE), vec![vec![0xcac0]]);
        assert!(e.get(this, PathingSearchRayCast::pCache).is_null());

        let collector = e.mem.alloc(0x70);
        e.mem.set_u32(collector, COLLECTOR_VTABLE);
        e.set(this, PathingSearchRayCast::pCollector, Ptr::new(collector));
        e.set(this, PathingSearchRayCast::pPickData, Ptr::new(0x5150));
        e.call_log = Some(vec![]);
        e.call(0x006e6db0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, 0x0f00_0004), vec![vec![collector, 0]]);
        assert_eq!(
            calls(&log, ALIGNED_FREE),
            vec![vec![collector], vec![0x5150]]
        );
        assert!(e.get(this, PathingSearchRayCast::pCollector).is_null());
        assert!(e.get(this, PathingSearchRayCast::pPickData).is_null());
        // Nothing left to free: only the cache call.
        e.call_log = Some(vec![]);
        e.call(0x006e6db0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, ALIGNED_FREE).is_empty());
        assert_eq!(calls(&log, CACHE_FREE).len(), 1);
    }

    #[test]
    fn stepped_cast_fails_on_a_missed_step_or_a_large_height_difference() {
        let mut e = ray_engine();
        let this: Ptr<PathingSearchRayCast> = e.new_object();
        e.set(this, PathingSearchRayCast::fActorRadius, 10.0);
        let start = e.mem.alloc(0xc);
        let end = e.mem.alloc(0xc);
        set_point(&mut e, start, [0.0, 0.0, 0.0]);
        set_point(&mut e, end, [25.0, 0.0, 63.0]);
        e.call_log = Some(vec![]);
        // Length ~68 / 10 = 6 steps at the start's height, then the end
        // at height 0: 63 below the end passes.
        assert!(e.call(0x006e6e40, &args![this, start, end]).bool());
        let log = e.call_log.take().unwrap();
        let steps = calls(&log, RAY_CAST_STEP);
        assert_eq!(steps.len(), 7);
        assert_eq!(calls(&log, FTOL).len(), 1);
        // 64 does not.
        set_point(&mut e, end, [25.0, 0.0, 64.0]);
        assert!(!e.call(0x006e6e40, &args![this, start, end]).bool());
        // A miss on the last cast fails.
        set_point(&mut e, end, [25.0, 0.0, 1.0]);
        let mut casts = 0;
        e.register_double(RAY_CAST_STEP, move |e, a| {
            casts += 1;
            copy_point(e, a[3], a[2]);
            (casts != 3).into_ret()
        });
        assert!(!e.call(0x006e6e40, &args![this, start, end]).bool());
        // A missed step fails at once.
        e.register(RAY_CAST_STEP, |_, _| false.into_ret());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x006e6e40, &args![this, start, end]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, RAY_CAST_STEP).len(), 1);
    }

    #[test]
    fn stepped_cast_fails_on_a_nan_height() {
        let mut e = ray_engine();
        let this: Ptr<PathingSearchRayCast> = e.new_object();
        e.set(this, PathingSearchRayCast::fActorRadius, 100.0);
        let start = e.mem.alloc(0xc);
        let end = e.mem.alloc(0xc);
        set_point(&mut e, end, [5.0, 0.0, 0.0]);
        // The last cast's hit point has a NaN height.
        e.register(RAY_CAST_STEP, |e, a| {
            e.mem.set_u32(a[3] + 8, 0x7fc0_0000);
            true.into_ret()
        });
        assert!(!e.call(0x006e6e40, &args![this, start, end]).bool());
    }

    #[test]
    fn single_cast_counts_and_succeeds_without_a_hit() {
        let mut e = ray_engine();
        let this: Ptr<PathingSearchRayCast> = e.new_object();
        let water = cell(&mut e, 0.0, false);
        let world = e.mem.u32(water);
        e.set(this, PathingSearchRayCast::pHavokWorld, Ptr::new(world));
        e.set(this, PathingSearchRayCast::pPickData, Ptr::new(0x5100));
        e.set(this, PathingSearchRayCast::pCollector, Ptr::new(0x5200));
        e.set(this, PathingSearchRayCast::iRayCastCount, 4);
        e.call_log = Some(vec![]);
        assert!(e.call(0x006e7190, &args![this, 0x10u32, 0x20u32]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, PICK_RESULT_0062A2B0), vec![vec![0x5130]]);
        assert_eq!(calls(&log, COLLECTOR_0062A290), vec![vec![0x5200]]);
        assert_eq!(calls(&log, PICK_DATA_SET_FROM), vec![vec![0x5100, 0x10]]);
        assert_eq!(calls(&log, PICK_DATA_SET_TO), vec![vec![0x5100, 0x20]]);
        assert_eq!(calls(&log, 0x0f00_00c8), vec![vec![world, 0x5100]]);
        assert_eq!(e.get(this, PathingSearchRayCast::iRayCastCount), 5);
        e.mem.set_u8(world + 8, 1);
        assert!(!e.call(0x006e7190, &args![this, 0x10u32, 0x20u32]).bool());
    }

    #[test]
    fn bounds_cover_the_edges_and_the_goal() {
        let mut e = ray_engine();
        let this: Ptr<PathingSearchRayCast> = e.new_object();
        set_point(&mut e, this.addr() + 0x2c + 4, [5.0, -7.0, 3.0]);
        let edges = e.mem.alloc(0x10);
        let buffer = e.mem.alloc(0x30);
        set_point(&mut e, buffer + 0xc, [1.0, 2.0, 9.0]);
        set_point(&mut e, buffer + 0x18 + 0xc, [-4.0, 6.0, 0.5]);
        e.mem.set_u32(edges + 4, buffer);
        e.mem.set_u32(edges + 8, 2);
        e.call(0x006e7230, &args![this, edges]);
        assert_eq!(point(&e, this.addr() + 0x5c), [-4.0, -7.0, 0.5]);
        assert_eq!(point(&e, this.addr() + 0x68), [5.0, 6.0, 9.0]);
        // Without edges only the goal counts.
        e.mem.set_u32(edges + 8, 0);
        e.call(0x006e7230, &args![this, edges]);
        assert_eq!(point(&e, this.addr() + 0x5c), [5.0, -7.0, 3.0]);
        assert_eq!(point(&e, this.addr() + 0x68), [5.0, -7.0, 3.0]);
    }

    #[test]
    fn remove_edge_moves_in_place_or_into_a_smaller_buffer() {
        let mut e = ray_engine();
        let array: Ptr<BSSimpleArray> = e.new_object();
        e.set(array, BSSimpleArray::pBuffer, 0x8000);
        e.set(array, BSSimpleArray::iSize, 5);
        e.set(array, BSSimpleArray::iReservedSize, 64);
        e.call_log = Some(vec![]);
        e.call(0x006e7500, &args![array, 2u32, true]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, EDGES_DESTROY),
            vec![vec![array.addr(), 0x8030, 1]]
        );
        assert_eq!(
            calls(&log, EDGES_MOVE),
            vec![vec![array.addr(), 0x8030, 0x8048, 2]]
        );
        assert_eq!(e.get(array, BSSimpleArray::iSize), 4);

        e.register(SHOULD_SHRINK, |_, _| true.into_ret());
        e.register(SHRUNK_CAPACITY, |_, _| 32u32.into_ret());
        e.register(0x0f00_0104, |_, a| (0x9000 + a[1]).into_ret());
        e.put_vtable(EDGES_VTABLE, &[0, 0x0f00_0104]);
        e.mem.set_u32(array.addr(), EDGES_VTABLE);
        e.call_log = Some(vec![]);
        e.call(0x006e7500, &args![array, 1u32, true]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, EDGES_MOVE),
            vec![
                vec![array.addr(), 0x9020, 0x8000, 1],
                vec![array.addr(), 0x9020 + 0x18, 0x8000 + 0x30, 3]
            ]
        );
        assert_eq!(
            calls(&log, EDGES_DESTROY),
            vec![vec![array.addr(), 0x8018, 1]]
        );
        assert_eq!(calls(&log, FREE_BUFFER), vec![vec![array.addr()]]);
        assert_eq!(e.get(array, BSSimpleArray::pBuffer), 0x9020);
        assert_eq!(e.get(array, BSSimpleArray::iReservedSize), 32);
        assert_eq!(e.get(array, BSSimpleArray::iSize), 3);

        // Without `shrink` the check is not made.
        e.call_log = Some(vec![]);
        e.call(0x006e7500, &args![array, 0u32, false]);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, SHOULD_SHRINK).is_empty());
        assert_eq!(e.get(array, BSSimpleArray::iSize), 2);
    }
}
