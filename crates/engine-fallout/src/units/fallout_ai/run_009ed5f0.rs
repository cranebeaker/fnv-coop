//! `fallout/ai/run_009ed5f0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `DialoguePackage` destructor body (`009edf40`).
const DIALOGUE_PACKAGE_DESTRUCT: u32 = 0x009e_df40;
/// The game's `free` (`00401030`, cdecl, one word).
const FREE: u32 = 0x0040_1030;
/// The word `009edd30` compares with (+0xb0 of `this`).
const COMPARED_WORD: u32 = 0xb0;

// Translated from 009edd30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `value` equals the word at +0xb0 of `this` (thiscall, `RET 4`;
/// unnamed in the engine map).
pub fn fn_009edd30(e: &mut Engine, this: Ptr, value: u32) -> bool {
    value == e.mem.u32(this.addr() + COMPARED_WORD)
}

// Translated from 009edd50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `DialoguePackage` scalar deleting destructor (by the destructor it
/// runs, `009edf40`, which stores that class's vtable): destroys the
/// object and frees it (`00401030`) when bit 0 of `flags` is set. Returns
/// `this`.
pub fn fn_009edd50(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(DIALOGUE_PACKAGE_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

/// The save-load object (`0x011de45c`) the package save code asks.
const SAVE_LOAD: u32 = 0x011d_e45c;
/// `TESSaveLoadGame::UseSaveGameBlocks` (`00862110`, Xbox PDB): `thiscall`; a `bool`.
const USE_SAVE_GAME_BLOCKS: u32 = 0x0086_2110;
/// `00825c00` (unnamed): `thiscall`; the save buffer's current position.
const SAVE_POSITION: u32 = 0x0082_5c00;
/// `008579b0` / `008579e0` (unnamed, `tessaveloadgame.cpp`): `thiscall`, two words
/// (`RET 8`): write and read (data, size).
const SAVE_WRITE: u32 = 0x0085_79b0;
const SAVE_READ: u32 = 0x0085_79e0;
/// The block tag `0x424c4f4b` written before and checked after a package's data.
const BLOCK_TAG: u32 = 0x424c_4f4b;
/// The `Setting` (`0x011de4e8`) whose byte (`00408d60`) turns the size reports on.
const SIZE_REPORT_SETTING: u32 = 0x011d_e4e8;
const SETTING_BYTE: u32 = 0x0040_8d60;
/// `004fd3e0` / `004fd3c0` (the first named `TES::GetWorldSpace` in the engine map;
/// folded getters): `thiscall` on the save-load object; the form being saved or loaded.
const SAVED_FORM: u32 = 0x004f_d3e0;
const LOADED_FORM: u32 = 0x004f_d3c0;
/// `004839c0` (unnamed, `tesform.cpp`): `cdecl`, one word (a form id); a form whose slot
/// `+0x130` gives its name.
const FORM_FOR_ID: u32 = 0x0048_39c0;
/// `Error` (`0040fbe0`, engine map) and `005b5e40` (unnamed): `cdecl` reports
/// (format, ...).
const ERROR: u32 = 0x0040_fbe0;
const REPORT: u32 = 0x005b_5e40;
/// `008df040` (unnamed, `highprocess.cpp`): `thiscall`; a byte for the report.
const FORM_TYPE_008DF040: u32 = 0x008d_f040;
/// The report texts: `"GetSaveSize(): %-5i for form %08X %s with flags ..."`
/// (`0x01012cb0`), `"GetSaveSize(): %-5i ending at line %i in file %s"` (`0x01012c78`),
/// the same two for `SaveGame()` (`0x010153a0`, `0x0101536c`), the save block overflow
/// (`0x01015318`), and the load checks (`0x01015718`, `0x010156a8`, `0x01015588`,
/// `0x01015500`, `0x010154a0`, `0x01015440`); the source file name (`0x01092ee8`).
const GET_SAVE_SIZE_FORM: u32 = 0x0101_2cb0;
const GET_SAVE_SIZE_LINE: u32 = 0x0101_2c78;
const SAVE_GAME_FORM: u32 = 0x0101_53a0;
const SAVE_GAME_LINE: u32 = 0x0101_536c;
const SAVE_BLOCK_OVERFLOW: u32 = 0x0101_5318;
const LOAD_BAD_TAG_FORM: u32 = 0x0101_5718;
const LOAD_BAD_TAG: u32 = 0x0101_56a8;
const LOAD_OVER_FORM: u32 = 0x0101_5588;
const LOAD_UNDER_FORM: u32 = 0x0101_5500;
const LOAD_OVER: u32 = 0x0101_54a0;
const LOAD_UNDER: u32 = 0x0101_5440;
const SOURCE_FILE: u32 = 0x0109_2ee8;
/// `00678fd0` (unnamed, `package.cpp`): `thiscall`; the base package's save size (16-bit).
const PACKAGE_SAVE_SIZE: u32 = 0x0067_8fd0;
/// `00679120` (unnamed, `package.cpp`): `thiscall`; the base package's save.
const PACKAGE_SAVE: u32 = 0x0067_9120;
/// `TESPackage::LoadGame` (`00679340`, engine map): `thiscall`.
const PACKAGE_LOAD: u32 = 0x0067_9340;
/// `TESForm::SaveGameDataOLD` (`00484ce0`) and `TESForm::LoadGameDataOLD` (`00484d00`),
/// engine map: `thiscall`, two words (`RET 8`).
const SAVE_GAME_DATA_OLD: u32 = 0x0048_4ce0;
const LOAD_GAME_DATA_OLD: u32 = 0x0048_4d00;
/// `TESPackage::SaveGame` (`006797c0`) and `TESPackage::LoadGame` (`006798c0`, the map's
/// `LoadGame_ov2`): `thiscall`, one word (`RET 4`).
const PACKAGE_SAVE_GAME: u32 = 0x0067_97c0;
const PACKAGE_LOAD_GAME: u32 = 0x0067_98c0;
/// `00865e50` / `00864980` (unnamed): the buffer's save (data, size, 0; `RET 0xc`) and
/// load (data, size; `RET 8`).
const BUFFER_SAVE: u32 = 0x0086_5e50;
const BUFFER_LOAD: u32 = 0x0086_4980;
/// `TESPackage::TESPackage` (`006707c0`, Xbox PDB), the `DialoguePackage` vtable
/// (`01092f6c`, RTTI `.?AVDialoguePackage@@`) and `0041a250` (unnamed, the member at
/// `+0x80`'s constructor).
const PACKAGE_CONSTRUCT: u32 = 0x0067_07c0;
const DIALOGUE_PACKAGE_VTABLE: u32 = 0x0109_2f6c;
const MEMBER_CONSTRUCT_0041A250: u32 = 0x0041_a250;

/// The form `004839c0` finds for the id at `form`, its name (slot `+0x130`), and the
/// words the reports pass: the id (`+0`), the byte at `+9` and the word at `+5`
/// (unaligned).
fn form_report_words(e: &mut Engine, form: u32) -> (u32, u32, u32, u32) {
    let id = e.mem.u32(form);
    let named = e.call(FORM_FOR_ID, &args![id]).u32();
    named_report_words(e, form, named)
}

/// [`form_report_words`] with the form `004839c0` found already.
fn named_report_words(e: &mut Engine, form: u32, named: u32) -> (u32, u32, u32, u32) {
    let word = e.mem.u32(form + 0x5);
    let byte = u32::from(e.mem.u8(form + 0x9));
    let name = e.vcall(named, 0x130, &[]).u32();
    let id = e.mem.u32(form);
    (id, name, byte, word)
}

// Translated from 009ed5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only; the `DialoguePackage` save size): the base
/// package's (`00678fd0`), plus 6 with save blocks (`00862110`; the tag and the length),
/// plus 0xc (the point at `+0x80`), all 16-bit. With the size-report setting
/// (`0x011de4e8`) the added size is reported (`Error`, line 0xd3) for the form being saved
/// (`004fd3e0`) or without one.
pub fn fn_009ed5f0(e: &mut Engine, this: Ptr) -> u16 {
    let mut size = 0u16;
    let base_size = e.call(PACKAGE_SAVE_SIZE, &args![this]).u16();
    size = size.wrapping_add(base_size);
    let base = size;
    let save_load = e.mem.u32(SAVE_LOAD);
    if e.call(USE_SAVE_GAME_BLOCKS, &args![save_load]).u8() != 0 {
        size = size.wrapping_add(4);
        size = size.wrapping_add(2);
    }
    size = size.wrapping_add(0xc);
    let enabled = e.call(SETTING_BYTE, &args![SIZE_REPORT_SETTING]).u32();
    if e.mem.u8(enabled) != 0 {
        let added = u32::from(size).wrapping_sub(u32::from(base));
        let save_load = e.mem.u32(SAVE_LOAD);
        let form = e.call(SAVED_FORM, &args![save_load]).u32();
        if form != 0 {
            let id = e.mem.u32(form);
            let named = e.call(FORM_FOR_ID, &args![id]).u32();
            let word = e.mem.u32(form + 0x5);
            let name = e.vcall(named, 0x130, &[]).u32();
            let id = e.mem.u32(form);
            e.call(
                ERROR,
                &args![
                    GET_SAVE_SIZE_FORM,
                    added,
                    id,
                    name,
                    word,
                    0xd3u32,
                    SOURCE_FILE
                ],
            );
        } else {
            e.call(
                ERROR,
                &args![GET_SAVE_SIZE_LINE, added, 0xd3u32, SOURCE_FILE],
            );
        }
    }
    size
}

// Translated from 009ed700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only; the `DialoguePackage` save): the base package's
/// (`00679120`); with save blocks the tag `0x424c4f4b` and a 16-bit length placeholder
/// (`008579b0`) whose position (`00825c00`) is kept; a 16-bit 0; the point at `+0x80`
/// (`TESForm::SaveGameDataOLD`, 0xc bytes). With the size-report setting the size written
/// is reported (`Error`, line 0x106). With save blocks the placeholder at the kept
/// position then gets the block's length (a report, line 0x106, when it is above 0xffff).
pub fn fn_009ed700(e: &mut Engine, this: Ptr) {
    e.call(PACKAGE_SAVE, &args![this]);
    e.with_stack(0x10, |e, frame| {
        let length = frame.addr();
        let tag = frame.addr() + 0x4;
        let zero = frame.addr() + 0x8;
        e.mem.set_u16(length, 0);
        let mut position = 0u32;
        let save_load = e.mem.u32(SAVE_LOAD);
        let mut start = e.call(SAVE_POSITION, &args![save_load]).u32();
        let enabled = e.call(SETTING_BYTE, &args![SIZE_REPORT_SETTING]).u32();
        if e.mem.u8(enabled) != 0 {
            let save_load = e.mem.u32(SAVE_LOAD);
            start = e.call(SAVE_POSITION, &args![save_load]).u32();
        }
        let save_load = e.mem.u32(SAVE_LOAD);
        if e.call(USE_SAVE_GAME_BLOCKS, &args![save_load]).u8() != 0 {
            e.mem.set_u32(tag, BLOCK_TAG);
            let save_load = e.mem.u32(SAVE_LOAD);
            e.call(SAVE_WRITE, &args![save_load, tag, 4u32]);
            let save_load = e.mem.u32(SAVE_LOAD);
            position = e.call(SAVE_POSITION, &args![save_load]).u32();
            let save_load = e.mem.u32(SAVE_LOAD);
            e.call(SAVE_WRITE, &args![save_load, length, 2u32]);
        }
        e.mem.set_u16(zero, 0);
        let save_load = e.mem.u32(SAVE_LOAD);
        let _zero_position = e.call(SAVE_POSITION, &args![save_load]).u32();
        let save_load = e.mem.u32(SAVE_LOAD);
        e.call(SAVE_WRITE, &args![save_load, zero, 2u32]);
        e.call(SAVE_GAME_DATA_OLD, &args![this, this.addr() + 0x80, 0xcu32]);
        let enabled = e.call(SETTING_BYTE, &args![SIZE_REPORT_SETTING]).u32();
        if e.mem.u8(enabled) != 0 {
            let save_load = e.mem.u32(SAVE_LOAD);
            let end = e.call(SAVE_POSITION, &args![save_load]).u32();
            let save_load = e.mem.u32(SAVE_LOAD);
            let form = e.call(SAVED_FORM, &args![save_load]).u32();
            let written = end.wrapping_sub(start);
            if form != 0 {
                let id = e.mem.u32(form);
                let named = e.call(FORM_FOR_ID, &args![id]).u32();
                let word = e.mem.u32(form + 0x5);
                let name = e.vcall(named, 0x130, &[]).u32();
                let id = e.mem.u32(form);
                e.call(
                    ERROR,
                    &args![
                        SAVE_GAME_FORM,
                        written,
                        id,
                        name,
                        word,
                        0x106u32,
                        SOURCE_FILE
                    ],
                );
            } else {
                e.call(
                    ERROR,
                    &args![SAVE_GAME_LINE, written, 0x106u32, SOURCE_FILE],
                );
            }
        }
        let save_load = e.mem.u32(SAVE_LOAD);
        if e.call(USE_SAVE_GAME_BLOCKS, &args![save_load]).u8() != 0 {
            let save_load = e.mem.u32(SAVE_LOAD);
            let current = e.call(SAVE_POSITION, &args![save_load]).u32();
            let kept = position;
            if current > kept.wrapping_add(0xffff) {
                e.call(REPORT, &args![SAVE_BLOCK_OVERFLOW, SOURCE_FILE, 0x106u32]);
            }
            e.mem.set_u16(position, current.wrapping_sub(kept) as u16);
        }
    });
}

// Translated from 009ed8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only; the `DialoguePackage` load): the base package's
/// (`TESPackage::LoadGame`); with save blocks the tag (reported when it is not
/// `0x424c4f4b`, line 0x10e, for the form being loaded `004fd3c0` or with `008df040`'s
/// byte) and the 16-bit block length (`008579e0`, its position kept); the point at
/// `+0x80` (`TESForm::LoadGameDataOLD`, 0xc bytes). With save blocks, a position after the
/// load other than the kept position plus the length is reported (line 0x136; by how
/// much, past or short).
pub fn fn_009ed8d0(e: &mut Engine, this: Ptr) {
    e.call(PACKAGE_LOAD, &args![this]);
    e.with_stack(0x8, |e, frame| {
        let length = frame.addr();
        let tag = frame.addr() + 0x4;
        e.mem.set_u16(length, 0);
        let mut start = 0u32;
        let save_load = e.mem.u32(SAVE_LOAD);
        if e.call(USE_SAVE_GAME_BLOCKS, &args![save_load]).u8() != 0 {
            let save_load = e.mem.u32(SAVE_LOAD);
            e.call(SAVE_READ, &args![save_load, tag, 4u32]);
            if e.mem.u32(tag) != BLOCK_TAG {
                let save_load = e.mem.u32(SAVE_LOAD);
                let form = e.call(LOADED_FORM, &args![save_load]).u32();
                if form != 0 {
                    let (id, name, byte, word) = form_report_words(e, form);
                    e.call(
                        REPORT,
                        &args![
                            LOAD_BAD_TAG_FORM,
                            SOURCE_FILE,
                            0x10eu32,
                            id,
                            name,
                            byte,
                            word
                        ],
                    );
                } else {
                    let save_load = e.mem.u32(SAVE_LOAD);
                    let kind = u32::from(e.call(FORM_TYPE_008DF040, &args![save_load]).u8());
                    e.call(REPORT, &args![LOAD_BAD_TAG, SOURCE_FILE, 0x10eu32, kind]);
                }
            }
            let save_load = e.mem.u32(SAVE_LOAD);
            start = e.call(SAVE_POSITION, &args![save_load]).u32();
            let save_load = e.mem.u32(SAVE_LOAD);
            e.call(SAVE_READ, &args![save_load, length, 2u32]);
        }
        e.call(LOAD_GAME_DATA_OLD, &args![this, this.addr() + 0x80, 0xcu32]);
        let save_load = e.mem.u32(SAVE_LOAD);
        if e.call(USE_SAVE_GAME_BLOCKS, &args![save_load]).u8() == 0 {
            return;
        }
        let save_load = e.mem.u32(SAVE_LOAD);
        let current = e.call(SAVE_POSITION, &args![save_load]).u32();
        let save_load = e.mem.u32(SAVE_LOAD);
        let form = e.call(LOADED_FORM, &args![save_load]).u32();
        let expected = u32::from(e.mem.u16(length)).wrapping_add(start);
        if form != 0 {
            let id = e.mem.u32(form);
            let named = e.call(FORM_FOR_ID, &args![id]).u32();
            if current > expected {
                let (id, name, byte, word) = named_report_words(e, form, named);
                let expected = u32::from(e.mem.u16(length)).wrapping_add(start);
                e.call(
                    REPORT,
                    &args![
                        LOAD_OVER_FORM,
                        current.wrapping_sub(expected),
                        SOURCE_FILE,
                        0x136u32,
                        id,
                        name,
                        byte,
                        word
                    ],
                );
            } else if current < expected {
                let (id, name, byte, word) = named_report_words(e, form, named);
                let expected = u32::from(e.mem.u16(length)).wrapping_add(start);
                e.call(
                    REPORT,
                    &args![
                        LOAD_UNDER_FORM,
                        expected.wrapping_sub(current),
                        SOURCE_FILE,
                        0x136u32,
                        id,
                        name,
                        byte,
                        word
                    ],
                );
            }
        } else if current > expected {
            let save_load = e.mem.u32(SAVE_LOAD);
            let kind = u32::from(e.call(FORM_TYPE_008DF040, &args![save_load]).u8());
            e.call(
                REPORT,
                &args![
                    LOAD_OVER,
                    current.wrapping_sub(expected),
                    SOURCE_FILE,
                    0x136u32,
                    kind
                ],
            );
        } else if current < expected {
            let save_load = e.mem.u32(SAVE_LOAD);
            let kind = u32::from(e.call(FORM_TYPE_008DF040, &args![save_load]).u8());
            e.call(
                REPORT,
                &args![
                    LOAD_UNDER,
                    expected.wrapping_sub(current),
                    SOURCE_FILE,
                    0x136u32,
                    kind
                ],
            );
        }
    });
}

// Translated from 009edb60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): `TESPackage::SaveGame` into `buffer`,
/// then the point at `+0x80` (`00865e50`, 0xc bytes).
pub fn fn_009edb60(e: &mut Engine, this: Ptr, buffer: u32) {
    e.call(PACKAGE_SAVE_GAME, &args![this, buffer]);
    e.call(
        BUFFER_SAVE,
        &args![buffer, this.addr() + 0x80, 0xcu32, 0u32],
    );
}

// Translated from 009edb90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): `TESPackage::LoadGame` from `buffer`,
/// then the point at `+0x80` (`00864980`, 0xc bytes).
pub fn fn_009edb90(e: &mut Engine, this: Ptr, buffer: u32) {
    e.call(PACKAGE_LOAD_GAME, &args![this, buffer]);
    e.call(BUFFER_LOAD, &args![buffer, this.addr() + 0x80, 0xcu32]);
}

// Translated from 009edbc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009edbc0 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// Unnamed in the engine map (`ECX` only): the `DialoguePackage` constructor (vtable
/// `01092f6c`): `TESPackage::TESPackage`, the vtable, the member at `+0x80` (`0041a250`),
/// then the words `+0x8c`, `+0x94`, `+0x9c`, `+0xa4`, `+0xa8`, `+0xac`, `+0xb0`, `+0xb4`,
/// `+0xb8`, `+0xc4` cleared, the float at `+0x90` 0.0, `+0xc8` 0x32, the bytes `+0x98`,
/// `+0xa0`, `+0xbd` to `+0xc2` cleared and `+0xbc`, `+0xcc` set (in the exe's order).
/// Returns `this`.
pub fn fn_009edbc0(e: &mut Engine, this: Ptr) -> Ptr {
    let p = this.addr();
    e.call(PACKAGE_CONSTRUCT, &args![this]);
    e.mem.set_u32(p, DIALOGUE_PACKAGE_VTABLE);
    e.call(MEMBER_CONSTRUCT_0041A250, &args![p + 0x80]);
    e.mem.set_u32(p + 0xa8, 0);
    e.mem.set_u32(p + 0xac, 0);
    e.mem.set_u32(p + 0xa4, 0);
    e.mem.set_u32(p + 0x90, 0.0f32.to_bits());
    e.mem.set_u32(p + 0x8c, 0);
    e.mem.set_u32(p + 0x94, 0);
    e.mem.set_u32(p + 0xb0, 0);
    e.mem.set_u32(p + 0xb8, 0);
    e.mem.set_u8(p + 0x98, 0);
    e.mem.set_u8(p + 0xbd, 0);
    e.mem.set_u8(p + 0xbe, 0);
    e.mem.set_u8(p + 0xbf, 0);
    e.mem.set_u8(p + 0xc0, 0);
    e.mem.set_u8(p + 0xbc, 1);
    e.mem.set_u8(p + 0xc1, 0);
    e.mem.set_u32(p + 0x9c, 0);
    e.mem.set_u32(p + 0xb4, 0);
    e.mem.set_u32(p + 0xc4, 0);
    e.mem.set_u32(p + 0xc8, 0x32);
    e.mem.set_u8(p + 0xa0, 0);
    e.mem.set_u32(p + 0xb8, 0);
    e.mem.set_u8(p + 0xc2, 0);
    e.mem.set_u8(p + 0xcc, 1);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009e_dd30, fn_009edd30(Ptr, u32) -> bool),
        entry!(0x009e_dd50, fn_009edd50(Ptr, u32) -> Ptr),
        entry!(0x009e_d5f0, fn_009ed5f0(Ptr) -> u16),
        entry!(0x009e_d700, fn_009ed700(Ptr)),
        entry!(0x009e_d8d0, fn_009ed8d0(Ptr)),
        entry!(0x009e_db60, fn_009edb60(Ptr, u32)),
        entry!(0x009e_db90, fn_009edb90(Ptr, u32)),
        entry!(0x009e_dbc0, fn_009edbc0(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_009edd30_compares_with_the_word_at_0xb0() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xb4);
        e.mem.set_u32(this + COMPARED_WORD, 0x1234_5678);
        assert_eq!(
            e.call(0x009e_dd30, &args![this, 0x1234_5678u32]).u32() & 0xff,
            1
        );
        assert_eq!(
            e.call(0x009e_dd30, &args![this, 0x1234_5679u32]).u32() & 0xff,
            0
        );
    }

    #[test]
    fn fn_009edd50_destroys_then_frees_only_with_bit_zero() {
        let mut e = Engine::new();
        e.register(DIALOGUE_PACKAGE_DESTRUCT, |_, _| Ret::default());
        e.register(FREE, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        for (flags, frees) in [(0u32, false), (2, false), (1, true), (3, true)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x009e_dd50, &args![this, flags]).u32(), this);
            let log = e.call_log.take().unwrap();
            let order: Vec<_> = log
                .iter()
                .filter(|(a, _)| *a == DIALOGUE_PACKAGE_DESTRUCT || *a == FREE)
                .cloned()
                .collect();
            let mut expected = vec![(DIALOGUE_PACKAGE_DESTRUCT, vec![this])];
            if frees {
                expected.push((FREE, vec![this]));
            }
            assert_eq!(order, expected, "flags {flags}");
        }
    }
}

#[cfg(test)]
mod save_tests {
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
            .filter(|call| call.0 == addr)
            .map(|call| call.1.clone())
            .collect()
    }

    /// The save-load object's state lives at 0x7e00: `+0` blocks on, `+4` the position,
    /// `+8` the saved/loaded form, `+0xc` the report setting byte, `+0x10` the bytes
    /// read next (tag), `+0x14` the length read next.
    fn doubles(e: &mut Engine) {
        e.map(SAVE_LOAD & !0xfff, 0x1000);
        e.mem.set_u32(SAVE_LOAD, 0x7e00);
        e.map(0x7e00, 0x100);
        e.register(USE_SAVE_GAME_BLOCKS, |e, a| ret(e.mem.u32(a[0])));
        e.register(SAVE_POSITION, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(SAVE_WRITE, |e, a| {
            let position = e.mem.u32(a[0] + 4);
            e.mem.set_u32(a[0] + 4, position + a[2]);
            Ret::default()
        });
        e.register(SAVE_READ, |e, a| {
            let source = if a[2] == 4 { a[0] + 0x10 } else { a[0] + 0x14 };
            let bytes = e.mem.bytes(source, a[2]);
            e.mem.write(a[1], &bytes);
            let position = e.mem.u32(a[0] + 4);
            e.mem.set_u32(a[0] + 4, position + a[2]);
            Ret::default()
        });
        e.register(SETTING_BYTE, |_, _| ret(0x7e0c));
        e.register(SAVED_FORM, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(LOADED_FORM, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(FORM_FOR_ID, |_, _| ret(0x7e40));
        e.mem.set_u32(0x7e40, 0x7e60);
        e.mem.set_u32(0x7e60 + 0x130, 0x7130);
        e.register(0x7130, |_, _| ret(0x5a5a));
        e.register(FORM_TYPE_008DF040, |_, _| ret(0x2b));
        e.register(PACKAGE_SAVE_SIZE, |_, _| ret(0x1_0010));
        for addr in [
            ERROR,
            REPORT,
            PACKAGE_SAVE,
            PACKAGE_LOAD,
            SAVE_GAME_DATA_OLD,
            LOAD_GAME_DATA_OLD,
            PACKAGE_SAVE_GAME,
            PACKAGE_LOAD_GAME,
            BUFFER_SAVE,
            BUFFER_LOAD,
            PACKAGE_CONSTRUCT,
            MEMBER_CONSTRUCT_0041A250,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
    }

    fn form(e: &mut Engine) -> u32 {
        let form = e.mem.alloc(0x10);
        e.mem.set_u32(form, 0x0012_3456);
        e.mem.set_u8(form + 9, 0x3c);
        e.mem.set_u32(0x7e08, form);
        form
    }

    #[test]
    fn fn_009ed5f0_adds_the_block_and_the_point() {
        let mut e = Engine::new();
        doubles(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009e_d5f0, &args![0x4000u32]).u16(), 0x1c);
        assert!(calls(&e, ERROR).is_empty());
        e.mem.set_u32(0x7e00, 1);
        e.mem.set_u8(0x7e0c, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009e_d5f0, &args![0x4000u32]).u16(), 0x22);
        assert_eq!(
            calls(&e, ERROR),
            vec![vec![GET_SAVE_SIZE_LINE, 0x12, 0xd3, SOURCE_FILE]]
        );
        let form = form(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x009e_d5f0, &args![0x4000u32]);
        let word = e.mem.u32(form + 5);
        assert_eq!(
            calls(&e, ERROR),
            vec![vec![
                GET_SAVE_SIZE_FORM,
                0x12,
                0x0012_3456,
                0x5a5a,
                word,
                0xd3,
                SOURCE_FILE
            ]]
        );
    }

    #[test]
    fn fn_009ed700_writes_the_block_and_its_length() {
        let mut e = Engine::new();
        doubles(&mut e);
        e.mem.set_u32(0x7e00, 1);
        e.mem.set_u8(0x7e0c, 1);
        // The position is an address the length is written back to.
        let buffer = e.mem.alloc(0x40);
        e.mem.set_u32(0x7e04, buffer);
        e.call_log = Some(vec![]);
        e.call(0x009e_d700, &args![0x4000u32]);
        let writes: Vec<u32> = calls(&e, SAVE_WRITE).iter().map(|c| c[2]).collect();
        assert_eq!(writes, vec![4, 2, 2]);
        assert_eq!(
            calls(&e, SAVE_GAME_DATA_OLD),
            vec![vec![0x4000, 0x4080, 0xc]]
        );
        // After the tag: position buffer + 4; the block grew by 4 more (the two 2-byte
        // writes) before the point (the double writes nothing for it).
        assert_eq!(e.mem.u16(buffer + 4), 4);
        assert_eq!(
            calls(&e, ERROR),
            vec![vec![SAVE_GAME_LINE, 8, 0x106, SOURCE_FILE]]
        );
        assert!(calls(&e, REPORT).is_empty());
        // Without blocks: only the 16-bit zero.
        e.mem.set_u32(0x7e00, 0);
        e.mem.set_u8(0x7e0c, 0);
        e.call_log = Some(vec![]);
        e.call(0x009e_d700, &args![0x4000u32]);
        assert_eq!(calls(&e, SAVE_WRITE).len(), 1);
    }

    #[test]
    fn fn_009ed8d0_checks_the_block() {
        let mut e = Engine::new();
        doubles(&mut e);
        e.mem.set_u32(0x7e00, 1);
        e.mem.set_u32(0x7e10, BLOCK_TAG);
        e.mem.set_u16(0x7e14, 2);
        e.call_log = Some(vec![]);
        e.call(0x009e_d8d0, &args![0x4000u32]);
        // Read 4 + 2: the position after is 6, the block started at 4 with length 2.
        assert!(calls(&e, REPORT).is_empty());
        assert_eq!(
            calls(&e, LOAD_GAME_DATA_OLD),
            vec![vec![0x4000, 0x4080, 0xc]]
        );
        // A wrong tag and a short block, without a form.
        e.mem.set_u32(0x7e04, 0);
        e.mem.set_u32(0x7e10, 0x1111_1111);
        e.mem.set_u16(0x7e14, 5);
        e.call_log = Some(vec![]);
        e.call(0x009e_d8d0, &args![0x4000u32]);
        assert_eq!(
            calls(&e, REPORT),
            vec![
                vec![LOAD_BAD_TAG, SOURCE_FILE, 0x10e, 0x2b],
                vec![LOAD_UNDER, 3, SOURCE_FILE, 0x136, 0x2b]
            ]
        );
        // A long block with a form: the form is found once for the check.
        let form = form(&mut e);
        e.mem.set_u32(0x7e04, 0);
        e.mem.set_u32(0x7e10, BLOCK_TAG);
        e.mem.set_u16(0x7e14, 0);
        e.call_log = Some(vec![]);
        e.call(0x009e_d8d0, &args![0x4000u32]);
        let word = e.mem.u32(form + 5);
        assert_eq!(
            calls(&e, REPORT),
            vec![vec![
                LOAD_OVER_FORM,
                2,
                SOURCE_FILE,
                0x136,
                0x0012_3456,
                0x5a5a,
                0x3c,
                word
            ]]
        );
        assert_eq!(calls(&e, FORM_FOR_ID).len(), 1);
    }

    #[test]
    fn fn_009edb60_and_fn_009edb90_add_the_point() {
        let mut e = Engine::new();
        doubles(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x009e_db60, &args![0x4000u32, 0x6000u32]);
        e.call(0x009e_db90, &args![0x4000u32, 0x6000u32]);
        assert_eq!(calls(&e, PACKAGE_SAVE_GAME), vec![vec![0x4000, 0x6000]]);
        assert_eq!(calls(&e, BUFFER_SAVE), vec![vec![0x6000, 0x4080, 0xc, 0]]);
        assert_eq!(calls(&e, PACKAGE_LOAD_GAME), vec![vec![0x4000, 0x6000]]);
        assert_eq!(calls(&e, BUFFER_LOAD), vec![vec![0x6000, 0x4080, 0xc]]);
    }

    #[test]
    fn fn_009edbc0_constructs_the_dialogue_package() {
        let mut e = Engine::new();
        doubles(&mut e);
        let this = e.mem.alloc(0xd0);
        e.mem.write(this, &[0xcd; 0xd0]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009e_dbc0, &args![this]).u32(), this);
        assert_eq!(calls(&e, PACKAGE_CONSTRUCT), vec![vec![this]]);
        assert_eq!(
            calls(&e, MEMBER_CONSTRUCT_0041A250),
            vec![vec![this + 0x80]]
        );
        assert_eq!(e.mem.u32(this), DIALOGUE_PACKAGE_VTABLE);
        for offset in [
            0x8cu32, 0x90, 0x94, 0x9c, 0xa4, 0xa8, 0xac, 0xb0, 0xb4, 0xb8, 0xc4,
        ] {
            assert_eq!(e.mem.u32(this + offset), 0, "{offset:#x}");
        }
        assert_eq!(e.mem.u32(this + 0xc8), 0x32);
        for offset in [0x98u32, 0xa0, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2] {
            assert_eq!(e.mem.u8(this + offset), 0, "{offset:#x}");
        }
        assert_eq!(e.mem.u8(this + 0xbc), 1);
        assert_eq!(e.mem.u8(this + 0xcc), 1);
        assert_eq!(e.mem.u8(this + 0x99), 0xcd);
    }
}
