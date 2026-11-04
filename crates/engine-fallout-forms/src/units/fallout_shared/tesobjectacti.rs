//! `fallout shared/tesobjectacti.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Record tags (`u32::from_le_bytes` of the four characters) the loader `00510a90` tells apart.
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
const INAM: u32 = u32::from_le_bytes(*b"INAM");
const RNAM: u32 = u32::from_le_bytes(*b"RNAM");
const SNAM: u32 = u32::from_le_bytes(*b"SNAM");
const VNAM: u32 = u32::from_le_bytes(*b"VNAM");
const WNAM: u32 = u32::from_le_bytes(*b"WNAM");
const XATO: u32 = u32::from_le_bytes(*b"XATO");

// Translated from 00510a90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00510a90 - the security-cookie check (00ec408c) at the end is not translated.
/// Record loader of `tesobjectacti.cpp` (the engine map gives the function
/// no name, so the class is unconfirmed). `this` is the form and `file` the `TESFile`
/// positioned on its record. Always answers true.
///
/// It runs `TESForm::LoadForm` (`00485110`), `00484ab0` with 0, and walks the chunks:
///
/// - `EDID`: reads the chunk (limit 0x200) and runs virtual slot `+0x134` on it; `OBND`: slot
///   `+0xe0` with the file;
/// - `FULL`: `00487050` on `+0x30`; `MODL`, `MODD`, `MOSD`, `MODT`: `TESModel::LoadModelChunk`
///   (`004892d0`) on `+0x3c`; `MODS`: the texture swaps into `+0x3c` (`0048a7b0`);
/// - `SCRI`: reads a word, `006ecd40` on `+0x5c` with it and `0048cd90` on `+0x5c` with the form;
/// - `DSTD`, `DEST`: `004781e0` on `+0x68`;
/// - `INAM`, `RNAM`, `SNAM`, `VNAM`, `WNAM`: reads a word and gives it to `004febd0`,
///   `004fb090`, `004febb0`, `00442a80` and `008ac890` on the form;
/// - `XATO`: when the chunk is longer than one byte, reads it into a heap block (`00401000`,
///   freed with `00401030`), builds an 8 byte string from it (`0040c0e0`) and passes that by
///   value to `00510ea0`.
///
/// The buffers the game takes from the stack (`__alloca_probe_16`) are heap blocks here.
pub fn fn_00510a90(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let base = this.addr();
    e.call(0x0048_5110, &args![this, file]);
    e.call(0x0048_4ab0, &args![this, 0u32]);
    loop {
        let chunk = e.call(0x0047_26b0, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            INAM => {
                let value = read_word(e, file);
                e.call(0x004f_ebd0, &args![this, value]);
            }
            DSTD | DEST => {
                let target = if base != 0 { base + 0x68 } else { 0 };
                e.call(0x0047_81e0, &args![target, file]);
            }
            OBND => {
                e.vcall(base, 0xe0, &args![file]);
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
            FULL => {
                let target = if base != 0 { base + 0x30 } else { 0 };
                e.call(0x0048_7050, &args![target, file]);
            }
            MODS => {
                let size = e.call(0x0040_1660, &args![file]).u32();
                let buffer = e.mem.alloc(size);
                e.call(0x0047_2890, &args![file, buffer, size]);
                let swap_flags = e.call(0x0040_3570, &args![file]).u32() & 0xffff;
                e.call(0x0048_a7b0, &args![base + 0x3c, buffer, size, swap_flags]);
                e.mem.free(buffer);
            }
            SCRI => {
                let value = read_word(e, file);
                e.call(0x006e_cd40, &args![base + 0x5c, value]);
                e.call(0x0048_cd90, &args![base + 0x5c, this]);
            }
            RNAM => {
                let value = read_word(e, file);
                e.call(0x004f_b090, &args![this, value]);
            }
            SNAM => {
                let value = read_word(e, file);
                e.call(0x004f_ebb0, &args![this, value]);
            }
            VNAM => {
                let value = read_word(e, file);
                e.call(0x0044_2a80, &args![this, value]);
            }
            WNAM => {
                let value = read_word(e, file);
                e.call(0x008a_c890, &args![this, value]);
            }
            XATO => {
                let size = e.call(0x0040_1660, &args![file]).i32();
                if size > 1 {
                    let buffer = e.call(0x0040_1000, &args![size]).u32();
                    e.call(0x0047_2890, &args![file, buffer, size]);
                    let text = e.mem.alloc(8);
                    e.call(0x0040_c0e0, &args![text, buffer]);
                    let (first, second) = (e.mem.u32(text), e.mem.u32(text + 4));
                    e.mem.free(text);
                    e.call(0x0051_0ea0, &args![this, first, second]);
                    e.call(0x0040_1030, &args![buffer]);
                }
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

/// `TESForm::GetFile` (Xbox PDB, `00484e60`, `RET 4`).
const GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex` (Xbox PDB, `00485d50`, cdecl, two words).
const ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// Looks up a form by id (`004839c0`, cdecl, one word).
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The RTTI type descriptor `.?AVTESForm@@`, every cast's source type.
const TYPE_TES_FORM: u32 = 0x0118_3028;
/// `TESForm`'s form id getter (`0084e3a0`).
const FORM_ID: u32 = 0x0084_e3a0;
/// The warning printer (`005b5e40`, cdecl, variable arguments).
const WARNING: u32 = 0x005b_5e40;

/// The form pointers `InitItem` resolves, in its order: the offset in the
/// activator, the RTTI type descriptor of the cast's target and the message
/// for a missing one ("MASTERFILE: Could not find looping sound / activate
/// sound / pRadioTemplate / water form / radio station (%08X) for activator
/// '%s' (%08X).").
const RESOLVED_FORMS: [(u32, u32, u32); 5] = [
    (0x74, 0x0118_32fc, 0x0102_a028),
    (0x78, 0x0118_32fc, 0x0102_9fd8),
    (0x7c, 0x0118_32fc, 0x0102_9f88),
    (0x80, 0x0118_4118, 0x0102_9f40),
    (0x84, 0x0118_9e04, 0x0102_9ef0),
];

// Translated from 00510770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectACTI::InitItem` (Xbox PDB): unless form flag 0x8 is set
/// (`004013e0`), runs the components' `InitItem` with the activator
/// (`TESScriptableForm` at `+0x5C`, `0048cd90`; `BGSDestructibleObjectForm`
/// at `+0x68`, `00478be0`; `TESModelTextureSwap` at `+0x3C`, `0048aa80`),
/// then resolves the five form ids at `+0x74`..`+0x84` that are not zero:
/// each gets the compile index of the activator's file (`00485d50` with
/// `TESForm::GetFile(-1)`), is looked up (`004839c0`) and cast from
/// `TESForm` to its type (`RESOLVED_FORMS`), the result stored back; a
/// missing one is reported (with the id, vtable slot 0x130's answer and the
/// form id). The radio station at `+0x84` is also reported, and cleared,
/// when `004fedc0` on it answers false. Finally form flag 0x8 is set
/// (`00484ab0`).
pub fn tes_object_acti_init_item(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.call(0x0040_13e0, &args![this]).bool() {
        return;
    }
    e.call(0x0048_cd90, &args![base + 0x5c, this]);
    e.call(0x0047_8be0, &args![base + 0x68, this]);
    e.call(0x0048_aa80, &args![base + 0x3c, this]);
    e.with_stack(4, |e, id_at| {
        for (offset, target, message) in RESOLVED_FORMS {
            let id = e.mem.u32(base + offset);
            e.mem.set_u32(id_at.addr(), id);
            if id == 0 {
                continue;
            }
            let file = e.call(GET_FILE, &args![this, u32::MAX]).u32();
            e.call(ADD_COMPILE_INDEX, &args![id_at, file]);
            let id = e.mem.u32(id_at.addr());
            let form = e.call(LOOKUP_FORM, &args![id]).u32();
            let cast = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![form, 0u32, TYPE_TES_FORM, target, 0u32],
                )
                .u32();
            e.mem.set_u32(base + offset, cast);
            let missing = if offset == 0x84 {
                let station = e.mem.u32(base + offset);
                station == 0 || !e.call(0x004f_edc0, &args![station]).bool()
            } else {
                e.mem.u32(base + offset) == 0
            };
            if missing {
                let form_id = e.call(FORM_ID, &args![this]).u32();
                let name = e.vcall(base, 0x130, &args![]).u32();
                let id = e.mem.u32(id_at.addr());
                e.call(WARNING, &args![message, id, name, form_id]);
                if offset == 0x84 {
                    e.mem.set_u32(base + offset, 0);
                }
            }
        }
    });
    e.call(0x0048_4ab0, &args![this, true]);
}

// Translated from 00510ea0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00510ea0 - the exception-unwinding frame (FS:[0]) is not translated.
/// Takes an 8-byte `BSStringT` by value (two words, `RET 8`; `00510a90`
/// passes the `XATO` text): sets the string at `this + 0x88` from it
/// (`00438470` with the argument's address), then destroys the argument
/// (`004037d0`).
pub fn fn_00510ea0(e: &mut Engine, this: Ptr, text_data: u32, text_lengths: u32) {
    e.with_stack(8, |e, text| {
        e.mem.set_u32(text.addr(), text_data);
        e.mem.set_u32(text.addr() + 4, text_lengths);
        e.call(0x0043_8470, &args![this.addr() + 0x88, text]);
        e.call(0x0040_37d0, &args![text]);
    });
}

/// `TESForm::StartForm` (`004855a0`) and `TESForm::CloseForm` (`00485680`),
/// `thiscall` on the form, no stack words.
const START_FORM: u32 = 0x0048_55a0;
const CLOSE_FORM: u32 = 0x0048_5680;
/// The component saves (engine map; `thiscall`): `TESFullName::Save`
/// (`00487010`), `TESModelTextureSwap::Save` (`0048a520`, three chunk tags),
/// `TESScriptableForm::Save` (`0048cca0`) and
/// `BGSDestructibleObjectForm::Save` (`00477f20`).
const FULL_NAME_SAVE: u32 = 0x0048_7010;
const MODEL_TEXTURE_SWAP_SAVE: u32 = 0x0048_a520;
const SCRIPTABLE_FORM_SAVE: u32 = 0x0048_cca0;
const DESTRUCTIBLE_SAVE: u32 = 0x0047_7f20;
/// `00485910` (cdecl(chunk, word)): adds a one-word chunk;
/// `TESForm::AddChunkArray` (`004856f0`, cdecl(chunk, data, size)).
const ADD_CHUNK_WORD: u32 = 0x0048_5910;
const ADD_CHUNK_ARRAY: u32 = 0x0048_56f0;
/// The `BSStringT` at +0x88: its length (`004048e0`, `thiscall`), a copy of
/// it into a string the caller gives (`005110e0`, `thiscall` on the
/// activator, `RET 4`; answers that string), the string's text (`00559450`)
/// and its destructor (`004037d0`).
const STRING_LENGTH: u32 = 0x0040_48e0;
const COPY_ACTIVATION_PROMPT: u32 = 0x0051_10e0;
const STRING_TEXT: u32 = 0x0055_9450;
const STRING_DESTRUCT: u32 = 0x0040_37d0;
/// The forms `Save` writes by id, with their tags: +0x74 `SNAM`, +0x78
/// `VNAM`, +0x7c `INAM`, +0x80 `WNAM`, +0x84 `RNAM`.
const SAVED_FORMS: [(u32, u32); 5] = [
    (0x74, SNAM),
    (0x78, VNAM),
    (0x7c, INAM),
    (0x80, WNAM),
    (0x84, RNAM),
];

// Translated from 00510f00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00510f00 - the FS:[0] exception frame (it destroys the two string copies if a step throws) is not translated.
/// `TESObjectACTI::Save` (engine map; no stack words): `StartForm`, the full
/// name, the model with texture swaps (`MODL`, `MODT`, `MODS`), the script
/// and the destructible data; each set form of +0x74, +0x78, +0x7c, +0x80,
/// +0x84 by id (`SNAM`, `VNAM`, `INAM`, `WNAM`, `RNAM`); when the string at
/// +0x88 is not empty, a copy of it (`005110e0`) gives the length and a
/// second copy the text of chunk `XATO` (with its terminator); both copies
/// are destroyed; `CloseForm`. The two string copies the game keeps on its
/// stack are a block here.
pub fn tes_object_acti_save(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(START_FORM, &args![this]);
    e.call(FULL_NAME_SAVE, &args![base + 0x30]);
    e.call(
        MODEL_TEXTURE_SWAP_SAVE,
        &args![base + 0x3c, MODL, MODT, MODS],
    );
    e.call(SCRIPTABLE_FORM_SAVE, &args![base + 0x5c]);
    e.call(DESTRUCTIBLE_SAVE, &args![base + 0x68]);
    for (offset, tag) in SAVED_FORMS {
        let form = e.mem.u32(base + offset);
        if form != 0 {
            let id = e.call(FORM_ID, &args![form]).u32();
            e.call(ADD_CHUNK_WORD, &args![tag, id]);
        }
    }
    if e.call(STRING_LENGTH, &args![base + 0x88]).u32() != 0 {
        e.with_stack(0x10, |e, strings| {
            let first = strings.addr();
            let second = strings.addr() + 8;
            let copy = e.call(COPY_ACTIVATION_PROMPT, &args![this, first]).u32();
            let size = e.call(STRING_LENGTH, &args![copy]).u32().wrapping_add(1);
            let copy = e.call(COPY_ACTIVATION_PROMPT, &args![this, second]).u32();
            let text = e.call(STRING_TEXT, &args![copy]).u32();
            e.call(ADD_CHUNK_ARRAY, &args![XATO, text, size]);
            e.call(STRING_DESTRUCT, &args![second]);
            e.call(STRING_DESTRUCT, &args![first]);
        });
    }
    e.call(CLOSE_FORM, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0051_0a90, fn_00510a90(Ptr, Ptr) -> bool),
        entry!(0x0051_0770, tes_object_acti_init_item(Ptr)),
        entry!(0x0051_0ea0, fn_00510ea0(Ptr, u32, u32)),
        entry!(0x0051_0f00, tes_object_acti_save(Ptr)),
    ]
}

#[cfg(test)]
mod b023_tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn init_item_resolves_the_five_forms() {
        let mut e = Engine::new();
        e.register(0x0040_13e0, |_, _| ret(1));
        let this = e.mem.alloc(0x90);
        e.call_log = Some(vec![]);
        e.call(0x0051_0770, &[this]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);

        e.register(0x0040_13e0, |_, _| ret(0));
        for a in [0x0048_cd90, 0x0047_8be0, 0x0048_aa80, WARNING, 0x0048_4ab0] {
            e.register(a, |_, _| Ret::default());
        }
        e.register(GET_FILE, |_, _| ret(0x0400_0000));
        e.register(ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id + 0x100);
            Ret::default()
        });
        // Ids 1 (+0x100) and 4 resolve, the others not; the cast adds a mark.
        e.register(LOOKUP_FORM, |_, a| {
            ret(if a[0] == 0x101 || a[0] == 0x104 || a[0] == 0x105 {
                a[0] + 0x1000
            } else {
                0
            })
        });
        e.register(RT_DYNAMIC_CAST, |_, a| {
            ret(if a[0] == 0 { 0 } else { a[0] + 0x2_0000 })
        });
        e.register(0x004f_edc0, |_, _| ret(0x100));
        e.register(FORM_ID, |_, _| ret(0x00ab_cdef));
        e.register(0x7000_0000, |_, _| ret(0x0102_0304));
        let mut slots = vec![0u32; 0x134 / 4];
        slots[0x130 / 4] = 0x7000_0000;
        e.put_vtable(0x0600_0000, &slots);
        e.mem.set_u32(this, 0x0600_0000);
        // +0x74 = 1, +0x78 = 0 (skipped), +0x7c = 3 (missing), +0x80 = 4,
        // +0x84 = 5 (found, but 004fedc0 answers false).
        for (offset, id) in [(0x74, 1), (0x78, 0), (0x7c, 3), (0x80, 4), (0x84, 5)] {
            e.mem.set_u32(this + offset, id);
        }
        e.call_log = Some(vec![]);
        e.call(0x0051_0770, &[this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0048_cd90), vec![vec![this + 0x5c, this]]);
        assert_eq!(calls_to(&log, 0x0047_8be0), vec![vec![this + 0x68, this]]);
        assert_eq!(calls_to(&log, 0x0048_aa80), vec![vec![this + 0x3c, this]]);
        assert_eq!(e.mem.u32(this + 0x74), 0x2_1101);
        assert_eq!(e.mem.u32(this + 0x78), 0);
        assert_eq!(e.mem.u32(this + 0x7c), 0);
        assert_eq!(e.mem.u32(this + 0x80), 0x2_1104);
        assert_eq!(e.mem.u32(this + 0x84), 0);
        assert_eq!(calls_to(&log, 0x004f_edc0), vec![vec![0x2_1105]]);
        let casts: Vec<u32> = calls_to(&log, RT_DYNAMIC_CAST)
            .iter()
            .map(|w| w[3])
            .collect();
        assert_eq!(
            casts,
            vec![0x0118_32fc, 0x0118_32fc, 0x0118_4118, 0x0118_9e04]
        );
        assert_eq!(
            calls_to(&log, WARNING),
            vec![
                vec![0x0102_9f88, 0x103, 0x0102_0304, 0x00ab_cdef],
                vec![0x0102_9ef0, 0x105, 0x0102_0304, 0x00ab_cdef],
            ]
        );
        assert_eq!(calls_to(&log, 0x0048_4ab0), vec![vec![this, 1]]);

        // A radio station that is not found is reported without asking
        // 004fedc0; one it accepts stays.
        e.mem.write(this + 0x74, &[0; 0x14]);
        e.mem.set_u32(this + 0x84, 9);
        e.call_log = Some(vec![]);
        e.call(0x0051_0770, &[this]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, 0x004f_edc0).is_empty());
        assert_eq!(calls_to(&log, WARNING).len(), 1);
        e.register(0x004f_edc0, |_, _| ret(1));
        e.mem.set_u32(this + 0x84, 5);
        e.call_log = Some(vec![]);
        e.call(0x0051_0770, &[this]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, WARNING).is_empty());
        assert_eq!(e.mem.u32(this + 0x84), 0x2_1105);
    }

    #[test]
    fn the_string_argument_is_set_then_destroyed() {
        let mut e = Engine::new();
        e.register(0x0043_8470, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0x1111_2222);
            assert_eq!(e.mem.u32(a[1] + 4), 0x3333_4444);
            Ret::default()
        });
        e.register(0x0040_37d0, |_, _| Ret::default());
        let this = e.mem.alloc(0x90);
        e.call_log = Some(vec![]);
        e.call(0x0051_0ea0, &[this, 0x1111_2222, 0x3333_4444]);
        let log = e.call_log.take().unwrap();
        let set = calls_to(&log, 0x0043_8470);
        assert_eq!(set[0][0], this + 0x88);
        assert_eq!(calls_to(&log, 0x0040_37d0), vec![vec![set[0][1]]]);
    }
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
                0x006e_cd40,
                0x0048_cd90,
                0x0047_81e0,
                0x004f_ebd0,
                0x004f_b090,
                0x004f_ebb0,
                0x0044_2a80,
                0x008a_c890,
                0x0051_0ea0,
                0x0040_1030,
            ],
        );
        // `00401000` hands out a block, `0040c0e0` builds the two words of the string in `text`.
        e.register(0x0040_1000, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(0x0040_c0e0, |e, a| {
            e.mem.set_u32(a[0], 0x1111_2222);
            e.mem.set_u32(a[0] + 4, 0x3333_4444);
            Ret::default()
        });
        let this = load_test::object(
            &mut e,
            0x70,
            0x0900_0000,
            &[(0xe0, 0x0090_0500), (0x134, 0x0090_0600)],
        );
        let (ret, calls) = run(&mut e, 0x0051_0a90, &[this, FILE]);
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
            (tag(b"DSTD"), 0x0047_81e0, 0x68),
            (tag(b"DEST"), 0x0047_81e0, 0x68),
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
            [vec![this + 0x5c, CHUNK_VALUE]]
        );
        assert_eq!(calls_to(&calls, 0x0048_cd90), [vec![this + 0x5c, this]]);
        for (chunk, callee) in [
            (tag(b"INAM"), 0x004f_ebd0),
            (tag(b"RNAM"), 0x004f_b090),
            (tag(b"SNAM"), 0x004f_ebb0),
            (tag(b"VNAM"), 0x0044_2a80),
            (tag(b"WNAM"), 0x008a_c890),
        ] {
            let (_, this, _, calls) = load(&[chunk]);
            assert_eq!(calls_to(&calls, callee), [vec![this, CHUNK_VALUE]]);
        }
        // `TNAM` and `UNAM` are not read.
        let (_, _, _, calls) = load(&[tag(b"TNAM"), tag(b"UNAM")]);
        assert_eq!(calls.len(), 2);
    }

    #[test]
    fn the_xato_chunk_becomes_a_string_passed_by_value() {
        let (_, this, _, calls) = load(&[tag(b"XATO")]);
        let read = calls_to(&calls, 0x0047_2890);
        assert_eq!((read[0][0], read[0][2]), (FILE, CHUNK_SIZE));
        let buffer = read[0][1];
        assert_eq!(
            calls_to(&calls, 0x0051_0ea0),
            [vec![this, 0x1111_2222, 0x3333_4444]]
        );
        assert_eq!(calls_to(&calls, 0x0040_1030), [vec![buffer]]);
        assert_eq!(calls_to(&calls, 0x0040_1000), [vec![CHUNK_SIZE]]);
    }

    #[test]
    fn a_one_byte_xato_chunk_is_ignored() {
        let mut e = load_test::engine(0, &[tag(b"XATO")], false);
        stub(&mut e, &[0x0048_5110, 0x0048_4ab0]);
        e.register(0x0040_1660, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x70);
        let (_, calls) = run(&mut e, 0x0051_0a90, &[this, FILE]);
        assert_eq!(calls.len(), 2);
    }
}

#[cfg(test)]
mod save_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn only(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn engine(prompt_length: u32) -> Engine {
        let mut e = Engine::new();
        for a in [
            START_FORM,
            CLOSE_FORM,
            FULL_NAME_SAVE,
            MODEL_TEXTURE_SWAP_SAVE,
            SCRIPTABLE_FORM_SAVE,
            DESTRUCTIBLE_SAVE,
            ADD_CHUNK_WORD,
            ADD_CHUNK_ARRAY,
            STRING_DESTRUCT,
        ] {
            e.register(a, |_, _| Ret::default());
        }
        e.register(FORM_ID, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register_double(STRING_LENGTH, move |_, _| prompt_length.into_ret());
        e.register(COPY_ACTIVATION_PROMPT, |_, a| a[1].into_ret());
        e.register(STRING_TEXT, |_, a| (a[0] + 0x1000).into_ret());
        e
    }

    #[test]
    fn save_writes_components_forms_and_the_prompt() {
        let mut e = engine(5);
        let this = e.mem.alloc(0x90);
        let sound = e.mem.alloc(0x18);
        e.mem.set_u32(sound + 0xc, 0x0000_0123);
        e.mem.set_u32(this + 0x74, sound);
        e.mem.set_u32(this + 0x84, sound);
        e.call_log = Some(vec![]);
        e.call(0x0051_0f00, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(only(&log, START_FORM), vec![vec![this]]);
        assert_eq!(only(&log, FULL_NAME_SAVE), vec![vec![this + 0x30]]);
        assert_eq!(
            only(&log, MODEL_TEXTURE_SWAP_SAVE),
            vec![vec![this + 0x3c, MODL, MODT, MODS]]
        );
        assert_eq!(only(&log, SCRIPTABLE_FORM_SAVE), vec![vec![this + 0x5c]]);
        assert_eq!(only(&log, DESTRUCTIBLE_SAVE), vec![vec![this + 0x68]]);
        assert_eq!(
            only(&log, ADD_CHUNK_WORD),
            vec![vec![SNAM, 0x123], vec![RNAM, 0x123]]
        );
        let copies = only(&log, COPY_ACTIVATION_PROMPT);
        assert_eq!(copies.len(), 2);
        let (first, second) = (copies[0][1], copies[1][1]);
        assert_eq!(second, first + 8);
        assert_eq!(
            only(&log, ADD_CHUNK_ARRAY),
            vec![vec![XATO, second + 0x1000, 6]]
        );
        assert_eq!(only(&log, STRING_DESTRUCT), vec![vec![second], vec![first]]);
        assert_eq!(only(&log, CLOSE_FORM), vec![vec![this]]);
    }

    #[test]
    fn save_skips_an_empty_prompt() {
        let mut e = engine(0);
        let this = e.mem.alloc(0x90);
        e.call_log = Some(vec![]);
        e.call(0x0051_0f00, &args![this]);
        let log = e.call_log.take().unwrap();
        assert!(only(&log, COPY_ACTIVATION_PROMPT).is_empty());
        assert!(only(&log, ADD_CHUNK_ARRAY).is_empty());
        assert!(only(&log, ADD_CHUNK_WORD).is_empty());
        assert_eq!(only(&log, CLOSE_FORM).len(), 1);
    }
}
