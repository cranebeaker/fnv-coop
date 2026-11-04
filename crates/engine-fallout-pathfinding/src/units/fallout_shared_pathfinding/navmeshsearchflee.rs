//! `fallout shared/pathfinding/navmeshsearchflee.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `NavMeshSearchFlee` is the A* search behind `PathingRequestFlee`: an
//! `AStarSearch<NavMeshSearchNode, NavMeshSearchLink>` (vtable slot `+4`
//! `GetNodeConnections`, slot `+8` `IsGoalNode`) plus the request, the start
//! node, the centre of the avoid nodes, the distance bound and an optional
//! goal it prefers to run towards. As in `NavMeshSearchClosePoint`, the
//! members after the base sit 4 bytes lower on PC than in the Xbox PDB
//! (`pCurrentRequest` is at `+0x2058`, the PDB says `+0x205C`).
//!
//! The search nodes are `AStarNode<NavMeshSearchNode, NavMeshSearchLink>`
//! (layout in `navmeshsearchclosepoint.rs`; `fCost`, at `+0`, is read here
//! at its offset). The request is a `PathingRequestFlee` (a
//! `PathingRequest`, layout in `pathfind.rs`, with `fMinFleeDistance` at
//! `+0xB0`), read through its small getters.
//!
//! The unit also holds three one-line getters on `PathingRequest` and
//! `PathingLocation` and the node key function the other searches share.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_shared_pathfinding::navmeshsearchclosepoint::AStarNodeNavMeshSearch;

layout! {
    /// `NavMeshSearchFlee` (Xbox PDB), 0x2094 bytes on the Xbox; only the
    /// members this unit touches, at their PC offsets.
    pub struct NavMeshSearchFlee: 0x2090 {
        /// `pCurrentRequest` (Xbox PDB): `const PathingRequestFlee *`.
        0x2058 pCurrentRequest: Ptr,
        /// `fDistanceUpperBound` (Xbox PDB).
        0x207C fDistanceUpperBound: f32,
        /// `bFavorGoingTowardsGoal` (Xbox PDB).
        0x2080 bFavorGoingTowardsGoal: bool,
    }
}

/// `StartNode` (Xbox PDB): a `NavMeshSearchNode` (navmesh, triangle,
/// centre; 0x14 bytes).
const START_NODE: u32 = 0x205c;
/// `AvoidNodeCenter` (Xbox PDB): an `NiPoint3`.
const AVOID_NODE_CENTER: u32 = 0x2070;
/// `GoalLocation` (Xbox PDB): an `NiPoint3`.
const GOAL_LOCATION: u32 = 0x2084;

/// `0084e3a0(form)`: the form's id (the word at `+0xC`).
const FORM_ID: u32 = 0x0084_e3a0;
/// `NavMesh::GetCenter` (`00558220`, map name; thiscall `(mesh, out,
/// triangle)`): writes the triangle's centre.
const GET_CENTER: u32 = 0x0055_8220;
/// `00439ef0(point, out, other)`: `out = point - other`, returns `out`.
const VECTOR_SUBTRACT: u32 = 0x0043_9ef0;
/// `00457990(vector)`: the length, in ST0.
const VECTOR_LENGTH: u32 = 0x0045_7990;
/// `005330e0(&value, minimum, maximum)` (cdecl): clamps a float in place.
const CLAMP: u32 = 0x0053_30e0;
/// `0048d150(request)`: the address of the request's origin (`+0xC`).
const REQUEST_ORIGIN: u32 = 0x0048_d150;
/// `005c3420(location, out)`: copies the location's position, returns `out`.
const LOCATION_POSITION: u32 = 0x005c_3420;
/// The global three-word vector the avoid centre and the direction start
/// from (copied as words).
const DEFAULT_POINT: u32 = 0x011f_426c;
/// `006d2c20(request)`: the float at `+0xB0` (`PathingRequestFlee`'s
/// `fMinFleeDistance`), in ST0.
const REQUEST_MIN_FLEE_DISTANCE: u32 = 0x006d_2c20;
/// `0069e390(request)`: the request's avoid node array, or 0.
const REQUEST_AVOID_NODES: u32 = 0x0069_e390;
/// `006dc9f0(array, &point, &out)`: computes the avoid nodes' centre around
/// the point into `out` (its float result in ST0 is dropped).
const AVOID_NODES_CENTER: u32 = 0x006d_c9f0;
/// `AStarSearch<..>::AddStartNode` (`006a7c90`, map name; thiscall `(this,
/// key, &node data, cost)`): the start node.
const ADD_START_NODE: u32 = 0x006a_7c90;
/// `AStarSearch<..>::Search` (`006a7d60`, map name; thiscall `(this, path,
/// limit, allow incomplete)`).
const SEARCH: u32 = 0x006a_7d60;
/// `0076b610(array)`: whether the array's `iSize` is 0.
const ARRAY_IS_EMPTY: u32 = 0x0076_b610;
/// `006b31a0(path)`: the path's last element.
const PATH_LAST: u32 = 0x006b_31a0;
/// `006a6eb0(node data, out)`: writes the node's centre.
const NODE_CENTER: u32 = 0x006a_6eb0;
/// The double `0.5`.
const HALF: u32 = 0x0101_1588;
/// The `float` `256.0` the half distance is limited to.
const MAX_HALF_DISTANCE: u32 = 0x0103_2adc;
/// `0040ebd0(a, b)` (cdecl, two floats): the smaller, in ST0.
const MIN: u32 = 0x0040_ebd0;
/// `00404010(a, b)` (cdecl, two floats): the larger, in ST0.
const MAX: u32 = 0x0040_4010;
/// `004a7290(vector)`: the squared length, in ST0.
const LENGTH_SQUARED: u32 = 0x004a_7290;
/// `_finite` (`00ec7595`, cdecl `double`).
const FINITE: u32 = 0x00ec_7595;
/// `004a0c10(vector)`: normalizes the vector in place (zero when its length
/// is at most `1e-6`).
const NORMALIZE: u32 = 0x004a_0c10;
/// `NavMesh::GetMatchingEdge_ov3` (`0068f460`, map name; `(mesh, triangle,
/// edge, &mesh, &triangle, &other)`).
const GET_MATCHING_EDGE: u32 = 0x0068_f460;
/// `NavMesh::GetEdgeExtraInfo` (`0068f230`, map name; `(mesh, triangle,
/// edge)`).
const GET_EDGE_EXTRA_INFO: u32 = 0x0068_f230;
/// `005582f0(mesh, triangle)`: the triangle's record.
const GET_TRIANGLE: u32 = 0x0055_82f0;
/// `00691140(record, mask)`: whether the record's flags meet the mask.
const TRIANGLE_HAS_FLAG: u32 = 0x0069_1140;
/// The triangle flag of a door, and the one the radius test checks.
const TRIANGLE_DOOR_FLAG: u32 = 0x1000;
const TRIANGLE_BLOCKED_FLAG: u32 = 0x10;
/// `0073cb80(request)`: `bCantOpenDoors` (`+0x98`).
const REQUEST_CANT_OPEN_DOORS: u32 = 0x0073_cb80;
/// `NavMesh::GetClosedDoorInfoForTriangle` (`00699af0`, map name).
const GET_DOOR_INFO: u32 = 0x0069_9af0;
/// `004839c0(form id)` (cdecl): the form, or 0.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `DoorLock::DoorLock` (`00501d20`), `~DoorLock` (`00501e50`) and
/// `CanActorUnlockDoorLock` (`00502450`, `(lock, lock data, door, &flag)`).
const DOOR_LOCK_CONSTRUCT: u32 = 0x0050_1d20;
const DOOR_LOCK_DESTRUCT: u32 = 0x0050_1e50;
const DOOR_LOCK_CAN_UNLOCK: u32 = 0x0050_2450;
/// `006c9900(request)`: the address of the request's lock data.
const REQUEST_LOCK_DATA: u32 = 0x006c_9900;
/// `007af430(door)`: the door's base form.
const DOOR_BASE_FORM: u32 = 0x007a_f430;
/// `TESObjectDOOR::IsMinimalUse` (`00518000`, map name).
const IS_MINIMAL_USE: u32 = 0x0051_8000;
/// The cost of a minimal-use door (`float` `100.0`).
const MINIMAL_USE_DOOR_COST: u32 = 0x0101_6410;
/// `0069ef80(request)`: `fActorRadius`, in ST0.
const REQUEST_ACTOR_RADIUS: u32 = 0x0069_ef80;
/// `00403e20(setting)`: the address of a setting's float.
const SETTING_VALUE: u32 = 0x0040_3e20;
/// The setting whose float is compared with the actor radius.
const RADIUS_SETTING: u32 = 0x011d_6ef0;
/// `00589850(vector)`: the length of x and y, in ST0.
const FLAT_LENGTH: u32 = 0x0058_9850;
/// `PathingAvoidNodeArray::GetCostForPoint` (`006dc990`, map name; `(array,
/// &point)`), in ST0.
const AVOID_COST_FOR_POINT: u32 = 0x006d_c990;
/// `NiPoint3::Dot` (`004b6190`, map name), in ST0.
const DOT: u32 = 0x004b_6190;
/// The double `1000000.0` weighting the direction penalty.
const DIRECTION_WEIGHT: u32 = 0x0106_b798;
/// The double `10.0` weighting the distance to the goal.
const GOAL_WEIGHT: u32 = 0x0102_0758;
/// `AStarSearch<..>::GetOrCreateAStarNode` (`006a7e00`, map name; `(this,
/// key, cost, &node)`): 0 created, 1 found and better, otherwise skip.
const GET_OR_CREATE_NODE: u32 = 0x006a_7e00;
/// `007cb2e0(list, &item)`: appends a word.
const LIST_ADD: u32 = 0x007c_b2e0;
/// `0042fb00(holder)` / `0042fa40(holder)`: constructor (null) and
/// destructor of a local one-word smart pointer; `00559450(holder)` reads
/// its pointer.
const HOLDER_CONSTRUCT: u32 = 0x0042_fb00;
const HOLDER_DESTRUCT: u32 = 0x0042_fa40;
const HOLDER_GET: u32 = 0x0055_9450;
/// `PathingLocation::GetNavMeshAndTriangle` (`006dd640`, map name;
/// `(location, &mesh holder, &triangle)`).
const GET_NAV_MESH_AND_TRIANGLE: u32 = 0x006d_d640;
/// `00413f40(node)`: the address of the node's data (`+8`).
const NODE_DATA: u32 = 0x0041_3f40;
/// `009962d0(search)`: the word at `+0x2050` of the search.
const SEARCH_WORD_2050: u32 = 0x0099_62d0;
/// `00595c80(vector)`: the squared length of x and y, in ST0.
const FLAT_LENGTH_SQUARED: u32 = 0x0059_5c80;
/// The double `0.0`.
const ZERO: u32 = 0x0101_2060;

/// Copies `words` words from `source` to `dest`.
fn copy_words(e: &mut Engine, dest: u32, source: u32, words: u32) {
    for word in 0..words {
        let value = e.mem.u32(source.wrapping_add(4 * word));
        e.mem.set_u32(dest.wrapping_add(4 * word), value);
    }
}

// Translated from 006a92e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshSearchFlee::BuildNodePath` (Xbox PDB): runs the flee search for
/// `request` from `start_node` into `path`. Keeps the request and the start
/// node; takes the request origin's position; resets the avoid centre to
/// the vector at `011f426c` and, when the request has avoid nodes, lets
/// `006dc9f0` compute it around the origin; sets the distance bound to the
/// request's minimum flee distance. Adds the start node (cost 0.0) and fills
/// it ([`nav_mesh_search_flee_set_node_data`]). When the request's
/// destination location lacks flag bit 1 ([`fn_006a9520`]) the search
/// favours its position as a goal. Then `Search(path, limit,
/// bAllowIncompletePath)`; when that fails but left a path, it still
/// succeeds if the path's last node is farther (squared distance) from the
/// origin than `min(half the minimum flee distance, 256.0)`. `RET 0x10`.
pub fn nav_mesh_search_flee_build_node_path(
    e: &mut Engine,
    this: Ptr<NavMeshSearchFlee>,
    request: Ptr,
    start_node: Ptr,
    path: Ptr,
    limit: u32,
) -> bool {
    let t = this.addr();
    e.set(this, NavMeshSearchFlee::pCurrentRequest, request);
    copy_words(e, t.wrapping_add(START_NODE), start_node.addr(), 5);
    e.with_stack(0x30, |e, stack| {
        let s = stack.addr();
        let origin_position = s;
        let goal = s + 0xc;
        let last_center = s + 0x18;
        let difference = s + 0x24;
        let origin = e.call(REQUEST_ORIGIN, &args![request]).u32();
        e.call(LOCATION_POSITION, &args![origin, origin_position]);
        copy_words(e, t.wrapping_add(AVOID_NODE_CENTER), DEFAULT_POINT, 3);
        let bound = e.call(REQUEST_MIN_FLEE_DISTANCE, &args![request]).f32();
        e.set(this, NavMeshSearchFlee::fDistanceUpperBound, bound);
        let current = e.get(this, NavMeshSearchFlee::pCurrentRequest);
        if e.call(REQUEST_AVOID_NODES, &args![current]).u32() != 0 {
            let avoid_nodes = e.call(REQUEST_AVOID_NODES, &args![current]).u32();
            e.call(
                AVOID_NODES_CENTER,
                &args![
                    avoid_nodes,
                    origin_position,
                    t.wrapping_add(AVOID_NODE_CENTER)
                ],
            );
        }
        let start = start_node.addr();
        let mesh = Ptr::new(e.mem.u32(start));
        let triangle = e.mem.u16(start.wrapping_add(4));
        let key = fn_006a94f0(e, this.cast(), mesh, triangle);
        let node = e
            .call(ADD_START_NODE, &args![this, key, start_node, 0.0f32])
            .u32();
        nav_mesh_search_flee_set_node_data(e, this, Ptr::new(node), mesh, triangle);
        let destination = fn_006a9540(e, request);
        if !fn_006a9520(e, destination) {
            e.set(this, NavMeshSearchFlee::bFavorGoingTowardsGoal, true);
            let current = e.get(this, NavMeshSearchFlee::pCurrentRequest);
            let destination = fn_006a9540(e, current);
            let position = e.call(LOCATION_POSITION, &args![destination, goal]).u32();
            copy_words(e, t.wrapping_add(GOAL_LOCATION), position, 3);
        }
        let current = e.get(this, NavMeshSearchFlee::pCurrentRequest);
        let allow_incomplete = u32::from(fn_006a9560(e, current));
        let mut found = e
            .call(SEARCH, &args![this, path, limit, allow_incomplete])
            .u8();
        if found == 0 && !e.call(ARRAY_IS_EMPTY, &args![path]).bool() {
            let last = e.call(PATH_LAST, &args![path]).u32();
            e.call(NODE_CENTER, &args![last, last_center]);
            let current = e.get(this, NavMeshSearchFlee::pCurrentRequest);
            let distance = e.call(REQUEST_MIN_FLEE_DISTANCE, &args![current]).f64();
            let half = (distance * e.global::<f64>(HALF)) as f32;
            let cap = x87_quiet(e.global::<f32>(MAX_HALF_DISTANCE));
            let reach = e.call(MIN, &args![half, cap]).f32();
            e.call(
                VECTOR_SUBTRACT,
                &args![origin_position, difference, last_center],
            );
            let squared = e.call(LENGTH_SQUARED, &args![difference]).f64();
            if f64::from(reach) * f64::from(reach) < squared {
                found = 1;
            }
        }
        found != 0
    })
}

// Translated from 006a94f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the key of a search node: the low 16 bits of
/// the navmesh's form id (`0084e3a0`) above the triangle index. `this` (a
/// search) is not read. `RET 8`.
pub fn fn_006a94f0(e: &mut Engine, _this: Ptr, mesh: Ptr, triangle: u16) -> u32 {
    let id = e.call(FORM_ID, &args![mesh]).u32();
    ((id & 0xffff) << 16) | u32::from(triangle)
}

// Translated from 006a9520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether bit 1 of the `PathingLocation`'s flag
/// byte (`uiFlags`, `+0x26`) is set.
pub fn fn_006a9520(e: &mut Engine, location: Ptr) -> bool {
    e.mem.u8(location.addr().wrapping_add(0x26)) & 2 != 0
}

// Translated from 006a9540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the address of a `PathingRequest`'s
/// `Destination` (`+0x34`).
pub fn fn_006a9540(_e: &mut Engine, request: Ptr) -> Ptr {
    Ptr::new(request.addr().wrapping_add(0x34))
}

// Translated from 006a9560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a `PathingRequest`'s `bAllowIncompletePath`
/// byte (`+0x9B`).
pub fn fn_006a9560(e: &mut Engine, request: Ptr) -> u8 {
    e.mem.u8(request.addr().wrapping_add(0x9b))
}

// Translated from 006a9580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshSearchFlee::SetNodeData` (Xbox PDB): fills a search node for
/// `triangle` of `mesh`: the mesh, the triangle, its centre (`00558220`,
/// copied through x87) and the heuristic: the distance bound minus the
/// length (`00457990`) of the centre minus the avoid centre, clamped to
/// 0.0..bound (`005330e0`). `RET 0xc`.
pub fn nav_mesh_search_flee_set_node_data(
    e: &mut Engine,
    this: Ptr<NavMeshSearchFlee>,
    node: Ptr<AStarNodeNavMeshSearch>,
    mesh: Ptr,
    triangle: u16,
) {
    e.set(node, AStarNodeNavMeshSearch::pNavMesh, mesh);
    e.set(node, AStarNodeNavMeshSearch::sTriangle, triangle);
    e.with_stack(0x1c, |e, stack| {
        let center = stack.addr();
        let offset = center + 0xc;
        let heuristic = center + 0x18;
        e.call(GET_CENTER, &args![mesh, center, u32::from(triangle)]);
        for (field, delta) in [
            (AStarNodeNavMeshSearch::CenterX, 0u32),
            (AStarNodeNavMeshSearch::CenterY, 4),
            (AStarNodeNavMeshSearch::CenterZ, 8),
        ] {
            let value = x87_quiet(e.mem.f32(center + delta));
            e.set(node, field, value);
        }
        e.call(
            VECTOR_SUBTRACT,
            &args![center, offset, this.addr().wrapping_add(AVOID_NODE_CENTER)],
        );
        let length = e.call(VECTOR_LENGTH, &args![offset]).f64();
        let bound = e.get(this, NavMeshSearchFlee::fDistanceUpperBound);
        e.mem.set_f32(heuristic, (f64::from(bound) - length) as f32);
        e.call(CLAMP, &args![heuristic, 0.0f32, x87_quiet(bound)]);
        let value = x87_quiet(e.mem.f32(heuristic));
        e.set(node, AStarNodeNavMeshSearch::fHeuristic, value);
    });
}

/// The stack locals of `GetNodeConnections` the callees get addresses of.
struct Locals {
    position: u32,
    direction: u32,
    mesh: u32,
    triangle: u32,
    other: u32,
    lock: u32,
    flag: u32,
    center: u32,
    offset: u32,
    unit: u32,
    to_goal: u32,
    new_node: u32,
}

impl Locals {
    const SIZE: u32 = 0x8c;

    fn new(base: u32) -> Self {
        Locals {
            position: base,
            direction: base + 0xc,
            mesh: base + 0x18,
            triangle: base + 0x1c,
            other: base + 0x20,
            lock: base + 0x24,
            flag: base + 0x44,
            center: base + 0x48,
            offset: base + 0x54,
            unit: base + 0x60,
            to_goal: base + 0x6c,
            new_node: base + 0x78,
        }
    }
}

/// The edge's door check: `None` when the edge is skipped, else the cost
/// (1.0, or the minimal-use door cost).
fn door_cost(e: &mut Engine, request: Ptr, mesh: u32, triangle: u32, l: &Locals) -> Option<f32> {
    let mut cost = 1.0f32;
    let record = e.call(GET_TRIANGLE, &args![mesh, triangle]).u32();
    if !e
        .call(TRIANGLE_HAS_FLAG, &args![record, TRIANGLE_DOOR_FLAG])
        .bool()
    {
        return Some(cost);
    }
    if e.call(REQUEST_CANT_OPEN_DOORS, &args![request]).bool() {
        return None;
    }
    let info = e.call(GET_DOOR_INFO, &args![mesh, triangle]).u32();
    let door_id = e.mem.u32(info);
    let door = e.call(LOOKUP_FORM, &args![door_id]).u32();
    if door == 0 {
        return Some(cost);
    }
    e.call(DOOR_LOCK_CONSTRUCT, &args![l.lock, door]);
    let lock_data = e.call(REQUEST_LOCK_DATA, &args![request]).u32();
    let unlocks = e
        .call(
            DOOR_LOCK_CAN_UNLOCK,
            &args![l.lock, lock_data, door, l.flag],
        )
        .bool();
    if !unlocks {
        e.call(DOOR_LOCK_DESTRUCT, &args![l.lock]);
        return None;
    }
    let base = e.call(DOOR_BASE_FORM, &args![door]).u32();
    if e.call(IS_MINIMAL_USE, &args![base]).bool() {
        cost = x87_quiet(e.global::<f32>(MINIMAL_USE_DOOR_COST));
    }
    e.call(DOOR_LOCK_DESTRUCT, &args![l.lock]);
    Some(cost)
}

/// One edge of `GetNodeConnections`.
fn expand_edge(
    e: &mut Engine,
    this: Ptr<NavMeshSearchFlee>,
    node: Ptr<AStarNodeNavMeshSearch>,
    list: Ptr,
    edge: u32,
    has_avoid_nodes: bool,
    l: &Locals,
) {
    let node_mesh = e.get(node, AStarNodeNavMeshSearch::pNavMesh);
    let node_triangle = u32::from(e.get(node, AStarNodeNavMeshSearch::sTriangle));
    let found = e
        .call(
            GET_MATCHING_EDGE,
            &args![node_mesh, node_triangle, edge, l.mesh, l.triangle, l.other],
        )
        .bool();
    if !found {
        return;
    }
    let extra = e
        .call(GET_EDGE_EXTRA_INFO, &args![node_mesh, node_triangle, edge])
        .u32();
    if extra != 0 && e.mem.u32(extra) == 1 {
        return;
    }
    let request = e.get(this, NavMeshSearchFlee::pCurrentRequest);
    let mesh = e.mem.u32(l.mesh);
    let triangle = u32::from(e.mem.u16(l.triangle));
    let Some(cost) = door_cost(e, request, mesh, triangle, l) else {
        return;
    };
    let request = e.get(this, NavMeshSearchFlee::pCurrentRequest);
    let radius = e.call(REQUEST_ACTOR_RADIUS, &args![request]).f64();
    let setting = e.call(SETTING_VALUE, &args![RADIUS_SETTING]).u32();
    if f64::from(e.mem.f32(setting)) < radius {
        let record = e.call(GET_TRIANGLE, &args![mesh, triangle]).u32();
        if e.call(TRIANGLE_HAS_FLAG, &args![record, TRIANGLE_BLOCKED_FLAG])
            .bool()
        {
            return;
        }
    }
    e.call(GET_CENTER, &args![mesh, l.center, triangle]);
    e.call(VECTOR_SUBTRACT, &args![l.center, l.offset, l.position]);
    let flat = e.call(FLAT_LENGTH, &args![l.offset]).f64();
    let node_cost = e.mem.f32(node.addr()); // AStarNode::fCost (Xbox PDB) +0x00
    let mut total = (flat * f64::from(cost) + f64::from(node_cost)) as f32;
    if has_avoid_nodes {
        let request = e.get(this, NavMeshSearchFlee::pCurrentRequest);
        let avoid_nodes = e.call(REQUEST_AVOID_NODES, &args![request]).u32();
        let avoid_cost = e
            .call(AVOID_COST_FOR_POINT, &args![avoid_nodes, l.center])
            .f64();
        total = (avoid_cost + f64::from(total)) as f32;
        copy_words(e, l.unit, l.offset, 3);
        e.call(NORMALIZE, &args![l.unit]);
        let along = e.call(DOT, &args![l.unit, l.direction]).f32();
        let penalty = e.call(MAX, &args![0.0f32, along]).f32();
        let weight = e.global::<f64>(DIRECTION_WEIGHT);
        total = (f64::from(penalty) * weight + f64::from(total)) as f32;
    }
    if e.get(this, NavMeshSearchFlee::bFavorGoingTowardsGoal) {
        let to_goal = e
            .call(
                VECTOR_SUBTRACT,
                &args![l.center, l.to_goal, this.addr().wrapping_add(GOAL_LOCATION)],
            )
            .u32();
        let distance = e.call(FLAT_LENGTH, &args![to_goal]).f32();
        let weight = e.global::<f64>(GOAL_WEIGHT);
        total = (f64::from(distance) * weight + f64::from(total)) as f32;
    }
    let key = fn_006a94f0(e, this.cast(), Ptr::new(mesh), triangle as u16);
    let status = e
        .call(
            GET_OR_CREATE_NODE,
            &args![this, key, x87_quiet(total), l.new_node],
        )
        .i32();
    if status == 0 {
        let new_node = Ptr::new(e.mem.u32(l.new_node));
        nav_mesh_search_flee_set_node_data(e, this, new_node, Ptr::new(mesh), triangle as u16);
    } else if status != 1 {
        return;
    }
    let new_node = Ptr::<AStarNodeNavMeshSearch>::new(e.mem.u32(l.new_node));
    let parent_mesh = e.get(node, AStarNodeNavMeshSearch::pNavMesh);
    e.set(
        new_node,
        AStarNodeNavMeshSearch::ParentLink_pNavMesh,
        parent_mesh,
    );
    let parent_triangle = e.get(node, AStarNodeNavMeshSearch::sTriangle);
    e.set(
        new_node,
        AStarNodeNavMeshSearch::ParentLink_sTriangle,
        parent_triangle,
    );
    e.set(
        new_node,
        AStarNodeNavMeshSearch::ParentLink_iEdgeIndex,
        edge,
    );
    e.call(LIST_ADD, &args![list, l.new_node]);
}

// Translated from 006a9620 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006a9620 - FS:[0] exception frame (unwinding of the DoorLock temporary) is not translated.
/// `NavMeshSearchFlee::GetNodeConnections` (Xbox PDB): appends to `list` the
/// search nodes reached over edges 0, 1 and 2 of `node`'s triangle. The
/// direction away from danger is the vector at `011f426c`, or, with avoid
/// nodes, the normalized avoid centre minus the node's centre. A node cost
/// that is not finite (`_finite`) is reset to 0.0 first. Each edge with a
/// neighbour (`0068f460`) and no extra info of type 1 has cost 1.0; a door
/// triangle (flag 0x1000) is skipped when the request cannot open doors or
/// the actor cannot unlock the door's `DoorLock`, and costs the `float` at
/// `01016410` when the door is minimal-use; the edge is also skipped when
/// the setting float at `011d6ef0` is below the actor radius and the
/// triangle has flag 0x10. The neighbour's cost is the node's cost plus the
/// flat distance to its centre times the edge cost, plus (with avoid nodes)
/// the avoid cost at the centre and `1e6 * max(0, dir . unit offset)`, plus
/// (favouring the goal) `10 *` the flat distance from the goal. The node is
/// created or found (`006a7e00`), filled when new, given the parent link
/// and appended (`007cb2e0`) unless the existing node is not better.
/// Returns true. `RET 8`.
pub fn nav_mesh_search_flee_get_node_connections(
    e: &mut Engine,
    this: Ptr<NavMeshSearchFlee>,
    node: Ptr<AStarNodeNavMeshSearch>,
    list: Ptr,
) -> bool {
    e.with_stack(Locals::SIZE, |e, stack| {
        let l = Locals::new(stack.addr());
        let avoid_offset = stack.addr() + 0x7c;
        e.call(NODE_CENTER, &args![node.addr().wrapping_add(8), l.position]);
        let request = e.get(this, NavMeshSearchFlee::pCurrentRequest);
        let avoid_nodes = e.call(REQUEST_AVOID_NODES, &args![request]).u32();
        copy_words(e, l.direction, DEFAULT_POINT, 3);
        if avoid_nodes != 0 {
            let offset = e
                .call(
                    VECTOR_SUBTRACT,
                    &args![
                        this.addr().wrapping_add(AVOID_NODE_CENTER),
                        avoid_offset,
                        l.position
                    ],
                )
                .u32();
            copy_words(e, l.direction, offset, 3);
            e.call(NORMALIZE, &args![l.direction]);
        }
        let node_cost = x87_quiet(e.mem.f32(node.addr())); // AStarNode::fCost (Xbox PDB) +0x00
        if e.call(FINITE, &args![f64::from(node_cost)]).u32() == 0 {
            e.mem.set_f32(node.addr(), 0.0);
        }
        for edge in 0..3u32 {
            expand_edge(e, this, node, list, edge, avoid_nodes != 0, &l);
        }
        true
    })
}

// Translated from 006a9a70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006a9a70 - FS:[0] exception frame (unwinding of the local smart pointer) is not translated.
/// `NavMeshSearchFlee::IsGoalNode` (Xbox PDB): whether `node` ends the
/// flight. Favouring the goal, the node is a goal when it is the navmesh
/// and triangle of the request's destination (`006dd640`). Otherwise: not
/// the node at `+0x2050` (`009962d0`); its flat squared distance from the
/// request origin (`00595c80`) at least the minimum flee distance squared;
/// favouring the goal, the node no farther (squared, `004a7290`) from the
/// goal than the origin is; and the avoid nodes' cost at its centre
/// (`006dc990`, 0.0 without avoid nodes) at most 0.0. `RET 4`.
pub fn nav_mesh_search_flee_is_goal_node(
    e: &mut Engine,
    this: Ptr<NavMeshSearchFlee>,
    node: Ptr<AStarNodeNavMeshSearch>,
) -> bool {
    let request = e.get(this, NavMeshSearchFlee::pCurrentRequest);
    let destination = fn_006a9540(e, request);
    e.with_stack(0x48, |e, stack| {
        let s = stack.addr();
        let holder = s;
        let triangle = s + 4;
        let center = s + 8;
        let origin_position = s + 0x14;
        let difference = s + 0x20;
        let to_goal = s + 0x2c;
        let origin_to_goal = s + 0x38;
        if e.get(this, NavMeshSearchFlee::bFavorGoingTowardsGoal) {
            e.call(HOLDER_CONSTRUCT, &args![holder]);
            let located = e
                .call(
                    GET_NAV_MESH_AND_TRIANGLE,
                    &args![destination, holder, triangle],
                )
                .bool();
            if located {
                let data = e.call(NODE_DATA, &args![node]).u32();
                let mesh = e.call(HOLDER_GET, &args![holder]).u32();
                if e.mem.u32(data) == mesh && e.mem.u16(data + 4) == e.mem.u16(triangle) {
                    e.call(HOLDER_DESTRUCT, &args![holder]);
                    return true;
                }
            }
            e.call(HOLDER_DESTRUCT, &args![holder]);
        }
        if node.addr() == e.call(SEARCH_WORD_2050, &args![this]).u32() {
            return false;
        }
        e.call(NODE_CENTER, &args![node.addr().wrapping_add(8), center]);
        let request = e.get(this, NavMeshSearchFlee::pCurrentRequest);
        let origin = e.call(REQUEST_ORIGIN, &args![request]).u32();
        let position = e
            .call(LOCATION_POSITION, &args![origin, origin_position])
            .u32();
        let offset = e
            .call(VECTOR_SUBTRACT, &args![position, difference, center])
            .u32();
        let squared = e.call(FLAT_LENGTH_SQUARED, &args![offset]).f32();
        let first = e.call(REQUEST_MIN_FLEE_DISTANCE, &args![request]).f64();
        let second = e.call(REQUEST_MIN_FLEE_DISTANCE, &args![request]).f64();
        if f64::from(squared) < second * first {
            return false;
        }
        if e.get(this, NavMeshSearchFlee::bFavorGoingTowardsGoal) {
            let goal = this.addr().wrapping_add(GOAL_LOCATION);
            e.call(VECTOR_SUBTRACT, &args![goal, to_goal, center]);
            let origin = e.call(REQUEST_ORIGIN, &args![request]).u32();
            let position = e
                .call(LOCATION_POSITION, &args![origin, origin_position])
                .u32();
            e.call(VECTOR_SUBTRACT, &args![position, origin_to_goal, goal]);
            let origin_distance = e.call(LENGTH_SQUARED, &args![origin_to_goal]).f64();
            let node_distance = e.call(LENGTH_SQUARED, &args![to_goal]).f64();
            if node_distance > origin_distance {
                return false;
            }
        }
        let mut cost = 0.0f32;
        if e.call(REQUEST_AVOID_NODES, &args![request]).u32() != 0 {
            let avoid_nodes = e.call(REQUEST_AVOID_NODES, &args![request]).u32();
            cost = e
                .call(AVOID_COST_FOR_POINT, &args![avoid_nodes, center])
                .f32();
        }
        f64::from(cost) <= e.global::<f64>(ZERO)
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x006a92e0,
            nav_mesh_search_flee_build_node_path(
                Ptr<NavMeshSearchFlee>,
                Ptr,
                Ptr,
                Ptr,
                u32,
            ) -> bool
        ),
        entry!(0x006a94f0, fn_006a94f0(Ptr, Ptr, u16) -> u32),
        entry!(0x006a9520, fn_006a9520(Ptr) -> bool),
        entry!(0x006a9540, fn_006a9540(Ptr) -> Ptr),
        entry!(0x006a9560, fn_006a9560(Ptr) -> u8),
        entry!(
            0x006a9580,
            nav_mesh_search_flee_set_node_data(
                Ptr<NavMeshSearchFlee>,
                Ptr<AStarNodeNavMeshSearch>,
                Ptr,
                u16,
            )
        ),
        entry!(
            0x006a9620,
            nav_mesh_search_flee_get_node_connections(
                Ptr<NavMeshSearchFlee>,
                Ptr<AStarNodeNavMeshSearch>,
                Ptr,
            ) -> bool
        ),
        entry!(
            0x006a9a70,
            nav_mesh_search_flee_is_goal_node(
                Ptr<NavMeshSearchFlee>,
                Ptr<AStarNodeNavMeshSearch>,
            ) -> bool
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn f(e: &Engine, a: u32) -> f32 {
        e.mem.f32(a)
    }

    fn set3(e: &mut Engine, p: u32, v: [f32; 3]) {
        for (i, x) in v.iter().enumerate() {
            e.mem.set_f32(p + 4 * i as u32, *x);
        }
    }

    /// Every callee doubled (answering 0) and the vector helpers computed in
    /// `f32`; the constants the code reads are set.
    fn flee_engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x011f_4000,
            0x0101_6000,
            0x0106_b000,
            0x0102_0000,
            0x0101_2000,
            0x0101_1000,
            0x0103_2000,
        ] {
            e.map(page, 0x1000);
        }
        set3(&mut e, DEFAULT_POINT, [0.0, 0.0, 1.0]);
        e.set_global(MINIMAL_USE_DOOR_COST, 100.0f32);
        e.set_global(DIRECTION_WEIGHT, 1e6f64);
        e.set_global(GOAL_WEIGHT, 10.0f64);
        e.set_global(ZERO, 0.0f64);
        e.set_global(HALF, 0.5f64);
        e.set_global(MAX_HALF_DISTANCE, 256.0f32);
        for addr in [
            FORM_ID,
            GET_CENTER,
            CLAMP,
            REQUEST_ORIGIN,
            LOCATION_POSITION,
            REQUEST_MIN_FLEE_DISTANCE,
            REQUEST_AVOID_NODES,
            AVOID_NODES_CENTER,
            ADD_START_NODE,
            SEARCH,
            ARRAY_IS_EMPTY,
            PATH_LAST,
            NODE_CENTER,
            GET_MATCHING_EDGE,
            GET_EDGE_EXTRA_INFO,
            GET_TRIANGLE,
            TRIANGLE_HAS_FLAG,
            REQUEST_CANT_OPEN_DOORS,
            GET_DOOR_INFO,
            LOOKUP_FORM,
            DOOR_LOCK_CONSTRUCT,
            DOOR_LOCK_DESTRUCT,
            DOOR_LOCK_CAN_UNLOCK,
            REQUEST_LOCK_DATA,
            DOOR_BASE_FORM,
            IS_MINIMAL_USE,
            REQUEST_ACTOR_RADIUS,
            AVOID_COST_FOR_POINT,
            GET_OR_CREATE_NODE,
            LIST_ADD,
            HOLDER_CONSTRUCT,
            HOLDER_DESTRUCT,
            HOLDER_GET,
            GET_NAV_MESH_AND_TRIANGLE,
            SEARCH_WORD_2050,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(VECTOR_SUBTRACT, |e, a| {
            for i in 0..3 {
                let v = f(e, a[0] + 4 * i) - f(e, a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        e.register(VECTOR_LENGTH, |e, a| {
            let d: f32 = (0..3).map(|i| f(e, a[0] + 4 * i).powi(2)).sum();
            f64::from(d.sqrt()).into_ret()
        });
        e.register(LENGTH_SQUARED, |e, a| {
            let d: f32 = (0..3).map(|i| f(e, a[0] + 4 * i).powi(2)).sum();
            f64::from(d).into_ret()
        });
        e.register(FLAT_LENGTH, |e, a| {
            let d = f(e, a[0]).powi(2) + f(e, a[0] + 4).powi(2);
            f64::from(d.sqrt()).into_ret()
        });
        e.register(FLAT_LENGTH_SQUARED, |e, a| {
            let d = f(e, a[0]).powi(2) + f(e, a[0] + 4).powi(2);
            f64::from(d).into_ret()
        });
        e.register(NORMALIZE, |e, a| {
            let d: f32 = (0..3).map(|i| f(e, a[0] + 4 * i).powi(2)).sum();
            let d = d.sqrt();
            for i in 0..3 {
                let v = f(e, a[0] + 4 * i) / d;
                e.mem.set_f32(a[0] + 4 * i, v);
            }
            Ret::default()
        });
        e.register(DOT, |e, a| {
            let d: f32 = (0..3)
                .map(|i| f(e, a[0] + 4 * i) * f(e, a[1] + 4 * i))
                .sum();
            f64::from(d).into_ret()
        });
        e.register(MAX, |_, a| {
            let (x, y) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
            f64::from(if x <= y { y } else { x }).into_ret()
        });
        e.register(MIN, |_, a| {
            let (x, y) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
            f64::from(if y <= x { y } else { x }).into_ret()
        });
        e.register(FINITE, |_, a| {
            u32::from(f64::take(a, &mut 0).is_finite()).into_ret()
        });
        e.register(SETTING_VALUE, |_, _| 0x0101_6800u32.into_ret());
        e.set_global(0x0101_6800, 50.0f32);
        e
    }

    #[test]
    fn small_getters_and_node_key() {
        let mut e = flee_engine();
        e.register(FORM_ID, |_, _| 0x0012_3456u32.into_ret());
        e.call_log = Some(vec![]);
        let key = e.call(0x006a94f0, &args![0u32, 0xa000u32, 7u16]).u32();
        assert_eq!(key, 0x3456_0007);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, FORM_ID), vec![vec![0xa000]]);

        let location = e.mem.alloc(0x28);
        assert!(!e.call(0x006a9520, &args![location]).bool());
        e.mem.set_u8(location + 0x26, 2);
        assert!(e.call(0x006a9520, &args![location]).bool());
        e.mem.set_u8(location + 0x26, 0xfd);
        assert!(!e.call(0x006a9520, &args![location]).bool());

        assert_eq!(e.call(0x006a9540, &args![0x1000u32]).u32(), 0x1034);
        let request = e.mem.alloc(0xb4);
        e.mem.set_u8(request + 0x9b, 1);
        assert_eq!(e.call(0x006a9560, &args![request]).u8(), 1);
    }

    #[test]
    fn set_node_data_measures_from_the_avoid_centre() {
        let mut e = flee_engine();
        e.register(GET_CENTER, |e, a| {
            e.mem.set_f32(a[1], 3.0);
            e.mem.set_f32(a[1] + 4, 4.0);
            e.mem.set_u32(a[1] + 8, 0x7f80_0001);
            Ret::default()
        });
        e.register(VECTOR_LENGTH, |_, _| 30.0f64.into_ret());
        let this: Ptr<NavMeshSearchFlee> = e.new_object();
        e.set(this, NavMeshSearchFlee::fDistanceUpperBound, 100.0);
        let node: Ptr<AStarNodeNavMeshSearch> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x006a9580, &args![this, node, 0xa000u32, 7u16]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            e.get(node, AStarNodeNavMeshSearch::pNavMesh),
            Ptr::new(0xa000)
        );
        assert_eq!(e.get(node, AStarNodeNavMeshSearch::sTriangle), 7);
        assert_eq!(e.get(node, AStarNodeNavMeshSearch::CenterY), 4.0);
        assert_eq!(e.mem.u32(node.addr() + 0x18), 0x7fc0_0001);
        assert_eq!(calls(&log, GET_CENTER)[0][2], 7);
        let subtract = &calls(&log, VECTOR_SUBTRACT)[0];
        assert_eq!(subtract[2], this.addr() + 0x2070);
        let clamp = &calls(&log, CLAMP)[0];
        assert_eq!(&clamp[1..], &[0, 100.0f32.to_bits()]);
        assert_eq!(e.get(node, AStarNodeNavMeshSearch::fHeuristic), 70.0);
    }

    /// A search with its request, a node (cost 2.0, mesh `a000`, triangle
    /// 5) and the doubles of a neighbour (mesh `c000`, triangle 9, centre
    /// (3, 4, 0)) over edge 1 only.
    fn connection_setup(e: &mut Engine) -> (Ptr<NavMeshSearchFlee>, Ptr<AStarNodeNavMeshSearch>) {
        let this: Ptr<NavMeshSearchFlee> = e.new_object();
        e.set(this, NavMeshSearchFlee::pCurrentRequest, Ptr::new(0x5000));
        let node: Ptr<AStarNodeNavMeshSearch> = e.new_object();
        e.mem.set_f32(node.addr(), 2.0);
        e.set(node, AStarNodeNavMeshSearch::pNavMesh, Ptr::new(0xa000));
        e.set(node, AStarNodeNavMeshSearch::sTriangle, 5);
        e.register(GET_MATCHING_EDGE, |e, a| {
            if a[2] != 1 {
                return 0u32.into_ret();
            }
            e.mem.set_u32(a[3], 0xc000);
            e.mem.set_u16(a[4], 9);
            1u32.into_ret()
        });
        e.register(GET_CENTER, |e, a| {
            for (i, v) in [3.0f32, 4.0, 0.0].iter().enumerate() {
                e.mem.set_f32(a[1] + 4 * i as u32, *v);
            }
            Ret::default()
        });
        e.register(REQUEST_ACTOR_RADIUS, |_, _| 10.0f64.into_ret());
        e.register(FORM_ID, |_, _| 0x0001_0002u32.into_ret());
        e.register(GET_OR_CREATE_NODE, |e, a| {
            let node = e.mem.alloc(0x3c);
            e.mem.set_u32(a[3], node);
            0u32.into_ret()
        });
        (this, node)
    }

    #[test]
    fn get_node_connections_scores_the_neighbour() {
        let mut e = flee_engine();
        let (this, node) = connection_setup(&mut e);
        // Avoid nodes: the centre is at (-10, 0, 0), the cost 0.5.
        e.register(REQUEST_AVOID_NODES, |_, _| 0xb000u32.into_ret());
        e.register(AVOID_COST_FOR_POINT, |_, _| 0.5f64.into_ret());
        set3(&mut e, this.addr() + 0x2070, [-10.0, 0.0, 0.0]);
        // Favouring a goal at (3, 8, 0).
        e.set(this, NavMeshSearchFlee::bFavorGoingTowardsGoal, true);
        set3(&mut e, this.addr() + 0x2084, [3.0, 8.0, 0.0]);
        e.call_log = Some(vec![]);
        assert!(e.call(0x006a9620, &args![this, node, 0x6000u32]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, GET_MATCHING_EDGE).len(), 3);
        // 5 * 1.0 + 2.0, + 0.5, + 1e6 * max(0, -0.6), + 10 * 4.
        let create = calls(&log, GET_OR_CREATE_NODE);
        assert_eq!(create.len(), 1);
        assert_eq!(
            &create[0][..3],
            &[this.addr(), 0x0002_0009, 47.5f32.to_bits()]
        );
        let slot = create[0][3];
        let added = Ptr::<AStarNodeNavMeshSearch>::new(e.mem.u32(slot));
        assert_eq!(calls(&log, LIST_ADD), vec![vec![0x6000, slot]]);
        assert_eq!(
            e.get(added, AStarNodeNavMeshSearch::pNavMesh),
            Ptr::new(0xc000)
        );
        assert_eq!(
            e.get(added, AStarNodeNavMeshSearch::ParentLink_pNavMesh),
            Ptr::new(0xa000)
        );
        assert_eq!(
            e.get(added, AStarNodeNavMeshSearch::ParentLink_sTriangle),
            5
        );
        assert_eq!(
            e.get(added, AStarNodeNavMeshSearch::ParentLink_iEdgeIndex),
            1
        );
        assert_eq!(calls(&log, NORMALIZE).len(), 2);

        // A node that exists and is not better is not added.
        e.register(GET_OR_CREATE_NODE, |_, _| 2u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x006a9620, &args![this, node, 0x6000u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, LIST_ADD).is_empty());
    }

    #[test]
    fn get_node_connections_resets_an_infinite_cost_and_checks_doors() {
        let mut e = flee_engine();
        let (this, node) = connection_setup(&mut e);
        e.mem.set_f32(node.addr(), f32::INFINITY);
        // Door triangle the request cannot open: skipped.
        e.register(TRIANGLE_HAS_FLAG, |_, a| {
            (a[1] == TRIANGLE_DOOR_FLAG).into_ret()
        });
        e.register(REQUEST_CANT_OPEN_DOORS, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x006a9620, &args![this, node, 0x6000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.f32(node.addr()), 0.0);
        assert!(calls(&log, GET_OR_CREATE_NODE).is_empty());

        // A door that cannot be unlocked: skipped, the lock destroyed.
        e.register(REQUEST_CANT_OPEN_DOORS, |_, _| 0u32.into_ret());
        e.register(GET_DOOR_INFO, |e, _| {
            let info = e.mem.alloc(4);
            e.mem.set_u32(info, 0x1234);
            info.into_ret()
        });
        e.register(LOOKUP_FORM, |_, _| 0xd000u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x006a9620, &args![this, node, 0x6000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, LOOKUP_FORM), vec![vec![0x1234]]);
        assert_eq!(calls(&log, DOOR_LOCK_DESTRUCT).len(), 1);
        assert!(calls(&log, GET_OR_CREATE_NODE).is_empty());

        // Unlockable minimal-use door: cost 100.
        e.register(DOOR_LOCK_CAN_UNLOCK, |_, _| 1u32.into_ret());
        e.register(IS_MINIMAL_USE, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x006a9620, &args![this, node, 0x6000u32]);
        let log = e.call_log.take().unwrap();
        let create = calls(&log, GET_OR_CREATE_NODE);
        assert_eq!(create[0][2], 500.0f32.to_bits());
        assert_eq!(calls(&log, DOOR_LOCK_DESTRUCT).len(), 1);

        // The setting below the actor radius and a blocked triangle: skipped.
        e.set_global(0x0101_6800, 5.0f32);
        e.register(TRIANGLE_HAS_FLAG, |_, a| {
            (a[1] == TRIANGLE_BLOCKED_FLAG).into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x006a9620, &args![this, node, 0x6000u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, GET_OR_CREATE_NODE).is_empty());
    }

    /// A search whose request (`5000`) has its origin at (0, 0, 0), minimum
    /// flee distance 10 and a node centred at (`x`, 0, 0).
    fn goal_setup(e: &mut Engine, x: f32) -> (Ptr<NavMeshSearchFlee>, Ptr<AStarNodeNavMeshSearch>) {
        let this: Ptr<NavMeshSearchFlee> = e.new_object();
        e.map(0x5000, 0x100);
        e.set(this, NavMeshSearchFlee::pCurrentRequest, Ptr::new(0x5000));
        let node: Ptr<AStarNodeNavMeshSearch> = e.new_object();
        e.register_double(NODE_CENTER, move |e, a| {
            for (i, v) in [x, 0.0f32, 0.0].iter().enumerate() {
                e.mem.set_f32(a[1] + 4 * i as u32, *v);
            }
            Ret::default()
        });
        e.register(LOCATION_POSITION, |e, a| {
            for i in 0..3 {
                e.mem.set_f32(a[1] + 4 * i, 0.0);
            }
            a[1].into_ret()
        });
        e.register(REQUEST_MIN_FLEE_DISTANCE, |_, _| 10.0f64.into_ret());
        (this, node)
    }

    #[test]
    fn is_goal_node_cases() {
        let mut e = flee_engine();
        let (this, node) = goal_setup(&mut e, 12.0);
        assert!(e.call(0x006a9a70, &args![this, node]).bool());
        // The node at +0x2050 is never a goal.
        let start = node.addr();
        e.register_double(SEARCH_WORD_2050, move |_, _| start.into_ret());
        assert!(!e.call(0x006a9a70, &args![this, node]).bool());
        e.register(SEARCH_WORD_2050, |_, _| Ret::default());
        // A positive avoid cost fails.
        e.register(REQUEST_AVOID_NODES, |_, _| 0xb000u32.into_ret());
        e.register(AVOID_COST_FOR_POINT, |_, _| 0.25f64.into_ret());
        assert!(!e.call(0x006a9a70, &args![this, node]).bool());
        e.register(REQUEST_AVOID_NODES, |_, _| Ret::default());
        // Closer than the minimum flee distance.
        let (this, node) = goal_setup(&mut e, 9.0);
        assert!(!e.call(0x006a9a70, &args![this, node]).bool());
    }

    #[test]
    fn is_goal_node_favouring_a_goal() {
        let mut e = flee_engine();
        let (this, node) = goal_setup(&mut e, 12.0);
        e.set(this, NavMeshSearchFlee::bFavorGoingTowardsGoal, true);
        // The goal at (20, 0, 0): the node is nearer to it than the origin.
        set3(&mut e, this.addr() + 0x2084, [20.0, 0.0, 0.0]);
        e.call_log = Some(vec![]);
        assert!(e.call(0x006a9a70, &args![this, node]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, GET_NAV_MESH_AND_TRIANGLE)[0][0], 0x5034);
        assert_eq!(calls(&log, HOLDER_DESTRUCT).len(), 1);
        // The goal behind the origin: the node is farther from it.
        set3(&mut e, this.addr() + 0x2084, [-20.0, 0.0, 0.0]);
        assert!(!e.call(0x006a9a70, &args![this, node]).bool());
        // The node is the destination's navmesh and triangle.
        e.set(node, AStarNodeNavMeshSearch::pNavMesh, Ptr::new(0xa000));
        e.set(node, AStarNodeNavMeshSearch::sTriangle, 3);
        e.register(GET_NAV_MESH_AND_TRIANGLE, |e, a| {
            e.mem.set_u32(a[1], 0xa000);
            e.mem.set_u16(a[2], 3);
            1u32.into_ret()
        });
        e.register(NODE_DATA, |_, a| (a[0] + 8).into_ret());
        e.register(HOLDER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.call_log = Some(vec![]);
        assert!(e.call(0x006a9a70, &args![this, node]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, HOLDER_DESTRUCT).len(), 1);
        assert!(calls(&log, NODE_CENTER).is_empty());
    }

    #[test]
    fn build_node_path() {
        let mut e = flee_engine();
        let (this, _) = goal_setup(&mut e, 0.0);
        e.register(ADD_START_NODE, |e, _| e.mem.alloc(0x3c).into_ret());
        let start = e.mem.alloc(0x14);
        e.mem.set_u32(start, 0xa000);
        e.mem.set_u16(start + 4, 4);
        e.mem.set_u8(0x5000 + 0x9b, 1);
        // The destination has flag bit 1: no goal.
        e.mem.set_u8(0x5034 + 0x26, 2);
        e.register(SEARCH, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        let found = e.call(0x006a92e0, &args![this, 0x5000u32, start, 0x7000u32, 33u32]);
        assert!(found.bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(
            e.get(this, NavMeshSearchFlee::pCurrentRequest),
            Ptr::new(0x5000)
        );
        assert_eq!(e.mem.u32(this.addr() + 0x205c), 0xa000);
        assert_eq!(e.get(this, NavMeshSearchFlee::fDistanceUpperBound), 10.0);
        assert_eq!(f(&e, this.addr() + 0x2078), 1.0);
        assert!(!e.get(this, NavMeshSearchFlee::bFavorGoingTowardsGoal));
        assert_eq!(
            calls(&log, ADD_START_NODE),
            vec![vec![this.addr(), 4, start, 0]]
        );
        assert_eq!(calls(&log, SEARCH), vec![vec![this.addr(), 0x7000, 33, 1]]);
        assert!(calls(&log, AVOID_NODES_CENTER).is_empty());

        // No flag: the destination's position becomes the goal; the search
        // fails, but the last node is 300 away (more than min(5, 256)).
        e.mem.set_u8(0x5034 + 0x26, 0);
        e.register(LOCATION_POSITION, |e, a| {
            let v = if a[0] == 0x5034 { 7.0 } else { 0.0 };
            for i in 0..3 {
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        e.register(REQUEST_ORIGIN, |_, a| (a[0] + 0xc).into_ret());
        e.register(SEARCH, |_, _| 0u32.into_ret());
        e.register(PATH_LAST, |_, _| 0x9990u32.into_ret());
        e.register(NODE_CENTER, |e, a| {
            for (i, v) in [300.0f32, 0.0, 0.0].iter().enumerate() {
                e.mem.set_f32(a[1] + 4 * i as u32, *v);
            }
            Ret::default()
        });
        e.register(REQUEST_AVOID_NODES, |_, _| 0xb000u32.into_ret());
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x006a92e0, &args![this, 0x5000u32, start, 0x7000u32, 33u32])
            .bool());
        let log = e.call_log.take().unwrap();
        assert!(e.get(this, NavMeshSearchFlee::bFavorGoingTowardsGoal));
        assert_eq!(f(&e, this.addr() + 0x2084), 7.0);
        assert_eq!(
            calls(&log, MIN),
            vec![vec![5.0f32.to_bits(), 256.0f32.to_bits()]]
        );
        let centers = calls(&log, NODE_CENTER);
        assert_eq!((centers.len(), centers[0][0]), (1, 0x9990));
        let center = &calls(&log, AVOID_NODES_CENTER)[0];
        assert_eq!((center[0], center[2]), (0xb000, this.addr() + 0x2070));

        // The last node within reach: still a failure.
        e.register(NODE_CENTER, |e, a| {
            for i in 0..3 {
                e.mem.set_f32(a[1] + 4 * i, 1.0);
            }
            Ret::default()
        });
        assert!(!e
            .call(0x006a92e0, &args![this, 0x5000u32, start, 0x7000u32, 33u32])
            .bool());
        // An empty path is a failure without the distance test.
        e.register(ARRAY_IS_EMPTY, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x006a92e0, &args![this, 0x5000u32, start, 0x7000u32, 33u32])
            .bool());
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, PATH_LAST).is_empty());
    }
}
