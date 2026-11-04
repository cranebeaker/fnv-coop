//! `fallout shared/pathfinding/navmeshinfomap.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006b81d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Folded map key equality: compares full 32-bit words, returns AL.
pub fn keys_equal(_e: &mut Engine, _this: Ptr, left: u32, right: u32) -> u8 {
    u8::from(left == right)
}

/// The virtual slot (byte offset) of the map that answers the bucket index of a key.
const SLOT_BUCKET_OF_KEY: u32 = 0x04;

// Translated from 006b7f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`navmeshinfomap.cpp`). `thiscall`, `RET 0xc`: the iterator step of
/// a chained hash map (`this`: vtable, bucket count at +4, bucket array at +8; an entry: next
/// at +0, key at +4, value at +8). `iterator` points to the current entry: its key and value are
/// stored through `key` and `value`, then the iterator moves to the entry's next, or, at the end
/// of the chain, to the first entry of a later non-empty bucket (the current key's bucket index
/// comes from virtual slot +4 of the map), or to null after the last bucket. The key is read
/// from the entry again for the slot call, as the exe does.
pub fn fn_006b7f20(e: &mut Engine, this: Ptr, iterator: Ptr, key: Ptr, value: Ptr) {
    let entry = e.mem.u32(iterator.addr());
    let entry_key = e.mem.u32(entry.wrapping_add(4));
    e.mem.set_u32(key.addr(), entry_key);
    let entry_value = e.mem.u32(entry.wrapping_add(8));
    e.mem.set_u32(value.addr(), entry_value);
    let next = e.mem.u32(entry);
    if next != 0 {
        e.mem.set_u32(iterator.addr(), next);
        return;
    }
    let key_again = e.mem.u32(entry.wrapping_add(4));
    let bucket = e
        .vcall(this.addr(), SLOT_BUCKET_OF_KEY, &args![key_again])
        .u32();
    let mut index = bucket.wrapping_add(1);
    loop {
        if index >= e.mem.u32(this.addr().wrapping_add(4)) {
            e.mem.set_u32(iterator.addr(), 0);
            return;
        }
        let buckets = e.mem.u32(this.addr().wrapping_add(8));
        let first = e.mem.u32(buckets.wrapping_add(index.wrapping_mul(4)));
        if first != 0 {
            e.mem.set_u32(iterator.addr(), first);
            return;
        }
        index = index.wrapping_add(1);
    }
}

/// `TESForm::TESForm` (`00483370`, engine map).
const TES_FORM_CTOR: u32 = 0x0048_3370;
/// `004f15a0` (unnamed in the engine map; `thiscall`, one word): stores the
/// byte at +4 (here `TESForm`'s form type, 0x38).
const SET_FORM_TYPE_004F15A0: u32 = 0x004f_15a0;
/// The vtable of `NavMeshInfoMap` (RTTI `.?AVNavMeshInfoMap@@`).
const NAV_MESH_INFO_MAP_VTABLE: u32 = 0x0106_bb8c;
/// The vtable of `NiTPointerMap<unsigned int, NavMeshInfo*>` (RTTI).
const INFO_MAP_VTABLE: u32 = 0x0106_c134;
/// The vtable of the `NiTMap` of worldspace ids to cell-key maps (RTTI
/// `.?AV?$NiTMap@IPAV?$NiTMap@IPAV?$BSSimpleArray@PAVNavMeshInfo@@...`).
const WORLD_MAP_VTABLE: u32 = 0x0106_c154;
/// The vtable of the `NiTMap` of cell keys to `BSSimpleArray<NavMeshInfo*>*` (RTTI).
const CELL_MAP_VTABLE: u32 = 0x0106_c174;
/// The vtable of the `NiTMapBase<NiTPointerAllocator<unsigned int>, unsigned int, NavMeshInfo*>` (RTTI).
const INFO_MAP_BASE_VTABLE: u32 = 0x0106_c194;
/// The vtable of the `NiTMapBase` under the worldspace map (RTTI `NiTMapBase<DFALL<NiTMapItem<...NiTMap...>>>`).
const WORLD_MAP_BASE_VTABLE: u32 = 0x0106_c1b4;
/// The vtable of the `NiTMapBase` under the cell map (RTTI `NiTMapBase<DFALL<NiTMapItem<...BSSimpleArray...>>>`).
const CELL_MAP_BASE_VTABLE: u32 = 0x0106_c1d4;
/// `00aa1070` (unnamed in the engine map; cdecl, the size): the map's
/// bucket array allocation.
const BUCKET_ALLOCATE_00AA1070: u32 = 0x00aa_1070;
/// `00aa10f0` (unnamed in the engine map; cdecl, the block): frees the
/// bucket array.
const BUCKET_FREE_00AA10F0: u32 = 0x00aa_10f0;
/// `00403d30` (unnamed in the engine map; cdecl: destination, byte, count): fills memory.
const FILL_00403D30: u32 = 0x0040_3d30;
/// `00438af0` (unnamed in the engine map): empties a map (`RemoveAll`).
const MAP_REMOVE_ALL_00438AF0: u32 = 0x0043_8af0;
/// `MemoryManager` allocation (`00401000`, cdecl, the size).
const ALLOCATE: u32 = 0x0040_1000;
/// `MemoryManager` deallocation (`00401030`, cdecl, the block).
const DEALLOCATE: u32 = 0x0040_1030;
/// `004b9ba0` (unnamed in the engine map): a map's first position (the
/// first entry, or null).
const MAP_FIRST_004B9BA0: u32 = 0x004b_9ba0;
/// `006b6390` (`NavMeshInfo` scalar deleting destructor; `thiscall`, `RET 4`).
const NAV_MESH_INFO_DELETE: u32 = 0x006b_6390;
/// `0044ddc0` (unnamed in the engine map): an array's size.
const ARRAY_SIZE_0044DDC0: u32 = 0x0044_ddc0;
/// `006a7ad0` (unnamed in the engine map; `thiscall`, `RET 4`): the address
/// of an array's element.
const ARRAY_AT_006A7AD0: u32 = 0x006a_7ad0;
/// `00877a30` (unnamed in the engine map; `thiscall`, `RET 4`): the address
/// of an array's element.
const ARRAY_AT_00877A30: u32 = 0x0087_7a30;
/// `006286d0` (named `LowProcess::GetNumberOfItemsActivated` in the engine
/// map, a folded body): the word the cell check counts non-zero.
const INFO_VALUE_006286D0: u32 = 0x0062_86d0;
/// `00484730` (unnamed in the engine map; `thiscall`, `RET 4`): the
/// `TESForm` step `006b68b0` runs first with its byte.
const FORM_STEP_00484730: u32 = 0x0048_4730;
/// `004534f0` (unnamed in the engine map; `thiscall`, `RET 4`): run on each
/// info with 0.
const INFO_STEP_004534F0: u32 = 0x0045_34f0;
/// `00690830` (unnamed in the engine map): the object the cast checks.
const INFO_PARENT_00690830: u32 = 0x0069_0830;
/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The cast's source type descriptor (`.?AVTESForm@@`).
const CAST_SOURCE_TYPE: u32 = 0x0118_3028;
/// The cast's target type descriptor (`.?AVTESObjectCELL@@`).
const CAST_TARGET_TYPE: u32 = 0x0118_3fb4;
/// `00464f30` (unnamed in the engine map; cdecl, one word): the key of a
/// cell or form.
const KEY_OF_00464F30: u32 = 0x0046_4f30;
/// `00853130` (unnamed in the engine map; `thiscall`, `RET 8`: key, the
/// address of the value): a map lookup (true when found).
const MAP_GET_AT_00853130: u32 = 0x0085_3130;
/// `NiTMapBase<..>::SetAt` (`00844700`, engine map; `thiscall`, `RET 8`: key, value).
const MAP_SET_AT_00844700: u32 = 0x0084_4700;
/// `004839c0` (unnamed in the engine map; cdecl, one word): the form of an id.
const LOOKUP_FORM_004839C0: u32 = 0x0048_39c0;
/// `005861b0` (unnamed in the engine map): a byte of the worldspace that
/// picks the larger cell map.
const WORLD_FLAG_005861B0: u32 = 0x0058_61b0;
/// `0069aee0` (unnamed in the engine map): the `BSSimpleArray` constructor.
const ARRAY_CTOR_0069AEE0: u32 = 0x0069_aee0;
/// `0069af10` (unnamed in the engine map): the `BSSimpleArray` destructor.
const ARRAY_DTOR_0069AF10: u32 = 0x0069_af10;
/// `00719b20` (unnamed in the engine map; `thiscall`, `RET 0xC`: the address
/// of the item, a start index, a comparison function): the item's index, or -1.
const ARRAY_FIND_00719B20: u32 = 0x0071_9b20;
/// The comparison function `0x009a3830` the search is given.
const COMPARE_009A3830: u32 = 0x009a_3830;
/// `007cb2e0` (unnamed in the engine map; `thiscall`, `RET 4`): appends the
/// item whose address it gets.
const ARRAY_ADD_007CB2E0: u32 = 0x007c_b2e0;
/// `00425fd0` (unnamed in the engine map): a byte of the cell that sends
/// the lookup to the interior map entry.
const CELL_IS_INTERIOR_00425FD0: u32 = 0x0042_5fd0;
/// `TESObjectCELL::GetDataX` (`00544c30`, engine map).
const CELL_GET_DATA_X: u32 = 0x0054_4c30;
/// `TESObjectCELL::GetDataY` (`00544c60`, engine map).
const CELL_GET_DATA_Y: u32 = 0x0054_4c60;
/// `00587410` (unnamed in the engine map; cdecl, x and y): the cell key.
const CELL_KEY_00587410: u32 = 0x0058_7410;
/// `TESObjectCELL::GetWorldSpace` (`0054ddd0`, engine map).
const CELL_GET_WORLD_SPACE: u32 = 0x0054_ddd0;
/// `0084e3a0` (unnamed in the engine map): the worldspace's id word.
const FORM_ID_0084E3A0: u32 = 0x0084_e3a0;
/// `00408840` (unnamed in the engine map; cdecl, a float): a float result
/// (ST0) compared with the location limit.
const FLOAT_OF_00408840: u32 = 0x0040_8840;
/// The location limit (a double) the info's float at +0x18 is checked against.
const LOCATION_LIMIT: u32 = 0x0106_ab98;
/// `005b5e40` (unnamed in the engine map; cdecl, a format and its words): prints a warning.
const PRINT_WARNING_005B5E40: u32 = 0x005b_5e40;
/// `"MASTERFILE: Could not find Parent Space with ID %08x for NavmeshInfo %08x"`.
const NO_PARENT_SPACE_FORMAT: u32 = 0x0106_bd60;
/// `"MASTERFILE: NavmeshInfo %08x has bad approx location, ignoring"`.
const BAD_LOCATION_FORMAT: u32 = 0x0106_bd1c;
/// `00559450` (unnamed in the engine map): the word at +0 of its `this`.
const FIRST_OF_00559450: u32 = 0x0055_9450;
/// `NiTMapBase<..>::RemoveAt` (`00405430`, engine map; `thiscall`, `RET 4`).
const MAP_REMOVE_AT_00405430: u32 = 0x0040_5430;
/// `NavMeshInfo::InitDoorPointers` (`006b5490`, engine map).
const INIT_DOOR_POINTERS: u32 = 0x006b_5490;
/// The virtual slot (byte offset) `006b6b20` calls on the map with 1 at the end.
const SLOT_MAP_FINISH: u32 = 0xc8;

/// The shared body of the three `NiTMapBase` constructors: stores
/// `vtable`, the bucket count, clears the entry count at +0xC, allocates
/// the bucket array (`00aa1070`, count * 4 bytes) into +8 and zero-fills it.
fn map_base_ctor(e: &mut Engine, this: Ptr, hash_size: u32, vtable: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, vtable);
    e.mem.set_u32(base + 4, hash_size);
    e.mem.set_u32(base + 0xc, 0);
    let bytes = e.mem.u32(base + 4) << 2;
    let buckets = e.call(BUCKET_ALLOCATE_00AA1070, &args![bytes]).u32();
    e.mem.set_u32(base + 8, buckets);
    let bytes = e.mem.u32(base + 4) << 2;
    let buckets = e.mem.u32(base + 8);
    e.call(FILL_00403D30, &args![buckets, 0u32, bytes]);
    this
}

/// The shared body of the three `NiTMapBase` destructors: stores `vtable`,
/// empties the map (`00438af0`) and frees the bucket array (`00aa10f0`).
fn map_base_dtor(e: &mut Engine, this: Ptr, vtable: u32) {
    e.mem.set_u32(this.addr(), vtable);
    e.call(MAP_REMOVE_ALL_00438AF0, &args![this]);
    let buckets = e.mem.u32(this.addr() + 8);
    e.call(BUCKET_FREE_00AA10F0, &args![buckets]);
}

// Translated from 006b7eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned int>, unsigned int, NavMeshInfo*>`
/// constructor (RTTI of the vtable; unnamed in the engine map; `RET 4`).
pub fn fn_006b7eb0(e: &mut Engine, this: Ptr, hash_size: u32) -> Ptr {
    map_base_ctor(e, this, hash_size, INFO_MAP_BASE_VTABLE)
}

// Translated from 006b8060 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMapBase` constructor under the worldspace map (RTTI of the
/// vtable; unnamed in the engine map; `RET 4`).
pub fn fn_006b8060(e: &mut Engine, this: Ptr, hash_size: u32) -> Ptr {
    map_base_ctor(e, this, hash_size, WORLD_MAP_BASE_VTABLE)
}

// Translated from 006b8160 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMapBase` constructor under the cell map (RTTI of the vtable;
/// unnamed in the engine map; `RET 4`).
pub fn fn_006b8160(e: &mut Engine, this: Ptr, hash_size: u32) -> Ptr {
    map_base_ctor(e, this, hash_size, CELL_MAP_BASE_VTABLE)
}

// Translated from 006b7d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerMap<unsigned int, NavMeshInfo*>` constructor (RTTI of the
/// vtable; unnamed in the engine map; `RET 4`): the base constructor with
/// `hash_size`, then the class's vtable. Returns `this`.
pub fn fn_006b7d90(e: &mut Engine, this: Ptr, hash_size: u32) -> Ptr {
    fn_006b7eb0(e, this, hash_size);
    e.mem.set_u32(this.addr(), INFO_MAP_VTABLE);
    this
}

// Translated from 006b7dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the `NiTMap` of worldspace ids to cell maps (RTTI of the
/// vtable; unnamed in the engine map; `RET 4`). Returns `this`.
pub fn fn_006b7dc0(e: &mut Engine, this: Ptr, hash_size: u32) -> Ptr {
    fn_006b8060(e, this, hash_size);
    e.mem.set_u32(this.addr(), WORLD_MAP_VTABLE);
    this
}

// Translated from 006b7df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the `NiTMap` of cell keys to info arrays (RTTI of the
/// vtable; unnamed in the engine map; `RET 4`). Returns `this`.
pub fn fn_006b7df0(e: &mut Engine, this: Ptr, hash_size: u32) -> Ptr {
    fn_006b8160(e, this, hash_size);
    e.mem.set_u32(this.addr(), CELL_MAP_VTABLE);
    this
}

// Translated from 006b8030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of `NiTMapBase<NiTPointerAllocator<unsigned int>, unsigned int, NavMeshInfo*>`
/// (unnamed in the engine map).
pub fn fn_006b8030(e: &mut Engine, this: Ptr) {
    map_base_dtor(e, this, INFO_MAP_BASE_VTABLE);
}

// Translated from 006b8130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of the `NiTMapBase` under the worldspace map (unnamed
/// in the engine map).
pub fn fn_006b8130(e: &mut Engine, this: Ptr) {
    map_base_dtor(e, this, WORLD_MAP_BASE_VTABLE);
}

// Translated from 006b8250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of the `NiTMapBase` under the cell map (unnamed in the
/// engine map).
pub fn fn_006b8250(e: &mut Engine, this: Ptr) {
    map_base_dtor(e, this, CELL_MAP_BASE_VTABLE);
}

// Translated from 006b7fd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006b7fd0 - its FS:[0] exception frame (the unwind state that runs the base destructor when RemoveAll throws) is not translated
/// `NiTPointerMap<unsigned int, NavMeshInfo*>` destructor body (unnamed in
/// the engine map): stores the class's vtable, empties the map
/// (`00438af0`), then the base destructor `006b8030`.
pub fn fn_006b7fd0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), INFO_MAP_VTABLE);
    e.call(MAP_REMOVE_ALL_00438AF0, &args![this]);
    fn_006b8030(e, this);
}

// Translated from 006b80d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006b80d0 - its FS:[0] exception frame (the unwind state that runs the base destructor when RemoveAll throws) is not translated
/// Destructor body of the worldspace `NiTMap` (unnamed in the engine map):
/// stores the class's vtable, empties the map, then the base destructor
/// `006b8130`.
pub fn fn_006b80d0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), WORLD_MAP_VTABLE);
    e.call(MAP_REMOVE_ALL_00438AF0, &args![this]);
    fn_006b8130(e, this);
}

// Translated from 006b81f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006b81f0 - its FS:[0] exception frame (the unwind state that runs the base destructor when RemoveAll throws) is not translated
/// Destructor body of the cell `NiTMap` (unnamed in the engine map): stores
/// the class's vtable, empties the map, then the base destructor `006b8250`.
pub fn fn_006b81f0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), CELL_MAP_VTABLE);
    e.call(MAP_REMOVE_ALL_00438AF0, &args![this]);
    fn_006b8250(e, this);
}

// Translated from 006b7e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the cell `NiTMap` (engine map name
/// `NiTMap<unsigned_int_BSSimpleArray<NavMeshInfo_P_1024>_P>::_scalar_deleting_destructor_`;
/// `RET 4`): the destructor body `006b81f0`, then `00401030` when bit 0 of
/// `flags` is set. Returns `this`.
pub fn fn_006b7e80(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006b81f0(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 006b5c30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006b5c30 - its FS:[0] exception frame (the unwind states that destroy the form and the maps when a later constructor throws) is not translated
/// `NavMeshInfoMap::NavMeshInfoMap` (Xbox PDB): the `TESForm` constructor,
/// the class's vtable, clears the byte at +0x18, constructs the info map at
/// +0x1C (`006b7d90`, 0x2719 buckets) and the worldspace map at +0x2C
/// (`006b7dc0`, 0x67 buckets), clears the byte at +0x3C and sets the form
/// type to 0x38 (`004f15a0`). Returns `this`.
pub fn nav_mesh_info_map_nav_mesh_info_map(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(TES_FORM_CTOR, &args![this]);
    e.mem.set_u32(base, NAV_MESH_INFO_MAP_VTABLE);
    e.mem.set_u8(base + 0x18, 0);
    fn_006b7d90(e, Ptr::new(base + 0x1c), 0x2719);
    fn_006b7dc0(e, Ptr::new(base + 0x2c), 0x67);
    e.mem.set_u8(base + 0x3c, 0);
    e.call(SET_FORM_TYPE_004F15A0, &args![this, 0x38u32]);
    this
}

// Translated from 006b77b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `__RTDynamicCast` of what `00690830` returns
/// for `this` from `TESForm` to `TESObjectCELL` (the type descriptors
/// `01183028` and `01183fb4`). Returns the cast result.
pub fn fn_006b77b0(e: &mut Engine, this: Ptr) -> u32 {
    let object = e.call(INFO_PARENT_00690830, &args![this]).u32();
    e.call(
        RT_DYNAMIC_CAST,
        &args![object, 0u32, CAST_SOURCE_TYPE, CAST_TARGET_TYPE, 0u32],
    )
    .u32()
}

// Translated from 006b7b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word): the key (`00464f30`) of the
/// cast object `006b77b0` finds for `info`, or, when that is null, of the
/// word at `info` + 0xC.
pub fn fn_006b7b30(e: &mut Engine, info: Ptr) -> u32 {
    let object = fn_006b77b0(e, info);
    if object != 0 {
        e.call(KEY_OF_00464F30, &args![object]).u32()
    } else {
        let word = e.mem.u32(info.addr() + 0xc);
        e.call(KEY_OF_00464F30, &args![word]).u32()
    }
}

/// `00853130(map, key, out)` with `out` a stack slot: the found flag and
/// the value.
fn map_get(e: &mut Engine, map: u32, key: u32, slot: u32) -> bool {
    e.call(MAP_GET_AT_00853130, &args![map, key, slot]).u8() != 0
}

// Translated from 006b7850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`): the info array of the
/// cell `key` in `world`: the worldspace map at +0x2C looked up with the
/// worldspace's id (`0084e3a0`), then that map looked up with
/// `00464f30(key)`. Null when either lookup fails.
pub fn fn_006b7850(e: &mut Engine, this: Ptr, world: Ptr, key: u32) -> u32 {
    e.with_stack(8, |e, locals| {
        let inner_slot = locals.addr();
        let array_slot = locals.addr() + 4;
        let id = e.call(FORM_ID_0084E3A0, &args![world]).u32();
        if !map_get(e, this.addr() + 0x2c, id, inner_slot) {
            return 0;
        }
        let cell_key = e.call(KEY_OF_00464F30, &args![key]).u32();
        let inner = e.mem.u32(inner_slot);
        if !map_get(e, inner, cell_key, array_slot) {
            return 0;
        }
        e.mem.u32(array_slot)
    })
}

// Translated from 006b78b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): the info array of an
/// interior `cell`: the worldspace map's entry 0, then that map looked up
/// with `00464f30(cell)`. Null when either lookup fails.
pub fn fn_006b78b0(e: &mut Engine, this: Ptr, cell: Ptr) -> u32 {
    e.with_stack(8, |e, locals| {
        let inner_slot = locals.addr();
        let array_slot = locals.addr() + 4;
        if !map_get(e, this.addr() + 0x2c, 0, inner_slot) {
            return 0;
        }
        let cell_key = e.call(KEY_OF_00464F30, &args![cell]).u32();
        let inner = e.mem.u32(inner_slot);
        if !map_get(e, inner, cell_key, array_slot) {
            return 0;
        }
        e.mem.u32(array_slot)
    })
}

// Translated from 006b77e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshInfoMap::FindNavMeshInfoForCell` (Xbox PDB; `RET 4`): null for
/// a null cell; for a cell whose byte `00425fd0` is set, `006b78b0`;
/// otherwise `006b7850` with the cell's worldspace and the key
/// `00587410(x, y)` of its grid position (y read first).
pub fn nav_mesh_info_map_find_nav_mesh_info_for_cell(e: &mut Engine, this: Ptr, cell: Ptr) -> u32 {
    if cell.is_null() {
        return 0;
    }
    if e.call(CELL_IS_INTERIOR_00425FD0, &args![cell]).u8() != 0 {
        return fn_006b78b0(e, this, cell);
    }
    let y = e.call(CELL_GET_DATA_Y, &args![cell]).u32();
    let x = e.call(CELL_GET_DATA_X, &args![cell]).u32();
    let key = e.call(CELL_KEY_00587410, &args![x, y]).u32();
    let world = e.call(CELL_GET_WORLD_SPACE, &args![cell]).ptr();
    fn_006b7850(e, this, world, key)
}

// Translated from 006b7980 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006b7980 - its FS:[0] exception frame (the unwind states that free the new map or array block when its constructor throws) is not translated
/// `NavMeshInfoMap::AddInfoToCellKeyMap` (Xbox PDB; `RET 4`): files `info`
/// under its worldspace id (0 when `006b77b0` finds the cast object, else
/// the word at `info` + 4) and its cell key (`006b7b30`). A missing
/// worldspace entry gets a new cell map (0x10 bytes, `006b7df0`) of 0x67
/// buckets, or 0x3F1 for id 0 or a worldspace form whose byte `005861b0`
/// is clear (null when the allocation fails), stored with `SetAt`. A
/// missing cell entry gets a new array (0x10 bytes, `0069aee0`). The info
/// is appended (`007cb2e0`) unless the array already holds it (`00719b20`
/// with the comparison `009a3830` from index 0 is not -1). Both calls get
/// the address of the argument's own stack slot.
pub fn nav_mesh_info_map_add_info_to_cell_key_map(e: &mut Engine, this: Ptr, info: u32) {
    // The argument's slot, the cell map (EBP-0x1C) and the array (EBP-0x10).
    e.with_stack(0xc, |e, locals| {
        let info_slot = locals.addr();
        let inner_slot = locals.addr() + 4;
        let array_slot = locals.addr() + 8;
        e.mem.set_u32(info_slot, info);
        let key = fn_006b7b30(e, Ptr::new(info));
        let world = if fn_006b77b0(e, Ptr::new(info)) != 0 {
            0
        } else {
            e.mem.u32(info.wrapping_add(4))
        };
        if !map_get(e, this.addr() + 0x2c, world, inner_slot) {
            let mut size = 0x67;
            if world == 0 {
                size = 0x3f1;
            } else {
                let form = e.call(LOOKUP_FORM_004839C0, &args![world]).u32();
                if form != 0 && e.call(WORLD_FLAG_005861B0, &args![form]).u8() == 0 {
                    size = 0x3f1;
                }
            }
            let block = e.call(ALLOCATE, &args![0x10u32]).u32();
            let inner = if block != 0 {
                fn_006b7df0(e, Ptr::new(block), size).addr()
            } else {
                0
            };
            e.mem.set_u32(inner_slot, inner);
            e.call(
                MAP_SET_AT_00844700,
                &args![this.addr() + 0x2c, world, inner],
            );
        }
        let inner = e.mem.u32(inner_slot);
        if !map_get(e, inner, key, array_slot) {
            let block = e.call(ALLOCATE, &args![0x10u32]).u32();
            let array = if block != 0 {
                e.call(ARRAY_CTOR_0069AEE0, &args![block]).u32()
            } else {
                0
            };
            e.mem.set_u32(array_slot, array);
            let inner = e.mem.u32(inner_slot);
            e.call(MAP_SET_AT_00844700, &args![inner, key, array]);
        }
        let array = e.mem.u32(array_slot);
        let index = e
            .call(
                ARRAY_FIND_00719B20,
                &args![array, info_slot, 0u32, COMPARE_009A3830],
            )
            .u32();
        if index == 0xffff_ffff {
            let array = e.mem.u32(array_slot);
            e.call(ARRAY_ADD_007CB2E0, &args![array, info_slot]);
        }
    });
}

// Translated from 006b7cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a check over the worldspace map at +0x2C.
/// For every info array (of every cell map) with more than one entry, it
/// counts the entries whose info word `006286d0` (on the element
/// `006a7ad0` gives) is non-zero; the result is false as soon as one array
/// counts fewer than its size minus one, else true.
pub fn fn_006b7cb0(e: &mut Engine, this: Ptr) -> bool {
    let world_map = Ptr::new(this.addr() + 0x2c);
    // it (EBP-4), key (EBP-0x10), value (EBP-0x14), it2 (EBP-0xC), key2
    // (EBP-0x1C), value2 (EBP-0x18).
    e.with_stack(0x18, |e, locals| {
        let base = locals.addr();
        let (it, key, value) = (base, base + 4, base + 8);
        let (it2, key2, value2) = (base + 0xc, base + 0x10, base + 0x14);
        let mut result = true;
        let first = e.call(MAP_FIRST_004B9BA0, &args![world_map]).u32();
        e.mem.set_u32(it, first);
        while e.mem.u32(it) != 0 {
            fn_006b7f20(e, world_map, Ptr::new(it), Ptr::new(key), Ptr::new(value));
            let inner = e.mem.u32(value);
            let first = e.call(MAP_FIRST_004B9BA0, &args![inner]).u32();
            e.mem.set_u32(it2, first);
            while e.mem.u32(it2) != 0 {
                let inner = e.mem.u32(value);
                fn_006b7f20(
                    e,
                    Ptr::new(inner),
                    Ptr::new(it2),
                    Ptr::new(key2),
                    Ptr::new(value2),
                );
                let array = e.mem.u32(value2);
                if e.call(ARRAY_SIZE_0044DDC0, &args![array]).u32() > 1 {
                    let mut counted: u32 = 0;
                    let mut index: u32 = 0;
                    loop {
                        let array = e.mem.u32(value2);
                        if index >= e.call(ARRAY_SIZE_0044DDC0, &args![array]).u32() {
                            break;
                        }
                        let element = e.call(ARRAY_AT_006A7AD0, &args![array, index]).u32();
                        let info = e.mem.u32(element);
                        if e.call(INFO_VALUE_006286D0, &args![info]).u32() != 0 {
                            counted = counted.wrapping_add(1);
                        }
                        index = index.wrapping_add(1);
                    }
                    let array = e.mem.u32(value2);
                    let size = e.call(ARRAY_SIZE_0044DDC0, &args![array]).u32();
                    if counted < size.wrapping_sub(1) {
                        result = false;
                    }
                }
            }
        }
        result
    })
}

// Translated from 006b68b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): runs the `TESForm`
/// step `00484730` with `flag`; when `flag` is clear, runs `004534f0(info, 0)`
/// on every info of the map at +0x1C.
pub fn fn_006b68b0(e: &mut Engine, this: Ptr, flag: u8) {
    e.call(FORM_STEP_00484730, &args![this, u32::from(flag)]);
    if flag != 0 {
        return;
    }
    let info_map = Ptr::new(this.addr() + 0x1c);
    // it (EBP-4), key (EBP-0xC), value (EBP-8).
    e.with_stack(0xc, |e, locals| {
        let (it, key, value) = (locals.addr(), locals.addr() + 4, locals.addr() + 8);
        let first = e.call(MAP_FIRST_004B9BA0, &args![info_map]).u32();
        e.mem.set_u32(it, first);
        while e.mem.u32(it) != 0 {
            fn_006b7f20(e, info_map, Ptr::new(it), Ptr::new(key), Ptr::new(value));
            let info = e.mem.u32(value);
            e.call(INFO_STEP_004534F0, &args![info, 0u32]);
        }
    });
}

// Translated from 006b6b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: empties the navmesh info map. Every info of
/// the map at +0x1C is deleted (`006b6390(info, 1)`, skipped for null),
/// the map emptied (`00438af0`); then for every cell map of the worldspace
/// map at +0x2C, each of its arrays and then the cell map itself are
/// deleted through their vtable slot 0 with 1 (null skipped), and the
/// worldspace map is emptied. Last, the form's virtual slot +0xC8 runs
/// with 1.
pub fn fn_006b6b20(e: &mut Engine, this: Ptr) {
    let info_map = Ptr::new(this.addr() + 0x1c);
    let world_map = Ptr::new(this.addr() + 0x2c);
    // it (EBP-4), key (EBP-0xC), value (EBP-8); the nested walk's it2
    // (EBP-0x14), key2 (EBP-0x20), value2 (EBP-0x1C).
    e.with_stack(0x18, |e, locals| {
        let base = locals.addr();
        let (it, key, value) = (base, base + 4, base + 8);
        let (it2, key2, value2) = (base + 0xc, base + 0x10, base + 0x14);
        let first = e.call(MAP_FIRST_004B9BA0, &args![info_map]).u32();
        e.mem.set_u32(it, first);
        while e.mem.u32(it) != 0 {
            fn_006b7f20(e, info_map, Ptr::new(it), Ptr::new(key), Ptr::new(value));
            let info = e.mem.u32(value);
            if info != 0 {
                e.call(NAV_MESH_INFO_DELETE, &args![info, 1u32]);
            }
        }
        e.call(MAP_REMOVE_ALL_00438AF0, &args![info_map]);
        let first = e.call(MAP_FIRST_004B9BA0, &args![world_map]).u32();
        e.mem.set_u32(it, first);
        while e.mem.u32(it) != 0 {
            fn_006b7f20(e, world_map, Ptr::new(it), Ptr::new(key), Ptr::new(value));
            let inner = e.mem.u32(value);
            let first = e.call(MAP_FIRST_004B9BA0, &args![inner]).u32();
            e.mem.set_u32(it2, first);
            while e.mem.u32(it2) != 0 {
                let inner = e.mem.u32(value);
                fn_006b7f20(
                    e,
                    Ptr::new(inner),
                    Ptr::new(it2),
                    Ptr::new(key2),
                    Ptr::new(value2),
                );
                let array = e.mem.u32(value2);
                if array != 0 {
                    e.vcall(array, 0, &args![1u32]);
                }
            }
            let inner = e.mem.u32(value);
            if inner != 0 {
                e.vcall(inner, 0, &args![1u32]);
            }
        }
        e.call(MAP_REMOVE_ALL_00438AF0, &args![world_map]);
    });
    e.vcall(this.addr(), SLOT_MAP_FINISH, &args![1u32]);
}

// Translated from 006b63c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006b63c0 - its FS:[0] exception frame (the unwind state that destroys the local array when a call throws) is not translated
/// `NavMeshInfoMap::InitItem` (Xbox PDB): with a local array (`0069aee0`),
/// walks the info map at +0x1C. For each info: its parent space id (+4)
/// is looked up (`004839c0`); a missing one prints the parent-space
/// warning (id, then the map key) and marks the info bad; the float at
/// +0x18 through `00408840` above the location limit prints the
/// bad-location warning (key) and marks it bad too. A bad info goes to the
/// local array (`007cb2e0` with the address of its slot); a good one gets
/// the parent at +0x1C and is filed by `AddInfoToCellKeyMap`. While the
/// byte at +0x3C is clear, each info also runs `InitDoorPointers`. Then
/// every bad info is removed from the map (`RemoveAt` with its first word,
/// `00559450`) and deleted (`006b6390(info, 1)`, null skipped), the byte at
/// +0x3C is set, the check `006b7cb0` runs and the local array is
/// destroyed (`0069af10`).
pub fn nav_mesh_info_map_init_item(e: &mut Engine, this: Ptr) {
    let info_map = Ptr::new(this.addr() + 0x1c);
    // The local array (EBP-0x20, 0x10 bytes), it (EBP-0x10), key
    // (EBP-0x30), info (EBP-0x2C).
    e.with_stack(0x1c, |e, locals| {
        let array = locals.addr();
        let (it, key, info_slot) = (array + 0x10, array + 0x14, array + 0x18);
        e.call(ARRAY_CTOR_0069AEE0, &args![array]);
        let first = e.call(MAP_FIRST_004B9BA0, &args![info_map]).u32();
        e.mem.set_u32(it, first);
        while e.mem.u32(it) != 0 {
            fn_006b7f20(
                e,
                info_map,
                Ptr::new(it),
                Ptr::new(key),
                Ptr::new(info_slot),
            );
            let mut bad = false;
            let info = e.mem.u32(info_slot);
            let parent_id = e.mem.u32(info.wrapping_add(4));
            let parent = e.call(LOOKUP_FORM_004839C0, &args![parent_id]).u32();
            if parent == 0 {
                let map_key = e.mem.u32(key);
                let parent_id = e.mem.u32(info.wrapping_add(4));
                e.call(
                    PRINT_WARNING_005B5E40,
                    &args![NO_PARENT_SPACE_FORMAT, parent_id, map_key],
                );
                bad = true;
            }
            let info = e.mem.u32(info_slot);
            let location = x87_quiet_bits(e.mem.u32(info.wrapping_add(0x18)));
            let value = e.call(FLOAT_OF_00408840, &args![location]).f64();
            if value > e.global::<f64>(LOCATION_LIMIT) {
                let map_key = e.mem.u32(key);
                e.call(PRINT_WARNING_005B5E40, &args![BAD_LOCATION_FORMAT, map_key]);
                bad = true;
            }
            if bad {
                e.call(ARRAY_ADD_007CB2E0, &args![array, info_slot]);
            } else {
                let info = e.mem.u32(info_slot);
                e.mem.set_u32(info.wrapping_add(0x1c), parent);
                nav_mesh_info_map_add_info_to_cell_key_map(e, this, info);
            }
            if e.mem.u8(this.addr() + 0x3c) == 0 {
                let info = e.mem.u32(info_slot);
                e.call(INIT_DOOR_POINTERS, &args![info]);
            }
        }
        let mut index: u32 = 0;
        while index < e.call(ARRAY_SIZE_0044DDC0, &args![array]).u32() {
            let element = e.call(ARRAY_AT_00877A30, &args![array, index]).u32();
            let info = e.mem.u32(element);
            let info_key = e.call(FIRST_OF_00559450, &args![info]).u32();
            e.call(MAP_REMOVE_AT_00405430, &args![info_map, info_key]);
            let element = e.call(ARRAY_AT_00877A30, &args![array, index]).u32();
            let info = e.mem.u32(element);
            if info != 0 {
                e.call(NAV_MESH_INFO_DELETE, &args![info, 1u32]);
            }
            index = index.wrapping_add(1);
        }
        e.mem.set_u8(this.addr() + 0x3c, 1);
        fn_006b7cb0(e, this);
        e.call(ARRAY_DTOR_0069AF10, &args![array]);
    });
}

/// `00401170` (`ECX` only): the byte at +4 of a form (its form type), zero-extended.
const FORM_TYPE_00401170: u32 = 0x0040_1170;
/// `00484020` (`thiscall`, `RET 4`, `tesform.cpp`): the comparison `006b6910`
/// falls back on.
const FORM_COMPARE_00484020: u32 = 0x0048_4020;
/// `TESWorldSpace::GetCellFromKey` (`00587630`, engine map; `thiscall`,
/// `RET 4`: the cell key).
const WORLD_SPACE_GET_CELL_FROM_KEY: u32 = 0x0058_7630;
/// `00690800` (`navmesh.cpp`, `ECX` only): `__RTDynamicCast` of what
/// `00690830` returns to the type descriptor `01183fd0`.
const INFO_WORLD_SPACE_00690800: u32 = 0x0069_0800;
/// `BSSimpleArray<..>::SetSize` (`006f2290`, engine map; `thiscall`, `RET 8`).
const ARRAY_SET_SIZE_006F2290: u32 = 0x006f_2290;
/// `BSSimpleArray<..>::SetReservedSize` (`0084b5c0`, engine map; `thiscall`, `RET 4`).
const ARRAY_SET_RESERVED_SIZE_0084B5C0: u32 = 0x0084_b5c0;
/// `0069b080` (`ECX` only): whether the word at +0xC of a map (its entry
/// count) is zero.
const MAP_IS_EMPTY_0069B080: u32 = 0x0069_b080;
/// `BSSimpleArray<..>::Remove` (`006bf8f0`, engine map; `thiscall`, `RET 8`:
/// the index, a byte).
const ARRAY_REMOVE_006BF8F0: u32 = 0x006b_f8f0;
/// `0076b610` (`ECX` only): whether an array's size (+8) is zero.
const ARRAY_IS_EMPTY_0076B610: u32 = 0x0076_b610;
/// `0042fb00` (`ECX` only): constructs an empty `NiPointer` (`extradatalist.cpp`).
const NI_POINTER_CONSTRUCT_0042FB00: u32 = 0x0042_fb00;
/// `0042fa40` (`ECX` only): destroys an `NiPointer`.
const NI_POINTER_DESTRUCT_0042FA40: u32 = 0x0042_fa40;
/// `00690860` (`navmesh.cpp`; `thiscall`, `RET 4`: the address of an
/// `NiPointer`): stores the info's `pNavMesh` (+0x54) into the pointer
/// (`00690890`) and returns whether it is set.
const INFO_NAV_MESH_00690860: u32 = 0x0069_0860;
/// `TESForm::GetFile` (`00484e60`, engine map; `thiscall`, `RET 4`: the index).
const FORM_GET_FILE: u32 = 0x0048_4e60;
/// `TESFile::GetMaster` (`00471c20`, engine map; `ECX` only): bit 0 of the word at +0x3E8.
const FILE_GET_MASTER: u32 = 0x0047_1c20;
/// `00460340` (`ECX` only): whether bit 1 of the word at +8 of a form is set.
const FORM_FLAG_2_00460340: u32 = 0x0046_0340;
/// `00440d80` (`ECX` only): whether bit 5 (0x20) of the word at +8 of a form is set.
const FORM_FLAG_20_00440D80: u32 = 0x0044_0d80;
/// `0047c850` (`ECX` only): returns 0 in AL (`tesactorbasedata.cpp`).
const ALWAYS_ZERO_0047C850: u32 = 0x0047_c850;
/// `006986d0` (`navmesh_util.cpp`; `ECX` only): the step `006b6580` runs on
/// each navmesh whose form flag 2 is set and flag 0x20 clear.
const NAV_MESH_STEP_006986D0: u32 = 0x0069_86d0;
/// The global holding the object whose list `006b6580` counts form type 0x43 in.
const FORM_LIST_011C54C4: u32 = 0x011c_54c4;
/// `TESForm::StartForm` (`004855a0`, engine map; `ECX` only).
const FORM_START_FORM: u32 = 0x0048_55a0;
/// `00485910` (`tesform.cpp`; cdecl: the chunk id, a word): adds a
/// four-byte chunk.
const FORM_ADD_WORD_DATA_00485910: u32 = 0x0048_5910;
/// `TESForm::__AddChunkData` (`00485990`, engine map; cdecl: the chunk id,
/// the data, its size).
const FORM_ADD_CHUNK_DATA: u32 = 0x0048_5990;
/// `TESForm::CloseForm` (`00485680`, engine map; `ECX` only).
const FORM_CLOSE_FORM: u32 = 0x0048_5680;
/// The chunk ids `006b6740` writes: the bytes `NVER`, `NVMI`, `NVCI`.
const CHUNK_NVER: u32 = 0x5245_564e;
const CHUNK_NVMI: u32 = 0x494d_564e;
const CHUNK_NVCI: u32 = 0x4943_564e;
/// `006b4880` (`ECX` only): the size of a `NavMeshInfo`'s `NVMI` data.
const INFO_DATA_SIZE_006B4880: u32 = 0x006b_4880;
/// `006b48f0` (`thiscall`, `RET 4`: the buffer): writes a `NavMeshInfo`'s `NVMI` data.
const INFO_WRITE_DATA_006B48F0: u32 = 0x006b_48f0;
/// `006b4e10` (`ECX` only): the size of a `NavMeshInfo`'s connection data.
const INFO_CONNECTIONS_SIZE_006B4E10: u32 = 0x006b_4e10;
/// `006b4e80` (`thiscall`, `RET 4`: the buffer): writes a `NavMeshInfo`'s
/// connection data.
const INFO_WRITE_CONNECTIONS_006B4E80: u32 = 0x006b_4e80;
/// `00401500` (no arguments): the byte at `011c54ba` that asks for
/// byte-swapped words.
const SWAP_BYTES_00401500: u32 = 0x0040_1500;
/// `00401080` (cdecl: the address of a word, a byte): reverses the word's bytes.
const BYTE_SWAP_00401080: u32 = 0x0040_1080;
/// The virtual slot (byte offset) of a cell that gives the `%s` of the
/// `CheckMap` warnings.
const SLOT_CELL_NAME: u32 = 0x130;
/// An empty string (`""`, the byte at `01011584` is 0).
const EMPTY_STRING: u32 = 0x0101_1584;
/// `0069dfb0` (`ECX` only): whether the word at +0x54 of a `NavMeshInfo`
/// (`pNavMesh`) is set.
const INFO_HAS_NAV_MESH_0069DFB0: u32 = 0x0069_dfb0;
/// `NavMesh::FixPortalsFromNavMeshDeletion` (`006953b0`, engine map; cdecl, one word).
const FIX_PORTALS_FROM_NAV_MESH_DELETION: u32 = 0x0069_53b0;
/// `00693d70` (`navmesh_loadsave.cpp`; `thiscall`, `RET 4`): the info of an id, or null.
const INFO_OF_ID_00693D70: u32 = 0x0069_3d70;
/// `00587520` (cdecl: a cell key, the addresses of two halfwords): splits the
/// key into its high (x) and low (y) halfwords.
const SPLIT_CELL_KEY_00587520: u32 = 0x0058_7520;
/// `00694de0` / `00694e10` (`ECX` only): construct and destroy the
/// triangle array `CheckMap` passes to `FindPortalTriangles`.
const TRIANGLE_ARRAY_CONSTRUCT_00694DE0: u32 = 0x0069_4de0;
const TRIANGLE_ARRAY_DESTRUCT_00694E10: u32 = 0x0069_4e10;
/// `NavMesh::FindPortalTriangles` (`00697590`, engine map; `thiscall`,
/// `RET 8`: the neighbouring info, the triangle array).
const FIND_PORTAL_TRIANGLES: u32 = 0x0069_7590;
/// `00695370` (`ECX` only): a navmesh's triangle count.
const TRIANGLE_COUNT_00695370: u32 = 0x0069_5370;
/// `00693d30` (`thiscall`, `RET 4`: a halfword index): a navmesh triangle's address.
const TRIANGLE_AT_00693D30: u32 = 0x0069_3d30;
/// `0069e090` (`ECX` only): whether an edge link (a word, then a halfword)
/// has a non-null word and a halfword other than -1.
const EDGE_LINK_SET_0069E090: u32 = 0x0069_e090;
/// `006b5a90` (`ECX` only): a navmesh's door count.
const DOOR_COUNT_006B5A90: u32 = 0x006b_5a90;
/// `0068f6b0` (`thiscall`, `RET 4`: a halfword index): the address of a
/// navmesh's door entry.
const DOOR_AT_0068F6B0: u32 = 0x0068_f6b0;
/// `006b5560` (`thiscall`, `RET 4`: the navmesh): rebuilds a `NavMeshInfo`
/// from its navmesh.
const INFO_REBUILD_006B5560: u32 = 0x006b_5560;
/// The `CheckMap` warnings (`"PATHFINDING: NavMeshInfo %08x ..."`): no
/// parent space, no matching navmesh, a navmesh marked deleted (the three
/// with `in cell %s`), a portal with no navmesh, no triangle portal to a
/// neighbour, a navmesh portal the info lacks, a door portal the info lacks,
/// and a door portal the navmesh lacks.
const NO_PARENT_SPACE_WARNING: u32 = 0x0106_c0d8;
const NO_NAV_MESH_WARNING: u32 = 0x0106_c078;
const DELETED_NAV_MESH_WARNING: u32 = 0x0106_c008;
const PORTAL_WITHOUT_NAV_MESH_WARNING: u32 = 0x0106_bfa8;
const NO_TRIANGLE_PORTAL_WARNING: u32 = 0x0106_bf18;
const MISSING_PORTAL_WARNING: u32 = 0x0106_be98;
const MISSING_DOOR_PORTAL_WARNING: u32 = 0x0106_be20;
const EXTRA_DOOR_PORTAL_WARNING: u32 = 0x0106_bdb0;
/// `006911c0` (`ECX` only): the product of the first words of two of a
/// navmesh's members (`00559450` twice): the count `006dfba0` builds.
const NAV_MESH_COUNT_006911C0: u32 = 0x0069_11c0;
/// `008454f0` (`thiscall`, `RET 4`): empties an array (with 1, also frees the buffer).
const ARRAY_RELEASE_008454F0: u32 = 0x0084_54f0;
/// `0068efb0` (`ECX` only): the word at +0x104 of a navmesh.
const NAV_MESH_VALUE_0068EFB0: u32 = 0x0068_efb0;
/// `006e1070` (`pathinglosgridmap.cpp`, `ECX` only): constructs the array at +4
/// of the 0x1C-byte item.
const ITEM_ARRAY_CONSTRUCT_006E1070: u32 = 0x006e_1070;
/// `006df200` (`thiscall`, `RET 8`): clears the item's byte at +0 and stores
/// its two words at +0x14 and +0x18.
const ITEM_INIT_006DF200: u32 = 0x006d_f200;

// Translated from 006b6910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`NavMeshInfoMap`'s virtual slot +0x3C;
/// `thiscall`, `RET 4`): 1 when `form`'s type (`00401170`) is 0x43, else what
/// `00484020(this, form)` returns (all of EAX).
pub fn fn_006b6910(e: &mut Engine, this: Ptr, form: Ptr) -> u32 {
    // `MOV AL,1` over EAX = 0x43's zero upper bytes: EAX is 1.
    if e.call(FORM_TYPE_00401170, &args![form]).u32() == 0x43 {
        return 1;
    }
    e.call(FORM_COMPARE_00484020, &args![this, form]).u32()
}

// Translated from 006b6940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshInfoMap::AddNavMeshInfo` (engine map; `RET 4`): files `info` in the
/// info map at +0x1C under its id (`00559450`, `SetAt`), then in the cell key
/// map (`AddInfoToCellKeyMap`), and runs the form's virtual slot +0xC8 with 1.
pub fn nav_mesh_info_map_add_nav_mesh_info(e: &mut Engine, this: Ptr, info: u32) {
    let id = e.call(FIRST_OF_00559450, &args![info]).u32();
    e.call(MAP_SET_AT_00844700, &args![this.addr() + 0x1c, id, info]);
    nav_mesh_info_map_add_info_to_cell_key_map(e, this, info);
    e.vcall(this.addr(), SLOT_MAP_FINISH, &args![1u32]);
}

// Translated from 006b6990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): removes the info of id `key` from
/// the info map at +0x1C when it is there (`00853130`, `RemoveAt`), takes it
/// out of the cell key maps (`006b7b70`) unless the worldspace map at +0x2C
/// is empty (`0069b080`), and runs the form's virtual slot +0xC8 with 1.
/// The info itself is not deleted.
pub fn fn_006b6990(e: &mut Engine, this: Ptr, key: u32) {
    let info_map = this.addr() + 0x1c;
    e.with_stack(4, |e, info_slot| {
        if !map_get(e, info_map, key, info_slot.addr()) {
            return;
        }
        e.call(MAP_REMOVE_AT_00405430, &args![info_map, key]);
        let empty = e
            .call(MAP_IS_EMPTY_0069B080, &args![this.addr() + 0x2c])
            .u8();
        if empty == 0 {
            let info = e.mem.u32(info_slot.addr());
            fn_006b7b70(e, this, info);
        }
        e.vcall(this.addr(), SLOT_MAP_FINISH, &args![1u32]);
    });
}

// Translated from 006b6a00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006b6a00 - its FS:[0] exception frame (the unwind state that destroys the local NiPointer) is not translated
/// `NavMeshInfoMap::DeleteNavMeshInfo` (engine map; `RET 4`): when the info
/// map at +0x1C holds the id `key`, the info leaves the cell key maps
/// (`006b7b70`); with a local `NiPointer` (`0042fb00`) filled with its navmesh
/// (`00690860`), the flag is set when that navmesh's first file
/// (`TESForm::GetFile(0)`) exists and is a master (`TESFile::GetMaster`).
/// The info is deleted (`006b6390(info, 1)`, null skipped), removed from the
/// map (`RemoveAt`), the form's virtual slot +0xC8 runs with 1 when the flag
/// is set, and the pointer is destroyed (`0042fa40`).
pub fn nav_mesh_info_map_delete_nav_mesh_info(e: &mut Engine, this: Ptr, key: u32) {
    let info_map = this.addr() + 0x1c;
    // The info (EBP-0x10) and the NiPointer (EBP-0x18).
    e.with_stack(8, |e, locals| {
        let info_slot = locals.addr();
        let mesh = locals.addr() + 4;
        if !map_get(e, info_map, key, info_slot) {
            return;
        }
        let info = e.mem.u32(info_slot);
        fn_006b7b70(e, this, info);
        let mut master = false;
        e.call(NI_POINTER_CONSTRUCT_0042FB00, &args![mesh]);
        let info = e.mem.u32(info_slot);
        if e.call(INFO_NAV_MESH_00690860, &args![info, mesh]).u8() != 0 {
            let nav_mesh = e.call(FIRST_OF_00559450, &args![mesh]).u32();
            let file = e.call(FORM_GET_FILE, &args![nav_mesh, 0u32]).u32();
            if file != 0 && e.call(FILE_GET_MASTER, &args![file]).u8() != 0 {
                master = true;
            }
        }
        let info = e.mem.u32(info_slot);
        if info != 0 {
            e.call(NAV_MESH_INFO_DELETE, &args![info, 1u32]);
        }
        e.call(MAP_REMOVE_AT_00405430, &args![info_map, key]);
        if master {
            e.vcall(this.addr(), SLOT_MAP_FINISH, &args![1u32]);
        }
        e.call(NI_POINTER_DESTRUCT_0042FA40, &args![mesh]);
    });
}

// Translated from 006b7770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): the cell of a `NavMeshInfo`: with
/// a worldspace (`00690800`), `TESWorldSpace::GetCellFromKey` with the
/// info's `iCellKey` (+0xC, Xbox PDB); otherwise the cell cast `006b77b0`.
pub fn fn_006b7770(e: &mut Engine, info: Ptr) -> u32 {
    let world = e.call(INFO_WORLD_SPACE_00690800, &args![info]).u32();
    if world != 0 {
        // NavMeshInfo::iCellKey (Xbox PDB) +0xC.
        let cell_key = e.mem.u32(info.addr().wrapping_add(0xc));
        e.call(WORLD_SPACE_GET_CELL_FROM_KEY, &args![world, cell_key])
            .u32()
    } else {
        fn_006b77b0(e, info)
    }
}

// Translated from 006b7910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): fills `out` with every info of the
/// info map at +0x1C: `SetSize(0, 0)`, `SetReservedSize(0x800)`, then each
/// value appended (`007cb2e0` with the address of its slot).
pub fn fn_006b7910(e: &mut Engine, this: Ptr, out: Ptr) {
    let info_map = Ptr::new(this.addr() + 0x1c);
    e.call(ARRAY_SET_SIZE_006F2290, &args![out, 0u32, 0u32]);
    e.call(ARRAY_SET_RESERVED_SIZE_0084B5C0, &args![out, 0x800u32]);
    // it (EBP-4), value (EBP-8), key (EBP-0xC).
    e.with_stack(0xc, |e, locals| {
        let (key, value, it) = (locals.addr(), locals.addr() + 4, locals.addr() + 8);
        let first = e.call(MAP_FIRST_004B9BA0, &args![info_map]).u32();
        e.mem.set_u32(it, first);
        while e.mem.u32(it) != 0 {
            fn_006b7f20(e, info_map, Ptr::new(it), Ptr::new(key), Ptr::new(value));
            e.call(ARRAY_ADD_007CB2E0, &args![out, value]);
        }
    });
}

// Translated from 006b7b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`), the reverse of
/// `AddInfoToCellKeyMap`: finds `info`'s array under its worldspace id (0
/// when `006b77b0` finds the cast object, else the word at `info` + 4) and
/// its cell key (`006b7b30`); nothing happens when a lookup fails or the
/// array lacks the info (`00719b20` with `009a3830` from 0, given the
/// address of the argument's own slot). Otherwise the entry is removed
/// (`006bf8f0(index, 1)`); an array left empty (`0076b610`) is removed from
/// its cell map and deleted (vtable slot 0 with 1, null skipped), and a
/// cell map left empty (`0069b080`) likewise from the worldspace map.
pub fn fn_006b7b70(e: &mut Engine, this: Ptr, info: u32) {
    // The argument's slot (EBP+8), the array (EBP-4) and the cell map (EBP-0x14).
    e.with_stack(0xc, |e, locals| {
        let info_slot = locals.addr();
        let array_slot = locals.addr() + 4;
        let inner_slot = locals.addr() + 8;
        e.mem.set_u32(info_slot, info);
        let key = fn_006b7b30(e, Ptr::new(info));
        let world = if fn_006b77b0(e, Ptr::new(info)) != 0 {
            0
        } else {
            e.mem.u32(info.wrapping_add(4))
        };
        if !map_get(e, this.addr() + 0x2c, world, inner_slot) {
            return;
        }
        let inner = e.mem.u32(inner_slot);
        if !map_get(e, inner, key, array_slot) {
            return;
        }
        let array = e.mem.u32(array_slot);
        let index = e
            .call(
                ARRAY_FIND_00719B20,
                &args![array, info_slot, 0u32, COMPARE_009A3830],
            )
            .u32();
        if index == 0xffff_ffff {
            return;
        }
        e.call(ARRAY_REMOVE_006BF8F0, &args![array, index, 1u32]);
        if e.call(ARRAY_IS_EMPTY_0076B610, &args![array]).u8() == 0 {
            return;
        }
        let inner = e.mem.u32(inner_slot);
        e.call(MAP_REMOVE_AT_00405430, &args![inner, key]);
        let array = e.mem.u32(array_slot);
        if array != 0 {
            e.vcall(array, 0, &args![1u32]);
        }
        let inner = e.mem.u32(inner_slot);
        if e.call(MAP_IS_EMPTY_0069B080, &args![inner]).u8() == 0 {
            return;
        }
        e.call(MAP_REMOVE_AT_00405430, &args![this.addr() + 0x2c, world]);
        let inner = e.mem.u32(inner_slot);
        if inner != 0 {
            e.vcall(inner, 0, &args![1u32]);
        }
    });
}

/// `006b7770`, the cell's name (virtual slot +0x130) or `""` without a
/// cell, then `005b5e40(format, info id, name)`: the `in cell %s` warnings
/// of `CheckMap` (the cell asked for twice, as the exe does).
fn warn_with_cell(e: &mut Engine, format: u32, info: u32) {
    let name = if fn_006b7770(e, Ptr::new(info)) != 0 {
        let cell = fn_006b7770(e, Ptr::new(info));
        e.vcall(cell, SLOT_CELL_NAME, &args![]).u32()
    } else {
        EMPTY_STRING
    };
    let id = e.call(FIRST_OF_00559450, &args![info]).u32();
    e.call(PRINT_WARNING_005B5E40, &args![format, id, name]);
}

/// One of `CheckMap`'s two portal walks over the array at `info` + `offset`
/// (`AdjacentNavMeshes` +0x24 or `PreferredAdjacentNavMeshes` +0x34, Xbox
/// PDB): an entry that is null or whose id has no info in the map
/// (`00693d70`) is reported and removed (`006bf8f0(index, 1)`), and the walk
/// goes on at the same index.
fn remove_dangling_portals(e: &mut Engine, this: Ptr, info_slot: u32, offset: u32) {
    let mut index: u32 = 0;
    loop {
        let info = e.mem.u32(info_slot);
        let array = info.wrapping_add(offset);
        if index >= e.call(ARRAY_SIZE_0044DDC0, &args![array]).u32() {
            break;
        }
        let mut missing = false;
        let element = e.call(ARRAY_AT_00877A30, &args![array, index]).u32();
        if e.mem.u32(element) == 0 {
            missing = true;
        } else {
            let element = e.call(ARRAY_AT_00877A30, &args![array, index]).u32();
            let neighbour = e.mem.u32(element);
            let id = e.call(FIRST_OF_00559450, &args![neighbour]).u32();
            if e.call(INFO_OF_ID_00693D70, &args![this, id]).u32() == 0 {
                missing = true;
            }
        }
        if missing {
            warn_with_cell(e, PORTAL_WITHOUT_NAV_MESH_WARNING, info);
            let info = e.mem.u32(info_slot);
            e.call(
                ARRAY_REMOVE_006BF8F0,
                &args![info.wrapping_add(offset), index, 1u32],
            );
            index = index.wrapping_sub(1);
        }
        index = index.wrapping_add(1);
    }
}

/// One of `CheckMap`'s two triangle-portal walks over the array at `info` +
/// `offset`: for each neighbour (kept at `target_slot`), a local triangle
/// array at `triangles` (`00694de0`) is filled by
/// `NavMesh::FindPortalTriangles` on the navmesh in `mesh`; an empty result
/// (`0076b610`) is reported and ends the walk with true. The array is
/// destroyed (`00694e10`) each time.
fn find_missing_triangle_portal(
    e: &mut Engine,
    info_slot: u32,
    offset: u32,
    mesh: u32,
    target_slot: u32,
    triangles: u32,
) -> bool {
    let mut index: u32 = 0;
    loop {
        let info = e.mem.u32(info_slot);
        let array = info.wrapping_add(offset);
        if index >= e.call(ARRAY_SIZE_0044DDC0, &args![array]).u32() {
            return false;
        }
        let element = e.call(ARRAY_AT_00877A30, &args![array, index]).u32();
        let target = e.mem.u32(element);
        e.mem.set_u32(target_slot, target);
        e.call(TRIANGLE_ARRAY_CONSTRUCT_00694DE0, &args![triangles]);
        let nav_mesh = e.call(FIRST_OF_00559450, &args![mesh]).u32();
        e.call(FIND_PORTAL_TRIANGLES, &args![nav_mesh, target, triangles]);
        if e.call(ARRAY_IS_EMPTY_0076B610, &args![triangles]).u8() != 0 {
            let target = e.mem.u32(target_slot);
            let target_id = e.call(FIRST_OF_00559450, &args![target]).u32();
            let info = e.mem.u32(info_slot);
            let id = e.call(FIRST_OF_00559450, &args![info]).u32();
            e.call(
                PRINT_WARNING_005B5E40,
                &args![NO_TRIANGLE_PORTAL_WARNING, id, target_id],
            );
            e.call(TRIANGLE_ARRAY_DESTRUCT_00694E10, &args![triangles]);
            return true;
        }
        e.call(TRIANGLE_ARRAY_DESTRUCT_00694E10, &args![triangles]);
        index = index.wrapping_add(1);
    }
}

/// Whether the array at `info` + `offset` holds `target` (the first equal
/// entry ends the search).
fn portal_listed(e: &mut Engine, info: u32, offset: u32, target: u32) -> bool {
    let array = info.wrapping_add(offset);
    let count = e.call(ARRAY_SIZE_0044DDC0, &args![array]).u32();
    let mut index: u32 = 0;
    while index < count {
        let element = e.call(ARRAY_AT_00877A30, &args![array, index]).u32();
        if e.mem.u32(element) == target {
            return true;
        }
        index = index.wrapping_add(1);
    }
    false
}

/// The portal and door checks of `CheckMap` for an info whose navmesh is in
/// `mesh`: whether the info needs rebuilding (each check stops at its
/// first finding and reports it).
fn portals_disagree(
    e: &mut Engine,
    info_slot: u32,
    mesh: u32,
    near: u32,
    frame: &dyn Fn(u32) -> u32,
) -> bool {
    let mut changed = false;
    if find_missing_triangle_portal(e, info_slot, 0x24, mesh, frame(0x88), frame(0x84)) {
        changed = true;
    }
    if find_missing_triangle_portal(e, info_slot, 0x34, mesh, frame(0xa0), frame(0x9c)) {
        changed = true;
    }
    // Every linked triangle edge of the navmesh: a neighbour neither array
    // lists but the nearby infos hold (`00719b20` on the local array).
    let nav_mesh = e.call(FIRST_OF_00559450, &args![mesh]).u32();
    let triangles = e.call(TRIANGLE_COUNT_00695370, &args![nav_mesh]).u32();
    let target_slot = frame(0xb0);
    let mut index: u32 = 0;
    while index < triangles {
        let nav_mesh = e.call(FIRST_OF_00559450, &args![mesh]).u32();
        let triangle = e
            .call(TRIANGLE_AT_00693D30, &args![nav_mesh, index & 0xffff])
            .u32();
        if e.call(EDGE_LINK_SET_0069E090, &args![triangle.wrapping_add(4)])
            .u8()
            != 0
        {
            let nav_mesh = e.call(FIRST_OF_00559450, &args![mesh]).u32();
            let triangle = e
                .call(TRIANGLE_AT_00693D30, &args![nav_mesh, index & 0xffff])
                .u32();
            let target = e.mem.u32(triangle.wrapping_add(4));
            e.mem.set_u32(target_slot, target);
            // Both arrays are searched (the exe runs both walks).
            let info = e.mem.u32(info_slot);
            let in_adjacent = portal_listed(e, info, 0x24, target);
            let in_preferred = portal_listed(e, info, 0x34, target);
            if !in_adjacent && !in_preferred {
                let found = e
                    .call(
                        ARRAY_FIND_00719B20,
                        &args![near, target_slot, 0u32, COMPARE_009A3830],
                    )
                    .u32();
                if found != 0xffff_ffff {
                    let target = e.mem.u32(target_slot);
                    let target_id = e.call(FIRST_OF_00559450, &args![target]).u32();
                    let id = e.call(FIRST_OF_00559450, &args![info]).u32();
                    e.call(
                        PRINT_WARNING_005B5E40,
                        &args![MISSING_PORTAL_WARNING, id, target_id],
                    );
                    changed = true;
                    break;
                }
            }
        }
        index = index.wrapping_add(1);
    }
    // Every door of the navmesh must be in `ConnectedDoors` (+0x44).
    let mut index: u32 = 0;
    loop {
        let nav_mesh = e.call(FIRST_OF_00559450, &args![mesh]).u32();
        if index >= e.call(DOOR_COUNT_006B5A90, &args![nav_mesh]).u32() {
            break;
        }
        let nav_mesh = e.call(FIRST_OF_00559450, &args![mesh]).u32();
        let door = e
            .call(DOOR_AT_0068F6B0, &args![nav_mesh, index & 0xffff])
            .u32();
        let info = e.mem.u32(info_slot);
        let found = e
            .call(
                ARRAY_FIND_00719B20,
                &args![info.wrapping_add(0x44), door, 0u32, COMPARE_009A3830],
            )
            .u32();
        if found == 0xffff_ffff {
            let reference = e.mem.u32(door);
            let door_id = e.call(FORM_ID_0084E3A0, &args![reference]).u32();
            let id = e.call(FIRST_OF_00559450, &args![info]).u32();
            e.call(
                PRINT_WARNING_005B5E40,
                &args![MISSING_DOOR_PORTAL_WARNING, id, door_id],
            );
            changed = true;
            break;
        }
        index = index.wrapping_add(1);
    }
    // Every door in `ConnectedDoors` must be one of the navmesh's.
    let mut index: u32 = 0;
    loop {
        let info = e.mem.u32(info_slot);
        let doors = info.wrapping_add(0x44);
        if index >= e.call(ARRAY_SIZE_0044DDC0, &args![doors]).u32() {
            break;
        }
        let element = e.call(ARRAY_AT_00877A30, &args![doors, index]).u32();
        let door = e.mem.u32(element);
        let mut found = false;
        let mut door_index: u32 = 0;
        loop {
            let nav_mesh = e.call(FIRST_OF_00559450, &args![mesh]).u32();
            if door_index >= e.call(DOOR_COUNT_006B5A90, &args![nav_mesh]).u32() {
                break;
            }
            let nav_mesh = e.call(FIRST_OF_00559450, &args![mesh]).u32();
            let entry = e
                .call(DOOR_AT_0068F6B0, &args![nav_mesh, door_index & 0xffff])
                .u32();
            if e.mem.u32(entry) == door {
                found = true;
                break;
            }
            door_index = door_index.wrapping_add(1);
        }
        if !found {
            let door_id = e.call(FORM_ID_0084E3A0, &args![door]).u32();
            let id = e.call(FIRST_OF_00559450, &args![info]).u32();
            e.call(
                PRINT_WARNING_005B5E40,
                &args![EXTRA_DOOR_PORTAL_WARNING, id, door_id],
            );
            changed = true;
            break;
        }
        index = index.wrapping_add(1);
    }
    changed
}

// Translated from 006b6c60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006b6c60 - its FS:[0] exception frame (the unwind states that destroy the local arrays and NiPointers) is not translated
/// `NavMeshInfoMap::CheckMap` (engine map, from its strings): checks every
/// info of the map at +0x1C, collecting the bad ones in a local array
/// (`0069aee0`). An info with no parent (`00690830` null), with no navmesh
/// (`0069dfb0`; also `NavMesh::FixPortalsFromNavMeshDeletion`) or whose
/// navmesh pointer (`00690860`) comes back empty is reported (`in cell %s`)
/// and collected. Otherwise: a navmesh with form flag 0x20 (`00440d80`)
/// collects the info but the checks go on; dangling portals of both portal
/// arrays are reported and removed; with a worldspace (`00690800`) the infos
/// of the eight neighbouring cells (`00587520`, `00587410`, `006b7850`) are
/// gathered; with the navmesh again in a second pointer, the triangle,
/// portal and door checks run and any finding rebuilds the info
/// (`006b5560`). Finally every collected info is removed from the map
/// (`006b6990` with its id) and deleted (`006b6390(info, 1)`, null skipped),
/// and the local array is destroyed.
pub fn nav_mesh_info_map_check_map(e: &mut Engine, this: Ptr) {
    let info_map = Ptr::new(this.addr() + 0x1c);
    // The frame from EBP-0x110; `at(n)` is EBP-n.
    e.with_stack(0x110, |e, frame| {
        let base = frame.addr();
        let at = move |offset: u32| base + 0x110 - offset;
        let bad = at(0x20);
        let (it, key, info_slot) = (at(0x10), at(0x28), at(0x24));
        let (first_mesh, second_mesh, near) = (at(0x2c), at(0x30), at(0x40));
        let (cell_x, cell_y) = (at(0x58), at(0x54));
        e.call(ARRAY_CTOR_0069AEE0, &args![bad]);
        let first = e.call(MAP_FIRST_004B9BA0, &args![info_map]).u32();
        e.mem.set_u32(it, first);
        while e.mem.u32(it) != 0 {
            fn_006b7f20(
                e,
                info_map,
                Ptr::new(it),
                Ptr::new(key),
                Ptr::new(info_slot),
            );
            let info = e.mem.u32(info_slot);
            if e.call(INFO_PARENT_00690830, &args![info]).u32() == 0 {
                warn_with_cell(e, NO_PARENT_SPACE_WARNING, info);
                e.call(ARRAY_ADD_007CB2E0, &args![bad, info_slot]);
                continue;
            }
            let info = e.mem.u32(info_slot);
            if e.call(INFO_HAS_NAV_MESH_0069DFB0, &args![info]).u8() == 0 {
                warn_with_cell(e, NO_NAV_MESH_WARNING, info);
                let info = e.mem.u32(info_slot);
                e.call(FIX_PORTALS_FROM_NAV_MESH_DELETION, &args![info]);
                e.call(ARRAY_ADD_007CB2E0, &args![bad, info_slot]);
                continue;
            }
            e.call(NI_POINTER_CONSTRUCT_0042FB00, &args![first_mesh]);
            let info = e.mem.u32(info_slot);
            if e.call(INFO_NAV_MESH_00690860, &args![info, first_mesh])
                .u8()
                == 0
            {
                warn_with_cell(e, DELETED_NAV_MESH_WARNING, info);
                e.call(ARRAY_ADD_007CB2E0, &args![bad, info_slot]);
                e.call(NI_POINTER_DESTRUCT_0042FA40, &args![first_mesh]);
                continue;
            }
            let nav_mesh = e.call(FIRST_OF_00559450, &args![first_mesh]).u32();
            if e.call(FORM_FLAG_20_00440D80, &args![nav_mesh]).u8() != 0 {
                e.call(ARRAY_ADD_007CB2E0, &args![bad, info_slot]);
            }
            remove_dangling_portals(e, this, info_slot, 0x24);
            remove_dangling_portals(e, this, info_slot, 0x34);
            e.call(ARRAY_CTOR_0069AEE0, &args![near]);
            let info = e.mem.u32(info_slot);
            if e.call(INFO_WORLD_SPACE_00690800, &args![info]).u32() != 0 {
                // NavMeshInfo::iCellKey (Xbox PDB) +0xC.
                let cell_key = e.mem.u32(info.wrapping_add(0xc));
                e.call(SPLIT_CELL_KEY_00587520, &args![cell_key, cell_x, cell_y]);
                let own_x = |e: &Engine| i32::from(e.mem.u16(cell_x) as i16);
                let own_y = |e: &Engine| i32::from(e.mem.u16(cell_y) as i16);
                let mut x = own_x(e).wrapping_sub(1);
                while x <= own_x(e).wrapping_add(1) {
                    let mut y = own_y(e).wrapping_sub(1);
                    while y <= own_y(e).wrapping_add(1) {
                        if x != own_x(e) || y != own_y(e) {
                            let cell = e
                                .call(
                                    CELL_KEY_00587410,
                                    &args![x as u32 & 0xffff, y as u32 & 0xffff],
                                )
                                .u32();
                            let info = e.mem.u32(info_slot);
                            let world = e.call(INFO_WORLD_SPACE_00690800, &args![info]).ptr();
                            let list = fn_006b7850(e, this, world, cell);
                            if list != 0 {
                                let mut index: u32 = 0;
                                while index < e.call(ARRAY_SIZE_0044DDC0, &args![list]).u32() {
                                    let element =
                                        e.call(ARRAY_AT_006A7AD0, &args![list, index]).u32();
                                    e.call(ARRAY_ADD_007CB2E0, &args![near, element]);
                                    index = index.wrapping_add(1);
                                }
                            }
                        }
                        y = y.wrapping_add(1);
                    }
                    x = x.wrapping_add(1);
                }
            }
            e.call(NI_POINTER_CONSTRUCT_0042FB00, &args![second_mesh]);
            let info = e.mem.u32(info_slot);
            if e.call(INFO_NAV_MESH_00690860, &args![info, second_mesh])
                .u8()
                != 0
                && portals_disagree(e, info_slot, second_mesh, near, &at)
            {
                let nav_mesh = e.call(FIRST_OF_00559450, &args![second_mesh]).u32();
                let info = e.mem.u32(info_slot);
                e.call(INFO_REBUILD_006B5560, &args![info, nav_mesh]);
            }
            e.call(NI_POINTER_DESTRUCT_0042FA40, &args![second_mesh]);
            e.call(ARRAY_DTOR_0069AF10, &args![near]);
            e.call(NI_POINTER_DESTRUCT_0042FA40, &args![first_mesh]);
        }
        let mut index: u32 = 0;
        while index < e.call(ARRAY_SIZE_0044DDC0, &args![bad]).u32() {
            let element = e.call(ARRAY_AT_00877A30, &args![bad, index]).u32();
            let info = e.mem.u32(element);
            let id = e.call(FIRST_OF_00559450, &args![info]).u32();
            fn_006b6990(e, this, id);
            let element = e.call(ARRAY_AT_00877A30, &args![bad, index]).u32();
            let info = e.mem.u32(element);
            if info != 0 {
                e.call(NAV_MESH_INFO_DELETE, &args![info, 1u32]);
            }
            index = index.wrapping_add(1);
        }
        e.call(ARRAY_DTOR_0069AF10, &args![bad]);
    });
}

// Translated from 006b6580 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006b6580 - its FS:[0] exception frame (the unwind state that destroys the local NiPointer) is not translated
/// Unnamed in the engine map (`NavMeshInfoMap`'s virtual slot +0x1C;
/// `ECX` only): nothing without form flag 2 (`00460340`). Otherwise it
/// counts the entries of the list in the object at `011c54c4` (count
/// `0084e3a0`, entries `00877a30`) whose form type is 0x43; with none, it
/// walks the info map at +0x1C for one whose `0047c850` is set, and without
/// one runs the form's virtual slot +0xC8 with 0 and stops. Then, for each
/// info whose navmesh pointer (`00690860`) is set, a navmesh with form flag
/// 2 and without flag 0x20 gets `006986d0`; then `CheckMap`. Returns 1.
pub fn fn_006b6580(e: &mut Engine, this: Ptr) -> u8 {
    if e.call(FORM_FLAG_2_00460340, &args![this]).u8() == 0 {
        return 1;
    }
    let list: u32 = e.global(FORM_LIST_011C54C4);
    let count = e.call(FORM_ID_0084E3A0, &args![list]).u32();
    let mut of_type: u32 = 0;
    let mut index: u32 = 0;
    while index < count {
        let list: u32 = e.global(FORM_LIST_011C54C4);
        let element = e.call(ARRAY_AT_00877A30, &args![list, index]).u32();
        let form = e.mem.u32(element);
        if e.call(FORM_TYPE_00401170, &args![form]).u32() == 0x43 {
            of_type = of_type.wrapping_add(1);
        }
        index = index.wrapping_add(1);
    }
    let info_map = Ptr::new(this.addr() + 0x1c);
    // it (EBP-0x24), key (EBP-0x30), value (EBP-0x2C); it2 (EBP-0x10),
    // key2 (EBP-0x3C), value2 (EBP-0x38), the NiPointer (EBP-0x34).
    e.with_stack(0x1c, |e, locals| {
        let base = locals.addr();
        let (it, key, value) = (base, base + 4, base + 8);
        let (it2, key2, value2, mesh) = (base + 0xc, base + 0x10, base + 0x14, base + 0x18);
        if of_type == 0 {
            let mut found = false;
            let first = e.call(MAP_FIRST_004B9BA0, &args![info_map]).u32();
            e.mem.set_u32(it, first);
            while e.mem.u32(it) != 0 {
                fn_006b7f20(e, info_map, Ptr::new(it), Ptr::new(key), Ptr::new(value));
                let info = e.mem.u32(value);
                if e.call(ALWAYS_ZERO_0047C850, &args![info]).u8() != 0 {
                    found = true;
                    break;
                }
            }
            if !found {
                e.vcall(this.addr(), SLOT_MAP_FINISH, &args![0u32]);
                return;
            }
        }
        let first = e.call(MAP_FIRST_004B9BA0, &args![info_map]).u32();
        e.mem.set_u32(it2, first);
        while e.mem.u32(it2) != 0 {
            fn_006b7f20(e, info_map, Ptr::new(it2), Ptr::new(key2), Ptr::new(value2));
            e.call(NI_POINTER_CONSTRUCT_0042FB00, &args![mesh]);
            let info = e.mem.u32(value2);
            if e.call(INFO_NAV_MESH_00690860, &args![info, mesh]).u8() != 0 {
                let nav_mesh = e.call(FIRST_OF_00559450, &args![mesh]).u32();
                if e.call(FORM_FLAG_2_00460340, &args![nav_mesh]).u8() != 0 {
                    let nav_mesh = e.call(FIRST_OF_00559450, &args![mesh]).u32();
                    if e.call(FORM_FLAG_20_00440D80, &args![nav_mesh]).u8() == 0 {
                        let nav_mesh = e.call(FIRST_OF_00559450, &args![mesh]).u32();
                        e.call(NAV_MESH_STEP_006986D0, &args![nav_mesh]);
                    }
                }
            }
            e.call(NI_POINTER_DESTRUCT_0042FA40, &args![mesh]);
        }
        nav_mesh_info_map_check_map(e, this);
    });
    1
}

// Translated from 006b6740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`NavMeshInfoMap`'s virtual slot +0x2C; `ECX`
/// only): writes the form. `TESForm::StartForm`, an `NVER` word chunk of
/// 0xB (`00485910`); per info of the map at +0x1C an `NVMI` chunk (a block
/// of `006b4880` bytes, filled by `006b48f0`); then per info an `NVCI`
/// chunk: a block of `006b4e10` + 4 bytes holding the info's id (`00559450`,
/// byte-swapped by `00401080` when `00401500` asks) and the connection data
/// (`006b4e80`). Each block comes from `00401000` and goes back to
/// `00401030`. Last `TESForm::CloseForm`.
pub fn fn_006b6740(e: &mut Engine, this: Ptr) {
    let info_map = Ptr::new(this.addr() + 0x1c);
    e.call(FORM_START_FORM, &args![this]);
    e.call(FORM_ADD_WORD_DATA_00485910, &args![CHUNK_NVER, 0xbu32]);
    // it (EBP-4), value (EBP-0xC), key (EBP-0x14), the id word (EBP-0x18),
    // value2 (EBP-0x20), key2 (EBP-0x2C).
    e.with_stack(0x18, |e, locals| {
        let base = locals.addr();
        let (it, value, key) = (base, base + 4, base + 8);
        let (word, value2, key2) = (base + 0xc, base + 0x10, base + 0x14);
        let first = e.call(MAP_FIRST_004B9BA0, &args![info_map]).u32();
        e.mem.set_u32(it, first);
        while e.mem.u32(it) != 0 {
            fn_006b7f20(e, info_map, Ptr::new(it), Ptr::new(key), Ptr::new(value));
            let info = e.mem.u32(value);
            let size = e.call(INFO_DATA_SIZE_006B4880, &args![info]).u32();
            let block = e.call(ALLOCATE, &args![size]).u32();
            let info = e.mem.u32(value);
            e.call(INFO_WRITE_DATA_006B48F0, &args![info, block]);
            e.call(FORM_ADD_CHUNK_DATA, &args![CHUNK_NVMI, block, size]);
            e.call(DEALLOCATE, &args![block]);
        }
        let first = e.call(MAP_FIRST_004B9BA0, &args![info_map]).u32();
        e.mem.set_u32(it, first);
        while e.mem.u32(it) != 0 {
            fn_006b7f20(e, info_map, Ptr::new(it), Ptr::new(key2), Ptr::new(value2));
            let info = e.mem.u32(value2);
            let size = e
                .call(INFO_CONNECTIONS_SIZE_006B4E10, &args![info])
                .u32()
                .wrapping_add(4);
            let block = e.call(ALLOCATE, &args![size]).u32();
            let info = e.mem.u32(value2);
            let id = e.call(FIRST_OF_00559450, &args![info]).u32();
            e.mem.set_u32(word, id);
            if e.call(SWAP_BYTES_00401500, &args![]).u8() != 0 {
                e.call(BYTE_SWAP_00401080, &args![word, 0u32]);
            }
            let id = e.mem.u32(word);
            e.mem.set_u32(block, id);
            let info = e.mem.u32(value2);
            e.call(
                INFO_WRITE_CONNECTIONS_006B4E80,
                &args![info, block.wrapping_add(4)],
            );
            e.call(FORM_ADD_CHUNK_DATA, &args![CHUNK_NVCI, block, size]);
            e.call(DEALLOCATE, &args![block]);
        }
    });
    e.call(FORM_CLOSE_FORM, &args![this]);
}

// Translated from 006dfcb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006dfcb0 - its FS:[0] exception frame (the unwind state that destroys the member array) is not translated
/// Unnamed in the engine map (`RET 8`): constructs the 0x1C-byte item
/// `006dfba0` makes: the array at +4 (`006e1070`), then `006df200(first,
/// second)` (the byte at +0 cleared, the words at +0x14 and +0x18).
/// Returns `this`.
pub fn fn_006dfcb0(e: &mut Engine, this: Ptr, first: u32, second: u32) -> Ptr {
    e.call(
        ITEM_ARRAY_CONSTRUCT_006E1070,
        &args![this.addr().wrapping_add(4)],
    );
    e.call(ITEM_INIT_006DF200, &args![this, first, second]);
    this
}

// Translated from 006dfba0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006dfba0 - its FS:[0] exception frame (the unwind state that frees the new item when its constructor throws) is not translated
/// The engine map names this `NavMeshInfoMap::GetAllNavMeshInfo` (from the
/// call graph); the body is an array method (`RET 4`), so the name is not
/// used. `navmesh` points to the word holding a navmesh. The count is
/// `006911c0` of that navmesh; when the word at +0x10 is null or the size
/// (`0044ddc0`) differs from the count, the array is emptied
/// (`008454f0(0)`), reserved to the count (`0084b5c0`) and gets one new
/// 0x1C-byte item per index (`00401000`; `006dfcb0` with the navmesh's word
/// `0068efb0` and the index; null when the allocation fails), appended
/// (`007cb2e0` with the address of a slot holding it); then the word at
/// +0x10 becomes the navmesh's word `0068efb0`.
pub fn fn_006dfba0(e: &mut Engine, this: Ptr, navmesh: Ptr) {
    let nav_mesh = e.call(FIRST_OF_00559450, &args![navmesh]).u32();
    let count = e.call(NAV_MESH_COUNT_006911C0, &args![nav_mesh]).u32();
    if e.mem.u32(this.addr().wrapping_add(0x10)) != 0
        && e.call(ARRAY_SIZE_0044DDC0, &args![this]).u32() == count
    {
        return;
    }
    e.call(ARRAY_RELEASE_008454F0, &args![this, 0u32]);
    e.call(ARRAY_SET_RESERVED_SIZE_0084B5C0, &args![this, count]);
    // The slot the new item is appended from (EBP-0x18).
    e.with_stack(4, |e, slot| {
        let mut index: u32 = 0;
        while index < count {
            let block = e.call(ALLOCATE, &args![0x1cu32]).u32();
            let item = if block != 0 {
                let nav_mesh = e.call(FIRST_OF_00559450, &args![navmesh]).u32();
                let word = e.call(NAV_MESH_VALUE_0068EFB0, &args![nav_mesh]).u32();
                fn_006dfcb0(e, Ptr::new(block), word, index).addr()
            } else {
                0
            };
            e.mem.set_u32(slot.addr(), item);
            e.call(ARRAY_ADD_007CB2E0, &args![this, slot]);
            index = index.wrapping_add(1);
        }
    });
    let nav_mesh = e.call(FIRST_OF_00559450, &args![navmesh]).u32();
    let word = e.call(NAV_MESH_VALUE_0068EFB0, &args![nav_mesh]).u32();
    e.mem.set_u32(this.addr().wrapping_add(0x10), word);
}

// Translated from 006b7e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerMap<unsigned int, NavMeshInfo*>` scalar deleting destructor
/// (engine map, from the vtable; `RET 4`): the destructor body `006b7fd0`,
/// then `00401030(this)` when bit 0 of `flags` is set. Returns `this`.
pub fn ni_t_pointer_map_nav_mesh_info_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_006b7fd0(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 006b7e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the worldspace `NiTMap` (engine map
/// `NiTMap<unsigned_int_NiTMap<..>_P>::_scalar_deleting_destructor_`;
/// `RET 4`): the destructor body `006b80d0`, then `00401030(this)` when bit
/// 0 of `flags` is set. Returns `this`.
pub fn ni_t_map_world_space_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_006b80d0(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 006b8280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned int>, unsigned int, NavMeshInfo*>`
/// scalar deleting destructor (engine map, from the vtable; `RET 4`): the
/// destructor body `006b8030`, then `00401030(this)` when bit 0 of `flags`
/// is set. Returns `this`.
pub fn ni_t_map_base_nav_mesh_info_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_006b8030(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 006b82b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the `NiTMapBase` under the worldspace map
/// (engine map, from the vtable; `RET 4`): the destructor body `006b8130`,
/// then `00401030(this)` when bit 0 of `flags` is set. Returns `this`.
pub fn ni_t_map_base_world_space_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_006b8130(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 006b82e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the `NiTMapBase` under the cell map
/// (engine map, from the vtable; `RET 4`): the destructor body `006b8250`,
/// then `00401030(this)` when bit 0 of `flags` is set. Returns `this`.
pub fn ni_t_map_base_cell_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006b8250(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006b_6910, fn_006b6910(Ptr, Ptr) -> u32),
        entry!(0x006b_6940, nav_mesh_info_map_add_nav_mesh_info(Ptr, u32)),
        entry!(0x006b_6990, fn_006b6990(Ptr, u32)),
        entry!(
            0x006b_6a00,
            nav_mesh_info_map_delete_nav_mesh_info(Ptr, u32)
        ),
        entry!(0x006b_7770, fn_006b7770(Ptr) -> u32),
        entry!(0x006b_7910, fn_006b7910(Ptr, Ptr)),
        entry!(0x006b_7b70, fn_006b7b70(Ptr, u32)),
        entry!(0x006b_6c60, nav_mesh_info_map_check_map(Ptr)),
        entry!(0x006b_6580, fn_006b6580(Ptr) -> u8),
        entry!(0x006b_6740, fn_006b6740(Ptr)),
        entry!(0x006d_fcb0, fn_006dfcb0(Ptr, u32, u32) -> Ptr),
        entry!(0x006d_fba0, fn_006dfba0(Ptr, Ptr)),
        entry!(0x006b_7e20, ni_t_pointer_map_nav_mesh_info_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x006b_7e50, ni_t_map_world_space_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x006b_8280, ni_t_map_base_nav_mesh_info_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x006b_82b0, ni_t_map_base_world_space_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x006b_82e0, ni_t_map_base_cell_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x006b_7f20, fn_006b7f20(Ptr, Ptr, Ptr, Ptr)),
        entry!(0x006b81d0, keys_equal(Ptr, u32, u32) -> u8),
        entry!(0x006b_7eb0, fn_006b7eb0(Ptr, u32) -> Ptr),
        entry!(0x006b_8060, fn_006b8060(Ptr, u32) -> Ptr),
        entry!(0x006b_8160, fn_006b8160(Ptr, u32) -> Ptr),
        entry!(0x006b_7d90, fn_006b7d90(Ptr, u32) -> Ptr),
        entry!(0x006b_7dc0, fn_006b7dc0(Ptr, u32) -> Ptr),
        entry!(0x006b_7df0, fn_006b7df0(Ptr, u32) -> Ptr),
        entry!(0x006b_8030, fn_006b8030(Ptr)),
        entry!(0x006b_8130, fn_006b8130(Ptr)),
        entry!(0x006b_8250, fn_006b8250(Ptr)),
        entry!(0x006b_7fd0, fn_006b7fd0(Ptr)),
        entry!(0x006b_80d0, fn_006b80d0(Ptr)),
        entry!(0x006b_81f0, fn_006b81f0(Ptr)),
        entry!(0x006b_7e80, fn_006b7e80(Ptr, u32) -> Ptr),
        entry!(0x006b_5c30, nav_mesh_info_map_nav_mesh_info_map(Ptr) -> Ptr),
        entry!(0x006b_77b0, fn_006b77b0(Ptr) -> u32),
        entry!(0x006b_7b30, fn_006b7b30(Ptr) -> u32),
        entry!(0x006b_7850, fn_006b7850(Ptr, Ptr, u32) -> u32),
        entry!(0x006b_78b0, fn_006b78b0(Ptr, Ptr) -> u32),
        entry!(0x006b_77e0, nav_mesh_info_map_find_nav_mesh_info_for_cell(Ptr, Ptr) -> u32),
        entry!(
            0x006b_7980,
            nav_mesh_info_map_add_info_to_cell_key_map(Ptr, u32)
        ),
        entry!(0x006b_7cb0, fn_006b7cb0(Ptr) -> bool),
        entry!(0x006b_68b0, fn_006b68b0(Ptr, u8)),
        entry!(0x006b_6b20, fn_006b6b20(Ptr)),
        entry!(0x006b_63c0, nav_mesh_info_map_init_item(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_equality_matches_native_low_byte_vectors() {
        // Generated native vectors, FalloutNV.exe 1.4.0.525 SHA256
        // 19406942e48724d797300c4ea6be9ac69a32f670b8c35db09279a2258422739f,
        // Ryzen AI 9 HX 370, FCW027F/MXCSR1F80. Only AL is the boolean result.
        let mut e = Engine::new();
        for (left, right, expected) in [
            (0u32, 0u32, 1u8),
            (0x12345678, 0x12345678, 1),
            (0x12345678, 0x12345679, 0),
            (0, 0x80000000, 0),
        ] {
            assert_eq!(e.call(0x006b81d0, &args![0u32, left, right]).u8(), expected);
        }
    }
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    /// A map with `buckets` buckets whose slot +4 answers `key % buckets`; entries are
    /// (next, key, value).
    fn map(e: &mut Engine, buckets: u32) -> u32 {
        let mut slots = [0u32; 3];
        slots[1] = 0x0f00_a000;
        e.put_vtable(0x0f00_a100, &slots);
        e.register_double(0x0f00_a000, move |_, a| (a[1] % buckets).into_ret());
        let map = e.mem.alloc(0x10);
        let table = e.mem.alloc(4 * buckets);
        e.mem.set_u32(map, 0x0f00_a100);
        e.mem.set_u32(map + 4, buckets);
        e.mem.set_u32(map + 8, table);
        map
    }

    fn entry(e: &mut Engine, next: u32, key: u32, value: u32) -> u32 {
        let entry = e.mem.alloc(0xc);
        e.mem.set_u32(entry, next);
        e.mem.set_u32(entry + 4, key);
        e.mem.set_u32(entry + 8, value);
        entry
    }

    fn step(e: &mut Engine, map: u32, current: u32) -> (u32, u32, u32) {
        let out = e.mem.alloc(0xc);
        e.mem.set_u32(out, current);
        e.call(0x006b_7f20, &args![map, out, out + 4, out + 8]);
        (e.mem.u32(out), e.mem.u32(out + 4), e.mem.u32(out + 8))
    }

    #[test]
    fn at_006b7f20_follows_the_chain_when_the_entry_has_a_next() {
        let mut e = Engine::new();
        let map = map(&mut e, 4);
        let second = entry(&mut e, 0, 8, 0xbbbb);
        let first = entry(&mut e, second, 4, 0xaaaa);
        assert_eq!(step(&mut e, map, first), (second, 4, 0xaaaa));
    }

    #[test]
    fn at_006b7f20_moves_to_the_next_non_empty_bucket_at_the_end_of_a_chain() {
        let mut e = Engine::new();
        let map = map(&mut e, 4);
        let table = e.mem.u32(map + 8);
        let in_bucket_three = entry(&mut e, 0, 7, 0xdddd);
        e.mem.set_u32(table + 12, in_bucket_three);
        // Key 5 sits in bucket 1; buckets 2 is empty, bucket 3 has an entry.
        let last = entry(&mut e, 0, 5, 0xcccc);
        assert_eq!(step(&mut e, map, last), (in_bucket_three, 5, 0xcccc));
    }

    #[test]
    fn at_006b7f20_ends_with_null_after_the_last_bucket() {
        let mut e = Engine::new();
        let map = map(&mut e, 4);
        let last = entry(&mut e, 0, 7, 0xeeee);
        // Key 7 is in the last bucket (3): nothing follows.
        assert_eq!(step(&mut e, map, last), (0, 7, 0xeeee));
    }
}

#[cfg(test)]
mod info_map_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    const MAP_VTABLE: u32 = 0x0f03_0000;
    const BUCKET_OF_KEY: u32 = 0x0f03_1000;

    /// The doubles that walk maps made by `map_of` (one bucket; slot +4
    /// answers bucket 0; `004b9ba0` gives the first entry).
    fn walkable(e: &mut Engine) {
        e.put_vtable(MAP_VTABLE, &[0, BUCKET_OF_KEY]);
        e.register(BUCKET_OF_KEY, |_, _| Ret::default());
        e.register(MAP_FIRST_004B9BA0, |e, a| {
            let buckets = e.mem.u32(a[0] + 8);
            e.mem.u32(buckets).into_ret()
        });
    }

    /// A one-bucket chained map holding `entries` (key, value) in order.
    fn map_of(e: &mut Engine, entries: &[(u32, u32)]) -> u32 {
        let map = e.mem.alloc(0x10);
        let table = e.mem.alloc(4);
        e.mem.set_u32(map, MAP_VTABLE);
        e.mem.set_u32(map + 4, 1);
        e.mem.set_u32(map + 8, table);
        let mut next = 0;
        for (key, value) in entries.iter().rev() {
            let entry = e.mem.alloc(0xc);
            e.mem.set_u32(entry, next);
            e.mem.set_u32(entry + 4, *key);
            e.mem.set_u32(entry + 8, *value);
            next = entry;
        }
        e.mem.set_u32(table, next);
        map
    }

    /// Writes a map made by `map_of` into the map member at `at`.
    fn place_map(e: &mut Engine, at: u32, entries: &[(u32, u32)]) {
        let map = map_of(e, entries);
        for i in 0..4 {
            let word = e.mem.u32(map + 4 * i);
            e.mem.set_u32(at + 4 * i, word);
        }
    }

    /// `00853130` over maps made by `map_of`, `00464f30` as key + 1 and the
    /// worldspace id at +0xC.
    fn lookups(e: &mut Engine) {
        e.register(MAP_GET_AT_00853130, |e, a| {
            let buckets = e.mem.u32(a[0] + 8);
            let mut entry = e.mem.u32(buckets);
            while entry != 0 {
                if e.mem.u32(entry + 4) == a[1] {
                    let value = e.mem.u32(entry + 8);
                    e.mem.set_u32(a[2], value);
                    return 1u32.into_ret();
                }
                entry = e.mem.u32(entry);
            }
            Ret::default()
        });
        e.register(KEY_OF_00464F30, |_, a| (a[0] + 1).into_ret());
        e.register(FORM_ID_0084E3A0, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
    }

    #[test]
    fn at_006b7eb0_family_builds_the_bucket_arrays() {
        for (address, vtable) in [
            (0x006b_7eb0u32, INFO_MAP_BASE_VTABLE),
            (0x006b_8060, WORLD_MAP_BASE_VTABLE),
            (0x006b_8160, CELL_MAP_BASE_VTABLE),
            (0x006b_7d90, INFO_MAP_VTABLE),
            (0x006b_7dc0, WORLD_MAP_VTABLE),
            (0x006b_7df0, CELL_MAP_VTABLE),
        ] {
            let mut e = Engine::new();
            e.register(BUCKET_ALLOCATE_00AA1070, |_, _| 0x0bad_0000u32.into_ret());
            e.register(FILL_00403D30, |_, _| Ret::default());
            let this = e.mem.alloc(0x10);
            e.mem.set_u32(this + 0xc, 0x55);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(address, &args![this, 0x67u32]).u32(), this);
            let log = e.call_log.take().unwrap();
            assert_eq!(e.mem.u32(this), vtable);
            assert_eq!(e.mem.u32(this + 4), 0x67);
            assert_eq!(e.mem.u32(this + 8), 0x0bad_0000);
            assert_eq!(e.mem.u32(this + 0xc), 0);
            assert_eq!(
                calls_to(&log, BUCKET_ALLOCATE_00AA1070),
                vec![vec![0x67 * 4]]
            );
            assert_eq!(
                calls_to(&log, FILL_00403D30),
                vec![vec![0x0bad_0000, 0, 0x67 * 4]]
            );
        }
    }

    #[test]
    fn at_006b8030_family_empties_and_frees() {
        for (address, removes, vtable) in [
            (0x006b_8030u32, 1usize, INFO_MAP_BASE_VTABLE),
            (0x006b_8130, 1, WORLD_MAP_BASE_VTABLE),
            (0x006b_8250, 1, CELL_MAP_BASE_VTABLE),
            (0x006b_7fd0, 2, INFO_MAP_BASE_VTABLE),
            (0x006b_80d0, 2, WORLD_MAP_BASE_VTABLE),
            (0x006b_81f0, 2, CELL_MAP_BASE_VTABLE),
        ] {
            let mut e = Engine::new();
            e.register(MAP_REMOVE_ALL_00438AF0, |_, _| Ret::default());
            e.register(BUCKET_FREE_00AA10F0, |_, _| Ret::default());
            let this = e.mem.alloc(0x10);
            e.mem.set_u32(this + 8, 0x0bad_0000);
            e.call_log = Some(vec![]);
            e.call(address, &args![this]);
            let log = e.call_log.take().unwrap();
            assert_eq!(e.mem.u32(this), vtable);
            assert_eq!(calls_to(&log, MAP_REMOVE_ALL_00438AF0).len(), removes);
            assert_eq!(
                calls_to(&log, BUCKET_FREE_00AA10F0),
                vec![vec![0x0bad_0000]]
            );
        }
    }

    #[test]
    fn at_006b7e80_destroys_then_deletes_on_flag_one() {
        for (flags, deletes) in [(0u32, 0usize), (1, 1), (2, 0)] {
            let mut e = Engine::new();
            e.register(MAP_REMOVE_ALL_00438AF0, |_, _| Ret::default());
            e.register(BUCKET_FREE_00AA10F0, |_, _| Ret::default());
            e.register(DEALLOCATE, |_, _| Ret::default());
            let this = e.mem.alloc(0x10);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x006b_7e80, &args![this, flags]).u32(), this);
            let log = e.call_log.take().unwrap();
            assert_eq!(e.mem.u32(this), CELL_MAP_BASE_VTABLE);
            assert_eq!(calls_to(&log, DEALLOCATE).len(), deletes);
        }
    }

    #[test]
    fn at_006b5c30_builds_the_form_and_its_maps() {
        let mut e = Engine::new();
        e.register(TES_FORM_CTOR, |_, _| Ret::default());
        e.register(BUCKET_ALLOCATE_00AA1070, |e, a| {
            e.mem.alloc(a[0]).into_ret()
        });
        e.register(FILL_00403D30, |_, _| Ret::default());
        e.register(SET_FORM_TYPE_004F15A0, |_, _| Ret::default());
        let this = e.mem.alloc(0x40);
        e.mem.write(this, &[0xcd; 0x40]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006b_5c30, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), NAV_MESH_INFO_MAP_VTABLE);
        assert_eq!(e.mem.u8(this + 0x18), 0);
        assert_eq!(e.mem.u32(this + 0x1c), INFO_MAP_VTABLE);
        assert_eq!(e.mem.u32(this + 0x20), 0x2719);
        assert_eq!(e.mem.u32(this + 0x2c), WORLD_MAP_VTABLE);
        assert_eq!(e.mem.u32(this + 0x30), 0x67);
        assert_eq!(e.mem.u8(this + 0x3c), 0);
        assert_eq!(calls_to(&log, TES_FORM_CTOR), vec![vec![this]]);
        assert_eq!(
            calls_to(&log, SET_FORM_TYPE_004F15A0),
            vec![vec![this, 0x38]]
        );
    }

    #[test]
    fn at_006b77b0_casts_the_parent_and_006b7b30_keys_it() {
        let mut e = Engine::new();
        e.register(INFO_PARENT_00690830, |e, a| {
            e.mem.u32(a[0] + 0x40).into_ret()
        });
        e.register(RT_DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(KEY_OF_00464F30, |_, a| (a[0] ^ 0xffff).into_ret());
        let info = e.mem.alloc(0x44);
        e.mem.set_u32(info + 0xc, 0x0c0c);
        e.mem.set_u32(info + 0x40, 0x4040);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006b_77b0, &args![info]).u32(), 0x4040);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, RT_DYNAMIC_CAST),
            vec![vec![0x4040, 0, CAST_SOURCE_TYPE, CAST_TARGET_TYPE, 0]]
        );
        assert_eq!(e.call(0x006b_7b30, &args![info]).u32(), 0x4040 ^ 0xffff);
        e.mem.set_u32(info + 0x40, 0);
        assert_eq!(e.call(0x006b_7b30, &args![info]).u32(), 0x0c0c ^ 0xffff);
    }

    #[test]
    fn at_006b7850_and_006b78b0_look_up_both_levels() {
        let mut e = Engine::new();
        lookups(&mut e);
        let cells = map_of(&mut e, &[(0x11, 0xaaaa), (0x501, 0xbbbb)]);
        let interiors = map_of(&mut e, &[(0x601, 0xcccc)]);
        let this = e.mem.alloc(0x40);
        place_map(&mut e, this + 0x2c, &[(0x0100_0001, cells), (0, interiors)]);
        let world = e.mem.alloc(0x10);
        e.mem.set_u32(world + 0xc, 0x0100_0001);
        assert_eq!(
            e.call(0x006b_7850, &args![this, world, 0x10u32]).u32(),
            0xaaaa
        );
        assert_eq!(e.call(0x006b_7850, &args![this, world, 0x99u32]).u32(), 0);
        e.mem.set_u32(world + 0xc, 0x0200_0000);
        assert_eq!(e.call(0x006b_7850, &args![this, world, 0x10u32]).u32(), 0);
        assert_eq!(e.call(0x006b_78b0, &args![this, 0x600u32]).u32(), 0xcccc);
        assert_eq!(e.call(0x006b_78b0, &args![this, 0x500u32]).u32(), 0);
    }

    #[test]
    fn at_006b77e0_picks_the_interior_or_exterior_lookup() {
        let mut e = Engine::new();
        lookups(&mut e);
        e.register(CELL_IS_INTERIOR_00425FD0, |e, a| {
            u32::from(e.mem.u8(a[0])).into_ret()
        });
        e.register(CELL_GET_DATA_X, |_, _| 3u32.into_ret());
        e.register(CELL_GET_DATA_Y, |_, _| 5u32.into_ret());
        e.register(CELL_KEY_00587410, |_, a| (a[0] * 0x100 + a[1]).into_ret());
        e.register(CELL_GET_WORLD_SPACE, |e, a| e.mem.u32(a[0] + 4).into_ret());
        let cells = map_of(&mut e, &[(0x306, 0xaaaa)]);
        let this = e.mem.alloc(0x40);
        let world = e.mem.alloc(0x10);
        e.mem.set_u32(world + 0xc, 0x77);
        let cell = e.mem.alloc(8);
        e.mem.set_u32(cell + 4, world);
        place_map(&mut e, this + 0x2c, &[(0x77, cells)]);
        assert_eq!(e.call(0x006b_77e0, &args![this, 0u32]).u32(), 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006b_77e0, &args![this, cell]).u32(), 0xaaaa);
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = log
            .iter()
            .map(|c| c.0)
            .filter(|a| [CELL_GET_DATA_X, CELL_GET_DATA_Y].contains(a))
            .collect();
        assert_eq!(order, vec![CELL_GET_DATA_Y, CELL_GET_DATA_X]);
        assert_eq!(calls_to(&log, CELL_KEY_00587410), vec![vec![3, 5]]);
        // Interior: entry 0, keyed by the cell itself (+1 by the double).
        let interior = e.mem.alloc(8);
        e.mem.set_u8(interior, 1);
        let rooms = map_of(&mut e, &[(interior + 1, 0xdddd)]);
        place_map(&mut e, this + 0x2c, &[(0, rooms)]);
        assert_eq!(e.call(0x006b_77e0, &args![this, interior]).u32(), 0xdddd);
    }

    fn filing(e: &mut Engine) {
        lookups(e);
        e.register(INFO_PARENT_00690830, |_, _| Ret::default());
        e.register(RT_DYNAMIC_CAST, |_, _| Ret::default());
        e.register(LOOKUP_FORM_004839C0, |_, a| a[0].into_ret());
        e.register(MAP_SET_AT_00844700, |_, _| Ret::default());
    }

    #[test]
    fn at_006b7980_creates_missing_maps_and_appends_once() {
        let mut e = Engine::new();
        filing(&mut e);
        e.register(WORLD_FLAG_005861B0, |_, _| Ret::default());
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(BUCKET_ALLOCATE_00AA1070, |e, a| {
            e.mem.alloc(a[0]).into_ret()
        });
        e.register(FILL_00403D30, |_, _| Ret::default());
        e.register(ARRAY_CTOR_0069AEE0, |_, a| a[0].into_ret());
        e.register(ARRAY_FIND_00719B20, |_, _| 0xffff_ffffu32.into_ret());
        e.register(ARRAY_ADD_007CB2E0, |e, a| {
            let info = e.mem.u32(a[1]);
            e.mem.set_u32(a[0] + 4, info);
            Ret::default()
        });
        let this = e.mem.alloc(0x40);
        place_map(&mut e, this + 0x2c, &[]);
        let info = e.mem.alloc(0x20);
        e.mem.set_u32(info + 4, 0x0100_0001);
        e.mem.set_u32(info + 0xc, 0x42);
        e.call_log = Some(vec![]);
        e.call(0x006b_7980, &args![this, info]);
        let log = e.call_log.take().unwrap();
        let sets = calls_to(&log, MAP_SET_AT_00844700);
        assert_eq!(sets.len(), 2);
        assert_eq!(sets[0][..2], [this + 0x2c, 0x0100_0001]);
        let cell_map = sets[0][2];
        // The worldspace's byte is clear: the larger cell map.
        assert_eq!(e.mem.u32(cell_map), CELL_MAP_VTABLE);
        assert_eq!(e.mem.u32(cell_map + 4), 0x3f1);
        assert_eq!(sets[1][..2], [cell_map, 0x43]);
        let array = sets[1][2];
        assert_eq!(e.mem.u32(array + 4), info);
        let find = &calls_to(&log, ARRAY_FIND_00719B20)[0];
        assert_eq!(find[0], array);
        assert_eq!(find[2..], [0, COMPARE_009A3830]);

        // Both maps present and the info already filed: nothing new.
        let filed = map_of(&mut e, &[(0x43, array)]);
        place_map(&mut e, this + 0x2c, &[(0x0100_0001, filed)]);
        e.register(ARRAY_FIND_00719B20, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x006b_7980, &args![this, info]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, MAP_SET_AT_00844700).is_empty());
        assert!(calls_to(&log, ARRAY_ADD_007CB2E0).is_empty());
    }

    #[test]
    fn at_006b7980_stores_null_maps_when_allocation_fails() {
        let mut e = Engine::new();
        filing(&mut e);
        e.register(WORLD_FLAG_005861B0, |_, _| 1u32.into_ret());
        e.register(ALLOCATE, |_, _| Ret::default());
        e.register(ARRAY_FIND_00719B20, |_, _| Ret::default());
        // Nothing is ever found (the null cell map is not read).
        e.register(MAP_GET_AT_00853130, |_, _| Ret::default());
        let this = e.mem.alloc(0x40);
        let info = e.mem.alloc(0x20);
        e.mem.set_u32(info + 4, 0x0100_0001);
        e.call_log = Some(vec![]);
        e.call(0x006b_7980, &args![this, info]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, MAP_SET_AT_00844700),
            vec![vec![this + 0x2c, 0x0100_0001, 0], vec![0, 1, 0]]
        );
        assert_eq!(calls_to(&log, WORLD_FLAG_005861B0), vec![vec![0x0100_0001]]);
    }

    #[test]
    fn at_006b7cb0_checks_each_array_count() {
        let mut e = Engine::new();
        walkable(&mut e);
        // An array: size at +0, element words from +4.
        e.register(ARRAY_SIZE_0044DDC0, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(ARRAY_AT_006A7AD0, |_, a| (a[0] + 4 + 4 * a[1]).into_ret());
        e.register(INFO_VALUE_006286D0, |e, a| e.mem.u32(a[0]).into_ret());
        let array = |e: &mut Engine, flags: &[u32]| {
            let array = e.mem.alloc(4 + 4 * flags.len() as u32);
            e.mem.set_u32(array, flags.len() as u32);
            for (i, flag) in flags.iter().enumerate() {
                let info = e.mem.alloc(4);
                e.mem.set_u32(info, *flag);
                e.mem.set_u32(array + 4 + 4 * i as u32, info);
            }
            array
        };
        let good = array(&mut e, &[1, 0, 1]);
        let single = array(&mut e, &[0]);
        let cells = map_of(&mut e, &[(1, good), (2, single)]);
        let this = e.mem.alloc(0x40);
        place_map(&mut e, this + 0x2c, &[(7, cells)]);
        assert!(e.call(0x006b_7cb0, &args![this]).bool());
        let bad = array(&mut e, &[1, 0, 0]);
        let cells = map_of(&mut e, &[(1, good), (2, bad)]);
        place_map(&mut e, this + 0x2c, &[(7, cells)]);
        assert!(!e.call(0x006b_7cb0, &args![this]).bool());
    }

    #[test]
    fn at_006b68b0_runs_the_form_step_then_each_info() {
        let mut e = Engine::new();
        walkable(&mut e);
        e.register(FORM_STEP_00484730, |_, _| Ret::default());
        e.register(INFO_STEP_004534F0, |_, _| Ret::default());
        let this = e.mem.alloc(0x40);
        place_map(&mut e, this + 0x1c, &[(1, 0x1111), (2, 0x2222)]);
        e.call_log = Some(vec![]);
        e.call(0x006b_68b0, &args![this, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, FORM_STEP_00484730), vec![vec![this, 0]]);
        assert_eq!(
            calls_to(&log, INFO_STEP_004534F0),
            vec![vec![0x1111, 0], vec![0x2222, 0]]
        );
        e.call_log = Some(vec![]);
        e.call(0x006b_68b0, &args![this, 0x101u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, FORM_STEP_00484730), vec![vec![this, 1]]);
        assert!(calls_to(&log, INFO_STEP_004534F0).is_empty());
    }

    #[test]
    fn at_006b6b20_deletes_everything() {
        let mut e = Engine::new();
        walkable(&mut e);
        e.register(NAV_MESH_INFO_DELETE, |_, _| Ret::default());
        e.register(MAP_REMOVE_ALL_00438AF0, |_, _| Ret::default());
        // Deletable objects: slot 0 is the deleting destructor; the cell
        // map's slot +4 is also its bucket-of-key.
        e.put_vtable(0x0f03_2000, &[0x0f03_3000, BUCKET_OF_KEY]);
        e.register(0x0f03_3000, |_, _| Ret::default());
        let mut slots = vec![0u32; 0x33];
        slots[0x32] = 0x0f03_4000;
        e.put_vtable(0x0f03_5000, &slots);
        e.register(0x0f03_4000, |_, _| Ret::default());
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array, 0x0f03_2000);
        let cells = map_of(&mut e, &[(1, array), (2, 0)]);
        e.mem.set_u32(cells, 0x0f03_2000);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this, 0x0f03_5000);
        place_map(&mut e, this + 0x1c, &[(1, 0x1111), (2, 0)]);
        place_map(&mut e, this + 0x2c, &[(9, cells)]);
        e.call_log = Some(vec![]);
        e.call(0x006b_6b20, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, NAV_MESH_INFO_DELETE), vec![vec![0x1111, 1]]);
        assert_eq!(
            calls_to(&log, MAP_REMOVE_ALL_00438AF0),
            vec![vec![this + 0x1c], vec![this + 0x2c]]
        );
        assert_eq!(
            calls_to(&log, 0x0f03_3000),
            vec![vec![array, 1], vec![cells, 1]]
        );
        assert_eq!(calls_to(&log, 0x0f03_4000), vec![vec![this, 1]]);
    }

    #[test]
    fn at_006b63c0_files_good_infos_and_drops_bad_ones() {
        let mut e = Engine::new();
        walkable(&mut e);
        filing(&mut e);
        e.map(0x0106_a000, 0x1000);
        e.set_global::<f64>(LOCATION_LIMIT, 100.0);
        e.register(ARRAY_CTOR_0069AEE0, |_, _| Ret::default());
        e.register(ARRAY_DTOR_0069AF10, |_, _| Ret::default());
        // Parent forms exist for ids below 0x1000.
        e.register(LOOKUP_FORM_004839C0, |_, a| {
            let form = if a[0] < 0x1000 { a[0] + 0x10_0000 } else { 0 };
            form.into_ret()
        });
        e.register(PRINT_WARNING_005B5E40, |_, _| Ret::default());
        e.register(FLOAT_OF_00408840, |_, a| {
            f32::from_bits(a[0]).abs().into_ret()
        });
        // Arrays: size at +0, items from +4.
        e.register(ARRAY_ADD_007CB2E0, |e, a| {
            let size = e.mem.u32(a[0]);
            let info = e.mem.u32(a[1]);
            e.mem.set_u32(a[0] + 4 + 4 * size, info);
            e.mem.set_u32(a[0], size + 1);
            Ret::default()
        });
        e.register(ARRAY_SIZE_0044DDC0, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(ARRAY_AT_00877A30, |_, a| (a[0] + 4 + 4 * a[1]).into_ret());
        e.register(FIRST_OF_00559450, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(MAP_REMOVE_AT_00405430, |_, _| Ret::default());
        e.register(NAV_MESH_INFO_DELETE, |_, _| Ret::default());
        e.register(INIT_DOOR_POINTERS, |_, _| Ret::default());
        // The good info is already filed.
        e.register(ARRAY_FIND_00719B20, |_, _| Ret::default());
        let info = |e: &mut Engine, id: u32, parent: u32, location: f32| {
            let info = e.mem.alloc(0x20);
            e.mem.set_u32(info, id);
            e.mem.set_u32(info + 4, parent);
            e.mem.set_u32(info + 0xc, 0x42);
            e.mem.set_f32(info + 0x18, location);
            info
        };
        let good = info(&mut e, 0xa, 0x0100, 5.0);
        let orphan = info(&mut e, 0xb, 0x2000, 5.0);
        let far = info(&mut e, 0xc, 0x0100, -500.0);
        let this = e.mem.alloc(0x40);
        place_map(
            &mut e,
            this + 0x1c,
            &[(0xa, good), (0xb, orphan), (0xc, far)],
        );
        // An empty array, so the closing check (`006b7cb0`) passes it.
        let array = e.mem.alloc(0x10);
        let filed = map_of(&mut e, &[(0x43, array)]);
        place_map(&mut e, this + 0x2c, &[(0x0100, filed)]);
        e.call_log = Some(vec![]);
        e.call(0x006b_63c0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(good + 0x1c), 0x0100 + 0x10_0000);
        assert_eq!(e.mem.u32(orphan + 0x1c), 0);
        assert_eq!(
            calls_to(&log, PRINT_WARNING_005B5E40),
            vec![
                vec![NO_PARENT_SPACE_FORMAT, 0x2000, 0xb],
                vec![BAD_LOCATION_FORMAT, 0xc],
            ]
        );
        assert_eq!(calls_to(&log, FLOAT_OF_00408840)[0], vec![5.0f32.to_bits()]);
        assert_eq!(
            calls_to(&log, MAP_REMOVE_AT_00405430),
            vec![vec![this + 0x1c, 0xb], vec![this + 0x1c, 0xc]]
        );
        assert_eq!(
            calls_to(&log, NAV_MESH_INFO_DELETE),
            vec![vec![orphan, 1], vec![far, 1]]
        );
        assert_eq!(calls_to(&log, INIT_DOOR_POINTERS).len(), 3);
        assert_eq!(e.mem.u8(this + 0x3c), 1);
        assert_eq!(calls_to(&log, ARRAY_DTOR_0069AF10).len(), 1);
        // With the byte set, no door pointers.
        place_map(&mut e, this + 0x1c, &[(0xa, good)]);
        e.call_log = Some(vec![]);
        e.call(0x006b_63c0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, INIT_DOOR_POINTERS).is_empty());
    }
}

#[cfg(test)]
mod map_edit_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Vec<(u32, Vec<u32>)>;
    type Words = Rc<RefCell<Vec<u32>>>;

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    const MAP_VTABLE: u32 = 0x0f04_0000;
    const BUCKET_OF_KEY: u32 = 0x0f04_1000;
    const FORM_VTABLE: u32 = 0x0f04_2000;
    const FINISH: u32 = 0x0f04_3000;
    const OBJECT_VTABLE: u32 = 0x0f04_4000;
    const OBJECT_DELETE: u32 = 0x0f04_5000;

    /// A one-bucket chained map holding `entries` (key, value) in order,
    /// written at `at` (four words).
    fn place_map(e: &mut Engine, at: u32, entries: &[(u32, u32)]) {
        let table = e.mem.alloc(4);
        e.mem.set_u32(at, MAP_VTABLE);
        e.mem.set_u32(at + 4, 1);
        e.mem.set_u32(at + 8, table);
        e.mem.set_u32(at + 0xc, entries.len() as u32);
        let mut next = 0;
        for (key, value) in entries.iter().rev() {
            let entry = e.mem.alloc(0xc);
            e.mem.set_u32(entry, next);
            e.mem.set_u32(entry + 4, *key);
            e.mem.set_u32(entry + 8, *value);
            next = entry;
        }
        e.mem.set_u32(table, next);
    }

    /// An array (buffer +4, size +8) holding `words`, written at `at`.
    fn place_array(e: &mut Engine, at: u32, words: &[u32]) {
        let buffer = e.mem.alloc(0x100);
        for (i, w) in words.iter().enumerate() {
            e.mem.set_u32(buffer + 4 * i as u32, *w);
        }
        e.mem.set_u32(at + 4, buffer);
        e.mem.set_u32(at + 8, words.len() as u32);
    }

    fn array_words(e: &Engine, at: u32) -> Vec<u32> {
        let buffer = e.mem.u32(at + 4);
        (0..e.mem.u32(at + 8))
            .map(|i| e.mem.u32(buffer + 4 * i))
            .collect()
    }

    /// The map, array and form doubles every test here uses: maps made by
    /// `place_map` (lookups, first entry, empty test), arrays made by
    /// `place_array` (size, element, add, find, remove, construct),
    /// `00559450` as the word at +0, a recording `RemoveAt`/`SetAt`, the
    /// form id word at +0xC, and the map form `this` (0x50 bytes) whose slot
    /// +0xC8 is `FINISH`.
    fn world(e: &mut Engine) -> u32 {
        e.put_vtable(MAP_VTABLE, &[0, BUCKET_OF_KEY]);
        e.register(BUCKET_OF_KEY, |_, _| Ret::default());
        let mut form_slots = vec![0u32; 0x34];
        form_slots[0xc8 / 4] = FINISH;
        e.put_vtable(FORM_VTABLE, &form_slots);
        e.register(FINISH, |_, _| Ret::default());
        e.put_vtable(OBJECT_VTABLE, &[OBJECT_DELETE]);
        e.register(OBJECT_DELETE, |_, _| Ret::default());
        e.register(MAP_FIRST_004B9BA0, |e, a| {
            let buckets = e.mem.u32(a[0] + 8);
            e.mem.u32(buckets).into_ret()
        });
        e.register(MAP_GET_AT_00853130, |e, a| {
            if e.mem.u32(a[0]) != MAP_VTABLE {
                return Ret::default();
            }
            let buckets = e.mem.u32(a[0] + 8);
            let mut entry = e.mem.u32(buckets);
            while entry != 0 {
                if e.mem.u32(entry + 4) == a[1] {
                    let value = e.mem.u32(entry + 8);
                    e.mem.set_u32(a[2], value);
                    return 1u32.into_ret();
                }
                entry = e.mem.u32(entry);
            }
            Ret::default()
        });
        e.register(MAP_IS_EMPTY_0069B080, |e, a| {
            (e.mem.u32(a[0] + 0xc) == 0).into_ret()
        });
        e.register(MAP_REMOVE_AT_00405430, |_, _| Ret::default());
        e.register(MAP_SET_AT_00844700, |_, _| Ret::default());
        e.register(ARRAY_SIZE_0044DDC0, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(ARRAY_AT_00877A30, |e, a| {
            (e.mem.u32(a[0] + 4) + 4 * a[1]).into_ret()
        });
        e.register(ARRAY_AT_006A7AD0, |e, a| {
            (e.mem.u32(a[0] + 4) + 4 * a[1]).into_ret()
        });
        e.register(ARRAY_CTOR_0069AEE0, |e, a| {
            place_array(e, a[0], &[]);
            a[0].into_ret()
        });
        e.register(ARRAY_DTOR_0069AF10, |_, _| Ret::default());
        e.register(ARRAY_ADD_007CB2E0, |e, a| {
            let (buffer, size) = (e.mem.u32(a[0] + 4), e.mem.u32(a[0] + 8));
            let word = e.mem.u32(a[1]);
            e.mem.set_u32(buffer + 4 * size, word);
            e.mem.set_u32(a[0] + 8, size + 1);
            Ret::default()
        });
        e.register(ARRAY_FIND_00719B20, |e, a| {
            let wanted = e.mem.u32(a[1]);
            let words = array_words(e, a[0]);
            match words.iter().position(|w| *w == wanted) {
                Some(i) => (i as u32).into_ret(),
                None => 0xffff_ffffu32.into_ret(),
            }
        });
        e.register(ARRAY_REMOVE_006BF8F0, |e, a| {
            let mut words = array_words(e, a[0]);
            words.remove(a[1] as usize);
            place_array(e, a[0], &words);
            Ret::default()
        });
        e.register(ARRAY_IS_EMPTY_0076B610, |e, a| {
            (e.mem.u32(a[0] + 8) == 0).into_ret()
        });
        e.register(FIRST_OF_00559450, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(FORM_ID_0084E3A0, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register(NAV_MESH_INFO_DELETE, |_, _| Ret::default());
        let this = e.mem.alloc(0x50);
        e.mem.set_u32(this, FORM_VTABLE);
        place_map(e, this + 0x1c, &[]);
        place_map(e, this + 0x2c, &[]);
        this
    }

    /// A `NavMeshInfo` (0x5C bytes) with its id, empty arrays at +0x24,
    /// +0x34 and +0x44, and `parent` at +0x1C (what the `00690830` double
    /// returns).
    fn info(e: &mut Engine, id: u32, parent: u32) -> u32 {
        let info = e.mem.alloc(0x5c);
        e.mem.set_u32(info, id);
        e.mem.set_u32(info + 0x1c, parent);
        for at in [0x24, 0x34, 0x44] {
            place_array(e, info + at, &[]);
        }
        info
    }

    /// The cast doubles that `006b7b30`/`006b77b0` need (no cast object) and
    /// a key of `00464f30` one above its argument.
    fn casts(e: &mut Engine) {
        e.register(INFO_PARENT_00690830, |_, _| Ret::default());
        e.register(RT_DYNAMIC_CAST, |_, _| Ret::default());
        e.register(KEY_OF_00464F30, |_, a| (a[0] + 1).into_ret());
    }

    /// Records the word at the second argument of each call to `address`.
    fn record_second_word(e: &mut Engine, address: u32) -> Words {
        let seen: Words = Rc::new(RefCell::new(vec![]));
        let sink = seen.clone();
        e.register_double(address, move |e, a| {
            sink.borrow_mut().push(e.mem.u32(a[1]));
            Ret::default()
        });
        seen
    }

    /// The navmesh pointer double: the info's +0x54 into the pointer.
    fn nav_mesh_pointer(e: &mut Engine) {
        e.register(INFO_NAV_MESH_00690860, |e, a| {
            let mesh = e.mem.u32(a[0] + 0x54);
            e.mem.set_u32(a[1], mesh);
            (mesh != 0).into_ret()
        });
        e.register(NI_POINTER_CONSTRUCT_0042FB00, |_, _| Ret::default());
        e.register(NI_POINTER_DESTRUCT_0042FA40, |_, _| Ret::default());
    }

    #[test]
    fn at_006b6910_type_0x43_is_one_else_the_comparison() {
        let mut e = Engine::new();
        e.register(FORM_TYPE_00401170, |e, a| {
            u32::from(e.mem.u8(a[0] + 4)).into_ret()
        });
        e.register(FORM_COMPARE_00484020, |_, _| 0x1234_5600u32.into_ret());
        let form = e.mem.alloc(8);
        e.mem.set_u8(form + 4, 0x43);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006b_6910, &args![0x5000u32, form]).u32(), 1);
        assert!(calls_to(&e.call_log.take().unwrap(), FORM_COMPARE_00484020).is_empty());
        e.mem.set_u8(form + 4, 0x42);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x006b_6910, &args![0x5000u32, form]).u32(),
            0x1234_5600
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, FORM_COMPARE_00484020),
            vec![vec![0x5000, form]]
        );
    }

    #[test]
    fn at_006b6940_files_the_info_in_both_maps_then_finishes() {
        let mut e = Engine::new();
        let this = world(&mut e);
        casts(&mut e);
        let info = info(&mut e, 0x77, 0);
        e.mem.set_u32(info + 4, 0x20);
        e.mem.set_u32(info + 0xc, 0x30);
        // The worldspace 0x20 already has a cell map with the array of key 0x31.
        let inner = e.mem.alloc(0x10);
        let array = e.mem.alloc(0x10);
        place_array(&mut e, array, &[]);
        place_map(&mut e, inner, &[(0x31, array)]);
        place_map(&mut e, this + 0x2c, &[(0x20, inner)]);
        e.call_log = Some(vec![]);
        e.call(0x006b_6940, &args![this, info]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, MAP_SET_AT_00844700),
            vec![vec![this + 0x1c, 0x77, info]]
        );
        assert_eq!(array_words(&e, array), vec![info]);
        assert_eq!(calls_to(&log, FINISH), vec![vec![this, 1]]);
    }

    #[test]
    fn at_006b6990_removes_a_known_info_and_unfiles_it() {
        let mut e = Engine::new();
        let this = world(&mut e);
        casts(&mut e);
        let info = info(&mut e, 0x77, 0);
        place_map(&mut e, this + 0x1c, &[(0x77, info)]);
        // Unknown id: nothing but the lookup.
        e.call_log = Some(vec![]);
        e.call(0x006b_6990, &args![this, 0x78u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
        // Empty worldspace map: no unfiling.
        e.call_log = Some(vec![]);
        e.call(0x006b_6990, &args![this, 0x77u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, MAP_REMOVE_AT_00405430),
            vec![vec![this + 0x1c, 0x77]]
        );
        assert!(calls_to(&log, INFO_PARENT_00690830).is_empty());
        assert_eq!(calls_to(&log, FINISH), vec![vec![this, 1]]);
        // A worldspace map with an entry: `006b7b70` runs (it asks for the cast).
        let inner = e.mem.alloc(0x10);
        place_map(&mut e, inner, &[]);
        place_map(&mut e, this + 0x2c, &[(0x99, inner)]);
        e.call_log = Some(vec![]);
        e.call(0x006b_6990, &args![this, 0x77u32]);
        let log = e.call_log.take().unwrap();
        assert!(!calls_to(&log, INFO_PARENT_00690830).is_empty());
        assert_eq!(calls_to(&log, FINISH), vec![vec![this, 1]]);
        assert!(calls_to(&log, NAV_MESH_INFO_DELETE).is_empty());
    }

    #[test]
    fn at_006b6a00_deletes_the_info_and_finishes_for_a_master_file() {
        for master in [true, false] {
            let mut e = Engine::new();
            let this = world(&mut e);
            casts(&mut e);
            nav_mesh_pointer(&mut e);
            e.register(FORM_GET_FILE, |_, _| 0x6000u32.into_ret());
            if master {
                e.register(FILE_GET_MASTER, |_, _| 1u32.into_ret());
            } else {
                e.register(FILE_GET_MASTER, |_, _| Ret::default());
            }
            let info = info(&mut e, 0x77, 0);
            e.mem.set_u32(info + 0x54, 0x7000);
            place_map(&mut e, this + 0x1c, &[(0x77, info)]);
            e.call_log = Some(vec![]);
            e.call(0x006b_6a00, &args![this, 0x77u32]);
            let log = e.call_log.take().unwrap();
            assert_eq!(calls_to(&log, FORM_GET_FILE), vec![vec![0x7000, 0]]);
            assert_eq!(calls_to(&log, NAV_MESH_INFO_DELETE), vec![vec![info, 1]]);
            assert_eq!(
                calls_to(&log, MAP_REMOVE_AT_00405430),
                vec![vec![this + 0x1c, 0x77]]
            );
            assert_eq!(calls_to(&log, FINISH).len(), usize::from(master));
            let constructed = calls_to(&log, NI_POINTER_CONSTRUCT_0042FB00);
            assert_eq!(constructed.len(), 1);
            assert_eq!(calls_to(&log, NI_POINTER_DESTRUCT_0042FA40), constructed);
        }
    }

    #[test]
    fn at_006b7770_the_cell_from_the_worldspace_or_the_cast() {
        let mut e = Engine::new();
        e.register(INFO_WORLD_SPACE_00690800, |e, a| {
            e.mem.u32(a[0] + 0x20).into_ret()
        });
        e.register(WORLD_SPACE_GET_CELL_FROM_KEY, |_, a| {
            (a[0] + a[1]).into_ret()
        });
        e.register(INFO_PARENT_00690830, |_, _| 0x8000u32.into_ret());
        e.register(RT_DYNAMIC_CAST, |_, a| (a[0] + 1).into_ret());
        let info = e.mem.alloc(0x5c);
        e.mem.set_u32(info + 0xc, 0x12);
        e.mem.set_u32(info + 0x20, 0x100);
        assert_eq!(e.call(0x006b_7770, &args![info]).u32(), 0x112);
        e.mem.set_u32(info + 0x20, 0);
        assert_eq!(e.call(0x006b_7770, &args![info]).u32(), 0x8001);
    }

    #[test]
    fn at_006b7910_lists_every_info() {
        let mut e = Engine::new();
        let this = world(&mut e);
        e.register(ARRAY_SET_SIZE_006F2290, |_, _| Ret::default());
        e.register(ARRAY_SET_RESERVED_SIZE_0084B5C0, |_, _| Ret::default());
        let added = record_second_word(&mut e, ARRAY_ADD_007CB2E0);
        place_map(&mut e, this + 0x1c, &[(1, 0xa1), (2, 0xa2), (3, 0xa3)]);
        let out = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x006b_7910, &args![this, out]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, ARRAY_SET_SIZE_006F2290),
            vec![vec![out, 0, 0]]
        );
        assert_eq!(
            calls_to(&log, ARRAY_SET_RESERVED_SIZE_0084B5C0),
            vec![vec![out, 0x800]]
        );
        assert_eq!(*added.borrow(), vec![0xa1, 0xa2, 0xa3]);
    }

    #[test]
    fn at_006b7b70_removes_the_info_and_empty_containers() {
        let mut e = Engine::new();
        let this = world(&mut e);
        casts(&mut e);
        let info = info(&mut e, 0x77, 0);
        e.mem.set_u32(info + 4, 0x20);
        e.mem.set_u32(info + 0xc, 0x30);
        let other = 0x4444;
        let array = e.mem.alloc(0x10);
        let inner = e.mem.alloc(0x10);
        place_map(&mut e, inner, &[(0x31, array)]);
        place_map(&mut e, this + 0x2c, &[(0x20, inner)]);
        // Another info stays: only the entry goes.
        place_array(&mut e, array, &[other, info]);
        e.call_log = Some(vec![]);
        e.call(0x006b_7b70, &args![this, info]);
        let log = e.call_log.take().unwrap();
        assert_eq!(array_words(&e, array), vec![other]);
        assert!(calls_to(&log, MAP_REMOVE_AT_00405430).is_empty());
        // Not in the array: nothing removed.
        e.call_log = Some(vec![]);
        e.call(0x006b_7b70, &args![this, info]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, ARRAY_REMOVE_006BF8F0).is_empty());
        // The last info: the array and then the emptied cell map go (both
        // deleted through their vtable slot 0).
        place_array(&mut e, array, &[info]);
        e.mem.set_u32(array, OBJECT_VTABLE);
        e.mem.set_u32(inner, OBJECT_VTABLE);
        e.mem.set_u32(inner + 0xc, 0);
        e.register_double(MAP_GET_AT_00853130, move |e, a| {
            let value = match (a[0], a[1]) {
                (m, 0x20) if m == this + 0x2c => inner,
                (m, 0x31) if m == inner => array,
                _ => return Ret::default(),
            };
            e.mem.set_u32(a[2], value);
            1u32.into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x006b_7b70, &args![this, info]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, MAP_REMOVE_AT_00405430),
            vec![vec![inner, 0x31], vec![this + 0x2c, 0x20]]
        );
        assert_eq!(
            calls_to(&log, OBJECT_DELETE),
            vec![vec![array, 1], vec![inner, 1]]
        );
    }

    /// The doubles `CheckMap` needs beyond `world`: navmeshes (flags at +8,
    /// door count +0x10, door entries +0x14, no triangles), infos (parent
    /// +0x1C, worldspace +0x20, navmesh +0x54) and recorded warnings.
    fn check_map_world(e: &mut Engine) -> u32 {
        let this = world(e);
        nav_mesh_pointer(e);
        for callee in [
            FIX_PORTALS_FROM_NAV_MESH_DELETION,
            INFO_REBUILD_006B5560,
            TRIANGLE_ARRAY_CONSTRUCT_00694DE0,
            TRIANGLE_ARRAY_DESTRUCT_00694E10,
            FIND_PORTAL_TRIANGLES,
            PRINT_WARNING_005B5E40,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.register(RT_DYNAMIC_CAST, |_, _| Ret::default());
        e.register(INFO_PARENT_00690830, |e, a| {
            e.mem.u32(a[0] + 0x1c).into_ret()
        });
        e.register(INFO_WORLD_SPACE_00690800, |e, a| {
            e.mem.u32(a[0] + 0x20).into_ret()
        });
        e.register(INFO_HAS_NAV_MESH_0069DFB0, |e, a| {
            (e.mem.u32(a[0] + 0x54) != 0).into_ret()
        });
        e.register(FORM_FLAG_20_00440D80, |e, a| {
            (e.mem.u32(a[0] + 8) & 0x20 != 0).into_ret()
        });
        e.register(INFO_OF_ID_00693D70, |_, _| 1u32.into_ret());
        e.register(TRIANGLE_COUNT_00695370, |_, _| Ret::default());
        e.register(DOOR_COUNT_006B5A90, |e, a| {
            e.mem.u32(a[0] + 0x10).into_ret()
        });
        e.register(DOOR_AT_0068F6B0, |e, a| {
            (e.mem.u32(a[0] + 0x14) + 4 * a[1]).into_ret()
        });
        e.register(SPLIT_CELL_KEY_00587520, |e, a| {
            e.mem.set_u16(a[1], (a[0] >> 16) as u16);
            e.mem.set_u16(a[2], a[0] as u16);
            Ret::default()
        });
        e.register(CELL_KEY_00587410, |_, a| ((a[0] << 16) | a[1]).into_ret());
        this
    }

    #[test]
    fn at_006b6c60_reports_removes_and_rebuilds() {
        let mut e = Engine::new();
        let this = check_map_world(&mut e);
        // A navmesh with one door (its entry holds the door, id 0xD00).
        let door = e.mem.alloc(0x10);
        e.mem.set_u32(door + 0xc, 0xd00);
        let door_entries = e.mem.alloc(4);
        e.mem.set_u32(door_entries, door);
        let world_space = e.mem.alloc(0x10);
        let mesh = e.mem.alloc(0x18);
        e.mem.set_u32(mesh + 0x10, 1);
        e.mem.set_u32(mesh + 0x14, door_entries);
        // An orphan, a healthy info with the door (and a worldspace at cell
        // (5, -2)) and one whose door list lacks it.
        let orphan = info(&mut e, 0x11, 0);
        let healthy = info(&mut e, 0x22, 0x900);
        e.mem.set_u32(healthy + 0x54, mesh);
        e.mem.set_u32(healthy + 0x20, world_space);
        e.mem.set_u32(healthy + 0xc, 0x0005_fffe);
        place_array(&mut e, healthy + 0x44, &[door]);
        let stale = info(&mut e, 0x33, 0x900);
        e.mem.set_u32(stale + 0x54, mesh);
        place_map(
            &mut e,
            this + 0x1c,
            &[(0x11, orphan), (0x22, healthy), (0x33, stale)],
        );
        e.call_log = Some(vec![]);
        e.call(0x006b_6c60, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, PRINT_WARNING_005B5E40),
            vec![
                vec![NO_PARENT_SPACE_WARNING, 0x11, EMPTY_STRING],
                vec![MISSING_DOOR_PORTAL_WARNING, 0x33, 0xd00],
            ]
        );
        assert_eq!(
            calls_to(&log, INFO_REBUILD_006B5560),
            vec![vec![stale, mesh]]
        );
        // The eight cells around (5, -2) of the healthy info's worldspace.
        let cells: Vec<u32> = calls_to(&log, CELL_KEY_00587410)
            .iter()
            .map(|a| (a[0] << 16) | a[1])
            .collect();
        let mut want = vec![];
        for x in 4..=6i32 {
            for y in -3..=-1i32 {
                if (x, y) != (5, -2) {
                    want.push(((x as u32 & 0xffff) << 16) | (y as u32 & 0xffff));
                }
            }
        }
        assert_eq!(cells, want);
        // Only the orphan is collected: removed from the map and deleted.
        assert_eq!(
            calls_to(&log, MAP_REMOVE_AT_00405430),
            vec![vec![this + 0x1c, 0x11]]
        );
        assert_eq!(calls_to(&log, NAV_MESH_INFO_DELETE), vec![vec![orphan, 1]]);
        // Every NiPointer constructed is destroyed.
        assert_eq!(
            calls_to(&log, NI_POINTER_CONSTRUCT_0042FB00).len(),
            calls_to(&log, NI_POINTER_DESTRUCT_0042FA40).len()
        );
    }

    #[test]
    fn at_006b6c60_a_dangling_portal_is_removed() {
        let mut e = Engine::new();
        let this = check_map_world(&mut e);
        e.register(INFO_OF_ID_00693D70, |_, a| {
            u32::from(a[1] != 0x66).into_ret()
        });
        let mesh = e.mem.alloc(0x18);
        let known = info(&mut e, 0x55, 0x900);
        let gone = info(&mut e, 0x66, 0x900);
        let owner = info(&mut e, 0x44, 0x900);
        e.mem.set_u32(owner + 0x54, mesh);
        place_array(&mut e, owner + 0x24, &[gone, known, 0]);
        place_map(&mut e, this + 0x1c, &[(0x44, owner)]);
        e.call(0x006b_6c60, &args![this]);
        assert_eq!(array_words(&e, owner + 0x24), vec![known]);
    }

    #[test]
    fn at_006b6c60_a_missing_navmesh_fixes_portals_and_collects() {
        let mut e = Engine::new();
        let this = check_map_world(&mut e);
        // The cell of the info (the cast object) names itself through slot +0x130.
        let mut cell_slots = vec![0u32; 0x4d];
        cell_slots[(SLOT_CELL_NAME / 4) as usize] = 0x0f04_6000;
        e.put_vtable(0x0f04_7000, &cell_slots);
        let cell = e.mem.alloc(4);
        e.mem.set_u32(cell, 0x0f04_7000);
        e.register_double(RT_DYNAMIC_CAST, move |_, _| cell.into_ret());
        e.register(0x0f04_6000, |_, _| 0x5400u32.into_ret());
        let lonely = info(&mut e, 0x44, 0x900);
        place_map(&mut e, this + 0x1c, &[(0x44, lonely)]);
        e.call_log = Some(vec![]);
        e.call(0x006b_6c60, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, PRINT_WARNING_005B5E40),
            vec![vec![NO_NAV_MESH_WARNING, 0x44, 0x5400]]
        );
        assert_eq!(
            calls_to(&log, FIX_PORTALS_FROM_NAV_MESH_DELETION),
            vec![vec![lonely]]
        );
        assert_eq!(calls_to(&log, NAV_MESH_INFO_DELETE), vec![vec![lonely, 1]]);
    }

    #[test]
    fn at_006b6580_counts_type_0x43_then_walks_and_checks() {
        let mut e = Engine::new();
        let this = world(&mut e);
        nav_mesh_pointer(&mut e);
        e.register(FORM_FLAG_2_00460340, |e, a| {
            (e.mem.u32(a[0] + 8) & 2 != 0).into_ret()
        });
        e.register(FORM_FLAG_20_00440D80, |e, a| {
            (e.mem.u32(a[0] + 8) & 0x20 != 0).into_ret()
        });
        e.register(FORM_TYPE_00401170, |e, a| {
            u32::from(e.mem.u8(a[0] + 4)).into_ret()
        });
        e.register(ALWAYS_ZERO_0047C850, |_, _| Ret::default());
        e.register(NAV_MESH_STEP_006986D0, |_, _| Ret::default());
        // The list object at 011c54c4: one form of type 0x42.
        e.map(0x011c_5000, 0x1000);
        let list = e.mem.alloc(0x10);
        let form = e.mem.alloc(8);
        e.mem.set_u8(form + 4, 0x42);
        place_array(&mut e, list, &[form]);
        e.mem.set_u32(list + 0xc, 1);
        e.set_global(FORM_LIST_011C54C4, list);
        // Flag 2 clear: nothing.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006b_6580, &args![this]).u8(), 1);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
        // No type 0x43 and no info that asks: the finish slot with 0.
        e.mem.set_u32(this + 8, 2);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006b_6580, &args![this]).u8(), 1);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, FINISH), vec![vec![this, 0]]);
        assert!(calls_to(&log, ARRAY_CTOR_0069AEE0).is_empty());
        // A type-0x43 form: the navmesh step for the meshes with flag 2
        // and without 0x20, then `CheckMap` (its local array).
        e.mem.set_u8(form + 4, 0x43);
        let (good, deleted) = (e.mem.alloc(0x10), e.mem.alloc(0x10));
        e.mem.set_u32(good + 8, 2);
        e.mem.set_u32(deleted + 8, 0x22);
        let first = info(&mut e, 1, 0x900);
        e.mem.set_u32(first + 0x54, good);
        let second = info(&mut e, 2, 0x900);
        e.mem.set_u32(second + 0x54, deleted);
        place_map(&mut e, this + 0x1c, &[(1, first), (2, second)]);
        // `CheckMap`'s doubles on these healthy infos.
        e.register(INFO_PARENT_00690830, |_, _| 0x900u32.into_ret());
        e.register(INFO_HAS_NAV_MESH_0069DFB0, |_, _| 1u32.into_ret());
        e.register(INFO_WORLD_SPACE_00690800, |_, _| Ret::default());
        e.register(TRIANGLE_COUNT_00695370, |_, _| Ret::default());
        e.register(DOOR_COUNT_006B5A90, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006b_6580, &args![this]).u8(), 1);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, NAV_MESH_STEP_006986D0), vec![vec![good]]);
        // `CheckMap` collects the info whose navmesh has flag 0x20: its
        // removal (`006b6990`) finishes with 1, and it is deleted.
        assert_eq!(calls_to(&log, FINISH), vec![vec![this, 1]]);
        assert_eq!(calls_to(&log, NAV_MESH_INFO_DELETE), vec![vec![second, 1]]);
        // `CheckMap`: its bad-info array plus one neighbour array per info.
        assert_eq!(calls_to(&log, ARRAY_CTOR_0069AEE0).len(), 3);
    }

    #[test]
    fn at_006b6740_writes_the_three_chunk_kinds() {
        for swap in [false, true] {
            let mut e = Engine::new();
            let this = world(&mut e);
            e.register(FORM_START_FORM, |_, _| Ret::default());
            e.register(FORM_CLOSE_FORM, |_, _| Ret::default());
            e.register(FORM_ADD_WORD_DATA_00485910, |_, _| Ret::default());
            e.register(INFO_DATA_SIZE_006B4880, |_, _| 0x20u32.into_ret());
            e.register(INFO_WRITE_DATA_006B48F0, |_, _| Ret::default());
            e.register(INFO_CONNECTIONS_SIZE_006B4E10, |_, _| 0xcu32.into_ret());
            e.register(INFO_WRITE_CONNECTIONS_006B4E80, |_, _| Ret::default());
            e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
            e.register(DEALLOCATE, |_, _| Ret::default());
            if swap {
                e.register(SWAP_BYTES_00401500, |_, _| 1u32.into_ret());
            } else {
                e.register(SWAP_BYTES_00401500, |_, _| Ret::default());
            }
            e.register(BYTE_SWAP_00401080, |e, a| {
                let word = e.mem.u32(a[0]).swap_bytes();
                e.mem.set_u32(a[0], word);
                Ret::default()
            });
            let chunks: Words = Rc::new(RefCell::new(vec![]));
            let sink = chunks.clone();
            e.register_double(FORM_ADD_CHUNK_DATA, move |e, a| {
                sink.borrow_mut().extend([a[0], a[2], e.mem.u32(a[1])]);
                Ret::default()
            });
            let info = info(&mut e, 0x1122_3344, 0);
            place_map(&mut e, this + 0x1c, &[(0x1122_3344, info)]);
            e.call_log = Some(vec![]);
            e.call(0x006b_6740, &args![this]);
            let log = e.call_log.take().unwrap();
            assert_eq!(
                calls_to(&log, FORM_ADD_WORD_DATA_00485910),
                vec![vec![CHUNK_NVER, 0xb]]
            );
            let id = if swap { 0x4433_2211 } else { 0x1122_3344 };
            assert_eq!(chunks.borrow()[..2], [CHUNK_NVMI, 0x20]);
            assert_eq!(chunks.borrow()[3..], [CHUNK_NVCI, 0x10, id]);
            assert_eq!(calls_to(&log, INFO_WRITE_CONNECTIONS_006B4E80).len(), 1);
            assert_eq!(calls_to(&log, DEALLOCATE).len(), 2);
            assert_eq!(calls_to(&log, FORM_CLOSE_FORM), vec![vec![this]]);
        }
    }

    #[test]
    fn at_006dfcb0_builds_the_item() {
        let mut e = Engine::new();
        e.register(ITEM_ARRAY_CONSTRUCT_006E1070, |_, _| Ret::default());
        e.register(ITEM_INIT_006DF200, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x006d_fcb0, &args![0x7000u32, 5u32, 6u32]).u32(),
            0x7000
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, ITEM_ARRAY_CONSTRUCT_006E1070),
            vec![vec![0x7004]]
        );
        assert_eq!(calls_to(&log, ITEM_INIT_006DF200), vec![vec![0x7000, 5, 6]]);
    }

    #[test]
    fn at_006dfba0_rebuilds_when_stale_or_resized() {
        let mut e = Engine::new();
        world(&mut e);
        e.register(NAV_MESH_COUNT_006911C0, |e, a| {
            e.mem.u32(a[0] + 0x10).into_ret()
        });
        e.register(NAV_MESH_VALUE_0068EFB0, |_, _| 0xabcu32.into_ret());
        e.register(ARRAY_RELEASE_008454F0, |e, a| {
            e.mem.set_u32(a[0] + 8, 0);
            Ret::default()
        });
        e.register(ARRAY_SET_RESERVED_SIZE_0084B5C0, |_, _| Ret::default());
        e.register(ITEM_ARRAY_CONSTRUCT_006E1070, |_, _| Ret::default());
        e.register(ITEM_INIT_006DF200, |e, a| {
            e.mem.set_u32(a[0] + 0x14, a[1]);
            e.mem.set_u32(a[0] + 0x18, a[2]);
            Ret::default()
        });
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        let mesh = e.mem.alloc(0x14);
        e.mem.set_u32(mesh + 0x10, 2);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, mesh);
        let this = e.mem.alloc(0x14);
        place_array(&mut e, this, &[]);
        e.call(0x006d_fba0, &args![this, holder]);
        let items = array_words(&e, this);
        assert_eq!(items.len(), 2);
        for (i, item) in items.iter().enumerate() {
            assert_eq!(e.mem.u32(item + 0x14), 0xabc);
            assert_eq!(e.mem.u32(item + 0x18), i as u32);
        }
        assert_eq!(e.mem.u32(this + 0x10), 0xabc);
        // Up to date (word set, same size): untouched.
        e.call_log = Some(vec![]);
        e.call(0x006d_fba0, &args![this, holder]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, ARRAY_RELEASE_008454F0).is_empty());
        // A failed allocation appends null.
        e.mem.set_u32(this + 0x10, 0);
        e.register(ALLOCATE, |_, _| Ret::default());
        e.call(0x006d_fba0, &args![this, holder]);
        assert_eq!(array_words(&e, this), vec![0, 0]);
    }

    #[test]
    fn deleting_destructors_run_their_body_then_free_on_bit_zero() {
        for (address, vtable) in [
            (0x006b_7e20, INFO_MAP_BASE_VTABLE),
            (0x006b_7e50, WORLD_MAP_BASE_VTABLE),
            (0x006b_8280, INFO_MAP_BASE_VTABLE),
            (0x006b_82b0, WORLD_MAP_BASE_VTABLE),
            (0x006b_82e0, CELL_MAP_BASE_VTABLE),
        ] {
            for (flags, frees) in [(0u32, false), (1, true), (2, false), (3, true)] {
                let mut e = Engine::new();
                e.register(MAP_REMOVE_ALL_00438AF0, |_, _| Ret::default());
                e.register(BUCKET_FREE_00AA10F0, |_, _| Ret::default());
                e.register(DEALLOCATE, |_, _| Ret::default());
                let this = e.mem.alloc(0x10);
                e.mem.set_u32(this + 8, 0x9000);
                e.call_log = Some(vec![]);
                assert_eq!(e.call(address, &args![this, flags]).u32(), this);
                let log = e.call_log.take().unwrap();
                assert_eq!(e.mem.u32(this), vtable);
                assert_eq!(calls_to(&log, BUCKET_FREE_00AA10F0), vec![vec![0x9000]]);
                let freed = calls_to(&log, DEALLOCATE);
                assert_eq!(freed.len(), usize::from(frees));
                if frees {
                    assert_eq!(freed[0], vec![this]);
                }
            }
        }
    }
}
