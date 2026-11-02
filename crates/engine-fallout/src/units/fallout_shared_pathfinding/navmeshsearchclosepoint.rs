//! `fallout shared/pathfinding/navmeshsearchclosepoint.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `NavMeshSearchClosePoint` is the A* search that looks for the point
//! closest to a goal: an `AStarSearch<NavMeshSearchNode, NavMeshSearchLink>`
//! (vtable slot `+4` is `GetNodeConnections`, slot `+8` is `IsGoalNode`) plus
//! the request it serves and the search limits. On PC the members after the
//! base sit 4 bytes lower than in the Xbox PDB (`pCurrentRequest` is at
//! `+0x2058`, the PDB says `+0x205C`).
//!
//! The nodes are `AStarNode<NavMeshSearchNode, NavMeshSearchLink>` (0x3C
//! bytes): the cost, the heuristic, the node data (navmesh, triangle and
//! centre) and the link to the parent (navmesh, triangle and edge).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00558220(mesh, out, triangle)`: `NavMesh::GetCenter` (writes 3 floats).
const GET_CENTER: u32 = 0x0055_8220;
/// `00439ef0(this, out, other)`: `out = this - other` for 3-float vectors.
const VECTOR_SUBTRACT: u32 = 0x0043_9ef0;
/// `00457990(vector)`: returns a length in ST0 (`this` = the 3-float vector).
const VECTOR_LENGTH: u32 = 0x0045_7990;
/// `005330e0(&value, minimum, maximum)`: clamps the float in place (cdecl).
const CLAMP: u32 = 0x0053_30e0;
/// `006a6eb0(node data, out)`: writes the node's centre (3 floats).
const NODE_CENTER: u32 = 0x006a_6eb0;
/// `00487f50()`: a random number.
const RANDOM: u32 = 0x0048_7f50;
/// `0068f460(mesh, triangle, edge, &mesh, &triangle, &other)`:
/// `NavMesh::GetMatchingEdge_ov3` (map name): finds the triangle across the
/// edge. The three outputs are the neighbouring navmesh, the neighbouring
/// triangle and a third word; returns whether there is one.
const GET_MATCHING_EDGE: u32 = 0x0068_f460;
/// `0068f230(mesh, triangle, edge)`: `NavMesh::GetEdgeExtraInfo` (map name):
/// the extra info of the edge, or 0; its first word is the type.
const GET_EDGE_EXTRA_INFO: u32 = 0x0068_f230;
/// `005582f0(mesh, triangle)`: the 16-byte record of the triangle.
const GET_TRIANGLE: u32 = 0x0055_82f0;
/// `00691140(record, mask)`: whether `mask` meets the record's flags (`+0xC`).
const TRIANGLE_HAS_FLAG: u32 = 0x0069_1140;
/// Triangle flag that marks a closed door.
const TRIANGLE_DOOR_FLAG: u32 = 0x1000;
/// Triangle flag the actor radius check tests.
const TRIANGLE_BLOCKED_FLAG: u32 = 0x10;
/// `0073cb80(request)`: reads the byte at `+0x98` (`bCantOpenDoors`).
const REQUEST_CANT_OPEN_DOORS: u32 = 0x0073_cb80;
/// `00699af0(mesh, triangle)`: `NavMesh::GetClosedDoorInfoForTriangle`
/// (map name): the address of the door's form id.
const GET_DOOR_INFO: u32 = 0x0069_9af0;
/// `004839c0(form id)`: the form with that id, or 0.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `00501d20(lock, door)`: `DoorLock::DoorLock`.
const DOOR_LOCK_CONSTRUCT: u32 = 0x0050_1d20;
/// `00502450(lock, actor lock data, door, &flag)`:
/// `DoorLock::CanActorUnlockDoorLock` (map name).
const DOOR_LOCK_CAN_UNLOCK: u32 = 0x0050_2450;
/// `00501e50(lock)`: `DoorLock` destructor.
const DOOR_LOCK_DESTRUCT: u32 = 0x0050_1e50;
/// `006c9900(request)`: the address of the request's lock data (`+0x5C`).
const REQUEST_LOCK_DATA: u32 = 0x006c_9900;
/// `007af430(door)`: the word at `+0x20` (the door's base form).
const DOOR_BASE_FORM: u32 = 0x007a_f430;
/// `00518000(base form)`: whether bit 3 of the byte at `+0x84` is set
/// (`TESObjectDOOR::IsMinimalUse`, map name).
const IS_MINIMAL_USE: u32 = 0x0051_8000;
/// The cost of a door the actor may open when it is minimal-use (`100.0f`).
const MINIMAL_USE_DOOR_COST: u32 = 0x0101_6410;
/// `0069ef80(request)`: the float at `+0x68` of the request
/// (`fActorRadius`), in ST0.
const REQUEST_ACTOR_RADIUS: u32 = 0x0069_ef80;
/// `00403e20(ECX)`: the address of the float inside a settings object (`+4`).
const SETTING_VALUE: u32 = 0x0040_3e20;
/// The settings object whose float `GetNodeConnections` compares with the
/// actor radius.
const RADIUS_SETTING: u32 = 0x011d_6ef0;
/// `006a94f0(this, mesh, triangle)`: the node key (the triangle in the low
/// 16 bits, the navmesh's key in the high 16).
const NODE_KEY: u32 = 0x006a_94f0;
/// `006a7e00(this, key, cost, &node)`: `GetOrCreateAStarNode` (map name):
/// 0 when it created the node, 1 when it exists and is better, else 2 or
/// more.
const GET_OR_CREATE_NODE: u32 = 0x006a_7e00;
/// `007cb2e0(list, &item)`: appends the word at `item` to the list.
const LIST_ADD: u32 = 0x007c_b2e0;
/// `0048d150(request)`: the address of the request's origin (`+0xC`).
const REQUEST_ORIGIN: u32 = 0x0048_d150;
/// `005c3420(origin, out)`: copies the origin's three position floats.
const ORIGIN_POSITION: u32 = 0x005c_3420;
/// `00589850(vector)`: the length of the first two components, in ST0.
const FLAT_LENGTH: u32 = 0x0058_9850;
/// `0068efb0(mesh)`: the cell or world the navmesh belongs to.
const MESH_OWNER: u32 = 0x0068_efb0;
/// `006dcf20(this, position, owner)`: `PathingLocation` constructor for a
/// position in the owner (the local object is 0x28 bytes).
const PATHING_LOCATION_CONSTRUCT: u32 = 0x006d_cf20;
/// `004ff7e0(this)`: `PathingLocation` destructor body.
const PATHING_LOCATION_DESTRUCT: u32 = 0x004f_f7e0;
/// `006d9480(location, float)` (cdecl): whether the location is valid for an
/// actor of that radius.
const LOCATION_IS_VALID: u32 = 0x006d_9480;
/// `0069e390(request)`: the request's avoid node array (`+0x94`), or 0.
const REQUEST_AVOID_NODES: u32 = 0x0069_e390;
/// `006dc990(array, &point)`: `PathingAvoidNodeArray::GetCostForPoint`
/// (map name for the folded body), in ST0.
const AVOID_COST_FOR_POINT: u32 = 0x006d_c990;
/// The double `1.0`.
const ONE: u32 = 0x0101_2070;
/// The double `0.0`.
const ZERO: u32 = 0x0101_2060;

/// Offset of the `NiPoint3` the search measures its heuristic from.
const AVOID_NODE_CENTER: u32 = 0x208c;

layout! {
    /// `NavMeshSearchClosePoint` (Xbox PDB), 0x209C bytes on PC; only the
    /// members this unit touches (PC offsets).
    pub struct NavMeshSearchClosePoint: 0x209C {
        /// `pCurrentRequest` (Xbox PDB): `const PathingRequestClosePoint *`.
        0x2058 pCurrentRequest: Ptr,
        /// `fMinDistance` (Xbox PDB).
        0x2070 fMinDistance: f32,
        /// `fDistanceUpperBound` (Xbox PDB).
        0x2098 fDistanceUpperBound: f32,
    }

    /// `AStarNode<NavMeshSearchNode, NavMeshSearchLink>` (Xbox PDB), 0x3C
    /// bytes; only the members this unit touches.
    pub struct AStarNodeNavMeshSearch: 0x3C {
        /// `fHeuristic` (Xbox PDB).
        0x04 fHeuristic: f32,
        /// `NodeData.pNavMesh` (Xbox PDB).
        0x08 pNavMesh: Ptr,
        /// `NodeData.sTriangle` (Xbox PDB).
        0x0C sTriangle: u16,
        /// `NodeData.CenterX` (Xbox PDB).
        0x10 CenterX: f32,
        /// `NodeData.CenterY` (Xbox PDB).
        0x14 CenterY: f32,
        /// `NodeData.CenterZ` (Xbox PDB).
        0x18 CenterZ: f32,
        /// `ParentLink.pNavMesh` (Xbox PDB).
        0x1C ParentLink_pNavMesh: Ptr,
        /// `ParentLink.sTriangle` (Xbox PDB).
        0x20 ParentLink_sTriangle: u16,
        /// `ParentLink.iEdgeIndex` (Xbox PDB).
        0x24 ParentLink_iEdgeIndex: u32,
    }
}

/// A float the exe copies through `FLD` and `FSTP`: a signaling NaN comes
/// out quiet.
fn quiet(value: f32) -> f32 {
    let bits = value.to_bits();
    if bits & 0x7f80_0000 == 0x7f80_0000 && bits & 0x007f_ffff != 0 {
        f32::from_bits(bits | 0x0040_0000)
    } else {
        value
    }
}

// Translated from 006a8d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshSearchClosePoint::SetNodeData` (Xbox PDB): fills a new search node
/// for `triangle` of `mesh`: the mesh and triangle, the triangle's centre
/// (`00558220`), and the heuristic: the upper bound minus the length
/// (`00457990`) of the vector from the search's goal point (`+0x208C`) to
/// the centre, clamped to between 0 and the upper bound (`005330e0`).
pub fn nav_mesh_search_close_point_set_node_data(
    e: &mut Engine,
    this: Ptr<NavMeshSearchClosePoint>,
    node: Ptr<AStarNodeNavMeshSearch>,
    mesh: Ptr,
    triangle: u16,
) {
    e.set(node, AStarNodeNavMeshSearch::pNavMesh, mesh);
    e.set(node, AStarNodeNavMeshSearch::sTriangle, triangle);
    let scratch = e.mem.alloc(0x20);
    let center = scratch;
    let offset = scratch + 0xc;
    let heuristic = scratch + 0x18;
    e.call(GET_CENTER, &args![mesh, center, u32::from(triangle)]);
    for (field, delta) in [
        (AStarNodeNavMeshSearch::CenterX, 0u32),
        (AStarNodeNavMeshSearch::CenterY, 4),
        (AStarNodeNavMeshSearch::CenterZ, 8),
    ] {
        let value = e.mem.f32(center + delta);
        e.set(node, field, quiet(value));
    }
    let goal = this.addr().wrapping_add(AVOID_NODE_CENTER);
    e.call(VECTOR_SUBTRACT, &args![center, offset, goal]);
    let length = e.call(VECTOR_LENGTH, &args![offset]).f64();
    let upper_bound = e.get(this, NavMeshSearchClosePoint::fDistanceUpperBound);
    e.mem
        .set_f32(heuristic, (f64::from(upper_bound) - length) as f32);
    e.call(CLAMP, &args![heuristic, 0.0f32, quiet(upper_bound)]);
    let value = e.mem.f32(heuristic);
    e.set(node, AStarNodeNavMeshSearch::fHeuristic, quiet(value));
    e.mem.free(scratch);
}

/// The scratch words `GetNodeConnections` keeps on the stack for the edge it
/// is looking at: the new node's address, the neighbouring navmesh, a third
/// output word and the neighbouring triangle.
struct EdgeScratch {
    node: u32,
    mesh: u32,
    other: u32,
    triangle: u32,
}

impl EdgeScratch {
    fn new(base: u32) -> Self {
        EdgeScratch {
            node: base,
            mesh: base + 4,
            other: base + 8,
            triangle: base + 12,
        }
    }
}

/// Expands one edge of the node: the body of the loop of `GetNodeConnections`.
#[allow(clippy::too_many_arguments)]
fn expand_edge(
    e: &mut Engine,
    this: Ptr<NavMeshSearchClosePoint>,
    node: Ptr<AStarNodeNavMeshSearch>,
    list: Ptr,
    request: Ptr,
    edge: u32,
    scratch: &EdgeScratch,
    lock: u32,
    flag: u32,
) {
    let node_mesh = e.get(node, AStarNodeNavMeshSearch::pNavMesh);
    let node_triangle = u32::from(e.get(node, AStarNodeNavMeshSearch::sTriangle));
    let found = e
        .call(
            GET_MATCHING_EDGE,
            &args![
                node_mesh,
                node_triangle,
                edge,
                scratch.mesh,
                scratch.triangle,
                scratch.other
            ],
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
    let mut cost = 1.0f32;
    let neighbour_mesh = e.mem.u32(scratch.mesh);
    let neighbour_triangle = u32::from(e.mem.u16(scratch.triangle));
    let record = e
        .call(GET_TRIANGLE, &args![neighbour_mesh, neighbour_triangle])
        .u32();
    if e.call(TRIANGLE_HAS_FLAG, &args![record, TRIANGLE_DOOR_FLAG])
        .bool()
    {
        if e.call(REQUEST_CANT_OPEN_DOORS, &args![request]).bool() {
            return;
        }
        let door_info = e
            .call(GET_DOOR_INFO, &args![neighbour_mesh, neighbour_triangle])
            .u32();
        let door_id = e.mem.u32(door_info);
        let door = e.call(LOOKUP_FORM, &args![door_id]).u32();
        if door != 0 {
            e.call(DOOR_LOCK_CONSTRUCT, &args![lock, door]);
            let lock_data = e.call(REQUEST_LOCK_DATA, &args![request]).u32();
            let can_unlock = e
                .call(DOOR_LOCK_CAN_UNLOCK, &args![lock, lock_data, door, flag])
                .bool();
            if !can_unlock {
                e.call(DOOR_LOCK_DESTRUCT, &args![lock]);
                return;
            }
            let base_form = e.call(DOOR_BASE_FORM, &args![door]).u32();
            if e.call(IS_MINIMAL_USE, &args![base_form]).bool() {
                cost = quiet(e.global::<f32>(MINIMAL_USE_DOOR_COST));
            }
            e.call(DOOR_LOCK_DESTRUCT, &args![lock]);
        }
    }
    let actor_radius = e.call(REQUEST_ACTOR_RADIUS, &args![request]).f64();
    let setting = e.call(SETTING_VALUE, &args![RADIUS_SETTING]).u32();
    if f64::from(e.mem.f32(setting)) < actor_radius {
        let record = e
            .call(GET_TRIANGLE, &args![neighbour_mesh, neighbour_triangle])
            .u32();
        if e.call(TRIANGLE_HAS_FLAG, &args![record, TRIANGLE_BLOCKED_FLAG])
            .bool()
        {
            return;
        }
    }
    let mut heuristic = 0.0f32;
    if f64::from(cost) != e.global::<f64>(ONE) {
        heuristic = quiet(cost);
    }
    let key = e
        .call(NODE_KEY, &args![this, neighbour_mesh, neighbour_triangle])
        .u32();
    let status = e
        .call(
            GET_OR_CREATE_NODE,
            &args![this, key, heuristic, scratch.node],
        )
        .i32();
    if status == 0 {
        let new_node = e.mem.u32(scratch.node);
        nav_mesh_search_close_point_set_node_data(
            e,
            this,
            Ptr::new(new_node),
            Ptr::new(neighbour_mesh),
            neighbour_triangle as u16,
        );
    } else if status != 1 {
        return;
    }
    let new_node = Ptr::<AStarNodeNavMeshSearch>::new(e.mem.u32(scratch.node));
    e.set(
        new_node,
        AStarNodeNavMeshSearch::ParentLink_pNavMesh,
        node_mesh,
    );
    e.set(
        new_node,
        AStarNodeNavMeshSearch::ParentLink_sTriangle,
        node_triangle as u16,
    );
    e.set(
        new_node,
        AStarNodeNavMeshSearch::ParentLink_iEdgeIndex,
        edge,
    );
    e.call(LIST_ADD, &args![list, scratch.node]);
}

// Translated from 006a8dc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006a8dc0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// `NavMeshSearchClosePoint::GetNodeConnections` (Xbox PDB): appends to
/// `list` the search nodes reached over the three edges of the triangle of
/// `node`, visiting the edges in a random order (a random number modulo 6
/// picks the first edge, `n / 2`, and the step, `n % 2 + 1`, mod 3).
/// For each edge with a neighbour (`0068f460`), unless the edge's extra info
/// has type 1, the cost is 1.0; if the neighbour triangle has flag 0x1000
/// (a closed door) the edge is skipped when the request cannot open doors
/// (`0073cb80`) or, when the triangle's door form exists, when the actor
/// cannot unlock its `DoorLock` (`00502450`), and the cost is the float at
/// `01016410` for a minimal-use door. The edge is also skipped when the
/// setting float at `011d6ef0` is below the actor radius (`0069ef80`) and
/// the triangle has flag 0x10. Then the node is created or found (`006a7e00`;
/// the heuristic passed is 0.0 for cost 1.0, else the cost), set up with
/// `SetNodeData` when new, given the parent link (this node's navmesh and
/// triangle, the edge) and appended to the list (`007cb2e0`), unless the
/// existing node is not better. Always returns true. The call to `006a6eb0`
/// on the node data at the start only fills a local the function never reads
/// again. The exception-unwinding frame (and the `DoorLock` temporary's
/// unwinding state) is not translated.
pub fn nav_mesh_search_close_point_get_node_connections(
    e: &mut Engine,
    this: Ptr<NavMeshSearchClosePoint>,
    node: Ptr<AStarNodeNavMeshSearch>,
    list: Ptr,
) -> bool {
    let position = e.mem.alloc(0xc);
    e.call(NODE_CENTER, &args![node.addr().wrapping_add(8), position]);
    let random = e.call(RANDOM, &args![]).u32() % 6;
    let start = random / 2;
    let step = (random & 1) + 1;
    let request = e.get(this, NavMeshSearchClosePoint::pCurrentRequest);
    let block = e.mem.alloc(0x30);
    let scratch = EdgeScratch::new(block);
    let lock = block + 0x10;
    let flag = block + 0x2c;
    let mut edge = start;
    for _ in 0..3 {
        expand_edge(e, this, node, list, request, edge, &scratch, lock, flag);
        edge = (edge + step) % 3;
    }
    e.mem.free(block);
    e.mem.free(position);
    true
}

// Translated from 006a90a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006a90a0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// `NavMeshSearchClosePoint::IsGoalNode` (Xbox PDB): whether `node` is an
/// acceptable result. False when the flat distance (`00589850`) between the
/// request's origin position and the node's centre is below the minimum
/// distance (`+0x2070`). Otherwise a `PathingLocation` is built at the node
/// centre in its navmesh's owner (`006dcf20`); false when it is not valid
/// for the actor radius (`006d9480`). Else the cost of the point among the
/// request's avoid nodes (`006dc990`, 0 without an avoid node array) must be
/// at most the double `0.0` (a NaN cost fails). The exception-unwinding
/// frame is not translated.
pub fn nav_mesh_search_close_point_is_goal_node(
    e: &mut Engine,
    this: Ptr<NavMeshSearchClosePoint>,
    node: Ptr<AStarNodeNavMeshSearch>,
) -> bool {
    let request = e.get(this, NavMeshSearchClosePoint::pCurrentRequest);
    let scratch = e.mem.alloc(0x30);
    let center = scratch;
    let origin_position = scratch + 0xc;
    let difference = scratch + 0x18;
    e.call(NODE_CENTER, &args![node.addr().wrapping_add(8), center]);
    let origin = e.call(REQUEST_ORIGIN, &args![request]).u32();
    e.call(ORIGIN_POSITION, &args![origin, origin_position]);
    e.call(VECTOR_SUBTRACT, &args![origin_position, difference, center]);
    let distance = e.call(FLAT_LENGTH, &args![difference]).f32();
    let minimum = e.get(this, NavMeshSearchClosePoint::fMinDistance);
    let result = if minimum > distance {
        false
    } else {
        let mesh = e.get(node, AStarNodeNavMeshSearch::pNavMesh);
        let owner = e.call(MESH_OWNER, &args![mesh]).u32();
        e.with_stack(0x28, |e, location| {
            e.call(PATHING_LOCATION_CONSTRUCT, &args![location, center, owner]);
            let radius = e.call(REQUEST_ACTOR_RADIUS, &args![request]).f32();
            let valid = e.call(LOCATION_IS_VALID, &args![location, radius]).bool();
            let result = if valid {
                let mut cost = 0.0f32;
                if e.call(REQUEST_AVOID_NODES, &args![request]).u32() != 0 {
                    let avoid_nodes = e.call(REQUEST_AVOID_NODES, &args![request]).u32();
                    cost = e
                        .call(AVOID_COST_FOR_POINT, &args![avoid_nodes, center])
                        .f32();
                }
                f64::from(cost) <= e.global::<f64>(ZERO)
            } else {
                false
            };
            e.call(PATHING_LOCATION_DESTRUCT, &args![location]);
            result
        })
    };
    e.mem.free(scratch);
    result
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x006a8d20,
            nav_mesh_search_close_point_set_node_data(
                Ptr<NavMeshSearchClosePoint>,
                Ptr<AStarNodeNavMeshSearch>,
                Ptr,
                u16,
            )
        ),
        entry!(
            0x006a8dc0,
            nav_mesh_search_close_point_get_node_connections(
                Ptr<NavMeshSearchClosePoint>,
                Ptr<AStarNodeNavMeshSearch>,
                Ptr,
            ) -> bool
        ),
        entry!(
            0x006a90a0,
            nav_mesh_search_close_point_is_goal_node(
                Ptr<NavMeshSearchClosePoint>,
                Ptr<AStarNodeNavMeshSearch>,
            ) -> bool
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log.clone().unwrap()
    }

    fn silent(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    #[test]
    fn set_node_data_measures_from_the_goal_point() {
        let mut e = Engine::new();
        e.register(GET_CENTER, |e, a| {
            e.mem.set_f32(a[1], 1.0);
            e.mem.set_f32(a[1] + 4, 2.0);
            // A signaling NaN comes out of FLD/FSTP quiet.
            e.mem.set_u32(a[1] + 8, 0x7f80_0001);
            Ret::default()
        });
        silent(&mut e, &[VECTOR_SUBTRACT, CLAMP]);
        e.register(VECTOR_LENGTH, |_, _| Ret {
            st0: 30.0,
            ..Ret::default()
        });
        let this: Ptr<NavMeshSearchClosePoint> = e.new_object();
        e.set(this, NavMeshSearchClosePoint::fDistanceUpperBound, 100.0);
        let node: Ptr<AStarNodeNavMeshSearch> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x006a8d20, &args![this, node, 0xa000u32, 7u16]);
        assert_eq!(
            e.get(node, AStarNodeNavMeshSearch::pNavMesh),
            Ptr::new(0xa000)
        );
        assert_eq!(e.get(node, AStarNodeNavMeshSearch::sTriangle), 7);
        assert_eq!(e.get(node, AStarNodeNavMeshSearch::CenterX), 1.0);
        assert_eq!(e.get(node, AStarNodeNavMeshSearch::CenterY), 2.0);
        assert_eq!(e.mem.u32(node.addr() + 0x18), 0x7fc0_0001);
        // The clamp double leaves the value; the heuristic is 100 - 30.
        assert_eq!(e.get(node, AStarNodeNavMeshSearch::fHeuristic), 70.0);
        let log = calls(&e);
        let center = log[1].1[1];
        assert_eq!(log[1], (GET_CENTER, vec![0xa000, center, 7]));
        assert_eq!(
            log[2],
            (
                VECTOR_SUBTRACT,
                vec![center, log[2].1[1], this.addr() + 0x208c]
            )
        );
        assert_eq!(log[3], (VECTOR_LENGTH, vec![log[2].1[1]]));
        assert_eq!(log[4].1[1..], [0.0f32.to_bits(), 100.0f32.to_bits()]);
    }

    /// Everything `GetNodeConnections` calls, answering for a world with one
    /// open edge (edge `open_edge`) whose neighbour is navmesh 0xb000
    /// triangle 9. `status` is what the node lookup answers.
    struct World {
        open_edge: u32,
        random: u32,
        status: u32,
    }

    fn world(e: &mut Engine, w: &World) {
        let open_edge = w.open_edge;
        let random = w.random;
        let status = w.status;
        e.register_double(RANDOM, move |_, _| Ret {
            eax: random,
            ..Ret::default()
        });
        silent(
            e,
            &[
                NODE_CENTER,
                DOOR_LOCK_CONSTRUCT,
                DOOR_LOCK_DESTRUCT,
                LIST_ADD,
            ],
        );
        e.register_double(GET_MATCHING_EDGE, move |e, a| {
            if a[2] != open_edge {
                return Ret::default();
            }
            e.mem.set_u32(a[3], 0xb000);
            e.mem.set_u32(a[4], 9);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.register(GET_EDGE_EXTRA_INFO, |_, _| Ret::default());
        e.register(GET_TRIANGLE, |_, a| Ret {
            eax: 0x6000 + a[1],
            ..Ret::default()
        });
        e.register(TRIANGLE_HAS_FLAG, |_, _| Ret::default());
        e.register(REQUEST_ACTOR_RADIUS, |_, _| Ret {
            st0: 5.0,
            ..Ret::default()
        });
        let limit = e.mem.alloc(8);
        e.mem.set_f32(limit + 4, 10.0);
        e.register_double(SETTING_VALUE, move |_, _| Ret {
            eax: limit + 4,
            ..Ret::default()
        });
        e.register(NODE_KEY, |_, a| Ret {
            eax: (a[1] << 16) | a[2],
            ..Ret::default()
        });
        e.register_double(GET_OR_CREATE_NODE, move |e, a| {
            let new_node = e.mem.alloc(0x3c);
            e.mem.set_u32(a[3], new_node);
            Ret {
                eax: status,
                ..Ret::default()
            }
        });
        silent(e, &[GET_CENTER, VECTOR_SUBTRACT, CLAMP]);
        e.register(VECTOR_LENGTH, |_, _| Ret::default());
        e.map(ONE, 8);
        e.mem.set_f64(ONE, 1.0);
        e.map(MINIMAL_USE_DOOR_COST, 4);
        e.mem.set_f32(MINIMAL_USE_DOOR_COST, 100.0);
    }

    fn search(e: &mut Engine) -> (Ptr<NavMeshSearchClosePoint>, Ptr<AStarNodeNavMeshSearch>) {
        let this: Ptr<NavMeshSearchClosePoint> = e.new_object();
        e.set(
            this,
            NavMeshSearchClosePoint::pCurrentRequest,
            Ptr::new(0x7000),
        );
        let node: Ptr<AStarNodeNavMeshSearch> = e.new_object();
        e.set(node, AStarNodeNavMeshSearch::pNavMesh, Ptr::new(0xa000));
        e.set(node, AStarNodeNavMeshSearch::sTriangle, 5);
        (this, node)
    }

    #[test]
    fn connections_follow_the_open_edge_and_link_the_new_node() {
        let mut e = Engine::new();
        world(
            &mut e,
            &World {
                open_edge: 1,
                random: 0,
                status: 0,
            },
        );
        let (this, node) = search(&mut e);
        e.call_log = Some(vec![]);
        assert!(e.call(0x006a8dc0, &args![this, node, 0x9000u32]).bool());
        let log = calls(&e);
        let edges: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == GET_MATCHING_EDGE)
            .map(|c| c.1[2])
            .collect();
        assert_eq!(edges, vec![0, 1, 2]);
        let created: Vec<_> = log.iter().filter(|c| c.0 == GET_OR_CREATE_NODE).collect();
        assert_eq!(created.len(), 1);
        // Cost 1.0 gives heuristic 0.0; the key is mesh 0xb000 / triangle 9.
        assert_eq!(created[0].1[1..3], [(0xb000 << 16) | 9, 0]);
        let added: Vec<_> = log.iter().filter(|c| c.0 == LIST_ADD).collect();
        assert_eq!(added.len(), 1);
        let new_node = Ptr::<AStarNodeNavMeshSearch>::new(e.mem.u32(added[0].1[1]));
        assert_eq!(added[0].1[0], 0x9000);
        assert_eq!(
            e.get(new_node, AStarNodeNavMeshSearch::pNavMesh),
            Ptr::new(0xb000)
        );
        assert_eq!(e.get(new_node, AStarNodeNavMeshSearch::sTriangle), 9);
        assert_eq!(
            e.get(new_node, AStarNodeNavMeshSearch::ParentLink_pNavMesh),
            Ptr::new(0xa000)
        );
        assert_eq!(
            e.get(new_node, AStarNodeNavMeshSearch::ParentLink_sTriangle),
            5
        );
        assert_eq!(
            e.get(new_node, AStarNodeNavMeshSearch::ParentLink_iEdgeIndex),
            1
        );
    }

    #[test]
    fn connections_start_and_step_from_the_random_number() {
        let mut e = Engine::new();
        // 11 % 6 = 5: start 2, step 2 -> edges 2, 1, 0.
        world(
            &mut e,
            &World {
                open_edge: 9,
                random: 11,
                status: 0,
            },
        );
        let (this, node) = search(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x006a8dc0, &args![this, node, 0x9000u32]);
        let edges: Vec<u32> = calls(&e)
            .iter()
            .filter(|c| c.0 == GET_MATCHING_EDGE)
            .map(|c| c.1[2])
            .collect();
        assert_eq!(edges, vec![2, 1, 0]);
    }

    #[test]
    fn connections_reuse_a_better_node_and_drop_a_worse_one() {
        let mut e = Engine::new();
        world(
            &mut e,
            &World {
                open_edge: 0,
                random: 0,
                status: 1,
            },
        );
        let (this, node) = search(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x006a8dc0, &args![this, node, 0x9000u32]);
        assert_eq!(calls(&e).iter().filter(|c| c.0 == LIST_ADD).count(), 1);
        let mut e = Engine::new();
        world(
            &mut e,
            &World {
                open_edge: 0,
                random: 0,
                status: 2,
            },
        );
        let (this, node) = search(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x006a8dc0, &args![this, node, 0x9000u32]);
        assert_eq!(calls(&e).iter().filter(|c| c.0 == LIST_ADD).count(), 0);
    }

    #[test]
    fn connections_skip_an_edge_with_extra_info_of_type_one() {
        let mut e = Engine::new();
        world(
            &mut e,
            &World {
                open_edge: 0,
                random: 0,
                status: 0,
            },
        );
        let info = e.mem.alloc(4);
        e.mem.set_u32(info, 1);
        e.register_double(GET_EDGE_EXTRA_INFO, move |_, _| Ret {
            eax: info,
            ..Ret::default()
        });
        let (this, node) = search(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x006a8dc0, &args![this, node, 0x9000u32]);
        assert!(calls(&e).iter().all(|c| c.0 != GET_OR_CREATE_NODE));
    }

    /// A closed door on the neighbour triangle (flag 0x1000).
    fn closed_door(e: &mut Engine, cant_open: bool, can_unlock: bool, minimal: bool) {
        e.register(TRIANGLE_HAS_FLAG, |_, a| Ret {
            eax: u32::from(a[1] == TRIANGLE_DOOR_FLAG),
            ..Ret::default()
        });
        e.register_double(REQUEST_CANT_OPEN_DOORS, move |_, _| Ret {
            eax: u32::from(cant_open),
            ..Ret::default()
        });
        let door_id = e.mem.alloc(4);
        e.mem.set_u32(door_id, 0x1234);
        e.register_double(GET_DOOR_INFO, move |_, _| Ret {
            eax: door_id,
            ..Ret::default()
        });
        e.register(LOOKUP_FORM, |_, a| Ret {
            eax: if a[0] == 0x1234 { 0x5500 } else { 0 },
            ..Ret::default()
        });
        e.register(REQUEST_LOCK_DATA, |_, a| Ret {
            eax: a[0] + 0x5c,
            ..Ret::default()
        });
        e.register_double(DOOR_LOCK_CAN_UNLOCK, move |_, _| Ret {
            eax: u32::from(can_unlock),
            ..Ret::default()
        });
        e.register(DOOR_BASE_FORM, |_, a| Ret {
            eax: a[0] + 0x20,
            ..Ret::default()
        });
        e.register_double(IS_MINIMAL_USE, move |_, _| Ret {
            eax: u32::from(minimal),
            ..Ret::default()
        });
    }

    #[test]
    fn a_closed_door_the_actor_cannot_open_blocks_the_edge() {
        let mut e = Engine::new();
        world(
            &mut e,
            &World {
                open_edge: 0,
                random: 0,
                status: 0,
            },
        );
        closed_door(&mut e, true, true, false);
        let (this, node) = search(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x006a8dc0, &args![this, node, 0x9000u32]);
        let log = calls(&e);
        assert!(log.iter().all(|c| c.0 != GET_OR_CREATE_NODE));
        assert!(log.iter().all(|c| c.0 != DOOR_LOCK_CONSTRUCT));
    }

    #[test]
    fn a_locked_door_the_actor_cannot_unlock_blocks_the_edge() {
        let mut e = Engine::new();
        world(
            &mut e,
            &World {
                open_edge: 0,
                random: 0,
                status: 0,
            },
        );
        closed_door(&mut e, false, false, false);
        let (this, node) = search(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x006a8dc0, &args![this, node, 0x9000u32]);
        let log = calls(&e);
        assert!(log.iter().all(|c| c.0 != GET_OR_CREATE_NODE));
        let construct = log.iter().find(|c| c.0 == DOOR_LOCK_CONSTRUCT).unwrap();
        assert_eq!(construct.1[1], 0x5500);
        assert_eq!(log.iter().filter(|c| c.0 == DOOR_LOCK_DESTRUCT).count(), 1);
    }

    #[test]
    fn a_minimal_use_door_costs_the_door_constant() {
        let mut e = Engine::new();
        world(
            &mut e,
            &World {
                open_edge: 0,
                random: 0,
                status: 0,
            },
        );
        closed_door(&mut e, false, true, true);
        let (this, node) = search(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x006a8dc0, &args![this, node, 0x9000u32]);
        let log = calls(&e);
        let created = log.iter().find(|c| c.0 == GET_OR_CREATE_NODE).unwrap();
        assert_eq!(created.1[2], 100.0f32.to_bits());
        assert_eq!(log.iter().filter(|c| c.0 == DOOR_LOCK_DESTRUCT).count(), 1);
    }

    #[test]
    fn a_blocked_triangle_is_skipped_when_the_actor_is_wider_than_the_setting() {
        let mut e = Engine::new();
        world(
            &mut e,
            &World {
                open_edge: 0,
                random: 0,
                status: 0,
            },
        );
        e.register(TRIANGLE_HAS_FLAG, |_, a| Ret {
            eax: u32::from(a[1] == TRIANGLE_BLOCKED_FLAG),
            ..Ret::default()
        });
        // Setting 10.0 is not below the radius 5.0: the edge is used.
        let (this, node) = search(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x006a8dc0, &args![this, node, 0x9000u32]);
        assert!(calls(&e).iter().any(|c| c.0 == GET_OR_CREATE_NODE));
        // Radius 20.0 exceeds the setting: the blocked triangle is skipped.
        e.register(REQUEST_ACTOR_RADIUS, |_, _| Ret {
            st0: 20.0,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x006a8dc0, &args![this, node, 0x9000u32]);
        assert!(calls(&e).iter().all(|c| c.0 != GET_OR_CREATE_NODE));
    }

    /// Doubles for `IsGoalNode`; the origin is a flat distance `distance`
    /// from the node, the avoid cost is `cost` (none when `None`).
    fn goal_world(e: &mut Engine, distance: f32, valid: bool, cost: Option<f32>) {
        e.register(NODE_CENTER, |_, _| Ret::default());
        e.register(REQUEST_ORIGIN, |_, a| Ret {
            eax: a[0] + 0xc,
            ..Ret::default()
        });
        silent(
            e,
            &[
                ORIGIN_POSITION,
                VECTOR_SUBTRACT,
                PATHING_LOCATION_CONSTRUCT,
                PATHING_LOCATION_DESTRUCT,
            ],
        );
        e.register_double(FLAT_LENGTH, move |_, _| Ret {
            st0: f64::from(distance),
            ..Ret::default()
        });
        e.register(MESH_OWNER, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        e.register(REQUEST_ACTOR_RADIUS, |_, _| Ret {
            st0: 5.0,
            ..Ret::default()
        });
        e.register_double(LOCATION_IS_VALID, move |_, _| Ret {
            eax: u32::from(valid),
            ..Ret::default()
        });
        e.register_double(REQUEST_AVOID_NODES, move |_, _| Ret {
            eax: if cost.is_some() { 0x8000 } else { 0 },
            ..Ret::default()
        });
        e.register_double(AVOID_COST_FOR_POINT, move |_, _| Ret {
            st0: f64::from(cost.unwrap_or(0.0)),
            ..Ret::default()
        });
        e.map(ZERO, 8);
        e.mem.set_f64(ZERO, 0.0);
    }

    fn is_goal(distance: f32, minimum: f32, valid: bool, cost: Option<f32>) -> (bool, Vec<u32>) {
        let mut e = Engine::new();
        goal_world(&mut e, distance, valid, cost);
        let (this, node) = search(&mut e);
        e.set(this, NavMeshSearchClosePoint::fMinDistance, minimum);
        e.call_log = Some(vec![]);
        let result = e.call(0x006a90a0, &args![this, node]).bool();
        (result, calls(&e).iter().map(|c| c.0).collect())
    }

    #[test]
    fn goal_needs_the_minimum_distance_first() {
        let (result, log) = is_goal(10.0, 20.0, true, None);
        assert!(!result);
        assert!(!log.contains(&PATHING_LOCATION_CONSTRUCT));
        // Equal distance is accepted.
        let (result, _) = is_goal(20.0, 20.0, true, None);
        assert!(result);
        // A NaN distance is accepted by the comparison.
        let (result, _) = is_goal(f32::NAN, 20.0, true, None);
        assert!(result);
    }

    #[test]
    fn goal_needs_a_valid_location() {
        let (result, log) = is_goal(30.0, 20.0, false, None);
        assert!(!result);
        assert!(log.contains(&PATHING_LOCATION_DESTRUCT));
        assert!(!log.contains(&REQUEST_AVOID_NODES));
    }

    #[test]
    fn goal_accepts_a_point_with_no_avoid_cost() {
        assert!(is_goal(30.0, 20.0, true, None).0);
        assert!(is_goal(30.0, 20.0, true, Some(0.0)).0);
        assert!(is_goal(30.0, 20.0, true, Some(-3.0)).0);
        assert!(!is_goal(30.0, 20.0, true, Some(0.5)).0);
        assert!(!is_goal(30.0, 20.0, true, Some(f32::NAN)).0);
    }
}
