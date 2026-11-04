//! `fallout shared/bgsimpactdataset.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0058e810 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0058e810 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x5f (0x4c bytes); `ECX` is the object.
/// The engine map names it `BGSImpactDataSet::BGSImpactDataSet` (Xbox PDB); the RTTI name of the vtable it stores at `+0` (`0x0103_323c`) is `BGSImpactDataSet`, which agrees.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_0058e810(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x004f_9d90, &args![base + 0x18]);
    e.mem.set_u32(base, 0x0103_323c);
    e.mem.set_u32(base + 0x18, 0x0103_3224);
    e.call(0x004f_15a0, &args![this, 0x5fu32]);
    e.call(0x0058_eb10, &args![this]);
    this
}

// Translated from 0058f850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMap<BGSImpactData_P_BGSImpactData_P>` scalar deleting destructor (engine map name): runs the destructor body `0058f900` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn bs_map_bgs_impact_data_p_bgs_impact_data_p_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x0058_f900, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0058f9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMapBase<BGSImpactData_P_BGSImpactData_P>` scalar deleting destructor (engine map name): runs the destructor body `0058f960` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn bs_map_base_bgs_impact_data_p_bgs_impact_data_p_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x0058_f960, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// The impact data set override (`011ca834`), or null: `0058eeb0` sets it.
const OVERRIDE_SET: u32 = 0x011c_a834;
/// The default impact data set (`011ca830`).
const DEFAULT_SET: u32 = 0x011c_a830;
/// `0058f6f0` (`ECX` = the override set, two stack words: material, impact data).
const OVERRIDE_LOOKUP_0058F6F0: u32 = 0x0058_f6f0;
/// `0058f7f0` (`ECX` = a set, one stack word: a material): the set's impact data for it.
const SET_IMPACT_0058F7F0: u32 = 0x0058_f7f0;
/// `008374e0` (`cdecl`: material, impact data) and `00837500` (no words).
const SET_MATERIAL_IMPACT_008374E0: u32 = 0x0083_74e0;
const MATERIALS_SET_00837500: u32 = 0x0083_7500;
/// The game's `memset` wrapper (`00403d30`, `cdecl`: destination, byte, size).
const MEMSET: u32 = 0x0040_3d30;
/// `004013e0` (`ECX`): whether form flag 8 is set (`AL`).
const FORM_FLAG_8_SET: u32 = 0x0040_13e0;
/// `00484ab0` (`ECX`, one stack word): sets or clears form flag 8.
const SET_FORM_FLAG_8: u32 = 0x0048_4ab0;
/// `TESForm::GetFile` (Xbox PDB, `00484e60`, `ECX`, one stack word).
const GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex` (Xbox PDB, `00485d50`, `cdecl`: address of a form id, file).
const ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// `004839c0` (`cdecl`, one word): the form of a form id, or null.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (`00ec43fb`, `cdecl`: object, vftable delta, source type, target type,
/// is-reference).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptors `.?AVTESForm@@`, `.?AVBGSImpactDataSet@@`, `.?AVBGSImpactData@@`.
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_IMPACT_DATA_SET: u32 = 0x0118_62dc;
const TYPE_IMPACT_DATA: u32 = 0x0118_62fc;
/// `0084e3a0` (`ECX`): the word at +0xc (the form id).
const FORM_ID_0084E3A0: u32 = 0x0084_e3a0;
/// The release build's logging stub (`005b5e40`, `cdecl`, variable words).
const LOG: u32 = 0x005b_5e40;
/// "MASTERFILE: Unable to find impact data (%08X) on impact data set %s (%08X)."
const MESSAGE_NO_IMPACT_DATA: u32 = 0x0103_3390;
/// The allocator (`00401000`, `cdecl`, one word: the size).
const ALLOCATE: u32 = 0x0040_1000;
/// `"DefaultImpactDataSet"`.
const DEFAULT_SET_NAME: u32 = 0x0103_3374;
/// The data handler (`011c3f2c`) and `TESDataHandler::AddFormToDataHandler` (Xbox PDB,
/// `004603b0`, `ECX` = data handler, one stack word).
const DATA_HANDLER: u32 = 0x011c_3f2c;
const ADD_FORM_TO_DATA_HANDLER: u32 = 0x0046_03b0;
/// The default set's form id.
const DEFAULT_SET_ID: u32 = 0x276;
/// The model loader (`011c3b3c`) and its `00443d30` (`ECX`, seven stack words).
const MODEL_LOADER: u32 = 0x011c_3b3c;
const MODEL_LOADER_QUEUE_00443D30: u32 = 0x0044_3d30;
/// `0048cee0` (`ECX` = a model): the length of its name.
const MODEL_NAME_LENGTH_0048CEE0: u32 = 0x0048_cee0;
/// `006733e0` (`ECX`): the word at +0x48 (here the impact data's texture set).
const WORD_48_006733E0: u32 = 0x0067_33e0;
/// `BGSTextureSet::QueueTextureSet` (Xbox PDB, `005930e0`, `ECX`, three stack words).
const QUEUE_TEXTURE_SET: u32 = 0x0059_30e0;
/// `TESFile::GetTESForm` (Xbox PDB, `00472660`, `ECX` = file): the record's form type.
const GET_TES_FORM: u32 = 0x0047_2660;
/// `TESForm::LoadForm` (Xbox PDB, `00485110`, `ECX` = form, one stack word: the file).
const LOAD_FORM: u32 = 0x0048_5110;
/// `TESFile::GetTESChunk` (Xbox PDB, `004726b0`, `ECX` = file): the chunk's code, 0 at the end.
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `00401660` (`ECX` = file): the current chunk's size.
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `TESFile::GetChunkData_ov3` (Xbox PDB, `00472890`, `ECX` = file, two stack words:
/// destination, size).
const GET_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
/// `00401680` (`ECX` = file): whether the file's data needs its byte order swapped (`AL`).
const NEEDS_SWAP: u32 = 0x0040_1680;
/// `00401080` (`cdecl`: address of a word, a byte flag): swaps the word's byte order.
const SWAP_WORD: u32 = 0x0040_1080;
/// `004726f0` (`ECX` = file): steps to the next chunk; false at the record's end (`AL`).
const NEXT_CHUNK: u32 = 0x0047_26f0;
/// Chunk codes the impact data set loader reads (the four characters, first byte lowest).
const CHUNK_DATA: u32 = 0x4154_4144;
const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_OBND: u32 = 0x444e_424f;
/// The impact data set's twelve material slots, from +0x1c.
const SLOTS: u32 = 0x1c;

// Translated from 0058e9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (in this unit's range; `ECX` and one stack word, `RET 4`): the impact
/// data of material slot `material` (`+0x1c + material * 4`); with an override set
/// (`011ca834`) its `0058f6f0` answer for the material and that data instead.
pub fn fn_0058e9d0(e: &mut Engine, this: Ptr, material: u32) -> u32 {
    let slot = this
        .addr()
        .wrapping_add(material.wrapping_mul(4))
        .wrapping_add(SLOTS);
    let set = e.global::<u32>(OVERRIDE_SET);
    if set != 0 {
        let data = e.mem.u32(slot);
        e.call(OVERRIDE_LOOKUP_0058F6F0, &args![set, material, data])
            .u32()
    } else {
        e.mem.u32(slot)
    }
}

// Translated from 0058eb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (the `BGSImpactDataSet` constructor `0058e810` runs it): clears the
/// twelve material slots (`memset` of 0x30 bytes at +0x1c).
pub fn fn_0058eb10(e: &mut Engine, this: Ptr) {
    e.call(MEMSET, &args![this.addr() + SLOTS, 0u32, 0x30u32]);
}

// Translated from 0058e8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`cdecl`, one word): the impact data slot of a material code. The
/// exe's byte map and jump table in `.text` (`0058e9a4`, `0058e97c`) are these arms; codes
/// above 0x1f answer 4.
pub fn fn_0058e8f0(_e: &mut Engine, material: u32) -> u32 {
    match material {
        0 | 10 | 19 => 0,
        1 => 7,
        2 | 18 => 1,
        3 | 24 => 3,
        4 => 2,
        6 | 7 => 6,
        8 => 8,
        9 | 12 | 30 | 31 => 5,
        16 | 17 | 22 | 23 | 25 | 28 | 29 => 9,
        _ => 4,
    }
}

// Translated from 0058e8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BGSImpactDataSet` destructor body (unnamed in the map; `ECX`): stores the two vtables
/// (`0103323c` at +0, `01033224` at +0x18), then `TESForm::~TESForm` (`00483630`).
pub fn fn_0058e8c0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0103_323c);
    e.mem.set_u32(this.addr() + 0x18, 0x0103_3224);
    e.call(0x0048_3630, &args![this]);
}

// Translated from 0058eeb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`cdecl`, one word): makes `set` the override set (`011ca834`), then
/// for each material 0 to 9 hands `008374e0` the material and the set's impact data for it
/// (`0058f7f0`; 0 without a set), then `00837500`.
pub fn fn_0058eeb0(e: &mut Engine, set: u32) {
    e.set_global(OVERRIDE_SET, set);
    for material in 0..10u32 {
        let data = if set != 0 {
            e.call(SET_IMPACT_0058F7F0, &args![set, material]).u32()
        } else {
            0
        };
        e.call(SET_MATERIAL_IMPACT_008374E0, &args![material, data]);
    }
    e.call(MATERIALS_SET_00837500, &args![]);
}

// Translated from 0058edd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BGSImpactDataSet` `InitItem` (unnamed in the map; `ECX`): unless form flag 8 is set,
/// resolves each nonzero material slot (a form id until now) to its form cast to
/// `BGSImpactData`, after giving it the compile index of the set's file; a slot left null is
/// reported with the id, the set's name (vtable slot +0x130) and form id. Then form flag 8 is
/// set.
pub fn fn_0058edd0(e: &mut Engine, this: Ptr) {
    if e.call(FORM_FLAG_8_SET, &args![this]).bool() {
        return;
    }
    e.with_stack(4, |e, id_at| {
        for material in 0..12u32 {
            let slot = this.addr() + SLOTS + material * 4;
            let id = e.mem.u32(slot);
            e.mem.set_u32(id_at.addr(), id);
            if id == 0 {
                continue;
            }
            let file = e.call(GET_FILE, &args![this, u32::MAX]).u32();
            e.call(ADD_COMPILE_INDEX, &args![id_at, file]);
            let id = e.mem.u32(id_at.addr());
            let form = e.call(LOOKUP_FORM, &args![id]).u32();
            let data = e
                .call(
                    DYNAMIC_CAST,
                    &args![form, 0u32, TYPE_TES_FORM, TYPE_IMPACT_DATA, 0u32],
                )
                .u32();
            e.mem.set_u32(slot, data);
            if e.mem.u32(slot) != 0 {
                continue;
            }
            let form_id = e.call(FORM_ID_0084E3A0, &args![this]).u32();
            let name = e.vcall(this.addr(), 0x130, &args![]).u32();
            let id = e.mem.u32(id_at.addr());
            e.call(LOG, &args![MESSAGE_NO_IMPACT_DATA, id, name, form_id]);
        }
    });
    e.call(SET_FORM_FLAG_8, &args![this, 1u32]);
}

// Translated from 0058ea10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0058ea10 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f109cb, state variable) is not translated
/// Unnamed in the map (no words): the default impact data set (`011ca830`) is form 0x276 cast
/// to `BGSImpactDataSet`; when there is none, a new one (`00401000` of 0x4c bytes, made by
/// `0058e810`, null when the allocation fails) takes the form id 0x276 (vtable slot +0x128
/// with 1), the editor id `"DefaultImpactDataSet"` (slot +0x134) and is filed in the data
/// handler (`TESDataHandler::AddFormToDataHandler`).
pub fn fn_0058ea10(e: &mut Engine) {
    let form = e.call(LOOKUP_FORM, &args![DEFAULT_SET_ID]).u32();
    let set = e
        .call(
            DYNAMIC_CAST,
            &args![form, 0u32, TYPE_TES_FORM, TYPE_IMPACT_DATA_SET, 0u32],
        )
        .u32();
    e.set_global(DEFAULT_SET, set);
    if e.global::<u32>(DEFAULT_SET) != 0 {
        return;
    }
    let block = e.call(ALLOCATE, &args![0x4cu32]).u32();
    let set = if block != 0 {
        fn_0058e810(e, Ptr::new(block)).addr()
    } else {
        0
    };
    e.set_global(DEFAULT_SET, set);
    let set = e.global::<u32>(DEFAULT_SET);
    e.vcall(set, 0x128, &args![DEFAULT_SET_ID, 1u32]);
    let set = e.global::<u32>(DEFAULT_SET);
    e.vcall(set, 0x134, &args![DEFAULT_SET_NAME]);
    let set = e.global::<u32>(DEFAULT_SET);
    let data_handler = e.global::<u32>(DATA_HANDLER);
    e.call(ADD_FORM_TO_DATA_HANDLER, &args![data_handler, set]);
}

// Translated from 0058ef10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map: slot of the `BGSImpactDataSet`'s vtable at +0x18 (`ECX` is that +0x18,
/// two stack words, `RET 8`): for each of the twelve materials, the set's impact data
/// ([`fn_0058e9d0`] on `this - 0x18`); when there is one, its model (+0x18) is queued with
/// the model loader (`00443d30` with the two words, 1, 1, 0, 0) when it has a name
/// (`0048cee0`), and its texture set (`006733e0`), when there is one, queued too
/// (`BGSTextureSet::QueueTextureSet` with the two words and 0).
pub fn fn_0058ef10(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    let set = Ptr::new(this.addr().wrapping_sub(0x18));
    for material in 0..12u32 {
        let data = fn_0058e9d0(e, set, material);
        if data == 0 {
            continue;
        }
        if e.call(MODEL_NAME_LENGTH_0048CEE0, &args![data + 0x18])
            .u32()
            != 0
        {
            let model = if data != 0 { data + 0x18 } else { 0 };
            let loader = e.global::<u32>(MODEL_LOADER);
            e.call(
                MODEL_LOADER_QUEUE_00443D30,
                &args![loader, model, first, second, 1u32, 1u32, 0u32, 0u32],
            );
        }
        let texture_set = e.call(WORD_48_006733E0, &args![data]).u32();
        if texture_set != 0 {
            e.call(QUEUE_TEXTURE_SET, &args![texture_set, first, second, 0u32]);
        }
    }
}

// Translated from 0058ec00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0058ec00 - the /GS cookie check (00ec408c) is not translated; the EDID buffer from __alloca_probe_16 (00ec5ec0) is a with_stack block of the chunk size (its 16-byte alignment padding, and the stack growing with each EDID chunk, are not modelled)
/// The `BGSImpactDataSet` `Load` (unnamed in the map; `ECX` and one stack word, the file,
/// `RET 4`): false unless `TESFile::GetTESForm`'s whole `EAX` is 0x5f. Otherwise
/// `TESForm::LoadForm`, then each chunk: `EDID` (a stack buffer of the chunk's size, read
/// with at most 0x200 bytes, given to vtable slot +0x134), `OBND` (vtable slot +0xe0 with the
/// file), `DATA` (the twelve material slots, 0x30 bytes at +0x1c, each word byte-swapped when
/// the file needs it). Then true.
pub fn fn_0058ec00(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    if e.call(GET_TES_FORM, &args![file]).u32() != 0x5f {
        return false;
    }
    let base = this.addr();
    e.call(LOAD_FORM, &args![this, file]);
    loop {
        let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            CHUNK_DATA => {
                e.call(GET_CHUNK_DATA_SIZED, &args![file, base + SLOTS, 0x30u32]);
                if e.call(NEEDS_SWAP, &args![file]).bool() {
                    for material in 0..12u32 {
                        e.call(SWAP_WORD, &args![base + SLOTS + material * 4, 0u32]);
                    }
                }
            }
            CHUNK_EDID => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                e.with_stack(size, |e, name| {
                    e.call(GET_CHUNK_DATA_SIZED, &args![file, name, 0x200u32]);
                    e.vcall(base, 0x134, &args![name]);
                });
            }
            CHUNK_OBND => {
                e.vcall(base, 0xe0, &args![file]);
            }
            _ => {}
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

// Translated from 0058f7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, one word: an index; `RET 4`): the address of the
/// 0x1E-byte entry `index` from `this + 0x30` when its first byte is not 0 (tested
/// sign-extended, so any non-zero byte), else null.
pub fn fn_0058f7f0(e: &mut Engine, this: Ptr, index: i32) -> u32 {
    let entry = this
        .addr()
        .wrapping_add((index as u32).wrapping_mul(0x1e))
        .wrapping_add(0x30);
    if e.mem.u8(entry) as i8 != 0 {
        entry
    } else {
        0
    }
}

/// `ImpactSwap` (Xbox PDB, 0x15C bytes): `pRemappedData`, twelve
/// `BSMap<BGSImpactData *,BGSImpactData *>` pointers from +0x00, and
/// `pAltFootstepMaterialNames`, 300 bytes of names at +0x30.
const SWAP_MAPS: u32 = 12;
const SWAP_NAMES: u32 = 0x30;
const SWAP_NAMES_SIZE: u32 = 300;
/// The map helpers: `004b9ba0` (the first position, 0 when empty),
/// `006b7f20` (`RET 0xC`: the position, key and value out; steps the
/// position), `00438af0` (removes every entry), `BSMapBase::SetAt` (engine
/// map, `006f9a80`, `RET 8`: key, value), `00853130` (`RET 8`: key, value
/// out; the lookup).
const MAP_FIRST: u32 = 0x004b_9ba0;
const MAP_NEXT: u32 = 0x006b_7f20;
const MAP_REMOVE_ALL: u32 = 0x0043_8af0;
const MAP_SET_AT: u32 = 0x006f_9a80;
const MAP_LOOKUP: u32 = 0x0085_3130;
/// The map's entry count (`0084e3a0` reads the word at +0xC).
const MAP_COUNT: u32 = FORM_ID_0084E3A0;
/// The free (`00401030`, cdecl, one word).
const FREE: u32 = 0x0040_1030;
/// `memcpy` (`00401460`), `_memcmp` (`00ec4835`) and `_memset`
/// (`00ec61c0`), cdecl, three words each.
const MEMCPY: u32 = 0x0040_1460;
const MEMCMP: u32 = 0x00ec_4835;
const CRT_MEMSET: u32 = 0x00ec_61c0;
/// `00401500`: whether saved data is byte-swapped (`AL`).
const SAVE_SWAPS_ENDIAN: u32 = 0x0040_1500;
/// `005a6b10` (`ECX`): swaps the three words of a 12-byte record.
const SWAP_RECORD: u32 = 0x005a_6b10;
/// `TESForm::StartForm`, `TESForm::CloseForm` (Xbox PDB) and the chunk
/// writers `TESForm::AddChunkArray` (`004856f0`), `AddChunkArray_ov2`
/// (`00485710`) and `__AddChunkData` (`00485990`), cdecl: code, data, size.
const START_FORM: u32 = 0x0048_55a0;
const CLOSE_FORM: u32 = 0x0048_5680;
const ADD_CHUNK_ARRAY: u32 = 0x0048_56f0;
const ADD_CHUNK_ARRAY_WORDS: u32 = 0x0048_5710;
const ADD_CHUNK_DATA: u32 = 0x0048_5990;
/// Chunk codes `IMPF` (the footstep names) and `IMPS` (one swap).
const CHUNK_IMPF: u32 = 0x4650_4d49;
const CHUNK_IMPS: u32 = 0x5350_4d49;
/// "Could not find old ImpactData (%08X) for ImpactSwap on '%s' (%08X)."
/// and its "new" twin.
const MESSAGE_NO_OLD_DATA: u32 = 0x0103_3428;
const MESSAGE_NO_NEW_DATA: u32 = 0x0103_33e0;
/// The vtables of `BSMap<BGSImpactData *,BGSImpactData *>` (`01033470`)
/// and its base `BSMapBase` (`01033490`), from the engine map's names of
/// their scalar deleting destructors.
const MAP_VTABLE: u32 = 0x0103_3470;
const MAP_BASE_VTABLE: u32 = 0x0103_3490;

/// The map at `swap + material * 4` deleted (its vtable slot 0, the scalar
/// deleting destructor, with 1) when there is one, and the slot cleared.
fn delete_swap_map(e: &mut Engine, swap: u32, material: u32) {
    let map = e.mem.u32(swap + material * 4);
    if map != 0 {
        e.vcall(map, 0, &args![1u32]);
    }
    e.mem.set_u32(swap + material * 4, 0);
}

/// Each (key, value) of `map` in the map's own order (`004b9ba0`, then
/// `006b7f20` with zeroed key and value words), handed to `each`.
fn for_each_pair(e: &mut Engine, map: u32, mut each: impl FnMut(&mut Engine, u32, u32)) {
    e.with_stack(0xc, |e, words| {
        let position = words.addr();
        let key = position + 4;
        let value = position + 8;
        let first = e.call(MAP_FIRST, &args![map]).u32();
        e.mem.set_u32(position, first);
        while e.mem.u32(position) != 0 {
            e.mem.set_u32(key, 0);
            e.mem.set_u32(value, 0);
            e.call(MAP_NEXT, &args![map, position, key, value]);
            let (k, v) = (e.mem.u32(key), e.mem.u32(value));
            each(e, k, v);
        }
    });
}

// Translated from 0058eb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The impact data set's copy (`RET 4`: the source form; the engine map has
/// no name): when the source casts to a `BGSImpactDataSet`,
/// `TESForm::CopyAllComponents` and its twelve material slots (0x30 bytes
/// at +0x1C).
pub fn fn_0058eb30(e: &mut Engine, this: Ptr, source: u32) {
    let set = e
        .call(
            DYNAMIC_CAST,
            &args![source, 0u32, TYPE_TES_FORM, TYPE_IMPACT_DATA_SET, 0u32],
        )
        .u32();
    if set == 0 {
        return;
    }
    e.call(0x0048_51b0, &args![this, set]);
    e.call(MEMCPY, &args![this.addr() + SLOTS, set + SLOTS, 0x30u32]);
}

// Translated from 0058eb90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The impact data set's compare (`RET 4`: the other form; the engine map
/// has no name): true (different) unless the other form casts to a
/// `BGSImpactDataSet`, `TESForm::CompareAllComponents` finds no difference
/// and the twelve slots match.
pub fn fn_0058eb90(e: &mut Engine, this: Ptr, other: u32) -> bool {
    let set = e
        .call(
            DYNAMIC_CAST,
            &args![other, 0u32, TYPE_TES_FORM, TYPE_IMPACT_DATA_SET, 0u32],
        )
        .u32();
    if set == 0 {
        return true;
    }
    if e.call(0x0048_5270, &args![this, set]).bool() {
        return true;
    }
    e.call(MEMCMP, &args![this.addr() + SLOTS, set + SLOTS, 0x30u32])
        .u32()
        != 0
}

// Translated from 0058ed50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The impact data set's save (the engine map has no name):
/// `TESForm::StartForm`, a `DATA` chunk of the twelve slots' form ids (0
/// for an empty slot, `TESForm::AddChunkArray_ov2` of 12 words) and
/// `TESForm::CloseForm`.
pub fn fn_0058ed50(e: &mut Engine, this: Ptr) {
    e.call(START_FORM, &args![this]);
    e.with_stack(0x30, |e, ids| {
        for material in 0..12u32 {
            let data = e.mem.u32(this.addr() + SLOTS + material * 4);
            let id = if data == 0 {
                0
            } else {
                e.call(FORM_ID_0084E3A0, &args![data]).u32()
            };
            e.mem.set_u32(ids.addr() + material * 4, id);
        }
        e.call(ADD_CHUNK_ARRAY_WORDS, &args![CHUNK_DATA, ids, 12u32]);
    });
    e.call(CLOSE_FORM, &args![this]);
}

// Translated from 0058efd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `ImpactSwap` constructor (the engine map has no name; `ECX`):
/// zeroes the twelve map pointers and the 300 bytes of footstep names
/// (`memset`) and returns the swap.
pub fn fn_0058efd0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(MEMSET, &args![this, 0u32, 0x30u32]);
    e.call(
        MEMSET,
        &args![this.addr() + SWAP_NAMES, 0u32, SWAP_NAMES_SIZE],
    );
    this
}

// Translated from 0058f010 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `ImpactSwap` destructor body (the engine map has no name; `ECX`):
/// deletes each of the twelve maps and clears its pointer.
pub fn fn_0058f010(e: &mut Engine, this: Ptr) {
    for material in 0..SWAP_MAPS {
        delete_swap_map(e, this.addr(), material);
    }
}

// Translated from 0058f080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImpactSwap::Save` (engine map): for each map, each pair as an `IMPS`
/// chunk of 12 bytes (the material, the old and the new impact data's form
/// ids, 0 for none), swapped before and after the write when saving swaps
/// bytes; then the footstep names as one `IMPF` chunk of 300 bytes.
pub fn impact_swap_save(e: &mut Engine, this: Ptr) {
    for material in 0..SWAP_MAPS {
        let map = e.mem.u32(this.addr() + material * 4);
        if map == 0 {
            continue;
        }
        // The locals: the position at -0x8, the record (material, old,
        // new) at -0x14.
        e.with_stack(0x10, |e, words| {
            let record = words.addr();
            let position = record + 0xc;
            let first = e.call(MAP_FIRST, &args![map]).u32();
            e.mem.set_u32(position, first);
            while e.mem.u32(position) != 0 {
                e.mem.set_u32(record, material);
                e.call(MAP_NEXT, &args![map, position, record + 4, record + 8]);
                for at in [record + 4, record + 8] {
                    let data = e.mem.u32(at);
                    if data != 0 {
                        let id = e.call(FORM_ID_0084E3A0, &args![data]).u32();
                        e.mem.set_u32(at, id);
                    }
                }
                if e.call(SAVE_SWAPS_ENDIAN, &args![]).bool() {
                    e.call(SWAP_RECORD, &args![record]);
                }
                e.call(ADD_CHUNK_DATA, &args![CHUNK_IMPS, record, 0xcu32]);
                if e.call(SAVE_SWAPS_ENDIAN, &args![]).bool() {
                    e.call(SWAP_RECORD, &args![record]);
                }
            }
        });
    }
    e.call(
        ADD_CHUNK_ARRAY,
        &args![CHUNK_IMPF, this.addr() + SWAP_NAMES, SWAP_NAMES_SIZE],
    );
}

// Translated from 0058f180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImpactSwap`'s chunk reader (`RET 4`: the file; the engine map has no
/// name): an `IMPF` chunk fills the 300 bytes of footstep names; an `IMPS`
/// chunk is read into a zeroed 12-byte record (swapped when the file needs
/// it) whose material, old and new impact data go to [`fn_0058f740`];
/// other chunks are left.
pub fn fn_0058f180(e: &mut Engine, this: Ptr, file: Ptr) {
    let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
    if chunk == CHUNK_IMPF {
        e.call(
            GET_CHUNK_DATA_SIZED,
            &args![file, this.addr() + SWAP_NAMES, SWAP_NAMES_SIZE],
        );
    } else if chunk == CHUNK_IMPS {
        let record = e.with_stack(0xc, |e, record| {
            e.call(MEMSET, &args![record, 0u32, 0xcu32]);
            e.call(GET_CHUNK_DATA_SIZED, &args![file, record, 0xcu32]);
            if e.call(NEEDS_SWAP, &args![file]).bool() {
                e.call(SWAP_RECORD, &args![record]);
            }
            [0, 4, 8].map(|o| e.mem.u32(record.addr() + o))
        });
        fn_0058f740(e, this, record[0], record[1], record[2]);
    }
}

// Translated from 0058f210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImpactSwap::InitItem` (engine map; `RET 4`: the form that holds the
/// swap). Each map's pairs (form ids until now) are copied into a block of
/// 12 bytes per entry (`00401000`; the size saturates at 0xFFFFFFFF when
/// the product overflows) and the map emptied; each id is given the
/// form's file's compile index and resolved to a `BGSImpactData` (a missing
/// one reported with the id, the form's name and form id), and pairs with
/// either side resolved go back in. The block is freed, and a map left
/// empty is deleted.
pub fn impact_swap_init_item(e: &mut Engine, this: Ptr, form: u32) {
    for material in 0..SWAP_MAPS {
        let map = e.mem.u32(this.addr() + material * 4);
        if map == 0 {
            continue;
        }
        let count = e.call(MAP_COUNT, &args![map]).u32();
        let (size, overflow) = count.overflowing_mul(12);
        let block = e
            .call(ALLOCATE, &args![if overflow { u32::MAX } else { size }])
            .u32();
        e.with_stack(4, |e, position| {
            let first = e.call(MAP_FIRST, &args![map]).u32();
            e.mem.set_u32(position.addr(), first);
            let mut index = 0u32;
            while e.mem.u32(position.addr()) != 0 {
                let entry = block.wrapping_add(index.wrapping_mul(12));
                e.call(MAP_NEXT, &args![map, position, entry + 4, entry + 8]);
                index = index.wrapping_add(1);
            }
        });
        e.call(MAP_REMOVE_ALL, &args![map]);
        for index in 0..count {
            let entry = block.wrapping_add(index.wrapping_mul(12));
            let old = resolve_impact_data(e, form, entry + 4, MESSAGE_NO_OLD_DATA);
            let new = resolve_impact_data(e, form, entry + 8, MESSAGE_NO_NEW_DATA);
            if old != 0 || new != 0 {
                let map = e.mem.u32(this.addr() + material * 4);
                e.call(MAP_SET_AT, &args![map, old, new]);
            }
        }
        e.call(FREE, &args![block]);
        let map = e.mem.u32(this.addr() + material * 4);
        if e.call(MAP_COUNT, &args![map]).u32() == 0 {
            delete_swap_map(e, this.addr(), material);
        }
    }
}

/// `ImpactSwap::InitItem`'s resolution of the form id at `id_at` (0 stays
/// 0): the compile index of `form`'s file, the form cast to
/// `BGSImpactData`, and `message` when that fails.
fn resolve_impact_data(e: &mut Engine, form: u32, id_at: u32, message: u32) -> u32 {
    let id = e.mem.u32(id_at);
    if id == 0 {
        return 0;
    }
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), id);
        let file = e.call(GET_FILE, &args![form, u32::MAX]).u32();
        e.call(ADD_COMPILE_INDEX, &args![local, file]);
        let id = e.mem.u32(local.addr());
        let found = e.call(LOOKUP_FORM, &args![id]).u32();
        let data = e
            .call(
                DYNAMIC_CAST,
                &args![found, 0u32, TYPE_TES_FORM, TYPE_IMPACT_DATA, 0u32],
            )
            .u32();
        if data == 0 {
            let form_id = e.call(FORM_ID_0084E3A0, &args![form]).u32();
            let name = e.vcall(form, 0x130, &args![]).u32();
            let id = e.mem.u32(local.addr());
            e.call(LOG, &args![message, id, name, form_id]);
        }
        data
    })
}

// Translated from 0058f4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImpactSwap`'s copy (`RET 4`: the source swap; the engine map has no
/// name): deletes its own maps, adds each pair of the source's maps
/// ([`fn_0058f740`]) and copies the 300 bytes of footstep names.
pub fn fn_0058f4c0(e: &mut Engine, this: Ptr, source: u32) {
    for material in 0..SWAP_MAPS {
        delete_swap_map(e, this.addr(), material);
    }
    for material in 0..SWAP_MAPS {
        let map = e.mem.u32(source + material * 4);
        if map == 0 {
            continue;
        }
        for_each_pair(e, map, |e, old, new| {
            fn_0058f740(e, this, material, old, new);
        });
    }
    e.call(
        MEMCPY,
        &args![
            this.addr() + SWAP_NAMES,
            source + SWAP_NAMES,
            SWAP_NAMES_SIZE
        ],
    );
}

// Translated from 0058f5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImpactSwap::Compare` (engine map; `RET 4`: the other swap): true
/// (different) without another swap, when a material has a map on one side
/// only, maps of different counts, a pair whose old data the other swap
/// maps elsewhere ([`fn_0058f6f0`]), or different footstep names.
pub fn impact_swap_compare(e: &mut Engine, this: Ptr, other: u32) -> bool {
    if other == 0 {
        return true;
    }
    for material in 0..SWAP_MAPS {
        let own = e.mem.u32(this.addr() + material * 4);
        let theirs = e.mem.u32(other + material * 4);
        if own == 0 || theirs == 0 {
            if own != 0 || theirs != 0 {
                return true;
            }
            continue;
        }
        let own_count = e.call(MAP_COUNT, &args![own]).u32();
        if own_count != e.call(MAP_COUNT, &args![theirs]).u32() {
            return true;
        }
        let differs = e.with_stack(0xc, |e, words| {
            let position = words.addr();
            let first = e.call(MAP_FIRST, &args![own]).u32();
            e.mem.set_u32(position, first);
            while e.mem.u32(position) != 0 {
                e.mem.set_u32(position + 4, 0);
                e.mem.set_u32(position + 8, 0);
                e.call(MAP_NEXT, &args![own, position, position + 4, position + 8]);
                let old = e.mem.u32(position + 4);
                if fn_0058f6f0(e, Ptr::new(other), material, old) != e.mem.u32(position + 8) {
                    return true;
                }
            }
            false
        });
        if differs {
            return true;
        }
    }
    e.call(
        MEMCMP,
        &args![
            this.addr() + SWAP_NAMES,
            other + SWAP_NAMES,
            SWAP_NAMES_SIZE
        ],
    )
    .u32()
        != 0
}

// Translated from 0058f6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImpactSwap`'s lookup (`RET 8`: a material and an impact data; the
/// engine map has no name): with a map for the material, the data it maps
/// that impact data to (`00853130`; 0 when it has none); without one, the
/// impact data itself.
pub fn fn_0058f6f0(e: &mut Engine, this: Ptr, material: u32, data: u32) -> u32 {
    let map = e
        .mem
        .u32(this.addr().wrapping_add(material.wrapping_mul(4)));
    if map == 0 {
        return data;
    }
    e.with_stack(4, |e, found| {
        e.call(MAP_LOOKUP, &args![map, data, found]);
        e.mem.u32(found.addr())
    })
}

// Translated from 0058f740 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0058f740 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `ImpactSwap`'s add (`RET 0xC`: material, old and new impact data; the
/// engine map has no name): nothing when old and new are the same;
/// otherwise the material's map is made when missing (`00401000` of 0x10
/// bytes, [`fn_0058f820`] with 37 buckets, null when the allocation fails)
/// and the pair set in it (`BSMapBase::SetAt`).
pub fn fn_0058f740(e: &mut Engine, this: Ptr, material: u32, old: u32, new: u32) {
    if old == new {
        return;
    }
    let slot = this.addr().wrapping_add(material.wrapping_mul(4));
    if e.mem.u32(slot) == 0 {
        let block = e.call(ALLOCATE, &args![0x10u32]).u32();
        let map = if block == 0 {
            0
        } else {
            fn_0058f820(e, Ptr::new(block), 0x25).addr()
        };
        e.mem.set_u32(slot, map);
    }
    let map = e.mem.u32(slot);
    e.call(MAP_SET_AT, &args![map, old, new]);
}

// Translated from 0058f820 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSMap<BGSImpactData *,BGSImpactData *>` constructor (`RET 4`: the
/// bucket count; the engine map has no name): the base ([`fn_0058f880`]),
/// then the map's own vtable (`01033470`). Returns the map.
pub fn fn_0058f820(e: &mut Engine, this: Ptr, buckets: u32) -> Ptr {
    fn_0058f880(e, this, buckets);
    e.mem.set_u32(this.addr(), MAP_VTABLE);
    this
}

// Translated from 0058f880 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSMapBase<BGSImpactData *,BGSImpactData *>` constructor (`RET 4`:
/// the bucket count; the engine map has no name): its vtable (`01033490`),
/// the bucket count at +4, no entries (+0xC), and with buckets a zeroed
/// table of that many words at +8 (`00401000`, `_memset`). Returns the map.
pub fn fn_0058f880(e: &mut Engine, this: Ptr, buckets: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, MAP_BASE_VTABLE);
    e.mem.set_u32(base + 4, buckets);
    e.mem.set_u32(base + 0xc, 0);
    if buckets != 0 {
        let size = e.mem.u32(base + 4) << 2;
        let table = e.call(ALLOCATE, &args![size]).u32();
        e.mem.set_u32(base + 8, table);
        let size = e.mem.u32(base + 4) << 2;
        let table = e.mem.u32(base + 8);
        e.call(CRT_MEMSET, &args![table, 0u32, size]);
    }
    this
}

// Translated from 0058f900 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0058f900 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// The `BSMap<BGSImpactData *,BGSImpactData *>` destructor body (the
/// engine map has no name; `ECX`): its vtable (`01033470`), every entry
/// removed (`00438af0`), then the base's destructor body ([`fn_0058f960`]).
pub fn fn_0058f900(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), MAP_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    fn_0058f960(e, this);
}

// Translated from 0058f960 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSMapBase<BGSImpactData *,BGSImpactData *>` destructor body (the
/// engine map has no name; `ECX`): its vtable (`01033490`) and the bucket
/// table (+8) freed when there is one.
pub fn fn_0058f960(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), MAP_BASE_VTABLE);
    let table = e.mem.u32(this.addr() + 8);
    if table != 0 {
        e.call(FREE, &args![table]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0058_eb30, fn_0058eb30(Ptr, u32)),
        entry!(0x0058_eb90, fn_0058eb90(Ptr, u32) -> bool),
        entry!(0x0058_ed50, fn_0058ed50(Ptr)),
        entry!(0x0058_efd0, fn_0058efd0(Ptr) -> Ptr),
        entry!(0x0058_f010, fn_0058f010(Ptr)),
        entry!(0x0058_f080, impact_swap_save(Ptr)),
        entry!(0x0058_f180, fn_0058f180(Ptr, Ptr)),
        entry!(0x0058_f210, impact_swap_init_item(Ptr, u32)),
        entry!(0x0058_f4c0, fn_0058f4c0(Ptr, u32)),
        entry!(0x0058_f5c0, impact_swap_compare(Ptr, u32) -> bool),
        entry!(0x0058_f6f0, fn_0058f6f0(Ptr, u32, u32) -> u32),
        entry!(0x0058_f740, fn_0058f740(Ptr, u32, u32, u32)),
        entry!(0x0058_f820, fn_0058f820(Ptr, u32) -> Ptr),
        entry!(0x0058_f880, fn_0058f880(Ptr, u32) -> Ptr),
        entry!(0x0058_f900, fn_0058f900(Ptr)),
        entry!(0x0058_f960, fn_0058f960(Ptr)),
        entry!(0x0058_f7f0, fn_0058f7f0(Ptr, i32) -> u32),
        entry!(0x0058_e810, fn_0058e810(Ptr) -> Ptr),
        entry!(0x0058_f850, bs_map_bgs_impact_data_p_bgs_impact_data_p_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0058_f9a0, bs_map_base_bgs_impact_data_p_bgs_impact_data_p_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0058_e9d0, fn_0058e9d0(Ptr, u32) -> u32),
        entry!(0x0058_eb10, fn_0058eb10(Ptr)),
        entry!(0x0058_e8f0, fn_0058e8f0(u32) -> u32),
        entry!(0x0058_e8c0, fn_0058e8c0(Ptr)),
        entry!(0x0058_eeb0, fn_0058eeb0(u32)),
        entry!(0x0058_edd0, fn_0058edd0(Ptr)),
        entry!(0x0058_ea10, fn_0058ea10()),
        entry!(0x0058_ef10, fn_0058ef10(Ptr, u32, u32)),
        entry!(0x0058_ec00, fn_0058ec00(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_0058e810_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0058_e810,
            size: 0x4c,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x004f_9d90, &[A::T(0x18)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x5f)]),
                (0x0058_eb10, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0103_323c), (0x18, 4, 0x0103_3224)],
        });
    }
    #[test]
    fn at_0058f850_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0058_f900, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0058_f850, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0058_f850)
                .collect();
            let mut expected = vec![(0x0058_f900, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_0058f9a0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0058_f960, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0058_f9a0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0058_f9a0)
                .collect();
            let mut expected = vec![(0x0058_f960, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod impact_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011c_a000, 0x1000);
        e.map(0x011c_3000, 0x1000);
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn fn_0058e9d0_reads_a_slot_or_asks_the_override() {
        let mut e = engine();
        let set = e.mem.alloc(0x4c);
        e.mem.set_u32(set + 0x1c + 3 * 4, 0x0d33);
        assert_eq!(e.call(0x0058_e9d0, &args![set, 3u32]).u32(), 0x0d33);
        e.set_global(OVERRIDE_SET, 0x0777u32);
        e.register(OVERRIDE_LOOKUP_0058F6F0, |_, a| (a[2] + a[1]).into_ret());
        assert_eq!(e.call(0x0058_e9d0, &args![set, 3u32]).u32(), 0x0d36);
        assert_eq!(
            calls(&e, OVERRIDE_LOOKUP_0058F6F0),
            vec![vec![0x0777, 3, 0x0d33]]
        );
    }

    #[test]
    fn fn_0058eb10_clears_the_slots() {
        let mut e = engine();
        e.register(MEMSET, |_, a| a[0].into_ret());
        e.call(0x0058_eb10, &args![0x5000u32]);
        assert_eq!(calls(&e, MEMSET), vec![vec![0x501c, 0, 0x30]]);
    }

    #[test]
    fn fn_0058e8f0_maps_materials() {
        let mut e = engine();
        let want = [
            0, 7, 1, 3, 2, 4, 6, 6, 8, 5, 0, 4, 5, 4, 4, 4, 9, 9, 1, 0, 4, 4, 9, 9, 3, 9, 4, 4, 9,
            9, 5, 5,
        ];
        for (material, slot) in want.iter().enumerate() {
            assert_eq!(
                e.call(0x0058_e8f0, &args![material as u32]).u32(),
                *slot,
                "material {material}"
            );
        }
        assert_eq!(e.call(0x0058_e8f0, &args![0x20u32]).u32(), 4);
        assert_eq!(e.call(0x0058_e8f0, &args![u32::MAX]).u32(), 4);
    }

    #[test]
    fn fn_0058e8c0_stores_the_vtables_then_destroys() {
        let mut e = engine();
        e.register(0x0048_3630, |_, _| Ret::default());
        let this = e.mem.alloc(0x4c);
        e.call(0x0058_e8c0, &args![this]);
        assert_eq!(e.mem.u32(this), 0x0103_323c);
        assert_eq!(e.mem.u32(this + 0x18), 0x0103_3224);
        assert_eq!(calls(&e, 0x0048_3630), vec![vec![this]]);
    }

    #[test]
    fn fn_0058eeb0_hands_out_each_material() {
        let mut e = engine();
        e.register(SET_IMPACT_0058F7F0, |_, a| (0x100 + a[1]).into_ret());
        e.register(SET_MATERIAL_IMPACT_008374E0, |_, _| Ret::default());
        e.register(MATERIALS_SET_00837500, |_, _| Ret::default());
        e.call(0x0058_eeb0, &args![0x0888u32]);
        assert_eq!(e.global::<u32>(OVERRIDE_SET), 0x0888);
        let handed = calls(&e, SET_MATERIAL_IMPACT_008374E0);
        assert_eq!(handed.len(), 10);
        assert_eq!(handed[9], vec![9, 0x109]);
        assert_eq!(calls(&e, MATERIALS_SET_00837500).len(), 1);
        e.call(0x0058_eeb0, &args![0u32]);
        assert_eq!(e.global::<u32>(OVERRIDE_SET), 0);
        assert_eq!(calls(&e, SET_IMPACT_0058F7F0).len(), 10);
        assert_eq!(calls(&e, SET_MATERIAL_IMPACT_008374E0)[10], vec![0, 0]);
    }

    fn init_rig(flag_set: bool) -> (Engine, u32) {
        let mut e = engine();
        e.register_double(FORM_FLAG_8_SET, move |_, _| flag_set.into_ret());
        e.register(SET_FORM_FLAG_8, |_, _| Ret::default());
        e.register(GET_FILE, |_, _| 0x0777u32.into_ret());
        e.register(ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        // Ids ending in 1 have no form.
        e.register(LOOKUP_FORM, |_, a| {
            if a[0] & 0xf == 1 {
                0u32.into_ret()
            } else {
                (a[0] + 0x0900_0000).into_ret()
            }
        });
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(FORM_ID_0084E3A0, |_, _| 0x4444u32.into_ret());
        e.register(LOG, |_, _| Ret::default());
        e.put_vtable(0x0f00_f000, &[0; 0x4d]);
        e.mem.set_u32(0x0f00_f130, 0x0f00_f400);
        e.register(0x0f00_f400, |_, _| 0x0abc_0000u32.into_ret());
        let this = e.mem.alloc(0x4c);
        e.mem.set_u32(this, 0x0f00_f000);
        e.mem.set_u32(this + 0x1c, 0x20);
        e.mem.set_u32(this + 0x1c + 11 * 4, 0x31);
        (e, this)
    }

    #[test]
    fn fn_0058edd0_resolves_the_slots() {
        let (mut e, this) = init_rig(false);
        e.call(0x0058_edd0, &args![this]);
        assert_eq!(e.mem.u32(this + 0x1c), 0x0a00_0020);
        assert_eq!(e.mem.u32(this + 0x1c + 11 * 4), 0);
        assert_eq!(e.mem.u32(this + 0x20), 0);
        assert_eq!(calls(&e, GET_FILE).len(), 2);
        assert_eq!(
            calls(&e, DYNAMIC_CAST)[0],
            vec![0x0a00_0020, 0, TYPE_TES_FORM, TYPE_IMPACT_DATA, 0]
        );
        assert_eq!(
            calls(&e, LOG),
            vec![vec![
                MESSAGE_NO_IMPACT_DATA,
                0x0100_0031,
                0x0abc_0000,
                0x4444
            ]]
        );
        assert_eq!(calls(&e, SET_FORM_FLAG_8), vec![vec![this, 1]]);
        let (mut e, this) = init_rig(true);
        e.call(0x0058_edd0, &args![this]);
        assert!(calls(&e, GET_FILE).is_empty());
        assert!(calls(&e, SET_FORM_FLAG_8).is_empty());
    }

    fn default_rig(found: u32) -> Engine {
        let mut e = engine();
        e.register_double(LOOKUP_FORM, move |_, _| found.into_ret());
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        // The constructor's callees.
        for addr in [0x0048_3370u32, 0x004f_9d90, 0x004f_15a0] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(MEMSET, |_, a| a[0].into_ret());
        e.put_vtable(0x0103_3000, &[0; 0x200]);
        e.mem.set_u32(0x0103_323c + 0x128, 0x0f01_0128);
        e.mem.set_u32(0x0103_323c + 0x134, 0x0f01_0134);
        e.register(0x0f01_0128, |_, _| Ret::default());
        e.register(0x0f01_0134, |_, _| Ret::default());
        e.set_global(DATA_HANDLER, 0x0555_0000u32);
        e.register(ADD_FORM_TO_DATA_HANDLER, |_, _| Ret::default());
        e
    }

    #[test]
    fn fn_0058ea10_keeps_a_loaded_default_set() {
        let mut e = default_rig(0x0d00_0000);
        e.call(0x0058_ea10, &args![]);
        assert_eq!(e.global::<u32>(DEFAULT_SET), 0x0d00_0000);
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![0x0d00_0000, 0, TYPE_TES_FORM, TYPE_IMPACT_DATA_SET, 0]]
        );
        assert_eq!(calls(&e, LOOKUP_FORM), vec![vec![DEFAULT_SET_ID]]);
        assert!(calls(&e, ALLOCATE).is_empty());
    }

    #[test]
    fn fn_0058ea10_makes_the_default_set() {
        let mut e = default_rig(0);
        e.call(0x0058_ea10, &args![]);
        let set = e.global::<u32>(DEFAULT_SET);
        assert_ne!(set, 0);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![0x4c]]);
        assert_eq!(e.mem.u32(set), 0x0103_323c);
        assert_eq!(calls(&e, 0x0f01_0128), vec![vec![set, DEFAULT_SET_ID, 1]]);
        assert_eq!(calls(&e, 0x0f01_0134), vec![vec![set, DEFAULT_SET_NAME]]);
        assert_eq!(
            calls(&e, ADD_FORM_TO_DATA_HANDLER),
            vec![vec![0x0555_0000, set]]
        );
    }

    #[test]
    fn fn_0058ef10_queues_models_and_texture_sets() {
        let mut e = engine();
        let set = e.mem.alloc(0x4c);
        let named = e.mem.alloc(0x50);
        let plain = e.mem.alloc(0x50);
        e.mem.set_u32(named + 0x48, 0x0ccc_0000);
        e.mem.set_u32(set + 0x1c, named);
        e.mem.set_u32(set + 0x1c + 5 * 4, plain);
        e.register_double(MODEL_NAME_LENGTH_0048CEE0, move |_, a| {
            u32::from(a[0] == named + 0x18).into_ret()
        });
        e.set_global(MODEL_LOADER, 0x0666_0000u32);
        e.register(MODEL_LOADER_QUEUE_00443D30, |_, _| Ret::default());
        e.register(WORD_48_006733E0, |e, a| e.mem.u32(a[0] + 0x48).into_ret());
        e.register(QUEUE_TEXTURE_SET, |_, _| Ret::default());
        e.call(0x0058_ef10, &args![set + 0x18, 0x11u32, 0x22u32]);
        assert_eq!(
            calls(&e, MODEL_LOADER_QUEUE_00443D30),
            vec![vec![0x0666_0000, named + 0x18, 0x11, 0x22, 1, 1, 0, 0]]
        );
        assert_eq!(
            calls(&e, QUEUE_TEXTURE_SET),
            vec![vec![0x0ccc_0000, 0x11, 0x22, 0]]
        );
        assert_eq!(calls(&e, WORD_48_006733E0), vec![vec![named], vec![plain]]);
    }

    type Script = Rc<RefCell<(Vec<(u32, u32)>, usize)>>;

    const FILE: u32 = 0x0e00_0000;

    fn load_rig(chunks: Vec<(u32, u32)>, swap: bool) -> (Engine, u32) {
        let mut e = engine();
        let script: Script = Rc::new(RefCell::new((chunks, 0)));
        e.register(GET_TES_FORM, |_, _| 0x5fu32.into_ret());
        e.register(LOAD_FORM, |_, _| Ret::default());
        let s = script.clone();
        e.register_double(GET_TES_CHUNK, move |_, _| {
            let s = s.borrow();
            s.0.get(s.1).map_or(0, |c| c.0).into_ret()
        });
        let s = script.clone();
        e.register_double(CHUNK_SIZE, move |_, _| {
            let s = s.borrow();
            s.0[s.1].1.into_ret()
        });
        let s = script;
        e.register_double(NEXT_CHUNK, move |_, _| {
            let mut s = s.borrow_mut();
            s.1 += 1;
            (s.1 < s.0.len()).into_ret()
        });
        e.register(GET_CHUNK_DATA_SIZED, |_, _| Ret::default());
        e.register(SWAP_WORD, |_, _| Ret::default());
        e.register_double(NEEDS_SWAP, move |_, _| swap.into_ret());
        e.put_vtable(0x0f01_1000, &[0; 0x4e]);
        e.mem.set_u32(0x0f01_10e0, 0x0f01_1100);
        e.mem.set_u32(0x0f01_1134, 0x0f01_1200);
        e.register(0x0f01_1100, |_, _| Ret::default());
        e.register(0x0f01_1200, |_, _| Ret::default());
        let this = e.mem.alloc(0x4c);
        e.mem.set_u32(this, 0x0f01_1000);
        e.call_log = Some(vec![]);
        (e, this)
    }

    #[test]
    fn fn_0058ec00_checks_the_whole_form_type_word() {
        let (mut e, this) = load_rig(vec![], false);
        e.register(GET_TES_FORM, |_, _| 0x15fu32.into_ret());
        assert!(!e.call(0x0058_ec00, &args![this, FILE]).bool());
        assert!(calls(&e, LOAD_FORM).is_empty());
    }

    #[test]
    fn fn_0058ec00_reads_each_chunk() {
        let chunks = vec![(CHUNK_EDID, 9), (CHUNK_OBND, 0xc), (CHUNK_DATA, 0x30)];
        let (mut e, this) = load_rig(chunks, true);
        assert!(e.call(0x0058_ec00, &args![this, FILE]).bool());
        assert_eq!(calls(&e, LOAD_FORM), vec![vec![this, FILE]]);
        let edid = calls(&e, 0x0f01_1200);
        assert_eq!(
            calls(&e, GET_CHUNK_DATA_SIZED),
            vec![vec![FILE, edid[0][1], 0x200], vec![FILE, this + 0x1c, 0x30]]
        );
        assert_eq!(calls(&e, 0x0f01_1100), vec![vec![this, FILE]]);
        let swapped = calls(&e, SWAP_WORD);
        assert_eq!(swapped.len(), 12);
        assert_eq!(swapped[11], vec![this + 0x1c + 11 * 4, 0]);
        let (mut e, this) = load_rig(vec![(CHUNK_DATA, 0x30)], false);
        assert!(e.call(0x0058_ec00, &args![this, FILE]).bool());
        assert!(calls(&e, SWAP_WORD).is_empty());
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    #[test]
    fn fn_0058f7f0_answers_a_present_entry_or_null() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x30 + 3 * 0x1e);
        e.mem.set_u8(this + 0x30 + 0x1e, 0x80);
        assert_eq!(e.call(0x0058_f7f0, &args![this, 0u32]).u32(), 0);
        assert_eq!(
            e.call(0x0058_f7f0, &args![this, 1u32]).u32(),
            this + 0x30 + 0x1e
        );
        assert_eq!(e.call(0x0058_f7f0, &args![this, 2u32]).u32(), 0);
    }
}

#[cfg(test)]
mod n085_tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::BTreeMap;
    use std::rc::Rc;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    type Maps = Rc<RefCell<BTreeMap<u32, Vec<(u32, u32)>>>>;

    /// Map doubles over Rust vectors keyed by the map's address: the
    /// position is the next index plus one.
    fn map_doubles(e: &mut Engine) -> Maps {
        let maps: Maps = Rc::new(RefCell::new(BTreeMap::new()));
        let m = maps.clone();
        e.register_double(MAP_FIRST, move |_, a| {
            ret(u32::from(
                m.borrow().get(&a[0]).is_some_and(|v| !v.is_empty()),
            ))
        });
        let m = maps.clone();
        e.register_double(MAP_NEXT, move |e, a| {
            let at = e.mem.u32(a[1]) as usize - 1;
            let pairs = m.borrow()[&a[0]].clone();
            e.mem.set_u32(a[2], pairs[at].0);
            e.mem.set_u32(a[3], pairs[at].1);
            let next = if at + 1 < pairs.len() {
                at as u32 + 2
            } else {
                0
            };
            e.mem.set_u32(a[1], next);
            Ret::default()
        });
        let m = maps.clone();
        e.register_double(MAP_COUNT, move |_, a| {
            ret(m.borrow().get(&a[0]).map_or(0, |v| v.len() as u32))
        });
        let m = maps.clone();
        e.register_double(MAP_REMOVE_ALL, move |_, a| {
            m.borrow_mut().insert(a[0], vec![]);
            Ret::default()
        });
        let m = maps.clone();
        e.register_double(MAP_SET_AT, move |_, a| {
            m.borrow_mut().entry(a[0]).or_default().push((a[1], a[2]));
            Ret::default()
        });
        let m = maps.clone();
        e.register_double(MAP_LOOKUP, move |e, a| {
            if let Some(&(_, v)) = m.borrow()[&a[0]].iter().find(|p| p.0 == a[1]) {
                e.mem.set_u32(a[2], v);
            }
            Ret::default()
        });
        maps
    }

    #[test]
    fn set_copy_and_compare_use_the_twelve_slots() {
        for cast in [0u32, 0x9000] {
            let mut e = Engine::new();
            let this: Ptr = Ptr::new(e.mem.alloc(0x4c));
            e.register_double(DYNAMIC_CAST, move |_, _| ret(cast));
            quiet(&mut e, &[0x0048_51b0, MEMCPY]);
            e.register(0x0048_5270, |_, _| ret(0));
            e.register(MEMCMP, |_, _| ret(0));
            e.call_log = Some(vec![]);
            e.call(0x0058_eb30, &args![this, 0x5000u32]);
            let different = e.call(0x0058_eb90, &args![this, 0x5000u32]).bool();
            assert_eq!(
                calls(&e, DYNAMIC_CAST)[0],
                vec![0x5000, 0, TYPE_TES_FORM, TYPE_IMPACT_DATA_SET, 0]
            );
            if cast == 0 {
                assert!(calls(&e, MEMCPY).is_empty());
                assert!(different);
            } else {
                assert_eq!(
                    calls(&e, MEMCPY),
                    vec![vec![this.addr() + 0x1c, cast + 0x1c, 0x30]]
                );
                assert_eq!(
                    calls(&e, MEMCMP),
                    vec![vec![this.addr() + 0x1c, cast + 0x1c, 0x30]]
                );
                assert!(!different);
            }
        }
        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(0x4c));
        e.register(DYNAMIC_CAST, |_, _| ret(0x9000));
        e.register(0x0048_5270, |_, _| ret(1));
        assert!(e.call(0x0058_eb90, &args![this, 0x5000u32]).bool());
        e.register(0x0048_5270, |_, _| ret(0));
        e.register(MEMCMP, |_, _| ret(5));
        assert!(e.call(0x0058_eb90, &args![this, 0x5000u32]).bool());
    }

    #[test]
    fn set_save_writes_the_form_ids() {
        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(0x4c));
        e.mem.set_u32(this.addr() + 0x1c, 0x100);
        e.mem.set_u32(this.addr() + 0x1c + 11 * 4, 0x200);
        e.register(FORM_ID_0084E3A0, |_, a| ret(a[0] + 1));
        quiet(&mut e, &[START_FORM, CLOSE_FORM]);
        let seen = Rc::new(RefCell::new(vec![]));
        let s = seen.clone();
        e.register_double(ADD_CHUNK_ARRAY_WORDS, move |e, a| {
            s.borrow_mut()
                .extend((0..a[2]).map(|i| e.mem.u32(a[1] + 4 * i)));
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0058_ed50, &args![this]);
        assert_eq!(calls(&e, START_FORM), vec![vec![this.addr()]]);
        assert_eq!(calls(&e, ADD_CHUNK_ARRAY_WORDS)[0][0], CHUNK_DATA);
        let mut expected = vec![0u32; 12];
        expected[0] = 0x101;
        expected[11] = 0x201;
        assert_eq!(*seen.borrow(), expected);
        assert_eq!(calls(&e, CLOSE_FORM), vec![vec![this.addr()]]);
    }

    #[test]
    fn swap_constructor_and_destructor() {
        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(0x15c));
        quiet(&mut e, &[MEMSET]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0058_efd0, &args![this]).u32(), this.addr());
        assert_eq!(
            calls(&e, MEMSET),
            vec![vec![this.addr(), 0, 0x30], vec![this.addr() + 0x30, 0, 300]]
        );
        // The destructor deletes each map through vtable slot 0 with 1.
        let map = e.mem.alloc(0x10);
        let vtable = e.mem.alloc(4);
        e.mem.set_u32(vtable, 0x7400_0000);
        e.mem.set_u32(map, vtable);
        e.mem.set_u32(this.addr() + 8, map);
        quiet(&mut e, &[0x7400_0000]);
        e.call_log = Some(vec![]);
        e.call(0x0058_f010, &args![this]);
        assert_eq!(calls(&e, 0x7400_0000), vec![vec![map, 1]]);
        assert_eq!(e.mem.u32(this.addr() + 8), 0);
    }

    #[test]
    fn swap_save_writes_each_pair_and_the_names() {
        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(0x15c));
        let maps = map_doubles(&mut e);
        e.mem.set_u32(this.addr() + 4 * 3, 0x6000);
        maps.borrow_mut()
            .insert(0x6000, vec![(0x10, 0), (0x20, 0x30)]);
        e.register(FORM_ID_0084E3A0, |_, a| ret(a[0] + 0x100));
        e.register(SAVE_SWAPS_ENDIAN, |_, _| ret(1));
        quiet(&mut e, &[SWAP_RECORD, ADD_CHUNK_ARRAY]);
        let seen = Rc::new(RefCell::new(vec![]));
        let s = seen.clone();
        e.register_double(ADD_CHUNK_DATA, move |e, a| {
            s.borrow_mut().push([0, 4, 8].map(|o| e.mem.u32(a[1] + o)));
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0058_f080, &args![this]);
        assert_eq!(*seen.borrow(), vec![[3, 0x110, 0], [3, 0x120, 0x130]]);
        assert!(calls(&e, ADD_CHUNK_DATA)
            .iter()
            .all(|a| a[0] == CHUNK_IMPS && a[2] == 0xc));
        assert_eq!(calls(&e, SWAP_RECORD).len(), 4);
        assert_eq!(
            calls(&e, ADD_CHUNK_ARRAY),
            vec![vec![CHUNK_IMPF, this.addr() + 0x30, 300]]
        );
    }

    #[test]
    fn swap_load_reads_the_names_or_one_pair() {
        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(0x15c));
        let file: Ptr = Ptr::new(e.mem.alloc(4));
        let _ = map_doubles(&mut e);
        e.register(MEMSET, |_, _| Ret::default());
        e.register(GET_CHUNK_DATA_SIZED, |e, a| {
            if a[2] == 0xc {
                for (i, w) in [2u32, 0x10, 0x20].iter().enumerate() {
                    e.mem.set_u32(a[1] + 4 * i as u32, *w);
                }
            }
            Ret::default()
        });
        e.register(NEEDS_SWAP, |_, _| ret(1));
        quiet(&mut e, &[SWAP_RECORD]);
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(CRT_MEMSET, |_, _| Ret::default());
        e.register(GET_TES_CHUNK, |_, _| ret(CHUNK_IMPS));
        e.call_log = Some(vec![]);
        e.call(0x0058_f180, &args![this, file]);
        assert_eq!(calls(&e, SWAP_RECORD).len(), 1);
        // A new map for material 2 with the pair.
        let map = e.mem.u32(this.addr() + 8);
        assert_ne!(map, 0);
        assert_eq!(calls(&e, MAP_SET_AT), vec![vec![map, 0x10, 0x20]]);
        e.register(GET_TES_CHUNK, |_, _| ret(CHUNK_IMPF));
        e.call_log = Some(vec![]);
        e.call(0x0058_f180, &args![this, file]);
        assert_eq!(
            calls(&e, GET_CHUNK_DATA_SIZED),
            vec![vec![file.addr(), this.addr() + 0x30, 300]]
        );
        e.register(GET_TES_CHUNK, |_, _| ret(CHUNK_DATA));
        e.call_log = Some(vec![]);
        e.call(0x0058_f180, &args![this, file]);
        assert!(calls(&e, GET_CHUNK_DATA_SIZED).is_empty());
    }

    #[test]
    fn swap_init_item_resolves_the_pairs_and_drops_empty_maps() {
        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(0x15c));
        let maps = map_doubles(&mut e);
        let form = e.mem.alloc(8);
        let vtable = e.mem.alloc(0x134);
        e.mem.set_u32(form, vtable);
        e.mem.set_u32(vtable + 0x130, 0x7500_0130);
        e.register(0x7500_0130, |_, _| ret(0x4e41));
        // Material 0: one pair resolved, one with both sides missing.
        // Material 1: nothing resolves, so the map is deleted.
        e.mem.set_u32(this.addr(), 0x6000);
        let map_vtable = e.mem.alloc(4);
        e.mem.set_u32(map_vtable, 0x7600_0000);
        let second = e.mem.alloc(0x10);
        e.mem.set_u32(second, map_vtable);
        e.mem.set_u32(this.addr() + 4, second);
        maps.borrow_mut()
            .insert(0x6000, vec![(0x11, 0x12), (0x91, 0x92)]);
        maps.borrow_mut().insert(second, vec![(0x93, 0)]);
        quiet(&mut e, &[0x7600_0000, LOG, FREE]);
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(GET_FILE, |_, _| ret(0x7f));
        e.register(ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        e.register(LOOKUP_FORM, |_, a| ret(a[0]));
        e.register(DYNAMIC_CAST, |_, a| {
            ret(if a[0] & 0xf0 == 0x10 {
                a[0] + 0x1000
            } else {
                0
            })
        });
        // `0084e3a0` is both the map count and the form id: the count for
        // the maps, 0xABCD for the form.
        let m = maps.clone();
        e.register_double(FORM_ID_0084E3A0, move |_, a| {
            ret(m.borrow().get(&a[0]).map_or(0xabcd, |v| v.len() as u32))
        });
        e.call_log = Some(vec![]);
        e.call(0x0058_f210, &args![this, form]);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![24], vec![12]]);
        assert_eq!(maps.borrow()[&0x6000], vec![(0x0100_1011, 0x0100_1012)]);
        let logged = calls(&e, LOG);
        assert_eq!(
            logged,
            vec![
                vec![MESSAGE_NO_OLD_DATA, 0x0100_0091, 0x4e41, 0xabcd],
                vec![MESSAGE_NO_NEW_DATA, 0x0100_0092, 0x4e41, 0xabcd],
                vec![MESSAGE_NO_OLD_DATA, 0x0100_0093, 0x4e41, 0xabcd],
            ]
        );
        assert_eq!(calls(&e, GET_FILE)[0], vec![form, u32::MAX]);
        assert_eq!(calls(&e, FREE).len(), 2);
        assert_eq!(calls(&e, 0x7600_0000), vec![vec![second, 1]]);
        assert_eq!(e.mem.u32(this.addr() + 4), 0);
        assert_eq!(e.mem.u32(this.addr()), 0x6000);
    }

    #[test]
    fn swap_copy_replaces_the_maps_and_names() {
        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(0x15c));
        let source = e.mem.alloc(0x15c);
        let maps = map_doubles(&mut e);
        e.mem.set_u32(source + 4 * 5, 0x6000);
        maps.borrow_mut().insert(0x6000, vec![(1, 2), (3, 3)]);
        let old_map = e.mem.alloc(0x10);
        e.mem.set_u32(this.addr() + 4 * 5, old_map);
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(CRT_MEMSET, |_, _| Ret::default());
        let map_vtable = e.mem.alloc(4);
        e.mem.set_u32(map_vtable, 0x7600_0000);
        e.mem.set_u32(old_map, map_vtable);
        quiet(&mut e, &[0x7600_0000, MEMCPY]);
        e.call_log = Some(vec![]);
        e.call(0x0058_f4c0, &args![this, source]);
        assert_eq!(calls(&e, 0x7600_0000), vec![vec![old_map, 1]]);
        let made = e.mem.u32(this.addr() + 4 * 5);
        assert_ne!(made, old_map);
        // (3, 3) is the same on both sides: not added.
        assert_eq!(calls(&e, MAP_SET_AT), vec![vec![made, 1, 2]]);
        assert_eq!(e.mem.u32(made), MAP_VTABLE);
        assert_eq!(e.mem.u32(made + 4), 0x25);
        assert_eq!(
            calls(&e, MEMCPY),
            vec![vec![this.addr() + 0x30, source + 0x30, 300]]
        );
    }

    #[test]
    fn swap_compare_checks_maps_pairs_and_names() {
        let setup = |e: &mut Engine| {
            let this: Ptr = Ptr::new(e.mem.alloc(0x15c));
            let other = e.mem.alloc(0x15c);
            let maps = map_doubles(e);
            e.mem.set_u32(this.addr(), 0x6000);
            e.mem.set_u32(other, 0x6100);
            maps.borrow_mut().insert(0x6000, vec![(1, 2)]);
            maps.borrow_mut().insert(0x6100, vec![(1, 2)]);
            e.register(MEMCMP, |_, _| ret(0));
            (this, other, maps)
        };
        let mut e = Engine::new();
        let (this, other, maps) = setup(&mut e);
        assert!(!e.call(0x0058_f5c0, &args![this, other]).bool());
        assert!(e.call(0x0058_f5c0, &args![this, 0u32]).bool());
        // A different value for the key.
        maps.borrow_mut().insert(0x6100, vec![(1, 9)]);
        assert!(e.call(0x0058_f5c0, &args![this, other]).bool());
        // Different counts.
        maps.borrow_mut().insert(0x6100, vec![(1, 2), (4, 5)]);
        assert!(e.call(0x0058_f5c0, &args![this, other]).bool());
        // A map on one side only.
        let mut e = Engine::new();
        let (this, other, _) = setup(&mut e);
        e.mem.set_u32(other + 4, 0x6200);
        assert!(e.call(0x0058_f5c0, &args![this, other]).bool());
        // Different names.
        let mut e = Engine::new();
        let (this, other, _) = setup(&mut e);
        e.register(MEMCMP, |_, _| ret(1));
        e.call_log = Some(vec![]);
        assert!(e.call(0x0058_f5c0, &args![this, other]).bool());
        assert_eq!(
            calls(&e, MEMCMP),
            vec![vec![this.addr() + 0x30, other + 0x30, 300]]
        );
    }

    #[test]
    fn swap_lookup_maps_or_passes_through() {
        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(0x15c));
        let maps = map_doubles(&mut e);
        assert_eq!(e.call(0x0058_f6f0, &args![this, 4u32, 0x77u32]).u32(), 0x77);
        e.mem.set_u32(this.addr() + 16, 0x6000);
        maps.borrow_mut().insert(0x6000, vec![(0x77, 0x88)]);
        assert_eq!(e.call(0x0058_f6f0, &args![this, 4u32, 0x77u32]).u32(), 0x88);
        assert_eq!(e.call(0x0058_f6f0, &args![this, 4u32, 0x66u32]).u32(), 0);
    }

    #[test]
    fn swap_add_skips_identity_and_survives_a_failed_allocation() {
        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(0x15c));
        let _ = map_doubles(&mut e);
        e.register(ALLOCATE, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(0x0058_f740, &args![this, 1u32, 5u32, 5u32]);
        assert!(calls(&e, ALLOCATE).is_empty());
        e.call(0x0058_f740, &args![this, 1u32, 5u32, 6u32]);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![0x10]]);
        assert_eq!(calls(&e, MAP_SET_AT), vec![vec![0, 5, 6]]);
    }

    #[test]
    fn map_constructors_and_destructors() {
        let mut e = Engine::new();
        let map: Ptr = Ptr::new(e.mem.alloc(0x10));
        e.register(ALLOCATE, |_, _| ret(0x5550));
        quiet(&mut e, &[CRT_MEMSET, MAP_REMOVE_ALL, FREE]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0058_f880, &args![map, 3u32]).u32(), map.addr());
        assert_eq!(e.mem.u32(map.addr()), MAP_BASE_VTABLE);
        assert_eq!(e.mem.u32(map.addr() + 4), 3);
        assert_eq!(e.mem.u32(map.addr() + 8), 0x5550);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![12]]);
        assert_eq!(calls(&e, CRT_MEMSET), vec![vec![0x5550, 0, 12]]);
        assert_eq!(e.call(0x0058_f820, &args![map, 0u32]).u32(), map.addr());
        assert_eq!(e.mem.u32(map.addr()), MAP_VTABLE);
        assert_eq!(calls(&e, ALLOCATE).len(), 1);
        e.mem.set_u32(map.addr() + 8, 0x5550);
        e.call_log = Some(vec![]);
        e.call(0x0058_f900, &args![map]);
        assert_eq!(calls(&e, MAP_REMOVE_ALL), vec![vec![map.addr()]]);
        assert_eq!(calls(&e, FREE), vec![vec![0x5550]]);
        assert_eq!(e.mem.u32(map.addr()), MAP_BASE_VTABLE);
        e.mem.set_u32(map.addr() + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x0058_f960, &args![map]);
        assert!(calls(&e, FREE).is_empty());
    }
}
