//! `fallout shared/run_005f0040` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005f0380 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `005f0820`, which calls `005f0860` for each index
/// 0 to 0x15, and returns nothing. Parameters: one word, `this`; the disassembly of the
/// function ends in a plain `RET` and passes only `ECX` on. `005f0820`'s disassembly is a
/// counting loop that ends on its exit test without loading a result (Ghidra types it void).
pub fn fn_005f0380(e: &mut Engine, this: Ptr) {
    e.call(0x005f_0820, &args![this]);
}

/// The PC `BGSVoiceType`'s editor ID string (`BSStringT` at +0x1C; the Xbox
/// PDB keeps the editor ID in `TESForm`).
const VOICE_EDITOR_ID: u32 = 0x1c;
/// `TESFile::GetTESForm` (Xbox PDB, `00472660`).
const FILE_GET_FORM_TYPE: u32 = 0x0047_2660;
/// `TESForm::LoadForm` (Xbox PDB, `00485110`).
const FORM_LOAD_FORM: u32 = 0x0048_5110;
/// `TESFile::GetTESChunk` (Xbox PDB, `004726b0`).
const FILE_GET_CHUNK: u32 = 0x0047_26b0;
/// `004726f0` (`ECX` = file): moves to the next chunk, false at the end.
const FILE_NEXT_CHUNK: u32 = 0x0047_26f0;
/// `00401660` (`ECX` = file): the current chunk's size.
const FILE_CHUNK_SIZE: u32 = 0x0040_1660;
/// `TESFile::GetChunkData_ov3` (Xbox PDB, `00472890`): the destination and
/// the most bytes to read.
const FILE_GET_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
const SIG_EDID: u32 = 0x4449_4445;
const SIG_OBND: u32 = 0x444e_424f;
const SIG_DNAM: u32 = 0x4d41_4e44;

// Translated from 005f0040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSVoiceType`'s vtable slot 0x134 (`SetEditorID` in the Xbox PDB's
/// `TESForm` vtable): sets the editor ID string (`004037f0` with `id`, 0).
/// Returns true.
pub fn fn_005f0040(e: &mut Engine, this: Ptr, id: Ptr) -> bool {
    e.call(
        0x0040_37f0,
        &args![this.byte_add(VOICE_EDITOR_ID), id, 0u32],
    );
    true
}

// Translated from 005f0060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSVoiceType`'s vtable slot 0x130 (`GetEditorID` in the Xbox PDB): the
/// editor ID string's text (`00559450`, called again for the result), or
/// the empty string at `01011584` when that is null.
pub fn fn_005f0060(e: &mut Engine, this: Ptr) -> u32 {
    let text = this.byte_add(VOICE_EDITOR_ID);
    if e.call(0x0055_9450, &args![text]).u32() == 0 {
        0x0101_1584
    } else {
        e.call(0x0055_9450, &args![text]).u32()
    }
}

// Translated from 005f00d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005f00d0 - the exception-unwinding frame (FS:[0]) and its state numbers are not translated
/// `BGSVoiceType`'s destructor body: stores its vtable, runs `00483710`
/// on `this`, destroys the editor ID string (`004037d0`) and the `TESForm`
/// base (`TESForm::~TESForm`, `00483630`).
pub fn fn_005f00d0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0104_733c);
    e.call(0x0048_3710, &args![this]);
    e.call(0x0040_37d0, &args![this.byte_add(VOICE_EDITOR_ID)]);
    e.call(0x0048_3630, &args![this]);
}

// Translated from 005f00a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSVoiceType`'s scalar deleting destructor: the destructor body
/// (`005f00d0`), then frees `this` (`00401030`) when bit 0 of `flags` is
/// set. Returns `this`.
pub fn fn_005f00a0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_005f00d0(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 005f01a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005f01a0 - the /GS cookie check (00ec408c) is not translated; the EDID buffer from _RtlCheckStack12 (00ec5ec0, the _alloca) is a with_stack block of the chunk size (its alignment, and the stack growing with each EDID chunk, are not modelled)
/// `BGSVoiceType`'s `Load` (vtable slot 0x20): false unless the record is
/// type 0x5D. Otherwise `TESForm::LoadForm`, then every chunk: `EDID` into
/// a stack buffer of the chunk's size (at most 0x200 bytes) given to
/// vtable slot 0x134, `OBND` to vtable slot 0xE0 with the file, `DNAM` one
/// byte into +0x18. Returns true.
pub fn fn_005f01a0(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    if e.call(FILE_GET_FORM_TYPE, &args![file]).u32() != 0x5d {
        return false;
    }
    e.call(FORM_LOAD_FORM, &args![this, file]);
    loop {
        let chunk = e.call(FILE_GET_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            SIG_EDID => {
                let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
                e.with_stack(size, |e, buffer| {
                    e.call(FILE_GET_CHUNK_DATA_SIZED, &args![file, buffer, 0x200u32]);
                    e.vcall(this.addr(), 0x134, &args![buffer]);
                });
            }
            SIG_OBND => {
                e.vcall(this.addr(), 0xe0, &args![file]);
            }
            SIG_DNAM => {
                e.call(
                    FILE_GET_CHUNK_DATA_SIZED,
                    &args![file, this.byte_add(0x18), 1u32],
                );
            }
            _ => {}
        }
        if !e.call(FILE_NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

// Translated from 005f0360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears 0x58 bytes at `this`: `00403d30` (`cdecl`: `this`, 0, 0x58).
/// Returns `this`.
pub fn fn_005f0360(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0040_3d30, &args![this, 0u32, 0x58u32]);
    this
}

/// `__RTDynamicCast` (`00ec43fb`, cdecl), the source type descriptor and
/// the target `005f02a0` and `005f02f0` pass (`01186318`, the type
/// descriptor named `.?AVBGSVoiceType@@`).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const CAST_SOURCE: u32 = 0x0118_3028;
const VOICE_TYPE_TYPE: u32 = 0x0118_6318;
/// Whether the file's words are swapped (`00401500`, no arguments).
const SWAPPED: u32 = 0x0040_1500;
/// The chunk writers (cdecl): a byte (`004858f0`), a word (`00485910`).
const WRITE_BYTE_CHUNK: u32 = 0x0048_58f0;
const WRITE_WORD_CHUNK: u32 = 0x0048_5910;

// Translated from 005f0140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSVoiceType`'s record writer (no stack words; the engine map has no
/// name): `TESForm::StartForm`, the byte at +0x18 as `DNAM`
/// (`TESForm::__AddChunkData('DNAM', +0x18, 1)`, cdecl), with `00483710`
/// run on it before and after when `00401500` answers true, then
/// `TESForm::CloseForm`.
pub fn fn_005f0140(e: &mut Engine, this: Ptr) {
    let data = this.addr() + 0x18;
    e.call(0x0048_55a0, &args![this]);
    if e.call(SWAPPED, &args![]).bool() {
        e.call(0x0048_3710, &args![data]);
    }
    e.call(0x0048_5990, &args![SIG_DNAM, data, 1u32]);
    if e.call(SWAPPED, &args![]).bool() {
        e.call(0x0048_3710, &args![data]);
    }
    e.call(0x0048_5680, &args![this]);
}

// Translated from 005f02a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSVoiceType`'s copy (`RET 4`: the source form; the engine map has no
/// name): the source through `__RTDynamicCast(source, 0, 01183028,
/// 01186318, 0)`; nothing without it, otherwise the byte at +0x18 and
/// `TESForm::CopyAllComponents` given the uncast source.
pub fn fn_005f02a0(e: &mut Engine, this: Ptr, source: u32) {
    let other = e
        .call(
            DYNAMIC_CAST,
            &args![source, 0u32, CAST_SOURCE, VOICE_TYPE_TYPE, 0u32],
        )
        .u32();
    if other == 0 {
        return;
    }
    let byte = e.mem.u8(other + 0x18);
    e.mem.set_u8(this.addr() + 0x18, byte);
    e.call(0x0048_51b0, &args![this, source]);
}

// Translated from 005f02f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSVoiceType`'s compare (`RET 4`: the other form; true when they
/// differ; the engine map has no name): true when the cast as in
/// `005f02a0` fails or the bytes at +0x18 differ, otherwise
/// `TESForm::CompareAllComponents`.
pub fn fn_005f02f0(e: &mut Engine, this: Ptr, form: u32) -> bool {
    let other = e
        .call(
            DYNAMIC_CAST,
            &args![form, 0u32, CAST_SOURCE, VOICE_TYPE_TYPE, 0u32],
        )
        .u32();
    if other == 0 {
        return true;
    }
    if e.mem.i8(this.addr() + 0x18) != e.mem.i8(other + 0x18) {
        return true;
    }
    e.call(0x0048_5270, &args![this, other]).bool()
}

// Translated from 005f03a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CreatureSounds`' chunk writer (no stack words; the engine map has no
/// name): for each sound type 0 to 0x15 whose list (`005f0ac0`) is set and
/// not empty, the type as `CSDT`, then for each item up to an empty node
/// the sound's form id (`0084e3a0`, 0 without a sound) as `CSDI` and the
/// chance byte (+4) as `CSDC`.
pub fn fn_005f03a0(e: &mut Engine, this: Ptr) {
    let mut kind = 0i32;
    while kind < 0x16 {
        let mut node = e.call(0x005f_0ac0, &args![this, kind]).u32();
        if node != 0 && !e.call(0x0082_56d0, &args![node]).bool() {
            e.call(WRITE_WORD_CHUNK, &args![0x5444_5343u32, kind]);
            while node != 0 && !e.call(0x0082_56d0, &args![node]).bool() {
                let slot = e.call(0x0068_15c0, &args![node]).u32();
                let sound = e.mem.u32(slot);
                let mut id = 0u32;
                let form = e.mem.u32(sound);
                if form != 0 {
                    id = e.call(0x0084_e3a0, &args![form]).u32();
                }
                e.call(WRITE_WORD_CHUNK, &args![0x4944_5343u32, id]);
                let chance = e.mem.u8(sound + 4);
                e.call(WRITE_BYTE_CHUNK, &args![0x4344_5343u32, u32::from(chance)]);
                node = e.call(0x0072_6070, &args![node]).u32();
            }
        }
        kind += 1;
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005f_0140, fn_005f0140(Ptr)),
        entry!(0x005f_02a0, fn_005f02a0(Ptr, u32)),
        entry!(0x005f_02f0, fn_005f02f0(Ptr, u32) -> bool),
        entry!(0x005f_03a0, fn_005f03a0(Ptr)),
        entry!(0x005f_0380, fn_005f0380(Ptr)),
        entry!(0x005f_0040, fn_005f0040(Ptr, Ptr) -> bool),
        entry!(0x005f_0060, fn_005f0060(Ptr) -> u32),
        entry!(0x005f_00d0, fn_005f00d0(Ptr)),
        entry!(0x005f_00a0, fn_005f00a0(Ptr, u32) -> Ptr),
        entry!(0x005f_01a0, fn_005f01a0(Ptr, Ptr) -> bool),
        entry!(0x005f_0360, fn_005f0360(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod thunk_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fwd_a03_005f0380_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x005f_0820, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x005f_0380, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x005f_0820), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod b020_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls(e: &mut Engine, skip: u32) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 != skip)
            .collect()
    }

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    #[test]
    fn b020_005f0040_sets_the_editor_id() {
        let mut e = Engine::new();
        e.register(0x0040_37f0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert!(e.call(0x005f_0040, &args![0x3000u32, 0x7000u32]).bool());
        assert_eq!(
            calls(&mut e, 0x005f_0040),
            vec![(0x0040_37f0, vec![0x301c, 0x7000, 0])]
        );
    }

    #[test]
    fn b020_005f0060_answers_the_text_or_empty() {
        let mut e = Engine::new();
        e.register(0x0055_9450, |_, _| ret(0x7000));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005f_0060, &args![0x3000u32]).u32(), 0x7000);
        assert_eq!(
            calls(&mut e, 0x005f_0060),
            vec![(0x0055_9450, vec![0x301c]), (0x0055_9450, vec![0x301c])]
        );
        e.register(0x0055_9450, |_, _| ret(0));
        assert_eq!(e.call(0x005f_0060, &args![0x3000u32]).u32(), 0x0101_1584);
    }

    #[test]
    fn b020_005f00d0_and_005f00a0_destroy() {
        let mut e = Engine::new();
        for a in [0x0048_3710, 0x0040_37d0, 0x0048_3630, 0x0040_1030] {
            e.register(a, |_, _| Ret::default());
        }
        let p = e.mem.alloc(0x28);
        e.call_log = Some(vec![]);
        e.call(0x005f_00d0, &args![p]);
        assert_eq!(e.mem.u32(p), 0x0104_733c);
        let body = vec![
            (0x0048_3710, vec![p]),
            (0x0040_37d0, vec![p + 0x1c]),
            (0x0048_3630, vec![p]),
        ];
        assert_eq!(calls(&mut e, 0x005f_00d0), body);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005f_00a0, &args![p, 0u32]).u32(), p);
        assert_eq!(calls(&mut e, 0x005f_00a0), body);
        e.call_log = Some(vec![]);
        e.call(0x005f_00a0, &args![p, 1u32]);
        let mut freed = body.clone();
        freed.push((0x0040_1030, vec![p]));
        assert_eq!(calls(&mut e, 0x005f_00a0), freed);
    }

    #[test]
    fn b020_005f01a0_loads_the_chunks() {
        let mut e = Engine::new();
        e.register(FILE_GET_FORM_TYPE, |_, _| ret(0x5d));
        let queue = Rc::new(RefCell::new(vec![
            SIG_EDID,
            SIG_OBND,
            SIG_DNAM,
            0x1111_1111,
        ]));
        let q = queue.clone();
        e.register_double(FILE_GET_CHUNK, move |_, _| ret(q.borrow()[0]));
        e.register_double(FILE_NEXT_CHUNK, move |_, _| {
            let mut v = queue.borrow_mut();
            v.remove(0);
            ret(u32::from(!v.is_empty()))
        });
        e.register(FILE_CHUNK_SIZE, |_, _| ret(0x10));
        for a in [
            FORM_LOAD_FORM,
            FILE_GET_CHUNK_DATA_SIZED,
            0x0000_1134,
            0x0000_10e0,
        ] {
            e.register(a, |_, _| Ret::default());
        }
        e.put_vtable(0x0150_0000, &[0; 0x50]);
        e.mem.set_u32(0x0150_0134, 0x0000_1134);
        e.mem.set_u32(0x0150_00e0, 0x0000_10e0);
        let p = e.mem.alloc(0x28);
        e.mem.set_u32(p, 0x0150_0000);
        e.call_log = Some(vec![]);
        assert!(e.call(0x005f_01a0, &args![p, 0x9000u32]).bool());
        let log = calls(&mut e, 0x005f_01a0);
        let buffer = log
            .iter()
            .find(|c| c.0 == FILE_GET_CHUNK_DATA_SIZED)
            .unwrap()
            .1[1];
        let interesting: Vec<_> = log
            .into_iter()
            .filter(|c| c.0 != FILE_GET_CHUNK && c.0 != FILE_NEXT_CHUNK)
            .collect();
        assert_eq!(
            interesting,
            vec![
                (FILE_GET_FORM_TYPE, vec![0x9000]),
                (FORM_LOAD_FORM, vec![p, 0x9000]),
                (FILE_CHUNK_SIZE, vec![0x9000]),
                (FILE_GET_CHUNK_DATA_SIZED, vec![0x9000, buffer, 0x200]),
                (0x0000_1134, vec![p, buffer]),
                (0x0000_10e0, vec![p, 0x9000]),
                (FILE_GET_CHUNK_DATA_SIZED, vec![0x9000, p + 0x18, 1]),
            ]
        );
        e.register(FILE_GET_FORM_TYPE, |_, _| ret(0x15d));
        assert!(!e.call(0x005f_01a0, &args![p, 0x9000u32]).bool());
    }

    #[test]
    fn b020_005f0360_clears_0x58_bytes() {
        let mut e = Engine::new();
        e.register(0x0040_3d30, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005f_0360, &args![0x3000u32]).u32(), 0x3000);
        assert_eq!(
            calls(&mut e, 0x005f_0360),
            vec![(0x0040_3d30, vec![0x3000, 0, 0x58])]
        );
    }
}

#[cfg(test)]
mod n041_tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn voice_type_save_copy_and_compare() {
        let mut e = Engine::new();
        for a in [
            0x0048_55a0u32,
            0x0048_3710,
            0x0048_5990,
            0x0048_5680,
            0x0048_51b0,
        ] {
            e.register(a, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x20);
        for swap in [0u32, 1] {
            e.register_double(SWAPPED, move |_, _| ret(swap));
            e.call_log = Some(vec![]);
            e.call(0x005f_0140, &args![this]);
            assert_eq!(calls(&e, 0x0048_5990), vec![vec![SIG_DNAM, this + 0x18, 1]]);
            assert_eq!(calls(&e, 0x0048_3710).len(), 2 * swap as usize);
            assert_eq!(calls(&e, 0x0048_5680), vec![vec![this]]);
        }
        e.register(DYNAMIC_CAST, |_, a| ret(if a[0] == 1 { 0 } else { a[0] }));
        let other = e.mem.alloc(0x20);
        e.mem.set_u8(other + 0x18, 0x81);
        e.call_log = Some(vec![]);
        e.call(0x005f_02a0, &args![this, other]);
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![other, 0, CAST_SOURCE, VOICE_TYPE_TYPE, 0]]
        );
        assert_eq!(e.mem.u8(this + 0x18), 0x81);
        assert_eq!(calls(&e, 0x0048_51b0), vec![vec![this, other]]);
        e.call_log = Some(vec![]);
        e.call(0x005f_02a0, &args![this, 1u32]);
        assert!(calls(&e, 0x0048_51b0).is_empty());
        e.register(0x0048_5270, |_, _| ret(0));
        assert!(!e.call(0x005f_02f0, &args![this, other]).bool());
        assert!(e.call(0x005f_02f0, &args![this, 1u32]).bool());
        e.mem.set_u8(other + 0x18, 1);
        assert!(e.call(0x005f_02f0, &args![this, other]).bool());
        e.mem.set_u8(other + 0x18, 0x81);
        e.register(0x0048_5270, |_, _| ret(1));
        assert!(e.call(0x005f_02f0, &args![this, other]).bool());
    }

    #[test]
    fn fn_005f03a0_writes_each_sound_type() {
        let mut e = Engine::new();
        // Type 3 has two sounds (one without a form), type 5 an empty list.
        let this = e.mem.alloc(0x58);
        let first = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        let form = e.mem.alloc(0x10);
        e.mem.set_u32(first, form);
        e.mem.set_u8(first + 4, 40);
        e.mem.set_u8(second + 4, 60);
        let list = e.mem.alloc(8);
        let node = e.mem.alloc(8);
        e.mem.set_u32(list, first);
        e.mem.set_u32(list + 4, node);
        e.mem.set_u32(node, second);
        let empty = e.mem.alloc(8);
        e.mem.set_u32(this + 0xc, list);
        e.mem.set_u32(this + 0x14, empty);
        e.register(0x005f_0ac0, |e, a| ret(e.mem.u32(a[0] + 4 * a[1])));
        e.register(0x0082_56d0, |e, a| ret(u32::from(e.mem.u32(a[0]) == 0)));
        e.register(0x0068_15c0, |_, a| ret(a[0]));
        e.register(0x0072_6070, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(0x0084_e3a0, |_, _| ret(0xabcd));
        e.register(WRITE_WORD_CHUNK, |_, _| Ret::default());
        e.register(WRITE_BYTE_CHUNK, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x005f_03a0, &args![this]);
        assert_eq!(calls(&e, 0x005f_0ac0).len(), 0x16);
        assert_eq!(
            calls(&e, WRITE_WORD_CHUNK),
            vec![
                vec![0x5444_5343, 3],
                vec![0x4944_5343, 0xabcd],
                vec![0x4944_5343, 0]
            ]
        );
        assert_eq!(
            calls(&e, WRITE_BYTE_CHUNK),
            vec![vec![0x4344_5343, 40], vec![0x4344_5343, 60]]
        );
        assert_eq!(calls(&e, 0x0084_e3a0), vec![vec![form]]);
    }
}
