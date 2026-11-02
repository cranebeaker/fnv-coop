//! `fallout shared/bgsmovablestatic.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESFile::GetTESForm` (Xbox PDB name of `00472660`; `ECX` = file): the
/// form type of the record (the result is not used here).
const GET_TES_FORM: u32 = 0x0047_2660;
/// `TESForm::LoadForm` (Xbox PDB name of `00485110`; `ECX` = form, the file).
const TES_FORM_LOAD_FORM: u32 = 0x0048_5110;
/// `TESFile::GetTESChunk` (Xbox PDB name of `004726b0`; `ECX` = file): the id
/// of the current chunk, 0 at the end.
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `004726f0` (`ECX` = file): moves to the next chunk; true when there is one.
const NEXT_CHUNK: u32 = 0x0047_26f0;
/// The current chunk size (`00401660`, `TESFile` `+0x25C`).
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `TESFile::GetChunkData` (Xbox PDB overload `_ov3`, `00472890`): `thiscall`
/// (file; buffer, size).
const GET_CHUNK_DATA: u32 = 0x0047_2890;
/// `TESFile::GetChunkData` (Xbox PDB name of `004727f0`; `ECX` = file, the
/// address of a word): reads the chunk into the word.
const GET_CHUNK_WORD: u32 = 0x0047_27f0;
/// `00403570` (`ECX` = file): a 16-bit value (the result is in `AX`) that the
/// texture swap unpacking takes as its last argument.
const FILE_WORD: u32 = 0x0040_3570;
/// The component loader `00487050` (`cdecl`: component, file).
const LOAD_FULL_NAME: u32 = 0x0048_7050;
/// `TESModel::LoadModelChunk` (Xbox PDB name of `004892d0`; `cdecl`: model,
/// file).
const LOAD_MODEL_CHUNK: u32 = 0x0048_92d0;
/// `TESModelTextureSwap::UnpackTextureSwapChunkData` (Xbox PDB name of
/// `0048a7b0`; `ECX` = the model at `+0x30`, buffer, size, a 16-bit value).
const UNPACK_TEXTURE_SWAP: u32 = 0x0048_a7b0;
/// `BGSDestructibleObjectForm::LoadChunk` (Xbox PDB name of `004781e0`;
/// `cdecl`: destructible component, file).
const LOAD_DESTRUCTIBLE_CHUNK: u32 = 0x0047_81e0;
/// `TESForm::LoadData` (Xbox PDB name of `004861f0`; `ECX` = form, file,
/// destination, a flag).
const TES_FORM_LOAD_DATA: u32 = 0x0048_61f0;
/// `004fbf00` (`ECX` = the object at `this - 0x14`, one word): takes the
/// `SNAM` value.
const SET_SOUND: u32 = 0x004f_bf00;

/// Chunk ids, as the little-endian words of their four letters.
const CHUNK_MODL: u32 = 0x4c44_4f4d;
const CHUNK_MODD: u32 = 0x4444_4f4d;
const CHUNK_MOSD: u32 = 0x4453_4f4d;
const CHUNK_MODT: u32 = 0x5444_4f4d;
const CHUNK_MODS: u32 = 0x5344_4f4d;
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_DATA: u32 = 0x4154_4144;
const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_DSTD: u32 = 0x4454_5344;
const CHUNK_DEST: u32 = 0x5453_4544;
const CHUNK_FULL: u32 = 0x4c4c_5546;
const CHUNK_SNAM: u32 = 0x4d41_4e53;

/// The object's virtual slot called with the file for an `OBND` chunk.
const SLOT_LOAD_BOUNDS: u32 = 0xe0;
/// The object's virtual slot called with the text of an `EDID` chunk.
const SLOT_SET_EDITOR_ID: u32 = 0x134;

/// How far below `this` the object the chunk readers work on starts.
const OBJECT_OFFSET: u32 = 0x14;
/// `this + 0x30`: the model component the model and texture swap chunks go to.
const MODEL_OFFSET: u32 = 0x30;
/// `this - 8`: the destructible component the destruction chunks go to.
const DESTRUCTIBLE_BACKWARD: u32 = 8;
/// `this + 0x5c`: where the `DATA` chunk is read to.
const DATA_OFFSET: u32 = 0x5c;
/// The most an `EDID` chunk is read with.
const EDITOR_ID_MAXIMUM: u32 = 0x200;

// Translated from 004fbc70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004fbc70 - the /GS stack cookie check (00ec408c) is not translated.
/// `BGSMovableStatic::Load` (Xbox PDB; `thiscall`, `RET 4`, `file`), answers
/// true. The code addresses the object relative to `this`: the object the
/// readers are given starts at `this - 0x14`, and a `this` of exactly `0x14`
/// counts as a null object for the model and destruction components. It reads
/// the form header (`TESFile::GetTESForm` is called and its result dropped,
/// `TESForm::LoadForm`), then every chunk of the record:
/// - `MODL`, `MODD`, `MOSD`, `MODT`: `TESModel::LoadModelChunk` on `this + 0x30`;
/// - `MODS`: the chunk is read into a stack buffer of its size and unpacked as
///   a texture swap on `this + 0x30`;
/// - `OBND`: virtual slot `0xe0` with the file;
/// - `EDID`: read into a stack buffer (at most 0x200 bytes) and passed to
///   virtual slot `0x134`;
/// - `FULL`: `00487050` on `this - 0x14`;
/// - `DEST`, `DSTD`: `BGSDestructibleObjectForm::LoadChunk` on `this - 8`;
/// - `DATA`: `TESForm::LoadData` with the destination `this + 0x5c`;
/// - `SNAM`: the chunk word handed to `004fbf00` on `this - 0x14`.
///
/// The stack cookie check is not translated.
pub fn bgs_movable_static_load(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let object = this.addr();
    let base = object.wrapping_sub(OBJECT_OFFSET);
    // `this - 0x14` null: the model and destruction components are null.
    let model = if base == 0 { 0 } else { object + MODEL_OFFSET };
    let destructible = if base == 0 {
        0
    } else {
        object.wrapping_sub(DESTRUCTIBLE_BACKWARD)
    };
    e.call(GET_TES_FORM, &args![file]);
    e.call(TES_FORM_LOAD_FORM, &args![this, file]);
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
                    e.call(GET_CHUNK_DATA, &args![file, buffer, EDITOR_ID_MAXIMUM]);
                    e.vcall(object, SLOT_SET_EDITOR_ID, &args![buffer]);
                });
            }
            CHUNK_OBND => {
                e.vcall(object, SLOT_LOAD_BOUNDS, &args![file]);
            }
            CHUNK_FULL => {
                e.call(LOAD_FULL_NAME, &args![base, file]);
            }
            CHUNK_MODL | CHUNK_MODD | CHUNK_MOSD | CHUNK_MODT => {
                e.call(LOAD_MODEL_CHUNK, &args![model, file]);
            }
            CHUNK_MODS => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                let reserved = size.wrapping_add(15) & !15;
                e.with_stack(reserved, |e, buffer| {
                    e.call(GET_CHUNK_DATA, &args![file, buffer, size]);
                    let word = e.call(FILE_WORD, &args![file]).u16();
                    e.call(
                        UNPACK_TEXTURE_SWAP,
                        &args![object + MODEL_OFFSET, buffer, size, u32::from(word)],
                    );
                });
            }
            CHUNK_DEST | CHUNK_DSTD => {
                e.call(LOAD_DESTRUCTIBLE_CHUNK, &args![destructible, file]);
            }
            CHUNK_DATA => {
                e.call(
                    TES_FORM_LOAD_DATA,
                    &args![this, file, object + DATA_OFFSET, 1u32],
                );
            }
            CHUNK_SNAM => {
                let value = e.with_stack(4, |e, word| {
                    e.mem.set_u32(word.addr(), 0);
                    e.call(GET_CHUNK_WORD, &args![file, word]);
                    e.mem.u32(word.addr())
                });
                e.call(SET_SOUND, &args![base, value]);
            }
            _ => {}
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

/// `TESForm::StartForm` and `TESForm::CloseForm` (Xbox PDB; `004855a0`, `00485680`,
/// `thiscall`).
const FORM_START_FORM: u32 = 0x0048_55a0;
const FORM_CLOSE_FORM: u32 = 0x0048_5680;
/// `TESFullName::Save` (Xbox PDB name of `00487010`, `thiscall`).
const FULL_NAME_SAVE: u32 = 0x0048_7010;
/// `TESModelTextureSwap::Save` (Xbox PDB name of `0048a520`; `RET 0xc`: the model,
/// texture and texture-swap chunk signatures).
const MODEL_TEXTURE_SWAP_SAVE: u32 = 0x0048_a520;
/// `BGSDestructibleObjectForm::Save` (Xbox PDB name of `00477f20`, `thiscall`).
const DESTRUCTIBLE_SAVE: u32 = 0x0047_7f20;
/// `00401500` (`cdecl`, no arguments, `AL`): whether saved data is byte-swapped.
const SWAP_ON_SAVE: u32 = 0x0040_1500;
/// `00483710` (`ECX` = the data byte): the byte's swap, which does nothing.
const SWAP_DATA_BYTE: u32 = 0x0048_3710;
/// `TESForm::__SaveData` (Xbox PDB name of `00485e20`; `RET 8`: address, size).
const FORM_SAVE_DATA: u32 = 0x0048_5e20;
/// `00485910` (`cdecl`: chunk signature, word): saves a chunk holding one word.
const ADD_CHUNK_WORD: u32 = 0x0048_5910;
/// `0084e3a0` (`ECX` = form): the word at +0xC (`TESForm::iFormID`).
const FORM_ID: u32 = 0x0084_e3a0;
/// `this + 0x58`: the sound form (`TESSound*`) saved as `SNAM`.
const SOUND_OFFSET: u32 = 0x58;

// Translated from 004fbf20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSMovableStatic::Save` (Xbox PDB; `thiscall`, no stack word): starts the form record,
/// saves the full name (`00487010` on `this - 0x14`), the model (`MODL`, `MODT`, `MODS`
/// on `this + 0x30`), the destruction data (`00477f20` on `this - 8`), the one data byte
/// at `this + 0x5C` (`TESForm::__SaveData`, with the byte's no-op swap `00483710` before
/// and after when `00401500` says so), and the sound's form id (`SNAM`, when a sound is
/// set at `this + 0x58`), then closes the record.
pub fn bgs_movable_static_save(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(FORM_START_FORM, &args![this]);
    e.call(FULL_NAME_SAVE, &args![base.wrapping_sub(OBJECT_OFFSET)]);
    e.call(
        MODEL_TEXTURE_SWAP_SAVE,
        &args![
            base.wrapping_add(MODEL_OFFSET),
            CHUNK_MODL,
            CHUNK_MODT,
            CHUNK_MODS
        ],
    );
    e.call(
        DESTRUCTIBLE_SAVE,
        &args![base.wrapping_sub(DESTRUCTIBLE_BACKWARD)],
    );
    let data = base.wrapping_add(DATA_OFFSET);
    if e.call(SWAP_ON_SAVE, &[]).u8() != 0 {
        e.call(SWAP_DATA_BYTE, &args![data]);
    }
    e.call(FORM_SAVE_DATA, &args![this, data, 1u32]);
    if e.call(SWAP_ON_SAVE, &[]).u8() != 0 {
        e.call(SWAP_DATA_BYTE, &args![data]);
    }
    let sound = e.mem.u32(base.wrapping_add(SOUND_OFFSET));
    if sound != 0 {
        let id = e.call(FORM_ID, &args![sound]).u32();
        e.call(ADD_CHUNK_WORD, &args![CHUNK_SNAM, id]);
    }
    e.call(FORM_CLOSE_FORM, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004fbc70, bgs_movable_static_load(Ptr, Ptr) -> bool),
        entry!(0x004f_bb70, bgs_movable_static_init_item(Ptr)),
        entry!(0x004f_bf00, fn_004fbf00(Ptr, u32)),
        entry!(0x004f_bf20, bgs_movable_static_save(Ptr)),
    ]
}

/// `004013e0` (thiscall, bool): when true, `InitItem` does nothing.
const FORM_TEST_4013E0: u32 = 0x0040_13e0;
/// `BGSDestructibleObjectForm::InitItem` (Xbox PDB; thiscall(form)).
const DESTRUCTIBLE_INIT_ITEM: u32 = 0x0047_8be0;
/// `TESModelTextureSwap::InitItem` (Xbox PDB; thiscall(form)).
const TEXTURE_SWAP_INIT_ITEM: u32 = 0x0048_aa80;
/// `TESForm::GetFile` (Xbox PDB; thiscall(index)).
const TES_FORM_GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex` (Xbox PDB; cdecl(form id address, file)).
const TES_FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// The form lookup by id (`004839c0`, cdecl(id)).
const LOOKUP_FORM_BY_ID: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (cdecl(object, offset, source type, target type, is
/// reference)) and its two type descriptors here (not read here).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const CAST_SOURCE_TYPE: u32 = 0x0118_3028;
const CAST_TARGET_TYPE: u32 = 0x0118_32fc;
/// The form's id (`0084e3a0`, thiscall).
const FORM_ID_84E3A0: u32 = 0x0084_e3a0;
/// The form's virtual slot giving its editor id (no words).
const SLOT_GET_EDITOR_ID: u32 = 0x130;
/// The warning printer (`005b5e40`, cdecl(format, ...)).
const WARNING: u32 = 0x005b_5e40;
/// `"MASTERFILE: Could not find looping sound (%08X) for movable static
/// '%s' (%08X)."`.
const LOOPING_SOUND_WARNING: u32 = 0x0102_4e98;
/// `00484ab0` (`tesform.cpp`, thiscall(flag)).
const TES_FORM_SET_FLAG_484AB0: u32 = 0x0048_4ab0;
/// The looping sound's form id, then the sound, at `this + 0x58`.
const LOOPING_SOUND: u32 = 0x58;

// Translated from 004fbb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSMovableStatic::InitItem` (Xbox PDB): nothing when `004013e0` says
/// so; else the destructible form (`this - 8`) and the texture swap
/// (`this + 0x30`) are initialised with the form (`this`, null when `this`
/// is exactly 0x14, the code's base adjustment), the looping-sound form id
/// at `+0x58` (when not 0) is resolved like `BGSTalkingActivator`'s voice
/// type (compile index, lookup, `__RTDynamicCast`) with a `MASTERFILE`
/// warning when that gives null, and `00484ab0(1)` ends it.
pub fn bgs_movable_static_init_item(e: &mut Engine, this: Ptr) {
    if e.call(FORM_TEST_4013E0, &args![this]).bool() {
        return;
    }
    let base = this.addr();
    let form = if base == OBJECT_OFFSET { 0 } else { base };
    e.call(
        DESTRUCTIBLE_INIT_ITEM,
        &args![base.wrapping_sub(DESTRUCTIBLE_BACKWARD), form],
    );
    let form = if base == OBJECT_OFFSET { 0 } else { base };
    e.call(
        TEXTURE_SWAP_INIT_ITEM,
        &args![base.wrapping_add(MODEL_OFFSET), form],
    );
    let sound_id = e.mem.u32(base.wrapping_add(LOOPING_SOUND));
    if sound_id != 0 {
        e.with_stack(4, |e, id| {
            e.mem.set_u32(id.addr(), sound_id);
            let file = e.call(TES_FORM_GET_FILE, &args![this, u32::MAX]).u32();
            e.call(TES_FORM_ADD_COMPILE_INDEX, &args![id, file]);
            let id_value = e.mem.u32(id.addr());
            let found = e.call(LOOKUP_FORM_BY_ID, &args![id_value]).u32();
            let sound = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![found, 0u32, CAST_SOURCE_TYPE, CAST_TARGET_TYPE, 0u32],
                )
                .u32();
            e.mem.set_u32(base.wrapping_add(LOOPING_SOUND), sound);
            if e.mem.u32(base.wrapping_add(LOOPING_SOUND)) == 0 {
                let form_id = e.call(FORM_ID_84E3A0, &args![this]).u32();
                let name = e.vcall(base, SLOT_GET_EDITOR_ID, &args![]).u32();
                let id_value = e.mem.u32(id.addr());
                e.call(
                    WARNING,
                    &args![LOOPING_SOUND_WARNING, id_value, name, form_id],
                );
            }
        });
    }
    e.call(TES_FORM_SET_FLAG_484AB0, &args![this, 1u32]);
}

// Translated from 004fbf00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` at `this + 0x6C` (the looping sound, with `this` the
/// object `Load` passes at `this - 0x14`).
pub fn fn_004fbf00(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr().wrapping_add(0x6c), value);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    const FILE: u32 = 0x6000;
    const OBJECT_TABLE: u32 = 0x0700_0000;
    const TARGET_LOAD_BOUNDS: u32 = 0x0050_0001;
    const TARGET_SET_EDITOR_ID: u32 = 0x0050_0002;

    type CallLog = Vec<(u32, Vec<u32>)>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls_to(log: &CallLog, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// Runs the loader on a record of `chunks` with the object at `this`
    /// (when 0, a real object is allocated and its address used; the object
    /// holds the vtable at +0 either way). Returns the call log (without the
    /// top-level call), the address of `this` and the result.
    fn run(chunks: Vec<u32>, this: u32, chunk_size: u32) -> (CallLog, u32, bool) {
        let mut e = Engine::new();
        let position = Rc::new(Cell::new(0usize));
        let (list, at) = (chunks.clone(), position.clone());
        e.register_double(GET_TES_CHUNK, move |_, _| {
            ret(list.get(at.get()).copied().unwrap_or(0))
        });
        let total = chunks.len();
        e.register_double(NEXT_CHUNK, move |_, _| {
            position.set(position.get() + 1);
            ret(u32::from(position.get() < total))
        });
        for addr in [
            GET_TES_FORM,
            TES_FORM_LOAD_FORM,
            GET_CHUNK_DATA,
            LOAD_FULL_NAME,
            LOAD_MODEL_CHUNK,
            UNPACK_TEXTURE_SWAP,
            LOAD_DESTRUCTIBLE_CHUNK,
            TES_FORM_LOAD_DATA,
            SET_SOUND,
            TARGET_LOAD_BOUNDS,
            TARGET_SET_EDITOR_ID,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register_double(CHUNK_SIZE, move |_, _| ret(chunk_size));
        e.register(FILE_WORD, |_, _| ret(0xabcd_0007));
        e.register(GET_CHUNK_WORD, |e, a| {
            e.mem.set_u32(a[1], 0x0bad_f00d);
            Ret::default()
        });
        let mut table = vec![0u32; 0x4e];
        table[0xe0 / 4] = TARGET_LOAD_BOUNDS;
        table[0x134 / 4] = TARGET_SET_EDITOR_ID;
        e.put_vtable(OBJECT_TABLE, &table);
        let this = if this == 0 {
            let object = e.mem.alloc(0x80);
            e.mem.set_u32(object, OBJECT_TABLE);
            object
        } else {
            this
        };
        e.call_log = Some(vec![]);
        let result = e.call(0x004fbc70, &args![this, FILE]).bool();
        let log = e.call_log.take().unwrap();
        (log.into_iter().skip(1).collect(), this, result)
    }

    #[test]
    fn the_header_is_read_before_the_chunks() {
        let (log, this, result) = run(vec![], 0, 0);
        assert!(result);
        assert_eq!(log[0], (GET_TES_FORM, vec![FILE]));
        assert_eq!(log[1], (TES_FORM_LOAD_FORM, vec![this, FILE]));
        assert_eq!(log[2], (GET_TES_CHUNK, vec![FILE]));
        assert_eq!(log.len(), 3);
    }

    #[test]
    fn the_virtual_chunks_go_to_the_objects_slots() {
        let (log, this, _) = run(vec![CHUNK_OBND, CHUNK_EDID, 0x5555_5555], 0, 5);
        assert_eq!(calls_to(&log, TARGET_LOAD_BOUNDS), vec![vec![this, FILE]]);
        let data = calls_to(&log, GET_CHUNK_DATA);
        assert_eq!(data.len(), 1);
        assert_eq!((data[0][0], data[0][2]), (FILE, 0x200));
        assert_eq!(
            calls_to(&log, TARGET_SET_EDITOR_ID),
            vec![vec![this, data[0][1]]]
        );
        // Nothing else is read for the unknown chunk.
        assert!(calls_to(&log, LOAD_MODEL_CHUNK).is_empty());
        assert_eq!(calls_to(&log, NEXT_CHUNK).len(), 3);
    }

    #[test]
    fn full_data_and_snam_use_offsets_from_this() {
        let (log, this, _) = run(vec![CHUNK_FULL, CHUNK_DATA, CHUNK_SNAM], 0, 0);
        assert_eq!(
            calls_to(&log, LOAD_FULL_NAME),
            vec![vec![this - 0x14, FILE]]
        );
        assert_eq!(
            calls_to(&log, TES_FORM_LOAD_DATA),
            vec![vec![this, FILE, this + 0x5c, 1]]
        );
        let word = calls_to(&log, GET_CHUNK_WORD);
        assert_eq!(word.len(), 1);
        assert_eq!(word[0][0], FILE);
        assert_eq!(
            calls_to(&log, SET_SOUND),
            vec![vec![this - 0x14, 0x0bad_f00d]]
        );
    }

    #[test]
    fn the_model_chunks_go_to_the_model_component() {
        let kinds = vec![CHUNK_MODL, CHUNK_MODD, CHUNK_MOSD, CHUNK_MODT];
        let (log, this, _) = run(kinds, 0, 0);
        assert_eq!(
            calls_to(&log, LOAD_MODEL_CHUNK),
            vec![vec![this + 0x30, FILE]; 4]
        );
    }

    #[test]
    fn the_destruction_chunks_go_to_the_destructible_component() {
        let (log, this, _) = run(vec![CHUNK_DEST, CHUNK_DSTD], 0, 0);
        assert_eq!(
            calls_to(&log, LOAD_DESTRUCTIBLE_CHUNK),
            vec![vec![this - 8, FILE]; 2]
        );
    }

    #[test]
    fn a_this_of_0x14_counts_as_a_null_object_for_the_components() {
        let (log, _, _) = run(vec![CHUNK_MODL, CHUNK_DEST], 0x14, 0);
        assert_eq!(calls_to(&log, LOAD_MODEL_CHUNK), vec![vec![0, FILE]]);
        assert_eq!(calls_to(&log, LOAD_DESTRUCTIBLE_CHUNK), vec![vec![0, FILE]]);
    }

    #[test]
    fn a_texture_swap_chunk_is_unpacked_from_a_buffer_of_its_size() {
        let (log, this, _) = run(vec![CHUNK_MODS], 0, 20);
        let data = calls_to(&log, GET_CHUNK_DATA);
        assert_eq!(data.len(), 1);
        assert_eq!((data[0][0], data[0][2]), (FILE, 20));
        assert_eq!(
            calls_to(&log, UNPACK_TEXTURE_SWAP),
            vec![vec![this + 0x30, data[0][1], 20, 7]]
        );
    }
}

#[cfg(test)]
mod b016_tests {
    use super::*;

    type CallLog = [(u32, Vec<u32>)];

    fn calls(log: &CallLog, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        for address in [
            FORM_TEST_4013E0,
            DESTRUCTIBLE_INIT_ITEM,
            TEXTURE_SWAP_INIT_ITEM,
            TES_FORM_ADD_COMPILE_INDEX,
            TES_FORM_SET_FLAG_484AB0,
            WARNING,
            LOOKUP_FORM_BY_ID,
            RT_DYNAMIC_CAST,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(TES_FORM_GET_FILE, |_, _| 0x0f11_e000u32.into_ret());
        e.register(FORM_ID_84E3A0, |_, _| 0x0abcu32.into_ret());
        e.register(0x0012_3130, |_, _| 0x0ed1_0000u32.into_ret());
        e
    }

    fn form(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x200);
        e.mem.set_u32(vtable + SLOT_GET_EDITOR_ID, 0x0012_3130);
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this, vtable);
        this
    }

    #[test]
    fn init_item_initialises_components_and_resolves_the_sound() {
        let mut e = engine();
        e.register(LOOKUP_FORM_BY_ID, |_, a| (a[0] + 0x1000).into_ret());
        e.register(RT_DYNAMIC_CAST, |_, a| a[0].into_ret());
        let this = form(&mut e);
        e.mem.set_u32(this + 0x58, 0x77);
        e.call_log = Some(vec![]);
        e.call(0x004f_bb70, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, DESTRUCTIBLE_INIT_ITEM),
            vec![vec![this - 8, this]]
        );
        assert_eq!(
            calls(&log, TEXTURE_SWAP_INIT_ITEM),
            vec![vec![this + 0x30, this]]
        );
        assert_eq!(
            calls(&log, RT_DYNAMIC_CAST),
            vec![vec![0x1077, 0, CAST_SOURCE_TYPE, CAST_TARGET_TYPE, 0]]
        );
        assert_eq!(e.mem.u32(this + 0x58), 0x1077);
        assert!(calls(&log, WARNING).is_empty());
        assert_eq!(calls(&log, TES_FORM_SET_FLAG_484AB0), vec![vec![this, 1]]);
    }

    #[test]
    fn init_item_warns_or_skips() {
        let mut e = engine();
        let this = form(&mut e);
        e.mem.set_u32(this + 0x58, 0x77);
        e.call_log = Some(vec![]);
        e.call(0x004f_bb70, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, WARNING),
            vec![vec![LOOPING_SOUND_WARNING, 0x77, 0x0ed1_0000, 0x0abc]]
        );
        e.register(FORM_TEST_4013E0, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x004f_bb70, &args![this]);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, DESTRUCTIBLE_INIT_ITEM).is_empty());
        assert!(calls(&log, TES_FORM_SET_FLAG_484AB0).is_empty());
    }

    #[test]
    fn init_item_with_this_at_0x14_passes_null_forms() {
        let mut e = engine();
        e.map(0, 0x1000);
        e.call_log = Some(vec![]);
        e.call(0x004f_bb70, &args![0x14u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, DESTRUCTIBLE_INIT_ITEM), vec![vec![0x0c, 0]]);
        assert_eq!(calls(&log, TEXTURE_SWAP_INIT_ITEM), vec![vec![0x44, 0]]);
    }

    #[test]
    fn sound_setter_stores_at_0x6c() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x80);
        e.call(0x004f_bf00, &args![this, 9u32]);
        assert_eq!(e.mem.u32(this + 0x6c), 9);
    }
}

#[cfg(test)]
mod tests_n036 {
    use super::*;

    fn run(sound: u32, swap: bool) -> (u32, Vec<(u32, Vec<u32>)>) {
        let mut e = Engine::new();
        for addr in [
            FORM_START_FORM,
            FORM_CLOSE_FORM,
            FULL_NAME_SAVE,
            MODEL_TEXTURE_SWAP_SAVE,
            DESTRUCTIBLE_SAVE,
            SWAP_DATA_BYTE,
            FORM_SAVE_DATA,
            ADD_CHUNK_WORD,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(FORM_ID, |_, a| (a[0] + 1).into_ret());
        e.register_double(SWAP_ON_SAVE, move |_, _| u32::from(swap).into_ret());
        let object = e.mem.alloc(0x80);
        let this = object + 0x14;
        e.mem.set_u32(this + 0x58, sound);
        e.call_log = Some(vec![]);
        e.call(0x004f_bf20, &args![this]);
        let log = e.call_log.take().unwrap();
        (
            this,
            log.into_iter().filter(|c| c.0 != SWAP_ON_SAVE).collect(),
        )
    }

    #[test]
    fn save_writes_every_part_in_order() {
        let (this, log) = run(0x9000, true);
        assert_eq!(
            log[1..],
            [
                (FORM_START_FORM, vec![this]),
                (FULL_NAME_SAVE, vec![this - 0x14]),
                (
                    MODEL_TEXTURE_SWAP_SAVE,
                    vec![this + 0x30, CHUNK_MODL, CHUNK_MODT, CHUNK_MODS]
                ),
                (DESTRUCTIBLE_SAVE, vec![this - 8]),
                (SWAP_DATA_BYTE, vec![this + 0x5c]),
                (FORM_SAVE_DATA, vec![this, this + 0x5c, 1]),
                (SWAP_DATA_BYTE, vec![this + 0x5c]),
                (FORM_ID, vec![0x9000]),
                (ADD_CHUNK_WORD, vec![CHUNK_SNAM, 0x9001]),
                (FORM_CLOSE_FORM, vec![this]),
            ]
        );
    }

    #[test]
    fn save_without_sound_or_swaps() {
        let (this, log) = run(0, false);
        assert_eq!(
            log[5..],
            [
                (FORM_SAVE_DATA, vec![this, this + 0x5c, 1]),
                (FORM_CLOSE_FORM, vec![this]),
            ]
        );
    }
}
