//! `fallout shared/bgsdebris.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESFile::GetTESForm` (`00472660`, engine map): the record type byte.
const GET_TES_FORM: u32 = 0x0047_2660;
/// `TESForm::LoadForm` (`00485110`, engine map).
const TES_FORM_LOAD_FORM: u32 = 0x0048_5110;
/// `00484ab0` (`tesform.cpp`): `ECX` = the form, one word.
const TES_FORM_MARK_00484AB0: u32 = 0x0048_4ab0;
/// `TESFile::GetTESChunk` (`004726b0`, engine map): 0 at the end.
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `004726f0`: moves to the next chunk; false when there is none.
const NEXT_CHUNK: u32 = 0x0047_26f0;
/// `TESFile::GetChunkData_ov3` (`00472890`, engine map): buffer and size.
const GET_CHUNK_DATA: u32 = 0x0047_2890;
/// `00401660`: the current chunk's size.
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `00401000`: `operator new` (cdecl).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `00401030`: `operator delete` (cdecl).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `0044a670`: the length of a C string (cdecl).
const STRING_LENGTH: u32 = 0x0044_a670;
/// `004fa500`: adds one debris model (`ECX` = the debris; percentage,
/// model path, flags) and returns the new entry.
const ADD_DEBRIS_DATA: u32 = 0x004f_a500;
/// `TESTextureList::Load` (`00489940`, engine map): `ECX` = the list; the
/// file, the form and the model name.
const TES_TEXTURE_LIST_LOAD: u32 = 0x0048_9940;
/// `0084e3a0`: the form's id.
const FORM_ID: u32 = 0x0084_e3a0;
/// `"MODELS: No last debris data when loading texture list for debris '%s' (%08X)"`.
const NO_LAST_DATA_WARNING: u32 = 0x0102_4988;
/// `"UNKNOWN"`.
const UNKNOWN_NAME: u32 = 0x0101_5890;
/// The record type `BGSDebris::Load` accepts.
const FORM_TYPE_DEBRIS_RECORD: u32 = 0x52;

const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_DATA: u32 = 0x4154_4144;
const CHUNK_MODT: u32 = 0x5444_4f4d;

// Translated from 004f9f10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004f9f10 - the exception-unwinding frame (FS:[0]) and the /GS security-cookie check (00ec408c) are not translated; the __alloca_probe_16 (00ec5ec0) EDID buffer is a with_stack block of the chunk size rounded up to 16 bytes.
/// `BGSDebris::Load` (Xbox PDB): false unless the record type is 0x52;
/// then vtable slot 0x18, `TESForm::LoadForm`, `00484ab0(0)` and each chunk
/// of `file` (one stack word) until none is left:
/// - `EDID` (at most 0x200 bytes) to slot 0x134, `OBND` to slot 0xE0;
/// - `DATA`: the chunk read into a new block of its size; when it is not
///   empty its first byte (percentage), the C string after it and the byte
///   after the string (flags, 0 when past the end) go to `004fa500`, whose
///   result is the last debris entry; the block is freed;
/// - `MODT`: the texture list of the last entry (+0x8, with its model name
///   at +0x4) loads through `TESTextureList::Load`; without one the warning
///   is logged and a temporary list loads with `"UNKNOWN"` and is destroyed.
///
/// Returns true.
pub fn bgs_debris_load(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let base = this.addr();
    if e.call(GET_TES_FORM, &args![file]).u8() as u32 != FORM_TYPE_DEBRIS_RECORD {
        return false;
    }
    e.vcall(base, 0x18, &args![]);
    e.call(TES_FORM_LOAD_FORM, &args![this, file]);
    e.call(TES_FORM_MARK_00484AB0, &args![this, 0u32]);
    let mut last = 0u32;
    loop {
        let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            CHUNK_EDID => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                let reserved = size.wrapping_add(15) & !15;
                e.with_stack(reserved, |e, buffer| {
                    e.call(GET_CHUNK_DATA, &args![file, buffer, 0x200u32]);
                    e.vcall(base, 0x134, &args![buffer]);
                });
            }
            CHUNK_OBND => {
                e.vcall(base, 0xe0, &args![file]);
            }
            CHUNK_DATA => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                let block = e.call(OPERATOR_NEW, &args![size]).u32();
                e.call(GET_CHUNK_DATA, &args![file, block, size]);
                let end = block.wrapping_add(size);
                let mut cursor = block;
                if cursor < end {
                    let percentage = e.mem.u8(cursor);
                    cursor = cursor.wrapping_add(1);
                    let name = cursor;
                    let length = e.call(STRING_LENGTH, &args![cursor]).u32();
                    cursor = cursor.wrapping_add(length).wrapping_add(1);
                    let mut flags = 0u8;
                    if cursor < end {
                        flags = e.mem.u8(cursor);
                    }
                    last = e
                        .call(
                            ADD_DEBRIS_DATA,
                            &args![this, u32::from(percentage), name, u32::from(flags)],
                        )
                        .u32();
                }
                e.call(OPERATOR_DELETE, &args![block]);
            }
            CHUNK_MODT => {
                if last != 0 {
                    let name = e.mem.u32(last + 4);
                    e.call(TES_TEXTURE_LIST_LOAD, &args![last + 8, file, this, name]);
                } else {
                    let form_id = e.call(FORM_ID, &args![this]).u32();
                    let editor_id = e.vcall(base, 0x130, &args![]).u32();
                    e.call(
                        0x005b_5e40,
                        &args![NO_LAST_DATA_WARNING, editor_id, form_id],
                    );
                    e.with_stack(8, |e, list| {
                        e.call(0x0048_8fd0, &args![list]);
                        e.call(
                            TES_TEXTURE_LIST_LOAD,
                            &args![list, file, this, UNKNOWN_NAME],
                        );
                        e.call(0x0048_8ff0, &args![list]);
                    });
                }
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
    vec![entry!(0x004f_9f10, bgs_debris_load(Ptr, Ptr) -> bool)]
}

#[cfg(test)]
mod b028_tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    /// Chunk reader doubles over a list of (tag, data) chunks.
    fn load_engine(record_type: u32, chunks: Vec<(u32, Vec<u8>)>) -> Engine {
        let mut e = Engine::new();
        let list = Rc::new(chunks);
        let current = Rc::new(Cell::new(0usize));
        e.register_double(GET_TES_FORM, move |_, _| record_type.into_ret());
        for callee in [
            TES_FORM_LOAD_FORM,
            TES_FORM_MARK_00484AB0,
            OPERATOR_DELETE,
            TES_TEXTURE_LIST_LOAD,
            0x005b_5e40,
            0x0048_8fd0,
            0x0048_8ff0,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        let (l, c) = (list.clone(), current.clone());
        e.register_double(GET_TES_CHUNK, move |_, _| {
            l.get(c.get()).map_or(0, |chunk| chunk.0).into_ret()
        });
        let (l, c) = (list.clone(), current.clone());
        e.register_double(NEXT_CHUNK, move |_, _| {
            c.set(c.get() + 1);
            u32::from(c.get() < l.len()).into_ret()
        });
        let (l, c) = (list.clone(), current.clone());
        e.register_double(CHUNK_SIZE, move |_, _| {
            (l[c.get()].1.len() as u32).into_ret()
        });
        let (l, c) = (list, current);
        e.register_double(GET_CHUNK_DATA, move |e, a| {
            let data = &l[c.get()].1;
            let n = (a[2] as usize).min(data.len());
            e.mem.write(a[1], &data[..n]);
            Ret::default()
        });
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0].max(1)).into_ret());
        e.register(STRING_LENGTH, |e, a| {
            (e.mem.cstr(a[0]).len() as u32).into_ret()
        });
        e.register(FORM_ID, |_, _| 0x0001_2345u32.into_ret());
        e.put_vtable(0x0200_0000, &[0; 0x80]);
        e.mem.set_u32(0x0200_0000 + 0x18, 0x0090_0000);
        e.mem.set_u32(0x0200_0000 + 0x134, 0x0090_0010);
        e.mem.set_u32(0x0200_0000 + 0xe0, 0x0090_0020);
        e.mem.set_u32(0x0200_0000 + 0x130, 0x0090_0030);
        for target in [0x0090_0000, 0x0090_0010, 0x0090_0020] {
            e.register(target, |_, _| Ret::default());
        }
        e.register(0x0090_0030, |_, _| 0x6000u32.into_ret());
        e
    }

    fn debris(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this, 0x0200_0000);
        this
    }

    #[test]
    fn load_refuses_other_records() {
        let mut e = load_engine(0x51, vec![]);
        let this = debris(&mut e);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x004f_9f10, &args![this, 0x7000u32]).bool());
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }

    #[test]
    fn load_adds_debris_data_and_its_texture_list() {
        let chunks = vec![
            (CHUNK_EDID, b"Debris\0".to_vec()),
            (CHUNK_OBND, vec![0; 12]),
            (CHUNK_DATA, b"\x32a.nif\0\x07".to_vec()),
            (CHUNK_MODT, vec![1, 2]),
            (CHUNK_DATA, b"\x10b.nif\0".to_vec()),
            (CHUNK_DATA, vec![]),
        ];
        let mut e = load_engine(0x52, chunks);
        let entry = e.mem.alloc(0x10);
        e.mem.set_u32(entry + 4, 0x5555);
        e.register_double(ADD_DEBRIS_DATA, move |_, _| entry.into_ret());
        let this = debris(&mut e);
        e.call_log = Some(vec![]);
        assert!(e.call(0x004f_9f10, &args![this, 0x7000u32]).bool());
        let log = e.call_log.take().unwrap();
        let of = |addr: u32| -> Vec<Vec<u32>> {
            log.iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(of(0x0090_0000), vec![vec![this]]);
        assert_eq!(of(0x0090_0020), vec![vec![this, 0x7000]]);
        let added = of(ADD_DEBRIS_DATA);
        assert_eq!(added.len(), 2);
        assert_eq!((added[0][1], added[0][3]), (0x32, 7));
        assert_eq!((added[1][1], added[1][3]), (0x10, 0));
        assert_eq!(
            of(TES_TEXTURE_LIST_LOAD),
            vec![vec![entry + 8, 0x7000, this, 0x5555]]
        );
        // Every DATA block is freed, the empty one too.
        assert_eq!(of(OPERATOR_DELETE).len(), 3);
        assert_eq!(of(0x005b_5e40).len(), 0);
    }

    #[test]
    fn load_warns_when_a_texture_list_has_no_debris_data() {
        let mut e = load_engine(0x52, vec![(CHUNK_MODT, vec![1])]);
        let this = debris(&mut e);
        e.call_log = Some(vec![]);
        assert!(e.call(0x004f_9f10, &args![this, 0x7000u32]).bool());
        let log = e.call_log.take().unwrap();
        let warning: Vec<_> = log.iter().filter(|c| c.0 == 0x005b_5e40).collect();
        assert_eq!(
            warning[0].1,
            vec![NO_LAST_DATA_WARNING, 0x6000, 0x0001_2345]
        );
        let loads: Vec<_> = log
            .iter()
            .filter(|c| c.0 == TES_TEXTURE_LIST_LOAD)
            .collect();
        let list = loads[0].1[0];
        assert_eq!(loads[0].1[1..], [0x7000, this, UNKNOWN_NAME]);
        assert!(log.contains(&(0x0048_8fd0, vec![list])));
        assert!(log.contains(&(0x0048_8ff0, vec![list])));
    }
}
