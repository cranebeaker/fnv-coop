//! `fallout shared/pathfinding/navmeshinfosearch.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAV?$AStarNode@PBVNavMeshInfo@@PAVTESObjectREFR@@@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0106C264: u32 = 0x0106_c264;
/// `008454f0` (unnamed in the engine map): `thiscall`, `this` plus one stack word (`RET 4`). When the
/// buffer pointer (+4) is set it runs the instance's element destroy step; with the word non-zero it
/// then calls the array's virtual slot +8 on the buffer and clears the buffer pointer and the reserved
/// size (+0xc); finally it clears the size (+8).
const ARRAY_RELEASE_008454F0: u32 = 0x0084_54f0;

// Translated from 006b9610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAV?$AStarNode@PBVNavMeshInfo@@PAVTESObjectREFR@@@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_006b9610(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106C264);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

/// `NavMeshInfoSearch` / `AStarSearch<const NavMeshInfo *, TESObjectREFR *>`
/// fields (Xbox PDB names; the PC offsets from `AStarNodeMap` on are 4
/// below the PDB's, the node heap being 4 bytes smaller): `NodeHeap` (+4),
/// `AStarNodeMap` (+0x14), `SearchQueue` (+0x1ff4), `BestNodeSoFar`
/// (+0x2048), `pLastSearchedNode` (+0x204c), `pStartAStarNode` (+0x2050),
/// `bIsRunningInTasklet` (+0x2054), `StartLocation` (+0x2058), `pStartMesh`
/// (+0x2064), `GoalLocation` (+0x2068), `pGoalMesh` (+0x2074),
/// `pActorLockData` (+0x2078), `DoorLockDisposition` (+0x207c),
/// `bSameSpace` (+0x2080), `TempDoorLock` (+0x2084).
const NODE_HEAP: u32 = 0x4;
const NODE_MAP: u32 = 0x14;
const SEARCH_QUEUE: u32 = 0x1ff4;
const BEST_NODE: u32 = 0x2048;
const LAST_SEARCHED_NODE: u32 = 0x204c;
const START_NODE: u32 = 0x2050;
const RUNNING_IN_TASKLET: u32 = 0x2054;
const START_LOCATION: u32 = 0x2058;
const START_MESH: u32 = 0x2064;
const GOAL_LOCATION: u32 = 0x2068;
const GOAL_MESH: u32 = 0x2074;
const ACTOR_LOCK_DATA: u32 = 0x2078;
const DOOR_LOCK_DISPOSITION: u32 = 0x207c;
const SAME_SPACE: u32 = 0x2080;
const TEMP_DOOR_LOCK: u32 = 0x2084;
/// `AStarNode` fields (Xbox PDB, same on PC): `fCost` (+0), `fHeuristic`
/// (+4), `NodeData` (+8), `ParentLink` (+0xc), `pParentAStarNode` (+0x10),
/// `ppPrevNode` (+0x14), `pNextNode` (+0x18), `iKey` (+0x1c),
/// `pNextNodeInBucket` (+0x20).
const NODE_HEURISTIC: u32 = 0x4;
const NODE_DATA: u32 = 0x8;
const NODE_PARENT_LINK: u32 = 0xc;
const NODE_PARENT: u32 = 0x10;
const NODE_PREVIOUS: u32 = 0x14;
const NODE_NEXT: u32 = 0x18;
const NODE_KEY: u32 = 0x1c;
const NODE_NEXT_IN_BUCKET: u32 = 0x20;
/// Vtables (RTTI checked): `NavMeshInfoSearch`, its `AStarSearch` base, and
/// the `BSScrapArray` / `BSSimpleArray` of `AStarNode *`.
const NAV_MESH_INFO_SEARCH_VTABLE: u32 = 0x0106_c22c;
const A_STAR_SEARCH_VTABLE: u32 = 0x0106_c240;
const SCRAP_ARRAY_VTABLE: u32 = 0x0106_c250;
/// The search's vtable slots: `GetNodeConnections` (`0x4`, node, array) and
/// `IsGoalNode` (`0x8`, node).
const GET_NODE_CONNECTIONS_SLOT: u32 = 0x4;
const IS_GOAL_NODE_SLOT: u32 = 0x8;
/// Constants the exe reads: `01012054` (the queue's starting fitness, -1.0
/// in the exe), `01016970` (the start heuristic, `FLT_MAX`), `0101ffa0`
/// (the heuristic scale, a double 0.1), `01017a40` (a double 100.0),
/// `0101e2bc` (the cost factor's floor, 0.1), `01021928` (a double 3.0) and
/// `0106c238` (the door penalty, 409600.0).
const QUEUE_START_FITNESS: u32 = 0x0101_2054;
const START_HEURISTIC: u32 = 0x0101_6970;
const HEURISTIC_SCALE: u32 = 0x0101_ffa0;
const FACTOR_SCALE: u32 = 0x0101_7a40;
const FACTOR_FLOOR: u32 = 0x0101_e2bc;
const EDGE_SCALE: u32 = 0x0102_1928;
const DOOR_PENALTY: u32 = 0x0106_c238;
/// Callees by address (`thiscall` unless said otherwise; stack words in
/// brackets).
const FILL_MEMORY: u32 = 0x0040_3d30;
const SCRAP_HEAP_INIT: u32 = 0x00aa_5410;
const SCRAP_ALLOCATE: u32 = 0x006a_1f90;
const SIMPLE_ARRAY_INIT: u32 = 0x006b_3eb0;
const NODE_MAP_CONSTRUCT: u32 = 0x006b_3de0;
const NODE_MAP_DESTRUCT: u32 = 0x0048_3710;
const NODE_HEAP_DESTRUCT: u32 = 0x0098_2610;
const DOOR_LOCK_DESTRUCT: u32 = 0x0050_1e50;
const CURRENT_THREAD_HEAP_HOLDER: u32 = 0x0040_1020;
const GET_THREAD_SCRAP_HEAP: u32 = 0x00aa_42e0;
const QUEUE_BUCKET_INDEX: u32 = 0x006f_4790;
const MAP_BUCKET_INDEX: u32 = 0x0099_6840;
const QUEUE_POP_BEST: u32 = 0x0099_6ce0;
const TASKLET_MANAGER_INSTANCE: u32 = 0x00b0_0a00;
const CURRENT_THREAD_ID: u32 = 0x0040_fc90;
const TASKLET_SLOT: u32 = 0xc;
const BUILD_PATH: u32 = 0x0099_6ae0;
const TASKLET_YIELD: u32 = 0x0040_fca0;
const ARRAY_SIZE: u32 = 0x0044_ddc0;
const ARRAY_ELEMENT_ADDRESS: u32 = 0x0087_7a30;
const ARRAY_ADD: u32 = 0x007c_b2e0;
const POINT_SUBTRACT: u32 = 0x0043_9ef0;
const POINT_LENGTH: u32 = 0x0045_7990;
const MESH_SPACE: u32 = 0x0069_0800;
const INFO_COST_VALUE: u32 = 0x007d_f1f0;
const CLAMP: u32 = 0x0053_30e0;
const REFERENCE_FLAG_800: u32 = 0x0044_0da0;
const DOOR_LOCK_SET_FROM_DOOR: u32 = 0x0050_1e80;
const DOOR_LOCK_CAN_UNLOCK: u32 = 0x0050_2450;
const REFERENCE_BASE_FORM: u32 = 0x007a_f430;
const DOOR_IS_MINIMAL_USE: u32 = 0x0051_8000;
const DOOR_TEST: u32 = 0x0057_b4b0;
const DOOR_TELEPORT: u32 = 0x0056_8e50;
const FIRST_WORD: u32 = 0x0055_9450;
const REFERENCE_EXTRA_LIST: u32 = 0x005d_43c0;
const GET_NAV_MESH_PORTAL: u32 = 0x0042_e2a0;
const TES_SINGLETON: u32 = 0x011d_ea10;
const GET_NAV_MESH_INFO_MAP: u32 = 0x0045_af00;
const INFO_MAP_LOOKUP: u32 = 0x0069_3d70;
const INFO_TEST: u32 = 0x0068_f320;
const PLUS_4: u32 = 0x0071_7e50;

/// The x87 condition bits (`AH` after `FNSTSW AX`: C0 0x01, C2 0x04, C3
/// 0x40) of comparing `st0` with `source`.
fn fcom_status(st0: f64, source: f64) -> u8 {
    if st0.is_nan() || source.is_nan() {
        0x45
    } else if st0 < source {
        0x01
    } else if st0 == source {
        0x40
    } else {
        0
    }
}

/// `TEST AH,mask` then `JP`: the parity flag of the masked bits.
fn parity(status: u8, mask: u8) -> bool {
    (status & mask).count_ones() % 2 == 0
}

/// A double the exe reads from its data.
fn double_at(e: &Engine, address: u32) -> f64 {
    f64::from_bits(e.mem.u64(address))
}

// Translated from 006b9130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the float at `this + 4` in `ST0`
/// (`AStarNode::fHeuristic` for a node; folded, other callers pass other
/// objects).
pub fn fn_006b9130(e: &mut Engine, this: Ptr) -> f64 {
    x87_load32(e.mem.u32(this.addr().wrapping_add(4)))
}

// Translated from 006b9710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a node's fitness, `fCost + fHeuristic` (x87,
/// stored as a float and loaded back).
pub fn fn_006b9710(e: &mut Engine, this: Ptr) -> f64 {
    let cost = x87_load32(e.mem.u32(this.addr()));
    let heuristic = x87_m32(e.mem.u32(this.addr().wrapping_add(NODE_HEURISTIC)));
    x87_load32(x87_store32(x87_add(cost, heuristic)))
}

// Translated from 006b96c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `AStarNode` constructor (unnamed in the engine map): cost and
/// heuristic 0.0, parent, links, key and bucket link cleared. Answers
/// `this`.
pub fn fn_006b96c0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    let zero = x87_store32(0.0);
    e.mem.set_u32(base, zero);
    e.mem.set_u32(base.wrapping_add(NODE_HEURISTIC), zero);
    for offset in [
        NODE_PARENT,
        NODE_PREVIOUS,
        NODE_NEXT,
        NODE_KEY,
        NODE_NEXT_IN_BUCKET,
    ] {
        e.mem.set_u32(base.wrapping_add(offset), 0);
    }
    this
}

// Translated from 006b9420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a new `AStarNode` from the node heap (0x24
/// bytes, `006a1f90`, `cdecl` (size, heap)), constructed (`006b96c0`)
/// unless the allocation failed.
pub fn fn_006b9420(e: &mut Engine, this: Ptr) -> u32 {
    let block = e.call(SCRAP_ALLOCATE, &args![0x24u32, this]).u32();
    if block != 0 {
        fn_006b96c0(e, Ptr::new(block)).addr()
    } else {
        0
    }
}

// Translated from 006b9400 (decompiled, FalloutNV.exe 1.4.0.525)
/// The node heap's constructor (unnamed in the engine map): its scrap heap
/// with 0x4800 bytes (`00aa5410`). Answers `this`.
pub fn fn_006b9400(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(SCRAP_HEAP_INIT, &args![this, 0x4800u32]);
    this
}

// Translated from 006b9470 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `AStarQueue` constructor (unnamed in the engine map):
/// `fMaxFitness` from `01012054` (a `FLD`/`FSTP` copy) and the 0x50 bytes of
/// buckets cleared (`00403d30`). Answers `this`.
pub fn fn_006b9470(e: &mut Engine, this: Ptr) -> Ptr {
    let start = x87_quiet_bits(e.global::<u32>(QUEUE_START_FITNESS));
    e.mem.set_u32(this.addr(), start);
    e.call(
        FILL_MEMORY,
        &args![this.addr().wrapping_add(4), 0u32, 0x50u32],
    );
    this
}

// Translated from 006b9540 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `AStarQueue` removal (unnamed in the engine map, `RET 4`): unlinks
/// the node from its bucket chain (the previous link takes `pNextNode`, the
/// next node's `ppPrevNode` takes the node's) and clears both links.
pub fn fn_006b9540(e: &mut Engine, _this: Ptr, node: Ptr) {
    let base = node.addr();
    let previous = e.mem.u32(base.wrapping_add(NODE_PREVIOUS));
    if previous != 0 {
        let next = e.mem.u32(base.wrapping_add(NODE_NEXT));
        e.mem.set_u32(previous, next);
    }
    let next = e.mem.u32(base.wrapping_add(NODE_NEXT));
    if next != 0 {
        let previous = e.mem.u32(base.wrapping_add(NODE_PREVIOUS));
        e.mem.set_u32(next.wrapping_add(NODE_PREVIOUS), previous);
    }
    e.mem.set_u32(base.wrapping_add(NODE_PREVIOUS), 0);
    e.mem.set_u32(base.wrapping_add(NODE_NEXT), 0);
}

// Translated from 006b94a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `AStarQueue` insertion (unnamed in the engine map, `RET 4`): in the
/// node's bucket (`006f4790`), after every node whose fitness the new one's
/// is above (`006b9710`, the bucket's stored as a double first), linked in
/// both directions.
pub fn fn_006b94a0(e: &mut Engine, this: Ptr, node: Ptr) {
    let index = e.call(QUEUE_BUCKET_INDEX, &args![this, node]).u32();
    let mut slot = this
        .addr()
        .wrapping_add(index.wrapping_mul(4))
        .wrapping_add(4);
    while e.mem.u32(slot) != 0 {
        let there = fn_006b9710(e, Ptr::new(e.mem.u32(slot)));
        let fitness = fn_006b9710(e, node);
        if fcom_status(fitness, there) & 0x41 != 0 {
            break;
        }
        slot = e.mem.u32(slot).wrapping_add(NODE_NEXT);
    }
    let following = e.mem.u32(slot);
    if following != 0 {
        e.mem.set_u32(
            following.wrapping_add(NODE_PREVIOUS),
            node.addr().wrapping_add(NODE_NEXT),
        );
    }
    let following = e.mem.u32(slot);
    e.mem
        .set_u32(node.addr().wrapping_add(NODE_NEXT), following);
    e.mem.set_u32(node.addr().wrapping_add(NODE_PREVIOUS), slot);
    e.mem.set_u32(slot, node.addr());
}

// Translated from 006b8c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshInfoSearch::IsGoalNode` (Xbox PDB, `RET 4`): whether the node's
/// `NodeData` is `pGoalMesh` (0 or 1 in `EAX`).
pub fn nav_mesh_info_search_is_goal_node(e: &mut Engine, this: Ptr, node: Ptr) -> u32 {
    let data = e.mem.u32(node.addr().wrapping_add(NODE_DATA));
    u32::from(data == e.mem.u32(this.addr().wrapping_add(GOAL_MESH)))
}

// Translated from 006b8fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 0xc`: key, cost, the address that gets
/// the node): finds the key's node in the node map's bucket (`00996840`)
/// chain. A found node with a higher cost than the new one is taken out of
/// the queue (`006b9540`), given the cost and answered with 1; one not
/// cheaper answers 2. Otherwise a new node (`006b9420`) takes the key and
/// cost, goes first in the bucket, and 0 is answered.
pub fn fn_006b8fe0(e: &mut Engine, this: Ptr, key: u32, cost: f32, out: Ptr) -> u32 {
    let base = this.addr();
    let index = e
        .call(MAP_BUCKET_INDEX, &args![base.wrapping_add(NODE_MAP), key])
        .u32();
    let bucket = base
        .wrapping_add(index.wrapping_mul(4))
        .wrapping_add(NODE_MAP);
    let mut node = e.mem.u32(bucket);
    while node != 0 {
        if e.mem.u32(node.wrapping_add(NODE_KEY)) == key {
            let old = x87_load32(e.mem.u32(node));
            if parity(fcom_status(x87_load32(cost.to_bits()), old), 0x05) {
                return 2;
            }
            e.mem.set_u32(out.addr(), node);
            let found = e.mem.u32(out.addr());
            fn_006b9540(
                e,
                Ptr::new(base.wrapping_add(SEARCH_QUEUE)),
                Ptr::new(found),
            );
            let found = e.mem.u32(out.addr());
            e.mem.set_u32(found, x87_quiet_bits(cost.to_bits()));
            return 1;
        }
        node = e.mem.u32(node.wrapping_add(NODE_NEXT_IN_BUCKET));
    }
    let made = fn_006b9420(e, Ptr::new(base.wrapping_add(NODE_HEAP)));
    e.mem.set_u32(out.addr(), made);
    let made = e.mem.u32(out.addr());
    e.mem.set_u32(made.wrapping_add(NODE_KEY), key);
    e.mem.set_u32(made, x87_quiet_bits(cost.to_bits()));
    let first = e.mem.u32(bucket);
    e.mem.set_u32(made.wrapping_add(NODE_NEXT_IN_BUCKET), first);
    e.mem.set_u32(bucket, made);
    0
}

// Translated from 006b8ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 0xc`: key, the address of the node
/// data, heuristic): the start node. `006b8fe0` with cost 0.0; a new node
/// gets the heuristic and the data, goes into the queue (`006b94a0`) and
/// becomes `pStartAStarNode`; a new or updated node is answered, else null.
pub fn fn_006b8ec0(e: &mut Engine, this: Ptr, key: u32, data: Ptr, heuristic: f32) -> u32 {
    let base = this.addr();
    e.with_stack(4, |e, local| {
        let result = fn_006b8fe0(e, this, key, 0.0, local);
        let node = e.mem.u32(local.addr());
        match result {
            0 => {
                e.mem.set_u32(
                    node.wrapping_add(NODE_HEURISTIC),
                    x87_quiet_bits(heuristic.to_bits()),
                );
                let value = e.mem.u32(data.addr());
                e.mem.set_u32(node.wrapping_add(NODE_DATA), value);
                fn_006b94a0(e, Ptr::new(base.wrapping_add(SEARCH_QUEUE)), Ptr::new(node));
                e.mem.set_u32(base.wrapping_add(START_NODE), node);
                node
            }
            1 => node,
            _ => 0,
        }
    })
}

// Translated from 006b9760 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSSimpleArray<AStarNode *>` constructor (unnamed in the engine map;
/// vtable by RTTI): the vtable, then `006b3eb0` with 0, 0. Answers `this`.
pub fn fn_006b9760(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106C264);
    e.call(SIMPLE_ARRAY_INIT, &args![this, 0u32, 0u32]);
    this
}

// Translated from 006b9590 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSScrapArray<AStarNode *>` constructor (unnamed in the engine map;
/// vtable by RTTI): the `BSSimpleArray` base (`006b9760`), its own vtable,
/// the thread's scrap heap (`00aa42e0` on `00401020`'s answer) at +0x10,
/// then `006b3eb0` with 0, 0. Answers `this`.
// Translation incomplete: 006b9590 - the SEH frame (FS:[0], unwind of the base) is not translated.
pub fn fn_006b9590(e: &mut Engine, this: Ptr) -> Ptr {
    fn_006b9760(e, this);
    e.mem.set_u32(this.addr(), SCRAP_ARRAY_VTABLE);
    let holder = e.call(CURRENT_THREAD_HEAP_HOLDER, &args![]).u32();
    let heap = e.call(GET_THREAD_SCRAP_HEAP, &args![holder]).u32();
    e.mem.set_u32(this.addr().wrapping_add(0x10), heap);
    e.call(SIMPLE_ARRAY_INIT, &args![this, 0u32, 0u32]);
    this
}

// Translated from 006b9660 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSScrapArray<AStarNode *>` destructor body (unnamed in the engine
/// map): its vtable, the buffer released (`008454f0` with 1), then the
/// `BSSimpleArray` base (`006b9610`).
// Translation incomplete: 006b9660 - the SEH frame (FS:[0], unwind of the base) is not translated.
pub fn fn_006b9660(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), SCRAP_ARRAY_VTABLE);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
    fn_006b9610(e, this);
}

// Translated from 006b8d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `AStarSearch` constructor (unnamed in the engine map; vtable by
/// RTTI): the vtable, the node heap (`006b9400`), the node map
/// (`006b3de0`), the queue (`006b9470`), and no best, last or start node,
/// not in a tasklet. Answers `this`.
// Translation incomplete: 006b8d90 - the SEH frame (FS:[0], unwind of the members) is not translated.
pub fn fn_006b8d90(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, A_STAR_SEARCH_VTABLE);
    fn_006b9400(e, Ptr::new(base.wrapping_add(NODE_HEAP)));
    e.call(NODE_MAP_CONSTRUCT, &args![base.wrapping_add(NODE_MAP)]);
    fn_006b9470(e, Ptr::new(base.wrapping_add(SEARCH_QUEUE)));
    e.mem.set_u32(base.wrapping_add(BEST_NODE), 0);
    e.mem.set_u32(base.wrapping_add(LAST_SEARCHED_NODE), 0);
    e.mem.set_u32(base.wrapping_add(START_NODE), 0);
    e.mem.set_u8(base.wrapping_add(RUNNING_IN_TASKLET), 0);
    this
}

// Translated from 006b8e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `AStarSearch` destructor body (unnamed in the engine map): the
/// vtable, then the queue and the node map (`00483710` each) and the node
/// heap (`00982610`).
// Translation incomplete: 006b8e40 - the SEH frame (FS:[0], unwind of the members) is not translated.
pub fn fn_006b8e40(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, A_STAR_SEARCH_VTABLE);
    e.call(NODE_MAP_DESTRUCT, &args![base.wrapping_add(SEARCH_QUEUE)]);
    e.call(NODE_MAP_DESTRUCT, &args![base.wrapping_add(NODE_MAP)]);
    e.call(NODE_HEAP_DESTRUCT, &args![base.wrapping_add(NODE_HEAP)]);
}

// Translated from 006b8420 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NavMeshInfoSearch` destructor body (unnamed in the engine map;
/// vtable by RTTI): the vtable, `TempDoorLock`'s destructor
/// (`DoorLock::~DoorLock`), then the `AStarSearch` base (`006b8e40`).
// Translation incomplete: 006b8420 - the SEH frame (FS:[0], unwind of the members) is not translated.
pub fn fn_006b8420(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, NAV_MESH_INFO_SEARCH_VTABLE);
    e.call(
        DOOR_LOCK_DESTRUCT,
        &args![base.wrapping_add(TEMP_DOOR_LOCK)],
    );
    fn_006b8e40(e, this);
}

// Translated from 006b9180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AStarSearch::Search` (unnamed in the engine map): runs the search from
/// the queue. With a stack `BSScrapArray` of connections (`006b9590`,
/// destroyed with `006b9660` on every way out), it pops the best node
/// (`00996ce0`) into `pLastSearchedNode`; none answers null. The first
/// popped node is the best so far. Each popped node that is the goal
/// (vtable `+0x8`) becomes the best and is answered. Otherwise its
/// connections (vtable `+0x4` into the emptied array, `008454f0` with 0) are
/// each taken out of the queue; a connection that is the node's parent
/// ends the search with null; others get the node as parent, go back into
/// the queue and replace the best when their heuristic is below its. In a
/// tasklet every 50th node yields (`0040fca0(0)`). When the queue runs out,
/// `pLastSearchedNode` goes back to the last node popped and null is
/// answered.
// Translation incomplete: 006b9180 - the SEH frame (FS:[0], unwind of the stack BSScrapArray) is not translated.
pub fn fn_006b9180(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    let queue = Ptr::new(base.wrapping_add(SEARCH_QUEUE));
    e.with_stack(0x14, |e, connections| {
        fn_006b9590(e, connections);
        let finish = |e: &mut Engine, result: u32| {
            fn_006b9660(e, connections);
            result
        };
        let mut previous = e.mem.u32(base.wrapping_add(LAST_SEARCHED_NODE));
        let popped = e.call(QUEUE_POP_BEST, &args![queue]).u32();
        e.mem.set_u32(base.wrapping_add(LAST_SEARCHED_NODE), popped);
        if e.mem.u32(base.wrapping_add(LAST_SEARCHED_NODE)) == 0 {
            return finish(e, 0);
        }
        let current = e.mem.u32(base.wrapping_add(LAST_SEARCHED_NODE));
        e.mem.set_u32(base.wrapping_add(BEST_NODE), current);
        let mut yield_count: u32 = 0;
        while e.mem.u32(base.wrapping_add(LAST_SEARCHED_NODE)) != 0 {
            let current = e.mem.u32(base.wrapping_add(LAST_SEARCHED_NODE));
            if e.vcall(base, IS_GOAL_NODE_SLOT, &args![current]).bool() {
                let current = e.mem.u32(base.wrapping_add(LAST_SEARCHED_NODE));
                e.mem.set_u32(base.wrapping_add(BEST_NODE), current);
                let result = e.mem.u32(base.wrapping_add(LAST_SEARCHED_NODE));
                return finish(e, result);
            }
            e.call(ARRAY_RELEASE_008454F0, &args![connections, 0u32]);
            let current = e.mem.u32(base.wrapping_add(LAST_SEARCHED_NODE));
            e.vcall(
                base,
                GET_NODE_CONNECTIONS_SLOT,
                &args![current, connections],
            );
            let mut index: u32 = 0;
            while index < e.call(ARRAY_SIZE, &args![connections]).u32() {
                let at = e
                    .call(ARRAY_ELEMENT_ADDRESS, &args![connections, index])
                    .u32();
                let node = e.mem.u32(at);
                fn_006b9540(e, queue, Ptr::new(node));
                let current = e.mem.u32(base.wrapping_add(LAST_SEARCHED_NODE));
                if e.mem.u32(current.wrapping_add(NODE_PARENT)) == node {
                    return finish(e, 0);
                }
                let current = e.mem.u32(base.wrapping_add(LAST_SEARCHED_NODE));
                e.mem.set_u32(node.wrapping_add(NODE_PARENT), current);
                fn_006b94a0(e, queue, Ptr::new(node));
                let heuristic = fn_006b9130(e, Ptr::new(node));
                let best = Ptr::new(e.mem.u32(base.wrapping_add(BEST_NODE)));
                let best_heuristic = fn_006b9130(e, best);
                if fcom_status(best_heuristic, heuristic) & 0x41 == 0 {
                    e.mem.set_u32(base.wrapping_add(BEST_NODE), node);
                }
                index = index.wrapping_add(1);
            }
            if e.mem.u8(base.wrapping_add(RUNNING_IN_TASKLET)) != 0 {
                yield_count = yield_count.wrapping_add(1);
                if yield_count == 0x32 {
                    yield_count = 0;
                    e.call(TASKLET_YIELD, &args![0u32]);
                }
            }
            previous = e.mem.u32(base.wrapping_add(LAST_SEARCHED_NODE));
            let popped = e.call(QUEUE_POP_BEST, &args![queue]).u32();
            e.mem.set_u32(base.wrapping_add(LAST_SEARCHED_NODE), popped);
        }
        e.mem
            .set_u32(base.wrapping_add(LAST_SEARCHED_NODE), previous);
        finish(e, 0)
    })
}

// Translated from 006b8f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 0xc`: two words passed to `00996ae0`
/// and a byte): runs the search. `bIsRunningInTasklet` is the tasklet
/// manager's answer (`00b00a00`, vtable `+0xc` with `0040fc90`'s word). A
/// goal found (`006b9180`) answers true; otherwise, when the byte is set,
/// the best node so far is used. A node used gets `00996ae0` (node and the
/// two words).
pub fn fn_006b8f40(e: &mut Engine, this: Ptr, first: u32, second: u32, use_best: u8) -> bool {
    let base = this.addr();
    let manager = e.call(TASKLET_MANAGER_INSTANCE, &args![]).u32();
    let thread = e.call(CURRENT_THREAD_ID, &args![]).u32();
    let running = e.vcall(manager, TASKLET_SLOT, &args![thread]).u8();
    e.mem.set_u8(base.wrapping_add(RUNNING_IN_TASKLET), running);
    let found = fn_006b9180(e, this);
    let (result, node) = if found != 0 {
        (true, found)
    } else if use_best != 0 {
        (false, e.mem.u32(base.wrapping_add(BEST_NODE)))
    } else {
        (false, 0)
    };
    if node != 0 {
        e.call(BUILD_PATH, &args![this, node, first, second]);
    }
    result
}

// Translated from 006b8c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 0x20`): sets up and runs a search: the
/// lock data, the start location (copied), start mesh, goal location
/// (copied), goal mesh, two words passed on to the path step, and the door
/// lock disposition. Same space when the start mesh's space (`00690800`)
/// is set and the goal's is the same. The start heuristic is the distance
/// from the start mesh's point (+0x10) to the goal location times the
/// double at `0101ffa0` in the same space, else 1.0 (`01016970`'s value is
/// stored first and replaced). The start node is made from the start mesh
/// (`006b8ec0`, the data at the stack word's own address) and the search
/// runs (`006b8f40` with the best node allowed); answers its result.
pub fn fn_006b8c50(
    e: &mut Engine,
    this: Ptr,
    lock_data: u32,
    start: Ptr,
    start_mesh: u32,
    goal: Ptr,
    goal_mesh: u32,
    first: u32,
    second: u32,
    disposition: u32,
) -> bool {
    let base = this.addr();
    let words = e.mem.bytes(start.addr(), 0xc);
    e.mem.write(base.wrapping_add(START_LOCATION), &words);
    e.mem.set_u32(base.wrapping_add(START_MESH), start_mesh);
    let words = e.mem.bytes(goal.addr(), 0xc);
    e.mem.write(base.wrapping_add(GOAL_LOCATION), &words);
    e.mem.set_u32(base.wrapping_add(GOAL_MESH), goal_mesh);
    e.mem.set_u32(base.wrapping_add(ACTOR_LOCK_DATA), lock_data);
    e.mem
        .set_u32(base.wrapping_add(DOOR_LOCK_DISPOSITION), disposition);
    e.mem.set_u8(base.wrapping_add(SAME_SPACE), 0);
    if e.call(MESH_SPACE, &args![start_mesh]).u32() != 0 {
        let start_space = e.call(MESH_SPACE, &args![start_mesh]).u32();
        if start_space == e.call(MESH_SPACE, &args![goal_mesh]).u32() {
            e.mem.set_u8(base.wrapping_add(SAME_SPACE), 1);
        }
    }
    // The exe first stores this value in the local, which both branches
    // then overwrite.
    let _overwritten = x87_quiet_bits(e.global::<u32>(START_HEURISTIC));
    let heuristic = if e.mem.u8(base.wrapping_add(SAME_SPACE)) != 0 {
        e.with_stack(0xc, |e, difference| {
            let mesh = e.mem.u32(base.wrapping_add(START_MESH));
            let at = e
                .call(
                    POINT_SUBTRACT,
                    &args![
                        base.wrapping_add(GOAL_LOCATION),
                        difference,
                        mesh.wrapping_add(0x10)
                    ],
                )
                .u32();
            let length = e.call(POINT_LENGTH, &args![at]).f64();
            x87_store32(x87_mul(length, double_at(e, HEURISTIC_SCALE)))
        })
    } else {
        x87_store32(1.0)
    };
    e.with_stack(4, |e, mesh_slot| {
        e.mem.set_u32(mesh_slot.addr(), start_mesh);
        fn_006b8ec0(
            e,
            this,
            start_mesh,
            mesh_slot,
            f32::from_bits(x87_quiet_bits(heuristic)),
        );
    });
    fn_006b8f40(e, this, first, second, 1)
}

/// The point a mesh contributes: the search's own location for its start
/// (or goal) mesh, else the mesh's point (+0x10); copied into `into`.
fn mesh_point(e: &mut Engine, this: u32, mesh: u32, own_mesh: u32, own_location: u32, into: u32) {
    let from = if mesh == e.mem.u32(this.wrapping_add(own_mesh)) {
        this.wrapping_add(own_location)
    } else {
        mesh.wrapping_add(0x10)
    };
    let words = e.mem.bytes(from, 0xc);
    e.mem.write(into, &words);
}

/// The length of `to - from` (`00439ef0` into a scratch point, `00457990`).
fn distance(e: &mut Engine, to: u32, from: u32) -> f64 {
    e.with_stack(0xc, |e, scratch| {
        let at = e.call(POINT_SUBTRACT, &args![to, scratch, from]).u32();
        e.call(POINT_LENGTH, &args![at]).f64()
    })
}

/// The new node's heuristic for a connection to `mesh`: in the same space
/// the distance from the goal location to the mesh's point times the double
/// at `0101ffa0`, else 1.0.
fn connection_heuristic(e: &mut Engine, this: u32, mesh: u32) -> u32 {
    if e.mem.u8(this.wrapping_add(SAME_SPACE)) != 0 {
        let length = distance(e, this.wrapping_add(GOAL_LOCATION), mesh.wrapping_add(0x10));
        x87_store32(x87_mul(length, double_at(e, HEURISTIC_SCALE)))
    } else {
        x87_store32(1.0)
    }
}

/// One adjacency loop of [`nav_mesh_info_search_get_node_connections`]: for
/// each non-null mesh in the array at `info + array`, the cost from the
/// node's mesh point to the mesh's (`distance * factor`, times the double
/// at `01021928` when `scaled`, plus the node's cost) goes through
/// `006b8fe0`; a new node gets the mesh and its heuristic, and a new or
/// cheaper node gets no parent link and is added to the output array.
fn add_mesh_connections(
    e: &mut Engine,
    this: Ptr,
    info: u32,
    array: u32,
    scaled: bool,
    costs: (u32, u32),
    out: Ptr,
) {
    let base = this.addr();
    let (factor, cost) = costs;
    let list = info.wrapping_add(array);
    let count = e.call(ARRAY_SIZE, &args![list]).u32();
    let mut index: u32 = 0;
    while index < count {
        let at = e.call(ARRAY_ELEMENT_ADDRESS, &args![list, index]).u32();
        let other = e.mem.u32(at);
        if other != 0 {
            let new_cost = e.with_stack(0x18, |e, points| {
                let from = points.addr();
                let to = from + 0xc;
                mesh_point(e, base, info, START_MESH, START_LOCATION, from);
                mesh_point(e, base, other, GOAL_MESH, GOAL_LOCATION, to);
                let length = distance(e, to, from);
                let mut value = x87_mul(length, x87_m32(factor));
                if scaled {
                    value = x87_mul(value, double_at(e, EDGE_SCALE));
                }
                x87_store32(x87_add(value, x87_m32(cost)))
            });
            e.with_stack(4, |e, local| {
                let result = fn_006b8fe0(
                    e,
                    this,
                    other,
                    f32::from_bits(x87_quiet_bits(new_cost)),
                    local,
                );
                if result == 0 {
                    let node = e.mem.u32(local.addr());
                    e.mem.set_u32(node.wrapping_add(NODE_DATA), other);
                    let heuristic = connection_heuristic(e, base, other);
                    let node = e.mem.u32(local.addr());
                    e.mem.set_u32(node.wrapping_add(NODE_HEURISTIC), heuristic);
                }
                if result <= 1 {
                    let node = e.mem.u32(local.addr());
                    e.mem.set_u32(node.wrapping_add(NODE_PARENT_LINK), 0);
                    e.call(ARRAY_ADD, &args![out, local]);
                }
            });
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 006b8490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshInfoSearch::GetNodeConnections` (Xbox PDB, `RET 8`: the node,
/// the output array): adds the node's neighbours to the array and answers
/// true. The cost factor is `1 - 007df1f0(mesh) * 100` (x87 `FSUBRP`,
/// clamped by `005330e0` to the float at `0101e2bc` and 1.0). The meshes of
/// the node's mesh's two adjacency arrays (+0x24, edges scaled by the
/// double at `01021928`; +0x34, unscaled) are added. With lock data or the
/// door disposition 1, each door of the array at +0x44 (skipped when bit
/// 0x800 of its flags is set) is checked with the temporary door lock
/// (`DoorLock::SetFromDoorRef`, `CanActorUnlockDoorLock`): a locked or
/// unopenable door is skipped for disposition 0 and costs the penalty at
/// `0106c238` for disposition 2, as does a minimal-use door
/// (`TESObjectDOOR::IsMinimalUse` of its base); for disposition 2 a door
/// `0057b4b0` refuses is skipped. Its teleport (`00568e50`) destination's
/// portal (`ExtraDataList::GetNavMeshPortal` of `00559450`'s reference's
/// extra list) gives the mesh on the other side (`00693d70` on the `TES`'s
/// nav mesh info map, unless `0068f320`); the cost is the walk to this
/// door plus the walk from the other door, times the factor and the double
/// at `01021928`, plus the node's cost and the penalty; a new node's
/// heuristic is twice the node's in the same space (else 1.0), and the
/// door is the parent link.
pub fn nav_mesh_info_search_get_node_connections(
    e: &mut Engine,
    this: Ptr,
    node: Ptr,
    out: Ptr,
) -> bool {
    let base = this.addr();
    let cost = x87_quiet_bits(e.mem.u32(node.addr()));
    let info = e.mem.u32(node.addr().wrapping_add(NODE_DATA));
    let factor = e.with_stack(4, |e, factor| {
        let value = e.call(INFO_COST_VALUE, &args![info]).f64();
        let scaled = x87_mul(value, double_at(e, FACTOR_SCALE));
        e.mem
            .set_u32(factor.addr(), x87_store32(x87_sub(1.0, scaled)));
        let floor = x87_quiet_bits(e.global::<u32>(FACTOR_FLOOR));
        e.call(CLAMP, &args![factor, floor, x87_store32(1.0)]);
        e.mem.u32(factor.addr())
    });
    add_mesh_connections(e, this, info, 0x24, true, (factor, cost), out);
    add_mesh_connections(e, this, info, 0x34, false, (factor, cost), out);
    if e.mem.u32(base.wrapping_add(ACTOR_LOCK_DATA)) == 0
        && e.mem.u32(base.wrapping_add(DOOR_LOCK_DISPOSITION)) != 1
    {
        return true;
    }
    let doors = info.wrapping_add(0x44);
    let count = e.call(ARRAY_SIZE, &args![doors]).u32();
    let mut index: u32 = 0;
    while index < count {
        let at = e.call(ARRAY_ELEMENT_ADDRESS, &args![doors, index]).u32();
        let door = e.mem.u32(at);
        index = index.wrapping_add(1);
        if door == 0 || e.call(REFERENCE_FLAG_800, &args![door]).bool() {
            continue;
        }
        add_door_connection(e, this, node, info, door, (factor, cost), out);
    }
    true
}

/// One door of [`nav_mesh_info_search_get_node_connections`] (from
/// `006b88d2` to the loop's end).
fn add_door_connection(
    e: &mut Engine,
    this: Ptr,
    node: Ptr,
    info: u32,
    door: u32,
    costs: (u32, u32),
    out: Ptr,
) {
    let base = this.addr();
    let (factor, cost) = costs;
    let lock = base.wrapping_add(TEMP_DOOR_LOCK);
    e.call(DOOR_LOCK_SET_FROM_DOOR, &args![lock, door]);
    let (can_unlock, blocked) = e.with_stack(4, |e, flag| {
        let actor = e.mem.u32(base.wrapping_add(ACTOR_LOCK_DATA));
        let can = e
            .call(DOOR_LOCK_CAN_UNLOCK, &args![lock, actor, door, flag])
            .u8();
        (can, e.mem.u8(flag.addr()))
    });
    let mut penalty = x87_store32(0.0);
    if can_unlock == 0 || blocked != 0 {
        match e.mem.u32(base.wrapping_add(DOOR_LOCK_DISPOSITION)) {
            0 => return,
            2 => penalty = x87_quiet_bits(e.global::<u32>(DOOR_PENALTY)),
            _ => {}
        }
    }
    let door_base = e.call(REFERENCE_BASE_FORM, &args![door]).u32();
    if e.call(DOOR_IS_MINIMAL_USE, &args![door_base]).bool() {
        penalty = x87_quiet_bits(e.global::<u32>(DOOR_PENALTY));
    }
    if e.mem.u32(base.wrapping_add(DOOR_LOCK_DISPOSITION)) == 2
        && e.call(DOOR_TEST, &args![door]).bool()
    {
        return;
    }
    if e.call(DOOR_TELEPORT, &args![door]).u32() == 0 {
        return;
    }
    let teleport = e.call(DOOR_TELEPORT, &args![door]).u32();
    let destination = e.call(FIRST_WORD, &args![teleport]).u32();
    if destination == 0 {
        return;
    }
    let extra = e.call(REFERENCE_EXTRA_LIST, &args![destination]).u32();
    let portal = e.call(GET_NAV_MESH_PORTAL, &args![extra]).u32();
    if portal == 0 {
        return;
    }
    let tes = e.global::<u32>(TES_SINGLETON);
    let map = e.call(GET_NAV_MESH_INFO_MAP, &args![tes]).u32();
    let key = e.mem.u32(portal.wrapping_add(0xc));
    let other = e.call(INFO_MAP_LOOKUP, &args![map, key]).u32();
    if other == 0 || e.call(INFO_TEST, &args![other]).bool() {
        return;
    }
    let new_cost = e.with_stack(0x18, |e, points| {
        let from = points.addr();
        let to = from + 0xc;
        mesh_point(e, base, info, START_MESH, START_LOCATION, from);
        mesh_point(e, base, other, GOAL_MESH, GOAL_LOCATION, to);
        let teleport = e.call(DOOR_TELEPORT, &args![destination]).u32();
        let there = e.call(PLUS_4, &args![teleport]).u32();
        let first = distance(e, there, from);
        let teleport = e.call(DOOR_TELEPORT, &args![door]).u32();
        let here = e.call(PLUS_4, &args![teleport]).u32();
        let second = distance(e, to, here);
        let mut value = x87_store32(x87_add(second, first));
        value = x87_store32(x87_mul(x87_load32(value), x87_m32(factor)));
        value = x87_store32(x87_mul(x87_load32(value), double_at(e, EDGE_SCALE)));
        value = x87_store32(x87_add(x87_load32(value), x87_m32(cost)));
        x87_store32(x87_add(x87_load32(value), x87_m32(penalty)))
    });
    e.with_stack(4, |e, local| {
        let result = fn_006b8fe0(
            e,
            this,
            other,
            f32::from_bits(x87_quiet_bits(new_cost)),
            local,
        );
        if result == 0 {
            let made = e.mem.u32(local.addr());
            e.mem.set_u32(made.wrapping_add(NODE_DATA), other);
            let heuristic = if e.mem.u8(base.wrapping_add(SAME_SPACE)) != 0 {
                let value = fn_006b9130(e, node);
                x87_store32(x87_add(value, value))
            } else {
                x87_store32(1.0)
            };
            let made = e.mem.u32(local.addr());
            e.mem.set_u32(made.wrapping_add(NODE_HEURISTIC), heuristic);
        }
        if result <= 1 {
            let made = e.mem.u32(local.addr());
            e.mem.set_u32(made.wrapping_add(NODE_PARENT_LINK), door);
            e.call(ARRAY_ADD, &args![out, local]);
        }
    });
}

/// `MemoryManager` deallocation (`00401030`, cdecl, the block).
const DEALLOCATE: u32 = 0x0040_1030;

// Translated from 006b83f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshInfoSearch` scalar deleting destructor (engine map, from the
/// vtable; `RET 4`): the destructor body `006b8420`, then `00401030(this)`
/// when bit 0 of `flags` is set. Returns `this`.
pub fn nav_mesh_info_search_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_006b8420(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 006b9150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AStarSearch<const NavMeshInfo *, TESObjectREFR *>` scalar deleting
/// destructor (engine map, from the vtable; `RET 4`): the destructor body
/// `006b8e40`, then `00401030(this)` when bit 0 of `flags` is set. Returns
/// `this`.
pub fn a_star_search_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006b8e40(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 006b9630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<AStarNode<const NavMeshInfo *, TESObjectREFR *> *,1024>`
/// scalar deleting destructor (engine map, from the vtable; `RET 4`): the
/// destructor body `006b9610`, then `00401030(this)` when bit 0 of `flags`
/// is set. Returns `this`.
pub fn bs_simple_array_a_star_node_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_006b9610(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 006b9730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapArray<AStarNode<const NavMeshInfo *, TESObjectREFR *> *,1024>`
/// scalar deleting destructor (engine map, from the vtable; `RET 4`): the
/// destructor body `006b9660`, then `00401030(this)` when bit 0 of `flags`
/// is set. Returns `this`.
pub fn bs_scrap_array_a_star_node_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_006b9660(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006b_83f0, nav_mesh_info_search_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x006b_9150, a_star_search_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x006b_9630, bs_simple_array_a_star_node_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x006b_9730, bs_scrap_array_a_star_node_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x006b_9610, fn_006b9610(Ptr)),
        entry!(0x006b_9130, fn_006b9130(Ptr) -> f64),
        entry!(0x006b_9710, fn_006b9710(Ptr) -> f64),
        entry!(0x006b_96c0, fn_006b96c0(Ptr) -> Ptr),
        entry!(0x006b_9420, fn_006b9420(Ptr) -> u32),
        entry!(0x006b_9400, fn_006b9400(Ptr) -> Ptr),
        entry!(0x006b_9470, fn_006b9470(Ptr) -> Ptr),
        entry!(0x006b_9540, fn_006b9540(Ptr, Ptr)),
        entry!(0x006b_94a0, fn_006b94a0(Ptr, Ptr)),
        entry!(0x006b_8c20, nav_mesh_info_search_is_goal_node(Ptr, Ptr) -> u32),
        entry!(0x006b_8fe0, fn_006b8fe0(Ptr, u32, f32, Ptr) -> u32),
        entry!(0x006b_8ec0, fn_006b8ec0(Ptr, u32, Ptr, f32) -> u32),
        entry!(0x006b_9760, fn_006b9760(Ptr) -> Ptr),
        entry!(0x006b_9590, fn_006b9590(Ptr) -> Ptr),
        entry!(0x006b_9660, fn_006b9660(Ptr)),
        entry!(0x006b_8d90, fn_006b8d90(Ptr) -> Ptr),
        entry!(0x006b_8e40, fn_006b8e40(Ptr)),
        entry!(0x006b_8420, fn_006b8420(Ptr)),
        entry!(0x006b_9180, fn_006b9180(Ptr) -> u32),
        entry!(0x006b_8f40, fn_006b8f40(Ptr, u32, u32, u8) -> bool),
        entry!(0x006b_8c50, fn_006b8c50(Ptr, u32, Ptr, u32, Ptr, u32, u32, u32, u32) -> bool),
        entry!(0x006b_8490, nav_mesh_info_search_get_node_connections(Ptr, Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod array_destructor_tests {
    use super::*;

    fn check_array_destructor(address: u32, vtable: u32, release: u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.register(release, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(address, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), vtable);
        let hits: Vec<_> = log.iter().filter(|c| c.0 == release).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![this, 1]);
    }

    #[test]
    fn fn_006b9610_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x006b_9610, ARRAY_VTABLE_0106C264, ARRAY_RELEASE_008454F0);
    }
}

#[cfg(test)]
mod b025_tests {
    use super::*;

    const SEARCH_VTABLE: u32 = 0x00ab_0000;
    const CONNECTIONS_FN: u32 = 0x00ab_1000;
    const IS_GOAL_FN: u32 = 0x00ab_1004;
    const MANAGER: u32 = 0x00ac_0000;
    const MANAGER_VTABLE: u32 = 0x00ac_0100;
    const TASKLET_FN: u32 = 0x00ac_1000;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// Doubles: one queue bucket, map buckets by key modulo 4, nodes from the
    /// heap, and arrays as (+4 buffer of 0x40 words, +8 size).
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x10000);
        e.map(0x0106_c000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        e.set_global::<f32>(QUEUE_START_FITNESS, -1.0);
        e.set_global::<f32>(START_HEURISTIC, f32::MAX);
        e.mem.set_f64(HEURISTIC_SCALE, 0.1);
        e.mem.set_f64(FACTOR_SCALE, 100.0);
        e.set_global::<f32>(FACTOR_FLOOR, 0.1);
        e.mem.set_f64(EDGE_SCALE, 3.0);
        e.set_global::<f32>(DOOR_PENALTY, 409600.0);
        e.register(QUEUE_BUCKET_INDEX, |_, _| 0u32.into_ret());
        e.register(MAP_BUCKET_INDEX, |_, a| (a[1] % 4).into_ret());
        e.register(SCRAP_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(ARRAY_SIZE, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(ARRAY_ELEMENT_ADDRESS, |e, a| {
            (e.mem.u32(a[0] + 4) + 4 * a[1]).into_ret()
        });
        e.register(ARRAY_ADD, |e, a| {
            if e.mem.u32(a[0] + 4) == 0 {
                let buffer = e.mem.alloc(0x100);
                e.mem.set_u32(a[0] + 4, buffer);
            }
            let size = e.mem.u32(a[0] + 8);
            let value = e.mem.u32(a[1]);
            let buffer = e.mem.u32(a[0] + 4);
            e.mem.set_u32(buffer + 4 * size, value);
            e.mem.set_u32(a[0] + 8, size + 1);
            Ret::default()
        });
        e.register(ARRAY_RELEASE_008454F0, |e, a| {
            e.mem.set_u32(a[0] + 8, 0);
            Ret::default()
        });
        e.register(POINT_SUBTRACT, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) - e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        e.register(POINT_LENGTH, |e, a| {
            let (x, y, z) = (e.mem.f32(a[0]), e.mem.f32(a[0] + 4), e.mem.f32(a[0] + 8));
            Ret {
                st0: f64::from(x * x + y * y + z * z).sqrt(),
                ..Ret::default()
            }
        });
        for addr in [
            FILL_MEMORY,
            SCRAP_HEAP_INIT,
            SIMPLE_ARRAY_INIT,
            NODE_MAP_CONSTRUCT,
            NODE_MAP_DESTRUCT,
            NODE_HEAP_DESTRUCT,
            DOOR_LOCK_DESTRUCT,
            BUILD_PATH,
            TASKLET_YIELD,
            CLAMP,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(CURRENT_THREAD_HEAP_HOLDER, |_, _| 0x5000u32.into_ret());
        e.register(GET_THREAD_SCRAP_HEAP, |_, a| (a[0] + 1).into_ret());
        e.register(TASKLET_MANAGER_INSTANCE, |_, _| MANAGER.into_ret());
        e.register(CURRENT_THREAD_ID, |_, _| 7u32.into_ret());
        e.register(TASKLET_FN, |_, _| 0u32.into_ret());
        e.map(MANAGER, 4);
        e.mem.set_u32(MANAGER, MANAGER_VTABLE);
        e.put_vtable(MANAGER_VTABLE, &[0, 0, 0, TASKLET_FN]);
        e.put_vtable(SEARCH_VTABLE, &[0, CONNECTIONS_FN, IS_GOAL_FN]);
        e
    }

    fn search(e: &mut Engine) -> u32 {
        let s = e.mem.alloc(0x20a0);
        e.mem.set_u32(s, SEARCH_VTABLE);
        s
    }

    fn node(e: &mut Engine, cost: f32, heuristic: f32) -> u32 {
        let n = e.mem.alloc(0x24);
        e.mem.set_f32(n, cost);
        e.mem.set_f32(n + 4, heuristic);
        n
    }

    #[test]
    fn node_basics() {
        let mut e = engine();
        let n = node(&mut e, 1.5, 2.25);
        assert_eq!(e.call(0x006b_9130, &args![n]).f64(), 2.25);
        assert_eq!(e.call(0x006b_9710, &args![n]).f64(), 3.75);
        e.mem.write(n, &[0xff; 0x24]);
        assert_eq!(e.call(0x006b_96c0, &args![n]).u32(), n);
        assert_eq!(
            (0..9).map(|i| e.mem.u32(n + 4 * i)).collect::<Vec<_>>(),
            [0, 0, 0xffff_ffff, 0xffff_ffff, 0, 0, 0, 0, 0]
        );
        let heap = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        let made = e.call(0x006b_9420, &args![heap]).u32();
        assert_eq!(calls_to(&e, SCRAP_ALLOCATE), vec![vec![0x24, heap]]);
        assert_eq!(e.mem.u32(made + 0x20), 0);
        e.register(SCRAP_ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x006b_9420, &args![heap]).u32(), 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006b_9400, &args![heap]).u32(), heap);
        assert_eq!(calls_to(&e, SCRAP_HEAP_INIT), vec![vec![heap, 0x4800]]);
    }

    #[test]
    fn queue_construct_insert_and_remove() {
        let mut e = engine();
        let queue = e.mem.alloc(0x54);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006b_9470, &args![queue]).u32(), queue);
        assert_eq!(e.mem.f32(queue), -1.0);
        assert_eq!(calls_to(&e, FILL_MEMORY), vec![vec![queue + 4, 0, 0x50]]);
        // Inserted in ascending fitness.
        let a = node(&mut e, 1.0, 1.0);
        let b = node(&mut e, 5.0, 0.0);
        let c = node(&mut e, 3.0, 0.0);
        for n in [a, b, c] {
            e.call(0x006b_94a0, &args![queue, n]);
        }
        let order = |e: &Engine| {
            let mut v = vec![];
            let mut n = e.mem.u32(queue + 4);
            while n != 0 {
                v.push(n);
                n = e.mem.u32(n + 0x18);
            }
            v
        };
        assert_eq!(order(&e), [a, c, b]);
        assert_eq!(e.mem.u32(a + 0x14), queue + 4);
        assert_eq!(e.mem.u32(c + 0x14), a + 0x18);
        // Removing the middle one relinks its neighbours.
        e.call(0x006b_9540, &args![queue, c]);
        assert_eq!(order(&e), [a, b]);
        assert_eq!(e.mem.u32(b + 0x14), a + 0x18);
        assert_eq!((e.mem.u32(c + 0x14), e.mem.u32(c + 0x18)), (0, 0));
        // A node without links: nothing to unlink.
        e.call(0x006b_9540, &args![queue, c]);
        assert_eq!(order(&e), [a, b]);
    }

    #[test]
    fn map_lookup_new_cheaper_and_worse() {
        let mut e = engine();
        let s = search(&mut e);
        let out = e.mem.alloc(4);
        assert_eq!(e.call(0x006b_8fe0, &args![s, 6u32, 4.0f32, out]).u32(), 0);
        let made = e.mem.u32(out);
        assert_eq!((e.mem.u32(made + 0x1c), e.mem.f32(made)), (6, 4.0));
        assert_eq!(e.mem.u32(s + 0x14 + 4 * 2), made);
        // A second key in the same bucket goes first.
        assert_eq!(e.call(0x006b_8fe0, &args![s, 10u32, 1.0f32, out]).u32(), 0);
        let second = e.mem.u32(out);
        assert_eq!(e.mem.u32(second + 0x20), made);
        // Not cheaper: 2.
        e.mem.set_u32(out, 0);
        assert_eq!(e.call(0x006b_8fe0, &args![s, 6u32, 4.0f32, out]).u32(), 2);
        assert_eq!(e.mem.u32(out), 0);
        // Cheaper (found further down the bucket): 1, out of the queue, new cost.
        e.call(0x006b_94a0, &args![s + 0x1ff4, made]);
        assert_eq!(e.call(0x006b_8fe0, &args![s, 6u32, 2.0f32, out]).u32(), 1);
        assert_eq!(e.mem.u32(out), made);
        assert_eq!(e.mem.f32(made), 2.0);
        assert_eq!(e.mem.u32(s + 0x1ff8), 0);
    }

    #[test]
    fn start_node_and_constructors() {
        let mut e = engine();
        let s = search(&mut e);
        let data = e.mem.alloc(4);
        e.mem.set_u32(data, 0x777);
        let n = e.call(0x006b_8ec0, &args![s, 3u32, data, 9.0f32]).u32();
        assert_eq!(
            (e.mem.f32(n), e.mem.f32(n + 4), e.mem.u32(n + 8)),
            (0.0, 9.0, 0x777)
        );
        assert_eq!(e.mem.u32(s + 0x2050), n);
        assert_eq!(e.mem.u32(s + 0x1ff8), n);
        // Again: not cheaper, null.
        assert_eq!(e.call(0x006b_8ec0, &args![s, 3u32, data, 9.0f32]).u32(), 0);
        // The constructors and destructors.
        let s = search(&mut e);
        e.mem.write(s + 0x2048, &[0xff; 0x10]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006b_8d90, &args![s]).u32(), s);
        assert_eq!(e.mem.u32(s), A_STAR_SEARCH_VTABLE);
        assert_eq!(calls_to(&e, SCRAP_HEAP_INIT), vec![vec![s + 4, 0x4800]]);
        assert_eq!(calls_to(&e, NODE_MAP_CONSTRUCT), vec![vec![s + 0x14]]);
        assert_eq!(e.mem.f32(s + 0x1ff4), -1.0);
        assert_eq!(
            [0x2048, 0x204c, 0x2050].map(|o| e.mem.u32(s + o)),
            [0, 0, 0]
        );
        assert_eq!(e.mem.u8(s + 0x2054), 0);
        e.call_log = Some(vec![]);
        e.call(0x006b_8420, &args![s]);
        let order: Vec<(u32, Vec<u32>)> = e.call_log.take().unwrap().into_iter().skip(1).collect();
        assert_eq!(
            order,
            vec![
                (DOOR_LOCK_DESTRUCT, vec![s + 0x2084]),
                (NODE_MAP_DESTRUCT, vec![s + 0x1ff4]),
                (NODE_MAP_DESTRUCT, vec![s + 0x14]),
                (NODE_HEAP_DESTRUCT, vec![s + 4]),
            ]
        );
        assert_eq!(e.mem.u32(s), A_STAR_SEARCH_VTABLE);
    }

    #[test]
    fn scrap_array_construct_and_destroy() {
        let mut e = engine();
        let a = e.mem.alloc(0x14);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006b_9590, &args![a]).u32(), a);
        assert_eq!(e.mem.u32(a), SCRAP_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(a + 0x10), 0x5001);
        assert_eq!(calls_to(&e, SIMPLE_ARRAY_INIT), vec![vec![a, 0, 0]; 2]);
        e.call_log = Some(vec![]);
        e.call(0x006b_9660, &args![a]);
        assert_eq!(calls_to(&e, ARRAY_RELEASE_008454F0), vec![vec![a, 1]; 2]);
        assert_eq!(e.mem.u32(a), ARRAY_VTABLE_0106C264);
        assert_eq!(e.call(0x006b_9760, &args![a]).u32(), a);
    }

    #[test]
    fn is_goal_node_compares_the_goal_mesh() {
        let mut e = engine();
        let s = search(&mut e);
        e.mem.set_u32(s + 0x2074, 0x99);
        let n = node(&mut e, 0.0, 0.0);
        e.mem.set_u32(n + 8, 0x99);
        assert_eq!(e.call(0x006b_8c20, &args![s, n]).u32(), 1);
        e.mem.set_u32(n + 8, 0x98);
        assert_eq!(e.call(0x006b_8c20, &args![s, n]).u32(), 0);
    }

    #[test]
    fn search_runs_until_the_goal() {
        let mut e = engine();
        let s = search(&mut e);
        // Goal: data 0x30. Connections: node with data d gets d + 0x10 (new
        // map entries through 006b8fe0), up to 0x30.
        e.register(IS_GOAL_FN, |e, a| {
            u32::from(e.mem.u32(a[1] + 8) == 0x30).into_ret()
        });
        e.register(CONNECTIONS_FN, |e, a| {
            let data = e.mem.u32(a[1] + 8);
            let s = Ptr::new(a[0]);
            let out = e.mem.alloc(4);
            fn_006b8fe0(e, s, data + 0x10, 1.0, Ptr::new(out));
            let n = e.mem.u32(out);
            e.mem.set_u32(n + 8, data + 0x10);
            e.mem.set_f32(n + 4, 100.0 - data as f32);
            e.call(ARRAY_ADD, &args![a[2], out]);
            Ret::default()
        });
        e.register(QUEUE_POP_BEST, |e, a| {
            let first = e.mem.u32(a[0] + 4);
            if first != 0 {
                fn_006b9540(e, Ptr::new(a[0]), Ptr::new(first));
            }
            first.into_ret()
        });
        let data = e.mem.alloc(4);
        e.mem.set_u32(data, 0x10);
        let start = e.call(0x006b_8ec0, &args![s, 0x10u32, data, 50.0f32]).u32();
        e.mem.set_u8(s + 0x2054, 1);
        let goal = e.call(0x006b_9180, &args![s]).u32();
        assert_eq!(e.mem.u32(goal + 8), 0x30);
        assert_eq!(e.mem.u32(s + 0x2048), goal);
        let parent = e.mem.u32(goal + 0x10);
        assert_eq!(e.mem.u32(parent + 8), 0x20);
        assert_eq!(e.mem.u32(parent + 0x10), start);
        // An empty queue: null and the last node kept.
        let s = search(&mut e);
        e.mem.set_u32(s + 0x204c, 0x4444);
        assert_eq!(e.call(0x006b_9180, &args![s]).u32(), 0);
        assert_eq!(e.mem.u32(s + 0x204c), 0);
    }

    #[test]
    fn search_ends_when_the_queue_runs_out_or_a_parent_comes_back() {
        let mut e = engine();
        e.register(IS_GOAL_FN, |_, _| 0u32.into_ret());
        e.register(CONNECTIONS_FN, |_, _| Ret::default());
        e.register(QUEUE_POP_BEST, |e, a| {
            let first = e.mem.u32(a[0] + 4);
            if first != 0 {
                fn_006b9540(e, Ptr::new(a[0]), Ptr::new(first));
            }
            first.into_ret()
        });
        let s = search(&mut e);
        let n = node(&mut e, 0.0, 0.0);
        e.call(0x006b_94a0, &args![s + 0x1ff4, n]);
        assert_eq!(e.call(0x006b_9180, &args![s]).u32(), 0);
        assert_eq!(e.mem.u32(s + 0x204c), n);
        // A connection that is the node's parent: null at once.
        let parent = node(&mut e, 0.0, 0.0);
        let child = node(&mut e, 0.0, 0.0);
        e.mem.set_u32(child + 0x10, parent);
        e.mem.set_u32(0x0101_3000, parent);
        e.register(CONNECTIONS_FN, |e, a| {
            let out = e.mem.alloc(4);
            let parent = e.mem.u32(0x0101_3000);
            e.mem.set_u32(out, parent);
            e.call(ARRAY_ADD, &args![a[2], out]);
            Ret::default()
        });
        let s = search(&mut e);
        e.call(0x006b_94a0, &args![s + 0x1ff4, child]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006b_9180, &args![s]).u32(), 0);
        // Emptied once in the loop, then released twice by the destructor.
        let releases: Vec<u32> = calls_to(&e, ARRAY_RELEASE_008454F0)
            .iter()
            .map(|c| c[1])
            .collect();
        assert_eq!(releases, [0, 1, 1]);
    }

    #[test]
    fn run_uses_the_goal_or_the_best_node() {
        let mut e = engine();
        e.register(IS_GOAL_FN, |_, _| 1u32.into_ret());
        e.register(QUEUE_POP_BEST, |e, a| e.mem.u32(a[0] + 4).into_ret());
        let s = search(&mut e);
        let n = node(&mut e, 0.0, 0.0);
        e.call(0x006b_94a0, &args![s + 0x1ff4, n]);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x006b_8f40, &args![s, 0x11u32, 0x22u32, 0u32])
            .bool());
        assert_eq!(calls_to(&e, BUILD_PATH), vec![vec![s, n, 0x11, 0x22]]);
        assert_eq!(calls_to(&e, TASKLET_FN), vec![vec![MANAGER, 7]]);
        // Nothing found: the best node only when asked.
        let s = search(&mut e);
        e.mem.set_u32(s + 0x2048, 0x4848);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x006b_8f40, &args![s, 1u32, 2u32, 0u32]).bool());
        assert!(calls_to(&e, BUILD_PATH).is_empty());
        e.mem.set_u32(s + 0x2048, 0x4848);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x006b_8f40, &args![s, 1u32, 2u32, 1u32]).bool());
        assert_eq!(calls_to(&e, BUILD_PATH), vec![vec![s, 0x4848, 1, 2]]);
    }

    fn point(e: &mut Engine, x: f32, y: f32) -> u32 {
        let p = e.mem.alloc(0xc);
        e.mem.set_f32(p, x);
        e.mem.set_f32(p + 4, y);
        p
    }

    /// A mesh info: its point at +0x10, three arrays (+0x24, +0x34, +0x44)
    /// of the given items.
    fn mesh(e: &mut Engine, x: f32, y: f32, arrays: [&[u32]; 3]) -> u32 {
        let m = e.mem.alloc(0x60);
        e.mem.set_f32(m + 0x10, x);
        e.mem.set_f32(m + 0x14, y);
        for (k, items) in arrays.iter().enumerate() {
            let a = m + 0x24 + 0x10 * k as u32;
            let buffer = e.mem.alloc(4 * items.len() as u32 + 4);
            for (i, v) in items.iter().enumerate() {
                e.mem.set_u32(buffer + 4 * i as u32, *v);
            }
            e.mem.set_u32(a + 4, buffer);
            e.mem.set_u32(a + 8, items.len() as u32);
        }
        m
    }

    #[test]
    fn set_up_and_run_a_search() {
        let mut e = engine();
        e.register(IS_GOAL_FN, |_, _| 1u32.into_ret());
        e.register(QUEUE_POP_BEST, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(MESH_SPACE, |e, a| e.mem.u32(a[0]).into_ret());
        let s = search(&mut e);
        let start_mesh = mesh(&mut e, 0.0, 0.0, [&[], &[], &[]]);
        let goal_mesh = mesh(&mut e, 0.0, 0.0, [&[], &[], &[]]);
        e.mem.set_u32(start_mesh, 5);
        e.mem.set_u32(goal_mesh, 5);
        let start = point(&mut e, 1.0, 2.0);
        let goal = point(&mut e, 30.0, 40.0);
        assert!(e
            .call(
                0x006b_8c50,
                &args![s, 0xaa_u32, start, start_mesh, goal, goal_mesh, 0x11u32, 0x22u32, 2u32]
            )
            .bool());
        assert_eq!(e.mem.f32(s + 0x2058), 1.0);
        assert_eq!(e.mem.f32(s + 0x206c), 40.0);
        assert_eq!(
            (e.mem.u32(s + 0x2064), e.mem.u32(s + 0x2074)),
            (start_mesh, goal_mesh)
        );
        assert_eq!((e.mem.u32(s + 0x2078), e.mem.u32(s + 0x207c)), (0xaa, 2));
        assert_eq!(e.mem.u8(s + 0x2080), 1);
        // The start node: data the start mesh, heuristic 50 * 0.1.
        let n = e.mem.u32(s + 0x2050);
        assert_eq!(e.mem.u32(n + 8), start_mesh);
        assert_eq!(
            e.mem.f32(n + 4),
            x87_load32(x87_store32(x87_mul(50.0, 0.1))) as f32
        );
        // Different spaces: heuristic 1.0.
        e.mem.set_u32(goal_mesh, 6);
        let s = search(&mut e);
        e.call(
            0x006b_8c50,
            &args![s, 0u32, start, start_mesh, goal, goal_mesh, 0u32, 0u32, 0u32],
        );
        assert_eq!(e.mem.u8(s + 0x2080), 0);
        assert_eq!(e.mem.f32(e.mem.u32(s + 0x2050) + 4), 1.0);
    }

    #[test]
    fn get_node_connections_adds_meshes_and_doors() {
        let mut e = engine();
        e.register(INFO_COST_VALUE, |_, _| Ret::default());
        let s = search(&mut e);
        let a = mesh(&mut e, 3.0, 4.0, [&[], &[], &[]]);
        let b = mesh(&mut e, 0.0, 8.0, [&[], &[], &[]]);
        let other_side = mesh(&mut e, 100.0, 0.0, [&[], &[], &[]]);
        let door = e.mem.alloc(0x40);
        let here = mesh(&mut e, 0.0, 0.0, [&[a], &[0, b], &[0, door]]);
        e.mem.set_u8(s + 0x2080, 1);
        let current = node(&mut e, 10.0, 7.0);
        e.mem.set_u32(current + 8, here);
        let out = e.mem.alloc(0x10);
        // Doors are skipped without lock data and disposition 1.
        assert!(e.call(0x006b_8490, &args![s, current, out]).bool());
        assert_eq!(e.mem.u32(out + 8), 2);
        let buffer = e.mem.u32(out + 4);
        let (na, nb) = (e.mem.u32(buffer), e.mem.u32(buffer + 4));
        // a: 5 * factor 1 * 3 + 10; heuristic from the goal (0,0): 5 * 0.1.
        assert_eq!((e.mem.u32(na + 8), e.mem.f32(na)), (a, 25.0));
        assert_eq!(e.mem.u32(na + 0xc), 0);
        // b: unscaled, 8 + 10.
        assert_eq!((e.mem.u32(nb + 8), e.mem.f32(nb)), (b, 18.0));
        // With disposition 1 (a new search) the door leads to the other side.
        e.register(REFERENCE_FLAG_800, |_, _| 0u32.into_ret());
        e.register(DOOR_LOCK_SET_FROM_DOOR, |_, _| Ret::default());
        e.register(DOOR_LOCK_CAN_UNLOCK, |_, _| 1u32.into_ret());
        e.register(REFERENCE_BASE_FORM, |_, a| a[0].into_ret());
        e.register(DOOR_IS_MINIMAL_USE, |_, _| 0u32.into_ret());
        // The door's teleport: +0 the destination reference, +4 a point.
        let teleport = e.mem.alloc(0x10);
        e.mem.set_u32(teleport, 0x6000);
        e.register_double(DOOR_TELEPORT, move |_, _| teleport.into_ret());
        e.register(FIRST_WORD, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(PLUS_4, |_, a| (a[0] + 4).into_ret());
        e.register(REFERENCE_EXTRA_LIST, |_, a| (a[0] + 0x44).into_ret());
        let portal = e.mem.alloc(0x10);
        e.register_double(GET_NAV_MESH_PORTAL, move |_, _| portal.into_ret());
        e.register(GET_NAV_MESH_INFO_MAP, |_, _| 0x7000u32.into_ret());
        e.register_double(INFO_MAP_LOOKUP, move |_, _| other_side.into_ret());
        e.register(INFO_TEST, |_, _| 0u32.into_ret());
        let s = search(&mut e);
        e.mem.set_u8(s + 0x2080, 1);
        e.mem.set_u32(s + 0x207c, 1);
        let out = e.mem.alloc(0x10);
        let current = node(&mut e, 10.0, 7.0);
        e.mem.set_u32(current + 8, here);
        e.call_log = Some(vec![]);
        assert!(e.call(0x006b_8490, &args![s, current, out]).bool());
        assert_eq!(e.mem.u32(out + 8), 3);
        let made = e.mem.u32(e.mem.u32(out + 4) + 8);
        // Both teleport points are at the origin: 0 + 100, * 3, + 10.
        assert_eq!((e.mem.u32(made + 8), e.mem.f32(made)), (other_side, 310.0));
        assert_eq!(e.mem.f32(made + 4), 14.0);
        assert_eq!(e.mem.u32(made + 0xc), door);
        assert_eq!(
            calls_to(&e, DOOR_LOCK_SET_FROM_DOOR),
            vec![vec![s + 0x2084, door]]
        );
        // Locked with disposition 0: skipped; with 2: the penalty.
        e.register(DOOR_LOCK_CAN_UNLOCK, |_, _| 0u32.into_ret());
        let s = search(&mut e);
        e.mem.set_u8(s + 0x2080, 1);
        e.mem.set_u32(s + 0x2078, 0x99);
        let out = e.mem.alloc(0x10);
        let current = node(&mut e, 10.0, 7.0);
        e.mem.set_u32(current + 8, here);
        e.call(0x006b_8490, &args![s, current, out]);
        assert_eq!(e.mem.u32(out + 8), 2);
        e.mem.set_u32(s + 0x207c, 2);
        e.register(DOOR_TEST, |_, _| 0u32.into_ret());
        let s2 = search(&mut e);
        e.mem.write(s2 + 0x2058, &e.mem.bytes(s + 0x2058, 0x48));
        let out = e.mem.alloc(0x10);
        e.call(0x006b_8490, &args![s2, current, out]);
        let made = e.mem.u32(e.mem.u32(out + 4) + 8);
        assert_eq!(e.mem.f32(made), 409910.0);
    }
}

#[cfg(test)]
mod deleting_destructor_tests {
    use super::*;

    /// Runs a scalar deleting destructor with `flags` over an object big
    /// enough for `NavMeshInfoSearch`, with every outside callee of the
    /// destructor bodies doubled: checks the result, the vtable the body
    /// leaves and whether `00401030` frees the object.
    fn check(address: u32, vtable: u32) {
        for (flags, frees) in [(0u32, false), (1, true), (2, false), (3, true)] {
            let mut e = Engine::new();
            for callee in [
                ARRAY_RELEASE_008454F0,
                DOOR_LOCK_DESTRUCT,
                NODE_MAP_DESTRUCT,
                NODE_HEAP_DESTRUCT,
                DEALLOCATE,
            ] {
                e.register(callee, |_, _| Ret::default());
            }
            let this = e.mem.alloc(0x2100);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(address, &args![this, flags]).u32(), this);
            let log = e.call_log.take().unwrap();
            assert_eq!(e.mem.u32(this), vtable);
            let freed: Vec<_> = log.iter().filter(|c| c.0 == DEALLOCATE).collect();
            assert_eq!(freed.len(), usize::from(frees));
            if frees {
                assert_eq!(freed[0].1, vec![this]);
            }
        }
    }

    #[test]
    fn at_006b83f0_runs_the_search_destructor_then_frees_on_bit_zero() {
        check(0x006b_83f0, A_STAR_SEARCH_VTABLE);
    }

    #[test]
    fn at_006b9150_runs_the_a_star_destructor_then_frees_on_bit_zero() {
        check(0x006b_9150, A_STAR_SEARCH_VTABLE);
    }

    #[test]
    fn at_006b9630_runs_the_array_destructor_then_frees_on_bit_zero() {
        check(0x006b_9630, ARRAY_VTABLE_0106C264);
    }

    #[test]
    fn at_006b9730_runs_the_scrap_array_destructor_then_frees_on_bit_zero() {
        check(0x006b_9730, ARRAY_VTABLE_0106C264);
    }
}
