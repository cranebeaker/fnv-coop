//! `fallout shared/tesobjectmisc.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Record tags (`u32::from_le_bytes` of the four characters) the loader `0051aaa0` tells apart.
const MODL: u32 = u32::from_le_bytes(*b"MODL");
const MODD: u32 = u32::from_le_bytes(*b"MODD");
const MOSD: u32 = u32::from_le_bytes(*b"MOSD");
const MODT: u32 = u32::from_le_bytes(*b"MODT");
const MODS: u32 = u32::from_le_bytes(*b"MODS");
const OBND: u32 = u32::from_le_bytes(*b"OBND");
const EDID: u32 = u32::from_le_bytes(*b"EDID");
const FULL: u32 = u32::from_le_bytes(*b"FULL");
const SCRI: u32 = u32::from_le_bytes(*b"SCRI");
const DSTD: u32 = u32::from_le_bytes(*b"DSTD");
const DEST: u32 = u32::from_le_bytes(*b"DEST");
const DATA: u32 = u32::from_le_bytes(*b"DATA");
const ICON: u32 = u32::from_le_bytes(*b"ICON");
const MICO: u32 = u32::from_le_bytes(*b"MICO");
const RNAM: u32 = u32::from_le_bytes(*b"RNAM");
const YNAM: u32 = u32::from_le_bytes(*b"YNAM");
const ZNAM: u32 = u32::from_le_bytes(*b"ZNAM");

// Translated from 0051aaa0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0051aaa0 - the security-cookie check (00ec408c) at the end is not translated.
/// Record loader of `tesobjectmisc.cpp` (the engine map gives the function no name, so the class
/// is unconfirmed). `this` is the form and `file` the `TESFile` positioned on its record.
/// Always answers true.
///
/// It runs `TESForm::LoadForm` (`00485110`), `00484ab0` with 0, and walks the chunks:
///
/// - `EDID`: reads the chunk (limit 0x200) and runs virtual slot `+0x134` on it; `OBND`: slot
///   `+0xe0` with the file;
/// - `FULL`: `00487050` on `+0x30`; `MODL`, `MODD`, `MOSD`, `MODT`: `TESModel::LoadModelChunk`
///   (`004892d0`) on `+0x3c`; `MODS`: the texture swaps into `+0x3c` (`0048a7b0`);
/// - `ICON`: `0048e3d0` on `+0x5c`; `MICO`: `0047a290` on `+0x8c`;
/// - `SCRI`: reads a word, `006ecd40` on `+0x68` with it and `0048cd90` on `+0x68` with the form;
/// - `DSTD`, `DEST`: `004781e0` on `+0x84`;
/// - `DATA`: `TESForm::LoadData` (`004861f0`) with a null buffer and size 0;
/// - `YNAM`: reads a word and runs `006ecd40` on `+0x9c`; `ZNAM`: reads a word and runs
///   `00403550` on `+0x9c`; `RNAM`: reads a word and gives it to `009db930` on the form.
///
/// The buffers the game takes from the stack (`__alloca_probe_16`) are heap blocks here.
pub fn fn_0051aaa0(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let base = this.addr();
    e.call(0x0048_5110, &args![this, file]);
    e.call(0x0048_4ab0, &args![this, 0u32]);
    loop {
        let chunk = e.call(0x0047_26b0, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            FULL => {
                let target = if base != 0 { base + 0x30 } else { 0 };
                e.call(0x0048_7050, &args![target, file]);
            }
            MODL | MODD | MOSD | MODT => {
                let model = if base != 0 { base + 0x3c } else { 0 };
                e.call(0x0048_92d0, &args![model, file]);
            }
            EDID => {
                let size = e.call(0x0040_1660, &args![file]).u32();
                let buffer = e.mem.alloc(size);
                e.call(0x0047_2890, &args![file, buffer, 0x200u32]);
                e.vcall(base, 0x134, &args![buffer]);
                e.mem.free(buffer);
            }
            OBND => {
                e.vcall(base, 0xe0, &args![file]);
            }
            MODS => {
                let size = e.call(0x0040_1660, &args![file]).u32();
                let buffer = e.mem.alloc(size);
                e.call(0x0047_2890, &args![file, buffer, size]);
                let swap_flags = e.call(0x0040_3570, &args![file]).u32() & 0xffff;
                e.call(0x0048_a7b0, &args![base + 0x3c, buffer, size, swap_flags]);
                e.mem.free(buffer);
            }
            ICON => {
                let target = if base != 0 { base + 0x5c } else { 0 };
                e.call(0x0048_e3d0, &args![target, file]);
            }
            MICO => {
                let target = if base != 0 { base + 0x8c } else { 0 };
                e.call(0x0047_a290, &args![target, file]);
            }
            SCRI => {
                let value = read_word(e, file);
                e.call(0x006e_cd40, &args![base + 0x68, value]);
                e.call(0x0048_cd90, &args![base + 0x68, this]);
            }
            DSTD | DEST => {
                let target = if base != 0 { base + 0x84 } else { 0 };
                e.call(0x0047_81e0, &args![target, file]);
            }
            YNAM => {
                let value = read_word(e, file);
                e.call(0x006e_cd40, &args![base + 0x9c, value]);
            }
            ZNAM => {
                let value = read_word(e, file);
                e.call(0x0040_3550, &args![base + 0x9c, value]);
            }
            DATA => {
                e.call(0x0048_61f0, &args![this, file, 0u32, 0u32]);
            }
            RNAM => {
                let value = read_word(e, file);
                e.call(0x009d_b930, &args![this, value]);
            }
            _ => {}
        }
        if !e.call(0x0047_26f0, &args![file]).bool() {
            break;
        }
    }
    true
}

/// A zeroed local read with `TESFile::GetChunkData` (`004727f0`).
fn read_word(e: &mut Engine, file: Ptr) -> u32 {
    let local = e.mem.alloc(4);
    e.call(0x0047_27f0, &args![file, local]);
    let value = e.mem.u32(local);
    e.mem.free(local);
    value
}

// Translated from 0051a9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectMISC::InitItem` (Xbox PDB; `ECX` only, plain `RET`): unless `004013e0` says
/// otherwise, runs the components' `InitItem`s with the form (`0048cd90` on the scriptable
/// form at +0x68, `00478be0` on +0x84, `0048aa80` on the model texture swap at +0x3c,
/// `0047b6a0` on the pickup/putdown sounds at +0x9c); when the word at +0xa8 (a sound's
/// form ID as loaded) is not 0, fixes it up in a stack slot through `00485d50` (cdecl, the
/// slot and the form's file from `00484e60` with -1), looks it up with `004839c0`, casts it
/// with `__RTDynamicCast` (`00ec43fb`: `.?AVTESForm@@` at `0x0118_3028` to
/// `.?AVTESSound@@` at `0x0118_32fc`) and stores the result at +0xa8; a null result prints
/// "MASTERFILE: Could not find pLoopingSound (%08X) for container '%s' (%08X)."
/// (`0x0102_b038`, through `005b5e40`) with the ID, the form's name (vtable slot +0x130) and
/// the word `0084e3a0` gives. Then `00484ab0` with 1.
pub fn tes_object_misc_init_item(e: &mut Engine, this: Ptr) {
    if e.call(0x0040_13e0, &args![this]).bool() {
        return;
    }
    let base = this.addr();
    e.call(0x0048_cd90, &args![base + 0x68, this]);
    e.call(0x0047_8be0, &args![base + 0x84, this]);
    e.call(0x0048_aa80, &args![base + 0x3c, this]);
    e.call(0x0047_b6a0, &args![base + 0x9c, this]);
    let loaded = e.mem.u32(base + 0xa8);
    if loaded != 0 {
        let id = e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), loaded);
            let file = e.call(0x0048_4e60, &args![this, 0xffff_ffffu32]).u32();
            e.call(0x0048_5d50, &args![slot, file]);
            e.mem.u32(slot.addr())
        });
        let form = e.call(0x0048_39c0, &args![id]).u32();
        let sound = e
            .call(
                0x00ec_43fb,
                &args![form, 0u32, 0x0118_3028u32, 0x0118_32fcu32, 0u32],
            )
            .u32();
        e.mem.set_u32(base + 0xa8, sound);
        if e.mem.u32(base + 0xa8) == 0 {
            let form_id = e.call(0x0084_e3a0, &args![this]).u32();
            let name = e.vcall(base, 0x130, &args![]).u32();
            e.call(0x005b_5e40, &args![0x0102_b038u32, id, name, form_id]);
        }
    }
    e.call(0x0048_4ab0, &args![this, 1u32]);
}

/// `TESForm::StartForm` (`004855a0`), `TESForm::SaveData` (`00485e00`) and
/// `TESForm::CloseForm` (`00485680`), `thiscall` on the form, no stack words.
const START_FORM: u32 = 0x0048_55a0;
const FORM_SAVE_DATA: u32 = 0x0048_5e00;
const CLOSE_FORM: u32 = 0x0048_5680;
/// The component saves (engine map; `thiscall`): `TESFullName::Save`
/// (`00487010`), `TESModelTextureSwap::Save` (`0048a520`, three chunk
/// tags), `TESTexture::Save` (`0048e370`, a tag), `BGSMessageIcon::Save`
/// (`0047a240`, a tag), `TESScriptableForm::Save` (`0048cca0`),
/// `BGSDestructibleObjectForm::Save` (`00477f20`) and
/// `BGSPickupPutdownSounds::Save` (`0047b560`).
const FULL_NAME_SAVE: u32 = 0x0048_7010;
const MODEL_TEXTURE_SWAP_SAVE: u32 = 0x0048_a520;
const TEXTURE_SAVE: u32 = 0x0048_e370;
const MESSAGE_ICON_SAVE: u32 = 0x0047_a240;
const SCRIPTABLE_FORM_SAVE: u32 = 0x0048_cca0;
const DESTRUCTIBLE_SAVE: u32 = 0x0047_7f20;
const PICKUP_PUTDOWN_SAVE: u32 = 0x0047_b560;
/// `00485910` (cdecl(chunk, word)): adds a one-word chunk; the form-id
/// getter (`0084e3a0`, `thiscall`: the word at +0xc).
const ADD_CHUNK_WORD: u32 = 0x0048_5910;
const FORM_ID: u32 = 0x0084_e3a0;

// Translated from 0051ae80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectMISC::Save` (engine map; no stack words): `StartForm`, the full
/// name (+0x30), the model with texture swaps (+0x3c; `MODL`, `MODT`,
/// `MODS`), the icon (+0x5c, `ICON`), the message icon (+0x8c, `MICO`),
/// script (+0x68), destructible data (+0x84), pickup/putdown sounds
/// (+0x9c), `TESForm::SaveData`, the form at +0xa8 by id as `RNAM` when set,
/// `CloseForm`.
pub fn tes_object_misc_save(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(START_FORM, &args![this]);
    e.call(FULL_NAME_SAVE, &args![base + 0x30]);
    e.call(
        MODEL_TEXTURE_SWAP_SAVE,
        &args![base + 0x3c, MODL, MODT, MODS],
    );
    e.call(TEXTURE_SAVE, &args![base + 0x5c, ICON]);
    e.call(MESSAGE_ICON_SAVE, &args![base + 0x8c, MICO]);
    e.call(SCRIPTABLE_FORM_SAVE, &args![base + 0x68]);
    e.call(DESTRUCTIBLE_SAVE, &args![base + 0x84]);
    e.call(PICKUP_PUTDOWN_SAVE, &args![base + 0x9c]);
    e.call(FORM_SAVE_DATA, &args![this]);
    let sound = e.mem.u32(base + 0xa8);
    if sound != 0 {
        let id = e.call(FORM_ID, &args![sound]).u32();
        e.call(ADD_CHUNK_WORD, &args![RNAM, id]);
    }
    e.call(CLOSE_FORM, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0051_aaa0, fn_0051aaa0(Ptr, Ptr) -> bool),
        entry!(0x0051_a9a0, tes_object_misc_init_item(Ptr)),
        entry!(0x0051_ae80, tes_object_misc_save(Ptr)),
    ]
}

#[cfg(test)]
mod movement_path_tests {
    use crate::prelude::*;
    use crate::units::fallout_shared::tesmodel::load_test::{
        self, calls_to, run, stub, tag, CHUNK_SIZE, CHUNK_VALUE, FILE,
    };

    type Loaded = (Engine, u32, bool, load_test::Calls);

    /// Runs the loader on a form and a file holding `chunks`. The form's virtual slots `+0xe0`
    /// and `+0x134` are the doubles `0x0090_0500` and `0x0090_0600`.
    fn load(chunks: &[u32]) -> Loaded {
        let mut e = load_test::engine(0, chunks, false);
        stub(
            &mut e,
            &[
                0x0048_5110,
                0x0048_4ab0,
                0x0048_7050,
                0x0048_92d0,
                0x0048_a7b0,
                0x0048_e3d0,
                0x0047_a290,
                0x006e_cd40,
                0x0048_cd90,
                0x0047_81e0,
                0x0040_3550,
                0x0048_61f0,
                0x009d_b930,
            ],
        );
        let this = load_test::object(
            &mut e,
            0xa0,
            0x0900_0000,
            &[(0xe0, 0x0090_0500), (0x134, 0x0090_0600)],
        );
        let (ret, calls) = run(&mut e, 0x0051_aaa0, &[this, FILE]);
        (e, this, ret.bool(), calls)
    }

    #[test]
    fn an_empty_record_loads_the_form_and_answers_true() {
        let (_, this, answer, calls) = load(&[]);
        assert!(answer);
        assert_eq!(
            calls,
            [
                (0x0048_5110, vec![this, FILE]),
                (0x0048_4ab0, vec![this, 0])
            ]
        );
    }

    #[test]
    fn the_buffer_and_virtual_chunks() {
        let (_, this, _, calls) = load(&[tag(b"EDID")]);
        let read = calls_to(&calls, 0x0047_2890);
        assert_eq!((read[0][0], read[0][2]), (FILE, 0x200));
        assert_eq!(calls_to(&calls, 0x0090_0600), [vec![this, read[0][1]]]);

        let (_, this, _, calls) = load(&[tag(b"OBND")]);
        assert_eq!(calls_to(&calls, 0x0090_0500), [vec![this, FILE]]);

        let (_, this, _, calls) = load(&[tag(b"MODS")]);
        let read = calls_to(&calls, 0x0047_2890);
        assert_eq!((read[0][0], read[0][2]), (FILE, CHUNK_SIZE));
        assert_eq!(
            calls_to(&calls, 0x0048_a7b0),
            [vec![this + 0x3c, read[0][1], CHUNK_SIZE, 0x1234]]
        );
    }

    #[test]
    fn the_components_get_their_chunks() {
        for (chunk, callee, offset) in [
            (tag(b"FULL"), 0x0048_7050, 0x30),
            (tag(b"MODL"), 0x0048_92d0, 0x3c),
            (tag(b"MODD"), 0x0048_92d0, 0x3c),
            (tag(b"MOSD"), 0x0048_92d0, 0x3c),
            (tag(b"MODT"), 0x0048_92d0, 0x3c),
            (tag(b"ICON"), 0x0048_e3d0, 0x5c),
            (tag(b"MICO"), 0x0047_a290, 0x8c),
            (tag(b"DSTD"), 0x0047_81e0, 0x84),
            (tag(b"DEST"), 0x0047_81e0, 0x84),
        ] {
            let (_, this, _, calls) = load(&[chunk]);
            assert_eq!(
                calls_to(&calls, callee),
                [vec![this + offset, FILE]],
                "{chunk:08x}"
            );
        }
    }

    #[test]
    fn the_word_chunks_reach_their_setters() {
        let (_, this, _, calls) = load(&[tag(b"SCRI")]);
        assert_eq!(
            calls_to(&calls, 0x006e_cd40),
            [vec![this + 0x68, CHUNK_VALUE]]
        );
        assert_eq!(calls_to(&calls, 0x0048_cd90), [vec![this + 0x68, this]]);

        let (_, this, _, calls) = load(&[tag(b"YNAM")]);
        assert_eq!(
            calls_to(&calls, 0x006e_cd40),
            [vec![this + 0x9c, CHUNK_VALUE]]
        );
        let (_, this, _, calls) = load(&[tag(b"ZNAM")]);
        assert_eq!(
            calls_to(&calls, 0x0040_3550),
            [vec![this + 0x9c, CHUNK_VALUE]]
        );
        let (_, this, _, calls) = load(&[tag(b"RNAM")]);
        assert_eq!(calls_to(&calls, 0x009d_b930), [vec![this, CHUNK_VALUE]]);
    }

    #[test]
    fn the_data_chunk_loads_nothing() {
        let (_, this, _, calls) = load(&[tag(b"DATA")]);
        assert_eq!(calls_to(&calls, 0x0048_61f0), [vec![this, FILE, 0, 0]]);
    }

    #[test]
    fn unknown_chunks_are_skipped() {
        let (_, _, answer, calls) = load(&[tag(b"XXXX"), tag(b"YYYY")]);
        assert!(answer);
        assert_eq!(calls.len(), 2);
    }

    /// Doubles for an `InitItem` that resolves one sound: IDs below 0x2000 are found (the cast
    /// gives the form + 0x100), the fix-up adds 1, the name is 0x7777 and the form ID 0xbeef.
    fn misc_sound_case(e: &mut Engine, checked: bool) -> u32 {
        let form = e.mem.alloc(0xb0);
        e.put_vtable(0x0300_0000, &[0; 0x50]);
        e.mem.set_u32(0x0300_0000 + 0x130, 0x0300_1130);
        e.mem.set_u32(form, 0x0300_0000);
        e.register(0x0300_1130, |_, _| Ret {
            eax: 0x7777,
            ..Ret::default()
        });
        e.register_double(0x0040_13e0, move |_, _| Ret {
            eax: u32::from(checked),
            ..Ret::default()
        });
        e.register(0x0048_4e60, |_, _| Ret {
            eax: 0x9000,
            ..Ret::default()
        });
        e.register(0x0048_5d50, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id + 1);
            Ret::default()
        });
        e.register(0x0048_39c0, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(0x00ec_43fb, |_, a| Ret {
            eax: if a[0] < 0x2000 { a[0] + 0x100 } else { 0 },
            ..Ret::default()
        });
        e.register(0x0084_e3a0, |_, _| Ret {
            eax: 0xbeef,
            ..Ret::default()
        });
        for address in [
            0x005b_5e40u32,
            0x0048_4ab0,
            0x0048_aa80,
            0x0048_cd90,
            0x0047_8be0,
            0x0047_b6a0,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        form
    }

    #[test]
    fn misc_init_item_runs_the_components_and_resolves_the_sound() {
        for (loaded, stored, warns) in
            [(0u32, 0u32, false), (0x10, 0x111, false), (0x3000, 0, true)]
        {
            let mut e = Engine::new();
            let form = misc_sound_case(&mut e, false);
            e.mem.set_u32(form + 0xa8, loaded);
            e.call_log = Some(vec![]);
            e.call(0x0051_a9a0, &args![form]);
            let log = e.call_log.take().unwrap();
            assert_eq!(e.mem.u32(form + 0xa8), stored);
            assert_eq!(
                log[2..6],
                [
                    (0x0048_cd90, vec![form + 0x68, form]),
                    (0x0047_8be0, vec![form + 0x84, form]),
                    (0x0048_aa80, vec![form + 0x3c, form]),
                    (0x0047_b6a0, vec![form + 0x9c, form]),
                ]
            );
            let warnings: Vec<_> = log.iter().filter(|call| call.0 == 0x005b_5e40).collect();
            if warns {
                assert_eq!(warnings[0].1, vec![0x0102_b038, 0x3001, 0x7777, 0xbeef]);
            } else {
                assert!(warnings.is_empty());
            }
            let casts: Vec<_> = log.iter().filter(|call| call.0 == 0x00ec_43fb).collect();
            assert_eq!(casts.len(), usize::from(loaded != 0));
            if loaded != 0 {
                assert_eq!(casts[0].1[2..], [0x0118_3028, 0x0118_32fc, 0]);
            }
            assert_eq!(log.last().unwrap(), &(0x0048_4ab0, vec![form, 1]));
        }
        let mut e = Engine::new();
        let form = misc_sound_case(&mut e, true);
        e.call_log = Some(vec![]);
        e.call(0x0051_a9a0, &args![form]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }
}

#[cfg(test)]
mod save_tests {
    use super::*;

    #[test]
    fn save_writes_each_component_and_the_sound() {
        let mut e = Engine::new();
        for a in [
            START_FORM,
            FORM_SAVE_DATA,
            CLOSE_FORM,
            FULL_NAME_SAVE,
            MODEL_TEXTURE_SWAP_SAVE,
            TEXTURE_SAVE,
            MESSAGE_ICON_SAVE,
            SCRIPTABLE_FORM_SAVE,
            DESTRUCTIBLE_SAVE,
            PICKUP_PUTDOWN_SAVE,
            ADD_CHUNK_WORD,
        ] {
            e.register(a, |_, _| Ret::default());
        }
        e.register(FORM_ID, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        let this = e.mem.alloc(0xb0);
        e.call_log = Some(vec![]);
        e.call(0x0051_ae80, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (START_FORM, vec![this]),
                (FULL_NAME_SAVE, vec![this + 0x30]),
                (MODEL_TEXTURE_SWAP_SAVE, vec![this + 0x3c, MODL, MODT, MODS]),
                (TEXTURE_SAVE, vec![this + 0x5c, ICON]),
                (MESSAGE_ICON_SAVE, vec![this + 0x8c, MICO]),
                (SCRIPTABLE_FORM_SAVE, vec![this + 0x68]),
                (DESTRUCTIBLE_SAVE, vec![this + 0x84]),
                (PICKUP_PUTDOWN_SAVE, vec![this + 0x9c]),
                (FORM_SAVE_DATA, vec![this]),
                (CLOSE_FORM, vec![this])
            ]
        );
        let sound = e.mem.alloc(0x18);
        e.mem.set_u32(sound + 0xc, 0x0789);
        e.mem.set_u32(this + 0xa8, sound);
        e.call_log = Some(vec![]);
        e.call(0x0051_ae80, &args![this]);
        let log = e.call_log.take().unwrap();
        let added: Vec<_> = log.iter().filter(|c| c.0 == ADD_CHUNK_WORD).collect();
        assert_eq!(added.len(), 1);
        assert_eq!(added[0].1, vec![RNAM, 0x0789]);
    }
}
