//! `fallout shared/pathfinding/navmeshsearchmaxcost.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006ab480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `0106b87c` (RTTI `.?AVNavMeshSearchMaxCost@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `006a6b00`. One word, plain `RET`, returns nothing.
pub fn fn_006ab480(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0106_b87c);
    e.call(0x006a_6b00, &args![this]);
}

/// Vtable of `BSSimpleArray<NavMeshSearchNode,1024>` (RTTI
/// `.?AV?$BSSimpleArray@UNavMeshSearchNode@@$0EAA@@@`).
const SIMPLE_ARRAY_SEARCH_NODE_VTABLE: u32 = 0x0106_b8a0;
/// Vtable of `BSScrapArray<NavMeshSearchNode,1024>` (RTTI
/// `.?AV?$BSScrapArray@UNavMeshSearchNode@@$0EAA@@@`).
const SCRAP_ARRAY_SEARCH_NODE_VTABLE: u32 = 0x0106_b88c;
/// The array's release (`008454f0`, thiscall, one word: 1 here); unnamed in
/// the engine map.
const ARRAY_RELEASE_008454F0: u32 = 0x0084_54f0;
/// `ScrapHeap::Allocate` (`00aa54a0`, engine map): thiscall on the scrap
/// heap, size and alignment.
const SCRAP_HEAP_ALLOCATE: u32 = 0x00aa_54a0;
/// The scrap-array allocation alignment the code reads (`010a2720`).
const SCRAP_ALIGNMENT: u32 = 0x010a_2720;
/// `BSScrapArray`'s scrap heap pointer (+0x10, as the code reads it).
const SCRAP_HEAP: u32 = 0x10;
/// Size of a `NavMeshSearchNode` (the code's `IMUL ECX,ECX,0x14`).
const SEARCH_NODE_SIZE: u32 = 0x14;

// Translated from 006abc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NavMeshSearchNode,1024>`'s destructor body (by the
/// vtable it stores; unnamed in the engine map): the class's vtable, then
/// `008454f0(this, 1)`.
pub fn fn_006abc30(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), SIMPLE_ARRAY_SEARCH_NODE_VTABLE);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

// Translated from 006abc50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006abc50 - the function's C++ exception frame (FS:[0], unwind state) is not translated
/// `BSScrapArray<NavMeshSearchNode,1024>`'s destructor body (by the vtable
/// it stores): the scrap array's vtable, `008454f0(this, 1)`, then the
/// base `BSSimpleArray` destructor body (`006abc30`).
pub fn fn_006abc50(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), SCRAP_ARRAY_SEARCH_NODE_VTABLE);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
    fn_006abc30(e, this);
}

// Translated from 006abcb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapArray<NavMeshSearchNode_1024>::_Allocate` (engine map): `count`
/// nodes of 0x14 bytes from the array's scrap heap (+0x10) through
/// `ScrapHeap::Allocate`, aligned as the global at `010a2720` says.
pub fn bs_scrap_array_navmesh_search_node_allocate(e: &mut Engine, this: Ptr, count: u32) -> u32 {
    let alignment: u32 = e.global(SCRAP_ALIGNMENT);
    let heap = e.mem.u32(this.addr() + SCRAP_HEAP);
    e.call(
        SCRAP_HEAP_ALLOCATE,
        &args![heap, count.wrapping_mul(SEARCH_NODE_SIZE), alignment],
    )
    .u32()
}

/// `operator delete` (`00401030`, cdecl, one word).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `NavMeshSearch::IsGoalNode` (`006a79c0`, engine map; `RET 4`).
const NAVMESH_SEARCH_IS_GOAL_NODE: u32 = 0x006a_79c0;
/// `006abf60`, `006abf90` (unnamed in the engine map): the
/// `BSSimpleArray<NavMeshSearchNode,1024>` and
/// `BSSimpleArray<NavMeshSearchLink,1024>` base constructors.
const NODE_ARRAY_CTOR_006ABF60: u32 = 0x006a_bf60;
const LINK_ARRAY_CTOR_006ABF90: u32 = 0x006a_bf90;
/// `006abdc0`, `006abde0` (unnamed in the engine map): the destructor
/// bodies of `BSSimpleArray<NavMeshSearchLink,1024>` and
/// `BSScrapArray<NavMeshSearchLink,1024>`.
const LINK_ARRAY_DTOR_006ABDC0: u32 = 0x006a_bdc0;
const LINK_SCRAP_ARRAY_DTOR_006ABDE0: u32 = 0x006a_bde0;
/// Vtable of `BSScrapArray<NavMeshSearchLink,1024>` (`0106b8b4`, stored by
/// its constructor).
const SCRAP_ARRAY_SEARCH_LINK_VTABLE: u32 = 0x0106_b8b4;
/// `00401020` (unnamed in the engine map): the scrap-heap holder.
const SCRAP_HOLDER_00401020: u32 = 0x0040_1020;
/// `00aa42e0` (unnamed in the engine map): the scrap heap of the holder.
const SCRAP_HEAP_00AA42E0: u32 = 0x00aa_42e0;
/// `009a3d00`, `006b3f40` (unnamed in the engine map; `RET 8`: reserve,
/// size): the arrays' initializations.
const NODE_ARRAY_INIT_009A3D00: u32 = 0x009a_3d00;
const LINK_ARRAY_INIT_006B3F40: u32 = 0x006b_3f40;
/// `00401460` (unnamed in the engine map; cdecl: destination, source,
/// bytes): a copy.
const COPY_00401460: u32 = 0x0040_1460;
/// `ScrapHeap::Deallocate` (`00aa5610`, engine map; `RET 4`).
const SCRAP_HEAP_DEALLOCATE: u32 = 0x00aa_5610;
/// `0044ddc0` (unnamed in the engine map): a `BSSimpleArray`'s size.
const ARRAY_SIZE_0044DDC0: u32 = 0x0044_ddc0;
/// `NavMesh::GetCenter` (`00558220`, engine map; `RET 8`: out, triangle).
const GET_CENTER_00558220: u32 = 0x0055_8220;
/// `006a6eb0` (navmeshsearch unit; `RET 4`: out): a search node's center
/// into `out`; returns `out`.
const NODE_CENTER_006A6EB0: u32 = 0x006a_6eb0;
/// `00439ef0` (unnamed in the engine map; `RET 8`: out, other): the 3D
/// difference `this - other` into `out`; returns `out`.
const POINT3_MINUS_00439EF0: u32 = 0x0043_9ef0;
/// `00457990` (unnamed in the engine map): a 3D vector's length (ST0).
const POINT3_LENGTH_00457990: u32 = 0x0045_7990;
/// `00589850` (unnamed in the engine map): a 3D vector's length (ST0).
const LENGTH_00589850: u32 = 0x0058_9850;
/// `006a94f0` (unnamed in the engine map; `RET 8`: navmesh, triangle): a
/// node key.
const NODE_KEY_006A94F0: u32 = 0x006a_94f0;
/// `AStarSearch<NavMeshSearchNode,NavMeshSearchLink>::AddStartNode`
/// (`006a7c90`, decompiler name; `RET 0xC`).
const ADD_START_NODE_006A7C90: u32 = 0x006a_7c90;
/// `AStarSearch<NavMeshSearchNode,NavMeshSearchLink>::Search` (`006a7d60`,
/// decompiler name; `RET 0xC`).
const SEARCH_006A7D60: u32 = 0x006a_7d60;
/// `006a7c40` (navmeshsearch unit): resets the search.
const SEARCH_RESET_006A7C40: u32 = 0x006a_7c40;
/// `AStarSearch<NavMeshSearchNode,NavMeshSearchLink>::GetOrCreateAStarNode`
/// (`006a7e00`, decompiler name; `RET 0xC`: key, cost, out node).
const NODE_FOR_KEY_006A7E00: u32 = 0x006a_7e00;
/// `006a6ef0` (navmeshsearch unit; `RET 0xC`): `NavMeshSearch`'s node
/// data set-up (node, navmesh, triangle).
const SET_NODE_DATA_006A6EF0: u32 = 0x006a_6ef0;
/// `006a9520` (unnamed in the engine map): a flag of a `PathingLocation`
/// (AL).
const LOCATION_FLAG_006A9520: u32 = 0x006a_9520;
/// `PathingLocation::ResolveNavMeshInfo` (`006dd6f0`, engine map; `RET 4`).
const RESOLVE_NAVMESH_INFO_006DD6F0: u32 = 0x006d_d6f0;
/// `PathingLocation::GetNavMeshAndTriangle` (`006dd640`, engine map; `RET 8`).
const GET_NAVMESH_AND_TRIANGLE_006DD640: u32 = 0x006d_d640;
/// `0042fb00`, `0042fa40` (unnamed in the engine map): the navmesh holder's
/// constructor and destructor.
const HOLDER_CTOR_0042FB00: u32 = 0x0042_fb00;
const HOLDER_DTOR_0042FA40: u32 = 0x0042_fa40;
/// `00559450` (unnamed in the engine map): the word at +0 of its `this`.
const FIRST_OF_00559450: u32 = 0x0055_9450;
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
/// `BSSimpleArray<BGSBodyPart*,1024>::AddUninitialized` (`007cb2e0`, engine
/// map name of the folded body; `RET 4`).
const ARRAY_PUSH_007CB2E0: u32 = 0x007c_b2e0;

/// `NavMeshSearch` and `NavMeshSearchMaxCost` fields (Xbox PDB names, PC
/// offsets).
const BEST_NODE_SO_FAR: u32 = 0x2048;
const CURRENT_REQUEST: u32 = 0x2058;
const START_NODE: u32 = 0x205c;
const GOAL_NODE: u32 = 0x2070;
const MAX_DIST: u32 = 0x20a4;
const HAS_GOAL_NODE: u32 = 0x20a8;

fn copy_words(e: &mut Engine, to: u32, from: u32, count: u32) {
    for i in 0..count {
        let word = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, word);
    }
}

fn delete_if(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// `NavMesh::GetCenter` into `temp`, copied through the FPU to `to`.
fn center_into(e: &mut Engine, navmesh: u32, triangle: u32, temp: u32, to: u32) {
    let center = e
        .call(GET_CENTER_00558220, &args![navmesh, temp, triangle])
        .u32();
    for i in 0..3 {
        let word = x87_quiet_bits(e.mem.u32(center + 4 * i));
        e.mem.set_u32(to + 4 * i, word);
    }
}

/// The distance from the start node's center to the goal node's
/// (`006a6eb0` into `goal_temp` and `start_temp`, `00439ef0` into
/// `difference`, `00457990`), stored as a float.
fn start_to_goal(
    e: &mut Engine,
    this: u32,
    goal_temp: u32,
    difference: u32,
    start_temp: u32,
) -> u32 {
    let goal = e
        .call(NODE_CENTER_006A6EB0, &args![this + GOAL_NODE, goal_temp])
        .u32();
    let start = e
        .call(NODE_CENTER_006A6EB0, &args![this + START_NODE, start_temp])
        .u32();
    let delta = e
        .call(POINT3_MINUS_00439EF0, &args![start, difference, goal])
        .u32();
    x87_store32(e.call(POINT3_LENGTH_00457990, &args![delta]).f64())
}

// Translated from 006ab450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshSearchMaxCost::scalar deleting destructor` (Xbox PDB):
/// `006ab480`, then `operator delete` when bit 0 of `flags` is set.
pub fn navmesh_search_max_cost_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_006ab480(e, this);
    delete_if(e, this, flags)
}

// Translated from 006abb60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshSearchMaxCost::IsGoalNode` (Xbox PDB; `RET 4`): false without a
/// goal node (`bHasGoalNode` +0x20A8), else `NavMeshSearch::IsGoalNode`.
pub fn navmesh_search_max_cost_is_goal_node(e: &mut Engine, this: Ptr, node: Ptr) -> bool {
    if e.mem.u8(this.addr() + HAS_GOAL_NODE) == 0 {
        return false;
    }
    e.call(NAVMESH_SEARCH_IS_GOAL_NODE, &args![this, node])
        .bool()
}

// Translated from 006abb90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `BestNodeSoFar` (+0x2048).
pub fn fn_006abb90(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + BEST_NODE_SO_FAR)
}

// Translated from 006abbb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006abbb0 - its FS:[0] exception frame (the unwind state of the base construction) is not translated
/// The constructor of `BSScrapArray<NavMeshSearchNode,1024>` (by the vtable
/// it stores): the `BSSimpleArray` base (`006abf60`), its own vtable, the
/// scrap heap at +0x10 (`00aa42e0` on the holder `00401020` returns), then
/// `009a3d00(this, 0, 0)`. Returns `this`.
pub fn fn_006abbb0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(NODE_ARRAY_CTOR_006ABF60, &args![this]);
    e.mem.set_u32(this.addr(), SCRAP_ARRAY_SEARCH_NODE_VTABLE);
    let holder = e.call(SCRAP_HOLDER_00401020, &args![]).u32();
    let heap = e.call(SCRAP_HEAP_00AA42E0, &args![holder]).u32();
    e.mem.set_u32(this.addr() + SCRAP_HEAP, heap);
    e.call(NODE_ARRAY_INIT_009A3D00, &args![this, 0u32, 0u32]);
    this
}

// Translated from 006abd40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006abd40 - its FS:[0] exception frame (the unwind state of the base construction) is not translated
/// The constructor of `BSScrapArray<NavMeshSearchLink,1024>` (by the vtable
/// it stores): the `BSSimpleArray` base (`006abf90`), its own vtable, the
/// scrap heap at +0x10, then `006b3f40(this, 0, 0)`. Returns `this`.
pub fn fn_006abd40(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(LINK_ARRAY_CTOR_006ABF90, &args![this]);
    e.mem.set_u32(this.addr(), SCRAP_ARRAY_SEARCH_LINK_VTABLE);
    let holder = e.call(SCRAP_HOLDER_00401020, &args![]).u32();
    let heap = e.call(SCRAP_HEAP_00AA42E0, &args![holder]).u32();
    e.mem.set_u32(this.addr() + SCRAP_HEAP, heap);
    e.call(LINK_ARRAY_INIT_006B3F40, &args![this, 0u32, 0u32]);
    this
}

// Translated from 006abce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapArray<NavMeshSearchNode,1024>::_Reallocate` (Xbox PDB; `RET 8`):
/// a new buffer of `count` 0x14-byte nodes from the scrap heap (+0x10;
/// alignment word at `010a2720`), the current size's nodes copied from
/// `old` (`00401460`), `old` freed (`ScrapHeap::Deallocate`). Returns the
/// new buffer.
pub fn bs_scrap_array_navmesh_search_node_reallocate(
    e: &mut Engine,
    this: Ptr,
    old: u32,
    count: u32,
) -> u32 {
    let alignment: u32 = e.global(SCRAP_ALIGNMENT);
    let heap = e.mem.u32(this.addr() + SCRAP_HEAP);
    let buffer = e
        .call(
            SCRAP_HEAP_ALLOCATE,
            &args![heap, count.wrapping_mul(SEARCH_NODE_SIZE), alignment],
        )
        .u32();
    let size = e.call(ARRAY_SIZE_0044DDC0, &args![this]).u32();
    e.call(
        COPY_00401460,
        &args![buffer, old, size.wrapping_mul(SEARCH_NODE_SIZE)],
    );
    let heap = e.mem.u32(this.addr() + SCRAP_HEAP);
    e.call(SCRAP_HEAP_DEALLOCATE, &args![heap, old]);
    buffer
}

// Translated from 006abea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NavMeshSearchNode,1024>::scalar deleting destructor` (Xbox
/// PDB): `006abc30`, then `operator delete` when bit 0 of `flags` is set.
pub fn bs_simple_array_navmesh_search_node_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_006abc30(e, this);
    delete_if(e, this, flags)
}

// Translated from 006abed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NavMeshSearchLink,1024>::scalar deleting destructor` (Xbox
/// PDB): `006abdc0`, then `operator delete` when bit 0 of `flags` is set.
pub fn bs_simple_array_navmesh_search_link_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(LINK_ARRAY_DTOR_006ABDC0, &args![this]);
    delete_if(e, this, flags)
}

// Translated from 006abf00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapArray<NavMeshSearchNode,1024>::scalar deleting destructor` (Xbox
/// PDB): `006abc50`, then `operator delete` when bit 0 of `flags` is set.
pub fn bs_scrap_array_navmesh_search_node_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_006abc50(e, this);
    delete_if(e, this, flags)
}

// Translated from 006abf30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapArray<NavMeshSearchLink,1024>::scalar deleting destructor` (Xbox
/// PDB): `006abde0`, then `operator delete` when bit 0 of `flags` is set.
pub fn bs_scrap_array_navmesh_search_link_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(LINK_SCRAP_ARRAY_DTOR_006ABDE0, &args![this]);
    delete_if(e, this, flags)
}

// Translated from 006ab4a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006ab4a0 - its FS:[0] exception frame (the unwind state of the navmesh holder) is not translated
/// `NavMeshSearchMaxCost::BuildNodePath` (Xbox PDB; `RET 0x14`: the request,
/// the start node data, the goal location, the node and link arrays):
/// keeps the start node and request, empties both arrays (`008454f0` with
/// 0), resets the search (`006a7c40`); a goal location without the flag
/// `006a9520` that resolves (`006dd6f0`) and gives a navmesh and triangle
/// (`006dd640`) becomes the goal node (with its center; `bHasGoalNode` set).
/// Adds the start node (`006a7c90` with the start data, heuristic the
/// distance between the start and goal nodes' centers) and returns the
/// search's result (`006a7d60(nodes, links, 0)`).
pub fn navmesh_search_max_cost_build_node_path(
    e: &mut Engine,
    this: Ptr,
    request: u32,
    start: Ptr,
    goal: u32,
    nodes: u32,
    links: u32,
) -> bool {
    let base = this.addr();
    e.with_stack(0x4c, |e, locals| {
        let ebp = locals.addr() + 0x4c;
        copy_words(e, base + START_NODE, start.addr(), 5);
        e.mem.set_u32(base + CURRENT_REQUEST, request);
        e.call(ARRAY_RELEASE_008454F0, &args![nodes, 0u32]);
        e.call(ARRAY_RELEASE_008454F0, &args![links, 0u32]);
        e.call(SEARCH_RESET_006A7C40, &args![this]);
        if e.call(LOCATION_FLAG_006A9520, &args![goal]).u8() == 0
            && e.call(RESOLVE_NAVMESH_INFO_006DD6F0, &args![goal, 0u32])
                .u8()
                != 0
        {
            let holder = ebp - 0x18;
            e.call(HOLDER_CTOR_0042FB00, &args![holder]);
            if e.call(
                GET_NAVMESH_AND_TRIANGLE_006DD640,
                &args![goal, holder, ebp - 0x14],
            )
            .u8()
                != 0
            {
                let navmesh = e.call(FIRST_OF_00559450, &args![holder]).u32();
                e.mem.set_u32(base + GOAL_NODE, navmesh);
                let triangle = e.mem.u16(ebp - 0x14);
                e.mem.set_u16(base + GOAL_NODE + 4, triangle);
                let navmesh = e.mem.u32(base + GOAL_NODE);
                let triangle = u32::from(e.mem.u16(base + GOAL_NODE + 4));
                center_into(e, navmesh, triangle, ebp - 0x24, base + GOAL_NODE + 8);
                e.mem.set_u8(base + HAS_GOAL_NODE, 1);
            }
            e.call(HOLDER_DTOR_0042FA40, &args![holder]);
        }
        let heuristic = start_to_goal(e, base, ebp - 0x3c, ebp - 0x48, ebp - 0x30);
        let navmesh = e.mem.u32(start.addr());
        let triangle = u32::from(e.mem.u16(start.addr() + 4));
        let key = e
            .call(NODE_KEY_006A94F0, &args![this, navmesh, triangle])
            .u32();
        e.call(ADD_START_NODE_006A7C90, &args![this, key, start, heuristic]);
        e.call(SEARCH_006A7D60, &args![this, nodes, links, 0u32])
            .u8()
            != 0
    })
}

// Translated from 006ab670 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006ab670 - its FS:[0] exception frame (the unwind state of the local scrap arrays) is not translated
/// Unnamed in the engine map (`RET 0x18`: out, an unread word, the start
/// node data, a point, the goal node data, the maximum distance): sets the
/// start node (with its center), adds it (`006a7c90`, heuristic the
/// distance to the goal node's center), takes the goal node unless it is
/// null with triangle 0xFFFF (`bHasGoalNode`, with its center) and
/// `fMaxDist`, and searches (`006a7d60` with local scrap arrays, `006abbb0`
/// and `006abd40`). Found: `point` is copied to `out`; not found: the
/// center of `BestNodeSoFar` (`006abb90`, `006a6eb0`). Returns `out`.
pub fn fn_006ab670(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
    _unused_0c: u32,
    start: Ptr,
    point: Ptr,
    goal: Ptr,
    max_dist: f32,
) -> Ptr {
    let base = this.addr();
    e.with_stack(0x74, |e, locals| {
        let ebp = locals.addr() + 0x74;
        let navmesh = e.mem.u32(start.addr());
        e.mem.set_u32(base + START_NODE, navmesh);
        let triangle = e.mem.u16(start.addr() + 4);
        e.mem.set_u16(base + START_NODE + 4, triangle);
        let navmesh = e.mem.u32(base + START_NODE);
        let triangle = u32::from(e.mem.u16(base + START_NODE + 4));
        center_into(e, navmesh, triangle, ebp - 0x18, base + START_NODE + 8);
        let heuristic = start_to_goal(e, base, ebp - 0x58, ebp - 0x64, ebp - 0x4c);
        let navmesh = e.mem.u32(base + START_NODE);
        let triangle = u32::from(e.mem.u16(base + START_NODE + 4));
        let key = e
            .call(NODE_KEY_006A94F0, &args![this, navmesh, triangle])
            .u32();
        e.call(
            ADD_START_NODE_006A7C90,
            &args![this, key, base + START_NODE, heuristic],
        );
        let goal = goal.addr();
        if e.mem.u32(goal) != 0 || e.mem.u16(goal + 4) != 0xffff {
            e.mem.set_u8(base + HAS_GOAL_NODE, 1);
            let navmesh = e.mem.u32(goal);
            e.mem.set_u32(base + GOAL_NODE, navmesh);
            let triangle = e.mem.u16(goal + 4);
            e.mem.set_u16(base + GOAL_NODE + 4, triangle);
            let navmesh = e.mem.u32(base + GOAL_NODE);
            let triangle = u32::from(e.mem.u16(base + GOAL_NODE + 4));
            let center = e
                .call(GET_CENTER_00558220, &args![navmesh, ebp - 0x70, triangle])
                .u32();
            copy_words(e, ebp - 0x18, center, 3);
            for i in 0..3 {
                let word = x87_quiet_bits(e.mem.u32(ebp - 0x18 + 4 * i));
                e.mem.set_u32(base + GOAL_NODE + 8 + 4 * i, word);
            }
        } else {
            e.mem.set_u8(base + HAS_GOAL_NODE, 0);
        }
        e.mem
            .set_u32(base + MAX_DIST, x87_quiet_bits(max_dist.to_bits()));
        let node_array = ebp - 0x40;
        let link_array = ebp - 0x2c;
        fn_006abbb0(e, Ptr::new(node_array));
        fn_006abd40(e, Ptr::new(link_array));
        let found = e
            .call(SEARCH_006A7D60, &args![this, node_array, link_array, 0u32])
            .u8();
        if found != 0 {
            copy_words(e, out.addr(), point.addr(), 3);
        } else {
            let best = fn_006abb90(e, this);
            e.call(NODE_CENTER_006A6EB0, &args![best + 8, out]);
        }
        e.call(LINK_SCRAP_ARRAY_DTOR_006ABDE0, &args![link_array]);
        fn_006abc50(e, Ptr::new(node_array));
        out
    })
}

/// One edge `edge` of `006ab8e0`; adds a connection unless the edge is
/// skipped.
fn max_cost_connection(
    e: &mut Engine,
    this: u32,
    ebp: u32,
    node: u32,
    connections: u32,
    edge: u32,
) {
    let navmesh = e.mem.u32(node + 8);
    let triangle = u32::from(e.mem.u16(node + 0xc));
    let next_navmesh = ebp - 0x1c;
    let next_triangle = ebp - 0x14;
    let matched = e
        .call(
            GET_MATCHING_EDGE_0068F460,
            &args![
                navmesh,
                triangle,
                edge,
                next_navmesh,
                next_triangle,
                ebp - 0x18
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
    let center = ebp - 0x28;
    e.call(
        GET_CENTER_00558220,
        &args![
            e.mem.u32(next_navmesh),
            center,
            u32::from(e.mem.u16(next_triangle))
        ],
    );
    let here = e
        .call(NODE_CENTER_006A6EB0, &args![node + 8, ebp - 0x68])
        .u32();
    let step = e
        .call(POINT3_MINUS_00439EF0, &args![center, ebp - 0x74, here])
        .u32();
    let length = e.call(LENGTH_00589850, &args![step]).f64();
    let cost = x87_add(length, x87_m32(e.mem.u32(node)));
    e.mem.set_u32(ebp - 0x30, x87_store32(cost));
    let limit = x87_load32(e.mem.u32(this + MAX_DIST));
    if limit < x87_load32(e.mem.u32(ebp - 0x30)) {
        return;
    }
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
        e.mem.set_u32(ebp - 0x3c, door);
        let lock = e.call(DOOR_LOCK_004839C0, &args![door]).u32();
        e.mem.set_u32(ebp - 0x38, lock);
        if lock != 0 {
            let temp_lock = ebp - 0x5c;
            e.call(DOOR_LOCK_CTOR_00501D20, &args![temp_lock, lock]);
            let request = e.mem.u32(this + CURRENT_REQUEST);
            let actor = e.call(REQUEST_ACTOR_006C9900, &args![request]).u32();
            e.mem.set_u32(ebp - 0x40, actor);
            let can = e
                .call(
                    CAN_UNLOCK_00502450,
                    &args![temp_lock, actor, lock, ebp - 0x41],
                )
                .u8();
            if can == 0 {
                e.call(DOOR_LOCK_DTOR_00501E50, &args![temp_lock]);
                return;
            }
            let form = e.call(LOCK_FORM_007AF430, &args![lock]).u32();
            if e.call(IS_MINIMAL_USE_00518000, &args![form]).u8() != 0 {
                e.call(DOOR_LOCK_DTOR_00501E50, &args![temp_lock]);
                return;
            }
            e.call(DOOR_LOCK_DTOR_00501E50, &args![temp_lock]);
        }
    }
    let cost = x87_quiet_bits(e.mem.u32(ebp - 0x30));
    let next = ebp - 0x2c;
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
            e.call(
                SET_NODE_DATA_006A6EF0,
                &args![
                    this,
                    new_node,
                    e.mem.u32(next_navmesh),
                    u32::from(e.mem.u16(next_triangle))
                ],
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

// Translated from 006ab8e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006ab8e0 - its FS:[0] exception frame (the unwind state of the temporary DoorLock) is not translated
/// `NavMeshSearchMaxCost::GetNodeConnections` (Xbox PDB; `RET 8`: node,
/// connections): for each of the triangle's three edges with a matching
/// edge (`GetMatchingEdge_ov3`) whose extra info is not of type 1, the cost
/// (the node's cost plus the distance between the centers) must not exceed
/// `fMaxDist` (unordered passes); door triangles (flag 0x1000) are skipped
/// when the request's flag (`0073cb80`) is set, the actor cannot unlock the
/// lock (`DoorLock`) or the door is minimal-use. The node for the key
/// (`006a7e00`; a new one through `006a6ef0`) gets the link and is
/// appended. Returns true.
pub fn navmesh_search_max_cost_get_node_connections(
    e: &mut Engine,
    this: Ptr,
    node: Ptr,
    connections: Ptr,
) -> bool {
    let base = this.addr();
    e.with_stack(0x7c, |e, locals| {
        let ebp = locals.addr() + 0x7c;
        let mut edge = 0i32;
        while edge < 3 {
            max_cost_connection(e, base, ebp, node.addr(), connections.addr(), edge as u32);
            edge = edge.wrapping_add(1);
        }
        true
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006a_b480, fn_006ab480(Ptr)),
        entry!(0x006a_bc30, fn_006abc30(Ptr)),
        entry!(0x006a_bc50, fn_006abc50(Ptr)),
        entry!(
            0x006a_bcb0,
            bs_scrap_array_navmesh_search_node_allocate(Ptr, u32) -> u32
        ),
        entry!(
            0x006a_b450,
            navmesh_search_max_cost_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x006a_bb60,
            navmesh_search_max_cost_is_goal_node(Ptr, Ptr) -> bool
        ),
        entry!(0x006a_bb90, fn_006abb90(Ptr) -> u32),
        entry!(0x006a_bbb0, fn_006abbb0(Ptr) -> Ptr),
        entry!(0x006a_bd40, fn_006abd40(Ptr) -> Ptr),
        entry!(
            0x006a_bce0,
            bs_scrap_array_navmesh_search_node_reallocate(Ptr, u32, u32) -> u32
        ),
        entry!(
            0x006a_bea0,
            bs_simple_array_navmesh_search_node_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x006a_bed0,
            bs_simple_array_navmesh_search_link_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x006a_bf00,
            bs_scrap_array_navmesh_search_node_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x006a_bf30,
            bs_scrap_array_navmesh_search_link_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x006a_b4a0,
            navmesh_search_max_cost_build_node_path(Ptr, u32, Ptr, u32, u32, u32) -> bool
        ),
        entry!(
            0x006a_b670,
            fn_006ab670(Ptr, Ptr, u32, Ptr, Ptr, Ptr, f32) -> Ptr
        ),
        entry!(
            0x006a_b8e0,
            navmesh_search_max_cost_get_node_connections(Ptr, Ptr, Ptr) -> bool
        ),
    ]
}

#[cfg(test)]
mod tests {
    #[test]
    fn fn_006ab480_stores_the_vtable_then_forwards_this() {
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
        e.call(0x006a_b480, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x006a_6b00)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0106_b87c);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0106_b87c,
            "the vtable was stored before the call"
        );
    }

    /// Doubles the array release so it records the vtable it sees.
    fn record_release(e: &mut crate::prelude::Engine) {
        use crate::prelude::*;
        e.register(super::ARRAY_RELEASE_008454F0, |e, a| {
            let seen = e.mem.u32(a[0]);
            let count = e.mem.u32(a[0] + 4);
            e.mem.set_u32(a[0] + 8 + count * 4, seen);
            e.mem.set_u32(a[0] + 4, count + 1);
            Ret::default()
        });
    }

    #[test]
    fn simple_array_destructor_sets_its_vtable_and_releases() {
        use crate::prelude::*;
        let mut e = Engine::new();
        record_release(&mut e);
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(0x006a_bc30, &args![this]);
        assert_eq!(e.mem.u32(this), super::SIMPLE_ARRAY_SEARCH_NODE_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 1);
        assert_eq!(e.mem.u32(this + 8), super::SIMPLE_ARRAY_SEARCH_NODE_VTABLE);
        let log = e.call_log.take().unwrap();
        let releases: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == super::ARRAY_RELEASE_008454F0)
            .collect();
        assert_eq!(releases.len(), 1);
        assert_eq!(releases[0].1, vec![this, 1]);
    }

    #[test]
    fn scrap_array_destructor_releases_as_scrap_then_as_simple_array() {
        use crate::prelude::*;
        let mut e = Engine::new();
        record_release(&mut e);
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(0x006a_bc50, &args![this]);
        // Two releases: the first with the scrap array's vtable, the second
        // with the simple array's.
        assert_eq!(e.mem.u32(this + 4), 2);
        assert_eq!(e.mem.u32(this + 8), super::SCRAP_ARRAY_SEARCH_NODE_VTABLE);
        assert_eq!(e.mem.u32(this + 12), super::SIMPLE_ARRAY_SEARCH_NODE_VTABLE);
        assert_eq!(e.mem.u32(this), super::SIMPLE_ARRAY_SEARCH_NODE_VTABLE);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .filter(|(a, _)| *a == super::ARRAY_RELEASE_008454F0)
            .all(|(_, w)| *w == vec![this, 1]));
    }

    #[test]
    fn scrap_array_allocate_asks_its_scrap_heap_for_count_nodes() {
        use crate::prelude::*;
        let mut e = Engine::new();
        e.map(super::SCRAP_ALIGNMENT, 4);
        e.mem.set_u32(super::SCRAP_ALIGNMENT, 4);
        e.register(super::SCRAP_HEAP_ALLOCATE, |_, _| Ret {
            eax: 0x0abc_0000,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this + super::SCRAP_HEAP, 0x0555_0000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006a_bcb0, &args![this, 3u32]).u32(), 0x0abc_0000);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == super::SCRAP_HEAP_ALLOCATE)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x0555_0000, 0x3c, 4]);
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
        let length = |e: &mut Engine, a: &[u32]| {
            let (x, y, z) = (e.mem.f32(a[0]), e.mem.f32(a[0] + 4), e.mem.f32(a[0] + 8));
            f64::from((x * x + y * y + z * z).sqrt()).into_ret()
        };
        e.register(POINT3_LENGTH_00457990, length);
        e.register(LENGTH_00589850, length);
        e.register(NODE_CENTER_006A6EB0, |e, a| {
            for i in 0..3 {
                let word = e.mem.u32(a[0] + 8 + 4 * i);
                e.mem.set_u32(a[1] + 4 * i, word);
            }
            a[1].into_ret()
        });
        e.register(NODE_KEY_006A94F0, |_, a| (a[1] + a[2]).into_ret());
        e.register(FIRST_OF_00559450, |e, a| e.mem.u32(a[0]).into_ret());
    }

    fn check_scalar_deleting(address: u32, destroyed: u32) {
        let mut e = Engine::new();
        stub(&mut e, &[OPERATOR_DELETE, destroyed]);
        let this = e.mem.alloc(0x20b0);
        for flags in [0u32, 1] {
            let log = logged(&mut e, |e| {
                assert_eq!(e.call(address, &args![this, flags]).u32(), this);
            });
            assert!(!calls_to(&log, destroyed).is_empty());
            assert_eq!(calls_to(&log, OPERATOR_DELETE).len(), flags as usize);
        }
    }

    #[test]
    fn at_006ab450_deletes_the_max_cost_search() {
        check_scalar_deleting(0x006a_b450, 0x006a_6b00);
    }

    #[test]
    fn at_006abea0_deletes_a_node_array() {
        check_scalar_deleting(0x006a_bea0, ARRAY_RELEASE_008454F0);
    }

    #[test]
    fn at_006abed0_deletes_a_link_array() {
        check_scalar_deleting(0x006a_bed0, LINK_ARRAY_DTOR_006ABDC0);
    }

    #[test]
    fn at_006abf00_deletes_a_node_scrap_array() {
        check_scalar_deleting(0x006a_bf00, ARRAY_RELEASE_008454F0);
    }

    #[test]
    fn at_006abf30_deletes_a_link_scrap_array() {
        check_scalar_deleting(0x006a_bf30, LINK_SCRAP_ARRAY_DTOR_006ABDE0);
    }

    #[test]
    fn at_006abb60_needs_a_goal_node() {
        let mut e = Engine::new();
        e.register(NAVMESH_SEARCH_IS_GOAL_NODE, |_, _| 1u32.into_ret());
        let this = e.mem.alloc(0x20b0);
        assert!(!e.call(0x006a_bb60, &args![this, 0x10u32]).bool());
        e.mem.set_u8(this + HAS_GOAL_NODE, 1);
        let log = logged(&mut e, |e| {
            assert!(e.call(0x006a_bb60, &args![this, 0x10u32]).bool());
        });
        assert_eq!(
            calls_to(&log, NAVMESH_SEARCH_IS_GOAL_NODE),
            vec![vec![this, 0x10]]
        );
    }

    #[test]
    fn at_006abb90_returns_the_best_node() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20b0);
        e.mem.set_u32(this + BEST_NODE_SO_FAR, 0x1234);
        assert_eq!(e.call(0x006a_bb90, &args![this]).u32(), 0x1234);
    }

    #[test]
    fn at_006abbb0_and_006abd40_construct_the_scrap_arrays() {
        let mut e = Engine::new();
        stub(
            &mut e,
            &[
                NODE_ARRAY_CTOR_006ABF60,
                LINK_ARRAY_CTOR_006ABF90,
                NODE_ARRAY_INIT_009A3D00,
                LINK_ARRAY_INIT_006B3F40,
            ],
        );
        e.register(SCRAP_HOLDER_00401020, |_, _| 0x0100u32.into_ret());
        e.register(SCRAP_HEAP_00AA42E0, |_, a| (a[0] + 1).into_ret());
        for (address, vtable, init) in [
            (
                0x006a_bbb0u32,
                SCRAP_ARRAY_SEARCH_NODE_VTABLE,
                NODE_ARRAY_INIT_009A3D00,
            ),
            (
                0x006a_bd40,
                SCRAP_ARRAY_SEARCH_LINK_VTABLE,
                LINK_ARRAY_INIT_006B3F40,
            ),
        ] {
            let this = e.mem.alloc(0x14);
            let log = logged(&mut e, |e| {
                assert_eq!(e.call(address, &args![this]).u32(), this);
            });
            assert_eq!(e.mem.u32(this), vtable);
            assert_eq!(e.mem.u32(this + 0x10), 0x0101);
            assert_eq!(calls_to(&log, init), vec![vec![this, 0, 0]]);
        }
    }

    #[test]
    fn at_006abce0_reallocates_from_the_scrap_heap() {
        let mut e = Engine::new();
        e.map(SCRAP_ALIGNMENT, 4);
        e.mem.set_u32(SCRAP_ALIGNMENT, 8);
        e.register(SCRAP_HEAP_ALLOCATE, |_, _| 0x0abc_0000u32.into_ret());
        e.register(ARRAY_SIZE_0044DDC0, |e, a| e.mem.u32(a[0] + 8).into_ret());
        stub(&mut e, &[COPY_00401460, SCRAP_HEAP_DEALLOCATE]);
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this + 8, 2);
        e.mem.set_u32(this + SCRAP_HEAP, 0x0555_0000);
        let log = logged(&mut e, |e| {
            assert_eq!(
                e.call(0x006a_bce0, &args![this, 0x7000u32, 5u32]).u32(),
                0x0abc_0000
            );
        });
        assert_eq!(
            calls_to(&log, SCRAP_HEAP_ALLOCATE),
            vec![vec![0x0555_0000, 0x64, 8]]
        );
        assert_eq!(
            calls_to(&log, COPY_00401460),
            vec![vec![0x0abc_0000, 0x7000, 0x28]]
        );
        assert_eq!(
            calls_to(&log, SCRAP_HEAP_DEALLOCATE),
            vec![vec![0x0555_0000, 0x7000]]
        );
    }

    #[test]
    fn at_006ab4a0_sets_the_goal_and_searches() {
        let mut e = Engine::new();
        geometry(&mut e);
        stub(
            &mut e,
            &[
                ARRAY_RELEASE_008454F0,
                SEARCH_RESET_006A7C40,
                LOCATION_FLAG_006A9520,
                HOLDER_CTOR_0042FB00,
                HOLDER_DTOR_0042FA40,
                ADD_START_NODE_006A7C90,
            ],
        );
        e.register(RESOLVE_NAVMESH_INFO_006DD6F0, |_, _| 1u32.into_ret());
        e.register(GET_NAVMESH_AND_TRIANGLE_006DD640, |e, a| {
            e.mem.set_u32(a[1], 0x5000);
            e.mem.set_u16(a[2], 7);
            1u32.into_ret()
        });
        e.register(SEARCH_006A7D60, |_, _| 1u32.into_ret());
        let this = e.mem.alloc(0x20b0);
        let start = e.mem.alloc(0x14);
        e.mem.set_u32(start, 0x4000);
        e.mem.set_u16(start + 4, 2);
        e.mem.set_f32(start + 8, 3.0);
        let log = logged(&mut e, |e| {
            assert!(e
                .call(
                    0x006a_b4a0,
                    &args![this, 0x44u32, start, 0x55u32, 0x66u32, 0x77u32]
                )
                .bool());
        });
        assert_eq!(e.mem.u32(this + CURRENT_REQUEST), 0x44);
        assert_eq!(e.mem.u32(this + GOAL_NODE), 0x5000);
        assert_eq!(e.mem.f32(this + GOAL_NODE + 8), 7.0);
        assert_eq!(e.mem.u8(this + HAS_GOAL_NODE), 1);
        assert_eq!(
            calls_to(&log, ARRAY_RELEASE_008454F0),
            vec![vec![0x66, 0], vec![0x77, 0]]
        );
        let added = calls_to(&log, ADD_START_NODE_006A7C90);
        assert_eq!(added, vec![vec![this, 0x4002, start, 4.0f32.to_bits()]]);
        assert_eq!(
            calls_to(&log, SEARCH_006A7D60),
            vec![vec![this, 0x66, 0x77, 0]]
        );
        // A flagged goal location is not looked up.
        e.register(LOCATION_FLAG_006A9520, |_, _| 1u32.into_ret());
        let log = logged(&mut e, |e| {
            e.call(
                0x006a_b4a0,
                &args![this, 0x44u32, start, 0x55u32, 0x66u32, 0x77u32],
            );
        });
        assert!(calls_to(&log, GET_NAVMESH_AND_TRIANGLE_006DD640).is_empty());
    }

    #[test]
    fn at_006ab670_finds_the_point_or_the_best_node() {
        let mut e = Engine::new();
        geometry(&mut e);
        stub(
            &mut e,
            &[
                ADD_START_NODE_006A7C90,
                NODE_ARRAY_CTOR_006ABF60,
                LINK_ARRAY_CTOR_006ABF90,
                NODE_ARRAY_INIT_009A3D00,
                LINK_ARRAY_INIT_006B3F40,
                SCRAP_HOLDER_00401020,
                SCRAP_HEAP_00AA42E0,
                LINK_SCRAP_ARRAY_DTOR_006ABDE0,
                ARRAY_RELEASE_008454F0,
            ],
        );
        e.register(SEARCH_006A7D60, |_, _| 1u32.into_ret());
        let this = e.mem.alloc(0x20b0);
        let start = e.mem.alloc(0x14);
        e.mem.set_u32(start, 0x4000);
        e.mem.set_u16(start + 4, 2);
        let goal = e.mem.alloc(0x14);
        e.mem.set_u16(goal + 4, 0xffff);
        let point = e.mem.alloc(0xc);
        e.mem.set_f32(point, 11.0);
        let out = e.mem.alloc(0xc);
        let run = |e: &mut Engine| {
            e.call(
                0x006a_b670,
                &args![this, out, 0u32, start, point, goal, 6.0f32.to_bits()],
            )
            .u32()
        };
        assert_eq!(run(&mut e), out);
        assert_eq!(e.mem.f32(out), 11.0);
        assert_eq!(e.mem.u8(this + HAS_GOAL_NODE), 0);
        assert_eq!(e.mem.f32(this + MAX_DIST), 6.0);
        assert_eq!(e.mem.f32(this + START_NODE + 8), 2.0);
        // Not found: the best node's center; a real goal node is taken.
        e.register(SEARCH_006A7D60, |_, _| Ret::default());
        let best = e.mem.alloc(0x3c);
        e.mem.set_f32(best + 0x10, 13.0);
        e.mem.set_u32(this + BEST_NODE_SO_FAR, best);
        e.mem.set_u16(goal + 4, 5);
        assert_eq!(run(&mut e), out);
        assert_eq!(e.mem.f32(out), 13.0);
        assert_eq!(e.mem.u8(this + HAS_GOAL_NODE), 1);
        assert_eq!(e.mem.f32(this + GOAL_NODE + 8), 5.0);
    }

    #[test]
    fn at_006ab8e0_connects_within_the_maximum_distance() {
        let mut e = Engine::new();
        geometry(&mut e);
        e.register(GET_MATCHING_EDGE_0068F460, |e, a| {
            if a[2] != 0 {
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
                ARRAY_PUSH_007CB2E0,
                SET_NODE_DATA_006A6EF0,
            ],
        );
        e.register(NODE_FOR_KEY_006A7E00, |e, a| {
            let node = e.mem.alloc(0x3c);
            e.mem.set_u32(a[3], node);
            Ret::default()
        });
        let this = e.mem.alloc(0x20b0);
        e.mem.set_f32(this + MAX_DIST, 3.0);
        let node = e.mem.alloc(0x3c);
        e.mem.set_u32(node + 8, 0x4000);
        e.mem.set_f32(node + 0x10, 2.0);
        e.mem.set_f32(node, 1.0);
        let connections = e.mem.alloc(0x10);
        // Cost 1 + 4 above the maximum 3: nothing.
        let log = logged(&mut e, |e| {
            assert!(e.call(0x006a_b8e0, &args![this, node, connections]).bool());
        });
        assert!(calls_to(&log, NODE_FOR_KEY_006A7E00).is_empty());
        e.mem.set_f32(this + MAX_DIST, 5.0);
        let log = logged(&mut e, |e| {
            e.call(0x006a_b8e0, &args![this, node, connections]);
        });
        let found = calls_to(&log, NODE_FOR_KEY_006A7E00);
        assert_eq!(found.len(), 1);
        assert_eq!((found[0][1], found[0][2]), (0x5006, 5.0f32.to_bits()));
        assert_eq!(calls_to(&log, SET_NODE_DATA_006A6EF0).len(), 1);
        assert_eq!(calls_to(&log, ARRAY_PUSH_007CB2E0)[0][0], connections);
    }
}
