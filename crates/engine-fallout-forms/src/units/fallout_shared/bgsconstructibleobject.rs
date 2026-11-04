//! `fallout shared/bgsconstructibleobject.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `004013e0` (`ECX` = form, `AL`): when true, `InitItem` does nothing.
const FORM_SKIPS_INIT: u32 = 0x0040_13e0;
/// `TESObjectMISC::InitItem` (engine map name of `0051a9a0`, `thiscall`).
const MISC_INIT_ITEM: u32 = 0x0051_a9a0;
/// `TESForm::GetFile` (`00484e60`, `RET 4`: the index, -1 here).
const FORM_GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex` (`00485d50`, `cdecl`: a form id's address, the file).
const FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// `004839c0` (`cdecl`, a form id): the loaded form with that id, or 0.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (`00ec43fb`, `cdecl`, five words).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// Type descriptors: `.?AVTESForm@@`, `.?AVBGSListForm@@`, `.?AVTESBoundObject@@`.
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_BGS_LIST_FORM: u32 = 0x0118_63d4;
const TYPE_TES_BOUND_OBJECT: u32 = 0x0118_3108;
/// `0084e3a0` (`ECX` = form): the word at +0xC (`TESForm::iFormID`).
const FORM_ID: u32 = 0x0084_e3a0;
/// The form's name virtual slot (byte offset), printed as `%s`.
const NAME_SLOT: u32 = 0x130;
/// `005b5e40` (`cdecl`, variable): prints a warning.
const PRINT: u32 = 0x005b_5e40;
/// "MASTERFILE: Could not find item list (%08X) on constructible object '%s' (%08X)."
const MISSING_ITEM_LIST_FORMAT: u32 = 0x0102_4780;
/// "MASTERFILE: Could not find created item (%08X) on constructible object '%s' (%08X)."
const MISSING_CREATED_ITEM_FORMAT: u32 = 0x0102_4728;
/// `00484ab0` (`ECX` = form, `RET 4`): sets (`true`) or clears form flag 0x8.
const SET_FORM_FLAG_8: u32 = 0x0048_4ab0;

/// `TESForm::LoadForm` (`00485110`, `RET 4`: the file).
const FORM_LOAD_FORM: u32 = 0x0048_5110;
/// `TESFile::GetTESChunk` (`004726b0`, `ECX` = file): the current chunk's signature.
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `TESFile::NextChunk` (`004726f0`, `ECX` = file, `AL`): moves to the next chunk.
const NEXT_CHUNK: u32 = 0x0047_26f0;
/// `00401660` (`ECX` = file): the current chunk's size.
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `TESFile::GetChunkData` (`00472890`, `RET 8`: address, at most this many bytes).
const GET_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
/// `TESFile::GetChunkData` (`004727f0`, `RET 4`: the address of a word).
const GET_CHUNK_WORD: u32 = 0x0047_27f0;
/// `00403570` (`ECX` = file, `AX`): the word the texture swap unpacking is given.
const FILE_WORD: u32 = 0x0040_3570;
/// `00487050` (`cdecl`: full name, file): loads the `FULL` chunk.
const LOAD_FULL_NAME: u32 = 0x0048_7050;
/// `TESModel::LoadModelChunk` (`004892d0`, `cdecl`: model, file).
const LOAD_MODEL_CHUNK: u32 = 0x0048_92d0;
/// `TESModelTextureSwap::UnpackTextureSwapChunkData` (`0048a7b0`, `RET 0xc`: data, size,
/// the file's word).
const UNPACK_TEXTURE_SWAP: u32 = 0x0048_a7b0;
/// `TESTexture::LoadTextureChunk` (`0048e3d0`, `cdecl`: texture, file).
const LOAD_TEXTURE_CHUNK: u32 = 0x0048_e3d0;
/// `0047a290` (`cdecl`: message icon, file): loads the icon's texture chunk.
const LOAD_MESSAGE_ICON: u32 = 0x0047_a290;
/// `006ecd40` (`ECX` = object, `RET 4`): stores its argument at +4.
const SET_WORD_4: u32 = 0x006e_cd40;
/// `TESScriptableForm::InitItem` (decompiler name of `0048cd90`; `ECX` = scriptable
/// form, `RET 4`: the owning form).
const SCRIPTABLE_INIT_ITEM: u32 = 0x0048_cd90;
/// `BGSDestructibleObjectForm::LoadChunk` (`004781e0`, `cdecl`: component, file).
const LOAD_DESTRUCTIBLE_CHUNK: u32 = 0x0047_81e0;
/// `TESForm::LoadData` (`004861f0`, `RET 0xc`: file, destination, flag).
const FORM_LOAD_DATA: u32 = 0x0048_61f0;

/// `TESForm::StartForm` and `TESForm::CloseForm` (`004855a0`, `00485680`, `thiscall`).
const FORM_START_FORM: u32 = 0x0048_55a0;
const FORM_CLOSE_FORM: u32 = 0x0048_5680;
/// `TESFullName::Save` (`00487010`, `thiscall`).
const FULL_NAME_SAVE: u32 = 0x0048_7010;
/// `TESModelTextureSwap::Save` (`0048a520`, `RET 0xc`: three chunk signatures).
const MODEL_TEXTURE_SWAP_SAVE: u32 = 0x0048_a520;
/// `TESTexture::Save` (`0048e370`, `RET 4`: the chunk signature).
const TEXTURE_SAVE: u32 = 0x0048_e370;
/// `BGSMessageIcon::Save` (`0047a240`, `RET 4`: the chunk signature).
const MESSAGE_ICON_SAVE: u32 = 0x0047_a240;
/// `TESScriptableForm::Save` (`0048cca0`, `thiscall`).
const SCRIPTABLE_SAVE: u32 = 0x0048_cca0;
/// `BGSDestructibleObjectForm::Save` (`00477f20`, `thiscall`).
const DESTRUCTIBLE_SAVE: u32 = 0x0047_7f20;
/// `TESForm::SaveData` (`00485e00`, `thiscall`).
const FORM_SAVE_DATA: u32 = 0x0048_5e00;
/// `00485910` (`cdecl`: chunk signature, word): saves a chunk holding one word.
const ADD_CHUNK_WORD: u32 = 0x0048_5910;

/// Chunk signatures, as the little-endian words of their four letters.
const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_DATA: u32 = 0x4154_4144;
const CHUNK_MODD: u32 = 0x4444_4f4d;
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_MOSD: u32 = 0x4453_4f4d;
const CHUNK_DSTD: u32 = 0x4454_5344;
const CHUNK_SCRI: u32 = 0x4952_4353;
const CHUNK_MODL: u32 = 0x4c44_4f4d;
const CHUNK_FULL: u32 = 0x4c4c_5546;
const CHUNK_CNAM: u32 = 0x4d41_4e43;
const CHUNK_RNAM: u32 = 0x4d41_4e52;
const CHUNK_ICON: u32 = 0x4e4f_4349;
const CHUNK_MICO: u32 = 0x4f43_494d;
const CHUNK_MODS: u32 = 0x5344_4f4d;
const CHUNK_MODT: u32 = 0x5444_4f4d;
const CHUNK_DEST: u32 = 0x5453_4544;

/// The most an `EDID` chunk is read with.
const EDITOR_ID_MAXIMUM: u32 = 0x200;
/// The object's virtual slots called with the file for `OBND` and with the text of an
/// `EDID` chunk.
const SLOT_LOAD_BOUNDS: u32 = 0xe0;
const SLOT_SET_EDITOR_ID: u32 = 0x134;

/// Component offsets inside the object (PC), as the code adds them to `this`.
const FULL_NAME_OFFSET: u32 = 0x30;
const MODEL_OFFSET: u32 = 0x3c;
const TEXTURE_OFFSET: u32 = 0x5c;
const SCRIPTABLE_OFFSET: u32 = 0x68;
const DESTRUCTIBLE_OFFSET: u32 = 0x84;
const MESSAGE_ICON_OFFSET: u32 = 0x8c;
/// The item list (`BGSListForm*`, a form id until `InitItem`) and the created item
/// (`TESBoundObject*`, likewise), saved as `RNAM` and `CNAM`.
const ITEM_LIST_OFFSET: u32 = 0xac;
const CREATED_ITEM_OFFSET: u32 = 0xb0;

/// Resolves the form id stored at `this + offset` (when non-zero): adjusted to the form's
/// file, looked up, cast from `TESForm` to `target_type`, and stored back; a warning with
/// `format` when nothing is found.
fn resolve_form_id(e: &mut Engine, this: Ptr, offset: u32, target_type: u32, format: u32) {
    let slot = this.addr() + offset;
    let loaded = e.mem.u32(slot);
    if loaded == 0 {
        return;
    }
    let adjusted = e.with_stack(4, |e, id| {
        e.mem.set_u32(id.addr(), loaded);
        let file = e.call(FORM_GET_FILE, &args![this, 0xffff_ffffu32]).u32();
        e.call(FORM_ADD_COMPILE_INDEX, &args![id, file]);
        e.mem.u32(id.addr())
    });
    let form = e.call(LOOKUP_FORM, &args![adjusted]).u32();
    let cast = e
        .call(
            DYNAMIC_CAST,
            &args![form, 0u32, TYPE_TES_FORM, target_type, 0u32],
        )
        .u32();
    e.mem.set_u32(slot, cast);
    if e.mem.u32(slot) == 0 {
        let id = e.call(FORM_ID, &args![this]).u32();
        let name = e.vcall(this.addr(), NAME_SLOT, &[]).u32();
        e.call(PRINT, &args![format, adjusted, name, id]);
    }
}

// Translated from 004f95a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSConstructibleObject::InitItem` (Xbox PDB; `thiscall`, no stack word): unless
/// `004013e0` answers true, runs `TESObjectMISC::InitItem` (`0051a9a0`), resolves the
/// item list id (+0xAC, a `BGSListForm`) and the created item id (+0xB0, a
/// `TESBoundObject`), each adjusted to the form's file, looked up and cast, with a
/// `MASTERFILE` warning naming the object when one is not found, and sets form flag 0x8
/// (`00484ab0` with 1).
pub fn bgs_constructible_object_init_item(e: &mut Engine, this: Ptr) {
    if e.call(FORM_SKIPS_INIT, &args![this]).u8() != 0 {
        return;
    }
    e.call(MISC_INIT_ITEM, &args![this]);
    resolve_form_id(
        e,
        this,
        ITEM_LIST_OFFSET,
        TYPE_BGS_LIST_FORM,
        MISSING_ITEM_LIST_FORMAT,
    );
    resolve_form_id(
        e,
        this,
        CREATED_ITEM_OFFSET,
        TYPE_TES_BOUND_OBJECT,
        MISSING_CREATED_ITEM_FORMAT,
    );
    e.call(SET_FORM_FLAG_8, &args![this, 1u32]);
}

/// Reads one word chunk (cleared first) through `004727f0`.
fn read_chunk_word(e: &mut Engine, file: Ptr) -> u32 {
    e.with_stack(4, |e, word| {
        e.mem.set_u32(word.addr(), 0);
        e.call(GET_CHUNK_WORD, &args![file, word]);
        e.mem.u32(word.addr())
    })
}

// Translated from 004f96f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004f96f0 - the /GS stack cookie check (00ec408c) is not translated
/// The constructible object's record load (`thiscall`, `RET 4`, `file`; unnamed in the
/// engine map), answers true. Reads the form header (`TESForm::LoadForm`), clears form
/// flag 0x8 (`00484ab0` with 0), then every chunk of the record:
/// - `EDID`: read into a stack buffer of the chunk's size (`_alloca_probe_16`, at most
///   0x200 bytes read) and passed to virtual slot `0x134`;
/// - `DATA`: `TESForm::LoadData` with no destination and flag 0;
/// - `OBND`: virtual slot `0xe0` with the file;
/// - `FULL`: `00487050` on `this + 0x30`;
/// - `MODL`, `MODD`, `MOSD`, `MODT`: `TESModel::LoadModelChunk` on `this + 0x3C`;
/// - `MODS`: the chunk is read into a stack buffer of its size and unpacked as a texture
///   swap on `this + 0x3C` (no null test there);
/// - `ICON`: `TESTexture::LoadTextureChunk` on `this + 0x5C`;
/// - `MICO`: `0047a290` on `this + 0x8C`;
/// - `SCRI`: the script form id stored at `this + 0x6C` (`006ecd40` on `this + 0x68`), then
///   `0048cd90` on `this + 0x68` with the object;
/// - `DEST`, `DSTD`: `BGSDestructibleObjectForm::LoadChunk` on `this + 0x84`;
/// - `RNAM`, `CNAM`: the item list and created item form ids (+0xAC, +0xB0).
///
/// The components are null for a null `this`, as the code tests.
pub fn fn_004f96f0(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let object = this.addr();
    let component = |offset: u32| if object == 0 { 0 } else { object + offset };
    e.call(FORM_LOAD_FORM, &args![this, file]);
    e.call(SET_FORM_FLAG_8, &args![this, 0u32]);
    loop {
        let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            CHUNK_EDID => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                // `_alloca_probe_16`: the buffer is the size rounded up to 16 bytes.
                let reserved = size.wrapping_add(15) & !15;
                e.with_stack(reserved, |e, buffer| {
                    e.call(
                        GET_CHUNK_DATA_SIZED,
                        &args![file, buffer, EDITOR_ID_MAXIMUM],
                    );
                    e.vcall(object, SLOT_SET_EDITOR_ID, &args![buffer]);
                });
            }
            CHUNK_DATA => {
                e.call(FORM_LOAD_DATA, &args![this, file, 0u32, 0u32]);
            }
            CHUNK_OBND => {
                e.vcall(object, SLOT_LOAD_BOUNDS, &args![file]);
            }
            CHUNK_FULL => {
                e.call(LOAD_FULL_NAME, &args![component(FULL_NAME_OFFSET), file]);
            }
            CHUNK_MODL | CHUNK_MODD | CHUNK_MOSD | CHUNK_MODT => {
                e.call(LOAD_MODEL_CHUNK, &args![component(MODEL_OFFSET), file]);
            }
            CHUNK_MODS => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                let reserved = size.wrapping_add(15) & !15;
                e.with_stack(reserved, |e, buffer| {
                    e.call(GET_CHUNK_DATA_SIZED, &args![file, buffer, size]);
                    let word = e.call(FILE_WORD, &args![file]).u16();
                    e.call(
                        UNPACK_TEXTURE_SWAP,
                        &args![
                            object.wrapping_add(MODEL_OFFSET),
                            buffer,
                            size,
                            u32::from(word)
                        ],
                    );
                });
            }
            CHUNK_ICON => {
                e.call(LOAD_TEXTURE_CHUNK, &args![component(TEXTURE_OFFSET), file]);
            }
            CHUNK_MICO => {
                e.call(
                    LOAD_MESSAGE_ICON,
                    &args![component(MESSAGE_ICON_OFFSET), file],
                );
            }
            CHUNK_SCRI => {
                let script = read_chunk_word(e, file);
                let scriptable = object.wrapping_add(SCRIPTABLE_OFFSET);
                e.call(SET_WORD_4, &args![scriptable, script]);
                e.call(SCRIPTABLE_INIT_ITEM, &args![scriptable, this]);
            }
            CHUNK_DEST | CHUNK_DSTD => {
                e.call(
                    LOAD_DESTRUCTIBLE_CHUNK,
                    &args![component(DESTRUCTIBLE_OFFSET), file],
                );
            }
            CHUNK_RNAM => {
                let id = read_chunk_word(e, file);
                e.mem.set_u32(object.wrapping_add(ITEM_LIST_OFFSET), id);
            }
            CHUNK_CNAM => {
                let id = read_chunk_word(e, file);
                e.mem.set_u32(object.wrapping_add(CREATED_ITEM_OFFSET), id);
            }
            _ => {}
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

/// Saves the form id of the form at `this + offset` (when set) as a `chunk` word.
fn save_form_id(e: &mut Engine, this: Ptr, offset: u32, chunk: u32) {
    let form = e.mem.u32(this.addr() + offset);
    if form != 0 {
        let id = e.call(FORM_ID, &args![form]).u32();
        e.call(ADD_CHUNK_WORD, &args![chunk, id]);
    }
}

// Translated from 004f9a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSConstructibleObject::Save` (Xbox PDB; `thiscall`, no stack word): starts the form
/// record, saves the full name (+0x30), the model (+0x3C: `MODL`, `MODT`, `MODS`), the
/// icon texture (+0x5C, `ICON`), the message icon (+0x8C, `MICO`), the script (+0x68),
/// the destruction data (+0x84) and the form data (`TESForm::SaveData`), then the item
/// list (`RNAM`) and created item (`CNAM`) form ids when set, and closes the record.
pub fn bgs_constructible_object_save(e: &mut Engine, this: Ptr) {
    let object = this.addr();
    e.call(FORM_START_FORM, &args![this]);
    e.call(FULL_NAME_SAVE, &args![object + FULL_NAME_OFFSET]);
    e.call(
        MODEL_TEXTURE_SWAP_SAVE,
        &args![object + MODEL_OFFSET, CHUNK_MODL, CHUNK_MODT, CHUNK_MODS],
    );
    e.call(TEXTURE_SAVE, &args![object + TEXTURE_OFFSET, CHUNK_ICON]);
    e.call(
        MESSAGE_ICON_SAVE,
        &args![object + MESSAGE_ICON_OFFSET, CHUNK_MICO],
    );
    e.call(SCRIPTABLE_SAVE, &args![object + SCRIPTABLE_OFFSET]);
    e.call(DESTRUCTIBLE_SAVE, &args![object + DESTRUCTIBLE_OFFSET]);
    e.call(FORM_SAVE_DATA, &args![this]);
    save_form_id(e, this, ITEM_LIST_OFFSET, CHUNK_RNAM);
    save_form_id(e, this, CREATED_ITEM_OFFSET, CHUNK_CNAM);
    e.call(FORM_CLOSE_FORM, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004f_95a0, bgs_constructible_object_init_item(Ptr)),
        entry!(0x004f_96f0, fn_004f96f0(Ptr, Ptr) -> bool),
        entry!(0x004f_9a90, bgs_constructible_object_save(Ptr)),
    ]
}

#[cfg(test)]
mod tests_n036 {
    use super::*;

    type CallLog = Vec<(u32, Vec<u32>)>;

    fn to(log: &CallLog, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An object with a vtable whose name slot answers `0x7777` and whose other slots
    /// do nothing.
    fn object(e: &mut Engine) -> u32 {
        e.put_vtable(0x0170_0000, &[0x0000_3000; 0x50]);
        e.register(0x0000_3000, |_, _| 0x7777u32.into_ret());
        let this = e.mem.alloc(0xc0);
        e.mem.set_u32(this, 0x0170_0000);
        this
    }

    fn init_engine(skip: bool, found: u32) -> Engine {
        let mut e = Engine::new();
        e.register_double(FORM_SKIPS_INIT, move |_, _| u32::from(skip).into_ret());
        e.register(MISC_INIT_ITEM, |_, _| Ret::default());
        e.register(FORM_GET_FILE, |_, _| 0x55u32.into_ret());
        e.register(FORM_ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0200_0000);
            Ret::default()
        });
        e.register(LOOKUP_FORM, |_, a| (a[0] ^ 0x0200_0000).into_ret());
        e.register_double(DYNAMIC_CAST, move |_, a| {
            if found != 0 { a[0] + found } else { 0 }.into_ret()
        });
        e.register(FORM_ID, |_, _| 0x1234u32.into_ret());
        e.register(PRINT, |_, _| Ret::default());
        e.register(SET_FORM_FLAG_8, |_, _| Ret::default());
        e
    }

    #[test]
    fn init_item_resolves_both_forms() {
        let mut e = init_engine(false, 0x10);
        let this = object(&mut e);
        e.mem.set_u32(this + 0xac, 0x41);
        e.mem.set_u32(this + 0xb0, 0x42);
        e.call_log = Some(vec![]);
        e.call(0x004f_95a0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this + 0xac), 0x51);
        assert_eq!(e.mem.u32(this + 0xb0), 0x52);
        assert_eq!(
            to(&log, DYNAMIC_CAST),
            vec![
                vec![0x41, 0, TYPE_TES_FORM, TYPE_BGS_LIST_FORM, 0],
                vec![0x42, 0, TYPE_TES_FORM, TYPE_TES_BOUND_OBJECT, 0]
            ]
        );
        assert_eq!(to(&log, FORM_GET_FILE), vec![vec![this, 0xffff_ffff]; 2]);
        assert!(to(&log, PRINT).is_empty());
        assert_eq!(to(&log, SET_FORM_FLAG_8), vec![vec![this, 1]]);
    }

    #[test]
    fn init_item_warns_for_missing_forms_and_skips_empty_ids() {
        let mut e = init_engine(false, 0);
        let this = object(&mut e);
        e.mem.set_u32(this + 0xb0, 0x42);
        e.call_log = Some(vec![]);
        e.call(0x004f_95a0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(to(&log, LOOKUP_FORM).len(), 1);
        assert_eq!(
            to(&log, PRINT),
            vec![vec![
                MISSING_CREATED_ITEM_FORMAT,
                0x0200_0042,
                0x7777,
                0x1234
            ]]
        );
        let mut e = init_engine(true, 0);
        let this = object(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x004f_95a0, &args![this]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }

    /// Runs the load over `chunks` (each a signature and, for word chunks, its word).
    fn load(chunks: &[(u32, u32)], this_null: bool) -> (u32, CallLog) {
        let mut e = Engine::new();
        let this = if this_null { 0 } else { object(&mut e) };
        let file = e.mem.alloc(0x40);
        let list: Vec<(u32, u32)> = chunks.to_vec();
        e.mem.set_u32(file, 0);
        e.register_double(GET_TES_CHUNK, move |e, a| {
            let index = e.mem.u32(a[0]) as usize;
            list.get(index).map_or(0, |c| c.0).into_ret()
        });
        let words: Vec<(u32, u32)> = chunks.to_vec();
        e.register_double(GET_CHUNK_WORD, move |e, a| {
            let index = e.mem.u32(a[0]) as usize;
            e.mem.set_u32(a[1], words[index].1);
            Ret::default()
        });
        let count = chunks.len() as u32;
        e.register_double(NEXT_CHUNK, move |e, a| {
            let index = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], index);
            u32::from(index < count).into_ret()
        });
        e.register(CHUNK_SIZE, |_, _| 0x12u32.into_ret());
        e.register(FILE_WORD, |_, _| 0xffff_0042u32.into_ret());
        for addr in [
            FORM_LOAD_FORM,
            SET_FORM_FLAG_8,
            GET_CHUNK_DATA_SIZED,
            LOAD_FULL_NAME,
            LOAD_MODEL_CHUNK,
            UNPACK_TEXTURE_SWAP,
            LOAD_TEXTURE_CHUNK,
            LOAD_MESSAGE_ICON,
            SET_WORD_4,
            SCRIPTABLE_INIT_ITEM,
            LOAD_DESTRUCTIBLE_CHUNK,
            FORM_LOAD_DATA,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        assert!(e.call(0x004f_96f0, &args![this, file]).bool());
        let mut log = e.call_log.take().unwrap();
        log.retain(|c| {
            ![
                GET_TES_CHUNK,
                NEXT_CHUNK,
                GET_CHUNK_WORD,
                CHUNK_SIZE,
                FILE_WORD,
            ]
            .contains(&c.0)
        });
        if !this_null {
            assert_eq!(e.mem.u32(this + 0xac), words_of(chunks, CHUNK_RNAM));
            assert_eq!(e.mem.u32(this + 0xb0), words_of(chunks, CHUNK_CNAM));
        }
        (this, log)
    }

    fn words_of(chunks: &[(u32, u32)], tag: u32) -> u32 {
        chunks.iter().find(|c| c.0 == tag).map_or(0, |c| c.1)
    }

    #[test]
    fn load_dispatches_each_chunk() {
        let (this, log) = load(
            &[
                (CHUNK_FULL, 0),
                (CHUNK_MODT, 0),
                (CHUNK_ICON, 0),
                (CHUNK_MICO, 0),
                (CHUNK_DSTD, 0),
                (CHUNK_DATA, 0),
                (CHUNK_SCRI, 0x5c),
                (CHUNK_RNAM, 0x61),
                (CHUNK_CNAM, 0x62),
                (CHUNK_MODS, 0),
                (0x5858_5858, 0),
            ],
            false,
        );
        let file = log[0].1[1];
        let mods = &to(&log, UNPACK_TEXTURE_SWAP)[0];
        assert_eq!(
            log[1..],
            [
                (FORM_LOAD_FORM, vec![this, file]),
                (SET_FORM_FLAG_8, vec![this, 0]),
                (LOAD_FULL_NAME, vec![this + 0x30, file]),
                (LOAD_MODEL_CHUNK, vec![this + 0x3c, file]),
                (LOAD_TEXTURE_CHUNK, vec![this + 0x5c, file]),
                (LOAD_MESSAGE_ICON, vec![this + 0x8c, file]),
                (LOAD_DESTRUCTIBLE_CHUNK, vec![this + 0x84, file]),
                (FORM_LOAD_DATA, vec![this, file, 0, 0]),
                (SET_WORD_4, vec![this + 0x68, 0x5c]),
                (SCRIPTABLE_INIT_ITEM, vec![this + 0x68, this]),
                (GET_CHUNK_DATA_SIZED, vec![file, mods[1], 0x12]),
                (UNPACK_TEXTURE_SWAP, vec![this + 0x3c, mods[1], 0x12, 0x42]),
            ]
        );
    }

    #[test]
    fn load_editor_id_and_bounds_go_through_the_vtable() {
        let (this, log) = load(&[(CHUNK_EDID, 0), (CHUNK_OBND, 0)], false);
        let file = log[0].1[1];
        let read = to(&log, GET_CHUNK_DATA_SIZED);
        assert_eq!(read.len(), 1);
        assert_eq!((read[0][0], read[0][2]), (file, EDITOR_ID_MAXIMUM));
        assert_eq!(
            to(&log, 0x0000_3000),
            vec![vec![this, read[0][1]], vec![this, file]]
        );
    }

    #[test]
    fn load_with_a_null_object_passes_null_components() {
        let (_, log) = load(&[(CHUNK_FULL, 0), (CHUNK_MODL, 0), (CHUNK_DEST, 0)], true);
        let file = log[0].1[1];
        assert_eq!(to(&log, LOAD_FULL_NAME), vec![vec![0, file]]);
        assert_eq!(to(&log, LOAD_MODEL_CHUNK), vec![vec![0, file]]);
        assert_eq!(to(&log, LOAD_DESTRUCTIBLE_CHUNK), vec![vec![0, file]]);
    }

    #[test]
    fn save_writes_every_part_in_order() {
        let mut e = Engine::new();
        for addr in [
            FORM_START_FORM,
            FORM_CLOSE_FORM,
            FULL_NAME_SAVE,
            MODEL_TEXTURE_SWAP_SAVE,
            TEXTURE_SAVE,
            MESSAGE_ICON_SAVE,
            SCRIPTABLE_SAVE,
            DESTRUCTIBLE_SAVE,
            FORM_SAVE_DATA,
            ADD_CHUNK_WORD,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(FORM_ID, |_, a| (a[0] + 1).into_ret());
        let this = e.mem.alloc(0xc0);
        e.mem.set_u32(this + 0xb0, 0x9000);
        e.call_log = Some(vec![]);
        e.call(0x004f_9a90, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (FORM_START_FORM, vec![this]),
                (FULL_NAME_SAVE, vec![this + 0x30]),
                (
                    MODEL_TEXTURE_SWAP_SAVE,
                    vec![this + 0x3c, CHUNK_MODL, CHUNK_MODT, CHUNK_MODS]
                ),
                (TEXTURE_SAVE, vec![this + 0x5c, CHUNK_ICON]),
                (MESSAGE_ICON_SAVE, vec![this + 0x8c, CHUNK_MICO]),
                (SCRIPTABLE_SAVE, vec![this + 0x68]),
                (DESTRUCTIBLE_SAVE, vec![this + 0x84]),
                (FORM_SAVE_DATA, vec![this]),
                (FORM_ID, vec![0x9000]),
                (ADD_CHUNK_WORD, vec![CHUNK_CNAM, 0x9001]),
                (FORM_CLOSE_FORM, vec![this]),
            ]
        );
    }
}
