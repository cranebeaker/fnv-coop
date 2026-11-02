//! `fallout shared/bgsmenuicon.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00590210 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00590210 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// `BGSMenuIcon::BGSMenuIcon` (Xbox PDB): the form constructor `CreateFormOfType` (`00465110`) runs for form type 5 (0x24 bytes).
/// `ECX` is the object; it runs the base and component constructors, stores the vtable pointers of the object and its components, and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn bgsmenuicon_bgsmenuicon(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0040_2c20, &args![base + 0x18]);
    e.mem.set_u32(base, 0x0103_3654);
    e.mem.set_u32(base + 0x18, 0x0103_3634);
    e.call(0x0048_3710, &args![this]);
    e.call(0x004f_15a0, &args![this, 0x5]);
    this
}

// Translated from 005902a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `005902d0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_005902a0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0059_02d0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// `TESFile::GetTESForm` (engine map): the current record's form type.
const GET_TES_FORM: u32 = 0x0047_2660;
/// `TESForm::LoadForm` (engine map): `__thiscall` on the form, the file.
const TES_FORM_LOAD_FORM: u32 = 0x0048_5110;
/// `TESFile::GetTESChunk` (engine map): the current chunk's signature.
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// The current chunk's size (the word at `file + 0x25c`).
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `TESFile::GetChunkData_ov3` (engine map): destination, byte count.
const GET_CHUNK_DATA: u32 = 0x0047_2890;
/// Moves to the next chunk; false at the end of the record.
const NEXT_CHUNK: u32 = 0x0047_26f0;
/// `TESTexture::LoadTextureChunk` (engine map), cdecl: texture, file.
const LOAD_TEXTURE_CHUNK: u32 = 0x0048_e3d0;
/// The form type `GetTESForm` must answer for this record.
const FORM_TYPE_MENU_ICON: u8 = 5;
const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_ICON: u32 = 0x4e4f_4349;
/// The form's virtual slot called with the text of an `EDID` chunk.
const SLOT_SET_EDITOR_ID: u32 = 0x134;
/// The form's virtual slot called with the file for an `OBND` chunk.
const SLOT_LOAD_BOUNDS: u32 = 0xe0;
/// The most an `EDID` chunk is read with.
const EDITOR_ID_MAXIMUM: u32 = 0x200;
/// `this + 0x18`: the texture component (its vtable is stored there by the
/// constructor).
const TEXTURE_OFFSET: u32 = 0x18;

// Translated from 005902d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005902d0 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated.
/// The destructor body `fn_005902a0` runs (that scalar deleting destructor
/// is slot +0x10 of the vtable the constructor stores, `01033654`): it
/// stores the object's and the texture component's vtables back
/// (`01033654`, and `01033634` at +0x18), runs `00483710` on `this` (an
/// empty function in the exe), `00402c40` on the texture component at
/// +0x18 and `TESForm::~TESForm` (`00483630`, engine map) on `this`. `ECX`,
/// no stack words. The exception-unwinding frame is not translated.
pub fn fn_005902d0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0103_3654);
    e.mem.set_u32(base + 0x18, 0x0103_3634);
    e.call(0x0048_3710, &args![this]);
    e.call(0x0040_2c40, &args![base + 0x18]);
    e.call(0x0048_3630, &args![this]);
}

// Translated from 00590350 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00590350 - the /GS stack cookie check (00ec408c) is not translated.
/// The `Load` slot (+0x20) of the vtable at `01033654` that
/// `BGSMenuIcon::BGSMenuIcon` stores: `__thiscall`, `RET 4`, `file`. When
/// the record's form type (`TESFile::GetTESForm`) is not 5 it answers false.
/// Otherwise it reads the form header (`TESForm::LoadForm`) and then every
/// chunk of the record, and answers true:
/// - `EDID`: read into a stack buffer of the chunk's size (`_alloca_probe_16`,
///   at most 0x200 bytes read) and passed to virtual slot 0x134;
/// - `OBND`: virtual slot 0xe0 with the file;
/// - `ICON`: `TESTexture::LoadTextureChunk` on the texture at `this + 0x18`
///   (null when `this` is null).
///
/// Other chunks are skipped. The stack cookie check is not translated.
pub fn fn_00590350(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let object = this.addr();
    if e.call(GET_TES_FORM, &args![file]).u8() != FORM_TYPE_MENU_ICON {
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
                    e.call(GET_CHUNK_DATA, &args![file, buffer, EDITOR_ID_MAXIMUM]);
                    e.vcall(object, SLOT_SET_EDITOR_ID, &args![buffer]);
                });
            }
            CHUNK_OBND => {
                e.vcall(object, SLOT_LOAD_BOUNDS, &args![file]);
            }
            CHUNK_ICON => {
                let texture = if object == 0 {
                    0
                } else {
                    object + TEXTURE_OFFSET
                };
                e.call(LOAD_TEXTURE_CHUNK, &args![texture, file]);
            }
            _ => {}
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

/// `TESForm::StartForm` and `TESForm::CloseForm` (`004855a0`, `00485680`, `thiscall`).
const FORM_START_FORM: u32 = 0x0048_55a0;
const FORM_CLOSE_FORM: u32 = 0x0048_5680;
/// `TESTexture::Save` (`0048e370`, `RET 4`: the chunk signature).
const TEXTURE_SAVE: u32 = 0x0048_e370;

// Translated from 00590470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSMenuIcon::Save` (Xbox PDB; `thiscall`, no stack word): starts the form record,
/// saves the texture (+0x18) as an `ICON` chunk (`0048e370`) and closes the record.
pub fn bgs_menu_icon_save(e: &mut Engine, this: Ptr) {
    e.call(FORM_START_FORM, &args![this]);
    e.call(
        TEXTURE_SAVE,
        &args![this.addr() + TEXTURE_OFFSET, CHUNK_ICON],
    );
    e.call(FORM_CLOSE_FORM, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0059_0210, bgsmenuicon_bgsmenuicon(Ptr) -> Ptr),
        entry!(0x0059_02a0, fn_005902a0(Ptr, u32) -> Ptr),
        entry!(0x0059_02d0, fn_005902d0(Ptr)),
        entry!(0x0059_0350, fn_00590350(Ptr, Ptr) -> bool),
        entry!(0x0059_0470, bgs_menu_icon_save(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn bgsmenuicon_bgsmenuicon_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0059_0210,
            size: 0x24,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_2c20, &[A::T(0x18)]),
                (0x0048_3710, &[A::T(0x0)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x5)]),
            ],
            stores: &[(0x0, 4, 0x0103_3654), (0x18, 4, 0x0103_3634)],
        });
    }
    #[test]
    fn at_005902a0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0059_02d0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0059_02a0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0059_02a0)
                .collect();
            let mut expected = vec![(0x0059_02d0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod b022_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const VTABLE: u32 = 0x0200_0000;
    const SET_EDITOR_ID: u32 = 0x0300_0000;
    const LOAD_BOUNDS: u32 = 0x0300_0010;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    #[test]
    fn the_destructor_restores_the_vtables_and_runs_the_bases() {
        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(0x24));
        for callee in [0x0048_3710, 0x0040_2c40, 0x0048_3630] {
            e.register(callee, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x0059_02d0, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), 0x0103_3654);
        assert_eq!(e.mem.u32(this.addr() + 0x18), 0x0103_3634);
        let log: Vec<(u32, Vec<u32>)> = e.call_log.take().unwrap().into_iter().skip(1).collect();
        assert_eq!(
            log,
            vec![
                (0x0048_3710, vec![this.addr()]),
                (0x0040_2c40, vec![this.addr() + 0x18]),
                (0x0048_3630, vec![this.addr()]),
            ]
        );
    }

    /// A file double whose chunks come from `chunks`; the next-chunk call
    /// answers true while chunks are left.
    fn setup(form_type: u32, chunks: &[u32]) -> (Engine, Ptr, Ptr) {
        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(0x24));
        e.mem.set_u32(this.addr(), VTABLE);
        e.map(VTABLE, 0x200);
        e.mem.set_u32(VTABLE + SLOT_SET_EDITOR_ID, SET_EDITOR_ID);
        e.mem.set_u32(VTABLE + SLOT_LOAD_BOUNDS, LOAD_BOUNDS);
        let file: Ptr = Ptr::new(e.mem.alloc(0x300));
        e.register_double(GET_TES_FORM, move |_, _| ret(form_type));
        let queue = Rc::new(RefCell::new(chunks.to_vec()));
        let current = queue.clone();
        e.register_double(GET_TES_CHUNK, move |_, _| {
            ret(current.borrow().first().copied().unwrap_or(0))
        });
        e.register_double(NEXT_CHUNK, move |_, _| {
            let mut q = queue.borrow_mut();
            q.remove(0);
            ret(u32::from(!q.is_empty()))
        });
        e.register(CHUNK_SIZE, |_, _| ret(0x11));
        for callee in [
            TES_FORM_LOAD_FORM,
            GET_CHUNK_DATA,
            LOAD_TEXTURE_CHUNK,
            SET_EDITOR_ID,
            LOAD_BOUNDS,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        (e, this, file)
    }

    #[test]
    fn another_form_type_answers_false_and_reads_nothing() {
        let (mut e, this, file) = setup(6, &[CHUNK_EDID]);
        assert!(!e.call(0x0059_0350, &args![this, file]).bool());
        assert!(calls_to(&e, TES_FORM_LOAD_FORM).is_empty());
        assert!(calls_to(&e, GET_TES_CHUNK).is_empty());
    }

    #[test]
    fn each_chunk_goes_to_its_reader() {
        let (mut e, this, file) = setup(5, &[CHUNK_EDID, CHUNK_OBND, 0x4c4c_5546, CHUNK_ICON]);
        assert!(e.call(0x0059_0350, &args![this, file]).bool());
        assert_eq!(
            calls_to(&e, TES_FORM_LOAD_FORM),
            vec![vec![this.addr(), file.addr()]]
        );
        let reads = calls_to(&e, GET_CHUNK_DATA);
        assert_eq!(reads.len(), 1);
        assert_eq!(reads[0][0], file.addr());
        assert_eq!(reads[0][2], EDITOR_ID_MAXIMUM);
        assert_eq!(
            calls_to(&e, SET_EDITOR_ID),
            vec![vec![this.addr(), reads[0][1]]]
        );
        assert_eq!(
            calls_to(&e, LOAD_BOUNDS),
            vec![vec![this.addr(), file.addr()]]
        );
        assert_eq!(
            calls_to(&e, LOAD_TEXTURE_CHUNK),
            vec![vec![this.addr() + 0x18, file.addr()]]
        );
        assert_eq!(calls_to(&e, NEXT_CHUNK).len(), 4);
    }

    #[test]
    fn a_zero_chunk_ends_the_record_at_once() {
        let (mut e, this, file) = setup(5, &[]);
        assert!(e.call(0x0059_0350, &args![this, file]).bool());
        assert_eq!(calls_to(&e, GET_TES_CHUNK).len(), 1);
        assert!(calls_to(&e, NEXT_CHUNK).is_empty());
    }

    #[test]
    fn a_null_form_reads_the_icon_into_a_null_texture() {
        let (mut e, _, file) = setup(5, &[CHUNK_ICON]);
        assert!(e.call(0x0059_0350, &args![Ptr::<()>::NULL, file]).bool());
        assert_eq!(calls_to(&e, LOAD_TEXTURE_CHUNK), vec![vec![0, file.addr()]]);
    }
}

#[cfg(test)]
mod tests_n036 {
    use super::*;

    #[test]
    fn save_writes_the_icon_texture() {
        let mut e = Engine::new();
        for addr in [FORM_START_FORM, FORM_CLOSE_FORM, TEXTURE_SAVE] {
            e.register(addr, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x0059_0470, &args![0x6000u32]);
        assert_eq!(
            e.call_log.take().unwrap()[1..],
            [
                (FORM_START_FORM, vec![0x6000]),
                (TEXTURE_SAVE, vec![0x6018, CHUNK_ICON]),
                (FORM_CLOSE_FORM, vec![0x6000]),
            ]
        );
    }
}
