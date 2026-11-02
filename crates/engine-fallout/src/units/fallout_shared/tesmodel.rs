//! `fallout shared/tesmodel.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_004890e0` calls on the member (`00489620`).
const CALLEE_00489620: u32 = 0x0048_9620;

// Translated from 004890e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Named `TESModel::ClearDataComponent` (Xbox PDB name in the engine map). Thin wrapper: calls `00489620` with `ECX` = `this + 0xc` and passes on its `bool` result (`AL`).
pub fn fn_004890e0(e: &mut Engine, this: Ptr) -> bool {
    e.call(CALLEE_00489620, &args![this.byte_add(0xc)]).bool()
}

/// Record tags (`u32::from_le_bytes` of the four characters) `TESModel::LoadModelChunk` tells apart.
const MODL: u32 = u32::from_le_bytes(*b"MODL");
const PNAM: u32 = u32::from_le_bytes(*b"PNAM");
const MODD: u32 = u32::from_le_bytes(*b"MODD");
const MOSD: u32 = u32::from_le_bytes(*b"MOSD");
const MODT: u32 = u32::from_le_bytes(*b"MODT");

// Translated from 004892d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004892d0 - the security-cookie check (00ec408c) at the end is not translated.
/// `TESModel::LoadModelChunk` (Xbox PDB): reads the file's current chunk into `model`; does nothing
/// when `file` or `model` is null. cdecl, two arguments.
///
/// - `MODL` and `PNAM`: reads the whole chunk into a buffer of the chunk's size
///   (`TESFile::GetChunkData`, `00472890`, with limit 0) and runs virtual slot `+0x18` of
///   `model` on it.
/// - `MODD` and `MOSD`: reads one byte (`00472890` with 1) and stores it through `004893c0`.
/// - `MODT`: runs `TESModel::LoadModelTextureChunk` (`004893e0`) on `model` and `file`.
///
/// The buffer the game takes from the stack (`__alloca_probe_16`) is a heap block here.
pub fn tesmodel_load_model_chunk(e: &mut Engine, model: Ptr, file: Ptr) {
    if file.addr() == 0 || model.addr() == 0 {
        return;
    }
    // TESFile::GetTESChunk
    let chunk = e.call(0x0047_26b0, &args![file]).u32();
    match chunk {
        MODL | PNAM => {
            // The chunk's size: 00401660.
            let size = e.call(0x0040_1660, &args![file]).u32();
            let buffer = e.mem.alloc(size);
            e.call(0x0047_2890, &args![file, buffer, 0u32]);
            e.vcall(model.addr(), 0x18, &args![buffer]);
            e.mem.free(buffer);
        }
        MODD | MOSD => {
            let byte = e.mem.alloc(1);
            e.call(0x0047_2890, &args![file, byte, 1u32]);
            let value = e.mem.u8(byte);
            e.call(0x0048_93c0, &args![model, u32::from(value)]);
            e.mem.free(byte);
        }
        MODT => {
            e.call(0x0048_93e0, &args![model, file]);
        }
        _ => {}
    }
}

layout! {
    /// `TESModel` (Xbox PDB): a form component; the PC code uses the same
    /// offsets.
    pub struct TESModel: 0x18 {
        /// The vtable.
        0x00 vtable: u32,
        /// `cModel` (Xbox PDB): a `BSStringT<char>`.
        0x04 cModel: Inline<crate::types::BSStringT>,
        /// `TextureList` (Xbox PDB).
        0x0C TextureList: Inline<TESTextureList>,
        /// `cFlags` (Xbox PDB).
        0x14 cFlags: u8,
    }

    /// `TESTextureList` (Xbox PDB).
    pub struct TESTextureList: 0x08 {
        /// `cTextureCount` (Xbox PDB).
        0x00 cTextureCount: u8,
        /// `pTextureOffsetArray` (Xbox PDB): `BSFileEntry**`.
        0x04 pTextureOffsetArray: u32,
    }
}

/// `TESModel`'s vtable (RTTI `.?AVTESModel@@`).
const TES_MODEL_VTABLE: u32 = 0x0101_cf44;
/// `__RTDynamicCast(object, offset, source, target, is_reference)` and the
/// type descriptors the casts here name.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TES_MODEL_TYPE: u32 = 0x0118_31e8;
const TES_FORM_TYPE: u32 = 0x0118_3028;
const BASE_FORM_COMPONENT_TYPE: u32 = 0x0118_3040;
const TES_OBJECT_REFR_TYPE: u32 = 0x0118_41cc;
/// `TESBipedModelForm::GetWorldTESModel` (engine map; cdecl(form,
/// reference)).
const GET_WORLD_TES_MODEL: u32 = 0x0048_1010;
/// The first word of an object (`00559450`, `thiscall`): a `BSStringT`'s
/// text.
const FIRST_WORD: u32 = 0x0055_9450;
/// `BSStringT` members: set (`004037f0(text, length)`), destructor
/// (`004037d0`).
const STRING_SET: u32 = 0x0040_37f0;
const STRING_DESTRUCT: u32 = 0x0040_37d0;
/// `TESTextureList`'s clear (`00488ff0`, `run_00488f10.rs`).
const TEXTURE_LIST_CLEAR: u32 = 0x0048_8ff0;
/// `strcpy_s(dest, size, source)` (wrapper `00406d30`), `strrchr` (wrapper
/// `0040ab30`), `_stricmp` (wrapper `00404dc0`).
const STRCPY_S: u32 = 0x0040_6d30;
const STRRCHR: u32 = 0x0040_ab30;
const STRICMP: u32 = 0x0040_4dc0;
/// `"_far.NIF"`.
const FAR_SUFFIX: u32 = 0x0101_cf64;
/// The empty string `TESModel::GetModel` answers without a model.
const EMPTY_STRING: u32 = 0x0101_1584;
/// The length of the string at `this + 4` (`0048cee0`).
const MODEL_LENGTH: u32 = 0x0048_cee0;
/// `operator new(size)`, `operator delete(ptr)` (cdecl).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `TESFile` members: the chunk's size (`00401660`), the chunk's data
/// (`00472890(buffer, size)`).
const CHUNK_SIZE: u32 = 0x0040_1660;
const CHUNK_DATA: u32 = 0x0047_2890;
/// Fills bytes (`00403d30(dest, value, count)`).
const FILL_BYTES: u32 = 0x0040_3d30;
/// The two tables `004898f0` picks an entry from, a word per index.
const PRIMARY_TABLE: u32 = 0x011f_8190;
const FALLBACK_TABLE: u32 = 0x011f_81b4;
/// The bit test `008c1be0` (`*this & mask`).
const MASK_TEST: u32 = 0x008c_1be0;
/// The settings whose values `00408d60` returns (a pointer to the value
/// byte; null settings give a cleared static byte).
const SETTING_VALUE: u32 = 0x0040_8d60;
const LOAD_SETTING: u32 = 0x011c_5574;
const WARN_SETTING: u32 = 0x011c_555c;
/// Answers the byte at `011acdf8` (`004898e0`).
const WARN_FLAG: u32 = 0x0048_98e0;
/// Finds the archive file entry for a texture (`00af6540(1, &second,
/// &first, 0)`).
const FIND_FILE_ENTRY: u32 = 0x00af_6540;
/// The form-id getter (`0084e3a0`, the word at +0xc) and the log call
/// (`005b5e40`, cdecl with a format; does nothing in this build).
const FORM_ID: u32 = 0x0084_e3a0;
const LOG: u32 = 0x005b_5e40;
/// The three "MASTERFILE: Failed to find archive file entry ..." formats
/// (with form, with model only, unknown model).
const MISSING_WITH_FORM: u32 = 0x0101_d020;
const MISSING_WITH_MODEL: u32 = 0x0101_cfc0;
const MISSING_UNKNOWN: u32 = 0x0101_cf70;

/// `__RTDynamicCast(object, 0, source, target, 0)`.
fn dynamic_cast(e: &mut Engine, object: u32, source: u32, target: u32) -> u32 {
    e.call(RT_DYNAMIC_CAST, &args![object, 0u32, source, target, 0u32])
        .u32()
}

/// A model's name: its vtable slot `+0x14`.
fn model_name(e: &mut Engine, model: u32) -> u32 {
    e.vcall(model, 0x14, &[]).u32()
}

// Translated from 00489040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESModel::InitializeDataComponent` (Xbox PDB): empties `cModel` when it
/// has text (`004037f0(null, 0)`) and clears `cFlags`.
pub fn tes_model_initialize_data_component(e: &mut Engine, this: Ptr<TESModel>) {
    let name = this.addr() + 4;
    if e.call(FIRST_WORD, &args![name]).u32() != 0 {
        e.call(STRING_SET, &args![name, 0u32, 0u32]);
    }
    e.set(this, TESModel::cFlags, 0);
}

// Translated from 00489070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESModel::~TESModel` (the class by its vtable): the vtable,
/// `TESModel::ClearDataComponent` (`004890e0`), the texture list's clear
/// (`00488ff0`) and `cModel`'s destructor.
// Translation incomplete: 00489070 - the FS:[0] exception frame (destroys the members if a step throws) is not translated
pub fn tes_model_destruct(e: &mut Engine, this: Ptr<TESModel>) {
    e.set(this, TESModel::vtable, TES_MODEL_VTABLE);
    fn_004890e0(e, this.cast());
    e.call(TEXTURE_LIST_CLEAR, &args![this.addr() + 0xc]);
    e.call(STRING_DESTRUCT, &args![this.addr() + 4]);
}

// Translated from 00489120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESModel::GetDistantModelName` (Xbox PDB): copies the model's name
/// (vtable slot `+0x14`) into the 0x104-byte `buffer` and, when it has a
/// `.`, replaces the last one and what follows with `"_far.NIF"`.
pub fn tes_model_get_distant_model_name(e: &mut Engine, this: Ptr<TESModel>, buffer: u32) {
    let name = model_name(e, this.addr());
    e.call(STRCPY_S, &args![buffer, 0x104u32, name]);
    let dot = e.call(STRRCHR, &args![buffer, u32::from(b'.')]).u32();
    if dot != 0 {
        let room = 0x104u32.wrapping_sub(dot.wrapping_sub(buffer));
        e.call(STRCPY_S, &args![dot, room, FAR_SUFFIX]);
    }
}

// Translated from 00489190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESModel::GetDistantModelName` for a form (Xbox PDB `_ov2`, cdecl):
/// when the form is a `TESModel` (`dynamic_cast`), its distant model name
/// into `buffer`.
pub fn tes_model_get_distant_model_name_ov2(e: &mut Engine, buffer: u32, form: u32) {
    if form != 0 {
        let model = dynamic_cast(e, form, TES_FORM_TYPE, TES_MODEL_TYPE);
        if model != 0 {
            tes_model_get_distant_model_name(e, Ptr::new(model), buffer);
        }
    }
}

// Translated from 00489280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESModel::IsModelChunk` (Xbox PDB, cdecl): whether `tag` is `MODL`,
/// `MODD`, `MOSD` or `MODT` (the compiler's signed split around `MODL`).
pub fn tes_model_is_model_chunk(_e: &mut Engine, tag: u32) -> bool {
    if (tag as i32) > (MODL as i32) {
        tag == MODT
    } else {
        tag == MODL || tag == MODD || tag == MOSD
    }
}

// Translated from 004893c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `cFlags`.
pub fn fn_004893c0(e: &mut Engine, this: Ptr<TESModel>, flags: u8) {
    e.set(this, TESModel::cFlags, flags);
}

// Translated from 004893e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESModel::LoadModelTextureChunk` (Xbox PDB, cdecl): with a model and a
/// file, loads the texture list (`TESTextureList::Load`) from the file for
/// the model's form (`dynamic_cast` to `TESForm`) and name.
pub fn tes_model_load_model_texture_chunk(e: &mut Engine, model: u32, file: u32) {
    if model == 0 || file == 0 {
        return;
    }
    let name = model_name(e, model);
    let form = dynamic_cast(e, model, TES_MODEL_TYPE, TES_FORM_TYPE);
    tes_texture_list_load(e, Ptr::new(model + 0xc), file, form, name);
}

// Translated from 00489430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESModel::CopyComponent` (Xbox PDB): when `other` is a `TESModel`,
/// sets this model's name (vtable slot `+0x18`) to its name.
pub fn tes_model_copy_component(e: &mut Engine, this: Ptr<TESModel>, other: u32) {
    let model = dynamic_cast(e, other, BASE_FORM_COMPONENT_TYPE, TES_MODEL_TYPE);
    if model == 0 {
        return;
    }
    let name = model_name(e, model);
    e.vcall(this.addr(), 0x18, &args![name]);
}

// Translated from 00489480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESModel::CompareComponent` (Xbox PDB): true when the two differ: when
/// `other` is no `TESModel`; when both names exist and differ ignoring case
/// (`_stricmp`); when only one name exists and it is not empty.
pub fn tes_model_compare_component(e: &mut Engine, this: Ptr<TESModel>, other: u32) -> bool {
    let model = dynamic_cast(e, other, BASE_FORM_COMPONENT_TYPE, TES_MODEL_TYPE);
    if model == 0 {
        return true;
    }
    let mine = model_name(e, this.addr());
    let theirs = model_name(e, model);
    if mine != 0 && theirs != 0 {
        return e.call(STRICMP, &args![mine, theirs]).u32() != 0;
    }
    // Reached with one name missing; the both-present test is the exe's.
    if mine != 0 && theirs != 0 {
        return false;
    }
    if mine != 0 && e.mem.u8(mine) != 0 {
        return true;
    }
    if theirs != 0 && e.mem.u8(theirs) != 0 {
        return true;
    }
    false
}

/// The model of a form: the form itself as a `TESModel`, else
/// `TESBipedModelForm::GetWorldTESModel(form, form as TESObjectREFR)`.
fn model_of(e: &mut Engine, form: u32) -> u32 {
    let model = dynamic_cast(e, form, TES_FORM_TYPE, TES_MODEL_TYPE);
    if model != 0 {
        return model;
    }
    let reference = dynamic_cast(e, form, TES_FORM_TYPE, TES_OBJECT_REFR_TYPE);
    e.call(GET_WORLD_TES_MODEL, &args![form, reference]).u32()
}

// Translated from 00489540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESModel::GetModelLength` (Xbox PDB, cdecl): the length of the form's
/// model name (`0048cee0`), 0 without a model.
pub fn tes_model_get_model_length(e: &mut Engine, form: u32) -> u32 {
    let model = model_of(e, form);
    if model != 0 {
        e.call(MODEL_LENGTH, &args![model]).u32()
    } else {
        0
    }
}

// Translated from 004895b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESModel::GetModel` (Xbox PDB, cdecl): the form's model name, the
/// empty string without a model.
pub fn tes_model_get_model(e: &mut Engine, form: u32) -> u32 {
    let model = model_of(e, form);
    if model != 0 {
        model_name(e, model)
    } else {
        EMPTY_STRING
    }
}

// Translated from 00489620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESTextureList`'s clear: frees `pTextureOffsetArray` when there is
/// one (answering true), and clears the count.
pub fn fn_00489620(e: &mut Engine, this: Ptr<TESTextureList>) -> bool {
    let mut freed = false;
    if e.get(this, TESTextureList::pTextureOffsetArray) != 0 {
        let array = e.get(this, TESTextureList::pTextureOffsetArray);
        e.call(0x0040_1030, &args![array]);
        e.set(this, TESTextureList::pTextureOffsetArray, 0);
        freed = true;
    }
    e.set(this, TESTextureList::cTextureCount, 0);
    freed
}

// Translated from 00489670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESTextureList::LoadFromBuffer` (Xbox PDB): only when `004898f0(1)`
/// gives an object whose `+0x160` part passes `004898a0` and the setting
/// at `011c5574` is on: keeps `count`, allocates the entry array (`count *
/// 4`, all ones on overflow) and, for each 0x18-byte record of `buffer`,
/// builds two 8-byte keys (`00489860`, from its bytes 0..3 and word 4, and
/// bytes 0x10..0x13 and word 0x14) and stores the archive file entry
/// `00af6540(1, &second, &first, 0)`. A missing entry is logged
/// (`005b5e40`, a no-op in this build) when the setting at `011c555c` and
/// the flag `004898e0` are on: with the form and model name, the model
/// name only, or neither.
pub fn tes_texture_list_load_from_buffer(
    e: &mut Engine,
    this: Ptr<TESTextureList>,
    buffer: u32,
    count: u32,
    form: u32,
    name: u32,
) {
    let source = fn_004898f0(e, 1);
    if source == 0 || !fn_004898a0(e, Ptr::new(source + 0x160)) {
        return;
    }
    let setting = e.call(SETTING_VALUE, &args![LOAD_SETTING]).u32();
    if e.mem.u8(setting) == 0 {
        return;
    }
    e.set(this, TESTextureList::cTextureCount, count as u8);
    // `MUL` by 4; `SETO`/`NEG`/`OR` make an overflowing size all ones.
    let bytes = count.checked_mul(4).unwrap_or(u32::MAX);
    let array = e.call(OPERATOR_NEW, &args![bytes]).u32();
    e.set(this, TESTextureList::pTextureOffsetArray, array);
    let mut index = 0;
    while index < count {
        let record = buffer.wrapping_add(index.wrapping_mul(0x18));
        let entry = e.with_stack(0x10, |e, keys| {
            let first = keys.addr();
            let second = keys.addr() + 8;
            let word = e.mem.u32(record + 4);
            let bytes = [0, 1, 2, 3].map(|i| e.mem.u8(record + i));
            fn_00489860(
                e,
                Ptr::new(first),
                bytes[0],
                bytes[1],
                bytes[2],
                bytes[3],
                word,
            );
            let word = e.mem.u32(record + 0x14);
            let bytes = [0x10, 0x11, 0x12, 0x13].map(|i| e.mem.u8(record + i));
            fn_00489860(
                e,
                Ptr::new(second),
                bytes[0],
                bytes[1],
                bytes[2],
                bytes[3],
                word,
            );
            e.call(FIND_FILE_ENTRY, &args![1u32, second, first, 0u32])
                .u32()
        });
        let array = e.get(this, TESTextureList::pTextureOffsetArray);
        e.mem
            .set_u32(array.wrapping_add(index.wrapping_mul(4)), entry);
        let warn = e.call(SETTING_VALUE, &args![WARN_SETTING]).u32();
        if e.mem.u8(warn) != 0 && e.call(WARN_FLAG, &[]).bool() {
            let array = e.get(this, TESTextureList::pTextureOffsetArray);
            if e.mem.u32(array.wrapping_add(index.wrapping_mul(4))) == 0 {
                if form != 0 && name != 0 {
                    let id = e.call(FORM_ID, &args![form]).u32();
                    let form_name = e.vcall(form, 0x130, &[]).u32();
                    e.call(LOG, &args![MISSING_WITH_FORM, name, form_name, id]);
                } else if name != 0 {
                    e.call(LOG, &args![MISSING_WITH_MODEL, name]);
                } else {
                    e.call(LOG, &args![MISSING_UNKNOWN]);
                }
            }
        }
        index += 1;
    }
}

// Translated from 00489860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of an 8-byte key (class unconfirmed): four bytes, then a
/// word. Returns `this`.
pub fn fn_00489860(
    e: &mut Engine,
    this: Ptr,
    first: u8,
    second: u8,
    third: u8,
    fourth: u8,
    word: u32,
) -> Ptr {
    e.mem.set_u8(this.addr(), first);
    e.mem.set_u8(this.addr() + 1, second);
    e.mem.set_u8(this.addr() + 2, third);
    e.mem.set_u8(this.addr() + 3, fourth);
    e.mem.set_u32(this.addr() + 4, word);
    this
}

// Translated from 004898a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 8 (0x100) is set in the word at `this + 0xc`
/// (`004898c0`).
pub fn fn_004898a0(e: &mut Engine, this: Ptr) -> bool {
    fn_004898c0(e, this, 0x100) != 0
}

// Translated from 004898c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `this + 0xc` masked with `mask` (`008c1be0`).
pub fn fn_004898c0(e: &mut Engine, this: Ptr, mask: u32) -> u32 {
    e.call(MASK_TEST, &args![this.addr() + 0xc, mask]).u32()
}

// Translated from 004898f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word for `index` (cdecl): from the table at `011f8190` when it is
/// set there, else from the one at `011f81b4`.
pub fn fn_004898f0(e: &mut Engine, index: u32) -> u32 {
    let primary = PRIMARY_TABLE.wrapping_add(index.wrapping_mul(4));
    let slot = if e.call(FIRST_WORD, &args![primary]).u32() != 0 {
        PRIMARY_TABLE.wrapping_add(index.wrapping_mul(4))
    } else {
        FALLBACK_TABLE.wrapping_add(index.wrapping_mul(4))
    };
    e.call(FIRST_WORD, &args![slot]).u32()
}

// Translated from 00489940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESTextureList::Load` (Xbox PDB): reads the file's current chunk, when
/// its size is a whole number of 0x18-byte records and not empty, into a
/// cleared buffer and loads the list from it (`LoadFromBuffer`). The
/// buffer the game takes from the stack (`_alloca`) is a heap block here.
// Translation incomplete: 00489940 - the security-cookie check (00ec408c) at the end is not translated
pub fn tes_texture_list_load(
    e: &mut Engine,
    this: Ptr<TESTextureList>,
    file: u32,
    form: u32,
    name: u32,
) {
    let size = e.call(CHUNK_SIZE, &args![file]).u32();
    if size % 0x18 != 0 {
        return;
    }
    let count = size / 0x18;
    let bytes = count * 0x18;
    if count == 0 {
        return;
    }
    let buffer = e.mem.alloc(bytes);
    e.call(FILL_BYTES, &args![buffer, 0u32, bytes]);
    e.call(CHUNK_DATA, &args![file, buffer, bytes]);
    tes_texture_list_load_from_buffer(e, this, buffer, count, form, name);
    e.mem.free(buffer);
}

/// `TESForm::AddChunkArray` (`004856f0`, cdecl(chunk, data, size)).
const ADD_CHUNK_ARRAY: u32 = 0x0048_56f0;
/// `004858f0` (cdecl(chunk, byte)): adds a one-byte chunk.
const ADD_CHUNK_BYTE: u32 = 0x0048_58f0;
/// `008d0600` (thiscall, `RET 8`): the texture list's save step; its body
/// does nothing in this build.
const TEXTURE_LIST_SAVE: u32 = 0x008d_0600;
/// `00843150` (thiscall): the byte at `this + 0x14` (`cFlags`).
const FLAGS_BYTE: u32 = 0x0084_3150;
/// The byte `004898e0` answers.
const WARN_FLAG_BYTE: u32 = 0x011a_cdf8;

// Translated from 004891d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESModel::Save` (Xbox PDB, `RET 8`): only when the model name is not
/// empty (`0048cee0`): with a `model_chunk`, adds the name with its
/// terminator as that chunk (`TESForm::AddChunkArray(model_chunk, name,
/// length + 1)`); with a `texture_chunk`, runs `008d0600` on the texture
/// list (`this + 0xc`) with `(texture_chunk, name)`. The name is the model's
/// vtable slot `+0x14`, asked again for each use.
pub fn tes_model_save(e: &mut Engine, this: Ptr<TESModel>, model_chunk: u32, texture_chunk: u32) {
    if e.call(MODEL_LENGTH, &args![this]).u32() == 0 {
        return;
    }
    if model_chunk != 0 {
        let size = e.call(MODEL_LENGTH, &args![this]).u32().wrapping_add(1);
        let name = model_name(e, this.addr());
        e.call(ADD_CHUNK_ARRAY, &args![model_chunk, name, size]);
    }
    if texture_chunk != 0 {
        let name = model_name(e, this.addr());
        e.call(
            TEXTURE_LIST_SAVE,
            &args![this.addr() + 0xc, texture_chunk, name],
        );
    }
}

// Translated from 00489240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESModel::SaveFlags` (Xbox PDB, `RET 4`): when the flags byte
/// (`00843150`) is not zero, adds it as `chunk` (`004858f0(chunk, flags)`).
pub fn tes_model_save_flags(e: &mut Engine, this: Ptr<TESModel>, chunk: u32) {
    let flags = e.call(FLAGS_BYTE, &args![this]).u32() as u8;
    if flags != 0 {
        e.call(ADD_CHUNK_BYTE, &args![chunk, u32::from(flags)]);
    }
}

// Translated from 004898e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `011acdf8` (the flag `TESTextureList::LoadFromBuffer` asks
/// before logging a missing texture).
pub fn fn_004898e0(e: &mut Engine) -> u8 {
    e.global::<u8>(WARN_FLAG_BYTE)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0048_90e0, fn_004890e0(Ptr) -> bool),
        entry!(0x0048_92d0, tesmodel_load_model_chunk(Ptr, Ptr)),
        entry!(
            0x00489040,
            tes_model_initialize_data_component(Ptr<TESModel>)
        ),
        entry!(0x00489070, tes_model_destruct(Ptr<TESModel>)),
        entry!(
            0x00489120,
            tes_model_get_distant_model_name(Ptr<TESModel>, u32)
        ),
        entry!(0x00489190, tes_model_get_distant_model_name_ov2(u32, u32)),
        entry!(0x00489280, tes_model_is_model_chunk(u32) -> bool),
        entry!(0x004893c0, fn_004893c0(Ptr<TESModel>, u8)),
        entry!(0x004893e0, tes_model_load_model_texture_chunk(u32, u32)),
        entry!(0x00489430, tes_model_copy_component(Ptr<TESModel>, u32)),
        entry!(
            0x00489480,
            tes_model_compare_component(Ptr<TESModel>, u32) -> bool
        ),
        entry!(0x00489540, tes_model_get_model_length(u32) -> u32),
        entry!(0x004895b0, tes_model_get_model(u32) -> u32),
        entry!(0x00489620, fn_00489620(Ptr<TESTextureList>) -> bool),
        entry!(
            0x00489670,
            tes_texture_list_load_from_buffer(Ptr<TESTextureList>, u32, u32, u32, u32)
        ),
        entry!(
            0x00489860,
            fn_00489860(Ptr, u8, u8, u8, u8, u32) -> Ptr
        ),
        entry!(0x004898a0, fn_004898a0(Ptr) -> bool),
        entry!(0x004898c0, fn_004898c0(Ptr, u32) -> u32),
        entry!(0x004898f0, fn_004898f0(u32) -> u32),
        entry!(
            0x00489940,
            tes_texture_list_load(Ptr<TESTextureList>, u32, u32, u32)
        ),
        entry!(0x0048_91d0, tes_model_save(Ptr<TESModel>, u32, u32)),
        entry!(0x0048_9240, tes_model_save_flags(Ptr<TESModel>, u32)),
        entry!(0x0048_98e0, fn_004898e0() -> u8),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_004890e0_passes_this_plus_c_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_00489620, |_, a| Ret {
            eax: a[0] & 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0048_90e0, &args![0x4100u32]).bool());
        assert!(e.call(0x0048_90e0, &args![0x4100u32 + 1]).bool());
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_00489620).collect();
        assert_eq!(inner.len(), 2);
        assert_eq!(inner[0].1, vec![0x4100 + 0xc]);
    }
}

/// Test support shared by the record loaders of this work package: `TESFile` doubles that walk a
/// list of chunks.
#[cfg(test)]
pub(crate) mod load_test {
    use crate::prelude::*;
    use std::cell::Cell;
    use std::rc::Rc;

    /// The file object the tests pass (never dereferenced: every `TESFile` function is a double).
    pub const FILE: u32 = 0x0f11_e000;
    /// The size `00401660` reports for the current chunk.
    pub const CHUNK_SIZE: u32 = 0x20;
    /// The value `TESFile::GetChunkData` (`004727f0`) writes.
    pub const CHUNK_VALUE: u32 = 0x1234_5678;
    pub type Calls = Vec<(u32, Vec<u32>)>;

    /// Addresses of the `TESFile` callees every loader runs; [`run`] leaves them out of the log.
    const FILE_CALLS: [u32; 6] = [
        0x0047_2660,
        0x0047_26b0,
        0x0047_26f0,
        0x0040_1660,
        0x0040_1680,
        0x0040_3570,
    ];

    /// An engine whose file is positioned on the first of `chunks`. `form_type` is what
    /// `TESFile::GetTESForm` answers and `version_flag` what `00401680` answers. `00472890`
    /// fills the buffer with `0x5a` (at most `CHUNK_SIZE` bytes) and `004727f0` stores
    /// [`CHUNK_VALUE`].
    pub fn engine(form_type: u32, chunks: &[u32], version_flag: bool) -> Engine {
        let mut e = Engine::new();
        let position = Rc::new(Cell::new(0usize));
        let chunks = chunks.to_vec();
        e.register_double(0x0047_2660, move |_, _| Ret {
            eax: form_type,
            ..Ret::default()
        });
        let at = position.clone();
        e.register_double(0x0047_26b0, move |_, _| Ret {
            eax: chunks.get(at.get()).copied().unwrap_or(0),
            ..Ret::default()
        });
        e.register_double(0x0047_26f0, move |_, _| {
            position.set(position.get() + 1);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.register_double(0x0040_1660, |_, _| Ret {
            eax: CHUNK_SIZE,
            ..Ret::default()
        });
        e.register_double(0x0040_1680, move |_, _| Ret {
            eax: u32::from(version_flag),
            ..Ret::default()
        });
        e.register_double(0x0040_3570, |_, _| Ret {
            eax: 0xabcd_1234,
            ..Ret::default()
        });
        e.register_double(0x0047_2890, |e, a| {
            let (buffer, limit) = (a[1], a[2]);
            let count = if limit == 0 {
                CHUNK_SIZE
            } else {
                limit.min(CHUNK_SIZE)
            };
            e.mem.write(buffer, &vec![0x5a; count as usize]);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.register_double(0x0047_27f0, |e, a| {
            e.mem.set_u32(a[1], CHUNK_VALUE);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e
    }

    /// Registers do-nothing doubles for `callees`.
    pub fn stub(e: &mut Engine, callees: &[u32]) {
        for callee in callees {
            e.register(*callee, |_, _| Ret::default());
        }
    }

    /// Calls `addr` with `words` and returns the answer and the calls it made, without the
    /// `TESFile` queries of [`FILE_CALLS`].
    pub fn run(e: &mut Engine, addr: u32, words: &[u32]) -> (Ret, Calls) {
        e.call_log = Some(vec![]);
        let ret = e.call(addr, words);
        let log = e.call_log.take().unwrap();
        let calls = log
            .into_iter()
            .skip(1)
            .filter(|(callee, _)| !FILE_CALLS.contains(callee))
            .collect();
        (ret, calls)
    }

    /// The record tag of four characters.
    pub fn tag(text: &[u8; 4]) -> u32 {
        u32::from_le_bytes(*text)
    }

    /// A zeroed object of `size` bytes whose vtable, placed at `vtable`, has the given
    /// (byte offset, target) slots; every target is a do-nothing double.
    pub fn object(e: &mut Engine, size: u32, vtable: u32, slots: &[(u32, u32)]) -> u32 {
        let this = e.mem.alloc(size);
        e.map(vtable, 0x200);
        e.mem.set_u32(this, vtable);
        for (offset, target) in slots {
            e.mem.set_u32(vtable + offset, *target);
            e.register(*target, |_, _| Ret::default());
        }
        this
    }

    /// The calls to `addr`.
    pub fn calls_to(calls: &Calls, addr: u32) -> Vec<Vec<u32>> {
        calls
            .iter()
            .filter(|(callee, _)| *callee == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }
}

#[cfg(test)]
mod movement_path_tests {
    use super::load_test::{self, tag, FILE};
    use crate::prelude::*;

    const MODEL: u32 = 0x0800_0000;

    /// An engine with a model whose virtual slot `+0x18` is a double at `0x0090_0200`.
    fn setup(chunks: &[u32]) -> Engine {
        let mut e = load_test::engine(0, chunks, false);
        e.put_vtable(0x0090_1000, &[0, 0, 0, 0, 0, 0, 0x0090_0200]);
        e.map(MODEL, 0x40);
        e.mem.set_u32(MODEL, 0x0090_1000);
        load_test::stub(&mut e, &[0x0090_0200, 0x0048_93c0, 0x0048_93e0]);
        e
    }

    #[test]
    fn nothing_happens_without_a_model_or_a_file() {
        let mut e = setup(&[tag(b"MODL")]);
        let (_, calls) = load_test::run(&mut e, 0x0048_92d0, &[0, FILE]);
        assert!(calls.is_empty());
        let (_, calls) = load_test::run(&mut e, 0x0048_92d0, &[MODEL, 0]);
        assert!(calls.is_empty());
    }

    #[test]
    fn the_name_chunks_are_read_whole_and_given_to_the_model() {
        for chunk in [tag(b"MODL"), tag(b"PNAM")] {
            let mut e = setup(&[chunk]);
            let (_, calls) = load_test::run(&mut e, 0x0048_92d0, &[MODEL, FILE]);
            assert_eq!(calls.len(), 2);
            let read = &calls[0];
            assert_eq!((read.0, read.1[0], read.1[2]), (0x0047_2890, FILE, 0));
            let buffer = read.1[1];
            assert_eq!(calls[1], (0x0090_0200, vec![MODEL, buffer]));
            // The buffer is released again.
            assert_eq!(e.mem.block_size(buffer), None);
        }
    }

    #[test]
    fn the_flag_chunks_read_one_byte_and_store_it() {
        for chunk in [tag(b"MODD"), tag(b"MOSD")] {
            let mut e = setup(&[chunk]);
            let (_, calls) = load_test::run(&mut e, 0x0048_92d0, &[MODEL, FILE]);
            assert_eq!(calls.len(), 2);
            assert_eq!(
                (calls[0].0, calls[0].1[0], calls[0].1[2]),
                (0x0047_2890, FILE, 1)
            );
            assert_eq!(calls[1], (0x0048_93c0, vec![MODEL, 0x5a]));
        }
    }

    #[test]
    fn the_texture_chunk_goes_to_the_texture_loader_and_others_are_ignored() {
        let mut e = setup(&[tag(b"MODT")]);
        let (_, calls) = load_test::run(&mut e, 0x0048_92d0, &[MODEL, FILE]);
        assert_eq!(calls, [(0x0048_93e0, vec![MODEL, FILE])]);

        let mut e = setup(&[tag(b"EDID")]);
        let (_, calls) = load_test::run(&mut e, 0x0048_92d0, &[MODEL, FILE]);
        assert!(calls.is_empty());
        // Only the first chunk is read.
        let mut e = setup(&[tag(b"EDID"), tag(b"MODT")]);
        let (_, calls) = load_test::run(&mut e, 0x0048_92d0, &[MODEL, FILE]);
        assert!(calls.is_empty());
    }
}

#[cfg(test)]
mod model_component_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn no_op(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn run(e: &mut Engine, addr: u32, words: &[u32]) -> (Ret, Log) {
        e.call_log = Some(vec![]);
        let r = e.call(addr, words);
        (r, e.call_log.take().unwrap())
    }

    /// The cast double: answers `object` when `target` is `wanted`, else 0.
    fn cast_to(e: &mut Engine, wanted: u32) {
        e.register_double(RT_DYNAMIC_CAST, move |_, a| {
            (if a[3] == wanted { a[0] } else { 0 }).into_ret()
        });
    }

    /// A model whose vtable slot `+0x14` answers `name` and `+0x18` is a
    /// double.
    fn model(e: &mut Engine, name: u32) -> u32 {
        let vtable = e.mem.alloc(0x20);
        let getter = 0x0f40_0000 + (vtable & 0xffff);
        e.mem.set_u32(vtable + 0x14, getter);
        e.mem.set_u32(vtable + 0x18, 0x0f41_0018);
        e.register_double(getter, move |_, _| name.into_ret());
        e.register(0x0f41_0018, no_op);
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this, vtable);
        this
    }

    fn text(e: &mut Engine, s: &[u8]) -> u32 {
        let p = e.mem.alloc(s.len() as u32 + 1);
        e.mem.set_cstr(p, s);
        p
    }

    #[test]
    fn initialize_empties_a_set_name_and_clears_the_flags() {
        let mut e = Engine::new();
        e.register(STRING_SET, no_op);
        let this: Ptr<TESModel> = e.new_object();
        e.set(this, TESModel::cFlags, 7);
        e.register(FIRST_WORD, |_, _| 0x1234u32.into_ret());
        let (_, log) = run(&mut e, 0x0048_9040, &args![this]);
        assert_eq!(calls(&log, STRING_SET), vec![vec![this.addr() + 4, 0, 0]]);
        assert_eq!(e.get(this, TESModel::cFlags), 0);
        e.register(FIRST_WORD, |_, _| 0u32.into_ret());
        let (_, log) = run(&mut e, 0x0048_9040, &args![this]);
        assert!(calls(&log, STRING_SET).is_empty());
    }

    #[test]
    fn destructor_clears_then_destroys_the_members() {
        let mut e = Engine::new();
        for addr in [CALLEE_00489620, TEXTURE_LIST_CLEAR, STRING_DESTRUCT] {
            e.register(addr, no_op);
        }
        let this: Ptr<TESModel> = e.new_object();
        let (_, log) = run(&mut e, 0x0048_9070, &args![this]);
        assert_eq!(e.get(this, TESModel::vtable), TES_MODEL_VTABLE);
        let order: Log = log.into_iter().skip(1).collect();
        let a = this.addr();
        assert_eq!(
            order,
            vec![
                (CALLEE_00489620, vec![a + 0xc]),
                (TEXTURE_LIST_CLEAR, vec![a + 0xc]),
                (STRING_DESTRUCT, vec![a + 4]),
            ]
        );
    }

    #[test]
    fn distant_name_swaps_the_extension() {
        let mut e = Engine::new();
        let name = text(&mut e, b"meshes\\a.nif");
        let this = model(&mut e, name);
        let buffer = e.mem.alloc(0x104);
        e.register(STRCPY_S, |e, a| {
            let s = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &s);
            Ret::default()
        });
        e.register(STRRCHR, |e, a| {
            let s = e.mem.cstr(a[0]);
            let at = s.iter().rposition(|c| u32::from(*c) == a[1]);
            at.map_or(0, |i| a[0] + i as u32).into_ret()
        });
        e.map(FAR_SUFFIX & !0xfff, 0x1000);
        e.mem.set_cstr(FAR_SUFFIX, b"_far.NIF");
        let (_, log) = run(&mut e, 0x0048_9120, &args![this, buffer]);
        assert_eq!(e.mem.cstr(buffer), b"meshes\\a_far.NIF".to_vec());
        let copies = calls(&log, STRCPY_S);
        assert_eq!(copies[0], vec![buffer, 0x104, name]);
        assert_eq!(copies[1], vec![buffer + 8, 0x104 - 8, FAR_SUFFIX]);
        // No dot: only the copy.
        let plain = text(&mut e, b"plain");
        let this = model(&mut e, plain);
        let (_, log) = run(&mut e, 0x0048_9120, &args![this, buffer]);
        assert_eq!(calls(&log, STRCPY_S).len(), 1);
    }

    #[test]
    fn distant_name_of_a_form_needs_a_model() {
        let mut e = Engine::new();
        let name = text(&mut e, b"x");
        let this = model(&mut e, name);
        e.register(STRCPY_S, no_op);
        e.register(STRRCHR, |_, _| 0u32.into_ret());
        cast_to(&mut e, TES_MODEL_TYPE);
        let (_, log) = run(&mut e, 0x0048_9190, &args![0x500u32, this]);
        assert_eq!(
            calls(&log, RT_DYNAMIC_CAST),
            vec![vec![this, 0, TES_FORM_TYPE, TES_MODEL_TYPE, 0]]
        );
        assert_eq!(calls(&log, STRCPY_S), vec![vec![0x500, 0x104, name]]);
        // A null form: nothing.
        let (_, log) = run(&mut e, 0x0048_9190, &args![0x500u32, 0u32]);
        assert_eq!(log.len(), 1);
        // Not a model: no copy.
        cast_to(&mut e, 0);
        let (_, log) = run(&mut e, 0x0048_9190, &args![0x500u32, this]);
        assert!(calls(&log, STRCPY_S).is_empty());
    }

    #[test]
    fn model_chunks_are_the_four_tags() {
        let mut e = Engine::new();
        for (tag, yes) in [
            (MODL, true),
            (MODD, true),
            (MOSD, true),
            (MODT, true),
            (PNAM, false),
            (u32::from_le_bytes(*b"EDID"), false),
            (0xffff_ffff, false),
        ] {
            assert_eq!(e.call(0x0048_9280, &args![tag]).bool(), yes);
        }
    }

    #[test]
    fn set_flags_stores_the_byte() {
        let mut e = Engine::new();
        let this: Ptr<TESModel> = e.new_object();
        e.call(0x0048_93c0, &args![this, 0x1ffu32]);
        assert_eq!(e.get(this, TESModel::cFlags), 0xff);
    }

    #[test]
    fn texture_chunk_loads_the_list_for_the_form() {
        let mut e = Engine::new();
        let name = text(&mut e, b"n");
        let this = model(&mut e, name);
        cast_to(&mut e, TES_FORM_TYPE);
        // The chunk size is not a whole record: Load stops after asking.
        e.register(CHUNK_SIZE, |_, _| 5u32.into_ret());
        let (_, log) = run(&mut e, 0x0048_93e0, &args![this, 0x900u32]);
        assert_eq!(
            calls(&log, RT_DYNAMIC_CAST),
            vec![vec![this, 0, TES_MODEL_TYPE, TES_FORM_TYPE, 0]]
        );
        assert_eq!(calls(&log, CHUNK_SIZE), vec![vec![0x900]]);
        for (m, f) in [(0u32, 0x900u32), (this, 0)] {
            let (_, log) = run(&mut e, 0x0048_93e0, &args![m, f]);
            assert_eq!(log.len(), 1);
        }
    }

    #[test]
    fn copy_component_takes_the_other_name() {
        let mut e = Engine::new();
        let name = text(&mut e, b"other");
        let other = model(&mut e, name);
        let this = model(&mut e, 0);
        cast_to(&mut e, TES_MODEL_TYPE);
        let (_, log) = run(&mut e, 0x0048_9430, &args![this, other]);
        assert_eq!(
            calls(&log, RT_DYNAMIC_CAST)[0],
            vec![other, 0, BASE_FORM_COMPONENT_TYPE, TES_MODEL_TYPE, 0]
        );
        assert_eq!(calls(&log, 0x0f41_0018), vec![vec![this, name]]);
        cast_to(&mut e, 0);
        let (_, log) = run(&mut e, 0x0048_9430, &args![this, other]);
        assert!(calls(&log, 0x0f41_0018).is_empty());
    }

    #[test]
    fn compare_component_reports_differences() {
        let mut e = Engine::new();
        e.register(STRICMP, |e, a| {
            let l = e.mem.cstr(a[0]).to_ascii_lowercase();
            let r = e.mem.cstr(a[1]).to_ascii_lowercase();
            u32::from(l != r).into_ret()
        });
        let a = text(&mut e, b"Mesh");
        let b = text(&mut e, b"mesh");
        let c = text(&mut e, b"other");
        let empty = text(&mut e, b"");
        cast_to(&mut e, TES_MODEL_TYPE);
        for (mine, theirs, differs) in [
            (a, b, false),
            (a, c, true),
            (a, 0, true),
            (0, c, true),
            (empty, 0, false),
            (0, empty, false),
            (0, 0, false),
        ] {
            let this = model(&mut e, mine);
            let other = model(&mut e, theirs);
            assert_eq!(
                e.call(0x0048_9480, &args![this, other]).bool(),
                differs,
                "{mine:x} {theirs:x}"
            );
        }
        // Not a model: different.
        cast_to(&mut e, 0);
        let this = model(&mut e, a);
        assert!(e.call(0x0048_9480, &args![this, this]).bool());
    }

    #[test]
    fn model_length_and_name_fall_back_to_the_world_model() {
        let mut e = Engine::new();
        let name = text(&mut e, b"abc");
        let direct = model(&mut e, name);
        e.register(MODEL_LENGTH, |_, a| (a[0] & 0xff).into_ret());
        // The form is a model itself.
        cast_to(&mut e, TES_MODEL_TYPE);
        assert_eq!(e.call(0x0048_9540, &args![direct]).u32(), direct & 0xff);
        assert_eq!(e.call(0x0048_95b0, &args![direct]).u32(), name);
        // Not a model: the world model of the form as a reference.
        let world = model(&mut e, name);
        cast_to(&mut e, TES_OBJECT_REFR_TYPE);
        e.register_double(GET_WORLD_TES_MODEL, move |_, _| world.into_ret());
        let (r, log) = run(&mut e, 0x0048_9540, &args![0x700u32]);
        assert_eq!(r.u32(), world & 0xff);
        assert_eq!(calls(&log, GET_WORLD_TES_MODEL), vec![vec![0x700, 0x700]]);
        // Neither: 0 and the empty string.
        e.register(GET_WORLD_TES_MODEL, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x0048_9540, &args![0x700u32]).u32(), 0);
        assert_eq!(e.call(0x0048_95b0, &args![0x700u32]).u32(), EMPTY_STRING);
    }

    #[test]
    fn texture_list_clear_frees_the_array() {
        let mut e = Engine::new();
        e.register(0x0040_1030, no_op);
        let list: Ptr<TESTextureList> = e.new_object();
        e.set(list, TESTextureList::cTextureCount, 3);
        e.set(list, TESTextureList::pTextureOffsetArray, 0x4440);
        let (r, log) = run(&mut e, 0x0048_9620, &args![list]);
        assert!(r.bool());
        assert_eq!(calls(&log, 0x0040_1030), vec![vec![0x4440]]);
        assert_eq!(e.get(list, TESTextureList::pTextureOffsetArray), 0);
        assert_eq!(e.get(list, TESTextureList::cTextureCount), 0);
        let (r, log) = run(&mut e, 0x0048_9620, &args![list]);
        assert!(!r.bool());
        assert!(calls(&log, 0x0040_1030).is_empty());
    }

    /// Doubles for `LoadFromBuffer`'s gates: the source object (with the
    /// 0x100 bit), both settings and the warning flag as given.
    fn gates(e: &mut Engine, source_ok: bool, load: u8, warn: u8, flag: bool) {
        e.map(PRIMARY_TABLE & !0xfff, 0x1000);
        let source = e.mem.alloc(0x170);
        e.mem.set_u32(PRIMARY_TABLE + 4, source);
        e.register(FIRST_WORD, |e, a| e.mem.u32(a[0]).into_ret());
        e.register_double(MASK_TEST, move |_, a| {
            (if source_ok { a[1] } else { 0 }).into_ret()
        });
        let values = e.mem.alloc(2);
        e.mem.set_u8(values, load);
        e.mem.set_u8(values + 1, warn);
        e.register_double(SETTING_VALUE, move |_, a| {
            (if a[0] == LOAD_SETTING {
                values
            } else {
                values + 1
            })
            .into_ret()
        });
        e.register_double(WARN_FLAG, move |_, _| flag.into_ret());
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(LOG, no_op);
        e.register(FORM_ID, |_, _| 0x00ab_cdefu32.into_ret());
    }

    /// Two records: the first's entry is found, the second's is not.
    fn records(e: &mut Engine) -> u32 {
        let buffer = e.mem.alloc(0x30);
        for (i, b) in [1u8, 2, 3, 4].iter().enumerate() {
            e.mem.set_u8(buffer + i as u32, *b);
            e.mem.set_u8(buffer + 0x10 + i as u32, *b + 0x10);
        }
        e.mem.set_u32(buffer + 4, 0x1111);
        e.mem.set_u32(buffer + 0x14, 0x2222);
        e.register(FIND_FILE_ENTRY, |e, a| {
            // Found only for the first record (word 0x1111 in the first key).
            (if e.mem.u32(a[2] + 4) == 0x1111 {
                0x7700u32
            } else {
                0
            })
            .into_ret()
        });
        buffer
    }

    #[test]
    fn load_from_buffer_finds_each_entry_and_logs_misses() {
        let mut e = Engine::new();
        gates(&mut e, true, 1, 1, true);
        let buffer = records(&mut e);
        let name = text(&mut e, b"model.nif");
        let vtable = e.mem.alloc(0x134);
        e.mem.set_u32(vtable + 0x130, 0x0f42_0130);
        e.register(0x0f42_0130, |_, _| 0x0f43_0000u32.into_ret());
        let form = e.mem.alloc(0x18);
        e.mem.set_u32(form, vtable);
        let list: Ptr<TESTextureList> = e.new_object();
        let (_, log) = run(&mut e, 0x0048_9670, &args![list, buffer, 2u32, form, name]);
        assert_eq!(e.get(list, TESTextureList::cTextureCount), 2);
        let array = e.get(list, TESTextureList::pTextureOffsetArray);
        assert_eq!(e.mem.u32(array), 0x7700);
        assert_eq!(e.mem.u32(array + 4), 0);
        assert_eq!(calls(&log, OPERATOR_NEW), vec![vec![8]]);
        let finds = calls(&log, FIND_FILE_ENTRY);
        assert_eq!(finds.len(), 2);
        assert_eq!((finds[0][0], finds[0][3]), (1, 0));
        assert_eq!(
            calls(&log, LOG),
            vec![vec![MISSING_WITH_FORM, name, 0x0f43_0000, 0x00ab_cdef]]
        );
        // Without a form, or a name, the shorter messages.
        let list: Ptr<TESTextureList> = e.new_object();
        let (_, log) = run(&mut e, 0x0048_9670, &args![list, buffer, 2u32, 0u32, name]);
        assert_eq!(calls(&log, LOG), vec![vec![MISSING_WITH_MODEL, name]]);
        let (_, log) = run(&mut e, 0x0048_9670, &args![list, buffer, 2u32, form, 0u32]);
        assert_eq!(calls(&log, LOG), vec![vec![MISSING_UNKNOWN]]);
    }

    #[test]
    fn load_from_buffer_keys_hold_the_record_bytes() {
        let mut e = Engine::new();
        gates(&mut e, true, 1, 0, false);
        let buffer = records(&mut e);
        e.register(FIND_FILE_ENTRY, |e, a| {
            // The second argument is the key from bytes 0x10..0x17, the
            // third the one from bytes 0..7.
            assert_eq!(e.mem.bytes(a[2], 8), vec![1, 2, 3, 4, 0x11, 0x11, 0, 0]);
            assert_eq!(
                e.mem.bytes(a[1], 8),
                vec![0x11, 0x12, 0x13, 0x14, 0x22, 0x22, 0, 0]
            );
            1u32.into_ret()
        });
        let list: Ptr<TESTextureList> = e.new_object();
        let (_, log) = run(&mut e, 0x0048_9670, &args![list, buffer, 1u32, 0u32, 0u32]);
        assert_eq!(calls(&log, FIND_FILE_ENTRY).len(), 1);
        assert!(calls(&log, LOG).is_empty());
    }

    #[test]
    fn load_from_buffer_needs_the_source_and_the_setting() {
        for (source_ok, load) in [(false, 1u8), (true, 0)] {
            let mut e = Engine::new();
            gates(&mut e, source_ok, load, 1, true);
            let buffer = records(&mut e);
            let list: Ptr<TESTextureList> = e.new_object();
            let (_, log) = run(&mut e, 0x0048_9670, &args![list, buffer, 2u32, 0u32, 0u32]);
            assert!(calls(&log, OPERATOR_NEW).is_empty());
            assert_eq!(e.get(list, TESTextureList::cTextureCount), 0);
        }
        // No source object at all.
        let mut e = Engine::new();
        gates(&mut e, true, 1, 1, true);
        e.mem.set_u32(PRIMARY_TABLE + 4, 0);
        e.mem.set_u32(FALLBACK_TABLE + 4, 0);
        let list: Ptr<TESTextureList> = e.new_object();
        let (_, log) = run(&mut e, 0x0048_9670, &args![list, 0u32, 2u32, 0u32, 0u32]);
        assert!(calls(&log, MASK_TEST).is_empty());
    }

    #[test]
    fn key_constructor_stores_four_bytes_and_a_word() {
        let mut e = Engine::new();
        let key = e.mem.alloc(8);
        let r = e.call(
            0x0048_9860,
            &args![key, 1u32, 2u32, 0x103u32, 4u32, 0xdeadu32],
        );
        assert_eq!(r.u32(), key);
        assert_eq!(e.mem.bytes(key, 8), vec![1, 2, 3, 4, 0xad, 0xde, 0, 0]);
    }

    #[test]
    fn bit_test_masks_the_word_at_plus_c() {
        let mut e = Engine::new();
        e.register(MASK_TEST, |e, a| (e.mem.u32(a[0]) & a[1]).into_ret());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, 0x0000_0101);
        assert_eq!(e.call(0x0048_98c0, &args![this, 0xffu32]).u32(), 1);
        assert!(e.call(0x0048_98a0, &args![this]).bool());
        e.mem.set_u32(this + 0xc, 0x0000_00ff);
        assert!(!e.call(0x0048_98a0, &args![this]).bool());
    }

    #[test]
    fn table_word_prefers_the_primary_table() {
        let mut e = Engine::new();
        e.register(FIRST_WORD, |e, a| e.mem.u32(a[0]).into_ret());
        e.map(PRIMARY_TABLE & !0xfff, 0x1000);
        e.mem.set_u32(FALLBACK_TABLE + 8, 0x22);
        assert_eq!(e.call(0x0048_98f0, &args![2u32]).u32(), 0x22);
        e.mem.set_u32(PRIMARY_TABLE + 8, 0x11);
        assert_eq!(e.call(0x0048_98f0, &args![2u32]).u32(), 0x11);
    }

    #[test]
    fn texture_list_load_reads_whole_records() {
        let mut e = Engine::new();
        gates(&mut e, true, 1, 0, false);
        e.register(CHUNK_SIZE, |_, _| 0x30u32.into_ret());
        e.register(FILL_BYTES, no_op);
        e.register(CHUNK_DATA, |e, a| {
            e.mem.set_u32(a[1] + 4, 0x1111);
            Ret::default()
        });
        e.register(FIND_FILE_ENTRY, |_, _| 5u32.into_ret());
        let list: Ptr<TESTextureList> = e.new_object();
        let (_, log) = run(&mut e, 0x0048_9940, &args![list, 0x900u32, 0u32, 0u32]);
        let fill = calls(&log, FILL_BYTES);
        assert_eq!(fill[0][1..], [0, 0x30]);
        assert_eq!(calls(&log, CHUNK_DATA), vec![vec![0x900, fill[0][0], 0x30]]);
        assert_eq!(e.get(list, TESTextureList::cTextureCount), 2);
        // Not a whole number of records, or empty: nothing is read.
        for size in [0x31u32, 0] {
            e.register_double(CHUNK_SIZE, move |_, _| size.into_ret());
            let (_, log) = run(&mut e, 0x0048_9940, &args![list, 0x900u32, 0u32, 0u32]);
            assert!(calls(&log, CHUNK_DATA).is_empty());
        }
    }
}

#[cfg(test)]
mod save_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn run(e: &mut Engine, addr: u32, words: &[u32]) -> (Ret, Log) {
        e.call_log = Some(vec![]);
        let r = e.call(addr, words);
        (r, e.call_log.take().unwrap())
    }

    /// A model whose name (vtable slot `+0x14`) is `0x5000` and whose length
    /// (`0048cee0`) is `length`.
    fn model(e: &mut Engine, length: u32) -> u32 {
        let this = e.mem.alloc(0x18);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(vtable + 0x14, 0x0f42_0014);
        e.register(0x0f42_0014, |_, _| 0x5000u32.into_ret());
        e.register_double(MODEL_LENGTH, move |_, _| length.into_ret());
        e.register(ADD_CHUNK_ARRAY, |_, _| Ret::default());
        e.register(TEXTURE_LIST_SAVE, |_, _| Ret::default());
        this
    }

    #[test]
    fn save_writes_the_name_and_the_texture_list() {
        let mut e = Engine::new();
        let this = model(&mut e, 7);
        let (_, log) = run(
            &mut e,
            0x0048_91d0,
            &args![this, 0x4c444f4du32, 0x5444_4f4du32],
        );
        assert_eq!(
            calls(&log, ADD_CHUNK_ARRAY),
            vec![vec![0x4c444f4d, 0x5000, 8]]
        );
        assert_eq!(
            calls(&log, TEXTURE_LIST_SAVE),
            vec![vec![this + 0xc, 0x5444_4f4d, 0x5000]]
        );
        let (_, log) = run(&mut e, 0x0048_91d0, &args![this, 0u32, 0u32]);
        assert!(calls(&log, ADD_CHUNK_ARRAY).is_empty());
        assert!(calls(&log, TEXTURE_LIST_SAVE).is_empty());
    }

    #[test]
    fn save_skips_an_empty_name() {
        let mut e = Engine::new();
        let this = model(&mut e, 0);
        let (_, log) = run(&mut e, 0x0048_91d0, &args![this, 1u32, 2u32]);
        assert!(calls(&log, ADD_CHUNK_ARRAY).is_empty());
        assert!(calls(&log, TEXTURE_LIST_SAVE).is_empty());
    }

    #[test]
    fn save_flags_writes_a_set_byte_only() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x18);
        e.register(FLAGS_BYTE, |e, a| {
            u32::from(e.mem.u8(a[0] + 0x14)).into_ret()
        });
        e.register(ADD_CHUNK_BYTE, |_, _| Ret::default());
        let (_, log) = run(&mut e, 0x0048_9240, &args![this, 0x4444_4f4du32]);
        assert!(calls(&log, ADD_CHUNK_BYTE).is_empty());
        e.mem.set_u8(this + 0x14, 0x82);
        let (_, log) = run(&mut e, 0x0048_9240, &args![this, 0x4444_4f4du32]);
        assert_eq!(calls(&log, ADD_CHUNK_BYTE), vec![vec![0x4444_4f4d, 0x82]]);
    }

    #[test]
    fn warn_flag_reads_its_byte() {
        let mut e = Engine::new();
        e.map(WARN_FLAG_BYTE & !0xfff, 0x1000);
        e.set_global::<u8>(WARN_FLAG_BYTE, 3);
        assert_eq!(e.call(0x0048_98e0, &[]).u32() & 0xff, 3);
    }
}
