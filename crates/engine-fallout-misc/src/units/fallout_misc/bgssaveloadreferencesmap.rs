//! `fallout/misc/saveload/bgssaveloadreferencesmap.cpp` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00853130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Map lookup, returning success in AL. A failed lookup leaves the output
/// untouched. Bucket pointers, links and values are read after the relevant
/// hash/equality callback, as in the PC instructions.
pub fn lookup(e: &mut Engine, this: Ptr, key: u32, out: Ptr<u32>) -> u8 {
    let bucket = e.vcall(this.addr(), 4, &args![key]).u32().wrapping_mul(4);
    let table = e.mem.u32(this.addr().wrapping_add(8));
    let mut node = e.mem.u32(table.wrapping_add(bucket));
    while node != 0 {
        let existing_key = e.mem.u32(node.wrapping_add(4));
        if e.vcall(this.addr(), 8, &args![key, existing_key]).u8() != 0 {
            let value = e.mem.u32(node.wrapping_add(8));
            e.mem.set_u32(out.addr(), value);
            return 1;
        }
        node = e.mem.u32(node);
    }
    0
}

// Translated from 00853240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the item's value then releases it through the allocator at +0xc.
pub fn delete_item(e: &mut Engine, this: Ptr, item: Ptr) {
    e.mem.set_u32(item.addr().wrapping_add(8), 0);
    e.call(0x006b8310, &args![this.addr().wrapping_add(0xc), item]);
}

/// `00aa1070`: the allocator (cdecl, byte count).
const ALLOCATE: u32 = 0x00aa_1070;
/// `00403d30` (`memset`, cdecl: destination, value, byte count).
const MEMSET: u32 = 0x0040_3d30;
/// The bucket count `00852810` and `00852a70` pass on.
const BUCKET_COUNT: u32 = 0x25;

/// Shared body of `00853270` and `00853370`: stores `vtable`, the bucket
/// count at +4 and 0 at +0xc, allocates `4 * count` bytes for the table
/// (+8) and zeroes it.
fn init_bucket_table(e: &mut Engine, this: Ptr, vtable: u32, bucket_count: u32) {
    let t = this.addr();
    e.mem.set_u32(t, vtable);
    e.mem.set_u32(t + 4, bucket_count);
    e.mem.set_u32(t + 0xc, 0);
    let bytes = e.mem.u32(t + 4) << 2;
    let table = e.call(ALLOCATE, &args![bytes]).u32();
    e.mem.set_u32(t + 8, table);
    let bytes = e.mem.u32(t + 4) << 2;
    let table = e.mem.u32(t + 8);
    e.call(MEMSET, &args![table, 0u32, bytes]);
}

// Translated from 00853270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: bucket-table initializer with the vtable
/// `010800a8`. Returns `this`.
pub fn fn_00853270(e: &mut Engine, this: Ptr, bucket_count: u32) -> Ptr {
    init_bucket_table(e, this, 0x0108_00a8, bucket_count);
    this
}

// Translated from 00853370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: bucket-table initializer with the vtable
/// `010800c8`. Returns `this`.
pub fn fn_00853370(e: &mut Engine, this: Ptr, bucket_count: u32) -> Ptr {
    init_bucket_table(e, this, 0x0108_00c8, bucket_count);
    this
}

// Translated from 008530c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the bucket-table initializer with the vtable
/// `01080088` (same body as `00853270`). Returns `this`.
pub fn fn_008530c0(e: &mut Engine, this: Ptr, bucket_count: u32) -> Ptr {
    init_bucket_table(e, this, 0x0108_0088, bucket_count);
    this
}

// Translated from 00852fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `008530c0(this, word)` and then the vtable
/// `01080028`. Returns `this`.
pub fn fn_00852fa0(e: &mut Engine, this: Ptr, word: u32) -> Ptr {
    fn_008530c0(e, this, word);
    e.mem.set_u32(this.addr(), 0x0108_0028);
    this
}

// Translated from 00852fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `00853270(this, bucket_count)`, then the
/// vtable `01080048`. Returns `this`.
pub fn fn_00852fd0(e: &mut Engine, this: Ptr, bucket_count: u32) -> Ptr {
    fn_00853270(e, this, bucket_count);
    e.mem.set_u32(this.addr(), 0x0108_0048);
    this
}

// Translated from 00853000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `00853370(this, bucket_count)`, then the
/// vtable `01080068`. Returns `this`.
pub fn fn_00853000(e: &mut Engine, this: Ptr, bucket_count: u32) -> Ptr {
    fn_00853370(e, this, bucket_count);
    e.mem.set_u32(this.addr(), 0x0108_0068);
    this
}

// Translated from 00852810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `00852fa0(this, 0x25)`, then the vtable
/// `0107fef4`. Returns `this`.
pub fn fn_00852810(e: &mut Engine, this: Ptr) -> Ptr {
    fn_00852fa0(e, this, BUCKET_COUNT);
    e.mem.set_u32(this.addr(), 0x0107_fef4);
    this
}

// Translated from 00852a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the constructor of the object that holds three
/// maps: `00852fd0(this, 0x25)`, `00852810` on the member at +0x10 and
/// `00853000(this + 0x20, 0x25)`. Returns `this`.
// Translation incomplete: 00852a70 - SEH exception unwinding is unsupported.
pub fn fn_00852a70(e: &mut Engine, this: Ptr) -> Ptr {
    fn_00852fd0(e, this, BUCKET_COUNT);
    fn_00852810(e, this.byte_add(0x10).cast());
    fn_00853000(e, this.byte_add(0x20).cast(), BUCKET_COUNT);
    this
}

/// `00438af0` (unnamed in the engine map): `thiscall`, no stack words; empties a hash map (the
/// shared `RemoveAll` body of the `NiTMapBase` instances).
const MAP_REMOVE_ALL: u32 = 0x0043_8af0;
/// `00aa10f0` (unnamed in the engine map): `cdecl`, one word; frees a block from `00aa1070`.
const FREE: u32 = 0x00aa_10f0;
/// `008528d0` (unnamed in the engine map): `thiscall`, no stack words; empties the map at `+0x10`
/// of `BGSSaveLoadReferencesMap`.
const MAP_0X10_REMOVE_ALL: u32 = 0x0085_28d0;

/// Shared body of the three bucket-table destructors `00853210`, `00853340` and `00853440`:
/// stores `vtable`, empties the map (`00438af0`) and frees its bucket table (`+8`) through
/// `00aa10f0`.
fn destroy_bucket_table(e: &mut Engine, this: Ptr, vtable: u32) {
    e.mem.set_u32(this.addr(), vtable);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let table = e.mem.u32(this.addr().wrapping_add(8));
    e.call(FREE, &args![table]);
}

// Translated from 00853210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: destructor of the bucket table `008530c0` builds (vtable
/// `01080088`): stores the vtable, empties the map through `00438af0` and frees the bucket table
/// (`+8`) through `00aa10f0`.
pub fn fn_00853210(e: &mut Engine, this: Ptr) {
    destroy_bucket_table(e, this, 0x0108_0088);
}

// Translated from 00853340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: destructor of the bucket table `00853270` builds (vtable
/// `010800a8`), the same body as `00853210`.
pub fn fn_00853340(e: &mut Engine, this: Ptr) {
    destroy_bucket_table(e, this, 0x0108_00a8);
}

// Translated from 00853440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: destructor of the bucket table `00853370` builds (vtable
/// `010800c8`), the same body as `00853210`.
pub fn fn_00853440(e: &mut Engine, this: Ptr) {
    destroy_bucket_table(e, this, 0x0108_00c8);
}

// Translated from 008531b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008531b0 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// Unnamed in the engine map: destructor of the map `00852fa0` builds: stores the vtable
/// `01080028`, empties the map through `00438af0`, then runs the base destructor `00853210` of
/// this unit.
pub fn fn_008531b0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0108_0028);
    e.call(MAP_REMOVE_ALL, &args![this]);
    fn_00853210(e, this);
}

// Translated from 008532e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008532e0 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// Unnamed in the engine map: destructor of the map `00852fd0` builds: stores the vtable
/// `01080048`, empties the map through `00438af0`, then runs the base destructor `00853340` of
/// this unit.
pub fn fn_008532e0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0108_0048);
    e.call(MAP_REMOVE_ALL, &args![this]);
    fn_00853340(e, this);
}

// Translated from 008533e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008533e0 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// Unnamed in the engine map: destructor of the map `00853000` builds: stores the vtable
/// `01080068`, empties the map through `00438af0`, then runs the base destructor `00853440` of
/// this unit.
pub fn fn_008533e0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0108_0068);
    e.call(MAP_REMOVE_ALL, &args![this]);
    fn_00853440(e, this);
}

// Translated from 00852870 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00852870 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// Unnamed in the engine map: destructor of the map `00852810` builds: stores the vtable
/// `0107fef4`, empties it through `008528d0`, then runs `008531b0` of this unit.
pub fn fn_00852870(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0107_fef4);
    e.call(MAP_0X10_REMOVE_ALL, &args![this]);
    fn_008531b0(e, this);
}

// Translated from 00852b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: empties the three maps of `BGSSaveLoadReferencesMap`: the map at
/// `+0` through `00438af0`, the one at `+0x10` through `008528d0`; for the one at `+0x20` it takes
/// the first position from `004b9ba0` and, while the position is non-zero, gets the next entry
/// through `006b7f20(map, &position, &key, &value)` (key and value cleared first) and deletes a
/// non-null value through its vtable slot 0 with 1 (the scalar deleting destructor); then that
/// map is emptied through `00438af0`.
pub fn fn_00852b60(e: &mut Engine, this: Ptr) {
    e.call(MAP_REMOVE_ALL, &args![this]);
    let second = this.addr().wrapping_add(0x10);
    e.call(MAP_0X10_REMOVE_ALL, &args![second]);
    let third = this.addr().wrapping_add(0x20);
    e.with_stack(0xc, |e, locals| {
        let value_slot = locals.addr();
        let key_slot = value_slot + 4;
        let position_slot = value_slot + 8;
        let first = e.call(0x004b_9ba0, &args![third]).u32();
        e.mem.set_u32(position_slot, first);
        while e.mem.u32(position_slot) != 0 {
            e.mem.set_u32(key_slot, 0);
            e.mem.set_u32(value_slot, 0);
            e.call(
                0x006b_7f20,
                &args![third, position_slot, key_slot, value_slot],
            );
            let value = e.mem.u32(value_slot);
            if value != 0 {
                e.vcall(value, 0, &args![1u32]);
            }
        }
    });
    e.call(MAP_REMOVE_ALL, &args![third]);
}

// Translated from 00852ae0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00852ae0 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// Unnamed in the engine map: destructor of `BGSSaveLoadReferencesMap` (the object `00852a70`
/// builds): empties the maps through `00852b60`, then destroys the members in reverse order:
/// `008533e0` on `+0x20`, `00852870` on `+0x10` and `008532e0` on `this`.
pub fn fn_00852ae0(e: &mut Engine, this: Ptr) {
    fn_00852b60(e, this);
    fn_008533e0(e, this.byte_add(0x20).cast());
    fn_00852870(e, this.byte_add(0x10).cast());
    fn_008532e0(e, this);
}

// Translated from 00852d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadReferencesMap::LoadReferencesForCell` (Xbox PDB). `RET 8`: `cell` and a word
/// passed on to each `LoadChangedReference` (`00852e40`). When `00425fd0(cell)` is true the
/// cell's key is `0084e3a0(cell)` and the map is the one at `+0x10`. Otherwise the key is
/// `00587410(TESObjectCELL::GetDataX(cell), TESObjectCELL::GetDataY(cell))` (`00544c30`,
/// `00544c60`, called Y first) and the map is looked up (`00853130` of this unit) in the map at
/// `+0x20` under `0084e3a0(TESObjectCELL::GetWorldSpace(cell))` (`0054ddd0`); a failed lookup
/// leaves it null. With a map, a successful lookup of the cell's key gives a list: each of its
/// `0044ddc0` entries (`006a7ad0(list, index)` gives the entry's address) is loaded through
/// `00852e40(this, entry, word)`; the key is then removed from the map (`00405430`) and the list
/// is deleted through its vtable slot 0 with 1.
pub fn bgs_save_load_references_map_load_references_for_cell(
    e: &mut Engine,
    this: Ptr,
    cell: Ptr,
    word: u32,
) {
    e.with_stack(8, |e, locals| {
        let map_slot = locals.addr();
        let list_slot = map_slot + 4;
        e.mem.set_u32(map_slot, 0);
        let key = if e.call(0x0042_5fd0, &args![cell]).bool() {
            let key = e.call(0x0084_e3a0, &args![cell]).u32();
            e.mem.set_u32(map_slot, this.addr().wrapping_add(0x10));
            key
        } else {
            let y = e.call(0x0054_4c60, &args![cell]).u32();
            let x = e.call(0x0054_4c30, &args![cell]).u32();
            let key = e.call(0x0058_7410, &args![x, y]).u32();
            let world = e.call(0x0054_ddd0, &args![cell]).u32();
            let world_key = e.call(0x0084_e3a0, &args![world]).u32();
            lookup(e, this.byte_add(0x20).cast(), world_key, Ptr::new(map_slot));
            key
        };
        let map = e.mem.u32(map_slot);
        if map == 0 {
            return;
        }
        e.mem.set_u32(list_slot, 0);
        if lookup(e, Ptr::new(map), key, Ptr::new(list_slot)) == 0 {
            return;
        }
        let mut index = 0u32;
        loop {
            let list = e.mem.u32(list_slot);
            let count = e.call(0x0044_ddc0, &args![list]).u32();
            if index >= count {
                break;
            }
            let list = e.mem.u32(list_slot);
            let entry = e.call(0x006a_7ad0, &args![list, index]).u32();
            let reference = e.mem.u32(entry);
            e.call(0x0085_2e40, &args![this, reference, word]);
            index = index.wrapping_add(1);
        }
        let map = e.mem.u32(map_slot);
        e.call(0x0040_5430, &args![map, key]);
        let list = e.mem.u32(list_slot);
        if list != 0 {
            e.vcall(list, 0, &args![1u32]);
        }
    });
}

/// `00401030` (cdecl, one word): the game's `operator delete`.
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `00401000` (cdecl: size): the game's `operator new`.
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `00719b20` (`thiscall`, `RET 0xc`: the address of the value, a start
/// index, a compare function): the value's index in the array, -1 when
/// absent.
const ARRAY_FIND: u32 = 0x0071_9b20;
/// The compare function `00852a10` passes (`009a3830`).
const ARRAY_COMPARE: u32 = 0x009a_3830;
/// `BSSimpleArray::_MoveItems` (`009a4320`, `thiscall`, `RET 8`: index,
/// count): removes `count` items at `index`.
const ARRAY_MOVE_ITEMS: u32 = 0x009a_4320;
/// `TESWorldSpace::GetKeyForWorldCoord` (`00587440`, cdecl, one word).
const KEY_FOR_WORLD_COORD: u32 = 0x0058_7440;
/// `00844700` (`thiscall`, `RET 8`: key, value): the map's `SetAt`.
const MAP_SET_AT: u32 = 0x0084_4700;
/// `00852950` (`thiscall`, `RET 8`: key, reference): adds the reference to
/// the cell's array map.
const ARRAY_MAP_ADD: u32 = 0x0085_2950;
/// The save/load game singleton pointer (`011ddf38`).
const SAVE_LOAD_GAME: u32 = 0x011d_df38;
/// `00559450` (`thiscall`): the word at `this`.
const WORD_AT: u32 = 0x0055_9450;
/// `CombatTargetMap::GetTargetAttackerArray` (`009a4250`, `thiscall`,
/// `RET 4`; a folded body): the map's value for a key, null when absent.
const MAP_VALUE: u32 = 0x009a_4250;
/// `0084e730` (cdecl: id, form type, word): the changed form's kind (5 a
/// created reference, 6 a moved one, by the warnings).
const CHANGE_KIND: u32 = 0x0084_e730;
/// `SaveGameWarning` (`00845cc0`, cdecl with variable arguments).
const SAVE_GAME_WARNING: u32 = 0x0084_5cc0;
/// The three warnings `LoadChangedReference` gives.
const WARNING_NOT_CREATED_OR_MOVED: u32 = 0x0107_ffc0;
const WARNING_NO_UNLOADED_BUFFER: u32 = 0x0107_ff58;
const WARNING_NO_CELL_OR_WORLD: u32 = 0x0107_ff10;
/// `00849280` (`thiscall`): constructs the initial-data local (0x30 bytes).
const INITIAL_DATA_CONSTRUCT: u32 = 0x0084_9280;
/// `BGSUnloadedFormBuffer::GetAdvancedBuffer` (`00866390`, `thiscall`).
const ADVANCED_BUFFER: u32 = 0x0086_6390;
/// `BGSSaveLoadInitialData::LoadInitialDataStruct_ov3` (`0084e930`, cdecl:
/// data, kind, structure).
const LOAD_INITIAL_DATA: u32 = 0x0084_e930;
/// `BGSSaveLoadInitialData::CreateReference` (`0084f8e0`, cdecl: id,
/// structure, 0).
const CREATE_REFERENCE: u32 = 0x0084_f8e0;
/// `BGSSaveLoadGame::LoadForm` (`008495d0`, `thiscall`, `RET 4`).
const LOAD_FORM: u32 = 0x0084_95d0;
/// `ReferenceInitialData::GetOriginalLocationCellAndWorld` (`0084e6a0`,
/// `thiscall`, `RET 8`: cell out, world out).
const ORIGINAL_CELL_AND_WORLD: u32 = 0x0084_e6a0;
/// `BGSReconstructFormsInAllFilesMap::AddReference` (`00843750`, `RET 0xc`:
/// cell, id, 0) and `AddReference_ov2` (`008438b0`, `RET 0x14`: world, two
/// coordinate words, id, 0).
const RECONSTRUCT_ADD_REFERENCE: u32 = 0x0084_3750;
const RECONSTRUCT_ADD_REFERENCE_IN_WORLD: u32 = 0x0084_38b0;

/// The shared body of the scalar deleting destructors: `destroy` on
/// `this`, then `operator delete` (`00401030`) when bit 0 of `flags` is
/// set. Answers `this`.
fn scalar_delete(e: &mut Engine, this: Ptr, flags: u32, destroy: fn(&mut Engine, Ptr)) -> Ptr {
    destroy(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00852840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSCellNumericIDArrayMap` scalar deleting destructor (engine map
/// name; `RET 4`): `00852870`, then deleted on flag bit 0.
pub fn fn_00852840(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    scalar_delete(e, this, flags, fn_00852870)
}

// Translated from 00853030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor (engine map: `NiTMap<unsigned int,
/// BSSimpleArray<unsigned int,1024>*>`; `RET 4`): `008531b0`, then deleted
/// on flag bit 0.
pub fn fn_00853030(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    scalar_delete(e, this, flags, fn_008531b0)
}

// Translated from 00853060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor (engine map: `NiTPointerMap<unsigned int,
/// unsigned int>`; `RET 4`): `008532e0`, then deleted on flag bit 0.
pub fn fn_00853060(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    scalar_delete(e, this, flags, fn_008532e0)
}

// Translated from 00853090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor (engine map: `NiTPointerMap<unsigned int,
/// BGSCellNumericIDArrayMap*>`; `RET 4`): `008533e0`, then deleted on flag
/// bit 0.
pub fn fn_00853090(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    scalar_delete(e, this, flags, fn_008533e0)
}

// Translated from 00853470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the `NiTMapBase` under `00853030` (engine
/// map name; `RET 4`): `00853210`, then deleted on flag bit 0.
pub fn fn_00853470(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    scalar_delete(e, this, flags, fn_00853210)
}

// Translated from 008534a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the `NiTMapBase` under `00853060` (engine
/// map name; `RET 4`): `00853340`, then deleted on flag bit 0.
pub fn fn_008534a0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    scalar_delete(e, this, flags, fn_00853340)
}

// Translated from 008534d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the `NiTMapBase` under `00853090` (engine
/// map name; `RET 4`): `00853440`, then deleted on flag bit 0.
pub fn fn_008534d0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    scalar_delete(e, this, flags, fn_00853440)
}

// Translated from 00852a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSCellNumericIDArrayMap::RemoveReference` (Xbox PDB; `thiscall`,
/// `RET 8`: key, reference): when the map holds an array under `key`
/// (`00853130`), the reference's index in it (`00719b20` with the address
/// of the argument word, from 0, compare `009a3830`) is removed
/// (`_MoveItems` with 1) unless it is -1.
pub fn bgs_cell_numeric_id_array_map_remove_reference(
    e: &mut Engine,
    this: Ptr,
    key: u32,
    reference: u32,
) {
    e.with_stack(8, |e, locals| {
        let array_slot = locals.addr();
        let reference_slot = array_slot + 4;
        e.mem.set_u32(array_slot, 0);
        e.mem.set_u32(reference_slot, reference);
        if lookup(e, this, key, Ptr::new(array_slot)) == 0 {
            return;
        }
        let array = e.mem.u32(array_slot);
        let index = e
            .call(
                ARRAY_FIND,
                &args![array, reference_slot, 0u32, ARRAY_COMPARE],
            )
            .u32();
        if index != 0xffff_ffff {
            let array = e.mem.u32(array_slot);
            e.call(ARRAY_MOVE_ITEMS, &args![array, index, 1u32]);
        }
    });
}

// Translated from 00852c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0xc`: key, reference,
/// coordinate word): the cell array map under `key` in the map at +0x20
/// (`00853130`), or a new one (0x10 bytes, `00401000`, `00852810`; null
/// when the allocation fails) set there (`00844700`); the reference is
/// added to it (`00852950`) under the key of the coordinate word
/// (`TESWorldSpace::GetKeyForWorldCoord`).
// Translation incomplete: 00852c00 - the C++ exception frame (FS:[0] chain, unwind state 0 that frees the new map) and the /GS cookie word are not translated.
pub fn fn_00852c00(e: &mut Engine, this: Ptr, key: u32, reference: u32, coordinate: u32) {
    let worlds = this.addr().wrapping_add(0x20);
    let map = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        if lookup(e, Ptr::new(worlds), key, slot.cast()) == 0 {
            let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
            let made = if block == 0 {
                0
            } else {
                fn_00852810(e, Ptr::new(block)).addr()
            };
            e.mem.set_u32(slot.addr(), made);
            e.call(MAP_SET_AT, &args![worlds, key, made]);
        }
        e.mem.u32(slot.addr())
    });
    let cell_key = e.call(KEY_FOR_WORLD_COORD, &args![coordinate]).u32();
    e.call(ARRAY_MAP_ADD, &args![map, cell_key, reference]);
}

// Translated from 00852cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadReferencesMap::RemoveReference` (Xbox PDB; `thiscall`,
/// `RET 0xc`: key, reference, coordinate word): when the map at +0x20
/// holds a cell array map under `key`, removes the reference from it
/// (`00852a10`) under the key of the coordinate word.
pub fn bgs_save_load_references_map_remove_reference(
    e: &mut Engine,
    this: Ptr,
    key: u32,
    reference: u32,
    coordinate: u32,
) {
    let worlds = this.addr().wrapping_add(0x20);
    let map = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        let found = lookup(e, Ptr::new(worlds), key, slot.cast());
        (found != 0).then(|| e.mem.u32(slot.addr()))
    });
    if let Some(map) = map {
        let cell_key = e.call(KEY_FOR_WORLD_COORD, &args![coordinate]).u32();
        bgs_cell_numeric_id_array_map_remove_reference(e, Ptr::new(map), cell_key, reference);
    }
}

// Translated from 00852e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadReferencesMap::LoadChangedReference` (Xbox PDB; `thiscall`,
/// `RET 8`: the reference's id, the reconstruct map; `this` is not read).
/// The save/load game's changes map entry for the id (`00559450`,
/// `009a4250`); nothing without one. Its kind (`0084e730` with 0x3a and
/// the entry's first word) must be 5 or 6, else a warning. Without an
/// unloaded buffer (the entry's +4) a warning; otherwise the initial data
/// (a 0x30-byte local, `00849280`) is loaded from the advanced buffer
/// (`00866390`, `0084e930`). Kind 5 creates the reference (`0084f8e0`) and
/// loads it (`BGSSaveLoadGame::LoadForm`); kind 6 finds its original cell
/// and world (`0084e6a0`; a warning with the data's +8 word when it fails)
/// and adds it to the reconstruct map by cell (`00843750`) or, without a
/// cell, by world and the data's +0xc and +0x10 words (`008438b0`).
pub fn bgs_save_load_references_map_load_changed_reference(
    e: &mut Engine,
    _this: Ptr,
    id: u32,
    reconstruct: u32,
) {
    let game = e.global::<u32>(SAVE_LOAD_GAME);
    let changes = e.call(WORD_AT, &args![game]).u32();
    let entry = e.call(MAP_VALUE, &args![changes, id]).u32();
    if entry == 0 {
        return;
    }
    let first = e.mem.u32(entry);
    let kind = e.call(CHANGE_KIND, &args![id, 0x3au32, first]).u32();
    if kind != 5 && kind != 6 {
        e.call(SAVE_GAME_WARNING, &args![WARNING_NOT_CREATED_OR_MOVED, id]);
        return;
    }
    let buffer = entry.wrapping_add(4);
    if e.call(WORD_AT, &args![buffer]).u32() == 0 {
        e.call(SAVE_GAME_WARNING, &args![WARNING_NO_UNLOADED_BUFFER, id]);
        return;
    }
    e.with_stack(0x38, |e, locals| {
        let data = locals.addr();
        let cell_slot = data + 0x30;
        let world_slot = data + 0x34;
        e.call(INITIAL_DATA_CONSTRUCT, &args![data]);
        let advanced = e.call(ADVANCED_BUFFER, &args![buffer]).u32();
        e.call(LOAD_INITIAL_DATA, &args![advanced, kind, data]);
        if kind == 5 {
            let reference = e.call(CREATE_REFERENCE, &args![id, data, 0u32]).u32();
            if reference != 0 {
                let game = e.global::<u32>(SAVE_LOAD_GAME);
                e.call(LOAD_FORM, &args![game, reference]);
            }
            return;
        }
        e.mem.set_u32(cell_slot, 0);
        e.mem.set_u32(world_slot, 0);
        let found = e
            .call(ORIGINAL_CELL_AND_WORLD, &args![data, cell_slot, world_slot])
            .bool();
        if !found {
            let location = e.mem.u32(data + 8);
            e.call(
                SAVE_GAME_WARNING,
                &args![WARNING_NO_CELL_OR_WORLD, location, id],
            );
            return;
        }
        let cell = e.mem.u32(cell_slot);
        if cell != 0 {
            e.call(
                RECONSTRUCT_ADD_REFERENCE,
                &args![reconstruct, cell, id, 0u32],
            );
        } else {
            let world = e.mem.u32(world_slot);
            let x = e.mem.u32(data + 0xc);
            let y = e.mem.u32(data + 0x10);
            e.call(
                RECONSTRUCT_ADD_REFERENCE_IN_WORLD,
                &args![reconstruct, world, x, y, id, 0u32],
            );
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0085_2840, fn_00852840(Ptr, u32) -> Ptr),
        entry!(0x0085_3030, fn_00853030(Ptr, u32) -> Ptr),
        entry!(0x0085_3060, fn_00853060(Ptr, u32) -> Ptr),
        entry!(0x0085_3090, fn_00853090(Ptr, u32) -> Ptr),
        entry!(0x0085_3470, fn_00853470(Ptr, u32) -> Ptr),
        entry!(0x0085_34a0, fn_008534a0(Ptr, u32) -> Ptr),
        entry!(0x0085_34d0, fn_008534d0(Ptr, u32) -> Ptr),
        entry!(
            0x0085_2a10,
            bgs_cell_numeric_id_array_map_remove_reference(Ptr, u32, u32)
        ),
        entry!(0x0085_2c00, fn_00852c00(Ptr, u32, u32, u32)),
        entry!(
            0x0085_2cd0,
            bgs_save_load_references_map_remove_reference(Ptr, u32, u32, u32)
        ),
        entry!(
            0x0085_2e40,
            bgs_save_load_references_map_load_changed_reference(Ptr, u32, u32)
        ),
        entry!(0x00853130, lookup(Ptr, u32, Ptr<u32>) -> u8),
        entry!(0x00853240, delete_item(Ptr, Ptr)),
        entry!(0x00853270, fn_00853270(Ptr, u32) -> Ptr),
        entry!(0x00853370, fn_00853370(Ptr, u32) -> Ptr),
        entry!(0x008530c0, fn_008530c0(Ptr, u32) -> Ptr),
        entry!(0x00852fa0, fn_00852fa0(Ptr, u32) -> Ptr),
        entry!(0x00852fd0, fn_00852fd0(Ptr, u32) -> Ptr),
        entry!(0x00853000, fn_00853000(Ptr, u32) -> Ptr),
        entry!(0x00852810, fn_00852810(Ptr) -> Ptr),
        entry!(0x00852a70, fn_00852a70(Ptr) -> Ptr),
        entry!(0x0085_3210, fn_00853210(Ptr)),
        entry!(0x0085_3340, fn_00853340(Ptr)),
        entry!(0x0085_3440, fn_00853440(Ptr)),
        entry!(0x0085_31b0, fn_008531b0(Ptr)),
        entry!(0x0085_32e0, fn_008532e0(Ptr)),
        entry!(0x0085_33e0, fn_008533e0(Ptr)),
        entry!(0x0085_2870, fn_00852870(Ptr)),
        entry!(0x0085_2b60, fn_00852b60(Ptr)),
        entry!(0x0085_2ae0, fn_00852ae0(Ptr)),
        entry!(
            0x0085_2d20,
            bgs_save_load_references_map_load_references_for_cell(Ptr, Ptr, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn doubles(e: &mut Engine) {
        e.register(ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret::default()
        });
    }

    #[test]
    fn the_bucket_table_initializers_differ_only_in_the_vtable() {
        for (addr, vtable) in [(0x00853270, 0x0108_00a8), (0x00853370, 0x0108_00c8)] {
            let mut e = Engine::new();
            doubles(&mut e);
            let this = e.mem.alloc(0x10);
            e.mem.set_u32(this + 0xc, 5);
            let r = e.call(addr, &args![this, 7u32]);
            assert_eq!(r.u32(), this);
            assert_eq!(e.mem.u32(this), vtable);
            assert_eq!(e.mem.u32(this + 4), 7);
            assert_eq!(e.mem.u32(this + 0xc), 0);
            let table = e.mem.u32(this + 8);
            assert_eq!(e.mem.bytes(table, 28), vec![0; 28]);
        }
    }

    #[test]
    fn fn_008530c0_builds_a_zeroed_bucket_table() {
        let mut e = Engine::new();
        doubles(&mut e);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, 5);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x008530c0, &args![this, 3u32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x0108_0088);
        assert_eq!(e.mem.u32(this + 4), 3);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        let table = e.mem.u32(this + 8);
        assert_eq!(e.mem.bytes(table, 12), vec![0; 12]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (ALLOCATE, vec![12]));
        assert_eq!(log[2], (MEMSET, vec![table, 0, 12]));
    }

    #[test]
    fn the_wrappers_store_their_own_vtables() {
        let mut e = Engine::new();
        doubles(&mut e);
        let this = e.mem.alloc(0x10);
        e.call(0x00852fa0, &args![this, 0x11u32]);
        assert_eq!(e.mem.u32(this), 0x0108_0028);
        assert_eq!(e.mem.u32(this + 4), 0x11, "the base initializer ran");
        e.call(0x00852fd0, &args![this, 2u32]);
        assert_eq!((e.mem.u32(this), e.mem.u32(this + 4)), (0x0108_0048, 2));
        e.call(0x00853000, &args![this, 3u32]);
        assert_eq!((e.mem.u32(this), e.mem.u32(this + 4)), (0x0108_0068, 3));
        e.call(0x00852810, &args![this]);
        assert_eq!(e.mem.u32(this), 0x0107_fef4);
    }

    #[test]
    fn the_constructor_builds_three_maps() {
        let mut e = Engine::new();
        doubles(&mut e);
        let this = e.mem.alloc(0x30);
        let r = e.call(0x00852a70, &args![this]);
        assert_eq!(r.u32(), this);
        assert_eq!(e.mem.u32(this), 0x0108_0048);
        assert_eq!(e.mem.u32(this + 4), 0x25);
        assert_eq!(e.mem.u32(this + 0x10), 0x0107_fef4);
        assert_eq!(e.mem.u32(this + 0x20), 0x0108_0068);
        assert_eq!(e.mem.u32(this + 0x24), 0x25);
    }
}

#[cfg(test)]
mod b035_tests {
    use super::*;

    fn calls(log: &[(u32, Vec<u32>)], address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    #[test]
    fn the_bucket_table_destructors_empty_and_free() {
        for (address, vtable) in [
            (0x0085_3210, 0x0108_0088),
            (0x0085_3340, 0x0108_00a8),
            (0x0085_3440, 0x0108_00c8),
        ] {
            let mut e = Engine::new();
            quiet(&mut e, &[MAP_REMOVE_ALL, FREE]);
            let this = e.mem.alloc(0x10);
            e.mem.set_u32(this + 8, 0x7700);
            e.call_log = Some(vec![]);
            e.call(address, &args![this]);
            let log = e.call_log.take().unwrap();
            assert_eq!(e.mem.u32(this), vtable);
            assert_eq!(
                log[1..],
                [(MAP_REMOVE_ALL, vec![this]), (FREE, vec![0x7700])]
            );
        }
    }

    #[test]
    fn the_derived_destructors_chain_to_their_bases() {
        for (address, vtable, base_vtable) in [
            (0x0085_31b0, 0x0108_0028, 0x0108_0088),
            (0x0085_32e0, 0x0108_0048, 0x0108_00a8),
            (0x0085_33e0, 0x0108_0068, 0x0108_00c8),
        ] {
            let mut e = Engine::new();
            quiet(&mut e, &[FREE]);
            let this = e.mem.alloc(0x10);
            e.mem.set_u32(this + 8, 0x7700);
            let seen = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
            let record = seen.clone();
            e.register_double(MAP_REMOVE_ALL, move |e, a| {
                record.borrow_mut().push(e.mem.u32(a[0]));
                Ret::default()
            });
            e.call(address, &args![this]);
            assert_eq!(*seen.borrow(), vec![vtable, base_vtable]);
            assert_eq!(e.mem.u32(this), base_vtable);
        }
    }

    #[test]
    fn fn_00852870_empties_then_runs_the_map_destructor() {
        let mut e = Engine::new();
        quiet(&mut e, &[MAP_REMOVE_ALL, FREE, MAP_0X10_REMOVE_ALL]);
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x0085_2870, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, MAP_0X10_REMOVE_ALL), vec![vec![this]]);
        assert_eq!(calls(&log, MAP_REMOVE_ALL).len(), 2);
        assert_eq!(e.mem.u32(this), 0x0108_0088);
    }

    #[test]
    fn fn_00852b60_deletes_the_values_of_the_third_map() {
        let mut e = Engine::new();
        quiet(&mut e, &[MAP_REMOVE_ALL, MAP_0X10_REMOVE_ALL]);
        let this = e.mem.alloc(0x30);
        // Two entries: the first with a value, the second without; position 2 ends the walk.
        e.register(0x004b_9ba0, |_, _| 1u32.into_ret());
        e.register(0x006b_7f20, |e, a| {
            let position = e.mem.u32(a[1]);
            let value = if position == 1 { 0x6000 } else { 0 };
            e.mem.set_u32(a[3], value);
            e.mem.set_u32(a[1], if position == 1 { 2 } else { 0 });
            Ret::default()
        });
        e.map(0x6000, 0x1000);
        e.mem.set_u32(0x6000, 0x6100);
        e.mem.set_u32(0x6100, 0x7000_0010);
        quiet(&mut e, &[0x7000_0010]);
        e.call_log = Some(vec![]);
        e.call(0x0085_2b60, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, MAP_REMOVE_ALL),
            vec![vec![this], vec![this + 0x20]]
        );
        assert_eq!(calls(&log, MAP_0X10_REMOVE_ALL), vec![vec![this + 0x10]]);
        assert_eq!(calls(&log, 0x006b_7f20).len(), 2);
        assert_eq!(calls(&log, 0x006b_7f20)[0][0], this + 0x20);
        assert_eq!(calls(&log, 0x7000_0010), vec![vec![0x6000, 1]]);
    }

    #[test]
    fn fn_00852ae0_destroys_the_members_in_reverse() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[MAP_REMOVE_ALL, MAP_0X10_REMOVE_ALL, FREE, 0x006b_7f20],
        );
        e.register(0x004b_9ba0, |_, _| 0u32.into_ret());
        let this = e.mem.alloc(0x30);
        e.call_log = Some(vec![]);
        e.call(0x0085_2ae0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, MAP_0X10_REMOVE_ALL),
            vec![vec![this + 0x10], vec![this + 0x10]]
        );
        assert_eq!(e.mem.u32(this), 0x0108_00a8);
        assert_eq!(e.mem.u32(this + 0x10), 0x0108_0088);
        assert_eq!(e.mem.u32(this + 0x20), 0x0108_00c8);
        let frees = calls(&log, MAP_REMOVE_ALL);
        assert_eq!(*frees.last().unwrap(), vec![this]);
    }

    const HASH: u32 = 0x7000_0004;
    const EQUAL: u32 = 0x7000_0008;
    const MAP_VTABLE: u32 = 0x6800;

    /// A one-bucket map (the hash is always 0) holding `entries` as (key, value).
    fn one_bucket_map(e: &mut Engine, entries: &[(u32, u32)]) -> u32 {
        let map = e.mem.alloc(0x10);
        e.mem.set_u32(map, MAP_VTABLE);
        let table = e.mem.alloc(4);
        e.mem.set_u32(map + 8, table);
        let mut next = 0;
        for (key, value) in entries {
            let node = e.mem.alloc(0xc);
            e.mem.set_u32(node, next);
            e.mem.set_u32(node + 4, *key);
            e.mem.set_u32(node + 8, *value);
            next = node;
        }
        e.mem.set_u32(table, next);
        map
    }

    fn map_doubles(e: &mut Engine) {
        e.map(MAP_VTABLE & !0xfff, 0x1000);
        e.put_vtable(MAP_VTABLE, &[0, HASH, EQUAL]);
        e.register(HASH, |_, _| 0u32.into_ret());
        e.register(EQUAL, |_, a| (a[1] == a[2]).into_ret());
    }

    /// A list of `entries` reference words: `0044ddc0` gives the count at `+4`, `006a7ad0` the
    /// entry's address in the block at `+8`; vtable slot 0 is the deleting destructor.
    fn list(e: &mut Engine, entries: &[u32]) -> u32 {
        let list = e.mem.alloc(0x10);
        let vtable = e.mem.alloc(4);
        e.mem.set_u32(vtable, 0x7000_0020);
        e.mem.set_u32(list, vtable);
        e.mem.set_u32(list + 4, entries.len() as u32);
        let block = e.mem.alloc(4 * entries.len() as u32 + 4);
        for (index, entry) in entries.iter().enumerate() {
            e.mem.set_u32(block + 4 * index as u32, *entry);
        }
        e.mem.set_u32(list + 8, block);
        e.register(0x0044_ddc0, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(0x006a_7ad0, |e, a| {
            (e.mem.u32(a[0] + 8) + 4 * a[1]).into_ret()
        });
        e.register(0x7000_0020, |_, _| Ret::default());
        list
    }

    #[test]
    fn load_references_for_cell_uses_the_map_at_0x10_for_the_first_kind_of_cell() {
        let mut e = Engine::new();
        map_doubles(&mut e);
        let this = e.mem.alloc(0x30);
        let references = list(&mut e, &[0xa1, 0xa2]);
        let inner = one_bucket_map(&mut e, &[(0x55, references)]);
        // The map at +0x10 is the object's own member: copy the built map's words there.
        for offset in [0u32, 4, 8, 0xc] {
            let word = e.mem.u32(inner + offset);
            e.mem.set_u32(this + 0x10 + offset, word);
        }
        e.register(0x0042_5fd0, |_, _| true.into_ret());
        e.register(0x0084_e3a0, |_, _| 0x55u32.into_ret());
        quiet(&mut e, &[0x0085_2e40, 0x0040_5430]);
        e.call_log = Some(vec![]);
        e.call(0x0085_2d20, &args![this, 0xc0u32, 0x99u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, 0x0085_2e40),
            vec![vec![this, 0xa1, 0x99], vec![this, 0xa2, 0x99]]
        );
        assert_eq!(calls(&log, 0x0040_5430), vec![vec![this + 0x10, 0x55]]);
        assert_eq!(calls(&log, 0x7000_0020), vec![vec![references, 1]]);
    }

    #[test]
    fn load_references_for_cell_finds_an_exterior_cell_through_its_world() {
        let mut e = Engine::new();
        map_doubles(&mut e);
        let this = e.mem.alloc(0x30);
        let references = list(&mut e, &[0xb1]);
        let cells = one_bucket_map(&mut e, &[(0x1234, references)]);
        let worlds = one_bucket_map(&mut e, &[(0x77, cells)]);
        for offset in [0u32, 4, 8, 0xc] {
            let word = e.mem.u32(worlds + offset);
            e.mem.set_u32(this + 0x20 + offset, word);
        }
        e.register(0x0042_5fd0, |_, _| false.into_ret());
        e.register(0x0054_4c30, |_, _| 3u32.into_ret());
        e.register(0x0054_4c60, |_, _| 4u32.into_ret());
        e.register(0x0058_7410, |_, a| {
            assert_eq!(a, [3, 4]);
            0x1234u32.into_ret()
        });
        e.register(0x0054_ddd0, |_, _| 0x900u32.into_ret());
        e.register(0x0084_e3a0, |_, a| {
            assert_eq!(a, [0x900]);
            0x77u32.into_ret()
        });
        quiet(&mut e, &[0x0085_2e40, 0x0040_5430]);
        e.call_log = Some(vec![]);
        e.call(0x0085_2d20, &args![this, 0xc0u32, 5u32]);
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = log
            .iter()
            .map(|(a, _)| *a)
            .filter(|a| [0x0054_4c30, 0x0054_4c60].contains(a))
            .collect();
        assert_eq!(order, [0x0054_4c60, 0x0054_4c30]);
        assert_eq!(calls(&log, 0x0085_2e40), vec![vec![this, 0xb1, 5]]);
        assert_eq!(calls(&log, 0x0040_5430), vec![vec![cells, 0x1234]]);
        assert_eq!(calls(&log, 0x7000_0020), vec![vec![references, 1]]);
    }

    #[test]
    fn load_references_for_cell_stops_without_a_map_or_a_list() {
        let mut e = Engine::new();
        map_doubles(&mut e);
        let this = e.mem.alloc(0x30);
        let worlds = one_bucket_map(&mut e, &[]);
        for offset in [0u32, 4, 8, 0xc] {
            let word = e.mem.u32(worlds + offset);
            e.mem.set_u32(this + 0x20 + offset, word);
        }
        e.register(0x0042_5fd0, |_, _| false.into_ret());
        quiet(
            &mut e,
            &[0x0054_4c30, 0x0054_4c60, 0x0058_7410, 0x0054_ddd0],
        );
        e.register(0x0084_e3a0, |_, _| 0x77u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0085_2d20, &args![this, 0xc0u32, 5u32]);
        // The world has no map: nothing more is called.
        let log = e.call_log.take().unwrap();
        assert_eq!(log.last().unwrap().0, HASH);
        // A map without the cell's key: no list is loaded.
        let empty = one_bucket_map(&mut e, &[]);
        for offset in [0u32, 4, 8, 0xc] {
            let word = e.mem.u32(empty + offset);
            e.mem.set_u32(this + 0x10 + offset, word);
        }
        e.register(0x0042_5fd0, |_, _| true.into_ret());
        quiet(&mut e, &[0x0085_2e40, 0x0040_5430]);
        e.call_log = Some(vec![]);
        e.call(0x0085_2d20, &args![this, 0xc0u32, 5u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, 0x0085_2e40).is_empty());
        assert!(calls(&log, 0x0040_5430).is_empty());
    }
}

#[cfg(test)]
mod n072_tests {
    use super::*;

    const TEST_MAP_VTABLE: u32 = 0x0150_0000;
    const TEST_HASH: u32 = 0x0150_1004;
    const TEST_EQUAL: u32 = 0x0150_1008;

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    /// A one-bucket map (vtable: hash 0, key equality) at `at` (0x10
    /// bytes) holding `entries`.
    fn fill_map(e: &mut Engine, at: u32, entries: &[(u32, u32)]) {
        e.mem.set_u32(at, TEST_MAP_VTABLE);
        let table = e.mem.alloc(4);
        e.mem.set_u32(at + 8, table);
        let mut next = 0;
        for (key, value) in entries {
            let node = e.mem.alloc(0xc);
            e.mem.set_u32(node, next);
            e.mem.set_u32(node + 4, *key);
            e.mem.set_u32(node + 8, *value);
            next = node;
        }
        e.mem.set_u32(table, next);
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.put_vtable(TEST_MAP_VTABLE, &[0, TEST_HASH, TEST_EQUAL]);
        e.register(TEST_HASH, |_, _| 0u32.into_ret());
        e.register(TEST_EQUAL, |_, a| (a[1] == a[2]).into_ret());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn the_scalar_deleting_destructors_destroy_then_delete_on_bit_0() {
        for (address, body_vtable) in [
            (0x0085_3470, 0x0108_0088),
            (0x0085_34a0, 0x0108_00a8),
            (0x0085_34d0, 0x0108_00c8),
            (0x0085_3030, 0x0108_0088),
            (0x0085_3060, 0x0108_00a8),
            (0x0085_3090, 0x0108_00c8),
            (0x0085_2840, 0x0108_0088),
        ] {
            for (flags, deletes) in [(0u32, false), (3, true)] {
                let mut e = Engine::new();
                for a in [MAP_REMOVE_ALL, FREE, MAP_0X10_REMOVE_ALL, OPERATOR_DELETE] {
                    e.register(a, |_, _| Ret::default());
                }
                let this = e.mem.alloc(0x10);
                e.call_log = Some(vec![]);
                assert_eq!(e.call(address, &args![this, flags]).u32(), this);
                assert_eq!(e.mem.u32(this), body_vtable);
                let deleted = calls(&e, OPERATOR_DELETE);
                assert_eq!(deleted == vec![vec![this]], deletes);
                assert_eq!(deleted.is_empty(), !deletes);
            }
        }
    }

    #[test]
    fn remove_reference_from_a_cell_array() {
        let mut e = engine();
        e.register(ARRAY_FIND, |e, a| {
            if e.mem.u32(a[1]) == 0x99 {
                3u32.into_ret()
            } else {
                0xffff_ffffu32.into_ret()
            }
        });
        e.register(ARRAY_MOVE_ITEMS, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        fill_map(&mut e, this, &[(0x10, 0x4400)]);
        e.call(0x0085_2a10, &args![this, 0x10u32, 0x99u32]);
        e.call(0x0085_2a10, &args![this, 0x10u32, 0x98u32]);
        e.call(0x0085_2a10, &args![this, 0x11u32, 0x99u32]);
        assert_eq!(calls(&e, ARRAY_FIND).len(), 2);
        assert_eq!(calls(&e, ARRAY_FIND)[0][0], 0x4400);
        assert_eq!(calls(&e, ARRAY_FIND)[0][2..], [0, ARRAY_COMPARE]);
        assert_eq!(calls(&e, ARRAY_MOVE_ITEMS), vec![vec![0x4400, 3, 1]]);
    }

    #[test]
    fn fn_00852c00_adds_to_an_existing_or_new_map() {
        let mut e = engine();
        e.register(KEY_FOR_WORLD_COORD, |_, a| (a[0] + 1).into_ret());
        e.register(ARRAY_MAP_ADD, |_, _| Ret::default());
        e.register(MAP_SET_AT, |_, _| Ret::default());
        let this = e.mem.alloc(0x30);
        fill_map(&mut e, this + 0x20, &[(7, 0x4400)]);
        e.call(0x0085_2c00, &args![this, 7u32, 0x55u32, 0x30u32]);
        assert_eq!(calls(&e, ARRAY_MAP_ADD), vec![vec![0x4400, 0x31, 0x55]]);
        assert!(calls(&e, MAP_SET_AT).is_empty());
        // A new map (built by `00852810` on the new block).
        let made = e.mem.alloc(0x10);
        e.register_double(OPERATOR_NEW, move |_, _| made.into_ret());
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MEMSET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0085_2c00, &args![this, 8u32, 0x56u32, 0x40u32]);
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![0x10]]);
        assert_eq!(e.mem.u32(made), 0x0107_fef4);
        assert_eq!(calls(&e, MAP_SET_AT), vec![vec![this + 0x20, 8, made]]);
        assert_eq!(calls(&e, ARRAY_MAP_ADD), vec![vec![made, 0x41, 0x56]]);
    }

    #[test]
    fn remove_reference_through_the_world_map() {
        let mut e = engine();
        e.register(KEY_FOR_WORLD_COORD, |_, a| (a[0] + 1).into_ret());
        e.register(ARRAY_FIND, |_, _| 0xffff_ffffu32.into_ret());
        let this = e.mem.alloc(0x30);
        let cells = e.mem.alloc(0x10);
        fill_map(&mut e, cells, &[(0x31, 0x4400)]);
        fill_map(&mut e, this + 0x20, &[(7, cells)]);
        e.call(0x0085_2cd0, &args![this, 7u32, 0x55u32, 0x30u32]);
        e.call(0x0085_2cd0, &args![this, 9u32, 0x55u32, 0x30u32]);
        assert_eq!(calls(&e, KEY_FOR_WORLD_COORD), vec![vec![0x30]]);
        assert_eq!(calls(&e, ARRAY_FIND)[0][0], 0x4400);
    }

    /// The changes-map entry for id 0x500: the word `first`, then the
    /// buffer word.
    fn changed(e: &mut Engine, kind: u32, buffer: u32) -> u32 {
        e.map(0x011d_d000, 0x1000);
        e.set_global::<u32>(SAVE_LOAD_GAME, 0x6000);
        let entry = e.mem.alloc(8);
        e.mem.set_u32(entry, 0x1234);
        e.mem.set_u32(entry + 4, buffer);
        e.register(WORD_AT, |e, a| e.mem.u32(a[0]).into_ret());
        e.map(0x6000, 0x10);
        e.mem.set_u32(0x6000, 0x7000);
        e.register_double(MAP_VALUE, move |_, a| {
            let value = if a[1] == 0x500 { entry } else { 0 };
            value.into_ret()
        });
        e.register_double(CHANGE_KIND, move |_, _| kind.into_ret());
        for a in [
            SAVE_GAME_WARNING,
            INITIAL_DATA_CONSTRUCT,
            LOAD_INITIAL_DATA,
            LOAD_FORM,
            RECONSTRUCT_ADD_REFERENCE,
            RECONSTRUCT_ADD_REFERENCE_IN_WORLD,
        ] {
            e.register(a, |_, _| Ret::default());
        }
        e.register(ADVANCED_BUFFER, |_, _| 0x8800u32.into_ret());
        e.register(INITIAL_DATA_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 8, 0xaa);
            e.mem.set_u32(a[0] + 0xc, 0xbb);
            e.mem.set_u32(a[0] + 0x10, 0xcc);
            Ret::default()
        });
        entry
    }

    #[test]
    fn load_changed_reference_warns_for_other_kinds_and_missing_buffers() {
        let mut e = engine();
        changed(&mut e, 4, 0x9000);
        e.call(0x0085_2e40, &args![0u32, 0x500u32, 0x3000u32]);
        e.call(0x0085_2e40, &args![0u32, 0x501u32, 0x3000u32]);
        assert_eq!(
            calls(&e, SAVE_GAME_WARNING),
            vec![vec![WARNING_NOT_CREATED_OR_MOVED, 0x500]]
        );
        assert_eq!(calls(&e, CHANGE_KIND), vec![vec![0x500, 0x3a, 0x1234]]);
        let mut e = engine();
        changed(&mut e, 5, 0);
        e.call(0x0085_2e40, &args![0u32, 0x500u32, 0x3000u32]);
        assert_eq!(
            calls(&e, SAVE_GAME_WARNING),
            vec![vec![WARNING_NO_UNLOADED_BUFFER, 0x500]]
        );
    }

    #[test]
    fn load_changed_reference_creates_and_loads_a_created_reference() {
        let mut e = engine();
        let entry = changed(&mut e, 5, 0x9000);
        e.register(CREATE_REFERENCE, |_, _| 0x4321u32.into_ret());
        e.call(0x0085_2e40, &args![0u32, 0x500u32, 0x3000u32]);
        assert_eq!(calls(&e, ADVANCED_BUFFER), vec![vec![entry + 4]]);
        let load = calls(&e, LOAD_INITIAL_DATA);
        assert_eq!(load[0][..2], [0x8800, 5]);
        let data = calls(&e, INITIAL_DATA_CONSTRUCT)[0][0];
        assert_eq!(load[0][2], data);
        assert_eq!(calls(&e, CREATE_REFERENCE), vec![vec![0x500, data, 0]]);
        assert_eq!(calls(&e, LOAD_FORM), vec![vec![0x6000, 0x4321]]);
        // A reference that is not created is not loaded.
        e.register(CREATE_REFERENCE, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0085_2e40, &args![0u32, 0x500u32, 0x3000u32]);
        assert!(calls(&e, LOAD_FORM).is_empty());
    }

    #[test]
    fn load_changed_reference_adds_a_moved_reference_by_cell_or_world() {
        let mut e = engine();
        changed(&mut e, 6, 0x9000);
        e.register(ORIGINAL_CELL_AND_WORLD, |e, a| {
            e.mem.set_u32(a[1], 0xce11);
            true.into_ret()
        });
        e.call(0x0085_2e40, &args![0u32, 0x500u32, 0x3000u32]);
        assert_eq!(
            calls(&e, RECONSTRUCT_ADD_REFERENCE),
            vec![vec![0x3000, 0xce11, 0x500, 0]]
        );
        e.register(ORIGINAL_CELL_AND_WORLD, |e, a| {
            e.mem.set_u32(a[2], 0x3071d);
            true.into_ret()
        });
        e.call(0x0085_2e40, &args![0u32, 0x500u32, 0x3000u32]);
        assert_eq!(
            calls(&e, RECONSTRUCT_ADD_REFERENCE_IN_WORLD),
            vec![vec![0x3000, 0x3071d, 0xbb, 0xcc, 0x500, 0]]
        );
        e.register(ORIGINAL_CELL_AND_WORLD, |_, _| false.into_ret());
        e.call(0x0085_2e40, &args![0u32, 0x500u32, 0x3000u32]);
        assert_eq!(
            calls(&e, SAVE_GAME_WARNING),
            vec![vec![WARNING_NO_CELL_OR_WORLD, 0xaa, 0x500]]
        );
    }
}
