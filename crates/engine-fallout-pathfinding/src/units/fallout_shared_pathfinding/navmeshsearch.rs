//! `fallout shared/pathfinding/navmeshsearch.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `NavMeshSearch` is the navmesh A* search: an
//! `AStarSearch<NavMeshSearchNode, NavMeshSearchLink>` base (vtable slot `+4`
//! is `GetNodeConnections`, slot `+8` is `IsGoalNode`) with the request, the
//! start and goal nodes, the bad links, the high-level solution and a
//! `DoorLock`. This unit also holds the template code of that base: the node
//! heap, the node map, the open queue (`AStarQueue`, 20 fitness buckets on
//! PC; the Xbox PDB says 80) and the arrays the search fills.
//!
//! PC layout: the `AStarSearch` base is 0x2058 bytes (the Xbox PDB says
//! 0x205C; the node map starts at `+0x14`, not `+0x18`), so every member
//! after the node heap sits 4 bytes lower than in the Xbox PDB.
//!
//! x87 note: the game computes in extended precision and stores `float`
//! results; the translations compute in `f64` and round to `f32` at each
//! `float` store (docs/ENGINE_CRATE.md). Floats copied through `FLD`/`FSTP`
//! go through [`x87_quiet`] / [`x87_quiet_bits`].

#[allow(unused_imports)]
use crate::prelude::*;

pub(crate) use super::navmeshsearchclosepoint::AStarNodeNavMeshSearch as AStarNode;

/// `00401030(p)` (cdecl): frees a block through the memory manager.
const FREE: u32 = 0x0040_1030;
/// `00401020()`: the memory manager singleton (`011f6238`).
const MEMORY_MANAGER: u32 = 0x0040_1020;
/// `00aa42e0(manager)`: `MemoryManager::GetThreadScrapHeap` (map name).
const GET_THREAD_SCRAP_HEAP: u32 = 0x00aa_42e0;
/// `00403d30(dest, value, count)` (cdecl): fills memory (`memset`).
const MEMSET: u32 = 0x0040_3d30;
/// `0040fc90()`: the current thread id (through the import at `00fdf1d0`).
const CURRENT_THREAD_ID: u32 = 0x0040_fc90;
/// `0040fca0(milliseconds)` (cdecl): sleeps (through the import at
/// `00fdf104`).
const SLEEP: u32 = 0x0040_fca0;
/// `00413f40(node)`: the address of the node's data (`node + 8`).
const NODE_DATA: u32 = 0x0041_3f40;
/// `00416870(out, x, y, z)`: builds a 3-float vector at `out`.
const VECTOR_SET: u32 = 0x0041_6870;
/// `00439ef0(this, out, other)`: `out = this - other` for 3-float vectors,
/// returns `out`.
const VECTOR_SUBTRACT: u32 = 0x0043_9ef0;
/// `0044ddc0(array)`: a `BSSimpleArray`'s `iSize` (`+8`).
const ARRAY_SIZE: u32 = 0x0044_ddc0;
/// `00457990(vector)`: the vector's length, in ST0.
const VECTOR_LENGTH: u32 = 0x0045_7990;
/// `00595c80(vector)`: a length of the vector, in ST0 (`IsGoalNode`
/// compares it with the request's radius).
const VECTOR_LENGTH_595C80: u32 = 0x0059_5c80;
/// `00589850(vector)`: the length of the first two components, in ST0.
const FLAT_LENGTH: u32 = 0x0058_9850;
/// `004a7290(vector)`: a length measure of the vector in ST0 that
/// `GetNodeConnections` compares with the square of the request's centre
/// radius.
const VECTOR_MEASURE_4A7290: u32 = 0x004a_7290;
/// `00483710(this)`: the destructor the `AStarSearch` destructor runs on its
/// node map and its queue.
const MEMBER_DESTRUCT_483710: u32 = 0x0048_3710;
/// `004839c0(form id)` (cdecl): the form with that id, or 0.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `004b13c0(vector)` (cdecl): `GetZAngleFromVector` (map name), in ST0.
const GET_Z_ANGLE_FROM_VECTOR: u32 = 0x004b_13c0;
/// `004e44f0(&value, minimum, maximum)` (cdecl): `BSWrap` (map name).
const BS_WRAP: u32 = 0x004e_44f0;
/// `00408840(value)` (cdecl): a float function of one float, in ST0 (the
/// turn penalty applies it to an angle and to a distance difference).
const FLOAT_FUNCTION_408840: u32 = 0x0040_8840;
/// `00495460(node)`: the float at `+0x1C` of a `NavMeshStaticAvoidNode`, in
/// ST0.
const STATIC_AVOID_NODE_COST: u32 = 0x0049_5460;
/// `004f5d90(node, heuristic)`: stores the A* node's heuristic.
const NODE_SET_HEURISTIC: u32 = 0x004f_5d90;
/// `00501e50(lock)`: `DoorLock::~DoorLock` (map name).
const DOOR_LOCK_DESTRUCT: u32 = 0x0050_1e50;
/// `00501e80(lock, door)`: `DoorLock::SetFromDoorRef` (map name).
const DOOR_LOCK_SET_FROM_DOOR: u32 = 0x0050_1e80;
/// `00502450(lock, actor lock data, door, &flag)`:
/// `DoorLock::CanActorUnlockDoorLock` (map name).
const DOOR_LOCK_CAN_UNLOCK: u32 = 0x0050_2450;
/// `00518000(base form)`: `TESObjectDOOR::IsMinimalUse` (map name).
const IS_MINIMAL_USE: u32 = 0x0051_8000;
/// `0052ce20(queue, fitness)`: sets the queue's maximum fitness.
const QUEUE_SET_MAX_FITNESS: u32 = 0x0052_ce20;
/// `005330e0(&value, minimum, maximum)` (cdecl): clamps the float in place.
const CLAMP: u32 = 0x0053_30e0;
/// `00558220(mesh, out, triangle)`: `NavMesh::GetCenter` (map name), returns
/// `out`.
const GET_CENTER: u32 = 0x0055_8220;
/// `005582f0(mesh, triangle)`: the 16-byte record of the triangle.
const GET_TRIANGLE: u32 = 0x0055_82f0;
/// `0057b460(door)`: a test on the door reference; a door that passes it
/// cannot be crossed when the request does not ignore locks.
const DOOR_TEST_57B460: u32 = 0x0057_b460;
/// `006815c0(this)`: an empty constructor (returns `this`) the code runs on
/// its two edge-vertex locals.
const VECTOR_CONSTRUCT: u32 = 0x0068_15c0;
/// `0068f0c0(mesh, triangle, edge, out_a, out_b)`: the two vertices of the
/// triangle's edge.
const GET_EDGE_VERTICES: u32 = 0x0068_f0c0;
/// `0068f1d0(triangle record, edge)`: whether the edge's bit is set in the
/// record's flags (`+0xC`).
const TRIANGLE_EDGE_FLAG: u32 = 0x0068_f1d0;
/// `0068f230(mesh, triangle, edge)`: `NavMesh::GetEdgeExtraInfo` (map name):
/// the extra info of the edge, or 0; its first word is the type.
const GET_EDGE_EXTRA_INFO: u32 = 0x0068_f230;
/// `0068f2c0(triangle record, edge)`: the edge's 16-bit link (`+6 + 2*edge`).
const TRIANGLE_EDGE_LINK: u32 = 0x0068_f2c0;
/// `0068f460(mesh, triangle, edge, &mesh, &triangle, &edge)`:
/// `NavMesh::GetMatchingEdge_ov3` (map name): the triangle across the edge
/// and the edge index on its side; returns whether there is one.
const GET_MATCHING_EDGE: u32 = 0x0068_f460;
/// `00691140(record, mask)`: whether `mask` meets the record's flags.
const TRIANGLE_HAS_FLAG: u32 = 0x0069_1140;
/// `00691660(mesh, triangle, array)`: `NavMesh::GetStaticAvoidNodesForTri`
/// (map name).
const GET_STATIC_AVOID_NODES: u32 = 0x0069_1660;
/// `00693b70(record)`: whether bit 0x40 of the record's flags is set.
const TRIANGLE_FLAG_40: u32 = 0x0069_3b70;
/// `00693d30(mesh, index)`: the entry `index` of the navmesh's array whose
/// word at `+4` the high-level solution is searched for.
const MESH_LINK_ENTRY: u32 = 0x0069_3d30;
/// `00699af0(mesh, triangle)`: `NavMesh::GetClosedDoorInfoForTriangle` (map
/// name): the address of the door's form id.
const GET_DOOR_INFO: u32 = 0x0069_9af0;
/// `0069ad00(info, &out)`: the test the search start runs on each
/// `NavMeshInfo` of the high-level solution.
const INFO_TEST_69AD00: u32 = 0x0069_ad00;
/// `0069e390(request)`: the request's avoid node array (`+0x94`), or 0.
const REQUEST_AVOID_NODES: u32 = 0x0069_e390;
/// `0069ef80(request)`: the request's `fActorRadius` (`+0x68`), in ST0.
const REQUEST_ACTOR_RADIUS: u32 = 0x0069_ef80;
/// `006a1440(array, index)`: the address of element `index` of a 12-byte
/// element array.
const ARRAY_ELEMENT_12: u32 = 0x006a_1440;
/// `006a1f90(size, heap)` (cdecl): allocates from a scrap heap.
const SCRAP_HEAP_ALLOCATE: u32 = 0x006a_1f90;
/// `006a5ec0(record)`: whether the triangle record has flag 0x200.
const TRIANGLE_FLAG_200: u32 = 0x006a_5ec0;
/// `006a8a30(array, first, count)`: constructs `count` 0x14-byte elements.
const CONSTRUCT_NODES: u32 = 0x006a_8a30;
/// `006a94f0(search, mesh, triangle)`: the node key.
const NODE_KEY: u32 = 0x006a_94f0;
/// `006b3160(array, index)`: the address of element `index` of a 0x14-byte
/// element array.
const ARRAY_ELEMENT_20: u32 = 0x006b_3160;
/// `006b3de0(map)`: the node map's constructor.
const NODE_MAP_CONSTRUCT: u32 = 0x006b_3de0;
/// `006b3eb0(array, size, reserve)`: a `BSSimpleArray`'s initialisation.
const ARRAY_INIT: u32 = 0x006b_3eb0;
/// `006b9130(node)`: the node's `fHeuristic` (`+4`), in ST0.
const NODE_HEURISTIC: u32 = 0x006b_9130;
/// `006b9470(queue)`: the queue's constructor.
const QUEUE_CONSTRUCT: u32 = 0x006b_9470;
/// `006b9710(node)`: the node's fitness (`fCost + fHeuristic`), in ST0.
const NODE_FITNESS: u32 = 0x006b_9710;
/// `006c9900(request)`: the address of the request's `LockData` (`+0x5C`).
const REQUEST_LOCK_DATA: u32 = 0x006c_9900;
/// `006d03d0(array, first, count)`: constructs `count` 12-byte elements.
const CONSTRUCT_LINKS: u32 = 0x006d_03d0;
/// `006dc590(array, size, count)`: reallocates a 12-byte element array.
const REALLOCATE_LINKS: u32 = 0x006d_c590;
/// `006dc990(array, &point)`: `PathingAvoidNodeArray::GetCostForPoint` (map
/// name), in ST0.
const AVOID_COST_FOR_POINT: u32 = 0x006d_c990;
/// `006f4790(queue, node)`: the queue bucket of the node's fitness.
const QUEUE_BUCKET: u32 = 0x006f_4790;
/// `00719b20(array, &key, start, predicate)`: the index of the first element
/// from `start` the predicate accepts, or -1.
const ARRAY_FIND: u32 = 0x0071_9b20;
/// The predicate `GetNodeConnections` passes to [`ARRAY_FIND`].
const FIND_PREDICATE: u32 = 0x009a_3830;
/// `0072ba80(array, first, count)`: destroys `count` elements.
const DESTROY_ELEMENTS: u32 = 0x0072_ba80;
/// `0073cb80(request)`: the request's `bCantOpenDoors` (`+0x98`).
const REQUEST_CANT_OPEN_DOORS: u32 = 0x0073_cb80;
/// `007af430(door)`: the word at `+0x20` (the door's base form).
const DOOR_BASE_FORM: u32 = 0x007a_f430;
/// `007cb2e0(array, &item)`: appends the word at `item` to the array.
const ARRAY_ADD: u32 = 0x007c_b2e0;
/// `008454f0(array, free_buffer)`: empties a `BSSimpleArray`.
const ARRAY_CLEAR: u32 = 0x0084_54f0;
/// `00877a30(array, index)`: the address of element `index` of a word array.
const ARRAY_ELEMENT_4: u32 = 0x0087_7a30;
/// `00982610(heap)`: the node heap's destructor.
const NODE_HEAP_DESTRUCT: u32 = 0x0098_2610;
/// `00996800(map)`: empties the node map.
const NODE_MAP_CLEAR: u32 = 0x0099_6800;
/// `00996840(map, key)`: the node map bucket of a key.
const NODE_MAP_BUCKET: u32 = 0x0099_6840;
/// `009a3c50(array, size, count)`: reallocates a 0x14-byte element array.
const REALLOCATE_NODES: u32 = 0x009a_3c50;
/// `009b0bd0(request)`: a float of the request in ST0 (`IsGoalNode` squares
/// it into the goal distance).
const REQUEST_VALUE_9B0BD0: u32 = 0x009b_0bd0;
/// `00aa5410(heap, size)`: the scrap heap's constructor.
const SCRAP_HEAP_CONSTRUCT: u32 = 0x00aa_5410;
/// `00b00a00()`: `BSTaskletManager::QInstance` (map name).
const TASKLET_MANAGER: u32 = 0x00b0_0a00;
/// `0042f7d0(this, count)`, `0042fb00(this)`, `0042f4c0(this, &value)`,
/// `0042fa40(this)`, `0042f830(this)`: the local array and the local
/// pointer the search start fills from the high-level solution
/// (constructor, constructor, assignment, destructor, destructor).
const LOCAL_ARRAY_CONSTRUCT: u32 = 0x0042_f7d0;
const LOCAL_POINTER_CONSTRUCT: u32 = 0x0042_fb00;
const LOCAL_POINTER_ASSIGN: u32 = 0x0042_f4c0;
const LOCAL_POINTER_DESTRUCT: u32 = 0x0042_fa40;
const LOCAL_ARRAY_DESTRUCT: u32 = 0x0042_f830;

/// `NavMeshSearch` vtable.
const NAV_MESH_SEARCH_VTABLE: u32 = 0x0106_b63c;
/// `AStarSearch<NavMeshSearchNode, NavMeshSearchLink>` vtable.
const A_STAR_SEARCH_VTABLE: u32 = 0x0106_b650;
/// The vtable `006a7f60` / `006a7f90` set (`BSSimpleArray` of
/// `NavMeshStaticAvoidNode*`).
const STATIC_AVOID_ARRAY_VTABLE: u32 = 0x0106_b660;
/// The vtable `006a8530` / `006a8600` set (`BSScrapArray` of A* node
/// pointers).
const NODE_SCRAP_ARRAY_VTABLE: u32 = 0x0106_b674;
/// The vtable `006a8a00` / `006a85b0` set (`BSSimpleArray` of A* node
/// pointers).
const NODE_ARRAY_VTABLE: u32 = 0x0106_b688;

/// `0.01` (`double`, the `float` 0.01 widened).
const ONE_HUNDREDTH: u32 = 0x0101_6408;
/// `100.0` (`double`).
const HUNDRED_DOUBLE: u32 = 0x0101_7a40;
/// `0.25` (`double`).
const QUARTER: u32 = 0x0102_90b0;
/// `0.0` (`double`).
const ZERO: u32 = 0x0101_2060;
/// `100.0f`: the cost factor of a minimal-use door and of a bad link.
const HUNDRED: u32 = 0x0101_6410;
/// `5.0f`: the cost factor of a link between water and land.
const SIDE_CHANGE_COST: u32 = 0x0101_712c;
/// `4.0f`: the cost factor of a link between two water triangles.
const SAME_SIDE_COST: u32 = 0x0101_e570;
/// `512.0f`: the edge width that halves the cost no more.
const EDGE_WIDTH_SCALE: u32 = 0x0102_31a0;
/// `0.5` (`double`).
const HALF: u32 = 0x0101_1588;
/// `pi` and `-pi` (`float`).
const PI_FLOAT: u32 = 0x0102_b3c8;
const MINUS_PI_FLOAT: u32 = 0x0106_b648;
/// `pi` (`double`).
const PI_DOUBLE: u32 = 0x0101_ff40;
/// A `float` in `.data` (1.0 in the exe) that scales the turn penalty.
const TURN_PENALTY_SCALE: u32 = 0x0119_cdf8;
/// The settings object whose float `GetNodeConnections` compares with the
/// actor radius, and `00403e20(ECX)`, the address of its float.
const RADIUS_SETTING: u32 = 0x011d_6ef0;
const SETTING_VALUE: u32 = 0x0040_3e20;

/// Triangle flag of a closed door.
const TRIANGLE_DOOR_FLAG: u32 = 0x1000;
/// Triangle flag the actor radius check tests.
const TRIANGLE_NARROW_FLAG: u32 = 0x10;
/// Triangle flag of a triangle with static avoid nodes.
const TRIANGLE_AVOID_FLAG: u32 = 0x8000_0000;

/// Number of fitness buckets of the PC `AStarQueue`.
const QUEUE_BUCKETS: u32 = 20;

layout! {
    /// `AStarSearch<NavMeshSearchNode, NavMeshSearchLink>` (Xbox PDB), 0x2058
    /// bytes on PC (PC offsets).
    pub struct AStarSearchNavMesh: 0x2058 {
        /// `NodeHeap` (Xbox PDB): `AStarNodeHeap<..., 512>`, a `ScrapHeap`.
        0x0004 NodeHeap: Ptr,
        /// `AStarNodeMap` (Xbox PDB): `AStarMap<AStarNode<...>>`.
        0x0014 AStarNodeMap: Ptr,
        /// `SearchQueue` (Xbox PDB): `AStarQueue<...>`.
        0x1FF4 SearchQueue: Ptr,
        /// `BestNodeSoFar` (Xbox PDB).
        0x2048 BestNodeSoFar: Ptr<AStarNode>,
        /// `pLastSearchedNode` (Xbox PDB).
        0x204C pLastSearchedNode: Ptr<AStarNode>,
        /// `pStartAStarNode` (Xbox PDB).
        0x2050 pStartAStarNode: Ptr<AStarNode>,
        /// `bIsRunningInTasklet` (Xbox PDB).
        0x2054 bIsRunningInTasklet: u8,
    }

    /// `NavMeshSearch` (Xbox PDB), 0x20A4 bytes on PC (PC offsets).
    pub struct NavMeshSearch: 0x20A4 {
        /// `pCurrentRequest` (Xbox PDB): `const PathingRequest*`.
        0x2058 pCurrentRequest: Ptr,
        /// `StartNode.pNavMesh` (Xbox PDB).
        0x205C StartNode_pNavMesh: Ptr,
        /// `StartNode.sTriangle` (Xbox PDB).
        0x2060 StartNode_sTriangle: u16,
        /// `GoalNode.pNavMesh` (Xbox PDB).
        0x2070 GoalNode_pNavMesh: Ptr,
        /// `GoalNode.sTriangle` (Xbox PDB).
        0x2074 GoalNode_sTriangle: u16,
        /// `pBadNodes` (Xbox PDB): `const BSSimpleArray<NavMeshSearchLink>*`.
        0x2084 pBadNodes: Ptr,
        /// `pHighLevelSolution` (Xbox PDB):
        /// `const BSSimpleArray<const NavMeshInfo*>*`.
        0x2088 pHighLevelSolution: Ptr,
    }

    /// `AStarQueue<NavMeshSearchNode, NavMeshSearchLink>` (Xbox PDB), 0x54
    /// bytes on PC: the maximum fitness and 20 bucket lists.
    pub struct AStarQueueNavMesh: 0x54 {
        /// `fMaxFitness` (Xbox PDB).
        0x00 fMaxFitness: f32,
    }
}

/// Offset of the `StartNode` (`NavMeshSearchNode`, 0x14 bytes) in
/// `NavMeshSearch`.
const START_NODE: u32 = 0x205c;
/// Offset of the `GoalNode`.
const GOAL_NODE: u32 = 0x2070;
/// Offset of the `TempDoorLock` (`DoorLock`).
const TEMP_DOOR_LOCK: u32 = 0x208c;
/// Offset of the first queue bucket in `AStarQueue`.
const QUEUE_FIRST_BUCKET: u32 = 0x4;

// `AStarNode<NavMeshSearchNode, NavMeshSearchLink>` members that
// `navmeshsearchclosepoint.rs` does not declare (Xbox PDB names, PC offsets).
#[allow(non_upper_case_globals)]
impl AStarNode {
    /// `fCost` (Xbox PDB).
    pub const fCost: Field<AStarNode, f32> = Field::new(0x00);
    /// `pParentAStarNode` (Xbox PDB).
    pub const pParentAStarNode: Field<AStarNode, Ptr<AStarNode>> = Field::new(0x28);
    /// `ppPrevNode` (Xbox PDB): the link that points at this node in its
    /// queue list.
    pub const ppPrevNode: Field<AStarNode, u32> = Field::new(0x2c);
    /// `pNextNode` (Xbox PDB): the next node in the queue list.
    pub const pNextNode: Field<AStarNode, u32> = Field::new(0x30);
    /// `iKey` (Xbox PDB).
    pub const iKey: Field<AStarNode, u32> = Field::new(0x34);
    /// `pNextNodeInBucket` (Xbox PDB): the next node in the map bucket.
    pub const pNextNodeInBucket: Field<AStarNode, u32> = Field::new(0x38);
}

/// Copies a `float` the way `FLD dword` / `FSTP dword` does.
fn copy_float(e: &mut Engine, from: u32, to: u32) {
    let bits = e.mem.u32(from);
    e.mem.set_u32(to, x87_quiet_bits(bits));
}

/// A `float` global loaded with `FLD` and stored again.
fn float_global(e: &Engine, addr: u32) -> f32 {
    x87_quiet(e.global::<f32>(addr))
}

// Translated from 006a6ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshSearch::_scalar_deleting_destructor_` (Xbox PDB): destroys the
/// search and frees it when bit 0 of `flags` is set.
pub fn nav_mesh_search_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NavMeshSearch>,
    flags: u32,
) -> Ptr<NavMeshSearch> {
    fn_006a6b00(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 006a6b00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006a6b00 - the exception-unwinding frame (FS:[0]) is not translated
/// The `NavMeshSearch` destructor: destroys the `TempDoorLock`, then the
/// `AStarSearch` base.
pub fn fn_006a6b00(e: &mut Engine, this: Ptr<NavMeshSearch>) {
    e.mem.set_u32(this.addr(), NAV_MESH_SEARCH_VTABLE);
    e.call(DOOR_LOCK_DESTRUCT, &args![this.byte_add(TEMP_DOOR_LOCK)]);
    fn_006a7bc0(e, this.cast());
}

// Translated from 006a6b70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006a6b70 - the exception-unwinding frame (FS:[0]) is not translated
/// Runs the navmesh search for `request` from the triangle `start` to the
/// triangle `goal` (each a `NavMeshSearchLink`-like pair: navmesh, then the
/// 16-bit triangle at `+4`): empties the output arrays, resets the search,
/// fills the start and goal nodes with their centres, sets the queue's
/// maximum fitness to twice the start-goal distance, adds the start node
/// with that distance as its heuristic, and keeps `high_level_solution`.
/// When there is one, it builds a local array with one entry per
/// `NavMeshInfo` of the solution that passes `0069ad00`. Then it searches
/// (allowing a partial path) into `nodes` and `links` and returns whether a
/// goal was found.
#[allow(clippy::too_many_arguments)]
pub fn fn_006a6b70(
    e: &mut Engine,
    this: Ptr<NavMeshSearch>,
    request: Ptr,
    start: Ptr,
    goal: Ptr,
    nodes: Ptr,
    links: Ptr,
    high_level_solution: Ptr,
) -> bool {
    e.set(this, NavMeshSearch::pCurrentRequest, request);
    e.call(ARRAY_CLEAR, &args![nodes, 0u32]);
    e.call(ARRAY_CLEAR, &args![links, 0u32]);
    fn_006a7c40(e, this.cast());

    let center = e.mem.alloc(0xc);
    let start_mesh = e.mem.u32(start.addr());
    e.set(
        this,
        NavMeshSearch::StartNode_pNavMesh,
        Ptr::new(start_mesh),
    );
    let start_triangle = e.mem.u16(start.addr() + 4);
    e.set(this, NavMeshSearch::StartNode_sTriangle, start_triangle);
    e.call(GET_CENTER, &args![start_mesh, center, start_triangle]);
    for i in 0..3 {
        copy_float(e, center + 4 * i, this.addr() + START_NODE + 8 + 4 * i);
    }

    let goal_mesh = e.mem.u32(goal.addr());
    e.set(this, NavMeshSearch::GoalNode_pNavMesh, Ptr::new(goal_mesh));
    let goal_triangle = e.mem.u16(goal.addr() + 4);
    e.set(this, NavMeshSearch::GoalNode_sTriangle, goal_triangle);
    let goal_center = e.mem.alloc(0xc);
    let result = e
        .call(GET_CENTER, &args![goal_mesh, goal_center, goal_triangle])
        .u32();
    for i in 0..3 {
        let word = e.mem.u32(result + 4 * i);
        e.mem.set_u32(center + 4 * i, word);
    }
    for i in 0..3 {
        copy_float(e, center + 4 * i, this.addr() + GOAL_NODE + 8 + 4 * i);
    }

    let scratch = e.mem.alloc(0x48);
    let distance = start_goal_distance(e, this, scratch);
    let max_fitness = (distance + distance) as f32;
    fn_006a7d30(e, this.cast(), max_fitness);
    let heuristic = start_goal_distance(e, this, scratch + 0x24) as f32;
    let key = e
        .call(NODE_KEY, &args![this, start_mesh, start_triangle])
        .u32();
    a_star_search_add_start_node(e, this.cast(), key, this.byte_add(START_NODE), heuristic);

    e.set(this, NavMeshSearch::pHighLevelSolution, high_level_solution);
    if !high_level_solution.is_null() {
        let infos = e.mem.alloc(0x10);
        let pointer = e.mem.alloc(4);
        let count = e.call(ARRAY_SIZE, &args![high_level_solution]).u32();
        e.call(LOCAL_ARRAY_CONSTRUCT, &args![infos, count]);
        let mut index = 0u32;
        while index < e.call(ARRAY_SIZE, &args![high_level_solution]).u32() {
            e.call(LOCAL_POINTER_CONSTRUCT, &args![pointer]);
            let element = fn_006a7ad0(e, high_level_solution, index);
            let info = e.mem.u32(element);
            if e.call(INFO_TEST_69AD00, &args![info, pointer]).bool() {
                let slot = e.call(ARRAY_ELEMENT_4, &args![infos, index]).u32();
                e.call(LOCAL_POINTER_ASSIGN, &args![slot, pointer]);
            }
            e.call(LOCAL_POINTER_DESTRUCT, &args![pointer]);
            index += 1;
        }
        e.call(LOCAL_ARRAY_DESTRUCT, &args![infos]);
        e.mem.free(pointer);
        e.mem.free(infos);
    }
    e.mem.free(scratch);
    e.mem.free(goal_center);
    e.mem.free(center);
    a_star_search_search(e, this.cast(), nodes, links, true)
}

/// `|start centre - goal centre|` as `fn_006a6b70` measures it (the centres
/// through [`fn_006a6eb0`], the difference `0x00439ef0`, the length
/// `00457990`), with 0x24 bytes of locals at `scratch`.
fn start_goal_distance(e: &mut Engine, this: Ptr<NavMeshSearch>, scratch: u32) -> f64 {
    let goal = fn_006a6eb0(e, this.byte_add(GOAL_NODE), Ptr::new(scratch));
    let start = fn_006a6eb0(e, this.byte_add(START_NODE), Ptr::new(scratch + 0xc));
    let difference = e
        .call(VECTOR_SUBTRACT, &args![start, scratch + 0x18, goal])
        .u32();
    e.call(VECTOR_LENGTH, &args![difference]).f64()
}

// Translated from 006a6eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the centre of the navmesh search node at `this`
/// (`NavMeshSearchNode`: `CenterX/Y/Z` at `+8`) as a vector at `out` and
/// returns `out`.
pub fn fn_006a6eb0(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let x = x87_quiet_bits(e.mem.u32(this.addr() + 0x8));
    let y = x87_quiet_bits(e.mem.u32(this.addr() + 0xc));
    let z = x87_quiet_bits(e.mem.u32(this.addr() + 0x10));
    e.call(VECTOR_SET, &args![out, x, y, z]);
    out
}

// Translated from 006a6ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshSearch::SetNodeData` (Xbox PDB): stores the navmesh, the
/// triangle and the triangle's centre in the A* node, and sets its
/// heuristic to 0.01 times the distance from that centre to the goal
/// node's centre.
pub fn nav_mesh_search_set_node_data(
    e: &mut Engine,
    this: Ptr<NavMeshSearch>,
    node: Ptr<AStarNode>,
    mesh: Ptr,
    triangle: u16,
) {
    let data = e.call(NODE_DATA, &args![node]).u32();
    e.mem.set_u32(data, mesh.addr());
    e.mem.set_u16(data + 4, triangle);
    let center = e.mem.alloc(0xc);
    e.call(GET_CENTER, &args![mesh, center, triangle]);
    for i in 0..3 {
        copy_float(e, center + 4 * i, data + 8 + 4 * i);
    }
    let scratch = e.mem.alloc(0x24);
    let goal = fn_006a6eb0(e, this.byte_add(GOAL_NODE), Ptr::new(scratch));
    let here = fn_006a6eb0(e, Ptr::new(data), Ptr::new(scratch + 0xc));
    let difference = e
        .call(VECTOR_SUBTRACT, &args![here, scratch + 0x18, goal])
        .u32();
    let distance = e.call(VECTOR_LENGTH, &args![difference]).f64() as f32;
    let one_hundredth: f64 = e.global(ONE_HUNDREDTH);
    let heuristic = (distance as f64 * one_hundredth) as f32;
    e.call(NODE_SET_HEURISTIC, &args![node, heuristic.to_bits()]);
    e.mem.free(scratch);
    e.mem.free(center);
}

/// The locals of [`nav_mesh_search_get_node_connections`] passed by address.
struct ConnectionLocals {
    /// `GetMatchingEdge`'s outputs: navmesh, triangle, edge on its side.
    neighbour_mesh: u32,
    neighbour_triangle: u32,
    neighbour_edge: u32,
    /// The high-level solution key.
    key: u32,
    /// `DoorLock::CanActorUnlockDoorLock`'s flag.
    unlock_flag: u32,
    /// The neighbour's centre.
    center: u32,
    /// The found or created node.
    node: u32,
    /// The ratio [`CLAMP`] clamps.
    ratio: u32,
    /// The two edge vertices.
    vertex_a: u32,
    vertex_b: u32,
    /// The static avoid node array (0x10 bytes).
    avoid_nodes: u32,
    /// Vectors and differences (0x80 bytes).
    scratch: u32,
}

impl ConnectionLocals {
    fn new(e: &mut Engine) -> Self {
        let mut a = |size| e.mem.alloc(size);
        ConnectionLocals {
            neighbour_mesh: a(4),
            neighbour_triangle: a(4),
            neighbour_edge: a(4),
            key: a(4),
            unlock_flag: a(4),
            center: a(0xc),
            node: a(4),
            ratio: a(4),
            vertex_a: a(0xc),
            vertex_b: a(0xc),
            avoid_nodes: a(0x10),
            scratch: a(0x80),
        }
    }

    fn free(self, e: &mut Engine) {
        for block in [
            self.neighbour_mesh,
            self.neighbour_triangle,
            self.neighbour_edge,
            self.key,
            self.unlock_flag,
            self.center,
            self.node,
            self.ratio,
            self.vertex_a,
            self.vertex_b,
            self.avoid_nodes,
            self.scratch,
        ] {
            e.mem.free(block);
        }
    }
}

// Translated from 006a6fa0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006a6fa0 - the exception-unwinding frame (FS:[0]) is not translated
/// `NavMeshSearch::GetNodeConnections` (Xbox PDB): for each of the three
/// edges of the node's triangle, finds the triangle across it and, unless
/// the link is excluded, adds the neighbour's A* node (created or made
/// cheaper) to `connections`.
///
/// A link is excluded when the high-level solution is set and the edge is
/// flagged as leading to a navmesh info that is not in it; when there is no
/// neighbour, the edge's extra info has type 1, or the neighbour is the
/// node itself; when the request's centre radius is positive and the
/// neighbour's centre lies outside it; when it is a closed door the actor
/// cannot open (or the request cannot open doors); when the actor radius is
/// above the radius setting and the neighbour is flagged narrow (0x10);
/// when the neighbour is water (0x200) and the request can neither swim nor
/// fly; and when the node lookup answers neither "created" nor "cheaper".
///
/// The link cost is the flat distance between the centres, scaled by 0.01
/// for 0x40 triangles, by `1 + avoid cost` for the request's avoid nodes, by
/// the sum of the static avoid nodes of 0x80000000 triangles, by
/// `0.5 + 0.5 * clamp(1 - edge width / 512)`, by the water factor (5 when
/// only one side is water, 4 when both are), by 100 for a bad link and by
/// 100 for a minimal-use door; with `bTryToUseCurvedPath`, also by a turn
/// penalty. The node's own cost is added. Always returns true.
pub fn nav_mesh_search_get_node_connections(
    e: &mut Engine,
    this: Ptr<NavMeshSearch>,
    node: Ptr<AStarNode>,
    connections: Ptr,
) -> bool {
    let data = e.call(NODE_DATA, &args![node]).u32();
    let locals = ConnectionLocals::new(e);
    for edge in 0..3u32 {
        connect_edge(e, this, node, connections, data, edge, &locals);
    }
    locals.free(e);
    true
}

/// One edge of [`nav_mesh_search_get_node_connections`]; returns early
/// where the exe continues with the next edge.
fn connect_edge(
    e: &mut Engine,
    this: Ptr<NavMeshSearch>,
    node: Ptr<AStarNode>,
    connections: Ptr,
    data: u32,
    edge: u32,
    l: &ConnectionLocals,
) {
    let mesh = e.mem.u32(data);
    let triangle = e.mem.u16(data + 4);
    let solution = e.get(this, NavMeshSearch::pHighLevelSolution);
    if !solution.is_null() {
        let record = e.call(GET_TRIANGLE, &args![mesh, triangle]).u32();
        if e.call(TRIANGLE_EDGE_FLAG, &args![record, edge]).bool() {
            let link = e.call(TRIANGLE_EDGE_LINK, &args![record, edge]).u16();
            let entry = e.call(MESH_LINK_ENTRY, &args![mesh, link]).u32();
            let key = e.mem.u32(entry + 4);
            e.mem.set_u32(l.key, key);
            let found = e
                .call(ARRAY_FIND, &args![solution, l.key, 0u32, FIND_PREDICATE])
                .i32();
            if found == -1 {
                return;
            }
        }
    }

    let matched = e
        .call(
            GET_MATCHING_EDGE,
            &args![
                mesh,
                triangle,
                edge,
                l.neighbour_mesh,
                l.neighbour_triangle,
                l.neighbour_edge
            ],
        )
        .bool();
    if !matched {
        return;
    }
    let extra = e
        .call(GET_EDGE_EXTRA_INFO, &args![mesh, triangle, edge])
        .u32();
    if extra != 0 && e.mem.u32(extra) == 1 {
        return;
    }
    let other_mesh = e.mem.u32(l.neighbour_mesh);
    let other_triangle = e.mem.u16(l.neighbour_triangle);
    if mesh == other_mesh && triangle == other_triangle {
        return;
    }

    let request = e.get(this, NavMeshSearch::pCurrentRequest);
    let zero: f64 = e.global(ZERO);
    let center_radius = fn_006a78f0(e, request) as f64;
    if center_radius > zero {
        let near = l.scratch;
        e.call(GET_CENTER, &args![other_mesh, near, other_triangle]);
        let request_center = fn_006a7910(e, request, Ptr::new(l.scratch + 0xc));
        e.call(
            VECTOR_SUBTRACT,
            &args![near, l.scratch + 0x18, request_center],
        );
        let measure = e
            .call(VECTOR_MEASURE_4A7290, &args![l.scratch + 0x18])
            .f64();
        let first = fn_006a78f0(e, request) as f64;
        let second = fn_006a78f0(e, request) as f64;
        if second * first < measure {
            return;
        }
    }

    let mut door_cost = 1.0f32;
    let record = e
        .call(GET_TRIANGLE, &args![other_mesh, other_triangle])
        .u32();
    if e.call(TRIANGLE_HAS_FLAG, &args![record, TRIANGLE_DOOR_FLAG])
        .bool()
    {
        if e.call(REQUEST_CANT_OPEN_DOORS, &args![request]).bool() {
            return;
        }
        let info = e
            .call(GET_DOOR_INFO, &args![other_mesh, other_triangle])
            .u32();
        let door_id = e.mem.u32(info);
        let door = e.call(LOOKUP_FORM, &args![door_id]).u32();
        if door != 0 {
            if fn_006a79a0(e, request) == 0 {
                if e.call(DOOR_TEST_57B460, &args![door]).bool() {
                    return;
                }
                let lock = this.byte_add(TEMP_DOOR_LOCK);
                e.call(DOOR_LOCK_SET_FROM_DOOR, &args![lock, door]);
                let lock_data = e.call(REQUEST_LOCK_DATA, &args![request]).u32();
                let can_unlock = e
                    .call(
                        DOOR_LOCK_CAN_UNLOCK,
                        &args![lock, lock_data, door, l.unlock_flag],
                    )
                    .bool();
                if !can_unlock {
                    return;
                }
            }
            let base = e.call(DOOR_BASE_FORM, &args![door]).u32();
            if e.call(IS_MINIMAL_USE, &args![base]).bool() {
                door_cost = float_global(e, HUNDRED);
            }
        }
    }

    let actor_radius = e.call(REQUEST_ACTOR_RADIUS, &args![request]).f64();
    let setting = e.call(SETTING_VALUE, &args![RADIUS_SETTING]).u32();
    let limit = e.mem.f32(setting) as f64;
    if limit < actor_radius {
        let record = e
            .call(GET_TRIANGLE, &args![other_mesh, other_triangle])
            .u32();
        if e.call(TRIANGLE_HAS_FLAG, &args![record, TRIANGLE_NARROW_FLAG])
            .bool()
        {
            return;
        }
    }

    let mut side_cost = 1.0f32;
    let record = e
        .call(GET_TRIANGLE, &args![other_mesh, other_triangle])
        .u32();
    let other_water = e.call(TRIANGLE_FLAG_200, &args![record]).u8();
    let record = fn_006a78d0(e, Ptr::new(data));
    let own_water = e.call(TRIANGLE_FLAG_200, &args![record]).u8();
    if fn_006a7940(e, request) == 0 && fn_006a7960(e, request) == 0 {
        if other_water != 0 {
            return;
        }
    } else if other_water != own_water {
        side_cost = float_global(e, SIDE_CHANGE_COST);
    } else if other_water != 0 {
        side_cost = float_global(e, SAME_SIDE_COST);
    }

    let mut bad_link_cost = 1.0f32;
    let bad_nodes = e.get(this, NavMeshSearch::pBadNodes);
    if !bad_nodes.is_null() {
        let mut index = 0u32;
        while index < e.call(ARRAY_SIZE, &args![bad_nodes]).u32() {
            let link = fn_006a7af0(e, bad_nodes, index);
            if e.mem.u16(link + 4) == e.get(node, AStarNode::sTriangle) {
                let link = fn_006a7af0(e, bad_nodes, index);
                if e.mem.u32(link) == e.get(node, AStarNode::pNavMesh).addr() {
                    let link = fn_006a7af0(e, bad_nodes, index);
                    if e.mem.u32(link + 8) == edge {
                        bad_link_cost = float_global(e, HUNDRED);
                        break;
                    }
                }
            }
            index += 1;
        }
    }

    e.call(GET_CENTER, &args![other_mesh, l.center, other_triangle]);
    let here = e.call(NODE_DATA, &args![node]).u32();
    let here_center = l.scratch + 0x24;
    fn_006a6eb0(e, Ptr::new(here), Ptr::new(here_center));
    let difference = e
        .call(
            VECTOR_SUBTRACT,
            &args![l.center, l.scratch + 0x30, here_center],
        )
        .u32();
    let mut cost = e.call(FLAT_LENGTH, &args![difference]).f64() as f32;

    let record = e
        .call(GET_TRIANGLE, &args![other_mesh, other_triangle])
        .u32();
    if e.call(TRIANGLE_FLAG_40, &args![record]).bool() {
        let one_hundredth: f64 = e.global(ONE_HUNDREDTH);
        cost = (cost as f64 * one_hundredth) as f32;
    }

    if !request.is_null() && e.call(REQUEST_AVOID_NODES, &args![request]).u32() != 0 {
        let avoid = e.call(REQUEST_AVOID_NODES, &args![request]).u32();
        let avoid_cost = e.call(AVOID_COST_FOR_POINT, &args![avoid, l.center]).f64() as f32;
        if avoid_cost as f64 > zero {
            cost = ((avoid_cost as f64 + 1.0) * cost as f64) as f32;
        }
    }

    let record = e
        .call(GET_TRIANGLE, &args![other_mesh, other_triangle])
        .u32();
    if e.call(TRIANGLE_HAS_FLAG, &args![record, TRIANGLE_AVOID_FLAG])
        .bool()
    {
        let array = Ptr::new(l.avoid_nodes);
        fn_006a7f60(e, array);
        let found = e
            .call(
                GET_STATIC_AVOID_NODES,
                &args![other_mesh, other_triangle, array],
            )
            .bool();
        if found {
            let mut sum = 0.0f32;
            let mut index = 0u32;
            while index < e.call(ARRAY_SIZE, &args![array]).u32() {
                let slot = e.call(ARRAY_ELEMENT_4, &args![array, index]).u32();
                let avoid_node = e.mem.u32(slot);
                let value = e.call(STATIC_AVOID_NODE_COST, &args![avoid_node]).f64();
                sum = (value + sum as f64) as f32;
                index += 1;
            }
            cost = (cost as f64 * sum as f64) as f32;
        }
        fn_006a7f90(e, array);
    }

    e.call(VECTOR_CONSTRUCT, &args![l.vertex_a]);
    e.call(VECTOR_CONSTRUCT, &args![l.vertex_b]);
    let other_edge = e.mem.u32(l.neighbour_edge);
    e.call(
        GET_EDGE_VERTICES,
        &args![
            other_mesh,
            other_triangle,
            other_edge,
            l.vertex_a,
            l.vertex_b
        ],
    );
    let span = e
        .call(
            VECTOR_SUBTRACT,
            &args![l.vertex_a, l.scratch + 0x3c, l.vertex_b],
        )
        .u32();
    let width = e.call(FLAT_LENGTH, &args![span]).f64() as f32;
    let scale = float_global(e, EDGE_WIDTH_SCALE);
    let ratio = (1.0 - width as f64 / scale as f64) as f32;
    e.mem.set_f32(l.ratio, ratio);
    e.call(CLAMP, &args![l.ratio, 0.0f32.to_bits(), 1.0f32.to_bits()]);
    let ratio = e.mem.f32(l.ratio);
    let half: f64 = e.global(HALF);
    cost = ((ratio as f64 * half + half) * cost as f64) as f32;
    cost = (cost as f64 * side_cost as f64) as f32;
    cost = (cost as f64 * bad_link_cost as f64) as f32;
    cost = (cost as f64 * door_cost as f64) as f32;

    if fn_006a7980(e, request) != 0 {
        let goal = fn_006a6eb0(e, this.byte_add(GOAL_NODE), Ptr::new(l.scratch + 0x48));
        let start = fn_006a6eb0(e, this.byte_add(START_NODE), Ptr::new(l.scratch + 0x54));
        let from_goal = l.scratch + 0x60;
        e.call(VECTOR_SUBTRACT, &args![start, from_goal, goal]);
        let goal = fn_006a6eb0(e, this.byte_add(GOAL_NODE), Ptr::new(l.scratch + 0x48));
        let to_goal = l.scratch + 0x6c;
        e.call(VECTOR_SUBTRACT, &args![l.center, to_goal, goal]);
        let start_angle = e.call(GET_Z_ANGLE_FROM_VECTOR, &args![from_goal]).f64() as f32;
        let angle = e.call(GET_Z_ANGLE_FROM_VECTOR, &args![to_goal]).f64() as f32;
        let turn_at = l.scratch + 0x78;
        e.mem
            .set_f32(turn_at, (angle as f64 - start_angle as f64) as f32);
        let minimum = float_global(e, MINUS_PI_FLOAT);
        let maximum = float_global(e, PI_FLOAT);
        e.call(
            BS_WRAP,
            &args![turn_at, minimum.to_bits(), maximum.to_bits()],
        );
        let wrapped = x87_quiet_bits(e.mem.u32(turn_at));
        let turn = e.call(FLOAT_FUNCTION_408840, &args![wrapped]).f64() as f32;
        let start_distance = e.call(VECTOR_LENGTH, &args![from_goal]).f64() as f32;
        let pi: f64 = e.global(PI_DOUBLE);
        let expected = (((pi - turn as f64) * start_distance as f64) / pi) as f32;
        let distance = e.call(VECTOR_LENGTH, &args![to_goal]).f64() as f32;
        let excess = (distance as f64 - expected as f64) as f32;
        let penalty = e
            .call(FLOAT_FUNCTION_408840, &args![excess.to_bits()])
            .f64();
        let penalty_scale: f32 = e.global(TURN_PENALTY_SCALE);
        cost = (penalty * penalty_scale as f64 * cost as f64) as f32;
    }

    let total = (fn_006a7f50(e, node.cast()) as f64 + cost as f64) as f32;
    let key = e
        .call(NODE_KEY, &args![this, other_mesh, other_triangle])
        .u32();
    let status = a_star_search_get_or_create_a_star_node(
        e,
        this.cast(),
        key,
        x87_quiet(total),
        Ptr::new(l.node),
    );
    let neighbour: Ptr<AStarNode> = Ptr::new(e.mem.u32(l.node));
    match status {
        0 => {
            nav_mesh_search_set_node_data(e, this, neighbour, Ptr::new(other_mesh), other_triangle)
        }
        1 => {}
        _ => return,
    }
    let parent_mesh = e.get(node, AStarNode::pNavMesh);
    e.set(neighbour, AStarNode::ParentLink_pNavMesh, parent_mesh);
    let parent_triangle = e.get(node, AStarNode::sTriangle);
    e.set(neighbour, AStarNode::ParentLink_sTriangle, parent_triangle);
    e.set(neighbour, AStarNode::ParentLink_iEdgeIndex, edge);
    e.call(ARRAY_ADD, &args![connections, l.node]);
}

// Translated from 006a78d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The triangle record (`005582f0`) of the navmesh search node at `this`
/// (navmesh at `+0`, triangle at `+4`).
pub fn fn_006a78d0(e: &mut Engine, this: Ptr) -> u32 {
    let mesh = e.mem.u32(this.addr());
    let triangle = e.mem.u16(this.addr() + 4);
    e.call(GET_TRIANGLE, &args![mesh, triangle]).u32()
}

// Translated from 006a78f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `float` at `+0x78` (`PathingRequest::fCenterRadius`, Xbox PDB), in
/// ST0.
pub fn fn_006a78f0(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr() + 0x78))
}

// Translated from 006a7910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the three words at `+0x7C` (`PathingRequest::Center`, Xbox PDB)
/// to `out` and returns `out`.
pub fn fn_006a7910(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    for i in 0..3 {
        let word = e.mem.u32(this.addr() + 0x7c + 4 * i);
        e.mem.set_u32(out.addr() + 4 * i, word);
    }
    out
}

// Translated from 006a7940 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `+0x9D` (`PathingRequest::bCanSwim`, Xbox PDB).
pub fn fn_006a7940(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x9d)
}

// Translated from 006a7960 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `+0x9E` (`PathingRequest::bCanFly`, Xbox PDB).
pub fn fn_006a7960(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x9e)
}

// Translated from 006a7980 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `+0xA2` (`PathingRequest::bTryToUseCurvedPath`, Xbox PDB).
pub fn fn_006a7980(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0xa2)
}

// Translated from 006a79a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `+0xA4` (`PathingRequest::bIgnoreLocks`, Xbox PDB).
pub fn fn_006a79a0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0xa4)
}

// Translated from 006a79c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshSearch::IsGoalNode` (Xbox PDB): with a request whose actor
/// radius is above 100, a node is a goal when the length (`00595c80`) of
/// `goal centre - node centre` is below `r * (r * 0.25)` (`r` from
/// `009b0bd0`) and above the square of that difference's `z`. Otherwise
/// (or when that fails) the node is a goal when it is the goal triangle.
pub fn nav_mesh_search_is_goal_node(
    e: &mut Engine,
    this: Ptr<NavMeshSearch>,
    node: Ptr<AStarNode>,
) -> bool {
    let request = e.get(this, NavMeshSearch::pCurrentRequest);
    if !request.is_null() {
        let radius = e.call(REQUEST_ACTOR_RADIUS, &args![request]).f64();
        let hundred: f64 = e.global(HUNDRED_DOUBLE);
        if radius > hundred {
            let scratch = e.mem.alloc(0x24);
            let data = e.call(NODE_DATA, &args![node]).u32();
            let here = fn_006a6eb0(e, Ptr::new(data), Ptr::new(scratch));
            let goal = fn_006a6eb0(e, this.byte_add(GOAL_NODE), Ptr::new(scratch + 0xc));
            let difference = scratch + 0x18;
            e.call(VECTOR_SUBTRACT, &args![goal, difference, here]);
            let length = e.call(VECTOR_LENGTH_595C80, &args![difference]).f64() as f32;
            let z = e.mem.f32(difference + 8);
            let z_squared = (z as f64 * z as f64) as f32;
            let quarter: f64 = e.global(QUARTER);
            let first = e.call(REQUEST_VALUE_9B0BD0, &args![request]).f64() * quarter;
            let second = e.call(REQUEST_VALUE_9B0BD0, &args![request]).f64();
            e.mem.free(scratch);
            if (length as f64) < second * first && length > z_squared {
                return true;
            }
        }
    }
    let data = e.call(NODE_DATA, &args![node]).u32();
    if e.mem.u32(data) != e.get(this, NavMeshSearch::GoalNode_pNavMesh).addr() {
        return false;
    }
    let data = e.call(NODE_DATA, &args![node]).u32();
    e.mem.u16(data + 4) == e.get(this, NavMeshSearch::GoalNode_sTriangle)
}

// Translated from 006a7ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of element `index` of a word array (`00877a30`).
pub fn fn_006a7ad0(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    e.call(ARRAY_ELEMENT_4, &args![this, index]).u32()
}

// Translated from 006a7af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of element `index` of a 12-byte element array (`006a1440`).
pub fn fn_006a7af0(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    e.call(ARRAY_ELEMENT_12, &args![this, index]).u32()
}

// Translated from 006a7b10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006a7b10 - the exception-unwinding frame (FS:[0]) is not translated
/// The `AStarSearch<NavMeshSearchNode, NavMeshSearchLink>` constructor:
/// the node heap (`0x7800` bytes), the node map, the queue, and no best,
/// last or start node.
pub fn fn_006a7b10(e: &mut Engine, this: Ptr<AStarSearchNavMesh>) -> Ptr<AStarSearchNavMesh> {
    e.mem.set_u32(this.addr(), A_STAR_SEARCH_VTABLE);
    fn_006a8380(e, this.byte_add(0x4));
    e.call(NODE_MAP_CONSTRUCT, &args![this.byte_add(0x14)]);
    e.call(QUEUE_CONSTRUCT, &args![this.byte_add(0x1ff4)]);
    e.set(this, AStarSearchNavMesh::BestNodeSoFar, Ptr::NULL);
    e.set(this, AStarSearchNavMesh::pLastSearchedNode, Ptr::NULL);
    e.set(this, AStarSearchNavMesh::pStartAStarNode, Ptr::NULL);
    e.set(this, AStarSearchNavMesh::bIsRunningInTasklet, 0);
    this
}

// Translated from 006a7bc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006a7bc0 - the exception-unwinding frame (FS:[0]) is not translated
/// The `AStarSearch` destructor: destroys the queue, the node map and the
/// node heap.
pub fn fn_006a7bc0(e: &mut Engine, this: Ptr<AStarSearchNavMesh>) {
    e.mem.set_u32(this.addr(), A_STAR_SEARCH_VTABLE);
    e.call(MEMBER_DESTRUCT_483710, &args![this.byte_add(0x1ff4)]);
    e.call(MEMBER_DESTRUCT_483710, &args![this.byte_add(0x14)]);
    e.call(NODE_HEAP_DESTRUCT, &args![this.byte_add(0x4)]);
}

// Translated from 006a7c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets the search: empties the node map and the queue and forgets the
/// best, last and start nodes.
pub fn fn_006a7c40(e: &mut Engine, this: Ptr<AStarSearchNavMesh>) {
    e.call(NODE_MAP_CLEAR, &args![this.byte_add(0x14)]);
    fn_006a84e0(e, this.byte_add(0x1ff4).cast());
    e.set(this, AStarSearchNavMesh::BestNodeSoFar, Ptr::NULL);
    e.set(this, AStarSearchNavMesh::pLastSearchedNode, Ptr::NULL);
    e.set(this, AStarSearchNavMesh::pStartAStarNode, Ptr::NULL);
}

// Translated from 006a7c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AStarSearch<NavMeshSearchNode, NavMeshSearchLink>::AddStartNode` (Xbox
/// PDB): creates the node for `key` at cost 0 with `heuristic` and a copy
/// of the 0x14-byte node data at `node_data`, queues it and makes it the
/// start node. Returns the new node, the existing node when the lookup
/// answers 1, else 0.
pub fn a_star_search_add_start_node(
    e: &mut Engine,
    this: Ptr<AStarSearchNavMesh>,
    key: u32,
    node_data: Ptr,
    heuristic: f32,
) -> Ptr<AStarNode> {
    let out = e.mem.alloc(4);
    let status = a_star_search_get_or_create_a_star_node(e, this, key, 0.0, Ptr::new(out));
    let node: Ptr<AStarNode> = Ptr::new(e.mem.u32(out));
    e.mem.free(out);
    match status {
        0 => {
            e.set(node, AStarNode::fHeuristic, x87_quiet(heuristic));
            for i in 0..5 {
                let word = e.mem.u32(node_data.addr() + 4 * i);
                e.mem.set_u32(node.addr() + 8 + 4 * i, word);
            }
            a_star_queue_add_node(e, this.byte_add(0x1ff4).cast(), node);
            e.set(this, AStarSearchNavMesh::pStartAStarNode, node);
            node
        }
        1 => node,
        _ => Ptr::NULL,
    }
}

// Translated from 006a7d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the queue's maximum fitness (`0052ce20`).
pub fn fn_006a7d30(e: &mut Engine, this: Ptr<AStarSearchNavMesh>, max_fitness: f32) {
    let value = x87_quiet(max_fitness);
    e.call(
        QUEUE_SET_MAX_FITNESS,
        &args![this.byte_add(0x1ff4), value.to_bits()],
    );
}

// Translated from 006a7d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AStarSearch<NavMeshSearchNode, NavMeshSearchLink>::Search` (Xbox PDB):
/// notes whether it runs in a tasklet (`BSTaskletManager` slot `+0xC` with
/// the current thread id), runs the search loop and builds the path into
/// `nodes` and `links` from the goal found, or, with `allow_partial`, from
/// the best node so far. Returns whether a goal was found.
pub fn a_star_search_search(
    e: &mut Engine,
    this: Ptr<AStarSearchNavMesh>,
    nodes: Ptr,
    links: Ptr,
    allow_partial: bool,
) -> bool {
    let manager = e.call(TASKLET_MANAGER, &[]).u32();
    let thread = e.call(CURRENT_THREAD_ID, &[]).u32();
    let in_tasklet = e.vcall(manager, 0xc, &args![thread]).u8();
    e.set(this, AStarSearchNavMesh::bIsRunningInTasklet, in_tasklet);
    let goal = fn_006a8010(e, this);
    let (found, last) = if !goal.is_null() {
        (true, goal)
    } else if allow_partial {
        (false, e.get(this, AStarSearchNavMesh::BestNodeSoFar))
    } else {
        (false, Ptr::NULL)
    };
    if !last.is_null() {
        fn_006a8290(e, this, last, nodes, links);
    }
    found
}

// Translated from 006a7e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AStarSearch<NavMeshSearchNode, NavMeshSearchLink>::GetOrCreateAStarNode`
/// (Xbox PDB): looks `key` up in its map bucket. A node that exists and
/// costs more than `cost` is taken out of the queue, given `cost` and
/// written to `out` (returns 1); one that does not is left (returns 2). A
/// missing node is allocated with `key` and `cost`, put at the head of the
/// bucket and written to `out` (returns 0).
pub fn a_star_search_get_or_create_a_star_node(
    e: &mut Engine,
    this: Ptr<AStarSearchNavMesh>,
    key: u32,
    cost: f32,
    out: Ptr,
) -> u32 {
    let bucket_index = e
        .call(NODE_MAP_BUCKET, &args![this.byte_add(0x14), key])
        .u32();
    let bucket = this.addr().wrapping_add(bucket_index.wrapping_mul(4)) + 0x14;
    let head = e.mem.u32(bucket);
    if head != 0 {
        let mut candidate = head;
        loop {
            let node: Ptr<AStarNode> = Ptr::new(candidate);
            if e.get(node, AStarNode::iKey) == key {
                let current = e.mem.f32(candidate) as f64;
                if (cost as f64) < current {
                    e.mem.set_u32(out.addr(), candidate);
                    fn_006a8490(e, this.byte_add(0x1ff4).cast(), node);
                    e.set(node, AStarNode::fCost, x87_quiet(cost));
                    return 1;
                }
                return 2;
            }
            candidate = e.get(node, AStarNode::pNextNodeInBucket);
            if candidate == 0 {
                break;
            }
        }
    }
    let node = fn_006a83a0(e, this.byte_add(0x4));
    e.mem.set_u32(out.addr(), node.addr());
    e.set(node, AStarNode::iKey, key);
    e.set(node, AStarNode::fCost, x87_quiet(cost));
    let head = e.mem.u32(bucket);
    e.set(node, AStarNode::pNextNodeInBucket, head);
    e.mem.set_u32(bucket, node.addr());
    0
}

// Translated from 006a7f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `float` at `this` (an A* node's `fCost`), in ST0.
pub fn fn_006a7f50(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr()))
}

// Translated from 006a7f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the `BSSimpleArray` of `NavMeshStaticAvoidNode*`
/// (`006b3eb0` with no size and no reserve).
pub fn fn_006a7f60(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), STATIC_AVOID_ARRAY_VTABLE);
    e.call(ARRAY_INIT, &args![this, 0u32, 0u32]);
    this
}

// Translated from 006a7f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the `BSSimpleArray` of `NavMeshStaticAvoidNode*`:
/// empties it and frees its buffer.
pub fn fn_006a7f90(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), STATIC_AVOID_ARRAY_VTABLE);
    e.call(ARRAY_CLEAR, &args![this, 1u32]);
}

// Translated from 006a7fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `AStarSearch<NavMeshSearchNode, NavMeshSearchLink>` scalar deleting
/// destructor (slot 0 of its vtable `0106b650`): destroys the search and
/// frees it when bit 0 of `flags` is set.
pub fn fn_006a7fb0(
    e: &mut Engine,
    this: Ptr<AStarSearchNavMesh>,
    flags: u32,
) -> Ptr<AStarSearchNavMesh> {
    fn_006a7bc0(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 006a7fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NavMeshStaticAvoidNode *,1024>::_scalar_deleting_destructor_`
/// (Xbox PDB).
pub fn bs_simple_array_nav_mesh_static_avoid_node_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_006a7f90(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 006a8010 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006a8010 - the exception-unwinding frame (FS:[0]) is not translated
/// The search loop: takes the best node out of the queue until it is
/// empty. A goal node (vtable slot `+8`) ends the search and is returned.
/// Otherwise its connections (slot `+4`, into a local `BSScrapArray`) are
/// taken out of the queue, linked to it and queued again, and the best node
/// so far becomes a connection with a smaller heuristic. A connection that
/// is the node's own parent ends the search with 0. In a tasklet, the loop
/// sleeps 0 ms every 50 nodes. When the queue runs dry the last searched
/// node goes back to the one before and the loop returns 0.
pub fn fn_006a8010(e: &mut Engine, this: Ptr<AStarSearchNavMesh>) -> Ptr<AStarNode> {
    let queue: Ptr<AStarQueueNavMesh> = this.byte_add(0x1ff4).cast();
    let connections = Ptr::new(e.mem.alloc(0x14));
    fn_006a8530(e, connections);
    let mut previous = e.get(this, AStarSearchNavMesh::pLastSearchedNode);
    let first = fn_006a8660(e, queue);
    e.set(this, AStarSearchNavMesh::pLastSearchedNode, first);
    let result = 'search: {
        if first.is_null() {
            break 'search Ptr::NULL;
        }
        e.set(this, AStarSearchNavMesh::BestNodeSoFar, first);
        let mut yields = 0u32;
        // The exe reads `pLastSearchedNode` again at each use.
        let last = AStarSearchNavMesh::pLastSearchedNode;
        while !e.get(this, last).is_null() {
            let current = e.get(this, last);
            if e.vcall(this.addr(), 0x8, &args![current]).bool() {
                let current = e.get(this, last);
                e.set(this, AStarSearchNavMesh::BestNodeSoFar, current);
                break 'search e.get(this, last);
            }
            e.call(ARRAY_CLEAR, &args![connections, 0u32]);
            let current = e.get(this, last);
            e.vcall(this.addr(), 0x4, &args![current, connections]);
            let mut index = 0u32;
            while index < e.call(ARRAY_SIZE, &args![connections]).u32() {
                let slot = e.call(ARRAY_ELEMENT_4, &args![connections, index]).u32();
                let connection: Ptr<AStarNode> = Ptr::new(e.mem.u32(slot));
                fn_006a8490(e, queue, connection);
                let current = e.get(this, last);
                if e.get(current, AStarNode::pParentAStarNode) == connection {
                    break 'search Ptr::NULL;
                }
                let current = e.get(this, last);
                e.set(connection, AStarNode::pParentAStarNode, current);
                a_star_queue_add_node(e, queue, connection);
                let candidate = e.call(NODE_HEURISTIC, &args![connection]).f64();
                let best = e.get(this, AStarSearchNavMesh::BestNodeSoFar);
                let best_heuristic = e.call(NODE_HEURISTIC, &args![best]).f64();
                if best_heuristic > candidate {
                    e.set(this, AStarSearchNavMesh::BestNodeSoFar, connection);
                }
                index += 1;
            }
            if e.get(this, AStarSearchNavMesh::bIsRunningInTasklet) != 0 {
                yields += 1;
                if yields == 50 {
                    yields = 0;
                    e.call(SLEEP, &args![0u32]);
                }
            }
            previous = e.get(this, AStarSearchNavMesh::pLastSearchedNode);
            let next = fn_006a8660(e, queue);
            e.set(this, AStarSearchNavMesh::pLastSearchedNode, next);
        }
        e.set(this, AStarSearchNavMesh::pLastSearchedNode, previous);
        Ptr::NULL
    };
    fn_006a8600(e, connections);
    e.mem.free(connections.addr());
    result
}

// Translated from 006a8290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds the path that ends at `last`: counts the nodes up the parent
/// chain, sizes `nodes` (0x14-byte node data) to that count and `links`
/// (12-byte links) to one less, then fills them backwards, each node's
/// data and, except for the first node, its parent link. Returns false for
/// a null `last`.
pub fn fn_006a8290(
    e: &mut Engine,
    _this: Ptr<AStarSearchNavMesh>,
    last: Ptr<AStarNode>,
    nodes: Ptr,
    links: Ptr,
) -> bool {
    if last.is_null() {
        return false;
    }
    let mut count = 0i32;
    let mut node = last;
    while !node.is_null() {
        count += 1;
        node = e.get(node, AStarNode::pParentAStarNode);
    }
    fn_006a8750(e, nodes, count as u32, 1);
    fn_006a8890(e, links, (count - 1) as u32, 1);
    let mut node = last;
    let mut index = count - 1;
    while !node.is_null() {
        let slot = e.call(ARRAY_ELEMENT_20, &args![nodes, index as u32]).u32();
        for i in 0..5 {
            let word = e.mem.u32(node.addr() + 8 + 4 * i);
            e.mem.set_u32(slot + 4 * i, word);
        }
        if index > 0 {
            let slot = e
                .call(ARRAY_ELEMENT_12, &args![links, (index - 1) as u32])
                .u32();
            for i in 0..3 {
                let word = e.mem.u32(node.addr() + 0x1c + 4 * i);
                e.mem.set_u32(slot + 4 * i, word);
            }
        }
        node = e.get(node, AStarNode::pParentAStarNode);
        index -= 1;
    }
    true
}

// Translated from 006a8380 (decompiled, FalloutNV.exe 1.4.0.525)
/// The node heap's constructor: a scrap heap of `0x7800` bytes.
pub fn fn_006a8380(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(SCRAP_HEAP_CONSTRUCT, &args![this, 0x7800u32]);
    this
}

// Translated from 006a83a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Allocates a 0x3C-byte A* node from the node heap and initialises it
/// ([`fn_006a8700`]); 0 when the allocation fails.
pub fn fn_006a83a0(e: &mut Engine, this: Ptr) -> Ptr<AStarNode> {
    let block = e.call(SCRAP_HEAP_ALLOCATE, &args![0x3cu32, this]).u32();
    if block == 0 {
        Ptr::NULL
    } else {
        fn_006a8700(e, Ptr::new(block))
    }
}

// Translated from 006a83f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AStarQueue<NavMeshSearchNode, NavMeshSearchLink>::AddNode` (Xbox PDB):
/// inserts the node into its fitness bucket before the first node whose
/// fitness is not smaller than its own (the list stays sorted by fitness).
pub fn a_star_queue_add_node(e: &mut Engine, this: Ptr<AStarQueueNavMesh>, node: Ptr<AStarNode>) {
    let bucket = e.call(QUEUE_BUCKET, &args![this, node]).u32();
    let mut link = this
        .addr()
        .wrapping_add(bucket.wrapping_mul(4))
        .wrapping_add(QUEUE_FIRST_BUCKET);
    loop {
        let next = e.mem.u32(link);
        if next == 0 {
            break;
        }
        let other = e.call(NODE_FITNESS, &args![next]).f64();
        let own = e.call(NODE_FITNESS, &args![node]).f64();
        if own > other {
            link = next + 0x30;
        } else {
            break;
        }
    }
    let next = e.mem.u32(link);
    if next != 0 {
        e.set(
            Ptr::<AStarNode>::new(next),
            AStarNode::ppPrevNode,
            node.addr() + 0x30,
        );
    }
    e.set(node, AStarNode::pNextNode, next);
    e.set(node, AStarNode::ppPrevNode, link);
    e.mem.set_u32(link, node.addr());
}

// Translated from 006a8490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes the node out of its queue list (`this`, the queue, is not used).
pub fn fn_006a8490(e: &mut Engine, _this: Ptr<AStarQueueNavMesh>, node: Ptr<AStarNode>) {
    let previous = e.get(node, AStarNode::ppPrevNode);
    if previous != 0 {
        let next = e.get(node, AStarNode::pNextNode);
        e.mem.set_u32(previous, next);
    }
    let next = e.get(node, AStarNode::pNextNode);
    if next != 0 {
        let previous = e.get(node, AStarNode::ppPrevNode);
        e.set(Ptr::<AStarNode>::new(next), AStarNode::ppPrevNode, previous);
    }
    e.set(node, AStarNode::ppPrevNode, 0);
    e.set(node, AStarNode::pNextNode, 0);
}

// Translated from 006a84e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties the queue: clears its 20 bucket heads.
pub fn fn_006a84e0(e: &mut Engine, this: Ptr<AStarQueueNavMesh>) {
    e.call(
        MEMSET,
        &args![this.byte_add(QUEUE_FIRST_BUCKET), 0u32, QUEUE_BUCKETS * 4],
    );
}

// Translated from 006a8500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees a `BSSimpleArray`'s buffer through its vtable slot `+8`
/// (`_Deallocate`) and forgets it.
pub fn fn_006a8500(e: &mut Engine, this: Ptr) {
    let buffer = e.mem.u32(this.addr() + 4);
    e.vcall(this.addr(), 0x8, &args![buffer]);
    e.mem.set_u32(this.addr() + 4, 0);
}

// Translated from 006a8530 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006a8530 - the exception-unwinding frame (FS:[0]) is not translated
/// The constructor of the `BSScrapArray` of A* node pointers: the
/// `BSSimpleArray` base, the thread's scrap heap at `+0x10`, then an empty
/// initialisation.
pub fn fn_006a8530(e: &mut Engine, this: Ptr) -> Ptr {
    fn_006a8a00(e, this);
    e.mem.set_u32(this.addr(), NODE_SCRAP_ARRAY_VTABLE);
    let manager = e.call(MEMORY_MANAGER, &[]).u32();
    let heap = e.call(GET_THREAD_SCRAP_HEAP, &args![manager]).u32();
    e.mem.set_u32(this.addr() + 0x10, heap);
    e.call(ARRAY_INIT, &args![this, 0u32, 0u32]);
    this
}

// Translated from 006a85b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the `BSSimpleArray` of A* node pointers: empties it
/// and frees its buffer.
pub fn fn_006a85b0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), NODE_ARRAY_VTABLE);
    e.call(ARRAY_CLEAR, &args![this, 1u32]);
}

// Translated from 006a85d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<AStarNode<NavMeshSearchNode,NavMeshSearchLink> *,1024>::_scalar_deleting_destructor_`
/// (Xbox PDB).
pub fn bs_simple_array_a_star_node_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_006a85b0(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 006a8600 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006a8600 - the exception-unwinding frame (FS:[0]) is not translated
/// The destructor of the `BSScrapArray` of A* node pointers.
pub fn fn_006a8600(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), NODE_SCRAP_ARRAY_VTABLE);
    e.call(ARRAY_CLEAR, &args![this, 1u32]);
    fn_006a85b0(e, this);
}

// Translated from 006a8660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes the first node of the first non-empty bucket out of the queue;
/// 0 when the queue is empty.
pub fn fn_006a8660(e: &mut Engine, this: Ptr<AStarQueueNavMesh>) -> Ptr<AStarNode> {
    for bucket in 0..QUEUE_BUCKETS {
        let link = this.addr() + QUEUE_FIRST_BUCKET + bucket * 4;
        let head = e.mem.u32(link);
        if head == 0 {
            continue;
        }
        let node: Ptr<AStarNode> = Ptr::new(head);
        let next = e.get(node, AStarNode::pNextNode);
        if next == 0 {
            e.mem.set_u32(link, 0);
        } else {
            e.set(Ptr::<AStarNode>::new(next), AStarNode::ppPrevNode, link);
            e.mem.set_u32(link, next);
        }
        e.set(node, AStarNode::ppPrevNode, 0);
        e.set(node, AStarNode::pNextNode, 0);
        return node;
    }
    Ptr::NULL
}

// Translated from 006a8700 (decompiled, FalloutNV.exe 1.4.0.525)
/// The A* node constructor: zero cost and heuristic, no parent, no queue or
/// bucket links, key 0.
pub fn fn_006a8700(e: &mut Engine, this: Ptr<AStarNode>) -> Ptr<AStarNode> {
    e.set(this, AStarNode::fCost, 0.0);
    e.set(this, AStarNode::fHeuristic, 0.0);
    e.set(this, AStarNode::pParentAStarNode, Ptr::NULL);
    e.set(this, AStarNode::ppPrevNode, 0);
    e.set(this, AStarNode::pNextNode, 0);
    e.set(this, AStarNode::iKey, 0);
    e.set(this, AStarNode::pNextNodeInBucket, 0);
    this
}

/// The `SetSize` shared by [`fn_006a8750`] and [`fn_006a8890`]: element
/// size, reallocation and construction callees differ.
fn array_set_size(
    e: &mut Engine,
    this: Ptr,
    size: u32,
    shrink: u8,
    stride: u32,
    reallocate: u32,
    construct: u32,
) {
    let a = this.addr();
    if size == 0 {
        e.call(ARRAY_CLEAR, &args![this, shrink]);
        return;
    }
    let reserved = e.mem.u32(a + 0xc);
    if size > reserved {
        if reserved == 0 {
            let buffer = e.vcall(a, 0x4, &args![size]).u32();
            e.mem.set_u32(a + 4, buffer);
            e.mem.set_u32(a + 0xc, size);
        } else {
            let count = e.mem.u32(a + 8);
            e.call(reallocate, &args![this, size, count]);
            e.mem.set_u32(a + 0xc, size);
        }
        let count = e.mem.u32(a + 8);
        let first = count.wrapping_mul(stride).wrapping_add(e.mem.u32(a + 4));
        e.call(construct, &args![this, first, size.wrapping_sub(count)]);
        e.mem.set_u32(a + 8, size);
    } else if size < e.mem.u32(a + 8) {
        let count = e.mem.u32(a + 8);
        let first = size.wrapping_mul(stride).wrapping_add(e.mem.u32(a + 4));
        e.call(
            DESTROY_ELEMENTS,
            &args![this, first, count.wrapping_sub(size)],
        );
        e.mem.set_u32(a + 8, size);
        if shrink != 0 && size <= e.mem.u32(a + 0xc) >> 2 {
            e.call(reallocate, &args![this, size, size]);
            e.mem.set_u32(a + 0xc, size);
        }
    } else {
        let count = e.mem.u32(a + 8);
        let first = count.wrapping_mul(stride).wrapping_add(e.mem.u32(a + 4));
        e.call(construct, &args![this, first, size.wrapping_sub(count)]);
        e.mem.set_u32(a + 8, size);
    }
}

// Translated from 006a8750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SetSize` of a `BSSimpleArray` of 0x14-byte `NavMeshSearchNode`s: size 0
/// empties it (`shrink` frees the buffer); growing past the reserve
/// allocates (slot `+4`) or reallocates and constructs the new elements;
/// shrinking destroys the dropped elements and, with `shrink`, reallocates
/// to the size when it is at most a quarter of the reserve.
pub fn fn_006a8750(e: &mut Engine, this: Ptr, size: u32, shrink: u8) {
    array_set_size(
        e,
        this,
        size,
        shrink,
        0x14,
        REALLOCATE_NODES,
        CONSTRUCT_NODES,
    );
}

// Translated from 006a8890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SetSize` of a `BSSimpleArray` of 12-byte `NavMeshSearchLink`s, as
/// [`fn_006a8750`].
pub fn fn_006a8890(e: &mut Engine, this: Ptr, size: u32, shrink: u8) {
    array_set_size(
        e,
        this,
        size,
        shrink,
        0xc,
        REALLOCATE_LINKS,
        CONSTRUCT_LINKS,
    );
}

// Translated from 006a89d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapArray<AStarNode<NavMeshSearchNode,NavMeshSearchLink> *,1024>::_scalar_deleting_destructor_`
/// (Xbox PDB).
pub fn bs_scrap_array_a_star_node_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_006a8600(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 006a8a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the `BSSimpleArray` of A* node pointers (`006b3eb0`
/// with no size and no reserve).
pub fn fn_006a8a00(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), NODE_ARRAY_VTABLE);
    e.call(ARRAY_INIT, &args![this, 0u32, 0u32]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x006a6ad0,
            nav_mesh_search_scalar_deleting_destructor(
                Ptr<NavMeshSearch>,
                u32,
            ) -> Ptr<NavMeshSearch>
        ),
        entry!(0x006a6b00, fn_006a6b00(Ptr<NavMeshSearch>)),
        entry!(
            0x006a6b70,
            fn_006a6b70(Ptr<NavMeshSearch>, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(0x006a6eb0, fn_006a6eb0(Ptr, Ptr) -> Ptr),
        entry!(
            0x006a6ef0,
            nav_mesh_search_set_node_data(Ptr<NavMeshSearch>, Ptr<AStarNode>, Ptr, u16)
        ),
        entry!(
            0x006a6fa0,
            nav_mesh_search_get_node_connections(Ptr<NavMeshSearch>, Ptr<AStarNode>, Ptr) -> bool
        ),
        entry!(0x006a78d0, fn_006a78d0(Ptr) -> u32),
        entry!(0x006a78f0, fn_006a78f0(Ptr) -> f32),
        entry!(0x006a7910, fn_006a7910(Ptr, Ptr) -> Ptr),
        entry!(0x006a7940, fn_006a7940(Ptr) -> u8),
        entry!(0x006a7960, fn_006a7960(Ptr) -> u8),
        entry!(0x006a7980, fn_006a7980(Ptr) -> u8),
        entry!(0x006a79a0, fn_006a79a0(Ptr) -> u8),
        entry!(
            0x006a79c0,
            nav_mesh_search_is_goal_node(Ptr<NavMeshSearch>, Ptr<AStarNode>) -> bool
        ),
        entry!(0x006a7ad0, fn_006a7ad0(Ptr, u32) -> u32),
        entry!(0x006a7af0, fn_006a7af0(Ptr, u32) -> u32),
        entry!(
            0x006a7b10,
            fn_006a7b10(Ptr<AStarSearchNavMesh>) -> Ptr<AStarSearchNavMesh>
        ),
        entry!(0x006a7bc0, fn_006a7bc0(Ptr<AStarSearchNavMesh>)),
        entry!(0x006a7c40, fn_006a7c40(Ptr<AStarSearchNavMesh>)),
        entry!(
            0x006a7c90,
            a_star_search_add_start_node(Ptr<AStarSearchNavMesh>, u32, Ptr, f32) -> Ptr<AStarNode>
        ),
        entry!(0x006a7d30, fn_006a7d30(Ptr<AStarSearchNavMesh>, f32)),
        entry!(
            0x006a7d60,
            a_star_search_search(Ptr<AStarSearchNavMesh>, Ptr, Ptr, bool) -> bool
        ),
        entry!(
            0x006a7e00,
            a_star_search_get_or_create_a_star_node(Ptr<AStarSearchNavMesh>, u32, f32, Ptr) -> u32
        ),
        entry!(0x006a7f50, fn_006a7f50(Ptr) -> f32),
        entry!(0x006a7f60, fn_006a7f60(Ptr) -> Ptr),
        entry!(0x006a7f90, fn_006a7f90(Ptr)),
        entry!(
            0x006a7fb0,
            fn_006a7fb0(Ptr<AStarSearchNavMesh>, u32) -> Ptr<AStarSearchNavMesh>
        ),
        entry!(0x006a7fe0, bs_simple_array_nav_mesh_static_avoid_node_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x006a8010,
            fn_006a8010(Ptr<AStarSearchNavMesh>) -> Ptr<AStarNode>
        ),
        entry!(
            0x006a8290,
            fn_006a8290(Ptr<AStarSearchNavMesh>, Ptr<AStarNode>, Ptr, Ptr) -> bool
        ),
        entry!(0x006a8380, fn_006a8380(Ptr) -> Ptr),
        entry!(0x006a83a0, fn_006a83a0(Ptr) -> Ptr<AStarNode>),
        entry!(
            0x006a83f0,
            a_star_queue_add_node(Ptr<AStarQueueNavMesh>, Ptr<AStarNode>)
        ),
        entry!(
            0x006a8490,
            fn_006a8490(Ptr<AStarQueueNavMesh>, Ptr<AStarNode>)
        ),
        entry!(0x006a84e0, fn_006a84e0(Ptr<AStarQueueNavMesh>)),
        entry!(0x006a8500, fn_006a8500(Ptr)),
        entry!(0x006a8530, fn_006a8530(Ptr) -> Ptr),
        entry!(0x006a85b0, fn_006a85b0(Ptr)),
        entry!(0x006a85d0, bs_simple_array_a_star_node_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x006a8600, fn_006a8600(Ptr)),
        entry!(
            0x006a8660,
            fn_006a8660(Ptr<AStarQueueNavMesh>) -> Ptr<AStarNode>
        ),
        entry!(0x006a8700, fn_006a8700(Ptr<AStarNode>) -> Ptr<AStarNode>),
        entry!(0x006a8750, fn_006a8750(Ptr, u32, u8)),
        entry!(0x006a8890, fn_006a8890(Ptr, u32, u8)),
        entry!(0x006a89d0, bs_scrap_array_a_star_node_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x006a8a00, fn_006a8a00(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type CallLog = Vec<(u32, Vec<u32>)>;

    fn eax(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn silent(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    fn calls_to(e: &Engine, address: u32) -> CallLog {
        e.call_log
            .clone()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == address)
            .collect()
    }

    fn vector(e: &Engine, at: u32) -> [f32; 3] {
        [e.mem.f32(at), e.mem.f32(at + 4), e.mem.f32(at + 8)]
    }

    fn set_vector(e: &mut Engine, at: u32, v: [f32; 3]) {
        for (i, c) in v.iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, *c);
        }
    }

    /// The exe's constants this unit reads, with their values in the exe.
    fn constants(e: &mut Engine) {
        for page in [
            0x0101_1000,
            0x0101_2000,
            0x0101_6000,
            0x0101_7000,
            0x0101_e000,
            0x0101_f000,
            0x0102_3000,
            0x0102_9000,
            0x0102_b000,
            0x0106_b000,
            0x0119_c000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(ONE_HUNDREDTH, 0.01f32 as f64);
        e.set_global(HUNDRED_DOUBLE, 100.0f64);
        e.set_global(QUARTER, 0.25f64);
        e.set_global(ZERO, 0.0f64);
        e.set_global(HUNDRED, 100.0f32);
        e.set_global(SIDE_CHANGE_COST, 5.0f32);
        e.set_global(SAME_SIDE_COST, 4.0f32);
        e.set_global(EDGE_WIDTH_SCALE, 512.0f32);
        e.set_global(HALF, 0.5f64);
        e.set_global(PI_FLOAT, std::f32::consts::PI);
        e.set_global(MINUS_PI_FLOAT, -std::f32::consts::PI);
        e.set_global(PI_DOUBLE, std::f32::consts::PI as f64);
        e.set_global(TURN_PENALTY_SCALE, 1.0f32);
    }

    /// Doubles for the vector helpers, the array helpers and the node
    /// accessors, computing what their bodies compute.
    fn common(e: &mut Engine) {
        constants(e);
        e.register(VECTOR_SET, |e, a| {
            for i in 0..3 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            eax(a[0])
        });
        e.register(VECTOR_SUBTRACT, |e, a| {
            let (p, q) = (vector(e, a[0]), vector(e, a[2]));
            set_vector(e, a[1], [p[0] - q[0], p[1] - q[1], p[2] - q[2]]);
            eax(a[1])
        });
        e.register(VECTOR_LENGTH, |e, a| {
            let v = vector(e, a[0]);
            st0(((v[0] * v[0] + v[1] * v[1] + v[2] * v[2]) as f64).sqrt())
        });
        e.register(FLAT_LENGTH, |e, a| {
            let v = vector(e, a[0]);
            st0(((v[0] * v[0] + v[1] * v[1]) as f64).sqrt())
        });
        e.register(NODE_DATA, |_, a| eax(a[0] + 8));
        e.register(ARRAY_SIZE, |e, a| eax(e.mem.u32(a[0] + 8)));
        e.register(ARRAY_ELEMENT_4, |e, a| eax(e.mem.u32(a[0] + 4) + 4 * a[1]));
        e.register(
            ARRAY_ELEMENT_12,
            |e, a| eax(e.mem.u32(a[0] + 4) + 12 * a[1]),
        );
        e.register(
            ARRAY_ELEMENT_20,
            |e, a| eax(e.mem.u32(a[0] + 4) + 20 * a[1]),
        );
        e.register(ARRAY_ADD, |e, a| {
            let size = e.mem.u32(a[0] + 8);
            let item = e.mem.u32(a[1]);
            let buffer = e.mem.u32(a[0] + 4);
            e.mem.set_u32(buffer + 4 * size, item);
            e.mem.set_u32(a[0] + 8, size + 1);
            Ret::default()
        });
        e.register(NODE_FITNESS, |e, a| {
            st0((e.mem.f32(a[0]) + e.mem.f32(a[0] + 4)) as f64)
        });
        e.register(NODE_HEURISTIC, |e, a| st0(e.mem.f32(a[0] + 4) as f64));
        e.register(QUEUE_BUCKET, |_, _| eax(0));
        e.register(NODE_MAP_BUCKET, |_, _| eax(0));
        e.register(SCRAP_HEAP_ALLOCATE, |e, a| eax(e.mem.alloc(a[0])));
        e.register(NODE_KEY, |_, a| eax((a[1] << 4) | a[2]));
        silent(
            e,
            &[
                ARRAY_CLEAR,
                ARRAY_INIT,
                MEMSET,
                NODE_MAP_CLEAR,
                NODE_SET_HEURISTIC,
                MEMORY_MANAGER,
                GET_THREAD_SCRAP_HEAP,
                CONSTRUCT_NODES,
                CONSTRUCT_LINKS,
                FREE,
            ],
        );
    }

    /// A word array with room for `capacity` words.
    fn word_array(e: &mut Engine, capacity: u32) -> Ptr {
        let array = e.mem.alloc(0x14);
        let buffer = e.mem.alloc(4 * capacity);
        e.mem.set_u32(array + 4, buffer);
        Ptr::new(array)
    }

    fn new_node(e: &mut Engine, cost: f32, heuristic: f32) -> Ptr<AStarNode> {
        let node: Ptr<AStarNode> = e.new_object();
        e.set(node, AStarNode::fCost, cost);
        e.set(node, AStarNode::fHeuristic, heuristic);
        node
    }

    fn search(e: &mut Engine) -> Ptr<NavMeshSearch> {
        e.new_object()
    }

    #[test]
    fn scalar_deleting_destructor_destroys_and_frees_on_bit_0() {
        let mut e = Engine::new();
        common(&mut e);
        silent(
            &mut e,
            &[
                DOOR_LOCK_DESTRUCT,
                MEMBER_DESTRUCT_483710,
                NODE_HEAP_DESTRUCT,
            ],
        );
        let this = search(&mut e);
        e.call_log = Some(vec![]);
        let r = e.call(0x006a6ad0, &args![this, 1u32]).u32();
        assert_eq!(r, this.addr());
        assert_eq!(calls_to(&e, FREE), vec![(FREE, vec![this.addr()])]);
        e.call_log = Some(vec![]);
        e.call(0x006a6ad0, &args![this, 2u32]);
        assert!(calls_to(&e, FREE).is_empty());
        assert_eq!(calls_to(&e, DOOR_LOCK_DESTRUCT).len(), 1);
    }

    #[test]
    fn destructor_destroys_the_door_lock_then_the_base() {
        let mut e = Engine::new();
        silent(
            &mut e,
            &[
                DOOR_LOCK_DESTRUCT,
                MEMBER_DESTRUCT_483710,
                NODE_HEAP_DESTRUCT,
            ],
        );
        let this = search(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x006a6b00, &args![this]);
        let a = this.addr();
        let log = e.call_log.clone().unwrap();
        assert_eq!(
            log[1..],
            [
                (DOOR_LOCK_DESTRUCT, vec![a + 0x208c]),
                (MEMBER_DESTRUCT_483710, vec![a + 0x1ff4]),
                (MEMBER_DESTRUCT_483710, vec![a + 0x14]),
                (NODE_HEAP_DESTRUCT, vec![a + 4]),
            ]
        );
        assert_eq!(e.mem.u32(a), A_STAR_SEARCH_VTABLE);
    }

    /// Doubles for a search run whose search loop finds no goal.
    fn search_run(e: &mut Engine, this: Ptr<NavMeshSearch>) {
        common(e);
        e.register(GET_CENTER, |e, a| {
            let v = if a[2] == 2 { [3.0, 4.0, 0.0] } else { [0.0; 3] };
            set_vector(e, a[1], v);
            eax(a[1])
        });
        silent(
            e,
            &[
                QUEUE_SET_MAX_FITNESS,
                CURRENT_THREAD_ID,
                LOCAL_ARRAY_CONSTRUCT,
                LOCAL_POINTER_CONSTRUCT,
                LOCAL_POINTER_ASSIGN,
                LOCAL_POINTER_DESTRUCT,
                LOCAL_ARRAY_DESTRUCT,
            ],
        );
        // The tasklet manager answers "not in a tasklet"; the search's own
        // vtable answers "not a goal" and adds no connections.
        e.register(0x0000_1110, |_, _| eax(0));
        e.put_vtable(0x0000_2000, &[0, 0, 0, 0x0000_1110]);
        let manager = e.mem.alloc(4);
        e.mem.set_u32(manager, 0x0000_2000);
        e.register_double(TASKLET_MANAGER, move |_, _| eax(manager));
        e.register(0x0000_1120, |_, _| eax(0));
        e.put_vtable(0x0000_3000, &[0, 0x0000_1120, 0x0000_1120]);
        e.mem.set_u32(this.addr(), 0x0000_3000);
    }

    #[test]
    fn search_start_fills_the_start_and_goal_and_returns_the_partial_path() {
        let mut e = Engine::new();
        let this = search(&mut e);
        search_run(&mut e, this);
        e.register(INFO_TEST_69AD00, |_, a| eax((a[0] == 0x7700) as u32));
        // The node array allocates through its vtable slot +4.
        let buffer = e.mem.alloc(0x28);
        e.register_double(0x0000_1130, move |_, _| eax(buffer));
        e.put_vtable(0x0000_4000, &[0, 0x0000_1130]);
        let nodes = e.mem.alloc(0x10);
        e.mem.set_u32(nodes, 0x0000_4000);
        let links = e.mem.alloc(0x10);
        let start = e.mem.alloc(8);
        e.mem.set_u32(start, 0xa000);
        e.mem.set_u16(start + 4, 1);
        let goal = e.mem.alloc(8);
        e.mem.set_u32(goal, 0xb000);
        e.mem.set_u16(goal + 4, 2);
        let solution = word_array(&mut e, 2);
        let solution_buffer = e.mem.u32(solution.addr() + 4);
        e.mem.set_u32(solution_buffer, 0x7700);
        e.mem.set_u32(solution_buffer + 4, 0x7800);
        e.mem.set_u32(solution.addr() + 8, 2);
        e.call_log = Some(vec![]);
        let found = e
            .call(
                0x006a6b70,
                &args![this, 0x5000u32, start, goal, nodes, links, solution],
            )
            .bool();
        assert!(!found);
        let a = this.addr();
        assert_eq!(
            e.get(this, NavMeshSearch::pCurrentRequest),
            Ptr::new(0x5000)
        );
        assert_eq!(e.mem.u32(a + 0x2070), 0xb000);
        assert_eq!(vector(&e, a + 0x2078), [3.0, 4.0, 0.0]);
        assert_eq!(e.get(this, NavMeshSearch::pHighLevelSolution), solution);
        // Twice the start-goal distance as the maximum fitness.
        assert_eq!(
            calls_to(&e, QUEUE_SET_MAX_FITNESS),
            vec![(QUEUE_SET_MAX_FITNESS, vec![a + 0x1ff4, 10.0f32.to_bits()])]
        );
        // One accepted NavMeshInfo out of two.
        assert_eq!(calls_to(&e, LOCAL_POINTER_ASSIGN).len(), 1);
        // The start node (heuristic 5) is the best node so far: the path is
        // that one node.
        let base: Ptr<AStarSearchNavMesh> = this.cast();
        let start_node = e.get(base, AStarSearchNavMesh::pStartAStarNode);
        assert_eq!(e.get(start_node, AStarNode::fHeuristic), 5.0);
        assert_eq!(e.mem.u32(buffer), 0xa000);
        assert_eq!(e.mem.u16(buffer + 4), 1);
        assert_eq!(e.mem.u32(nodes + 8), 1);
        assert_eq!(
            calls_to(&e, ARRAY_CLEAR)[..2],
            [(ARRAY_CLEAR, vec![nodes, 0]), (ARRAY_CLEAR, vec![links, 0])]
        );
    }

    #[test]
    fn node_center_passes_the_three_floats_quieted() {
        let mut e = Engine::new();
        silent(&mut e, &[VECTOR_SET]);
        let node = e.mem.alloc(0x14);
        e.mem.set_f32(node + 8, 1.5);
        e.mem.set_u32(node + 0xc, 0x7f80_0001);
        e.mem.set_f32(node + 0x10, -2.0);
        e.call_log = Some(vec![]);
        let r = e.call(0x006a6eb0, &args![node, 0x9000u32]).u32();
        assert_eq!(r, 0x9000);
        assert_eq!(
            calls_to(&e, VECTOR_SET),
            vec![(
                VECTOR_SET,
                vec![0x9000, 1.5f32.to_bits(), 0x7fc0_0001, (-2.0f32).to_bits()]
            )]
        );
    }

    #[test]
    fn set_node_data_stores_the_triangle_and_a_hundredth_of_the_goal_distance() {
        let mut e = Engine::new();
        common(&mut e);
        e.register(GET_CENTER, |e, a| {
            set_vector(e, a[1], [0.0; 3]);
            // A signalling NaN comes out of FLD/FSTP quiet.
            e.mem.set_u32(a[1] + 8, 0x7f80_0002);
            eax(a[1])
        });
        let this = search(&mut e);
        set_vector(&mut e, this.addr() + 0x2078, [30.0, 40.0, 0.0]);
        let node = new_node(&mut e, 0.0, 0.0);
        e.call_log = Some(vec![]);
        e.call(0x006a6ef0, &args![this, node, 0xa000u32, 7u16]);
        assert_eq!(e.get(node, AStarNode::pNavMesh), Ptr::new(0xa000));
        assert_eq!(e.get(node, AStarNode::sTriangle), 7);
        assert_eq!(e.mem.u32(node.addr() + 0x18), 0x7fc0_0002);
        assert_eq!(
            calls_to(&e, GET_CENTER)[0].1,
            vec![0xa000, calls_to(&e, GET_CENTER)[0].1[1], 7]
        );

        // A centre at the origin: 0.01 of the distance 50 to the goal.
        e.register(GET_CENTER, |e, a| {
            set_vector(e, a[1], [0.0; 3]);
            eax(a[1])
        });
        e.call_log = Some(vec![]);
        e.call(0x006a6ef0, &args![this, node, 0xa000u32, 7u16]);
        let expected = (50.0f64 * (0.01f32 as f64)) as f32;
        assert_eq!(
            calls_to(&e, NODE_SET_HEURISTIC),
            vec![(NODE_SET_HEURISTIC, vec![node.addr(), expected.to_bits()])]
        );
    }

    /// The world of `GetNodeConnections`: the node is navmesh 0xa000
    /// triangle 1 at the origin; its edge 0 leads to navmesh 0xb000
    /// triangle 2 at (3, 4, 0) through a 256-wide edge. `flags` are the
    /// neighbour record's flags (record 0x6002).
    fn connections_world(e: &mut Engine, flags: u32) {
        common(e);
        e.register(GET_CENTER, |e, a| {
            let v = if a[2] == 2 { [3.0, 4.0, 0.0] } else { [0.0; 3] };
            set_vector(e, a[1], v);
            eax(a[1])
        });
        e.register(GET_MATCHING_EDGE, |e, a| {
            if a[2] != 0 {
                return eax(0);
            }
            e.mem.set_u32(a[3], 0xb000);
            e.mem.set_u16(a[4], 2);
            e.mem.set_u32(a[5], 1);
            eax(1)
        });
        e.register(GET_EDGE_EXTRA_INFO, |_, _| eax(0));
        e.register(GET_TRIANGLE, |_, a| eax(0x6000 + a[1]));
        e.register_double(TRIANGLE_HAS_FLAG, move |_, a| {
            eax((a[0] == 0x6002 && flags & a[1] != 0) as u32)
        });
        e.register_double(TRIANGLE_FLAG_200, move |_, a| {
            eax((a[0] == 0x6002 && flags & 0x200 != 0) as u32)
        });
        e.register(TRIANGLE_FLAG_40, |_, _| eax(0));
        e.register(REQUEST_CANT_OPEN_DOORS, |_, _| eax(1));
        e.register(REQUEST_ACTOR_RADIUS, |_, _| st0(1.0));
        let limit = e.mem.alloc(4);
        e.mem.set_f32(limit, 10.0);
        e.register_double(SETTING_VALUE, move |_, _| eax(limit));
        e.register(REQUEST_AVOID_NODES, |_, _| eax(0));
        silent(e, &[VECTOR_CONSTRUCT, CLAMP]);
        e.register(GET_EDGE_VERTICES, |e, a| {
            set_vector(e, a[3], [0.0; 3]);
            set_vector(e, a[4], [256.0, 0.0, 0.0]);
            eax(1)
        });
    }

    /// Runs `GetNodeConnections` in [`connections_world`]; returns the list.
    fn run_connections(e: &mut Engine, swim: bool) -> Ptr {
        let this = search(e);
        let request = e.mem.alloc(0xb0);
        e.mem.set_u8(request + 0x9d, swim as u8);
        e.set(this, NavMeshSearch::pCurrentRequest, Ptr::new(request));
        let node = new_node(e, 2.0, 0.0);
        e.set(node, AStarNode::pNavMesh, Ptr::new(0xa000));
        e.set(node, AStarNode::sTriangle, 1);
        let list = word_array(e, 4);
        e.call_log = Some(vec![]);
        assert!(e.call(0x006a6fa0, &args![this, node, list]).bool());
        list
    }

    fn first_added(e: &Engine, list: Ptr) -> Ptr<AStarNode> {
        Ptr::new(e.mem.u32(e.mem.u32(list.addr() + 4)))
    }

    #[test]
    fn node_connections_add_the_neighbour_with_its_cost() {
        let mut e = Engine::new();
        connections_world(&mut e, 0);
        let list = run_connections(&mut e, false);
        assert_eq!(e.mem.u32(list.addr() + 8), 1);
        let added = first_added(&e, list);
        // Distance 5, width factor 0.5 + 0.5 * (1 - 256/512), plus the
        // node's own cost 2.
        assert_eq!(e.get(added, AStarNode::fCost), 5.75);
        assert_eq!(e.get(added, AStarNode::pNavMesh), Ptr::new(0xb000));
        assert_eq!(e.get(added, AStarNode::sTriangle), 2);
        assert_eq!(
            e.get(added, AStarNode::ParentLink_pNavMesh),
            Ptr::new(0xa000)
        );
        assert_eq!(e.get(added, AStarNode::ParentLink_sTriangle), 1);
        assert_eq!(e.get(added, AStarNode::ParentLink_iEdgeIndex), 0);
        assert_eq!(calls_to(&e, GET_MATCHING_EDGE).len(), 3);
        // The width comes from the neighbour's edge (1).
        assert_eq!(calls_to(&e, GET_EDGE_VERTICES)[0].1[..3], [0xb000, 2, 1]);
    }

    #[test]
    fn node_connections_skip_doors_the_request_cannot_open() {
        let mut e = Engine::new();
        connections_world(&mut e, TRIANGLE_DOOR_FLAG);
        let list = run_connections(&mut e, false);
        assert_eq!(e.mem.u32(list.addr() + 8), 0);
    }

    #[test]
    fn node_connections_skip_water_unless_the_actor_swims() {
        let mut e = Engine::new();
        connections_world(&mut e, 0x200);
        let list = run_connections(&mut e, false);
        assert_eq!(e.mem.u32(list.addr() + 8), 0);
        let mut e = Engine::new();
        connections_world(&mut e, 0x200);
        let list = run_connections(&mut e, true);
        let added = first_added(&e, list);
        // Land to water costs 5 times as much.
        assert_eq!(e.get(added, AStarNode::fCost), 2.0 + 3.75 * 5.0);
    }

    #[test]
    fn node_triangle_record_reads_mesh_and_triangle() {
        let mut e = Engine::new();
        e.register(GET_TRIANGLE, |_, a| eax(a[0] + a[1]));
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, 0xa000);
        e.mem.set_u16(node + 4, 3);
        assert_eq!(e.call(0x006a78d0, &args![node]).u32(), 0xa003);
    }

    #[test]
    fn center_radius_is_the_float_at_0x78() {
        let mut e = Engine::new();
        let request = e.mem.alloc(0xb0);
        e.mem.set_f32(request + 0x78, 2.5);
        assert_eq!(e.call(0x006a78f0, &args![request]).f32(), 2.5);
        e.mem.set_u32(request + 0x78, 0x7f80_0001);
        assert_eq!(
            fn_006a78f0(&mut e, Ptr::new(request)).to_bits(),
            0x7fc0_0001
        );
    }

    #[test]
    fn request_center_is_copied_out() {
        let mut e = Engine::new();
        let request = e.mem.alloc(0xb0);
        for i in 0..3 {
            e.mem.set_u32(request + 0x7c + 4 * i, 10 + i);
        }
        let out = e.mem.alloc(0xc);
        assert_eq!(e.call(0x006a7910, &args![request, out]).u32(), out);
        assert_eq!(
            [e.mem.u32(out), e.mem.u32(out + 4), e.mem.u32(out + 8)],
            [10, 11, 12]
        );
    }

    fn byte_getter(address: u32, offset: u32) {
        let mut e = Engine::new();
        let request = e.mem.alloc(0xb0);
        assert_eq!(e.call(address, &args![request]).u8(), 0);
        e.mem.set_u8(request + offset, 2);
        assert_eq!(e.call(address, &args![request]).u8(), 2);
    }

    #[test]
    fn can_swim_is_the_byte_at_0x9d() {
        byte_getter(0x006a7940, 0x9d);
    }

    #[test]
    fn can_fly_is_the_byte_at_0x9e() {
        byte_getter(0x006a7960, 0x9e);
    }

    #[test]
    fn curved_path_is_the_byte_at_0xa2() {
        byte_getter(0x006a7980, 0xa2);
    }

    #[test]
    fn ignore_locks_is_the_byte_at_0xa4() {
        byte_getter(0x006a79a0, 0xa4);
    }

    #[test]
    fn goal_node_is_the_goal_triangle_or_close_enough_for_a_large_actor() {
        let mut e = Engine::new();
        common(&mut e);
        let this = search(&mut e);
        e.set(this, NavMeshSearch::GoalNode_pNavMesh, Ptr::new(0xb000));
        e.set(this, NavMeshSearch::GoalNode_sTriangle, 2);
        let node = new_node(&mut e, 0.0, 0.0);
        e.set(node, AStarNode::pNavMesh, Ptr::new(0xb000));
        e.set(node, AStarNode::sTriangle, 2);
        assert!(e.call(0x006a79c0, &args![this, node]).bool());
        e.set(node, AStarNode::sTriangle, 3);
        assert!(!e.call(0x006a79c0, &args![this, node]).bool());

        // A request with an actor radius above 100: within r * (r * 0.25)
        // of the goal and above the vertical difference squared.
        let request = e.mem.alloc(0xb0);
        e.set(this, NavMeshSearch::pCurrentRequest, Ptr::new(request));
        e.register(REQUEST_ACTOR_RADIUS, |_, _| st0(150.0));
        e.register(REQUEST_VALUE_9B0BD0, |_, _| st0(4.0));
        e.register(VECTOR_LENGTH_595C80, |e, a| {
            let v = vector(e, a[0]);
            st0(((v[0] * v[0] + v[1] * v[1] + v[2] * v[2]) as f64).sqrt())
        });
        set_vector(&mut e, this.addr() + 0x2078, [3.0, 0.0, 0.0]);
        set_vector(&mut e, node.addr() + 0x10, [0.0, 0.0, 0.0]);
        // Length 3 < 4 * (4 * 0.25) and 3 > 0.
        assert!(e.call(0x006a79c0, &args![this, node]).bool());
        set_vector(&mut e, this.addr() + 0x2078, [5.0, 0.0, 0.0]);
        assert!(!e.call(0x006a79c0, &args![this, node]).bool());
        // An actor radius of exactly 100 does not qualify.
        set_vector(&mut e, this.addr() + 0x2078, [3.0, 0.0, 0.0]);
        e.register(REQUEST_ACTOR_RADIUS, |_, _| st0(100.0));
        assert!(!e.call(0x006a79c0, &args![this, node]).bool());
    }

    #[test]
    fn word_element_forwards_to_00877a30() {
        let mut e = Engine::new();
        e.register(ARRAY_ELEMENT_4, |_, a| eax(a[0] + 4 * a[1]));
        assert_eq!(e.call(0x006a7ad0, &args![0x1000u32, 3u32]).u32(), 0x100c);
    }

    #[test]
    fn link_element_forwards_to_006a1440() {
        let mut e = Engine::new();
        e.register(ARRAY_ELEMENT_12, |_, a| eax(a[0] + 12 * a[1]));
        assert_eq!(e.call(0x006a7af0, &args![0x1000u32, 2u32]).u32(), 0x1018);
    }

    #[test]
    fn a_star_constructor_builds_the_members() {
        let mut e = Engine::new();
        silent(
            &mut e,
            &[SCRAP_HEAP_CONSTRUCT, NODE_MAP_CONSTRUCT, QUEUE_CONSTRUCT],
        );
        let this: Ptr<AStarSearchNavMesh> = e.new_object();
        let a = this.addr();
        e.mem.set_u32(a + 0x2048, 9);
        e.mem.set_u8(a + 0x2054, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006a7b10, &args![this]).u32(), a);
        let log = e.call_log.clone().unwrap();
        assert_eq!(
            log[1..],
            [
                (SCRAP_HEAP_CONSTRUCT, vec![a + 4, 0x7800]),
                (NODE_MAP_CONSTRUCT, vec![a + 0x14]),
                (QUEUE_CONSTRUCT, vec![a + 0x1ff4]),
            ]
        );
        assert_eq!(e.mem.u32(a), A_STAR_SEARCH_VTABLE);
        assert_eq!(e.mem.u32(a + 0x2048), 0);
        assert_eq!(e.mem.u8(a + 0x2054), 0);
    }

    #[test]
    fn a_star_destructor_destroys_queue_map_and_heap() {
        let mut e = Engine::new();
        silent(&mut e, &[MEMBER_DESTRUCT_483710, NODE_HEAP_DESTRUCT]);
        let this: Ptr<AStarSearchNavMesh> = e.new_object();
        let a = this.addr();
        e.call_log = Some(vec![]);
        e.call(0x006a7bc0, &args![this]);
        let log = e.call_log.clone().unwrap();
        assert_eq!(
            log[1..],
            [
                (MEMBER_DESTRUCT_483710, vec![a + 0x1ff4]),
                (MEMBER_DESTRUCT_483710, vec![a + 0x14]),
                (NODE_HEAP_DESTRUCT, vec![a + 4]),
            ]
        );
        assert_eq!(e.mem.u32(a), A_STAR_SEARCH_VTABLE);
    }

    #[test]
    fn reset_empties_map_and_queue() {
        let mut e = Engine::new();
        silent(&mut e, &[NODE_MAP_CLEAR, MEMSET]);
        let this: Ptr<AStarSearchNavMesh> = e.new_object();
        let a = this.addr();
        for offset in [0x2048, 0x204c, 0x2050] {
            e.mem.set_u32(a + offset, 5);
        }
        e.call_log = Some(vec![]);
        e.call(0x006a7c40, &args![this]);
        assert_eq!(
            calls_to(&e, NODE_MAP_CLEAR),
            vec![(NODE_MAP_CLEAR, vec![a + 0x14])]
        );
        assert_eq!(
            calls_to(&e, MEMSET),
            vec![(MEMSET, vec![a + 0x1ff8, 0, 0x50])]
        );
        for offset in [0x2048, 0x204c, 0x2050] {
            assert_eq!(e.mem.u32(a + offset), 0);
        }
    }

    #[test]
    fn add_start_node_creates_queues_and_remembers_it() {
        let mut e = Engine::new();
        common(&mut e);
        let this: Ptr<AStarSearchNavMesh> = e.new_object();
        let data = e.mem.alloc(0x14);
        for i in 0..5 {
            e.mem.set_u32(data + 4 * i, 0x100 + i);
        }
        let node = e
            .call(0x006a7c90, &args![this, 0x77u32, data, 3.0f32])
            .ptr::<AStarNode>();
        assert_eq!(e.get(node, AStarNode::fHeuristic), 3.0);
        assert_eq!(e.get(node, AStarNode::fCost), 0.0);
        assert_eq!(e.get(node, AStarNode::iKey), 0x77);
        assert_eq!(e.mem.u32(node.addr() + 0x18), 0x104);
        assert_eq!(e.get(this, AStarSearchNavMesh::pStartAStarNode), node);
        // Queued in bucket 0.
        assert_eq!(e.mem.u32(this.addr() + 0x1ff8), node.addr());
        // The same key again: the node exists and 0 is not cheaper.
        let again = e
            .call(0x006a7c90, &args![this, 0x77u32, data, 3.0f32])
            .u32();
        assert_eq!(again, 0);
        // A cheaper start (the lookup answers 1) returns the existing node.
        e.set(node, AStarNode::fCost, 1.0);
        let again = e
            .call(0x006a7c90, &args![this, 0x77u32, data, 3.0f32])
            .u32();
        assert_eq!(again, node.addr());
    }

    #[test]
    fn max_fitness_goes_to_the_queue() {
        let mut e = Engine::new();
        silent(&mut e, &[QUEUE_SET_MAX_FITNESS]);
        let this: Ptr<AStarSearchNavMesh> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x006a7d30, &args![this, 0x7f80_0001u32]);
        assert_eq!(
            calls_to(&e, QUEUE_SET_MAX_FITNESS),
            vec![(
                QUEUE_SET_MAX_FITNESS,
                vec![this.addr() + 0x1ff4, 0x7fc0_0001]
            )]
        );
    }

    #[test]
    fn search_returns_the_goal_and_builds_its_path() {
        let mut e = Engine::new();
        let this = search(&mut e);
        search_run(&mut e, this);
        // Slot +8 answers "goal" now.
        e.register(0x0000_1120, |_, _| eax(1));
        let base: Ptr<AStarSearchNavMesh> = this.cast();
        let node = new_node(&mut e, 0.0, 1.0);
        e.set(node, AStarNode::pNavMesh, Ptr::new(0xa000));
        a_star_queue_add_node(&mut e, base.byte_add(0x1ff4).cast(), node);
        let buffer = e.mem.alloc(0x28);
        e.register_double(0x0000_1130, move |_, _| eax(buffer));
        e.put_vtable(0x0000_4000, &[0, 0x0000_1130]);
        let nodes = e.mem.alloc(0x10);
        e.mem.set_u32(nodes, 0x0000_4000);
        let links = e.mem.alloc(0x10);
        assert!(e.call(0x006a7d60, &args![base, nodes, links, false]).bool());
        assert_eq!(e.mem.u32(buffer), 0xa000);
        assert_eq!(e.get(base, AStarSearchNavMesh::BestNodeSoFar), node);

        // Nothing queued and no partial path: nothing is built.
        e.call_log = Some(vec![]);
        assert!(!e.call(0x006a7d60, &args![base, nodes, links, false]).bool());
        assert!(calls_to(&e, ARRAY_ELEMENT_20).is_empty());
    }

    #[test]
    fn get_or_create_node_creates_updates_or_leaves() {
        let mut e = Engine::new();
        common(&mut e);
        let this: Ptr<AStarSearchNavMesh> = e.new_object();
        let out = e.mem.alloc(4);
        let first = e.call(0x006a7e00, &args![this, 1u32, 5.0f32, out]).u32();
        assert_eq!(first, 0);
        let one: Ptr<AStarNode> = Ptr::new(e.mem.u32(out));
        let second = e.call(0x006a7e00, &args![this, 2u32, 6.0f32, out]).u32();
        assert_eq!(second, 0);
        let two: Ptr<AStarNode> = Ptr::new(e.mem.u32(out));
        // Bucket 0 now holds two, then one.
        assert_eq!(e.mem.u32(this.addr() + 0x14), two.addr());
        assert_eq!(e.get(two, AStarNode::pNextNodeInBucket), one.addr());
        // Key 1 again, more expensive: left alone.
        e.mem.set_u32(out, 0);
        assert_eq!(e.call(0x006a7e00, &args![this, 1u32, 7.0f32, out]).u32(), 2);
        assert_eq!(e.mem.u32(out), 0);
        // Key 1 cheaper (found down the chain): updated and unqueued.
        a_star_queue_add_node(&mut e, this.byte_add(0x1ff4).cast(), one);
        assert_eq!(e.call(0x006a7e00, &args![this, 1u32, 4.0f32, out]).u32(), 1);
        assert_eq!(e.mem.u32(out), one.addr());
        assert_eq!(e.get(one, AStarNode::fCost), 4.0);
        assert_eq!(e.mem.u32(this.addr() + 0x1ff8), 0);
    }

    #[test]
    fn cost_is_the_first_float() {
        let mut e = Engine::new();
        let node = e.mem.alloc(4);
        e.mem.set_f32(node, 1.25);
        assert_eq!(e.call(0x006a7f50, &args![node]).f32(), 1.25);
    }

    #[test]
    fn static_avoid_array_constructor_initialises_empty() {
        let mut e = Engine::new();
        silent(&mut e, &[ARRAY_INIT]);
        let array = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006a7f60, &args![array]).u32(), array);
        assert_eq!(e.mem.u32(array), STATIC_AVOID_ARRAY_VTABLE);
        assert_eq!(
            calls_to(&e, ARRAY_INIT),
            vec![(ARRAY_INIT, vec![array, 0, 0])]
        );
    }

    #[test]
    fn static_avoid_array_destructor_frees_the_buffer() {
        let mut e = Engine::new();
        silent(&mut e, &[ARRAY_CLEAR]);
        let array = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x006a7f90, &args![array]);
        assert_eq!(e.mem.u32(array), STATIC_AVOID_ARRAY_VTABLE);
        assert_eq!(
            calls_to(&e, ARRAY_CLEAR),
            vec![(ARRAY_CLEAR, vec![array, 1])]
        );
    }

    #[test]
    fn a_star_scalar_deleting_destructor() {
        let mut e = Engine::new();
        silent(&mut e, &[MEMBER_DESTRUCT_483710, NODE_HEAP_DESTRUCT, FREE]);
        let this: Ptr<AStarSearchNavMesh> = e.new_object();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006a7fb0, &args![this, 1u32]).u32(), this.addr());
        assert_eq!(calls_to(&e, FREE), vec![(FREE, vec![this.addr()])]);
        assert_eq!(calls_to(&e, NODE_HEAP_DESTRUCT).len(), 1);
        e.call_log = Some(vec![]);
        e.call(0x006a7fb0, &args![this, 0u32]);
        assert!(calls_to(&e, FREE).is_empty());
    }

    #[test]
    fn static_avoid_array_scalar_deleting_destructor() {
        let mut e = Engine::new();
        silent(&mut e, &[ARRAY_CLEAR, FREE]);
        let array = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x006a7fe0, &args![array, 0u32]);
        assert!(calls_to(&e, FREE).is_empty());
        e.call(0x006a7fe0, &args![array, 1u32]);
        assert_eq!(calls_to(&e, FREE), vec![(FREE, vec![array])]);
    }

    #[test]
    fn search_loop_expands_until_a_goal() {
        let mut e = Engine::new();
        common(&mut e);
        let this: Ptr<AStarSearchNavMesh> = e.new_object();
        let start = new_node(&mut e, 0.0, 9.0);
        let next = new_node(&mut e, 1.0, 2.0);
        a_star_queue_add_node(&mut e, this.byte_add(0x1ff4).cast(), start);
        // Slot +4 connects `start` to `next`; slot +8 says `next` is a goal.
        e.register_double(0x0000_1140, move |e, a| {
            if a[1] == start.addr() {
                let buffer = e.mem.alloc(4);
                e.mem.set_u32(buffer, next.addr());
                e.mem.set_u32(a[2] + 4, buffer);
                e.mem.set_u32(a[2] + 8, 1);
            }
            eax(1)
        });
        e.register_double(0x0000_1150, move |_, a| eax((a[1] == next.addr()) as u32));
        e.put_vtable(0x0000_5000, &[0, 0x0000_1140, 0x0000_1150]);
        e.mem.set_u32(this.addr(), 0x0000_5000);
        let found = e.call(0x006a8010, &args![this]).u32();
        assert_eq!(found, next.addr());
        assert_eq!(e.get(next, AStarNode::pParentAStarNode), start);
        assert_eq!(e.get(this, AStarSearchNavMesh::BestNodeSoFar), next);

        // An empty queue: nothing found.
        e.set(this, AStarSearchNavMesh::pLastSearchedNode, start);
        assert_eq!(e.call(0x006a8010, &args![this]).u32(), 0);
        assert_eq!(
            e.get(this, AStarSearchNavMesh::pLastSearchedNode),
            Ptr::NULL
        );

        // A connection that is the node's own parent ends the search.
        e.set(start, AStarNode::pParentAStarNode, next);
        a_star_queue_add_node(&mut e, this.byte_add(0x1ff4).cast(), start);
        assert_eq!(e.call(0x006a8010, &args![this]).u32(), 0);
    }

    #[test]
    fn path_is_filled_backwards_from_the_last_node() {
        let mut e = Engine::new();
        common(&mut e);
        let this: Ptr<AStarSearchNavMesh> = e.new_object();
        let first = new_node(&mut e, 0.0, 0.0);
        let second = new_node(&mut e, 0.0, 0.0);
        e.set(first, AStarNode::pNavMesh, Ptr::new(0xa1));
        e.set(second, AStarNode::pNavMesh, Ptr::new(0xa2));
        e.set(second, AStarNode::ParentLink_pNavMesh, Ptr::new(0xa1));
        e.set(second, AStarNode::pParentAStarNode, first);
        let nodes = e.mem.alloc(0x10);
        let node_buffer = e.mem.alloc(0x28);
        e.mem.set_u32(nodes + 4, node_buffer);
        e.mem.set_u32(nodes + 0xc, 2);
        let links = e.mem.alloc(0x10);
        let link_buffer = e.mem.alloc(0xc);
        e.mem.set_u32(links + 4, link_buffer);
        e.mem.set_u32(links + 0xc, 1);
        assert!(e
            .call(0x006a8290, &args![this, second, nodes, links])
            .bool());
        assert_eq!(
            [e.mem.u32(node_buffer), e.mem.u32(node_buffer + 0x14)],
            [0xa1, 0xa2]
        );
        assert_eq!(e.mem.u32(link_buffer), 0xa1);
        assert_eq!([e.mem.u32(nodes + 8), e.mem.u32(links + 8)], [2, 1]);
        assert!(!e.call(0x006a8290, &args![this, 0u32, nodes, links]).bool());
    }

    #[test]
    fn node_heap_is_a_0x7800_scrap_heap() {
        let mut e = Engine::new();
        silent(&mut e, &[SCRAP_HEAP_CONSTRUCT]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006a8380, &args![0x4000u32]).u32(), 0x4000);
        assert_eq!(
            calls_to(&e, SCRAP_HEAP_CONSTRUCT),
            vec![(SCRAP_HEAP_CONSTRUCT, vec![0x4000, 0x7800])]
        );
    }

    #[test]
    fn node_allocation_initialises_or_answers_0() {
        let mut e = Engine::new();
        let block = e.mem.alloc(0x3c);
        e.mem.set_u32(block + 0x34, 9);
        e.register_double(SCRAP_HEAP_ALLOCATE, move |_, a| {
            assert_eq!(a[..2], [0x3c, 0x4000]);
            eax(block)
        });
        assert_eq!(e.call(0x006a83a0, &args![0x4000u32]).u32(), block);
        assert_eq!(e.mem.u32(block + 0x34), 0);
        e.register(SCRAP_HEAP_ALLOCATE, |_, _| eax(0));
        assert_eq!(e.call(0x006a83a0, &args![0x4000u32]).u32(), 0);
    }

    #[test]
    fn queue_keeps_each_bucket_sorted_by_fitness() {
        let mut e = Engine::new();
        common(&mut e);
        let queue: Ptr<AStarQueueNavMesh> = e.new_object();
        let middle = new_node(&mut e, 2.0, 0.0);
        let low = new_node(&mut e, 1.0, 0.0);
        let high = new_node(&mut e, 3.0, 0.0);
        let equal = new_node(&mut e, 2.0, 0.0);
        for node in [middle, low, high, equal] {
            e.call(0x006a83f0, &args![queue, node]);
        }
        let mut order = vec![];
        let mut at = e.mem.u32(queue.addr() + 4);
        while at != 0 {
            order.push(at);
            at = e.mem.u32(at + 0x30);
        }
        assert_eq!(
            order,
            [low.addr(), equal.addr(), middle.addr(), high.addr()]
        );
        assert_eq!(e.get(low, AStarNode::ppPrevNode), queue.addr() + 4);
        assert_eq!(e.get(equal, AStarNode::ppPrevNode), low.addr() + 0x30);
    }

    #[test]
    fn unlink_takes_a_node_out_of_its_list() {
        let mut e = Engine::new();
        common(&mut e);
        let queue: Ptr<AStarQueueNavMesh> = e.new_object();
        let a = new_node(&mut e, 1.0, 0.0);
        let b = new_node(&mut e, 2.0, 0.0);
        let c = new_node(&mut e, 3.0, 0.0);
        for node in [a, b, c] {
            a_star_queue_add_node(&mut e, queue, node);
        }
        e.call(0x006a8490, &args![queue, b]);
        assert_eq!(e.get(a, AStarNode::pNextNode), c.addr());
        assert_eq!(e.get(c, AStarNode::ppPrevNode), a.addr() + 0x30);
        assert_eq!(e.get(b, AStarNode::pNextNode), 0);
        assert_eq!(e.get(b, AStarNode::ppPrevNode), 0);
        // An unlinked node stays unlinked.
        e.call(0x006a8490, &args![queue, b]);
        assert_eq!(e.get(a, AStarNode::pNextNode), c.addr());
    }

    #[test]
    fn queue_clear_zeroes_the_buckets() {
        let mut e = Engine::new();
        silent(&mut e, &[MEMSET]);
        e.call_log = Some(vec![]);
        e.call(0x006a84e0, &args![0x3000u32]);
        assert_eq!(calls_to(&e, MEMSET), vec![(MEMSET, vec![0x3004, 0, 0x50])]);
    }

    #[test]
    fn free_buffer_goes_through_slot_8() {
        let mut e = Engine::new();
        e.register(0x0000_1160, |_, _| Ret::default());
        e.put_vtable(0x0000_6000, &[0, 0, 0x0000_1160]);
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array, 0x0000_6000);
        e.mem.set_u32(array + 4, 0x8888);
        e.call_log = Some(vec![]);
        e.call(0x006a8500, &args![array]);
        assert_eq!(
            calls_to(&e, 0x0000_1160),
            vec![(0x0000_1160, vec![array, 0x8888])]
        );
        assert_eq!(e.mem.u32(array + 4), 0);
    }

    #[test]
    fn scrap_array_constructor_takes_the_thread_heap() {
        let mut e = Engine::new();
        silent(&mut e, &[ARRAY_INIT]);
        e.register(MEMORY_MANAGER, |_, _| eax(0x011f_6238));
        e.register(GET_THREAD_SCRAP_HEAP, |_, a| eax(a[0] + 1));
        let array = e.mem.alloc(0x14);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006a8530, &args![array]).u32(), array);
        assert_eq!(e.mem.u32(array), NODE_SCRAP_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(array + 0x10), 0x011f_6239);
        assert_eq!(calls_to(&e, ARRAY_INIT).len(), 2);
    }

    #[test]
    fn node_array_destructor_frees_the_buffer() {
        let mut e = Engine::new();
        silent(&mut e, &[ARRAY_CLEAR]);
        let array = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x006a85b0, &args![array]);
        assert_eq!(e.mem.u32(array), NODE_ARRAY_VTABLE);
        assert_eq!(
            calls_to(&e, ARRAY_CLEAR),
            vec![(ARRAY_CLEAR, vec![array, 1])]
        );
    }

    #[test]
    fn node_array_scalar_deleting_destructor() {
        let mut e = Engine::new();
        silent(&mut e, &[ARRAY_CLEAR, FREE]);
        let array = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x006a85d0, &args![array, 1u32]);
        assert_eq!(calls_to(&e, FREE), vec![(FREE, vec![array])]);
        assert_eq!(e.mem.u32(array), NODE_ARRAY_VTABLE);
    }

    #[test]
    fn scrap_array_destructor_clears_twice() {
        let mut e = Engine::new();
        silent(&mut e, &[ARRAY_CLEAR]);
        let array = e.mem.alloc(0x14);
        e.call_log = Some(vec![]);
        e.call(0x006a8600, &args![array]);
        assert_eq!(calls_to(&e, ARRAY_CLEAR).len(), 2);
        assert_eq!(e.mem.u32(array), NODE_ARRAY_VTABLE);
    }

    #[test]
    fn pop_takes_the_head_of_the_first_non_empty_bucket() {
        let mut e = Engine::new();
        common(&mut e);
        let queue: Ptr<AStarQueueNavMesh> = e.new_object();
        assert_eq!(e.call(0x006a8660, &args![queue]).u32(), 0);
        let a = new_node(&mut e, 1.0, 0.0);
        let b = new_node(&mut e, 2.0, 0.0);
        e.register(QUEUE_BUCKET, |_, _| eax(3));
        a_star_queue_add_node(&mut e, queue, a);
        a_star_queue_add_node(&mut e, queue, b);
        assert_eq!(e.call(0x006a8660, &args![queue]).u32(), a.addr());
        assert_eq!(e.mem.u32(queue.addr() + 4 + 12), b.addr());
        assert_eq!(e.get(b, AStarNode::ppPrevNode), queue.addr() + 4 + 12);
        assert_eq!(e.get(a, AStarNode::pNextNode), 0);
        assert_eq!(e.call(0x006a8660, &args![queue]).u32(), b.addr());
        assert_eq!(e.mem.u32(queue.addr() + 4 + 12), 0);
    }

    #[test]
    fn node_constructor_zeroes_the_node() {
        let mut e = Engine::new();
        let node = e.mem.alloc(0x3c);
        for i in 0..15 {
            e.mem.set_u32(node + 4 * i, 0xffff_ffff);
        }
        assert_eq!(e.call(0x006a8700, &args![node]).u32(), node);
        for offset in [0, 4, 0x28, 0x2c, 0x30, 0x34, 0x38] {
            assert_eq!(e.mem.u32(node + offset), 0);
        }
        // The node data and the parent link are left.
        assert_eq!(e.mem.u32(node + 8), 0xffff_ffff);
    }

    /// An array of `size` elements with room for `reserved`, buffer at
    /// 0x1000; its slot +4 allocates 0x9000.
    fn sized_array(e: &mut Engine, size: u32, reserved: u32) -> u32 {
        e.register(0x0000_1170, |_, _| eax(0x9000));
        e.put_vtable(0x0000_7000, &[0, 0x0000_1170]);
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array, 0x0000_7000);
        e.mem.set_u32(array + 4, 0x1000);
        e.mem.set_u32(array + 8, size);
        e.mem.set_u32(array + 0xc, reserved);
        array
    }

    #[test]
    fn node_array_set_size_grows_and_shrinks() {
        let mut e = Engine::new();
        silent(
            &mut e,
            &[
                ARRAY_CLEAR,
                REALLOCATE_NODES,
                CONSTRUCT_NODES,
                DESTROY_ELEMENTS,
            ],
        );
        e.call_log = Some(vec![]);
        // 0 empties.
        let array = sized_array(&mut e, 3, 4);
        e.call(0x006a8750, &args![array, 0u32, 1u8]);
        assert_eq!(
            calls_to(&e, ARRAY_CLEAR),
            vec![(ARRAY_CLEAR, vec![array, 1])]
        );
        // From nothing: allocates through slot +4.
        let array = sized_array(&mut e, 0, 0);
        e.call(0x006a8750, &args![array, 2u32, 1u8]);
        assert_eq!(e.mem.u32(array + 4), 0x9000);
        assert_eq!([e.mem.u32(array + 8), e.mem.u32(array + 0xc)], [2, 2]);
        assert_eq!(calls_to(&e, CONSTRUCT_NODES)[0].1, vec![array, 0x9000, 2]);
        // Past the reserve: reallocates.
        let array = sized_array(&mut e, 2, 3);
        e.call(0x006a8750, &args![array, 5u32, 0u8]);
        assert_eq!(calls_to(&e, REALLOCATE_NODES)[0].1, vec![array, 5, 2]);
        assert_eq!(calls_to(&e, CONSTRUCT_NODES)[1].1, vec![array, 0x1028, 3]);
        // Within the reserve.
        let array = sized_array(&mut e, 1, 8);
        e.call(0x006a8750, &args![array, 3u32, 0u8]);
        assert_eq!(calls_to(&e, CONSTRUCT_NODES)[2].1, vec![array, 0x1014, 2]);
        // Shrinking to a quarter of the reserve reallocates.
        let array = sized_array(&mut e, 6, 8);
        e.call(0x006a8750, &args![array, 2u32, 1u8]);
        assert_eq!(calls_to(&e, DESTROY_ELEMENTS)[0].1, vec![array, 0x1028, 4]);
        assert_eq!(calls_to(&e, REALLOCATE_NODES)[1].1, vec![array, 2, 2]);
        assert_eq!(e.mem.u32(array + 0xc), 2);
        // Not small enough: no reallocation.
        let array = sized_array(&mut e, 6, 8);
        e.call(0x006a8750, &args![array, 3u32, 1u8]);
        assert_eq!(calls_to(&e, REALLOCATE_NODES).len(), 2);
        assert_eq!(e.mem.u32(array + 8), 3);
    }

    #[test]
    fn link_array_set_size_uses_12_byte_elements() {
        let mut e = Engine::new();
        silent(
            &mut e,
            &[
                ARRAY_CLEAR,
                REALLOCATE_LINKS,
                CONSTRUCT_LINKS,
                DESTROY_ELEMENTS,
            ],
        );
        e.call_log = Some(vec![]);
        let array = sized_array(&mut e, 2, 3);
        e.call(0x006a8890, &args![array, 5u32, 0u8]);
        assert_eq!(calls_to(&e, REALLOCATE_LINKS)[0].1, vec![array, 5, 2]);
        assert_eq!(calls_to(&e, CONSTRUCT_LINKS)[0].1, vec![array, 0x1018, 3]);
        let array = sized_array(&mut e, 6, 8);
        e.call(0x006a8890, &args![array, 2u32, 1u8]);
        assert_eq!(calls_to(&e, DESTROY_ELEMENTS)[0].1, vec![array, 0x1018, 4]);
        assert_eq!(calls_to(&e, REALLOCATE_LINKS)[1].1, vec![array, 2, 2]);
        e.call(0x006a8890, &args![array, 0u32, 0u8]);
        assert_eq!(
            calls_to(&e, ARRAY_CLEAR),
            vec![(ARRAY_CLEAR, vec![array, 0])]
        );
    }

    #[test]
    fn scrap_array_scalar_deleting_destructor() {
        let mut e = Engine::new();
        silent(&mut e, &[ARRAY_CLEAR, FREE]);
        let array = e.mem.alloc(0x14);
        e.call_log = Some(vec![]);
        e.call(0x006a89d0, &args![array, 1u32]);
        assert_eq!(calls_to(&e, FREE), vec![(FREE, vec![array])]);
        assert_eq!(calls_to(&e, ARRAY_CLEAR).len(), 2);
    }

    #[test]
    fn node_array_constructor_initialises_empty() {
        let mut e = Engine::new();
        silent(&mut e, &[ARRAY_INIT]);
        let array = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006a8a00, &args![array]).u32(), array);
        assert_eq!(e.mem.u32(array), NODE_ARRAY_VTABLE);
        assert_eq!(
            calls_to(&e, ARRAY_INIT),
            vec![(ARRAY_INIT, vec![array, 0, 0])]
        );
    }
}
