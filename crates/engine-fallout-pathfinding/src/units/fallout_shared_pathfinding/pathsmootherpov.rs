//! `fallout shared/pathfinding/pathsmootherpov.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `PathSmootherPOVSearch` (Xbox PDB) smooths a navmesh corridor into a
//! path: it collects points of view (`PathSmootherPOV`: the start, the goal,
//! corridor corners, avoid nodes) and runs an A* search
//! (`AStarSearch<PathSmootherPOVNode,PathSmootherPOVLink>`) over them.
//!
//! Locals the game keeps on its stack and passes by address live in a
//! [`Frame`]: `l.at(0x44)` is the address of the game's `[EBP - 0x44]`, so
//! the translation reads like the disassembly. The game leaves those locals
//! uninitialized; here they start zeroed.
//!
//! x87 note: values are computed in `f64` and rounded to `f32` where the
//! code stores a `float`; `FLD`/`FSTP` copies of a float go through
//! [`x87_quiet`] / [`x87_quiet_bits`].

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSSimpleArray;

/// `00726bf0(this, address, count)`: runs a no-result per-element step over
/// `count` words (the unit `fallout/interface/run_00726bf0`).
const FILL_ELEMENTS: u32 = 0x0072_6bf0;

// --- Callees outside this unit (called by address) --------------------------

/// `00401000(size)` (cdecl): the game's heap allocation.
const ALLOCATE: u32 = 0x0040_1000;
/// `00401030(block)` (cdecl): the game's heap release.
const DEALLOCATE: u32 = 0x0040_1030;
/// `0044ddc0(array)`: the array's word at +8 (its element count).
const ARRAY_SIZE: u32 = 0x0044_ddc0;
/// `006815c0(this)`: returns `this` (`mov eax, ecx`); the code calls it as a
/// constructor of points and handles and as an accessor.
const SELF: u32 = 0x0068_15c0;
/// `006240d0(this)`: the edge handle constructor the code runs on
/// `PathSmootherPOV::EdgeHandle` and `PathEdgeVert::Edge` (body not read).
const EDGE_HANDLE_CONSTRUCT: u32 = 0x0062_40d0;
/// `00439e90(this, out, other)`: `out = this + other` (three floats).
const POINT_ADD: u32 = 0x0043_9e90;
/// `00439ef0(this, out, other)`: `out = this - other`.
const POINT_SUB: u32 = 0x0043_9ef0;
/// `0045bb20(this, out, float)`: `out = this * float`.
const POINT_SCALE: u32 = 0x0045_bb20;
/// `00457990(this)`: the point's length (ST0).
const POINT_LENGTH: u32 = 0x0045_7990;
/// `004b3800(this, out, other)`: `out = this x other` (cross product).
const POINT_CROSS: u32 = 0x004b_3800;
/// `0053d1a0(this, out, other)`: the cross product made unit length (zero
/// when its length is at most 1e-6).
const POINT_UNIT_CROSS: u32 = 0x0053_d1a0;
/// `004b6190(this, other)`: dot product (ST0).
const POINT_DOT: u32 = 0x004b_6190;
/// `004a0c10(this)`: scales the point to unit length (zeroes it when its
/// length is at most 1e-6).
const POINT_NORMALIZE: u32 = 0x004a_0c10;
/// `0063c8a0(this, other)`: `this += other`.
const POINT_ADD_ASSIGN: u32 = 0x0063_c8a0;
/// `00439180(this, float)`: `this *= float`.
const POINT_SCALE_ASSIGN: u32 = 0x0043_9180;
/// `004a7290(this)`: the squared length, rounded to a `float` (ST0).
const POINT_LENGTH_SQUARED: u32 = 0x004a_7290;
/// `007cb2e0(array, &element)`: appends the word at `&element` to a
/// `BSSimpleArray` of words (the engine map's name,
/// `BSSimpleArray<BGSBodyPart*>::AddUninitialized`, is the folded body).
const ARRAY_APPEND_WORD: u32 = 0x007c_b2e0;
/// `00877a30(array, index)`: address of word element `index`.
const ARRAY_WORD_AT: u32 = 0x0087_7a30;
/// `0084b5c0(array, count)`: the engine map's
/// `BSSimpleArray<Actor*>::SetReservedSize` (folded body).
const ARRAY_SET_RESERVED: u32 = 0x0084_b5c0;
/// `006f9790(array, count)`: the same step for the `PathEdgeVert` arrays
/// (body not read).
const EDGE_VERT_ARRAY_RESERVE: u32 = 0x006f_9790;
/// `008454f0(array, flag)`: the arrays' clear / free step (the destructors
/// pass 1, the search set-up 0).
const ARRAY_CLEAR: u32 = 0x0084_54f0;
/// `0084e3a0(array)`: a count of the avoid-node array (body not read).
const ARRAY_COUNT_0084E3A0: u32 = 0x0084_e3a0;
/// `0059bb30(navmesh)`: the word at +0x24 (the mesh's cell).
const NAVMESH_CELL: u32 = 0x0059_bb30;
/// `0068efb0(navmesh)`: the mesh's `NavMeshInfo` (+0x104).
const NAVMESH_INFO: u32 = 0x0068_efb0;
/// `0068efd0(navmesh, out, triangle, edge)`: the two `u16` vertex indexes of
/// an edge.
const NAVMESH_EDGE_VERTEX_INDEXES: u32 = 0x0068_efd0;
/// `0068f0a0(navmesh, index)`: address of vertex `index`'s position.
const NAVMESH_VERTEX: u32 = 0x0068_f0a0;
/// `0068f040(navmesh, out, triangle, edge)`: `NavMesh::GetEdgeVertices`
/// (Xbox PDB name in the engine map): two vertex position pointers.
const NAVMESH_GET_EDGE_VERTICES: u32 = 0x0068_f040;
/// `0068f0c0(navmesh, triangle, edge, out_a, out_b)`: the edge's two end
/// points (body not read).
const NAVMESH_EDGE_ENDS: u32 = 0x0068_f0c0;
/// `0068f460(navmesh, triangle, edge, &mesh, &triangle, &edge)`:
/// `NavMesh::GetMatchingEdge` (engine map, `_ov3`).
const NAVMESH_GET_MATCHING_EDGE: u32 = 0x0068_f460;
/// `0068f670(navmesh, edge_handle, out_handle)`: `NavMesh::GetMatchingEdge`
/// (engine map, `_ov4`), a bool.
const NAVMESH_GET_MATCHING_EDGE_HANDLE: u32 = 0x0068_f670;
/// `00697980(navmesh, triangle, point, above, below, &z)`:
/// `NavMesh::ComputeTriangleZForLocation` (engine map).
const NAVMESH_COMPUTE_TRIANGLE_Z: u32 = 0x0069_7980;
/// `00698320(navmesh, triangle, edge, from, to)`: the edge of the triangle
/// the segment leaves through, or -1 (body not read).
const NAVMESH_EXIT_EDGE: u32 = 0x0069_8320;
/// `006971e0(navmesh, out, triangle)`: a per-triangle vector (body not read;
/// the code takes the dot product of two of them).
const NAVMESH_TRIANGLE_VECTOR: u32 = 0x0069_71e0;
/// `00699af0(navmesh, triangle)`: `NavMesh::GetClosedDoorInfoForTriangle`
/// (engine map).
const NAVMESH_CLOSED_DOOR_INFO: u32 = 0x0069_9af0;
/// `00691140(triangle, mask)`: whether any bit of `mask` is set in the
/// triangle's flags.
const TRIANGLE_HAS_FLAG: u32 = 0x0069_1140;
/// `00691620(navmesh)`: number of static avoid nodes.
const NAVMESH_AVOID_NODE_COUNT: u32 = 0x0069_1620;
/// `00691640(navmesh, index)`: address of static avoid node `index`.
const NAVMESH_AVOID_NODE: u32 = 0x0069_1640;
/// `0069a660(this, navmesh, vertex)`: builds a vertex handle (word, `u16`).
const VERT_HANDLE_SET: u32 = 0x0069_a660;
/// `0069a690(this, other)`: copies a vertex handle.
const VERT_HANDLE_COPY: u32 = 0x0069_a690;
/// `0069a6c0(this, navmesh, triangle, edge)`: builds an edge handle (word,
/// `u16`, word).
const EDGE_HANDLE_SET: u32 = 0x0069_a6c0;
/// `006bc870(this, other)`: copies an edge handle.
const EDGE_HANDLE_COPY: u32 = 0x006b_c870;
/// `006d4ca0(this, other)`: whether two handles name the same mesh word and
/// `u16`.
const HANDLES_EQUAL: u32 = 0x006d_4ca0;
/// `006a7af0(array, index)`: address of 12-byte element `index` (the edge
/// array).
const EDGE_ARRAY_AT: u32 = 0x006a_7af0;
/// `006dad90(array, index)`: address of element `index` of the triangle
/// array.
const TRIANGLE_ARRAY_AT: u32 = 0x006d_ad90;
/// `006a78d0(element)`: the triangle of a triangle-array element.
const TRIANGLE_ELEMENT_TRIANGLE: u32 = 0x006a_78d0;
/// `006a6eb0(element, out)`: a point of a triangle-array element (its +8 is
/// the height the code compares).
const TRIANGLE_ELEMENT_POINT: u32 = 0x006a_6eb0;
/// `0045cbc0(tes, point, cell)`: the water height at a point.
const WATER_HEIGHT: u32 = 0x0045_cbc0;
/// The `TES` singleton pointer `0045cbc0` is called on.
const TES_SINGLETON: u32 = 0x011d_ea10;
/// `0069ef80(request)`: the request's float at +0x68 (the walker radius).
const REQUEST_RADIUS: u32 = 0x0069_ef80;
/// `0069e390(request)`: the request's word at +0x94 (through `00559450`).
const REQUEST_OBSTACLES: u32 = 0x0069_e390;
/// `006a7980(request)`: the request's byte at +0xa2.
const REQUEST_BYTE_A2: u32 = 0x006a_7980;
/// `006a9540(request)`: the address +0x34 of the request.
const REQUEST_FIELD_34: u32 = 0x006a_9540;
/// `006a9520(this)`: whether bit 1 of the byte at +0x26 is set.
const BYTE_26_BIT_1: u32 = 0x006a_9520;
/// `0069f900(this, a, b)`: returns `a + b`.
const SUM_OF_TWO: u32 = 0x0069_f900;
/// `006ced90(array)`: address of the last 12-byte element.
const NODE_ARRAY_LAST: u32 = 0x006c_ed90;
/// `006c4350(this, other)`: builds a 12-byte record from a handle (word and
/// `u16` copied, body read up to there).
const RECORD_FROM_HANDLE: u32 = 0x006c_4350;
/// `006cf4c0(this, record)`: adds a 12-byte record to the object's array
/// (body read up to the element address).
const ADD_RECORD: u32 = 0x006c_f4c0;
/// `004a7bd0(this)`: the float at +0x18 (ST0).
const FLOAT_18: u32 = 0x004a_7bd0;
/// `007af430(this)`: the word at +0x20.
const WORD_20: u32 = 0x007a_f430;
/// `00559450(this)`: the word at +0.
const WORD_0: u32 = 0x0055_9450;
/// `006a1400(list, index)`: element `index` (body not read).
const OBSTACLE_AT: u32 = 0x006a_1400;
/// `006a0480(this)`: constructor of the 0x18-byte location record
/// `006d4570` fills (body not read).
const LOCATION_RECORD_CONSTRUCT: u32 = 0x006a_0480;
/// `006dd010(this, point, navmesh_info, triangle)`: builds a
/// `PathingLocation` (body not read).
const PATHING_LOCATION_CONSTRUCT: u32 = 0x006d_d010;
/// `004ff7e0(this)`: its destructor (body not read).
const PATHING_LOCATION_DESTRUCT: u32 = 0x004f_f7e0;
/// `006d4570(location, radius, out)` (cdecl): finds a point for a location
/// within a radius (pathfind.rs), a bool.
const FIND_POINT_NEAR: u32 = 0x006d_4570;
/// `00719b20(array, &item, start, compare)`: index of `item` or -1 (body not
/// read).
const ARRAY_FIND: u32 = 0x0071_9b20;
/// The compare function `006af2b0` passes to `00719b20`.
const FIND_COMPARE: u32 = 0x009a_3830;
/// `0069f920(solution, location, point)`: adds a location and a point to the
/// solution (body not read).
const SOLUTION_ADD: u32 = 0x0069_f920;
/// `006ba550(out, a, b, from, to)` (cdecl): a point on the segment `a`-`b`
/// (body not read).
const SEGMENT_POINT: u32 = 0x006b_a550;
/// `006ba100(a, b, from, to, out)` (cdecl): a 2D segment intersection, a
/// bool (body not read).
const SEGMENT_INTERSECT: u32 = 0x006b_a100;
/// `00717e50(this)`: `BaseProcess::GetActorPackageThatIsRunning` in the
/// engine map (folded name; called on `006b05a0`'s result).
const LAST_NODE_FIELD: u32 = 0x0071_7e50;
/// `005c3420(this, out)`: a point of that object.
const NODE_POINT: u32 = 0x005c_3420;
/// `006dd4f0(this)`: `PathingLocation::GetCell` (engine map).
const PATHING_LOCATION_GET_CELL: u32 = 0x006d_d4f0;
/// `0076b610(array)`: whether the word at +8 is zero.
const ARRAY_IS_EMPTY: u32 = 0x0076_b610;
/// `006e8fe0(array)`: the last element (body not read).
const ARRAY_LAST: u32 = 0x006e_8fe0;
/// `00413f40(this)`: address +8 of the object (an A* node's `NodeData`).
const NODE_DATA: u32 = 0x0041_3f40;

// --- Constants read from the exe ---------------------------------------------

/// `0.5` (`float`).
const HALF: u32 = 0x0101_6248;
/// `100.0` (`float`).
const HUNDRED: u32 = 0x0101_6410;
/// `0.2` (`float`).
const FIFTH: u32 = 0x0101_df44;
/// `64.0` (`float`): the depth below a point `00697980` searches.
const SEARCH_BELOW: u32 = 0x0101_e6ec;
/// `180.0` (`float`): the height above a point `00697980` searches.
const SEARCH_ABOVE: u32 = 0x0106_b158;
/// `10.0` (`float`).
const TEN: u32 = 0x0101_7b78;
/// `1.01` (`float`).
const OFFSET_FACTOR: u32 = 0x0106_b9e0;
/// `0.01` (`float`).
const STEP_FACTOR: u32 = 0x0101_3ea4;
/// The zero point (`NiPoint3::ZERO` shape: three floats).
const ZERO_POINT: u32 = 0x011f_426c;
/// The up axis `004b3800` / `0053d1a0` are called on.
const UP_AXIS: u32 = 0x011a_9484;
/// `0.4` (`double`).
const D_0_4: u32 = 0x0102_90c0;
/// `0.25` (`double`).
const D_0_25: u32 = 0x0102_90b0;
/// `0.75` (`double`).
const D_0_75: u32 = 0x0101_de30;
/// `50.0` (`double`).
const D_50: u32 = 0x0101_e2c0;
/// `0.0` (`double`).
const D_ZERO: u32 = 0x0101_2060;
/// `0.05` (`double`).
const D_0_05: u32 = 0x0103_c7c8;
/// `-0.05` (`double`).
const D_MINUS_0_05: u32 = 0x0106_b9d8;
/// `1.2` (`double`).
const D_1_2: u32 = 0x0102_90f0;
/// `0.1` (`double`).
const D_0_1: u32 = 0x0101_ffa0;
/// `0.9` (`double`).
const D_0_9: u32 = 0x0106_b9e8;
/// `1024.0` (`double`).
const D_1024: u32 = 0x0101_ecb8;

// --- Vtables (RTTI names read from the exe) ---------------------------------

/// `PathSmootherPOVSearch`.
const VTABLE_SEARCH: u32 = 0x0106_b9cc;
/// `AStarSearch<PathSmootherPOVNode,PathSmootherPOVLink>`.
const VTABLE_ASTAR_SEARCH: u32 = 0x0106_ba0c;
/// `BSSimpleArray<PathSmootherPOV*,1024>`.
const VTABLE_POV_ARRAY: u32 = 0x0106_ba1c;
/// `BSSimpleArray<PathSmootherPOVNode,1024>`.
const VTABLE_NODE_ARRAY: u32 = 0x0106_ba30;
/// `BSSimpleArray<PathSmootherPOVLink,1024>`.
const VTABLE_LINK_ARRAY: u32 = 0x0106_ba44;
/// `BSSimpleArray<PathSmootherPOVSearch::PathEdgeVert,1024>`.
const VTABLE_EDGE_VERT_ARRAY: u32 = 0x0106_ba58;
/// `BSSimpleArray<NavMesh*,1024>`.
const VTABLE_NAVMESH_ARRAY: u32 = 0x0106_ba6c;
/// `BSScrapArray<AStarNode<PathSmootherPOVNode,PathSmootherPOVLink>*,1024>`.
const VTABLE_ASTAR_NODE_SCRAP_ARRAY: u32 = 0x0106_ba80;
/// `BSSimpleArray<AStarNode<PathSmootherPOVNode,PathSmootherPOVLink>*,1024>`.
const VTABLE_ASTAR_NODE_ARRAY: u32 = 0x0106_ba94;

// --- Layouts -----------------------------------------------------------------

/// `AStarSearch::NodeHeap` (+0x04, 0x10 bytes on PC).
pub(crate) const NODE_HEAP: u32 = 0x04;
/// `AStarSearch::AStarNodeMap` (+0x14): buckets of node pointers.
pub(crate) const NODE_MAP: u32 = 0x14;
/// `AStarSearch::SearchQueue` (+0x1ff4).
pub(crate) const SEARCH_QUEUE: u32 = 0x1ff4;

layout! {
    /// `AStarSearch<PathSmootherPOVNode,PathSmootherPOVLink>` (Xbox PDB),
    /// PC offsets: the node heap is 0x10 bytes (0x14 on the Xbox), so
    /// everything after it sits 4 lower.
    pub struct AStarSearch: 0x2058 {
        /// `BestNodeSoFar` (Xbox PDB).
        0x2048 BestNodeSoFar: u32,
        /// `pLastSearchedNode` (Xbox PDB).
        0x204C pLastSearchedNode: u32,
        /// `pStartAStarNode` (Xbox PDB).
        0x2050 pStartAStarNode: u32,
        /// `bIsRunningInTasklet` (Xbox PDB).
        0x2054 bIsRunningInTasklet: u8,
    }

    /// `PathSmootherPOVSearch` (Xbox PDB), PC offsets (4 lower than the
    /// Xbox's, see [`AStarSearch`]); the size is from the last field.
    pub struct PathSmootherPOVSearch: 0x2110 {
        /// `pRequest` (Xbox PDB).
        0x2058 pRequest: u32,
        /// `pSolution` (Xbox PDB).
        0x205C pSolution: u32,
        /// `StartLocation.x` (Xbox PDB `NiPoint3`).
        0x2060 StartLocation: f32,
        /// `GoalLocation.x`.
        0x206C GoalLocation: f32,
        /// `bUnderWaterPath` (Xbox PDB).
        0x2078 bUnderWaterPath: u8,
        /// `Triangles` (Xbox PDB): the corridor's triangle array.
        0x207C Triangles: u32,
        /// `Edges` (Xbox PDB): the corridor's edge array.
        0x2080 Edges: u32,
        /// `pStartPov` (Xbox PDB).
        0x20C4 pStartPov: u32,
        /// `pGoalPov` (Xbox PDB).
        0x20C8 pGoalPov: u32,
        /// `pDetailedPathInfos` (Xbox PDB).
        0x210C pDetailedPathInfos: u32,
    }

    /// `PathSmootherPOV` (Xbox PDB), 0x4c bytes (the size `new` asks for).
    pub struct PathSmootherPOV: 0x4C {
        /// `eType` (Xbox PDB).
        0x00 eType: u32,
        /// `Center.x` (Xbox PDB).
        0x04 Center: f32,
        /// `fRadius` (Xbox PDB).
        0x10 fRadius: f32,
        /// `fCostFactor` (Xbox PDB).
        0x38 fCostFactor: f32,
        /// `iThisSideIndex` (Xbox PDB).
        0x3C iThisSideIndex: u32,
        /// `iOtherSideIndex` (Xbox PDB).
        0x40 iOtherSideIndex: u32,
        /// `iAvoidNodeIndex` (Xbox PDB).
        0x44 iAvoidNodeIndex: u32,
        /// `iVertexIndex` (Xbox PDB).
        0x48 iVertexIndex: u16,
    }

    /// `AStarNode<PathSmootherPOVNode,PathSmootherPOVLink>` (Xbox PDB), 0x68
    /// bytes, the same offsets on PC: `NodeData` at +8 (12 bytes),
    /// `ParentLink` at +0x14 (a `PathSmootherPOVLink`, 0x40 bytes).
    pub struct PovAStarNode: 0x68 {
        /// `fCost` (Xbox PDB).
        0x00 fCost: f32,
        /// `fHeuristic` (Xbox PDB).
        0x04 fHeuristic: f32,
        /// `pParentAStarNode` (Xbox PDB).
        0x54 pParentAStarNode: u32,
        /// `ppPrevNode` (Xbox PDB): the queue slot that points at the node.
        0x58 ppPrevNode: u32,
        /// `pNextNode` (Xbox PDB): the next node in the queue bucket.
        0x5C pNextNode: u32,
        /// `iKey` (Xbox PDB).
        0x60 iKey: u32,
        /// `pNextNodeInBucket` (Xbox PDB): the next node in the map bucket.
        0x64 pNextNodeInBucket: u32,
    }

    /// `PathSmootherPOVSearch::PathEdgeVert` (Xbox PDB), 0x20 bytes.
    pub struct PathEdgeVert: 0x20 {
        /// `Vert` (Xbox PDB): navmesh word, then the `u16` vertex.
        0x00 Vert: u32,
        /// `iOtherSideIndex` (Xbox PDB).
        0x14 iOtherSideIndex: u32,
        /// `iThisSideIndex` (Xbox PDB).
        0x18 iThisSideIndex: u32,
        /// `iPOVIndex` (Xbox PDB).
        0x1C iPOVIndex: u32,
    }
}

// The `PathSmootherPOV` field offsets the code writes outside the layout.
const POV_EDGE_HANDLE: u32 = 0x14;
const POV_EDGE1: u32 = 0x20;
const POV_EDGE2: u32 = 0x2c;
// The search's arrays (`BSSimpleArray`s, 0x10 bytes each).
const LEFT_POVS: u32 = 0x2084;
const RIGHT_POVS: u32 = 0x2094;
const AVOID_NODE_POVS: u32 = 0x20a4;
const ALL_POVS: u32 = 0x20b4;
const NODES: u32 = 0x20cc;
const LINKS: u32 = 0x20dc;
const LEFT_VERTS: u32 = 0x20ec;
const RIGHT_VERTS: u32 = 0x20fc;
const START: u32 = 0x2060;
const GOAL: u32 = 0x206c;

/// The game's stack frame of a function: `at(off)` is `[EBP - off]`.
#[derive(Clone, Copy)]
struct Frame {
    top: u32,
}

impl Frame {
    fn at(self, off: u32) -> u32 {
        self.top - off
    }
}

/// Runs `f` with a zeroed frame of `size` bytes.
fn with_frame<R>(e: &mut Engine, size: u32, f: impl FnOnce(&mut Engine, Frame) -> R) -> R {
    e.with_stack(size, |e, p| {
        f(
            e,
            Frame {
                top: p.addr() + size,
            },
        )
    })
}

/// Copies three words (a point) with `MOV`s.
fn copy3(e: &mut Engine, dst: u32, src: u32) {
    for k in 0..3 {
        let w = e.mem.u32(src + 4 * k);
        e.mem.set_u32(dst + 4 * k, w);
    }
}

fn size(e: &mut Engine, array: u32) -> u32 {
    e.call(ARRAY_SIZE, &args![array]).u32()
}

fn f64_at(e: &Engine, addr: u32) -> f64 {
    e.mem.f64(addr)
}

/// `a < b`, false when either is NaN (the x87 flag tests the code makes:
/// a negated `less` is "not below", NaN included).
fn less(a: f64, b: f64) -> bool {
    a < b
}

/// A `float` constant pushed with `FLD`/`FSTP`: its bits, quieted.
fn pushed_float(e: &Engine, addr: u32) -> u32 {
    x87_quiet_bits(e.mem.u32(addr))
}

/// `new PathSmootherPOV`: `00401000(0x4c)` and, when that is not null, the
/// constructor `006ae120`; null otherwise.
fn new_pov(e: &mut Engine) -> u32 {
    let block = e.call(ALLOCATE, &args![0x4cu32]).u32();
    if block != 0 {
        fn_006ae120(e, Ptr::new(block)).addr()
    } else {
        0
    }
}

/// The request's walker radius (`0069ef80`, ST0).
fn request_radius(e: &mut Engine, s: u32) -> f64 {
    let request = e.mem.u32(s + 0x2058);
    e.call(REQUEST_RADIUS, &args![request]).f64()
}

/// `FLD1; FSTP [pov + 0x38]`.
fn set_cost_factor_one(e: &mut Engine, pov: u32) {
    e.mem.set_f32(pov + 0x38, 1.0);
}

/// The zero point copied into `Edge1` and `Edge2`.
fn zero_edges(e: &mut Engine, pov: u32) {
    copy3(e, pov + POV_EDGE1, ZERO_POINT);
    copy3(e, pov + POV_EDGE2, ZERO_POINT);
}

// Translated from 006b3eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the constructor shape of a `BSSimpleArray`
/// (buffer at +4, size at +8, reserved size at +0xc; the three start at 0).
/// `reserved_hint` and `size` are the two stack words: the reserved size is
/// the larger of the two (unsigned). When it is nonzero the buffer comes from
/// virtual slot +4 of the object (argument: the reserved size) and
/// +0xc is set; when `size` is nonzero the buffer's `size` elements are
/// prepared with `00726bf0` and +8 is set. The class is not confirmed from
/// the exe, only the layout.
pub fn fn_006b3eb0(e: &mut Engine, this: Ptr<BSSimpleArray>, reserved_hint: u32, size: u32) {
    init_array(e, this, reserved_hint, size, |e, buffer| {
        e.call(FILL_ELEMENTS, &args![this, buffer, size]);
    });
}

/// The array initialiser shape `006b3eb0`, `006b3f40` and `006b4190` share:
/// buffer, size and reserved size 0; the reserved size is the larger of
/// `reserved_hint` and `size` (unsigned); when nonzero the buffer comes from
/// virtual slot +4 (`_Allocate`, Xbox PDB) and +0xc is set; when `size` is
/// nonzero `prepare(buffer)` runs on the buffer and +8 is set.
fn init_array(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    reserved_hint: u32,
    size: u32,
    prepare: impl FnOnce(&mut Engine, u32),
) {
    e.set(this, BSSimpleArray::pBuffer, 0);
    e.set(this, BSSimpleArray::iSize, 0);
    e.set(this, BSSimpleArray::iReservedSize, 0);
    let mut reserved = reserved_hint;
    if size > reserved {
        reserved = size;
    }
    if reserved > 0 {
        let buffer = e.vcall(this.addr(), 4, &args![reserved]).u32();
        e.set(this, BSSimpleArray::pBuffer, buffer);
        e.set(this, BSSimpleArray::iReservedSize, reserved);
    }
    if size > 0 {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        prepare(e, buffer);
        e.set(this, BSSimpleArray::iSize, size);
    }
}
// Translated from 006ad660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathSmootherPOVSearch::`scalar deleting destructor'` (Xbox PDB): runs
/// the destructor `006ad690` and frees the object when bit 0 of `flags` is
/// set; returns `this`.
pub fn path_smoother_pov_search_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<PathSmootherPOVSearch>,
    flags: u32,
) -> Ptr<PathSmootherPOVSearch> {
    fn_006ad690(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 006ad690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `PathSmootherPOVSearch` destructor body.
/// Sets the class's vtable, destroys the arrays from the last to the first
/// (`RightVerts`, `LeftVerts`, `Links`, `Nodes`, `allPOVs`,
/// `AvoidNodePOVs`, `RightPOVs`, `LeftPOVs`) and then the `AStarSearch` base
/// (`006b3270`).
// Translation incomplete: 006ad690 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn fn_006ad690(e: &mut Engine, this: Ptr<PathSmootherPOVSearch>) {
    let s = this.addr();
    e.mem.set_u32(s, VTABLE_SEARCH);
    fn_006b3710(e, Ptr::new(s + RIGHT_VERTS));
    fn_006b3710(e, Ptr::new(s + LEFT_VERTS));
    fn_006b3640(e, Ptr::new(s + LINKS));
    fn_006b35f0(e, Ptr::new(s + NODES));
    fn_006b35a0(e, Ptr::new(s + ALL_POVS));
    fn_006b35a0(e, Ptr::new(s + AVOID_NODE_POVS));
    fn_006b35a0(e, Ptr::new(s + RIGHT_POVS));
    fn_006b35a0(e, Ptr::new(s + LEFT_POVS));
    fn_006b3270(e, this.cast());
}

/// `00697980` on the first (`006b3180`) or last (`006b31a0`) triangle of
/// `triangles` for the point at `s + point`: the point's z is replaced by
/// the triangle height found within 180 above / 64 below.
fn snap_point_z(
    e: &mut Engine,
    s: u32,
    triangles: Ptr,
    element: fn(&mut Engine, Ptr) -> Ptr,
    point: u32,
    z_slot: u32,
) {
    let z = x87_quiet(e.mem.f32(s + point + 8));
    e.mem.set_f32(z_slot, z);
    let below = pushed_float(e, SEARCH_BELOW);
    let above = pushed_float(e, SEARCH_ABOVE);
    let first = element(e, triangles);
    let triangle = u32::from(e.mem.u16(first.addr() + 4));
    let first = element(e, triangles);
    let navmesh = e.mem.u32(first.addr());
    e.call(
        NAVMESH_COMPUTE_TRIANGLE_Z,
        &args![navmesh, triangle, s + point, above, below, z_slot],
    );
    let z = x87_quiet(e.mem.f32(z_slot));
    e.mem.set_f32(s + point + 8, z);
}

/// The water test of `006ad770` for a point whose triangle element's cell
/// is known to be non-null: the element point's height (`006a6eb0`, +8)
/// under the water height (`0045cbc0`) + 50.
fn under_water(
    e: &mut Engine,
    triangles: Ptr,
    element: fn(&mut Engine, Ptr) -> Ptr,
    point: u32,
    out: u32,
) -> bool {
    let first = element(e, triangles);
    let p = e.call(TRIANGLE_ELEMENT_POINT, &args![first, out]).u32();
    let first = element(e, triangles);
    let navmesh = e.mem.u32(first.addr());
    let cell = e.call(NAVMESH_CELL, &args![navmesh]).u32();
    let tes = e.mem.u32(TES_SINGLETON);
    let water = e.call(WATER_HEIGHT, &args![tes, point, cell]).f64();
    let limit = water + f64_at(e, D_50);
    f64::from(e.mem.f32(p + 8)) < limit
}

// Translated from 006ad770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the search's entry. Stores the request,
/// corridor (`triangles`, `edges`), start and goal points, solution and
/// detailed path infos; decides whether both ends are under water (then sets
/// `bUnderWaterPath`) and otherwise snaps each end's z to its triangle;
/// reserves the arrays, builds the points of view (`006adcb0`), adds the
/// start node (`006b32f0`, estimate 0.75 x the distance) and runs the A*
/// search (`006b3380`). When it fails, `006b1970` runs and, with a
/// `retry_target`, `006b2fc0` and (for a last node whose POV type is 0 or 1)
/// its edge record goes to `retry_target` (`006cf4c0`). Then the solution is
/// written (`006afcc0`), closed-door infos of the corridor's triangles with
/// flag 0x1000 are added to the solution (+0x30), and every POV is freed.
/// Returns whether the search succeeded.
#[allow(clippy::too_many_arguments)]
pub fn fn_006ad770(
    e: &mut Engine,
    this: Ptr<PathSmootherPOVSearch>,
    request: Ptr,
    triangles: Ptr,
    edges: Ptr,
    start: Ptr,
    goal: Ptr,
    solution: Ptr,
    retry_target: Ptr,
    detailed_infos: Ptr,
) -> bool {
    let s = this.addr();
    with_frame(e, 0x78, |e, l| {
        e.mem.set_u32(s + 0x2058, request.addr());
        copy3(e, s + START, start.addr());
        copy3(e, s + GOAL, goal.addr());
        e.mem.set_u32(s + 0x207c, triangles.addr());
        e.mem.set_u32(s + 0x2080, edges.addr());
        e.mem.set_u32(s + 0x205c, solution.addr());
        e.mem.set_u32(s + 0x210c, detailed_infos.addr());

        let first = fn_006b3180(e, triangles);
        let navmesh = e.mem.u32(first.addr());
        let cell = e.call(NAVMESH_CELL, &args![navmesh]).u32();
        let start_under = if cell != 0 {
            under_water(e, triangles, fn_006b3180, start.addr(), l.at(0x48))
        } else {
            true
        };
        if !start_under {
            snap_point_z(e, s, triangles, fn_006b3180, START, l.at(0x14));
        }

        let mut goal_under = false;
        let field = e.call(REQUEST_FIELD_34, &args![request]).u32();
        if !e.call(BYTE_26_BIT_1, &args![field]).bool() {
            let last = fn_006b31a0(e, triangles);
            let navmesh = e.mem.u32(last.addr());
            let cell = e.call(NAVMESH_CELL, &args![navmesh]).u32();
            if cell != 0 && under_water(e, triangles, fn_006b31a0, goal.addr(), l.at(0x54)) {
                goal_under = true;
            }
        }
        if !goal_under {
            snap_point_z(e, s, triangles, fn_006b31a0, GOAL, l.at(0x18));
        }
        if start_under && goal_under {
            e.mem.set_u8(s + 0x2078, 1);
        }

        let n = size(e, edges.addr());
        e.call(ARRAY_SET_RESERVED, &args![s + LEFT_POVS, n]);
        let n = size(e, edges.addr());
        e.call(ARRAY_SET_RESERVED, &args![s + RIGHT_POVS, n]);
        let n = size(e, edges.addr());
        e.call(EDGE_VERT_ARRAY_RESERVE, &args![s + LEFT_VERTS, n]);
        let n = size(e, edges.addr());
        e.call(EDGE_VERT_ARRAY_RESERVE, &args![s + RIGHT_VERTS, n]);
        let request_word = e.mem.u32(s + 0x2058);
        let reserve = if e.call(REQUEST_OBSTACLES, &args![request_word]).u32() != 0 {
            let request_word = e.mem.u32(s + 0x2058);
            let obstacles = e.call(REQUEST_OBSTACLES, &args![request_word]).u32();
            size(e, obstacles) << 1
        } else {
            0
        };
        e.call(ARRAY_SET_RESERVED, &args![s + AVOID_NODE_POVS, reserve]);
        let edge_count = size(e, edges.addr());
        let avoid = e
            .call(ARRAY_COUNT_0084E3A0, &args![s + AVOID_NODE_POVS])
            .u32();
        let all = avoid
            .wrapping_add(edge_count.wrapping_mul(2))
            .wrapping_add(2);
        e.call(ARRAY_SET_RESERVED, &args![s + ALL_POVS, all]);

        fn_006adcb0(e, this);

        // The start node's data: the start POV, direction 0 (the third word
        // is left as the stack had it).
        let start_pov = e.mem.u32(s + 0x20c4);
        e.mem.set_u32(l.at(0xc), start_pov);
        e.mem.set_u32(l.at(0x8), 0);
        e.call(POINT_SUB, &args![s + GOAL, l.at(0x60), s + START]);
        let length = e.call(POINT_LENGTH, &args![l.at(0x60)]).f64();
        let estimate = (length * f64_at(e, D_0_75)) as f32;
        let start_pov = e.mem.u32(s + 0x20c4);
        let key = e.call(SUM_OF_TWO, &args![s, start_pov, 0u32]).u32();
        fn_006b32f0(e, this.cast(), key, Ptr::new(l.at(0xc)), estimate);
        e.call(ARRAY_CLEAR, &args![s + NODES, 0u32]);
        e.call(ARRAY_CLEAR, &args![s + LINKS, 0u32]);
        let found = fn_006b3380(e, this.cast(), Ptr::new(s + NODES), Ptr::new(s + LINKS), 1);
        if !found {
            fn_006b1970(e, this);
            if !retry_target.is_null() {
                fn_006b2fc0(e, this, retry_target);
                let last = e.call(NODE_ARRAY_LAST, &args![s + NODES]).u32();
                let pov = e.mem.u32(last);
                let mut matches = e.mem.u32(pov) == 0;
                if !matches {
                    let last = e.call(NODE_ARRAY_LAST, &args![s + NODES]).u32();
                    let pov = e.mem.u32(last);
                    matches = e.mem.u32(pov) == 1;
                }
                if matches {
                    let last = e.call(NODE_ARRAY_LAST, &args![s + NODES]).u32();
                    let pov = e.mem.u32(last);
                    e.call(
                        RECORD_FROM_HANDLE,
                        &args![l.at(0x24), pov + POV_EDGE_HANDLE],
                    );
                    let w = e.mem.u32(l.at(0x24));
                    e.mem.set_u32(l.at(0x30), w);
                    let h = e.mem.u16(l.at(0x20));
                    e.mem.set_u16(l.at(0x2c), h);
                    let w = e.mem.u32(l.at(0x1c));
                    e.mem.set_u32(l.at(0x28), w);
                    e.call(ADD_RECORD, &args![retry_target, l.at(0x30)]);
                }
            }
        }
        fn_006afcc0(e, this, u8::from(!found));

        let mut i = 0u32;
        while i < size(e, triangles.addr()) {
            let element = fn_006b3160(e, triangles, i);
            let triangle = e.call(TRIANGLE_ELEMENT_TRIANGLE, &args![element]).u32();
            if e.call(TRIANGLE_HAS_FLAG, &args![triangle, 0x1000u32])
                .bool()
            {
                let element = fn_006b3160(e, triangles, i);
                let index = u32::from(e.mem.u16(element.addr() + 4));
                let element = fn_006b3160(e, triangles, i);
                let navmesh = e.mem.u32(element.addr());
                let info = e
                    .call(NAVMESH_CLOSED_DOOR_INFO, &args![navmesh, index])
                    .u32();
                let w = e.mem.u32(info);
                e.mem.set_u32(l.at(0x38), w);
                let solution = e.mem.u32(s + 0x205c);
                e.call(ARRAY_APPEND_WORD, &args![solution + 0x30, l.at(0x38)]);
            }
            i += 1;
        }
        let mut i = 0u32;
        while i < size(e, s + ALL_POVS) {
            let slot = e.call(ARRAY_WORD_AT, &args![s + ALL_POVS, i]).u32();
            let pov = e.mem.u32(slot);
            e.call(DEALLOCATE, &args![pov]);
            i += 1;
        }
        found
    })
}

/// Appends the POV pointer held in the frame slot `slot` to the search's
/// array at `s + array`.
fn append_pov(e: &mut Engine, s: u32, array: u32, slot: u32) {
    e.call(ARRAY_APPEND_WORD, &args![s + array, slot]);
}

/// The `PathSmootherPOV` set-up repeated in `006adcb0` after the center is
/// written: radius = distance(start, goal) x `factor` (a `double`),
/// `iVertexIndex` = 0xffff, cost factor 100; added to `allPOVs` and
/// `AvoidNodePOVs`.
fn finish_goal_pov(e: &mut Engine, s: u32, slot: u32, distance_slot: u32, factor: u32) {
    e.call(POINT_SUB, &args![s + START, distance_slot, s + GOAL]);
    let length = e.call(POINT_LENGTH, &args![distance_slot]).f64();
    let pov = e.mem.u32(slot);
    e.mem
        .set_f32(pov + 0x10, (length * f64_at(e, factor)) as f32);
    e.mem.set_u16(pov + 0x48, 0xffff);
    let cost = x87_quiet(e.mem.f32(HUNDRED));
    e.mem.set_f32(pov + 0x38, cost);
    append_pov(e, s, ALL_POVS, slot);
    append_pov(e, s, AVOID_NODE_POVS, slot);
}

// Translated from 006adcb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: builds the points of view: the start POV
/// (`006ae190`), the corridor's edge vertices and corner POVs (`006ae370`),
/// the goal POV (`006ae280`), avoid-node POVs (`006aefc0`, `006af2b0`) and
/// the end-point POVs (`006af500`). When the request's byte at +0xa2 is set,
/// three type-4 POVs (cost factor 100) are added to `allPOVs` and
/// `AvoidNodePOVs`: one at the middle of start and goal (radius 0.4 x their
/// distance) and two beside the goal (goal + 0.2 x (start - goal) +/- 0.2 x
/// the perpendicular `up x (start - goal)`, radius 0.25 x the distance).
// Translation incomplete: 006adcb0 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn fn_006adcb0(e: &mut Engine, this: Ptr<PathSmootherPOVSearch>) {
    let s = this.addr();
    fn_006ae190(e, this);
    fn_006ae370(e, this);
    fn_006ae280(e, this);
    fn_006aefc0(e, this);
    fn_006af2b0(e, this);
    fn_006af500(e, this);
    let request = e.mem.u32(s + 0x2058);
    if !e.call(REQUEST_BYTE_A2, &args![request]).bool() {
        return;
    }
    with_frame(e, 0xe8, |e, l| {
        // The middle POV.
        let pov = new_pov(e);
        e.mem.set_u32(l.at(0x24), pov);
        e.mem.set_u32(pov, 4);
        let half = pushed_float(e, HALF);
        e.call(POINT_ADD, &args![s + START, l.at(0x44), s + GOAL]);
        e.call(POINT_SCALE, &args![l.at(0x44), l.at(0x50), half]);
        copy3(e, pov + 4, l.at(0x50));
        finish_goal_pov(e, s, l.at(0x24), l.at(0x5c), D_0_4);

        // Direction and its perpendicular.
        e.call(POINT_SUB, &args![s + START, l.at(0x30), s + GOAL]);
        e.call(POINT_CROSS, &args![UP_AXIS, l.at(0x20), l.at(0x30)]);

        // goal + 0.2 x direction + 0.2 x perpendicular.
        let pov = new_pov(e);
        e.mem.set_u32(l.at(0x10), pov);
        e.mem.set_u32(pov, 4);
        let fifth = pushed_float(e, FIFTH);
        e.call(POINT_SCALE, &args![l.at(0x20), l.at(0x88), fifth]);
        let fifth = pushed_float(e, FIFTH);
        e.call(POINT_SCALE, &args![l.at(0x30), l.at(0x70), fifth]);
        e.call(POINT_ADD, &args![s + GOAL, l.at(0x7c), l.at(0x70)]);
        e.call(POINT_ADD, &args![l.at(0x7c), l.at(0x94), l.at(0x88)]);
        copy3(e, pov + 4, l.at(0x94));
        finish_goal_pov(e, s, l.at(0x10), l.at(0xa0), D_0_25);

        // goal + 0.2 x direction - 0.2 x perpendicular.
        let pov = new_pov(e);
        e.mem.set_u32(l.at(0x14), pov);
        e.mem.set_u32(pov, 4);
        let fifth = pushed_float(e, FIFTH);
        e.call(POINT_SCALE, &args![l.at(0x20), l.at(0xcc), fifth]);
        let fifth = pushed_float(e, FIFTH);
        e.call(POINT_SCALE, &args![l.at(0x30), l.at(0xb4), fifth]);
        e.call(POINT_ADD, &args![s + GOAL, l.at(0xc0), l.at(0xb4)]);
        e.call(POINT_SUB, &args![l.at(0xc0), l.at(0xd8), l.at(0xcc)]);
        copy3(e, pov + 4, l.at(0xd8));
        finish_goal_pov(e, s, l.at(0x14), l.at(0xe4), D_0_25);
    });
}

// Translated from 006ae120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `PathSmootherPOV` constructor: type -1,
/// center = the zero point, radius 0, the edge handle and both edge points
/// constructed (`006240d0`, `006815c0`), cost factor 1. Returns `this`.
pub fn fn_006ae120(e: &mut Engine, this: Ptr<PathSmootherPOV>) -> Ptr<PathSmootherPOV> {
    let p = this.addr();
    e.mem.set_u32(p, 0xffff_ffff);
    copy3(e, p + 4, ZERO_POINT);
    e.mem.set_f32(p + 0x10, 0.0);
    e.call(EDGE_HANDLE_CONSTRUCT, &args![p + POV_EDGE_HANDLE]);
    e.call(SELF, &args![p + POV_EDGE1]);
    e.call(SELF, &args![p + POV_EDGE2]);
    e.mem.set_f32(p + 0x38, 1.0);
    this
}

/// `006ae190` / `006ae280`: a new POV of `kind` at the point `s + point`
/// (radius 0, `iVertexIndex` 0xffff) stored at `s + slot` and added to
/// `allPOVs`.
fn end_pov(e: &mut Engine, s: u32, slot: u32, kind: u32, point: u32) {
    let pov = new_pov(e);
    e.mem.set_u32(s + slot, pov);
    let pov = e.mem.u32(s + slot);
    e.mem.set_u32(pov, kind);
    let pov = e.mem.u32(s + slot);
    copy3(e, pov + 4, s + point);
    let pov = e.mem.u32(s + slot);
    e.mem.set_f32(pov + 0x10, 0.0);
    let pov = e.mem.u32(s + slot);
    e.mem.set_u16(pov + 0x48, 0xffff);
    e.call(ARRAY_APPEND_WORD, &args![s + ALL_POVS, s + slot]);
}

// Translated from 006ae190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: creates the start POV (`pStartPov`, type 2,
/// at the start point) and adds it to `allPOVs`.
// Translation incomplete: 006ae190 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn fn_006ae190(e: &mut Engine, this: Ptr<PathSmootherPOVSearch>) {
    end_pov(e, this.addr(), 0x20c4, 2, START);
}

// Translated from 006ae280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: creates the goal POV (`pGoalPov`, type 3, at
/// the goal point) and adds it to `allPOVs`.
// Translation incomplete: 006ae280 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn fn_006ae280(e: &mut Engine, this: Ptr<PathSmootherPOVSearch>) {
    end_pov(e, this.addr(), 0x20c8, 3, GOAL);
}

/// The edge array element `index` (`006a7af0` on `Edges`): its navmesh word
/// (+0), triangle `u16` (+4) and edge word (+8), each read through a fresh
/// element call as the game does.
fn edge_navmesh(e: &mut Engine, edges: u32, index: u32) -> u32 {
    let element = e.call(EDGE_ARRAY_AT, &args![edges, index]).u32();
    e.mem.u32(element)
}

fn edge_triangle(e: &mut Engine, edges: u32, index: u32) -> u32 {
    let element = e.call(EDGE_ARRAY_AT, &args![edges, index]).u32();
    u32::from(e.mem.u16(element + 4))
}

fn edge_edge(e: &mut Engine, edges: u32, index: u32) -> u32 {
    let element = e.call(EDGE_ARRAY_AT, &args![edges, index]).u32();
    e.mem.u32(element + 8)
}

fn handles_equal(e: &mut Engine, a: u32, b: u32) -> bool {
    e.call(HANDLES_EQUAL, &args![a, b]).bool()
}

/// Whether `vert` names the last vertex of the side array at `s + side`
/// (`006b3140`, then `006d4ca0`).
fn equals_last(e: &mut Engine, s: u32, vert: u32, side: u32) -> bool {
    let last = fn_006b3140(e, Ptr::new(s + side)).addr();
    handles_equal(e, vert, last)
}

// Translated from 006ae370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: collects the corridor's edge vertices by
/// side. For every edge of `Edges` it builds the edge handle and the two
/// vertex handles. The first edge's vertices go left / right by the sign of
/// the z of `(v0 - v1) x (v0 - start)` (> 0: `v1` left, `v0` right); a later
/// edge adds the vertex that is new: one equal to the last right (left)
/// vertex puts the other on the left (right); when neither matches, the
/// edge shared with the previous triangle (`NavMesh::GetMatchingEdge`)
/// decides the same way. Then the corner POVs are made for every left and
/// right vertex (`006aeba0`), and each vertex POV's `iOtherSideIndex` is
/// set to the POV of the first later vertex on the other side that has one.
pub fn fn_006ae370(e: &mut Engine, this: Ptr<PathSmootherPOVSearch>) {
    let s = this.addr();
    with_frame(e, 0x90, |e, l| {
        e.call(ARRAY_CLEAR, &args![s + LEFT_VERTS, 0u32]);
        e.call(ARRAY_CLEAR, &args![s + RIGHT_VERTS, 0u32]);
        let mut i = 0u32;
        loop {
            let edges = e.mem.u32(s + 0x2080);
            if i >= size(e, edges) {
                break;
            }
            let edge = edge_edge(e, edges, i);
            let triangle = edge_triangle(e, edges, i);
            let navmesh = edge_navmesh(e, edges, i);
            e.call(EDGE_HANDLE_SET, &args![l.at(0x18), navmesh, triangle, edge]);
            let edge = edge_edge(e, edges, i);
            let triangle = edge_triangle(e, edges, i);
            let navmesh = edge_navmesh(e, edges, i);
            e.call(
                NAVMESH_EDGE_VERTEX_INDEXES,
                &args![navmesh, l.at(0x24), triangle, edge],
            );
            let first = u32::from(e.mem.u16(l.at(0x24)));
            let navmesh = edge_navmesh(e, edges, i);
            e.call(VERT_HANDLE_SET, &args![l.at(0xc), navmesh, first]);
            let second = u32::from(e.mem.u16(l.at(0x22)));
            let navmesh = edge_navmesh(e, edges, i);
            e.call(VERT_HANDLE_SET, &args![l.at(0x20), navmesh, second]);
            let (v0, v1, edge_handle) = (l.at(0xc), l.at(0x20), l.at(0x18));
            if i == 0 {
                let mesh1 = e.mem.u32(v1);
                let index1 = u32::from(e.mem.u16(v1 + 4));
                let p1 = e.call(NAVMESH_VERTEX, &args![mesh1, index1]).u32();
                let mesh0 = e.mem.u32(v0);
                let index0 = u32::from(e.mem.u16(v0 + 4));
                let p0 = e.call(NAVMESH_VERTEX, &args![mesh0, index0]).u32();
                e.call(POINT_SUB, &args![p0, l.at(0x30), p1]);
                let p0 = e.call(NAVMESH_VERTEX, &args![mesh0, index0]).u32();
                e.call(POINT_SUB, &args![p0, l.at(0x8c), s + START]);
                e.call(POINT_CROSS, &args![l.at(0x30), l.at(0x3c), l.at(0x8c)]);
                let z = f64::from(e.mem.f32(l.at(0x34)));
                if z > f64_at(e, D_ZERO) {
                    fn_006aea90(e, this, Ptr::new(v1), Ptr::new(edge_handle));
                    fn_006aeb00(e, this, Ptr::new(v0), Ptr::new(edge_handle));
                } else {
                    fn_006aea90(e, this, Ptr::new(v0), Ptr::new(edge_handle));
                    fn_006aeb00(e, this, Ptr::new(v1), Ptr::new(edge_handle));
                }
            } else {
                if equals_last(e, s, v0, LEFT_VERTS) {
                    fn_006aeb00(e, this, Ptr::new(v1), Ptr::new(edge_handle));
                } else if equals_last(e, s, v0, RIGHT_VERTS) {
                    fn_006aea90(e, this, Ptr::new(v1), Ptr::new(edge_handle));
                } else if equals_last(e, s, v1, LEFT_VERTS) {
                    fn_006aeb00(e, this, Ptr::new(v0), Ptr::new(edge_handle));
                } else if equals_last(e, s, v1, RIGHT_VERTS) {
                    fn_006aea90(e, this, Ptr::new(v0), Ptr::new(edge_handle));
                } else {
                    // The edge shared with the previous triangle.
                    e.mem.set_u32(l.at(0x68), 0);
                    e.mem.set_u16(l.at(0x44), 0xffff);
                    e.mem.set_u32(l.at(0x54), 0xffff_ffff);
                    let edge = edge_edge(e, edges, i - 1);
                    let triangles = e.mem.u32(s + 0x207c);
                    let element = e.call(TRIANGLE_ARRAY_AT, &args![triangles, i - 1]).u32();
                    let triangle = u32::from(e.mem.u16(element + 4));
                    let element = e.call(TRIANGLE_ARRAY_AT, &args![triangles, i - 1]).u32();
                    let navmesh = e.mem.u32(element);
                    e.call(
                        NAVMESH_GET_MATCHING_EDGE,
                        &args![navmesh, triangle, edge, l.at(0x68), l.at(0x44), l.at(0x54)],
                    );
                    let mesh = e.mem.u32(l.at(0x68));
                    let tri = u32::from(e.mem.u16(l.at(0x44)));
                    let shared_edge = e.mem.u32(l.at(0x54));
                    e.call(EDGE_HANDLE_SET, &args![l.at(0x50), mesh, tri, shared_edge]);
                    e.call(
                        NAVMESH_EDGE_VERTEX_INDEXES,
                        &args![mesh, l.at(0x40), tri, shared_edge],
                    );
                    let first = u32::from(e.mem.u16(l.at(0x40)));
                    let navmesh = edge_navmesh(e, edges, i);
                    e.call(VERT_HANDLE_SET, &args![l.at(0x64), navmesh, first]);
                    let second = u32::from(e.mem.u16(l.at(0x3e)));
                    let navmesh = edge_navmesh(e, edges, i);
                    e.call(VERT_HANDLE_SET, &args![l.at(0x5c), navmesh, second]);
                    let shared = Ptr::new(l.at(0x50));
                    if handles_equal(e, v0, l.at(0x64)) {
                        fn_006aeb00(e, this, Ptr::new(v1), shared);
                    } else if handles_equal(e, v0, l.at(0x5c)) {
                        fn_006aea90(e, this, Ptr::new(v1), shared);
                    } else if handles_equal(e, v1, l.at(0x64)) {
                        fn_006aeb00(e, this, Ptr::new(v0), shared);
                    } else if handles_equal(e, v1, l.at(0x5c)) {
                        fn_006aea90(e, this, Ptr::new(v0), shared);
                    }
                }
            }
            i += 1;
        }

        let mut i = 0u32;
        while i < size(e, s + LEFT_VERTS) {
            fn_006aeba0(e, this, Ptr::new(s + LEFT_VERTS), i, 1);
            i += 1;
        }
        let mut i = 0u32;
        while i < size(e, s + RIGHT_VERTS) {
            fn_006aeba0(e, this, Ptr::new(s + RIGHT_VERTS), i, 0);
            i += 1;
        }
        link_other_side(e, s, LEFT_VERTS, RIGHT_VERTS, LEFT_POVS);
        link_other_side(e, s, RIGHT_VERTS, LEFT_VERTS, RIGHT_POVS);
    });
}

/// The two last loops of `006ae370`: for every vertex of `side` with a POV,
/// the first vertex of `other` from its `iOtherSideIndex` on that has a POV
/// gives the POV's `iOtherSideIndex` (the POV is element `iPOVIndex` of
/// `povs`).
fn link_other_side(e: &mut Engine, s: u32, side: u32, other: u32, povs: u32) {
    let mut i = 0u32;
    while i < size(e, s + side) {
        let vert = fn_006b3120(e, Ptr::new(s + side), i).addr();
        if e.mem.u32(vert + 0x1c) != 0xffff_ffff {
            let vert = fn_006b3120(e, Ptr::new(s + side), i).addr();
            let mut j = e.mem.u32(vert + 0x14);
            while j < size(e, s + other) {
                let candidate = fn_006b3120(e, Ptr::new(s + other), j).addr();
                if e.mem.u32(candidate + 0x1c) != 0xffff_ffff {
                    let candidate = fn_006b3120(e, Ptr::new(s + other), j).addr();
                    let vert = fn_006b3120(e, Ptr::new(s + side), i).addr();
                    let index = e.mem.u32(vert + 0x1c);
                    let slot = e.call(ARRAY_WORD_AT, &args![s + povs, index]).u32();
                    let pov = e.mem.u32(slot);
                    let value = e.mem.u32(candidate + 0x1c);
                    e.mem.set_u32(pov + 0x40, value);
                    break;
                }
                j += 1;
            }
        }
        i += 1;
    }
}

/// `006aea90` / `006aeb00`: a `PathEdgeVert` for `vert` and `edge`, with
/// `iOtherSideIndex` = the other array's size, `iThisSideIndex` = this
/// array's size and `iPOVIndex` -1, added to `s + side` (`006b3730`).
fn add_edge_vert(e: &mut Engine, s: u32, side: u32, other: u32, vert: Ptr, edge: Ptr) {
    with_frame(e, 0x24, |e, l| {
        fn_006aeb70(e, Ptr::new(l.at(0x20)));
        e.call(VERT_HANDLE_COPY, &args![l.at(0x20), vert]);
        e.call(EDGE_HANDLE_COPY, &args![l.at(0x18), edge]);
        let other_size = size(e, s + other);
        e.mem.set_u32(l.at(0xc), other_size);
        let this_size = size(e, s + side);
        e.mem.set_u32(l.at(0x8), this_size);
        e.mem.set_u32(l.at(0x4), 0xffff_ffff);
        fn_006b3730(e, Ptr::new(s + side), Ptr::new(l.at(0x20)));
    });
}

// Translated from 006aea90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: adds a left `PathEdgeVert` (`LeftVerts`) for
/// `vert` and `edge`: `iOtherSideIndex` = `RightVerts`' size,
/// `iThisSideIndex` = `LeftVerts`' size, `iPOVIndex` -1.
pub fn fn_006aea90(e: &mut Engine, this: Ptr<PathSmootherPOVSearch>, vert: Ptr, edge: Ptr) {
    add_edge_vert(e, this.addr(), LEFT_VERTS, RIGHT_VERTS, vert, edge);
}

// Translated from 006aeb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the right-side twin of `006aea90`
/// (`RightVerts`, other side `LeftVerts`).
pub fn fn_006aeb00(e: &mut Engine, this: Ptr<PathSmootherPOVSearch>, vert: Ptr, edge: Ptr) {
    add_edge_vert(e, this.addr(), RIGHT_VERTS, LEFT_VERTS, vert, edge);
}

// Translated from 006aeb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (Ghidra matches the shape to a CRT
/// `SafeSQueue` constructor): the `PathEdgeVert` constructor: `006815c0` on
/// the vertex handle and `006240d0` on the edge handle (+8). Returns `this`.
pub fn fn_006aeb70(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(SELF, &args![this]);
    e.call(EDGE_HANDLE_CONSTRUCT, &args![this.addr() + 8]);
    this
}

/// A vertex position of `verts` element `index`: `0068f0a0` on its navmesh
/// and `u16` vertex.
fn vert_position(e: &mut Engine, verts: Ptr, index: u32) -> u32 {
    let element = fn_006b3120(e, verts, index).addr();
    let vertex = u32::from(e.mem.u16(element + 4));
    let element = fn_006b3120(e, verts, index).addr();
    let navmesh = e.mem.u32(element);
    e.call(NAVMESH_VERTEX, &args![navmesh, vertex]).u32()
}

// Translated from 006aeba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: decides whether vertex `index` of `verts` (a
/// side's `PathEdgeVert` array; `left` says which) is a corner and makes
/// its POV. `prev` = previous vertex (the start for the first) - vertex,
/// `next` = next vertex (the goal for the last) - vertex, both flattened
/// (z 0) and normalized; the turn is `prev x next` (left) or `next x prev`
/// (right). The first and last vertices are corners; others are when the
/// turn's z is under 0.05 (`next . prev` > 0) or under -0.05 (otherwise).
/// A corner gets a POV of type 0 (right) or 1 (left) at the vertex with the
/// vertex's edge handle, `Edge1` = prev, `Edge2` = next, cost factor 1,
/// radius = walker radius x 1.2, the vertex index, `iOtherSideIndex` -1,
/// `iThisSideIndex` = the side's POV count (also stored as the vertex's
/// `iPOVIndex`), added to `allPOVs` and the side's POV array.
// Translation incomplete: 006aeba0 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn fn_006aeba0(
    e: &mut Engine,
    this: Ptr<PathSmootherPOVSearch>,
    verts: Ptr,
    index: u32,
    left: u8,
) {
    let s = this.addr();
    with_frame(e, 0xa0, |e, l| {
        let p = vert_position(e, verts, index);
        copy3(e, l.at(0x18), p);
        e.call(SELF, &args![l.at(0x44)]);
        e.call(SELF, &args![l.at(0x38)]);
        if index == 0 {
            e.call(POINT_SUB, &args![s + START, l.at(0x54), l.at(0x18)]);
            copy3(e, l.at(0x44), l.at(0x54));
        } else {
            let p = vert_position(e, verts, index - 1);
            e.call(POINT_SUB, &args![p, l.at(0x60), l.at(0x18)]);
            copy3(e, l.at(0x44), l.at(0x60));
        }
        let last = size(e, verts.addr()).wrapping_sub(1);
        if index == last {
            e.call(POINT_SUB, &args![s + GOAL, l.at(0x6c), l.at(0x18)]);
            copy3(e, l.at(0x38), l.at(0x6c));
        } else {
            let p = vert_position(e, verts, index + 1);
            e.call(POINT_SUB, &args![p, l.at(0x78), l.at(0x18)]);
            copy3(e, l.at(0x38), l.at(0x78));
        }
        e.mem.set_f32(l.at(0x3c), 0.0);
        e.mem.set_f32(l.at(0x30), 0.0);
        e.call(POINT_NORMALIZE, &args![l.at(0x44)]);
        e.call(POINT_NORMALIZE, &args![l.at(0x38)]);
        let turn = if left != 0 {
            e.call(POINT_CROSS, &args![l.at(0x44), l.at(0x88), l.at(0x38)])
                .u32()
        } else {
            e.call(POINT_CROSS, &args![l.at(0x38), l.at(0x94), l.at(0x44)])
                .u32()
        };
        copy3(e, l.at(0x2c), turn);
        let dot = e.call(POINT_DOT, &args![l.at(0x38), l.at(0x44)]).f64() as f32;
        let turn_z = f64::from(e.mem.f32(l.at(0x24)));
        let corner = if index == 0 || index == size(e, verts.addr()).wrapping_sub(1) {
            true
        } else if f64::from(dot) > f64_at(e, D_ZERO) {
            turn_z < f64_at(e, D_0_05)
        } else {
            turn_z < f64_at(e, D_MINUS_0_05)
        };
        if !corner {
            return;
        }
        let pov = new_pov(e);
        e.mem.set_u32(l.at(0x48), pov);
        e.mem.set_u32(pov, u32::from(left == 0));
        let vert = fn_006b3120(e, verts, index).addr();
        e.call(EDGE_HANDLE_COPY, &args![pov + POV_EDGE_HANDLE, vert + 8]);
        copy3(e, pov + POV_EDGE1, l.at(0x44));
        copy3(e, pov + POV_EDGE2, l.at(0x38));
        copy3(e, pov + 4, l.at(0x18));
        set_cost_factor_one(e, pov);
        let radius = request_radius(e, s);
        e.mem
            .set_f32(pov + 0x10, (radius * f64_at(e, D_1_2)) as f32);
        let vert = fn_006b3120(e, verts, index).addr();
        let vertex = e.mem.u16(vert + 4);
        e.mem.set_u16(pov + 0x48, vertex);
        e.mem.set_u32(pov + 0x40, 0xffff_ffff);
        let count = if left != 0 {
            size(e, s + LEFT_POVS)
        } else {
            size(e, s + RIGHT_POVS)
        };
        e.mem.set_u32(pov + 0x3c, count);
        let vert = fn_006b3120(e, verts, index).addr();
        e.mem.set_u32(vert + 0x1c, count);
        append_pov(e, s, ALL_POVS, l.at(0x48));
        if left != 0 {
            append_pov(e, s, LEFT_POVS, l.at(0x48));
        } else {
            append_pov(e, s, RIGHT_POVS, l.at(0x48));
        }
    });
}

/// `006aefc0`'s POV for one obstacle: `kind`, the obstacle's position
/// (`006815c0` on it), cost factor `cost`, radius = the obstacle's radius
/// (`004a7bd0`, times `scale` when given) + walker radius x 1.2,
/// `iOtherSideIndex` -1, `iAvoidNodeIndex` = `index`; added to `allPOVs` and
/// `AvoidNodePOVs`.
#[allow(clippy::too_many_arguments)]
fn obstacle_pov(
    e: &mut Engine,
    s: u32,
    slot: u32,
    obstacle: u32,
    kind: u32,
    cost: f32,
    scale: Option<u32>,
    index: u32,
) {
    let pov = new_pov(e);
    e.mem.set_u32(slot, pov);
    e.mem.set_u32(pov, kind);
    zero_edges(e, pov);
    let position = e.call(SELF, &args![obstacle]).u32();
    copy3(e, pov + 4, position);
    e.mem.set_f32(pov + 0x38, cost);
    let mut radius = e.call(FLOAT_18, &args![obstacle]).f64();
    if let Some(factor) = scale {
        radius *= f64_at(e, factor);
    }
    let walker = request_radius(e, s);
    e.mem
        .set_f32(pov + 0x10, (walker * f64_at(e, D_1_2) + radius) as f32);
    e.mem.set_u32(pov + 0x40, 0xffff_ffff);
    e.mem.set_u32(pov + 0x44, index);
    append_pov(e, s, ALL_POVS, slot);
    append_pov(e, s, AVOID_NODE_POVS, slot);
}

// Translated from 006aefc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: POVs for the request's obstacles (`0069e390`,
/// held through `006b37b0` / `006b37e0`): for each obstacle a type-4 POV
/// (cost 1, radius = its radius + walker radius x 1.2); an obstacle whose
/// word at +0x20 is 2 also gets a type-5 POV (cost 10, radius = 0.25 x its
/// radius + walker radius x 1.2).
// Translation incomplete: 006aefc0 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn fn_006aefc0(e: &mut Engine, this: Ptr<PathSmootherPOVSearch>) {
    let s = this.addr();
    with_frame(e, 0x4c, |e, l| {
        let request = e.mem.u32(s + 0x2058);
        let obstacles = e.call(REQUEST_OBSTACLES, &args![request]).u32();
        fn_006b37b0(e, Ptr::new(l.at(0x10)), obstacles);
        if e.call(WORD_0, &args![l.at(0x10)]).u32() != 0 {
            let mut i = 0u32;
            loop {
                let list = e.call(WORD_0, &args![l.at(0x10)]).u32();
                if i >= size(e, list) {
                    break;
                }
                let list = e.call(WORD_0, &args![l.at(0x10)]).u32();
                let obstacle = e.call(OBSTACLE_AT, &args![list, i]).u32();
                obstacle_pov(e, s, l.at(0x18), obstacle, 4, 1.0, None, i);
                if e.call(WORD_20, &args![obstacle]).u32() == 2 {
                    let cost = x87_quiet(e.mem.f32(TEN));
                    obstacle_pov(e, s, l.at(0x20), obstacle, 5, cost, Some(D_0_25), i);
                }
                i += 1;
            }
        }
        fn_006b37e0(e, Ptr::new(l.at(0x10)));
    });
}

// Translated from 006af2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: POVs for the static avoid nodes of every
/// navmesh the corridor crosses. The distinct navmeshes of `Triangles` are
/// collected in a local `BSSimpleArray<NavMesh*>` (`00719b20` with compare
/// `009a3830`, then `007cb2e0`); every avoid node of each gets a type-4 POV
/// at its position (cost 1, radius = its radius + walker radius x 1.2,
/// `iAvoidNodeIndex` = mesh index x 1000 + node index).
// Translation incomplete: 006af2b0 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn fn_006af2b0(e: &mut Engine, this: Ptr<PathSmootherPOVSearch>) {
    let s = this.addr();
    with_frame(e, 0x48, |e, l| {
        let meshes = l.at(0x1c);
        fn_006b3800(e, Ptr::new(meshes));
        let mut i = 0u32;
        loop {
            let triangles = e.mem.u32(s + 0x207c);
            if i >= size(e, triangles) {
                break;
            }
            let element = e.call(TRIANGLE_ARRAY_AT, &args![triangles, i]).u32();
            let found = e
                .call(ARRAY_FIND, &args![meshes, element, 0u32, FIND_COMPARE])
                .u32();
            if found == 0xffff_ffff {
                let element = e.call(TRIANGLE_ARRAY_AT, &args![triangles, i]).u32();
                e.call(ARRAY_APPEND_WORD, &args![meshes, element]);
            }
            i += 1;
        }
        let mut i = 0u32;
        while i < size(e, meshes) {
            let slot = e.call(ARRAY_WORD_AT, &args![meshes, i]).u32();
            let navmesh = e.mem.u32(slot);
            let mut j = 0u32;
            while j < e.call(NAVMESH_AVOID_NODE_COUNT, &args![navmesh]).u32() {
                let pov = new_pov(e);
                e.mem.set_u32(l.at(0x30), pov);
                e.mem.set_u32(pov, 4);
                zero_edges(e, pov);
                let node = e.call(NAVMESH_AVOID_NODE, &args![navmesh, j]).u32();
                let position = e.call(SELF, &args![node]).u32();
                copy3(e, pov + 4, position);
                set_cost_factor_one(e, pov);
                let node = e.call(NAVMESH_AVOID_NODE, &args![navmesh, j]).u32();
                let radius = e.call(FLOAT_18, &args![node]).f64();
                let walker = request_radius(e, s);
                e.mem
                    .set_f32(pov + 0x10, (walker * f64_at(e, D_1_2) + radius) as f32);
                e.mem.set_u32(pov + 0x40, 0xffff_ffff);
                e.mem
                    .set_u32(pov + 0x44, i.wrapping_mul(1000).wrapping_add(j));
                append_pov(e, s, ALL_POVS, l.at(0x30));
                append_pov(e, s, AVOID_NODE_POVS, l.at(0x30));
                j += 1;
            }
            i += 1;
        }
        e.call(0x006b_3830, &args![meshes]);
    });
}

/// `006af500`'s POV beside an end point: type 4 at `point` +/- (`add`)
/// `side` x walker radius x 1.01, cost 1, radius = walker radius,
/// `iOtherSideIndex` -1; added to `allPOVs` and `AvoidNodePOVs`.
#[allow(clippy::too_many_arguments)]
fn side_pov(
    e: &mut Engine,
    s: u32,
    l: Frame,
    slot: u32,
    point: u32,
    side: u32,
    temps: [u32; 3],
    add: bool,
) {
    let pov = new_pov(e);
    e.mem.set_u32(l.at(slot), pov);
    e.mem.set_u32(pov, 4);
    zero_edges(e, pov);
    let factor = pushed_float(e, OFFSET_FACTOR);
    let walker = request_radius(e, s) as f32;
    let scaled = e
        .call(POINT_SCALE, &args![side, l.at(temps[0]), walker])
        .u32();
    let scaled = e
        .call(POINT_SCALE, &args![scaled, l.at(temps[1]), factor])
        .u32();
    let op = if add { POINT_ADD } else { POINT_SUB };
    let center = e.call(op, &args![point, l.at(temps[2]), scaled]).u32();
    copy3(e, pov + 4, center);
    set_cost_factor_one(e, pov);
    let walker = request_radius(e, s);
    e.mem.set_f32(pov + 0x10, walker as f32);
    e.mem.set_u32(pov + 0x40, 0xffff_ffff);
    append_pov(e, s, ALL_POVS, l.at(slot));
    append_pov(e, s, AVOID_NODE_POVS, l.at(slot));
}

// Translated from 006af500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: POVs beside the start and the goal. For each
/// end a `PathingLocation` is built on its first (last) triangle's navmesh
/// info (`006dd010`) and `006d4570` looks for a point within the walker
/// radius; when it finds one, two type-4 POVs are placed at the end point
/// +/- `unit(up x (end - found point))` x walker radius x 1.01.
// Translation incomplete: 006af500 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn fn_006af500(e: &mut Engine, this: Ptr<PathSmootherPOVSearch>) {
    let s = this.addr();
    with_frame(e, 0x184, |e, l| {
        // Start.
        e.call(LOCATION_RECORD_CONSTRUCT, &args![l.at(0x3c)]);
        let triangles = Ptr::new(e.mem.u32(s + 0x207c));
        let first = fn_006b3180(e, triangles).addr();
        let triangle = u32::from(e.mem.u16(first + 4));
        let first = fn_006b3180(e, triangles).addr();
        let navmesh = e.mem.u32(first);
        let info = e.call(NAVMESH_INFO, &args![navmesh]).u32();
        e.call(
            PATHING_LOCATION_CONSTRUCT,
            &args![l.at(0x8c), s + START, info, triangle],
        );
        let walker = request_radius(e, s) as f32;
        if e.call(FIND_POINT_NEAR, &args![l.at(0x8c), walker, l.at(0x3c)])
            .bool()
        {
            e.call(POINT_SUB, &args![s + START, l.at(0x98), l.at(0x30)]);
            e.call(POINT_UNIT_CROSS, &args![UP_AXIS, l.at(0xa4), l.at(0x98)]);
            side_pov(
                e,
                s,
                l,
                0xa8,
                s + START,
                l.at(0xa4),
                [0xe0, 0xec, 0xf8],
                true,
            );
            side_pov(
                e,
                s,
                l,
                0xac,
                s + START,
                l.at(0xa4),
                [0x10c, 0x118, 0x124],
                false,
            );
        }
        // Goal.
        e.call(LOCATION_RECORD_CONSTRUCT, &args![l.at(0x24)]);
        let triangles = Ptr::new(e.mem.u32(s + 0x207c));
        let last = fn_006b31a0(e, triangles).addr();
        let triangle = u32::from(e.mem.u16(last + 4));
        let last = fn_006b31a0(e, triangles).addr();
        let navmesh = e.mem.u32(last);
        let info = e.call(NAVMESH_INFO, &args![navmesh]).u32();
        e.call(
            PATHING_LOCATION_CONSTRUCT,
            &args![l.at(0x64), s + GOAL, info, triangle],
        );
        let walker = request_radius(e, s) as f32;
        if e.call(FIND_POINT_NEAR, &args![l.at(0x64), walker, l.at(0x24)])
            .bool()
        {
            e.call(POINT_SUB, &args![s + GOAL, l.at(0xb8), l.at(0x18)]);
            e.call(POINT_UNIT_CROSS, &args![UP_AXIS, l.at(0xc4), l.at(0xb8)]);
            side_pov(
                e,
                s,
                l,
                0xc8,
                s + GOAL,
                l.at(0xc4),
                [0x138, 0x144, 0x150],
                true,
            );
            side_pov(
                e,
                s,
                l,
                0xcc,
                s + GOAL,
                l.at(0xc4),
                [0x164, 0x170, 0x17c],
                false,
            );
        }
        e.call(PATHING_LOCATION_DESTRUCT, &args![l.at(0x64)]);
        e.call(PATHING_LOCATION_DESTRUCT, &args![l.at(0x8c)]);
    });
}

/// `00439180` with a float pushed through `FLD`/`FSTP`.
fn scale_assign(e: &mut Engine, point: u32, factor: f32) {
    e.call(POINT_SCALE_ASSIGN, &args![point, x87_quiet(factor)]);
}

/// `00408840(float)` (cdecl, ST0).
fn float_00408840(e: &mut Engine, value: f32) -> f64 {
    e.call(0x0040_8840, &args![x87_quiet(value)]).f64()
}

/// The angle between two link angles (`00698620(this, out, other)`, then
/// `006a0840` on `out`, ST0 stored as a `double`).
fn angle_between(e: &mut Engine, this: u32, out: u32, other: u32) -> f64 {
    e.call(0x0069_8620, &args![this, out, other]);
    e.call(0x006a_0840, &args![out]).f64()
}

/// The POV radius of node `index` of `Nodes` (`006a1440`, `pPOV->fRadius`).
fn node_radius(e: &mut Engine, s: u32, index: u32) -> f32 {
    let node = e.call(0x006a_1440, &args![s + NODES, index]).u32();
    let pov = e.mem.u32(node);
    e.mem.f32(pov + 0x10)
}

/// The water clamp of `006afcc0` for a link location: the water height at
/// `link + point` in the cell of the navmesh at `link + mesh`; a point not
/// above it is raised to it.
fn clamp_to_water(e: &mut Engine, s: u32, index: u32, mesh: u32, point: u32) {
    let link = fn_006b30e0(e, Ptr::new(s + LINKS), index).addr();
    let navmesh = e.mem.u32(link + mesh);
    let cell = e.call(NAVMESH_CELL, &args![navmesh]).u32();
    let link = fn_006b30e0(e, Ptr::new(s + LINKS), index).addr();
    let tes = e.mem.u32(TES_SINGLETON);
    let water = e.call(WATER_HEIGHT, &args![tes, link + point, cell]).f64() as f32;
    let link = fn_006b30e0(e, Ptr::new(s + LINKS), index).addr();
    let z = f64::from(e.mem.f32(link + point + 8));
    if z <= f64::from(water) {
        let link = fn_006b30e0(e, Ptr::new(s + LINKS), index).addr();
        e.mem.set_f32(link + point + 8, x87_quiet(water));
    }
}

/// The offset of a solution point in `006afcc0`: `offset` (from
/// `006b0a60`) is scaled by `radius_scale` (or, with `angle`, by
/// `00408840(radius x angle)`), then by 3, and is cut to `length` when it
/// is longer.
fn scale_offset(e: &mut Engine, offset: u32, radius: f32, angle: Option<f64>, length: f32) {
    match angle {
        Some(angle) => {
            let product = (f64::from(radius) * angle) as f32;
            let factor = float_00408840(e, product) as f32;
            scale_assign(e, offset, factor);
        }
        None => scale_assign(e, offset, radius),
    }
    let three = e.mem.f32(0x0101_7718);
    scale_assign(e, offset, three);
    let squared = e.call(POINT_LENGTH_SQUARED, &args![offset]).f64();
    if f64::from(length) * f64::from(length) < squared {
        e.call(POINT_NORMALIZE, &args![offset]);
        scale_assign(e, offset, length);
    }
}

// Translated from 006afcc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: writes the smoothed path into the solution.
/// For every link of `Links` (exit location +0x10, entry location +0x28):
/// unless the turn into it is a small arc (radius x angle under 50 and
/// `006a0660` true on the angle range -pi/2..pi/2), the exit point is added
/// with an offset along the exit direction (`006b0a60`); on an under-water
/// path the exit (for later links) and entry points are raised to the
/// water (the last entry only when the solution's byte at +0x40 or
/// `retry_failed` is set). The segment between the two is walked across
/// navmesh boundaries (`006b05f0`) and the entry point is added the same
/// way. Offsets are radius (x `00408840(radius x angle)` between links)
/// x 3, cut to the segment's length. At the end, when the request's byte at
/// +0x99 is set and there are links, the solution's last node gets a
/// facing offset (`006b0570`) of the walker's xy length (`00589850` of the
/// last node's +0x2c point) in the request's heading (+0x8c), when that
/// heading is within pi/2 of the direction to the point.
// Translation incomplete: 006afcc0 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn fn_006afcc0(e: &mut Engine, this: Ptr<PathSmootherPOVSearch>, retry_failed: u8) {
    let s = this.addr();
    with_frame(e, 0x124, |e, l| {
        let links = Ptr::new(s + LINKS);
        let mut i = 0u32;
        while i < size(e, s + LINKS) {
            let exit = fn_006b30e0(e, links, i).addr() + 0x1c;
            let entry = fn_006b30e0(e, links, i).addr() + 0x34;
            e.call(POINT_SUB, &args![entry, l.at(0x58), exit]);
            let length = e.call(POINT_LENGTH, &args![l.at(0x58)]).f64() as f32;
            let mut add_exit = true;
            if i > 0 {
                let previous = fn_006b30e0(e, links, i - 1).addr() + 8;
                let link = fn_006b30e0(e, links, i).addr();
                e.call(0x0069_8620, &args![link, l.at(0x60), previous]);
                let half_pi = pushed_float(e, 0x0101_ff38);
                let upper = e.call(0x006a_04e0, &args![l.at(0xf8), half_pi]).u32();
                let minus_half_pi = pushed_float(e, 0x0101_6b78);
                let lower = e
                    .call(0x006a_04e0, &args![l.at(0x100), minus_half_pi])
                    .u32();
                e.call(0x006a_05e0, &args![l.at(0x74), lower, upper, 0u32]);
                let angle = e.call(0x006a_0840, &args![l.at(0x60)]).f64();
                let radius = node_radius(e, s, i);
                if f64::from(radius) * angle < f64_at(e, D_50)
                    && e.call(0x006a_0660, &args![l.at(0x74), l.at(0x60)]).bool()
                {
                    add_exit = false;
                }
                if e.mem.u8(s + 0x2078) != 0 {
                    clamp_to_water(e, s, i, 0x10, 0x1c);
                }
            }
            if add_exit {
                let link = fn_006b30e0(e, links, i).addr();
                let navmesh = e.mem.u32(link + 0x10);
                let info = e.call(NAVMESH_INFO, &args![navmesh]).u32();
                let link = fn_006b30e0(e, links, i).addr();
                e.call(0x006d_cf20, &args![l.at(0xac), link + 0x1c, info]);
                let node = e.call(0x006a_1440, &args![s + NODES, i]).u32();
                let direction = e.mem.u32(node + 4);
                let link = fn_006b30e0(e, links, i);
                fn_006b0a60(e, this, Ptr::new(l.at(0x84)), link, direction);
                if i > 0 {
                    let previous = fn_006b30e0(e, links, i - 1).addr() + 8;
                    let link = fn_006b30e0(e, links, i).addr();
                    let angle = angle_between(e, link, l.at(0xb4), previous);
                    let radius = node_radius(e, s, i);
                    scale_offset(e, l.at(0x84), radius, Some(angle), length);
                } else {
                    let radius = node_radius(e, s, i);
                    scale_offset(e, l.at(0x84), radius, None, length);
                }
                let solution = e.mem.u32(s + 0x205c);
                e.call(SOLUTION_ADD, &args![solution, l.at(0xac), l.at(0x84)]);
                e.call(PATHING_LOCATION_DESTRUCT, &args![l.at(0xac)]);
            }
            if e.mem.u8(s + 0x2078) != 0 {
                let mut clamp = i < size(e, s + LINKS).wrapping_sub(1);
                if !clamp {
                    let solution = e.mem.u32(s + 0x205c);
                    clamp = fn_006b05d0(e, Ptr::new(solution)) != 0 || retry_failed != 0;
                }
                if clamp {
                    clamp_to_water(e, s, i, 0x28, 0x34);
                }
            }
            let link = fn_006b30e0(e, links, i).addr();
            let navmesh = e.mem.u32(link + 0x28);
            let info = e.call(NAVMESH_INFO, &args![navmesh]).u32();
            let link = fn_006b30e0(e, links, i).addr();
            e.call(0x006d_cf20, &args![l.at(0x48), link + 0x34, info]);
            let node = e.call(0x006a_1440, &args![s + NODES, i + 1]).u32();
            let direction = e.mem.u32(node + 4);
            let link = fn_006b30e0(e, links, i).addr();
            fn_006b0a60(e, this, Ptr::new(l.at(0x1c)), Ptr::new(link + 8), direction);
            let to = fn_006b30e0(e, links, i).addr() + 0x34;
            let from = fn_006b30e0(e, links, i).addr() + 0x1c;
            let link = fn_006b30e0(e, links, i).addr();
            let triangle = e.mem.u16(link + 0x14);
            let link = fn_006b30e0(e, links, i).addr();
            let navmesh = e.mem.u32(link + 0x10);
            fn_006b05f0(e, this, navmesh, triangle, Ptr::new(from), Ptr::new(to));
            if i < size(e, s + LINKS).wrapping_sub(1) {
                let this_entry = fn_006b30e0(e, links, i).addr() + 8;
                let next = fn_006b30e0(e, links, i + 1).addr();
                let angle = angle_between(e, next, l.at(0xc4), this_entry);
                let radius = node_radius(e, s, i + 1);
                scale_offset(e, l.at(0x1c), radius, Some(angle), length);
            } else {
                let radius = node_radius(e, s, i + 1);
                scale_offset(e, l.at(0x1c), radius, None, length);
            }
            let solution = e.mem.u32(s + 0x205c);
            e.call(SOLUTION_ADD, &args![solution, l.at(0x48), l.at(0x1c)]);
            e.call(PATHING_LOCATION_DESTRUCT, &args![l.at(0x48)]);
            i += 1;
        }

        let request = e.mem.u32(s + 0x2058);
        if !e.call(0x0050_5c00, &args![request]).bool() {
            return;
        }
        if e.call(ARRAY_IS_EMPTY, &args![s + LINKS]).bool() {
            return;
        }
        let solution = e.mem.u32(s + 0x205c);
        let node = fn_006b05a0(e, Ptr::new(solution));
        let point = e.call(0x005d_8a70, &args![node]).u32();
        copy3(e, l.at(0xd8), point);
        let heading_to = e.call(0x004b_13c0, &args![l.at(0xd8)]).f64() as f32;
        let request = e.mem.u32(s + 0x2058);
        let heading = e.call(0x0050_7e10, &args![request]).f64() as f32;
        let difference = (f64::from(heading) - f64::from(heading_to)) as f32;
        e.mem.set_f32(l.at(0xdc), difference);
        let minus_pi = pushed_float(e, 0x0106_b648);
        let pi = pushed_float(e, 0x0102_b3c8);
        e.call(0x004e_44f0, &args![l.at(0xdc), minus_pi, pi]);
        let difference = e.mem.f32(l.at(0xdc));
        let size_of_difference = float_00408840(e, difference);
        if !less(size_of_difference, f64_at(e, 0x0106_4f50)) {
            return;
        }
        let reach = e.call(0x0058_9850, &args![l.at(0xd8)]).f64() as f32;
        e.call(SELF, &args![l.at(0xec)]);
        let cos = e.call(0x004e_4470, &args![x87_quiet(heading)]).f64();
        e.mem.set_f32(l.at(0xec), (cos * f64::from(reach)) as f32);
        let sin = e.call(0x004e_44b0, &args![x87_quiet(heading)]).f64();
        e.mem.set_f32(l.at(0xe8), (sin * f64::from(reach)) as f32);
        e.mem.set_f32(l.at(0xe4), 0.0);
        let solution = e.mem.u32(s + 0x205c);
        let node = fn_006b05a0(e, Ptr::new(solution));
        fn_006b0570(e, node, Ptr::new(l.at(0xec)));
    });
}

// Translated from 006b0570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: copies the three words at `point` to the
/// object's +0x2c (the point `005d8a70` returns).
pub fn fn_006b0570(e: &mut Engine, this: Ptr, point: Ptr) {
    copy3(e, this.addr() + 0x2c, point.addr());
}

// Translated from 006b05a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the last element of the array at +0x20
/// (`006e8fe0`), or null when it is empty (`0076b610`).
pub fn fn_006b05a0(e: &mut Engine, this: Ptr) -> Ptr {
    let array = this.addr() + 0x20;
    if e.call(ARRAY_IS_EMPTY, &args![array]).bool() {
        Ptr::NULL
    } else {
        e.call(ARRAY_LAST, &args![array]).ptr()
    }
}

// Translated from 006b05d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the byte at +0x40.
pub fn fn_006b05d0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x40)
}

// Translated from 006b05f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: walks the segment `from`-`to` from triangle
/// `triangle` of `navmesh` across triangle edges (at most 1000 steps,
/// `00698320` gives the exit edge, `NavMesh::GetMatchingEdge` the next
/// triangle). Crossing into another navmesh, the walk point moves to the
/// shared edge (`006ba550`) plus 0.01 x (edge middle - point). Where the two
/// triangles' vectors (`006971e0`) have a dot product under 0.9, the
/// crossing point of the segment and the edge (`006ba100`) - when it is
/// more than 32 units (squared length 1024) from `from`, `to` and the
/// solution's last node point - is added to the solution (`0069f920`) with
/// the walker radius x the unit direction, raised to the water on an
/// under-water path.
// Translation incomplete: 006b05f0 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn fn_006b05f0(
    e: &mut Engine,
    this: Ptr<PathSmootherPOVSearch>,
    navmesh: u32,
    triangle: u16,
    from: Ptr,
    to: Ptr,
) {
    let s = this.addr();
    with_frame(e, 0x158, |e, l| {
        copy3(e, l.at(0x3c), from.addr());
        copy3(e, l.at(0x30), to.addr());
        e.call(POINT_SUB, &args![l.at(0x30), l.at(0x48), l.at(0x3c)]);
        e.call(POINT_NORMALIZE, &args![l.at(0x48)]);
        copy3(e, l.at(0x24), l.at(0x3c));
        e.call(
            EDGE_HANDLE_SET,
            &args![l.at(0x18), navmesh, u32::from(triangle), 0xffff_ffffu32],
        );
        let current = l.at(0x18);
        let mut steps = 0i32;
        while steps < 1000 {
            steps += 1;
            let mesh = e.mem.u32(current);
            let tri = u32::from(e.mem.u16(current + 4));
            let edge_word = e.mem.u32(current + 8);
            let exit = e
                .call(
                    NAVMESH_EXIT_EDGE,
                    &args![mesh, tri, edge_word, l.at(0x24), l.at(0x30)],
                )
                .u32();
            if exit == 0xffff_ffff {
                break;
            }
            let tri = u32::from(e.mem.u16(current + 4));
            let mesh = e.mem.u32(current);
            e.call(EDGE_HANDLE_SET, &args![l.at(0x5c), mesh, tri, exit]);
            e.call(EDGE_HANDLE_CONSTRUCT, &args![l.at(0x6c)]);
            let mesh = e.mem.u32(l.at(0x5c));
            let matched = e
                .call(
                    NAVMESH_GET_MATCHING_EDGE_HANDLE,
                    &args![mesh, l.at(0x5c), l.at(0x6c)],
                )
                .bool();
            if !matched {
                break;
            }
            if e.mem.u32(current) != e.mem.u32(l.at(0x6c)) {
                // Into another navmesh: move the walk point onto the edge.
                let edge_word = e.mem.u32(l.at(0x64));
                let tri = u32::from(e.mem.u16(l.at(0x68)));
                let mesh = e.mem.u32(l.at(0x6c));
                e.call(
                    NAVMESH_GET_EDGE_VERTICES,
                    &args![mesh, l.at(0x98), tri, edge_word],
                );
                let half = pushed_float(e, HALF);
                let a = e.mem.u32(l.at(0x98));
                let b = e.mem.u32(l.at(0x94));
                let sum = e.call(POINT_ADD, &args![a, l.at(0x100), b]).u32();
                e.call(POINT_SCALE, &args![sum, l.at(0x90), half]);
                let a = e.mem.u32(l.at(0x98));
                let b = e.mem.u32(l.at(0x94));
                let on_edge = e
                    .call(
                        SEGMENT_POINT,
                        &args![l.at(0x10c), a, b, l.at(0x3c), l.at(0x30)],
                    )
                    .u32();
                copy3(e, l.at(0x24), on_edge);
                let step = pushed_float(e, STEP_FACTOR);
                let towards = e
                    .call(POINT_SUB, &args![l.at(0x90), l.at(0x118), l.at(0x24)])
                    .u32();
                let towards = e
                    .call(POINT_SCALE, &args![towards, l.at(0x124), step])
                    .u32();
                e.call(POINT_ADD_ASSIGN, &args![l.at(0x24), towards]);
            }
            let tri = u32::from(e.mem.u16(l.at(0x58)));
            let mesh = e.mem.u32(l.at(0x5c));
            e.call(NAVMESH_TRIANGLE_VECTOR, &args![mesh, l.at(0x78), tri]);
            let tri = u32::from(e.mem.u16(l.at(0x68)));
            let mesh = e.mem.u32(l.at(0x6c));
            e.call(NAVMESH_TRIANGLE_VECTOR, &args![mesh, l.at(0x84), tri]);
            let dot = e.call(POINT_DOT, &args![l.at(0x78), l.at(0x84)]).f64();
            if dot < f64_at(e, D_0_9) {
                add_crossing(e, s, l);
            }
            e.call(EDGE_HANDLE_COPY, &args![current, l.at(0x6c)]);
        }
    });
}

/// The crossing point step of `006b05f0` (frame of `006b05f0`).
fn add_crossing(e: &mut Engine, s: u32, l: Frame) {
    e.call(SELF, &args![l.at(0xbc)]);
    e.call(SELF, &args![l.at(0xb0)]);
    let edge_word = e.mem.u32(l.at(0x54));
    let tri = u32::from(e.mem.u16(l.at(0x58)));
    let mesh = e.mem.u32(l.at(0x5c));
    e.call(
        NAVMESH_EDGE_ENDS,
        &args![mesh, tri, edge_word, l.at(0xbc), l.at(0xb0)],
    );
    e.call(SELF, &args![l.at(0xa4)]);
    let crossing = l.at(0xa4);
    if !e
        .call(
            SEGMENT_INTERSECT,
            &args![l.at(0xbc), l.at(0xb0), l.at(0x3c), l.at(0x30), crossing],
        )
        .bool()
    {
        return;
    }
    let limit = f64_at(e, D_1024);
    let d = e
        .call(POINT_SUB, &args![crossing, l.at(0x130), l.at(0x3c)])
        .u32();
    if !less(limit, e.call(POINT_LENGTH_SQUARED, &args![d]).f64()) {
        return;
    }
    let d = e
        .call(POINT_SUB, &args![crossing, l.at(0x13c), l.at(0x30)])
        .u32();
    if !less(limit, e.call(POINT_LENGTH_SQUARED, &args![d]).f64()) {
        return;
    }
    let solution = Ptr::new(e.mem.u32(s + 0x205c));
    if fn_006b05a0(e, solution).is_null() {
        return;
    }
    let node = fn_006b05a0(e, solution);
    let field = e.call(LAST_NODE_FIELD, &args![node]).u32();
    let point = e.call(NODE_POINT, &args![field, l.at(0x148)]).u32();
    let d = e
        .call(POINT_SUB, &args![crossing, l.at(0x154), point])
        .u32();
    if !less(limit, e.call(POINT_LENGTH_SQUARED, &args![d]).f64()) {
        return;
    }
    if e.mem.u8(s + 0x2078) != 0 {
        let node = fn_006b05a0(e, solution);
        let field = e.call(LAST_NODE_FIELD, &args![node]).u32();
        let cell = e.call(PATHING_LOCATION_GET_CELL, &args![field]).u32();
        let tes = e.mem.u32(TES_SINGLETON);
        let water = e.call(WATER_HEIGHT, &args![tes, crossing, cell]).f64() as f32;
        let z = f64::from(e.mem.f32(crossing + 8));
        if z <= f64::from(water) {
            e.mem.set_f32(crossing + 8, x87_quiet(water));
        }
    }
    let tri = u32::from(e.mem.u16(l.at(0x58)));
    let mesh = e.mem.u32(l.at(0x5c));
    let info = e.call(NAVMESH_INFO, &args![mesh]).u32();
    e.call(
        PATHING_LOCATION_CONSTRUCT,
        &args![l.at(0xf0), crossing, info, tri],
    );
    let walker = request_radius(e, s) as f32;
    e.call(POINT_SCALE, &args![l.at(0x48), l.at(0xc8), walker]);
    let solution = e.mem.u32(s + 0x205c);
    e.call(SOLUTION_ADD, &args![solution, l.at(0xf0), l.at(0xc8)]);
    e.call(PATHING_LOCATION_DESTRUCT, &args![l.at(0xf0)]);
}

// Translated from 006b0a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the side direction of an angle (`angle`: its
/// two floats): `(-y, x, 0)` when `direction` is 0, `(y, -x, 0)` otherwise,
/// written to `out`; returns `out`. (`this` is only kept.)
pub fn fn_006b0a60(
    e: &mut Engine,
    _this: Ptr<PathSmootherPOVSearch>,
    out: Ptr,
    angle: Ptr,
    direction: u32,
) -> Ptr {
    with_frame(e, 0x10, |e, l| {
        e.call(SELF, &args![l.at(0xc)]);
        let x = x87_quiet_bits(e.mem.u32(angle.addr()));
        let y = x87_quiet_bits(e.mem.u32(angle.addr() + 4));
        let (first, second) = if direction == 0 {
            (y ^ 0x8000_0000, x)
        } else {
            (y, x ^ 0x8000_0000)
        };
        e.mem.set_u32(out.addr(), first);
        e.mem.set_u32(out.addr() + 4, second);
        e.mem.set_f32(out.addr() + 8, 0.0);
    });
    out
}

/// One connection loop of `GetNodeConnections`: `006b0f40` for the POVs of
/// the array at `s + array` from `start` up to its size (read once); with
/// `limit`, the loop stops at the fourth failure.
fn connect_range(
    e: &mut Engine,
    this: Ptr<PathSmootherPOVSearch>,
    node: Ptr,
    connections: Ptr,
    array: u32,
    start: u32,
    limit: bool,
) {
    let s = this.addr();
    let count = size(e, s + array);
    let mut failures = 0;
    let mut i = start;
    while i < count {
        let slot = e.call(ARRAY_WORD_AT, &args![s + array, i]).u32();
        let pov = Ptr::new(e.mem.u32(slot));
        let ok = fn_006b0f40(e, this, node, pov, connections);
        if limit && !ok {
            failures += 1;
            if failures == 4 {
                break;
            }
        }
        i += 1;
    }
}

// Translated from 006b0ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathSmootherPOVSearch::GetNodeConnections` (Xbox PDB): the A* node's
/// successors, added to `connections` by `006b0f40`. The goal POV is tried
/// first. From a left corner (type 0) the later left corners
/// (`iThisSideIndex + 1` on) and the right ones from `iOtherSideIndex` on
/// are tried, each run stopping at its fourth failure; from a right corner
/// (type 1) the same with the sides swapped; from the start (type 2) all
/// right then all left corners, with the same limit; from any other POV
/// all right and left corners. Every avoid-node POV is tried last.
/// Returns true.
pub fn path_smoother_pov_search_get_node_connections(
    e: &mut Engine,
    this: Ptr<PathSmootherPOVSearch>,
    node: Ptr,
    connections: Ptr,
) -> bool {
    let s = this.addr();
    let data = e.call(NODE_DATA, &args![node]).u32();
    let goal = Ptr::new(e.mem.u32(s + 0x20c8));
    fn_006b0f40(e, this, node, goal, connections);
    let pov = e.mem.u32(data);
    match e.mem.u32(pov) {
        0 => {
            let from = e.mem.u32(pov + 0x3c).wrapping_add(1);
            connect_range(e, this, node, connections, LEFT_POVS, from, true);
            let pov = e.mem.u32(data);
            let from = e.mem.u32(pov + 0x40);
            connect_range(e, this, node, connections, RIGHT_POVS, from, true);
        }
        1 => {
            let from = e.mem.u32(pov + 0x3c).wrapping_add(1);
            connect_range(e, this, node, connections, RIGHT_POVS, from, true);
            let pov = e.mem.u32(data);
            let from = e.mem.u32(pov + 0x40);
            connect_range(e, this, node, connections, LEFT_POVS, from, true);
        }
        2 => {
            connect_range(e, this, node, connections, RIGHT_POVS, 0, true);
            connect_range(e, this, node, connections, LEFT_POVS, 0, true);
        }
        _ => {
            connect_range(e, this, node, connections, RIGHT_POVS, 0, false);
            connect_range(e, this, node, connections, LEFT_POVS, 0, false);
        }
    }
    connect_range(e, this, node, connections, AVOID_NODE_POVS, 0, false);
    true
}

/// A copy of the 0x14-byte arc record at `arc` made with its copy
/// constructor `006a0620` (the game builds it in the argument area of a
/// by-value parameter); `f` gets the copy's address.
fn with_arc_copy<R>(e: &mut Engine, arc: u32, f: impl FnOnce(&mut Engine, Ptr) -> R) -> R {
    e.with_stack(0x14, |e, copy| {
        e.call(0x006a_0620, &args![copy, arc]);
        f(e, copy)
    })
}

/// The tangent table entry `k` of `006b0f40`'s frame: its direction word
/// (`[EBP - 0x54 + k * 0x14]`) and angles.
fn tangent(l: Frame, k: u32) -> (u32, u32, u32) {
    let base = l.at(0x54) + k * 0x14;
    (base, base + 4, base + 0xc)
}

// Translated from 006b0f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: tries the connection from A* node `node` to
/// POV `target` and adds the reached nodes to `connections`. Nothing is
/// tried (true) for the node's own POV or its parent's, or, off an
/// under-water path, a step steeper than 1.2 (z^2 over 1.44 x the xy
/// length^2). The common tangents of the two circles (`006bb570`, the
/// node's direction) give up to two ways; from the start node inside the
/// target circle the two ways are the tangent points of the start's
/// position instead. Each way that leaves on the target's side (a corner is
/// passed on its own side) and, from a corner node, turns at most 200
/// degrees on the arc, is checked: the arc on the node's circle
/// (`006b1e50`, its end within 180 in z, `006b23d0` off water paths), the
/// straight segment (`TripleCheckSegmentAgainstNavMesh`), the reached
/// triangle height (within -64..180 of the target's center unless above
/// water) and the avoid nodes (`006b2c00`). The cost is the node's
/// (`006a7f50`) + (1 - 0.9 x preferred share) x target cost factor x (arc
/// length + segment cost); the node is opened or improved (`006b3420`,
/// `006b1df0`) and its parent link written. Returns false when every way
/// tried failed.
pub fn fn_006b0f40(
    e: &mut Engine,
    this: Ptr<PathSmootherPOVSearch>,
    node: Ptr,
    target: Ptr,
    connections: Ptr,
) -> bool {
    let s = this.addr();
    let target = target.addr();
    with_frame(e, 0x1a8, |e, l| {
        let data = e.call(NODE_DATA, &args![node]).u32();
        let link = e.call(0x007d_6bb0, &args![node]).u32();
        if target == e.mem.u32(data) {
            return true;
        }
        if e.call(0x006a_1b60, &args![node]).u32() != 0 {
            let parent = e.call(0x006a_1b60, &args![node]).u32();
            let parent_data = e.call(NODE_DATA, &args![parent]).u32();
            if target == e.mem.u32(parent_data) {
                return true;
            }
        }
        let pov = e.mem.u32(data);
        e.call(POINT_SUB, &args![target + 4, l.at(0x14), pov + 4]);
        let xy = e.call(0x0059_5c80, &args![l.at(0x14)]).f64() as f32;
        let z = f64::from(e.mem.f32(l.at(0xc)));
        let zz = (z * z) as f32;
        if e.mem.u8(s + 0x2078) == 0 {
            let limit = f64::from(xy) * f64_at(e, D_1_2) * f64_at(e, D_1_2);
            if f64::from(zz) > limit {
                return true;
            }
        }
        let pov = e.mem.u32(data);
        let radius = x87_quiet(e.mem.f32(pov + 0x10));
        e.call(0x006a_04b0, &args![l.at(0x24), pov + 4, radius]);
        let target_radius = x87_quiet(e.mem.f32(target + 0x10));
        e.call(0x006a_04b0, &args![l.at(0x68), target + 4, target_radius]);
        e.call(0x006a_0870, &args![l.at(0x58)]);
        let direction = e.mem.u32(data + 4);
        let mut count = e
            .call(
                0x006b_b570,
                &args![l.at(0x24), l.at(0x68), direction, l.at(0x58)],
            )
            .u32();
        if e.call(0x006a_1b60, &args![node]).u32() == 0 {
            let pov = e.mem.u32(data);
            e.call(POINT_SUB, &args![pov + 4, l.at(0x80), target + 4]);
            let squared = e.call(POINT_LENGTH_SQUARED, &args![l.at(0x80)]).f64();
            let r = f64::from(e.mem.f32(target + 0x10));
            if r * r > squared {
                start_inside_target(e, l, target);
                count = 2;
            }
        }
        if count == 0 {
            return true;
        }
        let mut added = 0i32;
        let mut failed = 0i32;
        let mut k = 0i32;
        while k < count as i32 {
            let ku = k as u32;
            k += 1;
            let (dir_slot, exit_angle, _) = tangent(l, ku);
            let way = e.mem.u32(dir_slot);
            let target_type = e.mem.u32(target);
            if (target_type == 0 && way == 1) || (target_type == 1 && way == 0) {
                continue;
            }
            let parent = e.call(0x006a_1b60, &args![node]).u32();
            let mut from_corner = false;
            if parent != 0 {
                let parent = e.call(0x006a_1b60, &args![node]).u32();
                let parent_pov = e.mem.u32(parent + 8);
                from_corner = e.mem.u32(parent_pov) == 0;
                if !from_corner {
                    let parent = e.call(0x006a_1b60, &args![node]).u32();
                    let parent_pov = e.mem.u32(parent + 8);
                    from_corner = e.mem.u32(parent_pov) == 1;
                }
            }
            if from_corner {
                let angle = angle_between(e, exit_angle, l.at(0x140), link + 8) as f32;
                if e.mem.u32(data + 4) == 0 {
                    let mut a = angle;
                    if f64::from(a) < f64_at(e, D_ZERO) {
                        a = (f64::from(a) + e.mem.f64(0x0101_ff48)) as f32;
                    }
                    if f64::from(a) > e.mem.f64(0x0106_ba00) {
                        continue;
                    }
                } else {
                    let mut a = angle;
                    if f64::from(a) > f64_at(e, D_ZERO) {
                        a = (f64::from(a) - e.mem.f64(0x0101_ff48)) as f32;
                    }
                    if f64::from(a) < e.mem.f64(0x0106_b9f8) {
                        continue;
                    }
                }
            }
            match try_way(
                e,
                this,
                l,
                node,
                data,
                link,
                target,
                ku,
                connections,
                &mut added,
            ) {
                Way::Failed => failed += 1,
                Way::Skipped | Way::Done => {}
            }
        }
        !(added != 0 && added <= failed)
    })
}

/// The result of one way of `006b0f40`.
enum Way {
    /// A check failed: counted against the ways tried.
    Failed,
    /// Left without counting (an avoid-node block, or a worse cost).
    Skipped,
    /// The node was opened or improved.
    Done,
}

/// `006b0f40` from the start node inside the target circle: the two ways
/// are the tangent points of the start's position on the target circle
/// (both the flattened direction when the start is within half the radius).
fn start_inside_target(e: &mut Engine, l: Frame, target: u32) {
    e.mem.set_f32(l.at(0x78), 0.0);
    let length = e.call(0x0045_7910, &args![l.at(0x80)]).f64() as f32;
    e.call(SELF, &args![l.at(0x8c)]);
    e.call(SELF, &args![l.at(0x98)]);
    let r = e.mem.f32(target + 0x10);
    if f64::from(length) < f64::from(r) * e.mem.f64(0x0101_1588) {
        copy3(e, l.at(0x8c), l.at(0x80));
        copy3(e, l.at(0x98), l.at(0x80));
    } else {
        let r = f64::from(e.mem.f32(target + 0x10));
        let rest = (r * r - f64::from(length) * f64::from(length)) as f32;
        let root = e.call(0x0040_19b0, &args![x87_quiet(rest)]).f64() as f32;
        let x = x87_quiet_bits(e.mem.u32(l.at(0x80)));
        let minus_y = x87_quiet_bits(e.mem.u32(l.at(0x7c))) ^ 0x8000_0000;
        e.call(0x0041_6870, &args![l.at(0xac), minus_y, x, 0.0f32]);
        scale_assign(e, l.at(0xac), root);
        let along = e
            .call(
                POINT_SCALE,
                &args![l.at(0x80), l.at(0x154), x87_quiet(length)],
            )
            .u32();
        let p = e
            .call(POINT_ADD, &args![along, l.at(0x160), l.at(0xac)])
            .u32();
        copy3(e, l.at(0x8c), p);
        e.call(POINT_NORMALIZE, &args![l.at(0x8c)]);
        let along = e
            .call(
                POINT_SCALE,
                &args![l.at(0x80), l.at(0x16c), x87_quiet(length)],
            )
            .u32();
        let p = e
            .call(POINT_SUB, &args![along, l.at(0x178), l.at(0xac)])
            .u32();
        copy3(e, l.at(0x98), p);
        e.call(POINT_NORMALIZE, &args![l.at(0x98)]);
    }
    e.mem.set_u32(l.at(0x58), 2);
    e.mem.set_u32(l.at(0x54), 0);
    let quiet = |e: &Engine, a: u32| x87_quiet_bits(e.mem.u32(a));
    let (x, y) = (quiet(e, l.at(0x8c)), quiet(e, l.at(0x88)));
    e.mem.set_u32(l.at(0x50), x);
    e.mem.set_u32(l.at(0x4c), y);
    let (x, y) = (quiet(e, l.at(0x8c)), quiet(e, l.at(0x88)));
    e.mem.set_u32(l.at(0x48), x);
    e.mem.set_u32(l.at(0x44), y);
    e.mem.set_u32(l.at(0x40), 1);
    let (x, y) = (quiet(e, l.at(0x98)), quiet(e, l.at(0x94)));
    e.mem.set_u32(l.at(0x3c), x);
    e.mem.set_u32(l.at(0x38), y);
    let (x, y) = (quiet(e, l.at(0x98)), quiet(e, l.at(0x94)));
    e.mem.set_u32(l.at(0x34), x);
    e.mem.set_u32(l.at(0x30), y);
}

/// One way `k` of `006b0f40` (its frame `l`), from the arc check on.
#[allow(clippy::too_many_arguments)]
fn try_way(
    e: &mut Engine,
    this: Ptr<PathSmootherPOVSearch>,
    l: Frame,
    node: Ptr,
    data: u32,
    link: u32,
    target: u32,
    k: u32,
    connections: Ptr,
    added: &mut i32,
) -> Way {
    let s = this.addr();
    let (dir_slot, exit_angle, entry_angle) = tangent(l, k);
    let direction = e.mem.u32(data + 4);
    e.call(
        0x006a_05e0,
        &args![l.at(0x138), link + 8, exit_angle, direction],
    );
    e.call(LOCATION_RECORD_CONSTRUCT, &args![l.at(0x120)]);
    if e.call(0x006a_1b60, &args![node]).u32() == 0 {
        let triangles = Ptr::new(e.mem.u32(s + 0x207c));
        let first = fn_006b3180(e, triangles).addr();
        let navmesh = e.mem.u32(first);
        e.mem.set_u32(l.at(0x120), navmesh);
        let triangles = Ptr::new(e.mem.u32(s + 0x207c));
        let first = fn_006b3180(e, triangles).addr();
        let triangle = e.mem.u16(first + 4);
        e.mem.set_u16(l.at(0x11c), triangle);
        e.mem.set_u32(l.at(0x118), 0xffff_ffff);
        copy3(e, l.at(0x114), s + START);
    } else {
        e.call(0x006c_f900, &args![l.at(0x120), link + 0x28]);
    }
    *added += 1;
    e.call(LOCATION_RECORD_CONSTRUCT, &args![l.at(0xfc)]);
    if e.call(0x006a_1b60, &args![node]).u32() == 0 {
        e.call(0x006c_f900, &args![l.at(0xfc), l.at(0x120)]);
    } else {
        e.mem.set_u32(l.at(0x118), 0xffff_ffff);
        let ok = with_arc_copy(e, l.at(0x138), |e, arc| {
            fn_006b1e50(
                e,
                this,
                Ptr::new(l.at(0x120)),
                Ptr::new(l.at(0x24)),
                arc,
                Ptr::new(l.at(0xfc)),
            )
        });
        if !ok {
            return Way::Failed;
        }
        let d = e
            .call(POINT_SUB, &args![l.at(0xf0), l.at(0x188), l.at(0x24)])
            .u32();
        let dz = e.mem.f32(d + 8);
        if float_00408840(e, dz) >= e.mem.f64(0x0104_ed58) {
            return Way::Failed;
        }
    }
    // The point on the target circle and the straight segment to it.
    e.call(LOCATION_RECORD_CONSTRUCT, &args![l.at(0xd8)]);
    let r = x87_quiet(e.mem.f32(l.at(0x5c)));
    let offset = e
        .call(0x0069_e0c0, &args![entry_angle, l.at(0x194), r])
        .u32();
    e.call(POINT_ADD, &args![l.at(0x68), l.at(0x108), offset]);
    e.mem.set_u32(l.at(0xf4), 0xffff_ffff);
    e.mem.set_f32(l.at(0xb8), 0.0);
    let to_goal = u8::from(e.mem.u32(target) == 3);
    let pov = e.mem.u32(data);
    let from_start = u8::from(e.mem.u32(pov) == 2);
    if !path_smoother_pov_search_triple_check_segment_against_nav_mesh(
        e,
        this,
        Ptr::new(l.at(0xfc)),
        Ptr::new(l.at(0x108)),
        Ptr::new(l.at(0xd8)),
        from_start,
        to_goal,
        Ptr::new(l.at(0xb8)),
    ) {
        return Way::Failed;
    }
    if e.call(0x006a_1b60, &args![node]).u32() != 0 {
        e.mem.set_u32(l.at(0x118), 0xffff_ffff);
        if e.mem.u8(s + 0x2078) == 0 {
            let ok = with_arc_copy(e, l.at(0x138), |e, arc| {
                fn_006b23d0(e, this, Ptr::new(l.at(0x120)), Ptr::new(l.at(0x24)), arc)
            });
            if !ok {
                return Way::Failed;
            }
        }
    }
    // The reached triangle's height against the target's center.
    copy3(e, l.at(0xe4), l.at(0xcc));
    let flt_max = pushed_float(e, 0x0101_6970);
    let triangle = u32::from(e.mem.u16(l.at(0xd4)));
    let navmesh = e.mem.u32(l.at(0xd8));
    e.call(
        NAVMESH_COMPUTE_TRIANGLE_Z,
        &args![navmesh, triangle, l.at(0xcc), flt_max, flt_max, l.at(0xdc)],
    );
    let rise = (f64::from(e.mem.f32(l.at(0xdc))) - f64::from(e.mem.f32(l.at(0x60)))) as f32;
    let navmesh = e.mem.u32(l.at(0xd8));
    let cell = e.call(NAVMESH_CELL, &args![navmesh]).u32();
    let tes = e.mem.u32(TES_SINGLETON);
    let water = e.call(WATER_HEIGHT, &args![tes, l.at(0xcc), cell]).f64();
    let z = f64::from(e.mem.f32(l.at(0xc4)));
    let rise = f64::from(rise);
    if !less(z, water) && !less(e.mem.f64(0x0106_b9f0), rise) {
        return Way::Failed;
    }
    if !less(rise, e.mem.f64(0x0104_ed58)) {
        return Way::Failed;
    }
    e.mem.set_f32(l.at(0xb4), 0.0);
    if e.mem.u8(s + 0x2078) == 0 {
        let pov = Ptr::new(e.mem.u32(data));
        if !fn_006b2c00(
            e,
            this,
            pov,
            Ptr::new(target),
            Ptr::new(l.at(0xf0)),
            Ptr::new(l.at(0xcc)),
            Ptr::new(l.at(0xb4)),
        ) {
            return Way::Skipped;
        }
    }
    let share = (f64::from(e.mem.f32(l.at(0xb8))) * f64_at(e, D_0_9)) as f32;
    e.mem.set_f32(l.at(0xb8), share);
    e.call(0x0053_30e0, &args![l.at(0xb8), 0.0f32, 1.0f32]);
    let mut cost = e.call(0x006a_7f50, &args![node]).f64() as f32;
    let share = f64::from(e.mem.f32(l.at(0xb8)));
    let factor = f64::from(e.mem.f32(target + 0x38));
    if e.call(0x006a_1b60, &args![node]).u32() != 0 {
        let pov = e.mem.u32(data);
        let radius = x87_quiet(e.mem.f32(pov + 0x10));
        let arc = e.call(0x006a_07a0, &args![l.at(0x138), radius]).f64();
        cost = (arc * factor * (1.0 - share) + f64::from(cost)) as f32;
    }
    let segment = f64::from(e.mem.f32(l.at(0xb4)));
    cost = (segment * factor * (1.0 - share) + f64::from(cost)) as f32;
    let way = e.mem.u32(dir_slot);
    let key = e.call(SUM_OF_TWO, &args![s, target, way]).u32();
    let opened = fn_006b3420(e, this.cast(), key, cost, Ptr::new(l.at(0xbc)));
    match opened {
        0 => {
            let reached = Ptr::new(e.mem.u32(l.at(0xbc)));
            let way = e.mem.u32(dir_slot);
            fn_006b1df0(e, this, reached, Ptr::new(target), way);
        }
        1 => {}
        _ => return Way::Skipped,
    }
    let reached = e.mem.u32(l.at(0xbc));
    e.call(0x006c_f900, &args![reached + 0x24, l.at(0xfc)]);
    e.call(0x006c_f900, &args![reached + 0x3c, l.at(0xd8)]);
    e.call(0x0069_85f0, &args![reached + 0x14, exit_angle]);
    e.call(0x0069_85f0, &args![reached + 0x1c, entry_angle]);
    e.call(ARRAY_APPEND_WORD, &args![connections, l.at(0xbc)]);
    Way::Done
}

/// The water clamp used by `006b1970`: a location's point not above the
/// water height of its navmesh's cell is raised to it.
fn raise_location_to_water(e: &mut Engine, location: u32, slot: u32) {
    let navmesh = e.mem.u32(location);
    let cell = e.call(NAVMESH_CELL, &args![navmesh]).u32();
    let tes = e.mem.u32(TES_SINGLETON);
    let water = e
        .call(WATER_HEIGHT, &args![tes, location + 0xc, cell])
        .f64() as f32;
    e.mem.set_f32(slot, water);
    let z = f64::from(e.mem.f32(location + 0x14));
    if z <= f64::from(water) {
        e.mem.set_f32(location + 0x14, x87_quiet(water));
    }
}

// Translated from 006b1970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: after a failed search, links the last node
/// straight to the goal POV when it can. With links, the first common
/// tangent from the last node's circle to the goal circle is taken; the arc
/// from the last link's entry must pass `006b1e50` and `006b23d0`; the
/// segment to the goal circle is checked with
/// `TripleCheckSegmentAgainstNavMesh` (from the start, to the goal) or, for
/// a walker radius under 100, `CheckSegmentAgainstNavMesh` - a passing
/// segment must also pass the avoid nodes (`006b2c00`), a failing one is
/// kept as it ends. The end is raised to the water on an under-water path
/// and pulled back by twice the walker radius; the link (`006b3660`) and
/// the goal node (cost 0, the tangent's direction, `006cf4c0`) are added.
/// Returns whether they were.
pub fn fn_006b1970(e: &mut Engine, this: Ptr<PathSmootherPOVSearch>) -> bool {
    let s = this.addr();
    if e.call(ARRAY_IS_EMPTY, &args![s + LINKS]).bool() {
        return false;
    }
    with_frame(e, 0x12c, |e, l| {
        let last_node = e.call(NODE_ARRAY_LAST, &args![s + NODES]).u32();
        let last_link = fn_006b3100(e, Ptr::new(s + LINKS)).addr();
        let pov = e.mem.u32(last_node);
        let radius = x87_quiet(e.mem.f32(pov + 0x10));
        let pov = e.mem.u32(last_node);
        e.call(0x006a_04b0, &args![l.at(0x7c), pov + 4, radius]);
        let goal = e.mem.u32(s + 0x20c8);
        let radius = x87_quiet(e.mem.f32(goal + 0x10));
        let goal = e.mem.u32(s + 0x20c8);
        e.call(0x006a_04b0, &args![l.at(0xe4), goal + 4, radius]);
        e.call(0x006a_0870, &args![l.at(0xc8)]);
        let direction = e.mem.u32(last_node + 4);
        let count = e
            .call(
                0x006b_b570,
                &args![l.at(0x7c), l.at(0xe4), direction, l.at(0xc8)],
            )
            .u32();
        if count == 0 {
            return false;
        }
        let direction = e.mem.u32(last_node + 4);
        e.call(
            0x006a_05e0,
            &args![l.at(0x104), last_link + 8, l.at(0xc0), direction],
        );
        e.call(LOCATION_RECORD_CONSTRUCT, &args![l.at(0x98)]);
        let entry = Ptr::new(last_link + 0x28);
        let circle = Ptr::new(l.at(0x7c));
        if !with_arc_copy(e, l.at(0x104), |e, arc| {
            fn_006b1e50(e, this, entry, circle, arc, Ptr::new(l.at(0x98)))
        }) {
            return false;
        }
        if !with_arc_copy(e, l.at(0x104), |e, arc| {
            fn_006b23d0(e, this, entry, circle, arc)
        }) {
            return false;
        }
        e.call(LOCATION_RECORD_CONSTRUCT, &args![l.at(0x6c)]);
        let r = x87_quiet(e.mem.f32(l.at(0xd8)));
        let offset = e
            .call(0x0069_e0c0, &args![l.at(0xb8), l.at(0x124), r])
            .u32();
        e.call(POINT_ADD, &args![l.at(0xe4), l.at(0xd4), offset]);
        e.mem.set_u32(l.at(0x90), 0xffff_ffff);
        e.mem.set_f32(l.at(0x4), 0.0);
        let walker = request_radius(e, s);
        if walker < e.mem.f64(0x0101_7a40) || walker.is_nan() {
            if path_smoother_pov_search_check_segment_against_nav_mesh(
                e,
                this,
                Ptr::new(l.at(0x98)),
                Ptr::new(l.at(0xd4)),
                Ptr::new(l.at(0x6c)),
                Ptr::new(l.at(0x4)),
            ) {
                e.mem.set_f32(l.at(0x10c), 0.0);
                let pov = Ptr::new(e.mem.u32(last_node));
                let goal = Ptr::new(e.mem.u32(s + 0x20c8));
                if !fn_006b2c00(
                    e,
                    this,
                    pov,
                    goal,
                    Ptr::new(l.at(0x8c)),
                    Ptr::new(l.at(0xd4)),
                    Ptr::new(l.at(0x10c)),
                ) {
                    return false;
                }
            }
        } else if path_smoother_pov_search_triple_check_segment_against_nav_mesh(
            e,
            this,
            Ptr::new(l.at(0x98)),
            Ptr::new(l.at(0xd4)),
            Ptr::new(l.at(0x6c)),
            0,
            1,
            Ptr::new(l.at(0x4)),
        ) {
            e.mem.set_f32(l.at(0x108), 0.0);
            let pov = Ptr::new(e.mem.u32(last_node));
            let goal = e.mem.u32(s + 0x20c8);
            if !fn_006b2c00(
                e,
                this,
                pov,
                Ptr::new(goal),
                Ptr::new(l.at(0x8c)),
                Ptr::new(l.at(0xd4)),
                Ptr::new(l.at(0x108)),
            ) {
                return false;
            }
        }
        if e.mem.u8(s + 0x2078) != 0 {
            raise_location_to_water(e, l.at(0x6c), l.at(0x110));
        }
        e.call(POINT_SUB, &args![l.at(0xd4), l.at(0xf0), l.at(0x8c)]);
        e.call(POINT_NORMALIZE, &args![l.at(0xf0)]);
        let walker = request_radius(e, s);
        let twice = (walker + walker) as f32;
        scale_assign(e, l.at(0xf0), twice);
        e.call(0x0045_78c0, &args![l.at(0x60), l.at(0xf0)]);
        fn_006b1d80(e, Ptr::new(l.at(0x48)));
        e.call(0x006c_f900, &args![l.at(0x38), l.at(0x98)]);
        e.call(0x006c_f900, &args![l.at(0x20), l.at(0x6c)]);
        e.call(0x0069_85f0, &args![l.at(0x48), l.at(0xc0)]);
        e.call(0x0069_85f0, &args![l.at(0x40), l.at(0xb8)]);
        fn_006b3660(e, Ptr::new(s + LINKS), Ptr::new(l.at(0x48)));
        let goal = e.mem.u32(s + 0x20c8);
        e.mem.set_u32(l.at(0x54), goal);
        e.mem.set_f32(l.at(0x4c), 0.0);
        let way = e.mem.u32(l.at(0xc4));
        e.mem.set_u32(l.at(0x50), way);
        e.call(ADD_RECORD, &args![s + NODES, l.at(0x54)]);
        true
    })
}

// Translated from 006b1d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (Ghidra matches the shape to an MFC
/// `CPreviewView::PAGE_INFO` constructor): the `PathSmootherPOVLink`
/// constructor: `006815c0` on both angles (+0, +8), `006a0480` on both
/// locations (+0x10, +0x28). Returns `this`.
pub fn fn_006b1d80(e: &mut Engine, this: Ptr) -> Ptr {
    let p = this.addr();
    e.call(SELF, &args![p]);
    e.call(SELF, &args![p + 8]);
    e.call(LOCATION_RECORD_CONSTRUCT, &args![p + 0x10]);
    e.call(LOCATION_RECORD_CONSTRUCT, &args![p + 0x28]);
    this
}

// Translated from 006b1dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathSmootherPOVSearch::IsGoalNode` (Xbox PDB): whether the node's POV
/// (its `NodeData`, `00413f40`) is of type 3, the goal.
pub fn path_smoother_pov_search_is_goal_node(
    e: &mut Engine,
    _this: Ptr<PathSmootherPOVSearch>,
    node: Ptr,
) -> bool {
    let data = e.call(NODE_DATA, &args![node]).u32();
    let pov = e.mem.u32(data);
    e.mem.u32(pov) == 3
}

// Translated from 006b1df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: sets a new node's data (`pov`, `direction`)
/// and its estimate: 0.1 x the distance from the POV's center to the goal
/// POV's.
pub fn fn_006b1df0(
    e: &mut Engine,
    this: Ptr<PathSmootherPOVSearch>,
    node: Ptr,
    pov: Ptr,
    direction: u32,
) {
    let s = this.addr();
    with_frame(e, 0x10, |e, l| {
        let data = e.call(NODE_DATA, &args![node]).u32();
        e.mem.set_u32(data, pov.addr());
        let data = e.call(NODE_DATA, &args![node]).u32();
        e.mem.set_u32(data + 4, direction);
        let goal = e.mem.u32(s + 0x20c8);
        let d = e
            .call(POINT_SUB, &args![pov.addr() + 4, l.at(0xc), goal + 4])
            .u32();
        let length = e.call(POINT_LENGTH, &args![d]).f64();
        e.mem
            .set_f32(node.addr() + 4, (length * f64_at(e, D_0_1)) as f32);
    });
}

// Translated from 006b1e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: walks the arc `arc` (exit angle, entry angle,
/// direction; passed by value, here its address) around `circle` from
/// `location` in steps of pi/12 (direction 0) or -pi/12, at most 24, while
/// the step angle is still within the arc (`006a0660`): each chord must
/// pass `CheckSegmentAgainstNavMesh`; then the last chord to the entry
/// angle's point. `out` gets the location reached. Returns whether every
/// chord passed.
pub fn fn_006b1e50(
    e: &mut Engine,
    this: Ptr<PathSmootherPOVSearch>,
    location: Ptr,
    circle: Ptr,
    arc: Ptr,
    out: Ptr,
) -> bool {
    let arc = arc.addr();
    let circle = circle.addr();
    with_frame(e, 0xac, |e, l| {
        let step = if e.mem.u32(arc + 0x10) == 0 {
            e.mem.f32(0x0106_4f64)
        } else {
            e.mem.f32(0x0106_b3a8)
        };
        e.call(0x006a_04e0, &args![l.at(0x6c), x87_quiet(step)]);
        e.call(0x0069_85f0, &args![l.at(0x3c), arc]);
        e.call(0x006a_0e90, &args![l.at(0x3c), l.at(0x60), l.at(0x6c)]);
        let r = x87_quiet(e.mem.f32(circle + 0xc));
        let offset = e.call(0x0069_e0c0, &args![l.at(0x60), l.at(0x78), r]).u32();
        e.call(POINT_ADD, &args![circle, l.at(0x24), offset]);
        e.call(0x006a_0e50, &args![l.at(0x58), location]);
        e.call(LOCATION_RECORD_CONSTRUCT, &args![l.at(0x18)]);
        let mut steps = 0u32;
        e.mem.set_u32(l.at(0x40), 0xffff_ffff);
        e.mem.set_f32(l.at(0x28), 0.0);
        while e.call(0x006a_0660, &args![arc, l.at(0x60)]).bool() && steps < 0x18 {
            e.mem.set_u32(l.at(0x50), 0xffff_ffff);
            if !path_smoother_pov_search_check_segment_against_nav_mesh(
                e,
                this,
                Ptr::new(l.at(0x58)),
                Ptr::new(l.at(0x24)),
                Ptr::new(l.at(0x18)),
                Ptr::new(l.at(0x28)),
            ) {
                e.call(0x006c_f900, &args![out, l.at(0x18)]);
                return false;
            }
            e.call(0x006c_f900, &args![l.at(0x58), l.at(0x18)]);
            e.call(0x0069_85f0, &args![l.at(0x3c), l.at(0x60)]);
            let next = e
                .call(0x006a_0e90, &args![l.at(0x3c), l.at(0x80), l.at(0x6c)])
                .u32();
            e.call(0x0069_85f0, &args![l.at(0x60), next]);
            let r = x87_quiet(e.mem.f32(circle + 0xc));
            let offset = e.call(0x0069_e0c0, &args![l.at(0x60), l.at(0x8c), r]).u32();
            let p = e.call(POINT_ADD, &args![circle, l.at(0x98), offset]).u32();
            copy3(e, l.at(0x24), p);
            let edge = e.mem.u32(l.at(0x10));
            e.mem.set_u32(l.at(0x40), edge);
            steps += 1;
        }
        if steps == 0x18 {
            e.call(0x006c_f900, &args![out, l.at(0x18)]);
            return false;
        }
        let r = x87_quiet(e.mem.f32(circle + 0xc));
        let offset = e.call(0x0069_e0c0, &args![arc + 8, l.at(0xa4), r]).u32();
        e.call(POINT_ADD, &args![circle, l.at(0x34), offset]);
        e.mem.set_u32(l.at(0x50), 0xffff_ffff);
        let ok = path_smoother_pov_search_check_segment_against_nav_mesh(
            e,
            this,
            Ptr::new(l.at(0x58)),
            Ptr::new(l.at(0x34)),
            Ptr::new(l.at(0x18)),
            Ptr::new(l.at(0x28)),
        );
        e.call(0x006c_f900, &args![out, l.at(0x18)]);
        ok
    })
}

/// The uniform form of `006b1e50`: the arc's five words are copied to a
/// block standing for the by-value parameter.
fn fn_006b1e50_entry(e: &mut Engine, a: &[u32]) -> Ret {
    let words = &a[3..8];
    let ok = e.with_stack(0x14, |e, arc| {
        for (k, w) in words.iter().enumerate() {
            e.mem.set_u32(arc.addr() + 4 * k as u32, *w);
        }
        fn_006b1e50(
            e,
            Ptr::new(a[0]),
            Ptr::new(a[1]),
            Ptr::new(a[2]),
            arc,
            Ptr::new(a[8]),
        )
    });
    ok.into_ret()
}

// Translated from 006b2060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathSmootherPOVSearch::CheckSegmentAgainstNavMesh` (Xbox PDB): walks the
/// segment from `location`'s point to `to` across the navmesh (at most 1000
/// triangles), counting triangles with flag 0x40 (`00693b70`). It fails at
/// an edge without a neighbour, a neighbour whose edge info (`0068f230`)
/// is 1, or - when the setting at `011d6ef0` is under the walker radius - a
/// neighbour triangle with flag 0x10 (`00693b90`): `out` gets the crossing
/// with that edge (`006ba240`). Reaching `to`'s triangle, `to`'s z is set
/// to the triangle's height and `out` gets that location. `share` gets the
/// counted share of the triangles walked (0 when none).
pub fn path_smoother_pov_search_check_segment_against_nav_mesh(
    e: &mut Engine,
    this: Ptr<PathSmootherPOVSearch>,
    location: Ptr,
    to: Ptr,
    out: Ptr,
    share: Ptr,
) -> bool {
    let s = this.addr();
    with_frame(e, 0x118, |e, l| {
        copy3(e, l.at(0x34), location.addr() + 0xc);
        copy3(e, l.at(0x28), to.addr());
        e.call(POINT_SUB, &args![l.at(0x28), l.at(0x40), l.at(0x34)]);
        e.call(POINT_NORMALIZE, &args![l.at(0x40)]);
        copy3(e, l.at(0x1c), l.at(0x34));
        e.call(RECORD_FROM_HANDLE, &args![l.at(0xc), location]);
        let mut preferred = 0i32;
        let mut steps = 0i32;
        let set_share = |e: &mut Engine, preferred: i32, steps: i32| {
            let value = (f64::from(preferred) / f64::from(steps)) as f32;
            e.mem.set_f32(share.addr(), value);
        };
        while steps < 1000 {
            steps += 1;
            let navmesh = e.mem.u32(l.at(0xc));
            let tri = u32::from(e.mem.u16(l.at(0x8)));
            let triangle = e.call(0x0055_82f0, &args![navmesh, tri]).u32();
            if e.call(0x0069_3b70, &args![triangle]).bool() {
                preferred += 1;
            }
            let edge_word = e.mem.u32(l.at(0x4));
            let exit = e
                .call(
                    NAVMESH_EXIT_EDGE,
                    &args![navmesh, tri, edge_word, l.at(0x1c), l.at(0x28)],
                )
                .u32();
            if exit == 0xffff_ffff {
                let z = x87_quiet(e.mem.f32(l.at(0x20)));
                e.mem.set_f32(l.at(0x4c), z);
                let flt_max = pushed_float(e, 0x0101_6970);
                let tri = u32::from(e.mem.u16(l.at(0x8)));
                let navmesh = e.mem.u32(l.at(0xc));
                e.call(
                    NAVMESH_COMPUTE_TRIANGLE_Z,
                    &args![navmesh, tri, l.at(0x28), flt_max, flt_max, l.at(0x4c)],
                );
                let z = x87_quiet(e.mem.f32(l.at(0x4c)));
                e.mem.set_f32(l.at(0x20), z);
                let reached = e
                    .call(0x006a_0c10, &args![l.at(0xac), l.at(0xc), l.at(0x28)])
                    .u32();
                e.call(0x006c_f900, &args![out, reached]);
                set_share(e, preferred, steps);
                return true;
            }
            let tri = u32::from(e.mem.u16(l.at(0x8)));
            let navmesh = e.mem.u32(l.at(0xc));
            e.call(EDGE_HANDLE_SET, &args![l.at(0x58), navmesh, tri, exit]);
            e.call(EDGE_HANDLE_CONSTRUCT, &args![l.at(0x68)]);
            let mesh = e.mem.u32(l.at(0x58));
            let mut passable = e
                .call(
                    NAVMESH_GET_MATCHING_EDGE_HANDLE,
                    &args![mesh, l.at(0x58), l.at(0x68)],
                )
                .bool();
            if passable {
                let edge_word = e.mem.u32(l.at(0x50));
                let tri = u32::from(e.mem.u16(l.at(0x54)));
                let mesh = e.mem.u32(l.at(0x58));
                let info = e.call(0x0068_f230, &args![mesh, tri, edge_word]).u32();
                if info != 0 && e.mem.u32(info) == 1 {
                    passable = false;
                }
                let walker = request_radius(e, s);
                let setting = e.call(0x0040_3e20, &args![0x011d_6ef0u32]).u32();
                if f64::from(e.mem.f32(setting)) < walker {
                    let tri = u32::from(e.mem.u16(l.at(0x64)));
                    let mesh = e.mem.u32(l.at(0x68));
                    let triangle = e.call(0x0055_82f0, &args![mesh, tri]).u32();
                    if e.call(0x0069_3b90, &args![triangle]).bool() {
                        passable = false;
                    }
                }
            }
            if !passable {
                let tri = u32::from(e.mem.u16(l.at(0x54)));
                let mesh = e.mem.u32(l.at(0x58));
                e.call(
                    NAVMESH_GET_EDGE_VERTICES,
                    &args![mesh, l.at(0x94), tri, exit],
                );
                e.call(SELF, &args![l.at(0x8c)]);
                let a = e.mem.u32(l.at(0x94));
                let b = e.mem.u32(l.at(0x90));
                e.call(
                    0x006b_a240,
                    &args![l.at(0x34), l.at(0x28), a, b, l.at(0x8c)],
                );
                let blocked = e
                    .call(0x006a_0c10, &args![l.at(0xf4), l.at(0x58), l.at(0x8c)])
                    .u32();
                e.call(0x006c_f900, &args![out, blocked]);
                set_share(e, preferred, steps);
                return false;
            }
            if e.mem.u32(l.at(0xc)) != e.mem.u32(l.at(0x68)) {
                let edge_word = e.mem.u32(l.at(0x60));
                let tri = u32::from(e.mem.u16(l.at(0x64)));
                let mesh = e.mem.u32(l.at(0x68));
                e.call(
                    NAVMESH_GET_EDGE_VERTICES,
                    &args![mesh, l.at(0x80), tri, edge_word],
                );
                let half = pushed_float(e, HALF);
                let a = e.mem.u32(l.at(0x80));
                let b = e.mem.u32(l.at(0x7c));
                let sum = e.call(POINT_ADD, &args![a, l.at(0xb8), b]).u32();
                e.call(POINT_SCALE, &args![sum, l.at(0x78), half]);
                let a = e.mem.u32(l.at(0x80));
                let b = e.mem.u32(l.at(0x7c));
                let on_edge = e
                    .call(
                        SEGMENT_POINT,
                        &args![l.at(0xc4), a, b, l.at(0x34), l.at(0x28)],
                    )
                    .u32();
                copy3(e, l.at(0x1c), on_edge);
                let step = pushed_float(e, STEP_FACTOR);
                let towards = e
                    .call(POINT_SUB, &args![l.at(0x78), l.at(0xd0), l.at(0x1c)])
                    .u32();
                let towards = e.call(POINT_SCALE, &args![towards, l.at(0xdc), step]).u32();
                e.call(POINT_ADD_ASSIGN, &args![l.at(0x1c), towards]);
            }
            e.call(EDGE_HANDLE_COPY, &args![l.at(0xc), l.at(0x68)]);
        }
        let reached = e
            .call(0x006a_0c10, &args![l.at(0x10c), l.at(0xc), l.at(0x28)])
            .u32();
        e.call(0x006c_f900, &args![out, reached]);
        if steps > 0 {
            set_share(e, preferred, steps);
        } else {
            e.mem.set_f32(share.addr(), 0.0);
        }
        false
    })
}

/// `006b23d0`'s check on one side: the arc's exit point on a circle of
/// `radius` around `circle`'s center, the segment to it from `location`
/// (`CheckSegmentAgainstNavMesh`), then the arc on that circle from the
/// reached location (`006b1e50`).
#[allow(clippy::too_many_arguments)]
fn offset_arc_passes(
    e: &mut Engine,
    this: Ptr<PathSmootherPOVSearch>,
    l: Frame,
    location: Ptr,
    circle: u32,
    arc: u32,
    radius: f32,
    slots: [u32; 5],
) -> bool {
    let [offset, reached, ring, sum, end] = slots;
    let offset = e
        .call(0x0069_e0c0, &args![arc, l.at(offset), x87_quiet(radius)])
        .u32();
    e.call(LOCATION_RECORD_CONSTRUCT, &args![l.at(reached)]);
    e.call(0x006a_04b0, &args![l.at(ring), circle, x87_quiet(radius)]);
    let p = e.call(POINT_ADD, &args![circle, l.at(sum), offset]).u32();
    if !path_smoother_pov_search_check_segment_against_nav_mesh(
        e,
        this,
        location,
        Ptr::new(p),
        Ptr::new(l.at(reached)),
        Ptr::new(l.at(0x2c)),
    ) {
        return false;
    }
    e.call(LOCATION_RECORD_CONSTRUCT, &args![l.at(end)]);
    with_arc_copy(e, arc, |e, copy| {
        fn_006b1e50(
            e,
            this,
            Ptr::new(l.at(reached)),
            Ptr::new(l.at(ring)),
            copy,
            Ptr::new(l.at(end)),
        )
    })
}

// Translated from 006b23d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: checks that the walker fits along the arc
/// `arc` (by value; here its address) around `circle`: the arc is walked
/// (`006b1e50`) on the circles of radius `r - 0.9 x walker` and
/// `r + 0.9 x walker` (inner first for direction 0, outer first otherwise),
/// each reached from `location` by a checked segment. Returns whether all
/// passed.
pub fn fn_006b23d0(
    e: &mut Engine,
    this: Ptr<PathSmootherPOVSearch>,
    location: Ptr,
    circle: Ptr,
    arc: Ptr,
) -> bool {
    let s = this.addr();
    let circle = circle.addr();
    let arc = arc.addr();
    with_frame(e, 0xd8, |e, l| {
        let r = f64::from(e.mem.f32(circle + 0xc));
        let direction = e.mem.u32(arc + 0x10);
        let margin = request_radius(e, s) * f64_at(e, D_0_9);
        let first = (if direction == 0 {
            r - margin
        } else {
            margin + r
        }) as f32;
        e.mem.set_f32(l.at(0x74), first);
        e.mem.set_f32(l.at(0x2c), 0.0);
        if !offset_arc_passes(
            e,
            this,
            l,
            location,
            circle,
            arc,
            first,
            [0x38, 0xa4, 0x70, 0xb0, 0x8c],
        ) {
            return false;
        }
        let r = f64::from(e.mem.f32(circle + 0xc));
        let margin = request_radius(e, s) * f64_at(e, D_0_9);
        let second = (if direction == 0 {
            margin + r
        } else {
            r - margin
        }) as f32;
        e.mem.set_f32(l.at(0x3c), second);
        offset_arc_passes(
            e,
            this,
            l,
            location,
            circle,
            arc,
            second,
            [0x48, 0x60, 0x28, 0xc0, 0x18],
        )
    })
}

/// The uniform form of `006b23d0` (the arc's five words by value).
fn fn_006b23d0_entry(e: &mut Engine, a: &[u32]) -> Ret {
    let words = &a[3..8];
    let ok = e.with_stack(0x14, |e, arc| {
        for (k, w) in words.iter().enumerate() {
            e.mem.set_u32(arc.addr() + 4 * k as u32, *w);
        }
        fn_006b23d0(e, Ptr::new(a[0]), Ptr::new(a[1]), Ptr::new(a[2]), arc)
    });
    ok.into_ret()
}

/// `scale(scale(point, a), b)` through `0045bb20`, the two outputs in the
/// frame slots `first` and `second`; returns the second's address.
fn scale_twice(e: &mut Engine, l: Frame, point: u32, a: u32, b: u32, slots: [u32; 2]) -> u32 {
    let once = e.call(POINT_SCALE, &args![point, l.at(slots[0]), a]).u32();
    e.call(POINT_SCALE, &args![once, l.at(slots[1]), b]).u32()
}

/// `-point` through `004a0bd0(this, out)`.
fn negate(e: &mut Engine, point: u32, out: u32) -> u32 {
    e.call(0x004a_0bd0, &args![point, out]).u32()
}

/// One side offset of `TripleCheckSegmentAgainstNavMesh` at a flagged end:
/// `(side or -side) x half x 0.5 +/- along x 0.5`, copied to the frame slot
/// `dst`. `slots`: the along half, the negation (when `negative`), the two
/// scales and the sum.
#[allow(clippy::too_many_arguments)]
fn flagged_offset(
    e: &mut Engine,
    l: Frame,
    half: u32,
    negative: bool,
    add: bool,
    slots: [u32; 5],
    dst: u32,
) {
    let [along, neg, first, second, sum] = slots;
    let point_five = pushed_float(e, HALF);
    let along = e
        .call(POINT_SCALE, &args![l.at(0x50), l.at(along), point_five])
        .u32();
    let side = if negative {
        negate(e, l.at(0x44), l.at(neg))
    } else {
        l.at(0x44)
    };
    let point_five = pushed_float(e, HALF);
    let scaled = scale_twice(e, l, side, half, point_five, [first, second]);
    let op = if add { POINT_ADD } else { POINT_SUB };
    let p = e.call(op, &args![scaled, l.at(sum), along]).u32();
    copy3(e, l.at(dst), p);
}

/// One side offset at an end that is not flagged: `(side or -side) x
/// walker radius x 0.9`, copied to the frame slot `dst`.
fn plain_offset(
    e: &mut Engine,
    s: u32,
    l: Frame,
    negative: Option<u32>,
    slots: [u32; 2],
    dst: u32,
) {
    let factor = pushed_float(e, 0x0101_77e8);
    let walker = request_radius(e, s) as f32;
    let side = match negative {
        Some(out) => negate(e, l.at(0x44), l.at(out)),
        None => l.at(0x44),
    };
    let p = scale_twice(e, l, side, walker.to_bits(), factor, slots);
    copy3(e, l.at(dst), p);
}

// Translated from 006b25f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathSmootherPOVSearch::TripleCheckSegmentAgainstNavMesh` (Xbox PDB):
/// `CheckSegmentAgainstNavMesh` on the segment itself, then (off an
/// under-water path) on the two side lines a walker width apart: side =
/// `up x direction`, offset by 0.9 x the walker radius; at a flagged end
/// (`from_start` / `to_goal`) the side lines meet the end, offset by half of
/// that and half of 0.9 x the length (or the radius when shorter) along the
/// segment. On an under-water path `out`'s z is `to`'s; otherwise, unless
/// `to_goal`, it is the reached triangle's height. Returns whether all
/// passed.
#[allow(clippy::too_many_arguments)]
pub fn path_smoother_pov_search_triple_check_segment_against_nav_mesh(
    e: &mut Engine,
    this: Ptr<PathSmootherPOVSearch>,
    location: Ptr,
    to: Ptr,
    out: Ptr,
    from_start: u8,
    to_goal: u8,
    share: Ptr,
) -> bool {
    let s = this.addr();
    if !path_smoother_pov_search_check_segment_against_nav_mesh(e, this, location, to, out, share) {
        return false;
    }
    if e.mem.u8(s + 0x2078) != 0 {
        let z = x87_quiet(e.mem.f32(to.addr() + 8));
        e.mem.set_f32(out.addr() + 0x14, z);
        return true;
    }
    with_frame(e, 0x260, |e, l| {
        e.call(POINT_SUB, &args![to, l.at(0xb0), location.addr() + 0xc]);
        copy3(e, l.at(0x8c), l.at(0xb0));
        e.call(POINT_NORMALIZE, &args![l.at(0x8c)]);
        e.call(SELF, &args![l.at(0x5c)]);
        e.call(SELF, &args![l.at(0x34)]);
        e.call(SELF, &args![l.at(0x80)]);
        e.call(SELF, &args![l.at(0x28)]);
        let walker = request_radius(e, s);
        let walker_squared = request_radius(e, s) * walker;
        let squared = e.call(POINT_LENGTH_SQUARED, &args![l.at(0xb0)]).f64();
        let half = if squared < walker_squared {
            let length = e.call(POINT_LENGTH, &args![l.at(0xb0)]).f64();
            (length * f64_at(e, D_0_9)) as f32
        } else {
            (request_radius(e, s) * f64_at(e, D_0_9)) as f32
        };
        e.mem.set_f32(l.at(0x4), half);
        let half = x87_quiet(half).to_bits();
        e.call(POINT_SCALE, &args![l.at(0x8c), l.at(0x50), half]);
        e.call(POINT_CROSS, &args![UP_AXIS, l.at(0x44), l.at(0x8c)]);
        if from_start != 0 {
            flagged_offset(e, l, half, false, true, [0xf0, 0, 0xd8, 0xe4, 0xfc], 0x5c);
            flagged_offset(
                e,
                l,
                half,
                true,
                true,
                [0x12c, 0x108, 0x114, 0x120, 0x138],
                0x34,
            );
        } else {
            plain_offset(e, s, l, None, [0x144, 0x150], 0x5c);
            plain_offset(e, s, l, Some(0x15c), [0x168, 0x174], 0x34);
        }
        if to_goal != 0 {
            flagged_offset(
                e,
                l,
                half,
                false,
                false,
                [0x198, 0, 0x180, 0x18c, 0x1a4],
                0x80,
            );
            flagged_offset(
                e,
                l,
                half,
                true,
                false,
                [0x1d4, 0x1b0, 0x1bc, 0x1c8, 0x1e0],
                0x28,
            );
        } else {
            plain_offset(e, s, l, None, [0x1ec, 0x1f8], 0x80);
            plain_offset(e, s, l, Some(0x204), [0x210, 0x21c], 0x28);
        }
        let from = location.addr() + 0xc;
        let lines = [
            (location.addr(), from, 0x5c, 0x228, 0xc8),
            (l.at(0xc8), to.addr(), 0x80, 0x234, 0xa4),
            (location.addr(), from, 0x34, 0x240, 0x74),
            (l.at(0x74), to.addr(), 0x28, 0x24c, 0x1c),
        ];
        for (start, base, offset, sum, reached) in lines {
            e.call(LOCATION_RECORD_CONSTRUCT, &args![l.at(reached)]);
            let p = e
                .call(POINT_ADD, &args![base, l.at(sum), l.at(offset)])
                .u32();
            if !path_smoother_pov_search_check_segment_against_nav_mesh(
                e,
                this,
                Ptr::new(start),
                Ptr::new(p),
                Ptr::new(l.at(reached)),
                Ptr::new(l.at(0x38)),
            ) {
                return false;
            }
        }
        if to_goal == 0 {
            let o = out.addr();
            let z = x87_quiet(e.mem.f32(o + 0x14));
            e.mem.set_f32(l.at(0xcc), z);
            let flt_max = pushed_float(e, 0x0101_6970);
            let triangle = u32::from(e.mem.u16(o + 4));
            let navmesh = e.mem.u32(o);
            e.call(
                NAVMESH_COMPUTE_TRIANGLE_Z,
                &args![navmesh, triangle, o + 0xc, flt_max, flt_max, l.at(0xcc)],
            );
            let z = x87_quiet(e.mem.f32(l.at(0xcc)));
            e.mem.set_f32(o + 0x14, z);
        }
        true
    })
}

/// `006b2c00`'s section of the segment inside an avoid circle, by the kind
/// in `[EBP-0x68]` (`006bac20`): 5 the whole segment, 4 between the two
/// fractions, 1 from the start to the first fraction, 2 from the first
/// fraction to the end. Kinds 0 and 3 return `None` (no section).
fn avoid_section(e: &mut Engine, l: Frame, from: u32, to: u32) -> Option<()> {
    let along = |e: &mut Engine, fraction: u32, temps: [u32; 3], dst: u32| {
        let t = x87_quiet(e.mem.f32(l.at(fraction)));
        let d = e.call(POINT_SUB, &args![to, l.at(temps[0]), from]).u32();
        let d = e.call(POINT_SCALE, &args![d, l.at(temps[1]), t]).u32();
        let p = e.call(POINT_ADD, &args![from, l.at(temps[2]), d]).u32();
        copy3(e, l.at(dst), p);
    };
    match e.mem.u32(l.at(0x68)) {
        0 | 3 => return None,
        5 => {
            copy3(e, l.at(0x14), from);
            copy3(e, l.at(0x44), to);
        }
        4 => {
            along(e, 0x58, [0x9c, 0xa8, 0xb4], 0x14);
            along(e, 0x48, [0xc0, 0xcc, 0xd8], 0x44);
        }
        1 => {
            copy3(e, l.at(0x14), from);
            along(e, 0x58, [0xe4, 0xf0, 0xfc], 0x44);
        }
        2 => {
            along(e, 0x58, [0x108, 0x114, 0x120], 0x14);
            copy3(e, l.at(0x44), to);
        }
        _ => {}
    }
    Some(())
}

// Translated from 006b2c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the cost of the segment `from`-`to` from
/// `pov` to `target` against the avoid-node POVs. `cost` starts as the
/// segment's length; a segment shorter than 0.001 in xy passes at once.
/// For each avoid POV the segment touches (`006bab00`), the section inside
/// its circle (`006bac20`) adds its length x (1 + depth x the setting at
/// `011d7084`), depth = (radius - distance of the section from the center
/// (`006b9b10`)) / radius, at least 0. Crossing a type-5 POV other than the
/// target multiplies the cost by 10, or fails when the request's byte at
/// +0x9b is set and `pov` is not the start. Returns whether it passed.
pub fn fn_006b2c00(
    e: &mut Engine,
    this: Ptr<PathSmootherPOVSearch>,
    pov: Ptr,
    target: Ptr,
    from: Ptr,
    to: Ptr,
    cost: Ptr,
) -> bool {
    let s = this.addr();
    let (from, to) = (from.addr(), to.addr());
    with_frame(e, 0x134, |e, l| {
        let d = e.call(POINT_SUB, &args![to, l.at(0x84), from]).u32();
        let length = e.call(POINT_LENGTH, &args![d]).f64() as f32;
        e.mem.set_f32(cost.addr(), length);
        let d = e.call(POINT_SUB, &args![to, l.at(0x90), from]).u32();
        let xy = e.call(0x0059_5c80, &args![d]).f64();
        if xy < e.mem.f64(0x0101_6978) {
            return true;
        }
        let mut i = 0u32;
        while i < size(e, s + AVOID_NODE_POVS) {
            let slot = e.call(ARRAY_WORD_AT, &args![s + AVOID_NODE_POVS, i]).u32();
            let avoid = e.mem.u32(slot);
            i += 1;
            e.mem.set_u32(l.at(0x8), avoid);
            let radius = x87_quiet(e.mem.f32(avoid + 0x10));
            if !e
                .call(0x006b_ab00, &args![avoid + 4, radius, from, to])
                .bool()
            {
                continue;
            }
            let radius = x87_quiet(e.mem.f32(avoid + 0x10));
            e.call(0x006a_04b0, &args![l.at(0x78), avoid + 4, radius]);
            e.call(0x0069_8580, &args![l.at(0x30), from, to]);
            e.call(0x0069_85c0, &args![l.at(0x68)]);
            e.call(0x006b_ac20, &args![l.at(0x78), l.at(0x30), l.at(0x68)]);
            e.call(SELF, &args![l.at(0x14)]);
            e.call(SELF, &args![l.at(0x44)]);
            if avoid_section(e, l, from, to).is_none() {
                continue;
            }
            if e.mem.u32(avoid) == 5 && avoid != target.addr() {
                let request = e.mem.u32(s + 0x2058);
                if e.call(0x006a_9560, &args![request]).bool() && e.mem.u32(pov.addr()) != 2 {
                    return false;
                }
                let scaled = f64::from(e.mem.f32(cost.addr())) * e.mem.f64(0x0102_0758);
                e.mem.set_f32(cost.addr(), scaled as f32);
            }
            let d = e
                .call(POINT_SUB, &args![l.at(0x44), l.at(0x12c), l.at(0x14)])
                .u32();
            let section = e.call(POINT_LENGTH, &args![d]).f64() as f32;
            let distance = e
                .call(0x006b_9b10, &args![avoid + 4, l.at(0x14), l.at(0x44), 0u32])
                .f64() as f32;
            let radius = f64::from(e.mem.f32(avoid + 0x10));
            let mut depth = ((radius - f64::from(distance)) / radius) as f32;
            if f64::from(depth) < f64_at(e, D_ZERO) {
                depth = 0.0;
            }
            let setting = e.call(0x0040_3e20, &args![0x011d_7084u32]).u32();
            let weight = f64::from(e.mem.f32(setting));
            let total = (f64::from(depth) * weight + 1.0) * f64::from(section)
                + f64::from(e.mem.f32(cost.addr()));
            e.mem.set_f32(cost.addr(), total as f32);
        }
        true
    })
}

// Translated from 006b2fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: after a failed search, adds to `target`
/// (`006cf4c0`) every corridor edge whose length squared is under 4 x the
/// walker radius squared (too narrow for the walker). Returns whether any
/// was added (false for a null `target`).
pub fn fn_006b2fc0(e: &mut Engine, this: Ptr<PathSmootherPOVSearch>, target: Ptr) -> bool {
    if target.is_null() {
        return false;
    }
    let s = this.addr();
    with_frame(e, 0x40, |e, l| {
        let mut added = false;
        let mut i = 0u32;
        loop {
            let edges = e.mem.u32(s + 0x2080);
            if i >= size(e, edges) {
                break;
            }
            e.call(SELF, &args![l.at(0x14)]);
            e.call(SELF, &args![l.at(0x20)]);
            let edge = edge_edge(e, edges, i);
            let triangle = edge_triangle(e, edges, i);
            let navmesh = edge_navmesh(e, edges, i);
            e.call(
                NAVMESH_EDGE_ENDS,
                &args![navmesh, triangle, edge, l.at(0x14), l.at(0x20)],
            );
            let d = e
                .call(POINT_SUB, &args![l.at(0x14), l.at(0x2c), l.at(0x20)])
                .u32();
            let squared = e.call(POINT_LENGTH_SQUARED, &args![d]).f64();
            let first = request_radius(e, s);
            let second = request_radius(e, s);
            if second * first * f64_at(e, 0x0101_db80) > squared {
                let element = e.call(EDGE_ARRAY_AT, &args![edges, i]).u32();
                e.call(ADD_RECORD, &args![target, element]);
                added = true;
            }
            i += 1;
        }
        added
    })
}

// Translated from 006b30e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: address of element `index` of a
/// `BSSimpleArray` of 0x40-byte elements (`PathSmootherPOVLink`).
pub fn fn_006b30e0(e: &mut Engine, this: Ptr, index: u32) -> Ptr {
    let buffer = e.mem.u32(this.addr() + 4);
    Ptr::new((index << 6).wrapping_add(buffer))
}

// Translated from 006b3100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the last element of that array
/// (`006b30e0(size - 1)`).
pub fn fn_006b3100(e: &mut Engine, this: Ptr) -> Ptr {
    let last = e.mem.u32(this.addr() + 8).wrapping_sub(1);
    fn_006b30e0(e, this, last)
}

// Translated from 006b3120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: address of element `index` of a
/// `BSSimpleArray` of 0x20-byte elements (`PathEdgeVert`).
pub fn fn_006b3120(e: &mut Engine, this: Ptr, index: u32) -> Ptr {
    let buffer = e.mem.u32(this.addr() + 4);
    Ptr::new((index << 5).wrapping_add(buffer))
}

// Translated from 006b3140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the last element of that array.
pub fn fn_006b3140(e: &mut Engine, this: Ptr) -> Ptr {
    let last = e.mem.u32(this.addr() + 8).wrapping_sub(1);
    fn_006b3120(e, this, last)
}

// Translated from 006b3160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: address of element `index` of a
/// `BSSimpleArray` of 0x14-byte elements (the corridor's triangles).
pub fn fn_006b3160(e: &mut Engine, this: Ptr, index: u32) -> Ptr {
    let buffer = e.mem.u32(this.addr() + 4);
    Ptr::new(index.wrapping_mul(0x14).wrapping_add(buffer))
}

// Translated from 006b3180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: element 0 of that array.
pub fn fn_006b3180(e: &mut Engine, this: Ptr) -> Ptr {
    fn_006b3160(e, this, 0)
}

// Translated from 006b31a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the last element of that array.
pub fn fn_006b31a0(e: &mut Engine, this: Ptr) -> Ptr {
    let last = e.mem.u32(this.addr() + 8).wrapping_sub(1);
    fn_006b3160(e, this, last)
}

// Translated from 006b31c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `AStarSearch<PathSmootherPOVNode,
/// PathSmootherPOVLink>` constructor: vtable, node heap (`006a1f70`), node
/// map (`006b3de0`), queue (`006b9470`); `BestNodeSoFar`,
/// `pLastSearchedNode`, `pStartAStarNode` null and `bIsRunningInTasklet`
/// false. Returns `this`.
// Translation incomplete: 006b31c0 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn fn_006b31c0(e: &mut Engine, this: Ptr<AStarSearch>) -> Ptr<AStarSearch> {
    let p = this.addr();
    e.mem.set_u32(p, VTABLE_ASTAR_SEARCH);
    e.call(0x006a_1f70, &args![p + NODE_HEAP]);
    e.call(0x006b_3de0, &args![p + NODE_MAP]);
    e.call(0x006b_9470, &args![p + SEARCH_QUEUE]);
    e.set(this, AStarSearch::BestNodeSoFar, 0);
    e.set(this, AStarSearch::pLastSearchedNode, 0);
    e.set(this, AStarSearch::pStartAStarNode, 0);
    e.set(this, AStarSearch::bIsRunningInTasklet, 0);
    this
}

// Translated from 006b3270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `AStarSearch` destructor body: vtable,
/// then the queue and the node map (`00483710` on each) and the node heap
/// (`00982610`).
// Translation incomplete: 006b3270 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn fn_006b3270(e: &mut Engine, this: Ptr<AStarSearch>) {
    let p = this.addr();
    e.mem.set_u32(p, VTABLE_ASTAR_SEARCH);
    e.call(0x0048_3710, &args![p + SEARCH_QUEUE]);
    e.call(0x0048_3710, &args![p + NODE_MAP]);
    e.call(0x0098_2610, &args![p + NODE_HEAP]);
}

// Translated from 006b32f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: adds the start node: `006b3420` with cost 0;
/// a new node gets `estimate`, the 12-byte node data at `data`, is queued
/// (`006b3e10`) and becomes `pStartAStarNode`; returns the node (also an
/// existing one with a higher cost), null when an equal or cheaper one
/// exists.
pub fn fn_006b32f0(
    e: &mut Engine,
    this: Ptr<AStarSearch>,
    key: u32,
    data: Ptr,
    estimate: f32,
) -> u32 {
    let p = this.addr();
    with_frame(e, 0xc, |e, l| {
        match fn_006b3420(e, this, key, 0.0, Ptr::new(l.at(0x4))) {
            0 => {
                let node = e.mem.u32(l.at(0x4));
                e.mem.set_f32(node + 4, x87_quiet(estimate));
                copy3(e, node + 8, data.addr());
                e.call(0x006b_3e10, &args![p + SEARCH_QUEUE, node]);
                e.set(this, AStarSearch::pStartAStarNode, node);
                node
            }
            1 => e.mem.u32(l.at(0x4)),
            _ => 0,
        }
    })
}

// Translated from 006b3380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: runs the search. Stores whether this runs in
/// a tasklet (`BSTaskletManager` instance `00b00a00`, virtual slot +0xc on
/// `0040fc90`'s value), searches (`006b3a30`) and builds the path from the
/// goal node it returns into `nodes` and `links` (`006b3cb0`); without one
/// and with `use_best`, from `BestNodeSoFar`. Returns whether the goal was
/// reached.
pub fn fn_006b3380(
    e: &mut Engine,
    this: Ptr<AStarSearch>,
    nodes: Ptr,
    links: Ptr,
    use_best: u8,
) -> bool {
    let manager = e.call(0x00b0_0a00, &[]).u32();
    let value = e.call(0x0040_fc90, &[]).u32();
    let in_tasklet = e.vcall(manager, 0xc, &args![value]).u8();
    e.set(this, AStarSearch::bIsRunningInTasklet, in_tasklet);
    let goal = e.call(0x006b_3a30, &args![this]).u32();
    let (found, node) = if goal != 0 {
        (true, goal)
    } else if use_best != 0 {
        (false, e.get(this, AStarSearch::BestNodeSoFar))
    } else {
        (false, 0)
    };
    if node != 0 {
        e.call(0x006b_3cb0, &args![this, node, nodes, links]);
    }
    found
}

// Translated from 006b3420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: looks `key` up in the node map's bucket
/// (`00996840` gives the bucket index). A node with that key and a higher
/// cost is taken out of the queue (`006a1fb0`), gets `cost` and is
/// returned in `out` (1); one with an equal or lower cost, or an unordered
/// one, gives 2; otherwise a new node from the heap (`006b3d90`) with the
/// key and cost is put at the bucket's head and returned in `out` (0).
pub fn fn_006b3420(e: &mut Engine, this: Ptr<AStarSearch>, key: u32, cost: f32, out: Ptr) -> u32 {
    let p = this.addr();
    let index = e.call(0x0099_6840, &args![p + NODE_MAP, key]).u32();
    let bucket = p + NODE_MAP + index.wrapping_mul(4);
    let improve = |e: &mut Engine, node: u32| -> u32 {
        if f64::from(cost) < f64::from(e.mem.f32(node)) {
            e.mem.set_u32(out.addr(), node);
            e.call(0x006a_1fb0, &args![p + SEARCH_QUEUE, node]);
            e.mem.set_f32(node, x87_quiet(cost));
            1
        } else {
            2
        }
    };
    let head = e.mem.u32(bucket);
    if head != 0 {
        if e.mem.u32(head + 0x60) == key {
            return improve(e, head);
        }
        let mut node = e.mem.u32(head + 0x64);
        while node != 0 {
            if e.mem.u32(node + 0x60) == key {
                return improve(e, node);
            }
            node = e.mem.u32(node + 0x64);
        }
    }
    let node = e.call(0x006b_3d90, &args![p + NODE_HEAP]).u32();
    e.mem.set_u32(out.addr(), node);
    e.mem.set_u32(node + 0x60, key);
    e.mem.set_f32(node, x87_quiet(cost));
    let head = e.mem.u32(bucket);
    e.mem.set_u32(node + 0x64, head);
    e.mem.set_u32(bucket, node);
    0
}

/// The shape shared by the arrays' constructors: vtable, then the array
/// set-up with reserved size and size 0 (`constructor(this, 0, 0)`).
fn construct_array(e: &mut Engine, this: Ptr, vtable: u32, set_up: Option<u32>) -> Ptr {
    e.mem.set_u32(this.addr(), vtable);
    match set_up {
        Some(addr) => {
            e.call(addr, &args![this, 0u32, 0u32]);
        }
        None => fn_006b3eb0(e, this.cast(), 0, 0),
    }
    this
}

/// The shape shared by the arrays' destructors: vtable, then `008454f0`
/// with 1.
fn destroy_array(e: &mut Engine, this: Ptr, vtable: u32) {
    e.mem.set_u32(this.addr(), vtable);
    e.call(ARRAY_CLEAR, &args![this, 1u32]);
}

// Translated from 006b3570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSSimpleArray<PathSmootherPOV*,1024>`
/// constructor (vtable, `006b3eb0(0, 0)`). Returns `this`.
pub fn fn_006b3570(e: &mut Engine, this: Ptr) -> Ptr {
    construct_array(e, this, VTABLE_POV_ARRAY, None)
}

// Translated from 006b35a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSSimpleArray<PathSmootherPOV*,1024>`
/// destructor body (vtable, `008454f0(1)`).
pub fn fn_006b35a0(e: &mut Engine, this: Ptr) {
    destroy_array(e, this, VTABLE_POV_ARRAY);
}

// Translated from 006b35c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSSimpleArray<PathSmootherPOVNode,1024>`
/// constructor (vtable, `006b3f40(0, 0)`). Returns `this`.
pub fn fn_006b35c0(e: &mut Engine, this: Ptr) -> Ptr {
    construct_array(e, this, VTABLE_NODE_ARRAY, Some(0x006b_3f40))
}

// Translated from 006b35f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSSimpleArray<PathSmootherPOVNode,1024>`
/// destructor body.
pub fn fn_006b35f0(e: &mut Engine, this: Ptr) {
    destroy_array(e, this, VTABLE_NODE_ARRAY);
}

// Translated from 006b3610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSSimpleArray<PathSmootherPOVLink,1024>`
/// constructor (vtable, `006a2130(0, 0)`). Returns `this`.
pub fn fn_006b3610(e: &mut Engine, this: Ptr) -> Ptr {
    construct_array(e, this, VTABLE_LINK_ARRAY, Some(0x006a_2130))
}

// Translated from 006b3640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSSimpleArray<PathSmootherPOVLink,1024>`
/// destructor body.
pub fn fn_006b3640(e: &mut Engine, this: Ptr) {
    destroy_array(e, this, VTABLE_LINK_ARRAY);
}

/// `006b3660` / `006b3730`: appends a copy of `element`: the new index from
/// `index_of_new`, `prepare(this, address, 1)` on the slot, then
/// `copy(slot, element)`; returns the index.
fn add_element(
    e: &mut Engine,
    this: Ptr,
    element: Ptr,
    shift: u32,
    [index_of_new, prepare, copy]: [u32; 3],
) -> u32 {
    let index = e.call(index_of_new, &args![this]).u32();
    let buffer = e.mem.u32(this.addr() + 4);
    let slot = (index << shift).wrapping_add(buffer);
    e.call(prepare, &args![this, slot, 1u32]);
    let buffer = e.mem.u32(this.addr() + 4);
    let slot = (index << shift).wrapping_add(buffer);
    e.call(copy, &args![slot, element]);
    index
}

// Translated from 006b3660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: appends a copy of the `PathSmootherPOVLink`
/// at `link` to the link array (`006b3fd0`, `006a2630`, `006b3910`);
/// returns its index.
pub fn fn_006b3660(e: &mut Engine, this: Ptr, link: Ptr) -> u32 {
    add_element(e, this, link, 6, [0x006b_3fd0, 0x006a_2630, 0x006b_3910])
}

// Translated from 006b36b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the link array's buffer allocation:
/// `00401000(count x 0x40)`.
pub fn fn_006b36b0(e: &mut Engine, _this: Ptr, count: u32) -> u32 {
    e.call(ALLOCATE, &args![count << 6]).u32()
}

// Translated from 006b36e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the
/// `BSSimpleArray<PathSmootherPOVSearch::PathEdgeVert,1024>` constructor
/// (vtable, `006b4190(0, 0)`). Returns `this`.
pub fn fn_006b36e0(e: &mut Engine, this: Ptr) -> Ptr {
    construct_array(e, this, VTABLE_EDGE_VERT_ARRAY, Some(0x006b_4190))
}

// Translated from 006b3710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `PathEdgeVert` array's destructor body.
pub fn fn_006b3710(e: &mut Engine, this: Ptr) {
    destroy_array(e, this, VTABLE_EDGE_VERT_ARRAY);
}

// Translated from 006b3730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: appends a copy of the `PathEdgeVert` at
/// `vert` (`006b4060`, `006b40f0`, `006b39a0`); returns its index.
pub fn fn_006b3730(e: &mut Engine, this: Ptr, vert: Ptr) -> u32 {
    add_element(e, this, vert, 5, [0x006b_4060, 0x006b_40f0, 0x006b_39a0])
}

// Translated from 006b3780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `PathEdgeVert` array's buffer allocation:
/// `00401000(count x 0x20)`.
pub fn fn_006b3780(e: &mut Engine, _this: Ptr, count: u32) -> u32 {
    e.call(ALLOCATE, &args![count << 5]).u32()
}

// Translated from 006b37b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a holder of a reference-counted pointer:
/// stores `object` and, when it is not null, increments its count at +0x10
/// (`0040f6e0`). Returns `this`.
pub fn fn_006b37b0(e: &mut Engine, this: Ptr, object: u32) -> Ptr {
    e.mem.set_u32(this.addr(), object);
    if e.mem.u32(this.addr()) != 0 {
        let held = e.mem.u32(this.addr());
        e.call(0x0040_f6e0, &args![held + 0x10]);
    }
    this
}

// Translated from 006b37e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: that holder's destructor: when the pointer is
/// not null, `00401970` on its count at +0x10.
pub fn fn_006b37e0(e: &mut Engine, this: Ptr) {
    let held = e.mem.u32(this.addr());
    if held != 0 {
        let held = e.mem.u32(this.addr());
        e.call(0x0040_1970, &args![held + 0x10]);
    }
}

// Translated from 006b3800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSSimpleArray<NavMesh*,1024>`
/// constructor (vtable, `006b3eb0(0, 0)`). Returns `this`.
pub fn fn_006b3800(e: &mut Engine, this: Ptr) -> Ptr {
    construct_array(e, this, VTABLE_NAVMESH_ARRAY, None)
}

// Translated from 006b3830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSSimpleArray<NavMesh*,1024>` destructor
/// body (vtable, `008454f0(1)`).
pub fn fn_006b3830(e: &mut Engine, this: Ptr) {
    destroy_array(e, this, VTABLE_NAVMESH_ARRAY);
}

/// The scalar deleting destructors' tail: frees `this` (`00401030`) when
/// bit 0 of `flags` is set.
fn free_if_flagged(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 006b3850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AStarSearch<PathSmootherPOVNode,PathSmootherPOVLink>::`scalar deleting
/// destructor'` (Xbox PDB): the destructor `006b3270`, then frees the
/// object when bit 0 of `flags` is set; returns `this`.
pub fn astar_search_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<AStarSearch>,
    flags: u32,
) -> Ptr<AStarSearch> {
    fn_006b3270(e, this);
    free_if_flagged(e, this.cast(), flags).cast()
}

// Translated from 006b3880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<PathSmootherPOV*,1024>::`scalar deleting destructor'`
/// (Xbox PDB): the destructor `006b35a0`, then frees the object when bit 0
/// of `flags` is set; returns `this`.
pub fn pov_array_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006b35a0(e, this);
    free_if_flagged(e, this, flags)
}

// Translated from 006b38b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<PathSmootherPOVNode,1024>::`scalar deleting destructor'`
/// (Xbox PDB): the destructor `006b35f0`, then frees the object when bit 0
/// of `flags` is set; returns `this`.
pub fn pov_node_array_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006b35f0(e, this);
    free_if_flagged(e, this, flags)
}

// Translated from 006b38e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<PathSmootherPOVLink,1024>::`scalar deleting destructor'`
/// (Xbox PDB): the destructor `006b3640`, then frees the object when bit 0
/// of `flags` is set; returns `this`.
pub fn pov_link_array_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006b3640(e, this);
    free_if_flagged(e, this, flags)
}

// Translated from 006b3910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `PathSmootherPOVLink` copy: `006985f0`
/// on the two 8-byte parts (+0, +8) and `006cf900` on the two locations
/// (+0x10, +0x28), each from the same part of `other`. Returns `this`.
pub fn fn_006b3910(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    let (p, o) = (this.addr(), other.addr());
    e.call(0x0069_85f0, &args![p, o]);
    e.call(0x0069_85f0, &args![p + 8, o + 8]);
    e.call(0x006c_f900, &args![p + 0x10, o + 0x10]);
    e.call(0x006c_f900, &args![p + 0x28, o + 0x28]);
    this
}

// Translated from 006b3970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<PathSmootherPOVSearch::PathEdgeVert,1024>::`scalar
/// deleting destructor'` (Xbox PDB): the destructor `006b3710`, then frees
/// the object when bit 0 of `flags` is set; returns `this`.
pub fn edge_vert_array_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006b3710(e, this);
    free_if_flagged(e, this, flags)
}

// Translated from 006b39a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `PathEdgeVert` copy: the vertex handle
/// (`0069a690`), the edge handle at +8 (`006bc870`), then the words
/// `iOtherSideIndex`, `iThisSideIndex` and `iPOVIndex`. Returns `this`.
pub fn fn_006b39a0(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    let (p, o) = (this.addr(), other.addr());
    e.call(VERT_HANDLE_COPY, &args![p, o]);
    e.call(EDGE_HANDLE_COPY, &args![p + 8, o + 8]);
    for at in [0x14, 0x18, 0x1c] {
        let w = e.mem.u32(o + at);
        e.mem.set_u32(p + at, w);
    }
    this
}

// Translated from 006b3a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NavMesh*,1024>::`scalar deleting destructor'` (Xbox PDB):
/// the destructor `006b3830`, then frees the object when bit 0 of `flags`
/// is set; returns `this`.
pub fn navmesh_array_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006b3830(e, this);
    free_if_flagged(e, this, flags)
}

// Translated from 006b3a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the A* search loop. With a scrap array of
/// node pointers on the stack (`006b4220`, destroyed with `006b42f0` on
/// every way out), it takes the best node from the queue (`006b4350`) into
/// `pLastSearchedNode` (none: returns null) and makes it `BestNodeSoFar`.
/// Then, while there is a node: when virtual slot +8 (`IsGoalNode`, Xbox
/// PDB) says it is the goal, it becomes `BestNodeSoFar` and is returned;
/// otherwise the array is cleared (`008454f0(0)`), filled by virtual slot
/// +4 (`GetNodeConnections`) and each child is taken out of the queue
/// (`006a1fb0`), returns null when it is the node's own parent, else gets
/// the node as parent, is queued (`006b3e10`) and becomes `BestNodeSoFar`
/// when its value (`006b9130`) is below the best one's (NaN: not below).
/// In a tasklet it sleeps (`0040fca0(0)`) every 50 nodes. When the queue
/// runs out, `pLastSearchedNode` gets back the last node searched and the
/// result is null.
// Translation incomplete: 006b3a30 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn fn_006b3a30(e: &mut Engine, this: Ptr<AStarSearch>) -> u32 {
    let queue = Ptr::new(this.addr() + SEARCH_QUEUE);
    e.with_stack(0x14, |e, scrap| {
        fn_006b4220(e, scrap);
        let result = search_loop(e, this, queue, scrap);
        fn_006b42f0(e, scrap);
        result
    })
}

/// The body of `006b3a30` between the scrap array's constructor and
/// destructor.
fn search_loop(e: &mut Engine, this: Ptr<AStarSearch>, queue: Ptr, scrap: Ptr) -> u32 {
    let p = this.addr();
    let mut last = e.get(this, AStarSearch::pLastSearchedNode);
    let first = fn_006b4350(e, queue);
    e.set(this, AStarSearch::pLastSearchedNode, first);
    if first == 0 {
        return 0;
    }
    e.set(this, AStarSearch::BestNodeSoFar, first);
    let mut steps = 0u32;
    loop {
        let node = e.get(this, AStarSearch::pLastSearchedNode);
        if node == 0 {
            break;
        }
        if e.vcall(p, 8, &args![node]).u8() != 0 {
            e.set(this, AStarSearch::BestNodeSoFar, node);
            return node;
        }
        e.call(ARRAY_CLEAR, &args![scrap, 0u32]);
        e.vcall(p, 4, &args![node, scrap]);
        let mut index = 0u32;
        while index < size(e, scrap.addr()) {
            let slot = e.call(ARRAY_WORD_AT, &args![scrap, index]).u32();
            let child = e.mem.u32(slot);
            e.call(0x006a_1fb0, &args![queue, child]);
            let node = e.get(this, AStarSearch::pLastSearchedNode);
            let parent = e.get(
                Ptr::<PovAStarNode>::new(node),
                PovAStarNode::pParentAStarNode,
            );
            if parent == child {
                return 0;
            }
            let child_node = Ptr::<PovAStarNode>::new(child);
            e.set(child_node, PovAStarNode::pParentAStarNode, node);
            fn_006b3e10(e, queue, Ptr::new(child));
            let child_value = e.call(0x006b_9130, &args![child]).f64();
            let best = e.get(this, AStarSearch::BestNodeSoFar);
            let best_value = e.call(0x006b_9130, &args![best]).f64();
            if less(child_value, best_value) {
                e.set(this, AStarSearch::BestNodeSoFar, child);
            }
            index += 1;
        }
        if e.get(this, AStarSearch::bIsRunningInTasklet) != 0 {
            steps += 1;
            if steps == 0x32 {
                steps = 0;
                e.call(0x0040_fca0, &args![0u32]);
            }
        }
        last = e.get(this, AStarSearch::pLastSearchedNode);
        let next = fn_006b4350(e, queue);
        e.set(this, AStarSearch::pLastSearchedNode, next);
    }
    e.set(this, AStarSearch::pLastSearchedNode, last);
    0
}

// Translated from 006b3cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: builds the path ending at `node`: counts the
/// nodes up the parent chain, sizes `nodes` to that count (`006a8890(count,
/// 1)`) and `links` to one less (`006b4450(count - 1, 1)`), then walks the
/// chain again from the last index down: each node's 12-byte `NodeData` is
/// copied into `nodes` (`006a1440`) and, above index 0, its `ParentLink`
/// into `links[index - 1]` (`006b3910`). False for a null `node`, true
/// otherwise.
pub fn fn_006b3cb0(
    e: &mut Engine,
    _this: Ptr<AStarSearch>,
    node: Ptr<PovAStarNode>,
    nodes: Ptr,
    links: Ptr,
) -> bool {
    if node.addr() == 0 {
        return false;
    }
    let mut count = 0u32;
    let mut at = node;
    while at.addr() != 0 {
        count = count.wrapping_add(1);
        at = Ptr::new(e.get(at, PovAStarNode::pParentAStarNode));
    }
    e.call(0x006a_8890, &args![nodes, count, 1u32]);
    fn_006b4450(e, links, count.wrapping_sub(1), 1);
    let mut at = node;
    let mut index = count.wrapping_sub(1);
    while at.addr() != 0 {
        let data = at.addr() + 8;
        let slot = e.call(0x006a_1440, &args![nodes, index]).u32();
        copy3(e, slot, data);
        if (index as i32) > 0 {
            let link = fn_006b30e0(e, links, index.wrapping_sub(1));
            fn_006b3910(e, link, Ptr::new(at.addr() + 0x14));
        }
        at = Ptr::new(e.get(at, PovAStarNode::pParentAStarNode));
        index = index.wrapping_sub(1);
    }
    true
}

// Translated from 006b3d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a new A* node from the node heap: 0x68 bytes
/// from `006a1f90(0x68, heap)` (cdecl) and, when not null, the node
/// constructor `006b43f0`; null otherwise.
pub fn fn_006b3d90(e: &mut Engine, this: Ptr) -> u32 {
    let block = e.call(0x006a_1f90, &args![0x68u32, this]).u32();
    if block != 0 {
        fn_006b43f0(e, Ptr::new(block)).addr()
    } else {
        0
    }
}

// Translated from 006b3de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `AStarMap` constructor: `iCount` (+0x1fdc,
/// Xbox PDB) 0, then the 0x1fdc bytes of buckets zeroed with `00403d30`
/// (`memset`, cdecl). Returns `this`.
pub fn fn_006b3de0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr() + 0x1fdc, 0);
    e.call(0x0040_3d30, &args![this, 0u32, 0x1fdcu32]);
    this
}

// Translated from 006b3e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: queues `node` in the `AStarQueue` (Xbox PDB):
/// the bucket index comes from `006f4790(node)`; the node goes before the
/// first node of the bucket's list whose value (`006b9710`) is not below
/// its own (NaN included), linked through `pNextNode` / `ppPrevNode`.
pub fn fn_006b3e10(e: &mut Engine, this: Ptr, node: Ptr<PovAStarNode>) {
    let index = e.call(0x006f_4790, &args![this, node]).u32();
    let mut slot = this
        .addr()
        .wrapping_add(index.wrapping_mul(4))
        .wrapping_add(4);
    loop {
        let current = e.mem.u32(slot);
        if current == 0 {
            break;
        }
        let current_value = e.call(0x006b_9710, &args![current]).f64();
        let node_value = e.call(0x006b_9710, &args![node]).f64();
        // FCOMP; TEST AH,0x41: on only while the node's value is above.
        if !less(current_value, node_value) {
            break;
        }
        slot = current + 0x5c;
    }
    let current = e.mem.u32(slot);
    if current != 0 {
        e.set(
            Ptr::<PovAStarNode>::new(current),
            PovAStarNode::ppPrevNode,
            node.addr() + 0x5c,
        );
    }
    e.set(node, PovAStarNode::pNextNode, current);
    e.set(node, PovAStarNode::ppPrevNode, slot);
    e.mem.set_u32(slot, node.addr());
}

// Translated from 006b3f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSSimpleArray<PathSmootherPOVNode,1024>`
/// initialiser: the `006b3eb0` shape, with `006d03d0(buffer, size)` preparing
/// the elements.
pub fn fn_006b3f40(e: &mut Engine, this: Ptr<BSSimpleArray>, reserved_hint: u32, size: u32) {
    init_array(e, this, reserved_hint, size, |e, buffer| {
        e.call(0x006d_03d0, &args![this, buffer, size]);
    });
}

/// `006b3fd0` / `006b4060`: the index of a new last element. When the
/// array is full (`00438b90`): without a reserve the buffer comes from
/// virtual slot +4 with 4 and the reserve is 4; otherwise the new reserve
/// from `009a3910` goes to `grow(capacity, size)` and to +0xc. Then the size
/// grows by one; returns the old size.
fn index_of_new(e: &mut Engine, this: Ptr, grow: impl FnOnce(&mut Engine, u32, u32)) -> u32 {
    let p = this.addr();
    if e.call(0x0043_8b90, &args![this]).u8() != 0 {
        if e.mem.u32(p + 0xc) == 0 {
            let buffer = e.vcall(p, 4, &args![4u32]).u32();
            e.mem.set_u32(p + 4, buffer);
            e.mem.set_u32(p + 0xc, 4);
        } else {
            let capacity = e.call(0x009a_3910, &args![this]).u32();
            let size = e.mem.u32(p + 8);
            grow(e, capacity, size);
            e.mem.set_u32(p + 0xc, capacity);
        }
    }
    let size = e.mem.u32(p + 8).wrapping_add(1);
    e.mem.set_u32(p + 8, size);
    size.wrapping_sub(1)
}

// Translated from 006b3fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the link array's new-element index (see
/// [`index_of_new`]; grows with `006b4590`).
pub fn fn_006b3fd0(e: &mut Engine, this: Ptr) -> u32 {
    index_of_new(e, this, |e, capacity, size| {
        fn_006b4590(e, this, capacity, size);
    })
}

// Translated from 006b4060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `PathEdgeVert` array's new-element index
/// (see [`index_of_new`]; grows with `006f9eb0`).
pub fn fn_006b4060(e: &mut Engine, this: Ptr) -> u32 {
    index_of_new(e, this, |e, capacity, size| {
        e.call(0x006f_9eb0, &args![this, capacity, size]);
    })
}

// Translated from 006b40f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: constructs `count` `PathEdgeVert`s in place
/// from `buffer`: each 0x20-byte slot through `006e6da0(0x20, slot)`
/// (cdecl, the placement step) and, when that is not null, the constructor
/// `006aeb70`. `this` is not read.
// Translation incomplete: 006b40f0 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn fn_006b40f0(e: &mut Engine, _this: Ptr, buffer: u32, count: u32) {
    let mut index = 0u32;
    while index < count {
        let slot = (index << 5).wrapping_add(buffer);
        let placed = e.call(0x006e_6da0, &args![0x20u32, slot]).u32();
        if placed != 0 {
            fn_006aeb70(e, Ptr::new(placed));
        }
        index += 1;
    }
}

// Translated from 006b4190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `PathEdgeVert` array initialiser: the
/// `006b3eb0` shape, with `006b40f0(buffer, size)` constructing the
/// elements.
pub fn fn_006b4190(e: &mut Engine, this: Ptr<BSSimpleArray>, reserved_hint: u32, size: u32) {
    init_array(e, this, reserved_hint, size, |e, buffer| {
        fn_006b40f0(e, this.cast(), buffer, size);
    });
}

// Translated from 006b4220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the
/// `BSScrapArray<AStarNode<PathSmootherPOVNode,PathSmootherPOVLink>*,1024>`
/// constructor: the base constructor `006b4670`, the class's vtable, the
/// thread's scrap heap (`MemoryManager::GetThreadScrapHeap` `00aa42e0` on
/// `00401020`'s manager) in `pScrapHeap` (+0x10, Xbox PDB), then
/// `006b3eb0(0, 0)`. Returns `this`.
// Translation incomplete: 006b4220 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn fn_006b4220(e: &mut Engine, this: Ptr) -> Ptr {
    fn_006b4670(e, this);
    e.mem.set_u32(this.addr(), VTABLE_ASTAR_NODE_SCRAP_ARRAY);
    let manager = e.call(0x0040_1020, &[]).u32();
    let heap = e.call(0x00aa_42e0, &args![manager]).u32();
    e.mem.set_u32(this.addr() + 0x10, heap);
    fn_006b3eb0(e, this.cast(), 0, 0);
    this
}

// Translated from 006b42a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the
/// `BSSimpleArray<AStarNode<PathSmootherPOVNode,PathSmootherPOVLink>*,1024>`
/// destructor body (vtable, `008454f0(1)`).
pub fn fn_006b42a0(e: &mut Engine, this: Ptr) {
    destroy_array(e, this, VTABLE_ASTAR_NODE_ARRAY);
}

// Translated from 006b42c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<AStarNode<PathSmootherPOVNode,PathSmootherPOVLink>*,1024>
/// ::`scalar deleting destructor'` (Xbox PDB): the destructor `006b42a0`,
/// then frees the object when bit 0 of `flags` is set; returns `this`.
pub fn astar_node_array_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006b42a0(e, this);
    free_if_flagged(e, this, flags)
}

// Translated from 006b42f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSScrapArray` destructor body: its
/// vtable, `008454f0(1)`, then the base destructor `006b42a0`.
// Translation incomplete: 006b42f0 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn fn_006b42f0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_ASTAR_NODE_SCRAP_ARRAY);
    e.call(ARRAY_CLEAR, &args![this, 1u32]);
    fn_006b42a0(e, this);
}

// Translated from 006b4350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: takes the head node of the first non-empty
/// bucket of the `AStarQueue` (20 buckets from +4) out of the queue: its
/// next node (with `ppPrevNode` set to the bucket) becomes the head, and
/// the node's `ppPrevNode` / `pNextNode` are cleared. Null when every
/// bucket is empty.
pub fn fn_006b4350(e: &mut Engine, this: Ptr) -> u32 {
    for index in 0..0x14u32 {
        let slot = this.addr() + 4 + index * 4;
        let head = e.mem.u32(slot);
        if head == 0 {
            continue;
        }
        let node = Ptr::<PovAStarNode>::new(head);
        let next = e.get(node, PovAStarNode::pNextNode);
        if next != 0 {
            e.set(
                Ptr::<PovAStarNode>::new(next),
                PovAStarNode::ppPrevNode,
                slot,
            );
            e.mem.set_u32(slot, next);
        } else {
            e.mem.set_u32(slot, 0);
        }
        e.set(node, PovAStarNode::ppPrevNode, 0);
        e.set(node, PovAStarNode::pNextNode, 0);
        return head;
    }
    0
}

// Translated from 006b43f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (Ghidra matches the shape to an MFC
/// `CPreviewView::PAGE_INFO` constructor): the
/// `AStarNode<PathSmootherPOVNode,PathSmootherPOVLink>` constructor: costs
/// 0.0, the parent link constructed (`006b1d80`), parent, queue links, key
/// and bucket link 0 (`NodeData` is left as it is). Returns `this`.
pub fn fn_006b43f0(e: &mut Engine, this: Ptr<PovAStarNode>) -> Ptr<PovAStarNode> {
    e.set(this, PovAStarNode::fCost, 0.0);
    e.set(this, PovAStarNode::fHeuristic, 0.0);
    fn_006b1d80(e, Ptr::new(this.addr() + 0x14));
    e.set(this, PovAStarNode::pParentAStarNode, 0);
    e.set(this, PovAStarNode::ppPrevNode, 0);
    e.set(this, PovAStarNode::pNextNode, 0);
    e.set(this, PovAStarNode::iKey, 0);
    e.set(this, PovAStarNode::pNextNodeInBucket, 0);
    this
}

// Translated from 006b4450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `SetSize` shape of the link array (0x40-byte
/// elements). 0: `008454f0(shrink)`. Above the reserve: the buffer from
/// virtual slot +4 (no reserve yet) or `006b4590(count, size)`, the reserve
/// becomes `count`, the new elements are prepared (`006a2630`). Below the
/// size: the dropped elements go through `0072ba80`, and with `shrink` and
/// `count` at most a quarter of the reserve, `006b4590(count, count)` makes
/// `count` the reserve. Otherwise the new elements are prepared
/// (`006a2630`). The size becomes `count`.
pub fn fn_006b4450(e: &mut Engine, this: Ptr, count: u32, shrink: u8) {
    let p = this.addr();
    let element = |e: &mut Engine, index: u32| (index << 6).wrapping_add(e.mem.u32(p + 4));
    if count == 0 {
        e.call(ARRAY_CLEAR, &args![this, u32::from(shrink)]);
        return;
    }
    if count > e.mem.u32(p + 0xc) {
        if e.mem.u32(p + 0xc) == 0 {
            let buffer = e.vcall(p, 4, &args![count]).u32();
            e.mem.set_u32(p + 4, buffer);
        } else {
            let size = e.mem.u32(p + 8);
            fn_006b4590(e, this, count, size);
        }
        e.mem.set_u32(p + 0xc, count);
        let size = e.mem.u32(p + 8);
        let first = element(e, size);
        e.call(0x006a_2630, &args![this, first, count.wrapping_sub(size)]);
    } else if count < e.mem.u32(p + 8) {
        let size = e.mem.u32(p + 8);
        let first = element(e, count);
        e.call(0x0072_ba80, &args![this, first, size.wrapping_sub(count)]);
        e.mem.set_u32(p + 8, count);
        if shrink != 0 && count <= e.mem.u32(p + 0xc) >> 2 {
            fn_006b4590(e, this, count, count);
            e.mem.set_u32(p + 0xc, count);
        }
        return;
    } else {
        let size = e.mem.u32(p + 8);
        let first = element(e, size);
        e.call(0x006a_2630, &args![this, first, count.wrapping_sub(size)]);
    }
    e.mem.set_u32(p + 8, count);
}

// Translated from 006b4590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the link array's buffer change to `capacity`
/// elements. Without a buffer: virtual slot +4 (`_Allocate`, Xbox PDB) and
/// the reserve becomes `capacity`. When `size` equals the reserve: virtual
/// slot +0xc (`_Reallocate`) with the buffer and `capacity`. Otherwise a
/// new buffer from slot +4, the `size` elements moved into it
/// (`004a9b50(new, old, size)`), the old buffer released (`006a8500`), and
/// the new one stored. The reserve is set by the callers in the last two
/// cases.
pub fn fn_006b4590(e: &mut Engine, this: Ptr, capacity: u32, size: u32) {
    let p = this.addr();
    let buffer = e.mem.u32(p + 4);
    if buffer == 0 {
        let buffer = e.vcall(p, 4, &args![capacity]).u32();
        e.mem.set_u32(p + 4, buffer);
        e.mem.set_u32(p + 0xc, capacity);
    } else if size == e.mem.u32(p + 0xc) {
        let buffer = e.vcall(p, 0xc, &args![buffer, capacity]).u32();
        e.mem.set_u32(p + 4, buffer);
    } else {
        let fresh = e.vcall(p, 4, &args![capacity]).u32();
        let old = e.mem.u32(p + 4);
        e.call(0x004a_9b50, &args![this, fresh, old, size]);
        e.call(0x006a_8500, &args![this]);
        e.mem.set_u32(p + 4, fresh);
    }
}

// Translated from 006b4640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapArray<AStarNode<PathSmootherPOVNode,PathSmootherPOVLink>*,1024>
/// ::`scalar deleting destructor'` (Xbox PDB): the destructor `006b42f0`,
/// then frees the object when bit 0 of `flags` is set; returns `this`.
pub fn astar_node_scrap_array_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_006b42f0(e, this);
    free_if_flagged(e, this, flags)
}

// Translated from 006b4670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the
/// `BSSimpleArray<AStarNode<PathSmootherPOVNode,PathSmootherPOVLink>*,1024>`
/// constructor (vtable, `006b3eb0(0, 0)`). Returns `this`.
pub fn fn_006b4670(e: &mut Engine, this: Ptr) -> Ptr {
    construct_array(e, this, VTABLE_ASTAR_NODE_ARRAY, None)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x006ad660,
            path_smoother_pov_search_scalar_deleting_destructor(
                Ptr<PathSmootherPOVSearch>,
                u32,
            )
                -> Ptr<PathSmootherPOVSearch>
        ),
        entry!(0x006ad690, fn_006ad690(Ptr<PathSmootherPOVSearch>)),
        entry!(
            0x006ad770,
            fn_006ad770(Ptr<PathSmootherPOVSearch>, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(0x006adcb0, fn_006adcb0(Ptr<PathSmootherPOVSearch>)),
        entry!(
            0x006ae120,
            fn_006ae120(Ptr<PathSmootherPOV>) -> Ptr<PathSmootherPOV>
        ),
        entry!(0x006ae190, fn_006ae190(Ptr<PathSmootherPOVSearch>)),
        entry!(0x006ae280, fn_006ae280(Ptr<PathSmootherPOVSearch>)),
        entry!(0x006ae370, fn_006ae370(Ptr<PathSmootherPOVSearch>)),
        entry!(
            0x006aea90,
            fn_006aea90(Ptr<PathSmootherPOVSearch>, Ptr, Ptr)
        ),
        entry!(
            0x006aeb00,
            fn_006aeb00(Ptr<PathSmootherPOVSearch>, Ptr, Ptr)
        ),
        entry!(0x006aeb70, fn_006aeb70(Ptr) -> Ptr),
        entry!(
            0x006aeba0,
            fn_006aeba0(Ptr<PathSmootherPOVSearch>, Ptr, u32, u8)
        ),
        entry!(0x006aefc0, fn_006aefc0(Ptr<PathSmootherPOVSearch>)),
        entry!(0x006af2b0, fn_006af2b0(Ptr<PathSmootherPOVSearch>)),
        entry!(0x006af500, fn_006af500(Ptr<PathSmootherPOVSearch>)),
        entry!(0x006afcc0, fn_006afcc0(Ptr<PathSmootherPOVSearch>, u8)),
        entry!(0x006b0570, fn_006b0570(Ptr, Ptr)),
        entry!(0x006b05a0, fn_006b05a0(Ptr) -> Ptr),
        entry!(0x006b05d0, fn_006b05d0(Ptr) -> u8),
        entry!(
            0x006b05f0,
            fn_006b05f0(Ptr<PathSmootherPOVSearch>, u32, u16, Ptr, Ptr)
        ),
        entry!(
            0x006b0a60,
            fn_006b0a60(Ptr<PathSmootherPOVSearch>, Ptr, Ptr, u32) -> Ptr
        ),
        entry!(
            0x006b0ad0,
            path_smoother_pov_search_get_node_connections(
                Ptr<PathSmootherPOVSearch>,
                Ptr,
                Ptr,
            ) -> bool
        ),
        entry!(
            0x006b0f40,
            fn_006b0f40(Ptr<PathSmootherPOVSearch>, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(0x006b1970, fn_006b1970(Ptr<PathSmootherPOVSearch>) -> bool),
        entry!(0x006b1d80, fn_006b1d80(Ptr) -> Ptr),
        entry!(
            0x006b1dc0,
            path_smoother_pov_search_is_goal_node(Ptr<PathSmootherPOVSearch>, Ptr) -> bool
        ),
        entry!(
            0x006b1df0,
            fn_006b1df0(Ptr<PathSmootherPOVSearch>, Ptr, Ptr, u32)
        ),
        (0x006b1e50, fn_006b1e50_entry as AbiFn),
        entry!(
            0x006b2060,
            path_smoother_pov_search_check_segment_against_nav_mesh(
                Ptr<PathSmootherPOVSearch>,
                Ptr,
                Ptr,
                Ptr,
                Ptr,
            ) -> bool
        ),
        (0x006b23d0, fn_006b23d0_entry as AbiFn),
        entry!(
            0x006b25f0,
            path_smoother_pov_search_triple_check_segment_against_nav_mesh(
                Ptr<PathSmootherPOVSearch>,
                Ptr,
                Ptr,
                Ptr,
                u8,
                u8,
                Ptr,
            ) -> bool
        ),
        entry!(
            0x006b2c00,
            fn_006b2c00(Ptr<PathSmootherPOVSearch>, Ptr, Ptr, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(
            0x006b2fc0,
            fn_006b2fc0(Ptr<PathSmootherPOVSearch>, Ptr) -> bool
        ),
        entry!(0x006b30e0, fn_006b30e0(Ptr, u32) -> Ptr),
        entry!(0x006b3100, fn_006b3100(Ptr) -> Ptr),
        entry!(0x006b3120, fn_006b3120(Ptr, u32) -> Ptr),
        entry!(0x006b3140, fn_006b3140(Ptr) -> Ptr),
        entry!(0x006b3160, fn_006b3160(Ptr, u32) -> Ptr),
        entry!(0x006b3180, fn_006b3180(Ptr) -> Ptr),
        entry!(0x006b31a0, fn_006b31a0(Ptr) -> Ptr),
        entry!(
            0x006b31c0,
            fn_006b31c0(Ptr<AStarSearch>) -> Ptr<AStarSearch>
        ),
        entry!(0x006b3270, fn_006b3270(Ptr<AStarSearch>)),
        entry!(
            0x006b32f0,
            fn_006b32f0(Ptr<AStarSearch>, u32, Ptr, f32) -> u32
        ),
        entry!(
            0x006b3380,
            fn_006b3380(Ptr<AStarSearch>, Ptr, Ptr, u8) -> bool
        ),
        entry!(
            0x006b3420,
            fn_006b3420(Ptr<AStarSearch>, u32, f32, Ptr) -> u32
        ),
        entry!(0x006b3570, fn_006b3570(Ptr) -> Ptr),
        entry!(0x006b35a0, fn_006b35a0(Ptr)),
        entry!(0x006b35c0, fn_006b35c0(Ptr) -> Ptr),
        entry!(0x006b35f0, fn_006b35f0(Ptr)),
        entry!(0x006b3610, fn_006b3610(Ptr) -> Ptr),
        entry!(0x006b3640, fn_006b3640(Ptr)),
        entry!(0x006b3660, fn_006b3660(Ptr, Ptr) -> u32),
        entry!(0x006b36b0, fn_006b36b0(Ptr, u32) -> u32),
        entry!(0x006b36e0, fn_006b36e0(Ptr) -> Ptr),
        entry!(0x006b3710, fn_006b3710(Ptr)),
        entry!(0x006b3730, fn_006b3730(Ptr, Ptr) -> u32),
        entry!(0x006b3780, fn_006b3780(Ptr, u32) -> u32),
        entry!(0x006b37b0, fn_006b37b0(Ptr, u32) -> Ptr),
        entry!(0x006b37e0, fn_006b37e0(Ptr)),
        entry!(0x006b3800, fn_006b3800(Ptr) -> Ptr),
        entry!(0x006b3830, fn_006b3830(Ptr)),
        entry!(
            0x006b3850,
            astar_search_scalar_deleting_destructor(Ptr<AStarSearch>, u32) -> Ptr<AStarSearch>
        ),
        entry!(
            0x006b3880,
            pov_array_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x006b38b0,
            pov_node_array_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x006b38e0,
            pov_link_array_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x006b3910, fn_006b3910(Ptr, Ptr) -> Ptr),
        entry!(
            0x006b3970,
            edge_vert_array_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x006b39a0, fn_006b39a0(Ptr, Ptr) -> Ptr),
        entry!(
            0x006b3a00,
            navmesh_array_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x006b3a30, fn_006b3a30(Ptr<AStarSearch>) -> u32),
        entry!(
            0x006b3cb0,
            fn_006b3cb0(Ptr<AStarSearch>, Ptr<PovAStarNode>, Ptr, Ptr) -> bool
        ),
        entry!(0x006b3d90, fn_006b3d90(Ptr) -> u32),
        entry!(0x006b3de0, fn_006b3de0(Ptr) -> Ptr),
        entry!(0x006b3e10, fn_006b3e10(Ptr, Ptr<PovAStarNode>)),
        entry!(0x006b3eb0, fn_006b3eb0(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x006b3f40, fn_006b3f40(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x006b3fd0, fn_006b3fd0(Ptr) -> u32),
        entry!(0x006b4060, fn_006b4060(Ptr) -> u32),
        entry!(0x006b40f0, fn_006b40f0(Ptr, u32, u32)),
        entry!(0x006b4190, fn_006b4190(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x006b4220, fn_006b4220(Ptr) -> Ptr),
        entry!(0x006b42a0, fn_006b42a0(Ptr)),
        entry!(
            0x006b42c0,
            astar_node_array_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x006b42f0, fn_006b42f0(Ptr)),
        entry!(0x006b4350, fn_006b4350(Ptr) -> u32),
        entry!(
            0x006b43f0,
            fn_006b43f0(Ptr<PovAStarNode>) -> Ptr<PovAStarNode>
        ),
        entry!(0x006b4450, fn_006b4450(Ptr, u32, u8)),
        entry!(0x006b4590, fn_006b4590(Ptr, u32, u32)),
        entry!(
            0x006b4640,
            astar_node_scrap_array_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x006b4670, fn_006b4670(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn point(e: &Engine, a: u32) -> [f32; 3] {
        [e.mem.f32(a), e.mem.f32(a + 4), e.mem.f32(a + 8)]
    }

    fn set_point(e: &mut Engine, a: u32, p: [f32; 3]) {
        for (k, v) in p.iter().enumerate() {
            e.mem.set_f32(a + 4 * k as u32, *v);
        }
    }

    fn arg_f(w: u32) -> f32 {
        f32::from_bits(w)
    }

    /// The argument lists of the logged calls to `addr`.
    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// Gives the `BSSimpleArray` at `at` a buffer of `capacity` bytes.
    fn buffer(e: &mut Engine, at: u32, capacity: u32) -> u32 {
        let b = e.mem.alloc(capacity);
        e.mem.set_u32(at + 4, b);
        b
    }

    /// Appends `bytes` bytes from `src` to the array at `at`.
    fn push(e: &mut Engine, at: u32, src: &[u8]) {
        if e.mem.u32(at + 4) == 0 {
            buffer(e, at, 0x400);
        }
        let b = e.mem.u32(at + 4);
        let n = e.mem.u32(at + 8);
        e.mem.write(b + n * src.len() as u32, src);
        e.mem.set_u32(at + 8, n + 1);
    }

    fn push_word(e: &mut Engine, at: u32, w: u32) {
        push(e, at, &w.to_le_bytes());
    }

    fn element(e: &Engine, at: u32, index: u32, stride: u32) -> u32 {
        e.mem.u32(at + 4) + index * stride
    }

    fn word_at(e: &Engine, at: u32, index: u32) -> u32 {
        e.mem.u32(element(e, at, index, 4))
    }

    const F32S: &[(u32, u32)] = &[
        (HALF, 0x3f00_0000),
        (HUNDRED, 0x42c8_0000),
        (FIFTH, 0x3e4c_cccd),
        (SEARCH_BELOW, 0x4280_0000),
        (SEARCH_ABOVE, 0x4334_0000),
        (TEN, 0x4120_0000),
        (OFFSET_FACTOR, 0x3f81_47ae),
        (STEP_FACTOR, 0x3c23_d70a),
        (0x0101_ff38, 0x3fc9_0fdb),
        (0x0101_6b78, 0xbfc9_0fdb),
        (0x0101_7718, 0x4040_0000),
        (0x0102_b3c8, 0x4049_0fdb),
        (0x0106_b648, 0xc049_0fdb),
        (0x0106_4f64, 0x3e86_0a92),
        (0x0106_b3a8, 0xbe86_0a92),
        (0x0101_6970, 0x7f7f_ffff),
        (0x0101_77e8, 0x3f66_6666),
    ];

    const F64S: &[(u32, u64)] = &[
        (D_0_4, 0x3fd9_9999_a000_0000),
        (D_0_25, 0x3fd0_0000_0000_0000),
        (D_0_75, 0x3fe8_0000_0000_0000),
        (D_50, 0x4049_0000_0000_0000),
        (D_ZERO, 0),
        (D_0_05, 0x3fa9_9999_a000_0000),
        (D_MINUS_0_05, 0xbfa9_9999_a000_0000),
        (D_1_2, 0x3ff3_3333_4000_0000),
        (D_0_1, 0x3fb9_9999_a000_0000),
        (D_0_9, 0x3fec_cccc_c000_0000),
        (D_1024, 0x4090_0000_0000_0000),
        (0x0101_db80, 0x4010_0000_0000_0000),
        (0x0101_7a40, 0x4059_0000_0000_0000),
        (0x0101_1588, 0x3fe0_0000_0000_0000),
        (0x0101_ff48, 0x4019_21fb_6000_0000),
        (0x0106_ba00, 0x400b_eede_6000_0000),
        (0x0106_b9f8, 0xc00b_eede_6000_0000),
        (0x0104_ed58, 0x4066_8000_0000_0000),
        (0x0106_b9f0, 0xc050_0000_0000_0000),
        (0x0106_4f50, 0x3ff9_21fb_5444_2d18),
        (0x0101_6978, 0x3f50_624d_e000_0000),
        (0x0102_0758, 0x4024_0000_0000_0000),
    ];

    /// The tasklet manager object `00b00a00` returns in the tests.
    const MANAGER: u32 = 0x4600_0000;
    /// Where `00699af0` points (a closed-door info word).
    const DOOR_INFO: u32 = 0x4600_0100;
    /// The vertex table `0068f0a0` indexes (12 bytes per vertex).
    const VERTICES: u32 = 0x4600_1000;

    fn constants(e: &mut Engine) {
        for &(addr, bits) in F32S {
            e.map(addr, 4);
            e.mem.set_u32(addr, bits);
        }
        for &(addr, bits) in F64S {
            e.map(addr, 8);
            e.mem.set_u64(addr, bits);
        }
        e.map(ZERO_POINT, 12);
        e.map(UP_AXIS, 12);
        set_point(e, UP_AXIS, [0.0, 0.0, 1.0]);
        e.map(TES_SINGLETON, 4);
        e.mem.set_u32(TES_SINGLETON, 0x4600_0200);
        e.map(MANAGER, 0x200);
        e.mem.set_u32(MANAGER, MANAGER + 0x40);
        e.mem.set_u32(MANAGER + 0x40 + 0xc, 0x4400_0010);
        e.mem.set_u32(DOOR_INFO, 0xabc);
        e.map(VERTICES, 0x400);
    }

    fn noop(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn copy_bytes(e: &mut Engine, dst: u32, src: u32, n: u32) {
        let b = e.mem.bytes(src, n);
        e.mem.write(dst, &b);
    }

    fn binary(e: &mut Engine, a: &[u32], f: fn(f32, f32) -> f32) -> Ret {
        let (p, q) = (point(e, a[0]), point(e, a[2]));
        set_point(e, a[1], [f(p[0], q[0]), f(p[1], q[1]), f(p[2], q[2])]);
        ret(a[1])
    }

    fn cross(p: [f32; 3], q: [f32; 3]) -> [f32; 3] {
        [
            p[1] * q[2] - p[2] * q[1],
            p[2] * q[0] - p[0] * q[2],
            p[0] * q[1] - p[1] * q[0],
        ]
    }

    fn length(p: [f32; 3]) -> f32 {
        (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt()
    }

    fn unit(p: [f32; 3]) -> [f32; 3] {
        let l = length(p);
        if l <= 1e-6 {
            [0.0; 3]
        } else {
            [p[0] / l, p[1] / l, p[2] / l]
        }
    }

    /// Point arithmetic doubles that behave like the game's helpers.
    fn point_math(e: &mut Engine) {
        e.register(POINT_ADD, |e, a| binary(e, a, |x, y| x + y));
        e.register(POINT_SUB, |e, a| binary(e, a, |x, y| x - y));
        e.register(POINT_SCALE, |e, a| {
            let (p, f) = (point(e, a[0]), arg_f(a[2]));
            set_point(e, a[1], [p[0] * f, p[1] * f, p[2] * f]);
            ret(a[1])
        });
        e.register(POINT_LENGTH, |e, a| st0(f64::from(length(point(e, a[0])))));
        e.register(POINT_LENGTH_SQUARED, |e, a| {
            let l = length(point(e, a[0]));
            st0(f64::from(l * l))
        });
        e.register(0x0059_5c80, |e, a| {
            let p = point(e, a[0]);
            st0(f64::from(p[0] * p[0] + p[1] * p[1]))
        });
        e.register(POINT_CROSS, |e, a| {
            let c = cross(point(e, a[0]), point(e, a[2]));
            set_point(e, a[1], c);
            ret(a[1])
        });
        e.register(POINT_UNIT_CROSS, |e, a| {
            let c = unit(cross(point(e, a[0]), point(e, a[2])));
            set_point(e, a[1], c);
            ret(a[1])
        });
        e.register(POINT_DOT, |e, a| {
            let (p, q) = (point(e, a[0]), point(e, a[1]));
            st0(f64::from(p[0] * q[0] + p[1] * q[1] + p[2] * q[2]))
        });
        e.register(POINT_NORMALIZE, |e, a| {
            let u = unit(point(e, a[0]));
            set_point(e, a[0], u);
            Ret::default()
        });
        e.register(0x0045_7910, |e, a| {
            let p = point(e, a[0]);
            set_point(e, a[0], unit(p));
            st0(f64::from(length(p)))
        });
        e.register(POINT_ADD_ASSIGN, |e, a| {
            let (p, q) = (point(e, a[0]), point(e, a[1]));
            set_point(e, a[0], [p[0] + q[0], p[1] + q[1], p[2] + q[2]]);
            ret(a[0])
        });
        e.register(0x0045_78c0, |e, a| {
            let (p, q) = (point(e, a[0]), point(e, a[1]));
            set_point(e, a[0], [p[0] - q[0], p[1] - q[1], p[2] - q[2]]);
            ret(a[0])
        });
        e.register(POINT_SCALE_ASSIGN, |e, a| {
            let (p, f) = (point(e, a[0]), arg_f(a[1]));
            set_point(e, a[0], [p[0] * f, p[1] * f, p[2] * f]);
            ret(a[0])
        });
        e.register(0x0041_6870, |e, a| {
            set_point(e, a[0], [arg_f(a[1]), arg_f(a[2]), arg_f(a[3])]);
            ret(a[0])
        });
        e.register(0x004a_0bd0, |e, a| {
            let p = point(e, a[0]);
            set_point(e, a[1], [-p[0], -p[1], -p[2]]);
            ret(a[1])
        });
        e.register(0x0040_19b0, |_, a| st0(f64::from(arg_f(a[0]).sqrt())));
        e.register(0x0040_8840, |_, a| st0(f64::from(arg_f(a[0]).abs())));
        e.register(0x0069_e0c0, |e, a| {
            let (x, y, r) = (e.mem.f32(a[0]), e.mem.f32(a[0] + 4), arg_f(a[2]));
            set_point(e, a[1], [r * x, r * y, 0.0]);
            ret(a[1])
        });
        e.register(0x006a_04e0, |e, a| {
            let v = arg_f(a[1]);
            e.mem.set_f32(a[0], v.cos());
            e.mem.set_f32(a[0] + 4, v.sin());
            ret(a[0])
        });
        e.register(0x0053_30e0, |e, a| {
            let v = e.mem.f32(a[0]).clamp(arg_f(a[1]), arg_f(a[2]));
            e.mem.set_f32(a[0], v);
            Ret::default()
        });
        e.register(0x004e_4470, |_, a| st0(f64::from(arg_f(a[0]).cos())));
        e.register(0x004e_44b0, |_, a| st0(f64::from(arg_f(a[0]).sin())));
    }

    /// Container and record doubles.
    fn containers(e: &mut Engine) {
        e.register(ARRAY_SIZE, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(ARRAY_IS_EMPTY, |e, a| {
            ret(u32::from(e.mem.u32(a[0] + 8) == 0))
        });
        e.register(ARRAY_WORD_AT, |e, a| ret(element(e, a[0], a[1], 4)));
        e.register(ARRAY_APPEND_WORD, |e, a| {
            let w = e.mem.u32(a[1]);
            push_word(e, a[0], w);
            Ret::default()
        });
        e.register(ADD_RECORD, |e, a| {
            let bytes = e.mem.bytes(a[1], 12);
            push(e, a[0], &bytes);
            Ret::default()
        });
        e.register(ARRAY_COUNT_0084E3A0, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(NODE_ARRAY_LAST, |e, a| {
            let n = e.mem.u32(a[0] + 8);
            ret(element(e, a[0], n - 1, 12))
        });
        e.register(0x006a_1440, |e, a| ret(element(e, a[0], a[1], 12)));
        e.register(EDGE_ARRAY_AT, |e, a| ret(element(e, a[0], a[1], 12)));
        e.register(TRIANGLE_ARRAY_AT, |e, a| ret(element(e, a[0], a[1], 0x14)));
        for addr in [0x006b_3fd0, 0x006b_4060] {
            e.register(addr, |e, a| {
                let n = e.mem.u32(a[0] + 8);
                e.mem.set_u32(a[0] + 8, n + 1);
                ret(n)
            });
        }
        e.register(0x006b_3910, |e, a| {
            copy_bytes(e, a[0], a[1], 0x40);
            Ret::default()
        });
        e.register(0x006b_39a0, |e, a| {
            copy_bytes(e, a[0], a[1], 0x20);
            Ret::default()
        });
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(0x006b_3d90, |e, _| ret(e.mem.alloc(0x68)));
        e.register(NODE_DATA, |_, a| ret(a[0] + 8));
        e.register(0x007d_6bb0, |_, a| ret(a[0] + 0x14));
        e.register(0x006a_1b60, |e, a| ret(e.mem.u32(a[0] + 0x54)));
        e.register(0x006a_7f50, |e, a| st0(f64::from(e.mem.f32(a[0]))));
        e.register(SELF, |_, a| ret(a[0]));
        e.register(SUM_OF_TWO, |_, a| ret(a[1].wrapping_add(a[2])));
        e.register(WORD_0, |e, a| ret(e.mem.u32(a[0])));
        e.register(0x005d_8a70, |_, a| ret(a[0] + 0x2c));
        e.register(0x0040_3e20, |_, a| ret(a[0] + 4));
        e.register(REQUEST_FIELD_34, |_, a| ret(a[0] + 0x34));
        e.register(REQUEST_RADIUS, |_, _| st0(40.0));
        e.register(WATER_HEIGHT, |_, _| st0(-1.0e9));
        e.register(0x00b0_0a00, |_, _| ret(MANAGER));
        e.register(NAVMESH_CLOSED_DOOR_INFO, |_, _| ret(DOOR_INFO));
        e.register(NAVMESH_VERTEX, |_, a| ret(VERTICES + 12 * a[1]));
        e.register(NAVMESH_EDGE_VERTEX_INDEXES, |e, a| {
            e.mem.set_u16(a[1], 0);
            e.mem.set_u16(a[1] + 2, 1);
            ret(a[1])
        });
        e.register(NAVMESH_EXIT_EDGE, |_, _| ret(0xffff_ffff));
        e.register(ARRAY_FIND, |_, _| ret(0xffff_ffff));
        e.register(VERT_HANDLE_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u16(a[0] + 4, a[2] as u16);
            ret(a[0])
        });
        e.register(VERT_HANDLE_COPY, |e, a| {
            copy_bytes(e, a[0], a[1], 6);
            ret(a[0])
        });
        e.register(EDGE_HANDLE_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u16(a[0] + 4, a[2] as u16);
            e.mem.set_u32(a[0] + 8, a[3]);
            ret(a[0])
        });
        e.register(EDGE_HANDLE_COPY, |e, a| {
            copy_bytes(e, a[0], a[1], 12);
            ret(a[0])
        });
        e.register(RECORD_FROM_HANDLE, |e, a| {
            copy_bytes(e, a[0], a[1], 6);
            ret(a[0])
        });
        e.register(HANDLES_EQUAL, |e, a| {
            let same =
                e.mem.u32(a[0]) == e.mem.u32(a[1]) && e.mem.u16(a[0] + 4) == e.mem.u16(a[1] + 4);
            ret(u32::from(same))
        });
        e.register(0x006c_f900, |e, a| {
            copy_bytes(e, a[0], a[1], 0x18);
            ret(a[0])
        });
        e.register(0x006a_0e50, |e, a| {
            copy_bytes(e, a[0], a[1], 0x18);
            ret(a[0])
        });
        e.register(0x006a_0c10, |e, a| {
            copy_bytes(e, a[0], a[1], 6);
            copy_bytes(e, a[0] + 0xc, a[2], 12);
            ret(a[0])
        });
        e.register(0x006a_0620, |e, a| {
            copy_bytes(e, a[0], a[1], 0x14);
            ret(a[0])
        });
        e.register(0x0069_85f0, |e, a| {
            copy_bytes(e, a[0], a[1], 8);
            ret(a[0])
        });
        e.register(0x006a_04b0, |e, a| {
            copy_bytes(e, a[0], a[1], 12);
            e.mem.set_u32(a[0] + 12, a[2]);
            ret(a[0])
        });
        e.register(0x006a_0e90, |_, a| ret(a[1]));
        e.register(TRIANGLE_ELEMENT_POINT, |_, a| ret(a[1]));
        e.register(SEGMENT_POINT, |_, a| ret(a[0]));
        e.register(NODE_POINT, |_, a| ret(a[1]));
        e.register(0x0069_8620, |_, a| ret(a[1]));
    }

    /// Every other callee: does nothing and returns 0.
    const NOOPS: &[u32] = &[
        DEALLOCATE,
        EDGE_HANDLE_CONSTRUCT,
        ARRAY_SET_RESERVED,
        EDGE_VERT_ARRAY_RESERVE,
        ARRAY_CLEAR,
        NAVMESH_CELL,
        NAVMESH_INFO,
        NAVMESH_GET_EDGE_VERTICES,
        NAVMESH_EDGE_ENDS,
        NAVMESH_GET_MATCHING_EDGE,
        NAVMESH_GET_MATCHING_EDGE_HANDLE,
        NAVMESH_COMPUTE_TRIANGLE_Z,
        NAVMESH_TRIANGLE_VECTOR,
        TRIANGLE_HAS_FLAG,
        NAVMESH_AVOID_NODE_COUNT,
        NAVMESH_AVOID_NODE,
        TRIANGLE_ELEMENT_TRIANGLE,
        REQUEST_OBSTACLES,
        REQUEST_BYTE_A2,
        BYTE_26_BIT_1,
        FLOAT_18,
        WORD_20,
        OBSTACLE_AT,
        LOCATION_RECORD_CONSTRUCT,
        PATHING_LOCATION_CONSTRUCT,
        PATHING_LOCATION_DESTRUCT,
        FIND_POINT_NEAR,
        SOLUTION_ADD,
        SEGMENT_INTERSECT,
        LAST_NODE_FIELD,
        PATHING_LOCATION_GET_CELL,
        ARRAY_LAST,
        FILL_ELEMENTS,
        0x006a_0840,
        0x006a_05e0,
        0x006a_0660,
        0x006d_cf20,
        0x0050_5c00,
        0x004b_13c0,
        0x0050_7e10,
        0x004e_44f0,
        0x0058_9850,
        0x006a_0870,
        0x006b_b570,
        0x006a_07a0,
        0x0055_82f0,
        0x0069_3b70,
        0x0068_f230,
        0x0069_3b90,
        0x006b_a240,
        0x006b_ab00,
        0x0069_8580,
        0x0069_85c0,
        0x006b_ac20,
        0x006a_9560,
        0x006b_9b10,
        0x0048_3710,
        0x0098_2610,
        0x006a_1f70,
        0x006b_3de0,
        0x006b_9470,
        0x006b_3e10,
        0x0040_fc90,
        0x006b_3a30,
        0x006b_3cb0,
        0x0099_6840,
        0x006a_1fb0,
        0x006b_3f40,
        0x006a_2130,
        0x006b_4190,
        0x006a_2630,
        0x006b_40f0,
        0x0040_f6e0,
        0x0040_1970,
        0x006b_3830,
        0x0069_f920,
        0x4400_0010,
    ];

    /// An engine with every callee doubled and a `PathSmootherPOVSearch`
    /// with buffers on its arrays and a request; returns the search.
    fn world() -> (Engine, u32) {
        let mut e = Engine::new();
        constants(&mut e);
        point_math(&mut e);
        containers(&mut e);
        for &addr in NOOPS {
            e.register(addr, noop);
        }
        let s = e.mem.alloc(0x2110);
        for at in [
            LEFT_POVS,
            RIGHT_POVS,
            AVOID_NODE_POVS,
            ALL_POVS,
            NODES,
            LINKS,
            LEFT_VERTS,
            RIGHT_VERTS,
        ] {
            buffer(&mut e, s + at, 0x800);
        }
        let request = e.mem.alloc(0x100);
        e.mem.set_u32(s + 0x2058, request);
        (e, s)
    }

    /// A POV of `kind` at `center` with `radius`.
    fn pov(e: &mut Engine, kind: u32, center: [f32; 3], radius: f32) -> u32 {
        let p = e.mem.alloc(0x4c);
        e.mem.set_u32(p, kind);
        set_point(e, p + 4, center);
        e.mem.set_f32(p + 0x10, radius);
        e.mem.set_f32(p + 0x38, 1.0);
        p
    }

    /// An object whose virtual slot +4 hands out a fixed buffer and records
    /// the requested size in the object's first dword after the vtable.
    fn array(e: &mut Engine) -> (Ptr<BSSimpleArray>, u32) {
        let buffer = e.mem.alloc(64);
        e.put_vtable(0x4300_0000, &[0, 0x4400_0000]);
        e.register_double(0x4400_0000, move |_, _| Ret {
            eax: buffer,
            ..Ret::default()
        });
        let this = e.new_object::<BSSimpleArray>();
        e.mem.set_u32(this.addr(), 0x4300_0000);
        (this, buffer)
    }

    #[test]
    fn size_larger_than_hint_reserves_the_size_and_fills() {
        let mut e = Engine::new();
        e.register(FILL_ELEMENTS, |_, _| Ret::default());
        let (this, buffer) = array(&mut e);
        e.set(this, BSSimpleArray::iSize, 99);
        e.call_log = Some(vec![]);
        e.call(0x006b3eb0, &args![this, 2u32, 5u32]);
        assert_eq!(e.get(this, BSSimpleArray::pBuffer), buffer);
        assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 5);
        assert_eq!(e.get(this, BSSimpleArray::iSize), 5);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (0x4400_0000, vec![this.addr(), 5]));
        assert_eq!(log[2], (FILL_ELEMENTS, vec![this.addr(), buffer, 5]));
    }

    #[test]
    fn a_larger_hint_is_the_reserved_size() {
        let mut e = Engine::new();
        e.register(FILL_ELEMENTS, |_, _| Ret::default());
        let (this, buffer) = array(&mut e);
        e.call(0x006b3eb0, &args![this, 8u32, 3u32]);
        assert_eq!(e.get(this, BSSimpleArray::pBuffer), buffer);
        assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 8);
        assert_eq!(e.get(this, BSSimpleArray::iSize), 3, "size 3 is filled");
    }

    #[test]
    fn zero_zero_allocates_nothing() {
        let mut e = Engine::new();
        e.register(FILL_ELEMENTS, |_, _| panic!("nothing to prepare"));
        let (this, _) = array(&mut e);
        e.mem.set_u32(this.addr() + 4, 0x1111);
        e.call(0x006b3eb0, &args![this, 0u32, 0u32]);
        assert_eq!(e.get(this, BSSimpleArray::pBuffer), 0);
        assert_eq!(e.get(this, BSSimpleArray::iSize), 0);
        assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 0);
    }

    #[test]
    fn hint_without_size_reserves_and_leaves_size_zero() {
        let mut e = Engine::new();
        e.register(FILL_ELEMENTS, |_, _| panic!("no elements to prepare"));
        let (this, buffer) = array(&mut e);
        e.call(0x006b3eb0, &args![this, 8u32, 0u32]);
        assert_eq!(e.get(this, BSSimpleArray::pBuffer), buffer);
        assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 8);
        assert_eq!(e.get(this, BSSimpleArray::iSize), 0);
    }

    /// A standalone `BSSimpleArray` with a buffer.
    fn array_object(e: &mut Engine) -> u32 {
        let a = e.mem.alloc(0x10);
        buffer(e, a, 0x800);
        a
    }

    /// A one-element triangle array (navmesh word, triangle `u16`).
    fn triangles(e: &mut Engine, navmesh: u32, triangle: u16) -> u32 {
        let a = array_object(e);
        let mut bytes = vec![0u8; 0x14];
        bytes[0..4].copy_from_slice(&navmesh.to_le_bytes());
        bytes[4..6].copy_from_slice(&triangle.to_le_bytes());
        push(e, a, &bytes);
        a
    }

    /// Points recorded by a double.
    type Seen = std::rc::Rc<std::cell::RefCell<Vec<[f32; 3]>>>;

    /// Records the point at argument `arg` of each call to `addr`.
    fn record_point(e: &mut Engine, addr: u32, arg: usize) -> Seen {
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let log = seen.clone();
        e.register_double(addr, move |e, a| {
            log.borrow_mut().push(point(e, a[arg]));
            Ret::default()
        });
        seen
    }

    #[test]
    fn scalar_deleting_destructor_frees_only_with_bit_0() {
        let (mut e, s) = world();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006a_d660, &args![s, 1u32]).u32(), s);
        assert_eq!(calls(&e, DEALLOCATE), vec![vec![s]]);
        assert_eq!(e.mem.u32(s), VTABLE_ASTAR_SEARCH);
        e.call_log = Some(vec![]);
        e.call(0x006a_d660, &args![s, 0u32]);
        assert!(calls(&e, DEALLOCATE).is_empty());
    }

    #[test]
    fn destructor_destroys_the_arrays_last_first_then_the_base() {
        let (mut e, s) = world();
        e.call_log = Some(vec![]);
        e.call(0x006a_d690, &args![s]);
        let cleared: Vec<u32> = calls(&e, ARRAY_CLEAR)
            .iter()
            .map(|a| {
                assert_eq!(a[1], 1);
                a[0] - s
            })
            .collect();
        assert_eq!(
            cleared,
            vec![
                RIGHT_VERTS,
                LEFT_VERTS,
                LINKS,
                NODES,
                ALL_POVS,
                AVOID_NODE_POVS,
                RIGHT_POVS,
                LEFT_POVS
            ]
        );
        assert_eq!(e.mem.u32(s + RIGHT_VERTS), VTABLE_EDGE_VERT_ARRAY);
        assert_eq!(e.mem.u32(s + LINKS), VTABLE_LINK_ARRAY);
        assert_eq!(e.mem.u32(s + NODES), VTABLE_NODE_ARRAY);
        assert_eq!(e.mem.u32(s + LEFT_POVS), VTABLE_POV_ARRAY);
        assert_eq!(
            calls(&e, 0x0048_3710),
            vec![vec![s + SEARCH_QUEUE], vec![s + NODE_MAP]]
        );
        assert_eq!(calls(&e, 0x0098_2610), vec![vec![s + NODE_HEAP]]);
    }

    #[test]
    fn entry_snaps_the_goal_runs_the_search_and_frees_the_povs() {
        let (mut e, s) = world();
        let request = e.mem.u32(s + 0x2058);
        let navmesh = e.mem.alloc(0x40);
        let tris = triangles(&mut e, navmesh, 7);
        let edges = array_object(&mut e);
        let (start, goal) = (e.mem.alloc(12), e.mem.alloc(12));
        set_point(&mut e, start, [1.0, 2.0, 3.0]);
        set_point(&mut e, goal, [31.0, 42.0, 5.0]);
        let solution = e.mem.alloc(0x60);
        buffer(&mut e, solution + 0x30, 0x40);
        e.register(NAVMESH_COMPUTE_TRIANGLE_Z, |e, a| {
            e.mem.set_f32(a[5], 12.5);
            Ret::default()
        });
        e.register(TRIANGLE_HAS_FLAG, |_, a| ret(u32::from(a[1] == 0x1000)));
        let goal_node = e.mem.alloc(0x68);
        e.register_double(0x006b_3a30, move |_, _| ret(goal_node));
        e.call_log = Some(vec![]);
        let found = e
            .call(
                0x006a_d770,
                &args![s, request, tris, edges, start, goal, solution, 0u32, 0x99u32],
            )
            .bool();
        assert!(found);
        assert_eq!(
            point(&e, s + START),
            [1.0, 2.0, 3.0],
            "start under water: kept"
        );
        assert_eq!(point(&e, s + GOAL), [31.0, 42.0, 12.5], "goal snapped");
        assert_eq!(e.mem.u8(s + 0x2078), 0);
        assert_eq!(e.mem.u32(s + 0x210c), 0x99);
        let snap = calls(&e, NAVMESH_COMPUTE_TRIANGLE_Z);
        assert_eq!(snap.len(), 1);
        assert_eq!(
            &snap[0][..5],
            &[navmesh, 7, s + GOAL, 0x4334_0000, 0x4280_0000]
        );
        let reserved = calls(&e, ARRAY_SET_RESERVED);
        assert!(reserved.contains(&vec![s + ALL_POVS, 2]));
        assert_eq!(word_at(&e, solution + 0x30, 0), 0xabc);
        assert_eq!(
            calls(&e, 0x006b_3cb0),
            vec![vec![s, goal_node, s + NODES, s + LINKS]]
        );
        let start_node = e.get(Ptr::<AStarSearch>::new(s), AStarSearch::pStartAStarNode);
        let distance = length([30.0, 40.0, 9.5]);
        assert_eq!(
            e.mem.f32(start_node + 4),
            (f64::from(distance) * 0.75) as f32
        );
        assert_eq!(e.mem.u32(start_node + 8), e.mem.u32(s + 0x20c4));
        let freed: Vec<u32> = calls(&e, DEALLOCATE).iter().map(|a| a[0]).collect();
        assert_eq!(freed, vec![e.mem.u32(s + 0x20c4), e.mem.u32(s + 0x20c8)]);
    }

    #[test]
    fn point_set_up_adds_the_middle_and_goal_side_povs_when_asked() {
        let (mut e, s) = world();
        let value = triangles(&mut e, 0, 0);
        e.mem.set_u32(s + 0x207c, value);
        let value = array_object(&mut e);
        e.mem.set_u32(s + 0x2080, value);
        set_point(&mut e, s + GOAL, [10.0, 0.0, 0.0]);
        e.register(REQUEST_BYTE_A2, |_, _| ret(1));
        e.call(0x006a_dcb0, &args![s]);
        assert_eq!(e.mem.u32(s + AVOID_NODE_POVS + 8), 3);
        assert_eq!(e.mem.u32(s + ALL_POVS + 8), 5);
        let expected = [
            ([5.0, 0.0, 0.0], 4.0),
            ([8.0, -2.0, 0.0], 2.5),
            ([8.0, 2.0, 0.0], 2.5),
        ];
        for (k, (center, radius)) in expected.iter().enumerate() {
            let p = word_at(&e, s + AVOID_NODE_POVS, k as u32);
            assert_eq!(e.mem.u32(p), 4);
            assert_eq!(point(&e, p + 4), *center, "POV {k}");
            assert_eq!(e.mem.f32(p + 0x10), *radius);
            assert_eq!(e.mem.f32(p + 0x38), 100.0);
            assert_eq!(e.mem.u16(p + 0x48), 0xffff);
        }
    }

    #[test]
    fn pov_constructor_sets_the_defaults() {
        let (mut e, _) = world();
        let p = e.mem.alloc(0x4c);
        e.mem.set_f32(p + 0x10, 9.0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006a_e120, &args![p]).u32(), p);
        assert_eq!(e.mem.u32(p), 0xffff_ffff);
        assert_eq!(e.mem.f32(p + 0x10), 0.0);
        assert_eq!(e.mem.f32(p + 0x38), 1.0);
        assert_eq!(calls(&e, EDGE_HANDLE_CONSTRUCT), vec![vec![p + 0x14]]);
        assert_eq!(calls(&e, SELF), vec![vec![p + 0x20], vec![p + 0x2c]]);
    }

    #[test]
    fn start_pov_is_type_2_at_the_start() {
        let (mut e, s) = world();
        set_point(&mut e, s + START, [1.0, 2.0, 3.0]);
        e.call(0x006a_e190, &args![s]);
        let p = e.mem.u32(s + 0x20c4);
        assert_eq!(e.mem.u32(p), 2);
        assert_eq!(point(&e, p + 4), [1.0, 2.0, 3.0]);
        assert_eq!(e.mem.u16(p + 0x48), 0xffff);
        assert_eq!(word_at(&e, s + ALL_POVS, 0), p);
    }

    #[test]
    fn goal_pov_is_type_3_at_the_goal() {
        let (mut e, s) = world();
        set_point(&mut e, s + GOAL, [4.0, 5.0, 6.0]);
        e.call(0x006a_e280, &args![s]);
        let p = e.mem.u32(s + 0x20c8);
        assert_eq!(e.mem.u32(p), 3);
        assert_eq!(point(&e, p + 4), [4.0, 5.0, 6.0]);
        assert_eq!(word_at(&e, s + ALL_POVS, 0), p);
    }

    /// An edge-array element (navmesh, triangle, edge).
    fn push_edge(e: &mut Engine, at: u32, navmesh: u32, triangle: u16, edge: u32) {
        let mut bytes = vec![0u8; 12];
        bytes[0..4].copy_from_slice(&navmesh.to_le_bytes());
        bytes[4..6].copy_from_slice(&triangle.to_le_bytes());
        bytes[8..12].copy_from_slice(&edge.to_le_bytes());
        push(e, at, &bytes);
    }

    #[test]
    fn first_edge_sorts_its_vertices_by_the_turn_and_makes_corner_povs() {
        let (mut e, s) = world();
        let edges = array_object(&mut e);
        push_edge(&mut e, edges, 0x4700_0000, 3, 1);
        e.mem.set_u32(s + 0x2080, edges);
        set_point(&mut e, VERTICES + 12, [0.0, 10.0, 0.0]);
        set_point(&mut e, s + START, [-5.0, 5.0, 0.0]);
        e.call(0x006a_e370, &args![s]);
        assert_eq!(e.mem.u32(s + LEFT_VERTS + 8), 1);
        assert_eq!(e.mem.u32(s + RIGHT_VERTS + 8), 1);
        let left = element(&e, s + LEFT_VERTS, 0, 0x20);
        let right = element(&e, s + RIGHT_VERTS, 0, 0x20);
        assert_eq!((e.mem.u32(left), e.mem.u16(left + 4)), (0x4700_0000, 1));
        assert_eq!((e.mem.u32(right), e.mem.u16(right + 4)), (0x4700_0000, 0));
        let left_pov = word_at(&e, s + LEFT_POVS, 0);
        let right_pov = word_at(&e, s + RIGHT_POVS, 0);
        assert_eq!(e.mem.u32(left_pov), 0, "left corner type");
        assert_eq!(e.mem.u32(right_pov), 1, "right corner type");
        assert_eq!(point(&e, left_pov + 4), [0.0, 10.0, 0.0]);
        assert_eq!(e.mem.f32(left_pov + 0x10), (40.0 * 1.2f64) as f32);
        assert_eq!(e.mem.u16(left_pov + 0x48), 1);
        assert_eq!(e.mem.u32(left_pov + 0x40), 0, "linked to the right POV");
        assert_eq!(e.mem.u32(right_pov + 0x40), 0xffff_ffff);
    }

    #[test]
    fn left_edge_vert_records_both_sides_sizes() {
        let (mut e, s) = world();
        e.mem.set_u32(s + RIGHT_VERTS + 8, 3);
        let (vert, edge) = (e.mem.alloc(8), e.mem.alloc(12));
        e.mem.set_u32(vert, 0x4700_0000);
        e.mem.set_u16(vert + 4, 9);
        e.mem.set_u32(edge + 8, 2);
        e.call(0x006a_ea90, &args![s, vert, edge]);
        let v = element(&e, s + LEFT_VERTS, 0, 0x20);
        assert_eq!((e.mem.u32(v), e.mem.u16(v + 4)), (0x4700_0000, 9));
        assert_eq!(e.mem.u32(v + 0x10), 2);
        assert_eq!(e.mem.u32(v + 0x14), 3);
        assert_eq!(e.mem.u32(v + 0x18), 0);
        assert_eq!(e.mem.u32(v + 0x1c), 0xffff_ffff);
    }

    #[test]
    fn right_edge_vert_records_both_sides_sizes() {
        let (mut e, s) = world();
        e.mem.set_u32(s + LEFT_VERTS + 8, 2);
        let (vert, edge) = (e.mem.alloc(8), e.mem.alloc(12));
        e.call(0x006a_eb00, &args![s, vert, edge]);
        let v = element(&e, s + RIGHT_VERTS, 0, 0x20);
        assert_eq!(e.mem.u32(v + 0x14), 2);
        assert_eq!(e.mem.u32(v + 0x18), 0);
        assert_eq!(e.mem.u32(s + RIGHT_VERTS + 8), 1);
        assert_eq!(e.mem.u32(s + LEFT_VERTS + 8), 2);
    }

    #[test]
    fn edge_vert_constructor_builds_both_handles() {
        let (mut e, _) = world();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006a_eb70, &args![0x1000u32]).u32(), 0x1000);
        assert_eq!(calls(&e, SELF), vec![vec![0x1000]]);
        assert_eq!(calls(&e, EDGE_HANDLE_CONSTRUCT), vec![vec![0x1008]]);
    }

    /// Three left vertices at `points` (vertex indexes 0, 1, 2).
    fn three_verts(e: &mut Engine, s: u32, points: [[f32; 3]; 3]) {
        for (k, p) in points.iter().enumerate() {
            set_point(e, VERTICES + 12 * k as u32, *p);
            let mut bytes = vec![0u8; 0x20];
            bytes[0..4].copy_from_slice(&0x4700_0000u32.to_le_bytes());
            bytes[4..6].copy_from_slice(&(k as u16).to_le_bytes());
            bytes[0x1c..0x20].copy_from_slice(&0xffff_ffffu32.to_le_bytes());
            push(e, s + LEFT_VERTS, &bytes);
        }
    }

    #[test]
    fn a_straight_middle_vertex_is_no_corner() {
        let (mut e, s) = world();
        three_verts(&mut e, s, [[0.0; 3], [10.0, 0.0, 0.0], [20.0, 0.0, 0.0]]);
        e.call(0x006a_eba0, &args![s, s + LEFT_VERTS, 1u32, 1u32]);
        assert_eq!(e.mem.u32(s + ALL_POVS + 8), 0);
    }

    #[test]
    fn a_turning_middle_vertex_gets_a_corner_pov() {
        let (mut e, s) = world();
        three_verts(&mut e, s, [[0.0; 3], [10.0, 0.0, 0.0], [20.0, 10.0, 0.0]]);
        e.call(0x006a_eba0, &args![s, s + LEFT_VERTS, 1u32, 1u32]);
        let p = word_at(&e, s + LEFT_POVS, 0);
        assert_eq!(e.mem.u32(p), 0);
        assert_eq!(point(&e, p + 4), [10.0, 0.0, 0.0]);
        assert_eq!(
            point(&e, p + 0x20),
            [-1.0, 0.0, 0.0],
            "Edge1: to the previous"
        );
        assert_eq!(e.mem.u32(p + 0x3c), 0);
        assert_eq!(e.mem.u16(p + 0x48), 1);
        let v = element(&e, s + LEFT_VERTS, 1, 0x20);
        assert_eq!(e.mem.u32(v + 0x1c), 0);
        assert_eq!(word_at(&e, s + ALL_POVS, 0), p);
    }

    #[test]
    fn obstacles_get_povs_and_type_2_ones_a_second() {
        let (mut e, s) = world();
        let list = e.mem.alloc(0x20);
        e.mem.set_u32(list + 8, 1);
        e.register_double(REQUEST_OBSTACLES, move |_, _| ret(list));
        let obstacle = e.mem.alloc(0x30);
        set_point(&mut e, obstacle, [7.0, 8.0, 9.0]);
        e.register_double(OBSTACLE_AT, move |_, _| ret(obstacle));
        e.register(FLOAT_18, |_, _| st0(10.0));
        e.register(WORD_20, |_, _| ret(2));
        e.call_log = Some(vec![]);
        e.call(0x006a_efc0, &args![s]);
        assert_eq!(calls(&e, 0x0040_f6e0), vec![vec![list + 0x10]]);
        assert_eq!(calls(&e, 0x0040_1970), vec![vec![list + 0x10]]);
        let first = word_at(&e, s + AVOID_NODE_POVS, 0);
        let second = word_at(&e, s + AVOID_NODE_POVS, 1);
        assert_eq!(e.mem.u32(first), 4);
        assert_eq!(point(&e, first + 4), [7.0, 8.0, 9.0]);
        assert_eq!(e.mem.f32(first + 0x10), (40.0 * 1.2f64 + 10.0) as f32);
        assert_eq!(e.mem.u32(second), 5);
        assert_eq!(e.mem.f32(second + 0x38), 10.0);
        assert_eq!(e.mem.f32(second + 0x10), (40.0 * 1.2f64 + 2.5) as f32);
        assert_eq!(e.mem.u32(second + 0x44), 0);
    }

    #[test]
    fn avoid_nodes_of_each_crossed_navmesh_get_povs() {
        let (mut e, s) = world();
        let tris = triangles(&mut e, 0x4700_0000, 1);
        push(&mut e, tris, &[0u8; 0x14]);
        e.mem.set_u32(s + 0x207c, tris);
        let mut seen = 0;
        e.register_double(ARRAY_FIND, move |_, _| {
            seen += 1;
            ret(if seen == 1 { 0xffff_ffff } else { 0 })
        });
        e.register(NAVMESH_AVOID_NODE_COUNT, |_, _| ret(2));
        e.map(0x4710_0000, 0x40);
        e.register(NAVMESH_AVOID_NODE, |_, a| ret(0x4710_0000 + 0x20 * a[1]));
        e.register(FLOAT_18, |e, a| st0(f64::from(e.mem.f32(a[0] + 0x18))));
        set_point(&mut e, 0x4710_0020, [1.0, 2.0, 3.0]);
        e.mem.set_f32(0x4710_0020 + 0x18, 6.0);
        e.call(0x006a_f2b0, &args![s]);
        assert_eq!(e.mem.u32(s + AVOID_NODE_POVS + 8), 2);
        let p = word_at(&e, s + AVOID_NODE_POVS, 1);
        assert_eq!(point(&e, p + 4), [1.0, 2.0, 3.0]);
        assert_eq!(e.mem.f32(p + 0x10), (40.0 * 1.2f64 + 6.0) as f32);
        assert_eq!(e.mem.u32(p + 0x44), 1);
    }

    #[test]
    fn a_point_found_near_the_start_puts_povs_beside_it() {
        let (mut e, s) = world();
        let value = triangles(&mut e, 0, 0);
        e.mem.set_u32(s + 0x207c, value);
        let mut seen = 0;
        e.register_double(FIND_POINT_NEAR, move |e, a| {
            seen += 1;
            if seen == 1 {
                set_point(e, a[2] + 0xc, [10.0, 0.0, 0.0]);
                ret(1)
            } else {
                ret(0)
            }
        });
        e.call(0x006a_f500, &args![s]);
        assert_eq!(e.mem.u32(s + AVOID_NODE_POVS + 8), 2);
        let first = word_at(&e, s + AVOID_NODE_POVS, 0);
        let second = word_at(&e, s + AVOID_NODE_POVS, 1);
        let y = point(&e, first + 4)[1];
        assert!((y + 40.4).abs() < 1e-3, "{y}");
        assert_eq!(point(&e, second + 4)[1], -y);
        assert_eq!(e.mem.f32(first + 0x10), 40.0);
        assert_eq!(e.mem.u32(first), 4);
    }

    /// A link (0x40 bytes): exit angle, entry angle, exit and entry points.
    fn push_link(
        e: &mut Engine,
        s: u32,
        exit: [f32; 2],
        entry: [f32; 2],
        from: [f32; 3],
        to: [f32; 3],
    ) {
        let mut bytes = vec![0u8; 0x40];
        let mut put = |off: usize, v: f32| bytes[off..off + 4].copy_from_slice(&v.to_le_bytes());
        put(0, exit[0]);
        put(4, exit[1]);
        put(8, entry[0]);
        put(0xc, entry[1]);
        for k in 0..3 {
            put(0x1c + 4 * k, from[k]);
            put(0x34 + 4 * k, to[k]);
        }
        push(e, s + LINKS, &bytes);
    }

    /// A node record (POV, direction, radius word) in `Nodes`.
    fn push_node(e: &mut Engine, s: u32, pov: u32, direction: u32) {
        let mut bytes = vec![0u8; 12];
        bytes[0..4].copy_from_slice(&pov.to_le_bytes());
        bytes[4..8].copy_from_slice(&direction.to_le_bytes());
        push(e, s + NODES, &bytes);
    }

    #[test]
    fn solution_gets_offset_exit_and_entry_points() {
        let (mut e, s) = world();
        let value = e.mem.alloc(0x60);
        e.mem.set_u32(s + 0x205c, value);
        push_link(
            &mut e,
            s,
            [1.0, 0.0],
            [1.0, 0.0],
            [0.0; 3],
            [10.0, 0.0, 0.0],
        );
        let (p0, p1) = (pov(&mut e, 2, [0.0; 3], 2.0), pov(&mut e, 3, [0.0; 3], 5.0));
        push_node(&mut e, s, p0, 0);
        push_node(&mut e, s, p1, 1);
        let added = record_point(&mut e, SOLUTION_ADD, 2);
        e.call(0x006a_fcc0, &args![s, 0u32]);
        assert_eq!(
            *added.borrow(),
            vec![[0.0, 6.0, 0.0], [0.0, -10.0, 0.0]],
            "radius x 3, the second cut to the segment's length"
        );
    }

    #[test]
    fn point_copy_goes_to_plus_2c() {
        let (mut e, _) = world();
        let (node, p) = (e.mem.alloc(0x40), e.mem.alloc(12));
        set_point(&mut e, p, [1.0, 2.0, 3.0]);
        e.call(0x006b_0570, &args![node, p]);
        assert_eq!(point(&e, node + 0x2c), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn last_node_is_null_for_an_empty_array() {
        let (mut e, _) = world();
        let solution = e.mem.alloc(0x40);
        assert_eq!(e.call(0x006b_05a0, &args![solution]).u32(), 0);
        e.mem.set_u32(solution + 0x28, 1);
        e.register(ARRAY_LAST, |_, _| ret(0x1234));
        assert_eq!(e.call(0x006b_05a0, &args![solution]).u32(), 0x1234);
    }

    #[test]
    fn byte_40_is_returned() {
        let (mut e, _) = world();
        let p = e.mem.alloc(0x44);
        e.mem.set_u8(p + 0x40, 7);
        assert_eq!(e.call(0x006b_05d0, &args![p]).u32() & 0xff, 7);
    }

    #[test]
    fn walk_adds_a_crossing_between_unlike_triangles() {
        let (mut e, s) = world();
        let solution = e.mem.alloc(0x40);
        e.mem.set_u32(solution + 0x28, 1);
        e.mem.set_u32(s + 0x205c, solution);
        e.register(ARRAY_LAST, |_, _| ret(0x1234));
        let mut exits = 0;
        e.register_double(NAVMESH_EXIT_EDGE, move |_, _| {
            exits += 1;
            ret(if exits == 1 { 2 } else { 0xffff_ffff })
        });
        e.register(NAVMESH_GET_MATCHING_EDGE_HANDLE, |e, a| {
            copy_bytes(e, a[2], a[1], 12);
            ret(1)
        });
        e.register(SEGMENT_INTERSECT, |e, a| {
            set_point(e, a[4], [50.0, 0.0, 0.0]);
            ret(1)
        });
        let added = record_point(&mut e, SOLUTION_ADD, 2);
        let (from, to) = (e.mem.alloc(12), e.mem.alloc(12));
        set_point(&mut e, to, [100.0, 0.0, 0.0]);
        e.call(0x006b_05f0, &args![s, 0x4700_0000u32, 3u32, from, to]);
        assert_eq!(*added.borrow(), vec![[40.0, 0.0, 0.0]]);
    }

    #[test]
    fn side_direction_turns_by_the_direction() {
        let (mut e, s) = world();
        let (out, angle) = (e.mem.alloc(12), e.mem.alloc(8));
        e.mem.set_f32(angle, 3.0);
        e.mem.set_f32(angle + 4, 4.0);
        assert_eq!(e.call(0x006b_0a60, &args![s, out, angle, 0u32]).u32(), out);
        assert_eq!(point(&e, out), [-4.0, 3.0, 0.0]);
        e.call(0x006b_0a60, &args![s, out, angle, 1u32]);
        assert_eq!(point(&e, out), [4.0, -3.0, 0.0]);
    }

    /// A node whose POV is `pov` (no parent, cost 0).
    fn node_of(e: &mut Engine, pov: u32, direction: u32) -> u32 {
        let n = e.mem.alloc(0x68);
        e.mem.set_u32(n + 8, pov);
        e.mem.set_u32(n + 0xc, direction);
        n
    }

    /// The indexes `00877a30` was asked for on the array at `at`.
    fn asked(e: &Engine, at: u32) -> Vec<u32> {
        calls(e, ARRAY_WORD_AT)
            .iter()
            .filter(|a| a[0] == at)
            .map(|a| a[1])
            .collect()
    }

    #[test]
    fn connections_from_a_left_corner_try_later_left_then_right_from_the_other_index() {
        let (mut e, s) = world();
        let p = pov(&mut e, 0, [0.0; 3], 1.0);
        e.mem.set_u32(p + 0x3c, 0);
        e.mem.set_u32(p + 0x40, 1);
        e.mem.set_u32(s + 0x20c8, p);
        for _ in 0..3 {
            push_word(&mut e, s + LEFT_POVS, p);
        }
        for _ in 0..2 {
            push_word(&mut e, s + RIGHT_POVS, p);
        }
        push_word(&mut e, s + AVOID_NODE_POVS, p);
        let node = node_of(&mut e, p, 0);
        let connections = array_object(&mut e);
        e.call_log = Some(vec![]);
        assert!(e.call(0x006b_0ad0, &args![s, node, connections]).bool());
        assert_eq!(asked(&e, s + LEFT_POVS), vec![1, 2]);
        assert_eq!(asked(&e, s + RIGHT_POVS), vec![1]);
        assert_eq!(asked(&e, s + AVOID_NODE_POVS), vec![0]);
    }

    #[test]
    fn connections_from_the_start_try_every_corner() {
        let (mut e, s) = world();
        let p = pov(&mut e, 2, [0.0; 3], 1.0);
        e.mem.set_u32(s + 0x20c8, p);
        for _ in 0..3 {
            push_word(&mut e, s + LEFT_POVS, p);
        }
        for _ in 0..2 {
            push_word(&mut e, s + RIGHT_POVS, p);
        }
        let node = node_of(&mut e, p, 0);
        e.call_log = Some(vec![]);
        let connections = array_object(&mut e);
        e.call(0x006b_0ad0, &args![s, node, connections]);
        assert_eq!(asked(&e, s + RIGHT_POVS), vec![0, 1]);
        assert_eq!(asked(&e, s + LEFT_POVS), vec![0, 1, 2]);
    }

    #[test]
    fn connection_is_not_tried_to_its_own_pov_or_up_a_steep_step() {
        let (mut e, s) = world();
        let a = pov(&mut e, 2, [0.0; 3], 1.0);
        let node = node_of(&mut e, a, 0);
        let connections = array_object(&mut e);
        e.call_log = Some(vec![]);
        assert!(e.call(0x006b_0f40, &args![s, node, a, connections]).bool());
        let steep = pov(&mut e, 4, [1.0, 0.0, 10.0], 1.0);
        assert!(e
            .call(0x006b_0f40, &args![s, node, steep, connections])
            .bool());
        assert!(calls(&e, 0x006b_b570).is_empty());
        e.mem.set_u8(s + 0x2078, 1);
        assert!(e
            .call(0x006b_0f40, &args![s, node, steep, connections])
            .bool());
        assert_eq!(
            calls(&e, 0x006b_b570).len(),
            1,
            "under water: tangents asked"
        );
    }

    #[test]
    fn a_clear_way_opens_the_target_node() {
        let (mut e, s) = world();
        e.mem.set_u8(s + 0x2078, 1);
        let value = triangles(&mut e, 0x4700_0000, 2);
        e.mem.set_u32(s + 0x207c, value);
        let goal = pov(&mut e, 3, [200.0, 0.0, 0.0], 0.0);
        e.mem.set_u32(s + 0x20c8, goal);
        let a = pov(&mut e, 2, [0.0; 3], 0.0);
        let node = node_of(&mut e, a, 0);
        e.mem.set_f32(node, 5.0);
        let target = pov(&mut e, 4, [100.0, 0.0, 0.0], 10.0);
        e.register(0x006b_b570, |e, a| {
            let table = a[3];
            e.mem.set_u32(table + 4, 0);
            e.mem.set_f32(table + 8, 1.0);
            e.mem.set_f32(table + 0xc, 0.0);
            e.mem.set_f32(table + 0x10, 0.0);
            e.mem.set_f32(table + 0x14, 1.0);
            ret(1)
        });
        let connections = array_object(&mut e);
        assert!(e
            .call(0x006b_0f40, &args![s, node, target, connections])
            .bool());
        assert_eq!(e.mem.u32(connections + 8), 1);
        let opened = word_at(&e, connections, 0);
        assert_eq!(e.mem.f32(opened), 5.0, "cost: the node's");
        assert_eq!(e.mem.u32(opened + 0x60), target);
        assert_eq!(e.mem.u32(opened + 8), target);
        assert_eq!(e.mem.u32(opened + 0xc), 0);
        assert_eq!(
            e.mem.f32(opened + 4),
            10.0,
            "0.1 x the distance to the goal"
        );
        assert_eq!(
            (e.mem.f32(opened + 0x14), e.mem.f32(opened + 0x18)),
            (1.0, 0.0)
        );
        assert_eq!(
            (e.mem.f32(opened + 0x1c), e.mem.f32(opened + 0x20)),
            (0.0, 1.0)
        );
        assert_eq!(e.mem.u32(opened + 0x24), 0x4700_0000, "exit location");
    }

    #[test]
    fn goal_link_needs_links_and_a_tangent() {
        let (mut e, s) = world();
        assert!(!e.call(0x006b_1970, &args![s]).bool());
        push_link(&mut e, s, [1.0, 0.0], [1.0, 0.0], [0.0; 3], [1.0, 0.0, 0.0]);
        let p = pov(&mut e, 0, [0.0; 3], 1.0);
        push_node(&mut e, s, p, 0);
        let goal = pov(&mut e, 3, [9.0, 0.0, 0.0], 0.0);
        e.mem.set_u32(s + 0x20c8, goal);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x006b_1970, &args![s]).bool());
        assert_eq!(calls(&e, 0x006b_b570).len(), 1);
        assert_eq!(e.mem.u32(s + NODES + 8), 1, "no goal node added");
    }

    #[test]
    fn link_constructor_builds_angles_and_locations() {
        let (mut e, _) = world();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006b_1d80, &args![0x2000u32]).u32(), 0x2000);
        assert_eq!(calls(&e, SELF), vec![vec![0x2000], vec![0x2008]]);
        assert_eq!(
            calls(&e, LOCATION_RECORD_CONSTRUCT),
            vec![vec![0x2010], vec![0x2028]]
        );
    }

    #[test]
    fn goal_node_is_the_type_3_pov() {
        let (mut e, s) = world();
        let p = pov(&mut e, 3, [0.0; 3], 0.0);
        let goal = node_of(&mut e, p, 0);
        let p = pov(&mut e, 4, [0.0; 3], 0.0);
        let other = node_of(&mut e, p, 0);
        assert!(e.call(0x006b_1dc0, &args![s, goal]).bool());
        assert!(!e.call(0x006b_1dc0, &args![s, other]).bool());
    }

    #[test]
    fn node_data_and_estimate_are_set() {
        let (mut e, s) = world();
        let goal = pov(&mut e, 3, [0.0; 3], 0.0);
        e.mem.set_u32(s + 0x20c8, goal);
        let p = pov(&mut e, 4, [3.0, 4.0, 0.0], 0.0);
        let node = e.mem.alloc(0x68);
        e.call(0x006b_1df0, &args![s, node, p, 1u32]);
        assert_eq!(e.mem.u32(node + 8), p);
        assert_eq!(e.mem.u32(node + 0xc), 1);
        assert_eq!(
            e.mem.f32(node + 4),
            (5.0 * f64::from_bits(0x3fb9_9999_a000_0000)) as f32
        );
    }

    /// An arc (exit (1, 0), entry (0, 1), `direction`), a circle of radius
    /// `r` at the origin and a location on navmesh `0x47000000`.
    fn arc_world(e: &mut Engine, r: f32, direction: u32) -> (u32, u32, u32, u32) {
        let arc = e.mem.alloc(0x14);
        e.mem.set_f32(arc, 1.0);
        e.mem.set_f32(arc + 0xc, 1.0);
        e.mem.set_u32(arc + 0x10, direction);
        let circle = e.mem.alloc(0x10);
        e.mem.set_f32(circle + 0xc, r);
        let location = e.mem.alloc(0x18);
        e.mem.set_u32(location, 0x4700_0000);
        (arc, circle, location, e.mem.alloc(0x18))
    }

    fn arc_words(e: &Engine, arc: u32) -> Vec<u32> {
        (0..5).map(|k| e.mem.u32(arc + 4 * k)).collect()
    }

    #[test]
    fn arc_walk_ends_with_the_chord_to_the_entry_point() {
        let (mut e, s) = world();
        let (arc, circle, location, out) = arc_world(&mut e, 5.0, 0);
        let mut words = vec![s, location, circle];
        words.extend(arc_words(&e, arc));
        words.push(out);
        assert!(e.call(0x006b_1e50, &words).bool());
        assert_eq!(e.mem.u32(out), 0x4700_0000);
        assert_eq!(point(&e, out + 0xc), [0.0, 5.0, 0.0]);
    }

    #[test]
    fn arc_walk_gives_up_after_24_steps() {
        let (mut e, s) = world();
        let (arc, circle, location, out) = arc_world(&mut e, 5.0, 1);
        e.register(0x006a_0660, |_, _| ret(1));
        e.call_log = Some(vec![]);
        let mut words = vec![s, location, circle];
        words.extend(arc_words(&e, arc));
        words.push(out);
        assert!(!e.call(0x006b_1e50, &words).bool());
        assert_eq!(calls(&e, NAVMESH_EXIT_EDGE).len(), 24);
        let steps: Vec<f32> = calls(&e, 0x006a_04e0).iter().map(|a| arg_f(a[1])).collect();
        assert_eq!(steps, vec![f32::from_bits(0xbe86_0a92)]);
    }

    #[test]
    fn segment_reaching_its_triangle_takes_its_height() {
        let (mut e, s) = world();
        let (location, to, out, share) = (
            e.mem.alloc(0x18),
            e.mem.alloc(12),
            e.mem.alloc(0x18),
            e.mem.alloc(4),
        );
        e.mem.set_u32(location, 0x4700_0000);
        set_point(&mut e, to, [3.0, 4.0, 0.0]);
        e.register(NAVMESH_COMPUTE_TRIANGLE_Z, |e, a| {
            e.mem.set_f32(a[5], 7.0);
            Ret::default()
        });
        e.mem.set_f32(share, 9.0);
        assert!(e
            .call(0x006b_2060, &args![s, location, to, out, share])
            .bool());
        assert_eq!(point(&e, out + 0xc), [3.0, 4.0, 7.0]);
        assert_eq!(e.mem.f32(share), 0.0);
    }

    #[test]
    fn segment_stops_at_an_edge_without_neighbour() {
        let (mut e, s) = world();
        let (location, to, out, share) = (
            e.mem.alloc(0x18),
            e.mem.alloc(12),
            e.mem.alloc(0x18),
            e.mem.alloc(4),
        );
        e.mem.set_u32(location, 0x4700_0000);
        e.register(NAVMESH_EXIT_EDGE, |_, _| ret(2));
        e.register(0x0069_3b70, |_, _| ret(1));
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x006b_2060, &args![s, location, to, out, share])
            .bool());
        assert_eq!(e.mem.u32(out), 0x4700_0000);
        assert_eq!(
            e.mem.f32(share),
            1.0,
            "the one triangle walked had flag 0x40"
        );
        assert_eq!(calls(&e, 0x006b_a240).len(), 1);
    }

    #[test]
    fn walker_fit_checks_the_inner_then_the_outer_circle() {
        let (mut e, s) = world();
        let (arc, circle, location, _) = arc_world(&mut e, 50.0, 0);
        e.call_log = Some(vec![]);
        let mut words = vec![s, location, circle];
        words.extend(arc_words(&e, arc));
        assert!(e.call(0x006b_23d0, &words).bool());
        let margin = 40.0 * f64::from_bits(0x3fec_cccc_c000_0000);
        let radii: Vec<f32> = calls(&e, 0x006a_04b0)
            .iter()
            .filter(|a| a[1] == circle)
            .map(|a| arg_f(a[2]))
            .collect();
        assert_eq!(radii, vec![(50.0 - margin) as f32, (margin + 50.0) as f32]);
    }

    #[test]
    fn triple_check_under_water_takes_the_end_height() {
        let (mut e, s) = world();
        e.mem.set_u8(s + 0x2078, 1);
        let (location, to, out, share) = (
            e.mem.alloc(0x18),
            e.mem.alloc(12),
            e.mem.alloc(0x18),
            e.mem.alloc(4),
        );
        set_point(&mut e, to, [5.0, 0.0, 3.0]);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x006b_25f0, &args![s, location, to, out, 0u32, 0u32, share])
            .bool());
        assert_eq!(e.mem.f32(out + 0x14), 3.0);
        assert_eq!(calls(&e, NAVMESH_EXIT_EDGE).len(), 1);
    }

    #[test]
    fn triple_check_walks_both_side_lines() {
        let (mut e, s) = world();
        let (location, to, out, share) = (
            e.mem.alloc(0x18),
            e.mem.alloc(12),
            e.mem.alloc(0x18),
            e.mem.alloc(4),
        );
        set_point(&mut e, to, [100.0, 0.0, 0.0]);
        let targets = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let log = targets.clone();
        e.register_double(NAVMESH_EXIT_EDGE, move |e, a| {
            log.borrow_mut().push(point(e, a[4]));
            ret(0xffff_ffff)
        });
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x006b_25f0, &args![s, location, to, out, 0u32, 0u32, share])
            .bool());
        assert_eq!(calls(&e, NAVMESH_EXIT_EDGE).len(), 5);
        assert_eq!(calls(&e, NAVMESH_COMPUTE_TRIANGLE_Z).len(), 6);
        let side = (40.0 * f64::from(f32::from_bits(0x3f66_6666))) as f32;
        let targets = targets.borrow().clone();
        assert_eq!(targets[1], [0.0, side, 0.0]);
        assert_eq!(targets[2], [100.0, side, 0.0]);
        assert_eq!(targets[3], [0.0, -side, 0.0]);
        assert_eq!(targets[4], [100.0, -side, 0.0]);
        e.call_log = Some(vec![]);
        e.call(0x006b_25f0, &args![s, location, to, out, 1u32, 1u32, share]);
        assert_eq!(
            calls(&e, NAVMESH_COMPUTE_TRIANGLE_Z).len(),
            5,
            "no end height at the goal"
        );
    }

    #[test]
    fn avoid_cost_is_the_length_for_a_short_xy_segment() {
        let (mut e, s) = world();
        let (from, to, cost) = (e.mem.alloc(12), e.mem.alloc(12), e.mem.alloc(4));
        set_point(&mut e, to, [0.0, 0.0, 3.0]);
        push_word(&mut e, s + AVOID_NODE_POVS, 0);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x006b_2c00, &args![s, 0u32, 0u32, from, to, cost])
            .bool());
        assert_eq!(e.mem.f32(cost), 3.0);
        assert!(calls(&e, ARRAY_WORD_AT).is_empty());
    }

    /// One avoid POV of `kind` at (5, 0, 0), radius 4, that the segment
    /// (0, 0, 0)-(10, 0, 0) crosses whole.
    fn crossed_avoid(e: &mut Engine, s: u32, kind: u32) -> (u32, u32, u32) {
        let a = pov(e, kind, [5.0, 0.0, 0.0], 4.0);
        push_word(e, s + AVOID_NODE_POVS, a);
        e.register(0x006b_ab00, |_, _| ret(1));
        e.register(0x006b_ac20, |e, a| {
            e.mem.set_u32(a[2], 5);
            Ret::default()
        });
        e.register(0x006b_9b10, |_, _| st0(1.0));
        e.map(0x011d_7084, 8);
        e.mem.set_f32(0x011d_7088, 2.0);
        let (from, to, cost) = (e.mem.alloc(12), e.mem.alloc(12), e.mem.alloc(4));
        set_point(e, to, [10.0, 0.0, 0.0]);
        (from, to, cost)
    }

    #[test]
    fn avoid_cost_adds_the_weighted_section_inside() {
        let (mut e, s) = world();
        let (from, to, cost) = crossed_avoid(&mut e, s, 4);
        let p = pov(&mut e, 2, [0.0; 3], 0.0);
        assert!(e
            .call(0x006b_2c00, &args![s, p, 0u32, from, to, cost])
            .bool());
        assert_eq!(e.mem.f32(cost), 10.0 + (0.75 * 2.0 + 1.0) * 10.0);
    }

    #[test]
    fn a_type_5_avoid_pov_blocks_when_the_request_says_so() {
        let (mut e, s) = world();
        let (from, to, cost) = crossed_avoid(&mut e, s, 5);
        e.register(0x006a_9560, |_, _| ret(1));
        let p = pov(&mut e, 4, [0.0; 3], 0.0);
        assert!(!e
            .call(0x006b_2c00, &args![s, p, 0u32, from, to, cost])
            .bool());
        let start = pov(&mut e, 2, [0.0; 3], 0.0);
        assert!(e
            .call(0x006b_2c00, &args![s, start, 0u32, from, to, cost])
            .bool());
        assert_eq!(
            e.mem.f32(cost),
            10.0 * 10.0 + 25.0,
            "x 10, then the section"
        );
    }

    #[test]
    fn narrow_corridor_edges_are_kept() {
        let (mut e, s) = world();
        assert!(!e.call(0x006b_2fc0, &args![s, 0u32]).bool());
        let edges = array_object(&mut e);
        push_edge(&mut e, edges, 0x4700_0000, 1, 0);
        push_edge(&mut e, edges, 0x4700_0000, 2, 1);
        e.mem.set_u32(s + 0x2080, edges);
        e.register(NAVMESH_EDGE_ENDS, |e, a| {
            let width = if a[1] == 1 { 50.0 } else { 100.0 };
            set_point(e, a[4], [width, 0.0, 0.0]);
            Ret::default()
        });
        let target = array_object(&mut e);
        assert!(e.call(0x006b_2fc0, &args![s, target]).bool());
        assert_eq!(e.mem.u32(target + 8), 1);
        assert_eq!(e.mem.u16(element(&e, target, 0, 12) + 4), 1);
    }

    #[test]
    fn link_element_address() {
        let (mut e, _) = world();
        let a = array_object(&mut e);
        let b = e.mem.u32(a + 4);
        assert_eq!(e.call(0x006b_30e0, &args![a, 3u32]).u32(), b + 0xc0);
    }

    #[test]
    fn last_link_element_address() {
        let (mut e, _) = world();
        let a = array_object(&mut e);
        e.mem.set_u32(a + 8, 2);
        assert_eq!(
            e.call(0x006b_3100, &args![a]).u32(),
            e.mem.u32(a + 4) + 0x40
        );
    }

    #[test]
    fn edge_vert_element_address() {
        let (mut e, _) = world();
        let a = array_object(&mut e);
        assert_eq!(
            e.call(0x006b_3120, &args![a, 3u32]).u32(),
            e.mem.u32(a + 4) + 0x60
        );
    }

    #[test]
    fn last_edge_vert_element_address() {
        let (mut e, _) = world();
        let a = array_object(&mut e);
        e.mem.set_u32(a + 8, 2);
        assert_eq!(
            e.call(0x006b_3140, &args![a]).u32(),
            e.mem.u32(a + 4) + 0x20
        );
    }

    #[test]
    fn triangle_element_address() {
        let (mut e, _) = world();
        let a = array_object(&mut e);
        assert_eq!(
            e.call(0x006b_3160, &args![a, 3u32]).u32(),
            e.mem.u32(a + 4) + 0x3c
        );
    }

    #[test]
    fn first_triangle_element_address() {
        let (mut e, _) = world();
        let a = array_object(&mut e);
        assert_eq!(e.call(0x006b_3180, &args![a]).u32(), e.mem.u32(a + 4));
    }

    #[test]
    fn last_triangle_element_address() {
        let (mut e, _) = world();
        let a = array_object(&mut e);
        e.mem.set_u32(a + 8, 3);
        assert_eq!(
            e.call(0x006b_31a0, &args![a]).u32(),
            e.mem.u32(a + 4) + 0x28
        );
    }

    #[test]
    fn astar_constructor_builds_the_parts_and_clears_the_state() {
        let (mut e, _) = world();
        let p = e.mem.alloc(0x2058);
        e.mem.set_u32(p + 0x2048, 1);
        e.mem.set_u32(p + 0x2050, 1);
        e.mem.set_u8(p + 0x2054, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006b_31c0, &args![p]).u32(), p);
        assert_eq!(e.mem.u32(p), VTABLE_ASTAR_SEARCH);
        assert_eq!(calls(&e, 0x006a_1f70), vec![vec![p + 4]]);
        assert_eq!(calls(&e, 0x006b_3de0), vec![vec![p + 0x14]]);
        assert_eq!(calls(&e, 0x006b_9470), vec![vec![p + 0x1ff4]]);
        assert_eq!(e.mem.u32(p + 0x2048), 0);
        assert_eq!(e.mem.u32(p + 0x2050), 0);
        assert_eq!(e.mem.u8(p + 0x2054), 0);
    }

    #[test]
    fn astar_destructor_destroys_queue_map_heap() {
        let (mut e, _) = world();
        let p = e.mem.alloc(0x2058);
        e.call_log = Some(vec![]);
        e.call(0x006b_3270, &args![p]);
        assert_eq!(e.mem.u32(p), VTABLE_ASTAR_SEARCH);
        assert_eq!(
            calls(&e, 0x0048_3710),
            vec![vec![p + 0x1ff4], vec![p + 0x14]]
        );
        assert_eq!(calls(&e, 0x0098_2610), vec![vec![p + 4]]);
    }

    #[test]
    fn start_node_is_created_once() {
        let (mut e, s) = world();
        let data = e.mem.alloc(12);
        e.mem.set_u32(data, 0x77);
        e.call_log = Some(vec![]);
        let node = e.call(0x006b_32f0, &args![s, 9u32, data, 2.5f32]).u32();
        assert_ne!(node, 0);
        assert_eq!(e.mem.f32(node + 4), 2.5);
        assert_eq!(e.mem.u32(node + 8), 0x77);
        assert_eq!(e.mem.u32(node + 0x60), 9);
        assert_eq!(calls(&e, 0x006b_3e10), vec![vec![s + 0x1ff4, node]]);
        assert_eq!(e.mem.u32(s + 0x2050), node);
        assert_eq!(e.call(0x006b_32f0, &args![s, 9u32, data, 2.5f32]).u32(), 0);
    }

    #[test]
    fn search_builds_from_the_goal_or_the_best_node() {
        let (mut e, s) = world();
        e.register(0x4400_0010, |_, _| ret(1));
        e.mem.set_u32(s + 0x2048, 0x55);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x006b_3380, &args![s, 1u32, 2u32, 0u32]).bool());
        assert!(calls(&e, 0x006b_3cb0).is_empty());
        assert_eq!(e.mem.u8(s + 0x2054), 1);
        assert!(!e.call(0x006b_3380, &args![s, 1u32, 2u32, 1u32]).bool());
        assert_eq!(calls(&e, 0x006b_3cb0), vec![vec![s, 0x55, 1, 2]]);
        e.register(0x006b_3a30, |_, _| ret(0x66));
        assert!(e.call(0x006b_3380, &args![s, 1u32, 2u32, 0u32]).bool());
        assert_eq!(calls(&e, 0x006b_3cb0)[1], vec![s, 0x66, 1, 2]);
    }

    #[test]
    fn node_lookup_opens_improves_or_keeps() {
        let (mut e, s) = world();
        let out = e.mem.alloc(4);
        assert_eq!(e.call(0x006b_3420, &args![s, 1u32, 5.0f32, out]).u32(), 0);
        let first = e.mem.u32(out);
        assert_eq!(e.call(0x006b_3420, &args![s, 2u32, 3.0f32, out]).u32(), 0);
        let second = e.mem.u32(out);
        assert_eq!(e.mem.u32(s + 0x14), second, "new nodes go to the head");
        assert_eq!(e.mem.u32(second + 0x64), first);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006b_3420, &args![s, 1u32, 4.0f32, out]).u32(), 1);
        assert_eq!(e.mem.u32(out), first);
        assert_eq!(e.mem.f32(first), 4.0);
        assert_eq!(calls(&e, 0x006a_1fb0), vec![vec![s + 0x1ff4, first]]);
        assert_eq!(e.call(0x006b_3420, &args![s, 2u32, 3.0f32, out]).u32(), 2);
        assert_eq!(e.call(0x006b_3420, &args![s, 1u32, f32::NAN, out]).u32(), 2);
    }

    fn check_constructed(e: &mut Engine, addr: u32, vtable: u32) -> u32 {
        let a = e.mem.alloc(0x10);
        e.mem.set_u32(a + 8, 5);
        assert_eq!(e.call(addr, &args![a]).u32(), a);
        assert_eq!(e.mem.u32(a), vtable);
        a
    }

    #[test]
    fn pov_array_constructor() {
        let (mut e, _) = world();
        let a = check_constructed(&mut e, 0x006b_3570, VTABLE_POV_ARRAY);
        assert_eq!(e.mem.u32(a + 8), 0, "006b3eb0(0, 0)");
    }

    #[test]
    fn navmesh_array_constructor() {
        let (mut e, _) = world();
        let a = check_constructed(&mut e, 0x006b_3800, VTABLE_NAVMESH_ARRAY);
        assert_eq!(e.mem.u32(a + 8), 0);
    }

    #[test]
    fn node_array_constructor() {
        let (mut e, _) = world();
        e.call_log = Some(vec![]);
        let a = check_constructed(&mut e, 0x006b_35c0, VTABLE_NODE_ARRAY);
        assert_eq!(calls(&e, 0x006b_3f40), vec![vec![a, 0, 0]]);
    }

    #[test]
    fn link_array_constructor() {
        let (mut e, _) = world();
        e.call_log = Some(vec![]);
        let a = check_constructed(&mut e, 0x006b_3610, VTABLE_LINK_ARRAY);
        assert_eq!(calls(&e, 0x006a_2130), vec![vec![a, 0, 0]]);
    }

    #[test]
    fn edge_vert_array_constructor() {
        let (mut e, _) = world();
        e.call_log = Some(vec![]);
        let a = check_constructed(&mut e, 0x006b_36e0, VTABLE_EDGE_VERT_ARRAY);
        assert_eq!(calls(&e, 0x006b_4190), vec![vec![a, 0, 0]]);
    }

    fn check_destroyed(e: &mut Engine, addr: u32, vtable: u32) {
        let a = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(addr, &args![a]);
        assert_eq!(e.mem.u32(a), vtable);
        assert_eq!(calls(e, ARRAY_CLEAR), vec![vec![a, 1]]);
    }

    #[test]
    fn pov_array_destructor() {
        let (mut e, _) = world();
        check_destroyed(&mut e, 0x006b_35a0, VTABLE_POV_ARRAY);
    }

    #[test]
    fn node_array_destructor() {
        let (mut e, _) = world();
        check_destroyed(&mut e, 0x006b_35f0, VTABLE_NODE_ARRAY);
    }

    #[test]
    fn link_array_destructor() {
        let (mut e, _) = world();
        check_destroyed(&mut e, 0x006b_3640, VTABLE_LINK_ARRAY);
    }

    #[test]
    fn edge_vert_array_destructor() {
        let (mut e, _) = world();
        check_destroyed(&mut e, 0x006b_3710, VTABLE_EDGE_VERT_ARRAY);
    }

    #[test]
    fn link_is_appended_as_a_copy() {
        let (mut e, _) = world();
        let a = array_object(&mut e);
        e.mem.set_u32(a + 8, 1);
        let link = e.mem.alloc(0x40);
        e.mem.set_u32(link + 0x3c, 0x99);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006b_3660, &args![a, link]).u32(), 1);
        let slot = e.mem.u32(a + 4) + 0x40;
        assert_eq!(calls(&e, 0x006a_2630), vec![vec![a, slot, 1]]);
        assert_eq!(e.mem.u32(slot + 0x3c), 0x99);
    }

    #[test]
    fn edge_vert_is_appended_as_a_copy() {
        let (mut e, _) = world();
        let a = array_object(&mut e);
        let vert = e.mem.alloc(0x20);
        e.mem.set_u32(vert + 0x1c, 0x98);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006b_3730, &args![a, vert]).u32(), 0);
        let slot = e.mem.u32(a + 4);
        assert_eq!(calls(&e, 0x006b_40f0), vec![vec![a, slot, 1]]);
        assert_eq!(e.mem.u32(slot + 0x1c), 0x98);
    }

    #[test]
    fn link_buffer_is_64_bytes_per_element() {
        let (mut e, _) = world();
        e.call_log = Some(vec![]);
        assert_ne!(e.call(0x006b_36b0, &args![0u32, 3u32]).u32(), 0);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![0xc0]]);
    }

    #[test]
    fn edge_vert_buffer_is_32_bytes_per_element() {
        let (mut e, _) = world();
        e.call_log = Some(vec![]);
        e.call(0x006b_3780, &args![0u32, 3u32]);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![0x60]]);
    }

    #[test]
    fn holder_counts_a_held_object() {
        let (mut e, _) = world();
        let h = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006b_37b0, &args![h, 0x5000u32]).u32(), h);
        assert_eq!(e.mem.u32(h), 0x5000);
        assert_eq!(calls(&e, 0x0040_f6e0), vec![vec![0x5010]]);
        e.call(0x006b_37b0, &args![h, 0u32]);
        assert_eq!(calls(&e, 0x0040_f6e0).len(), 1);
    }

    #[test]
    fn holder_release_only_for_a_held_object() {
        let (mut e, _) = world();
        let h = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        e.call(0x006b_37e0, &args![h]);
        assert!(calls(&e, 0x0040_1970).is_empty());
        e.mem.set_u32(h, 0x5000);
        e.call(0x006b_37e0, &args![h]);
        assert_eq!(calls(&e, 0x0040_1970), vec![vec![0x5010]]);
    }

    // --- 006b3830 .. 006b4670 ---------------------------------------------

    /// An engine with only the translations and the doubles a test adds.
    fn bare(doubled: &[u32]) -> Engine {
        let mut e = Engine::new();
        for &addr in doubled {
            e.register(addr, noop);
        }
        e.call_log = Some(vec![]);
        e
    }

    /// The vtable of a test array: slot +4 allocates `count x 0x40` bytes,
    /// slot +0xc answers with `REALLOCATED`.
    const ARRAY_VTABLE: u32 = 0x4500_0000;
    const ARRAY_ALLOCATE: u32 = 0x4500_1004;
    const ARRAY_REALLOCATE: u32 = 0x4500_100c;
    const REALLOCATED: u32 = 0x4500_2000;

    fn test_array(e: &mut Engine, buffer: u32, size: u32, reserved: u32) -> Ptr {
        e.put_vtable(ARRAY_VTABLE, &[0, ARRAY_ALLOCATE, 0, ARRAY_REALLOCATE]);
        e.register(ARRAY_ALLOCATE, |e, a| ret(e.mem.alloc(a[1] * 0x40)));
        e.register(ARRAY_REALLOCATE, |_, _| ret(REALLOCATED));
        let a = e.mem.alloc(0x14);
        e.mem.set_u32(a, ARRAY_VTABLE);
        e.mem.set_u32(a + 4, buffer);
        e.mem.set_u32(a + 8, size);
        e.mem.set_u32(a + 0xc, reserved);
        Ptr::new(a)
    }

    fn check_scalar_delete(e: &mut Engine, addr: u32, vtable: u32) {
        let a = e.mem.alloc(0x2058);
        assert_eq!(e.call(addr, &args![a, 0u32]).u32(), a);
        assert_eq!(e.mem.u32(a), vtable);
        assert!(calls(e, DEALLOCATE).is_empty(), "bit 0 clear: kept");
        assert_eq!(e.call(addr, &args![a, 3u32]).u32(), a);
        assert_eq!(calls(e, DEALLOCATE), vec![vec![a]]);
    }

    fn array_deleter(addr: u32, vtable: u32) {
        let mut e = bare(&[DEALLOCATE, ARRAY_CLEAR]);
        check_scalar_delete(&mut e, addr, vtable);
        assert!(calls(&e, ARRAY_CLEAR).iter().all(|w| w[1] == 1));
    }

    #[test]
    fn navmesh_array_destructor_body() {
        let mut e = bare(&[ARRAY_CLEAR]);
        check_destroyed(&mut e, 0x006b_3830, VTABLE_NAVMESH_ARRAY);
    }

    #[test]
    fn astar_search_scalar_delete() {
        let mut e = bare(&[DEALLOCATE, 0x0048_3710, 0x0098_2610]);
        check_scalar_delete(&mut e, 0x006b_3850, VTABLE_ASTAR_SEARCH);
        assert_eq!(calls(&e, 0x0098_2610).len(), 2, "006b3270 ran each time");
    }

    #[test]
    fn pov_array_scalar_delete() {
        array_deleter(0x006b_3880, VTABLE_POV_ARRAY);
    }

    #[test]
    fn pov_node_array_scalar_delete() {
        array_deleter(0x006b_38b0, VTABLE_NODE_ARRAY);
    }

    #[test]
    fn pov_link_array_scalar_delete() {
        array_deleter(0x006b_38e0, VTABLE_LINK_ARRAY);
    }

    #[test]
    fn edge_vert_array_scalar_delete() {
        array_deleter(0x006b_3970, VTABLE_EDGE_VERT_ARRAY);
    }

    #[test]
    fn navmesh_array_scalar_delete() {
        array_deleter(0x006b_3a00, VTABLE_NAVMESH_ARRAY);
    }

    #[test]
    fn astar_node_array_scalar_delete() {
        array_deleter(0x006b_42c0, VTABLE_ASTAR_NODE_ARRAY);
    }

    #[test]
    fn astar_node_scrap_array_scalar_delete() {
        let mut e = bare(&[DEALLOCATE, ARRAY_CLEAR]);
        check_scalar_delete(&mut e, 0x006b_4640, VTABLE_ASTAR_NODE_ARRAY);
        assert_eq!(calls(&e, ARRAY_CLEAR).len(), 4, "two per destruction");
    }

    #[test]
    fn link_copy_copies_the_four_parts() {
        let mut e = bare(&[0x0069_85f0, 0x006c_f900]);
        let (a, b) = (e.mem.alloc(0x40), e.mem.alloc(0x40));
        assert_eq!(e.call(0x006b_3910, &args![a, b]).u32(), a);
        assert_eq!(calls(&e, 0x0069_85f0), vec![vec![a, b], vec![a + 8, b + 8]]);
        assert_eq!(
            calls(&e, 0x006c_f900),
            vec![vec![a + 0x10, b + 0x10], vec![a + 0x28, b + 0x28]]
        );
    }

    #[test]
    fn edge_vert_copy_copies_handles_and_indexes() {
        let mut e = bare(&[VERT_HANDLE_COPY, EDGE_HANDLE_COPY]);
        let (a, b) = (e.mem.alloc(0x20), e.mem.alloc(0x20));
        for (k, at) in [0x14u32, 0x18, 0x1c].into_iter().enumerate() {
            e.mem.set_u32(b + at, 7 + k as u32);
        }
        assert_eq!(e.call(0x006b_39a0, &args![a, b]).u32(), a);
        assert_eq!(calls(&e, VERT_HANDLE_COPY), vec![vec![a, b]]);
        assert_eq!(calls(&e, EDGE_HANDLE_COPY), vec![vec![a + 8, b + 8]]);
        assert_eq!(
            [
                e.mem.u32(a + 0x14),
                e.mem.u32(a + 0x18),
                e.mem.u32(a + 0x1c)
            ],
            [7, 8, 9]
        );
    }

    const SEARCH_VTABLE: u32 = 0x4700_0000;
    const CONNECTIONS: u32 = 0x4700_1004;
    const IS_GOAL: u32 = 0x4700_1008;

    /// A search engine: node values (`fHeuristic`, `006b9130`), queue
    /// buckets all 0; returns the engine and the `AStarSearch`.
    fn search_engine() -> (Engine, u32) {
        let mut e = bare(&[0x006a_1fb0, 0x0040_fca0]);
        e.register(ARRAY_CLEAR, |e, a| {
            if a[1] == 0 {
                e.mem.set_u32(a[0] + 8, 0);
            }
            Ret::default()
        });
        e.register(0x0040_1020, |_, _| ret(0x4800_0000));
        e.register(0x00aa_42e0, |_, _| ret(0x4800_0100));
        e.register(ARRAY_SIZE, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(ARRAY_WORD_AT, |e, a| ret(element(e, a[0], a[1], 4)));
        e.register(0x006b_9130, |e, a| st0(f64::from(e.mem.f32(a[0] + 4))));
        e.register(0x006f_4790, |_, _| ret(0));
        e.register(0x006b_9710, |e, a| {
            st0(f64::from(e.mem.f32(a[0])) + f64::from(e.mem.f32(a[0] + 4)))
        });
        e.put_vtable(SEARCH_VTABLE, &[0, CONNECTIONS, IS_GOAL]);
        let s = e.mem.alloc(0x2058);
        e.mem.set_u32(s, SEARCH_VTABLE);
        (e, s)
    }

    /// `GetNodeConnections` gives each node's `children`; `IsGoalNode` is
    /// true for `goal`.
    fn connect(e: &mut Engine, children: Vec<(u32, Vec<u32>)>, goal: u32) {
        e.register_double(CONNECTIONS, move |e, a| {
            let list = children
                .iter()
                .find(|(n, _)| *n == a[1])
                .map(|(_, c)| c.clone())
                .unwrap_or_default();
            let b = e.mem.alloc(4 * list.len() as u32 + 4);
            for (k, c) in list.iter().enumerate() {
                e.mem.set_u32(b + 4 * k as u32, *c);
            }
            e.mem.set_u32(a[2] + 4, b);
            e.mem.set_u32(a[2] + 8, list.len() as u32);
            Ret::default()
        });
        e.register_double(IS_GOAL, move |_, a| ret(u32::from(a[1] == goal)));
    }

    fn node(e: &mut Engine, value: f32) -> u32 {
        let n = e.mem.alloc(0x68);
        e.mem.set_f32(n + 4, value);
        n
    }

    #[test]
    fn search_returns_the_goal_and_queues_children() {
        let (mut e, s) = search_engine();
        let [a, b, c] = [10.0, 5.0, 3.0].map(|v| node(&mut e, v));
        connect(&mut e, vec![(a, vec![b, c])], c);
        e.mem.set_u32(s + SEARCH_QUEUE + 4, a);
        assert_eq!(e.call(0x006b_3a30, &args![s]).u32(), c);
        assert_eq!(e.mem.u32(s + 0x2048), c, "the goal is the best node");
        assert_eq!(e.mem.u32(b + 0x54), a);
        assert_eq!(e.mem.u32(c + 0x54), a);
        assert_eq!(e.mem.u32(s + SEARCH_QUEUE + 4), b, "b is still queued");
        let clears = calls(&e, ARRAY_CLEAR);
        assert_eq!(clears.last().unwrap()[1], 1, "scrap array destroyed");
        assert_eq!(calls(&e, 0x00aa_42e0), vec![vec![0x4800_0000]]);
    }

    #[test]
    fn search_without_goal_keeps_the_last_node_searched() {
        let (mut e, s) = search_engine();
        let [a, b] = [10.0, f32::NAN].map(|v| node(&mut e, v));
        connect(&mut e, vec![(a, vec![b])], 0);
        e.mem.set_u32(s + SEARCH_QUEUE + 4, a);
        assert_eq!(e.call(0x006b_3a30, &args![s]).u32(), 0);
        assert_eq!(e.mem.u32(s + 0x204c), b);
        assert_eq!(e.mem.u32(s + 0x2048), a, "a NaN value is not below");
    }

    #[test]
    fn search_with_an_empty_queue_finds_nothing() {
        let (mut e, s) = search_engine();
        connect(&mut e, vec![], 0);
        e.mem.set_u32(s + 0x204c, 0x1234);
        assert_eq!(e.call(0x006b_3a30, &args![s]).u32(), 0);
        assert_eq!(e.mem.u32(s + 0x204c), 0);
        assert!(calls(&e, IS_GOAL).is_empty());
    }

    #[test]
    fn search_stops_when_a_child_is_the_parent() {
        let (mut e, s) = search_engine();
        let [a, b] = [10.0, 5.0].map(|v| node(&mut e, v));
        connect(&mut e, vec![(a, vec![b])], 0);
        e.mem.set_u32(a + 0x54, b);
        e.mem.set_u32(s + SEARCH_QUEUE + 4, a);
        assert_eq!(e.call(0x006b_3a30, &args![s]).u32(), 0);
        assert!(calls(&e, 0x006f_4790).is_empty(), "nothing queued");
    }

    #[test]
    fn search_in_a_tasklet_sleeps_every_50_nodes() {
        let (mut e, s) = search_engine();
        let chain: Vec<u32> = (0..51).map(|_| node(&mut e, 1.0)).collect();
        let links = chain.windows(2).map(|w| (w[0], vec![w[1]])).collect();
        connect(&mut e, links, 0);
        e.mem.set_u8(s + 0x2054, 1);
        e.mem.set_u32(s + SEARCH_QUEUE + 4, chain[0]);
        assert_eq!(e.call(0x006b_3a30, &args![s]).u32(), 0);
        assert_eq!(calls(&e, 0x0040_fca0), vec![vec![0]]);
        assert_eq!(e.mem.u32(s + 0x204c), chain[50]);
    }

    #[test]
    fn path_is_built_from_the_parent_chain() {
        let mut e = bare(&[0x006a_8890, 0x006a_2630, 0x0069_85f0, 0x006c_f900]);
        e.register(0x006a_1440, |e, a| ret(element(e, a[0], a[1], 12)));
        let nodes = e.mem.alloc(0x10);
        buffer(&mut e, nodes, 0x40);
        let link_buffer = e.mem.alloc(0x200);
        let links = test_array(&mut e, link_buffer, 0, 8);
        let chain: Vec<u32> = (0..3).map(|_| e.mem.alloc(0x68)).collect();
        for (k, n) in chain.iter().enumerate() {
            e.mem.set_u32(n + 8, 0x100 + k as u32);
            if k > 0 {
                e.mem.set_u32(n + 0x54, chain[k - 1]);
            }
        }
        let goal = chain[2];
        assert!(e.call(0x006b_3cb0, &args![0u32, goal, nodes, links]).bool());
        assert_eq!(calls(&e, 0x006a_8890), vec![vec![nodes, 3, 1]]);
        assert_eq!(e.mem.u32(links.addr() + 8), 2);
        for k in 0..3 {
            assert_eq!(e.mem.u32(element(&e, nodes, k, 12)), 0x100 + k);
        }
        let firsts: Vec<Vec<u32>> = calls(&e, 0x0069_85f0).into_iter().step_by(2).collect();
        assert_eq!(
            firsts,
            vec![
                vec![link_buffer + 0x40, chain[2] + 0x14],
                vec![link_buffer, chain[1] + 0x14]
            ]
        );
        assert!(!e.call(0x006b_3cb0, &args![0u32, 0u32, nodes, links]).bool());
    }

    #[test]
    fn heap_node_is_constructed_or_null() {
        let mut e = bare(&[SELF, LOCATION_RECORD_CONSTRUCT]);
        let block = e.mem.alloc(0x68);
        e.mem.set_u32(block + 0x60, 9);
        e.register_double(0x006a_1f90, move |_, _| ret(block));
        assert_eq!(e.call(0x006b_3d90, &args![0x5000u32]).u32(), block);
        assert_eq!(calls(&e, 0x006a_1f90), vec![vec![0x68, 0x5000]]);
        assert_eq!(e.mem.u32(block + 0x60), 0);
        e.register(0x006a_1f90, |_, _| ret(0));
        assert_eq!(e.call(0x006b_3d90, &args![0x5000u32]).u32(), 0);
        assert_eq!(calls(&e, SELF).len(), 2, "constructed once");
    }

    #[test]
    fn node_map_is_cleared() {
        let mut e = bare(&[0x0040_3d30]);
        let m = e.mem.alloc(0x1fe0);
        e.mem.set_u32(m + 0x1fdc, 5);
        assert_eq!(e.call(0x006b_3de0, &args![m]).u32(), m);
        assert_eq!(e.mem.u32(m + 0x1fdc), 0);
        assert_eq!(calls(&e, 0x0040_3d30), vec![vec![m, 0, 0x1fdc]]);
    }

    #[test]
    fn queue_keeps_each_bucket_in_value_order() {
        let mut e = bare(&[]);
        e.register(0x006f_4790, |_, _| ret(1));
        e.register(0x006b_9710, |e, a| {
            st0(f64::from(e.mem.f32(a[0])) + f64::from(e.mem.f32(a[0] + 4)))
        });
        let q = e.mem.alloc(0x54);
        let slot = q + 8;
        let [a, b, c, d] = [5.0, 3.0, 7.0, f32::NAN].map(|v| node(&mut e, v));
        e.call(0x006b_3e10, &args![q, a]);
        assert_eq!(e.mem.u32(slot), a);
        assert_eq!(e.mem.u32(a + 0x58), slot);
        e.call(0x006b_3e10, &args![q, b]);
        assert_eq!(e.mem.u32(slot), b, "lower value first");
        assert_eq!(e.mem.u32(b + 0x5c), a);
        assert_eq!(e.mem.u32(a + 0x58), b + 0x5c);
        e.call(0x006b_3e10, &args![q, c]);
        assert_eq!(e.mem.u32(a + 0x5c), c, "highest value last");
        assert_eq!(e.mem.u32(c + 0x58), a + 0x5c);
        assert_eq!(e.mem.u32(c + 0x5c), 0);
        e.call(0x006b_3e10, &args![q, d]);
        assert_eq!(e.mem.u32(slot), d, "NaN stops at the head");
        assert_eq!(e.mem.u32(b + 0x58), d + 0x5c);
    }

    #[test]
    fn node_array_initialiser_prepares_with_006d03d0() {
        let mut e = bare(&[0x006d_03d0]);
        let (this, buffer) = array(&mut e);
        e.call(0x006b_3f40, &args![this, 2u32, 5u32]);
        assert_eq!(e.get(this, BSSimpleArray::pBuffer), buffer);
        assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 5);
        assert_eq!(e.get(this, BSSimpleArray::iSize), 5);
        assert_eq!(calls(&e, 0x006d_03d0), vec![vec![this.addr(), buffer, 5]]);
        e.call(0x006b_3f40, &args![this, 0u32, 0u32]);
        assert_eq!(e.get(this, BSSimpleArray::pBuffer), 0);
        assert_eq!(calls(&e, 0x006d_03d0).len(), 1);
    }

    fn full_check(e: &mut Engine) {
        e.register(0x0043_8b90, |e, a| {
            ret(u32::from(e.mem.u32(a[0] + 8) == e.mem.u32(a[0] + 0xc)))
        });
        e.register(0x009a_3910, |_, _| ret(16));
    }

    #[test]
    fn link_index_grows_the_array_when_full() {
        let mut e = bare(&[]);
        full_check(&mut e);
        let a = test_array(&mut e, 0, 0, 0);
        assert_eq!(e.call(0x006b_3fd0, &args![a]).u32(), 0);
        assert_eq!(calls(&e, ARRAY_ALLOCATE), vec![vec![a.addr(), 4]]);
        assert_eq!(e.mem.u32(a.addr() + 0xc), 4);
        assert_eq!(e.call(0x006b_3fd0, &args![a]).u32(), 1, "not full");
        e.mem.set_u32(a.addr() + 8, 4);
        let old = e.mem.u32(a.addr() + 4);
        assert_eq!(e.call(0x006b_3fd0, &args![a]).u32(), 4);
        assert_eq!(calls(&e, ARRAY_REALLOCATE), vec![vec![a.addr(), old, 16]]);
        assert_eq!(e.mem.u32(a.addr() + 4), REALLOCATED);
        assert_eq!(e.mem.u32(a.addr() + 0xc), 16);
        assert_eq!(e.mem.u32(a.addr() + 8), 5);
    }

    #[test]
    fn edge_vert_index_grows_with_006f9eb0() {
        let mut e = bare(&[0x006f_9eb0]);
        full_check(&mut e);
        let a = test_array(&mut e, 0x100, 4, 4);
        assert_eq!(e.call(0x006b_4060, &args![a]).u32(), 4);
        assert_eq!(calls(&e, 0x006f_9eb0), vec![vec![a.addr(), 16, 4]]);
        assert_eq!(e.mem.u32(a.addr() + 0xc), 16);
        let b = test_array(&mut e, 0, 0, 0);
        assert_eq!(e.call(0x006b_4060, &args![b]).u32(), 0);
        assert_eq!(calls(&e, ARRAY_ALLOCATE), vec![vec![b.addr(), 4]]);
    }

    #[test]
    fn edge_verts_are_constructed_in_place() {
        let mut e = bare(&[SELF, EDGE_HANDLE_CONSTRUCT]);
        e.register(0x006e_6da0, |_, a| ret(a[1]));
        let b = e.mem.alloc(0x40);
        e.call(0x006b_40f0, &args![0u32, b, 2u32]);
        assert_eq!(
            calls(&e, 0x006e_6da0),
            vec![vec![0x20, b], vec![0x20, b + 0x20]]
        );
        assert_eq!(
            calls(&e, EDGE_HANDLE_CONSTRUCT),
            vec![vec![b + 8], vec![b + 0x28]]
        );
        e.register(0x006e_6da0, |_, _| ret(0));
        e.call(0x006b_40f0, &args![0u32, b, 1u32]);
        assert_eq!(calls(&e, SELF).len(), 2, "a null slot is not constructed");
    }

    #[test]
    fn edge_vert_array_initialiser_constructs_the_elements() {
        let mut e = bare(&[SELF, EDGE_HANDLE_CONSTRUCT]);
        e.register(0x006e_6da0, |_, a| ret(a[1]));
        let (this, buffer) = array(&mut e);
        e.call(0x006b_4190, &args![this, 0u32, 2u32]);
        assert_eq!(e.get(this, BSSimpleArray::iSize), 2);
        assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 2);
        assert_eq!(
            calls(&e, 0x006e_6da0),
            vec![vec![0x20, buffer], vec![0x20, buffer + 0x20]]
        );
    }

    #[test]
    fn scrap_array_takes_the_thread_scrap_heap() {
        let mut e = bare(&[]);
        e.register(0x0040_1020, |_, _| ret(0x777));
        e.register(0x00aa_42e0, |_, _| ret(0x888));
        let a = e.mem.alloc(0x14);
        e.mem.set_u32(a + 8, 3);
        assert_eq!(e.call(0x006b_4220, &args![a]).u32(), a);
        assert_eq!(e.mem.u32(a), VTABLE_ASTAR_NODE_SCRAP_ARRAY);
        assert_eq!(e.mem.u32(a + 0x10), 0x888);
        assert_eq!(e.mem.u32(a + 8), 0);
        assert_eq!(calls(&e, 0x00aa_42e0), vec![vec![0x777]]);
    }

    #[test]
    fn astar_node_array_destructor() {
        let mut e = bare(&[ARRAY_CLEAR]);
        check_destroyed(&mut e, 0x006b_42a0, VTABLE_ASTAR_NODE_ARRAY);
    }

    #[test]
    fn scrap_array_destructor_runs_the_base_destructor() {
        let mut e = bare(&[ARRAY_CLEAR]);
        let a = e.mem.alloc(0x14);
        e.call(0x006b_42f0, &args![a]);
        assert_eq!(e.mem.u32(a), VTABLE_ASTAR_NODE_ARRAY);
        assert_eq!(calls(&e, ARRAY_CLEAR), vec![vec![a, 1], vec![a, 1]]);
    }

    #[test]
    fn queue_pops_the_first_non_empty_bucket() {
        let mut e = bare(&[]);
        let q = e.mem.alloc(0x54);
        assert_eq!(e.call(0x006b_4350, &args![q]).u32(), 0);
        let (a, b) = (e.mem.alloc(0x68), e.mem.alloc(0x68));
        let slot = q + 4 + 3 * 4;
        e.mem.set_u32(slot, a);
        e.mem.set_u32(a + 0x58, slot);
        e.mem.set_u32(a + 0x5c, b);
        e.mem.set_u32(b + 0x58, a + 0x5c);
        e.mem.set_u32(q + 4 + 19 * 4, 0x9999);
        assert_eq!(e.call(0x006b_4350, &args![q]).u32(), a);
        assert_eq!(e.mem.u32(slot), b);
        assert_eq!(e.mem.u32(b + 0x58), slot);
        assert_eq!([e.mem.u32(a + 0x58), e.mem.u32(a + 0x5c)], [0, 0]);
        assert_eq!(e.call(0x006b_4350, &args![q]).u32(), b);
        assert_eq!(e.mem.u32(slot), 0);
    }

    #[test]
    fn astar_node_constructor_clears_the_node() {
        let mut e = bare(&[SELF, LOCATION_RECORD_CONSTRUCT]);
        let n = e.mem.alloc(0x68);
        for at in (0..0x68).step_by(4) {
            e.mem.set_u32(n + at, 0xffff_ffff);
        }
        assert_eq!(e.call(0x006b_43f0, &args![n]).u32(), n);
        assert_eq!([e.mem.u32(n), e.mem.u32(n + 4)], [0, 0]);
        assert_eq!(e.mem.u32(n + 8), 0xffff_ffff, "NodeData is left");
        for at in [0x54, 0x58, 0x5c, 0x60, 0x64] {
            assert_eq!(e.mem.u32(n + at), 0);
        }
        assert_eq!(
            calls(&e, LOCATION_RECORD_CONSTRUCT),
            vec![vec![n + 0x24], vec![n + 0x3c]]
        );
    }

    #[test]
    fn link_array_set_size_branches() {
        let mut e = bare(&[ARRAY_CLEAR, 0x006a_2630, 0x0072_ba80, 0x004a_9b50]);
        e.register(0x006a_8500, |e, a| {
            e.mem.set_u32(a[0] + 4, 0);
            Ret::default()
        });
        let a = test_array(&mut e, 0, 0, 0);
        let p = a.addr();
        e.call(0x006b_4450, &args![a, 0u32, 1u32]);
        assert_eq!(calls(&e, ARRAY_CLEAR), vec![vec![p, 1]]);
        e.call(0x006b_4450, &args![a, 8u32, 0u32]);
        let buffer = e.mem.u32(p + 4);
        assert_eq!(calls(&e, ARRAY_ALLOCATE), vec![vec![p, 8]]);
        assert_eq!(calls(&e, 0x006a_2630), vec![vec![p, buffer, 8]]);
        assert_eq!([e.mem.u32(p + 8), e.mem.u32(p + 0xc)], [8, 8]);
        e.call(0x006b_4450, &args![a, 6u32, 0u32]);
        assert_eq!(calls(&e, 0x0072_ba80), vec![vec![p, buffer + 0x180, 2]]);
        assert_eq!([e.mem.u32(p + 8), e.mem.u32(p + 0xc)], [6, 8]);
        e.call(0x006b_4450, &args![a, 7u32, 0u32]);
        assert_eq!(calls(&e, 0x006a_2630)[1], vec![p, buffer + 0x180, 1]);
        e.call(0x006b_4450, &args![a, 2u32, 1u32]);
        assert_eq!([e.mem.u32(p + 8), e.mem.u32(p + 0xc)], [2, 2], "shrunk");
        assert_eq!(calls(&e, 0x004a_9b50).len(), 1, "moved to a new buffer");
        let moved = e.mem.u32(p + 4);
        e.call(0x006b_4450, &args![a, 4u32, 0u32]);
        assert_eq!(
            calls(&e, ARRAY_REALLOCATE),
            vec![vec![p, moved, 4]],
            "size 2 = reserve 2: reallocated"
        );
        assert_eq!(e.mem.u32(p + 4), REALLOCATED);
        assert_eq!([e.mem.u32(p + 8), e.mem.u32(p + 0xc)], [4, 4]);
    }

    #[test]
    fn link_buffer_change_branches() {
        let mut e = bare(&[0x004a_9b50, 0x006a_8500]);
        let a = test_array(&mut e, 0, 0, 0);
        let p = a.addr();
        e.call(0x006b_4590, &args![a, 4u32, 0u32]);
        assert_ne!(e.mem.u32(p + 4), 0);
        assert_eq!(e.mem.u32(p + 0xc), 4);
        let old = e.mem.u32(p + 4);
        e.call(0x006b_4590, &args![a, 8u32, 4u32]);
        assert_eq!(calls(&e, ARRAY_REALLOCATE), vec![vec![p, old, 8]]);
        assert_eq!(e.mem.u32(p + 4), REALLOCATED);
        assert_eq!(e.mem.u32(p + 0xc), 4, "the caller sets the reserve");
        e.call(0x006b_4590, &args![a, 8u32, 2u32]);
        let fresh = e.mem.u32(p + 4);
        assert_eq!(calls(&e, 0x004a_9b50), vec![vec![p, fresh, REALLOCATED, 2]]);
        assert_eq!(calls(&e, 0x006a_8500), vec![vec![p]]);
    }

    #[test]
    fn astar_node_array_constructor() {
        let mut e = bare(&[]);
        let a = check_constructed(&mut e, 0x006b_4670, VTABLE_ASTAR_NODE_ARRAY);
        assert_eq!(e.mem.u32(a + 8), 0);
    }
}
