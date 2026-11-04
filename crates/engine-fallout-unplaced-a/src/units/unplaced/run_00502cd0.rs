//! `(unplaced)/run_00502cd0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Byte-swaps the four bytes at an address in place (`00401080`, cdecl:
/// the address, a flag byte).
const SWAP_4_BYTES: u32 = 0x0040_1080;

/// `TESFile::GetTESForm` (Xbox PDB name of `00472660`; `ECX` = file): the form
/// type of the record, in `AL`.
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
/// `00401680` (`ECX` = file): the byte at `TESFile` `+0x299`, which the chunk
/// readers use to decide whether the record's words are swapped.
const SWAP_FLAG: u32 = 0x0040_1680;
/// `TESFile::GetChunkData` (Xbox PDB overload `_ov3`, `00472890`): `thiscall`
/// (file; buffer, size).
const GET_CHUNK_DATA: u32 = 0x0047_2890;
/// `TESFile::GetChunkData` (Xbox PDB name of `004727f0`; `ECX` = file, the
/// address of a word): reads the chunk into the word.
const GET_CHUNK_WORD: u32 = 0x0047_27f0;
/// `00403570` (`ECX` = file): a 16-bit value (the result is in `AX`) that the
/// texture swap unpacking takes as its last argument.
const FILE_WORD: u32 = 0x0040_3570;
/// The component loader `00487050` (`cdecl`: component, file) for `FULL`.
const LOAD_FULL_NAME: u32 = 0x0048_7050;
/// `TESModel::LoadModelChunk` (Xbox PDB name of `004892d0`; `cdecl`: model,
/// file).
const LOAD_MODEL_CHUNK: u32 = 0x0048_92d0;
/// `TESModelTextureSwap::UnpackTextureSwapChunkData` (Xbox PDB name of
/// `0048a7b0`; `ECX` = the model at `+0x3c`, buffer, size, a 16-bit value).
const UNPACK_TEXTURE_SWAP: u32 = 0x0048_a7b0;
/// `BGSDestructibleObjectForm::LoadChunk` (Xbox PDB name of `004781e0`;
/// `cdecl`: destructible component, file).
const LOAD_DESTRUCTIBLE_CHUNK: u32 = 0x0047_81e0;
/// `TESTexture::LoadTextureChunk` (Xbox PDB name of `0048e3d0`; `cdecl`:
/// texture component, file).
const LOAD_TEXTURE_CHUNK: u32 = 0x0048_e3d0;
/// `0047a290` (`cdecl`: component, file), the loader of the `MICO` chunk.
const LOAD_MICO: u32 = 0x0047_a290;
/// `TESForm::LoadData` (Xbox PDB name of `004861f0`; `ECX` = form, file,
/// destination, byte count).
const TES_FORM_LOAD_DATA: u32 = 0x0048_61f0;
/// `00503240` (`ECX` = the `DAT2` block at `+0xb0`): the swap routine for it.
const SWAP_DAT2: u32 = 0x0050_3240;
/// `00923000` (`ECX` = this): answers `this + 0xd4`.
const SUB_OBJECT_ACCESSOR: u32 = 0x0092_3000;
/// `00470470` (`ECX` = the sub-object `00923000` answers, no stack word).
const SUB_OBJECT_PREPARE: u32 = 0x0047_0470;
/// `00484ab0` (`ECX` = this, one stack word, 0 here).
const FORM_PREPARE: u32 = 0x0048_4ab0;
/// `00905820` (`ECX` = the sub-object `00923000` answers, one stack word: the
/// address of the word the `RCIL` chunk held).
const SUB_OBJECT_ADD: u32 = 0x0090_5820;
/// `006ecd40` (`ECX` = component, one stack word); the engine map names it
/// `CombatProcedureAttackMelee::Initialize` because the linker folded the
/// identical code.
const COMPONENT_SET_WORD: u32 = 0x006e_cd40;
/// `0048cd90` (Xbox PDB `TESScriptableForm::InitItem`; `ECX` = the component
/// at `+0x9c`, one stack word: the form).
const SCRIPTABLE_INIT_ITEM: u32 = 0x0048_cd90;
/// `00403550` (`ECX` = component, one stack word).
const COMPONENT_SET_OTHER_WORD: u32 = 0x0040_3550;
/// `004037f0` (`ECX` = a string, two stack words: the text and a length).
const SET_STRING: u32 = 0x0040_37f0;

/// The form type of the record this loader accepts (`0x29`).
const FORM_TYPE: u8 = 0x29;

/// Chunk ids, as the little-endian words of their four letters.
const CHUNK_FULL: u32 = 0x4c4c_5546;
const CHUNK_MODL: u32 = 0x4c44_4f4d;
const CHUNK_MODD: u32 = 0x4444_4f4d;
const CHUNK_MOSD: u32 = 0x4453_4f4d;
const CHUNK_MODT: u32 = 0x5444_4f4d;
const CHUNK_MODS: u32 = 0x5344_4f4d;
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_DATA: u32 = 0x4154_4144;
const CHUNK_DAT2: u32 = 0x3254_4144;
const CHUNK_DSTD: u32 = 0x4454_5344;
const CHUNK_DEST: u32 = 0x5453_4544;
const CHUNK_SCRI: u32 = 0x4952_4353;
const CHUNK_RCIL: u32 = 0x4c49_4352;
const CHUNK_ICON: u32 = 0x4e4f_4349;
const CHUNK_MICO: u32 = 0x4f43_494d;
const CHUNK_ONAM: u32 = 0x4d41_4e4f;
const CHUNK_QNAM: u32 = 0x4d41_4e51;
const CHUNK_YNAM: u32 = 0x4d41_4e59;
const CHUNK_ZNAM: u32 = 0x4d41_4e5a;

/// The object's virtual slot called with the file for an `OBND` chunk.
const SLOT_LOAD_BOUNDS: u32 = 0xe0;
/// The object's virtual slot called with the text of an `EDID` chunk.
const SLOT_SET_EDITOR_ID: u32 = 0x134;
/// The most an `EDID` chunk is read with.
const EDITOR_ID_MAXIMUM: u32 = 0x200;

/// Offsets from `this` of the components the chunks go to.
const FULL_NAME_OFFSET: u32 = 0x30;
const MODEL_OFFSET: u32 = 0x3c;
const TEXTURE_OFFSET: u32 = 0x5c;
const MICO_OFFSET: u32 = 0x68;
const DESTRUCTIBLE_OFFSET: u32 = 0x88;
const OTHER_WORD_OFFSET: u32 = 0x90;
const SCRIPT_OFFSET: u32 = 0x9c;
const DATA_OFFSET: u32 = 0xa8;
const DAT2_OFFSET: u32 = 0xb0;
const STRING_ONAM_OFFSET: u32 = 0xc4;
const STRING_QNAM_OFFSET: u32 = 0xcc;

// Translated from 00503210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Byte-swaps the word at `this` in place: `00401080` with `this` and a
/// zero flag byte. The quest target loader (`00610120`) calls it.
pub fn fn_00503210(e: &mut Engine, this: Ptr) {
    e.call(SWAP_4_BYTES, &args![this, 0u32]);
}

/// A component's address, or null when the object is null (the compiler's
/// `this ? &this->component : 0`).
fn component(object: u32, offset: u32) -> u32 {
    if object == 0 {
        0
    } else {
        object + offset
    }
}

/// `_alloca_probe_16`: the stack buffer is the size rounded up to 16 bytes.
fn stack_size(size: u32) -> u32 {
    size.wrapping_add(15) & !15
}

/// A word the chunk is read into (starts at 0).
fn read_chunk_word(e: &mut Engine, file: Ptr) -> u32 {
    e.with_stack(4, |e, word| {
        e.mem.set_u32(word.addr(), 0);
        e.call(GET_CHUNK_WORD, &args![file, word]);
        e.mem.u32(word.addr())
    })
}

/// Reads the whole chunk into a stack buffer and gives it to the string at
/// `string` (`GetChunkData(buffer, 0)`, then `004037f0(buffer, 0)`).
fn string_from_chunk(e: &mut Engine, file: Ptr, string: u32) {
    let size = e.call(CHUNK_SIZE, &args![file]).u32();
    e.with_stack(stack_size(size), |e, buffer| {
        e.call(GET_CHUNK_DATA, &args![file, buffer, 0u32]);
        e.call(SET_STRING, &args![string, buffer, 0u32]);
    });
}

// Translated from 00502cd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00502cd0 - the /GS stack cookie check (00ec408c) is not translated.
/// Unnamed in the engine map (`thiscall`, `RET 4`, `file`): the `Load` slot
/// (`+0x20`) of the virtual table `01026064`, the one the form constructor
/// `005026b0` (`TESAmmo::TESAmmo`, form type `0x29`) stores. Answers false when
/// the record is not of form type `0x29`; otherwise it runs `00923000` and
/// `00470470` on the sub-object at `this + 0xd4`, `TESForm::LoadForm`, `00484ab0`
/// with 0, then reads every chunk of the record and answers true:
/// - `FULL`: `00487050` on `this + 0x30` (null when `this` is null);
/// - `MODL`, `MODD`, `MOSD`, `MODT`: `TESModel::LoadModelChunk` on `this + 0x3c`;
/// - `MODS`: read into a stack buffer of the chunk's size and unpacked as a
///   texture swap on `this + 0x3c`;
/// - `ICON`: `TESTexture::LoadTextureChunk` on `this + 0x5c`; `MICO`: `0047a290`
///   on `this + 0x68`;
/// - `DEST`, `DSTD`: `BGSDestructibleObjectForm::LoadChunk` on `this + 0x88`;
/// - `OBND`: virtual slot `0xe0` with the file; `EDID`: read into a stack buffer
///   (at most 0x200 bytes) and passed to virtual slot `0x134`;
/// - `DATA`: `TESForm::LoadData` into `this + 0xa8` (8 bytes), and when the
///   file's swap flag is set `00503210` on it (checked after the chunk's own
///   handling);
/// - `DAT2`: 0x14 bytes into `this + 0xb0`, then `00503240` on it when the swap
///   flag is set;
/// - `SCRI`: the chunk's word goes to `006ecd40` on `this + 0x9c`, then
///   `TESScriptableForm::InitItem` (`0048cd90`) runs on it with `this`;
/// - `RCIL`: the chunk's word goes to `00905820` on the sub-object at
///   `this + 0xd4`;
/// - `YNAM` and `ZNAM`: the chunk's word goes to `006ecd40` and `00403550` on
///   `this + 0x90`;
/// - `ONAM` and `QNAM`: the chunk is read into a stack buffer and handed to
///   `004037f0` on the strings at `this + 0xc4` and `this + 0xcc`.
///
/// Other chunks are skipped. The stack cookie check is not translated.
pub fn fn_00502cd0(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let object = this.addr();
    if e.call(GET_TES_FORM, &args![file]).u8() != FORM_TYPE {
        return false;
    }
    let sub_object = e.call(SUB_OBJECT_ACCESSOR, &args![this]).u32();
    e.call(SUB_OBJECT_PREPARE, &args![sub_object]);
    e.call(TES_FORM_LOAD_FORM, &args![this, file]);
    e.call(FORM_PREPARE, &args![this, 0u32]);
    loop {
        let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            CHUNK_FULL => {
                let target = component(object, FULL_NAME_OFFSET);
                e.call(LOAD_FULL_NAME, &args![target, file]);
            }
            CHUNK_MODL | CHUNK_MODD | CHUNK_MOSD | CHUNK_MODT => {
                let model = component(object, MODEL_OFFSET);
                e.call(LOAD_MODEL_CHUNK, &args![model, file]);
            }
            CHUNK_DAT2 => {
                e.call(GET_CHUNK_DATA, &args![file, object + DAT2_OFFSET, 0x14u32]);
                if e.call(SWAP_FLAG, &args![file]).bool() {
                    e.call(SWAP_DAT2, &args![object + DAT2_OFFSET]);
                }
            }
            CHUNK_DATA => {
                e.call(
                    TES_FORM_LOAD_DATA,
                    &args![this, file, object + DATA_OFFSET, 8u32],
                );
            }
            CHUNK_EDID => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                e.with_stack(stack_size(size), |e, buffer| {
                    e.call(GET_CHUNK_DATA, &args![file, buffer, EDITOR_ID_MAXIMUM]);
                    e.vcall(object, SLOT_SET_EDITOR_ID, &args![buffer]);
                });
            }
            CHUNK_OBND => {
                e.vcall(object, SLOT_LOAD_BOUNDS, &args![file]);
            }
            CHUNK_DSTD | CHUNK_DEST => {
                let target = component(object, DESTRUCTIBLE_OFFSET);
                e.call(LOAD_DESTRUCTIBLE_CHUNK, &args![target, file]);
            }
            CHUNK_SCRI => {
                let word = read_chunk_word(e, file);
                e.call(COMPONENT_SET_WORD, &args![object + SCRIPT_OFFSET, word]);
                e.call(SCRIPTABLE_INIT_ITEM, &args![object + SCRIPT_OFFSET, this]);
            }
            CHUNK_RCIL => {
                let word = read_chunk_word(e, file);
                // The game copies the word to a second local before handing
                // its address on.
                e.with_stack(4, |e, copy| {
                    e.mem.set_u32(copy.addr(), word);
                    let sub_object = e.call(SUB_OBJECT_ACCESSOR, &args![this]).u32();
                    e.call(SUB_OBJECT_ADD, &args![sub_object, copy]);
                });
            }
            CHUNK_ICON => {
                let target = component(object, TEXTURE_OFFSET);
                e.call(LOAD_TEXTURE_CHUNK, &args![target, file]);
            }
            CHUNK_MICO => {
                let target = component(object, MICO_OFFSET);
                e.call(LOAD_MICO, &args![target, file]);
            }
            CHUNK_MODS => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                e.with_stack(stack_size(size), |e, buffer| {
                    e.call(GET_CHUNK_DATA, &args![file, buffer, size]);
                    let word = e.call(FILE_WORD, &args![file]).u16();
                    e.call(
                        UNPACK_TEXTURE_SWAP,
                        &args![object + MODEL_OFFSET, buffer, size, u32::from(word)],
                    );
                });
            }
            CHUNK_ONAM => string_from_chunk(e, file, object + STRING_ONAM_OFFSET),
            CHUNK_QNAM => string_from_chunk(e, file, object + STRING_QNAM_OFFSET),
            CHUNK_YNAM => {
                let word = read_chunk_word(e, file);
                e.call(COMPONENT_SET_WORD, &args![object + OTHER_WORD_OFFSET, word]);
            }
            CHUNK_ZNAM => {
                let word = read_chunk_word(e, file);
                e.call(
                    COMPONENT_SET_OTHER_WORD,
                    &args![object + OTHER_WORD_OFFSET, word],
                );
            }
            _ => {}
        }
        if chunk == CHUNK_DATA && e.call(SWAP_FLAG, &args![file]).bool() {
            fn_00503210(e, Ptr::new(object + DATA_OFFSET));
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

// Translated from 00503240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX`, plain `RET`; called by `00502cd0`, two `Save`s and two
/// `Load`s): reverses the byte order of the words at +8 and +0x10 (`00401080`, each with
/// the byte 0 the body zeroes).
pub fn fn_00503240(e: &mut Engine, this: Ptr) {
    e.call(SWAP_4_BYTES, &args![this.addr() + 8, 0u32]);
    e.call(SWAP_4_BYTES, &args![this.addr() + 0x10, 0u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0050_3240, fn_00503240(Ptr)),
        entry!(0x00503210, fn_00503210(Ptr)),
        entry!(0x00502cd0, fn_00502cd0(Ptr, Ptr) -> bool),
    ]
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
    const CHUNK_WORD: u32 = 0x0bad_f00d;

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

    /// What a run is given.
    struct Record {
        chunks: Vec<u32>,
        form_type: u32,
        null_this: bool,
        chunk_size: u32,
        swap: bool,
    }

    impl Record {
        fn of(chunks: Vec<u32>) -> Record {
            Record {
                chunks,
                form_type: 0x29,
                null_this: false,
                chunk_size: 0,
                swap: false,
            }
        }
    }

    /// Runs the loader on a record. Returns the call log (without the top-level
    /// call), the address of `this` and the result.
    fn run(record: Record) -> (CallLog, u32, bool) {
        let mut e = Engine::new();
        let position = Rc::new(Cell::new(0usize));
        let (list, at) = (record.chunks.clone(), position.clone());
        e.register_double(GET_TES_CHUNK, move |_, _| {
            ret(list.get(at.get()).copied().unwrap_or(0))
        });
        let total = record.chunks.len();
        e.register_double(NEXT_CHUNK, move |_, _| {
            position.set(position.get() + 1);
            ret(u32::from(position.get() < total))
        });
        let form_type = record.form_type;
        e.register_double(GET_TES_FORM, move |_, _| ret(0x7700 | form_type));
        for addr in [
            TES_FORM_LOAD_FORM,
            GET_CHUNK_DATA,
            LOAD_FULL_NAME,
            LOAD_MODEL_CHUNK,
            UNPACK_TEXTURE_SWAP,
            LOAD_DESTRUCTIBLE_CHUNK,
            LOAD_TEXTURE_CHUNK,
            LOAD_MICO,
            TES_FORM_LOAD_DATA,
            SWAP_DAT2,
            SWAP_4_BYTES,
            SUB_OBJECT_PREPARE,
            FORM_PREPARE,
            SUB_OBJECT_ADD,
            COMPONENT_SET_WORD,
            SCRIPTABLE_INIT_ITEM,
            COMPONENT_SET_OTHER_WORD,
            SET_STRING,
            TARGET_LOAD_BOUNDS,
            TARGET_SET_EDITOR_ID,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(SUB_OBJECT_ACCESSOR, |_, a| ret(a[0] + 0xd4));
        let chunk_size = record.chunk_size;
        e.register_double(CHUNK_SIZE, move |_, _| ret(chunk_size));
        let swap = record.swap;
        e.register_double(SWAP_FLAG, move |_, _| ret(0x5500 | u32::from(swap)));
        e.register(FILE_WORD, |_, _| ret(0xabcd_0007));
        e.register(GET_CHUNK_WORD, |e, a| {
            e.mem.set_u32(a[1], CHUNK_WORD);
            Ret::default()
        });
        let mut table = vec![0u32; 0x4e];
        table[0xe0 / 4] = TARGET_LOAD_BOUNDS;
        table[0x134 / 4] = TARGET_SET_EDITOR_ID;
        e.put_vtable(OBJECT_TABLE, &table);
        let this = if record.null_this {
            0
        } else {
            let object = e.mem.alloc(0xe0);
            e.mem.set_u32(object, OBJECT_TABLE);
            object
        };
        e.call_log = Some(vec![]);
        let result = e.call(0x00502cd0, &args![this, FILE]).bool();
        let log = e.call_log.take().unwrap();
        (log.into_iter().skip(1).collect(), this, result)
    }

    #[test]
    fn a_record_of_another_form_type_is_refused() {
        let mut record = Record::of(vec![CHUNK_FULL]);
        record.form_type = 0x2a;
        let (log, _, result) = run(record);
        assert!(!result);
        assert_eq!(log, vec![(GET_TES_FORM, vec![FILE])]);
    }

    #[test]
    fn the_header_is_prepared_before_the_chunks() {
        let (log, this, result) = run(Record::of(vec![]));
        assert!(result);
        assert_eq!(log[0], (GET_TES_FORM, vec![FILE]));
        assert_eq!(log[1], (SUB_OBJECT_ACCESSOR, vec![this]));
        assert_eq!(log[2], (SUB_OBJECT_PREPARE, vec![this + 0xd4]));
        assert_eq!(log[3], (TES_FORM_LOAD_FORM, vec![this, FILE]));
        assert_eq!(log[4], (FORM_PREPARE, vec![this, 0]));
        assert_eq!(log[5], (GET_TES_CHUNK, vec![FILE]));
        assert_eq!(log.len(), 6);
    }

    #[test]
    fn the_virtual_chunks_go_to_the_objects_slots() {
        let mut record = Record::of(vec![CHUNK_OBND, CHUNK_EDID, 0x5555_5555]);
        record.chunk_size = 5;
        let (log, this, _) = run(record);
        assert_eq!(calls_to(&log, TARGET_LOAD_BOUNDS), vec![vec![this, FILE]]);
        let data = calls_to(&log, GET_CHUNK_DATA);
        assert_eq!(data.len(), 1);
        assert_eq!((data[0][0], data[0][2]), (FILE, 0x200));
        assert_eq!(
            calls_to(&log, TARGET_SET_EDITOR_ID),
            vec![vec![this, data[0][1]]]
        );
        assert!(calls_to(&log, LOAD_MODEL_CHUNK).is_empty());
        assert_eq!(calls_to(&log, NEXT_CHUNK).len(), 3);
    }

    #[test]
    fn the_components_get_their_chunks() {
        let chunks = vec![
            CHUNK_FULL, CHUNK_MODL, CHUNK_MODD, CHUNK_MOSD, CHUNK_MODT, CHUNK_ICON, CHUNK_MICO,
            CHUNK_DSTD, CHUNK_DEST,
        ];
        let (log, this, _) = run(Record::of(chunks));
        assert_eq!(
            calls_to(&log, LOAD_FULL_NAME),
            vec![vec![this + 0x30, FILE]]
        );
        assert_eq!(
            calls_to(&log, LOAD_MODEL_CHUNK),
            vec![vec![this + 0x3c, FILE]; 4]
        );
        assert_eq!(
            calls_to(&log, LOAD_TEXTURE_CHUNK),
            vec![vec![this + 0x5c, FILE]]
        );
        assert_eq!(calls_to(&log, LOAD_MICO), vec![vec![this + 0x68, FILE]]);
        assert_eq!(
            calls_to(&log, LOAD_DESTRUCTIBLE_CHUNK),
            vec![vec![this + 0x88, FILE]; 2]
        );
    }

    #[test]
    fn a_null_object_has_null_components() {
        let mut record = Record::of(vec![
            CHUNK_FULL, CHUNK_MODL, CHUNK_ICON, CHUNK_MICO, CHUNK_DEST,
        ]);
        record.null_this = true;
        let (log, _, _) = run(record);
        assert_eq!(calls_to(&log, LOAD_FULL_NAME), vec![vec![0, FILE]]);
        assert_eq!(calls_to(&log, LOAD_MODEL_CHUNK), vec![vec![0, FILE]]);
        assert_eq!(calls_to(&log, LOAD_TEXTURE_CHUNK), vec![vec![0, FILE]]);
        assert_eq!(calls_to(&log, LOAD_MICO), vec![vec![0, FILE]]);
        assert_eq!(calls_to(&log, LOAD_DESTRUCTIBLE_CHUNK), vec![vec![0, FILE]]);
    }

    #[test]
    fn a_texture_swap_chunk_is_unpacked_from_a_buffer_of_its_size() {
        let mut record = Record::of(vec![CHUNK_MODS]);
        record.chunk_size = 20;
        let (log, this, _) = run(record);
        let data = calls_to(&log, GET_CHUNK_DATA);
        assert_eq!(data.len(), 1);
        assert_eq!((data[0][0], data[0][2]), (FILE, 20));
        assert_eq!(
            calls_to(&log, UNPACK_TEXTURE_SWAP),
            vec![vec![this + 0x3c, data[0][1], 20, 7]]
        );
    }

    #[test]
    fn data_swaps_only_when_the_file_asks_for_it() {
        let (log, this, _) = run(Record::of(vec![CHUNK_DATA]));
        assert_eq!(
            calls_to(&log, TES_FORM_LOAD_DATA),
            vec![vec![this, FILE, this + 0xa8, 8]]
        );
        assert!(calls_to(&log, SWAP_4_BYTES).is_empty());

        let mut record = Record::of(vec![CHUNK_DATA]);
        record.swap = true;
        let (log, this, _) = run(record);
        assert_eq!(calls_to(&log, SWAP_4_BYTES), vec![vec![this + 0xa8, 0]]);
        // The flag is only looked at after a DATA chunk.
        let mut record = Record::of(vec![CHUNK_OBND]);
        record.swap = true;
        let (log, _, _) = run(record);
        assert!(calls_to(&log, SWAP_FLAG).is_empty());
        assert!(calls_to(&log, SWAP_4_BYTES).is_empty());
    }

    #[test]
    fn dat2_reads_0x14_bytes_and_swaps_on_request() {
        let (log, this, _) = run(Record::of(vec![CHUNK_DAT2]));
        assert_eq!(
            calls_to(&log, GET_CHUNK_DATA),
            vec![vec![FILE, this + 0xb0, 0x14]]
        );
        assert!(calls_to(&log, SWAP_DAT2).is_empty());

        let mut record = Record::of(vec![CHUNK_DAT2]);
        record.swap = true;
        let (log, this, _) = run(record);
        assert_eq!(calls_to(&log, SWAP_DAT2), vec![vec![this + 0xb0]]);
    }

    #[test]
    fn the_word_chunks_go_to_their_components() {
        let (log, this, _) = run(Record::of(vec![
            CHUNK_SCRI, CHUNK_YNAM, CHUNK_ZNAM, CHUNK_RCIL,
        ]));
        assert_eq!(
            calls_to(&log, COMPONENT_SET_WORD),
            vec![vec![this + 0x9c, CHUNK_WORD], vec![this + 0x90, CHUNK_WORD]]
        );
        assert_eq!(
            calls_to(&log, SCRIPTABLE_INIT_ITEM),
            vec![vec![this + 0x9c, this]]
        );
        assert_eq!(
            calls_to(&log, COMPONENT_SET_OTHER_WORD),
            vec![vec![this + 0x90, CHUNK_WORD]]
        );
        let add = calls_to(&log, SUB_OBJECT_ADD);
        assert_eq!(add.len(), 1);
        assert_eq!(add[0][0], this + 0xd4);
    }

    #[test]
    fn the_rcil_word_is_handed_on_by_address() {
        let mut e = Engine::new();
        e.register(GET_TES_FORM, |_, _| ret(0x29));
        for addr in [SUB_OBJECT_PREPARE, TES_FORM_LOAD_FORM, FORM_PREPARE] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(SUB_OBJECT_ACCESSOR, |_, a| ret(a[0] + 0xd4));
        e.register(GET_CHUNK_WORD, |e, a| {
            e.mem.set_u32(a[1], 0x1234_5678);
            Ret::default()
        });
        let seen = Rc::new(Cell::new(0u32));
        let seen_in_double = seen.clone();
        e.register_double(SUB_OBJECT_ADD, move |e, a| {
            seen_in_double.set(e.mem.u32(a[1]));
            Ret::default()
        });
        let chunks = Rc::new(Cell::new(0u32));
        let chunk_in_double = chunks.clone();
        e.register_double(GET_TES_CHUNK, move |_, _| {
            let next = chunk_in_double.get();
            chunk_in_double.set(next + 1);
            ret(if next == 0 { CHUNK_RCIL } else { 0 })
        });
        e.register(NEXT_CHUNK, |_, _| ret(1));
        e.call(0x00502cd0, &args![0x5000u32, FILE]);
        assert_eq!(seen.get(), 0x1234_5678);
    }

    #[test]
    fn the_name_chunks_go_to_their_strings_from_a_buffer_of_their_size() {
        let mut record = Record::of(vec![CHUNK_ONAM, CHUNK_QNAM]);
        record.chunk_size = 33;
        let (log, this, _) = run(record);
        let data = calls_to(&log, GET_CHUNK_DATA);
        assert_eq!(data.len(), 2);
        assert!(data.iter().all(|call| call[0] == FILE && call[2] == 0));
        assert_eq!(
            calls_to(&log, SET_STRING),
            vec![
                vec![this + 0xc4, data[0][1], 0],
                vec![this + 0xcc, data[1][1], 0]
            ]
        );
    }

    #[test]
    fn the_swap_helper_swaps_this_with_a_zero_flag() {
        let mut e = Engine::new();
        e.register(SWAP_4_BYTES, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0050_3210, &[0x4000]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(SWAP_4_BYTES, vec![0x4000, 0])]);
    }
}

#[cfg(test)]
mod n075_tests {
    use super::*;

    #[test]
    fn fn_00503240_swaps_the_words_at_8_and_0x10() {
        let mut e = Engine::new();
        e.register(SWAP_4_BYTES, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0050_3240, &args![0x1000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (SWAP_4_BYTES, vec![0x1008, 0]),
                (SWAP_4_BYTES, vec![0x1010, 0])
            ]
        );
    }
}
