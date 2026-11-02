//! `fallout/interface/menus/tutorialmenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The tutorial menu shows the pages of the help manual, a list of
//! messages (`BGSMessage`) kept in a form list. The menu object is the
//! singleton at `011db088` (the Xbox PDB's static `pMe`); it holds its
//! tiles at +0x28 (eight pointers), the current page and the page count.
//!
//! The compiler's exception-unwinding frames are not translated, so the two
//! functions that keep a temporary string (`007e8890`, `007e8ed0`) carry a
//! `Translation incomplete` marker.

#[allow(unused_imports)]
use crate::prelude::*;

/// `TutorialMenu::pMe` (Xbox PDB static): the menu singleton.
const TUTORIAL_MENU: u32 = 0x011d_b088;
/// Returns the word at +4 of its argument (`Menu::pMenu`, or a list node's
/// next pointer).
const WORD_AT_FOUR: u32 = 0x0072_6070;
/// Sets a float trait from an integer: `FILD`, then `00a012d0` with the
/// flag set. Arguments: tile, trait id, integer.
const SET_TRAIT_FROM_INT: u32 = 0x0070_0320;
/// Sets a float trait (tile, trait id, float, flag).
const SET_FLOAT_TRAIT: u32 = 0x00a0_12d0;
/// Sets a string trait (tile, trait id, string, flag).
const SET_STRING_TRAIT: u32 = 0x00a0_1350;
/// The float of a trait, in `ST0` (tile, trait id).
const GET_FLOAT_TRAIT: u32 = 0x00a0_11b0;
/// `Tile::GetMaximumDepth` (engine map), the float in `ST0`.
const GET_MAXIMUM_DEPTH: u32 = 0x00a0_1530;
/// `Interface::GetCurrentRenderedMenu` (engine map).
const GET_CURRENT_RENDERED_MENU: u32 = 0x0070_7ad0;
/// `Menu::StartFadeOut` (engine map).
const MENU_START_FADE_OUT: u32 = 0x00a1_d910;
/// `Menu::InstantFadeOut` (engine map).
const MENU_INSTANT_FADE_OUT: u32 = 0x00a1_d9e0;
/// `Menu::SetMenuTile` (engine map): the `TileMenu *` and a flag.
const MENU_SET_MENU_TILE: u32 = 0x00a1_dc70;
/// `Menu::PrepForVisibility` (engine map), one flag argument.
const MENU_PREP_FOR_VISIBILITY: u32 = 0x00a1_dc20;
/// `StartMenu::ChooseBack` (engine map), no arguments.
const START_MENU_CHOOSE_BACK: u32 = 0x007d_0e40;
/// `Interface::IsTopMenuID` (engine map), cdecl, one class id.
const IS_TOP_MENU_ID: u32 = 0x0070_2450;
/// `InterfaceManager::ClearMenuButton` (engine map), cdecl, one button.
const CLEAR_MENU_BUTTON: u32 = 0x0071_7de0;
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
/// Returns the constant 0x423, the class id of this menu: `Create` looks
/// the menu up by it and compares it with the new menu's `GetClass` slot.
const MENU_CLASS: u32 = 0x007c_c6d0;
/// Whether the gamepad interface is in use (a byte flag and a further
/// check, `004b71d0`).
const GAMEPAD_IN_USE: u32 = 0x004b_71d0;
/// The form with the given id (cdecl, one id), or null.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// The embedded list of a form list (`this + 0x18`).
const FORM_LIST_OF: u32 = 0x0050_0940;
/// Returns its `this` (the address of the first member); the item of a list
/// node is read through it.
const THIS_ITEM: u32 = 0x0068_15c0;
/// An empty member function (`RET`).
const EMPTY_MEMBER: u32 = 0x0048_3710;
/// Returns the word at +0x28 of its object.
const WORD_AT_0X28: u32 = 0x0045_cd60;
/// Returns the word at +4 of the object at its `this`, 0 for a null object
/// (the value of a game setting in the calls below).
const SETTING_VALUE: u32 = 0x0040_3df0;
/// Constructs an 8-byte `BSStringT<char>` in place.
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
/// Destroys an 8-byte `BSStringT<char>`.
const STRING_DESTROY: u32 = 0x0040_37d0;
/// `sprintf` into a `BSStringT` (string, format, arguments...), cdecl.
const STRING_FORMAT: u32 = 0x0040_6f60;
/// The character data of a `BSStringT` (its first word).
const STRING_C_STR: u32 = 0x0055_9450;
/// Prints a message (level, format, arguments...), cdecl.
const PRINT_MESSAGE: u32 = 0x005b_5e40;
/// `MapMarkerData::GetLocationName` (engine map) called on the message's
/// embedded object at +0x18; returns a string.
const MESSAGE_TITLE: u32 = 0x0040_8da0;
/// `Tile::AddUserTrait` (engine map), cdecl (name, id).
const ADD_USER_TRAIT: u32 = 0x00a0_0940;
/// `__RTDynamicCast`.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;

/// The doubles 6006.0 and 102.0 the tile root's trait `0xfa7` is compared with.
const KIND_ONE_DOUBLE: u32 = 0x0107_04f0;
const KIND_TWO_DOUBLE: u32 = 0x0107_04e8;

/// `Tile`'s RTTI type descriptor.
const TYPE_TILE: u32 = 0x0119_f4cc;
/// `TileMenu`'s RTTI type descriptor.
const TYPE_TILE_MENU: u32 = 0x0119_f49c;
/// `TESForm`'s RTTI type descriptor.
const TYPE_FORM: u32 = 0x0118_3028;
/// `BGSMessage`'s RTTI type descriptor.
const TYPE_MESSAGE: u32 = 0x0118_6130;
/// `BGSListForm`'s RTTI type descriptor.
const TYPE_FORM_LIST: u32 = 0x0118_63d4;

/// `"Data\Menus\tutorial_menu.xml"`.
const MENU_FILE: u32 = 0x0107_7ba4;
/// `"%d/%d"`.
const PAGE_FORMAT: u32 = 0x0107_3b14;
/// `"Warning:  Unable to find valid starting message for Tutorial Menu."`.
const NO_STARTING_MESSAGE: u32 = 0x0107_7b60;
/// `"End of Help Manual reached while searching for page %d in help manual."`.
const END_OF_MANUAL: u32 = 0x0107_7bc8;
/// `"_current_value"`.
const CURRENT_VALUE_NAME: u32 = 0x0107_02d8;
/// A `float` constant (255.0) used for an alpha trait.
const FULL_ALPHA: u32 = 0x0102_3cd8;
/// The game setting objects whose values are the texts of the menu's
/// tiles (title, the two page buttons, the done button).
const SETTING_TITLE: u32 = 0x011d_2340;
const SETTING_NEXT: u32 = 0x011d_34b0;
const SETTING_PREVIOUS: u32 = 0x011d_4468;
const SETTING_DONE: u32 = 0x011d_22b0;

/// Trait ids (the `Tile::enumTrait` numbers the PC build uses).
const TRAIT_VISIBLE: i32 = 0xfa3;
const TRAIT_STACKING_TYPE: i32 = 0xfa7;
const TRAIT_DEPTH: i32 = 0xfad;
const TRAIT_ALPHA: i32 = 0xfa9;
const TRAIT_TARGET: i32 = 0xfaf;
const TRAIT_IS_HTML: i32 = 0xfbe;
const TRAIT_STRING: i32 = 0xfc4;
const TRAIT_0X1772: i32 = 0x1772;

layout! {
    /// `TutorialMenu` (Xbox PDB): a `Menu` (0x28 bytes) followed by eight
    /// tiles. The PC object has no `iTimer`: the help manual node pointer
    /// follows the page count at +0x50 (the PDB has `iTimer` there and
    /// `pHelpManual` at +0x54). The allocation size is not read from a
    /// constructor; 0x54 is the end of the last field.
    pub struct TutorialMenu: 0x54 {
        /// `iCurrentPage` (Xbox PDB).
        0x48 iCurrentPage: u32,
        /// `iNumPages` (Xbox PDB).
        0x4C iNumPages: u32,
        /// `pHelpManual` (Xbox PDB name, PC offset): the first node of the
        /// manual's message list (`BSSimpleList<TESForm *>`).
        0x50 pHelpManual: u32,
    }
}

/// Offset of the first of the eight tile pointers (`pTiles`, Xbox PDB); the
/// code addresses the tiles by fixed offsets, the tests fill the table.
#[cfg(test)]
const TILES: u32 = 0x28;

/// The singleton, read from its global each time as the code does.
fn menu(e: &Engine) -> Ptr<TutorialMenu> {
    Ptr::new(e.global(TUTORIAL_MENU))
}

/// The tile pointer at `this + offset`.
fn tile_at(e: &Engine, this: Ptr<TutorialMenu>, offset: u32) -> u32 {
    e.mem.u32(this.addr() + offset)
}

fn set_trait_from_int(e: &mut Engine, tile: u32, trait_id: i32, value: i32) {
    e.call(SET_TRAIT_FROM_INT, &args![tile, trait_id, value]);
}

fn set_string(e: &mut Engine, tile: u32, trait_id: i32, text: u32) {
    e.call(SET_STRING_TRAIT, &args![tile, trait_id, text, true]);
}

// Translated from 007e8d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TutorialMenu::Close` (Xbox PDB): with a menu open, sets the trait
/// `0x1772` of its `pMenu` tile to 1, starts its fade out (an instant one
/// when something is the current rendered menu) and chooses "back" in the
/// start menu.
pub fn tutorial_menu_close(e: &mut Engine) {
    if menu(e).is_null() {
        return;
    }
    let this = menu(e);
    let tile = e.call(WORD_AT_FOUR, &args![this]).u32();
    set_trait_from_int(e, tile, TRAIT_0X1772, 1);
    if e.call(GET_CURRENT_RENDERED_MENU, &args![]).u32() != 0 {
        let this = menu(e);
        e.call(MENU_INSTANT_FADE_OUT, &args![this]);
    } else {
        let this = menu(e);
        e.call(MENU_START_FADE_OUT, &args![this]);
    }
    e.call(START_MENU_CHOOSE_BACK, &args![]);
}

// Translated from 007e8db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TutorialMenu::DoClick` (Xbox PDB): button 1 shows the next page, 2 the
/// previous page and 3 closes the menu.
///
/// The second word (the clicked tile) is only substituted from the menu's
/// tile table (index 0 to 7) when it is 0; the substituted value is never
/// used again, so only the dispatch is translated.
pub fn tutorial_menu_do_click(
    e: &mut Engine,
    this: Ptr<TutorialMenu>,
    button: i32,
    _unused_1: u32,
) {
    match button {
        1 => fn_007e8e20(e, this),
        2 => fn_007e8e40(e, this),
        3 => tutorial_menu_close(e),
        _ => {}
    }
}

// Translated from 007e8e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Shows the next page: `SetPage(iCurrentPage + 1)`.
pub fn fn_007e8e20(e: &mut Engine, this: Ptr<TutorialMenu>) {
    let page = e.get(this, TutorialMenu::iCurrentPage).wrapping_add(1);
    tutorial_menu_set_page(e, this, page);
}

// Translated from 007e8e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Shows the previous page: `SetPage(iCurrentPage - 1)`. From page 0 the
/// unsigned page number wraps and `SetPage` ignores it.
pub fn fn_007e8e40(e: &mut Engine, this: Ptr<TutorialMenu>) {
    let page = e.get(this, TutorialMenu::iCurrentPage).wrapping_sub(1);
    tutorial_menu_set_page(e, this, page);
}

// Translated from 007e8e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// A virtual slot that takes two words (`RET 8`, neither read) and always
/// answers false. The slot is shared with other classes (the same body is
/// referenced from three vtables).
pub fn fn_007e8e60(
    _e: &mut Engine,
    _this: Ptr<TutorialMenu>,
    _unused_0: u32,
    _unused_1: u32,
) -> bool {
    false
}

// Translated from 007e8e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TutorialMenu::DoGamepadUpEvent` (Xbox PDB): when this menu is the top
/// menu (`Interface::IsTopMenuID` of its `GetClass` slot, `+0x34`) and the
/// button is 10, clears that menu button and closes the menu; returns
/// whether it did. The second word is not read (`RET 8`).
pub fn tutorial_menu_do_gamepad_up_event(
    e: &mut Engine,
    this: Ptr<TutorialMenu>,
    button: i32,
    _unused_1: u32,
) -> bool {
    let class = e.vcall(this.addr(), 0x34, &args![]).u32();
    if e.call(IS_TOP_MENU_ID, &args![class]).bool() && button == 10 {
        e.call(INTERFACE_SINGLETON, &args![]);
        e.call(CLEAR_MENU_BUTTON, &args![10i32]);
        tutorial_menu_close(e);
        true
    } else {
        false
    }
}

// Translated from 007e8ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TutorialMenu::SetPage` (Xbox PDB): for a page below the page count,
/// stores it, writes `"%d/%d"` (page + 1, page count) into the string trait
/// of the tile at +0x3c, walks the help manual `page` nodes (a message
/// when the list ends early) and shows the message found there
/// (`SetTitleAndText`, then `UpdatePrevNext`). A page at or above the count
/// does nothing.
///
/// The page number is counted down as the walk goes, also on the step that
/// ends it, so the message printed when the list runs out shows the number
/// of steps still wanted.
// Translation incomplete: 007e8ed0 - the C++ exception-unwinding frame (FS:[0]) that destroys the temporary string is not translated
pub fn tutorial_menu_set_page(e: &mut Engine, this: Ptr<TutorialMenu>, mut page: u32) {
    let page_count = e.get(this, TutorialMenu::iNumPages);
    if page >= page_count {
        return;
    }
    // The exe's frame (`SUB ESP,0x10` under the exception record, then the record link):
    // the page text at `EBP-0x18` (+8).
    e.with_stack(0x20, |e, frame| {
        let text = frame.addr() + 0x8;
        e.call(STRING_CONSTRUCT, &args![text]);
        e.set(this, TutorialMenu::iCurrentPage, page);
        e.call(
            STRING_FORMAT,
            &args![text, PAGE_FORMAT, page.wrapping_add(1), page_count],
        );
        let c_str = e.call(STRING_C_STR, &args![text]).u32();
        let tile = tile_at(e, this, 0x3c);
        set_string(e, tile, TRAIT_STRING, c_str);

        let mut node = e.get(this, TutorialMenu::pHelpManual);
        while node != 0 {
            let steps_left = page;
            page = page.wrapping_sub(1);
            if steps_left == 0 {
                break;
            }
            node = e.call(WORD_AT_FOUR, &args![node]).u32();
        }
        if node == 0 {
            e.call(PRINT_MESSAGE, &args![0x12u32, END_OF_MANUAL, page]);
        } else {
            let item = e.call(THIS_ITEM, &args![node]).u32();
            let form = e.mem.u32(item);
            let message = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![form, 0i32, TYPE_FORM, TYPE_MESSAGE, 0i32],
                )
                .u32();
            tutorial_menu_set_title_and_text(e, this, message);
            tutorial_menu_update_prev_next(e, this);
        }
        e.call(STRING_DESTROY, &args![text]);
    });
}

// Translated from 007e9010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TutorialMenu::UpdatePrevNext` (Xbox PDB): sets the target trait
/// (`0xfaf`) of the tile at +0x30 to whether the current page is not the
/// first and that of the tile at +0x2c to whether another page follows it
/// (unsigned comparisons).
pub fn tutorial_menu_update_prev_next(e: &mut Engine, this: Ptr<TutorialMenu>) {
    let current = e.get(this, TutorialMenu::iCurrentPage);
    let previous = tile_at(e, this, 0x30);
    set_trait_from_int(e, previous, TRAIT_TARGET, (current != 0) as i32);
    let page_count = e.get(this, TutorialMenu::iNumPages);
    let next = tile_at(e, this, 0x2c);
    set_trait_from_int(
        e,
        next,
        TRAIT_TARGET,
        (current.wrapping_add(1) < page_count) as i32,
    );
}

// Translated from 007e9060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TutorialMenu::SetTitleAndText` (Xbox PDB): for a message, sets the
/// string trait of the tile at +0x38 to the title (`00408da0` on the
/// message's object at +0x18); sets the tile at +0x28 to the message's
/// description (slot `+0x10` of its object at +0x24 with `'DESC'`),
/// setting the is-HTML trait (`0xfbe`) when the text starts with `<`; and
/// resets the `_current_value`
/// user trait of the tile at +0x40 to 0.
pub fn tutorial_menu_set_title_and_text(e: &mut Engine, this: Ptr<TutorialMenu>, message: u32) {
    if message == 0 {
        return;
    }
    let title_tile = tile_at(e, this, 0x38);
    if title_tile != 0 {
        let title = e.call(MESSAGE_TITLE, &args![message + 0x18]).u32();
        set_string(e, title_tile, TRAIT_STRING, title);
    }
    let text_tile = tile_at(e, this, 0x28);
    if text_tile != 0 {
        let text = e
            .vcall(message + 0x24, 0x10, &args![0u32, 0x4353_4544u32])
            .u32();
        if text != 0 {
            let marked = e.mem.i8(text) == b'<' as i8;
            set_trait_from_int(e, text_tile, TRAIT_IS_HTML, marked as i32);
            set_string(e, text_tile, TRAIT_STRING, text);
        }
    }
    let value_tile = tile_at(e, this, 0x40);
    if value_tile != 0 {
        let user_trait = e
            .call(ADD_USER_TRAIT, &args![CURRENT_VALUE_NAME, -1i32])
            .i32();
        e.call(
            SET_FLOAT_TRAIT,
            &args![value_tile, user_trait, 0.0f32, false],
        );
    }
}

// Translated from 007e8890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TutorialMenu::Create` (Xbox PDB): builds the menu from
/// `Data\Menus\tutorial_menu.xml`. Arguments: the message to open on
/// (null for the first message of the manual) and whether the page buttons
/// are shown. Returns whether the menu was created.
///
/// Deletes an existing menu of the class first. When the file does not give
/// a menu of this class the new object is deleted (when it exists) and the
/// result is false. With the buttons, the help manual is the form list `0x165`
/// (gamepad in use) or `0x163`; its first message is the default start. The
/// manual is then walked to count the pages and find the start message's
/// page; a missing start message prints a warning and deletes the menu.
// Translation incomplete: 007e8890 - the C++ exception-unwinding frame (FS:[0]) that destroys the temporary string is not translated
pub fn tutorial_menu_create(e: &mut Engine, mut start_message: u32, with_buttons: bool) -> bool {
    let class = e.call(MENU_CLASS, &args![]).u32();
    let old = e.call(TILE_GET_MENU_BY_CLASS, &args![class]).u32();
    if old != 0 {
        e.vcall(old, 0, &args![1u32]);
    }
    let singleton = e.call(INTERFACE_SINGLETON, &args![]).u32();
    let reader = e.call(TILE_READER, &args![singleton]).u32();
    let root = e.call(TILE_READ_FILE, &args![reader, MENU_FILE]).u32();
    let found_menu = if root != 0 {
        e.call(TILE_GET_MENU, &args![root]).u32()
    } else {
        0
    };
    e.set_global(TUTORIAL_MENU, found_menu);

    if !menu(e).is_null() {
        let this_class = e.vcall(menu(e).addr(), 0x34, &args![]).u32();
        let wanted = e.call(MENU_CLASS, &args![]).u32();
        if this_class == wanted {
            return finish_create(e, root, &mut start_message, with_buttons);
        }
    }
    // Not a menu of this class (or none): delete the object, if any.
    let me = menu(e);
    if !me.is_null() && e.call(WORD_AT_FOUR, &args![me]).u32() != 0 {
        let me = menu(e);
        e.vcall(me.addr(), 0, &args![1u32]);
    }
    false
}

/// The part of [`tutorial_menu_create`] after the menu of the right class
/// was read from the file.
fn finish_create(e: &mut Engine, root: u32, start_message: &mut u32, with_buttons: bool) -> bool {
    let tile_menu = e
        .call(
            RT_DYNAMIC_CAST,
            &args![root, 0i32, TYPE_TILE, TYPE_TILE_MENU, 0i32],
        )
        .u32();
    let me = menu(e);
    e.call(MENU_SET_MENU_TILE, &args![me, tile_menu, 0u32]);
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
        let depth = e.call(GET_MAXIMUM_DEPTH, &args![]).f32();
        e.call(SET_FLOAT_TRAIT, &args![root, TRAIT_DEPTH, depth, true]);
    }
    let setting = e.call(SETTING_VALUE, &args![SETTING_TITLE]).u32();
    let tile = tile_at(e, menu(e), 0x44);
    set_string(e, tile, TRAIT_STRING, setting);

    if with_buttons {
        let list_id = if e.call(GAMEPAD_IN_USE, &args![]).bool() {
            0x165u32
        } else {
            0x163u32
        };
        let form = e.call(LOOKUP_FORM, &args![list_id]).u32();
        let form_list = e
            .call(
                RT_DYNAMIC_CAST,
                &args![form, 0i32, TYPE_FORM, TYPE_FORM_LIST, 0i32],
            )
            .u32();
        if form_list != 0 {
            let manual = e.call(FORM_LIST_OF, &args![form_list]).u32();
            let me = menu(e);
            e.set(me, TutorialMenu::pHelpManual, manual);
            if *start_message == 0 {
                let node = e.get(menu(e), TutorialMenu::pHelpManual);
                let item = e.call(THIS_ITEM, &args![node]).u32();
                let first = e.mem.u32(item);
                *start_message = e
                    .call(
                        RT_DYNAMIC_CAST,
                        &args![first, 0i32, TYPE_FORM, TYPE_MESSAGE, 0i32],
                    )
                    .u32();
            }
        }
        let text = e.call(SETTING_VALUE, &args![SETTING_NEXT]).u32();
        let tile = tile_at(e, menu(e), 0x2c);
        set_string(e, tile, TRAIT_STRING, text);
        let text = e.call(SETTING_VALUE, &args![SETTING_PREVIOUS]).u32();
        let tile = tile_at(e, menu(e), 0x30);
        set_string(e, tile, TRAIT_STRING, text);
    } else {
        let tile = tile_at(e, menu(e), 0x2c);
        set_trait_from_int(e, tile, TRAIT_VISIBLE, 0);
        let tile = tile_at(e, menu(e), 0x30);
        set_trait_from_int(e, tile, TRAIT_VISIBLE, 0);
    }
    let text = e.call(SETTING_VALUE, &args![SETTING_DONE]).u32();
    let tile = tile_at(e, menu(e), 0x34);
    set_string(e, tile, TRAIT_STRING, text);
    if *start_message == 0 {
        e.call(PRINT_MESSAGE, &args![0x12u32, NO_STARTING_MESSAGE]);
    }

    // Count the pages and find the one holding the start message.
    let me = menu(e);
    e.set(me, TutorialMenu::iNumPages, 0);
    e.set(me, TutorialMenu::iCurrentPage, 0);
    let mut found = false;
    let mut node = e.get(me, TutorialMenu::pHelpManual);
    while node != 0 {
        let item = e.call(THIS_ITEM, &args![node]).u32();
        if e.mem.u32(item) == 0 {
            break;
        }
        if !found {
            let item = e.call(THIS_ITEM, &args![node]).u32();
            if e.mem.u32(item) == *start_message {
                let me = menu(e);
                let count = e.get(me, TutorialMenu::iNumPages);
                e.set(me, TutorialMenu::iCurrentPage, count);
                found = true;
            }
        }
        let me = menu(e);
        let count = e.get(me, TutorialMenu::iNumPages);
        e.set(me, TutorialMenu::iNumPages, count.wrapping_add(1));
        node = e.call(WORD_AT_FOUR, &args![node]).u32();
    }

    if found {
        // The frame of `007e8890` (`SUB ESP,0x48` under the exception record, then `ESI` and
        // the record link): the page text at `EBP-0x28` (+0x34).
        e.with_stack(0x5c, |e, frame| {
            let text = frame.addr() + 0x34;
            e.call(STRING_CONSTRUCT, &args![text]);
            let me = menu(e);
            let current = e.get(me, TutorialMenu::iCurrentPage);
            let count = e.get(me, TutorialMenu::iNumPages);
            e.call(
                STRING_FORMAT,
                &args![text, PAGE_FORMAT, current.wrapping_add(1), count],
            );
            let c_str = e.call(STRING_C_STR, &args![text]).u32();
            let tile = tile_at(e, menu(e), 0x3c);
            set_string(e, tile, TRAIT_STRING, c_str);
            e.call(STRING_DESTROY, &args![text]);
        });
    } else {
        let tile = tile_at(e, menu(e), 0x3c);
        set_trait_from_int(e, tile, TRAIT_VISIBLE, 0);
        let me = menu(e);
        e.set(me, TutorialMenu::iNumPages, 0);
    }

    if *start_message == 0 {
        let me = menu(e);
        if !me.is_null() {
            e.vcall(me.addr(), 0, &args![1u32]);
        }
        return false;
    }
    let me = menu(e);
    tutorial_menu_set_title_and_text(e, me, *start_message);
    let me = menu(e);
    tutorial_menu_update_prev_next(e, me);
    let me = menu(e);
    e.call(EMPTY_MEMBER, &args![me]);
    let alpha = through_x87(e.global::<f32>(FULL_ALPHA));
    let hud = e.call(INTERFACE_SINGLETON, &args![]).u32();
    let hud_tile = e.call(WORD_AT_0X28, &args![hud]).u32();
    e.call(SET_FLOAT_TRAIT, &args![hud_tile, TRAIT_ALPHA, alpha, true]);
    let me = menu(e);
    e.call(MENU_PREP_FOR_VISIBILITY, &args![me, 0u32]);
    true
}

/// A `float` copied through `FLD` / `FSTP`: a signaling NaN comes out quiet.
fn through_x87(value: f32) -> f32 {
    if value.is_nan() {
        f32::from_bits(value.to_bits() | 0x0040_0000)
    } else {
        value
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x007e8890, tutorial_menu_create(u32, bool) -> bool),
        entry!(0x007e8d60, tutorial_menu_close()),
        entry!(
            0x007e8db0,
            tutorial_menu_do_click(Ptr<TutorialMenu>, i32, u32)
        ),
        entry!(0x007e8e20, fn_007e8e20(Ptr<TutorialMenu>)),
        entry!(0x007e8e40, fn_007e8e40(Ptr<TutorialMenu>)),
        entry!(0x007e8e60, fn_007e8e60(Ptr<TutorialMenu>, u32, u32) -> bool),
        entry!(
            0x007e8e80,
            tutorial_menu_do_gamepad_up_event(Ptr<TutorialMenu>, i32, u32) -> bool
        ),
        entry!(0x007e8ed0, tutorial_menu_set_page(Ptr<TutorialMenu>, u32)),
        entry!(
            0x007e9010,
            tutorial_menu_update_prev_next(Ptr<TutorialMenu>)
        ),
        entry!(
            0x007e9060,
            tutorial_menu_set_title_and_text(Ptr<TutorialMenu>, u32)
        ),
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

    /// The vtable of the test objects: every slot is the destructor double
    /// except `GetClass` (`+0x34`) and the description slot (`+0x10`).
    const VTABLE: u32 = 0x0f30_0000;
    const DESTRUCTOR: u32 = 0x0f30_1000;
    const DESCRIPTION: u32 = 0x0f30_1010;
    const GET_CLASS: u32 = 0x0f30_1034;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011d_b000, 0x1000);
        e.map(0x0107_0000, 0x1000);
        e.map(0x0102_3000, 0x1000);
        e.set_global(KIND_ONE_DOUBLE, 6006.0f64);
        e.set_global(KIND_TWO_DOUBLE, 102.0f64);
        e.set_global(FULL_ALPHA, 255.0f32);
        for addr in [
            SET_TRAIT_FROM_INT,
            SET_FLOAT_TRAIT,
            SET_STRING_TRAIT,
            MENU_START_FADE_OUT,
            MENU_INSTANT_FADE_OUT,
            MENU_PREP_FOR_VISIBILITY,
            MENU_SET_MENU_TILE,
            START_MENU_CHOOSE_BACK,
            CLEAR_MENU_BUTTON,
            STRING_CONSTRUCT,
            STRING_DESTROY,
            PRINT_MESSAGE,
            EMPTY_MEMBER,
            DESTRUCTOR,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(WORD_AT_FOUR, |e, a| word(e.mem.u32(a[0] + 4)));
        e.register(THIS_ITEM, |_, a| word(a[0]));
        e.register(GET_CURRENT_RENDERED_MENU, |_, _| word(0));
        e.register(INTERFACE_SINGLETON, |_, _| word(0x5000));
        e.register(IS_TOP_MENU_ID, |_, _| word(1));
        e.register(RT_DYNAMIC_CAST, |_, a| word(a[0]));
        e.register(MESSAGE_TITLE, |_, a| word(a[0]));
        e.register(SETTING_VALUE, |_, a| word(a[0]));
        e.register(STRING_FORMAT, |e, a| {
            let text = format!("{}/{}", a[2], a[3]);
            let buffer = e.mem.alloc(16);
            e.mem.set_cstr(buffer, text.as_bytes());
            e.mem.set_u32(a[0], buffer);
            Ret::default()
        });
        e.register(STRING_C_STR, |e, a| word(e.mem.u32(a[0])));
        e.put_vtable(VTABLE, &[DESTRUCTOR; 14]);
        e.mem.set_u32(VTABLE + 0x34, GET_CLASS);
        e.mem.set_u32(VTABLE + 0x10, DESCRIPTION);
        e.register(ADD_USER_TRAIT, |_, _| word(0x777));
        e.register(GET_CLASS, |_, _| word(0x423));
        e.register(DESCRIPTION, |_, _| word(0));
        e.call_log = Some(vec![]);
        e
    }

    /// The local the next `with_stack(size)` on the running thread takes.
    fn next_frame(e: &mut Engine, size: u32) -> u32 {
        e.next_stack_local(size)
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

    /// A chain of list nodes (item at +0, next at +4); returns the head.
    fn list(e: &mut Engine, items: &[u32]) -> u32 {
        let mut next = 0;
        for item in items.iter().rev() {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, *item);
            e.mem.set_u32(node + 4, next);
            next = node;
        }
        next
    }

    /// A message object: its description object at +0x24 uses the test
    /// vtable (no text unless a test sets one).
    fn message(e: &mut Engine) -> u32 {
        let message = e.mem.alloc(0x40);
        e.mem.set_u32(message + 0x24, VTABLE);
        message
    }

    fn messages(e: &mut Engine, count: usize) -> Vec<u32> {
        (0..count).map(|_| message(e)).collect()
    }

    /// The tile pointer the test menu holds at `this + offset`.
    fn tile(offset: u32) -> u32 {
        0x6000 + offset
    }

    /// A tutorial menu with the tile pointers `tile(offset)`, its `pMenu`
    /// tile 0x6100, the given help manual and page state.
    fn menu_with(e: &mut Engine, items: &[u32], current: u32, pages: u32) -> Ptr<TutorialMenu> {
        let this: Ptr<TutorialMenu> = e.new_object();
        e.mem.set_u32(this.addr(), VTABLE);
        e.mem.set_u32(this.addr() + 4, 0x6100);
        for slot in 0..8 {
            let offset = TILES + 4 * slot;
            e.mem.set_u32(this.addr() + offset, tile(offset));
        }
        let head = list(e, items);
        e.set(this, TutorialMenu::pHelpManual, head);
        e.set(this, TutorialMenu::iCurrentPage, current);
        e.set(this, TutorialMenu::iNumPages, pages);
        e.set_global(TUTORIAL_MENU, this.addr());
        this
    }

    fn enabled_calls(e: &Engine) -> Vec<Vec<u32>> {
        calls(e, SET_TRAIT_FROM_INT)
            .into_iter()
            .filter(|call| call[1] == TRAIT_TARGET as u32)
            .collect()
    }

    /// The string-trait calls made on the tile at `offset`.
    fn strings_on(e: &Engine, offset: u32) -> Vec<Vec<u32>> {
        calls(e, SET_STRING_TRAIT)
            .into_iter()
            .filter(|call| call[0] == tile(offset))
            .collect()
    }

    #[test]
    fn close_without_a_menu_does_nothing() {
        let mut e = engine();
        e.call(0x007e_8d60, &args![]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn close_fades_out_and_goes_back() {
        let mut e = engine();
        let this = menu_with(&mut e, &[], 0, 1);
        e.call_log = Some(vec![]);
        e.call(0x007e_8d60, &args![]);
        assert_eq!(
            calls(&e, SET_TRAIT_FROM_INT),
            vec![vec![0x6100, TRAIT_0X1772 as u32, 1]]
        );
        assert_eq!(calls(&e, MENU_START_FADE_OUT), vec![vec![this.addr()]]);
        assert!(calls(&e, MENU_INSTANT_FADE_OUT).is_empty());
        assert_eq!(calls(&e, START_MENU_CHOOSE_BACK).len(), 1);
    }

    #[test]
    fn close_fades_instantly_when_a_menu_is_rendered() {
        let mut e = engine();
        let this = menu_with(&mut e, &[], 0, 1);
        e.register(GET_CURRENT_RENDERED_MENU, |_, _| word(0x1234));
        e.call_log = Some(vec![]);
        e.call(0x007e_8d60, &args![]);
        assert_eq!(calls(&e, MENU_INSTANT_FADE_OUT), vec![vec![this.addr()]]);
        assert!(calls(&e, MENU_START_FADE_OUT).is_empty());
        assert_eq!(calls(&e, START_MENU_CHOOSE_BACK).len(), 1);
    }

    #[test]
    fn click_dispatches_next_previous_and_close() {
        let mut e = engine();
        let items = messages(&mut e, 3);
        let this = menu_with(&mut e, &items, 1, 3);
        e.call(0x007e_8db0, &args![this, 1i32, 0u32]);
        assert_eq!(e.get(this, TutorialMenu::iCurrentPage), 2);
        e.call(0x007e_8db0, &args![this, 2i32, 0u32]);
        e.call(0x007e_8db0, &args![this, 2i32, 0u32]);
        assert_eq!(e.get(this, TutorialMenu::iCurrentPage), 0);
        // An unknown button does nothing; button 3 closes the menu.
        e.call_log = Some(vec![]);
        e.call(0x007e_8db0, &args![this, 9i32, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        e.call(0x007e_8db0, &args![this, 3i32, 0x1234u32]);
        assert_eq!(calls(&e, START_MENU_CHOOSE_BACK).len(), 1);
    }

    #[test]
    fn previous_from_the_first_page_changes_nothing() {
        let mut e = engine();
        let items = messages(&mut e, 2);
        let this = menu_with(&mut e, &items, 0, 2);
        e.call_log = Some(vec![]);
        e.call(0x007e_8e40, &args![this]);
        assert_eq!(e.get(this, TutorialMenu::iCurrentPage), 0);
        assert!(calls(&e, SET_STRING_TRAIT).is_empty());
    }

    #[test]
    fn next_from_the_last_page_changes_nothing() {
        let mut e = engine();
        let items = messages(&mut e, 2);
        let this = menu_with(&mut e, &items, 1, 2);
        e.call_log = Some(vec![]);
        e.call(0x007e_8e20, &args![this]);
        assert_eq!(e.get(this, TutorialMenu::iCurrentPage), 1);
        assert!(calls(&e, SET_STRING_TRAIT).is_empty());
    }

    #[test]
    fn the_shared_slot_answers_false() {
        let mut e = engine();
        let this = menu_with(&mut e, &[], 0, 1);
        assert!(!e.call(0x007e_8e60, &args![this, 1u32, 2u32]).bool());
    }

    #[test]
    fn gamepad_button_ten_on_the_top_menu_closes_it() {
        let mut e = engine();
        let this = menu_with(&mut e, &[], 0, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x007e_8e80, &args![this, 10i32, 0u32]).bool());
        assert_eq!(calls(&e, IS_TOP_MENU_ID), vec![vec![0x423]]);
        assert_eq!(calls(&e, CLEAR_MENU_BUTTON), vec![vec![10]]);
        assert_eq!(calls(&e, START_MENU_CHOOSE_BACK).len(), 1);
    }

    #[test]
    fn gamepad_other_buttons_or_another_top_menu_do_nothing() {
        let mut e = engine();
        let this = menu_with(&mut e, &[], 0, 1);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x007e_8e80, &args![this, 9i32, 0u32]).bool());
        e.register(IS_TOP_MENU_ID, |_, _| word(0));
        assert!(!e.call(0x007e_8e80, &args![this, 10i32, 0u32]).bool());
        assert!(calls(&e, CLEAR_MENU_BUTTON).is_empty());
        assert!(calls(&e, START_MENU_CHOOSE_BACK).is_empty());
    }

    #[test]
    fn set_page_shows_the_message_of_the_page() {
        let mut e = engine();
        let items = messages(&mut e, 3);
        let this = menu_with(&mut e, &items, 0, 3);
        e.call_log = Some(vec![]);
        e.call(0x007e_8ed0, &args![this, 2u32]);
        assert_eq!(e.get(this, TutorialMenu::iCurrentPage), 2);
        // "3/3" goes to the page counter tile (at +0x3c).
        let counter = strings_on(&e, 0x3c);
        assert_eq!(counter.len(), 1);
        assert_eq!(counter[0][1], TRAIT_STRING as u32);
        assert_eq!(e.mem.cstr(counter[0][2]), b"3/3");
        assert_eq!(counter[0][3], 1);
        // The third message was cast, and its title set on the tile at +0x38.
        let casts = calls(&e, RT_DYNAMIC_CAST);
        assert_eq!(casts[0][0], items[2]);
        assert_eq!(casts[0][1..], [0, TYPE_FORM, TYPE_MESSAGE, 0]);
        assert_eq!(calls(&e, MESSAGE_TITLE), vec![vec![items[2] + 0x18]]);
        // Page 2 of 3 has a previous page and no next one.
        assert_eq!(
            enabled_calls(&e),
            vec![
                vec![tile(0x30), TRAIT_TARGET as u32, 1],
                vec![tile(0x2c), TRAIT_TARGET as u32, 0]
            ]
        );
        // The temporary string was destroyed.
        assert_eq!(calls(&e, STRING_CONSTRUCT), calls(&e, STRING_DESTROY));
    }

    #[test]
    fn set_page_ignores_pages_beyond_the_count() {
        let mut e = engine();
        let this = menu_with(&mut e, &[], 0, 2);
        e.call_log = Some(vec![]);
        e.call(0x007e_8ed0, &args![this, 2u32]);
        e.call(0x007e_8ed0, &args![this, u32::MAX]);
        assert_eq!(e.get(this, TutorialMenu::iCurrentPage), 0);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn set_page_reports_a_manual_shorter_than_the_page_count() {
        let mut e = engine();
        // Five pages are claimed but the list has three messages.
        let items = messages(&mut e, 3);
        let this = menu_with(&mut e, &items, 0, 5);
        e.call_log = Some(vec![]);
        let frame = next_frame(&mut e, 0x20);
        e.call(0x007e_8ed0, &args![this, 4u32]);
        // The 0x20-byte frame: the page text at +8.
        assert_eq!(calls(&e, STRING_CONSTRUCT)[0][0], frame + 8);
        // The walk takes three steps before the end, leaving 4 - 3 = 1.
        assert_eq!(calls(&e, PRINT_MESSAGE), vec![vec![0x12, END_OF_MANUAL, 1]]);
        assert!(calls(&e, RT_DYNAMIC_CAST).is_empty());
        assert_eq!(calls(&e, STRING_DESTROY).len(), 1);
    }

    #[test]
    fn set_page_zero_uses_the_first_node() {
        let mut e = engine();
        let items = messages(&mut e, 2);
        let this = menu_with(&mut e, &items, 1, 2);
        e.call_log = Some(vec![]);
        e.call(0x007e_8ed0, &args![this, 0u32]);
        assert_eq!(calls(&e, RT_DYNAMIC_CAST)[0][0], items[0]);
        assert_eq!(e.get(this, TutorialMenu::iCurrentPage), 0);
    }

    #[test]
    fn prev_next_follow_the_page() {
        let mut e = engine();
        for (current, pages, previous, next) in [
            (0u32, 1u32, 0u32, 0u32),
            (0, 3, 0, 1),
            (1, 3, 1, 1),
            (2, 3, 1, 0),
        ] {
            let this = menu_with(&mut e, &[], current, pages);
            e.call_log = Some(vec![]);
            e.call(0x007e_9010, &args![this]);
            assert_eq!(
                enabled_calls(&e),
                vec![
                    vec![tile(0x30), TRAIT_TARGET as u32, previous],
                    vec![tile(0x2c), TRAIT_TARGET as u32, next]
                ],
                "page {current} of {pages}"
            );
        }
    }

    /// A message whose `'DESC'` text (slot `+0x10` of its object at +0x24)
    /// is `text`.
    fn message_with_text(e: &mut Engine, text: &[u8]) -> u32 {
        let message = message(e);
        let buffer = e.mem.alloc(32);
        e.mem.set_cstr(buffer, text);
        e.register_double(DESCRIPTION, move |_, a| {
            // `ECX` is the object at +0x24; the words are 0 and 'DESC'.
            assert_eq!(a[1..], [0, 0x4353_4544]);
            word(buffer)
        });
        message
    }

    #[test]
    fn title_and_text_set_the_tiles() {
        let mut e = engine();
        let this = menu_with(&mut e, &[], 0, 1);
        let message = message_with_text(&mut e, b"<b>Hello");
        e.register(ADD_USER_TRAIT, |_, _| word(0x1999));
        e.call_log = Some(vec![]);
        e.call(0x007e_9060, &args![this, message]);
        // Title on the tile at +0x38, text on the tile at +0x28.
        assert_eq!(
            strings_on(&e, 0x38),
            vec![vec![tile(0x38), TRAIT_STRING as u32, message + 0x18, 1]]
        );
        let text = strings_on(&e, 0x28);
        assert_eq!(text.len(), 1);
        assert_eq!(e.mem.cstr(text[0][2]), b"<b>Hello");
        // The text starts with '<': the is-HTML trait is set.
        assert_eq!(
            calls(&e, SET_TRAIT_FROM_INT),
            vec![vec![tile(0x28), TRAIT_IS_HTML as u32, 1]]
        );
        // The user trait of the tile at +0x40 is reset to 0.
        assert_eq!(
            calls(&e, ADD_USER_TRAIT),
            vec![vec![CURRENT_VALUE_NAME, u32::MAX]]
        );
        assert_eq!(
            calls(&e, SET_FLOAT_TRAIT),
            vec![vec![tile(0x40), 0x1999, 0, 0]]
        );
    }

    #[test]
    fn text_without_an_angle_bracket_clears_the_trait() {
        let mut e = engine();
        let this = menu_with(&mut e, &[], 0, 1);
        let message = message_with_text(&mut e, b"Plain");
        e.register(ADD_USER_TRAIT, |_, _| word(5));
        e.call_log = Some(vec![]);
        e.call(0x007e_9060, &args![this, message]);
        assert_eq!(
            calls(&e, SET_TRAIT_FROM_INT),
            vec![vec![tile(0x28), TRAIT_IS_HTML as u32, 0]]
        );
    }

    #[test]
    fn missing_tiles_and_a_null_message_are_skipped() {
        let mut e = engine();
        let this = menu_with(&mut e, &[], 0, 1);
        for offset in [0x28u32, 0x38, 0x40] {
            e.mem.set_u32(this.addr() + offset, 0);
        }
        e.call_log = Some(vec![]);
        e.call(0x007e_9060, &args![this, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        let message = message(&mut e);
        e.call(0x007e_9060, &args![this, message]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn a_null_description_leaves_the_text_tile_alone() {
        let mut e = engine();
        let this = menu_with(&mut e, &[], 0, 1);
        let message = message(&mut e);
        e.register(ADD_USER_TRAIT, |_, _| word(7));
        e.call_log = Some(vec![]);
        e.call(0x007e_9060, &args![this, message]);
        assert!(calls(&e, SET_TRAIT_FROM_INT).is_empty());
        // Only the title was set.
        assert_eq!(calls(&e, SET_STRING_TRAIT).len(), 1);
    }

    /// A world for `Create`: the file gives `menu`; every callee is a double.
    struct World {
        e: Engine,
        menu: Ptr<TutorialMenu>,
        items: Vec<u32>,
    }

    const ROOT: u32 = 0x5200;
    const FORM_LIST: u32 = 0xf1f1;

    fn world(count: usize) -> World {
        let mut e = engine();
        let items = messages(&mut e, count);
        let menu = menu_with(&mut e, &[], 0, 0);
        let manual = list(&mut e, &items);
        let menu_address = menu.addr();
        e.register(MENU_CLASS, |_, _| word(0x423));
        e.register(TILE_GET_MENU_BY_CLASS, |_, _| word(0));
        e.register(INTERFACE_SINGLETON, |_, _| word(0x5000));
        e.register(TILE_READER, |_, _| word(0x5100));
        e.register(TILE_READ_FILE, |_, _| word(ROOT));
        e.register_double(TILE_GET_MENU, move |_, _| word(menu_address));
        e.register(GET_FLOAT_TRAIT, |_, _| Ret {
            st0: 6006.0,
            ..Ret::default()
        });
        e.register(GET_MAXIMUM_DEPTH, |_, _| Ret {
            st0: 7.5,
            ..Ret::default()
        });
        e.register(WORD_AT_0X28, |_, _| word(0x5300));
        e.register(GAMEPAD_IN_USE, |_, _| word(0));
        e.register(LOOKUP_FORM, |_, a| word(0xf000 + a[0]));
        e.register_double(FORM_LIST_OF, move |_, _| word(manual));
        e.register(RT_DYNAMIC_CAST, |_, a| {
            if a[3] == TYPE_FORM_LIST {
                word(FORM_LIST)
            } else {
                word(a[0])
            }
        });
        e.call_log = Some(vec![]);
        World { e, menu, items }
    }

    #[test]
    fn create_with_buttons_opens_on_the_given_message() {
        let mut w = world(3);
        let start = w.items[1];
        assert!(w.e.call(0x007e_8890, &args![start, true]).bool());
        let menu = w.menu;
        let e = &mut w.e;
        // The file was read through the reader of the interface singleton.
        assert_eq!(calls(e, TILE_READER), vec![vec![0x5000]]);
        assert_eq!(calls(e, TILE_READ_FILE), vec![vec![0x5100, MENU_FILE]]);
        assert_eq!(e.global::<u32>(TUTORIAL_MENU), menu.addr());
        // The class 0x163 manual (no gamepad) was looked up and cast.
        assert_eq!(calls(e, LOOKUP_FORM), vec![vec![0x163]]);
        assert_eq!(calls(e, FORM_LIST_OF), vec![vec![FORM_LIST]]);
        assert_eq!(e.get(menu, TutorialMenu::iNumPages), 3);
        assert_eq!(e.get(menu, TutorialMenu::iCurrentPage), 1);
        // The tile root's type 6006 gets the maximum depth.
        assert_eq!(
            calls(e, SET_FLOAT_TRAIT)[0],
            vec![ROOT, TRAIT_DEPTH as u32, 7.5f32.to_bits(), 1]
        );
        // Texts of the tiles come from the game settings.
        assert_eq!(strings_on(e, 0x44)[0][2], SETTING_TITLE);
        assert_eq!(strings_on(e, 0x2c)[0][2], SETTING_NEXT);
        assert_eq!(strings_on(e, 0x30)[0][2], SETTING_PREVIOUS);
        assert_eq!(strings_on(e, 0x34)[0][2], SETTING_DONE);
        assert_eq!(e.mem.cstr(strings_on(e, 0x3c)[0][2]), b"2/3");
        // The message of page 1 is shown.
        assert_eq!(calls(e, MESSAGE_TITLE), vec![vec![start + 0x18]]);
        // The HUD tile gets the full alpha, then the menu is prepared.
        let alpha = calls(e, SET_FLOAT_TRAIT).pop().unwrap();
        assert_eq!(
            alpha,
            vec![0x5300, TRAIT_ALPHA as u32, 255.0f32.to_bits(), 1]
        );
        assert_eq!(
            calls(e, MENU_PREP_FOR_VISIBILITY),
            vec![vec![menu.addr(), 0]]
        );
        assert_eq!(
            calls(e, MENU_SET_MENU_TILE),
            vec![vec![menu.addr(), ROOT, 0]]
        );
    }

    #[test]
    fn create_uses_the_gamepad_manual_and_the_first_message_by_default() {
        let mut w = world(2);
        w.e.register(GAMEPAD_IN_USE, |_, _| word(1));
        let frame = next_frame(&mut w.e, 0x5c);
        assert!(w.e.call(0x007e_8890, &args![0u32, true]).bool());
        // The 0x5c-byte frame of 007e8890: the page text at +0x34.
        assert!(calls(&w.e, STRING_CONSTRUCT).contains(&vec![frame + 0x34]));
        assert_eq!(calls(&w.e, LOOKUP_FORM), vec![vec![0x165]]);
        assert_eq!(w.e.get(w.menu, TutorialMenu::iCurrentPage), 0);
        assert_eq!(w.e.get(w.menu, TutorialMenu::iNumPages), 2);
        assert_eq!(calls(&w.e, MESSAGE_TITLE), vec![vec![w.items[0] + 0x18]]);
    }

    #[test]
    fn create_without_buttons_hides_them_and_counts_no_pages() {
        let mut w = world(0);
        let start = message(&mut w.e);
        assert!(w.e.call(0x007e_8890, &args![start, false]).bool());
        let e = &mut w.e;
        assert!(calls(e, LOOKUP_FORM).is_empty());
        let hidden: Vec<Vec<u32>> = calls(e, SET_TRAIT_FROM_INT)
            .into_iter()
            .filter(|call| call[1] == TRAIT_VISIBLE as u32)
            .collect();
        // Next, previous, and (no page was found) the page counter.
        assert_eq!(
            hidden,
            vec![
                vec![tile(0x2c), TRAIT_VISIBLE as u32, 0],
                vec![tile(0x30), TRAIT_VISIBLE as u32, 0],
                vec![tile(0x3c), TRAIT_VISIBLE as u32, 0],
            ]
        );
        assert_eq!(e.get(w.menu, TutorialMenu::iNumPages), 0);
    }

    #[test]
    fn create_with_a_start_message_outside_the_manual_hides_the_counter() {
        let mut w = world(2);
        let outside = message(&mut w.e);
        assert!(w.e.call(0x007e_8890, &args![outside, true]).bool());
        assert_eq!(w.e.get(w.menu, TutorialMenu::iNumPages), 0);
        assert!(strings_on(&w.e, 0x3c).is_empty());
        // The message is still shown.
        assert_eq!(calls(&w.e, MESSAGE_TITLE), vec![vec![outside + 0x18]]);
    }

    #[test]
    fn create_without_a_start_message_warns_and_deletes_the_menu() {
        let mut w = world(0);
        assert!(!w.e.call(0x007e_8890, &args![0u32, false]).bool());
        assert_eq!(
            calls(&w.e, PRINT_MESSAGE),
            vec![vec![0x12, NO_STARTING_MESSAGE]]
        );
        assert_eq!(calls(&w.e, DESTRUCTOR), vec![vec![w.menu.addr(), 1]]);
        // The singleton still points at the deleted object.
        assert_eq!(w.e.global::<u32>(TUTORIAL_MENU), w.menu.addr());
        assert!(calls(&w.e, MENU_PREP_FOR_VISIBILITY).is_empty());
    }

    #[test]
    fn create_checks_the_trait_against_both_types() {
        for (value, depth_set, reads) in [
            (6006.0f64, true, 1),
            (102.0, true, 2),
            (5.0, false, 2),
            (f64::NAN, false, 2),
        ] {
            let mut w = world(1);
            w.e.register_double(GET_FLOAT_TRAIT, move |_, _| Ret {
                st0: value,
                ..Ret::default()
            });
            let start = w.items[0];
            w.e.call(0x007e_8890, &args![start, false]);
            assert_eq!(calls(&w.e, GET_FLOAT_TRAIT).len(), reads, "{value}");
            let depth = calls(&w.e, SET_FLOAT_TRAIT)
                .iter()
                .any(|call| call[1] == TRAIT_DEPTH as u32);
            assert_eq!(depth, depth_set, "{value}");
        }
    }

    #[test]
    fn create_deletes_an_existing_menu_first() {
        let mut w = world(1);
        let old = w.e.mem.alloc(0x54);
        w.e.mem.set_u32(old, VTABLE);
        w.e.register_double(TILE_GET_MENU_BY_CLASS, move |_, a| {
            assert_eq!(a[0], 0x423);
            word(old)
        });
        let start = w.items[0];
        w.e.call(0x007e_8890, &args![start, false]);
        assert_eq!(calls(&w.e, DESTRUCTOR)[0], vec![old, 1]);
    }

    #[test]
    fn create_fails_without_a_file_menu() {
        let mut w = world(1);
        w.e.register(TILE_READ_FILE, |_, _| word(0));
        let start = w.items[0];
        assert!(!w.e.call(0x007e_8890, &args![start, true]).bool());
        assert_eq!(w.e.global::<u32>(TUTORIAL_MENU), 0);
        assert!(calls(&w.e, TILE_GET_MENU).is_empty());
        assert!(calls(&w.e, DESTRUCTOR).is_empty());
    }

    #[test]
    fn create_deletes_a_menu_of_another_class() {
        let mut w = world(1);
        w.e.register(GET_CLASS, |_, _| word(0x999));
        let start = w.items[0];
        assert!(!w.e.call(0x007e_8890, &args![start, true]).bool());
        assert_eq!(calls(&w.e, DESTRUCTOR), vec![vec![w.menu.addr(), 1]]);
        assert!(calls(&w.e, MENU_SET_MENU_TILE).is_empty());
    }

    #[test]
    fn create_keeps_a_menu_of_another_class_without_a_tile() {
        // The word at +4 (pMenu) is null: the object is not deleted.
        let mut w = world(1);
        w.e.register(GET_CLASS, |_, _| word(0x999));
        w.e.mem.set_u32(w.menu.addr() + 4, 0);
        let start = w.items[0];
        assert!(!w.e.call(0x007e_8890, &args![start, true]).bool());
        assert!(calls(&w.e, DESTRUCTOR).is_empty());
    }

    #[test]
    fn create_survives_a_missing_form_list() {
        let mut w = world(1);
        w.e.register(RT_DYNAMIC_CAST, |_, a| {
            if a[3] == TYPE_FORM_LIST {
                word(0)
            } else {
                word(a[0])
            }
        });
        let start = w.items[0];
        assert!(w.e.call(0x007e_8890, &args![start, true]).bool());
        assert!(calls(&w.e, FORM_LIST_OF).is_empty());
        assert_eq!(w.e.get(w.menu, TutorialMenu::iNumPages), 0);
    }
}
