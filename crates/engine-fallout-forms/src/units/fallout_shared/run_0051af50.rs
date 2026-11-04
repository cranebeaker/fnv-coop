//! `fallout shared/run_0051af50` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0051b140 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0051b140 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x20 (0x58 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_0051b140(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_e820, &args![this]);
    e.call(0x0048_a3d0, &args![base + 0x30]);
    e.mem.set_u32(base, 0x0102_ba2c);
    e.mem.set_u32(base + 0x30, 0x0102_ba04);
    e.call(0x004f_15a0, &args![this, 0x20]);
    e.mem.set_u8(base + 0x50, 0xff);
    e.mem.set_u32(base + 0x54, 0x0);
    this
}

// Translated from 0051b130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x3c (`SUB ECX,0x3c`) and jumps to `0051a810`. `0051a810` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_0051b130(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x0051_a810, &args![this.addr().wrapping_sub(0x3c), flags])
        .u32()
}

/// Record tags (`u32::from_le_bytes` of the four characters) the loader `0051b280` tells apart.
const MODL: u32 = u32::from_le_bytes(*b"MODL");
const MODD: u32 = u32::from_le_bytes(*b"MODD");
const MOSD: u32 = u32::from_le_bytes(*b"MOSD");
const MODT: u32 = u32::from_le_bytes(*b"MODT");
const MODS: u32 = u32::from_le_bytes(*b"MODS");
const OBND: u32 = u32::from_le_bytes(*b"OBND");
const EDID: u32 = u32::from_le_bytes(*b"EDID");
const BRUS: u32 = u32::from_le_bytes(*b"BRUS");
const RNAM: u32 = u32::from_le_bytes(*b"RNAM");

// Translated from 0051b280 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0051b280 - the security-cookie check (00ec408c) at the end is not translated.
/// Record loader of the form `0051b140` constructs (form type 0x20; the engine map gives the
/// function no name, so the class is unconfirmed). `this` is the form and `file` the `TESFile`
/// positioned on its record. Always answers true.
///
/// It asks the file for the record's form type (`00472660`, the answer is not used), runs
/// `TESForm::LoadForm` (`00485110`) and then walks the record's chunks (`TESFile::GetTESChunk`
/// `004726b0` / `NextChunk` `004726f0`):
///
/// - `MODL`, `MODD`, `MOSD`, `MODT`: `TESModel::LoadModelChunk` (`004892d0`) on the model at
///   `+0x30` (null for a null form);
/// - `MODS`: reads the chunk and unpacks its texture swaps into the model at `+0x30`
///   (`0048a7b0` with the buffer, its size and the file's `AX` word at `00403570`);
/// - `OBND`: virtual slot `+0xe0` with the file;
/// - `EDID`: reads the chunk (limit 0x200) into a buffer and runs virtual slot `+0x134` on it;
/// - `BRUS`: reads a word with `TESFile::GetChunkData` and stores it as a byte at `+0x50`,
///   0xff when it is above 10;
/// - `RNAM`: reads a word into `+0x54`.
///
/// The buffers the game takes from the stack (`__alloca_probe_16`) are heap blocks here.
pub fn fn_0051b280(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let base = this.addr();
    e.call(0x0047_2660, &args![file]);
    e.call(0x0048_5110, &args![this, file]);
    loop {
        let chunk = e.call(0x0047_26b0, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            MODL | MODD | MOSD | MODT => {
                let model = if base != 0 { base + 0x30 } else { 0 };
                e.call(0x0048_92d0, &args![model, file]);
            }
            OBND => {
                e.vcall(base, 0xe0, &args![file]);
            }
            EDID => {
                let size = e.call(0x0040_1660, &args![file]).u32();
                let buffer = e.mem.alloc(size);
                e.call(0x0047_2890, &args![file, buffer, 0x200u32]);
                e.vcall(base, 0x134, &args![buffer]);
                e.mem.free(buffer);
            }
            MODS => {
                let size = e.call(0x0040_1660, &args![file]).u32();
                let buffer = e.mem.alloc(size);
                e.call(0x0047_2890, &args![file, buffer, size]);
                let swap_flags = e.call(0x0040_3570, &args![file]).u32() & 0xffff;
                e.call(0x0048_a7b0, &args![base + 0x30, buffer, size, swap_flags]);
                e.mem.free(buffer);
            }
            BRUS => {
                let value = e.mem.alloc(4);
                e.call(0x0047_27f0, &args![file, value]);
                let value_read = e.mem.u32(value);
                let stored = if value_read > 10 { 0xff } else { value_read };
                e.mem.set_u8(base + 0x50, stored as u8);
                e.mem.free(value);
            }
            RNAM => {
                let value = e.mem.alloc(4);
                e.call(0x0047_27f0, &args![file, value]);
                let value_read = e.mem.u32(value);
                e.mem.set_u32(base + 0x54, value_read);
                e.mem.free(value);
            }
            _ => {}
        }
        if !e.call(0x0047_26f0, &args![file]).bool() {
            break;
        }
    }
    true
}

// Translated from 0051b1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0051b200` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0051b1d0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0051_b200, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// The vtables `0051b200` stores (offset, vtable), in its order.
const DESTRUCTOR_VTABLES_0051B200: [(u32, u32); 2] = [(0, 0x0102_ba2c), (0x30, 0x0102_ba04)];
/// The member and base destructors `0051b200` runs (`thiscall`, no stack word), with the
/// offset each is called on, in its order: `00483710` (unit `tesform.cpp`) on `this`,
/// `0048a490` (unit `tesmodeltextureswap.cpp`) on +0x30, and the base `0050e9f0` (unit
/// `tesobject.cpp`) on `this`.
const DESTRUCTOR_STEPS_0051B200: [(u32, u32); 3] =
    [(0x0048_3710, 0), (0x0048_a490, 0x30), (0x0050_e9f0, 0)];

// Translated from 0051b200 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0051b200 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00ef35c3, state variable) is not translated
/// Unnamed in the engine map (`thiscall`, no stack word): the destructor of the object
/// `fn_0051b140` builds. Sets the vtables `0102ba2c` (+0) and `0102ba04` (+0x30), then runs
/// `00483710` on `this`, `0048a490` on +0x30 and the base destructor `0050e9f0` on `this`.
pub fn fn_0051b200(e: &mut Engine, this: Ptr) {
    for (offset, vtable) in DESTRUCTOR_VTABLES_0051B200 {
        e.mem.set_u32(this.addr() + offset, vtable);
    }
    for (destructor, offset) in DESTRUCTOR_STEPS_0051B200 {
        e.call(destructor, &args![this.addr() + offset]);
    }
}

/// `__RTDynamicCast` (`00ec43fb`, `cdecl`, five words) from `.?AVTESForm@@`
/// (`01183028`) to `.?AVTESObjectMISC@@` (`0118654c`).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_OBJECT_MISC: u32 = 0x0118_654c;
/// `TESForm::CopyAllComponents` and `CompareAllComponents` (`004851b0`,
/// `00485270`, engine map; `thiscall`, one word).
const COPY_ALL_COMPONENTS: u32 = 0x0048_51b0;
const COMPARE_ALL_COMPONENTS: u32 = 0x0048_5270;
/// `008cdd90` (`thiscall`, a word answer) and its setter `009db930`
/// (`thiscall`, one word).
const GET_008CDD90: u32 = 0x008c_dd90;
const SET_009DB930: u32 = 0x009d_b930;
/// `TESForm::StartForm` and `CloseForm` (`004855a0`, `00485680`, engine
/// map; `thiscall`), `TESModelTextureSwap::Save` (`0048a520`, three chunk
/// names), the chunk writers `004858f0` (`cdecl`: name, byte) and
/// `00485910` (`cdecl`: name, word), and `0084e3a0` (`thiscall`).
const START_FORM: u32 = 0x0048_55a0;
const CLOSE_FORM: u32 = 0x0048_5680;
const MODEL_SAVE: u32 = 0x0048_a520;
const WRITE_CHUNK_BYTE: u32 = 0x0048_58f0;
const WRITE_CHUNK_WORD: u32 = 0x0048_5910;
const MEMBER_0084E3A0: u32 = 0x0084_e3a0;

/// `form` cast to `TESObjectMISC`, or 0.
fn cast_to_misc(e: &mut Engine, form: u32) -> u32 {
    e.call(
        RT_DYNAMIC_CAST,
        &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_OBJECT_MISC, 0u32],
    )
    .u32()
}

// Translated from 0051af50 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`thiscall`, one word, `RET 4`; a copy): when `source` casts
/// to `TESObjectMISC`, sets `009db930` to what `008cdd90` answers for the
/// cast form, then `TESForm::CopyAllComponents(source)`.
pub fn fn_0051af50(e: &mut Engine, this: Ptr, source: u32) {
    let other = cast_to_misc(e, source);
    if other == 0 {
        return;
    }
    let value = e.call(GET_008CDD90, &args![other]).u32();
    e.call(SET_009DB930, &args![this, value]);
    e.call(COPY_ALL_COMPONENTS, &args![this, source]);
}

// Translated from 0051afb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`thiscall`, one word, `RET 4`): whether the form differs
/// from `form`: true when it does not cast to `TESObjectMISC` or the two
/// answer differently to `008cdd90`, else what
/// `TESForm::CompareAllComponents` answers (as 1 or 0).
pub fn fn_0051afb0(e: &mut Engine, this: Ptr, form: u32) -> bool {
    let other = cast_to_misc(e, form);
    if other == 0 {
        return true;
    }
    let mine = e.call(GET_008CDD90, &args![this]).u32();
    if mine != e.call(GET_008CDD90, &args![other]).u32() {
        return true;
    }
    e.call(COMPARE_ALL_COMPONENTS, &args![this, other]).bool()
}

// Translated from 0051b020 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (a save-size sum): the 16-bit sum of `00484bf0` on `this`
/// and `0048e9b0` on `this + 0x74`, each given `argument`. `RET 4`.
pub fn fn_0051b020(e: &mut Engine, this: Ptr, argument: u32) -> u16 {
    let mut size: u16 = 0;
    size = size.wrapping_add(e.call(0x0048_4bf0, &args![this, argument]).u16());
    let component = this.addr().wrapping_add(0x74);
    size.wrapping_add(e.call(0x0048_e9b0, &args![component, argument]).u16())
}

// Translated from 0051b070 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: `00484c20` on `this`, then `0048e9e0` on `this + 0x74`,
/// each given `argument`. `RET 4`.
pub fn fn_0051b070(e: &mut Engine, this: Ptr, argument: u32) {
    e.call(0x0048_4c20, &args![this, argument]);
    e.call(0x0048_e9e0, &args![this.addr() + 0x74, argument]);
}

// Translated from 0051b0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: `00484c50` on `this`, then `0048ea10` on `this + 0x74`,
/// each given both stack words. `RET 8`.
pub fn fn_0051b0a0(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    e.call(0x0048_4c50, &args![this, first, second]);
    e.call(0x0048_ea10, &args![this.addr() + 0x74, first, second]);
}

// Translated from 0051b0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: `00484d60` on `this`, then `0048ea40` on `this + 0x74`,
/// each given `argument`. `RET 4`.
pub fn fn_0051b0d0(e: &mut Engine, this: Ptr, argument: u32) {
    e.call(0x0048_4d60, &args![this, argument]);
    e.call(0x0048_ea40, &args![this.addr() + 0x74, argument]);
}

// Translated from 0051b100 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: `00484da0` on `this`, then `0048ea80` on `this + 0x74`,
/// each given `argument`. `RET 4`.
pub fn fn_0051b100(e: &mut Engine, this: Ptr, argument: u32) {
    e.call(0x0048_4da0, &args![this, argument]);
    e.call(0x0048_ea80, &args![this.addr() + 0x74, argument]);
}

// Translated from 0051b4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`thiscall`; the form's save): `StartForm`, the model at
/// +0x30 saved with the chunk names `MODL`, `MODT`, `MODS`, a `BRUS` chunk
/// with the byte at +0x50, an `RNAM` chunk with what `0084e3a0` answers
/// for the word at +0x54 when it is set, then `CloseForm`.
pub fn fn_0051b4b0(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    e.call(START_FORM, &args![this]);
    e.call(MODEL_SAVE, &args![at + 0x30, MODL, MODT, MODS]);
    let byte = u32::from(e.mem.u8(at + 0x50));
    e.call(WRITE_CHUNK_BYTE, &args![BRUS, byte]);
    let member = e.mem.u32(at + 0x54);
    if member != 0 {
        let value = e.call(MEMBER_0084E3A0, &args![member]).u32();
        e.call(WRITE_CHUNK_WORD, &args![RNAM, value]);
    }
    e.call(CLOSE_FORM, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0051_b140, fn_0051b140(Ptr) -> Ptr),
        entry!(0x0051_b130, fn_0051b130(Ptr, u32) -> u32),
        entry!(0x0051_b280, fn_0051b280(Ptr, Ptr) -> bool),
        entry!(0x0051_b1d0, fn_0051b1d0(Ptr, u32) -> Ptr),
        entry!(0x0051_b200, fn_0051b200(Ptr)),
        entry!(0x0051_af50, fn_0051af50(Ptr, u32)),
        entry!(0x0051_afb0, fn_0051afb0(Ptr, u32) -> bool),
        entry!(0x0051_b020, fn_0051b020(Ptr, u32) -> u16),
        entry!(0x0051_b070, fn_0051b070(Ptr, u32)),
        entry!(0x0051_b0a0, fn_0051b0a0(Ptr, u32, u32)),
        entry!(0x0051_b0d0, fn_0051b0d0(Ptr, u32)),
        entry!(0x0051_b100, fn_0051b100(Ptr, u32)),
        entry!(0x0051_b4b0, fn_0051b4b0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_0051b140_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0051_b140,
            size: 0x58,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_e820, &[A::T(0x0)]),
                (0x0048_a3d0, &[A::T(0x30)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x20)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_ba2c),
                (0x30, 4, 0x0102_ba04),
                (0x50, 1, 0xff),
                (0x54, 4, 0x0),
            ],
        });
    }
}

#[cfg(test)]
mod le81_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and `extra` stack words, with a
    /// double over `target` that returns `0xdeadbeef`; checks that the
    /// double ran once, with `this` moved back by `back` and `extra` passed on.
    fn check(thunk: u32, target: u32, back: u32, extra: &[u32]) -> Ret {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let mut words = vec![0x2000u32];
        words.extend_from_slice(extra);
        let result = e.call(thunk, &words);
        let log = e.call_log.take().unwrap();
        let mut expected = vec![0x2000 - back];
        expected.extend_from_slice(extra);
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, expected);
        result
    }

    #[test]
    fn le81_thunk_0051b130() {
        assert_eq!(
            check(0x0051_b130, 0x0051_a810, 0x3c, &[0x1]).u32(),
            0xdead_beef
        );
    }
}

#[cfg(test)]
mod movement_path_tests {
    use crate::prelude::*;
    use crate::units::fallout_shared::tesmodel::load_test::{
        self, calls_to, run, stub, tag, CHUNK_SIZE, CHUNK_VALUE, FILE,
    };

    /// Runs the loader on a form and a file holding `chunks`; returns the form, the answer and
    /// the calls. The form's virtual slots `+0xe0` and `+0x134` are the doubles `0x0090_0500`
    /// and `0x0090_0600`.
    fn load(chunks: &[u32]) -> (Engine, u32, bool, load_test::Calls) {
        let mut e = load_test::engine(0x20, chunks, false);
        stub(&mut e, &[0x0048_5110, 0x0048_92d0, 0x0048_a7b0]);
        let this = load_test::object(
            &mut e,
            0x60,
            0x0900_0000,
            &[(0xe0, 0x0090_0500), (0x134, 0x0090_0600)],
        );
        let (ret, calls) = run(&mut e, 0x0051_b280, &[this, FILE]);
        (e, this, ret.bool(), calls)
    }

    #[test]
    fn an_empty_record_loads_the_form_and_answers_true() {
        let (_, this, answer, calls) = load(&[]);
        assert!(answer);
        assert_eq!(calls, [(0x0048_5110, vec![this, FILE])]);
    }

    #[test]
    fn the_model_chunks_go_to_the_model_loader() {
        for chunk in [tag(b"MODL"), tag(b"MODD"), tag(b"MOSD"), tag(b"MODT")] {
            let (_, this, _, calls) = load(&[chunk]);
            assert_eq!(
                calls_to(&calls, 0x0048_92d0),
                [vec![this + 0x30, FILE]],
                "{chunk:08x}"
            );
            assert_eq!(calls.len(), 2);
        }
    }

    #[test]
    fn the_bound_chunk_runs_virtual_slot_0xe0() {
        let (_, this, _, calls) = load(&[tag(b"OBND")]);
        assert_eq!(calls_to(&calls, 0x0090_0500), [vec![this, FILE]]);
    }

    #[test]
    fn the_editor_id_is_read_into_a_buffer_and_given_to_slot_0x134() {
        let (e, this, _, calls) = load(&[tag(b"EDID")]);
        let read = calls_to(&calls, 0x0047_2890);
        assert_eq!(read.len(), 1);
        assert_eq!((read[0][0], read[0][2]), (FILE, 0x200));
        assert_eq!(calls_to(&calls, 0x0090_0600), [vec![this, read[0][1]]]);
        assert_eq!(e.mem.block_size(read[0][1]), None);
    }

    #[test]
    fn the_texture_swap_chunk_is_unpacked_with_the_files_flag_word() {
        let (_, this, _, calls) = load(&[tag(b"MODS")]);
        let read = calls_to(&calls, 0x0047_2890);
        assert_eq!((read[0][0], read[0][2]), (FILE, CHUNK_SIZE));
        // The unpacker gets the buffer, its size and the low word of the file's answer.
        assert_eq!(
            calls_to(&calls, 0x0048_a7b0),
            [vec![this + 0x30, read[0][1], CHUNK_SIZE, 0x1234]]
        );
    }

    #[test]
    fn the_brus_byte_is_capped_at_0xff_and_the_rnam_word_is_kept() {
        let (e, this, _, _) = load(&[tag(b"BRUS"), tag(b"RNAM")]);
        // The double writes 0x12345678: above 10, so 0xff.
        assert_eq!(e.mem.u8(this + 0x50), 0xff);
        assert_eq!(e.mem.u32(this + 0x54), CHUNK_VALUE);

        for (value, stored) in [(0u32, 0u8), (10, 10), (11, 0xff)] {
            let mut e = load_test::engine(0x20, &[tag(b"BRUS")], false);
            stub(&mut e, &[0x0048_5110]);
            e.register_double(0x0047_27f0, move |e, a| {
                e.mem.set_u32(a[1], value);
                Ret::default()
            });
            let this = e.mem.alloc(0x60);
            run(&mut e, 0x0051_b280, &[this, FILE]);
            assert_eq!(e.mem.u8(this + 0x50), stored, "{value}");
        }
    }

    #[test]
    fn unknown_chunks_are_skipped_and_the_walk_goes_on() {
        let (e, this, answer, calls) = load(&[tag(b"XXXX"), tag(b"RNAM")]);
        assert!(answer);
        assert_eq!(calls_to(&calls, 0x0047_27f0).len(), 1);
        assert_eq!(e.mem.u32(this + 0x54), CHUNK_VALUE);
    }
    #[test]
    fn at_0051b1d0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0051_b200, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0051_b1d0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0051_b1d0)
                .collect();
            let mut expected = vec![(0x0051_b200, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod tests_b043 {
    use super::*;

    #[test]
    fn fn_0051b200_sets_the_vtables_and_destroys_the_members() {
        let mut e = Engine::new();
        for (destructor, _) in DESTRUCTOR_STEPS_0051B200 {
            e.register(destructor, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x0051_b200, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (0x0048_3710, vec![this]),
                (0x0048_a490, vec![this + 0x30]),
                (0x0050_e9f0, vec![this]),
            ]
        );
        assert_eq!(e.mem.u32(this), 0x0102_ba2c);
        assert_eq!(e.mem.u32(this + 0x30), 0x0102_ba04);
    }
}

#[cfg(test)]
mod n040_tests {
    use super::*;

    fn calls_to(e: &mut Engine, targets: &[u32]) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| targets.contains(a))
            .collect()
    }

    #[test]
    fn fn_0051af50_and_0051afb0_copy_and_compare_misc_items() {
        let mut e = Engine::new();
        e.register(RT_DYNAMIC_CAST, |e, a| {
            assert_eq!(a[1..], [0, TYPE_TES_FORM, TYPE_TES_OBJECT_MISC, 0]);
            let form = if e.mem.u32(a[0] + 0xc) == 0 { a[0] } else { 0 };
            Ret {
                eax: form,
                ..Ret::default()
            }
        });
        e.register(COPY_ALL_COMPONENTS, |_, _| Ret::default());
        e.register(SET_009DB930, |_, _| Ret::default());
        e.register(GET_008CDD90, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        e.register(COMPARE_ALL_COMPONENTS, |e, a| Ret {
            eax: e.mem.u32(a[1] + 0x10),
            ..Ret::default()
        });
        let this = e.mem.alloc(0x20);
        let other = e.mem.alloc(0x20);
        e.mem.set_u32(other + 4, 0x44);
        e.mem.set_u32(other + 0xc, 1);
        e.call_log = Some(vec![]);
        e.call(0x0051_af50, &[this, other]);
        assert!(calls_to(&mut e, &[COPY_ALL_COMPONENTS, SET_009DB930]).is_empty());
        assert!(e.call(0x0051_afb0, &[this, other]).bool());
        e.mem.set_u32(other + 0xc, 0);
        e.call_log = Some(vec![]);
        e.call(0x0051_af50, &[this, other]);
        assert_eq!(
            calls_to(&mut e, &[COPY_ALL_COMPONENTS, SET_009DB930]),
            vec![
                (SET_009DB930, vec![this, 0x44]),
                (COPY_ALL_COMPONENTS, vec![this, other]),
            ]
        );
        assert!(e.call(0x0051_afb0, &[this, other]).bool());
        e.mem.set_u32(this + 4, 0x44);
        assert!(!e.call(0x0051_afb0, &[this, other]).bool());
        e.mem.set_u32(other + 0x10, 1);
        assert!(e.call(0x0051_afb0, &[this, other]).bool());
    }

    #[test]
    fn fn_0051b020_and_friends_use_the_component_at_0x74() {
        let cases: [(u32, u32, u32, &[u32]); 5] = [
            (0x0051_b020, 0x0048_4bf0, 0x0048_e9b0, &[7]),
            (0x0051_b070, 0x0048_4c20, 0x0048_e9e0, &[7]),
            (0x0051_b0a0, 0x0048_4c50, 0x0048_ea10, &[7, 8]),
            (0x0051_b0d0, 0x0048_4d60, 0x0048_ea40, &[7]),
            (0x0051_b100, 0x0048_4da0, 0x0048_ea80, &[7]),
        ];
        for (addr, form, component, extra) in cases {
            let mut e = Engine::new();
            e.register(form, |_, _| Ret {
                eax: 0xffff,
                ..Ret::default()
            });
            e.register(component, |_, _| Ret {
                eax: 3,
                ..Ret::default()
            });
            let mut words = vec![0x1000];
            words.extend_from_slice(extra);
            e.call_log = Some(vec![]);
            let result = e.call(addr, &words);
            if addr == 0x0051_b020 {
                assert_eq!(result.u16(), 2);
            }
            let mut moved = vec![0x1074];
            moved.extend_from_slice(extra);
            assert_eq!(
                calls_to(&mut e, &[form, component]),
                vec![(form, words.clone()), (component, moved)]
            );
        }
    }

    #[test]
    fn fn_0051b4b0_saves_model_brush_and_rnam() {
        let mut e = Engine::new();
        let targets = [
            START_FORM,
            MODEL_SAVE,
            WRITE_CHUNK_BYTE,
            MEMBER_0084E3A0,
            WRITE_CHUNK_WORD,
            CLOSE_FORM,
        ];
        for t in targets {
            e.register(t, |_, _| Ret {
                eax: 0x99,
                ..Ret::default()
            });
        }
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x50, 0xffff_ff07);
        e.call_log = Some(vec![]);
        e.call(0x0051_b4b0, &[this]);
        let model = vec![this + 0x30, 0x4c44_4f4d, 0x5444_4f4d, 0x5344_4f4d];
        assert_eq!(
            calls_to(&mut e, &targets),
            vec![
                (START_FORM, vec![this]),
                (MODEL_SAVE, model.clone()),
                (WRITE_CHUNK_BYTE, vec![0x5355_5242, 7]),
                (CLOSE_FORM, vec![this]),
            ]
        );
        e.mem.set_u32(this + 0x54, 0x6000);
        e.call_log = Some(vec![]);
        e.call(0x0051_b4b0, &[this]);
        assert_eq!(
            calls_to(&mut e, &targets),
            vec![
                (START_FORM, vec![this]),
                (MODEL_SAVE, model),
                (WRITE_CHUNK_BYTE, vec![0x5355_5242, 7]),
                (MEMBER_0084E3A0, vec![0x6000]),
                (WRITE_CHUNK_WORD, vec![0x4d41_4e52, 0x99]),
                (CLOSE_FORM, vec![this]),
            ]
        );
    }
}
