//! `(unplaced)/run_00408c30` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// A static byte the null case writes zero to and returns the address of.
const EMPTY_BYTE: u32 = 0x0120_2800;

// Translated from 00408d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the field at `+4` of `this`; for a null `this`, writes 0 to
/// the static byte at `01202800` and returns its address instead.
pub fn fn_00408d60(e: &mut Engine, this: Ptr) -> Ptr {
    if this.is_null() {
        e.mem.set_u8(EMPTY_BYTE, 0);
        Ptr::new(EMPTY_BYTE)
    } else {
        Ptr::new(this.addr() + 4)
    }
}

/// The pointer [`fn_00408c30`] hands to `UseSaveGameBlocks` and `004fd3e0`.
const SAVE_LOAD_GAME: u32 = 0x011d_e45c;
/// `TESSaveLoadGame::UseSaveGameBlocks` (Xbox PDB, `00862110`, `ECX`), `AL`.
const USE_SAVE_GAME_BLOCKS: u32 = 0x0086_2110;
/// The first entry of the effect-setting list [`fn_00408c30`] walks.
const SETTING_LIST: u32 = 0x011c_3400;
/// `006815c0` (`ECX` = entry): the address of the entry's item word.
const ITEM_ADDRESS_006815C0: u32 = 0x0068_15c0;
/// `00726070` (`ECX` = entry): the next entry.
const NEXT_ENTRY_00726070: u32 = 0x0072_6070;
/// `00403e60` (`effectitem.cpp` in the engine map; `ECX`, one stack word: a mask), `AL`.
const HAS_FLAG_00403E60: u32 = 0x0040_3e60;
/// The object whose +4 text [`fn_00408d60`] answers (the debug text tested for a first
/// byte).
const DEBUG_TEXT_OWNER: u32 = 0x011d_e4e8;
/// `004fd3e0` (`ECX` = [`SAVE_LOAD_GAME`]'s pointer): the form whose sizes are reported.
const CURRENT_FORM_004FD3E0: u32 = 0x004f_d3e0;
/// `004839c0` (`cdecl`, one word): the form of a form id.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `Error` (engine map, `0040fbe0`, `cdecl`, variadic).
const ERROR: u32 = 0x0040_fbe0;
/// `"GetSaveSize(): %-5i for form %08X %s with flags %08X ending at line %i in file %s"`.
const FORMAT_WITH_FORM: u32 = 0x0101_2cb0;
/// `"GetSaveSize(): %-5i ending at line %i in file %s"`.
const FORMAT_WITHOUT_FORM: u32 = 0x0101_2c78;
/// The source file name (`...\Magic\EffectSettingCollection.cpp`).
const SOURCE_FILE: u32 = 0x0101_2d08;

// Translated from 00408c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The effect-setting collection's save size (no name in the engine map; the messages
/// name `GetSaveSize` and `EffectSettingCollection.cpp`; no arguments, plain `RET`; called
/// by `00954d40`): 6 when `TESSaveLoadGame::UseSaveGameBlocks` answers true, plus 4, plus
/// 4 for each entry of the list at `011c3400` (up to the first null item) whose item
/// answers true to `00403e60(0x200000)`. When the text at `011de4ec` ([`fn_00408d60`] on
/// `011de4e8`) is not empty, reports the size through `Error` at line 0x77: with the form
/// `004fd3e0` answers (its id, its vtable slot +0x130 name of the form `004839c0` finds,
/// and the word at its +5), or without one when that is null. The reported size is the
/// size minus its low 16 bits at the start (0). Answers the size.
pub fn fn_00408c30(e: &mut Engine) -> u32 {
    let mut size: u32 = 0;
    let start = size as u16;
    let save_load_game: u32 = e.global(SAVE_LOAD_GAME);
    if e.call(USE_SAVE_GAME_BLOCKS, &args![save_load_game]).u8() != 0 {
        size = size.wrapping_add(4);
        size = size.wrapping_add(2);
    }
    size = size.wrapping_add(4);
    let mut entry = SETTING_LIST;
    while entry != 0 {
        let item_address = e.call(ITEM_ADDRESS_006815C0, &args![entry]).u32();
        let item = e.mem.u32(item_address);
        if item == 0 {
            break;
        }
        if e.call(HAS_FLAG_00403E60, &args![item, 0x0020_0000u32]).u8() != 0 {
            size = size.wrapping_add(4);
        }
        entry = e.call(NEXT_ENTRY_00726070, &args![entry]).u32();
    }
    let text = fn_00408d60(e, Ptr::new(DEBUG_TEXT_OWNER));
    if e.mem.u8(text.addr()) != 0 {
        let reported = size.wrapping_sub(u32::from(start));
        let save_load_game: u32 = e.global(SAVE_LOAD_GAME);
        let form = e.call(CURRENT_FORM_004FD3E0, &args![save_load_game]).u32();
        if form != 0 {
            let id = e.mem.u32(form);
            let found = e.call(LOOKUP_FORM, &args![id]).u32();
            let flags = e.mem.u32(form.wrapping_add(5));
            let name = e.vcall(found, 0x130, &args![]).u32();
            e.call(
                ERROR,
                &args![
                    FORMAT_WITH_FORM,
                    reported,
                    id,
                    name,
                    flags,
                    0x77u32,
                    SOURCE_FILE
                ],
            );
        } else {
            e.call(
                ERROR,
                &args![FORMAT_WITHOUT_FORM, reported, 0x77u32, SOURCE_FILE],
            );
        }
    }
    size
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00408d60, fn_00408d60(Ptr) -> Ptr),
        entry!(0x0040_8c30, fn_00408c30() -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn points_at_the_field_at_4() {
        let mut e = Engine::new();
        let object: Ptr = Ptr::new(e.mem.alloc(0x10));
        assert_eq!(e.call(0x0040_8d60, &args![object]).u32(), object.addr() + 4);
    }

    #[test]
    fn null_this_gives_the_zeroed_static_byte() {
        let mut e = Engine::new();
        e.map(0x0120_2000, 0x1000);
        e.mem.set_u8(EMPTY_BYTE, 9);
        assert_eq!(
            e.call(0x0040_8d60, &args![Ptr::<()>::NULL]).u32(),
            EMPTY_BYTE
        );
        assert_eq!(e.mem.u8(EMPTY_BYTE), 0);
    }
}

#[cfg(test)]
mod save_size_tests {
    use super::*;

    /// The list: three entries (items 0x7000 flagged, 0x7100 not, then a null item).
    fn engine(blocks: bool, text: u8, form: u32) -> Engine {
        let mut e = Engine::new();
        e.map(0x011c_3000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        e.set_global(SAVE_LOAD_GAME, 0x0090_0000u32);
        e.register(
            USE_SAVE_GAME_BLOCKS,
            if blocks {
                |_, _| 1u32.into_ret()
            } else {
                |_, _| 0x100u32.into_ret()
            },
        );
        // Entries at 011c3400, 011c3410, 011c3420; item word at +0, next at +4.
        for (i, item) in [0x7000u32, 0x7100, 0].iter().enumerate() {
            let entry = SETTING_LIST + 0x10 * i as u32;
            e.mem.set_u32(entry, *item);
            e.mem.set_u32(entry + 4, entry + 0x10);
        }
        e.register(ITEM_ADDRESS_006815C0, |_, a| a[0].into_ret());
        e.register(NEXT_ENTRY_00726070, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(HAS_FLAG_00403E60, |_, a| {
            u32::from(a[0] == 0x7000).into_ret()
        });
        e.mem.set_u8(DEBUG_TEXT_OWNER + 4, text);
        e.register_double(CURRENT_FORM_004FD3E0, move |_, _| form.into_ret());
        e.register(LOOKUP_FORM, |_, _| 0x0080_0000u32.into_ret());
        e.register(0x0060_0130, |_, _| 0x0012_3000u32.into_ret());
        let mut slots = vec![0u32; 0x4d];
        slots[0x130 / 4] = 0x0060_0130;
        e.put_vtable(0x0070_0000, &slots);
        e.map(0x0080_0000, 4);
        e.mem.set_u32(0x0080_0000, 0x0070_0000);
        e.register(ERROR, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    fn errors(e: &mut Engine) -> Vec<Vec<u32>> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == ERROR)
            .map(|(_, w)| w)
            .collect()
    }

    #[test]
    fn fn_00408c30_counts_blocks_and_flagged_settings() {
        let mut e = engine(true, 0, 0);
        assert_eq!(e.call(0x0040_8c30, &[]).u32(), 6 + 4 + 4);
        assert!(errors(&mut e).is_empty());
        let mut e = engine(false, 0, 0);
        assert_eq!(e.call(0x0040_8c30, &[]).u32(), 4 + 4);
    }

    #[test]
    fn fn_00408c30_reports_with_and_without_a_form() {
        let mut e = engine(false, b'x', 0);
        assert_eq!(e.call(0x0040_8c30, &[]).u32(), 8);
        assert_eq!(
            errors(&mut e),
            vec![vec![FORMAT_WITHOUT_FORM, 8, 0x77, SOURCE_FILE]]
        );
        let form = 0x0050_0000;
        let mut e = engine(true, b'x', form);
        e.map(form, 0x10);
        e.mem.set_u32(form, 0x0001_2345);
        e.mem.set_u32(form + 5, 0xdead_beef);
        assert_eq!(e.call(0x0040_8c30, &[]).u32(), 14);
        assert_eq!(
            errors(&mut e),
            vec![vec![
                FORMAT_WITH_FORM,
                14,
                0x0001_2345,
                0x0012_3000,
                0xdead_beef,
                0x77,
                SOURCE_FILE
            ]]
        );
    }
}
