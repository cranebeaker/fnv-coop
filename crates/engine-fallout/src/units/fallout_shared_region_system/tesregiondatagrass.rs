//! `fallout shared/region system/tesregiondatagrass.cpp` (Xbox PDB source unit), subsystem `fallout shared/region system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004f1690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataGrass` scalar deleting destructor (engine map name): runs the destructor body `004f1730` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn tes_region_data_grass_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x004f_1730, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// `004f14a0` (`thiscall`, no stack word; unit `fallout shared/region system/tesregiondata.cpp`):
/// the base destructor.
const REGION_DATA_DESTRUCTOR: u32 = 0x004f_14a0;

// Translated from 004f1730 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004f1730 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00ef2f08, state variable) is not translated
/// Unnamed in the engine map (unit `tesregiondatagrass.cpp`, `thiscall`, no stack word): the
/// `TESRegionDataGrass` destructor run by its scalar deleting destructor. Sets the vtable
/// `01023b24`; when the object at +8 is set, calls its vtable slot 0 with 1; then the base
/// destructor `004f14a0`. The exe tests the pointer twice (the second test is the same and
/// cannot fail).
pub fn fn_004f1730(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0102_3b24);
    if e.mem.u32(this.addr() + 8) != 0 {
        let list = e.mem.u32(this.addr() + 8);
        if list != 0 {
            e.vcall(list, 0, &args![1u32]);
        }
    }
    e.call(REGION_DATA_DESTRUCTOR, &args![this]);
}

use crate::units::fallout_shared_region_system::tesregiondata::TESRegionData;

/// `TESRegionDataGrass`'s vtable (`01023b24`, the one its destructor stores).
const TES_REGION_DATA_GRASS_VTABLE: u32 = 0x0102_3b24;
/// The `TESRegionData` copy constructor (`004f1460`, `thiscall`, `RET 4`).
const REGION_DATA_COPY_CONSTRUCTOR: u32 = 0x004f_1460;
/// `TESRegionData::Save` (`004f14c0`, engine map; no stack word).
const REGION_DATA_SAVE: u32 = 0x004f_14c0;
/// `004f1540` (unnamed in the engine map; no stack word): `TESRegionData::bOverride` (+4).
const REGION_DATA_GET_OVERRIDE: u32 = 0x004f_1540;
/// `004f15a0` (unnamed in the engine map; `RET 4`): stores its byte in `bOverride`.
const REGION_DATA_SET_OVERRIDE: u32 = 0x004f_15a0;
/// `004f15c0` (unnamed in the engine map; `RET 4`): stores its byte in
/// `cPriority` when it is at most 100.
const REGION_DATA_SET_PRIORITY: u32 = 0x004f_15c0;
/// `005bb4d0` (unnamed in the engine map; no stack word): the byte at +6
/// (`TESRegionData::cPriority`, Xbox PDB).
const REGION_DATA_GET_PRIORITY: u32 = 0x005b_b4d0;
/// `0044edb0` (no stack word; the engine map's name
/// `BaseProcess::GetCurrentProcedureIndex` is a folded body): the grass
/// object list's count.
const LIST_COUNT_0044EDB0: u32 = 0x0044_edb0;
/// `004f5ec0` (unnamed in the engine map; `thiscall`, `RET 4`): adds an
/// object to the grass object list.
const LIST_ADD_004F5EC0: u32 = 0x004f_5ec0;
/// `004f5ef0` (unnamed in the engine map; no stack word): empties the grass
/// object list.
const LIST_CLEAR_004F5EF0: u32 = 0x004f_5ef0;
/// `004f6060` (unnamed in the engine map; `thiscall`, `RET 4`): copies the
/// list into the list given (a new list for null) and returns it.
const LIST_COPY_004F6060: u32 = 0x004f_6060;
/// `004f61d0` (unnamed in the engine map; `thiscall`, `RET 0xC`): blends
/// the list with another list, each with its weight.
const LIST_BLEND_004F61D0: u32 = 0x004f_61d0;
/// `006815c0` (unnamed in the engine map; no stack word): the address of a
/// list node's item.
const NODE_ITEM_006815C0: u32 = 0x0068_15c0;
/// `00726070` (unnamed in the engine map; no stack word): the node after a
/// list node.
const NODE_NEXT_00726070: u32 = 0x0072_6070;
/// `0084e3a0` (unnamed in the engine map; no stack word): the word at +0xC
/// of a form (`TESForm`'s form id on PC).
const FORM_ID_0084E3A0: u32 = 0x0084_e3a0;
/// `00401500` (unnamed in the engine map): the byte at `011c54ba`.
const SWAP_IDS_00401500: u32 = 0x0040_1500;
/// `00462230` (unnamed in the engine map; no stack word): converts both
/// words of an 8-byte record through `00401080`.
const CONVERT_PAIR_00462230: u32 = 0x0046_2230;
/// `TESForm::__AddChunkData` (`00485990`, engine map; cdecl: type, data, size).
const ADD_CHUNK_DATA: u32 = 0x0048_5990;
/// `TESFile::GetTESChunk` (`004726b0`, Xbox PDB).
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `00401660` (unnamed in the engine map): the current chunk's size.
const CHUNK_SIZE_00401660: u32 = 0x0040_1660;
/// `TESFile::GetChunkData_ov3` (`00472890`, Xbox PDB; `thiscall`, buffer and size).
const GET_CHUNK_DATA: u32 = 0x0047_2890;
/// `00401680` (unnamed in the engine map; no stack word): a byte flag of the file.
const FILE_FLAG_00401680: u32 = 0x0040_1680;
/// `00891170` (unnamed in the engine map; no stack word): the file's name.
const FILE_NAME_00891170: u32 = 0x0089_1170;
/// `005b5e40` (unnamed in the engine map; cdecl, format and arguments): prints a warning.
const PRINT_WARNING_005B5E40: u32 = 0x005b_5e40;
/// `"MODELS: Invalid Region Grass Object Data in file \"%s\"."`.
const INVALID_GRASS_DATA_FORMAT: u32 = 0x0102_3b50;
/// `TESRegionGrassObject::TESRegionGrassObject` (`004f5620`, Xbox PDB;
/// `thiscall`, `RET 4`: the 8-byte record).
const GRASS_OBJECT_CONSTRUCTOR: u32 = 0x004f_5620;
/// `00406cc0` (unnamed in the engine map; cdecl, a `float`): returns a value in ST0.
const FLOAT_HELPER_00406CC0: u32 = 0x0040_6cc0;
/// `0.005` (`double`).
const HALF_PERCENT: u32 = 0x0102_3b88;
/// `MemoryManager` allocation (`00401000`, cdecl, the size).
const ALLOCATE: u32 = 0x0040_1000;
/// `MemoryManager` deallocation (`00401030`, cdecl, the block).
const DEALLOCATE: u32 = 0x0040_1030;
/// `RDGS` (as a little-endian word), the chunk type of the grass objects.
const CHUNK_RDGS: u32 = 0x5347_4452;
/// The virtual slot (byte offset) of `GetID` (Xbox PDB `TESRegionData` slot +0x10).
const SLOT_GET_ID: u32 = 0x10;
/// The virtual slot (byte offset) of `GetObjectList` (Xbox PDB
/// `TESRegionDataGrass` slot +0x28).
const SLOT_GET_OBJECT_LIST: u32 = 0x28;
/// `TESRegionObjectBase::GetObject` (Xbox PDB slot +0x04).
const SLOT_GET_OBJECT: u32 = 0x4;
/// `TESRegionGrassObject` scalar deleting destructor (Xbox PDB slot +0x08).
const SLOT_DELETING_DESTRUCTOR: u32 = 0x8;
/// `TESRegionGrassObject::GetParentTexture` (Xbox PDB slot +0x0C).
const SLOT_GET_PARENT_TEXTURE: u32 = 0xc;
/// The `GetID` of grass region data.
const GRASS_ID: u32 = 6;

// Translated from 004f16c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004f16c0 - its FS:[0] exception frame (the unwind state that runs the base destructor when the list copy throws) is not translated
/// `TESRegionDataGrass` copy constructor (unnamed in the engine map;
/// `thiscall`, `RET 4`): the `TESRegionData` copy constructor with
/// `source`, the class's vtable, then `pObjects` (+8, Xbox PDB) becomes a
/// copy of the source's list (`004f6060` on the source's list with null).
/// Returns `this`.
pub fn fn_004f16c0(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    e.call(REGION_DATA_COPY_CONSTRUCTOR, &args![this, source]);
    e.mem.set_u32(this.addr(), TES_REGION_DATA_GRASS_VTABLE);
    // `TESRegionDataGrass::pObjects` (Xbox PDB) +0x08 of the source.
    let source_list = e.mem.u32(source.addr() + 8);
    let copy = e.call(LIST_COPY_004F6060, &args![source_list, 0u32]).u32();
    e.mem.set_u32(this.addr() + 8, copy);
    this
}

// Translated from 004f17c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataGrass::Save` (Xbox PDB): counts the object list (`0044edb0`
/// on `GetObjectList`), runs `TESRegionData::Save`, then, when there are
/// objects, writes an `RDGS` chunk of 8-byte records (each object's
/// `GetObject` form id, then its `GetParentTexture` form id or 0) built in
/// an allocated buffer of `count * 8` bytes (0xFFFFFFFF on overflow). Only
/// nodes with an item add a record. When the byte `00401500` reads is set,
/// each of the `count` records goes through `00462230` before and again
/// after the chunk is written. The buffer is freed.
pub fn tes_region_data_grass_save(e: &mut Engine, this: Ptr) {
    let list = e.vcall(this.addr(), SLOT_GET_OBJECT_LIST, &args![]).u32();
    let count = e.call(LIST_COUNT_0044EDB0, &args![list]).u32();
    e.call(REGION_DATA_SAVE, &args![this]);
    if count == 0 {
        return;
    }
    let list = e.vcall(this.addr(), SLOT_GET_OBJECT_LIST, &args![]).u32();
    let mut node = if list == 0 { 0 } else { list + 4 };
    let mut index: u32 = 0;
    let size = count.checked_mul(8).unwrap_or(u32::MAX);
    let buffer = e.call(ALLOCATE, &args![size]).u32();
    while node != 0 {
        let item_slot = e.call(NODE_ITEM_006815C0, &args![node]).u32();
        let item = e.mem.u32(item_slot);
        if item != 0 {
            let object = e.vcall(item, SLOT_GET_OBJECT, &args![]).u32();
            let object_id = e.call(FORM_ID_0084E3A0, &args![object]).u32();
            e.mem
                .set_u32(buffer.wrapping_add(index.wrapping_mul(8)), object_id);
            let texture_id = if e.vcall(item, SLOT_GET_PARENT_TEXTURE, &args![]).u32() == 0 {
                0
            } else {
                let texture = e.vcall(item, SLOT_GET_PARENT_TEXTURE, &args![]).u32();
                e.call(FORM_ID_0084E3A0, &args![texture]).u32()
            };
            e.mem.set_u32(
                buffer.wrapping_add(index.wrapping_mul(8)).wrapping_add(4),
                texture_id,
            );
            index = index.wrapping_add(1);
        }
        node = e.call(NODE_NEXT_00726070, &args![node]).u32();
    }
    if e.call(SWAP_IDS_00401500, &args![]).bool() {
        for i in 0..count {
            e.call(
                CONVERT_PAIR_00462230,
                &args![buffer.wrapping_add(i.wrapping_mul(8))],
            );
        }
    }
    e.call(ADD_CHUNK_DATA, &args![CHUNK_RDGS, buffer, count << 3]);
    if e.call(SWAP_IDS_00401500, &args![]).bool() {
        for i in 0..count {
            e.call(
                CONVERT_PAIR_00462230,
                &args![buffer.wrapping_add(i.wrapping_mul(8))],
            );
        }
    }
    e.call(DEALLOCATE, &args![buffer]);
}

// Translated from 004f1980 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004f1980 - its FS:[0] exception frame (the unwind state that frees the object block when its constructor throws) is not translated
/// `TESRegionDataGrass::LoadRegionDataGrass` (Xbox PDB; `thiscall`, `RET 4`):
/// false for a null `file` or when the current chunk is not `RDGS`; false
/// with the warning `"MODELS: Invalid Region Grass Object Data in file
/// \"%s\"."` (the file's name, `00891170`) when the chunk's size is not a
/// multiple of 8. Otherwise, for a non-empty chunk, the chunk is read into
/// an allocated buffer and each 8-byte record (first converted through
/// `00462230` when the file's flag `00401680` is set) becomes a
/// `TESRegionGrassObject` (0xC bytes, `004f5620`): one whose `GetObject`
/// is set is added to `GetObjectList` (`004f5ec0`), any other is deleted
/// (slot +8 with 1). The buffer is freed; returns true.
pub fn tes_region_data_grass_load_region_data_grass(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    if file.is_null() || e.call(GET_TES_CHUNK, &args![file]).u32() != CHUNK_RDGS {
        return false;
    }
    let size = e.call(CHUNK_SIZE_00401660, &args![file]).u32();
    let count = size >> 3;
    if size % 8 != 0 {
        let name = e.call(FILE_NAME_00891170, &args![file]).u32();
        e.call(
            PRINT_WARNING_005B5E40,
            &args![INVALID_GRASS_DATA_FORMAT, name],
        );
        return false;
    }
    if count == 0 {
        return true;
    }
    let buffer = e
        .call(ALLOCATE, &args![count.checked_mul(8).unwrap_or(u32::MAX)])
        .u32();
    e.call(GET_CHUNK_DATA, &args![file, buffer, size]);
    for i in 0..count {
        let record = buffer.wrapping_add(i.wrapping_mul(8));
        if e.call(FILE_FLAG_00401680, &args![file]).bool() {
            e.call(CONVERT_PAIR_00462230, &args![record]);
        }
        let block = e.call(ALLOCATE, &args![0xcu32]).u32();
        let object = if block == 0 {
            0
        } else {
            e.call(GRASS_OBJECT_CONSTRUCTOR, &args![block, record])
                .u32()
        };
        if object != 0 {
            if e.vcall(object, SLOT_GET_OBJECT, &args![]).u32() != 0 {
                let list = e.vcall(this.addr(), SLOT_GET_OBJECT_LIST, &args![]).u32();
                e.call(LIST_ADD_004F5EC0, &args![list, object]);
            } else {
                e.vcall(object, SLOT_DELETING_DESTRUCTOR, &args![1u32]);
            }
        }
    }
    e.call(DEALLOCATE, &args![buffer]);
    true
}

// Translated from 004f1b60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004f1b60 - its FS:[0] exception frame (the unwind state that frees the block when the copy constructor throws) is not translated
/// `TESRegionDataGrass::Copy` (Xbox PDB): allocates 0xC bytes and, when that
/// succeeds, copy-constructs a `TESRegionDataGrass` of `this` in them
/// (`004f16c0`). Returns the copy, or null.
pub fn tes_region_data_grass_copy(e: &mut Engine, this: Ptr) -> Ptr {
    let block = e.call(ALLOCATE, &args![0xcu32]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    fn_004f16c0(e, Ptr::new(block), this)
}

/// The tail every "take the source" branch of `BlendInto` shares: the
/// source's `bOverride` and priority, then this list emptied and the
/// source's list copied into it.
fn blend_take_source(e: &mut Engine, this: Ptr, source: Ptr) {
    let flag = e.call(REGION_DATA_GET_OVERRIDE, &args![source]).u8();
    e.call(REGION_DATA_SET_OVERRIDE, &args![this, flag as u32]);
    let priority = e.call(REGION_DATA_GET_PRIORITY, &args![source]).u8();
    e.call(REGION_DATA_SET_PRIORITY, &args![this, priority as u32]);
    let list = e.vcall(this.addr(), SLOT_GET_OBJECT_LIST, &args![]).u32();
    e.call(LIST_CLEAR_004F5EF0, &args![list]);
    let target = e.vcall(this.addr(), SLOT_GET_OBJECT_LIST, &args![]).u32();
    let source_list = e.vcall(source.addr(), SLOT_GET_OBJECT_LIST, &args![]).u32();
    e.call(LIST_COPY_004F6060, &args![source_list, target]);
}

/// `005bb4d0` on an object, as the `MOVZX` the exe does.
fn priority(e: &mut Engine, data: Ptr) -> i32 {
    e.call(REGION_DATA_GET_PRIORITY, &args![data]).u8() as i32
}

// Translated from 004f1be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataGrass::BlendInto` (Xbox PDB; `thiscall`, `RET 8`): merges
/// grass data `source` (its `GetID` must be 6, and `enabled` nonzero) into
/// this one. When this data's `bIgnore` is set, the source is taken (its
/// `bOverride` and priority, and this list replaced by a copy of the
/// source's). Otherwise nothing happens when the source's `bIgnore` is set.
/// With this data's `bOverride` set, the source is taken only when its
/// `bOverride` is set and its priority is higher. Otherwise the source is
/// taken when its `bOverride` is set; else the lists are blended
/// (`004f61d0` with this priority, the source's list and the source's
/// priority, as floats) and the priority becomes
/// `trunc(00406cc0((this*this + (100-this)*src + (100-src)*this + src*src) * 0.005))`
/// (each priority read anew, in the exe's order).
pub fn tes_region_data_grass_blend_into(e: &mut Engine, this: Ptr, source: Ptr, enabled: u32) {
    if source.is_null()
        || e.vcall(source.addr(), SLOT_GET_ID, &args![]).u32() != GRASS_ID
        || enabled == 0
    {
        return;
    }
    let this_data: Ptr<TESRegionData> = this.cast();
    let source_data: Ptr<TESRegionData> = source.cast();
    if e.get(this_data, TESRegionData::bIgnore) != 0 {
        blend_take_source(e, this, source);
        return;
    }
    if e.get(source_data, TESRegionData::bIgnore) != 0 {
        return;
    }
    if e.get(this_data, TESRegionData::bOverride) != 0 {
        if e.call(REGION_DATA_GET_OVERRIDE, &args![source]).u8() != 0
            && priority(e, source) > priority(e, this)
        {
            blend_take_source(e, this, source);
        }
        return;
    }
    if e.call(REGION_DATA_GET_OVERRIDE, &args![source]).u8() != 0 {
        blend_take_source(e, this, source);
        return;
    }
    let source_weight = x87_store32(f64::from(priority(e, source)));
    let source_list = e.vcall(source.addr(), SLOT_GET_OBJECT_LIST, &args![]).u32();
    let this_weight = x87_store32(f64::from(priority(e, this)));
    let list = e.vcall(this.addr(), SLOT_GET_OBJECT_LIST, &args![]).u32();
    e.call(
        LIST_BLEND_004F61D0,
        &args![list, this_weight, source_list, source_weight],
    );
    let a = priority(e, this);
    let b = priority(e, this);
    let c = priority(e, this);
    let d = priority(e, source);
    let first = a * b + (100 - c) * d;
    let mut total = x87_store32(f64::from(first));
    let f = priority(e, source);
    let g = priority(e, this);
    let h = priority(e, source);
    let i = priority(e, source);
    let second = (100 - f) * g + h * i;
    total = x87_store32(x87_add(f64::from(second), x87_m32(total)));
    let half_percent: f64 = e.global(HALF_PERCENT);
    total = x87_store32(x87_mul(x87_load32(total), half_percent));
    let rounded = e.call(FLOAT_HELPER_00406CC0, &args![total]).f64();
    let value = crate::x87::fistp_chop(rounded) as u8;
    e.call(REGION_DATA_SET_PRIORITY, &args![this, value as u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004f_1690, tes_region_data_grass_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x004f_1730, fn_004f1730(Ptr)),
        entry!(0x004f_16c0, fn_004f16c0(Ptr, Ptr) -> Ptr),
        entry!(0x004f_17c0, tes_region_data_grass_save(Ptr)),
        entry!(0x004f_1980, tes_region_data_grass_load_region_data_grass(Ptr, Ptr) -> bool),
        entry!(0x004f_1b60, tes_region_data_grass_copy(Ptr) -> Ptr),
        entry!(0x004f_1be0, tes_region_data_grass_blend_into(Ptr, Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_004f1690_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x004f_1730, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x004f_1690, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x004f_1690)
                .collect();
            let mut expected = vec![(0x004f_1730, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod tests_b043 {
    use super::*;

    #[test]
    fn fn_004f1730_destroys_the_held_list_then_the_base() {
        let mut e = Engine::new();
        e.register(REGION_DATA_DESTRUCTOR, |_, _| Ret::default());
        e.put_vtable(0x0150_0000, &[0x0150_1000]);
        e.register(0x0150_1000, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        let list = e.mem.alloc(4);
        e.mem.set_u32(list, 0x0150_0000);
        e.mem.set_u32(this + 8, list);
        e.call_log = Some(vec![]);
        e.call(0x004f_1730, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (0x0150_1000, vec![list, 1]),
                (REGION_DATA_DESTRUCTOR, vec![this]),
            ]
        );
        assert_eq!(e.mem.u32(this), 0x0102_3b24);
        e.mem.set_u32(this + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x004f_1730, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(REGION_DATA_DESTRUCTOR, vec![this])]);
    }
}

#[cfg(test)]
mod tests_n073 {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Default::default()
        }
    }

    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// Doubles for the list walk (a node is two words: item, next), the
    /// form id and the allocator.
    fn list_doubles(e: &mut Engine) {
        e.register(NODE_ITEM_006815C0, |_, a| ret(a[0]));
        e.register(NODE_NEXT_00726070, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(FORM_ID_0084E3A0, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(DEALLOCATE, |_, _| Ret::default());
    }

    /// A vtable at `0x0150_0200` whose slot 0x28 (`GetObjectList`) is `0x0150_100c`.
    fn object_list_vtable(e: &mut Engine) {
        let mut slots = vec![0u32; 10];
        slots.push(0x0150_100c);
        e.put_vtable(0x0150_0200, &slots);
    }

    #[test]
    fn fn_004f16c0_copies_the_base_and_the_list() {
        let mut e = Engine::new();
        e.register(REGION_DATA_COPY_CONSTRUCTOR, |_, _| Ret::default());
        e.register(LIST_COPY_004F6060, |_, _| ret(0x5000));
        let this: Ptr = Ptr::new(e.mem.alloc(0xc));
        let source: Ptr = Ptr::new(e.mem.alloc(0xc));
        e.mem.set_u32(source.addr() + 8, 0x4444);
        e.call_log = Some(vec![]);
        let r = e.call(0x004f_16c0, &args![this, source]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(r, this.addr());
        assert_eq!(e.mem.u32(this.addr()), TES_REGION_DATA_GRASS_VTABLE);
        assert_eq!(e.mem.u32(this.addr() + 8), 0x5000);
        assert_eq!(
            calls_to(&log, REGION_DATA_COPY_CONSTRUCTOR),
            vec![vec![this.addr(), source.addr()]]
        );
        assert_eq!(calls_to(&log, LIST_COPY_004F6060), vec![vec![0x4444, 0]]);
    }

    #[test]
    fn save_writes_one_record_per_item_and_converts_when_asked() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        e.register(REGION_DATA_SAVE, |_, _| Ret::default());
        e.register(LIST_COUNT_0044EDB0, |_, _| ret(2));
        e.register(SWAP_IDS_00401500, |_, _| ret(1));
        e.register(CONVERT_PAIR_00462230, |_, _| Ret::default());
        let written = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let w = written.clone();
        e.register_double(ADD_CHUNK_DATA, move |e, a| {
            w.borrow_mut().push((a[0], a[2], e.mem.bytes(a[1], 16)));
            Ret::default()
        });
        let grass = e.mem.alloc(0x10);
        e.mem.set_u32(grass + 0xc, 0x111);
        let texture = e.mem.alloc(0x10);
        e.mem.set_u32(texture + 0xc, 0x222);
        // Item vtables: slot 4 GetObject, slot 0xC GetParentTexture.
        e.put_vtable(0x0150_0000, &[0, 0x0150_1000, 0, 0x0150_1004]);
        e.put_vtable(0x0150_0100, &[0, 0x0150_1000, 0, 0x0150_1008]);
        e.register_double(0x0150_1000, move |_, _| ret(grass));
        e.register_double(0x0150_1004, move |_, _| ret(texture));
        e.register(0x0150_1008, |_, _| ret(0));
        let item_a = e.mem.alloc(4);
        e.mem.set_u32(item_a, 0x0150_0000);
        let item_b = e.mem.alloc(4);
        e.mem.set_u32(item_b, 0x0150_0100);
        // The list's first node is at +4: item_a, then a node without an item, then item_b.
        let list = e.mem.alloc(0x14);
        let node2 = e.mem.alloc(8);
        let node3 = e.mem.alloc(8);
        e.mem.set_u32(list + 4, item_a);
        e.mem.set_u32(list + 8, node2);
        e.mem.set_u32(node2 + 4, node3);
        e.mem.set_u32(node3, item_b);
        object_list_vtable(&mut e);
        e.register_double(0x0150_100c, move |_, _| ret(list));
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this, 0x0150_0200);
        e.call_log = Some(vec![]);
        e.call(0x004f_17c0, &args![this]);
        let log = e.call_log.take().unwrap();
        let w = written.borrow();
        assert_eq!(w.len(), 1);
        assert_eq!(w[0].0, CHUNK_RDGS);
        assert_eq!(w[0].1, 16);
        let words: Vec<u32> = w[0]
            .2
            .chunks(4)
            .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]))
            .collect();
        assert_eq!(words, vec![0x111, 0x222, 0x111, 0]);
        assert_eq!(calls_to(&log, CONVERT_PAIR_00462230).len(), 4);
        assert_eq!(calls_to(&log, REGION_DATA_SAVE), vec![vec![this]]);
        assert_eq!(calls_to(&log, DEALLOCATE).len(), 1);
    }

    #[test]
    fn save_with_an_empty_list_only_saves_the_base() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        e.register(REGION_DATA_SAVE, |_, _| Ret::default());
        e.register(LIST_COUNT_0044EDB0, |_, _| ret(0));
        object_list_vtable(&mut e);
        e.register(0x0150_100c, |_, _| ret(0x7000));
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this, 0x0150_0200);
        e.call_log = Some(vec![]);
        e.call(0x004f_17c0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, LIST_COUNT_0044EDB0), vec![vec![0x7000]]);
        assert_eq!(calls_to(&log, REGION_DATA_SAVE), vec![vec![this]]);
        assert!(calls_to(&log, ALLOCATE).is_empty());
    }

    #[test]
    fn load_reads_records_keeps_objects_with_a_form_and_deletes_the_rest() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        let file = e.mem.alloc(4);
        e.register(GET_TES_CHUNK, |_, _| ret(CHUNK_RDGS));
        e.register(CHUNK_SIZE_00401660, |_, _| ret(16));
        e.register(GET_CHUNK_DATA, |e, a| {
            e.mem.set_u32(a[1], 5);
            e.mem.set_u32(a[1] + 8, 0);
            Ret::default()
        });
        e.register(FILE_FLAG_00401680, |_, _| ret(1));
        e.register(CONVERT_PAIR_00462230, |_, _| Ret::default());
        e.put_vtable(0x0150_0000, &[0, 0x0150_1000, 0x0150_1004]);
        e.register(0x0150_1000, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(0x0150_1004, |_, _| Ret::default());
        // The constructor double copies the record's first word to +4.
        e.register(GRASS_OBJECT_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], 0x0150_0000);
            let id = e.mem.u32(a[1]);
            e.mem.set_u32(a[0] + 4, id);
            ret(a[0])
        });
        e.register(LIST_ADD_004F5EC0, |_, _| Ret::default());
        object_list_vtable(&mut e);
        e.register(0x0150_100c, |_, _| ret(0x7000));
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this, 0x0150_0200);
        e.call_log = Some(vec![]);
        assert!(e.call(0x004f_1980, &args![this, file]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, CONVERT_PAIR_00462230).len(), 2);
        let adds = calls_to(&log, LIST_ADD_004F5EC0);
        assert_eq!(adds.len(), 1);
        assert_eq!(adds[0][0], 0x7000);
        let deletes = calls_to(&log, 0x0150_1004);
        assert_eq!(deletes.len(), 1);
        assert_eq!(deletes[0][1], 1);
        assert_eq!(calls_to(&log, DEALLOCATE).len(), 1);
    }

    #[test]
    fn load_refuses_other_chunks_and_warns_on_a_bad_size() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        let this = e.mem.alloc(0xc);
        assert!(!e.call(0x004f_1980, &args![this, 0u32]).bool());
        let file = e.mem.alloc(4);
        e.register(GET_TES_CHUNK, |_, _| ret(0x1234));
        assert!(!e.call(0x004f_1980, &args![this, file]).bool());
        e.register(GET_TES_CHUNK, |_, _| ret(CHUNK_RDGS));
        e.register(CHUNK_SIZE_00401660, |_, _| ret(12));
        e.register(FILE_NAME_00891170, |_, _| ret(0x9999));
        e.register(PRINT_WARNING_005B5E40, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x004f_1980, &args![this, file]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, PRINT_WARNING_005B5E40),
            vec![vec![INVALID_GRASS_DATA_FORMAT, 0x9999]]
        );
        e.register(CHUNK_SIZE_00401660, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert!(e.call(0x004f_1980, &args![this, file]).bool());
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, ALLOCATE).is_empty());
    }

    #[test]
    fn copy_allocates_and_copy_constructs() {
        let mut e = Engine::new();
        e.register(ALLOCATE, |_, _| ret(0));
        let this = e.mem.alloc(0xc);
        assert_eq!(e.call(0x004f_1b60, &args![this]).u32(), 0);
        let block = e.mem.alloc(0xc);
        e.register_double(ALLOCATE, move |_, _| ret(block));
        e.register(REGION_DATA_COPY_CONSTRUCTOR, |_, _| Ret::default());
        e.register(LIST_COPY_004F6060, |_, _| ret(0x5000));
        assert_eq!(e.call(0x004f_1b60, &args![this]).u32(), block);
        assert_eq!(e.mem.u32(block + 8), 0x5000);
    }

    /// A target and a source of GetID 6, with `GetObjectList` returning +8.
    fn blend_setup(e: &mut Engine) -> (u32, u32) {
        let slots: Vec<u32> = (0..11).map(|i| 0x0150_1000 + 4 * i).collect();
        e.put_vtable(0x0150_0000, &slots);
        e.register(0x0150_1010, |_, _| ret(GRASS_ID));
        e.register(0x0150_1028, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(REGION_DATA_GET_OVERRIDE, |e, a| {
            ret(e.mem.u8(a[0] + 4) as u32)
        });
        e.register(REGION_DATA_GET_PRIORITY, |e, a| {
            ret(e.mem.u8(a[0] + 6) as u32)
        });
        e.register(REGION_DATA_SET_OVERRIDE, |e, a| {
            e.mem.set_u8(a[0] + 4, a[1] as u8);
            Ret::default()
        });
        e.register(REGION_DATA_SET_PRIORITY, |e, a| {
            e.mem.set_u8(a[0] + 6, a[1] as u8);
            Ret::default()
        });
        e.register(LIST_CLEAR_004F5EF0, |_, _| Ret::default());
        e.register(LIST_COPY_004F6060, |_, _| Ret::default());
        e.register(LIST_BLEND_004F61D0, |_, _| Ret::default());
        let this = e.mem.alloc(0xc);
        let source = e.mem.alloc(0xc);
        e.mem.set_u32(this, 0x0150_0000);
        e.mem.set_u32(source, 0x0150_0000);
        e.mem.set_u32(this + 8, 0xaaaa);
        e.mem.set_u32(source + 8, 0xbbbb);
        (this, source)
    }

    #[test]
    fn blend_into_takes_the_source_when_this_is_ignored() {
        let mut e = Engine::new();
        let (this, source) = blend_setup(&mut e);
        e.mem.set_u8(this + 5, 1);
        e.mem.set_u8(source + 4, 1);
        e.mem.set_u8(source + 6, 70);
        e.call_log = Some(vec![]);
        e.call(0x004f_1be0, &args![this, source, 1u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u8(this + 4), 1);
        assert_eq!(e.mem.u8(this + 6), 70);
        assert_eq!(calls_to(&log, LIST_CLEAR_004F5EF0), vec![vec![0xaaaa]]);
        assert_eq!(
            calls_to(&log, LIST_COPY_004F6060),
            vec![vec![0xbbbb, 0xaaaa]]
        );
    }

    #[test]
    fn blend_into_ignores_bad_sources() {
        let mut e = Engine::new();
        let (this, source) = blend_setup(&mut e);
        e.mem.set_u8(this + 5, 1);
        e.call_log = Some(vec![]);
        e.call(0x004f_1be0, &args![this, source, 0u32]);
        e.call(0x004f_1be0, &args![this, 0u32, 1u32]);
        e.register(0x0150_1010, |_, _| ret(5));
        e.call(0x004f_1be0, &args![this, source, 1u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, LIST_COPY_004F6060).is_empty());
    }

    #[test]
    fn blend_into_skips_an_ignored_source_and_respects_override() {
        let mut e = Engine::new();
        let (this, source) = blend_setup(&mut e);
        e.mem.set_u8(source + 5, 1);
        e.mem.set_u8(source + 4, 1);
        e.call_log = Some(vec![]);
        e.call(0x004f_1be0, &args![this, source, 1u32]);
        assert!(calls_to(&e.call_log.take().unwrap(), LIST_COPY_004F6060).is_empty());
        // This overrides: the source needs its override and a higher priority.
        e.mem.set_u8(source + 5, 0);
        e.mem.set_u8(this + 4, 1);
        e.mem.set_u8(this + 6, 50);
        e.mem.set_u8(source + 6, 50);
        e.call_log = Some(vec![]);
        e.call(0x004f_1be0, &args![this, source, 1u32]);
        assert!(calls_to(&e.call_log.take().unwrap(), LIST_COPY_004F6060).is_empty());
        e.mem.set_u8(source + 6, 51);
        e.call_log = Some(vec![]);
        e.call(0x004f_1be0, &args![this, source, 1u32]);
        assert_eq!(
            calls_to(&e.call_log.take().unwrap(), LIST_COPY_004F6060).len(),
            1
        );
        assert_eq!(e.mem.u8(this + 6), 51);
    }

    #[test]
    fn blend_into_takes_an_overriding_source_or_blends() {
        let mut e = Engine::new();
        let (this, source) = blend_setup(&mut e);
        e.mem.set_u8(source + 4, 1);
        e.mem.set_u8(source + 6, 30);
        e.call_log = Some(vec![]);
        e.call(0x004f_1be0, &args![this, source, 1u32]);
        assert_eq!(
            calls_to(&e.call_log.take().unwrap(), LIST_COPY_004F6060).len(),
            1
        );
        assert_eq!(e.mem.u8(this + 6), 30);
        // Blend: this 40, source 80.
        e.mem.set_u8(source + 4, 0);
        e.mem.set_u8(this + 4, 0);
        e.mem.set_u8(this + 6, 40);
        e.mem.set_u8(source + 6, 80);
        e.map(HALF_PERCENT & !0xfff, 0x1000);
        e.set_global(HALF_PERCENT, 0.005f64);
        e.register(FLOAT_HELPER_00406CC0, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])),
            ..Default::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x004f_1be0, &args![this, source, 1u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, LIST_BLEND_004F61D0),
            vec![vec![0xaaaa, 40f32.to_bits(), 0xbbbb, 80f32.to_bits()]]
        );
        // (40*40 + 60*80 + 20*40 + 80*80) * 0.005 = 68.
        assert_eq!(e.mem.u8(this + 6), 68);
    }
}
