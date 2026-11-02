//! `fallout shared/pathfinding/pathsmoother.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@UAvoidNode@PathSmoother@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0106D88C: u32 = 0x0106_d88c;
/// `008454f0` (unnamed in the engine map): `thiscall`, `this` plus one stack word (`RET 4`). When the
/// buffer pointer (+4) is set it runs the instance's element destroy step; with the word non-zero it
/// then calls the array's virtual slot +8 on the buffer and clears the buffer pointer and the reserved
/// size (+0xc); finally it clears the size (+8).
const ARRAY_RELEASE_008454F0: u32 = 0x0084_54f0;

// Translated from 006f2e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@UAvoidNode@PathSmoother@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_006f2e00(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106D88C);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

/// `0042f5d0` (cdecl, two words: the buffer and its new size in bytes; it
/// runs `00401020` then `00aa4150` on them, the heap's reallocation).
const BUFFER_REALLOCATE_0042F5D0: u32 = 0x0042_f5d0;

// Translated from 006f2e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSSimpleArray<PathSmoother::AvoidNode,1024>` instance's buffer
/// reallocation (thiscall, `RET 8`; `this` unused): `0042f5d0(buffer,
/// count * 16)`, the elements being 16 bytes. Answers the new buffer (the exe
/// ends on that call and leaves its `EAX`; `_Reallocate` answers the buffer
/// in the Xbox PDB).
pub fn fn_006f2e20(e: &mut Engine, _this: Ptr, buffer: u32, count: u32) -> u32 {
    e.call(BUFFER_REALLOCATE_0042F5D0, &args![buffer, count << 4])
        .u32()
}

/// The vtables of `BSScrapArray<PathSmoother::AvoidNode,1024>` (`0106d878`)
/// and `BSScrapArray<bool,1024>` (`0106c990`), from the RTTI before them.
const SCRAP_ARRAY_VTABLE_0106D878: u32 = 0x0106_d878;
const SCRAP_BOOL_ARRAY_VTABLE_0106C990: u32 = 0x0106_c990;
/// `00401030` (cdecl: block): frees a block.
const FREE_BLOCK: u32 = 0x0040_1030;
/// `0044ddc0` (`thiscall`): an array's element count.
const COUNT_0044DDC0: u32 = 0x0044_ddc0;
/// `00877a30` (`thiscall`, `RET 4`: index): the address of an element.
const ELEMENT_00877A30: u32 = 0x0087_7a30;
/// `ScrapHeap::Allocate` (engine map, `00aa54a0`; `thiscall`, `RET 8`:
/// size, alignment) and `ScrapHeap::Deallocate` (`00aa5610`; `RET 4`).
const SCRAP_HEAP_ALLOCATE: u32 = 0x00aa_54a0;
const SCRAP_HEAP_DEALLOCATE: u32 = 0x00aa_5610;
/// The alignment word the scrap arrays allocate with (`010a2720`; the exe
/// holds 4 there).
const SCRAP_ALIGNMENT: u32 = 0x010a_2720;
/// `00401460` (cdecl: destination, source, byte count): copies bytes.
const COPY_BYTES_00401460: u32 = 0x0040_1460;
/// `00401020` (no arguments): the memory manager;
/// `MemoryManager::GetThreadScrapHeap` (engine map, `00aa42e0`) on it.
const MEMORY_MANAGER: u32 = 0x0040_1020;
const GET_THREAD_SCRAP_HEAP: u32 = 0x00aa_42e0;
/// `0076b610` (`thiscall`): a byte of an array.
const ARRAY_CHECK_0076B610: u32 = 0x0076_b610;
/// `BSSimpleArray<BGSBodyPart_P_1024>::_MoveItems` (engine map,
/// `009a4320`; `thiscall`, `RET 8`).
const MOVE_ITEMS_009A4320: u32 = 0x009a_4320;
/// The destructor bodies `PathSmoother::~PathSmoother` runs on its members
/// (`thiscall`, no stack words), and its `+0x30` member's (`00982610`).
const MEMBER_DESTRUCT_006F2BB0: u32 = 0x006f_2bb0;
const MEMBER_DESTRUCT_006F25D0: u32 = 0x006f_25d0;
const MEMBER_DESTRUCT_006F2810: u32 = 0x006f_2810;
const MEMBER_DESTRUCT_006F26F0: u32 = 0x006f_26f0;
const MEMBER_DESTRUCT_00982610: u32 = 0x0098_2610;
/// `006ed510` (`thiscall`): the destructor body `006ed4e0` runs.
const DESTRUCT_006ED510: u32 = 0x006e_d510;
/// `006d0420` (`thiscall`): the array constructor `006f2ca0` starts with.
const ARRAY_CONSTRUCT_006D0420: u32 = 0x006d_0420;
/// `0069b940` (`thiscall`, `RET 8`: reserve, size): the array's
/// initialisation.
const ARRAY_INIT_0069B940: u32 = 0x0069_b940;
/// `006f31f0` (`thiscall`): makes room for one more element, returns its
/// index.
const ADD_SLOT_006F31F0: u32 = 0x006f_31f0;
/// `006f3280` (`thiscall`, `RET 8`: first, count): constructs elements.
const CONSTRUCT_RANGE_006F3280: u32 = 0x006f_3280;
/// `006f3320` (`thiscall`, `RET 8`: reserve, size): the array's
/// initialisation.
const ARRAY_INIT_006F3320: u32 = 0x006f_3320;
/// The destructor bodies of the scalar deleting destructors below that
/// are not in this file.
const DESTRUCT_006F25B0: u32 = 0x006f_25b0;
const DESTRUCT_006F26D0: u32 = 0x006f_26d0;
const DESTRUCT_006F27F0: u32 = 0x006f_27f0;

/// The arrays of element pointers at `array` (`0044ddc0`, `00877a30`):
/// with `destroy`, each element's `006ed4e0(element, 0)`; then each element
/// is given back to the scrap heap at `+0x30` (`ScrapHeap::Deallocate`).
fn release_elements(e: &mut Engine, base: u32, array: u32, destroy: bool) {
    let mut index = 0u32;
    while index < e.call(COUNT_0044DDC0, &args![base + array]).u32() {
        if destroy {
            let slot = e.call(ELEMENT_00877A30, &args![base + array, index]).u32();
            let element = e.mem.u32(slot);
            fn_006ed4e0(e, Ptr::new(element), 0);
        }
        let slot = e.call(ELEMENT_00877A30, &args![base + array, index]).u32();
        let element = e.mem.u32(slot);
        e.call(SCRAP_HEAP_DEALLOCATE, &args![base + 0x30, element]);
        index += 1;
    }
}

// Translated from 006ed220 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006ed220 - the exception-unwinding frame (FS:[0]) that destroys the remaining members on unwind is not translated.
/// `PathSmoother::~PathSmoother` (engine map; `thiscall`, plain `RET`): the
/// elements of the arrays at `+0x40` and `+0x54` are destroyed
/// (`006ed4e0` with 0) and given back to the scrap heap at `+0x30`
/// (between the two, `+0x54` twice gets `_MoveItems(0, 1)` when its
/// `0076b610` is clear), both arrays released (`008454f0` with 1); the
/// elements of `+0x68`, `+0x7c` and then `+0x90` are given back too, and
/// the arrays released; then the members are destroyed in reverse:
/// `+0xa4` (`006f2bb0`), `+0x90` (`006f25d0`), `+0x7c` and `+0x68`
/// (`006f2810`), `+0x54` and `+0x40` (`006f26f0`), `+0x30` (`00982610`).
pub fn path_smoother_destructor(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    release_elements(e, base, 0x40, true);
    if !e.call(ARRAY_CHECK_0076B610, &args![base + 0x54]).bool() {
        e.call(MOVE_ITEMS_009A4320, &args![base + 0x54, 0u32, 1u32]);
    }
    if !e.call(ARRAY_CHECK_0076B610, &args![base + 0x54]).bool() {
        e.call(MOVE_ITEMS_009A4320, &args![base + 0x54, 0u32, 1u32]);
    }
    release_elements(e, base, 0x54, true);
    e.call(ARRAY_RELEASE_008454F0, &args![base + 0x40, 1u32]);
    e.call(ARRAY_RELEASE_008454F0, &args![base + 0x54, 1u32]);
    release_elements(e, base, 0x68, false);
    release_elements(e, base, 0x7c, false);
    e.call(ARRAY_RELEASE_008454F0, &args![base + 0x68, 1u32]);
    e.call(ARRAY_RELEASE_008454F0, &args![base + 0x7c, 1u32]);
    release_elements(e, base, 0x90, false);
    e.call(ARRAY_RELEASE_008454F0, &args![base + 0x90, 1u32]);
    e.call(MEMBER_DESTRUCT_006F2BB0, &args![base + 0xa4]);
    e.call(MEMBER_DESTRUCT_006F25D0, &args![base + 0x90]);
    e.call(MEMBER_DESTRUCT_006F2810, &args![base + 0x7c]);
    e.call(MEMBER_DESTRUCT_006F2810, &args![base + 0x68]);
    e.call(MEMBER_DESTRUCT_006F26F0, &args![base + 0x54]);
    e.call(MEMBER_DESTRUCT_006F26F0, &args![base + 0x40]);
    e.call(MEMBER_DESTRUCT_00982610, &args![base + 0x30]);
}

/// A scalar deleting destructor: `destruct` on `this`, then the block is
/// freed (`00401030`) when bit 0 of `flags` is set. Returns `this`.
fn scalar_delete(e: &mut Engine, this: Ptr, flags: u32, destruct: impl FnOnce(&mut Engine)) -> Ptr {
    destruct(e);
    if flags & 1 != 0 {
        e.call(FREE_BLOCK, &args![this]);
    }
    this
}

// Translated from 006ed4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor (`thiscall`; flags; `RET 4`; no map name):
/// the destructor body `006ed510`, then the block is freed when bit 0 of
/// `flags` is set. Returns `this`.
pub fn fn_006ed4e0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    scalar_delete(e, this, flags, |e| {
        e.call(DESTRUCT_006ED510, &args![this]);
    })
}

/// `ScrapHeap::Allocate` on the array's scrap heap (`+0x10`) for `bytes`,
/// with the alignment word at `010a2720`.
fn scrap_allocate(e: &mut Engine, this: Ptr, bytes: u32) -> u32 {
    let alignment = e.global::<u32>(SCRAP_ALIGNMENT);
    let heap = e.mem.u32(this.addr() + 0x10);
    e.call(SCRAP_HEAP_ALLOCATE, &args![heap, bytes, alignment])
        .u32()
}

/// The shared body of the `_Reallocate`s: a new block for `count`
/// elements of `size` bytes, the current elements (`0044ddc0`) copied from
/// `old`, the old block given back. Returns the new block.
fn scrap_reallocate(e: &mut Engine, this: Ptr, old: u32, count: u32, size: u32) -> u32 {
    let block = scrap_allocate(e, this, count.wrapping_mul(size));
    let used = e.call(COUNT_0044DDC0, &args![this]).u32();
    e.call(
        COPY_BYTES_00401460,
        &args![block, old, used.wrapping_mul(size)],
    );
    let heap = e.mem.u32(this.addr() + 0x10);
    e.call(SCRAP_HEAP_DEALLOCATE, &args![heap, old]);
    block
}

// Translated from 006f2c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapArray<PathingLocation,1024>::_Allocate` (engine map;
/// `thiscall`; count; `RET 4`): `count` 0x28-byte elements from the scrap
/// heap at `+0x10`.
pub fn bs_scrap_array_pathing_location_allocate(e: &mut Engine, this: Ptr, count: u32) -> u32 {
    scrap_allocate(e, this, count.wrapping_mul(0x28))
}

// Translated from 006f2c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapArray<PathingLocation,1024>::_Reallocate` (engine map;
/// `thiscall`; old buffer, count; `RET 8`): a new block for `count`
/// 0x28-byte elements, the array's current ones copied over (`00401460`),
/// the old block given back (`ScrapHeap::Deallocate`). Returns the new
/// block.
pub fn bs_scrap_array_pathing_location_reallocate(
    e: &mut Engine,
    this: Ptr,
    old: u32,
    count: u32,
) -> u32 {
    scrap_reallocate(e, this, old, count, 0x28)
}

// Translated from 006f2ca0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f2ca0 - the exception-unwinding frame (FS:[0]) is not translated.
/// A `BSScrapArray<bool,1024>` constructor (`thiscall`; size; `RET 4`; no
/// map name): `006d0420` on `this`, the vtable `0106c990`, the thread's
/// scrap heap (`GetThreadScrapHeap` of `00401020`) at `+0x10`, then
/// `0069b940(size, size)`. Returns `this`.
pub fn fn_006f2ca0(e: &mut Engine, this: Ptr, size: u32) -> Ptr {
    e.call(ARRAY_CONSTRUCT_006D0420, &args![this]);
    e.mem.set_u32(this.addr(), SCRAP_BOOL_ARRAY_VTABLE_0106C990);
    let manager = e.call(MEMORY_MANAGER, &[]).u32();
    let heap = e.call(GET_THREAD_SCRAP_HEAP, &args![manager]).u32();
    e.mem.set_u32(this.addr() + 0x10, heap);
    e.call(ARRAY_INIT_0069B940, &args![this, size, size]);
    this
}

// Translated from 006f2d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends a 16-byte element (`thiscall`; the element; `RET 4`; no map
/// name): a slot from `006f31f0`, the element there constructed
/// (`006f3280` with 1) and the 16 bytes copied from `item`. Returns the
/// index.
pub fn fn_006f2d20(e: &mut Engine, this: Ptr, item: Ptr) -> u32 {
    let index = e.call(ADD_SLOT_006F31F0, &args![this]).u32();
    let slot = (index << 4).wrapping_add(e.mem.u32(this.addr() + 4));
    e.call(CONSTRUCT_RANGE_006F3280, &args![this, slot, 1u32]);
    let slot = e.mem.u32(this.addr() + 4).wrapping_add(index << 4);
    for offset in [0, 4, 8, 0xc] {
        let word = e.mem.u32(item.addr() + offset);
        e.mem.set_u32(slot + offset, word);
    }
    index
}

// Translated from 006f2d80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f2d80 - the exception-unwinding frame (FS:[0]) is not translated.
/// `BSScrapArray<PathSmoother::AvoidNode,1024>`'s constructor (`thiscall`,
/// plain `RET`; no map name): the `BSSimpleArray` part ([`fn_006f31c0`]),
/// the vtable `0106d878`, the thread's scrap heap at `+0x10`, then
/// `006f3320(0, 0)`. Returns `this`.
pub fn fn_006f2d80(e: &mut Engine, this: Ptr) -> Ptr {
    fn_006f31c0(e, this);
    e.mem.set_u32(this.addr(), SCRAP_ARRAY_VTABLE_0106D878);
    let manager = e.call(MEMORY_MANAGER, &[]).u32();
    let heap = e.call(GET_THREAD_SCRAP_HEAP, &args![manager]).u32();
    e.mem.set_u32(this.addr() + 0x10, heap);
    e.call(ARRAY_INIT_006F3320, &args![this, 0u32, 0u32]);
    this
}

// Translated from 006f2e40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f2e40 - the exception-unwinding frame (FS:[0]) is not translated.
/// `BSScrapArray<PathSmoother::AvoidNode,1024>`'s destructor body
/// (`thiscall`, plain `RET`; no map name): the vtable `0106d878`, the
/// array released (`008454f0` with 1), then the `BSSimpleArray` part's
/// destructor ([`fn_006f2e00`]).
pub fn fn_006f2e40(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), SCRAP_ARRAY_VTABLE_0106D878);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
    fn_006f2e00(e, this);
}

// Translated from 006f2ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapArray<PathSmoother::AvoidNode,1024>::_Allocate` (engine map;
/// `thiscall`; count; `RET 4`): `count` 16-byte elements from the scrap
/// heap at `+0x10`.
pub fn bs_scrap_array_avoid_node_allocate(e: &mut Engine, this: Ptr, count: u32) -> u32 {
    scrap_allocate(e, this, count << 4)
}

// Translated from 006f2ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapArray<PathSmoother::AvoidNode,1024>::_Reallocate` (engine map;
/// `thiscall`; old buffer, count; `RET 8`): as the `PathingLocation` one,
/// with 16-byte elements.
pub fn bs_scrap_array_avoid_node_reallocate(
    e: &mut Engine,
    this: Ptr,
    old: u32,
    count: u32,
) -> u32 {
    scrap_reallocate(e, this, old, count, 0x10)
}

// Translated from 006f2f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<PathSmoother::PathEdge*,1024>`'s scalar deleting
/// destructor (engine map; `thiscall`; flags; `RET 4`): `006f25b0`, then
/// freed on bit 0.
pub fn bs_simple_array_path_edge_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    scalar_delete(e, this, flags, |e| {
        e.call(DESTRUCT_006F25B0, &args![this]);
    })
}

// Translated from 006f2f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapArray<PathSmoother::PathEdge*,1024>`'s scalar deleting
/// destructor (engine map; `thiscall`; flags; `RET 4`): `006f25d0`, then
/// freed on bit 0.
pub fn bs_scrap_array_path_edge_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    scalar_delete(e, this, flags, |e| {
        e.call(MEMBER_DESTRUCT_006F25D0, &args![this]);
    })
}

// Translated from 006f2f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<PathSmoother::UniqueVert*,1024>`'s scalar deleting
/// destructor (engine map; `thiscall`; flags; `RET 4`): `006f26d0`, then
/// freed on bit 0.
pub fn bs_simple_array_unique_vert_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    scalar_delete(e, this, flags, |e| {
        e.call(DESTRUCT_006F26D0, &args![this]);
    })
}

// Translated from 006f2fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapArray<PathSmoother::UniqueVert*,1024>`'s scalar deleting
/// destructor (engine map; `thiscall`; flags; `RET 4`): `006f26f0`, then
/// freed on bit 0.
pub fn bs_scrap_array_unique_vert_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    scalar_delete(e, this, flags, |e| {
        e.call(MEMBER_DESTRUCT_006F26F0, &args![this]);
    })
}

// Translated from 006f2ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<PathSmoother::BoundsEdge*,1024>`'s scalar deleting
/// destructor (engine map; `thiscall`; flags; `RET 4`): `006f27f0`, then
/// freed on bit 0.
pub fn bs_simple_array_bounds_edge_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    scalar_delete(e, this, flags, |e| {
        e.call(DESTRUCT_006F27F0, &args![this]);
    })
}

// Translated from 006f3020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapArray<PathSmoother::BoundsEdge*,1024>`'s scalar deleting
/// destructor (engine map; `thiscall`; flags; `RET 4`): `006f2810`, then
/// freed on bit 0.
pub fn bs_scrap_array_bounds_edge_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    scalar_delete(e, this, flags, |e| {
        e.call(MEMBER_DESTRUCT_006F2810, &args![this]);
    })
}

// Translated from 006f3050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapArray<PathingLocation,1024>`'s scalar deleting destructor
/// (engine map; `thiscall`; flags; `RET 4`): `006f2bb0`, then freed on
/// bit 0.
pub fn bs_scrap_array_pathing_location_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    scalar_delete(e, this, flags, |e| {
        e.call(MEMBER_DESTRUCT_006F2BB0, &args![this]);
    })
}

// Translated from 006f3080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<PathSmoother::AvoidNode,1024>`'s scalar deleting
/// destructor (engine map; `thiscall`; flags; `RET 4`): [`fn_006f2e00`],
/// then freed on bit 0.
pub fn bs_simple_array_avoid_node_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    scalar_delete(e, this, flags, |e| fn_006f2e00(e, this))
}

// Translated from 006f30b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapArray<PathSmoother::AvoidNode,1024>`'s scalar deleting
/// destructor (engine map; `thiscall`; flags; `RET 4`): [`fn_006f2e40`],
/// then freed on bit 0.
pub fn bs_scrap_array_avoid_node_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    scalar_delete(e, this, flags, |e| fn_006f2e40(e, this))
}

// Translated from 006f31c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<PathSmoother::AvoidNode,1024>`'s constructor
/// (`thiscall`, plain `RET`; no map name): the vtable `0106d88c`, then
/// `006f3320(0, 0)`. Returns `this`.
pub fn fn_006f31c0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106D88C);
    e.call(ARRAY_INIT_006F3320, &args![this, 0u32, 0u32]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006f_2e00, fn_006f2e00(Ptr)),
        entry!(0x006f_2e20, fn_006f2e20(Ptr, u32, u32) -> u32),
        entry!(0x006e_d220, path_smoother_destructor(Ptr)),
        entry!(0x006e_d4e0, fn_006ed4e0(Ptr, u32) -> Ptr),
        entry!(
            0x006f_2c10,
            bs_scrap_array_pathing_location_allocate(Ptr, u32) -> u32
        ),
        entry!(
            0x006f_2c40,
            bs_scrap_array_pathing_location_reallocate(Ptr, u32, u32) -> u32
        ),
        entry!(0x006f_2ca0, fn_006f2ca0(Ptr, u32) -> Ptr),
        entry!(0x006f_2d20, fn_006f2d20(Ptr, Ptr) -> u32),
        entry!(0x006f_2d80, fn_006f2d80(Ptr) -> Ptr),
        entry!(0x006f_2e40, fn_006f2e40(Ptr)),
        entry!(0x006f_2ea0, bs_scrap_array_avoid_node_allocate(Ptr, u32) -> u32),
        entry!(
            0x006f_2ed0,
            bs_scrap_array_avoid_node_reallocate(Ptr, u32, u32) -> u32
        ),
        entry!(
            0x006f_2f30,
            bs_simple_array_path_edge_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x006f_2f60,
            bs_scrap_array_path_edge_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x006f_2f90,
            bs_simple_array_unique_vert_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x006f_2fc0,
            bs_scrap_array_unique_vert_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x006f_2ff0,
            bs_simple_array_bounds_edge_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x006f_3020,
            bs_scrap_array_bounds_edge_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x006f_3050,
            bs_scrap_array_pathing_location_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x006f_3080,
            bs_simple_array_avoid_node_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x006f_30b0,
            bs_scrap_array_avoid_node_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x006f_31c0, fn_006f31c0(Ptr) -> Ptr),
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
    fn fn_006f2e00_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x006f_2e00, ARRAY_VTABLE_0106D88C, ARRAY_RELEASE_008454F0);
    }

    #[test]
    fn fn_006f2e20_reallocates_sixteen_bytes_per_element() {
        let mut e = Engine::new();
        e.register(BUFFER_REALLOCATE_0042F5D0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x006f_2e20, &args![0x1234u32, 0x0abc_0000u32, 5u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|c| c.0 == BUFFER_REALLOCATE_0042F5D0)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x0abc_0000, 0x50]);
    }

    type Log = Vec<(u32, Vec<u32>)>;

    fn run(e: &mut Engine, addr: u32, words: &[u32]) -> (Ret, Log) {
        e.call_log = Some(vec![]);
        let result = e.call(addr, words);
        let log = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != addr)
            .collect();
        (result, log)
    }

    #[test]
    fn scalar_deleting_destructors_run_their_body_then_free_on_bit_0() {
        for (addr, body) in [
            (0x006e_d4e0u32, DESTRUCT_006ED510),
            (0x006f_2f30, DESTRUCT_006F25B0),
            (0x006f_2f60, MEMBER_DESTRUCT_006F25D0),
            (0x006f_2f90, DESTRUCT_006F26D0),
            (0x006f_2fc0, MEMBER_DESTRUCT_006F26F0),
            (0x006f_2ff0, DESTRUCT_006F27F0),
            (0x006f_3020, MEMBER_DESTRUCT_006F2810),
            (0x006f_3050, MEMBER_DESTRUCT_006F2BB0),
        ] {
            let mut e = Engine::new();
            e.register(body, |_, _| Ret::default());
            e.register(FREE_BLOCK, |_, _| Ret::default());
            let this = e.mem.alloc(0x14);
            let (result, log) = run(&mut e, addr, &[this, 0x2]);
            assert_eq!(result.u32(), this);
            assert_eq!(log, vec![(body, vec![this])]);
            let (_, log) = run(&mut e, addr, &[this, 0x3]);
            assert_eq!(log, vec![(body, vec![this]), (FREE_BLOCK, vec![this])]);
        }
    }

    #[test]
    fn avoid_node_scalar_deleting_destructors_run_the_array_destructors() {
        let mut e = Engine::new();
        e.register(ARRAY_RELEASE_008454F0, |_, _| Ret::default());
        e.register(FREE_BLOCK, |_, _| Ret::default());
        let this = e.mem.alloc(0x14);
        let (result, log) = run(&mut e, 0x006f_3080, &[this, 1]);
        assert_eq!(result.u32(), this);
        assert_eq!(
            log,
            vec![
                (ARRAY_RELEASE_008454F0, vec![this, 1]),
                (FREE_BLOCK, vec![this])
            ]
        );
        assert_eq!(e.mem.u32(this), ARRAY_VTABLE_0106D88C);
        let (_, log) = run(&mut e, 0x006f_30b0, &[this, 0]);
        // The scrap array's own release, then the simple array's.
        assert_eq!(
            log,
            vec![
                (ARRAY_RELEASE_008454F0, vec![this, 1]),
                (ARRAY_RELEASE_008454F0, vec![this, 1]),
            ]
        );
        assert_eq!(e.mem.u32(this), ARRAY_VTABLE_0106D88C);
    }

    #[test]
    fn fn_006f2e40_stores_its_vtable_before_releasing() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x14);
        e.register_double(ARRAY_RELEASE_008454F0, move |e, _| {
            // Records the vtable seen at each release at +0x10.
            let seen = e.mem.u32(this);
            let count = e.mem.u32(this + 4);
            e.mem.set_u32(this + 8 + 4 * count, seen);
            e.mem.set_u32(this + 4, count + 1);
            Ret::default()
        });
        e.call(0x006f_2e40, &args![this]);
        assert_eq!(e.mem.u32(this + 8), SCRAP_ARRAY_VTABLE_0106D878);
        assert_eq!(e.mem.u32(this + 0xc), ARRAY_VTABLE_0106D88C);
    }

    /// A scrap array with its heap at `+0x10` and doubles for the heap.
    fn scrap_array(e: &mut Engine) -> u32 {
        e.map(SCRAP_ALIGNMENT & !0xfff, 0x1000);
        e.set_global(SCRAP_ALIGNMENT, 4u32);
        e.register(SCRAP_HEAP_ALLOCATE, |_, a| (0x9000 + a[1]).into_ret());
        e.register(SCRAP_HEAP_DEALLOCATE, |_, _| Ret::default());
        e.register(COPY_BYTES_00401460, |_, _| Ret::default());
        e.register(COUNT_0044DDC0, |_, _| 3u32.into_ret());
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this + 0x10, 0x7700);
        this
    }

    #[test]
    fn scrap_array_allocations_use_the_element_size() {
        for (allocate, reallocate, size) in [
            (0x006f_2c10u32, 0x006f_2c40u32, 0x28u32),
            (0x006f_2ea0, 0x006f_2ed0, 0x10),
        ] {
            let mut e = Engine::new();
            let this = scrap_array(&mut e);
            let (result, log) = run(&mut e, allocate, &[this, 5]);
            assert_eq!(result.u32(), 0x9000 + 5 * size);
            assert_eq!(log, vec![(SCRAP_HEAP_ALLOCATE, vec![0x7700, 5 * size, 4])]);
            let (result, log) = run(&mut e, reallocate, &[this, 0x5000, 8]);
            assert_eq!(result.u32(), 0x9000 + 8 * size);
            assert_eq!(
                log,
                vec![
                    (SCRAP_HEAP_ALLOCATE, vec![0x7700, 8 * size, 4]),
                    (COUNT_0044DDC0, vec![this]),
                    (
                        COPY_BYTES_00401460,
                        vec![0x9000 + 8 * size, 0x5000, 3 * size]
                    ),
                    (SCRAP_HEAP_DEALLOCATE, vec![0x7700, 0x5000]),
                ]
            );
        }
    }

    #[test]
    fn scrap_array_constructors_take_the_thread_heap() {
        let mut e = Engine::new();
        e.register(MEMORY_MANAGER, |_, _| 0x6600u32.into_ret());
        e.register(GET_THREAD_SCRAP_HEAP, |_, a| (a[0] + 1).into_ret());
        e.register(ARRAY_CONSTRUCT_006D0420, |_, a| a[0].into_ret());
        e.register(ARRAY_INIT_0069B940, |_, _| Ret::default());
        e.register(ARRAY_INIT_006F3320, |_, _| Ret::default());
        let this = e.mem.alloc(0x14);
        let (result, log) = run(&mut e, 0x006f_2ca0, &[this, 7]);
        assert_eq!(result.u32(), this);
        assert_eq!(e.mem.u32(this), SCRAP_BOOL_ARRAY_VTABLE_0106C990);
        assert_eq!(e.mem.u32(this + 0x10), 0x6601);
        assert_eq!(
            log,
            vec![
                (ARRAY_CONSTRUCT_006D0420, vec![this]),
                (MEMORY_MANAGER, vec![]),
                (GET_THREAD_SCRAP_HEAP, vec![0x6600]),
                (ARRAY_INIT_0069B940, vec![this, 7, 7]),
            ]
        );
        let this = e.mem.alloc(0x14);
        let (result, log) = run(&mut e, 0x006f_2d80, &[this]);
        assert_eq!(result.u32(), this);
        assert_eq!(e.mem.u32(this), SCRAP_ARRAY_VTABLE_0106D878);
        assert_eq!(e.mem.u32(this + 0x10), 0x6601);
        assert_eq!(
            log,
            vec![
                (ARRAY_INIT_006F3320, vec![this, 0, 0]),
                (MEMORY_MANAGER, vec![]),
                (GET_THREAD_SCRAP_HEAP, vec![0x6600]),
                (ARRAY_INIT_006F3320, vec![this, 0, 0]),
            ]
        );
        let this = e.mem.alloc(0x14);
        let (result, log) = run(&mut e, 0x006f_31c0, &[this]);
        assert_eq!(result.u32(), this);
        assert_eq!(e.mem.u32(this), ARRAY_VTABLE_0106D88C);
        assert_eq!(log, vec![(ARRAY_INIT_006F3320, vec![this, 0, 0])]);
    }

    #[test]
    fn fn_006f2d20_appends_sixteen_bytes() {
        let mut e = Engine::new();
        e.register(ADD_SLOT_006F31F0, |_, _| 2u32.into_ret());
        e.register(CONSTRUCT_RANGE_006F3280, |_, _| Ret::default());
        let this = e.mem.alloc(0x14);
        let buffer = e.mem.alloc(0x40);
        e.mem.set_u32(this + 4, buffer);
        let item = e.mem.alloc(0x10);
        for i in 0..4u32 {
            e.mem.set_u32(item + 4 * i, 0x100 + i);
        }
        let (result, log) = run(&mut e, 0x006f_2d20, &[this, item]);
        assert_eq!(result.u32(), 2);
        assert_eq!(
            log,
            vec![
                (ADD_SLOT_006F31F0, vec![this]),
                (CONSTRUCT_RANGE_006F3280, vec![this, buffer + 0x20, 1]),
            ]
        );
        for i in 0..4u32 {
            assert_eq!(e.mem.u32(buffer + 0x20 + 4 * i), 0x100 + i);
        }
    }

    #[test]
    fn path_smoother_destructor_releases_and_destroys_the_members() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xb8);
        // Each array holds one element pointer: the array's address plus
        // 0x1000.
        e.register(COUNT_0044DDC0, |_, _| 1u32.into_ret());
        e.register(ELEMENT_00877A30, |e, a| {
            let slot = e.mem.alloc(4);
            e.mem.set_u32(slot, a[0] + 0x1000);
            slot.into_ret()
        });
        e.register(ARRAY_CHECK_0076B610, |_, _| 0x100u32.into_ret());
        for addr in [
            SCRAP_HEAP_DEALLOCATE,
            MOVE_ITEMS_009A4320,
            ARRAY_RELEASE_008454F0,
            DESTRUCT_006ED510,
            MEMBER_DESTRUCT_006F2BB0,
            MEMBER_DESTRUCT_006F25D0,
            MEMBER_DESTRUCT_006F2810,
            MEMBER_DESTRUCT_006F26F0,
            MEMBER_DESTRUCT_00982610,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        let (_, log) = run(&mut e, 0x006e_d220, &[this]);
        let calls: Log = log
            .into_iter()
            .filter(|(a, _)| *a != COUNT_0044DDC0 && *a != ELEMENT_00877A30)
            .collect();
        let element = |offset: u32| this + offset + 0x1000;
        let heap = this + 0x30;
        assert_eq!(
            calls,
            vec![
                (DESTRUCT_006ED510, vec![element(0x40)]),
                (SCRAP_HEAP_DEALLOCATE, vec![heap, element(0x40)]),
                (ARRAY_CHECK_0076B610, vec![this + 0x54]),
                (MOVE_ITEMS_009A4320, vec![this + 0x54, 0, 1]),
                (ARRAY_CHECK_0076B610, vec![this + 0x54]),
                (MOVE_ITEMS_009A4320, vec![this + 0x54, 0, 1]),
                (DESTRUCT_006ED510, vec![element(0x54)]),
                (SCRAP_HEAP_DEALLOCATE, vec![heap, element(0x54)]),
                (ARRAY_RELEASE_008454F0, vec![this + 0x40, 1]),
                (ARRAY_RELEASE_008454F0, vec![this + 0x54, 1]),
                (SCRAP_HEAP_DEALLOCATE, vec![heap, element(0x68)]),
                (SCRAP_HEAP_DEALLOCATE, vec![heap, element(0x7c)]),
                (ARRAY_RELEASE_008454F0, vec![this + 0x68, 1]),
                (ARRAY_RELEASE_008454F0, vec![this + 0x7c, 1]),
                (SCRAP_HEAP_DEALLOCATE, vec![heap, element(0x90)]),
                (ARRAY_RELEASE_008454F0, vec![this + 0x90, 1]),
                (MEMBER_DESTRUCT_006F2BB0, vec![this + 0xa4]),
                (MEMBER_DESTRUCT_006F25D0, vec![this + 0x90]),
                (MEMBER_DESTRUCT_006F2810, vec![this + 0x7c]),
                (MEMBER_DESTRUCT_006F2810, vec![this + 0x68]),
                (MEMBER_DESTRUCT_006F26F0, vec![this + 0x54]),
                (MEMBER_DESTRUCT_006F26F0, vec![this + 0x40]),
                (MEMBER_DESTRUCT_00982610, vec![this + 0x30]),
            ]
        );
        // A set 0076b610 skips the moves.
        e.register(ARRAY_CHECK_0076B610, |_, _| 1u32.into_ret());
        let (_, log) = run(&mut e, 0x006e_d220, &[this]);
        assert!(!log.iter().any(|(a, _)| *a == MOVE_ITEMS_009A4320));
    }

    #[test]
    fn fn_006f2e20_answers_the_new_buffer() {
        let mut e = Engine::new();
        e.register(BUFFER_REALLOCATE_0042F5D0, |_, _| 0x0abd_0000u32.into_ret());
        assert_eq!(
            e.call(0x006f_2e20, &args![0x1234u32, 0x0abc_0000u32, 5u32])
                .u32(),
            0x0abd_0000
        );
    }
}
