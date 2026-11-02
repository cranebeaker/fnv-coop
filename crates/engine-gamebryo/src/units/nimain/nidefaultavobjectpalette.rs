//! `NiMain/nidefaultavobjectpalette.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a84c70`: the jump target of `00a6e540`.
const JUMP_TARGET_00A6E540: u32 = 0x00a8_4c70;

// Translated from 00a6e540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDefaultAVObjectPalette::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00a84c70` (engine map: `BSAnimNote::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiDefaultAVObjectPalette@@UAA_NAAVNiStream@@@Z`.
pub fn ni_default_av_object_palette_register_streamables(
    e: &mut Engine,
    this: Ptr,
    stream: Ptr,
) -> bool {
    e.call(JUMP_TARGET_00A6E540, &args![this, stream]).bool()
}

// The palette (`NiDefaultAVObjectPalette`, its layout in `nicontrollermanager.rs`) holds an
// `NiTFixedStringMap<NiAVObject *>` at +8 (vtable, bucket count +0xc, buckets +0x10, item count
// +0x14) and the scene at +0x18. Map items are `{next, key (a fixed-string handle), value}`;
// a fixed-string handle points at its characters and is counted at handle - 8, except the
// empty string's handle (the word at `0109b220`).

use crate::types::NiTPointerMap;

/// The word holding the handle of the empty `NiFixedString` (never counted).
const EMPTY_FIXED_STRING: u32 = 0x0109_b220;
/// `InterlockedIncrement` and `InterlockedDecrement` (import slots, one word).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The memory manager singleton and its `Allocate` (`thiscall`, the size).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// `operator new` (`cdecl`, the size), the table allocator and free (`cdecl`, the size /
/// the block) and `_memset` (`cdecl`, (destination, value, size)).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
const TABLE_ALLOCATE: u32 = 0x00aa_1070;
const TABLE_FREE: u32 = 0x00aa_10f0;
const MEMSET: u32 = 0x00ec_61c0;
/// `NiObject`'s constructor (`thiscall`) and `NiObject::ProcessClone` (`thiscall`, the
/// cloning process).
const OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
const OBJECT_PROCESS_CLONE: u32 = 0x00a5_d430;
/// `00aa1000` (`thiscall`, (clone, cloning process)): the base member copy.
const OBJECT_COPY_MEMBERS: u32 = 0x00aa_1000;
/// `00a88a40` (`thiscall`, the stream; engine map `NiInterpolator::LoadBinary`): the base's
/// `LoadBinary`; `00a724d0` (`thiscall`, the stream): the base's `LinkObject`.
const BASE_LOAD_BINARY: u32 = 0x00a8_8a40;
const BASE_LINK_OBJECT: u32 = 0x00a7_24d0;
/// `NiStream::ResolveLinkID` (`thiscall`, no stack words) and `NiStream::LoadCString`
/// (`thiscall`, the pointer to fill with an allocated copy).
const STREAM_RESOLVE_LINK_ID: u32 = 0x00a6_4620;
const STREAM_LOAD_C_STRING: u32 = 0x00a6_4ce0;
/// `00a5b690` (`cdecl`, the characters): a counted fixed-string handle for them.
const FIXED_STRING_FROM_CHARS: u32 = 0x00a5_b690;
/// `00a53c30` (`thiscall` on a clone map, (key, &value)): the lookup; answers whether the key
/// is there (`AL`).
const CLONE_MAP_GET_AT: u32 = 0x00a5_3c30;
/// The fixed-string map's `SetAt` (`thiscall`, (&key, value)), `RemoveAt` (`thiscall`,
/// &key) and `RemoveAll` (`thiscall`).
const MAP_SET_AT: u32 = 0x00a6_e980;
const MAP_REMOVE_AT: u32 = 0x00aa_0250;
const MAP_REMOVE_ALL: u32 = 0x00aa_01f0;
/// `NiDefaultAVObjectPalette`'s vtable and the fixed-string map's vtables
/// (`NiTFixedStringMapBase`, then `NiTFixedStringMap`), and the vtables `LinkObject`'s local
/// map is given (`0109ca08`, then `0109ca18`).
const NI_DEFAULT_AV_OBJECT_PALETTE_VTABLE: u32 = 0x0109_6284;
const FIXED_STRING_MAP_BASE_VTABLE: u32 = 0x0109_6114;
const FIXED_STRING_MAP_VTABLE: u32 = 0x0109_618c;
const LOCAL_MAP_BASE_VTABLE: u32 = 0x0109_ca08;
const LOCAL_MAP_VTABLE: u32 = 0x0109_ca18;

fn empty_handle(e: &Engine) -> u32 {
    e.mem.u32(EMPTY_FIXED_STRING)
}

/// Counts a fixed-string handle unless it is the empty string's.
fn acquire(e: &mut Engine, handle: u32) {
    if handle != empty_handle(e) {
        e.call(INTERLOCKED_INCREMENT, &args![handle.wrapping_sub(8)]);
    }
}

/// Releases a fixed-string handle unless it is the empty string's.
fn release(e: &mut Engine, handle: u32) {
    if handle != empty_handle(e) {
        e.call(INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
    }
}

/// The first item of a map's buckets (the first non-null bucket), or 0.
fn first_item(e: &Engine, size: u32, table: u32) -> u32 {
    for index in 0..size {
        let item = e.mem.u32(table.wrapping_add(index.wrapping_mul(4)));
        if item != 0 {
            return item;
        }
    }
    0
}

// Translated from 00a6e550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nidefaultavobjectpalette.cpp`; `thiscall`, `RET 0xc`,
/// `this` not read; by its body the fixed-string map's item setter): stores `value` in
/// `item` (+8) and, when the item's key (+4) is not already `*key`, the new key, counted, in
/// place of the old one, released.
pub fn fn_00a6e550(e: &mut Engine, _this: Ptr, item: Ptr, key: Ptr, value: u32) {
    let handle = e.mem.u32(key.addr());
    let i = item.addr();
    if e.mem.u32(i + 4) != handle {
        acquire(e, handle);
        let old = e.mem.u32(i + 4);
        release(e, old);
        e.mem.set_u32(i + 4, handle);
    }
    e.mem.set_u32(i + 8, value);
}

// Translated from 00a6e5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDefaultAVObjectPalette::ProcessClone` (Xbox PDB): `NiObject::ProcessClone`; the clone
/// of this palette from the cloning process's clone map (null when it is not there) gets as
/// scene the clone of the scene (when this palette has a scene; when the scene is not in the
/// map the lookup leaves the clone's own address there), and for every item of this map
/// (walked with `00a8f770`) whose key and object are set and whose object has a clone, the
/// clone palette's `SetAVObject` (slot `+0x90`) with the key and the object's clone.
pub fn ni_default_av_object_palette_process_clone(e: &mut Engine, this: Ptr, cloning_process: Ptr) {
    let t = this.addr();
    e.call(OBJECT_PROCESS_CLONE, &args![this, cloning_process]);
    e.with_stack(0x10, |e, locals| {
        let clone_slot = locals.addr();
        let position = locals.addr() + 4;
        let key = locals.addr() + 8;
        let value = locals.addr() + 0xc;
        let map = e.mem.u32(cloning_process.addr());
        e.mem.set_u32(clone_slot, 0);
        e.call(CLONE_MAP_GET_AT, &args![map, this, clone_slot]);
        let clone = e.mem.u32(clone_slot);
        let scene = e.mem.u32(t + 0x18);
        if scene != 0 {
            let map = e.mem.u32(cloning_process.addr());
            e.call(CLONE_MAP_GET_AT, &args![map, scene, clone_slot]);
            let cloned_scene = e.mem.u32(clone_slot);
            e.mem.set_u32(clone + 0x18, cloned_scene);
        }
        if e.mem.u32(t + 0x14) == 0 {
            return;
        }
        let first = first_item(e, e.mem.u32(t + 0xc), e.mem.u32(t + 0x10));
        e.mem.set_u32(position, first);
        while e.mem.u32(position) != 0 {
            let empty = empty_handle(e);
            e.mem.set_u32(key, empty);
            fn_00a8f770(
                e,
                Ptr::new(t + 8),
                Ptr::new(position),
                Ptr::new(key),
                Ptr::new(value),
            );
            let object = e.mem.u32(value);
            if e.mem.u32(key) != 0 && object != 0 {
                let map = e.mem.u32(cloning_process.addr());
                e.with_stack(4, |e, cloned| {
                    if e.call(CLONE_MAP_GET_AT, &args![map, object, cloned]).bool() {
                        let cloned_object = e.mem.u32(cloned.addr());
                        e.vcall(clone, 0x90, &args![key, cloned_object]);
                    }
                });
            }
            let handle = e.mem.u32(key);
            release(e, handle);
        }
    });
}

// Translated from 00a6e870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RecurseAndAddObjectsToPalette` (Xbox PDB; `cdecl`, (object, palette)): with both set,
/// a named object (`NiObjectNET` name +8) is added to the palette (`SetAVObject`, slot
/// `+0x90`, with the name and the object), and when the object is a node (`IsNode`, slot
/// `+0xc`, answers the node) each child of its child array (base +0xa0, count +0xa6) is
/// added the same way.
pub fn recurse_and_add_objects_to_palette(e: &mut Engine, object: Ptr, palette: Ptr) {
    let o = object.addr();
    if o == 0 || palette.addr() == 0 {
        return;
    }
    if e.mem.u32(o + 8) != 0 {
        e.vcall(palette.addr(), 0x90, &args![o + 8, object]);
    }
    let node = e.vcall(o, 0xc, &args![]).u32();
    if node == 0 {
        return;
    }
    let mut index = 0u32;
    while index < u32::from(e.mem.u16(node + 0xa6)) {
        let child = e.mem.u32(e.mem.u32(node + 0xa0) + index * 4);
        recurse_and_add_objects_to_palette(e, Ptr::new(child), palette);
        index += 1;
    }
}

// Translated from 00a6e8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RecurseAndRemoveObjectsFromPalette` (Xbox PDB; `cdecl`, (object, palette)): as
/// `RecurseAndAddObjectsToPalette`, with `SetAVObject` given null for each named object.
pub fn recurse_and_remove_objects_from_palette(e: &mut Engine, object: Ptr, palette: Ptr) {
    let o = object.addr();
    if o == 0 || palette.addr() == 0 {
        return;
    }
    if e.mem.u32(o + 8) != 0 {
        e.vcall(palette.addr(), 0x90, &args![o + 8, 0u32]);
    }
    let node = e.vcall(o, 0xc, &args![]).u32();
    if node == 0 {
        return;
    }
    let mut index = 0u32;
    while index < u32::from(e.mem.u16(node + 0xa6)) {
        let child = e.mem.u32(e.mem.u32(node + 0xa0) + index * 4);
        recurse_and_remove_objects_from_palette(e, Ptr::new(child), palette);
        index += 1;
    }
}

// Translated from 00a6ea40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDefaultAVObjectPalette::LoadBinary` (Xbox PDB): the base's `LoadBinary` (`00a88a40`),
/// the scene from a link id (`NiStream::ResolveLinkID`, uncounted), then a 4-byte count of
/// entries (read into the stream argument's stack word, set to 0 first), each a C string
/// (`NiStream::LoadCString`) and an object link id; with both set the object is added under
/// the string's fixed string (`00a5b690`, released after) through `SetAVObject` (slot
/// `+0x90`). Each string's copy is freed (`00aa10f0`).
pub fn ni_default_av_object_palette_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(BASE_LOAD_BINARY, &args![this, stream]);
    let scene = e.call(STREAM_RESOLVE_LINK_ID, &args![stream]).u32();
    e.mem.set_u32(this.addr() + 0x18, scene);
    e.with_stack(0xc, |e, locals| {
        let count_word = locals.addr();
        let string = locals.addr() + 4;
        let handle = locals.addr() + 8;
        // NiStream::m_pkIstr +0x24c; the input's position +4 and read function +8.
        let input = e.mem.u32(stream.addr() + 0x24c);
        e.mem.set_u32(count_word, 0);
        e.mem.set_u32(handle, 4);
        let function = e.mem.u32(input + 8);
        let done = e
            .call(function, &args![input, count_word, 4u32, handle, 1u32])
            .u32();
        let position = e.mem.u32(input + 4).wrapping_add(done);
        e.mem.set_u32(input + 4, position);
        while e.mem.u32(count_word) != 0 {
            let remaining = e.mem.u32(count_word) - 1;
            e.mem.set_u32(count_word, remaining);
            e.mem.set_u32(string, 0);
            e.call(STREAM_LOAD_C_STRING, &args![stream, string]);
            let object = e.call(STREAM_RESOLVE_LINK_ID, &args![stream]).u32();
            let chars = e.mem.u32(string);
            if chars != 0 && object != 0 {
                let fixed = e.call(FIXED_STRING_FROM_CHARS, &args![chars]).u32();
                e.mem.set_u32(handle, fixed);
                e.vcall(this.addr(), 0x90, &args![handle, object]);
                let fixed = e.mem.u32(handle);
                release(e, fixed);
            }
            let chars = e.mem.u32(string);
            e.call(TABLE_FREE, &args![chars]);
        }
    });
}

/// Whether two C strings differ (the compiler's inline `strcmp`, two bytes per step).
fn strings_differ(e: &Engine, first: u32, second: u32) -> bool {
    let mut a = first;
    let mut b = second;
    loop {
        let x = e.mem.u8(a);
        if x != e.mem.u8(b) {
            return true;
        }
        if x == 0 {
            return false;
        }
        let y = e.mem.u8(a + 1);
        if y != e.mem.u8(b + 1) {
            return true;
        }
        a += 2;
        b += 2;
        if y == 0 {
            return false;
        }
    }
}

// Translated from 00a6eb60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDefaultAVObjectPalette::LinkObject` (Xbox PDB): the base's `LinkObject` (`00a724d0`),
/// then the objects that were renamed since they were added are moved to their new names: a
/// local fixed-string map (vtables `0109ca08` then `0109ca18`, 0x25 buckets from
/// `00aa1070`, zeroed) collects each item of this map whose key and object are set and whose
/// object's name (`NiObjectNET` +8) is set and differs from the key, as key to the new name;
/// then for each collected entry the object filed under the old key (looked up in this map's
/// chain of the key's bucket, `key % bucket count`; null when missing) is filed under the new
/// name's fixed string (`00a5b690`, `SetAt`) and the old key removed (`RemoveAt`). The local
/// map is emptied twice (`RemoveAll`, as its two destructors run) and its buckets freed.
pub fn ni_default_av_object_palette_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    let t = this.addr();
    e.call(BASE_LINK_OBJECT, &args![this, stream]);
    e.with_stack(0x20, |e, locals| {
        let local_map = Ptr::<NiTPointerMap>::new(locals.addr());
        let m = local_map.addr();
        let position = m + 0x10;
        let key = m + 0x14;
        let value = m + 0x18;
        let handle = m + 0x1c;
        e.mem.set_u32(m, LOCAL_MAP_BASE_VTABLE);
        e.set(local_map, NiTPointerMap::m_uiHashSize, 0x25);
        e.set(local_map, NiTPointerMap::m_uiCount, 0);
        let table = e.call(TABLE_ALLOCATE, &args![0x94u32]).u32();
        let size = e.get(local_map, NiTPointerMap::m_uiHashSize);
        e.set(local_map, NiTPointerMap::m_ppkHashTable, table);
        e.call(MEMSET, &args![table, 0u32, size.wrapping_mul(4)]);
        e.mem.set_u32(m, LOCAL_MAP_VTABLE);
        let first = first_item(e, e.mem.u32(t + 0xc), e.mem.u32(t + 0x10));
        e.mem.set_u32(position, first);
        while e.mem.u32(position) != 0 {
            let empty = empty_handle(e);
            e.mem.set_u32(key, empty);
            fn_00a8f770(
                e,
                Ptr::new(t + 8),
                Ptr::new(position),
                Ptr::new(key),
                Ptr::new(value),
            );
            let old = e.mem.u32(key);
            let object = e.mem.u32(value);
            if old != 0 && object != 0 {
                let name = e.mem.u32(object + 8);
                if name != 0 && strings_differ(e, old, name) {
                    e.call(MAP_SET_AT, &args![m, key, name]);
                }
            }
            release(e, old);
        }
        let first = first_item(
            e,
            e.get(local_map, NiTPointerMap::m_uiHashSize),
            e.get(local_map, NiTPointerMap::m_ppkHashTable),
        );
        e.mem.set_u32(position, first);
        while e.mem.u32(position) != 0 {
            let empty = empty_handle(e);
            e.mem.set_u32(key, empty);
            fn_00a8f770(
                e,
                Ptr::new(m),
                Ptr::new(position),
                Ptr::new(key),
                Ptr::new(value),
            );
            let old = e.mem.u32(key);
            let name = e.mem.u32(value);
            if old != 0 && name != 0 {
                let fixed = e.call(FIXED_STRING_FROM_CHARS, &args![name]).u32();
                let bucket = old % e.mem.u32(t + 0xc);
                e.mem.set_u32(handle, fixed);
                let mut item = e.mem.u32(e.mem.u32(t + 0x10) + bucket * 4);
                let mut found = 0;
                while item != 0 {
                    if old == e.mem.u32(item + 4) {
                        found = e.mem.u32(item + 8);
                        break;
                    }
                    item = e.mem.u32(item);
                }
                e.call(MAP_SET_AT, &args![t + 8, handle, found]);
                e.call(MAP_REMOVE_AT, &args![t + 8, key]);
                release(e, fixed);
            }
            release(e, old);
        }
        e.mem.set_u32(m, LOCAL_MAP_VTABLE);
        e.call(MAP_REMOVE_ALL, &args![m]);
        e.mem.set_u32(m, LOCAL_MAP_BASE_VTABLE);
        e.call(MAP_REMOVE_ALL, &args![m]);
        let table = e.get(local_map, NiTPointerMap::m_ppkHashTable);
        e.call(TABLE_FREE, &args![table]);
    });
}

// Translated from 00a6ef50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nidefaultavobjectpalette.cpp`; `thiscall`, `this` not
/// read; by its body the fixed-string map's item allocation): a 0xc-byte item from the
/// memory manager with its key word (+4) 0 (written at 4 for a failed allocation, as the
/// exe does). Answers the item.
pub fn fn_00a6ef50(e: &mut Engine, _this: Ptr) -> u32 {
    let item = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0xcu32])
        .u32();
    e.mem.set_u32(item.wrapping_add(4), 0);
    item
}

// Translated from 00a6ef70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDefaultAVObjectPalette::CreateClone` (Xbox PDB): a new palette (0x1c bytes, `operator
/// new`, the `NiObject` constructor, the vtable `01096284`, an empty map of 0x25 zeroed
/// buckets from `00aa1070`, no scene; null when the allocation fails) given the base's
/// members (`00aa1000`, which runs on null too). The items are filled in by `ProcessClone`.
pub fn ni_default_av_object_palette_create_clone(
    e: &mut Engine,
    this: Ptr,
    cloning_process: Ptr,
) -> Ptr {
    let block = e.call(OPERATOR_NEW, &args![0x1cu32]).u32();
    if block == 0 {
        e.call(OBJECT_COPY_MEMBERS, &args![this, 0u32, cloning_process]);
        return Ptr::new(0);
    }
    e.call(OBJECT_CONSTRUCT, &args![block]);
    e.mem.set_u32(block, NI_DEFAULT_AV_OBJECT_PALETTE_VTABLE);
    e.mem.set_u32(block + 8, FIXED_STRING_MAP_BASE_VTABLE);
    e.mem.set_u32(block + 0xc, 0x25);
    e.mem.set_u32(block + 0x14, 0);
    let table = e.call(TABLE_ALLOCATE, &args![0x94u32]).u32();
    let size = e.mem.u32(block + 0xc);
    e.mem.set_u32(block + 0x10, table);
    e.call(MEMSET, &args![table, 0u32, size.wrapping_mul(4)]);
    e.mem.set_u32(block + 8, FIXED_STRING_MAP_VTABLE);
    e.mem.set_u32(block + 0x18, 0);
    e.call(OBJECT_COPY_MEMBERS, &args![this, block, cloning_process]);
    Ptr::new(block)
}

// Translated from 00a8f770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTFixedStringMapBase<..., NiAVObject *>::GetNext` (Xbox PDB; `thiscall` on the map,
/// (&position, &key, &value)): the item at `*position` gives its key (assigned to `*key` as a
/// fixed string: the new one counted, the old one released, when they differ) and its
/// value; `*position` moves to the next item of its chain or else of the following buckets
/// (from the item key's bucket, `key % bucket count`, plus one), 0 at the end.
pub fn fn_00a8f770(e: &mut Engine, this: Ptr, position: Ptr, key: Ptr, value: Ptr) {
    let map = Ptr::<NiTPointerMap>::new(this.addr());
    let item = e.mem.u32(position.addr());
    let handle = e.mem.u32(item + 4);
    if e.mem.u32(key.addr()) != handle {
        acquire(e, handle);
        let old = e.mem.u32(key.addr());
        release(e, old);
        e.mem.set_u32(key.addr(), handle);
    }
    let item_value = e.mem.u32(item + 8);
    e.mem.set_u32(value.addr(), item_value);
    let next = e.mem.u32(item);
    if next != 0 {
        e.mem.set_u32(position.addr(), next);
        return;
    }
    let size = e.get(map, NiTPointerMap::m_uiHashSize);
    let mut index = e.mem.u32(item + 4) % size + 1;
    while index < size {
        let table = e.get(map, NiTPointerMap::m_ppkHashTable);
        let candidate = e.mem.u32(table + index * 4);
        if candidate != 0 {
            e.mem.set_u32(position.addr(), candidate);
            return;
        }
        index += 1;
    }
    e.mem.set_u32(position.addr(), 0);
}

/// `00a724d0` (`thiscall`, the stream): the base's `SaveBinary` (the body the base's
/// `LinkObject` shares; the engine map names it `NiPSysEmitterCtlrData::LinkObject`).
const BASE_SAVE_BINARY: u32 = 0x00a7_24d0;
/// `00a64160` (`thiscall` on the stream, the characters): writes a C string.
const STREAM_SAVE_C_STRING: u32 = 0x00a6_4160;
/// `00a88a50` (`thiscall`, the other object): the base's `IsEqual` (the engine map names it
/// `Ni2DBuffer::IsEqual`, folded code).
const BASE_IS_EQUAL: u32 = 0x00a8_8a50;
/// `00ee8ac0` (`thiscall` on a map): the first item of its buckets, or 0.
const MAP_GET_FIRST_POS: u32 = 0x00ee_8ac0;
/// The memory manager's `Deallocate` (`thiscall`, the block).
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// `NiAVObjectPalette::GetViewerStrings` (`thiscall`, the strings array).
const BASE_GET_VIEWER_STRINGS: u32 = 0x00a8_8a70;
/// `NiGetViewerString` of a class name (`cdecl`, one word).
const GET_VIEWER_STRING: u32 = 0x00a7_d330;
/// `0096ad30` (`thiscall`, the new size) grows an `NiTArray`; `0096ae90` (`thiscall`, index
/// and the address of the element) stores an element.
const ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const ARRAY_SET_AT: u32 = 0x0096_ae90;
/// `sprintf` (`cdecl`: buffer, format, then the format's arguments).
const SPRINTF: u32 = 0x00ec_623a;
/// The first word of `NiDefaultAVObjectPalette`'s `NiRTTI` (its class name).
const PALETTE_RTTI_NAME: u32 = 0x011f_4a04;
/// `"m_pkScene = %s"`, `"UNKNOWN"` and `"%s"`.
const FORMAT_SCENE: u32 = 0x0109_ca24;
const TEXT_UNKNOWN: u32 = 0x0101_5890;
const FORMAT_STRING: u32 = 0x0101_9f08;

// Translated from 00a6e6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDefaultAVObjectPalette::SaveBinary` (Xbox PDB): the base's `SaveBinary` (`00a724d0`), the
/// scene's link id (the stream's slot +0x30), the item count (+0x14) as 4 bytes (from a local
/// word; the stream argument's stack word is the byte-count word), then for every item (walked
/// from the first non-null bucket with [`fn_00a8f770`], the key taken into the stream
/// argument's stack word, set to the empty string first) the key's characters (`00a64160`)
/// and the object's link id; each key is released after.
///
/// Ghidra drops the release as unreachable; the disassembly runs it.
pub fn ni_default_av_object_palette_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let t = this.addr();
    e.call(BASE_SAVE_BINARY, &args![this, stream]);
    let scene = e.mem.u32(t + 0x18);
    e.vcall(stream.addr(), 0x30, &args![scene]);
    e.with_stack(0x10, |e, locals| {
        let position = locals.addr();
        let count = locals.addr() + 4;
        let value = locals.addr() + 8;
        let slot = locals.addr() + 0xc;
        let items = e.mem.u32(t + 0x14);
        // NiStream::m_pkOstr +0x250; the output's position +4 and write function +0xc.
        let output = e.mem.u32(stream.addr() + 0x250);
        e.mem.set_u32(count, items);
        let function = e.mem.u32(output + 0xc);
        e.mem.set_u32(slot, 4);
        let done = e
            .call(function, &args![output, count, 4u32, slot, 1u32])
            .u32();
        let at = e.mem.u32(output + 4).wrapping_add(done);
        e.mem.set_u32(output + 4, at);
        if e.mem.u32(t + 0x14) == 0 {
            return;
        }
        let size = e.mem.u32(t + 0xc);
        let table = e.mem.u32(t + 0x10);
        let first = first_item(e, size, table);
        e.mem.set_u32(position, first);
        if first == 0 {
            return;
        }
        loop {
            let empty = empty_handle(e);
            e.mem.set_u32(slot, empty);
            fn_00a8f770(
                e,
                Ptr::new(t + 8),
                Ptr::new(position),
                Ptr::new(slot),
                Ptr::new(value),
            );
            let key = e.mem.u32(slot);
            e.call(STREAM_SAVE_C_STRING, &args![stream, key]);
            let object = e.mem.u32(value);
            e.vcall(stream.addr(), 0x30, &args![object]);
            release(e, key);
            if e.mem.u32(position) == 0 {
                break;
            }
        }
    });
}

// Translated from 00a6e7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDefaultAVObjectPalette::IsEqual` (Xbox PDB): the base's comparison (`00a88a50`) and the
/// same item count (+0x14); then every key of this map (walked from `00ee8ac0` with
/// [`fn_00a8f770`], into the `other` argument's stack word, set to the empty string first)
/// must find an object in `other` (its slot +0x8c with the key); the key is released after
/// each lookup. The objects themselves are not compared.
pub fn ni_default_av_object_palette_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let t = this.addr();
    if !e.call(BASE_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let count = e.mem.u32(t + 0x14);
    if e.mem.u32(other.addr() + 0x14) != count {
        return false;
    }
    if count == 0 {
        return true;
    }
    let map = t + 8;
    let first = e.call(MAP_GET_FIRST_POS, &args![map]).u32();
    if first == 0 {
        return true;
    }
    e.with_stack(0xc, |e, locals| {
        let position = locals.addr();
        let value = locals.addr() + 4;
        let slot = locals.addr() + 8;
        e.mem.set_u32(position, first);
        loop {
            let empty = empty_handle(e);
            e.mem.set_u32(slot, empty);
            fn_00a8f770(
                e,
                Ptr::new(map),
                Ptr::new(position),
                Ptr::new(slot),
                Ptr::new(value),
            );
            let found = e.vcall(other.addr(), 0x8c, &args![slot]).u32();
            let key = e.mem.u32(slot);
            release(e, key);
            if found == 0 {
                return false;
            }
            if e.mem.u32(position) == 0 {
                return true;
            }
        }
    })
}

// Translated from 00a6eb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTFixedStringMap<char const *>::_scalar_deleting_destructor_` (Xbox PDB): the map's two
/// destructors (vtable `0109ca18`, `RemoveAll`; vtable `0109ca08`, `RemoveAll`), its buckets
/// (+8) freed (`00aa10f0`) and, when bit 0 of the flags byte is set, the block given back to
/// the memory manager (`00aa4060`). Answers `this`.
pub fn ni_t_fixed_string_map_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.mem.set_u32(this.addr(), LOCAL_MAP_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    e.mem.set_u32(this.addr(), LOCAL_MAP_BASE_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let table = e.mem.u32(this.addr() + 8);
    e.call(TABLE_FREE, &args![table]);
    if flags as u8 & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// Appends the string pointer held in the word at `slot` to the viewer-string array (grown to
/// count + `m_usGrowBy` by `0096ad30` when full, stored by `0096ae90`).
fn append_viewer_string(e: &mut Engine, strings: Ptr, slot: u32) {
    let count = e.mem.u16(strings.addr() + 0xa) as u32;
    let capacity = e.mem.u16(strings.addr() + 8) as u32;
    if count >= capacity {
        let grow_by = e.mem.u16(strings.addr() + 0xe) as u32;
        e.call(ARRAY_SET_SIZE, &args![strings, grow_by + count]);
    }
    e.call(ARRAY_SET_AT, &args![strings, count, slot]);
}

// Translated from 00a6ed90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDefaultAVObjectPalette::GetViewerStrings` (Xbox PDB): the base's strings (`00a88a70`),
/// the class name (`NiGetViewerString` of the first word of the `NiRTTI` at `011f4a04`), then
/// `"m_pkScene = <scene name>"` (`"UNKNOWN"` for a scene without a name; the scene at +0x18 is
/// read without a null check) in 0x60 bytes from the memory manager; then for every item (as
/// `SaveBinary` walks them) its key as `"%s"` in 0x60 bytes, appended by the array's inline
/// `SetAtGrow` (grown by `0096ad30` when full; the count of non-null elements, +0xc, follows
/// the stored pointer), the key released after.
pub fn ni_default_av_object_palette_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    let t = this.addr();
    let s = strings.addr();
    e.call(BASE_GET_VIEWER_STRINGS, &args![this, strings]);
    e.with_stack(0x10, |e, locals| {
        let position = locals.addr();
        let value = locals.addr() + 4;
        let slot = locals.addr() + 8;
        let class_name = e.global::<u32>(PALETTE_RTTI_NAME);
        let text = e.call(GET_VIEWER_STRING, &args![class_name]).u32();
        e.mem.set_u32(slot, text);
        append_viewer_string(e, strings, slot);
        let text = e
            .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x60u32])
            .u32();
        let scene = e.mem.u32(t + 0x18);
        let mut name = e.mem.u32(scene.wrapping_add(8));
        e.mem.set_u32(slot, text);
        if name == 0 {
            name = TEXT_UNKNOWN;
        }
        e.call(SPRINTF, &args![text, FORMAT_SCENE, name]);
        append_viewer_string(e, strings, slot);
        if e.mem.u32(t + 0x14) == 0 {
            return;
        }
        let size = e.mem.u32(t + 0xc);
        let map = t + 8;
        let table = e.mem.u32(t + 0x10);
        let first = first_item(e, size, table);
        e.mem.set_u32(position, first);
        if first == 0 {
            return;
        }
        loop {
            let empty = empty_handle(e);
            e.mem.set_u32(slot, empty);
            fn_00a8f770(
                e,
                Ptr::new(map),
                Ptr::new(position),
                Ptr::new(slot),
                Ptr::new(value),
            );
            let text = e
                .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x60u32])
                .u32();
            let key = e.mem.u32(slot);
            e.call(SPRINTF, &args![text, FORMAT_STRING, key]);
            let index = e.mem.u16(s + 0xa) as u32;
            let capacity = e.mem.u16(s + 8) as u32;
            if index >= capacity {
                let grow_by = e.mem.u16(s + 0xe) as u32;
                e.call(ARRAY_SET_SIZE, &args![strings, grow_by + index]);
            }
            let used = e.mem.u16(s + 0xa) as u32;
            if index < used {
                let base = e.mem.u32(s + 4);
                let held = e.mem.u32(base + index * 4);
                if text != 0 {
                    if held == 0 {
                        let non_null = e.mem.u16(s + 0xc).wrapping_add(1);
                        e.mem.set_u16(s + 0xc, non_null);
                    }
                } else if held != 0 {
                    let non_null = e.mem.u16(s + 0xc).wrapping_add(0xffff);
                    e.mem.set_u16(s + 0xc, non_null);
                }
            } else {
                e.mem.set_u16(s + 0xa, (index + 1) as u16);
                if text != 0 {
                    let non_null = e.mem.u16(s + 0xc).wrapping_add(1);
                    e.mem.set_u16(s + 0xc, non_null);
                }
            }
            let base = e.mem.u32(s + 4);
            e.mem.set_u32(base + index * 4, text);
            release(e, key);
            if e.mem.u32(position) == 0 {
                break;
            }
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a6_e540, ni_default_av_object_palette_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00a6_e550, fn_00a6e550(Ptr, Ptr, Ptr, u32)),
        entry!(
            0x00a6_e5b0,
            ni_default_av_object_palette_process_clone(Ptr, Ptr)
        ),
        entry!(0x00a6_e870, recurse_and_add_objects_to_palette(Ptr, Ptr)),
        entry!(
            0x00a6_e8e0,
            recurse_and_remove_objects_from_palette(Ptr, Ptr)
        ),
        entry!(
            0x00a6_ea40,
            ni_default_av_object_palette_load_binary(Ptr, Ptr)
        ),
        entry!(
            0x00a6_eb60,
            ni_default_av_object_palette_link_object(Ptr, Ptr)
        ),
        entry!(0x00a6_ef50, fn_00a6ef50(Ptr) -> u32),
        entry!(
            0x00a6_ef70,
            ni_default_av_object_palette_create_clone(Ptr, Ptr) -> Ptr
        ),
        entry!(0x00a8_f770, fn_00a8f770(Ptr, Ptr, Ptr, Ptr)),
        entry!(
            0x00a6_e6c0,
            ni_default_av_object_palette_save_binary(Ptr, Ptr)
        ),
        entry!(0x00a6_e7b0, ni_default_av_object_palette_is_equal(Ptr, Ptr) -> bool),
        entry!(
            0x00a6_eb20,
            ni_t_fixed_string_map_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00a6_ed90,
            ni_default_av_object_palette_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00a6e540_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a6_e540;
        const TARGET: u32 = 0x00a8_4c70;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }
}

#[cfg(test)]
mod palette_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The vtable of the test objects: `IsNode` (+0xc) answers the word at +0x40 and
    /// `SetAVObject` (+0x90) records its words.
    const VTABLE: u32 = 0x0300_0000;
    const IS_NODE: u32 = 0x0300_100c;
    const SET_AV_OBJECT: u32 = 0x0300_1090;

    /// An engine with the empty string's handle 0x9000 (uncounted), counting interlocked
    /// doubles and the test vtable.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0109_b000, 0x1000);
        e.mem.set_u32(EMPTY_FIXED_STRING, 0x9000);
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        let mut slots = vec![0u32; 0x25];
        slots[3] = IS_NODE;
        slots[0x24] = SET_AV_OBJECT;
        e.put_vtable(VTABLE, &slots);
        e.register(IS_NODE, |e, a| e.mem.u32(a[0] + 0x40).into_ret());
        e.register(SET_AV_OBJECT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    /// A counted fixed-string handle (count word at -8) for `text`.
    fn handle(e: &mut Engine, text: &[u8]) -> u32 {
        let block = e.mem.alloc(8 + text.len() as u32 + 1);
        e.mem.set_u32(block, 1);
        e.mem.set_cstr(block + 8, text);
        block + 8
    }

    fn object(e: &mut Engine, name: u32) -> u32 {
        let o = e.mem.alloc(0xb0);
        e.mem.set_u32(o, VTABLE);
        e.mem.set_u32(o + 8, name);
        o
    }

    /// A palette whose map has `buckets` buckets and the given items (key, value), each put
    /// in the bucket `key % buckets` (chained in order).
    fn palette(e: &mut Engine, buckets: u32, items: &[(u32, u32)]) -> u32 {
        let p = e.mem.alloc(0x1c);
        e.mem.set_u32(p, VTABLE);
        let table = e.mem.alloc(4 * buckets);
        e.mem.set_u32(p + 0xc, buckets);
        e.mem.set_u32(p + 0x10, table);
        e.mem.set_u32(p + 0x14, items.len() as u32);
        for (key, value) in items.iter().rev() {
            let item = e.mem.alloc(0xc);
            let slot = table + 4 * (key % buckets);
            e.mem.set_u32(item, e.mem.u32(slot));
            e.mem.set_u32(item + 4, *key);
            e.mem.set_u32(item + 8, *value);
            e.mem.set_u32(slot, item);
        }
        p
    }

    #[test]
    fn item_setter_counts_a_new_key() {
        let mut e = engine();
        let old = handle(&mut e, b"a");
        let new = handle(&mut e, b"b");
        let item = e.mem.alloc(0xc);
        e.mem.set_u32(item + 4, old);
        let key = e.mem.alloc(4);
        e.mem.set_u32(key, new);
        e.call(0x00a6_e550, &args![0u32, item, key, 0x77u32]);
        assert_eq!((e.mem.u32(item + 4), e.mem.u32(item + 8)), (new, 0x77));
        assert_eq!((e.mem.u32(old - 8), e.mem.u32(new - 8)), (0, 2));
        // The same key: only the value.
        e.call(0x00a6_e550, &args![0u32, item, key, 0x88u32]);
        assert_eq!(e.mem.u32(new - 8), 2);
        assert_eq!(e.mem.u32(item + 8), 0x88);
    }

    #[test]
    fn get_next_walks_chains_and_buckets() {
        let mut e = engine();
        let a = handle(&mut e, b"a");
        let b = handle(&mut e, b"b");
        let buckets = 7;
        let p = palette(&mut e, buckets, &[(a, 0x100), (b, 0x200)]);
        let table = e.mem.u32(p + 0x10);
        let first = first_item(&e, buckets, table);
        let position = e.mem.alloc(4);
        let key = e.mem.alloc(4);
        let value = e.mem.alloc(4);
        e.mem.set_u32(position, first);
        e.mem.set_u32(key, 0x9000);
        let mut seen = vec![];
        while e.mem.u32(position) != 0 {
            e.call(0x00a8_f770, &args![p + 8, position, key, value]);
            seen.push((e.mem.u32(key), e.mem.u32(value)));
        }
        let held = seen.last().unwrap().0;
        seen.sort();
        let mut expected = vec![(a, 0x100), (b, 0x200)];
        expected.sort();
        assert_eq!(seen, expected);
        // The last key stays held by the key word.
        assert_eq!(e.mem.u32(held - 8), 2);
    }

    #[test]
    fn recursion_adds_and_removes_named_objects_of_the_tree() {
        let mut e = engine();
        let root_name = handle(&mut e, b"root");
        let child_name = handle(&mut e, b"child");
        let root = object(&mut e, root_name);
        let child = object(&mut e, child_name);
        let unnamed = object(&mut e, 0);
        let children = e.mem.alloc(12);
        e.mem.set_u32(children, child);
        e.mem.set_u32(children + 8, unnamed);
        e.mem.set_u32(root + 0x40, root);
        e.mem.set_u32(root + 0xa0, children);
        e.mem.set_u16(root + 0xa6, 3);
        let p = palette(&mut e, 7, &[]);
        e.call(0x00a6_e870, &args![root, p]);
        assert_eq!(
            calls_to(&e, SET_AV_OBJECT),
            vec![vec![p, root + 8, root], vec![p, child + 8, child]]
        );
        e.call_log = Some(vec![]);
        e.call(0x00a6_e8e0, &args![root, p]);
        assert_eq!(
            calls_to(&e, SET_AV_OBJECT),
            vec![vec![p, root + 8, 0], vec![p, child + 8, 0]]
        );
        // A null palette does nothing.
        e.call_log = Some(vec![]);
        e.call(0x00a6_e870, &args![root, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn process_clone_gives_the_clone_the_cloned_scene_and_objects() {
        let mut e = engine();
        e.register(OBJECT_PROCESS_CLONE, |_, _| Ret::default());
        let a = handle(&mut e, b"a");
        let b = handle(&mut e, b"b");
        let scene = object(&mut e, 0);
        let objects = [object(&mut e, a), object(&mut e, b)];
        let p = palette(&mut e, 7, &[(a, objects[0]), (b, objects[1])]);
        e.mem.set_u32(p + 0x18, scene);
        let clone = palette(&mut e, 7, &[]);
        // The clone map: the palette, the scene and the first object have clones.
        let pairs = [(p, clone), (scene, 0x5555), (objects[0], 0x6666)];
        e.register_double(CLONE_MAP_GET_AT, move |e, a| {
            match pairs.iter().find(|(k, _)| *k == a[1]) {
                Some((_, v)) => {
                    e.mem.set_u32(a[2], *v);
                    1u32.into_ret()
                }
                None => 0u32.into_ret(),
            }
        });
        let process = e.mem.alloc(4);
        e.mem.set_u32(process, 0x4444);
        e.call(0x00a6_e5b0, &args![p, process]);
        assert_eq!(e.mem.u32(clone + 0x18), 0x5555);
        let sets = calls_to(&e, SET_AV_OBJECT);
        assert_eq!(sets.len(), 1);
        assert_eq!((sets[0][0], sets[0][2]), (clone, 0x6666));
        assert_eq!((e.mem.u32(a - 8), e.mem.u32(b - 8)), (1, 1));
    }

    #[test]
    fn load_binary_adds_each_named_link() {
        let mut e = engine();
        e.register(BASE_LOAD_BINARY, |_, _| Ret::default());
        e.register(TABLE_FREE, |_, _| Ret::default());
        let mut links = vec![0x1000u32, 0, 0x2000, 0x3000];
        e.register_double(STREAM_RESOLVE_LINK_ID, move |_, _| {
            links.remove(0).into_ret()
        });
        let mut strings = vec![0x7000u32, 0x7100, 0];
        e.register_double(STREAM_LOAD_C_STRING, move |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0);
            e.mem.set_u32(a[1], strings.remove(0));
            Ret::default()
        });
        let fixed = handle(&mut e, b"x");
        e.register_double(FIXED_STRING_FROM_CHARS, move |_, _| fixed.into_ret());
        let stream = e.mem.alloc(0x300);
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(stream + 0x24c, input);
        e.mem.set_u32(input + 8, 0x0310_0000);
        e.register(0x0310_0000, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            e.mem.set_u32(a[1], 3);
            4u32.into_ret()
        });
        let p = palette(&mut e, 7, &[]);
        e.call(0x00a6_ea40, &args![p, stream]);
        assert_eq!(e.mem.u32(p + 0x18), 0x1000);
        // Entries: ("7000", 0) skipped, ("7100", 0x2000) added, (null, 0x3000) skipped.
        let sets = calls_to(&e, SET_AV_OBJECT);
        assert_eq!(sets.len(), 1);
        assert_eq!(sets[0][2], 0x2000);
        assert_eq!(calls_to(&e, FIXED_STRING_FROM_CHARS), vec![vec![0x7100]]);
        assert_eq!(
            calls_to(&e, TABLE_FREE),
            vec![vec![0x7000], vec![0x7100], vec![0]]
        );
        assert_eq!(e.mem.u32(fixed - 8), 0);
    }

    #[test]
    fn link_object_moves_renamed_objects() {
        let mut e = engine();
        e.register(BASE_LINK_OBJECT, |_, _| Ret::default());
        e.register(TABLE_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(TABLE_FREE, |_, _| Ret::default());
        e.register(MEMSET, |_, _| Ret::default());
        e.register(MAP_REMOVE_ALL, |_, _| Ret::default());
        e.register(MAP_REMOVE_AT, |_, _| Ret::default());
        let fixed = handle(&mut e, b"new");
        e.register_double(FIXED_STRING_FROM_CHARS, move |_, _| fixed.into_ret());
        // The local map's SetAt puts a single item in its first bucket; this map's SetAt is
        // only recorded.
        e.register(MAP_SET_AT, |e, a| {
            let key = e.mem.u32(a[1]);
            if e.mem.u32(a[0]) == LOCAL_MAP_VTABLE {
                let item = e.mem.alloc(0xc);
                e.mem.set_u32(item + 4, key);
                e.mem.set_u32(item + 8, a[2]);
                let table = e.mem.u32(a[0] + 8);
                e.mem.set_u32(table, item);
                e.mem.set_u32(key - 8, e.mem.u32(key - 8) + 1);
            }
            Ret::default()
        });
        let old = handle(&mut e, b"old");
        let same = handle(&mut e, b"same");
        let renamed_name = handle(&mut e, b"new");
        let renamed = object(&mut e, renamed_name);
        let kept = object(&mut e, same);
        let p = palette(&mut e, 7, &[(old, renamed), (same, kept)]);
        e.call(0x00a6_eb60, &args![p, 0x5000u32]);
        let sets = calls_to(&e, MAP_SET_AT);
        // One rename collected (old -> "new"), then filed under the new name.
        assert_eq!(sets.len(), 2);
        assert_eq!(sets[0][2], renamed_name);
        assert_eq!(sets[1][0], p + 8);
        assert_eq!(sets[1][2], renamed);
        assert_eq!(calls_to(&e, MAP_REMOVE_AT)[0][0], p + 8);
        assert_eq!(calls_to(&e, MAP_REMOVE_ALL).len(), 2);
        assert_eq!(calls_to(&e, TABLE_FREE).len(), 1);
    }

    #[test]
    fn item_allocation_clears_the_key() {
        let mut e = engine();
        e.register(MEMORY_ALLOCATE, |e, a| {
            let block = e.mem.alloc(a[1]);
            e.mem.write(block, &[0xee; 0xc]);
            block.into_ret()
        });
        let item = e.call(0x00a6_ef50, &args![0u32]).u32();
        assert_eq!(e.mem.u32(item + 4), 0);
        assert_eq!(e.mem.u32(item), 0xeeee_eeee);
    }

    #[test]
    fn create_clone_builds_an_empty_palette() {
        let mut e = engine();
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(OBJECT_CONSTRUCT, |_, _| Ret::default());
        e.register(TABLE_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MEMSET, |_, _| Ret::default());
        e.register(OBJECT_COPY_MEMBERS, |_, _| Ret::default());
        let clone = e.call(0x00a6_ef70, &args![0x1000u32, 0x55u32]).u32();
        assert_eq!(e.mem.u32(clone), NI_DEFAULT_AV_OBJECT_PALETTE_VTABLE);
        assert_eq!(e.mem.u32(clone + 8), FIXED_STRING_MAP_VTABLE);
        assert_eq!(e.mem.u32(clone + 0xc), 0x25);
        assert_ne!(e.mem.u32(clone + 0x10), 0);
        assert_eq!((e.mem.u32(clone + 0x14), e.mem.u32(clone + 0x18)), (0, 0));
        assert_eq!(calls_to(&e, MEMSET)[0][1..], [0, 0x94]);
        assert_eq!(
            calls_to(&e, OBJECT_COPY_MEMBERS),
            vec![vec![0x1000, clone, 0x55]]
        );
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a6_ef70, &args![0x1000u32, 0x55u32]).u32(), 0);
        assert_eq!(calls_to(&e, OBJECT_COPY_MEMBERS)[1], vec![0x1000, 0, 0x55]);
    }
}

#[cfg(test)]
mod n015_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::calls_to;

    const OTHER_VTABLE: u32 = 0x0300_2000;
    const GET_AV_OBJECT: u32 = 0x0300_3000;
    const SAVE_LINK_ID: u32 = 0x0300_3004;
    const WRITE: u32 = 0x0300_3008;

    /// An engine with the empty string's handle 0x9000 (uncounted) and counting interlocked
    /// doubles.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0109_b000, 0x1000);
        e.map(0x011f_4000, 0x1000);
        e.mem.set_u32(EMPTY_FIXED_STRING, 0x9000);
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.call_log = Some(vec![]);
        e
    }

    /// A counted fixed-string handle (count word at -8, count 1) for `text`.
    fn handle(e: &mut Engine, text: &[u8]) -> u32 {
        let block = e.mem.alloc(8 + text.len() as u32 + 1);
        e.mem.set_u32(block, 1);
        e.mem.set_cstr(block + 8, text);
        block + 8
    }

    /// A palette whose map has `buckets` buckets and the given items (key, value).
    fn palette(e: &mut Engine, buckets: u32, items: &[(u32, u32)]) -> u32 {
        let p = e.mem.alloc(0x1c);
        let table = e.mem.alloc(4 * buckets);
        e.mem.set_u32(p + 0xc, buckets);
        e.mem.set_u32(p + 0x10, table);
        e.mem.set_u32(p + 0x14, items.len() as u32);
        for (key, value) in items.iter().rev() {
            let item = e.mem.alloc(0xc);
            let slot = table + 4 * (key % buckets);
            e.mem.set_u32(item, e.mem.u32(slot));
            e.mem.set_u32(item + 4, *key);
            e.mem.set_u32(item + 8, *value);
            e.mem.set_u32(slot, item);
        }
        p
    }

    fn stream(e: &mut Engine) -> (u32, u32) {
        e.put_vtable(0x0300_4000, &[0; 13]);
        e.mem.set_u32(0x0300_4030, SAVE_LINK_ID);
        e.register(SAVE_LINK_ID, |_, _| Ret::default());
        e.register(WRITE, |_, a| a[2].into_ret());
        let stream = e.mem.alloc(0x300);
        e.mem.set_u32(stream, 0x0300_4000);
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(output + 0xc, WRITE);
        e.mem.set_u32(stream + 0x250, output);
        (stream, output)
    }

    #[test]
    fn save_binary_writes_the_scene_count_and_items() {
        let mut e = engine();
        e.register(BASE_SAVE_BINARY, |_, _| Ret::default());
        e.register(STREAM_SAVE_C_STRING, |_, _| Ret::default());
        let (stream, output) = stream(&mut e);
        let first = handle(&mut e, b"a");
        let second = handle(&mut e, b"b");
        let this = palette(&mut e, 3, &[(first, 0x111), (second, 0x222)]);
        e.mem.set_u32(this + 0x18, 0x5555);
        e.call(0x00a6_e6c0, &args![this, stream]);
        assert_eq!(calls_to(&e, BASE_SAVE_BINARY), vec![vec![this, stream]]);
        let writes = calls_to(&e, WRITE);
        assert_eq!(writes.len(), 1);
        assert_eq!((writes[0][0], writes[0][2], writes[0][4]), (output, 4, 1));
        assert_eq!(e.mem.u32(output + 4), 4);
        let mut keys = calls_to(&e, STREAM_SAVE_C_STRING);
        keys.sort();
        let mut expected = vec![vec![stream, first], vec![stream, second]];
        expected.sort();
        assert_eq!(keys, expected);
        let mut links: Vec<u32> = calls_to(&e, SAVE_LINK_ID).iter().map(|c| c[1]).collect();
        assert_eq!(links.remove(0), 0x5555);
        links.sort();
        assert_eq!(links, vec![0x111, 0x222]);
        // Taken and released: the counts are back to 1.
        assert_eq!((e.mem.u32(first - 8), e.mem.u32(second - 8)), (1, 1));

        // An empty map writes only the count.
        let this = palette(&mut e, 3, &[]);
        e.call_log = Some(vec![]);
        e.call(0x00a6_e6c0, &args![this, stream]);
        assert!(calls_to(&e, STREAM_SAVE_C_STRING).is_empty());
        assert_eq!(calls_to(&e, WRITE).len(), 1);
    }

    #[test]
    fn is_equal_looks_up_every_key_in_the_other_palette() {
        let mut e = engine();
        e.register(BASE_IS_EQUAL, |_, _| 1u32.into_ret());
        e.register(MAP_GET_FIRST_POS, |e, a| {
            let size = e.mem.u32(a[0] + 4);
            let table = e.mem.u32(a[0] + 8);
            first_item(e, size, table).into_ret()
        });
        e.put_vtable(OTHER_VTABLE, &[0; 0x24]);
        e.mem.set_u32(OTHER_VTABLE + 0x8c, GET_AV_OBJECT);
        // The other palette knows the key "a" only.
        e.register(GET_AV_OBJECT, |e, a| {
            let key = e.mem.u32(a[1]);
            u32::from(e.mem.cstr(key) == b"a").into_ret()
        });
        let a = handle(&mut e, b"a");
        let b = handle(&mut e, b"b");
        let this = palette(&mut e, 3, &[(a, 0x111)]);
        let other = palette(&mut e, 3, &[(a, 0x999)]);
        e.mem.set_u32(other, OTHER_VTABLE);
        assert!(e.call(0x00a6_e7b0, &args![this, other]).bool());
        assert_eq!(calls_to(&e, GET_AV_OBJECT).len(), 1);
        assert_eq!(e.mem.u32(a - 8), 1);
        // A key the other palette lacks.
        let this = palette(&mut e, 3, &[(a, 0x111), (b, 0x222)]);
        e.mem.set_u32(other + 0x14, 2);
        assert!(!e.call(0x00a6_e7b0, &args![this, other]).bool());
        assert_eq!((e.mem.u32(a - 8), e.mem.u32(b - 8)), (1, 1));
        // Different counts, or a failed base comparison.
        e.mem.set_u32(other + 0x14, 1);
        assert!(!e.call(0x00a6_e7b0, &args![this, other]).bool());
        e.register(BASE_IS_EQUAL, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00a6_e7b0, &args![this, other]).bool());
        // No items: equal without a lookup.
        e.register(BASE_IS_EQUAL, |_, _| 1u32.into_ret());
        let empty = palette(&mut e, 3, &[]);
        let other_empty = palette(&mut e, 3, &[]);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00a6_e7b0, &args![empty, other_empty]).bool());
        assert!(calls_to(&e, MAP_GET_FIRST_POS).is_empty());
    }

    #[test]
    fn fixed_string_map_destructor_empties_and_frees() {
        let mut e = engine();
        e.register(MAP_REMOVE_ALL, |e, a| {
            let vtable = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 0xc, vtable);
            Ret::default()
        });
        e.register(TABLE_FREE, |_, _| Ret::default());
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        let map = e.mem.alloc(0x10);
        e.mem.set_u32(map + 8, 0x4444);
        assert_eq!(e.call(0x00a6_eb20, &args![map, 0u32]).u32(), map);
        let removes: Vec<u32> = calls_to(&e, MAP_REMOVE_ALL).iter().map(|c| c[0]).collect();
        assert_eq!(removes, vec![map, map]);
        assert_eq!(e.mem.u32(map), LOCAL_MAP_BASE_VTABLE);
        assert_eq!(e.mem.u32(map + 0xc), LOCAL_MAP_BASE_VTABLE);
        assert_eq!(calls_to(&e, TABLE_FREE), vec![vec![0x4444]]);
        assert!(calls_to(&e, MEMORY_DEALLOCATE).is_empty());
        e.call(0x00a6_eb20, &args![map, 0x101u32]);
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, map]]
        );
    }

    #[test]
    fn viewer_strings_list_the_class_scene_and_keys() {
        let mut e = engine();
        e.register(BASE_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(GET_VIEWER_STRING, |_, _| 0x7000u32.into_ret());
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(SPRINTF, |_, _| Ret::default());
        e.register(ARRAY_SET_SIZE, |e, a| {
            let base = e.mem.alloc(4 * a[1]);
            let old = e.mem.u32(a[0] + 4);
            let count = e.mem.u16(a[0] + 0xa) as u32;
            for index in 0..count {
                let word = e.mem.u32(old + 4 * index);
                e.mem.set_u32(base + 4 * index, word);
            }
            e.mem.set_u32(a[0] + 4, base);
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        e.register(ARRAY_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(base + 4 * a[1], value);
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            let non_null = e.mem.u16(a[0] + 0xc) + 1;
            e.mem.set_u16(a[0] + 0xc, non_null);
            Ret::default()
        });
        e.mem.set_u32(PALETTE_RTTI_NAME, 0x0109_b100);
        let a = handle(&mut e, b"a");
        let b = handle(&mut e, b"b");
        let this = palette(&mut e, 5, &[(a, 0x111), (b, 0x222)]);
        let scene = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0x18, scene);
        let strings = e.mem.alloc(0x10);
        let first_base = e.mem.alloc(4);
        e.mem.set_u32(strings + 4, first_base);
        e.mem.set_u16(strings + 8, 1);
        e.mem.set_u16(strings + 0xe, 1);
        e.call(0x00a6_ed90, &args![this, strings]);
        assert_eq!(calls_to(&e, GET_VIEWER_STRING), vec![vec![0x0109_b100]]);
        let sprintf = calls_to(&e, SPRINTF);
        assert_eq!(sprintf.len(), 3);
        assert_eq!(sprintf[0][1..], [FORMAT_SCENE, TEXT_UNKNOWN]);
        let mut keys: Vec<u32> = sprintf[1..].iter().map(|c| c[2]).collect();
        keys.sort();
        let mut expected = vec![a, b];
        expected.sort();
        assert_eq!(keys, expected);
        assert!(sprintf[1..].iter().all(|c| c[1] == FORMAT_STRING));
        // Four strings, all non-null; the array grew three times by one.
        assert_eq!(e.mem.u16(strings + 0xa), 4);
        assert_eq!(e.mem.u16(strings + 0xc), 4);
        assert_eq!(calls_to(&e, ARRAY_SET_SIZE).len(), 3);
        let base = e.mem.u32(strings + 4);
        assert_eq!(e.mem.u32(base), 0x7000);
        assert_eq!(e.mem.u32(base + 4), sprintf[0][0]);
        assert_eq!(e.mem.u32(base + 8), sprintf[1][0]);
        assert_eq!(e.mem.u32(base + 0xc), sprintf[2][0]);
        assert_eq!((e.mem.u32(a - 8), e.mem.u32(b - 8)), (1, 1));

        // A named scene, no items.
        let this = palette(&mut e, 5, &[]);
        e.mem.set_u32(scene + 8, a);
        e.mem.set_u32(this + 0x18, scene);
        e.call_log = Some(vec![]);
        e.call(0x00a6_ed90, &args![this, strings]);
        let sprintf = calls_to(&e, SPRINTF);
        assert_eq!(sprintf.len(), 1);
        assert_eq!(sprintf[0][2], a);
    }
}
