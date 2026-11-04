//! `fallout shared/teshair.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

const GET_TES_FORM: u32 = 0x0047_2660;
const TES_FORM_LOAD_FORM: u32 = 0x0048_5110;
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// The current chunk's size (the word at `file + 0x25c`).
const CHUNK_SIZE: u32 = 0x0040_1660;
const GET_CHUNK_DATA: u32 = 0x0047_2890;
const NEXT_CHUNK: u32 = 0x0047_26f0;
/// The full-name reader (`00487050`, cdecl: name, file).
const LOAD_FULL_NAME: u32 = 0x0048_7050;
/// `TESModel::LoadModelChunk` and `TESTexture::LoadTextureChunk` (engine
/// map), cdecl.
const LOAD_MODEL_CHUNK: u32 = 0x0048_92d0;
const LOAD_TEXTURE_CHUNK: u32 = 0x0048_e3d0;
/// The form type of a hair record.
const FORM_TYPE_HAIR: u32 = 10;
const CHUNK_MODL: u32 = 0x4c44_4f4d;
const CHUNK_MODT: u32 = 0x5444_4f4d;
const CHUNK_DATA: u32 = 0x4154_4144;
const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_FULL: u32 = 0x4c4c_5546;
const CHUNK_ICON: u32 = 0x4e4f_4349;

/// `this + offset`, or null when `this` is null.
fn component(this: u32, offset: u32) -> u32 {
    if this == 0 {
        0
    } else {
        this + offset
    }
}

// Translated from 005fdcb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005fdcb0 - the /GS stack cookie check (00ec408c) is not translated.
/// `TESHair::Load` (engine map; the `Load` slot +0x20 of the `TESHair`
/// vtable `01049b9c`): `__thiscall`, `RET 4`, `file`. Answers false when
/// the record's form type is not 10; otherwise reads the form header and
/// every chunk and answers true:
/// - `EDID`: a stack buffer of the chunk's size (at most 0x200 read) for
///   virtual slot 0x134; `OBND`: virtual slot 0xe0 with the file;
/// - `FULL`: `00487050` on +0x18; `MODL`, `MODT`:
///   `TESModel::LoadModelChunk` on +0x24; `ICON`:
///   `TESTexture::LoadTextureChunk` on +0x3c (each null when `this` is
///   null);
/// - `DATA`: one byte into +0x48.
///
/// The stack cookie check is not translated.
pub fn tes_hair_load(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let hair = this.addr();
    if e.call(GET_TES_FORM, &args![file]).u32() != FORM_TYPE_HAIR {
        return false;
    }
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
                    e.call(GET_CHUNK_DATA, &args![file, buffer, 0x200u32]);
                    e.vcall(hair, 0x134, &args![buffer]);
                });
            }
            CHUNK_OBND => {
                e.vcall(hair, 0xe0, &args![file]);
            }
            CHUNK_FULL => {
                e.call(LOAD_FULL_NAME, &args![component(hair, 0x18), file]);
            }
            CHUNK_MODL | CHUNK_MODT => {
                e.call(LOAD_MODEL_CHUNK, &args![component(hair, 0x24), file]);
            }
            CHUNK_ICON => {
                e.call(LOAD_TEXTURE_CHUNK, &args![component(hair, 0x3c), file]);
            }
            CHUNK_DATA => {
                e.call(GET_CHUNK_DATA, &args![file, hair + 0x48, 1u32]);
            }
            _ => {}
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

/// `TESForm::StartForm` (`004855a0`), `TESFullName::Save` (`00487010`),
/// `TESModel::Save` (`004891d0`, two chunk words), `TESTexture::Save`
/// (`0048e370`, one chunk word), `TESForm::AddChunk` with a byte
/// (`004858f0`, `cdecl`) and `TESForm::CloseForm` (`00485680`).
const SAVE_FORM_START: u32 = 0x0048_55a0;
const SAVE_FULL_NAME: u32 = 0x0048_7010;
const SAVE_MODEL: u32 = 0x0048_91d0;
const SAVE_TEXTURE: u32 = 0x0048_e370;
const SAVE_CHUNK_BYTE: u32 = 0x0048_58f0;
const SAVE_FORM_CLOSE: u32 = 0x0048_5680;

// Translated from 005fdc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESHair::Save` (Xbox PDB): starts the form, saves the full name
/// (+0x18), the model (+0x24, `MODL`, `MODT`) and the texture (+0x3C,
/// `ICON`), adds the flags byte at +0x48 as `DATA` and closes the form.
pub fn tes_hair_save(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    e.call(SAVE_FORM_START, &args![this]);
    e.call(SAVE_FULL_NAME, &args![at + 0x18]);
    e.call(SAVE_MODEL, &args![at + 0x24, CHUNK_MODL, CHUNK_MODT]);
    e.call(SAVE_TEXTURE, &args![at + 0x3c, CHUNK_ICON]);
    let flags = u32::from(e.mem.u8(at + 0x48));
    e.call(SAVE_CHUNK_BYTE, &args![CHUNK_DATA, flags]);
    e.call(SAVE_FORM_CLOSE, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005f_dcb0, tes_hair_load(Ptr, Ptr) -> bool),
        entry!(0x005f_dc50, tes_hair_save(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const VTABLE: u32 = 0x0200_0000;
    const SET_EDITOR_ID: u32 = 0x0300_0000;
    const LOAD_BOUNDS: u32 = 0x0300_0010;

    type Chunks = Rc<RefCell<Vec<(u32, Vec<u8>)>>>;
    type Text = Rc<RefCell<Vec<u8>>>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn setup(form_type: u32, chunks: &[(u32, &[u8])]) -> (Engine, u32, Ptr, Text) {
        let mut e = Engine::new();
        e.map(VTABLE, 0x200);
        e.mem.set_u32(VTABLE + 0x134, SET_EDITOR_ID);
        e.mem.set_u32(VTABLE + 0xe0, LOAD_BOUNDS);
        let text: Text = Rc::new(RefCell::new(vec![]));
        let seen = text.clone();
        e.register_double(SET_EDITOR_ID, move |e, a| {
            *seen.borrow_mut() = e.mem.cstr(a[1]);
            Ret::default()
        });
        e.register(LOAD_BOUNDS, |_, _| Ret::default());
        let hair = e.mem.alloc(0x4c);
        e.mem.set_u32(hair, VTABLE);
        let file: Ptr = Ptr::new(e.mem.alloc(0x300));
        let queue: Chunks = Rc::new(RefCell::new(
            chunks.iter().map(|(s, d)| (*s, d.to_vec())).collect(),
        ));
        e.register_double(GET_TES_FORM, move |_, _| ret(form_type));
        let q = queue.clone();
        e.register_double(GET_TES_CHUNK, move |_, _| {
            ret(q.borrow().first().map_or(0, |c| c.0))
        });
        let q = queue.clone();
        e.register_double(CHUNK_SIZE, move |_, _| {
            ret(q.borrow().first().map_or(0, |c| c.1.len() as u32))
        });
        let q = queue.clone();
        e.register_double(GET_CHUNK_DATA, move |e, a| {
            let data = q.borrow().first().map(|c| c.1.clone()).unwrap_or_default();
            let n = (a[2] as usize).min(data.len());
            e.mem.write(a[1], &data[..n]);
            Ret::default()
        });
        e.register_double(NEXT_CHUNK, move |_, _| {
            let mut q = queue.borrow_mut();
            q.remove(0);
            ret(u32::from(!q.is_empty()))
        });
        for callee in [
            TES_FORM_LOAD_FORM,
            LOAD_FULL_NAME,
            LOAD_MODEL_CHUNK,
            LOAD_TEXTURE_CHUNK,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        (e, hair, file, text)
    }

    #[test]
    fn another_form_type_answers_false() {
        let (mut e, hair, file, _) = setup(9, &[(CHUNK_DATA, &[1])]);
        assert!(!e.call(0x005f_dcb0, &args![hair, file]).bool());
        assert!(calls_to(&e, TES_FORM_LOAD_FORM).is_empty());
    }

    #[test]
    fn every_chunk_goes_to_its_reader() {
        let chunks: [(u32, &[u8]); 8] = [
            (CHUNK_EDID, b"HairB\0"),
            (CHUNK_OBND, &[0; 12]),
            (CHUNK_FULL, b"x\0"),
            (CHUNK_MODL, b"m\0"),
            (CHUNK_MODT, &[0; 4]),
            (CHUNK_ICON, b"i\0"),
            (CHUNK_DATA, &[7]),
            (0x5858_5858, &[1]),
        ];
        let (mut e, hair, file, text) = setup(10, &chunks);
        assert!(e.call(0x005f_dcb0, &args![hair, file]).bool());
        let f = file.addr();
        assert_eq!(calls_to(&e, TES_FORM_LOAD_FORM), vec![vec![hair, f]]);
        assert_eq!(*text.borrow(), b"HairB");
        assert_eq!(calls_to(&e, LOAD_BOUNDS), vec![vec![hair, f]]);
        assert_eq!(calls_to(&e, LOAD_FULL_NAME), vec![vec![hair + 0x18, f]]);
        assert_eq!(
            calls_to(&e, LOAD_MODEL_CHUNK),
            vec![vec![hair + 0x24, f]; 2]
        );
        assert_eq!(calls_to(&e, LOAD_TEXTURE_CHUNK), vec![vec![hair + 0x3c, f]]);
        assert_eq!(e.mem.u8(hair + 0x48), 7);
        assert_eq!(calls_to(&e, NEXT_CHUNK).len(), 8);
    }

    #[test]
    fn a_null_hair_reads_components_into_null() {
        let chunks: [(u32, &[u8]); 3] =
            [(CHUNK_FULL, &[0]), (CHUNK_MODL, &[0]), (CHUNK_ICON, &[0])];
        let (mut e, _, file, _) = setup(10, &chunks);
        assert!(e.call(0x005f_dcb0, &args![0u32, file]).bool());
        let f = file.addr();
        assert_eq!(calls_to(&e, LOAD_FULL_NAME), vec![vec![0, f]]);
        assert_eq!(calls_to(&e, LOAD_MODEL_CHUNK), vec![vec![0, f]]);
        assert_eq!(calls_to(&e, LOAD_TEXTURE_CHUNK), vec![vec![0, f]]);
    }
}

#[cfg(test)]
mod n043_tests {
    use super::*;

    #[test]
    fn save_writes_name_model_texture_and_flags() {
        let mut e = Engine::new();
        for addr in [
            SAVE_FORM_START,
            SAVE_FULL_NAME,
            SAVE_MODEL,
            SAVE_TEXTURE,
            SAVE_CHUNK_BYTE,
            SAVE_FORM_CLOSE,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x50);
        e.mem.set_u8(this + 0x48, 5);
        e.call_log = Some(vec![]);
        e.call(0x005f_dc50, &args![this]);
        let log: Vec<(u32, Vec<u32>)> = e.call_log.take().unwrap().into_iter().skip(1).collect();
        assert_eq!(
            log,
            vec![
                (SAVE_FORM_START, vec![this]),
                (SAVE_FULL_NAME, vec![this + 0x18]),
                (SAVE_MODEL, vec![this + 0x24, CHUNK_MODL, CHUNK_MODT]),
                (SAVE_TEXTURE, vec![this + 0x3c, CHUNK_ICON]),
                (SAVE_CHUNK_BYTE, vec![CHUNK_DATA, 5]),
                (SAVE_FORM_CLOSE, vec![this]),
            ]
        );
    }
}
