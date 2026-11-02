//! `fallout shared/tesobjectcell.cpp` (Xbox PDB source unit), part 4: its functions from `00558e20` up to
//! (not including) `ffffffff` in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The unit's shared layouts and helpers are in
//! [`super::tesobjectcell`]; anything public there may be used here.
//!
//! The first session of this part translated the first 40 open or traced
//! functions, `00558e20` to `00559d90`. They are the template instances the
//! compiler emitted in this unit for the cell's loaded-data maps and
//! arrays: the `NiTMap`/`NiTMapBase` instances (`TESObjectREFR *` to
//! `NiNode *`, `TESForm *` to `TESObjectREFR *`, `TESObjectREFR *` to
//! `NiPointer<BSMultiBoundNode>` and to `TESObjectREFR *`), the
//! `BSMapBase`/`BSScrapMap` of `TESObjectREFR *` to `bool`, the
//! `NiTArray`/`NiTPrimitiveArray` of `TESObjectREFR *`, a pointer list, and
//! the `NiTMapBase<NiAVObject *, TESObjectCELL::QUEUED_ATTACH>` operations
//! (`SetAt`, `GetNext`, the value assignment). The second session
//! translated the remaining 15 (`00559df0` to `0055a220`: the rest of that
//! queued-attach map and its allocator); the range is complete.
//!
//! The map classes share one layout (vtable, bucket count, bucket array,
//! item count; [`NiTPointerMap`]) and one vtable shape (Xbox PDB): slot
//! `+0x00` the scalar deleting destructor, `+0x04` `KeyToHashIndex`, `+0x08`
//! `IsKeysEqual`, `+0x0C` `SetValue`, `+0x10` `ClearValue`, `+0x14`
//! `NewItem`, `+0x18` `DeleteItem`. An item starts with its next pointer
//! and its key.
//!
//! Callees named here by what their bodies do: `00438af0` removes every
//! item of a map (`ClearValue` then `DeleteItem` of each); `00558f80`,
//! `00559080`, `00559230` and `00559df0` restore the base class vtable of
//! the map they are called on and free its bucket array; `00403d30` is a
//! `memset` wrapper `(destination, value, size)` and `00ec61c0` the CRT
//! `memset`; `00aa1070` and `00401000` allocate a block (cdecl size),
//! `00401030` frees one; `006e5cc0` is `NiPointer::operator=` (`this`,
//! address of the other pointer) and `0040f6e0` adds a reference to an
//! `NiRefObject`.
//!
//! Not translated: the C++ exception-unwinding frames (`FS:[0]` chains and
//! the stack cookie) of the destructors, constructors and `SetAt` that have
//! them.

#[allow(unused_imports)]
use super::tesobjectcell::*;
#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{NiTArray, NiTListItem, NiTPointerList, NiTPointerMap};

/// Frees a block (cdecl, the block).
const DEALLOCATE: u32 = 0x0040_1030;
/// Allocates a block (cdecl, the size).
const ALLOCATE: u32 = 0x0040_1000;
/// Allocates a block through the memory manager singleton (cdecl, the size).
const ALLOCATE_MANAGED: u32 = 0x00aa_1070;
/// `memset` wrapper (cdecl `destination`, `value`, `size`) and the CRT
/// `memset` it calls.
const FILL_BYTES: u32 = 0x0040_3d30;
const CRT_MEMSET: u32 = 0x00ec_61c0;
/// Removes every item of a map: `ClearValue` then `DeleteItem` of each.
const MAP_REMOVE_ALL: u32 = 0x0043_8af0;
/// Base destructors of the map instances: restore the base vtable and free
/// the bucket array.
const MAP_BASE_DESTRUCT_REFERENCE_NODE: u32 = 0x0055_8f80;
const MAP_BASE_DESTRUCT_FORM_REFERENCE: u32 = 0x0055_9080;
const MAP_BASE_DESTRUCT_MULTI_BOUND: u32 = 0x0055_9230;
const MAP_BASE_DESTRUCT_REFERENCE_REFERENCE: u32 = 0x0055_9df0;
/// `NiPointer::operator=` (`this` = the slot, the other slot's address).
const NI_POINTER_ASSIGN: u32 = 0x006e_5cc0;
/// Adds a reference to an `NiRefObject` (`this`).
const REFERENCE_ADD: u32 = 0x0040_f6e0;
/// Copy constructor of `TESObjectCELL::QUEUED_ATTACH` (`this` = the new
/// value, the source's address) and its destructor (`this`).
const QUEUED_ATTACH_COPY: u32 = 0x0054_adb0;
const QUEUED_ATTACH_DESTRUCT: u32 = 0x0062_08d0;
/// Frees an `NiTArray` buffer (cdecl, the buffer).
const ARRAY_BUFFER_FREE: u32 = 0x004e_de70;
/// Allocates an `NiTArray` buffer of `count` elements (cdecl, the count).
const ARRAY_BUFFER_ALLOCATE: u32 = 0x0096_afc0;
/// Releases the object a Havok smart pointer slot points to (`this` = the
/// object).
const OBJECT_RELEASE: u32 = 0x00c9_05b0;
/// `ScrapHeap` constructor, destructor, `Allocate(size, alignment)` and
/// `Deallocate(block)` (`this` = the heap).
const SCRAP_HEAP_CONSTRUCT: u32 = 0x00aa_53f0;
const SCRAP_HEAP_DESTRUCT: u32 = 0x00aa_5460;
const SCRAP_HEAP_ALLOCATE: u32 = 0x00aa_54a0;
const SCRAP_HEAP_DEALLOCATE: u32 = 0x00aa_5610;
/// The alignment word the `ScrapHeap` allocations of this unit pass.
const SCRAP_ALIGNMENT: u32 = 0x010a_2720;
/// Allocates a node of a pointer list (`this` = the list).
const LIST_ALLOCATE_NODE: u32 = 0x0047_05c0;
/// Frees a node to a list's allocator (`this` = the allocator at `list +
/// 8`, the node).
const LIST_FREE_NODE: u32 = 0x006b_8310;
/// Base destructor run by the destructor of the class that owns the list.
const LIST_OWNER_BASE_DESTRUCT: u32 = 0x0048_3710;
/// Destructor body of the class `fn_00559970` deletes (a function of this
/// unit outside this part).
const CELL_HELPER_DESTRUCT: u32 = 0x0054_22b0;

/// Frees a block through the memory manager singleton (cdecl, the block).
const DEALLOCATE_MANAGED: u32 = 0x00aa_10f0;
/// Constructor of a smart pointer slot (`this` = the slot, the object to
/// hold: 0 here).
const POINTER_CONSTRUCT: u32 = 0x0063_3c90;
/// Item allocator of the queued-attach map (the `DFALL` allocator at
/// `map + 0xC`): constructor body `0055a270`, and the deleting destructor
/// `0055a240` (`this` = the item, a flags word).
const ITEM_ALLOCATOR_CONSTRUCT: u32 = 0x0055_a270;
const ITEM_ALLOCATOR_DELETE: u32 = 0x0055_a240;

/// Vtables of the map instances. `NiTMapBase` is the base class and `NiTMap`
/// the class derived from it (Xbox PDB names).
const NI_T_MAP_REFERENCE_NODE_VTABLE: u32 = 0x0102_f2f8;
const NI_T_MAP_FORM_REFERENCE_VTABLE: u32 = 0x0102_f318;
const NI_T_MAP_MULTI_BOUND_VTABLE: u32 = 0x0102_f338;
const NI_T_MAP_BASE_REFERENCE_NODE_VTABLE: u32 = 0x0102_f358;
const NI_T_MAP_BASE_FORM_REFERENCE_VTABLE: u32 = 0x0102_f378;
const NI_T_MAP_BASE_MULTI_BOUND_VTABLE: u32 = 0x0102_f398;
const NI_T_ARRAY_VTABLE: u32 = 0x0102_f3b8;
const NI_T_PRIMITIVE_ARRAY_VTABLE: u32 = 0x0102_f3c0;
const BS_SCRAP_MAP_VTABLE: u32 = 0x0102_f3c8;
const BS_MAP_BASE_VTABLE: u32 = 0x0102_f3e8;
const NI_T_MAP_REFERENCE_REFERENCE_VTABLE: u32 = 0x0102_f408;
const NI_T_MAP_BASE_REFERENCE_REFERENCE_VTABLE: u32 = 0x0102_f428;
const NI_T_MAP_QUEUED_ATTACH_VTABLE: u32 = 0x0102_f448;
const NI_T_MAP_BASE_QUEUED_ATTACH_VTABLE: u32 = 0x0102_f468;

/// Byte offsets of the map vtable slots (Xbox PDB order).
const MAP_SLOT_KEY_TO_HASH_INDEX: u32 = 0x04;
const MAP_SLOT_IS_KEYS_EQUAL: u32 = 0x08;
const MAP_SLOT_SET_VALUE: u32 = 0x0c;
const MAP_SLOT_CLEAR_VALUE: u32 = 0x10;
const MAP_SLOT_NEW_ITEM: u32 = 0x14;

layout! {
    /// `TESObjectCELL::QUEUED_ATTACH` (Xbox PDB), 0xC bytes: the value of the
    /// cell's queued-attach map.
    pub struct QueuedAttach: 0x0c {
        /// `spAttachChild` (Xbox PDB): `NiPointer<NiAVObject>`.
        0x00 spAttachChild: u32,
        /// `spAttachNode` (Xbox PDB): `NiPointer<NiNode>`.
        0x04 spAttachNode: u32,
        /// `bFirstAvail` (Xbox PDB).
        0x08 bFirstAvail: u8,
    }

    /// The `ScrapHeap` of a `BSScrapMap` (Xbox PDB), 0x14 bytes.
    pub struct ScrapHeap: 0x14 {
    }

    /// `BSScrapMap<K, V>` (Xbox PDB), 0x28 bytes: the `BSMapBase` fields
    /// (vtable, bucket count, bucket array, item count; the layout of
    /// [`NiTPointerMap`]) then these.
    pub struct BSScrapMap: 0x28 {
        /// `m_uiHashSize` (Xbox PDB).
        0x04 m_uiHashSize: u32,
        /// `m_ppkHashTable` (Xbox PDB).
        0x08 m_ppkHashTable: u32,
        /// `m_uiCount` (Xbox PDB).
        0x0C m_uiCount: u32,
        /// `bClearOnDestruct` (Xbox PDB).
        0x10 bClearOnDestruct: u8,
        /// `MapHeap` (Xbox PDB): the heap the items are allocated from.
        0x14 MapHeap: Inline<ScrapHeap>,
    }
}

/// Frees `this` when bit 0 of `flags` is set: the end of every scalar
/// deleting destructor.
fn free_if_requested(e: &mut Engine, this: Ptr, flags: u32) {
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
}

/// The constructor body shared by the `NiTMapBase` instances of this part:
/// the bucket array is allocated and zeroed.
fn construct_map_base(e: &mut Engine, this: Ptr<NiTPointerMap>, vtable: u32, hash_size: u32) {
    e.mem.set_u32(this.addr(), vtable);
    e.set(this, NiTPointerMap::m_uiHashSize, hash_size);
    e.set(this, NiTPointerMap::m_uiCount, 0);
    let bytes = hash_size.wrapping_mul(4);
    let table = e.call(ALLOCATE_MANAGED, &args![bytes]).u32();
    e.set(this, NiTPointerMap::m_ppkHashTable, table);
    let size = e.get(this, NiTPointerMap::m_uiHashSize).wrapping_mul(4);
    e.call(FILL_BYTES, &args![table, 0u32, size]);
}

/// The destructor body shared by the `NiTMap` instances of this part: sets
/// the vtable, removes every item, then runs the base destructor `base`.
fn destruct_map(e: &mut Engine, this: Ptr, vtable: u32, base: u32) {
    e.mem.set_u32(this.addr(), vtable);
    e.call(MAP_REMOVE_ALL, &args![this]);
    e.call(base, &args![this]);
}

// Translated from 00558e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<TESForm *, TESObjectREFR *>::scalar deleting destructor` (Xbox
/// PDB): runs the destructor `fn_00559020`, then frees the map when bit 0 of
/// `flags` is set. Returns `this`.
pub fn fn_00558e20(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00559020(e, this);
    free_if_requested(e, this, flags);
    this
}

// Translated from 00558e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<TESObjectREFR *, NiPointer<BSMultiBoundNode> >::scalar deleting
/// destructor` (Xbox PDB): runs the destructor `fn_005591d0`, then frees the
/// map when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00558e50(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_005591d0(e, this);
    free_if_requested(e, this, flags);
    this
}

// Translated from 00558e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases the object the slot at `this` points to (`00c905b0`), if any,
/// and clears the slot. The compiler shares this body between
/// `Concurrency::cancellation_token::_Clear` and the Havok smart pointer
/// release.
pub fn fn_00558e80(e: &mut Engine, this: Ptr) {
    let object = e.mem.u32(this.addr());
    if object != 0 {
        e.call(OBJECT_RELEASE, &args![object]);
    }
    e.mem.set_u32(this.addr(), 0);
}

// Translated from 00558eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase` constructor of the `TESObjectREFR *` to `NiNode *` map
/// (the base class of the map `fn_00558f20` destroys): sets the base vtable,
/// the bucket count and an allocated, zeroed bucket array. Returns `this`.
pub fn fn_00558eb0(e: &mut Engine, this: Ptr<NiTPointerMap>, hash_size: u32) -> Ptr<NiTPointerMap> {
    construct_map_base(e, this, NI_T_MAP_BASE_REFERENCE_NODE_VTABLE, hash_size);
    this
}

// Translated from 00558f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<TESObjectREFR *, NiNode *>` destructor: sets the vtable, removes
/// every item (`00438af0`) and runs the base destructor `00558f80`. The
/// exception-unwinding frame is not translated.
pub fn fn_00558f20(e: &mut Engine, this: Ptr) {
    destruct_map(
        e,
        this,
        NI_T_MAP_REFERENCE_NODE_VTABLE,
        MAP_BASE_DESTRUCT_REFERENCE_NODE,
    );
}

// Translated from 00558fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase` constructor of the `TESForm *` to `TESObjectREFR *` map:
/// like `fn_00558eb0` with that map's base vtable. Returns `this`.
pub fn fn_00558fb0(e: &mut Engine, this: Ptr<NiTPointerMap>, hash_size: u32) -> Ptr<NiTPointerMap> {
    construct_map_base(e, this, NI_T_MAP_BASE_FORM_REFERENCE_VTABLE, hash_size);
    this
}

// Translated from 00559020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<TESForm *, TESObjectREFR *>` destructor: sets the vtable, removes
/// every item and runs the base destructor `00559080`. The
/// exception-unwinding frame is not translated.
pub fn fn_00559020(e: &mut Engine, this: Ptr) {
    destruct_map(
        e,
        this,
        NI_T_MAP_FORM_REFERENCE_VTABLE,
        MAP_BASE_DESTRUCT_FORM_REFERENCE,
    );
}

// Translated from 005590b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase` constructor of the `TESObjectREFR *` to
/// `NiPointer<BSMultiBoundNode>` map: like `fn_00558eb0` with that map's
/// base vtable. Returns `this`.
pub fn fn_005590b0(e: &mut Engine, this: Ptr<NiTPointerMap>, hash_size: u32) -> Ptr<NiTPointerMap> {
    construct_map_base(e, this, NI_T_MAP_BASE_MULTI_BOUND_VTABLE, hash_size);
    this
}

// Translated from 005591d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<TESObjectREFR *, NiPointer<BSMultiBoundNode> >` destructor: sets
/// the vtable, removes every item and runs the base destructor `00559230`.
/// The exception-unwinding frame is not translated.
pub fn fn_005591d0(e: &mut Engine, this: Ptr) {
    destruct_map(
        e,
        this,
        NI_T_MAP_MULTI_BOUND_VTABLE,
        MAP_BASE_DESTRUCT_MULTI_BOUND,
    );
}

// Translated from 00559260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SetAt` of the `NiTMapBase<..., NiAVObject *,
/// TESObjectCELL::QUEUED_ATTACH>` instance (the Xbox PDB names `SetValue`
/// at `00559f20` for the same map): the value is passed by value as three
/// words (child, node, flag) and destroyed by the function. The bucket is
/// found through virtual slot `+0x04`; an item whose key `IsKeysEqual`
/// (slot `+0x08`) accepts gets the value assigned (`fn_00559900`);
/// otherwise a new item (`NewItem`, slot `+0x14`) is filled by `SetValue`
/// (slot `+0x0C`, with a copy of the value), pushed at the head of the
/// bucket and counted. The exception-unwinding frame is not translated.
pub fn fn_00559260(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    key: u32,
    value_child: u32,
    value_node: u32,
    value_flag: u32,
) {
    e.with_stack(12, |e, value| {
        e.mem.set_u32(value.addr(), value_child);
        e.mem.set_u32(value.addr() + 4, value_node);
        e.mem.set_u32(value.addr() + 8, value_flag);
        let index = e
            .vcall(this.addr(), MAP_SLOT_KEY_TO_HASH_INDEX, &args![key])
            .u32();
        let table = e.get(this, NiTPointerMap::m_ppkHashTable);
        let mut item = e.mem.u32(table.wrapping_add(index.wrapping_mul(4)));
        while item != 0 {
            let item_key = e.mem.u32(item + 4);
            if e.vcall(this.addr(), MAP_SLOT_IS_KEYS_EQUAL, &args![key, item_key])
                .bool()
            {
                fn_00559900(e, Ptr::new(item + 8), value.cast());
                e.call(QUEUED_ATTACH_DESTRUCT, &args![value]);
                return;
            }
            item = e.mem.u32(item);
        }
        let item = e.vcall(this.addr(), MAP_SLOT_NEW_ITEM, &args![]).u32();
        let copy = e.with_stack(12, |e, copy| {
            e.call(QUEUED_ATTACH_COPY, &args![copy, value]);
            [
                e.mem.u32(copy.addr()),
                e.mem.u32(copy.addr() + 4),
                e.mem.u32(copy.addr() + 8),
            ]
        });
        e.vcall(
            this.addr(),
            MAP_SLOT_SET_VALUE,
            &args![item, key, copy[0], copy[1], copy[2]],
        );
        let table = e.get(this, NiTPointerMap::m_ppkHashTable);
        let slot = table.wrapping_add(index.wrapping_mul(4));
        let head = e.mem.u32(slot);
        e.mem.set_u32(item, head);
        e.mem.set_u32(slot, item);
        let count = e.get(this, NiTPointerMap::m_uiCount);
        e.set(this, NiTPointerMap::m_uiCount, count.wrapping_add(1));
        e.call(QUEUED_ATTACH_DESTRUCT, &args![value]);
    });
}

// Translated from 005593a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GetNext` of the `NiTMapBase<..., NiAVObject *,
/// TESObjectCELL::QUEUED_ATTACH>` instance: `position` is the address of
/// the iteration slot holding the current item. Writes the item's key to
/// `key_out`, assigns its value to the value at `value_out`
/// (`fn_00559900`) and moves the slot to the next item of the bucket or, at
/// the end of the bucket, to the first item of the next non-empty bucket
/// (0 after the last).
pub fn fn_005593a0(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    position: Ptr,
    key_out: Ptr,
    value_out: Ptr,
) {
    let item = e.mem.u32(position.addr());
    let key = e.mem.u32(item + 4);
    e.mem.set_u32(key_out.addr(), key);
    fn_00559900(e, value_out.cast(), Ptr::new(item + 8));
    let next = e.mem.u32(item);
    if next != 0 {
        e.mem.set_u32(position.addr(), next);
        return;
    }
    let item_key = e.mem.u32(item + 4);
    let mut index = e
        .vcall(this.addr(), MAP_SLOT_KEY_TO_HASH_INDEX, &args![item_key])
        .u32()
        .wrapping_add(1);
    while index < e.get(this, NiTPointerMap::m_uiHashSize) {
        let table = e.get(this, NiTPointerMap::m_ppkHashTable);
        let candidate = e.mem.u32(table.wrapping_add(index.wrapping_mul(4)));
        if candidate != 0 {
            e.mem.set_u32(position.addr(), candidate);
            return;
        }
        index = index.wrapping_add(1);
    }
    e.mem.set_u32(position.addr(), 0);
}

// Translated from 00559460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<TESObjectREFR *, NiTMallocInterface<int> >` destructor (the
/// compiler shares the body with the streambuf destructors): sets the
/// vtable and frees the element buffer.
pub fn fn_00559460(e: &mut Engine, this: Ptr<NiTArray>) {
    e.mem.set_u32(this.addr(), NI_T_ARRAY_VTABLE);
    let buffer = e.get(this, NiTArray::m_pBase);
    e.call(ARRAY_BUFFER_FREE, &args![buffer]);
}

// Translated from 005594b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPrimitiveArray<TESObjectREFR *>` constructor: the `NiTArray`
/// constructor (`fn_00559ad0`) with `max_size` and `grow_by`, then the
/// primitive array's vtable. Returns `this`.
pub fn fn_005594b0(
    e: &mut Engine,
    this: Ptr<NiTArray>,
    max_size: u16,
    grow_by: u16,
) -> Ptr<NiTArray> {
    fn_00559ad0(e, this, max_size, grow_by);
    e.mem.set_u32(this.addr(), NI_T_PRIMITIVE_ARRAY_VTABLE);
    this
}

// Translated from 005595e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapMap<TESObjectREFR *, bool>::BSScrapMap` (Xbox PDB): builds the
/// `BSMapBase` part without buckets (`fn_00559b40` with size 0), sets the
/// scrap map vtable, constructs the `ScrapHeap` at `+0x14`, stores the bucket
/// count and the clear-on-destruct flag, allocates the bucket array from
/// that heap (alignment: the word at `010a2720`) and zeroes it. Returns
/// `this`. The exception-unwinding frame is not translated.
pub fn bs_scrap_map_bs_scrap_map(
    e: &mut Engine,
    this: Ptr<BSScrapMap>,
    hash_size: u32,
    clear_on_destruct: u8,
) -> Ptr<BSScrapMap> {
    fn_00559b40(e, this.cast(), 0);
    e.mem.set_u32(this.addr(), BS_SCRAP_MAP_VTABLE);
    let heap = this.at(BSScrapMap::MapHeap);
    e.call(SCRAP_HEAP_CONSTRUCT, &args![heap]);
    e.set(this, BSScrapMap::m_uiHashSize, hash_size);
    e.set(this, BSScrapMap::bClearOnDestruct, clear_on_destruct);
    let bytes = hash_size.wrapping_shl(2);
    let alignment = e.global::<u32>(SCRAP_ALIGNMENT);
    let table = e
        .call(SCRAP_HEAP_ALLOCATE, &args![heap, bytes, alignment])
        .u32();
    e.set(this, BSScrapMap::m_ppkHashTable, table);
    e.call(FILL_BYTES, &args![table, 0u32, bytes]);
    this
}

// Translated from 005596a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMapBase<TESObjectREFR *, bool>` destructor: sets the vtable and frees
/// the bucket array if there is one.
pub fn fn_005596a0(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    e.mem.set_u32(this.addr(), BS_MAP_BASE_VTABLE);
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    if table != 0 {
        e.call(DEALLOCATE, &args![table]);
    }
}

// Translated from 005596e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapMap<TESObjectREFR *, bool>` destructor: sets the vtable, removes
/// the items (`fn_00559bc0`) when `bClearOnDestruct` is set, clears the
/// bucket array, count and bucket count (so the base destructor frees
/// nothing: the buckets belong to the heap), destroys the `ScrapHeap` and
/// runs the `BSMapBase` destructor (`fn_005596a0`). The exception-unwinding
/// frame is not translated.
pub fn fn_005596e0(e: &mut Engine, this: Ptr<BSScrapMap>) {
    e.mem.set_u32(this.addr(), BS_SCRAP_MAP_VTABLE);
    if e.get(this, BSScrapMap::bClearOnDestruct) != 0 {
        fn_00559bc0(e, this.cast());
    }
    e.set(this, BSScrapMap::m_ppkHashTable, 0);
    e.set(this, BSScrapMap::m_uiCount, 0);
    e.set(this, BSScrapMap::m_uiHashSize, 0);
    e.call(SCRAP_HEAP_DESTRUCT, &args![this.at(BSScrapMap::MapHeap)]);
    fn_005596a0(e, this.cast());
}

// Translated from 00559780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapMap<TESObjectREFR *, bool>::NewItem` (Xbox PDB): allocates a
/// 0xC-byte item from the map's `ScrapHeap` (alignment: the word at
/// `010a2720`).
pub fn bs_scrap_map_new_item(e: &mut Engine, this: Ptr<BSScrapMap>) -> Ptr {
    let alignment = e.global::<u32>(SCRAP_ALIGNMENT);
    e.call(
        SCRAP_HEAP_ALLOCATE,
        &args![this.at(BSScrapMap::MapHeap), 0x0cu32, alignment],
    )
    .ptr()
}

// Translated from 005597a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapMap<TESObjectREFR *, bool>::DeleteItem` (Xbox PDB): clears the
/// item's value (`+8`) and gives the item back to the map's `ScrapHeap`.
pub fn bs_scrap_map_delete_item(e: &mut Engine, this: Ptr<BSScrapMap>, item: Ptr) {
    e.mem.set_u8(item.addr() + 8, 0);
    e.call(
        SCRAP_HEAP_DEALLOCATE,
        &args![this.at(BSScrapMap::MapHeap), item],
    );
}

// Translated from 005597d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds a pointer to the end of a pointer list (the `AddTail` of an
/// `NiTPointerList<TESObjectREFR *>`; the name is not confirmed):
/// `element` is the address of the pointer to add. Allocates a node
/// (`004705c0`), stores the pointer in it and appends it (`fn_00559a70`).
pub fn fn_005597d0(e: &mut Engine, this: Ptr<NiTPointerList>, element: Ptr) {
    let node = e.call(LIST_ALLOCATE_NODE, &args![this]).u32();
    let value = e.mem.u32(element.addr());
    e.mem.set_u32(node + 8, value);
    fn_00559a70(e, this, Ptr::new(node));
}

// Translated from 00559810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the class that owns a pointer list at `this`: clears the
/// list (`fn_00559c30`), then runs the base destructor `00483710`. The
/// exception-unwinding frame is not translated.
pub fn fn_00559810(e: &mut Engine, this: Ptr<NiTPointerList>) {
    fn_00559c30(e, this);
    e.call(LIST_OWNER_BASE_DESTRUCT, &args![this]);
}

// Translated from 00559870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<..., TESObjectREFR *, NiNode *>::scalar deleting destructor`
/// (Xbox PDB): runs the base destructor `00558f80`, then frees the map when
/// bit 0 of `flags` is set. Returns `this`.
pub fn fn_00559870(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(MAP_BASE_DESTRUCT_REFERENCE_NODE, &args![this]);
    free_if_requested(e, this, flags);
    this
}

// Translated from 005598a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<..., TESForm *, TESObjectREFR *>::scalar deleting
/// destructor` (Xbox PDB): runs the base destructor `00559080`, then frees
/// the map when bit 0 of `flags` is set. Returns `this`.
pub fn fn_005598a0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(MAP_BASE_DESTRUCT_FORM_REFERENCE, &args![this]);
    free_if_requested(e, this, flags);
    this
}

// Translated from 005598d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<..., TESObjectREFR *, NiPointer<BSMultiBoundNode> >::scalar
/// deleting destructor` (Xbox PDB): runs the base destructor `00559230`,
/// then frees the map when bit 0 of `flags` is set. Returns `this`.
pub fn fn_005598d0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(MAP_BASE_DESTRUCT_MULTI_BOUND, &args![this]);
    free_if_requested(e, this, flags);
    this
}

// Translated from 00559900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::QUEUED_ATTACH::operator=`: assigns both smart pointers
/// (`NiPointer::operator=`, which releases the old object and adds a
/// reference to the new one) and copies the flag byte. Returns `this`.
pub fn fn_00559900(
    e: &mut Engine,
    this: Ptr<QueuedAttach>,
    other: Ptr<QueuedAttach>,
) -> Ptr<QueuedAttach> {
    e.call(NI_POINTER_ASSIGN, &args![this, other]);
    e.call(
        NI_POINTER_ASSIGN,
        &args![this.byte_add(4), other.byte_add(4)],
    );
    let flag = e.get(other, QueuedAttach::bFirstAvail);
    e.set(this, QueuedAttach::bFirstAvail, flag);
    this
}

// Translated from 00559940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the `NiTArray` of `TESObjectREFR *`: runs
/// `fn_00559460`, then frees the array when bit 0 of `flags` is set. Returns
/// `this`.
pub fn fn_00559940(e: &mut Engine, this: Ptr<NiTArray>, flags: u32) -> Ptr<NiTArray> {
    fn_00559460(e, this);
    free_if_requested(e, this.cast(), flags);
    this
}

// Translated from 00559970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the class whose destructor body is
/// `005422b0` (a function of this unit outside this part): runs it, then
/// frees the object when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00559970(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(CELL_HELPER_DESTRUCT, &args![this]);
    free_if_requested(e, this, flags);
    this
}

// Translated from 005599a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMapBase<TESObjectREFR *, bool>::scalar deleting destructor` (Xbox
/// PDB): runs `fn_005596a0`, then frees the map when bit 0 of `flags` is
/// set. Returns `this`.
pub fn fn_005599a0(e: &mut Engine, this: Ptr<NiTPointerMap>, flags: u32) -> Ptr<NiTPointerMap> {
    fn_005596a0(e, this);
    free_if_requested(e, this.cast(), flags);
    this
}

// Translated from 005599d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapMap<TESObjectREFR *, bool>::scalar deleting destructor` (Xbox
/// PDB): runs `fn_005596e0`, then frees the map when bit 0 of `flags` is
/// set. Returns `this`.
pub fn fn_005599d0(e: &mut Engine, this: Ptr<BSScrapMap>, flags: u32) -> Ptr<BSScrapMap> {
    fn_005596e0(e, this);
    free_if_requested(e, this.cast(), flags);
    this
}

// Translated from 00559a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills `count` words at `destination` with the word `value` points to
/// (cdecl; the word is read for every element). Nothing is written for a
/// count of zero or less.
pub fn fn_00559a00(e: &mut Engine, destination: Ptr, count: i32, value: Ptr) {
    let mut index = 0i32;
    while index < count {
        let word = e.mem.u32(value.addr());
        let slot = destination
            .addr()
            .wrapping_add((index as u32).wrapping_mul(4));
        e.mem.set_u32(slot, word);
        index += 1;
    }
}

// Translated from 00559a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPointer` copy constructor: copies the pointer at `other` into `this`
/// and adds a reference to the object when it is not null. Returns `this`.
pub fn fn_00559a40(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    let object = e.mem.u32(other.addr());
    e.mem.set_u32(this.addr(), object);
    if object != 0 {
        e.call(REFERENCE_ADD, &args![object]);
    }
    this
}

// Translated from 00559a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends a node to a pointer list (name not confirmed): links `node`
/// (next 0, previous the old tail) after the tail, makes it the head when
/// the list was empty, makes it the tail and counts it.
pub fn fn_00559a70(e: &mut Engine, this: Ptr<NiTPointerList>, node: Ptr<NiTListItem>) {
    e.set(node, NiTListItem::m_pkNext, 0);
    let tail = e.get(this, NiTPointerList::m_pkTail);
    e.set(node, NiTListItem::m_pkPrev, tail);
    if tail == 0 {
        e.set(this, NiTPointerList::m_pkHead, node.addr());
    } else {
        e.mem.set_u32(tail, node.addr());
    }
    e.set(this, NiTPointerList::m_pkTail, node.addr());
    let count = e.get(this, NiTPointerList::m_uiCount);
    e.set(this, NiTPointerList::m_uiCount, count.wrapping_add(1));
    // `NV_SPEECH_CHECK` (diagnostic only, reads memory): an add to a watched list.
    crate::units::fallout_shared_facegen::bsfacegenkeyframe::speech_check::linked(
        e,
        this.addr(),
        node.addr(),
    );
}

// Translated from 00559ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<TESObjectREFR *, NiTMallocInterface<int> >` constructor: sets
/// the vtable, the capacity and growth step, a size and element count of 0,
/// and allocates the buffer (`0096afc0`) when the capacity is not 0 (a null
/// buffer otherwise). Returns `this`.
pub fn fn_00559ad0(
    e: &mut Engine,
    this: Ptr<NiTArray>,
    max_size: u16,
    grow_by: u16,
) -> Ptr<NiTArray> {
    e.mem.set_u32(this.addr(), NI_T_ARRAY_VTABLE);
    e.set(this, NiTArray::m_usMaxSize, max_size);
    e.set(this, NiTArray::m_usGrowBy, grow_by);
    e.set(this, NiTArray::m_usSize, 0);
    e.set(this, NiTArray::m_usESize, 0);
    if e.get(this, NiTArray::m_usMaxSize) > 0 {
        let capacity = u32::from(e.get(this, NiTArray::m_usMaxSize));
        let buffer = e.call(ARRAY_BUFFER_ALLOCATE, &args![capacity]).u32();
        e.set(this, NiTArray::m_pBase, buffer);
    } else {
        e.set(this, NiTArray::m_pBase, 0);
    }
    this
}

// Translated from 00559b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMapBase<TESObjectREFR *, bool>` constructor: sets the vtable, the
/// bucket count and a count of 0; for a non-zero bucket count allocates the
/// bucket array (`00401000`) and zeroes it. Returns `this`.
pub fn fn_00559b40(e: &mut Engine, this: Ptr<NiTPointerMap>, hash_size: u32) -> Ptr<NiTPointerMap> {
    e.mem.set_u32(this.addr(), BS_MAP_BASE_VTABLE);
    e.set(this, NiTPointerMap::m_uiHashSize, hash_size);
    e.set(this, NiTPointerMap::m_uiCount, 0);
    if hash_size != 0 {
        let bytes = e.get(this, NiTPointerMap::m_uiHashSize).wrapping_shl(2);
        let table = e.call(ALLOCATE, &args![bytes]).u32();
        e.set(this, NiTPointerMap::m_ppkHashTable, table);
        let bytes = e.get(this, NiTPointerMap::m_uiHashSize).wrapping_shl(2);
        e.call(CRT_MEMSET, &args![table, 0u32, bytes]);
    }
    this
}

// Translated from 00559bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMapBase` remove-all: calls `ClearValue` (virtual slot `+0x10`) on
/// every item of every bucket, reading an item's next pointer after the call,
/// and sets the count to 0.
pub fn fn_00559bc0(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    let mut bucket = 0u32;
    while bucket < e.get(this, NiTPointerMap::m_uiHashSize) {
        let table = e.get(this, NiTPointerMap::m_ppkHashTable);
        let mut item = e.mem.u32(table.wrapping_add(bucket.wrapping_mul(4)));
        while item != 0 {
            e.vcall(this.addr(), MAP_SLOT_CLEAR_VALUE, &args![item]);
            item = e.mem.u32(item);
        }
        bucket = bucket.wrapping_add(1);
    }
    e.set(this, NiTPointerMap::m_uiCount, 0);
}

// Translated from 00559c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes every node of a pointer list (the `RemoveAll` of an
/// `NiTPointerList`; the name is not confirmed): frees each node
/// (`fn_00559c90`), then resets the count, head and tail to 0.
pub fn fn_00559c30(e: &mut Engine, this: Ptr<NiTPointerList>) {
    let mut node = e.get(this, NiTPointerList::m_pkHead);
    while node != 0 {
        let current = node;
        node = e.mem.u32(node);
        fn_00559c90(e, this, Ptr::new(current));
    }
    e.set(this, NiTPointerList::m_uiCount, 0);
    e.set(this, NiTPointerList::m_pkHead, 0);
    e.set(this, NiTPointerList::m_pkTail, 0);
}

// Translated from 00559c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases a pointer list node (name not confirmed): clears the node's
/// element and gives the node to the list's allocator (`this + 8`,
/// `006b8310`).
pub fn fn_00559c90(e: &mut Engine, this: Ptr<NiTPointerList>, node: Ptr<NiTListItem>) {
    e.set(node, NiTListItem::m_element, 0);
    e.call(LIST_FREE_NODE, &args![this.byte_add(8), node]);
}

// Translated from 00559cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<TESObjectREFR *, TESObjectREFR *>` constructor: the `NiTMapBase`
/// constructor (`fn_00559d20`) with the bucket count, then the derived
/// vtable. Returns `this`.
pub fn fn_00559cc0(e: &mut Engine, this: Ptr<NiTPointerMap>, hash_size: u32) -> Ptr<NiTPointerMap> {
    fn_00559d20(e, this, hash_size);
    e.mem
        .set_u32(this.addr(), NI_T_MAP_REFERENCE_REFERENCE_VTABLE);
    this
}

// Translated from 00559cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<TESObjectREFR *, TESObjectREFR *>::scalar deleting destructor`
/// (Xbox PDB): runs the destructor `fn_00559d90`, then frees the map when
/// bit 0 of `flags` is set. Returns `this`.
pub fn fn_00559cf0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00559d90(e, this);
    free_if_requested(e, this, flags);
    this
}

// Translated from 00559d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase` constructor of the `TESObjectREFR *` to `TESObjectREFR *`
/// map: like `fn_00558eb0` with that map's base vtable. Returns `this`.
pub fn fn_00559d20(e: &mut Engine, this: Ptr<NiTPointerMap>, hash_size: u32) -> Ptr<NiTPointerMap> {
    construct_map_base(e, this, NI_T_MAP_BASE_REFERENCE_REFERENCE_VTABLE, hash_size);
    this
}

// Translated from 00559d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<TESObjectREFR *, TESObjectREFR *>` destructor: sets the vtable,
/// removes every item and runs the base destructor `00559df0`. The
/// exception-unwinding frame is not translated.
pub fn fn_00559d90(e: &mut Engine, this: Ptr) {
    destruct_map(
        e,
        this,
        NI_T_MAP_REFERENCE_REFERENCE_VTABLE,
        MAP_BASE_DESTRUCT_REFERENCE_REFERENCE,
    );
}

// Translated from 00559df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Base destructor of the `NiTMapBase<TESObjectREFR *, TESObjectREFR *>`
/// instance: sets the base vtable, removes every item (`00438af0`) and frees
/// the bucket array (`00aa10f0`).
pub fn fn_00559df0(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    e.mem
        .set_u32(this.addr(), NI_T_MAP_BASE_REFERENCE_REFERENCE_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    e.call(DEALLOCATE_MANAGED, &args![table]);
}

// Translated from 00559e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<..., TESObjectREFR *, TESObjectREFR *>::scalar deleting
/// destructor` (Xbox PDB): runs the base destructor `fn_00559df0`, then
/// frees the map when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00559e20(e: &mut Engine, this: Ptr<NiTPointerMap>, flags: u32) -> Ptr<NiTPointerMap> {
    fn_00559df0(e, this);
    free_if_requested(e, this.cast(), flags);
    this
}

// Translated from 00559e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<NiAVObject *, TESObjectCELL::QUEUED_ATTACH>` constructor: the
/// `NiTMapBase` constructor (`fn_00559eb0`) with the bucket count, then the
/// derived vtable. Returns `this`.
pub fn fn_00559e50(e: &mut Engine, this: Ptr<NiTPointerMap>, hash_size: u32) -> Ptr<NiTPointerMap> {
    fn_00559eb0(e, this, hash_size);
    e.mem.set_u32(this.addr(), NI_T_MAP_QUEUED_ATTACH_VTABLE);
    this
}

// Translated from 00559e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<NiAVObject *, TESObjectCELL::QUEUED_ATTACH>::scalar deleting
/// destructor` (Xbox PDB): runs the destructor `fn_00559f90`, then frees the
/// map when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00559e80(e: &mut Engine, this: Ptr<NiTPointerMap>, flags: u32) -> Ptr<NiTPointerMap> {
    fn_00559f90(e, this);
    free_if_requested(e, this.cast(), flags);
    this
}

// Translated from 00559eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase` constructor of the `NiAVObject *` to
/// `TESObjectCELL::QUEUED_ATTACH` map: like `fn_00558eb0` with that map's
/// base vtable. Returns `this`.
pub fn fn_00559eb0(e: &mut Engine, this: Ptr<NiTPointerMap>, hash_size: u32) -> Ptr<NiTPointerMap> {
    construct_map_base(e, this, NI_T_MAP_BASE_QUEUED_ATTACH_VTABLE, hash_size);
    this
}

// Translated from 00559f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<..., NiAVObject *, TESObjectCELL::QUEUED_ATTACH>::SetValue`
/// (Xbox PDB): stores `key` in the item (`+4`) and assigns the value
/// (`fn_00559900`) to the item's value (`+8`). The value arrives by value as
/// three words (child, node, flag) and is destroyed by the function. The
/// exception-unwinding frame is not translated.
pub fn fn_00559f20(
    e: &mut Engine,
    _this: Ptr<NiTPointerMap>,
    item: Ptr,
    key: u32,
    value_child: u32,
    value_node: u32,
    value_flag: u32,
) {
    e.mem.set_u32(item.addr() + 4, key);
    e.with_stack(12, |e, value| {
        e.mem.set_u32(value.addr(), value_child);
        e.mem.set_u32(value.addr() + 4, value_node);
        e.mem.set_u32(value.addr() + 8, value_flag);
        fn_00559900(e, Ptr::new(item.addr() + 8), value.cast());
        e.call(QUEUED_ATTACH_DESTRUCT, &args![value]);
    });
}

// Translated from 00559f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<NiAVObject *, TESObjectCELL::QUEUED_ATTACH>` destructor: sets the
/// vtable, removes every item (`00438af0`) and runs the base destructor
/// `fn_00559ff0`. The exception-unwinding frame is not translated.
pub fn fn_00559f90(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    e.mem.set_u32(this.addr(), NI_T_MAP_QUEUED_ATTACH_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    fn_00559ff0(e, this);
}

// Translated from 00559ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Base destructor of the `NiAVObject *` to `TESObjectCELL::QUEUED_ATTACH`
/// map: sets the base vtable, removes every item (`00438af0`) and frees the
/// bucket array (`00aa10f0`).
pub fn fn_00559ff0(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    e.mem
        .set_u32(this.addr(), NI_T_MAP_BASE_QUEUED_ATTACH_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    e.call(DEALLOCATE_MANAGED, &args![table]);
}

// Translated from 0055a020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<NiAVObject *, TESObjectCELL::QUEUED_ATTACH>::NewItem` (Xbox PDB):
/// allocates an item through the map's item allocator at `+0xC`
/// (`fn_0055a160`).
pub fn fn_0055a020(e: &mut Engine, this: Ptr<NiTPointerMap>) -> Ptr {
    fn_0055a160(e, this.byte_add(0x0c))
}

// Translated from 0055a040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<NiAVObject *, TESObjectCELL::QUEUED_ATTACH>::DeleteItem` (Xbox
/// PDB): assigns an empty value (a default `QUEUED_ATTACH`, `fn_0055a0c0`)
/// to the item's value at `+8`, destroys that temporary and gives the item
/// to the item allocator (`fn_0055a1e0`). The exception-unwinding frame is
/// not translated.
pub fn fn_0055a040(e: &mut Engine, this: Ptr<NiTPointerMap>, item: Ptr) {
    e.with_stack(12, |e, empty| {
        fn_0055a0c0(e, empty.cast(), 0);
        fn_00559900(e, Ptr::new(item.addr() + 8), empty.cast());
        e.call(QUEUED_ATTACH_DESTRUCT, &args![empty]);
    });
    fn_0055a1e0(e, this.byte_add(0x0c), item);
}

// Translated from 0055a0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::QUEUED_ATTACH` default constructor: constructs both
/// smart pointers empty (`00633c90` with 0) and clears the flag byte. The
/// argument word is never read. Returns `this`. The exception-unwinding
/// frame is not translated.
pub fn fn_0055a0c0(e: &mut Engine, this: Ptr<QueuedAttach>, _unused_1: u32) -> Ptr<QueuedAttach> {
    e.call(POINTER_CONSTRUCT, &args![this, 0u32]);
    e.call(POINTER_CONSTRUCT, &args![this.byte_add(4), 0u32]);
    e.set(this, QueuedAttach::bFirstAvail, 0);
    this
}

// Translated from 0055a130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<..., NiAVObject *, TESObjectCELL::QUEUED_ATTACH>::scalar
/// deleting destructor` (Xbox PDB): runs the base destructor `fn_00559ff0`,
/// then frees the map when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0055a130(e: &mut Engine, this: Ptr<NiTPointerMap>, flags: u32) -> Ptr<NiTPointerMap> {
    fn_00559ff0(e, this);
    free_if_requested(e, this.cast(), flags);
    this
}

// Translated from 0055a160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DFALL<NiTMapItem<NiAVObject *, QUEUED_ATTACH> >::Allocate` (Xbox PDB):
/// allocates 0x14 bytes (`00401000`) and constructs the allocator item in
/// them (`fn_0055a220`); null when the allocation fails. The
/// exception-unwinding frame is not translated.
pub fn fn_0055a160(e: &mut Engine, _this: Ptr) -> Ptr {
    let block = e.call(ALLOCATE, &args![0x14u32]).u32();
    if block != 0 {
        fn_0055a220(e, Ptr::new(block))
    } else {
        Ptr::new(0)
    }
}

// Translated from 0055a1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Gives an item back (the `Deallocate` of the `DFALL` allocator; name not
/// confirmed): when `item` is not null, runs its deleting destructor
/// (`0055a240`, flags 1) and returns that result, else returns 0.
pub fn fn_0055a1e0(e: &mut Engine, _this: Ptr, item: Ptr) -> u32 {
    if item.addr() != 0 {
        e.call(ITEM_ALLOCATOR_DELETE, &args![item, 1u32]).u32()
    } else {
        0
    }
}

// Translated from 0055a220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the allocator item: runs the constructor body `0055a270`
/// on `this`. Returns `this`.
pub fn fn_0055a220(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(ITEM_ALLOCATOR_CONSTRUCT, &args![this]);
    this
}

/// This part's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00558e20, fn_00558e20(Ptr, u32) -> Ptr),
        entry!(0x00558e50, fn_00558e50(Ptr, u32) -> Ptr),
        entry!(0x00558e80, fn_00558e80(Ptr)),
        entry!(
            0x00558eb0,
            fn_00558eb0(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(0x00558f20, fn_00558f20(Ptr)),
        entry!(
            0x00558fb0,
            fn_00558fb0(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(0x00559020, fn_00559020(Ptr)),
        entry!(
            0x005590b0,
            fn_005590b0(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(0x005591d0, fn_005591d0(Ptr)),
        entry!(
            0x00559260,
            fn_00559260(Ptr<NiTPointerMap>, u32, u32, u32, u32)
        ),
        entry!(0x005593a0, fn_005593a0(Ptr<NiTPointerMap>, Ptr, Ptr, Ptr)),
        entry!(0x00559460, fn_00559460(Ptr<NiTArray>)),
        entry!(
            0x005594b0,
            fn_005594b0(Ptr<NiTArray>, u16, u16) -> Ptr<NiTArray>
        ),
        entry!(
            0x005595e0,
            bs_scrap_map_bs_scrap_map(Ptr<BSScrapMap>, u32, u8) -> Ptr<BSScrapMap>
        ),
        entry!(0x005596a0, fn_005596a0(Ptr<NiTPointerMap>)),
        entry!(0x005596e0, fn_005596e0(Ptr<BSScrapMap>)),
        entry!(0x00559780, bs_scrap_map_new_item(Ptr<BSScrapMap>) -> Ptr),
        entry!(0x005597a0, bs_scrap_map_delete_item(Ptr<BSScrapMap>, Ptr)),
        entry!(0x005597d0, fn_005597d0(Ptr<NiTPointerList>, Ptr)),
        entry!(0x00559810, fn_00559810(Ptr<NiTPointerList>)),
        entry!(0x00559870, fn_00559870(Ptr, u32) -> Ptr),
        entry!(0x005598a0, fn_005598a0(Ptr, u32) -> Ptr),
        entry!(0x005598d0, fn_005598d0(Ptr, u32) -> Ptr),
        entry!(
            0x00559900,
            fn_00559900(Ptr<QueuedAttach>, Ptr<QueuedAttach>) -> Ptr<QueuedAttach>
        ),
        entry!(0x00559940, fn_00559940(Ptr<NiTArray>, u32) -> Ptr<NiTArray>),
        entry!(0x00559970, fn_00559970(Ptr, u32) -> Ptr),
        entry!(
            0x005599a0,
            fn_005599a0(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x005599d0,
            fn_005599d0(Ptr<BSScrapMap>, u32) -> Ptr<BSScrapMap>
        ),
        entry!(0x00559a00, fn_00559a00(Ptr, i32, Ptr)),
        entry!(0x00559a40, fn_00559a40(Ptr, Ptr) -> Ptr),
        entry!(
            0x00559a70,
            fn_00559a70(Ptr<NiTPointerList>, Ptr<NiTListItem>)
        ),
        entry!(
            0x00559ad0,
            fn_00559ad0(Ptr<NiTArray>, u16, u16) -> Ptr<NiTArray>
        ),
        entry!(
            0x00559b40,
            fn_00559b40(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(0x00559bc0, fn_00559bc0(Ptr<NiTPointerMap>)),
        entry!(0x00559c30, fn_00559c30(Ptr<NiTPointerList>)),
        entry!(
            0x00559c90,
            fn_00559c90(Ptr<NiTPointerList>, Ptr<NiTListItem>)
        ),
        entry!(
            0x00559cc0,
            fn_00559cc0(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(0x00559cf0, fn_00559cf0(Ptr, u32) -> Ptr),
        entry!(
            0x00559d20,
            fn_00559d20(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(0x00559d90, fn_00559d90(Ptr)),
        entry!(0x00559df0, fn_00559df0(Ptr<NiTPointerMap>)),
        entry!(
            0x00559e20,
            fn_00559e20(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00559e50,
            fn_00559e50(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00559e80,
            fn_00559e80(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00559eb0,
            fn_00559eb0(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00559f20,
            fn_00559f20(Ptr<NiTPointerMap>, Ptr, u32, u32, u32, u32)
        ),
        entry!(0x00559f90, fn_00559f90(Ptr<NiTPointerMap>)),
        entry!(0x00559ff0, fn_00559ff0(Ptr<NiTPointerMap>)),
        entry!(0x0055a020, fn_0055a020(Ptr<NiTPointerMap>) -> Ptr),
        entry!(0x0055a040, fn_0055a040(Ptr<NiTPointerMap>, Ptr)),
        entry!(
            0x0055a0c0,
            fn_0055a0c0(Ptr<QueuedAttach>, u32) -> Ptr<QueuedAttach>
        ),
        entry!(
            0x0055a130,
            fn_0055a130(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(0x0055a160, fn_0055a160(Ptr) -> Ptr),
        entry!(0x0055a1e0, fn_0055a1e0(Ptr, Ptr) -> u32),
        entry!(0x0055a220, fn_0055a220(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The vtable of the test maps; slot `n` leads to the double `fake(n)`.
    const TEST_VTABLE: u32 = 0x0130_0000;

    fn fake(slot: u32) -> u32 {
        0x7000_0000 + slot
    }

    /// Registers do-nothing doubles returning 0.
    fn quiet(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    /// The addresses called, in order, without the call under test.
    fn sequence(log: &[(u32, Vec<u32>)]) -> Vec<u32> {
        log.iter().skip(1).map(|(a, _)| *a).collect()
    }

    /// The calls made, in order, without the call under test.
    fn calls(log: &[(u32, Vec<u32>)]) -> Vec<(u32, Vec<u32>)> {
        log.iter().skip(1).cloned().collect()
    }

    fn calls_to(log: &[(u32, Vec<u32>)], address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// Checks a scalar deleting destructor: it makes `inner` calls, then
    /// frees the object only when bit 0 of the flags is set, and returns the
    /// object.
    fn scalar_deleting(address: u32, inner: &[u32]) {
        for (flags, freed) in [(0u32, false), (1, true), (2, false), (3, true)] {
            let mut e = Engine::new();
            quiet(&mut e, inner);
            quiet(&mut e, &[DEALLOCATE]);
            let object = e.mem.alloc(0x40);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(address, &args![object, flags]).u32(), object);
            let log = e.call_log.take().unwrap();
            let mut want = inner.to_vec();
            if freed {
                want.push(DEALLOCATE);
                assert_eq!(log.last().unwrap().1, vec![object]);
            }
            assert_eq!(sequence(&log), want);
        }
    }

    /// Checks a `NiTMapBase` constructor: vtable, bucket count, count 0,
    /// a bucket array allocated and zeroed through the managed allocator.
    fn map_base_constructor(address: u32, vtable: u32) {
        let mut e = Engine::new();
        e.register(ALLOCATE_MANAGED, |e, a| e.mem.alloc(a[0]).into_ret());
        quiet(&mut e, &[FILL_BYTES]);
        let map = e.mem.alloc(0x10);
        e.mem.set_u32(map + 0x0c, 99);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(address, &args![map, 8u32]).u32(), map);
        let log = e.call_log.take().unwrap();
        let table = e.mem.u32(map + 8);
        assert_ne!(table, 0);
        assert_eq!(e.mem.u32(map), vtable);
        assert_eq!(e.mem.u32(map + 4), 8);
        assert_eq!(e.mem.u32(map + 0x0c), 0);
        assert_eq!(
            calls(&log),
            vec![
                (ALLOCATE_MANAGED, vec![32]),
                (FILL_BYTES, vec![table, 0, 32])
            ]
        );
    }

    /// Checks a `NiTMap` destructor: vtable, remove-all, base destructor.
    fn map_destructor(address: u32, vtable: u32, base: u32) {
        let mut e = Engine::new();
        quiet(&mut e, &[MAP_REMOVE_ALL, base]);
        let map = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(address, &args![map]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(map), vtable);
        assert_eq!(
            calls(&log),
            vec![(MAP_REMOVE_ALL, vec![map]), (base, vec![map])]
        );
    }

    /// A map of 4 buckets whose vtable leads to doubles: the hash is the key
    /// modulo 4, keys are equal when the words are, `SetValue` stores the
    /// key and the three value words, `NewItem` allocates 0x14 bytes.
    fn queued_map_engine() -> (Engine, u32) {
        let mut e = Engine::new();
        let slots: Vec<u32> = (0..=6).map(|i| fake(4 * i)).collect();
        e.put_vtable(TEST_VTABLE, &slots);
        e.register(fake(MAP_SLOT_KEY_TO_HASH_INDEX), |_, a| {
            (a[1] % 4).into_ret()
        });
        e.register(fake(MAP_SLOT_IS_KEYS_EQUAL), |_, a| {
            u32::from(a[1] == a[2]).into_ret()
        });
        e.register(fake(MAP_SLOT_SET_VALUE), |e, a| {
            e.mem.set_u32(a[1] + 4, a[2]);
            for i in 0..3 {
                e.mem.set_u32(a[1] + 8 + 4 * i, a[3 + i as usize]);
            }
            Ret::default()
        });
        e.register(fake(MAP_SLOT_NEW_ITEM), |e, _| e.mem.alloc(0x14).into_ret());
        e.register(QUEUED_ATTACH_COPY, |e, a| {
            for i in 0..3 {
                let word = e.mem.u32(a[1] + 4 * i);
                e.mem.set_u32(a[0] + 4 * i, word);
            }
            a[0].into_ret()
        });
        quiet(&mut e, &[QUEUED_ATTACH_DESTRUCT]);
        e.register(NI_POINTER_ASSIGN, |e, a| {
            let word = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], word);
            Ret::default()
        });
        let map = e.mem.alloc(0x10);
        let table = e.mem.alloc(16);
        e.mem.set_u32(map, TEST_VTABLE);
        e.mem.set_u32(map + 4, 4);
        e.mem.set_u32(map + 8, table);
        (e, map)
    }

    #[test]
    fn scalar_deleting_destructor_of_the_form_reference_map() {
        scalar_deleting(
            0x0055_8e20,
            &[MAP_REMOVE_ALL, MAP_BASE_DESTRUCT_FORM_REFERENCE],
        );
    }

    #[test]
    fn scalar_deleting_destructor_of_the_multi_bound_map() {
        scalar_deleting(
            0x0055_8e50,
            &[MAP_REMOVE_ALL, MAP_BASE_DESTRUCT_MULTI_BOUND],
        );
    }

    #[test]
    fn clear_releases_the_object_and_zeroes_the_slot() {
        let mut e = Engine::new();
        quiet(&mut e, &[OBJECT_RELEASE]);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, 0x1234);
        e.call_log = Some(vec![]);
        e.call(0x0055_8e80, &args![slot]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log), vec![(OBJECT_RELEASE, vec![0x1234])]);
        assert_eq!(e.mem.u32(slot), 0);
        // An empty slot is not released.
        e.call_log = Some(vec![]);
        e.call(0x0055_8e80, &args![slot]);
        assert!(calls(&e.call_log.take().unwrap()).is_empty());
    }

    #[test]
    fn reference_node_map_base_constructor() {
        map_base_constructor(0x0055_8eb0, NI_T_MAP_BASE_REFERENCE_NODE_VTABLE);
    }

    #[test]
    fn reference_node_map_destructor() {
        map_destructor(
            0x0055_8f20,
            NI_T_MAP_REFERENCE_NODE_VTABLE,
            MAP_BASE_DESTRUCT_REFERENCE_NODE,
        );
    }

    #[test]
    fn form_reference_map_base_constructor() {
        map_base_constructor(0x0055_8fb0, NI_T_MAP_BASE_FORM_REFERENCE_VTABLE);
    }

    #[test]
    fn form_reference_map_destructor() {
        map_destructor(
            0x0055_9020,
            NI_T_MAP_FORM_REFERENCE_VTABLE,
            MAP_BASE_DESTRUCT_FORM_REFERENCE,
        );
    }

    #[test]
    fn multi_bound_map_base_constructor() {
        map_base_constructor(0x0055_90b0, NI_T_MAP_BASE_MULTI_BOUND_VTABLE);
    }

    #[test]
    fn multi_bound_map_destructor() {
        map_destructor(
            0x0055_91d0,
            NI_T_MAP_MULTI_BOUND_VTABLE,
            MAP_BASE_DESTRUCT_MULTI_BOUND,
        );
    }

    #[test]
    fn set_at_adds_items_at_the_head_of_their_bucket() {
        let (mut e, map) = queued_map_engine();
        e.call_log = Some(vec![]);
        e.call(0x0055_9260, &args![map, 5u32, 0x11u32, 0x22u32, 1u32]);
        let log = e.call_log.take().unwrap();
        let table = e.mem.u32(map + 8);
        let first = e.mem.u32(table + 4);
        assert_ne!(first, 0);
        assert_eq!(e.mem.u32(first), 0);
        assert_eq!(e.mem.u32(first + 4), 5);
        assert_eq!(
            [
                e.mem.u32(first + 8),
                e.mem.u32(first + 12),
                e.mem.u32(first + 16)
            ],
            [0x11, 0x22, 1]
        );
        assert_eq!(e.mem.u32(map + 0x0c), 1);
        // The value is copied once and the parameter destroyed once.
        assert_eq!(calls_to(&log, QUEUED_ATTACH_COPY).len(), 1);
        assert_eq!(calls_to(&log, QUEUED_ATTACH_DESTRUCT).len(), 1);
        assert!(calls_to(&log, NI_POINTER_ASSIGN).is_empty());
        // Key 9 hashes to the same bucket and goes in front.
        e.call(0x0055_9260, &args![map, 9u32, 0x33u32, 0x44u32, 0u32]);
        let second = e.mem.u32(table + 4);
        assert_ne!(second, first);
        assert_eq!(e.mem.u32(second), first);
        assert_eq!(e.mem.u32(second + 4), 9);
        assert_eq!(e.mem.u32(map + 0x0c), 2);
    }

    #[test]
    fn set_at_assigns_the_value_of_an_existing_key() {
        let (mut e, map) = queued_map_engine();
        e.call(0x0055_9260, &args![map, 5u32, 0x11u32, 0x22u32, 1u32]);
        let table = e.mem.u32(map + 8);
        let item = e.mem.u32(table + 4);
        e.call_log = Some(vec![]);
        e.call(0x0055_9260, &args![map, 5u32, 0x55u32, 0x66u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(map + 0x0c), 1);
        assert_eq!(e.mem.u32(table + 4), item);
        assert_eq!(e.mem.u32(item + 8), 0x55);
        assert_eq!(e.mem.u32(item + 12), 0x66);
        assert_eq!(e.mem.u8(item + 16), 0);
        assert_eq!(calls_to(&log, NI_POINTER_ASSIGN).len(), 2);
        assert!(calls_to(&log, QUEUED_ATTACH_COPY).is_empty());
        assert_eq!(calls_to(&log, QUEUED_ATTACH_DESTRUCT).len(), 1);
        assert!(calls_to(&log, fake(MAP_SLOT_NEW_ITEM)).is_empty());
    }

    #[test]
    fn get_next_walks_the_buckets() {
        let (mut e, map) = queued_map_engine();
        let table = e.mem.u32(map + 8);
        let (a, b, c) = (e.mem.alloc(0x14), e.mem.alloc(0x14), e.mem.alloc(0x14));
        for (item, next, key, child, flag) in
            [(a, b, 5, 0xa1, 1u8), (b, 0, 9, 0xb1, 0), (c, 0, 7, 0xc1, 1)]
        {
            e.mem.set_u32(item, next);
            e.mem.set_u32(item + 4, key);
            e.mem.set_u32(item + 8, child);
            e.mem.set_u8(item + 16, flag);
        }
        e.mem.set_u32(table + 4, a);
        e.mem.set_u32(table + 12, c);
        let position = e.mem.alloc(4);
        let key_out = e.mem.alloc(4);
        let value = e.mem.alloc(0x0c);
        e.mem.set_u32(position, a);
        e.call(0x0055_93a0, &args![map, position, key_out, value]);
        assert_eq!(e.mem.u32(key_out), 5);
        assert_eq!(e.mem.u32(value), 0xa1);
        assert_eq!(e.mem.u8(value + 8), 1);
        assert_eq!(e.mem.u32(position), b);
        // The end of bucket 1 leads to bucket 3.
        e.call(0x0055_93a0, &args![map, position, key_out, value]);
        assert_eq!(e.mem.u32(key_out), 9);
        assert_eq!(e.mem.u32(value), 0xb1);
        assert_eq!(e.mem.u8(value + 8), 0);
        assert_eq!(e.mem.u32(position), c);
        // After the last bucket the position is 0.
        e.call(0x0055_93a0, &args![map, position, key_out, value]);
        assert_eq!(e.mem.u32(key_out), 7);
        assert_eq!(e.mem.u32(position), 0);
    }

    #[test]
    fn array_destructor_frees_the_buffer() {
        let mut e = Engine::new();
        quiet(&mut e, &[ARRAY_BUFFER_FREE]);
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array + 4, 0x5000);
        e.call_log = Some(vec![]);
        e.call(0x0055_9460, &args![array]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(array), NI_T_ARRAY_VTABLE);
        assert_eq!(calls(&log), vec![(ARRAY_BUFFER_FREE, vec![0x5000])]);
    }

    fn array_engine() -> Engine {
        let mut e = Engine::new();
        e.register(ARRAY_BUFFER_ALLOCATE, |e, a| {
            e.mem.alloc(a[0] * 4).into_ret()
        });
        e
    }

    #[test]
    fn array_constructor_allocates_a_buffer_for_a_capacity() {
        let mut e = array_engine();
        let array = e.mem.alloc(0x10);
        e.mem.set_u16(array + 0x0a, 7);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0055_9ad0, &args![array, 4u16, 2u16]).u32(), array);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(array), NI_T_ARRAY_VTABLE);
        assert_eq!(e.mem.u16(array + 8), 4);
        assert_eq!(e.mem.u16(array + 0x0a), 0);
        assert_eq!(e.mem.u16(array + 0x0c), 0);
        assert_eq!(e.mem.u16(array + 0x0e), 2);
        assert_ne!(e.mem.u32(array + 4), 0);
        assert_eq!(calls(&log), vec![(ARRAY_BUFFER_ALLOCATE, vec![4])]);
    }

    #[test]
    fn array_constructor_without_capacity_has_no_buffer() {
        let mut e = array_engine();
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array + 4, 0x77);
        e.call_log = Some(vec![]);
        e.call(0x0055_9ad0, &args![array, 0u16, 8u16]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(array + 4), 0);
        assert_eq!(e.mem.u16(array + 0x0e), 8);
        assert!(calls(&log).is_empty());
    }

    #[test]
    fn primitive_array_constructor_sets_its_own_vtable() {
        let mut e = array_engine();
        let array = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0055_94b0, &args![array, 3u16, 1u16]).u32(), array);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(array), NI_T_PRIMITIVE_ARRAY_VTABLE);
        assert_eq!(e.mem.u16(array + 8), 3);
        assert_eq!(e.mem.u16(array + 0x0e), 1);
        assert_eq!(calls(&log), vec![(ARRAY_BUFFER_ALLOCATE, vec![3])]);
    }

    #[test]
    fn scrap_map_constructor_allocates_its_buckets_from_the_heap() {
        let mut e = Engine::new();
        e.map(0x010a_2000, 0x1000);
        e.set_global(SCRAP_ALIGNMENT, 4u32);
        quiet(
            &mut e,
            &[SCRAP_HEAP_CONSTRUCT, FILL_BYTES, ALLOCATE, CRT_MEMSET],
        );
        e.register(SCRAP_HEAP_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        let map = e.mem.alloc(0x28);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0055_95e0, &args![map, 8u32, 1u8]).u32(), map);
        let log = e.call_log.take().unwrap();
        let table = e.mem.u32(map + 8);
        assert_ne!(table, 0);
        assert_eq!(e.mem.u32(map), BS_SCRAP_MAP_VTABLE);
        assert_eq!(e.mem.u32(map + 4), 8);
        assert_eq!(e.mem.u32(map + 0x0c), 0);
        assert_eq!(e.mem.u8(map + 0x10), 1);
        // The base part allocates nothing (bucket count 0).
        assert_eq!(
            calls(&log),
            vec![
                (SCRAP_HEAP_CONSTRUCT, vec![map + 0x14]),
                (SCRAP_HEAP_ALLOCATE, vec![map + 0x14, 32, 4]),
                (FILL_BYTES, vec![table, 0, 32]),
            ]
        );
    }

    #[test]
    fn map_base_destructor_frees_the_buckets_only_when_there_are_some() {
        let mut e = Engine::new();
        quiet(&mut e, &[DEALLOCATE]);
        let map = e.mem.alloc(0x10);
        e.mem.set_u32(map + 8, 0x4000);
        e.call_log = Some(vec![]);
        e.call(0x0055_96a0, &args![map]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(map), BS_MAP_BASE_VTABLE);
        assert_eq!(calls(&log), vec![(DEALLOCATE, vec![0x4000])]);
        e.mem.set_u32(map + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x0055_96a0, &args![map]);
        assert!(calls(&e.call_log.take().unwrap()).is_empty());
    }

    /// A scrap map with two items in bucket 1 and a vtable whose
    /// `ClearValue` double is `fake(0x10)`.
    fn scrap_map_with_items(clear: u8) -> (Engine, u32, u32, u32) {
        let mut e = Engine::new();
        let slots: Vec<u32> = (0..=6).map(|i| fake(4 * i)).collect();
        e.put_vtable(BS_SCRAP_MAP_VTABLE, &slots);
        quiet(
            &mut e,
            &[fake(MAP_SLOT_CLEAR_VALUE), SCRAP_HEAP_DESTRUCT, DEALLOCATE],
        );
        let map = e.mem.alloc(0x28);
        let table = e.mem.alloc(16);
        let (first, second) = (e.mem.alloc(0x0c), e.mem.alloc(0x0c));
        e.mem.set_u32(first, second);
        e.mem.set_u32(table + 4, first);
        e.mem.set_u32(map + 4, 4);
        e.mem.set_u32(map + 8, table);
        e.mem.set_u32(map + 0x0c, 2);
        e.mem.set_u8(map + 0x10, clear);
        (e, map, first, second)
    }

    #[test]
    fn scrap_map_destructor_clears_the_items_when_asked() {
        let (mut e, map, first, second) = scrap_map_with_items(1);
        e.call_log = Some(vec![]);
        e.call(0x0055_96e0, &args![map]);
        let log = e.call_log.take().unwrap();
        // The buckets belong to the heap: nothing is freed.
        assert_eq!(
            calls(&log),
            vec![
                (fake(MAP_SLOT_CLEAR_VALUE), vec![map, first]),
                (fake(MAP_SLOT_CLEAR_VALUE), vec![map, second]),
                (SCRAP_HEAP_DESTRUCT, vec![map + 0x14]),
            ]
        );
        assert_eq!(e.mem.u32(map), BS_MAP_BASE_VTABLE);
        assert_eq!(e.mem.u32(map + 4), 0);
        assert_eq!(e.mem.u32(map + 8), 0);
        assert_eq!(e.mem.u32(map + 0x0c), 0);
    }

    #[test]
    fn scrap_map_destructor_keeps_the_items_without_the_flag() {
        let (mut e, map, _, _) = scrap_map_with_items(0);
        e.call_log = Some(vec![]);
        e.call(0x0055_96e0, &args![map]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log), vec![(SCRAP_HEAP_DESTRUCT, vec![map + 0x14])]);
    }

    #[test]
    fn scrap_map_new_item_and_delete_item_use_the_heap() {
        let mut e = Engine::new();
        e.map(0x010a_2000, 0x1000);
        e.set_global(SCRAP_ALIGNMENT, 4u32);
        e.register(SCRAP_HEAP_ALLOCATE, |_, _| 0x9000u32.into_ret());
        quiet(&mut e, &[SCRAP_HEAP_DEALLOCATE]);
        let map = e.mem.alloc(0x28);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0055_9780, &args![map]).u32(), 0x9000);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log),
            vec![(SCRAP_HEAP_ALLOCATE, vec![map + 0x14, 0x0c, 4])]
        );
        let item = e.mem.alloc(0x0c);
        e.mem.set_u8(item + 8, 1);
        e.call_log = Some(vec![]);
        e.call(0x0055_97a0, &args![map, item]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u8(item + 8), 0);
        assert_eq!(
            calls(&log),
            vec![(SCRAP_HEAP_DEALLOCATE, vec![map + 0x14, item])]
        );
    }

    #[test]
    fn list_append_links_nodes_in_order() {
        let mut e = Engine::new();
        let list = e.mem.alloc(0x0c);
        let (first, second) = (e.mem.alloc(0x0c), e.mem.alloc(0x0c));
        e.mem.set_u32(first, 0xdead);
        e.call(0x0055_9a70, &args![list, first]);
        assert_eq!(e.mem.u32(list), first);
        assert_eq!(e.mem.u32(list + 4), first);
        assert_eq!(e.mem.u32(list + 8), 1);
        assert_eq!(e.mem.u32(first), 0);
        assert_eq!(e.mem.u32(first + 4), 0);
        e.call(0x0055_9a70, &args![list, second]);
        assert_eq!(e.mem.u32(list), first);
        assert_eq!(e.mem.u32(list + 4), second);
        assert_eq!(e.mem.u32(list + 8), 2);
        assert_eq!(e.mem.u32(first), second);
        assert_eq!(e.mem.u32(second), 0);
        assert_eq!(e.mem.u32(second + 4), first);
    }

    #[test]
    fn list_add_tail_stores_the_pointer_in_a_new_node() {
        let mut e = Engine::new();
        let node = e.mem.alloc(0x0c);
        e.register_double(LIST_ALLOCATE_NODE, move |_, _| node.into_ret());
        let list = e.mem.alloc(0x0c);
        let element = e.mem.alloc(4);
        e.mem.set_u32(element, 0xabcd);
        e.call_log = Some(vec![]);
        e.call(0x0055_97d0, &args![list, element]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log), vec![(LIST_ALLOCATE_NODE, vec![list])]);
        assert_eq!(e.mem.u32(node + 8), 0xabcd);
        assert_eq!(e.mem.u32(list), node);
        assert_eq!(e.mem.u32(list + 4), node);
        assert_eq!(e.mem.u32(list + 8), 1);
    }

    #[test]
    fn list_remove_all_frees_every_node_and_resets_the_list() {
        let mut e = Engine::new();
        quiet(&mut e, &[LIST_FREE_NODE]);
        let list = e.mem.alloc(0x0c);
        let (first, second) = (e.mem.alloc(0x0c), e.mem.alloc(0x0c));
        e.mem.set_u32(first, second);
        e.mem.set_u32(first + 8, 1);
        e.mem.set_u32(second + 8, 2);
        e.mem.set_u32(list, first);
        e.mem.set_u32(list + 4, second);
        e.mem.set_u32(list + 8, 2);
        e.call_log = Some(vec![]);
        e.call(0x0055_9c30, &args![list]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log),
            vec![
                (LIST_FREE_NODE, vec![list + 8, first]),
                (LIST_FREE_NODE, vec![list + 8, second]),
            ]
        );
        assert_eq!(e.mem.u32(first + 8), 0);
        assert_eq!(e.mem.u32(second + 8), 0);
        assert_eq!(
            [e.mem.u32(list), e.mem.u32(list + 4), e.mem.u32(list + 8)],
            [0, 0, 0]
        );
    }

    #[test]
    fn list_node_release_clears_the_element() {
        let mut e = Engine::new();
        quiet(&mut e, &[LIST_FREE_NODE]);
        let list = e.mem.alloc(0x0c);
        let node = e.mem.alloc(0x0c);
        e.mem.set_u32(node + 8, 5);
        e.call_log = Some(vec![]);
        e.call(0x0055_9c90, &args![list, node]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(node + 8), 0);
        assert_eq!(calls(&log), vec![(LIST_FREE_NODE, vec![list + 8, node])]);
    }

    #[test]
    fn list_owner_destructor_clears_the_list_then_runs_the_base() {
        let mut e = Engine::new();
        quiet(&mut e, &[LIST_FREE_NODE, LIST_OWNER_BASE_DESTRUCT]);
        let list = e.mem.alloc(0x0c);
        let node = e.mem.alloc(0x0c);
        e.mem.set_u32(list, node);
        e.mem.set_u32(list + 4, node);
        e.mem.set_u32(list + 8, 1);
        e.call_log = Some(vec![]);
        e.call(0x0055_9810, &args![list]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log),
            vec![
                (LIST_FREE_NODE, vec![list + 8, node]),
                (LIST_OWNER_BASE_DESTRUCT, vec![list]),
            ]
        );
        assert_eq!(e.mem.u32(list + 8), 0);
    }

    #[test]
    fn map_base_scalar_deleting_destructors() {
        scalar_deleting(0x0055_9870, &[MAP_BASE_DESTRUCT_REFERENCE_NODE]);
        scalar_deleting(0x0055_98a0, &[MAP_BASE_DESTRUCT_FORM_REFERENCE]);
        scalar_deleting(0x0055_98d0, &[MAP_BASE_DESTRUCT_MULTI_BOUND]);
    }

    #[test]
    fn value_assignment_assigns_both_pointers_and_copies_the_flag() {
        let mut e = Engine::new();
        quiet(&mut e, &[NI_POINTER_ASSIGN]);
        let (this, other) = (e.mem.alloc(0x0c), e.mem.alloc(0x0c));
        e.mem.set_u8(other + 8, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0055_9900, &args![this, other]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log),
            vec![
                (NI_POINTER_ASSIGN, vec![this, other]),
                (NI_POINTER_ASSIGN, vec![this + 4, other + 4]),
            ]
        );
        assert_eq!(e.mem.u8(this + 8), 1);
    }

    #[test]
    fn array_scalar_deleting_destructor() {
        scalar_deleting(0x0055_9940, &[ARRAY_BUFFER_FREE]);
    }

    #[test]
    fn helper_scalar_deleting_destructor() {
        scalar_deleting(0x0055_9970, &[CELL_HELPER_DESTRUCT]);
    }

    #[test]
    fn map_base_scalar_deleting_destructor_of_the_bool_map() {
        // No buckets: the destructor itself makes no call.
        scalar_deleting(0x0055_99a0, &[]);
    }

    #[test]
    fn scrap_map_scalar_deleting_destructor() {
        // The flag byte is clear: only the heap is destroyed.
        scalar_deleting(0x0055_99d0, &[SCRAP_HEAP_DESTRUCT]);
    }

    #[test]
    fn fill_writes_the_word_to_every_element() {
        let mut e = Engine::new();
        let buffer = e.mem.alloc(16);
        let value = e.mem.alloc(4);
        e.mem.set_u32(value, 0xfeed);
        e.mem.set_u32(buffer + 12, 1);
        e.call(0x0055_9a00, &args![buffer, 3i32, value]);
        assert_eq!(
            [
                e.mem.u32(buffer),
                e.mem.u32(buffer + 4),
                e.mem.u32(buffer + 8),
                e.mem.u32(buffer + 12)
            ],
            [0xfeed, 0xfeed, 0xfeed, 1]
        );
        // A count of zero or less writes nothing.
        e.call(0x0055_9a00, &args![buffer, 0i32, value]);
        e.call(0x0055_9a00, &args![buffer + 12, -2i32, value]);
        assert_eq!(e.mem.u32(buffer + 12), 1);
    }

    #[test]
    fn smart_pointer_copy_adds_a_reference() {
        let mut e = Engine::new();
        quiet(&mut e, &[REFERENCE_ADD]);
        let (this, other) = (e.mem.alloc(4), e.mem.alloc(4));
        e.mem.set_u32(other, 0x6000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0055_9a40, &args![this, other]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), 0x6000);
        assert_eq!(calls(&log), vec![(REFERENCE_ADD, vec![0x6000])]);
        // A null pointer is copied without a reference.
        e.mem.set_u32(other, 0);
        e.call_log = Some(vec![]);
        e.call(0x0055_9a40, &args![this, other]);
        assert_eq!(e.mem.u32(this), 0);
        assert!(calls(&e.call_log.take().unwrap()).is_empty());
    }

    #[test]
    fn bs_map_base_constructor_allocates_buckets_only_for_a_size() {
        let mut e = Engine::new();
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        quiet(&mut e, &[CRT_MEMSET]);
        let map = e.mem.alloc(0x10);
        e.mem.set_u32(map + 0x0c, 9);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0055_9b40, &args![map, 5u32]).u32(), map);
        let log = e.call_log.take().unwrap();
        let table = e.mem.u32(map + 8);
        assert_ne!(table, 0);
        assert_eq!(e.mem.u32(map), BS_MAP_BASE_VTABLE);
        assert_eq!(e.mem.u32(map + 4), 5);
        assert_eq!(e.mem.u32(map + 0x0c), 0);
        assert_eq!(
            calls(&log),
            vec![(ALLOCATE, vec![20]), (CRT_MEMSET, vec![table, 0, 20])]
        );
        // Size 0: no allocation and the bucket pointer is left alone.
        let other = e.mem.alloc(0x10);
        e.mem.set_u32(other + 8, 0x77);
        e.call_log = Some(vec![]);
        e.call(0x0055_9b40, &args![other, 0u32]);
        assert!(calls(&e.call_log.take().unwrap()).is_empty());
        assert_eq!(e.mem.u32(other + 8), 0x77);
        assert_eq!(e.mem.u32(other + 4), 0);
    }

    #[test]
    fn remove_all_clears_every_item_and_the_count() {
        let (mut e, map, first, second) = scrap_map_with_items(0);
        // The buckets: bucket 1 holds two items, bucket 3 one.
        let table = e.mem.u32(map + 8);
        e.mem.set_u32(map, BS_SCRAP_MAP_VTABLE);
        let third = e.mem.alloc(0x0c);
        e.mem.set_u32(table + 12, third);
        e.call_log = Some(vec![]);
        e.call(0x0055_9bc0, &args![map]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log),
            vec![
                (fake(MAP_SLOT_CLEAR_VALUE), vec![map, first]),
                (fake(MAP_SLOT_CLEAR_VALUE), vec![map, second]),
                (fake(MAP_SLOT_CLEAR_VALUE), vec![map, third]),
            ]
        );
        assert_eq!(e.mem.u32(map + 0x0c), 0);
    }

    #[test]
    fn reference_reference_map_constructors_and_destructors() {
        map_base_constructor(0x0055_9d20, NI_T_MAP_BASE_REFERENCE_REFERENCE_VTABLE);
        map_destructor(
            0x0055_9d90,
            NI_T_MAP_REFERENCE_REFERENCE_VTABLE,
            MAP_BASE_DESTRUCT_REFERENCE_REFERENCE,
        );
        scalar_deleting(
            0x0055_9cf0,
            &[MAP_REMOVE_ALL, MAP_BASE_DESTRUCT_REFERENCE_REFERENCE],
        );
    }

    #[test]
    fn reference_reference_map_constructor_sets_the_derived_vtable() {
        let mut e = Engine::new();
        e.register(ALLOCATE_MANAGED, |e, a| e.mem.alloc(a[0]).into_ret());
        quiet(&mut e, &[FILL_BYTES]);
        let map = e.mem.alloc(0x10);
        assert_eq!(e.call(0x0055_9cc0, &args![map, 16u32]).u32(), map);
        assert_eq!(e.mem.u32(map), NI_T_MAP_REFERENCE_REFERENCE_VTABLE);
        assert_eq!(e.mem.u32(map + 4), 16);
        assert_ne!(e.mem.u32(map + 8), 0);
    }

    #[test]
    fn queued_attach_map_scalar_deleting_destructors() {
        scalar_deleting(0x0055_9e20, &[MAP_REMOVE_ALL, DEALLOCATE_MANAGED]);
        scalar_deleting(
            0x0055_9e80,
            &[MAP_REMOVE_ALL, MAP_REMOVE_ALL, DEALLOCATE_MANAGED],
        );
        scalar_deleting(0x0055_a130, &[MAP_REMOVE_ALL, DEALLOCATE_MANAGED]);
    }

    #[test]
    fn queued_attach_map_constructors() {
        map_base_constructor(0x0055_9eb0, NI_T_MAP_BASE_QUEUED_ATTACH_VTABLE);
        let mut e = Engine::new();
        e.register(ALLOCATE_MANAGED, |e, a| e.mem.alloc(a[0]).into_ret());
        quiet(&mut e, &[FILL_BYTES]);
        let map = e.mem.alloc(0x20);
        assert_eq!(e.call(0x0055_9e50, &args![map, 8u32]).u32(), map);
        assert_eq!(e.mem.u32(map), NI_T_MAP_QUEUED_ATTACH_VTABLE);
        assert_eq!(e.mem.u32(map + 4), 8);
        assert_ne!(e.mem.u32(map + 8), 0);
    }

    #[test]
    fn base_destructors_free_the_bucket_array() {
        for (address, vtable) in [
            (0x0055_9df0u32, NI_T_MAP_BASE_REFERENCE_REFERENCE_VTABLE),
            (0x0055_9ff0, NI_T_MAP_BASE_QUEUED_ATTACH_VTABLE),
        ] {
            let mut e = Engine::new();
            quiet(&mut e, &[MAP_REMOVE_ALL, DEALLOCATE_MANAGED]);
            let map = e.mem.alloc(0x10);
            e.mem.set_u32(map + 8, 0x4444);
            e.call_log = Some(vec![]);
            e.call(address, &args![map]);
            let log = e.call_log.take().unwrap();
            assert_eq!(e.mem.u32(map), vtable);
            assert_eq!(
                calls(&log),
                vec![
                    (MAP_REMOVE_ALL, vec![map]),
                    (DEALLOCATE_MANAGED, vec![0x4444])
                ]
            );
        }
    }

    #[test]
    fn queued_attach_map_destructor_runs_the_base_destructor() {
        let mut e = Engine::new();
        quiet(&mut e, &[MAP_REMOVE_ALL, DEALLOCATE_MANAGED]);
        let map = e.mem.alloc(0x10);
        e.mem.set_u32(map + 8, 0x4444);
        e.call_log = Some(vec![]);
        e.call(0x0055_9f90, &args![map]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(map), NI_T_MAP_BASE_QUEUED_ATTACH_VTABLE);
        assert_eq!(
            sequence(&log),
            vec![MAP_REMOVE_ALL, MAP_REMOVE_ALL, DEALLOCATE_MANAGED]
        );
    }

    #[test]
    fn set_value_stores_the_key_and_assigns_the_value() {
        let mut e = Engine::new();
        quiet(&mut e, &[QUEUED_ATTACH_DESTRUCT]);
        e.register(NI_POINTER_ASSIGN, |e, a| {
            let word = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], word);
            Ret::default()
        });
        let map = e.mem.alloc(0x10);
        let item = e.mem.alloc(0x14);
        e.call_log = Some(vec![]);
        e.call(
            0x0055_9f20,
            &args![map, item, 0x77u32, 0x11u32, 0x22u32, 1u32],
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(item + 4), 0x77);
        assert_eq!(e.mem.u32(item + 8), 0x11);
        assert_eq!(e.mem.u32(item + 12), 0x22);
        assert_eq!(e.mem.u8(item + 16), 1);
        assert_eq!(calls_to(&log, QUEUED_ATTACH_DESTRUCT).len(), 1);
    }

    #[test]
    fn allocator_functions() {
        let mut e = Engine::new();
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(ITEM_ALLOCATOR_CONSTRUCT, |_, _| Ret::default());
        e.register(ITEM_ALLOCATOR_DELETE, |_, a| (a[1] + 100).into_ret());
        let map = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        let block = e.call(0x0055_a020, &args![map]).u32();
        let log = e.call_log.take().unwrap();
        assert_ne!(block, 0);
        assert_eq!(
            calls(&log),
            vec![
                (ALLOCATE, vec![0x14]),
                (ITEM_ALLOCATOR_CONSTRUCT, vec![block])
            ]
        );
        // Giving back an item runs its deleting destructor; null does not.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0055_a1e0, &args![map, block]).u32(), 101);
        assert_eq!(
            calls(&e.call_log.take().unwrap()),
            vec![(ITEM_ALLOCATOR_DELETE, vec![block, 1])]
        );
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0055_a1e0, &args![map, 0u32]).u32(), 0);
        assert!(calls(&e.call_log.take().unwrap()).is_empty());
        // The item constructor returns its argument.
        assert_eq!(e.call(0x0055_a220, &args![block]).u32(), block);
        // A failed allocation gives null and no construction.
        e.register(ALLOCATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0055_a160, &args![map]).u32(), 0);
        assert_eq!(calls(&e.call_log.take().unwrap()).len(), 1);
    }

    #[test]
    fn default_queued_attach_is_empty() {
        let mut e = Engine::new();
        quiet(&mut e, &[POINTER_CONSTRUCT]);
        let value = e.mem.alloc(0x0c);
        e.mem.set_u8(value + 8, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0055_a0c0, &args![value, 0u32]).u32(), value);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u8(value + 8), 0);
        assert_eq!(
            calls(&log),
            vec![
                (POINTER_CONSTRUCT, vec![value, 0]),
                (POINTER_CONSTRUCT, vec![value + 4, 0])
            ]
        );
    }

    #[test]
    fn delete_item_empties_the_value_and_frees_the_item() {
        let mut e = Engine::new();
        quiet(&mut e, &[POINTER_CONSTRUCT, QUEUED_ATTACH_DESTRUCT]);
        e.register(ITEM_ALLOCATOR_DELETE, |_, _| Ret::default());
        e.register(NI_POINTER_ASSIGN, |e, a| {
            let word = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], word);
            Ret::default()
        });
        let map = e.mem.alloc(0x20);
        let item = e.mem.alloc(0x14);
        e.mem.set_u32(item + 8, 0x1234);
        e.mem.set_u8(item + 16, 1);
        e.call_log = Some(vec![]);
        e.call(0x0055_a040, &args![map, item]);
        let log = e.call_log.take().unwrap();
        // The default value is all zero, so the assignment empties the slot.
        assert_eq!(e.mem.u32(item + 8), 0);
        assert_eq!(e.mem.u8(item + 16), 0);
        assert_eq!(calls_to(&log, QUEUED_ATTACH_DESTRUCT).len(), 1);
        assert_eq!(calls_to(&log, ITEM_ALLOCATOR_DELETE), vec![vec![item, 1]]);
    }
}
