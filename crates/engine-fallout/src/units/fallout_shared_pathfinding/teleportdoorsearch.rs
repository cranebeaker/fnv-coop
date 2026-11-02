//! `fallout shared/pathfinding/teleportdoorsearch.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAV?$AStarNode@UParentSpaceNode@@UTeleportLink@@@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0106D930: u32 = 0x0106_d930;
/// `008454f0` (unnamed in the engine map): `thiscall`, `this` plus one stack word (`RET 4`). When the
/// buffer pointer (+4) is set it runs the instance's element destroy step; with the word non-zero it
/// then calls the array's virtual slot +8 on the buffer and clears the buffer pointer and the reserved
/// size (+0xc); finally it clears the size (+8).
const ARRAY_RELEASE_008454F0: u32 = 0x0084_54f0;

// Translated from 006f4640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAV?$AStarNode@UParentSpaceNode@@UTeleportLink@@@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_006f4640(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106D930);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

/// `TeleportDoorSearch`'s vtable (`0106d8fc`), the `AStarSearch` base's
/// (`0106d90c`) and the node-array vtable `006f45c0` writes (`0106d91c`).
const TELEPORT_DOOR_SEARCH_VTABLE: u32 = 0x0106_d8fc;
const A_STAR_SEARCH_VTABLE: u32 = 0x0106_d90c;
const NODE_ARRAY_VTABLE: u32 = 0x0106_d91c;
/// The search's members (PC offsets, from the code): the node pool at +4,
/// the node hash at +0x14, the open-list buckets at +0x1FF4, the best node
/// +0x2048, the current node +0x204C, the last added node +0x2050, the
/// "time-sliced" byte +0x2054; `TeleportDoorSearch`'s start location data
/// +0x2058 (worldspace flag byte, worldspace +0x205C, cell +0x2060), its
/// start `PathingLocation` +0x2064, the goal (flag byte +0x208C,
/// worldspace +0x2090, cell +0x2094), the mode +0x2098, the cost limit
/// `float` +0x209C, the actor +0x20A0, the door array +0x20A4 and the lock
/// mode +0x20B4.
const NODE_POOL: u32 = 0x4;
const NODE_HASH: u32 = 0x14;
const OPEN_LIST: u32 = 0x1ff4;
const BEST_NODE: u32 = 0x2048;
const CURRENT_NODE: u32 = 0x204c;
const LAST_ADDED: u32 = 0x2050;
const TIME_SLICED: u32 = 0x2054;
const START_DATA: u32 = 0x2058;
const START_WORLDSPACE: u32 = 0x205c;
const START_CELL: u32 = 0x2060;
const START_LOCATION: u32 = 0x2064;
const GOAL_IS_WORLDSPACE: u32 = 0x208c;
const GOAL_WORLDSPACE: u32 = 0x2090;
const GOAL_CELL: u32 = 0x2094;
const MODE: u32 = 0x2098;
const COST_LIMIT: u32 = 0x209c;
const ACTOR: u32 = 0x20a0;
const DOORS: u32 = 0x20a4;
const LOCK_MODE: u32 = 0x20b4;
/// The search node (0x38 bytes): cost `float` +0, the heuristic `float`
/// +4, the space data +8 (worldspace flag byte, worldspace +0xC, cell
/// +0x10), the door +0x14, its position +0x18, the parent +0x24, the
/// open-list link (`pprev` +0x28, `next` +0x2C), the hash key +0x30 and
/// the hash chain +0x34.
const NODE_PARENT: u32 = 0x24;
const NODE_PPREV: u32 = 0x28;
const NODE_NEXT: u32 = 0x2c;
const NODE_KEY: u32 = 0x30;
const NODE_HASH_NEXT: u32 = 0x34;

fn word(e: &Engine, address: u32) -> u32 {
    e.mem.u32(address)
}

// Translated from 006f4320 (decompiled, FalloutNV.exe 1.4.0.525)
/// The node pool's constructor (the engine map has no name): `00aa5410`
/// with 0x7000. Answers `this`.
pub fn fn_006f4320(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x00aa_5410, &args![this, 0x7000u32]);
    this
}

// Translated from 006f4880 (decompiled, FalloutNV.exe 1.4.0.525)
/// The node array's base constructor (the engine map has no name): the
/// array vtable `0106d930`, then `006b3eb0` with two zero words. Answers
/// `this`.
pub fn fn_006f4880(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106D930);
    e.call(0x006b_3eb0, &args![this, 0u32, 0u32]);
    this
}

// Translated from 006f47f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The search node's constructor (the engine map has no name): both
/// `float`s 0.0, the member at +0x14 constructed (`006d5320`), the parent,
/// links, key and chain null. Answers `this`.
pub fn fn_006f47f0(e: &mut Engine, this: Ptr) -> Ptr {
    let node = this.addr();
    e.mem.set_f32(node, 0.0);
    e.mem.set_f32(node + 4, 0.0);
    e.call(0x006d_5320, &args![node + 0x14]);
    for offset in [NODE_PARENT, NODE_PPREV, NODE_NEXT, NODE_KEY, NODE_HASH_NEXT] {
        e.mem.set_u32(node + offset, 0);
    }
    this
}

// Translated from 006f4340 (decompiled, FalloutNV.exe 1.4.0.525)
/// A new search node from the pool (the engine map has no name): 0x38
/// bytes from `006a1f90` (`cdecl (size, pool)`), constructed
/// (`006f47f0`); null when the pool gives none.
pub fn fn_006f4340(e: &mut Engine, this: Ptr) -> u32 {
    let block = e.call(0x006a_1f90, &args![0x38u32, this]).u32();
    if block == 0 {
        return 0;
    }
    fn_006f47f0(e, Ptr::new(block)).addr()
}

// Translated from 006f4430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes `node` off its open-list bucket (the engine map has no name):
/// the word its `pprev` points at takes its `next`, the next node's
/// `pprev` takes its `pprev`; both links cleared. `this` is not read.
pub fn fn_006f4430(e: &mut Engine, _this: Ptr, node: u32) {
    let pprev = word(e, node + NODE_PPREV);
    if pprev != 0 {
        let next = word(e, node + NODE_NEXT);
        e.mem.set_u32(pprev, next);
    }
    let next = word(e, node + NODE_NEXT);
    if next != 0 {
        let pprev = word(e, node + NODE_PPREV);
        e.mem.set_u32(next + NODE_PPREV, pprev);
    }
    e.mem.set_u32(node + NODE_PPREV, 0);
    e.mem.set_u32(node + NODE_NEXT, 0);
}

/// `19.0` (`double`, `010356e8`): the bucket scale.
const BUCKET_SCALE: u32 = 0x0103_56e8;

// Translated from 006f4790 (decompiled, FalloutNV.exe 1.4.0.525)
/// The open-list bucket of `node` (the engine map has no name): with its
/// estimate (`006b9710`, stored as a `float`) at or above the `float` at
/// `this`, 0x13; otherwise `estimate * 19.0 / limit` truncated (x87, the
/// compiler's chopping `FISTP qword`, low word).
pub fn fn_006f4790(e: &mut Engine, this: Ptr, node: u32) -> u32 {
    let estimate = x87_store32(e.call(0x006b_9710, &args![node]).f64());
    let estimate = x87_load32(estimate);
    let limit = x87_load32(word(e, this.addr()));
    // Unordered compares go on to the division, as the exe's flag test does.
    if limit <= estimate {
        return 0x13;
    }
    let scale = f64::from_bits(e.global::<u64>(BUCKET_SCALE));
    let value = x87_div(x87_mul(estimate, scale), x87_m32(word(e, this.addr())));
    x87_fistp64_chop(X87Ext::load64(value)) as u32
}

// Translated from 006f4390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Puts `node` into its open-list bucket (the engine map has no name),
/// after every node whose estimate (`006b9710`, kept as a `double`) is
/// below the new node's.
pub fn fn_006f4390(e: &mut Engine, this: Ptr, node: u32) {
    let bucket = fn_006f4790(e, this, node);
    let mut slot = this
        .addr()
        .wrapping_add(bucket.wrapping_mul(4))
        .wrapping_add(4);
    while word(e, slot) != 0 {
        let current = e.call(0x006b_9710, &args![word(e, slot)]).f64();
        let estimate = e.call(0x006b_9710, &args![node]).f64();
        if !(estimate > current) {
            break;
        }
        slot = word(e, slot) + NODE_NEXT;
    }
    if word(e, slot) != 0 {
        let next = word(e, slot);
        e.mem.set_u32(next + NODE_PPREV, node + NODE_NEXT);
    }
    let next = word(e, slot);
    e.mem.set_u32(node + NODE_NEXT, next);
    e.mem.set_u32(node + NODE_PPREV, slot);
    e.mem.set_u32(slot, node);
}

// Translated from 006f46f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes the first node of the first non-empty bucket of the 20 (the
/// engine map has no name); null when all are empty.
pub fn fn_006f46f0(e: &mut Engine, this: Ptr) -> u32 {
    for index in 0..0x14u32 {
        let slot = this.addr() + 4 * index + 4;
        if word(e, slot) == 0 {
            continue;
        }
        let node = word(e, slot);
        let next = word(e, node + NODE_NEXT);
        if next != 0 {
            e.mem.set_u32(next + NODE_PPREV, slot);
            let next = word(e, word(e, slot) + NODE_NEXT);
            e.mem.set_u32(slot, next);
        } else {
            e.mem.set_u32(slot, 0);
        }
        e.mem.set_u32(node + NODE_PPREV, 0);
        e.mem.set_u32(node + NODE_NEXT, 0);
        return node;
    }
    0
}

// Translated from 006f45c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f45c0 - its C++ exception frame (FS:[0]) is not translated
/// The node array's constructor (the engine map has no name): the base
/// (`006f4880`), the vtable `0106d91c`, the thread's scrap heap
/// (`MemoryManager::GetThreadScrapHeap` on `00401020`'s memory manager) at
/// +0x10, then `006b3eb0` with two zero words. Answers `this`.
pub fn fn_006f45c0(e: &mut Engine, this: Ptr) -> Ptr {
    fn_006f4880(e, this);
    e.mem.set_u32(this.addr(), NODE_ARRAY_VTABLE);
    let manager = e.call(0x0040_1020, &args![]).u32();
    let heap = e.call(0x00aa_42e0, &args![manager]).u32();
    e.mem.set_u32(this.addr() + 0x10, heap);
    e.call(0x006b_3eb0, &args![this, 0u32, 0u32]);
    this
}

// Translated from 006f4690 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f4690 - its C++ exception frame (FS:[0]) is not translated
/// The node array's destructor (the engine map has no name): the vtable
/// `0106d91c`, the buffer released (`008454f0` with 1), then the base
/// destructor (`006f4640`).
pub fn fn_006f4690(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), NODE_ARRAY_VTABLE);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
    fn_006f4640(e, this);
}

// Translated from 006f4480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray::SetSize` for 16-byte elements (the engine map has no
/// name; `thiscall (size, shrink)`, `RET 8`; data +4, size +8, capacity
/// +0xC): size 0 releases (`008454f0` with `shrink`); a size beyond the
/// capacity allocates (vtable +4) or reallocates (`009f6540`) and builds
/// the new elements (`006dbcf0`); a smaller size destroys the tail
/// (`0072ba80`) and, with `shrink` and a size at most a quarter of the
/// capacity, reallocates to it; otherwise the new elements are built.
pub fn fn_006f4480(e: &mut Engine, this: Ptr, size: u32, shrink: bool) {
    let array = this.addr();
    if size == 0 {
        e.call(ARRAY_RELEASE_008454F0, &args![this, u32::from(shrink)]);
        return;
    }
    if size > word(e, array + 0xc) {
        if word(e, array + 0xc) == 0 {
            let data = e.vcall(array, 4, &args![size]).u32();
            e.mem.set_u32(array + 4, data);
            e.mem.set_u32(array + 0xc, size);
        } else {
            let current = word(e, array + 8);
            e.call(0x009f_6540, &args![this, size, current]);
            e.mem.set_u32(array + 0xc, size);
        }
        let current = word(e, array + 8);
        let start = (current << 4).wrapping_add(word(e, array + 4));
        e.call(0x006d_bcf0, &args![this, start, size.wrapping_sub(current)]);
        e.mem.set_u32(array + 8, size);
    } else if size < word(e, array + 8) {
        let current = word(e, array + 8);
        let start = (size << 4).wrapping_add(word(e, array + 4));
        e.call(0x0072_ba80, &args![this, start, current.wrapping_sub(size)]);
        e.mem.set_u32(array + 8, size);
        if shrink && size <= word(e, array + 0xc) >> 2 {
            e.call(0x009f_6540, &args![this, size, size]);
            e.mem.set_u32(array + 0xc, size);
        }
    } else {
        let current = word(e, array + 8);
        let start = (current << 4).wrapping_add(word(e, array + 4));
        e.call(0x006d_bcf0, &args![this, start, size.wrapping_sub(current)]);
        e.mem.set_u32(array + 8, size);
    }
}

// Translated from 006f3e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds or makes the node for `key` (the engine map has no name;
/// `thiscall (key, cost, &node)`, `RET 0xC`): in the key's hash chain
/// (`00996840` answers the slot), a node with the key answers 1 when
/// `cost` is below its cost (it is taken off the open list and given the
/// cost) and 2 otherwise. Without one, a new node from the pool gets the
/// key and cost, goes to the front of the chain, and 0 is answered. The
/// node goes to `*node` in cases 0 and 1.
pub fn fn_006f3e30(e: &mut Engine, this: Ptr, key: u32, cost: f32, out: Ptr) -> u32 {
    let search = this.addr();
    let cost_bits = x87_quiet_bits(cost.to_bits());
    let index = e.call(0x0099_6840, &args![search + NODE_HASH, key]).u32();
    let slot = search
        .wrapping_add(index.wrapping_mul(4))
        .wrapping_add(NODE_HASH);
    let mut node = word(e, slot);
    while node != 0 {
        if word(e, node + NODE_KEY) == key {
            if cost < e.mem.f32(node) {
                e.mem.set_u32(out.addr(), node);
                let found = word(e, out.addr());
                fn_006f4430(e, Ptr::new(search + OPEN_LIST), found);
                let found = word(e, out.addr());
                e.mem.set_u32(found, cost_bits);
                return 1;
            }
            return 2;
        }
        node = word(e, node + NODE_HASH_NEXT);
    }
    let created = fn_006f4340(e, Ptr::new(search + NODE_POOL));
    e.mem.set_u32(out.addr(), created);
    e.mem.set_u32(created + NODE_KEY, key);
    e.mem.set_u32(created, cost_bits);
    let first = word(e, slot);
    e.mem.set_u32(created + NODE_HASH_NEXT, first);
    e.mem.set_u32(slot, created);
    0
}

// Translated from 006f3d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds the start node for `key` (the engine map has no name; `thiscall
/// (key, &space data, estimate)`, `RET 0xC`): a new node (cost 0) gets the
/// estimate (through the FPU) and the 3 words of space data, goes into the
/// open list and becomes the last added; an existing cheaper one is
/// answered as it is; otherwise null.
pub fn fn_006f3d00(e: &mut Engine, this: Ptr, key: u32, data: Ptr, estimate: f32) -> u32 {
    let search = this.addr();
    e.with_stack(4, |e, node| {
        let result = fn_006f3e30(e, this, key, 0.0, node);
        let node = word(e, node.addr());
        match result {
            0 => {
                e.mem.set_u32(node + 4, x87_quiet_bits(estimate.to_bits()));
                for i in 0..3 {
                    let value = word(e, data.addr() + 4 * i);
                    e.mem.set_u32(node + 8 + 4 * i, value);
                }
                fn_006f4390(e, Ptr::new(search + OPEN_LIST), node);
                e.mem.set_u32(search + LAST_ADDED, node);
                node
            }
            1 => node,
            _ => 0,
        }
    })
}

// Translated from 006f4230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the path ending at `node` (the engine map has no name;
/// `thiscall (node, points, links)`, `RET 0xC`): false without a node.
/// The point array is sized to the number of nodes up the parent chain
/// (`006a8890` with 1) and the link array to one less (`006f4480` with
/// 1); walking from `node` up, each node's position (+8, 3 words) goes to
/// its point (`006a1440`, last first) and, but for the first node, its 4
/// words at +0x14 to the link before it (`00558dd0`). True.
pub fn fn_006f4230(e: &mut Engine, _this: Ptr, node: u32, points: Ptr, links: Ptr) -> bool {
    if node == 0 {
        return false;
    }
    let mut count = 0u32;
    let mut current = node;
    while current != 0 {
        count = count.wrapping_add(1);
        current = word(e, current + NODE_PARENT);
    }
    e.call(0x006a_8890, &args![points, count, 1u32]);
    fn_006f4480(e, links, count.wrapping_sub(1), true);
    let mut current = node;
    let mut index = count.wrapping_sub(1);
    while current != 0 {
        let point = e.call(0x006a_1440, &args![points, index]).u32();
        for i in 0..3 {
            let value = word(e, current + 8 + 4 * i);
            e.mem.set_u32(point + 4 * i, value);
        }
        if index as i32 > 0 {
            let link = e
                .call(0x0055_8dd0, &args![links, index.wrapping_sub(1)])
                .u32();
            for i in 0..4 {
                let value = word(e, current + 0x14 + 4 * i);
                e.mem.set_u32(link + 4 * i, value);
            }
        }
        current = word(e, current + NODE_PARENT);
        index = index.wrapping_sub(1);
    }
    true
}

// Translated from 006f3bd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f3bd0 - its C++ exception frame (FS:[0]) is not translated
/// `AStarSearch<ParentSpaceNode,TeleportLink>::AStarSearch` (Xbox PDB): the
/// vtable `0106d90c`, the node pool (`006f4320`), the hash (`006b3de0`)
/// and the open list (`006b9470`) constructed; best, current and last
/// added nodes null, not time-sliced. Answers `this`.
pub fn a_star_search_constructor(e: &mut Engine, this: Ptr) -> Ptr {
    let search = this.addr();
    e.mem.set_u32(search, A_STAR_SEARCH_VTABLE);
    fn_006f4320(e, Ptr::new(search + NODE_POOL));
    e.call(0x006b_3de0, &args![search + NODE_HASH]);
    e.call(0x006b_9470, &args![search + OPEN_LIST]);
    e.mem.set_u32(search + BEST_NODE, 0);
    e.mem.set_u32(search + CURRENT_NODE, 0);
    e.mem.set_u32(search + LAST_ADDED, 0);
    e.mem.set_u8(search + TIME_SLICED, 0);
    this
}

// Translated from 006f3c80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f3c80 - its C++ exception frame (FS:[0]) is not translated
/// The `AStarSearch` destructor (the engine map has no name): the vtable
/// `0106d90c`, the open list and the hash destroyed (`00483710` each),
/// then the node pool (`00982610`).
pub fn fn_006f3c80(e: &mut Engine, this: Ptr) {
    let search = this.addr();
    e.mem.set_u32(search, A_STAR_SEARCH_VTABLE);
    e.call(0x0048_3710, &args![search + OPEN_LIST]);
    e.call(0x0048_3710, &args![search + NODE_HASH]);
    e.call(0x0098_2610, &args![search + NODE_POOL]);
}

// Translated from 006f33b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f33b0 - its C++ exception frame (FS:[0]) is not translated
/// `TeleportDoorSearch::TeleportDoorSearch` (Xbox PDB): the `AStarSearch`
/// base, the class's vtable, the start `PathingLocation` (`006dcbb0`) and
/// the door array (`005e04c0`) constructed. Answers `this`.
pub fn teleport_door_search_constructor(e: &mut Engine, this: Ptr) -> Ptr {
    let search = this.addr();
    a_star_search_constructor(e, this);
    e.mem.set_u32(search, TELEPORT_DOOR_SEARCH_VTABLE);
    e.call(0x006d_cbb0, &args![search + START_LOCATION]);
    e.call(0x005e_04c0, &args![search + DOORS]);
    this
}

// Translated from 006f3460 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f3460 - its C++ exception frame (FS:[0]) is not translated
/// `TeleportDoorSearch::~TeleportDoorSearch` (Xbox PDB): the class's
/// vtable, the door array (`005e04f0`) and the start location
/// (`004ff7e0`) destroyed, then the `AStarSearch` destructor.
pub fn teleport_door_search_destructor(e: &mut Engine, this: Ptr) {
    let search = this.addr();
    e.mem.set_u32(search, TELEPORT_DOOR_SEARCH_VTABLE);
    e.call(0x005e_04f0, &args![search + DOORS]);
    e.call(0x004f_f7e0, &args![search + START_LOCATION]);
    fn_006f3c80(e, this);
}

// Translated from 006f3b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TeleportDoorSearch::IsGoalNode` (Xbox PDB; `RET 4`): in mode 3, any
/// worldspace node whose worldspace has map data (`00586400`) with both of
/// its first two words set; otherwise a node with the goal's worldspace
/// flag, cell and worldspace.
pub fn teleport_door_search_is_goal_node(e: &mut Engine, this: Ptr, node: u32) -> bool {
    let search = this.addr();
    if word(e, search + MODE) == 3 {
        let worldspace = word(e, node + 0xc);
        let map_data = if worldspace != 0 {
            e.call(0x0058_6400, &args![worldspace]).u32()
        } else {
            0
        };
        return e.mem.u8(node + 8) != 0
            && map_data != 0
            && word(e, map_data) != 0
            && word(e, map_data + 4) != 0;
    }
    e.mem.u8(node + 8) == e.mem.u8(search + GOAL_IS_WORLDSPACE)
        && word(e, node + 0x10) == word(e, search + GOAL_CELL)
        && word(e, node + 0xc) == word(e, search + GOAL_WORLDSPACE)
}

// Translated from 006f3d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AStarSearch<ParentSpaceNode,TeleportLink>::Search` (Xbox PDB;
/// `thiscall (points, links, partial)`, `RET 0xC`): the tasklet manager
/// (vtable +0xC with `0040fc90`'s answer) sets the time-sliced byte; the
/// search runs (`006f3fb0`). A goal node answers true; without one, with
/// `partial` the best node is used, answering false. The path to the node
/// used, if any, is written (`006f4230`).
pub fn a_star_search_search(
    e: &mut Engine,
    this: Ptr,
    points: Ptr,
    links: Ptr,
    partial: bool,
) -> bool {
    let search = this.addr();
    let manager = e.call(0x00b0_0a00, &args![]).u32();
    let thread = e.call(0x0040_fc90, &args![]).u32();
    let sliced = e.vcall(manager, 0xc, &args![thread]).u8();
    e.mem.set_u8(search + TIME_SLICED, sliced);
    let found = fn_006f3fb0(e, this);
    let (result, node) = if found != 0 {
        (true, found)
    } else if partial {
        (false, word(e, search + BEST_NODE))
    } else {
        (false, 0)
    };
    if node != 0 {
        fn_006f4230(e, this, node, points, links);
    }
    result
}

// Translated from 006f3fb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f3fb0 - its C++ exception frame (FS:[0], the neighbour array's unwind) is not translated
/// The search loop (the engine map has no name): nodes come off the open
/// list cheapest first; a goal node (vtable +8) becomes the best and is
/// answered. Otherwise its neighbours (vtable +4 into a local node array)
/// are taken off the open list; one that is the node's parent ends the
/// search (null); each other gets the node as parent, goes back into the
/// open list, and becomes the best when its value (`006b9130`, as a
/// `double`) is below the best's. When time-sliced, every 50th node
/// yields (`0040fca0` with 0). An empty open list restores the last node
/// as current and answers null.
pub fn fn_006f3fb0(e: &mut Engine, this: Ptr) -> u32 {
    let search = this.addr();
    e.with_stack(0x14, |e, neighbours| {
        fn_006f45c0(e, neighbours);
        let mut previous = word(e, search + CURRENT_NODE);
        let first = fn_006f46f0(e, Ptr::new(search + OPEN_LIST));
        e.mem.set_u32(search + CURRENT_NODE, first);
        if word(e, search + CURRENT_NODE) == 0 {
            fn_006f4690(e, neighbours);
            return 0;
        }
        let current = word(e, search + CURRENT_NODE);
        e.mem.set_u32(search + BEST_NODE, current);
        let mut steps = 0u32;
        while word(e, search + CURRENT_NODE) != 0 {
            let current = word(e, search + CURRENT_NODE);
            if e.vcall(search, 8, &args![current]).bool() {
                let current = word(e, search + CURRENT_NODE);
                e.mem.set_u32(search + BEST_NODE, current);
                fn_006f4690(e, neighbours);
                return current;
            }
            e.call(ARRAY_RELEASE_008454F0, &args![neighbours, 0u32]);
            let current = word(e, search + CURRENT_NODE);
            e.vcall(search, 4, &args![current, neighbours]);
            let mut index = 0u32;
            while index < e.call(0x0044_ddc0, &args![neighbours]).u32() {
                let slot = e.call(0x0087_7a30, &args![neighbours, index]).u32();
                let neighbour = word(e, slot);
                fn_006f4430(e, Ptr::new(search + OPEN_LIST), neighbour);
                let current = word(e, search + CURRENT_NODE);
                if word(e, current + NODE_PARENT) == neighbour {
                    fn_006f4690(e, neighbours);
                    return 0;
                }
                e.mem.set_u32(neighbour + NODE_PARENT, current);
                fn_006f4390(e, Ptr::new(search + OPEN_LIST), neighbour);
                let value = e.call(0x006b_9130, &args![neighbour]).f64();
                let best = word(e, search + BEST_NODE);
                let best_value = e.call(0x006b_9130, &args![best]).f64();
                if best_value > value {
                    e.mem.set_u32(search + BEST_NODE, neighbour);
                }
                index = index.wrapping_add(1);
            }
            if e.mem.u8(search + TIME_SLICED) != 0 {
                steps = steps.wrapping_add(1);
                if steps == 0x32 {
                    steps = 0;
                    e.call(0x0040_fca0, &args![0u32]);
                }
            }
            previous = word(e, search + CURRENT_NODE);
            let next = fn_006f46f0(e, Ptr::new(search + OPEN_LIST));
            e.mem.set_u32(search + CURRENT_NODE, next);
        }
        e.mem.set_u32(search + CURRENT_NODE, previous);
        fn_006f4690(e, neighbours);
        0
    })
}

/// `PathingLocation::GetWorldspace` (`00441110`) and `GetCell`
/// (`006dd4f0`), both `thiscall`.
const LOCATION_WORLDSPACE: u32 = 0x0044_1110;
const LOCATION_CELL: u32 = 0x006d_d4f0;

// Translated from 006f34e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TeleportDoorSearch::BuildNodePath` (Xbox PDB; `thiscall (start, goal,
/// path, mode, cost limit, actor, lock mode)`, `RET 0x1C`): the start
/// location is kept; its worldspace (else its cell; else false) becomes
/// the start node's key and space data. Mode 2 with a worldspace start
/// answers false. The cost limit, lock mode and (unless in mode 3) the
/// goal's worldspace or cell are kept, then the actor. The path's two
/// points take the start and goal positions (`005c3420`), the start node
/// is added (estimate 1.0) and the search run (`006f3d90`, partial
/// allowed); its answer is answered.
#[allow(clippy::too_many_arguments)]
pub fn teleport_door_search_build_node_path(
    e: &mut Engine,
    this: Ptr,
    start: Ptr,
    goal: Ptr,
    path: Ptr,
    mode: u32,
    cost_limit: f32,
    actor: u32,
    lock_mode: u32,
) -> bool {
    let search = this.addr();
    e.call(0x006d_cce0, &args![search + START_LOCATION, start]);
    let key = if e.call(LOCATION_WORLDSPACE, &args![start]).u32() != 0 {
        let worldspace = e.call(LOCATION_WORLDSPACE, &args![start]).u32();
        e.mem.set_u32(search + START_WORLDSPACE, worldspace);
        e.mem.set_u8(search + START_DATA, 1);
        e.mem.set_u32(search + START_CELL, 0);
        word(e, search + START_WORLDSPACE)
    } else if e.call(LOCATION_CELL, &args![start]).u32() != 0 {
        let cell = e.call(LOCATION_CELL, &args![start]).u32();
        e.mem.set_u32(search + START_CELL, cell);
        e.mem.set_u8(search + START_DATA, 0);
        e.mem.set_u32(search + START_WORLDSPACE, 0);
        word(e, search + START_CELL)
    } else {
        return false;
    };
    e.mem.set_u32(search + MODE, mode);
    if word(e, search + START_WORLDSPACE) != 0 && mode == 2 {
        return false;
    }
    e.mem
        .set_u32(search + COST_LIMIT, x87_quiet_bits(cost_limit.to_bits()));
    e.mem.set_u32(search + LOCK_MODE, lock_mode);
    if word(e, search + MODE) != 3 {
        if e.call(LOCATION_WORLDSPACE, &args![goal]).u32() != 0 {
            let worldspace = e.call(LOCATION_WORLDSPACE, &args![goal]).u32();
            e.mem.set_u32(search + GOAL_WORLDSPACE, worldspace);
            e.mem.set_u8(search + GOAL_IS_WORLDSPACE, 1);
            e.mem.set_u32(search + GOAL_CELL, 0);
        } else if e.call(LOCATION_CELL, &args![goal]).u32() != 0 {
            let cell = e.call(LOCATION_CELL, &args![goal]).u32();
            e.mem.set_u32(search + GOAL_CELL, cell);
            e.mem.set_u8(search + GOAL_IS_WORLDSPACE, 0);
            e.mem.set_u32(search + GOAL_WORLDSPACE, 0);
        }
    }
    e.mem.set_u32(search + ACTOR, actor);
    e.with_stack(0x18, |e, frame| {
        let start_point = e.call(0x005c_3420, &args![start, frame.addr()]).u32();
        for i in 0..3 {
            let value = word(e, start_point + 4 * i);
            e.mem.set_u32(path.addr() + 0x20 + 4 * i, value);
        }
        let goal_point = e.call(0x005c_3420, &args![goal, frame.addr() + 0xc]).u32();
        for i in 0..3 {
            let value = word(e, goal_point + 4 * i);
            e.mem.set_u32(path.addr() + 0x2c + 4 * i, value);
        }
    });
    fn_006f3d00(e, this, key, Ptr::new(search + START_DATA), 1.0);
    a_star_search_search(e, this, path, Ptr::new(path.addr() + 0x10), true)
}

/// `409600.0` (`float`, `0106c238`): the extra cost of a locked or
/// minimal-use door.
const DOOR_PENALTY: u32 = 0x0106_c238;
/// `0.0` (`double`, `01012060`).
const ZERO: u32 = 0x0101_2060;
/// The door's teleport data (`00568e50`, `thiscall`) and its destination
/// worldspace (`0043a320`) and cell (`0043a2b0`).
const TELEPORT_DATA: u32 = 0x0056_8e50;
const TELEPORT_WORLDSPACE: u32 = 0x0043_a320;
const TELEPORT_CELL: u32 = 0x0043_a2b0;
/// `DoorLock`'s constructor (`00501d20`, `thiscall (lock, door)`),
/// `CanActorUnlockDoorLock` (`00502450`, `thiscall (lock, actor, door,
/// &flag)`) and destructor (`00501e50`).
const DOOR_LOCK_CONSTRUCTOR: u32 = 0x0050_1d20;
const DOOR_LOCK_CAN_UNLOCK: u32 = 0x0050_2450;
const DOOR_LOCK_DESTRUCTOR: u32 = 0x0050_1e50;

fn teleport_worldspace(e: &mut Engine, door: u32) -> u32 {
    let data = e.call(TELEPORT_DATA, &args![door]).u32();
    e.call(TELEPORT_WORLDSPACE, &args![data]).u32()
}

fn teleport_cell(e: &mut Engine, door: u32) -> u32 {
    let data = e.call(TELEPORT_DATA, &args![door]).u32();
    e.call(TELEPORT_CELL, &args![data]).u32()
}

// Translated from 006f36d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f36d0 - its C++ exception frame (FS:[0], the DoorLock local's unwind) is not translated
/// `TeleportDoorSearch::GetNodeConnections` (Xbox PDB; `thiscall (node,
/// neighbours)`, `RET 8`): the doors of the node's worldspace
/// (`00588270`) or cell (`0054db50`) are gathered; with lock mode 1 or 3
/// or an actor, each door is considered (skipping those `00440da0`
/// rejects): its destination (worldspace, else cell); in mode 1 a
/// worldspace destination other than the start's is skipped, in mode 2
/// any worldspace destination. A door the actor cannot unlock (or only
/// with a key) is skipped in lock mode 0 and costs the penalty `0106c238`
/// in lock mode 2; a minimal-use door costs it too unless in lock mode 3.
/// The step cost is the distance (`00457990` of `00439ef0`) from the
/// node's position, or from the start location's position for a node
/// without one, to the door's position (vtable +0x1F4); with a nonzero
/// cost limit a door beyond it is skipped. The destination's node is found
/// or made (`006f3e30`): a new one gets the destination's space data and
/// estimate 1.0; a new or cheaper one gets the door and its position
/// (`00717e50`) and is added to `neighbours` (`007cb2e0`). True.
pub fn teleport_door_search_get_node_connections(
    e: &mut Engine,
    this: Ptr,
    node: u32,
    neighbours: Ptr,
) -> bool {
    let search = this.addr();
    let doors = search + DOORS;
    e.call(ARRAY_RELEASE_008454F0, &args![doors, 0u32]);
    if e.mem.u8(node + 8) != 0 {
        let worldspace = word(e, node + 0xc);
        e.call(0x0058_8270, &args![worldspace, doors]);
    } else {
        let cell = word(e, node + 0x10);
        e.call(0x0054_db50, &args![cell, doors]);
    }
    if !(word(e, search + LOCK_MODE) == 1
        || word(e, search + LOCK_MODE) == 3
        || word(e, search + ACTOR) != 0)
    {
        return true;
    }
    let zero = f64::from_bits(e.global::<u64>(ZERO));
    // The exe's locals from `EBP - 0x80`: offsets below are `0x80 - k` for
    // the local at `EBP - k`.
    e.with_stack(0x80, |e, frame| {
        let frame = frame.addr();
        let local = |k: u32| frame + 0x80 - k;
        let mut index = 0u32;
        while index < e.call(0x0044_ddc0, &args![doors]).u32() {
            let slot = e.call(0x0087_7a30, &args![doors, index]).u32();
            let door = word(e, slot);
            index = index.wrapping_add(1);
            if e.call(0x0044_0da0, &args![door]).bool() {
                continue;
            }
            let destination = if teleport_worldspace(e, door) != 0 {
                teleport_worldspace(e, door)
            } else {
                teleport_cell(e, door)
            };
            match word(e, search + MODE) {
                1 => {
                    if teleport_worldspace(e, door) != 0
                        && teleport_worldspace(e, door) != word(e, search + START_WORLDSPACE)
                    {
                        continue;
                    }
                }
                2 => {
                    if teleport_worldspace(e, door) != 0 {
                        continue;
                    }
                }
                _ => {}
            }
            let lock = local(0x50);
            let key_only = local(0x21);
            e.call(DOOR_LOCK_CONSTRUCTOR, &args![lock, door]);
            let actor = word(e, search + ACTOR);
            let unlockable = e
                .call(DOOR_LOCK_CAN_UNLOCK, &args![lock, actor, door, key_only])
                .bool();
            let mut penalty = 0.0f32.to_bits();
            if !unlockable || e.mem.u8(key_only) != 0 {
                match word(e, search + LOCK_MODE) {
                    0 => {
                        e.call(DOOR_LOCK_DESTRUCTOR, &args![lock]);
                        continue;
                    }
                    2 => penalty = x87_quiet_bits(e.global::<u32>(DOOR_PENALTY)),
                    _ => {}
                }
            }
            let base = e.call(0x007a_f430, &args![door]).u32();
            if e.call(0x0051_8000, &args![base]).bool() && word(e, search + LOCK_MODE) != 3 {
                penalty = x87_quiet_bits(e.global::<u32>(DOOR_PENALTY));
            }
            let position = e.vcall(door, 0x1f4, &args![]).u32();
            let door_position = local(0x30);
            for i in 0..3 {
                let value = word(e, position + 4 * i);
                e.mem.set_u32(door_position + 4 * i, value);
            }
            let difference = if e.call(0x0059_bb30, &args![node]).u32() != 0 {
                e.call(0x0043_9ef0, &args![node + 0x18, local(0x64), door_position])
                    .u32()
            } else {
                let start = e
                    .call(0x005c_3420, &args![search + START_LOCATION, local(0x70)])
                    .u32();
                e.call(0x0043_9ef0, &args![start, local(0x7c), door_position])
                    .u32()
            };
            let distance = x87_store32(e.call(0x0045_7990, &args![difference]).f64());
            if x87_load32(word(e, search + COST_LIMIT)) != zero {
                let reached = x87_add(x87_load32(word(e, node)), x87_m32(distance));
                if x87_load32(word(e, search + COST_LIMIT)) < reached {
                    e.call(DOOR_LOCK_DESTRUCTOR, &args![lock]);
                    continue;
                }
            }
            let total = x87_store32(x87_add(
                x87_add(x87_load32(word(e, node)), x87_m32(distance)),
                x87_m32(penalty),
            ));
            teleport_cell(e, door);
            let found = local(0x1c);
            let result = fn_006f3e30(e, this, destination, f32::from_bits(total), Ptr::new(found));
            if result == 0 {
                let created = word(e, found);
                if teleport_worldspace(e, door) != 0 {
                    e.mem.set_u8(created + 8, 1);
                    e.mem.set_u32(created + 0x10, 0);
                    let worldspace = teleport_worldspace(e, door);
                    e.mem.set_u32(word(e, found) + 0xc, worldspace);
                } else {
                    e.mem.set_u8(created + 8, 0);
                    e.mem.set_u32(created + 0xc, 0);
                    let cell = teleport_cell(e, door);
                    e.mem.set_u32(word(e, found) + 0x10, cell);
                }
                e.mem.set_f32(word(e, found) + 4, 1.0);
            }
            if result <= 1 {
                e.mem.set_u32(word(e, found) + 0x14, door);
                let data = e.call(TELEPORT_DATA, &args![door]).u32();
                let position = e.call(0x0071_7e50, &args![data]).u32();
                let target = word(e, found) + 0x18;
                for i in 0..3 {
                    let value = word(e, position + 4 * i);
                    e.mem.set_u32(target + 4 * i, value);
                }
                e.call(0x007c_b2e0, &args![neighbours, found]);
            }
            e.call(DOOR_LOCK_DESTRUCTOR, &args![lock]);
        }
    });
    true
}

/// `00401030` (cdecl: block): frees a block.
const FREE_BLOCK: u32 = 0x0040_1030;

// Translated from 006f3430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TeleportDoorSearch::scalar deleting destructor` (engine map;
/// `thiscall`; flags; `RET 4`): [`teleport_door_search_destructor`], then
/// the block is freed (`00401030`) when bit 0 of `flags` is set. Returns
/// `this`.
pub fn teleport_door_search_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    teleport_door_search_destructor(e, this);
    if flags & 1 != 0 {
        e.call(FREE_BLOCK, &args![this]);
    }
    this
}

// Translated from 006f3f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AStarSearch<ParentSpaceNode,TeleportLink>::scalar deleting destructor`
/// (engine map; `thiscall`; flags; `RET 4`): [`fn_006f3c80`], then the
/// block is freed when bit 0 of `flags` is set. Returns `this`.
pub fn a_star_search_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006f3c80(e, this);
    if flags & 1 != 0 {
        e.call(FREE_BLOCK, &args![this]);
    }
    this
}

// Translated from 006f4660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<AStarNode<ParentSpaceNode,TeleportLink>*,1024>::scalar
/// deleting destructor` (engine map; `thiscall`; flags; `RET 4`):
/// [`fn_006f4640`], then the block is freed when bit 0 of `flags` is set.
/// Returns `this`.
pub fn bs_simple_array_node_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_006f4640(e, this);
    if flags & 1 != 0 {
        e.call(FREE_BLOCK, &args![this]);
    }
    this
}

// Translated from 006f4850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapArray<AStarNode<ParentSpaceNode,TeleportLink>*,1024>::scalar
/// deleting destructor` (engine map; `thiscall`; flags; `RET 4`):
/// [`fn_006f4690`], then the block is freed when bit 0 of `flags` is set.
/// Returns `this`.
pub fn bs_scrap_array_node_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_006f4690(e, this);
    if flags & 1 != 0 {
        e.call(FREE_BLOCK, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006f_4640, fn_006f4640(Ptr)),
        entry!(0x006f_4320, fn_006f4320(Ptr) -> Ptr),
        entry!(0x006f_4880, fn_006f4880(Ptr) -> Ptr),
        entry!(0x006f_47f0, fn_006f47f0(Ptr) -> Ptr),
        entry!(0x006f_4340, fn_006f4340(Ptr) -> u32),
        entry!(0x006f_4430, fn_006f4430(Ptr, u32)),
        entry!(0x006f_4790, fn_006f4790(Ptr, u32) -> u32),
        entry!(0x006f_4390, fn_006f4390(Ptr, u32)),
        entry!(0x006f_46f0, fn_006f46f0(Ptr) -> u32),
        entry!(0x006f_45c0, fn_006f45c0(Ptr) -> Ptr),
        entry!(0x006f_4690, fn_006f4690(Ptr)),
        entry!(0x006f_4480, fn_006f4480(Ptr, u32, bool)),
        entry!(0x006f_3e30, fn_006f3e30(Ptr, u32, f32, Ptr) -> u32),
        entry!(0x006f_3d00, fn_006f3d00(Ptr, u32, Ptr, f32) -> u32),
        entry!(0x006f_4230, fn_006f4230(Ptr, u32, Ptr, Ptr) -> bool),
        entry!(0x006f_3bd0, a_star_search_constructor(Ptr) -> Ptr),
        entry!(0x006f_3c80, fn_006f3c80(Ptr)),
        entry!(0x006f_33b0, teleport_door_search_constructor(Ptr) -> Ptr),
        entry!(0x006f_3460, teleport_door_search_destructor(Ptr)),
        entry!(0x006f_3b00, teleport_door_search_is_goal_node(Ptr, u32) -> bool),
        entry!(0x006f_3d90, a_star_search_search(Ptr, Ptr, Ptr, bool) -> bool),
        entry!(0x006f_3fb0, fn_006f3fb0(Ptr) -> u32),
        entry!(0x006f_34e0, teleport_door_search_build_node_path(Ptr, Ptr, Ptr, Ptr, u32, f32, u32, u32) -> bool),
        entry!(0x006f_36d0, teleport_door_search_get_node_connections(Ptr, u32, Ptr) -> bool),
        entry!(
            0x006f_3430,
            teleport_door_search_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x006f_3f80,
            a_star_search_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x006f_4660,
            bs_simple_array_node_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x006f_4850,
            bs_scrap_array_node_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
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
    fn fn_006f4640_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x006f_4640, ARRAY_VTABLE_0106D930, ARRAY_RELEASE_008454F0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tds_calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|call| call.0 == addr)
            .map(|call| call.1.clone())
            .collect()
    }

    fn tds_engine(doubles: &[u32]) -> Engine {
        let mut e = Engine::new();
        for addr in doubles {
            e.register(*addr, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    /// A search node (0x38 bytes) with cost `cost`.
    fn tds_node(e: &mut Engine, cost: f32) -> u32 {
        let node = e.mem.alloc(0x38);
        e.mem.set_f32(node, cost);
        node
    }

    #[test]
    fn node_pool_constructor_passes_0x7000() {
        let mut e = tds_engine(&[0x00aa_5410]);
        let this = e.mem.alloc(0x10);
        assert_eq!(e.call(0x006f_4320, &args![this]).u32(), this);
        assert_eq!(tds_calls(&e, 0x00aa_5410), vec![vec![this, 0x7000]]);
    }

    #[test]
    fn array_base_constructor_sets_the_vtable_and_sets_up() {
        let mut e = tds_engine(&[0x006b_3eb0]);
        let this = e.mem.alloc(0x14);
        assert_eq!(e.call(0x006f_4880, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), ARRAY_VTABLE_0106D930);
        assert_eq!(tds_calls(&e, 0x006b_3eb0), vec![vec![this, 0, 0]]);
    }

    #[test]
    fn node_constructor_clears_the_node() {
        let mut e = tds_engine(&[0x006d_5320]);
        let node = e.mem.alloc(0x38);
        for i in 0..14 {
            e.mem.set_u32(node + 4 * i, 0xffff_ffff);
        }
        assert_eq!(e.call(0x006f_47f0, &args![node]).u32(), node);
        assert_eq!(e.mem.f32(node), 0.0);
        assert_eq!(e.mem.f32(node + 4), 0.0);
        for offset in [0x24, 0x28, 0x2c, 0x30, 0x34] {
            assert_eq!(e.mem.u32(node + offset), 0);
        }
        assert_eq!(tds_calls(&e, 0x006d_5320), vec![vec![node + 0x14]]);
    }

    #[test]
    fn pool_node_is_built_or_null() {
        let mut e = tds_engine(&[0x006d_5320]);
        e.register(0x006a_1f90, |e, a| e.mem.alloc(a[0]).into_ret());
        let pool = e.mem.alloc(0x10);
        let node = e.call(0x006f_4340, &args![pool]).u32();
        assert_ne!(node, 0);
        assert_eq!(tds_calls(&e, 0x006a_1f90), vec![vec![0x38, pool]]);
        assert_eq!(tds_calls(&e, 0x006d_5320), vec![vec![node + 0x14]]);
        e.register(0x006a_1f90, |_, _| Ret::default());
        assert_eq!(e.call(0x006f_4340, &args![pool]).u32(), 0);
    }

    /// An open list (0x54 bytes: 20 buckets at +4) with the bucket limit.
    fn tds_open_list(e: &mut Engine, limit: f32) -> u32 {
        let list = e.mem.alloc(0x58);
        e.mem.set_f32(list, limit);
        list
    }

    #[test]
    fn unlinking_takes_a_node_out_of_its_bucket() {
        let mut e = tds_engine(&[]);
        let list = tds_open_list(&mut e, 1.0);
        let (first, second) = (tds_node(&mut e, 0.0), tds_node(&mut e, 0.0));
        // Bucket 0: first, second.
        e.mem.set_u32(list + 4, first);
        e.mem.set_u32(first + 0x28, list + 4);
        e.mem.set_u32(first + 0x2c, second);
        e.mem.set_u32(second + 0x28, first + 0x2c);
        e.call(0x006f_4430, &args![list, first]);
        assert_eq!(e.mem.u32(list + 4), second);
        assert_eq!(e.mem.u32(second + 0x28), list + 4);
        assert_eq!((e.mem.u32(first + 0x28), e.mem.u32(first + 0x2c)), (0, 0));
        e.call(0x006f_4430, &args![list, second]);
        assert_eq!(e.mem.u32(list + 4), 0);
    }

    #[test]
    fn bucket_index_scales_the_estimate_or_answers_the_last_bucket() {
        let mut e = tds_engine(&[]);
        e.map(BUCKET_SCALE, 8);
        e.set_global(BUCKET_SCALE, 19.0f64);
        e.register(0x006b_9710, |e, a| {
            f64::from(e.mem.f32(a[0] + 4)).into_ret()
        });
        let list = tds_open_list(&mut e, 100.0);
        let node = tds_node(&mut e, 0.0);
        e.mem.set_f32(node + 4, 50.0);
        // 50 * 19 / 100 = 9.5, truncated.
        assert_eq!(e.call(0x006f_4790, &args![list, node]).u32(), 9);
        e.mem.set_f32(node + 4, 100.0);
        assert_eq!(e.call(0x006f_4790, &args![list, node]).u32(), 0x13);
        e.mem.set_f32(node + 4, 150.0);
        assert_eq!(e.call(0x006f_4790, &args![list, node]).u32(), 0x13);
    }

    #[test]
    fn insertion_keeps_each_bucket_sorted_and_popping_takes_the_front() {
        let mut e = tds_engine(&[]);
        e.map(BUCKET_SCALE, 8);
        e.set_global(BUCKET_SCALE, 19.0f64);
        e.register(0x006b_9710, |e, a| {
            f64::from(e.mem.f32(a[0] + 4)).into_ret()
        });
        let list = tds_open_list(&mut e, 100.0);
        let nodes: Vec<u32> = [3.0f32, 1.0, 2.0, 90.0]
            .iter()
            .map(|estimate| {
                let node = tds_node(&mut e, 0.0);
                e.mem.set_f32(node + 4, *estimate);
                node
            })
            .collect();
        for node in &nodes {
            e.call(0x006f_4390, &args![list, *node]);
        }
        // Bucket 0 holds 1, 2, 3 in order; 90 is in bucket 17.
        assert_eq!(e.mem.u32(list + 4), nodes[1]);
        assert_eq!(e.mem.u32(nodes[1] + 0x2c), nodes[2]);
        assert_eq!(e.mem.u32(nodes[2] + 0x2c), nodes[0]);
        assert_eq!(e.mem.u32(nodes[2] + 0x28), nodes[1] + 0x2c);
        assert_eq!(e.mem.u32(list + 4 + 4 * 17), nodes[3]);
        for expected in [nodes[1], nodes[2], nodes[0], nodes[3], 0] {
            assert_eq!(e.call(0x006f_46f0, &args![list]).u32(), expected);
        }
    }

    #[test]
    fn node_array_constructor_takes_the_scrap_heap() {
        let mut e = tds_engine(&[0x006b_3eb0]);
        e.register(0x0040_1020, |_, _| 0x011f_6238u32.into_ret());
        e.register(0x00aa_42e0, |_, a| (a[0] + 1).into_ret());
        let this = e.mem.alloc(0x14);
        assert_eq!(e.call(0x006f_45c0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), NODE_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(this + 0x10), 0x011f_6239);
        assert_eq!(tds_calls(&e, 0x006b_3eb0).len(), 2);
    }

    #[test]
    fn node_array_destructor_releases_then_runs_the_base() {
        let mut e = tds_engine(&[ARRAY_RELEASE_008454F0]);
        let this = e.mem.alloc(0x14);
        e.call(0x006f_4690, &args![this]);
        assert_eq!(e.mem.u32(this), ARRAY_VTABLE_0106D930);
        assert_eq!(
            tds_calls(&e, ARRAY_RELEASE_008454F0),
            vec![vec![this, 1], vec![this, 1]]
        );
    }

    #[test]
    fn set_size_allocates_grows_shrinks_and_releases() {
        const ALLOCATE: u32 = 0x0300_0000;
        let mut e = tds_engine(&[
            ARRAY_RELEASE_008454F0,
            0x009f_6540,
            0x006d_bcf0,
            0x0072_ba80,
        ]);
        e.register(ALLOCATE, |_, a| (0x8000 + a[1]).into_ret());
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, ALLOCATE);
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array, vtable);
        // Empty: allocate 4, build all.
        e.call(0x006f_4480, &args![array, 4u32, 1u32]);
        assert_eq!(
            (
                e.mem.u32(array + 4),
                e.mem.u32(array + 8),
                e.mem.u32(array + 0xc)
            ),
            (0x8004, 4, 4)
        );
        assert_eq!(tds_calls(&e, 0x006d_bcf0), vec![vec![array, 0x8004, 4]]);
        // Beyond the capacity: reallocate, build the new tail.
        e.call(0x006f_4480, &args![array, 6u32, 1u32]);
        assert_eq!(tds_calls(&e, 0x009f_6540), vec![vec![array, 6, 4]]);
        assert_eq!(tds_calls(&e, 0x006d_bcf0)[1], vec![array, 0x8004 + 0x40, 2]);
        // Smaller with shrink at a quarter: destroy the tail, reallocate.
        e.call(0x006f_4480, &args![array, 1u32, 1u32]);
        assert_eq!(
            tds_calls(&e, 0x0072_ba80),
            vec![vec![array, 0x8004 + 0x10, 5]]
        );
        assert_eq!(tds_calls(&e, 0x009f_6540)[1], vec![array, 1, 1]);
        assert_eq!(e.mem.u32(array + 0xc), 1);
        // Zero releases.
        e.call(0x006f_4480, &args![array, 0u32, 0u32]);
        assert_eq!(tds_calls(&e, ARRAY_RELEASE_008454F0), vec![vec![array, 0]]);
    }

    /// A search object large enough for every member, with the hash slot
    /// answer `0` and a pool that allocates.
    fn tds_search(e: &mut Engine) -> u32 {
        e.register(0x0099_6840, |_, _| Ret::default());
        e.register(0x006a_1f90, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(0x006d_5320, |_, _| Ret::default());
        e.mem.alloc(0x20b8)
    }

    #[test]
    fn find_or_make_node_creates_updates_or_keeps() {
        let mut e = tds_engine(&[]);
        let search = tds_search(&mut e);
        let out = e.mem.alloc(4);
        assert_eq!(
            e.call(0x006f_3e30, &args![search, 7u32, 5.0f32, out]).u32(),
            0
        );
        let node = e.mem.u32(out);
        assert_eq!(e.mem.u32(search + 0x14), node);
        assert_eq!((e.mem.u32(node + 0x30), e.mem.f32(node)), (7, 5.0));
        // A second key goes in front; the first is found behind it.
        e.call(0x006f_3e30, &args![search, 8u32, 1.0f32, out]);
        assert_eq!(e.mem.u32(e.mem.u32(search + 0x14) + 0x34), node);
        assert_eq!(
            e.call(0x006f_3e30, &args![search, 7u32, 6.0f32, out]).u32(),
            2
        );
        assert_eq!(
            e.call(0x006f_3e30, &args![search, 7u32, 4.0f32, out]).u32(),
            1
        );
        assert_eq!(e.mem.u32(out), node);
        assert_eq!(e.mem.f32(node), 4.0);
    }

    #[test]
    fn start_node_is_added_once() {
        let mut e = tds_engine(&[]);
        let search = tds_search(&mut e);
        e.map(BUCKET_SCALE, 8);
        e.register(0x006b_9710, |_, _| 0.0f64.into_ret());
        let data = e.mem.alloc(0xc);
        for i in 0..3 {
            e.mem.set_u32(data + 4 * i, 0x100 + i);
        }
        let node = e
            .call(0x006f_3d00, &args![search, 9u32, data, 2.0f32])
            .u32();
        assert_ne!(node, 0);
        assert_eq!(e.mem.f32(node + 4), 2.0);
        assert_eq!(e.mem.u32(node + 0xc), 0x101);
        assert_eq!(e.mem.u32(search + 0x2050), node);
        // Bucket 0x13 (a limit of 0 is not above the estimate).
        assert_eq!(e.mem.u32(search + 0x1ff4 + 4 + 4 * 0x13), node);
        // The same key at cost 0: not cheaper, null.
        assert_eq!(
            e.call(0x006f_3d00, &args![search, 9u32, data, 2.0f32])
                .u32(),
            0
        );
    }

    #[test]
    fn path_is_written_from_the_end_node_back() {
        let mut e = tds_engine(&[0x006a_8890]);
        let points = e.mem.alloc(0x30);
        let links = e.mem.alloc(0x40);
        e.register(0x006a_1440, |e, a| {
            (e.mem.u32(a[0]) + 0xc * a[1]).into_ret()
        });
        e.register(0x0055_8dd0, |e, a| {
            (e.mem.u32(a[0] + 4) + 0x10 * a[1]).into_ret()
        });
        e.register(0x006d_bcf0, |_, _| Ret::default());
        e.register(0x0300_0000, |e, a| e.mem.alloc(0x10 * a[1]).into_ret());
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, 0x0300_0000);
        e.mem.set_u32(links, vtable);
        let point_data = e.mem.alloc(0x24);
        e.mem.set_u32(points, point_data);
        let (start, end) = (tds_node(&mut e, 0.0), tds_node(&mut e, 0.0));
        e.mem.set_u32(end + 0x24, start);
        for i in 0..3 {
            e.mem.set_u32(start + 8 + 4 * i, 0x10 + i);
            e.mem.set_u32(end + 8 + 4 * i, 0x20 + i);
        }
        for i in 0..4 {
            e.mem.set_u32(end + 0x14 + 4 * i, 0x30 + i);
        }
        assert!(e.call(0x006f_4230, &args![0u32, end, points, links]).bool());
        assert_eq!(tds_calls(&e, 0x006a_8890), vec![vec![points, 2, 1]]);
        assert_eq!(e.mem.u32(point_data), 0x10);
        assert_eq!(e.mem.u32(point_data + 0xc), 0x20);
        let link_data = e.mem.u32(links + 4);
        assert_eq!(e.mem.u32(link_data + 0xc), 0x33);
        assert!(!e
            .call(0x006f_4230, &args![0u32, 0u32, points, links])
            .bool());
    }

    #[test]
    fn a_star_constructor_and_destructor_handle_their_members() {
        let mut e = tds_engine(&[
            0x00aa_5410,
            0x006b_3de0,
            0x006b_9470,
            0x0048_3710,
            0x0098_2610,
        ]);
        let this = e.mem.alloc(0x2058);
        e.mem.set_u8(this + 0x2054, 1);
        assert_eq!(e.call(0x006f_3bd0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), A_STAR_SEARCH_VTABLE);
        assert_eq!(e.mem.u8(this + 0x2054), 0);
        assert_eq!(tds_calls(&e, 0x006b_9470), vec![vec![this + 0x1ff4]]);
        e.call(0x006f_3c80, &args![this]);
        assert_eq!(
            tds_calls(&e, 0x0048_3710),
            vec![vec![this + 0x1ff4], vec![this + 0x14]]
        );
        assert_eq!(tds_calls(&e, 0x0098_2610), vec![vec![this + 4]]);
    }

    #[test]
    fn teleport_door_search_constructor_and_destructor() {
        let mut e = tds_engine(&[
            0x00aa_5410,
            0x006b_3de0,
            0x006b_9470,
            0x0048_3710,
            0x0098_2610,
            0x006d_cbb0,
            0x005e_04c0,
            0x005e_04f0,
            0x004f_f7e0,
        ]);
        let this = e.mem.alloc(0x20b8);
        assert_eq!(e.call(0x006f_33b0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), TELEPORT_DOOR_SEARCH_VTABLE);
        assert_eq!(tds_calls(&e, 0x006d_cbb0), vec![vec![this + 0x2064]]);
        e.call(0x006f_3460, &args![this]);
        assert_eq!(tds_calls(&e, 0x005e_04f0), vec![vec![this + 0x20a4]]);
        assert_eq!(e.mem.u32(this), A_STAR_SEARCH_VTABLE);
    }

    #[test]
    fn goal_test_by_mode() {
        let mut e = tds_engine(&[]);
        let this = e.mem.alloc(0x20b8);
        let node = tds_node(&mut e, 0.0);
        e.mem.set_u8(node + 8, 1);
        e.mem.set_u32(node + 0xc, 0x4000);
        e.mem.set_u8(this + 0x208c, 1);
        e.mem.set_u32(this + 0x2090, 0x4000);
        assert!(e.call(0x006f_3b00, &args![this, node]).bool());
        e.mem.set_u32(node + 0x10, 1);
        assert!(!e.call(0x006f_3b00, &args![this, node]).bool());
        // Mode 3: worldspace map data with both words set.
        e.mem.set_u32(this + 0x2098, 3);
        let map_data = e.mem.alloc(8);
        e.register_double(0x0058_6400, move |_, _| map_data.into_ret());
        assert!(!e.call(0x006f_3b00, &args![this, node]).bool());
        e.mem.set_u32(map_data, 1);
        e.mem.set_u32(map_data + 4, 1);
        assert!(e.call(0x006f_3b00, &args![this, node]).bool());
    }

    /// A search whose vtable has the goal test (+8: true for the node in
    /// the control block's first word) and the neighbour gathering (+4:
    /// the control block's list, +4, of its count, +8, nodes).
    fn tds_loop_engine() -> (Engine, u32, u32) {
        let mut e = tds_engine(&[0x006b_3eb0, 0x0040_fca0, 0x006f_4640]);
        e.register(0x0040_1020, |_, _| Ret::default());
        e.register(0x00aa_42e0, |_, _| Ret::default());
        e.register(ARRAY_RELEASE_008454F0, |_, _| Ret::default());
        e.map(BUCKET_SCALE, 8);
        e.register(0x006b_9710, |_, _| 0.0f64.into_ret());
        e.register(0x006b_9130, |e, a| f64::from(e.mem.f32(a[0])).into_ret());
        let control = e.mem.alloc(0xc);
        e.register_double(0x0300_0000, move |e, a| {
            u32::from(a[1] == e.mem.u32(control)).into_ret()
        });
        e.register_double(0x0300_0010, move |e, a| {
            let list = e.mem.u32(control + 4);
            let count = e.mem.u32(control + 8);
            e.mem.set_u32(a[2] + 4, list);
            e.mem.set_u32(a[2] + 8, count);
            Ret::default()
        });
        e.register(0x0044_ddc0, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(0x0087_7a30, |e, a| {
            (e.mem.u32(a[0] + 4) + 4 * a[1]).into_ret()
        });
        let vtable = e.mem.alloc(0xc);
        e.mem.set_u32(vtable + 4, 0x0300_0010);
        e.mem.set_u32(vtable + 8, 0x0300_0000);
        let search = e.mem.alloc(0x20b8);
        e.mem.set_u32(search, vtable);
        (e, search, control)
    }

    #[test]
    fn search_loop_reaches_the_goal_through_a_neighbour() {
        let (mut e, search, control) = tds_loop_engine();
        let start = tds_node(&mut e, 2.0);
        let goal = tds_node(&mut e, 3.0);
        let list = e.mem.alloc(4);
        e.mem.set_u32(list, goal);
        e.mem.set_u32(control, goal);
        e.mem.set_u32(control + 4, list);
        e.mem.set_u32(control + 8, 1);
        e.mem.set_u32(search + 0x1ff4 + 4, start);
        e.mem.set_u32(start + 0x28, search + 0x1ff4 + 4);
        assert_eq!(e.call(0x006f_3fb0, &args![search]).u32(), goal);
        assert_eq!(e.mem.u32(goal + 0x24), start);
        assert_eq!(e.mem.u32(search + 0x2048), goal);
    }

    #[test]
    fn search_loop_stops_on_a_parent_or_an_empty_list() {
        let (mut e, search, control) = tds_loop_engine();
        assert_eq!(e.call(0x006f_3fb0, &args![search]).u32(), 0);
        let start = tds_node(&mut e, 2.0);
        let parent = tds_node(&mut e, 1.0);
        e.mem.set_u32(start + 0x24, parent);
        let list = e.mem.alloc(4);
        e.mem.set_u32(list, parent);
        e.mem.set_u32(control + 4, list);
        e.mem.set_u32(control + 8, 1);
        e.mem.set_u32(search + 0x1ff4 + 4, start);
        assert_eq!(e.call(0x006f_3fb0, &args![search]).u32(), 0);
        assert_eq!(e.mem.u32(search + 0x2048), start);
        // Without neighbours the list runs dry: the last node stays current.
        e.mem.set_u32(control + 8, 0);
        e.mem.set_u32(search + 0x1ff4 + 4, start);
        e.mem.set_u32(start + 0x2c, 0);
        assert_eq!(e.call(0x006f_3fb0, &args![search]).u32(), 0);
        assert_eq!(e.mem.u32(search + 0x204c), start);
    }

    #[test]
    fn search_runs_and_writes_the_path() {
        let mut e = tds_engine(&[0x006a_8890, 0x006d_bcf0]);
        e.register(0x00b0_0a00, |_, _| 0x0300_0100u32.into_ret());
        e.register(0x0040_fc90, |_, _| 5u32.into_ret());
        e.map(0x0300_0100, 4);
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(0x0300_0100, vtable);
        e.mem.set_u32(vtable + 0xc, 0x0300_0200);
        e.register(0x0300_0200, |_, _| 1u32.into_ret());
        // The search loop finds nothing (empty open list); with `partial`
        // the best node's path is written.
        for addr in [0x006b_3eb0, ARRAY_RELEASE_008454F0, 0x006f_4640] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(0x0040_1020, |_, _| Ret::default());
        e.register(0x00aa_42e0, |_, _| Ret::default());
        e.register(0x006a_1440, |e, a| e.mem.alloc(0xc + a[1]).into_ret());
        let search = e.mem.alloc(0x20b8);
        let best = tds_node(&mut e, 0.0);
        e.mem.set_u32(search + 0x2048, best);
        let points = e.mem.alloc(0x10);
        let links = e.mem.alloc(0x10);
        assert!(!e
            .call(0x006f_3d90, &args![search, points, links, 1u32])
            .bool());
        assert_eq!(e.mem.u8(search + 0x2054), 1);
        assert_eq!(tds_calls(&e, 0x006a_8890), vec![vec![points, 1, 1]]);
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x006f_3d90, &args![search, points, links, 0u32])
            .bool());
        assert!(tds_calls(&e, 0x006a_8890).is_empty());
    }

    #[test]
    fn build_node_path_refuses_without_a_place_or_in_mode_2_outside() {
        let mut e = tds_engine(&[0x006d_cce0]);
        e.register(LOCATION_WORLDSPACE, |_, _| Ret::default());
        e.register(LOCATION_CELL, |_, _| Ret::default());
        let search = e.mem.alloc(0x20b8);
        let path = e.mem.alloc(0x40);
        let call = |e: &mut Engine, mode: u32| {
            e.call(
                0x006f_34e0,
                &args![search, 0x100u32, 0x200u32, path, mode, 1.0f32, 0u32, 0u32],
            )
            .bool()
        };
        assert!(!call(&mut e, 0));
        e.register(LOCATION_WORLDSPACE, |_, a| {
            (if a[0] == 0x100 { 0x4000u32 } else { 0 }).into_ret()
        });
        assert!(!call(&mut e, 2));
        assert_eq!(e.mem.u32(search + 0x205c), 0x4000);
        assert_eq!(e.mem.u8(search + 0x2058), 1);
        assert_eq!(e.mem.u32(search + 0x2098), 2);
    }

    #[test]
    fn get_node_connections_without_actor_or_lock_mode_only_gathers() {
        let mut e = tds_engine(&[ARRAY_RELEASE_008454F0, 0x0058_8270, 0x0054_db50]);
        let search = e.mem.alloc(0x20b8);
        let node = tds_node(&mut e, 0.0);
        e.mem.set_u8(node + 8, 1);
        e.mem.set_u32(node + 0xc, 0x4000);
        assert!(e.call(0x006f_36d0, &args![search, node, 0u32]).bool());
        assert_eq!(
            tds_calls(&e, 0x0058_8270),
            vec![vec![0x4000, search + 0x20a4]]
        );
        e.mem.set_u8(node + 8, 0);
        e.mem.set_u32(node + 0x10, 0x5000);
        assert!(e.call(0x006f_36d0, &args![search, node, 0u32]).bool());
        assert_eq!(
            tds_calls(&e, 0x0054_db50),
            vec![vec![0x5000, search + 0x20a4]]
        );
        assert!(tds_calls(&e, 0x0044_ddc0).is_empty());
    }

    #[test]
    fn get_node_connections_adds_a_reachable_door() {
        const POSITION: u32 = 0x0300_0000;
        let mut e = tds_engine(&[
            ARRAY_RELEASE_008454F0,
            0x0054_db50,
            DOOR_LOCK_CONSTRUCTOR,
            DOOR_LOCK_DESTRUCTOR,
            0x007c_b2e0,
            0x006d_5320,
            0x0099_6840,
        ]);
        e.map(ZERO, 8);
        e.map(DOOR_PENALTY, 4);
        e.set_global(DOOR_PENALTY, 100.0f32);
        let door = e.mem.alloc(0x10);
        let vtable = e.mem.alloc(0x200);
        e.mem.set_u32(door, vtable);
        e.mem.set_u32(vtable + 0x1f4, POSITION);
        let door_position = e.mem.alloc(0xc);
        e.register_double(POSITION, move |_, _| door_position.into_ret());
        let doors = e.mem.alloc(4);
        e.mem.set_u32(doors, door);
        e.register(0x0044_ddc0, |_, _| 1u32.into_ret());
        e.register_double(0x0087_7a30, move |_, _| doors.into_ret());
        e.register(0x0044_0da0, |_, _| Ret::default());
        e.register(TELEPORT_DATA, |_, _| 0x6000u32.into_ret());
        e.register(TELEPORT_WORLDSPACE, |_, _| Ret::default());
        e.register(TELEPORT_CELL, |_, _| 0x7000u32.into_ret());
        e.register(DOOR_LOCK_CAN_UNLOCK, |_, _| Ret::default());
        e.register(0x007a_f430, |_, _| Ret::default());
        e.register(0x0051_8000, |_, _| Ret::default());
        e.register(0x0059_bb30, |_, _| 1u32.into_ret());
        e.register(0x0043_9ef0, |_, a| a[1].into_ret());
        e.register(0x0045_7990, |_, _| 10.0f64.into_ret());
        let package = e.mem.alloc(0xc);
        e.mem.set_u32(package, 0x99);
        e.register_double(0x0071_7e50, move |_, _| package.into_ret());
        e.register(0x006a_1f90, |e, a| e.mem.alloc(a[0]).into_ret());
        let search = e.mem.alloc(0x20b8);
        e.mem.set_u32(search + 0x20b4, 2);
        e.mem.set_u32(search + 0x20a0, 0x1234);
        let node = tds_node(&mut e, 5.0);
        assert!(e.call(0x006f_36d0, &args![search, node, 0x8000u32]).bool());
        // Locked (cannot unlock) in lock mode 2: 5 + 10 + 100.
        let added = tds_calls(&e, 0x007c_b2e0);
        assert_eq!(added.len(), 1);
        let created = e.mem.u32(added[0][1]);
        assert_eq!(e.mem.f32(created), 115.0);
        assert_eq!(e.mem.u32(created + 0x30), 0x7000);
        assert_eq!(e.mem.u32(created + 0x10), 0x7000);
        assert_eq!(e.mem.u32(created + 0x14), door);
        assert_eq!(e.mem.u32(created + 0x18), 0x99);
        assert_eq!(e.mem.f32(created + 4), 1.0);
        // Lock mode 0: the locked door is skipped.
        e.mem.set_u32(search + 0x20b4, 0);
        e.call_log = Some(vec![]);
        e.call(0x006f_36d0, &args![search, node, 0x8000u32]);
        assert!(tds_calls(&e, 0x007c_b2e0).is_empty());
        assert_eq!(tds_calls(&e, DOOR_LOCK_DESTRUCTOR).len(), 1);
    }

    #[test]
    fn scalar_deleting_destructors_destroy_then_free_on_bit_0() {
        // (destructor, the call its body makes first)
        for (addr, first) in [
            (0x006f_3430u32, 0x005e_04f0u32),
            (0x006f_3f80, 0x0048_3710),
            (0x006f_4660, ARRAY_RELEASE_008454F0),
            (0x006f_4850, ARRAY_RELEASE_008454F0),
        ] {
            for flags in [0u32, 1, 2, 3] {
                let mut e = tds_engine(&[
                    0x005e_04f0,
                    0x004f_f7e0,
                    0x0048_3710,
                    0x0098_2610,
                    ARRAY_RELEASE_008454F0,
                    FREE_BLOCK,
                ]);
                let this = e.mem.alloc(0x20c0);
                assert_eq!(e.call(addr, &args![this, flags]).u32(), this);
                let log = e.call_log.take().unwrap();
                assert_eq!(log[1].0, first);
                let freed: Vec<_> = log.iter().filter(|c| c.0 == FREE_BLOCK).collect();
                if flags & 1 != 0 {
                    assert_eq!(freed.len(), 1);
                    assert_eq!(freed[0].1, vec![this]);
                    assert_eq!(log.last().unwrap().0, FREE_BLOCK);
                } else {
                    assert!(freed.is_empty());
                }
            }
        }
    }
}
