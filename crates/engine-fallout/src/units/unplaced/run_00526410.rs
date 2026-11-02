//! `(unplaced)/run_00526410` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005265c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005265c0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x65 (0x44 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0102_cd94`) is `BGSLightingTemplate`, so this is the constructor of `BGSLightingTemplate`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_005265c0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.mem.set_u32(base, 0x0102_cd94);
    e.call(0x0052_6640, &args![base + 0x18]);
    e.call(0x004f_15a0, &args![this, 0x65u32]);
    e.call(0x0052_6710, &args![this]);
    this
}

// Translated from 005266c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `005266f0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_005266c0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0052_66f0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// `BGSLightingTemplate`'s vtable (its RTTI name, read through the locator
/// at `0102cd90`).
const LIGHTING_TEMPLATE_VTABLE: u32 = 0x0102_cd94;
/// `TESForm::~TESForm` (Xbox PDB name of `00483630`; `ECX` = form).
const TES_FORM_DESTRUCTOR: u32 = 0x0048_3630;
/// `00403d30` (cdecl: destination, byte, count): fills memory.
const FILL_MEMORY: u32 = 0x0040_3d30;
/// `TESFile::GetTESForm` (Xbox PDB name of `00472660`; `ECX` = file): the
/// current record's form type.
const GET_TES_FORM: u32 = 0x0047_2660;
/// `TESForm::LoadForm` (Xbox PDB name of `00485110`; `ECX` = form, the file).
const TES_FORM_LOAD_FORM: u32 = 0x0048_5110;
/// `TESFile::GetTESChunk` (Xbox PDB name of `004726b0`; `ECX` = file).
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `004726f0` (`ECX` = file): moves to the next chunk; true when there is one.
const NEXT_CHUNK: u32 = 0x0047_26f0;
/// `00401660` (`ECX` = file): the current chunk's size.
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `TESFile::GetChunkData` (Xbox PDB overload `_ov3`, `00472890`): `thiscall`
/// (file; buffer, size).
const GET_CHUNK_DATA: u32 = 0x0047_2890;
/// `TESForm::LoadData` (Xbox PDB name of `004861f0`; `ECX` = form; file,
/// destination, size).
const TES_FORM_LOAD_DATA: u32 = 0x0048_61f0;
/// `00401680` (`ECX` = file): whether the data read needs its byte order
/// swapped.
const FILE_NEEDS_SWAP: u32 = 0x0040_1680;
/// `00526790` (`ECX` = the data at `+0x18`): the data block's byte swap.
const SWAP_DATA: u32 = 0x0052_6790;
/// The form type this loader reads (0x65).
const LIGHTING_TEMPLATE_FORM_TYPE: u32 = 0x65;
/// The most an `EDID` chunk is read (0x200 bytes).
const EDITOR_ID_MAXIMUM: u32 = 0x200;
/// Virtual slot `0x134`: called with the editor id's characters.
const SLOT_SET_EDITOR_ID: u32 = 0x134;
/// Virtual slot `0xe0`: called with the file for `OBND`.
const SLOT_LOAD_BOUNDS: u32 = 0xe0;
/// The chunk tags.
const CHUNK_DATA: u32 = 0x4154_4144;
const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_OBND: u32 = 0x444e_424f;

// Translated from 00526640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initialises the 0x2c-byte data block at `this` (the block at
/// `BGSLightingTemplate + 0x18`, which `005265c0` passes): words +0x00 to
/// +0x08 zero, floats +0x0c and +0x10 0.0, words +0x14 and +0x18 zero, floats
/// +0x1c 1.0, +0x20 0.0, +0x24 1.0, word +0x28 zero. Returns `this`. The
/// engine map gives it no name.
pub fn fn_00526640(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, 0);
    e.mem.set_u32(base + 0x4, 0);
    e.mem.set_u32(base + 0x8, 0);
    // FLDZ / FSTP float.
    e.mem.set_u32(base + 0xc, x87_store32(0.0));
    e.mem.set_u32(base + 0x10, x87_store32(0.0));
    e.mem.set_u32(base + 0x14, 0);
    e.mem.set_u32(base + 0x18, 0);
    // FLD1 / FSTP float.
    e.mem.set_u32(base + 0x1c, x87_store32(1.0));
    e.mem.set_u32(base + 0x20, x87_store32(0.0));
    e.mem.set_u32(base + 0x24, x87_store32(1.0));
    e.mem.set_u32(base + 0x28, 0);
    this
}

// Translated from 005266f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSLightingTemplate`'s destructor body (the vtable it stores, `0102cd94`,
/// is `BGSLightingTemplate`'s by RTTI): puts back the class vtable and runs
/// `TESForm::~TESForm` (`00483630`). The engine map gives it no name.
pub fn fn_005266f0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), LIGHTING_TEMPLATE_VTABLE);
    e.call(TES_FORM_DESTRUCTOR, &args![this]);
}

// Translated from 00526710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the 0x2c-byte data block at `this + 0x18` (`00403d30` with 0 and
/// 0x2c). The engine map gives it no name.
pub fn fn_00526710(e: &mut Engine, this: Ptr) {
    e.call(FILL_MEMORY, &args![this.addr() + 0x18, 0u32, 0x2cu32]);
}

// Translated from 005268a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005268a0 - the security-cookie check (00ec408c) is not translated.
/// `BGSLightingTemplate`'s record loader (`thiscall`, the file; `RET 4`; the
/// engine map gives it no name): false when the record is not of form type
/// 0x65; otherwise `TESForm::LoadForm`, then chunk by chunk until a zero tag
/// or no next chunk (`004726f0`):
/// - `DATA`: `TESForm::LoadData` into `this + 0x18`, 0x2c bytes;
/// - `EDID`: read into a stack buffer of the chunk's size (the `_alloca`
///   `00ec5ec0` rounds it up to 16 bytes), at most 0x200 bytes, then virtual
///   slot `0x134` with the buffer;
/// - `OBND`: virtual slot `0xe0` with the file.
///
/// After the chunks, when the data read needs its byte order swapped
/// (`00401680`), the block at `+0x18` is swapped (`00526790`). Returns true.
pub fn fn_005268a0(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let object = this.addr();
    if e.call(GET_TES_FORM, &args![file]).u32() != LIGHTING_TEMPLATE_FORM_TYPE {
        return false;
    }
    e.call(TES_FORM_LOAD_FORM, &args![this, file]);
    loop {
        let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            CHUNK_DATA => {
                e.call(
                    TES_FORM_LOAD_DATA,
                    &args![this, file, object + 0x18, 0x2cu32],
                );
            }
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
            _ => {}
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    if e.call(FILE_NEEDS_SWAP, &args![file]).bool() {
        e.call(SWAP_DATA, &args![object + 0x18]);
    }
    true
}

// Translated from 005269c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unless `004013e0` reports true for `this`, runs `00483710` on it and
/// `00484ab0` with 1. The engine map gives it no name.
pub fn fn_005269c0(e: &mut Engine, this: Ptr) {
    if e.call(0x0040_13e0, &args![this]).bool() {
        return;
    }
    e.call(0x0048_3710, &args![this]);
    e.call(0x0048_4ab0, &args![this, 1u32]);
}

/// The save buffer's change flags: `00428110` (`ECX` = buffer, one stack word: a local it
/// fills) and `0042ce30` (the same for a load buffer) answer the flags' address; `004280f0`
/// (`ECX` = that, one stack word: a mask) whether a bit of the mask is set (`AL`).
const CHANGE_FLAGS: u32 = 0x0042_8110;
const LOAD_CHANGE_FLAGS: u32 = 0x0042_ce30;
const FLAGS_TEST: u32 = 0x0042_80f0;
/// `BGSSaveGameBuffer`'s byte save (`00865e50`, `ECX` = buffer: data, size, 0) and
/// `BGSLoadGameBuffer`'s byte load (`00864980`, `ECX` = buffer: destination, size).
const SAVE_BYTES: u32 = 0x0086_5e50;
const LOAD_BYTES: u32 = 0x0086_4980;
/// The base save, load and revert bodies (`00484d60`, `00484da0`, `004534f0`; `ECX` = the
/// form, one stack word: the buffer).
const FORM_SAVE_GAME: u32 = 0x0048_4d60;
const FORM_LOAD_GAME: u32 = 0x0048_4da0;
const FORM_REVERT: u32 = 0x0045_34f0;
/// `TESForm::StartForm` (`004855a0`), `TESForm::__SaveData` (`00485e20`: data, size) and
/// `TESForm::CloseForm` (`00485680`), all `ECX` = the form.
const START_FORM: u32 = 0x0048_55a0;
const SAVE_DATA: u32 = 0x0048_5e20;
const CLOSE_FORM: u32 = 0x0048_5680;
/// `00401500`: the byte global that says the save is written byte-swapped (`AL`).
const SAVE_SWAPS: u32 = 0x0040_1500;
/// `00401080` (`cdecl`: address, byte): reverses a word's byte order.
const SWAP_WORD: u32 = 0x0040_1080;
/// `__RTDynamicCast` (`00ec43fb`), the RTTI type descriptors `.?AVTESForm@@` and
/// `.?AVBGSLightingTemplate@@`, `TESForm::CopyAllComponents` (`004851b0`),
/// `TESForm::CompareAllComponents` (`00485270`), `00401460` (copies bytes) and `memcmp`.
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_LIGHTING_TEMPLATE: u32 = 0x0118_a8d0;
const COPY_ALL_COMPONENTS: u32 = 0x0048_51b0;
const COMPARE_ALL_COMPONENTS: u32 = 0x0048_5270;
const COPY_BYTES: u32 = 0x0040_1460;
const MEMCMP: u32 = 0x00ec_4835;

/// Whether the buffer's change flags (through `flags_of`) have a bit of `mask`.
fn flags_have(e: &mut Engine, flags_of: u32, buffer: Ptr, mask: u32) -> bool {
    e.with_stack(8, |e, local| {
        let flags = e.call(flags_of, &args![buffer, local]).u32();
        e.call(FLAGS_TEST, &args![flags, mask]).u8() != 0
    })
}

// Translated from 00526410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEncounterZone`'s `SaveGame` slot (+0x54 of its vtable `0102cbbc`; unnamed in the
/// engine map; `ECX` and one stack word, `RET 4`): the base save (`00484d60`); with change
/// flag 2, the low bit of the byte at +0x1e (one byte); with change flag 0x80000000, the
/// 0x10 bytes at +0x20.
pub fn fn_00526410(e: &mut Engine, this: Ptr, buffer: Ptr) {
    e.call(FORM_SAVE_GAME, &args![this, buffer]);
    if flags_have(e, CHANGE_FLAGS, buffer, 2) {
        e.with_stack(4, |e, byte| {
            let flags = e.mem.u8(this.addr() + 0x1e) as i8 as i32;
            e.mem.set_u8(byte.addr(), (flags & 1) as u8);
            e.call(SAVE_BYTES, &args![buffer, byte, 1u32, 0u32]);
        });
    }
    if flags_have(e, CHANGE_FLAGS, buffer, 0x8000_0000) {
        e.call(
            SAVE_BYTES,
            &args![buffer, this.addr() + 0x20, 0x10u32, 0u32],
        );
    }
}

// Translated from 005264a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005264a0 - the /GS cookie check (00ec408c) on the frame's buffers is not translated.
/// `BGSEncounterZone`'s `LoadGame` slot (+0x5c of its vtable; unnamed in the engine map;
/// `ECX` and one stack word, `RET 4`): the base load (`00484da0`); with change flag 2, a
/// byte read into a local (the byte at +0x1e is rewritten from itself, unchanged: the exe
/// ORs its own low bit back in, not the byte read); with change flag 0x80000000, the 0x10
/// bytes at +0x20, or 8 bytes into a local when the buffer's version (its vtable slot +0,
/// `AL`) is below 8.
pub fn fn_005264a0(e: &mut Engine, this: Ptr, buffer: Ptr) {
    e.call(FORM_LOAD_GAME, &args![this, buffer]);
    let base = this.addr();
    e.with_stack(0xc, |e, frame| {
        let byte = frame.addr();
        let old = frame.addr() + 4;
        if flags_have(e, CHANGE_FLAGS, buffer, 2) {
            e.mem.set_u8(byte, 0);
            e.call(LOAD_BYTES, &args![buffer, byte, 1u32]);
            let high = (e.mem.u8(base + 0x1e) as i8 as i32) & !1;
            let low = (e.mem.u8(base + 0x1e) as i8 as i32) & 1;
            e.mem.set_u8(base + 0x1e, (high | low) as u8);
        }
        if flags_have(e, CHANGE_FLAGS, buffer, 0x8000_0000) {
            let version = e.vcall(buffer.addr(), 0, &args![]).u8();
            if version < 8 {
                e.call(LOAD_BYTES, &args![buffer, old, 8u32]);
            } else {
                e.call(LOAD_BYTES, &args![buffer, base + 0x20, 0x10u32]);
            }
        }
    });
}

// Translated from 00526570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEncounterZone`'s `Revert` slot (+0x70 of its vtable; unnamed in the engine map;
/// `ECX` and one stack word, `RET 4`): the base revert (`004534f0`); with change flag
/// 0x80000000 (`0042ce30`), zeroes the 0x10 bytes at +0x20 (`00403d30`).
pub fn fn_00526570(e: &mut Engine, this: Ptr, buffer: Ptr) {
    e.call(FORM_REVERT, &args![this, buffer]);
    if flags_have(e, LOAD_CHANGE_FLAGS, buffer, 0x8000_0000) {
        e.call(FILL_MEMORY, &args![this.addr() + 0x20, 0u32, 0x10u32]);
    }
}

// Translated from 00526730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSLightingTemplate`'s form-save slot (+0x2c of its vtable `0102cd94`; unnamed in the
/// engine map; `ECX`, plain `RET`): `TESForm::StartForm`, the 0x2c-byte data at +0x18
/// (`TESForm::__SaveData`), byte-swapped before and back after when `00401500` says so
/// ([`fn_00526790`]), then `TESForm::CloseForm`.
pub fn fn_00526730(e: &mut Engine, this: Ptr) {
    let data = Ptr::new(this.addr() + 0x18);
    e.call(START_FORM, &args![this]);
    if e.call(SAVE_SWAPS, &args![]).u8() != 0 {
        fn_00526790(e, data);
    }
    e.call(SAVE_DATA, &args![this, data, 0x2cu32]);
    if e.call(SAVE_SWAPS, &args![]).u8() != 0 {
        fn_00526790(e, data);
    }
    e.call(CLOSE_FORM, &args![this]);
}

// Translated from 00526790 (decompiled, FalloutNV.exe 1.4.0.525)
/// The lighting template data's byte swap (unnamed in the engine map; `ECX`, plain `RET`;
/// also called by `005268a0`, `00541ce0`, `005422d0`, ...): reverses the byte order of the
/// ten words at +0 to +0x24 (`00401080`, each with the byte 0 the body zeroes).
pub fn fn_00526790(e: &mut Engine, this: Ptr) {
    for word in 0..10u32 {
        e.call(SWAP_WORD, &args![this.addr() + 4 * word, 0u32]);
    }
}

// Translated from 005269f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSLightingTemplate`'s `Copy` slot (+0x108 of its vtable; unnamed in the engine map;
/// `ECX` and one stack word, `RET 4`): when `source` casts to a `BGSLightingTemplate`,
/// copies its 0x2c bytes at +0x18 (`00401460`), then `TESForm::CopyAllComponents` from
/// `source`.
pub fn fn_005269f0(e: &mut Engine, this: Ptr, source: Ptr) {
    let template = e
        .call(
            DYNAMIC_CAST,
            &args![source, 0u32, TYPE_TES_FORM, TYPE_LIGHTING_TEMPLATE, 0u32],
        )
        .u32();
    if template == 0 {
        return;
    }
    e.call(
        COPY_BYTES,
        &args![this.addr() + 0x18, template + 0x18, 0x2cu32],
    );
    e.call(COPY_ALL_COMPONENTS, &args![this, source]);
}

// Translated from 00526a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSLightingTemplate`'s `Compare` slot (+0x10c of its vtable; unnamed in the engine map;
/// `ECX` and one stack word, `RET 4`): true (different) when `other` is not a
/// `BGSLightingTemplate`, when the 0x2c bytes at +0x18 differ (`memcmp`) or when
/// `TESForm::CompareAllComponents` answers true; false otherwise.
pub fn fn_00526a50(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let template = e
        .call(
            DYNAMIC_CAST,
            &args![other, 0u32, TYPE_TES_FORM, TYPE_LIGHTING_TEMPLATE, 0u32],
        )
        .u32();
    if template == 0 {
        return true;
    }
    if e.call(MEMCMP, &args![this.addr() + 0x18, template + 0x18, 0x2cu32])
        .u32()
        != 0
    {
        return true;
    }
    e.call(COMPARE_ALL_COMPONENTS, &args![this, template]).u8() != 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0052_6410, fn_00526410(Ptr, Ptr)),
        entry!(0x0052_64a0, fn_005264a0(Ptr, Ptr)),
        entry!(0x0052_6570, fn_00526570(Ptr, Ptr)),
        entry!(0x0052_6730, fn_00526730(Ptr)),
        entry!(0x0052_6790, fn_00526790(Ptr)),
        entry!(0x0052_69f0, fn_005269f0(Ptr, Ptr)),
        entry!(0x0052_6a50, fn_00526a50(Ptr, Ptr) -> bool),
        entry!(0x0052_65c0, fn_005265c0(Ptr) -> Ptr),
        entry!(0x0052_66c0, fn_005266c0(Ptr, u32) -> Ptr),
        entry!(0x0052_6640, fn_00526640(Ptr) -> Ptr),
        entry!(0x0052_66f0, fn_005266f0(Ptr)),
        entry!(0x0052_6710, fn_00526710(Ptr)),
        entry!(0x0052_68a0, fn_005268a0(Ptr, Ptr) -> bool),
        entry!(0x0052_69c0, fn_005269c0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_005265c0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0052_65c0,
            size: 0x44,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0052_6640, &[A::T(0x18)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x65)]),
                (0x0052_6710, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0102_cd94)],
        });
    }
    #[test]
    fn at_005266c0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0052_66f0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0052_66c0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0052_66c0)
                .collect();
            let mut expected = vec![(0x0052_66f0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod b027_tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

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
    fn at_00526640_sets_the_start_values() {
        let mut e = Engine::new();
        let block: Ptr = Ptr::new(e.mem.alloc(0x2c));
        for i in 0..11 {
            e.mem.set_u32(block.addr() + 4 * i, 0xffff_ffff);
        }
        assert_eq!(e.call(0x0052_6640, &args![block]).u32(), block.addr());
        let words: Vec<u32> = (0..11).map(|i| e.mem.u32(block.addr() + 4 * i)).collect();
        assert_eq!(
            words,
            vec![0, 0, 0, 0, 0, 0, 0, 0x3f80_0000, 0, 0x3f80_0000, 0]
        );
    }

    #[test]
    fn at_005266f0_restores_the_vtable_and_runs_the_form_destructor() {
        let mut e = Engine::new();
        e.register(TES_FORM_DESTRUCTOR, |_, _| ret(0));
        let this: Ptr = Ptr::new(e.mem.alloc(0x44));
        e.call_log = Some(vec![]);
        e.call(0x0052_66f0, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), LIGHTING_TEMPLATE_VTABLE);
        assert_eq!(calls(&e, TES_FORM_DESTRUCTOR), vec![vec![this.addr()]]);
    }

    #[test]
    fn at_00526710_clears_the_data_block() {
        let mut e = Engine::new();
        e.register(FILL_MEMORY, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(0x0052_6710, &args![0x4000u32]);
        assert_eq!(calls(&e, FILL_MEMORY), vec![vec![0x4018, 0, 0x2c]]);
    }

    /// Runs the loader over `chunks` with the given form type and swap flag.
    fn load(form_type: u32, chunks: Vec<u32>, swap: bool) -> (Engine, u32, bool) {
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
        e.register_double(GET_TES_FORM, move |_, _| ret(form_type));
        e.register_double(FILE_NEEDS_SWAP, move |_, _| ret(u32::from(swap)));
        e.register(CHUNK_SIZE, |_, _| ret(0x21));
        for addr in [
            TES_FORM_LOAD_FORM,
            GET_CHUNK_DATA,
            TES_FORM_LOAD_DATA,
            SWAP_DATA,
            0x0f00_0100,
            0x0f00_0200,
        ] {
            e.register(addr, |_, _| ret(0));
        }
        let mut slots = vec![0u32; 0x138 / 4];
        slots[(SLOT_LOAD_BOUNDS / 4) as usize] = 0x0f00_0100;
        slots[(SLOT_SET_EDITOR_ID / 4) as usize] = 0x0f00_0200;
        e.put_vtable(0x0f00_0000, &slots);
        let this = e.mem.alloc(0x44);
        e.mem.set_u32(this, 0x0f00_0000);
        e.call_log = Some(vec![]);
        let result = e.call(0x0052_68a0, &args![this, 0x7000u32]).bool();
        (e, this, result)
    }

    #[test]
    fn at_005268a0_refuses_another_form_type() {
        let (e, _, result) = load(0x64, vec![CHUNK_DATA], false);
        assert!(!result);
        assert!(calls(&e, TES_FORM_LOAD_FORM).is_empty());
    }

    #[test]
    fn at_005268a0_reads_each_chunk() {
        let (e, this, result) = load(0x65, vec![CHUNK_EDID, CHUNK_OBND, CHUNK_DATA, 0x1234], true);
        assert!(result);
        assert_eq!(calls(&e, TES_FORM_LOAD_FORM), vec![vec![this, 0x7000]]);
        assert_eq!(
            calls(&e, TES_FORM_LOAD_DATA),
            vec![vec![this, 0x7000, this + 0x18, 0x2c]]
        );
        let reads = calls(&e, GET_CHUNK_DATA);
        assert_eq!(reads.len(), 1);
        assert_eq!(reads[0][2], EDITOR_ID_MAXIMUM);
        assert_eq!(calls(&e, 0x0f00_0200), vec![vec![this, reads[0][1]]]);
        assert_eq!(calls(&e, 0x0f00_0100), vec![vec![this, 0x7000]]);
        assert_eq!(calls(&e, SWAP_DATA), vec![vec![this + 0x18]]);
    }

    #[test]
    fn at_005268a0_stops_at_a_zero_tag_and_skips_the_swap() {
        let (e, _, result) = load(0x65, vec![0, CHUNK_DATA], false);
        assert!(result);
        assert!(calls(&e, TES_FORM_LOAD_DATA).is_empty());
        assert!(calls(&e, NEXT_CHUNK).is_empty());
        assert!(calls(&e, SWAP_DATA).is_empty());
    }

    #[test]
    fn at_005269c0_marks_only_when_not_done() {
        let mut e = Engine::new();
        e.register(0x0048_3710, |_, _| ret(0));
        e.register(0x0048_4ab0, |_, _| ret(0));
        e.register(0x0040_13e0, |_, _| ret(0x100));
        e.call_log = Some(vec![]);
        e.call(0x0052_69c0, &args![0x4000u32]);
        assert_eq!(calls(&e, 0x0048_3710), vec![vec![0x4000]]);
        assert_eq!(calls(&e, 0x0048_4ab0), vec![vec![0x4000, 1]]);
        e.register(0x0040_13e0, |_, _| ret(1));
        e.call_log = Some(vec![]);
        e.call(0x0052_69c0, &args![0x4000u32]);
        assert!(calls(&e, 0x0048_3710).is_empty());
    }
}

#[cfg(test)]
mod n075_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls(log: &Log, callee: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == callee)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// An engine whose change flags answer `flags & mask`; the buffer's slot +0 answers
    /// `version`.
    fn engine(flags: u32, version: u32) -> (Engine, u32) {
        let mut e = Engine::new();
        e.register(CHANGE_FLAGS, |_, _| 0x7000u32.into_ret());
        e.register(LOAD_CHANGE_FLAGS, |_, _| 0x7000u32.into_ret());
        e.register_double(FLAGS_TEST, move |_, a| {
            u32::from(flags & a[1] != 0).into_ret()
        });
        e.register_double(0x0060_0000, move |_, _| version.into_ret());
        e.put_vtable(0x0070_0000, &[0x0060_0000]);
        let buffer = e.mem.alloc(4);
        e.mem.set_u32(buffer, 0x0070_0000);
        for callee in [
            SAVE_BYTES,
            LOAD_BYTES,
            FORM_SAVE_GAME,
            FORM_LOAD_GAME,
            FORM_REVERT,
            FILL_MEMORY,
            START_FORM,
            SAVE_DATA,
            CLOSE_FORM,
            SWAP_WORD,
            COPY_BYTES,
            COPY_ALL_COMPONENTS,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        (e, buffer)
    }

    #[test]
    fn fn_00526410_saves_the_flagged_parts() {
        let (mut e, buffer) = engine(2 | 0x8000_0000, 0);
        let zone = e.mem.alloc(0x30);
        e.mem.set_u8(zone + 0x1e, 0x83);
        e.register(SAVE_BYTES, |e, a| {
            if a[2] == 1 {
                assert_eq!(e.mem.u8(a[1]), 1);
            }
            Ret::default()
        });
        e.call(0x0052_6410, &args![zone, buffer]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, FORM_SAVE_GAME), vec![vec![zone, buffer]]);
        let saves = calls(&log, SAVE_BYTES);
        assert_eq!(saves.len(), 2);
        assert_eq!(saves[1], vec![buffer, zone + 0x20, 0x10, 0]);
        let (mut e, buffer) = engine(0, 0);
        e.call(0x0052_6410, &args![zone, buffer]);
        assert!(calls(&e.call_log.take().unwrap(), SAVE_BYTES).is_empty());
    }

    #[test]
    fn fn_005264a0_loads_by_version() {
        let (mut e, buffer) = engine(2 | 0x8000_0000, 8);
        let zone = e.mem.alloc(0x30);
        e.mem.set_u8(zone + 0x1e, 0x82);
        e.call(0x0052_64a0, &args![zone, buffer]);
        assert_eq!(e.mem.u8(zone + 0x1e), 0x82);
        let log = e.call_log.take().unwrap();
        let loads = calls(&log, LOAD_BYTES);
        assert_eq!(loads.len(), 2);
        assert_eq!(loads[0][2], 1);
        assert_eq!(loads[1], vec![buffer, zone + 0x20, 0x10]);
        let (mut e, buffer) = engine(0x8000_0000, 7);
        e.call(0x0052_64a0, &args![zone, buffer]);
        let log = e.call_log.take().unwrap();
        let loads = calls(&log, LOAD_BYTES);
        assert_eq!(loads.len(), 1);
        assert_eq!(loads[0][2], 8);
        assert_ne!(loads[0][1], zone + 0x20);
    }

    #[test]
    fn fn_00526570_clears_the_flagged_bytes() {
        let (mut e, buffer) = engine(0x8000_0000, 0);
        e.call(0x0052_6570, &args![0x1000u32, buffer]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, FORM_REVERT), vec![vec![0x1000, buffer]]);
        assert_eq!(calls(&log, FILL_MEMORY), vec![vec![0x1020, 0, 0x10]]);
        let (mut e, buffer) = engine(0, 0);
        e.call(0x0052_6570, &args![0x1000u32, buffer]);
        assert!(calls(&e.call_log.take().unwrap(), FILL_MEMORY).is_empty());
    }

    #[test]
    fn fn_00526730_and_00526790_save_and_swap_the_data() {
        for swaps in [false, true] {
            let (mut e, _) = engine(0, 0);
            e.register(
                SAVE_SWAPS,
                if swaps {
                    |_, _| 1u32.into_ret()
                } else {
                    |_, _| 0u32.into_ret()
                },
            );
            e.call(0x0052_6730, &args![0x1000u32]);
            let log = e.call_log.take().unwrap();
            assert_eq!(calls(&log, SAVE_DATA), vec![vec![0x1000, 0x1018, 0x2c]]);
            let swapped = calls(&log, SWAP_WORD);
            assert_eq!(swapped.len(), if swaps { 20 } else { 0 });
            if swaps {
                assert_eq!(swapped[0], vec![0x1018, 0]);
                assert_eq!(swapped[9], vec![0x103c, 0]);
            }
            assert_eq!(log.last().unwrap(), &(CLOSE_FORM, vec![0x1000]));
        }
    }

    #[test]
    fn fn_005269f0_and_00526a50_copy_and_compare_templates() {
        let (mut e, _) = engine(0, 0);
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.call(0x0052_69f0, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, COPY_BYTES), vec![vec![0x1018, 0x2018, 0x2c]]);
        assert_eq!(calls(&log, COPY_ALL_COMPONENTS), vec![vec![0x1000, 0x2000]]);
        e.register(MEMCMP, |_, _| 0u32.into_ret());
        e.register(COMPARE_ALL_COMPONENTS, |_, _| 0x100u32.into_ret());
        assert!(!e.call(0x0052_6a50, &args![0x1000u32, 0x2000u32]).bool());
        e.register(COMPARE_ALL_COMPONENTS, |_, _| 1u32.into_ret());
        assert!(e.call(0x0052_6a50, &args![0x1000u32, 0x2000u32]).bool());
        e.register(MEMCMP, |_, _| 1u32.into_ret());
        assert!(e.call(0x0052_6a50, &args![0x1000u32, 0x2000u32]).bool());
        e.register(DYNAMIC_CAST, |_, _| 0u32.into_ret());
        assert!(e.call(0x0052_6a50, &args![0x1000u32, 0x2000u32]).bool());
        e.call_log = Some(vec![]);
        e.call(0x0052_69f0, &args![0x1000u32, 0x2000u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }
}
