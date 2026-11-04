//! `fallout/interface/run_007ab810` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The entered name (`011daed8`, a string) and `00559450` (`thiscall`): its word at +0, the
/// characters.
const ENTERED_NAME: u32 = 0x011d_aed8;
const FIRST_ITEM_00559450: u32 = 0x0055_9450;
/// `Font::ConvertCharacter` (`00a122b0`, `cdecl`, one word: the character's address).
const CONVERT_CHARACTER: u32 = 0x00a1_22b0;
/// `005bd5b0` (no words): the font manager; `005bd5c0` (`thiscall`, one word): its font of
/// that index.
const FONT_MANAGER_005BD5B0: u32 = 0x005b_d5b0;
const GET_FONT_005BD5C0: u32 = 0x005b_d5c0;
/// `0.0` (`double`, `01012060`).
const ZERO: u32 = 0x0101_2060;
/// `TESSoundFile::SetSoundFile` (engine map name for the string set `00489100`,
/// `thiscall`, one word).
const SET_STRING_00489100: u32 = 0x0048_9100;
/// `StatsMenu::UpdatePlayerName` (`007df4e0`, no words).
const UPDATE_PLAYER_NAME: u32 = 0x007d_f4e0;
/// `00700320` (`thiscall`, two words) on the menu's tile (+4).
const TILE_SET_00700320: u32 = 0x0070_0320;

// Translated from 007ab810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no words): the characters of the entered name at
/// `011daed8` (its first word, `00559450`).
pub fn fn_007ab810(e: &mut Engine) -> u32 {
    e.call(FIRST_ITEM_00559450, &args![ENTERED_NAME]).u32()
}

/// Whether the glyph of `character` in font `index` has zero width: `Some(true)` when its
/// float +0x14c (glyphs of 0x38 bytes at the font's +0x38) equals 0.0 (`01012060`; `FCOMP`,
/// `TEST AH,0x44`, `JNP`), `Some(false)` when not, `None` without the font or its glyphs.
fn zero_width(e: &mut Engine, index: u32, character: u8) -> Option<bool> {
    let manager = e.call(FONT_MANAGER_005BD5B0, &args![]).u32();
    let font = e.call(GET_FONT_005BD5C0, &args![manager, index]).u32();
    if font == 0 || e.mem.u32(font + 0x38) == 0 {
        return None;
    }
    let glyphs = e.mem.u32(font + 0x38);
    let width = e.mem.u32(
        glyphs
            .wrapping_add(u32::from(character) * 0x38)
            .wrapping_add(0x14c),
    );
    let width = x87_load32(width);
    let zero = e.mem.f64(ZERO);
    Some(width == zero)
}

// Translated from 007ab820 (decompiled, FalloutNV.exe 1.4.0.525)
/// The player name check (unnamed in the engine map; `PlayerNameEntryMenu`'s constructor
/// passes it to the text entry; `cdecl`, one word: the name). For each character until the
/// end, or until one is found bad: the character is converted (`Font::ConvertCharacter`);
/// `\` and `~` are bad; for fonts 0 and 6 (`005bd5c0`), when the font has glyphs, a glyph of
/// zero width makes it bad, any other character but a space counts as visible. True (`AL`)
/// when a visible character was seen and none was bad; a null name is false.
pub fn fn_007ab820(e: &mut Engine, name: u32) -> bool {
    let mut visible = false;
    let mut bad = false;
    let mut cursor = name;
    e.with_stack(4, |e, slot| {
        while cursor != 0 && !bad && e.mem.i8(cursor) != 0 {
            let character = e.mem.u8(cursor);
            e.mem.set_u8(slot.addr(), character);
            e.call(CONVERT_CHARACTER, &args![slot]);
            let character = e.mem.u8(slot.addr());
            bad = character == 0x5c || character == 0x7e;
            for index in [0u32, 6] {
                match zero_width(e, index, character) {
                    Some(true) => bad = true,
                    Some(false) => visible = visible || character != 0x20,
                    None => {}
                }
            }
            cursor = cursor.wrapping_add(1);
        }
    });
    visible && !bad
}

// Translated from 007ab9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: a name): with a name, the player's
/// form (+0x28) takes it (its +0xd0 string, `00489100`), gets its slot +0x48 with 0x20,
/// and the stats menu's player name is updated (`StatsMenu::UpdatePlayerName`). Either
/// way the menu's tile (+4) gets `00700320(0x1772, 1)` and the word +0x24 becomes 2.
pub fn fn_007ab9a0(e: &mut Engine, this: Ptr, name: u32) {
    if name != 0 {
        let form = e.mem.u32(this.addr() + 0x28);
        e.call(SET_STRING_00489100, &args![form + 0xd0, name]);
        let form = e.mem.u32(this.addr() + 0x28);
        e.vcall(form, 0x48, &args![0x20u32]);
        e.call(UPDATE_PLAYER_NAME, &args![]);
    }
    let tile = e.mem.u32(this.addr() + 4);
    e.call(TILE_SET_00700320, &args![tile, 0x1772u32, 1u32]);
    e.mem.set_u32(this.addr() + 0x24, 2);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x007a_b810, fn_007ab810() -> u32),
        entry!(0x007a_b820, fn_007ab820(u32) -> bool),
        entry!(0x007a_b9a0, fn_007ab9a0(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn called(e: &Engine, target: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == target)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn entered_name_is_the_strings_first_word() {
        let mut e = Engine::new();
        e.register(FIRST_ITEM_00559450, |_, a| (a[0] ^ 1).into_ret());
        assert_eq!(e.call(0x007a_b810, &[]).u32(), ENTERED_NAME ^ 1);
    }

    /// Two fonts with glyph tables; glyph widths 1.0 except `zero` (0.0).
    fn check_engine(zero: Option<u8>, fonts: bool) -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        e.mem.set_f64(ZERO, 0.0);
        e.register(CONVERT_CHARACTER, |_, _| Ret::default());
        e.register(FONT_MANAGER_005BD5B0, |_, _| 0x7000u32.into_ret());
        let font = e.mem.alloc(0x3c);
        if fonts {
            let glyphs = e.mem.alloc(0x38 * 256 + 0x150);
            for c in 0..256u32 {
                let width = if zero == Some(c as u8) { 0.0f32 } else { 1.0 };
                e.mem.set_f32(glyphs + c * 0x38 + 0x14c, width);
            }
            e.mem.set_u32(font + 0x38, glyphs);
        }
        e.register_double(GET_FONT_005BD5C0, move |_, _| font.into_ret());
        e.call_log = Some(vec![]);
        e
    }

    fn check(e: &mut Engine, text: &[u8]) -> bool {
        let name = e.mem.alloc(text.len() as u32 + 1);
        e.mem.write(name, text);
        e.call(0x007a_b820, &[name]).bool()
    }

    #[test]
    fn name_check_needs_a_visible_character() {
        let mut e = check_engine(None, true);
        assert!(check(&mut e, b"Courier"));
        assert!(check(&mut e, b" a "));
        assert!(!check(&mut e, b"   "));
        assert!(!check(&mut e, b""));
        assert!(!e.call(0x007a_b820, &[0]).bool());
        let fonts = called(&e, GET_FONT_005BD5C0);
        assert_eq!(fonts[0], vec![0x7000, 0]);
        assert_eq!(fonts[1], vec![0x7000, 6]);
    }

    #[test]
    fn name_check_refuses_bad_characters() {
        let mut e = check_engine(Some(b'#'), true);
        assert!(!check(&mut e, b"a\\b"));
        assert!(!check(&mut e, b"a~"));
        assert!(!check(&mut e, b"a#"));
        // The scan stops at the first bad character.
        e.call_log = Some(vec![]);
        assert!(!check(&mut e, b"#abc"));
        assert_eq!(called(&e, CONVERT_CHARACTER).len(), 1);
    }

    #[test]
    fn name_check_without_glyphs_sees_nothing_visible() {
        let mut e = check_engine(None, false);
        assert!(!check(&mut e, b"abc"));
        // A bad character is still bad.
        assert!(!check(&mut e, b"~"));
    }

    #[test]
    fn take_name_sets_the_form_and_the_tile() {
        let mut e = Engine::new();
        e.register(SET_STRING_00489100, |_, _| Ret::default());
        e.register(UPDATE_PLAYER_NAME, |_, _| Ret::default());
        e.register(TILE_SET_00700320, |_, _| Ret::default());
        e.register(0x7048, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x4c);
        e.mem.set_u32(vtable + 0x48, 0x7048);
        let form = e.mem.alloc(0x100);
        e.mem.set_u32(form, vtable);
        let this = e.mem.alloc(0x2c);
        e.mem.set_u32(this + 4, 0x7171);
        e.mem.set_u32(this + 0x28, form);
        e.call_log = Some(vec![]);
        e.call(0x007a_b9a0, &[this, 0x4e00]);
        assert_eq!(
            called(&e, SET_STRING_00489100),
            vec![vec![form + 0xd0, 0x4e00]]
        );
        assert_eq!(called(&e, 0x7048), vec![vec![form, 0x20]]);
        assert_eq!(called(&e, UPDATE_PLAYER_NAME).len(), 1);
        assert_eq!(called(&e, TILE_SET_00700320), vec![vec![0x7171, 0x1772, 1]]);
        assert_eq!(e.mem.u32(this + 0x24), 2);
        // Without a name only the tile and the state.
        e.call_log = Some(vec![]);
        e.mem.set_u32(this + 0x24, 0);
        e.call(0x007a_b9a0, &[this, 0]);
        assert!(called(&e, SET_STRING_00489100).is_empty());
        assert_eq!(called(&e, TILE_SET_00700320).len(), 1);
        assert_eq!(e.mem.u32(this + 0x24), 2);
    }
}
