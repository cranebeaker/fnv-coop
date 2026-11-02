//! `fallout shared/bgsstaticcollection.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

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
/// `00403570` (`ECX` = file): a 16-bit value (the result is in `AX`) that the
/// texture swap unpacking takes as its last argument.
const FILE_WORD: u32 = 0x0040_3570;
/// `TESModel::LoadModelChunk` (Xbox PDB name of `004892d0`; `cdecl`: model,
/// file).
const LOAD_MODEL_CHUNK: u32 = 0x0048_92d0;
/// `TESModelTextureSwap::UnpackTextureSwapChunkData` (Xbox PDB name of
/// `0048a7b0`; `ECX` = the model at `+0x30`, buffer, size, a 16-bit value).
const UNPACK_TEXTURE_SWAP: u32 = 0x0048_a7b0;

/// Chunk ids, as the little-endian words of their four letters.
const CHUNK_MODL: u32 = 0x4c44_4f4d;
const CHUNK_MODD: u32 = 0x4444_4f4d;
const CHUNK_MOSD: u32 = 0x4453_4f4d;
const CHUNK_MODT: u32 = 0x5444_4f4d;
const CHUNK_MODS: u32 = 0x5344_4f4d;
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_EDID: u32 = 0x4449_4445;

/// The object's virtual slot called with the file for an `OBND` chunk.
const SLOT_LOAD_BOUNDS: u32 = 0xe0;
/// The object's virtual slot called with the text of an `EDID` chunk.
const SLOT_SET_EDITOR_ID: u32 = 0x134;

/// `this + 0x30`: the model component the model and texture swap chunks go to.
const MODEL_OFFSET: u32 = 0x30;
/// The most an `EDID` chunk is read with.
const EDITOR_ID_MAXIMUM: u32 = 0x200;

// Translated from 004fe390 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004fe390 - the /GS stack cookie check (00ec408c) is not translated.
/// `BGSStaticCollection::Load` (Xbox PDB; `thiscall`, `RET 4`, `file`), answers
/// true. It reads the form header (`TESForm::LoadForm`) and then every chunk of
/// the record:
/// - `MODL`, `MODD`, `MOSD`, `MODT`: `TESModel::LoadModelChunk` on `this + 0x30`
///   (null when `this` is null);
/// - `MODS`: the chunk is read into a stack buffer of its size and unpacked as a
///   texture swap on `this + 0x30`;
/// - `OBND`: virtual slot `0xe0` with the file;
/// - `EDID`: read into a stack buffer (at most 0x200 bytes) and passed to
///   virtual slot `0x134`.
///
/// Other chunks are skipped. The stack cookie check is not translated.
pub fn bgs_static_collection_load(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let object = this.addr();
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
            CHUNK_MODL | CHUNK_MODD | CHUNK_MOSD | CHUNK_MODT => {
                let model = if object == 0 {
                    0
                } else {
                    object + MODEL_OFFSET
                };
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
            _ => {}
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x004fe390, bgs_static_collection_load(Ptr, Ptr) -> bool)]
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

    /// Runs the loader on a record of `chunks`. With `null_this` the object is
    /// null (only the chunks that do not touch its vtable can run); otherwise a
    /// real object holding the vtable at +0 is allocated. Returns the call log
    /// (without the top-level call), the address of `this` and the result.
    fn run(chunks: Vec<u32>, null_this: bool, chunk_size: u32) -> (CallLog, u32, bool) {
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
            TES_FORM_LOAD_FORM,
            GET_CHUNK_DATA,
            LOAD_MODEL_CHUNK,
            UNPACK_TEXTURE_SWAP,
            TARGET_LOAD_BOUNDS,
            TARGET_SET_EDITOR_ID,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register_double(CHUNK_SIZE, move |_, _| ret(chunk_size));
        e.register(FILE_WORD, |_, _| ret(0xabcd_0007));
        let mut table = vec![0u32; 0x4e];
        table[0xe0 / 4] = TARGET_LOAD_BOUNDS;
        table[0x134 / 4] = TARGET_SET_EDITOR_ID;
        e.put_vtable(OBJECT_TABLE, &table);
        let this = if null_this {
            0
        } else {
            let object = e.mem.alloc(0x80);
            e.mem.set_u32(object, OBJECT_TABLE);
            object
        };
        e.call_log = Some(vec![]);
        let result = e.call(0x004fe390, &args![this, FILE]).bool();
        let log = e.call_log.take().unwrap();
        (log.into_iter().skip(1).collect(), this, result)
    }

    #[test]
    fn the_header_is_read_before_the_chunks() {
        let (log, this, result) = run(vec![], false, 0);
        assert!(result);
        assert_eq!(log[0], (TES_FORM_LOAD_FORM, vec![this, FILE]));
        assert_eq!(log[1], (GET_TES_CHUNK, vec![FILE]));
        assert_eq!(log.len(), 2);
    }

    #[test]
    fn the_virtual_chunks_go_to_the_objects_slots() {
        let (log, this, _) = run(vec![CHUNK_OBND, CHUNK_EDID, 0x5555_5555], false, 5);
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
    fn the_model_chunks_go_to_the_model_component() {
        let kinds = vec![CHUNK_MODL, CHUNK_MODD, CHUNK_MOSD, CHUNK_MODT];
        let (log, this, _) = run(kinds, false, 0);
        assert_eq!(
            calls_to(&log, LOAD_MODEL_CHUNK),
            vec![vec![this + 0x30, FILE]; 4]
        );
    }

    #[test]
    fn a_null_object_has_a_null_model_component() {
        let (log, _, _) = run(vec![CHUNK_MODL, CHUNK_MODT], true, 0);
        assert_eq!(calls_to(&log, LOAD_MODEL_CHUNK), vec![vec![0, FILE]; 2]);
    }

    #[test]
    fn a_texture_swap_chunk_is_unpacked_from_a_buffer_of_its_size() {
        let (log, this, _) = run(vec![CHUNK_MODS], false, 20);
        let data = calls_to(&log, GET_CHUNK_DATA);
        assert_eq!(data.len(), 1);
        assert_eq!((data[0][0], data[0][2]), (FILE, 20));
        assert_eq!(
            calls_to(&log, UNPACK_TEXTURE_SWAP),
            vec![vec![this + 0x30, data[0][1], 20, 7]]
        );
    }

    #[test]
    fn a_texture_swap_chunk_does_not_check_for_a_null_object() {
        let (log, _, _) = run(vec![CHUNK_MODS], true, 4);
        let data = calls_to(&log, GET_CHUNK_DATA);
        assert_eq!(
            calls_to(&log, UNPACK_TEXTURE_SWAP),
            vec![vec![0x30, data[0][1], 4, 7]]
        );
    }
}
