//! `fallout/interface/run_007e6320` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The run is the text edit menu (`007e6320` to `007e6760`) and, after it,
//! the constructor of the trait menu (`007e6780`). The text edit menu is
//! the class whose RTTI type descriptor is `.?AVTextEditMenu@@`
//! (`0119f704`, read in the executable); it is loaded from
//! `Data\Menus\Dialog\TextEditMenu.xml` with menu class `0x41b`. The
//! Xbox PDB has no such class, so the field names below are descriptive.
//! The menu object is the singleton at `011daec4`, with an embedded text
//! editor at +0x34 (`00716a70`..`00717050` work on it) and a function
//! pointer at +0x58 that is asked whether the current text is acceptable.
//!
//! The trait menu constructor (`007e6780`) is confirmed as `TraitMenu`'s by
//! its vtable (`010779bc`, complete object locator type
//! `.?AVTraitMenu@@`). Its exception frame is not translated.

#[allow(unused_imports)]
use crate::prelude::*;

/// The text edit menu singleton.
const TEXT_EDIT_MENU: u32 = 0x011d_aec4;
/// The `BSStringT` that receives the accepted text (its `c_str` getter is
/// `007ab810`).
const ENTERED_TEXT: u32 = 0x011d_aed8;
/// The menu class id of the text edit menu (what `007e65a0` returns).
const TEXT_EDIT_CLASS: i32 = 0x41b;

/// Returns the word at +4 of its argument (`Menu::pMenu`).
const WORD_AT_FOUR: u32 = 0x0072_6070;
/// Sets a float trait from an integer (tile, trait id, integer).
const SET_TRAIT_FROM_INT: u32 = 0x0070_0320;
/// Sets a float trait (tile, trait id, float, flag).
const SET_FLOAT_TRAIT: u32 = 0x00a0_12d0;
/// Sets a string trait (tile, trait id, string, flag).
const SET_STRING_TRAIT: u32 = 0x00a0_1350;
/// The float of a trait, in `ST0` (tile, trait id).
const GET_FLOAT_TRAIT: u32 = 0x00a0_11b0;
/// `Tile::GetString` (engine map): the string of a trait (tile, trait id).
const GET_STRING_TRAIT: u32 = 0x00a0_11f0;
/// `Tile::IsTrue` (engine map): whether a trait is set (tile, trait id).
const IS_TRAIT_TRUE: u32 = 0x00a0_1230;
/// `Tile::GetMaximumDepth` (engine map), the float in `ST0`.
const GET_MAXIMUM_DEPTH: u32 = 0x00a0_1530;
/// `Menu::StartFadeOut` (engine map).
const MENU_START_FADE_OUT: u32 = 0x00a1_d910;
/// `Menu::SetMenuTile` (engine map): the `TileMenu *` and a flag.
const MENU_SET_MENU_TILE: u32 = 0x00a1_dc70;
/// `Menu::PrepForVisibility` (engine map), one flag argument.
const MENU_PREP_FOR_VISIBILITY: u32 = 0x00a1_dc20;
/// The getter of the word at `011d8a80` (the interface singleton).
const INTERFACE_SINGLETON: u32 = 0x004b_7210;
/// Reads the word at +0x9c of the interface singleton (the tile reader).
const TILE_READER: u32 = 0x0058_6150;
/// `Tile::ReadFile` (engine map): reader, path; returns the tile root.
const TILE_READ_FILE: u32 = 0x00a0_1b00;
/// `Tile::GetMenu` (engine map).
const TILE_GET_MENU: u32 = 0x00a0_3c90;
/// `Tile::GetMenuByClass` (engine map), cdecl, one class id.
const TILE_GET_MENU_BY_CLASS: u32 = 0x00a0_9030;
/// `Tile::AddUserTrait` (engine map), cdecl (name, id).
const ADD_USER_TRAIT: u32 = 0x00a0_0940;
/// Returns the word at +4 of the object at its `this`, 0 for a null object
/// (the value of a game setting).
const SETTING_VALUE: u32 = 0x0040_3df0;
/// The setting object whose value is the text of the tile at +0x2c.
const SETTING_BUTTON: u32 = 0x011d_38b8;
/// The editor's text (its `BSStringT` at +0 returns the characters).
const EDITOR_TEXT: u32 = 0x0045_8b50;
/// Sets a `BSStringT` from characters (string object, characters, length).
const STRING_ASSIGN: u32 = 0x0040_37f0;
/// The editor's setter that stores `value - 5` at +0x14, replaced by the result of
/// `00715d40` when negative (`00716aa0`, takes the integer).
const EDITOR_STORE_MINUS_FIVE: u32 = 0x0071_6aa0;
/// The editor's setter of its starting text (`00716a70`, takes the
/// characters).
const EDITOR_SET_TEXT: u32 = 0x0071_6a70;
/// The editor's setter of its active flag (`00717010`, takes the flag).
const EDITOR_SET_ACTIVE: u32 = 0x0071_7010;
/// The editor's active flag (the byte at +0x21).
const EDITOR_IS_ACTIVE: u32 = 0x0071_6ae0;
/// The editor's per-frame step (`00717050`).
const EDITOR_STEP: u32 = 0x0071_7050;
/// The editor's key handler (`00716b00`, takes the key).
const EDITOR_KEY: u32 = 0x0071_6b00;
/// The editor's current text with its cursor (`007170a0`, takes a flag).
const EDITOR_DISPLAY_TEXT: u32 = 0x0071_70a0;
/// `_ftol2_sse` (`00ec62c0`): truncates the float in `ST0` to an integer.
const FLOAT_TO_INT: u32 = 0x00ec_62c0;
/// `__RTDynamicCast`.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `Menu::Menu` (engine map has no name; `00a1c4a0` is the base
/// constructor the trait menu's constructor starts with).
const MENU_CONSTRUCT: u32 = 0x00a1_c4a0;
/// The trait menu's list box constructor (`00786bb0`, at +0x54).
const LIST_BOX_CONSTRUCT: u32 = 0x0078_6bb0;
/// The trait menu's perk list constructor (`0096a2d0`, at +0x84).
const PERK_LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// `TraitMenu`'s vtable.
const TRAIT_MENU_VTABLE: u32 = 0x0107_79bc;

/// `Tile`'s RTTI type descriptor.
const TYPE_TILE: u32 = 0x0119_f4cc;
/// `TileMenu`'s RTTI type descriptor.
const TYPE_TILE_MENU: u32 = 0x0119_f49c;
/// `Menu`'s RTTI type descriptor.
const TYPE_MENU: u32 = 0x0119_f34c;
/// `TextEditMenu`'s RTTI type descriptor.
const TYPE_TEXT_EDIT_MENU: u32 = 0x0119_f704;

/// `"Data\Menus\Dialog\TextEditMenu.xml"`.
const MENU_FILE: u32 = 0x0107_7910;
/// The doubles 6006.0 and 102.0 the tile root's trait `0xfa7` is compared with.
const KIND_ONE_DOUBLE: u32 = 0x0107_04f0;
const KIND_TWO_DOUBLE: u32 = 0x0107_04e8;

/// Trait ids (the `Tile::enumTrait` numbers the PC build uses).
const TRAIT_STACKING_TYPE: i32 = 0xfa7;
const TRAIT_DEPTH: i32 = 0xfad;
const TRAIT_TARGET: i32 = 0xfaf;
const TRAIT_WRAP_WIDTH: i32 = 0xfba;
const TRAIT_STRING: i32 = 0xfc4;
const TRAIT_0X1772: i32 = 0x1772;

/// Offset of the embedded text editor.
const EDITOR: u32 = 0x34;

layout! {
    /// The text edit menu. Tile pointers: +0x28 receives the starting text
    /// and shows the editor text, +0x2c has a game setting as its text and
    /// takes the text check's answer, +0x30 receives the prompt. The PC
    /// allocation size is not read from a constructor, 0x5c is the end of the
    /// last field.
    pub struct TextEditMenu: 0x5c {
        /// The function (cdecl, one argument: the editor's text) that says
        /// whether the text is acceptable; the result is the target trait
        /// of the tile at +0x2c. Null for none. Not in the Xbox PDB.
        0x58 pTextCheck: u32,
    }
}

/// A `float` copied through `FLD` / `FSTP`: a signaling NaN comes out quiet.
fn through_x87(value: f32) -> f32 {
    if value.is_nan() {
        f32::from_bits(value.to_bits() | 0x0040_0000)
    } else {
        value
    }
}

fn singleton(e: &Engine) -> Ptr<TextEditMenu> {
    Ptr::new(e.global(TEXT_EDIT_MENU))
}

fn tile_at(e: &Engine, this: Ptr<TextEditMenu>, offset: u32) -> u32 {
    e.mem.u32(this.addr() + offset)
}

// Translated from 007e6320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Opens the text edit menu from `Data\Menus\Dialog\TextEditMenu.xml`.
/// Arguments: the prompt text (string trait of the tile at +0x30), the
/// starting text (tile at +0x28 and the editor), and the text check function
/// (stored at +0x58). Returns whether the menu was created.
///
/// An existing menu of the class is deleted first. The maximum depth is read
/// before the file, and set on the tile root (trait `0xfad`) when its type
/// (stacking type, trait `0xfa7`) is 6006 or 102. The editor's value is the tile at
/// +0x28's wrap width (trait `0xfba`), truncated to an integer. The singleton is set to
/// the menu (cast to `TextEditMenu`); when the file gives no menu of this
/// class the new object is deleted (when its `pMenu` is set) and the result
/// is false.
pub fn fn_007e6320(e: &mut Engine, prompt: u32, start_text: u32, text_check: u32) -> bool {
    let existing = e
        .call(TILE_GET_MENU_BY_CLASS, &args![TEXT_EDIT_CLASS])
        .u32();
    if existing != 0 {
        e.vcall(existing, 0, &args![1u32]);
    }
    let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
    let depth = e.call(GET_MAXIMUM_DEPTH, &args![]).f32();
    let reader = e.call(TILE_READER, &args![interface]).u32();
    let root = e.call(TILE_READ_FILE, &args![reader, MENU_FILE]).u32();
    let menu = e.call(TILE_GET_MENU, &args![root]).u32();
    if menu != 0 {
        let class = e.vcall(menu, 0x34, &args![]).u32();
        if class == fn_007e65a0(e) {
            let tile_menu = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![root, 0i32, TYPE_TILE, TYPE_TILE_MENU, 0i32],
                )
                .u32();
            e.call(MENU_SET_MENU_TILE, &args![menu, tile_menu, 0u32]);
            let this = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![menu, 0i32, TYPE_MENU, TYPE_TEXT_EDIT_MENU, 0i32],
                )
                .u32();
            e.set_global(TEXT_EDIT_MENU, this);
            let kind = e
                .call(GET_FLOAT_TRAIT, &args![root, TRAIT_STACKING_TYPE])
                .f32();
            let kind_matches = kind as f64 == e.global::<f64>(KIND_ONE_DOUBLE) || {
                let again = e
                    .call(GET_FLOAT_TRAIT, &args![root, TRAIT_STACKING_TYPE])
                    .f32();
                again as f64 == e.global::<f64>(KIND_TWO_DOUBLE)
            };
            if kind_matches {
                let depth = through_x87(depth);
                e.call(SET_FLOAT_TRAIT, &args![root, TRAIT_DEPTH, depth, true]);
            }
            let this = singleton(e);
            let prompt_tile = tile_at(e, this, 0x30);
            e.call(
                SET_STRING_TRAIT,
                &args![prompt_tile, TRAIT_STRING, prompt, true],
            );
            let this = singleton(e);
            let text_tile = tile_at(e, this, 0x28);
            e.call(
                SET_STRING_TRAIT,
                &args![text_tile, TRAIT_STRING, start_text, true],
            );
            let limit = e
                .call(GET_FLOAT_TRAIT, &args![text_tile, TRAIT_WRAP_WIDTH])
                .f32();
            let limit = e.call(FLOAT_TO_INT, &args![limit as f64]).i32();
            let editor = singleton(e).addr() + EDITOR;
            e.call(EDITOR_STORE_MINUS_FIVE, &args![editor, limit]);
            let text = e
                .call(GET_STRING_TRAIT, &args![text_tile, TRAIT_STRING])
                .u32();
            let editor = singleton(e).addr() + EDITOR;
            e.call(EDITOR_SET_TEXT, &args![editor, text]);
            let editor = singleton(e).addr() + EDITOR;
            e.call(EDITOR_SET_ACTIVE, &args![editor, 1u32]);
            let editor = singleton(e).addr() + EDITOR;
            fn_007e6580(e, Ptr::new(editor), 1);
            let button = e.call(SETTING_VALUE, &args![SETTING_BUTTON]).u32();
            let this = singleton(e);
            let button_tile = tile_at(e, this, 0x2c);
            e.call(
                SET_STRING_TRAIT,
                &args![button_tile, TRAIT_STRING, button, true],
            );
            let this = singleton(e);
            e.set(this, TextEditMenu::pTextCheck, text_check);
            let this = singleton(e);
            fn_007e6700(e, this);
            e.call(MENU_PREP_FOR_VISIBILITY, &args![menu, 0u32]);
            return true;
        }
    }
    if menu != 0 && e.call(WORD_AT_FOUR, &args![menu]).u32() != 0 {
        e.vcall(menu, 0, &args![1u32]);
    }
    false
}

// Translated from 007e6580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the byte at +0x22 of the text editor (the object at menu +0x34).
pub fn fn_007e6580(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x22, value);
}

// Translated from 007e65a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The menu class id of the text edit menu: `0x41b`.
pub fn fn_007e65a0(_e: &mut Engine) -> u32 {
    TEXT_EDIT_CLASS as u32
}

// Translated from 007e65b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Closes the text edit menu: with one open, sets the trait `0x1772` of its
/// `pMenu` tile to 1, starts its fade out and clears the singleton.
pub fn fn_007e65b0(e: &mut Engine) {
    if singleton(e).is_null() {
        return;
    }
    let this = singleton(e);
    let tile = e.call(WORD_AT_FOUR, &args![this]).u32();
    e.call(SET_TRAIT_FROM_INT, &args![tile, TRAIT_0X1772, 1i32]);
    let this = singleton(e);
    e.call(MENU_START_FADE_OUT, &args![this]);
    e.set_global(TEXT_EDIT_MENU, 0u32);
}

// Translated from 007e65f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Per-frame step (a virtual slot, `+0x2c`): while the editor is active,
/// steps it (`00717050`) and refreshes the menu (`007e6700`).
pub fn fn_007e65f0(e: &mut Engine, this: Ptr<TextEditMenu>) {
    let editor = this.addr() + EDITOR;
    if e.call(EDITOR_IS_ACTIVE, &args![editor]).bool() {
        e.call(EDITOR_STEP, &args![editor]);
        fn_007e6700(e, this);
    }
}

// Translated from 007e6620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Key handler (a virtual slot, `+0x30`). The key `0x80000008` clicks the
/// tile at +0x2c (virtual slot `+0xc` with 1 and that tile) when its trait
/// `0xfaf` is set; any other key goes to the editor while it is active and
/// refreshes the menu. Returns whether the key was handled.
pub fn fn_007e6620(e: &mut Engine, this: Ptr<TextEditMenu>, key: u32) -> bool {
    if key == 0x8000_0008 {
        let button = tile_at(e, this, 0x2c);
        if e.call(IS_TRAIT_TRUE, &args![button, TRAIT_TARGET]).bool() {
            let button = tile_at(e, this, 0x2c);
            e.vcall(this.addr(), 0xc, &args![1u32, button]);
            true
        } else {
            false
        }
    } else {
        let editor = this.addr() + EDITOR;
        if e.call(EDITOR_IS_ACTIVE, &args![editor]).bool() {
            e.call(EDITOR_KEY, &args![editor, key]);
            fn_007e6700(e, this);
            true
        } else {
            false
        }
    }
}

// Translated from 007e66a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Click handler (a virtual slot, `+0xc`): button 1 stores the editor's
/// text in the global string at `011daed8` and closes the menu. The second
/// word (the clicked tile, substituted from the first three tiles when 0)
/// is not used afterwards.
pub fn fn_007e66a0(e: &mut Engine, this: Ptr<TextEditMenu>, button: i32, _unused_1: u32) {
    if button == 1 {
        let text = e.call(EDITOR_TEXT, &args![this.addr() + EDITOR]).u32();
        e.call(STRING_ASSIGN, &args![ENTERED_TEXT, text, 0u32]);
        fn_007e65b0(e);
    }
}

// Translated from 007e6700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Refreshes the menu: the editor's display text goes to the string trait of
/// the tile at +0x28; with a text check function (+0x58), the check's answer
/// for the editor's text becomes the target trait (`0xfaf`) of the tile at +0x2c.
pub fn fn_007e6700(e: &mut Engine, this: Ptr<TextEditMenu>) {
    let editor = this.addr() + EDITOR;
    let shown = e.call(EDITOR_DISPLAY_TEXT, &args![editor]).u32();
    let text_tile = tile_at(e, this, 0x28);
    e.call(
        SET_STRING_TRAIT,
        &args![text_tile, TRAIT_STRING, shown, true],
    );
    let text_check = e.get(this, TextEditMenu::pTextCheck);
    if text_check != 0 {
        let text = e.call(EDITOR_TEXT, &args![editor]).u32();
        let accepted = e.call(text_check, &args![text]).u8();
        let button = tile_at(e, this, 0x2c);
        e.call(
            SET_TRAIT_FROM_INT,
            &args![button, TRAIT_TARGET, accepted as i32],
        );
    }
}

// Translated from 007e6760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the global string at `011daed8` holds text: `007ab810` (the
/// string's character pointer) is not null.
pub fn fn_007e6760(e: &mut Engine, this: Ptr) -> bool {
    e.call(0x007a_b810, &args![this]).u32() != 0
}

// Translated from 007e6780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TraitMenu::TraitMenu` (confirmed by the vtable's RTTI; the engine map
/// has no name for it): constructs the `Menu` base, installs the vtable
/// `010779bc`, constructs the list box (+0x54) and the perk list (+0x84),
/// and registers the eight user traits the menu's tiles use. Each
/// `AddUserTrait(name, -1)` result is stored in a static (the Xbox PDB's
/// `iUT_BaseValue` at `011daf6c`, `iUT_ExtraValue` `011daf60`,
/// `iUT_AddedValue` `011daf58`, `iUT_CurrPoints` `011daf64`,
/// `iUT_MaxPoints` `011daf68`, `iUT_Rank` `011daf78`, `iUT_Selected`
/// `011daf5c` and `iUT_TextAlpha` `011daf70`). Returns `this`.
// Translation incomplete: 007e6780 - the C++ exception-unwinding frame (FS:[0]) that destroys the constructed members when a later step throws is not translated
pub fn fn_007e6780(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(MENU_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), TRAIT_MENU_VTABLE);
    e.call(LIST_BOX_CONSTRUCT, &args![this.addr() + 0x54]);
    e.call(PERK_LIST_CONSTRUCT, &args![this.addr() + 0x84]);
    // (user trait name, the static that receives its id)
    const USER_TRAITS: [(u32, u32); 8] = [
        (0x0107_1ba0, 0x011d_af6c),
        (0x0107_1b94, 0x011d_af60),
        (0x0107_1b88, 0x011d_af58),
        (0x0107_1b7c, 0x011d_af64),
        (0x0107_1b70, 0x011d_af68),
        (0x0107_3cc0, 0x011d_af78),
        (0x0107_1b64, 0x011d_af5c),
        (0x0107_3cb4, 0x011d_af70),
    ];
    for (name, id_static) in USER_TRAITS {
        let id = e.call(ADD_USER_TRAIT, &args![name, -1i32]).i32();
        e.set_global(id_static, id);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x007e6320, fn_007e6320(u32, u32, u32) -> bool),
        entry!(0x007e6580, fn_007e6580(Ptr, u8)),
        entry!(0x007e65a0, fn_007e65a0() -> u32),
        entry!(0x007e65b0, fn_007e65b0()),
        entry!(0x007e65f0, fn_007e65f0(Ptr<TextEditMenu>)),
        entry!(0x007e6620, fn_007e6620(Ptr<TextEditMenu>, u32) -> bool),
        entry!(0x007e66a0, fn_007e66a0(Ptr<TextEditMenu>, i32, u32)),
        entry!(0x007e6700, fn_007e6700(Ptr<TextEditMenu>)),
        entry!(0x007e6760, fn_007e6760(Ptr) -> bool),
        entry!(0x007e6780, fn_007e6780(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// The vtable of the test menu: every slot is the destructor double
    /// except the click slot (`+0xc`) and `GetClass` (`+0x34`).
    const VTABLE: u32 = 0x0f30_0000;
    const DESTRUCTOR: u32 = 0x0f30_1000;
    const DO_CLICK: u32 = 0x0f30_100c;
    const GET_CLASS: u32 = 0x0f30_1034;
    const TEXT_CHECK: u32 = 0x0f40_0000;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011d_a000, 0x1000);
        e.map(0x0107_0000, 0x1000);
        e.set_global(KIND_ONE_DOUBLE, 6006.0f64);
        e.set_global(KIND_TWO_DOUBLE, 102.0f64);
        for addr in [
            SET_TRAIT_FROM_INT,
            SET_FLOAT_TRAIT,
            SET_STRING_TRAIT,
            MENU_START_FADE_OUT,
            MENU_SET_MENU_TILE,
            MENU_PREP_FOR_VISIBILITY,
            STRING_ASSIGN,
            EDITOR_STORE_MINUS_FIVE,
            EDITOR_SET_TEXT,
            EDITOR_SET_ACTIVE,
            EDITOR_STEP,
            EDITOR_KEY,
            DESTRUCTOR,
            DO_CLICK,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(WORD_AT_FOUR, |e, a| word(e.mem.u32(a[0] + 4)));
        e.register(IS_TRAIT_TRUE, |_, _| word(1));
        e.register(EDITOR_IS_ACTIVE, |_, _| word(1));
        e.register(EDITOR_TEXT, |_, _| word(0x7777));
        e.register(EDITOR_DISPLAY_TEXT, |_, _| word(0x9999));
        e.register(RT_DYNAMIC_CAST, |_, a| word(a[0]));
        e.register(SETTING_VALUE, |_, a| word(a[0]));
        e.register(GET_STRING_TRAIT, |_, _| word(0x8888));
        e.register(FLOAT_TO_INT, |_, a| {
            let value = f64::from_bits(a[0] as u64 | (a[1] as u64) << 32);
            word(value as i32 as u32)
        });
        e.register(TEXT_CHECK, |_, _| word(1));
        e.put_vtable(VTABLE, &[DESTRUCTOR; 14]);
        e.mem.set_u32(VTABLE + 0xc, DO_CLICK);
        e.mem.set_u32(VTABLE + 0x34, GET_CLASS);
        e.register(GET_CLASS, |_, _| word(0x41b));
        e.call_log = Some(vec![]);
        e
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(called, _)| *called == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// The tile pointer the test menu holds at `this + offset`.
    fn tile(offset: u32) -> u32 {
        0x6000 + offset
    }

    /// A text edit menu with tiles at +0x28, +0x2c, +0x30 and `pMenu`
    /// 0x6100; not made the singleton.
    fn menu(e: &mut Engine) -> Ptr<TextEditMenu> {
        let this: Ptr<TextEditMenu> = e.new_object();
        e.mem.set_u32(this.addr(), VTABLE);
        e.mem.set_u32(this.addr() + 4, 0x6100);
        for offset in [0x28u32, 0x2c, 0x30] {
            e.mem.set_u32(this.addr() + offset, tile(offset));
        }
        this
    }

    #[test]
    fn the_class_is_0x41b() {
        let mut e = engine();
        assert_eq!(e.call(0x007e_65a0, &args![]).u32(), 0x41b);
    }

    #[test]
    fn fn_007e6580_sets_the_byte_at_0x22() {
        let mut e = engine();
        let editor = e.mem.alloc(0x30);
        e.mem.set_u8(editor + 0x21, 0xaa);
        e.call(0x007e_6580, &args![editor, 1u32]);
        assert_eq!(e.mem.u8(editor + 0x22), 1);
        e.call(0x007e_6580, &args![editor, 0u32]);
        assert_eq!(e.mem.u8(editor + 0x22), 0);
        assert_eq!(e.mem.u8(editor + 0x21), 0xaa);
    }

    #[test]
    fn closing_without_a_menu_does_nothing() {
        let mut e = engine();
        e.call(0x007e_65b0, &args![]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn closing_fades_out_and_clears_the_singleton() {
        let mut e = engine();
        let this = menu(&mut e);
        e.set_global(TEXT_EDIT_MENU, this.addr());
        e.call_log = Some(vec![]);
        e.call(0x007e_65b0, &args![]);
        assert_eq!(
            calls(&e, SET_TRAIT_FROM_INT),
            vec![vec![0x6100, TRAIT_0X1772 as u32, 1]]
        );
        assert_eq!(calls(&e, MENU_START_FADE_OUT), vec![vec![this.addr()]]);
        assert_eq!(e.global::<u32>(TEXT_EDIT_MENU), 0);
    }

    #[test]
    fn the_idle_step_runs_only_while_the_editor_is_active() {
        let mut e = engine();
        let this = menu(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x007e_65f0, &args![this]);
        let editor = this.addr() + 0x34;
        assert_eq!(calls(&e, EDITOR_IS_ACTIVE), vec![vec![editor]]);
        assert_eq!(calls(&e, EDITOR_STEP), vec![vec![editor]]);
        // The refresh set the display text on the tile at +0x28.
        assert_eq!(
            calls(&e, SET_STRING_TRAIT),
            vec![vec![tile(0x28), TRAIT_STRING as u32, 0x9999, 1]]
        );
        e.register(EDITOR_IS_ACTIVE, |_, _| word(0));
        e.call_log = Some(vec![]);
        e.call(0x007e_65f0, &args![this]);
        assert!(calls(&e, EDITOR_STEP).is_empty());
        assert!(calls(&e, SET_STRING_TRAIT).is_empty());
    }

    #[test]
    fn the_back_key_clicks_the_enabled_ok_tile() {
        let mut e = engine();
        let this = menu(&mut e);
        e.call_log = Some(vec![]);
        assert!(e.call(0x007e_6620, &args![this, 0x8000_0008u32]).bool());
        assert_eq!(
            calls(&e, IS_TRAIT_TRUE),
            vec![vec![tile(0x2c), TRAIT_TARGET as u32]]
        );
        // DoClick(1, the OK tile) through slot 0xc.
        assert_eq!(calls(&e, DO_CLICK), vec![vec![this.addr(), 1, tile(0x2c)]]);
        assert!(calls(&e, EDITOR_KEY).is_empty());
    }

    #[test]
    fn the_back_key_does_nothing_when_the_ok_tile_is_not_set() {
        let mut e = engine();
        let this = menu(&mut e);
        e.register(IS_TRAIT_TRUE, |_, _| word(0));
        e.call_log = Some(vec![]);
        assert!(!e.call(0x007e_6620, &args![this, 0x8000_0008u32]).bool());
        assert!(calls(&e, DO_CLICK).is_empty());
    }

    #[test]
    fn other_keys_go_to_the_active_editor() {
        let mut e = engine();
        let this = menu(&mut e);
        e.call_log = Some(vec![]);
        assert!(e.call(0x007e_6620, &args![this, 0x41u32]).bool());
        assert_eq!(calls(&e, EDITOR_KEY), vec![vec![this.addr() + 0x34, 0x41]]);
        // The refresh follows.
        assert_eq!(calls(&e, SET_STRING_TRAIT).len(), 1);
        e.register(EDITOR_IS_ACTIVE, |_, _| word(0));
        e.call_log = Some(vec![]);
        assert!(!e.call(0x007e_6620, &args![this, 0x41u32]).bool());
        assert!(calls(&e, EDITOR_KEY).is_empty());
    }

    #[test]
    fn clicking_ok_stores_the_text_and_closes() {
        let mut e = engine();
        let this = menu(&mut e);
        e.set_global(TEXT_EDIT_MENU, this.addr());
        e.call_log = Some(vec![]);
        e.call(0x007e_66a0, &args![this, 1i32, 0u32]);
        assert_eq!(
            calls(&e, STRING_ASSIGN),
            vec![vec![ENTERED_TEXT, 0x7777, 0]]
        );
        assert_eq!(calls(&e, MENU_START_FADE_OUT).len(), 1);
        assert_eq!(e.global::<u32>(TEXT_EDIT_MENU), 0);
    }

    #[test]
    fn clicking_another_button_does_nothing() {
        let mut e = engine();
        let this = menu(&mut e);
        e.set_global(TEXT_EDIT_MENU, this.addr());
        e.call_log = Some(vec![]);
        e.call(0x007e_66a0, &args![this, 2i32, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        assert_eq!(e.global::<u32>(TEXT_EDIT_MENU), this.addr());
    }

    #[test]
    fn refresh_sets_the_display_text_and_the_check_result() {
        let mut e = engine();
        let this = menu(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x007e_6700, &args![this]);
        // No text check: only the display text.
        assert_eq!(
            calls(&e, SET_STRING_TRAIT),
            vec![vec![tile(0x28), TRAIT_STRING as u32, 0x9999, 1]]
        );
        assert!(calls(&e, SET_TRAIT_FROM_INT).is_empty());
        // With a check: it is called with the editor's text.
        e.set(this, TextEditMenu::pTextCheck, TEXT_CHECK);
        e.call(0x007e_6700, &args![this]);
        assert_eq!(calls(&e, TEXT_CHECK), vec![vec![0x7777]]);
        assert_eq!(
            calls(&e, SET_TRAIT_FROM_INT),
            vec![vec![tile(0x2c), TRAIT_TARGET as u32, 1]]
        );
        // Only the low byte of its answer counts.
        e.register(TEXT_CHECK, |_, _| word(0x1200));
        e.call(0x007e_6700, &args![this]);
        assert_eq!(calls(&e, SET_TRAIT_FROM_INT)[1][2], 0);
    }

    #[test]
    fn the_text_test_follows_the_string_pointer() {
        let mut e = engine();
        let this = menu(&mut e);
        e.register(0x007a_b810, |_, _| word(0x1234));
        assert!(e.call(0x007e_6760, &args![this]).bool());
        e.register(0x007a_b810, |_, _| word(0));
        assert!(!e.call(0x007e_6760, &args![this]).bool());
    }

    #[test]
    fn the_trait_menu_constructor_registers_its_user_traits() {
        let mut e = engine();
        let this = e.mem.alloc(0x8c);
        e.register(MENU_CONSTRUCT, |_, a| word(a[0]));
        e.register(LIST_BOX_CONSTRUCT, |_, _| Ret::default());
        e.register(PERK_LIST_CONSTRUCT, |_, _| Ret::default());
        let mut next = 100u32;
        e.register_double(ADD_USER_TRAIT, move |_, _| {
            next += 1;
            word(next)
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007e_6780, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), TRAIT_MENU_VTABLE);
        assert_eq!(calls(&e, MENU_CONSTRUCT), vec![vec![this]]);
        assert_eq!(calls(&e, LIST_BOX_CONSTRUCT), vec![vec![this + 0x54]]);
        assert_eq!(calls(&e, PERK_LIST_CONSTRUCT), vec![vec![this + 0x84]]);
        // Names in call order, each with id -1.
        let names: Vec<u32> = calls(&e, ADD_USER_TRAIT).iter().map(|c| c[0]).collect();
        assert_eq!(
            names,
            [
                0x0107_1ba0,
                0x0107_1b94,
                0x0107_1b88,
                0x0107_1b7c,
                0x0107_1b70,
                0x0107_3cc0,
                0x0107_1b64,
                0x0107_3cb4
            ]
        );
        assert!(calls(&e, ADD_USER_TRAIT).iter().all(|c| c[1] == u32::MAX));
        // The ids land in the statics, in the order of the calls.
        let statics = [
            0x011d_af6cu32,
            0x011d_af60,
            0x011d_af58,
            0x011d_af64,
            0x011d_af68,
            0x011d_af78,
            0x011d_af5c,
            0x011d_af70,
        ];
        for (index, address) in statics.iter().enumerate() {
            assert_eq!(e.global::<i32>(*address), 101 + index as i32);
        }
    }

    /// A world for the opener: the file gives `menu`; every callee is a
    /// double.
    struct World {
        e: Engine,
        menu: Ptr<TextEditMenu>,
    }

    const ROOT: u32 = 0x5200;

    fn world() -> World {
        let mut e = engine();
        let menu = menu(&mut e);
        let menu_address = menu.addr();
        e.register(TILE_GET_MENU_BY_CLASS, |_, _| word(0));
        e.register(INTERFACE_SINGLETON, |_, _| word(0x5000));
        e.register(TILE_READER, |_, _| word(0x5100));
        e.register(TILE_READ_FILE, |_, _| word(ROOT));
        e.register_double(TILE_GET_MENU, move |_, _| word(menu_address));
        e.register(GET_MAXIMUM_DEPTH, |_, _| Ret {
            st0: 7.5,
            ..Ret::default()
        });
        e.register(GET_FLOAT_TRAIT, |_, a| Ret {
            st0: if a[1] == TRAIT_WRAP_WIDTH as u32 {
                12.75
            } else {
                6006.0
            },
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        World { e, menu }
    }

    #[test]
    fn the_opener_builds_the_menu() {
        let mut w = world();
        assert!(w
            .e
            .call(0x007e_6320, &args![0x1111u32, 0x2222u32, TEXT_CHECK])
            .bool());
        let menu = w.menu;
        let e = &mut w.e;
        let editor = menu.addr() + 0x34;
        assert_eq!(calls(e, TILE_GET_MENU_BY_CLASS), vec![vec![0x41b]]);
        assert_eq!(calls(e, TILE_READER), vec![vec![0x5000]]);
        assert_eq!(calls(e, TILE_READ_FILE), vec![vec![0x5100, MENU_FILE]]);
        assert_eq!(
            calls(e, MENU_SET_MENU_TILE),
            vec![vec![menu.addr(), ROOT, 0]]
        );
        assert_eq!(e.global::<u32>(TEXT_EDIT_MENU), menu.addr());
        // Type 6006: the maximum depth is set on the root.
        assert_eq!(
            calls(e, SET_FLOAT_TRAIT),
            vec![vec![ROOT, TRAIT_DEPTH as u32, 7.5f32.to_bits(), 1]]
        );
        // Prompt on the tile at +0x30, starting text on the one at +0x28
        // (then the refresh sets the display text there), button text on
        // the one at +0x2c.
        let strings = calls(e, SET_STRING_TRAIT);
        assert_eq!(
            strings,
            vec![
                vec![tile(0x30), TRAIT_STRING as u32, 0x1111, 1],
                vec![tile(0x28), TRAIT_STRING as u32, 0x2222, 1],
                vec![tile(0x2c), TRAIT_STRING as u32, SETTING_BUTTON, 1],
                vec![tile(0x28), TRAIT_STRING as u32, 0x9999, 1],
            ]
        );
        // The wrap width 12.75 is truncated to 12 for the editor.
        assert_eq!(calls(e, EDITOR_STORE_MINUS_FIVE), vec![vec![editor, 12]]);
        assert_eq!(calls(e, EDITOR_SET_TEXT), vec![vec![editor, 0x8888]]);
        assert_eq!(calls(e, EDITOR_SET_ACTIVE), vec![vec![editor, 1]]);
        assert_eq!(e.mem.u8(editor + 0x22), 1);
        // The text check is stored and used by the refresh.
        assert_eq!(e.get(menu, TextEditMenu::pTextCheck), TEXT_CHECK);
        assert_eq!(calls(e, TEXT_CHECK), vec![vec![0x7777]]);
        assert_eq!(
            calls(e, MENU_PREP_FOR_VISIBILITY),
            vec![vec![menu.addr(), 0]]
        );
    }

    #[test]
    fn the_opener_reads_the_type_trait_twice_and_only_sets_the_depth_for_two_types() {
        for (value, depth_set, reads) in [
            (6006.0f64, true, 1),
            (102.0, true, 2),
            (5.0, false, 2),
            (f64::NAN, false, 2),
        ] {
            let mut w = world();
            w.e.register_double(GET_FLOAT_TRAIT, move |_, a| Ret {
                st0: if a[1] == TRAIT_STACKING_TYPE as u32 {
                    value
                } else {
                    3.0
                },
                ..Ret::default()
            });
            w.e.call(0x007e_6320, &args![1u32, 2u32, 0u32]);
            let kind_reads = calls(&w.e, GET_FLOAT_TRAIT)
                .iter()
                .filter(|c| c[1] == TRAIT_STACKING_TYPE as u32)
                .count();
            assert_eq!(kind_reads, reads, "{value}");
            assert_eq!(calls(&w.e, SET_FLOAT_TRAIT).len(), depth_set as usize);
        }
    }

    #[test]
    fn the_opener_without_a_check_does_not_call_one() {
        let mut w = world();
        assert!(w.e.call(0x007e_6320, &args![1u32, 2u32, 0u32]).bool());
        assert!(calls(&w.e, TEXT_CHECK).is_empty());
        assert!(calls(&w.e, SET_TRAIT_FROM_INT).is_empty());
    }

    #[test]
    fn the_opener_deletes_an_existing_menu_first() {
        let mut w = world();
        let old = w.e.mem.alloc(0x5c);
        w.e.mem.set_u32(old, VTABLE);
        w.e.register_double(TILE_GET_MENU_BY_CLASS, move |_, a| {
            assert_eq!(a[0], 0x41b);
            word(old)
        });
        w.e.call(0x007e_6320, &args![1u32, 2u32, 0u32]);
        assert_eq!(calls(&w.e, DESTRUCTOR)[0], vec![old, 1]);
    }

    #[test]
    fn the_opener_deletes_a_menu_of_another_class() {
        let mut w = world();
        w.e.register(GET_CLASS, |_, _| word(0x999));
        assert!(!w.e.call(0x007e_6320, &args![1u32, 2u32, 0u32]).bool());
        assert_eq!(calls(&w.e, DESTRUCTOR), vec![vec![w.menu.addr(), 1]]);
        assert!(calls(&w.e, MENU_SET_MENU_TILE).is_empty());
        assert_eq!(w.e.global::<u32>(TEXT_EDIT_MENU), 0);
        // Without a pMenu tile the object is left alone.
        let mut w = world();
        w.e.register(GET_CLASS, |_, _| word(0x999));
        w.e.mem.set_u32(w.menu.addr() + 4, 0);
        assert!(!w.e.call(0x007e_6320, &args![1u32, 2u32, 0u32]).bool());
        assert!(calls(&w.e, DESTRUCTOR).is_empty());
    }

    #[test]
    fn the_opener_fails_without_a_file_menu() {
        let mut w = world();
        w.e.register(TILE_GET_MENU, |_, _| word(0));
        assert!(!w.e.call(0x007e_6320, &args![1u32, 2u32, 0u32]).bool());
        assert!(calls(&w.e, DESTRUCTOR).is_empty());
        assert!(calls(&w.e, WORD_AT_FOUR).is_empty());
    }
}
