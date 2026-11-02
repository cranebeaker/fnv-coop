//! `fallout shared/pathfinding/navmeshsearchhide.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006a9d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `0106b7dc` (RTTI `.?AVNavMeshSearchHide@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `006a6b00`. One word, plain `RET`, returns nothing.
pub fn fn_006a9d60(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0106_b7dc);
    e.call(0x006a_6b00, &args![this]);
}

/// `operator delete` (`00401030`, cdecl, one word).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The globals (a `NiPoint3`, `011f426c`) the search copies as its avoid
/// center before it has one.
const ORIGIN_011F426C: u32 = 0x011f_426c;
/// `00413f40` (unnamed in the engine map): returns `this + 8`.
const PLUS_EIGHT_00413F40: u32 = 0x0041_3f40;
/// `00439ef0` (unnamed in the engine map; `RET 8`: out, other): the 3D
/// difference `this - other` into `out`; returns `out`.
const POINT3_MINUS_00439EF0: u32 = 0x0043_9ef0;
/// `00457990` (unnamed in the engine map): the length of the 3D vector
/// `this` (ST0).
const POINT3_LENGTH_00457990: u32 = 0x0045_7990;
/// `00589850` (unnamed in the engine map): a 3D vector's length (ST0).
const LENGTH_00589850: u32 = 0x0058_9850;
/// `NavMesh::GetCenter` (`00558220`, engine map; `RET 8`: out, triangle).
const GET_CENTER_00558220: u32 = 0x0055_8220;
/// `005330e0` (unnamed in the engine map; cdecl: a float's address, low,
/// high): clamps the float in place.
const CLAMP_005330E0: u32 = 0x0053_30e0;
/// `0048d150` (unnamed in the engine map): the request's start location.
const REQUEST_START_0048D150: u32 = 0x0048_d150;
/// `00984230` (unnamed in the engine map): the hide request's threat
/// location.
const REQUEST_THREAT_00984230: u32 = 0x0098_4230;
/// `005c3420` (unnamed in the engine map; `RET 4`: out): a location's
/// position into `out`; returns `out`.
const LOCATION_POSITION_005C3420: u32 = 0x005c_3420;
/// `0069e390` (navmeshpovsearch unit): the request's avoid nodes (0 when
/// none).
const AVOID_NODES_0069E390: u32 = 0x0069_e390;
/// `006dc9f0` (unnamed in the engine map; `RET 8`: point, out): the avoid
/// nodes' radius around a point, their center to `out` (ST0).
const AVOID_RADIUS_006DC9F0: u32 = 0x006d_c9f0;
/// `PathingAvoidNodeArray::GetCostForPoint` (`006dc990`, decompiler name;
/// `RET 4`): the avoid nodes' cost at a point (ST0).
const AVOID_COST_006DC990: u32 = 0x006d_c990;
/// `006a6eb0` (navmeshsearch unit; `RET 4`: out): a search node's center
/// into `out`; returns `out`.
const NODE_CENTER_006A6EB0: u32 = 0x006a_6eb0;
/// `006a94f0` (unnamed in the engine map; `RET 8`: navmesh, triangle): a
/// node key.
const NODE_KEY_006A94F0: u32 = 0x006a_94f0;
/// `AStarSearch<NavMeshSearchNode,NavMeshSearchLink>::AddStartNode`
/// (`006a7c90`, decompiler name; `RET 0xC`: key, node data, heuristic).
const ADD_START_NODE_006A7C90: u32 = 0x006a_7c90;
/// `AStarSearch<NavMeshSearchNode,NavMeshSearchLink>::Search` (`006a7d60`,
/// decompiler name; `RET 0xC`).
const SEARCH_006A7D60: u32 = 0x006a_7d60;
/// `AStarSearch<NavMeshSearchNode,NavMeshSearchLink>::GetOrCreateAStarNode`
/// (`006a7e00`, decompiler name; `RET 0xC`: key, cost, out node): 0 new, 1
/// cheaper, other: keep the old one.
const NODE_FOR_KEY_006A7E00: u32 = 0x006a_7e00;
/// `0076b610` (unnamed in the engine map): a flag of the solution (AL).
const SOLUTION_FLAG_0076B610: u32 = 0x0076_b610;
/// `0045cd60` (unnamed in the engine map): a search node's parent.
const NODE_PARENT_0045CD60: u32 = 0x0045_cd60;
/// `NavMesh::GetMatchingEdge_ov3` (`0068f460`, engine map; `RET 0x14`).
const GET_MATCHING_EDGE_0068F460: u32 = 0x0068_f460;
/// `NavMesh::GetEdgeExtraInfo` (`0068f230`, engine map; `RET 8`).
const GET_EDGE_EXTRA_INFO_0068F230: u32 = 0x0068_f230;
/// `005582f0` (unnamed in the engine map; `RET 4`): the triangle record of
/// a navmesh.
const TRIANGLE_005582F0: u32 = 0x0055_82f0;
/// `00691140` (unnamed in the engine map; `RET 4`): whether a triangle has
/// a flag (AL).
const TRIANGLE_HAS_FLAG_00691140: u32 = 0x0069_1140;
/// `0073cb80` (unnamed in the engine map), on the request (AL).
const REQUEST_FLAG_0073CB80: u32 = 0x0073_cb80;
/// `NavMesh::GetClosedDoorInfoForTriangle` (`00699af0`, engine map; `RET 4`).
const CLOSED_DOOR_INFO_00699AF0: u32 = 0x0069_9af0;
/// `004839c0` (unnamed in the engine map; cdecl): the door's lock.
const DOOR_LOCK_004839C0: u32 = 0x0048_39c0;
/// `DoorLock::DoorLock` (`00501d20`), `DoorLock::~DoorLock` (`00501e50`),
/// `DoorLock::CanActorUnlockDoorLock` (`00502450`, `RET 0xC`), engine map.
const DOOR_LOCK_CTOR_00501D20: u32 = 0x0050_1d20;
const DOOR_LOCK_DTOR_00501E50: u32 = 0x0050_1e50;
const CAN_UNLOCK_00502450: u32 = 0x0050_2450;
/// `006c9900` (unnamed in the engine map): the request's actor.
const REQUEST_ACTOR_006C9900: u32 = 0x006c_9900;
/// `007af430` (unnamed in the engine map): the lock's door form.
const LOCK_FORM_007AF430: u32 = 0x007a_f430;
/// `TESObjectDOOR::IsMinimalUse` (`00518000`, engine map).
const IS_MINIMAL_USE_00518000: u32 = 0x0051_8000;
/// The cost factor (float, `01016410`) of a minimal-use door.
const DOOR_COST: u32 = 0x0101_6410;
/// `0069ef80` (navmeshpovsearch unit): the request's float at +0x68 (ST0).
const REQUEST_VALUE_0069EF80: u32 = 0x0069_ef80;
/// `00403e20` (unnamed in the engine map): the address of a setting's
/// value; the setting at `011d6ef0`.
const SETTING_VALUE_00403E20: u32 = 0x0040_3e20;
const SETTING_011D6EF0: u32 = 0x011d_6ef0;
/// The `double` the distance to the threat is divided by.
const THREAT_RANGE: u32 = 0x0102_8338;
/// `00404010` (unnamed in the engine map; cdecl, two floats; ST0).
const MAX_00404010: u32 = 0x0040_4010;
/// `NiPoint3::Dot` (`004b6190`, engine map; `RET 4`; ST0).
const DOT_004B6190: u32 = 0x004b_6190;
/// `BSSimpleArray<BGSBodyPart*,1024>::AddUninitialized` (`007cb2e0`, engine
/// map name of the folded body; `RET 4`): appends the word at the address
/// it is given.
const ARRAY_PUSH_007CB2E0: u32 = 0x007c_b2e0;
/// `006a78d0` (unnamed in the engine map): the triangle record of a search
/// node's data.
const NODE_TRIANGLE_006A78D0: u32 = 0x006a_78d0;
/// `00690770` (unnamed in the engine map): a flag of a triangle (AL).
const TRIANGLE_FLAG_00690770: u32 = 0x0069_0770;
/// `00885bd0` (unnamed in the engine map): the hide request's distance
/// (ST0).
const REQUEST_DISTANCE_00885BD0: u32 = 0x0088_5bd0;
/// `006a5b30` (unnamed in the engine map; `RET 0xC`): a triangle's three
/// vertex indices (16-bit) to three addresses.
const TRIANGLE_VERTICES_006A5B30: u32 = 0x006a_5b30;
/// `006815c0` (unnamed in the engine map): returns its `this`.
const IDENTITY_006815C0: u32 = 0x0068_15c0;
/// `0068f0a0` (unnamed in the engine map; `RET 4`): the address of a
/// navmesh vertex's position.
const VERTEX_POSITION_0068F0A0: u32 = 0x0068_f0a0;
/// `006b9790` (unnamed in the engine map; cdecl, five words).
const TRIANGLE_TEST_006B9790: u32 = 0x006b_9790;
/// `00698030` (unnamed in the engine map; `RET 4`): a 16-bit value of the
/// triangle for a side.
const TRIANGLE_SIDE_00698030: u32 = 0x0069_8030;
/// `0.0` (double).
const ZERO: u32 = 0x0101_2060;

/// `NavMeshSearch` and `NavMeshSearchHide` fields (Xbox PDB names, PC
/// offsets).
const CURRENT_REQUEST: u32 = 0x2058;
const CURRENT_HIDE_REQUEST: u32 = 0x20a4;
const HIDE_START_NODE: u32 = 0x20a8;
const AVOID_NODE_CENTER: u32 = 0x20bc;
const DISTANCE_UPPER_BOUND: u32 = 0x20c8;
const THREAT_LOCATION: u32 = 0x20cc;

fn copy_words(e: &mut Engine, to: u32, from: u32, count: u32) {
    for i in 0..count {
        let word = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, word);
    }
}

/// The heuristic of `006a9d80` and `006a9f60`: the distance upper bound
/// minus the distance from `center` to the avoid center (`00439ef0` into
/// `temp`, `00457990`), stored at `value` and clamped to `[0, bound]`
/// (`005330e0`).
fn bounded_distance(e: &mut Engine, this: u32, center: u32, temp: u32, value: u32) {
    let difference = e
        .call(
            POINT3_MINUS_00439EF0,
            &args![center, temp, this + AVOID_NODE_CENTER],
        )
        .u32();
    let length = e.call(POINT3_LENGTH_00457990, &args![difference]).f64();
    let bound = e.mem.u32(this + DISTANCE_UPPER_BOUND);
    e.mem
        .set_u32(value, x87_store32(x87_sub(x87_m32(bound), length)));
    let bound = x87_quiet_bits(e.mem.u32(this + DISTANCE_UPPER_BOUND));
    e.call(CLAMP_005330E0, &args![value, x87_store32(0.0), bound]);
}

// Translated from 006a9d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshSearchHide::scalar deleting destructor` (Xbox PDB): `006a9d60`,
/// then `operator delete` when bit 0 of `flags` is set. Returns `this`.
pub fn navmesh_search_hide_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_006a9d60(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 006a9f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshSearchHide::SetNodeData` (Xbox PDB; `RET 0xC`): the node's
/// navmesh (+8) and triangle (+0xC, 16 bits), its center (+0x10,
/// `NavMesh::GetCenter`) and its heuristic (+4): the distance upper bound
/// minus the center's distance to the avoid center, clamped to
/// `[0, bound]`.
pub fn navmesh_search_hide_set_node_data(
    e: &mut Engine,
    this: Ptr,
    node: Ptr,
    navmesh: u32,
    triangle: u32,
) {
    let base = this.addr();
    let node = node.addr();
    e.with_stack(0x20, |e, locals| {
        let ebp = locals.addr() + 0x20;
        e.mem.set_u32(node + 8, navmesh);
        e.mem.set_u16(node + 0xc, triangle as u16);
        let center = ebp - 0xc;
        e.call(
            GET_CENTER_00558220,
            &args![navmesh, center, triangle & 0xffff],
        );
        for i in 0..3 {
            let word = x87_quiet_bits(e.mem.u32(center + 4 * i));
            e.mem.set_u32(node + 0x10 + 4 * i, word);
        }
        bounded_distance(e, base, center, ebp - 0x1c, ebp - 0x10);
        let heuristic = x87_quiet_bits(e.mem.u32(ebp - 0x10));
        e.mem.set_u32(node + 4, heuristic);
    });
}

// Translated from 006a9d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 0x10`: the hide request, the start node
/// data, two words for the search): runs the hide search. Keeps the request
/// (`pCurrentRequest`, `pCurrentHideRequest`) and the start node (five
/// words); the avoid center is the origin at `011f426c` and the distance
/// bound 0 unless the request has avoid nodes (`0069e390`; `006dc9f0`
/// gives both). Adds the start node (`006a7c90`, heuristic as
/// `006a9f60`'s), keeps the threat location (`00984230`, `005c3420`) and
/// searches (`006a7d60(first, second, 1)`). True when the solution's flag
/// (`0076b610` on `first`) is clear.
pub fn fn_006a9d80(
    e: &mut Engine,
    this: Ptr,
    request: u32,
    start: Ptr,
    first: u32,
    second: u32,
) -> bool {
    let base = this.addr();
    e.with_stack(0x58, |e, locals| {
        let ebp = locals.addr() + 0x58;
        e.mem.set_u32(base + CURRENT_REQUEST, request);
        e.mem.set_u32(base + CURRENT_HIDE_REQUEST, request);
        copy_words(e, base + HIDE_START_NODE, start.addr(), 5);
        let location = e.call(REQUEST_START_0048D150, &args![request]).u32();
        e.call(LOCATION_POSITION_005C3420, &args![location, ebp - 0x3c]);
        copy_words(e, base + AVOID_NODE_CENTER, ORIGIN_011F426C, 3);
        e.mem.set_u32(base + DISTANCE_UPPER_BOUND, x87_store32(0.0));
        let hide_request = e.mem.u32(base + CURRENT_HIDE_REQUEST);
        if e.call(AVOID_NODES_0069E390, &args![hide_request]).u32() != 0 {
            let hide_request = e.mem.u32(base + CURRENT_HIDE_REQUEST);
            let nodes = e.call(AVOID_NODES_0069E390, &args![hide_request]).u32();
            let radius = e
                .call(
                    AVOID_RADIUS_006DC9F0,
                    &args![nodes, ebp - 0x3c, base + AVOID_NODE_CENTER],
                )
                .f64();
            e.mem
                .set_u32(base + DISTANCE_UPPER_BOUND, x87_store32(radius));
        }
        let data = ebp - 0x30;
        let navmesh = e.mem.u32(start.addr());
        e.mem.set_u32(data, navmesh);
        let triangle = e.mem.u16(start.addr() + 4);
        e.mem.set_u16(data + 4, triangle);
        let center = ebp - 0xc;
        e.call(
            GET_CENTER_00558220,
            &args![navmesh, center, u32::from(e.mem.u16(data + 4))],
        );
        for i in 0..3 {
            let word = x87_quiet_bits(e.mem.u32(center + 4 * i));
            e.mem.set_u32(data + 8 + 4 * i, word);
        }
        let node_center = e.call(NODE_CENTER_006A6EB0, &args![data, ebp - 0x48]).u32();
        bounded_distance(e, base, node_center, ebp - 0x1c, ebp - 0x10);
        let heuristic = x87_quiet_bits(e.mem.u32(ebp - 0x10));
        let navmesh = e.mem.u32(data);
        let triangle = u32::from(e.mem.u16(data + 4));
        let key = e
            .call(NODE_KEY_006A94F0, &args![this, navmesh, triangle])
            .u32();
        e.call(ADD_START_NODE_006A7C90, &args![this, key, data, heuristic]);
        let hide_request = e.mem.u32(base + CURRENT_HIDE_REQUEST);
        let threat = e.call(REQUEST_THREAT_00984230, &args![hide_request]).u32();
        let position = e
            .call(LOCATION_POSITION_005C3420, &args![threat, ebp - 0x54])
            .u32();
        copy_words(e, base + THREAT_LOCATION, position, 3);
        e.call(SEARCH_006A7D60, &args![this, first, second, 1u32]);
        e.call(SOLUTION_FLAG_0076B610, &args![first]).u8() == 0
    })
}

/// One edge `edge` of `006aa000`; adds a connection unless the edge is
/// skipped.
fn hide_connection(e: &mut Engine, this: u32, ebp: u32, node: u32, connections: u32, edge: u32) {
    let navmesh = e.mem.u32(node + 8);
    let triangle = u32::from(e.mem.u16(node + 0xc));
    let next_navmesh = ebp - 0x28;
    let next_triangle = ebp - 0x20;
    let next_edge = ebp - 0x24;
    let matched = e
        .call(
            GET_MATCHING_EDGE_0068F460,
            &args![
                navmesh,
                triangle,
                edge,
                next_navmesh,
                next_triangle,
                next_edge
            ],
        )
        .u8();
    if matched == 0 {
        return;
    }
    let navmesh = e.mem.u32(node + 8);
    let triangle = u32::from(e.mem.u16(node + 0xc));
    let info = e
        .call(
            GET_EDGE_EXTRA_INFO_0068F230,
            &args![navmesh, triangle, edge],
        )
        .u32();
    if info != 0 && e.mem.u32(info) == 1 {
        return;
    }
    e.mem.set_u32(ebp - 0x70, x87_store32(1.0));
    let record = e
        .call(
            TRIANGLE_005582F0,
            &args![e.mem.u32(next_navmesh), u32::from(e.mem.u16(next_triangle))],
        )
        .u32();
    if e.call(TRIANGLE_HAS_FLAG_00691140, &args![record, 0x1000u32])
        .u8()
        != 0
    {
        let request = e.mem.u32(this + CURRENT_HIDE_REQUEST);
        if e.call(REQUEST_FLAG_0073CB80, &args![request]).u8() != 0 {
            return;
        }
        let info = e
            .call(
                CLOSED_DOOR_INFO_00699AF0,
                &args![e.mem.u32(next_navmesh), u32::from(e.mem.u16(next_triangle))],
            )
            .u32();
        let door = e.mem.u32(info);
        e.mem.set_u32(ebp - 0x7c, door);
        let lock = e.call(DOOR_LOCK_004839C0, &args![door]).u32();
        e.mem.set_u32(ebp - 0x78, lock);
        if lock != 0 {
            let temp_lock = ebp - 0x9c;
            e.call(DOOR_LOCK_CTOR_00501D20, &args![temp_lock, lock]);
            let request = e.mem.u32(this + CURRENT_REQUEST);
            let actor = e.call(REQUEST_ACTOR_006C9900, &args![request]).u32();
            e.mem.set_u32(ebp - 0x80, actor);
            let can = e
                .call(
                    CAN_UNLOCK_00502450,
                    &args![temp_lock, actor, lock, ebp - 0x81],
                )
                .u8();
            if can == 0 {
                e.call(DOOR_LOCK_DTOR_00501E50, &args![temp_lock]);
                return;
            }
            let form = e.call(LOCK_FORM_007AF430, &args![lock]).u32();
            if e.call(IS_MINIMAL_USE_00518000, &args![form]).u8() != 0 {
                let factor = x87_quiet_bits(e.global::<u32>(DOOR_COST));
                e.mem.set_u32(ebp - 0x70, factor);
            }
            e.call(DOOR_LOCK_DTOR_00501E50, &args![temp_lock]);
        }
    }
    let request = e.mem.u32(this + CURRENT_HIDE_REQUEST);
    let limit = e.call(REQUEST_VALUE_0069EF80, &args![request]).f64();
    let setting = e
        .call(SETTING_VALUE_00403E20, &args![SETTING_011D6EF0])
        .u32();
    if x87_load32(e.mem.u32(setting)) < limit {
        let record = e
            .call(
                TRIANGLE_005582F0,
                &args![e.mem.u32(next_navmesh), u32::from(e.mem.u16(next_triangle))],
            )
            .u32();
        if e.call(TRIANGLE_HAS_FLAG_00691140, &args![record, 0x10u32])
            .u8()
            != 0
        {
            return;
        }
    }
    let center = ebp - 0x4c;
    e.call(
        GET_CENTER_00558220,
        &args![
            e.mem.u32(next_navmesh),
            center,
            u32::from(e.mem.u16(next_triangle))
        ],
    );
    let step = ebp - 0x38;
    e.call(POINT3_MINUS_00439EF0, &args![center, step, ebp - 0x18]);
    let length = e.call(LENGTH_00589850, &args![step]).f64();
    let cost = x87_mul(length, x87_m32(e.mem.u32(ebp - 0x70)));
    let cost = x87_add(cost, x87_m32(e.mem.u32(node)));
    e.mem.set_u32(ebp - 0x58, x87_store32(cost));
    e.mem.set_u32(ebp - 0x40, x87_store32(0.0));
    let request = e.mem.u32(this + CURRENT_HIDE_REQUEST);
    if e.call(AVOID_NODES_0069E390, &args![request]).u32() != 0 {
        let request = e.mem.u32(this + CURRENT_HIDE_REQUEST);
        let nodes = e.call(AVOID_NODES_0069E390, &args![request]).u32();
        let avoid = e.call(AVOID_COST_006DC990, &args![nodes, center]).f64();
        e.mem.set_u32(ebp - 0x40, x87_store32(avoid));
    }
    let threat = e
        .call(
            POINT3_MINUS_00439EF0,
            &args![center, ebp - 0xc4, this + THREAT_LOCATION],
        )
        .u32();
    let distance = e.call(LENGTH_00589850, &args![threat]).f64();
    e.mem.set_u32(ebp - 0x6c, x87_store32(distance));
    let range: f64 = e.global(THREAT_RANGE);
    let near = x87_sub(1.0, x87_div(x87_load32(e.mem.u32(ebp - 0x6c)), range));
    e.mem.set_u32(ebp - 0x74, x87_store32(near));
    let near = x87_quiet_bits(e.mem.u32(ebp - 0x74));
    let term = e.call(MAX_00404010, &args![near, x87_store32(0.0)]).f64();
    e.mem.set_u32(
        ebp - 0x40,
        x87_store32(x87_add(term, x87_m32(e.mem.u32(ebp - 0x40)))),
    );
    let away = ebp - 0x64;
    e.call(
        POINT3_MINUS_00439EF0,
        &args![this + THREAT_LOCATION, away, ebp - 0x18],
    );
    let dot = e.call(DOT_004B6190, &args![away, step]).f64();
    e.mem.set_u32(ebp - 0x3c, x87_store32(dot));
    let away_length = e.call(POINT3_LENGTH_00457990, &args![away]).f64();
    e.mem.set_u32(ebp - 0x50, x87_store32(away_length));
    let step_length = e.call(POINT3_LENGTH_00457990, &args![step]).f64();
    e.mem.set_u32(ebp - 0x2c, x87_store32(step_length));
    let zero: f64 = e.global(ZERO);
    if x87_load32(e.mem.u32(ebp - 0x50)) == zero || x87_load32(e.mem.u32(ebp - 0x2c)) == zero {
        e.mem.set_u32(ebp - 0x3c, x87_store32(0.0));
    } else {
        let product = x87_mul(
            x87_load32(e.mem.u32(ebp - 0x50)),
            x87_m32(e.mem.u32(ebp - 0x2c)),
        );
        let cosine = x87_div(x87_m32(e.mem.u32(ebp - 0x3c)), product);
        e.mem.set_u32(ebp - 0x3c, x87_store32(cosine));
    }
    let cosine = x87_quiet_bits(e.mem.u32(ebp - 0x3c));
    let term = e.call(MAX_00404010, &args![cosine, x87_store32(0.0)]).f64();
    e.mem.set_u32(
        ebp - 0x40,
        x87_store32(x87_add(term, x87_m32(e.mem.u32(ebp - 0x40)))),
    );
    let factor = x87_add(x87_load32(e.mem.u32(ebp - 0x40)), 1.0);
    let total = x87_mul(factor, x87_m32(e.mem.u32(ebp - 0x58)));
    e.mem.set_u32(ebp - 0x58, x87_store32(total));
    let cost = x87_quiet_bits(e.mem.u32(ebp - 0x58));
    let next = ebp - 0x54;
    let key = e
        .call(
            NODE_KEY_006A94F0,
            &args![
                this,
                e.mem.u32(next_navmesh),
                u32::from(e.mem.u16(next_triangle))
            ],
        )
        .u32();
    match e
        .call(NODE_FOR_KEY_006A7E00, &args![this, key, cost, next])
        .u32()
    {
        0 => {
            let new_node = e.mem.u32(next);
            navmesh_search_hide_set_node_data(
                e,
                Ptr::new(this),
                Ptr::new(new_node),
                e.mem.u32(next_navmesh),
                u32::from(e.mem.u16(next_triangle)),
            );
        }
        1 => {}
        _ => return,
    }
    let new_node = e.mem.u32(next);
    let navmesh = e.mem.u32(node + 8);
    e.mem.set_u32(new_node + 0x1c, navmesh);
    let triangle = e.mem.u16(node + 0xc);
    e.mem.set_u16(new_node + 0x20, triangle);
    e.mem.set_u32(new_node + 0x24, edge);
    e.call(ARRAY_PUSH_007CB2E0, &args![connections, next]);
}

// Translated from 006aa000 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006aa000 - its FS:[0] exception frame (the unwind state of the temporary DoorLock) is not translated
/// `NavMeshSearchHide::GetNodeConnections` (Xbox PDB; `RET 8`: node,
/// connections): from the node's center (`006a6eb0`; the request's start
/// position, `0048d150`, when the node has no parent `0045cd60`), for each
/// of the triangle's three edges with a matching edge
/// (`GetMatchingEdge_ov3`) whose extra info is not of type 1: door
/// triangles (flag 0x1000) are skipped when the request's flag
/// (`0073cb80`) is set or the actor cannot unlock the door's lock
/// (`DoorLock`), and cost the factor at `01016410` for a minimal-use door;
/// triangles with flag 0x10 are skipped when the setting at `011d6ef0` is
/// below the request's float. The cost is the step's length times the
/// factor plus the node's cost, times one plus the avoid nodes' cost
/// (`006dc990`), the nearness to the threat (`1 - distance / range`) and the
/// cosine towards the threat, each kept non-negative (`00404010`). The node
/// for the key (`006a7e00`; a new one through `006a9f60`) gets the link and
/// is appended. Returns true.
pub fn navmesh_search_hide_get_node_connections(
    e: &mut Engine,
    this: Ptr,
    node: Ptr,
    connections: Ptr,
) -> bool {
    let base = this.addr();
    let node = node.addr();
    e.with_stack(0xd8, |e, locals| {
        let ebp = locals.addr() + 0xd8;
        let position = if e.call(NODE_PARENT_0045CD60, &args![node]).u32() != 0 {
            e.call(NODE_CENTER_006A6EB0, &args![node + 8, ebp - 0xac])
                .u32()
        } else {
            let request = e.mem.u32(base + CURRENT_HIDE_REQUEST);
            let location = e.call(REQUEST_START_0048D150, &args![request]).u32();
            e.call(LOCATION_POSITION_005C3420, &args![location, ebp - 0xb8])
                .u32()
        };
        copy_words(e, ebp - 0x18, position, 3);
        let mut edge = 0i32;
        while edge < 3 {
            hide_connection(e, base, ebp, node, connections.addr(), edge as u32);
            edge = edge.wrapping_add(1);
        }
        true
    })
}

// Translated from 006aa470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshSearchHide::IsGoalNode` (Xbox PDB; `RET 4`): false unless the
/// node's triangle (`006a78d0`) has the flag `00690770` tests and its center
/// (`006a6eb0`) is not nearer the threat than the request's distance
/// (`00885bd0`; nearer, ordered, is false). The triangle must then face away
/// from the threat on a side with a value (`006b9790` on its three vertices,
/// `00698030` with 1 for a negative first result, else with 0 for a negative
/// third). True when the avoid nodes' cost at the center (`006dc990`, 0
/// without avoid nodes) is not above 0 (ordered).
pub fn navmesh_search_hide_is_goal_node(e: &mut Engine, this: Ptr, node: Ptr) -> bool {
    let base = this.addr();
    e.with_stack(0x50, |e, locals| {
        let ebp = locals.addr() + 0x50;
        let data = e.call(PLUS_EIGHT_00413F40, &args![node]).u32();
        let triangle = e.call(NODE_TRIANGLE_006A78D0, &args![data]).u32();
        if e.call(TRIANGLE_FLAG_00690770, &args![triangle]).u8() == 0 {
            return false;
        }
        let center = ebp - 0x18;
        e.call(NODE_CENTER_006A6EB0, &args![node.addr() + 8, center]);
        let threat = e
            .call(
                POINT3_MINUS_00439EF0,
                &args![base + THREAT_LOCATION, ebp - 0x48, center],
            )
            .u32();
        let distance = e.call(LENGTH_00589850, &args![threat]).f64();
        e.mem.set_u32(ebp - 0x2c, x87_store32(distance));
        let request = e.mem.u32(base + CURRENT_HIDE_REQUEST);
        let limit = e.call(REQUEST_DISTANCE_00885BD0, &args![request]).f64();
        if x87_load32(e.mem.u32(ebp - 0x2c)) < limit {
            return false;
        }
        let navmesh = e.mem.u32(data);
        e.call(
            TRIANGLE_VERTICES_006A5B30,
            &args![triangle, ebp - 0x3c, ebp - 0xc, ebp - 0x20],
        );
        let result = ebp - 0x38;
        e.call(IDENTITY_006815C0, &args![result]);
        let third = e
            .call(
                VERTEX_POSITION_0068F0A0,
                &args![navmesh, u32::from(e.mem.u16(ebp - 0x20))],
            )
            .u32();
        let second = e
            .call(
                VERTEX_POSITION_0068F0A0,
                &args![navmesh, u32::from(e.mem.u16(ebp - 0xc))],
            )
            .u32();
        let first = e
            .call(
                VERTEX_POSITION_0068F0A0,
                &args![navmesh, u32::from(e.mem.u16(ebp - 0x3c))],
            )
            .u32();
        e.call(
            TRIANGLE_TEST_006B9790,
            &args![first, second, third, base + THREAT_LOCATION, result],
        );
        let zero: f64 = e.global(ZERO);
        let facing = (x87_load32(e.mem.u32(result)) < zero
            && e.call(TRIANGLE_SIDE_00698030, &args![triangle, 1u32]).u16() != 0)
            || (x87_load32(e.mem.u32(result + 8)) < zero
                && e.call(TRIANGLE_SIDE_00698030, &args![triangle, 0u32]).u16() != 0);
        if !facing {
            return false;
        }
        e.mem.set_u32(ebp - 0x28, x87_store32(0.0));
        let request = e.mem.u32(base + CURRENT_HIDE_REQUEST);
        if e.call(AVOID_NODES_0069E390, &args![request]).u32() != 0 {
            let request = e.mem.u32(base + CURRENT_HIDE_REQUEST);
            let nodes = e.call(AVOID_NODES_0069E390, &args![request]).u32();
            let cost = e.call(AVOID_COST_006DC990, &args![nodes, center]).f64();
            e.mem.set_u32(ebp - 0x28, x87_store32(cost));
        }
        x87_load32(e.mem.u32(ebp - 0x28)) <= zero
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006a_9d60, fn_006a9d60(Ptr)),
        entry!(
            0x006a_9d30,
            navmesh_search_hide_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x006a_9f60,
            navmesh_search_hide_set_node_data(Ptr, Ptr, u32, u32)
        ),
        entry!(0x006a_9d80, fn_006a9d80(Ptr, u32, Ptr, u32, u32) -> bool),
        entry!(
            0x006a_a000,
            navmesh_search_hide_get_node_connections(Ptr, Ptr, Ptr) -> bool
        ),
        entry!(0x006a_a470, navmesh_search_hide_is_goal_node(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    #[test]
    fn fn_006a9d60_stores_the_vtable_then_forwards_this() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcd; 8]);
        // The double copies the vtable word it finds when it runs to `+4`.
        e.register(0x006a_6b00, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x006a_9d60, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x006a_6b00)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0106_b7dc);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0106_b7dc,
            "the vtable was stored before the call"
        );
    }
}

#[cfg(test)]
mod n031_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    fn stub(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    fn logged(e: &mut Engine, run: impl FnOnce(&mut Engine)) -> Log {
        e.call_log = Some(vec![]);
        run(e);
        e.call_log.take().unwrap()
    }

    /// Vector and center doubles: the center of triangle `t` is `(t, 0, 0)`;
    /// lengths are Euclidean; the clamp clamps.
    fn geometry(e: &mut Engine) {
        e.register(PLUS_EIGHT_00413F40, |_, a| (a[0] + 8).into_ret());
        e.register(GET_CENTER_00558220, |e, a| {
            e.mem.set_f32(a[1], a[2] as f32);
            e.mem.set_f32(a[1] + 4, 0.0);
            e.mem.set_f32(a[1] + 8, 0.0);
            a[1].into_ret()
        });
        e.register(POINT3_MINUS_00439EF0, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) - e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        let length = |e: &mut Engine, a: &[u32]| {
            let (x, y, z) = (e.mem.f32(a[0]), e.mem.f32(a[0] + 4), e.mem.f32(a[0] + 8));
            f64::from((x * x + y * y + z * z).sqrt()).into_ret()
        };
        e.register(POINT3_LENGTH_00457990, length);
        e.register(LENGTH_00589850, length);
        e.register(CLAMP_005330E0, |e, a| {
            let (low, high) = (f32::from_bits(a[1]), f32::from_bits(a[2]));
            let v = e.mem.f32(a[0]).max(low).min(high);
            e.mem.set_f32(a[0], v);
            Ret::default()
        });
        e.register(NODE_CENTER_006A6EB0, |e, a| {
            for i in 0..3 {
                let word = e.mem.u32(a[0] + 8 + 4 * i);
                e.mem.set_u32(a[1] + 4 * i, word);
            }
            a[1].into_ret()
        });
        e.register(LOCATION_POSITION_005C3420, |e, a| {
            for i in 0..3 {
                let word = e.mem.u32(a[0] + 4 * i);
                e.mem.set_u32(a[1] + 4 * i, word);
            }
            a[1].into_ret()
        });
        e.map(0x0101_0000, 0x10_0000);
        e.map(0x011d_6000, 0x1000);
        e.map(0x011f_4000, 0x1000);
    }

    #[test]
    fn at_006a9d30_destroys_and_deletes_by_flag() {
        let mut e = Engine::new();
        stub(&mut e, &[0x006a_6b00, OPERATOR_DELETE]);
        let this = e.mem.alloc(0x20dc);
        for flags in [0u32, 1] {
            let log = logged(&mut e, |e| {
                assert_eq!(e.call(0x006a_9d30, &args![this, flags]).u32(), this);
            });
            assert_eq!(calls_to(&log, 0x006a_6b00), vec![vec![this]]);
            assert_eq!(calls_to(&log, OPERATOR_DELETE).len(), flags as usize);
        }
    }

    #[test]
    fn at_006a9f60_sets_the_node_and_its_bounded_heuristic() {
        let mut e = Engine::new();
        geometry(&mut e);
        let this = e.mem.alloc(0x20dc);
        e.mem.set_f32(this + DISTANCE_UPPER_BOUND, 10.0);
        let node = e.mem.alloc(0x3c);
        e.call(0x006a_9f60, &args![this, node, 0x5000u32, 0x0003_0004u32]);
        assert_eq!(e.mem.u32(node + 8), 0x5000);
        assert_eq!(e.mem.u16(node + 0xc), 4);
        assert_eq!(e.mem.f32(node + 0x10), 4.0);
        assert_eq!(e.mem.f32(node + 4), 6.0);
        // Farther than the bound: clamped to 0.
        e.mem.set_f32(this + DISTANCE_UPPER_BOUND, 2.0);
        e.call(0x006a_9f60, &args![this, node, 0x5000u32, 4u32]);
        assert_eq!(e.mem.f32(node + 4), 0.0);
    }

    #[test]
    fn at_006a9d80_sets_up_and_runs_the_search() {
        let mut e = Engine::new();
        geometry(&mut e);
        let location = e.mem.alloc(0xc);
        e.register_double(REQUEST_START_0048D150, move |_, _| location.into_ret());
        e.register_double(REQUEST_THREAT_00984230, move |_, _| location.into_ret());
        e.register(AVOID_NODES_0069E390, |_, _| 0x7000u32.into_ret());
        e.register(AVOID_RADIUS_006DC9F0, |_, _| 8.0f64.into_ret());
        e.register(NODE_KEY_006A94F0, |_, a| (a[1] ^ a[2]).into_ret());
        stub(&mut e, &[ADD_START_NODE_006A7C90, SEARCH_006A7D60]);
        e.register(SOLUTION_FLAG_0076B610, |_, _| Ret::default());
        e.mem.set_f32(location, 9.0);
        let this = e.mem.alloc(0x20dc);
        let start = e.mem.alloc(0x14);
        e.mem.set_u32(start, 0x5000);
        e.mem.set_u16(start + 4, 3);
        let log = logged(&mut e, |e| {
            assert!(e
                .call(
                    0x006a_9d80,
                    &args![this, 0x4444u32, start, 0x11u32, 0x22u32]
                )
                .bool());
        });
        assert_eq!(e.mem.u32(this + CURRENT_REQUEST), 0x4444);
        assert_eq!(e.mem.u32(this + CURRENT_HIDE_REQUEST), 0x4444);
        assert_eq!(e.mem.u32(this + HIDE_START_NODE), 0x5000);
        assert_eq!(e.mem.f32(this + DISTANCE_UPPER_BOUND), 8.0);
        assert_eq!(e.mem.f32(this + THREAT_LOCATION), 9.0);
        let added = calls_to(&log, ADD_START_NODE_006A7C90);
        assert_eq!(added.len(), 1);
        assert_eq!(added[0][1], 0x5003);
        // Bound 8 minus the start center's distance 3 to the avoid center.
        assert_eq!(added[0][3], 5.0f32.to_bits());
        assert_eq!(
            calls_to(&log, SEARCH_006A7D60),
            vec![vec![this, 0x11, 0x22, 1]]
        );
        e.register(SOLUTION_FLAG_0076B610, |_, _| 1u32.into_ret());
        assert!(!e
            .call(
                0x006a_9d80,
                &args![this, 0x4444u32, start, 0x11u32, 0x22u32]
            )
            .bool());
    }

    /// The edge doubles of the connection searches: only `edge` matches,
    /// into triangle 6 of navmesh 0x5000; no flags; the setting not below.
    fn edges(e: &mut Engine, edge: u32) {
        let matching = edge;
        e.register_double(GET_MATCHING_EDGE_0068F460, move |e, a| {
            if a[2] != matching {
                return Ret::default();
            }
            e.mem.set_u32(a[3], 0x5000);
            e.mem.set_u16(a[4], 6);
            1u32.into_ret()
        });
        stub(
            e,
            &[
                GET_EDGE_EXTRA_INFO_0068F230,
                TRIANGLE_005582F0,
                TRIANGLE_HAS_FLAG_00691140,
                AVOID_NODES_0069E390,
                ARRAY_PUSH_007CB2E0,
            ],
        );
        e.register(REQUEST_VALUE_0069EF80, |_, _| 1.0f64.into_ret());
        e.register(SETTING_VALUE_00403E20, |_, _| 0x0101_0000u32.into_ret());
        e.set_global::<f32>(0x0101_0000, 5.0);
        e.register(MAX_00404010, |_, a| {
            f64::from(f32::from_bits(a[0]).max(f32::from_bits(a[1]))).into_ret()
        });
        e.register(DOT_004B6190, |_, _| Ret::default());
        e.register(NODE_KEY_006A94F0, |_, a| (a[1] + a[2]).into_ret());
        e.register(NODE_FOR_KEY_006A7E00, |e, a| {
            let node = e.mem.alloc(0x3c);
            e.mem.set_u32(a[3], node);
            Ret::default()
        });
    }

    #[test]
    fn at_006aa000_connects_the_matching_edges() {
        let mut e = Engine::new();
        geometry(&mut e);
        edges(&mut e, 1);
        e.register(NODE_PARENT_0045CD60, |_, _| 1u32.into_ret());
        e.set_global::<f64>(THREAT_RANGE, 512.0);
        let this = e.mem.alloc(0x20dc);
        e.mem.set_f32(this + DISTANCE_UPPER_BOUND, 100.0);
        let node = e.mem.alloc(0x3c);
        e.mem.set_u32(node + 8, 0x4000);
        e.mem.set_u16(node + 0xc, 2);
        e.mem.set_f32(node + 0x10, 2.0);
        let connections = e.mem.alloc(0x10);
        let log = logged(&mut e, |e| {
            assert!(e.call(0x006a_a000, &args![this, node, connections]).bool());
        });
        assert_eq!(calls_to(&log, GET_MATCHING_EDGE_0068F460).len(), 3);
        let pushes = calls_to(&log, ARRAY_PUSH_007CB2E0);
        assert_eq!(pushes.len(), 1);
        assert_eq!(pushes[0][0], connections);
        let made = calls_to(&log, NODE_FOR_KEY_006A7E00);
        assert_eq!(made[0][1], 0x5006);
        // Step 6 - 2 = 4 at factor 1, times 1 + 0 avoid + nearness
        // (1 - 6/512) + cosine 0.
        let expected = 4.0f32 * (1.0 + (1.0 - 6.0 / 512.0));
        assert_eq!(f32::from_bits(made[0][2]), expected);
        // A matching edge with extra info of type 1 is skipped.
        let info = e.mem.alloc(4);
        e.mem.set_u32(info, 1);
        e.register_double(GET_EDGE_EXTRA_INFO_0068F230, move |_, _| info.into_ret());
        let log = logged(&mut e, |e| {
            e.call(0x006a_a000, &args![this, node, connections]);
        });
        assert!(calls_to(&log, ARRAY_PUSH_007CB2E0).is_empty());
    }

    #[test]
    fn at_006aa470_tests_the_hide_goal() {
        let mut e = Engine::new();
        geometry(&mut e);
        let this = e.mem.alloc(0x20dc);
        let node = e.mem.alloc(0x3c);
        e.mem.set_f32(node + 0x10, 10.0);
        e.register(NODE_TRIANGLE_006A78D0, |_, _| 0x6000u32.into_ret());
        e.register(TRIANGLE_FLAG_00690770, |_, _| Ret::default());
        assert!(!e.call(0x006a_a470, &args![this, node]).bool());
        e.register(TRIANGLE_FLAG_00690770, |_, _| 1u32.into_ret());
        // Nearer the threat than the request's distance: not a goal.
        e.register(REQUEST_DISTANCE_00885BD0, |_, _| 20.0f64.into_ret());
        assert!(!e.call(0x006a_a470, &args![this, node]).bool());
        e.register(REQUEST_DISTANCE_00885BD0, |_, _| 5.0f64.into_ret());
        stub(
            &mut e,
            &[
                TRIANGLE_VERTICES_006A5B30,
                IDENTITY_006815C0,
                AVOID_NODES_0069E390,
            ],
        );
        e.register(VERTEX_POSITION_0068F0A0, |_, _| 0x0101_0100u32.into_ret());
        // Not facing away: not a goal.
        e.register(TRIANGLE_TEST_006B9790, |e, a| {
            e.mem.set_f32(a[4], 1.0);
            e.mem.set_f32(a[4] + 8, 1.0);
            Ret::default()
        });
        assert!(!e.call(0x006a_a470, &args![this, node]).bool());
        e.register(TRIANGLE_TEST_006B9790, |e, a| {
            e.mem.set_f32(a[4], 1.0);
            e.mem.set_f32(a[4] + 8, -1.0);
            Ret::default()
        });
        e.register(TRIANGLE_SIDE_00698030, |_, a| {
            u32::from(a[1] == 0).into_ret()
        });
        let log = logged(&mut e, |e| {
            assert!(e.call(0x006a_a470, &args![this, node]).bool());
        });
        assert_eq!(
            calls_to(&log, TRIANGLE_SIDE_00698030),
            vec![vec![0x6000, 0]]
        );
        // An avoid cost above 0 is not a goal.
        e.register(AVOID_NODES_0069E390, |_, _| 1u32.into_ret());
        e.register(AVOID_COST_006DC990, |_, _| 0.5f64.into_ret());
        assert!(!e.call(0x006a_a470, &args![this, node]).bool());
    }
}
