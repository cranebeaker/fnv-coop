//! `fallout shared/pathfinding/navmeshobstaclecutter.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006bc300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ObstacleUndoData` scalar deleting destructor (Xbox PDB name from the engine map; identical-code folding can share this body): runs the
/// destructor body at `006bc330` and frees 0x70 bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn obstacle_undo_data_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x006b_c330, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0x70u32]);
    }
    this
}

/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@UBoundsTriangleInfo@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0106C2E8: u32 = 0x0106_c2e8;
/// `008454f0` (unnamed in the engine map): `thiscall`, `this` plus one stack word (`RET 4`). When the
/// buffer pointer (+4) is set it runs the instance's element destroy step; with the word non-zero it
/// then calls the array's virtual slot +8 on the buffer and clears the buffer pointer and the reserved
/// size (+0xc); finally it clears the size (+8).
const ARRAY_RELEASE_008454F0: u32 = 0x0084_54f0;
/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@UPortalEdgeSwap@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0106C2FC: u32 = 0x0106_c2fc;

// Translated from 006bf6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@UBoundsTriangleInfo@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_006bf6c0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106C2E8);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

// Translated from 006bf710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@UPortalEdgeSwap@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_006bf710(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106C2FC);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

/// `BSScrapArray<NavMeshObstacleVertex*>`'s vtable (RTTI
/// `.?AV?$BSScrapArray@PAUNavMeshObstacleVertex@@$0EAA@@@`).
const VERTEX_ARRAY_VTABLE: u32 = 0x0106_c310;
/// `BSScrapArray<NavMeshObstacleEdge*>`'s vtable (RTTI
/// `.?AV?$BSScrapArray@PAUNavMeshObstacleEdge@@$0EAA@@@`).
const EDGE_ARRAY_VTABLE: u32 = 0x0106_c338;
/// `ObstacleUndoData`'s vtable (RTTI `.?AVObstacleUndoData@@`).
const UNDO_DATA_VTABLE: u32 = 0x0106_c2d4;
/// `MemoryManager::GetThreadScrapHeap` (`00aa42e0`, `thiscall` on the memory manager that
/// `00401020` returns).
const GET_THREAD_SCRAP_HEAP: u32 = 0x00aa_42e0;
/// `00401020` (no arguments): the memory manager.
const MEMORY_MANAGER: u32 = 0x0040_1020;

// Translated from 006bbc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only, plain `RET`): runs `ScrapHeap::ScrapHeap`
/// (`00aa53f0`) on the object and returns it.
pub fn fn_006bbc30(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x00aa_53f0, &args![this]);
    this
}

/// The `BSScrapArray` constructors here: the base constructor (`base`, `ECX` only), the
/// instance's vtable, the scrap heap at +0x10 (`heap`, or the thread's scrap heap when it is
/// null), then `006b3eb0` with the two size words.
fn construct_scrap_array(
    e: &mut Engine,
    this: Ptr,
    base: u32,
    vtable: u32,
    capacity: u32,
    count: u32,
    heap: u32,
) -> Ptr {
    e.call(base, &args![this]);
    e.mem.set_u32(this.addr(), vtable);
    let heap = if heap != 0 {
        heap
    } else {
        let manager = e.call(MEMORY_MANAGER, &args![]).u32();
        e.call(GET_THREAD_SCRAP_HEAP, &args![manager]).u32()
    };
    e.mem.set_u32(this.addr() + 0x10, heap);
    e.call(0x006b_3eb0, &args![this, capacity, count]);
    this
}

// Translated from 006bf7d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006bf7d0 - the exception-unwinding frame (FS:[0] chain, state variable) is not translated.
/// The `BSScrapArray<NavMeshObstacleVertex*>` constructor (unnamed in the engine map;
/// `ECX` and three stack words, `RET 0xc`): the base constructor `006bff00`, the vtable
/// `0x0106_c310`, the scrap heap (the third word, or `MemoryManager::GetThreadScrapHeap`
/// when it is 0) at +0x10, then `006b3eb0` with the first two words. Returns the array.
pub fn fn_006bf7d0(e: &mut Engine, this: Ptr, capacity: u32, count: u32, heap: u32) -> Ptr {
    construct_scrap_array(
        e,
        this,
        0x006b_ff00,
        VERTEX_ARRAY_VTABLE,
        capacity,
        count,
        heap,
    )
}

// Translated from 006bfa10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006bfa10 - the exception-unwinding frame (FS:[0] chain, state variable) is not translated.
/// The `BSScrapArray<NavMeshObstacleEdge*>` constructor (unnamed in the engine map; `ECX`
/// and three stack words, `RET 0xc`): as [`fn_006bf7d0`] with the base constructor
/// `006bff30` and the vtable `0x0106_c338`.
pub fn fn_006bfa10(e: &mut Engine, this: Ptr, capacity: u32, count: u32, heap: u32) -> Ptr {
    construct_scrap_array(
        e,
        this,
        0x006b_ff30,
        EDGE_ARRAY_VTABLE,
        capacity,
        count,
        heap,
    )
}

// Translated from 006bf890 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006bf890 - the exception-unwinding frame (FS:[0] chain, state variable) is not translated.
/// The `BSScrapArray<NavMeshObstacleVertex*>` destructor (unnamed in the engine map; `ECX`
/// only, plain `RET`): stores the vtable `0x0106_c310`, releases the buffer through
/// `008454f0` with 1, then runs the base destructor `006bf870`.
pub fn fn_006bf890(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VERTEX_ARRAY_VTABLE);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
    e.call(0x006b_f870, &args![this]);
}

// Translated from 006bfad0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006bfad0 - the exception-unwinding frame (FS:[0] chain, state variable) is not translated.
/// The `BSScrapArray<NavMeshObstacleEdge*>` destructor (unnamed in the engine map; `ECX`
/// only, plain `RET`): stores the vtable `0x0106_c338`, releases the buffer through
/// `008454f0` with 1, then runs the base destructor `006bfab0`.
pub fn fn_006bfad0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), EDGE_ARRAY_VTABLE);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
    e.call(0x006b_fab0, &args![this]);
}

// Translated from 006bf690 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSSimpleArray<BoundsTriangleInfo>` constructor (unnamed in the engine map; `ECX`
/// and two stack words, `RET 8`): stores the vtable `0x0106_c2e8` and runs `006b3f40` with
/// the two words. Returns the array.
pub fn fn_006bf690(e: &mut Engine, this: Ptr, capacity: u32, count: u32) -> Ptr {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106C2E8);
    e.call(0x006b_3f40, &args![this, capacity, count]);
    this
}

// Translated from 006bf6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSSimpleArray<PortalEdgeSwap>` constructor (unnamed in the engine map; `ECX` only,
/// plain `RET`): stores the vtable `0x0106_c2fc` and runs `006bfe70` with 0 and 0. Returns
/// the array.
pub fn fn_006bf6e0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106C2FC);
    e.call(0x006b_fe70, &args![this, 0u32, 0u32]);
    this
}

// Translated from 006bbb80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006bbb80 - the exception-unwinding frame (FS:[0] chain, handler 00f03171, state variable) is not translated.
/// `NavMeshObstacleCutter::NavMeshObstacleCutter` (Xbox PDB; `ECX` only, plain `RET`):
/// `00633c90` on +4 with 0, the scrap heap at +0xc ([`fn_006bbc30`]), then the vertex array
/// at +0x1c and the edge array at +0x30 (0x100, 0, and the scrap heap's address from
/// `006815c0`). Returns the cutter.
pub fn nav_mesh_obstacle_cutter_nav_mesh_obstacle_cutter(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0063_3c90, &args![base + 4, 0u32]);
    fn_006bbc30(e, Ptr::new(base + 0xc));
    let heap = e.call(0x0068_15c0, &args![base + 0xc]).u32();
    fn_006bf7d0(e, Ptr::new(base + 0x1c), 0x100, 0, heap);
    let heap = e.call(0x0068_15c0, &args![base + 0xc]).u32();
    fn_006bfa10(e, Ptr::new(base + 0x30), 0x100, 0, heap);
    this
}

// Translated from 006bbc50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006bbc50 - the exception-unwinding frame (FS:[0] chain, handler 00f031bc, state variable) is not translated.
/// `NavMeshObstacleCutter::~NavMeshObstacleCutter` (Xbox PDB; `ECX` only, plain `RET`): for
/// each edge of the array at +0x30 (count `0044ddc0`, element `00877a30`) whose word at
/// `00559450` for the edge's +0x1c is not null, releases that object through `00401970`;
/// then the edge array ([`fn_006bfad0`]), the vertex array ([`fn_006bf890`]), the scrap heap
/// (`00982610` on +0xc) and `0045cec0` on +4 are destroyed.
pub fn nav_mesh_obstacle_cutter_destructor(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let mut index = 0u32;
    while index < e.call(0x0044_ddc0, &args![base + 0x30]).u32() {
        let slot = e.call(0x0087_7a30, &args![base + 0x30, index]).u32();
        let edge = e.mem.u32(slot);
        if e.call(0x0055_9450, &args![edge + 0x1c]).u32() != 0 {
            let slot = e.call(0x0087_7a30, &args![base + 0x30, index]).u32();
            let edge = e.mem.u32(slot);
            let object = e.call(0x0055_9450, &args![edge + 0x1c]).u32();
            e.call(0x0040_1970, &args![object]);
        }
        index += 1;
    }
    fn_006bfad0(e, Ptr::new(base + 0x30));
    fn_006bf890(e, Ptr::new(base + 0x1c));
    e.call(0x0098_2610, &args![base + 0xc]);
    e.call(0x0045_cec0, &args![base + 4]);
}

// Translated from 006bc230 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006bc230 - the exception-unwinding frame (FS:[0] chain, handler 00f0325f, state variable) is not translated.
/// The `ObstacleUndoData` constructor (unnamed in the engine map; `ECX` only, plain `RET`):
/// the base constructor `004968b0`, the vtable `0x0106_c2d4`, then the members: `00633c90`
/// on +0xc with 0, `0069ada0` on +0x20 (0x20, 0), the `BoundsTriangleInfo` array on +0x30
/// ([`fn_006bf690`] with 0x20, 0), `006916f0` on +0x40 with 0x25, the `PortalEdgeSwap`
/// array on +0x50 ([`fn_006bf6e0`]), `006917d0` on +0x60, and 0 at +8. Returns the object.
pub fn fn_006bc230(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0049_68b0, &args![this]);
    e.mem.set_u32(base, UNDO_DATA_VTABLE);
    e.call(0x0063_3c90, &args![base + 0xc, 0u32]);
    e.call(0x0069_ada0, &args![base + 0x20, 0x20u32, 0u32]);
    fn_006bf690(e, Ptr::new(base + 0x30), 0x20, 0);
    e.call(0x0069_16f0, &args![base + 0x40, 0x25u32]);
    fn_006bf6e0(e, Ptr::new(base + 0x50));
    e.call(0x0069_17d0, &args![base + 0x60]);
    e.mem.set_u32(base + 8, 0);
    this
}

// Translated from 006bc330 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006bc330 - the exception-unwinding frame (FS:[0] chain, handler 00f0325f, state variable) is not translated.
/// The `ObstacleUndoData` destructor (unnamed in the engine map; the scalar deleting
/// destructor `006bc300` runs it; `ECX` only, plain `RET`): the members in reverse order:
/// `00691800` on +0x60, the `PortalEdgeSwap` array on +0x50 ([`fn_006bf710`]), `006919a0`
/// on +0x40, the `BoundsTriangleInfo` array on +0x30 ([`fn_006bf6c0`]), `00691800` on
/// +0x20, `0045cec0` on +0xc, then the base destructor `00496910`.
pub fn fn_006bc330(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(0x0069_1800, &args![base + 0x60]);
    fn_006bf710(e, Ptr::new(base + 0x50));
    e.call(0x0069_19a0, &args![base + 0x40]);
    fn_006bf6c0(e, Ptr::new(base + 0x30));
    e.call(0x0069_1800, &args![base + 0x20]);
    e.call(0x0045_cec0, &args![base + 0xc]);
    e.call(0x0049_6910, &args![this]);
}
/// `BSSimpleArray`/`BSScrapArray` element access used throughout the cutter (all `thiscall`
/// on the array): `0044ddc0` the count, `00877a30` the address of the element at an index.
const ARRAY_COUNT: u32 = 0x0044_ddc0;
const ARRAY_ELEMENT: u32 = 0x0087_7a30;
/// Appending a word (`007cb2e0`, `thiscall`: the address of the word).
const ARRAY_APPEND: u32 = 0x007c_b2e0;
/// `005eeda0` and `006a1440` (`thiscall`, one word): the address of the element at an
/// index of the undo data's triangle-index array (+0x20) and saved-triangle array (+0x30).
const TRIANGLE_INDEX_AT: u32 = 0x005e_eda0;
const SAVED_TRIANGLE_AT: u32 = 0x006a_1440;
/// `005582f0` (`thiscall` on the navmesh, one word: a 16-bit index): the triangle.
const NAVMESH_TRIANGLE: u32 = 0x0055_82f0;
/// `0068f0a0` (`thiscall` on the navmesh, one word: a 16-bit index): the vertex position.
const NAVMESH_VERTEX: u32 = 0x0068_f0a0;
/// `00559450` (`thiscall`): the pointer an `NiPointer` holds.
const NI_POINTER_GET: u32 = 0x0055_9450;
/// `00401970`: releases a reference-counted object.
const RELEASE: u32 = 0x0040_1970;
/// `006be0c0` (`thiscall` on an edge): whether the edge takes part in the triangle search.
const EDGE_IS_OPEN: u32 = 0x006b_e0c0;
/// `006beea0` (`thiscall` on the cutter, two vertices): whether an edge may join them.
const EDGE_ALLOWED: u32 = 0x006b_eea0;
/// `006bf4d0` (`thiscall` on the cutter: two vertices, a kind): a new edge.
const NEW_EDGE: u32 = 0x006b_f4d0;
/// `006bf3c0` (`thiscall` on the cutter: a 16-bit vertex index, three flags): a new vertex.
const NEW_VERTEX: u32 = 0x006b_f3c0;
/// `006bcfe0` (`thiscall` on an edge: a 16-bit triangle index, a side).
const EDGE_SET_TRIANGLE: u32 = 0x006b_cfe0;
/// The comparison functions the sorted searches pass (`006bfb70` for portal entries,
/// `009a3830` for indices, `0069b1b0` and `0069b2a0` for triangle indices) and the sort's
/// (`006bfb30`).
const PORTAL_COMPARE: u32 = 0x006b_fb70;
const INDEX_COMPARE: u32 = 0x009a_3830;
const TRIANGLE_COMPARE: u32 = 0x0069_b1b0;
const TRIANGLE_COMPARE_2: u32 = 0x0069_b2a0;
const DESCENDING_COMPARE: u32 = 0x006b_fb30;

fn array_count(e: &mut Engine, array: u32) -> u32 {
    e.call(ARRAY_COUNT, &args![array]).u32()
}

/// The word stored at index `index` of `array` (an edge or vertex pointer).
fn array_word(e: &mut Engine, array: u32, index: u32) -> u32 {
    let slot = e.call(ARRAY_ELEMENT, &args![array, index]).u32();
    e.mem.u32(slot)
}

/// Releases the object the `NiPointer` at +0x1c of the edge at `index` holds, when there is
/// one (the pointer is read again for the release, as the code does).
fn release_edge_object(e: &mut Engine, array: u32, index: u32) {
    let edge = array_word(e, array, index);
    if e.call(NI_POINTER_GET, &args![edge + 0x1c]).u32() != 0 {
        let edge = array_word(e, array, index);
        let object = e.call(NI_POINTER_GET, &args![edge + 0x1c]).u32();
        e.call(RELEASE, &args![object]);
    }
}

/// The triangle of the navmesh (`[this]`) whose 16-bit index `index` is.
fn triangle(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let navmesh = e.mem.u32(this.addr());
    e.call(NAVMESH_TRIANGLE, &args![navmesh, index & 0xffff])
        .u32()
}

/// The position of the vertex whose 16-bit index is the word at `vertex`.
fn vertex_position(e: &mut Engine, this: Ptr, vertex: u32) -> u32 {
    let index = u32::from(e.mem.u16(vertex));
    let navmesh = e.mem.u32(this.addr());
    e.call(NAVMESH_VERTEX, &args![navmesh, index]).u32()
}

// Translated from 006bf340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` the cutter, unused; one stack word, the edge, `RET 4`):
/// whether the edge is to be dropped. True when either end vertex has its byte +2 clear and
/// its byte +6 set (checked for the first vertex, then the second); otherwise true when both
/// vertices have byte +2 set and the edge's kind (+0x10) is not 1.
pub fn fn_006bf340(e: &mut Engine, _this: Ptr, edge: Ptr) -> bool {
    let first = e.mem.u32(edge.addr());
    if e.mem.u8(first + 2) == 0 && e.mem.u8(first + 6) != 0 {
        return true;
    }
    let second = e.mem.u32(edge.addr() + 4);
    if e.mem.u8(second + 2) == 0 && e.mem.u8(second + 6) != 0 {
        return true;
    }
    let first = e.mem.u32(edge.addr());
    if e.mem.u8(first + 2) == 0 {
        return false;
    }
    let second = e.mem.u32(edge.addr() + 4);
    e.mem.u8(second + 2) != 0 && e.mem.u32(edge.addr() + 0x10) != 1
}

// Translated from 006bf2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` the cutter, one stack word: a vertex, `RET 4`): gives
/// the navmesh position of the vertex (its 16-bit index through `0068f0a0`) to `006bf2d0`
/// on the cutter and returns that answer (`AL`).
pub fn fn_006bf2a0(e: &mut Engine, this: Ptr, vertex: Ptr) -> bool {
    let position = vertex_position(e, this, vertex.addr());
    e.call(0x006b_f2d0, &args![this, position]).bool()
}

// Translated from 006bec20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` the cutter, plain `RET`): `006910a0` on the navmesh.
pub fn fn_006bec20(e: &mut Engine, this: Ptr) {
    let navmesh = e.mem.u32(this.addr());
    e.call(0x0069_10a0, &args![navmesh]);
}

// Translated from 006beb80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` the cutter, plain `RET`): when the undo data (+8) added
/// triangles (count at +0x1c), `006965d0` runs on the navmesh with the first new triangle's
/// 16-bit index (+0x18) and the last one's (first + count - 1, as a word).
pub fn fn_006beb80(e: &mut Engine, this: Ptr) {
    let undo = e.mem.u32(this.addr() + 8);
    if e.mem.u32(undo + 0x1c) == 0 {
        return;
    }
    let first = e.mem.u32(undo + 0x18);
    let added = e.mem.u32(undo + 0x1c);
    let last = first.wrapping_add(added).wrapping_sub(1);
    let first_half = u32::from(e.mem.u16(undo + 0x18));
    let navmesh = e.mem.u32(this.addr());
    e.call(0x0069_65d0, &args![navmesh, first_half, last]);
}

// Translated from 006be660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` the cutter, plain `RET`): every vertex of the vertex
/// array (+0x1c) whose byte +2 is set is passed to `00691820` on the navmesh's +0xe4 and on
/// the undo data's +0x60.
pub fn fn_006be660(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let mut index = 0u32;
    while index < array_count(e, base + 0x1c) {
        let vertex = array_word(e, base + 0x1c, index);
        if e.mem.u8(vertex + 2) != 0 {
            let vertex = array_word(e, base + 0x1c, index);
            let navmesh = e.mem.u32(base);
            e.call(0x0069_1820, &args![navmesh + 0xe4, vertex]);
            let vertex = array_word(e, base + 0x1c, index);
            let undo = e.mem.u32(base + 8);
            e.call(0x0069_1820, &args![undo + 0x60, vertex]);
        }
        index += 1;
    }
}

// Translated from 006be6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` the cutter, plain `RET`): for each entry of the undo
/// data's array at +0x60 (`005eeda0`), the sorted search `0069b240` on the navmesh's +0xe4
/// (the entry, 0, `0069b1b0`); a found index is removed there (`0069add0` with 1).
pub fn fn_006be6f0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let mut index = 0u32;
    loop {
        let undo = e.mem.u32(base + 8);
        if index >= array_count(e, undo + 0x60) {
            break;
        }
        let undo = e.mem.u32(base + 8);
        let entry = e.call(TRIANGLE_INDEX_AT, &args![undo + 0x60, index]).u32();
        let navmesh = e.mem.u32(base);
        let found = e
            .call(
                0x0069_b240,
                &args![navmesh + 0xe4, entry, 0u32, TRIANGLE_COMPARE],
            )
            .u32();
        if found != 0xffff_ffff {
            let navmesh = e.mem.u32(base);
            e.call(0x0069_add0, &args![navmesh + 0xe4, found, 1u32]);
        }
        index += 1;
    }
}

// Translated from 006be770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` the cutter, plain `RET`): the new triangles (from the
/// undo data's first index +0x18, count +0x1c; the index as a word) get `00693bd0` with the
/// cutter's word at +0x44 plus 1; the saved triangles (undo +0x30) get it with their own
/// `00699790` value plus 1.
pub fn fn_006be770(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let undo = e.mem.u32(base + 8);
    let mut index = e.mem.u32(undo + 0x18);
    loop {
        let undo = e.mem.u32(base + 8);
        let end = e.mem.u32(undo + 0x18).wrapping_add(e.mem.u32(undo + 0x1c));
        if index >= end {
            break;
        }
        let value = e.mem.u32(base + 0x44).wrapping_add(1);
        let tri = triangle(e, this, index);
        e.call(0x0069_3bd0, &args![tri, value]);
        index += 1;
    }
    let mut index = 0u32;
    loop {
        let undo = e.mem.u32(base + 8);
        if index >= array_count(e, undo + 0x30) {
            break;
        }
        let undo = e.mem.u32(base + 8);
        let saved = e.call(SAVED_TRIANGLE_AT, &args![undo + 0x30, index]).u32();
        let tri = triangle(e, this, u32::from(e.mem.u16(saved)));
        let value = u32::from(e.call(0x0069_9790, &args![tri]).u16()) + 1;
        e.call(0x0069_3bd0, &args![tri, value]);
        index += 1;
    }
}

// Translated from 006be830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` the cutter, plain `RET`): for each edge (+0x30) whose
/// triangles at +8 and +0xa are both set (not 0xffff), links them through `006907d0`: the
/// triangle at +8 gets (the edge's word +0xc, the triangle at +0xa) and the one at +0xa gets
/// (the edge's word +0xe, the triangle at +8). Every field is read through a fresh element
/// lookup, as the code does.
pub fn fn_006be830(e: &mut Engine, this: Ptr) {
    let edges = this.addr() + 0x30;
    let half = |e: &mut Engine, index: u32, offset: u32| {
        let edge = array_word(e, edges, index);
        u32::from(e.mem.u16(edge + offset))
    };
    let mut index = 0u32;
    while index < array_count(e, edges) {
        if half(e, index, 8) != 0xffff && half(e, index, 0xa) != 0xffff {
            let other = half(e, index, 0xa);
            let side = half(e, index, 0xc);
            let own = half(e, index, 8);
            let tri = triangle(e, this, own);
            e.call(0x0069_07d0, &args![tri, side, other]);
            let other = half(e, index, 8);
            let side = half(e, index, 0xe);
            let own = half(e, index, 0xa);
            let tri = triangle(e, this, own);
            e.call(0x0069_07d0, &args![tri, side, other]);
        }
        index += 1;
    }
}

// Translated from 006bc940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` the cutter, plain `RET`): folds `00699760` (cdecl: the
/// running value, a triangle's `00699790` value as a 16-bit word) over the triangles of the
/// undo data's index array (+0x20) and then its saved triangles (+0x30), starting from 0;
/// returns the result.
pub fn fn_006bc940(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    let mut value = 0u32;
    for (array_offset, accessor) in [(0x20u32, TRIANGLE_INDEX_AT), (0x30, SAVED_TRIANGLE_AT)] {
        let mut index = 0u32;
        loop {
            let undo = e.mem.u32(base + 8);
            if index >= array_count(e, undo + array_offset) {
                break;
            }
            let undo = e.mem.u32(base + 8);
            let entry = e.call(accessor, &args![undo + array_offset, index]).u32();
            let tri = triangle(e, this, u32::from(e.mem.u16(entry)));
            let level = u32::from(e.call(0x0069_9790, &args![tri]).u16());
            value = e.call(0x0069_9760, &args![value, level]).u32();
            index += 1;
        }
    }
    value
}

// Translated from 006bde50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` the cutter, plain `RET`): for each vertex (+0x1c) whose
/// byte +2 is set, and each vertex whose byte +6 is clear and byte +3 or +4 set, an edge
/// between them is added (`006bf4d0` with kind 0, appended to +0x30) when `006beea0`
/// allows it.
pub fn fn_006bde50(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let vertices = base + 0x1c;
    let mut first = 0u32;
    while first < array_count(e, vertices) {
        let from = array_word(e, vertices, first);
        if e.mem.u8(from + 2) != 0 {
            let mut second = 0u32;
            while second < array_count(e, vertices) {
                let to = array_word(e, vertices, second);
                if e.mem.u8(to + 6) == 0
                    && (e.mem.u8(to + 3) != 0 || e.mem.u8(to + 4) != 0)
                    && e.call(EDGE_ALLOWED, &args![this, from, to]).bool()
                {
                    let edge = e.call(NEW_EDGE, &args![this, from, to, 0u32]).u32();
                    e.with_stack(4, |e, slot| {
                        e.mem.set_u32(slot.addr(), edge);
                        e.call(ARRAY_APPEND, &args![base + 0x30, slot]);
                    });
                }
                second += 1;
            }
        }
        first += 1;
    }
}

/// Whether the edge at `index` is of kind 1 (+0x10) and one of its vertices (the first, then
/// the second) has neither byte +5 nor byte +4 set; every read goes through a fresh element
/// lookup, as the code does.
fn kind_one_with_lone_vertex(e: &mut Engine, edges: u32, index: u32) -> bool {
    let edge = array_word(e, edges, index);
    if e.mem.u32(edge + 0x10) != 1 {
        return false;
    }
    for offset in [0u32, 4] {
        let edge = array_word(e, edges, index);
        let vertex = e.mem.u32(edge + offset);
        if e.mem.u8(vertex + 5) != 0 {
            continue;
        }
        let edge = array_word(e, edges, index);
        let vertex = e.mem.u32(edge + offset);
        if e.mem.u8(vertex + 4) == 0 {
            return true;
        }
    }
    false
}

// Translated from 006bdcb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshObstacleCutter::RemoveInvalidEdges` (Xbox PDB; `ECX` the cutter, plain `RET`):
/// removes (releasing the edge's object at +0x1c first, then `009a4320` with the index and
/// 1, the index stepping back) every edge that [`fn_006bf340`] drops, and every edge of
/// kind 1 one of whose vertices has neither byte +5 nor byte +4 set.
pub fn nav_mesh_obstacle_cutter_remove_invalid_edges(e: &mut Engine, this: Ptr) {
    let edges = this.addr() + 0x30;
    let mut index = 0u32;
    while index < array_count(e, edges) {
        let edge = array_word(e, edges, index);
        let remove =
            fn_006bf340(e, this, Ptr::new(edge)) || kind_one_with_lone_vertex(e, edges, index);
        if remove {
            release_edge_object(e, edges, index);
            e.call(0x009a_4320, &args![edges, index, 1u32]);
            index = index.wrapping_sub(1);
        }
        index = index.wrapping_add(1);
    }
}

/// The two edges' ends when they share one: `(shared, first's other, second's other)`, or
/// `None`. The cases are tried in the code's order (first/first, first/second,
/// second/first, second/second).
fn shared_end(e: &mut Engine, first: u32, second: u32) -> Option<(u32, u32, u32)> {
    let (a0, a1) = (e.mem.u32(first), e.mem.u32(first + 4));
    let (b0, b1) = (e.mem.u32(second), e.mem.u32(second + 4));
    if a0 == b0 {
        Some((a0, a1, b1))
    } else if a0 == b1 {
        Some((a0, a1, b0))
    } else if a1 == b0 {
        Some((a1, a0, b1))
    } else if a1 == b1 {
        Some((a1, a0, b0))
    } else {
        None
    }
}

// Translated from 006bdf50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` the cutter, plain `RET`): for each pair of open edges
/// (`006be0c0`; the second after the first in the array) that share an end, the edge
/// closing the triangle (between the two other ends) is added (`006bf4d0` with kind 0,
/// appended to +0x30) when `006beea0` allows it.
pub fn fn_006bdf50(e: &mut Engine, this: Ptr) {
    let edges = this.addr() + 0x30;
    let mut first = 0u32;
    while first < array_count(e, edges) {
        let a = array_word(e, edges, first);
        if e.call(EDGE_IS_OPEN, &args![a]).bool() {
            let mut second = first + 1;
            while second < array_count(e, edges) {
                let b = array_word(e, edges, second);
                if e.call(EDGE_IS_OPEN, &args![b]).bool() {
                    if let Some((_, from, to)) = shared_end(e, a, b) {
                        if e.call(EDGE_ALLOWED, &args![this, from, to]).bool() {
                            let edge = e.call(NEW_EDGE, &args![this, from, to, 0u32]).u32();
                            e.with_stack(4, |e, slot| {
                                e.mem.set_u32(slot.addr(), edge);
                                e.call(ARRAY_APPEND, &args![edges, slot]);
                            });
                        }
                    }
                }
                second += 1;
            }
        }
        first += 1;
    }
}

// Translated from 006bca20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` the cutter, one stack word: the saved-triangle array,
/// `RET 4`): for each triangle of the undo data's index array (+0x20), flagged with
/// `00519270` (0x100), each side `k` (0..3) that `0068f1d0` says is not a border and
/// `0068f200` says has a neighbour: the neighbour's 16-bit index (`0068f2c0`), when below
/// the navmesh's triangle count (`00558200`) and not in the index array (sorted search
/// `0069b240` with `0069b2a0`) nor already in the saved array (16-bit word at each entry),
/// is saved: the array grows by one (`006a8890` with the new count and 1), the entry gets the
/// index, the neighbour is flagged (`00691570`, 0x100), and for each of its sides the entry
/// records whether it is a border (byte +2+side) and its neighbour word (+6+2*side).
pub fn fn_006bca20(e: &mut Engine, this: Ptr, saved: Ptr) {
    let base = this.addr();
    let list = saved.addr();
    let mut index = 0u32;
    loop {
        let undo = e.mem.u32(base + 8);
        if index >= array_count(e, undo + 0x20) {
            break;
        }
        let undo = e.mem.u32(base + 8);
        let entry = e.call(TRIANGLE_INDEX_AT, &args![undo + 0x20, index]).u32();
        let tri = triangle(e, this, u32::from(e.mem.u16(entry)));
        e.call(0x0051_9270, &args![tri, 0x100u32]);
        let mut side = 0i32;
        while side < 3 {
            let k = side as u32;
            if !e.call(0x0068_f1d0, &args![tri, k]).bool()
                && e.call(0x0068_f200, &args![tri, k]).bool()
            {
                e.with_stack(4, |e, slot| {
                    let neighbour = u32::from(e.call(0x0068_f2c0, &args![tri, k]).u16());
                    e.mem.set_u32(slot.addr(), neighbour);
                    let navmesh = e.mem.u32(base);
                    if neighbour >= e.call(0x0055_8200, &args![navmesh]).u32() {
                        return;
                    }
                    let other = triangle(e, this, neighbour);
                    let undo = e.mem.u32(base + 8);
                    let found = e
                        .call(
                            0x0069_b240,
                            &args![undo + 0x20, slot, 0u32, TRIANGLE_COMPARE_2],
                        )
                        .u32();
                    if found != 0xffff_ffff {
                        return;
                    }
                    let mut listed = false;
                    let mut position = 0u32;
                    while position < array_count(e, list) {
                        let saved_entry = e.call(SAVED_TRIANGLE_AT, &args![list, position]).u32();
                        if u32::from(e.mem.u16(saved_entry)) == e.mem.u32(slot.addr()) {
                            listed = true;
                            break;
                        }
                        position += 1;
                    }
                    if listed {
                        return;
                    }
                    let count = array_count(e, list);
                    e.call(0x006a_8890, &args![list, count.wrapping_add(1), 1u32]);
                    let saved_entry = e.call(SAVED_TRIANGLE_AT, &args![list, count]).u32();
                    let neighbour = e.mem.u16(slot.addr());
                    e.mem.set_u16(saved_entry, neighbour);
                    e.call(0x0069_1570, &args![other, 0x100u32]);
                    for other_side in 0..3u32 {
                        let border = e.call(0x0068_f1d0, &args![other, other_side]).bool();
                        let saved_entry = e.call(SAVED_TRIANGLE_AT, &args![list, count]).u32();
                        e.mem.set_u8(saved_entry + 2 + other_side, u8::from(border));
                        let word = e.call(0x0068_f2c0, &args![other, other_side]).u16();
                        let saved_entry = e.call(SAVED_TRIANGLE_AT, &args![list, count]).u32();
                        e.mem.set_u16(saved_entry + 6 + other_side * 2, word);
                    }
                });
            }
            side += 1;
        }
        index += 1;
    }
}

// Translated from 006bcc60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` the cutter, plain `RET`): builds the cut's border edges.
/// For each triangle of the undo data's index array (+0x20) and each side `k` (0..3): a
/// border side (`0068f1d0`) is kind 4 (outer); a side without a neighbour (0xffff from
/// `0068f2c0`) kind 3; a side whose neighbour is not in the index array (sorted search with
/// `0069b1b0`) kind 2 (inner). For such a side, its two vertices (`005582d0` with `k` and
/// `(k + 1) % 3`) are found in the vertex array (`006bf170`) or created (`006bf3c0` with the
/// index, 0 and the inner/outer flags, appended to +0x1c); found ones get the flags OR-ed
/// into bytes +3 (inner) and +4 (outer). The edge (`006bf4d0` with the kind) then gets: for
/// kind 2, its triangles from `NavMesh::GetMatchingEdge` (`006bcfe0`); for kind 3,
/// `006997b0` on the navmesh with the triangle and the edge's +0x1c; for kind 4, +0x20
/// cleared, the triangle at +0x24, the side at +0x28 and the neighbour word at +0x2c. It is
/// appended to +0x30, and for `k < 2` `00691040` records its +0x16, +0x18 and +0x19.
pub fn fn_006bcc60(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(0x20, |e, locals| {
        let neighbour_slot = locals.addr();
        let vertex_slot = locals.addr() + 4;
        let edge_slot = locals.addr() + 8;
        let matched = locals.addr() + 0xc;
        let first_triangle = locals.addr() + 0x10;
        let second_triangle = locals.addr() + 0x14;
        let mut index = 0u32;
        loop {
            let undo = e.mem.u32(base + 8);
            if index >= array_count(e, undo + 0x20) {
                break;
            }
            let undo = e.mem.u32(base + 8);
            let entry = e.call(TRIANGLE_INDEX_AT, &args![undo + 0x20, index]).u32();
            let tri = triangle(e, this, u32::from(e.mem.u16(entry)));
            let mut side = 0i32;
            while side < 3 {
                let k = side as u32;
                let mut kind = 3u32;
                let mut inner = false;
                let mut outer = false;
                if e.call(0x0068_f1d0, &args![tri, k]).bool() {
                    kind = 4;
                    outer = true;
                } else if e.call(0x0068_f2c0, &args![tri, k]).u16() == 0xffff {
                    kind = 3;
                    outer = true;
                } else {
                    let neighbour = e.call(0x0068_f2c0, &args![tri, k]).u16();
                    e.mem.set_u16(neighbour_slot, neighbour);
                    let undo = e.mem.u32(base + 8);
                    let found = e
                        .call(
                            0x0069_b240,
                            &args![undo + 0x20, neighbour_slot, 0u32, TRIANGLE_COMPARE],
                        )
                        .u32();
                    if found == 0xffff_ffff {
                        kind = 2;
                        inner = true;
                    }
                }
                if inner || outer {
                    let ends = [
                        e.call(0x0055_82d0, &args![tri, k]).u16(),
                        e.call(0x0055_82d0, &args![tri, ((side + 1) % 3) as u32])
                            .u16(),
                    ];
                    let mut vertices = [0u32; 2];
                    for (end, vertex_index) in ends.into_iter().enumerate() {
                        let found = e
                            .call(0x006b_f170, &args![this, u32::from(vertex_index)])
                            .u32();
                        if found == 0xffff_ffff {
                            let vertex = e
                                .call(
                                    NEW_VERTEX,
                                    &args![
                                        this,
                                        u32::from(vertex_index),
                                        0u32,
                                        u32::from(inner),
                                        u32::from(outer)
                                    ],
                                )
                                .u32();
                            e.mem.set_u32(vertex_slot, vertex);
                            e.call(ARRAY_APPEND, &args![base + 0x1c, vertex_slot]);
                            vertices[end] = e.mem.u32(vertex_slot);
                        } else {
                            let vertex = array_word(e, base + 0x1c, found);
                            let flag = u8::from(e.mem.u8(vertex + 3) != 0 || inner);
                            let vertex = array_word(e, base + 0x1c, found);
                            e.mem.set_u8(vertex + 3, flag);
                            let vertex = array_word(e, base + 0x1c, found);
                            let flag = u8::from(e.mem.u8(vertex + 4) != 0 || outer);
                            let vertex = array_word(e, base + 0x1c, found);
                            e.mem.set_u8(vertex + 4, flag);
                            vertices[end] = array_word(e, base + 0x1c, found);
                        }
                    }
                    let edge = e
                        .call(NEW_EDGE, &args![this, vertices[0], vertices[1], kind])
                        .u32();
                    e.mem.set_u32(edge_slot, edge);
                    if kind == 2 {
                        let undo = e.mem.u32(base + 8);
                        let entry = e.call(TRIANGLE_INDEX_AT, &args![undo + 0x20, index]).u32();
                        let own = u32::from(e.mem.u16(entry));
                        let navmesh = e.mem.u32(base);
                        if e.call(
                            0x0068_f460,
                            &args![navmesh, own, k, matched, first_triangle, second_triangle],
                        )
                        .bool()
                        {
                            let edge = e.mem.u32(edge_slot);
                            let first = u32::from(e.mem.u16(first_triangle));
                            let second = u32::from(e.mem.u16(second_triangle));
                            e.call(EDGE_SET_TRIANGLE, &args![edge, first, second]);
                        }
                    }
                    if kind == 3 {
                        let edge = e.mem.u32(edge_slot);
                        let undo = e.mem.u32(base + 8);
                        let entry = e.call(TRIANGLE_INDEX_AT, &args![undo + 0x20, index]).u32();
                        let own = u32::from(e.mem.u16(entry));
                        let navmesh = e.mem.u32(base);
                        e.call(0x0069_97b0, &args![navmesh, own, edge + 0x1c]);
                    }
                    if kind == 4 {
                        let edge = e.mem.u32(edge_slot);
                        e.mem.set_u32(edge + 0x20, 0);
                        let undo = e.mem.u32(base + 8);
                        let entry = e.call(TRIANGLE_INDEX_AT, &args![undo + 0x20, index]).u32();
                        let own = e.mem.u16(entry);
                        let edge = e.mem.u32(edge_slot);
                        e.mem.set_u16(edge + 0x24, own);
                        e.mem.set_u32(edge + 0x28, k);
                        let neighbour = e.call(0x0068_f2c0, &args![tri, k]).u16();
                        let edge = e.mem.u32(edge_slot);
                        e.mem.set_u16(edge + 0x2c, neighbour);
                    }
                    e.call(ARRAY_APPEND, &args![base + 0x30, edge_slot]);
                    if k < 2 {
                        let edge = e.mem.u32(edge_slot);
                        e.call(
                            0x0069_1040,
                            &args![tri, k & 0xffff, edge + 0x16, edge + 0x18, edge + 0x19],
                        );
                    }
                }
                side += 1;
            }
            index += 1;
        }
    });
}

/// The pair step the portal bookkeeping shares (`006be950` and `006bc3e0`): with the
/// stack entries `first` and `second` (0x1c bytes from `first`: the two 0xc-byte halves
/// and a flag byte), the sorted search `0049f7d0` (`006bfb70`) for `first` in `list`; a
/// found index goes once into `indices` (searched with `00719b20` and `009a3830`, appended
/// through `007cb2e0`), otherwise `first` is added to the list's +0x10 (`0069b310`) unless
/// it is there. Then `second` is added to `list` unless it is there.
fn merge_portal_pair(e: &mut Engine, list: u32, first: u32, second: u32, indices: u32, slot: u32) {
    let found = e
        .call(0x0049_f7d0, &args![list, first, 0u32, PORTAL_COMPARE])
        .u32();
    e.mem.set_u32(slot, found);
    if found != 0xffff_ffff {
        let listed = e
            .call(0x0071_9b20, &args![indices, slot, 0u32, INDEX_COMPARE])
            .u32();
        if listed == 0xffff_ffff {
            e.call(ARRAY_APPEND, &args![indices, slot]);
        }
    } else {
        let present = e
            .call(
                0x0049_f7d0,
                &args![list + 0x10, first, 0u32, PORTAL_COMPARE],
            )
            .u32();
        if present == 0xffff_ffff {
            e.call(0x0069_b310, &args![list + 0x10, first]);
        }
    }
    let present = e
        .call(0x0049_f7d0, &args![list, second, 0u32, PORTAL_COMPARE])
        .u32();
    if present == 0xffff_ffff {
        e.call(0x0069_b310, &args![list, second]);
    }
}

/// The tail both portal steps share: unless the index array is empty (`0076b610`), it is
/// sorted (`00729970` with `006bfb30`) and each index, from the last to the first, is
/// removed from `list` (`BSSimpleArray<NavMeshEdgeHandle>::Remove` with 1).
fn remove_listed_portals(e: &mut Engine, list: u32, indices: u32) {
    if e.call(0x0076_b610, &args![indices]).bool() {
        return;
    }
    e.call(0x0072_9970, &args![indices, DESCENDING_COMPARE]);
    let mut position = array_count(e, indices).wrapping_sub(1) as i32;
    while position >= 0 {
        let index = array_word(e, indices, position as u32);
        bs_simple_array_nav_mesh_edge_handle_remove(e, Ptr::new(list), index, 1);
        position -= 1;
    }
}

// Translated from 006be950 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006be950 - the exception-unwinding frame (FS:[0] chain, handler 00f032b8, state variable) is not translated.
/// Unnamed in the engine map (`ECX` the cutter, one stack word: the portal list, `RET 4`):
/// with a stack index array (`00438660`/`00438690`), each outer edge (kind 4, +0x10) with a
/// triangle (+8 not 0xffff) makes a portal pair: an entry (`006240d0`, then word 0, the
/// triangle at +4 and the edge's +0xc word at +8) copied (`006bc870`) into the second half,
/// the edge's +0x20 data copied into the first (built by `006bc8b0`), the flag 1, the pair
/// given to `006bf730` on the undo data's +0x50, then merged into the list (see
/// `merge_portal_pair`). Afterwards the listed indices are removed (see
/// `remove_listed_portals`).
pub fn fn_006be950(e: &mut Engine, this: Ptr, list: Ptr) {
    let base = this.addr();
    let edges = base + 0x30;
    e.with_stack(0x40, |e, locals| {
        let first = locals.addr();
        let second = first + 0xc;
        let flag = first + 0x18;
        let entry = first + 0x1c;
        let slot = first + 0x28;
        let indices = first + 0x2c;
        e.call(0x0043_8660, &args![indices]);
        let mut index = 0u32;
        while index < array_count(e, edges) {
            let edge = array_word(e, edges, index);
            if e.mem.u32(edge + 0x10) == 4 {
                let edge = array_word(e, edges, index);
                if e.mem.u16(edge + 8) != 0xffff {
                    e.call(0x0062_40d0, &args![entry]);
                    e.mem.set_u32(entry, 0);
                    let edge = array_word(e, edges, index);
                    let own = e.mem.u16(edge + 8);
                    e.mem.set_u16(entry + 4, own);
                    let edge = array_word(e, edges, index);
                    let side = u32::from(e.mem.u16(edge + 0xc));
                    e.mem.set_u32(entry + 8, side);
                    e.call(0x006b_c8b0, &args![first]);
                    e.call(0x006b_c870, &args![second, entry]);
                    let edge = array_word(e, edges, index);
                    e.call(0x006b_c870, &args![first, edge + 0x20]);
                    e.mem.set_u8(flag, 1);
                    let undo = e.mem.u32(base + 8);
                    e.call(0x006b_f730, &args![undo + 0x50, first]);
                    merge_portal_pair(e, list.addr(), first, second, indices, slot);
                }
            }
            index += 1;
        }
        remove_listed_portals(e, list.addr(), indices);
        e.call(0x0043_8690, &args![indices]);
    });
}

// Translated from 006be120 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006be120 - the /GS cookie check (00ec408c) is not translated.
/// Unnamed in the engine map (`ECX` the cutter, plain `RET`): fills the cut with triangles.
/// For each open edge `a` (`006be0c0`) and each later open edge `b` that does not share its
/// triangle (+8, unless that is 0xffff) and shares an end (`p` shared, `q` and `r` the other
/// ends of `a` and `b`), the first open edge `c` after `b` that joins `q` and `r` (either
/// way round) and passes `006bec40` and `006bede0` (with `p`, `q`, `r`) closes a triangle,
/// unless the cross product (`004b3800`) of `q - p` and `r - p` (`00439ef0` on the
/// positions) has a length (`004a7290`) below 0.1. The triangle (`00691160`) gets its
/// vertices (`006be630`), the edges' records (`006ad2f0`, sides 0 and 1) and the outer
/// edges' neighbour words (`00690e30`), in an order set by the sign of the cross product's
/// z (below 0.0: `p, r, q` with `b` and `c`; otherwise `p, q, r` with `a` and `c`); it is
/// added to the navmesh (`00691000`) and the three edges learn their side (`006bcfe0`). A
/// result of true means a triangle was added. After a triangle, the search for `a` stops
/// once `a` is no longer open; the next `b` is tried otherwise.
pub fn fn_006be120(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    let edges = base + 0x30;
    let mut added = false;
    e.with_stack(0x38, |e, locals| {
        let tri = locals.addr();
        let to_q = tri + 0x14;
        let to_r = tri + 0x20;
        let cross = tri + 0x2c;
        let mut first = 0u32;
        while first < array_count(e, edges) {
            let a = array_word(e, edges, first);
            if e.call(EDGE_IS_OPEN, &args![a]).bool() {
                let mut second = first + 1;
                let mut done = false;
                while !done && second < array_count(e, edges) {
                    let b = array_word(e, edges, second);
                    let candidate = e.call(EDGE_IS_OPEN, &args![b]).bool()
                        && !(e.mem.u16(a + 8) != 0xffff && e.mem.u16(a + 8) == e.mem.u16(b + 8));
                    if candidate {
                        if let Some((p, q, r)) = shared_end(e, a, b) {
                            let mut third = second + 1;
                            while third < array_count(e, edges) {
                                let c = array_word(e, edges, third);
                                third += 1;
                                if !e.call(EDGE_IS_OPEN, &args![c]).bool() {
                                    continue;
                                }
                                let (c0, c1) = (e.mem.u32(c), e.mem.u32(c + 4));
                                if !((c0 == q && c1 == r) || (c0 == r && c1 == q)) {
                                    continue;
                                }
                                if !e.call(0x006b_ec40, &args![this, p, q, r]).bool()
                                    || !e.call(0x006b_ede0, &args![this, p, q, r]).bool()
                                {
                                    continue;
                                }
                                let p_position = vertex_position(e, this, p);
                                let q_position = vertex_position(e, this, q);
                                e.call(0x0043_9ef0, &args![q_position, to_q, p_position]);
                                let p_position = vertex_position(e, this, p);
                                let r_position = vertex_position(e, this, r);
                                e.call(0x0043_9ef0, &args![r_position, to_r, p_position]);
                                e.call(0x004b_3800, &args![to_q, cross, to_r]);
                                let length = e.call(0x004a_7290, &args![cross]).f64();
                                let minimum: f64 = e.global(0x0101_ffa0);
                                // FCOMP with 0.1, `TEST AH,5; JP`: on to the triangle unless
                                // the length is below it.
                                if length < minimum {
                                    break;
                                }
                                e.call(0x0069_1160, &args![tri]);
                                let zero: f64 = e.global(0x0101_2060);
                                let below = x87_load32(e.mem.u32(cross + 8)) < zero;
                                let (pv, qv, rv) = (
                                    u32::from(e.mem.u16(p)),
                                    u32::from(e.mem.u16(q)),
                                    u32::from(e.mem.u16(r)),
                                );
                                let (first_edge, outer_sides) = if below {
                                    e.call(0x006b_e630, &args![tri, pv, rv, qv]);
                                    (b, [2u32, 0, 1])
                                } else {
                                    e.call(0x006b_e630, &args![tri, pv, qv, rv]);
                                    (a, [0u32, 2, 1])
                                };
                                for (record, edge) in [(0u32, first_edge), (1, c)] {
                                    let flags = (
                                        u32::from(e.mem.u16(edge + 0x16)),
                                        u32::from(e.mem.u8(edge + 0x18)),
                                        u32::from(e.mem.u8(edge + 0x19)),
                                    );
                                    e.call(
                                        0x006a_d2f0,
                                        &args![tri, record, flags.0, flags.1, flags.2],
                                    );
                                }
                                for (edge, side) in [
                                    (a, outer_sides[0]),
                                    (b, outer_sides[1]),
                                    (c, outer_sides[2]),
                                ] {
                                    if e.mem.u32(edge + 0x10) == 4 {
                                        let neighbour = u32::from(e.mem.u16(edge + 0x2c));
                                        e.call(0x0069_0e30, &args![tri, side, neighbour]);
                                    }
                                }
                                let navmesh = e.mem.u32(base);
                                let new_triangle =
                                    u32::from(e.call(0x0069_1000, &args![navmesh, tri]).u16());
                                added = true;
                                let zero: f64 = e.global(0x0101_2060);
                                let below = x87_load32(e.mem.u32(cross + 8)) < zero;
                                let sides = if below { [2u32, 0, 1] } else { [0u32, 2, 1] };
                                for (edge, side) in [(a, sides[0]), (b, sides[1]), (c, sides[2])] {
                                    e.call(EDGE_SET_TRIANGLE, &args![edge, new_triangle, side]);
                                }
                                if !e.call(EDGE_IS_OPEN, &args![a]).bool() {
                                    done = true;
                                }
                                break;
                            }
                        }
                    }
                    second += 1;
                }
            }
            first += 1;
        }
    });
    added
}
// Translated from 0061bf70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NavMeshEdgeHandle,1024>::Remove` (Xbox PDB; `ECX` the array, two stack
/// words: the index and a shrink flag byte, `RET 8`; 0xc-byte elements, buffer at +4, count
/// at +8, capacity at +0xc). With the flag and `006f3170` true, a new buffer of the
/// capacity `00869600` gives is allocated (vtable slot +4), the elements before the index
/// are moved into it (`006dc6e0`), the removed one is destroyed (`0072ba80` with 1), the
/// rest moved after it (with the count minus 1, as the code passes it), the old buffer freed
/// (`006a8500`) and the new buffer and capacity stored. Otherwise the element is destroyed
/// and the later ones moved down in place. The count drops by one.
pub fn bs_simple_array_nav_mesh_edge_handle_remove(
    e: &mut Engine,
    this: Ptr,
    index: u32,
    shrink: u8,
) {
    let base = this.addr();
    let offset = index.wrapping_mul(0xc);
    if shrink != 0 && e.call(0x006f_3170, &args![this]).bool() {
        let capacity = e.call(0x0086_9600, &args![this]).u32();
        let buffer = e.vcall(base, 4, &args![capacity]).u32();
        let old = e.mem.u32(base + 4);
        e.call(0x006d_c6e0, &args![this, buffer, old, index]);
        let removed = offset.wrapping_add(e.mem.u32(base + 4));
        e.call(0x0072_ba80, &args![this, removed, 1u32]);
        let count = e.mem.u32(base + 8).wrapping_sub(1);
        let source = e.mem.u32(base + 4).wrapping_add(offset).wrapping_add(0xc);
        e.call(
            0x006d_c6e0,
            &args![this, offset.wrapping_add(buffer), source, count],
        );
        e.call(0x006a_8500, &args![this]);
        e.mem.set_u32(base + 4, buffer);
        e.mem.set_u32(base + 0xc, capacity);
    } else {
        let removed = offset.wrapping_add(e.mem.u32(base + 4));
        e.call(0x0072_ba80, &args![this, removed, 1u32]);
        let count = e.mem.u32(base + 8).wrapping_sub(index).wrapping_sub(1);
        let source = e.mem.u32(base + 4).wrapping_add(offset).wrapping_add(0xc);
        let target = offset.wrapping_add(e.mem.u32(base + 4));
        e.call(0x006d_c6e0, &args![this, target, source, count]);
    }
    let count = e.mem.u32(base + 8).wrapping_sub(1);
    e.mem.set_u32(base + 8, count);
}

// Translated from 0049c250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<..., unsigned short, NiPointer<ObstacleData>>::RemoveAt` (Xbox PDB; `ECX` the
/// map, one stack word: the 16-bit key, `RET 4`): finds the key's bucket (vtable slot +4 on
/// the key) in the bucket array (+8) and walks its chain (item +0 next, +4 the 16-bit key)
/// comparing keys through vtable slot +8 (the key, the item's key). A found item is
/// unlinked, cleared (slot +0x10) and freed (slot +0x18), the count (+0xc) drops and the
/// answer is true; false otherwise.
pub fn ni_t_map_base_obstacle_data_remove_at(e: &mut Engine, this: Ptr, key: u16) -> bool {
    let base = this.addr();
    let key = u32::from(key);
    let bucket = e.vcall(base, 4, &args![key]).u32();
    let head = e.mem.u32(base + 8).wrapping_add(bucket.wrapping_mul(4));
    let mut previous = e.mem.u32(head);
    if previous == 0 {
        return false;
    }
    let item_key = u32::from(e.mem.u16(previous + 4));
    let found = if e.vcall(base, 8, &args![key, item_key]).bool() {
        let next = e.mem.u32(previous);
        let head = e.mem.u32(base + 8).wrapping_add(bucket.wrapping_mul(4));
        e.mem.set_u32(head, next);
        previous
    } else {
        let mut item = e.mem.u32(previous);
        while item != 0 {
            let item_key = u32::from(e.mem.u16(item + 4));
            if e.vcall(base, 8, &args![key, item_key]).bool() {
                break;
            }
            previous = item;
            item = e.mem.u32(item);
        }
        if item == 0 {
            return false;
        }
        let next = e.mem.u32(item);
        e.mem.set_u32(previous, next);
        item
    };
    e.vcall(base, 0x10, &args![found]);
    e.vcall(base, 0x18, &args![found]);
    let count = e.mem.u32(base + 0xc).wrapping_sub(1);
    e.mem.set_u32(base + 0xc, count);
    true
}

/// The two halves of `edge` around `middle` (`006bf4d0` from the edge's first end to
/// `middle`, then from `middle` to its second end, both with `kind`).
fn split_edge_halves(e: &mut Engine, this: Ptr, edge: u32, middle: u32, kind: u32) -> (u32, u32) {
    let from = e.mem.u32(edge);
    let to = e.mem.u32(edge + 4);
    let first = e.call(NEW_EDGE, &args![this, from, middle, kind]).u32();
    let second = e.call(NEW_EDGE, &args![this, middle, to, kind]).u32();
    (first, second)
}

// Translated from 006bd540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshObstacleCutter::SplitEdges` (Xbox PDB; `ECX` the cutter, plain `RET`): splits
/// each kind-1 edge (the obstacle's) where it crosses a kind-3 or kind-4 edge (the cut's
/// border) that shares no vertex with it. The crossing point comes from `006ba100` (cdecl:
/// the border edge's two positions, the edge's two positions, the stack point, prepared by
/// `006815c0`). When it lies within 0.0001 (`0101_41cc`, as a `float`) of one of the four
/// ends (the border edge's first, then second, then the edge's first, then second, each
/// distance the length `00595c80` of `00439ef0`'s difference), that vertex is used and only
/// the other edge is split; otherwise a new vertex is made (`00690fe0` on the navmesh with a
/// copy of the point in an `006240d0` record, `006bf3c0` with 1, 0, 1, appended to +0x1c)
/// and both are split. A split edge's halves (`006bf4d0`) are marked at +0x2e and take its
/// +0x16 word, its +0x18/+0x19 flags on the matching half, its +0x1c object (`006e5cc0`)
/// and, for the border edge, its +0x20 data (`006bc870`) and +0x2c word. The split edges
/// are released and removed (`006bf8f0`, the later index first) with the outer index
/// following, the halves appended to +0x30, and the search moves to the next obstacle edge.
pub fn nav_mesh_obstacle_cutter_split_edges(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let edges = base + 0x30;
    e.with_stack(0x60, |e, locals| {
        let point = locals.addr();
        let difference = point + 0xc;
        let record = point + 0x18;
        let chosen_slot = point + 0x24;
        let halves = point + 0x28;
        let mut index = 0u32;
        while index < array_count(e, edges) {
            let edge = array_word(e, edges, index);
            if e.mem.u32(edge + 0x10) != 1 {
                index = index.wrapping_add(1);
                continue;
            }
            let (own_first, own_second) = (e.mem.u32(edge), e.mem.u32(edge + 4));
            let own_first_position = vertex_position(e, this, own_first);
            let own_second_position = vertex_position(e, this, own_second);
            let mut other_index = 0u32;
            while other_index < array_count(e, edges) {
                let other = array_word(e, edges, other_index);
                let other_kind = e.mem.u32(other + 0x10);
                if other_kind != 3 && other_kind != 4 {
                    other_index += 1;
                    continue;
                }
                let (other_first, other_second) = (e.mem.u32(other), e.mem.u32(other + 4));
                let other_first_position = vertex_position(e, this, other_first);
                let other_second_position = vertex_position(e, this, other_second);
                if other_first == own_first
                    || other_second == own_first
                    || other_first == own_second
                    || other_second == own_second
                {
                    other_index += 1;
                    continue;
                }
                e.call(0x0068_15c0, &args![point]);
                let crosses = e
                    .call(
                        0x006b_a100,
                        &args![
                            other_first_position,
                            other_second_position,
                            own_first_position,
                            own_second_position,
                            point
                        ],
                    )
                    .bool();
                if !crosses {
                    other_index += 1;
                    continue;
                }
                // `FLD`/`FSTP` of the `float` 0.0001 into a local, loaded again for each test.
                let epsilon_bits = x87_quiet_bits(e.global::<u32>(0x0101_41cc));
                let epsilon = x87_load32(epsilon_bits);
                let near = |e: &mut Engine, position: u32| {
                    let vector = e
                        .call(0x0043_9ef0, &args![position, difference, point])
                        .u32();
                    let length = e.call(0x0059_5c80, &args![vector]).f64();
                    // `FLD epsilon; FCOMPP; TEST AH,0x41; JNZ`: near only when the
                    // epsilon is greater (not equal, not unordered).
                    epsilon > length
                };
                let (chosen, split_own, split_other) = if near(e, other_first_position) {
                    (other_first, true, false)
                } else if near(e, other_second_position) {
                    (other_second, true, false)
                } else if near(e, own_first_position) {
                    (own_first, false, true)
                } else if near(e, own_second_position) {
                    (own_second, false, true)
                } else {
                    e.call(0x0062_40d0, &args![record]);
                    for word in 0..3u32 {
                        let value = e.mem.u32(point + 4 * word);
                        e.mem.set_u32(record + 4 * word, value);
                    }
                    let navmesh = e.mem.u32(base);
                    let vertex_index =
                        u32::from(e.call(0x0069_0fe0, &args![navmesh, record]).u16());
                    let vertex = e
                        .call(NEW_VERTEX, &args![this, vertex_index, 1u32, 0u32, 1u32])
                        .u32();
                    e.mem.set_u32(chosen_slot, vertex);
                    e.call(ARRAY_APPEND, &args![base + 0x1c, chosen_slot]);
                    (e.mem.u32(chosen_slot), true, true)
                };
                let mut own_halves = (0u32, 0u32);
                let mut other_halves = (0u32, 0u32);
                if split_own {
                    own_halves = split_edge_halves(e, this, edge, chosen, 1);
                    let (first, second) = own_halves;
                    e.mem.set_u8(first + 0x2e, 1);
                    e.mem.set_u8(second + 0x2e, 1);
                    let word = e.mem.u16(edge + 0x16);
                    e.mem.set_u16(first + 0x16, word);
                    e.mem.set_u16(second + 0x16, word);
                    if e.mem.u8(edge + 0x18) != 0 {
                        e.mem.set_u8(first + 0x18, 1);
                    }
                    if e.mem.u8(edge + 0x19) != 0 {
                        e.mem.set_u8(second + 0x19, 1);
                    }
                    e.call(0x006e_5cc0, &args![first + 0x1c, edge + 0x1c]);
                    e.call(0x006e_5cc0, &args![second + 0x1c, edge + 0x1c]);
                }
                if split_other {
                    let kind = e.mem.u32(other + 0x10);
                    let first = e
                        .call(NEW_EDGE, &args![this, other_first, chosen, kind])
                        .u32();
                    let kind = e.mem.u32(other + 0x10);
                    let second = e
                        .call(NEW_EDGE, &args![this, chosen, other_second, kind])
                        .u32();
                    other_halves = (first, second);
                    e.mem.set_u8(first + 0x2e, 1);
                    e.call(0x006b_c870, &args![first + 0x20, other + 0x20]);
                    let word = e.mem.u16(other + 0x2c);
                    e.mem.set_u16(first + 0x2c, word);
                    e.mem.set_u8(second + 0x2e, 1);
                    e.call(0x006b_c870, &args![second + 0x20, other + 0x20]);
                    let word = e.mem.u16(other + 0x2c);
                    e.mem.set_u16(second + 0x2c, word);
                    let word = e.mem.u16(other + 0x16);
                    e.mem.set_u16(first + 0x16, word);
                    e.mem.set_u16(second + 0x16, word);
                    if e.mem.u8(other + 0x18) != 0 {
                        e.mem.set_u8(first + 0x18, 1);
                    }
                    if e.mem.u8(other + 0x19) != 0 {
                        e.mem.set_u8(second + 0x19, 1);
                    }
                    e.call(0x006e_5cc0, &args![first + 0x1c, other + 0x1c]);
                    e.call(0x006e_5cc0, &args![second + 0x1c, other + 0x1c]);
                }
                let remove = |e: &mut Engine, at: u32| {
                    e.call(0x006b_f8f0, &args![edges, at, 1u32]);
                };
                if split_own && split_other {
                    release_edge_object(e, edges, index);
                    release_edge_object(e, edges, other_index);
                    if index > other_index {
                        remove(e, index);
                        remove(e, other_index);
                        index = index.wrapping_sub(2);
                    } else {
                        remove(e, other_index);
                        remove(e, index);
                        index = index.wrapping_sub(1);
                    }
                } else {
                    if split_own {
                        release_edge_object(e, edges, index);
                        remove(e, index);
                        index = index.wrapping_sub(1);
                    }
                    if split_other {
                        release_edge_object(e, edges, other_index);
                        remove(e, other_index);
                        if index > other_index {
                            index = index.wrapping_sub(1);
                        }
                    }
                }
                for (split, (first, second)) in
                    [(split_own, own_halves), (split_other, other_halves)]
                {
                    if split {
                        e.mem.set_u32(halves, first);
                        e.call(ARRAY_APPEND, &args![edges, halves]);
                        e.mem.set_u32(halves + 4, second);
                        e.call(ARRAY_APPEND, &args![edges, halves + 4]);
                    }
                }
                break;
            }
            index = index.wrapping_add(1);
        }
    });
}

// Translated from 006bbd30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006bbd30 - the exception-unwinding frame (FS:[0] chain, handler 00f031f7, state variable) is not translated.
/// `NavMeshObstacleCutter::SplitNavMeshFromObstacle` (Xbox PDB; `ECX` the cutter, three
/// stack words: the navmesh, the obstacle, the portal list, `RET 0xc`). Stores the navmesh,
/// the obstacle (`NiPointer` assignment `006e5cc0` at +4), empties the vertex array, releases
/// the edges' objects and empties the edge array (`008454f0` with 0), and makes the undo
/// data (0x70 bytes through `00aa13e0`, [`fn_006bc230`], at +8). `006987d0` on the navmesh
/// collects the triangles the obstacle touches (the obstacle, a point of three 5.0 floats
/// from `00416870`, the undo data's +0x20); with none (`0076b610`) the undo data is deleted
/// (vtable slot 0 with 1) and the answer is false. Otherwise the undo data gets kind 1
/// (+8), the obstacle (+0xc), the navmesh's `00990a60` count (+0x10) and triangle count
/// (+0x18); +0x44 gets [`fn_006bc940`]; the neighbours are saved ([`fn_006bca20`]), the
/// cut triangles flagged (`00691570`, 0x20), the border edges built ([`fn_006bcc60`]); each
/// cut triangle's obstacle data (`GetAt` on the navmesh's +0xe0 map) moves into the undo
/// data's +0x40 map (`SetAt` with an `NiPointer` copy, `00559a40`) and out of the navmesh's
/// ([`ni_t_map_base_obstacle_data_remove_at`]). `NavMesh::AddObstacleUndoData`,
/// [`nav_mesh_obstacle_cutter_split_edges`], vertices passing [`fn_006bf2a0`] get byte +6,
/// [`nav_mesh_obstacle_cutter_remove_invalid_edges`], [`fn_006bde50`], then
/// [`fn_006be120`]: when it adds no triangle, the navmesh's +0x28 and +0x38 lists drop the
/// saved counts (`00694890`/`006949d0` with 1), the undo data is deleted and the answer is
/// false. Otherwise [`fn_006bdf50`], [`fn_006be120`] again, [`fn_006be830`],
/// [`fn_006be950`] with the list, the undo data's added counts (+0x14, +0x1c), the levels
/// ([`fn_006be770`]), each new triangle mapped to the obstacle in the navmesh's +0xe0 map,
/// [`fn_006beb80`], [`fn_006bec20`], [`fn_006be660`], `00699510` on the navmesh with the
/// undo data, and true.
pub fn nav_mesh_obstacle_cutter_split_nav_mesh_from_obstacle(
    e: &mut Engine,
    this: Ptr,
    navmesh: Ptr,
    obstacle: Ptr,
    portals: Ptr,
) -> bool {
    let base = this.addr();
    e.mem.set_u32(base, navmesh.addr());
    e.call(0x006e_5cc0, &args![base + 4, obstacle]);
    e.call(ARRAY_RELEASE_008454F0, &args![base + 0x1c, 0u32]);
    let mut index = 0u32;
    while index < array_count(e, base + 0x30) {
        release_edge_object(e, base + 0x30, index);
        index += 1;
    }
    e.call(ARRAY_RELEASE_008454F0, &args![base + 0x30, 0u32]);
    let block = e.call(0x00aa_13e0, &args![0x70u32]).u32();
    let undo = if block == 0 {
        0
    } else {
        fn_006bc230(e, Ptr::new(block)).addr()
    };
    e.mem.set_u32(base + 8, undo);
    let size_bits = x87_quiet_bits(e.global::<u32>(0x0101_712c));
    let collected = e.with_stack(0xc, |e, point| {
        let undo = e.mem.u32(base + 8);
        let extent = e
            .call(0x0041_6870, &args![point, size_bits, size_bits, size_bits])
            .u32();
        let navmesh = e.mem.u32(base);
        e.call(0x0069_87d0, &args![navmesh, obstacle, extent, undo + 0x20]);
        let undo = e.mem.u32(base + 8);
        !e.call(0x0076_b610, &args![undo + 0x20]).bool()
    });
    let delete_undo = |e: &mut Engine| {
        let undo = e.mem.u32(base + 8);
        if undo != 0 {
            e.vcall(undo, 0, &args![1u32]);
        }
    };
    if !collected {
        delete_undo(e);
        return false;
    }
    let undo = e.mem.u32(base + 8);
    e.mem.set_u32(undo + 8, 1);
    e.call(0x006e_5cc0, &args![undo + 0xc, obstacle]);
    let navmesh = e.mem.u32(base);
    let first_count = e.call(0x0099_0a60, &args![navmesh]).u32();
    e.mem.set_u32(undo + 0x10, first_count);
    let navmesh = e.mem.u32(base);
    let triangle_count = e.call(0x0055_8200, &args![navmesh]).u32();
    let undo = e.mem.u32(base + 8);
    e.mem.set_u32(undo + 0x18, triangle_count);
    let level = fn_006bc940(e, this);
    e.mem.set_u32(base + 0x44, level);
    let undo = e.mem.u32(base + 8);
    fn_006bca20(e, this, Ptr::new(undo + 0x30));
    let mut index = 0u32;
    loop {
        let undo = e.mem.u32(base + 8);
        if index >= array_count(e, undo + 0x20) {
            break;
        }
        let undo = e.mem.u32(base + 8);
        let entry = e.call(TRIANGLE_INDEX_AT, &args![undo + 0x20, index]).u32();
        let tri = triangle(e, this, u32::from(e.mem.u16(entry)));
        e.call(0x0069_1570, &args![tri, 0x20u32]);
        index += 1;
    }
    fn_006bcc60(e, this);
    let mut index = 0u32;
    loop {
        let undo = e.mem.u32(base + 8);
        if index >= array_count(e, undo + 0x20) {
            break;
        }
        let undo = e.mem.u32(base + 8);
        let entry = e.call(TRIANGLE_INDEX_AT, &args![undo + 0x20, index]).u32();
        let key = u32::from(e.mem.u16(entry));
        e.with_stack(8, |e, slots| {
            let data = slots.addr();
            let argument = data + 4;
            e.call(0x0063_3c90, &args![data, 0u32]);
            let navmesh = e.mem.u32(base);
            let map = e.mem.u32(navmesh + 0xe0);
            if e.call(0x0069_afb0, &args![map, key, data]).bool() {
                e.call(0x0055_9a40, &args![argument, data]);
                let pointer = e.mem.u32(argument);
                let undo = e.mem.u32(base + 8);
                e.call(0x0069_46a0, &args![undo + 0x40, key, pointer]);
                let navmesh = e.mem.u32(base);
                let map = e.mem.u32(navmesh + 0xe0);
                ni_t_map_base_obstacle_data_remove_at(e, Ptr::new(map), key as u16);
            }
            e.call(0x0045_cec0, &args![data]);
        });
        index += 1;
    }
    e.call(0x006b_d040, &args![this]);
    nav_mesh_obstacle_cutter_split_edges(e, this);
    let mut index = 0u32;
    while index < array_count(e, base + 0x1c) {
        let vertex = array_word(e, base + 0x1c, index);
        if fn_006bf2a0(e, this, Ptr::new(vertex)) {
            let vertex = array_word(e, base + 0x1c, index);
            e.mem.set_u8(vertex + 6, 1);
        }
        index += 1;
    }
    nav_mesh_obstacle_cutter_remove_invalid_edges(e, this);
    fn_006bde50(e, this);
    if !fn_006be120(e, this) {
        let navmesh = e.mem.u32(base);
        let undo = e.mem.u32(base + 8);
        let saved = e.mem.u32(undo + 0x10);
        e.call(0x0069_4890, &args![navmesh + 0x28, saved, 1u32]);
        let navmesh = e.mem.u32(base);
        let undo = e.mem.u32(base + 8);
        let saved = e.mem.u32(undo + 0x18);
        e.call(0x0069_49d0, &args![navmesh + 0x38, saved, 1u32]);
        delete_undo(e);
        return false;
    }
    fn_006bdf50(e, this);
    fn_006be120(e, this);
    fn_006be830(e, this);
    fn_006be950(e, this, portals);
    let navmesh = e.mem.u32(base);
    let count = e.call(0x0099_0a60, &args![navmesh]).u32();
    let undo = e.mem.u32(base + 8);
    let added = count.wrapping_sub(e.mem.u32(undo + 0x10));
    e.mem.set_u32(undo + 0x14, added);
    let navmesh = e.mem.u32(base);
    let count = e.call(0x0055_8200, &args![navmesh]).u32();
    let undo = e.mem.u32(base + 8);
    let added = count.wrapping_sub(e.mem.u32(undo + 0x18));
    e.mem.set_u32(undo + 0x1c, added);
    fn_006be770(e, this);
    let undo = e.mem.u32(base + 8);
    let mut index = e.mem.u32(undo + 0x18);
    loop {
        let undo = e.mem.u32(base + 8);
        let end = e.mem.u32(undo + 0x18).wrapping_add(e.mem.u32(undo + 0x1c));
        if index >= end {
            break;
        }
        e.with_stack(4, |e, argument| {
            e.call(0x0055_9a40, &args![argument, base + 4]);
            let pointer = e.mem.u32(argument.addr());
            let navmesh = e.mem.u32(base);
            let map = e.mem.u32(navmesh + 0xe0);
            e.call(0x0069_46a0, &args![map, index & 0xffff, pointer]);
        });
        index += 1;
    }
    fn_006beb80(e, this);
    fn_006bec20(e, this);
    fn_006be660(e, this);
    let navmesh = e.mem.u32(base);
    let undo = e.mem.u32(base + 8);
    e.call(0x0069_9510, &args![navmesh, undo]);
    true
}

// Translated from 006bc3e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006bc3e0 - the exception-unwinding frame (FS:[0] chain, handler 00f03290, state variable) is not translated.
/// `NavMeshObstacleCutter::UndoNavMeshFromObstacleUndoData` (Xbox PDB; `ECX` the cutter,
/// three stack words: the navmesh, the undo data, the portal list, `RET 0xc`; answers true).
/// Stores the navmesh and the undo data; with a stack index array, each saved portal pair of
/// the undo data (+0x50, `006bf670`: the second half from the entry, the first from entry
/// +0xc, built by `006bc8b0`/`006bc870`, flag 0) is merged back into the list (see
/// `merge_portal_pair`) and the listed indices removed. The cut triangles get `00519270`
/// (0x20); the saved triangles (+0x30) get `00519270` (0x100) and each side's neighbour
/// back (`00690e30` for a border side, `006907d0` otherwise); the new triangles get
/// `00691570` (0x20); the saved triangles' levels drop by one (`00693bd0` with
/// `00699790` - 1); the new triangles leave the navmesh's +0xe0 obstacle map
/// ([`ni_t_map_base_obstacle_data_remove_at`]) and every entry of the undo data's +0x40 map
/// goes back into it (`004b9ba0`/`GetNext` `006947e0`, `SetAt` with an `NiPointer` copy).
/// Then [`fn_006bec20`], [`fn_006be6f0`] and the undo data's kind (+8) becomes 3.
pub fn nav_mesh_obstacle_cutter_undo_nav_mesh_from_obstacle_undo_data(
    e: &mut Engine,
    this: Ptr,
    navmesh: Ptr,
    undo: Ptr,
    portals: Ptr,
) -> bool {
    let base = this.addr();
    e.mem.set_u32(base, navmesh.addr());
    e.mem.set_u32(base + 8, undo.addr());
    let list = portals.addr();
    e.with_stack(0x40, |e, locals| {
        let first = locals.addr();
        let second = first + 0xc;
        let flag = first + 0x18;
        let slot = first + 0x1c;
        let indices = first + 0x20;
        e.call(0x0043_8660, &args![indices]);
        let mut index = 0u32;
        loop {
            let undo = e.mem.u32(base + 8);
            if index >= array_count(e, undo + 0x50) {
                break;
            }
            e.call(0x006b_c8b0, &args![first]);
            let undo = e.mem.u32(base + 8);
            let entry = e.call(0x006b_f670, &args![undo + 0x50, index]).u32();
            e.call(0x006b_c870, &args![second, entry]);
            let undo = e.mem.u32(base + 8);
            let entry = e.call(0x006b_f670, &args![undo + 0x50, index]).u32();
            e.call(0x006b_c870, &args![first, entry + 0xc]);
            e.mem.set_u8(flag, 0);
            merge_portal_pair(e, list, first, second, indices, slot);
            index += 1;
        }
        remove_listed_portals(e, list, indices);
        let mut index = 0u32;
        loop {
            let undo = e.mem.u32(base + 8);
            if index >= array_count(e, undo + 0x20) {
                break;
            }
            let undo = e.mem.u32(base + 8);
            let entry = e.call(TRIANGLE_INDEX_AT, &args![undo + 0x20, index]).u32();
            let tri = triangle(e, this, u32::from(e.mem.u16(entry)));
            e.call(0x0051_9270, &args![tri, 0x20u32]);
            index += 1;
        }
        let mut index = 0u32;
        loop {
            let undo = e.mem.u32(base + 8);
            if index >= array_count(e, undo + 0x30) {
                break;
            }
            let undo = e.mem.u32(base + 8);
            let entry = e.call(SAVED_TRIANGLE_AT, &args![undo + 0x30, index]).u32();
            let tri = triangle(e, this, u32::from(e.mem.u16(entry)));
            e.call(0x0051_9270, &args![tri, 0x100u32]);
            let mut side = 0i32;
            while side < 3 {
                let k = side as u32;
                let undo = e.mem.u32(base + 8);
                let entry = e.call(SAVED_TRIANGLE_AT, &args![undo + 0x30, index]).u32();
                let border = e.mem.u8(entry + 2 + k) != 0;
                let undo = e.mem.u32(base + 8);
                let entry = e.call(SAVED_TRIANGLE_AT, &args![undo + 0x30, index]).u32();
                let neighbour = u32::from(e.mem.u16(entry + 6 + k * 2));
                let restore = if border { 0x0069_0e30 } else { 0x0069_07d0 };
                e.call(restore, &args![tri, k, neighbour]);
                side += 1;
            }
            index += 1;
        }
        let undo = e.mem.u32(base + 8);
        let mut index = e.mem.u32(undo + 0x18);
        loop {
            let undo = e.mem.u32(base + 8);
            let end = e.mem.u32(undo + 0x18).wrapping_add(e.mem.u32(undo + 0x1c));
            if index >= end {
                break;
            }
            let tri = triangle(e, this, index);
            e.call(0x0069_1570, &args![tri, 0x20u32]);
            index += 1;
        }
        let mut index = 0u32;
        loop {
            let undo = e.mem.u32(base + 8);
            if index >= array_count(e, undo + 0x30) {
                break;
            }
            let undo = e.mem.u32(base + 8);
            let entry = e.call(SAVED_TRIANGLE_AT, &args![undo + 0x30, index]).u32();
            let tri = triangle(e, this, u32::from(e.mem.u16(entry)));
            let level = u32::from(e.call(0x0069_9790, &args![tri]).u16()).wrapping_sub(1);
            e.call(0x0069_3bd0, &args![tri, level]);
            index += 1;
        }
        let undo = e.mem.u32(base + 8);
        let mut index = e.mem.u32(undo + 0x18);
        loop {
            let undo = e.mem.u32(base + 8);
            let end = e.mem.u32(undo + 0x18).wrapping_add(e.mem.u32(undo + 0x1c));
            if index >= end {
                break;
            }
            let navmesh = e.mem.u32(base);
            let map = e.mem.u32(navmesh + 0xe0);
            ni_t_map_base_obstacle_data_remove_at(e, Ptr::new(map), index as u16);
            index += 1;
        }
        e.with_stack(0x10, |e, iteration| {
            let position = iteration.addr();
            let key = position + 4;
            let data = position + 8;
            let argument = position + 0xc;
            let undo = e.mem.u32(base + 8);
            let start = e.call(0x004b_9ba0, &args![undo + 0x40]).u32();
            e.mem.set_u32(position, start);
            while e.mem.u32(position) != 0 {
                e.call(0x0063_3c90, &args![data, 0u32]);
                let undo = e.mem.u32(base + 8);
                e.call(0x0069_47e0, &args![undo + 0x40, position, key, data]);
                e.call(0x0055_9a40, &args![argument, data]);
                let pointer = e.mem.u32(argument);
                let navmesh = e.mem.u32(base);
                let map = e.mem.u32(navmesh + 0xe0);
                let key_word = u32::from(e.mem.u16(key));
                e.call(0x0069_46a0, &args![map, key_word, pointer]);
                e.call(0x0045_cec0, &args![data]);
            }
        });
        fn_006bec20(e, this);
        fn_006be6f0(e, this);
        let undo = e.mem.u32(base + 8);
        e.mem.set_u32(undo + 8, 3);
        e.call(0x0043_8690, &args![indices]);
    });
    true
}

/// `MemoryManager` deallocation (`00401030`, cdecl, the block).
const DEALLOCATE: u32 = 0x0040_1030;
/// `006bf870` (`unplaced/run_006bf870.rs`): the
/// `BSSimpleArray<NavMeshObstacleVertex*>` destructor body.
const VERTEX_SIMPLE_ARRAY_DESTRUCT: u32 = 0x006b_f870;
/// `006bfab0` (`unplaced/run_006bfab0.rs`): the
/// `BSSimpleArray<NavMeshObstacleEdge*>` destructor body.
const EDGE_SIMPLE_ARRAY_DESTRUCT: u32 = 0x006b_fab0;
/// `006bc870` (`fallout shared/pathfinding/run_006bc870.rs`; `RET 4`):
/// copies the word at +0, the half-word at +4 and the word at +8 of its
/// argument into `this`; returns `this`.
const COPY_EDGE_HANDLE: u32 = 0x006b_c870;

// Translated from 006bfbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<BoundsTriangleInfo,1024>` scalar deleting destructor
/// (engine map, from the vtable; `RET 4`): the destructor body `006bf6c0`,
/// then `00401030(this)` when bit 0 of `flags` is set. Returns `this`.
pub fn bs_simple_array_bounds_triangle_info_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_006bf6c0(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 006bfc10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<PortalEdgeSwap,1024>` scalar deleting destructor (engine
/// map, from the vtable; `RET 4`): the destructor body `006bf710`, then
/// `00401030(this)` when bit 0 of `flags` is set. Returns `this`.
pub fn bs_simple_array_portal_edge_swap_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_006bf710(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 006bfc40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`navmeshobstaclecutter.cpp`; `thiscall`,
/// `RET 4`; called by `006bf730`): copies `other` into `this` member by
/// member: the 0xC-byte part at +0 and the one at +0xC through `006bc870`,
/// then the byte at +0x18. Returns `this`.
pub fn fn_006bfc40(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    e.call(COPY_EDGE_HANDLE, &args![this, other]);
    e.call(
        COPY_EDGE_HANDLE,
        &args![
            this.addr().wrapping_add(0xc),
            other.addr().wrapping_add(0xc)
        ],
    );
    let byte = e.mem.u8(other.addr().wrapping_add(0x18));
    e.mem.set_u8(this.addr().wrapping_add(0x18), byte);
    this
}

// Translated from 006bfc80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NavMeshObstacleVertex *,1024>` scalar deleting destructor
/// (engine map, from the vtable; `RET 4`): the destructor body `006bf870`,
/// then `00401030(this)` when bit 0 of `flags` is set. Returns `this`.
pub fn bs_simple_array_nav_mesh_obstacle_vertex_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(VERTEX_SIMPLE_ARRAY_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 006bfcb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapArray<NavMeshObstacleVertex *,1024>` scalar deleting destructor
/// (engine map, from the vtable; `RET 4`): the destructor body `006bf890`,
/// then `00401030(this)` when bit 0 of `flags` is set. Returns `this`.
pub fn bs_scrap_array_nav_mesh_obstacle_vertex_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_006bf890(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 006bfce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NavMeshObstacleEdge *,1024>` scalar deleting destructor
/// (engine map, from the vtable; `RET 4`): the destructor body `006bfab0`,
/// then `00401030(this)` when bit 0 of `flags` is set. Returns `this`.
pub fn bs_simple_array_nav_mesh_obstacle_edge_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(EDGE_SIMPLE_ARRAY_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 006bfd10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapArray<NavMeshObstacleEdge *,1024>` scalar deleting destructor
/// (engine map, from the vtable; `RET 4`): the destructor body `006bfad0`,
/// then `00401030(this)` when bit 0 of `flags` is set. Returns `this`.
pub fn bs_scrap_array_nav_mesh_obstacle_edge_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_006bfad0(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006b_fbe0, bs_simple_array_bounds_triangle_info_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x006b_fc10, bs_simple_array_portal_edge_swap_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x006b_fc40, fn_006bfc40(Ptr, Ptr) -> Ptr),
        entry!(0x006b_fc80, bs_simple_array_nav_mesh_obstacle_vertex_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x006b_fcb0, bs_scrap_array_nav_mesh_obstacle_vertex_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x006b_fce0, bs_simple_array_nav_mesh_obstacle_edge_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x006b_fd10, bs_scrap_array_nav_mesh_obstacle_edge_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x006b_c300, obstacle_undo_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x006b_f6c0, fn_006bf6c0(Ptr)),
        entry!(0x006b_f710, fn_006bf710(Ptr)),
        entry!(0x006b_bc30, fn_006bbc30(Ptr) -> Ptr),
        entry!(0x006b_f7d0, fn_006bf7d0(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x006b_fa10, fn_006bfa10(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x006b_f890, fn_006bf890(Ptr)),
        entry!(0x006b_fad0, fn_006bfad0(Ptr)),
        entry!(0x006b_f690, fn_006bf690(Ptr, u32, u32) -> Ptr),
        entry!(0x006b_f6e0, fn_006bf6e0(Ptr) -> Ptr),
        entry!(0x006b_bb80, nav_mesh_obstacle_cutter_nav_mesh_obstacle_cutter(Ptr) -> Ptr),
        entry!(0x006b_bc50, nav_mesh_obstacle_cutter_destructor(Ptr)),
        entry!(0x006b_c230, fn_006bc230(Ptr) -> Ptr),
        entry!(0x006b_c330, fn_006bc330(Ptr)),
        entry!(0x006b_f340, fn_006bf340(Ptr, Ptr) -> bool),
        entry!(0x006b_f2a0, fn_006bf2a0(Ptr, Ptr) -> bool),
        entry!(0x006b_ec20, fn_006bec20(Ptr)),
        entry!(0x006b_eb80, fn_006beb80(Ptr)),
        entry!(0x006b_e660, fn_006be660(Ptr)),
        entry!(0x006b_e6f0, fn_006be6f0(Ptr)),
        entry!(0x006b_e770, fn_006be770(Ptr)),
        entry!(0x006b_e830, fn_006be830(Ptr)),
        entry!(0x006b_c940, fn_006bc940(Ptr) -> u32),
        entry!(0x006b_de50, fn_006bde50(Ptr)),
        entry!(
            0x006b_dcb0,
            nav_mesh_obstacle_cutter_remove_invalid_edges(Ptr)
        ),
        entry!(0x006b_df50, fn_006bdf50(Ptr)),
        entry!(0x006b_ca20, fn_006bca20(Ptr, Ptr)),
        entry!(0x006b_cc60, fn_006bcc60(Ptr)),
        entry!(0x006b_e950, fn_006be950(Ptr, Ptr)),
        entry!(0x006b_e120, fn_006be120(Ptr) -> bool),
        entry!(
            0x0061_bf70,
            bs_simple_array_nav_mesh_edge_handle_remove(Ptr, u32, u8)
        ),
        entry!(0x0049_c250, ni_t_map_base_obstacle_data_remove_at(Ptr, u16) -> bool),
        entry!(0x006b_d540, nav_mesh_obstacle_cutter_split_edges(Ptr)),
        entry!(0x006b_bd30, nav_mesh_obstacle_cutter_split_nav_mesh_from_obstacle(Ptr, Ptr, Ptr, Ptr) -> bool),
        entry!(0x006b_c3e0, nav_mesh_obstacle_cutter_undo_nav_mesh_from_obstacle_undo_data(Ptr, Ptr, Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_deleting_destructor(address: u32, body: u32, size: u32) {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(e.mem.alloc(size));
        e.register(body, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        for flags in [0u32, 1, 2, 3] {
            assert_eq!(e.call(address, &args![this, flags]).u32(), this.addr());
        }
        let log = e.call_log.take().unwrap();
        let calls_to = |target: u32| {
            log.iter()
                .filter(|(x, _)| *x == target)
                .cloned()
                .collect::<Vec<_>>()
        };
        assert_eq!(calls_to(body).len(), 4);
        assert_eq!(
            calls_to(0x00aa_1460),
            vec![(0x00aa_1460, vec![this.addr(), size]); 2]
        );
    }

    #[test]
    fn obstacle_undo_data_scalar_deleting_destructor_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x006b_c300, 0x006b_c330, 0x70);
    }
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
    fn fn_006bf6c0_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x006b_f6c0, ARRAY_VTABLE_0106C2E8, ARRAY_RELEASE_008454F0);
    }

    #[test]
    fn fn_006bf710_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x006b_f710, ARRAY_VTABLE_0106C2FC, ARRAY_RELEASE_008454F0);
    }
}

#[cfg(test)]
mod cutter_tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls_to(log: &[(u32, Vec<u32>)], address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|call| call.0 == address)
            .map(|call| call.1.clone())
            .collect()
    }

    fn array_words(e: &Engine, array: u32) -> Vec<u32> {
        let buffer = e.mem.u32(array + 4);
        (0..e.mem.u32(array + 8))
            .map(|index| e.mem.u32(buffer + 4 * index))
            .collect()
    }

    /// Doubles for the array accessors over the test arrays: words (stride 4), 16-bit
    /// triangle indices (stride 2), saved triangles (stride 0xc), portal pairs (stride 0x18).
    fn array_doubles(e: &mut Engine) {
        e.register(ARRAY_COUNT, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(ARRAY_ELEMENT, |e, a| ret(e.mem.u32(a[0] + 4) + 4 * a[1]));
        e.register(
            TRIANGLE_INDEX_AT,
            |e, a| ret(e.mem.u32(a[0] + 4) + 2 * a[1]),
        );
        e.register(SAVED_TRIANGLE_AT, |e, a| {
            ret(e.mem.u32(a[0] + 4) + 0xc * a[1])
        });
        e.register(0x006b_f670, |e, a| ret(e.mem.u32(a[0] + 4) + 0x18 * a[1]));
        e.register(ARRAY_APPEND, |e, a| {
            let count = e.mem.u32(a[0] + 8);
            let buffer = e.mem.u32(a[0] + 4);
            let word = e.mem.u32(a[1]);
            e.mem.set_u32(buffer + 4 * count, word);
            e.mem.set_u32(a[0] + 8, count + 1);
            Ret::default()
        });
        // Removing words (`006bf8f0` and `009a4320`: the index and the count).
        for remove in [0x006b_f8f0u32, 0x009a_4320] {
            e.register(remove, |e, a| {
                let count = e.mem.u32(a[0] + 8);
                let buffer = e.mem.u32(a[0] + 4);
                for index in a[1]..count - a[2] {
                    let word = e.mem.u32(buffer + 4 * (index + a[2]));
                    e.mem.set_u32(buffer + 4 * index, word);
                }
                e.mem.set_u32(a[0] + 8, count - a[2]);
                Ret::default()
            });
        }
    }

    /// A vertex: 16-bit index at +0, flag bytes +2..+6 as given.
    fn vertex(e: &mut Engine, index: u16, flags: [u8; 5]) -> u32 {
        let vertex = e.mem.alloc(0x10);
        e.mem.set_u16(vertex, index);
        for (offset, flag) in flags.iter().enumerate() {
            e.mem.set_u8(vertex + 2 + offset as u32, *flag);
        }
        vertex
    }

    /// An edge between two vertices with a kind.
    fn edge(e: &mut Engine, from: u32, to: u32, kind: u32) -> u32 {
        let edge = e.mem.alloc(0x30);
        e.mem.set_u32(edge, from);
        e.mem.set_u32(edge + 4, to);
        e.mem.set_u32(edge + 0x10, kind);
        e.mem.set_u16(edge + 8, 0xffff);
        edge
    }

    /// A cutter (0x48 bytes) with the navmesh `0x0700_0000`, its undo data (0x70 bytes, the
    /// arrays at +0x20/+0x30/+0x50/+0x60 empty) and empty vertex/edge arrays. The undo data
    /// is returned too.
    fn cutter(e: &mut Engine) -> (u32, u32) {
        let this = e.mem.alloc(0x48);
        e.map(0x0700_0000, 0x100);
        e.mem.set_u32(this, 0x0700_0000);
        let undo = e.mem.alloc(0x70);
        e.mem.set_u32(this + 8, undo);
        for offset in [0x20u32, 0x30, 0x50, 0x60] {
            let buffer = e.mem.alloc(0x400);
            e.mem.set_u32(undo + offset + 4, buffer);
        }
        for offset in [0x1cu32, 0x30] {
            let buffer = e.mem.alloc(0x400);
            e.mem.set_u32(this + offset + 4, buffer);
        }
        array_doubles(e);
        (this, undo)
    }

    fn set_words(e: &mut Engine, array: u32, words: &[u32]) {
        let buffer = e.mem.u32(array + 4);
        for (index, word) in words.iter().enumerate() {
            e.mem.set_u32(buffer + 4 * index as u32, *word);
        }
        e.mem.set_u32(array + 8, words.len() as u32);
    }

    fn set_indices(e: &mut Engine, array: u32, indices: &[u16]) {
        let buffer = e.mem.u32(array + 4);
        for (position, index) in indices.iter().enumerate() {
            e.mem.set_u16(buffer + 2 * position as u32, *index);
        }
        e.mem.set_u32(array + 8, indices.len() as u32);
    }

    /// The triangle double: 005582f0 gives `0x0710_0000 + 0x100 * index`.
    fn triangle_doubles(e: &mut Engine) {
        e.register(NAVMESH_TRIANGLE, |_, a| {
            ret(0x0710_0000 + 0x100 * (a[1] & 0xffff))
        });
    }

    #[test]
    fn scrap_heap_and_array_constructors() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x14);
        for address in [
            0x00aa_53f0u32,
            0x006b_ff00,
            0x006b_ff30,
            0x006b_3eb0,
            0x006b_3f40,
            0x006b_fe70,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(MEMORY_MANAGER, |_, _| ret(0x0880_0000));
        e.register(GET_THREAD_SCRAP_HEAP, |_, _| ret(0x0880_1000));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006b_bc30, &args![object]).u32(), object);
        assert_eq!(
            e.call(0x006b_f7d0, &args![object, 0x100u32, 0u32, 0x5000u32])
                .u32(),
            object
        );
        assert_eq!(e.mem.u32(object), VERTEX_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(object + 0x10), 0x5000);
        e.call(0x006b_fa10, &args![object, 0x20u32, 3u32, 0u32]);
        assert_eq!(e.mem.u32(object), EDGE_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(object + 0x10), 0x0880_1000);
        e.call(0x006b_f690, &args![object, 0x20u32, 0u32]);
        assert_eq!(e.mem.u32(object), ARRAY_VTABLE_0106C2E8);
        e.call(0x006b_f6e0, &args![object]);
        assert_eq!(e.mem.u32(object), ARRAY_VTABLE_0106C2FC);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x00aa_53f0), vec![vec![object]]);
        assert_eq!(calls_to(&log, 0x006b_ff00), vec![vec![object]]);
        assert_eq!(calls_to(&log, 0x006b_ff30), vec![vec![object]]);
        assert_eq!(
            calls_to(&log, 0x006b_3eb0),
            vec![vec![object, 0x100, 0], vec![object, 0x20, 3]]
        );
        assert_eq!(
            calls_to(&log, GET_THREAD_SCRAP_HEAP),
            vec![vec![0x0880_0000]]
        );
        assert_eq!(calls_to(&log, 0x006b_3f40), vec![vec![object, 0x20, 0]]);
        assert_eq!(calls_to(&log, 0x006b_fe70), vec![vec![object, 0, 0]]);
    }

    #[test]
    fn scrap_array_destructors_release_then_run_the_base() {
        for (address, vtable, base) in [
            (0x006b_f890u32, VERTEX_ARRAY_VTABLE, 0x006b_f870u32),
            (0x006b_fad0, EDGE_ARRAY_VTABLE, 0x006b_fab0),
        ] {
            let mut e = Engine::new();
            let object = e.mem.alloc(0x14);
            e.register(ARRAY_RELEASE_008454F0, |_, _| Ret::default());
            e.register(base, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            e.call(address, &args![object]);
            assert_eq!(e.mem.u32(object), vtable);
            let log = e.call_log.take().unwrap();
            assert_eq!(
                log[1..],
                [
                    (ARRAY_RELEASE_008454F0, vec![object, 1]),
                    (base, vec![object])
                ]
            );
        }
    }

    #[test]
    fn cutter_constructor_builds_its_members() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x48);
        for address in [
            0x0063_3c90u32,
            0x00aa_53f0,
            0x006b_ff00,
            0x006b_ff30,
            0x006b_3eb0,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(0x0068_15c0, |_, a| ret(a[0]));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006b_bb80, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0063_3c90), vec![vec![this + 4, 0]]);
        assert_eq!(calls_to(&log, 0x00aa_53f0), vec![vec![this + 0xc]]);
        assert_eq!(e.mem.u32(this + 0x1c), VERTEX_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(this + 0x30), EDGE_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(this + 0x1c + 0x10), this + 0xc);
        assert_eq!(e.mem.u32(this + 0x30 + 0x10), this + 0xc);
        assert_eq!(
            calls_to(&log, 0x006b_3eb0),
            vec![vec![this + 0x1c, 0x100, 0], vec![this + 0x30, 0x100, 0]]
        );
    }

    #[test]
    fn cutter_destructor_releases_edge_objects_and_members() {
        let mut e = Engine::new();
        let (this, _) = cutter(&mut e);
        let with_object = edge(&mut e, 0, 0, 1);
        e.mem.set_u32(with_object + 0x1c, 0x0900_0000);
        let without = edge(&mut e, 0, 0, 1);
        set_words(&mut e, this + 0x30, &[with_object, without]);
        e.register(NI_POINTER_GET, |e, a| ret(e.mem.u32(a[0])));
        for address in [
            RELEASE,
            ARRAY_RELEASE_008454F0,
            0x006b_fab0,
            0x006b_f870,
            0x0098_2610,
            0x0045_cec0,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x006b_bc50, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, RELEASE), vec![vec![0x0900_0000]]);
        assert_eq!(e.mem.u32(this + 0x30), EDGE_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(this + 0x1c), VERTEX_ARRAY_VTABLE);
        assert_eq!(calls_to(&log, 0x0098_2610), vec![vec![this + 0xc]]);
        assert_eq!(log.last().unwrap(), &(0x0045_cec0, vec![this + 4]));
    }

    #[test]
    fn undo_data_constructor_and_destructor() {
        let mut e = Engine::new();
        let undo = e.mem.alloc(0x70);
        e.mem.set_u32(undo + 8, 0x77);
        for address in [
            0x0049_68b0u32,
            0x0063_3c90,
            0x0069_ada0,
            0x006b_3f40,
            0x0069_16f0,
            0x006b_fe70,
            0x0069_17d0,
            0x0069_1800,
            ARRAY_RELEASE_008454F0,
            0x0069_19a0,
            0x0045_cec0,
            0x0049_6910,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006b_c230, &args![undo]).u32(), undo);
        assert_eq!(e.mem.u32(undo), UNDO_DATA_VTABLE);
        assert_eq!(e.mem.u32(undo + 8), 0);
        assert_eq!(e.mem.u32(undo + 0x30), ARRAY_VTABLE_0106C2E8);
        assert_eq!(e.mem.u32(undo + 0x50), ARRAY_VTABLE_0106C2FC);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x0069_ada0),
            vec![vec![undo + 0x20, 0x20, 0]]
        );
        assert_eq!(calls_to(&log, 0x0069_16f0), vec![vec![undo + 0x40, 0x25]]);
        assert_eq!(calls_to(&log, 0x0069_17d0), vec![vec![undo + 0x60]]);
        e.call_log = Some(vec![]);
        e.call(0x006b_c330, &args![undo]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x0069_1800),
            vec![vec![undo + 0x60], vec![undo + 0x20]]
        );
        assert_eq!(calls_to(&log, 0x0069_19a0), vec![vec![undo + 0x40]]);
        assert_eq!(calls_to(&log, ARRAY_RELEASE_008454F0).len(), 2);
        assert_eq!(log.last().unwrap(), &(0x0049_6910, vec![undo]));
    }

    #[test]
    fn fn_006bf340_decides_which_edges_to_drop() {
        let mut e = Engine::new();
        // (first vertex bytes +2/+6, second vertex bytes +2/+6, kind, dropped)
        for (first, second, kind, dropped) in [
            ((0u8, 1u8), (1u8, 0u8), 1u32, true),
            ((1, 1), (0, 1), 1, true),
            ((1, 0), (1, 0), 2, true),
            ((1, 0), (1, 0), 1, false),
            ((0, 0), (1, 0), 2, false),
            ((1, 0), (0, 0), 2, false),
        ] {
            let a = vertex(&mut e, 0, [first.0, 0, 0, 0, first.1]);
            let b = vertex(&mut e, 1, [second.0, 0, 0, 0, second.1]);
            let edge = edge(&mut e, a, b, kind);
            assert_eq!(e.call(0x006b_f340, &args![0u32, edge]).bool(), dropped);
        }
    }

    #[test]
    fn fn_006bf2a0_asks_with_the_vertex_position() {
        let mut e = Engine::new();
        let (this, _) = cutter(&mut e);
        let v = vertex(&mut e, 7, [0; 5]);
        e.register(NAVMESH_VERTEX, |_, a| ret(0x0800_0000 + a[1]));
        e.register(0x006b_f2d0, |_, _| ret(1));
        e.call_log = Some(vec![]);
        assert!(e.call(0x006b_f2a0, &args![this, v]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, NAVMESH_VERTEX), vec![vec![0x0700_0000, 7]]);
        assert_eq!(calls_to(&log, 0x006b_f2d0), vec![vec![this, 0x0800_0007]]);
    }

    #[test]
    fn fn_006bec20_and_fn_006beb80_touch_the_navmesh() {
        let mut e = Engine::new();
        let (this, undo) = cutter(&mut e);
        e.register(0x0069_10a0, |_, _| Ret::default());
        e.register(0x0069_65d0, |_, _| Ret::default());
        e.mem.set_u32(undo + 0x18, 0x0002_0010);
        e.call_log = Some(vec![]);
        e.call(0x006b_ec20, &args![this]);
        e.call(0x006b_eb80, &args![this]);
        e.mem.set_u32(undo + 0x1c, 3);
        e.call(0x006b_eb80, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0069_10a0), vec![vec![0x0700_0000]]);
        // The first index as a 16-bit word, the last as a whole word.
        assert_eq!(
            calls_to(&log, 0x0069_65d0),
            vec![vec![0x0700_0000, 0x10, 0x0002_0012]]
        );
    }

    #[test]
    fn fn_006be660_records_flagged_vertices() {
        let mut e = Engine::new();
        let (this, undo) = cutter(&mut e);
        let flagged = vertex(&mut e, 1, [1, 0, 0, 0, 0]);
        let plain = vertex(&mut e, 2, [0; 5]);
        set_words(&mut e, this + 0x1c, &[plain, flagged]);
        e.register(0x0069_1820, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x006b_e660, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x0069_1820),
            vec![vec![0x0700_00e4, flagged], vec![undo + 0x60, flagged]]
        );
    }

    #[test]
    fn fn_006be6f0_removes_found_entries() {
        let mut e = Engine::new();
        let (this, undo) = cutter(&mut e);
        set_indices(&mut e, undo + 0x60, &[4, 5]);
        let first_entry = e.mem.u32(undo + 0x64);
        e.register_double(0x0069_b240, move |_, a| {
            ret(if a[1] == first_entry { 9 } else { 0xffff_ffff })
        });
        e.register(0x0069_add0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x006b_e6f0, &args![this]);
        let log = e.call_log.take().unwrap();
        let searches = calls_to(&log, 0x0069_b240);
        assert_eq!(searches.len(), 2);
        assert_eq!(
            searches[0],
            vec![0x0700_00e4, first_entry, 0, TRIANGLE_COMPARE]
        );
        assert_eq!(calls_to(&log, 0x0069_add0), vec![vec![0x0700_00e4, 9, 1]]);
    }

    #[test]
    fn fn_006be770_sets_the_levels() {
        let mut e = Engine::new();
        let (this, undo) = cutter(&mut e);
        triangle_doubles(&mut e);
        e.mem.set_u32(this + 0x44, 6);
        e.mem.set_u32(undo + 0x18, 0x0001_0003);
        e.mem.set_u32(undo + 0x1c, 2);
        // One saved triangle, index 8.
        let saved = e.mem.u32(undo + 0x34);
        e.mem.set_u16(saved, 8);
        e.mem.set_u32(undo + 0x38, 1);
        e.register(0x0069_9790, |_, _| ret(0xffff_0004));
        e.register(0x0069_3bd0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x006b_e770, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x0069_3bd0),
            vec![
                vec![0x0710_0300, 7],
                vec![0x0710_0400, 7],
                vec![0x0710_0800, 5],
            ]
        );
    }

    #[test]
    fn fn_006be830_links_triangles_across_edges() {
        let mut e = Engine::new();
        let (this, _) = cutter(&mut e);
        triangle_doubles(&mut e);
        let both = edge(&mut e, 0, 0, 1);
        e.mem.set_u16(both + 8, 2);
        e.mem.set_u16(both + 0xa, 3);
        e.mem.set_u16(both + 0xc, 1);
        e.mem.set_u16(both + 0xe, 2);
        let one = edge(&mut e, 0, 0, 1);
        e.mem.set_u16(one + 8, 4);
        e.mem.set_u16(one + 0xa, 0xffff);
        set_words(&mut e, this + 0x30, &[one, both]);
        e.register(0x0069_07d0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x006b_e830, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x0069_07d0),
            vec![vec![0x0710_0200, 1, 3], vec![0x0710_0300, 2, 2]]
        );
    }

    #[test]
    fn fn_006bc940_folds_the_levels() {
        let mut e = Engine::new();
        let (this, undo) = cutter(&mut e);
        triangle_doubles(&mut e);
        set_indices(&mut e, undo + 0x20, &[1, 2]);
        let saved = e.mem.u32(undo + 0x34);
        e.mem.set_u16(saved, 3);
        e.mem.set_u32(undo + 0x38, 1);
        e.register(0x0069_9790, |_, a| ret(0xabcd_0000 | (a[0] >> 8 & 0xff)));
        e.register(0x0069_9760, |_, a| ret(a[0] * 10 + a[1]));
        assert_eq!(e.call(0x006b_c940, &args![this]).u32(), 123);
    }

    #[test]
    fn fn_006bde50_joins_flagged_vertices() {
        let mut e = Engine::new();
        let (this, _) = cutter(&mut e);
        let source = vertex(&mut e, 1, [1, 0, 0, 0, 0]);
        let inner = vertex(&mut e, 2, [0, 1, 0, 0, 0]);
        let blocked = vertex(&mut e, 3, [0, 1, 0, 0, 1]);
        let plain = vertex(&mut e, 4, [0; 5]);
        set_words(&mut e, this + 0x1c, &[source, inner, blocked, plain]);
        e.register(EDGE_ALLOWED, |_, a| ret(u32::from(a[2] != a[1])));
        e.register(NEW_EDGE, |_, a| ret(a[1] ^ a[2]));
        e.call(0x006b_de50, &args![this]);
        assert_eq!(array_words(&e, this + 0x30), vec![source ^ inner]);
    }

    #[test]
    fn remove_invalid_edges_drops_dropped_and_dangling_edges() {
        let mut e = Engine::new();
        let (this, _) = cutter(&mut e);
        let anchored = vertex(&mut e, 1, [1, 0, 1, 0, 0]);
        let other_anchored = vertex(&mut e, 2, [1, 0, 0, 1, 0]);
        let lone = vertex(&mut e, 3, [1, 0, 0, 0, 0]);
        let keep = edge(&mut e, anchored, other_anchored, 1);
        let dangling = edge(&mut e, anchored, lone, 1);
        e.mem.set_u32(dangling + 0x1c, 0x0900_0000);
        let dropped = edge(&mut e, anchored, other_anchored, 2);
        let free = vertex(&mut e, 4, [0; 5]);
        let kind_two = edge(&mut e, free, lone, 2);
        set_words(&mut e, this + 0x30, &[dangling, keep, dropped, kind_two]);
        e.register(NI_POINTER_GET, |e, a| ret(e.mem.u32(a[0])));
        e.register(RELEASE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x006b_dcb0, &args![this]);
        assert_eq!(array_words(&e, this + 0x30), vec![keep, kind_two]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, RELEASE), vec![vec![0x0900_0000]]);
        assert_eq!(calls_to(&log, 0x009a_4320).len(), 2);
        assert!(calls_to(&log, 0x006b_f8f0).is_empty());
    }

    #[test]
    fn fn_006bdf50_closes_triangles_between_open_edges() {
        let mut e = Engine::new();
        let (this, _) = cutter(&mut e);
        let a = vertex(&mut e, 1, [0; 5]);
        let b = vertex(&mut e, 2, [0; 5]);
        let c = vertex(&mut e, 3, [0; 5]);
        let d = vertex(&mut e, 4, [0; 5]);
        let first = edge(&mut e, a, b, 1);
        let second = edge(&mut e, c, b, 1);
        let unrelated = edge(&mut e, d, d, 1);
        let closed = edge(&mut e, a, c, 1);
        e.mem.set_u8(closed + 0x2f, 1);
        set_words(&mut e, this + 0x30, &[first, second, unrelated, closed]);
        e.register(EDGE_IS_OPEN, |e, a| {
            ret(u32::from(e.mem.u8(a[0] + 0x2f) == 0))
        });
        e.register(EDGE_ALLOWED, |_, _| ret(1));
        e.register(NEW_EDGE, |e, a| {
            let new = e.mem.alloc(0x30);
            e.mem.set_u32(new, a[1]);
            e.mem.set_u32(new + 4, a[2]);
            e.mem.set_u8(new + 0x2f, 1);
            ret(new)
        });
        e.call_log = Some(vec![]);
        e.call(0x006b_df50, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, NEW_EDGE), vec![vec![this, a, c, 0]]);
        assert_eq!(array_words(&e, this + 0x30).len(), 5);
    }

    #[test]
    fn fn_006bca20_saves_new_neighbours() {
        let mut e = Engine::new();
        let (this, undo) = cutter(&mut e);
        triangle_doubles(&mut e);
        set_indices(&mut e, undo + 0x20, &[1]);
        let saved = undo + 0x30;
        // Triangle 1: side 0 border, side 1 neighbour 2 (new), side 2 neighbour 1 (cut).
        e.register(0x0068_f1d0, |_, a| {
            ret(u32::from(
                a[0] == 0x0710_0100 && a[1] == 0 || a[0] == 0x0710_0200 && a[1] == 1,
            ))
        });
        e.register(0x0068_f200, |_, _| ret(1));
        e.register(0x0068_f2c0, |_, a| {
            ret(match (a[0], a[1]) {
                (0x0710_0100, 1) => 2,
                (0x0710_0100, 2) => 1,
                (_, side) => 0x40 + side,
            })
        });
        e.register(0x0055_8200, |_, _| ret(10));
        e.register(0x0069_b240, |e, a| {
            ret(if e.mem.u32(a[1]) == 1 { 0 } else { 0xffff_ffff })
        });
        e.register(0x006a_8890, |e, a| {
            e.mem.set_u32(a[0] + 8, a[1]);
            Ret::default()
        });
        for address in [0x0051_9270u32, 0x0069_1570] {
            e.register(address, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x006b_ca20, &args![this, saved]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(saved + 8), 1);
        let entry = e.mem.u32(saved + 4);
        assert_eq!(e.mem.u16(entry), 2);
        assert_eq!(e.mem.bytes(entry + 2, 3), vec![0, 1, 0]);
        assert_eq!(
            [
                e.mem.u16(entry + 6),
                e.mem.u16(entry + 8),
                e.mem.u16(entry + 0xa)
            ],
            [0x40, 0x41, 0x42]
        );
        assert_eq!(calls_to(&log, 0x0051_9270), vec![vec![0x0710_0100, 0x100]]);
        assert_eq!(calls_to(&log, 0x0069_1570), vec![vec![0x0710_0200, 0x100]]);
        // Running again finds triangle 2 already saved.
        e.call_log = Some(vec![]);
        e.call(0x006b_ca20, &args![this, saved]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, 0x006a_8890).is_empty());
        assert_eq!(e.mem.u32(saved + 8), 1);
    }

    #[test]
    fn fn_006bcc60_builds_border_edges_by_kind() {
        let mut e = Engine::new();
        let (this, undo) = cutter(&mut e);
        triangle_doubles(&mut e);
        set_indices(&mut e, undo + 0x20, &[1]);
        // Side 0: border (kind 4); side 1: no neighbour (kind 3); side 2: neighbour 5 not cut
        // (kind 2).
        e.register(0x0068_f1d0, |_, a| ret(u32::from(a[1] == 0)));
        e.register(0x0068_f2c0, |_, a| {
            ret(match a[1] {
                1 => 0xffff,
                2 => 5,
                _ => 9,
            })
        });
        e.register(0x0069_b240, |_, _| ret(0xffff_ffff));
        e.register(0x0055_82d0, |_, a| ret(0x20 + a[1]));
        // Vertex 0x21 exists already (index 0 of the vertex array); the others are new.
        let existing = vertex(&mut e, 0x21, [0; 5]);
        set_words(&mut e, this + 0x1c, &[existing]);
        // 006bf170 finds a vertex of the array by its 16-bit index.
        e.register(0x006b_f170, |e, a| {
            let count = e.mem.u32(a[0] + 0x1c + 8);
            let buffer = e.mem.u32(a[0] + 0x1c + 4);
            let found = (0..count).find(|position| {
                let vertex = e.mem.u32(buffer + 4 * position);
                u32::from(e.mem.u16(vertex)) == a[1]
            });
            ret(found.unwrap_or(0xffff_ffff))
        });
        e.register(NEW_VERTEX, |e, a| {
            let new = e.mem.alloc(0x10);
            e.mem.set_u16(new, a[1] as u16);
            e.mem.set_u8(new + 3, a[3] as u8);
            e.mem.set_u8(new + 4, a[4] as u8);
            ret(new)
        });
        e.register(NEW_EDGE, |e, a| {
            let new = e.mem.alloc(0x30);
            e.mem.set_u32(new, a[1]);
            e.mem.set_u32(new + 4, a[2]);
            e.mem.set_u32(new + 0x10, a[3]);
            ret(new)
        });
        e.register(0x0068_f460, |e, a| {
            e.mem.set_u16(a[4], 0x11);
            e.mem.set_u16(a[5], 0x22);
            ret(1)
        });
        for address in [EDGE_SET_TRIANGLE, 0x0069_97b0, 0x0069_1040] {
            e.register(address, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x006b_cc60, &args![this]);
        let log = e.call_log.take().unwrap();
        let edges = array_words(&e, this + 0x30);
        assert_eq!(edges.len(), 3);
        let kinds: Vec<_> = edges.iter().map(|edge| e.mem.u32(edge + 0x10)).collect();
        assert_eq!(kinds, vec![4, 3, 2]);
        // Kind 4: the triangle, side and neighbour recorded.
        assert_eq!(e.mem.u16(edges[0] + 0x24), 1);
        assert_eq!(e.mem.u32(edges[0] + 0x28), 0);
        assert_eq!(e.mem.u16(edges[0] + 0x2c), 9);
        // Kind 3: the navmesh is told; kind 2: the matching edge's triangles.
        assert_eq!(
            calls_to(&log, 0x0069_97b0),
            vec![vec![0x0700_0000, 1, edges[1] + 0x1c]]
        );
        assert_eq!(
            calls_to(&log, EDGE_SET_TRIANGLE),
            vec![vec![edges[2], 0x11, 0x22]]
        );
        // Sides 0 and 1 only get 00691040.
        assert_eq!(
            calls_to(&log, 0x0069_1040),
            vec![
                vec![
                    0x0710_0100,
                    0,
                    edges[0] + 0x16,
                    edges[0] + 0x18,
                    edges[0] + 0x19
                ],
                vec![
                    0x0710_0100,
                    1,
                    edges[1] + 0x16,
                    edges[1] + 0x18,
                    edges[1] + 0x19
                ],
            ]
        );
        // The existing vertex got the outer flag; new vertices carry their flags.
        assert_eq!(e.mem.u8(existing + 4), 1);
        let vertices = array_words(&e, this + 0x1c);
        assert_eq!(vertices.len(), 3);
        let new_inner = calls_to(&log, NEW_VERTEX)
            .into_iter()
            .filter(|call| call[3] == 1)
            .count();
        assert_eq!(new_inner, 0);
    }

    #[test]
    fn fn_006be950_collects_outer_portals() {
        let mut e = Engine::new();
        let (this, undo) = cutter(&mut e);
        let outer = edge(&mut e, 0, 0, 4);
        e.mem.set_u16(outer + 8, 3);
        e.mem.set_u16(outer + 0xc, 1);
        let no_triangle = edge(&mut e, 0, 0, 4);
        let inner = edge(&mut e, 0, 0, 2);
        set_words(&mut e, this + 0x30, &[outer, no_triangle, inner]);
        let list = e.mem.alloc(0x20);
        e.register(0x0043_8660, |e, a| {
            let buffer = e.mem.alloc(0x100);
            e.mem.set_u32(a[0] + 4, buffer);
            e.mem.set_u32(a[0] + 8, 0);
            Ret::default()
        });
        e.register(0x0062_40d0, |_, _| Ret::default());
        e.register(0x006b_c8b0, |_, _| Ret::default());
        e.register(0x006b_c870, |e, a| {
            let bytes = e.mem.bytes(a[1], 0xc);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        e.register(0x006b_f730, |_, _| Ret::default());
        // The first entry is found in the list at index 4; the second is not.
        e.register(0x0049_f7d0, |e, a| {
            ret(if e.mem.u16(a[1] + 4) == 0 {
                4
            } else {
                0xffff_ffff
            })
        });
        e.register(0x0071_9b20, |_, _| ret(0xffff_ffff));
        e.register(0x0069_b310, |_, _| Ret::default());
        e.register(0x0076_b610, |e, a| ret(u32::from(e.mem.u32(a[0] + 8) == 0)));
        e.register(0x0072_9970, |_, _| Ret::default());
        e.register(0x0043_8690, |_, _| Ret::default());
        e.register(0x006f_3170, |_, _| ret(0));
        e.register(0x0072_ba80, |_, _| Ret::default());
        e.register(0x006d_c6e0, |_, _| Ret::default());
        e.mem.set_u32(list + 8, 6);
        let list_buffer = e.mem.alloc(0x60);
        e.mem.set_u32(list + 4, list_buffer);
        e.call_log = Some(vec![]);
        e.call(0x006b_e950, &args![this, list]);
        let log = e.call_log.take().unwrap();
        // One pair built, from the outer edge with a triangle.
        let pairs = calls_to(&log, 0x006b_f730);
        assert_eq!(pairs.len(), 1);
        assert_eq!(pairs[0][0], undo + 0x50);
        let first = pairs[0][1];
        assert_eq!(e.mem.u16(first + 0xc + 4), 3);
        assert_eq!(e.mem.u32(first + 0xc + 8), 1);
        assert_eq!(e.mem.u8(first + 0x18), 1);
        // The found index went into the index array, then was removed from the list.
        assert_eq!(calls_to(&log, ARRAY_APPEND).len(), 1);
        assert_eq!(
            calls_to(&log, 0x0072_ba80),
            vec![vec![list, e.mem.u32(list + 4) + 4 * 0xc, 1]]
        );
        assert_eq!(e.mem.u32(list + 8), 5);
        // The second half was not in the list, so it was added.
        assert_eq!(calls_to(&log, 0x0069_b310), vec![vec![list, first + 0xc]]);
    }

    #[test]
    fn simple_array_remove_moves_or_reallocates() {
        // In place.
        let mut e = Engine::new();
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array + 4, 0x6000);
        e.mem.set_u32(array + 8, 5);
        e.register(0x006f_3170, |_, _| ret(0));
        e.register(0x0072_ba80, |_, _| Ret::default());
        e.register(0x006d_c6e0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0061_bf70, &args![array, 2u32, 1u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0072_ba80), vec![vec![array, 0x6018, 1]]);
        assert_eq!(
            calls_to(&log, 0x006d_c6e0),
            vec![vec![array, 0x6018, 0x6024, 2]]
        );
        assert_eq!(e.mem.u32(array + 8), 4);
        // Shrinking into a new buffer.
        let mut e = Engine::new();
        let array = e.mem.alloc(0x10);
        e.put_vtable(0x0300_0000, &[0, 0x0300_1004]);
        e.mem.set_u32(array, 0x0300_0000);
        e.mem.set_u32(array + 4, 0x6000);
        e.mem.set_u32(array + 8, 5);
        e.register(0x006f_3170, |_, _| ret(1));
        e.register(0x0086_9600, |_, _| ret(8));
        e.register(0x0300_1004, |_, _| ret(0x7000));
        for address in [0x0072_ba80u32, 0x006d_c6e0, 0x006a_8500] {
            e.register(address, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x0061_bf70, &args![array, 2u32, 0x101u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0300_1004), vec![vec![array, 8]]);
        assert_eq!(
            calls_to(&log, 0x006d_c6e0),
            vec![
                vec![array, 0x7000, 0x6000, 2],
                vec![array, 0x7018, 0x6024, 4]
            ]
        );
        assert_eq!(calls_to(&log, 0x006a_8500), vec![vec![array]]);
        assert_eq!(e.mem.u32(array + 4), 0x7000);
        assert_eq!(e.mem.u32(array + 0xc), 8);
        assert_eq!(e.mem.u32(array + 8), 4);
    }

    /// A map whose buckets are 4 words at +8, hashing a key to `key % 4` (slot +4),
    /// comparing keys by equality (slot +8); slots +0x10/+0x18 are doubles.
    fn obstacle_map(e: &mut Engine, keys: &[(u16, usize)]) -> (u32, Vec<u32>) {
        let map = e.mem.alloc(0x10);
        let buckets = e.mem.alloc(0x10);
        e.mem.set_u32(map + 8, buckets);
        e.mem.set_u32(map + 0xc, keys.len() as u32);
        e.put_vtable(
            0x0300_0000,
            &[0, 0x0300_1004, 0x0300_1008, 0, 0x0300_1010, 0, 0x0300_1018],
        );
        e.mem.set_u32(map, 0x0300_0000);
        e.register(0x0300_1004, |_, a| ret(a[1] % 4));
        e.register(0x0300_1008, |_, a| ret(u32::from(a[1] == a[2])));
        e.register(0x0300_1010, |_, _| Ret::default());
        e.register(0x0300_1018, |_, _| Ret::default());
        let mut items = vec![];
        for (key, bucket) in keys {
            let item = e.mem.alloc(8);
            e.mem.set_u16(item + 4, *key);
            let head = buckets + 4 * *bucket as u32;
            let old = e.mem.u32(head);
            e.mem.set_u32(item, old);
            e.mem.set_u32(head, item);
            items.push(item);
        }
        (map, items)
    }

    #[test]
    fn obstacle_map_remove_at_unlinks_the_item() {
        // Bucket 1 holds 9 -> 5 -> 1 (inserted 1, 5, 9).
        let mut e = Engine::new();
        let (map, items) = obstacle_map(&mut e, &[(1, 1), (5, 1), (9, 1)]);
        let buckets = e.mem.u32(map + 8);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0049_c250, &args![map, 5u32]).bool());
        assert_eq!(e.mem.u32(items[2]), items[0]);
        assert_eq!(e.mem.u32(map + 0xc), 2);
        assert!(e.call(0x0049_c250, &args![map, 9u32]).bool());
        assert_eq!(e.mem.u32(buckets + 4), items[0]);
        assert!(!e.call(0x0049_c250, &args![map, 13u32]).bool());
        assert!(!e.call(0x0049_c250, &args![map, 2u32]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x0300_1010),
            vec![vec![map, items[1]], vec![map, items[2]]]
        );
        assert_eq!(calls_to(&log, 0x0300_1018).len(), 2);
        assert_eq!(e.mem.u32(map + 0xc), 1);
    }

    /// Doubles for the triangle fill: vertex positions are their 16-bit index in x (a test
    /// position block per vertex), differences and cross products computed for real, the
    /// cross product's z set to `z`.
    fn fill_doubles(e: &mut Engine, length: f32, z: f32) {
        e.map(0x0101_ffa0, 8);
        e.set_global(0x0101_ffa0, f64::from(0.1f32));
        e.map(0x0101_2060, 8);
        e.set_global(0x0101_2060, 0.0f64);
        e.register(NAVMESH_VERTEX, |_, a| ret(0x0800_0000 + 0x10 * a[1]));
        e.register(0x0043_9ef0, |_, a| ret(a[1]));
        e.register_double(0x004b_3800, move |e, a| {
            e.mem.set_f32(a[1] + 8, z);
            Ret::default()
        });
        e.register_double(0x004a_7290, move |_, _| Ret {
            st0: f64::from(length),
            ..Ret::default()
        });
        e.register(EDGE_IS_OPEN, |e, a| {
            ret(u32::from(e.mem.u8(a[0] + 0x2f) == 0))
        });
        e.register(0x006b_ec40, |_, _| ret(1));
        e.register(0x006b_ede0, |_, _| ret(1));
        e.register(0x0069_1000, |_, _| ret(0x0055));
        for address in [
            0x0069_1160u32,
            0x006b_e630,
            0x006a_d2f0,
            0x0069_0e30,
            EDGE_SET_TRIANGLE,
        ] {
            e.register(address, |_, _| Ret::default());
        }
    }

    /// Three edges a-b, b-c, a-c (the third outer, kind 4 with neighbour 0x33).
    fn triangle_edges(e: &mut Engine, this: u32) -> [u32; 3] {
        let a = vertex(e, 1, [0; 5]);
        let b = vertex(e, 2, [0; 5]);
        let c = vertex(e, 3, [0; 5]);
        let first = edge(e, a, b, 1);
        let second = edge(e, a, c, 1);
        let third = edge(e, c, b, 4);
        e.mem.set_u16(third + 0x2c, 0x33);
        set_words(e, this + 0x30, &[first, second, third]);
        [first, second, third]
    }

    #[test]
    fn fn_006be120_adds_a_triangle_with_the_winding_of_z() {
        for (z, vertices, sides) in [
            (1.0f32, [1u32, 2, 3], [0u32, 2, 1]),
            (-1.0, [1, 3, 2], [2, 0, 1]),
        ] {
            let mut e = Engine::new();
            let (this, _) = cutter(&mut e);
            fill_doubles(&mut e, 1.0, z);
            let edges = triangle_edges(&mut e, this);
            e.call_log = Some(vec![]);
            assert!(e.call(0x006b_e120, &args![this]).bool());
            let log = e.call_log.take().unwrap();
            let tri = calls_to(&log, 0x0069_1160)[0][0];
            assert_eq!(
                calls_to(&log, 0x006b_e630),
                vec![vec![tri, vertices[0], vertices[1], vertices[2]]]
            );
            let records = calls_to(&log, 0x006a_d2f0);
            assert_eq!(records[0][1..3], [0, 0]);
            assert_eq!(records[0][0], tri);
            assert_eq!(records.len(), 2);
            assert_eq!(calls_to(&log, 0x0069_0e30), vec![vec![tri, 1, 0x33]]);
            assert_eq!(
                calls_to(&log, EDGE_SET_TRIANGLE),
                vec![
                    vec![edges[0], 0x55, sides[0]],
                    vec![edges[1], 0x55, sides[1]],
                    vec![edges[2], 0x55, sides[2]],
                ]
            );
        }
    }

    #[test]
    fn fn_006be120_skips_degenerate_triangles() {
        for length in [0.05f32, 0.099] {
            let mut e = Engine::new();
            let (this, _) = cutter(&mut e);
            fill_doubles(&mut e, length, 1.0);
            triangle_edges(&mut e, this);
            e.call_log = Some(vec![]);
            assert!(!e.call(0x006b_e120, &args![this]).bool());
            let log = e.call_log.take().unwrap();
            assert!(calls_to(&log, 0x0069_1000).is_empty());
        }
        // NaN lengths go on (the flag test treats unordered as not below).
        let mut e = Engine::new();
        let (this, _) = cutter(&mut e);
        fill_doubles(&mut e, f32::NAN, 1.0);
        triangle_edges(&mut e, this);
        assert!(e.call(0x006b_e120, &args![this]).bool());
    }

    #[test]
    fn fn_006be120_needs_open_edges_on_different_triangles() {
        let mut e = Engine::new();
        let (this, _) = cutter(&mut e);
        fill_doubles(&mut e, 1.0, 1.0);
        let edges = triangle_edges(&mut e, this);
        e.mem.set_u16(edges[0] + 8, 7);
        e.mem.set_u16(edges[1] + 8, 7);
        assert!(!e.call(0x006b_e120, &args![this]).bool());
        let mut e = Engine::new();
        let (this, _) = cutter(&mut e);
        fill_doubles(&mut e, 1.0, 1.0);
        let edges = triangle_edges(&mut e, this);
        e.mem.set_u8(edges[2] + 0x2f, 1);
        assert!(!e.call(0x006b_e120, &args![this]).bool());
    }

    /// Doubles for `SplitEdges`: positions are at `0x0800_0000 + 0x10 * index` holding the
    /// index as x; the crossing test answers `crossing` and writes the point (x = 10.0);
    /// distances are |x - 10| (the difference double writes it).
    fn split_doubles(e: &mut Engine, crossing: bool) {
        e.map(0x0800_0000, 0x1000);
        for index in 0..0x40u32 {
            e.mem.set_f32(0x0800_0000 + 0x10 * index, index as f32);
        }
        e.register(NAVMESH_VERTEX, |_, a| ret(0x0800_0000 + 0x10 * a[1]));
        e.register(0x0068_15c0, |_, a| ret(a[0]));
        e.register_double(0x006b_a100, move |e, a| {
            e.mem.set_f32(a[4], 10.0);
            ret(u32::from(crossing))
        });
        e.register(0x0043_9ef0, |e, a| {
            let x = e.mem.f32(a[0]) - e.mem.f32(a[2]);
            e.mem.set_f32(a[1], x.abs());
            ret(a[1])
        });
        e.register(0x0059_5c80, |e, a| Ret {
            st0: f64::from(e.mem.f32(a[0])),
            ..Ret::default()
        });
        e.register(0x0062_40d0, |_, _| Ret::default());
        e.register(0x0069_0fe0, |_, _| ret(0x0030));
        e.register(NEW_VERTEX, |e, a| {
            let new = e.mem.alloc(0x10);
            e.mem.set_u16(new, a[1] as u16);
            ret(new)
        });
        e.register(NEW_EDGE, |e, a| {
            let new = e.mem.alloc(0x30);
            e.mem.set_u32(new, a[1]);
            e.mem.set_u32(new + 4, a[2]);
            e.mem.set_u32(new + 0x10, a[3]);
            ret(new)
        });
        e.register(NI_POINTER_GET, |_, _| ret(0));
        for address in [0x006e_5cc0u32, 0x006b_c870] {
            e.register(address, |_, _| Ret::default());
        }
        e.map(0x0101_41cc, 4);
        e.set_global(0x0101_41cc, 0.0001f32);
    }

    #[test]
    fn split_edges_cuts_both_edges_at_a_new_vertex() {
        let mut e = Engine::new();
        let (this, _) = cutter(&mut e);
        split_doubles(&mut e, true);
        let a = vertex(&mut e, 0, [0; 5]);
        let b = vertex(&mut e, 20, [0; 5]);
        let c = vertex(&mut e, 5, [0; 5]);
        let d = vertex(&mut e, 15, [0; 5]);
        let obstacle = edge(&mut e, a, b, 1);
        e.mem.set_u16(obstacle + 0x16, 0x77);
        e.mem.set_u8(obstacle + 0x18, 1);
        let border = edge(&mut e, c, d, 3);
        e.mem.set_u16(border + 0x2c, 0x66);
        e.mem.set_u8(border + 0x19, 1);
        set_words(&mut e, this + 0x30, &[obstacle, border]);
        e.call_log = Some(vec![]);
        e.call(0x006b_d540, &args![this]);
        let log = e.call_log.take().unwrap();
        let vertices = array_words(&e, this + 0x1c);
        assert_eq!(vertices.len(), 1);
        let middle = vertices[0];
        assert_eq!(calls_to(&log, NEW_VERTEX), vec![vec![this, 0x30, 1, 0, 1]]);
        let edges = array_words(&e, this + 0x30);
        assert_eq!(edges.len(), 4);
        let ends: Vec<_> = edges
            .iter()
            .map(|edge| {
                (
                    e.mem.u32(*edge),
                    e.mem.u32(edge + 4),
                    e.mem.u32(edge + 0x10),
                )
            })
            .collect();
        assert_eq!(
            ends,
            vec![
                (a, middle, 1),
                (middle, b, 1),
                (c, middle, 3),
                (middle, d, 3)
            ]
        );
        assert_eq!(e.mem.u16(edges[0] + 0x16), 0x77);
        assert_eq!(e.mem.u8(edges[0] + 0x18), 1);
        assert_eq!(e.mem.u8(edges[1] + 0x18), 0);
        assert_eq!(e.mem.u16(edges[2] + 0x2c), 0x66);
        assert_eq!(e.mem.u8(edges[3] + 0x19), 1);
        assert!(edges.iter().all(|edge| e.mem.u8(edge + 0x2e) == 1));
        assert_eq!(calls_to(&log, 0x006b_f8f0).len(), 2);
    }

    #[test]
    fn split_edges_uses_a_nearby_end() {
        // The crossing point (10.0) lies on the border edge's first end (index 10): only the
        // obstacle edge is split, at that vertex.
        let mut e = Engine::new();
        let (this, _) = cutter(&mut e);
        split_doubles(&mut e, true);
        let a = vertex(&mut e, 0, [0; 5]);
        let b = vertex(&mut e, 20, [0; 5]);
        let c = vertex(&mut e, 10, [0; 5]);
        let d = vertex(&mut e, 15, [0; 5]);
        let obstacle = edge(&mut e, a, b, 1);
        let border = edge(&mut e, c, d, 4);
        set_words(&mut e, this + 0x30, &[obstacle, border]);
        e.call(0x006b_d540, &args![this]);
        assert!(array_words(&e, this + 0x1c).is_empty());
        let edges = array_words(&e, this + 0x30);
        let ends: Vec<_> = edges
            .iter()
            .map(|edge| (e.mem.u32(*edge), e.mem.u32(edge + 4)))
            .collect();
        assert_eq!(ends, vec![(c, d), (a, c), (c, b)]);
    }

    #[test]
    fn split_edges_skips_sharing_or_non_crossing_edges() {
        for (crossing, shared) in [(false, false), (true, true)] {
            let mut e = Engine::new();
            let (this, _) = cutter(&mut e);
            split_doubles(&mut e, crossing);
            let a = vertex(&mut e, 0, [0; 5]);
            let b = vertex(&mut e, 20, [0; 5]);
            let c = if shared { a } else { vertex(&mut e, 5, [0; 5]) };
            let d = vertex(&mut e, 15, [0; 5]);
            let obstacle = edge(&mut e, a, b, 1);
            let border = edge(&mut e, c, d, 3);
            set_words(&mut e, this + 0x30, &[obstacle, border]);
            e.call_log = Some(vec![]);
            e.call(0x006b_d540, &args![this]);
            let log = e.call_log.take().unwrap();
            assert_eq!(array_words(&e, this + 0x30), vec![obstacle, border]);
            assert_eq!(calls_to(&log, 0x006b_a100).len(), usize::from(!shared));
        }
    }

    /// Doubles for the steps `SplitNavMeshFromObstacle` and the undo call that are not under
    /// test: every called function returns 0 unless set.
    fn quiet(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    #[test]
    fn split_nav_mesh_gives_up_without_touched_triangles() {
        let mut e = Engine::new();
        let (this, _) = cutter(&mut e);
        let undo_block = e.mem.alloc(0x70);
        e.put_vtable(UNDO_DATA_VTABLE, &[0x0300_1000]);
        e.register_double(0x00aa_13e0, move |_, _| ret(undo_block));
        quiet(
            &mut e,
            &[
                0x006e_5cc0,
                ARRAY_RELEASE_008454F0,
                0x0049_68b0,
                0x0063_3c90,
                0x0069_ada0,
                0x006b_3f40,
                0x0069_16f0,
                0x006b_fe70,
                0x0069_17d0,
                0x0041_6870,
                0x0069_87d0,
                0x0300_1000,
            ],
        );
        e.register(0x0076_b610, |_, _| ret(1));
        e.map(0x0101_712c, 4);
        e.set_global(0x0101_712c, 5.0f32);
        e.call_log = Some(vec![]);
        let answer = e
            .call(
                0x006b_bd30,
                &args![this, 0x0700_0000u32, 0x0950_0000u32, 0x0960_0000u32],
            )
            .bool();
        assert!(!answer);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this + 8), undo_block);
        let extent = calls_to(&log, 0x0041_6870);
        assert_eq!(extent[0][1..], [5.0f32.to_bits(); 3]);
        assert_eq!(
            calls_to(&log, 0x0069_87d0),
            vec![vec![0x0700_0000, 0x0950_0000, 0, undo_block + 0x20]]
        );
        // The undo data (built by 006bc230) is deleted through its vtable slot 0 with 1.
        assert_eq!(e.mem.u32(undo_block), UNDO_DATA_VTABLE);
        assert_eq!(calls_to(&log, 0x0300_1000), vec![vec![undo_block, 1]]);
    }

    #[test]
    fn undo_restores_the_triangles_and_the_obstacle_map() {
        let mut e = Engine::new();
        let (this, undo) = cutter(&mut e);
        triangle_doubles(&mut e);
        let list = e.mem.alloc(0x20);
        // Nothing saved in the portal pairs; one cut triangle (1), one saved triangle (2) with
        // side 0 a border, new triangles 5 and 6.
        set_indices(&mut e, undo + 0x20, &[1]);
        let saved = e.mem.u32(undo + 0x34);
        e.mem.set_u16(saved, 2);
        e.mem.set_u8(saved + 2, 1);
        e.mem.set_u16(saved + 6, 0x40);
        e.mem.set_u16(saved + 8, 0x41);
        e.mem.set_u16(saved + 0xa, 0x42);
        e.mem.set_u32(undo + 0x38, 1);
        e.mem.set_u32(undo + 0x18, 5);
        e.mem.set_u32(undo + 0x1c, 2);
        // The navmesh's obstacle map at +0xe0.
        let (map, _) = obstacle_map(&mut e, &[(5, 1), (6, 2)]);
        e.mem.set_u32(0x0700_00e0, map);
        e.register(0x0043_8660, |e, a| {
            let buffer = e.mem.alloc(0x100);
            e.mem.set_u32(a[0] + 4, buffer);
            Ret::default()
        });
        e.register(0x0076_b610, |_, _| ret(1));
        e.register(0x0069_9790, |_, _| ret(3));
        // The undo data's map has one entry: key 9.
        e.register(0x004b_9ba0, |_, _| ret(0x5555));
        e.register(0x0069_47e0, |e, a| {
            e.mem.set_u32(a[1], 0);
            e.mem.set_u16(a[2], 9);
            e.mem.set_u32(a[3], 0x0970_0000);
            Ret::default()
        });
        e.register(0x0055_9a40, |e, a| {
            let pointer = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], pointer);
            Ret::default()
        });
        quiet(
            &mut e,
            &[
                0x0051_9270,
                0x0069_0e30,
                0x0069_07d0,
                0x0069_1570,
                0x0069_3bd0,
                0x0063_3c90,
                0x0069_46a0,
                0x0045_cec0,
                0x0069_10a0,
                0x0069_b240,
                0x0043_8690,
            ],
        );
        e.call_log = Some(vec![]);
        let answer = e
            .call(0x006b_c3e0, &args![this, 0x0700_0000u32, undo, list])
            .bool();
        assert!(answer);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(undo + 8), 3);
        assert_eq!(
            calls_to(&log, 0x0051_9270),
            vec![vec![0x0710_0100, 0x20], vec![0x0710_0200, 0x100]]
        );
        assert_eq!(
            calls_to(&log, 0x0069_0e30),
            vec![vec![0x0710_0200, 0, 0x40]]
        );
        assert_eq!(
            calls_to(&log, 0x0069_07d0),
            vec![vec![0x0710_0200, 1, 0x41], vec![0x0710_0200, 2, 0x42]]
        );
        assert_eq!(
            calls_to(&log, 0x0069_1570),
            vec![vec![0x0710_0500, 0x20], vec![0x0710_0600, 0x20]]
        );
        assert_eq!(calls_to(&log, 0x0069_3bd0), vec![vec![0x0710_0200, 2]]);
        // Both new triangles left the navmesh's map; key 9 went back into it.
        assert_eq!(e.mem.u32(map + 0xc), 0);
        assert_eq!(calls_to(&log, 0x0069_46a0), vec![vec![map, 9, 0x0970_0000]]);
    }

    /// Array constructors that give the array a buffer (count 0).
    fn buffered_constructor(e: &mut Engine, address: u32) {
        e.register(address, |e, a| {
            let buffer = e.mem.alloc(0x400);
            e.mem.set_u32(a[0] + 4, buffer);
            e.mem.set_u32(a[0] + 8, 0);
            Ret::default()
        });
    }

    #[test]
    fn split_nav_mesh_cuts_fills_and_records_the_new_triangles() {
        let mut e = Engine::new();
        let (this, _) = cutter(&mut e);
        triangle_doubles(&mut e);
        fill_doubles(&mut e, 1.0, 1.0);
        e.mem.set_u32(this + 4, 0x0950_0000);
        e.mem.set_u32(0x0700_00e0, 0x0780_0000);
        // A triangle of kind-1 edges (vertices anchored with byte +4, so none is dropped).
        let a = vertex(&mut e, 1, [0, 0, 1, 0, 0]);
        let b = vertex(&mut e, 2, [0, 0, 1, 0, 0]);
        let c = vertex(&mut e, 3, [0, 0, 1, 0, 0]);
        let edges = [
            edge(&mut e, a, b, 1),
            edge(&mut e, a, c, 1),
            edge(&mut e, c, b, 1),
        ];
        set_words(&mut e, this + 0x30, &edges);
        let undo_block = e.mem.alloc(0x70);
        e.register_double(0x00aa_13e0, move |_, _| ret(undo_block));
        for address in [
            0x0069_ada0u32,
            0x006b_3f40,
            0x006b_fe70,
            0x0069_17d0,
            0x0043_8660,
        ] {
            buffered_constructor(&mut e, address);
        }
        e.register(0x0069_87d0, |e, a| {
            let buffer = e.mem.u32(a[3] + 4);
            e.mem.set_u16(buffer, 1);
            e.mem.set_u32(a[3] + 8, 1);
            Ret::default()
        });
        e.register(0x0076_b610, |e, a| ret(u32::from(e.mem.u32(a[0] + 8) == 0)));
        let first_counts = std::rc::Rc::new(std::cell::Cell::new(0u32));
        let seen = first_counts.clone();
        e.register_double(0x0099_0a60, move |_, _| {
            seen.set(seen.get() + 1);
            ret(if seen.get() == 1 { 10 } else { 12 })
        });
        let triangle_counts = std::rc::Rc::new(std::cell::Cell::new(0u32));
        let counted = triangle_counts.clone();
        e.register_double(0x0055_8200, move |_, _| {
            counted.set(counted.get() + 1);
            ret(if counted.get() == 1 { 20 } else { 22 })
        });
        // The cut triangle has no new neighbours and no border sides.
        e.register(0x0068_f1d0, |_, _| ret(0));
        e.register(0x0068_f200, |_, _| ret(0));
        e.register(0x0068_f2c0, |_, _| ret(5));
        e.register(0x0069_b240, |_, _| ret(0));
        e.register(0x0069_9790, |_, _| ret(2));
        e.register(0x0069_9760, |_, a| ret(a[0] + a[1]));
        e.register(0x0069_afb0, |_, _| ret(0));
        e.register(NI_POINTER_GET, |_, _| ret(0));
        e.register(EDGE_ALLOWED, |_, _| ret(0));
        e.register(0x0055_9a40, |e, a| {
            let pointer = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], pointer);
            Ret::default()
        });
        e.register(0x006b_f2d0, |_, _| ret(0));
        e.map(0x0101_712c, 4);
        e.set_global(0x0101_712c, 5.0f32);
        quiet(
            &mut e,
            &[
                0x006e_5cc0,
                ARRAY_RELEASE_008454F0,
                0x0049_68b0,
                0x0063_3c90,
                0x0069_16f0,
                0x0041_6870,
                0x0051_9270,
                0x0069_1570,
                0x0045_cec0,
                0x006b_d040,
                0x0043_8690,
                0x0069_3bd0,
                0x0069_46a0,
                0x0069_65d0,
                0x0069_10a0,
                0x0069_9510,
            ],
        );
        e.call_log = Some(vec![]);
        let answer = e
            .call(
                0x006b_bd30,
                &args![this, 0x0700_0000u32, 0x0950_0000u32, 0x0960_0000u32],
            )
            .bool();
        assert!(answer);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(undo_block + 8), 1);
        assert_eq!(e.mem.u32(undo_block + 0x10), 10);
        assert_eq!(e.mem.u32(undo_block + 0x14), 2);
        assert_eq!(e.mem.u32(undo_block + 0x18), 20);
        assert_eq!(e.mem.u32(undo_block + 0x1c), 2);
        // The levels fold: 0 + 2 for the one cut triangle.
        assert_eq!(e.mem.u32(this + 0x44), 2);
        assert_eq!(calls_to(&log, 0x0051_9270), vec![vec![0x0710_0100, 0x100]]);
        assert_eq!(calls_to(&log, 0x0069_1570), vec![vec![0x0710_0100, 0x20]]);
        // Triangles were added by both fill passes.
        assert_eq!(calls_to(&log, 0x0069_1000).len(), 2);
        assert_eq!(
            calls_to(&log, 0x0069_46a0),
            vec![
                vec![0x0780_0000, 20, 0x0950_0000],
                vec![0x0780_0000, 21, 0x0950_0000],
            ]
        );
        assert_eq!(calls_to(&log, 0x0069_65d0), vec![vec![0x0700_0000, 20, 21]]);
        assert_eq!(
            calls_to(&log, 0x0069_9510),
            vec![vec![0x0700_0000, undo_block]]
        );
        assert!(calls_to(&log, 0x0069_4890).is_empty());
    }

    #[test]
    fn split_nav_mesh_rolls_back_when_nothing_fills_the_cut() {
        let mut e = Engine::new();
        let (this, _) = cutter(&mut e);
        triangle_doubles(&mut e);
        let undo_block = e.mem.alloc(0x70);
        e.put_vtable(UNDO_DATA_VTABLE, &[0x0300_1000]);
        e.register_double(0x00aa_13e0, move |_, _| ret(undo_block));
        for address in [0x0069_ada0u32, 0x006b_3f40, 0x006b_fe70, 0x0069_17d0] {
            buffered_constructor(&mut e, address);
        }
        e.register(0x0069_87d0, |e, a| {
            let buffer = e.mem.u32(a[3] + 4);
            e.mem.set_u16(buffer, 1);
            e.mem.set_u32(a[3] + 8, 1);
            Ret::default()
        });
        e.register(0x0076_b610, |e, a| ret(u32::from(e.mem.u32(a[0] + 8) == 0)));
        e.register(0x0099_0a60, |_, _| ret(10));
        e.register(0x0055_8200, |_, _| ret(20));
        e.register(0x0068_f1d0, |_, _| ret(0));
        e.register(0x0068_f200, |_, _| ret(0));
        e.register(0x0068_f2c0, |_, _| ret(5));
        e.register(0x0069_b240, |_, _| ret(0));
        e.register(0x0069_9790, |_, _| ret(2));
        e.register(0x0069_9760, |_, a| ret(a[0] + a[1]));
        e.register(0x0069_afb0, |_, _| ret(0));
        e.map(0x0101_712c, 4);
        quiet(
            &mut e,
            &[
                0x006e_5cc0,
                ARRAY_RELEASE_008454F0,
                0x0049_68b0,
                0x0063_3c90,
                0x0069_16f0,
                0x0041_6870,
                0x0051_9270,
                0x0069_1570,
                0x0045_cec0,
                0x006b_d040,
                0x0069_4890,
                0x0069_49d0,
                0x0300_1000,
            ],
        );
        e.call_log = Some(vec![]);
        let answer = e
            .call(
                0x006b_bd30,
                &args![this, 0x0700_0000u32, 0x0950_0000u32, 0x0960_0000u32],
            )
            .bool();
        assert!(!answer);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0069_4890), vec![vec![0x0700_0028, 10, 1]]);
        assert_eq!(calls_to(&log, 0x0069_49d0), vec![vec![0x0700_0038, 20, 1]]);
        assert_eq!(calls_to(&log, 0x0300_1000), vec![vec![undo_block, 1]]);
    }
}

#[cfg(test)]
mod deleting_destructor_tests {
    use super::*;

    /// Runs a scalar deleting destructor with each `flags`, every outside
    /// callee doubled: checks the result, the vtable the body leaves (when it
    /// stores one), the body callee run on `this` (when it is outside this
    /// file) and whether `00401030` frees the object.
    fn check(address: u32, vtable: Option<u32>, body: Option<u32>) {
        for (flags, frees) in [(0u32, false), (1, true), (2, false), (3, true)] {
            let mut e = Engine::new();
            for callee in [
                ARRAY_RELEASE_008454F0,
                VERTEX_SIMPLE_ARRAY_DESTRUCT,
                EDGE_SIMPLE_ARRAY_DESTRUCT,
                DEALLOCATE,
            ] {
                e.register(callee, |_, _| Ret::default());
            }
            let this = e.mem.alloc(0x10);
            e.mem.set_u32(this, 0x1234_5678);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(address, &args![this, flags]).u32(), this);
            let log = e.call_log.take().unwrap();
            assert_eq!(e.mem.u32(this), vtable.unwrap_or(0x1234_5678));
            if let Some(body) = body {
                let ran: Vec<_> = log.iter().filter(|c| c.0 == body).collect();
                assert_eq!(ran.len(), 1);
                assert_eq!(ran[0].1, vec![this]);
            }
            let freed: Vec<_> = log.iter().filter(|c| c.0 == DEALLOCATE).collect();
            assert_eq!(freed.len(), usize::from(frees));
            if frees {
                assert_eq!(freed[0].1, vec![this]);
            }
        }
    }

    #[test]
    fn at_006bfbe0_bounds_triangle_info_array() {
        check(0x006b_fbe0, Some(ARRAY_VTABLE_0106C2E8), None);
    }

    #[test]
    fn at_006bfc10_portal_edge_swap_array() {
        check(0x006b_fc10, Some(ARRAY_VTABLE_0106C2FC), None);
    }

    #[test]
    fn at_006bfc80_vertex_simple_array() {
        check(0x006b_fc80, None, Some(VERTEX_SIMPLE_ARRAY_DESTRUCT));
    }

    #[test]
    fn at_006bfcb0_vertex_scrap_array() {
        check(
            0x006b_fcb0,
            Some(VERTEX_ARRAY_VTABLE),
            Some(VERTEX_SIMPLE_ARRAY_DESTRUCT),
        );
    }

    #[test]
    fn at_006bfce0_edge_simple_array() {
        check(0x006b_fce0, None, Some(EDGE_SIMPLE_ARRAY_DESTRUCT));
    }

    #[test]
    fn at_006bfd10_edge_scrap_array() {
        check(
            0x006b_fd10,
            Some(EDGE_ARRAY_VTABLE),
            Some(EDGE_SIMPLE_ARRAY_DESTRUCT),
        );
    }

    #[test]
    fn at_006bfc40_copies_both_parts_and_the_byte() {
        let mut e = Engine::new();
        e.register(COPY_EDGE_HANDLE, |_, a| a[0].into_ret());
        let this = e.mem.alloc(0x1c);
        let other = e.mem.alloc(0x1c);
        e.mem.set_u8(other + 0x18, 0xa5);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006b_fc40, &args![this, other]).u32(), this);
        let log = e.call_log.take().unwrap();
        let copies: Vec<_> = log
            .iter()
            .filter(|c| c.0 == COPY_EDGE_HANDLE)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(
            copies,
            vec![vec![this, other], vec![this + 0xc, other + 0xc]]
        );
        assert_eq!(e.mem.u8(this + 0x18), 0xa5);
    }
}
