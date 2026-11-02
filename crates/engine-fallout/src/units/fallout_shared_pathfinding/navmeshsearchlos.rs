//! `fallout shared/pathfinding/navmeshsearchlos.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The byte `006aabf0` returns (+0x39 of its `this`).
const FLAG_BYTE: u32 = 0x39;

// Translated from 006aabf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A getter (unnamed in the engine map): the byte at +0x39 of `this`.
pub fn fn_006aabf0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + FLAG_BYTE)
}

/// `operator delete` (`00401030`, cdecl, one word).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The vtable of `NavMeshSearchLOS` (`0106b830`, stored by its
/// destructor).
const SEARCH_LOS_VTABLE: u32 = 0x0106_b830;
/// `0045cec0` (unnamed in the engine map): a smart pointer's destructor.
const POINTER_RELEASE_0045CEC0: u32 = 0x0045_cec0;
/// `006a7bc0` (navmeshsearch unit): the `AStarSearch` base destructor.
const ASTAR_DTOR_006A7BC0: u32 = 0x006a_7bc0;
/// `005c3420` (unnamed in the engine map; `RET 4`: out): a location's
/// position into `out`; returns `out`.
const LOCATION_POSITION_005C3420: u32 = 0x005c_3420;
/// `NavMesh::GetCenter` (`00558220`, engine map; `RET 8`: out, triangle).
const GET_CENTER_00558220: u32 = 0x0055_8220;
/// `006a6eb0` (navmeshsearch unit; `RET 4`: out): a search node's center
/// into `out`; returns `out`.
const NODE_CENTER_006A6EB0: u32 = 0x006a_6eb0;
/// `00439ef0` (unnamed in the engine map; `RET 8`: out, other): the 3D
/// difference `this - other` into `out`; returns `out`.
const POINT3_MINUS_00439EF0: u32 = 0x0043_9ef0;
/// `00589850` (unnamed in the engine map): a 3D vector's length (ST0).
const LENGTH_00589850: u32 = 0x0058_9850;
/// `004a7290` (unnamed in the engine map): a 3D vector's squared length
/// (ST0, rounded to a float first).
const LENGTH_SQUARED_004A7290: u32 = 0x004a_7290;
/// `00595c80` (unnamed in the engine map): a 3D vector's horizontal length
/// (ST0).
const LENGTH_2D_00595C80: u32 = 0x0059_5c80;
/// `00853130` (unnamed in the engine map; `RET 8`: key, out): looks a key
/// up in the map at +8 of `this` (AL: found; the value to `out`).
const MAP_LOOKUP_00853130: u32 = 0x0085_3130;
/// The score `006ab3b0` gives an entry it has not (`0x7F`).
const NO_SCORE: u8 = 0x7f;
/// `00984230` (unnamed in the engine map): the request's target location
/// (here its LOS map holder).
const REQUEST_TARGET_00984230: u32 = 0x0098_4230;
/// `00559450` (unnamed in the engine map): the word at +0 of its `this`.
const FIRST_OF_00559450: u32 = 0x0055_9450;
/// `00aa13e0` (unnamed in the engine map; cdecl, a size): an allocation.
const ALLOCATE_00AA13E0: u32 = 0x00aa_13e0;
/// `00508050` (unnamed in the engine map): a float of the target (ST0).
const TARGET_VALUE_00508050: u32 = 0x0050_8050;
/// `006e1370` (unnamed in the engine map; `RET 0x10`: a point by value, a
/// float): the 0x50-byte LOS map constructor.
const LOS_MAP_CTOR_006E1370: u32 = 0x006e_1370;
/// `0066b0d0` (unnamed in the engine map; `RET 4`): sets a smart pointer.
const POINTER_SET_0066B0D0: u32 = 0x0066_b0d0;
/// `006e07a0` (unnamed in the engine map; `RET 4`): fills the LOS map from
/// the target.
const LOS_MAP_FILL_006E07A0: u32 = 0x006e_07a0;
/// `0048d150` (unnamed in the engine map): the request's start location.
const REQUEST_START_0048D150: u32 = 0x0048_d150;
/// `006a94f0` (unnamed in the engine map; `RET 8`: navmesh, triangle): a
/// node key.
const NODE_KEY_006A94F0: u32 = 0x006a_94f0;
/// `AStarSearch<NavMeshSearchNode,NavMeshSearchLink>::AddStartNode`
/// (`006a7c90`, decompiler name; `RET 0xC`).
const ADD_START_NODE_006A7C90: u32 = 0x006a_7c90;
/// `AStarSearch<NavMeshSearchNode,NavMeshSearchLink>::Search` (`006a7d60`,
/// decompiler name; `RET 0xC`).
const SEARCH_006A7D60: u32 = 0x006a_7d60;
/// `AStarSearch<NavMeshSearchNode,NavMeshSearchLink>::GetOrCreateAStarNode`
/// (`006a7e00`, decompiler name; `RET 0xC`: key, cost, out node).
const NODE_FOR_KEY_006A7E00: u32 = 0x006a_7e00;
/// `0042fb00`, `0042fa40` (unnamed in the engine map): the navmesh holder's
/// constructor and destructor.
const HOLDER_CTOR_0042FB00: u32 = 0x0042_fb00;
const HOLDER_DTOR_0042FA40: u32 = 0x0042_fa40;
/// `006a9540` (unnamed in the engine map): the request's goal.
const REQUEST_GOAL_006A9540: u32 = 0x006a_9540;
/// `006dcc40` (unnamed in the engine map; `RET 4`): a `PathingLocation`
/// constructor.
const LOCATION_CTOR_006DCC40: u32 = 0x006d_cc40;
/// `004ff7e0` (unnamed in the engine map): the `PathingLocation` destructor
/// body.
const LOCATION_DTOR_004FF7E0: u32 = 0x004f_f7e0;
/// `PathingLocation::ResolveNavMeshInfo` (`006dd6f0`, engine map; `RET 4`).
const RESOLVE_NAVMESH_INFO_006DD6F0: u32 = 0x006d_d6f0;
/// `PathingLocation::GetNavMeshAndTriangle` (`006dd640`, engine map; `RET 8`).
const GET_NAVMESH_AND_TRIANGLE_006DD640: u32 = 0x006d_d640;
/// `004a8f20` (unnamed in the engine map; cdecl, two ints; AL).
const PICK_004A8F20: u32 = 0x004a_8f20;
/// `NavMesh::GetMatchingEdge_ov3` (`0068f460`, engine map; `RET 0x14`).
const GET_MATCHING_EDGE_0068F460: u32 = 0x0068_f460;
/// `NavMesh::GetEdgeExtraInfo` (`0068f230`, engine map; `RET 8`).
const GET_EDGE_EXTRA_INFO_0068F230: u32 = 0x0068_f230;
/// `0084e3a0` (unnamed in the engine map): a navmesh's key.
const NAVMESH_KEY_0084E3A0: u32 = 0x0084_e3a0;
/// `006e2050` (unnamed in the engine map; `RET 4`): whether the LOS map
/// covers a navmesh (AL).
const MAP_HAS_006E2050: u32 = 0x006e_2050;
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
/// `00979260` (unnamed in the engine map): the LOS request's range (ST0).
const REQUEST_RANGE_00979260: u32 = 0x0097_9260;
/// `0069ef80` (navmeshpovsearch unit): the request's float at +0x68 (ST0).
const REQUEST_VALUE_0069EF80: u32 = 0x0069_ef80;
/// `00403e20` (unnamed in the engine map): the address of a setting's
/// value; the setting at `011d6ef0`.
const SETTING_VALUE_00403E20: u32 = 0x0040_3e20;
const SETTING_011D6EF0: u32 = 0x011d_6ef0;
/// `0069e390` (navmeshpovsearch unit): the request's avoid nodes.
const AVOID_NODES_0069E390: u32 = 0x0069_e390;
/// `PathingAvoidNodeArray::GetCostForPoint` (`006dc990`, decompiler name;
/// `RET 4`; ST0).
const AVOID_COST_006DC990: u32 = 0x006d_c990;
/// `004a0c10` (unnamed in the engine map): normalizes a 3D vector.
const NORMALIZE_004A0C10: u32 = 0x004a_0c10;
/// `NiPoint3::Dot` (`004b6190`, engine map; `RET 4`; ST0).
const DOT_004B6190: u32 = 0x004b_6190;
/// `00404010` (unnamed in the engine map; cdecl, two floats; ST0).
const MAX_00404010: u32 = 0x0040_4010;
/// `BSSimpleArray<BGSBodyPart*,1024>::AddUninitialized` (`007cb2e0`, engine
/// map name of the folded body; `RET 4`).
const ARRAY_PUSH_007CB2E0: u32 = 0x007c_b2e0;
/// `006ab360` (unnamed in the engine map): the request's array of points.
const REQUEST_POINTS_006AB360: u32 = 0x006a_b360;
/// `0044ddc0` (unnamed in the engine map): a `BSSimpleArray`'s size.
const ARRAY_SIZE_0044DDC0: u32 = 0x0044_ddc0;
/// `006a1440` (navmeshpovsearch unit; `RET 4`): the address of a 12-byte
/// element.
const ELEMENT_006A1440: u32 = 0x006a_1440;
/// `00644a50` (unnamed in the engine map): the request's distance (ST0).
const REQUEST_DISTANCE_00644A50: u32 = 0x0064_4a50;
/// `00413f40` (unnamed in the engine map): returns `this + 8`.
const PLUS_EIGHT_00413F40: u32 = 0x0041_3f40;
/// `006ab380` (unnamed in the engine map; `RET 4`: out): the LOS map's
/// center into `out`; returns `out`.
const MAP_CENTER_006AB380: u32 = 0x006a_b380;
/// `0.0` (double).
const ZERO: u32 = 0x0101_2060;

/// `NavMeshSearchLOS` fields (Xbox PDB names, PC offsets).
const CURRENT_REQUEST: u32 = 0x2058;
const START_NODE: u32 = 0x205c;
const LOS_MAP: u32 = 0x2070;
const HAS_GOAL_NODE: u32 = 0x2074;
const GOAL_NODE: u32 = 0x2078;
const MIN_LOS_SCORE: u32 = 0x208c;
const FIRST_IGNORED_GOAL: u32 = 0x2090;
const LAST_VALID_NODE: u32 = 0x2094;
const CLOSEST_NODE: u32 = 0x2098;
const CLOSEST_NODE_DISTANCE_SQUARED: u32 = 0x209c;

fn copy_words(e: &mut Engine, to: u32, from: u32, count: u32) {
    for i in 0..count {
        let word = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, word);
    }
}

/// `NavMesh::GetCenter` of a node's triangle into `temp`, copied through
/// the FPU to `to`.
fn center_into(e: &mut Engine, navmesh: u32, triangle: u32, temp: u32, to: u32) {
    e.call(GET_CENTER_00558220, &args![navmesh, temp, triangle]);
    for i in 0..3 {
        let word = x87_quiet_bits(e.mem.u32(temp + 4 * i));
        e.mem.set_u32(to + 4 * i, word);
    }
}

// Translated from 006aa710 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006aa710 - its FS:[0] exception frame (the unwind state of the LOS map pointer) is not translated
/// The destructor body of `NavMeshSearchLOS` (by the vtable it stores):
/// releases `spLOSMap` (`0045cec0`), then the `AStarSearch` base
/// (`006a7bc0`).
pub fn fn_006aa710(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), SEARCH_LOS_VTABLE);
    e.call(POINTER_RELEASE_0045CEC0, &args![this.addr() + LOS_MAP]);
    e.call(ASTAR_DTOR_006A7BC0, &args![this]);
}

// Translated from 006aa6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshSearchLOS::scalar deleting destructor` (Xbox PDB): `006aa710`,
/// then `operator delete` when bit 0 of `flags` is set. Returns `this`.
pub fn navmesh_search_los_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006aa710(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 006aabd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): the position of the location at
/// +0x1C (`005c3420`) into `out`; returns `out`.
pub fn fn_006aabd0(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    e.call(LOCATION_POSITION_005C3420, &args![this.addr() + 0x1c, out]);
    out
}

// Translated from 006ab150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): copies the three words at +0xC8
/// into `out`; returns `out`.
pub fn fn_006ab150(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    copy_words(e, out.addr(), this.addr() + 0xc8, 3);
    out
}

// Translated from 006ab3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 8`): the LOS score of a navmesh key and
/// triangle: the key's base (`00853130` on the map at +8) plus the 16-bit
/// `triangle` indexes the byte buffer at +0x18 when below the count at
/// +0x2C (unsigned); otherwise, or for an unknown key, 0x7F.
pub fn fn_006ab3b0(e: &mut Engine, this: Ptr, key: u32, triangle: u32) -> u8 {
    let base = this.addr();
    e.with_stack(4, |e, found| {
        if e.call(MAP_LOOKUP_00853130, &args![base + 8, key, found])
            .u8()
            == 0
        {
            return NO_SCORE;
        }
        let index = (triangle & 0xffff).wrapping_add(e.mem.u32(found.addr()));
        if index >= e.mem.u32(base + 0x2c) {
            return NO_SCORE;
        }
        let buffer = e.mem.u32(base + 0x18);
        e.mem.u8(index.wrapping_add(buffer))
    })
}

// Translated from 006aac10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshSearchLOS::SetNodeData` (Xbox PDB; `RET 0xC`): the node's
/// navmesh (+8), triangle (+0xC, 16 bits) and center (+0x10,
/// `NavMesh::GetCenter`); its heuristic (+4) is the distance to the goal
/// node's center (`006a6eb0`, `00589850`) when there is a goal node, else 0.
pub fn navmesh_search_los_set_node_data(
    e: &mut Engine,
    this: Ptr,
    node: Ptr,
    navmesh: u32,
    triangle: u32,
) {
    let base = this.addr();
    let node = node.addr();
    e.with_stack(0x28, |e, locals| {
        let ebp = locals.addr() + 0x28;
        e.mem.set_u32(node + 8, navmesh);
        e.mem.set_u16(node + 0xc, triangle as u16);
        let center = ebp - 0xc;
        center_into(e, navmesh, triangle & 0xffff, center, node + 0x10);
        if e.mem.u8(base + HAS_GOAL_NODE) != 0 {
            let goal = e
                .call(NODE_CENTER_006A6EB0, &args![base + GOAL_NODE, ebp - 0x18])
                .u32();
            let difference = e
                .call(POINT3_MINUS_00439EF0, &args![center, ebp - 0x24, goal])
                .u32();
            let distance = e.call(LENGTH_00589850, &args![difference]).f64();
            e.mem.set_u32(node + 4, x87_store32(distance));
        } else {
            e.mem.set_u32(node + 4, x87_store32(0.0));
        }
    });
}

// Translated from 006aa780 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006aa780 - its FS:[0] exception frame (the unwind state of the LOS map allocation, holder and location) is not translated
/// `NavMeshSearchLOS::BuildNodePath` (Xbox PDB; `RET 0x10`: the request, the
/// start node data, two words for the search): keeps the request and the
/// start node; false when the request's target (`00984230`) has no object.
/// Makes the LOS map (0x50 bytes, `006e1370` with the target's position
/// `006aabd0` and float `00508050`; null when the allocation fails) into
/// `spLOSMap` and fills it (`006e07a0`), clears `pFirstIgnoredGoal`, sets
/// the start node's center, adds it (`006a7c90`; heuristic the distance to
/// the goal node's center when there is one, else 0), finds the goal node
/// from the request's goal location (`006dd6f0`, `006dd640`; with its
/// center), sets `cMinLOSScore` from the map's byte (`004a8f20(byte + 1,
/// -1)`) and returns the search's result (`006a7d60(first, second, 0)`).
pub fn navmesh_search_los_build_node_path(
    e: &mut Engine,
    this: Ptr,
    request: u32,
    start: Ptr,
    first: u32,
    second: u32,
) -> bool {
    let base = this.addr();
    e.with_stack(0xa8, |e, locals| {
        let ebp = locals.addr() + 0xa8;
        e.mem.set_u32(base + CURRENT_REQUEST, request);
        copy_words(e, base + START_NODE, start.addr(), 5);
        let target = e.call(REQUEST_TARGET_00984230, &args![request]).u32();
        if e.call(FIRST_OF_00559450, &args![target]).u32() == 0 {
            return false;
        }
        let block = e.call(ALLOCATE_00AA13E0, &args![0x50u32]).u32();
        let map = if block != 0 {
            let target = e.call(REQUEST_TARGET_00984230, &args![request]).u32();
            let object = e.call(FIRST_OF_00559450, &args![target]).u32();
            let float = e.call(TARGET_VALUE_00508050, &args![object]).f64();
            let float = x87_store32(float);
            let target = e.call(REQUEST_TARGET_00984230, &args![request]).u32();
            let object = e.call(FIRST_OF_00559450, &args![target]).u32();
            let position = fn_006aabd0(e, Ptr::new(object), Ptr::new(ebp - 0x78)).addr();
            let x = e.mem.u32(position);
            let y = e.mem.u32(position + 4);
            let z = e.mem.u32(position + 8);
            e.call(LOS_MAP_CTOR_006E1370, &args![block, x, y, z, float])
                .u32()
        } else {
            0
        };
        e.call(POINTER_SET_0066B0D0, &args![base + LOS_MAP, map]);
        let target = e.call(REQUEST_TARGET_00984230, &args![request]).u32();
        let object = e.call(FIRST_OF_00559450, &args![target]).u32();
        e.call(LOS_MAP_FILL_006E07A0, &args![object, base + LOS_MAP]);
        e.mem.set_u32(base + FIRST_IGNORED_GOAL, 0);
        let location = e.call(REQUEST_START_0048D150, &args![request]).u32();
        e.call(LOCATION_POSITION_005C3420, &args![location, ebp - 0x58]);
        let navmesh = e.mem.u32(start.addr());
        e.mem.set_u32(base + START_NODE, navmesh);
        let triangle = e.mem.u16(start.addr() + 4);
        e.mem.set_u16(base + START_NODE + 4, triangle);
        let navmesh = e.mem.u32(base + START_NODE);
        let triangle = u32::from(e.mem.u16(base + START_NODE + 4));
        center_into(e, navmesh, triangle, ebp - 0x1c, base + START_NODE + 8);
        if e.mem.u8(base + HAS_GOAL_NODE) != 0 {
            let goal = e
                .call(NODE_CENTER_006A6EB0, &args![base + GOAL_NODE, ebp - 0x90])
                .u32();
            let from = e
                .call(NODE_CENTER_006A6EB0, &args![base + START_NODE, ebp - 0x84])
                .u32();
            let difference = e
                .call(POINT3_MINUS_00439EF0, &args![from, ebp - 0x9c, goal])
                .u32();
            let distance = e.call(LENGTH_00589850, &args![difference]).f64();
            e.mem.set_u32(ebp - 0x20, x87_store32(distance));
        } else {
            e.mem.set_u32(ebp - 0x20, x87_store32(0.0));
        }
        let heuristic = x87_quiet_bits(e.mem.u32(ebp - 0x20));
        let navmesh = e.mem.u32(base + START_NODE);
        let triangle = u32::from(e.mem.u16(base + START_NODE + 4));
        let key = e
            .call(NODE_KEY_006A94F0, &args![this, navmesh, triangle])
            .u32();
        e.call(
            ADD_START_NODE_006A7C90,
            &args![this, key, base + START_NODE, heuristic],
        );
        let holder = ebp - 0x24;
        e.call(HOLDER_CTOR_0042FB00, &args![holder]);
        e.mem.set_u8(base + HAS_GOAL_NODE, 0);
        let request = e.mem.u32(base + CURRENT_REQUEST);
        let goal = e.call(REQUEST_GOAL_006A9540, &args![request]).u32();
        let location = ebp - 0x4c;
        e.call(LOCATION_CTOR_006DCC40, &args![location, goal]);
        if e.call(RESOLVE_NAVMESH_INFO_006DD6F0, &args![location, 0u32])
            .u8()
            != 0
            && e.call(
                GET_NAVMESH_AND_TRIANGLE_006DD640,
                &args![location, holder, ebp - 0x10],
            )
            .u8()
                != 0
        {
            e.mem.set_u8(base + HAS_GOAL_NODE, 1);
            let navmesh = e.call(FIRST_OF_00559450, &args![holder]).u32();
            e.mem.set_u32(base + GOAL_NODE, navmesh);
            let triangle = e.mem.u16(ebp - 0x10);
            e.mem.set_u16(base + GOAL_NODE + 4, triangle);
            let navmesh = e.mem.u32(base + GOAL_NODE);
            let triangle = u32::from(e.mem.u16(base + GOAL_NODE + 4));
            center_into(e, navmesh, triangle, ebp - 0x64, base + GOAL_NODE + 8);
        }
        let map = e.call(FIRST_OF_00559450, &args![base + LOS_MAP]).u32();
        let score = fn_006aabf0(e, Ptr::new(map)) as i8;
        let pick = e
            .call(
                PICK_004A8F20,
                &args![i32::from(score).wrapping_add(1), 0xffff_ffffu32],
            )
            .u8();
        e.mem.set_u8(base + MIN_LOS_SCORE, pick);
        let found = e
            .call(SEARCH_006A7D60, &args![this, first, second, 0u32])
            .u8();
        e.call(LOCATION_DTOR_004FF7E0, &args![location]);
        e.call(HOLDER_DTOR_0042FA40, &args![holder]);
        found != 0
    })
}

/// One edge `edge` of `006aacb0`; adds a connection unless the edge is
/// skipped.
fn los_connection(e: &mut Engine, this: u32, ebp: u32, node: u32, connections: u32, edge: u32) {
    let navmesh = e.mem.u32(node + 8);
    let triangle = u32::from(e.mem.u16(node + 0xc));
    let next_navmesh = ebp - 0x28;
    let next_triangle = ebp - 0x20;
    let matched = e
        .call(
            GET_MATCHING_EDGE_0068F460,
            &args![
                navmesh,
                triangle,
                edge,
                next_navmesh,
                next_triangle,
                ebp - 0x24
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
    if e.mem.u32(next_navmesh) != e.mem.u32(node + 8) {
        let key = e
            .call(NAVMESH_KEY_0084E3A0, &args![e.mem.u32(next_navmesh)])
            .u32();
        let map = e.call(FIRST_OF_00559450, &args![this + LOS_MAP]).u32();
        if e.call(MAP_HAS_006E2050, &args![map, key]).u8() == 0 {
            return;
        }
    }
    e.mem.set_u32(ebp - 0x48, x87_store32(1.0));
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
        let request = e.mem.u32(this + CURRENT_REQUEST);
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
        e.mem.set_u32(ebp - 0x50, door);
        let lock = e.call(DOOR_LOCK_004839C0, &args![door]).u32();
        e.mem.set_u32(ebp - 0x4c, lock);
        if lock != 0 {
            let temp_lock = ebp - 0x70;
            e.call(DOOR_LOCK_CTOR_00501D20, &args![temp_lock, lock]);
            let request = e.mem.u32(this + CURRENT_REQUEST);
            let actor = e.call(REQUEST_ACTOR_006C9900, &args![request]).u32();
            e.mem.set_u32(ebp - 0x54, actor);
            let can = e
                .call(
                    CAN_UNLOCK_00502450,
                    &args![temp_lock, actor, lock, ebp - 0x55],
                )
                .u8();
            if can == 0 {
                e.call(DOOR_LOCK_DTOR_00501E50, &args![temp_lock]);
                return;
            }
            let form = e.call(LOCK_FORM_007AF430, &args![lock]).u32();
            if e.call(IS_MINIMAL_USE_00518000, &args![form]).u8() != 0 {
                let factor = x87_quiet_bits(e.global::<u32>(DOOR_COST));
                e.mem.set_u32(ebp - 0x48, factor);
            }
            e.call(DOOR_LOCK_DTOR_00501E50, &args![temp_lock]);
        }
    }
    let request = e.mem.u32(this + CURRENT_REQUEST);
    let range = e.call(REQUEST_RANGE_00979260, &args![request]).f64();
    let zero: f64 = e.global(ZERO);
    if range != zero {
        let request = e.mem.u32(this + CURRENT_REQUEST);
        let point = fn_006ab150(e, Ptr::new(request), Ptr::new(ebp - 0xac)).addr();
        let center = e
            .call(
                GET_CENTER_00558220,
                &args![
                    e.mem.u32(next_navmesh),
                    ebp - 0xa0,
                    u32::from(e.mem.u16(next_triangle))
                ],
            )
            .u32();
        e.call(POINT3_MINUS_00439EF0, &args![center, ebp - 0x7c, point]);
        let squared = e.call(LENGTH_SQUARED_004A7290, &args![ebp - 0x7c]).f64();
        let request = e.mem.u32(this + CURRENT_REQUEST);
        let first = e.call(REQUEST_RANGE_00979260, &args![request]).f64();
        let request = e.mem.u32(this + CURRENT_REQUEST);
        let second = e.call(REQUEST_RANGE_00979260, &args![request]).f64();
        if x87_mul(second, first) < squared {
            return;
        }
    }
    let request = e.mem.u32(this + CURRENT_REQUEST);
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
    let center = ebp - 0x38;
    e.call(
        GET_CENTER_00558220,
        &args![
            e.mem.u32(next_navmesh),
            center,
            u32::from(e.mem.u16(next_triangle))
        ],
    );
    let position = ebp - 0x18;
    let step = e
        .call(POINT3_MINUS_00439EF0, &args![position, ebp - 0xb8, center])
        .u32();
    let length = e.call(LENGTH_00589850, &args![step]).f64();
    e.mem.set_u32(ebp - 0x40, x87_store32(length));
    e.mem.set_u32(ebp - 0x2c, x87_store32(0.0));
    let request = e.mem.u32(this + CURRENT_REQUEST);
    if e.call(AVOID_NODES_0069E390, &args![request]).u32() != 0 {
        let request = e.mem.u32(this + CURRENT_REQUEST);
        let nodes = e.call(AVOID_NODES_0069E390, &args![request]).u32();
        let avoid = e.call(AVOID_COST_006DC990, &args![nodes, center]).f64();
        e.mem.set_u32(ebp - 0x2c, x87_store32(avoid));
    }
    if e.mem.u8(this + HAS_GOAL_NODE) != 0 {
        let goal = e
            .call(NODE_CENTER_006A6EB0, &args![this + GOAL_NODE, ebp - 0xc4])
            .u32();
        let to_goal = ebp - 0x88;
        e.call(POINT3_MINUS_00439EF0, &args![position, to_goal, goal]);
        let to_next = ebp - 0x94;
        e.call(POINT3_MINUS_00439EF0, &args![position, to_next, center]);
        e.call(NORMALIZE_004A0C10, &args![to_goal]);
        e.call(NORMALIZE_004A0C10, &args![to_next]);
        let dot = e.call(DOT_004B6190, &args![to_goal, to_next]).f64();
        let turn = x87_mul(x87_sub(1.0, dot), x87_m32(e.mem.u32(ebp - 0x40)));
        e.mem.set_u32(ebp - 0x40, x87_store32(turn));
        let turn = x87_quiet_bits(e.mem.u32(ebp - 0x40));
        let kept = e.call(MAX_00404010, &args![turn, x87_store32(0.0)]).f64();
        e.mem.set_u32(ebp - 0x40, x87_store32(kept));
    }
    let cost = x87_add(x87_load32(e.mem.u32(node)), x87_m32(e.mem.u32(ebp - 0x40)));
    let factor = x87_add(x87_load32(e.mem.u32(ebp - 0x2c)), 1.0);
    e.mem
        .set_u32(ebp - 0x40, x87_store32(x87_mul(cost, factor)));
    let cost = x87_mul(
        x87_load32(e.mem.u32(ebp - 0x40)),
        x87_m32(e.mem.u32(ebp - 0x48)),
    );
    e.mem.set_u32(ebp - 0x40, x87_store32(cost));
    let cost = x87_quiet_bits(e.mem.u32(ebp - 0x40));
    let next = ebp - 0x3c;
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
            navmesh_search_los_set_node_data(
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

// Translated from 006aacb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006aacb0 - its FS:[0] exception frame (the unwind state of the temporary DoorLock) is not translated
/// `NavMeshSearchLOS::GetNodeConnections` (Xbox PDB; `RET 8`: node,
/// connections): from the node's center (`006a6eb0`), for each of the
/// triangle's three edges with a matching edge (`GetMatchingEdge_ov3`)
/// whose extra info is not of type 1 and whose navmesh the LOS map covers
/// (`006e2050`, checked on a change of navmesh): door triangles and flag
/// 0x10 triangles as in `006aa000`; with a range (`00979260`, not 0) the
/// next triangle's center must be within it of the request's point
/// (`006ab150`; squared distance not above the range squared). The cost is
/// the step's length (turned by `1 - dot` of the directions to the goal and
/// to the next center when there is a goal node, kept non-negative by
/// `00404010`) plus the node's cost, times one plus the avoid nodes' cost,
/// times the door factor; the node for the key (`006a7e00`; a new one
/// through `006aac10`) gets the link and is appended. Returns true.
pub fn navmesh_search_los_get_node_connections(
    e: &mut Engine,
    this: Ptr,
    node: Ptr,
    connections: Ptr,
) -> bool {
    let base = this.addr();
    let node = node.addr();
    e.with_stack(0xe4, |e, locals| {
        let ebp = locals.addr() + 0xe4;
        e.call(NODE_CENTER_006A6EB0, &args![node + 8, ebp - 0x18]);
        let mut edge = 0i32;
        while edge < 3 {
            los_connection(e, base, ebp, node, connections.addr(), edge as u32);
            edge = edge.wrapping_add(1);
        }
        true
    })
}

// Translated from 006ab180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshSearchLOS::IsGoalNode` (Xbox PDB; `RET 4`): the node's LOS score
/// (`006ab3b0` with its navmesh's key `0084e3a0` and triangle). A negative
/// score is a goal unless the node's center is, horizontally
/// (`00595c80`), farther than the request's distance (`00644a50`, squared)
/// from one of the request's points (`006ab360`, elements `006a1440`); then
/// it is the first ignored goal (when none is kept yet) and not a goal. A
/// score other than 0x7F makes the node the last valid one; with 0x7F and
/// no last valid node, a node other than the start becomes the closest node
/// when its squared distance to the map's center (`006ab380`) is below the
/// kept one. Those cases are not goals.
pub fn navmesh_search_los_is_goal_node(e: &mut Engine, this: Ptr, node: Ptr) -> bool {
    let base = this.addr();
    e.with_stack(0x60, |e, locals| {
        let ebp = locals.addr() + 0x60;
        let navmesh = e.mem.u32(node.addr() + 8);
        let triangle = u32::from(e.mem.u16(node.addr() + 0xc));
        let key = e.call(NAVMESH_KEY_0084E3A0, &args![navmesh]).u32();
        let map = e.call(FIRST_OF_00559450, &args![base + LOS_MAP]).u32();
        let score = fn_006ab3b0(e, Ptr::new(map), key, triangle);
        e.mem.set_u8(ebp - 1, score);
        if (e.mem.u8(ebp - 1) as i8) < 0 {
            let request = e.mem.u32(base + CURRENT_REQUEST);
            let points = e.call(REQUEST_POINTS_006AB360, &args![request]).u32();
            let mut index = 0u32;
            while index < e.call(ARRAY_SIZE_0044DDC0, &args![points]).u32() {
                let center = ebp - 0x24;
                e.call(NODE_CENTER_006A6EB0, &args![node.addr() + 8, center]);
                let point = e.call(ELEMENT_006A1440, &args![points, index]).u32();
                e.call(POINT3_MINUS_00439EF0, &args![center, ebp - 0x18, point]);
                let horizontal = e.call(LENGTH_2D_00595C80, &args![ebp - 0x18]).f64();
                let request = e.mem.u32(base + CURRENT_REQUEST);
                let first = e.call(REQUEST_DISTANCE_00644A50, &args![request]).f64();
                let request = e.mem.u32(base + CURRENT_REQUEST);
                let second = e.call(REQUEST_DISTANCE_00644A50, &args![request]).f64();
                if x87_mul(second, first) > horizontal {
                    if e.mem.u32(base + FIRST_IGNORED_GOAL) == 0 {
                        e.mem.set_u32(base + FIRST_IGNORED_GOAL, node.addr());
                    }
                    return false;
                }
                index = index.wrapping_add(1);
            }
            return true;
        }
        if e.mem.u8(ebp - 1) != NO_SCORE {
            e.mem.set_u32(base + LAST_VALID_NODE, node.addr());
            return false;
        }
        if e.mem.u32(base + LAST_VALID_NODE) != 0 {
            return false;
        }
        let data = e.call(PLUS_EIGHT_00413F40, &args![node]).u32();
        if e.mem.u16(data + 4) == e.mem.u16(base + START_NODE + 4) {
            let data = e.call(PLUS_EIGHT_00413F40, &args![node]).u32();
            if e.mem.u32(data) == e.mem.u32(base + START_NODE) {
                return false;
            }
        }
        let data = e.call(PLUS_EIGHT_00413F40, &args![node]).u32();
        let center = e.call(NODE_CENTER_006A6EB0, &args![data, ebp - 0x4c]).u32();
        let map = e.call(FIRST_OF_00559450, &args![base + LOS_MAP]).u32();
        let map_center = e.call(MAP_CENTER_006AB380, &args![map, ebp - 0x40]).u32();
        e.call(
            POINT3_MINUS_00439EF0,
            &args![map_center, ebp - 0x30, center],
        );
        let squared = e.call(LENGTH_SQUARED_004A7290, &args![ebp - 0x30]).f64();
        e.mem.set_u32(ebp - 0x34, x87_store32(squared));
        let closest = x87_load32(e.mem.u32(base + CLOSEST_NODE_DISTANCE_SQUARED));
        if closest > x87_load32(e.mem.u32(ebp - 0x34)) {
            e.mem.set_u32(base + CLOSEST_NODE, node.addr());
            let squared = x87_quiet_bits(e.mem.u32(ebp - 0x34));
            e.mem.set_u32(base + CLOSEST_NODE_DISTANCE_SQUARED, squared);
        }
        false
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006a_abf0, fn_006aabf0(Ptr) -> u8),
        entry!(0x006a_a710, fn_006aa710(Ptr)),
        entry!(
            0x006a_a6e0,
            navmesh_search_los_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x006a_abd0, fn_006aabd0(Ptr, Ptr) -> Ptr),
        entry!(0x006a_b150, fn_006ab150(Ptr, Ptr) -> Ptr),
        entry!(0x006a_b3b0, fn_006ab3b0(Ptr, u32, u32) -> u8),
        entry!(
            0x006a_ac10,
            navmesh_search_los_set_node_data(Ptr, Ptr, u32, u32)
        ),
        entry!(
            0x006a_a780,
            navmesh_search_los_build_node_path(Ptr, u32, Ptr, u32, u32) -> bool
        ),
        entry!(
            0x006a_acb0,
            navmesh_search_los_get_node_connections(Ptr, Ptr, Ptr) -> bool
        ),
        entry!(0x006a_b180, navmesh_search_los_is_goal_node(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_006aabf0_returns_the_byte_at_0x39() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        e.mem.set_u8(this + FLAG_BYTE, 0xa5);
        assert_eq!(e.call(0x006a_abf0, &args![this]).u32() & 0xff, 0xa5);
        e.mem.set_u8(this + FLAG_BYTE, 0);
        assert_eq!(e.call(0x006a_abf0, &args![this]).u32() & 0xff, 0);
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

    /// The center of triangle `t` is `(t, 0, 0)`; vector helpers on floats.
    fn geometry(e: &mut Engine) {
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
        e.register(LENGTH_00589850, |e, a| {
            let (x, y, z) = (e.mem.f32(a[0]), e.mem.f32(a[0] + 4), e.mem.f32(a[0] + 8));
            f64::from((x * x + y * y + z * z).sqrt()).into_ret()
        });
        e.register(LENGTH_SQUARED_004A7290, |e, a| {
            let (x, y, z) = (e.mem.f32(a[0]), e.mem.f32(a[0] + 4), e.mem.f32(a[0] + 8));
            f64::from(x * x + y * y + z * z).into_ret()
        });
        e.register(LENGTH_2D_00595C80, |e, a| {
            let (x, y) = (e.mem.f32(a[0]), e.mem.f32(a[0] + 4));
            f64::from((x * x + y * y).sqrt()).into_ret()
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
        e.register(FIRST_OF_00559450, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(PLUS_EIGHT_00413F40, |_, a| (a[0] + 8).into_ret());
        e.map(0x0101_0000, 0x10_0000);
        e.map(0x011d_6000, 0x1000);
    }

    #[test]
    fn at_006aa710_and_006aa6e0_destroy_the_los_search() {
        let mut e = Engine::new();
        stub(
            &mut e,
            &[
                POINTER_RELEASE_0045CEC0,
                ASTAR_DTOR_006A7BC0,
                OPERATOR_DELETE,
            ],
        );
        let this = e.mem.alloc(0x20a0);
        let log = logged(&mut e, |e| {
            e.call(0x006a_a710, &args![this]);
        });
        assert_eq!(e.mem.u32(this), SEARCH_LOS_VTABLE);
        assert_eq!(
            calls_to(&log, POINTER_RELEASE_0045CEC0),
            vec![vec![this + LOS_MAP]]
        );
        assert_eq!(calls_to(&log, ASTAR_DTOR_006A7BC0), vec![vec![this]]);
        for flags in [0u32, 1] {
            let log = logged(&mut e, |e| {
                assert_eq!(e.call(0x006a_a6e0, &args![this, flags]).u32(), this);
            });
            assert_eq!(calls_to(&log, ASTAR_DTOR_006A7BC0).len(), 1);
            assert_eq!(calls_to(&log, OPERATOR_DELETE).len(), flags as usize);
        }
    }

    #[test]
    fn at_006aabd0_and_006ab150_copy_their_points() {
        let mut e = Engine::new();
        geometry(&mut e);
        let this = e.mem.alloc(0xd4);
        e.mem.set_f32(this + 0x1c, 5.0);
        e.mem.set_f32(this + 0xcc, 6.0);
        let out = e.mem.alloc(0xc);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x006a_abd0, &args![this, out]).u32(), out);
        });
        assert_eq!(
            calls_to(&log, LOCATION_POSITION_005C3420),
            vec![vec![this + 0x1c, out]]
        );
        assert_eq!(e.mem.f32(out), 5.0);
        assert_eq!(e.call(0x006a_b150, &args![this, out]).u32(), out);
        assert_eq!(e.mem.f32(out + 4), 6.0);
    }

    #[test]
    fn at_006ab3b0_reads_the_score() {
        let mut e = Engine::new();
        e.register(MAP_LOOKUP_00853130, |e, a| {
            if a[1] != 7 {
                return Ret::default();
            }
            e.mem.set_u32(a[2], 2);
            1u32.into_ret()
        });
        let this = e.mem.alloc(0x30);
        let buffer = e.mem.alloc(8);
        e.mem.set_u8(buffer + 5, 0x42);
        e.mem.set_u32(this + 0x18, buffer);
        e.mem.set_u32(this + 0x2c, 6);
        assert_eq!(
            e.call(0x006a_b3b0, &args![this, 7u32, 0x0001_0003u32])
                .u32(),
            0x42
        );
        assert_eq!(e.call(0x006a_b3b0, &args![this, 7u32, 4u32]).u32(), 0x7f);
        assert_eq!(e.call(0x006a_b3b0, &args![this, 8u32, 0u32]).u32(), 0x7f);
    }

    #[test]
    fn at_006aac10_sets_the_node_with_or_without_a_goal() {
        let mut e = Engine::new();
        geometry(&mut e);
        let this = e.mem.alloc(0x20a0);
        let node = e.mem.alloc(0x3c);
        e.call(0x006a_ac10, &args![this, node, 0x5000u32, 3u32]);
        assert_eq!((e.mem.u32(node + 8), e.mem.u16(node + 0xc)), (0x5000, 3));
        assert_eq!(e.mem.f32(node + 0x10), 3.0);
        assert_eq!(e.mem.f32(node + 4), 0.0);
        e.mem.set_u8(this + HAS_GOAL_NODE, 1);
        e.mem.set_f32(this + GOAL_NODE + 8, 7.0);
        e.call(0x006a_ac10, &args![this, node, 0x5000u32, 3u32]);
        assert_eq!(e.mem.f32(node + 4), 4.0);
    }

    #[test]
    fn at_006aa780_builds_the_los_map_and_searches() {
        let mut e = Engine::new();
        geometry(&mut e);
        let holder = e.mem.alloc(4);
        e.register_double(REQUEST_TARGET_00984230, move |_, _| holder.into_ret());
        let start = e.mem.alloc(0x14);
        e.mem.set_u32(start, 0x5000);
        e.mem.set_u16(start + 4, 2);
        let this = e.mem.alloc(0x20a0);
        // No target object: false at once.
        assert!(!e
            .call(0x006a_a780, &args![this, 0x4444u32, start, 1u32, 2u32])
            .bool());
        let target = e.mem.alloc(0x40);
        e.mem.set_u32(holder, target);
        e.register(ALLOCATE_00AA13E0, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(TARGET_VALUE_00508050, |_, _| 3.0f64.into_ret());
        e.register(LOS_MAP_CTOR_006E1370, |_, a| a[0].into_ret());
        stub(
            &mut e,
            &[
                POINTER_SET_0066B0D0,
                LOS_MAP_FILL_006E07A0,
                ADD_START_NODE_006A7C90,
                HOLDER_CTOR_0042FB00,
                HOLDER_DTOR_0042FA40,
                LOCATION_CTOR_006DCC40,
                LOCATION_DTOR_004FF7E0,
                REQUEST_GOAL_006A9540,
                RESOLVE_NAVMESH_INFO_006DD6F0,
            ],
        );
        e.register_double(REQUEST_START_0048D150, move |_, _| target.into_ret());
        e.register(NODE_KEY_006A94F0, |_, a| (a[1] + a[2]).into_ret());
        e.register(PICK_004A8F20, |_, a| (a[0] as u8).into_ret());
        e.register(SEARCH_006A7D60, |_, _| 1u32.into_ret());
        e.register(POINTER_SET_0066B0D0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        let log = logged(&mut e, |e| {
            assert!(e
                .call(0x006a_a780, &args![this, 0x4444u32, start, 1u32, 2u32])
                .bool());
        });
        let made = calls_to(&log, LOS_MAP_CTOR_006E1370);
        assert_eq!(made.len(), 1);
        assert_eq!(made[0][4], 3.0f32.to_bits());
        assert_eq!(e.mem.u32(this + START_NODE), 0x5000);
        assert_eq!(e.mem.f32(this + START_NODE + 8), 2.0);
        assert_eq!(e.mem.u8(this + HAS_GOAL_NODE), 0);
        assert_eq!(e.mem.u32(this + FIRST_IGNORED_GOAL), 0);
        // The map's byte (+0x39, 0) plus one picked against -1.
        assert_eq!(calls_to(&log, PICK_004A8F20), vec![vec![1, 0xffff_ffff]]);
        assert_eq!(e.mem.u8(this + MIN_LOS_SCORE), 1);
        assert_eq!(calls_to(&log, SEARCH_006A7D60), vec![vec![this, 1, 2, 0]]);
        assert_eq!(calls_to(&log, ADD_START_NODE_006A7C90)[0][1], 0x5002);
    }

    #[test]
    fn at_006aacb0_connects_edges_the_map_covers() {
        let mut e = Engine::new();
        geometry(&mut e);
        e.register(GET_MATCHING_EDGE_0068F460, |e, a| {
            if a[2] != 2 {
                return Ret::default();
            }
            e.mem.set_u32(a[3], 0x5000);
            e.mem.set_u16(a[4], 6);
            1u32.into_ret()
        });
        stub(
            &mut e,
            &[
                GET_EDGE_EXTRA_INFO_0068F230,
                TRIANGLE_005582F0,
                TRIANGLE_HAS_FLAG_00691140,
                AVOID_NODES_0069E390,
                REQUEST_RANGE_00979260,
                ARRAY_PUSH_007CB2E0,
                NAVMESH_KEY_0084E3A0,
            ],
        );
        e.register(REQUEST_VALUE_0069EF80, |_, _| 1.0f64.into_ret());
        e.register(SETTING_VALUE_00403E20, |_, _| 0x0101_0000u32.into_ret());
        e.set_global::<f32>(0x0101_0000, 5.0);
        e.register(NODE_KEY_006A94F0, |_, a| (a[1] + a[2]).into_ret());
        e.register(NODE_FOR_KEY_006A7E00, |e, a| {
            let node = e.mem.alloc(0x3c);
            e.mem.set_u32(a[3], node);
            1u32.into_ret()
        });
        let this = e.mem.alloc(0x20a0);
        let node = e.mem.alloc(0x3c);
        e.mem.set_u32(node + 8, 0x4000);
        e.mem.set_f32(node + 0x10, 2.0);
        e.mem.set_f32(node, 1.0);
        let connections = e.mem.alloc(0x10);
        // The map does not cover the other navmesh: nothing.
        e.register(MAP_HAS_006E2050, |_, _| Ret::default());
        let log = logged(&mut e, |e| {
            assert!(e.call(0x006a_acb0, &args![this, node, connections]).bool());
        });
        assert!(calls_to(&log, ARRAY_PUSH_007CB2E0).is_empty());
        e.register(MAP_HAS_006E2050, |_, _| 1u32.into_ret());
        let log = logged(&mut e, |e| {
            e.call(0x006a_acb0, &args![this, node, connections]);
        });
        let found = calls_to(&log, NODE_FOR_KEY_006A7E00);
        assert_eq!(found.len(), 1);
        // (cost 1 + step 4) * (avoid 0 + 1) * door factor 1.
        assert_eq!(found[0][2], 5.0f32.to_bits());
        assert_eq!(calls_to(&log, ARRAY_PUSH_007CB2E0)[0][0], connections);
    }

    #[test]
    fn at_006ab180_sorts_the_nodes_by_score() {
        let mut e = Engine::new();
        geometry(&mut e);
        stub(&mut e, &[NAVMESH_KEY_0084E3A0]);
        let map = e.mem.alloc(0x30);
        let scores = e.mem.alloc(0x10);
        e.mem.set_u32(map + 0x18, scores);
        e.mem.set_u32(map + 0x2c, 0x10);
        e.register(MAP_LOOKUP_00853130, |e, a| {
            e.mem.set_u32(a[2], 0);
            1u32.into_ret()
        });
        let this = e.mem.alloc(0x20a0);
        e.mem.set_u32(this + LOS_MAP, map);
        let points = e.mem.alloc(0x10);
        e.register_double(REQUEST_POINTS_006AB360, move |_, _| points.into_ret());
        e.register(ARRAY_SIZE_0044DDC0, |e, a| e.mem.u32(a[0] + 8).into_ret());
        let node = e.mem.alloc(0x3c);
        e.mem.set_u16(node + 0xc, 3);
        // A negative score with no points: a goal.
        e.mem.set_u8(scores + 3, 0xff);
        assert!(e.call(0x006a_b180, &args![this, node]).bool());
        // A point farther than the distance: the first ignored goal.
        let point = e.mem.alloc(0xc);
        e.mem.set_f32(point, 50.0);
        e.register_double(ELEMENT_006A1440, move |_, _| point.into_ret());
        e.mem.set_u32(points + 8, 1);
        e.register(REQUEST_DISTANCE_00644A50, |_, _| 10.0f64.into_ret());
        assert!(!e.call(0x006a_b180, &args![this, node]).bool());
        assert_eq!(e.mem.u32(this + FIRST_IGNORED_GOAL), node);
        // A score other than 0x7F: the last valid node.
        e.mem.set_u8(scores + 3, 5);
        assert!(!e.call(0x006a_b180, &args![this, node]).bool());
        assert_eq!(e.mem.u32(this + LAST_VALID_NODE), node);
        // 0x7F without a valid node: the closest node when nearer.
        e.mem.set_u8(scores + 3, 0x7f);
        e.mem.set_u32(this + LAST_VALID_NODE, 0);
        e.mem.set_f32(this + CLOSEST_NODE_DISTANCE_SQUARED, 100.0);
        e.mem.set_f32(node + 0x10, 4.0);
        e.register(MAP_CENTER_006AB380, |e, a| {
            e.mem.set_f32(a[1], 1.0);
            e.mem.set_f32(a[1] + 4, 0.0);
            e.mem.set_f32(a[1] + 8, 0.0);
            a[1].into_ret()
        });
        assert!(!e.call(0x006a_b180, &args![this, node]).bool());
        assert_eq!(e.mem.u32(this + CLOSEST_NODE), node);
        assert_eq!(e.mem.f32(this + CLOSEST_NODE_DISTANCE_SQUARED), 9.0);
    }
}
