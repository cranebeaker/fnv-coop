//! `fallout shared/region system/tesregiondataimposter.cpp` (Xbox PDB source unit), subsystem `fallout shared/region system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004f1f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataImposter` scalar deleting destructor (engine map name): runs the destructor body `004f2040` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn tes_region_data_imposter_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x004f_2040, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// The vtable of `TESRegionDataImposter` (RTTI `.?AVTESRegionDataImposter@@`, read at the word before it).
const TES_REGION_DATA_IMPOSTER_VTABLE: u32 = 0x0102_3bc8;
/// `004f14a0`: the `TESRegionData` destructor body.
const TES_REGION_DATA_DTOR: u32 = 0x004f_14a0;
/// `00413f40` (unnamed in the engine map): returns its `this` + 8 (here the
/// imposter list `m_Imposters`, Xbox PDB).
const LIST_OF_00413F40: u32 = 0x0041_3f40;
/// `00470470` (unnamed in the engine map): run on the imposter list by the
/// list clear `004f2370`.
const LIST_CLEAR_00470470: u32 = 0x0047_0470;
/// `0046ffb0` (unnamed in the engine map): the destructor run on the list at +8.
const LIST_DTOR_0046FFB0: u32 = 0x0046_ffb0;
/// `005f65d0` (unnamed in the engine map; `thiscall`, `RET 4`): asked with
/// the address of the reference whether the list already holds it.
const LIST_CONTAINS_005F65D0: u32 = 0x005f_65d0;
/// `00905820` (unnamed in the engine map; `thiscall`, `RET 4`): adds the
/// reference whose address it is given to the list at +8.
const LIST_ADD_00905820: u32 = 0x0090_5820;
/// `TESFile::GetTESChunk` (`004726b0`, Xbox PDB).
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `00401660` (unnamed in the engine map): the file's word at +0x25C (the
/// current chunk's size).
const CHUNK_SIZE_00401660: u32 = 0x0040_1660;
/// `TESFile::GetChunkData_ov3` (`00472890`, Xbox PDB; `thiscall`, buffer and size).
const GET_CHUNK_DATA: u32 = 0x0047_2890;
/// `00401680` (unnamed in the engine map): the file's byte at +0x299.
const FILE_FLAG_00401680: u32 = 0x0040_1680;
/// `00401080` (unnamed in the engine map; cdecl, the address of a form id
/// and a byte): called on each id read when the file's byte at +0x299 is set.
const FIX_ID_00401080: u32 = 0x0040_1080;
/// `MemoryManager` allocation (`00401000`, cdecl, the size).
const ALLOCATE: u32 = 0x0040_1000;
/// `MemoryManager` deallocation (`00401030`, cdecl, the block).
const DEALLOCATE: u32 = 0x0040_1030;
/// `RDID` (as a little-endian word), the chunk type of the imposter reference ids.
const CHUNK_RDID: u32 = 0x4449_4452;
/// `006815c0` (unnamed in the engine map): returns its `this` (a list
/// node; the item is the node's first word).
const NODE_ITEM_006815C0: u32 = 0x0068_15c0;
/// `00726070` (unnamed in the engine map): the node's next (word at +4).
const NODE_FOLLOWING_00726070: u32 = 0x0072_6070;
/// `TESForm::GetFile` (`00484e60`, Xbox PDB; `thiscall`, one word).
const TES_FORM_GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex` (`00485d50`, Xbox PDB; cdecl, the address of
/// a form id and a file).
const TES_FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// `004839c0` (unnamed in the engine map; cdecl, one word): given the id,
/// returns the form the cast then checks.
const LOOKUP_FORM_004839C0: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The cast's source type descriptor (`.?AVTESForm@@`).
const CAST_SOURCE_TYPE: u32 = 0x0118_3028;
/// The cast's target type descriptor (`.?AVTESObjectREFR@@`).
const CAST_TARGET_TYPE: u32 = 0x0118_41cc;
/// `00726c60` (unnamed in the engine map; `thiscall`, `RET 4`): given the
/// address of a word, used on the node with the cast result.
const NODE_SET_ITEM_00726C60: u32 = 0x0072_6c60;
/// `0063f7b0` (unnamed in the engine map): run on the node whose item did
/// not resolve.
const NODE_REMOVE_0063F7B0: u32 = 0x0063_f7b0;

// Translated from 004f2040 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004f2040 - its FS:[0] exception frame (the unwind states of the members' destructors) is not translated
/// `TESRegionDataImposter` destructor body (unnamed in the engine map):
/// stores the class's vtable, clears the imposter list (`004f2370`), runs
/// the list's destructor (`0046ffb0` on +8), then the `TESRegionData`
/// destructor body (`004f14a0`).
pub fn fn_004f2040(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), TES_REGION_DATA_IMPOSTER_VTABLE);
    fn_004f2370(e, this);
    e.call(LIST_DTOR_0046FFB0, &args![this.addr() + 8]);
    e.call(TES_REGION_DATA_DTOR, &args![this]);
}

// Translated from 004f2370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: runs `00470470` on the imposter list (the
/// address `00413f40` returns for `this`) and returns true.
pub fn fn_004f2370(e: &mut Engine, this: Ptr) -> bool {
    let list = e.call(LIST_OF_00413F40, &args![this]).u32();
    e.call(LIST_CLEAR_00470470, &args![list]);
    true
}

// Translated from 004f2390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): adds `reference` to
/// the imposter list unless it is there already. Both list calls get the
/// address of the argument's own stack slot: `005f65d0` on the list
/// `00413f40` returns says whether it is present; when it is not,
/// `00905820` adds it to the list at +8 and the function returns true;
/// otherwise false.
pub fn fn_004f2390(e: &mut Engine, this: Ptr, reference: u32) -> bool {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), reference);
        let list = e.call(LIST_OF_00413F40, &args![this]).u32();
        if e.call(LIST_CONTAINS_005F65D0, &args![list, slot]).u8() != 0 {
            return false;
        }
        e.call(LIST_ADD_00905820, &args![this.addr() + 8, slot]);
        true
    })
}

// Translated from 004f2190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataImposter::LoadRegionDataImposter` (Xbox PDB; `thiscall`,
/// `RET 4`): false for a null `file` or when the current chunk
/// (`TESFile::GetTESChunk`) is not `RDID`. Otherwise it reads the chunk
/// (size from `00401660`) into a block of `size / 4` words (allocated with
/// the compiler's overflow check: a product that overflows asks for
/// 0xFFFFFFFF bytes), and for each word: when the file's byte `00401680` is
/// set, `00401080` adjusts the id in place (with a zero byte), then the id
/// is added through `004f2390`. The block is freed and true returned.
pub fn tes_region_data_imposter_load_region_data_imposter(
    e: &mut Engine,
    this: Ptr,
    file: Ptr,
) -> bool {
    if file.is_null() {
        return false;
    }
    if e.call(GET_TES_CHUNK, &args![file]).u32() != CHUNK_RDID {
        return false;
    }
    let size = e.call(CHUNK_SIZE_00401660, &args![file]).u32();
    let count = size / 4;
    let bytes = count.checked_mul(4).unwrap_or(u32::MAX);
    let ids = e.call(ALLOCATE, &args![bytes]).u32();
    e.call(GET_CHUNK_DATA, &args![file, ids, size]);
    e.with_stack(4, |e, slot| {
        for index in 0..count {
            let id = e.mem.u32(ids.wrapping_add(index.wrapping_mul(4)));
            e.mem.set_u32(slot.addr(), id);
            if e.call(FILE_FLAG_00401680, &args![file]).u8() != 0 {
                e.call(FIX_ID_00401080, &args![slot, 0u32]);
            }
            let id = e.mem.u32(slot.addr());
            fn_004f2390(e, this, id);
        }
    });
    e.call(DEALLOCATE, &args![ids]);
    true
}

// Translated from 004f2290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataImposter::InitItem` (Xbox PDB; `thiscall`, `RET 4`):
/// resolves the imposter list's form ids. From the list's first node
/// (`00413f40`): a node whose item is non-zero has the item (an id) given
/// its compile index from `file`'s `TESForm::GetFile(-1)` (no file when
/// `file` is null), looked up (`004839c0`) and dynamically cast, and the
/// cast result goes to the node (`00726c60`). While the last cast result
/// (kept from node to node; it starts null) is null, `0063f7b0` runs on the
/// node and the same node is looked at again unless its item is now null,
/// which ends the walk; otherwise the walk moves to the next node, and ends
/// after the last.
pub fn tes_region_data_imposter_init_item(e: &mut Engine, this: Ptr, file: Ptr) {
    // The two locals whose addresses the calls get: the cast result
    // (EBP-0xC) and the id (EBP-8).
    e.with_stack(8, |e, locals| {
        let cast_slot = locals.addr();
        let id_slot = locals.addr() + 4;
        e.mem.set_u32(cast_slot, 0);
        let mut node = e.call(LIST_OF_00413F40, &args![this]).u32();
        loop {
            if node == 0 {
                return;
            }
            let item = e.call(NODE_ITEM_006815C0, &args![node]).u32();
            if e.mem.u32(item) != 0 {
                let item = e.call(NODE_ITEM_006815C0, &args![node]).u32();
                let id = e.mem.u32(item);
                e.mem.set_u32(id_slot, id);
                let owner = if file.is_null() {
                    0
                } else {
                    e.call(TES_FORM_GET_FILE, &args![file, 0xffff_ffffu32])
                        .u32()
                };
                e.call(TES_FORM_ADD_COMPILE_INDEX, &args![id_slot, owner]);
                let id = e.mem.u32(id_slot);
                let form = e.call(LOOKUP_FORM_004839C0, &args![id]).u32();
                let cast = e
                    .call(
                        RT_DYNAMIC_CAST,
                        &args![form, 0u32, CAST_SOURCE_TYPE, CAST_TARGET_TYPE, 0u32],
                    )
                    .u32();
                e.mem.set_u32(cast_slot, cast);
                e.call(NODE_SET_ITEM_00726C60, &args![node, cast_slot]);
            }
            if e.mem.u32(cast_slot) == 0 {
                e.call(NODE_REMOVE_0063F7B0, &args![node]);
                let item = e.call(NODE_ITEM_006815C0, &args![node]).u32();
                if e.mem.u32(item) == 0 {
                    return;
                }
            } else {
                node = e.call(NODE_FOLLOWING_00726070, &args![node]).u32();
            }
        }
    });
}

/// The `TESRegionData` copy constructor (`004f1460`, `thiscall`, `RET 4`).
const REGION_DATA_COPY_CONSTRUCTOR: u32 = 0x004f_1460;
/// `TESRegionData::Save` (`004f14c0`, engine map; no stack word).
const REGION_DATA_SAVE: u32 = 0x004f_14c0;
/// `0096a2d0` (unnamed in the engine map; no stack word): constructs the
/// imposter list at +8.
const LIST_CONSTRUCTOR_0096A2D0: u32 = 0x0096_a2d0;
/// `005ae380` (no stack word; the engine map's name `VATS::GetCount` is a
/// folded body): the list's count.
const LIST_COUNT_005AE380: u32 = 0x005a_e380;
/// `0084e3a0` (unnamed in the engine map; no stack word): the word at +0xC
/// of a form (`TESForm`'s form id on PC).
const FORM_ID_0084E3A0: u32 = 0x0084_e3a0;
/// `TESForm::AddChunkArray_ov2` (`00485710`, engine map; cdecl: type,
/// words, count).
const ADD_CHUNK_ARRAY: u32 = 0x0048_5710;
/// The virtual slot (byte offset) of `GetID` (Xbox PDB `TESRegionData` slot +0x10).
const SLOT_GET_ID: u32 = 0x10;
/// The `GetID` of imposter region data.
const IMPOSTER_ID: u32 = 8;

// Translated from 004f1f70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004f1f70 - its FS:[0] exception frame (the unwind states that destroy the list and the base when a call throws) is not translated
/// `TESRegionDataImposter` copy constructor (unnamed in the engine map;
/// `thiscall`, `RET 4`): the `TESRegionData` copy constructor with
/// `source`, the class's vtable, the list at +8 constructed (`0096a2d0`)
/// and cleared (`00470470` on the list `00413f40` returns). When the
/// source's `GetID` is 8, each node of the source's list (`00413f40`) has
/// its item's address (`006815c0`) added to this list (`00905820` on +8).
/// Returns `this`.
pub fn fn_004f1f70(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    e.call(REGION_DATA_COPY_CONSTRUCTOR, &args![this, source]);
    e.mem.set_u32(this.addr(), TES_REGION_DATA_IMPOSTER_VTABLE);
    e.call(LIST_CONSTRUCTOR_0096A2D0, &args![this.addr() + 8]);
    let list = e.call(LIST_OF_00413F40, &args![this]).u32();
    e.call(LIST_CLEAR_00470470, &args![list]);
    if e.vcall(source.addr(), SLOT_GET_ID, &args![]).u32() == IMPOSTER_ID {
        let mut node = e.call(LIST_OF_00413F40, &args![source]).u32();
        while node != 0 {
            let item = e.call(NODE_ITEM_006815C0, &args![node]).u32();
            e.call(LIST_ADD_00905820, &args![this.addr() + 8, item]);
            node = e.call(NODE_FOLLOWING_00726070, &args![node]).u32();
        }
    }
    this
}

// Translated from 004f20b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataImposter::Save` (Xbox PDB): runs `TESRegionData::Save`,
/// then, when the list at +8 has a count (`005ae380`), writes an `RDID`
/// chunk of the form ids (`0084e3a0`) of the items of its first `count`
/// nodes, gathered in an allocated block of `count * 4` bytes (0xFFFFFFFF
/// on overflow; `TESForm::AddChunkArray_ov2`), and frees the block.
pub fn tes_region_data_imposter_save(e: &mut Engine, this: Ptr) {
    e.call(REGION_DATA_SAVE, &args![this]);
    let count = e.call(LIST_COUNT_005AE380, &args![this.addr() + 8]).u32();
    if count == 0 {
        return;
    }
    let mut node = this.addr() + 8;
    let ids = e
        .call(ALLOCATE, &args![count.checked_mul(4).unwrap_or(u32::MAX)])
        .u32();
    let mut index: u32 = 0;
    while node != 0 && index < count {
        let item_slot = e.call(NODE_ITEM_006815C0, &args![node]).u32();
        let item = e.mem.u32(item_slot);
        let id = e.call(FORM_ID_0084E3A0, &args![item]).u32();
        e.mem.set_u32(ids.wrapping_add(index.wrapping_mul(4)), id);
        node = e.call(NODE_FOLLOWING_00726070, &args![node]).u32();
        index += 1;
    }
    e.call(ADD_CHUNK_ARRAY, &args![CHUNK_RDID, ids, count]);
    e.call(DEALLOCATE, &args![ids]);
}

// Translated from 004f23d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004f23d0 - its FS:[0] exception frame (the unwind state that frees the block when the copy constructor throws) is not translated
/// `TESRegionDataImposter::Copy` (Xbox PDB): allocates 0x10 bytes and, when
/// that succeeds, copy-constructs a `TESRegionDataImposter` of `this` in
/// them (`004f1f70`). Returns the copy, or null.
pub fn tes_region_data_imposter_copy(e: &mut Engine, this: Ptr) -> Ptr {
    let block = e.call(ALLOCATE, &args![0x10u32]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    fn_004f1f70(e, Ptr::new(block), this)
}

// Translated from 004f2450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataImposter::BlendInto` (Xbox PDB; `thiscall`, `RET 8`): only
/// asks a non-null `source` for its `GetID` (slot +0x10) and ignores the
/// answer; `enabled` is not read.
pub fn tes_region_data_imposter_blend_into(
    e: &mut Engine,
    _unused_this: Ptr,
    source: Ptr,
    _unused_enabled: u32,
) {
    if !source.is_null() {
        e.vcall(source.addr(), SLOT_GET_ID, &args![]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004f_1f70, fn_004f1f70(Ptr, Ptr) -> Ptr),
        entry!(0x004f_20b0, tes_region_data_imposter_save(Ptr)),
        entry!(0x004f_23d0, tes_region_data_imposter_copy(Ptr) -> Ptr),
        entry!(
            0x004f_2450,
            tes_region_data_imposter_blend_into(Ptr, Ptr, u32)
        ),
        entry!(0x004f_1f40, tes_region_data_imposter_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x004f_2040, fn_004f2040(Ptr)),
        entry!(0x004f_2370, fn_004f2370(Ptr) -> bool),
        entry!(0x004f_2390, fn_004f2390(Ptr, u32) -> bool),
        entry!(0x004f_2190, tes_region_data_imposter_load_region_data_imposter(Ptr, Ptr) -> bool),
        entry!(0x004f_2290, tes_region_data_imposter_init_item(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_004f1f40_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x004f_2040, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x004f_1f40, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x004f_1f40)
                .collect();
            let mut expected = vec![(0x004f_2040, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod imposter_tests {
    use super::*;

    fn list_of(e: &mut Engine, a: &[u32]) -> Ret {
        let _ = e;
        (a[0] + 8).into_ret()
    }

    #[test]
    fn at_004f2040_clears_and_destroys_the_list_then_the_base() {
        let mut e = Engine::new();
        e.register(LIST_OF_00413F40, list_of);
        e.register(LIST_CLEAR_00470470, |_, _| Ret::default());
        e.register(LIST_DTOR_0046FFB0, |_, _| Ret::default());
        e.register(TES_REGION_DATA_DTOR, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x004f_2040, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), TES_REGION_DATA_IMPOSTER_VTABLE);
        let calls: Vec<_> = log.into_iter().filter(|c| c.0 != 0x004f_2040).collect();
        assert_eq!(
            calls,
            vec![
                (LIST_OF_00413F40, vec![this]),
                (LIST_CLEAR_00470470, vec![this + 8]),
                (LIST_DTOR_0046FFB0, vec![this + 8]),
                (TES_REGION_DATA_DTOR, vec![this]),
            ]
        );
    }

    #[test]
    fn at_004f2370_clears_the_list_and_returns_true() {
        let mut e = Engine::new();
        e.register(LIST_OF_00413F40, list_of);
        e.register(LIST_CLEAR_00470470, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert!(e.call(0x004f_2370, &args![this]).bool());
        let log = e.call_log.take().unwrap();
        let clears: Vec<_> = log.iter().filter(|c| c.0 == LIST_CLEAR_00470470).collect();
        assert_eq!(clears[0].1, vec![this + 8]);
    }

    #[test]
    fn at_004f2390_adds_only_missing_references() {
        let mut e = Engine::new();
        e.register(LIST_OF_00413F40, list_of);
        // Present when the reference is odd.
        e.register(LIST_CONTAINS_005F65D0, |e, a| {
            u32::from(e.mem.u32(a[1]) & 1 != 0).into_ret()
        });
        e.register_double(LIST_ADD_00905820, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0] + 4, value);
            Ret::default()
        });
        let this = e.mem.alloc(0x10);
        assert!(e.call(0x004f_2390, &args![this, 0x1234u32]).bool());
        assert_eq!(e.mem.u32(this + 12), 0x1234);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x004f_2390, &args![this, 0x1235u32]).bool());
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|c| c.0 != LIST_ADD_00905820));
        assert_eq!(e.mem.u32(this + 12), 0x1234);
    }

    #[test]
    fn at_004f2190_refuses_null_files_and_other_chunks() {
        let mut e = Engine::new();
        e.register(GET_TES_CHUNK, |_, _| 0x5052_4d44u32.into_ret());
        let this = e.mem.alloc(0x10);
        assert!(!e.call(0x004f_2190, &args![this, 0u32]).bool());
        let file = e.mem.alloc(0x300);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x004f_2190, &args![this, file]).bool());
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|c| c.0 != ALLOCATE));
    }

    #[test]
    fn at_004f2190_reads_each_id_and_adds_it() {
        let mut e = Engine::new();
        e.register(GET_TES_CHUNK, |_, _| CHUNK_RDID.into_ret());
        e.register(CHUNK_SIZE_00401660, |_, _| 9u32.into_ret());
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(DEALLOCATE, |_, _| Ret::default());
        e.register(GET_CHUNK_DATA, |e, a| {
            e.mem.set_u32(a[1], 0x0100_0001);
            e.mem.set_u32(a[1] + 4, 0x0100_0002);
            Ret::default()
        });
        e.register(FILE_FLAG_00401680, |_, _| 1u32.into_ret());
        e.register(FIX_ID_00401080, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0200_0000);
            Ret::default()
        });
        e.register(LIST_OF_00413F40, list_of);
        e.register(LIST_CONTAINS_005F65D0, |_, _| Ret::default());
        e.register(LIST_ADD_00905820, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        let file = e.mem.alloc(0x300);
        e.call_log = Some(vec![]);
        assert!(e.call(0x004f_2190, &args![this, file]).bool());
        let log = e.call_log.take().unwrap();
        let alloc: Vec<_> = log.iter().filter(|c| c.0 == ALLOCATE).collect();
        assert_eq!(alloc[0].1, vec![8]);
        let block = log
            .iter()
            .find(|c| c.0 == GET_CHUNK_DATA)
            .unwrap()
            .1
            .clone();
        assert_eq!(block[0], file);
        assert_eq!(block[2], 9);
        let fixes: Vec<_> = log.iter().filter(|c| c.0 == FIX_ID_00401080).collect();
        assert_eq!(fixes.len(), 2);
        assert_eq!(fixes[0].1[1], 0);
        let added: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == LIST_ADD_00905820)
            .map(|c| c.1[0])
            .collect();
        assert_eq!(added, vec![this + 8, this + 8]);
        let freed: Vec<_> = log.iter().filter(|c| c.0 == DEALLOCATE).collect();
        assert_eq!(freed[0].1, vec![block[1]]);
    }

    #[test]
    fn at_004f2290_resolves_items_and_removes_unresolved_ones() {
        let mut e = Engine::new();
        // Nodes: (item, next). First resolves, second does not and its
        // removal leaves a null item, which ends the walk.
        let second = e.mem.alloc(8);
        let first = e.mem.alloc(8);
        e.mem.set_u32(first, 0x10);
        e.mem.set_u32(first + 4, second);
        e.mem.set_u32(second, 0x20);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, first);
        e.register(LIST_OF_00413F40, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(NODE_ITEM_006815C0, |_, a| a[0].into_ret());
        e.register(NODE_FOLLOWING_00726070, |e, a| {
            e.mem.u32(a[0] + 4).into_ret()
        });
        e.register(TES_FORM_GET_FILE, |_, _| 0x77u32.into_ret());
        e.register(TES_FORM_ADD_COMPILE_INDEX, |_, _| Ret::default());
        e.register(LOOKUP_FORM_004839C0, |_, a| (a[0] + 0x1000).into_ret());
        e.register(RT_DYNAMIC_CAST, |_, a| {
            let cast = if a[0] == 0x1010 { a[0] } else { 0 };
            cast.into_ret()
        });
        e.register(NODE_SET_ITEM_00726C60, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            Ret::default()
        });
        e.register(NODE_REMOVE_0063F7B0, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        let file = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x004f_2290, &args![this, file]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(first), 0x1010);
        assert_eq!(e.mem.u32(second), 0);
        let files: Vec<_> = log.iter().filter(|c| c.0 == TES_FORM_GET_FILE).collect();
        assert_eq!(files[0].1, vec![file, 0xffff_ffff]);
        let compile: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == TES_FORM_ADD_COMPILE_INDEX)
            .map(|c| c.1[1])
            .collect();
        assert_eq!(compile, vec![0x77, 0x77]);
        let casts: Vec<_> = log.iter().filter(|c| c.0 == RT_DYNAMIC_CAST).collect();
        assert_eq!(
            casts[0].1,
            vec![0x1010, 0, CAST_SOURCE_TYPE, CAST_TARGET_TYPE, 0]
        );
        let removed: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == NODE_REMOVE_0063F7B0)
            .map(|c| c.1[0])
            .collect();
        assert_eq!(removed, vec![second]);
    }

    #[test]
    fn at_004f2290_without_a_file_passes_no_file_and_skips_empty_nodes() {
        let mut e = Engine::new();
        let node = e.mem.alloc(8);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, node);
        e.register(LIST_OF_00413F40, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(NODE_ITEM_006815C0, |_, a| a[0].into_ret());
        e.register(NODE_REMOVE_0063F7B0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        // An empty node with no cast result yet: removed, then its item is
        // still null, so the walk ends.
        e.call(0x004f_2290, &args![this, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|c| c.0 != TES_FORM_GET_FILE));
        assert_eq!(
            log.iter().filter(|c| c.0 == NODE_REMOVE_0063F7B0).count(),
            1
        );
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

    /// List doubles: a node is two words (item, next); `00413f40` is +8.
    fn list_doubles(e: &mut Engine) {
        e.register(LIST_OF_00413F40, |_, a| ret(a[0] + 8));
        e.register(NODE_ITEM_006815C0, |_, a| ret(a[0]));
        e.register(NODE_FOLLOWING_00726070, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(FORM_ID_0084E3A0, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(DEALLOCATE, |_, _| Ret::default());
    }

    #[test]
    fn fn_004f1f70_copies_the_source_list_of_imposter_data_only() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        e.register(REGION_DATA_COPY_CONSTRUCTOR, |_, _| Ret::default());
        e.register(LIST_CONSTRUCTOR_0096A2D0, |_, _| Ret::default());
        e.register(LIST_CLEAR_00470470, |_, _| Ret::default());
        e.register(LIST_ADD_00905820, |_, _| Ret::default());
        let slots: Vec<u32> = (0..5).map(|i| 0x0150_1000 + 4 * i).collect();
        e.put_vtable(0x0150_0000, &slots);
        e.register(0x0150_1010, |_, _| ret(IMPOSTER_ID));
        let source = e.mem.alloc(0x10);
        e.mem.set_u32(source, 0x0150_0000);
        let node2 = e.mem.alloc(8);
        e.mem.set_u32(source + 8, 0x11);
        e.mem.set_u32(source + 12, node2);
        e.mem.set_u32(node2, 0x22);
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004f_1f70, &args![this, source]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), TES_REGION_DATA_IMPOSTER_VTABLE);
        assert_eq!(
            calls_to(&log, LIST_CONSTRUCTOR_0096A2D0),
            vec![vec![this + 8]]
        );
        assert_eq!(calls_to(&log, LIST_CLEAR_00470470), vec![vec![this + 8]]);
        assert_eq!(
            calls_to(&log, LIST_ADD_00905820),
            vec![vec![this + 8, source + 8], vec![this + 8, node2]]
        );
        // Another kind of data: nothing is added.
        e.register(0x0150_1010, |_, _| ret(7));
        e.call_log = Some(vec![]);
        e.call(0x004f_1f70, &args![this, source]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, LIST_ADD_00905820).is_empty());
    }

    #[test]
    fn save_writes_the_ids_of_the_first_count_items() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        e.register(REGION_DATA_SAVE, |_, _| Ret::default());
        e.register(LIST_COUNT_005AE380, |_, _| ret(2));
        let written = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let w = written.clone();
        e.register_double(ADD_CHUNK_ARRAY, move |e, a| {
            w.borrow_mut()
                .push((a[0], a[2], e.mem.u32(a[1]), e.mem.u32(a[1] + 4)));
            Ret::default()
        });
        let form_a = e.mem.alloc(0x10);
        e.mem.set_u32(form_a + 0xc, 0xa1);
        let form_b = e.mem.alloc(0x10);
        e.mem.set_u32(form_b + 0xc, 0xb2);
        let this = e.mem.alloc(0x10);
        let node2 = e.mem.alloc(8);
        let node3 = e.mem.alloc(8);
        e.mem.set_u32(this + 8, form_a);
        e.mem.set_u32(this + 12, node2);
        e.mem.set_u32(node2, form_b);
        e.mem.set_u32(node2 + 4, node3);
        e.call_log = Some(vec![]);
        e.call(0x004f_20b0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(*written.borrow(), vec![(CHUNK_RDID, 2, 0xa1, 0xb2)]);
        assert_eq!(calls_to(&log, FORM_ID_0084E3A0).len(), 2);
        assert_eq!(calls_to(&log, ALLOCATE), vec![vec![8]]);
        assert_eq!(calls_to(&log, DEALLOCATE).len(), 1);
        // An empty list only saves the base.
        e.register(LIST_COUNT_005AE380, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(0x004f_20b0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, REGION_DATA_SAVE), vec![vec![this]]);
        assert!(calls_to(&log, ALLOCATE).is_empty());
    }

    #[test]
    fn copy_allocates_and_copy_constructs() {
        let mut e = Engine::new();
        e.register(ALLOCATE, |_, _| ret(0));
        let this = e.mem.alloc(0x10);
        assert_eq!(e.call(0x004f_23d0, &args![this]).u32(), 0);
        let block = e.mem.alloc(0x10);
        e.register_double(ALLOCATE, move |_, _| ret(block));
        e.register(REGION_DATA_COPY_CONSTRUCTOR, |_, _| Ret::default());
        e.register(LIST_CONSTRUCTOR_0096A2D0, |_, _| Ret::default());
        e.register(LIST_OF_00413F40, |_, a| ret(a[0] + 8));
        e.register(LIST_CLEAR_00470470, |_, _| Ret::default());
        e.put_vtable(0x0150_0000, &[0, 0, 0, 0, 0x0150_1010]);
        e.register(0x0150_1010, |_, _| ret(0));
        e.mem.set_u32(this, 0x0150_0000);
        assert_eq!(e.call(0x004f_23d0, &args![this]).u32(), block);
        assert_eq!(e.mem.u32(block), TES_REGION_DATA_IMPOSTER_VTABLE);
    }

    #[test]
    fn blend_into_only_asks_a_source_for_its_id() {
        let mut e = Engine::new();
        e.put_vtable(0x0150_0000, &[0, 0, 0, 0, 0x0150_1010]);
        e.register(0x0150_1010, |_, _| ret(8));
        let this = e.mem.alloc(0x10);
        let source = e.mem.alloc(0x10);
        e.mem.set_u32(source, 0x0150_0000);
        e.call_log = Some(vec![]);
        e.call(0x004f_2450, &args![this, source, 0u32]);
        e.call(0x004f_2450, &args![this, 0u32, 1u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0150_1010), vec![vec![source]]);
    }
}
